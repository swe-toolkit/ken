//! Deep CLI parsing and checking use a sized worker instead of the main stack.
//! Promise class: durable CLI acceptance, spec 33 §8.3, 39 §2.1 and 70 §1.
//! Unix re-exec reports overflow past the 256-MiB worker with C0023.

#[cfg(target_os = "linux")]
use std::os::unix::process::ExitStatusExt;
use std::path::Path;
use std::process::{Command, Output};

#[cfg(unix)]
const FAULT_MARKER: &str = "KEN_CLI_WORKER_CHILD_FAULT_FD";

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
fn ken_child_command() -> Command {
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
    ken.env_remove("RUST_MIN_STACK");
    #[cfg(unix)]
    ken.env_remove(FAULT_MARKER);
    ken
}

fn invoke(command: &str, path: &Path, additional_paths: &[&Path]) -> Output {
    ken_child_command()
        .arg(command)
        .arg(path)
        .args(additional_paths)
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
/// the worker; AC-2's production and lowered-stack rows test that boundary.
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
/// THE GAP: this does not cover programs beyond the 256-MiB worker cliff;
/// AC-2's separate refusal rows do.
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
/// possible larger proof goal; AC-2 pins past-worker exhaustion separately.
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
/// THE GAP: this is a bounded source; the separate AC-2 rows reach the cliff.
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
            .env_remove(FAULT_MARKER)
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
/// THE GAP: this stays below the cliff; AC-2's larger source tests it.
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

// The million-deep fixture must be built linearly, not by cloning each prefix.
// Its 6-MB source exceeds the measured production worker's parser stack.
#[cfg(unix)]
fn million_nested_nat() -> (tempfile::TempDir, std::path::PathBuf) {
    const DEPTH: usize = 1_000_000;
    let dir = tempfile::tempdir().expect("deep parser fixture directory");
    let path = dir.path().join("deep_nat.ken");
    let mut source = String::with_capacity(6 * DEPTH + 32);
    source.push_str("const deep : Nat = ");
    for _ in 0..DEPTH {
        source.push_str("Suc (");
    }
    source.push_str("Zero");
    for _ in 0..DEPTH {
        source.push(')');
    }
    source.push_str("\n\n"); // The blank line terminates the REPL declaration.
    std::fs::write(&path, source).expect("write deep parser fixture");
    (dir, path)
}

#[cfg(unix)]
fn assert_c0023(output: &Output, context: &str) {
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(1),
        "{context}: worker overflow must be coded, status={:?} stderr={stderr}",
        output.status
    );
    let lines: Vec<_> = stderr.lines().collect();
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.starts_with("error[C0023]: "))
            .count(),
        1,
        "{context}: exactly one C0023 diagnostic: {stderr}"
    );
    assert_eq!(
        lines.last().copied(),
        Some("error[C0023]: ken: the elaboration worker exhausted its native stack"),
        "{context}: coded refusal must be last: {stderr}"
    );
}

/// MEASURED: a million-deep source on check exits 1 with exactly one final
/// C0023 line and no stdout. CLAIMED: Unix parent reports parser overflow
/// past the production 256-MiB stack instead of aborting. THE GAP: other
/// parsing subcommands have their own rows; this says nothing about OOM.
#[cfg(unix)]
#[test]
fn million_nat_check_reports_worker_overflow() {
    let (_dir, path) = million_nested_nat();
    let output = invoke("check", &path, &[]);
    assert_c0023(&output, "N=1M check");
    assert!(output.stdout.is_empty(), "check must not write stdout");
}

/// MEASURED: the same parse overflow under run is coded and has no stdout.
/// CLAIMED: the process boundary encloses run before program execution.
/// THE GAP: this case never reaches interpretation; the K=160 run control does.
#[cfg(unix)]
#[test]
fn million_nat_run_reports_worker_overflow() {
    let (_dir, path) = million_nested_nat();
    let output = invoke("run", &path, &[]);
    assert_c0023(&output, "N=1M run");
    assert!(output.stdout.is_empty(), "run must not execute a program");
}

/// MEASURED: native-build reports one final C0023 rather than an abort.
/// CLAIMED: the child encloses native-build's source parser and worker.
/// THE GAP: successful artifact emission is pinned separately at K=160.
#[cfg(unix)]
#[test]
fn million_nat_native_build_reports_worker_overflow() {
    let (_dir, path) = million_nested_nat();
    let output_dir = tempfile::tempdir().expect("native output directory");
    let artifact_dir = output_dir.path().join("artifact");
    let profile = output_dir.path().join("profile.json");
    std::fs::write(&profile, NATIVE_RESOURCE_PROFILE).expect("write native profile");
    let output = invoke("native-build", &path, &[&artifact_dir, &profile]);
    assert_c0023(&output, "N=1M native-build");
    assert!(
        output.stdout.is_empty(),
        "no native artifact path on failure"
    );
}

/// MEASURED: fmt --check on a million-deep source gives one final C0023.
/// CLAIMED: the formatter's recursive parser runs inside the same child.
/// THE GAP: the N=400 C0016 control separately checks the non-overflow path.
#[cfg(unix)]
#[test]
fn million_nat_fmt_reports_worker_overflow() {
    let (_dir, path) = million_nested_nat();
    let output = invoke("fmt", Path::new("--check"), &[&path]);
    assert_c0023(&output, "N=1M fmt --check");
    assert!(
        output.stdout.is_empty(),
        "fmt --check must not write stdout"
    );
}

/// MEASURED: a million-deep declaration fed as REPL stdin yields one final
/// C0023. CLAIMED: the REPL parses inside the child, even though the REPL may
/// print its banner. THE GAP: the output check deliberately excludes stdout.
#[cfg(unix)]
#[test]
fn million_nat_repl_reports_worker_overflow() {
    let (_dir, path) = million_nested_nat();
    let stdin = std::fs::File::open(&path).expect("open REPL source");
    let output = ken_child_command()
        .arg("repl")
        .stdin(stdin)
        .output()
        .expect("run REPL with deep source on stdin");
    assert_c0023(&output, "N=1M repl");
}

#[cfg(all(unix, debug_assertions))]
fn lowered_stack(command: &str, path: &Path, extra: &[&Path], kib: usize) -> Output {
    ken_child_command()
        .arg(command)
        .arg(path)
        .args(extra)
        .env("KEN_CLI_DEBUG_WORKER_STACK_KIB", kib.to_string())
        .output()
        .expect("spawn Ken CLI with stated debug worker stack")
}

/// MEASURED: 8192-KiB debug worker refuses r1 K=160 for check, run, and
/// native-build with C0023. CLAIMED: the child sees the elaborator's
/// refresh_embedded_elim_evidence depth on all three paths. THE GAP: the
/// parser and kernel walker have separate lower-stack rows below.
#[cfg(all(unix, debug_assertions))]
#[test]
fn debug_eight_mib_r1_overflow_through_three_commands() {
    for command in ["check", "run", "native-build"] {
        let (_dir, path) = fixture(160, command == "run" || command == "native-build");
        let output_dir = tempfile::tempdir().expect("native output directory");
        let artifact_dir = output_dir.path().join("artifact");
        let profile = output_dir.path().join("profile.json");
        std::fs::write(&profile, NATIVE_RESOURCE_PROFILE).expect("write native profile");
        let output = if command == "native-build" {
            lowered_stack(command, &path, &[&artifact_dir, &profile], 8192)
        } else {
            lowered_stack(command, &path, &[], 8192)
        };
        assert_c0023(&output, &format!("8-MiB worker, K=160 {command}"));
        assert!(output.stdout.is_empty(), "{command}: no successful output");
    }
}

fn literal_cons_goal_source(depth: usize) -> String {
    let mut list = String::from("Nil Nat");
    for _ in 0..depth {
        list = format!("Cons Nat Zero ({list})");
    }
    let original = r1_program(0, false);
    let build_call = "build (Zero) (Nil Nat)";
    assert_eq!(original.matches(build_call).count(), 2, "both goal sides");
    original.replace(build_call, &list)
}

/// MEASURED: an 8192-KiB debug worker reports C0023 on a literal Cons
/// K=160 list. CLAIMED: the elaborator walker is reached by a different
/// deep source form than r1. THE GAP: sites are identified by the D0
/// backtrace; this status row alone does not identify the Rust stack frame.
#[cfg(all(unix, debug_assertions))]
#[test]
fn debug_eight_mib_literal_cons_overflows() {
    let dir = tempfile::tempdir().expect("literal list fixture directory");
    let path = dir.path().join("deep_list.ken");
    std::fs::write(&path, literal_cons_goal_source(160)).expect("write literal goal");
    let output = lowered_stack("check", &path, &[], 8192);
    assert_c0023(&output, "8-MiB worker, literal Cons K=160");
    assert!(output.stdout.is_empty());
}

/// MEASURED: a 4096-KiB debug worker reports C0023 on r1 K=160.
/// CLAIMED: the D0 kernel Term::clone walker also lies inside the child.
/// THE GAP: the exit code cannot itself name the overflow site; the D0
/// backtrace supplies the site, not this test.
#[cfg(all(unix, debug_assertions))]
#[test]
fn debug_four_mib_kernel_clone_overflows() {
    let (_dir, path) = fixture(160, false);
    let output = lowered_stack("check", &path, &[], 4096);
    assert_c0023(&output, "4-MiB worker, r1 K=160");
}

/// MEASURED: check and fmt --check both C0023 on N=600 with an 8192-KiB
/// debug worker. CLAIMED: both parser routes are contained by the child.
/// THE GAP: N=400 fmt C0016 ensures this is not a blanket refusal.
#[cfg(all(unix, debug_assertions))]
#[test]
fn debug_eight_mib_parser_overflows_for_check_and_fmt() {
    let (_dir, path, _) = nested_nat_fixture(600);
    for (command, first, extra) in [
        ("check", path.as_path(), Vec::new()),
        ("fmt", Path::new("--check"), vec![path.as_path()]),
    ] {
        let output = lowered_stack(command, first, &extra, 8192);
        assert_c0023(&output, &format!("8-MiB worker, N=600 {command}"));
    }
}

/// MEASURED: the huge debug-only stack request clamps to 256 MiB and r1
/// K=160 still checks. CLAIMED: the knob cannot enlarge the production
/// bound; the result alone cannot measure the actual allocation. THE GAP:
/// ordinary K=160 also checks, so a detector mutation should target a B row.
#[cfg(all(unix, debug_assertions))]
#[test]
fn debug_worker_stack_knob_clamps_without_refusing() {
    let (_dir, path) = fixture(160, false);
    let output = lowered_stack("check", &path, &[], 1_048_576);
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert!(output.stdout.is_empty());
}

#[cfg(target_os = "linux")]
fn externally_signal_worker(signal: &str) -> Output {
    use std::process::Stdio;
    use std::time::{Duration, Instant};

    let mut parent = ken_child_command()
        .arg("repl")
        .stdin(Stdio::piped()) // Hold stdin open until the child is signalled.
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("start held-open REPL parent");
    let parent_pid = parent.id();
    let children = format!("/proc/{parent_pid}/task/{parent_pid}/children");
    let deadline = Instant::now() + Duration::from_secs(15);
    let worker_pid = loop {
        if let Ok(text) = std::fs::read_to_string(&children) {
            if let Some(pid) = text.split_whitespace().next() {
                break pid.parse::<u32>().expect("child pid");
            }
        }
        assert!(
            Instant::now() < deadline,
            "REPL child never appeared for {parent_pid}"
        );
        std::thread::sleep(Duration::from_millis(10));
    };
    let killed = Command::new("kill")
        .arg(format!("-{signal}"))
        .arg(worker_pid.to_string())
        .status()
        .expect("signal the REPL child from outside");
    assert!(killed.success(), "kill -{signal} {worker_pid} failed");
    drop(parent.stdin.take());
    parent
        .wait_with_output()
        .expect("wait for REPL parent after child signal")
}

/// MEASURED: external SIGABRT of the child causes the parent itself to die
/// by signal 6, without C0023. CLAIMED: SIGABRT alone is not mislabelled as
/// a native-stack overflow. THE GAP: this does not cover allocation aborts.
#[cfg(target_os = "linux")]
#[test]
fn external_abort_does_not_claim_worker_stack_overflow() {
    let output = externally_signal_worker("ABRT");
    assert_eq!(output.status.signal(), Some(6), "{output:?}");
    assert!(
        !String::from_utf8_lossy(&output.stderr).contains("C0023"),
        "{output:?}"
    );
}

/// MEASURED: external SIGKILL of the child is re-raised by the parent as
/// signal 9. CLAIMED: the Unix child boundary preserves unclassified deaths.
/// THE GAP: the ABRT discriminator separately tests the fault-record gate.
#[cfg(target_os = "linux")]
#[test]
fn external_kill_propagates_signal() {
    let output = externally_signal_worker("KILL");
    assert_eq!(output.status.signal(), Some(9), "{output:?}");
}

/// MEASURED: a caller-set invalid marker cannot turn a million-deep check
/// into the old uncoded rc134 abort; it returns one final C0023, no stdout.
/// CLAIMED: the child role is not inferred from mere marker presence.
/// THE GAP: valid-but-foreign and parent-claimed markers need separate rows.
#[cfg(unix)]
#[test]
fn invalid_inherited_worker_marker_still_reports_overflow() {
    let (_dir, path) = million_nested_nat();
    let output = ken_child_command()
        .arg("check")
        .arg(&path)
        .env(FAULT_MARKER, "invalid")
        .output()
        .expect("check with inherited invalid marker");
    assert_c0023(&output, "N=1M check, invalid inherited marker");
    assert!(
        output.stdout.is_empty(),
        "invalid marker must not leak stdout"
    );
}

/// MEASURED: fd 1 with a foreign pid cannot claim the child role, returns
/// C0023 and never writes its fault byte to stdout. CLAIMED: the parent-pid
/// check precedes any recorder installation. THE GAP: a deliberate direct
/// parent that passes a matching own pipe can still forge the protocol.
#[cfg(unix)]
#[test]
fn foreign_parent_worker_marker_still_reports_overflow() {
    let (_dir, path) = million_nested_nat();
    let foreign_pid = if std::process::id() == 1 { 2 } else { 1 };
    let output = ken_child_command()
        .arg("check")
        .arg(&path)
        .env(FAULT_MARKER, format!("1:{foreign_pid}:0:0"))
        .output()
        .expect("check with foreign parent and fd 1");
    assert_c0023(&output, "N=1M check, foreign parent and fd 1");
    assert!(
        output.stdout.is_empty(),
        "fault byte must never reach stdout"
    );
}

/// MEASURED: a marker claiming the actual parent but an fd 1 pipe with the
/// wrong dev/ino fails before parsing with one C0022 and zero stdout bytes.
/// CLAIMED: a parent-claimed broken channel is rejected rather than running
/// unsupervised. THE GAP: no test can prove a deliberate full forgery absent.
#[cfg(unix)]
#[test]
fn parent_claimed_wrong_pipe_identity_is_coded() {
    let (_dir, path) = million_nested_nat();
    let marker = format!("1:{}:0:0", std::process::id());
    let output = ken_child_command()
        .arg("check")
        .arg(&path)
        .env(FAULT_MARKER, marker)
        .output()
        .expect("check with parent-claimed wrong pipe identity");
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    assert!(
        output.stdout.is_empty(),
        "protocol fault must not write stdout"
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stderr).as_ref(),
        "error[C0022]: ken: cannot spawn elaboration worker: the worker marker names no pipe from this process's parent\n",
        "protocol fault must be one registered diagnostic"
    );
}

/// MEASURED: N=400 still checks after an invalid inherited marker is
/// discarded. CLAIMED: malformed env cannot cause a blanket C0022 refusal;
/// normal shallow checks still run in the authenticated child. THE GAP:
/// this one source is not a substitute for the million-deep error row.
#[cfg(unix)]
#[test]
fn invalid_inherited_worker_marker_preserves_check_control() {
    let (_dir, path, _) = nested_nat_fixture(400);
    let output = ken_child_command()
        .arg("check")
        .arg(&path)
        .env(FAULT_MARKER, "invalid")
        .output()
        .expect("check bounded source with invalid inherited marker");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert!(output.stdout.is_empty(), "check must not run a program");
}
