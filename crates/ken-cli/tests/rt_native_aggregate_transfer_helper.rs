//! RT-NATIVE-AGGREGATE-DATA-NOT-CODE AC-2: run-time-selected host results
//! transferred through out-of-line transfer helpers keep their behavior.
//!
//! The root is `fixtures/rt_escape_buffer_then_readat.ken`, a verbatim copy of
//! `ESCAPE_BUFFER_THEN_READAT` in `rt_escape_second_resource_native.rs`. Its
//! resource-bracket outcomes are HostResult transfers, which the native build
//! emits through helpers. The root is built once, under the transfer-helper
//! observation, and the linked process and the interpreter each run it twice
//! on the same source and arguments. The runs differ only in the host state:
//! with `held.bin` present the bracket returns `Ok` and the root exits 0; with
//! it absent `FsOpen` fails with a `FileError` whose cause is
//! `Io(NotFound)`, and the root exits 81.
//!
//! **Promise classes.** That native and interpreter agree on each outcome
//! (exit status, terminal error and effect trace), and that the two outcomes
//! give different exit codes, is a durable invariant. The codes 0 and 81 are
//! the fixture's own arms. That the build defines at least one transfer
//! helper is the reach witness: forcing the inline path (AC-3) leaves the
//! behavior unchanged and makes that assertion fail.

#![cfg(target_os = "linux")]

const ROOT: &str = include_str!("fixtures/rt_escape_buffer_then_readat.ken");

/// The native build and both executors run on this thread. Measured on this
/// test at the candidate: it passes at a stated 2 MiB stack. The 8 MiB
/// provision applies 6 MiB of headroom. `Builder::stack_size` keeps the
/// provision local rather than ambient.
const ROW_STACK_MEASURED_PEAK_BYTES: usize = 2 * 1024 * 1024;
const ROW_STACK_HEADROOM_BYTES: usize = 6 * 1024 * 1024;
const ROW_STACK_BYTES: usize = ROW_STACK_MEASURED_PEAK_BYTES + ROW_STACK_HEADROOM_BYTES;

fn roots() -> [std::path::PathBuf; 1] {
    [std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")]
}

struct Paired {
    native: ken_runtime::EffectObservation,
    interpreted: ken_runtime::EffectObservation,
}

fn run_paired(
    compiled: &ken_elaborator::compiler_driver::NativeProgramBuildOutput,
    held: bool,
) -> Paired {
    let root = tempfile::tempdir().unwrap();
    if held {
        std::fs::write(root.path().join("held.bin"), b"held resource").unwrap();
    }
    let native = ken_runtime::run_bound_process_effect_observation(
        &compiled.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: root.path().to_owned(),
            plan_hash: compiled.plan_transport_hash,
        },
    )
    .expect("the linked artifact runs");
    let mut host = ken_interp::PosixHost::new_at(root.path());
    let interpreted = ken_cli::run_program_effect_observation(
        ROOT,
        ken_cli::SourceFormat::Ken,
        &roots(),
        &[b"native-program".to_vec()],
        &[],
        root.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .expect("the root runs in the interpreter");
    Paired {
        native,
        interpreted,
    }
}

fn assert_lanes_agree(label: &str, paired: &Paired, status: i32) {
    assert_eq!(paired.native.exit_status, status, "{label}: native status");
    assert_eq!(paired.interpreted.exit_status, status, "{label}: interpreter status");
    assert_eq!(paired.native.terminal_error, None, "{label}: native terminal error");
    assert_eq!(paired.interpreted.terminal_error, None, "{label}: interpreter terminal error");
    assert_eq!(
        paired.native.effect_trace, paired.interpreted.effect_trace,
        "{label}: effect traces"
    );
}

#[test]
fn run_time_selected_host_results_cross_transfer_helpers_in_both_outcomes() {
    std::thread::Builder::new()
        .name("rt-transfer-helper-row".to_string())
        .stack_size(ROW_STACK_BYTES)
        .spawn(|| {
            let build = tempfile::tempdir().unwrap();
            let (compiled, helpers) = ken_runtime::with_transfer_helper_observations(|| {
                ken_cli::build_native_program_with_roots(
                    ROOT,
                    ken_cli::SourceFormat::Ken,
                    &roots(),
                    "native-program",
                    build.path(),
                    ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                )
            });
            let compiled = compiled.unwrap_or_else(|error| panic!("native build: {error:?}"));
            assert!(
                !helpers.is_empty(),
                "the bracket outcomes must reach an out-of-line transfer helper"
            );

            let ok = run_paired(&compiled, true);
            assert_lanes_agree("Ok outcome", &ok, 0);
            let err = run_paired(&compiled, false);
            assert_lanes_agree("Err outcome", &err, 81);
            let [event] = err.native.effect_trace.as_slice() else {
                panic!("the Err outcome performs exactly the failed FsOpen: {:?}", err.native.effect_trace);
            };
            assert_eq!(event.operation, ken_runtime::HostOpV1::FsOpen);
            assert!(
                matches!(
                    &event.outcome,
                    ken_runtime::CanonicalOutcomeV1::Error(ken_runtime::SemanticErrorV1::File(
                        ken_host::FileErrorIdentityV1 {
                            operation: ken_runtime::HostOpV1::FsOpen,
                            cause: ken_host::FileErrorCauseV1::Io(ken_runtime::IoErrorIdentityV1::NotFound),
                            ..
                        }
                    ))
                ),
                "the FsOpen failure carries the IOError alternative NotFound: {:?}",
                event.outcome
            );
        })
        .expect("spawn stated-stack row")
        .join()
        .expect("row thread");
}
