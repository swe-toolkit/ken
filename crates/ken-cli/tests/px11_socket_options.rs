//! Spec 38 §1.13.2, PX11 typed socket option families. Each interpreter row
//! reaches the checked Ken surface, the reference interpreter and the shared
//! Linux socket leaves. The native lane refuses every option-family
//! operation at build time; the stream-only control is
//! `px11_stream_only_program_still_builds_and_runs_natively` in the datagram
//! suite, which builds the same stream fixture natively.

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

/// A fresh canonical root, and the Unix pathname a row may bind in it.
fn root_with_path(leaf: &str) -> (tempfile::TempDir, PathBuf, PathBuf) {
    let root = tempfile::Builder::new()
        .prefix("ken-px11-opt-")
        .tempdir_in("/tmp")
        .unwrap();
    let canonical = std::fs::canonicalize(root.path()).unwrap();
    let path = canonical.join(leaf);
    (root, canonical, path)
}

fn readback(selector: &str) -> (Output, PathBuf, tempfile::TempDir) {
    let (root, canonical, path) = root_with_path("listener.sock");
    let output = run(
        "px11_option_readback.ken",
        &canonical,
        &[selector, path.to_str().unwrap()],
    );
    (output, path, root)
}

/// SOURCE: spec/30-surface/38-ffi-io.md §1.13.2 (AC-1 R1).
/// Promise class: durable invariant. MEASURED: over IPv4 loopback, on the
/// outbound stream and on the stream accepted from it, NoDelay and KeepAlive
/// each read False when fresh, True after a True set and False after a False
/// set, and the program exits 0. CLAIMED: both options set and read back on
/// an IPv4 stream of either origin. THE GAP: the readback is the kernel's
/// value by the host-unit rows
/// `stream_options_read_back_the_kernel_value_on_every_ip_stream` and
/// `stream_option_readback_is_the_backend_answer_not_the_last_set`; this row
/// cannot tell a kernel read from an echo on its own.
#[test]
fn px11_option_ipv4_streams_read_back_each_option() {
    let (output, _path, _root) = readback("4");
    assert_eq!(output.status.code(), Some(0), "{}", describe(&output));
}

/// SOURCE: spec/30-surface/38-ffi-io.md §1.13.2 (AC-1 R1).
/// Promise class: durable invariant. The IPv6 loopback form of the IPv4 row,
/// on the outbound and the accepted stream, with the same
/// MEASURED/CLAIMED/GAP statement for `::1`.
#[test]
fn px11_option_ipv6_streams_read_back_each_option() {
    let (output, _path, _root) = readback("6");
    assert_eq!(output.status.code(), Some(0), "{}", describe(&output));
}

/// SOURCE: spec/30-surface/38-ffi-io.md §1.13.2 (AC-1 R4).
/// Promise class: durable invariant. MEASURED: under Net Unix plus FS AFull,
/// on the Unix outbound stream and the stream accepted from it, set and get
/// of NoDelay and KeepAlive each return ResourceHostIO InvalidInput, and the
/// program exits 0; the listener's pathname exists, so the streams were
/// real. CLAIMED: an option on a Unix stream refuses by the typed error
/// before any syscall. THE GAP: the CLI counts no syscalls; the zero-leaf
/// order is the host-unit row
/// `unix_streams_unknown_codes_revocation_and_release_refuse_before_any_leaf`.
/// AC-3 M-fit removes the fit check and this row exits 95: the NoDelay leaf
/// is reached and its ENOTSUP arrives as ResourceHostIO IOError.Unsupported,
/// by the host's canonical errno mapping.
#[test]
fn px11_option_unix_streams_refuse_both_options() {
    let (output, path, _root) = readback("unix");
    assert_eq!(output.status.code(), Some(0), "{}", describe(&output));
    assert!(path.exists(), "the Unix listener was bound");
}

fn reuse(mode: &str) -> (Output, PathBuf, tempfile::TempDir) {
    let (root, canonical, path) = root_with_path("reusable.sock");
    let output = run(
        "px11_option_reuse.ken",
        &canonical,
        &[mode, path.to_str().unwrap()],
    );
    (output, path, root)
}

/// SOURCE: spec/30-surface/38-ffi-io.md §1.13.2 (AC-1 R2).
/// Promise class: durable invariant. MEASURED: in one process, a reusable
/// IPv4 listener on port zero accepts one connection, the accepted stream is
/// released before the client and the listener after both, and a second
/// reusable listener on the same port is acquired (exit 0); the twin with
/// both listeners plain, and the mixed run with a plain first listener and
/// a reusable second, each exit 98, the classified ResourceHostIO
/// (IOError.Other 98). CLAIMED: address reuse rebinds a port a closed
/// connection left behind exactly when the closed connection came from a
/// reusable listener and the rebinding listener is reusable. THE GAP: the
/// rows do not read TIME_WAIT; that the released server side is in
/// TIME_WAIT is the Architect's measured premise for this release order.
/// AC-3 M-order sets the option after bind() and the rebind arm exits 98.
#[test]
fn px11_option_reusable_listener_rebinds_only_with_reuse_on_both_sides() {
    for (mode, expected) in [("rr", 0), ("pp", 98), ("pr", 98)] {
        let (output, _path, _root) = reuse(mode);
        assert_eq!(output.status.code(), Some(expected), "{mode}: {}", describe(&output));
    }
}

/// SOURCE: spec/30-surface/38-ffi-io.md §1.13.2 (AC-1 R3).
/// Promise class: durable invariant. MEASURED: a second reusable listener on
/// the port of a live reusable listener exits 98, the classified
/// ResourceHostIO (IOError.Other 98). CLAIMED: address reuse never admits a
/// second listener on a live listener's port. THE GAP: IPv4 only.
#[test]
fn px11_option_reusable_listener_never_takes_a_live_port() {
    let (output, _path, _root) = reuse("live");
    assert_eq!(output.status.code(), Some(98), "{}", describe(&output));
}

/// SOURCE: spec/30-surface/38-ffi-io.md §1.13.2 (AC-1 R5).
/// Promise class: durable invariant. MEASURED: under Net Unix plus FS AFull,
/// a reusable listener on a Unix path beneath the root exits 22, the
/// classified ResourceHostIO InvalidInput, and no inode exists at the path.
/// CLAIMED: the reusable acquisition is IP only and its Unix refusal precedes
/// any syscall. THE GAP: the absent inode is the measured no-bind evidence;
/// the zero-leaf order is the host-unit row
/// `reusable_listener_refuses_unix_and_sets_reuse_only_on_its_own_acquisition`.
/// AC-3 M-unix-reuse removes the refusal and this row exits 0 with the inode.
#[test]
fn px11_option_reusable_listener_refuses_unix_without_inode() {
    let (output, path, _root) = reuse("unix");
    assert_eq!(output.status.code(), Some(22), "{}", describe(&output));
    assert!(
        std::fs::symlink_metadata(&path).is_err(),
        "a refused reusable Unix listen must not create an inode"
    );
}

/// SOURCE: spec/30-surface/38-ffi-io.md §1.13.2 (AC-1 R6).
/// Promise class: durable invariant. MEASURED: a program with no Net
/// declaration whose main visits Net and calls the reusable listener fails
/// before main runs, with neither exit 0 nor the CapabilityDenied branch's
/// 84, and no inode exists at the Unix path it would bind. CLAIMED: without
/// a Net grant no option-family acquisition is reachable. THE GAP: the
/// refusal is program admission's, a build-time stop; the run-time
/// CapabilityDenied without a grant is the host-unit reusable-listener row.
#[test]
fn px11_option_program_without_net_is_refused_before_listen() {
    let (_root, canonical, path) = root_with_path("never.sock");
    let output = run("px11_option_no_net.ken", &canonical, &[path.to_str().unwrap()]);
    assert_ne!(output.status.code(), Some(0), "{}", describe(&output));
    assert_ne!(output.status.code(), Some(84), "refused before main: {}", describe(&output));
    assert!(std::fs::symlink_metadata(&path).is_err(), "no listen ran");
}

/// SOURCE: spec/30-surface/38-ffi-io.md §1.13.2 (AC-1 R7).
/// Promise class: durable invariant. MEASURED: a program applying
/// set_stream_option to a Resource ResourceKind.SocketListener is refused
/// at elaboration with a K0004 kernel TypeMismatch, it never runs (no inode
/// at the listener path it would bind), and a native build of it emits no
/// artifact. The well-typed twin, the same call on a SocketStream, is the
/// readback fixture, which elaborates and runs. CLAIMED: option calls on a listener are excluded by
/// type. THE GAP: the diagnostic prints kernel identities, not kind names,
/// so the row does not pin which kinds mismatched; one wrong kind, and the
/// datagram exclusion is the same index and is not separately run.
#[test]
fn px11_option_call_on_listener_fails_elaboration() {
    let (_root, canonical, path) = root_with_path("typed.sock");
    let output = run("px11_option_listener_type.ken", &canonical, &[path.to_str().unwrap()]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_ne!(output.status.code(), Some(0), "{}", describe(&output));
    assert!(
        stderr.contains("error[K0004]") && stderr.contains("TypeMismatch"),
        "an elaboration type mismatch: {}",
        describe(&output)
    );
    assert!(std::fs::symlink_metadata(&path).is_err(), "the program never ran");
    let work = tempfile::tempdir().unwrap();
    let (built, build_dir) = native_build("px11_option_listener_type.ken", work.path());
    assert_ne!(built.status.code(), Some(0), "{}", describe(&built));
    let produced = std::fs::read_dir(&build_dir).map(|entries| entries.count()).unwrap_or(0);
    assert_eq!(produced, 0, "a refused build emits no artifact");
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
        (0x070C..=0x070E).contains(&code),
        "{name}: the refusal names an option-family operation, 1804 to 1806: {refusal}"
    );
    let produced = std::fs::read_dir(&build_dir)
        .map(|entries| entries.count())
        .unwrap_or(0);
    assert_eq!(produced, 0, "{name}: a refused build emits no artifact");
}

/// SOURCE: spec/30-surface/38-ffi-io.md §1.13.2 (AC-2).
/// Promise class: durable invariant. MEASURED: native-building each AC-1
/// fixture that calls a new operation fails with N0001 "is a represented
/// unavailable lane" naming one of the option-family host-family codes 1804
/// to 1806, and leaves no artifact. CLAIMED: the native lane refuses the
/// option-family operations at build time by a typed cause and never
/// miscompiles. THE GAP: which operation lowering reaches first is
/// incidental to source order and is not pinned; the stream-only control is
/// in the datagram suite.
#[test]
fn px11_option_programs_refuse_native_build_by_typed_cause() {
    for name in ["px11_option_readback.ken", "px11_option_reuse.ken"] {
        assert_native_refusal(name);
    }
}
