//! Spec 36 §1.1–1.4 and Architect AC-0b: a written local callback row is
//! released by the shared row plane and checked at a declared-purity boundary.
//! Promise class: durable invariant. MEASURED: the production CLI checks
//! separate programs and attributes each refusal to its named declaration.
//! CLAIMED: no undeclared written callback row escapes through a local binder.
//! THE GAP: the current surface row walk does not infer rows not written
//! anywhere (nor the body of a built lambda, owned by CLOSURE-BODY).

use std::process::{Command, Output};

const CALLBACK: &str =
    include_str!("fixtures/lang_effect_row_local_latent/fn_calls_console_cb.ken");
const UNDERDECLARED: &str =
    include_str!("fixtures/lang_effect_row_local_latent/proc_cb_undeclared.ken");
const P6: &str = include_str!("fixtures/lang_effect_row_local_latent/p6_global_result_arrow.ken");
const P6B: &str =
    include_str!("fixtures/lang_effect_row_local_latent/p6b_forward_to_pure_result.ken");
const P8: &str = include_str!("fixtures/lang_effect_row_local_latent/p8_subsumption.ken");
const QA_LET: &str = include_str!("fixtures/lang_effect_row_local_latent/qa_higher_order_let.ken");

fn cli(name: &str, verb: &str, source: &str) -> Output {
    let root = std::env::temp_dir().join(format!("ken-local-latent-{}", std::process::id()));
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

#[test]
fn fn_callback_refuses_console() {
    assert_escape("callback", CALLBACK, "run_with", "Console");
}

#[test]
fn underdeclared_proc_refuses_console() {
    assert_escape("underdeclared", UNDERDECLARED, "run_with", "Console");
}

#[test]
fn p6_returning_a_written_callback_refuses_at_mk() {
    assert_escape("p6", P6, "mk", "Console");
}

#[test]
fn p6b_passing_a_written_callback_through_a_pure_result_refuses_at_go() {
    assert_escape("p6b", P6B, "go", "Console");
}

#[test]
fn p8_passing_a_written_callback_through_pure_parameter_refuses_at_go3() {
    assert_escape("p8", P8, "go3", "Console");
}

#[test]
fn pure_callback_and_shadowed_pure_let_remain_accepted() {
    // A pure arrow never releases Console just because it returns HostIO.
    assert_accept(
        "pure-forward",
        r#"fn run_pure (f : Unit -> HostIO APartial Unit)
  : HostIO APartial Unit = f MkUnit
fn go4 (g : Unit -> HostIO APartial Unit)
  : HostIO APartial Unit = run_pure g
"#,
    );
    // The inner f has a written pure arrow and shadows the Console parameter.
    assert_accept(
        "shadowed-let",
        r#"fn shadow
  (f : Unit ->[Console] HostIO APartial Unit) : HostIO APartial Unit =
  let f : Unit -> HostIO APartial Unit =
    \x. host_pure APartial Unit MkUnit in f MkUnit
"#,
    );
}

#[test]
fn an_unbound_written_row_variable_is_not_the_empty_row() {
    assert_escape(
        "row-tail",
        r#"fn run_poly (f : Unit ->[e] HostIO APartial Unit)
  : HostIO APartial Unit = f MkUnit
"#,
        "run_poly",
        "unknown latent row variable",
    );
}

#[test]
fn a_constructor_field_carries_its_callback_row_to_the_match() {
    assert_escape(
        "box-field",
        r#"data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
fn open (b : Box) : HostIO APartial Unit =
  match b { MkBox g ↦ g MkUnit }
"#,
        "open",
        "Console",
    );
}

#[test]
fn a_type_argument_carries_its_callback_row_to_the_match() {
    assert_escape(
        "option-argument",
        r#"fn open_opt (o : Option (Unit ->[Console] HostIO APartial Unit))
  : HostIO APartial Unit =
  match o {
    Some g ↦ g MkUnit;
    None ↦ host_pure APartial Unit MkUnit
  }
"#,
        "open_opt",
        "Console",
    );
}

#[test]
fn record_and_alias_admission_carry_written_field_rows() {
    assert_escape(
        "record-field",
        r#"record Carrier { cb : Unit ->[Console] HostIO APartial Unit }
fn open_record (r : Carrier) : HostIO APartial Unit = r.cb MkUnit
"#,
        "open_record",
        "Console",
    );
    assert_escape(
        "alias-body",
        r#"def Callback = Unit ->[Console] HostIO APartial Unit
fn open_alias (f : Callback) : HostIO APartial Unit = f MkUnit
"#,
        "open_alias",
        "Console",
    );
}

#[test]
fn higher_order_let_keeps_its_checked_dispatch_and_output_order() {
    let out = cli("qa-higher-order", "run", QA_LET);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        out.stdout, b"ab\n",
        "the callback runs before its continuation"
    );
}
