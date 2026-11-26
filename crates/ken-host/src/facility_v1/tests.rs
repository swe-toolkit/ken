use super::*;
use crate::{open_at, open_resource_at_v1, open_root, OpenRequest, PathComponent, RootPath};
use HostFacilityV1::*;

fn fixture(tag: &str) -> (std::path::PathBuf, crate::RootedHandle) {
    let directory = std::env::temp_dir().join(format!(
        "ken-host-abim2-{tag}-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir(&directory).unwrap();
    let root = open_root(&RootPath::new(&directory).unwrap()).unwrap();
    (directory, root)
}

#[test]
fn every_facility_is_present_on_this_host() {
    for facility in [Pidfd, Statx, ProcSelfFd, StreamSocketInet4, StreamSocketInet6, StreamSocketUnix, Accept4] {
        require(facility).unwrap_or_else(|error| panic!("{facility:?}: {error}"));
    }
}

#[test]
fn statx_absent_refuses_handle_metadata_and_present_succeeds() {
    let (directory, root) = fixture("statx");
    let leaf = PathComponent::new(b"f").unwrap();
    drop(open_at(&root, &leaf, OpenRequest::CreateNew).unwrap());
    let resource = open_resource_at_v1(&root, &leaf, OpenRequest::Read).unwrap();
    let absent = with_facility_fault(Statx, libc::EPERM, || crate::resource_metadata_v1(&resource));
    let absent = absent.unwrap_err().into_io_error();
    assert_eq!(crate::io_error_identity_v1(&absent), crate::IoErrorIdentityV1::Unsupported);
    assert!(crate::resource_metadata_v1(&resource).is_ok());
    std::fs::remove_dir_all(directory).unwrap();
}

#[test]
fn proc_absent_refuses_tree_removal_and_leaves_the_tree() {
    let (directory, root) = fixture("proc");
    std::fs::create_dir_all(directory.join("t/u")).unwrap();
    let leaf = PathComponent::new(b"t").unwrap();
    let absent = with_facility_fault(ProcSelfFd, libc::ENOENT, || crate::remove_directory_tree(&root, &leaf));
    let absent = absent.unwrap_err().into_io_error();
    assert_eq!(crate::io_error_identity_v1(&absent), crate::IoErrorIdentityV1::Unsupported);
    assert!(directory.join("t/u").is_dir());
    crate::remove_directory_tree(&root, &leaf).unwrap();
    assert!(!directory.join("t").exists());
    std::fs::remove_dir_all(directory).unwrap();
}

#[test]
fn inet6_absent_refuses_only_inet6() {
    let root = crate::FsHandle::Virtual(0);
    let mut loopback = [0u8; 16];
    loopback[15] = 1;
    let v6 = crate::SocketAddressV1::Inet6 { octets: loopback, port: 0 };
    let v4 = crate::SocketAddressV1::Inet4 { octets: [127, 0, 0, 1], port: 0 };
    let (six, four) = with_facility_fault(StreamSocketInet6, libc::EAFNOSUPPORT, || {
        (crate::net_v1::listen(&root, &v6), crate::net_v1::listen(&root, &v4))
    });
    assert_eq!(six.unwrap_err(), crate::IoErrorIdentityV1::Unsupported);
    assert!(four.is_ok());
    assert!(crate::net_v1::listen(&root, &v6).is_ok());
}

#[test]
fn accept4_absent_refuses_accept() {
    let root = crate::FsHandle::Virtual(0);
    let listener = crate::net_v1::listen(&root, &crate::SocketAddressV1::Inet4 { octets: [127, 0, 0, 1], port: 0 }).unwrap();
    let error = with_facility_fault(Accept4, libc::EPERM, || crate::net_v1::accept(&listener)).unwrap_err();
    assert_eq!(crate::io_error_identity_v1(&error), crate::IoErrorIdentityV1::Unsupported);
}

#[test]
fn an_undecided_probe_is_not_cached() {
    let error = with_facility_fault(Statx, libc::EMFILE, || require(Statx)).unwrap_err();
    assert_eq!(error.raw_os_error(), Some(libc::EMFILE));
    require(Statx).unwrap();
}

#[test]
fn a_decisive_probe_runs_once_per_process() {
    require(StreamSocketUnix).unwrap();
    require(StreamSocketUnix).unwrap();
    assert_eq!(probes_run_for_test(StreamSocketUnix), 1);
}
