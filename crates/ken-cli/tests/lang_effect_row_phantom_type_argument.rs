//! Spec 36 §1.2, LANG-EFFECT-ROW-PHANTOM-TYPE-ARGUMENT AC-1/AC-2.
//! Promise class: durable behavioral invariant. MEASURED: Tag Box runs like
//! Tag Unit in four binder/annotation forms, while a held Box still refuses.
//! CLAIMED: only phantom type arguments are free of their carried row. THE GAP:
//! the elaborator's per-position unit pins cover additional charged shapes;
//! these CLI fixtures observe the false-refusal and held-row boundaries.

use std::process::{Command, Output};

const PREFIX: &str = r#"program capabilities FS APartial
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
data PBox = MkPBox (Unit -> HostIO APartial Unit)
data Tag (a : Type) : Type where { MkTag : PBox -> Tag a }
data Hold (a : Type) : Type where { MkHold : a -> Hold a }
fn quiet (x : Unit) : HostIO APartial Unit = host_pure APartial Unit MkUnit
"#;
const MAIN: &str = r#"proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_program APartial (print_line "b")
"#;

fn cli(name: &str, source: &str) -> Output {
    let dir = std::env::temp_dir().join(format!("ken-phantom-row-{}", std::process::id()));
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

fn assert_escape(name: &str, src: &str) {
    let out = cli(name, src);
    let err = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(1), "{name}: {err}");
    assert!(out.stdout.is_empty(), "{name}: refused program ran");
    assert!(
        err.starts_with("error[E0038]"),
        "{name}: wrong refusal: {err}"
    );
    assert!(
        err.contains("this let's right-hand side performs an action"),
        "{name}: wrong phase: {err}"
    );
}

const SCRUTINEE: &str = "match MkTag Box (MkPBox quiet) { MkTag p ↦ match p { MkPBox g ↦ let ignored = g MkUnit in MkUnit } }";
const LAMBDA_BINDER: &str = "((\\t. match t { MkTag p ↦ match p { MkPBox g ↦ let ignored = g MkUnit in MkUnit } }) : Tag Box -> Unit) (MkTag Box (MkPBox quiet))";
const NESTED_PATTERN: &str =
    "match MkTag Box (MkPBox quiet) { MkTag (MkPBox g) ↦ let ignored = g MkUnit in MkUnit }";
const ANNOTATED_LET: &str = "let t : Tag Box = MkTag Box (MkPBox quiet) in match t { MkTag p ↦ match p { MkPBox g ↦ let ignored = g MkUnit in MkUnit } }";

#[test]
fn phantom_argument_four_forms_are_byte_equal_to_tag_unit() {
    for (name, body) in [
        ("scrutinee", SCRUTINEE),
        ("lambda-binder", LAMBDA_BINDER),
        ("nested-pattern", NESTED_PATTERN),
        ("annotated-let", ANNOTATED_LET),
    ] {
        let candidate = source(body);
        let control = source(
            &body
                .replace("Tag Box", "Tag Unit")
                .replace("MkTag Box", "MkTag Unit"),
        );
        assert_prints_b(name, &candidate);
        assert_prints_b(&format!("{name}-unit-control"), &control);
    }
}

#[test]
fn held_box_and_list_box_callbacks_still_refuse_e0038() {
    for (name, scrutinee) in [
        ("hold-box", "match MkHold Box (MkBox action) { MkHold b ↦ match b { MkBox f ↦ let ignored = f MkUnit in MkUnit } }"),
        ("list-box", "match Cons Box (MkBox action) (Nil Box) { Nil ↦ MkUnit ; Cons b bs ↦ match b { MkBox f ↦ let ignored = f MkUnit in MkUnit } }"),
    ] {
        assert_escape(name, &source(scrutinee));
    }
}
