//! Specialization marker/generated-context creation through composed-call
//! binding resolution, end-to-end (`RT-CONTROL-INTEGRATION-TESTS-SPLIT` D1,
//! module 3 of 5, split from `control.rs`: `d5a_*`,
//! `continuation_case_binder_run_*`, `d4b_*`, `d3b_*`/`d3c_*`, `contsrc_*`,
//! `ced_d2_*` (first wave), `d6a_*` (second wave),
//! `erasing_a_seat_key_axis_*`, `d4a_*`, `d7a_*`,
//! `d8a_*`/`d8b_*`/`d8d_*`/`d8e_*`/`d8h_*`/`d8i_*`/`d8j_*`/`d8k_*`).

use super::*;
use super::control::px8j_capture_source_trace;
use crate::cranelift_backend::lowering::units::{
    continuation_case_binder_run, ContinuationCaseBinderSource,
};

/// **`RT-DECL-CLOSURE-PORT` `D5a` — the witness compiles, and its checked-IH
/// marker is consumed at the static-worker call edge exactly once.**
///
/// ⭐⭐ **This row was a localization scaffold asserting `outcome.is_err()`, and
/// it went red the moment the route became positive — which is exactly what it
/// was for.** It is now a real acceptance row: the `px8tr_nested_post_effect`
/// object emits, and the marker's ordered event log shows the consumption
/// happening **before** the worker call instruction exists.
///
/// **Promise class: durable invariant.** The subject is a relation — consumption
/// precedes emission, with the identities the checked plan names. Adding
/// specializations, fields or inputs keeps it green; consuming on the returned
/// word, consuming twice, or consuming under a different template reds it.
///
/// ⚠ The census and trace are still printed. They cost nothing on a green run
/// and are the first thing anyone wants when this goes red.
///
/// ## Why the trace scaffold is RETAINED, at checkpoint 4's close
///
/// The frame allows retiring the localization trace *only where a discriminator
/// now bears its claim*. Almost everything it used to carry has moved to
/// structured evidence: the population census to
/// `d5a_the_final_executable_population_is_the_emittable_set_minus_the_superseded_bodies`,
/// the capture projection to
/// `d5a_a_specialization_owned_edge_separates_root_provenance_from_its_immediate_slot`,
/// the context binding to the three binding rows, and the operand suffix to the
/// prefix relation in the marker event log.
///
/// ⛔ **One claim has no other bearer: the assembled binder run below.** The
/// `binder_index: 0` above proves entry 0 reaches the `StaticWorker` — the half
/// that matters most, and it is checked against the plan rather than against
/// this string — but nothing else measures that the run continues
/// `[field, input, input]` and stops there. `continuation_case_binder_run_*`
/// measures the *law* and stays green if production stops calling it. ⇒ The
/// trace stays until a discriminator carries that shape, and this paragraph is
/// the reason rather than an oversight.
#[test]
fn d5a_the_landed_object_fixture_consumes_its_ih_marker_before_emitting_the_worker_call() {
    reset_d5a_trace();
    reset_d5a_marker_events();
    let outcome = crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
        "d5a_localization",
        false,
    )
    .map(|_| ())
    .map_err(|error| format!("{error:?}"));
    // The planner-issued continuation-call census, taken independently of the
    // run above so a missing token and an unreached claim are distinguishable.
    let (entry_expr, declarations) =
        crate::cranelift_backend::test_objects::px8tr_nested_post_effect_planning_inputs();
    let declarations = declarations
        .iter()
        .map(|declaration| (declaration.symbol.as_str(), declaration))
        .collect::<BTreeMap<_, _>>();
    let census = plan_static_transition_graph_with_symbols(
        &entry_expr,
        &declarations,
        &crate::NativeProcessSymbols::legacy_prelude(),
        AbiRootIngress::Value,
        true,
    )
    .map(|plan| {
        let units = plan
            .emittable_units()
            .expect("units")
            .into_iter()
            .map(|unit| format!("{:?}@{:?}={:?}", unit.function(), unit.body_occurrence(), unit.definition()))
            .collect::<Vec<_>>();
        let calls = plan
            .continuation_calls()
            .expect("continuation calls")
            .iter()
            .map(|call| {
                format!(
                    "provenance={:?} EMITS-FROM={:?} result_root={:?} construct={:?} \
                     continuation={:?} alt={} pos={} target={:?}",
                    call.producer_owner(),
                    call.emission_owner(),
                    call.producer_result_origin(),
                    call.producer_construct_origin(),
                    call.continuation_origin(),
                    call.producer_alternative(),
                    call.recursive_position(),
                    call.target(),
                )
            })
            .collect::<Vec<_>>();
        // The capture projection of every planned specialization, taken from
        // the plan alone. ⭐ Independent of the emission run, so "the projection
        // does not resolve at its producer owner" is a fact about the planner
        // and not an artifact of where emission happened to abort.
        let specializations = plan
            .continuation_units()
            .expect("continuation units")
            .iter()
            .map(|unit| {
                format!(
                    "{:?} provenance={:?} EMITS-FROM={:?} consumer={:?} ordinary_params={} \
                     inputs={:?}",
                    unit.id(),
                    unit.producer_owner(),
                    unit.emission_owner(),
                    unit.consumer_owner(),
                    unit.ordinary_parameters(),
                    unit.continuation_inputs()
                        .expect("continuation inputs")
                        .iter()
                        // ⭐ ROOT provenance and IMMEDIATE slot side by side.
                        // Printing only the root pair would leave the whole
                        // point of `D5a` -- that the two differ for a
                        // specialization-owned edge -- invisible, and a reader
                        // would have to infer it from the emission not failing.
                        .map(|input| {
                            (
                                input.coordinate.expect_entry_abi().0,
                                input.coordinate.expect_entry_abi().1,
                                input.availability.expect_direct_emission_slot(),
                            )
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        // `D5a`: the generated producer execution contexts. ⭐ Printed from the
        // plan alone, so their existence, their enclosing specialization and
        // their ABI shape are POSITIVELY measured -- not inferred from the raw
        // worker's seat going quiet, which is a negative check with no control.
        let contexts = plan
            .continuation_contexts()
            .expect("continuation contexts")
            .iter()
            .map(|context| {
                format!(
                    "{:?} enclosing={:?} worker_body={:?} raw_owner={:?} params={} captures={} \
                     roots={:?}",
                    context.id(),
                    context.enclosing_specialization(),
                    context.worker_body_origin(),
                    context.raw_owner(),
                    context.header().parameters,
                    context.header().captures,
                    context
                        .captures()
                        .expect("context captures")
                        .iter()
                        .map(|input| {
                            let (owner, position, _) = input.coordinate.expect_entry_abi();
                            (owner, position)
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        // `D5a` checkpoint 1: the template-only set and the executable
        // population, printed from the plan alone. ⭐ Positively measured: the
        // alternative is to read "fn2 is still emitted" off the refusal that
        // follows, which cannot distinguish "the rule kept it" from "the rule
        // never ran".
        let population = format!(
            "template_only={:?} executable={:?} emittable={:?}",
            plan.template_only_worker_bodies().expect("template only"),
            plan.executable_units()
                .expect("executable units")
                .iter()
                .map(|unit| unit.function())
                .collect::<Vec<_>>(),
            plan.emittable_units()
                .expect("units")
                .iter()
                .map(|unit| unit.function())
                .collect::<Vec<_>>(),
        );
        let envelopes = plan
            .emittable_units()
            .expect("units")
            .into_iter()
            .map(|unit| {
                format!(
                    "{:?} params={} captures={}",
                    unit.function(),
                    unit.header().parameters,
                    unit.header().captures,
                )
            })
            .collect::<Vec<_>>();
        (units, calls, specializations, contexts, population, envelopes)
    });
    let trace = take_d5a_trace();
    eprintln!("=== D5a PLANNER CENSUS ===");
    match &census {
        Ok((units, calls, specializations, contexts, population, envelopes)) => {
            eprintln!("units ({}):", units.len());
            for unit in units {
                eprintln!("  {unit}");
            }
            eprintln!("continuation calls ({}):", calls.len());
            for call in calls {
                eprintln!("  {call}");
            }
            eprintln!("specializations ({}):", specializations.len());
            for specialization in specializations {
                eprintln!("  {specialization}");
            }
            eprintln!("generated contexts ({}):", contexts.len());
            for context in contexts {
                eprintln!("  {context}");
            }
            eprintln!("executable population:");
            eprintln!("  {population}");
            eprintln!("emitting-unit envelopes ({}):", envelopes.len());
            for envelope in envelopes {
                eprintln!("  {envelope}");
            }
        }
        Err(error) => eprintln!("  planning failed: {error:?}"),
    }
    eprintln!("=== D5a TRACE ({} entries) ===", trace.len());
    for entry in &trace {
        eprintln!("{entry}");
    }
    let events = d5a_marker_events();
    eprintln!("=== outcome: {outcome:?} ===");
    eprintln!("=== D5a MARKER EVENTS ({}) ===", events.len());
    for event in &events {
        eprintln!("  {event:?}");
    }
    outcome.expect(
        "the landed object fixture emits. Once the checked-IH marker is consumed \
         at the static-worker call edge there is no remaining refusal on this \
         route, and a failure here is a regression in the ported route rather \
         than an unfinished checkpoint",
    );

    // The ordered claim: every consumption sits at a call edge, immediately
    // ahead of the instruction it discharges.
    //
    // ⚠ Stated in this direction and NOT the converse. An ordinary static worker
    // call carries no marker and must stay untouched, so a `WorkerCallEmitted`
    // with no consumption before it is lawful — see
    // `d5a_an_unmarked_static_worker_call_is_untouched_by_the_marker_seam`.
    let consumptions = events
        .iter()
        .enumerate()
        .filter(|(_, event)| matches!(event, D5aMarkerEvent::Consumed { .. }))
        .collect::<Vec<_>>();
    assert!(
        !consumptions.is_empty(),
        "the witness must consume its checked-IH marker at the static-worker \
         call edge; an empty log means the seam was never reached and every \
         claim below would be vacuous. Events: {events:?}"
    );
    for (index, event) in &consumptions {
        assert_eq!(
            **event,
            D5aMarkerEvent::Consumed {
                call_template_id: 100,
                slot_template_id: 200,
                binder_index: 0,
                arity: 1,
            },
            "every consumption must be under the exact templates the fixture's \
             checked plan issues — call 100 naming slot 200, whose method binder \
             ordinal is 0, at arity 1"
        );
        assert!(
            matches!(
                events.get(index + 1),
                Some(D5aMarkerEvent::WorkerCallEmitted { .. })
            ),
            "the marker denotes the APPLICATION, so the very next event after \
             consuming it is the call instruction it discharges. Anything else \
             means it was consumed somewhere other than the call edge — and \
             consuming it AFTER the call would mean reading it off the returned \
             boundary word, the carrier decode the ruling forbids. Events: \
             {events:?}"
        );
    }

    // **MEASURED** by `continuation_case_binder_run_*` below: the binder-run law
    // produces the ruled order for the witness's exact coordinates.
    // **CLAIMED** by this checkpoint: the specialization body is *built* in that
    // order. **THE GAP**: a correct law that production does not consume. Those
    // controls call the plan function directly, so every one of them stays green
    // if `define_continuation_bodies` stops calling it.
    //
    // ⭐ The `binder_index: 0` above is now an INDEPENDENT second witness for the
    // same claim: the checked plan carries `recursive_position` (1) and
    // `method_binder_ordinal` (0) as separate fields, so a lowering that
    // conflated them would disagree with the plan and refuse. This assertion
    // reads the environment the definition actually assembled.
    //
    // ⭐ **`RT-CONTSRC-PRODUCER-LOCAL` `D6a` — this row also pins the CALL
    // ROUTE pair, because this witness is the one that separates them, and it
    // is the future `D6b` discriminator.** Its planner issues a generated
    // execution context for specialization 0 and that unit resolves it, so by
    // the route law on `StaticWorkerCallRoute` this body's induction hypothesis
    // renders `GeneratedContext` while its selected recursive constructor
    // argument renders `RawWorker`. The two bindings are otherwise identical --
    // same closure occurrence, body origin, declared arity and captures -- so
    // without the route in the rendering the pair is indistinguishable.
    //
    // ⛔ The assertion is on the EXACT pair, deliberately. It is not "the two
    // routes differ": that phrasing would be satisfied by the wrong assignment
    // as readily as the right one, and its negation -- equal rendered routes --
    // is a LAWFUL state elsewhere (the route-degenerate governed witness, where
    // no context is issued and both members carry `RawWorker`). So equal routes
    // must never be read as a reused binding, here or anywhere; what this row
    // pins is that *this* unit, which did resolve a context, assigns
    // `GeneratedContext` to the hypothesis and `RawWorker` to the argument.
    assert!(
        trace.iter().any(|entry| entry.contains(
            "env=[StaticWorker(GeneratedContext), Carried, StaticWorker(RawWorker), Carried, \
             Carried]"
        )),
        "the specialization body must be assembled in the ruled order — the IH \
         prefix first, then ALL the constructor arguments in source order (the \
         nonrecursive field at position 0, then the selected recursive argument \
         at position 1), then the two continuation inputs. Every rejected shape \
         is visible in this one vector: `env=[StaticWorker, Carried, Carried]` \
         omitted the nonrecursive field entirely; `env=[Carried, StaticWorker, \
         Carried, Carried]` read `recursive_position` as a lexical index; \
         `env=[StaticWorker(..), Carried, Carried, Carried]` is the pre-`D6a` \
         run that replaced the recursive argument with its own IH and shifted \
         both continuation inputs one slot early; and `RawWorker` on the \
         leading member is this context-resolving unit failing to route its \
         induction hypothesis through the context it resolved. (⛔ That last \
         reading is specific to THIS unit. Equal routes are lawful wherever no \
         context was issued, and are never by themselves evidence of a reused \
         binding.) Trace: {trace:?}"
    );
}

/// **`RT-DECL-CLOSURE-PORT` `D5a` — suppressing the consumption restores the
/// pending refusal.**
///
/// ⭐ The claim under test is that the positive route **depends on** the
/// consumption, not merely that it coexists with one. Without this row, the
/// green acceptance above is equally consistent with the seam being inert and
/// something else having fixed the route.
///
/// The mutation withholds only the consumption; the call is still emitted,
/// lawfully and unchanged. So this also states *where* the refusal lives: at
/// closeout, on a marker nobody discharged — not at the consumer.
///
/// **Promise class: durable invariant.**
#[test]
fn d5a_suppressing_the_marker_consumption_restores_the_pending_closeout_refusal() {
    reset_d5a_marker_events();
    let refusal = with_d5a_marker_mutation(D5aMarkerMutation::SuppressConsumption, || {
        crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
            "d5a_suppressed",
            false,
        )
        .map(|_| ())
        .map_err(|error| format!("{error:?}"))
        .expect_err(
            "with the consumption withheld the marker is still pending at \
             closeout, so the ported route must refuse. A compile here means \
             the consumption is inert and the acceptance row is green for \
             the wrong reason",
        )
    });
    assert!(
        refusal.contains("a checked computational-IH marker is a specialized-only surface"),
        "a PENDING marker must keep the specialized-template path and its \
         fail-closed carried refusal, unchanged. A different refusal would mean \
         this row stopped measuring the closeout arm: {refusal}"
    );
    let events = d5a_marker_events();
    assert!(
        !events
            .iter()
            .any(|event| matches!(event, D5aMarkerEvent::Consumed { .. })),
        "the mutation must withhold the consumption and nothing else: {events:?}"
    );
    assert!(
        events
            .iter()
            .any(|event| matches!(event, D5aMarkerEvent::WorkerCallEmitted { .. })),
        "the call itself must still be emitted. If the mutation also suppressed \
         the call, the refusal above would be about a route that never ran: \
         {events:?}"
    );
}

/// **`RT-DECL-CLOSURE-PORT` `D5a` — every identity the consumer requires refuses
/// BEFORE the worker call is emitted.**
///
/// ⭐ The mutations land on the **checked plan**, not on the consumer. That is
/// the discriminating choice: perturbing the consumer would ask whether it
/// agrees with itself, while perturbing the plan asks whether it is really
/// reading the checked templates it claims to.
///
/// **Promise class: durable invariant.** Each row asserts that one specific
/// disagreement between the plan and the emitted application is refused, and
/// names its own message so a later refusal cannot stand in for it.
#[test]
fn d5a_a_plan_the_application_disagrees_with_refuses_before_the_worker_call() {
    // ⛔ Each mutation RE-SEALS the template's binding fingerprint, so the plan
    // it produces is internally consistent and merely disagrees with the
    // program. Without that, every row is refused by the plan's own ingest
    // check ("computational IH call binding is inconsistent") and is green for
    // a reason that has nothing to do with this seam — measured, not assumed.
    fn reseal(plan: &mut crate::OrientedSubcontinuationPlanV1) {
        for call in &mut plan.computational_ih_calls {
            call.occurrence_binding_fingerprint = 0;
            call.occurrence_binding_fingerprint =
                crate::compiler_private_computational_ih_call_binding_fingerprint(call);
        }
        for slot in &mut plan.computational_ih_slots {
            slot.occurrence_binding_fingerprint = 0;
            slot.occurrence_binding_fingerprint =
                crate::compiler_private_computational_ih_slot_binding_fingerprint(slot);
        }
    }
    // (label, mutation, the refusal it must produce)
    #[allow(clippy::type_complexity)]
    let rows: Vec<(
        &str,
        Box<dyn Fn(&mut crate::OrientedSubcontinuationPlanV1)>,
        &str,
    )> = vec![
        (
            "stale arity",
            Box::new(|plan: &mut crate::OrientedSubcontinuationPlanV1| {
                plan.computational_ih_calls[0].arity = 2;
                reseal(plan);
            }),
            "names arity 2",
        ),
        // ⚠ **MEASURED**: this row is refused by the plan's own ingest
        // cross-reference check, *"computational IH call names a stale slot
        // template"* — not by the consumer's slot lookup. **THE GAP**: the
        // consumer's `ok_or_else` on that lookup is therefore unreachable
        // through any plan a compile will accept, and no control here reds it.
        // It is kept because the ruling names slot resolution as one of the
        // three identities, and because the ingest check is a different
        // authority that a later refactor could move — but it is defensive,
        // and this row measures ingest rather than the consumer. Stated so
        // nobody reads it as evidence for the consumer.
        (
            "a slot the plan does not hold",
            Box::new(|plan: &mut crate::OrientedSubcontinuationPlanV1| {
                plan.computational_ih_calls[0].slot_template_id = 999;
                reseal(plan);
            }),
            "computational IH call names a stale slot template",
        ),
        (
            "a method binder ordinal the call does not read",
            Box::new(|plan: &mut crate::OrientedSubcontinuationPlanV1| {
                plan.computational_ih_slots[0].method_binder_ordinal = 1;
                reseal(plan);
            }),
            "method ordinal 1 is outside the invocation's source-to-runtime binder map",
        ),
    ];
    for (label, mutate, expected) in rows {
        reset_d5a_marker_events();
        let refusal = crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object_with_plan(
            "d5a_plan_mutation",
            false,
            |plan| mutate(plan),
        )
        .map(|_| ())
        .map_err(|error| format!("{error:?}"))
        .err()
        .unwrap_or_else(|| {
            panic!(
                "{label}: the emitted application no longer matches its checked \
                 template, so it must be refused. A compile means the consumer \
                 is not reading that field at all"
            )
        });
        assert!(
            refusal.contains(expected),
            "{label}: must get this seam's OWN refusal, not a later one it \
             happens to also trip. Otherwise the row names a check that never \
             ran: {refusal}"
        );
        let events = d5a_marker_events();
        assert!(
            !events
                .iter()
                .any(|event| matches!(event, D5aMarkerEvent::Consumed { .. })),
            "{label}: a rejected consumption must leave the marker pending, never \
             discharge it: {events:?}"
        );
        assert!(
            !events
                .iter()
                .any(|event| matches!(event, D5aMarkerEvent::WorkerCallEmitted { .. })),
            "{label}: the refusal must land BEFORE the call instruction is \
             written. An emitted call means a mis-identified application was \
             already committed: {events:?}"
        );
    }

    // ⚠ The consumer's OWN arity guard, made reachable.
    //
    // On this witness the marker wraps the very call that reaches the consumer,
    // so entry and the consumer read the same two numbers and entry always
    // refuses first — the "stale arity" row above is measuring ENTRY. Relaxing
    // entry is what lets the consumer's guard be red rather than merely
    // asserted; it is ruled separately and is load-bearing wherever a marker's
    // wrapped call is not the one that reaches a static worker.
    reset_d5a_marker_events();
    let refusal = with_d5a_marker_mutation(D5aMarkerMutation::RelaxEntryArity, || {
        crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object_with_plan(
            "d5a_consumer_arity",
            false,
            |plan| {
                plan.computational_ih_calls[0].arity = 2;
                for call in &mut plan.computational_ih_calls {
                    call.occurrence_binding_fingerprint = 0;
                    call.occurrence_binding_fingerprint =
                        crate::compiler_private_computational_ih_call_binding_fingerprint(call);
                }
            },
        )
        .map(|_| ())
        .map_err(|error| format!("{error:?}"))
        .expect_err("the consumer's own arity guard must refuse the application")
    });
    assert!(
        refusal.contains("but the static worker call applies 1 arguments"),
        "with entry relaxed, the refusal must be the CONSUMER's arity guard — \
         entry's message names the marker's wrapped call instead: {refusal}"
    );
    assert!(
        !d5a_marker_events()
            .iter()
            .any(|event| matches!(event, D5aMarkerEvent::WorkerCallEmitted { .. })),
        "the consumer's arity guard must also refuse before emission"
    );
}

/// **`RT-DECL-CLOSURE-PORT` `D5a` — an ordinary static worker call is untouched
/// by the marker seam.**
///
/// ⭐ The whole seam is keyed on a *pending* marker, so the claim that matters
/// for everything already landed is a negative one: a program with no checked
/// plan at all reaches the same static-worker call and consumes nothing. Without
/// this row, "the consumer only fires under a marker" is an inference from
/// reading the code rather than a measurement.
///
/// The witness is the existing `RT-WORKER-BIND` one, which carries zero
/// continuation machinery.
///
/// **Promise class: durable invariant.**
#[test]
fn d5a_an_unmarked_static_worker_call_is_untouched_by_the_marker_seam() {
    reset_d5a_marker_events();
    let compiled = crate::cranelift_backend::artifact::compile_expr_for_lowering_tests(
        &super::constructors::static_worker_witness(true),
        &NativeSeedEnvironment::empty(),
    )
    .expect("the ordinary static-worker witness compiles, exactly as it did before");
    compiled.run(None).expect("and runs");
    let events = d5a_marker_events();
    assert!(
        events
            .iter()
            .any(|event| matches!(event, D5aMarkerEvent::WorkerCallEmitted { .. })),
        "the witness must actually reach the static-worker call edge, or this \
         row proves nothing about it: {events:?}"
    );
    assert!(
        !events
            .iter()
            .any(|event| matches!(event, D5aMarkerEvent::Consumed { .. })),
        "an unmarked call must consume nothing. A consumption here would mean \
         the seam fires on programs that never had a checked-IH marker: \
         {events:?}"
    );
}

/// **`RT-DECL-CLOSURE-PORT` `D5a` — the ruled computational-case binding law.**
///
/// The subject is [`continuation_case_binder_run`]: given the planner's own
/// coordinates, which environment slot does each operand take? The order **is**
/// the property, which is why these controls read a plan rather than assembled
/// Cranelift operands — a `Value` cannot exist without a live `FunctionBuilder`,
/// so an operand-level control could only re-run the pipeline and read a
/// refusal, which is red-vs-red and states nothing about the order.
///
/// **Promise class: durable invariant.** Each row asserts a relation between the
/// planner's coordinates and the produced run. An extension that admits a second
/// projected worker, more fields, or a different envelope layout keeps every one
/// of these green; only a change to the binding law itself reds them, and that
/// is a contract decision.
#[test]
fn continuation_case_binder_run_puts_the_ih_prefix_first_at_a_nonzero_recursive_position() {
    // The exact `px8tr_nested_post_effect` witness shape: `Vis(unit, k)` with
    // the recursive field at source position 1, one nonrecursive field at
    // source position 0, and two continuation inputs.
    let run = continuation_case_binder_run(
        2,
        &[1],
        1,
        &[ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField { source_position: 0 }],
        2,
    )
    .expect("the witness's own coordinates are a lawful binder run");

    assert_eq!(
        run,
        vec![
            ContinuationCaseBinderSource::InductionHypothesis,
            ContinuationCaseBinderSource::Ordinary(0),
            ContinuationCaseBinderSource::SelectedRecursiveArgument { source_position: 1 },
            ContinuationCaseBinderSource::ContinuationInput(0),
            ContinuationCaseBinderSource::ContinuationInput(1),
        ],
        "the IH prefix leads, then EVERY constructor argument in source order — \
         the nonrecursive field at position 0 and the selected recursive \
         argument at position 1 — and the continuation inputs tail in ordinal \
         order"
    );

    // ⭐ Stated separately because these three are the ruled discriminator, and
    // an exact-vector assertion alone does not say which part of it was the
    // defect.
    assert_eq!(
        run[0],
        ContinuationCaseBinderSource::InductionHypothesis,
        "`Var(0)` is the projected worker even though the recursive SOURCE \
         position is 1 — reading `recursive_position` as a lexical index puts \
         the ordinary field here, and the measured consequence was \
         `Unsupported(Call, \"callee is not a closure\")` on a `Unit`"
    );
    assert_eq!(
        run[1],
        ContinuationCaseBinderSource::Ordinary(0),
        "the nonrecursive field is read at its IH-offset position, not at its \
         constructor source position"
    );
    // ⛔ `RT-CONTSRC-PRODUCER-LOCAL` `D6a` — the recursive argument is a member
    // of the run in its own right, NOT a position the IH prefix already stands
    // for. Skipping it is what shifted every later binder down one slot, so the
    // ordinal of the FIRST continuation input is the load-bearing consequence:
    // it is 3 here, and the pre-`D6a` construction put it at 2.
    assert_eq!(
        run[2],
        ContinuationCaseBinderSource::SelectedRecursiveArgument { source_position: 1 },
        "the selected recursive constructor argument occupies its own source \
         position in the argument segment; the IH standing in for it is the \
         defect `D6a` repairs"
    );
    assert_eq!(
        run[3],
        ContinuationCaseBinderSource::ContinuationInput(0),
        "the outer-frame tail therefore begins one slot LATER than it did \
         before `D6a` — the shift is the observable half of the repair, and a \
         row asserting only the new member's presence would not see it"
    );
}

/// At recursive source position **0** the two readings coincide — which is
/// exactly why the defect needed a nonzero position to surface, and why this row
/// alone would have been a false green.
#[test]
fn continuation_case_binder_run_agrees_with_the_rejected_reading_at_source_position_zero() {
    let run = continuation_case_binder_run(
        2,
        &[0],
        0,
        &[ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField { source_position: 1 }],
        1,
    )
    .expect("a source-position-zero recursive field is a lawful binder run");

    assert_eq!(
        run,
        vec![
            ContinuationCaseBinderSource::InductionHypothesis,
            ContinuationCaseBinderSource::SelectedRecursiveArgument { source_position: 0 },
            ContinuationCaseBinderSource::Ordinary(0),
            ContinuationCaseBinderSource::ContinuationInput(0),
        ],
        "with the recursive field at source position 0, the IH prefix and the \
         rejected lexical reading both put the worker at slot 0; the recursive \
         argument then leads the argument segment, because source order is \
         source order"
    );
}

/// The envelope is a **role list**, so a field's index in it is not its
/// constructor source position. This row separates the two readings by permuting
/// the envelope — the only construction that can tell them apart.
#[test]
fn continuation_case_binder_run_resolves_a_field_by_source_position_not_envelope_ordinal() {
    let run = continuation_case_binder_run(
        3,
        &[1],
        1,
        &[
            ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField { source_position: 2 },
            ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField { source_position: 0 },
        ],
        0,
    )
    .expect("a permuted envelope covering every nonrecursive field is lawful");

    assert_eq!(
        run,
        vec![
            ContinuationCaseBinderSource::InductionHypothesis,
            ContinuationCaseBinderSource::Ordinary(1),
            ContinuationCaseBinderSource::SelectedRecursiveArgument { source_position: 1 },
            ContinuationCaseBinderSource::Ordinary(0),
        ],
        "constructor arguments follow SOURCE order (0, then the recursive 1, \
         then 2) while each nonrecursive one's operand is fetched from its own \
         envelope index (1 then 0); an implementation that walked the envelope \
         in order would produce `[IH, Ordinary(0), .., Ordinary(1)]`"
    );
}

/// ⛔ Every gap is a hard stop. A gap-filled run silently shifts every later
/// binder, which is a wrong program rather than a refused one — so each guard
/// asserts its own message, never merely `is_err`.
#[test]
fn continuation_case_binder_run_hard_stops_rather_than_leaving_a_hole() {
    let missing_field = continuation_case_binder_run(2, &[1], 1, &[], 0)
        .expect_err("an envelope covering no field cannot build a binder run");
    assert!(
        format!("{missing_field:?}").contains("has no nonrecursive field at source position 0"),
        "a missing role must name the exact source position it could not \
         resolve: {missing_field:?}"
    );

    let out_of_range = continuation_case_binder_run(1, &[3], 3, &[], 0)
        .expect_err("a recursive position outside the binder run cannot be bound");
    assert!(
        format!("{out_of_range:?}").contains("outside its own 1-binder run"),
        "an out-of-range recursive position must name the run it left: \
         {out_of_range:?}"
    );

    let worker_not_recursive = continuation_case_binder_run(
        2,
        &[1],
        0,
        &[ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField { source_position: 0 }],
        0,
    )
    .expect_err("a worker standing for no recursive field cannot be bound");
    assert!(
        format!("{worker_not_recursive:?}")
            .contains("stands for no induction hypothesis"),
        "a worker whose ruled position is not recursive must refuse rather than \
         take a slot: {worker_not_recursive:?}"
    );

    // ⚠ This is also the reason segment 1's reversal is **unobservable**: a
    // second recursive position has no projected worker, so no accepted case
    // ever has more than one IH, and reversed order coincides with forward
    // order. The clause is written as the law states it so that admitting a
    // second worker later is a change to the projection, not to this order.
    let second_position = continuation_case_binder_run(
        2,
        &[0, 1],
        1,
        &[ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField { source_position: 0 }],
        0,
    )
    .expect_err("a specialization projects exactly one worker");
    assert!(
        format!("{second_position:?}")
            .contains("projects no worker for"),
        "a second recursive position must hard-stop rather than reuse the one \
         projected worker: {second_position:?}"
    );
}

// ── `RT-DECL-CLOSURE-PORT` `D5a` checkpoint 4 — the ruled discriminators ─────
//
// Every row below runs against the POSITIVE route. The frame forbids
// red-versus-red evidence, so a control that merely reproduces a refusal the
// fixture already had is not admissible here: each mutation must move a green
// compile to a named refusal, or measure a plan fact the emission run consumes.

/// **`D5a` — root provenance and the immediate slot are two coordinates, and the
/// witness makes them genuinely differ.**
///
/// The ruling splits one question into two: *which owner's ABI position is this
/// value's root provenance*, and *where does the environment now emitting hold
/// it*. For a `Predeclared` emitter the two coincide by construction and the
/// production code enforces that as a consistency law. For a `Specialization`
/// emitter they are indices into different environments, and comparing them
/// would be the reverse map `evt_609am4v7cdt5b` forbids.
///
/// ⭐ **This row is the positive control for every other row in this group.**
/// If the fixture ever degenerated so that the two coordinates agreed
/// everywhere, the `Specialization` arm would read either field and get the
/// same answer, and a lowering bug that swapped them would be undetectable —
/// silently, with every neighbouring row still green. Its red therefore means
/// *the witness stopped discriminating*, not that the mechanism broke.
///
/// **Promise class: durable invariant.** The subject is the relation between
/// the two coordinates per emission-owner class, asserted exhaustively over the
/// planned population rather than against a literal census.
#[test]
fn d5a_a_specialization_owned_edge_separates_root_provenance_from_its_immediate_slot() {
    with_d5a_witness_plan(|plan| {
        let units = plan.continuation_units().expect("continuation units");
        // **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2a` rider — the predeclared rows
        // are COLLECTED here and asserted below, off a value that exists only
        // because the population is non-empty.**
        //
        // ⛔ The earlier form asserted the two equalities inside this loop and
        // guarded them with a separate `assert!(predeclared > 0, …)`. That is
        // the shape `D2a` repaired: the guard only NAMES the intent, so deleting
        // it leaves the equalities compiling and passing vacuously on an empty
        // unit list. The non-zero check is now the CONSTRUCTOR of the count the
        // assertion loop ranges over — remove it and this is a compile error,
        // not a silent pass.
        let mut predeclared_rows = Vec::new();
        let mut specialization_with_a_real_difference = 0usize;
        for unit in &units {
            let inputs = unit.continuation_inputs().expect("continuation inputs");
            match unit.emission_owner() {
                ContinuationEmissionOwner::Predeclared(owner) => {
                    predeclared_rows.push((owner, inputs));
                }
                ContinuationEmissionOwner::Specialization(_) => {
                    if inputs
                        .iter()
                        .any(|input| {
                            input.availability.expect_direct_emission_slot() != input.coordinate.expect_entry_abi().1
                        })
                    {
                        specialization_with_a_real_difference += 1;
                    }
                }
                // **`D2f` — a fusion-owned continuation is not planned by this
                // witness, and this arm refuses rather than counting one.**
                //
                // The two laws this control states are derived for the two
                // owner classes it names. Neither has been derived for a fused
                // region, whose emission owner is a third thing: silently
                // dropping one would shrink the population the laws range over
                // exactly the way `closure_shaped_captures` warns about, and
                // counting one as either class would assert a law nobody has
                // established for it.
                ContinuationEmissionOwner::Fusion(fusion) => {
                    panic!(
                        "the D5a witness planned a fusion-owned continuation ({fusion:?}); this \
                         control's root-provenance and immediate-slot laws are derived for \
                         predeclared and specialization owners only, so the fusion class needs \
                         its own measured law rather than admission to one of theirs"
                    );
                }
            }
        }
        let established_predeclared = std::num::NonZeroUsize::new(predeclared_rows.len()).expect(
            "the witness must still plan at least one predeclared-owned continuation, or the \
             equality law below is asserted over an empty population",
        );
        for (owner, inputs) in predeclared_rows.iter().take(established_predeclared.get()) {
            for input in inputs {
                assert_eq!(
                    input.coordinate.expect_entry_abi().0,
                    *owner,
                    "a predeclared emitter IS its inputs' root provenance owner, so a \
                     projection naming another owner was built against a different \
                     emitter than the one that will run"
                );
                assert_eq!(
                    input.availability.expect_direct_emission_slot(),
                    input.coordinate.expect_entry_abi().1,
                    "for a predeclared emitter the root ABI position and the immediate \
                     slot index the same environment, so they must agree; this is the \
                     consistency law that lets that arm read either field"
                );
            }
        }
        assert!(
            specialization_with_a_real_difference > 0,
            "the witness must still plan at least one specialization-owned continuation whose \
             immediate slot DIFFERS from its root ABI position. Without one, reading either \
             coordinate gives the same operands and every discriminator in this group is \
             vacuous: the distinction `D5a` exists to draw would be a distinction without a \
             difference on the only fixture that measures it"
        );
    });
}

/// **`D5a` — a generated context resolves under the continuation identity that
/// owns it, and under no other.**
///
/// The ruling forbids keying the binding on the worker body origin, on ABI
/// shape, on "a context exists", or on first match. `intern_generated_contexts`
/// states that as a **key** — `(enclosing_specialization, worker_body_origin)`,
/// leading with the identity — rather than as a check. This row measures the
/// consequence: the same body origin presented under a different continuation
/// identity resolves to nothing.
///
/// ⚠ **MEASURED**: the lookup returns `Some` only for a context's own enclosing
/// specialization, over every (identity, body) pair the plan admits.
/// **CLAIMED**: two continuation identities selecting one raw worker would
/// yield two distinct contexts. **THE GAP**: this witness plans two
/// specializations over *different* worker bodies, so the second half is
/// measured on the key's discriminating power and not on a second context. The
/// reaching half is
/// `d5a_a_transplanted_generated_context_binding_refuses_at_the_retarget`, which
/// hands one identity another's context and is refused.
///
/// **`RT-CONTSRC-PRODUCER-LOCAL` `D4b` — the generated-frame consumer is
/// exercised BEHAVIOURALLY, on a fixture that compiles and runs.**
///
/// ⭐⭐ **This retires a standing evidence boundary, and the retraction is the
/// point.** `D3b`'s record carried "0 of 60 consumer observations held a
/// generated emission owner", and concluded the generated-frame route could only
/// be proved by construction. That number was **wrong**. It was measured while
/// the capture-view defect was still live, so every generated-context capture
/// was refusing before it reached this consumer — the probe recorded the
/// breakage, not the design, and the figure was then carried forward as a
/// standing fact.
///
/// Re-measured on the repaired tree, `verify_entry_frame` takes the
/// generated-frame arm **30 times** across ordinary lowering tests, including
/// `nested_post_effect_checked_recursor_reaches_success_and_retains_exact_trap_provenance`,
/// which emits and **executes** a real object.
///
/// ⛔ Incidental traffic is not a control, which is why this row exists: it arms
/// a counter over a real compile, asserts the arm was actually taken, and then
/// perturbs the identity the consumer revalidates.
///
/// ⚠ **MEASURED**: the route is taken on a compiling fixture, and displacing the
/// claimed context id reds it with the agreement refusal. **CLAIMED**: the
/// three-sided revalidation is live on the behavioural path, not only where a
/// planner control drives it. **THE GAP**: this proves the recorded id must
/// agree with what its own key resolves to; it does not re-prove that the key
/// resolved uniquely — `d3b_every_generated_frame_requirement_resolves_to_exactly_one_context`
/// owns that.
///
/// **Promise class: durable invariant.**
#[test]
fn d4b_the_generated_frame_consumer_runs_on_a_real_compile() {
    use crate::cranelift_backend::lowering::{
        d4b_generated_frame_consumptions, reset_d4b_generated_frame_consumptions,
        set_d4b_frame_mutation, D4bFrameMutation,
    };

    let compile = |mutation| {
        reset_d4b_generated_frame_consumptions();
        set_d4b_frame_mutation(mutation);
        let outcome = crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
            "d4b_generated_frame",
            false,
        )
        .map(|_| ())
        .map_err(|error| format!("{error:?}"));
        let taken = d4b_generated_frame_consumptions();
        set_d4b_frame_mutation(D4bFrameMutation::Exact);
        (outcome, taken)
    };

    let (exact, taken) = compile(D4bFrameMutation::Exact);
    exact.expect("the witness must compile with the generated-frame route exact");
    // ⛔ THE non-vacuity assertion. Without it a green compile is equally
    // consistent with the fixture never reaching this consumer at all -- which
    // is precisely the state the retracted 0/60 figure described.
    assert!(
        taken > 0,
        "the witness must actually TAKE the generated-frame arm; with zero, the row proves only          that a program which never reaches this consumer still compiles"
    );

    let (refusal, mutated_taken) = compile(D4bFrameMutation::WrongClaimedContext);
    // ⛔ **At least once, NOT the same count.** The refusal short-circuits the
    // compile, so the mutated run necessarily reaches this arm fewer times than
    // the exact one -- an equality here reds on the short-circuit rather than on
    // anything about the guard. What must hold is that the arm was reached at
    // all, which is what makes the refusal attributable to the mutation.
    assert!(
        mutated_taken > 0,
        "the mutation must have reached the generated-frame arm, or the refusal below belongs \
         to something else entirely"
    );
    let refusal = refusal.expect_err(
        "a claimed context id disagreeing with the one its own key resolves to must refuse; a          compile means the recorded identity is decorative",
    );
    assert!(
        refusal.contains("the recorded identity and the key it was resolved from disagree"),
        "the refusal must be the identity-agreement one, not a downstream failure: {refusal}"
    );
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D3b` STAGE 2 — every structural frame
/// requirement resolves to exactly one context, and zero or multiple refuses.**
///
/// ⭐⭐ **This is the obligation the Architect kept in `D3b` rather than moving
/// to `D4b`.** It is proved by direct planner controls because that is the
/// plane the property lives in: finalization is a whole-plan pass, and whether
/// it resolves *every* requirement is not something any single compiled program
/// can witness.
///
/// ⚠ **A withdrawn premise, kept visible so it is not reconstructed.** This
/// comment previously added that behavioural activation "does not exist yet",
/// citing `0` of `60` consumer observations and concluding that no compiled
/// program reaches a generated `EntryFrame` claim. **That figure was wrong** —
/// it was measured while the capture-view defect was still live, so it recorded
/// the path refusing rather than the path being absent. `D4b` re-measured it and
/// exercises the same consumer behaviourally; see
/// `d4b_the_generated_frame_consumer_runs_on_a_real_compile`. ⛔ Nothing in this
/// row ever depended on that figure, and nothing here should be restated as if
/// it did.
///
/// ⛔ **The non-vacuity counter is asserted first, and its premise is
/// arithmetic.** A zero-or-multiple perturbation over an EMPTY requirement set
/// succeeds trivially — nothing to resolve, nothing to refuse — and every row
/// below would pass while measuring an empty loop. The counter is what
/// distinguishes "the refusals fire" from "there was nothing to fire on". That
/// is true whatever the route's reachability at any consumer.
///
/// ⚠ **MEASURED**: the witness carries generated frame requirements; they
/// resolve under the real context population and refuse under both
/// perturbations; and the publication gate refuses an unfinalized claim.
/// **CLAIMED**: no half-stamped claim can reach a consumer, because the only
/// conversion that builds a view requires a finalized entry. **THE GAP**: this
/// proves the whole-plan pass resolves and refuses correctly; that a *consumer*
/// then revalidates what it holds is `D4b`'s behavioural row, not this one.
///
/// **Promise class: durable invariant.**
#[test]
fn d3b_every_generated_frame_requirement_resolves_to_exactly_one_context() {
    use crate::cranelift_backend::planning::{
        d3b_publish_without_finalization, d3b_refinalize, D3bFinalizationPerturbation,
    };
    with_d5a_witness_plan(|plan| {
        let (generated, total) = d3b_refinalize(plan, D3bFinalizationPerturbation::Exact)
            .expect("the real context population must resolve every requirement");
        assert!(
            total > 0,
            "the witness must carry availability claims at all, or every row here is vacuous"
        );
        // ⛔ THE non-vacuity assertion. See the note above.
        assert!(
            generated > 0,
            "the witness must carry at least one GENERATED frame requirement ({generated} of              {total} claims); with none, both perturbations below succeed over an empty set and              prove nothing"
        );

        let refusal = d3b_refinalize(plan, D3bFinalizationPerturbation::DropContexts)
            .expect_err("a requirement naming no interned context must refuse at finalization");
        assert!(
            format!("{refusal:?}").contains("never interned"),
            "the refusal must be the zero-resolution one: {refusal:?}"
        );

        let refusal = d3b_refinalize(plan, D3bFinalizationPerturbation::DuplicateContexts)
            .expect_err("a requirement resolving to two contexts must refuse at finalization");
        assert!(
            format!("{refusal:?}").contains("share one (enclosing specialization, worker body)"),
            "the refusal must be the multiple-resolution one, not the zero one: {refusal:?}"
        );

        // ⛔ The publication gate: a view cannot be built without a finalized
        // entry, so a draft has no path to any consumer.
        let refusal = d3b_publish_without_finalization(plan)
            .expect_err("publishing an unfinalized claim must refuse");
        assert!(
            format!("{refusal:?}").contains("no finalized availability"),
            "the refusal must be the publication gate: {refusal:?}"
        );
    });
}

/// **Promise class: durable invariant.**
#[test]
fn d5a_a_generated_context_resolves_only_under_the_identity_that_encloses_it() {
    with_d5a_witness_plan(|plan| {
        let contexts = plan.continuation_contexts().expect("contexts");
        let units = plan.continuation_units().expect("units");
        assert!(
            !contexts.is_empty(),
            "the witness must plan at least one generated context; with none, both directions \
             below hold vacuously"
        );
        for context in &contexts {
            let found = plan
                .continuation_context_for(
                    context.enclosing_specialization(),
                    context.worker_body_origin(),
                )
                .expect("the lookup answers")
                .unwrap_or_else(|| {
                    panic!(
                        "context {:?} must resolve under its own key",
                        context.id()
                    )
                });
            assert_eq!(
                found.id(),
                context.id(),
                "the lookup must return the context whose key was presented, not a plausible one"
            );
            for unit in &units {
                if unit.id() == context.enclosing_specialization() {
                    continue;
                }
                assert!(
                    plan.continuation_context_for(unit.id(), context.worker_body_origin())
                        .expect("the lookup answers")
                        .is_none(),
                    "specialization {:?} does not enclose context {:?}, so presenting that \
                     context's WORKER BODY under this identity must resolve to nothing. A `Some` \
                     here means the binding is reachable from the body origin alone, which is \
                     exactly the reconstruction the ruling forbids",
                    unit.id(),
                    context.id(),
                );
            }
        }
    });
}

/// **`D5a` checkpoint 4 step 2 — the final executable population, as a relation.**
///
/// ⭐⭐ **This is the census that decides `fn2`'s branch, and the frame is
/// explicit that no earlier prediction settles it.** Checkpoint 1 measured
/// `template_only={}` on the partial graph and the frame recorded that answer as
/// *provisional*. With the retarget and the carried-invocation binding both
/// landed, the measured answer is that the ported worker body IS superseded:
/// it keeps its descriptor and leaves the emitted-`Function` population.
///
/// ⭐ The row asserts **relations, not the literal census**, so a fixture that
/// grows a unit stays green while a rule that suppresses the wrong population
/// reds:
///
/// 1. `executable = emittable \ template_only` — the no-phantom identity;
/// 2. every superseded body still has an emittable descriptor, so "absent from
///    the executable set" never means "absent from the plan";
/// 3. at least one worker body a specialization selects is **not** superseded —
///    the mixed caller population the ruling names. "One context exists" is not
///    a global suppression predicate, and without this clause a rule that
///    suppressed every worker body the moment any context appeared would pass
///    clauses 1 and 2 unchanged.
///
/// **Promise class: durable invariant.**
#[test]
fn d5a_the_final_executable_population_is_the_emittable_set_minus_the_superseded_bodies() {
    with_d5a_witness_plan(|plan| {
        let template_only = plan
            .template_only_worker_bodies()
            .expect("the superseded set");
        let emittable = plan
            .emittable_units()
            .expect("emittable units")
            .iter()
            .map(|unit| (unit.function(), unit.body_occurrence()))
            .collect::<Vec<_>>();
        let executable = plan
            .executable_units()
            .expect("executable units")
            .iter()
            .map(|unit| (unit.function(), unit.body_occurrence()))
            .collect::<Vec<_>>();

        let expected = emittable
            .iter()
            .filter(|(_, origin)| !template_only.contains(origin))
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(
            executable, expected,
            "the executable population must be exactly the emittable population minus the \
             superseded bodies. Declaring from one set and defining from the other is how an \
             undefined phantom appears, and this equality is what forbids it"
        );

        for origin in &template_only {
            assert!(
                emittable.iter().any(|(_, unit)| unit == origin),
                "superseded body {origin:?} must keep its emittable descriptor. Losing it would \
                 make 'template-only' mean 'deleted', and the raw worker's identity and arity \
                 validation reads that descriptor"
            );
        }

        assert!(
            !template_only.is_empty(),
            "the witness must supersede at least one worker body, or this row measures a \
             population the retarget never touched and the checkpoint-4 census question is \
             unanswered"
        );
        let selected = plan
            .continuation_units()
            .expect("units")
            .iter()
            .map(|unit| unit.worker_body_origin())
            .collect::<BTreeSet<_>>();
        assert!(
            selected.iter().any(|body| !template_only.contains(body)),
            "at least one selected worker body must SURVIVE as an executable unit. A generated \
             context existing anywhere in the artifact is not a licence to suppress every raw \
             worker; a mixed caller population retains the raw `Function`, and without this \
             clause a global suppression rule would satisfy every other assertion here"
        );
    });
}

/// **`D5a` checkpoint 4 step 3 — the detached-result seat's five formerly
/// unexercised guards, each reached by a real mutation on the compiling
/// witness.**
///
/// ⭐⭐ The seat's doc comment carried an explicit *"UNEXERCISED GUARDS — do not
/// read these as tested"* block through checkpoints 1 to 3, because the only
/// fixture that reaches it refused further along and any control written then
/// would have compared a red against a red. The route is now positive — the
/// trace shows `DETACHED-SEAT edge result=... construct=... pos=1` on a compile
/// that succeeds — so every guard is reachable, and each row below moves that
/// green compile to one named refusal.
///
/// ⛔ Every mutation perturbs what the seat is **handed**; none perturbs a
/// guard. A control that edited the condition would ask whether the condition
/// agrees with itself.
///
/// **Promise class: durable invariant.** Each row names the message its own
/// guard produces, so a later refusal moving in front cannot stand in for it —
/// which is the failure this whole group exists to prevent.
#[test]
fn d5a_the_detached_result_seats_five_guards_are_each_reached_by_a_real_mutation() {
    // (label, mutation, a phrase unique to the guard it must red)
    let rows = [
        (
            "multi-member projection",
            D5aRouteMutation::DuplicateResidualEdge,
            "undischarged causal calls onto one",
        ),
        (
            "result is not a specialized constructor",
            D5aRouteMutation::CarryNonConstructorResult,
            "is not a specialized constructor",
        ),
        (
            "identity disagreement",
            D5aRouteMutation::StripLoweredConstructorIdentity,
            "is not the planner's own constructor",
        ),
        (
            "position outside the planned field run",
            D5aRouteMutation::PerturbRecursivePosition,
            "outside the planned",
        ),
        (
            "field run against the declared ordinary run",
            D5aRouteMutation::PerturbOrdinaryParameterCount,
            // ⚠ Was `"must differ by one"`. `D9b` corrected this guard's
            // relation from `args.len() == ordinary_parameters + 1` — the
            // pre-`D9` premise that the ordinary run is the nonrecursive fields
            // alone — to the planner's own
            // `nonrecursive_field_count = ordinary_parameters - captures`, so
            // the sentence it refuses with now names the prefix. The guard, the
            // mutation and the refusal are unchanged; only the wording moved.
            "must exceed that prefix by one",
        ),
    ];
    for (label, mutation, expected) in rows {
        let refusal = with_d5a_route_mutation(mutation, || {
            crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
                "d5a_detached_seat",
                false,
            )
            .map(|_| ())
            .map_err(|error| format!("{error:?}"))
            .expect_err(&format!(
                "the `{label}` guard must refuse under {mutation:?}. A COMPILE here means \
                 the guard is inert on the only route that reaches it — and since the \
                 unmutated witness compiles, that would be a silently admitted defect \
                 rather than a red-versus-red ambiguity"
            ))
        });
        assert!(
            refusal.contains(expected),
            "the `{label}` guard must refuse with its OWN message. A different refusal means \
             some earlier authority moved in front of it and this row stopped measuring the \
             guard it names: {refusal}"
        );
    }
}

/// **`D5a` checkpoint 4 step 1 — a missing generated-context binding refuses at
/// the retarget.**
///
/// The retarget is what gives a continuation specialization a callee for its
/// worker body. With the binding withheld, the specialization is left calling a
/// raw unit that checkpoint 4 step 2 removed from the executable population —
/// and the ruling is explicit that skipping `define_function` for an
/// "emittable" raw unit mints an undefined phantom. This row measures that the
/// refusal comes **before** any such phantom can be emitted.
///
/// ⭐ Note what the message rules out: *neither* an emittable raw unit *nor* a
/// generated execution context. A retarget that quietly fell back to the raw
/// unit would satisfy the first disjunct and this row would stay green — the
/// message is worded, and asserted, so that it cannot.
///
/// **Promise class: durable invariant.**
#[test]
fn d5a_a_missing_generated_context_binding_refuses_before_any_phantom_is_emitted() {
    let refusal = with_d5a_route_mutation(D5aRouteMutation::SuppressContextBinding, || {
        crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
            "d5a_missing_binding",
            false,
        )
        .map(|_| ())
        .map_err(|error| format!("{error:?}"))
        .expect_err(
            "with the context binding withheld the specialization has no callee for its \
             worker body, because that body left the executable population when the \
             retarget landed. A COMPILE here would mean the raw unit is still emitted and \
             the retarget is decorative",
        )
    });
    assert!(
        refusal.contains("has no declared callee in this function"),
        "the refusal must be the retarget's own missing-callee stop. A different refusal means \
         the compile got past the seat this row is about: {refusal}"
    );
    assert!(
        d5a_route_applications() > 0,
        "the mutation must actually have fired. A refusal reached with the perturbation never \
         applied would be measuring the unmutated route"
    );
}

/// **`D5a` checkpoint 4 step 1 — two contexts claiming one identity-and-body key
/// is a hard stop, not a first-match preference.**
///
/// ⚠ **MEASURED**: presenting the lookup a population in which one context
/// matches the key twice produces the collision refusal, and the lookup does
/// not return either candidate. **CLAIMED**: the planner never builds such a
/// population. **THE GAP**: the two are independent — `intern_generated_contexts`
/// interns on exactly this key, so the duplicate is unreachable through any
/// plan a compile accepts. The mutation therefore presents the population the
/// stop is written for, which is the only way to ask the question. ⛔ It does
/// not re-emit the stop's error; that would prove a hardcoded string
/// propagates.
///
/// **Promise class: durable invariant.**
#[test]
fn d5a_two_generated_contexts_claiming_one_key_is_a_hard_stop() {
    let refusal = with_d5a_route_mutation(D5aRouteMutation::DuplicateContextBinding, || {
        crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
            "d5a_duplicate_binding",
            false,
        )
        .map(|_| ())
        .map_err(|error| format!("{error:?}"))
        .expect_err(
            "a key that resolves twice must refuse. Picking either candidate would make \
             lowering the authority for a binding the planner owns, and 'first match' is \
             the exact selection rule the ruling forbids",
        )
    });
    assert!(
        refusal.contains("two generated contexts claim one specialization and worker body"),
        "the refusal must be the collision stop itself: {refusal}"
    );
    assert!(
        d5a_route_applications() > 0,
        "the duplicate must actually have been presented; otherwise this row refused for some \
         unrelated reason and reads as a defence"
    );
}

/// **`D5a` checkpoint 4 step 1 — a transplanted generated-context binding
/// refuses at the retarget.**
///
/// ⭐⭐ **This row exists because the transplant was measured to COMPILE.** The
/// retarget trusted its resolved context wholesale and wrote the call record's
/// `origin` from the asking unit's own `worker_body_origin`, so
/// `call_static_worker`'s `target.origin != worker.body_origin` check compared
/// that value with itself on this path. Handed one specialization's context in
/// place of another's, lowering emitted a call that type-checked — the capture
/// suffix made the operand run agree — and transferred to a function executing
/// a different body. Nothing anywhere refused.
///
/// ⚠ **MEASURED**: with the two consistency comparisons in place, presenting a
/// foreign context refuses by name before any call is emitted. **CLAIMED**: the
/// planner never produces one, because `continuation_context_for` is keyed by
/// `(enclosing, worker_body)` and is the binding's only producer — pinned
/// independently by
/// `d5a_a_generated_context_resolves_only_under_the_identity_that_encloses_it`.
/// **THE GAP**: those are separate facts. Before this row, "unreachable by
/// construction" was carrying the entire guarantee and no check could observe a
/// violation, which left the ruling's transplanted-binding stop with nothing to
/// name.
///
/// **Promise class: durable invariant.**
#[test]
fn d5a_a_transplanted_generated_context_binding_refuses_at_the_retarget() {
    let refusal = with_d5a_route_mutation(D5aRouteMutation::TransplantContextBinding, || {
        crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
            "d5a_transplanted_binding",
            false,
        )
        .map(|_| ())
        .map_err(|error| format!("{error:?}"))
        .expect_err(
            "a context whose enclosing specialization is not the unit being defined must \
             refuse. This compiled before the stop existed, so a COMPILE here is a \
             regression to a state in which one specialization's captures cross another's \
             worker execution silently",
        )
    });
    assert!(
        refusal.contains("whose enclosing specialization is"),
        "the refusal must be the transplant stop itself, naming both identities. Anything else \
         means the foreign context got past the retarget: {refusal}"
    );
    assert!(
        d5a_route_applications() > 0,
        "the transplant declines when a unit has no foreign context to be handed, so a green \
         run could mean the perturbation never fired rather than that it was refused. This \
         requires it fired"
    );
}

/// **`D5a` — the retargeted worker call is the raw operand run PLUS the
/// generated context's capture suffix, and no other call gets one.**
///
/// ⭐ The mechanism's own claim is that *"keep the raw worker's ABI unchanged"*
/// and *"carry the continuation inputs across the worker execution"* are not in
/// tension because **one is a prefix of the other**. That is a relation between
/// two operand runs, so it is measured as one: the emission log records the raw
/// run and the supplied run separately, and the suffix length is compared
/// against the planner's own capture count for the context executing that body.
/// ⛔ One total would conflate "no suffix" with "a suffix of length zero" —
/// exactly the two cases this witness contains.
///
/// The witness is a **mixed** population, which is what makes the row
/// discriminating: one worker body is retargeted through a context and gets the
/// suffix, the other is a raw executable unit and must not.
///
/// ⚠ **MEASURED**: the suffix is appended to the retargeted call and to no
/// other. **CLAIMED**: the origin guard is what confines it. **THE GAP**: the
/// guard's false branch is **not reachable on this fixture** — the capture
/// stash is per-function and set only in the specialization whose worker body
/// was retargeted, and that function makes exactly one static-worker call, to
/// that body. A mutation dropping the guard therefore never fires (measured:
/// zero applications, green compile), so it was removed rather than committed
/// as a control that cannot red. The guard stays; it is defensive against a
/// function holding two worker calls, which no fixture yet produces.
///
/// **Promise class: durable invariant.**
#[test]
fn d5a_the_retargeted_worker_call_carries_the_raw_run_plus_the_context_capture_suffix() {
    reset_d5a_marker_events();
    crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
        "d5a_capture_suffix",
        false,
    )
    .map(|_| ())
    .map_err(|error| format!("{error:?}"))
    .expect("the witness compiles");
    let events = d5a_marker_events();
    let calls = events
        .iter()
        .filter_map(|event| match event {
            D5aMarkerEvent::WorkerCallEmitted {
                body_origin,
                raw_operands,
                supplied_operands,
                route,
            } => Some((*body_origin, *raw_operands, *supplied_operands, *route)),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(
        !calls.is_empty(),
        "no static-worker call was emitted, so every claim below is vacuous: {events:?}"
    );

    with_d5a_witness_plan(|plan| {
        let contexts = plan.continuation_contexts().expect("contexts");
        let mut suffixed = 0usize;
        let mut plain = 0usize;
        // ⛔ **`RT-CONTSRC-PRODUCER-LOCAL` `D6b` — classified by the event's own
        // ROUTE, not by whether a context exists for its body.** The pre-`D6b`
        // version keyed on `contexts.find(body)`, which `D6a` made ambiguous:
        // one body origin now carries two bindings, so that lookup answers the
        // same for the induction hypothesis and for the selected recursive
        // argument while their operand runs must differ. It would demand a
        // suffix on the raw call and red on the correct program.
        //
        // ⭐ The plan is still the independent oracle for the suffix's LENGTH —
        // the context's own declared capture count — so this reads the route
        // from the emission and the arity from the planner, and neither side
        // can satisfy the row alone.
        for (body, raw, supplied, route) in &calls {
            match route {
                StaticWorkerCallRoute::GeneratedContext => {
                    let context = contexts
                        .iter()
                        .find(|context| context.worker_body_origin() == *body)
                        .unwrap_or_else(|| {
                            panic!(
                                "a call routed to a generated context must have one planned for \
                                 its body {body:?}; the route is not a free choice at the call \
                                 edge"
                            )
                        });
                    let captures = context.header().captures as usize;
                    assert_eq!(
                        supplied - raw,
                        captures,
                        "the context-routed call to body {body:?} must carry the raw run followed \
                         by EXACTLY the enclosing frame's continuation inputs — the capture run \
                         its generated context declares. A shorter suffix drops inputs; a longer \
                         one is an arity error against a frame that might be large enough to \
                         absorb it silently"
                    );
                    assert!(
                        captures > 0,
                        "a context declaring zero captures would make the prefix relation hold \
                         trivially and this row would stop discriminating"
                    );
                    suffixed += 1;
                }
                StaticWorkerCallRoute::RawWorker => {
                    assert_eq!(
                        supplied, raw,
                        "a raw-routed call to body {body:?} is the raw operand run and nothing \
                         else. ⛔ This holds even when a generated context DOES exist for that \
                         body — which is exactly the `D6a` case, where the selected recursive \
                         argument and the induction hypothesis share a body origin and only the \
                         route separates them. A suffix here would be appended to a raw frame \
                         with no capture run to hold it"
                    );
                    plain += 1;
                }
            }
        }
        assert!(
            suffixed > 0 && plain > 0,
            "the witness must emit BOTH a retargeted and an unretargeted static-worker call \
             ({suffixed} suffixed, {plain} plain). With only one kind, the row cannot tell \
             'the suffix is confined to the retargeted call' from 'a suffix is appended \
             everywhere' or from 'no suffix is ever appended'"
        );
    });
}

/// **`D5a` — the capture projection indexes the emitting environment with the
/// IMMEDIATE slot, and both of its guards are reachable.**
///
/// Three reaching mutations, each scoped to the emission-owner class whose guard
/// it is written for:
///
/// - a **predeclared** emitter's direct-emission claim is `CurrentLexical`, so
///   moving its nearest-alias index off the depth the planner walked is caught by
///   re-walking the seat. ⛔ Not by an equality against the root ABI position:
///   `D3c` measured that equality false at nonzero binder depth, so it was the
///   defect and never the guard.
/// - a **specialization** emitter's claim is an `EntryFrame` one, so an
///   out-of-range slot is refused by frame membership *before* any environment
///   is indexed.
/// - the **root-position substitution** itself, which is in range and identically
///   shaped, and is refused by that same membership check.
///
/// ⛔ The out-of-range mutation is scoped to the specialization arm on purpose.
/// Applied to a predeclared emitter the current-lexical revalidation refuses
/// first, and the row would name the membership guard while measuring the
/// revalidation one — which is what the first draft did, measured before it was
/// committed.
///
/// ⭐⭐ **The residual this row used to carry is DISCHARGED, and saying so is
/// the point.** Until the `D3b` re-cut this comment recorded a measured gap:
/// indexing with `source_abi_position` **compiled**, `Ok(())`, because both
/// numbers were in range and the operands were untyped boundary words, and the
/// sibling sentinel
/// `d5a_reading_the_root_position_as_the_immediate_slot_is_currently_undetectable`
/// asserted exactly that. The re-cut supplies the independent oracle that was
/// missing — a frame's own declared membership, which is not the same answer as
/// "where does this environment hold it" and so can disagree with it. The
/// sentinel fired on good news and was deleted; its claim is the third row here,
/// inverted into a refusal.
///
/// ⚠ **MEASURED**: all three guards red under their own mutations, each with the
/// perturbation confirmed applied. **CLAIMED**: lowering resolves each consumer's
/// own claim against the environment that consumer actually holds. **THE GAP**:
/// this says nothing about whether the planner ASSIGNED the right index — it
/// re-runs the planner's own walk, so a defect there would be reproduced rather
/// than caught. `D2b`'s discriminator and `D3a`'s validator own that half.
///
/// **Promise class: durable invariant.**
#[test]
fn d5a_the_capture_projection_reads_the_immediate_slot_and_bounds_it() {
    let rows = [
        // ⭐ The predeclared emitter's claim is CURRENT-LEXICAL after the re-cut,
        // so moving its index off the walked binder depth is caught by the
        // planner's own re-walk of the seat -- not by the retired equality
        // against a root ABI position, which `D3c` measured as the defect rather
        // than the guard.
        (
            "the predeclared emitter's current-lexical revalidation",
            D5aRouteMutation::PerturbPredeclaredImmediateSlot,
            "does not hold that coordinate at",
        ),
        // ⭐ The specialization emitter's claim is an ENTRY-FRAME one, so an
        // out-of-range slot is caught by frame membership before any environment
        // is indexed. That is a strengthening: the old row reached the bounds
        // test, which fires only after a wrong-but-in-range slot has already been
        // read.
        (
            "the specialization emitter's entry-frame slot agreement",
            D5aRouteMutation::PerturbImmediateSlotOutOfRange,
            "the two disagree",
        ),
        // ⭐⭐ **The retired sentinel, folded in exactly where it said to fold
        // it.** `d5a_reading_the_root_position_as_the_immediate_slot_is_currently_undetectable`
        // asserted this substitution COMPILES, and named its retiring event as
        // "any mechanism that makes a swapped read detectable at this seat". The
        // `D3b` re-cut is that mechanism, so the sentinel goes red on good news
        // and its claim survives here inverted -- as a refusal rather than a
        // documented gap.
        //
        // ⛔ Same guard as the row above, different STIMULUS: that one moves the
        // slot out of range, this one substitutes the root ABI position, which is
        // in range and identically shaped. `D3c` measured that exact substitution
        // selecting a different operand with nothing to notice.
        (
            "the root-position substitution D3c measured as silent",
            D5aRouteMutation::ReadRootPositionAsImmediateSlot,
            "the two disagree",
        ),
    ];
    for (label, mutation, expected) in rows {
        let refusal = with_d5a_route_mutation(mutation, || {
            let refusal =
                crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
                    "d5a_capture_projection",
                    false,
                )
                .map(|_| ())
                .map_err(|error| format!("{error:?}"))
                .expect_err(&format!(
                    "{label} must refuse under {mutation:?}; a compile means the guard is \
                     inert on the route that reaches it"
                ));
            assert!(
                d5a_route_applications() > 0,
                "{label}: the mutation is scoped to one emission-owner class and declines \
                 for the other, so a refusal reached without it firing would be measuring \
                 the unmutated route"
            );
            refusal
        });
        assert!(
            refusal.contains(expected),
            "{label} must refuse with its OWN message, or this row is measuring the other \
             guard: {refusal}"
        );
    }
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D3b` — a coordinate the emission seat does not
/// hold is refused, whatever its domain.**
///
/// ⭐ **This row's authority MOVED under the re-cut; it was not lost, and it is
/// now stronger.** Its `D1` sentinel asserted the seam refused *every*
/// producer-local coordinate. `D3b` first replaced that with a CROSSED-PAIR law:
/// a producer-local coordinate carrying an entry-ABI availability was refused
/// because the two halves named different coordinate spaces. The re-cut retires
/// that law too — availability is no longer keyed to a root domain, so there is
/// no crossed pair left to detect, and a row still asserting one would be
/// asserting about a distinction the representation no longer draws.
///
/// ⛔ What the perturbation now meets is a **stronger** refusal, and the
/// difference matters: the old one refused a TYPE-LEVEL mismatch between two
/// tags, which a later extension that widened either domain would have silently
/// dissolved. The new one refuses because the injected coordinate **is genuinely
/// not in the environment standing at this seat** — a fact about the program,
/// checked by walking it, which no widening of a tag can make true.
///
/// ⚠ **MEASURED**: the object emission refuses for absence from the seat
/// environment, with the perturbation confirmed fired. **CLAIMED**: a coordinate
/// the seat does not hold cannot be indexed, regardless of which domain names it.
/// **THE GAP**: this says nothing about which index a coordinate the seat DOES
/// hold resolves to; `d3b_the_consumer_refuses_an_index_the_emission_seat_does_not_hold`
/// owns that half.
///
/// **Promise class: durable invariant.** A value absent from the emission seat's
/// environment must never resolve to a position in it, under any later extension
/// of either coordinate domain.
#[test]
fn contsrc_the_emission_resolver_refuses_a_producer_local_coordinate() {
    let refusal = with_d5a_route_mutation(
        D5aRouteMutation::PresentProducerLocalCoordinate,
        || {
            let refusal =
                crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
                    "contsrc_producer_local",
                    false,
                )
                .map(|_| ())
                .map_err(|error| format!("{error:?}"))
                .expect_err(
                    "the emission resolver must refuse a coordinate the seat environment does \
                     not hold; a compile means the seat revalidation is inert on the route \
                     that reaches it",
                );
            assert!(
                d5a_route_applications() > 0,
                "the perturbation must have fired, or this row measured the unmutated route"
            );
            refusal
        },
    );
    assert!(
        refusal.contains("not present in the lexical environment"),
        "the refusal must be the seat-absence one, not an incidental failure downstream: \
         {refusal}"
    );
}

/// **`D5a` checkpoint 4 step 1 — the carried invocation's retained source
/// coordinates are the key, and losing the binding fails closed.**
///
/// The binding must be resolved from the invocation's causal identity and
/// retained source coordinates — never from the body origin, the callee's ABI
/// shape, the existence of a context, or a first match. Perturbing the
/// coordinate the invocation presents is therefore the direct question: does
/// the retarget actually depend on it?
///
/// ⭐⭐ **This row forced a production repair.** The fail-closed test — *a body
/// with a generated context may not be called raw* — used to guard only the
/// **missing-coordinates** arm. Coordinates that were present but resolved to
/// nothing fell straight through to the raw target. On this witness that still
/// refused, but **only incidentally**, because the superseded body has no
/// `Function` left to call; had it remained executable the retarget would have
/// been dropped in silence. The guard now belongs to the *outcome* rather than
/// to one of the two routes into it, and the refusal names the coordinates that
/// were presented.
///
/// **Promise class: durable invariant.**
#[test]
fn d5a_perturbing_the_retained_source_coordinates_fails_closed_rather_than_calling_raw() {
    let refusal = with_d5a_route_mutation(D5aRouteMutation::PerturbCarriedInvocationCoordinates, || {
        crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
            "d5a_carried_coordinates",
            false,
        )
        .map(|_| ())
        .map_err(|error| format!("{error:?}"))
        .expect_err(
            "a coordinate the planner never issued must resolve no context, and a body that \
             HAS a context may not then be called raw. A compile here means the retarget \
             does not depend on the coordinates it claims to be keyed by",
        )
    });
    assert!(
        refusal.contains("resolved no generated execution context, and that body has one"),
        "the refusal must be the fail-closed stop, not an incidental failure further along — \
         which is exactly what this route produced before the guard covered both arms: {refusal}"
    );
    assert!(
        d5a_route_applications() > 0,
        "the coordinate perturbation must have fired"
    );
}

/// **`D5a` checkpoint 1 — a superseded worker body keeps its raw descriptor
/// authority.**
///
/// "Unchanged ordinary `fn2` ABI" means the raw worker's **descriptor and
/// source binding** survive so a generated context can validate and lower the
/// same body. It does **not** mean the body still receives a `Function` — the
/// measured census puts it in the template-only set. Those two facts are only
/// separable now that the retarget has landed: on a program with no retarget
/// the emittable and executable populations are identical and reading either
/// gives the same templates.
///
/// The mutation builds the raw template population from the **executable** set
/// instead of the **emittable** one — the "template-only means deleted"
/// reading. The static-worker constructor, which validates against that
/// descriptor and holds no `FuncRef` at all, then has nothing to validate
/// against.
///
/// **Promise class: durable invariant.**
#[test]
fn d5a_a_superseded_worker_body_keeps_its_raw_descriptor_authority() {
    let refusal = with_d5a_route_mutation(D5aRouteMutation::DropSupersededWorkerTemplates, || {
        crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
            "d5a_raw_descriptor",
            false,
        )
        .map(|_| ())
        .map_err(|error| format!("{error:?}"))
        .expect_err(
            "dropping the superseded body's descriptor must refuse. A compile would mean \
             nothing consumes the raw contract any more, and the checkpoint-1 separation \
             between descriptor authority and executable membership would be decorative",
        )
    });
    assert!(
        refusal.contains("no raw worker template for body origin"),
        "the refusal must come from the constructor that reads the raw descriptor: {refusal}"
    );
    assert!(
        d5a_route_applications() > 0,
        "the template population must actually have been narrowed"
    );
}

/// **`RT-CONTINUATION-EDGE-DISPOSITION` `D2` — the COMPOSITE early close, and
/// candidate totality is the refusal it reaches.**
///
/// **Re-identified from `D5a` checkpoint 2, because the mutation stopped being
/// a single-authority one.** It was written when the wrapper owned one ledger,
/// and it moved that ledger's window back to the first definition pass. The
/// wrapper now owns the candidate ledger as a sibling on the same lifetime, so
/// the same mutation moves **both** closeouts — and `D2` closes candidates
/// first, so candidate **totality** is the refusal it now reaches.
///
/// That ordering is not a preemption to undo: the claim law's `D2` domain does
/// not exist until totality has closed, because deriving the
/// `DirectCall ∪ ComposedCall` subset first lets an unsettled candidate simply
/// disappear from it.
///
/// The mutation still moves only the **window**, not any equality, so the
/// refusal remains attributable to the lifetime and to nothing else — which is
/// what "the defect was the lifetime, not the equality" meant and still means.
///
/// The row also measures that the domain really is generalized: the witness
/// plans a causal call whose emission owner is a `Specialization`, so a
/// closeout taken before that owner's `Function` exists is taken over a
/// strictly smaller population than the plan issued.
///
/// **`D5a`'s original single-authority promise is not lost with this
/// re-identification.** It lives in
/// `ced_d2_an_unclaimed_planned_token_is_missing_from_the_exact_equality_in_isolation`,
/// which reaches the discharge equality directly from a supplied complete
/// call-obligation domain.
///
/// **Promise class: durable invariant.**
#[test]
fn ced_d2_the_composite_early_close_reaches_candidate_totality_on_the_generalized_owner_domain() {
    with_d5a_witness_plan(|plan| {
        let specialization_owned = plan
            .continuation_calls()
            .expect("continuation calls")
            .iter()
            .filter(|call| {
                matches!(
                    call.emission_owner(),
                    ContinuationEmissionOwner::Specialization(_)
                )
            })
            .count();
        assert!(
            specialization_owned > 0,
            "the witness must plan at least one causal call emitted from a generated context. \
             With none, the ledger's domain is the pre-`D5a` predeclared one and closing it \
             early would lose nothing — the row would be green because there was nothing to \
             lose, not because the lifetime is right"
        );
    });
    let refusal = with_d5a_route_mutation(D5aRouteMutation::CloseLedgerAfterTheFirstPass, || {
        crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
            "d5a_ledger_lifetime",
            false,
        )
        .map(|_| ())
        .map_err(|error| format!("{error:?}"))
        .expect_err(
            "closing the ledger before any generated `Function` exists must refuse: the \
             specialization-owned token is planned and cannot yet have been claimed. A \
             compile here would mean the equality is satisfied by a population smaller than \
             the plan issued, which is the per-pass partial the ruling forbids",
        )
    });
    assert!(
        d5a_route_applications() > 0,
        "the early close must actually have happened"
    );
    // **RE-POINTED BY `D2`, and the reason is that this mutation no longer
    // moves one authority.** It moved the CLAIM ledger's window when it was
    // written; the wrapper now owns two sibling ledgers, so the same mutation
    // moves BOTH closeouts early. Candidate totality is the first production
    // refusal under it, and that is correct rather than a preemption to undo:
    // the claim law's `D2` domain does not exist until totality has closed,
    // because deriving the subset first lets an unsettled candidate simply
    // disappear from it.
    //
    // ⇒ This row is now the `D2` **composite-close** control, and it is named
    // for the authority it actually reaches. `D5a`'s original single-authority
    // promise — that the specialization-owned token is missing from the exact
    // claim/discharge equality — is NOT abandoned; it is owed a separate
    // isolated ledger-unit control whose candidate domain is already complete.
    // Rewriting only this string, keeping the old name and prose, would have
    // retired that oracle while looking like a fix.
    assert!(
        refusal.contains("reached the artifact closeout without a disposition"),
        "under the composite early close, candidate TOTALITY is the first production refusal: \
         the mutation closes both sibling ledgers before the generated pass, so candidates are \
         genuinely unsettled. Anything else -- in particular `the continuation candidate ledger \
         went missing` -- would mean the early close SUCCEEDED and this row is measuring the \
         aftermath instead of the closeout: {refusal}"
    );
}

/// **`D5a` — a specialization is interned BEFORE the fixed point descends into
/// its worker body.**
///
/// The descent that discovers nested producers is queued only when
/// `intern_specialization` reports the key newly inserted, and it carries the
/// id that interning just assigned as the enclosing emission owner. That
/// ordering is what makes the fixed point terminate on a recursive body: a
/// descent that rediscovers its own key finds it interned and adds no work.
///
/// ⭐ The consequence is checkable on the finished plan, over the planner's own
/// dense identities: a specialization produced *by* a descent names an
/// enclosing specialization that was interned **strictly earlier**. Interning
/// after the descent, or queuing work for a key not yet interned, cannot
/// produce that ordering.
///
/// ⚠ **MEASURED**: every specialization-owned unit names an enclosing id that
/// exists and is strictly smaller than its own, and the interned population is
/// exactly the unit population. **CLAIMED**: the fixed point terminates on a
/// recursive body. **THE GAP**: those are different statements. The termination
/// half rests on the `inserted` guard at the descent, and the honest reaching
/// mutation for it — queue the descent unconditionally — **does not terminate**,
/// so it cannot be committed as a control. It is not written rather than
/// written in a form that would hang CI.
///
/// **Promise class: durable invariant.**
#[test]
fn d5a_a_specialization_is_interned_before_the_descent_that_produced_it() {
    with_d5a_witness_plan(|plan| {
        let units = plan.continuation_units().expect("continuation units");
        let ids = units.iter().map(|unit| unit.id()).collect::<BTreeSet<_>>();
        assert_eq!(
            ids.len(),
            units.len(),
            "the interned population must be in bijection with the unit population; a repeated \
             dense id would mean two keys collapsed onto one specialization"
        );
        let mut descended = 0usize;
        for unit in &units {
            if let ContinuationEmissionOwner::Specialization(enclosing) = unit.emission_owner() {
                assert!(
                    ids.contains(&enclosing),
                    "specialization {:?} names enclosing {enclosing:?}, which is not an interned \
                     unit. A descent carrying an id that does not exist means work was queued \
                     before its key was interned",
                    unit.id()
                );
                assert!(
                    enclosing < unit.id(),
                    "specialization {:?} was produced by a descent from {enclosing:?}, so the \
                     enclosing key must have been interned STRICTLY EARLIER and hold the smaller \
                     dense id. Interning after the descent cannot produce this ordering — and \
                     nor can it terminate on a recursive body, because the key its own descent \
                     rediscovers would not yet be present",
                    unit.id()
                );
                descended += 1;
            }
        }
        assert!(
            descended > 0,
            "the witness must contain at least one specialization the fixed point reached by \
             DESCENDING into a worker body. Without one, the ordering above holds vacuously and \
             the recursive case is unmeasured"
        );
    });
}



// ══════════════════════════════════════════════════════════════════════════
//  `RT-DECL-CLOSURE-PORT` `D6a` UPSTREAM — the route is a predecessor-edge fact
// ══════════════════════════════════════════════════════════════════════════
//
// ⭐⭐ THE ONE FACT EVERY ROW BELOW RESTS ON, AND IT WAS MEASURED, NOT ASSUMED.
//
// The governed witness has **two** carried computational-match consumers, and
// they sit at the **same** `StaticOriginId(10)` under the **same** checked
// frame `7`. They receive **opposite** routes, from **different** producers:
//
// | seat | `incoming` | `frame_field` | supplied by | outcome |
// |---|---|---|---|---|
// | `Composed` | `CheckedSelectedRecursor` | `DirectScrutinee` | PRODUCER 2, the exact claimed `CSId(0)` call result | enters the elimination, emits the fallback |
// | `SourceMachine` | `DirectScrutinee` | `CheckedSelectedRecursor` | PRODUCER 1, the exact selecting recursor layer | re-entry at the same origin, takes the termination backedge |
//
// ⛔ **That table is the whole checkpoint.** The origin does not determine the
// route, the checked frame does not determine the route, and an
// occurrence-global projection would mark **both** rows checked — including the
// one whose predecessor is an ordinary direct descent. It is also why every row
// below keys on the **seat**, never on the origin alone: two records share that
// origin and an assertion naming only `StaticOriginId(10)` cannot say which
// edge it is about.
//
// ⚠ **A DISCREPANCY WITH THE FRAME, REPORTED RATHER THAN PAPERED OVER.** The
// frame's first upstream discriminator reads *"the exact claimed `CSId(1)` call
// result reaches origin 10 as `CheckedSelectedRecursor`"*. **It is `CSId(0)`.**
// `CSId(1)` is raised too, and correctly — but strictly *after* the fallback is
// emitted, inside the return case's own body, so its result reaches no carried
// consumer on this witness. The rows below assert what the artifact does. The
// mechanism the discriminator was reaching for — an exactly claimed and emitted
// continuation call result reaching the consumer as checked — holds exactly as
// ruled; only the identity named in the bullet differs.

/// One `D6a` observation of the governed witness: the route trace, the trap
/// provenance, the raw-worker census, and whether the mutation fired.
struct D6aObservation {
    route: Vec<D6aRouteEvent>,
    provenance: Vec<Px8trTrapProvenanceEvent>,
    static_worker_calls: usize,
    applications: usize,
    emitted: bool,
}

impl D6aObservation {
    /// The consumer record for one seat. ⛔ Keyed on the seat, never the
    /// origin — both seats share `StaticOriginId(10)`.
    fn consumer(&self, seat: D6aConsumerSeat) -> (SourceComputationalAnswerRoute, SourceComputationalAnswerRoute, SourceComputationalAnswerRoute) {
        self.route
            .iter()
            .find_map(|event| match event {
                D6aRouteEvent::ConsumerRoute {
                    seat: recorded,
                    incoming,
                    frame_field,
                    joined,
                    ..
                } if *recorded == seat => Some((*incoming, *frame_field, *joined)),
                _ => None,
            })
            .unwrap_or_else(|| panic!("the witness must reach the {seat:?} carried consumer"))
    }

    fn raised_targets(&self) -> Vec<ContinuationSpecializationId> {
        self.route
            .iter()
            .filter_map(|event| match event {
                D6aRouteEvent::CallResultRaised { target } => Some(*target),
                _ => None,
            })
            .collect()
    }

    fn fallbacks(&self) -> usize {
        self.route
            .iter()
            .filter(|event| matches!(event, D6aRouteEvent::CarriedFallbackEmitted { .. }))
            .count()
    }

    fn defaults(&self) -> Vec<SourceComputationalAnswerRoute> {
        self.route
            .iter()
            .filter_map(|event| match event {
                D6aRouteEvent::CarriedDefaultSealed { route, .. } => Some(*route),
                _ => None,
            })
            .collect()
    }

    fn eliminations_entered(&self) -> Vec<SourceComputationalAnswerRoute> {
        self.route
            .iter()
            .filter_map(|event| match event {
                D6aRouteEvent::CarriedEliminationEntered { route, .. } => Some(*route),
                _ => None,
            })
            .collect()
    }

    fn header_controls(&self) -> Vec<(CarriedComputationalLoopEdge, i64, i64)> {
        self.provenance
            .iter()
            .filter_map(|event| match event {
                Px8trTrapProvenanceEvent::CarriedLoopHeaderEdgeEmitted {
                    edge,
                    authored_control_word,
                    emitted_control_word,
                    ..
                } => Some((*edge, *authored_control_word, *emitted_control_word)),
                _ => None,
            })
            .collect()
    }

    /// Whether the artifact sealed the **exact planned** checked-`ITree`
    /// default into a generated unit's `TrapWord`, with the planner-issued
    /// identity intact. ⛔ Not "did it trap" — the fixture plans a second,
    /// unrelated default and a check that could not tell them apart would name
    /// nothing.
    fn sealed_the_exact_checked_itree_default(&self) -> bool {
        let expected = RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "PX8-TR checked ITree recursor default".to_string(),
        };
        self.provenance.iter().any(|event| matches!(
            event,
            Px8trTrapProvenanceEvent::PlannedTrapEmitted {
                trap,
                seat: PlannedTrapSeat::UnitTrapWord,
                planned_identity,
                emitted_word,
            } if trap == &expected && planned_identity == emitted_word && *planned_identity > 0
        ))
    }
}

fn observe_d6a(name: &str, mutation: D6aRouteMutation) -> D6aObservation {
    with_d6a_route_mutation(mutation, || {
        reset_d5a_marker_events();
        let outcome =
            crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
                name, false,
            );
        let (provenance, emitted) = match &outcome {
            Ok(object) => (object.provenance.clone(), true),
            Err(_) => (Vec::new(), false),
        };
        D6aObservation {
            route: d6a_route_trace(),
            provenance,
            static_worker_calls: d5a_marker_events()
                .iter()
                .filter(|event| matches!(event, D5aMarkerEvent::WorkerCallEmitted { .. }))
                .count(),
            applications: d6a_route_applications(),
            emitted,
        }
    })
}

/// **`D6a` upstream 1/8 — the exact claimed call result reaches the carried
/// consumer as checked, and that is what emits the fallback.**
///
/// ⚠ MEASURED / CLAIMED / THE GAP.
/// **MEASURED:** producer 2 raises an exactly claimed and emitted
/// continuation-specialization call result; the `Composed` consumer at origin
/// 10 receives it as `CheckedSelectedRecursor` while its **own** frame field
/// says `DirectScrutinee`; the join keeps checked; the elimination is entered
/// with checked; the fallback is emitted; nothing takes a closed default.
/// **CLAIMED:** the call-result producer, alone, carries this witness's
/// emission.
/// **THE GAP:** this row is compile-time. That the emitted artifact then *runs*
/// through the return case is the linked exit-0 half, asserted in
/// `object_linker_packaging`'s
/// `nested_post_effect_checked_recursor_reaches_success_and_retains_exact_trap_provenance`.
/// ⛔ Neither substitutes for the other, and this row does not claim the
/// runtime half.
///
/// **Promise class: durable invariant** — it asserts a relation between the two
/// producers and the consumer, over the planner's own identities. Adding a
/// specialization, renaming a constructor, or renumbering the plan keeps it
/// green; erasing the route on a forward turns it red, which is the defect it
/// exists for.
#[test]
fn d6a_the_exact_claimed_call_result_reaches_the_carried_consumer_as_checked() {
    let run = observe_d6a("d6a_exact_call_result", D6aRouteMutation::Exact);
    assert!(run.emitted, "the governed witness must still emit");
    assert_eq!(
        run.applications, 0,
        "the exact run must perturb nothing"
    );

    let (incoming, frame_field, joined) = run.consumer(D6aConsumerSeat::Composed);
    assert_eq!(
        incoming,
        SourceComputationalAnswerRoute::CheckedSelectedRecursor,
        "the exact claimed call result must arrive at the composed consumer already checked"
    );
    // ⭐ The load-bearing half. If the frame's own field were also checked here
    // this row could not tell the call-result producer apart from the
    // recursor-layer one, and would pass under the `ae45e804` defect.
    assert_eq!(
        frame_field,
        SourceComputationalAnswerRoute::DirectScrutinee,
        "the composed consumer's own field must be DIRECT, or this row cannot attribute the \
         route to the call-result producer and would stay green while the frame field \
         overwrote the incoming one"
    );
    assert_eq!(
        joined,
        SourceComputationalAnswerRoute::CheckedSelectedRecursor,
        "the join must keep the predecessor's checked route"
    );

    assert!(
        !run.raised_targets().is_empty(),
        "producer 2 must have fired at least once"
    );
    assert_eq!(
        run.eliminations_entered(),
        vec![SourceComputationalAnswerRoute::CheckedSelectedRecursor],
        "exactly one carried elimination is entered on this witness, and it is entered checked"
    );
    assert_eq!(run.fallbacks(), 1, "the checked-answer fallback is emitted once");
    assert_eq!(
        run.defaults(),
        vec![SourceComputationalAnswerRoute::CheckedSelectedRecursor],
        "the fail-closed successor is emitted beside the checked successor"
    );
    assert!(
        run.sealed_the_exact_checked_itree_default(),
        "the two-parameter header must retain the exact fail-closed default beside the checked CFG successor"
    );
}

/// **`D6a` upstream 2/8 — dropping ONLY the call-result route emits Direct at
/// the existing two-parameter header.**
///
/// The mutation is surgical: producer 1 is untouched and still supplies
/// `CheckedSelectedRecursor` at its own seat. Only the exactly claimed call
/// result comes back `DirectScrutinee`; the composed consumer joins
/// direct-with-direct and the initial edge emits explicit Direct control. Both
/// CFG successors remain present, including the exact planned fail-closed
/// default. Runtime selection is pinned by the D1 full-program child controls.
///
/// **Promise class: durable invariant.**
#[test]
fn d6a_dropping_only_the_call_result_route_emits_direct_header_control() {
    let run = observe_d6a("d6a_drop_call_result", D6aRouteMutation::DropCallResultRoute);
    assert!(
        run.applications > 0,
        "the drop must actually have been applied, or this row records a route it never reached"
    );
    assert!(
        run.emitted,
        "dropping the route must not break emission — the defect it reproduces is a SILENT \
         one, and a refusal here would mean this row measures something else"
    );

    let (incoming, _, joined) = run.consumer(D6aConsumerSeat::Composed);
    assert_eq!(incoming, SourceComputationalAnswerRoute::DirectScrutinee);
    assert_eq!(joined, SourceComputationalAnswerRoute::DirectScrutinee);

    // ⭐ Producer 1 is demonstrably still alive, which is what makes this a
    // control over ONE producer rather than over the transport as a whole.
    let (_, recursor_field, _) = run.consumer(D6aConsumerSeat::SourceMachine);
    assert_eq!(
        recursor_field,
        SourceComputationalAnswerRoute::CheckedSelectedRecursor,
        "the recursor-layer producer must be untouched by this mutation"
    );

    assert_eq!(
        run.fallbacks(),
        1,
        "the shared two-successor CFG is emitted independently of runtime control"
    );
    assert_eq!(
        run.defaults(),
        vec![SourceComputationalAnswerRoute::DirectScrutinee]
    );
    assert!(run.header_controls().iter().any(|(edge, authored, emitted)| {
        *edge == CarriedComputationalLoopEdge::Initial && *authored == 0 && *emitted == 0
    }));
    assert!(
        run.sealed_the_exact_checked_itree_default(),
        "the exact fail-closed successor must remain present beside the checked successor"
    );
}

/// **`D6a` upstream 3/8 — the SAME checked frame, with an ordinary direct
/// predecessor, stays `DirectScrutinee`.**
///
/// ⭐⭐ This is the control the frame names as *"the control that would have
/// caught the occurrence-global projection"*, and the witness supplies it
/// without any fixture work: **both** consumers sit at `StaticOriginId(10)`
/// under checked frame `7`, and their incoming routes **differ**.
///
/// ⛔ So `checked_frame_id.is_some()`, the match origin, the frame's presence,
/// and the existence of a continuation unit are each individually consistent
/// with **both** rows — every one of them would mark the direct predecessor
/// checked. Only the predecessor edge separates them.
///
/// **Promise class: durable invariant.** It asserts that two edges at one
/// origin disagree; any future mechanism that genuinely made them agree would
/// be a semantic change this row should stop.
#[test]
fn d6a_the_same_checked_frame_with_a_direct_predecessor_stays_direct() {
    let run = observe_d6a("d6a_same_frame_direct", D6aRouteMutation::Exact);
    let (composed_incoming, ..) = run.consumer(D6aConsumerSeat::Composed);
    let (machine_incoming, ..) = run.consumer(D6aConsumerSeat::SourceMachine);

    assert_eq!(
        composed_incoming,
        SourceComputationalAnswerRoute::CheckedSelectedRecursor
    );
    assert_eq!(
        machine_incoming,
        SourceComputationalAnswerRoute::DirectScrutinee,
        "an ordinary direct predecessor under the same checked frame must stay direct"
    );
    assert_ne!(
        composed_incoming, machine_incoming,
        "if these ever agree the witness has stopped discriminating, and every \
         occurrence-global projection the checkpoint forbids would pass"
    );

    // Both really are the same origin and the same checked frame — otherwise
    // "the SAME checked frame" is not what is being measured.
    let origins: BTreeSet<_> = run
        .route
        .iter()
        .filter_map(|event| match event {
            D6aRouteEvent::ConsumerRoute { static_origin, .. } => Some(*static_origin),
            _ => None,
        })
        .collect();
    assert_eq!(
        origins.len(),
        1,
        "the two consumers must share one origin, or this row is comparing different \
         occurrences and proves nothing about projection"
    );
    let checked_frames: BTreeSet<_> = run
        .route
        .iter()
        .filter_map(|event| match event {
            D6aRouteEvent::RecursorLayerSupplied {
                checked_frame_id, ..
            } => Some(*checked_frame_id),
            _ => None,
        })
        .collect();
    assert_eq!(
        checked_frames.len(),
        1,
        "one checked frame governs both consumers"
    );
    assert!(checked_frames.iter().all(Option::is_some));
}

/// **`D6a` upstream 4/8 — a raw or static-worker call cannot mint the checked
/// route.**
///
/// The witness emits **both** kinds of call. If a raw or static-worker result
/// could raise the route, the raise count would exceed the count of exactly
/// claimed continuation calls the planner issued.
///
/// ⚠ MEASURED / CLAIMED / THE GAP.
/// **MEASURED:** the artifact emits static-worker calls, and the raised
/// targets are exactly the planner's continuation-call targets — no more.
/// **CLAIMED:** no call outside the claimed continuation path mints the checked
/// route.
/// **THE GAP:** this is a census over the raises this fixture reaches, not a
/// proof about unreachable code. The closure argument is structural and lives
/// in the type: `RoutedAnswer::checked` is private to this module and the only
/// value that can produce one on a call result is built after the owner/affine
/// claim inside `claim_and_call_resolved_continuation` — a raw or worker call
/// returns a `RoutedAnswer::direct` whose route a caller can only *raise* by
/// joining, never by asserting. ⛔ That argument is a compile-time property of
/// item visibility, and it is stated here rather than tested, because a test
/// over source text is not a test of behaviour.
///
/// **Promise class: durable invariant.**
#[test]
fn d6a_a_raw_or_static_worker_call_cannot_mint_the_checked_route() {
    let run = observe_d6a("d6a_raw_cannot_mint", D6aRouteMutation::Exact);
    assert!(
        run.static_worker_calls > 0,
        "the witness must actually emit worker calls, or this row rules out a call class the \
         fixture never makes and is vacuous"
    );

    let planned: BTreeSet<_> = with_d5a_witness_plan(|plan| {
        plan.continuation_calls()
            .expect("continuation calls")
            .iter()
            .map(|call| call.target())
            .collect()
    });
    let raised: BTreeSet<_> = run.raised_targets().into_iter().collect();
    assert!(
        raised.is_subset(&planned),
        "every raise must name a planned continuation-call target; a raise outside that set \
         is a call class minting the route. raised={raised:?} planned={planned:?}"
    );
    assert_eq!(
        run.raised_targets().len(),
        raised.len(),
        "a target raised twice would mean one claimed identity produced two checked results"
    );
}

/// **`D6a` upstream 5/8 — the recursor-layer producer stays green
/// INDEPENDENTLY of the call-result producer.**
///
/// Dropping producer 1 alone flips only its own seat to `DirectScrutinee`. The
/// composed consumer is untouched, the fallback is still emitted, and the
/// artifact still builds.
///
/// ⭐ Read with row 2, this is what "two lawful producers" means operationally:
/// each mutation moves exactly one seat, and neither seat's answer is derivable
/// from the other's. ⛔ A single mutation disabling both could not tell
/// *independent* from *jointly dead*.
///
/// ⚠ And it records an asymmetry honestly: on **this** witness the emission is
/// owned by producer 2, so dropping producer 1 is invisible at the emission.
/// That is a fact about the fixture, not a claim that producer 1 is inert —
/// its own seat demonstrably changes answer.
///
/// **Promise class: durable invariant.**
#[test]
fn d6a_the_recursor_layer_producer_stays_green_independently() {
    let run = observe_d6a("d6a_drop_recursor", D6aRouteMutation::DropRecursorLayerRoute);
    assert!(
        run.applications > 0,
        "the recursor-layer drop must actually have fired"
    );
    assert!(run.emitted, "the artifact must still build");

    let (_, machine_field, machine_joined) = run.consumer(D6aConsumerSeat::SourceMachine);
    assert_eq!(
        machine_field,
        SourceComputationalAnswerRoute::DirectScrutinee,
        "producer 1's own seat must show the dropped answer, or the mutation did not reach it"
    );
    assert_eq!(machine_joined, SourceComputationalAnswerRoute::DirectScrutinee);

    let (composed_incoming, _, composed_joined) = run.consumer(D6aConsumerSeat::Composed);
    assert_eq!(
        composed_incoming,
        SourceComputationalAnswerRoute::CheckedSelectedRecursor,
        "producer 2 must be untouched by a producer-1 mutation"
    );
    assert_eq!(
        composed_joined,
        SourceComputationalAnswerRoute::CheckedSelectedRecursor
    );
    assert_eq!(
        run.fallbacks(),
        1,
        "the emission on this witness is producer 2's, so dropping producer 1 must not move it"
    );
    assert!(run.sealed_the_exact_checked_itree_default());
    assert!(run.header_controls().iter().any(|(edge, authored, emitted)| {
        *edge == CarriedComputationalLoopEdge::ActiveSelfResumption
            && *authored == 0
            && *emitted == 0
    }));
}

/// **`D6a` upstream 6/8 — mixed-route predecessors at one origin are preserved
/// as separate arms, and nothing collapses them.**
///
/// The frame requires that if composition would merge `DirectScrutinee` and
/// `CheckedSelectedRecursor` before the consumer, the arms are **preserved as
/// distinct predecessors** or the join **hard-stops** — and forbids collapsing
/// to either scalar or adding a runtime discriminator.
///
/// ⭐ The witness *is* the mixed-route fixture: two predecessor edges, one
/// origin, opposite incoming routes. They are preserved as separate arms
/// structurally — the checked composed edge opens the elimination, and the
/// direct source-machine edge is a re-entry at the same origin that takes the
/// existing termination backedge rather than opening a second one.
///
/// ⛔ **What this row forbids, concretely:** a single carried elimination
/// entered on a *collapsed* scalar. Exactly one elimination is entered and it
/// carries `CheckedSelectedRecursor`; the direct arm never enters one. If the
/// two routes were ever merged into one scalar before the consumer, either two
/// eliminations would be entered on one route or the single entry would carry
/// the wrong one.
///
/// **Promise class: durable invariant** — stated as a relation between the
/// consumer records and the entries, so a legitimate change in how many
/// predecessors the witness has keeps it green as long as no route is lost.
#[test]
fn d6a_mixed_route_predecessors_at_one_origin_stay_separate() {
    let run = observe_d6a("d6a_mixed_route", D6aRouteMutation::Exact);

    let incoming: Vec<_> = run
        .route
        .iter()
        .filter_map(|event| match event {
            D6aRouteEvent::ConsumerRoute { incoming, .. } => Some(*incoming),
            _ => None,
        })
        .collect();
    assert!(
        incoming.len() >= 2,
        "the witness must present at least two predecessor edges, or there is no merge to \
         preserve and this row is vacuous"
    );
    assert!(
        incoming
            .iter()
            .any(|route| *route == SourceComputationalAnswerRoute::CheckedSelectedRecursor)
            && incoming
                .iter()
                .any(|route| *route == SourceComputationalAnswerRoute::DirectScrutinee),
        "the edges must genuinely be MIXED: {incoming:?}"
    );

    // ⛔ No collapse: one elimination, entered on the checked route, and the
    // direct arm opened none of its own.
    assert_eq!(
        run.eliminations_entered(),
        vec![SourceComputationalAnswerRoute::CheckedSelectedRecursor],
        "a collapse would show up here — either as a second entry, or as a single entry \
         carrying the scalar the merge chose"
    );
    assert_eq!(run.fallbacks(), 1);
    assert_eq!(
        run.defaults(),
        vec![SourceComputationalAnswerRoute::CheckedSelectedRecursor]
    );
}

/// **`D6a` upstream 7/8 — planned, claimed and emitted call identity agree on
/// the EXACT identity, not merely on counts.**
///
/// Producer 2 records `identity.target()` — read back out of the opaque
/// `ContinuationCallIdentity` it consumed, **after** the owner/affine claim
/// succeeded and **after** the emitted callee was checked against that same
/// identity. So a raise is simultaneously evidence of claim and of emission.
///
/// ⭐ The assertion is **set equality over the planner's identities**, and the
/// reason the frame insists on it is visible in the numbers: the witness plans
/// two calls and raises two results, so a count-only check reads `2 == 2` and
/// would survive a lowering that raised one identity twice while never
/// reaching the other.
///
/// **Promise class: durable invariant** — the relation is closed over whatever
/// the plan issues, so a witness that grows a third specialization keeps it
/// green without edit.
#[test]
fn d6a_planned_claimed_and_emitted_identity_agree_exactly() {
    let run = observe_d6a("d6a_identity_agreement", D6aRouteMutation::Exact);
    let planned: BTreeSet<_> = with_d5a_witness_plan(|plan| {
        plan.continuation_calls()
            .expect("continuation calls")
            .iter()
            .map(|call| call.target())
            .collect()
    });
    let raised_list = run.raised_targets();
    let raised: BTreeSet<_> = raised_list.iter().copied().collect();

    assert!(
        planned.len() >= 2,
        "with fewer than two planned identities a set equality and a count check are the same \
         assertion, and this row would not be measuring what it claims"
    );
    assert_eq!(
        raised, planned,
        "the exactly claimed and emitted call results must be precisely the planned targets"
    );
    assert_eq!(
        raised_list.len(),
        raised.len(),
        "each planned identity is claimed and emitted exactly once"
    );
}

/// **`D6a` upstream 8/8 — the join is load-bearing: the frame's own field must
/// not overwrite an incoming checked route.**
///
/// ⭐⭐ This reproduces the exact defect measured at `ae45e804`, which is the
/// reason the recut exists. The mutation changes **one thing** — the consumer
/// assigns the frame's own recursor-layer field instead of joining it with the
/// predecessor's route — and the consequence is total and silent: the artifact
/// still builds, still links, and quietly takes the closed default.
///
/// ⛔ It is the silence that makes this row necessary. Nothing about the
/// compile distinguishes the defect from the repair; only the emitted route
/// and the planted trap identity do.
///
/// **Promise class: durable invariant** — a regression control over a defect
/// that has actually occurred.
#[test]
fn d6a_the_frame_field_must_not_overwrite_an_incoming_checked_route() {
    let run = observe_d6a(
        "d6a_overwrite_join",
        D6aRouteMutation::OverwriteIncomingWithFrameField,
    );
    assert!(run.applications > 0, "the overwrite must actually have fired");
    assert!(
        run.emitted,
        "the ae45e804 defect is SILENT — it compiles. A refusal here would mean this row is \
         measuring a different failure"
    );

    let (incoming, frame_field, joined) = run.consumer(D6aConsumerSeat::Composed);
    assert_eq!(
        incoming,
        SourceComputationalAnswerRoute::CheckedSelectedRecursor,
        "the predecessor still supplies the checked route — the mutation is in the JOIN"
    );
    assert_eq!(frame_field, SourceComputationalAnswerRoute::DirectScrutinee);
    assert_eq!(
        joined,
        SourceComputationalAnswerRoute::DirectScrutinee,
        "the overwrite must be what erases the route"
    );
    assert_eq!(
        run.fallbacks(),
        1,
        "the shared two-successor CFG remains emitted under a Direct control"
    );
    assert!(run.header_controls().iter().any(|(edge, authored, emitted)| {
        *edge == CarriedComputationalLoopEdge::Initial && *authored == 0 && *emitted == 0
    }));
    assert!(
        run.sealed_the_exact_checked_itree_default(),
        "the erased route must retain the exact planned fail-closed successor"
    );
}

/// **`RT-DECL-CLOSURE-PORT` `D7` — erasing any axis of the seat key, or
/// collapsing every seat onto one contract, rejects before the artifact is
/// finished.**
///
/// ⭐ **The mutation is applied in the POPULATION BUILDER only, never in the
/// recomputation the ledger close performs.** That asymmetry is what gives the
/// control its force: the planner's own rebuild-equality validation mutates on
/// both sides and cannot see any of these, so a green row here has to come from
/// the independent side actually running.
///
/// ⛔ The fixture is the governed nested bracket rather than the process pair,
/// and the reason is a near miss worth recording: the process pair's only
/// effect has ONE argument seat, so collapsing every argument ordinal to `0`
/// changed nothing and `EraseOrdinal` passed while measuring an identity. This
/// fixture's `BufferFreeze` and `FsReadAt` seats carry four and five arguments,
/// so the ordinal axis has something to erase.
///
/// MEASURED: the unmutated fixture compiles; each of the four erasures refuses
/// it; it compiles again once the mutation clears.
///
/// CLAIMED: operation, ordinal and need are load-bearing at a gate rather than
/// recorded and unread.
///
/// THE GAP: this measures the four erasures written here. It is not a proof
/// that no other perturbation of a seat record is admitted.
#[test]
fn erasing_a_seat_key_axis_or_collapsing_the_contract_rejects() {
    use crate::cranelift_backend::planning::{
        governed_nested_resource_bracket, set_effect_seat_plan_mutation, EffectSeatPlanMutation,
    };
    let expr = governed_nested_resource_bracket(3);
    set_effect_seat_plan_mutation(EffectSeatPlanMutation::Exact);
    recursive_port_process_compiles(&expr)
        .expect("the unmutated bracket compiles, so the rows below are not vacuous");
    for mutation in [
        EffectSeatPlanMutation::EraseOperation,
        EffectSeatPlanMutation::EraseOrdinal,
        EffectSeatPlanMutation::EraseNeed,
        EffectSeatPlanMutation::CollapseContract,
    ] {
        set_effect_seat_plan_mutation(mutation);
        let refusal = recursive_port_process_compiles(&expr);
        set_effect_seat_plan_mutation(EffectSeatPlanMutation::Exact);
        let error = match refusal {
            Ok(()) => panic!("{mutation:?} left the seat authority satisfied"),
            Err(error) => error.to_string(),
        };
        // ⛔ The discriminating half. Refusing is not enough: the ruling is that
        // a seat that cannot be satisfied is refused AS THAT SEAT, never handed
        // to the generic specialized-only failure. So the refusal must name a
        // seat, and must not be the generic surface's.
        assert!(
            error.contains("seat"),
            "{mutation:?} was refused without naming a seat: {error}"
        );
        assert!(
            !error.contains("is a specialized-only surface"),
            "{mutation:?} fell through to the generic specialized-only refusal: {error}"
        );
    }
    // ⛔ Restored, and re-measured rather than assumed: a mutation left set
    // would make every later test in this binary run against a mutated plan.
    set_effect_seat_plan_mutation(EffectSeatPlanMutation::Exact);
    recursive_port_process_compiles(&expr)
        .expect("the bracket compiles again once the mutation clears");
}

// ---------------------------------------------------------------------------
// `RT-CONTSRC-PRODUCER-LOCAL` `D4a` — the lowerable shifted producer-local
// population, and the control that measures which operand the emitting
// environment actually holds at the nearest-alias index.
// ---------------------------------------------------------------------------

fn d4a_unit() -> RuntimeExpr {
    RuntimeExpr::Construct {
        constructor: "ctor:prelude::Unit::MkUnit".to_string(),
        args: Vec::new(),
    }
}

fn d4a_trap(message: &str) -> RuntimeTrap {
    RuntimeTrap {
        code: RuntimeTrapCode::PatternMatchFailure,
        message: message.to_string(),
    }
}

/// One admitted host effect. ⭐ `ConsoleWrite` and not `ConsoleRead`: the lane,
/// not the shape, is what made the `D2b` fixture unlowerable, and
/// `CRANELIFT_HOST_EFFECT_CONSUMERS_V1` is a compile-time constant. ⛔ Not
/// `ConsoleIsTerminal` either — it is in that set and still plans no seat,
/// because it returns before seat synthesis.
///
/// The payload is the only difference between the two occurrences, and it
/// exists so a reader can tell them apart in source; nothing asserts on it.
fn d4a_console_write(payload: &[u8]) -> RuntimeExpr {
    RuntimeExpr::Effect {
        family: "Console".to_string(),
        operation: ken_host::HostOpV1::ConsoleWrite,
        capability: None,
        args: vec![
            RuntimeExpr::Construct {
                constructor: "ctor:prelude::Stream::Stdout".to_string(),
                args: Vec::new(),
            },
            RuntimeExpr::Value(crate::RuntimeValue::Bytes(payload.to_vec())),
        ],
    }
}

/// The consumer: a computational match whose scrutinee is the producer
/// construct, carrying a closure at the recursive position.
fn d4a_parameter_match(case_body: RuntimeExpr) -> RuntimeExpr {
    let worker = RuntimeExpr::LexicalClosure {
        captures: Vec::new(),
        params: vec!["worker".to_string()],
        body: Box::new(RuntimeExpr::Construct {
            constructor: "ctor:fixture::Contspec::Leaf".to_string(),
            args: Vec::new(),
        }),
    };
    RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: "ctor:fixture::Contspec::Node".to_string(),
            args: vec![worker],
        }),
        cases: vec![
            crate::RuntimeComputationalMatchCase {
                constructor: "ctor:fixture::Contspec::Leaf".to_string(),
                argument_binders: 0,
                recursive_positions: Vec::new(),
                body: d4a_unit(),
            },
            crate::RuntimeComputationalMatchCase {
                constructor: "ctor:fixture::Contspec::Node".to_string(),
                argument_binders: 1,
                recursive_positions: vec![0],
                body: case_body,
            },
        ],
        default: d4a_trap("d4a persistent continuation result"),
    }
}

/// **The `D4a` population fixture — and it supplies ONLY the population.**
///
/// ⛔ **It is not evidence of its own coordinate selection.** It is built to
/// exhibit a shifted producer-local emission that reaches lowering, and being
/// observed to exhibit one proves nothing about the derivation. The
/// discrimination is carried by the creation-seat attribution and the two
/// mutations in [`d4a_the_nearest_alias_slot_holds_the_operand_built_for_that_binding`].
///
/// **The shape, and why each piece is there:**
///
/// - the outer `Let` binds an admitted host-effect result, so the value is a
///   producer-local with a locator whose `environment_index` is `0`;
/// - the enclosing `Match` case pushes **one intervening binder** before the
///   emission seat, so the value has moved by the time it is emitted — this is
///   the shift, and it is the whole reason the fixture exists;
/// - the `Match` scrutinee's constructor argument is a **second** host effect
///   of the **same operation**, so the binder at the locator index is a decoy
///   with the same carrier, the same phase and the same lowering shape. ⭐ Only
///   its SSA word differs, which is what forces the oracle to be the SSA word
///   rather than any incidental discriminator.
///
/// ⛔ `contsrc_d2_both_binding_kinds_fixture` is NOT modified. This is additive;
/// that fixture and its `D2b` discriminator stand exactly as they were.
fn d4a_shifted_lowerable_fixture() -> RuntimeExpr {
    RuntimeExpr::Let {
        value: Box::new(d4a_console_write(b"nearest-alias")),
        body: Box::new(RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::Contspec::Node".to_string(),
                args: vec![d4a_console_write(b"decoy")],
            }),
            cases: vec![RuntimeMatchCase {
                constructor: "ctor:fixture::Contspec::Node".to_string(),
                binders: 1,
                body: d4a_parameter_match(RuntimeExpr::Var(3)),
            }],
            default: d4a_trap("d4a shifted lowerable"),
        }),
    }
}

/// Compile the fixture through the production planner and lowering path under
/// one slot selection, and return the shifted observation plus the operand
/// lowering recorded at the binder-creation seat for that binding.
fn d4a_observe(
    selection: crate::cranelift_backend::lowering::D4aSlotSelection,
) -> (crate::cranelift_backend::lowering::D4aSeamObservation, String) {
    use crate::cranelift_backend::lowering::{
        d4a_set_slot_selection, d4a_take_created, d4a_take_seam, D4aSlotSelection,
    };
    use crate::cranelift_backend::lowering::d4a_set_armed;
    let _ = d4a_take_seam();
    let _ = d4a_take_created();
    d4a_set_armed(true);
    d4a_set_slot_selection(selection);
    let expr = d4a_shifted_lowerable_fixture();
    // ⛔ The exact production entry the other controls in this file use. The
    // emission still refuses the producer-local coordinate downstream of the
    // observatory, so this is expected to be an error; `D4a` measures the
    // operands, it does not consume them.
    let _ = recursive_port_process_compiles(&expr);
    d4a_set_armed(false);
    d4a_set_slot_selection(D4aSlotSelection::Exact);
    let seam = d4a_take_seam();
    let created = d4a_take_created();

    // The shifted row is selected by the property that defines it, never by
    // ordinal: a fixture whose rows reorder must not silently measure a
    // different input.
    let shifted = seam
        .iter()
        .filter(|observation| observation.nearest_alias_index != observation.locator_index)
        .cloned()
        .collect::<Vec<_>>();
    let [shifted] = shifted.as_slice() else {
        panic!(
            "expected exactly one shifted producer-local input reaching lowering, got \
             {shifted:?} out of {seam:?}"
        );
    };
    let built = created
        .iter()
        .filter(|(origin, _)| *origin == shifted.binding_origin)
        .map(|(_, operand)| operand.clone())
        .collect::<Vec<_>>();
    let [built] = built.as_slice() else {
        panic!(
            "the binder-creation seat must record exactly one operand for binding origin \
             {:?}, got {built:?}",
            shifted.binding_origin
        );
    };
    (shifted.clone(), built.clone())
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D4a` — the emitting environment holds, at the
/// nearest-alias index, the operand lowering built for that exact binding.**
///
/// This is the property `D4a` exists to supply `D3b`, and the one the previous
/// round could not reach: at `52422da5` the single reaching emission had
/// `nearest_alias_index == locator.environment_index == 0`, which makes a real
/// nearest-alias selection and a locator pass-through observationally identical.
///
/// MEASURED: compiling `d4a_shifted_lowerable_fixture` through the production
/// planner and lowering path yields exactly one producer-local continuation
/// input whose `nearest_alias_index` differs from its locator's
/// `environment_index`; the operand the emitting context's environment holds at
/// that nearest-alias index is **the same Cranelift SSA value** lowering recorded
/// at the binder-creation seat for that binding's own occurrence; and the
/// operand at the locator index is a **different** SSA value of the same
/// carrier, phase and lowering shape.
///
/// CLAIMED: a consumer indexing this environment with `nearest_alias_index`
/// obtains the producer-local value, and one indexing it with the locator's
/// introduction index does not.
///
/// THE GAP: **no consumer indexes it yet.** The emission seam still refuses
/// every producer-local coordinate; this measures the operands such a consumer
/// would read, which is why the Architect's gate separates the two — the `D4a`
/// mutation proves the instrument, and `D3b`'s own mutation must prove the
/// consumer against this same fixture.
///
/// ⛔ The oracle is independent of the planner: the creation half is keyed by
/// the lowering's own occurrence id at the seat where it constructs the binder,
/// with no environment index in play. There is no planner re-walk, no index
/// arithmetic, and no fixture-authored expected index anywhere in this row.
///
/// **Promise class: durable invariant.**
#[test]
fn d4a_the_nearest_alias_slot_holds_the_operand_built_for_that_binding() {
    use crate::cranelift_backend::lowering::D4aSlotSelection;

    let (exact, built) = d4a_observe(D4aSlotSelection::Exact);

    // The population, stated as a precondition rather than as a result: the
    // fixture was built for this, so it is not evidence — it is what makes
    // everything below discriminating instead of vacuous.
    assert_ne!(
        exact.nearest_alias_index, exact.locator_index,
        "the fixture must reach a genuinely shifted emission; equal indices make the wrong \
         answer indistinguishable from the right one, which is the defect this checkpoint \
         exists to close"
    );

    // ⭐ THE PROPERTY. Attribution, not agreement: the operand at the nearest-alias
    // slot is the one lowering built for this binding's own occurrence.
    assert_eq!(
        exact.nearest_alias_operand, built,
        "the emitting environment does not hold, at the nearest-alias index, the operand lowering \
         built for binding origin {:?}",
        exact.binding_origin
    );

    // ⛔ THE VACUITY KILL. Had the derivation handed the locator's introduction
    // index through, it would have named a position holding a DIFFERENT value —
    // so this asserts the wrong answer is wrong, not merely that two numbers
    // are unequal.
    assert_ne!(
        exact.locator_operand, built,
        "the locator index holds the same operand as the nearest-alias index, so nothing here \
         distinguishes a real nearest-alias selection from passing the introduction index through"
    );

    // The decoy is same-shaped on every incidental axis, so the row above
    // cannot have been carried by a representation mismatch.
    let shape = |operand: &str| {
        operand
            .split_once('(')
            .map(|(head, _)| head.to_string())
            .unwrap_or_else(|| operand.to_string())
    };
    assert_eq!(
        shape(&exact.nearest_alias_operand),
        shape(&exact.locator_operand),
        "the decoy must match the nearest-alias operand's carrier, phase and lowering shape, or an \
         incidental refusal could carry this test instead of the index"
    );

    // `D4a` MUTATION 1 — consume the locator's introduction index.
    let (mutated, mutated_built) = d4a_observe(D4aSlotSelection::UseLocatorIndex);
    assert_eq!(
        mutated_built, built,
        "the mutation must perturb only which slot is read; the binder-creation seat is not on \
         its path and must record the same operand"
    );
    assert_ne!(
        mutated.nearest_alias_operand, mutated_built,
        "reading the locator index still produced the operand built for this binding, so the \
         instrument cannot tell the two slots apart and proves nothing about the index"
    );

    // `D4a` MUTATION 2 — exchange the two slots. Distinct from mutation 1: both
    // indices stay lawful and in bounds, so it survives a repair that merely
    // bounds-checks.
    let (swapped, swapped_built) = d4a_observe(D4aSlotSelection::SwapSlots);
    assert_eq!(swapped_built, built, "the creation seat is off the swap's path");
    assert_ne!(
        swapped.nearest_alias_operand, swapped_built,
        "swapping the slots left the nearest-alias position holding this binding's operand, so the \
         pairing is not what the oracle reads"
    );
    assert_eq!(
        swapped.locator_operand, built,
        "the swap must move this binding's operand to the locator position; if it did not, the \
         two reads are not the pair the exact row asserted about"
    );
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D3b` — the CONSUMER refuses a wrong index at
/// the actual consumption boundary.**
///
/// ⭐ **This is the half `D4a` could not prove, and the distinction is the
/// Architect's gate `evt_65xkzqppdqdaj`.** `D4a` proved the *instrument*: that
/// the nearest-alias slot and the locator slot of `d4a_shifted_lowerable_fixture`
/// hold different Cranelift SSA operands. That says nothing about whether
/// production notices when it reads the wrong one — and it could not, because
/// until `D3b` production refused every producer-local coordinate before
/// reaching an index at all. This row is the consumer's proof.
///
/// **Why a check is needed at all, rather than trusting the projection.** Every
/// incidental discriminator a consumer could otherwise rely on is EQUAL across
/// the positions of one seat environment: `D4a` measured both of this fixture's
/// inputs carrying `ValueWord` / `OwnedByFrame` / `ActivationFrame` and the same
/// referent affinity, and both operands lowering to a `HostResult` with the same
/// constructor pair. A consumer indexing with the wrong number would therefore
/// obtain a well-formed operand of exactly the right contract and emit a call
/// carrying the wrong value, silently.
///
/// MEASURED: with the consumer unmutated, this fixture's emission passes the
/// seat-consistency check and lowering proceeds past the emission seam. Under
/// each of the two committed consumer mutations it is refused *at that check*,
/// and the perturbation is confirmed to have fired.
///
/// CLAIMED: the seam consumes the projection's nearest-alias index and no other
/// number.
///
/// THE GAP: the check re-runs the planner's own walk, so it proves the consumer
/// indexes with the number the planner assigned — **not** that the assignment is
/// right. `D2b`'s discriminator and `D3a`'s validator own that half.
///
/// **Promise class: durable invariant.**
#[test]
fn d3b_the_consumer_refuses_an_index_the_emission_seat_does_not_hold() {
    use crate::cranelift_backend::lowering::{
        d3b_consumer_applications, set_d3b_consumer_mutation, D3bConsumerMutation,
    };

    let compile = |mutation| {
        set_d3b_consumer_mutation(mutation);
        let outcome = recursive_port_process_compiles(&d4a_shifted_lowerable_fixture());
        let applications = d3b_consumer_applications();
        set_d3b_consumer_mutation(D3bConsumerMutation::Exact);
        (format!("{outcome:?}"), applications)
    };

    // The positive control. ⛔ It deliberately does NOT assert a successful
    // compile: this fixture stops later, at a unit-body environment boundary
    // that is not this seam's and that `D3b` did not touch. What it asserts is
    // the discriminating fact — that the failure is not *this* check.
    let (exact, exact_applications) = compile(D3bConsumerMutation::Exact);
    assert_eq!(
        exact_applications, 0,
        "the unmutated route must not record a perturbation"
    );
    // ⛔ The marker is the re-cut guard's OWN sentence, not the retired
    // "emission-seat consistency check" phrase. That phrase named the equality
    // against a root ABI position, which `D3c` measured false and the re-cut
    // deleted; left here it would have matched nothing, so the positive control
    // would have passed vacuously and every row below with it.
    assert!(
        !exact.contains("does not hold that coordinate at"),
        "the unmutated consumer must pass the seat revalidation, or every mutation row below \
         is measuring a failure that was already there: {exact}"
    );

    for mutation in [
        D3bConsumerMutation::ConsumeLocatorIndex,
        D3bConsumerMutation::ShiftProducerLocalSlot,
    ] {
        let (refusal, applications) = compile(mutation);
        assert!(
            applications > 0,
            "{mutation:?} never fired, so this row measured the unmutated route"
        );
        assert!(
            refusal.contains("does not hold that coordinate at"),
            "{mutation:?} must be refused for naming an index the seat does not hold this \
             coordinate at, which is the exact proposition -- not by an incidental failure \
             downstream: {refusal}"
        );
    }
}

// ---------------------------------------------------------------------------
// `RT-CONTSRC-PRODUCER-LOCAL` `D3c` — does an entry-ABI value's ROOT ABI
// position remain its IMMEDIATE position at a predeclared emission seat that
// sits under an intervening binder?
// ---------------------------------------------------------------------------

/// Compile the bracket population through the production entry under one
/// position selection, and return the entry-ABI rows of the seats that satisfy
/// the Architect's conditions 1 and 2 together.
///
/// ⛔ **No fixture is authored here.** `governed_nested_resource_bracket` is the
/// existing production planner population that five landed controls already
/// compile; `D3c` measures it rather than building a shape to exhibit an answer.
/// The compile is expected to end in an error — this population reaches the
/// unit-body environment boundary recorded at `D3b` — and the observation is
/// taken at the emission seat, upstream of it.
fn d3c_observe(
    selection: crate::cranelift_backend::lowering::D3cPositionSelection,
) -> Vec<crate::cranelift_backend::lowering::D3cSeatObservation> {
    use crate::cranelift_backend::lowering::{
        d3c_set_armed, d3c_set_position_selection, d3c_take_seat, D3cPositionSelection,
    };
    let _ = d3c_take_seat();
    d3c_set_armed(true);
    d3c_set_position_selection(selection);
    // The population that reaches a seat under an intervening binder.
    let _ = recursive_port_process_compiles(
        &crate::cranelift_backend::planning::governed_nested_resource_bracket(3),
    );
    // ⭐ And the `D5a` witness, which reaches predeclared emission seats at
    // **zero** binder depth and compiles GREEN. It supplies the agreement half:
    // without it the divergence measured in the other population would be
    // equally consistent with an oracle that never lines up.
    let _ = crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
        "d3c_zero_depth_agreement",
        false,
    );
    d3c_set_armed(false);
    d3c_set_position_selection(D3cPositionSelection::MeasuredImmediate);

    d3c_take_seat()
}

/// The seats satisfying the Architect's conditions 1 and 2 **together**: a
/// predeclared emission holding both root domains in one required vector, whose
/// environment is longer than its entry ABI run because a binder intervened.
///
/// ⛔ Selected by those conditions, never by ordinal: a population whose seats
/// reorder must not silently measure a different emission.
fn d3c_shifted(
    seats: &[crate::cranelift_backend::lowering::D3cSeatObservation],
) -> Vec<crate::cranelift_backend::lowering::D3cSeatObservation> {
    seats
        .iter()
        .filter(|seat| {
            seat.entry_abi_inputs > 0
                && seat.producer_local_inputs > 0
                && seat.emission_environment.len() > seat.abi_operands
        })
        .cloned()
        .collect()
}

/// The descriptor's shape, with the SSA word dropped — `specialized-scalar` out
/// of `specialized-scalar(v15)`. Used only to prove a difference is **not** a
/// shape difference.
fn d3c_shape(operand: &str) -> &str {
    match operand.find('(') {
        Some(open) => &operand[..open],
        None => operand,
    }
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D3c` — an entry-ABI value's root ABI position
/// is NOT its immediate position at a predeclared emission seat under an
/// intervening binder.**
///
/// This is the Architect's bounded measurement (`evt_56jh63qntwtfe`, Steward
/// recut `evt_7he9qv8wbv1yq`), and **it authorizes no production edit.**
///
/// **The sentence that stood here is stale and its staleness mattered.** It
/// read: *"the projection's `RootIsImmediate` arm still copies
/// `source_abi_position` into `immediate_slot`, and the emission seam still
/// reads `producer_env` there."* That was true when this control was written
/// and is now false. `RootIsImmediate` is **retired** on both parent and
/// candidate; production resolves the claim through
/// `resolve_direct_emission_claim` on `ContinuationEnvironmentClaim::
/// CurrentLexical`, whose `nearest_alias_index` indexes the emitting
/// environment.
///
/// **Be exact about what retired**, because an earlier version of this
/// paragraph was not. `source_abi_position` did NOT retire: it is the root
/// component of `ContinuationSourceCoordinate::EntryAbi`, and production
/// consumes it there outside any `cfg(test)` gate. What retired is the
/// SUBSTITUTION of that root position for an index into the emitter's
/// environment, and that substitution now exists only as this observatory's
/// `cfg(test)` mutation. So there is **no live production consequence** behind
/// this measurement, and no production repair or residual node is authorized by
/// it.
///
/// MEASURED: compiling the existing `governed_nested_resource_bracket(3)`
/// population through the production planner and lowering path reaches a
/// predeclared emission seat whose required vector holds **both** an
/// `EntryAbi`-root input and a `ProducerLocal`-root input, with an emission
/// environment longer than the entry ABI operand run. At that seat, the operand
/// production's own entry walk recorded for ABI position `p` is **not** the
/// operand the emission environment holds at index `p`; the entry operand is
/// present in that environment exactly once, at a **different** index; and the
/// operand sitting at `p` is in bounds and of the identical lowering shape.
///
/// CLAIMED: the `RootIsImmediate` copy **was** unsound at nonzero lexical
/// depth — stated of the retired shape, which is what this measurement is
/// about. A predeclared emitter reading `producer_env[source_abi_position]`
/// obtains a well-formed operand of exactly the right contract that is **a
/// different value** — the silent-wrong-value class this node exists to
/// prevent, and the class no bounds check or contract check can see.
///
/// THE GAP: this measures **one** population at **one** depth. It does not
/// establish how the corrected representation should be spelled, and it does not
/// measure whether any *currently accepted* program reaches this seat — the
/// population it was found in already fails downstream for an unrelated reason.
/// ⛔ Per the checkpoint, the repair is structural and is not attempted here.
///
/// Promise class: **durable invariant** — it asserts a relation between two
/// independently derived answers to "which value is this", and no literal index
/// or count is pinned. If a later checkpoint corrects the representation so the
/// two agree, this control is the thing that must be re-cut deliberately, and
/// its failure would be the correction announcing itself.
///
/// **That re-cut has happened once, and not in the direction the paragraph
/// above anticipated.** `RT-SRCBODY-BIND-ORDER` `D1` did not make the two
/// answers agree; it made them differ by a KNOWN permutation, converting a
/// source body's ABI parameter run into the de Bruijn order `lower_expr`
/// resolves against. That reddened the zero-depth agreement half, which had
/// asserted positional identity between the two indexings.
///
/// The half is now asserted at the **exact derived position**, computed from
/// the seat's own descriptor facts. An intermediate cut used
/// membership-exactly-once and was **rejected as insufficient**: every
/// permutation of a run of unique operands satisfies membership, so it cannot
/// distinguish the intended conversion from arbitrary misalignment. See the
/// comment at the assertion for the mapping and why it is derived rather than
/// searched.
///
/// **A correction to a claim made in this doc by `RT-SRCBODY-BIND-ORDER`.**
/// It said `D1` "widened the defect this control measures", on the premise that
/// the `RootIsImmediate` copy was still live and had been accidentally correct
/// at zero binder depth. That premise is false — see the retirement noted at
/// the top — so there is no widened production defect and nothing to discharge.
/// `D1` changed what this **observatory** observes, and only that.
#[test]
fn d3c_an_entry_abi_root_position_is_not_the_immediate_position_under_a_binder() {
    use crate::cranelift_backend::lowering::D3cPositionSelection;

    let observed = d3c_observe(D3cPositionSelection::MeasuredImmediate);
    let measured = d3c_shifted(&observed);
    let [seat] = measured.as_slice() else {
        panic!(
            "expected exactly one entry-ABI input at a predeclared seat holding both root \
             domains under an intervening binder, got {measured:#?}"
        );
    };

    // ⭐ Positive control on the ORACLE, and it leads deliberately: an entry walk
    // that recorded nothing would make every comparison below -- the agreement
    // half included -- pass or fail for a reason that has nothing to do with a
    // moved position. ⛔ It is asserted over EVERY observed row, not just the
    // shifted one, so a starved oracle is attributed here rather than surfacing
    // as a confusing failure of whichever check happens to run first.
    for seat in &observed {
        assert_ne!(
            seat.entry_operand, "none",
            "the entry ABI walk recorded no operand at position {}, so this row measures a \
             missing oracle rather than a moved position: {seat:#?}",
            seat.source_abi_position
        );
    }

    // ⭐⭐ **The discriminating control, and the reason this measurement is about
    // the BINDER rather than about a misaligned oracle.**
    //
    // At the seats of this same population where NO binder intervenes -- the
    // emission environment is exactly the entry ABI run -- the two answers
    // agree, position for position. So the entry oracle is not offset in
    // general; it is correct wherever the projection's assumption holds, and
    // divergent exactly where a binder has been pushed. ⛔ Without this row the
    // measurement above is equally consistent with an oracle that never lines
    // up, which would establish nothing.
    let flush = observed
        .iter()
        .filter(|seat| seat.emission_environment.len() == seat.abi_operands)
        .collect::<Vec<_>>();
    assert!(
        !flush.is_empty(),
        "no zero-depth seat was observed, so the agreement half of this measurement is vacuous \
         and the divergence below cannot be attributed to the binder: {observed:#?}"
    );
    for seat in &flush {
        // `RT-SRCBODY-BIND-ORDER` `D3c` re-cut, Architect-directed.
        //
        // This half previously asserted POSITIONAL identity --
        // `emission_environment[source_abi_position] == entry_operand` -- and
        // that is the premise `D1` retires: the emission environment is indexed
        // by de Bruijn position and the entry ABI run by descriptor position,
        // and for a source body the two are reverses of each other. The old
        // equality was true only while nothing converted between them.
        //
        // An intermediate cut asserted MEMBERSHIP-once instead, and that is
        // insufficient: every permutation of a run of unique operands satisfies
        // it, so it cannot tell the intended conversion from arbitrary
        // misalignment. What replaces both is the EXACT derived position.
        //
        // The mapping is computed from the seat's own descriptor facts -- the
        // slot kind at the ABI position and the length of the `Parameter` run --
        // never by searching the environment for the operand. A search would
        // make the instrument agree with whatever production did, which is the
        // one thing an oracle must not do.
        //
        //   Parameter at ABI position p, run length P  ->  P - 1 - p
        //   Capture   at ABI position p                ->  p
        //
        // Captures keep descriptor order and sit strictly after the reversed
        // parameter prefix, so a capture's ABI position and its semantic
        // position coincide.
        let parameter_run = seat.source_parameter_run;
        let position = seat.source_abi_position as usize;
        let derived = match seat.source_slot_kind {
            Some(AbiSlotKind::Parameter) => {
                assert!(
                    position < parameter_run,
                    "the descriptor calls ABI position {position} a Parameter but its Parameter \
                     run is only {parameter_run} long, so the two recorded facts disagree and no \
                     mapping can be derived: {seat:#?}"
                );
                parameter_run - 1 - position
            }
            Some(AbiSlotKind::Capture) => position,
            other => panic!(
                "ABI position {position} has descriptor kind {other:?}, which is neither of the \
                 two kinds the entry run is built from; the observatory is reading a slot run it \
                 does not understand: {seat:#?}"
            ),
        };
        assert_eq!(
            seat.emission_environment.get(derived),
            Some(&seat.entry_operand),
            "at zero binder depth the entry ABI operand at position {position} must sit at the \
             semantic position its descriptor derives ({derived}); if it does not, the oracle is \
             misaligned and the shifted row below proves nothing: {seat:#?}"
        );
    }

    // The flip must not be a BOUNDS mismatch. The root position is a lawful
    // index into this environment -- and that is precisely what made the
    // RETIRED root-as-immediate shape silently wrong rather than loudly wrong:
    // indexing here with a root ABI position returned a value without error,
    // just not the right one. Current production reads neither position at this
    // seam; it resolves `CurrentLexical`'s `nearest_alias_index`. So the
    // in-bounds read below is the `D3c` mutation reconstructing that retired
    // substitution, not a description of what production does today.
    let at_root = seat
        .emission_environment
        .get(seat.source_abi_position as usize)
        .unwrap_or_else(|| {
            panic!(
                "the root ABI position {} is outside the emission environment, so the difference \
                 below would be a bounds failure rather than an operand-identity one: {seat:#?}",
                seat.source_abi_position
            )
        });

    // The measurement. Two independently derived answers to "which value sits
    // at ABI position p", and they disagree.
    assert_ne!(
        at_root, &seat.entry_operand,
        "the emission environment holds the entry ABI operand at the root position, so this \
         population does not exhibit the shift the checkpoint asks about: {seat:#?}"
    );

    // ⛔ Nor a SHAPE mismatch. The displacing operand is the same lowering shape,
    // which is exactly why no contract check at the seam can see the difference.
    assert_eq!(
        d3c_shape(at_root),
        d3c_shape(&seat.entry_operand),
        "the two operands differ in lowering shape, so the difference above could be read as a \
         representation mismatch rather than a moved position: {seat:#?}"
    );

    // A SHIFT, not an absence: the entry value is still in the environment, once,
    // somewhere else. Uniqueness matters — two occurrences would make "where it
    // is" ambiguous and the measured position meaningless.
    let occurrences = seat
        .emission_environment
        .iter()
        .enumerate()
        .filter(|(_, operand)| **operand == seat.entry_operand)
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let [immediate] = occurrences.as_slice() else {
        panic!(
            "the entry ABI operand must appear exactly once in the emission environment for its \
             immediate position to be well defined, found it at {occurrences:?}: {seat:#?}"
        );
    };
    assert_ne!(
        *immediate as u32, seat.source_abi_position,
        "the measured immediate position equals the root ABI position, which is the very \
         equality this measurement exists to test: {seat:#?}"
    );

    // The instrument agrees with the control's own derivation.
    assert_eq!(
        seat.observed_position,
        Some(*immediate as u32),
        "the instrument's measured position disagrees with the position this control derived \
         from the recorded environment: {seat:#?}"
    );
    assert_eq!(
        seat.observed_operand, seat.entry_operand,
        "reading the measured immediate position must yield the entry operand: {seat:#?}"
    );

    // ⭐ Condition 4 — substituting the root ABI position for the measured
    // immediate one FLIPS, and flips on operand identity. Production is
    // otherwise unchanged; only the position this instrument reads moves.
    let substituted = d3c_observe(D3cPositionSelection::SourceAbiPosition);
    let substituted = d3c_shifted(&substituted);
    let [substituted] = substituted.as_slice() else {
        panic!("the substituted run must reach the same one seat, got {substituted:#?}");
    };
    assert_eq!(
        substituted.observed_position,
        Some(seat.source_abi_position),
        "the substitution must read the root ABI position: {substituted:#?}"
    );
    assert_ne!(
        substituted.observed_operand, substituted.entry_operand,
        "substituting the root ABI position must yield a DIFFERENT operand than the entry walk \
         recorded — if it yields the same one, the position did not move and there is nothing to \
         correct: {substituted:#?}"
    );
    assert_eq!(
        d3c_shape(&substituted.observed_operand),
        d3c_shape(&substituted.entry_operand),
        "the substituted operand must be the same lowering shape, so the flip is carried by \
         identity alone: {substituted:#?}"
    );
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D7a` — the composed worker view is the
/// selecting unit's own worker, or one of the named refusals.**
///
/// The subject is the *specification* of
/// [`StaticTransitionPlan::composed_worker_view`], asserted exhaustively over
/// the planned population rather than against a literal census: group the units
/// by the **four-field causal selector** — producer `Construct` occurrence,
/// computational-frame origin, selected alternative, ruled recursive position —
/// and for each group the projection must
///
/// 1. refuse with the conflict message if the members disagree about the
///    worker — an ambiguous selector has no worker for any later question to be
///    about;
/// 2. otherwise refuse with the template-only message if the agreed raw worker
///    body is superseded — the selected recursive argument's route is raw
///    unconditionally, so that body is a target with a descriptor and no
///    emitted `Function`;
/// 3. otherwise answer, field for field, with that group's own key facts.
///
/// ## The four selector controls, each independent of the other three
///
/// Every field is perturbed **with the other three held exact**, so a refusal is
/// attributable to that field alone and no control is propped up by its
/// neighbours. The perturbed value is one the plan really has — an origin it
/// carries, the next alternative, the next position — so a selector that ignored
/// a field would answer rather than refuse.
///
/// ⭐ The producer `Construct` origin gets a **second, stronger** control that
/// the other three cannot have here: on both plans two layers share the other
/// three fields, so transplanting one layer's construct origin onto the other's
/// triple must return **that layer's** worker. Refusing is not the property —
/// *selecting* is. Different workers under distinct construct origins are
/// distinct questions, not a conflict, and this is what measures that.
///
/// **MEASURED**, each by a mutation on the compiling plans rather than by
/// assertion: clauses 2 and 3 both fire non-vacuously; each of the four selector
/// fields is load-bearing with the others exact; and the body-child and
/// ordered-capture re-checks stay independently live — comparing the body
/// against its closure instead of its body child, and shifting a capture
/// ordinal, each red this row with their own message.
///
/// **CLAIMED**: clause 1 — that two specializations answering one four-field
/// selector with different workers refuse rather than one being chosen.
///
/// **THE GAP**: with the causal field in the key, every group on both plans is a
/// singleton, so clause 1 is **unexercised** — read it as untested. That is the
/// inverse of the three-field selector's gap, where clause 1 was the only
/// reachable outcome, and it is the direction that matters: the unexercised
/// branch is now the *refusal*, not the answer.
///
/// **Promise class: durable invariant.**
#[test]
fn d7a_the_composed_worker_view_is_the_selecting_units_own_worker_or_a_named_refusal() {
    use crate::cranelift_backend::planning::{
        ComposedWorkerRouteEligibility, ContinuationEmissionOwner,
    };
    use std::collections::{BTreeMap, BTreeSet};

    type Selector = (
        ContinuationEmissionOwner,
        StaticOriginId,
        StaticOriginId,
        u32,
        u32,
    );

    fn selector_of(unit: &ContinuationUnitView<'_>) -> Selector {
        (
            unit.emission_owner(),
            unit.producer_construct_origin(),
            unit.continuation_origin(),
            unit.producer_alternative(),
            unit.recursive_position(),
        )
    }

    fn check(plan: &StaticTransitionPlan<'_>) {
        let units = plan.continuation_units().expect("continuation units");
        let template_only = plan
            .template_only_worker_bodies()
            .expect("the superseded set");
        assert!(
            !units.is_empty(),
            "the plan must intern at least one continuation specialization, or every clause \
             below quantifies over nothing"
        );

        let mut groups: BTreeMap<Selector, Vec<usize>> = BTreeMap::new();
        for (index, unit) in units.iter().enumerate() {
            groups.entry(selector_of(unit)).or_default().push(index);
        }

        let mut reached_template_only = 0usize;
        let mut reached_answer = 0usize;

        for (selector, members) in &groups {
            let answered = plan.composed_worker_view(
                selector.0, selector.1, selector.2, selector.3, selector.4,
            );

            let identities = members
                .iter()
                .map(|index| {
                    let unit = &units[*index];
                    (
                        unit.worker_closure_origin(),
                        unit.worker_body_origin(),
                        unit.worker_declared_arity(),
                        unit.worker_capture_count(),
                    )
                })
                .collect::<BTreeSet<_>>();
            if identities.len() > 1 {
                let refusal = answered
                    .expect_err("a group whose members name different workers must refuse");
                assert!(
                    format!("{refusal:?}").contains("different full worker identities"),
                    "the refusal must be the conflict one, and specifically NOT the \
                     template-only one: an ambiguous group has no agreed body for that \
                     question to be about, and reporting it would bury the ambiguity: \
                     {refusal:?}"
                );
                continue;
            }

            let unit = &units[members[0]];
            if template_only.contains(&unit.worker_body_origin()) {
                let refusal = answered.expect_err(
                    "a resolved group whose raw worker body is superseded must refuse: the \
                     selected recursive argument calls that body unconditionally",
                );
                assert!(
                    format!("{refusal:?}").contains("template-only"),
                    "the refusal must be the unexecutable-raw-target one: {refusal:?}"
                );
                reached_template_only += 1;
                continue;
            }

            let view = answered.expect("a group that resolves must answer");
            assert_eq!(
                (
                    view.closure_origin(),
                    view.body_origin(),
                    view.declared_arity(),
                    view.captures().len(),
                    view.recursive_position(),
                ),
                (
                    unit.worker_closure_origin(),
                    unit.worker_body_origin(),
                    unit.worker_declared_arity(),
                    unit.worker_capture_count(),
                    unit.recursive_position(),
                ),
                "the projection must be the selecting unit's own worker facts, not a plausible \
                 reconstruction of them"
            );
            let expected = match plan
                .continuation_context_for(unit.id(), unit.worker_body_origin())
                .expect("the context lookup answers")
            {
                Some(context) => {
                    ComposedWorkerRouteEligibility::GeneratedContextIssued(context.id())
                }
                None => ComposedWorkerRouteEligibility::RawOnly,
            };
            assert_eq!(
                view.route_eligibility(),
                expected,
                "route eligibility must be the planner's own singleton context resolution for \
                 this exact (specialization, worker body), never a re-derivation from whichever \
                 target exists"
            );
            reached_answer += 1;
        }

        // ⛔ One control per selector field, each with the other three EXACT, so
        // no control leans on another.
        //
        // ⚠ The substituted origin is COMPUTED, not picked. Reaching for a
        // convenient neighbour — the worker body — silently selected the *other*
        // layer, because in a nested specialization the inner layer's producer
        // `Construct` IS the outer layer's worker body. That control answered
        // instead of refusing, and it was right to: it had named a real
        // four-field key. So the substitute must be an origin the plan carries
        // and that no unit claims in either origin position.
        let claimed = units
            .iter()
            .flat_map(|unit| [unit.producer_construct_origin(), unit.continuation_origin()])
            .collect::<BTreeSet<_>>();
        let foreign = units
            .iter()
            .flat_map(|unit| [unit.worker_closure_origin(), unit.worker_body_origin()])
            .find(|origin| !claimed.contains(origin))
            .expect(
                "the plan must carry some origin no specialization claims as a construct or                  frame origin, or these two controls cannot be posed at all",
            );
        // ⛔ `D8a` — an owner no unit at any selector carries. Computed the
        // same way and for the same reason as `foreign` above: reaching for a
        // neighbouring owner would name a real one.
        let claimed_owners = units
            .iter()
            .map(|unit| unit.emission_owner())
            .collect::<BTreeSet<_>>();
        let foreign_owner = units
            .iter()
            .map(|unit| ContinuationEmissionOwner::Predeclared(unit.consumer_owner()))
            .chain(
                units
                    .iter()
                    .map(|unit| ContinuationEmissionOwner::Predeclared(unit.producer_owner())),
            )
            .find(|owner| !claimed_owners.contains(owner))
            .expect(
                "the plan must carry some predeclared function no specialization names as an \
                 emission owner, or the owner control cannot be posed at all",
            );
        for unit in &units {
            let exact = selector_of(unit);
            for (label, perturbed) in [
                (
                    "emission owner",
                    (foreign_owner, exact.1, exact.2, exact.3, exact.4),
                ),
                (
                    "producer Construct occurrence",
                    (exact.0, foreign, exact.2, exact.3, exact.4),
                ),
                ("frame origin", (exact.0, exact.1, foreign, exact.3, exact.4)),
                (
                    "selected alternative",
                    (exact.0, exact.1, exact.2, exact.3 + 1, exact.4),
                ),
                (
                    "recursive position",
                    (exact.0, exact.1, exact.2, exact.3, exact.4 + 1),
                ),
            ] {
                let refusal = plan
                    .composed_worker_view(
                        perturbed.0, perturbed.1, perturbed.2, perturbed.3, perturbed.4,
                    )
                    .expect_err(
                        "a selector no specialization claims must refuse, so that a consumer \
                         cannot be handed a neighbour's worker",
                    );
                assert!(
                    format!("{refusal:?}").contains("no continuation specialization claims"),
                    "perturbing the {label} — with the other three fields exact — must reach the \
                     ZERO-answer refusal, which is what shows THAT field participates in the \
                     selection on its own: {refusal:?}"
                );
            }
        }

        // ⭐ The transplant. For two layers sharing the other three fields, the
        // construct origin must SELECT, not merely be compared: each layer's own
        // origin under the shared triple must produce that layer's own worker.
        let mut transplanted = 0usize;
        for left in &units {
            for right in &units {
                if left.id() == right.id()
                    || left.continuation_origin() != right.continuation_origin()
                    || left.producer_alternative() != right.producer_alternative()
                    || left.recursive_position() != right.recursive_position()
                {
                    continue;
                }
                assert_ne!(
                    left.producer_construct_origin(),
                    right.producer_construct_origin(),
                    "two distinct specializations sharing all three source-text fields must \
                     differ in the causal one, or the four-field selector is no more \
                     discriminating than the three-field one it replaced"
                );
                let transplant = plan.composed_worker_view(
                    right.emission_owner(),
                    right.producer_construct_origin(),
                    left.continuation_origin(),
                    left.producer_alternative(),
                    left.recursive_position(),
                );
                let expected_body = right.worker_body_origin();
                match transplant {
                    Ok(view) => assert_eq!(
                        view.body_origin(),
                        expected_body,
                        "transplanting the construct origin must answer with the worker of the \
                         layer that origin names, not the layer whose other three fields were \
                         supplied"
                    ),
                    Err(refusal) => assert!(
                        template_only.contains(&expected_body)
                            && format!("{refusal:?}").contains("template-only"),
                        "the only lawful refusal for a transplant that names a real layer is \
                         that layer's own superseded body: {refusal:?}"
                    ),
                }
                transplanted += 1;
            }
        }
        assert!(
            transplanted > 0,
            "the plan must carry two layers sharing the three source-text fields, or the \
             transplant control measures nothing and the causal field is untested here"
        );

        assert!(
            reached_template_only > 0,
            "the unexecutable-raw-target refusal must be reached, or clause 2 is asserted over a \
             population that never exhibits it"
        );
        assert!(
            reached_answer > 0,
            "the POSITIVE answer must be reached. Without this the projection would be all \
             refusals — the exact shape that made the three-field selector unfit, and a \
             projection nothing can ever successfully ask is not a projection"
        );
    }

    // ⭐ Both plans in this crate that intern continuation specializations. The
    // D5a witness's workers carry NO captures, so on it alone the capture
    // provenance re-check quantifies over an empty run and a mutation that
    // breaks it stays green -- measured, not assumed. `contspec_nested_fixture`
    // carries one capture per worker, which is what reaches that clause.
    with_d5a_witness_plan(check);
    let expr = crate::cranelift_backend::planning::contspec_nested_fixture();
    let nested = plan_static_transition_graph(&expr, &BTreeMap::new())
        .expect("the nested continuation fixture plans");
    check(&nested);
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D7a` — the three source-text fields collide
/// exactly where the causal field resolves.**
///
/// The computational-frame origin, the selected alternative and the ruled
/// recursive position are all properties of the **source text**. When one source
/// computational match is specialized at more than one recursion layer, every
/// layer shares all three, and the layers are distinguished only by their
/// producer `Construct` occurrence.
///
/// ⭐ That is not a fixture accident. Both plans in this crate that intern
/// continuation specializations are nested, because nesting is what
/// `RT-CONTSRC-PRODUCER-LOCAL` is about.
///
/// This row is the standing proof that the causal field earns its place: the
/// three-text-field grouping must have a colliding group whose members name
/// **different** workers, while every four-field group is a **singleton** and no
/// four-field selector anywhere reaches the conflict refusal. Drop the field and
/// the first half stays true while the second half fails, which is exactly the
/// defect it was added to fix.
///
/// **Promise class: durable invariant.** The subject is the relation between the
/// two groupings over whatever population the plans carry, not a census of
/// either.
#[test]
fn d7a_the_three_field_selector_collides_where_the_four_field_selector_resolves() {
    use crate::cranelift_backend::planning::ContinuationEmissionOwner;
    use std::collections::{BTreeMap, BTreeSet};

    fn check(plan: &StaticTransitionPlan<'_>) {
        let units = plan.continuation_units().expect("continuation units");

        let mut by_text: BTreeMap<(StaticOriginId, u32, u32), Vec<usize>> = BTreeMap::new();
        let mut by_cause: BTreeMap<
            (
                ContinuationEmissionOwner,
                StaticOriginId,
                StaticOriginId,
                u32,
                u32,
            ),
            Vec<usize>,
        > = BTreeMap::new();
        for (index, unit) in units.iter().enumerate() {
            let text = (
                unit.continuation_origin(),
                unit.producer_alternative(),
                unit.recursive_position(),
            );
            by_text.entry(text).or_default().push(index);
            by_cause
                .entry((
                    unit.emission_owner(),
                    unit.producer_construct_origin(),
                    text.0,
                    text.1,
                    text.2,
                ))
                .or_default()
                .push(index);
        }

        let collided = by_text
            .values()
            .find(|members| members.len() > 1)
            .expect(
                "the plan must specialize one source match at more than one layer; with every \
                 source-text group a singleton the causal field would be redundant and this row \
                 would be measuring nothing",
            );
        let workers = collided
            .iter()
            .map(|index| {
                (
                    units[*index].worker_closure_origin(),
                    units[*index].worker_body_origin(),
                )
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(
            workers.len(),
            collided.len(),
            "the colliding layers must name DIFFERENT static workers. If they named the same one \
             the collision would be in name only, the three-field selector would resolve, and the \
             causal field would be carrying no weight"
        );

        for (selector, members) in &by_cause {
            assert_eq!(
                members.len(),
                1,
                "every four-field group must be a singleton; {selector:?} has {} members, which \
                 would mean the causal coordinate does not separate what the source text conflates",
                members.len()
            );
            // ⛔ And no four-field selector may reach the CONFLICT refusal —
            // singleton groups and a conflicting answer would mean the selector
            // in production reads fewer fields than this grouping does.
            if let Err(refusal) =
                plan.composed_worker_view(
                    selector.0, selector.1, selector.2, selector.3, selector.4,
                )
            {
                assert!(
                    !format!("{refusal:?}").contains("different full worker identities"),
                    "a four-field selector must never reach the conflict refusal when its group \
                     is a singleton: {refusal:?}"
                );
            }
        }
    }

    with_d5a_witness_plan(check);
    let expr = crate::cranelift_backend::planning::contspec_nested_fixture();
    let nested = plan_static_transition_graph(&expr, &BTreeMap::new())
        .expect("the nested continuation fixture plans");
    check(&nested);
}


/// Arm one `D8b` target-minting defect for one closure, then restore it.
///
/// ⛔ The restore is unconditional. A defect left armed leaks into every later
/// row on this thread, and the failure mode is silent: the next row's population
/// is wrong and its assertions still pass.
fn with_d8b_target_defect<T>(
    defect: crate::cranelift_backend::planning::ComposedCallTargetDefect,
    f: impl FnOnce(&StaticTransitionPlan<'_>) -> T,
) -> T {
    use crate::cranelift_backend::planning::{
        set_composed_call_target_defect, ComposedCallTargetDefect,
    };
    set_composed_call_target_defect(defect);
    let outcome = with_d5a_witness_plan(f);
    set_composed_call_target_defect(ComposedCallTargetDefect::Exact);
    outcome
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8b` — a composed-call target is its
/// selector's own callee, and the law that says so is live.**
///
/// The subject is the specification of
/// [`StaticTransitionPlan::composed_call_targets`] and its one law, asserted
/// over the planned population rather than against a census:
///
/// 1. **One target per exact `D8a` selector**, in bijection with the distinct
///    selectors the planner interned — not one per unit, and not one per body.
/// 2. **Each carries the whole view its own selector resolves to**, so the
///    callee and its calling provenance cannot disagree.
/// 3. **The two minting defects refuse**, each at its own point: a wrong body
///    leaves the selector intact and is caught by selector agreement; a
///    transplanted construct origin pairs one layer's owner with another's
///    construct — a pair no unit carries — and is refused by the selector itself,
///    one step earlier.
/// 4. **Nothing here asks an executability question.** Minting is driven from
///    the unreconciled resolution, so a target exists for the layer whose raw
///    body is superseded, even though `composed_worker_view` refuses for that
///    same selector. That divergence is the no-circularity property made
///    observable: if minting had gated on executability, the two would agree and
///    this clause would be untestable.
///
/// **MEASURED**: all four, on the real witness, over a non-empty population that
/// contains **both** a selector whose reconciled view answers and one whose
/// reconciled view refuses — without which clause 4 is asserting a divergence
/// the population cannot exhibit.
///
/// **Promise class: durable invariant.**
#[test]
fn d8b_a_composed_call_target_is_its_own_selectors_callee() {
    use crate::cranelift_backend::planning::ComposedCallTargetDefect;
    use std::collections::BTreeSet;

    with_d8b_target_defect(ComposedCallTargetDefect::Exact, |plan| {
        let units = plan.continuation_units().expect("units");
        let targets = plan.composed_call_targets().expect("the targets mint");
        assert!(
            !targets.is_empty(),
            "the witness must mint at least one composed-call target, or every clause here is \
             vacuous"
        );

        // Clause 1 — bijection with the distinct selectors, stated as set
        // equality so a duplicate and an omission are both caught.
        let selectors = units
            .iter()
            .map(|unit| {
                (
                    unit.emission_owner(),
                    unit.producer_construct_origin(),
                    unit.continuation_origin(),
                    unit.producer_alternative(),
                    unit.recursive_position(),
                )
            })
            .collect::<BTreeSet<_>>();
        let minted = targets
            .iter()
            .map(|target| target.selector())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            minted, selectors,
            "the minted selectors must be exactly the interned ones"
        );
        assert_eq!(
            minted.len(),
            targets.len(),
            "and one target each: a repeated selector means two callees for one call site"
        );

        // Clause 2 — the carried view is the one its own selector resolves to,
        // and the gate agrees over the whole population.
        assert_eq!(
            plan.verify_composed_call_targets()
                .expect("the exact population must satisfy selector agreement"),
            targets.len(),
            "the gate must report the whole population it checked"
        );

        // Clause 4 — minting does not ask the executability question.
        //
        // ⛔ The non-vacuity clause is the load-bearing half. Both outcomes must
        // be present: a selector whose reconciled view answers and one whose
        // reconciled view refuses. With only the first, a minting pass that DID
        // gate on executability would satisfy every assertion here.
        let mut answered = 0usize;
        let mut refused = 0usize;
        for target in &targets {
            let (owner, construct, frame, alternative, position) = target.selector();
            match plan.composed_worker_view(owner, construct, frame, alternative, position) {
                Ok(view) => {
                    assert_eq!(
                        &view,
                        target.worker(),
                        "where the reconciled view answers it must be the same worker the target \
                         carries; two spellings of one callee is the drift this design avoids"
                    );
                    answered += 1;
                }
                Err(refusal) => {
                    assert!(
                        format!("{refusal:?}").contains("template-only"),
                        "the only lawful refusal for a minted selector is its superseded body: \
                         {refusal:?}"
                    );
                    refused += 1;
                }
            }
        }
        assert!(
            answered > 0 && refused > 0,
            "the population must contain BOTH a selector the reconciled view answers and one it \
             refuses ({answered} answered, {refused} refused). A target minted for the refused \
             one is the whole no-circularity property: minting is driven from resolution alone, \
             so it does not wait on an answer D8c owns"
        );
    });

    // Clause 3 — the two minting defects, each at its own point.
    for (defect, expected, why) in [
        (
            ComposedCallTargetDefect::WrongBody,
            "minted for one layer",
            "the selector still resolves, so only the worker comparison can see this",
        ),
        (
            ComposedCallTargetDefect::TransplantConstruct,
            "no continuation specialization claims",
            "owner and construct come from different layers, a pair no unit carries",
        ),
    ] {
        with_d8b_target_defect(defect, |plan| {
            let refusal = plan
                .verify_composed_call_targets()
                .expect_err("an inconsistently minted target must refuse at the law");
            assert!(
                format!("{refusal:?}").contains(expected),
                "{defect:?} must reach its own refusal — {why} — not one it also happens to trip \
                 further along: {refusal:?}"
            );
        });
    }
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8a` — the emission owner is a function of the
/// four source coordinates, and the released fork resolves to that branch.**
///
/// ## The fork, measured before it was chosen
///
/// Either the planner structurally forbids two emission owners for one set of
/// four source coordinates, or it does not and the owner is a real
/// discriminator. **It forbids them, for two independent reasons**, and the
/// second was found by trying to violate the first.
///
/// **Reason one — the walks are disjoint.** `continuation_result_origins` does
/// not descend into `Closure` or `LexicalClosure`; both sit in its no-descent
/// arm. Every descent root is `worker.body_origin`, a closure's own body child.
/// So for one `continuation_origin` the seed walk stops at exactly the closure
/// whose body a descent later roots at, and two descent roots are
/// nested-or-disjoint because origins form a tree. A producer `Construct` is
/// reached by exactly one discovery, and its emission owner — decided solely by
/// that discovery's `enclosing_specialization` — is fixed by where it sits.
///
/// **Reason two — availability, and this one is measured.**
/// `set_continuation_descent_owner_duplication` removes reason one exactly:
/// every descent is pushed a second time with `enclosing_specialization: None`,
/// so the same nested producers are discovered as though top-level. That does
/// **not** yield two owners. Planning refuses first, on both plans, with
///
/// ```text
/// a continuation coordinate is not present in the lexical environment in force
/// at the emission seat
/// ```
///
/// ⇒ A nested producer's continuation coordinate is not available in the raw
/// owner's environment, so the second owner cannot be constructed even when the
/// traversal is made to offer it. That is the `D5a` availability law standing
/// behind the traversal, and it is why this is a structural fact rather than an
/// artifact of one walk's shape.
///
/// **MEASURED**: the invariant holds exhaustively on both plans over a
/// non-degenerate population — more than one source coordinate, and more than
/// one owner across them, so "one owner each" is distinguished from "one owner
/// overall". Removing reason one produces a refusal, not a collision, on both
/// plans, with the disarmed run as its positive control.
///
/// ⇒ **`D8b` amendment: there is no owner-collision refusal.** One was encoded
/// here and is deleted. Its population is proved impossible by the two reasons
/// above, so it was a check that could never fail — not defence in depth, and
/// not a residual worth carrying. What this row measures is the *impossibility*,
/// which is the real guarantee; the deleted guard only restated it where it
/// could not be exercised.
///
/// The owner's *selector* role is separately live: supplying an owner no unit
/// carries reaches the zero-answer refusal, controlled in
/// [`d7a_the_composed_worker_view_is_the_selecting_units_own_worker_or_a_named_refusal`].
///
/// **Promise class: durable invariant.**
#[test]
fn d8a_one_emission_owner_answers_one_composed_source_coordinate() {
    use crate::cranelift_backend::planning::{
        set_continuation_descent_owner_duplication, ContinuationEmissionOwner,
    };
    use std::collections::{BTreeMap, BTreeSet};

    type Source = (StaticOriginId, StaticOriginId, u32, u32);

    fn owners_by_source(
        plan: &StaticTransitionPlan<'_>,
    ) -> BTreeMap<Source, BTreeSet<ContinuationEmissionOwner>> {
        let mut grouped: BTreeMap<Source, BTreeSet<ContinuationEmissionOwner>> = BTreeMap::new();
        for unit in plan.continuation_units().expect("units").iter() {
            grouped
                .entry((
                    unit.producer_construct_origin(),
                    unit.continuation_origin(),
                    unit.producer_alternative(),
                    unit.recursive_position(),
                ))
                .or_default()
                .insert(unit.emission_owner());
        }
        grouped
    }

    let exact = |plan: &StaticTransitionPlan<'_>| {
        let grouped = owners_by_source(plan);
        for (source, owners) in &grouped {
            assert_eq!(
                owners.len(),
                1,
                "source coordinate {source:?} carries {owners:?}. A producer Construct is reached \
                 by exactly one continuation discovery, so more than one owner here means the \
                 seed and descent walks stopped being disjoint"
            );
        }
        // ⛔ Two non-vacuity clauses, and the second is the one that matters.
        // With a single coordinate the law is trivially true; with a single
        // owner in the whole plan, "one owner per coordinate" is
        // indistinguishable from "one owner overall" and the row would stay
        // green under a planner that had lost the distinction entirely.
        assert!(
            grouped.len() > 1,
            "the plan must carry more than one source coordinate, or the law holds trivially"
        );
        let distinct = grouped.values().flatten().copied().collect::<BTreeSet<_>>();
        assert!(
            distinct.len() > 1,
            "the plan must carry at least two DISTINCT emission owners across its coordinates, \
             or nothing here separates 'one owner per coordinate' from 'one owner overall': \
             {distinct:?}"
        );
        assert!(
            distinct
                .iter()
                .any(|owner| matches!(owner, ContinuationEmissionOwner::Predeclared(_)))
                && distinct
                    .iter()
                    .any(|owner| matches!(owner, ContinuationEmissionOwner::Specialization(_))),
            "and the two must be one of each CLASS -- two predeclared ids would leave the \
             specialization-owned arm unrepresented: {distinct:?}"
        );
    };

    // ⛔ The positive control, run FIRST and with the hook provably disarmed:
    // both plans build. Without it, the refusal below is equally consistent with
    // a witness that never planned.
    set_continuation_descent_owner_duplication(false);
    with_d5a_witness_plan(exact);
    let expr = crate::cranelift_backend::planning::contspec_nested_fixture();
    let nested = plan_static_transition_graph(&expr, &BTreeMap::new())
        .expect("the nested continuation fixture plans with the hook disarmed");
    exact(&nested);

    // ── removing reason one does not produce reason one's collision ─────
    let plan_both = |armed: bool| {
        set_continuation_descent_owner_duplication(armed);
        let (entry, declarations) =
            crate::cranelift_backend::test_objects::px8tr_nested_post_effect_planning_inputs();
        let declarations = declarations
            .iter()
            .map(|declaration| (declaration.symbol.as_str(), declaration))
            .collect::<BTreeMap<_, _>>();
        let witness = plan_static_transition_graph_with_symbols(
            &entry,
            &declarations,
            &crate::NativeProcessSymbols::legacy_prelude(),
            AbiRootIngress::Value,
            true,
        )
        .map(|_| ())
        .map_err(|error| format!("{error:?}"));
        let expr = crate::cranelift_backend::planning::contspec_nested_fixture();
        let nested = plan_static_transition_graph(&expr, &BTreeMap::new())
            .map(|_| ())
            .map_err(|error| format!("{error:?}"));
        set_continuation_descent_owner_duplication(false);
        (witness, nested)
    };

    let (witness, nested) = plan_both(true);
    for (label, outcome) in [("the D5a witness", witness), ("contspec_nested", nested)] {
        let refusal = outcome.expect_err(&format!(
            "{label}: discovering a nested producer as though top-level must refuse. If it \
             planned, the second owner IS constructible and D8a resolves to the discriminator \
             branch instead -- which would make the invariant above wrong, not merely untested"
        ));
        assert!(
            refusal.contains("not present in the lexical environment in force at the emission seat"),
            "{label}: the refusal must be the availability law -- that is the second, independent \
             reason the second owner cannot exist. Another refusal would mean the duplication \
             broke something else and says nothing about owners: {refusal}"
        );
    }
}


/// **`RT-MATCH-SCRUTINEE-DISPOSITION` `AC-6` — the first hash-pinned
/// intersection fixture moves to `FunctionizedUnits` and installs its planned
/// bindings.**
///
/// `RT-CONTSRC-PRODUCER-LOCAL` `D8d` originally pinned the opposite route: the
/// deferred fixture reached the composed recursive site under the retired
/// monolithic route, so it had no defining emission owner and installed no
/// target-derived binding. `D3-narrow` deliberately changes that premise. The
/// ordinary producer route accepts this intersection fixture, so its two
/// reached recursive positions now each install their planned binding.
///
/// This is not a new consumption claim. The fixture still consumes none of
/// those bindings; `D8e`'s separate witness remains the positive consumer. The
/// `D5a` population still carries targets without reaching this site, which is
/// the independent no-site control.
///
/// The binding is `D8d`'s whole deliverable and it is deliberately unreadable
/// until `D8e` supplies its consumer. That makes it indistinguishable, from the
/// outside, from a binding that was never built at all — so this row measures
/// the difference rather than asserting it, with a counter at the site and a
/// counter at the installation.
///
/// ## What is measured
///
/// 1. The composed deferred-constructor site is reached exactly twice through
///    a real object emission.
/// 2. Exactly two target-derived bindings are installed, one per reached site,
///    proving this intersection fixture took the narrowed ordinary route.
/// 3. Neither binding is consumed by this fixture.
/// 4. The `D5a` witness never reaches the composed site.
///
/// **Promise class: durable transition sentinel.** The exact tuple records the
/// route change that `D3-narrow` is supposed to make while keeping `D8e`'s
/// distinct positive-consumer population honest.
#[test]
fn d8d_the_composed_binding_site_tracks_the_narrowed_intersection_route() {
    use crate::cranelift_backend::lowering::{
        d8d_bindings, d8d_recursive_sites, d8e_consumptions, reset_d8d_bindings,
    };

    // (1), (2), and (3) — both live sites install, but neither consumes.
    reset_d8d_bindings();
    let deferred = RuntimeExpr::Match {
        scrutinee: Box::new(px8j_deferred_recursive_field_fixture()),
        cases: ["ctor:prelude::Result::Err", "ctor:prelude::Result::Ok"]
            .into_iter()
            .map(|constructor| RuntimeMatchCase {
                constructor: constructor.to_string(),
                binders: 1,
                body: RuntimeExpr::Construct {
                    constructor: crate::EXIT_SUCCESS_CONSTRUCTOR.to_string(),
                    args: Vec::new(),
                },
            })
            .collect(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "D8d composed site".to_string(),
        },
    };
    let (result, _trace) =
        px8j_capture_source_trace(&deferred, false, "ken_d8d_composed_site");
    result.expect("the deferred-constructor producer path lowers");
    let (sites, bindings) = (d8d_recursive_sites(), d8d_bindings());
    assert_eq!(
        (sites, bindings, d8e_consumptions()),
        (2, 2, 0),
        "D3-narrow must move this intersection fixture onto FunctionizedUnits: each reached \
         composed site installs its target-derived binding, while D8e's distinct consumer stays \
         absent"
    );

    // (4) — the independent target-bearing population never reaches the site.
    reset_d8d_bindings();
    crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
        "ken_d8d_witness_site",
        false,
    )
    .expect("the D5a witness compiles");
    assert_eq!(
        (d8d_recursive_sites(), d8d_bindings(), d8e_consumptions()),
        (0, 0, 0),
        "the D5a witness -- the plan D8b's target population was measured on -- must not reach \
         the composed site. It is the control that keeps D8e's witness honest: if this fixture \
         reached the site too, D8e's positive route could be inherited from it rather than built"
    );
}

// ── `RT-CONTSRC-PRODUCER-LOCAL` `D8e` — the witness ────────────────────────
//
// The `D8d` sentinel measured two populations that never meet: the fixture that
// REACHES the composed deferred-constructor site carries no defining emission
// owner, and the one plan that carries composed-call TARGETS never reaches the
// site. This fixture is the first program in which all four facts hold at once,
// through the ordinary production planner/lowering path:
//
//   1. `requires_heterogeneous_deforestation` on the selected constructor field;
//   2. an `immediate_binder_eliminator` bridge over that field's binder;
//   3. a functionized-unit definition, so `defining_emission_owner` is `Some`;
//   4. an interned specialization at the exact `D8a` five-field selector.
//
// ⭐ Each of the four is load-bearing and each was reached by MEASUREMENT, not
// by design. Three shapes were rejected on the way, and what each rejected is
// worth recording, because they are the constraints the shape is pinned by:
//
//   - The declaration body may not BE the `ComputationalMatch`. A
//     declaration-owned unit's source root is its planned seed node, and for
//     that shape the seed is the producer `Construct` -- so the continuation
//     value-environment walk starts BELOW its own continuation and the planner
//     refuses with "computational continuation is outside its source owner
//     subtree". The `Let` wrapper is what puts the walk above it.
//   - The wrapper may not be a `Match` whose computational scrutinee the
//     ordinary producer route declines. That is the narrowed
//     retired scrutinee residual, which selected the monolithic route. That
//     route defined no units, so fact 3 failed silently and the composed site
//     was reached with no owner.
//   - The selected field's arms must be statically selectable. A field whose
//     arms merge at runtime materializes a source join whose planned
//     representation is derived from the field's OWN arms (specialized
//     constructors, hence native scalar lanes) while lowering merges the
//     COMPOSED result there -- and `D8e`'s consumer returns a unit-call carrier.
//     Raising that join by giving an arm a carried field instead makes the
//     scrutinee carried, which the deferred-constructor case refuses outright.
//     ⇒ The two requirements are satisfiable together only where no merge is
//     built, which is what the compile-time `Bool::True` scrutinee arranges.
//
// ⚠ The witness does NOT compile, and the row asserts that refusal rather than
// hiding it. See the row's own header for what it is and is not evidence of.

/// **`D8e` — installation and consumption meet, and the causal projection the
/// meeting inherits refuses one plane later.**
///
/// ⭐ **What this row IS evidence of.** On a program built entirely through the
/// ordinary production planner and lowering path, the `D8d` binding is installed
/// at the selected recursive source-order position, the `D8e` consumer resolves
/// the exact source-machine `Var` callee to that binding *before* the value-only
/// `Var` path, and the shared route-selected emitter writes the call with the
/// exact raw operand run. Both counters the `D8d` sentinel pinned at zero
/// transition, and the emitted call is read back from the emitter's own log
/// rather than from the binding that requested it.
///
/// ⭐⭐ **The `D5a` detached-result seat is PASSED, and that is `D8h`-`D8k`'s
/// repair landing.** As written at `89e36ec1` this row refused there, and said
/// so: interning the specialization that supplies the `D8a` target necessarily
/// projects a causal call onto the same emitting unit, and that edge had
/// exactly two discharges -- a claim at `claim_and_call_continuation`, which
/// the composed path returns before, or a unit result that IS the producer
/// constructor, which the composed path exists to eliminate. The escalation was
/// right and the answer was a third discharge: `D8k` makes the residual filter
/// read "what NEITHER verified form has discharged", so a composed source
/// continuation clears its own edge. This row's refusal moving is the evidence
/// that landed.
///
/// ⛔ **What it is still NOT evidence of, said plainly.** The program does not
/// compile. It now stops LATER and elsewhere, building the specialization's
/// case binder run, because the producer's ordinary envelope carries no
/// nonrecursive field at the selected field's source position. That is a
/// different frontier, in specialization emission rather than causal closure,
/// and it is out of `D8k`'s scope -- reported, not accommodated.
///
/// **Transition sentinel, on the NEW boundary.** The row now pins that the
/// detached seat is *not* reached and that the envelope construction is. It
/// reds when either half moves -- if the detached seat returns, the `D8k`
/// filter has regressed; if the envelope refusal goes away, a composed witness
/// compiles end to end for the first time and the counter clauses above it
/// become a full positive route.
#[test]
fn d8e_the_composed_binding_is_installed_consumed_and_clears_its_own_causal_edge() {
    let (error, counters, markers) = d8e_witness_compile("d8e_witness", 3, true);
    let (sites, bindings, consumptions) = counters;

    assert!(
        sites > 0,
        "the witness must REACH the composed deferred-constructor site, or every clause below is \
         vacuous"
    );
    assert_eq!(
        (bindings, consumptions),
        (1, 3),
        "the two zeros the D8d sentinel pinned must BOTH transition on one program: one \
         target-derived StaticWorkerBinding installed at the selected recursive position, and \
         source-machine Var callees resolved to a static worker. A (1, 0) here means the binding \
         is still unreadable and D8e's consumer never fired. ⚠ The consumption count is THREE \
         and only one of them is composed -- see the facet assertion below, which is what makes \
         the number attributable instead of merely observed"
    );
    // `D8l2` item 8 — attribute the three consumptions.
    //
    // ⭐ Recorded at the seat from each binding's own facet. Two of the three
    // are `DirectSpecializationCall`: a specialization's own selected recursive
    // argument and its induction hypothesis are ordinary bindings that reach
    // the same seat, and they answer for no causal obligation. ⛔ It could not
    // be otherwise and still compile -- two composed claims of one identity
    // would have refused as a double discharge -- but this measures the
    // attribution rather than inferring it from that argument.
    let facets = crate::cranelift_backend::lowering::d8l2_consumed_facets();
    assert_eq!(
        (
            facets.len(),
            facets.iter().filter(|composed| **composed).count()
        ),
        (3, 1),
        "exactly one of the three consumptions may carry a composed authority; the other two are \
         ordinary. Facets: {facets:?}"
    );

    // The emitter's own record, not the binding's. ⛔ Read back from
    // `WorkerCallEmitted`, which is written AFTER the instruction exists, so
    // this is a fact about emission rather than about the request.
    let emitted = markers
        .iter()
        .filter_map(|event| match event {
            D5aMarkerEvent::WorkerCallEmitted {
                raw_operands,
                supplied_operands,
                route,
                ..
            } => Some((*raw_operands, *supplied_operands, *route)),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        emitted.len(),
        consumptions,
        "the shared emitter must write exactly one call per consumption -- stated as a relation \
         against the consumption count, not as a literal, because that count is a property of \
         how far this program now compiles rather than of the law. Emitted: {emitted:?}"
    );
    assert!(
        emitted
            .iter()
            .all(|entry| *entry == (1, 1, StaticWorkerCallRoute::RawWorker)),
        "and every one must receive the EXACT raw run: one source argument, zero stored captures \
         (every worker in this witness is capture-free), no generated-context suffix, and the \
         raw route D8d fixes for a selected recursive argument. A supplied count above the raw \
         count means a suffix was appended to a raw target: {emitted:?}"
    );

    // ⭐⭐ `D8l2` item 5 — THE WITNESS COMPILES. Installation, consumption,
    // verified discharge, the causal partition and specialization emission all
    // complete on one program built through the ordinary production path.
    //
    // ⛔ Asserted with the error attached: this row has moved twice, and each
    // time the useful fact was WHERE it stopped. A refusal here is a
    // regression, and which one it is decides who owns it.
    assert!(
        error.is_none(),
        "the composed witness must now compile end to end. If this refuses at the \
         detached-result seat, D8k's residual filter has regressed; if at the ordinary envelope, \
         D8l2's source-position population has; anything else is a new finding: {error:?}"
    );
    // And the causal ledger closed with a NON-EMPTY composed half -- the reach
    // that D8k's law row could not have, recorded beside it rather than in
    // place of it.
    assert_eq!(
        crate::cranelift_backend::lowering::d8j_discharged().len(),
        1,
        "one verified composed discharge must have entered the relation and survived whole-pass \
         closure. Zero means the program compiled without the composed half of the partition \
         ever being populated, which would make its closure vacuous"
    );
    // ⚠ The separate assertion that the outer raw-body unit-result closure
    // refusal is not reached is RETIRED, not dropped. `D8e`'s law names it, and
    // while this witness refused somewhere it was worth pinning which refusal
    // it gave. Now that the program compiles, `error.is_none()` above says no
    // refusal is reached at all -- that one among them -- and a separate clause
    // keyed on its message would be a check that can no longer fail.
}

/// **`D8e` control 1 — the consumption is attributable to the EXACT binding.**
///
/// ⛔ The perturbation moves the callee's index by one, onto the induction
/// hypothesis that sits immediately before the `D8d` binding in the same
/// environment. It is not a fabricated index: it names a real, live, adjacent
/// binding of the same call arity, so a consumer that resolved "some callable in
/// scope" rather than "this binding" would still fire.
///
/// ⭐ Installation is UNCHANGED, and that is the half that makes this a
/// discriminator rather than a smoke test: the binding is still installed, so
/// the zero consumption count is attributable to the callee alone.
#[test]
fn d8e_a_neighbouring_callee_installs_the_binding_and_consumes_nothing() {
    let (_error, (sites, bindings, consumptions), markers) =
        d8e_witness_compile("d8e_neighbour", 2, true);
    assert!(sites > 0, "the control must reach the same composed site");
    assert_eq!(
        (bindings, consumptions),
        (1, 0),
        "moving the callee one index onto the neighbouring induction hypothesis must leave \
         installation untouched and consume NOTHING. A (1, 1) here means D8e's consumer resolves \
         something weaker than the exact Var binding"
    );
    assert!(
        !markers.iter().any(|event| matches!(
            event,
            D5aMarkerEvent::WorkerCallEmitted { .. }
        )),
        "and with nothing consumed the shared emitter must not have written a worker call"
    );
}

/// **`D8e` control 2 — the value-only path still fails closed.**
///
/// The bridge becomes an ordinary `Match`, so its case bodies are lowered by
/// `lower_expr` rather than by the source machine, and the callee index moves to
/// the `D8d` binding's position in THAT environment. Same binding, same call,
/// same arity -- only the lowering path differs.
///
/// ⭐ This is what proves `D8e`'s consumer is the sole lawful way to read the
/// capsule, and that `D8d`'s fail-closed property survives it: off the
/// source-machine path the binding is refused in value position rather than
/// silently read.
#[test]
fn d8e_off_the_source_machine_path_the_binding_is_refused_in_value_position() {
    let (error, (sites, bindings, consumptions), _markers) =
        d8e_witness_compile("d8e_value_position", 2, false);
    assert!(sites > 0, "the control must reach the same composed site");
    assert_eq!(
        (bindings, consumptions),
        (1, 0),
        "the ordinary bridge installs the SAME binding -- installation is the outer frame's work \
         and does not depend on the bridge -- and consumes nothing, because D8e's consumer sits \
         on the source-machine Call arm alone"
    );
    let reason = format!("{:?}", error.expect("the value-position read must refuse"));
    assert!(
        reason.contains("a static worker binding has no value representation"),
        "and the refusal must be D8d's own fail-closed value-position guard, not an incidental \
         downstream failure: {reason}"
    );
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8h` — every composed-call target carries the
/// causal identity its own coordinate selects, and nothing weaker could have
/// chosen it.**
///
/// The subject is the pairing added to
/// [`StaticTransitionPlan::composed_call_targets`]: one opaque
/// [`ContinuationCallIdentity`] per target, resolved through the planner's own
/// four-field lookup and held to the coordinate's fifth field.
///
/// ## Clause 1 — planner authority
///
/// The expected identity is derived **in this test** by calling
/// `continuation_call_binding_for` on the target's own coordinate, and compared
/// against what the target carries. ⛔ Two reads of the target would agree with
/// each other whatever the minting did; this compares the target against the
/// authority it claims to have used. The identity is compared **whole**, so the
/// opaque call-site sequence participates without this row being able to see
/// it — which is the property a sequence accessor would have destroyed.
///
/// ## Clause 2 — nothing weaker could have chosen it
///
/// **MEASURED**, on the witness plan: the two targets agree on their producer's
/// **constructor symbol identity**, on declared **arity**, on **capture count**,
/// and on all three source-position fields — continuation origin, producer
/// alternative and recursive position — and carry **different** call identities.
///
/// **CLAIMED**: therefore none of constructor-symbol equality, arity, capture
/// count or source position can be the rule that assigned them, because each is
/// constant across a pair the pairing separates. Only the producer `Construct`
/// origin (and with it the emission owner) distinguishes the two, which is
/// `D7a`'s finding arriving at the identity.
///
/// **THE GAP**: the two workers' **body origins do differ** (each layer's
/// producer construct is the other's worker body), so a body-keyed rule is
/// excluded by the release's rule and by inspection of the minting code, **not**
/// by this measurement. This row cannot separate it, and says so rather than
/// implying a coverage it does not have.
///
/// ## Clause 3 — the forbidden rule's own answer is refused
///
/// `SameSymbolIdentity` installs exactly what a constructor-symbol-keyed pairing
/// would have produced, found by searching the population for that equality
/// rather than by hand. It leaves the selector and the worker untouched, so it
/// passes selector agreement and can only be caught by the pairing law.
///
/// **Promise class: durable invariant.** It asserts relations over the planned
/// population — equality against an independently resolved authority, and a
/// separation between two targets — never a count or a literal identity.
#[test]
fn d8h_a_composed_call_target_carries_the_identity_its_own_coordinate_selects() {
    use crate::cranelift_backend::planning::ComposedCallTargetDefect;

    with_d8b_target_defect(ComposedCallTargetDefect::Exact, |plan| {
        let targets = plan.composed_call_targets().expect("the targets mint");
        assert!(
            !targets.is_empty(),
            "the witness must mint at least one composed-call target, or every clause here is \
             vacuous"
        );

        // Clause 1 — against the planner's own lookup, not against the target.
        for target in &targets {
            let (owner, construct, frame, alternative, position) = target.selector();
            let authority = plan
                .continuation_call_binding_for(construct, frame, alternative, position)
                .expect("the lookup resolves")
                .expect(
                    "every minted target's coordinate must select a planner-issued call binding; \
                     a None here means the interned units and the call tokens have drifted apart \
                     and the target is unpaired",
                );
            assert_eq!(
                &authority,
                target.call_identity(),
                "the carried identity must be the one this target's own coordinate selects"
            );
            assert_eq!(
                target.call_identity().emission_owner(),
                owner,
                "and the coordinate's fifth field must be the identity's own emission owner: the \
                 interned unit and the call token are two derivations of who emits this call, and \
                 the pairing is only five-field if they are held to agree"
            );
        }

        // Clause 2 — the separated pair, and what it rules out.
        let mut separated = None;
        for (index, left) in targets.iter().enumerate() {
            for right in targets.iter().skip(index + 1) {
                let same_symbol = plan
                    .constructor_symbol_identity(left.selector().1)
                    .expect("symbol identity")
                    == plan
                        .constructor_symbol_identity(right.selector().1)
                        .expect("symbol identity");
                if same_symbol && left.call_identity() != right.call_identity() {
                    separated = Some((left, right));
                }
            }
        }
        let (left, right) = separated.expect(
            "the population must contain two targets whose producers name ONE constructor symbol \
             and whose call identities DIFFER. Without that pair, a pairing keyed on the \
             constructor symbol would agree with this one everywhere, and clause 3 would be \
             refusing a defect the population cannot actually exhibit",
        );
        assert_eq!(
            (
                left.worker().declared_arity(),
                left.worker().captures().len(),
                left.selector().2,
                left.selector().3,
                left.selector().4,
            ),
            (
                right.worker().declared_arity(),
                right.worker().captures().len(),
                right.selector().2,
                right.selector().3,
                right.selector().4,
            ),
            "and the separated pair must also agree on arity, capture count, continuation origin, \
             alternative and recursive position -- each of those is a rule the release forbids, \
             and each is ruled out only while it is CONSTANT across a pair the pairing separates. \
             If this ever differs, that reconstruction rule has become able to discriminate here \
             and is no longer excluded by measurement"
        );
        assert_ne!(
            left.worker().body_origin(),
            right.worker().body_origin(),
            "THE GAP, asserted so it cannot drift silently: the body origins DO differ on this \
             population, so a body-keyed rule is excluded by the release and by the minting code, \
             not by the separation above. If these ever became equal, this row would gain that \
             coverage and this assertion should be inverted rather than deleted"
        );
    });

    // Clause 3 — the forbidden rule's own answer, refused at the pairing law.
    with_d8b_target_defect(ComposedCallTargetDefect::SameSymbolIdentity, |plan| {
        let refusal = plan
            .verify_composed_call_targets()
            .expect_err("a symbol-keyed identity must refuse at the pairing law");
        let refusal = format!("{refusal:?}");
        assert!(
            refusal.contains("does not select"),
            "SameSymbolIdentity must reach the PAIRING law, not selector agreement one step \
             earlier: the selector and worker are untouched, so a refusal from anywhere else \
             means the switch perturbed more than the identity: {refusal}"
        );
    });
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8i` — the discharge facet is transported,
/// stated at every site, and refuses both ways it can be wrong.**
///
/// The subject is [`ContinuationDischarge`] as a **separate closed facet** from
/// [`StaticWorkerCallRoute`]: the route decides callee and operand run, the
/// discharge decides which causal obligation a consumption may answer for.
///
/// ## Clause 1 — both arms occur, each at the site its role dictates
///
/// **MEASURED** on the `D8e` witness: the composed selected recursive argument
/// carries [`ContinuationDischarge::ComposedSourceContinuation`] with the exact
/// identity `D8h` paired to that target — same emission owner, same target
/// specialization — and it is the **only** composed record on that program.
/// **MEASURED** on the `D5a` witness, which builds induction hypotheses and
/// specialization-owned recursive arguments through the same constructor: every
/// record is [`ContinuationDischarge::DirectSpecializationCall`].
///
/// **CLAIMED**: the facet is stated per site rather than defaulted or inferred.
/// A default would make one arm universal; an inference from `route` would tie
/// it to `RawWorker`, which **both** populations carry — the composed argument
/// and the ordinary hypothesis are route-identical here, which is precisely why
/// route cannot be the discriminator and why this is a second field.
///
/// **THE GAP**: "no default" is also a type-level fact — the constructor takes
/// the facet as a required argument, so omission is a compile error. That half
/// is not asserted here, because asserting it would mean testing the source
/// text. It was observed instead: adding the parameter reded three existing
/// call sites until each stated its arm.
///
/// ## Clause 2 — wrong-owner authority is refused at construction
///
/// An ordinary site is handed a **real** planner-issued authority whose
/// emission owner is not the defining one. It cannot be fabricated —
/// `ContinuationCallIdentity` has no constructor outside planning — so the
/// switch searches the target population for one, which is what makes the
/// refusal attributable to the owner disagreement rather than to a malformed
/// value.
///
/// ## Clause 3 — an ordinary binding is rejected, not answered with `None`
///
/// [`StaticWorkerBinding::composed_continuation_authority`] refuses on the
/// direct arm. A caller reaching it has already decided it is discharging a
/// composed obligation; "there isn't one" is an error, not an absence to be
/// `unwrap_or_default`-ed past.
///
/// **Promise class: durable invariant.** Relations over the two populations and
/// two refusals, never a count of bindings.
#[test]
fn d8i_the_discharge_facet_is_transported_stated_and_refuses_both_ways() {
    use crate::cranelift_backend::lowering::{
        d8i_discharges, reset_d8d_bindings, set_d8i_foreign_authority,
    };

    // Clause 1a — the composed program.
    let (error, (_sites, bindings, consumptions), _markers) =
        d8e_witness_compile("d8i_composed", 3, true);
    assert_eq!(
        (bindings, consumptions),
        (1, 3),
        "the witness must still install and consume exactly as D8e measured it; D8i changes \
         transport only, and a drift here means the facet altered the binding path"
    );
    assert!(
        error.is_none(),
        "and the witness must still compile underneath D8i: D8i changed transport only and must \
         not move that outcome either way: {error:?}"
    );
    let records = d8i_discharges();
    let composed = records
        .iter()
        .filter_map(|record| record.composed.as_ref())
        .collect::<Vec<_>>();
    assert_eq!(
        composed.len(),
        1,
        "exactly one binding on this program may carry a composed authority -- the selected \
         recursive argument D8d installs. More means the facet leaked onto an ordinary binding; \
         none means it was dropped in transport. Records: {records:?}"
    );
    let plan_target = with_d8e_witness_plan(|plan| {
        let targets = plan.composed_call_targets().expect("targets");
        assert_eq!(
            targets.len(),
            1,
            "the witness plan mints one target, so the identity below is unambiguous"
        );
        (
            targets[0].call_identity().emission_owner(),
            targets[0].call_identity().target(),
        )
    });
    assert_eq!(
        *composed[0], plan_target,
        "and the transported authority must be D8h's pairing for that exact target, derived here \
         from the plan rather than read back off the binding: transport that rebuilt the identity \
         would agree with itself but not with the planner"
    );

    // Clause 1b — the ordinary program, through the same constructor.
    reset_d8d_bindings();
    crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
        "ken_d8i_ordinary",
        false,
    )
    .expect("the D5a witness compiles");
    let ordinary = d8i_discharges();
    assert!(
        !ordinary.is_empty(),
        "the D5a witness must build static-worker bindings, or clause 1b is vacuous and proves \
         nothing about the ordinary arm"
    );
    assert!(
        ordinary.iter().all(|record| record.composed.is_none()),
        "every binding the D5a witness builds -- induction hypotheses and the specialization's \
         own selected recursive argument -- must carry the DIRECT arm. A composed authority here \
         would mean the specialization path had acquired one it never asked for: {ordinary:?}"
    );

    // Clause 2 — a real foreign authority at an ordinary site.
    reset_d8d_bindings();
    set_d8i_foreign_authority(true);
    let foreign = crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
        "ken_d8i_foreign",
        false,
    );
    set_d8i_foreign_authority(false);
    let refusal = format!(
        "{:?}",
        foreign.err().expect(
            "an ordinary binding handed an authority belonging to another emitter must refuse at \
             construction. If this compiles, either the guard is gone or the D5a plan no longer \
             carries a target whose emission owner differs from every defining one -- check which \
             before touching the guard"
        )
    );
    assert!(
        refusal.contains("belongs to a different emitter"),
        "and the refusal must be the owner guard, not an incidental failure downstream of the \
         substitution: {refusal}"
    );

    // Clause 3 — the reader rejects an ordinary binding.
    //
    // ⛔ The binding is built here as a literal, which is the strongest form
    // this clause can take: the composed arm is NOT constructible at this seat
    // even deliberately, because it needs a planner-issued identity. So the
    // only binding a test can hand this accessor is a direct one, and the
    // accessor's contract for that case is the whole subject.
    let ordinary_binding = StaticWorkerBinding {
        closure_origin: inert_test_static_origin(),
        body_origin: inert_test_static_origin(),
        declared_arity: 0,
        captures: Vec::new(),
        route: StaticWorkerCallRoute::RawWorker,
        discharge: ContinuationDischarge::DirectSpecializationCall,
        transport: None,
    };
    let refusal = format!(
        "{:?}",
        ordinary_binding
            .composed_continuation_authority()
            .err()
            .expect(
                "composed_continuation_authority must REFUSE on a direct binding. An Ok here \
                 means an ordinary binding can be read as authorizing a composed discharge; a \
                 None-shaped answer would mean a caller could skip past it with unwrap_or_default"
            )
    );
    assert!(
        refusal.contains("carries no composed causal authority"),
        "and the refusal must name the facet rather than fail for some unrelated reason: {refusal}"
    );
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8j` — the composed authority is discharged
/// once, after the call, and every way of getting there wrongly refuses.**
///
/// ## The positive route
///
/// On the root-owned witness the composed obligation is discharged **exactly
/// once**, and the identity that entered the relation is the one the plan pairs
/// with the target -- derived here from the plan, not read back off the record.
/// Two bindings are installed and one is consumed, so the relation is not
/// merely "one of everything": the second installed binding is never
/// discharged, because it is never consumed.
///
/// ⚠ **The program does not compile**, and this row does not pretend otherwise.
/// It stops later, in specialization emission, at an unrelated
/// ordinary-envelope refusal. The relation is populated before that, at the
/// point the root unit's CLIF is finalized, which is what is being measured.
///
/// ## What "discharge" is NOT
///
/// Installing the binding is not discharge -- two are installed and one is
/// discharged. Beginning the argument run is not discharge -- the run reaches
/// the seat through `CallArgument` and the claim happens after the emitter.
/// Seeing a worker-shaped value is not discharge -- the value-position path
/// refuses (`D8e`'s own row) and never reaches here.
///
/// ## The five discriminators
///
/// | switch | what it makes wrong | where it is caught |
/// |---|---|---|
/// | `SuppressDischargeAfterRealCall` | a real raw call, no record | the relation stays empty |
/// | `SubstituteAnotherExactIdentity` | the other identity at the SAME constructor symbol | verification 1, at the seat |
/// | `WrongClaimingOwner` | a claim from a function that is not the emission owner | verification 2, at the seat |
/// | `RedirectRecordedInstruction` | the record moved onto another real call | verification 4, on the finished CLIF |
/// | `DischargeFromOrdinaryBinding` | an ordinary clone of the same binding | the authority accessor |
/// | `RecordResultDefinedBeforeTheCall` | a value defined before the call | verification 5, on the finished CLIF |
/// | `SupplyOperandCountDisagreesWithTarget` | the reported run, after the real one was emitted | verification 4b, on the finished CLIF |
///
/// ⭐ The substitution is the same-symbol shortcut made concrete: the witness
/// interns **two** specializations at one producer `Construct` -- one
/// constructor symbol, two identities -- so a pairing that keyed on the symbol
/// would have to choose, and this switch installs the choice it would make.
///
/// A sixth switch, `RecordResultDefinedBeforeTheCall`, discriminates
/// verification 5 with a real earlier value of the same function.
///
/// **Verification 4b is discriminated too, and by the one lawful form.**
/// `SupplyOperandCountDisagreesWithTarget` moves the emitter's REPORTED operand
/// run after the real vector has been assembled and emitted. The identity, the
/// paired target, the owner, the recorded instruction, the decoded callee, the
/// downstream result and the source control all stay exact, so 4b is the first
/// refusal and the relation is empty for that reason and no other.
///
/// ⛔ **A whole-target substitution is not a control for it** — verification 1
/// or 4a would refuse first and mask it. That is why the perturbation is of the
/// evidence rather than of the target.
///
/// ⚠ The delta itself is arbitrary, and that is a property of this witness
/// rather than a choice: both its workers declare arity 1 with no captures, so
/// no adjacent real quantity differs from the true run. What carries the
/// control is the isolation — one field moves, every other verifier input stays
/// exact — not the value.
///
/// **Promise class: durable invariant.** Relations over one program's relation
/// and five refusals; the only literals are the arity of the population, which
/// the fixture fixes.
#[test]
fn d8j_the_composed_authority_is_discharged_once_after_the_call() {
    use crate::cranelift_backend::lowering::D8jMutation;

    let (error, discharged, (bindings, consumptions), identities) =
        d8j_root_witness_compile("d8j_exact", D8jMutation::Exact);
    assert_eq!(
        (bindings, consumptions),
        (2, 1),
        "the witness must install TWO composed bindings and consume ONE. Both halves matter: \
         without two installs the relation's single entry could be explained by there being only \
         one binding at all, and without exactly one consumption the discharge count is not \
         attributable to the consumption"
    );
    assert_eq!(
        discharged, 1,
        "exactly one composed obligation is discharged, and it enters the relation only after \
         the finished CLIF has been consulted. Zero means the claim never survived verification; \
         two would mean an installed-but-unconsumed binding also discharged"
    );
    let paired = d8j_root_witness_identities();
    assert!(
        paired.len() == 2 && paired.contains(&identities[0]),
        "the discharged identity must be one the PLAN pairs with a target, taken from the plan \
         rather than from the record: a relation that recorded whatever it was handed would agree \
         with itself. The population is two, which is what gives the substitution switch below \
         something lawful to substitute ({} paired)",
        paired.len()
    );
    let error = format!("{error:?}");
    assert!(
        !error.contains("composed discharge"),
        "and nothing in the D8j gate may be what stops this program: it stops later, in \
         specialization emission. A composed-discharge refusal here means the exact run is \
         failing its own verification: {error}"
    );

    // The five discriminators, each at its own point.
    for (mutation, expect, why) in [
        (
            D8jMutation::SubstituteAnotherExactIdentity,
            "the authority and the callee come from different targets",
            "the other identity at the same constructor symbol names the other position's \
             worker, so the paired target no longer matches the binding being consumed",
        ),
        (
            D8jMutation::WrongClaimingOwner,
            "only the emitting owner may answer",
            "the claim is made by a function that is not the identity's emission owner",
        ),
        (
            D8jMutation::RedirectRecordedInstruction,
            "is not the call the authority stands for",
            "the record names another real call, so the decoded callee disagrees with the \
             D8b/D8d target's raw worker",
        ),
        (
            D8jMutation::DischargeFromOrdinaryBinding,
            "carries no composed causal authority",
            "an ordinary clone of the same binding has no authority to present",
        ),
        (
            D8jMutation::RecordResultDefinedBeforeTheCall,
            "is not defined at or after its",
            "a value defined before the call cannot be what the call returned into the \
             continuation",
        ),
        (
            D8jMutation::SupplyOperandCountDisagreesWithTarget,
            "operands but its D8b/D8d target declares",
            "only the EVIDENCE about the operand run moves: the vector that was written, the \
             identity, the paired target, the owner, the recorded instruction, the decoded \
             callee, the downstream result and the source control are all still exact, so 4b \
             is the FIRST refusal and nothing earlier can be masking it",
        ),
    ] {
        let (error, discharged, _, _) = d8j_root_witness_compile("d8j_defect", mutation);
        assert_eq!(
            discharged, 0,
            "{mutation:?} must leave the relation EMPTY -- {why}"
        );
        let error = format!("{:?}", error.expect("the defect must refuse"));
        assert!(
            error.contains(expect),
            "{mutation:?} must reach its own refusal -- {why} -- not one it also happens to trip \
             further along: {error}"
        );
    }

    // Suppression is the one defect with no refusal: a real raw call is emitted
    // and nothing is recorded. ⛔ That is exactly why the relation has to be
    // asserted positively above rather than inferred from the absence of an
    // error -- this run has no error to distinguish it by.
    let (_error, discharged, (bindings, consumptions), _) =
        d8j_root_witness_compile("d8j_suppressed", D8jMutation::SuppressDischargeAfterRealCall);
    assert_eq!(
        (bindings, consumptions, discharged),
        (2, 1, 0),
        "suppressing the record after a REAL raw call must leave installation and consumption \
         untouched and the relation empty. If discharged is still 1, the relation is being \
         populated by something other than the claim seat"
    );
}

/// The identities the `D8j` witness's own plan pairs with its targets.
#[cfg(test)]
fn d8j_root_witness_identities(
) -> Vec<crate::cranelift_backend::planning::ContinuationCallIdentity> {
    let entry = d8j_root_witness_entry();
    let plan = plan_static_transition_graph_with_symbols(
        &entry,
        &BTreeMap::new(),
        &crate::NativeProcessSymbols::legacy_prelude(),
        AbiRootIngress::Value,
        true,
    )
    .expect("the D8j witness plans");
    plan.composed_call_targets()
        .expect("targets")
        .iter()
        .map(|target| target.call_identity().clone())
        .collect()
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8k` — the causal population is a disjoint
/// partition, and the composed half claims through the same slot the direct
/// half does.**
///
/// The whole-pass closeout no longer says `emitted == planned`. It says
///
/// ```text
/// resolved = declared = planned
/// claimed  = direct-emitted  ⊎  composed-consumed  =  call_obligations
/// ```
///
/// where `call_obligations` is the `DirectCall ∪ ComposedCall` subset `D2`
/// derives from the candidate ledger. Every identity in THIS row is a call
/// obligation, so the two populations coincide here and the clauses below read
/// exactly as they did before `D2`.
///
/// where the two halves are accumulated from two different kinds of evidence:
/// decoded direct-specialization emissions, and verified composed
/// source-continuation consumptions. **Declaration may remain over the full
/// planned set** -- an unused `FuncRef` is not an emitted call -- which is why
/// `declared == planned` survives unchanged beside the partition.
///
/// ## Why this row works on the ledger rather than end to end
///
/// **MEASURED**: no composed witness reaches `close_continuation_claim_ledger`
/// yet. Both stop earlier -- the declaration-owned one in specialization
/// emission at the ordinary-envelope frontier, the root-owned one at the same
/// place. So the closeout's composed half cannot be exercised by compiling a
/// program, and a row that only compiled one would assert nothing about it.
///
/// **CLAIMED**: the ledger is the seat of the law, and it is exercised here
/// with **real planner identities** taken from the `D8e` witness plan through
/// the ordinary projection -- not hand-built, which they cannot be.
///
/// **THE GAP**: this proves the law, not that any program reaches it. When a
/// composed witness compiles to closure, the end-to-end assertion becomes
/// available and should be added beside this rather than replacing it.
///
/// ## Clauses
///
/// 1. **The exact partition closes.** Declare the planned set, discharge the
///    one planned identity compositionally, and the closeout accepts -- with
///    the direct half EMPTY, which is the case the pre-`D8k` law could not
///    express at all.
/// 2. **A duplicate claim refuses.** An identity claimed directly and then
///    discharged compositionally is rejected where the second claim is made,
///    not at the closeout -- one obligation, one form.
/// 3. **An overlap refuses at the closeout.** Recording the same identity into
///    both halves without a second claim reaches the disjointness clause, which
///    is why disjointness is asserted separately from coverage.
/// 4. **A shortfall refuses.** Discharging nothing leaves the planned token in
///    neither half.
/// 4b. **Suppression restores the old refusal, end to end.** Turning the
///    composed discharge off on the declaration-owned witness removes the claim
///    the residual filter reads, so the causal edge is detached again and the
///    `D5a` seat refuses -- exactly where that witness refused at `89e36ec1`.
///    ⭐ This is the strongest available statement that the repair is carried by
///    the discharge rather than by the seat having been loosened.
/// 5. **A wrong owner refuses.** A composed discharge claimed by a function
///    that is not the identity's emission owner is rejected before it can
///    enter either population.
///
/// **Promise class: durable invariant.** Set relations over a planned
/// population the fixture does not fix; the only literal is that the population
/// is non-empty, which is asserted rather than assumed.
#[test]
fn d8k_the_causal_population_is_a_disjoint_partition_of_direct_and_composed() {
    use crate::cranelift_backend::lowering::units::{declare_unit_bundle, ContinuationClaimLedger};

    let entry = d8j_root_witness_entry();
    let plan = plan_static_transition_graph_with_symbols(
        &entry,
        &BTreeMap::new(),
        &crate::NativeProcessSymbols::legacy_prelude(),
        AbiRootIngress::Value,
        true,
    )
    .expect("the witness plans");
    // ⛔ The identities come from the plan's own pairing, which is the only way
    // to obtain one: `ContinuationCallIdentity` has no constructor outside
    // planning, so this row cannot fabricate its population even by accident.
    let identities = plan
        .composed_call_targets()
        .expect("targets")
        .iter()
        .map(|target| target.call_identity().clone())
        .collect::<Vec<_>>();
    assert!(
        !identities.is_empty(),
        "the witness must plan at least one causal identity, or every clause here is vacuous"
    );
    // `D2` — the derived call-obligation subset this control intends. Every
    // identity here IS an obligation: the clauses below discharge them
    // directly or compositionally, or deliberately fail to. Built from the
    // plan's own identities rather than written as a literal.
    let call_obligations = identities.iter().cloned().collect::<std::collections::BTreeSet<_>>();
    let owner = identities[0].emission_owner();
    let open = || {
        let mut module = new_object_module("d8k-partition").expect("module");
        let bundle = declare_unit_bundle(&mut module, &plan).expect("the bundle declares");
        let ledger = ContinuationClaimLedger::open(&plan, &bundle).expect("the ledger opens");
        (ledger, bundle)
    };

    // Clause 1 — the exact partition, with the direct half empty.
    let (mut ledger, _bundle) = open();
    ledger
        .record_declared(identities.iter().cloned())
        .expect("declaration covers the planned set");
    ledger
        .record_composed(identities.iter().cloned(), owner)
        .expect("a verified composed discharge claims its own identity");
    ledger.close(&call_obligations).expect(
        "declared = planned, and claimed = direct ⊎ composed = call_obligations with an EMPTY \
         direct half. A refusal here means the partition still requires a direct emission for \
         every obligation, which is exactly the law D8k replaced",
    );

    // Clause 2 — one obligation, one form.
    let (mut ledger, _bundle) = open();
    ledger
        .record_declared(identities.iter().cloned())
        .expect("declaration covers the planned set");
    ledger
        .claim_exact(&identities[0], owner)
        .expect("the direct claim is lawful on its own");
    let refusal = format!(
        "{:?}",
        ledger
            .record_composed(identities.iter().cloned(), owner)
            .expect_err("a second claim on one identity must refuse")
    );
    assert!(
        refusal.contains("claimed twice"),
        "the duplicate must be caught where the second claim is MADE, so the two forms cannot \
         both report success and leave the closeout to notice: {refusal}"
    );

    // Clause 3 — the disjointness clause at the closeout.
    let (mut ledger, _bundle) = open();
    ledger
        .record_declared(identities.iter().cloned())
        .expect("declaration covers the planned set");
    ledger
        .record_composed(identities.iter().cloned(), owner)
        .expect("the composed discharge claims");
    ledger
        .record_emitted(identities.iter().cloned())
        .expect("recording a direct emission does not itself claim");
    let refusal = format!(
        "{:?}",
        ledger
            .close(&call_obligations)
            .expect_err("an identity in both halves must refuse at the closeout")
    );
    assert!(
        refusal.contains("discharged BOTH"),
        "and it must reach the DISJOINTNESS clause, not the coverage one: the union is complete \
         here, so a law stated only as a union would accept this: {refusal}"
    );

    // Clause 4 — a shortfall.
    let (mut ledger, _bundle) = open();
    ledger
        .record_declared(identities.iter().cloned())
        .expect("declaration covers the planned set");
    let refusal = format!(
        "{:?}",
        ledger
            .close(&call_obligations)
            .expect_err("a planned token in neither half must refuse")
    );
    assert!(
        refusal.contains("neither directly emitted nor compositionally consumed"),
        "and the message must name both halves, so a reader can tell an unemitted token from one \
         discharged the other way: {refusal}"
    );

    // Clause 4b — SUPPRESSION, end to end, on the declaration-owned witness.
    //
    // ⭐⭐ This is the clause that ties `D8j`'s claim to `D8k`'s repair, and it
    // is the only one here that runs a whole compile. Suppressing the composed
    // discharge removes the claim the residual filter reads, so the causal edge
    // is detached again and the `D5a` seat refuses -- exactly where this witness
    // refused at `89e36ec1`.
    //
    // ⛔ It is the strongest available statement that the repair is CARRIED by
    // the discharge rather than by the seat having been loosened: turn the
    // discharge off and the old refusal comes straight back.
    {
        use crate::cranelift_backend::lowering::{set_d8j_mutation, D8jMutation};
        set_d8j_mutation(D8jMutation::SuppressDischargeAfterRealCall);
        let (error, _counters, _markers) = d8e_witness_compile("d8k_suppressed", 3, true);
        set_d8j_mutation(D8jMutation::Exact);
        let refusal = format!(
            "{:?}",
            error.expect("with nothing discharged the causal edge is detached again")
        );
        assert!(
            refusal.contains("detached-result seat"),
            "suppressing the composed discharge must restore the D5a refusal this witness used \
             to give. Any other refusal means the seat is being passed for a reason other than \
             the discharge, and D8k's repair would then be resting on something unmeasured: \
             {refusal}"
        );
    }

    // Clause 5 — wrong owner.
    let (mut ledger, _bundle) = open();
    ledger
        .record_declared(identities.iter().cloned())
        .expect("declaration covers the planned set");
    // ⛔ A REAL emission owner, and it has to come from another plan: this
    // witness's two identities sit at one producer `Construct` and so share one
    // owner. Taken from the `D5a` witness, whose population genuinely carries
    // both an owner class -- `Predeclared` and `Specialization` -- so the value
    // substituted here is one the planner mints rather than one this row
    // invented. Asserting it differs is what keeps the clause from perturbing
    // toward the owner it already has.
    let foreign = with_d5a_witness_plan(|plan| {
        plan.composed_call_targets()
            .expect("targets")
            .iter()
            .map(|target| target.call_identity().emission_owner())
            .find(|candidate| *candidate != owner)
    })
    .expect(
        "no plan in reach carries a second emission owner, so this clause would perturb toward \
         the owner it already has and pass vacuously",
    );
    let refusal = format!(
        "{:?}",
        ledger
            .record_composed([identities[0].clone()], foreign)
            .expect_err("a composed discharge claimed by a non-owner must refuse")
    );
    assert!(
        refusal.contains("is not its emission owner"),
        "and it must refuse at the owner clause, before the identity can enter either \
         population: {refusal}"
    );
}

