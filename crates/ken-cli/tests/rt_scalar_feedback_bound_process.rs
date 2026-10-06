//! Real checked-bound-process scalar-admission feedback gate.
//!
//! The source is shared byte-for-byte with the active native/interpreter
//! parity row. This test checks compiler feedback and successful preparation;
//! the separate parity row establishes native execution on the stated stack.

#[cfg(target_os = "linux")]
#[test]
fn bound_process_nat_retries_only_newly_refused_joins() {
    // Provision the same Nat source as the now-active interpreter/native
    // parity row, not a repair to a newly failing baseline test. On exact
    // 95cb816b7, with ambient RUST_MIN_STACK unset, a 20 Hz /proc/<pid>/smaps
    // sample of this worker's 256 MiB VMA observed 3,936 KiB resident stack
    // high-water: 262,144 KiB = 3,936 KiB observed + 258,208 KiB headroom.
    // This sampled residency is not an exact deepest-stack measurement; a
    // scratch 4 MiB Builder variant passed, but the ruled stated stack remains
    // 256 MiB. This test makes no default-stack adequacy claim.
    // Measurement: /tmp/qa-natfanout-95cb-* (789 no-logger observations).
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
            // Promise class: durable feedback-loop invariant on this checked
            // Nat source. MEASURED at 74daf7a: refused [1289, 1121, 947]
            // over four attempts; those numbers describe a run, not the pin.
            // CLAIMED: a successful build retries once per *new* refused join,
            // forces exactly those joins, then stops. THE GAP: this fixture
            // does not bound every program's number of static join origins;
            // the independent active differential pins native parity.
            assert_eq!(observations.len(), 1, "exactly one bound-process compile");
            let record = &observations[0];
            build.expect("the Nat fanout source compiles through scalar-join feedback");
            // The loop retries only when forced.insert(origin) is new, and the
            // successful attempt pushes no refusal: one retry per distinct
            // refusal, no extra forced join, then one successful attempt.
            let refused: std::collections::BTreeSet<u32> =
                record.refused_origins.iter().copied().collect();
            assert_eq!(refused.len(), record.refused_origins.len(), "no origin is refused twice");
            assert_eq!(record.forced_origins, refused, "only refused joins are forced to carriers");
            assert_eq!(record.attempts, record.refused_origins.len() + 1,
                "one retry per refusal, then success");
        })
        .expect("Nat test-local 256 MiB compiler thread")
        .join()
        .expect("Nat diagnostic thread did not abort");
}
