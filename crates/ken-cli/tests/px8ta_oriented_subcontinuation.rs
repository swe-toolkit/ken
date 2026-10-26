//! PX8-TA public checked-bracket oriented-subcontinuation controls.

fn output_dir(name: &str) -> tempfile::TempDir {
    let prefix = format!("ken-px8ta-{name}-");
    tempfile::Builder::new().prefix(&prefix).tempdir().unwrap()
}

const PX8DS_SIBLING_RECURSION_PROGRAM: &str = r#"program capabilities FS APartial
proc countdown (fuel : Nat)
  : HostIO APartial (Result Int Bool) visits [Console] =
  match fuel {
    Zero |-> Ret (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp
        (fs_resp APartial) ambient_resp)
      (Result Int Bool) (Ok Int Bool True);
    Suc rest |-> bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp
        (fs_resp APartial) ambient_resp)
      Bool (Result Int Bool)
      (host_console APartial Bool (is_terminal Stdout))
      (\terminal. match terminal {
        False |-> countdown rest;
        True |-> countdown rest
      })
  }

fn after_countdown (_outcome : Result Int Bool)
  : HostIO APartial (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp
      (fs_resp APartial) ambient_resp)
    (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit)

proc after_first_countdown (_outcome : Result Int Bool)
  : HostIO APartial (ResourceBodyResult Unit Unit) visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp
      (fs_resp APartial) ambient_resp)
    (Result Int Bool) (ResourceBodyResult Unit Unit)
    (countdown (Suc Zero))
    (\outcome. after_countdown outcome)

proc selected_buffer_body (terminal : Bool)
  : HostIO APartial (ResourceBodyResult Unit Unit) visits [Console] =
  match terminal {
    False |-> after_countdown (Ok Int Bool False);
    True |-> bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp
        (fs_resp APartial) ambient_resp)
      (Result Int Bool) (ResourceBodyResult Unit Unit)
      (countdown (Suc (Suc Zero)))
      (\outcome. after_first_countdown outcome)
  }

proc buffer_body (_buffer : BufferHandle)
  : HostIO APartial (ResourceBodyResult Unit Unit) visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp
      (fs_resp APartial) ambient_resp)
    Bool (ResourceBodyResult Unit Unit)
    (host_console APartial Bool (is_terminal Stdout))
    (\terminal. selected_buffer_body terminal)

fn finish_buffer
  (outcome : Result ResourceError (ResourceBracketResult Unit Unit))
  : HostIO APartial ExitCode =
  match outcome {
    Err _ |-> host_exit APartial (Failure 91);
    Ok (ResourceBracketOk _) |-> host_exit APartial Success;
    Ok _ |-> host_exit APartial (Failure 92)
  }

proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [FS, Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp
      (fs_resp APartial) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit)) ExitCode
    (withBuffer APartial Unit Unit (2 : Int) buffer_body)
    (\outcome. finish_buffer outcome)
"#;

#[path = "support/rt_bracket_nested_source.rs"]
mod bracket_source;
#[cfg(target_os = "linux")]
use bracket_source::nested_bracket_source;

#[cfg(target_os = "linux")]
fn run_depth(depth: usize) -> (ken_runtime::EffectObservation, usize) {
    let source = nested_bracket_source(depth);
    let dir = output_dir(&format!("depth-{depth}"));
    for index in 0..depth {
        std::fs::write(
            dir.path().join(format!("held-{index}.bin")),
            format!("held resource {index}"),
        )
        .unwrap();
    }
    let output = ken_cli::build_native_program(
        &source,
        ken_cli::SourceFormat::Ken,
        &format!("px8ta-depth-{depth}"),
        dir.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .unwrap_or_else(|error| {
        panic!("depth {depth} checked nested bracket reaches native lowering: {error:?}")
    });
    let plan = output
        .runtime_program
        .erased_core
        .metadata
        .checked_core
        .metadata
        .values()
        .find(|bytes| bytes.starts_with(ken_runtime::ORIENTED_SUBCONTINUATION_PLAN_V1_HEADER))
        .and_then(|bytes| ken_runtime::OrientedSubcontinuationPlanV1::decode(bytes).ok())
        .expect("checked nested bracket transports its oriented answer plan");
    let observation = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: dir.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .expect("linked nested bracket emits its canonical observation");
    (observation, plan.frames.len())
}

/// Promise class: durable invariant. MEASURED: the actual depth-three plan's
/// repeated-producer Deferred FsOpen has the same statically bounded handler
/// as its specialized FsOpen row. CLAIMED: the marked bracket release ends
/// the Deferred suffix without swallowing the intervening acquisition.
/// THE GAP: this planner observation does not prove the linked owner executes
/// that acquisition; the separate-process native effect trace tests that.
#[cfg(target_os = "linux")]
#[test]
fn nested_bracket_repeated_producer_has_bounded_deferred_owner() {
    // Measured here on β′ with a test-only Builder stack-size override:
    // 1920 KiB overflows, 1984 KiB passes (64-KiB grid). The stated
    // 256 MiB provision leaves 260160 KiB of headroom over that bound;
    // no ambient RUST_MIN_STACK is used to select the thread stack.
    std::thread::Builder::new()
        .name("px8ta-bounded-owner".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(check_nested_bracket_repeated_producer_owner)
        .expect("spawn stated-stack depth-three planner control")
        .join()
        .expect("depth-three planner control thread");
}

#[cfg(target_os = "linux")]
fn check_nested_bracket_repeated_producer_owner() {
    let dir = output_dir("depth-3-bounded-owner");
    let (built, plans) = ken_runtime::with_static_response_feasibility_diagnostics(|| {
        ken_cli::build_native_program(
            &nested_bracket_source(3),
            ken_cli::SourceFormat::Ken,
            "px8ta-depth-3-bounded-owner",
            dir.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });
    built.expect("the checked depth-three program builds a native artifact");
    let plans: Vec<_> = plans
        .iter()
        .filter(|plan| {
            plan.all_static_response_rows.iter().any(|row| row.operation == "FsOpen")
                && plan
                    .static_response_deferred
                    .iter()
                    .any(|row| row.operation == "FsOpen")
        })
        .collect();
    let [plan] = plans.as_slice() else {
        panic!("one checked depth-three response plan has both FsOpen rows: {plans:?}");
    };
    let specialized: Vec<_> = plan
        .all_static_response_rows
        .iter()
        .filter(|row| row.operation == "FsOpen")
        .collect();
    let deferred: Vec<_> = plan
        .static_response_deferred
        .iter()
        .filter(|row| row.operation == "FsOpen")
        .collect();
    let ([specialized], [deferred]) = (specialized.as_slice(), deferred.as_slice()) else {
        panic!("the nested checked program has one specialized and one Deferred FsOpen row");
    };
    assert_eq!(deferred.sub_case, "UnconsumedTransportCaller");
    assert_eq!(deferred.producer_call_origin, specialized.producer_call_origin);
    assert_ne!(deferred.vis_origin, specialized.vis_origin);
    assert_eq!(
        deferred.bounded_handler_owner.as_deref(),
        Some(specialized.base_owner.as_str()),
        "the Deferred nested acquisition must have its enclosing checked owner"
    );
}

/// Promise class: durable invariant. R-2′ in
/// docs/program/issues/RT-BRACKET-PRODUCER-ROUTE-NESTED-BODY-ORDER.md:
/// a nested Deferred region has no planned release order, so the planner
/// refuses depth two before native artifact emission. The gate is
/// test-support-only and requires an explicit forced-mode run.
#[cfg(target_os = "linux")]
#[test]
#[ignore = "run with RT_RECUT_FORCE_DEFERRED=1 and ken-runtime/px8-ds-test-support"]
fn forced_nested_deferred_depth_two_refuses_at_planning() {
    assert_nested_deferred_refuses_at_planning(2);
}

/// Promise class: durable invariant. R-1′ in
/// docs/program/issues/RT-BRACKET-PRODUCER-ROUTE-NESTED-BODY-ORDER.md:
/// depth three has two nested regions and refuses before the former trap-40
/// execution, not merely before its outer resource is released.
#[cfg(target_os = "linux")]
#[test]
#[ignore = "run with RT_RECUT_FORCE_DEFERRED=1 and ken-runtime/px8-ds-test-support"]
fn forced_nested_deferred_depth_three_refuses_at_planning() {
    assert_nested_deferred_refuses_at_planning(3);
}

#[cfg(target_os = "linux")]
fn assert_nested_deferred_refuses_at_planning(depth: usize) {
    assert!(std::env::var_os("RT_RECUT_FORCE_DEFERRED").is_some(),
        "forced nested Deferred refusal needs its explicit test-support gate");
    // Measured here with a test-only Builder stack-size override, at both
    // depths two and three: 1600 KiB overflows, 1664 KiB passes (64-KiB
    // grid). Stated 256 MiB leaves 260480 KiB of headroom over the passing
    // bound. Do not rely on the harness's ambient thread stack.
    std::thread::Builder::new()
        .name(format!("px8ta-deferred-refusal-{depth}"))
        .stack_size(256 * 1024 * 1024)
        .spawn(move || {
            let dir = output_dir(&format!("forced-deferred-{depth}"));
            let result = ken_cli::build_native_program(
                &nested_bracket_source(depth),
                ken_cli::SourceFormat::Ken,
                &format!("px8ta-forced-deferred-{depth}"),
                dir.path(),
                ken_runtime::boundary_resource_profile::starter_smoke_profile(),
            );
            let error = match result {
                Err(error) => error,
                Ok(_) => panic!("depth {depth}: nested Deferred emitted native code"),
            };
            let diagnostic = format!("{error:?}");
            assert!(diagnostic.contains(
                "a nested bracket region has no in-place settlement owner; \
                 deferred settlement order is not planned for nested regions"),
                "depth {depth}: expected the exact nested-region planner refusal, got {diagnostic}");
        })
        .expect("spawn stated-stack forced Deferred control")
        .join()
        .expect("forced Deferred control thread");
}

#[cfg(target_os = "linux")]
// Readmitted under RT-IGNORED-PASSING-ROWS, row 4 of 11.
//
// This row carried THREE layers of annotation, each refuting the one above it
// and all three still present. They are struck together rather than appended to:
//
//   1. "Ignored pending RT-CARRIER-BYTESPAN-OBSERVE ... it refuses at object
//      emission, so the program never executes" -- refuted by layer 2, three
//      lines below it, and by the row: the program emits, executes, exits 0.
//   2. "RT-SITEOP-CARRIED-WITNESS D1a/D2 ... D5 byte-span observation was not
//      the blocker" -- the correction to layer 1, filed below it.
//   3. "RT-SITEOP-CARRIED-WITNESS D2: ... this row next refuses because a
//      carried recursive hypothesis is an eliminated value, not a callable" --
//      the live label. Does not reproduce: no refusal, and neither the
//      byte-span nor the eliminated-value signature appears in any run.
//
// READMITTED ON A MUTATION, NOT ON THE GREEN. Perturbing the ResourceRelease
// arm of ken-host effect_v1.rs dispatch (the Target binding pushed from
// pending.identity, +1000) reds the strict-LIFO assertion in
// assert_depth_finishes_and_releases_lifo -- releases against opens reversed --
// with the perturbation visible in it: left [ResourceTraceIdentityV1(1001)]
// against right [ResourceTraceIdentityV1(1)]. An earlier mutation of abi_v1.rs
// record_resource_settlements left the row green: that is the process-exit
// finalize-all path, and a bracket that releases explicitly never reaches it.
#[test]
fn public_one_level_bracket_finishes_and_releases() {
    // S 426 is an authenticated bracket settlement on the Deferred producer
    // route: it emits no response owner and selects no owner-frame Load. E4's
    // source-to-loaded-slot pin now runs on the ordinary DOUBLE_RELEASE owner
    // in px7f_resource_native. This row keeps its LIFO behavioral observation.
    // Residual: a source-parameter slot swap cannot be instrumented here:
    // Deferred producer deforestation selects no owner-frame slot or Load.
    // LIFO therefore does not claim to test source-parameter slot identity.
    assert_depth_finishes_and_releases_lifo(1);
}

#[cfg(target_os = "linux")]
// Measured on the combined link-3/child-3 tree: depth 2 builds, executes,
// exits zero with no terminal error, and releases both resources in strict
// reverse acquisition order. It is no longer an ignored blocker.
#[test]
fn public_two_level_brackets_finish_and_release_lifo() {
    assert_nested_depth_on_existing_test_thread(2);
}

#[cfg(target_os = "linux")]
// Promise class: durable invariant. Depth 3 now builds, exits without a
// terminal error, and releases all three resources in reverse acquisition
// order through the same strict-LIFO observation as depths 1 and 2.
#[test]
fn public_three_level_brackets_finish_and_release_lifo() {
    assert_nested_depth_on_existing_test_thread(3);
}

#[cfg(target_os = "linux")]
fn assert_nested_depth_on_existing_test_thread(depth: usize) {
    // Preserve the old nested fixture's 256 MiB harness provision unchanged.
    // This is not evidence for default-stack lowering or depths beyond three.
    std::thread::Builder::new()
        .name("px8ta-nested-brackets".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(move || assert_depth_finishes_and_releases_lifo(depth))
        .expect("spawn large-stack nested-bracket control")
        .join()
        .expect("nested-bracket control thread");
}

#[cfg(target_os = "linux")]
// Durable invariant: the test-only retired flat-order plan has no M4 crossing
// authority. It must remain on the ordinary fail-closed closure refusal rather
// than acquiring the production plan's captured-environment representation.
#[test]
fn px8ds_retired_flat_order_does_not_gain_m4_representation() {
    std::thread::Builder::new()
        .name("px8ds-retired-flat".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(run_px8ds_retired_flat_control)
        .expect("spawn large-stack PX8-DS retired-flat control")
        .join()
        .expect("PX8-DS retired-flat control thread");
}

#[cfg(target_os = "linux")]
fn run_px8ds_retired_flat_control() {
    const REFUSAL: &str = "unsupported runtime-IR lowering: Closure: a closure cannot cross the boundary: it is runtime-local and live-domain only, and it has no durable lane";

    let retired_dir = output_dir("px8ds-retired-flat");
    let retired = ken_runtime::with_px8ds_retired_flat_order(|| {
        ken_cli::build_native_program(
            PX8DS_SIBLING_RECURSION_PROGRAM,
            ken_cli::SourceFormat::Ken,
            "px8ds-retired-flat",
            retired_dir.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    })
    .expect_err("the retired flat-order plan must retain the closure refusal");
    let retired = format!("{retired:?}");
    assert!(
        retired.contains(REFUSAL),
        "the retired plan must reach the exact closure refusal: {retired}"
    );
    assert_eq!(
        retired.matches(REFUSAL).count(),
        1,
        "the retired plan must report one exact closure refusal: {retired}"
    );
}

#[cfg(target_os = "linux")]
// Existing baseline provisioning for the ordinary px8ds row. This WP neither
// raises nor otherwise changes its stack budget.
//
// With ambient RUST_MIN_STACK absent, one freshly compiled temporary probe
// changed only this named ignored row: its Builder read the probed byte count
// before spawning, and its thread body called
// `observe_px8ds_real_same_depth_path` directly. The compiler reported the
// constant and assertion-bearing helper as unused, confirming that the probe
// body, rather than a cached ordinary body, was in the test binary. The same
// binary was bisected at 64 KiB resolution and run twice at each boundary:
//
//   1792 KiB  stack overflow twice
//   1856 KiB  completes the build-and-observe path twice
//
// The committed 256 MiB provision is therefore 260,288 KiB (254.1875 MiB)
// above the measured passing bound and 141.24 times that bound (140.24 times
// extra headroom). This is baseline fixture provisioning, not a regression
// repair.
const PX8DS_THREAD_STACK_BYTES: usize = 256 * 1024 * 1024;

#[cfg(target_os = "linux")]
// This native row compiles, links, and runs a large artifact on its separately
// provisioned thread. The assertions pin a successful false branch with one
// Console observation and one release. The true-branch P1 tail-call route is
// outside this row's observation (RT-P1-TAIL-CALL-EXIT-HANDLER).
#[test]
fn px8ds_real_same_depth_path_runs_exact_edges() {
    std::thread::Builder::new()
        .name("px8ds-real-siblings".to_string())
        .stack_size(PX8DS_THREAD_STACK_BYTES)
        .spawn(run_px8ds_real_same_depth_path)
        .expect("spawn large-stack PX8-DS ordinary-plan half")
        .join()
        .expect("PX8-DS ordinary-plan thread");
}

#[cfg(target_os = "linux")]
fn observe_px8ds_real_same_depth_path() -> ken_runtime::EffectObservation {
    let exact_dir = output_dir("px8ds-exact-edges");
    let exact = ken_cli::build_native_program(
        PX8DS_SIBLING_RECURSION_PROGRAM,
        ken_cli::SourceFormat::Ken,
        "px8ds-exact-edges",
        exact_dir.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .expect("exact dynamic edges compile the same checked source");
    ken_runtime::run_bound_process_effect_observation(
        &exact.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: exact_dir.path().to_owned(),
            plan_hash: exact.plan_transport_hash,
        },
    )
    .expect("the exact-edge linked artifact runs")
}

#[cfg(target_os = "linux")]
fn run_px8ds_real_same_depth_path() {
    let observation = observe_px8ds_real_same_depth_path();
    assert_eq!(observation.exit_status, 0, "{observation:?}");
    assert_eq!(observation.terminal_error, None);
    assert_eq!(
        observation
            .effect_trace
            .iter()
            .filter(|event| event.operation == ken_runtime::HostOpV1::ConsoleIsTerminal)
            .count(),
        1,
        "the live false branch must skip both recursive siblings"
    );
    assert_eq!(
        observation
            .effect_trace
            .iter()
            .map(|event| event.operation)
            .collect::<Vec<_>>(),
        vec![
            ken_runtime::HostOpV1::BufferAllocate,
            ken_runtime::HostOpV1::ConsoleIsTerminal,
            ken_runtime::HostOpV1::ResourceRelease,
        ]
    );
}

#[cfg(target_os = "linux")]
fn px8ds_swapped_arm_source() -> String {
    let original = PX8DS_SIBLING_RECURSION_PROGRAM;
    let false_then_true = "    False |-> after_countdown (Ok Int Bool False);\n    True |-> bind";
    let true_then_false = "    True |-> after_countdown (Ok Int Bool False);\n    False |-> bind";
    assert_eq!(
        original.matches(false_then_true).count(),
        1,
        "swap one fixed fixture arm"
    );
    original.replacen(false_then_true, true_then_false, 1)
}

#[cfg(target_os = "linux")]
fn observe_px8ds_p1_parity(
    source: &str,
    case: &str,
) -> (
    ken_runtime::EffectObservation,
    Vec<ken_runtime::StaticResponseFeasibilityDiagnostic>,
) {
    let dir = output_dir(case);
    let (compiled, plans) = ken_runtime::with_static_response_feasibility_diagnostics(|| {
        ken_cli::build_native_program(
            source,
            ken_cli::SourceFormat::Ken,
            case,
            dir.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });
    let compiled = compiled.unwrap_or_else(|error| panic!("{case}: native build: {error:?}"));
    let native = ken_runtime::run_bound_process_effect_observation(
        &compiled.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: dir.path().to_owned(),
            plan_hash: compiled.plan_transport_hash,
        },
    )
    .unwrap_or_else(|error| panic!("{case}: linked execution: {error:?}"));
    let mut host = ken_interp::PosixHost::new_at(dir.path());
    let interpreted = ken_cli::run_program_effect_observation(
        source,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        dir.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .unwrap_or_else(|error| panic!("{case}: interpreter execution: {error:?}"));
    assert_eq!(
        native.exit_status, interpreted.exit_status,
        "{case}: exit parity"
    );
    assert_eq!(
        native.terminal_error, interpreted.terminal_error,
        "{case}: terminal parity"
    );
    assert_eq!(
        native.terminal_exit, interpreted.terminal_exit,
        "{case}: exit class parity"
    );
    assert_eq!(native.stdout, interpreted.stdout, "{case}: stdout parity");
    assert_eq!(
        native.effect_trace, interpreted.effect_trace,
        "{case}: trace parity"
    );
    (native, plans)
}

#[cfg(target_os = "linux")]
fn assert_px8ds_p1_effect_trace(
    observation: &ken_runtime::EffectObservation,
    console_count: usize,
) {
    assert_eq!(observation.exit_status, 0, "{observation:?}");
    assert_eq!(observation.terminal_error, None, "{observation:?}");
    let mut expected = vec![ken_runtime::HostOpV1::BufferAllocate];
    expected.extend(std::iter::repeat_n(
        ken_runtime::HostOpV1::ConsoleIsTerminal,
        console_count,
    ));
    expected.push(ken_runtime::HostOpV1::ResourceRelease);
    let observed = observation
        .effect_trace
        .iter()
        .map(|event| event.operation)
        .collect::<Vec<_>>();
    assert_eq!(observed, expected, "{observation:?}");
}

/// Promise class: normative compatibility vector for this fixed checked
/// swapped-arm program. MEASURED: the selected false arm executes the full
/// countdown suffix under the linked native process and the interpreter with
/// the same canonical effects and successful exit. CLAIMED: a P1 K whose
/// only completions are direct static tail calls reaches its handler and
/// returns the checked answer. THE GAP: this vector covers the one checked
/// nested Console operation, not every response operation or depth.
#[cfg(target_os = "linux")]
#[test]
fn px8ds_p1_swapped_tail_call_exits_match_reference() {
    std::thread::Builder::new()
        .name("px8ds-p1-swapped".to_string())
        .stack_size(PX8DS_THREAD_STACK_BYTES)
        .spawn(|| {
            let (native, _) =
                observe_px8ds_p1_parity(&px8ds_swapped_arm_source(), "px8ds-p1-swapped");
            assert_px8ds_p1_effect_trace(&native, 4);
        })
        .expect("spawn with existing stated px8ds stack provision")
        .join()
        .expect("swapped P1 linked observation thread");
}

/// Promise class: normative compatibility vector for the unchanged original
/// checked program. MEASURED: native and interpreter observe one Console
/// effect on the false arm, and the planner projects both P1 rows onto the
/// same specialization owner. CLAIMED: new tail-call admission does not
/// perturb the original path or RELAY classification. THE GAP: the public
/// projection does not expose all RELAY row details; existing roster controls
/// retain that independent job.
#[cfg(target_os = "linux")]
#[test]
fn px8ds_p1_original_false_arm_and_handler_owner_remain_stable() {
    std::thread::Builder::new()
        .name("px8ds-p1-original".to_string())
        .stack_size(PX8DS_THREAD_STACK_BYTES)
        .spawn(|| {
            let (native, plans) =
                observe_px8ds_p1_parity(PX8DS_SIBLING_RECURSION_PROGRAM, "px8ds-p1-original");
            assert_px8ds_p1_effect_trace(&native, 1);
            let with_p1 = plans
                .iter()
                .filter_map(|plan| {
                    let rows = plan
                        .static_response_deferred
                        .iter()
                        .filter(|row| {
                            row.sub_case == "NoContinuationUnit"
                                && row.operation == "ConsoleIsTerminal"
                        })
                        .collect::<Vec<_>>();
                    (rows.len() == 2).then_some(rows)
                })
                .collect::<Vec<_>>();
            let [rows] = with_p1.as_slice() else {
                panic!("one response plan carries the original two Console P1 rows: {plans:?}");
            };
            for row in rows {
                assert_eq!(
                    row.handler_owner.as_deref(),
                    Some("Specialization(ContinuationSpecializationId(0))"),
                    "P1 row: {row:?}"
                );
            }
        })
        .expect("spawn with existing stated px8ds stack provision")
        .join()
        .expect("original P1 linked observation thread");
}

#[cfg(target_os = "linux")]
fn assert_depth_finishes_and_releases_lifo(depth: usize) {
    let (observation, planned_frames) = run_depth(depth);
    assert_eq!(observation.exit_status, 0, "depth {depth}: {observation:?}");
    assert_eq!(observation.terminal_error, None, "depth {depth}");
    assert!(
        planned_frames >= depth,
        "depth {depth} must retain every checked bracket continuation"
    );

    let opens = observation
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_runtime::HostOpV1::FsOpen)
        .map(|event| event.resource_bindings[0].1.clone())
        .collect::<Vec<_>>();
    let releases = observation
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_runtime::HostOpV1::ResourceRelease)
        .map(|event| event.resource_bindings[0].1.clone())
        .collect::<Vec<_>>();
    assert_eq!(opens.len(), depth, "depth {depth} acquisition count");
    assert_eq!(releases.len(), depth, "depth {depth} release count");
    assert_eq!(
        releases,
        opens.into_iter().rev().collect::<Vec<_>>(),
        "depth {depth} releases must be strict LIFO"
    );
}
