//! Linux stream-socket leaves. Every descriptor is acquired atomically close-on-exec.
//! Address validation precedes the first socket syscall; a failed connect drops
//! the newly created descriptor. No resolver or socket option enters this slice.

#![allow(unsafe_code)]

use std::io;
use std::mem::{offset_of, size_of};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SocketAddressV1 {
    Inet4 { octets: [u8; 4], port: u16 },
    Inet6 { octets: [u8; 16], port: u16 },
    Unix { path: Vec<u8> },
}

impl SocketAddressV1 {
    pub fn validate(&self) -> io::Result<()> {
        if let Self::Unix { path } = self {
            if !(2..=107).contains(&path.len()) || path.first() != Some(&b'/') || path.contains(&0)
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Unix socket path must be absolute, 2–107 bytes, and NUL-free",
                ));
            }
        }
        Ok(())
    }

    pub fn is_loopback(&self) -> bool {
        match self {
            Self::Inet4 { octets, .. } => octets[0] == 127,
            Self::Inet6 { octets, .. } => {
                *octets == [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
            }
            Self::Unix { .. } => false,
        }
    }

    fn as_sockaddr(&self) -> io::Result<(libc::sockaddr_storage, libc::socklen_t)> {
        self.validate()?;
        // `sockaddr_storage` has the size and alignment of every supported
        // sockaddr; each write initializes only the family-specific prefix.
        let mut storage: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
        let length = match self {
            Self::Inet4 { octets, port } => {
                let address = libc::sockaddr_in {
                    sin_family: libc::AF_INET as libc::sa_family_t,
                    sin_port: port.to_be(),
                    sin_addr: libc::in_addr {
                        s_addr: u32::from_ne_bytes(*octets),
                    },
                    sin_zero: [0; 8],
                };
                unsafe {
                    (std::ptr::from_mut(&mut storage) as *mut libc::sockaddr_in).write(address)
                };
                size_of::<libc::sockaddr_in>()
            }
            Self::Inet6 { octets, port } => {
                let address = libc::sockaddr_in6 {
                    sin6_family: libc::AF_INET6 as libc::sa_family_t,
                    sin6_port: port.to_be(),
                    sin6_flowinfo: 0,
                    sin6_addr: libc::in6_addr { s6_addr: *octets },
                    sin6_scope_id: 0,
                };
                unsafe {
                    (std::ptr::from_mut(&mut storage) as *mut libc::sockaddr_in6).write(address)
                };
                size_of::<libc::sockaddr_in6>()
            }
            Self::Unix { path } => {
                let mut address: libc::sockaddr_un = unsafe { std::mem::zeroed() };
                address.sun_family = libc::AF_UNIX as libc::sa_family_t;
                for (slot, byte) in address.sun_path.iter_mut().zip(path) {
                    *slot = *byte as libc::c_char;
                }
                unsafe {
                    (std::ptr::from_mut(&mut storage) as *mut libc::sockaddr_un).write(address)
                };
                offset_of!(libc::sockaddr_un, sun_path) + path.len() + 1
            }
        };
        Ok((storage, length as libc::socklen_t))
    }

    fn from_sockaddr(
        storage: &libc::sockaddr_storage,
        length: libc::socklen_t,
    ) -> io::Result<Self> {
        let invalid =
            || io::Error::new(io::ErrorKind::InvalidData, "malformed local socket address");
        match storage.ss_family as libc::c_int {
            libc::AF_INET if length as usize >= size_of::<libc::sockaddr_in>() => {
                let addr = unsafe { &*(storage as *const _ as *const libc::sockaddr_in) };
                Ok(Self::Inet4 {
                    octets: addr.sin_addr.s_addr.to_ne_bytes(),
                    port: u16::from_be(addr.sin_port),
                })
            }
            libc::AF_INET6 if length as usize >= size_of::<libc::sockaddr_in6>() => {
                let addr = unsafe { &*(storage as *const _ as *const libc::sockaddr_in6) };
                if addr.sin6_scope_id != 0 || addr.sin6_flowinfo != 0 {
                    return Err(invalid());
                }
                Ok(Self::Inet6 {
                    octets: addr.sin6_addr.s6_addr,
                    port: u16::from_be(addr.sin6_port),
                })
            }
            libc::AF_UNIX => {
                let base = offset_of!(libc::sockaddr_un, sun_path);
                if (length as usize) < base + 2
                    || (length as usize) > size_of::<libc::sockaddr_un>()
                {
                    return Err(invalid());
                }
                let addr = unsafe { &*(storage as *const _ as *const libc::sockaddr_un) };
                let bytes = &addr.sun_path[..(length as usize - base)];
                let end = bytes.iter().position(|b| *b == 0).ok_or_else(invalid)?;
                let value = Self::Unix {
                    path: bytes[..end].iter().map(|b| *b as u8).collect(),
                };
                value.validate().map_err(|_| invalid())?;
                Ok(value)
            }
            _ => Err(invalid()),
        }
    }
}

fn socket(address: &SocketAddressV1) -> io::Result<OwnedFd> {
    address.validate()?;
    let family = match address {
        SocketAddressV1::Inet4 { .. } => libc::AF_INET,
        SocketAddressV1::Inet6 { .. } => libc::AF_INET6,
        SocketAddressV1::Unix { .. } => libc::AF_UNIX,
    };
    let fd = unsafe { libc::socket(family, libc::SOCK_STREAM | libc::SOCK_CLOEXEC, 0) };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(unsafe { OwnedFd::from_raw_fd(fd) })
}

pub fn listen(address: &SocketAddressV1) -> io::Result<OwnedFd> {
    let (raw, length) = address.as_sockaddr()?;
    let fd = socket(address)?;
    if unsafe {
        libc::bind(
            fd.as_raw_fd(),
            &raw as *const _ as *const libc::sockaddr,
            length,
        )
    } != 0
    {
        return Err(io::Error::last_os_error());
    }
    if unsafe { libc::listen(fd.as_raw_fd(), libc::SOMAXCONN) } != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(fd)
}

pub fn accept(listener: &OwnedFd) -> io::Result<OwnedFd> {
    let fd = unsafe {
        libc::accept4(
            listener.as_raw_fd(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            libc::SOCK_CLOEXEC,
        )
    };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(unsafe { OwnedFd::from_raw_fd(fd) })
}

pub fn connect(address: &SocketAddressV1) -> io::Result<OwnedFd> {
    let (raw, length) = address.as_sockaddr()?;
    let fd = socket(address)?;
    if unsafe {
        libc::connect(
            fd.as_raw_fd(),
            &raw as *const _ as *const libc::sockaddr,
            length,
        )
    } != 0
    {
        return Err(io::Error::last_os_error());
    }
    Ok(fd)
}

pub fn receive(stream: &OwnedFd, buffer: &mut [u8]) -> io::Result<usize> {
    let count = unsafe {
        libc::recv(
            stream.as_raw_fd(),
            buffer.as_mut_ptr().cast(),
            buffer.len(),
            0,
        )
    };
    if count < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(count as usize)
    }
}

pub fn send(stream: &OwnedFd, buffer: &[u8]) -> io::Result<usize> {
    let count = unsafe {
        libc::send(
            stream.as_raw_fd(),
            buffer.as_ptr().cast(),
            buffer.len(),
            libc::MSG_NOSIGNAL,
        )
    };
    if count < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(count as usize)
    }
}

pub fn shutdown(stream: &OwnedFd, how: u64) -> io::Result<()> {
    let direction = match how {
        0 => libc::SHUT_RD,
        1 => libc::SHUT_WR,
        2 => libc::SHUT_RDWR,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid socket shutdown direction",
            ));
        }
    };
    if unsafe { libc::shutdown(stream.as_raw_fd(), direction) } != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

pub fn local_address(socket: &OwnedFd) -> io::Result<SocketAddressV1> {
    let mut storage: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
    let mut length = size_of::<libc::sockaddr_storage>() as libc::socklen_t;
    if unsafe {
        libc::getsockname(
            socket.as_raw_fd(),
            &mut storage as *mut _ as *mut libc::sockaddr,
            &mut length,
        )
    } != 0
    {
        return Err(io::Error::last_os_error());
    }
    SocketAddressV1::from_sockaddr(&storage, length)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_cloexec(fd: &OwnedFd) {
        let flags = unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_GETFD) };
        assert!(
            flags >= 0 && flags & libc::FD_CLOEXEC != 0,
            "socket must be close-on-exec"
        );
    }

    #[test]
    fn three_socket_families_exchange_bounded_bytes_and_keep_cloexec() {
        let dir = std::env::temp_dir().join(format!(
            "ken-px11-net-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir(&dir).unwrap();
        let unix_path = dir.join("s").into_os_string().into_encoded_bytes();
        for address in [
            SocketAddressV1::Inet4 {
                octets: [127, 0, 0, 1],
                port: 0,
            },
            SocketAddressV1::Inet6 {
                octets: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                port: 0,
            },
            SocketAddressV1::Unix {
                path: unix_path.clone(),
            },
        ] {
            let listener = listen(&address).unwrap();
            assert_cloexec(&listener);
            let assigned = local_address(&listener).unwrap();
            if !matches!(address, SocketAddressV1::Unix { .. }) {
                assert!(matches!(
                    assigned,
                    SocketAddressV1::Inet4 { port: 1.., .. }
                        | SocketAddressV1::Inet6 { port: 1.., .. }
                ));
            }
            let outbound = connect(&assigned).unwrap();
            assert_cloexec(&outbound);
            let accepted = accept(&listener).unwrap();
            assert_cloexec(&accepted);
            assert_eq!(send(&outbound, b"abcdef").unwrap(), 6);
            let mut received = [0u8; 3];
            assert_eq!(receive(&accepted, &mut received).unwrap(), 3);
            assert_eq!(&received, b"abc");
            assert_eq!(send(&accepted, b"xyz").unwrap(), 3);
            let mut reverse = [0u8; 3];
            assert_eq!(receive(&outbound, &mut reverse).unwrap(), 3);
            assert_eq!(&reverse, b"xyz");
            shutdown(&outbound, 1).unwrap();
            let mut remainder = [0u8; 4];
            assert_eq!(receive(&accepted, &mut remainder).unwrap(), 3);
            assert_eq!(&remainder[..3], b"def");
            assert_eq!(receive(&accepted, &mut remainder).unwrap(), 0);
        }
        std::fs::remove_file(dir.join("s")).unwrap();
        std::fs::remove_dir(dir).unwrap();
    }

    /// Promise class: durable invariant. MEASURED: three live socket
    /// descriptors have kernel socket inode links, but a PX10-spawned child
    /// blocked on its stdin pipe exposes none of those links in /proc/pid/fd.
    /// CLAIMED: a child spawned while a listener and two streams are open
    /// inherits none of them. THE GAP: PX10's closefrom(3) also protects this
    /// path; the independent F_GETFD row detects missing atomic CLOEXEC.
    #[test]
    fn px11_live_px10_child_has_no_parent_socket_descriptors() {
        use crate::process_v1::{ChildStdioV1, SpawnPlanV1};

        let listener = listen(&SocketAddressV1::Inet4 {
            octets: [127, 0, 0, 1],
            port: 0,
        })
        .unwrap();
        let connected = connect(&local_address(&listener).unwrap()).unwrap();
        let accepted = accept(&listener).unwrap();
        let socket_inodes = [&listener, &connected, &accepted]
            .into_iter()
            .map(|fd| {
                std::fs::read_link(format!("/proc/self/fd/{}", fd.as_raw_fd()))
                    .expect("held socket descriptor has a proc link")
            })
            .collect::<Vec<_>>();
        assert!(socket_inodes
            .iter()
            .all(|link| link.to_string_lossy().starts_with("socket:[")));
        assert_eq!(
            socket_inodes
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            3,
            "the listener and both streams have distinct descriptors"
        );

        let mut plan = SpawnPlanV1::new(b"/bin/sh".to_vec()).unwrap();
        plan.push_argument(b"/bin/sh".to_vec()).unwrap();
        plan.push_argument(b"-c".to_vec()).unwrap();
        plan.push_argument(b"read child_ready".to_vec()).unwrap();
        plan.set_stdio(0, ChildStdioV1::Pipe).unwrap();
        let mut child = plan.spawn().expect("PX10 child spawn");
        let parent_pipe = child.take_pipe(0).unwrap();
        let fds = std::fs::read_dir(format!("/proc/{}/fd", child.pid()))
            .expect("child remains alive on its stdin pipe")
            .map(|entry| {
                std::fs::read_link(entry.unwrap().path())
                    .expect("read child descriptor while blocked")
            })
            .collect::<Vec<_>>();
        for inode in &socket_inodes {
            assert!(
                !fds.contains(inode),
                "PX10 child inherited a parent's live socket {inode:?}"
            );
        }
        drop(parent_pipe);
        child.wait().expect("reap child after releasing stdin");
    }

    #[test]
    fn invalid_unix_path_is_rejected_before_a_socket_exists() {
        for path in [
            vec![],
            vec![b'/'],
            vec![b'a', b'b'],
            vec![b'/', 0],
            vec![b'/'; 108],
        ] {
            assert_eq!(
                listen(&SocketAddressV1::Unix { path: path.clone() })
                    .unwrap_err()
                    .kind(),
                io::ErrorKind::InvalidInput
            );
            assert_eq!(
                connect(&SocketAddressV1::Unix { path }).unwrap_err().kind(),
                io::ErrorKind::InvalidInput
            );
        }
        for len in [2, 107] {
            SocketAddressV1::Unix {
                path: vec![b'/'; len],
            }
            .validate()
            .unwrap();
        }
    }
}
