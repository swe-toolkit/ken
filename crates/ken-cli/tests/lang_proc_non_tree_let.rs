//! Spec 36 §2.4: an action in a non-tree-result let cannot be dropped.
//! Promise class: transition sentinel. Retire the check-time refusal when
//! §2.5's non-empty-row proc result becomes an ITree and its callers splice
//! that tree; until then the elaboration gap must refuse, not erase, an action.
//! MEASURED: check-time E0038, no run output for action-valued RHSs, and
//! accepted type/row/proof controls. CLAIMED: a performing ordinary let cannot
//! silently discard an action. THE GAP: other, non-let binders and the infer
//! guard's unreachable arm belong to the later §2.5 conformance work.

use std::process::{Command, Output};

fn cli(name: &str, subcommand: &str, source: &str) -> Output {
    let root = std::env::temp_dir().join(format!("ken-proc-nontree-let-{}", std::process::id()));
    std::fs::create_dir_all(&root).expect("create fixture root");
    let path = root.join(format!("{name}.ken"));
    std::fs::write(&path, source).expect("write inline source");
    Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg(subcommand)
        .arg(path)
        .output()
        .expect("run Ken CLI")
}

fn with_main(declarations: &str) -> String {
    format!(
        "program capabilities FS APartial\n{declarations}\n\
         proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
           : HostIO APartial ExitCode visits [Console] =\n\
           host_program APartial (print_line \"b\")\n"
    )
}

fn assert_checks(name: &str, source: &str) {
    let output = cli(name, "check", source);
    assert_eq!(
        output.status.code(),
        Some(0),
        "{name}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_gap_refusal(name: &str, source: &str) {
    let output = cli(name, "check", source);
    assert_eq!(
        output.status.code(),
        Some(1),
        "{name}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stdout.is_empty(),
        "{name}: check must not run actions"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("error[E0038]"), "{name}: {stderr}");
    assert!(
        stderr.contains("cannot be sequenced (36 §2.4)"),
        "{name}: {stderr}"
    );
    assert!(stderr.contains("§2.5"), "{name}: {stderr}");
}

#[test]
fn unused_performed_let_is_refused_before_running() {
    // AC-1 probe verbatim; on the fcc816c65 base it checked and printed b\n.
    let source = "program capabilities FS APartial\n\
        proc read : Int visits [Console] =\n\
          let r = write ConsoleStream.Stdout (bytes_encode \"a\") in 1\n\
        proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
          : HostIO APartial ExitCode visits [Console] =\n\
          let n : Int = read in host_program APartial (print_line \"b\")\n";
    assert_gap_refusal("frame-probe", source);
    let run = cli("frame-probe-run", "run", source);
    assert_ne!(run.status.code(), Some(0));
    assert!(
        run.stdout.is_empty(),
        "refused program must not run any action"
    );
}

#[test]
fn two_unconsumed_actions_are_refused_at_the_first_let() {
    let source = with_main(
        "proc pair : Int visits [Console] =\n\
           let r = write ConsoleStream.Stdout (bytes_encode \"a\") in\n\
           let s = write ConsoleStream.Stdout (bytes_encode \"a\") in 1",
    );
    assert_gap_refusal("two-lets", &source);
}

#[test]
fn let_binds_the_result_not_a_held_action_value() {
    let source = with_main(
        "data Held = MkHeld (IO (Result IOError Unit))\n\
         proc held : Held visits [Console] =\n\
           let r = write ConsoleStream.Stdout (bytes_encode \"a\") in MkHeld r",
    );
    assert_gap_refusal("held-action-let", &source);
}

#[test]
fn pure_non_tree_proc_keeps_its_value_and_run_output() {
    let source = with_main("proc direct_value : Int visits [Console] = 1");
    assert_checks("direct-value", &source);
    let run = cli("direct-value-run", "run", &source);
    assert_eq!(
        run.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(run.stdout, b"b\n");
}

#[test]
fn unapplied_effectful_function_is_a_value_not_an_action_tree() {
    assert_checks(
        "unapplied-function",
        &with_main("proc g : Int visits [Console] = let w = write in 1"),
    );
}

#[test]
fn constructor_holds_an_action_directly_without_performing_it() {
    let source = with_main(
        "data Held = MkHeld (IO (Result IOError Unit))\n\
         proc held_direct : Held visits [Console] =\n\
           MkHeld (write ConsoleStream.Stdout (bytes_encode \"a\"))",
    );
    assert_checks("direct-held", &source);
}

#[test]
fn tree_typed_rhs_with_empty_row_is_a_value() {
    let source = with_main(
        "proc keep (t : IO (Result IOError Unit)) : Int visits [Console] = let u = t in 1",
    );
    assert_checks("tree-value-no-row", &source);
}

#[test]
fn tree_result_sequences_and_binds_the_action_result() {
    let source = with_main(
        "data Res = MkRes (Result IOError Unit)\n\
         proc res_io : IO (Result IOError Unit) visits [Console] =\n\
           let r = write ConsoleStream.Stdout (bytes_encode \"a\") in\n\
           let h : Res = MkRes r in write ConsoleStream.Stdout (bytes_encode \"a\")",
    );
    assert_checks("tree-result", &source);
}

#[test]
fn proof_position_let_is_a_value_and_does_not_run() {
    // The full source also occurs in verify_space_obligation_hole_registration.
    let source = r#"
fn d (x : Int) (y : Int) : Int requires Not (Equal Int x y) = x
const k : Int = d 0 0
space S {
  mut cell : Int = 0
  proc call () : Int visits [S] = d 0 0
}
theorem ts1 : Top = let u = S.call in Proved
space T {
  mut cell : Int = d 0 0
}
"#;
    assert_checks("omega-proof-let", source);
}
