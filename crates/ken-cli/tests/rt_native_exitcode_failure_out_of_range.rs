//! RT-NATIVE-EXITCODE-FAILURE-OUT-OF-RANGE AC-1: a run-time `ExitCode.Failure`
//! payload outside `UInt8` traps natively the way the interpreter reports it.
//!
//! Each row is its own `#[test]` and builds one real native root, runs the
//! linked artifact, then runs the identical source through the interpreter on
//! the same root. The four rows differ only in the payload expression, which
//! is computed from `|argv0|` so no row is known at compile time.
//!
//! **Promise classes.** The paired status and interpreter outcome are a
//! durable invariant: native and interpreter must agree on every row. The
//! trap line is a normative compatibility vector: it is the starter's fixed
//! report for status `-3` (`object_linker_packaging.rs`), and the negative and
//! `2^64` rows exit 1 in every build, so only the line separates the repair
//! from main. The `0` and `3` rows are the controls that separate the repair
//! from a decoder that always traps.

#![cfg(target_os = "linux")]

const MALFORMED_FAILURE_LINE: &str = "ken native trap: malformed ExitCode::Failure payload";

/// Each row compiles and runs a whole native root on this thread. Measured on
/// these four rows at the candidate: all pass at a stated 2 MiB stack. The
/// 8 MiB provision applies 6 MiB of headroom. `Builder::stack_size` keeps the
/// provision local rather than ambient.
const ROW_STACK_MEASURED_PEAK_BYTES: usize = 2 * 1024 * 1024;
const ROW_STACK_HEADROOM_BYTES: usize = 6 * 1024 * 1024;
const ROW_STACK_BYTES: usize = ROW_STACK_MEASURED_PEAK_BYTES + ROW_STACK_HEADROOM_BYTES;

fn roots() -> [std::path::PathBuf; 1] {
    [std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")]
}

fn program(payload: &str) -> String {
    format!(
        "program capabilities FS AFull
import Capability.Process.Exit (host_pure)
proc main (input : ProcessInput) (_caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match input {{
    MkProcessInput arguments _environment _cwd |-> match arguments {{
      Nil |-> host_pure AFull ExitCode ExitCode.Success;
      Cons argv0 _rest |-> host_pure AFull ExitCode (ExitCode.Failure (int_to_uint8_raw ({payload})))
    }}
  }}
"
    )
}

struct Row {
    native_status: Option<i32>,
    native_stderr: String,
    interpreted: ken_runtime::EffectObservation,
}

fn run_row(label: &'static str, payload: &'static str) -> Row {
    std::thread::Builder::new()
        .name(label.to_string())
        .stack_size(ROW_STACK_BYTES)
        .spawn(move || {
            let source = program(payload);
            let root = tempfile::Builder::new()
                .prefix(&format!("ken-rt-exitcode-{label}-"))
                .tempdir()
                .unwrap();
            let compiled = ken_cli::build_native_program_with_roots(
                &source,
                ken_cli::SourceFormat::Ken,
                &roots(),
                "native-program",
                root.path(),
                ken_runtime::boundary_resource_profile::starter_smoke_profile(),
            )
            .unwrap_or_else(|error| panic!("{label}: native build: {error:?}"));
            let process = std::process::Command::new(&compiled.artifact.executable_path)
                .current_dir(root.path())
                .output()
                .unwrap_or_else(|error| panic!("{label}: native run: {error:?}"));
            let mut host = ken_interp::PosixHost::new_at(root.path());
            let interpreted = ken_cli::run_program_effect_observation(
                &source,
                ken_cli::SourceFormat::Ken,
                &roots(),
                &[b"native-program".to_vec()],
                &[],
                root.path().as_os_str().as_encoded_bytes(),
                &mut host,
            )
            .unwrap_or_else(|error| panic!("{label}: interpreter run: {error:?}"));
            Row {
                native_status: process.status.code(),
                native_stderr: String::from_utf8_lossy(&process.stderr).into_owned(),
                interpreted,
            }
        })
        .expect("spawn stated-stack exit row")
        .join()
        .expect("exit row thread")
}

fn assert_malformed(label: &'static str, payload: &'static str) {
    let row = run_row(label, payload);
    assert_eq!(row.native_status, Some(1), "{label}: native status");
    assert_eq!(
        row.native_stderr.lines().collect::<Vec<_>>(),
        [MALFORMED_FAILURE_LINE],
        "{label}: native stderr"
    );
    assert_eq!(
        row.interpreted.terminal_error,
        Some(ken_runtime::TerminalErrorV1::MalformedHostAbiField),
        "{label}: interpreter terminal error"
    );
}

fn assert_in_range(label: &'static str, payload: &'static str, status: i32) {
    let row = run_row(label, payload);
    assert_eq!(row.native_status, Some(status), "{label}: native status");
    assert_eq!(row.native_stderr, "", "{label}: native stderr");
    assert_eq!(row.interpreted.exit_status, status, "{label}: interpreter status");
    assert_eq!(row.interpreted.terminal_error, None, "{label}: interpreter terminal error");
}

#[test]
fn negative_runtime_failure_payload_traps_as_malformed() {
    assert_malformed("negative", "sub_int (0 : Int) (bytes_length argv0)");
}

#[test]
fn two_pow_64_runtime_failure_payload_traps_as_malformed() {
    assert_malformed("two-pow-64", "add_int (18446744073709551616 : Int) (bytes_length argv0)");
}

#[test]
fn zero_runtime_failure_payload_exits_one() {
    assert_in_range("zero", "sub_int (bytes_length argv0) (bytes_length argv0)", 1);
}

#[test]
fn three_runtime_failure_payload_exits_three() {
    assert_in_range(
        "three",
        "sub_int (bytes_length argv0) (sub_int (bytes_length argv0) (3 : Int))",
        3,
    );
}
