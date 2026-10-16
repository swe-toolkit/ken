//! The CLI must check a deep dependent goal on its sized elaboration worker.
//! Promise class: durable CLI acceptance, spec 33 §8.3 and 39 §2.1. A
//! 256-MiB worker moves the native-stack cliff; behavior past it is held AC-2.

use std::path::Path;
use std::process::{Command, Output};

fn r1_program(depth: usize, with_main: bool) -> String {
    let n = (0..depth).fold(String::from("Zero"), |n, _| format!("Suc ({n})"));
    let left = format!(
        "TR a (build ({n}) (Nil Nat)) (dual_lookup a (Suc m) xs (DualFZ m)) (dual_lookup a (Suc m) ys (DualFZ m))"
    );
    let mut source = format!(
        "data DualFin : Nat -> Type where {{ DualFZ : (n : Nat) -> DualFin (Suc n); DualFS : (n : Nat) -> DualFin n -> DualFin (Suc n) }}\n\
         data DualEnv (a : Type) : Nat -> Type where {{ DualNil : DualEnv a Zero; DualCons : (n : Nat) -> a -> DualEnv a n -> DualEnv a (Suc n) }}\n\
         fn dual_lookup (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : a = match i {{ DualFZ m ↦ match xs {{ DualCons k x rest ↦ x }}; DualFS m j ↦ match xs {{ DualCons k x rest ↦ dual_lookup a m rest j }} }}\n\
         data Tri (a : Type) : Type where {{ TR : List Nat -> a -> a -> Tri a }}\n\
         fn build (n : Nat) (acc : List Nat) : List Nat = match n {{ Zero ↦ acc; Suc m ↦ build m (Cons Nat Zero acc) }}\n\
         theorem r1 (a : Type) (m : Nat) (xs : DualEnv a (Suc m)) (ys : DualEnv a (Suc m)) : Equal (Tri a) ({left}) ({left}) = match xs {{ DualCons k x rest ↦ match ys {{ DualCons k2 y rest2 ↦ Refl }} }}\n"
    );
    if with_main {
        source.insert_str(0, "program capabilities FS APartial\n");
        source.push_str(
            "proc main (_input : ProcessInput) (_caps : ProgramCaps APartial) : HostIO APartial ExitCode visits [Console] = host_program APartial (print_line \"stack worker reached main\")\n",
        );
    }
    source
}

// The child process main thread has exactly an 8-MiB stack, independently
// of its caller's limit. The production worker is separately stated at
// 256 MiB. RUST_MIN_STACK is removed so neither path inherits ambient policy.
fn invoke(command: &str, path: &Path) -> Output {
    #[cfg(unix)]
    let mut ken = {
        let mut shell = Command::new("sh");
        shell
            .arg("-c")
            .arg("ulimit -c 0; ulimit -s 8192 || exit 77; exec \"$@\"")
            .arg("ken-native-stack-test")
            .arg(env!("CARGO_BIN_EXE_ken"));
        shell
    };
    #[cfg(not(unix))]
    let mut ken = Command::new(env!("CARGO_BIN_EXE_ken"));

    ken.arg(command)
        .arg(path)
        .env_remove("RUST_MIN_STACK")
        .output()
        .expect("spawn Ken CLI")
}

fn fixture(depth: usize, with_main: bool) -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("fixture directory");
    let path = dir.path().join("deep_goal.ken");
    std::fs::write(&path, r1_program(depth, with_main)).expect("write Ken fixture");
    (dir, path)
}

/// MEASURED: the K=0 version of the same source passes `ken check` on an
/// 8-MiB main thread. CLAIMED: the fixture is well formed without depth.
/// THE GAP: this does not exercise the large spine or the worker boundary.
#[test]
fn shallow_goal_control_checks_on_eight_mib_main() {
    let (_dir, path) = fixture(0, false);
    let output = invoke("check", &path);
    assert!(
        output.status.success(),
        "shallow ken check must pass: status={:?} stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
}

/// MEASURED: `ken check` accepts K=160 on a child whose main stack is 8 MiB.
/// CLAIMED: the CLI's 256-MiB worker protects the dependent goal through
/// elaboration and kernel re-check. THE GAP: a larger goal can still overflow
/// the worker; this only tests the ruled closure (a), not operator-held AC-2.
#[test]
fn deep_goal_checks_through_cli_worker() {
    let (_dir, path) = fixture(160, false);
    let output = invoke("check", &path);
    assert!(
        output.status.success(),
        "K=160 ken check must return Ok: status={:?} stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.is_empty(), "ken check must not run main");
}

/// MEASURED: the same deep checked theorem precedes a runnable `main`, and
/// `ken run` emits its real output. CLAIMED: the run path also reaches the
/// worker, without swallowing the program's exit or changing its stdout.
/// THE GAP: this does not cover programs beyond the 256-MiB worker cliff.
#[test]
fn deep_goal_runs_through_cli_worker() {
    let (_dir, path) = fixture(160, true);
    let output = invoke("run", &path);
    assert!(
        output.status.success(),
        "K=160 ken run must return Ok: status={:?} stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout, b"stack worker reached main\n",
        "ken run must still drive its actual main"
    );
}

/// MEASURED: the same proof at K=640 still checks through the release CLI.
/// CLAIMED: installing the worker does not turn a previously checking release
/// row into a refusal. THE GAP: a single fixed K cannot establish every
/// possible larger proof goal; past-worker exhaustion remains operator-held.
#[cfg(not(debug_assertions))]
#[test]
fn release_goal_at_six_forty_still_checks() {
    let (_dir, path) = fixture(640, false);
    let output = invoke("check", &path);
    assert!(
        output.status.success(),
        "release K=640 ken check must still pass: status={:?} stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
}
