//! `effect-composition` D5 §3 leg (2) — the SECOND distinct `(g,h)` pairing
//! through the general `resp_coproduct` (AC3's generality face), driving the real
//! `ken` CLI binary (subprocess) against a real elaborated program. No test
//! in this file hand-constructs a `Coproduct`/`InL`/`InR` value.
//!
//! Distinct from the {FS,Console} terminal-driver peel (`run_io`'s D3
//! mechanism, `fs_read_file_lines_flip_e2e.rs`): this exercises the
//! **pure-handler** role instead — `run_state` (a kernel-re-checked
//! `declare_def` fold, COEXIST-preserved, BV4) peels `StateOp Nat` out of
//! `Coproduct (StateOp Nat) ConsoleOp` via the now-general `resp_coproduct`, threads the
//! state, and re-emits the untouched `ConsoleOp` residual tree, which
//! `run_io` then runs directly (no `Coproduct` wrapper left to peel). Two
//! genuinely different coproduct-execution mechanisms, both resting on the
//! same general `resp_coproduct` (D1) — the executable ≥2-distinct-pairings
//! discriminator (`effect-composition-conformance.md` §3 leg (2)).

use std::path::PathBuf;
use std::process::Command;

use ken_elaborator::{ElabEnv, ElabError};

fn ken_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_ken"))
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// `state_get` (State) sequenced with `inject_r (print_line ...)` (Console) under
/// `Coproduct (StateOp Nat) ConsoleOp`, run through `run_state` then `run_io`.
/// `Pair Unit Nat` is a surface-nameable alias for `run_state`'s real return
/// type `Sigma Unit Nat` (`prelude.rs`'s `Pair := \a b. Sigma a b`).
/// `prog` declares `visits [Console]`: its bind continuation names
/// `print_line`, and a built closure pays its body's row where it is built.
const PROG: &str = r#"program capabilities FS APartial
proc prog : ITree ConsoleOp console_resp (Pair Unit Nat) visits [Console] =
  run_state Nat ConsoleOp console_resp Unit Zero
    (itree_bind (Coproduct (StateOp Nat) ConsoleOp) (resp_coproduct (StateOp Nat) ConsoleOp (resp_state Nat) console_resp) Nat Unit
      (state_get Nat ConsoleOp console_resp MkUnit)
      (\n . inject_r (StateOp Nat) ConsoleOp (resp_state Nat) console_resp Unit (print_line "state-console-pairing")))

proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_bind APartial (Pair Unit Nat) ExitCode
    (host_console APartial (Pair Unit Nat) prog)
    (\_ . host_pure APartial ExitCode ExitCode.Success)
"#;

/// Promise class: durable invariant. MEASURED: this same State-Console
/// program checks through the strict entry with explicit package imports,
/// and refuses the inclusion without its home import. CLAIMED: the specific
/// ITree binder and general inclusion preserve the checked composition.
/// THE GAP: the unchanged CLI run below independently observes the one
/// Console effect, rather than inferring execution from elaboration.
#[test]
fn state_console_source_rechecks_under_strict_without_internal_effect_names() {
    let root = workspace_root()
        .join("catalog/packages")
        .canonicalize()
        .expect("catalog root");
    let body = PROG
        .strip_prefix("program capabilities FS APartial\n")
        .expect("source program header");
    let other_imports = "import Capability.Console.Text (print_line)\n\
                         import Capability.Process.Exit (host_console, host_bind, host_pure)\n";
    let complete = format!(
        "program capabilities FS APartial\n{other_imports}\
         import Core.Effect.Coproduct (inject_r)\n{body}"
    );
    let mut strict = ElabEnv::new().expect("prelude");
    strict
        .elaborate_isolated_file_strict_v1(&[root.clone()], &complete)
        .expect("State-Console source with checked effect imports");
    let without_inject = format!("program capabilities FS APartial\n{other_imports}{body}");
    let mut negative = ElabEnv::new().expect("prelude");
    match negative.elaborate_isolated_file_strict_v1(&[root], &without_inject) {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "inject_r"),
        Err(error) => panic!("wrong no-import refusal: {error:?}"),
        Ok(_) => panic!("generic inclusion available without its package import"),
    }
}

#[test]
fn state_console_pairing_runs_through_run_state_then_run_io() {
    let tmp_dir = std::env::temp_dir().join("ken-effect-composition-e2e");
    std::fs::create_dir_all(&tmp_dir).expect("create tmp dir");
    let path = tmp_dir.join("state_console.ken");
    std::fs::write(&path, PROG).expect("write program");

    let output = Command::new(ken_bin())
        .arg("run")
        .arg(&path)
        .current_dir(workspace_root())
        .output()
        .unwrap_or_else(|e| panic!("failed to spawn `ken run`: {e}"));

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(output.status.success(), "must succeed; stderr: {stderr}");
    assert_eq!(
        stdout, "state-console-pairing\n",
        "the Console residual must print exactly once"
    );
}
