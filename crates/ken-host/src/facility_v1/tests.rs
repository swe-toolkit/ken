use super::*;
use crate::net_v1::{accept, connect, listen, local_address};
use crate::{
    FsHandle, IoErrorIdentityV1, OpenRequest, PathComponent, RootPath, RootedHandle,
    SocketAddressV1, open_at, open_resource_at_v1, open_root,
};
use HostFacilityV1::*;
use std::path::{Path, PathBuf};

struct Fixture(PathBuf);

impl Fixture {
    fn new(tag: &str) -> (Self, RootedHandle) {
        let directory = std::env::temp_dir().join(format!(
            "ken-abim2-{tag}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir(&directory).unwrap();
        let this = Self(std::fs::canonicalize(directory).unwrap());
        let root = open_root(&RootPath::new(&this.0).unwrap()).unwrap();
        (this, root)
    }

    fn path(&self, leaf: &str) -> PathBuf {
        self.0.join(leaf)
    }

    fn unix(&self, leaf: &str) -> SocketAddressV1 {
        SocketAddressV1::Unix {
            path: self.path(leaf).into_os_string().into_encoded_bytes(),
        }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn inet4() -> SocketAddressV1 {
    SocketAddressV1::Inet4 {
        octets: [127, 0, 0, 1],
        port: 0,
    }
}

fn inet6() -> SocketAddressV1 {
    let mut loopback = [0; 16];
    loopback[15] = 1;
    SocketAddressV1::Inet6 {
        octets: loopback,
        port: 0,
    }
}

fn assert_socket_file(path: &Path) {
    use std::os::unix::fs::FileTypeExt;
    assert!(
        std::fs::symlink_metadata(path)
            .unwrap()
            .file_type()
            .is_socket()
    );
}

fn assert_no_path(path: &Path) {
    assert_eq!(
        std::fs::symlink_metadata(path).unwrap_err().kind(),
        io::ErrorKind::NotFound,
        "unexpected path: {path:?}"
    );
}

// A bounded accept pin: even a missing or already-consumed connection
// cannot hang the test when the facility guard is removed.
fn nonblocking(listener: &OwnedFd) {
    use rustix::fs::{OFlags, fcntl_getfl, fcntl_setfl};
    fcntl_setfl(listener, fcntl_getfl(listener).unwrap() | OFlags::NONBLOCK).unwrap();
}

/// Promise class: durable invariant. MEASURED: EPERM during the probe
/// yields Unsupported through handle metadata; the same file works without
/// the fault. CLAIMED: the statx seat refuses before querying metadata.
/// THE GAP: The unknown-type arm of FsReadDirectoryBatch reaches statx only
/// through statx_checked, the seat the metadata rows drive; the test
/// filesystems report d_type, so no row drives that arm.
#[test]
fn statx_absent_refuses_handle_metadata_and_present_succeeds() {
    let (_fixture, root) = Fixture::new("statx");
    let leaf = PathComponent::new(b"f").unwrap();
    drop(open_at(&root, &leaf, OpenRequest::CreateNew).unwrap());
    let resource = open_resource_at_v1(&root, &leaf, OpenRequest::Read).unwrap();
    let absent = with_facility_fault(Statx, libc::EPERM, || {
        crate::resource_metadata_v1(&resource)
    });
    assert_eq!(
        absent.map_err(|error| crate::io_error_identity_v1(&error.into_io_error())),
        Err(IoErrorIdentityV1::Unsupported)
    );
    assert!(crate::resource_metadata_v1(&resource).is_ok());
}

/// Promise class: durable invariant. MEASURED: recursive removal leaves
/// the nested directory intact on ENOENT at the probe and erases it without
/// the seam. CLAIMED: absent /proc is Unsupported before removal.
/// THE GAP: one rooted tree, not kernel or namespace coverage.
#[test]
fn proc_absent_refuses_tree_removal_and_leaves_the_tree() {
    let (fixture, root) = Fixture::new("proc-tree");
    std::fs::create_dir_all(fixture.path("t/u")).unwrap();
    let leaf = PathComponent::new(b"t").unwrap();
    let absent = with_facility_fault(ProcSelfFd, libc::ENOENT, || {
        crate::remove_directory_tree(&root, &leaf)
    });
    assert_eq!(
        absent.map_err(|error| crate::io_error_identity_v1(&error.into_io_error())),
        Err(IoErrorIdentityV1::Unsupported)
    );
    assert!(fixture.path("t/u").is_dir());
    crate::remove_directory_tree(&root, &leaf).unwrap();
    assert_no_path(&fixture.path("t"));
}

/// Promise class: durable invariant. MEASURED: Unix listen refuses with
/// Unsupported (not CapabilityDenied), leaves no socket, and creates it
/// without the seam. CLAIMED: /proc admission runs before Unix binding.
/// THE GAP: this drives rooted Unix listen, not other path errors.
#[test]
fn proc_absent_refuses_unix_listen_without_binding() {
    let (fixture, root) = Fixture::new("proc-listen");
    let root = FsHandle::Posix(root);
    let address = fixture.unix("socket");
    let result = with_facility_fault(ProcSelfFd, libc::ENOENT, || listen(&root, &address));
    assert_eq!(result.unwrap_err(), IoErrorIdentityV1::Unsupported);
    assert_no_path(&fixture.path("socket"));
    let _listener = listen(&root, &address).unwrap();
    assert_socket_file(&fixture.path("socket"));
}

/// Promise class: durable invariant. MEASURED: Unix connect to a live
/// listener refuses with Unsupported and leaves its queue empty; the same
/// connect succeeds without the seam. CLAIMED: /proc admission runs before
/// connecting. THE GAP: one local live Unix listener and an injected errno.
#[test]
fn proc_absent_refuses_unix_connect_without_a_pending_client() {
    let (fixture, root) = Fixture::new("proc-connect");
    let root = FsHandle::Posix(root);
    let address = fixture.unix("socket");
    let listener = listen(&root, &address).unwrap();
    nonblocking(&listener);
    let result = with_facility_fault(ProcSelfFd, libc::ENOENT, || connect(&root, &address));
    assert_eq!(result.unwrap_err(), IoErrorIdentityV1::Unsupported);
    assert_eq!(
        accept(&listener).unwrap_err().kind(),
        io::ErrorKind::WouldBlock
    );
    let _client = connect(&root, &address).unwrap();
    accept(&listener).unwrap();
}

fn assert_socket_family_isolation(faulted: HostFacilityV1) {
    let (fixture, root) = Fixture::new("family");
    let root = FsHandle::Posix(root);
    let addresses = [
        (StreamSocketInet4, inet4()),
        (StreamSocketInet6, inet6()),
        (StreamSocketUnix, fixture.unix("socket")),
    ];
    with_facility_fault(faulted, libc::EAFNOSUPPORT, || {
        for (facility, address) in &addresses {
            let result = listen(&root, address);
            if *facility == faulted {
                assert_eq!(
                    result.unwrap_err(),
                    IoErrorIdentityV1::Unsupported,
                    "{facility:?}"
                );
                if *facility == StreamSocketUnix {
                    assert_no_path(&fixture.path("socket"));
                }
            } else {
                assert!(
                    result.is_ok(),
                    "{facility:?} under {faulted:?} fault: {result:?}"
                );
            }
        }
    });
    let address = &addresses.iter().find(|(f, _)| *f == faulted).unwrap().1;
    let _listener = listen(&root, address).expect("same listen without the probe fault");
}

// Promise class: durable invariant. MEASURED: each family refuses
// EAFNOSUPPORT as Unsupported while its siblings still listen, then itself
// listens without the seam. CLAIMED: absence is keyed by family.
// THE GAP: loopback/Unix listen only, not every endpoint or socket policy.
#[test]
fn inet4_absent_refuses_only_inet4() {
    assert_socket_family_isolation(StreamSocketInet4);
}

#[test]
fn inet6_absent_refuses_only_inet6() {
    assert_socket_family_isolation(StreamSocketInet6);
}

#[test]
fn unix_stream_absent_refuses_only_unix() {
    assert_socket_family_isolation(StreamSocketUnix);
}

/// Promise class: durable invariant. MEASURED: an already-connected client
/// remains queued across Unsupported; the same listener accepts it without
/// the seam. CLAIMED: refusal does not accept. THE GAP: one queued IPv4
/// client, rather than a kernel denying accept4.
#[test]
fn accept4_absent_refuses_accept_without_consuming_client() {
    let root = FsHandle::Virtual(0);
    let listener = listen(&root, &inet4()).unwrap();
    nonblocking(&listener);
    let _client = connect(&root, &local_address(&listener).unwrap()).unwrap();
    let result = with_facility_fault(Accept4, libc::EPERM, || accept(&listener).map(drop));
    assert_eq!(
        result.as_ref().err().map(crate::io_error_identity_v1),
        Some(IoErrorIdentityV1::Unsupported),
        "absent accept of a queued client: {result:?}"
    );
    accept(&listener).expect("absent accept must leave the client queued");
}

/// Promise class: durable invariant. MEASURED: EMFILE stays raw, and the
/// next real require succeeds. CLAIMED: EMFILE does not become absence.
/// THE GAP: the fault seam shares classification but bypasses the process
/// cache; the real cache inserts only after a successful classification.
#[test]
fn an_undecided_probe_is_not_cached() {
    let error = with_facility_fault(Statx, libc::EMFILE, || require(Statx)).unwrap_err();
    assert_eq!(
        error.raw_os_error(),
        Some(libc::EMFILE),
        "undecided error: {error:?}"
    );
    require(Statx).unwrap();
}

/// Promise class: durable invariant. MEASURED: two real requires see one
/// process-wide StreamSocketUnix probe. CLAIMED: a decisive presence is
/// reused. THE GAP: this counts the present case; absence uses the same
/// cache insertion path, but injected faults deliberately bypass it.
#[test]
fn a_decisive_probe_runs_once_per_process() {
    require(StreamSocketUnix).unwrap();
    require(StreamSocketUnix).unwrap();
    assert_eq!(probes_run_for_test(StreamSocketUnix), 1);
}
