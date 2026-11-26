//! Linux stream-socket leaves. Every descriptor is acquired atomically close-on-exec.
//! Address validation precedes the first socket syscall; a failed connect drops
//! the newly created descriptor. No resolver or socket option enters this slice.

#![allow(unsafe_code)]

use std::io;
use std::mem::{offset_of, size_of};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::ffi::OsStrExt;

use crate::{FsHandle, IoErrorIdentityV1, OpenRequest, PathComponent, RootedHandle};

type NetResult<T> = Result<T, IoErrorIdentityV1>;

fn io_identity(error: io::Error) -> IoErrorIdentityV1 {
    crate::io_error_identity_v1(&error)
}

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
    crate::facility_v1::require(match address {
        SocketAddressV1::Inet4 { .. } => crate::facility_v1::HostFacilityV1::StreamSocketInet4,
        SocketAddressV1::Inet6 { .. } => crate::facility_v1::HostFacilityV1::StreamSocketInet6,
        SocketAddressV1::Unix { .. } => crate::facility_v1::HostFacilityV1::StreamSocketUnix,
    })?;
    let fd = unsafe { libc::socket(family, libc::SOCK_STREAM | libc::SOCK_CLOEXEC, 0) };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(unsafe { OwnedFd::from_raw_fd(fd) })
}

// Only the IP arms reach this raw bind; Unix must resolve beneath the pinned root.
fn listen_ip(address: &SocketAddressV1) -> io::Result<OwnedFd> {
    let (raw, length) = address.as_sockaddr()?;
    let fd = socket(address)?;
    bind_and_listen(fd, &raw, length)
}

fn bind_and_listen(
    fd: OwnedFd,
    raw: &libc::sockaddr_storage,
    length: libc::socklen_t,
) -> io::Result<OwnedFd> {
    if unsafe {
        libc::bind(
            fd.as_raw_fd(),
            raw as *const _ as *const libc::sockaddr,
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

fn unix_parent(root: &RootedHandle, path: &[u8]) -> NetResult<(RootedHandle, Vec<u8>)> {
    let root_link = std::fs::read_link(
        crate::facility_v1::proc_self_fd_path(root.as_fd()).map_err(io_identity)?,
    )
    .map_err(|_| IoErrorIdentityV1::CapabilityDenied)?;
    let spelling = root_link.as_os_str().as_bytes();
    if !spelling.starts_with(b"/") || spelling.ends_with(b" (deleted)") {
        return Err(IoErrorIdentityV1::CapabilityDenied);
    }
    if path == spelling {
        return Err(IoErrorIdentityV1::InvalidInput);
    }
    let rest = if spelling == b"/" {
        path.strip_prefix(b"/".as_slice())
    } else {
        path.strip_prefix(spelling)
            .and_then(|suffix| suffix.strip_prefix(b"/".as_slice()))
    }
    .ok_or(IoErrorIdentityV1::CapabilityDenied)?;
    if rest.is_empty() || rest.ends_with(b"/") || rest.ends_with(b"/.") {
        return Err(IoErrorIdentityV1::InvalidInput);
    }
    let mut components = rest
        .split(|byte| *byte == b'/')
        .filter(|part| !part.is_empty() && *part != b".")
        .map(|part| {
            if part == b".." {
                return Err(IoErrorIdentityV1::CapabilityDenied);
            }
            PathComponent::new(part)
                .map_err(|error| io_identity(error.into_io_error()))
                .map(|_| part)
        })
        .collect::<NetResult<Vec<_>>>()?;
    let leaf = components.pop().ok_or(IoErrorIdentityV1::InvalidInput)?;
    let mut parent = root.clone();
    for name in components {
        let component =
            PathComponent::new(name).map_err(|error| io_identity(error.into_io_error()))?;
        if crate::readlink_at(&parent, &component).is_ok() {
            return Err(IoErrorIdentityV1::CapabilityDenied);
        }
        parent =
            crate::open_at(&parent, &component, OpenRequest::ReadDirectory).map_err(|error| {
                let error = error.into_io_error();
                if error.raw_os_error() == Some(libc::ELOOP) {
                    IoErrorIdentityV1::CapabilityDenied
                } else {
                    io_identity(error)
                }
            })?;
    }
    Ok((parent, leaf.to_vec()))
}

fn deny_symlink_leaf(parent: &RootedHandle, leaf: &[u8]) -> NetResult<()> {
    let component = PathComponent::new(leaf).map_err(|error| io_identity(error.into_io_error()))?;
    if crate::readlink_at(parent, &component).is_ok() {
        Err(IoErrorIdentityV1::CapabilityDenied)
    } else {
        Ok(())
    }
}

fn listen_unix(root: &RootedHandle, path: &[u8]) -> NetResult<OwnedFd> {
    let (parent, leaf) = unix_parent(root, path)?;
    // /proc/self/fd/ (14 bytes), at most ten decimal fd digits, slash,
    // and 82 leaf bytes fit Linux's 107-byte terminated sun_path limit.
    if leaf.len() > 82 {
        return Err(IoErrorIdentityV1::InvalidInput);
    }
    deny_symlink_leaf(&parent, &leaf)?;
    let mut proc_path = crate::facility_v1::proc_self_fd_path(parent.as_fd())
        .map_err(io_identity)?
        .into_bytes();
    proc_path.push(b'/');
    proc_path.extend_from_slice(&leaf);
    let resolved = SocketAddressV1::Unix { path: proc_path };
    let (raw, length) = resolved.as_sockaddr().map_err(io_identity)?;
    let fd = socket(&resolved).map_err(io_identity)?;
    bind_and_listen(fd, &raw, length).map_err(io_identity)
}

fn connect_unix(root: &RootedHandle, path: &[u8]) -> NetResult<OwnedFd> {
    let (parent, leaf) = unix_parent(root, path)?;
    deny_symlink_leaf(&parent, &leaf)?;
    let name = std::ffi::CString::new(leaf).expect("validated path component has no NUL");
    let raw_fd = unsafe {
        libc::openat(
            parent.as_fd().as_raw_fd(),
            name.as_ptr(),
            libc::O_PATH | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if raw_fd < 0 {
        return Err(io_identity(io::Error::last_os_error()));
    }
    let pinned_leaf = unsafe { OwnedFd::from_raw_fd(raw_fd) };
    let mut metadata: libc::stat = unsafe { std::mem::zeroed() };
    if unsafe { libc::fstat(pinned_leaf.as_raw_fd(), &mut metadata) } != 0 {
        return Err(io_identity(io::Error::last_os_error()));
    }
    if metadata.st_mode & libc::S_IFMT == libc::S_IFLNK {
        return Err(IoErrorIdentityV1::CapabilityDenied);
    }
    let resolved = SocketAddressV1::Unix {
        path: crate::facility_v1::proc_self_fd_path(&pinned_leaf)
            .map_err(io_identity)?
            .into_bytes(),
    };
    let (raw, length) = resolved.as_sockaddr().map_err(io_identity)?;
    let fd = socket(&resolved).map_err(io_identity)?;
    connect_raw(fd, &raw, length).map_err(io_identity)
}

pub fn listen(root: &FsHandle, address: &SocketAddressV1) -> NetResult<OwnedFd> {
    address.validate().map_err(io_identity)?;
    match address {
        SocketAddressV1::Inet4 { .. } | SocketAddressV1::Inet6 { .. } => {
            listen_ip(address).map_err(io_identity)
        }
        SocketAddressV1::Unix { path } => match root {
            FsHandle::Posix(root) => listen_unix(root, path),
            FsHandle::Virtual(_) => Err(IoErrorIdentityV1::CapabilityDenied),
        },
    }
}

pub fn accept(listener: &OwnedFd) -> io::Result<OwnedFd> {
    crate::facility_v1::require(crate::facility_v1::HostFacilityV1::Accept4)?;
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

// Only the IP arms reach this raw connect; Unix pins the leaf before socket().
fn connect_ip(address: &SocketAddressV1) -> io::Result<OwnedFd> {
    let (raw, length) = address.as_sockaddr()?;
    let fd = socket(address)?;
    connect_raw(fd, &raw, length)
}

fn connect_raw(
    fd: OwnedFd,
    raw: &libc::sockaddr_storage,
    length: libc::socklen_t,
) -> io::Result<OwnedFd> {
    if unsafe {
        libc::connect(
            fd.as_raw_fd(),
            raw as *const _ as *const libc::sockaddr,
            length,
        )
    } != 0
    {
        return Err(io::Error::last_os_error());
    }
    Ok(fd)
}

pub fn connect(root: &FsHandle, address: &SocketAddressV1) -> NetResult<OwnedFd> {
    address.validate().map_err(io_identity)?;
    match address {
        SocketAddressV1::Inet4 { .. } | SocketAddressV1::Inet6 { .. } => {
            connect_ip(address).map_err(io_identity)
        }
        SocketAddressV1::Unix { path } => match root {
            FsHandle::Posix(root) => connect_unix(root, path),
            FsHandle::Virtual(_) => Err(IoErrorIdentityV1::CapabilityDenied),
        },
    }
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
        let canonical_dir = std::fs::canonicalize(&dir).unwrap();
        let root = crate::FsHandle::Posix(
            crate::open_root(&crate::RootPath::new(&canonical_dir).unwrap()).unwrap(),
        );
        let unix_path = canonical_dir
            .join("s")
            .into_os_string()
            .into_encoded_bytes();
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
            let listener = listen(&root, &address).unwrap();
            assert_cloexec(&listener);
            let assigned = if matches!(address, SocketAddressV1::Unix { .. }) {
                address.clone()
            } else {
                local_address(&listener).unwrap()
            };
            if !matches!(address, SocketAddressV1::Unix { .. }) {
                assert!(matches!(
                    assigned,
                    SocketAddressV1::Inet4 { port: 1.., .. }
                        | SocketAddressV1::Inet6 { port: 1.., .. }
                ));
            }
            let outbound = connect(&root, &assigned).unwrap();
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

        let root = crate::FsHandle::Virtual(0);
        let listener = listen(
            &root,
            &SocketAddressV1::Inet4 {
                octets: [127, 0, 0, 1],
                port: 0,
            },
        )
        .unwrap();
        let connected = connect(&root, &local_address(&listener).unwrap()).unwrap();
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

    /// Promise class: durable invariant. MEASURED: the rooted Unix dispatcher
    /// distinguishes valid pinning, invalid leaf length, missing parent,
    /// non-directory parent, and occupied listener name. CLAIMED: confinement
    /// does not conflate path/input errors with capability denial. THE GAP:
    /// this is a host-unit boundary; the CLI cases exercise checked Ken.
    #[test]
    fn rooted_unix_errors_preserve_path_and_existing_name_identity() {
        let dir = std::env::temp_dir().join(format!(
            "ken-px11-root-errors-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir(&dir).unwrap();
        let dir = std::fs::canonicalize(dir).unwrap();
        let root =
            crate::FsHandle::Posix(crate::open_root(&crate::RootPath::new(&dir).unwrap()).unwrap());
        let address = |path: std::path::PathBuf| SocketAddressV1::Unix {
            path: path.into_os_string().into_encoded_bytes(),
        };
        assert_eq!(
            listen(
                &crate::FsHandle::Virtual(0),
                &address(dir.join("virtual.sock"))
            )
            .unwrap_err(),
            IoErrorIdentityV1::CapabilityDenied
        );
        assert_eq!(
            listen(&root, &address(dir.clone())).unwrap_err(),
            IoErrorIdentityV1::InvalidInput
        );
        assert_eq!(
            connect(&root, &address(dir.join("."))).unwrap_err(),
            IoErrorIdentityV1::InvalidInput
        );
        assert_eq!(
            connect(&root, &address(dir.join("missing/svc.sock"))).unwrap_err(),
            IoErrorIdentityV1::NotFound
        );
        std::fs::write(dir.join("file"), b"not a directory").unwrap();
        assert_eq!(
            connect(&root, &address(dir.join("file/svc.sock"))).unwrap_err(),
            IoErrorIdentityV1::NotDirectory
        );
        let slash_root =
            crate::FsHandle::Posix(crate::open_root(&crate::RootPath::new("/").unwrap()).unwrap());
        let long_leaf = SocketAddressV1::Unix {
            path: [b"/".as_slice(), vec![b'x'; 83].as_slice()].concat(),
        };
        long_leaf
            .validate()
            .expect("full path fits the original sun_path bound");
        assert_eq!(
            listen(&slash_root, &long_leaf).unwrap_err(),
            IoErrorIdentityV1::InvalidInput
        );
        let slash_socket = address(dir.join("slash.sock"));
        let _from_slash = listen(&slash_root, &slash_socket).unwrap();
        std::fs::remove_file(dir.join("slash.sock")).unwrap();
        let socket = address(dir.join("occupied.sock"));
        let _first = listen(&root, &socket).unwrap();
        assert_eq!(
            listen(&root, &socket).unwrap_err(),
            IoErrorIdentityV1::Other(98)
        );
        std::fs::remove_file(dir.join("occupied.sock")).unwrap();
        std::fs::remove_file(dir.join("file")).unwrap();
        std::fs::remove_dir(dir).unwrap();
    }

    /// Promise class: durable invariant. MEASURED: an opened root whose name
    /// was deleted, or a virtual root, refuses Unix acquisition on both arms.
    /// CLAIMED: an unreadable/deleted root cannot regain path authority.
    /// THE GAP: this checks deletion and virtuality, not every readlink error.
    #[test]
    fn deleted_and_virtual_roots_refuse_both_unix_acquisitions() {
        let dir = std::env::temp_dir().join(format!(
            "ken-px11-deleted-root-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir(&dir).unwrap();
        let dir = std::fs::canonicalize(dir).unwrap();
        let root =
            crate::FsHandle::Posix(crate::open_root(&crate::RootPath::new(&dir).unwrap()).unwrap());
        let address = SocketAddressV1::Unix {
            path: dir
                .join("absent.sock")
                .into_os_string()
                .into_encoded_bytes(),
        };
        std::fs::remove_dir(&dir).unwrap();
        for root in [&root, &crate::FsHandle::Virtual(0)] {
            assert_eq!(
                listen(root, &address).unwrap_err(),
                IoErrorIdentityV1::CapabilityDenied
            );
            assert_eq!(
                connect(root, &address).unwrap_err(),
                IoErrorIdentityV1::CapabilityDenied
            );
        }
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
                listen(
                    &crate::FsHandle::Virtual(0),
                    &SocketAddressV1::Unix { path: path.clone() }
                )
                .unwrap_err(),
                IoErrorIdentityV1::InvalidInput
            );
            assert_eq!(
                connect(
                    &crate::FsHandle::Virtual(0),
                    &SocketAddressV1::Unix { path }
                )
                .unwrap_err(),
                IoErrorIdentityV1::InvalidInput
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
