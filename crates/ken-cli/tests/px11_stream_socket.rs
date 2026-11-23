//! Spec 38 §1.13, PX11 stream sockets. Each case reaches the checked Ken
//! surface, the interpreter and native lanes, and the shared Linux socket leaf.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const PROFILE: &str = r#"{"runtime":{"invocation_epochs":18446744073709551615},"call_events":{"event_generations":18446744073709551615,"live_pending_slots":256},"invocation":{"nodes":65536,"words":262144,"data_bytes":1048576,"native_int_limbs":65536},"persistent":{"nodes":65536,"words":262144,"data_bytes":1048576,"native_int_limbs":65536}}"#;

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

fn build(name: &str, work: &Path) -> PathBuf {
    let build_dir = tempfile::tempdir_in(work).expect("private native output directory");
    let profile = work.join("px11-profile.json");
    std::fs::write(&profile, PROFILE).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("native-build")
        .arg(fixture(name))
        .arg(build_dir.path())
        .arg(profile)
        .output()
        .expect("build native PX11 artifact");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{name}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let artifact = String::from_utf8(output.stdout).expect("artifact path is UTF-8");
    let artifact = PathBuf::from(artifact.trim());
    assert!(
        artifact.starts_with(build_dir.path()) && artifact.is_file(),
        "{artifact:?}"
    );
    let _ = build_dir.keep();
    artifact
}

fn assert_parity(name: &str, interp: Output, native: Output) {
    assert_eq!(
        native.status.code(),
        interp.status.code(),
        "{name}: native={native:?}, interp={interp:?}"
    );
    assert_eq!(native.stdout, interp.stdout, "{name}: stdout");
    assert_eq!(native.stderr, interp.stderr, "{name}: stderr");
    assert_success(name, interp);
}

fn run_parity(name: &str, cwd: &Path, args: &[&str]) {
    let work = tempfile::tempdir().unwrap();
    let artifact = build(name, work.path());
    let interp = run(name, cwd, args);
    let native = Command::new(artifact)
        .current_dir(cwd)
        .args(args)
        .output()
        .unwrap();
    assert_parity(name, interp, native);
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

/// Promise class: durable invariant.
/// MEASURED: both lanes exchange the known payload and observe EOF, then leave
/// a socket pathname only for Unix. CLAIMED: stream operations agree across
/// families. THE GAP: this row measures no capacity; a change in a Net
/// operation's arena cost is not detected here.
#[test]
fn px11_all_three_families_exchange_exact_bytes_and_observe_eof() {
    let work = tempfile::tempdir().unwrap();
    let artifact = build("px11_stream_exchange.ken", work.path());
    for family in ["4", "6", "unix"] {
        // Use the same pathname with a fresh filesystem state on each lane.
        let root = tempfile::Builder::new()
            .prefix("ken-px11-")
            .tempdir_in("/tmp")
            .unwrap();
        let unix_path = root.path().join("listener.sock");
        let unix = unix_path.to_str().unwrap();
        std::fs::write(root.path().join("px11-payload.bin"), b"PX11ok!").unwrap();
        let interp = run("px11_stream_exchange.ken", root.path(), &[family, unix]);
        assert_eq!(
            unix_path.exists(),
            family == "unix",
            "{family}: interpreter; {interp:?}"
        );
        if family == "unix" {
            std::fs::remove_file(&unix_path).unwrap();
        }
        let native = Command::new(&artifact)
            .current_dir(root.path())
            .args([family, unix])
            .output()
            .unwrap();
        assert_eq!(
            unix_path.exists(),
            family == "unix",
            "{family}: native; {native:?}"
        );
        assert_parity(family, interp, native);
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
    run_parity("px11_missing_listener.ken", dir.path(), &[path]);
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
    run_parity(
        "px11_wrong_grant.ken",
        dir.path(),
        &[path.to_str().unwrap()],
    );
}

#[derive(Clone, Copy, Debug)]
enum UnixRootCase {
    ListenInside,
    ListenOutside,
    ListenLeafSymlink,
    ListenInRootSymlink,
    ConnectInside,
    ConnectInRootSymlink,
    ConnectOutside,
    ParentEscape,
    ParentSymlink,
    LeafSymlink,
}

/// MEASURED: a checked program with FS AFull "./jail" and Net Unix uses the
/// real interpreter/host dispatch to bind or connect at a named path. Outside
/// peers are live, outside bind leaves no inode, and symlink aliases and their
/// inside/outside peers stay unchanged. CLAIMED: Unix acquisition cannot extend
/// the presented FS root or follow a symlink, even to an in-root target.
/// THE GAP: this samples one canonical Linux temp root; descriptor pinning and
/// the other roots/error classes have separate host-level tests and review.
fn run_unix_root_case(case: UnixRootCase, artifact: Option<&Path>) -> Output {
    use std::os::unix::fs::{FileTypeExt, MetadataExt, symlink};
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
    let inside_peer_path = jail.join("svc.sock");
    let _inside_peer = UnixListener::bind(&inside_peer_path).expect("live inside peer");
    let outside_peer_path = outside.join("svc.sock");
    let _outside_peer = UnixListener::bind(&outside_peer_path).expect("live outside peer");
    symlink("../outside", jail.join("link")).unwrap();
    let alias_path = jail.join("alias.sock");
    symlink("../outside/svc.sock", &alias_path).unwrap();
    let inroot_alias_path = jail.join("inroot-alias.sock");
    symlink("svc.sock", &inroot_alias_path).unwrap();
    let observed_link_and_peer = match case {
        UnixRootCase::ListenLeafSymlink => Some((&alias_path, &outside_peer_path)),
        UnixRootCase::ListenInRootSymlink | UnixRootCase::ConnectInRootSymlink => {
            Some((&inroot_alias_path, &inside_peer_path))
        }
        _ => None,
    };
    let leaf_state = observed_link_and_peer.map(|(alias, peer_path)| {
        let link = std::fs::symlink_metadata(alias).unwrap();
        assert!(
            link.file_type().is_symlink(),
            "requested leaf must be a symlink"
        );
        let peer = std::fs::metadata(peer_path).unwrap();
        assert!(
            peer.file_type().is_socket(),
            "symlink target must be a live socket"
        );
        (
            std::fs::read_link(alias).unwrap(),
            (link.dev(), link.ino()),
            (peer.dev(), peer.ino()),
        )
    });
    let inside_made = jail.join("made.sock");
    let outside_made = outside.join("made.sock");
    assert!(!outside_made.exists());
    let (operation, address, expected) = match case {
        UnixRootCase::ListenInside => ("listen", inside_made.clone(), 0),
        UnixRootCase::ListenOutside => ("listen", outside_made.clone(), 84),
        UnixRootCase::ListenLeafSymlink => ("listen", alias_path.clone(), 84),
        UnixRootCase::ListenInRootSymlink => ("listen", inroot_alias_path.clone(), 84),
        UnixRootCase::ConnectInside => ("connect", inside_peer_path.clone(), 0),
        UnixRootCase::ConnectInRootSymlink => ("connect", inroot_alias_path.clone(), 84),
        UnixRootCase::ConnectOutside => ("connect", outside.join("svc.sock"), 84),
        UnixRootCase::ParentEscape => ("connect", jail.join("../outside/svc.sock"), 84),
        UnixRootCase::ParentSymlink => ("connect", jail.join("link/svc.sock"), 84),
        UnixRootCase::LeafSymlink => ("connect", alias_path.clone(), 84),
    };
    let args = [operation, address.to_str().expect("UTF-8 temp pathname")];
    let output = match artifact {
        None => run("rt_net_unix_root_scope.ken", &root, &args),
        Some(artifact) => Command::new(artifact)
            .current_dir(&root)
            .args(args)
            .output()
            .unwrap(),
    };
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
    if let Some(((alias, peer_path), (target, link_identity, peer_identity))) =
        observed_link_and_peer.zip(leaf_state)
    {
        let link = std::fs::symlink_metadata(alias).expect("requested symlink survives");
        assert!(link.file_type().is_symlink());
        assert_eq!(std::fs::read_link(alias).unwrap(), target);
        assert_eq!((link.dev(), link.ino()), link_identity);
        let peer = std::fs::metadata(peer_path).expect("symlink target peer survives");
        assert!(peer.file_type().is_socket());
        assert_eq!((peer.dev(), peer.ino()), peer_identity);
        let _connection = UnixStream::connect(peer_path)
            .expect("symlink target peer remains live after refused acquisition");
    }
    output
}

/// Use fresh listener/path state on each lane. THE GAP: this row measures no
/// capacity; a change in a Net operation's arena cost is not detected here.
fn assert_unix_root_case(case: UnixRootCase) {
    let work = tempfile::tempdir().unwrap();
    let artifact = build("rt_net_unix_root_scope.ken", work.path());
    let interp = run_unix_root_case(case, None);
    let native = run_unix_root_case(case, Some(&artifact));
    assert_eq!(
        native.status.code(),
        interp.status.code(),
        "{case:?}: exit status"
    );
    assert_eq!(native.stdout, interp.stdout, "{case:?}: stdout");
    assert_eq!(native.stderr, interp.stderr, "{case:?}: stderr");
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

/// Promise class: durable invariant. MEASURED: a live in-root socket reached
/// through an in-root symlink returns ResourceHostIO CapabilityDenied on
/// NetListen, retaining the symlink and peer inode/connection. CLAIMED: the
/// blanket leaf-symlink refusal applies even when its target stays in-root.
/// THE GAP: CLI cannot count socket syscalls; the guard order needs review.
#[test]
fn unix_listen_in_root_target_symlink_refuses_without_path_change() {
    assert_unix_root_case(UnixRootCase::ListenInRootSymlink);
}

/// Promise class: durable invariant; connect to a live in-root peer succeeds.
#[test]
fn unix_connect_inside_declared_fs_root_reaches_live_peer() {
    assert_unix_root_case(UnixRootCase::ConnectInside);
}

/// Promise class: durable invariant. MEASURED: an in-root leaf symlink to an
/// in-root live socket returns ResourceHostIO CapabilityDenied on NetConnect,
/// preserving link and peer inode/connection. CLAIMED: root confinement does
/// not permit following any symlink, including an in-root alias. THE GAP: CLI
/// does not count syscalls; guard ordering needs independent review.
#[test]
fn unix_connect_in_root_target_symlink_refuses_without_path_change() {
    assert_unix_root_case(UnixRootCase::ConnectInRootSymlink);
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
