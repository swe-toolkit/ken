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

use ken_elaborator::{ElabEnv, ElabError};

const ACTION: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line, write)
import Capability.Process.Exit (host_bind, host_console, host_program, host_pure)
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
    let root = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("ken-closure-body-{}", std::process::id()));
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

/// The ACTION prefix is the real source each affected test checks. Its
/// standalone tails are not programs: `action` is declared by this prefix.
/// MEASURED: the composed source with persisted package imports reaches the
/// same typed Strict accept/refusal; CLAIMED: the former DIVERGE tail rows do
/// not mask the actual effect-row boundary. THE GAP: CLI execution remains
/// a separate check of the same source through its switched route.
fn recheck_composed_under_strict(source: &str) -> Option<Result<(), ElabError>> {
    if !source.starts_with(ACTION) {
        return None;
    }
    let catalog = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../catalog/packages")
        .canonicalize()
        .expect("catalog root");
    let mut env = ElabEnv::new().expect("prelude");
    Some(
        env.elaborate_isolated_file_strict_v1(&[catalog], source)
            .map(|_| ()),
    )
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
    if let Some(result) = recheck_composed_under_strict(source) {
        let error = result.expect_err("the composed source still refuses under Strict");
        assert_eq!(error.code().as_str(), "E0038", "{name}: {error:?}");
        let ElabError::TypeMismatch { reason, .. } = error else {
            panic!("{name}: wrong Strict refusal class: {error:?}");
        };
        assert!(
            reason.contains(&format!("EffectEscapes in '{owner}': {effect}")),
            "{name}: wrong Strict owner/effect: {reason}"
        );
    }
}

fn assert_accept(name: &str, source: &str) {
    let out = check(name, source);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{name}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    if let Some(result) = recheck_composed_under_strict(source) {
        result.unwrap_or_else(|error| panic!("{name}: Strict changed acceptance: {error:?}"));
    }
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
    if let Some(result) = recheck_composed_under_strict(source) {
        result.unwrap_or_else(|error| panic!("{name}: Strict changed acceptance: {error:?}"));
    }
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

// This guards propagation through bound_value_latent's RAsc arm. The
// ascription itself can infer, so this is not an independent pin on
// let_rhs_row's value short-circuit (the unannotated case above is).
#[test]
fn ascribed_let_lambda_releases_body_where_named() {
    let source = with_action(
        r#"proc run_with (u : Unit)
  : HostIO APartial ExitCode visits [Console] =
  let k : Unit -> HostIO APartial Unit =
    ((\x. action x) : Unit -> HostIO APartial Unit) in
  let ignored = k MkUnit in
    host_program APartial (print_line "b")
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  run_with MkUnit
"#,
    );
    assert_accept("ascribed-let-lambda-value", &source);
    assert_run("ascribed-let-lambda", &source, b"ab\n");
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
}

#[test]
fn alias_whole_result_type_releases_callback_row() {
    assert_escape(
        "alias-whole",
        r#"program capabilities FS APartial
def Runner = (Unit ->[Console] HostIO APartial Unit) -> HostIO APartial Unit
fn run_with : Runner = \f. f MkUnit
"#,
        "run_with",
        "Console",
    );
}

#[test]
fn pure_alias_headed_spine_remains_accepted() {
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
fn returned_closure_releases_its_body_row() {
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
}

#[test]
fn named_proc_return_is_still_charged_at_the_pure_boundary() {
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

// The original effect-composition program, before its declaration names the
// closure body's Console row. A tree-typed result is not an exemption.
const P8_TREE_VALUE: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line)
import Capability.Process.Exit (host_bind, host_console, host_pure)
import Core.Effect.Coproduct (inject_r)
const prog : ITree ConsoleOp console_resp (Pair Unit Nat) =
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

/// Promise class: durable invariant. MEASURED: the actual P8 source keeps
/// its pure-const E0038 owner/effect under Strict with only ephemeral H
/// imports, while its declared-row repair accepts in both modes and the
/// existing CLI run keeps its output. CLAIMED: the generic ITree floor
/// vocabulary does not make a charged closure body silently pure.
/// THE GAP: Strict execution bytes wait on 3b's rooted route switch.
#[test]
fn tree_typed_value_closure_requires_a_declared_row() {
    assert_escape("p8-const-tree-value", P8_TREE_VALUE, "prog", "Console");
    let imports = "import Capability.Console.Text (print_line)\n\
                   import Capability.Process.Exit (host_bind, host_pure, host_console)\n\
                   import Core.Effect.Coproduct (inject_r)\n";
    let with_imports = |source: &str| {
        source.replacen(
            "program capabilities FS APartial\n",
            &format!("program capabilities FS APartial\n{imports}"),
            1,
        )
    };
    let catalog = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../catalog/packages")
        .canonicalize()
        .expect("catalog root");
    let mut selected = ElabEnv::new().expect("prelude");
    let error = selected
        .elaborate_isolated_file_strict_v1(&[catalog.clone()], &with_imports(P8_TREE_VALUE))
        .expect_err("pure P8 tree still escapes its Console row");
    assert_eq!(error.code().as_str(), "E0038", "{error:?}");
    assert!(
        matches!(error, ElabError::TypeMismatch { reason, .. }
            if reason.contains("EffectEscapes in 'prog': Console")),
        "the same closure body must be charged"
    );
    let declared = P8_TREE_VALUE.replacen(
        "const prog : ITree ConsoleOp console_resp (Pair Unit Nat) =",
        "proc prog : ITree ConsoleOp console_resp (Pair Unit Nat) visits [Console] =",
        1,
    );
    assert_accept("p8-declared-tree-value", &declared);
    let mut repaired = ElabEnv::new().expect("prelude");
    repaired
        .elaborate_isolated_file_strict_v1(&[catalog], &with_imports(&declared))
        .expect("declared P8 effect row remains accepted under Strict");
    assert_run(
        "p8-declared-tree-value",
        &declared,
        b"state-console-pairing\n",
    );
}
