//! Spec 36 §2.5, FSRA Increment 1: a non-space proc's final non-tree
//! result carries the applicable effect signature, indexed by a *written*
//! checked Cap. Promise class: durable invariant. MEASURED: core-type
//! conversion on seven discriminating shapes, exact CLI diagnostics, and
//! executed output for FS-plus-Console and arrow-final lets. CLAIMED: the
//! returned tree has the right signature, authority index and placement.
//! THE GAP: these examples do not enumerate every nested arrow or host op;
//! the summand classifier's HostOpV1 match is exhaustive.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{convert_type, Context, Decl};
use std::process::{Command, Output};

fn assert_core_type(case: &str, source: &str, name: &str, expected: &str) {
    let mut env = ElabEnv::new().expect("checked prelude");
    let source = format!("{source}\ndef Expected = {expected}\n");
    env.elaborate_file(&source)
        .unwrap_or_else(|error| panic!("{case}: {error:?}"));
    let id = env.globals[name];
    let expected_id = env.globals["Expected"];
    let Some(Decl::Transparent { ty: actual, .. }) = env.env.lookup(id) else {
        panic!("{case}: tested declaration is not a checked definition");
    };
    let Some(Decl::Transparent { body: expected, .. }) = env.env.lookup(expected_id) else {
        panic!("{case}: expected type is not a checked definition");
    };
    assert!(
        convert_type(&env.env, &Context::new(), actual, expected),
        "{case}: emitted core type {actual:?} differs from separately checked {expected:?}"
    );
}

fn cli(case: &str, subcommand: &str, source: &str) -> Output {
    let root = std::env::temp_dir().join(format!("ken-fsra-inc1-{}", std::process::id()));
    std::fs::create_dir_all(&root).expect("create fixture root");
    let path = root.join(format!("{case}.ken"));
    std::fs::write(&path, source).expect("write fixture");
    Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg(subcommand)
        .arg(path)
        .output()
        .expect("invoke Ken CLI")
}

fn assert_check_error(case: &str, source: &str, code: &str, reason: &str) {
    let output = cli(case, "check", source);
    assert_eq!(output.status.code(), Some(1), "{case}: {output:?}");
    assert!(output.stdout.is_empty(), "{case}: check cannot run effects");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(code), "{case}: wrong code: {stderr}");
    assert!(stderr.contains(reason), "{case}: wrong refusal: {stderr}");
}

#[test]
fn written_cap_binds_fs_tree_at_final_value() {
    assert_core_type(
        "P1 written dependent authority",
        "proc f (a : Auth) (c : Cap a) (x : Nat) : Nat visits [FS] = x",
        "f",
        "(a : Auth) -> Cap a -> Nat -> FS a Nat",
    );
}

#[test]
fn fs_plus_console_cap_gives_hostio_and_runs_one_action_before_result() {
    assert_core_type(
        "P2 closed authority and ambient",
        "proc f (c : Cap AFull) : String visits [FS, Console] =\n\
           let r = print_line \"x\" in \"one\"",
        "f",
        "Cap AFull -> HostIO AFull String",
    );
    let source = "program capabilities FS AFull\n\
        proc f (c : Cap AFull) : String visits [FS, Console] =\n\
          let r = print_line \"x\" in \"one\"\n\
        proc main (_input : ProcessInput) (caps : ProgramCaps AFull)\n\
          : HostIO AFull ExitCode visits [FS, Console] =\n\
          match caps {\n\
            ProgramCaps.MkProgramCaps cap |->\n\
              let selected : String = f cap in\n\
              host_program AFull (print_line selected)\n\
          }\n";
    let run = cli("P2-fs-console", "run", source);
    assert_eq!(
        run.status.code(),
        Some(0),
        "P2: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(run.stdout, b"x\none\n");
}

#[test]
fn host_operation_labels_select_their_family_summand() {
    assert_core_type(
        "P3 Process plus Console",
        "proc p (c : Cap APartial) : Nat visits [Process, Console] = Suc Zero",
        "p",
        "Cap APartial -> HostIO APartial Nat",
    );
    assert_core_type(
        "P3 FsOpen",
        "proc p (c : Cap AFull) : Nat visits [FsOpen] = Suc Zero",
        "p",
        "Cap AFull -> FS AFull Nat",
    );
}

#[test]
fn fs_label_without_written_cap_contributes_no_host_summand() {
    assert_core_type(
        "P4 FS only without Cap",
        "proc p (x : Nat) : Nat visits [FS] = x",
        "p",
        "Nat -> Nat",
    );
    assert_core_type(
        "P4 FS and Console without Cap",
        "proc p (x : Nat) : Nat visits [FS, Console] = x",
        "p",
        "Nat -> IO Nat",
    );
}

#[test]
fn written_cap_indices_must_convert_in_the_written_context() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let err = env
        .elaborate_file(
            "proc p (a : Auth) (first : Cap a) (second : Cap AFull)\n\
               : Nat visits [FS] = Suc Zero",
        )
        .expect_err("two distinct written Auth indices cannot share an FS tree");
    assert!(
        matches!(&err, ElabError::TypeMismatch { reason, .. }
            if reason.contains("written FS capabilities with different Auth indices")),
        "P5 must reject the index mismatch: {err:?}"
    );
    assert_core_type(
        "P5 equal bound Auth indices",
        "proc p (a : Auth) (first : Cap a) (second : Cap a)\n\
           : Nat visits [FS] = Suc Zero",
        "p",
        "(a : Auth) -> Cap a -> Cap a -> FS a Nat",
    );
}

#[test]
fn arrow_final_result_wraps_only_inside_the_last_lambda() {
    assert_core_type(
        "P6 arrow-final",
        "proc h : Unit -> Nat visits [Console] =\n\
           \\_. let r = print_line \"x\" in Suc Zero",
        "h",
        "Unit -> IO Nat",
    );
    let source = "program capabilities FS APartial\n\
        proc h : Unit -> Nat visits [Console] =\n\
          \\_. let r = print_line \"x\" in Suc Zero\n\
        proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
          : HostIO APartial ExitCode visits [Console] =\n\
          let n : Nat = h MkUnit in host_program APartial (print_line \"y\")\n";
    let run = cli("P6-arrow-inner", "run", source);
    assert_eq!(
        run.status.code(),
        Some(0),
        "P6: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(run.stdout, b"x\ny\n");
    assert_check_error(
        "P6-arrow-outer",
        "proc outer : Unit -> IO Unit visits [Console] =\n\
         let r = print_line \"x\" in\n\
         \\_. ITree.Ret ConsoleOp console_resp Unit MkUnit",
        "error[E0038]",
        "cannot be sequenced (36 §2.4)",
    );
}

#[test]
fn user_rebound_cap_name_does_not_supply_prelude_authority() {
    assert_core_type(
        "P7 shadowed Cap",
        "data Cap (a : Auth) : Type where { MkCap : Cap a }\n\
         proc p (c : Cap AFull) : Nat visits [FS, Console] = Suc Zero",
        "p",
        "Cap AFull -> IO Nat",
    );
}
