//! Spec 36 §1.2, LANG-EFFECT-ROW-PHANTOM-TYPE-ARGUMENT AC-1/AC-2.
//! Promise class: durable behavioral invariant. MEASURED: Tag Box runs like
//! Tag Unit in four binder/annotation forms, while a held Box still refuses.
//! CLAIMED: only phantom type arguments are free of their carried row. THE GAP:
//! the elaborator's per-position unit pins cover additional charged shapes;
//! these CLI fixtures observe the false-refusal and held-row boundaries.

use std::process::{Command, Output};

const PREFIX: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line, write)
import Capability.Process.Exit (host_bind, host_console, host_program, host_pure)
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
data PBox = MkPBox (Unit -> HostIO APartial Unit)
data Tag (a : Type) : Type where { MkTag : PBox -> Tag a }
data HoldP (a : Type) : Type where { MkHoldP : a -> PBox -> HoldP a }
data NestP (a : Type) : Type where { MkNestP : List a -> PBox -> NestP a }
fn quiet (x : Unit) : HostIO APartial Unit = host_pure APartial Unit MkUnit
"#;
const MAIN: &str = r#"proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_program APartial (print_line "b")
"#;

fn cli(name: &str, source: &str) -> Output {
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/tmp")
        .join(format!("ken-phantom-row-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("fixture directory");
    let path = dir.join(format!("{name}.ken"));
    std::fs::write(&path, source).expect("write inline Ken source");
    let output = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("run")
        .arg(&path)
        .output()
        .expect("run Ken CLI");
    let _ = std::fs::remove_file(&path);
    output
}

fn source(body: &str) -> String {
    format!("{PREFIX}proc fire (u : Unit) : Unit visits [Console] =\n  {body}\n{MAIN}")
}

fn assert_prints_b(name: &str, src: &str) {
    let out = cli(name, src);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{name}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(out.stdout, b"b\n", "{name}: unexpected output");
}

fn assert_carried_row_refusal(name: &str, src: &str) {
    // The exact base Tag Box repro refuses at this pure callback's let.
    // E0087 alone is insufficient: an unrelated effectful Box callback
    // also yields E0087 at a different site.
    const SITE: &str = "ignored = g MkUnit in MkUnit";
    let start = src.find(SITE).expect("one PBox callback let");
    assert_eq!(src.rfind(SITE), Some(start), "{name}: ambiguous site");
    let out = cli(name, src);
    let err = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(1), "{name}: {err}");
    assert!(out.stdout.is_empty(), "{name}: refused program ran");
    assert!(
        err.starts_with("error[E0087]"),
        "{name}: wrong refusal: {err}"
    );
    assert!(err.contains("UnsupportedEffectInclusion"), "{name}: {err}");
    let span = format!(
        "span: Span {{ start: {start}, end: {} }}",
        start + SITE.len()
    );
    assert!(
        err.contains(&span),
        "{name}: refusal missed carried-row site {span}: {err}"
    );
}

const SCRUTINEE: &str = "match MkTag Box (MkPBox quiet) { MkTag p ↦ match p { MkPBox g ↦ let ignored = g MkUnit in MkUnit } }";
const LAMBDA_BINDER: &str = "((\\t. match t { MkTag p ↦ match p { MkPBox g ↦ let ignored = g MkUnit in MkUnit } }) : Tag Box -> Unit) (MkTag Box (MkPBox quiet))";
const NESTED_PATTERN: &str =
    "match MkTag Box (MkPBox quiet) { MkTag (MkPBox g) ↦ let ignored = g MkUnit in MkUnit }";
const ANNOTATED_LET: &str = "let t : Tag Box = MkTag Box (MkPBox quiet) in match t { MkTag p ↦ match p { MkPBox g ↦ let ignored = g MkUnit in MkUnit } }";

fn assert_phantom_pair(name: &str, body: &str) {
    let candidate = source(body);
    let control = source(
        &body
            .replace("Tag Box", "Tag Unit")
            .replace("MkTag Box", "MkTag Unit"),
    );
    assert_prints_b(name, &candidate);
    assert_prints_b(&format!("{name}-unit-control"), &control);
}

#[test]
fn phantom_scrutinee_prints_b_like_tag_unit() {
    assert_phantom_pair("scrutinee", SCRUTINEE);
}

#[test]
fn phantom_lambda_binder_prints_b_like_tag_unit() {
    assert_phantom_pair("lambda-binder", LAMBDA_BINDER);
}

#[test]
fn phantom_nested_pattern_prints_b_like_tag_unit() {
    assert_phantom_pair("nested-pattern", NESTED_PATTERN);
}

#[test]
fn phantom_annotated_let_prints_b_like_tag_unit() {
    assert_phantom_pair("annotated-let", ANNOTATED_LET);
}

#[test]
fn hold_box_field_charges_row_at_pbox_callback_let() {
    assert_carried_row_refusal("hold-box", &source(
        "match MkHoldP Box (MkBox action) (MkPBox quiet) { MkHoldP b p ↦ match p { MkPBox g ↦ let ignored = g MkUnit in MkUnit } }"));
    assert_prints_b("hold-unit", &source(
        "match MkHoldP Unit MkUnit (MkPBox quiet) { MkHoldP b p ↦ match p { MkPBox g ↦ let ignored = g MkUnit in MkUnit } }"));
}

#[test]
fn nest_box_field_charges_row_at_pbox_callback_let() {
    assert_carried_row_refusal("nest-box", &source(
        "match MkNestP Box (Cons Box (MkBox action) (Nil Box)) (MkPBox quiet) { MkNestP bs p ↦ match p { MkPBox g ↦ let ignored = g MkUnit in MkUnit } }"));
    assert_prints_b("nest-unit", &source(
        "match MkNestP Unit (Cons Unit MkUnit (Nil Unit)) (MkPBox quiet) { MkNestP bs p ↦ match p { MkPBox g ↦ let ignored = g MkUnit in MkUnit } }"));
}
