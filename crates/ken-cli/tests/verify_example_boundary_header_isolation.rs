//! A checked `ken example` may have its own header without changing the
//! program's admitted boundary (spec 33 §3.2.1; 62 §3.2).
//! Promise class: durable invariant. The fixture's declared authority, not
//! checked-only source, determines runner and native admission.

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use ken_elaborator::compiler_driver::{
    prepare_native_program_sources, CompilerSource, NativeProgramBuildError,
};
use ken_elaborator::program_admission::{admit_checked_main, ProgramAdmissionError};
use ken_elaborator::ElabEnv;
use tempfile::tempdir;

const FULL_EXAMPLE: &str = "program capabilities FS AFull\nconst zz_example_probe : Nat = Zero";
const BARE_EXAMPLE: &str = "const zz_example_probe : Nat = Zero";
const WRONG_ABI: &str = "program capabilities FS ANone\n\
    fn main (_input : ProcessInput) (_caps : ProgramCaps AFull) : HostIO AFull ExitCode = host_exit AFull Success";
const OWN_ABI: &str = "program capabilities FS ANone\n\
    fn main (_input : ProcessInput) (_caps : ProgramCaps ANone) : HostIO ANone ExitCode = host_exit ANone Success";
const PRINT_OWN_ABI: &str = "program capabilities FS ANone\n\
    proc main (_input : ProcessInput) (_caps : ProgramCaps ANone) : HostIO ANone ExitCode visits [Console] = host_program ANone (print_line \"own authority\")";
const HEADERLESS_ABI: &str = "fn main (_input : ProcessInput) (_caps : ProgramCaps AFull) : HostIO AFull ExitCode = host_exit AFull Success";
const FULL_WRITER_UNDER_ANONE: &str = r#"program capabilities FS ANone
proc main (input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match input {
    MkProcessInput arguments _environment _cwd |->
      match arguments {
        Nil |-> host_exit AFull (Failure 30) ;
        Cons path _ |-> match caps {
          MkProgramCaps cap |->
            bind (Coproduct (FSOp AFull) AmbientOp)
              (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
              (Result FileError Unit) ExitCode
              (inject_l (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp
                (Result FileError Unit) (writeFile cap path CreateNew path))
              (\written. match written {
                Err _ |-> host_exit AFull (Failure 31) ;
                Ok _ |-> host_exit AFull Success
              })
        }
      }
  }
"#;

fn literate(program: &str, example: Option<&str>) -> String {
    let mut text = format!("```ken\n{program}\n```\n");
    if let Some(example) = example {
        text.push_str(&format!("```ken example\n{example}\n```\n"));
    }
    text
}

fn run(path: &Path, source: &str) -> Output {
    fs::write(path, source).expect("write literate program");
    Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("run")
        .arg(path)
        .output()
        .expect("spawn production ken run")
}

fn assert_same_run(without: &Output, with: &Output) {
    assert_eq!(with.status.code(), without.status.code(), "exit status");
    assert_eq!(with.stdout, without.stdout, "stdout");
    assert_eq!(with.stderr, without.stderr, "stderr");
}

/// MEASURED: a real filesystem write requiring AFull cannot execute under
/// an ANone program boundary, with or without an AFull checked fence.
/// CLAIMED: example authority cannot be minted into a runner capability.
/// THE GAP: InvalidMainAbi is the narrow observable; the guarded path's
/// absence makes an incidental exit error unable to mask a live write.
#[test]
fn ken_run_example_header_cannot_mint_a_full_fs_write() {
    let dir = tempdir().expect("fixture root");
    let source = dir.path().join("entry.ken.md");
    let target = dir.path().join("forbidden.txt");
    let run_writer = |example| {
        fs::write(&source, literate(FULL_WRITER_UNDER_ANONE, example)).expect("write program");
        Command::new(env!("CARGO_BIN_EXE_ken"))
            .current_dir(dir.path())
            .arg("run")
            .arg(&source)
            .arg("--")
            .arg("forbidden.txt")
            .output()
            .expect("run writer through CLI")
    };
    let control = run_writer(None);
    assert!(!target.exists(), "ANone control must not write the target");
    let with = run_writer(Some(FULL_EXAMPLE));
    assert!(
        !target.exists(),
        "checked-only AFull must not write the target"
    );
    assert!(!control.status.success(), "ANone cannot admit AFull writer");
    assert!(
        String::from_utf8_lossy(&control.stderr).contains("ProgramCaps ANone"),
        "the mismatch must name the program boundary: {:?}",
        control.stderr
    );
    assert_same_run(&control, &with);
}

/// MEASURED: a runnable ANone header with an AFull-shaped main stays an
/// InvalidMainAbi refusal despite an AFull checked fence. CLAIMED: the
/// checked fence cannot mint the runner's authority. THE GAP: the paired
/// positive row also measures a valid ANone main so unconditional rejection
/// cannot satisfy both.
#[test]
fn ken_run_cannot_admit_a_full_main_from_an_example_header() {
    let dir = tempdir().expect("fixture root");
    let path = dir.path().join("entry.ken.md");
    let control = run(&path, &literate(WRONG_ABI, None));
    let with = run(&path, &literate(WRONG_ABI, Some(FULL_EXAMPLE)));
    assert!(!control.status.success(), "unmatched ANone ABI must refuse");
    assert!(
        String::from_utf8_lossy(&control.stderr).contains("ProgramCaps ANone"),
        "refusal must name the program's ANone ABI: {:?}",
        control.stderr
    );
    assert_same_run(&control, &with);
}

/// MEASURED: an ANone main with an AFull checked fence runs identically to
/// the no-example program. CLAIMED: checked-only authority does not widen
/// the runner's own authority. THE GAP: the mismatched-main row above is the
/// opposing discriminator for an illegitimate AFull substitution.
#[test]
fn ken_run_keeps_its_own_anone_main_with_a_full_example() {
    let dir = tempdir().expect("fixture root");
    let path = dir.path().join("entry.ken.md");
    let control = run(&path, &literate(PRINT_OWN_ABI, None));
    let with = run(&path, &literate(PRINT_OWN_ABI, Some(FULL_EXAMPLE)));
    assert!(
        control.status.success(),
        "own-authority run must succeed: {:?}",
        control.stderr
    );
    assert_eq!(control.stdout, b"own authority\n");
    assert_same_run(&control, &with);
}

fn native_error(program: &str, example: Option<&str>, dependency: bool) -> NativeProgramBuildError {
    let mut sources = Vec::new();
    if dependency {
        sources.push(CompilerSource::new(
            "dependency.ken.md",
            literate("const zz_dependency : Nat = Zero", example),
        ));
        sources.push(CompilerSource::new("entry.ken", program));
    } else {
        sources.push(CompilerSource::new(
            "entry.ken.md",
            literate(program, example),
        ));
    }
    prepare_native_program_sources("example_boundary_isolation", sources)
        .expect_err("native admission must refuse without a valid program boundary")
}

/// MEASURED: native preparation refuses the AFull main under the ANone
/// program header both with and without a checked AFull header. CLAIMED:
/// native authority follows the program, not the example. THE GAP: the
/// second native row catches laundering a header from a dependency.
#[test]
fn native_refuses_a_full_main_under_anone_despite_a_full_example() {
    for example in [None, Some(FULL_EXAMPLE)] {
        assert!(
            matches!(native_error(WRONG_ABI, example, false),
                NativeProgramBuildError::Admission(ProgramAdmissionError::InvalidMainAbi { authority })
                    if authority == "ANone"),
            "native must report the program's ANone authority"
        );
    }
}

/// MEASURED: a header only in a dependency's checked example does not
/// provide the later entry source's missing boundary. CLAIMED: a dependency
/// cannot supply runtime authority through checked-only content. THE GAP:
/// neither case has a real program header; both must reach typed admission.
#[test]
fn native_dependency_example_cannot_supply_missing_program_boundary() {
    for example in [None, Some(FULL_EXAMPLE)] {
        assert!(
            matches!(
                native_error(HEADERLESS_ABI, example, true),
                NativeProgramBuildError::Admission(ProgramAdmissionError::MissingProgramBoundary)
            ),
            "native must refuse the missing program boundary"
        );
    }
}

/// MEASURED: roots loading plus executing the entry's checked fences leaves
/// its program header equal to the header before the fence, even with AFull
/// in the fence. CLAIMED: the catalog-path route reaches the restoration
/// choke. THE GAP: root names are qualified and cannot use the direct CLI's
/// unqualified main admission; that admission is covered by the CLI rows.
#[test]
fn roots_entry_checked_example_cannot_replace_entry_boundary() {
    let root = tempdir().expect("catalog-like module root");
    let path = root.path().join("Entry.ken.md");
    for example in [None, Some(FULL_EXAMPLE)] {
        fs::write(&path, literate(OWN_ABI, example)).expect("write catalog-like entry");
        let mut env = ElabEnv::new().expect("prelude");
        env.elaborate_module_from_roots_v1(&[root.path().to_path_buf()], "Entry")
            .expect("entry with its own ANone program header");
        let program_header = env
            .boundary_header()
            .cloned()
            .expect("root program boundary");
        assert_eq!(
            program_header.capabilities.as_ref().expect("FS manifest")[0].authority,
            "ANone"
        );
        let results = env
            .execute_loaded_entry_checked_fences_v1("Entry")
            .expect("a checked AFull example is legal and executed");
        assert_eq!(results.len(), usize::from(example.is_some()));
        assert_eq!(env.boundary_header(), Some(&program_header));
    }
}

/// MEASURED: an example without a header preserves CLI exit/stdout/stderr,
/// including the successful ANone run. CLAIMED: restoration does not change
/// ordinary checked examples. THE GAP: this is one direct literate route;
/// the roots route above exercises another caller of the shared executor.
#[test]
fn headerless_example_keeps_ken_run_result_unchanged() {
    let dir = tempdir().expect("fixture root");
    let path = dir.path().join("entry.ken.md");
    let control = run(&path, &literate(PRINT_OWN_ABI, None));
    let with = run(&path, &literate(PRINT_OWN_ABI, Some(BARE_EXAMPLE)));
    assert!(control.status.success(), "valid ANone program must run");
    assert_same_run(&control, &with);
}

/// MEASURED: even when a headed example fails, the environment still
/// retains the program's ANone descriptor. CLAIMED: restoration surrounds
/// both Ok and Err exits. THE GAP: this pins the executor's error path;
/// the CLI correctly stops before admission on that same error.
#[test]
fn failed_headed_example_restores_program_boundary_before_returning_error() {
    let source = literate(
        OWN_ABI,
        Some("program capabilities FS AFull\nconst zz_bad : Nat = zz_unbound"),
    );
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_ken_md_file_v1(&source)
        .expect_err("bad checked example must fail elaboration");
    let descriptor = admit_checked_main(&env).expect("retained program authority");
    assert_eq!(descriptor.authority_name, "ANone");
}
