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

/// The child's inherited channels: `<fault fd>:<parent pid>:<fault dev>:
/// <fault ino>:<lifeline fd>:<lifeline dev>:<lifeline ino>`.
const WORKER_CHILD_VAR: &str = "KEN_CLI_WORKER_CHILD_FAULT_FD";

pub(crate) enum Role {
    /// The marker names two pipes this process holds.
    Child { fault: RawFd, lifeline: RawFd },
    /// No marker, or one naming pipes this process does not hold and not
    /// written by its parent.
    Parent,
    /// This process's parent wrote a marker naming pipes it does not hold.
    ProtocolFault,
}

/// Remove the marker before any child process starts. Holding both named
/// pipes authenticates this child even when its parent died before `role()`.
pub(crate) fn role() -> Role {
    let marker = std::env::var_os(WORKER_CHILD_VAR);
    std::env::remove_var(WORKER_CHILD_VAR);
    let Some(marker) = marker.as_deref().and_then(OsStr::to_str) else {
        return Role::Parent;
    };
    let fields: Vec<&str> = marker.split(':').collect();
    let [fault, pid, fault_dev, fault_ino, lifeline, life_dev, life_ino] = fields[..] else {
        return Role::Parent;
    };
    let (
        Ok(fault),
        Ok(pid),
        Ok(fault_dev),
        Ok(fault_ino),
        Ok(lifeline),
        Ok(life_dev),
        Ok(life_ino),
    ) = (
        fault.parse::<RawFd>(),
        pid.parse::<libc::pid_t>(),
        fault_dev.parse::<u64>(),
        fault_ino.parse::<u64>(),
        lifeline.parse::<RawFd>(),
        life_dev.parse::<u64>(),
        life_ino.parse::<u64>(),
    )
    else {
        return Role::Parent;
    };
    // Holding both named pipes is the child evidence; no descendant inherits
    // them. A dead parent changes getppid but closes the lifeline writer.
    if pipe_identity(fault) == Some((fault_dev, fault_ino))
        && pipe_identity(lifeline) == Some((life_dev, life_ino))
    {
        return Role::Child { fault, lifeline };
    }
    // SAFETY: getppid(2) has no preconditions and cannot fail.
    if pid == unsafe { libc::getppid() } {
        Role::ProtocolFault
    } else {
        Role::Parent
    }
}

/// `(st_dev, st_ino)` of `fd` when it is an open FIFO.
fn pipe_identity(fd: RawFd) -> Option<(u64, u64)> {
    // SAFETY: fstat(2) fills the zeroed local buffer or fails on a bad fd.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    if unsafe { libc::fstat(fd, &mut stat) } != 0 || stat.st_mode & libc::S_IFMT != libc::S_IFIFO {
        return None;
    }
    Some((stat.st_dev as u64, stat.st_ino as u64))
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
        // The parent owns the lifeline writer for all of run_parent. Exec of
        // the worker child closes that writer; only its reader is inherited.
        let mut life_fds = [0 as RawFd; 2];
        let (lifeline_reader, lifeline_writer) = unsafe {
            if libc::pipe(life_fds.as_mut_ptr()) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            let read = std::fs::File::from_raw_fd(life_fds[0]);
            let write = std::fs::File::from_raw_fd(life_fds[1]);
            if libc::fcntl(life_fds[1], libc::F_SETFD, libc::FD_CLOEXEC) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            (read, write)
        };
        let fd = writer.as_raw_fd();
        let (dev, ino) = pipe_identity(fd)
            .ok_or_else(|| std::io::Error::other("the worker pipe has no FIFO identity"))?;
        let life_fd = lifeline_reader.as_raw_fd();
        let (life_dev, life_ino) = pipe_identity(life_fd)
            .ok_or_else(|| std::io::Error::other("the lifeline has no FIFO identity"))?;
        let marker = format!(
            "{fd}:{}:{dev}:{ino}:{life_fd}:{life_dev}:{life_ino}",
            std::process::id()
        );
        let child = Command::new(executable)
            .args(std::env::args_os().skip(1))
            .env(WORKER_CHILD_VAR, marker)
            .spawn()?;
        drop(writer);
        drop(lifeline_reader);
        Ok((child, reader, lifeline_writer))
    })();
    let (mut child, mut reader, _lifeline_writer) = match spawned {
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

/// Child: keep both channels from user subprocesses; chain the fault
/// recorder and start the lifeline watcher before the worker thread spawns.
pub(crate) fn enter_child(fault: RawFd, lifeline: RawFd) {
    // SAFETY: descriptor flags and handler dispositions are set before any
    // child thread exists, including the watcher started below.
    unsafe {
        libc::fcntl(fault, libc::F_SETFD, libc::FD_CLOEXEC);
        libc::fcntl(lifeline, libc::F_SETFD, libc::FD_CLOEXEC);
        FAULT_FD.store(fault, Ordering::Relaxed);
        install(libc::SIGSEGV, &PREVIOUS_SEGV);
        install(libc::SIGBUS, &PREVIOUS_BUS);
    }
    // The parent never writes the lifeline. EOF or an error means its only
    // writer closed; a signal interruption is retried, not misclassified.
    std::thread::spawn(move || {
        let mut byte = 0u8;
        loop {
            // SAFETY: a one-byte read into a local buffer.
            let n = unsafe { libc::read(lifeline, (&mut byte as *mut u8).cast(), 1) };
            if n == -1 && std::io::Error::last_os_error().raw_os_error() == Some(libc::EINTR) {
                continue;
            }
            break;
        }
        // SAFETY: kill(2) on this process's own pid.
        unsafe { libc::kill(libc::getpid(), libc::SIGKILL) };
    });
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
