//! Linux-gnu child creation through glibc's `posix_spawn` actions only.
//! No Rust or Ken callback runs between fork and exec.

#![allow(unsafe_code)]

use std::ffi::{CStr, CString};
use std::io;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildStdioV1 {
    Pipe,
    Null,
    Inherit(u8),
}

#[derive(Clone, Debug)]
pub struct SpawnPlanV1 {
    program: Vec<u8>,
    arguments: Vec<Vec<u8>>,
    environment: Vec<(Vec<u8>, Vec<u8>)>,
    stdio: [Option<ChildStdioV1>; 3],
}

impl SpawnPlanV1 {
    pub fn new(program: Vec<u8>) -> io::Result<Self> {
        checked_cstring(&program)?;
        Ok(Self {
            program,
            arguments: Vec::new(),
            environment: Vec::new(),
            stdio: [None; 3],
        })
    }

    pub fn push_argument(&mut self, argument: Vec<u8>) -> io::Result<()> {
        checked_cstring(&argument)?;
        self.arguments.push(argument);
        Ok(())
    }

    pub fn push_environment(&mut self, key: Vec<u8>, value: Vec<u8>) -> io::Result<()> {
        if key.is_empty() || key.contains(&b'=') {
            return Err(invalid_input());
        }
        checked_cstring(&key)?;
        checked_cstring(&value)?;
        self.environment.push((key, value));
        Ok(())
    }

    pub fn set_stdio(&mut self, target: u8, stdio: ChildStdioV1) -> io::Result<()> {
        let entry = self
            .stdio
            .get_mut(target as usize)
            .ok_or_else(invalid_input)?;
        if entry.is_some() {
            return Err(invalid_input());
        }
        if let ChildStdioV1::Inherit(source) = stdio {
            if source > 2 {
                return Err(invalid_input());
            }
        }
        *entry = Some(stdio);
        Ok(())
    }

    pub fn program(&self) -> &[u8] {
        &self.program
    }

    pub fn spawn(self) -> io::Result<ChildProcessV1> {
        if self.arguments.is_empty() {
            return Err(invalid_input());
        }
        let program = checked_cstring(&self.program)?;
        let arguments = self
            .arguments
            .iter()
            .map(|argument| checked_cstring(argument))
            .collect::<io::Result<Vec<_>>>()?;
        let environment = self
            .environment
            .iter()
            .map(|(key, value)| {
                let mut entry = key.clone();
                entry.push(b'=');
                entry.extend(value);
                checked_cstring(&entry)
            })
            .collect::<io::Result<Vec<_>>>()?;
        let mut argv = arguments
            .iter()
            .map(|value| value.as_ptr())
            .collect::<Vec<_>>();
        argv.push(std::ptr::null());
        let mut envp = environment
            .iter()
            .map(|value| value.as_ptr())
            .collect::<Vec<_>>();
        envp.push(std::ptr::null());

        let mut snapshots: [Option<OwnedFd>; 3] = std::array::from_fn(|_| None);
        for (fd, target) in self.stdio.iter().enumerate() {
            if let Some(ChildStdioV1::Inherit(source)) = target {
                snapshots[fd] = Some(duplicate_cloexec_at_least_3(*source as RawFd)?);
            }
        }
        let mut child_ends: [Option<OwnedFd>; 3] = std::array::from_fn(|_| None);
        let mut parent_ends: [Option<OwnedFd>; 3] = std::array::from_fn(|_| None);
        for (fd, target) in self.stdio.iter().enumerate() {
            if *target == Some(ChildStdioV1::Pipe) {
                let mut pair = [-1; 2];
                // SAFETY: pair is writable and `pipe2` initializes both fds on success.
                if unsafe { libc::pipe2(pair.as_mut_ptr(), libc::O_CLOEXEC) } == -1 {
                    return Err(io::Error::last_os_error());
                }
                // SAFETY: a successful pipe2 transfers two unique owned descriptors.
                let read = unsafe { OwnedFd::from_raw_fd(pair[0]) };
                let write = unsafe { OwnedFd::from_raw_fd(pair[1]) };
                let (parent, child) = if fd == 0 {
                    (write, read)
                } else {
                    (read, write)
                };
                // Keep every file-action source above the three stdio targets.
                child_ends[fd] = Some(duplicate_cloexec_at_least_3(child.as_raw_fd())?);
                parent_ends[fd] = Some(duplicate_cloexec_at_least_3(parent.as_raw_fd())?);
            }
        }

        let mut actions = SpawnActionsV1::new()?;
        for fd in 0..3 {
            match self.stdio[fd].unwrap_or(ChildStdioV1::Null) {
                ChildStdioV1::Pipe => {
                    actions.dup2(child_ends[fd].as_ref().unwrap().as_raw_fd(), fd as RawFd)?;
                }
                ChildStdioV1::Null => {
                    let null = c"/dev/null";
                    let flags = if fd == 0 {
                        libc::O_RDONLY
                    } else {
                        libc::O_WRONLY
                    };
                    actions.open(fd as RawFd, null, flags)?;
                }
                ChildStdioV1::Inherit(_) => {
                    actions.dup2(snapshots[fd].as_ref().unwrap().as_raw_fd(), fd as RawFd)?;
                }
            }
        }
        actions.close_from_3()?;
        let attributes = SpawnAttributesV1::new_sigpipe_default()?;
        let mut pid: libc::pid_t = 0;
        // SAFETY: all pointers are live through the synchronous call and both
        // action/attribute values were initialized by their glibc constructors.
        let rc = unsafe {
            libc::posix_spawn(
                &mut pid,
                program.as_ptr(),
                &actions.0,
                &attributes.0,
                argv.as_ptr() as *const *mut libc::c_char,
                envp.as_ptr() as *const *mut libc::c_char,
            )
        };
        if rc != 0 {
            return Err(io::Error::from_raw_os_error(rc));
        }
        Ok(ChildProcessV1 {
            pid,
            exit: None,
            pending: parent_ends,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildExitV1 {
    Exited(u8),
    Signaled(u8),
}

#[derive(Debug)]
pub struct ChildProcessV1 {
    pid: libc::pid_t,
    exit: Option<ChildExitV1>,
    pending: [Option<OwnedFd>; 3],
}

impl ChildProcessV1 {
    pub fn pid(&self) -> libc::pid_t {
        self.pid
    }

    pub fn take_pipe(&mut self, fd: u8) -> io::Result<OwnedFd> {
        self.pending
            .get_mut(fd as usize)
            .and_then(Option::take)
            .ok_or_else(invalid_input)
    }

    pub fn wait(&mut self) -> io::Result<ChildExitV1> {
        if let Some(exit) = self.exit {
            return Ok(exit);
        }
        let mut status: libc::c_int = 0;
        loop {
            // SAFETY: valid pid from successful posix_spawn, writable status pointer.
            let result = unsafe { libc::waitpid(self.pid, &mut status, 0) };
            if result == -1 {
                let error = io::Error::last_os_error();
                if error.kind() == io::ErrorKind::Interrupted {
                    continue;
                }
                return Err(error);
            }
            break;
        }
        let exit = if libc::WIFEXITED(status) {
            ChildExitV1::Exited(libc::WEXITSTATUS(status) as u8)
        } else if libc::WIFSIGNALED(status) {
            ChildExitV1::Signaled(libc::WTERMSIG(status) as u8)
        } else {
            return Err(io::Error::from_raw_os_error(libc::ECHILD));
        };
        self.exit = Some(exit);
        Ok(exit)
    }

    pub fn settle(mut self, terminal_error: bool) -> io::Result<()> {
        // All untaken pipe ends close before the child is reaped.
        self.pending = std::array::from_fn(|_| None);
        if self.exit.is_none() && terminal_error {
            // SAFETY: the positive pid is owned by this child resource.
            if unsafe { libc::kill(self.pid, libc::SIGKILL) } == -1 {
                let error = io::Error::last_os_error();
                if error.raw_os_error() != Some(libc::ESRCH) {
                    return Err(error);
                }
            }
        }
        self.wait().map(|_| ())
    }
}

fn invalid_input() -> io::Error {
    io::Error::from(io::ErrorKind::InvalidInput)
}
fn checked_cstring(bytes: &[u8]) -> io::Result<CString> {
    CString::new(bytes).map_err(|_| invalid_input())
}
fn duplicate_cloexec_at_least_3(fd: RawFd) -> io::Result<OwnedFd> {
    // SAFETY: fcntl returns a fresh descriptor on success, owned here.
    let result = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 3) };
    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(unsafe { OwnedFd::from_raw_fd(result) })
    }
}

struct SpawnActionsV1(libc::posix_spawn_file_actions_t);
impl SpawnActionsV1 {
    fn new() -> io::Result<Self> {
        let mut value = std::mem::MaybeUninit::uninit();
        // SAFETY: glibc initializes value on success.
        check_spawn(unsafe { libc::posix_spawn_file_actions_init(value.as_mut_ptr()) })?;
        Ok(Self(unsafe { value.assume_init() }))
    }
    fn dup2(&mut self, from: RawFd, to: RawFd) -> io::Result<()> {
        check_spawn(unsafe { libc::posix_spawn_file_actions_adddup2(&mut self.0, from, to) })
    }
    fn open(&mut self, fd: RawFd, path: &CStr, flags: i32) -> io::Result<()> {
        check_spawn(unsafe {
            libc::posix_spawn_file_actions_addopen(&mut self.0, fd, path.as_ptr(), flags, 0)
        })
    }
    fn close_from_3(&mut self) -> io::Result<()> {
        check_spawn(unsafe { libc::posix_spawn_file_actions_addclosefrom_np(&mut self.0, 3) })
    }
}
impl Drop for SpawnActionsV1 {
    fn drop(&mut self) {
        unsafe { libc::posix_spawn_file_actions_destroy(&mut self.0) };
    }
}

struct SpawnAttributesV1(libc::posix_spawnattr_t);
impl SpawnAttributesV1 {
    fn new_sigpipe_default() -> io::Result<Self> {
        let mut value = std::mem::MaybeUninit::uninit();
        check_spawn(unsafe { libc::posix_spawnattr_init(value.as_mut_ptr()) })?;
        let mut value = Self(unsafe { value.assume_init() });
        let mut defaults = unsafe { std::mem::zeroed() };
        if unsafe { libc::sigemptyset(&mut defaults) } == -1
            || unsafe { libc::sigaddset(&mut defaults, libc::SIGPIPE) } == -1
        {
            return Err(io::Error::last_os_error());
        }
        check_spawn(unsafe { libc::posix_spawnattr_setsigdefault(&mut value.0, &defaults) })?;
        check_spawn(unsafe {
            libc::posix_spawnattr_setflags(&mut value.0, libc::POSIX_SPAWN_SETSIGDEF as i16)
        })?;
        Ok(value)
    }
}
impl Drop for SpawnAttributesV1 {
    fn drop(&mut self) {
        unsafe { libc::posix_spawnattr_destroy(&mut self.0) };
    }
}
fn check_spawn(rc: i32) -> io::Result<()> {
    if rc == 0 {
        Ok(())
    } else {
        Err(io::Error::from_raw_os_error(rc))
    }
}

#[cfg(test)]
mod tests;
