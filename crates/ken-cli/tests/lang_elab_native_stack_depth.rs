//! Deep CLI parsing and checking use a sized worker instead of the main stack.
//! Promise class: durable CLI acceptance, spec 33 §8.3, 39 §2.1 and 70 §1.
//! A 256-MiB worker moves the native-stack cliff; behavior past it is AC-2.

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
fn invoke(command: &str, path: &Path, additional_paths: &[&Path]) -> Output {
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
        .args(additional_paths)
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
    let output = invoke("check", &path, &[]);
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
    let output = invoke("check", &path, &[]);
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
    let output = invoke("run", &path, &[]);
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
    let output = invoke("check", &path, &[]);
    assert!(
        output.status.success(),
        "release K=640 ken check must still pass: status={:?} stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
}

// Same bounded resource profile as verify_entry_abi_checked_identity.rs.
const NATIVE_RESOURCE_PROFILE: &str = "{\"runtime\":{\"invocation_epochs\":1024},\"call_events\":{\"event_generations\":2048,\"live_pending_slots\":1024},\"invocation\":{\"nodes\":100000,\"words\":100000,\"data_bytes\":1000000,\"native_int_limbs\":100000},\"persistent\":{\"nodes\":100000,\"words\":100000,\"data_bytes\":1000000,\"native_int_limbs\":100000}}";

fn build_and_run_deep_goal(depth: usize) {
    let (_dir, source) = fixture(depth, true);
    let output = tempfile::tempdir().expect("native-build output directory");
    let profile = output.path().join("profile.json");
    let artifact_dir = output.path().join("artifact");
    std::fs::write(&profile, NATIVE_RESOURCE_PROFILE).expect("write native resource profile");
    let built = invoke("native-build", &source, &[&artifact_dir, &profile]);
    assert!(
        built.status.success(),
        "K={depth} native-build must succeed: status={:?} stderr={}",
        built.status,
        String::from_utf8_lossy(&built.stderr)
    );
    let artifact = String::from_utf8(built.stdout).expect("native executable path is UTF-8");
    let executable = std::path::Path::new(artifact.trim());
    assert!(
        executable.starts_with(&artifact_dir),
        "native-build must name the built artifact inside {artifact_dir:?}: {artifact:?}"
    );
    let run = Command::new(executable)
        .output()
        .expect("run emitted native program");
    assert!(
        run.status.success(),
        "K={depth} native program must exit successfully: stderr={}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(run.stdout, b"stack worker reached main\n");
}

/// MEASURED: native-build compiles and runs the K=0 version of the same
/// program on a CLI child with an 8-MiB main-thread stack. CLAIMED: the
/// native-build fixture and profile are valid without the deep spine.
/// THE GAP: K=0 alone does not exercise the stack-consuming walker.
#[test]
fn shallow_goal_native_build_control() {
    build_and_run_deep_goal(0);
}

/// MEASURED: native-build compiles the K=160 dependent goal and its emitted
/// program prints the real main output. CLAIMED: native-build elaboration
/// shares the 256-MiB CLI worker instead of aborting on the 8-MiB main stack.
/// THE GAP: this is a bounded source; behavior beyond the worker cliff is AC-2.
#[test]
fn deep_goal_native_build_uses_cli_worker() {
    build_and_run_deep_goal(160);
}

/// MEASURED: the CLI runs `help` under a 128-MiB virtual-address limit but
/// reports C0022 and exits 1 when that same limit prevents its 256-MiB worker
/// reservation. CLAIMED: worker-spawn failure is a coded CLI-front-end error.
/// THE GAP: this probes Linux RLIMIT_AS; other spawn failures share the same
/// `Err` arm but are not independently forced here.
#[cfg(target_os = "linux")]
#[test]
fn worker_spawn_failure_is_coded() {
    let (_dir, path) = fixture(0, false);
    let invoke_limited = |command| {
        Command::new("sh")
            .arg("-c")
            .arg("ulimit -c 0; ulimit -s 8192 || exit 77; ulimit -v 131072 || exit 78; exec \"$@\"")
            .arg("ken-native-stack-spawn-failure")
            .arg(env!("CARGO_BIN_EXE_ken"))
            .arg(command)
            .arg(&path)
            .env_remove("RUST_MIN_STACK")
            .output()
            .expect("spawn limited Ken CLI")
    };
    let help = invoke_limited("help");
    assert!(
        help.status.success(),
        "the limit must allow the CLI to start: status={:?} stderr={}",
        help.status,
        String::from_utf8_lossy(&help.stderr)
    );
    let check = invoke_limited("check");
    assert_eq!(check.status.code(), Some(1), "{check:?}");
    let stderr = String::from_utf8_lossy(&check.stderr);
    assert!(
        stderr.starts_with("error[C0022]: ken: cannot spawn elaboration worker: "),
        "worker-spawn failure must have its registered code: {stderr}"
    );
    assert_eq!(stderr.lines().count(), 1, "no uncoded panic: {stderr}");
}

fn nested_nat_source(depth: usize) -> String {
    let spine = (0..depth).fold(String::from("Zero"), |term, _| format!("Suc ({term})"));
    format!("const deep : Nat = {spine}\n")
}

fn nested_nat_fixture(depth: usize) -> (tempfile::TempDir, std::path::PathBuf, String) {
    let dir = tempfile::tempdir().expect("nested Nat fixture directory");
    let path = dir.path().join("deep_nat.ken");
    let source = nested_nat_source(depth);
    std::fs::write(&path, &source).expect("write nested Nat fixture");
    (dir, path, source)
}

fn check_then_format_nested_nat(depth: usize) {
    let (_dir, path, source) = nested_nat_fixture(depth);
    let checked = invoke("check", &path, &[]);
    assert!(
        checked.status.success(),
        "N={depth} ken check must accept the same source: status={:?} stderr={}",
        checked.status,
        String::from_utf8_lossy(&checked.stderr)
    );

    let formatted = invoke("fmt", Path::new("--check"), &[&path]);
    let stderr = String::from_utf8_lossy(&formatted.stderr);
    match formatted.status.code() {
        Some(0) => {}
        Some(1) => assert!(
            stderr.lines().any(|line| line.starts_with("error[C0016]:")),
            "N={depth} formatter refusal must be coded C0016: {stderr}"
        ),
        _ => panic!(
            "N={depth} fmt must return Ok or coded C0016, not an abort: status={:?} stderr={stderr}",
            formatted.status
        ),
    }
    assert_eq!(
        std::fs::read_to_string(&path).expect("re-read nested fixture"),
        source,
        "fmt --check must never rewrite the input"
    );
}

/// MEASURED: a shorter N=200 spine parses and formats under the same 8-MiB
/// main-thread limit. CLAIMED: the Ken fixture is well formed and the format
/// path can return a coded result even without the newly covered worker.
/// THE GAP: this control cannot detect the N=400 stack cliff.
#[test]
fn shallower_nested_nat_format_control() {
    check_then_format_nested_nat(200);
}

/// MEASURED: N=400 checks, then fmt --check returns Ok or C0016 on an
/// 8-MiB-main-thread child. CLAIMED: fmt's recursive parser now shares the
/// 256-MiB CLI worker rather than aborting at the old main-thread cliff.
/// THE GAP: the worker moves the cliff; AC-2 beyond it is operator-held.
#[test]
fn deep_nested_nat_format_uses_cli_worker() {
    check_then_format_nested_nat(400);
}

/// MEASURED: the same nesting form at N=4000 still checks successfully.
/// CLAIMED: moving fmt onto the worker leaves check's earlier successful
/// deeper row intact. THE GAP: this does not bound the format algorithm's
/// time or establish its behavior at N=4000.
#[test]
fn deeper_nested_nat_still_checks() {
    let (_dir, path, _) = nested_nat_fixture(4000);
    let checked = invoke("check", &path, &[]);
    assert!(
        checked.status.success(),
        "N=4000 ken check must still pass: status={:?} stderr={}",
        checked.status,
        String::from_utf8_lossy(&checked.stderr)
    );
}
