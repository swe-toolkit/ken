//! The lifeline watcher has a stated stack and a fallible, coded spawn.
//! Promise class: durable CLI worker-boundary behavior (Architect
//! evt_7p88cxwfmwnzf, spec 70 §1.2).

#![cfg(unix)]

use std::process::{Command, Output};

const FAULT_MARKER: &str = "KEN_CLI_WORKER_CHILD_FAULT_FD";
// This is deliberately an ambient stack setting, not a fixture stack size:
// it defeats the old bare thread::spawn by demanding an impossible watcher
// reservation. The repaired watcher's local 64-KiB Builder size ignores it.
const IMPOSSIBLE_AMBIENT_STACK: &str = "100000000000";

fn check(source: &std::path::Path, huge_ambient_stack: bool) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ken"));
    command
        .arg("check")
        .arg(source)
        .env_remove(FAULT_MARKER)
        .env_remove("RUST_MIN_STACK");
    if huge_ambient_stack {
        command.env("RUST_MIN_STACK", IMPOSSIBLE_AMBIENT_STACK);
    }
    command.output().expect("run Ken check")
}

/// MEASURED: the same checked program has status 0 and identical empty stdout
/// both with and without an impossible ambient watcher-stack request. CLAIMED:
/// the fixed-size watcher ignores RUST_MIN_STACK while preserving the normal
/// check result. THE GAP: an uncoded spawn failure would have to be caught;
/// reverting the watcher to bare spawn makes this exact huge-stack row rc 101.
#[test]
fn watcher_ignores_ambient_stack_and_check_keeps_stdout() {
    let directory = tempfile::tempdir().expect("fixture directory");
    let path = directory.path().join("ok.ken");
    std::fs::write(&path, "const one : Nat = Suc Zero\n").expect("write fixture");
    let baseline = check(&path, false);
    assert_eq!(baseline.status.code(), Some(0), "baseline: {baseline:?}");
    assert!(baseline.stderr.is_empty(), "baseline: {baseline:?}");
    let huge = check(&path, true);
    assert_eq!(huge.status.code(), Some(0), "huge ambient: {huge:?}");
    assert_eq!(huge.stdout, baseline.stdout, "stdout changed");
    assert_eq!(huge.stderr, baseline.stderr, "stderr changed");
}
