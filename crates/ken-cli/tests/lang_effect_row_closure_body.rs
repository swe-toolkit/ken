//! Spec 36 §1.2–1.4 and Architect CLOSURE-BODY AC-0: a closure's body
//! contributes to the shared row plane at construction, except when the
//! closure is the whole RHS of a let and its row is paid through its binder.
//! Promise class: durable invariant. MEASURED: `ken check` names the escaping
//! declaration and `ken run` observes the order of the two performed actions.
//! CLAIMED: no undeclared closure or alias-headed callback escapes, and
//! let-bound closures do not silently drop actions. THE GAP: these concrete
//! forms do not prove every higher-order use; the pay-at-build rule is an
//! intentionally conservative bound when a callee does not apply its argument.

use std::process::{Command, Output};

const ACTION: &str = r#"program capabilities FS APartial
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
"#;

fn with_action(tail: &str) -> String {
    format!("{ACTION}{tail}")
}

fn cli(name: &str, verb: &str, source: &str) -> Output {
    let root = std::env::temp_dir().join(format!("ken-closure-body-{}", std::process::id()));
    std::fs::create_dir_all(&root).expect("fixture directory");
    let path = root.join(format!("{name}.ken"));
    std::fs::write(&path, source).expect("write checked program");
    Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg(verb)
        .arg(&path)
        .output()
        .expect("run Ken CLI")
}

fn check(name: &str, source: &str) -> Output {
    cli(name, "check", source)
}

fn assert_escape(name: &str, source: &str, owner: &str, effect: &str) {
    let out = check(name, source);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(1), "{name}: {stderr}");
    assert!(out.stdout.is_empty(), "{name}: refusal emits no output");
    assert!(stderr.contains("error[E0038]"), "{name}: {stderr}");
    assert!(
        stderr.contains(&format!("EffectEscapes in '{owner}': {effect}")),
        "{name}: wrong refusal owner or effect: {stderr}"
    );
}

fn assert_accept(name: &str, source: &str) {
    let out = check(name, source);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{name}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

fn assert_run(name: &str, source: &str, output: &[u8]) {
    let out = cli(name, "run", source);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{name}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(out.stdout, output, "{name}: wrong action order or output");
}

const P5: &str = r#"fn sneaky (u : Unit) : HostIO APartial Unit =
  host_bind APartial Unit Unit (host_pure APartial Unit MkUnit) (\_. action MkUnit)
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [FS] =
  host_bind APartial Unit ExitCode (sneaky MkUnit)
    (\_. host_pure APartial ExitCode ExitCode.Success)
"#;

#[test]
fn p5_closure_body_cannot_escape_pure_sneaky() {
    assert_escape("p5", &with_action(P5), "sneaky", "Console");
    let declared = with_action(
        r#"proc sneaky (u : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial Unit Unit (host_pure APartial Unit MkUnit)
    (\_. action MkUnit)
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_bind APartial Unit ExitCode (sneaky MkUnit)
    (\_. host_pure APartial ExitCode ExitCode.Success)
"#,
    );
    assert_accept("p5-declared", &declared);
    assert_run("p5-declared", &declared, b"a");
    let pure = with_action(
        r#"fn quiet (u : Unit) : HostIO APartial Unit =
  host_bind APartial Unit Unit (host_pure APartial Unit MkUnit)
    (\_. host_pure APartial Unit MkUnit)
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_bind APartial Unit ExitCode (quiet MkUnit)
    (\_. host_program APartial (print_line "b"))
"#,
    );
    assert_accept("p5-pure", &pure);
    assert_run("p5-pure", &pure, b"b\n");
}

#[test]
fn nonleading_let_bound_closure_cannot_escape_pure_sneaky2() {
    assert_escape(
        "nonleading",
        &with_action(
            r#"fn sneaky2 (u : Unit) : HostIO APartial Unit =
  let k : Unit -> HostIO APartial Unit = \x. action x in
  host_bind APartial Unit Unit (host_pure APartial Unit MkUnit) k
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_bind APartial Unit ExitCode (sneaky2 MkUnit)
    (\_. host_program APartial (print_line "b"))
"#,
        ),
        "sneaky2",
        "Console",
    );
}

const LET_LAMBDA: &str = r#"proc run_with (u : Unit)
  : HostIO APartial ExitCode visits [Console] =
  let k : Unit -> HostIO APartial Unit = \x. action x in
  let ignored = k MkUnit in
    host_program APartial (print_line "b")
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  run_with MkUnit
"#;

#[test]
fn named_let_bound_closure_performs_before_continuation() {
    assert_run("let-lambda", &with_action(LET_LAMBDA), b"ab\n");
}

#[test]
fn inline_let_bound_continuation_performs_before_continuation() {
    assert_run(
        "let-bind-lambda",
        &with_action(
            r#"proc run_with (u : Unit)
  : HostIO APartial ExitCode visits [Console] =
  let ignored = host_bind APartial Unit Unit (host_pure APartial Unit MkUnit)
    (\_. action MkUnit) in
    host_program APartial (print_line "b")
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  run_with MkUnit
"#,
        ),
        b"ab\n",
    );
}

#[test]
fn a_let_lambda_is_still_a_value_not_a_performed_rhs() {
    assert_accept("let-lambda-value", &with_action(LET_LAMBDA));
}

#[test]
fn alias_headed_result_spine_releases_callback_row() {
    assert_escape(
        "alias-spine",
        &with_action(
            r#"def Runner = (Unit ->[Console] HostIO APartial Unit) -> HostIO APartial Unit
fn run_with (u : Unit) : Runner = \f. f MkUnit
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_bind APartial Unit ExitCode (run_with MkUnit action)
    (\_. host_program APartial (print_line "b"))
"#,
        ),
        "run_with",
        "Console",
    );
    assert_escape(
        "alias-whole",
        r#"program capabilities FS APartial
def Runner = (Unit ->[Console] HostIO APartial Unit) -> HostIO APartial Unit
fn run_with : Runner = \f. f MkUnit
"#,
        "run_with",
        "Console",
    );
    assert_accept(
        "alias-pure",
        &with_action(
            r#"def Runner0 = (Unit -> HostIO APartial Unit) -> HostIO APartial Unit
fn run_with (u : Unit) : Runner0 = \f. f MkUnit
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_program APartial (print_line "b")
"#,
        ),
    );
}

#[test]
fn returned_closure_and_named_proc_share_pay_at_build_policy() {
    assert_escape(
        "returned-closure",
        &with_action(
            r#"fn mk (u : Unit) : Unit ->[Console] HostIO APartial Unit =
  let h : Unit -> HostIO APartial Unit = \x. action x in h
"#,
        ),
        "mk",
        "Console",
    );
    assert_escape(
        "returned-global",
        &with_action(
            r#"fn mk2 (u : Unit) : Unit ->[Console] HostIO APartial Unit = action
"#,
        ),
        "mk2",
        "Console",
    );
}
