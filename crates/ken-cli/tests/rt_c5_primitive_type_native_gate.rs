//! RT-C5-PRIMITIVE-TYPE-ARGUMENTS: the interpreter is the C5 oracle; the
//! native backend refuses function-valued indexed motives before emission.

use ken_elaborator::checked_core::{CheckedCoreBodyViewError, StableSymbol};
use ken_elaborator::compiler_driver::{CompilerDriverError, NativeProgramBuildError};

const INDEXED_INT_CONVOY: &str = r#"program capabilities FS APartial
import Capability.Process.Exit (host_pure)

data Vec (A : Type) : Nat -> Type where {
  VNil : Vec A 0;
  VCons : (n : Nat) -> A -> Vec A n -> Vec A (n+1)
}

fn zipPrimitive (n : Nat) (v : Vec Int n) (w : Vec Int n) : Vec Int n =
  match v {
    VNil |-> VNil Int;
    VCons m a xs |-> match w {
      VCons _ b ys |-> VCons Int m a (zipPrimitive m xs ys)
    }
  }

const checkResult : Bool =
  match zipPrimitive (Suc Zero)
    (VCons Int Zero 7 (VNil Int))
    (VCons Int Zero 8 (VNil Int)) {
    VCons _ head _ |-> eq_int head 7
  }

fn main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode =
  match checkResult {
    True |-> host_pure APartial ExitCode ExitCode.Success;
    False |-> host_pure APartial ExitCode (ExitCode.Failure 1)
  }
"#;

const INDEXED_STRING_CONVOY: &str = include_str!("fixtures/rt_indexed_family/zip_string.ken");
const SUM_OCTETS: &str = include_str!("fixtures/rt_indexed_family/sum_octets.ken");

fn catalog_roots() -> [std::path::PathBuf; 1] {
    [std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")]
}

fn run_reference(source: &str) -> (i32, Vec<u8>) {
    let dir = tempfile::tempdir().expect("isolated interpreter directory");
    let mut host = ken_interp::CaptureHost::new(Vec::new());
    let outcome = ken_cli::run_program(
        source,
        ken_cli::SourceFormat::Ken,
        &catalog_roots(),
        &[],
        &[],
        dir.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .expect("reference interpreter runs the same checked source");
    (outcome.exit_status, host.stdout().to_vec())
}

fn assert_convoy_refusal(source: &str, stdout: &[u8]) {
    let reference = run_reference(source);
    assert_eq!(reference.0, 0);
    assert_eq!(reference.1, stdout);
    let dir = tempfile::tempdir().expect("isolated artifact directory");
    let err = ken_cli::build_native_program_with_roots(
        source,
        ken_cli::SourceFormat::Ken,
        &catalog_roots(),
        "rt-c5-indexed",
        dir.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .expect_err("a function-valued indexed motive must refuse at admission");
    assert_eq!(err.code().as_str(), "E0075", "{err}");
    match err {
        NativeProgramBuildError::Driver(CompilerDriverError::CheckedCoreBodyView {
            error, ..
        }) => {
            assert_eq!(error.lane(), "unsupported_dependent_motive");
            assert_eq!(
                error,
                CheckedCoreBodyViewError::UnsupportedDependentMotive {
                    symbol: StableSymbol::declaration("rt-c5-indexed", &[], "zipPrimitive"),
                    family: StableSymbol::declaration("rt-c5-indexed", &[], "Vec"),
                },
            );
        }
        other => panic!("expected the named indexed-motive refusal, got {other:?}"),
    }
}

/// Promise class: transition sentinel. RT-NATIVE-INDEXED-CONVOY-LOWERING
/// must replace these Int and String refusals with native rows compared
/// against `ken_cli::run_program` on the same checked sources.
#[test]
fn indexed_int_convoy_refuses_at_motive_admission() {
    assert_convoy_refusal(INDEXED_INT_CONVOY, b"");
}

#[test]
fn indexed_string_convoy_refuses_at_motive_admission() {
    assert_convoy_refusal(INDEXED_STRING_CONVOY, b"seven\n");
}

fn octet_fixture(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/rt_indexed_family")
        .join(name)
}

fn assert_native_octets(path: &std::path::Path, reference: (i32, Vec<u8>)) {
    assert_eq!(reference, (41, Vec::new()));
    let dir = tempfile::tempdir().expect("isolated artifact directory");
    let built = std::process::Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("native-build")
        .arg(path)
        .arg(dir.path())
        .arg(octet_fixture("profile.json"))
        .output()
        .expect("native compiler process runs");
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    let native = std::process::Command::new(dir.path().join("ken-starter"))
        .output()
        .expect("native executable runs");
    assert_eq!(native.status.code(), Some(reference.0), "{native:?}");
    assert_eq!(native.stdout, reference.1);
    assert!(native.stderr.is_empty(), "{native:?}");
}

#[test]
fn recursive_indexed_octet_sum_matches_interpreter() {
    assert_native_octets(&octet_fixture("sum_octets.ken"), run_reference(SUM_OCTETS));
}

#[test]
fn fixed_length_indexed_octet_matches_match_interpreter() {
    // Use ordinary CLI processes for the 16 nested refinements, which
    // overflow a default libtest stack in both library lanes.
    let path = octet_fixture("end_octets.ken");
    let reference = std::process::Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("run")
        .arg(&path)
        .output()
        .expect("reference interpreter process runs");
    assert!(reference.stderr.is_empty(), "{reference:?}");
    assert_native_octets(
        &path,
        (
            reference.status.code().expect("interpreter exit"),
            reference.stdout,
        ),
    );
}
