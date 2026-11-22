//! Spec 38 §1.13, PX11 stream sockets. Each case reaches the checked Ken
//! surface, interpreter, host dispatcher and Linux socket leaf.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const PROFILE: &str = r#"{"runtime":{"invocation_epochs":18446744073709551615},"call_events":{"event_generations":18446744073709551615,"live_pending_slots":64},"invocation":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64},"persistent":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64}}"#;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn run(name: &str, cwd: &Path, arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ken"))
        .current_dir(cwd)
        .arg("run")
        .arg(fixture(name))
        .arg("--")
        .args(arguments)
        .output()
        .expect("run checked Ken source")
}

fn assert_success(name: &str, output: Output) {
    assert_eq!(
        output.status.code(),
        Some(0),
        "{name}: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Promise class: durable invariant (the seven Net ops stay interpreter-live).
/// MEASURED: real Ken listener, ephemeral-port readback, connect/accept,
/// full-byte sends and reads in both directions, shutdown then EOF across
/// IPv4, IPv6 and Unix. CLAIMED: the three address families' stream contracts
/// are executable. THE GAP: the Linux host is required to support all three
/// address families; this case does not establish native support (AC-4 refuses).
#[test]
fn px11_all_three_families_exchange_exact_bytes_and_observe_eof() {
    for family in ["4", "6", "unix"] {
        let dir = tempfile::Builder::new()
            .prefix("ken-px11-")
            .tempdir_in("/tmp")
            .expect("isolated socket fixture directory");
        std::fs::write(dir.path().join("px11-payload.bin"), b"PX11ok!")
            .expect("known-answer payload");
        let unix_path = std::fs::canonicalize(dir.path())
            .expect("canonical default FS root")
            .join("listener.sock");
        let unix = unix_path.to_str().expect("UTF-8 temporary path");
        assert_success(
            family,
            run("px11_stream_exchange.ken", dir.path(), &[family, unix]),
        );
        if family == "unix" {
            assert!(
                unix_path.exists(),
                "socket pathname survives bracket release"
            );
        } else {
            assert!(
                !unix_path.exists(),
                "IP traffic does not create a Unix socket"
            );
        }
    }
}

/// Promise class: durable invariant. MEASURED: a valid absolute Unix address
/// without a listener returns the canonical NotFound cause through the
/// SocketOp ConnectSocket and SocketResource Some envelope. CLAIMED: the
/// address-aware System.Error conversion preserves typed failure identity.
/// THE GAP: the checked theorem in the fixture pins the address itself;
/// the runtime match observes the envelope's constructor and cause.
#[test]
fn px11_missing_unix_listener_has_typed_socket_error() {
    let dir = tempfile::Builder::new()
        .prefix("ken-px11-")
        .tempdir_in("/tmp")
        .unwrap();
    let unix_path = std::fs::canonicalize(dir.path())
        .expect("canonical default FS root")
        .join("missing.sock");
    assert!(!unix_path.exists(), "negative case requires no listener");
    let path = unix_path.to_str().unwrap();
    assert_success(
        "missing listener",
        run("px11_missing_listener.ken", dir.path(), &[path]),
    );
}

/// Promise class: durable invariant. MEASURED: a Unix address under only a
/// Loopback Net grant produces CapabilityDenied in the typed socket envelope.
/// CLAIMED: a grant for a different address class cannot authorize connect.
/// THE GAP: the host-level backend-call counter independently checks that
/// denial precedes the socket syscall; an eventual NotFound must not suffice.
#[test]
fn px11_wrong_grant_refuses_unix_connect() {
    let dir = tempfile::Builder::new()
        .prefix("ken-px11-")
        .tempdir_in("/tmp")
        .unwrap();
    let path = std::fs::canonicalize(dir.path())
        .expect("canonical default FS root")
        .join("absent.sock");
    assert_success(
        "wrong grant",
        run(
            "px11_wrong_grant.ken",
            dir.path(),
            &[path.to_str().unwrap()],
        ),
    );
}

#[derive(Clone, Copy, Debug)]
enum UnixRootCase {
    ListenInside,
    ListenOutside,
    ListenLeafSymlink,
    ConnectInside,
    ConnectOutside,
    ParentEscape,
    ParentSymlink,
    LeafSymlink,
}

/// MEASURED: a checked program with FS AFull "./jail" and Net Unix uses the
/// real interpreter/host dispatch to bind or connect at a named path. Outside
/// peers are live, outside bind leaves no inode, and a listener leaf symlink
/// preserves its link and outside peer. CLAIMED: Unix acquisition cannot extend
/// the presented FS root via spelling, parent or leaf symlinks.
/// THE GAP: this samples one canonical Linux temp root; descriptor pinning and
/// the other roots/error classes have separate host-level tests and review.
fn assert_unix_root_case(case: UnixRootCase) {
    use std::os::unix::fs::{symlink, FileTypeExt, MetadataExt};
    use std::os::unix::net::{UnixListener, UnixStream};

    let dir = tempfile::Builder::new()
        .prefix("ken-net-scope-")
        .tempdir_in("/tmp")
        .expect("isolated rooted Unix socket directory");
    let root = std::fs::canonicalize(dir.path()).expect("canonical test cwd");
    let jail = root.join("jail");
    let outside = root.join("outside");
    std::fs::create_dir(&jail).unwrap();
    std::fs::create_dir(&outside).unwrap();
    let _inside_peer = UnixListener::bind(jail.join("svc.sock")).expect("live inside peer");
    let outside_peer_path = outside.join("svc.sock");
    let _outside_peer = UnixListener::bind(&outside_peer_path).expect("live outside peer");
    symlink("../outside", jail.join("link")).unwrap();
    let alias_path = jail.join("alias.sock");
    symlink("../outside/svc.sock", &alias_path).unwrap();
    let leaf_state = if matches!(case, UnixRootCase::ListenLeafSymlink) {
        let link = std::fs::symlink_metadata(&alias_path).unwrap();
        assert!(
            link.file_type().is_symlink(),
            "listener leaf must be a symlink"
        );
        let peer = std::fs::metadata(&outside_peer_path).unwrap();
        assert!(
            peer.file_type().is_socket(),
            "outside peer must be a socket"
        );
        Some((
            std::fs::read_link(&alias_path).unwrap(),
            (link.dev(), link.ino()),
            (peer.dev(), peer.ino()),
        ))
    } else {
        None
    };
    let inside_made = jail.join("made.sock");
    let outside_made = outside.join("made.sock");
    assert!(!outside_made.exists());
    let (operation, address, expected) = match case {
        UnixRootCase::ListenInside => ("listen", inside_made.clone(), 0),
        UnixRootCase::ListenOutside => ("listen", outside_made.clone(), 84),
        UnixRootCase::ListenLeafSymlink => ("listen", alias_path.clone(), 84),
        UnixRootCase::ConnectInside => ("connect", jail.join("svc.sock"), 0),
        UnixRootCase::ConnectOutside => ("connect", outside.join("svc.sock"), 84),
        UnixRootCase::ParentEscape => ("connect", jail.join("../outside/svc.sock"), 84),
        UnixRootCase::ParentSymlink => ("connect", jail.join("link/svc.sock"), 84),
        UnixRootCase::LeafSymlink => ("connect", alias_path.clone(), 84),
    };
    let output = run(
        "rt_net_unix_root_scope.ken",
        &root,
        &[operation, address.to_str().expect("UTF-8 temp pathname")],
    );
    assert_eq!(
        output.status.code(),
        Some(expected),
        "{case:?}: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    assert_eq!(
        inside_made.exists(),
        matches!(case, UnixRootCase::ListenInside)
    );
    assert!(
        !outside_made.exists(),
        "outside bind must not create an inode"
    );
    if let Some((target, link_identity, peer_identity)) = leaf_state {
        let link = std::fs::symlink_metadata(&alias_path).expect("listener symlink survives");
        assert!(link.file_type().is_symlink());
        assert_eq!(std::fs::read_link(&alias_path).unwrap(), target);
        assert_eq!((link.dev(), link.ino()), link_identity);
        let peer = std::fs::metadata(&outside_peer_path).expect("outside peer survives");
        assert!(peer.file_type().is_socket());
        assert_eq!((peer.dev(), peer.ino()), peer_identity);
        let _connection = UnixStream::connect(&outside_peer_path)
            .expect("outside peer remains live after refused listen");
    }
}

/// Promise class: durable invariant; bind beneath the FS root succeeds.
#[test]
fn unix_listen_inside_declared_fs_root_creates_socket() {
    assert_unix_root_case(UnixRootCase::ListenInside);
}

/// Promise class: durable invariant; out-of-root bind is CapabilityDenied.
#[test]
fn unix_listen_outside_declared_fs_root_refuses_without_inode() {
    assert_unix_root_case(UnixRootCase::ListenOutside);
}

/// Promise class: durable invariant. MEASURED: NetListen returns the exact
/// ResourceHostIO CapabilityDenied branch; the symlink target and inode, live
/// outside peer inode and connection remain unchanged. CLAIMED: a listener
/// leaf symlink cannot escape the FS root, and refusal precedes socket().
/// THE GAP: the CLI does not count syscalls; guard ordering needs source review.
#[test]
fn unix_listen_leaf_symlink_refuses_live_outside_peer_without_path_change() {
    assert_unix_root_case(UnixRootCase::ListenLeafSymlink);
}

/// Promise class: durable invariant; connect to a live in-root peer succeeds.
#[test]
fn unix_connect_inside_declared_fs_root_reaches_live_peer() {
    assert_unix_root_case(UnixRootCase::ConnectInside);
}

/// Promise class: durable invariant; out-of-root live peer is denied by root.
#[test]
fn unix_connect_outside_declared_fs_root_refuses_live_peer() {
    assert_unix_root_case(UnixRootCase::ConnectOutside);
}

/// Promise class: durable invariant; `..` cannot escape the root.
#[test]
fn unix_connect_dotdot_refuses_live_outside_peer() {
    assert_unix_root_case(UnixRootCase::ParentEscape);
}

/// Promise class: durable invariant; parent symlink cannot escape the root.
#[test]
fn unix_connect_parent_symlink_refuses_live_outside_peer() {
    assert_unix_root_case(UnixRootCase::ParentSymlink);
}

/// Promise class: durable invariant; leaf symlink cannot escape the root.
#[test]
fn unix_connect_leaf_symlink_refuses_live_outside_peer() {
    assert_unix_root_case(UnixRootCase::LeafSymlink);
}

/// Promise class: transition sentinel, retired by the native-parity successor.
/// MEASURED: native-build of the same checked Net program returns the exact
/// pre-emission unavailable diagnostic and an empty output directory.
/// CLAIMED: no Net-declared program falls through to native object emission.
/// THE GAP: the gate applies to the declared Net plan; the test does not
/// enumerate every possible Net syntax spelling.
#[test]
fn px11_declared_net_native_build_refuses_before_emission() {
    let dir = tempfile::tempdir().unwrap();
    let output_dir = dir.path().join("native");
    std::fs::create_dir(&output_dir).unwrap();
    let profile = dir.path().join("profile.json");
    std::fs::write(&profile, PROFILE).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("native-build")
        .arg(fixture("px11_stream_exchange.ken"))
        .arg(&output_dir)
        .arg(profile)
        .output()
        .expect("attempt native build through checked source");
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("host_effect_lowering_unavailable"),
        "{stderr}"
    );
    let named: Vec<_> = ken_host::HostOpV1::ALL
        .into_iter()
        .filter(|operation| stderr.contains(&format!("{operation:?}")))
        .collect();
    assert_eq!(
        named,
        [
            ken_host::HostOpV1::NetListen,
            ken_host::HostOpV1::NetAccept,
            ken_host::HostOpV1::NetConnect,
            ken_host::HostOpV1::NetReceive,
            ken_host::HostOpV1::NetSend,
            ken_host::HostOpV1::NetShutdown,
            ken_host::HostOpV1::NetLocalAddress,
        ],
        "the refusal names exactly the seven unavailable Net operations: {stderr}"
    );
    assert!(stderr.contains("RepresentedUnavailable"), "{stderr}");
    assert_eq!(std::fs::read_dir(&output_dir).unwrap().count(), 0);
}
