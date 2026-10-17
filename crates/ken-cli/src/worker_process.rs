//! The process boundary around the CLI's elaboration worker (AC-2, option 1).
//!
//! The parent re-executes this binary with the same arguments and inherited
//! stdio, and waits. The child runs the unchanged 256-MiB worker path. A
//! native-stack overflow on any child thread is classified by the Rust
//! runtime's own guard-page handler, which prints its message and aborts; it
//! is never reimplemented here. The child only records, on a side channel,
//! that a SIGSEGV/SIGBUS reached that handler. The parent reports the coded
//! refusal exactly when the child died of SIGABRT after such a record. Every
//! other exit is propagated unchanged.

use crate::cli_errors::{emit_cli_error, CliError};
use std::ffi::OsStr;
use std::io::Read;
use std::os::fd::{AsRawFd, FromRawFd, RawFd};
use std::os::unix::process::ExitStatusExt;
use std::process::Command;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::OnceLock;

/// Present only in the child's environment: the side channel's descriptor.
const WORKER_CHILD_VAR: &str = "KEN_CLI_WORKER_CHILD_FAULT_FD";

pub(crate) fn is_child() -> bool {
    std::env::var_os(WORKER_CHILD_VAR).is_some()
}

/// Parent: run this invocation in a child process and return its exit code.
pub(crate) fn run_parent() -> i32 {
    let spawned = (|| -> std::io::Result<_> {
        let executable = std::env::current_exe()?;
        let mut fds = [0 as RawFd; 2];
        // SAFETY: pipe(2) writes two fresh descriptors into `fds`, each owned
        // exactly once below. Only the writer is made inheritable.
        let (reader, writer) = unsafe {
            if libc::pipe(fds.as_mut_ptr()) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            let reader = std::fs::File::from_raw_fd(fds[0]);
            let writer = std::fs::File::from_raw_fd(fds[1]);
            libc::fcntl(fds[0], libc::F_SETFD, libc::FD_CLOEXEC);
            (reader, writer)
        };
        let fd = writer.as_raw_fd();
        let child = Command::new(executable)
            .args(std::env::args_os().skip(1))
            .env(WORKER_CHILD_VAR, fd.to_string())
            .spawn()?;
        drop(writer);
        Ok((child, reader))
    })();
    let (mut child, mut reader) = match spawned {
        Ok(spawned) => spawned,
        Err(error) => {
            emit_cli_error(CliError::ElaborationWorkerSpawn(error));
            return 1;
        }
    };
    let status = match child.wait() {
        Ok(status) => status,
        Err(error) => {
            emit_cli_error(CliError::ElaborationWorkerSpawn(error));
            return 1;
        }
    };
    let mut faults = Vec::new();
    let _ = reader.read_to_end(&mut faults);
    if let Some(code) = status.code() {
        return code;
    }
    let signal = status.signal().unwrap_or(libc::SIGABRT);
    if signal == libc::SIGABRT && !faults.is_empty() {
        emit_cli_error(CliError::ElaborationWorkerStackOverflow);
        return 1;
    }
    // Not the runtime's overflow abort: terminate with the child's signal.
    // SAFETY: restoring the default disposition and raising in this
    // single-threaded parent has no other effect.
    unsafe {
        libc::signal(signal, libc::SIG_DFL);
        libc::raise(signal);
    }
    128 + signal
}

static FAULT_FD: AtomicI32 = AtomicI32::new(-1);
static PREVIOUS_SEGV: OnceLock<libc::sigaction> = OnceLock::new();
static PREVIOUS_BUS: OnceLock<libc::sigaction> = OnceLock::new();

/// Child: forget the marker, keep the side channel from user subprocesses,
/// and chain a recorder in front of the runtime's fault handler. Call before
/// the worker thread is spawned.
pub(crate) fn enter_child() {
    let fd = std::env::var_os(WORKER_CHILD_VAR);
    std::env::remove_var(WORKER_CHILD_VAR);
    let Some(fd) = fd
        .as_deref()
        .and_then(OsStr::to_str)
        .and_then(|s| s.parse::<RawFd>().ok())
    else {
        return;
    };
    // SAFETY: plain descriptor-flag and signal-disposition calls on this
    // process before any other thread exists.
    unsafe {
        libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC);
        FAULT_FD.store(fd, Ordering::Relaxed);
        install(libc::SIGSEGV, &PREVIOUS_SEGV);
        install(libc::SIGBUS, &PREVIOUS_BUS);
    }
}

unsafe fn install(signal: libc::c_int, previous: &OnceLock<libc::sigaction>) {
    let mut old: libc::sigaction = std::mem::zeroed();
    if libc::sigaction(signal, std::ptr::null(), &mut old) != 0 {
        return;
    }
    let _ = previous.set(old);
    let mut new: libc::sigaction = std::mem::zeroed();
    new.sa_sigaction = record_fault_then_chain as *const () as usize;
    new.sa_flags = libc::SA_SIGINFO | libc::SA_ONSTACK;
    libc::sigemptyset(&mut new.sa_mask);
    libc::sigaction(signal, &new, std::ptr::null_mut());
}

extern "C" fn record_fault_then_chain(
    signal: libc::c_int,
    info: *mut libc::siginfo_t,
    context: *mut libc::c_void,
) {
    // SAFETY: write(2) and sigaction(2) are async-signal-safe; the previous
    // action was stored before this handler was installed.
    unsafe {
        let fd = FAULT_FD.load(Ordering::Relaxed);
        libc::write(fd, b"F".as_ptr().cast(), 1);
        let previous = match signal {
            libc::SIGSEGV => PREVIOUS_SEGV.get(),
            _ => PREVIOUS_BUS.get(),
        };
        match previous {
            Some(old)
                if old.sa_flags & libc::SA_SIGINFO != 0
                    && old.sa_sigaction != libc::SIG_DFL
                    && old.sa_sigaction != libc::SIG_IGN =>
            {
                type Action = extern "C" fn(libc::c_int, *mut libc::siginfo_t, *mut libc::c_void);
                let handler = std::mem::transmute::<libc::sighandler_t, Action>(old.sa_sigaction);
                handler(signal, info, context);
            }
            _ => {
                libc::signal(signal, libc::SIG_DFL);
            }
        }
    }
}
