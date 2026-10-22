//! Black-box `ken check` verdicts for refined-result structural recursion.

use std::process::Command;

const ALIAS: &str = "def AnyNat = { x : Nat | Top }\n";

fn check(source: &str) -> (bool, String) {
    let dir = tempfile::tempdir().expect("unique fixture directory");
    let input = dir.path().join("companions.ken");
    std::fs::write(&input, format!("{ALIAS}{source}\n")).expect("write fixture");
    let output = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("check")
        .arg(&input)
        .output()
        .expect("ken check runs");
    (
        output.status.success(),
        String::from_utf8(output.stderr).expect("diagnostic is UTF-8"),
    )
}

fn accepted_with_open_goals(source: &str, expected: usize) {
    let (success, stderr) = check(source);
    assert!(success, "expected admitted recursion: {stderr}");
    assert!(
        stderr.contains(&format!(
            "ken check: {expected} open obligation(s), status unknown"
        )),
        "open obligations must remain visible: {stderr}"
    );
    assert_eq!(
        stderr.matches("unknown g.refinement.").count()
            + stderr.matches("unknown g.ensures.").count(),
        expected,
        "one reported site per goal: {stderr}"
    );
}

fn refused_k0014(source: &str) {
    let (success, stderr) = check(source);
    assert!(!success, "non-decreasing recursion must not accept");
    assert!(
        stderr.contains("error[K0014]")
            && stderr.contains("SCT: idempotent self-loop has no strictly-decreasing parameter"),
        "expected the SCT refusal rather than another failure: {stderr}"
    );
}

/// Promise class: durable invariant (spec 17 §4; spec 21 §5).
/// MEASURED: ken check admits descending recursion, prints three open goal
/// sites and status unknown. CLAIMED: companion admission preserves checked
/// recursion and its original obligations. THE GAP: exact trust identities
/// are observed by the separate elaborator and kernel tests, not this CLI.
#[test]
fn direct_dependent_result_accepts_with_open_obligations() {
    accepted_with_open_goals(
        "fn g (n : AnyNat) : { y : Nat | Equal Nat y n } = \
         match n { Zero |-> Zero ; Suc m |-> g m }",
        3,
    );
}

#[test]
fn suc_wrapped_dependent_result_accepts_with_open_obligations() {
    accepted_with_open_goals(
        "fn g (n : AnyNat) : { y : Nat | Equal Nat y n } = \
         match n { Zero |-> Zero ; Suc m |-> Suc (g m) }",
        3,
    );
}

#[test]
fn ensures_result_accepts_with_open_obligations() {
    accepted_with_open_goals(
        "fn g (n : AnyNat) : Nat ensures Equal Nat result n = \
         match n { Zero |-> Zero ; Suc m |-> Suc (g m) }",
        3,
    );
}

#[test]
fn accumulator_result_accepts_with_open_obligations() {
    accepted_with_open_goals(
        "fn g (n : AnyNat) (acc : Nat) : { y : Nat | Equal Nat y n } = \
         match n { Zero |-> Zero ; Suc m |-> g m (Suc acc) }",
        3,
    );
}

/// Promise class: durable invariant (spec 17 §4).
/// MEASURED: source-level non-descent and an unrelated literal-pair projection
/// preserve K0014; the descending plain control admits with one open goal.
/// CLAIMED: induced result projections cannot bypass SCT for unrelated cases.
/// THE GAP: the kernel probes separately exercise indirect companion cycles.
#[test]
fn unchanged_argument_remains_k0014() {
    refused_k0014(
        "fn g (n : AnyNat) : { y : Nat | Equal Nat y n } = \
         match n { Zero |-> Zero ; Suc m |-> g n }",
    );
}

#[test]
fn rebuilt_argument_remains_k0014() {
    refused_k0014(
        "fn g (n : AnyNat) : { y : Nat | Equal Nat y n } = \
         match n { Zero |-> Zero ; Suc m |-> g (Suc m) }",
    );
}

#[test]
fn literal_pair_projection_remains_k0014() {
    refused_k0014(
        "fn g (n : AnyNat) : Nat = match n { \
         Zero |-> ((Zero, Zero) : (v : Nat) × Nat).1 ; Suc m |-> g m }",
    );
}

#[test]
fn plain_descending_control_keeps_one_open_goal() {
    accepted_with_open_goals(
        "fn g (n : AnyNat) : Nat = \
         match n { Zero |-> Zero ; Suc m |-> g m }",
        1,
    );
}
