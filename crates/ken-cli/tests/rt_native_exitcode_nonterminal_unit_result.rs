//! RT-NATIVE-EXITCODE-NONTERMINAL-UNIT-RESULT AC-1: only the root result
//! becomes the process exit status. A generated unit that returns an
//! `ExitCode` to a caller hands back an ordinary carried constructor, and the
//! root decodes whatever it finally returns.
//!
//! Each row is its own `#[test]`. It builds one real native root, runs the
//! linked artifact, then runs the identical source through the interpreter on
//! the same root. Every payload is computed from `|argv0|`, so no row is known
//! at compile time.
//!
//! The rows are the Architect's AC-0 recut (21 programs):
//! - **Helper rows** (`H`, `J`, `K`): a recursive helper `pick` returns an
//!   `ExitCode` that `main` inspects, answering `Failure 7` on `Success` and
//!   `Failure 9` on `Failure`. `H` returns `Failure` from a tail position, `J`
//!   from inside a join in the helper, and `K` carries the code from a second
//!   recursive unit `codeof`.
//! - **Root-terminal controls** (`C`, `JC`, `KC`, `R`, `X`): the same shapes,
//!   with the helper result returned directly by `main`, the code computed in
//!   a unit and wrapped by the root (`R`), or the code produced inside a
//!   generated continuation context (`X`).
//!
//! **Promise classes.** The paired outcome is a durable invariant: native and
//! interpreter must agree on every row, each at the status the source
//! defines. The trap line is a normative compatibility vector: it is the
//! starter's fixed report for status `-3` (`object_linker_packaging.rs`).
//! Malformed rows exit 1 in every build, so only the line separates the root
//! decode of a non-immediate code (`-3`) from the ImmediateInt refusal it
//! replaced (`-1`, "malformed borrowed process input").
//!
//! MEASURED: each row's native status and stderr, and the interpreter's
//! status and terminal error. CLAIMED: a non-root unit never projects its
//! `ExitCode` result into the process status; only the root's terminal
//! result does. THE GAP: the rows cover tail, join and carried-code helper
//! results and one continuation context. Another non-root producer shape
//! is covered only by the shared `terminal_result_origins` predicate, which
//! every projecting seat reads.

#![cfg(target_os = "linux")]

const MALFORMED_FAILURE_LINE: &str = "ken native trap: malformed ExitCode::Failure payload";

/// Each row compiles and runs a whole native root on this thread. Measured on
/// these 21 rows at the candidate: all pass at a stated 2 MiB stack. The
/// 8 MiB provision applies 6 MiB of headroom. `Builder::stack_size` keeps the
/// provision local rather than ambient.
const ROW_STACK_MEASURED_PEAK_BYTES: usize = 2 * 1024 * 1024;
const ROW_STACK_HEADROOM_BYTES: usize = 6 * 1024 * 1024;
const ROW_STACK_BYTES: usize = ROW_STACK_MEASURED_PEAK_BYTES + ROW_STACK_HEADROOM_BYTES;

/// `0 - |argv0|`: negative, outside `UInt8`.
const NEG: &str = "sub_int (0 : Int) (bytes_length argv0)";
/// `2^64 + |argv0|`: a non-immediate code, outside `UInt8`.
const BIG: &str = "add_int (18446744073709551616 : Int) (bytes_length argv0)";
/// `|argv0| - (|argv0| - 3)`: the in-range code 3.
const THREE: &str = "sub_int (bytes_length argv0) (sub_int (bytes_length argv0) (3 : Int))";

/// `pick` reaches its `Failure` arm in two recursive steps.
const FAILURE_FUEL: &str = "Suc (Suc Zero)";
/// `join_helper`'s `pick` reaches its `Success` arm in one step.
const SUCCESS_FUEL: &str = "Suc Zero";

fn roots() -> [std::path::PathBuf; 1] {
    [std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")]
}

fn tail_helper(payload: &str) -> String {
    format!(
        "fn pick (fuel : Nat) (argv0 : Bytes) : ExitCode = match fuel {{
  Zero |-> ExitCode.Failure (int_to_uint8_raw ({payload}));
  Suc n |-> pick n argv0
}}
"
    )
}

fn join_helper(payload: &str) -> String {
    format!(
        "fn pick (fuel : Nat) (argv0 : Bytes) : ExitCode = match fuel {{
  Zero |-> ExitCode.Failure (int_to_uint8_raw ({payload}));
  Suc n |-> match n {{
    Zero |-> ExitCode.Success;
    Suc m |-> pick m argv0
  }}
}}
"
    )
}

fn codeof(payload: &str) -> String {
    format!(
        "fn codeof (fuel : Nat) (argv0 : Bytes) : UInt8 = match fuel {{
  Zero |-> int_to_uint8_raw ({payload});
  Suc n |-> codeof n argv0
}}
"
    )
}

fn carried_code_helper(payload: &str) -> String {
    format!(
        "{}fn pick (fuel : Nat) (argv0 : Bytes) : ExitCode = match fuel {{
  Zero |-> ExitCode.Failure (codeof (Suc Zero) argv0);
  Suc n |-> pick n argv0
}}
",
        codeof(payload)
    )
}

fn inspect(fuel: &str) -> String {
    format!(
        "match pick ({fuel}) argv0 {{
        ExitCode.Success |-> host_pure AFull ExitCode (ExitCode.Failure 7);
        ExitCode.Failure code |-> host_pure AFull ExitCode (ExitCode.Failure 9)
      }}"
    )
}

fn direct(fuel: &str) -> String {
    format!("host_pure AFull ExitCode (pick ({fuel}) argv0)")
}

/// `codeof` beside a `pick` that the root never calls, so the `R` rows differ
/// from the `KC` rows only in where `Failure` wraps the carried code.
fn root_carried_code_units(payload: &str) -> String {
    format!(
        "{}fn pick (fuel : Nat) (argv0 : Bytes) : ExitCode = ExitCode.Success
",
        codeof(payload)
    )
}

fn root_carried_code() -> String {
    "host_pure AFull ExitCode (ExitCode.Failure (codeof (Suc Zero) argv0))".to_string()
}

fn program(units: String, cons_arm: String) -> String {
    format!(
        "program capabilities FS AFull
import Capability.Process.Exit (host_pure)
{units}
proc main (input : ProcessInput) (_caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match input {{
    MkProcessInput arguments _environment _cwd |-> match arguments {{
      Nil |-> host_pure AFull ExitCode ExitCode.Success;
      Cons argv0 _rest |-> {cons_arm}
    }}
  }}
"
    )
}

/// The `Failure` is produced inside the generated continuation context of a
/// `host_bind`, after one console effect.
fn context_program(payload: &str) -> String {
    format!(
        "program capabilities FS APartial
import Capability.Console.Text (print_line)
import Capability.Process.Exit (host_bind, host_console, host_pure)
proc main (input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  match input {{
    MkProcessInput arguments _environment _cwd |-> match arguments {{
      Nil |-> host_pure APartial ExitCode ExitCode.Success;
      Cons argv0 _rest |-> host_bind APartial Unit ExitCode
        (host_console APartial Unit (print_line \"seed\"))
        (\\_. host_pure APartial ExitCode (ExitCode.Failure (int_to_uint8_raw ({payload}))))
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

fn run_row(label: &'static str, source: String) -> Row {
    std::thread::Builder::new()
        .name(label.to_string())
        .stack_size(ROW_STACK_BYTES)
        .spawn(move || {
            let root = tempfile::Builder::new()
                .prefix(&format!("ken-rt-exitcode-nt-{label}-"))
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

/// Both lanes exit with `status`; native prints nothing on stderr and the
/// interpreter reports no terminal error.
fn assert_status(label: &'static str, source: String, status: i32) {
    let row = run_row(label, source);
    assert_eq!(
        row.native_status,
        Some(status),
        "{label}: native status; stderr {:?}",
        row.native_stderr
    );
    assert_eq!(row.native_stderr, "", "{label}: native stderr");
    assert_eq!(row.interpreted.exit_status, status, "{label}: interpreter status");
    assert_eq!(row.interpreted.terminal_error, None, "{label}: interpreter terminal error");
}

/// Native exits 1 with exactly the `-3` trap line; the interpreter reports
/// `MalformedHostAbiField`.
fn assert_malformed(label: &'static str, source: String) {
    let row = run_row(label, source);
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

// Helper rows: main inspects the helper's ExitCode.

#[test]
fn h3_tail_helper_failure_inspected_by_main_exits_nine() {
    assert_status("H3", program(tail_helper(THREE), inspect(FAILURE_FUEL)), 9);
}

#[test]
fn hneg_tail_helper_negative_failure_inspected_by_main_exits_nine() {
    assert_status("Hneg", program(tail_helper(NEG), inspect(FAILURE_FUEL)), 9);
}

#[test]
fn h264_tail_helper_non_immediate_failure_inspected_by_main_exits_nine() {
    assert_status("H264", program(tail_helper(BIG), inspect(FAILURE_FUEL)), 9);
}

#[test]
fn j3_join_helper_failure_inspected_by_main_exits_nine() {
    assert_status("J3", program(join_helper(THREE), inspect(FAILURE_FUEL)), 9);
}

#[test]
fn jneg_join_helper_negative_failure_inspected_by_main_exits_nine() {
    assert_status("Jneg", program(join_helper(NEG), inspect(FAILURE_FUEL)), 9);
}

#[test]
fn j_success_join_helper_success_inspected_by_main_exits_seven() {
    assert_status("J-success", program(join_helper(THREE), inspect(SUCCESS_FUEL)), 7);
}

#[test]
fn k3_carried_code_helper_failure_inspected_by_main_exits_nine() {
    assert_status("K3", program(carried_code_helper(THREE), inspect(FAILURE_FUEL)), 9);
}

#[test]
fn kneg_carried_code_helper_negative_failure_inspected_by_main_exits_nine() {
    assert_status("Kneg", program(carried_code_helper(NEG), inspect(FAILURE_FUEL)), 9);
}

// Root-terminal controls: the root's own result is the process answer.

#[test]
fn c3_tail_helper_result_returned_by_main_exits_three() {
    assert_status("C3", program(tail_helper(THREE), direct(FAILURE_FUEL)), 3);
}

#[test]
fn cneg_tail_helper_negative_result_returned_by_main_traps_malformed() {
    assert_malformed("Cneg", program(tail_helper(NEG), direct(FAILURE_FUEL)));
}

#[test]
fn c264_tail_helper_non_immediate_result_returned_by_main_traps_malformed() {
    assert_malformed("C264", program(tail_helper(BIG), direct(FAILURE_FUEL)));
}

#[test]
fn jc3_join_helper_result_returned_by_main_exits_three() {
    assert_status("JC3", program(join_helper(THREE), direct(FAILURE_FUEL)), 3);
}

#[test]
fn jcneg_join_helper_negative_result_returned_by_main_traps_malformed() {
    assert_malformed("JCneg", program(join_helper(NEG), direct(FAILURE_FUEL)));
}

#[test]
fn kc3_carried_code_helper_result_returned_by_main_exits_three() {
    assert_status("KC3", program(carried_code_helper(THREE), direct(FAILURE_FUEL)), 3);
}

#[test]
fn kcneg_carried_code_helper_negative_result_returned_by_main_traps_malformed() {
    assert_malformed("KCneg", program(carried_code_helper(NEG), direct(FAILURE_FUEL)));
}

#[test]
fn kc264_carried_code_helper_non_immediate_result_returned_by_main_traps_malformed() {
    assert_malformed("KC264", program(carried_code_helper(BIG), direct(FAILURE_FUEL)));
}

#[test]
fn r3_root_failure_of_unit_computed_code_exits_three() {
    assert_status("R3", program(root_carried_code_units(THREE), root_carried_code()), 3);
}

#[test]
fn rneg_root_failure_of_unit_computed_negative_code_traps_malformed() {
    assert_malformed("Rneg", program(root_carried_code_units(NEG), root_carried_code()));
}

#[test]
fn x3_context_failure_exits_three() {
    assert_status("X3", context_program(THREE), 3);
}

#[test]
fn xneg_context_negative_failure_traps_malformed() {
    assert_malformed("Xneg", context_program(NEG));
}

#[test]
fn x264_context_non_immediate_failure_traps_malformed() {
    assert_malformed("X264", context_program(BIG));
}
