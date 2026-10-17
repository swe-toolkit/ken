//! CLI execution of an applied, ascribed λ after checked-core admission.
//! Promise class: durable execution observation. Spec 39 §5.4, 42 §3.2.

use std::process::Command;

const PROGRAM: &str = "program capabilities FS APartial\n\
    fn u1 (x : Nat) : Nat = ((λu. x) : (u : Nat) → Nat) Zero\n";

/// MEASURED: `ken run` checks the applied ascribed λ and drives opposite
/// branches for Zero and Suc Zero through the same u1 function. CLAIMED:
/// retaining the checked type does not disrupt reference execution. THE GAP:
/// this is the reference CLI, not native lowering or cross-target parity.
#[test]
fn applied_ascribed_lambda_runs_and_controls_output() {
    let fixture = tempfile::tempdir().expect("isolated Ken source");
    let path = fixture.path().join("ascription_inference.ken");
    for (arg, expected) in [
        ("Zero", b"wrong\n".as_slice()),
        ("(Suc Zero)", b"correct\n"),
    ] {
        let source = format!(
            "{PROGRAM}\
             proc main (_input : ProcessInput) (_caps : ProgramCaps APartial) : HostIO APartial ExitCode visits [Console] = host_program APartial (print_line (match u1 {arg} {{ Zero ↦ \"wrong\"; Suc n ↦ \"correct\" }}))\n"
        );
        std::fs::write(&path, source).expect("write Ken source");
        let output = Command::new(env!("CARGO_BIN_EXE_ken"))
            .arg("run")
            .arg(&path)
            .output()
            .expect("run Ken CLI");
        assert!(
            output.status.success(),
            "ken run on {arg}: status={:?} stderr={}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            output.stdout, expected,
            "u1 must supply the matched {arg} value"
        );
    }
}
