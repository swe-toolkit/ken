//! Plan-owned classification of immediate producer/eliminator bridges.
//!
//! The structural classifier in this module is shared by plan construction and
//! lowering. Planning owns the population and stores only a bounded descriptor;
//! lowering re-runs this pure classifier to borrow the cases/default it needs,
//! then checks every local coordinate against the stored descriptor.
//!
//! **Stratum A of the `#3676` re-cut.** This file carries the plan-INDEPENDENT
//! classifier only. It names no plan type, adds no field anywhere, and defines
//! no extension `impl`. The plan-coupled half of the original module -- the
//! deriving, building, publishing and validating half, its extension `impl`,
//! its plan field and its cfg-gated mutation guard -- is a successor slice and
//! is deliberately absent. Those names are kept out of this file entirely so
//! that a zero-hit grep for them is a real control rather than a count of
//! mentions in a comment.
//!
//! The classifier has NO CONSUMER until that successor lands, so a
//! production build reports `never used` here. That is not debris: it is the
//! only live indicator that this module sits on no live path, which is the
//! property this slice is defined by. Do NOT silence it with
//! `#[allow(dead_code)]` -- the warnings clear themselves when the successor
//! wires the classifier in. Architect ruling, 2026-09-16.

use std::collections::BTreeSet;

use super::{ContinuationCallIdentity, StaticOriginId};
use crate::{RuntimeExpr, RuntimeTrap};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum ImmediateBridgeConsumerKind {
    Ordinary,
    Computational,
    CheckedComputational { frame_id: u64 },
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum ImmediateBridgeCause {
    Heterogeneous,
    StaticHostOperation,
}

#[derive(Clone, Copy)]
pub(in crate::cranelift_backend) enum ImmediateBridgeConsumer<'a> {
    Ordinary {
        cases: &'a [crate::RuntimeMatchCase],
        default: &'a RuntimeTrap,
    },
    Computational {
        cases: &'a [crate::RuntimeComputationalMatchCase],
        default: &'a RuntimeTrap,
    },
    CheckedComputational {
        frame_id: u64,
        cases: &'a [crate::RuntimeComputationalMatchCase],
        default: &'a RuntimeTrap,
    },
}

impl ImmediateBridgeConsumer<'_> {
    pub(in crate::cranelift_backend) fn kind(self) -> ImmediateBridgeConsumerKind {
        match self {
            Self::Ordinary { .. } => ImmediateBridgeConsumerKind::Ordinary,
            Self::Computational { .. } => ImmediateBridgeConsumerKind::Computational,
            Self::CheckedComputational { frame_id, .. } => {
                ImmediateBridgeConsumerKind::CheckedComputational { frame_id }
            }
        }
    }
}

#[derive(Clone, Copy)]
pub(in crate::cranelift_backend) struct ImmediateBridgeSelection<'a> {
    pub(in crate::cranelift_backend) selected_field: usize,
    pub(in crate::cranelift_backend) consumer: ImmediateBridgeConsumer<'a>,
    pub(in crate::cranelift_backend) checked_ih_slots_wrapper: bool,
    pub(in crate::cranelift_backend) cause: ImmediateBridgeCause,
}

/// The bounded, plan-owned descriptor for one exact causal identity whose
/// producer is realized by an immediate bridge.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct ImmediateBridgeRealization {
    identity: ContinuationCallIdentity,
    producer_construct_origin: StaticOriginId,
    computational_frame_origin: StaticOriginId,
    alternative: u32,
    recursive_position: u32,
    case_body_origin: StaticOriginId,
    effective_bridge_body_origin: StaticOriginId,
    selected_field: u32,
    consumer: ImmediateBridgeConsumerKind,
    checked_ih_slots_wrapper: bool,
    cause: ImmediateBridgeCause,
}

impl ImmediateBridgeRealization {
    pub(in crate::cranelift_backend) fn identity(&self) -> &ContinuationCallIdentity {
        &self.identity
    }

    pub(in crate::cranelift_backend) fn producer_construct_origin(&self) -> StaticOriginId {
        self.producer_construct_origin
    }

    pub(in crate::cranelift_backend) fn computational_frame_origin(&self) -> StaticOriginId {
        self.computational_frame_origin
    }

    pub(in crate::cranelift_backend) fn alternative(&self) -> u32 {
        self.alternative
    }

    pub(in crate::cranelift_backend) fn recursive_position(&self) -> u32 {
        self.recursive_position
    }

    pub(in crate::cranelift_backend) fn case_body_origin(&self) -> StaticOriginId {
        self.case_body_origin
    }

    pub(in crate::cranelift_backend) fn effective_bridge_body_origin(&self) -> StaticOriginId {
        self.effective_bridge_body_origin
    }

    pub(in crate::cranelift_backend) fn selected_field(&self) -> u32 {
        self.selected_field
    }

    pub(in crate::cranelift_backend) fn consumer(&self) -> ImmediateBridgeConsumerKind {
        self.consumer
    }

    pub(in crate::cranelift_backend) fn checked_ih_slots_wrapper(&self) -> bool {
        self.checked_ih_slots_wrapper
    }

    pub(in crate::cranelift_backend) fn cause(&self) -> ImmediateBridgeCause {
        self.cause
    }
}

/// Classify only the four ruled immediate-bridge spellings. This function is
/// pure: it reads source structure and returns borrowed source components, but
/// stores no expression and creates no identity.
pub(in crate::cranelift_backend) fn classify_immediate_bridge<'a>(
    body: &'a RuntimeExpr,
    producer_args: &'a [RuntimeExpr],
    argument_binder_offset: usize,
) -> Option<ImmediateBridgeSelection<'a>> {
    let (scrutinee, consumer, checked_ih_slots_wrapper) = match body {
        RuntimeExpr::ComputationalMatch {
            scrutinee,
            cases,
            default,
        } => (
            scrutinee.as_ref(),
            ImmediateBridgeConsumer::Computational { cases, default },
            false,
        ),
        RuntimeExpr::Match {
            scrutinee,
            cases,
            default,
        } => (
            scrutinee.as_ref(),
            ImmediateBridgeConsumer::Ordinary { cases, default },
            false,
        ),
        RuntimeExpr::CheckedSubcontinuationFrame { frame_id, body } => {
            let RuntimeExpr::ComputationalMatch {
                scrutinee,
                cases,
                default,
            } = body.as_ref()
            else {
                return None;
            };
            (
                scrutinee.as_ref(),
                ImmediateBridgeConsumer::CheckedComputational {
                    frame_id: *frame_id,
                    cases,
                    default,
                },
                false,
            )
        }
        RuntimeExpr::CheckedComputationalIHSlots { body, .. } => {
            let RuntimeExpr::Match {
                scrutinee,
                cases,
                default,
            } = body.as_ref()
            else {
                return None;
            };
            (
                scrutinee.as_ref(),
                ImmediateBridgeConsumer::Ordinary { cases, default },
                true,
            )
        }
        _ => return None,
    };
    let RuntimeExpr::Var(index) = scrutinee else {
        return None;
    };
    let index = usize::try_from(*index).ok()?;
    let selected_field = index.checked_sub(argument_binder_offset)?;
    let producer = producer_args.get(selected_field)?;
    let heterogeneous = !checked_ih_slots_wrapper && requires_heterogeneous_deforestation(producer);
    let static_host_operation = statically_selects_host_operation(producer, consumer);
    let cause = if heterogeneous {
        ImmediateBridgeCause::Heterogeneous
    } else if static_host_operation {
        ImmediateBridgeCause::StaticHostOperation
    } else {
        return None;
    };
    Some(ImmediateBridgeSelection {
        selected_field,
        consumer,
        checked_ih_slots_wrapper,
        cause,
    })
}

fn statically_selects_host_operation(
    producer: &RuntimeExpr,
    consumer: ImmediateBridgeConsumer<'_>,
) -> bool {
    let ImmediateBridgeConsumer::Ordinary { cases, .. } = consumer else {
        return false;
    };
    statically_selected_case_reaches_host_effect(producer, cases)
}

fn statically_selected_case_reaches_host_effect(
    producer: &RuntimeExpr,
    cases: &[crate::RuntimeMatchCase],
) -> bool {
    let RuntimeExpr::Construct { constructor, args } = producer else {
        return false;
    };
    let Some(case) = cases.iter().find(|case| case.constructor == *constructor) else {
        return false;
    };
    if case.binders != args.len() {
        return false;
    }
    match &case.body {
        RuntimeExpr::Let { value, .. } if matches!(value.as_ref(), RuntimeExpr::Effect { .. }) => {
            true
        }
        RuntimeExpr::Match {
            scrutinee, cases, ..
        } => {
            let RuntimeExpr::Var(index) = scrutinee.as_ref() else {
                return false;
            };
            usize::try_from(*index)
                .ok()
                .filter(|index| *index < case.binders)
                .and_then(|index| args.get(index))
                .is_some_and(|field| statically_selected_case_reaches_host_effect(field, cases))
        }
        _ => false,
    }
}

pub(in crate::cranelift_backend) fn requires_heterogeneous_deforestation(
    expr: &RuntimeExpr,
) -> bool {
    matches!(
        expr,
        RuntimeExpr::Match { .. }
            | RuntimeExpr::ComputationalMatch { .. }
            | RuntimeExpr::If { .. }
            | RuntimeExpr::Call { .. }
    ) && produces_deforestable_aggregate_with_ih(expr, &BTreeSet::new())
}

pub(in crate::cranelift_backend) fn produces_deforestable_aggregate_with_ih(
    expr: &RuntimeExpr,
    aggregate_ihs: &BTreeSet<usize>,
) -> bool {
    match expr {
        RuntimeExpr::CheckedJoinSite { body, .. } => {
            produces_deforestable_aggregate_with_ih(body, aggregate_ihs)
        }
        RuntimeExpr::Construct { .. } => true,
        RuntimeExpr::Let { body, .. } => {
            produces_deforestable_aggregate_with_ih(body, &shifted_aggregate_ihs(aggregate_ihs, 1))
        }
        RuntimeExpr::Match { cases, .. } => {
            !cases.is_empty()
                && cases.iter().all(|case| {
                    produces_deforestable_aggregate_with_ih(
                        &case.body,
                        &shifted_aggregate_ihs(aggregate_ihs, case.binders),
                    )
                })
        }
        RuntimeExpr::ComputationalMatch { cases, .. } => {
            !cases.is_empty()
                && cases.iter().all(|case| {
                    let mut case_ihs = (0..case.recursive_positions.len()).collect::<BTreeSet<_>>();
                    case_ihs.extend(aggregate_ihs.iter().map(|index| {
                        index + case.recursive_positions.len() + case.argument_binders
                    }));
                    produces_deforestable_aggregate_with_ih(&case.body, &case_ihs)
                })
        }
        RuntimeExpr::If {
            then_expr,
            else_expr,
            ..
        } => {
            produces_deforestable_aggregate_with_ih(then_expr, aggregate_ihs)
                && produces_deforestable_aggregate_with_ih(else_expr, aggregate_ihs)
        }
        RuntimeExpr::Call { callee, .. } => {
            if let RuntimeExpr::Var(index) = callee.as_ref() {
                return usize::try_from(*index).is_ok_and(|index| aggregate_ihs.contains(&index));
            }
            match callee.as_ref() {
                RuntimeExpr::Closure {
                    captures,
                    params,
                    body,
                } => produces_deforestable_aggregate_with_ih(
                    body,
                    &shifted_aggregate_ihs(aggregate_ihs, params.len() + captures.len()),
                ),
                RuntimeExpr::LexicalClosure {
                    captures,
                    params,
                    body,
                } => produces_deforestable_aggregate_with_ih(
                    body,
                    &shifted_aggregate_ihs(aggregate_ihs, params.len() + captures.len()),
                ),
                _ => false,
            }
        }
        _ => false,
    }
}

fn shifted_aggregate_ihs(aggregate_ihs: &BTreeSet<usize>, by: usize) -> BTreeSet<usize> {
    aggregate_ihs.iter().map(|index| index + by).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RuntimeMatchCase, RuntimeTrapCode};

    // PROVENANCE RULE (`AC-6`). Every case below is one of two kinds and says
    // which in its own doc comment:
    //
    //   LIFTED      its expected verdict is attested by a named site on the
    //               reference branch `0f71ab5b9`, cited as `file:line`.
    //   NEW INTENT  no plan-level ancestor asserts this verdict. It is the
    //               author's reading and is labelled so, by name, in the test.
    //
    // The distinction is not decoration. `AC-2` -- stub the classifier and the
    // tests go red -- measures COUPLING, not FAITHFULNESS: a case whose
    // expectation was read off the implementation also goes red when the
    // implementation is stubbed. Only the citation separates the two.

    fn trap(message: &str) -> RuntimeTrap {
        RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: message.to_string(),
        }
    }

    fn construct(constructor: &str, args: Vec<RuntimeExpr>) -> RuntimeExpr {
        RuntimeExpr::Construct {
            constructor: constructor.to_string(),
            args,
        }
    }

    /// LIFTED -- `core/tests/mod.rs:1518-1522` at `0f71ab5b9`: *"routes its
    /// scrutinee through `lower_computational_producer_expr` when the scrutinee
    /// `requires_heterogeneous_deforestation` -- which a `Call` whose callee is
    /// a closure returning a `Construct` satisfies."*
    ///
    /// THE SPELLING IS THE ANCESTOR'S, NOT A CONVENIENT EQUIVALENT. That
    /// fixture builds the value through `nullary_closure` (`mod.rs:1471-1480`),
    /// which uses **`LexicalClosure`**. `Closure` reaches the same verdict, but
    /// only by a reading of the predicate's `Call` arm -- which is the
    /// implementation, not the ancestor. The `Closure` spelling is carried
    /// separately below and labelled as authored.
    fn heterogeneous_call_producer() -> RuntimeExpr {
        RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: Vec::new(),
                body: Box::new(construct("ctor:fixture::Pair::Mk", Vec::new())),
            }),
            args: Vec::new(),
        }
    }

    /// NEW INTENT. No plan-level ancestor attests the `Closure` spelling; the
    /// attested one is `LexicalClosure` (see above). That the predicate treats
    /// the two identically is read off its `Call` arm, so this case is the
    /// author's reading and is labelled rather than presented as a lift.
    fn heterogeneous_call_producer_ordinary_closure_spelling() -> RuntimeExpr {
        RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::Closure {
                captures: Vec::new(),
                params: vec!["x".to_string()],
                body: Box::new(construct("ctor:fixture::Pair::Mk", Vec::new())),
            }),
            args: Vec::new(),
        }
    }

    /// LIFTED -- `core/tests/mod.rs:2691-2693` at `0f71ab5b9`, inside
    /// `seed_call_port_producer_match_example()`: *"a `Match`, so
    /// `requires_heterogeneous_deforestation` holds, on a compile-time
    /// constructor, so exactly one arm is lowered and no join is merged."*
    fn heterogeneous_match_producer() -> RuntimeExpr {
        RuntimeExpr::Match {
            scrutinee: Box::new(construct("ctor:prelude::Bool::True", Vec::new())),
            cases: vec![RuntimeMatchCase {
                constructor: "ctor:prelude::Bool::True".to_string(),
                binders: 0,
                body: construct("ctor:fixture::Pair::Mk", Vec::new()),
            }],
            default: trap("heterogeneous match producer selected no case"),
        }
    }

    /// NEW INTENT. No plan-level site asserts a NEGATIVE verdict for
    /// `requires_heterogeneous_deforestation` over a named value; the recorded
    /// rejected shapes at `core/tests/specialization_binding.rs:4301-4302`
    /// constrain the surrounding FIXTURE, not this predicate's input. The
    /// falsity here is structural: `Var` is none of the four spellings the
    /// `matches!` guard admits.
    fn inert_producer() -> RuntimeExpr {
        RuntimeExpr::Var(9)
    }

    fn ordinary_consumer_body(scrutinee: RuntimeExpr) -> RuntimeExpr {
        RuntimeExpr::Match {
            scrutinee: Box::new(scrutinee),
            cases: vec![RuntimeMatchCase {
                constructor: "ctor:fixture::Pair::Mk".to_string(),
                binders: 0,
                body: RuntimeExpr::Var(0),
            }],
            default: trap("consumer selected no case"),
        }
    }

    #[test]
    fn lifted_call_closure_construct_producer_is_heterogeneous() {
        assert!(requires_heterogeneous_deforestation(
            &heterogeneous_call_producer()
        ));
    }

    #[test]
    fn new_intent_the_ordinary_closure_spelling_is_also_heterogeneous() {
        // NEW INTENT, see the helper: the attested spelling is `LexicalClosure`.
        assert!(requires_heterogeneous_deforestation(
            &heterogeneous_call_producer_ordinary_closure_spelling()
        ));
    }

    #[test]
    fn lifted_compile_time_constructor_match_producer_is_heterogeneous() {
        assert!(requires_heterogeneous_deforestation(
            &heterogeneous_match_producer()
        ));
    }

    #[test]
    fn new_intent_a_var_producer_is_not_heterogeneous() {
        // NEW INTENT, see `inert_producer`.
        assert!(!requires_heterogeneous_deforestation(&inert_producer()));
    }

    /// LIFTED -- `lowering/mod.rs:11739-11740` at `0f71ab5b9`, PRODUCTION code
    /// and therefore attested by every green run of that branch:
    ///
    ///     produces_deforestable_aggregate_with_ih(expr, &recursive_hypotheses)
    ///         && !produces_deforestable_aggregate_with_ih(expr, &BTreeSet::new())
    ///
    /// The shape CROSSES A BINDER on purpose. `shifted_aggregate_ihs` shifts
    /// the IH set by binder depth, and the `Let` arm recurses WITH the shift
    /// while other arms recurse WITHOUT it. A single-point case that never
    /// crosses a binder cannot tell those arms apart, so an off-by-one in the
    /// shift would be invisible to it. The differential over a binder-crossing
    /// shape is what catches it.
    #[test]
    fn lifted_differential_same_expr_opposite_verdicts_across_one_binder() {
        let expr = RuntimeExpr::Let {
            value: Box::new(RuntimeExpr::Var(0)),
            body: Box::new(RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::Var(1)),
                args: Vec::new(),
            }),
        };
        let ihs = BTreeSet::from([0usize]);
        assert!(
            produces_deforestable_aggregate_with_ih(&expr, &ihs),
            "IH 0 shifted across the Let's binder must reach the Var(1) callee"
        );
        assert!(
            !produces_deforestable_aggregate_with_ih(&expr, &BTreeSet::new()),
            "with no IHs the same expression must not produce a deforestable \
             aggregate -- this is the half that makes it a differential"
        );
    }

    #[test]
    fn lifted_heterogeneous_producer_selected_by_offset_arithmetic() {
        // Structure authored, producer LIFTED. `Var(3)` with offset 1 selects
        // `producer_args[2]`.
        let body = ordinary_consumer_body(RuntimeExpr::Var(3));
        let args = vec![
            inert_producer(),
            inert_producer(),
            heterogeneous_call_producer(),
        ];
        let selection = classify_immediate_bridge(&body, &args, 1)
            .expect("a heterogeneous producer at the selected field must classify");
        assert_eq!(selection.selected_field, 2);
        assert_eq!(selection.cause, ImmediateBridgeCause::Heterogeneous);
        assert!(!selection.checked_ih_slots_wrapper);
    }

    #[test]
    fn checked_ih_slots_wrapper_suppresses_the_heterogeneous_test() {
        // The ONLY place `checked_ih_slots_wrapper` does anything: it gates the
        // heterogeneous test off. Same producer, two wrappers, opposite verdicts.
        let inner = ordinary_consumer_body(RuntimeExpr::Var(0));
        let args = vec![heterogeneous_call_producer()];

        let bare = classify_immediate_bridge(&inner, &args, 0)
            .expect("the bare Match wrapper must classify as heterogeneous");
        assert_eq!(bare.cause, ImmediateBridgeCause::Heterogeneous);
        assert!(!bare.checked_ih_slots_wrapper);

        let wrapped = RuntimeExpr::CheckedComputationalIHSlots {
            slot_template_ids: Vec::new(),
            checked_occurrence_paths: Vec::new(),
            body: Box::new(inner),
        };
        assert!(
            classify_immediate_bridge(&wrapped, &args, 0).is_none(),
            "the IH-slots wrapper must suppress the heterogeneous cause, and \
             with no other cause available the classification must refuse"
        );
    }

    // ─── `AC-7`: one case per refusal point. Six of the seven are structural or
    // arithmetic and are authored freely -- the negative answer has an obvious
    // shape and no semantic verdict is being guessed. Only the last needs a
    // producer whose verdict is attested, and it reuses `inert_producer`.

    #[test]
    fn refuses_a_body_that_is_none_of_the_four_spellings() {
        // `:190` at `0f71ab5b9` -- the `_ => return None` arm of the dispatch.
        let args = vec![heterogeneous_call_producer()];
        assert!(classify_immediate_bridge(&RuntimeExpr::Var(0), &args, 0).is_none());
    }

    #[test]
    fn refuses_a_checked_subcontinuation_frame_whose_inner_form_is_wrong() {
        // `:163` -- the `return None` inside the frame arm. The frame admits
        // ONLY a ComputationalMatch inside; `:156` is the arm head, not the refusal.
        let args = vec![heterogeneous_call_producer()];
        let body = RuntimeExpr::CheckedSubcontinuationFrame {
            frame_id: 7,
            body: Box::new(ordinary_consumer_body(RuntimeExpr::Var(0))),
        };
        assert!(
            classify_immediate_bridge(&body, &args, 0).is_none(),
            "an ordinary Match inside the frame is the wrong inner form"
        );
    }

    #[test]
    fn refuses_an_ih_slots_wrapper_whose_inner_form_is_wrong() {
        // `:182` -- the `return None` inside the IH-slots arm. That wrapper admits
        // ONLY an ordinary Match inside; `:175` is the arm head, not the refusal.
        let args = vec![heterogeneous_call_producer()];
        let body = RuntimeExpr::CheckedComputationalIHSlots {
            slot_template_ids: Vec::new(),
            checked_occurrence_paths: Vec::new(),
            body: Box::new(RuntimeExpr::Var(0)),
        };
        assert!(classify_immediate_bridge(&body, &args, 0).is_none());
    }

    #[test]
    fn refuses_a_scrutinee_that_is_not_a_var() {
        // `:192`.
        let args = vec![heterogeneous_call_producer()];
        let body = ordinary_consumer_body(construct("ctor:prelude::Bool::True", Vec::new()));
        assert!(classify_immediate_bridge(&body, &args, 0).is_none());
    }

    #[test]
    fn refuses_when_the_binder_offset_underflows_the_scrutinee_index() {
        // `:196` -- `index.checked_sub(argument_binder_offset)` returns None.
        let args = vec![heterogeneous_call_producer()];
        let body = ordinary_consumer_body(RuntimeExpr::Var(1));
        assert!(
            classify_immediate_bridge(&body, &args, 2).is_none(),
            "offset 2 against Var(1) must underflow rather than wrap"
        );
    }

    #[test]
    fn refuses_when_the_selected_field_is_past_the_end_of_the_producer_args() {
        // `:197` -- `producer_args.get(selected_field)` returns None.
        let args = vec![heterogeneous_call_producer()];
        let body = ordinary_consumer_body(RuntimeExpr::Var(4));
        assert!(classify_immediate_bridge(&body, &args, 0).is_none());
    }

    #[test]
    fn refuses_when_neither_cause_holds() {
        // `:205`. The producer is structurally inert (see `inert_producer`) and
        // the consumer is an ordinary Match whose cases cannot statically select
        // it, so both causes are false and the classification refuses rather
        // than returning a selection with no cause.
        let args = vec![inert_producer()];
        let body = ordinary_consumer_body(RuntimeExpr::Var(0));
        assert!(classify_immediate_bridge(&body, &args, 0).is_none());
    }

    /// NEW INTENT -- NO PLAN-LEVEL ANCESTOR, AND THIS IS THE FIRST LIVE
    /// APPLICATION OF THAT LABEL.
    ///
    /// `statically_selects_host_operation` is named NOWHERE outside this module
    /// on the reference branch: two hits across `crates/`, the call at `:199`
    /// and the definition at `:215`. No test drives it, so the expected verdict
    /// below is the author's reading of the function and is **not** attested.
    /// It is labelled here rather than left to look like the lifted cases.
    ///
    /// The cause has no consumer until the plan-coupled successor slice lands;
    /// that slice is where it can be attested rather than authored.
    #[test]
    fn new_intent_static_host_operation_cause() {
        let producer = construct("ctor:fixture::Seat::Mk", Vec::new());
        let body = RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Var(0)),
            cases: vec![RuntimeMatchCase {
                constructor: "ctor:fixture::Seat::Mk".to_string(),
                binders: 0,
                body: RuntimeExpr::Let {
                    value: Box::new(RuntimeExpr::Effect {
                        family: "eff:fixture::Console".to_string(),
                        operation: ken_host::HostOpV1::ConsoleRead,
                        capability: None,
                        args: Vec::new(),
                    }),
                    body: Box::new(RuntimeExpr::Var(0)),
                },
            }],
            default: trap("static host operation fixture selected no case"),
        };
        let args = [producer];
        let selection = classify_immediate_bridge(&body, &args, 0)
            .expect("a statically selected case reaching a host effect must classify");
        assert_eq!(selection.cause, ImmediateBridgeCause::StaticHostOperation);
    }
}
