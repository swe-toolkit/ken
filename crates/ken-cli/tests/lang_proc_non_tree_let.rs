//! Spec 36 §§2.4–2.5: a closed ambient-row proc returns a tree, so its
//! performing lets run in source order and its pure result is returned once.
//! Promise class: durable invariant. MEASURED: output `ab\n` from a performed
//! write and a spliced caller, two performed writes in order, and an E0038
//! result mismatch. CLAIMED: the action is neither dropped nor stored as an
//! ordinary value. THE GAP: FS, open rows and arrow-valued results keep the
//! narrowed refusal until their distinct denotations are settled.

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

fn assert_result_type_mismatch(name: &str, source: &str) {
    let output = cli(name, "check", source);
    assert_eq!(output.status.code(), Some(1), "{name}: {output:?}");
    assert!(output.stdout.is_empty(), "{name}: check cannot run actions");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("error[E0038]"), "{name}: {stderr}");
    assert!(
        stderr.contains("returned value does not inhabit the declared result"),
        "{name}: {stderr}"
    );
}

#[test]
fn performed_non_tree_proc_sequences_before_spliced_caller() {
    // AC-1 R1: the body write and the caller write execute once, in order.
    let source = "program capabilities FS APartial\n\
        proc read : Int visits [Console] =\n\
          let r = write ConsoleStream.Stdout (bytes_encode \"a\") in 1\n\
        proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
          : HostIO APartial ExitCode visits [Console] =\n\
          let n : Int = read in host_program APartial (print_line \"b\")\n";
    assert_checks("frame-probe", source);
    let run = cli("frame-probe-run", "run", source);
    assert_eq!(
        run.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(run.stdout, b"ab\n", "the returned tree splices into HostIO");
}

#[test]
fn two_performed_lets_run_once_each_in_order() {
    let source = "program capabilities FS APartial\n\
        proc pair : Int visits [Console] =\n\
          let r = write ConsoleStream.Stdout (bytes_encode \"a\") in\n\
          let s = write ConsoleStream.Stdout (bytes_encode \"b\") in 1\n\
        proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
          : HostIO APartial ExitCode visits [Console] =\n\
          let n : Int = pair in host_program APartial (print_line \"c\")\n";
    assert_checks("two-lets", source);
    let run = cli("two-lets-run", "run", source);
    assert_eq!(
        run.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(
        run.stdout, b"abc\n",
        "both lets execute once, left to right"
    );
}

#[test]
fn let_binds_the_result_not_a_held_action_value() {
    let source = with_main(
        "data Held = MkHeld (IO (Result IOError Unit))\n\
         proc held : Held visits [Console] =\n\
           let r = write ConsoleStream.Stdout (bytes_encode \"a\") in MkHeld r",
    );
    assert_result_type_mismatch("held-action-let", &source);
}

/// Promise class: durable fail-closed boundary (36 §§2.4–2.5). An
/// arrow-valued declared result is not wrapped by this increment. Its
/// outer performing let must refuse, while the same let *inside* the
/// returned action's lambda remains a valid checked tree.
#[test]
fn function_result_outer_let_refuses_but_inner_lambda_sequences() {
    let outer = with_main(
        "proc outer : Unit -> IO Unit visits [Console] =\n\
         let w = print_line \"x\" in\n\
         \\_. ITree.Ret ConsoleOp console_resp Unit MkUnit",
    );
    let output = cli("function-result-outer", "check", &outer);
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("error[E0038]"), "{stderr}");
    assert!(
        stderr.contains("whose declared result is a function"),
        "{stderr}"
    );
    assert!(stderr.contains("cannot be sequenced (36 §2.4)"), "{stderr}");

    let inner = with_main(
        "proc inner : Unit -> IO Unit visits [Console] =\n\
         \\_. let w = print_line \"x\" in ITree.Ret ConsoleOp console_resp Unit MkUnit",
    );
    assert_checks("function-result-inner", &inner);
}

#[test]
fn pure_non_tree_proc_returns_its_value_and_keeps_run_output() {
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
