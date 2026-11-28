//! RT-NATIVE-CARRIED-RESPONSE-CONSTRUCTOR-MATCH. A bracket continuation that
//! passes a carried host-response field into a function which builds a
//! constructor from it and matches that constructor by nested patterns builds
//! natively and exits with the interpreter's status.
//!
//! The match compiler reverts tail columns over a scrutinee erasure cannot
//! reduce, which leaves an application of a dynamic match whose arms are
//! lexical closures. Erasure commutes that application into the arms
//! (`apply_runtime_callee`), so native lowering never joins on a closure.
//! Genuine closure values are unchanged and still meet the existing boundary
//! refusal.
//!
//! Every row runs through the `ken` binary with the PX11 parity profile and a
//! private tempdir as the default FS root and working directory.
//!
//! MEASURED: the four carried rows (p6, w1, w2, g2) build and agree with the
//! interpreter on exit status, stdout and stderr; the literal control (p1)
//! does too; the genuine-closure (g1) and partial two-column (m2) rows still
//! refuse with N0001 while the interpreter runs them. CLAIMED: the commute
//! admits exactly saturated applications of closure-armed matches to
//! duplicable arguments. THE GAP: the population is these seven programs.
//! A partial multi-column convoy (m2), a computational-match callee, a
//! non-duplicable argument (an effect, call, let or closure) and a match in
//! argument position are not admitted, and no row here proves them refused
//! for any other shape.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const PROFILE: &str = r#"{"runtime":{"invocation_epochs":18446744073709551615},"call_events":{"event_generations":18446744073709551615,"live_pending_slots":256},"invocation":{"nodes":65536,"words":262144,"data_bytes":1048576,"native_int_limbs":65536},"persistent":{"nodes":65536,"words":262144,"data_bytes":1048576,"native_int_limbs":65536}}"#;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/rt_native_carried_response_match")
        .join(name)
}

/// A private, canonical default FS root and working directory.
fn root() -> tempfile::TempDir {
    tempfile::Builder::new()
        .prefix("ken-carried-")
        .tempdir_in("/tmp")
        .unwrap()
}

fn canonical(root: &Path) -> PathBuf {
    std::fs::canonicalize(root).expect("canonical default FS root")
}

fn interpret(name: &str, cwd: &Path, arguments: &[&str]) -> Output {
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
    let profile = work.join("carried-profile.json");
    std::fs::write(&profile, PROFILE).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("native-build")
        .arg(fixture(name))
        .arg(build_dir.path())
        .arg(profile)
        .output()
        .expect("run ken native-build");
    (output, build_dir.keep())
}

fn build(name: &str, work: &Path) -> PathBuf {
    let (output, build_dir) = native_build(name, work);
    assert_eq!(
        output.status.code(),
        Some(0),
        "{name}: native-build: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let artifact = String::from_utf8(output.stdout).expect("artifact path is UTF-8");
    let artifact = PathBuf::from(artifact.trim());
    assert!(
        artifact.starts_with(&build_dir) && artifact.is_file(),
        "{name}: {artifact:?}"
    );
    artifact
}

/// Builds natively, runs both lanes with the same arguments in `cwd`, and
/// asserts they agree on everything observable and on `expected`.
fn assert_parity(name: &str, cwd: &Path, arguments: &[&str], expected: i32) {
    let work = tempfile::tempdir().unwrap();
    let artifact = build(name, work.path());
    let interp = interpret(name, cwd, arguments);
    let native = Command::new(artifact)
        .current_dir(cwd)
        .args(arguments)
        .output()
        .unwrap();
    assert_eq!(
        native.status.code(),
        interp.status.code(),
        "{name}: native={native:?}, interp={interp:?}"
    );
    assert_eq!(native.stdout, interp.stdout, "{name}: stdout");
    assert_eq!(native.stderr, interp.stderr, "{name}: stderr");
    assert_eq!(
        interp.status.code(),
        Some(expected),
        "{name}: stdout={} stderr={}",
        String::from_utf8_lossy(&interp.stdout),
        String::from_utf8_lossy(&interp.stderr)
    );
}

/// The native build refuses with N0001 and `reason`, while the interpreter
/// runs the same program, given a missing socket path, to `expected`.
fn assert_refuses(name: &str, reason: &str, expected: i32) {
    let work = tempfile::tempdir().unwrap();
    let (output, _) = native_build(name, work.path());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_ne!(output.status.code(), Some(0), "{name}: built: {stderr}");
    assert!(
        stderr.contains("error[N0001]") && stderr.contains(reason),
        "{name}: expected N0001 `{reason}`, got: {stderr}"
    );
    let root = root();
    let path = missing_socket(root.path());
    let interp = interpret(name, root.path(), &[&path]);
    assert_eq!(
        interp.status.code(),
        Some(expected),
        "{name}: interpreter: {interp:?}"
    );
}

fn missing_socket(root: &Path) -> String {
    let path = canonical(root).join("missing.sock");
    assert!(!path.exists(), "negative case requires no listener");
    path.to_str().unwrap().to_owned()
}

const CLOSURE_BOUNDARY: &str = "Closure: a closure cannot cross the boundary";
const CALLEE_NOT_CLOSURE: &str = "Call: callee is not a closure";

/// Promise class: durable invariant. The FS-only reproducer: a buffer
/// bracket continuation passes its carried `ResourceHostIO` identity to
/// `process_error_to_system` and matches the result by nested patterns. The
/// buffer body succeeds, so both lanes take the permitted `Ok` arm.
#[test]
fn p6_fs_carried_identity_builds_and_agrees() {
    let root = root();
    assert_parity("p6_fs_carried_identity_program.ken", root.path(), &["x"], 0);
}

/// Promise class: durable invariant. A Unix connect under only a Loopback
/// grant answers CapabilityDenied as a carried identity. Converting it by
/// `socket_error_to_system` and matching the envelope takes the True arm.
#[test]
fn w1_carried_wrong_grant_takes_denied_arm() {
    let root = root();
    let path = missing_socket(root.path());
    assert_parity("w1_carried_wrong_grant.ken", root.path(), &[&path], 0);
}

/// Promise class: durable invariant. The same program under a Unix grant
/// reaches the missing path and answers NotFound, so the envelope match takes
/// the False arm and both lanes exit 77. Distinguishes the selected arm from
/// a lowering that always takes one side.
#[test]
fn w2_carried_not_found_takes_other_arm() {
    let root = root();
    let path = missing_socket(root.path());
    assert_parity("w2_carried_not_found.ken", root.path(), &[&path], 77);
}

/// Promise class: durable invariant. A user-written `choose b : Nat → Nat`
/// whose arms are closures, applied at once to `Zero` with a carried-derived
/// scrutinee, commutes into its arms like the compiler-produced form.
#[test]
fn g2_closure_match_applied_at_once_builds_and_agrees() {
    let root = root();
    let path = missing_socket(root.path());
    assert_parity("g2_closure_join.ken", root.path(), &[&path], 0);
}

/// Promise class: durable invariant. The literal-identity control differs
/// from p6 only in passing `IOError.CapabilityDenied`; it builds and agrees
/// as before the change.
#[test]
fn p1_literal_identity_control_builds_and_agrees() {
    let root = root();
    assert_parity("p1_fs_typed_in_cont.ken", root.path(), &["x"], 0);
}

/// Promise class: normative compatibility vector for the N0001 cause text.
/// A genuine closure returned through a bracket result is a closure value,
/// not an applied closure-armed match, so the commute leaves it alone and the
/// native build still refuses. The interpreter runs it to 0.
#[test]
fn g1_genuine_bracket_closure_still_refuses() {
    assert_refuses("g1_genuine_closure.ken", CALLEE_NOT_CLOSURE, 0);
}

/// Promise class: normative compatibility vector for the N0001 cause text.
/// A two-column convoy over a constructed `MkTriple` leaves a partial
/// application, so the arity gate does not commute it and the closure
/// boundary refusal stands. The interpreter runs it to 0.
#[test]
fn m2_partial_two_column_convoy_still_refuses() {
    assert_refuses("m2_two_columns.ken", CLOSURE_BOUNDARY, 0);
}
