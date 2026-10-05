//! Real checked-bound-process scalar-admission feedback gate.
//!
//! The source is shared byte-for-byte with the ignored native/interpreter
//! parity row. This test checks compiler routing only; its later refusal is a
//! separate successor and does not establish native execution or parity.

#[cfg(target_os = "linux")]
#[test]
fn bound_process_nat_retries_only_the_refused_join() {
    std::thread::Builder::new()
        .name("rt-scalar-feedback-nat".to_owned())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            let root = tempfile::tempdir().expect("isolated checked program root");
            std::fs::write(root.path().join("held.bin"), b"held resource").expect("fixture file");
            let (build, observations) = ken_runtime::with_scalar_join_feedback_attempts(|| {
                ken_cli::build_native_program(
                    include_str!("rt_nat_fanout_escaped_resource.ken"),
                    ken_cli::SourceFormat::Ken,
                    "rt_escape_nat_fanout_escaped",
                    root.path(),
                    ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                )
            });
            // Promise: transition sentinel for this checked Nat source origin.
            // MEASURED: the bound-process object compiler reports its exact
            // attempt count and refused/forced origin identities. CLAIMED:
            // only the rejected scalar join enters a fresh carrier plan.
            // GAP: this is compiler preparation, not executable parity.
            assert_eq!(observations.len(), 1, "exactly one bound-process compile");
            let record = &observations[0];
            assert_eq!(record.attempts, 2, "one refusal and one re-plan");
            assert_eq!(record.refused_origins, [1289]);
            assert_eq!(record.forced_origins, [1289].into_iter().collect());
            if let Err(error) = build {
                assert!(
                    !format!("{error:?}")
                        .contains("Match: dynamic arms must produce scalar Int or Bool values"),
                    "the formerly refused merge must now use its carrier plan"
                );
            }
        })
        .expect("Nat test-local 256 MiB compiler thread")
        .join()
        .expect("Nat diagnostic thread did not abort");
}
