//! Spec 36 §§1.1–1.4, 2.4; LET-BOUND-SCRUTINEE Architect AC-0.
//! Promise class: durable invariant. MEASURED: checked CLI programs either
//! execute a let-bound scrutinee's action before their continuation or reject
//! a non-tree result with E0038; a pure callback prints only the continuation.
//! CLAIMED: a let-bound value releases its carried latent once, including
//! constructor applications and other value forms, but not a performed tree's
//! already-paid evaluation row. THE GAP: these rows cover the named value
//! forms, not an RCell read directly or nested under an operand-joining
//! form; CELL-SCRUTINEE-CARRIED-ROW pins both. An application returning a
//! closure remains a declared MATCH-BINDER residual.
//! Inline Ken sources keep the strict-isolated .ken fixture census unchanged.

use std::process::{Command, Output};

const HEADER: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line, write)
import Capability.Process.Exit (host_bind, host_console, host_program, host_pure)
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
fn quiet (x : Unit) : HostIO APartial Unit = host_pure APartial Unit MkUnit
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
data PBox = MkPBox (Unit -> HostIO APartial Unit)
data Flag = On | Off
def Callback = Unit ->[Console] HostIO APartial Unit
record Carrier { cb : Unit ->[Console] HostIO APartial Unit }
proc get_box (x : Unit) : HostIO APartial Box visits [Console] =
  host_pure APartial Box (MkBox action)
proc mk_tree (x : Unit) : HostIO APartial (HostIO APartial Unit) visits [Console] =
  host_pure APartial (HostIO APartial Unit) (action MkUnit)
proc get_cb (x : Unit) : HostIO APartial Callback visits [Console] =
  host_pure APartial Callback action
proc use_c (r : Carrier) : HostIO APartial ExitCode visits [Console] =
  let k = r.cb in
  let ignored = k MkUnit in
  host_program APartial (print_line "b")
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
"#;

const MATCH_B: &str = "  match b { MkBox f ↦ let ignored = f MkUnit in\n    host_program APartial (print_line \"b\") }\n";

const SPACE_HEADER: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line, write)
import Capability.Process.Exit (host_bind, host_console, host_program, host_pure)
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
space Holder {
  mut held : Box = MkBox action
  proc fire () : Unit visits [Holder, Console] =
"#;

const SPACE_TAIL: &str = r#"}
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_program APartial (print_line "b")
"#;

fn cli(name: &str, verb: &str, source: &str) -> Output {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/tmp")
        .join(format!("ken-let-bound-scrutinee-{}", std::process::id()));
    std::fs::create_dir_all(&root).expect("fixture directory");
    let path = root.join(format!("{name}.ken"));
    std::fs::write(&path, source).expect("write inline checked source");
    Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg(verb)
        .arg(path)
        .output()
        .expect("run Ken CLI")
}

fn assert_run(name: &str, source: &str, expected: &[u8]) {
    let out = cli(name, "run", source);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{name}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(out.stdout, expected, "{name}: wrong effect or order");
}

fn assert_non_tree_refusal(name: &str, source: &str) {
    let out = cli(name, "check", source);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(1), "{name}: {stderr}");
    assert!(out.stdout.is_empty(), "{name}: check must not execute");
    assert!(stderr.contains("error[E0038]"), "{name}: {stderr}");
    assert!(
        stderr.contains("cannot be sequenced (36 §2.4)"),
        "{name}: {stderr}"
    );
}

fn match_b(body_prefix: &str) -> String {
    format!("{HEADER}{body_prefix}{MATCH_B}")
}

#[test]
fn q01_unannotated_constructor_application_releases_callback() {
    assert_run("q01-app", &match_b("  let b = MkBox action in\n"), b"ab\n");
}

#[test]
fn q02_pure_callback_same_application_stays_pure() {
    let src = format!(
        "{HEADER}  let b = MkPBox quiet in\n  match b {{ MkPBox f ↦ let ignored = f MkUnit in\n    host_program APartial (print_line \"b\") }}\n"
    );
    assert_run("q02-pure", &src, b"b\n");
}

#[test]
fn q03_if_constructed_scrutinee_releases_callback() {
    assert_run(
        "q03-if",
        &match_b("  let b = if True then MkBox action else MkBox action in\n"),
        b"ab\n",
    );
}

#[test]
fn q04_match_constructed_scrutinee_releases_callback() {
    assert_run(
        "q04-match",
        &match_b("  let b = match On { On ↦ MkBox action ; Off ↦ MkBox action } in\n"),
        b"ab\n",
    );
}

#[test]
fn q05_nested_let_scrutinee_releases_callback() {
    assert_run(
        "q05-let",
        &match_b("  let b = (let c = MkBox action in c) in\n"),
        b"ab\n",
    );
}

#[test]
fn q06_pair_projection_scrutinee_releases_callback() {
    assert_run(
        "q06-pair-projection",
        &match_b("  let p = (MkBox action, MkUnit) in\n  let b = p.1 in\n"),
        b"ab\n",
    );
}

#[test]
fn q08_record_projection_passes_carried_callback() {
    assert_run(
        "q08-proj",
        &format!("{HEADER}  use_c ({{ cb = action }} : Carrier)\n"),
        b"ab\n",
    );
}

#[test]
fn q10_sequenced_box_result_releases_callback() {
    assert_run(
        "q10-sequenced-box",
        &match_b("  let b = get_box MkUnit in\n"),
        b"ab\n",
    );
}

#[test]
fn q11_sequenced_tree_result_is_not_performed_twice() {
    assert_run(
        "q11-sequenced-tree",
        &format!(
            "{HEADER}  let t = mk_tree MkUnit in\n  let s = t in\n  host_program APartial (print_line \"b\")\n"
        ),
        b"b\n",
    );
}

#[test]
fn q12_sequenced_arrow_result_releases_callback() {
    assert_run(
        "q12-sequenced-arrow",
        &format!(
            "{HEADER}  let k = get_cb MkUnit in\n  let ignored = k MkUnit in\n  host_program APartial (print_line \"b\")\n"
        ),
        b"ab\n",
    );
}

#[test]
fn q15_cell_bound_without_annotation_refuses_non_tree_action() {
    let src = format!(
        "{SPACE_HEADER}    let b = held in\n    match b {{ MkBox f ↦ let ignored = f MkUnit in MkUnit }}\n{SPACE_TAIL}"
    );
    assert_non_tree_refusal("q15-cell-let", &src);
}
