#![cfg(target_os = "linux")]

use ken_host::{ActivationRegionEndV1, RegionCountsV1};

const F1: &str = include_str!("fixtures/rt_arena_write_f1.ken");
const F2: &str = include_str!("fixtures/rt_arena_write_f2.ken");
const VF1: &str = include_str!("fixtures/rt_arena_write_vf1.ken");
const VF2: &str = include_str!("fixtures/rt_arena_write_vf2.ken");

fn catalog_roots() -> [std::path::PathBuf; 1] {
    [std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")]
}

fn run_source(
    source: &str,
    name: &str,
    writes_expected: usize,
    profile_multiple: usize,
    payload_bytes_expected: u64,
) -> ActivationRegionEndV1 {
    let source = source.to_owned();
    let name = name.to_owned();
    // The landed Inc2 W1 worker measured 2736 KiB resident on a 64-MiB
    // probe; 16 MiB gives 13648 KiB headroom. This is a stated baseline
    // provision, not a stack-depth pin or a changed regression threshold.
    std::thread::Builder::new()
        .name(format!("arena-cost-{name}"))
        .stack_size(16 * 1024 * 1024)
        .spawn(move || {
            let root = tempfile::tempdir().expect("isolated Process capability root");
            let mut profile = ken_runtime::boundary_resource_profile::starter_smoke_profile();
            for region in [&mut profile.invocation, &mut profile.persistent] {
                region.nodes *= profile_multiple;
                region.words *= profile_multiple;
                region.data_bytes *= profile_multiple;
                region.native_int_limbs *= profile_multiple;
            }
            let built = ken_cli::build_native_program_with_roots(
                &source,
                ken_cli::SourceFormat::Ken,
                &catalog_roots(),
                &format!("rt_arena_cost_{name}"),
                root.path().join("native"),
                profile,
            )
            .expect("rooted Strict checked Process native build");
            let (observation, region_end) =
                ken_runtime::run_bound_process_effect_observation_with_region_end(
                    &built.artifact,
                    &ken_runtime::NativeEffectRunOptionsV1 {
                        arguments: Vec::new(),
                        environment: Vec::new(),
                        cwd: root.path().to_owned(),
                        plan_hash: built.plan_transport_hash,
                    },
                    &[],
                )
                .expect("linked Process observation including region end");
            assert_eq!(observation.exit_status, 0, "{name}: {observation:?}");
            assert_eq!(observation.terminal_error, None, "{name}: {observation:?}");
            assert_eq!(
                observation.terminal_exit,
                ken_host::TerminalExitClass::NormalReturn,
                "{name}: {observation:?}"
            );
            let reads = observation
                .effect_trace
                .iter()
                .filter(|event| event.operation == ken_host::HostOpV1::ResourceStreamRead)
                .collect::<Vec<_>>();
            assert_eq!(reads.len(), 1, "{name}: {observation:?}");
            let ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::ReadProgress(
                ken_host::ReadProgressV1::ReadSome { transferred, .. },
            )) = &reads[0].outcome
            else {
                panic!("{name}: expected a successful stream read: {reads:?}");
            };
            assert_eq!(
                transferred.get(),
                payload_bytes_expected,
                "{name}: read payload length"
            );

            let writes = observation
                .effect_trace
                .iter()
                .filter(|event| event.operation == ken_host::HostOpV1::ResourceStreamWrite)
                .collect::<Vec<_>>();
            assert_eq!(writes.len(), writes_expected, "{name}: {observation:?}");
            for (index, write) in writes.into_iter().enumerate() {
                let ken_host::CanonicalOutcomeV1::Success(
                    ken_host::CanonicalReplyV1::WriteProgress(ken_host::WriteProgressV1::Wrote(
                        transferred,
                    )),
                ) = &write.outcome
                else {
                    panic!("{name} write {index} did not succeed: {write:?}");
                };
                assert_eq!(
                    transferred.get(),
                    payload_bytes_expected,
                    "{name} write {index} payload length"
                );
            }
            let end = region_end.expect("ordinary post-entry finish records both region ends");
            eprintln!("ARENA_COST {name} {end:?}");
            end
        })
        .expect("stated arena-cost worker")
        .join()
        .expect("arena-cost worker completes")
}

fn subtract_region(higher: RegionCountsV1, lower: RegionCountsV1) -> RegionCountsV1 {
    RegionCountsV1 {
        nodes: higher
            .nodes
            .checked_sub(lower.nodes)
            .expect("node count grows"),
        words: higher
            .words
            .checked_sub(lower.words)
            .expect("word count grows"),
        data_bytes: higher
            .data_bytes
            .checked_sub(lower.data_bytes)
            .expect("data count grows"),
        native_int_limbs: higher
            .native_int_limbs
            .checked_sub(lower.native_int_limbs)
            .expect("limb count grows"),
    }
}

fn delta(higher: ActivationRegionEndV1, lower: ActivationRegionEndV1) -> ActivationRegionEndV1 {
    ActivationRegionEndV1 {
        invocation: subtract_region(higher.invocation, lower.invocation),
        persistent: subtract_region(higher.persistent, lower.persistent),
    }
}

/// Promise: transition sentinel for the first chain-level write cost of this
/// lowering. `spec/40-runtime/44-capacity.md` specifies resource profiles;
/// `spec/40-runtime/45-native-backend.md` specifies native execution. The
/// measurement is the optional native-only `region_end` from published arena
/// headers; it does not change the shared interpreter effect observation.
/// CLAIMED: D1 is the first chain level's per-write cost on the current lowering. Its invocation words include the capture of dead binders at that depth and R's inline copy of its environment. It is measured from the region_end header counts.
/// GAP: The per-write invocation word cost grows by 6 words per earlier chain level, because every later capture record retains each earlier write's HostResult and its two pattern-bound reply values (defect node below). D2 and D3 are not pinned. Fixing capture policy or R's layout lowers D1, and that re-rules the literal. Such a change is not a write regression.
/// The pinned literal is the fixed F1/F2 fixture's measured baseline, not a
/// general rule for all payloads, write shapes, or future closure layouts.
#[test]
fn ac1_first_chain_level_write_d1_and_controls() {
    let f1 = run_source(F1, "F1", 1, 4, 7);
    let f2 = run_source(F2, "F2", 2, 4, 7);
    let vf1 = run_source(VF1, "VF1", 1, 4, 3);
    let vf2 = run_source(VF2, "VF2", 2, 4, 3);
    let f1_eightfold = run_source(F1, "F1_8x", 1, 8, 7);
    eprintln!("ARENA_COST F1={f1:?}; F2={f2:?}; VF1={vf1:?}; VF2={vf2:?}; F1_8x={f1_eightfold:?}");

    let d1 = delta(f2, f1);
    let payload_d1 = delta(vf2, vf1);
    let pinned = ActivationRegionEndV1 {
        invocation: RegionCountsV1 {
            nodes: 6,
            words: 103,
            data_bytes: 0,
            native_int_limbs: 0,
        },
        persistent: RegionCountsV1 {
            nodes: 2,
            words: 3,
            data_bytes: 0,
            native_int_limbs: 0,
        },
    };
    eprintln!("ARENA_COST D1={d1:?}; payload_D1={payload_d1:?}; pinned={pinned:?}");
    assert_eq!(d1, pinned, "first chain-level write cost changed");
    assert_eq!(payload_d1, pinned, "payload-variant D1 cost changed");
    assert_eq!(f1, f1_eightfold, "profile limits do not affect end counts");
}

/// Promise: durable control for AC-2. One write still succeeds under fourfold
/// limits when an extra zero-field invocation node is allocated per write:
/// the D1 differential above, not the ceiling, detects the mutation.
#[test]
fn ac2_one_write_survives_fourfold_capacity_ceiling() {
    let _ = run_source(F1, "F1_level_control", 1, 4, 7);
}
