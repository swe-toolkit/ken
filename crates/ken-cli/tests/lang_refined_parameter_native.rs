//! Reference execution of a kernel-checked refined-parameter recursive worker
//! behind its public wrapper (spec 17 §4, spec 39 §5.4, spec 42 §3.2).
//! Native execution is not delivered: calls carrying erased Ω arguments also
//! fail on the pre-existing native `requires` recursion path at base 1d82fd8ec.

use std::process::Command;

const SOURCE: &str = r#"program capabilities FS APartial
def AnyNat = { x : Nat | Top }
fn classify (n : Nat) : ExitCode = match n {
  Zero |-> ExitCode.Failure 100;
  Suc a |-> match a {
    Zero |-> ExitCode.Failure 1;
    Suc b |-> match b {
      Zero |-> ExitCode.Failure 2;
      Suc c |-> match c {
        Zero |-> ExitCode.Failure 3;
        Suc d |-> match d {
          Zero |-> ExitCode.Failure 4;
          Suc _ |-> ExitCode.Failure 200
        }
      }
    }
  }
}
fn refined_step (n : AnyNat) : Nat = match n { Zero |-> Zero ; Suc m |-> Suc (Suc (refined_step m)) }
fn main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode =
  host_exit APartial (classify (refined_step (Suc (Suc Zero))))
"#;

/// Promise class: durable reference-execution observation. MEASURED: `ken run`
/// checks the refined recursive program and exits 4 on two successors. CLAIMED:
/// the public wrapper reaches the private worker's recursive call; replacing
/// that call with the nonrecursive `Suc (Suc m)` instead exits 3. THE GAP:
/// this observes reference execution only, not native support or all inputs.
#[test]
fn refined_parameter_worker_runs_through_public_wrapper() {
    let directory = tempfile::tempdir().expect("isolated Ken source");
    let source = directory.path().join("refined_recursion.ken");
    std::fs::write(&source, SOURCE).expect("write checked source");
    let run = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("run")
        .arg(source)
        .output()
        .expect("run Ken source");
    assert_eq!(
        run.status.code(),
        Some(4),
        "checked refined recursion must yield Failure 4: stderr={}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(run.stdout.is_empty(), "exit-code fixture has no output");
}
