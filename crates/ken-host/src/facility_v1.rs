//! Host facilities a supported Linux host can lack or a policy can deny, each
//! probed at most once per process (spec 38 §1.8 (3), which also states which
//! operations need which facility).
//!
//! A probe performs none of an operation's effects. Its verdict is cached only
//! when it is decisive: present, or absent by an errno that means absence. Any
//! other probe failure (descriptor exhaustion, memory) settles nothing; the
//! operation fails with that error and the next need probes again.
#![allow(unsafe_code)]

use std::io;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::sync::Mutex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HostFacilityV1 {
    /// `pidfd_open`, `pidfd_send_signal` and `waitid(P_PIDFD)`.
    Pidfd,
    /// `statx`.
    Statx,
    /// The `/proc/self/fd` view of this process's own descriptors.
    ProcSelfFd,
    /// `socket(AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0)`.
    StreamSocketInet4,
    /// `socket(AF_INET6, SOCK_STREAM | SOCK_CLOEXEC, 0)`.
    StreamSocketInet6,
    /// `socket(AF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0)`.
    StreamSocketUnix,
    /// `accept4` with `SOCK_CLOEXEC`.
    Accept4,
}

static VERDICTS: Mutex<Vec<(HostFacilityV1, bool)>> = Mutex::new(Vec::new());

/// `Ok` while `facility` is present. `Unsupported` while its probe has found
/// it absent. The probe's own error when the probe could not decide.
pub(crate) fn require(facility: HostFacilityV1) -> io::Result<()> {
    #[cfg(test)]
    if let Some(errno) = injected_probe_errno(facility) {
        // The seam replaces the probe's syscall result; the classification is
        // the production one, and the cache is neither read nor written.
        return if decide(facility, Err(io::Error::from_raw_os_error(errno)))? {
            Ok(())
        } else {
            Err(unsupported())
        };
    }
    let mut verdicts = VERDICTS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(&(_, present)) = verdicts.iter().find(|(known, _)| *known == facility) {
        return if present { Ok(()) } else { Err(unsupported()) };
    }
    #[cfg(test)]
    count_probe(facility);
    let present = decide(facility, probe(facility))?;
    verdicts.push((facility, present));
    if present { Ok(()) } else { Err(unsupported()) }
}

/// `Ok(true)` present, `Ok(false)` absent, `Err` undecided. An absent verdict
/// is returned as `Ok(false)` so the caller caches it before refusing.
fn decide(facility: HostFacilityV1, outcome: io::Result<()>) -> io::Result<bool> {
    match outcome {
        Ok(()) => Ok(true),
        Err(error) => match error.raw_os_error() {
            Some(errno) if means_absent(facility, errno) => Ok(false),
            _ => Err(error),
        },
    }
}

fn means_absent(facility: HostFacilityV1, errno: i32) -> bool {
    use HostFacilityV1::*;
    match facility {
        Pidfd | Statx | Accept4 => {
            matches!(errno, libc::ENOSYS | libc::EINVAL | libc::EPERM | libc::EACCES)
        }
        ProcSelfFd => matches!(
            errno,
            libc::ENOENT | libc::ENOTDIR | libc::ENOSYS | libc::EPERM | libc::EACCES
        ),
        StreamSocketInet4 | StreamSocketInet6 | StreamSocketUnix => matches!(
            errno,
            libc::ENOSYS
                | libc::EINVAL
                | libc::EPERM
                | libc::EACCES
                | libc::EAFNOSUPPORT
                | libc::EPROTONOSUPPORT
        ),
    }
}

fn probe(facility: HostFacilityV1) -> io::Result<()> {
    use HostFacilityV1::*;
    match facility {
        Pidfd => {
            #[cfg(target_env = "gnu")]
            {
                crate::process_v1::probe_pidfd_v1()
            }
            #[cfg(not(target_env = "gnu"))]
            {
                Err(io::Error::from_raw_os_error(libc::ENOSYS))
            }
        }
        Statx => rustix::fs::statx(
            rustix::fs::CWD,
            "/",
            rustix::fs::AtFlags::empty(),
            rustix::fs::StatxFlags::TYPE,
        )
        .map(|_| ())
        .map_err(io::Error::from),
        ProcSelfFd => {
            use rustix::fs::{Mode, OFlags};
            let root = rustix::fs::open("/", OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC, Mode::empty())?;
            let link = std::fs::read_link(format!("/proc/self/fd/{}", root.as_raw_fd()))?;
            if link.as_os_str() == "/" {
                Ok(())
            } else {
                // A /proc that does not describe this process is as absent as
                // no /proc at all.
                Err(io::Error::from_raw_os_error(libc::ENOENT))
            }
        }
        StreamSocketInet4 => stream_socket_probe(libc::AF_INET),
        StreamSocketInet6 => stream_socket_probe(libc::AF_INET6),
        StreamSocketUnix => stream_socket_probe(libc::AF_UNIX),
        Accept4 => {
            // The kernel refuses a bad descriptor with EBADF only when the
            // call itself is admitted; nothing is accepted or created.
            // SAFETY: null address arguments are permitted.
            unsafe {
                libc::accept4(-1, std::ptr::null_mut(), std::ptr::null_mut(), libc::SOCK_CLOEXEC)
            };
            let error = io::Error::last_os_error();
            if error.raw_os_error() == Some(libc::EBADF) { Ok(()) } else { Err(error) }
        }
    }
}

fn stream_socket_probe(family: libc::c_int) -> io::Result<()> {
    // An unbound socket, closed at once: no address, no traffic.
    // SAFETY: socket returns a fresh descriptor on success, owned here.
    let fd = unsafe { libc::socket(family, libc::SOCK_STREAM | libc::SOCK_CLOEXEC, 0) };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }
    drop(unsafe { OwnedFd::from_raw_fd(fd) });
    Ok(())
}

fn unsupported() -> io::Error {
    io::Error::from(io::ErrorKind::Unsupported)
}

/// The one spelling of a `/proc/self/fd` path. Every use of the view goes
/// through here, so the facility is required before the path exists.
pub(crate) fn proc_self_fd_path(fd: impl std::os::fd::AsFd) -> io::Result<String> {
    require(HostFacilityV1::ProcSelfFd)?;
    Ok(format!("/proc/self/fd/{}", fd.as_fd().as_raw_fd()))
}

#[cfg(test)]
std::thread_local! {
    static FACILITY_FAULT: std::cell::Cell<Option<(HostFacilityV1, i32)>> =
        const { std::cell::Cell::new(None) };
}

#[cfg(test)]
static PROBES_RUN: Mutex<Vec<(HostFacilityV1, usize)>> = Mutex::new(Vec::new());

#[cfg(test)]
fn injected_probe_errno(facility: HostFacilityV1) -> Option<i32> {
    FACILITY_FAULT.with(|fault| fault.get().and_then(|(at, errno)| (at == facility).then_some(errno)))
}

#[cfg(test)]
fn count_probe(facility: HostFacilityV1) {
    let mut runs = PROBES_RUN.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    match runs.iter_mut().find(|(known, _)| *known == facility) {
        Some((_, count)) => *count += 1,
        None => runs.push((facility, 1)),
    }
}

#[cfg(test)]
pub(crate) fn probes_run_for_test(facility: HostFacilityV1) -> usize {
    PROBES_RUN
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .iter()
        .find(|(known, _)| *known == facility)
        .map_or(0, |(_, count)| *count)
}

/// Run `body` with `facility`'s probe reporting `errno`, on this thread only.
#[cfg(test)]
pub(crate) fn with_facility_fault<T>(
    facility: HostFacilityV1,
    errno: i32,
    body: impl FnOnce() -> T,
) -> T {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            FACILITY_FAULT.with(|fault| fault.set(None));
        }
    }
    FACILITY_FAULT.with(|fault| {
        assert!(fault.get().is_none());
        fault.set(Some((facility, errno)));
    });
    let _restore = Restore;
    body()
}

#[cfg(test)]
mod tests;
