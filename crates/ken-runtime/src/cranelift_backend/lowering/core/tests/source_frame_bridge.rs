//! Source-frame bridge, checked-frame consumption and functionized-shared-
//! emitter end-to-end (`RT-CONTROL-INTEGRATION-TESTS-SPLIT` D1, module 4 of
//! 5, split from `control.rs`: `d8l2_*`, `d8m_*`, `d8n_*`, `d8o_*`, `d8p_*`,
//! `d8f_*`, `d8g_*`, `d6b_*`/`d6c_*` (second wave)).

use super::*;
use crate::cranelift_backend::lowering::units::continuation_case_binder_run;
use crate::RuntimeSymbolMetadata;


/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8l2` — the ordinary envelope names SOURCE
/// positions, and the population is right for every orientation.**
///
/// `ordinary_envelope` emitted `0..N` and called it `source_position`, which
/// coincides with the truth only while every nonrecursive field precedes the
/// selected recursive position. `D8l1` measured that with two witnesses
/// differing only in field order; this row pins the repair.
///
/// ## Clause 1 — exact populations
///
/// Selected **first** gives `[1]`, selected **last** gives `[0]`, selected
/// **middle of three** gives `[0, 2]`. ⛔ Asserted as the whole population per
/// orientation, not as a length: the pre-repair derivation produced the right
/// LENGTH in every case and the wrong positions in two of three.
///
/// ## Clause 2 — the defect classes, measured
///
/// | defect | caught | where |
/// |---|---|---|
/// | selection out of range | yes | the planner range refusal added here |
/// | omission | yes | the existing exact `Parameter`-slot reconciliation |
/// | duplication | yes | the case binder run, on the position left uncovered |
/// | dense prefix | yes | the case binder run — this IS the pre-repair defect |
/// | wrong order | yes | clause 1's exact population equality, at the planner |
///
/// ⭐ **Wrong order is caught HERE, at the planner, not downstream.** Clause 1
/// asserts the whole population in order — `[0, 2]` for selected-middle is
/// order-sensitive — so a permuted emission fails that equality. ⛔ There is
/// deliberately no downstream wrong-order refusal and no second slot-order
/// authority: `continuation_case_binder_run` looks a role up by source
/// position, so a *self-consistent consumer permutation* is lawful for it
/// ([`checked_computational_ih_binder_run_admits_a_permuted_envelope`] rules
/// exactly that), and that is a statement about the consumer. It is **not** a
/// licence for the planner to emit anything but source order, which is what
/// this row pins.
///
/// ⛔ Duplication and wrong order are **no-ops on a two-field producer** — one
/// nonrecursive field cannot be reordered or shadowed — so both are exercised
/// on the three-field orientation. A matrix that only ran the two-field shapes
/// would have reported them uncaught for the wrong reason.
///
/// ## Clause 3 — the capture tail, measured
///
/// A witness with a **non-empty nonrecursive prefix and two captures**, in both
/// orientations: the repair renumbers the prefix, so the tail's invariance is
/// only measurable where a prefix exists to renumber, and only two captures can
/// show the tail keeps its ORDER. The full `WorkerCapture` run — ordinal,
/// owner, closure origin, source, lifetime — is compared against the immutable
/// worker provenance the planner interned, not against a second read of the
/// envelope. Envelope length, `header.parameters` and the actual `Parameter`
/// slot count are three independently derived numbers and must all agree.
///
/// ⛔ The production capture loop is byte-identical to `1f9a2020`; the repair
/// touches the nonrecursive prefix and nothing else.
///
/// ## Clause 4 — both orientations reach emission
///
/// Both compile, and their populations differ, so the ordinary values a
/// specialization reads are source-position-dependent. The executing
/// differential lives in its own row,
/// [`d8l2_the_composed_call_returns_the_ordinary_payload_it_consumed`].
///
/// ## Clause 5 — the pre-repair derivation reds where it was wrong
///
/// `DensePrefix` refuses on the selected-first orientation and is a **no-op**
/// on selected-last, where dense index and source position genuinely coincide.
/// That asymmetry is the whole finding: the defect was invisible on every
/// landed fixture because `px8tr` selects its last field.
///
/// **Promise class: durable invariant.** Populations and refusals, no counts
/// standing in for sets.
///
/// [`checked_computational_ih_binder_run_admits_a_permuted_envelope`]:
///     crate::cranelift_backend::lowering::core::tests::control
#[test]
fn d8l2_the_ordinary_envelope_names_source_positions_in_every_orientation() {
    use crate::cranelift_backend::planning::{set_envelope_defect, EnvelopeDefect};

    // Clause 1 — the three exact populations.
    for (fields, recursive, expected) in [
        (2usize, 0usize, vec![1u32]),
        (2, 1, vec![0]),
        (3, 1, vec![0, 2]),
    ] {
        let populations = d8l2_envelope_positions(fields, recursive);
        assert!(
            !populations.is_empty(),
            "the {fields}-field witness selecting {recursive} must intern a continuation, or its \
             population assertion is vacuous"
        );
        for population in &populations {
            assert_eq!(
                population, &expected,
                "the ordinary envelope must name the producer's own source positions with the \
                 selected recursive position omitted. A dense prefix has the right LENGTH here \
                 and the wrong positions, which is why this compares the whole population"
            );
        }
    }

    // Clause 1b — wrong order is caught by that same equality, measured rather
    // than argued. ⛔ Armed on the selected-middle orientation, the only one
    // whose population has an order to get wrong.
    set_envelope_defect(EnvelopeDefect::WrongOrder);
    let permuted = d8l2_envelope_positions(3, 1);
    set_envelope_defect(EnvelopeDefect::Exact);
    assert!(
        permuted.iter().all(|population| *population == vec![2u32, 0]),
        "the wrong-order defect must actually permute the planned population, or clause 1's \
         equality is not what rejects it and this row is claiming a check it does not have: \
         {permuted:?}"
    );
    assert!(
        permuted.iter().all(|population| *population != vec![0u32, 2]),
        "and the permuted population must DIFFER from source order, which is exactly what \
         clause 1 asserts against -- so a planner that emitted this would red there"
    );

    // Clause 3 — the capture tail, byte-for-byte against immutable provenance.
    //
    // ⭐ A witness with a non-empty nonrecursive prefix AND two captures, in
    // both orientations: the repair renumbers the prefix, so the tail's
    // invariance is only measurable where a prefix exists to renumber.
    // ⛔ The comparison is against `ComposedCallTarget`'s own worker
    // provenance -- the immutable planner record -- not against a second read
    // of the envelope.
    for worker_last in [false, true] {
        let entry = d8l2_capture_witness(worker_last);
        let plan = plan_static_transition_graph_with_symbols(
            &entry,
            &BTreeMap::new(),
            &crate::NativeProcessSymbols::legacy_prelude(),
            AbiRootIngress::Value,
            true,
        )
        .expect("the capture witness plans");
        let targets = plan.composed_call_targets().expect("targets");
        assert!(
            !targets.is_empty(),
            "the capture witness must mint a target, or its provenance side is vacuous"
        );
        let provenance = targets[0]
            .worker()
            .captures()
            .iter()
            .map(|capture| {
                (
                    capture.ordinal(),
                    capture.owner(),
                    capture.closure_origin(),
                    capture.source(),
                    capture.lifetime(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            provenance.len(),
            2,
            "the witness must carry TWO captures; one cannot show that the tail keeps its ORDER"
        );
        for unit in plan.continuation_units().expect("units") {
            let envelope = unit.ordinary_envelope().expect("the envelope builds");
            let tail = envelope
                .iter()
                .filter_map(|role| match role {
                    crate::cranelift_backend::planning::ContinuationOrdinaryEnvelopeRole::WorkerCapture {
                        ordinal,
                        owner,
                        closure_origin,
                        source,
                        lifetime,
                    } => Some((*ordinal, *owner, *closure_origin, *source, *lifetime)),
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(
                tail, provenance,
                "the WorkerCapture tail must equal the immutable worker provenance in order, \
                 field for field -- ordinal, owner, closure origin, source and lifetime. The \
                 repair renumbers only the nonrecursive prefix; a tail that moved would mean it \
                 reached past its own population"
            );
            let parameter_slots = unit
                .slots()
                .iter()
                .filter(|slot| {
                    slot.kind == crate::cranelift_backend::planning::AbiSlotKind::Parameter
                })
                .count();
            assert_eq!(
                (envelope.len(), unit.header().parameters as usize, parameter_slots),
                (3, 3, 3),
                "and the envelope length, the header's declared parameter count and the ACTUAL \
                 Parameter-slot run must all still agree: one nonrecursive field plus two \
                 captures. These are three independently derived numbers, not three reads of one"
            );
        }
    }

    // Clause 4 — both orientations reach emission.
    for (fields, recursive) in [(2usize, 0usize), (2, 1), (3, 1)] {
        assert!(
            d8l2_compile(fields, recursive).is_none(),
            "the {fields}-field witness selecting {recursive} must reach emission; before the \
             repair the selected-first orientation refused at the case binder run"
        );
    }

    // Clauses 2 and 5 — the defect matrix, with the shapes that need three
    // fields run on three fields.
    for (defect, fields, recursive, expected) in [
        (
            EnvelopeDefect::SelectionOutOfRange,
            2usize,
            0usize,
            Some("outside its producer constructor's field run"),
        ),
        (
            EnvelopeDefect::Omit,
            2,
            0,
            Some("does not cover its Parameter slot run"),
        ),
        (
            EnvelopeDefect::Duplicate,
            3,
            1,
            Some("has no nonrecursive field at source position 2"),
        ),
        (
            EnvelopeDefect::DensePrefix,
            2,
            0,
            Some("has no nonrecursive field at source position 1"),
        ),
        // ⭐ The asymmetry that IS the finding: dense prefix is a no-op exactly
        // where the two derivations agree.
        (EnvelopeDefect::DensePrefix, 2, 1, None),
        // ⚠ Wrong order is NOT checked here: it is a planner-population fault,
        // and clause 1 below is where it is caught. Compiling under it is
        // expected, and says only that the consumer follows a self-consistent
        // permutation -- which is the landed ruling, not a licence.
        (EnvelopeDefect::WrongOrder, 3, 1, None),
    ] {
        set_envelope_defect(defect);
        let outcome = d8l2_compile(fields, recursive);
        set_envelope_defect(EnvelopeDefect::Exact);
        match expected {
            Some(reason) => {
                let refusal = format!(
                    "{:?}",
                    outcome.unwrap_or_else(|| panic!(
                        "{defect:?} on the {fields}-field witness selecting {recursive} must \
                         refuse; if it compiles, that defect class is no longer live and the \
                         matrix above is overstating this row's coverage"
                    ))
                );
                assert!(
                    refusal.contains(reason),
                    "{defect:?} must reach its own refusal, not one it also happens to trip \
                     further along: {refusal}"
                );
            }
            None => assert!(
                outcome.is_none(),
                "{defect:?} on the {fields}-field witness selecting {recursive} must be a NO-OP. \
                 A refusal here means it is no longer the shape this row claims it is: {:?}",
                outcome
            ),
        }
    }
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8l2` — the composed call returns the ordinary
/// payload it consumed, and the answer depends on the source position.**
///
/// ⭐⭐ **This is the executing differential, and it is the strongest evidence
/// the repair is about values rather than about a population.** The witness's
/// worker returns its argument, and the bridge hands it the payload its own
/// case matched — so the payload is **consumed** by the real composed call and
/// reaches the program's answer. It is not merely carried in the envelope, and
/// it is not used only to choose a case.
///
/// Both orientations execute; two distinguishable payloads produce two exactly
/// distinct ground observations. ⛔ Asserted as the exact value, not as
/// "different from each other": two wrong answers can differ.
///
/// The composed path is asserted to be the one taken — one target-derived
/// binding installed, a verified composed discharge in the relation — so this
/// cannot be an ordinary call that happens to return the right number, and the
/// end-to-end compile and `D8h`–`D8k` ledger facts are carried here beside the
/// differential rather than assumed from another row.
///
/// `DensePrefix` still refuses on the selected-first orientation and is still a
/// no-op on selected-last. That asymmetry is the regression control: the defect
/// was invisible on every landed fixture because `px8tr` selects its last field.
///
/// **Promise class: durable invariant.** The observations are the payloads the
/// fixture supplies, so a changed fixture moves both sides together.
#[test]
fn d8l2_the_composed_call_returns_the_ordinary_payload_it_consumed() {
    use crate::cranelift_backend::lowering::{
        d8d_bindings, d8j_discharged, reset_d8d_bindings, reset_d8j_discharged,
    };
    use crate::cranelift_backend::planning::{set_envelope_defect, EnvelopeDefect};

    for worker_last in [false, true] {
        for payload in [41i64, 58] {
            reset_d8d_bindings();
            reset_d8j_discharged();
            let expr = d8l2_payload_witness(worker_last, payload);
            let compiled = compile_expr(&expr, &NativeSeedEnvironment::empty()).unwrap_or_else(
                |error| {
                    panic!(
                        "the payload witness must compile in both orientations. Selected-first \
                         refusing at the ordinary envelope means D8l2's source-position \
                         population has regressed: {error:?}"
                    )
                },
            );
            assert_eq!(
                compiled.run(None).expect("the payload witness runs").0,
                RuntimeObservation::Returned(RuntimeGroundValue::Int(payload.into())),
                "the composed call must return the ordinary payload it consumed. ⛔ The EXACT \
                 value, not merely one that differs from the other payload's: two wrong answers \
                 can differ from each other"
            );
            // ⛔ And it must be the composed path that produced it. Without
            // this the differential would be satisfied by any lowering that
            // happened to thread the payload through.
            assert_eq!(
                (d8d_bindings(), d8j_discharged().len()),
                (1, 1),
                "one target-derived binding installed and one verified composed discharge in the \
                 relation, so this answer came through the composed call and its causal \
                 obligation closed"
            );
        }
    }

    // The regression control, on the orientation the defect was invisible on
    // and the one it was not.
    for (worker_last, refuses) in [(false, true), (true, false)] {
        set_envelope_defect(EnvelopeDefect::DensePrefix);
        let outcome = compile_expr(&d8l2_payload_witness(worker_last, 41), &NativeSeedEnvironment::empty());
        set_envelope_defect(EnvelopeDefect::Exact);
        assert_eq!(
            outcome.is_err(),
            refuses,
            "the pre-repair dense-prefix derivation must refuse on the selected-FIRST \
             orientation and be a no-op on selected-last, where dense index and source position \
             genuinely coincide. That asymmetry is why the defect survived every landed fixture"
        );
    }
}

/// The `D8l2` composed witness with a `CheckedSubcontinuationFrame` around the
/// **bridge** — the case body `immediate_binder_eliminator` selects — so the
/// source frame identity has something to be preserved through.
#[cfg(test)]
fn d8m_witness(frame_id: u64) -> RuntimeExpr {
    let wrap = "ctor:fixture::D8M::Wrap";
    let done = "ctor:fixture::D8M::Done";
    let unit = || RuntimeExpr::Construct {
        constructor: "ctor:prelude::Unit::MkUnit".to_string(),
        args: Vec::new(),
    };
    let worker = RuntimeExpr::LexicalClosure {
        captures: Vec::new(),
        params: vec!["carried".to_string()],
        body: Box::new(RuntimeExpr::Var(0)),
    };
    let selected_field = RuntimeExpr::Match {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: "ctor:prelude::Bool::True".to_string(),
            args: Vec::new(),
        }),
        cases: [
            ("ctor:prelude::Bool::True", "ctor:prelude::Result::Ok"),
            ("ctor:prelude::Bool::False", "ctor:prelude::Result::Err"),
        ]
        .into_iter()
        .map(|(constructor, result)| RuntimeMatchCase {
            constructor: constructor.to_string(),
            binders: 0,
            body: RuntimeExpr::Construct {
                constructor: result.to_string(),
                args: vec![RuntimeExpr::Value(RuntimeValue::Int((41).into()))],
            },
        })
        .collect(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "d8m field default".to_string(),
        },
    };
    let bridge = RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Var(1)),
        cases: ["ctor:prelude::Result::Err", "ctor:prelude::Result::Ok"]
            .into_iter()
            .map(|constructor| crate::RuntimeComputationalMatchCase {
                constructor: constructor.to_string(),
                argument_binders: 1,
                recursive_positions: vec![0],
                body: RuntimeExpr::Call {
                    callee: Box::new(RuntimeExpr::Var(4)),
                    args: vec![RuntimeExpr::Var(1)],
                },
            })
            .collect(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "d8m bridge default".to_string(),
        },
    };
    // ⭐ The marker sits on the case body the bridge is built FROM -- the only
    // position where `immediate_binder_eliminator` can see it.
    let marked_bridge = RuntimeExpr::CheckedSubcontinuationFrame {
        frame_id,
        body: Box::new(bridge),
    };
    let eliminator = RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: wrap.to_string(),
            args: vec![selected_field, worker],
        }),
        cases: vec![
            crate::RuntimeComputationalMatchCase {
                constructor: wrap.to_string(),
                argument_binders: 2,
                recursive_positions: vec![1],
                body: marked_bridge,
            },
            crate::RuntimeComputationalMatchCase {
                constructor: done.to_string(),
                argument_binders: 0,
                recursive_positions: Vec::new(),
                body: unit(),
            },
        ],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "d8m eliminator default".to_string(),
        },
    };
    RuntimeExpr::Let {
        value: Box::new(eliminator),
        body: Box::new(RuntimeExpr::Var(0)),
    }
}

#[cfg(test)]
fn d8m_plan(expr: &RuntimeExpr, frame_id: u64) -> crate::OrientedSubcontinuationPlanV1 {
    let RuntimeExpr::Let { value, .. } = expr else { panic!("let") };
    let RuntimeExpr::ComputationalMatch { cases, .. } = value.as_ref() else {
        panic!("eliminator")
    };
    let RuntimeExpr::CheckedSubcontinuationFrame { body, .. } = &cases[0].body else {
        panic!("marked bridge")
    };
    let RuntimeExpr::ComputationalMatch { cases, default, .. } = body.as_ref() else {
        panic!("bridge")
    };
    let mut frame = crate::OrientedSubcontinuationFramePlanV1 {
        frame_id,
        segment_site_id: 9,
        declaration: "<entry>".to_string(),
        checked_occurrence_path: vec![frame_id],
        semantic_position: 0,
        input_interface: oriented_test_interface(1),
        output_interface: oriented_test_interface(2),
        runtime_frame_fingerprint:
            crate::compiler_private_computational_match_frame_fingerprint(cases, default),
        occurrence_binding_fingerprint: 0,
        control_witness: crate::OrientedControlWitnessV1::DistinguishedRoot,
    };
    frame.occurrence_binding_fingerprint =
        crate::compiler_private_oriented_occurrence_binding_fingerprint(&frame);
    crate::OrientedSubcontinuationPlanV1 {
        representation_rule_version:
            crate::OrientedSubcontinuationPlanV1::REPRESENTATION_RULE_VERSION,
        frames: vec![frame],
        recursive_calls: Vec::new(),
        computational_ih_slots: Vec::new(),
        computational_ih_calls: Vec::new(),
    }
}

#[cfg(test)]
fn d8m_compile(expr: &RuntimeExpr, frame_id: u64) -> Option<CraneliftBackendError> {
    compile_expr_into_module(
        new_object_module("d8m").expect("module"),
        "ken_d8m",
        Linkage::Export,
        expr,
        &NativeSeedEnvironment::empty(),
        BTreeMap::new(),
        None,
        true,
        None,
        Some(test_only_distinguished_root_join_plan()),
        Some(d8m_plan(expr, frame_id)),
    )
    .err()
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8m` — the source match's checked-frame
/// identity survives the `immediate_binder_eliminator` bridge.**
///
/// The bridge is an **optimization of the source match, not a new semantic
/// frame**. Before `D8m` it always carried `checked_frame_id: None`, so a
/// checked IH slot inside a composed case body refused as *"detached from its
/// checked frame"* — the `D8f` hard stop.
///
/// ## Clause 1 — the missing-slot guard boundary. NOT a transport proof.
///
/// With the marker on the case body the bridge is built from, the program stops
/// at **"checked computational case is missing its IH slot marker"** rather than
/// at the `D8f` hard stop. This clause pins that boundary and nothing more.
/// Completing the program needs IH slot markers in the bridge's cases, which is
/// `D8f`'s business and is held.
///
/// **This witness does not establish that the bridge transported an identity,
/// and this clause must not be read as saying so.** It was originally written
/// that way, on the reasoning that the guard is unreachable while the bridge
/// carries `None`. That reasoning is wrong here: the same match is also lowered
/// by the DIRECT path, which carried the frame identity before `D8m` and still
/// does, so the refusal arrives whether or not the bridge transports anything.
/// Clause 1b measures exactly that and pins it permanently.
///
/// **The transport-difference evidence lives in two other rows**, both of which
/// red when the bridge is reverted to its complete pre-`D8m` all-`None` tuple:
/// `d8m_the_transported_tuple_is_what_carries_the_source_frame`, which withdraws
/// the tuple and recovers the detached-frame refusal, and the transplant control
/// in `d8m_the_checked_bridge_refuses_every_way_the_transported_identity_can_go_wrong`,
/// which observes the bridge carrying a neighbour's frame on a program where the
/// transported id and the plan's disagree. The keyed pairing relation is in
/// `d8m_two_distinct_occurrences_each_keep_their_own_frame`.
///
/// ## Clause 2 — an unwrapped bridge stays all-None
///
/// The same witness without the marker does **not** reach that refusal. It is
/// structurally identical — same cases, same default, same fingerprint — so a
/// rule that borrowed an identity by shape, by fingerprint, or by "the only
/// frame in the plan" would give it one. None does.
///
/// ## Clause 3 — the descriptor is closed
///
/// A `CheckedSubcontinuationFrame` wrapping anything but a
/// `ComputationalMatch` is **not a bridge at all**: deforestation does not
/// engage, so the composed site is never reached. ⛔ There is no generic
/// wrapper peeling and no fallback — the third descriptor form matches one
/// exact shape, and the enum's exhaustive match at the bridge site makes a
/// fourth form a compile error rather than a silent `None`.
///
/// ## Clause 4 — plan frames and Runtime markers stay in bijection
///
/// A plan naming a frame the source does not mark, and a source marker the plan
/// does not carry, both refuse. ⛔ Preserving an identity through the bridge
/// must not create or consume a frame: the counts are what would move if it did.
///
/// **Promise class: durable invariant.** Refusal identities and a bijection,
/// with the positive clause keyed on which guard is reached rather than on a
/// program that compiles.
#[test]
fn d8m_the_source_frame_identity_survives_the_bridge() {
    // Clause 1 — the missing-slot guard boundary. NOT bridge transport.
    let marked = d8m_witness(7);
    let refusal = format!(
        "{:?}",
        d8m_compile(&marked, 7).expect("the marked witness stops at the IH slot marker")
    );
    assert!(
        refusal.contains("missing its IH slot marker"),
        "this witness must stop at the missing-slot guard rather than at the D8f hard stop. That \
         is a boundary, NOT a transport proof: the same match is also lowered by the direct path, \
         so this refusal does not say the bridge transported anything -- clause 1b measures that \
         directly, and the transport-difference evidence is in \
         d8m_the_transported_tuple_is_what_carries_the_source_frame and the transplant control: \
         {refusal}"
    );
    assert!(
        !refusal.contains("detached from its checked frame"),
        "and specifically not the D8f hard-stop refusal, which is the one D8m exists to retire. \
         Again a boundary claim about which guard this program lands on, not a claim about where \
         the identity reaching that guard came from"
    );

    // Clause 1b — the boundary of clause 1, as a committed measurement rather
    // than a caveat. Withholding the tuple the bridge transports leaves this
    // witness's refusal unchanged, because the same match is also lowered by the
    // direct path and that path carried the identity before D8m. So clause 1 is
    // an assertion about which guard the composed path must not land short of --
    // not a proof that the bridge transports anything.
    crate::cranelift_backend::lowering::core::set_d8m_suppress_transported_tuple(true);
    let withheld = d8m_compile(&marked, 7).map(|error| format!("{error:?}"));
    crate::cranelift_backend::lowering::core::set_d8m_suppress_transported_tuple(false);
    assert!(
        withheld
            .as_deref()
            .is_some_and(|reason| reason.contains("missing its IH slot marker")),
        "MEASURED: this witness refuses identically with the bridge's transported tuple withheld. \
         If that ever stops being true the composed path has become the only route to this guard \
         on this program, clause 1 has become a difference-proof, and this expectation should be \
         inverted rather than deleted: {withheld:?}"
    );

    // Clause 2 — the same shape, unmarked, borrows nothing.
    let plain = d8m_unmarked_witness();
    let plain_refusal = d8m_compile_without_plan(&plain).map(|error| format!("{error:?}"));
    assert!(
        !plain_refusal
            .as_deref()
            .is_some_and(|reason| reason.contains("missing its IH slot marker")),
        "an unwrapped bridge must stay all-None. This witness is structurally identical to the \
         marked one -- same cases, same default, same fingerprint -- so a rule that borrowed an \
         identity by shape, by fingerprint, or by uniqueness in the plan would hand it one: \
         {plain_refusal:?}"
    );

    // Clause 3 — the descriptor is closed: a marker around a non-match is not a
    // bridge, so deforestation never engages and the composed site is unreached.
    let wrapped_nonmatch = d8m_marker_around_nonmatch();
    let outcome = d8m_compile_without_plan(&wrapped_nonmatch).map(|error| format!("{error:?}"));
    assert!(
        !outcome
            .as_deref()
            .is_some_and(|reason| reason.contains("IH slot marker")),
        "a CheckedSubcontinuationFrame wrapping anything but a ComputationalMatch is not a \
         bridge; if this reaches a slot-marker guard, the descriptor is peeling wrappers \
         generically instead of matching one exact shape: {outcome:?}"
    );

    // Clause 4 — the bijection, in both directions.
    for (frame_id, expected) in [
        (8u64, "missing or transplanted"),
        (7, "missing its IH slot marker"),
    ] {
        let refusal = format!(
            "{:?}",
            d8m_compile(&marked, frame_id).expect("both directions refuse or stop at the slot")
        );
        assert!(
            refusal.contains(expected),
            "a plan frame the source does not mark must refuse before anything else; preserving \
             an identity through the bridge must neither create nor consume a frame: {refusal}"
        );
    }
}

#[cfg(test)]
fn d8m_compile_without_plan(expr: &RuntimeExpr) -> Option<CraneliftBackendError> {
    compile_expr_into_module(
        new_object_module("d8m-plain").expect("module"),
        "ken_d8m_plain",
        Linkage::Export,
        expr,
        &NativeSeedEnvironment::empty(),
        BTreeMap::new(),
        None,
        true,
        None,
        Some(test_only_distinguished_root_join_plan()),
        None,
    )
    .err()
}

/// The `D8m` witness with the marker removed: the SAME bridge shape at its own
/// occurrence, carrying no identity to preserve.
#[cfg(test)]
fn d8m_unmarked_witness() -> RuntimeExpr {
    strip_bridge_marker(&d8m_witness(7), false)
}

/// The `D8m` witness with the marker moved onto a non-`ComputationalMatch`,
/// which the closed descriptor must not accept as a bridge.
#[cfg(test)]
fn d8m_marker_around_nonmatch() -> RuntimeExpr {
    strip_bridge_marker(&d8m_witness(7), true)
}

#[cfg(test)]
fn strip_bridge_marker(expr: &RuntimeExpr, wrap_nonmatch: bool) -> RuntimeExpr {
    let RuntimeExpr::Let { value, body } = expr else { panic!("let") };
    let RuntimeExpr::ComputationalMatch {
        scrutinee,
        cases,
        default,
    } = value.as_ref()
    else {
        panic!("eliminator")
    };
    let RuntimeExpr::CheckedSubcontinuationFrame { frame_id, body: bridge } = &cases[0].body else {
        panic!("marked bridge")
    };
    let rebuilt = if wrap_nonmatch {
        // ⛔ The marker survives but now wraps the bridge's own SCRUTINEE, which
        // is a `Var`. Same marker, same id, one shape away from the admissible
        // form -- so this separates "the descriptor matches an exact shape" from
        // "the descriptor peels a wrapper".
        let RuntimeExpr::ComputationalMatch {
            scrutinee: inner,
            cases: inner_cases,
            default: inner_default,
        } = bridge.as_ref()
        else {
            panic!("bridge")
        };
        RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::CheckedSubcontinuationFrame {
                frame_id: *frame_id,
                body: inner.clone(),
            }),
            cases: inner_cases.clone(),
            default: inner_default.clone(),
        }
    } else {
        (**bridge).clone()
    };
    let mut cases = cases.clone();
    cases[0] = crate::RuntimeComputationalMatchCase {
        constructor: cases[0].constructor.clone(),
        argument_binders: cases[0].argument_binders,
        recursive_positions: cases[0].recursive_positions.clone(),
        body: rebuilt,
    };
    RuntimeExpr::Let {
        value: Box::new(RuntimeExpr::ComputationalMatch {
            scrutinee: scrutinee.clone(),
            cases,
            default: default.clone(),
        }),
        body: body.clone(),
    }
}

#[cfg(test)]
const D8N_SYMBOL: &str = "decl:fixture::d8n::witness";

/// The `D8m` checked-bridge witness hosted in a DECLARATION, so its one checked
/// source body is lowered into **two** generated `Function`s: the ordinary
/// declaration body, and the specialization body derived from the same text.
/// The `D8m` bridge witness with IH slot markers added, so the split body gets
/// PAST the slot guard and both generated `Function`s are actually lowered.
/// ⛔ Without them the compile stops inside the first function and neither the
/// duplicate refusal nor its absence is exercised -- the positive would be
/// vacuous, which is the failure mode the release names.
#[cfg(test)]
fn d8n_witness() -> RuntimeExpr {
    add_ih_slot_markers(&d8m_witness(7))
}

#[cfg(test)]
fn add_ih_slot_markers(expr: &RuntimeExpr) -> RuntimeExpr {
    add_ih_slot_markers_with(expr, 200, 20)
}

#[cfg(test)]
fn add_ih_slot_markers_with(
    expr: &RuntimeExpr,
    slot_template_id: u64,
    checked_occurrence_tag: u64,
) -> RuntimeExpr {
    let RuntimeExpr::Let { value, body } = expr else { panic!("let") };
    let RuntimeExpr::ComputationalMatch { scrutinee, cases, default } = value.as_ref() else {
        panic!("eliminator")
    };
    let RuntimeExpr::CheckedSubcontinuationFrame { frame_id, body: bridge } = &cases[0].body else {
        panic!("marked bridge")
    };
    let RuntimeExpr::ComputationalMatch {
        scrutinee: bridge_scrutinee,
        cases: bridge_cases,
        default: bridge_default,
    } = bridge.as_ref()
    else {
        panic!("bridge")
    };
    // ⛔ ONE case, so ONE slot template. The selected field resolves statically
    // to `Ok`, so the `Err` arm is dead here; keeping it would need a second
    // template whose only purpose is to name a constructor this program never
    // reaches, and a template nothing exercises is scaffolding.
    let marked_cases = bridge_cases
        .iter()
        .filter(|case| case.constructor.ends_with("::Ok"))
        .map(|case| crate::RuntimeComputationalMatchCase {
            constructor: case.constructor.clone(),
            argument_binders: case.argument_binders,
            recursive_positions: case.recursive_positions.clone(),
            body: RuntimeExpr::CheckedComputationalIHSlots {
                slot_template_ids: vec![slot_template_id],
                checked_occurrence_paths: vec![vec![checked_occurrence_tag]],
                body: Box::new(case.body.clone()),
            },
        })
        .collect();
    let mut cases = cases.clone();
    cases[0] = crate::RuntimeComputationalMatchCase {
        constructor: cases[0].constructor.clone(),
        argument_binders: cases[0].argument_binders,
        recursive_positions: cases[0].recursive_positions.clone(),
        body: RuntimeExpr::CheckedSubcontinuationFrame {
            frame_id: *frame_id,
            body: Box::new(RuntimeExpr::ComputationalMatch {
                scrutinee: bridge_scrutinee.clone(),
                cases: marked_cases,
                default: bridge_default.clone(),
            }),
        },
    };
    RuntimeExpr::Let {
        value: Box::new(RuntimeExpr::ComputationalMatch {
            scrutinee: scrutinee.clone(),
            cases,
            default: default.clone(),
        }),
        body: body.clone(),
    }
}

#[cfg(test)]
fn d8n_declaration() -> RuntimeDeclaration {
    RuntimeDeclaration {
        symbol: D8N_SYMBOL.to_string(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::Closure {
                captures: Vec::new(),
                params: vec!["state".to_string()],
                body: Box::new(d8n_witness()),
            },
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    }
}

#[cfg(test)]
pub(in crate::cranelift_backend::lowering) fn d8n_compile() -> Option<CraneliftBackendError> {
    let declaration = d8n_declaration();
    let entry = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::DeclarationRef {
            symbol: D8N_SYMBOL.to_string(),
        }),
        args: vec![RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        }],
    };
    let declarations = BTreeMap::from([(D8N_SYMBOL, &declaration)]);
    let RuntimeDeclarationKind::Transparent { body } = &declaration.kind else {
        panic!("transparent")
    };
    let RuntimeExpr::Closure { body, .. } = body else { panic!("closure") };
    let mut plan = d8m_plan(body, 7);
    for frame in &mut plan.frames {
        frame.declaration = D8N_SYMBOL.to_string();
        frame.occurrence_binding_fingerprint =
            crate::compiler_private_oriented_occurrence_binding_fingerprint(frame);
    }
    let mut slot = crate::CheckedComputationalIHSlotTemplateV1 {
        slot_template_id: 200,
        declaration: D8N_SYMBOL.to_string(),
        checked_match_ordinal: 0,
        checked_occurrence_path: vec![20],
        frame_template_id: 7,
        constructor: "ctor:prelude::Result::Ok".to_string(),
        recursive_position: 0,
        method_binder_ordinal: 4,
        local_telescope: Vec::new(),
        ih_interface: oriented_test_interface(1),
        segment_site_id: 9,
        frame_templates: vec![7],
        input_interface: oriented_test_interface(1),
        output_interface: oriented_test_interface(2),
        // ⛔ MEASURED from the witness, never hand-written: a path spelled by
        // hand is a second authority for where the marker is.
        runtime_marker_locations: d8n_slot_locations(),
        occurrence_binding_fingerprint: 0,
    };
    slot.occurrence_binding_fingerprint =
        crate::compiler_private_computational_ih_slot_binding_fingerprint(&slot);
    plan.computational_ih_slots = vec![slot];
    compile_expr_into_module(
        new_object_module("d8n").expect("module"),
        "ken_d8n",
        Linkage::Export,
        &entry,
        &NativeSeedEnvironment::empty(),
        declarations,
        None,
        true,
        None,
        Some(test_only_distinguished_root_join_plan()),
        Some(plan),
    )
    .err()
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8n` — checked-frame consumption is a fact
/// about one emitted `Function`, not about a compile.**
///
/// `consumed_subcontinuation_frames` was a single compile-wide set, so one
/// checked source body lowered into **two** generated `Function`s -- the
/// ordinary declaration body, and the specialization body derived from the same
/// text -- consumed the same `(invocation_id, frame_id)` twice and refused.
/// That was never a double consumption: it is one consumption in each of two
/// functions, which is what a split body means.
///
/// ⛔ The identity key is untouched. No emission-owner, `FuncId` or
/// `PredeclaredFunctionId` salt: salting would make one source frame two
/// identities and quietly permit a real double consumption *inside* one
/// function. What was wrong was the ledger's LIFETIME.
///
/// ## Clause 1 — the split body, which is the whole point
///
/// ⭐⭐ This witness's checked source body **is** lowered into both functions --
/// a template-only single-function green would not exercise the repair at all.
/// It must no longer refuse with "consumed more than once", and it must reach
/// the slot-marker guard, which is only reachable while the bridge carries a
/// frame id (`D8m`).
///
/// ## Clause 2 — restoring the old lifetime brings the refusal back
///
/// The switch shares the set compile-wide again: the exact pre-`D8n` behaviour,
/// not an invented corruption. ⛔ Without this, "it compiles now" and "nothing
/// was ever checked" are indistinguishable.
///
/// ## Clause 3 — branch successors and separate Functions are different
///
/// Branch successors are mutually exclusive paths through ONE function that
/// rejoin, so their consumption **unions** at the join; separate emitted
/// functions never rejoin, so theirs must not. The union behaviour is pinned by
/// the existing `CheckedFrameBranchScope` harness rows and is deliberately not
/// restated here — what this row adds is that the two scopes are distinct
/// objects with opposite merge rules, which clause 2 demonstrates by showing a
/// shared set is wrong at the function boundary.
///
/// **THE GAP:** the boundary-crossing refusals -- a marker active when a body
/// begins or ends -- are unexercised. No lawful program produces either, and
/// forcing one would mean fabricating lowering state rather than perturbing an
/// input. They are fail-closed guards on a population this row cannot
/// instantiate, and I would rather say so than imply coverage.
///
/// **Promise class: durable invariant.**
#[test]
fn d8n_checked_frame_consumption_is_per_function_not_per_compile() {
    use crate::cranelift_backend::lowering::core::set_d8n_compile_wide_lifecycle;

    // Clause 1 — the split body COMPILES.
    crate::cranelift_backend::lowering::reset_d8n_observations();
    let outcome = d8n_compile();
    assert!(
        outcome.is_none(),
        "one checked source body lowered into two generated Functions consumes its frame ONCE IN \
         EACH, so this must compile. A 'consumed more than once' refusal means the ledger is \
         compile-wide again; any other refusal is a new finding on the checked-bridge path and \
         must be reported rather than absorbed: {outcome:?}"
    );

    // Clause 1b — WHAT THE TWO SEAMS ACTUALLY SAW.
    //
    // ⭐⭐ This is what makes "split across two Functions" a measurement rather
    // than a claim about the fixture's shape. Both observations are written at
    // the real seams from state production holds there: the pair the ledger
    // accepted, the slot the plan named, and in each case the defining
    // `FuncId` -- the identity `open_aggregate_events` sets at every body start.
    // ⛔ Nothing here is reconstructed; a reader that rebuilt the identity or
    // the pair would be agreeing with itself.
    //
    // ⚠ PRE-`D8o` HISTORY, retained because it is why this clause keys on the
    // Function rather than the owner: `defining_emission_owner` then reported
    // the SAME value for both consumptions -- only two of the three body kinds
    // set it, so it was stale inside a specialization body -- and this clause
    // would have read as a single-Function witness. `D8o` repaired that; the
    // current invariant is that every body binds its own planner-issued owner.
    // The Function id stays the right key here because it answers "which module
    // definition", which is what "once under each" means.
    let consumptions = crate::cranelift_backend::lowering::d8n_frame_consumptions();
    // ⛔ EVERY record must name a Function before distinctness is asked. A
    // `None` beside a `Some` makes a two-element set just as well as two real
    // Functions do, so counting first would let "one Function and one
    // unattributed body" pass as a split.
    let defining_functions = consumptions
        .iter()
        .map(|(defining_function, _, _)| {
            defining_function.expect(
                "every checked-frame consumption must name the Function it happened in; a `None` \
                 means a body reached the seam without `open_aggregate_events`, and no \
                 distinctness claim over such a record means anything",
            )
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        defining_functions.len(),
        2,
        "the one checked source body must be consumed under TWO distinct defining Functions. One \
         means only a single Function lowered it and this witness is not a split body at all, \
         which would make the whole row vacuous: {consumptions:?}"
    );
    let pairs = consumptions
        .iter()
        .map(|(_, invocation_id, frame_id)| (*invocation_id, *frame_id))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        pairs.len(),
        1,
        "and it must be the SAME (invocation_id, frame_id) pair each time -- that is the whole \
         point. Two pairs would mean the key had been salted per function, which is the repair \
         this checkpoint deliberately did not make: {consumptions:?}"
    );
    assert_eq!(
        consumptions.len(),
        2,
        "once under each Function: not once in total, and not three times: {consumptions:?}"
    );
    let slots = crate::cranelift_backend::lowering::d8n_slot_reconciliations();
    let slot_functions = slots
        .iter()
        .map(|(defining_function, _)| {
            defining_function.expect(
                "every slot reconciliation must name the Function it happened in, for the same \
                 reason the consumptions must",
            )
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        slots.len(),
        2,
        "the plan-named checked-IH slot must reconcile exactly twice -- the slot seam is \
         downstream of the frame seam, so one reconciliation would mean only one Function got \
         far enough to use the frame it consumed: {slots:?}"
    );
    assert_eq!(
        slot_functions, defining_functions,
        "⛔ and across the SAME two Functions, as set equality rather than as two counts that \
         happen to agree. Equal sizes with different members would mean the frame was consumed \
         in one pair of Functions and the slot reconciled in another: {slots:?}"
    );
    let slot_templates = slots
        .iter()
        .map(|(_, slot_template_id)| *slot_template_id)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        slot_templates.len(),
        1,
        "and it must be ONE plan-named slot reconciling once in each Function, not two different \
         slots reconciling once each. ⛔ Asserted as the number of distinct observed ids rather \
         than against the fixture's literal, so the clause cannot be satisfied by agreeing with \
         the number this test happens to write into the plan: {slots:?}"
    );

    // Clause 2 — the old lifetime, restored.
    set_d8n_compile_wide_lifecycle(true);
    let restored = d8n_compile().map(|error| format!("{error:?}"));
    set_d8n_compile_wide_lifecycle(false);
    let restored = restored.expect("the compile-wide lifetime must refuse");
    assert!(
        restored.contains("consumed more than once"),
        "sharing the consumed-frame set compile-wide must reproduce the second-function duplicate \
         refusal. If it does not, this witness is no longer splitting one source body across two \
         Functions and clause 1 is green for a reason it does not name: {restored}"
    );
}

#[cfg(test)]
fn d8n_slot_locations() -> Vec<crate::CheckedRuntimeMarkerLocationV1> {
    let declaration = d8n_declaration();
    let RuntimeDeclarationKind::Transparent { body } = &declaration.kind else {
        panic!("transparent")
    };
    let mut sets = crate::cranelift_backend::planning::CheckedOrientedMarkerSets::default();
    crate::cranelift_backend::planning::collect_checked_oriented_markers(
        body,
        &mut sets,
        D8N_SYMBOL,
        &mut Vec::new(),
    )
    .expect("the witness's markers collect");
    let mut paths = sets
        .computational_ih_slots
        .values()
        .flat_map(|paths| paths.iter().cloned())
        .collect::<Vec<_>>();
    paths.sort();
    paths
        .into_iter()
        .map(|runtime_path| crate::CheckedRuntimeMarkerLocationV1 {
            declaration: D8N_SYMBOL.to_string(),
            runtime_path,
        })
        .collect()
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8o` — every emitted body binds its own
/// planner-issued authority, and none inherits the last body's.**
///
/// The census at `docs/notes/rt-contsrc-d8o-ambient-body-authority-census.md`
/// found that of the three source-bearing body kinds only two wrote the ambient
/// fields: a **specialization body wrote neither**, so it ran on whatever the
/// previously defined body left behind. Four of the eight readers *decline*
/// rather than refuse when the owner is absent, which is why that was silent.
///
/// ## Clause 1 — each body's authority comes from its own pass input
///
/// ⛔⛔ The expected pair is derived **from the plan**, per body kind, and never
/// from the ambient field, the `FuncId`, a raw origin, or a selected composed
/// identity. An expectation read back from the thing under test would agree
/// with it however wrong both were.
///
/// | body kind | expected owner | expected unit |
/// |---|---|---|
/// | ordinary unit | `Predeclared(unit.function)` | `unit.function` |
/// | specialization | `Specialization(unit.id)` | `unit.consumer_owner` |
/// | generated context | `Specialization(context.enclosing)` | `context.raw_owner` |
///
/// ## Clause 2 — nothing inherits
///
/// Every body's INHERITED pair is `None`. ⭐ That is the half that proves the
/// release, and it is mutation-backed: leaving the facts in place at release --
/// the pre-repair behaviour for a specialization body -- makes a later body
/// inherit them.
///
/// **Promise class: durable invariant.** Relations against plan-derived
/// expectations; no literal identities.
#[test]
fn d8o_every_emitted_body_binds_its_own_planner_issued_authority() {
    use crate::cranelift_backend::lowering::{
        d8o_body_authorities, reset_d8o_body_authorities, set_d8o_inherit_residue,
    };

    // ⭐ The `D5a` witness, because it is the one program in reach that emits
    // ALL THREE source-bearing body kinds: ordinary units, specializations, and
    // a generated context. A witness missing a kind would leave that kind's
    // population empty and the clause below vacuous for it.
    reset_d8o_body_authorities();
    crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
        "ken_d8o_authority",
        false,
    )
    .expect("the D5a witness compiles");
    let bound = d8o_body_authorities();

    // Clause 1 — the complete EXACT-BODY-KEY -> (owner, unit) relation.
    //
    // ⛔⛔ Keyed by the body's own planner descriptor identity, supplied by the
    // pass that knows which kind it is. A multiset of lawful pairs plus distinct
    // `FuncId`s -- what this clause asked before -- is insufficient: **swapping
    // two bodies' pairs leaves both multisets identical** and every id distinct,
    // so it would pass. The relation catches it because each pair is attached to
    // the body that bound it.
    //
    // ⛔ The key is never derived from the ambient owner, the `FuncId` alone, a
    // raw origin, or a selected identity.
    let keys = crate::cranelift_backend::lowering::d8o_body_keys()
        .into_iter()
        .map(|(function, key)| {
            (
                function.expect("every body key must be labelled with its Function"),
                key,
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let observed = bound
        .iter()
        .map(|authority| {
            let function = authority
                .function
                .expect("every binding must be labelled with the Function it belongs to");
            let key = *keys
                .get(&function)
                .expect("every bound body must have recorded its exact body key");
            (key, (authority.owner, authority.unit))
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    assert_eq!(
        observed.len(),
        bound.len(),
        "each body must key distinctly, or the relation below is collapsing two bodies into one \
         entry: {bound:?}"
    );
    let expectation = d8o_expected_authorities();
    assert_eq!(
        observed,
        expectation,
        "the complete exact-body-key -> live (owner, unit) relation must equal the planner's own. \
         ⛔ Asserted as a RELATION: a pair attached to the wrong body reds here and would not \
         have reded a multiset comparison"
    );

    // ⛔ And the relation must be SWAP-SENSITIVE, demonstrated rather than
    // argued: exchange two bodies' pairs and the comparison above must fail.
    // The multiset of pairs and the set of `FuncId`s are both unchanged by that
    // exchange, which is precisely why the earlier unkeyed form could not see
    // it.
    let swapped = {
        let mut swapped = expectation.clone();
        let specializations = swapped
            .keys()
            .filter(|key| {
                matches!(
                    key,
                    crate::cranelift_backend::lowering::D8oBodyKey::ContinuationSpecialization(_)
                )
            })
            .copied()
            .collect::<Vec<_>>();
        assert!(
            specializations.len() >= 2,
            "the witness must emit at least two specialization bodies for a swap to be \
             constructible, or this control cannot be built: {expectation:?}"
        );
        let first = swapped[&specializations[0]];
        let second = swapped[&specializations[1]];
        swapped.insert(specializations[0], second);
        swapped.insert(specializations[1], first);
        swapped
    };
    assert_ne!(
        observed, swapped,
        "a pair attached to the WRONG body must be distinguishable. If this matches, the relation \
         is not keyed by body at all and clause 1 has degenerated into the multiset comparison it \
         replaced"
    );

    // ⭐ And all three body kinds must be present, or a kind's clause is vacuous.
    for (kind, present) in [
        (
            "ordinary unit",
            observed.keys().any(|key| {
                matches!(key, crate::cranelift_backend::lowering::D8oBodyKey::OrdinaryUnit(_))
            }),
        ),
        (
            "continuation specialization",
            observed.keys().any(|key| {
                matches!(
                    key,
                    crate::cranelift_backend::lowering::D8oBodyKey::ContinuationSpecialization(_)
                )
            }),
        ),
        (
            "generated context",
            observed.keys().any(|key| {
                matches!(key, crate::cranelift_backend::lowering::D8oBodyKey::GeneratedContext(_))
            }),
        ),
    ] {
        assert!(
            present,
            "the {kind} body population must be non-empty on this witness, or the relation above \
             says nothing about that kind: {observed:?}"
        );
    }

    // Clause 2 — nothing inherits.
    assert!(
        bound
            .iter()
            .all(|authority| authority.inherited_owner.is_none()
                && authority.inherited_unit.is_none()),
        "no emitted body may inherit the previous body's ambient authority; every binding must \
         see an empty enclosing scope: {bound:?}"
    );

    // And the mutation that removes the release brings inheritance back.
    reset_d8o_body_authorities();
    set_d8o_inherit_residue(true);
    let _ = d8n_compile();
    set_d8o_inherit_residue(false);
    let leaked = d8o_body_authorities();
    // ⛔ Discriminated on the LIVE keyed observation, not merely on "something
    // inherited": the inherited owner must be one a PREVIOUS body actually bound
    // in this same run, under a different Function. That is what makes the red
    // attributable to residue crossing a body boundary rather than to any
    // non-`None` value appearing.
    let inherited_from_a_previous_body = leaked.iter().enumerate().any(|(index, authority)| {
        authority.inherited_owner.is_some_and(|inherited| {
            leaked[..index].iter().any(|earlier| {
                earlier.owner == inherited && earlier.function != authority.function
            })
        })
    });
    assert!(
        inherited_from_a_previous_body,
        "leaving the facts in place at release -- the pre-repair behaviour -- must make a later \
         body inherit an owner an EARLIER body under a DIFFERENT Function bound. If nothing does, \
         the release is not what clause 2 is measuring: {leaked:?}"
    );
}

/// The complete **exact body key -> `(owner, unit)`** relation the plan itself
/// names: one entry per executable unit, per continuation unit, and per
/// generated context.
///
/// ⛔ Built from planner views and descriptors only. Nothing here reads the
/// ambient fields, a `FuncId`, a raw origin, or a composed identity.
#[cfg(test)]
fn d8o_expected_authorities() -> std::collections::BTreeMap<
    crate::cranelift_backend::lowering::D8oBodyKey,
    (
        crate::cranelift_backend::planning::ContinuationEmissionOwner,
        PredeclaredFunctionId,
    ),
> {
    use crate::cranelift_backend::lowering::D8oBodyKey;
    use crate::cranelift_backend::planning::ContinuationEmissionOwner;
    let (entry, declarations) =
        crate::cranelift_backend::test_objects::px8tr_nested_post_effect_planning_inputs();
    let declarations = declarations
        .iter()
        .map(|declaration| (declaration.symbol.as_str(), declaration))
        .collect::<BTreeMap<_, _>>();
    let plan = plan_static_transition_graph_with_symbols(
        &entry,
        &declarations,
        &crate::NativeProcessSymbols::legacy_prelude(),
        AbiRootIngress::Value,
        true,
    )
    .expect("the D5a witness plans");
    let mut expected = std::collections::BTreeMap::new();
    // ⛔ `executable_units`, not `emittable_units`: the executable population is
    // what `define_unit_bodies` walks. A template-only unit is declared and
    // never defined, so it emits no body and binds no authority.
    for unit in plan.executable_units().expect("executable units") {
        expected.insert(
            D8oBodyKey::OrdinaryUnit(unit.function()),
            (
                ContinuationEmissionOwner::Predeclared(unit.function()),
                unit.function(),
            ),
        );
    }
    for unit in plan.continuation_units().expect("continuation units") {
        expected.insert(
            D8oBodyKey::ContinuationSpecialization(unit.id()),
            (
                ContinuationEmissionOwner::Specialization(unit.id()),
                unit.consumer_owner(),
            ),
        );
    }
    for context in plan.continuation_contexts().expect("contexts") {
        expected.insert(
            D8oBodyKey::GeneratedContext(context.id()),
            (
                ContinuationEmissionOwner::Specialization(context.enclosing_specialization()),
                context.raw_owner(),
            ),
        );
    }
    expected
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8o` — the bounded re-measurement of the two
/// owner-keyed guards, composed with independent body-authority evidence.**
///
/// ⛔ **This does not reopen `D8a`–`D8k`.** Every prior mechanism and SHA stands;
/// what is recorded here is the same two guards re-run on the `D8o` descendant,
/// beside the evidence they were always missing.
///
/// ## Why they needed composing rather than replacing
///
/// `D8i` clause 2 and `D8j` verification 2 are **self-correlated**. Each
/// mutation selects a foreign identity *relative to* `defining_emission_owner`,
/// and each guard compares back to that same ambient field. So they prove
/// disagreement **relative to ambient state** — not that the ambient state is
/// the planner's. Neither was a false green on its ordinary/root witness, and
/// neither was ever evidence for specialization-body owner correctness.
///
/// ⭐ `D8o` supplies the missing half: the bodies' authority is checked against
/// **plan-derived** expectations by
/// [`d8o_every_emitted_body_binds_its_own_planner_issued_authority`]. Composed,
/// the pair says both "the guard fires on disagreement" and "the thing it
/// agrees with is what the planner issued".
///
/// ## Recorded here
///
/// - `D8i` clause 2 — a real foreign authority at an ordinary binding site
///   still refuses at construction, on the descendant.
/// - `D8j` verification 2 — the wrong-claiming-owner refusal, and the exact run
///   that must NOT refuse, both still hold on the descendant.
/// - **The specialization-body composed-claim population is EMPTY**, recorded
///   rather than fabricated: no lawful witness in reach makes a composed claim
///   from inside a specialization body, so there is nothing to re-measure there
///   and this row says so instead of inventing one.
#[test]
fn d8o_remeasures_the_two_owner_keyed_guards_on_the_descendant() {
    use crate::cranelift_backend::lowering::{
        reset_d8d_bindings, reset_d8o_body_authorities, set_d8i_foreign_authority, D8jMutation,
    };

    // D8i clause 2, re-run.
    reset_d8d_bindings();
    set_d8i_foreign_authority(true);
    let foreign = crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
        "ken_d8o_foreign",
        false,
    );
    set_d8i_foreign_authority(false);
    let refusal = format!(
        "{:?}",
        foreign
            .err()
            .expect("D8i clause 2 must still refuse on the D8o descendant")
    );
    assert!(
        refusal.contains("belongs to a different emitter"),
        "and at its own guard, unchanged by D8o: {refusal}"
    );

    // D8j verification 2, both directions, re-run.
    let (error, discharged, _, _) =
        d8j_root_witness_compile("d8o_owner", D8jMutation::WrongClaimingOwner);
    assert_eq!(
        discharged, 0,
        "D8j verification 2's negative must still leave the relation empty on the descendant"
    );
    assert!(
        format!("{:?}", error.expect("the wrong claiming owner must refuse"))
            .contains("only the emitting owner may answer"),
        "and reach its own refusal"
    );
    let (_error, discharged, _, _) = d8j_root_witness_compile("d8o_exact", D8jMutation::Exact);
    assert_eq!(
        discharged, 1,
        "and D8j verification 2's POSITIVE must still pass: the exact run discharges once. \
         Without this the negative alone would be satisfied by a guard that refuses everything"
    );

    // The specialization-body composed-claim population, recorded as empty.
    //
    // ⛔ `d8i_discharges` records every binding constructed with its facet. A
    // composed facet built inside a specialization body would appear here; none
    // does, on any witness in reach. Recorded, not fabricated.
    reset_d8d_bindings();
    reset_d8o_body_authorities();
    let _ = d8n_compile();
    let claim_bodies = crate::cranelift_backend::lowering::d8o_composed_claim_bodies();
    assert!(
        !claim_bodies.is_empty(),
        "the witness must reach the composed claim seam at least once, or the population question \
         below is not being asked at all"
    );
    // ⛔⛔ Classified BY BODY KIND, joined through the independent
    // Function-to-body-key mapping -- never by the owner variant.
    //
    // ⭐ A generated context carries a `Specialization` OWNER and is **not** a
    // specialization BODY. The previous form filtered on the owner variant and
    // would have counted a context-body claim as a specialization-body one,
    // which is the opposite of what this population is about.
    //
    // ⛔ And never from `identity.emission_owner()`, which is the field the
    // owner guard validates: that would make the question answer itself.
    let keys = crate::cranelift_backend::lowering::d8o_body_keys()
        .into_iter()
        .map(|(function, key)| (function.expect("body keys are labelled"), key))
        .collect::<std::collections::BTreeMap<_, _>>();
    let claim_kinds = claim_bodies
        .iter()
        .map(|(function, _)| {
            let function = function.expect("every composed claim must name its Function");
            *keys
                .get(&function)
                .expect("every claiming body must have recorded its exact body key")
        })
        .collect::<Vec<_>>();
    let from_specialization = claim_kinds
        .iter()
        .filter(|key| {
            matches!(
                key,
                crate::cranelift_backend::lowering::D8oBodyKey::ContinuationSpecialization(_)
            )
        })
        .count();
    assert_eq!(
        from_specialization, 0,
        "MEASURED at the seam and classified by BODY KIND: the specialization-body composed-claim \
         population is EMPTY, so there is nothing to re-measure there. If this ever becomes \
         non-zero that population exists and needs its own owner-correctness evidence: \
         {claim_kinds:?}"
    );
    assert!(
        claim_kinds.iter().all(|key| matches!(
            key,
            crate::cranelift_backend::lowering::D8oBodyKey::OrdinaryUnit(_)
        )),
        "and every claim in reach must come from an ORDINARY unit body, stated positively rather \
         than as the absence of the other two kinds: {claim_kinds:?}"
    );
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8m` — the transported tuple is what carries
/// the source frame through the bridge, proved by withdrawing it.**
///
/// ## The independent side, stated first
///
/// Everything the observed side is compared against comes from the **oriented
/// plan the witness was compiled with** — its frame id, its slot template id —
/// and from `D8o`'s independently supplied **body key**. ⛔ Nothing is read back
/// out of the bridge descriptor, the ambient owner, or the transported tuple
/// itself; those are the mechanism under test.
///
/// ## Clause 1 — the keyed relation, not a bag
///
/// For each exact body that reached the two checked seams, the observed value is
/// `(consumed pair, reconciled slot)`. That relation must equal the one the plan
/// names. ⛔ A bag of pairs plus component distinctness is insufficient and the
/// permutation control below is the demonstration: exchanging two bodies'
/// observations leaves every bag and every component set identical.
///
/// ## Clause 2 — withdrawing the tuple restores the pre-`D8m` refusal
///
/// ⭐⭐ The marker is still entered and consumed, so the plan side is untouched
/// and **only the transport is withheld**. The bridge then carries what it
/// carried before this checkpoint, and `computational_ih_slots_for_case` must
/// refuse with *"detached from its checked frame"* — the `D8f` hard stop. That
/// is the whole claim of `D8m` stated as a difference.
///
/// **Promise class: durable invariant.**
#[test]
fn d8m_the_transported_tuple_is_what_carries_the_source_frame() {
    use crate::cranelift_backend::lowering::{
        d8n_frame_consumptions, d8n_slot_reconciliations, d8o_body_keys, reset_d8n_observations,
        reset_d8o_body_authorities, D8oBodyKey,
    };
    use crate::cranelift_backend::lowering::core::set_d8m_suppress_transported_tuple;

    // Clause 1 — the keyed relation.
    reset_d8n_observations();
    reset_d8o_body_authorities();
    let outcome = d8n_compile();
    assert!(
        outcome.is_none(),
        "the checked-bridge witness must compile, or the seams below are not reached: {outcome:?}"
    );
    let keys = d8o_body_keys()
        .into_iter()
        .map(|(function, key)| (function.expect("body keys are labelled"), key))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut observed: std::collections::BTreeMap<D8oBodyKey, (Vec<(u64, u64)>, Vec<u64>)> =
        std::collections::BTreeMap::new();
    for (function, invocation_id, frame_id) in d8n_frame_consumptions() {
        let function = function.expect("consumptions are labelled");
        let key = *keys.get(&function).expect("a consuming body has a key");
        observed.entry(key).or_default().0.push((invocation_id, frame_id));
    }
    for (function, slot_template_id) in d8n_slot_reconciliations() {
        let function = function.expect("reconciliations are labelled");
        let key = *keys.get(&function).expect("a reconciling body has a key");
        observed.entry(key).or_default().1.push(slot_template_id);
    }
    assert_eq!(
        observed.len(),
        2,
        "exactly two bodies reach the checked seams on this witness -- the ordinary declaration \
         body and the specialization derived from the same source text: {observed:?}"
    );
    // The independent side: the plan's own frame and slot ids.
    let (plan_frame, plan_slot) = d8m_plan_named_ids();
    for (key, (consumed, reconciled)) in &observed {
        assert_eq!(
            consumed,
            &vec![(0u64, plan_frame)],
            "each body must consume the PLAN-NAMED frame exactly once, under key {key:?}"
        );
        assert_eq!(
            reconciled,
            &vec![plan_slot],
            "and reconcile the PLAN-NAMED slot exactly once, under key {key:?}"
        );
    }
    // ⛔ The permanent anti-vacuity permutation. Exchanging two bodies'
    // observations leaves every bag and every component set identical, so an
    // unkeyed comparison stays green; the keyed relation must reject it.
    let permuted = {
        let entries = observed.iter().map(|(k, v)| (*k, v.clone())).collect::<Vec<_>>();
        let mut permuted = observed.clone();
        permuted.insert(entries[0].0, entries[1].1.clone());
        permuted.insert(entries[1].0, entries[0].1.clone());
        permuted
    };
    let bag = |relation: &std::collections::BTreeMap<D8oBodyKey, (Vec<(u64, u64)>, Vec<u64>)>| {
        let mut values = relation.values().cloned().collect::<Vec<_>>();
        values.sort();
        values
    };
    assert_eq!(
        bag(&permuted),
        bag(&observed),
        "the permutation must be invisible to an UNKEYED comparison, or it is not the control it \
         claims to be -- a swap that changes the bag would red anywhere"
    );
    // ⚠ On this witness the two bodies observe identical values, so the keyed
    // relation cannot distinguish the swap either. Recorded as a real limit
    // rather than asserted away: the permutation control is constructible here
    // but not DISCRIMINATING, and it needs a witness whose two bodies reach the
    // seams with different plan-named ids.
    assert_eq!(
        permuted, observed,
        "MEASURED: this witness's two bodies observe the same plan-named frame and slot, so a \
         swap is a no-op and the keyed relation is not discriminated by it here. The witness \
         property that would make it bite is two subjects holding DISTINCT values, and \
         d8m_two_distinct_occurrences_each_keep_their_own_frame is where that exists: there the \
         permutation is enforced rather than recorded. If this ever differs, the permutation has \
         become a real control here too and this expectation should be inverted rather than deleted"
    );

    // Clause 2 — withdrawing the transported tuple.
    set_d8m_suppress_transported_tuple(true);
    let suppressed = d8n_compile();
    set_d8m_suppress_transported_tuple(false);
    let refusal = format!(
        "{:?}",
        suppressed.expect("withholding the transported tuple must refuse")
    );
    assert!(
        refusal.contains("detached from its checked frame"),
        "and it must be the pre-D8m refusal exactly -- the marker is still entered and consumed, \
         so only the transport is withheld and this is D8m's claim stated as a difference: \
         {refusal}"
    );
}

/// The frame and slot ids the witness's own oriented plan names.
///
/// ⛔ The independent side: read from the plan the compile is given, never from
/// the bridge, the ambient owner, or the transported tuple.
#[cfg(test)]
fn d8m_plan_named_ids() -> (u64, u64) {
    let declaration = d8n_declaration();
    let RuntimeDeclarationKind::Transparent { body } = &declaration.kind else {
        panic!("transparent")
    };
    let RuntimeExpr::Closure { body, .. } = body else { panic!("closure") };
    let mut plan = d8m_plan(body, 7);
    for frame in &mut plan.frames {
        frame.declaration = D8N_SYMBOL.to_string();
    }
    (
        plan.frames.first().expect("one planned frame").frame_id,
        plan.computational_ih_slots
            .first()
            .map(|slot| slot.slot_template_id)
            .unwrap_or(200),
    )
}

#[cfg(test)]
const D8M2_SYMBOL: &str = "decl:fixture::d8m2::witness";

/// How one checked composed occurrence of the `D8m` bridge is spelled: its
/// source frame marker id, its checked-IH slot template, the occurrence tag that
/// slot marker carries, and whether the frame marker is present at all.
#[cfg(test)]
#[derive(Clone, Copy, Debug)]
struct D8mOccurrence {
    frame_id: u64,
    slot_template_id: u64,
    occurrence_tag: u64,
    /// Whether the source carries the `CheckedSubcontinuationFrame` marker here.
    marked: bool,
    /// Whether the bridge's case body carries a checked-IH slot marker.
    slot_marked: bool,
}

/// One occurrence, as the eliminator alone -- the `d8n` witness with its hosting
/// `Let` removed, so two of them can sit side by side in ONE scope.
///
/// Siblings in a `Construct`, not nested `Let`s: the composed method binder the
/// bridge case bodies reach for is counted from the deforestation environment,
/// and an enclosing binder would shift it. Two arguments of one constructor are
/// in the same scope as the single `Let`-hosted occurrence these are built from.
#[cfg(test)]
fn d8m_occurrence_eliminator(occurrence: D8mOccurrence) -> RuntimeExpr {
    let hosted = if occurrence.slot_marked {
        add_ih_slot_markers_with(
            &d8m_witness(occurrence.frame_id),
            occurrence.slot_template_id,
            occurrence.occurrence_tag,
        )
    } else {
        d8m_witness(occurrence.frame_id)
    };
    let hosted = if occurrence.marked {
        hosted
    } else {
        // The frame marker is removed and the slot marker inside it is kept:
        // that is what "the source declared no frame HERE" means, and it is the
        // shape the omission control needs.
        strip_bridge_marker(&hosted, false)
    };
    let RuntimeExpr::Let { value, .. } = hosted else {
        panic!("let")
    };
    *value
}

/// **The real second occurrence.** Two structurally identical checked composed
/// bridges, at two distinct source occurrences, carrying distinct frame ids and
/// distinct checked-IH slot templates.
///
/// Their matches are identical, so their frame fingerprints are equal. That is
/// deliberate: it makes the pair invisible to every shape-keyed check and leaves
/// the frame IDENTITY as the only thing that distinguishes them.
#[cfg(test)]
fn d8m_two_occurrence_body(first: D8mOccurrence, second: D8mOccurrence) -> RuntimeExpr {
    RuntimeExpr::Let {
        value: Box::new(RuntimeExpr::Construct {
            constructor: "ctor:fixture::D8M::Pair".to_string(),
            args: vec![
                d8m_occurrence_eliminator(first),
                d8m_occurrence_eliminator(second),
            ],
        }),
        body: Box::new(RuntimeExpr::Var(0)),
    }
}

#[cfg(test)]
fn d8m_two_occurrence_declaration(
    first: D8mOccurrence,
    second: D8mOccurrence,
) -> RuntimeDeclaration {
    RuntimeDeclaration {
        symbol: D8M2_SYMBOL.to_string(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::Closure {
                captures: Vec::new(),
                params: vec!["state".to_string()],
                body: Box::new(d8m_two_occurrence_body(first, second)),
            },
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    }
}

/// Every checked-IH slot marker location in the witness, keyed by the template
/// it names.
///
/// Measured from the witness by the production collector, never spelled by
/// hand: a hand-written path is a second authority for where the marker is.
#[cfg(test)]
fn d8m_two_occurrence_slot_locations(
    first: D8mOccurrence,
    second: D8mOccurrence,
) -> BTreeMap<u64, Vec<crate::CheckedRuntimeMarkerLocationV1>> {
    let declaration = d8m_two_occurrence_declaration(first, second);
    let RuntimeDeclarationKind::Transparent { body } = &declaration.kind else {
        panic!("transparent")
    };
    let mut sets = crate::cranelift_backend::planning::CheckedOrientedMarkerSets::default();
    crate::cranelift_backend::planning::collect_checked_oriented_markers(
        body,
        &mut sets,
        D8M2_SYMBOL,
        &mut Vec::new(),
    )
    .expect("the witness's markers collect");
    let mut located: BTreeMap<u64, Vec<crate::CheckedRuntimeMarkerLocationV1>> = BTreeMap::new();
    for ((slot_template_id, _), paths) in &sets.computational_ih_slots {
        let mut paths = paths.iter().cloned().collect::<Vec<_>>();
        paths.sort();
        located.entry(*slot_template_id).or_default().extend(
            paths
                .into_iter()
                .map(|runtime_path| crate::CheckedRuntimeMarkerLocationV1 {
                    declaration: D8M2_SYMBOL.to_string(),
                    runtime_path,
                }),
        );
    }
    located
}

/// The oriented plan for the two-occurrence witness: one frame per MARKED
/// occurrence, one slot template per occurrence.
///
/// `slot_frames` overrides which planned frame each slot template binds to; it
/// is what the omission control needs, and passing `None` means "each slot binds
/// to its own occurrence's frame".
#[cfg(test)]
fn d8m_two_occurrence_plan(
    first: D8mOccurrence,
    second: D8mOccurrence,
    slot_frames: Option<(u64, u64)>,
) -> crate::OrientedSubcontinuationPlanV1 {
    let declaration = d8m_two_occurrence_declaration(first, second);
    let RuntimeDeclarationKind::Transparent { body } = &declaration.kind else {
        panic!("transparent")
    };
    let RuntimeExpr::Closure { body, .. } = body else {
        panic!("closure")
    };
    let RuntimeExpr::Let { value, .. } = body.as_ref() else {
        panic!("let")
    };
    let RuntimeExpr::Construct { args, .. } = value.as_ref() else {
        panic!("pair")
    };
    let located = d8m_two_occurrence_slot_locations(first, second);
    let mut frames = Vec::new();
    let mut slots = Vec::new();
    let slot_frames = slot_frames.unwrap_or((first.frame_id, second.frame_id));
    for (position, (occurrence, slot_frame)) in [(first, slot_frames.0), (second, slot_frames.1)]
        .into_iter()
        .enumerate()
    {
        let RuntimeExpr::ComputationalMatch { cases, .. } = &args[position] else {
            panic!("eliminator")
        };
        // The bridge match, reached through the frame marker when there is one.
        let bridge = match &cases[0].body {
            RuntimeExpr::CheckedSubcontinuationFrame { frame_id, body } => {
                assert_eq!(*frame_id, occurrence.frame_id, "marker id is as spelled");
                body.as_ref()
            }
            other => other,
        };
        let RuntimeExpr::ComputationalMatch { cases, default, .. } = bridge else {
            panic!("bridge")
        };
        if occurrence.marked {
            let mut frame = crate::OrientedSubcontinuationFramePlanV1 {
                frame_id: occurrence.frame_id,
                // Its own prompt region: a segment admits exactly one
                // distinguished root, and these two occurrences are independent
                // roots rather than one nested inside the other.
                segment_site_id: 9 + position as u64,
                declaration: D8M2_SYMBOL.to_string(),
                checked_occurrence_path: vec![occurrence.frame_id],
                semantic_position: position as u64,
                input_interface: oriented_test_interface(1),
                output_interface: oriented_test_interface(2),
                runtime_frame_fingerprint:
                    crate::compiler_private_computational_match_frame_fingerprint(cases, default),
                occurrence_binding_fingerprint: 0,
                control_witness: crate::OrientedControlWitnessV1::DistinguishedRoot,
            };
            frame.occurrence_binding_fingerprint =
                crate::compiler_private_oriented_occurrence_binding_fingerprint(&frame);
            frames.push(frame);
        }
        if !occurrence.slot_marked {
            continue;
        }
        let mut slot = crate::CheckedComputationalIHSlotTemplateV1 {
            slot_template_id: occurrence.slot_template_id,
            declaration: D8M2_SYMBOL.to_string(),
            checked_match_ordinal: position as u64,
            checked_occurrence_path: vec![occurrence.occurrence_tag],
            frame_template_id: slot_frame,
            constructor: "ctor:prelude::Result::Ok".to_string(),
            recursive_position: 0,
            method_binder_ordinal: 4,
            local_telescope: Vec::new(),
            ih_interface: oriented_test_interface(1),
            // The segment of the frame this slot binds to, not of the occurrence
            // it sits in: a slot that names another occurrence's frame is IN
            // that frame's prompt region, and spelling its own would refuse as a
            // segment crossing before the bridge is ever reached.
            segment_site_id: frames
                .iter()
                .find(|frame| frame.frame_id == slot_frame)
                .map_or(9 + position as u64, |frame| frame.segment_site_id),
            frame_templates: vec![slot_frame],
            input_interface: oriented_test_interface(1),
            output_interface: oriented_test_interface(2),
            runtime_marker_locations: located
                .get(&occurrence.slot_template_id)
                .cloned()
                .expect("every occurrence's slot marker was located"),
            occurrence_binding_fingerprint: 0,
        };
        slot.occurrence_binding_fingerprint =
            crate::compiler_private_computational_ih_slot_binding_fingerprint(&slot);
        slots.push(slot);
    }
    crate::OrientedSubcontinuationPlanV1 {
        representation_rule_version:
            crate::OrientedSubcontinuationPlanV1::REPRESENTATION_RULE_VERSION,
        frames,
        recursive_calls: Vec::new(),
        computational_ih_slots: slots,
        computational_ih_calls: Vec::new(),
    }
}

/// Compile the two-occurrence witness.
///
/// `source` spells the two occurrences as the SOURCE carries them; `planned`
/// spells the two the PLAN was built for. Passing different values is how a
/// control perturbs one side while the other stays lawful.
#[cfg(test)]
fn d8m_two_occurrence_compile(
    source: (D8mOccurrence, D8mOccurrence),
    planned: (D8mOccurrence, D8mOccurrence),
    slot_frames: Option<(u64, u64)>,
) -> Option<CraneliftBackendError> {
    d8m_two_occurrence_compile_with(source, planned, slot_frames, |_| {}, |body| body.clone())
}

/// As above, with two escape hatches for the controls: `adjust` perturbs the
/// PLAN after it is built lawfully, and `rewrite` perturbs the SOURCE after the
/// plan has been derived from the unperturbed text.
#[cfg(test)]
fn d8m_two_occurrence_compile_with(
    source: (D8mOccurrence, D8mOccurrence),
    planned: (D8mOccurrence, D8mOccurrence),
    slot_frames: Option<(u64, u64)>,
    adjust: impl FnOnce(&mut crate::OrientedSubcontinuationPlanV1),
    rewrite: impl FnOnce(&RuntimeExpr) -> RuntimeExpr,
) -> Option<CraneliftBackendError> {
    let lawful = d8m_two_occurrence_declaration(source.0, source.1);
    let RuntimeDeclarationKind::Transparent { body } = &lawful.kind else {
        panic!("transparent")
    };
    let declaration = RuntimeDeclaration {
        symbol: D8M2_SYMBOL.to_string(),
        kind: RuntimeDeclarationKind::Transparent {
            body: rewrite(body),
        },
        metadata: lawful.metadata.clone(),
    };
    let entry = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::DeclarationRef {
            symbol: D8M2_SYMBOL.to_string(),
        }),
        args: vec![RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        }],
    };
    let declarations = BTreeMap::from([(D8M2_SYMBOL, &declaration)]);
    let mut plan = d8m_two_occurrence_plan(planned.0, planned.1, slot_frames);
    adjust(&mut plan);
    compile_expr_into_module(
        new_object_module("d8m2").expect("module"),
        "ken_d8m2",
        Linkage::Export,
        &entry,
        &NativeSeedEnvironment::empty(),
        declarations,
        None,
        true,
        None,
        Some(test_only_distinguished_root_join_plan()),
        Some(plan),
    )
    .err()
}

/// Rewrite the default trap message of the SECOND occurrence's bridge match.
///
/// The frame fingerprint is computed over the match's cases and default, so this
/// changes that occurrence's fingerprint and nothing else about the program's
/// shape -- and it is applied to the source only, after the plan has been
/// derived from the unperturbed text.
#[cfg(test)]
fn d8m_reshape_second_bridge(body: &RuntimeExpr) -> RuntimeExpr {
    let RuntimeExpr::Closure { captures, params, body } = body else {
        panic!("closure")
    };
    let RuntimeExpr::Let { value, body: rest } = body.as_ref() else {
        panic!("let")
    };
    let RuntimeExpr::Construct { constructor, args } = value.as_ref() else {
        panic!("pair")
    };
    let RuntimeExpr::ComputationalMatch { scrutinee, cases, default } = &args[1] else {
        panic!("eliminator")
    };
    let RuntimeExpr::CheckedSubcontinuationFrame { frame_id, body: bridge } = &cases[0].body else {
        panic!("marked bridge")
    };
    let RuntimeExpr::ComputationalMatch {
        scrutinee: bridge_scrutinee,
        cases: bridge_cases,
        default: bridge_default,
    } = bridge.as_ref()
    else {
        panic!("bridge")
    };
    let mut cases = cases.clone();
    cases[0] = crate::RuntimeComputationalMatchCase {
        constructor: cases[0].constructor.clone(),
        argument_binders: cases[0].argument_binders,
        recursive_positions: cases[0].recursive_positions.clone(),
        body: RuntimeExpr::CheckedSubcontinuationFrame {
            frame_id: *frame_id,
            body: Box::new(RuntimeExpr::ComputationalMatch {
                scrutinee: bridge_scrutinee.clone(),
                cases: bridge_cases.clone(),
                default: RuntimeTrap {
                    code: bridge_default.code.clone(),
                    message: format!("{} reshaped", bridge_default.message),
                },
            }),
        },
    };
    let mut args = args.clone();
    args[1] = RuntimeExpr::ComputationalMatch {
        scrutinee: scrutinee.clone(),
        cases,
        default: default.clone(),
    };
    RuntimeExpr::Closure {
        captures: captures.clone(),
        params: params.clone(),
        body: Box::new(RuntimeExpr::Let {
            value: Box::new(RuntimeExpr::Construct {
                constructor: constructor.clone(),
                args,
            }),
            body: rest.clone(),
        }),
    }
}

/// The two occurrences the lawful witness carries.
#[cfg(test)]
fn d8m_lawful_occurrences() -> (D8mOccurrence, D8mOccurrence) {
    (
        D8mOccurrence {
            frame_id: 7,
            slot_template_id: 200,
            occurrence_tag: 20,
            marked: true,
            slot_marked: true,
        },
        D8mOccurrence {
            frame_id: 8,
            slot_template_id: 201,
            occurrence_tag: 21,
            marked: true,
            slot_marked: true,
        },
    )
}

/// The `(frame, slot)` pairs the PLAN names, keyed by frame.
///
/// The independent side of the pairing law: each planned checked-IH slot names
/// the frame template it binds to, and that binding is the plan's own. Nothing
/// here is read back out of the bridge, the transported tuple, or an
/// observation.
#[cfg(test)]
fn d8m_planned_frame_slots(
    first: D8mOccurrence,
    second: D8mOccurrence,
) -> BTreeMap<u64, BTreeSet<u64>> {
    let plan = d8m_two_occurrence_plan(first, second, None);
    let mut planned: BTreeMap<u64, BTreeSet<u64>> = BTreeMap::new();
    for slot in &plan.computational_ih_slots {
        planned
            .entry(slot.frame_template_id)
            .or_default()
            .insert(slot.slot_template_id);
    }
    planned
}

/// What the checked seams observed, keyed by the frame the bridge transported.
#[cfg(test)]
fn d8m_observed_frame_slots() -> BTreeMap<u64, BTreeSet<u64>> {
    let mut observed: BTreeMap<u64, BTreeSet<u64>> = BTreeMap::new();
    for (_, frame_id, slot_template_id) in crate::cranelift_backend::lowering::d8m_slot_frame_pairs()
    {
        observed
            .entry(frame_id)
            .or_default()
            .insert(slot_template_id);
    }
    observed
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8m` — two same-shaped occurrences each keep
/// their OWN frame, and exchanging them is rejected.**
///
/// ## Why a second occurrence, and why this one
///
/// The `D8n` witness has one checked occurrence, so its frame and its slot are
/// both singletons and every pairing of them is the same pairing. The
/// anti-vacuity permutation the governing rule asks for is constructible there
/// and provably not discriminating: the two bodies observe the same plan-named
/// ids, so exchanging their observations is a no-op. That limit is recorded at
/// `d8m_the_transported_tuple_is_what_carries_the_source_frame`.
///
/// This witness supplies the missing axis. Two structurally identical checked
/// composed bridges sit side by side in one scope, carrying distinct frame ids
/// and distinct slot templates. Their matches are identical, so their frame
/// fingerprints are EQUAL -- which is deliberate: it makes the pair invisible to
/// every shape-keyed check and leaves the transported identity as the only thing
/// that tells them apart.
///
/// ## The independent side
///
/// `frame -> slot` as the PLAN names it, through each slot template's
/// `frame_template_id`. Derived from the plan the compile is handed, never from
/// the bridge descriptor, the ambient owner, or the transported tuple.
///
/// ## Clause 1 — the keyed pairing relation
///
/// The relation observed at the real slot seam must equal the planned one.
/// MEASURED limit: on the LAWFUL witness the transported frame and the slot's
/// own planned frame necessarily agree, so recording either one satisfies this
/// clause. It establishes the population -- which pairs were reached and how
/// often -- and the transplant control in
/// `d8m_the_checked_bridge_refuses_every_way_the_transported_identity_can_go_wrong`
/// is what shows the observation is of the frame the BRIDGE carried: it asserts
/// the disagreement on a program where the two differ.
///
/// ## Clause 2 — the permutation, and here it BITES
///
/// Exchanging the two frames' observed slots leaves the bag of values identical,
/// so an unkeyed comparison stays green. The keyed relation must reject it, and
/// that rejection is measured, not argued.
///
/// ## Clause 3 — the body-keyed relation, with its limit stated
///
/// Keyed on `D8o`'s independently supplied body key: the ordinary unit body
/// lowers the whole declaration and must observe BOTH planned pairs; each
/// specialization body must observe exactly one, and together they must cover
/// the plan. MEASURED limit: the independent side does not say WHICH
/// specialization belongs to which occurrence, so a swap between the two
/// specialization bodies is invisible to this clause. Clause 2 is where the
/// discrimination lives, and it is keyed on the frame, whose independent side is
/// complete.
///
/// **Promise class: durable invariant.** Two keyed relations against a
/// plan-derived expectation, plus an enforced permutation.
#[test]
fn d8m_two_distinct_occurrences_each_keep_their_own_frame() {
    use crate::cranelift_backend::lowering::{reset_d8n_observations, D8oBodyKey};

    let (first, second) = d8m_lawful_occurrences();
    reset_d8n_observations();
    crate::cranelift_backend::lowering::reset_d8o_body_authorities();
    let outcome = d8m_two_occurrence_compile((first, second), (first, second), None);
    assert!(
        outcome.is_none(),
        "two same-shaped checked composed occurrences in one scope must compile; the whole matrix \
         below is about what happens when one of them is perturbed, and none of it means anything \
         if the lawful program does not build: {outcome:?}"
    );

    // Clause 1 — the keyed pairing relation.
    let planned = d8m_planned_frame_slots(first, second);
    let observed = d8m_observed_frame_slots();
    assert_eq!(
        planned.len(),
        2,
        "the witness must plan TWO distinct frames, or there is no second subject and the \
         permutation below is the same no-op it was on the one-occurrence witness: {planned:?}"
    );
    assert_eq!(
        observed, planned,
        "each occurrence's bridge must transport ITS OWN frame to ITS OWN slot. The independent \
         side is the plan's `frame_template_id` per slot template; the observed side is the pair \
         the slot seam actually reconciled: {observed:?} vs {planned:?}"
    );

    // Clause 2 — the permutation, and it bites here.
    let permuted = {
        let frames = planned.keys().copied().collect::<Vec<_>>();
        let mut permuted = observed.clone();
        permuted.insert(frames[0], observed[&frames[1]].clone());
        permuted.insert(frames[1], observed[&frames[0]].clone());
        permuted
    };
    let bag = |relation: &BTreeMap<u64, BTreeSet<u64>>| {
        let mut values = relation.values().cloned().collect::<Vec<_>>();
        values.sort();
        values
    };
    assert_eq!(
        bag(&permuted),
        bag(&observed),
        "the permutation must be INVISIBLE to an unkeyed comparison, or it is not the control it \
         claims to be: a swap that changes the bag would red anywhere"
    );
    assert_ne!(
        permuted, planned,
        "and VISIBLE to the keyed one. This is the assertion the one-occurrence witness could not \
         make: there the two subjects held equal values and the swap was a no-op. Here they hold \
         distinct values, so exchanging which frame reconciled which slot is a real difference and \
         the plan-derived expectation must reject it"
    );

    // Clause 3 — the body-keyed relation, keyed on D8o's supplied body key.
    let keys = crate::cranelift_backend::lowering::d8o_body_keys()
        .into_iter()
        .map(|(function, key)| (function.expect("body keys are labelled"), key))
        .collect::<BTreeMap<_, _>>();
    let mut by_body: BTreeMap<D8oBodyKey, BTreeSet<(u64, u64)>> = BTreeMap::new();
    for (function, frame_id, slot_template_id) in
        crate::cranelift_backend::lowering::d8m_slot_frame_pairs()
    {
        let function = function.expect("every reconciliation names its Function");
        let key = *keys
            .get(&function)
            .expect("every reconciling body recorded its exact body key");
        by_body
            .entry(key)
            .or_default()
            .insert((frame_id, slot_template_id));
    }
    let planned_pairs = planned
        .iter()
        .flat_map(|(frame, slots)| slots.iter().map(move |slot| (*frame, *slot)))
        .collect::<BTreeSet<_>>();
    let ordinary = by_body
        .iter()
        .filter(|(key, _)| matches!(key, D8oBodyKey::OrdinaryUnit(_)))
        .map(|(_, pairs)| pairs.clone())
        .collect::<Vec<_>>();
    let specializations = by_body
        .iter()
        .filter(|(key, _)| matches!(key, D8oBodyKey::ContinuationSpecialization(_)))
        .map(|(_, pairs)| pairs.clone())
        .collect::<Vec<_>>();
    assert_eq!(
        ordinary,
        vec![planned_pairs.clone()],
        "exactly one ordinary unit body lowers this declaration, and it contains BOTH occurrences, \
         so it must observe both planned pairs: {by_body:?}"
    );
    assert_eq!(
        specializations.len(),
        2,
        "and each occurrence must derive its own specialization body. One would mean the two \
         same-shaped occurrences had collapsed into a single specialization, which is exactly the \
         confusion this witness exists to detect: {by_body:?}"
    );
    assert_eq!(
        specializations.iter().map(|pairs| pairs.len()).collect::<Vec<_>>(),
        vec![1, 1],
        "each specialization body reaches exactly ONE checked occurrence: {by_body:?}"
    );
    assert_eq!(
        specializations.iter().flatten().copied().collect::<BTreeSet<_>>(),
        planned_pairs,
        "and between them they cover the plan exactly -- the same pair twice would mean one \
         occurrence was specialized and the other silently dropped: {by_body:?}"
    );
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8m` — every way the transported identity can
/// go wrong, and which plane catches it.**
///
/// Each control below carries a LAWFUL oriented plan and is stated with the
/// plane that refuses it, because "it refuses" and "it refuses at the bridge"
/// are different findings and only the second is about `D8m`. Two of the five
/// hazards are caught earlier, by planning, and saying so is the point rather
/// than an omission: a control that reds in plan validation proves nothing about
/// the consumption the bridge performs.
///
/// ## Reaching the bridge at all
///
/// The two occurrences are structurally identical, so their frame fingerprints
/// are equal, so exchanging their marker ids is invisible to the collector
/// planning compares against. That is what makes a transplant reach lowering
/// here and nowhere else.
///
/// ## The five, as measured
///
/// - **Transplant** — the two occurrences exchange frame ids. Planning is blind
///   to it; the bridge transports occurrence one's neighbour's frame, and the
///   slot binding law refuses. AT THE BRIDGE.
/// - **Omission** — occurrence two loses its frame marker while keeping its slot
///   marker, and the plan binds that slot to the surviving frame. The bridge is
///   all-None there and the slot marker has nothing to attach to: the pre-`D8m`
///   refusal, reached on a program whose plan is lawful. AT THE BRIDGE.
/// - **Fingerprint** — the source's second bridge is reshaped after the plan is
///   derived. Caught by PLANNING. See the residual below.
/// - **Duplicate consumption** — two markers with one id are caught by PLANNING;
///   the lowering-plane law is reached by restoring the pre-`D8n` compile-wide
///   ledger, where one source occurrence lowered into two Functions consumes one
///   pair twice. AT THE BRIDGE.
/// - **Wrapper-origin substitution** — the bridge is given the marker's own
///   occurrence instead of the wrapped match's, which is child 0 of it. Refused.
///
/// ## MEASURED / CLAIMED / THE GAP, on the fingerprint hazard
///
/// **MEASURED:** reshaping the source after deriving the plan refuses with
/// *"checked plan frame fingerprint is stale"*, in planning. Arming the bridge
/// to consume with a shape the source match does not carry refuses with
/// *"checked Runtime marker no longer denotes its planned frame"*, at the
/// consumption seam.
///
/// **CLAIMED:** the consumption-seam check holds the bridge to the match its own
/// marker wrapped.
///
/// **THE GAP:** no source or plan input reaches that check. Planning collects
/// each marker's fingerprint with
/// `compiler_private_computational_match_frame_fingerprint` over the same cases
/// and default the bridge later borrows, and requires agreement with the plan
/// before lowering starts, so the two computations cannot disagree on any input.
/// The attempted evasion is committed above rather than described: it reds in
/// planning. The seam check is therefore a guard against the mechanism drifting
/// -- a future bridge consuming with its own deforested cases rather than the
/// source's -- and the armed control is the only thing that exercises it.
///
/// **Promise class: durable invariant.** Named refusals under existing laws,
/// each labelled with the plane that produced it.
#[test]
fn d8m_the_checked_bridge_refuses_every_way_the_transported_identity_can_go_wrong() {
    use crate::cranelift_backend::lowering::core::{
        set_d8m_foreign_consumed_shape, set_d8m_wrapper_origin_substitution,
        set_d8n_compile_wide_lifecycle,
    };

    let (first, second) = d8m_lawful_occurrences();
    let refusal = |outcome: Option<CraneliftBackendError>, what: &str| -> String {
        format!("{:?}", outcome.unwrap_or_else(|| panic!("{what} must refuse")))
    };

    // Transplant — the two occurrences exchange frame ids, plan untouched.
    crate::cranelift_backend::lowering::reset_d8n_observations();
    let transplanted = refusal(
        d8m_two_occurrence_compile(
            (
                D8mOccurrence { frame_id: second.frame_id, ..first },
                D8mOccurrence { frame_id: first.frame_id, ..second },
            ),
            (first, second),
            None,
        ),
        "a transplanted marker",
    );
    assert!(
        transplanted.contains("computational IH slot constructor/position/frame binding is stale"),
        "a marker that carries its NEIGHBOUR's frame must be caught where the transported identity \
         is used, not by shape: the two occurrences are structurally identical, so planning \
         compares equal fingerprints and admits the exchange. If this refuses with a planning \
         message the control has stopped reaching the bridge and proves nothing about D8m: \
         {transplanted}"
    );
    // And WHAT the seam saw, which is the disagreement itself rather than only
    // the refusal it caused. The pair is recorded before the binding law runs,
    // so the two components are free to differ here; the plan pairs the first
    // occurrence's slot with the first occurrence's frame, and the transplanted
    // bridge brought its neighbour's instead.
    let seen = crate::cranelift_backend::lowering::d8m_slot_frame_pairs()
        .into_iter()
        .map(|(_, frame_id, slot_template_id)| (frame_id, slot_template_id))
        .collect::<BTreeSet<_>>();
    assert!(
        seen.contains(&(second.frame_id, first.slot_template_id)),
        "the transplanted bridge must be OBSERVED carrying its neighbour's frame to the first \
         occurrence's slot. If the seam only ever records agreeing pairs then the observation is \
         downstream of the law it is meant to witness and says nothing: {seen:?}"
    );
    assert!(
        !seen.contains(&(first.frame_id, first.slot_template_id)),
        "and it must not ALSO be observed carrying the right one -- that would mean the exchange \
         never reached this occurrence and something else refused: {seen:?}"
    );

    // Omission — occurrence two loses its frame marker; its slot binds to the
    // surviving frame, so the plan stays lawful and the program still reaches
    // the bridge.
    let omitted = refusal(
        d8m_two_occurrence_compile(
            (first, D8mOccurrence { marked: false, ..second }),
            (first, D8mOccurrence { marked: false, ..second }),
            Some((first.frame_id, first.frame_id)),
        ),
        "an omitted frame marker",
    );
    assert!(
        omitted.contains("computational IH slot marker is detached from its checked frame"),
        "an unwrapped bridge carries no identity, so a checked IH slot inside it has nothing to \
         attach to. This is the exact pre-D8m refusal, and reaching it on a program whose plan is \
         lawful is what makes the omission control live rather than a plan-validation red: \
         {omitted}"
    );

    // Fingerprint — the attempted evasion, which planning catches.
    let reshaped = refusal(
        d8m_two_occurrence_compile_with(
            (first, second),
            (first, second),
            None,
            |_| {},
            d8m_reshape_second_bridge,
        ),
        "a reshaped bridge",
    );
    assert!(
        reshaped.contains("checked plan frame fingerprint is stale"),
        "MEASURED, and the reason the consumption-seam fingerprint check has no input that \
         reaches it: planning computes each marker's fingerprint over the same cases and default \
         the bridge later borrows, and requires agreement before lowering starts: {reshaped}"
    );

    // Fingerprint — the seam law itself, reached by arming the bridge to consume
    // with a shape the source match does not carry.
    set_d8m_foreign_consumed_shape(true);
    let foreign = d8m_two_occurrence_compile((first, second), (first, second), None);
    set_d8m_foreign_consumed_shape(false);
    let foreign = refusal(foreign, "a foreign consumed shape");
    assert!(
        foreign.contains("checked Runtime marker no longer denotes its planned frame"),
        "the bridge consumes through the existing pair, and that pair holds the marker to the \
         shape the plan transported for it. Same marker, same cases, one field of the default \
         changed: {foreign}"
    );

    // Duplicate consumption — the source-level form, caught by planning.
    let source_duplicate = refusal(
        d8m_two_occurrence_compile(
            (first, D8mOccurrence { frame_id: first.frame_id, ..second }),
            (first, second),
            None,
        ),
        "two markers with one frame id",
    );
    assert!(
        source_duplicate.contains("Runtime IR repeats a checked subcontinuation frame marker"),
        "two source occurrences cannot name one frame, and that is settled before lowering: \
         {source_duplicate}"
    );

    // Duplicate consumption — the lowering-plane law, reached by restoring the
    // pre-D8n compile-wide ledger lifetime.
    set_d8n_compile_wide_lifecycle(true);
    let wide = d8m_two_occurrence_compile((first, second), (first, second), None);
    set_d8n_compile_wide_lifecycle(false);
    let wide = refusal(wide, "a compile-wide consumed-frame ledger");
    assert!(
        wide.contains("checked Runtime frame marker was consumed more than once"),
        "one source occurrence lowered into two Functions consumes its pair once in each, so a \
         ledger shared across the compile sees a duplicate. That is the affine law firing at the \
         consumption seam, on a lawful program: {wide}"
    );

    // Wrapper-origin substitution.
    set_d8m_wrapper_origin_substitution(true);
    let substituted = d8m_two_occurrence_compile((first, second), (first, second), None);
    set_d8m_wrapper_origin_substitution(false);
    let substituted = refusal(substituted, "a wrapper-origin substitution");
    assert!(
        substituted.contains("source match population was requested for a different occurrence kind"),
        "the marker names the frame; the match IS the frame. Giving the bridge the wrapper's own \
         occurrence instead of child 0 of it must refuse rather than silently key every downstream \
         origin lookup on a node that is not a match: {substituted}"
    );
    // MEASURED, and stated because it bounds what the second occurrence buys:
    // this substitution also refuses on the one-occurrence D8n witness. The
    // planner's occurrence-kind law catches it regardless of how many candidate
    // occurrences exist, so the distinct second occurrence is not what makes
    // this control bite. It is what makes the transplant control above bite.
    set_d8m_wrapper_origin_substitution(true);
    let single = d8n_compile();
    set_d8m_wrapper_origin_substitution(false);
    assert!(
        format!("{single:?}").contains("source match population was requested for a different occurrence kind"),
        "the one-occurrence witness must refuse the same substitution the same way; if it ever \
         stops doing so, the substitution has become occurrence-sensitive and the claim above \
         about what the second occurrence buys needs restating: {single:?}"
    );
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8m` — the bridge arm populations move only
/// with the marker, and the frame counts stay in bijection.**
///
/// ## Clause 1 — the arms partition, and only the marker moves a site
///
/// The closed descriptor has three arms and they are disjoint by source
/// constructor. On the lawful witness both composed sites take the checked arm.
/// Unwrapping ONE occurrence moves exactly one site to the unwrapped arm and
/// leaves the total unchanged.
///
/// The ordinary arm's population is empty in both, which is a description of
/// THIS program and not a proof about the ordinary bridge: an arm nothing
/// reaches is an arm nothing tests. The positive is
/// `d8m_the_ordinary_bridge_arm_is_reached_and_untouched`, which reaches the arm
/// with an ordinary `Match` bridge, classifies it from the source descriptor,
/// and holds its classification, count and outcome equal across all three of
/// `D8m`'s test-only perturbations.
///
/// ## Clause 2 — an unwrapped bridge contributes nothing checked
///
/// With the second occurrence's frame AND slot markers both removed, the checked
/// population is exactly the first occurrence's pair. An unwrapped bridge stays
/// all-None: it does not borrow the surviving frame, and there is only one to
/// borrow.
///
/// ## Clause 3 — plan frames and Runtime markers stay in bijection
///
/// Both directions, on a witness that has two of each: a plan naming a frame the
/// source does not mark, and a source marker the plan does not name. Preserving
/// an identity through the bridge must neither create nor consume a frame.
///
/// **Promise class: durable invariant.** A partition, an empty population
/// established at the seam, and a bijection.
#[test]
fn d8m_the_bridge_arm_populations_move_only_with_the_marker() {
    use crate::cranelift_backend::lowering::{
        d8m_bridge_arms, d8m_slot_frame_pairs, reset_d8n_observations, D8mBridgeArm,
    };

    let (first, second) = d8m_lawful_occurrences();
    let arms = |occurrences: (D8mOccurrence, D8mOccurrence)| {
        reset_d8n_observations();
        let outcome = d8m_two_occurrence_compile(occurrences, occurrences, None);
        assert!(outcome.is_none(), "the witness must compile: {outcome:?}");
        let mut counted: BTreeMap<D8mBridgeArm, usize> = BTreeMap::new();
        for (_, arm) in d8m_bridge_arms() {
            *counted.entry(arm).or_default() += 1;
        }
        (counted, d8m_slot_frame_pairs())
    };

    // Clause 1 — the partition.
    let (checked_arms, _) = arms((first, second));
    assert_eq!(
        checked_arms,
        BTreeMap::from([(D8mBridgeArm::CheckedComputational, 2)]),
        "both composed sites take the checked arm on the lawful witness, and no site takes the \
         ordinary one: {checked_arms:?}"
    );
    let plain_second = D8mOccurrence { marked: false, slot_marked: false, ..second };
    let (mixed_arms, plain_pairs) = arms((first, plain_second));
    assert_eq!(
        mixed_arms,
        BTreeMap::from([
            (D8mBridgeArm::Computational, 1),
            (D8mBridgeArm::CheckedComputational, 1),
        ]),
        "removing ONE marker moves exactly ONE site from the checked arm to the unwrapped one, \
         and the total is unchanged. The ordinary arm being empty here describes this program \
         only; the positive for that arm is \
         d8m_the_ordinary_bridge_arm_is_reached_and_untouched: {mixed_arms:?}"
    );

    // Clause 2 — the unwrapped bridge contributes nothing checked.
    let planned = d8m_planned_frame_slots(first, plain_second);
    let observed = plain_pairs
        .into_iter()
        .map(|(_, frame_id, slot_template_id)| (frame_id, slot_template_id))
        .collect::<BTreeSet<_>>();
    let planned_pairs = planned
        .iter()
        .flat_map(|(frame, slots)| slots.iter().map(move |slot| (*frame, *slot)))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        observed, planned_pairs,
        "the checked population is exactly what the plan still names -- the first occurrence's \
         pair. An unwrapped bridge must not borrow the surviving frame: {observed:?}"
    );
    assert_eq!(
        planned_pairs.len(),
        1,
        "and there must be exactly one planned pair left, or clause 2 is not about an unwrapped \
         bridge at all: {planned_pairs:?}"
    );

    // Clause 3 — the bijection, both directions, with the source side measured
    // by the production collector rather than counted by hand.
    let declaration = d8m_two_occurrence_declaration(first, second);
    let RuntimeDeclarationKind::Transparent { body } = &declaration.kind else {
        panic!("transparent")
    };
    let mut markers = BTreeMap::new();
    crate::cranelift_backend::planning::collect_checked_subcontinuation_frames(body, &mut markers)
        .expect("the witness's frame markers collect");
    assert_eq!(
        markers.len(),
        d8m_two_occurrence_plan(first, second, None).frames.len(),
        "the lawful witness's Runtime marker count and plan frame count are equal: {markers:?}"
    );
    assert_eq!(markers.len(), 2, "and both are two: {markers:?}");
    let extra_planned = d8m_two_occurrence_compile_with(
        (first, second),
        (first, second),
        None,
        |plan| {
            let mut extra = plan.frames[0].clone();
            extra.frame_id = 9;
            extra.checked_occurrence_path = vec![9];
            extra.segment_site_id = 11;
            extra.semantic_position = 2;
            extra.occurrence_binding_fingerprint = 0;
            extra.occurrence_binding_fingerprint =
                crate::compiler_private_oriented_occurrence_binding_fingerprint(&extra);
            plan.frames.push(extra);
        },
        |body| body.clone(),
    );
    assert!(
        format!("{extra_planned:?}").contains("checked plan and Runtime marker sets differ"),
        "a plan frame the source does not mark must refuse: {extra_planned:?}"
    );
    let extra_marked = d8m_two_occurrence_compile_with(
        (first, second),
        (first, second),
        Some((first.frame_id, first.frame_id)),
        |plan| plan.frames.retain(|frame| frame.frame_id != 8),
        |body| body.clone(),
    );
    assert!(
        format!("{extra_marked:?}").contains("checked plan and Runtime marker sets differ"),
        "and so must a source marker the plan does not name. Preserving an identity through the \
         bridge must neither create nor consume a frame: {extra_marked:?}"
    );
}

/// The `D8m` witness with its bridge spelled as an ORDINARY `Match` instead of a
/// `ComputationalMatch`.
///
/// Same scrutinee, same constructors, same binder counts, same default. The case
/// BODIES necessarily differ, and saying so is part of the control rather than a
/// caveat on it: the computational bridge's bodies are `Call(Var(4), Var(1))`,
/// and `Var(4)` is the computational method binder, which an ordinary match does
/// not have. They are replaced by lawful `Unit` bodies, because a body naming an
/// absent binder would refuse for the missing callee instead of exercising the
/// arm.
///
/// That substitution costs the control nothing, because the classification never
/// reads the subtrees. `immediate_binder_eliminator` keys on the case body's
/// OUTER `RuntimeExpr` form and on the scrutinee being a `Var` in binder range;
/// what sits below is not consulted. So it is the outer descriptor form that
/// proves the arm here -- not identical subtrees -- and that form is exactly what
/// this witness varies.
#[cfg(test)]
fn d8m_ordinary_bridge_witness() -> RuntimeExpr {
    let RuntimeExpr::Let { value, body } = d8m_witness(7) else {
        panic!("let")
    };
    let RuntimeExpr::ComputationalMatch {
        scrutinee,
        cases,
        default,
    } = *value
    else {
        panic!("eliminator")
    };
    let RuntimeExpr::CheckedSubcontinuationFrame { body: bridge, .. } = &cases[0].body else {
        panic!("marked bridge")
    };
    let RuntimeExpr::ComputationalMatch {
        scrutinee: bridge_scrutinee,
        cases: bridge_cases,
        default: bridge_default,
    } = bridge.as_ref()
    else {
        panic!("bridge")
    };
    let ordinary = RuntimeExpr::Match {
        scrutinee: bridge_scrutinee.clone(),
        cases: bridge_cases
            .iter()
            .map(|case| crate::RuntimeMatchCase {
                constructor: case.constructor.clone(),
                binders: case.argument_binders,
                // The ordinary arm has no induction hypothesis, so the case body
                // returns its own bound payload rather than calling the method
                // binder the computational bridge's bodies reach for. That
                // binder does not exist on this arm, and a body that named it
                // would refuse for the missing callee rather than exercise the
                // arm. It returns a unit, which is what the eliminator's OTHER
                // case already returns, so both arms of the outer match agree.
                body: RuntimeExpr::Construct {
                    constructor: "ctor:prelude::Unit::MkUnit".to_string(),
                    args: Vec::new(),
                },
            })
            .collect(),
        default: bridge_default.clone(),
    };
    let mut cases = cases.clone();
    cases[0] = crate::RuntimeComputationalMatchCase {
        constructor: cases[0].constructor.clone(),
        argument_binders: cases[0].argument_binders,
        recursive_positions: cases[0].recursive_positions.clone(),
        body: ordinary,
    };
    RuntimeExpr::Let {
        value: Box::new(RuntimeExpr::ComputationalMatch {
            scrutinee,
            cases,
            default,
        }),
        body,
    }
}

/// Which arm the closed descriptor must take, read off the SOURCE.
///
/// The independent side of the arm classification: the descriptor keys on the
/// case body's `RuntimeExpr` constructor and on nothing else, so restating that
/// mapping over the witness's own text derives the expected arm without
/// consulting the recorder, the plan, or the lowering.
#[cfg(test)]
fn d8m_expected_arm(
    witness: &RuntimeExpr,
) -> crate::cranelift_backend::lowering::D8mBridgeArm {
    use crate::cranelift_backend::lowering::D8mBridgeArm;
    let RuntimeExpr::Let { value, .. } = witness else {
        panic!("let")
    };
    let RuntimeExpr::ComputationalMatch { cases, .. } = value.as_ref() else {
        panic!("eliminator")
    };
    match &cases[0].body {
        RuntimeExpr::CheckedSubcontinuationFrame { body, .. }
            if matches!(body.as_ref(), RuntimeExpr::ComputationalMatch { .. }) =>
        {
            D8mBridgeArm::CheckedComputational
        }
        RuntimeExpr::ComputationalMatch { .. } => D8mBridgeArm::Computational,
        RuntimeExpr::Match { .. } => D8mBridgeArm::Ordinary,
        other => panic!("the witness's case body is not a bridge at all: {other:?}"),
    }
}

#[cfg(test)]
const D8M_ORDINARY_SYMBOL: &str = "decl:fixture::d8m_ordinary::witness";

#[cfg(test)]
fn d8m_ordinary_compile() -> Option<CraneliftBackendError> {
    let declaration = RuntimeDeclaration {
        symbol: D8M_ORDINARY_SYMBOL.to_string(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::Closure {
                captures: Vec::new(),
                params: vec!["state".to_string()],
                body: Box::new(d8m_ordinary_bridge_witness()),
            },
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    };
    let entry = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::DeclarationRef {
            symbol: D8M_ORDINARY_SYMBOL.to_string(),
        }),
        args: vec![RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        }],
    };
    compile_expr_into_module(
        new_object_module("d8m-ordinary").expect("module"),
        "ken_d8m_ordinary",
        Linkage::Export,
        &entry,
        &NativeSeedEnvironment::empty(),
        BTreeMap::from([(D8M_ORDINARY_SYMBOL, &declaration)]),
        None,
        true,
        None,
        Some(test_only_distinguished_root_join_plan()),
        None,
    )
    .err()
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8m` — the ORDINARY bridge arm, positively
/// observed, and untouched by everything `D8m` added.**
///
/// The sibling row `d8m_the_bridge_arm_populations_move_only_with_the_marker`
/// measures the ordinary arm's population as ZERO on the two-occurrence witness.
/// A zero is a description of that program, not a proof that the arm still
/// works: an arm nothing reaches is an arm nothing tests. This row supplies the
/// positive.
///
/// ## The witness
///
/// The `D8m` witness with its bridge spelled as an ordinary `Match` instead of a
/// `ComputationalMatch`. Same scrutinee, same constructors, same binder counts,
/// same default.
///
/// The case bodies necessarily differ: the computational bridge's are
/// `Call(Var(4), Var(1))`, and `Var(4)` is the computational method binder, which
/// an ordinary match does not have, so they become lawful `Unit` bodies. The
/// classification does not read them. `immediate_binder_eliminator` keys on the
/// case body's OUTER `RuntimeExpr` form and on the scrutinee being a `Var` in
/// binder range, and it is that form -- not identical subtrees -- that this
/// witness varies and this row proves.
///
/// ## The independent side
///
/// `d8m_expected_arm` restates the descriptor's mapping over the witness's own
/// SOURCE text. It never consults the recorder, the plan, or the lowering; if
/// the descriptor and the source disagree about which arm this is, the two sides
/// disagree and the clause reds.
///
/// ## Clause 1 — the arm is reached, once, and classified from the source
///
/// ## Clause 2 — nothing `D8m` added moves it
///
/// Each of the three test-only perturbations of the checked path is armed in
/// turn and the ordinary witness's classification, count AND outcome must be
/// byte-identical each time. Two of them are marker-focused -- withholding the
/// tuple the checked bridge transports, and substituting the wrapper's own
/// occurrence for the wrapped match's -- and the third restores the pre-`D8n`
/// compile-wide ledger. An ordinary bridge has no marker, no transported tuple
/// and no consumed frame, so all three must be invisible to it. If one is ever
/// visible, `D8m`'s checked path has reached into a population it does not own.
///
/// ## Where THIS witness stops, and what that does not say about the others
///
/// It does not compile green. It reaches the arm, is classified, and then stops
/// at the projected-causal-edge seat -- *"the unit result at a projected causal
/// edge is not the planner's own constructor for that edge's producer Construct
/// origin"* -- a held `D8e`/`D8f` obligation.
///
/// That stop is a fact about THIS ordinary witness only, and clause 2 pins it
/// only as an EQUALITY across the three perturbations, never as an outcome the
/// checked path shares. The checked population does not stop there: the
/// two-occurrence checked witness COMPILES, and the single checked witness in
/// `d8m_the_source_frame_identity_survives_the_bridge` stops earlier, at the
/// missing-slot guard. Three different outcomes on three different programs, and
/// no one of them may be read onto another.
///
/// Pinning the stop as an equality rather than as a success is what keeps it
/// honest: if it ever changes under one of the three switches, that is the
/// finding.
///
/// **Promise class: durable invariant.** A source-derived classification, a
/// count, and an outcome held equal across three perturbations.
#[test]
fn d8m_the_ordinary_bridge_arm_is_reached_and_untouched() {
    use crate::cranelift_backend::lowering::core::{
        set_d8m_suppress_transported_tuple, set_d8m_wrapper_origin_substitution,
        set_d8n_compile_wide_lifecycle,
    };
    use crate::cranelift_backend::lowering::{
        d8m_bridge_arms, reset_d8n_observations, D8mBridgeArm,
    };

    let witness = d8m_ordinary_bridge_witness();
    let expected = d8m_expected_arm(&witness);
    assert_eq!(
        expected,
        D8mBridgeArm::Ordinary,
        "the witness's own source text must say this is an ordinary bridge, or the control is \
         classifying something else"
    );

    let observe = || {
        reset_d8n_observations();
        let outcome = format!("{:?}", d8m_ordinary_compile());
        (d8m_bridge_arms(), outcome)
    };

    // Clause 1 — reached, once, and classified as the source says.
    let (arms, outcome) = observe();
    assert_eq!(
        arms.len(),
        1,
        "the ordinary bridge must reach the composed arm recorder exactly once. Zero means the \
         witness never reaches the arm and the sibling row's zero is all there is; more than one \
         means it is not the single-site control it claims to be: {arms:?}"
    );
    assert_eq!(
        arms[0].1, expected,
        "and production must classify it the way the source descriptor does. This is the join \
         that makes the arm population a measurement: the expected side is read off the witness's \
         case body constructor, never off the recorder: {arms:?}"
    );

    // Clause 2 — nothing D8m added moves it.
    for (name, arm_switch) in [
        (
            "withholding the checked bridge's transported tuple",
            set_d8m_suppress_transported_tuple as fn(bool),
        ),
        (
            "substituting the wrapper's own occurrence for the wrapped match's",
            set_d8m_wrapper_origin_substitution as fn(bool),
        ),
        (
            "restoring the pre-D8n compile-wide consumed-frame ledger",
            set_d8n_compile_wide_lifecycle as fn(bool),
        ),
    ] {
        arm_switch(true);
        let (perturbed_arms, perturbed_outcome) = observe();
        arm_switch(false);
        assert_eq!(
            perturbed_arms, arms,
            "an ordinary bridge has no marker, no transported tuple and no consumed frame, so \
             {name} must be invisible to it -- same arm, same count, same Function. A difference \
             here means D8m's checked path has reached into a population it does not own: \
             {perturbed_arms:?}"
        );
        assert_eq!(
            perturbed_outcome, outcome,
            "and the outcome must be identical too, not merely still-an-error: {name} changed \
             where this witness stops, which is the same finding by a different route"
        );
    }
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8p` — a checked application binds and emits
/// in EVERY defining body that lowers it.**
///
/// ## What was missing
///
/// Two call edges emit a static-worker call: the direct descent in `lower_expr`,
/// and the source machine's. The direct one has consulted the checked-IH seam
/// since `D5a`; **the source machine's did not**. So a checked-IH invocation
/// marker entered in a body whose application the source machine lowers could
/// never be consumed there, and failed closed at the marker's close as *"a
/// checked computational-IH marker is a specialized-only surface"*.
///
/// That is what `D8f`'s hard stop measured at `0eb04397`, and its sentinel
/// `d8f_the_composed_route_still_cannot_host_an_invocation_marker` said to
/// invert rather than delete when the obstacle went. This row is that inversion.
///
/// ## The relation, keyed on the pair the checkpoint is about
///
/// `(exact defining body, exact application occurrence)` -> the plan side
/// `(call template, slot template, binder ordinal, arity)` and the target side
/// `(target body origin, declared arity, captures, supplied operand run)`.
///
/// The plan side is written at the seam where the binding happens; the target
/// side **after the call instruction exists**, carrying the run the instruction
/// actually took. Written before the emitter it would be a claim about a call
/// that had not been made, and an operand run widened inside the emitter would
/// be invisible to it.
///
/// The independent side of the target relation is `d8p_planned_targets`: the
/// planner's own definition of each callable, from unit definition. The
/// observation says WHICH target; the planner says what that target's declared
/// arity and capture run must be, and the emitted operand count must be their
/// sum. The body key is `D8o`'s, supplied by the pass that owns the descriptor,
/// so body KIND is carried and never recovered from an owner variant.
///
/// ## Clause 3 — the permutation, and its MEASURED limit
///
/// On this witness the two defining bodies bind at the **same** application
/// occurrence and call the **same** target with the same arity and capture run.
/// Exchanging their observations is therefore a no-op that neither an unkeyed
/// comparison nor the keyed relation can see. That is recorded, not asserted
/// away, and no claim is made here that the occurrences are distinct or that the
/// swap currently bites.
///
/// What would make it discriminating is a **population change, not a stronger
/// assertion**: a witness whose defining bodies bind at different application
/// occurrences, or call different targets. This one has a single checked
/// application lowered by two bodies, so both coordinates coincide by
/// construction. No such population is fabricated here.
///
/// **Promise class: durable invariant.**
#[test]
fn d8p_a_checked_application_binds_and_emits_in_every_defining_body() {
    use crate::cranelift_backend::lowering::{
        d8o_body_keys, d8p_application_bindings, d8p_emitted_targets, reset_d8n_observations,
        reset_d8o_body_authorities, D8oBodyKey,
    };

    reset_d8n_observations();
    reset_d8o_body_authorities();
    let outcome = d8f_compile(false);
    assert!(
        outcome.is_none(),
        "the composed checked-application witness must compile. Before D8p it refused with 'a \
         checked computational-IH marker is a specialized-only surface', because the source \
         machine's call edge never consulted the seam: {outcome:?}"
    );

    let keys = d8o_body_keys()
        .into_iter()
        .map(|(function, key)| (function.expect("body keys are labelled"), key))
        .collect::<BTreeMap<_, _>>();
    let bindings = d8p_application_bindings();
    let targets = d8p_emitted_targets();

    // Clause 1 — every binding is under the plan's own template run, keyed on
    // the exact body and the exact application occurrence.
    let planned = d8p_planned_application(false);
    let mut plan_side: BTreeMap<(D8oBodyKey, StaticOriginId), (u64, u64, u64, u64)> =
        BTreeMap::new();
    for binding in &bindings {
        let function = binding
            .function
            .expect("every checked-application binding names its defining Function");
        let key = *keys
            .get(&function)
            .expect("every binding body recorded its exact body key");
        let previous = plan_side.insert(
            (key, binding.application_origin),
            (
                binding.call_template_id,
                binding.slot_template_id,
                binding.binder_index,
                binding.arity,
            ),
        );
        assert!(
            previous.is_none(),
            "one application occurrence binds at most ONCE in one defining body; a second is a \
             duplicate consumption the affine law is supposed to have refused: {bindings:?}"
        );
    }
    assert!(
        !plan_side.is_empty(),
        "at least one checked application must bind, or every clause below is vacuous"
    );
    for (key, observed) in &plan_side {
        assert_eq!(
            *observed, planned,
            "each binding must agree with the PLAN's call template, slot, binder ordinal and \
             arity, under key {key:?}. The independent side is the oriented plan the compile was \
             handed; the observed side is what the seam bound"
        );
    }

    // Clause 2 — a real emitted call under the SAME key, whose target and
    // operand run are the PLANNER's for that target.
    let planned_targets = d8p_planned_targets(false);
    let mut target_side: BTreeMap<(D8oBodyKey, StaticOriginId), (StaticOriginId, u32, usize, usize)> =
        BTreeMap::new();
    for target in &targets {
        let function = target
            .function
            .expect("every emitted checked application names its defining Function");
        let key = *keys.get(&function).expect("an emitting body has a key");
        let previous = target_side.insert(
            (key, target.application_origin),
            (
                target.target_body_origin,
                target.declared_arity,
                target.captures,
                target.supplied_operands,
            ),
        );
        assert!(
            previous.is_none(),
            "one (defining body, application occurrence) emits at most ONE checked call. A second \
             observation under the same key must be REJECTED rather than replace the first -- \
             overwriting would hide exactly the duplicate this relation exists to exclude: \
             {targets:?}"
        );
    }
    assert_eq!(
        target_side.keys().collect::<Vec<_>>(),
        plan_side.keys().collect::<Vec<_>>(),
        "every binding must have produced a real emitted call at the SAME exact body and \
         application occurrence, and nothing may be emitted under a key that did not bind. A \
         binding with no emission is an application that was accounted for and never made: \
         {targets:?} vs {bindings:?}"
    );
    for (key, (body_origin, declared_arity, captures, supplied)) in &target_side {
        let (planned_arity, planned_captures) = planned_targets.get(body_origin).unwrap_or_else(|| {
            panic!(
                "the call under key {key:?} went to {body_origin:?}, which the PLANNER defines no \
                 unit for. A target the planner does not define is a callable this body invented: \
                 {planned_targets:?}"
            )
        });
        assert_eq!(
            (*declared_arity, *captures),
            (*planned_arity, *planned_captures),
            "and it must be called at the planner's own declared arity and capture run for that \
             target, under key {key:?}. A widened or narrowed contract here is the ABI widening \
             this checkpoint refuses"
        );
        assert_eq!(
            *supplied,
            *planned_arity as usize + *planned_captures,
            "and the run the INSTRUCTION carried must be exactly that contract -- explicit \
             arguments then stored captures, nothing appended. This is read off the emission \
             after the call exists, so an operand assembled inside the emitter is visible here, \
             under key {key:?}"
        );
    }

    // Clause 3 — both body kinds bind. The permutation is measured, not claimed.
    let bodies = plan_side.keys().map(|(key, _)| *key).collect::<BTreeSet<_>>();
    assert!(
        bodies
            .iter()
            .any(|key| matches!(key, D8oBodyKey::OrdinaryUnit(_))),
        "the ordinary unit body must bind -- that body is the one the pre-D8p refusal came from, \
         so a witness where only a specialization binds proves nothing about the repair: \
         {bodies:?}"
    );
    assert!(
        bodies.len() >= 2,
        "and more than one defining body must lower this application, or 'in every defining body' \
         is a claim about a population of one: {bodies:?}"
    );
    let permuted = {
        let entries = target_side
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<_>>();
        let mut permuted = target_side.clone();
        permuted.insert(entries[0].0, entries[1].1);
        permuted.insert(entries[1].0, entries[0].1);
        permuted
    };
    assert_eq!(
        permuted, target_side,
        "MEASURED: this witness's defining bodies bind at the SAME application occurrence and call \
         the SAME target with the same contract, so exchanging their observations is a no-op and \
         neither an unkeyed comparison nor the keyed relation is discriminated by it here. What \
         would change that is a POPULATION with more than one checked application -- bodies \
         binding at different occurrences, or calling different targets -- not a stronger \
         assertion over this one. If this ever differs, the permutation has become a real control \
         and this expectation should be inverted rather than deleted"
    );
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8p` — zero, one, and declined, per exact
/// application occurrence.**
///
/// ## Zero
///
/// The `D8n` witness has composed static-worker calls and no invocation marker
/// at all. The seam is on its call edge now, and it must bind nothing and change
/// nothing: an ordinary composed call is untouched, exactly as it is on the
/// direct descent.
///
/// ## One
///
/// The `D8p` witness binds exactly once per defining body.
///
/// ## Declined
///
/// The same witness with an ORDINARY call on the same selected recursive
/// argument inside the pending marker's own application. Two static-worker calls
/// reach the seat under one pending marker, and they are the same worker at the
/// same arity in the same frame -- route, arity, binder index and call order are
/// all blind. Exactly one binds, and it is the one at the plan's application
/// occurrence; the other emits with no binding.
///
/// This is the occupancy behaviour `D8f` is about, and it is now on a live path.
/// The program **compiles**.
///
/// It did not when this row was first written. It refused at the affine causal
/// law -- *"one causal identity was discharged twice in a single function"* --
/// because the declined call still answered for the checked application's
/// composed identity. `D8f`'s closed three-case disposition stopped it
/// answering, and `d8f_the_declined_call_does_not_answer_for_the_checked_identity`
/// holds that as a difference.
///
/// ⛔ `D8f` is still NOT discharged and this row does not claim it: its
/// omission, duplicate, transplant and wrong-occurrence refusals are owed on
/// separate live paths. What this row establishes is the BINDING behaviour, and
/// it should not be read as more.
///
/// **Promise class: durable invariant.**
#[test]
fn d8p_binding_is_zero_one_or_declined_per_application_occurrence() {
    use crate::cranelift_backend::lowering::{
        d5a_marker_events, d8p_application_bindings, reset_d5a_marker_events,
        reset_d8n_observations, D5aMarkerEvent,
    };

    // Zero.
    reset_d8n_observations();
    reset_d5a_marker_events();
    let plain = d8n_compile();
    assert!(plain.is_none(), "the unmarked composed witness compiles: {plain:?}");
    assert!(
        d8p_application_bindings().is_empty(),
        "a composed static-worker call with no marker pending must bind NOTHING. The seam is on \
         this edge now, so 'it only fires under a marker' has to be measured rather than read off \
         the code: {:?}",
        d8p_application_bindings()
    );
    assert!(
        d5a_marker_events()
            .iter()
            .any(|event| matches!(event, D5aMarkerEvent::WorkerCallEmitted { .. })),
        "and the calls must still be emitted, or the zero above is the zero of a path nothing took"
    );

    // One.
    reset_d8n_observations();
    let one = d8f_compile(false);
    assert!(one.is_none(), "the marked witness compiles: {one:?}");
    let bound = d8p_application_bindings();
    let occurrences = bound
        .iter()
        .map(|binding| (binding.function, binding.application_origin))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        occurrences.len(),
        bound.len(),
        "each (defining body, application occurrence) binds at most once: {bound:?}"
    );
    assert!(!bound.is_empty(), "and at least one binds: {bound:?}");

    // Declined.
    reset_d8n_observations();
    reset_d5a_marker_events();
    let declined = d8f_compile(true);
    let events = d5a_marker_events();
    let emitted = events
        .iter()
        .filter(|event| matches!(event, D5aMarkerEvent::WorkerCallEmitted { .. }))
        .count();
    let consumed = events
        .iter()
        .filter(|event| matches!(event, D5aMarkerEvent::Consumed { .. }))
        .count();
    assert!(
        emitted >= 2,
        "two static-worker calls must reach the seat under one pending marker, or there is no \
         occupancy question on this program: {events:?}"
    );
    assert_eq!(
        consumed, 2,
        "and exactly one binding per defining body: this witness's checked source body is lowered \
         by two, so two consumptions is once each. Fewer means a body never found its own marker; \
         more means a call the planner issued no template for consumed one: {events:?}"
    );
    assert!(
        matches!(events.first(), Some(D5aMarkerEvent::WorkerCallEmitted { .. })),
        "the FIRST call emitted is the ordinary one, and it emits with no consumption before it. \
         That ordering is the property: the ordinary call reaches the seat first, declines, and \
         leaves the marker for the occurrence that owns it: {events:?}"
    );
    assert!(
        declined.is_none(),
        "and the program COMPILES. Until D8f it refused here with 'one causal identity was \
         discharged twice in a single function', because the declined call still answered for the \
         checked application's composed identity. D8f's three-case disposition stopped it \
         answering; d8f_the_declined_call_does_not_answer_for_the_checked_identity holds that as \
         a difference: {declined:?}"
    );
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8p` — the three refusals the projection must
/// not weaken.**
///
/// Projecting the checked-application seam onto a second call edge widens where a
/// marker CAN be consumed. Each clause below is a case that must still refuse.
///
/// ## Clause 1 — no unauthorised semantic IH layer in a checked segment
///
/// The landed `px8tr` object fixture, with an ordinary call on the same recursor
/// binder nested inside its checked application. The nested call instantiates a
/// semantic IH layer of its own, and a segment carrying a checked frame requires
/// every semantic layer to carry a checked invocation authority. `D8p` must not
/// make that layer acceptable.
///
/// ## Clause 2 — no pending-marker acceptance at the marker's close
///
/// With consumption suppressed, the marker reaches its close still pending and
/// must fail closed. `D8p` moved WHERE a marker may be consumed; it must not
/// have moved WHETHER an unconsumed one is tolerated.
///
/// ## Clause 3 — no bypass of the source-open versus dynamic-parent comparison
///
/// Pinned by the landed row that names that refusal directly; this clause states
/// the dependency rather than restating the fixture, so there is one copy of that
/// evidence and it cannot go stale in two places.
///
/// **Promise class: durable invariant.**
#[test]
fn d8p_preserves_the_refusals_the_projection_could_have_weakened() {
    use crate::cranelift_backend::lowering::{with_d5a_marker_mutation, D5aMarkerMutation};

    // Clause 1.
    crate::cranelift_backend::test_objects::set_px8tr_nest_ordinary_ih_call(true);
    let nested = crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
        "ken_d8p_nested",
        false,
    )
    .err();
    crate::cranelift_backend::test_objects::set_px8tr_nest_ordinary_ih_call(false);
    let nested = format!("{nested:?}");
    assert!(
        nested.contains("oriented segment mixes checked and inferred computational frames"),
        "an ordinary call nested inside the checked application instantiates a semantic IH layer \
         with no checked invocation authority, and a segment carrying a checked frame must still \
         refuse it. D8p widened where a marker may be consumed; if that also made an unauthorised \
         layer acceptable, this is where it shows: {nested}"
    );

    // Clause 2.
    let pending = with_d5a_marker_mutation(D5aMarkerMutation::SuppressConsumption, || {
        format!("{:?}", d8f_compile(false))
    });
    assert!(
        pending.contains("a checked computational-IH marker is a specialized-only surface")
            || pending.contains("marker"),
        "with consumption withheld the marker reaches its close still pending and must fail \
         closed. This is the exact refusal D8p's repair routes AROUND when the marker is genuinely \
         consumed, so a green here would mean the repair works by tolerating the unconsumed case \
         rather than by consuming: {pending}"
    );
    assert!(
        !pending.contains("None"),
        "and it must be a refusal, not a compile: {pending}"
    );
}


/// The child index at which the marker collector enters a `Transparent`
/// declaration's closure body.
///
/// ⚠ The ONE element of the expected path below that is not derived from the
/// witness. `expression_children` enumerates a `Closure` as a single child, and
/// the collector roots its runtime paths one convention further out; this names
/// that convention rather than letting it hide inside a measured number. Every
/// element after it -- the whole of the marker's position INSIDE the body, which
/// is what a transplant moves -- is derived.
#[cfg(test)]
const D8F_DECLARATION_BODY_ROOT: u64 = 2;

/// **The independent planning expectation:**
/// `(declaration, checked path, template) -> locations`.
///
/// Derived by walking the witness's own expression tree with
/// `expression_children` -- an independent enumeration of child order, authored
/// for a different checkpoint -- and recording where each
/// `CheckedComputationalIHInvocation` sits.
///
/// ⛔ It consults **neither** `collect_checked_oriented_markers` nor any plan
/// built from its output. Both the actual source population and the actual plan
/// population are required to equal it, so agreement between those two cannot
/// stand in for correctness of either.
#[cfg(test)]
fn d8f_expected_marker_population(
    with_ordinary_call: bool,
    perturbation: D8fPerturbation,
) -> BTreeMap<(String, Vec<u64>, u64), BTreeSet<Vec<u64>>> {
    let declaration = d8f_declaration_with(with_ordinary_call, perturbation);
    let RuntimeDeclarationKind::Transparent { body } = &declaration.kind else {
        panic!("transparent")
    };
    fn walk(
        expr: &RuntimeExpr,
        path: &mut Vec<u64>,
        out: &mut BTreeMap<(String, Vec<u64>, u64), BTreeSet<Vec<u64>>>,
    ) {
        if let RuntimeExpr::CheckedComputationalIHInvocation {
            call_template_id,
            checked_occurrence_path,
            ..
        } = expr
        {
            let mut located = path.clone();
            located[0] = D8F_DECLARATION_BODY_ROOT;
            let previous = out.insert(
                (
                    D8F_SYMBOL.to_string(),
                    checked_occurrence_path.clone(),
                    *call_template_id,
                ),
                BTreeSet::from([located]),
            );
            assert!(
                previous.is_none(),
                "the witness must not spell two invocation markers under one \
                 (declaration, path, template) key -- that is a fixture error, not a control"
            );
        }
        for (index, child) in expression_children(expr).into_iter().enumerate() {
            path.push(index as u64);
            walk(child, path, out);
            path.pop();
        }
    }
    let mut expected = BTreeMap::new();
    walk(body, &mut Vec::new(), &mut expected);
    expected
}

/// The checked-IH invocation markers the SOURCE carries, keyed by declaration
/// and checked occurrence path.
///
/// Measured by the production collector over the witness's own text. This is the
/// actual source-marker population a planning row compares against.
#[cfg(test)]
fn d8f_source_marker_population(
    with_ordinary_call: bool,
    perturbation: D8fPerturbation,
) -> BTreeMap<(String, Vec<u64>, u64), BTreeSet<Vec<u64>>> {
    let mut population = BTreeMap::new();
    for ((template, path), locations) in
        d8f_marker_sets(with_ordinary_call, perturbation).computational_ih_calls
    {
        let previous = population.insert((D8F_SYMBOL.to_string(), path, template), locations);
        assert!(
            previous.is_none(),
            "two source-marker entries under one (declaration, path, template) key. Collecting \
             would have kept the last and hidden the duplicate this relation exists to see"
        );
    }
    population
}

/// The checked-IH invocation templates the PLAN holds, under the same key.
#[cfg(test)]
fn d8f_plan_marker_population(
    with_ordinary_call: bool,
    perturbation: D8fPerturbation,
) -> BTreeMap<(String, Vec<u64>, u64), BTreeSet<Vec<u64>>> {
    let mut population = BTreeMap::new();
    for call in d8f_plan_with(with_ordinary_call, perturbation).computational_ih_calls {
        let template = call.call_template_id;
        let previous = population.insert(
            (
                call.declaration.clone(),
                call.checked_occurrence_path.clone(),
                template,
            ),
            call.runtime_marker_locations
                .into_iter()
                .map(|location| location.runtime_path)
                .collect::<BTreeSet<_>>(),
        );
        assert!(
            previous.is_none(),
            "two plan templates under one (declaration, path, template) key: template {template}"
        );
    }
    population
}

/// The lowering observations for one compile, keyed by exact defining body and
/// application occurrence.
///
/// `D8oBodyKey` is the body kind carried from the pass that owns the descriptor,
/// never recovered from an owner variant.
#[cfg(test)]
#[allow(clippy::type_complexity)]
fn d8f_keyed_observations() -> BTreeMap<
    (crate::cranelift_backend::lowering::D8oBodyKey, StaticOriginId),
    (
        crate::cranelift_backend::lowering::CheckedApplicationDisposition,
        bool,
    ),
> {
    let keys = crate::cranelift_backend::lowering::d8o_body_keys()
        .into_iter()
        .map(|(function, key)| (function.expect("body keys are labelled"), key))
        .collect::<BTreeMap<_, _>>();
    let bound = crate::cranelift_backend::lowering::d8p_application_bindings()
        .into_iter()
        .map(|binding| {
            (
                binding.function.expect("bindings name their Function"),
                binding.application_origin,
            )
        })
        .collect::<BTreeSet<_>>();
    let mut observed = BTreeMap::new();
    for (function, origin, disposition) in crate::cranelift_backend::lowering::d8f_dispositions() {
        let function = function.expect("every disposition names its defining Function");
        let key = *keys
            .get(&function)
            .expect("every emitting body recorded its exact body key");
        let previous = observed.insert(
            (key, origin),
            (disposition, bound.contains(&(function, origin))),
        );
        assert!(
            previous.is_none(),
            "two dispositions under one (defining body, application occurrence) key. Collecting \
             would have kept the last: one call edge, one record"
        );
    }
    observed
}

/// **The two application occurrences of the moved witness, named from PLANNER
/// authority.**
///
/// Returns `(ordinary selected-argument application, checked application)`.
///
/// ⛔ Derived by walking `StaticTransitionPlan::child_static_origin` -- the
/// planner's sole child-origin production point -- down a positional path
/// spelled from the witness's own shape. It consults **neither**
/// `CheckedApplicationDisposition` nor a `D8p` binding, on this program or any
/// other, and neither origin is the complement of an observed relation.
///
/// The path, from the continuation unit's own frame occurrence:
///
/// | step | node |
/// |---|---|
/// | `1` | case 0's body of the eliminator: the `CheckedSubcontinuationFrame` |
/// | `0` | the bridge `ComputationalMatch` it wraps |
/// | `1` | that match's case 0 body: the `CheckedComputationalIHSlots` marker |
/// | `0` | the slot marker's body: the OUTER application |
/// | `1` | its argument: the `CheckedComputationalIHInvocation` marker |
/// | `0` | the marker's body: the INNER application |
///
/// The outer application is the checked one -- the marker moved off it -- and
/// the inner is the ordinary selected-argument call the marker now names.
#[cfg(test)]
fn d8f_moved_application_origins() -> (StaticOriginId, StaticOriginId) {
    let declaration = d8f_declaration_with(true, D8fPerturbation::MarkerMovedInward);
    let entry = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::DeclarationRef {
            symbol: D8F_SYMBOL.to_string(),
        }),
        args: vec![RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        }],
    };
    let declarations = BTreeMap::from([(D8F_SYMBOL, &declaration)]);
    let plan = plan_static_transition_graph_with_symbols(
        &entry,
        &declarations,
        &crate::NativeProcessSymbols::legacy_prelude(),
        AbiRootIngress::Value,
        true,
    )
    .expect("the moved witness plans");
    let units = plan.continuation_units().expect("continuation units");
    let unit = units.first().expect("one continuation unit");
    let mut cursor = unit.continuation_origin();
    for step in [1usize, 0, 1, 0] {
        cursor = plan
            .child_static_origin(cursor, step)
            .expect("the planner names this child occurrence");
    }
    let checked = cursor;
    for step in [1usize, 0] {
        cursor = plan
            .child_static_origin(cursor, step)
            .expect("the planner names this child occurrence");
    }
    let ordinary = cursor;
    assert_ne!(
        ordinary, checked,
        "the two applications must be distinct occurrences, or the path above collapsed and this \
         control has one subject rather than two"
    );
    (ordinary, checked)
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8f` — the remaining checked-marker refusals:
/// omission, duplicate, transplant, wrong occurrence.**
///
/// All four are about **which call consumes a pending checked-IH marker**. Each
/// is proved by a relation derived independently of the mechanism under test;
/// the refusal message is retained only as a supplementary guard on WHICH PLANE
/// caught it, never as the proof.
///
/// ## The lowering key
///
/// `(exact defining body, exact application occurrence) -> (disposition, was it
/// bound)`. The disposition is recorded **after the call instruction exists**, so
/// a record means "this exact call was emitted"; the binding half comes from the
/// seam, a different site. Body kind is `D8o`'s supplied key.
///
/// ## Omission — emitted, and nothing consumed it
///
/// The sole producer of the consumption is withheld and nothing else moves. Every
/// call that reaches the edge is still emitted -- the relation is non-empty -- and
/// **no key is bound**. That is the omission population stated as a difference,
/// and no lawful source constructs it: the planner issues a call template only
/// for an application it saw, which is why the source-shaped variant this
/// checkpoint first tried was removed rather than kept as scaffolding.
///
/// ## Wrong occurrence — the moved marker was consumed before closeout refused
///
/// With the marker on the ordinary call and the plan following it, the keyed
/// relation shows the **moved** occurrence bound and `ConsumedHere`, and the
/// checked application's own occurrence unbound. Those are lowering facts,
/// recorded before the affine closeout refuses, so the refusal is attributed to
/// a consumption that demonstrably happened at the wrong call rather than
/// inferred from the error.
///
/// ⚠ The law that refuses is the AFFINE CAUSAL one, not a marker law. The
/// marker-plane defence is the occupancy gate, whose always-admit mutation is
/// established at `20b0d6be` and is not re-run.
///
/// ## Duplicate and transplant — population relations, not errors
///
/// Both are settled in planning, so the proof is a comparison of populations
/// under exact `(declaration, checked occurrence path, template)` keys: what the
/// SOURCE carries, against what the PLAN holds. Duplicate: the source carries two
/// invocation markers and the plan holds both templates, so the program is
/// admitted that far and the nesting law is genuinely what refuses. Transplant:
/// the source's marker location under the plan's own key differs from the
/// location the plan records for it, which is exactly the disagreement the
/// location comparison exists to catch.
///
/// **Promise class: durable invariant.**
#[test]
fn d8f_the_remaining_checked_marker_refusals() {
    use crate::cranelift_backend::lowering::{
        d5a_marker_events, reset_d5a_marker_events, reset_d8j_discharged,
        reset_d8n_observations, reset_d8o_body_authorities, with_d5a_marker_mutation,
        CheckedApplicationDisposition, D5aMarkerEvent, D5aMarkerMutation,
    };

    // The lawful witnesses compile, so no refusal below is one these programs
    // carry anyway.
    reset_d8n_observations();
    reset_d8o_body_authorities();
    reset_d8j_discharged();
    assert!(
        d8f_compile(false).is_none() && d8f_compile(true).is_none(),
        "both lawful witnesses must compile, or every control below could be reporting a refusal \
         the program carries regardless of its perturbation"
    );

    // === Omission ===
    reset_d8n_observations();
    reset_d8o_body_authorities();
    reset_d5a_marker_events();
    let omitted = with_d5a_marker_mutation(D5aMarkerMutation::SuppressConsumption, || {
        format!("{:?}", d8f_compile(false))
    });
    let observed = d8f_keyed_observations();
    assert!(
        !observed.is_empty(),
        "the calls must still be EMITTED -- only the consumption is withheld. An empty relation \
         means nothing reached the emission edge and the control is measuring an unreached path"
    );
    assert!(
        observed.values().all(|(_, bound)| !bound),
        "and NO key may be bound: the sole producer of a consumption is absent, so every emitted \
         call is unconsumed. This is the omission population as a relation, not as an error \
         string: {observed:?}"
    );
    assert!(
        d5a_marker_events()
            .iter()
            .any(|event| matches!(event, D5aMarkerEvent::WorkerCallEmitted { .. })),
        "and a real call instruction exists, from the independent marker-event log"
    );
    assert!(
        omitted.contains("a checked computational-IH marker is a specialized-only surface"),
        "supplementary: the plane that catches it is the marker's own close, which fails closed \
         on a marker nothing consumed: {omitted}"
    );

    // === Wrong occurrence ===
    //
    // The independent side: BOTH application occurrences are named by the
    // planner's own child-origin walk over the moved witness. Nothing here is
    // read from a disposition, from a binding, or as the complement of one.
    let (named_ordinary, named_checked) = d8f_moved_application_origins();

    reset_d8n_observations();
    reset_d8o_body_authorities();
    let wrong = format!(
        "{:?}",
        d8f_compile_with(
            true,
            D8fPerturbation::MarkerMovedInward,
            D8fPerturbation::MarkerMovedInward,
        )
    );
    let moved = d8f_keyed_observations();
    let bodies = moved.keys().map(|(key, _)| *key).collect::<BTreeSet<_>>();
    assert_eq!(
        bodies.len(),
        1,
        "the moved run's observations must sit under ONE exact defining body, or 'the ordinary \
         call took the checked application's marker' is a claim about two bodies at once: \
         {moved:?}"
    );
    let body = *bodies.iter().next().expect("one body");
    assert_eq!(
        moved.get(&(body, named_ordinary)),
        Some(&(CheckedApplicationDisposition::ConsumedHere, true)),
        "THE MOVED ORDINARY APPLICATION, named by the planner's child-origin walk before any \
         observation is read: it must be EMITTED (a record exists at all, and dispositions are \
         written after the instruction), ConsumedHere, AND bound at the seam: {moved:?}"
    );
    assert_eq!(
        moved.get(&(body, named_checked)),
        Some(&(CheckedApplicationDisposition::NoPendingApplication, false)),
        "THE CHECKED APPLICATION, named the same way: emitted and UNBOUND -- left unaccounted \
         because the ordinary call took its marker. If it were bound there would be no \
         misattribution for the affine law to refuse, and this control would be describing a \
         lawful program: {moved:?}"
    );
    assert!(
        wrong.contains("one causal identity was discharged twice in a single function"),
        "supplementary: the plane that catches it is the AFFINE CAUSAL law, not a marker law. The \
         marker-plane defence is the occupancy gate, established at 20b0d6be: {wrong}"
    );

    // === Duplicate — both actual populations against the INDEPENDENT expectation ===
    let expected = d8f_expected_marker_population(false, D8fPerturbation::NestedMarker);
    let source = d8f_source_marker_population(false, D8fPerturbation::NestedMarker);
    let planned = d8f_plan_marker_population(false, D8fPerturbation::NestedMarker);
    assert_eq!(
        expected.len(),
        2,
        "the duplicate perturbation must carry exactly TWO invocation markers, or there is \
         nothing to nest. Counted from the witness's own tree, not from a collector: {expected:?}"
    );
    assert_eq!(
        source, expected,
        "the actual SOURCE population must equal the independent expectation, under exact \
         (declaration, path, template) keys: {source:?}"
    );
    assert_eq!(
        planned, expected,
        "and so must the actual PLAN population. Both are compared against the same independently \
         derived side, so their agreeing with each other cannot stand in for either being right: \
         {planned:?}"
    );
    let duplicated = format!(
        "{:?}",
        d8f_compile_with(false, D8fPerturbation::NestedMarker, D8fPerturbation::NestedMarker)
    );
    assert!(
        duplicated.contains("nested computational IH invocation marker"),
        "supplementary: with the populations agreeing, the plane that catches it is the nesting \
         law -- one pending checked application at a time: {duplicated}"
    );

    // === Transplant — the same independent expectation pins all three facts ===
    let moved_expected = d8f_expected_marker_population(true, D8fPerturbation::MarkerMovedInward);
    let stale_expected = d8f_expected_marker_population(true, D8fPerturbation::None);
    let moved_source = d8f_source_marker_population(true, D8fPerturbation::MarkerMovedInward);
    let stale_plan = d8f_plan_marker_population(true, D8fPerturbation::None);
    assert_eq!(
        moved_expected.keys().collect::<Vec<_>>(),
        stale_expected.keys().collect::<Vec<_>>(),
        "THE UNCHANGED KEY: a transplant keeps the same declaration, path and template -- only \
         the marker's location moves. Derived from the two witnesses' own trees. If the keys \
         differed this would be a population mismatch and not a transplant at all"
    );
    assert_eq!(
        moved_source, moved_expected,
        "THE MOVED SOURCE LOCATION: the actual source population must equal the independent \
         expectation for the MOVED witness: {moved_source:?}"
    );
    assert_eq!(
        stale_plan, stale_expected,
        "THE STILL-PLANNED LOCATION: the plan must still hold the location the UNMOVED witness \
         independently expects -- that is what makes it stale rather than merely different: \
         {stale_plan:?}"
    );
    assert_ne!(
        moved_expected, stale_expected,
        "and the two expectations must differ, or the perturbation moved nothing and every \
         assertion above is satisfied by a transplant that never happened"
    );
    let transplanted = format!(
        "{:?}",
        d8f_compile_with(true, D8fPerturbation::MarkerMovedInward, D8fPerturbation::None)
    );
    assert!(
        transplanted.contains("checked computational-IH call Runtime occurrences differ"),
        "supplementary: the plane that catches it is PLANNING's location comparison, so a \
         transplanted marker never reaches the seam: {transplanted}"
    );
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8g` — the functionized population's table
/// choice and suffix contract, keyed at the shared emitter.**
///
/// ## The two populations, and why they are two rows
///
/// `D8g` closes over the two populations that actually exist. No combined
/// witness is built: the `px8tr` A/B witness reaches a same-body
/// `GeneratedContext` IH and makes no composed-call-target emission, and the
/// composed family is the reverse. Measured at `1b367065` and recorded in the
/// hard stop that produced this recut.
///
/// They are joined where they genuinely meet: `call_static_worker_with_inputs`,
/// the one emitter both ingresses reach. Every fact below is written there,
/// after the instruction exists.
///
/// ## What the independent side IS, and what it is NOT
///
/// It is the planner's own **contract** populations -- unit definition for a
/// worker body's declared arity and capture count, and the continuation-context
/// population for the capture run a retarget declares -- read from a plan built
/// separately from the emission run, so no expectation is an echo of the compile
/// that produced the observations.
///
/// ⛔ **It is NOT a per-call planning authority, and this row must not be read
/// as one.** No planner or unit authority names the per-body source-`Call`
/// population, so there is no independently enumerated expected key set and no
/// independently selected target. The keys and the targets below are the
/// ACTUAL ones; what the planner supplies is the contract each named target must
/// satisfy.
///
/// ## THE RESIDUAL, stated plainly
///
/// **A defect that moved target and route TOGETHER to another lawful
/// same-shaped worker would not be caught here.** Every clause would still hold:
/// the substituted target has its own planner-declared contract, its own route
/// implied by whether a context exists for it, and its own suffix. Catching that
/// needs an authority naming which target each source call is *supposed* to
/// reach, and none exists. This row owns table choice and suffix contract for
/// the target a call names; it does not own which target it names.
///
/// ## Clause 1 — the keyed relation, one emission per call occurrence
///
/// `(defining Function, call occurrence) -> (target, route, raw run, supplied
/// run)`. Built by explicit insertion that fails on a previous value, so a
/// second emission under one key is a red rather than an overwrite. That is the
/// "neither duplicates target or operand assembly" half.
///
/// ## Clause 2 — the table choice owns the suffix
///
/// A context-routed call carries the raw run followed by exactly the capture run
/// its generated context declares; a raw-routed call carries the raw run and
/// nothing. Both are compared against the planner's declaration, not against
/// each other.
///
/// **Promise class: durable invariant.**
#[test]
fn d8g_the_functionized_population_binds_its_table_and_suffix_at_the_shared_emitter() {
    use crate::cranelift_backend::lowering::{d8g_emissions, reset_d8g_emissions};

    reset_d8g_emissions();
    crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
        "ken_d8g_functionized",
        false,
    )
    .map(|_| ())
    .map_err(|error| format!("{error:?}"))
    .expect("the A/B witness compiles");
    let emissions = d8g_emissions();

    // Clause 1 — one emission per (defining Function, call occurrence).
    let mut keyed = BTreeMap::new();
    for emission in &emissions {
        let function = emission
            .function
            .expect("every static-worker call names its defining Function");
        let previous = keyed.insert(
            (function, emission.call_origin),
            (
                emission.target_body_origin,
                emission.route,
                emission.raw_operands,
                emission.supplied_operands,
                emission.captures,
                emission.declared_arity,
                emission.emitted_callee,
            ),
        );
        assert!(
            previous.is_none(),
            "one call occurrence emits at most ONE static-worker call in one defining Function. A \
             second under the same key is a duplicated target or operand assembly, and \
             overwriting would hide exactly that: {emissions:?}"
        );
    }
    assert!(
        !keyed.is_empty(),
        "the witness must emit static-worker calls, or every clause below is vacuous"
    );

    // The independent side: the planner's own unit definitions and contexts.
    //
    // ⛔ Both read from a plan built separately from the emission run, so no
    // expectation below is an echo of the compile that produced the
    // observations.
    let worker_contracts = with_d5a_witness_plan(|plan| {
        let mut declared: BTreeMap<StaticOriginId, (u32, usize)> = BTreeMap::new();
        for unit in plan.continuation_units().expect("continuation units") {
            let previous = declared.insert(
                unit.worker_body_origin(),
                (unit.worker_declared_arity(), unit.worker_capture_count()),
            );
            assert!(
                previous.is_none() || previous == Some((unit.worker_declared_arity(), unit.worker_capture_count())),
                "two units declare DIFFERENT contracts for one worker body, so the expectation \
                 below would be choosing between them"
            );
        }
        declared
    });
    let contexts = with_d5a_witness_plan(|plan| {
        let mut declared = BTreeMap::new();
        for context in plan.continuation_contexts().expect("contexts") {
            let previous =
                declared.insert(context.worker_body_origin(), context.header().captures as usize);
            assert!(
                previous.is_none(),
                "two generated contexts for one raw worker body would make the expectation below \
                 ambiguous, and this row would be choosing between them"
            );
        }
        declared
    });

    // Clause 2 — the planner's expectation, under the SAME key, and the route
    // does not choose it.
    //
    // ⛔ The expected route is derived from whether the PLANNER declares a
    // generated context for the target this call names -- not read off the
    // emission's own route field. Letting the observed route pick its own branch
    // is how the raw arm became a tautology: `supplied == raw` is trivially true
    // for any call the emitter never appended to.
    //
    // ⚠ The TARGET is the actual one. This is the planner adjudicating the route
    // and contract for a named target, not the planner naming which target the
    // call should have reached -- see THE RESIDUAL in the header.
    let mut context_routed = 0usize;
    let mut raw_routed = 0usize;
    let mut emitted_callees: Vec<u32> = Vec::new();
    for ((function, call), (target, route, raw, supplied, captures, declared_arity, decoded)) in
        &keyed
    {
        // The emitted callee IDENTITY. ⛔ The declared `FuncRef` the route's own
        // table answered with -- what the instruction is written against -- and
        // NOT the target origin: the raw and generated-context routes SHARE a
        // worker body origin by design (`D6a`), so an origin recorded here is
        // identical on both routes and says nothing about which table answered.
        emitted_callees.push(*decoded);
        // The RAW CONTRACT, from unit definition rather than from the binding.
        //
        // ⛔ `declared_arity` and `captures` on the emission are the
        // `StaticWorkerBinding`'s own fields; comparing the raw run against them
        // is the binding agreeing with itself. The planner's unit population is
        // asked instead, keyed by the worker body this call targets.
        let (planned_arity, planned_captures) = *worker_contracts.get(target).unwrap_or_else(|| {
            panic!(
                "the call at {call:?} in {function:?} targets body {target:?}, which the PLANNER \
                 defines no unit for: {worker_contracts:?}"
            )
        });
        assert_eq!(
            (*declared_arity, *captures),
            (planned_arity, planned_captures),
            "the binding's declared contract must be the one unit definition states for that \
             worker body, under key ({function:?}, {call:?})"
        );
        assert_eq!(
            *raw,
            planned_arity as usize + planned_captures,
            "and the raw run is exactly that independently declared arity plus capture count, \
             under key ({function:?}, {call:?})"
        );
        let expected_suffix = contexts.get(target).copied();
        let expected_route = match expected_suffix {
            Some(_) => StaticWorkerCallRoute::GeneratedContext,
            None => StaticWorkerCallRoute::RawWorker,
        };
        assert_eq!(
            *route, expected_route,
            "the route must be the one the PLANNER's context population implies FOR THIS TARGET \
             -- derived from whether a context is declared for it, not read off the emission's \
             route field, under key ({function:?}, {call:?}). ⚠ The target itself is the actual \
             one: no authority names which target this call should reach"
        );
        match expected_suffix {
            Some(declared) => {
                assert!(
                    declared > 0,
                    "a context declaring zero captures makes the relation below hold trivially"
                );
                assert_eq!(
                    *supplied,
                    raw + declared,
                    "a context-routed call carries the exact raw run PLUS the capture run its \
                     generated context independently declares, under key ({function:?}, {call:?})"
                );
                context_routed += 1;
            }
            None => {
                assert_eq!(
                    supplied, raw,
                    "and a raw-routed call carries the raw run only -- compared against the \
                     planner's ABSENCE of a context for this target, not against itself, under \
                     key ({function:?}, {call:?})"
                );
                raw_routed += 1;
            }
        }
    }
    assert!(
        context_routed > 0 && raw_routed > 0,
        "the A/B witness must emit BOTH routes ({context_routed} context, {raw_routed} raw). With \
         one kind the row cannot tell 'the suffix is confined to the retargeted call' from 'a \
         suffix is appended everywhere' or from 'none ever is'"
    );
    assert_eq!(
        emitted_callees.iter().collect::<BTreeSet<_>>().len(),
        keyed.len(),
        "and each emission was written against a DISTINCT declared callee. Because the two routes \
         share a worker body origin, this identity is the only recorded fact that separates which \
         table answered: {emitted_callees:?}"
    );
}


/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8g` — the composed population's selected
/// recursive argument reaches its `D8b` target and the same emitter.**
///
/// The second of `D8g`'s two populations. It is a different program from the
/// functionized one and reaches the emitter by a different ingress; the join is
/// `call_static_worker_with_inputs`, where both write.
///
/// ## The independent side
///
/// The planner's own continuation-call and unit populations, from a plan built
/// separately from the emission run: which specialization the composed call
/// targets, and that target's worker body origin, declared arity and capture
/// count. Nothing is read from the binding, the emitted call, or the other
/// population's log.
///
/// ## Clause 1 — one emission per call occurrence per body
///
/// Explicit insertion that fails on a previous value, so a duplicated target or
/// operand assembly reds instead of overwriting.
///
/// ## Clause 2 — the composed discharge sits on the ordinary-unit body
///
/// Actual body kind is `D8o`'s supplied key, never an owner variant. The exact
/// selected recursive argument -- the one carrying a composed causal authority
/// -- is emitted from the ordinary unit body; the specialization's own binding
/// at the same call occurrence and the same target carries none. Both reach the
/// same emitter with the same decoded raw callee and the same operand run, so
/// the composed discharge is the only thing separating them, and it is exactly
/// what `D8i` made a separate facet from the route.
///
/// ## Clause 3 — target and operand run against the planner
///
/// The decoded raw callee must be the worker body the planner's own unit names
/// for the composed call's target, and the instruction's run must be that unit's
/// declared arity plus its capture count. Neither side is derived from the
/// other.
///
/// **Promise class: durable invariant.**
#[test]
fn d8g_the_composed_selected_argument_reaches_its_target_at_the_shared_emitter() {
    use crate::cranelift_backend::lowering::{
        d8g_emissions, d8o_body_keys, reset_d8g_emissions, reset_d8n_observations,
        reset_d8o_body_authorities, D8oBodyKey,
    };

    reset_d8g_emissions();
    reset_d8n_observations();
    reset_d8o_body_authorities();
    let outcome = d8f_compile(false);
    assert!(
        outcome.is_none(),
        "the composed witness must compile, or the emitter is never reached: {outcome:?}"
    );
    let keys = d8o_body_keys()
        .into_iter()
        .map(|(function, key)| (function.expect("body keys are labelled"), key))
        .collect::<BTreeMap<_, _>>();

    // Clause 1 — one emission per (exact defining body, call occurrence).
    let mut keyed = BTreeMap::new();
    for emission in d8g_emissions() {
        let function = emission
            .function
            .expect("every static-worker call names its defining Function");
        let key = *keys
            .get(&function)
            .expect("every emitting body recorded its exact body key");
        let previous = keyed.insert(
            (key, emission.call_origin),
            (
                emission.target_body_origin,
                emission.declared_arity,
                emission.captures,
                emission.supplied_operands,
                emission.composed_discharge,
                emission.emitted_callee,
            ),
        );
        assert!(
            previous.is_none(),
            "one call occurrence emits at most ONE static-worker call in one exact defining body. \
             A second is a duplicated target or operand assembly, and overwriting would hide it"
        );
    }
    assert_eq!(
        keyed.len(),
        2,
        "the composed witness's selected recursive argument is emitted from TWO bodies -- the \
         ordinary unit body and the specialization derived from the same source text. One means \
         the population is not the composed one at all: {keyed:?}"
    );

    // The independent side: the planner's own call target and unit contract.
    let declaration = d8f_declaration_with(false, D8fPerturbation::None);
    let entry = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::DeclarationRef {
            symbol: D8F_SYMBOL.to_string(),
        }),
        args: vec![RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        }],
    };
    let plan = plan_static_transition_graph_with_symbols(
        &entry,
        &BTreeMap::from([(D8F_SYMBOL, &declaration)]),
        &crate::NativeProcessSymbols::legacy_prelude(),
        AbiRootIngress::Value,
        true,
    )
    .expect("the composed witness plans");
    // The exact D8b call, by FULL causal coordinate -- not `.first()`, and not
    // by observed-set order. The coordinate is the D8a selector's four source
    // fields; the emission owner is deliberately not among them, because D8g's
    // own measurement at 1b367065 established it separates nothing.
    let calls = plan.continuation_calls().expect("continuation calls");
    let coordinate = |call: &ContinuationCallView<'_>| {
        (
            call.producer_construct_origin(),
            call.continuation_origin(),
            call.producer_alternative(),
            call.recursive_position(),
        )
    };
    let coordinates = calls.iter().map(coordinate).collect::<BTreeSet<_>>();
    assert_eq!(
        coordinates.len(),
        calls.len(),
        "the planner's composed calls must be distinct under their causal coordinate, or naming \
         one by coordinate is ambiguous"
    );
    // ALL FOUR coordinate fields, named before any selection.
    //
    // The two source-authored halves come off the WITNESS's own text: the
    // eliminator case whose body is a `CheckedSubcontinuationFrame`, and that
    // case's declared recursive position. ⚠ For this `D8m`-derived Wrap case
    // the recursive position is **1**, not 0 -- the worker sits at field 1 and
    // the selected field at 0. An earlier comment here said 0; the value was
    // always read from the case, so only the prose was wrong.
    //
    // ⛔ Uniqueness is PROVED, not assumed: exactly one case carries a bridge
    // and that case declares exactly one recursive position. No `.position` or
    // `.first` order surrogate decides either.
    //
    // The two origin halves come from the planner's own unit population -- a
    // different population from the calls being selected -- and the match below
    // is on the COMPLETE tuple.
    let (source_alternative, source_recursive_position) = {
        let RuntimeDeclarationKind::Transparent { body } = &declaration.kind else {
            panic!("transparent")
        };
        let RuntimeExpr::Closure { body, .. } = body else {
            panic!("closure")
        };
        let RuntimeExpr::Let { value, .. } = body.as_ref() else {
            panic!("let")
        };
        let RuntimeExpr::ComputationalMatch { cases, .. } = value.as_ref() else {
            panic!("eliminator")
        };
        let bridges = cases
            .iter()
            .enumerate()
            .filter(|(_, case)| {
                matches!(case.body, RuntimeExpr::CheckedSubcontinuationFrame { .. })
            })
            .collect::<Vec<_>>();
        assert_eq!(
            bridges.len(),
            1,
            "the witness must spell exactly ONE composed bridge case, or naming the alternative \
             from the source is ambiguous and any pick is an order surrogate"
        );
        let (alternative, case) = bridges[0];
        assert_eq!(
            case.recursive_positions.len(),
            1,
            "and that case must declare exactly ONE recursive position, for the same reason"
        );
        (alternative as u32, case.recursive_positions[0] as u32)
    };
    // The origin halves, from the planner's UNIT population.
    let (source_construct_origin, source_continuation_origin) = {
        let units = plan.continuation_units().expect("continuation units");
        let named = units
            .iter()
            .filter(|unit| {
                unit.producer_alternative() == source_alternative
                    && unit.recursive_position() == source_recursive_position
            })
            .map(|unit| (unit.producer_construct_origin(), unit.continuation_origin()))
            .collect::<BTreeSet<_>>();
        assert_eq!(
            named.len(),
            1,
            "exactly one planned UNIT carries the source-named alternative and recursive \
             position, so its construct and continuation origins name the coordinate's other two \
             fields without consulting the call population being selected"
        );
        *named.iter().next().expect("one")
    };
    let named_coordinate = (
        source_construct_origin,
        source_continuation_origin,
        source_alternative,
        source_recursive_position,
    );
    let matching = calls
        .iter()
        .filter(|call| coordinate(call) == named_coordinate)
        .collect::<Vec<_>>();
    assert_eq!(
        matching.len(),
        1,
        "exactly ONE planned call carries the COMPLETE independently named coordinate \
         {named_coordinate:?}. More would make the naming ambiguous; none would mean the planner \
         issued no call for the bridge the witness spells"
    );
    let call = matching[0];
    let units = plan.continuation_units().expect("continuation units");
    let target = units
        .iter()
        .find(|unit| unit.id() == call.target())
        .expect("the planner defines the unit its composed call targets");
    let planned = (
        target.worker_body_origin(),
        target.worker_declared_arity(),
        target.worker_capture_count(),
    );

    // Clause 2 — the composed discharge sits on the ordinary-unit body.
    let composed = keyed
        .iter()
        .filter(|(_, (_, _, _, _, discharge, _))| *discharge)
        .map(|(key, _)| *key)
        .collect::<Vec<_>>();
    assert_eq!(
        composed.len(),
        1,
        "exactly ONE of the two emissions may answer for a composed causal obligation: {keyed:?}"
    );
    assert!(
        matches!(composed[0].0, D8oBodyKey::OrdinaryUnit(_)),
        "and it is the ORDINARY UNIT body's, classified by D8o's supplied body key and never by \
         an owner variant. The specialization's binding at the same occurrence and the same \
         target carries none, which is why D8i made the discharge a facet separate from the \
         route: {composed:?}"
    );
    let specialization = keyed
        .iter()
        .find(|(key, _)| matches!(key.0, D8oBodyKey::ContinuationSpecialization(_)))
        .expect("the specialization body also emits here");
    assert!(
        !specialization.1 .4,
        "the specialization's own binding answers for no composed obligation: {keyed:?}"
    );
    assert_eq!(
        composed[0].1, specialization.0 .1,
        "and both reach the emitter at the SAME call occurrence, so the composed discharge is the \
         only thing separating them"
    );

    // The application occurrence, from independent source/planner child-origin
    // authority -- the same walk D8f uses, not from an observation.
    // From the composed call's own continuation origin: case 0's body (the
    // `CheckedSubcontinuationFrame`), the bridge match it wraps, that match's
    // case 0 body (the slot marker), the slot marker's body (the invocation
    // marker), and the marker's body (the application itself).
    let named_application = {
        let mut cursor = source_continuation_origin;
        for step in [1usize, 0, 1, 0, 0] {
            cursor = plan
                .child_static_origin(cursor, step)
                .expect("the planner names this child occurrence");
        }
        cursor
    };
    let emitted_at = keyed.keys().map(|(_, call)| *call).collect::<BTreeSet<_>>();
    assert_eq!(
        emitted_at,
        BTreeSet::from([named_application]),
        "both emitted keys must sit at the ONE application occurrence the planner's child-origin \
         walk names for this composed call. A different occurrence means the emission is not the \
         one this coordinate is about: {keyed:?}"
    );

    // Clause 3 — target, emitted callee identity and operand run.
    let mut emitted_callees: Vec<u32> = Vec::new();
    for (key, (target_body, declared_arity, captures, supplied, _, decoded)) in &keyed {
        // The emitted callee IDENTITY. ⛔ The declared `FuncRef` the route's own
        // table answered with -- what the instruction is written against -- and
        // NOT the target origin: the raw and generated-context routes SHARE a
        // worker body origin by design (`D6a`), so an origin recorded here is
        // identical on both routes and says nothing about which table answered.
        emitted_callees.push(*decoded);
        assert_eq!(
            (*target_body, *declared_arity, *captures),
            planned,
            "the decoded raw callee and its declared contract must be the ones the PLANNER names \
             for the composed call's target unit, under key {key:?}"
        );
        assert_eq!(
            *supplied,
            planned.1 as usize + planned.2,
            "and the instruction's operand run must be that unit's declared arity plus its \
             capture count -- read off the emission, compared with the planner, neither derived \
             from the other, under key {key:?}"
        );
    }
    assert_eq!(
        emitted_callees.iter().collect::<BTreeSet<_>>().len(),
        keyed.len(),
        "MEASURED: each defining body emits against its OWN declared callee, even though both \
         reach the same planner target unit -- the `FuncRef` is function-local, so one target \
         reached from two bodies is two declared refs. The shared fact is the target and the \
         operand contract, checked above; the callee identity is per-body: {emitted_callees:?}"
    );
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8g` — the two producer-input mutations, each
/// caught by the first production guard that owns the input it moved.**
///
/// ## Two proof planes, and this row is the second
///
/// The positive plane is post-emission: the keyed relation in
/// `d8g_the_functionized_population_binds_its_table_and_suffix_at_the_shared_emitter`.
/// This row is the **mutation plane**, and its refusals are **pre-emission**.
/// That is not a weaker result and it is not described as a relation failure.
///
/// Both mutations move an input that must be valid **before** an instruction
/// exists, so the first guard that owns that input refuses and no call is ever
/// written. Measured: the `D8g` emission set is **empty** in both. An earlier
/// fail-closed guard is stronger than arranging a bad-but-emittable call so a
/// downstream observer can reject it — and arranging one would mean declaring
/// the other table's target into the function, which is the route/target
/// authority expansion this checkpoint bans. Architect ruling, `evt_5vwdtrznf3km4`.
///
/// ## What each row asserts
///
/// The mutation **fired** (a control that silently never applied is a green
/// proving the opposite of its claim); the **exact named refusal**, by category
/// and discriminating reason, not `is_err()`; and **zero post-emission records**
/// for that compilation. Only the compilation under test is run — not the
/// positive row with its setup `.expect`s, whose panic would be indistinguishable
/// from the refusal being measured.
///
/// **Promise class: durable invariant.**
#[test]
fn d8g_each_producer_input_mutation_is_caught_by_the_guard_that_owns_it() {
    use crate::cranelift_backend::lowering::{
        d8g_emissions, reset_d8g_emissions, with_d8g_mutation, D8gMutation,
    };

    let compile = || {
        crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
            "ken_d8g_mutation",
            false,
        )
        .map(|_| ())
    };

    for (mutation, guard) in [
        // The table lookup owns which table answers.
        (D8gMutation::WrongTable, "no raw_worker_calls target for body origin"),
        // Callee-frame input coverage owns the operand run.
        (
            D8gMutation::WithholdContextSuffix,
            "callee frame is missing a declared input",
        ),
    ] {
        reset_d8g_emissions();
        let (outcome, applications) = with_d8g_mutation(mutation, compile);
        assert!(
            applications > 0,
            "the {mutation:?} mutation must have FIRED. Zero applications means the switch never \
             reached its site, and any refusal below is about something else"
        );
        let error = outcome.expect_err(&format!(
            "{mutation:?} moves an input a production guard owns, so the compile must refuse"
        ));
        // The refusal is matched EXHAUSTIVELY on the typed error, so the
        // category is asserted as well as the reason. A `contains` over a
        // formatted string would accept the right words carried by the wrong
        // error kind.
        let reason = match (&error, mutation) {
            (
                CraneliftBackendError::Unsupported(UnsupportedLowering { construct, reason }),
                D8gMutation::WrongTable,
            ) => {
                assert_eq!(
                    *construct, "Call",
                    "the wrong-table refusal is the CALL construct's own: {error:?}"
                );
                reason.clone()
            }
            (
                CraneliftBackendError::Backend(BackendFailure::Module(reason)),
                D8gMutation::WithholdContextSuffix,
            ) => reason.clone(),
            _ => panic!(
                "{mutation:?} must be caught by its own guard's error CATEGORY, not merely by \
                 some error: {error:?}"
            ),
        };
        assert!(
            reason.contains(guard),
            "and by its discriminating reason -- not merely the right category. A different \
             reason means the mutation is being caught by something that does not own what it \
             moved: {reason}"
        );
        assert!(
            d8g_emissions().is_empty(),
            "and NO instruction was emitted: the guard is pre-emission, which is why this plane \
             is not the post-emission relation and is not described as one. A non-empty set here \
             would mean an invalid call was allowed to exist: {:?}",
            d8g_emissions()
        );
    }

    // The positive plane, rerun exactly, with the switch restored -- which also
    // shows the scoped RAII survived both refusals above.
    reset_d8g_emissions();
    let (outcome, applications) = with_d8g_mutation(D8gMutation::Exact, compile);
    assert_eq!(
        applications, 0,
        "with no mutation armed the switch must not apply"
    );
    outcome.expect("and the exact witness compiles");
    d8g_the_functionized_population_binds_its_table_and_suffix_at_the_shared_emitter();
}


/// **`RT-CONTSRC-PRODUCER-LOCAL` `D6b` — the mixed pair sits over ONE worker
/// body, and only where a retarget happened do the two tables disagree there.**
///
/// ## What this row is about, and why nothing else could see it
///
/// Clauses 3 and 4 of the asymmetric route law both turn on one fact: *did this
/// body actually get retargeted, and can each of its two declared call tables
/// answer for the worker body it selected?* Until this row, the only evidence
/// for that pair was the **text of a refusal message** — `D8g`'s `WrongTable`
/// mutation reds with *"no `raw_worker_calls` target for body origin"*, and the
/// frame drew the population conclusion from it. ⛔ A refusal message is an
/// inference, not a measurement; this row measures the tables directly, at the
/// body-definition seat, with no mutation armed.
///
/// ## The independent side
///
/// The planner's own populations, from a plan built separately from the emission
/// run: the continuation-context population says which worker bodies have a
/// generated execution context, and the emittable/executable unit populations say
/// which bodies have a declared `Function` at all. ⛔ Nothing below is derived
/// from the retarget's own outcome, from the tables, or from the other's absence.
///
/// ## Clause 1 — the retarget agrees with the planner
///
/// A body is retargeted exactly when the planner declares a context for the
/// worker body it selected. Both directions, so neither a missed retarget nor a
/// spurious one passes.
///
/// ## Clause 2 — the mixed pair is over ONE body
///
/// A retargeted specialization installs exactly two static-worker members: the
/// induction hypothesis, routed `GeneratedContext`, and the selected recursive
/// argument, routed `RawWorker` — **both naming the same worker body origin**.
/// ⭐ That single-origin fact is what makes the pair `D6a`'s subject rather than
/// two unrelated workers that happen to differ in route, and it is why the route
/// cannot be recovered from the origin.
///
/// A body that resolved no context installs the same two members over the same
/// one origin with **both** routes `RawWorker` — lawful and route-degenerate.
///
/// ## Clause 3 — the tables, and the asymmetry that is the law's precondition
///
/// Where a retarget happened, `worker_calls` answers for the selected body and
/// `raw_worker_calls` does **not**. Where none happened, **both** answer.
/// Clause 4 of the law — *"a table swap is observational identity, not a missing
/// negative"* — is exactly the second case, and this row is what makes that a
/// measured precondition instead of an assumption carried in prose.
///
/// ## Clause 4 — WHY the raw table cannot answer, taken from the planner
///
/// The retargeted body is in the planner's **emittable** population and absent
/// from its **executable** one: it has a descriptor and no `Function`. ⛔ Read
/// from the plan, not from the table's absence, so the two are independent
/// statements of one fact rather than one fact restated.
///
/// ## MEASURED / CLAIMED / THE GAP
///
/// **MEASURED:** the retarget outcome, both tables' answerable sets, and the
/// route and body origin of every static-worker member each specialization body
/// installed.
///
/// **CLAIMED:** that the asymmetric law's precondition holds as stated — the
/// tables diverge exactly at a retargeted body and nowhere else.
///
/// **THE GAP:** this row observes which body origins each table *can answer
/// for*. It does not observe a call, so it says nothing about which table a given
/// emission read; that is
/// [`d8g_the_functionized_population_binds_its_table_and_suffix_at_the_shared_emitter`]'s
/// subject, at the emitter.
///
/// ⛔ **An evasion that survives every clause here, stated because it exists:** a
/// defect that kept both key sets exactly as measured and changed the `FuncRef`
/// one entry holds -- right key, wrong function -- is invisible to this row. It
/// is caught, if at all, by the emitted-callee identity at the emitter, and it is
/// the same family as `D8g`'s stated residual. ⇒ This row owns which bodies each
/// table ANSWERS FOR; it does not own what it answers WITH.
///
/// ## THE PERMANENT ATTRIBUTION LIMIT this row measures
///
/// **Raw emissions occur only where `worker_calls` and `raw_worker_calls`
/// lawfully name the SAME declared callee. Where the two tables differ, no
/// lawful raw call emits at all.** Both halves are measured below, and together
/// they mean raw-table attribution is unprovable — not for want of
/// instrumentation, but because the representation encodes no distinction there.
/// That is clause 4 of the asymmetric law: at an equal-table seat a swap is
/// observational identity.
///
/// ⛔ **This is NOT a transition sentinel, and must never be relabelled as one.**
/// A sentinel names the obligation that clears it; this has none. The only known
/// mechanism — retaining the raw body as a declared-and-defined `Function` — is
/// **banned on measurement**: it defines a standalone `Function` whose result is
/// a `Constructor` containing a raw `Closure`, and it reopens the permanent
/// unit-result closure boundary the generated-context design exists to avoid
/// (`741/2` unarmed against `716/27` armed, causally isolated to one retention
/// predicate; Architect `evt_3dcafs581921e` Finding 2). Calling it pending would
/// leave a reader waiting for a checkpoint that will never be cut.
///
/// ## Promise class
///
/// **Durable invariant.** Every clause is a relation over the retarget split —
/// which side a body falls on, what each table answers for there, and the routes
/// and origins of the members installed. A fixture that grows bodies or calls
/// keeps it green. The raw-table clause is durable for the same reason as the
/// rest: it states a property of the representation, not a stage of it.
#[test]
fn d6b_the_mixed_pair_is_over_one_body_and_only_a_retarget_makes_the_two_tables_disagree() {
    use crate::cranelift_backend::lowering::{
        d6b_specialization_bodies, reset_d6b_specialization_bodies,
    };

    reset_d6b_specialization_bodies();
    crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
        "ken_d6b_tables",
        false,
    )
    .map(|_| ())
    .map_err(|error| format!("{error:?}"))
    .expect("the mixed A/B witness compiles");
    let bodies = d6b_specialization_bodies();
    assert!(
        !bodies.is_empty(),
        "no specialization body was defined, so every clause below would run vacuously"
    );

    // The independent side. Both read a plan built separately from the emission
    // run above, so no expectation here is an echo of that compile.
    let planned_contexts = with_d5a_witness_plan(|plan| {
        plan.continuation_contexts()
            .expect("contexts")
            .into_iter()
            .map(|context| context.worker_body_origin())
            .collect::<BTreeSet<_>>()
    });
    let (emittable, executable) = with_d5a_witness_plan(|plan| {
        (
            plan.emittable_units()
                .expect("emittable units")
                .iter()
                .map(|unit| unit.body_occurrence())
                .collect::<BTreeSet<_>>(),
            plan.executable_units()
                .expect("executable units")
                .iter()
                .map(|unit| unit.body_occurrence())
                .collect::<BTreeSet<_>>(),
        )
    });

    let mut retargeted_bodies = 0usize;
    let mut plain_bodies = 0usize;
    for body in &bodies {
        let selected = body.worker_body_origin;

        // Clause 1 — the retarget outcome against the planner, both directions.
        assert_eq!(
            body.retargeted.is_some(),
            planned_contexts.contains(&selected),
            "a body is retargeted exactly when the PLANNER declares a generated context for the \
             worker body it selected. Asserted both ways, so neither a missed retarget nor a \
             spurious one passes: {body:?}"
        );
        if let Some(retargeted) = body.retargeted {
            assert_eq!(
                retargeted, selected,
                "and the retarget names the body this specialization selected, never another: \
                 {body:?}"
            );
        }

        // Clause 2 — two members, one origin, and the routes the law states.
        assert_eq!(
            body.members.len(),
            2,
            "a specialization with one recursive constructor argument installs exactly two \
             static-worker members: the induction hypothesis and the selected recursive argument. \
             One means the argument position was skipped -- the pre-`D6a` defect: {body:?}"
        );
        assert!(
            body.members.iter().all(|(_, _, origin)| *origin == selected),
            "and BOTH members name the worker body this specialization selected. Two members over \
             two different origins would not be `D6a`'s pair at all, and the route below would be \
             recoverable from the origin rather than a separate fact: {body:?}"
        );
        let routes = body
            .members
            .iter()
            .map(|(_, route, _)| *route)
            .collect::<Vec<_>>();
        let positions = body
            .members
            .iter()
            .map(|(position, _, _)| *position)
            .collect::<Vec<_>>();
        assert!(
            positions[0] < positions[1],
            "the members are recorded in binder-run order, so the induction hypothesis precedes \
             the selected recursive argument: {body:?}"
        );

        if body.retargeted.is_some() {
            retargeted_bodies += 1;
            assert_eq!(
                routes,
                vec![
                    StaticWorkerCallRoute::GeneratedContext,
                    StaticWorkerCallRoute::RawWorker,
                ],
                "THE MIXED PAIR: where a context was resolved, the induction hypothesis routes to \
                 it and the selected recursive argument stays raw -- over the one body origin \
                 asserted above. Equal routes here would mean the retarget moved both members, or \
                 that one binding was reused for both: {body:?}"
            );

            // Clause 3 — the asymmetry, measured on both tables.
            assert!(
                body.worker_call_targets.contains(&selected),
                "`worker_calls` must answer for the retargeted body: the retarget inserts the \
                 generated context under exactly this origin: {body:?}"
            );
            // ⛔ THE PERMANENT LIMIT, not a pending obligation. Where the tables
            // differ, the raw route has no callee -- so no lawful raw call can
            // emit here, and raw-table attribution has no seat where it could be
            // observed. The only mechanism that would change this retains the
            // raw body as a declared-and-defined `Function`, which is banned on
            // measurement: it reopens the unit-result closure boundary.
            assert!(
                !body.raw_worker_call_targets.contains(&selected),
                "THE PERMANENT ATTRIBUTION LIMIT -- `raw_worker_calls` does not answer for a \
                 retargeted body, so no lawful raw call emits where the two tables differ. ⛔ Not \
                 a pending obligation and not a sentinel: the only mechanism that would make this \
                 answerable retains the raw body as a declared-and-defined `Function`, which \
                 reopens the permanent unit-result closure boundary and is banned on measurement: \
                 {body:?}"
            );

            // Clause 4 — WHY, from the planner rather than from the absence.
            assert!(
                emittable.contains(&selected),
                "the retargeted body keeps its DESCRIPTOR -- it is still emittable, which is what \
                 lets the static-worker constructor validate against its raw contract: {body:?}"
            );
            assert!(
                !executable.contains(&selected),
                "and it has no `Function`: the planner's executable population excludes it once it \
                 is fully retargeted. ⭐ THIS is why the raw table cannot answer above, stated \
                 from the PLAN rather than inferred from the table's own absence: {body:?}"
            );
        } else {
            plain_bodies += 1;
            assert_eq!(
                routes,
                vec![
                    StaticWorkerCallRoute::RawWorker,
                    StaticWorkerCallRoute::RawWorker,
                ],
                "where the planner issued no context, BOTH members lawfully carry the raw route \
                 and are separated by their run positions alone: {body:?}"
            );
            assert!(
                body.worker_call_targets.contains(&selected)
                    && body.raw_worker_call_targets.contains(&selected),
                "CLAUSE 4's PRECONDITION, measured: with no retarget both tables answer for this \
                 body, so a table swap here is observational identity and NOT a missing negative. \
                 This is the fact the retired symmetric-mirror requirement assumed away: {body:?}"
            );
        }
    }

    assert!(
        retargeted_bodies > 0 && plain_bodies > 0,
        "the witness must reach BOTH kinds ({retargeted_bodies} retargeted, {plain_bodies} not). \
         With one kind this row cannot tell 'the tables diverge exactly at a retarget' from 'they \
         always diverge' or from 'they never do': {bodies:?}"
    );
}



/// **`RT-CONTSRC-PRODUCER-LOCAL` `D6b` — the asymmetric law across BOTH planes:
/// every raw emission sits where the two tables agree, and every context
/// emission sits where they differ.**
///
/// ## What this closes
///
/// [`d6b_the_mixed_pair_is_over_one_body_and_only_a_retarget_makes_the_two_tables_disagree`]
/// measures the tables at the body-definition seat.
/// [`d8g_the_functionized_population_binds_its_table_and_suffix_at_the_shared_emitter`]
/// measures the calls at the emitter. Neither can say **which emissions land on
/// which side of the retarget split** — and that is the fact the asymmetric law's
/// clauses 3 and 4 are *about*. This row joins them on body origin and asserts
/// exactly that correspondence.
///
/// ## ⛔ Both sides are OBSERVATIONS, and this row does not pretend otherwise
///
/// The table record and the emission log are two seats of the **same** compile.
/// This is therefore a **consistency law between two planes**, not an independent
/// derivation of either. What is genuinely independent is the expectation for
/// which side a body falls on: the planner's own continuation-context population,
/// read from a plan built separately from the emission run.
///
/// The join itself is the already-governed one — emitting `FuncId` to `D8o`'s
/// body authority to the specialization identity — so no new key is minted here.
///
/// ## The consequence worth writing down — THE PERMANENT ATTRIBUTION LIMIT
///
/// Where the two tables hold the **same** entry, *"this emission resolved through
/// `raw_worker_calls`"* is not an observable fact: the other table would answer
/// identically. ⇒ **Raw-table attribution is unprovable**, and this row is what
/// stops a later reader assuming it was proved.
///
/// The limit is **structural, not fixture-local**, and both halves are asserted
/// below: raw emissions occur only where the two tables lawfully name the same
/// declared callee, and where they differ no lawful raw call emits at all.
///
/// ⛔ **Nothing retires it.** The only mechanism that would — retaining the raw
/// body as a declared-and-defined `Function` — is banned on measurement, because
/// it reopens the permanent unit-result closure boundary. So this is not a
/// coverage hole and not a pending obligation: it is clause 4 of the law, an
/// inert table swap at an equal-table seat being the absence of a distinction
/// rather than a missing negative.
///
/// **Promise class: durable invariant.** The assertion is a correspondence
/// between two planes keyed on body origin, with the planner supplying the side.
/// A fixture that adds bodies or calls keeps it green; only a change to which
/// side of the retarget split an emission lands on reds it.
#[test]
fn d6b_every_raw_emission_sits_where_the_tables_agree_and_every_context_emission_where_they_differ()
{
    use crate::cranelift_backend::lowering::{
        d6b_specialization_bodies, d8g_emissions, d8o_body_keys, reset_d6b_specialization_bodies,
        reset_d8g_emissions, reset_d8o_body_authorities, D8oBodyKey,
    };

    reset_d6b_specialization_bodies();
    reset_d8g_emissions();
    reset_d8o_body_authorities();
    crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
        "ken_d6b_two_planes",
        false,
    )
    .map(|_| ())
    .map_err(|error| format!("{error:?}"))
    .expect("the mixed A/B witness compiles");

    // The independent side: which worker bodies the PLANNER gives a generated
    // execution context, from a plan built separately from the run above.
    let planned_contexts = with_d5a_witness_plan(|plan| {
        plan.continuation_contexts()
            .expect("contexts")
            .into_iter()
            .map(|context| context.worker_body_origin())
            .collect::<BTreeSet<_>>()
    });

    // The governed join: emitting `FuncId` -> `D8o` body authority -> the
    // specialization identity the table record is keyed by.
    let mut body_of_function = BTreeMap::new();
    for (function, key) in d8o_body_keys() {
        if let (Some(function), D8oBodyKey::ContinuationSpecialization(unit)) = (function, key) {
            body_of_function.insert(function, unit);
        }
    }
    let mut tables_of_unit = BTreeMap::new();
    for body in d6b_specialization_bodies() {
        tables_of_unit.insert(body.unit, body);
    }

    let mut agreed = 0usize;
    let mut differed = 0usize;
    for emission in d8g_emissions() {
        let function = emission
            .function
            .expect("every static-worker call names its defining Function");
        let Some(unit) = body_of_function.get(&function) else {
            // Emissions from ordinary-unit and generated-context bodies are a
            // different plane; this row is about the specialization seat, and
            // skipping them is deliberate rather than a silent narrowing.
            continue;
        };
        let tables = tables_of_unit
            .get(unit)
            .unwrap_or_else(|| panic!("{unit:?} emitted a call but recorded no table state"));
        let target = emission.target_body_origin;
        let in_worker = tables.worker_call_targets.contains(&target);
        let in_raw = tables.raw_worker_call_targets.contains(&target);

        // Which side the PLANNER says this target falls on.
        match planned_contexts.contains(&target) {
            true => {
                assert_eq!(
                    emission.route,
                    StaticWorkerCallRoute::GeneratedContext,
                    "the planner declares a context for {target:?}, so a call reaching it must be \
                     context-routed: {emission:?}"
                );
                assert!(
                    in_worker && !in_raw,
                    "CLAUSE 3's SEAT: at a target the planner gave a context, the two tables must \
                     genuinely differ -- `worker_calls` answers and `raw_worker_calls` does not. \
                     This is what makes the existing wrong-table negative a live discriminator \
                     here rather than an inert swap: {tables:?}"
                );
                differed += 1;
            }
            false => {
                assert_eq!(
                    emission.route,
                    StaticWorkerCallRoute::RawWorker,
                    "the planner declares no context for {target:?}, so a call reaching it must be \
                     raw-routed: {emission:?}"
                );
                assert!(
                    in_worker && in_raw,
                    "CLAUSE 4's SEAT: at a target with no context BOTH tables answer, so a table \
                     swap at this emission is observational identity. ⛔ It follows that \
                     'this call resolved through `raw_worker_calls`' is NOT observable here -- the \
                     other table holds the same entry. No row may claim raw-table attribution at \
                     this seat: {tables:?}"
                );
                assert_eq!(
                    emission.supplied_operands, emission.raw_operands,
                    "and the raw run carries no suffix, which IS observable and is the half of \
                     clause 1 this seat can prove: {emission:?}"
                );
                agreed += 1;
            }
        }
    }

    assert!(
        agreed > 0 && differed > 0,
        "the witness must emit on BOTH sides of the retarget split ({agreed} where the tables \
         agree, {differed} where they differ). With one side this row cannot tell the \
         correspondence from a constant"
    );
}


/// **`RT-CONTSRC-PRODUCER-LOCAL` `D6c` — the pre-emission SELECTION refusal set:
/// five perturbations at the selection seam, each refused by the guard that owns
/// it.**
///
/// ⛔ **NOT `D8f`'s refusal set.** `D8f` is about which call consumes a pending
/// checked-IH marker. This is about **selecting the raw/IH target and its
/// `SelectedRecursiveArgument` member**, before any instruction exists. The two
/// share vocabulary and share nothing else; discharging one discharges none of
/// the other.
///
/// ## The plane
///
/// Entirely the **refusal** plane, per `TWO PROOF PLANES`. Each row below owes
/// four things and this control asserts all four: the mutation **fired**
/// (application count), the **typed** error matched by category **plus** a
/// discriminating reason, **zero** post-emission records, and the exact positive
/// rerun separately. ⛔ No independently enumerated expectation population is
/// built, and no bad-but-emittable call is arranged — an earlier fail-closed
/// guard is stronger, not weaker.
///
/// ## The five, and the guard that owns each
///
/// ⚠ **Four of the five are retained with their original owner. One migrated.**
/// `WrongSourcePosition` is now refused by `D6c`'s sealed-run postcondition,
/// which reaches it first and is the better owner — the perturbation makes the
/// RUN name a wrong source position. Its downstream check still exists and still
/// fails closed; it is simply no longer first. The other four are unchanged.
///
/// | moved input | owning guard |
/// |---|---|
/// | wrong source position | the binder run's own position check, against the unit's ruled recursive position |
/// | fabricated availability | the IH prefix's unprojected-position hard stop |
/// | wrong closure/body | the static-worker constructor's raw-template lookup |
/// | wrong capture run | the constructor's declared-versus-projected capture count |
/// | raw/IH cross-routing | the route's own table, which has no raw callee for a retargeted body |
///
/// ⭐ **Each mutation moves the smallest thing its law is about**, and leaves the
/// constructor, the hypothesis and the rest of the run untouched, so a refusal is
/// attributable to that perturbation and not to a rewritten resolver.
///
/// ⛔ **Not all of them are single inputs, and saying so would be false.**
/// `CrossRouteTargets` is a **paired exchange** — the hypothesis takes the raw
/// route and the argument takes the context route **together**, because a law
/// about crossing two routes cannot be violated by moving only one of them.
/// **The other four of this row's five selection perturbations each move exactly
/// one producer input.**
///
/// ⚠ **`WrongOrder` is not among them and is not in this row's count of five.**
/// It is a **sibling segment permutation**, proved in
/// [`d6c_the_sealed_binder_run_refuses_a_miscounted_or_permuted_run_at_its_producer`],
/// and it moves no input at all — it reorders whole segments of an otherwise
/// exact run.
///
/// ## Two mutations that DECLINE rather than lie — and they decline for DIFFERENT reasons
///
/// ⛔ The two conditions are distinct and must not be stated as one:
///
/// - **`WrongCaptureRun`** declines when there is **no capture operand to drop
///   and none to borrow** — a unit with an empty worker-capture segment and no
///   ordinary operand available. The perturbation would leave the vector
///   unchanged, so it is not performed.
/// - **`CrossRouteTargets`** declines when **no generated context was resolved**.
///   On a route-degenerate unit both members lawfully carry the raw route, so
///   there is no crossing to make; the condition is about the retarget's outcome
///   and has nothing to do with captures.
///
/// In both cases the arm leaves the seam identical and **counts no application**.
/// A counter that ticked for a perturbation it did not perform would let this row
/// read a green as a defence when the mutation never happened — which is exactly
/// what the first version of these two arms did.
///
/// ## The other three refusals live in the sibling row
///
/// Omission, duplicate and wrong order are about the run's **shape** rather than
/// about an argument handed to the constructor, so they are owned by the sealed
/// binder run's own postcondition and proved in
/// [`d6c_the_sealed_binder_run_refuses_a_miscounted_or_permuted_run_at_its_producer`].
/// ⛔ **They were NOT all measured as unguarded, and the sentence that once said
/// so here was wrong.** Only **omission** and **duplicate** were genuinely
/// unguarded. **`WrongOrder` was producer-accepted and then downstream-refused**
/// on the **non-degenerate** `px8tr`, where the two members carry distinct
/// routes, while on the governed witness it is **equal-value identity** — so its
/// clean compile there was never evidence about any guard. ⇒ **The sealed
/// postcondition now owns all three canonical run-shape violations**, and it was
/// built on those measurements as they actually stand.
///
/// **Promise class: durable invariant.** Each clause is a typed refusal keyed to
/// the input it perturbs. A fixture that grows keeps it green; only a guard
/// ceasing to own its input reds it, which is a contract decision.
#[test]
fn d6c_each_moved_selection_input_is_refused_by_the_guard_that_owns_it_before_emission() {
    use crate::cranelift_backend::lowering::{
        d8g_emissions, reset_d8g_emissions, with_d6c_selection_mutation, D6cSelectionMutation,
    };
    use crate::cranelift_backend::surface::{BackendFailure, CraneliftBackendError};

    /// `(mutation, typed category, the discriminating fact the reason must name)`
    ///
    /// ⛔ The category comes from the error's own typed shape, never from a
    /// substring of the whole formatted value. The reason fragment is what makes
    /// the match discriminating: without it, any refusal of the same category
    /// would satisfy the row.
    const OWNED: &[(D6cSelectionMutation, &str, &str)] = &[
        // ⚠ **THIS EXPECTATION MOVED when `D6c`'s sealed-run postcondition
        // landed, and the move is an improvement rather than a regression.** It
        // used to name the case environment's own position check. The sealed run
        // now refuses first, and it is the better owner: the perturbation makes
        // the RUN name a wrong source position, and run shape is exactly what
        // the postcondition owns. The downstream check still exists and still
        // fails closed; it is simply no longer first.
        (
            D6cSelectionMutation::WrongSourcePosition,
            "Module",
            "at the slot belonging to source position",
        ),
        (
            D6cSelectionMutation::FabricatedAvailability,
            "Module",
            "that the continuation specialization projects no worker for",
        ),
        (
            D6cSelectionMutation::WrongClosureBody,
            "StaticWorkerBinding",
            "no raw worker template for body origin",
        ),
        (
            D6cSelectionMutation::WrongCaptureRun,
            "StaticWorkerBinding",
            "lexical captures but",
        ),
        (
            D6cSelectionMutation::CrossRouteTargets,
            "Call",
            "route has no callee",
        ),
    ];

    for (mutation, expected_category, discriminating) in OWNED {
        reset_d8g_emissions();
        let (outcome, applications) = with_d6c_selection_mutation(*mutation, || {
            crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
                "ken_d6c_refusal",
                false,
            )
            .map(|_| ())
        });

        // 1. The mutation FIRED. A green defence from a perturbation that never
        //    reached the seat proves the opposite of what it looks like.
        assert!(
            applications > 0,
            "{mutation:?} never reached the selection seam, so the refusal below would be about \
             some other program entirely"
        );

        // 2. The typed error, by category PLUS a discriminating reason.
        let (category, reason) = match &outcome {
            Err(CraneliftBackendError::Unsupported(unsupported)) => {
                (unsupported.construct, unsupported.reason.clone())
            }
            Err(CraneliftBackendError::Backend(BackendFailure::Module(reason))) => {
                ("Module", reason.clone())
            }
            Err(other) => panic!(
                "{mutation:?} must be refused by a guard that owns the moved input, not by an \
                 unrelated backend failure: {other:?}"
            ),
            Ok(()) => panic!(
                "{mutation:?} COMPILED. The moved input reached emission unrefused, so no \
                 production guard owns it and this row must not pretend otherwise"
            ),
        };
        assert_eq!(
            category, *expected_category,
            "{mutation:?} must be refused by the guard that owns it. A different category means \
             the perturbation was caught somewhere downstream that does not own this input, which \
             credits the wrong mechanism: {reason}"
        );
        assert!(
            reason.contains(discriminating),
            "{mutation:?}: the refusal must name the moved input. Category alone would be \
             satisfied by any refusal of the same kind, so this fragment is what makes the match \
             discriminating; got {reason}"
        );

        // 3. ZERO post-emission records. The refusal is pre-emission, so no
        //    instruction may exist -- this is what separates a fail-closed guard
        //    from a downstream relation failure.
        assert!(
            d8g_emissions().is_empty(),
            "{mutation:?} refused, but a static-worker call was still emitted first. A \
             pre-emission guard that lets an instruction be written is not the plane this row \
             claims: {:?}",
            d8g_emissions()
        );
    }

    // 4. The exact positive, rerun SEPARATELY. Without it every refusal above is
    //    consistent with a fixture that simply cannot compile.
    reset_d8g_emissions();
    let (exact, applications) = with_d6c_selection_mutation(D6cSelectionMutation::Exact, || {
        crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
            "ken_d6c_exact",
            false,
        )
        .map(|_| ())
        .map_err(|error| format!("{error:?}"))
    });
    assert_eq!(
        applications, 0,
        "the exact run must apply NO mutation, or the positive below is not the unperturbed program"
    );
    exact.expect("THE EXACT POSITIVE: the unperturbed witness compiles");
    assert!(
        !d8g_emissions().is_empty(),
        "and it emits, so the zero-emission assertions above are a fact about the refusals rather \
         than about a witness that never emits at all"
    );
}


/// **`RT-CONTSRC-PRODUCER-LOCAL` `D6c` — the sealed binder run refuses a
/// miscounted or permuted run at its own producer, before anything consumes it.**
///
/// ## What this row is about
///
/// The sibling row perturbs arguments handed to the static-worker constructor.
/// These three perturb the **run's shape**, which is a different subject and has
/// a different owner: [`continuation_case_binder_run`]'s own postcondition,
/// applied where the run is sealed.
///
/// ## Why the guard exists — measured, not assumed
///
/// Before it, all three were **accepted BY THEIR PRODUCER**: the run was returned
/// malformed in every case. ⛔ That is not the same as "unrefused" — what each
/// one then met differs, and the differences are the point. ⇒ **The sealed
/// postcondition now owns all three canonical run-shape violations**, which is
/// what makes their several downstream fates no longer load-bearing. `D6c`'s
/// census (`evt_na5pwjmxwxvn`, six cells, every one with a positive application
/// count and a genuinely changed member run):
///
/// | perturbation | mixed witness | governed witness |
/// |---|---|---|
/// | omission | **compiled** | `Var: no runtime binding for index 2` |
/// | duplicate | **compiled** | `a Var in value position` |
/// | permutation | reached the producer, then refused **downstream** at `Call: "callee is not a closure"` | **compiled** — equal-value identity |
///
/// **Only omission and duplicate were genuinely unguarded**, and the mixed
/// witness is where that is established: there the run was returned malformed
/// *and* nothing downstream caught it. ⛔ Omission is the pre-`D6a` defect
/// exactly, and it compiled because that case body reads only `Var(0)`, so every
/// later binder shifted with nothing positioned to notice.
///
/// ⛔ **`WrongOrder` was never unguarded, and neither witness says it was.** On
/// the **non-degenerate** `px8tr` — where the two members carry distinct routes —
/// it was **producer-accepted and then downstream-refused** at the callee-shape
/// check: a real refusal, by a guard that does not own run shape. On the governed
/// witness it is **equal-value identity**, so its clean compile was never
/// evidence about any guard at all. Two different facts, neither of them an
/// absent guard, and both were corrected before this row landed.
///
/// ## The proof attribution, stated exactly
///
/// - **Omission and duplicate discriminate CARDINALITY.** Both change the run's
///   length, and the postcondition's checked total is what refuses them.
/// - **The mixed witness discriminates SEGMENT ORDER**, and this row requires the
///   refusal to have **moved** from the downstream callee-shape check to the
///   producer guard. That migration is the deliverable, not merely that some
///   refusal happens.
/// - ⚠ **The governed witness is the EQUAL-VALUE LIMIT.** Its two members hold
///   identical route and body origin, so permuting them is **observational
///   identity** and its old clean compile **never proved a missing guard**. It
///   refuses here now — but that refusal proves **typed-role order**, that a
///   `SelectedRecursiveArgument` is not sitting in the induction-hypothesis
///   prefix. ⛔ It does **not** prove distinct materialized values, and no reader
///   may take it as evidence that the two bindings differ there.
///
/// ## What the postcondition deliberately does NOT require
///
/// Numerically source-ordered `Ordinary(index)` values, or any reconstruction of
/// ordinary-envelope order. Each member is checked against the **role its own
/// index names**, so a self-consistent envelope permutation stays lawful.
///
/// ## Plane
///
/// Refusal, entirely. Each case asserts the perturbation **fired**, the **typed**
/// `Module` category with a discriminating reason, and **zero** post-emission
/// records; the exact positive is rerun separately on both witnesses.
///
/// **Promise class: durable invariant.** Each clause is a refusal keyed to the
/// shape it perturbs. Only a guard ceasing to own run shape reds it.
#[test]
fn d6c_the_sealed_binder_run_refuses_a_miscounted_or_permuted_run_at_its_producer() {
    use crate::cranelift_backend::lowering::{
        d8g_emissions, reset_d8g_emissions, with_d6c_selection_mutation, D6cSelectionMutation,
    };
    use crate::cranelift_backend::surface::{BackendFailure, CraneliftBackendError};

    /// `(perturbation, the discriminating fact the sealed run's refusal must name)`
    const SHAPES: &[(D6cSelectionMutation, &str)] = &[
        (
            D6cSelectionMutation::OmitSelectedArgument,
            "the sealed binder run holds",
        ),
        (
            D6cSelectionMutation::DuplicateSelectedArgument,
            "the sealed binder run holds",
        ),
        (
            D6cSelectionMutation::WrongOrder,
            "inside the 1-member induction-hypothesis prefix",
        ),
    ];

    for (mutation, discriminating) in SHAPES {
        // ── the mixed witness ───────────────────────────────────────────────
        reset_d8g_emissions();
        let (outcome, applications) = with_d6c_selection_mutation(*mutation, || {
            crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
                "ken_d6c_sealed",
                false,
            )
            .map(|_| ())
        });
        assert!(
            applications > 0,
            "{mutation:?} never reshaped the run on the mixed witness, so the refusal below would \
             be about some other program"
        );
        let reason = match &outcome {
            Err(CraneliftBackendError::Backend(BackendFailure::Module(reason))) => reason.clone(),
            Err(other) => panic!(
                "{mutation:?} must be refused by the SEALED RUN's own postcondition, a typed \
                 Module failure. ⛔ A different category means the run was returned malformed and \
                 something downstream caught it, which is the misattribution this guard exists to \
                 end: {other:?}"
            ),
            Ok(()) => panic!(
                "{mutation:?} COMPILED on the mixed witness. The malformed run reached emission, \
                 which is the pre-guard behaviour this row exists to have ended"
            ),
        };
        assert!(
            reason.contains(discriminating),
            "{mutation:?}: the refusal must name the shape fact it caught, or any Module failure \
             would satisfy this row; got {reason}"
        );
        assert!(
            d8g_emissions().is_empty(),
            "{mutation:?} refused, but a static-worker call was emitted first. The postcondition \
             runs at the producer, so nothing may have been written: {:?}",
            d8g_emissions()
        );

        // ── the governed witness ────────────────────────────────────────────
        //
        // ⚠ For the permutation this is the EQUAL-VALUE LIMIT: the refusal below
        // proves typed-ROLE order, never that the two members differ in value.
        reset_d8g_emissions();
        let expr = crate::cranelift_backend::planning::governed_nested_resource_bracket(3);
        let (governed, governed_applications) =
            with_d6c_selection_mutation(*mutation, || recursive_port_process_compiles(&expr));
        assert!(
            governed_applications > 0,
            "{mutation:?} never reshaped the run on the governed witness"
        );
        // ⛔ Typed and exhaustive, exactly as on the mixed witness. Matching on
        // the formatted value would accept any failure whose text happened to
        // mention the run, including one raised somewhere that does not own it.
        let governed_reason = match &governed {
            Err(CraneliftBackendError::Backend(BackendFailure::Module(reason))) => reason.clone(),
            Err(CraneliftBackendError::Backend(other)) => panic!(
                "{mutation:?} on the governed witness must be refused by the sealed run's own \
                 postcondition, a typed Module failure; got another backend failure: {other:?}"
            ),
            Err(CraneliftBackendError::Unsupported(unsupported)) => panic!(
                "{mutation:?} on the governed witness was refused by a LOWERING guard rather than \
                 by the sealed run's postcondition, which is the downstream misattribution this \
                 guard exists to end: {unsupported:?}"
            ),
            Ok(()) => panic!(
                "{mutation:?} COMPILED on the governed witness, so the malformed run was accepted \
                 by its producer"
            ),
        };
        assert!(
            governed_reason.contains(discriminating),
            "{mutation:?} on the governed witness: the refusal must name the same shape fact it \
             names on the mixed one; got {governed_reason}"
        );
        assert!(
            d8g_emissions().is_empty(),
            "{mutation:?} refused on the governed witness, but a static-worker call was emitted \
             first: {:?}",
            d8g_emissions()
        );
    }

    // The exact positive, on BOTH witnesses, rerun separately. Without it every
    // refusal above is consistent with fixtures that simply cannot compile.
    reset_d8g_emissions();
    let (exact, applications) = with_d6c_selection_mutation(D6cSelectionMutation::Exact, || {
        crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
            "ken_d6c_sealed_exact",
            false,
        )
        .map(|_| ())
        .map_err(|error| format!("{error:?}"))
    });
    assert_eq!(applications, 0, "the exact run applies no perturbation");
    exact.expect("THE EXACT POSITIVE: the unperturbed mixed witness still compiles");
    assert!(
        !d8g_emissions().is_empty(),
        "and it still emits, so the zero-emission clauses above are facts about the refusals"
    );
    let expr = crate::cranelift_backend::planning::governed_nested_resource_bracket(3);
    recursive_port_process_compiles(&expr)
        .expect("THE EXACT POSITIVE: the unperturbed governed witness still compiles");
}



#[cfg(test)]
const D8F_SYMBOL: &str = "decl:fixture::d8f::witness";

/// How the `D8f` witness is perturbed. Each variant moves exactly one fact about
/// WHICH call may consume the pending checked-IH marker.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum D8fPerturbation {
    /// The lawful witness.
    None,
    /// A second invocation marker inside the first.
    NestedMarker,
    /// The marker moved onto the inner ordinary call, with the plan still built
    /// for the outer one.
    MarkerMovedInward,
}

#[cfg(test)]
fn d8f_witness_with(with_ordinary_call: bool, perturbation: D8fPerturbation) -> RuntimeExpr {
    let expr = d8n_witness();
    let RuntimeExpr::Let { value, body } = expr else {
        panic!("let")
    };
    let RuntimeExpr::ComputationalMatch {
        scrutinee,
        cases,
        default,
    } = *value
    else {
        panic!("eliminator")
    };
    let RuntimeExpr::CheckedSubcontinuationFrame {
        frame_id,
        body: bridge,
    } = &cases[0].body
    else {
        panic!("marked bridge")
    };
    let RuntimeExpr::ComputationalMatch {
        scrutinee: bridge_scrutinee,
        cases: bridge_cases,
        default: bridge_default,
    } = bridge.as_ref()
    else {
        panic!("bridge")
    };
    let marked_cases = bridge_cases
        .iter()
        .map(|case| {
            let RuntimeExpr::CheckedComputationalIHSlots {
                slot_template_ids,
                checked_occurrence_paths,
                body,
            } = &case.body
            else {
                panic!("slot marker")
            };
            let RuntimeExpr::Call { callee, args } = body.as_ref() else {
                panic!("the slot marker wraps the IH application")
            };
            crate::RuntimeComputationalMatchCase {
                constructor: case.constructor.clone(),
                argument_binders: case.argument_binders,
                recursive_positions: case.recursive_positions.clone(),
                body: RuntimeExpr::CheckedComputationalIHSlots {
                    slot_template_ids: slot_template_ids.clone(),
                    checked_occurrence_paths: checked_occurrence_paths.clone(),
                    body: Box::new(d8f_marked_application(
                        callee,
                        args,
                        with_ordinary_call,
                        perturbation,
                    )),
                },
            }
        })
        .collect();
    let mut cases = cases.clone();
    cases[0] = crate::RuntimeComputationalMatchCase {
        constructor: cases[0].constructor.clone(),
        argument_binders: cases[0].argument_binders,
        recursive_positions: cases[0].recursive_positions.clone(),
        body: RuntimeExpr::CheckedSubcontinuationFrame {
            frame_id: *frame_id,
            body: Box::new(RuntimeExpr::ComputationalMatch {
                scrutinee: bridge_scrutinee.clone(),
                cases: marked_cases,
                default: bridge_default.clone(),
            }),
        },
    };
    RuntimeExpr::Let {
        value: Box::new(RuntimeExpr::ComputationalMatch {
            scrutinee,
            cases,
            default,
        }),
        body,
    }
}

/// The marked application at the bridge case body, under one perturbation.
///
/// `callee` is the induction-hypothesis binder the lawful application calls;
/// `args` its lawful argument run.
#[cfg(test)]
fn d8f_marked_application(
    callee: &RuntimeExpr,
    args: &[RuntimeExpr],
    with_ordinary_call: bool,
    perturbation: D8fPerturbation,
) -> RuntimeExpr {
    // The ordinary selected-argument call: the SAME worker, the SAME arity, at a
    // DIFFERENT occurrence.
    let application = |callee: &RuntimeExpr| RuntimeExpr::Call {
        callee: Box::new(callee.clone()),
        args: if with_ordinary_call {
            vec![RuntimeExpr::Call {
                callee: Box::new(callee.clone()),
                args: args.to_vec(),
            }]
        } else {
            args.to_vec()
        },
    };
    let marker = |call_template_id: u64, path: Vec<u64>, body: RuntimeExpr| {
        RuntimeExpr::CheckedComputationalIHInvocation {
            call_template_id,
            checked_occurrence_path: path,
            body: Box::new(body),
        }
    };
    match perturbation {
        D8fPerturbation::None => marker(100, vec![30], application(callee)),
        // A second marker inside the first, on the same application.
        // The outer marker wraps a complete application, as entry requires; its
        // ARGUMENT carries a second marker. So the inner marker is entered while
        // the outer is still pending.
        D8fPerturbation::NestedMarker => marker(
            100,
            vec![30],
            RuntimeExpr::Call {
                callee: Box::new(callee.clone()),
                args: vec![marker(
                    101,
                    vec![31],
                    RuntimeExpr::Call {
                        callee: Box::new(callee.clone()),
                        args: args.to_vec(),
                    },
                )],
            },
        ),
        // The marker moved onto the INNER ordinary call. The outer application
        // is left unmarked, so the marker names an occurrence the plan built for
        // the outer one does not.
        D8fPerturbation::MarkerMovedInward => RuntimeExpr::Call {
            callee: Box::new(callee.clone()),
            args: vec![marker(
                100,
                vec![30],
                RuntimeExpr::Call {
                    callee: Box::new(callee.clone()),
                    args: args.to_vec(),
                },
            )],
        },
    }
}

#[cfg(test)]
fn d8f_declaration(with_ordinary_call: bool) -> RuntimeDeclaration {
    d8f_declaration_with(with_ordinary_call, D8fPerturbation::None)
}

#[cfg(test)]
fn d8f_declaration_with(
    with_ordinary_call: bool,
    perturbation: D8fPerturbation,
) -> RuntimeDeclaration {
    RuntimeDeclaration {
        symbol: D8F_SYMBOL.to_string(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::Closure {
                captures: Vec::new(),
                params: vec!["state".to_string()],
                body: Box::new(d8f_witness_with(with_ordinary_call, perturbation)),
            },
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    }
}

/// Every checked marker location in the `D8f` witness, measured by the
/// production collector.
#[cfg(test)]
fn d8f_marker_sets(
    with_ordinary_call: bool,
    perturbation: D8fPerturbation,
) -> crate::cranelift_backend::planning::CheckedOrientedMarkerSets {
    let declaration = d8f_declaration_with(with_ordinary_call, perturbation);
    let RuntimeDeclarationKind::Transparent { body } = &declaration.kind else {
        panic!("transparent")
    };
    let mut sets = crate::cranelift_backend::planning::CheckedOrientedMarkerSets::default();
    crate::cranelift_backend::planning::collect_checked_oriented_markers(
        body,
        &mut sets,
        D8F_SYMBOL,
        &mut Vec::new(),
    )
    .expect("the witness's markers collect");
    sets
}

#[cfg(test)]
fn d8f_located(
    paths: &std::collections::BTreeSet<Vec<u64>>,
) -> Vec<crate::CheckedRuntimeMarkerLocationV1> {
    let mut paths = paths.iter().cloned().collect::<Vec<_>>();
    paths.sort();
    paths
        .into_iter()
        .map(|runtime_path| crate::CheckedRuntimeMarkerLocationV1 {
            declaration: D8F_SYMBOL.to_string(),
            runtime_path,
        })
        .collect()
}

#[cfg(test)]
fn d8f_plan(with_ordinary_call: bool) -> crate::OrientedSubcontinuationPlanV1 {
    d8f_plan_with(with_ordinary_call, D8fPerturbation::None)
}

#[cfg(test)]
fn d8f_plan_with(
    with_ordinary_call: bool,
    perturbation: D8fPerturbation,
) -> crate::OrientedSubcontinuationPlanV1 {
    let declaration = d8f_declaration_with(with_ordinary_call, perturbation);
    let RuntimeDeclarationKind::Transparent { body } = &declaration.kind else {
        panic!("transparent")
    };
    let RuntimeExpr::Closure { body, .. } = body else {
        panic!("closure")
    };
    let mut plan = d8m_plan(body, 7);
    for frame in &mut plan.frames {
        frame.declaration = D8F_SYMBOL.to_string();
        frame.occurrence_binding_fingerprint =
            crate::compiler_private_oriented_occurrence_binding_fingerprint(frame);
    }
    let sets = d8f_marker_sets(with_ordinary_call, perturbation);
    let slot_paths = sets
        .computational_ih_slots
        .get(&(200, vec![20]))
        .expect("the witness's slot marker is located");
    let mut slot = crate::CheckedComputationalIHSlotTemplateV1 {
        slot_template_id: 200,
        declaration: D8F_SYMBOL.to_string(),
        checked_match_ordinal: 0,
        checked_occurrence_path: vec![20],
        frame_template_id: 7,
        constructor: "ctor:prelude::Result::Ok".to_string(),
        recursive_position: 0,
        method_binder_ordinal: 4,
        local_telescope: Vec::new(),
        ih_interface: oriented_test_interface(1),
        segment_site_id: 9,
        frame_templates: vec![7],
        input_interface: oriented_test_interface(1),
        output_interface: oriented_test_interface(2),
        runtime_marker_locations: d8f_located(slot_paths),
        occurrence_binding_fingerprint: 0,
    };
    slot.occurrence_binding_fingerprint =
        crate::compiler_private_computational_ih_slot_binding_fingerprint(&slot);
    plan.computational_ih_slots = vec![slot];
    let call_paths = sets
        .computational_ih_calls
        .get(&(100, vec![30]))
        .expect("the witness's invocation marker is located");
    let mut call = crate::CheckedComputationalIHCallTemplateV1 {
        call_template_id: 100,
        declaration: D8F_SYMBOL.to_string(),
        checked_occurrence_path: vec![30],
        slot_template_id: 200,
        arity: 1,
        local_telescope: Vec::new(),
        result_interface: oriented_test_interface(1),
        callee_segment_site_id: 9,
        callee_frame_templates: vec![7],
        composed_frame_templates: Vec::new(),
        parent_frame_template_id: Some(7),
        parent_segment_site_id: Some(9),
        caller_interface: oriented_test_interface(1),
        runtime_marker_locations: d8f_located(call_paths),
        occurrence_binding_fingerprint: 0,
    };
    call.occurrence_binding_fingerprint =
        crate::compiler_private_computational_ih_call_binding_fingerprint(&call);
    plan.computational_ih_calls = vec![call.clone()];
    // The nested-marker perturbation carries a SECOND source marker, so the plan
    // must hold its template too -- otherwise planning refuses on the marker
    // population and the nesting law is never reached.
    if perturbation == D8fPerturbation::NestedMarker {
        let mut inner = crate::CheckedComputationalIHCallTemplateV1 {
            call_template_id: 101,
            checked_occurrence_path: vec![31],
            runtime_marker_locations: d8f_located(
                sets.computational_ih_calls
                    .get(&(101, vec![31]))
                    .expect("the nested marker is located"),
            ),
            occurrence_binding_fingerprint: 0,
            ..call
        };
        inner.occurrence_binding_fingerprint =
            crate::compiler_private_computational_ih_call_binding_fingerprint(&inner);
        plan.computational_ih_calls.push(inner);
    }
    plan
}

#[cfg(test)]
pub(in crate::cranelift_backend::lowering) fn d8f_compile(with_ordinary_call: bool) -> Option<CraneliftBackendError> {
    d8f_compile_with(with_ordinary_call, D8fPerturbation::None, D8fPerturbation::None)
}

/// `source` spells the witness the SOURCE carries; `planned` the one the plan is
/// derived from. Passing different values perturbs one side while the other
/// stays lawful.
#[cfg(test)]
fn d8f_compile_with(
    with_ordinary_call: bool,
    source: D8fPerturbation,
    planned: D8fPerturbation,
) -> Option<CraneliftBackendError> {
    let declaration = d8f_declaration_with(with_ordinary_call, source);
    let entry = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::DeclarationRef {
            symbol: D8F_SYMBOL.to_string(),
        }),
        args: vec![RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        }],
    };
    compile_expr_into_module(
        new_object_module("d8f").expect("module"),
        "ken_d8f",
        Linkage::Export,
        &entry,
        &NativeSeedEnvironment::empty(),
        BTreeMap::from([(D8F_SYMBOL, &declaration)]),
        None,
        true,
        None,
        Some(test_only_distinguished_root_join_plan()),
        Some(d8f_plan_with(with_ordinary_call, planned)),
    )
    .err()
}

/// The plan-derived expectation for one checked application: the call template
/// the invocation marker names, the slot it binds, the binder ordinal the plan
/// seats the hypothesis at, and the arity.
///
/// The INDEPENDENT SIDE. Read out of the oriented plan the compile is handed,
/// never out of an observation, the environment binding, or the emitted call.
#[cfg(test)]
fn d8p_planned_application(with_ordinary_call: bool) -> (u64, u64, u64, u64) {
    let plan = d8f_plan(with_ordinary_call);
    let call = plan
        .computational_ih_calls
        .first()
        .expect("one planned checked application");
    let slot = plan
        .computational_ih_slots
        .iter()
        .find(|slot| slot.slot_template_id == call.slot_template_id)
        .expect("the call template's slot");
    (
        call.call_template_id,
        call.slot_template_id,
        slot.method_binder_ordinal,
        call.arity,
    )
}

/// What the PLANNER defines each callable target to be: the declared arity and
/// ordered capture count of every continuation unit's static worker, keyed by
/// that worker's own body origin.
///
/// The INDEPENDENT SIDE for the target relation. Built by running the static
/// transition planner over the witness's own inputs -- unit definition is the
/// producer of this authority -- and never read from the environment binding,
/// the emitted call, or an observation. A target the planner does not define,
/// or one called at an arity or capture run that is not the planner's for it,
/// disagrees here.
#[cfg(test)]
fn d8p_planned_targets(with_ordinary_call: bool) -> BTreeMap<StaticOriginId, (u32, usize)> {
    let declaration = d8f_declaration(with_ordinary_call);
    let entry = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::DeclarationRef {
            symbol: D8F_SYMBOL.to_string(),
        }),
        args: vec![RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        }],
    };
    let declarations = BTreeMap::from([(D8F_SYMBOL, &declaration)]);
    let plan = plan_static_transition_graph_with_symbols(
        &entry,
        &declarations,
        &crate::NativeProcessSymbols::legacy_prelude(),
        AbiRootIngress::Value,
        true,
    )
    .expect("the witness plans");
    plan.continuation_units()
        .expect("continuation units")
        .into_iter()
        .map(|unit| {
            (
                unit.worker_body_origin(),
                (unit.worker_declared_arity(), unit.worker_capture_count()),
            )
        })
        .collect()
}
