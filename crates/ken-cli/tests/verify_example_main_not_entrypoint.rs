//! A checked example's main is checked but cannot supply a program entrypoint.
//! Contract: spec 33 §3.2.1 and 62 §3.2; frame VERIFY-EXAMPLE-MAIN-NOT-ENTRYPOINT.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use ken_cli::{run_program, RunError, SourceFormat};
use ken_elaborator::compiler_driver::{
    prepare_native_program_sources, CompilerSource, NativeProgramBuildError,
};
use ken_elaborator::program_admission::{admit_checked_main, ProgramAdmissionError};
use ken_elaborator::ElabEnv;
use ken_interp::PosixHost;
use tempfile::tempdir;

const MAIN: &str = "fn main (_input : ProcessInput) (_caps : ProgramCaps ANone) : HostIO ANone ExitCode = host_pure ANone ExitCode ExitCode.Success";
// The first fence has a valid program boundary but no main; only the
// checked example declares main. Source-only routes have no catalog root,
// so its wrapper import is supplied solely to the strict-entry pin below.
const EXAMPLE_ONLY: &str = "# Example-only entry point\n\n```ken\nprogram capabilities FS ANone\nconst plain_probe : Bool = True\n```\n\n```ken example\nfn main (_input : ProcessInput) (_caps : ProgramCaps ANone) : HostIO ANone ExitCode = host_pure ANone ExitCode ExitCode.Success\n```\n";

/// Promise class: durable invariant. MEASURED: the original literate source
/// fails at its example range, the same source with an ephemeral import
/// elaborates but has no program main, and its exact example body in the
/// document's own tangled environment names host_pure as the first Strict
/// miss. CLAIMED: the example needs that import and cannot supply an entry.
/// THE GAP: the plain-body result identifies the name, not the cause inside
/// the literate wrapper; the paired literate checks differ only by the import.
/// Runner/native/CLI admission is exercised independently below.
#[test]
fn strict_example_only_main_requires_import_then_still_is_not_entrypoint() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../catalog/packages")
        .canonicalize()
        .expect("catalog root");
    let roots = [root];
    let extracted = ken_elaborator::literate::extract_ken_md(EXAMPLE_ONLY)
        .expect("literate fixture extraction");
    let [range] = extracted.example_ranges.as_slice() else {
        panic!("fixture must contain exactly one checked example");
    };

    let mut unimported = ElabEnv::new().expect("prelude");
    match unimported.elaborate_isolated_ken_md_file_strict_v1(&roots, EXAMPLE_ONLY) {
        Err(ken_elaborator::ElabError::ParseError { msg, span }) => {
            assert_eq!(msg, "a 'ken example' block failed to elaborate");
            assert_eq!(span.start..span.end, range.clone());
        }
        Err(error) => panic!("unexpected strict refusal: {error:?}"),
        Ok(_) => panic!("example accessed host_pure without its package import"),
    }

    // The plain check uses the document's own tangled source and exact body.
    // It does not manufacture a different program header or import context.
    let body_start = extracted.source.len() + 1;
    let plain = format!("{}\n{}", extracted.source, &EXAMPLE_ONLY[range.clone()]);
    let mut body_check = ElabEnv::new().expect("prelude");
    match body_check.elaborate_isolated_file_strict_v1(&roots, &plain) {
        Err(ken_elaborator::ElabError::UnboundName { name, span }) => {
            assert_eq!(name, "host_pure");
            assert!(
                span.start >= body_start && span.end <= plain.len(),
                "first missing name must lie in the appended example: {span:?}"
            );
        }
        Err(error) => panic!("unexpected plain-body refusal: {error:?}"),
        Ok(_) => panic!("example body accessed host_pure without its package import"),
    }

    let imported = EXAMPLE_ONLY.replacen(
        "program capabilities FS ANone\n",
        "program capabilities FS ANone\nimport Capability.Process.Exit (host_pure)\n",
        1,
    );
    assert_ne!(imported, EXAMPLE_ONLY, "import must be composed");
    let mut selected = ElabEnv::new().expect("prelude");
    selected
        .elaborate_isolated_ken_md_file_strict_v1(&roots, &imported)
        .expect("checked example with explicit package import");
    assert_eq!(
        admit_checked_main(&selected),
        Err(ProgramAdmissionError::MissingMain),
        "an imported checked-example main is not the program entrypoint"
    );
}

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
