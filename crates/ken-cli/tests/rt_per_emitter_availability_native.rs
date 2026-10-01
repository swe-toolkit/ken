//! Planner-only per-emitter capture census on checked native source witnesses.
//! Promise class: transition sentinel. These fixture-specific capture counts
//! must be reviewed if specialization planning legitimately grows its inputs.

#[cfg(target_os = "linux")]
const ONE: &str = include_str!("rt_per_emitter_one_bracket.ken");
#[cfg(target_os = "linux")]
const TWO: &str = include_str!("rt_ignored_two_buffer_witness.ken");

#[cfg(target_os = "linux")]
#[test]
fn one_and_two_brackets_keep_their_native_refusal_and_exact_emitter_census() {
    // The two-bracket predecessor measured a 2 MiB worker stack overflow;
    // provision its measured 4 MiB baseline plus 4 MiB of headroom.
    std::thread::Builder::new()
        .name("rt-per-emitter-brackets".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            let compile = |label: &str, source: &str| {
                let root = tempfile::Builder::new()
                    .prefix("ken-rt-per-emitter-")
                    .tempdir()
                    .expect("unique native output root");
                let (built, reports) = ken_runtime::with_per_emitter_availability_diagnostics(|| {
                    ken_cli::build_native_program(
                        source,
                        ken_cli::SourceFormat::Ken,
                        label,
                        root.path(),
                        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                    )
                });
                assert_eq!(reports.len(), 1, "one checked source has one static plan");
                if label == "rt_per_emitter_one" {
                    let compiled = built.expect("the one-bracket c91 control builds natively");
                    let process = std::process::Command::new(&compiled.artifact.executable_path)
                        .current_dir(root.path()).output().expect("run one bracket");
                    assert_eq!(process.stdout, b"");
                    assert_eq!(process.stderr, b"");
                    assert_eq!(process.status.code(), Some(0));
                } else {
                    let error = built.expect_err("two brackets still need the held residual WP");
                    let ken_elaborator::compiler_driver::NativeProgramBuildError::Packaging(error) = error else {
                        panic!("the planner may not newly refuse the witness: {error:?}");
                    };
                    assert_eq!(error.stage, ken_runtime::ObjectLinkerPackagingStage::ObjectEmission);
                    assert_eq!(error.field, "checked_process_object");
                    assert_eq!(error.reason,
                        "unsupported runtime-IR lowering: BoundaryCarrier: a carried recursive hypothesis is an eliminated value, not a callable, so it takes no arguments, but the call provides 1");
                }
                reports.into_iter().next().unwrap()
            };
            let one = compile("rt_per_emitter_one", ONE);
            assert_eq!(one.materializations.len(), 1);
            assert_eq!(one.unclassified.len(), 1);
            assert_eq!(one.materializations[0].kind, "ConstructEmission");
            assert!(one.materializations[0].owner.starts_with("Predeclared("));
            assert_eq!(capture_counts(&one.materializations[0]), (5, 4, 9, 0));

            let two = compile("rt_per_emitter_two", TWO);
            assert_eq!(two.materializations.len(), 2);
            assert_eq!(two.unclassified.len(), 2);
            let counts = two.materializations.iter().map(|point| {
                assert_eq!(point.kind, "ConstructEmission");
                assert!(point.owner.starts_with("Predeclared("));
                capture_counts(point)
            }).collect::<std::collections::BTreeSet<_>>();
            assert_eq!(counts, [(5, 4, 9, 0), (7, 6, 13, 0)].into());
        })
        .expect("spawn stated-stack bracket witnesses")
        .join()
        .expect("bracket witnesses thread");
}

#[cfg(target_os = "linux")]
fn capture_counts(
    point: &ken_runtime::PerEmitterMaterializationDiagnostic,
) -> (usize, usize, usize, usize) {
    let w = point
        .captures
        .iter()
        .filter(|capture| capture.run == "Worker")
        .count();
    let c = point
        .captures
        .iter()
        .filter(|capture| capture.run == "Context")
        .count();
    let finalized = point
        .captures
        .iter()
        .filter(|capture| capture.result.starts_with("Finalized("))
        .count();
    let unfinalizable = point.captures.len() - finalized;
    assert_eq!(w + c, finalized + unfinalizable);
    (w, c, finalized, unfinalizable)
}
