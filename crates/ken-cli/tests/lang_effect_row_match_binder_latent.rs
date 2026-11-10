//! Spec 36 §1.1–1.4 and §2.4; Architect MATCH-BINDER-LATENT AC-0.
//! Promise class: durable invariant. MEASURED: checked Ken programs execute
//! match-bound callbacks before their continuation, or refuse a non-tree
//! result rather than dropping the action. CLAIMED: a match binder releases
//! what its scrutinee releases, including nested, virtual, alias, and copied
//! binders. THE GAP: the row plane is per type, not per constructor field, so
//! a pure field of a mixed-row type can be over-approximated and refused; the
//! per-field plane is a successor. An application returning a closure carries
//! its evaluation row, not the returned closure's own latent. The Architect's
//! 36 early-refusal corpus files were not fully observed.

use std::process::{Command, Output};

fn cli(name: &str, verb: &str, source: &str) -> Output {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/tmp")
        .join(format!("ken-match-binder-{}", std::process::id()));
    std::fs::create_dir_all(&root).expect("fixture directory");
    let path = root.join(format!("{name}.ken"));
    std::fs::write(&path, source).expect("write inline source");
    Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg(verb)
        .arg(path)
        .output()
        .expect("run Ken CLI")
}

fn assert_check(name: &str, source: &str) {
    let out = cli(name, "check", source);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{name}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

fn assert_run(name: &str, source: &str, expected: &[u8]) {
    let out = cli(name, "run", source);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{name}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        out.stdout, expected,
        "{name}: action was lost or misordered"
    );
}

// Each source is copied verbatim from the Architect's 788bb5d9e D0 probe.
// Inline Rust constants do not enter the strict-isolated .ken fixture census.
const P1: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line, write)
import Capability.Process.Exit (host_bind, host_console, host_program, host_pure)
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
proc run_with (b : Box)
  : HostIO APartial ExitCode visits [Console] =
  match b { MkBox f ↦ let ignored = f MkUnit in
    host_program APartial (print_line "b") }
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  run_with (MkBox action)
"#;

const P2: &str = r#"program capabilities FS APartial
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
proc run_with (b : PBox)
  : HostIO APartial ExitCode visits [Console] =
  match b { MkPBox f ↦ let ignored = f MkUnit in
    host_program APartial (print_line "b") }
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  run_with (MkPBox quiet)
"#;

const P3: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line, write)
import Capability.Process.Exit (host_bind, host_console, host_program, host_pure)
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
fn quiet (x : Unit) : HostIO APartial Unit = host_pure APartial Unit MkUnit
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  match MkBox action { MkBox f ↦ let ignored = f MkUnit in
    host_program APartial (print_line "b") }
"#;

const P4: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line, write)
import Capability.Process.Exit (host_bind, host_console, host_program, host_pure)
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
fn quiet (x : Unit) : HostIO APartial Unit = host_pure APartial Unit MkUnit
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
proc run_with (b : Box) (u : Unit)
  : HostIO APartial ExitCode visits [Console] =
  match b { MkBox f ↦ match u { MkUnit ↦ let ignored = f MkUnit in
    host_program APartial (print_line "b") } }
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  run_with (MkBox action) MkUnit
"#;

const P5: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line, write)
import Capability.Process.Exit (host_bind, host_console, host_program, host_pure)
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
fn quiet (x : Unit) : HostIO APartial Unit = host_pure APartial Unit MkUnit
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
data BoxPair = MkBoxPair Box Unit
proc run_with (p : BoxPair)
  : HostIO APartial ExitCode visits [Console] =
  match p { MkBoxPair (MkBox f) _ ↦ let ignored = f MkUnit in
    host_program APartial (print_line "b") }
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  run_with (MkBoxPair (MkBox action) MkUnit)
"#;

const P6: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line, write)
import Capability.Process.Exit (host_bind, host_console, host_program, host_pure)
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
fn quiet (x : Unit) : HostIO APartial Unit = host_pure APartial Unit MkUnit
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
proc run_with (b : Box)
  : HostIO APartial ExitCode visits [Console] =
  match b { MkBox f as whole ↦ let ignored = f MkUnit in
    host_program APartial (print_line "b") }
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  run_with (MkBox action)
"#;

const P7: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line, write)
import Capability.Process.Exit (host_bind, host_console, host_program, host_pure)
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
fn quiet (x : Unit) : HostIO APartial Unit = host_pure APartial Unit MkUnit
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
proc run_with (b : Box)
  : HostIO APartial ExitCode visits [Console] =
  match b { MkBox f as whole ↦ match whole { MkBox g ↦ let ignored = g MkUnit in
    host_program APartial (print_line "b") } }
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  run_with (MkBox action)
"#;

const P9: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line, write)
import Capability.Process.Exit (host_bind, host_console, host_program, host_pure)
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
fn quiet (x : Unit) : HostIO APartial Unit = host_pure APartial Unit MkUnit
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
proc count (b : Box) : Int visits [Console] =
  match b { MkBox f ↦ let u = f MkUnit in 1 }
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_program APartial (print_line "b")
"#;

const P10: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line, write)
import Capability.Process.Exit (host_bind, host_console, host_program, host_pure)
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
fn quiet (x : Unit) : HostIO APartial Unit = host_pure APartial Unit MkUnit
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
proc count (b : Box) : Int visits [Console] =
  match b { MkBox f ↦ let k : (Unit -> HostIO APartial Unit) -> Int = \h. let u = h MkUnit in 1 in k quiet }
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_program APartial (print_line "b")
"#;

const P11: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line, write)
import Capability.Process.Exit (host_bind, host_console, host_program, host_pure)
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
fn quiet (x : Unit) : HostIO APartial Unit = host_pure APartial Unit MkUnit
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
proc run_with (b : Box)
  : HostIO APartial ExitCode visits [Console] =
  match b { MkBox f ↦ let k = f in let ignored = k MkUnit in
    host_program APartial (print_line "b") }
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  run_with (MkBox action)
"#;

const P12: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line, write)
import Capability.Process.Exit (host_bind, host_console, host_program, host_pure)
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
fn quiet (x : Unit) : HostIO APartial Unit = host_pure APartial Unit MkUnit
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
proc run_with (b : Box)
  : HostIO APartial ExitCode visits [Console] =
  let r = match b { MkBox f ↦ let ignored = f MkUnit in
    host_pure APartial Unit MkUnit } in
  host_program APartial (print_line "b")
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  run_with (MkBox action)
"#;

const P14: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line, write)
import Capability.Process.Exit (host_bind, host_console, host_program, host_pure)
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
fn quiet (x : Unit) : HostIO APartial Unit = host_pure APartial Unit MkUnit
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
proc run_with (b : Box) (u : Unit)
  : HostIO APartial ExitCode visits [Console] =
  match b { MkBox f as whole ↦ match u { MkUnit ↦ match whole { MkBox g ↦
    let ignored = g MkUnit in host_program APartial (print_line "b") } } }
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  run_with (MkBox action) MkUnit
"#;

const P15: &str = r#"program capabilities FS APartial
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
fn open2 (b : PBox) : HostIO APartial Unit =
  match b { MkPBox f as w ↦ match w { MkPBox g ↦ let k = g in k MkUnit } }
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_program APartial (print_line "b")
"#;

#[test]
fn r1_flat_match_binder_sequences_action_before_continuation() {
    assert_run("r1-flat", P1, b"ab\n");
}

#[test]
fn r2_pure_field_does_not_perform() {
    assert_run("r2-pure-field", P2, b"b\n");
}

#[test]
fn r3_application_scrutinee_releases_its_evaluation_row() {
    assert_run("r3-app-scrut", P3, b"ab\n");
}

#[test]
fn r4_inner_unrelated_match_preserves_outer_latent() {
    assert_run("r4-inner-unrelated", P4, b"ab\n");
}

#[test]
fn r5_nested_pattern_releases_outer_latent() {
    assert_run("r5-nested-pattern", P5, b"ab\n");
}

#[test]
fn r6_as_alias_arm_does_not_shift_the_binder() {
    assert_run("r6-as-arm", P6, b"ab\n");
}

#[test]
fn r7_rematched_alias_releases_enclosing_latent() {
    assert_run("r7-rematch-alias", P7, b"ab\n");
}

#[test]
fn r8_let_bound_copy_of_binder_releases_latent() {
    assert_run("r8-let-copy", P11, b"ab\n");
}

#[test]
fn r9_inferred_match_in_let_rhs_sequences() {
    assert_run("r9-inferred-match", P12, b"ab\n");
}

#[test]
fn r10_outer_alias_inside_inner_arm_releases_latent() {
    assert_run("r10-outer-alias", P14, b"ab\n");
}

/// Transition sentinel, owned by SPEC-EFFECTS-DECLARED-TREE-RESULT-DENOTATION:
/// count's tree is IO Int and the callback's declared tree is HostIO APartial
/// Unit, which no inclusion maps into IO, so the let refuses instead of
/// dropping the action.
#[test]
fn r11_non_tree_result_refuses_instead_of_dropping_action() {
    let out = cli("r11-nontree", "check", P9);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(1), "r11: {stderr}");
    assert!(out.stdout.is_empty(), "r11: check cannot perform an action");
    assert!(stderr.contains("error[E0087]"), "r11: {stderr}");
    assert!(
        stderr.contains("UnsupportedEffectInclusion"),
        "r11: {stderr}"
    );
}

#[test]
fn c1_lambda_parameter_pushed_inside_arm_keeps_its_pure_row() {
    assert_check("c1-lambda-check", P10);
    assert_run("c1-lambda-run", P10, b"b\n");
}

#[test]
fn c2_pure_alias_rematch_keeps_an_empty_row() {
    assert_check("c2-pure-alias-check", P15);
    assert_run("c2-pure-alias-run", P15, b"b\n");
}
