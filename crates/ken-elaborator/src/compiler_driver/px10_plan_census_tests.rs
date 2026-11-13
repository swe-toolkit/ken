//! PX10-NATIVE-SITE-IDENTITY AC-2: how far the three px10 programs plan.
//!
//! The Process gate refuses all three before object emission, so this census
//! hands each preparation straight to the emission the gate guards. Nothing
//! here runs a native program; emission beneath that gate establishes no parity.

use super::*;

const SMOKE: &str = include_str!("../../../ken-cli/tests/fixtures/px10_child_wait_smoke.ken");
const ARGV_ENV_PIPE: &str = include_str!("../../../ken-cli/tests/fixtures/px10_argv_env_pipe.ken");
const LIVE_CHILD: &str = include_str!("../../../ken-cli/tests/fixtures/px10_live_child.ken");

fn px10_native_source(source: &str) -> CompilerSource {
    CompilerSource::new("src/main.ken", source)
        .with_roots(vec![
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
        ])
}

/// The z3 adapter job runs this module as effective root: its container is
/// created with no `--user` and its image sets no `USER`. Startup admission
/// refuses root unless the program declares the allowance (ADR 0019 §2), so
/// the two native-executing rows declare it in their own source.
fn root_allowed_smoke() -> String {
    let source = SMOKE.replacen(
        "program capabilities FS AFull, Process\n",
        "program capabilities FS AFull, Process, RootExecution Allow\n",
        1,
    );
    assert_ne!(source, SMOKE, "the smoke fixture header moved");
    source
}

/// Measured on every runner: an effective-root runner admits this plan.
fn assert_admitted_as_root(plan: &NativeEntrypointPlanV1) {
    assert!(
        ken_host::admit_root_execution(
            ken_host::EffectiveUidSnapshotV1::scripted(0),
            plan.allows_root_execution(),
        )
        .is_ok(),
        "an effective-root runner would refuse this row before its first effect"
    );
}

fn emit_below_gate(
    name: &str,
    source: &str,
) -> Result<NativeProgramBuildOutput, NativeProgramBuildError> {
    let preparation = prepare_native_program_sources(name, vec![px10_native_source(source)])
        .expect("the px10 program prepares");
    assert!(
        preparation.plan.process_declared,
        "the census must plan a program the Process gate refuses"
    );
    let root = tempfile::tempdir().expect("census output root");
    emit_native_program_preparation(
        preparation,
        root.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
}

/// Promise class: transition sentinel, retired when PX10-NATIVE-SPAWN-PARITY
/// promotes Process and removes the labelled-transport containment.
/// MEASURED: removing the Process bit from the checked smoke preparation
/// refuses its issued transport before object finish, on the named construct.
/// CLAIMED: the census-only emission route cannot ship unexecuted transport
/// merely because a caller mistakenly treats the plan as Process-free.
/// THE GAP: this test does not establish labelled-transport execution parity.
#[test]
fn px10_set_issuing_smoke_without_process_gate_refuses_containment() {
    in_census_thread("px10-containment-without-process-bit", || {
        let mut preparation = prepare_native_program_sources(
            "px10_containment_without_process_bit",
            vec![px10_native_source(SMOKE)],
        )
        .expect("smoke prepares");
        assert!(
            preparation.plan.process_declared,
            "the original plan is gated"
        );
        preparation.plan.process_declared = false;
        let root = tempfile::tempdir().expect("census output root");
        let error = emit_native_program_preparation(
            preparation,
            root.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
        .expect_err("an ungated set-issuing plan cannot emit an artifact");
        let NativeProgramBuildError::Packaging(error) = error else {
            panic!("expected a packaging refusal, got {error:?}");
        };
        assert_eq!(
            error.stage,
            ken_runtime::ObjectLinkerPackagingStage::ObjectEmission
        );
        assert_eq!(
            error.source_cause,
            Some(ken_runtime::LoweringCause::Fragment)
        );
        assert!(error.reason.contains("ProducerAlternativeSet"), "{error:?}");
    });
}

/// Promise class: durable invariant. MEASURED: a real smoke
/// planner issues two-member sets, builds beneath the Process gate, and
/// rejects readers that add one foreign same-result identity or drop a
/// member. CLAIMED: a reader cannot interpret an issued labelled word with
/// a different member set. THE GAP: these two probes do not enumerate every
/// possible foreign identity; they pin both cardinality directions at the
/// actual issued-set resolver.
#[test]
fn px10_issued_set_reader_mismatch_refuses_at_planning() {
    in_census_thread("px10-reader-member-mismatch", || {
        let (built, readers) =
            ken_runtime::with_producer_alternative_reader_negative_observations(|| {
                emit_below_gate("px10_reader_member_mismatch", SMOKE)
            });
        built.expect("the unchanged smoke fixture builds beneath its Process gate");
        assert_eq!(readers.len(), 3, "smoke must issue three reader sets");
        for reader in readers {
            assert_eq!(
                reader.issued_members, 2,
                "smoke reader set at {}",
                reader.result
            );
            assert!(
                reader
                    .extra_member_refusal
                    .as_deref()
                    .is_some_and(|reason| reason.contains(
                        "the result reader's expected member set differs from the issued producer alternative set"
                    )),
                "foreign reader at {}: {reader:?}", reader.result
            );
            assert!(
                reader
                    .dropped_member_refusal
                    .as_deref()
                    .is_some_and(|reason| reason.contains(
                        "a single-producer result reader names a result with an issued producer alternative set"
                    )),
                "singleton reader at {}: {reader:?}", reader.result
            );
        }
    });
}

/// Promise class: transition sentinel. MEASURED: the unavailable Process
/// operation reaches native dispatch only on this census route, not through
/// a product program (emission or the Process gate refuses first). Its host
/// status is decoded as ProcessPlanOpen, and the executable names the operation
/// rather than reporting malformed borrowed input. CLAIMED: when this refusal
/// reaches a native process, the diagnostic preserves its cause. THE GAP: no
/// labelled-transport execution parity or product admission is established.
#[test]
fn unavailable_process_operation_names_its_host_refusal() {
    in_census_thread("px10-unavailable-native-diagnostic", || {
        let preparation = prepare_native_program_sources(
            "px10_unavailable_native_diagnostic",
            vec![px10_native_source(&root_allowed_smoke())],
        )
        .expect("px10 smoke prepares");
        assert!(
            preparation.plan.process_declared,
            "the Process gate still owns admission"
        );
        assert_admitted_as_root(&preparation.plan);
        let root = tempfile::tempdir().expect("census output root");
        let output = emit_native_program_preparation(
            preparation,
            root.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
        .expect("smoke emits only beneath its Process gate");
        let error = ken_runtime::run_bound_process_effect_observation(
            &output.artifact,
            &ken_runtime::NativeEffectRunOptionsV1 {
                arguments: Vec::new(),
                environment: Vec::new(),
                cwd: root.path().to_owned(),
                plan_hash: output.plan_transport_hash,
            },
        )
        .expect_err("the unavailable operation cannot run natively");
        let ran = std::process::Command::new(&output.artifact.executable_path)
            .current_dir(root.path())
            .output()
            .expect("linked census artifact runs");
        assert!(
            matches!(
                &error,
                ken_runtime::NativeEffectRunErrorV1::OperationUnavailable {
                    operation: ken_host::HostOpV1::ProcessPlanOpen
                }
            ),
            "the exact host refusal must survive: {error:?}; stderr: {:?}",
            String::from_utf8_lossy(&ran.stderr)
        );
        assert_eq!(ran.status.code(), Some(1), "stderr: {:?}", ran.stderr);
        assert_eq!(
            ran.stderr,
            b"ken native trap: host operation ProcessPlanOpen (0x0601) is unavailable to native code\n"
        );
        assert!(!String::from_utf8_lossy(&ran.stderr).contains("malformed borrowed process input"));
    });
}

/// Promise class: regression. A NativeTested ConsoleWrite still completes and
/// yields its one unchanged event. The unavailable-op refusal has no product
/// witness today: emission or the Process gate refuses first.
#[test]
fn native_tested_console_write_retains_its_single_event() {
    const SOURCE: &str = r#"program capabilities FS APartial, RootExecution Allow
import Capability.Console.Text (print_line)
import Capability.Process.Exit (host_program)
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_program APartial (print_line "unavailable-control")
"#;
    in_census_thread("native-tested-unavailable-control", || {
        let preparation = prepare_native_program_sources(
            "native_tested_unavailable_control",
            vec![px10_native_source(SOURCE)],
        )
        .expect("NativeTested Console program prepares");
        assert!(
            !preparation.plan.process_declared,
            "Console does not use the Process gate"
        );
        assert_admitted_as_root(&preparation.plan);
        let root = tempfile::tempdir().expect("control output root");
        let output = emit_native_program_preparation(
            preparation,
            root.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
        .expect("NativeTested Console program emits");
        let observed = ken_runtime::run_bound_process_effect_observation(
            &output.artifact,
            &ken_runtime::NativeEffectRunOptionsV1 {
                arguments: Vec::new(),
                environment: Vec::new(),
                cwd: root.path().to_owned(),
                plan_hash: output.plan_transport_hash,
            },
        )
        .expect("NativeTested Console program runs");
        assert_eq!(observed.exit_status, 0);
        assert_eq!(observed.stdout, b"unavailable-control\n");
        assert_eq!(
            observed.effect_trace,
            vec![ken_runtime::EffectEvent {
                sequence: 0,
                operation: ken_host::HostOpV1::ConsoleWrite,
                capability: None,
                resource_bindings: Vec::new(),
                request: ken_runtime::CanonicalRequestV1::ConsoleWrite {
                    stream: ken_runtime::ConsoleStreamV1::Stdout,
                    bytes: b"unavailable-control\n".to_vec(),
                },
                outcome: ken_runtime::CanonicalOutcomeV1::Success(
                    ken_runtime::CanonicalReplyV1::Unit,
                ),
            }]
        );
    });
}

/// Promise class: durable invariant over these committed source fixtures.
/// MEASURED: the actual writer compares the emitted answer with its returned
/// operand, including SSA-word identity for carried values. Each foreign
/// issued member is reached and its answer stays raw.
/// CLAIMED: no alternative member mints a [Child, Label] answer.
/// THE GAP: a specialized-to-specialized substitution cannot be distinguished
/// by the operand-kind comparison; the foreign-member mutation must wrap a
/// value and therefore changes that kind or the carried SSA word.
#[test]
fn px10_argv_alternative_writer_keeps_foreign_members_raw() {
    assert_alternative_writer_raw(
        "px10_answer_argv",
        ARGV_ENV_PIPE,
        "Predeclared(PredeclaredFunctionId(17))",
        1769,
        &[1, 2, 3],
    );
}

#[test]
fn px10_live_alternative_writer_keeps_foreign_members_raw() {
    assert_alternative_writer_raw(
        "px10_answer_live",
        LIVE_CHILD,
        "Predeclared(PredeclaredFunctionId(15))",
        1286,
        &[2, 3, 4],
    );
}

fn assert_alternative_writer_raw(
    name: &'static str,
    source: &'static str,
    foreign_owner: &'static str,
    foreign_result: u32,
    targets: &'static [u32],
) {
    use ken_runtime::ProducerAlternativeAnswerObservation as Answer;
    in_census_thread(name, move || {
        let (built, observed) = ken_runtime::with_producer_alternative_answer_observations(|| {
            emit_below_gate(name, source)
        });
        built.expect("the committed fixture emits beneath its Process gate");
        let writers: Vec<_> = observed
            .iter()
            .filter_map(|row| match row {
                Answer::Writer {
                    owner,
                    result,
                    target,
                    issued_member,
                    raw_answer,
                    returned_carried,
                } => Some((
                    owner,
                    *result,
                    *target,
                    *issued_member,
                    *raw_answer,
                    *returned_carried,
                )),
                Answer::Seat { .. } => None,
            })
            .collect();
        assert!(!writers.is_empty(), "{name}: writer observation must fire");
        assert!(
            writers.iter().all(|row| row.4),
            "{name}: a writer wrapped its answer: {writers:?}"
        );
        let mut foreign: Vec<_> = writers
            .iter()
            .filter(|row| row.0 == foreign_owner && row.1 == foreign_result && row.3)
            .map(|row| row.2)
            .collect();
        foreign.sort_unstable();
        assert_eq!(
            foreign, targets,
            "{name}: each foreign issued member must reach the writer raw: {writers:?}"
        );
    });
}

/// Promise class: durable invariant over these committed source fixtures.
/// MEASURED: each issued-set seat receives a carried SSA word, returns that
/// same word and adds no IR instruction while selecting its Alternatives arm.
/// CLAIMED: the detached seat cannot assert a private labelled-record schema.
/// THE GAP: unrelated instructions outside this arm are not in this window;
/// the pin covers assertions emitted at the exact seat being observed.
#[test]
fn px10_alternative_seats_pass_carried_words_without_assertion() {
    use ken_runtime::ProducerAlternativeAnswerObservation as Answer;
    for (name, source) in [
        ("px10_answer_smoke", SMOKE),
        ("px10_answer_argv", ARGV_ENV_PIPE),
        ("px10_answer_live", LIVE_CHILD),
    ] {
        in_census_thread(name, move || {
            let (built, observed) =
                ken_runtime::with_producer_alternative_answer_observations(|| {
                    emit_below_gate(name, source)
                });
            built.expect("the committed fixture emits beneath its Process gate");
            let seats: Vec<_> = observed
                .iter()
                .filter_map(|row| match row {
                    Answer::Seat {
                        owner,
                        result,
                        members,
                        carried,
                        same_word,
                        instructions_emitted,
                    } => Some((
                        owner,
                        result,
                        members,
                        carried,
                        same_word,
                        instructions_emitted,
                    )),
                    Answer::Writer { .. } => None,
                })
                .collect();
            assert_eq!(
                seats.len(),
                3,
                "{name}: all three issued-set seats must lower: {seats:?}"
            );
            for seat in seats {
                assert_eq!(*seat.2, 2, "{name}: {seat:?}");
                assert!(*seat.3 && *seat.4 && *seat.5 == 0,
                    "{name}: issued-set seat must return its carried word with no schema assertion: {seat:?}");
            }
        });
    }
}

fn in_census_thread(name: &str, run: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name(name.to_owned())
        // The px10 native-build suites measured 3292 KiB resident stack for
        // these programs; 16 MiB is the stack they state.
        .stack_size(16 * 1024 * 1024)
        .spawn(run)
        .expect("spawn census thread")
        .join()
        .expect("census thread");
}

/// Each static region exit (a region's settlement Vis) has exactly one owner
/// per plane, and the planes name exactly `expected`.
fn assert_region_exit_owners(
    name: &str,
    observed: &[ken_runtime::RegionExitOwnerObservation],
    expected: &[(ken_runtime::RegionExitPlane, u32, u32)],
) {
    let mut owners = std::collections::BTreeMap::<_, Vec<u32>>::new();
    for row in observed {
        owners
            .entry((row.plane, row.exit))
            .or_default()
            .push(row.owner);
    }
    for ((plane, exit), found) in &owners {
        assert_eq!(
            found.len(),
            1,
            "{name}: exit {exit} has {found:?} as {plane:?} owners"
        );
    }
    let observed: Vec<_> = observed
        .iter()
        .map(|row| (row.plane, row.exit, row.owner))
        .collect();
    assert_eq!(observed, expected, "{name}: region exit owners");
}

/// Promise class: transition sentinel, retired by
/// PX10-NATIVE-BRANCH-ALTERNATIVES. MEASURED: the smoke plan completes,
/// recording one feasibility diagnostic with no returned-Vis protocol error,
/// and builds an artifact beneath the separate Process admission gate. Its
/// settlement 623 resumes through fold 12 and is returned by Vis 658.
/// MEASURED (B1a D0-7, D0-1-to-D0-7-attribution.txt): region-0 acquire
/// 710→686, body-entry Construct 682→658, outcome merge 641→617, and
/// settlement Construct 647→623 preserve producer and order. The ReturnFold
/// producer remains computational match 12; the ReturnedVisSettlement
/// producer is the same region-0 body-entry Construct, not the fold.
/// CLAIMED: the return fold and returned bracket settlement retain their
/// checked owners. THE GAP: the artifact is not run; this proves no parity.
#[test]
fn px10_smoke_plans_past_return_fold_and_returned_settlement() {
    use ken_runtime::RegionExitPlane::{ReturnFold, ReturnedVisSettlement};
    in_census_thread("px10-census-smoke", || {
        let ((built, owners), diagnostics) =
            ken_runtime::with_static_response_feasibility_diagnostics(|| {
                ken_runtime::with_region_exit_owner_observations(|| {
                    emit_below_gate("px10_census_smoke", SMOKE)
                })
            });
        assert_eq!(
            diagnostics.len(),
            1,
            "the smoke plan completes exactly once"
        );
        for protocol in &diagnostics[0].returned_vis_protocols {
            assert!(
                protocol.error.is_none(),
                "returned-Vis protocol refused: {protocol:?}"
            );
        }
        assert_region_exit_owners(
            "smoke",
            &owners,
            &[(ReturnFold, 623, 12), (ReturnedVisSettlement, 623, 658)],
        );
        let _built = built.expect("smoke emits an artifact beneath the Process gate");
    });
}

/// Promise class: transition sentinel, retired by this WP's Increment 2.
/// MEASURED: argv_env_pipe and live_child build artifacts beneath the separate
/// Process gate after argv's generated context-4 Result contract becomes None.
/// Both retain exact static region-exit owners. CLAIMED: the planner cannot
/// confuse an ordinary successor Vis under Excluded{Relay} with a completed
/// K Ret publication. THE GAP: neither program runs natively here.
#[test]
fn px10_argv_and_live_emit_below_process_gate() {
    use ken_runtime::RegionExitPlane::{ReturnFold, ReturnedVisSettlement};
    for (name, source, expected, seat) in [
        (
            "px10_census_argv_env_pipe",
            ARGV_ENV_PIPE,
            &[
                (ReturnFold, 1160, 571),
                (ReturnFold, 1405, 1165),
                (ReturnedVisSettlement, 1160, 1783),
                (ReturnedVisSettlement, 1405, 1500),
            ][..],
            None,
        ),
        (
            "px10_census_live_child",
            LIVE_CHILD,
            &[
                (ReturnFold, 601, 12),
                (ReturnFold, 846, 606),
                (ReturnedVisSettlement, 601, 1300),
                (ReturnedVisSettlement, 846, 1019),
            ][..],
            None,
        ),
    ] {
        in_census_thread(name, move || {
            let (((built, owners), seat_refusals), diagnostics) =
                ken_runtime::with_static_response_feasibility_diagnostics(|| {
                    ken_runtime::with_detached_result_seat_refusals(|| {
                        ken_runtime::with_region_exit_owner_observations(|| {
                            emit_below_gate(name, source)
                        })
                    })
                });
            if name == "px10_census_argv_env_pipe" {
                let contracts = diagnostics
                    .iter()
                    .flat_map(|diagnostic| &diagnostic.generated_context_result_contracts)
                    .filter(|(context, _)| *context == 4)
                    .collect::<Vec<_>>();
                assert!(!contracts.is_empty(), "the argv K context must be queried");
                assert!(
                    contracts.iter().all(|(_, contract)| contract.is_none()),
                    "an ordinary successor Vis is not a completed K Ret: {contracts:?}"
                );
            }
            let _built = built.expect("px10 program emits an artifact beneath the Process gate");
            assert_eq!(
                seat_refusals,
                seat.into_iter().collect::<Vec<_>>(),
                "{name}: exact downstream detached-result seat"
            );
            assert_region_exit_owners(name, &owners, expected);
        });
    }
}

/// Promise class: transition sentinel for the three B1a G-ERASURE fixtures;
/// re-derive the population if a fixture changes or B2 removes the Process gate.
/// MEASURED: the below-gate native build succeeds, seven actual self-placed
/// checked-IH calls have no response owner and each has an Escaped residual;
/// every installed relay pair has a route. CLAIMED: a self transport cannot
/// mint a response owner or leave an unrouted relay in these three plans.
/// THE GAP: native execution parity remains a B2 obligation, not a B1a test.
fn assert_b1a_g_erasure(name: &'static str, source: String) {
    in_census_thread(name, move || {
        let (built, diagnostics) =
            ken_runtime::with_static_response_feasibility_diagnostics(|| {
                emit_below_gate(name, &source)
            });
        // The old 9807 owner/context refusal is a build error, not a stderr
        // observation; a successful artifact proves that refusal is absent.
        let _built = built.expect("B1a G-ERASURE must build without 9807");
        assert_eq!(diagnostics.len(), 1, "{name}: one completed plan");
        let plan = &diagnostics[0];
        let self_vis: std::collections::BTreeSet<_> =
            plan.self_transport_vis_origins.iter().copied().collect();
        assert_eq!(
            self_vis.len(),
            7,
            "{name}: exact G self-transport population"
        );
        assert_eq!(
            plan.self_transport_vis_origins.len(),
            self_vis.len(),
            "{name}: one placement per self-transport Vis"
        );
        assert!(
            plan.owned_self_transport_vis_origins.is_empty(),
            "{name}: a self transport acquired a response owner: {:?}",
            plan.owned_self_transport_vis_origins
        );
        let escaped: std::collections::BTreeSet<_> = plan
            .static_response_deferred
            .iter()
            .filter(|row| row.sub_case == "EscapedCheckedIhCaller")
            .map(|row| row.vis_origin)
            .collect();
        assert_eq!(
            escaped, self_vis,
            "{name}: each self transport must retain its ordinary Construct route"
        );
        let pairs: std::collections::BTreeSet<_> =
            plan.relay_pair_sources.iter().cloned().collect();
        let routed: std::collections::BTreeSet<_> =
            plan.routed_relay_pair_sources.iter().cloned().collect();
        assert!(
            !pairs.is_empty(),
            "{name}: the fixture must exercise relay edges"
        );
        assert_eq!(
            pairs, routed,
            "{name}: every relay pair has a performing route"
        );
    });
}

#[test]
fn px10_g_erasure_argv_has_unowned_self_transports_and_routed_relays() {
    assert_b1a_g_erasure("px10_g_erasure_argv", ARGV_ENV_PIPE.to_owned());
}

#[test]
fn px10_g_erasure_live_has_unowned_self_transports_and_routed_relays() {
    assert_b1a_g_erasure("px10_g_erasure_live", LIVE_CHILD.to_owned());
}

#[test]
fn px10_g_erasure_null_has_unowned_self_transports_and_routed_relays() {
    let body = ARGV_ENV_PIPE.replacen("        ChildPipe)", "        ChildNull)", 1);
    assert_ne!(body, ARGV_ENV_PIPE, "the stdout disposition moved");
    let null = body.replacen(
        "import Capability.Process.Child (ChildPipe,",
        "import Capability.Process.Child (ChildNull,",
        1,
    );
    assert_b1a_g_erasure("px10_g_erasure_null", null);
}

/// Promise class: durable invariant (PX10 B-closure AC-2). MEASURED: argv's
/// relays 1223 and 992 and live's relays 664 and 433 are relay members of
/// their owners' returned-Vis protocols, numbered once per emitting copy
/// with distinct nonzero discriminants rather than by a response row, and
/// kept in the settled member set. Copies of 1223 and 664 resume at the rows
/// whose Vis are 1448 and 908; copies of 992 and 433 resume by their planned
/// named trap. CLAIMED: each named relay has a numbered copy with the
/// expected resume, and copies retain separate discriminants. THE GAP: this
/// census does not prove that every emitting owner minted a copy or that
/// other copies' resumes agree with their issuers; the artifact is not run,
/// the expected seat is read from the plane that chose it, and a trap's
/// reason text is not projected. These origin pins are re-derived by producer
/// and order on the B1a tree against `B-attribution-r2-structural-comparison.txt`
/// (SHA256 2b45cce5e87da7f616767ea12772d28abdbb74143000f014853e907546babc06),
/// which found equal owner/relay/resume/successor structure under that mapping.
#[test]
fn px10_relays_are_numbered_protocol_members() {
    for (name, source, relays) in [
        (
            "px10_census_relay_argv_env_pipe",
            ARGV_ENV_PIPE,
            [(1223, 1687, Some(1448)), (992, 1783, None)],
        ),
        (
            "px10_census_relay_live_child",
            LIVE_CHILD,
            [(664, 1204, Some(908)), (433, 1300, None)],
        ),
    ] {
        in_census_thread(name, move || {
            let (built, diagnostics) =
                ken_runtime::with_static_response_feasibility_diagnostics(|| {
                    emit_below_gate(name, source)
                });
            let _built = built.expect("px10 program emits an artifact beneath the Process gate");
            assert_eq!(diagnostics.len(), 1, "{name}: one completed plan");
            let plan = &diagnostics[0];
            let mut discriminants = std::collections::BTreeSet::new();
            for (relay, owner, resume_vis) in relays {
                let protocol = plan
                    .returned_vis_protocols
                    .iter()
                    .find(|protocol| protocol.owner_origin == owner)
                    .unwrap_or_else(|| panic!("{name}: owner {owner} has a returned-Vis protocol"));
                assert!(protocol.error.is_none(), "{name}: {protocol:?}");
                let members = protocol
                    .contexts
                    .iter()
                    .flat_map(|(_, members)| members)
                    .filter(|member| member.origin == relay)
                    .collect::<Vec<_>>();
                assert_eq!(
                    members.len(),
                    1,
                    "{name}: relay {relay} is one member of {owner}"
                );
                assert!(
                    members[0].relay,
                    "{name}: {relay} is a relay member: {:?}",
                    members[0]
                );
                assert_eq!(
                    members[0].successor_id, None,
                    "{name}: relay {relay} is numbered by its own origin, not a response row"
                );
                assert!(
                    plan.settled_returned_vis_protocols
                        .iter()
                        .any(|(settled, members)| *settled == owner && members.contains(&relay)),
                    "{name}: relay {relay} stays in owner {owner}'s settled member set"
                );
                let numbered = plan
                    .settled_relays
                    .iter()
                    .filter(|(origin, _, _)| *origin == relay)
                    .collect::<Vec<_>>();
                assert!(!numbered.is_empty(), "{name}: relay {relay} is numbered");
                for (_, discriminant, _) in &numbered {
                    assert_ne!(*discriminant, 0, "{name}: zero is never a Vis member");
                    assert!(
                        discriminants.insert(*discriminant),
                        "{name}: relay {relay} shares discriminant {discriminant}"
                    );
                }
                assert!(
                    numbered
                        .iter()
                        .any(|(_, _, resume)| resume.map(|(_, row_vis)| row_vis) == resume_vis),
                    "{name}: relay {relay} has a copy resuming at {resume_vis:?} (None is a planned named trap): {numbered:?}"
                );
            }
        });
    }
}

/// Promise class: transition sentinel for B1a's exact original-fixture
/// projection (RT-NATIVE-BRACKET-RETURN-CLAUSE AC-1). MEASURED (B1a D0-4):
/// the live carried-stack seats Vis 1307/1429 and null seats Vis 1806/1928
/// descend from retired main matches. Their DropBracketReturnClause mutation
/// applies zero times on each B1a fixture; the unmutated Let-body build emits.
/// CLAIMED: a performed bind lowers its Ret body under the original region
/// eliminators, without a carried re-emission stack. THE GAP: the Let route's
/// BracketReturnClause check reads the clause with its frame present, but this
/// projection does not inject a missing Let-route clause. F native/interpreter
/// parity and Buffer1 release order were measured, not committed, in B1a.
/// Scratch M-B1A restores the old carried seats and their exact refusal.
#[test]
fn live_child_has_no_carried_return_clause_drop_under_b1a() {
    assert_dropped_return_clause_refuses("px10_return_clause_live_child", LIVE_CHILD.to_owned());
}

/// Promise class: transition sentinel; the null-stdout row of the pin above.
#[test]
fn null_stdout_has_no_carried_return_clause_drop_under_b1a() {
    let null_body = ARGV_ENV_PIPE.replacen("        ChildPipe)", "        ChildNull)", 1);
    assert_ne!(
        null_body, ARGV_ENV_PIPE,
        "the argv fixture's stdout disposition moved"
    );
    let null = null_body.replacen(
        "import Capability.Process.Child (ChildPipe,",
        "import Capability.Process.Child (ChildNull,",
        1,
    );
    assert_dropped_return_clause_refuses("px10_return_clause_null_stdout", null);
}

fn assert_dropped_return_clause_refuses(name: &'static str, source: String) {
    in_census_thread(name, move || {
        emit_below_gate(name, &source)
            .expect("the unmutated program emits beneath the Process gate");
        let (built, applications) = ken_runtime::with_handler_owned_deferred_response_mutation(
            ken_runtime::HandlerOwnedDeferredResponseMutation::DropBracketReturnClause,
            || emit_below_gate(name, &source),
        );
        assert_eq!(
            applications, 0,
            "{name}: the retired carried stack must not provide a drop seat"
        );
        let dropped = ken_runtime::dropped_bracket_return_clauses();
        assert!(dropped.is_empty(), "{name}: no carried return clause was dropped: {dropped:?}");
        built.expect("the no-op carried-stack mutation leaves the Let-body build available");
    });
}
