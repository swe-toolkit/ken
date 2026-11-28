//! Spec 38 §1.13.1, PX11 datagram sockets. Each interpreter row reaches the
//! checked Ken surface, the reference interpreter and the shared Linux
//! datagram leaf. The native lane refuses every datagram operation at build
//! time; the stream-only control still builds and runs natively.

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

fn native_build(name: &str, work: &Path) -> (Output, PathBuf) {
    let build_dir = tempfile::tempdir_in(work).expect("private native output directory");
    let profile = work.join("px11-profile.json");
    std::fs::write(&profile, PROFILE).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("native-build")
        .arg(fixture(name))
        .arg(build_dir.path())
        .arg(profile)
        .output()
        .expect("invoke native build");
    (output, build_dir.keep())
}

fn describe(output: &Output) -> String {
    format!(
        "status={:?} stdout={} stderr={}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn temp_root() -> tempfile::TempDir {
    tempfile::Builder::new()
        .prefix("ken-px11-dg-")
        .tempdir_in("/tmp")
        .unwrap()
}

/// One exchange row in a fresh root holding the seven-byte payload. Returns
/// the run and the two Unix pathnames the row may bind.
fn exchange(selector: &str) -> (Output, PathBuf, PathBuf, tempfile::TempDir) {
    let root = temp_root();
    let canonical = std::fs::canonicalize(root.path()).unwrap();
    std::fs::write(canonical.join("px11-payload.bin"), b"PX11ok!").unwrap();
    let receiver = canonical.join("receiver.sock");
    let sender = canonical.join("sender.sock");
    let output = run(
        "px11_datagram_exchange.ken",
        &canonical,
        &[selector, receiver.to_str().unwrap(), sender.to_str().unwrap()],
    );
    (output, receiver, sender, root)
}

fn assert_exchange(selector: &str) {
    let (output, receiver, sender, _root) = exchange(selector);
    assert_eq!(output.status.code(), Some(0), "{selector}: {}", describe(&output));
    let unix = selector == "unix";
    assert_eq!(receiver.exists(), unix, "{selector}: receiver pathname");
    assert_eq!(sender.exists(), unix, "{selector}: release does not unlink the sender");
}

/// Promise class: durable invariant. MEASURED: over IPv4 loopback, the
/// receiver reads its port-zero local address, the sender sends the fixed
/// seven-byte payload, and the receiver's 16-byte window gets exactly those
/// bytes with WholeDatagram and Some sender; replying to that sender reaches
/// the sending socket. CLAIMED: an IPv4 datagram arrives whole with a sender
/// that names the sending socket. THE GAP: Ken has no SocketAddress equality,
/// so the fixture proves the sender by routing a reply to it; byte-exact
/// equality with the sender's local address is the host-unit row
/// `three_datagram_families_report_exact_bytes_and_sender`.
#[test]
fn px11_datagram_ipv4_exchange_reports_bytes_fit_and_routable_sender() {
    assert_exchange("4");
}

/// Promise class: durable invariant. The IPv6 loopback form of the IPv4 row,
/// with the same MEASURED/CLAIMED/GAP statement for `::1`.
#[test]
fn px11_datagram_ipv6_exchange_reports_bytes_fit_and_routable_sender() {
    assert_exchange("6");
}

/// Promise class: durable invariant. MEASURED: two Unix datagram sockets
/// bound beneath the FS root exchange the payload whole; the receiver's
/// reported sender is admitted by the root rule and routes a reply to the
/// sending socket, which a `/proc/<pid>/fd` spelling outside the root could
/// not; both pathnames remain after release. CLAIMED: the Unix sender is
/// reported as its validated bind address. THE GAP: byte-exact equality is
/// the host-unit row; the unlink-and-rebind residual is not exercised.
#[test]
fn px11_datagram_unix_exchange_reports_bytes_fit_and_validated_sender() {
    assert_exchange("unix");
}

/// Promise class: durable invariant. MEASURED: the seven-byte payload
/// received into a window of six delivers "PX11ok" with TruncatedDatagram,
/// and the six-byte prefix replied to the sender arrives whole. CLAIMED:
/// truncation is reported as a typed fit, exactly when the datagram exceeds
/// the window. THE GAP: dropping MSG_TRUNC from the receive leaf turns the
/// fit into WholeDatagram and this row exits 54; that mutation is run by hand.
#[test]
fn px11_datagram_truncation_reports_truncated_fit_and_prefix() {
    assert_exchange("t");
}

/// Run the rooted bind fixture: FS AFull "./jail", Net Unix.
fn unix_root_bind(leaf_from_root: &str) -> (Output, PathBuf, tempfile::TempDir) {
    let root = temp_root();
    let canonical = std::fs::canonicalize(root.path()).unwrap();
    std::fs::create_dir(canonical.join("jail")).unwrap();
    std::fs::create_dir(canonical.join("outside")).unwrap();
    let target = canonical.join(leaf_from_root);
    let output = run(
        "px11_datagram_unix_root.ken",
        &canonical,
        &[target.to_str().unwrap()],
    );
    (output, target, root)
}

/// Promise class: durable invariant. MEASURED: with FS AFull "./jail" and Net
/// Unix, a datagram bind beneath the jail exits 0 and leaves its socket
/// pathname. CLAIMED: the root rule admits an in-root datagram bind. THE GAP:
/// the control half of the outside-root row.
#[test]
fn px11_datagram_unix_bind_inside_declared_root_creates_socket() {
    let (output, target, _root) = unix_root_bind("jail/bound.sock");
    assert_eq!(output.status.code(), Some(0), "{}", describe(&output));
    assert!(target.exists(), "in-root bind leaves its pathname");
}

/// Promise class: durable invariant. MEASURED: the same program binding a
/// sibling directory outside the jail exits 84, the pattern-matched
/// ResourceHostIO CapabilityDenied branch, and no inode exists at the
/// requested path. CLAIMED: a datagram Unix bind cannot extend the declared
/// FS root, and its refusal precedes socket(). THE GAP: the CLI counts no
/// syscalls; the absent inode is the measured no-bind evidence. AC-3 mutates
/// the datagram bind's `unix_parent` call to resolve beneath `/` and this row
/// reddens.
#[test]
fn px11_datagram_unix_bind_outside_declared_root_refuses_without_inode() {
    let (output, target, _root) = unix_root_bind("outside/escaped.sock");
    assert_eq!(output.status.code(), Some(84), "{}", describe(&output));
    assert!(
        std::fs::symlink_metadata(&target).is_err(),
        "outside bind must not create an inode"
    );
}

/// Promise class: durable invariant. MEASURED: a program declaring only Net
/// Loopback binds a loopback datagram socket, and its send to the Inet peer
/// 192.0.2.1 exits 84, the pattern-matched CapabilityDenied branch. CLAIMED:
/// each send's peer is admitted against the socket's retained grant before
/// any syscall. THE GAP: for IP the no-syscall order is CLAIMED, not
/// measured here: TEST-NET-1 leaves no observable trace. The host row
/// `datagram_send_refusals_precede_the_backend` measures zero leaf calls.
#[test]
fn px11_datagram_loopback_grant_refuses_inet_peer() {
    let root = temp_root();
    std::fs::write(root.path().join("px11-payload.bin"), b"PX11ok!").unwrap();
    let output = run("px11_datagram_wrong_grant.ken", root.path(), &[]);
    assert_eq!(output.status.code(), Some(84), "{}", describe(&output));
}

/// Promise class: durable invariant. MEASURED: a program with no Net
/// declaration whose main visits Net fails before main runs, and no inode
/// exists at the Unix path it would bind. CLAIMED: without a Net grant no
/// datagram bind is reachable. THE GAP: the refusal is program admission's
/// UnsupportedEffectRow, a build-time stop; the run-time CapabilityDenied
/// for a grant without Net is the host row
/// `datagram_bind_refusals_precede_the_backend`.
#[test]
fn px11_datagram_program_without_net_is_refused_before_bind() {
    let root = temp_root();
    let canonical = std::fs::canonicalize(root.path()).unwrap();
    let target = canonical.join("never.sock");
    let output = run("px11_datagram_no_net.ken", &canonical, &[target.to_str().unwrap()]);
    assert_ne!(output.status.code(), Some(0), "{}", describe(&output));
    assert_ne!(output.status.code(), Some(84), "refused before main: {}", describe(&output));
    assert!(std::fs::symlink_metadata(&target).is_err(), "no bind ran");
}

/// Run the stale-sender fixture in a fresh root: B at `b.sock`, A at
/// `a/s.sock` (released after sending), C at `c/<later_leaf>`.
fn stale_sender(later_leaf: &str) -> (Output, PathBuf, tempfile::TempDir) {
    let root = temp_root();
    let canonical = std::fs::canonicalize(root.path()).unwrap();
    std::fs::write(canonical.join("px11-payload.bin"), b"PX11ok!").unwrap();
    std::fs::create_dir(canonical.join("a")).unwrap();
    std::fs::create_dir(canonical.join("c")).unwrap();
    let later = canonical.join("c").join(later_leaf);
    let output = run(
        "px11_datagram_stale_sender.ken",
        &canonical,
        &[
            canonical.join("b.sock").to_str().unwrap(),
            canonical.join("a/s.sock").to_str().unwrap(),
            later.to_str().unwrap(),
        ],
    );
    (output, later, root)
}

/// Promise class: durable invariant. MEASURED: in one interpreter process,
/// A at `a/s.sock` sends seven bytes to B and is released; C binds at
/// `c/s.sock`, the same leaf, so it can record A's released kernel name.
/// Inside C's bracket, B receives A's queued datagram and replies to the
/// reported sender: the reply is refused with `CapabilityDenied` (exit 53),
/// never delivered to C (exit 99 before the fix, and under the AC-2
/// mutation). CLAIMED: a released sender's queued datagram is never
/// reported as the later socket, so a reply cannot reach it. THE GAP: the
/// interpreter cannot observe the recorded names; that the two names
/// collide is shown by the mutation's exit 99 and measured directly by the
/// host row `released_unix_datagram_name_is_not_attributed_to_a_later_bind`.
#[test]
fn px11_datagram_released_sender_is_not_reported_as_a_later_bind() {
    let (output, later, _root) = stale_sender("s.sock");
    assert_eq!(output.status.code(), Some(53), "{}", describe(&output));
    assert!(later.exists(), "C was bound");
}

/// Promise class: durable invariant. MEASURED: the different-leaf control,
/// C at `c/t.sock`: the recorded names differ, the queued datagram's sender
/// is the raw `/proc` spelling, and the reply is refused with
/// `CapabilityDenied` (exit 53), identically with and without the fix.
/// CLAIMED: the reproducer's exit 53 is the existing refusal of a
/// kernel-recorded spelling, not a new refusal. THE GAP: the live
/// non-colliding sender is the Unix exchange row above and the host row
/// `unix_datagram_live_sender_keeps_attribution_beside_a_released_name`.
#[test]
fn px11_datagram_released_sender_control_reports_the_raw_spelling() {
    let (output, later, _root) = stale_sender("t.sock");
    assert_eq!(output.status.code(), Some(53), "{}", describe(&output));
    assert!(later.exists(), "C was bound");
}

fn assert_native_refusal(name: &str) {
    let work = tempfile::tempdir().unwrap();
    let (output, build_dir) = native_build(name, work.path());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_ne!(output.status.code(), Some(0), "{name}: {}", describe(&output));
    let refusal = stderr
        .lines()
        .find(|line| line.starts_with("error["))
        .unwrap_or_else(|| panic!("{name}: no diagnostic: {stderr}"));
    assert!(refusal.starts_with("error[N0001]"), "{name}: typed lowering code: {refusal}");
    let reason = refusal
        .split_once("unsupported runtime-IR lowering: Effect: effect ")
        .unwrap_or_else(|| panic!("{name}: construct Effect: {refusal}"))
        .1;
    let named = reason
        .strip_suffix(" is a represented unavailable lane")
        .unwrap_or_else(|| panic!("{name}: represented-unavailable reason: {refusal}"));
    // The family spelling is incidental; the operation is its host-family code.
    let code = named
        .rsplit_once('.')
        .and_then(|(_, code)| code.parse::<u16>().ok())
        .unwrap_or_else(|| panic!("{name}: no operation code: {refusal}"));
    assert!(
        (0x0708..=0x070B).contains(&code),
        "{name}: the refusal names a datagram operation, 1800 to 1803: {refusal}"
    );
    let produced = std::fs::read_dir(&build_dir)
        .map(|entries| entries.count())
        .unwrap_or(0);
    assert_eq!(produced, 0, "{name}: a refused build emits no artifact");
}

/// Promise class: durable invariant. MEASURED: native-building each datagram
/// fixture fails with N0001 "is a represented unavailable lane" naming one of
/// the four datagram host-family codes 1800 to 1803, and leaves no artifact.
/// CLAIMED: the native lane refuses the datagram operations at build time by
/// a typed cause and never miscompiles. THE GAP: which of the four lowering
/// reaches first is incidental to source order and is not pinned.
#[test]
fn px11_datagram_programs_refuse_native_build_by_typed_cause() {
    for name in [
        "px11_datagram_exchange.ken",
        "px11_datagram_unix_root.ken",
        "px11_datagram_wrong_grant.ken",
    ] {
        assert_native_refusal(name);
    }
}

/// Promise class: durable invariant. MEASURED: the stream-only exchange
/// fixture still builds natively in this suite and its artifact exchanges
/// the payload over IPv4 loopback with exit 0. CLAIMED: the datagram refusal
/// is specific to the datagram operations, not a regression that refuses all
/// Net natively. THE GAP: one family; the stream suite covers all three.
#[test]
fn px11_stream_only_program_still_builds_and_runs_natively() {
    let work = tempfile::tempdir().unwrap();
    let (output, build_dir) = native_build("px11_stream_exchange.ken", work.path());
    assert_eq!(output.status.code(), Some(0), "{}", describe(&output));
    let artifact = PathBuf::from(String::from_utf8(output.stdout).unwrap().trim());
    assert!(artifact.starts_with(&build_dir) && artifact.is_file(), "{artifact:?}");
    let root = temp_root();
    std::fs::write(root.path().join("px11-payload.bin"), b"PX11ok!").unwrap();
    let unix = root.path().join("listener.sock");
    let ran = Command::new(&artifact)
        .current_dir(root.path())
        .args(["4", unix.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(ran.status.code(), Some(0), "{}", describe(&ran));
}
