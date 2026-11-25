#![cfg(target_os = "linux")]

const SOURCE: &str = r#"program capabilities FS APartial
import Capability.Console.Text (print_line)
import Capability.Process.Exit (host_bind, host_console, host_pure)
import Data.Numeric.FixedWidth (intToUInt64)

proc decide (length : Int) : HostIO APartial (Result Bool UInt8) visits [Console] =
  GUARD

proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_bind APartial (Result Bool UInt8) ExitCode (decide INPUT_LENGTH)
    (\outcome. match outcome {
      Err False |-> host_pure APartial ExitCode (ExitCode.Failure 23);
      Err True |-> host_pure APartial ExitCode (ExitCode.Failure 24);
      Ok _byte |-> host_pure APartial ExitCode (ExitCode.Failure 25)
    })
"#;

const MATCH: &str = r#"match intToUInt64 length {
    None |-> host_pure APartial (Result Bool UInt8) (Err Bool UInt8 False);
    Some _bounded |-> host_bind APartial Unit (Result Bool UInt8)
      (host_console APartial Unit (print_line "unselected"))
      (\_unit. host_pure APartial (Result Bool UInt8) (Ok Bool UInt8 7))
  }"#;

const IF: &str = r#"if (leq_int length 255) then
    host_bind APartial Unit (Result Bool UInt8)
      (host_console APartial Unit (print_line "unselected"))
      (\_unit. host_pure APartial (Result Bool UInt8) (Ok Bool UInt8 7))
  else
    host_pure APartial (Result Bool UInt8) (Err Bool UInt8 False)"#;

// Durable invariant. MEASURED: identical complete observations of exit 23
// and no console visit. CLAIMED: compile-time exclusion of the Vis preserves
// the selected Ret. THE GAP: parity alone does not identify the disposition
// seat; the base refusal and consumer mutations distinguish that mechanism.
#[test]
fn static_match_ret_skips_console_vis_in_both_lanes() {
    assert_ret_skips_console(MATCH, "18446744073709551616");
}

#[test]
fn static_if_ret_skips_console_vis_in_both_lanes() {
    assert_ret_skips_console(IF, "256");
}

fn assert_ret_skips_console(guard: &str, length: &str) {
    let source = SOURCE.replace("GUARD", guard).replace("INPUT_LENGTH", length);
    let roots = [std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")];
    let dir = tempfile::tempdir().unwrap();
    let mut host = ken_interp::PosixHost::new_at(dir.path());
    let interpreted = ken_cli::run_program_effect_observation(
        &source,
        ken_cli::SourceFormat::Ken,
        &roots,
        &[b"ken".to_vec()],
        &[],
        dir.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .expect("checked HostIO guard runs in the interpreter");
    assert_eq!(interpreted.exit_status, 23);
    assert!(interpreted.effect_trace.is_empty());
    let artifact = ken_cli::build_native_program_with_roots(
        &source,
        ken_cli::SourceFormat::Ken,
        &roots,
        "rt-static-unselected-call",
        dir.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .expect("checked HostIO guard emits a native artifact");
    let native = ken_runtime::run_bound_process_effect_observation(
        &artifact.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: dir.path().to_owned(),
            plan_hash: artifact.plan_transport_hash,
        },
    )
    .expect("checked HostIO guard runs natively");
    assert_eq!(native, interpreted);
}
