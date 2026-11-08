//! Spec 36 §§2.4–2.5: only a changed proc's own result position may
//! introduce a returned tree. Promise class: durable invariant.
//! MEASURED: inferred let RHSs, conditions, and scrutinees in changed procs
//! refuse with their pure const controls' diagnostic, including a formerly
//! executable branch. CLAIMED: entering a subterm through inference cannot
//! give its plain branch a tree-result Ret coercion. THE GAP: the checked
//! entry and the other infer callers are covered by the existing RTM suites
//! and the Architect's full fan-in census (AC-0).

use std::process::{Command, Output};

use ken_elaborator::ElabEnv;

fn cli(name: &str, command: &str, source: &str) -> Output {
    let root = std::env::temp_dir().join(format!(
        "ken-proc-returned-tree-infer-entry-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&root).expect("create fixture root");
    let path = root.join(format!("{name}.ken"));
    std::fs::write(&path, source).expect("write fixture");
    Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg(command)
        .arg(path)
        .output()
        .expect("run Ken CLI")
}

fn source(declarations: &str) -> String {
    format!(
        "program capabilities FS APartial\n{declarations}\n\
         proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
           : HostIO APartial ExitCode visits [Console] =\n\
           host_program APartial (print_line \"done\")\n"
    )
}

fn assert_refusal(name: &str, command: &str, program: &str, code: &str) {
    let output = cli(name, command, program);
    assert_eq!(output.status.code(), Some(1), "{name}: {output:?}");
    assert!(output.stdout.is_empty(), "{name}: ran an action");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(&format!("error[{code}]")),
        "{name}: {stderr}"
    );
}

/// The typed Strict verdict must repeat the existing CLI oracle's kind.
/// MEASURED: the same generated source with only ephemeral H imports reaches
/// the stated K0004/E0013 arm; CLAIMED: ConsoleOp's scoped floor entry
/// preserves each pure-position control; THE GAP: the CLI route stays Legacy
/// until 3b and is measured by the unchanged `assert_refusal` calls.
fn assert_strict_refusal(program: &str, expected: &str) {
    let catalog = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../catalog/packages")
        .canonicalize()
        .expect("catalog root");
    let imported = program.replacen(
        "program capabilities FS APartial\n",
        "program capabilities FS APartial\n\
         import Capability.Console.Text (IO, print_line)\n\
         import Capability.Process.Exit (host_program)\n",
        1,
    );
    let mut env = ElabEnv::new().expect("prelude");
    let error = env
        .elaborate_isolated_file_strict_v1(&[catalog], &imported)
        .expect_err("pure-position Console tree must still refuse");
    assert_eq!(error.code().as_str(), expected, "{error:?}");
}

fn assert_inferred_subterm(case: usize) {
    const MATCH: &str =
        "match True { True ↦ (ITree.Ret ConsoleOp console_resp Int 3 : IO Int); False ↦ 4 }";
    let cases = [
        ("let-rhs", format!("let r = {MATCH} in 1"), "K0004"),
        (
            "nested-let-rhs",
            format!("let r = (let u = MkUnit in {MATCH}) in 1"),
            "K0004",
        ),
        (
            "scrutinee",
            format!("match (let r = {MATCH} in True) {{ True ↦ 1; False ↦ 2 }}"),
            "K0004",
        ),
        (
            "condition",
            format!("if (let r = {MATCH} in True) then 1 else 2"),
            "E0013",
        ),
    ];
    let (name, body, code) = &cases[case];
    let changed = source(&format!("proc p : Int visits [Console] = {body}"));
    let control = source(&format!("const c : Int = {body}"));
    assert_refusal(&format!("proc-{name}"), "check", &changed, code);
    assert_refusal(&format!("const-{name}"), "check", &control, code);
    assert_strict_refusal(&changed, code);
    assert_strict_refusal(&control, code);
}

#[test]
fn unannotated_let_rhs_does_not_inherit_returned_tree_mode() {
    assert_inferred_subterm(0);
}

#[test]
fn nested_unannotated_let_rhs_does_not_inherit_returned_tree_mode() {
    assert_inferred_subterm(1);
}

#[test]
fn match_scrutinee_does_not_inherit_returned_tree_mode() {
    assert_inferred_subterm(2);
}

#[test]
fn if_condition_does_not_inherit_returned_tree_mode() {
    assert_inferred_subterm(3);
}

fn run_form(choice: &str) -> String {
    format!(
        "program capabilities FS APartial\n\
         proc p (b : Bool) : String visits [Console] =\n\
           let r = match b {{ True ↦ (print_line \"t\" : IO Unit); \
           False ↦ MkUnit }} in \"done\"\n\
         proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
           : HostIO APartial ExitCode visits [Console] =\n\
           let s : String = p {choice} in host_program APartial (print_line s)\n"
    )
}

#[test]
fn pure_ret_arm_controls_run_form_type_mismatch() {
    // The pure const controls the same arm-type mismatch using a Ret; the
    // literal print_line action would instead fail its purity gate (E0038)
    // before the mismatched arm is checked, so it cannot control K0004.
    let control = source(
        "const c : Bool -> String = \\b. let r = match b { True ↦ \
         (ITree.Ret ConsoleOp console_resp Unit MkUnit : IO Unit); \
         False ↦ MkUnit } in \"done\"",
    );
    assert_refusal("const-run-control", "check", &control, "K0004");
    assert_strict_refusal(&control, "K0004");
}

#[test]
fn inferred_match_in_changed_proc_refuses_plain_arm() {
    assert_refusal("proc-run-check", "check", &run_form("True"), "K0004");
}

#[test]
fn inferred_match_true_arm_cannot_execute_after_plain_arm_coercion() {
    assert_refusal("proc-run-true", "run", &run_form("True"), "K0004");
}

#[test]
fn inferred_match_false_arm_cannot_execute_after_plain_arm_coercion() {
    assert_refusal("proc-run-false", "run", &run_form("False"), "K0004");
}
