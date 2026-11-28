//! Linux stream-socket leaves. Every descriptor is acquired atomically close-on-exec.
//! Address validation precedes the first socket syscall; a failed connect drops
//! the newly created descriptor. No resolver enters this slice, and the only
//! socket options are the typed families of spec 38 §1.13.2.

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

/// Whether a stream-socket resource is IP (IPv4 or IPv6) or Unix. Every
/// listener and stream resource records it at mint from its checked
/// address, and an accepted stream copies its listener's record. The stream
/// option fit check reads only this record, never the descriptor (spec 38
/// §1.13.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SocketFamilyV1 {
    Ip,
    Unix,
}

impl SocketFamilyV1 {
    pub fn of(address: &SocketAddressV1) -> Self {
        match address {
            SocketAddressV1::Inet4 { .. } | SocketAddressV1::Inet6 { .. } => Self::Ip,
            SocketAddressV1::Unix { .. } => Self::Unix,
        }
    }
}

/// One settable stream option (spec 38 §1.13.2). Each names one fixed
/// kernel option; there is no integer `(level, optname)` form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamOptionV1 {
    /// `TCP_NODELAY`, code 0.
    NoDelay,
    /// `SO_KEEPALIVE`, code 1.
    KeepAlive,
}

impl StreamOptionV1 {
    /// The option for a wire code, or `None` for a code above 1.
    pub const fn from_code(code: u64) -> Option<Self> {
        match code {
            0 => Some(Self::NoDelay),
            1 => Some(Self::KeepAlive),
            _ => None,
        }
    }

    pub const fn code(self) -> u64 {
        match self {
            Self::NoDelay => 0,
            Self::KeepAlive => 1,
        }
    }

    const fn level_and_name(self) -> (libc::c_int, libc::c_int) {
        match self {
            Self::NoDelay => (libc::IPPROTO_TCP, libc::TCP_NODELAY),
            Self::KeepAlive => (libc::SOL_SOCKET, libc::SO_KEEPALIVE),
        }
    }
}

fn set_int_option(
    fd: &OwnedFd,
    level: libc::c_int,
    name: libc::c_int,
    value: libc::c_int,
) -> io::Result<()> {
    if unsafe {
        libc::setsockopt(
            fd.as_raw_fd(),
            level,
            name,
            &value as *const libc::c_int as *const libc::c_void,
            size_of::<libc::c_int>() as libc::socklen_t,
        )
    } != 0
    {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

/// Bind and listen with `SO_REUSEADDR` set after `socket()` and before
/// `bind()`; otherwise identical to [`listen`]. The executor refuses a Unix
/// address before this leaf (spec 38 §1.13.2), so the Unix arm here is the
/// plain Unix listen: the option has no bind effect on `AF_UNIX`.
pub fn listen_reusable(root: &FsHandle, address: &SocketAddressV1) -> NetResult<OwnedFd> {
    address.validate().map_err(io_identity)?;
    match address {
        SocketAddressV1::Inet4 { .. } | SocketAddressV1::Inet6 { .. } => (|| {
            let (raw, length) = address.as_sockaddr()?;
            let fd = socket(address)?;
            set_int_option(&fd, libc::SOL_SOCKET, libc::SO_REUSEADDR, 1)?;
            bind_and_listen(fd, &raw, length)
        })()
        .map_err(io_identity),
        SocketAddressV1::Unix { .. } => listen(root, address),
    }
}

/// Set a stream option to `enabled` with one `setsockopt`.
pub fn set_stream_option(
    stream: &OwnedFd,
    option: StreamOptionV1,
    enabled: bool,
) -> io::Result<()> {
    let (level, name) = option.level_and_name();
    set_int_option(stream, level, name, libc::c_int::from(enabled))
}

/// Read a stream option's kernel value with one `getsockopt`; `true`
/// exactly when it is nonzero. Never a cached copy.
pub fn stream_option(stream: &OwnedFd, option: StreamOptionV1) -> io::Result<bool> {
    let (level, name) = option.level_and_name();
    let mut value: libc::c_int = 0;
    let mut length = size_of::<libc::c_int>() as libc::socklen_t;
    if unsafe {
        libc::getsockopt(
            stream.as_raw_fd(),
            level,
            name,
            &mut value as *mut libc::c_int as *mut libc::c_void,
            &mut length,
        )
    } != 0
    {
        return Err(io::Error::last_os_error());
    }
    Ok(value != 0)
}

/// The largest datagram payload, in bytes, for every family: the IPv4 UDP
/// maximum, applied uniformly (spec 38 §1.13.1).
pub const DATAGRAM_PAYLOAD_MAX_V1: usize = 65_507;

/// One live datagram socket's exact kernel-recorded Unix name and the
/// validated address it was bound with.
struct BoundDatagramNameV1 {
    serial: u64,
    name: Vec<u8>,
    address: SocketAddressV1,
}

/// The per-process table from each live Unix datagram socket's exact bound
/// name to its validated address. An entry lives from bind to release; a
/// released entry's name and address are kept for the process lifetime,
/// because a datagram it sent can still be queued when a later bind reuses
/// the same descriptor number and leaf.
static BOUND_DATAGRAM_NAMES: std::sync::Mutex<DatagramNameTableV1> =
    std::sync::Mutex::new((0, Vec::new(), Vec::new()));

type DatagramNameTableV1 = (u64, Vec<BoundDatagramNameV1>, Vec<(Vec<u8>, SocketAddressV1)>);

fn bound_datagram_names() -> std::sync::MutexGuard<'static, DatagramNameTableV1> {
    BOUND_DATAGRAM_NAMES
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// A bound datagram socket. Every datagram socket is bound: bind is its only
/// acquisition. A Unix socket keeps its pinned parent directory open until
/// release, so the descriptor number inside its kernel-recorded name cannot
/// be reused by another live bind in this process.
pub struct DatagramSocketV1 {
    fd: OwnedFd,
    address: SocketAddressV1,
    registration: Option<u64>,
    _parent: Option<RootedHandle>,
}

impl std::fmt::Debug for DatagramSocketV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DatagramSocketV1")
            .field("fd", &self.fd)
            .field("address", &self.address)
            .field("registration", &self.registration)
            .finish_non_exhaustive()
    }
}

impl Drop for DatagramSocketV1 {
    fn drop(&mut self) {
        if let Some(serial) = self.registration.take() {
            let mut names = bound_datagram_names();
            if let Some(index) = names.1.iter().position(|entry| entry.serial == serial) {
                let entry = names.1.remove(index);
                if !names.2.iter().any(|(name, address)| {
                    *name == entry.name && *address == entry.address
                }) {
                    names.2.push((entry.name, entry.address));
                }
            }
        }
    }
}

impl DatagramSocketV1 {
    /// The validated address this socket was bound with. An IP port of zero
    /// stays zero here; [`datagram_local_address`] reads the assigned port.
    pub fn bound_address(&self) -> &SocketAddressV1 {
        &self.address
    }

    #[cfg(test)]
    pub(crate) fn as_raw_fd_for_test(&self) -> std::os::fd::RawFd {
        self.fd.as_raw_fd()
    }
}

/// Whether two addresses are of the same socket family.
pub fn same_family(left: &SocketAddressV1, right: &SocketAddressV1) -> bool {
    std::mem::discriminant(left) == std::mem::discriminant(right)
}

fn datagram_socket(address: &SocketAddressV1) -> io::Result<OwnedFd> {
    address.validate()?;
    let family = match address {
        SocketAddressV1::Inet4 { .. } => libc::AF_INET,
        SocketAddressV1::Inet6 { .. } => libc::AF_INET6,
        SocketAddressV1::Unix { .. } => libc::AF_UNIX,
    };
    crate::facility_v1::require(match address {
        SocketAddressV1::Inet4 { .. } => crate::facility_v1::HostFacilityV1::DatagramSocketInet4,
        SocketAddressV1::Inet6 { .. } => crate::facility_v1::HostFacilityV1::DatagramSocketInet6,
        SocketAddressV1::Unix { .. } => crate::facility_v1::HostFacilityV1::DatagramSocketUnix,
    })?;
    let fd = unsafe { libc::socket(family, libc::SOCK_DGRAM | libc::SOCK_CLOEXEC, 0) };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(unsafe { OwnedFd::from_raw_fd(fd) })
}

fn bind_raw(fd: OwnedFd, raw: &libc::sockaddr_storage, length: libc::socklen_t) -> io::Result<OwnedFd> {
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
    Ok(fd)
}

fn datagram_bind_unix(
    root: &RootedHandle,
    address: &SocketAddressV1,
    path: &[u8],
) -> NetResult<DatagramSocketV1> {
    let (parent, leaf) = unix_parent(root, path)?;
    // /proc/ (6 bytes), at most seven pid digits, /fd/ (4), at most ten fd
    // digits and a slash leave 79 leaf bytes within the 107-byte sun_path.
    if leaf.len() > 79 {
        return Err(IoErrorIdentityV1::InvalidInput);
    }
    deny_symlink_leaf(&parent, &leaf)?;
    // The bind spelling names this process by its pid, not `/proc/self`, so
    // the name the kernel records is unique among live processes.
    let mut name = crate::facility_v1::proc_pid_fd_path(parent.as_fd())
        .map_err(io_identity)?
        .into_bytes();
    name.push(b'/');
    name.extend_from_slice(&leaf);
    let resolved = SocketAddressV1::Unix { path: name.clone() };
    let (raw, length) = resolved.as_sockaddr().map_err(io_identity)?;
    let fd = datagram_socket(&resolved).map_err(io_identity)?;
    let fd = bind_raw(fd, &raw, length).map_err(io_identity)?;
    let mut names = bound_datagram_names();
    names.0 += 1;
    let serial = names.0;
    names.1.push(BoundDatagramNameV1 {
        serial,
        name,
        address: address.clone(),
    });
    drop(names);
    Ok(DatagramSocketV1 {
        fd,
        address: address.clone(),
        registration: Some(serial),
        _parent: Some(parent),
    })
}

/// Bind one blocking datagram socket, with no options set. The Unix arm
/// resolves beneath the grant's root by the stream listener's rules and
/// leaves no inode when it refuses. A failure after `socket()` closes it.
pub fn datagram_bind(root: &FsHandle, address: &SocketAddressV1) -> NetResult<DatagramSocketV1> {
    address.validate().map_err(io_identity)?;
    match address {
        SocketAddressV1::Inet4 { .. } | SocketAddressV1::Inet6 { .. } => {
            let (raw, length) = address.as_sockaddr().map_err(io_identity)?;
            let fd = datagram_socket(address).map_err(io_identity)?;
            let fd = bind_raw(fd, &raw, length).map_err(io_identity)?;
            Ok(DatagramSocketV1 {
                fd,
                address: address.clone(),
                registration: None,
                _parent: None,
            })
        }
        SocketAddressV1::Unix { path } => match root {
            FsHandle::Posix(root) => datagram_bind_unix(root, address, path),
            FsHandle::Virtual(_) => Err(IoErrorIdentityV1::CapabilityDenied),
        },
    }
}

fn send_to_raw(
    socket: &DatagramSocketV1,
    raw: &libc::sockaddr_storage,
    length: libc::socklen_t,
    bytes: &[u8],
) -> io::Result<()> {
    let count = unsafe {
        libc::sendto(
            socket.fd.as_raw_fd(),
            bytes.as_ptr().cast(),
            bytes.len(),
            libc::MSG_NOSIGNAL,
            raw as *const _ as *const libc::sockaddr,
            length,
        )
    };
    if count < 0 {
        return Err(io::Error::last_os_error());
    }
    // A datagram is sent whole or not at all; a short count is not a state
    // this operation can report as success.
    if count as usize != bytes.len() {
        return Err(io::Error::from_raw_os_error(libc::EMSGSIZE));
    }
    Ok(())
}

/// Send `bytes` as one datagram to `peer`. The caller has admitted the peer
/// against the socket's grant, its family and the payload bound. A Unix peer
/// is admitted by the stream connect rule and addressed through its pinned
/// leaf, a spelling that is never observable.
pub fn datagram_send_to(
    socket: &DatagramSocketV1,
    root: &FsHandle,
    peer: &SocketAddressV1,
    bytes: &[u8],
) -> NetResult<()> {
    peer.validate().map_err(io_identity)?;
    match peer {
        SocketAddressV1::Inet4 { .. } | SocketAddressV1::Inet6 { .. } => {
            let (raw, length) = peer.as_sockaddr().map_err(io_identity)?;
            send_to_raw(socket, &raw, length, bytes).map_err(io_identity)
        }
        SocketAddressV1::Unix { path } => {
            let FsHandle::Posix(root) = root else {
                return Err(IoErrorIdentityV1::CapabilityDenied);
            };
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
            send_to_raw(socket, &raw, length, bytes).map_err(io_identity)
        }
    }
}

/// The address a received datagram's sender is reported as, or `None` when
/// it has no identity in this slice's `SocketAddress`.
fn datagram_sender(
    storage: &libc::sockaddr_storage,
    length: libc::socklen_t,
) -> Option<SocketAddressV1> {
    let length = length as usize;
    match storage.ss_family as libc::c_int {
        libc::AF_INET if length >= size_of::<libc::sockaddr_in>() => {
            let addr = unsafe { &*(storage as *const _ as *const libc::sockaddr_in) };
            Some(SocketAddressV1::Inet4 {
                octets: addr.sin_addr.s_addr.to_ne_bytes(),
                port: u16::from_be(addr.sin_port),
            })
        }
        libc::AF_INET6 if length >= size_of::<libc::sockaddr_in6>() => {
            let addr = unsafe { &*(storage as *const _ as *const libc::sockaddr_in6) };
            // Flowinfo is not part of the address identity; a scoped
            // address has no representation here.
            (addr.sin6_scope_id == 0).then(|| SocketAddressV1::Inet6 {
                octets: addr.sin6_addr.s6_addr,
                port: u16::from_be(addr.sin6_port),
            })
        }
        libc::AF_UNIX => {
            let base = offset_of!(libc::sockaddr_un, sun_path);
            if length <= base || length > size_of::<libc::sockaddr_un>() {
                return None;
            }
            let addr = unsafe { &*(storage as *const _ as *const libc::sockaddr_un) };
            let raw = &addr.sun_path[..length - base];
            if raw.first() == Some(&0) {
                return None;
            }
            let end = raw.iter().position(|byte| *byte == 0).unwrap_or(raw.len());
            let name = raw[..end].iter().map(|byte| *byte as u8).collect::<Vec<_>>();
            let names = bound_datagram_names();
            // A name is attributed only when every bind that has recorded it in
            // this process carries the same validated address.
            if let Some(entry) = names.1.iter().rev().find(|entry| entry.name == name) {
                if names.2.iter().all(|(released, address)| {
                    *released != name || *address == entry.address
                }) {
                    return Some(entry.address.clone());
                }
            }
            drop(names);
            let path = SocketAddressV1::Unix { path: name };
            path.validate().ok().map(|()| path)
        }
        _ => None,
    }
}

/// One received datagram: the delivered prefix length, the datagram's true
/// length, and its sender.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatagramReceivedV1 {
    pub delivered: usize,
    pub datagram_length: usize,
    pub sender: Option<SocketAddressV1>,
}

/// Block for one datagram. `MSG_TRUNC` makes the kernel report the
/// datagram's true length, so truncation into `buffer` is observable; the
/// excess is discarded.
pub fn datagram_receive_from(
    socket: &DatagramSocketV1,
    buffer: &mut [u8],
) -> io::Result<DatagramReceivedV1> {
    let mut storage: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
    let mut length = size_of::<libc::sockaddr_storage>() as libc::socklen_t;
    let count = unsafe {
        libc::recvfrom(
            socket.fd.as_raw_fd(),
            buffer.as_mut_ptr().cast(),
            buffer.len(),
            libc::MSG_TRUNC,
            &mut storage as *mut _ as *mut libc::sockaddr,
            &mut length,
        )
    };
    if count < 0 {
        return Err(io::Error::last_os_error());
    }
    let datagram_length = count as usize;
    Ok(DatagramReceivedV1 {
        delivered: datagram_length.min(buffer.len()),
        datagram_length,
        sender: datagram_sender(&storage, length),
    })
}

/// The socket's local address: the OS-assigned port for an IP bind, and the
/// validated bind address for Unix.
pub fn datagram_local_address(socket: &DatagramSocketV1) -> io::Result<SocketAddressV1> {
    match &socket.address {
        SocketAddressV1::Unix { .. } => Ok(socket.address.clone()),
        SocketAddressV1::Inet4 { .. } | SocketAddressV1::Inet6 { .. } => local_address(&socket.fd),
    }
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

    struct DatagramDir(std::path::PathBuf);

    impl DatagramDir {
        fn new(tag: &str) -> Self {
            let dir = std::env::temp_dir().join(format!(
                "ken-px11-dgram-{tag}-{}-{:?}",
                std::process::id(),
                std::thread::current().id()
            ));
            std::fs::create_dir(&dir).unwrap();
            Self(std::fs::canonicalize(dir).unwrap())
        }

        fn root(&self) -> crate::FsHandle {
            crate::FsHandle::Posix(
                crate::open_root(&crate::RootPath::new(&self.0).unwrap()).unwrap(),
            )
        }

        fn unix(&self, leaf: &str) -> SocketAddressV1 {
            SocketAddressV1::Unix {
                path: self.0.join(leaf).into_os_string().into_encoded_bytes(),
            }
        }
    }

    impl Drop for DatagramDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn assert_datagram_cloexec(socket: &DatagramSocketV1) {
        let flags = unsafe { libc::fcntl(socket.as_raw_fd_for_test(), libc::F_GETFD) };
        assert!(flags >= 0 && flags & libc::FD_CLOEXEC != 0);
        let mut kind: libc::c_int = 0;
        let mut length = size_of::<libc::c_int>() as libc::socklen_t;
        assert_eq!(
            unsafe {
                libc::getsockopt(
                    socket.as_raw_fd_for_test(),
                    libc::SOL_SOCKET,
                    libc::SO_TYPE,
                    (&mut kind as *mut libc::c_int).cast(),
                    &mut length,
                )
            },
            0
        );
        assert_eq!(kind, libc::SOCK_DGRAM, "datagram sockets are SOCK_DGRAM");
    }

    /// Promise class: durable invariant. MEASURED: for IPv4, IPv6 and Unix,
    /// two bound sockets exchange one datagram whose bytes, true length and
    /// sender address equal the payload and the sender's own local address,
    /// exactly; a Unix sender is reported as its validated bind address, not
    /// the `/proc/<pid>/fd` spelling the kernel recorded. CLAIMED: the
    /// receiver learns the exact sender identity for each family. THE GAP:
    /// one loopback host; scoped IPv6 and abstract senders are pinned by the
    /// `datagram_sender` rows below.
    #[test]
    fn three_datagram_families_report_exact_bytes_and_sender() {
        let dir = DatagramDir::new("families");
        let root = dir.root();
        for (receiver, sender) in [
            (
                SocketAddressV1::Inet4 { octets: [127, 0, 0, 1], port: 0 },
                SocketAddressV1::Inet4 { octets: [127, 0, 0, 1], port: 0 },
            ),
            (
                SocketAddressV1::Inet6 { octets: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], port: 0 },
                SocketAddressV1::Inet6 { octets: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], port: 0 },
            ),
            (dir.unix("receiver.sock"), dir.unix("sender.sock")),
        ] {
            let receiver = datagram_bind(&root, &receiver).unwrap();
            let sender = datagram_bind(&root, &sender).unwrap();
            assert_datagram_cloexec(&receiver);
            assert_datagram_cloexec(&sender);
            let receiver_address = datagram_local_address(&receiver).unwrap();
            let sender_address = datagram_local_address(&sender).unwrap();
            if !matches!(receiver_address, SocketAddressV1::Unix { .. }) {
                assert!(matches!(
                    (&receiver_address, &sender_address),
                    (SocketAddressV1::Inet4 { port: 1.., .. }, SocketAddressV1::Inet4 { port: 1.., .. })
                        | (SocketAddressV1::Inet6 { port: 1.., .. }, SocketAddressV1::Inet6 { port: 1.., .. })
                ));
            }
            datagram_send_to(&sender, &root, &receiver_address, b"PX11ok!").unwrap();
            let mut buffer = [0u8; 16];
            let received = datagram_receive_from(&receiver, &mut buffer).unwrap();
            assert_eq!(
                received,
                DatagramReceivedV1 {
                    delivered: 7,
                    datagram_length: 7,
                    sender: Some(sender_address.clone()),
                },
                "{receiver_address:?}"
            );
            assert_eq!(&buffer[..7], b"PX11ok!");
        }
    }

    /// Promise class: durable invariant. MEASURED: a seven-byte datagram
    /// received into six bytes delivers the six-byte prefix and reports the
    /// true length seven; the excess is discarded, so the next receive sees
    /// the next datagram whole. CLAIMED: truncation is observable through
    /// MSG_TRUNC and never leaks into the following datagram. THE GAP: one
    /// family (IPv4); Unix datagram MSG_TRUNC is exercised by the CLI row.
    #[test]
    fn datagram_receive_reports_true_length_and_discards_excess() {
        let root = crate::FsHandle::Virtual(0);
        let loopback = SocketAddressV1::Inet4 { octets: [127, 0, 0, 1], port: 0 };
        let receiver = datagram_bind(&root, &loopback).unwrap();
        let sender = datagram_bind(&root, &loopback).unwrap();
        let target = datagram_local_address(&receiver).unwrap();
        datagram_send_to(&sender, &root, &target, b"PX11ok!").unwrap();
        datagram_send_to(&sender, &root, &target, b"xy").unwrap();
        let mut short = [0u8; 6];
        let first = datagram_receive_from(&receiver, &mut short).unwrap();
        assert_eq!((first.delivered, first.datagram_length), (6, 7));
        assert_eq!(&short, b"PX11ok");
        let mut whole = [0u8; 6];
        let second = datagram_receive_from(&receiver, &mut whole).unwrap();
        assert_eq!((second.delivered, second.datagram_length), (2, 2));
        assert_eq!(&whole[..2], b"xy");
    }

    /// Promise class: durable invariant. MEASURED: a live Unix datagram
    /// socket's kernel-recorded name is its `/proc/<pid>/fd/<parent>/<leaf>`
    /// bind spelling, the registry maps exactly that name to the validated
    /// address, and release removes the entry while the pathname stays.
    /// CLAIMED: the sender table lives from bind to release and never
    /// unlinks. THE GAP: the documented unlink-and-rebind residual is not
    /// exercised.
    #[test]
    fn unix_datagram_name_registry_lives_from_bind_to_release() {
        let dir = DatagramDir::new("registry");
        let root = dir.root();
        let address = dir.unix("named.sock");
        let socket = datagram_bind(&root, &address).unwrap();
        let mut storage: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
        let mut length = size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        assert_eq!(
            unsafe {
                libc::getsockname(
                    socket.as_raw_fd_for_test(),
                    &mut storage as *mut _ as *mut libc::sockaddr,
                    &mut length,
                )
            },
            0
        );
        let recorded = SocketAddressV1::from_sockaddr(&storage, length).unwrap();
        let SocketAddressV1::Unix { path: recorded } = recorded else { unreachable!() };
        let pid = std::fs::read_link("/proc/self").unwrap();
        let prefix = format!("/proc/{}/fd/", pid.to_str().unwrap()).into_bytes();
        assert!(recorded.starts_with(&prefix), "{:?}", String::from_utf8_lossy(&recorded));
        assert!(recorded.ends_with(b"/named.sock"));
        assert_eq!(datagram_sender(&storage, length), Some(address.clone()));
        assert!(dir.0.join("named.sock").exists());
        drop(socket);
        assert_eq!(
            datagram_sender(&storage, length),
            Some(SocketAddressV1::Unix { path: recorded }),
            "a released name is no longer a live entry, so it is reported as spelled"
        );
        assert!(dir.0.join("named.sock").exists(), "release does not unlink");
    }

    /// The kernel-recorded name of a bound Unix datagram socket, read by
    /// `getsockname` independently of the sender table.
    fn recorded_unix_name(socket: &DatagramSocketV1) -> Vec<u8> {
        let mut storage: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
        let mut length = size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        assert_eq!(
            unsafe {
                libc::getsockname(
                    socket.as_raw_fd_for_test(),
                    &mut storage as *mut _ as *mut libc::sockaddr,
                    &mut length,
                )
            },
            0
        );
        let SocketAddressV1::Unix { path } = SocketAddressV1::from_sockaddr(&storage, length)
            .unwrap()
        else {
            unreachable!()
        };
        path
    }

    /// Promise class: durable invariant. MEASURED: A at `a<i>/stale-s.sock`
    /// sends seven bytes to B and is released; C is then bound at
    /// `c<i>/stale-s.sock`, retried over fresh directories until
    /// `getsockname` shows C recorded exactly A's released name (the
    /// parent descriptor number was reused). B then receives A's queued
    /// datagram: its sender is the raw `/proc` spelling, not C's validated
    /// address; replying to it is `CapabilityDenied`, and nothing reaches C.
    /// C's own datagram is also reported by its raw spelling, the cost the
    /// spec states. CLAIMED: a released bind's name is never attributed to a
    /// later socket of a different validated address, so a reply cannot
    /// reach a socket that did not send the datagram. THE GAP: the collision
    /// is constructed in one process by descriptor reuse; a foreign
    /// process's bind through this process's `/proc` spelling is the spec's
    /// residual and is not exercised.
    #[test]
    fn released_unix_datagram_name_is_not_attributed_to_a_later_bind() {
        let dir = DatagramDir::new("stale");
        let root = dir.root();
        let b_address = dir.unix("stale-b.sock");
        let b = datagram_bind(&root, &b_address).unwrap();
        let mut buffer = [0u8; 16];
        let mut collided = None;
        for attempt in 0..64 {
            std::fs::create_dir(dir.0.join(format!("a{attempt}"))).unwrap();
            std::fs::create_dir(dir.0.join(format!("c{attempt}"))).unwrap();
            let a = datagram_bind(&root, &dir.unix(&format!("a{attempt}/stale-s.sock"))).unwrap();
            let a_name = recorded_unix_name(&a);
            datagram_send_to(&a, &root, &b_address, b"PX11ok!").unwrap();
            drop(a);
            let c_address = dir.unix(&format!("c{attempt}/stale-s.sock"));
            let c = datagram_bind(&root, &c_address).unwrap();
            if recorded_unix_name(&c) == a_name {
                collided = Some((a_name, c, c_address));
                break;
            }
            // Another thread took the descriptor; drain A's datagram.
            let drained = datagram_receive_from(&b, &mut buffer).unwrap();
            assert_eq!(drained.sender, Some(SocketAddressV1::Unix { path: a_name }));
        }
        let (a_name, c, c_address) = collided.expect("a later bind reuses the released name");
        let stale = datagram_receive_from(&b, &mut buffer).unwrap();
        assert_eq!(&buffer[..7], b"PX11ok!");
        assert_ne!(stale.sender, Some(c_address.clone()), "never the later socket");
        assert_eq!(stale.sender, Some(SocketAddressV1::Unix { path: a_name.clone() }));
        assert_eq!(
            datagram_send_to(&b, &root, stale.sender.as_ref().unwrap(), b"reply!!").unwrap_err(),
            IoErrorIdentityV1::CapabilityDenied
        );
        unsafe { libc::fcntl(c.as_raw_fd_for_test(), libc::F_SETFL, libc::O_NONBLOCK) };
        assert!(datagram_receive_from(&c, &mut buffer).is_err(), "nothing reached C");
        datagram_send_to(&c, &root, &b_address, b"live!!!").unwrap();
        let live = datagram_receive_from(&b, &mut buffer).unwrap();
        assert_eq!(live.sender, Some(SocketAddressV1::Unix { path: a_name }), "the stated cost");
    }

    /// Promise class: durable invariant. MEASURED: the different-leaf
    /// control (A at `a/fresh-s.sock` released, C at `c/fresh-t.sock`, so
    /// the recorded names differ) reports A's queued datagram by its raw
    /// `/proc` spelling and refuses the reply with `CapabilityDenied`; the
    /// live C, whose name no released bind recorded, is reported as its
    /// validated address; and a socket unlinked and rebound by this process
    /// at the same validated path keeps that attribution. CLAIMED: the
    /// released-name rule removes attribution only where a released bind of
    /// a different validated address shares the name, never from a live
    /// sender in the ordinary case. THE GAP: in-process host boundary; the
    /// interpreter rows are the CLI reproducer, control and Unix exchange.
    #[test]
    fn unix_datagram_live_sender_keeps_attribution_beside_a_released_name() {
        let dir = DatagramDir::new("fresh");
        std::fs::create_dir(dir.0.join("a")).unwrap();
        std::fs::create_dir(dir.0.join("c")).unwrap();
        let root = dir.root();
        let b_address = dir.unix("fresh-b.sock");
        let b = datagram_bind(&root, &b_address).unwrap();
        let mut buffer = [0u8; 16];
        let a = datagram_bind(&root, &dir.unix("a/fresh-s.sock")).unwrap();
        let a_name = recorded_unix_name(&a);
        datagram_send_to(&a, &root, &b_address, b"PX11ok!").unwrap();
        drop(a);
        let c_address = dir.unix("c/fresh-t.sock");
        let c = datagram_bind(&root, &c_address).unwrap();
        assert_ne!(recorded_unix_name(&c), a_name, "the leaves differ");
        let stale = datagram_receive_from(&b, &mut buffer).unwrap();
        assert_eq!(stale.sender, Some(SocketAddressV1::Unix { path: a_name }));
        assert_eq!(
            datagram_send_to(&b, &root, stale.sender.as_ref().unwrap(), b"reply!!").unwrap_err(),
            IoErrorIdentityV1::CapabilityDenied
        );
        datagram_send_to(&c, &root, &b_address, b"live!!!").unwrap();
        let live = datagram_receive_from(&b, &mut buffer).unwrap();
        assert_eq!(live.sender, Some(c_address.clone()), "a live non-colliding sender");
        datagram_send_to(&b, &root, &c_address, b"reply!!").unwrap();
        assert_eq!(datagram_receive_from(&c, &mut buffer).unwrap().delivered, 7);
        let d_address = dir.unix("fresh-d.sock");
        drop(datagram_bind(&root, &d_address).unwrap());
        std::fs::remove_file(dir.0.join("fresh-d.sock")).unwrap();
        let d = datagram_bind(&root, &d_address).unwrap();
        datagram_send_to(&d, &root, &b_address, b"rebound").unwrap();
        let rebound = datagram_receive_from(&b, &mut buffer).unwrap();
        assert_eq!(rebound.sender, Some(d_address), "same-address rebind keeps attribution");
    }

    /// Promise class: durable invariant. MEASURED: the sender decoder maps an
    /// unnamed Unix sender, an abstract name, a scoped IPv6 sender and an
    /// unknown family to None, keeps an IPv6 sender with flowinfo set but
    /// drops the flowinfo, and reports an unregistered valid absolute name
    /// as that path. CLAIMED: only identities `SocketAddress` can carry are
    /// reported. THE GAP: synthetic sockaddrs; the kernel's own encodings
    /// for these senders are not produced here.
    #[test]
    fn datagram_sender_reports_only_representable_identities() {
        let base = offset_of!(libc::sockaddr_un, sun_path);
        let unix = |name: &[u8], length: usize| {
            let mut storage: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
            let address = unsafe { &mut *(std::ptr::from_mut(&mut storage) as *mut libc::sockaddr_un) };
            address.sun_family = libc::AF_UNIX as libc::sa_family_t;
            for (slot, byte) in address.sun_path.iter_mut().zip(name) {
                *slot = *byte as libc::c_char;
            }
            (storage, length as libc::socklen_t)
        };
        let (storage, length) = unix(b"", base);
        assert_eq!(datagram_sender(&storage, length), None, "unnamed");
        let (storage, length) = unix(b"\0abstract", base + 9);
        assert_eq!(datagram_sender(&storage, length), None, "abstract");
        let (storage, length) = unix(b"relative", base + 9);
        assert_eq!(datagram_sender(&storage, length), None, "not absolute");
        let (storage, length) = unix(b"/elsewhere/peer.sock\0", base + 21);
        assert_eq!(
            datagram_sender(&storage, length),
            Some(SocketAddressV1::Unix { path: b"/elsewhere/peer.sock".to_vec() })
        );
        let inet6 = |scope: u32| {
            let mut storage: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
            let address = unsafe { &mut *(std::ptr::from_mut(&mut storage) as *mut libc::sockaddr_in6) };
            address.sin6_family = libc::AF_INET6 as libc::sa_family_t;
            address.sin6_port = 4242u16.to_be();
            address.sin6_flowinfo = 0x0102_0304;
            address.sin6_scope_id = scope;
            address.sin6_addr.s6_addr = [0xfe, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
            (storage, size_of::<libc::sockaddr_in6>() as libc::socklen_t)
        };
        let (storage, length) = inet6(0);
        assert_eq!(
            datagram_sender(&storage, length),
            Some(SocketAddressV1::Inet6 {
                octets: [0xfe, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                port: 4242,
            })
        );
        let (storage, length) = inet6(3);
        assert_eq!(datagram_sender(&storage, length), None, "scoped IPv6");
        let mut storage: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
        storage.ss_family = libc::AF_PACKET as libc::sa_family_t;
        assert_eq!(datagram_sender(&storage, 16), None, "unknown family");
    }

    /// Promise class: durable invariant. MEASURED: a datagram Unix bind
    /// outside the root, through `..`, at a leaf symlink, on a virtual root,
    /// or with an 80-byte leaf refuses with the stream listener's identities
    /// and creates no inode; a 79-byte leaf binds. CLAIMED: the datagram bind
    /// reuses the stream FS-root rule with its own 79-byte leaf cap. THE GAP:
    /// a host-unit boundary; the CLI row exercises checked Ken.
    #[test]
    fn unix_datagram_bind_obeys_fs_root_and_leaf_cap() {
        let dir = DatagramDir::new("root");
        std::fs::create_dir(dir.0.join("jail")).unwrap();
        std::fs::create_dir(dir.0.join("outside")).unwrap();
        std::os::unix::fs::symlink("../outside/linked.sock", dir.0.join("jail/alias.sock")).unwrap();
        let jail = crate::FsHandle::Posix(
            crate::open_root(&crate::RootPath::new(dir.0.join("jail")).unwrap()).unwrap(),
        );
        for (address, expected, inode) in [
            (dir.unix("outside/made.sock"), IoErrorIdentityV1::CapabilityDenied, dir.0.join("outside/made.sock")),
            (dir.unix("jail/../outside/dot.sock"), IoErrorIdentityV1::CapabilityDenied, dir.0.join("outside/dot.sock")),
            (dir.unix("jail/alias.sock"), IoErrorIdentityV1::CapabilityDenied, dir.0.join("outside/linked.sock")),
            (dir.unix("jail/missing/made.sock"), IoErrorIdentityV1::NotFound, dir.0.join("jail/missing")),
        ] {
            assert_eq!(datagram_bind(&jail, &address).unwrap_err(), expected, "{address:?}");
            assert!(std::fs::symlink_metadata(&inode).is_err(), "no inode at {inode:?}");
        }
        assert_eq!(
            datagram_bind(&crate::FsHandle::Virtual(0), &dir.unix("jail/virtual.sock")).unwrap_err(),
            IoErrorIdentityV1::CapabilityDenied
        );
        // Short roots so that the leaf, not sun_path, is the binding limit.
        let tmp = std::fs::canonicalize("/tmp").unwrap();
        let tmp_root =
            crate::FsHandle::Posix(crate::open_root(&crate::RootPath::new(&tmp).unwrap()).unwrap());
        let leaf = |len: usize| {
            let prefix = format!("kd{}-{:?}-", std::process::id(), std::thread::current().id())
                .replace(['(', ')'], "");
            format!("{prefix}{}", "y".repeat(len - prefix.len()))
        };
        let at = |leaf: &str| SocketAddressV1::Unix {
            path: tmp.join(leaf).into_os_string().into_encoded_bytes(),
        };
        let long = at(&leaf(80));
        long.validate().expect("the full path fits sun_path");
        assert_eq!(datagram_bind(&tmp_root, &long).unwrap_err(), IoErrorIdentityV1::InvalidInput);
        assert!(std::fs::symlink_metadata(tmp.join(leaf(80))).is_err());
        let fits = leaf(79);
        let bound = datagram_bind(&tmp_root, &at(&fits)).unwrap();
        assert!(tmp.join(&fits).exists());
        drop(bound);
        std::fs::remove_file(tmp.join(&fits)).unwrap();
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
