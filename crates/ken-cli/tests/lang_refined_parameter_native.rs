//! Reference/native consumer of a kernel-checked carrier worker behind a
//! public refined-parameter wrapper (spec 17 §4, spec 39 §5.4).

use std::process::Command;

const SOURCE: &str = "program capabilities FS APartial\n\
    def AnyNat = { x : Nat | Top }\n\
    fn refined_step (n : AnyNat) : Nat =
    match n { Zero |-> Zero ; Suc m |-> Suc (refined_step m) }\n\
    proc main (_input : ProcessInput) (_caps : ProgramCaps APartial) : HostIO APartial ExitCode visits [Console] =
    host_program APartial (print_line (match refined_step (Suc Zero) { Zero |-> \"wrong\"; Suc m |-> \"correct\" }))\n";

const NATIVE_RESOURCE_PROFILE: &str = "{\"runtime\":{\"invocation_epochs\":1024},\"call_events\":{\"event_generations\":2048,\"live_pending_slots\":1024},\"invocation\":{\"nodes\":100000,\"words\":100000,\"data_bytes\":1000000,\"native_int_limbs\":100000},\"persistent\":{\"nodes\":100000,\"words\":100000,\"data_bytes\":1000000,\"native_int_limbs\":100000}}";

fn fixture() -> (tempfile::TempDir, std::path::PathBuf) {
    let directory = tempfile::tempdir().expect("fixture directory");
    let source = directory.path().join("refined_recursion.ken");
    std::fs::write(&source, SOURCE).expect("write checked source");
    (directory, source)
}

/// Promise class: durable execution observation. MEASURED: a source-level
/// recursive call traverses the private worker, and the checked public call
/// produces the branch selected by Suc Zero. CLAIMED: the wrapper remains an
/// ordinary callable source identity after the internal SCT transform.
/// THE GAP: this tests reference execution, not native symbol closure.
#[test]
fn refined_parameter_worker_runs_through_public_wrapper() {
    let (_dir, source) = fixture();
    let run = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("run")
        .arg(source)
        .output()
        .expect("run Ken source");
    assert!(
        run.status.success(),
        "ken run failed: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(run.stdout, b"correct\n");
}

/// Promise class: durable native/reference parity. MEASURED: the same checked
/// source builds and runs natively, producing the selected Suc branch. CLAIMED:
/// its worker has a stable native symbol and recursion metadata recognized by
/// the native barrier path. THE GAP: this is one bounded source/host, not all
/// native targets or an arbitrary-depth performance claim.
#[test]
fn refined_parameter_worker_builds_and_runs_natively() {
    let (_dir, source) = fixture();
    let output = tempfile::tempdir().expect("native artifact directory");
    let profile = output.path().join("profile.json");
    let artifact_dir = output.path().join("artifact");
    std::fs::write(&profile, NATIVE_RESOURCE_PROFILE).expect("native resource profile");
    let built = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("native-build")
        .arg(source)
        .arg(&artifact_dir)
        .arg(&profile)
        .output()
        .expect("build native Ken program");
    assert!(
        built.status.success(),
        "native-build failed: {}",
        String::from_utf8_lossy(&built.stderr)
    );
    let executable = String::from_utf8(built.stdout).expect("UTF-8 executable path");
    let run = Command::new(executable.trim())
        .output()
        .expect("run native artifact");
    assert!(
        run.status.success(),
        "native program failed: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(run.stdout, b"correct\n");
}
