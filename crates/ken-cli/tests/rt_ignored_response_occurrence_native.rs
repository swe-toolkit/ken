//! Two sequential resource brackets: native/interpreter observation parity.
//!
//! The checked source builds two constructions of one continuation. Dynamic
//! recursive-position captures must travel with each construction's value.

#[cfg(target_os = "linux")]
const TWO_BUFFER_WITNESS: &str = include_str!("rt_ignored_two_buffer_witness.ken");
#[cfg(target_os = "linux")]
const DISTINGUISHABLE_BRACKETS: &str = include_str!("rt_two_bracket_distinguishable.ken");
#[cfg(target_os = "linux")]
const ONE_BUFFER_WITNESS: &str = include_str!("rt_one_buffer_residual_layout.ken");
#[cfg(target_os = "linux")]
const PLAIN_MATCH_WITNESS: &str = include_str!("rt_plain_match_layout.ken");
#[cfg(target_os = "linux")]
const PLAIN_MATCH_C91_IR: &str = include_str!("rt_plain_match_layout.c91.clif");

// Measured on this exact checked witness: a stated 2 MiB worker aborts from
// stack overflow; 4 MiB completes the planner and reaches BoundaryCarrier.
// Provision that measured upper bound plus 4 MiB of local headroom. This
// baseline is not a depth/stack claim, and the explicit Builder budget takes
// precedence over ambient RUST_MIN_STACK.
#[cfg(target_os = "linux")]
const WITNESS_STACK_BYTES: usize = 4 * 1024 * 1024 + 4 * 1024 * 1024;

#[cfg(target_os = "linux")]
#[test]
fn sequential_brackets_carry_independent_residuals_with_native_parity() {
    std::thread::Builder::new()
        .name("rt-ignored-sequential-brackets".to_string())
        .stack_size(WITNESS_STACK_BYTES)
        .spawn(run_sequential_brackets_witness)
        .expect("spawn stated-stack witness")
        .join()
        .expect("sequential-brackets witness thread");
}

// A stated 2 MiB worker overflowed; a stated 4 MiB worker completed this
// fixture. Provision the measured completing bound (4 MiB) plus 4 MiB of
// local headroom. Completion bounds the peak below 4 MiB but does not measure
// its exact depth. This worker size does not rely on ambient RUST_MIN_STACK.
#[cfg(target_os = "linux")]
const DISTINGUISHABLE_STACK_BYTES: usize = 4 * 1024 * 1024 + 4 * 1024 * 1024;

/// Promise class: durable invariant. A first bracket whose body fails and a
/// second whose body succeeds must retain that pairing across native and
/// interpreter execution. Exit 21 and capacities 1/2 are fixed-fixture values;
/// swapping the responses has a different exit, while internal planner changes
/// preserving the pair leave the observation equal.
#[cfg(target_os = "linux")]
#[test]
fn distinguishable_brackets_keep_native_interpreter_pairing() {
    std::thread::Builder::new()
        .name("rt-distinguishable-brackets".to_string())
        .stack_size(DISTINGUISHABLE_STACK_BYTES)
        .spawn(run_distinguishable_brackets_witness)
        .expect("spawn stated-stack distinguishable witness")
        .join()
        .expect("distinguishable witness thread");
}

/// Promise class: durable invariant. The one-bracket baseline retains its
/// native/interpreter observation even if its private field layout changes.
#[cfg(target_os = "linux")]
#[test]
fn one_bracket_retains_native_parity() {
    std::thread::Builder::new()
        .name("rt-residual-one-bracket".to_string())
        .stack_size(WITNESS_STACK_BYTES)
        .spawn(run_one_bracket_layout)
        .expect("spawn stated-stack one-bracket witness")
        .join()
        .expect("one-bracket witness thread");
}

#[cfg(target_os = "linux")]
#[test]
fn ordinary_match_has_empty_residual_population_and_c91_identical_unit_ir() {
    std::thread::Builder::new()
        .name("rt-residual-plain-match".to_string())
        .stack_size(WITNESS_STACK_BYTES)
        .spawn(run_plain_recursive_match_census)
        .expect("spawn stated-stack plain match")
        .join()
        .expect("plain match thread");
}

#[cfg(target_os = "linux")]
fn run_plain_recursive_match_census() {
    let output = tempfile::Builder::new()
        .prefix("ken-rt-residual-plain-match-")
        .tempdir()
        .expect("unique plain match output root");
    let (((compiled, guards), plans), emitted_ir) =
        ken_runtime::with_plain_native_unit_ir_observations(|| {
            ken_runtime::with_recursive_residual_disposition_census(|| {
                ken_runtime::with_recursive_residual_match_guard_observations(
                    ken_runtime::RecursiveResidualMatchGuardMutation::Exact,
                    || ken_cli::build_native_program(
                        PLAIN_MATCH_WITNESS, ken_cli::SourceFormat::Ken,
                        "native-program", output.path(),
                        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                    ),
                )
            })
        });
    compiled.expect("ordinary match builds natively");
    assert!(!plans.is_empty(), "the planner must report the actual context population");
    assert!(plans.iter().all(|plan| plan.parent_sites == 0 && plan.checked_s.is_empty()
        && plan.contexts.iter().all(|&(worker, captures, missing)|
            worker == 0 && captures == 0 && missing == 0)),
        "the measured ordinary-match plan must have W=0, M=empty, and S=empty: {plans:?}");
    assert!(!guards.is_empty(), "the emitted IR must contain ordinary match binders");
    assert!(guards.iter().all(|row| !row.planned_in_s && row.emitted_class_calls == 0),
        "empty S must emit zero ordinary match guards: {guards:?}");
    // Promise class: transition sentinel for this exact-base WP. The fixture
    // records the emitted checked unit bodies on c91, not repository prose.
    // Cranelift's display appends a blank separator; line breaks after the
    // final brace are display whitespace, not unit IR. Compare the body
    // byte-for-byte so intentional codegen changes still require review.
    assert_eq!(emitted_ir.trim_end_matches('\n').as_bytes(),
        PLAIN_MATCH_C91_IR.trim_end_matches('\n').as_bytes(),
        "the measured zero-S native unit IR differs from c91");
}

#[cfg(target_os = "linux")]
fn run_one_bracket_layout() {
    let output = tempfile::Builder::new()
        .prefix("ken-rt-residual-one-bracket-")
        .tempdir()
        .expect("unique one-bracket output root");
    let ((compiled, rows), dispositions) =
        ken_runtime::with_recursive_residual_disposition_census(|| {
            ken_runtime::with_recursive_residual_match_guard_observations(
                ken_runtime::RecursiveResidualMatchGuardMutation::Exact,
                || ken_cli::build_native_program(
                    ONE_BUFFER_WITNESS, ken_cli::SourceFormat::Ken,
                    "rt_one_buffer_residual_layout", output.path(),
                    ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                ),
            )
        });
    let compiled = compiled.expect("one checked bracket builds natively");
    assert!(dispositions.iter().any(|plan| plan.parent_sites == 1
        && plan.checked_s.len() == 1
        && plan.contexts.iter().any(|&(worker, captures, missing)|
            worker == 5 && captures == 4 && missing == 2)),
        "the one-bracket fixture must MEASURE one Vis creation site with Wrapped W5/C4/M2 and |S|=1: {dispositions:?}");
    assert!(rows.iter().all(|row| row.emitted_class_calls == usize::from(row.planned_in_s)),
        "the one-bracket emitter must respect its planned exact S");
    // Promise class: normative compatibility vector for this exact-base WP.
    // The same fixture on clean c91 emitted zero stdout/stderr bytes and exited
    // 0. The W5-suppression control separately proves the private carriage is
    // necessary; external equality is not used to infer an internal Plain tag.
    let process = std::process::Command::new(&compiled.artifact.executable_path)
        .current_dir(output.path())
        .output()
        .expect("execute the linked one-bracket native artifact");
    assert_eq!(process.stdout, b"", "stdout must match the c91 native artifact");
    assert_eq!(process.stderr, b"", "stderr must match the c91 native artifact");
    assert_eq!(process.status.code(), Some(0), "exit must match the c91 native artifact");
    let native = ken_runtime::run_bound_process_effect_observation(
        &compiled.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(), environment: Vec::new(),
            cwd: output.path().to_owned(), plan_hash: compiled.plan_transport_hash,
        },
    ).expect("one native bracket executes");
    let mut host = ken_interp::PosixHost::new_at(output.path());
    let interpreted = ken_cli::run_program_effect_observation(
        ONE_BUFFER_WITNESS, ken_cli::SourceFormat::Ken, &[], &[],
        output.path().as_os_str().as_encoded_bytes(), &mut host,
    ).expect("one checked bracket interprets");
    assert_eq!(native, interpreted, "one-bracket observation is byte-identical by value");
}

#[cfg(target_os = "linux")]
/// Promise class: durable negative control. Removing the measured W5
/// carriage recovers the exact base refusal on each bracket population.
#[cfg(target_os = "linux")]
#[test]
fn suppressing_w5_cannot_publish_either_bracket_witness() {
    std::thread::Builder::new()
        .name("rt-residual-w5-suppression".to_string())
        .stack_size(WITNESS_STACK_BYTES)
        .spawn(|| {
            for (name, source) in [
                ("one", ONE_BUFFER_WITNESS),
                ("two", TWO_BUFFER_WITNESS),
            ] {
                let output = tempfile::Builder::new()
                    .prefix("ken-rt-residual-suppressed-")
                    .tempdir()
                    .expect("unique suppressed-bracket output root");
                let (built, applied) =
                    ken_runtime::with_recursive_residual_disposition_mutation(
                        ken_runtime::RecursiveResidualDispositionMutation::SuppressW5,
                        || ken_cli::build_native_program(
                            source, ken_cli::SourceFormat::Ken, name, output.path(),
                            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                        ),
                    );
                assert!(applied > 0, "the {name}-bracket suppression must reach W5");
                let error = built.expect_err("missing W5 must refuse the native build");
                let reason = format!("{error:?}");
                assert!(reason.contains("BoundaryCarrier")
                    && reason.contains("call provides 1"),
                    "the {name}-bracket mutation must restore the exact base refusal: {reason}");
            }
        })
        .expect("spawn stated-stack suppression control")
        .join()
        .expect("suppression-control thread");
}

/// Promise class: durable negative control. Worker Parameter-tail cardinality
/// and context Capture cardinality are distinct contracts; corrupt each
/// independently on the real checked two-bracket plan and require refusal.
#[cfg(target_os = "linux")]
#[test]
fn worker_and_context_capture_counts_each_refuse_when_corrupted() {
    std::thread::Builder::new()
        .name("rt-residual-separate-cardinalities".to_string())
        .stack_size(WITNESS_STACK_BYTES)
        .spawn(|| {
            use ken_runtime::RecursiveResidualDispositionMutation as Mutation;
            for (name, mutation) in [
                ("worker", Mutation::WorkerCardinalityPlusOne),
                ("context", Mutation::ContextCardinalityPlusOne),
            ] {
                let output = tempfile::Builder::new()
                    .prefix("ken-rt-residual-bad-capture-count-")
                    .tempdir()
                    .expect("unique corrupt-cardinality output root");
                let (built, applied) =
                    ken_runtime::with_recursive_residual_disposition_mutation(
                        mutation,
                        || ken_cli::build_native_program(
                            TWO_BUFFER_WITNESS, ken_cli::SourceFormat::Ken,
                            name, output.path(),
                            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                        ),
                    );
                assert!(applied > 0, "{name} capture mutation must reach a descriptor");
                let error = built.expect_err("a mismatched capture run must refuse");
                let reason = format!("{error:?}");
                assert!(reason.contains("recursive residual"),
                    "the {name} cardinality must refuse at the residual contract: {reason}");
            }
        })
        .expect("spawn stated-stack capture-cardinality control")
        .join()
        .expect("capture-cardinality control thread");
}

#[cfg(target_os = "linux")]
fn run_sequential_brackets_witness() {
    let output = tempfile::Builder::new()
        .prefix("ken-rt-ignored-two-buffer-")
        .tempdir()
        .expect("unique native output root");
    let ((compiled, diagnostics), dispositions) =
        ken_runtime::with_recursive_residual_disposition_census(|| {
            ken_runtime::with_static_response_feasibility_diagnostics(|| {
                ken_cli::build_native_program(
                    TWO_BUFFER_WITNESS,
                    ken_cli::SourceFormat::Ken,
                    "rt_ignored_two_buffer_witness",
                    output.path(),
                    ken_runtime::boundary_resource_profile::starter_smoke_profile(),
                )
            })
        });
    let compiled_output = compiled.expect("both checked brackets must build as a native object");
    assert!(dispositions.iter().any(|plan| plan.parent_sites == 2
        && plan.contexts.len() == 2 && plan.checked_s.len() == 1
        && plan.contexts.contains(&(5, 4, 2))
        && plan.contexts.contains(&(7, 6, 4))),
        "the two Vis creation sites must retain W5/C4/M2 and W7/C6/M4 contexts: {dispositions:?}");
    let rows = diagnostics
        .iter()
        .flat_map(|plan| &plan.all_static_response_rows)
        .filter(|row| row.operation == "ResourceRelease")
        .collect::<Vec<_>>();
    assert_eq!(
        rows.len(),
        2,
        "the actual checked witness must plan two response-Vis rows"
    );
    assert_ne!(rows[0].vis_origin, rows[1].vis_origin);
    assert_ne!(
        rows[0].producer_call_origin, rows[1].producer_call_origin,
        "two occurrences of one response constructor must select distinct calls"
    );
    assert_ne!(
        rows[0].selected_leaf, rows[1].selected_leaf,
        "the selected calls must belong to distinct producer leaves"
    );
    for row in &rows {
        assert!(
            row.eliminating_cm.is_some(),
            "Vis {}: selected leaf {} must have a Vis-case CM",
            row.vis_origin,
            row.selected_leaf,
        );
        assert!(
            row.cm_scrutinee_contains_vis,
            "Vis {}: selected leaf {} in CM {:?} must be in that CM's scrutinee",
            row.vis_origin, row.selected_leaf, row.eliminating_cm,
        );
    }
    assert_ne!(rows[0].eliminating_cm, rows[1].eliminating_cm);

    let native = ken_runtime::run_bound_process_effect_observation(
        &compiled_output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: output.path().to_owned(),
            plan_hash: compiled_output.plan_transport_hash,
        },
    )
    .expect("the linked two-bracket object executes without a runtime trap");
    let mut host = ken_interp::PosixHost::new_at(output.path());
    let interpreted = ken_cli::run_program_effect_observation(
        TWO_BUFFER_WITNESS,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        output.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .expect("the same checked source executes under the interpreter");
    assert_eq!(native.exit_status, interpreted.exit_status);
    assert_eq!(native.terminal_error, interpreted.terminal_error);
    assert_eq!(native.terminal_exit, interpreted.terminal_exit);
    assert_eq!(native.exit_status, 0, "both brackets finish at Success");
    assert_eq!(native.terminal_error, None);
    let non_release = |observed: &ken_runtime::EffectObservation| {
        observed.effect_trace.iter()
            .filter(|event| event.operation != ken_runtime::HostOpV1::ResourceRelease)
            .cloned().collect::<Vec<_>>()
    };
    assert_eq!(non_release(&native), non_release(&interpreted));
    // RT-BRACKET-RELEASE-ORDER-PARITY tracks the independent ordering gap.
    // The pair of release observations must agree as a set, not in chronology.
    let releases = |observed: &ken_runtime::EffectObservation| {
        let mut events = observed.effect_trace.iter()
            .filter(|event| event.operation == ken_runtime::HostOpV1::ResourceRelease)
            .map(|event| format!("{:?}", (
                event.resource_bindings.clone(), event.request.clone(), event.outcome.clone(),
            ))).collect::<Vec<_>>();
        events.sort();
        events
    };
    let native_releases = releases(&native);
    assert_eq!(native_releases.len(), 2, "the witness executes both release effects");
    assert_eq!(native_releases, releases(&interpreted));
}

#[cfg(target_os = "linux")]
fn run_distinguishable_brackets_witness() {
    let output = tempfile::Builder::new()
        .prefix("ken-rt-distinguishable-brackets-")
        .tempdir()
        .expect("unique distinguishable-bracket output root");
    let compiled = ken_cli::build_native_program(
        DISTINGUISHABLE_BRACKETS,
        ken_cli::SourceFormat::Ken,
        "rt_two_bracket_distinguishable",
        output.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .expect("two distinguishable brackets build natively");
    let native = ken_runtime::run_bound_process_effect_observation(
        &compiled.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: output.path().to_owned(),
            plan_hash: compiled.plan_transport_hash,
        },
    )
    .expect("linked distinguishable-bracket object executes");
    let mut host = ken_interp::PosixHost::new_at(output.path());
    let interpreted = ken_cli::run_program_effect_observation(
        DISTINGUISHABLE_BRACKETS,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        output.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .expect("the same distinguishable source executes under the interpreter");

    assert_eq!(native.exit_status, interpreted.exit_status);
    assert_eq!(
        native.exit_status, 21,
        "first BodyError and second Ok select 21"
    );
    assert_eq!(native.terminal_error, interpreted.terminal_error);
    let non_release = |observed: &ken_runtime::EffectObservation| {
        observed
            .effect_trace
            .iter()
            .filter(|event| event.operation != ken_runtime::HostOpV1::ResourceRelease)
            .cloned()
            .collect::<Vec<_>>()
    };
    assert_eq!(non_release(&native), non_release(&interpreted));

    let releases = |observed: &ken_runtime::EffectObservation| {
        observed
            .effect_trace
            .iter()
            .filter(|event| event.operation == ken_runtime::HostOpV1::ResourceRelease)
            .cloned()
            .collect::<Vec<_>>()
    };
    let native_releases = releases(&native);
    let interpreted_releases = releases(&interpreted);
    assert_eq!(native_releases.len(), 2, "both buffers must be released");
    assert_eq!(interpreted_releases.len(), 2);
    let release_set = |events: &[ken_runtime::EffectEvent]| {
        let mut rows = events
            .iter()
            .map(|event| {
                format!(
                    "{:?}",
                    (&event.resource_bindings, &event.request, &event.outcome,)
                )
            })
            .collect::<Vec<_>>();
        rows.sort();
        rows
    };
    assert_eq!(
        release_set(&native_releases),
        release_set(&interpreted_releases)
    );

    // BufferAllocate and ResourceRelease both record the allocated buffer as a
    // Target binding. ResourceRelease's request itself is a unit variant, so
    // capacity is checked at allocation, and release is linked by identity.
    let target_identity = |event: &ken_runtime::EffectEvent| {
        let targets = event
            .resource_bindings
            .iter()
            .filter(|(role, _)| *role == ken_runtime::ResourceBindingRole::Target)
            .map(|(_, identity)| *identity)
            .collect::<Vec<_>>();
        assert_eq!(
            targets.len(),
            1,
            "one target identity per buffer event: {event:?}"
        );
        targets[0]
    };
    let release_ids = native_releases
        .iter()
        .map(|event| target_identity(event))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        release_ids.len(),
        2,
        "the two releases must target distinct buffers"
    );
    let allocations = native
        .effect_trace
        .iter()
        .filter(|event| event.operation == ken_runtime::HostOpV1::BufferAllocate)
        .collect::<Vec<_>>();
    assert_eq!(allocations.len(), 2, "each bracket allocates one buffer");
    let mut capacities = allocations
        .iter()
        .map(|event| {
            let ken_runtime::CanonicalRequestV1::BufferAllocate { capacity } = &event.request
            else {
                panic!("allocation event lacks its typed capacity: {event:?}");
            };
            *capacity
        })
        .collect::<Vec<_>>();
    capacities.sort_unstable();
    assert_eq!(capacities, [1, 2], "the two requested capacities differ");
    let allocated_ids = allocations
        .iter()
        .map(|event| target_identity(event))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        release_ids, allocated_ids,
        "release exactly the allocated buffers"
    );
}
