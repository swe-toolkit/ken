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
    /// `socket(AF_INET, SOCK_DGRAM | SOCK_CLOEXEC, 0)`.
    DatagramSocketInet4,
    /// `socket(AF_INET6, SOCK_DGRAM | SOCK_CLOEXEC, 0)`.
    DatagramSocketInet6,
    /// `socket(AF_UNIX, SOCK_DGRAM | SOCK_CLOEXEC, 0)`.
    DatagramSocketUnix,
}

/// The verdicts decided so far. Production has one, for the whole process.
struct VerdictCacheV1 {
    verdicts: Mutex<Vec<(HostFacilityV1, bool)>>,
    #[cfg(test)]
    probes_run: Mutex<Vec<(HostFacilityV1, usize)>>,
}

static VERDICTS: VerdictCacheV1 = VerdictCacheV1::new();

impl VerdictCacheV1 {
    const fn new() -> Self {
        Self {
            verdicts: Mutex::new(Vec::new()),
            #[cfg(test)]
            probes_run: Mutex::new(Vec::new()),
        }
    }

    fn require(&self, facility: HostFacilityV1) -> io::Result<()> {
        let mut verdicts = self.verdicts.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(&(_, present)) = verdicts.iter().find(|(known, _)| *known == facility) {
            return if present { Ok(()) } else { Err(unsupported()) };
        }
        #[cfg(test)]
        self.count_probe(facility);
        let present = decide(facility, probe(facility))?;
        verdicts.push((facility, present));
        if present { Ok(()) } else { Err(unsupported()) }
    }
}

/// `Ok` while `facility` is present. `Unsupported` while its probe has found
/// it absent. The probe's own error when the probe could not decide.
pub(crate) fn require(facility: HostFacilityV1) -> io::Result<()> {
    #[cfg(test)]
    if let Some(scope) = FAULT_SCOPE.with(std::cell::Cell::get) {
        return scope.cache.require(facility);
    }
    VERDICTS.require(facility)
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
        StreamSocketInet4 | StreamSocketInet6 | StreamSocketUnix | DatagramSocketInet4
        | DatagramSocketInet6 | DatagramSocketUnix => matches!(
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
        Pidfd => observed(Pidfd, pidfd_probe()),
        Statx => observed(
            Statx,
            rustix::fs::statx(
                rustix::fs::CWD,
                "/",
                rustix::fs::AtFlags::empty(),
                rustix::fs::StatxFlags::TYPE,
            )
            .map(|_| ())
            .map_err(io::Error::from),
        ),
        ProcSelfFd => observed(ProcSelfFd, proc_self_fd_probe()),
        StreamSocketInet4 => observed(StreamSocketInet4, stream_socket_probe(libc::AF_INET)),
        StreamSocketInet6 => observed(StreamSocketInet6, stream_socket_probe(libc::AF_INET6)),
        StreamSocketUnix => observed(StreamSocketUnix, stream_socket_probe(libc::AF_UNIX)),
        Accept4 => observed(Accept4, accept4_probe()),
        DatagramSocketInet4 => observed(
            DatagramSocketInet4,
            socket_probe(libc::AF_INET, libc::SOCK_DGRAM),
        ),
        DatagramSocketInet6 => observed(
            DatagramSocketInet6,
            socket_probe(libc::AF_INET6, libc::SOCK_DGRAM),
        ),
        DatagramSocketUnix => observed(
            DatagramSocketUnix,
            socket_probe(libc::AF_UNIX, libc::SOCK_DGRAM),
        ),
    }
}

/// A probe arm's syscall result, which a test fault scope may replace.
fn observed(facility: HostFacilityV1, outcome: io::Result<()>) -> io::Result<()> {
    #[cfg(test)]
    if let Some(errno) = scripted_probe_errno(facility) {
        return Err(io::Error::from_raw_os_error(errno));
    }
    let _ = facility;
    outcome
}

fn pidfd_probe() -> io::Result<()> {
    #[cfg(target_env = "gnu")]
    {
        crate::process_v1::probe_pidfd_v1()
    }
    #[cfg(not(target_env = "gnu"))]
    {
        Err(io::Error::from_raw_os_error(libc::ENOSYS))
    }
}

fn proc_self_fd_probe() -> io::Result<()> {
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

fn accept4_probe() -> io::Result<()> {
    // The kernel refuses a bad descriptor with EBADF only when the call
    // itself is admitted; nothing is accepted or created.
    // SAFETY: null address arguments are permitted.
    unsafe { libc::accept4(-1, std::ptr::null_mut(), std::ptr::null_mut(), libc::SOCK_CLOEXEC) };
    let error = io::Error::last_os_error();
    if error.raw_os_error() == Some(libc::EBADF) { Ok(()) } else { Err(error) }
}

fn stream_socket_probe(family: libc::c_int) -> io::Result<()> {
    socket_probe(family, libc::SOCK_STREAM)
}

fn socket_probe(family: libc::c_int, kind: libc::c_int) -> io::Result<()> {
    // An unbound socket, closed at once: no address, no traffic.
    // SAFETY: socket returns a fresh descriptor on success, owned here.
    let fd = unsafe { libc::socket(family, kind | libc::SOCK_CLOEXEC, 0) };
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

/// The `/proc/<pid>/fd` spelling of one of this process's descriptors, with
/// the pid as `/proc` itself names this process. A datagram Unix bind records
/// this spelling as its name, so it must not resolve per reader.
pub(crate) fn proc_pid_fd_path(fd: impl std::os::fd::AsFd) -> io::Result<String> {
    require(HostFacilityV1::ProcSelfFd)?;
    let pid = std::fs::read_link("/proc/self")?;
    let pid = pid.to_str().filter(|pid| !pid.is_empty() && pid.bytes().all(|byte| byte.is_ascii_digit()))
        .ok_or_else(|| io::Error::from_raw_os_error(libc::ENOENT))?
        .to_owned();
    Ok(format!("/proc/{pid}/fd/{}", fd.as_fd().as_raw_fd()))
}

/// A test's private verdict cache plus the scripted results of one
/// facility's probes. Leaked so the thread-local can hold a plain reference.
#[cfg(test)]
struct FaultScope {
    facility: HostFacilityV1,
    script: std::cell::RefCell<std::collections::VecDeque<Option<i32>>>,
    cache: VerdictCacheV1,
}

#[cfg(test)]
std::thread_local! {
    static FAULT_SCOPE: std::cell::Cell<Option<&'static FaultScope>> =
        const { std::cell::Cell::new(None) };
}

#[cfg(test)]
fn scripted_probe_errno(facility: HostFacilityV1) -> Option<i32> {
    let scope = FAULT_SCOPE.with(std::cell::Cell::get)?;
    if scope.facility != facility {
        return None;
    }
    scope.script.borrow_mut().pop_front().flatten()
}

#[cfg(test)]
impl VerdictCacheV1 {
    fn count_probe(&self, facility: HostFacilityV1) {
        let mut runs = self.probes_run.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        match runs.iter_mut().find(|(known, _)| *known == facility) {
            Some((_, count)) => *count += 1,
            None => runs.push((facility, 1)),
        }
    }

    fn probes_run(&self, facility: HostFacilityV1) -> usize {
        self.probes_run
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .iter()
            .find(|(known, _)| *known == facility)
            .map_or(0, |(_, count)| *count)
    }
}

/// Probes of `facility` the process-wide cache has run.
#[cfg(test)]
pub(crate) fn probes_run_for_test(facility: HostFacilityV1) -> usize {
    VERDICTS.probes_run(facility)
}

/// Run `body` on this thread against a fresh verdict cache, through the
/// production lookup, probe arm, classification and insertion. The n-th probe
/// of `facility` in the scope takes `script[n]`: `Some(errno)` replaces that
/// probe arm's syscall result, and `None` or a spent script lets the syscall
/// answer. Returns `body`'s value and how many probes of `facility` ran.
#[cfg(test)]
pub(crate) fn with_facility_script<T>(
    facility: HostFacilityV1,
    script: &[Option<i32>],
    body: impl FnOnce() -> T,
) -> (T, usize) {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            FAULT_SCOPE.with(|scope| scope.set(None));
        }
    }
    let scope: &'static FaultScope = Box::leak(Box::new(FaultScope {
        facility,
        script: std::cell::RefCell::new(script.iter().copied().collect()),
        cache: VerdictCacheV1::new(),
    }));
    FAULT_SCOPE.with(|slot| {
        assert!(slot.get().is_none());
        slot.set(Some(scope));
    });
    let _restore = Restore;
    let value = body();
    (value, scope.cache.probes_run(facility))
}

/// Run `body` with `facility`'s first probe reporting `errno`, on this thread
/// only, against a fresh verdict cache.
#[cfg(test)]
pub(crate) fn with_facility_fault<T>(
    facility: HostFacilityV1,
    errno: i32,
    body: impl FnOnce() -> T,
) -> T {
    with_facility_script(facility, &[Some(errno)], body).0
}

#[cfg(test)]
mod tests;
