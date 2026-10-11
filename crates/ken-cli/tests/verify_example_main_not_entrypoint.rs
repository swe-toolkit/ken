//! A checked example's main is checked but cannot supply a program entrypoint.
//! Contract: spec 33 §3.2.1 and 62 §3.2; frame VERIFY-EXAMPLE-MAIN-NOT-ENTRYPOINT.

use std::fs;
use std::process::Command;

use ken_cli::{run_program, RunError, SourceFormat};
use ken_elaborator::compiler_driver::{
    prepare_native_program_sources, CompilerSource, NativeProgramBuildError,
};
use ken_elaborator::program_admission::{admit_checked_main, ProgramAdmissionError};
use ken_elaborator::ElabEnv;
use ken_interp::PosixHost;
use tempfile::tempdir;

const MAIN: &str = "fn main (_input : ProcessInput) (_caps : ProgramCaps ANone) : HostIO ANone ExitCode = host_exit ANone Success";
// This is the exact AC-0 source: the first fence has a valid program boundary
// but no main, and only the checked example declares main.
const EXAMPLE_ONLY: &str = "# Example-only entry point\n\n```ken\nprogram capabilities FS ANone\nconst plain_probe : Bool = True\n```\n\n```ken example\nfn main (_input : ProcessInput) (_caps : ProgramCaps ANone) : HostIO ANone ExitCode = host_exit ANone Success\n```\n";

fn with_program_main(example: Option<&str>) -> String {
    let mut source = format!("```ken\nprogram capabilities FS ANone\n{MAIN}\n```\n");
    if let Some(example) = example {
        source.push_str(&format!("```ken example\n{example}\n```\n"));
    }
    source
}

fn run_cli(source: &str) -> std::process::Output {
    let dir = tempdir().expect("fixture directory");
    let path = dir.path().join("entry.ken.md");
    fs::write(&path, source).expect("write fixture");
    Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("run")
        .arg(&path)
        .output()
        .expect("spawn ken run")
}

/// Promise: durable invariant. MEASURED: the exact AC-0 input returns the
/// typed missing-entry errors at the real runner and native preparation paths,
/// and the CLI refuses it. CLAIMED: a checked example cannot provide the
/// program's entry. THE GAP: this one input has no earlier elaboration error;
/// the printed-control row below proves the CLI can observe real execution.
#[test]
fn example_only_main_refused_before_either_entrypoint_admission() {
    let mut host = PosixHost::new();
    let run = run_program(
        EXAMPLE_ONLY,
        SourceFormat::LiterateKen,
        &[],
        &[],
        b"/",
        &mut host,
    );
    assert!(
        matches!(run, Err(RunError::MissingEntrypoint)),
        "runner must map typed MissingMain before execution, got {run:?}"
    );

    let native = prepare_native_program_sources(
        "example_main_ac1",
        vec![CompilerSource::new("entry.ken.md", EXAMPLE_ONLY)],
    );
    assert!(
        matches!(
            native,
            Err(NativeProgramBuildError::Admission(
                ProgramAdmissionError::MissingMain
            ))
        ),
        "native must refuse during admission, not on a missing package target: {native:?}"
    );

    let cli = run_cli(EXAMPLE_ONLY);
    assert!(!cli.status.success(), "checked-only entry must not run");
    assert!(
        cli.stdout.is_empty(),
        "missing entry must not produce output"
    );
    assert!(
        String::from_utf8_lossy(&cli.stderr).contains("missing entrypoint 'main'"),
        "CLI must report the admission failure: {:?}",
        cli.stderr
    );
}

/// Promise: durable invariant. MEASURED: a visible checked-only Console main
/// is rejected without printing, whereas moving that main into a plain fence
/// prints through the same CLI route. CLAIMED: the negative outcome is an
/// entry-selection refusal, not a CLI path that never runs valid programs.
/// THE GAP: this tests the direct CLI entry, not the roots-loaded module route.
#[test]
fn checked_only_main_cannot_print_but_own_main_can() {
    let printable = "proc main (_input : ProcessInput) (_caps : ProgramCaps ANone) : HostIO ANone ExitCode visits [Console] = host_program ANone (print_line \"entered main\")";
    let checked =
        format!("```ken\nprogram capabilities FS ANone\n```\n```ken example\n{printable}\n```\n");
    let own = format!("```ken\nprogram capabilities FS ANone\n{printable}\n```\n");
    let checked_result = run_cli(&checked);
    assert!(
        !checked_result.status.success(),
        "checked-only main must refuse"
    );
    assert!(
        checked_result.stdout.is_empty(),
        "checked-only main must not print"
    );
    assert!(
        String::from_utf8_lossy(&checked_result.stderr).contains("missing entrypoint 'main'"),
        "must refuse at admission: {:?}",
        checked_result.stderr
    );
    let own_result = run_cli(&own);
    assert!(
        own_result.status.success(),
        "own main must execute: {:?}",
        own_result.stderr
    );
    assert_eq!(own_result.stdout, b"entered main\n");
}

/// Promise: durable invariant. MEASURED: with/without an unrelated example,
/// program admission retains the exact own-main GlobalId, both production
/// routes admit, and native preparation selects the same main stable symbol.
/// CLAIMED: checked-example isolation does not break ordinary own-main entry.
/// THE GAP: a duplicate example main still rejects in the unchanged package
/// duplicate suite, not in this unrelated-example control.
#[test]
fn own_main_followed_by_unrelated_example_keeps_both_admissions() {
    let plain = with_program_main(None);
    let with = with_program_main(Some("const zz_unrelated : Nat = Zero"));
    let mut own_ids = Vec::new();
    let mut plans = Vec::new();
    let mut outcomes = Vec::new();
    for source in [&plain, &with] {
        let mut env = ElabEnv::new().expect("prelude");
        env.elaborate_ken_md_file_v1(source)
            .expect("own main and unrelated example must elaborate");
        own_ids.push(admit_checked_main(&env).expect("own main must admit").main);
        let mut host = PosixHost::new();
        outcomes.push(
            run_program(source, SourceFormat::LiterateKen, &[], &[], b"/", &mut host)
                .expect("reference runner must admit own main")
                .exit_status,
        );
        let preparation = prepare_native_program_sources(
            "example_main_ac2",
            vec![CompilerSource::new("entry.ken.md", source)],
        )
        .expect("native must prepare own main");
        plans.push(preparation.plan().main().clone());
    }
    assert_eq!(
        own_ids[0], own_ids[1],
        "example must not change own main ID"
    );
    assert_eq!(plans[0], plans[1], "native must select the same own main");
    assert_eq!(
        outcomes[0], outcomes[1],
        "runner result must remain unchanged"
    );
}

/// Promise: durable invariant. MEASURED: a successful checked-only main then
/// a failing fence leaves neither main entry binding behind. CLAIMED: the
/// executor restores program entry state on error as well as on success.
/// THE GAP: the deliberately failing second fence only tests this error order.
#[test]
fn failed_later_example_restores_absent_program_main() {
    let mut source = EXAMPLE_ONLY.to_string();
    source.push_str("```ken example\nconst bad : Nat = unbound_name\n```\n");
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_ken_md_file_v1(&source)
        .expect_err("later checked fence must fail");
    assert!(
        env.globals.get("main").is_none(),
        "example must not leave an entry binding"
    );
    assert!(
        env.effect_rows.get("main").is_none(),
        "example must not leave an entry row"
    );
    assert_eq!(
        admit_checked_main(&env),
        Err(ProgramAdmissionError::MissingMain)
    );
}
