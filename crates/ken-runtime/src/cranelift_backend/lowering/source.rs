//! Source-machine vocabulary and control — the states, continuations and
//! dispatch for a lowered value evaluated through the source machine.
//!
//! `RT-SOURCE-MACHINE-TYPES-SPLIT` `D1`. Extends the `boundary.rs` seam
//! (item 11): the owner's control (`core.rs`'s dispatch cluster) and its own
//! state types (`SourceMachineState`, `SourceContinuation`,
//! `SourceContinuationTerminal`, `SourceCallOutcome`, moved from `mod.rs`)
//! relocate here. Every other type the moving methods manipulate
//! (`SourcePrefixTemplate`, `SourceControl`, `SourceSelectedContinuation`,
//! `CheckedApplicationDisposition`, `ScalarMergeKind`, and siblings) stays
//! declared at the `mod.rs` hub — traced usage shows each is shared with
//! retained checked-invocation/continuation-frame or value-merge machinery,
//! not exclusive to this cluster — matching `boundary.rs`'s own
//! hub-stays/methods-move shape (item 10) rather than widening a shared type's
//! home for this slice's convenience.
//!
//! `RT-SOURCE-MACHINE-TYPES-SPLIT` `D2` narrowed the transitional widenings
//! `D1` ledgered (item-9 discipline: ledger at the move, discharge once the
//! reason clears). `install_recursor_invocation` needed `pub(super)` only for
//! the four `core/tests/control.rs` tests that reached it directly; `D2`
//! moved those tests into this file's own `tests` module (below), so it is
//! private again. `SourceContinuation`/`SourceContinuationTerminal` and
//! `lower_source_machine`/`lower_source_machine_with_continuation` keep
//! `pub(super)` — each has a SURVIVING caller the test move did not touch:
//! `SourceContinuation` is a field of `mod.rs`'s retained `SourceControl` and
//! is also constructed directly by `core/tests/constructors.rs` and
//! `core/tests/effects.rs` (neither in the `D0` move population);
//! `SourceContinuationTerminal` likewise by `constructors.rs`;
//! `lower_source_machine` is called from two retained core.rs sites
//! (`lower_computational_producer_expr`'s and its sibling's recursive-position
//! arms); `lower_source_machine_with_continuation` is called directly by
//! `constructors.rs`. `SourceCarriedControlMutation`/
//! `with_source_carried_control_mutation` keep `pub(super)` for the same
//! reason: `constructors.rs` constructs and calls them directly (four sites),
//! and `constructors.rs`'s own tests were never part of the `D0`/`D2` move
//! population (that ledger scoped `AC-2` to `control.rs` only).

use super::*;
use super::core::CheckedFrameBranchScope;

pub(super) enum SourceContinuation<'a> {
    Terminal(SourceContinuationTerminal<'a>),
    CheckedRecursiveInvocationReturn {
        instance: CheckedRecursiveInvocationInstance,
        next: Box<SourceContinuation<'a>>,
    },
    CheckedComputationalIHInvocationReturn {
        call_template_id: u64,
        next: Box<SourceContinuation<'a>>,
    },
    ReturnFromSelectedCase {
        delimiter: SelectedCaseReturnDelimiter,
        next: Box<SourceContinuation<'a>>,
    },
    LetBody {
        body: OwnedSourceOccurrence,
        env: Vec<LoweringEnvironmentBinding>,
        next: Box<SourceContinuation<'a>>,
    },
    ApplyRecursorSelection {
        layer: ComputationalRecursorLayer,
        next: Box<SourceContinuation<'a>>,
    },
    UnwindRecursorSegment {
        stack: RecursorUnwindStack,
        resume_cursor: ContinuationCursorId,
        next: Box<SourceContinuation<'a>>,
    },
    IfScrutinee {
        then_expr: OwnedSourceOccurrence,
        else_expr: OwnedSourceOccurrence,
        env: Vec<LoweringEnvironmentBinding>,
        next: Box<SourceContinuation<'a>>,
    },
    ConstructArgument {
        constructor: RuntimeSymbol,
        static_origin: StaticOriginId,
        remaining: Vec<OwnedSourceOccurrence>,
        lowered: Vec<LoweringOperand>,
        env: Vec<LoweringEnvironmentBinding>,
        next: Box<SourceContinuation<'a>>,
    },
    /// ⭐ `static_origin` is the **match occurrence's own** origin, carried in
    /// the same constructor as the cloned cases. Case *i*'s body is derived from
    /// it positionally at the point of use (`child(static_origin, 1 + i)`).
    ///
    /// A parallel `Vec<StaticOriginId>` beside `cases` would be the obvious
    /// alternative and is worse: two vectors can desync, and a desync is
    /// undetectable here. One parent origin cannot.
    MatchScrutinee {
        cases: Vec<crate::RuntimeMatchCase>,
        default: RuntimeTrap,
        env: Vec<LoweringEnvironmentBinding>,
        static_origin: StaticOriginId,
        next: Box<SourceContinuation<'a>>,
    },
    ComputationalMatchScrutinee {
        cases: Vec<crate::RuntimeComputationalMatchCase>,
        default: RuntimeTrap,
        env: Vec<LoweringEnvironmentBinding>,
        static_origin: StaticOriginId,
        provenance: RecursorFrameProvenance,
        checked_frame_id: Option<u64>,
        answer_route: SourceComputationalAnswerRoute,
        next: Box<SourceContinuation<'a>>,
    },
    ProjectRecord {
        field: String,
        next: Box<SourceContinuation<'a>>,
    },
    CallCallee {
        args: Vec<OwnedSourceOccurrence>,
        env: Vec<LoweringEnvironmentBinding>,
        next: Box<SourceContinuation<'a>>,
    },
    CallArgument {
        callee: SourceCallee,
        remaining: Vec<OwnedSourceOccurrence>,
        lowered: Vec<LoweringOperand>,
        env: Vec<LoweringEnvironmentBinding>,
        next: Box<SourceContinuation<'a>>,
    },
}

pub(super) enum SourceContinuationTerminal<'a> {
    ReturnValue,
    /// The unique affine handoff from source evaluation back to the producer.
    /// The stored unwind segment is consumed here; it is not inferred from
    /// provenance or reconstructed from the cursor.
    ReturnToProducerHole {
        stack: RecursorUnwindStack,
        resume_cursor: ContinuationCursorId,
        expected: ContinuationCursorId,
        active: &'a ActiveContinuationFrame<'a>,
        root_authority: Option<RootTerminalAnswerAuthority>,
    },
    ResumeOuter {
        expected: ContinuationCursorId,
        active: &'a ActiveContinuationFrame<'a>,
        root_authority: Option<RootTerminalAnswerAuthority>,
    },
    JumpToJoin(SourcePredecessorEdge<'a>),
}

enum SourceMachineState<'a> {
    /// A pending expression the machine will evaluate.
    ///
    /// ⭐ This is the state the source-machine fallback arm feeds
    /// (`core.rs:2074`, `other => …lower_expr(builder, &other, &env)`), which
    /// hands over every form the machine's own dispatcher does not handle —
    /// closures included. That is why the origin has to be here rather than in a
    /// guessed subset of the frames: the machine and the direct descent are the
    /// same population reached two ways.
    Eval {
        expr: OwnedSourceOccurrence,
        env: Vec<LoweringEnvironmentBinding>,
        control: SourceControl<'a>,
    },
    Value {
        /// **`D6a` upstream half** -- the operand **and the route it arrived
        /// by**. ⛔ Not two independent facts: the route is a property of this
        /// exact predecessor, so pairing them here is what stops a later seat
        /// from having to guess which predecessor a value came from.
        value: RoutedAnswer,
        control: SourceControl<'a>,
    },
}

enum SourceCallOutcome<'a> {
    Continue(SourceMachineState<'a>),
    Complete(LoweringOperand),
}


/// `RT-CONTSRC-PRODUCER-LOCAL` `AC-1` -- the source-carried CONTROL mutation
/// family, for the activation-gate controls of families 5 and 2a.
///
/// Test-only, and deliberately NOT a widening of `D7`'s
/// `EffectSeatDispatchMutation`: that family perturbs effect-seat dispatch and
/// these two perturb the carried `Match` route. Sharing one enum would let a
/// control claim a mutation it did not apply.
///
/// Each variant refuses **after the real production decision has already been
/// taken**, and does nothing else. It never manufactures a carrier, terminal or
/// planner fact, never rewrites a join target, never duplicates the dispatch and
/// never lowers an alternative. That is what makes an application evidence that
/// the production path reached that exact point, rather than evidence about the
/// mutation's own machinery.
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SourceCarriedControlMutation {
    /// Production behaviour.
    Exact,
    /// Family 5 -- refuse an ALREADY-CLASSIFIED `LoweringOperand::Carried` with
    /// the exact refusal the pre-repair seat produced.
    RefuseClassifiedCarried,
    /// Family 2a -- refuse an ALREADY-SPLIT `SourcePrefixTerminal::Join` before
    /// its inherited target is used.
    RefuseSplitInheritedJoin,
}

#[cfg(test)]
thread_local! {
    static SOURCE_CARRIED_CONTROL_MUTATION: std::cell::Cell<SourceCarriedControlMutation> =
        const { std::cell::Cell::new(SourceCarriedControlMutation::Exact) };
    /// How many times the active mutation actually fired.
    ///
    /// This is the anti-vacuity instrument. A mutated run that refuses with
    /// the right message but a count of zero refused for some OTHER reason, and
    /// is not evidence.
    static SOURCE_CARRIED_CONTROL_APPLICATIONS: std::cell::Cell<u32> =
        const { std::cell::Cell::new(0) };
}

/// Restores `Exact` on drop, so a panicking control cannot leak a mutation into
/// the next test on this thread.
#[cfg(test)]
struct SourceCarriedControlMutationGuard;

#[cfg(test)]
impl Drop for SourceCarriedControlMutationGuard {
    fn drop(&mut self) {
        SOURCE_CARRIED_CONTROL_MUTATION.with(|cell| cell.set(SourceCarriedControlMutation::Exact));
    }
}

/// Run `body` under `mutation`, returning its value and the number of times the
/// mutation fired. The counter is reset on entry, and `Exact` is restored on
/// exit even if `body` panics.
#[cfg(test)]
pub(super) fn with_source_carried_control_mutation<R>(
    mutation: SourceCarriedControlMutation,
    body: impl FnOnce() -> R,
) -> (R, u32) {
    let _guard = SourceCarriedControlMutationGuard;
    SOURCE_CARRIED_CONTROL_APPLICATIONS.with(|cell| cell.set(0));
    SOURCE_CARRIED_CONTROL_MUTATION.with(|cell| cell.set(mutation));
    let value = body();
    let applications = SOURCE_CARRIED_CONTROL_APPLICATIONS.with(std::cell::Cell::get);
    (value, applications)
}

/// `Some(refusal)` when `mutation` is the active one; the caller returns it
/// unchanged. Counting happens here so a hook cannot fire without being counted.
#[cfg(test)]
fn source_carried_control_refusal(
    mutation: SourceCarriedControlMutation,
    construct: &'static str,
    reason: &'static str,
) -> Option<CraneliftBackendError> {
    if SOURCE_CARRIED_CONTROL_MUTATION.with(std::cell::Cell::get) != mutation {
        return None;
    }
    SOURCE_CARRIED_CONTROL_APPLICATIONS.with(|cell| cell.set(cell.get() + 1));
    Some(unsupported(construct, reason))
}

/// The status the carried source-machine `Match` returns when the boundary
/// word's CLASS is one this case set never decoded.
///
/// Named locally and deliberately: this is NOT claimed to be a canonical
/// carrier-wide failure word. It reuses the value the dynamic-constructor
/// emitter already returns for a malformed represented value, because both mean
/// *"this word is not the representation this chain decodes"*.
///
/// NOTHING PINS THIS VALUE, and an earlier draft of this comment claimed a
/// wrong-class control did. It does not exist and cannot be written today:
/// `mismatch_block` is emitted on the residual arm of the class chain, and a
/// sentinel sweep of the whole `ken-runtime` lib suite measured that arm as
/// reached ZERO times. `lower_source_carried_match` is entered exactly once in
/// the crate -- by
/// `constructors::ac1_source_machine_match_classifies_a_carried_scrutinee_by_phase`
/// -- and that entry refuses at join acquisition, before any selector is
/// emitted. So a divergence in this value is a SILENT DRIFT, not a test
/// failure. Retiring this paragraph needs a fixture that reaches the arm, not
/// a re-reading of the code.
const CARRIED_REPRESENTATION_MISMATCH_STATUS: i64 = MALFORMED_DYNAMIC_CONSTRUCTOR_STATUS;

/// One emitted-case descriptor for the carried source-machine `Match`.
struct SourceCarriedCase {
    index: usize,
    emitted: bool,
    identity: u64,
    binders: i64,
    borrowed: Option<(i64, usize)>,
}



/// **`RT-RECURSOR-TRANSPORT` `D2` trace helpers.** Test-only.
///
/// The ordered continuation stack, top first. `SourceContinuation` has no
/// `Debug`, and the ruling asks for *ordered kinds* rather than a rendering of
/// the payloads, so this walks the `next` chain and names each frame.
#[cfg(test)]
fn rt_continuation_kinds(continuation: &SourceContinuation<'_>) -> Vec<&'static str> {
    let mut kinds = Vec::new();
    let mut cursor = continuation;
    loop {
        let (kind, next): (&'static str, Option<&SourceContinuation<'_>>) = match cursor {
            SourceContinuation::Terminal(_) => ("Terminal", None),
            SourceContinuation::CheckedRecursiveInvocationReturn { next, .. } => {
                ("CheckedRecursiveInvocationReturn", Some(next))
            }
            SourceContinuation::CheckedComputationalIHInvocationReturn { next, .. } => {
                ("CheckedComputationalIHInvocationReturn", Some(next))
            }
            SourceContinuation::ReturnFromSelectedCase { next, .. } => {
                ("ReturnFromSelectedCase", Some(next))
            }
            SourceContinuation::LetBody { next, .. } => ("LetBody", Some(next)),
            SourceContinuation::ApplyRecursorSelection { next, .. } => {
                ("ApplyRecursorSelection", Some(next))
            }
            SourceContinuation::UnwindRecursorSegment { next, .. } => {
                ("UnwindRecursorSegment", Some(next))
            }
            SourceContinuation::IfScrutinee { next, .. } => ("IfScrutinee", Some(next)),
            SourceContinuation::ConstructArgument { next, .. } => ("ConstructArgument", Some(next)),
            SourceContinuation::MatchScrutinee { next, .. } => ("MatchScrutinee", Some(next)),
            SourceContinuation::ComputationalMatchScrutinee { next, .. } => {
                ("ComputationalMatchScrutinee", Some(next))
            }
            SourceContinuation::ProjectRecord { next, .. } => ("ProjectRecord", Some(next)),
            SourceContinuation::CallCallee { next, .. } => ("CallCallee", Some(next)),
            SourceContinuation::CallArgument { next, .. } => ("CallArgument", Some(next)),
        };
        kinds.push(kind);
        match next {
            Some(next) => cursor = next,
            None => break,
        }
    }
    kinds
}

/// An operand's PHASE and concrete value kind, which the ruling asks for
/// separately: "carried" and "specialized as a constructor" are different
/// facts and conflating them is how a non-constructor reads as fine.
#[cfg(test)]
fn rt_operand_desc(operand: &LoweringOperand) -> String {
    match operand {
        LoweringOperand::Carried(_) => "phase=Carried kind=<carried word>".to_string(),
        LoweringOperand::Specialized(lowered) => {
            format!("phase=Specialized kind={}", lowered_value_kind(lowered))
        }
    }
}


// `RT-SOURCE-MACHINE-TYPES-SPLIT` `D1` — split out of a shared
// `thread_local!` block in `core.rs` that also held the `CCR_D2_*` counters
// (a different, retained domain, which stayed there). Splitting a
// `thread_local!` macro invocation along a domain boundary is not a semantic
// change: each block declares independent TLS statics regardless of which
// invocation groups them syntactically.
#[cfg(test)]
thread_local! {
    /// `RT-LEXICAL-RECURSOR-CONSUMERS` `D2a` — backedge markers that ARRIVED at
    /// `ComputationalMatchScrutinee`. The denominator: without it, "the forward
    /// happened" and "the marker never reached this continuation" are the two
    /// readings a green row cannot separate.
    static LRC_D2A_BACKEDGE_ARRIVALS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// Arrivals actually forwarded by the new seat.
    static LRC_D2A_BACKEDGE_FORWARDS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// `D3`'s mutation: suppress the forward, restoring the pre-`D2a` refusal.
    static LRC_D2A_SUPPRESS_FORWARD: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    /// `D2b` — the abandoned-`Let` disposition mutation seat.
    static LRC_D2B_LET_DISPOSITION: std::cell::Cell<LrcD2bLetDisposition> =
        const { std::cell::Cell::new(LrcD2bLetDisposition::Exact) };
}

/// `D2b` — how the abandoned-`Let` arm selects its disposition subtree.
///
/// ⛔ Each variant is a selector a wrong derivation would plausibly produce, so
/// a refusal is attributable to the selector rather than to a rewritten arm.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum LrcD2bLetDisposition {
    /// Production: the planner's retained body root.
    Exact,
    /// The arm does nothing -- the pre-repair state.
    Suppress,
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn set_lrc_d2b_let_disposition(mode: LrcD2bLetDisposition) {
    LRC_D2B_LET_DISPOSITION.with(|cell| cell.set(mode));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn lrc_d2b_let_disposition() -> LrcD2bLetDisposition {
    LRC_D2B_LET_DISPOSITION.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn lrc_d2a_backedge_arrivals() -> usize {
    LRC_D2A_BACKEDGE_ARRIVALS.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn lrc_d2a_backedge_forwards() -> usize {
    LRC_D2A_BACKEDGE_FORWARDS.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_lrc_d2a_counts() {
    LRC_D2A_BACKEDGE_ARRIVALS.with(|count| count.set(0));
    LRC_D2A_BACKEDGE_FORWARDS.with(|count| count.set(0));
}

/// `D3`'s mutation seat: suppress the `D2a` forward so the pre-repair refusal
/// is producible again. ⛔ Test-only, and never set in production.
#[cfg(test)]
pub(in crate::cranelift_backend) fn set_lrc_d2a_suppress_forward(suppress: bool) {
    LRC_D2A_SUPPRESS_FORWARD.with(|cell| cell.set(suppress));
}


/// **`D8f`** — let the DECLINED call answer for the checked application's
/// composed causal identity, which is what it did before this checkpoint.
///
/// The call itself is unchanged either way; only the claim moves. That is what
/// makes the duplicate-discharge refusal a difference this checkpoint removed
/// rather than one it routed around.
#[cfg(test)]
thread_local! {
    static D8F_DECLINED_CALL_CLAIMS: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn set_d8f_declined_call_claims(armed: bool) {
    D8F_DECLINED_CALL_CLAIMS.with(|cell| cell.set(armed));
}

#[cfg(test)]
fn d8f_declined_call_claims() -> bool {
    D8F_DECLINED_CALL_CLAIMS.with(std::cell::Cell::get)
}


impl<'a> Lowering<'a> {
    /// Record a specialized/default selection made after the source-machine
    /// continuation resumes a computational match.
    ///
    /// The separate seam is test-visible because an initial constructor
    /// selection and a recursive revisit use different lowering routes. A
    /// mutation must be able to remove only the revisit edge while preserving
    /// the initial population entry and the generated-function closure check.
    fn record_source_machine_computational_match_selection(
        &mut self,
        match_origin: StaticOriginId,
        selected_case: Option<usize>,
    ) -> Result<(), CraneliftBackendError> {
        #[cfg(test)]
        if D8_JOIN_CONSUMPTION_MUTATION.with(std::cell::Cell::get)
            == JoinConsumptionMutation::OmitSourceMachineComputationalMatchSelection
        {
            self.function_local
                .emission_reachable_match_cases
                .entry(match_origin)
                .or_default();
            return Ok(());
        }
        self.disposition_statically_unselected_match_cases(match_origin, selected_case)
    }

}

impl<'a> Lowering<'a> {
    /// Carry a source-machine call's inputs across a **declared generated
    /// unit** boundary.
    ///
    /// ⛔ **No per-argument occurrence pairing, and its removal is the point.**
    /// The pairing existed so that an aggregate argument's planned ownership
    /// record would be resolved at the argument rather than at the callee's
    /// scheduling entry. That question no longer has a coordinate answer:
    /// ownership travels on the template and the schema is recovered from it,
    /// so the accumulator, the retained pending occurrence and the prefix
    /// template that mirrored it were carrying a fact nothing read.
    fn carry_source_call_inputs(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        origin: StaticOriginId,
        inputs: Vec<LoweringOperand>,
    ) -> Result<Vec<LoweringOperand>, CraneliftBackendError> {
        // ⭐ **`D7` — the A/B AGGREGATE-OWNERSHIP discriminator's only seam.**
        // It moves the certificate the template carries; under it one argument
        // keeps its value, its callee, its parameter slot, its shape and its
        // lane, and takes a SIBLING argument's producer occurrence, so a
        // refusal is attributable to the certificate alone.
        #[cfg(test)]
        let inputs = self.substitute_sibling_aggregate_producer(inputs);
        // ⭐ **`D7` — the call-USE coordinate discriminator, at the same seam.**
        // It moves the coordinate every input is transferred at while the
        // certificate mutation above moves the certificate one input carries.
        // The diagnostic callee is derived from the pre-mutation scheduling
        // entry, so it is not a third consumer of the mutated coordinate.
        // Same call, same arguments, same moment, two mutation axes.
        #[cfg(test)]
        let callee = self.generated_unit_call_entry_callee(origin);
        #[cfg(test)]
        let origin = self.call_input_transfer_origin_under_mutation(origin)?;
        let mut carried = Vec::with_capacity(inputs.len());
        for input in inputs {
            carried.push(self.carry_call_input(
                builder,
                origin,
                input,
                #[cfg(test)]
                GeneratedUnitCallInputCaller::SourceMachineDeclaredUnit,
                #[cfg(test)]
                callee,
            )?);
        }
        Ok(carried)
    }

}

impl<'a> Lowering<'a> {
    pub(super) fn lower_source_machine(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        occurrence: SourceOccurrence<'_>,
        env: &[LoweringEnvironmentBinding],
        active: &ActiveContinuationFrame<'_>,
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        let mut root_authority = self.root_terminal_authority.take();
        if let Some(authority) = &mut root_authority {
            match authority.outer_cursor {
                None => authority.outer_cursor = Some(active.cursor),
                Some(cursor) if cursor == active.cursor => {}
                Some(_) => {
                    return Err(unsupported(
                        "NativeJoinPlanV1",
                        "checked root answer authority was transplanted to another outer cursor",
                    ));
                }
            }
        }
        let control = SourceControl {
            continuation: SourceContinuation::Terminal(SourceContinuationTerminal::ResumeOuter {
                expected: active.cursor,
                active,
                root_authority,
            }),
            selected: SourceSelectedContinuation {
                activation: active.activation,
                cursor: active.cursor,
                parent: active.parent,
                pending: active.pending.to_vec(),
                selected_ancestry: active.selected_ancestry.to_vec(),
                selected_scope: active.selected_scope.cloned(),
            },
            selected_lineage: Vec::new(),
            terminal_outer: active.cursor,
        };
        self.lower_source_machine_with_continuation(
            builder,
            OwnedSourceOccurrence::cloned(occurrence),
            env.to_vec(),
            control,
        )
    }

    pub(super) fn lower_source_machine_with_continuation<'b>(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        expr: OwnedSourceOccurrence,
        env: Vec<LoweringEnvironmentBinding>,
        control: SourceControl<'b>,
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        let previous_source_root = self.source_control_root.replace(control.terminal_outer);
        self.live_source_continuations = self
            .live_source_continuations
            .checked_add(1)
            .expect("compiler-private live source-continuation depth exhausted");
        let result = self.lower_source_machine_with_continuation_inner(builder, expr, env, control);
        self.live_source_continuations = self
            .live_source_continuations
            .checked_sub(1)
            .expect("source-continuation depth must balance");
        self.source_control_root = previous_source_root;
        result
    }

    fn lower_source_machine_with_continuation_inner<'b>(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        expr: OwnedSourceOccurrence,
        env: Vec<LoweringEnvironmentBinding>,
        control: SourceControl<'b>,
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        let mut state = SourceMachineState::Eval { expr, env, control };
        loop {
            state = match state {
                SourceMachineState::Eval {
                    expr:
                        OwnedSourceOccurrence {
                            expr,
                            static_origin,
                        },
                    env,
                    mut control,
                } => match {
                    // The owned source machine is the third traversal route for
                    // these joins. Record the source occurrence here; later
                    // continuation helpers may only reborrow its token.
                    self.enter_source_occurrence_plan(static_origin)?;
                    expr
                } {
                    RuntimeExpr::CheckedSubcontinuationFrame { frame_id, body } => {
                        self.enter_checked_subcontinuation_frame(frame_id)?;
                        SourceMachineState::Eval {
                            expr: self.owned_child_occurrence(static_origin, 0, *body)?,
                            env,
                            control,
                        }
                    }
                    RuntimeExpr::CheckedRecursiveInvocation {
                        call_template_id,
                        body,
                        ..
                    } => {
                        let instance =
                            self.enter_checked_recursive_invocation(call_template_id, &body)?;
                        control.continuation =
                            SourceContinuation::CheckedRecursiveInvocationReturn {
                                instance,
                                next: Box::new(control.continuation),
                            };
                        SourceMachineState::Eval {
                            expr: self.owned_child_occurrence(static_origin, 0, *body)?,
                            env,
                            control,
                        }
                    }
                    RuntimeExpr::CheckedComputationalIHSlots { body, .. } => {
                        SourceMachineState::Eval {
                            expr: self.owned_child_occurrence(static_origin, 0, *body)?,
                            env,
                            control,
                        }
                    }
                    RuntimeExpr::CheckedComputationalIHInvocation {
                        call_template_id,
                        kind,
                        binder_morphism,
                        body,
                        ..
                    } => {
                        // `D8f` — the machine's own child derivation, taken
                        // BEFORE the marker is entered so the same occurrence is
                        // recorded and evaluated.
                        let body = self.owned_child_occurrence(static_origin, 0, *body)?;
                        self.enter_checked_computational_ih_invocation(
                            call_template_id,
                            kind,
                            binder_morphism,
                            &body.expr,
                            body.static_origin,
                        )?;
                        control.continuation =
                            SourceContinuation::CheckedComputationalIHInvocationReturn {
                                call_template_id,
                                next: Box::new(control.continuation),
                            };
                        SourceMachineState::Eval {
                            expr: body,
                            env,
                            control,
                        }
                    }
                    RuntimeExpr::Value(value) => SourceMachineState::Value {
                        value: RoutedAnswer::direct(LoweringOperand::Specialized(
                            self.lower_value(builder, &value)?,
                        )),
                        control,
                    },
                    // Same value-producing rule as the direct descent's `Var`:
                    // only `Value` yields a machine value, and a static worker
                    // binding fails closed here rather than entering one.
                    RuntimeExpr::Var(index) => SourceMachineState::Value {
                        value: RoutedAnswer::direct(
                            env.get(index as usize)
                                .ok_or_else(|| {
                                    unsupported(
                                        "Var",
                                        format!("no runtime binding for index {index}"),
                                    )
                                })?
                                .value_at({
                                    #[cfg(test)]
                                    crate::cranelift_backend::lowering::record_d2k_owner_event(
                                        crate::cranelift_backend::lowering::D2kOwnerEvent::ValueAtCaller { site: "core.rs source-machine Var" },
                                    );
                                    "a source-machine Var in value position"
                                })?
                                .clone(),
                        ),
                        control,
                    },
                    RuntimeExpr::Let { value, body } => {
                        control.continuation = SourceContinuation::LetBody {
                            body: self.owned_child_occurrence(static_origin, 1, *body)?,
                            env: env.clone(),
                            next: Box::new(control.continuation),
                        };
                        SourceMachineState::Eval {
                            expr: self.owned_child_occurrence(static_origin, 0, *value)?,
                            env: env.clone(),
                            control,
                        }
                    }
                    RuntimeExpr::Construct {
                        constructor,
                        mut args,
                    } => {
                        // `RT-SRCMACHINE-CTOR-RECOGNITION-ARM` -- ask the same
                        // classifier as direct descent before the source
                        // machine lowers any field. D1 established that every
                        // eligible state arrives with the complete argument
                        // run and no pending constructor continuation, so the
                        // existing template can open its conservation ledger
                        // without restructuring partial machine state.
                        let recognized =
                            Self::recognized_constructor_worker_fields(&args, &env);
                        if recognized.iter().any(Option::is_some) {
                            SourceMachineState::Value {
                                value: RoutedAnswer::direct(
                                    LoweringOperand::Specialized(
                                        self.static_worker_constructor_template(
                                            builder,
                                            static_origin,
                                            &constructor,
                                            &args,
                                            &recognized,
                                            &env,
                                        )?,
                                    ),
                                ),
                                control,
                            }
                        } else if args.is_empty() {
                            SourceMachineState::Value {
                                value: RoutedAnswer::direct(LoweringOperand::Specialized(
                                    self.finish_source_constructor(
                                        builder,
                                        constructor,
                                        static_origin,
                                        vec![],
                                    )?,
                                )),
                                control,
                            }
                        } else {
                            // Argument *i* is child *i*; the suffix keeps each
                            // pending term paired with its own origin, so the
                            // machine's positions cannot drift as it consumes them.
                            let first = args.remove(0);
                            let remaining = args
                                .into_iter()
                                .enumerate()
                                .map(|(offset, arg)| {
                                    self.owned_child_occurrence(static_origin, 1 + offset, arg)
                                })
                                .collect::<Result<Vec<_>, _>>()?;
                            control.continuation = SourceContinuation::ConstructArgument {
                                constructor,
                                static_origin,
                                remaining,
                                lowered: Vec::new(),
                                env: env.clone(),
                                next: Box::new(control.continuation),
                            };
                            SourceMachineState::Eval {
                                expr: self.owned_child_occurrence(static_origin, 0, first)?,
                                env,
                                control,
                            }
                        }
                    }
                    RuntimeExpr::Match {
                        scrutinee,
                        cases,
                        default,
                    } => {
                        control.continuation = SourceContinuation::MatchScrutinee {
                            cases,
                            default,
                            env: env.clone(),
                            static_origin,
                            next: Box::new(control.continuation),
                        };
                        SourceMachineState::Eval {
                            expr: self.owned_child_occurrence(static_origin, 0, *scrutinee)?,
                            env,
                            control,
                        }
                    }
                    RuntimeExpr::Call { callee, args } => {
                        let args = args
                            .into_iter()
                            .enumerate()
                            .map(|(position, arg)| {
                                self.owned_child_occurrence(static_origin, 1 + position, arg)
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        // **`RT-CONTSRC-PRODUCER-LOCAL` `D8e` — THE SOLE
                        // SOURCE-MACHINE CONSUMER of a `D8d` binding.**
                        //
                        // ⛔ It sits **ahead of the callee's own evaluation**,
                        // and that placement is the mechanism, not a
                        // convenience. Evaluating a `Var` callee first routes it
                        // through the machine's value arm, which calls
                        // `value_at` and fails closed on a static worker by
                        // design. So the binding is either consumed here or
                        // refused everywhere — there is no third outcome, and a
                        // `Var` resolving to `Value` falls through to the
                        // pre-existing route untouched.
                        //
                        // ⭐ Deliberately the same shape as the direct descent's
                        // sole consumer: an exact `Var`, read out of the
                        // environment by index, no shape inference and no
                        // planner query. The environment already holds the
                        // answer because `D8d` put it there; asking the planner
                        // again here would be the consumer-side target lookup
                        // this checkpoint excludes, and a second authority for
                        // one binding.
                        let static_worker = match callee.as_ref() {
                            RuntimeExpr::Var(index) => match env.get(*index as usize) {
                                Some(LoweringEnvironmentBinding::StaticWorker(worker)) => {
                                    Some((u64::from(*index), worker.clone()))
                                }
                                _ => None,
                            },
                            _ => None,
                        };
                        if let Some((binder_index, worker)) = static_worker {
                            // `D2k-1b-i` — the source machine's terminal
                            // disposition, counted into the same conservation
                            // ledger as the direct descent's.
                            self.static_worker_fields
                            .note_consuming_call(worker.transport, static_origin, self.defining_function_id)?;
                            #[cfg(test)]
                            d8e_record_consumption();
                            // `D8l2` — which facet this consumption carried,
                            // recorded beside the count so "three consumptions"
                            // can be attributed rather than merely counted.
                            #[cfg(test)]
                            crate::cranelift_backend::lowering::record_d8l2_consumed_facet(
                                matches!(
                                    worker.discharge,
                                    ContinuationDischarge::ComposedSourceContinuation(_)
                                ),
                            );
                            // Arguments are evaluated under the machine's own
                            // control and phase, exactly as for a value callee;
                            // only the completion differs.
                            let mut remaining = args;
                            if remaining.is_empty() {
                                // **`D8p` — THE CHECKED-APPLICATION SEAM, on the
                                // source machine's call edge.**
                                //
                                // The direct descent has consulted it since
                                // `D5a`; this edge did not, so a checked-IH
                                // marker entered in a body whose application the
                                // SOURCE MACHINE lowers could never be consumed
                                // and failed closed at the marker's close. The
                                // seam is the same function, on the same exact
                                // occurrence and binder ordinal -- no second
                                // authority, no target lookup, and no new
                                // identity.
                                //
                                // ⛔ Immediately BEFORE the call is written, so
                                // consumption still precedes the instruction it
                                // discharges, and after the arguments are in
                                // hand so an ordinary selected-argument call
                                // reaches the seat first and leaves the marker
                                // for the occurrence that owns it.
                                let pending = self.pending_computational_ih_call;
                                let disposition = self
                                    .consume_checked_ih_marker_at_static_worker_call(
                                        binder_index,
                                        0,
                                        static_origin,
                                    )?;
                                let materialized = match pending {
                                    Some(pending) => self
                                        .materialize_checked_ih_static_worker_application(
                                            builder,
                                            pending,
                                            disposition,
                                            &worker,
                                        )?,
                                    None => None,
                                };
                                if let Some(environment) = materialized {
                                    SourceMachineState::Value {
                                        value: RoutedAnswer::direct(environment),
                                        control,
                                    }
                                } else {
                                let before = self.live_source_continuations;
                                let (called, emission) = self.call_static_worker_with_inputs(
                                    builder,
                                    &worker,
                                    Vec::new(),
                                    static_origin,
                                    None,
                                )?
                                .into_emitted()?;
                                // `D8p` — the TARGET side, under the same key,
                                // written only now that the call instruction
                                // exists and carrying the run it actually took.
                                #[cfg(test)]
                                if disposition == CheckedApplicationDisposition::ConsumedHere {
                                    crate::cranelift_backend::lowering::record_d8p_emitted_target(
                                        crate::cranelift_backend::lowering::D8pEmittedTarget {
                                            function: self.defining_function_id,
                                            application_origin: static_origin,
                                            target_body_origin: worker.body_origin,
                                            declared_arity: worker.declared_arity,
                                            captures: worker.captures.len(),
                                            supplied_operands: emission.supplied_operands,
                                        },
                                    );
                                }
                                // `D8f` — the disposition, recorded AFTER the
                                // call instruction exists. A record here is
                                // therefore "this exact call was emitted, with
                                // this disposition", which is what an omission
                                // control needs: emitted, and not consumed.
                                #[cfg(test)]
                                crate::cranelift_backend::lowering::record_d8f_disposition(
                                    self.defining_function_id,
                                    static_origin,
                                    disposition,
                                );
                                // `D8j` — the call is emitted and its result is
                                // in hand under the SAME `control` this arm was
                                // entered with. Only now may a composed
                                // obligation be claimed.
                                // **`D8f` — THE CLAIM DISPOSITION, three cases,
                                // matched exhaustively.**
                                match disposition {
                                    // The `D8j` population: an ordinary composed
                                    // call, untouched by this seam, claims its
                                    // causal identity exactly as before.
                                    CheckedApplicationDisposition::NoPendingApplication
                                    // The checked application itself, claiming
                                    // the planner-issued identity once.
                                    | CheckedApplicationDisposition::ConsumedHere => {
                                        self.claim_composed_discharge(
                                            &worker, emission, &called, before,
                                        )?;
                                    }
                                    // ⛔ The declined call. It is emitted
                                    // unchanged and it claims NOTHING: the
                                    // identity belongs to the checked
                                    // application the planner issued it for, and
                                    // an ordinary selected-argument call
                                    // answering for it is a second discharge of
                                    // one obligation. The binding is not
                                    // reclassified and no identity is minted --
                                    // this call simply does not answer.
                                    CheckedApplicationDisposition::PendingAtAnotherOccurrence => {
                                        #[cfg(test)]
                                        if d8f_declined_call_claims() {
                                            self.claim_composed_discharge(
                                                &worker, emission, &called, before,
                                            )?;
                                        }
                                    }
                                }
                                SourceMachineState::Value {
                                    value: RoutedAnswer::direct(called),
                                    control,
                                }
                                }
                            } else {
                                let first = remaining.remove(0);
                                control.continuation = SourceContinuation::CallArgument {
                                    callee: SourceCallee::StaticWorker {
                                        worker,
                                        static_origin,
                                        binder_index,
                                    },
                                    remaining,
                                    lowered: Vec::new(),
                                    env: env.clone(),
                                    next: Box::new(control.continuation),
                                };
                                SourceMachineState::Eval {
                                    expr: first,
                                    env,
                                    control,
                                }
                            }
                        } else {
                            control.continuation = SourceContinuation::CallCallee {
                                args,
                                env: env.clone(),
                                next: Box::new(control.continuation),
                            };
                            SourceMachineState::Eval {
                                expr: self.owned_child_occurrence(static_origin, 0, *callee)?,
                                env,
                                control,
                            }
                        }
                    }
                    RuntimeExpr::ComputationalMatch {
                        scrutinee,
                        cases,
                        default,
                    } => {
                        let checked_frame_id =
                            self.consume_checked_subcontinuation_frame(&cases, &default)?;
                        control.continuation = SourceContinuation::ComputationalMatchScrutinee {
                            cases,
                            default,
                            env: env.clone(),
                            static_origin,
                            provenance: self.mint_recursor_frame_provenance(),
                            checked_frame_id,
                            answer_route: SourceComputationalAnswerRoute::DirectScrutinee,
                            next: Box::new(control.continuation),
                        };
                        SourceMachineState::Eval {
                            expr: self.owned_child_occurrence(static_origin, 0, *scrutinee)?,
                            env,
                            control,
                        }
                    }
                    // ⭐ The delegation point. Every form this dispatcher does not
                    // handle — closures included — goes to `lower_expr` here, and
                    // it now goes **as the same occurrence**: same term, same
                    // origin. This arm is why a "machine-only" subset could never
                    // have been threaded soundly.
                    other => SourceMachineState::Value {
                        value: RoutedAnswer::direct(self.lower_expr(
                            builder,
                            SourceOccurrence {
                                expr: &other,
                                static_origin,
                            },
                            &env,
                        )?),
                        control,
                    },
                },
                SourceMachineState::Value { value, mut control } => {
                    // ⭐⭐ `D6a` upstream half -- SPLIT THE PAIR ONCE, HERE.
                    //
                    // ⛔ `incoming_route` is the route THIS predecessor arrived
                    // by. Every transition below that forwards the same operand
                    // carries it forward; only a transition producing a NEW
                    // value starts at `DirectScrutinee`. Resetting on a forward
                    // is the erasure this checkpoint exists to prevent, and it
                    // is SILENT -- the compile stays green and the checked
                    // answer quietly takes the closed default.
                    let RoutedAnswer {
                        value,
                        route: incoming_route,
                        role: incoming_role,
                    } = value;
                    if matches!(value, LoweringOperand::Specialized(Lowered::Trap(_))) {
                        control.continuation = Self::discard_source_prefix(control.continuation);
                    }
                    match control.continuation {
                        SourceContinuation::Terminal(SourceContinuationTerminal::ReturnValue) => {
                            return Ok(value);
                        }
                        SourceContinuation::Terminal(
                            SourceContinuationTerminal::ReturnToProducerHole {
                                stack,
                                resume_cursor,
                                expected,
                                active,
                                root_authority,
                            },
                        ) => {
                            #[cfg(test)]
                            px8j_record_source_event(Px8jSourceTraceEvent::ReturnHole {
                                cursor: resume_cursor,
                            });
                            if active.cursor != expected {
                                return Err(unsupported(
                                    "ComputationalRecursor",
                                    "producer-hole terminal cursor mismatch",
                                ));
                            }
                            if matches!(value, LoweringOperand::Specialized(Lowered::Trap(_))) {
                                return Ok(value);
                            }
                            source_active_cursor(
                                &control.selected,
                                &control.selected_lineage,
                                resume_cursor,
                            )
                            .ok_or_else(|| {
                                unsupported(
                                    "ComputationalRecursor",
                                    "producer-hole resume cursor is no longer active",
                                )
                            })?;
                            control.continuation = SourceContinuation::UnwindRecursorSegment {
                                stack,
                                resume_cursor,
                                next: Box::new(SourceContinuation::Terminal(
                                    SourceContinuationTerminal::ResumeOuter {
                                        expected,
                                        active,
                                        root_authority,
                                    },
                                )),
                            };
                            SourceMachineState::Value { value: RoutedAnswer { value, route: incoming_route, role: incoming_role }, control }
                        }
                        SourceContinuation::Terminal(SourceContinuationTerminal::ResumeOuter {
                            expected,
                            active,
                            root_authority,
                        }) => {
                            #[cfg(test)]
                            px8j_record_source_event(Px8jSourceTraceEvent::ResumeOuter {
                                cursor: expected,
                            });
                            if active.cursor != expected {
                                return Err(unsupported(
                                    "ComputationalRecursor",
                                    "source continuation terminal cursor mismatch",
                                ));
                            }
                            self.restore_root_terminal_authority(root_authority, expected)?;
                            if matches!(value, LoweringOperand::Specialized(Lowered::Trap(_))) {
                                return Ok(value);
                            }
                            return self.resume_active_continuation(builder, value, *active);
                        }
                        SourceContinuation::Terminal(SourceContinuationTerminal::JumpToJoin(
                            edge,
                        )) => {
                            if matches!(value, LoweringOperand::Specialized(Lowered::Trap(_))) {
                                let failure = builder.ins().iconst(types::I64, -4);
                                builder.ins().return_(&[failure]);
                                return Ok(LoweringOperand::Specialized(Lowered::RecursiveBackedge));
                            }
                            let value = if edge.target.terminal_active_prefix.is_empty() {
                                value
                            } else {
                                let mut prefix = edge.target.terminal_active_prefix;
                                prefix.push(EliminatorFrame::InvocationReturn);
                                self.lower_computational_match_value_composed(
                                    builder, RoutedAnswer { value, route: incoming_route, role: incoming_role }, &prefix,
                                )?
                            };
                            match edge.target.join_plan.representation {
                                JoinResultRepresentation::NativeScalarPair => {
                                    let (value, actual_kind) =
                                        self.merge_planned_scalar_branch(
                                            builder,
                                            edge.target.join_plan.as_ref(),
                                            value,
                                            edge.target.required_kind,
                                            "NativeJoinPlanV1",
                                        )?;
                                    if actual_kind != ScalarMergeKind::RecursiveBackedge
                                        && actual_kind != edge.target.required_kind
                                    {
                                        return Err(unsupported(
                                            "NativeJoinPlanV1",
                                            format!(
                                                "predecessor {} for join {} produced \
                                                 {actual_kind:?}, planned {:?}",
                                                edge.predecessor_identity,
                                                edge.target.join_id,
                                                edge.target.required_kind
                                            ),
                                        ));
                                    }
                                    builder.ins().jump(
                                        edge.target.block,
                                        &[value.tag.into(), value.payload.into()],
                                    );
                                }
                                JoinResultRepresentation::CarrierWord => {
                                    let word = self.carried_join_arm(
                                        builder,
                                        edge.target.result_origin,
                                        value,
                                        Some(edge.target.required_kind),
                                        "NativeJoinPlanV1",
                                    )?;
                                    builder
                                        .ins()
                                        .jump(edge.target.block, &[word.word.into()]);
                                }
                            }
                            return Ok(LoweringOperand::Specialized(Lowered::RecursiveBackedge));
                        }
                        SourceContinuation::LetBody { body, env, next } => {
                            control.continuation = *next;
                            // `D2b` OBSERVATION ONLY, arm-local: what this arm
                            // saw. It decides nothing and changes no branch.
                            #[cfg(test)]
                            crate::cranelift_backend::lowering::lrc_d2b_record_let_arrival(
                                body.static_origin,
                                matches!(
                                    value,
                                    LoweringOperand::Specialized(Lowered::RecursiveBackedge)
                                ),
                            );
                            if matches!(value, LoweringOperand::Specialized(Lowered::RecursiveBackedge)) {
                                // ⭐⭐ `RT-LEXICAL-RECURSOR-CONSUMERS` `D2b` — THE
                                // ABANDONED LET BODY IS DISPOSITIONED, NOT
                                // CONSUMED.
                                //
                                // A backedge value means this `Let` never binds
                                // and its body never runs. The body is still a
                                // planned source subtree, so its joins are in
                                // the function's `required` set -- and with
                                // nothing executing them they are consumed by
                                // nobody, which `finalize_join_disposition`
                                // correctly refuses.
                                //
                                // ⛔ DISPOSITION, not consumption, and the
                                // distinction is the measured route rather than
                                // a preference: the body's `Call` is never
                                // entered, so there is no execution to consume
                                // it. An earlier plan tried to make it execute
                                // through a case-binder telescope; that was
                                // withdrawn once the route was measured, because
                                // the failing compile is this root machine's and
                                // never reaches a specialization definition.
                                //
                                // ⛔ The selector is the PLANNER'S OWN retained
                                // body root. Not a numeric origin, not a worker
                                // or closure root, and not the whole root
                                // function -- the last would swallow joins that
                                // legitimately executed and report a false
                                // accounting rather than a fix.
                                //
                                // ⛔ The body's source occurrence is deliberately
                                // NOT entered: it did not execute, and entering
                                // it would claim an execution that never
                                // happened.
                                #[cfg(test)]
                                {
                                    let root = match lrc_d2b_let_disposition() {
                                        LrcD2bLetDisposition::Exact => Some(body.static_origin),
                                        LrcD2bLetDisposition::Suppress => None,

                                    };
                                    if let Some(root) = root {
                                        self.disposition_statically_unselected_source_subtree(
                                            root,
                                        )?;
                                    }
                                }
                                #[cfg(not(test))]
                                self.disposition_statically_unselected_source_subtree(
                                    body.static_origin,
                                )?;
                                SourceMachineState::Value { value: RoutedAnswer { value, route: incoming_route, role: incoming_role }, control }
                            } else if matches!(value, LoweringOperand::Specialized(Lowered::Trap(_))) {
                                SourceMachineState::Value { value: RoutedAnswer { value, route: incoming_route, role: incoming_role }, control }
                            } else {
                                let body_env = env_with_operands([value], &env);
                                SourceMachineState::Eval {
                                    expr: body,
                                    env: body_env,
                                    control,
                                }
                            }
                        }
                        SourceContinuation::CheckedRecursiveInvocationReturn { instance, next } => {
                            self.leave_checked_recursive_invocation(instance)?;
                            control.continuation = *next;
                            SourceMachineState::Value { value: RoutedAnswer { value, route: incoming_route, role: incoming_role }, control }
                        }
                        SourceContinuation::CheckedComputationalIHInvocationReturn {
                            call_template_id,
                            next,
                        } => {
                            if self
                                .pending_computational_ih_call
                                .is_some_and(|pending| pending.call_template_id != call_template_id)
                            {
                                return Err(unsupported(
                                    "OrientedSubcontinuationPlanV1",
                                    "computational IH invocation return crossed another marker",
                                ));
                            }
                            let value = self.finish_checked_computational_ih_marker(value)?;
                            control.continuation = *next;
                            SourceMachineState::Value { value: RoutedAnswer { value, route: incoming_route, role: incoming_role }, control }
                        }
                        SourceContinuation::ReturnFromSelectedCase { delimiter, next } => {
                            let scope =
                                control.selected.selected_scope.as_ref().ok_or_else(|| {
                                    unsupported(
                                        "OrientedSubcontinuationPlanV1",
                                        "selected-case return has no open control obligation",
                                    )
                                })?;
                            if control.selected.activation != delimiter.activation
                                || control.selected.cursor != delimiter.cursor
                                || scope.scope_origin != delimiter.scope_origin
                                || scope.frame.checked_frame_id != delimiter.frame_id
                                || scope.frame.checked_invocation_id != delimiter.invocation_id
                            {
                                return Err(unsupported(
                                    "OrientedSubcontinuationPlanV1",
                                    "selected-case return delimiter does not match its open occurrence",
                                ));
                            }
                            let previous = control.selected_lineage.pop().ok_or_else(|| {
                                unsupported(
                                    "OrientedSubcontinuationPlanV1",
                                    "selected-case return has no exact parent control state",
                                )
                            })?;
                            control.selected = previous;
                            control.continuation = *next;
                            SourceMachineState::Value { value: RoutedAnswer { value, route: incoming_route, role: incoming_role }, control }
                        }
                        SourceContinuation::ApplyRecursorSelection { layer, next } => {
                            #[cfg(test)]
                            d5a_trace(format!(
                                "RT-D2 C APPLY-RECURSOR-SELECTION consumed \
layer_origin={:?} layer_role={:?} next_top={:?}",
                                layer.static_origin,
                                layer.role,
                                rt_continuation_kinds(next.as_ref()),
                            ));
                            #[cfg(test)]
                            match layer.role {
                                RecursorLayerRole::SelectsOccurrence { origin } => {
                                    px8j_record_source_event(Px8jSourceTraceEvent::Selection {
                                        origin,
                                    });
                                }
                                RecursorLayerRole::ExitsScope {
                                    origin,
                                    scope_origin,
                                    parent_scope,
                                } => px8j_record_source_event(Px8jSourceTraceEvent::Exit {
                                    origin,
                                    scope_origin,
                                    parent_scope,
                                }),
                            }
                            let answer_route =
                                SourceComputationalAnswerRoute::for_recursor_layer(&layer);
                            control.continuation =
                                SourceContinuation::ComputationalMatchScrutinee {
                                    cases: layer.cases,
                                    default: layer.default,
                                    env: layer.outer_env,
                                    static_origin: layer.static_origin,
                                    provenance: layer.provenance,
                                    checked_frame_id: layer.checked_frame_id,
                                    answer_route,
                                    next,
                                };
                            SourceMachineState::Value { value: RoutedAnswer { value, route: incoming_route, role: incoming_role }, control }
                        }
                        SourceContinuation::UnwindRecursorSegment {
                            mut stack,
                            resume_cursor,
                            next,
                        } => {
                            source_active_cursor(
                                &control.selected,
                                &control.selected_lineage,
                                resume_cursor,
                            )
                            .ok_or_else(|| {
                                unsupported(
                                    "ComputationalRecursor",
                                    "source recursor resume cursor is no longer active",
                                )
                            })?;
                            if let Some(layer) = stack.later_wrappers_in_construction_order.pop() {
                                #[cfg(test)]
                                if let RecursorLayerRole::ExitsScope {
                                    origin,
                                    scope_origin,
                                    parent_scope,
                                } = layer.role
                                {
                                    px8j_record_source_event(Px8jSourceTraceEvent::Exit {
                                        origin,
                                        scope_origin,
                                        parent_scope,
                                    });
                                }
                                let answer_route =
                                    SourceComputationalAnswerRoute::for_recursor_layer(&layer);
                                control.continuation =
                                    SourceContinuation::ComputationalMatchScrutinee {
                                        cases: layer.cases,
                                        default: layer.default,
                                        env: layer.outer_env,
                                        static_origin: layer.static_origin,
                                        provenance: layer.provenance,
                                        checked_frame_id: layer.checked_frame_id,
                                        answer_route,
                                        next: Box::new(SourceContinuation::UnwindRecursorSegment {
                                            stack,
                                            resume_cursor,
                                            next,
                                        }),
                                    };
                                SourceMachineState::Value { value: RoutedAnswer { value, route: incoming_route, role: incoming_role }, control }
                            } else {
                                control.continuation = *next;
                                SourceMachineState::Value { value: RoutedAnswer { value, route: incoming_route, role: incoming_role }, control }
                            }
                        }
                        SourceContinuation::ConstructArgument {
                            constructor,
                            static_origin,
                            mut remaining,
                            mut lowered,
                            env,
                            next,
                        } => {
                            if matches!(
                                &value,
                                LoweringOperand::Specialized(Lowered::RecursiveBackedge)
                            ) {
                                control.continuation = *next;
                                SourceMachineState::Value {
                                    value: RoutedAnswer {
                                        value,
                                        route: incoming_route,
                                        role: incoming_role,
                                    },
                                    control,
                                }
                            } else {
                            // A source constructor may receive the transported
                            // checked-IH environment as a carried word. Preserve
                            // phase in the field run; if any field is carried the
                            // constructor itself takes the ordinary governed
                            // carrier path instead of demanding a template.
                            lowered.push(value);
                            control.continuation = *next;
                            if remaining.is_empty() {
                                let destination_owner = self.defining_emission_owner.ok_or_else(|| {
                                    unsupported(
                                        "CheckedIhEnvironmentTransport",
                                        "a source-machine constructor crossing has no destination emission owner",
                                    )
                                })?;
                                if let Some(transport) = self
                                    .static_transition_plan
                                    .checked_ih_environment_transport_at(
                                        destination_owner,
                                        static_origin,
                                    )?
                                    .cloned()
                                {
                                    let position = transport.recursive_position() as usize;
                                    let child_origin = self
                                        .static_transition_plan
                                        .child_static_origin(static_origin, position)?;
                                    if child_origin != transport.seat() || position >= lowered.len() {
                                        return Err(unsupported(
                                            "CheckedIhEnvironmentTransport",
                                            "the source-machine crossing disagrees with the transport's stable closure field",
                                        ));
                                    }
                                    let environment = self
                                        .call_checked_ih_transport_from_case_environment(
                                            builder,
                                            &transport,
                                            &env,
                                        )?;
                                    if !matches!(&environment, LoweringOperand::Carried(_)) {
                                        return Err(unsupported(
                                            "CheckedIhEnvironmentTransport",
                                            "the source-machine transport did not yield an environment carrier word",
                                        ));
                                    }
                                    lowered[position] = environment;
                                }
                                let constructed = if lowered.iter().any(|field| {
                                    matches!(field, LoweringOperand::Carried(_))
                                }) {
                                    LoweringOperand::Carried(self.transfer_constructor_operands(
                                        builder,
                                        static_origin,
                                        &constructor,
                                        &lowered,
                                    )?)
                                } else {
                                    LoweringOperand::Specialized(self.finish_source_constructor(
                                        builder,
                                        constructor,
                                        static_origin,
                                        specialized_operands_at(
                                            &lowered,
                                            "a source constructor argument",
                                        )?,
                                    )?)
                                };
                                SourceMachineState::Value {
                                    value: RoutedAnswer::direct(constructed),
                                    control,
                                }
                            } else {
                                let first = remaining.remove(0);
                                control.continuation = SourceContinuation::ConstructArgument {
                                    constructor,
                                    static_origin,
                                    remaining,
                                    lowered,
                                    env: env.clone(),
                                    next: Box::new(control.continuation),
                                };
                                SourceMachineState::Eval {
                                    expr: first,
                                    env,
                                    control,
                                }
                            }
                            }
                        }
                        SourceContinuation::MatchScrutinee {
                            cases,
                            default,
                            env,
                            static_origin,
                            next,
                        } => {
                            self.enter_source_occurrence_plan(static_origin)?;
                            control.continuation = *next;
                            match value {
                                LoweringOperand::Specialized(Lowered::BoundedNat(nat)) => {
                                    return self.lower_source_bounded_nat_match(
                                        builder,
                                        nat,
                                        false,
                                        &cases,
                                        &default,
                                        static_origin,
                                        &env,
                                        control,
                                    );
                                }
                                LoweringOperand::Specialized(Lowered::StructuralNat(nat)) => {
                                    return self.lower_source_bounded_nat_match(
                                        builder,
                                        BoundedNatV1::derived_from_validated(nat.value),
                                        true,
                                        &cases,
                                        &default,
                                        static_origin,
                                        &env,
                                        control,
                                    );
                                }
                                LoweringOperand::Specialized(Lowered::Bool { value, known }) => {
                                    let true_case = cases.iter().enumerate().find(|(_, case)| {
                                        case.binders == 0
                                            && case.constructor.ends_with("::Bool::True")
                                    });
                                    let false_case = cases.iter().enumerate().find(|(_, case)| {
                                        case.binders == 0
                                            && case.constructor.ends_with("::Bool::False")
                                    });
                                    let (Some(true_case), Some(false_case)) =
                                        (true_case, false_case)
                                    else {
                                        return Err(unsupported(
                                            "Match",
                                            "Bool match requires zero-binder True and False cases",
                                        ));
                                    };
                                    if let Some(selected) = known {
                                        let (index, case) =
                                            if selected { true_case } else { false_case };
                                        self.disposition_statically_unselected_match_cases(
                                            static_origin,
                                            Some(index),
                                        )?;
                                        SourceMachineState::Eval {
                                            expr: self.owned_case_body_occurrence(
                                                static_origin,
                                                index,
                                                case.body.clone(),
                                            )?,
                                            env,
                                            control,
                                        }
                                    } else {
                                        let (true_index, true_case) = true_case;
                                        let (false_index, false_case) = false_case;
                                        let true_body = self.case_body_occurrence(
                                            static_origin,
                                            true_index,
                                            &true_case.body,
                                        )?;
                                        let false_body = self.case_body_occurrence(
                                            static_origin,
                                            false_index,
                                            &false_case.body,
                                        )?;
                                        return self.lower_source_dynamic_bool_match(
                                            builder,
                                            value,
                                            true_body,
                                            false_body,
                                            static_origin,
                                            &env,
                                            control,
                                        );
                                    }
                                }
                                LoweringOperand::Specialized(Lowered::HostResult {
                                    success,
                                    error,
                                    ok,
                                    err_constructor,
                                    ok_constructor,
                                }) => {
                                    return self.lower_source_dynamic_host_result_match(
                                        builder,
                                        success,
                                        *error,
                                        *ok,
                                        &err_constructor,
                                        &ok_constructor,
                                        &cases,
                                        default,
                                        static_origin,
                                        &env,
                                        control,
                                    );
                                }
                                LoweringOperand::Specialized(Lowered::DynamicConstructor(dynamic)) => {
                                    return self.lower_source_dynamic_constructor_match(
                                        builder,
                                        dynamic,
                                        &cases,
                                        &default,
                                        static_origin,
                                        &env,
                                        control,
                                    );
                                }
                                LoweringOperand::Specialized(Lowered::Constructor {
                                    constructor,
                                    args,
                                    ..
                                }) => {
                                    let Some((case_index, case)) = cases
                                        .iter()
                                        .enumerate()
                                        .find(|(_, case)| case.constructor == constructor)
                                    else {
                                        self.disposition_statically_unselected_match_cases(
                                            static_origin,
                                            None,
                                        )?;
                                        return Ok(LoweringOperand::Specialized(Lowered::Trap(default)));
                                    };
                                    self.disposition_statically_unselected_match_cases(
                                        static_origin,
                                        Some(case_index),
                                    )?;
                                    if case.binders != args.len() {
                                        return Err(unsupported(
                                            "Match",
                                            format!(
                                    "case {} expects {} binders but constructor has {} args",
                                    case.constructor,
                                    case.binders,
                                    args.len()
                                ),
                                        ));
                                    }
                                    #[cfg(test)]
                                    record_d2k_owner_event(D2kOwnerEvent::StaticMatchBinderDescent {
                    site: "bound_constructor_fields@source-machine",
                    eliminated_origin: static_origin,
                });
                                    let mut case_env = self.bound_constructor_fields(&args, &[])?;
                                    case_env.extend(env);
                                    SourceMachineState::Eval {
                                        expr: self.owned_case_body_occurrence(
                                            static_origin,
                                            case_index,
                                            case.body.clone(),
                                        )?,
                                        env: case_env,
                                        control,
                                    }
                                }
                                // A runtime boundary word has no compile-time
                                // template, so it is classified by PHASE before
                                // any arm asks for a `Lowered` shape. Without
                                // this arm it fell past every shape test onto
                                // the refusal below -- a true sentence about the
                                // wrong thing, naming a cause that is not the
                                // cause: the value is fine, the question is.
                                //
                                // The generic `lower_expr` `Match` emitter and
                                // the source machine's
                                // `ComputationalMatchScrutinee` both already
                                // carry this arm. This seat was the only one of
                                // the three missing it.
                                //
                                // It does NOT delegate to the generic seat's
                                // `lower_carried_match`: that helper returns a
                                // value to its caller, while this seat owns a
                                // continuation, so each case body is lowered
                                // under the match's original `next` through its
                                // own source predecessor edge.
                                //
                                // Taken as an explicit arm rather than a
                                // fallback, so this match is exhaustive over
                                // `LoweringOperand` and a third variant is a
                                // compile error here rather than a silent
                                // refusal.
                                LoweringOperand::Carried(word) => {
                                    // Family 5 control seam. The operand is
                                    // already classified `Carried` here, so a
                                    // refusal below is evidence about the real
                                    // dispatch decision and nothing else.
                                    #[cfg(test)]
                                    if let Some(refusal) = source_carried_control_refusal(
                                        SourceCarriedControlMutation::RefuseClassifiedCarried,
                                        "Match",
                                        "scrutinee is not a constructor value",
                                    ) {
                                        return Err(refusal);
                                    }
                                    return self.lower_source_carried_match(
                                        builder,
                                        word,
                                        &cases,
                                        &default,
                                        static_origin,
                                        &env,
                                        control,
                                    );
                                }
                                LoweringOperand::Specialized(_) => {
                                    return Err(unsupported(
                                        "Match",
                                        "scrutinee is not a constructor value",
                                    ));
                                }
                            }
                        }
                        SourceContinuation::ComputationalMatchScrutinee {
                            cases,
                            default,
                            env,
                            static_origin,
                            provenance,
                            checked_frame_id,
                            answer_route,
                            next,
                        } => 'computational_scrutinee: {
                            // ⭐⭐ `RT-LEXICAL-RECURSOR-CONSUMERS` `D2a` — THE
                            // BACKEDGE MARKER IS FORWARDED, NOT ELIMINATED.
                            //
                            // `RecursiveBackedge` is a **protocol marker**, not a
                            // value: there is nothing to eliminate and no case to
                            // select. Reaching the specialized selection below it
                            // met `"source scrutinee is not a constructor value"`
                            // — the same shape the `Carried` arm beneath already
                            // names as *a true sentence about the wrong thing*.
                            // The guard is right; the question was.
                            //
                            // ⛔ Taken BEFORE `enter_source_occurrence_plan`, so
                            // this seat consumes no occurrence plan, mints no
                            // authority, selects and dispositions no case, and
                            // constructs no value. It sets the continuation to
                            // `next` and hands the SAME marker onward.
                            //
                            // ⛔ `incoming_route` is carried, not reset. This
                            // FORWARDS an operand rather than producing a new
                            // one, and the `D6a` contract above is explicit that
                            // resetting a forward to `DirectScrutinee` is a
                            // silent erasure — the compile stays green while the
                            // checked answer quietly takes the closed default.
                            //
                            // ⛔ Neither guard is weakened. A genuine
                            // non-constructor scrutinee still reaches the
                            // selection below and is still refused; only the
                            // marker, which was never a scrutinee value, is
                            // routed past it.
                            if matches!(
                                &value,
                                LoweringOperand::Specialized(Lowered::RecursiveBackedge)
                            ) {
                                #[cfg(test)]
                                LRC_D2A_BACKEDGE_ARRIVALS.with(|count| {
                                    count.set(count.get().saturating_add(1))
                                });
                                #[cfg(test)]
                                let suppressed =
                                    LRC_D2A_SUPPRESS_FORWARD.with(std::cell::Cell::get);
                                #[cfg(not(test))]
                                let suppressed = false;
                                if !suppressed {
                                    #[cfg(test)]
                                    LRC_D2A_BACKEDGE_FORWARDS.with(|count| {
                                        count.set(count.get().saturating_add(1))
                                    });
                                    control.continuation = *next;
                                    break 'computational_scrutinee SourceMachineState::Value {
                                        value: RoutedAnswer {
                                            value,
                                            route: incoming_route,
                                            role: EliminatorRole::Scrutinee,
                                        },
                                        control,
                                    };
                                }
                            }
                            // `RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` — THE
                            // CONSUMER TAKEOVER, and the third member of the
                            // irreducible core.
                            //
                            // The redirected invocation already returned the
                            // FUSED result: the producer's body and this very
                            // suffix, run once inside the fused `Function`. So
                            // this seat has nothing left to eliminate, and
                            // running it would execute the suffix a second time
                            // — the defect node `:650` measured, which is why
                            // the redirect cannot ship without this.
                            //
                            // Taken BEFORE `enter_source_occurrence_plan`,
                            // like the backedge forward above and for a sharper
                            // reason: this occurrence's plan is consumed inside
                            // the fused body, by the eliminator frame built
                            // there. Entering it here as well would consume one
                            // occurrence plan twice.
                            //
                            // The claim is consumed at **its own seat**, not
                            // at this origin. `consume` checks the seat against
                            // the claim's redirected invocation, so a takeover
                            // offered at the wrong occurrence leaves the claim
                            // outstanding rather than spending it on a suffix it
                            // does not own.
                            //
                            // `incoming_route` is carried, not reset, for the
                            // same reason the backedge arm carries it: this
                            // FORWARDS an operand that a call already produced.
                            if let Some(taken) = self.take_fused_region_at(static_origin)? {
                                let _ = taken;
                                control.continuation = *next;
                                break 'computational_scrutinee SourceMachineState::Value {
                                    value: RoutedAnswer {
                                        value,
                                        route: incoming_route,
                                        role: EliminatorRole::Scrutinee,
                                    },
                                    control,
                                };
                            }
                            self.enter_source_occurrence_plan(static_origin)?;
                            #[cfg(test)]
                            d5a_trace(format!(
                                "RT-D2 D COMPUTATIONAL-MATCH-SCRUTINEE consumed \
match_origin={static_origin:?} input[{}] frame_route={answer_route:?} next_top={:?}",
                                rt_operand_desc(&value),
                                rt_continuation_kinds(next.as_ref()),
                            ));
                            // ⭐⭐ `AC-C4` — THE RESUMPTION POINT. An induction
                            // hypothesis over a carried child hands its word back
                            // as the machine's value, and it lands **here**: this
                            // continuation is what "resumes the same computational
                            // eliminator over that carried word" means on the
                            // source-machine route.
                            //
                            // ⛔ Taken before the specialized selection below,
                            // which reads `Lowered::Constructor` — a compile-time
                            // template the carried value does not have and must
                            // not be asked for. Without this arm the resumed word
                            // reaches `"source scrutinee is not a constructor
                            // value"`, which is a **true sentence about the wrong
                            // thing**: the value is fine, the question is.
                            if let LoweringOperand::Carried(word) = &value {
                                let word = *word;
                                let frame = ComputationalEliminatorFrame {
                                    cases: &cases,
                                    default: &default,
                                    env: &env,
                                    static_origin,
                                    retained_scrutinee_index: None,
                                    deferred_constructor_case: None,
                                    provenance,
                                    checked_frame_id,
                                    checked_invocation_id: checked_frame_id.map(|_| {
                                        self.active_recursive_invocations
                                            .last()
                                            .map_or(0, |instance| instance.invocation_instance_id)
                                    }),
                                    checked_invocation_source: self
                                        .active_recursive_invocations
                                        .last()
                                        .map(|instance| instance.source),
                                    checked_invocation_depth: self
                                        .active_recursive_invocations
                                        .last()
                                        .map_or(0, |instance| instance.semantic_depth),
                                    // ⭐⭐ `D6a` -- THE PREDECESSOR'S ROUTE RAISES THE
                                    // CONTINUATION'S, and never lowers it.
                                    //
                                    // The continuation's own field stays the
                                    // recursor-layer producer's authority. It is not
                                    // the sole authority: an exact claimed and emitted
                                    // continuation-specialization call result arrives
                                    // here already checked, and letting the
                                    // continuation's `DirectScrutinee` overwrite that
                                    // is precisely the drop measured at `ae45e804`.
                                    answer_route: RoutedAnswer {
                                        value: LoweringOperand::Carried(word),
                                        route: incoming_route,
                                        role: EliminatorRole::Scrutinee,
                                    }
                                    .raise(answer_route),
                                };
                                #[cfg(test)]
                                let mut frame = frame;
                                #[cfg(test)]
                                if d6a_route_mutation()
                                    == D6aRouteMutation::OverwriteIncomingWithFrameField
                                {
                                    record_d6a_route_application();
                                    frame.answer_route = answer_route;
                                }
                                #[cfg(test)]
                                record_d6a_route_event(D6aRouteEvent::ConsumerRoute {
                                    seat: D6aConsumerSeat::SourceMachine,
                                    static_origin,
                                    incoming: incoming_route,
                                    frame_field: answer_route,
                                    joined: frame.answer_route,
                                });
                                let eliminated = self
                                    .lower_carried_computational_match(builder, word, frame, &[])?;
                                control.continuation = *next;
                                break 'computational_scrutinee SourceMachineState::Value {
                                    value: RoutedAnswer::direct(eliminated),
                                    control,
                                };
                            }
                            let retained = value.clone();
                            #[cfg(test)]
                            let actual_constructor = match &value {
                                LoweringOperand::Specialized(Lowered::Constructor {
                                    constructor,
                                    ..
                                }) => Some(constructor.clone()),
                                LoweringOperand::Specialized(_) | LoweringOperand::Carried(_) => {
                                    None
                                }
                            };
                            let selected = match &value {
                                LoweringOperand::Specialized(Lowered::Constructor { constructor, .. }) => cases
                                    .iter()
                                    .enumerate()
                                    .find(|(_, case)| case.constructor == *constructor),
                                _ => None,
                            };
                            let (case_index, case) = if let Some(selected) = selected {
                                self.record_source_machine_computational_match_selection(
                                    static_origin,
                                    Some(selected.0),
                                )?;
                                selected
                            } else if answer_route
                                == SourceComputationalAnswerRoute::CheckedSelectedRecursor
                                && matches!(&value, LoweringOperand::Specialized(Lowered::Constructor { .. }))
                                && px8tr_deforested_answer_route_enabled()
                            {
                                let mut returns = cases.iter().enumerate().filter(|(_, case)| {
                                    case.argument_binders == 1
                                        && case.constructor.ends_with("::ITree::Ret")
                                });
                                let return_case = returns.next();
                                let exact_return = returns.next().is_none();
                                let mut visible = cases
                                    .iter()
                                    .filter(|case| case.constructor.ends_with("::ITree::Vis"));
                                let exact_visible = visible.next().is_some()
                                    && visible.next().is_none()
                                    && cases.len() == 2;
                                let Some((return_index, return_case)) =
                                    return_case.filter(|(_, return_case)| {
                                        exact_return
                                            && exact_visible
                                            && source_case_has_no_checked_control_markers(
                                                &return_case.body,
                                            )
                                    })
                                else {
                                    self.record_source_machine_computational_match_selection(
                                        static_origin,
                                        None,
                                    )?;
                                    #[cfg(test)]
                                    px8tr_record_trap_provenance(
                                        Px8trTrapProvenanceEvent::CheckedRecursorDefault {
                                            checked_frame_id: checked_frame_id.expect(
                                                "checked answer routes carry exact frame ids",
                                            ),
                                            actual_constructor,
                                            trap: default.clone(),
                                        },
                                    );
                                    return Ok(LoweringOperand::Specialized(Lowered::Trap(default)));
                                };
                                #[cfg(test)]
                                px8tr_record_trap_provenance(
                                    Px8trTrapProvenanceEvent::DeforestedAnswerResumed {
                                        checked_frame_id: checked_frame_id
                                            .expect("checked answer routes carry exact frame ids"),
                                        actual_constructor,
                                        return_constructor: return_case.constructor.clone(),
                                    },
                                );
                                self.record_source_machine_computational_match_selection(
                                    static_origin,
                                    Some(return_index),
                                )?;
                                let case_env = env_with_operands([retained], &env);
                                control.continuation = *next;
                                let body = self.owned_case_body_occurrence(
                                    static_origin,
                                    return_index,
                                    return_case.body.clone(),
                                )?;
                                return self.lower_source_machine_with_continuation(
                                    builder,
                                    body,
                                    case_env,
                                    control,
                                );
                            } else {
                                if !matches!(&value, LoweringOperand::Specialized(Lowered::Constructor { .. })) {
                                    return Err(unsupported(
                                        "ComputationalMatch",
                                        "source scrutinee is not a constructor value",
                                    ));
                                }
                                self.record_source_machine_computational_match_selection(
                                    static_origin,
                                    None,
                                )?;
                                #[cfg(test)]
                                if answer_route
                                    == SourceComputationalAnswerRoute::CheckedSelectedRecursor
                                {
                                    px8tr_record_trap_provenance(
                                        Px8trTrapProvenanceEvent::CheckedRecursorDefault {
                                            checked_frame_id: checked_frame_id.expect(
                                                "checked answer routes carry exact frame ids",
                                            ),
                                            actual_constructor,
                                            trap: default.clone(),
                                        },
                                    );
                                }
                                return Ok(LoweringOperand::Specialized(Lowered::Trap(default)));
                            };
                            let LoweringOperand::Specialized(Lowered::Constructor { args, .. }) = value else {
                                unreachable!("a selected source case has a constructor value")
                            };
                            if case.argument_binders != args.len() {
                                return Err(unsupported(
                                    "ComputationalMatch",
                                    format!(
                                        "case {} expects {} constructor arguments but value has {}",
                                        case.constructor,
                                        case.argument_binders,
                                        args.len()
                                    ),
                                ));
                            }
                            let mut seen = BTreeSet::new();
                            for position in case.recursive_positions.iter().copied() {
                                if !seen.insert(position) || position >= args.len() {
                                    return Err(unsupported(
                                        "ComputationalMatch",
                                        format!(
                                            "case {} has malformed recursive position {position}",
                                            case.constructor
                                        ),
                                    ));
                                }
                            }
                            let frame = ComputationalEliminatorFrame {
                                cases: &cases,
                                default: &default,
                                env: &env,
                                static_origin,
                                retained_scrutinee_index: None,
                                deferred_constructor_case: None,
                                provenance,
                                checked_frame_id,
                                checked_invocation_id: checked_frame_id.map(|_| {
                                    self.active_recursive_invocations
                                        .last()
                                        .map_or(0, |instance| instance.invocation_instance_id)
                                }),
                                checked_invocation_source: self
                                    .active_recursive_invocations
                                    .last()
                                    .map(|instance| instance.source),
                                checked_invocation_depth: self
                                    .active_recursive_invocations
                                    .last()
                                    .map_or(0, |instance| instance.semantic_depth),
                                // `D6a` -- the same source continuation's fact. The
                                // specialized arm above already consumed it for its own
                                // selection; a nested CARRIED resumption of this frame must
                                // see the same route rather than a weaker one.
                                answer_route,
                            };
                            let activation = self.mint_continuation_activation();
                            let cursor = self.mint_continuation_cursor();
                            let mut ancestry = control.selected.selected_ancestry.clone();
                            ancestry.push(provenance);
                            let mut induction_hypotheses =
                                Vec::with_capacity(case.recursive_positions.len());
                            let ih_slots =
                                self.computational_ih_slots_for_case(case, frame.checked_frame_id)?;
                            let producer_origin = self.mint_recursor_producer_origin();
                            #[cfg(test)]
                            px8j_record_source_event(Px8jSourceTraceEvent::Mint {
                                path: Px8jProducerPath::SourceMachine,
                                origin: producer_origin,
                                cursor,
                                siblings: case.recursive_positions.len(),
                                parent_scope: control
                                    .selected
                                    .selected_scope
                                    .as_ref()
                                    .map(|scope| scope.scope_origin),
                            });
                            let parent = control.selected.parent;
                            {
                                let qold = control.selected.as_active(&control.selected_lineage);
                                for position in case.recursive_positions.iter().rev().copied() {
                                    let slot_template_id = case
                                        .recursive_positions
                                        .iter()
                                        .position(|candidate| *candidate == position)
                                        .and_then(|index| ih_slots[index]);
                                    let induction_hypothesis = self.make_computational_recursor(
                                        LoweringOperand::Specialized(
                                            args[position]
                                                .specialized_at(
                                                    "a source-machine computational recursor's \
                                                     selected recursive argument",
                                                )?
                                                .clone(),
                                        ),
                                        cases.clone(),
                                        default.clone(),
                                        env.clone(),
                                        static_origin,
                                        provenance,
                                        frame.checked_frame_id,
                                        slot_template_id,
                                        producer_origin,
                                        position,
                                        RecursorLayerRole::SelectsOccurrence {
                                            origin: producer_origin,
                                        },
                                        activation,
                                        cursor,
                                        Some(&qold),
                                        Some((
                                            &control.selected,
                                            control.selected_lineage.as_slice(),
                                        )),
                                        None,
                                    )?;
                                    #[cfg(test)]
                                    px8j_record_recursor_carrier(
                                        Px8jProducerPath::SourceMachine,
                                        &induction_hypothesis,
                                    );
                                    induction_hypotheses.push(LoweringEnvironmentBinding::Value(induction_hypothesis));
                                }
                            }
                            let frame_env = match self.materialize_eliminator_frame_env(
                                builder,
                                EliminatorFrame::Computational(frame),
                                retained.specialized_ref_at("an eliminator frame's scrutinee")?,
                            )? {
                                Ok(frame_env) => frame_env,
                                Err(trap) => return Ok(LoweringOperand::Specialized(Lowered::Trap(trap))),
                            };
                            let mut case_env = induction_hypotheses;
                            #[cfg(test)]
                            record_d2k_owner_event(D2kOwnerEvent::StaticMatchBinderDescent {
                    site: "extend_constructor_fields@source-machine-composed",
                    eliminated_origin: frame.static_origin,
                });
                            self.extend_constructor_fields(&mut case_env, &args)?;
                            case_env.extend(frame_env);
                            let previous_selected = control.selected.clone();
                            let pending = std::mem::take(&mut control.selected.pending);
                            let selected_scope = OwnedSelectedScope {
                                scope_origin: producer_origin,
                                parent_scope: control
                                    .selected
                                    .selected_scope
                                    .as_ref()
                                    .map(|scope| scope.scope_origin),
                                frame: ComputationalRecursorFramePayload {
                                    cases: cases.clone(),
                                    default: default.clone(),
                                    outer_env: env.clone(),
                                    static_origin,
                                    provenance,
                                    checked_frame_id: frame.checked_frame_id,
                                    checked_invocation_id: frame.checked_invocation_id,
                                    checked_invocation_source: frame.checked_invocation_source,
                                    checked_invocation_depth: frame.checked_invocation_depth,
                                },
                            };
                            #[cfg(test)]
                            let selected_scope =
                                (!PX8J_DELETE_OWNED_SELECTED_SCOPE.get()).then_some(selected_scope);
                            #[cfg(not(test))]
                            let selected_scope = Some(selected_scope);
                            control.continuation = if frame.checked_frame_id.is_some() {
                                let selected_scope_ref =
                                    selected_scope.as_ref().ok_or_else(|| {
                                        unsupported(
                                            "OrientedSubcontinuationPlanV1",
                                            "checked selection has no owned open-control obligation",
                                        )
                                    })?;
                                SourceContinuation::ReturnFromSelectedCase {
                                    delimiter: SelectedCaseReturnDelimiter {
                                        activation,
                                        cursor,
                                        scope_origin: selected_scope_ref.scope_origin,
                                        frame_id: selected_scope_ref.frame.checked_frame_id,
                                        invocation_id: selected_scope_ref
                                            .frame
                                            .checked_invocation_id,
                                    },
                                    next,
                                }
                            } else {
                                *next
                            };
                            control.selected = SourceSelectedContinuation {
                                activation,
                                cursor,
                                parent,
                                pending,
                                selected_ancestry: ancestry,
                                selected_scope,
                            };
                            control.selected_lineage.push(previous_selected);
                            let body = self.owned_case_body_occurrence(
                                static_origin,
                                case_index,
                                case.body.clone(),
                            )?;
                            SourceMachineState::Eval {
                                expr: body,
                                env: case_env,
                                control,
                            }
                        }
                        SourceContinuation::CallCallee {
                            mut args,
                            env,
                            next,
                        } => {
                            control.continuation = *next;
                            if args.is_empty() {
                                match self.source_call_state(
                                    builder,
                                    value,
                                    Vec::new(),
                                    env,
                                    control,
                                )? {
                                    SourceCallOutcome::Continue(state) => state,
                                    SourceCallOutcome::Complete(value) => return Ok(value),
                                }
                            } else {
                                let first = args.remove(0);
                                control.continuation = SourceContinuation::CallArgument {
                                    callee: SourceCallee::Value(value),
                                    remaining: args,
                                    lowered: Vec::new(),
                                    env: env.clone(),
                                    next: Box::new(control.continuation),
                                };
                                SourceMachineState::Eval {
                                    expr: first,
                                    env,
                                    control,
                                }
                            }
                        }
                        SourceContinuation::CallArgument {
                            callee,
                            mut remaining,
                            mut lowered,
                            env,
                            next,
                        } => {
                            lowered.push(value);
                            control.continuation = *next;
                            if remaining.is_empty() {
                                match callee {
                                    // **`D8e` — the sole consumer's completion.**
                                    //
                                    // ⛔ Straight into the SHARED route-selected
                                    // emitter with the arguments the machine
                                    // just evaluated. Nothing is re-evaluated,
                                    // no target is looked up again, and no
                                    // operand run is assembled here: captures,
                                    // the route's suffix and the route's table
                                    // all live in the one emitter the direct
                                    // descent uses.
                                    SourceCallee::StaticWorker {
                                        worker,
                                        static_origin,
                                        binder_index,
                                    } => {
                                        // `D8p` — the same seam, at the
                                        // non-empty argument run's emission
                                        // seat. The arguments are already
                                        // lowered, so any ordinary call among
                                        // them has already reached this seat and
                                        // already declined the marker.
                                        let disposition = self
                                            .consume_checked_ih_marker_at_static_worker_call(
                                                binder_index,
                                                lowered.len(),
                                                static_origin,
                                            )?;
                                        let before = self.live_source_continuations;
                                        let (called, emission) = self
                                            .call_static_worker_with_inputs(
                                                builder,
                                                &worker,
                                                lowered,
                                                static_origin,
                                                None,
                                            )?
                                            .into_emitted()?;
                                        // `D8p` — the target side, post-instruction.
                                        #[cfg(test)]
                                        if disposition
                                            == CheckedApplicationDisposition::ConsumedHere
                                        {
                                            crate::cranelift_backend::lowering::record_d8p_emitted_target(
                                                crate::cranelift_backend::lowering::D8pEmittedTarget {
                                                    function: self.defining_function_id,
                                                    application_origin: static_origin,
                                                    target_body_origin: worker.body_origin,
                                                    declared_arity: worker.declared_arity,
                                                    captures: worker.captures.len(),
                                                    supplied_operands: emission.supplied_operands,
                                                },
                                            );
                                        }
                                        // `D8f` — post-instruction, as above.
                                        #[cfg(test)]
                                        crate::cranelift_backend::lowering::record_d8f_disposition(
                                            self.defining_function_id,
                                            static_origin,
                                            disposition,
                                        );
                                        // `D8j` — the non-empty argument run
                                        // reached here through `CallArgument`
                                        // under the machine's own control, and
                                        // the raw call is now written. This is
                                        // the seat, and it is after all three.
                                        //
                                        // `D8f` — and the same three dispositions
                                        // decide whether it answers.
                                        match disposition {
                                            CheckedApplicationDisposition::NoPendingApplication
                                            | CheckedApplicationDisposition::ConsumedHere => {
                                                self.claim_composed_discharge(
                                                    &worker, emission, &called, before,
                                                )?;
                                            }
                                            CheckedApplicationDisposition::PendingAtAnotherOccurrence => {
                                                #[cfg(test)]
                                                if d8f_declined_call_claims() {
                                                    self.claim_composed_discharge(
                                                        &worker, emission, &called, before,
                                                    )?;
                                                }
                                            }
                                        }
                                        SourceMachineState::Value {
                                            value: RoutedAnswer::direct(called),
                                            control,
                                        }
                                    }
                                    SourceCallee::Value(callee) => match self
                                        .source_call_state(builder, callee, lowered, env, control)?
                                    {
                                        SourceCallOutcome::Continue(state) => state,
                                        SourceCallOutcome::Complete(value) => return Ok(value),
                                    },
                                }
                            } else {
                                let first = remaining.remove(0);
                                control.continuation = SourceContinuation::CallArgument {
                                    callee,
                                    remaining,
                                    lowered,
                                    env: env.clone(),
                                    next: Box::new(control.continuation),
                                };
                                SourceMachineState::Eval {
                                    expr: first,
                                    env,
                                    control,
                                }
                            }
                        }
                        SourceContinuation::IfScrutinee { .. }
                        | SourceContinuation::ProjectRecord { .. } => {
                            return Err(unsupported(
                                "ComputationalRecursor",
                                "source continuation frame is not implemented",
                            ));
                        }
                    }
                }
            };
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn lower_source_bounded_nat_match<'b>(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        nat: BoundedNatV1,
        structural: bool,
        cases: &[crate::RuntimeMatchCase],
        _default: &RuntimeTrap,
        static_origin: StaticOriginId,
        env: &[LoweringEnvironmentBinding],
        suffix_control: SourceControl<'b>,
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        let zero = cases.iter().enumerate().find(|(_, case)| {
            case.constructor == self.process_symbols.nat_zero && case.binders == 0
        });
        let suc = cases.iter().enumerate().find(|(_, case)| {
            case.constructor == self.process_symbols.nat_suc && case.binders == 1
        });
        let (Some((zero_index, zero)), Some((suc_index, suc))) = (zero, suc) else {
            return Err(unsupported(
                "BoundedNat",
                "structural Nat source match requires exact Zero and Suc predecessor arms",
            ));
        };
        let zero_body = self.case_body_occurrence(static_origin, zero_index, &zero.body)?;
        let suc_body = self.case_body_occurrence(static_origin, suc_index, &suc.body)?;

        let (source_prefix_template, terminal) =
            Self::split_source_prefix(suffix_control.continuation)?;
        let mut local_completion = None;
        let (source_prefix_template, target) = match terminal {
            SourcePrefixTerminal::Join(inherited_edge) => {
                let fanout = SourceBranchFanout {
                    source_prefix_template,
                    inherited_edge,
                };
                (fanout.source_prefix_template, fanout.inherited_edge.target)
            }
            SourcePrefixTerminal::ResumeOuter { root_authority } => {
                let active = suffix_control
                    .selected
                    .as_active(&suffix_control.selected_lineage);
                let (prefix, suffix_pending, required_kind, site_id) =
                    self.planned_active_scalar_cut(active)?;
                let join_id = self.next_source_join;
                self.next_source_join = self
                    .next_source_join
                    .checked_add(1)
                    .expect("compiler-private source join identity exhausted");
                let join_plan = std::rc::Rc::new(self.consumed_join_plan_token(static_origin)?);
                let merge = builder.create_block();
                self.append_planned_join_params(builder, merge, join_plan.as_ref());
                local_completion = Some((
                    merge,
                    suffix_pending.to_vec(),
                    required_kind,
                    site_id,
                    root_authority,
                ));
                (
                    source_prefix_template,
                    SourceJoinTarget {
                        join_id,
                        block: merge,
                        expected_outer: suffix_control.terminal_outer,
                        required_kind,
                        join_plan,
                        result_origin: static_origin,
                        terminal_active_prefix: prefix,
                    },
                )
            }
        };

        let zero_block = builder.create_block();
        let suc_block = builder.create_block();
        let predecessor = nat.predecessor(builder);
        let is_zero =
            builder
                .ins()
                .icmp_imm(cranelift_codegen::ir::condcodes::IntCC::Equal, nat.value, 0);
        builder.ins().brif(is_zero, zero_block, &[], suc_block, &[]);

        let mut frame_scope =
            CheckedFrameBranchScope::capture(&self.consumed_subcontinuation_frames);
        for (arm_name, block, case_body, predecessor) in [
            ("Zero", zero_block, zero_body, None),
            ("Suc", suc_block, suc_body, Some(predecessor)),
        ] {
            builder.switch_to_block(block);
            let arm_env = predecessor
                .map(|predecessor| {
                    vec![if structural {
                        Lowered::StructuralNat(StructuralNatV1 {
                            value: predecessor.value,
                        })
                    } else {
                        Lowered::BoundedNat(predecessor)
                    }]
                })
                .unwrap_or_default();
            let mut arm_env = env_with(arm_env, &[]);
            arm_env.extend_from_slice(env);
            let edge = self.mint_source_predecessor(target.clone());
            let continuation =
                Self::instantiate_source_prefix_template(&source_prefix_template, edge)?;
            let branch_control = SourceControl {
                continuation,
                selected: suffix_control.selected.clone(),
                selected_lineage: suffix_control.selected_lineage.clone(),
                terminal_outer: suffix_control.terminal_outer,
            };
            let lowered = self.lower_forked_branch(
                builder,
                &mut frame_scope,
                OwnedSourceOccurrence::cloned(case_body),
                arm_env,
                branch_control,
            )?;
            if self.seal_source_trap_branch(builder, &lowered)? {
                // A trap terminates this mutually exclusive predecessor.
            } else if !matches!(
                lowered,
                LoweringOperand::Specialized(Lowered::RecursiveBackedge)
            ) {
                let detail = match &lowered {
                    LoweringOperand::Specialized(Lowered::Trap(trap)) => {
                        format!("Trap({}: {:?})", trap.message, trap.code)
                    }
                    LoweringOperand::Specialized(other) => lowered_value_kind(other).to_string(),
                    // ⛔ No wildcard: a carried operand reaching a join
                    // diagnostic must name itself, not fall into `other`.
                    LoweringOperand::Carried(_) => "BoundaryCarrier".to_string(),
                };
                return Err(unsupported(
                    "NativeJoinPlanV1",
                    format!(
                        "bounded-Nat {arm_name} arm produced {detail} instead of sealing its distinct affine predecessor edge"
                    ),
                ));
            }
        }
        self.consumed_subcontinuation_frames = frame_scope.finish();

        let Some((merge, suffix_pending, required_kind, _site_id, root_authority)) =
            local_completion
        else {
            return Ok(LoweringOperand::Specialized(Lowered::RecursiveBackedge));
        };
        let merged = self.finish_planned_join(
            builder,
            merge,
            target.join_plan.as_ref(),
            Some(required_kind),
            "NativeJoinPlanV1",
        );
        let suffix_active = ActiveContinuationFrame {
            activation: suffix_control.selected.activation,
            cursor: suffix_control.selected.cursor,
            parent: suffix_control.selected.parent,
            pending: &suffix_pending,
            selected_ancestry: &suffix_control.selected.selected_ancestry,
            source_lineage: &suffix_control.selected_lineage,
            source_selected_cursor: Some(suffix_control.selected.cursor),
            selected_scope: suffix_control.selected.selected_scope.as_ref(),
        };
        self.restore_root_terminal_authority(root_authority, suffix_control.terminal_outer)?;
        self.resume_active_continuation(builder, merged?, suffix_active)
    }

    /// Lower one mutually-exclusive match arm with the checked-subcontinuation-
    /// frame consumption set rewound by `frame_scope`, then fold the arm's
    /// resulting consumptions into that scope's union.
    ///
    /// A dynamic match lowers its shared post-match continuation once per arm —
    /// each arm inlines its own copy of the source-prefix template. The arms are
    /// mutually exclusive at run time (selected by one `brif`), so a checked
    /// subcontinuation frame occurring in that shared continuation is a *distinct
    /// lawful activation per arm*, not a repeated consumption of one activation.
    /// `consumed_subcontinuation_frames` is a single per-lowering set, so without
    /// this fork the second arm's lawful consume of the same
    /// `(invocation_id, frame_id)` is misreported as "consumed more than once"
    /// (RT-ESCAPE: e.g. an escaped resource used by a host op whose `Result`
    /// match fans out). Rewinding to the pre-match baseline before each arm
    /// preserves the affine check *within* a single control-flow path — a real
    /// double-consume on one path still collides — and is neither a set-clear nor
    /// a key-salt: it is per-branch scoping. Unioning the arms afterward keeps
    /// every frame consumed on any arm marked consumed for the post-join
    /// continuation, so a genuine revisit *across* the join still rejects.
    fn lower_forked_branch<'b>(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        frame_scope: &mut CheckedFrameBranchScope,
        expr: OwnedSourceOccurrence,
        env: Vec<LoweringEnvironmentBinding>,
        control: SourceControl<'b>,
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        self.consumed_subcontinuation_frames = frame_scope.start_successor();
        let lowered = self.lower_source_machine_with_continuation(builder, expr, env, control)?;
        frame_scope.merge_successor(&self.consumed_subcontinuation_frames);
        Ok(lowered)
    }

    fn lower_source_dynamic_bool_match<'b>(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        condition: cranelift_codegen::ir::Value,
        true_body: SourceOccurrence<'_>,
        false_body: SourceOccurrence<'_>,
        static_origin: StaticOriginId,
        env: &[LoweringEnvironmentBinding],
        suffix_control: SourceControl<'b>,
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        let (source_prefix_template, terminal) =
            Self::split_source_prefix(suffix_control.continuation)?;
        let mut local_completion = None;
        let target = match terminal {
            SourcePrefixTerminal::Join(inherited_edge) => inherited_edge.target,
            SourcePrefixTerminal::ResumeOuter { root_authority } => {
                let active = suffix_control
                    .selected
                    .as_active(&suffix_control.selected_lineage);
                let (prefix, suffix_pending, required_kind, site_id) =
                    self.planned_active_scalar_cut(active)?;
                let join_id = self.next_source_join;
                self.next_source_join = self
                    .next_source_join
                    .checked_add(1)
                    .expect("compiler-private source join identity exhausted");
                let join_plan = std::rc::Rc::new(self.consumed_join_plan_token(static_origin)?);
                let merge = builder.create_block();
                self.append_planned_join_params(builder, merge, join_plan.as_ref());
                local_completion = Some((
                    merge,
                    suffix_pending.to_vec(),
                    required_kind,
                    site_id,
                    root_authority,
                ));
                SourceJoinTarget {
                    join_id,
                    block: merge,
                    expected_outer: suffix_control.terminal_outer,
                    required_kind,
                    join_plan,
                    result_origin: static_origin,
                    terminal_active_prefix: prefix,
                }
            }
        };
        let true_block = builder.create_block();
        let false_block = builder.create_block();
        builder
            .ins()
            .brif(condition, true_block, &[], false_block, &[]);
        let mut frame_scope =
            CheckedFrameBranchScope::capture(&self.consumed_subcontinuation_frames);
        for (predecessor_id, block, body) in
            [(0, true_block, true_body), (1, false_block, false_body)]
        {
            builder.switch_to_block(block);
            let edge = self.mint_source_predecessor(target.clone());
            let continuation =
                Self::instantiate_source_prefix_template(&source_prefix_template, edge)?;
            let branch_control = SourceControl {
                continuation,
                selected: suffix_control.selected.clone(),
                selected_lineage: suffix_control.selected_lineage.clone(),
                terminal_outer: suffix_control.terminal_outer,
            };
            let lowered = self.lower_forked_branch(
                builder,
                &mut frame_scope,
                OwnedSourceOccurrence::cloned(body),
                env.to_vec(),
                branch_control,
            )?;
            if self.seal_source_trap_branch(builder, &lowered)? {
                // A trap terminates this mutually exclusive predecessor.
            } else if !matches!(
                lowered,
                LoweringOperand::Specialized(Lowered::RecursiveBackedge)
            ) {
                return Err(unsupported(
                    "NativeJoinPlanV1",
                    format!(
                        "Bool predecessor {predecessor_id} did not seal its distinct affine join edge"
                    ),
                ));
            }
        }
        self.consumed_subcontinuation_frames = frame_scope.finish();
        let Some((merge, suffix_pending, required_kind, _site_id, root_authority)) =
            local_completion
        else {
            return Ok(LoweringOperand::Specialized(Lowered::RecursiveBackedge));
        };
        let merged = self.finish_planned_join(
            builder,
            merge,
            target.join_plan.as_ref(),
            Some(required_kind),
            "NativeJoinPlanV1",
        );
        let suffix_active = ActiveContinuationFrame {
            activation: suffix_control.selected.activation,
            cursor: suffix_control.selected.cursor,
            parent: suffix_control.selected.parent,
            pending: &suffix_pending,
            selected_ancestry: &suffix_control.selected.selected_ancestry,
            source_lineage: &suffix_control.selected_lineage,
            source_selected_cursor: Some(suffix_control.selected.cursor),
            selected_scope: suffix_control.selected.selected_scope.as_ref(),
        };
        self.restore_root_terminal_authority(root_authority, suffix_control.terminal_outer)?;
        self.resume_active_continuation(builder, merged?, suffix_active)
    }

    #[allow(clippy::too_many_arguments)]
    fn lower_source_dynamic_host_result_match<'b>(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        success: cranelift_codegen::ir::Value,
        error: Lowered,
        ok: Lowered,
        err_constructor: &str,
        ok_constructor: &str,
        cases: &[crate::RuntimeMatchCase],
        default: RuntimeTrap,
        static_origin: StaticOriginId,
        env: &[LoweringEnvironmentBinding],
        suffix_control: SourceControl<'b>,
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        let (source_prefix_template, terminal) =
            Self::split_source_prefix(suffix_control.continuation)?;
        let mut local_completion = None;
        let target = match terminal {
            SourcePrefixTerminal::Join(inherited_edge) => inherited_edge.target,
            SourcePrefixTerminal::ResumeOuter { root_authority } => {
                let active = suffix_control
                    .selected
                    .as_active(&suffix_control.selected_lineage);
                let (prefix, suffix_pending, required_kind, site_id) =
                    self.planned_active_scalar_cut(active)?;
                let join_id = self.next_source_join;
                self.next_source_join = self
                    .next_source_join
                    .checked_add(1)
                    .expect("compiler-private source join identity exhausted");
                let join_plan = std::rc::Rc::new(self.consumed_join_plan_token(static_origin)?);
                let merge = builder.create_block();
                self.append_planned_join_params(builder, merge, join_plan.as_ref());
                local_completion = Some((
                    merge,
                    suffix_pending.to_vec(),
                    required_kind,
                    site_id,
                    root_authority,
                ));
                SourceJoinTarget {
                    join_id,
                    block: merge,
                    expected_outer: suffix_control.terminal_outer,
                    required_kind,
                    join_plan,
                    result_origin: static_origin,
                    terminal_active_prefix: prefix,
                }
            }
        };
        let ok_block = builder.create_block();
        let err_block = builder.create_block();
        builder.ins().brif(success, ok_block, &[], err_block, &[]);

        let mut frame_scope =
            CheckedFrameBranchScope::capture(&self.consumed_subcontinuation_frames);
        for (predecessor_id, block, constructor, payload) in [
            (0, ok_block, ok_constructor, ok),
            (1, err_block, err_constructor, error),
        ] {
            builder.switch_to_block(block);
            let edge = self.mint_source_predecessor(target.clone());
            let continuation =
                Self::instantiate_source_prefix_template(&source_prefix_template, edge)?;
            let branch_control = SourceControl {
                continuation,
                selected: suffix_control.selected.clone(),
                selected_lineage: suffix_control.selected_lineage.clone(),
                terminal_outer: suffix_control.terminal_outer,
            };
            let lowered = if let Some((index, case)) = cases
                .iter()
                .enumerate()
                .find(|(_, case)| case.constructor == constructor && case.binders == 1)
            {
                let arm_env = env_with([payload], env);
                let body =
                    self.owned_case_body_occurrence(static_origin, index, case.body.clone())?;
                self.lower_forked_branch(builder, &mut frame_scope, body, arm_env, branch_control)?
            } else {
                // ⚠ THE ONE SYNTHESIZED TERM in the whole lowering: no source
                // occurrence exists for "this alternative has no case", so the
                // machine is handed a fresh `Trap` built from the match's own
                // `default`. `default` is an ATOM of the match occurrence, not a
                // child of it, so the honest origin for this term is the match
                // occurrence's own — and `Trap` is a leaf, so no child is ever
                // derived from it. ⛔ Do not mint an origin here.
                self.lower_forked_branch(
                    builder,
                    &mut frame_scope,
                    OwnedSourceOccurrence {
                        expr: RuntimeExpr::Trap(default.clone()),
                        static_origin,
                    },
                    env.to_vec(),
                    branch_control,
                )?
            };
            if self.seal_source_trap_branch(builder, &lowered)? {
                // A trap terminates this mutually exclusive predecessor.
            } else if !matches!(
                lowered,
                LoweringOperand::Specialized(Lowered::RecursiveBackedge)
            ) {
                return Err(unsupported(
                    "NativeJoinPlanV1",
                    format!(
                        "HostResult predecessor {predecessor_id} did not seal its distinct affine join edge"
                    ),
                ));
            }
        }
        self.consumed_subcontinuation_frames = frame_scope.finish();

        let Some((merge, suffix_pending, required_kind, _site_id, root_authority)) =
            local_completion
        else {
            return Ok(LoweringOperand::Specialized(Lowered::RecursiveBackedge));
        };
        let merged = self.finish_planned_join(
            builder,
            merge,
            target.join_plan.as_ref(),
            Some(required_kind),
            "NativeJoinPlanV1",
        );
        let suffix_active = ActiveContinuationFrame {
            activation: suffix_control.selected.activation,
            cursor: suffix_control.selected.cursor,
            parent: suffix_control.selected.parent,
            pending: &suffix_pending,
            selected_ancestry: &suffix_control.selected.selected_ancestry,
            source_lineage: &suffix_control.selected_lineage,
            source_selected_cursor: Some(suffix_control.selected.cursor),
            selected_scope: suffix_control.selected.selected_scope.as_ref(),
        };
        self.restore_root_terminal_authority(root_authority, suffix_control.terminal_outer)?;
        self.resume_active_continuation(builder, merged?, suffix_active)
    }

    /// One emitted-case descriptor, resolved BEFORE any block exists.
    ///
    /// Every planner and schema question the carried source-machine `Match`
    /// route asks is answered here. Once the first block is created the
    /// selector construction consults only these descriptors and the emitted
    /// carrier helpers, so a malformed table cannot be discovered halfway
    /// through an emitted graph.
    fn source_carried_descriptors(
        &self,
        cases: &[crate::RuntimeMatchCase],
        static_origin: StaticOriginId,
    ) -> Result<Vec<SourceCarriedCase>, CraneliftBackendError> {
        let mut descriptors = Vec::with_capacity(cases.len());
        for (index, case) in cases.iter().enumerate() {
            let emitted = self.source_carried_case_is_emitted(static_origin, index)?;
            let identity = self
                .static_transition_plan
                .case_constructor_identity(static_origin, index)?
                .tag_abi_word()?;
            let binders = i64::try_from(case.binders).map_err(|_| {
                unsupported(
                    "BoundaryCarrier",
                    "a case binds more constructor arguments than the carrier ABI can count",
                )
            })?;
            descriptors.push(SourceCarriedCase {
                index,
                emitted,
                identity,
                binders,
                borrowed: borrowed_constructor_identity(&self.process_symbols, &case.constructor),
            });
        }
        Ok(descriptors)
    }

    /// The planner's emission verdict for one ordinary `Match` case, asked per
    /// case and never defaulted. An origin with no record is a refusal to
    /// answer, so it fails closed rather than being read as `Reachable`.
    fn source_carried_case_is_emitted(
        &self,
        static_origin: StaticOriginId,
        index: usize,
    ) -> Result<bool, CraneliftBackendError> {
        let status = self
            .static_transition_plan
            .case_emission_status(static_origin, index)?
            .ok_or_else(|| {
                unsupported(
                    "Match",
                    "a carried source match has no planned case-emission verdict",
                )
            })?;
        // Exhaustive over the verdict, with no wildcard: a third status would
        // be a compile error here rather than silently reading as "do not
        // emit".
        Ok(match status {
            CaseEmissionStatus::Reachable => true,
            CaseEmissionStatus::Eliminated => false,
        })
    }

    /// Lower one preallocated semantic leaf of a carried source match.
    ///
    /// The leaf already holds its bindings as block parameters, so the selector
    /// graph that jumped here is closed. The CALLER mints this leaf's one
    /// source predecessor and instantiates the prefix on it, then hands the
    /// resulting control in; this lowers the case body under it. The only
    /// accepted terminal results are a sealed trap or a `RecursiveBackedge`.
    #[allow(clippy::too_many_arguments)]
    fn lower_source_carried_leaf<'b>(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        frame_scope: &mut CheckedFrameBranchScope,
        static_origin: StaticOriginId,
        index: usize,
        body: RuntimeExpr,
        bindings: Vec<LoweringOperand>,
        env: &[LoweringEnvironmentBinding],
        branch_control: SourceControl<'b>,
    ) -> Result<(), CraneliftBackendError> {
        let case_body = self.owned_case_body_occurrence(static_origin, index, body)?;
        let lowered = self.lower_forked_branch(
            builder,
            frame_scope,
            case_body,
            env_with_operands(bindings, env),
            branch_control,
        )?;
        if self.seal_source_trap_branch(builder, &lowered)? {
            // A trap terminates this mutually exclusive predecessor.
            return Ok(());
        }
        if !matches!(
            lowered,
            LoweringOperand::Specialized(Lowered::RecursiveBackedge)
        ) {
            return Err(unsupported(
                "NativeJoinPlanV1",
                format!("carried-match leaf {index} did not seal its distinct affine join edge"),
            ));
        }
        Ok(())
    }

    /// Eliminate a CARRIED scrutinee at the source-machine ordinary `Match`
    /// seat, under the source machine's own continuation discipline.
    ///
    /// A runtime boundary word has no compile-time template, so the case cannot
    /// be selected at compile time. The dispatch is the carrier ABI's and
    /// nothing else: the canonical case identity comes from the planner, the tag
    /// and field count are read through the carrier, and each projected child
    /// stays `Carried` into the case environment. There is no decode and no
    /// `Carried -> Lowered` reconstruction anywhere on this path.
    ///
    /// This does NOT delegate to `Lowering::lower_carried_match`. That helper
    /// belongs to the generic `lower_expr` seat, which returns a value to its
    /// caller; this seat owns a continuation, so each case body must be lowered
    /// under the match's original `next` through a distinct source predecessor
    /// edge, and the suffix resumed exactly once at the join.
    ///
    /// Case emission is the planner's verdict, not this emitter's: a case the
    /// scrutinee's closed producer set eliminates is not emitted and its body is
    /// not lowered. The runtime closed default covers it, so an eliminated
    /// constructor arriving anyway still reaches a defined outcome rather than
    /// falling into a neighbouring case.
    fn lower_source_carried_match<'b>(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        scrutinee: CarriedBoundaryWord,
        cases: &[crate::RuntimeMatchCase],
        default: &RuntimeTrap,
        static_origin: StaticOriginId,
        env: &[LoweringEnvironmentBinding],
        suffix_control: SourceControl<'b>,
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        // ── PHASE 0 — PREVALIDATE, BEFORE A SINGLE BLOCK EXISTS ──
        //
        // A malformed table must fail as an `Err`, not as half an emitted
        // graph. Every planner and schema question is answered here, including
        // the carrier arena's pointer type, so nothing below the first
        // `create_block` asks one.
        let descriptors = self.source_carried_descriptors(cases, static_origin)?;
        let pointer_type = builder.func.dfg.value_type(self.carrier_arena()?);
        let emitted: Vec<usize> = descriptors
            .iter()
            .filter(|descriptor| descriptor.emitted)
            .map(|descriptor| descriptor.index)
            .collect();

        // The two Result alternatives are resolved INDEPENDENTLY. A decoded
        // `HostResult` is a fact about the word, not about the case list, so
        // neither the absence of one alternative nor the absence of both may
        // route it to representation mismatch -- it takes the closed default
        // instead. The binder shape is therefore validated only for an
        // alternative that is actually present.
        let ok_case = cases
            .iter()
            .position(|case| case.constructor == self.process_symbols.result_ok);
        let err_case = cases
            .iter()
            .position(|case| case.constructor == self.process_symbols.result_err);
        for alternative in [ok_case, err_case].into_iter().flatten() {
            if cases[alternative].binders != 1 {
                return Err(unsupported(
                    "HostResult",
                    "a present carried Result case must bind exactly one selected payload",
                ));
            }
        }

        // The whole case set must be process-input family: a runtime class test
        // alone would wrongly demand borrowed identities of unrelated source
        // constructors, which is why this is decided from the cases.
        let admits_borrowed = self.process_object
            && descriptors
                .iter()
                .all(|descriptor| descriptor.borrowed.is_some());
        if admits_borrowed {
            for descriptor in &descriptors {
                let (_, arity) = descriptor.borrowed.expect("admitted family");
                if cases[descriptor.index].binders != arity {
                    return Err(unsupported(
                        "Match",
                        format!(
                            "{} borrowed arity mismatch",
                            cases[descriptor.index].constructor
                        ),
                    ));
                }
            }
        }

        let (source_prefix_template, terminal) =
            Self::split_source_prefix(suffix_control.continuation)?;
        let mut local_completion = None;
        let target = match terminal {
            SourcePrefixTerminal::Join(inherited_edge) => {
                // Family 2a control seam. The prefix split has already
                // classified this terminal as an inherited join; the hook sits
                // before the target is used and never rewrites it.
                #[cfg(test)]
                if let Some(refusal) = source_carried_control_refusal(
                    SourceCarriedControlMutation::RefuseSplitInheritedJoin,
                    "NativeJoinPlanV1",
                    "MUTATION inherited join acquisition perturbed",
                ) {
                    return Err(refusal);
                }
                inherited_edge.target
            }
            SourcePrefixTerminal::ResumeOuter { root_authority } => {
                let active = suffix_control
                    .selected
                    .as_active(&suffix_control.selected_lineage);
                let (prefix, suffix_pending, required_kind, site_id) =
                    self.planned_active_scalar_cut(active)?;
                let join_id = self.next_source_join;
                self.next_source_join = self
                    .next_source_join
                    .checked_add(1)
                    .expect("compiler-private source join identity exhausted");
                let join_plan = std::rc::Rc::new(self.consumed_join_plan_token(static_origin)?);
                let merge = builder.create_block();
                self.append_planned_join_params(builder, merge, join_plan.as_ref());
                local_completion = Some((
                    merge,
                    suffix_pending.to_vec(),
                    required_kind,
                    site_id,
                    root_authority,
                ));
                SourceJoinTarget {
                    join_id,
                    block: merge,
                    expected_outer: suffix_control.terminal_outer,
                    required_kind,
                    join_plan,
                    result_origin: static_origin,
                    terminal_active_prefix: prefix,
                }
            }
        };
        let arm_selected = suffix_control.selected.clone();
        let arm_lineage = suffix_control.selected_lineage.clone();
        let arm_outer = suffix_control.terminal_outer;

        // ── PHASE 1 — ALLOCATE EVERY BLOCK ──
        //
        // Leaves take their bindings as BLOCK PARAMETERS. That is what makes
        // the selector graph closable before any body is lowered: a selector
        // never has to wait for a leaf, it just jumps with the values it read.
        let mismatch_block = builder.create_block();
        let default_block = builder.create_block();

        // One carried leaf per emitted case. Constructor and `HostResult` SHARE
        // a case's carried leaf, which is sound exactly because their payload
        // shapes agree there: a Result case binds one child, and the
        // represented route projects one field for it.
        let mut carried_leaves = Vec::new();
        for &index in &emitted {
            let leaf = builder.create_block();
            for _ in 0..cases[index].binders {
                builder.append_block_param(leaf, types::I64);
            }
            carried_leaves.push((index, leaf));
        }
        // Borrowed leaves are DISTINCT, never shared with the carried ones:
        // their environment is `BorrowedNativeValue` pointers into host memory,
        // and relabelling those as represented carrier children is precisely
        // the mislabelling this route must not do.
        let mut borrowed_leaves = Vec::new();
        if admits_borrowed {
            for &index in &emitted {
                let leaf = builder.create_block();
                for _ in 0..descriptors[index].borrowed.expect("admitted family").1 {
                    builder.append_block_param(leaf, pointer_type);
                }
                borrowed_leaves.push((index, leaf));
            }
        }
        let leaf_of = |leaves: &[(usize, cranelift_codegen::ir::Block)], index: usize| {
            leaves
                .iter()
                .find(|(candidate, _)| *candidate == index)
                .map(|(_, block)| *block)
        };

        // ── PHASE 2 — EMIT AND TERMINATE THE ENTIRE SELECTOR GRAPH ──
        //
        // Nothing below lowers a source body. Every path created here ends in a
        // `brif`, a `jump` or a `return_`, so no block is left live when leaf
        // lowering begins.
        let class = self.emit_carrier_class(builder, scrutinee)?;
        let mut class_test = builder.current_block().expect("carried source match block");

        {
            // Emitted unconditionally: whether the word IS a `HostResult` is a
            // runtime fact, so the selector exists even when the source cases
            // mention neither alternative. Each alternative resolves to its own
            // leaf, or to the shared default when it is absent or eliminated.
            let host_result = builder.create_block();
            let next_class = builder.create_block();
            if builder.current_block() != Some(class_test) {
                builder.switch_to_block(class_test);
            }
            let is_host_result = builder.ins().icmp_imm(
                cranelift_codegen::ir::condcodes::IntCC::Equal,
                class,
                BoundaryClass::HostResult as i64,
            );
            builder
                .ins()
                .brif(is_host_result, host_result, &[], next_class, &[]);

            builder.switch_to_block(host_result);
            let success = self.emit_carrier_host_success(builder, scrutinee)?;
            let payload = self.emit_carrier_host_payload(builder, scrutinee)?;
            let alternative_leaf = |alternative: Option<usize>| {
                alternative
                    .and_then(|index| leaf_of(&carried_leaves, index))
                    .unwrap_or(default_block)
            };
            let ok_target = alternative_leaf(ok_case);
            let err_target = alternative_leaf(err_case);
            let ok_args: Vec<_> = if ok_target == default_block {
                Vec::new()
            } else {
                vec![payload.word.into()]
            };
            let err_args: Vec<_> = if err_target == default_block {
                Vec::new()
            } else {
                vec![payload.word.into()]
            };
            builder
                .ins()
                .brif(success, ok_target, &ok_args, err_target, &err_args);
            class_test = next_class;
        }

        if admits_borrowed {
            let borrowed = builder.create_block();
            let next_class = builder.create_block();
            if builder.current_block() != Some(class_test) {
                builder.switch_to_block(class_test);
            }
            let is_borrowed = builder.ins().icmp_imm(
                cranelift_codegen::ir::condcodes::IntCC::Equal,
                class,
                BoundaryClass::BorrowedOpaque as i64,
            );
            builder
                .ins()
                .brif(is_borrowed, borrowed, &[], next_class, &[]);

            builder.switch_to_block(borrowed);
            let pointer = self.emit_carrier_scalar(builder, scrutinee)?;
            let kind = builder
                .ins()
                .load(types::I64, MemFlags::trusted(), pointer, 0);
            Self::require_i64(builder, kind, 2);
            let tag = builder
                .ins()
                .load(types::I64, MemFlags::trusted(), pointer, 8);
            let arity = builder
                .ins()
                .load(types::I64, MemFlags::trusted(), pointer, 24);
            let fields = builder
                .ins()
                .load(pointer_type, MemFlags::trusted(), pointer, 16);
            let mut borrowed_test = builder
                .current_block()
                .expect("borrowed carried source match block");
            for &index in &emitted {
                let (expected_tag, expected_arity) =
                    descriptors[index].borrowed.expect("admitted family");
                let leaf = leaf_of(&borrowed_leaves, index)
                    .expect("every emitted case has a borrowed leaf when admitted");
                let selected = builder.create_block();
                let next = builder.create_block();
                if builder.current_block() != Some(borrowed_test) {
                    builder.switch_to_block(borrowed_test);
                }
                let matched = builder.ins().icmp_imm(
                    cranelift_codegen::ir::condcodes::IntCC::Equal,
                    tag,
                    expected_tag,
                );
                builder.ins().brif(matched, selected, &[], next, &[]);

                builder.switch_to_block(selected);
                Self::require_i64(builder, arity, expected_arity as i64);
                if expected_arity != 0 {
                    Self::require_nonzero(builder, fields);
                }
                let projected: Vec<_> = (0..expected_arity)
                    .map(|position| {
                        builder
                            .ins()
                            .iadd_imm(fields, (position * 32) as i64)
                            .into()
                    })
                    .collect();
                builder.ins().jump(leaf, &projected);
                borrowed_test = next;
            }
            builder.switch_to_block(borrowed_test);
            builder.ins().jump(default_block, &[]);
            class_test = next_class;
        }

        // The represented-constructor chain, and the residual class test. A
        // class this case set never admitted reaches `mismatch_block`.
        let constructor_block = builder.create_block();
        if builder.current_block() != Some(class_test) {
            builder.switch_to_block(class_test);
        }
        let is_constructor = builder.ins().icmp_imm(
            cranelift_codegen::ir::condcodes::IntCC::Equal,
            class,
            BoundaryClass::Constructor as i64,
        );
        builder
            .ins()
            .brif(is_constructor, constructor_block, &[], mismatch_block, &[]);

        builder.switch_to_block(constructor_block);
        let tag = self.emit_carrier_tag(builder, scrutinee)?;
        let field_count = self.emit_carrier_field_count(builder, scrutinee)?;
        let mut tag_test = builder
            .current_block()
            .expect("represented carried source match block");
        for &index in &emitted {
            let leaf =
                leaf_of(&carried_leaves, index).expect("every emitted case has a carried leaf");
            let selected = builder.create_block();
            let next = builder.create_block();
            if builder.current_block() != Some(tag_test) {
                builder.switch_to_block(tag_test);
            }
            let identity = descriptors[index].identity;
            let matched = builder.ins().icmp_imm(
                cranelift_codegen::ir::condcodes::IntCC::Equal,
                tag,
                identity as i64,
            );
            builder.ins().brif(matched, selected, &[], next, &[]);

            builder.switch_to_block(selected);
            let binders = descriptors[index].binders;
            // The declared arity is checked against the word actually
            // delivered, at runtime, which is where a disagreement between a
            // case header and a boundary value belongs.
            Self::require_i64(builder, field_count, binders);
            let mut projected = Vec::with_capacity(cases[index].binders);
            for position in 0..cases[index].binders {
                let child = self.emit_carrier_field(builder, scrutinee, position)?;
                projected.push(child.word.into());
            }
            builder.ins().jump(leaf, &projected);
            tag_test = next;
        }
        builder.switch_to_block(tag_test);
        builder.ins().jump(default_block, &[]);

        // The representation this case set never decoded. It is NOT the match's
        // default: nothing was decoded, so no alternative was ruled out. It
        // mints no predecessor, consumes no branch frame, joins nothing, and
        // does not lower the default.
        builder.switch_to_block(mismatch_block);
        let mismatch = builder
            .ins()
            .iconst(types::I64, CARRIED_REPRESENTATION_MISMATCH_STATUS);
        builder.ins().return_(&[mismatch]);

        // ── PHASE 3 — LOWER ONLY THE PREALLOCATED SEMANTIC LEAVES ──
        let mut frame_scope =
            CheckedFrameBranchScope::capture(&self.consumed_subcontinuation_frames);

        for (index, leaf) in carried_leaves {
            builder.switch_to_block(leaf);
            let bindings: Vec<_> = builder
                .block_params(leaf)
                .to_vec()
                .into_iter()
                .map(|word| LoweringOperand::Carried(CarriedBoundaryWord { word }))
                .collect();
            let edge = self.mint_source_predecessor(target.clone());
            let continuation =
                Self::instantiate_source_prefix_template(&source_prefix_template, edge)?;
            let branch_control = SourceControl {
                continuation,
                selected: arm_selected.clone(),
                selected_lineage: arm_lineage.clone(),
                terminal_outer: arm_outer,
            };
            self.lower_source_carried_leaf(
                builder,
                &mut frame_scope,
                static_origin,
                index,
                cases[index].body.clone(),
                bindings,
                env,
                branch_control,
            )?;
        }

        for (index, leaf) in borrowed_leaves {
            builder.switch_to_block(leaf);
            let bindings: Vec<_> = builder
                .block_params(leaf)
                .to_vec()
                .into_iter()
                .map(|pointer| {
                    LoweringOperand::Specialized(Lowered::BorrowedNativeValue { pointer })
                })
                .collect();
            let edge = self.mint_source_predecessor(target.clone());
            let continuation =
                Self::instantiate_source_prefix_template(&source_prefix_template, edge)?;
            let branch_control = SourceControl {
                continuation,
                selected: arm_selected.clone(),
                selected_lineage: arm_lineage.clone(),
                terminal_outer: arm_outer,
            };
            self.lower_source_carried_leaf(
                builder,
                &mut frame_scope,
                static_origin,
                index,
                cases[index].body.clone(),
                bindings,
                env,
                branch_control,
            )?;
        }

        // The shared semantic default, lowered exactly once. `default` is an
        // ATOM of the match occurrence rather than a child of it, so the honest
        // origin for this synthesized term is the match occurrence's own;
        // `Trap` is a leaf, so no child is derived from it. Do not mint an
        // origin here.
        builder.switch_to_block(default_block);
        let edge = self.mint_source_predecessor(target.clone());
        let continuation = Self::instantiate_source_prefix_template(&source_prefix_template, edge)?;
        let default_control = SourceControl {
            continuation,
            selected: arm_selected.clone(),
            selected_lineage: arm_lineage.clone(),
            terminal_outer: arm_outer,
        };
        let lowered = self.lower_forked_branch(
            builder,
            &mut frame_scope,
            OwnedSourceOccurrence {
                expr: RuntimeExpr::Trap(default.clone()),
                static_origin,
            },
            env.to_vec(),
            default_control,
        )?;
        if !self.seal_source_trap_branch(builder, &lowered)?
            && !matches!(
                lowered,
                LoweringOperand::Specialized(Lowered::RecursiveBackedge)
            )
        {
            return Err(unsupported(
                "NativeJoinPlanV1",
                "carried-match default predecessor did not seal its distinct affine join edge",
            ));
        }

        // ── PHASE 4 — UNION ONCE, FINISH ONCE ──
        self.consumed_subcontinuation_frames = frame_scope.finish();
        let Some((merge, suffix_pending, required_kind, _site_id, root_authority)) =
            local_completion
        else {
            return Ok(LoweringOperand::Specialized(Lowered::RecursiveBackedge));
        };
        let merged = self.finish_planned_join(
            builder,
            merge,
            target.join_plan.as_ref(),
            Some(required_kind),
            "NativeJoinPlanV1",
        );
        let suffix_active = ActiveContinuationFrame {
            activation: suffix_control.selected.activation,
            cursor: suffix_control.selected.cursor,
            parent: suffix_control.selected.parent,
            pending: &suffix_pending,
            selected_ancestry: &suffix_control.selected.selected_ancestry,
            source_lineage: &suffix_control.selected_lineage,
            source_selected_cursor: Some(suffix_control.selected.cursor),
            selected_scope: suffix_control.selected.selected_scope.as_ref(),
        };
        self.restore_root_terminal_authority(root_authority, suffix_control.terminal_outer)?;
        self.resume_active_continuation(builder, merged?, suffix_active)
    }

    fn lower_source_dynamic_constructor_match<'b>(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        dynamic: DynamicConstructorV1,
        cases: &[crate::RuntimeMatchCase],
        default: &RuntimeTrap,
        static_origin: StaticOriginId,
        env: &[LoweringEnvironmentBinding],
        suffix_control: SourceControl<'b>,
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        validate_dynamic_constructor_alternatives(
            dynamic
                .alternatives
                .iter()
                .map(|alternative| (alternative.tag, alternative.constructor.as_str())),
        )?;
        if Self::source_terminal_join(&suffix_control.continuation).is_some() {
            return self.lower_source_nested_dynamic_constructor_match(
                builder,
                dynamic,
                cases,
                default,
                static_origin,
                env,
                suffix_control,
            );
        }
        self.lower_source_planned_dynamic_constructor_match(
            builder,
            dynamic,
            cases,
            default,
            static_origin,
            env,
            suffix_control,
        )
    }

    fn lower_source_nested_dynamic_constructor_match<'b>(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        dynamic: DynamicConstructorV1,
        cases: &[crate::RuntimeMatchCase],
        default: &RuntimeTrap,
        static_origin: StaticOriginId,
        env: &[LoweringEnvironmentBinding],
        suffix_control: SourceControl<'b>,
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        let (source_prefix_template, terminal) =
            Self::split_source_prefix(suffix_control.continuation)?;
        let SourcePrefixTerminal::Join(inherited_edge) = terminal else {
            return Err(unsupported(
                "NativeJoinPlanV1",
                "nested dynamic constructor has no affine terminal edge",
            ));
        };
        let fanout = SourceBranchFanout {
            source_prefix_template,
            inherited_edge,
        };
        let target = fanout.inherited_edge.target;
        let mut test_block = builder
            .current_block()
            .expect("dynamic constructor source match block");
        let mut frame_scope =
            CheckedFrameBranchScope::capture(&self.consumed_subcontinuation_frames);
        for alternative in dynamic.alternatives {
            let arm = builder.create_block();
            let next = builder.create_block();
            if builder.current_block() != Some(test_block) {
                builder.switch_to_block(test_block);
            }
            let selected = builder.ins().icmp_imm(
                cranelift_codegen::ir::condcodes::IntCC::Equal,
                dynamic.discriminator,
                alternative.tag,
            );
            builder.ins().brif(selected, arm, &[], next, &[]);
            builder.switch_to_block(arm);
            let (case_index, case) =
                match select_dynamic_constructor_case(cases, &alternative, default)? {
                    Ok(selected) => selected,
                    Err(_) => {
                        let failure = builder.ins().iconst(types::I64, -4);
                        builder.ins().return_(&[failure]);
                        test_block = next;
                        continue;
                    }
                };
            let edge = self.mint_source_predecessor(target.clone());
            let continuation =
                Self::instantiate_source_prefix_template(&fanout.source_prefix_template, edge)?;
            let control = SourceControl {
                continuation,
                selected: suffix_control.selected.clone(),
                selected_lineage: suffix_control.selected_lineage.clone(),
                terminal_outer: suffix_control.terminal_outer,
            };
            let lowered = self.lower_forked_branch(
                builder,
                &mut frame_scope,
                self.owned_case_body_occurrence(static_origin, case_index, case.body.clone())?,
                materialize_dynamic_constructor_env(&alternative, env),
                control,
            )?;
            if self.seal_source_trap_branch(builder, &lowered)? {
                // A trap terminates this mutually exclusive predecessor.
            } else if !matches!(
                lowered,
                LoweringOperand::Specialized(Lowered::RecursiveBackedge)
            ) {
                return Err(unsupported(
                    "NativeJoinPlanV1",
                    "nested dynamic constructor predecessor did not seal its edge",
                ));
            }
            test_block = next;
        }
        self.consumed_subcontinuation_frames = frame_scope.finish();
        builder.switch_to_block(test_block);
        let malformed = builder
            .ins()
            .iconst(types::I64, MALFORMED_DYNAMIC_CONSTRUCTOR_STATUS);
        builder.ins().return_(&[malformed]);
        Ok(LoweringOperand::Specialized(Lowered::RecursiveBackedge))
    }

    fn lower_source_planned_dynamic_constructor_match<'b>(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        dynamic: DynamicConstructorV1,
        cases: &[crate::RuntimeMatchCase],
        default: &RuntimeTrap,
        static_origin: StaticOriginId,
        env: &[LoweringEnvironmentBinding],
        suffix_control: SourceControl<'b>,
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        let active = suffix_control
            .selected
            .as_active(&suffix_control.selected_lineage);
        let (prefix, suffix_pending, required_kind, site_id) =
            self.planned_active_scalar_cut(active)?;
        let suffix_pending = suffix_pending.to_vec();
        let join_id = self.next_source_join;
        self.next_source_join = self
            .next_source_join
            .checked_add(1)
            .expect("compiler-private source join identity exhausted");
        let join_plan = std::rc::Rc::new(self.consumed_join_plan_token(static_origin)?);
        let merge = builder.create_block();
        self.append_planned_join_params(builder, merge, join_plan.as_ref());
        let target = SourceJoinTarget {
            join_id,
            block: merge,
            expected_outer: suffix_control.terminal_outer,
            required_kind,
            join_plan,
            result_origin: static_origin,
            terminal_active_prefix: prefix,
        };
        let (source_prefix_template, terminal) =
            Self::split_source_prefix(suffix_control.continuation)?;
        let root_authority = match terminal {
            SourcePrefixTerminal::ResumeOuter { root_authority } => root_authority,
            SourcePrefixTerminal::Join(_) => {
                return Err(unsupported(
                    "NativeJoinPlanV1",
                    "planned dynamic-constructor cut unexpectedly inherited an executable edge",
                ));
            }
        };
        let mut test_block = builder
            .current_block()
            .expect("dynamic constructor source match block");
        let mut frame_scope =
            CheckedFrameBranchScope::capture(&self.consumed_subcontinuation_frames);
        for (predecessor_id, alternative) in dynamic.alternatives.into_iter().enumerate() {
            let arm = builder.create_block();
            let next = builder.create_block();
            if builder.current_block() != Some(test_block) {
                builder.switch_to_block(test_block);
            }
            let selected = builder.ins().icmp_imm(
                cranelift_codegen::ir::condcodes::IntCC::Equal,
                dynamic.discriminator,
                alternative.tag,
            );
            builder.ins().brif(selected, arm, &[], next, &[]);
            builder.switch_to_block(arm);
            let (case_index, case) =
                match select_dynamic_constructor_case(cases, &alternative, default)? {
                    Ok(selected) => selected,
                    Err(_) => {
                        let failure = builder.ins().iconst(types::I64, -4);
                        builder.ins().return_(&[failure]);
                        test_block = next;
                        continue;
                    }
                };
            let edge = self.mint_source_predecessor(target.clone());
            let continuation =
                Self::instantiate_source_prefix_template(&source_prefix_template, edge)?;
            let control = SourceControl {
                continuation,
                selected: suffix_control.selected.clone(),
                selected_lineage: suffix_control.selected_lineage.clone(),
                terminal_outer: suffix_control.terminal_outer,
            };
            let lowered = self.lower_forked_branch(
                builder,
                &mut frame_scope,
                self.owned_case_body_occurrence(static_origin, case_index, case.body.clone())?,
                materialize_dynamic_constructor_env(&alternative, env),
                control,
            )?;
            if self.seal_source_trap_branch(builder, &lowered)? {
                // A trap terminates this mutually exclusive predecessor.
            } else if !matches!(
                lowered,
                LoweringOperand::Specialized(Lowered::RecursiveBackedge)
            ) {
                return Err(unsupported(
                    "NativeJoinPlanV1",
                    format!(
                        "dynamic-constructor predecessor {predecessor_id} for checked site {site_id} did not seal its affine join edge"
                    ),
                ));
            }
            test_block = next;
        }
        self.consumed_subcontinuation_frames = frame_scope.finish();
        builder.switch_to_block(test_block);
        let malformed = builder
            .ins()
            .iconst(types::I64, MALFORMED_DYNAMIC_CONSTRUCTOR_STATUS);
        builder.ins().return_(&[malformed]);
        let merged = self.finish_planned_join(
            builder,
            merge,
            target.join_plan.as_ref(),
            Some(required_kind),
            "NativeJoinPlanV1",
        );
        let suffix_active = ActiveContinuationFrame {
            activation: suffix_control.selected.activation,
            cursor: suffix_control.selected.cursor,
            parent: suffix_control.selected.parent,
            pending: &suffix_pending,
            selected_ancestry: &suffix_control.selected.selected_ancestry,
            source_lineage: &suffix_control.selected_lineage,
            source_selected_cursor: Some(suffix_control.selected.cursor),
            selected_scope: suffix_control.selected.selected_scope.as_ref(),
        };
        self.restore_root_terminal_authority(root_authority, suffix_control.terminal_outer)?;
        self.resume_active_continuation(builder, merged?, suffix_active)
    }

    fn source_call_state<'b>(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        callee: LoweringOperand,
        args: Vec<LoweringOperand>,
        env: Vec<LoweringEnvironmentBinding>,
        control: SourceControl<'b>,
    ) -> Result<SourceCallOutcome<'b>, CraneliftBackendError> {
        // ⭐ A call needs a **callable template** — `params`, `captures`, a body
        // occurrence. A carried boundary word carries none of those and cannot
        // acquire them (`§2g`: the carrier holds the SSA word and nothing else),
        // so this is a specialized-only surface. ⛔ Fails closed.
        let callee = callee.specialized_at("a source-machine call's callee")?;
        match callee {
            Lowered::Closure {
                captures,
                params,
                body,
                boundary_environment,
            } => {
                if params.len() != args.len() {
                    return Err(unsupported(
                        "Call",
                        format!(
                            "closure expects {} args but call provides {}",
                            params.len(),
                            args.len()
                        ),
                    ));
                }
                if let Some(environment) = boundary_environment {
                    let mut inputs = args;
                    inputs.extend(captures);
                    let called = self.call_boundary_closure_environment(
                        builder,
                        environment,
                        body,
                        &inputs,
                    )?;
                    return Ok(SourceCallOutcome::Complete(called));
                }
                // Continued source evaluation: unwrap only, no crossing.
                let mut call_env = bound_values(args);
                extend_captures(&mut call_env, captures);
                call_env.extend(env);
                Ok(SourceCallOutcome::Continue(SourceMachineState::Eval {
                    expr: self.machine_body_occurrence(body)?,
                    env: call_env,
                    control,
                }))
            }
            Lowered::DeclarationClosure {
                reference,
                symbol,
                captures,
                params,
                body,
            } => {
                // `RT-DECL-CLOSURE-PORT` `D4`, consumer 3 of 3 -- the
                // source-machine route. Its arguments are already operands, so
                // it hands them to the shared ordering directly; the arity
                // check lives there for every consumer rather than once here.
                // Crossing into a declared generated unit happens here, before
                // the shared call-target lowering.
                let args = self.carry_source_call_inputs(builder, body, args)?;
                let called = self.call_declaration_closure_unit(
                    builder, reference, &symbol, &params, captures, args,
                )?;
                Ok(SourceCallOutcome::Complete(called))
            }
            mut recursor @ Lowered::ComputationalRecursorClosure { .. } => {
                let transport = match (self.pending_computational_ih_call, &recursor) {
                    (Some(pending), Lowered::ComputationalRecursorClosure { invocation, .. }) => {
                        let plan =
                            self.oriented_subcontinuation_plan.as_ref().ok_or_else(|| {
                                unsupported(
                                    "CheckedIhEnvironmentTransport",
                                    "a checked-IH recursor force has no oriented plan",
                                )
                            })?;
                        let call = plan
                            .computational_ih_call(pending.call_template_id)
                            .ok_or_else(|| {
                                unsupported(
                                    "CheckedIhEnvironmentTransport",
                                    "a checked-IH recursor force has no call template",
                                )
                            })?;
                        if invocation.computational_ih_slot_template_id
                            != Some(call.slot_template_id)
                        {
                            return Err(unsupported(
                                "CheckedIhEnvironmentTransport",
                                "a checked-IH recursor force names a different planned slot",
                            ));
                        }
                        let eligible = match pending.kind {
                            crate::CheckedComputationalIHInvocationKind::OrdinaryApplication
                            | crate::CheckedComputationalIHInvocationKind::CheckedHostComputationTail => {
                                call.arity == 0 && args.is_empty()
                            }
                            crate::CheckedComputationalIHInvocationKind::CheckedHostVisContinuation => {
                                false
                            }
                        };
                        if !eligible {
                            None
                        } else {
                            let body = invocation.recursive_unit_body;
                            let coordinates = CarriedInvocationCoordinates::of(invocation)?;
                            let destination_owner = self.defining_emission_owner.ok_or_else(|| {
                                unsupported(
                                    "CheckedIhEnvironmentTransport",
                                    "a checked-IH transport force has no destination emission owner",
                                )
                            })?;
                            self.static_transition_plan
                                .checked_ih_environment_transport_for_invocation(
                                    destination_owner,
                                    body,
                                    coordinates.continuation_origin,
                                    coordinates.recursive_position,
                                )?
                                .cloned()
                        }
                    }
                    (None, Lowered::ComputationalRecursorClosure { .. }) => None,
                    (_, _) => unreachable!("this arm matched a computational recursor"),
                };
                if let Some(transport) = transport {
                    self.pending_computational_ih_call.take();
                    let returned = self.call_checked_ih_transport_from_case_environment(
                        builder, &transport, &env,
                    )?;
                    return Ok(SourceCallOutcome::Continue(SourceMachineState::Value {
                        value: RoutedAnswer::checked(returned),
                        control,
                    }));
                }
                let checked_ih_invocation =
                    self.mint_checked_computational_ih_instance(&mut recursor)?;
                if let Some(CheckedRecursiveInvocationInstance {
                    source: InvocationTemplateRef::ComputationalIHCall(call_template_id),
                    ..
                }) = checked_ih_invocation
                {
                    let plan = self.oriented_subcontinuation_plan.as_ref().ok_or_else(|| {
                        unsupported(
                            "OrientedSubcontinuationPlanV1",
                            "checked IH invocation has no oriented plan",
                        )
                    })?;
                    let call = plan
                        .computational_ih_call(call_template_id)
                        .ok_or_else(|| {
                            unsupported(
                                "OrientedSubcontinuationPlanV1",
                                "checked IH invocation has no exact call template",
                            )
                        })?;
                    let open = control.selected.selected_scope.as_ref().ok_or_else(|| {
                        unsupported(
                            "OrientedSubcontinuationPlanV1",
                            "checked IH invocation has no selected/open parent occurrence",
                        )
                    })?;
                    self.validate_source_dynamic_splice_parent(
                        checked_ih_invocation.expect("matched checked IH invocation"),
                        open,
                    )?;
                    if call.parent_frame_template_id != open.frame.checked_frame_id
                        || call.parent_segment_site_id
                            != open.frame.checked_frame_id.and_then(|frame_id| {
                                plan.frame(frame_id).map(|frame| frame.segment_site_id)
                            })
                    {
                        return Err(unsupported(
                            "OrientedSubcontinuationPlanV1",
                            "checked IH invocation parent edge does not match the active open occurrence",
                        ));
                    }
                }
                let (base, boundary) =
                    decompose_computational_recursor(LoweringOperand::Specialized(recursor));
                let (activation, invocation) =
                    boundary.expect("recursor closure carries an invocation segment");
                let recursive_unit_body = invocation.recursive_unit_body;
                // `D5a` checkpoint 4 step 1 — read the retained source
                // coordinates BEFORE the segment is installed, beside the
                // existing pre-move field read. Both are facts of the
                // invocation, so both are taken while it is still in hand
                // rather than reconstructed afterwards.
                let carried_coordinates = CarriedInvocationCoordinates::of(&invocation)?;
                if source_active_cursor(
                    &control.selected,
                    &control.selected_lineage,
                    invocation.resume_cursor,
                )
                .is_none()
                    && !recursor_invocation_is_checked(&invocation)
                {
                    return Err(unsupported(
                        "ComputationalRecursor",
                        "recursive invocation cursor is not live in source control",
                    ));
                }
                let armed = ArmedInvocation {
                    suspended: control,
                    expected_selected: invocation.resume_cursor,
                };
                if source_active_cursor(
                    &armed.suspended.selected,
                    &armed.suspended.selected_lineage,
                    armed.expected_selected,
                )
                .is_none()
                    && !recursor_invocation_is_checked(&invocation)
                {
                    return Err(unsupported(
                        "ComputationalRecursor",
                        "armed invocation endpoint changed selected cursor",
                    ));
                }
                // ⭐⭐ `AC-C4` — the carried residual on the source-machine
                // route. ⚠ This is the site where "installs the ALREADY-CHECKED
                // invocation segment" is literal: the refusal below runs
                // **before** `install_recursor_invocation`, which is exactly the
                // ordering control 5 measures.
                if let LoweringOperand::Carried(word) = base {
                    let mut suspended = armed.suspended;
                    suspended.continuation = self.install_recursor_invocation(
                        suspended.continuation,
                        activation,
                        invocation,
                        checked_ih_invocation,
                    )?;
                    #[cfg(test)]
                    d5a_trace(format!(
                        "RT-D2 A INSTALLED owner={:?} continuation_origin={:?} \
recursive_position={:?} body={:?} installed=ok top={:?}",
                        self.defining_emission_owner,
                        carried_coordinates.continuation_origin,
                        carried_coordinates.recursive_position,
                        recursive_unit_body,
                        rt_continuation_kinds(&suspended.continuation),
                    ));
                    if let Some(body) = recursive_unit_body {
                        let coordinates = carried_coordinates;
                        let args = self.carry_source_call_inputs(builder, body, args)?;
                        let value = self.call_declared_recursive_position_unit(
                            builder,
                            body,
                            &args,
                            Some(coordinates),
                        )?;
                        #[cfg(test)]
                        d5a_trace(format!(
                            "RT-D2 B RETURNED body={body:?} continuation_origin={:?} \
recursive_position={:?} returned[{}] still_installed_top={:?}",
                            coordinates.continuation_origin,
                            coordinates.recursive_position,
                            rt_operand_desc(&value),
                            rt_continuation_kinds(&suspended.continuation),
                        ));
                        return Ok(SourceCallOutcome::Continue(SourceMachineState::Value {
                            // ⛔ `D6a`: a declared recursive-position UNIT call is
                            // not a lawful producer, and its result crosses a
                            // function boundary besides -- which carries only the
                            // word. It starts direct; a caller with an exact
                            // claimed call identity re-attests its own.
                            value: RoutedAnswer::direct(value),
                            control: suspended,
                        }));
                    }
                    Self::reject_carried_residual_arguments(args.len())?;
                    return Ok(SourceCallOutcome::Continue(SourceMachineState::Value {
                        value: RoutedAnswer::direct(LoweringOperand::Carried(word)),
                        control: suspended,
                    }));
                }
                let base = base.specialized_at("a recursor residual in a source call")?;
                if let Lowered::BoundedNat(predecessor) = base {
                    if !args.is_empty() {
                        return Err(unsupported(
                            "BoundedNat",
                            "structural Nat recursive hypothesis takes no arguments",
                        ));
                    }
                    let mut suspended = armed.suspended;
                    suspended.continuation = self.install_recursor_invocation(
                        suspended.continuation,
                        activation,
                        invocation,
                        checked_ih_invocation,
                    )?;
                    return Ok(SourceCallOutcome::Continue(SourceMachineState::Value {
                        value: RoutedAnswer::direct(LoweringOperand::Specialized(
                            Lowered::BoundedNat(predecessor),
                        )),
                        control: suspended,
                    }));
                } else {
                    let Lowered::Closure {
                        captures,
                        params,
                        body,
                        ..
                    } = base
                    else {
                        return Err(unsupported(
                            "ComputationalMatch",
                            "recursive constructor field is not a closure",
                        ));
                    };
                    if params.len() != args.len() {
                        return Err(unsupported(
                            "ComputationalMatch",
                            format!(
                                "recursive field expects {} args but call provides {}",
                                params.len(),
                                args.len()
                            ),
                        ));
                    }
                    // Two roles, as elsewhere on this path: ordered unit-call
                    // inputs, or an environment prefix. Only the second binds.
                    // ⚠ The ARGUMENTS cross here; the CAPTURES do not, and that
                    // is a stated boundary rather than an oversight. A capture
                    // arrives inside an already-lowered `Lowered::Closure`, so
                    // a specialized one still reaches
                    // `call_declared_unit_target`'s fallback — where, since it
                    // carries its own producer certificate, it authorizes
                    // itself.
                    let mut call_inputs = self.carry_source_call_inputs(builder, body, args)?;
                    call_inputs.extend(captures);
                    let mut suspended = armed.suspended;
                    suspended.continuation = self.install_recursor_invocation(
                        suspended.continuation,
                        activation,
                        invocation,
                        checked_ih_invocation,
                    )?;
                    let coordinates = carried_coordinates;
                    let value = self.call_declared_recursive_position_unit(
                        builder,
                        body,
                        &call_inputs,
                        Some(coordinates),
                    )?;
                    return Ok(SourceCallOutcome::Continue(SourceMachineState::Value {
                        // A declared recursive-position unit call is not a
                        // lawful producer; its result crosses a function
                        // boundary and carries only the word.
                        value: RoutedAnswer::direct(value),
                        control: suspended,
                    }));
                }
            }
            _ => Err(unsupported("Call", "callee is not a closure")),
        }
    }

}

impl<'a> Lowering<'a> {
    /// The source machine's **owned working copy** of a retained body.
    ///
    /// ⚠ The machine's pending frames own their terms (`OwnedSourceOccurrence`)
    /// and must keep doing so — this is the population boundary of B2A-S, and it
    /// is forced rather than chosen. `lower_source_forked_match` hands the machine
    /// a **synthesized** `RuntimeExpr::Trap` that exists nowhere in the source
    /// tree and therefore has no planned occurrence to be resolved from; a frame
    /// that could only hold a borrowed view of a planned term could not represent
    /// it. So the frames stay owned, and this is where a tag becomes one.
    ///
    /// ⛔ That is **not** a surviving retained-body carrier. The distinction is
    /// which value is authoritative: a `Lowered::Closure` names its body by origin
    /// and holds no term, and this copy is made *at the point of use* from that
    /// name. Re-lowering the resolved term per call site is symptom-inventory
    /// entry 2, which `RT-FNSPLIT-B2F` owns and this WP does not claim.
    fn machine_body_occurrence(
        &self,
        static_origin: StaticOriginId,
    ) -> Result<OwnedSourceOccurrence, CraneliftBackendError> {
        Ok(OwnedSourceOccurrence::cloned(
            self.retained_body_occurrence(static_origin)?,
        ))
    }

    /// The owned form of `child_occurrence`, for the source machine's pending
    /// frames: it takes the child term **by value** and pairs it with its origin
    /// in one constructor, so no step of the machine can hold a term whose origin
    /// was dropped was dropped.
    fn owned_child_occurrence(
        &self,
        parent: StaticOriginId,
        position: usize,
        child: RuntimeExpr,
    ) -> Result<OwnedSourceOccurrence, CraneliftBackendError> {
        Ok(OwnedSourceOccurrence {
            expr: child,
            static_origin: self
                .static_transition_plan
                .child_static_origin(parent, position)?,
        })
    }

    /// The owned form of `case_body_occurrence`, for the source machine.
    fn owned_case_body_occurrence(
        &self,
        parent: StaticOriginId,
        index: usize,
        body: RuntimeExpr,
    ) -> Result<OwnedSourceOccurrence, CraneliftBackendError> {
        self.owned_child_occurrence(parent, 1 + index, body)
    }

}

impl<'a> Lowering<'a> {
    fn source_terminal_join<'b, 'c>(
        continuation: &'b SourceContinuation<'c>,
    ) -> Option<&'b SourceJoinTarget<'c>> {
        match continuation {
            SourceContinuation::Terminal(SourceContinuationTerminal::JumpToJoin(edge)) => {
                Some(&edge.target)
            }
            SourceContinuation::Terminal(
                SourceContinuationTerminal::ReturnValue
                | SourceContinuationTerminal::ReturnToProducerHole { .. }
                | SourceContinuationTerminal::ResumeOuter { .. },
            ) => None,
            SourceContinuation::LetBody { next, .. }
            | SourceContinuation::CheckedRecursiveInvocationReturn { next, .. }
            | SourceContinuation::CheckedComputationalIHInvocationReturn { next, .. }
            | SourceContinuation::ReturnFromSelectedCase { next, .. }
            | SourceContinuation::ApplyRecursorSelection { next, .. }
            | SourceContinuation::UnwindRecursorSegment { next, .. }
            | SourceContinuation::IfScrutinee { next, .. }
            | SourceContinuation::ConstructArgument { next, .. }
            | SourceContinuation::MatchScrutinee { next, .. }
            | SourceContinuation::ComputationalMatchScrutinee { next, .. }
            | SourceContinuation::ProjectRecord { next, .. }
            | SourceContinuation::CallCallee { next, .. }
            | SourceContinuation::CallArgument { next, .. } => Self::source_terminal_join(next),
        }
    }

    fn discard_source_prefix<'b>(continuation: SourceContinuation<'b>) -> SourceContinuation<'b> {
        match continuation {
            terminal @ SourceContinuation::Terminal(_) => terminal,
            SourceContinuation::CheckedRecursiveInvocationReturn { instance, next } => {
                SourceContinuation::CheckedRecursiveInvocationReturn {
                    instance,
                    next: Box::new(Self::discard_source_prefix(*next)),
                }
            }
            SourceContinuation::CheckedComputationalIHInvocationReturn {
                call_template_id,
                next,
            } => SourceContinuation::CheckedComputationalIHInvocationReturn {
                call_template_id,
                next: Box::new(Self::discard_source_prefix(*next)),
            },
            SourceContinuation::ReturnFromSelectedCase { next, .. } => {
                Self::discard_source_prefix(*next)
            }
            SourceContinuation::LetBody { next, .. }
            | SourceContinuation::ApplyRecursorSelection { next, .. }
            | SourceContinuation::UnwindRecursorSegment { next, .. }
            | SourceContinuation::IfScrutinee { next, .. }
            | SourceContinuation::ConstructArgument { next, .. }
            | SourceContinuation::MatchScrutinee { next, .. }
            | SourceContinuation::ComputationalMatchScrutinee { next, .. }
            | SourceContinuation::ProjectRecord { next, .. }
            | SourceContinuation::CallCallee { next, .. }
            | SourceContinuation::CallArgument { next, .. } => Self::discard_source_prefix(*next),
        }
    }

    fn replace_source_terminal_with_unwind<'b>(
        continuation: SourceContinuation<'b>,
        stack: RecursorUnwindStack,
        resume_cursor: ContinuationCursorId,
    ) -> Result<SourceContinuation<'b>, CraneliftBackendError> {
        Ok(match continuation {
            SourceContinuation::CheckedRecursiveInvocationReturn { instance, next } => {
                SourceContinuation::CheckedRecursiveInvocationReturn {
                    instance,
                    next: Box::new(Self::replace_source_terminal_with_unwind(
                        *next,
                        stack,
                        resume_cursor,
                    )?),
                }
            }
            SourceContinuation::CheckedComputationalIHInvocationReturn {
                call_template_id,
                next,
            } => SourceContinuation::CheckedComputationalIHInvocationReturn {
                call_template_id,
                next: Box::new(Self::replace_source_terminal_with_unwind(
                    *next,
                    stack,
                    resume_cursor,
                )?),
            },
            SourceContinuation::ReturnFromSelectedCase { delimiter, next } => {
                SourceContinuation::ReturnFromSelectedCase {
                    delimiter,
                    next: Box::new(Self::replace_source_terminal_with_unwind(
                        *next,
                        stack,
                        resume_cursor,
                    )?),
                }
            }
            SourceContinuation::LetBody { body, env, next } => SourceContinuation::LetBody {
                body,
                env,
                next: Box::new(Self::replace_source_terminal_with_unwind(
                    *next,
                    stack,
                    resume_cursor,
                )?),
            },
            SourceContinuation::ApplyRecursorSelection { layer, next } => {
                SourceContinuation::ApplyRecursorSelection {
                    layer,
                    next: Box::new(Self::replace_source_terminal_with_unwind(
                        *next,
                        stack,
                        resume_cursor,
                    )?),
                }
            }
            SourceContinuation::UnwindRecursorSegment {
                stack: outer_stack,
                resume_cursor: outer_cursor,
                next,
            } => SourceContinuation::UnwindRecursorSegment {
                stack: outer_stack,
                resume_cursor: outer_cursor,
                next: Box::new(Self::replace_source_terminal_with_unwind(
                    *next,
                    stack,
                    resume_cursor,
                )?),
            },
            SourceContinuation::IfScrutinee {
                then_expr,
                else_expr,
                env,
                next,
            } => SourceContinuation::IfScrutinee {
                then_expr,
                else_expr,
                env,
                next: Box::new(Self::replace_source_terminal_with_unwind(
                    *next,
                    stack,
                    resume_cursor,
                )?),
            },
            SourceContinuation::ConstructArgument {
                constructor,
                static_origin,
                remaining: arguments,
                lowered,
                env,
                next,
            } => SourceContinuation::ConstructArgument {
                constructor,
                static_origin,
                remaining: arguments,
                lowered,
                env,
                next: Box::new(Self::replace_source_terminal_with_unwind(
                    *next,
                    stack,
                    resume_cursor,
                )?),
            },
            SourceContinuation::MatchScrutinee {
                cases,
                default,
                env,
                static_origin,
                next,
            } => SourceContinuation::MatchScrutinee {
                cases,
                default,
                env,
                static_origin,
                next: Box::new(Self::replace_source_terminal_with_unwind(
                    *next,
                    stack,
                    resume_cursor,
                )?),
            },
            SourceContinuation::ComputationalMatchScrutinee {
                cases,
                default,
                env,
                static_origin,
                provenance,
                checked_frame_id,
                answer_route,
                next,
            } => SourceContinuation::ComputationalMatchScrutinee {
                cases,
                default,
                env,
                static_origin,
                provenance,
                checked_frame_id,
                answer_route,
                next: Box::new(Self::replace_source_terminal_with_unwind(
                    *next,
                    stack,
                    resume_cursor,
                )?),
            },
            SourceContinuation::ProjectRecord { field, next } => {
                SourceContinuation::ProjectRecord {
                    field,
                    next: Box::new(Self::replace_source_terminal_with_unwind(
                        *next,
                        stack,
                        resume_cursor,
                    )?),
                }
            }
            SourceContinuation::CallCallee { args, env, next } => SourceContinuation::CallCallee {
                args,
                env,
                next: Box::new(Self::replace_source_terminal_with_unwind(
                    *next,
                    stack,
                    resume_cursor,
                )?),
            },
            SourceContinuation::CallArgument {
                callee,
                remaining: arguments,
                lowered,
                env,
                next,
            } => SourceContinuation::CallArgument {
                callee,
                remaining: arguments,
                lowered,
                env,
                next: Box::new(Self::replace_source_terminal_with_unwind(
                    *next,
                    stack,
                    resume_cursor,
                )?),
            },
            SourceContinuation::Terminal(SourceContinuationTerminal::ResumeOuter {
                expected,
                active,
                root_authority,
            }) => SourceContinuation::Terminal(SourceContinuationTerminal::ReturnToProducerHole {
                stack,
                resume_cursor,
                expected,
                active,
                root_authority,
            }),
            terminal @ SourceContinuation::Terminal(_) => terminal,
        })
    }

    fn install_recursor_invocation<'b>(
        &mut self,
        continuation: SourceContinuation<'b>,
        activation: ContinuationActivationId,
        invocation: RecursorInvocationSegment,
        checked_ih_invocation: Option<CheckedRecursiveInvocationInstance>,
    ) -> Result<SourceContinuation<'b>, CraneliftBackendError> {
        if !recursor_invocation_is_checked(&invocation) {
            validate_recursor_invocation_install_shape(&invocation)?;
        }
        #[cfg(test)]
        px8j_record_source_event(Px8jSourceTraceEvent::Install {
            origin: invocation.origin,
            selection_cursor: invocation.resume_cursor,
            sibling_position: invocation.sibling_position,
            exits: invocation
                .unwind
                .later_wrappers_in_construction_order
                .iter()
                .filter_map(|layer| match layer.role {
                    RecursorLayerRole::ExitsScope {
                        scope_origin,
                        parent_scope,
                        ..
                    } => Some((scope_origin, parent_scope)),
                    RecursorLayerRole::SelectsOccurrence { .. } => None,
                })
                .collect(),
        });
        let sibling_position = invocation.sibling_position;
        let dynamic_splice_edges = self.take_dynamic_splice_edges(&invocation)?;
        let installed = compose_oriented_subcontinuation(
            self.oriented_subcontinuation_plan.as_ref(),
            checked_ih_invocation.or_else(|| self.active_recursive_invocations.last().copied()),
            activation,
            invocation,
            dynamic_splice_edges,
        )?;
        debug_assert_eq!(installed.activation, activation);
        debug_assert!(installed
            .control_ledger
            .iter()
            .all(|entry| match entry.role {
                RecursorLayerRole::SelectsOccurrence { origin }
                | RecursorLayerRole::ExitsScope { origin, .. } => {
                    origin == installed.producer_origin
                }
            }));
        debug_assert_eq!(installed.sibling_position, sibling_position);
        debug_assert!(installed.control_ledger.len() >= installed.semantic_frames.len());
        debug_assert!(installed.control_ledger.iter().all(|entry| {
            entry.frame_id.is_some() == entry.checked_witness.is_some()
                && (entry.frame_id.is_none()
                    || matches!(
                        entry.role,
                        RecursorLayerRole::SelectsOccurrence { .. }
                            | RecursorLayerRole::ExitsScope { .. }
                    ))
        }));
        if !installed.checked {
            let mut frames = installed.semantic_frames.into_iter();
            let selection = frames
                .next()
                .expect("validated recursor invocation has a selection frame");
            let stack = RecursorUnwindStack {
                later_wrappers_in_construction_order: frames.rev().collect(),
            };
            let continuation = Self::replace_source_terminal_with_unwind(
                continuation,
                stack,
                installed.resume_cursor,
            )?;
            return Ok(SourceContinuation::ApplyRecursorSelection {
                layer: selection,
                next: Box::new(continuation),
            });
        }
        let mut continuation = continuation;
        for layer in installed.semantic_frames.into_iter().rev() {
            continuation = SourceContinuation::ApplyRecursorSelection {
                layer,
                next: Box::new(continuation),
            };
        }
        Ok(continuation)
    }

    fn split_source_prefix<'b>(
        source: SourceContinuation<'b>,
    ) -> Result<(SourcePrefixTemplate, SourcePrefixTerminal<'b>), CraneliftBackendError> {
        Ok(match source {
            SourceContinuation::CheckedRecursiveInvocationReturn { instance, next } => {
                let (next, terminal) = Self::split_source_prefix(*next)?;
                (
                    SourcePrefixTemplate::CheckedRecursiveInvocationReturn {
                        instance,
                        next: Box::new(next),
                    },
                    terminal,
                )
            }
            SourceContinuation::CheckedComputationalIHInvocationReturn {
                call_template_id,
                next,
            } => {
                let (next, terminal) = Self::split_source_prefix(*next)?;
                (
                    SourcePrefixTemplate::CheckedComputationalIHInvocationReturn {
                        call_template_id,
                        next: Box::new(next),
                    },
                    terminal,
                )
            }
            SourceContinuation::ReturnFromSelectedCase { delimiter, next } => {
                let (next, terminal) = Self::split_source_prefix(*next)?;
                (
                    SourcePrefixTemplate::ReturnFromSelectedCase {
                        delimiter,
                        next: Box::new(next),
                    },
                    terminal,
                )
            }
            SourceContinuation::Terminal(SourceContinuationTerminal::ReturnValue) => {
                return Err(unsupported(
                    "NativeJoinPlanV1",
                    "source prefix has no exact outer terminal to split",
                ));
            }
            SourceContinuation::Terminal(SourceContinuationTerminal::ResumeOuter {
                expected,
                root_authority,
                ..
            }) => (
                SourcePrefixTemplate::Terminal {
                    expected_outer: expected,
                },
                SourcePrefixTerminal::ResumeOuter { root_authority },
            ),
            SourceContinuation::Terminal(SourceContinuationTerminal::ReturnToProducerHole {
                expected,
                root_authority,
                ..
            }) => (
                SourcePrefixTemplate::Terminal {
                    expected_outer: expected,
                },
                SourcePrefixTerminal::ResumeOuter { root_authority },
            ),
            SourceContinuation::Terminal(SourceContinuationTerminal::JumpToJoin(edge)) => (
                SourcePrefixTemplate::Terminal {
                    expected_outer: edge.target.expected_outer,
                },
                SourcePrefixTerminal::Join(edge),
            ),
            SourceContinuation::LetBody { body, env, next } => {
                let (next, terminal) = Self::split_source_prefix(*next)?;
                (
                    SourcePrefixTemplate::LetBody {
                        body,
                        env,
                        next: Box::new(next),
                    },
                    terminal,
                )
            }
            SourceContinuation::ApplyRecursorSelection { layer, next } => {
                let (next, terminal) = Self::split_source_prefix(*next)?;
                (
                    SourcePrefixTemplate::ApplyRecursorSelection {
                        layer,
                        next: Box::new(next),
                    },
                    terminal,
                )
            }
            SourceContinuation::UnwindRecursorSegment {
                stack,
                resume_cursor,
                next,
            } => {
                let (next, terminal) = Self::split_source_prefix(*next)?;
                (
                    SourcePrefixTemplate::UnwindRecursorSegment {
                        stack,
                        resume_cursor,
                        next: Box::new(next),
                    },
                    terminal,
                )
            }
            SourceContinuation::IfScrutinee {
                then_expr,
                else_expr,
                env,
                next,
            } => {
                let (next, terminal) = Self::split_source_prefix(*next)?;
                (
                    SourcePrefixTemplate::IfScrutinee {
                        then_expr,
                        else_expr,
                        env,
                        next: Box::new(next),
                    },
                    terminal,
                )
            }
            SourceContinuation::ConstructArgument {
                constructor,
                static_origin,
                remaining,
                lowered,
                env,
                next,
            } => {
                let (next, terminal) = Self::split_source_prefix(*next)?;
                (
                    SourcePrefixTemplate::ConstructArgument {
                        constructor,
                        static_origin,
                        remaining,
                        lowered,
                        env,
                        next: Box::new(next),
                    },
                    terminal,
                )
            }
            SourceContinuation::MatchScrutinee {
                cases,
                default,
                env,
                static_origin,
                next,
            } => {
                let (next, terminal) = Self::split_source_prefix(*next)?;
                (
                    SourcePrefixTemplate::MatchScrutinee {
                        cases,
                        default,
                        env,
                        static_origin,
                        next: Box::new(next),
                    },
                    terminal,
                )
            }
            SourceContinuation::ComputationalMatchScrutinee {
                cases,
                default,
                env,
                static_origin,
                provenance,
                checked_frame_id,
                answer_route,
                next,
            } => {
                let (next, terminal) = Self::split_source_prefix(*next)?;
                (
                    SourcePrefixTemplate::ComputationalMatchScrutinee {
                        cases,
                        default,
                        env,
                        static_origin,
                        provenance,
                        checked_frame_id,
                        answer_route,
                        next: Box::new(next),
                    },
                    terminal,
                )
            }
            SourceContinuation::ProjectRecord { field, next } => {
                let (next, terminal) = Self::split_source_prefix(*next)?;
                (
                    SourcePrefixTemplate::ProjectRecord {
                        field,
                        next: Box::new(next),
                    },
                    terminal,
                )
            }
            SourceContinuation::CallCallee { args, env, next } => {
                let (next, terminal) = Self::split_source_prefix(*next)?;
                (
                    SourcePrefixTemplate::CallCallee {
                        args,
                        env,
                        next: Box::new(next),
                    },
                    terminal,
                )
            }
            SourceContinuation::CallArgument {
                callee,
                remaining,
                lowered,
                env,
                next,
            } => {
                let (next, terminal) = Self::split_source_prefix(*next)?;
                (
                    SourcePrefixTemplate::CallArgument {
                        callee,
                        remaining,
                        lowered,
                        env,
                        next: Box::new(next),
                    },
                    terminal,
                )
            }
        })
    }

    fn instantiate_source_prefix_template<'b>(
        template: &SourcePrefixTemplate,
        edge: SourcePredecessorEdge<'b>,
    ) -> Result<SourceContinuation<'b>, CraneliftBackendError> {
        Ok(match template {
            SourcePrefixTemplate::Terminal { expected_outer } => {
                if *expected_outer != edge.target.expected_outer {
                    return Err(unsupported(
                        "NativeJoinPlanV1",
                        "source prefix terminal does not match the planned outer cursor",
                    ));
                }
                SourceContinuation::Terminal(SourceContinuationTerminal::JumpToJoin(edge))
            }
            SourcePrefixTemplate::CheckedRecursiveInvocationReturn { instance, next } => {
                SourceContinuation::CheckedRecursiveInvocationReturn {
                    instance: *instance,
                    next: Box::new(Self::instantiate_source_prefix_template(next, edge)?),
                }
            }
            SourcePrefixTemplate::CheckedComputationalIHInvocationReturn {
                call_template_id,
                next,
            } => SourceContinuation::CheckedComputationalIHInvocationReturn {
                call_template_id: *call_template_id,
                next: Box::new(Self::instantiate_source_prefix_template(next, edge)?),
            },
            SourcePrefixTemplate::ReturnFromSelectedCase { delimiter, next } => {
                SourceContinuation::ReturnFromSelectedCase {
                    delimiter: *delimiter,
                    next: Box::new(Self::instantiate_source_prefix_template(next, edge)?),
                }
            }
            SourcePrefixTemplate::LetBody { body, env, next } => SourceContinuation::LetBody {
                body: body.clone(),
                env: env.clone(),
                next: Box::new(Self::instantiate_source_prefix_template(next, edge)?),
            },
            SourcePrefixTemplate::ApplyRecursorSelection { layer, next } => {
                SourceContinuation::ApplyRecursorSelection {
                    layer: layer.clone(),
                    next: Box::new(Self::instantiate_source_prefix_template(next, edge)?),
                }
            }
            SourcePrefixTemplate::UnwindRecursorSegment {
                stack,
                resume_cursor,
                next,
            } => SourceContinuation::UnwindRecursorSegment {
                stack: stack.clone(),
                resume_cursor: *resume_cursor,
                next: Box::new(Self::instantiate_source_prefix_template(next, edge)?),
            },
            SourcePrefixTemplate::IfScrutinee {
                then_expr,
                else_expr,
                env,
                next,
            } => SourceContinuation::IfScrutinee {
                then_expr: then_expr.clone(),
                else_expr: else_expr.clone(),
                env: env.clone(),
                next: Box::new(Self::instantiate_source_prefix_template(next, edge)?),
            },
            SourcePrefixTemplate::ConstructArgument {
                constructor,
                static_origin,
                remaining,
                lowered,
                env,
                next,
            } => SourceContinuation::ConstructArgument {
                constructor: constructor.clone(),
                static_origin: *static_origin,
                remaining: remaining.clone(),
                lowered: lowered.clone(),
                env: env.clone(),
                next: Box::new(Self::instantiate_source_prefix_template(next, edge)?),
            },
            SourcePrefixTemplate::MatchScrutinee {
                cases,
                default,
                env,
                static_origin,
                next,
            } => SourceContinuation::MatchScrutinee {
                cases: cases.clone(),
                default: default.clone(),
                env: env.clone(),
                // D4: the template clone carries the origin with the cases. A
                // clone that copied the terms and dropped this field would
                // silently reintroduce the vacancy this unit closes.
                static_origin: *static_origin,
                next: Box::new(Self::instantiate_source_prefix_template(next, edge)?),
            },
            SourcePrefixTemplate::ComputationalMatchScrutinee {
                cases,
                default,
                env,
                static_origin,
                provenance,
                checked_frame_id,
                answer_route,
                next,
            } => SourceContinuation::ComputationalMatchScrutinee {
                cases: cases.clone(),
                default: default.clone(),
                env: env.clone(),
                static_origin: *static_origin,
                provenance: *provenance,
                checked_frame_id: *checked_frame_id,
                answer_route: *answer_route,
                next: Box::new(Self::instantiate_source_prefix_template(next, edge)?),
            },
            SourcePrefixTemplate::ProjectRecord { field, next } => {
                SourceContinuation::ProjectRecord {
                    field: field.clone(),
                    next: Box::new(Self::instantiate_source_prefix_template(next, edge)?),
                }
            }
            SourcePrefixTemplate::CallCallee { args, env, next } => {
                SourceContinuation::CallCallee {
                    args: args.clone(),
                    env: env.clone(),
                    next: Box::new(Self::instantiate_source_prefix_template(next, edge)?),
                }
            }
            SourcePrefixTemplate::CallArgument {
                callee,
                remaining,
                lowered,
                env,
                next,
            } => SourceContinuation::CallArgument {
                callee: callee.clone(),
                remaining: remaining.clone(),
                lowered: lowered.clone(),
                env: env.clone(),
                next: Box::new(Self::instantiate_source_prefix_template(next, edge)?),
            },
        })
    }

    fn mint_source_predecessor<'b>(
        &mut self,
        target: SourceJoinTarget<'b>,
    ) -> SourcePredecessorEdge<'b> {
        let predecessor_identity = self.next_source_predecessor;
        self.next_source_predecessor = self
            .next_source_predecessor
            .checked_add(1)
            .expect("compiler-private source predecessor identity exhausted");
        SourcePredecessorEdge {
            target,
            predecessor_identity,
        }
    }

    fn planned_active_scalar_cut<'b>(
        &mut self,
        active: ActiveContinuationFrame<'b>,
    ) -> Result<
        (
            Vec<EliminatorFrame<'b>>,
            &'b [EliminatorFrame<'b>],
            ScalarMergeKind,
            u64,
        ),
        CraneliftBackendError,
    > {
        for (index, frame) in active.pending.iter().copied().enumerate() {
            if let Some(site) = self.planned_join_site_for_frame(frame)? {
                let prefix_end = if matches!(frame, EliminatorFrame::InvocationReturn) {
                    index
                } else {
                    index + 1
                };
                return Ok((
                    active.pending[..prefix_end].to_vec(),
                    &active.pending[prefix_end..],
                    Self::scalar_kind_from_plan(site.answer_kind),
                    site.site_id,
                ));
            }
        }
        if active.pending.is_empty() {
            if let Some(site) =
                self.planned_join_site_for_frame(EliminatorFrame::InvocationReturn)?
            {
                return Ok((
                    Vec::new(),
                    active.pending,
                    Self::scalar_kind_from_plan(site.answer_kind),
                    site.site_id,
                ));
            }
        }
        Err(unsupported(
            "NativeJoinPlanV1",
            "active checked continuation has no planned scalar cut before its outer suffix",
        ))
    }

    fn finish_source_constructor(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        constructor: RuntimeSymbol,
        static_origin: StaticOriginId,
        lowered_args: Vec<Lowered>,
    ) -> Result<Lowered, CraneliftBackendError> {
        if lowered_args
            .iter()
            .any(|arg| matches!(arg, Lowered::RecursiveBackedge))
        {
            return Ok(Lowered::RecursiveBackedge);
        }
        if lowered_args.is_empty()
            && (constructor == self.process_symbols.bool_true
                || constructor == self.process_symbols.bool_false)
        {
            let known = constructor == self.process_symbols.bool_true;
            return Ok(Lowered::Bool {
                value: builder.ins().iconst(types::I64, i64::from(known)),
                known: Some(known),
            });
        }
        if constructor == self.process_symbols.nat_zero && lowered_args.is_empty() {
            return Ok(Lowered::StructuralNat(StructuralNatV1 {
                value: builder.ins().iconst(types::I64, 0),
            }));
        }
        if constructor == self.process_symbols.nat_suc {
            if let [Lowered::StructuralNat(predecessor)] = lowered_args.as_slice() {
                return Ok(Lowered::StructuralNat(StructuralNatV1 {
                    value: builder.ins().iadd_imm(predecessor.value, 1),
                }));
            }
        }
        Ok(Lowered::Constructor {
            constructor,
            synthesized_identity: Some(
                self.static_transition_plan
                    .constructor_symbol_identity(static_origin)?,
            ),
            // `D7` -- the allocation lane is the second fact resolved
            // at the producer and carried with the template.
            occurrence: Some(self.static_transition_plan.source_aggregate_occurrence(
                static_origin,
                PlannedAggregateShape::Constructor,
            )?),
            args: lowered_args
                .into_iter()
                .map(ConstructorField::specialized)
                .collect(),
        })
    }

}

#[cfg(test)]
mod tests {
    //! `RT-SOURCE-MACHINE-TYPES-SPLIT` `D2` -- the companion test move. These
    //! ten tests' primary discriminated property is the source-machine
    //! dispatch/install/carried-control this file owns (D0 ledger AC-2,
    //! `docs/program/issues/RT-SOURCE-MACHINE-TYPES-SPLIT.md`); moved here
    //! verbatim from `core/tests/control.rs`, item-11's `boundary.rs`
    //! precedent. Their shared fixtures (`host_result_closure_match` and
    //! siblings) stay at the `core/tests` hub, widened to
    //! `pub(in crate::cranelift_backend::lowering)` so both test subtrees
    //! reach them -- the multi-leaf-fixture LCA the frame's `D2` section
    //! anticipated moved up from `core/tests/mod.rs` to `lowering` itself
    //! once a sibling test subtree needed the same fixture.

    use super::*;
    use crate::cranelift_backend::UnsupportedLowering;
    use crate::cranelift_backend::lowering::core::tests::{
        host_result_closure_match, inert_test_static_origin, px8j_layered_recursive_result,
    };
    use crate::cranelift_backend::lowering::core::tests::control::{
        oriented_dynamic_sibling_fixture, px8j_aggregate_result, px8j_capture_source_trace,
        px8j_recursive_sibling_result, px8j_scope_chain_observation_result,
        root_authority_test_lowering, Px8dsEdgeMutation,
    };
    use crate::cranelift_backend::lowering::core::tests::source_frame_bridge::{
        d8f_compile, d8n_compile,
    };

    #[derive(Clone, Copy)]
    enum Px8jInstallMalformation {
        SelectionRole,
        UnwindRole,
        UnwindOrigin,
        RepeatedScopeIdentity,
    }


    fn run_px8j_source_machine_install(
        malformation: Option<Px8jInstallMalformation>,
    ) -> Result<SourceContinuation<'static>, CraneliftBackendError> {
        let seed_env = NativeSeedEnvironment::empty();
        let mut compiler = root_authority_test_lowering(&seed_env);
        compiler.native_join_plan = None;
        compiler.root_terminal_authority = None;
        compiler.process_object = false;

        let origin = RecursorProducerOriginId(17);
        let layer = |role| ComputationalRecursorLayer {
            cases: Vec::new(),
            default: RuntimeTrap {
                code: RuntimeTrapCode::ExplicitTrap,
                message: "PX8-J-ERR source install".to_string(),
            },
            outer_env: Vec::new(),
            static_origin: inert_test_static_origin(),
            provenance: RecursorFrameProvenance(18),
            role,
            checked_frame_id: None,
            checked_invocation_id: None,
            checked_invocation_source: None,
            checked_invocation_depth: 0,
            semantic_pending: matches!(role, RecursorLayerRole::SelectsOccurrence { .. }),
        };
        let selection = match malformation {
            Some(Px8jInstallMalformation::SelectionRole) => layer(RecursorLayerRole::ExitsScope {
                origin,
                scope_origin: RecursorProducerOriginId(18),
                parent_scope: None,
            }),
            _ => layer(RecursorLayerRole::SelectsOccurrence { origin }),
        };
        let unwind = match malformation {
            None => Vec::new(),
            Some(Px8jInstallMalformation::SelectionRole) => Vec::new(),
            Some(Px8jInstallMalformation::UnwindRole) => {
                vec![layer(RecursorLayerRole::SelectsOccurrence { origin })]
            }
            Some(Px8jInstallMalformation::UnwindOrigin) => {
                vec![layer(RecursorLayerRole::ExitsScope {
                    origin: RecursorProducerOriginId(99),
                    scope_origin: RecursorProducerOriginId(19),
                    parent_scope: None,
                })]
            }
            Some(Px8jInstallMalformation::RepeatedScopeIdentity) => vec![
                layer(RecursorLayerRole::ExitsScope {
                    origin,
                    scope_origin: RecursorProducerOriginId(19),
                    parent_scope: None,
                }),
                layer(RecursorLayerRole::ExitsScope {
                    origin,
                    scope_origin: RecursorProducerOriginId(19),
                    parent_scope: Some(RecursorProducerOriginId(19)),
                }),
            ],
        };
        let invocation = RecursorInvocationSegment::new(
            origin,
            0,
            selection,
            RecursorUnwindStack {
                later_wrappers_in_construction_order: unwind,
            },
            ContinuationCursorId(20),
            None,
            None,
        );
        assert!(!recursor_invocation_is_checked(&invocation));

        compiler.install_recursor_invocation(
            SourceContinuation::Terminal(SourceContinuationTerminal::ReturnValue),
            ContinuationActivationId(21),
            invocation,
            None,
        )
    }


    #[test]
    fn px8j_source_machine_install_rejects_repeated_scope_identity() {
        let error =
            match run_px8j_source_machine_install(Some(Px8jInstallMalformation::RepeatedScopeIdentity))
            {
                Ok(_) => panic!("the unchecked source-machine install must validate before CFG"),
                Err(error) => error,
            };
        assert!(matches!(
            error,
            CraneliftBackendError::Unsupported(UnsupportedLowering {
                construct: "ComputationalRecursor",
                reason,
            }) if reason == "recursor unwind repeats a selected scope identity"
        ));
    }


    #[test]
    fn px8j_source_machine_install_rejects_wrong_control_roles_and_origins() {
        for (malformation, expected_reason) in [
            (
                Px8jInstallMalformation::SelectionRole,
                "recursor selection role does not select the invocation origin",
            ),
            (
                Px8jInstallMalformation::UnwindRole,
                "recursor unwind role does not exit the invocation origin",
            ),
            (
                Px8jInstallMalformation::UnwindOrigin,
                "recursor unwind role does not exit the invocation origin",
            ),
        ] {
            let error = match run_px8j_source_machine_install(Some(malformation)) {
                Ok(_) => panic!("the unchecked source-machine install must validate before CFG"),
                Err(error) => error,
            };
            assert!(matches!(
                error,
                CraneliftBackendError::Unsupported(UnsupportedLowering {
                    construct: "ComputationalRecursor",
                    ref reason,
                }) if reason == expected_reason
            ));
        }
    }


    #[test]
    fn px8j_source_machine_install_accepts_valid_unchecked_segment() {
        let installed = run_px8j_source_machine_install(None)
            .expect("a valid unchecked source-machine invocation still installs");
        assert!(matches!(
            installed,
            SourceContinuation::ApplyRecursorSelection { .. }
        ));
    }


    fn run_px8ds_source_consumer(mutation: Px8dsEdgeMutation) -> Result<(), CraneliftBackendError> {
        let seed_env = NativeSeedEnvironment::empty();
        let mut compiler = root_authority_test_lowering(&seed_env);
        compiler.native_join_plan = None;
        compiler.root_terminal_authority = None;
        compiler.process_object = false;
        let (plan, mut segment, mut edges) = oriented_dynamic_sibling_fixture();
        compiler.oriented_subcontinuation_plan = Some(plan);

        match mutation {
            Px8dsEdgeMutation::Delete => {
                edges.remove(0);
            }
            Px8dsEdgeMutation::Duplicate => {
                segment
                    .dynamic_splice_edges
                    .push(segment.dynamic_splice_edges[0]);
            }
            Px8dsEdgeMutation::StaleParent => {
                edges[0].parent_invocation_instance_id = 99;
            }
            Px8dsEdgeMutation::CrossSibling => {
                let stolen = RecursorInvocationSegment {
                    dynamic_splice_edges: vec![segment.dynamic_splice_edges[0]],
                    ..segment.clone()
                };
                for edge in edges.drain(..) {
                    compiler.dynamic_splice_edges.insert(edge.edge_id, edge);
                }
                compiler.take_dynamic_splice_edges(&stolen)?;
            }
            Px8dsEdgeMutation::WrongStaticParent => {
                edges[0].parent_frame_template_id = 1;
            }
        }
        for edge in edges {
            compiler.dynamic_splice_edges.insert(edge.edge_id, edge);
        }
        compiler
            .install_recursor_invocation(
                SourceContinuation::Terminal(SourceContinuationTerminal::ReturnValue),
                ContinuationActivationId(90),
                segment,
                None,
            )
            .map(|_| ())
    }


    #[test]
    fn oriented_edge_mutations_reject_in_the_source_machine_consumer() {
        for (mutation, expected) in [
            (Px8dsEdgeMutation::Delete, "deleted, replayed"),
            (Px8dsEdgeMutation::Duplicate, "handle is duplicated"),
            (Px8dsEdgeMutation::StaleParent, "stale parent invocation"),
            (Px8dsEdgeMutation::CrossSibling, "consumed by a sibling"),
            (
                Px8dsEdgeMutation::WrongStaticParent,
                "disagrees with its checked static parent",
            ),
        ] {
            let error = match run_px8ds_source_consumer(mutation) {
                Ok(()) => panic!("source {mutation:?} must reject before CFG"),
                Err(error) => error,
            };
            assert!(
                matches!(
                    error,
                    CraneliftBackendError::Unsupported(UnsupportedLowering {
                        construct: "OrientedSubcontinuationPlanV1",
                        ref reason,
                    }) if reason.contains(expected)
                ),
                "source {mutation:?}: expected {expected:?}, got {error:?}"
            );
        }
    }


    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D8f` — the declined call is emitted and does
    /// not answer for the checked application's causal identity.**
    ///
    /// ## The three cases, and why a Boolean was the defect
    ///
    /// The source-machine call edge had one bit: consumed, or not. That spelled two
    /// different facts the same way. *"No checked application is pending"* and
    /// *"one is pending, at another occurrence"* both left the call unchanged, and
    /// both let it claim its composed causal identity -- so on the two-call witness
    /// the ordinary selected-argument call answered for the identity the planner
    /// issued for the checked application, and the checked application then answered
    /// for it again. The affine law refused, correctly: *"one causal identity was
    /// discharged twice in a single function"*.
    ///
    /// The boundary now matches a closed
    /// [`CheckedApplicationDisposition`](crate::cranelift_backend::lowering::CheckedApplicationDisposition)
    /// exhaustively. Nothing else moved: no identity gained an occurrence, none was
    /// minted, the binding is still composed, and the Function-local affine ledger
    /// is untouched. The declined call simply does not answer.
    ///
    /// ## Clause 1 — the live occurrence decision, per body
    ///
    /// The two calls are the same worker at the same arity in the same frame, so
    /// route, arity, binder index, call order and target shape are all blind. Per
    /// defining body, exactly one occurrence is `ConsumedHere` and exactly one is
    /// `PendingAtAnotherOccurrence`, and the consumed one is the occurrence the seam
    /// independently recorded as its binding. Those two logs are written at two
    /// different sites -- the seam and the integration boundary -- so their agreement
    /// is a relation, not a restatement.
    ///
    /// ## Clause 2 — no identity is discharged twice
    ///
    /// The composed causal identities actually discharged are read from closeout,
    /// which is a third site and knows nothing about dispositions.
    ///
    /// ⚠ The claim is **identity-global nonduplication**, not one claim per defining
    /// body. MEASURED: this witness's two defining bodies share ONE planner-issued
    /// causal identity, so a per-body count is the wrong shape here and would fail
    /// on a lawful program. The affine fact is that no identity appears twice in the
    /// discharged set -- which is exactly what the declined call answering would
    /// break.
    ///
    /// ## Clause 3 — the difference
    ///
    /// Letting the declined call claim -- the pre-`D8f` behaviour, with the call
    /// itself unchanged -- must bring the duplicate-discharge refusal back. Without
    /// this, "it compiles now" and "the affine law stopped noticing" are
    /// indistinguishable.
    ///
    /// ## Clause 4 — the `D8j` population is not collapsed
    ///
    /// A lawful composed call with no marker anywhere still claims. An over-broad
    /// repair that only claimed on `ConsumedHere` would silence the larger
    /// population, and this clause is what reds if one is written.
    ///
    /// **Promise class: durable invariant.**
    #[test]
    fn d8f_the_declined_call_does_not_answer_for_the_checked_identity() {
        use crate::cranelift_backend::lowering::source::set_d8f_declined_call_claims;
        use crate::cranelift_backend::lowering::{
            d8f_dispositions, d8j_discharged, d8p_application_bindings, reset_d8j_discharged,
            reset_d8n_observations, CheckedApplicationDisposition,
        };

        // Clause 1 — the live occurrence decision, per defining body.
        //
        // ⛔ The discharged-identity log is cleared HERE, immediately before the
        // program it is read for. `reset_d8n_observations` does not clear it, so
        // without this every claim below could belong to a preceding clause or a
        // preceding test, and the assertion would be attributable to nothing.
        reset_d8n_observations();
        reset_d8j_discharged();
        let outcome = d8f_compile(true);
        assert!(
            outcome.is_none(),
            "the two-call occupancy witness must compile. A 'discharged twice' refusal means the \
             declined call is answering again; any other refusal is a new finding on this route and \
             must be reported rather than absorbed: {outcome:?}"
        );
        let mut by_body: BTreeMap<FuncId, BTreeMap<StaticOriginId, CheckedApplicationDisposition>> =
            BTreeMap::new();
        for (function, origin, disposition) in d8f_dispositions() {
            let function = function.expect("every disposition names its defining Function");
            let previous = by_body.entry(function).or_default().insert(origin, disposition);
            assert!(
                previous.is_none(),
                "one call edge per (defining body, occurrence): a second disposition under one key \
                 would mean the same call reached the boundary twice"
            );
        }
        assert!(
            by_body.len() >= 2,
            "this witness's checked source body is lowered by more than one defining body, and the \
             disposition is a per-body fact: {by_body:?}"
        );
        // The independent side: which occurrence the SEAM recorded as its binding.
        // Written at the seam, not at the boundary these dispositions come from.
        let bound = d8p_application_bindings()
            .into_iter()
            .map(|binding| {
                (
                    binding.function.expect("bindings name their Function"),
                    binding.application_origin,
                )
            })
            .collect::<BTreeMap<_, _>>();
        for (function, dispositions) in &by_body {
            let consumed = dispositions
                .iter()
                .filter(|(_, disposition)| {
                    **disposition == CheckedApplicationDisposition::ConsumedHere
                })
                .map(|(origin, _)| *origin)
                .collect::<Vec<_>>();
            let declined = dispositions
                .iter()
                .filter(|(_, disposition)| {
                    **disposition == CheckedApplicationDisposition::PendingAtAnotherOccurrence
                })
                .map(|(origin, _)| *origin)
                .collect::<Vec<_>>();
            assert_eq!(
                consumed.len(),
                1,
                "exactly one occurrence in {function:?} is the checked application: {dispositions:?}"
            );
            assert_eq!(
                declined.len(),
                1,
                "and exactly one declines. Zero would mean the ordinary call never reached the seat \
                 with the marker pending, and the occupancy question is not posed at all: \
                 {dispositions:?}"
            );
            assert_ne!(
                consumed[0], declined[0],
                "and they are different occurrences, which is the only thing that tells the two calls \
                 apart: same worker, same arity, same frame"
            );
            assert_eq!(
                bound.get(function).copied(),
                Some(consumed[0]),
                "the occurrence the boundary treated as ConsumedHere must be the one the SEAM \
                 recorded binding at. Those two logs are written at different sites, so a \
                 disagreement here means the boundary is acting on a decision the seam did not make"
            );
        }

        // Clause 2 — no identity is discharged twice, read from closeout. NOT one
        // claim per defining body: the two bodies share one planner-issued identity.
        let claims = d8j_discharged();
        assert!(
            !claims.is_empty(),
            "the checked application must still discharge its composed causal identity. Zero is what \
             an over-broad repair produces -- one that stopped the declined call answering by \
             stopping every call answering: {claims:?}"
        );
        let distinct = claims.iter().cloned().collect::<BTreeSet<_>>();
        assert_eq!(
            distinct.len(),
            claims.len(),
            "and no identity may be discharged twice. MEASURED: this witness's two defining bodies \
             share ONE planner-issued causal identity, so 'once per body' would be the wrong \
             expectation here -- the affine fact is that the identity is answered for exactly once, \
             which is what the declined call answering would break: {claims:?}"
        );

        // Clause 3 — the difference. Let the declined call answer again.
        set_d8f_declined_call_claims(true);
        let doubled = d8f_compile(true);
        set_d8f_declined_call_claims(false);
        let doubled = format!("{doubled:?}");
        assert!(
            doubled.contains("one causal identity was discharged twice in a single function"),
            "letting the DECLINED call claim must bring the refusal back. The call itself is unchanged \
             either way -- only the claim moves -- so this is D8f's whole change stated as a \
             difference. Without it, 'it compiles now' and 'the affine law stopped noticing' are the \
             same observation: {doubled}"
        );

        // Clause 4 — the D8j population is not collapsed.
        //
        // ⛔ Cleared again immediately before THIS program, for the same reason: the
        // claims asserted below must be the no-marker program's own, not clause 1's
        // still sitting in the log.
        reset_d8n_observations();
        reset_d8j_discharged();
        let plain = d8n_compile();
        assert!(plain.is_none(), "the unmarked composed witness compiles: {plain:?}");
        assert!(
            !d8j_discharged().is_empty(),
            "a lawful composed call with no marker anywhere must still claim its causal identity. \
             This is the LARGER population, and a repair that only claimed on ConsumedHere would \
             silence it: {:?}",
            d8j_discharged()
        );
        assert!(
            d8f_dispositions()
                .iter()
                .all(|(_, _, disposition)| *disposition
                    == CheckedApplicationDisposition::NoPendingApplication),
            "and every disposition on that program is NoPendingApplication, so clause 4 is about the \
             first case and not about a program that quietly has a marker somewhere: {:?}",
            d8f_dispositions()
        );
    }


    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D6b` — the ORDINARY-UNIT COPY carries a word
    /// at the recursive position, so using it as a specialized callee fails closed
    /// there. A LOCAL control, and nothing wider.**
    ///
    /// ⛔ **READ THIS FIRST, because the row's name is narrower than it sounds.**
    /// *"The selected recursive argument is actually called"* is **DELIVERED, not
    /// blocked.** Through ordinary production planning and lowering the argument
    /// **is** called at the **source-machine** seat with its exact raw arguments and
    /// captures, and its result is consumed in the same continuation with no
    /// closure-valued unit result. That is the accepted **`D8d`/`D8e`/`D8j`**
    /// evidence — see
    /// [`d8g_the_composed_selected_argument_reaches_its_target_at_the_shared_emitter`],
    /// which asserts the composed selected recursive argument is emitted from **two**
    /// bodies. Architect `evt_6grnfx2psztcn`.
    ///
    /// ## What this row actually measures
    ///
    /// One narrower negative, on **one copy** of one case body. It is retained as a
    /// local fail-closed control and **nothing more**.
    ///
    /// ## What is armed
    ///
    /// One switch on the existing mixed fixture moves the checked application's
    /// callee from `Var(0)` to `Var(2)`. Both are members of the same case
    /// environment and the change is one index: `Var(0)` is the induction hypothesis
    /// and `Var(2)` is the selected recursive argument, per the binder run
    /// `[IH, ordinary field 0, SelectedRecursiveArgument{1}, ..]`.
    ///
    /// ## The measured behaviour
    ///
    /// That case body is lowered **twice**: once into each specialization body, where
    /// the environment binds both members as static workers, and once by the source
    /// machine into the **ordinary-unit copy**, where this position carries only a
    /// **word**. Using a word as a specialized callee must fail closed, and it does —
    /// `Unsupported(BoundaryCarrier, ..)`, before any specialization body is reached.
    ///
    /// ⛔ **Three things this row does NOT say**, each of which it was briefly
    /// written to say:
    ///
    /// 1. It does **not** generalize over the composed source-machine population.
    ///    The composed path calls the member lawfully; this is the ordinary-unit
    ///    copy alone.
    /// 2. It does **not** prescribe a future representation change. Failing closed
    ///    on a word in a specialized callee position is the boundary working.
    /// 3. It is **not** evidence about `raw_worker_calls`. It fails before any table
    ///    is consulted; that separate fact is measured by
    ///    [`d6b_the_mixed_pair_is_over_one_body_and_only_a_retarget_makes_the_two_tables_disagree`].
    ///
    /// ## The positive control
    ///
    /// The identical fixture with the switch disarmed compiles. Without it, a
    /// refusal for any unrelated reason would read as this one.
    ///
    /// **Promise class: durable invariant.** The claim is that a specialized-only
    /// surface refuses a carried word — a typed phase boundary, not a message. It is
    /// matched on the error's construct category with the callee edge named, never
    /// on formatted text alone.
    #[test]
    fn d6b_calling_the_selected_recursive_argument_in_the_ordinary_unit_copy_fails_closed_at_the_carrier() {
        use crate::cranelift_backend::surface::{CraneliftBackendError, UnsupportedLowering};

        // The positive control, first: the same fixture with nothing armed.
        crate::cranelift_backend::test_objects::set_px8tr_call_selected_recursive_argument(false);
        crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
            "ken_d6b_control",
            false,
        )
        .map(|_| ())
        .map_err(|error| format!("{error:?}"))
        .expect("POSITIVE CONTROL: the unarmed fixture compiles, so the refusal below is attributable \
                 to the one index that moved");

        crate::cranelift_backend::test_objects::set_px8tr_call_selected_recursive_argument(true);
        let outcome = crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
            "ken_d6b_selected_argument_called",
            false,
        )
        .map(|_| ());
        crate::cranelift_backend::test_objects::set_px8tr_call_selected_recursive_argument(false);

        match outcome {
            Err(CraneliftBackendError::Unsupported(UnsupportedLowering { construct, reason })) => {
                assert_eq!(
                    construct, "BoundaryCarrier",
                    "the refusal is the typed PHASE boundary, not a route, table or arity failure. A \
                     different category would mean the call reached somewhere this row does not \
                     describe: {reason}"
                );
                assert!(
                    reason.contains("a source-machine call's callee"),
                    "and it is the SOURCE MACHINE's callee edge that refused -- the ordinary unit \
                     body's copy of this case, not a specialization body. That is what places the \
                     obstacle upstream of the raw table: {reason}"
                );
            }
            other => panic!(
                "the ORDINARY-UNIT COPY carries a word at this position, so using it as a specialized \
                 callee must fail closed there. ⛔ This is the local control only -- the composed \
                 source-machine path calls this member lawfully (`D8d`/`D8e`/`D8j`), and a green here \
                 is not a claim about that path; got {other:?}"
            ),
        }
    }


    /// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2a` — each compile that reaches the
    /// backedge seat forwards its marker, while earlier projection-owned refusals
    /// remain outside that claim.**
    ///
    /// > **MEASURED:** counters reset inside each of the five compiles show that
    /// > only a strict subset reaches `ComputationalMatchScrutinee`. Each arriving
    /// > case forwards once with the route enabled and forwards zero times when it
    /// > is suppressed. The complement has zero arrivals in both legs and changes
    /// > at the `RT-REQUIRED-OCCURRENCE-PROJECTION` route before this seat.
    /// > **CLAIMED:** every marker that actually reaches this owner is forwarded;
    /// > nothing is claimed about a compile that refuses earlier. **THE GAP:** the
    /// > arriving rows still refuse later, and the genuine non-marker
    /// > non-constructor control remains D3's work.
    ///
    /// The arrival count is a per-case structural qualifier, not a pooled
    /// denominator. Every forward and rendering clause is inside that qualified
    /// branch and reads a `NonZeroUsize` constructed for that exact compile. The
    /// all-case aggregate is accumulated before qualification, while each
    /// forwarding and rendering obligation remains case-local.
    ///
    /// PROMISE CLASS: durable invariant. The test asserts relations over the
    /// observed arriving set and proves the complement's ownership by suppressing
    /// one real projection route; it does not pin the population's cardinality.
    #[test]
    fn lrc_d2a_forwards_each_arrival_and_excludes_projection_owned_early_refusals() {
        use crate::cranelift_backend::lowering::core::with_required_consumer_route_suppressed;
        use crate::cranelift_backend::lowering::source::{
            lrc_d2a_backedge_arrivals, lrc_d2a_backedge_forwards, reset_lrc_d2a_counts,
            set_lrc_d2a_suppress_forward,
        };
        struct Restore;
        impl Drop for Restore {
            fn drop(&mut self) {
                set_lrc_d2a_suppress_forward(false);
            }
        }
        const R1: &str = "source scrutinee is not a constructor value";

        // The five `R1` compiles: row 1's exact and deleted-scope pair, and row 4's
        // three depths. Named by shape, not transcribed from a census.
        let cases: Vec<(&str, RuntimeExpr, bool)> = vec![
            (
                "row1 owned-scope exact",
                host_result_closure_match(px8j_layered_recursive_result(1, 1)),
                false,
            ),
            (
                "row1 owned-scope deleted",
                host_result_closure_match(px8j_layered_recursive_result(1, 1)),
                true,
            ),
            (
                "row4 depth 1",
                host_result_closure_match(px8j_scope_chain_observation_result(1, 0)),
                false,
            ),
            (
                "row4 depth 2",
                host_result_closure_match(px8j_scope_chain_observation_result(2, 0)),
                false,
            ),
            (
                "row4 depth 3",
                host_result_closure_match(px8j_scope_chain_observation_result(3, 0)),
                false,
            ),
        ];

        let run = |suppress: bool| -> Vec<(&'static str, usize, usize, String)> {
            let _restore = Restore;
            let mut observed = Vec::new();
            for (label, expression, delete_scope) in &cases {
                reset_lrc_d2a_counts();
                set_lrc_d2a_suppress_forward(suppress);
                let (result, _trace) =
                    px8j_capture_source_trace(expression, *delete_scope, "ken_lrc_d2a");
                set_lrc_d2a_suppress_forward(false);
                observed.push((
                    *label,
                    lrc_d2a_backedge_arrivals(),
                    lrc_d2a_backedge_forwards(),
                    format!("{result:?}"),
                ));
            }
            observed
        };

        let enabled = run(false);
        let suppressed = run(true);

        assert_eq!(
            enabled.len(),
            suppressed.len(),
            "the two D2a legs must compile the same case population",
        );

        let mut qualified_cases = 0usize;
        let mut complement_cases = 0usize;
        let mut aggregate_arrivals = 0usize;
        let mut aggregate_forwards = 0usize;

        for (
            (label, arrivals, forwards, rendering),
            (suppressed_label, suppressed_arrivals, suppressed_forwards, suppressed_rendering),
        ) in enabled.iter().zip(&suppressed)
        {
            assert_eq!(
                label, suppressed_label,
                "the two D2a legs must preserve case identity",
            );
            aggregate_arrivals = aggregate_arrivals
                .checked_add(*arrivals)
                .expect("the all-case arrival aggregate overflowed");
            aggregate_forwards = aggregate_forwards
                .checked_add(*forwards)
                .expect("the all-case forward aggregate overflowed");

            if let Some(established_arrivals) = std::num::NonZeroUsize::new(*arrivals) {
                qualified_cases += 1;
                let established_suppressed_arrivals =
                    std::num::NonZeroUsize::new(*suppressed_arrivals).unwrap_or_else(|| {
                        panic!(
                            "the enabled leg reached D2a on {label}, but its suppressed twin did not"
                        )
                    });
                assert_eq!(
                    established_suppressed_arrivals, established_arrivals,
                    "suppressing only the forward changed the arrival population on {label}",
                );
                assert_eq!(
                    *forwards,
                    established_arrivals.get(),
                    "an arriving marker was not forwarded on {label}",
                );
                let unforwarded = established_suppressed_arrivals
                    .get()
                    .checked_sub(*suppressed_forwards)
                    .expect("the suppressed leg forwarded more markers than arrived");
                assert_eq!(
                    unforwarded,
                    established_suppressed_arrivals.get(),
                    "the suppressed leg still forwarded a marker on {label}",
                );
                assert!(
                    !rendering.contains(R1),
                    "R1 survives the enabled route on arriving case {label}: {rendering}",
                );
                assert!(
                    suppressed_rendering.contains(R1),
                    "suppressing the forward did not recreate R1 on arriving case {label}: \
                     {suppressed_rendering}",
                );
            } else {
                complement_cases += 1;
                assert!(
                    rendering.starts_with("Err("),
                    "a case outside D2a's arrival set must refuse before that seat: {label}: \
                     {rendering}",
                );
                assert_eq!(
                    (*forwards, *suppressed_arrivals, *suppressed_forwards),
                    (0, 0, 0),
                    "a non-arriving case has inconsistent D2a counters on {label}",
                );
                assert_eq!(
                    rendering, suppressed_rendering,
                    "the D2a forward switch changed a case that never reached its seat: {label}",
                );

                let (_, expression, delete_scope) = cases
                    .iter()
                    .find(|(case_label, _, _)| case_label == label)
                    .expect("every observation must retain its source case");
                let compile_without_projection = || {
                    let (result, _trace) = px8j_capture_source_trace(
                        expression,
                        *delete_scope,
                        "ken_lrc_d2a_without_required_projection",
                    );
                    format!("{result:?}")
                };
                let (without_projection, applications) =
                    with_required_consumer_route_suppressed(compile_without_projection);
                assert_eq!(
                    applications, 1,
                    "the non-arriving case must cross one real required-consumer projection route: \
                     {label}",
                );
                assert_ne!(
                    without_projection, *rendering,
                    "suppressing the required-consumer projection did not change the early refusal \
                     on {label}",
                );
            }
        }

        let established_qualified_cases = std::num::NonZeroUsize::new(qualified_cases)
            .expect("no compile reached D2a, so its forwarding claim ranges over nothing");
        assert!(
            aggregate_arrivals >= established_qualified_cases.get(),
            "each qualified case must contribute at least one arrival",
        );
        assert_eq!(
            aggregate_forwards, aggregate_arrivals,
            "the all-case forward aggregate disagrees with the all-case arrival aggregate",
        );
        assert!(
            complement_cases > 0,
            "the projection-owned complement disappeared; re-evaluate whether D2a now reaches every \
             case rather than silently broadening this branch",
        );
    }


    /// The observation one run yields. Grouped so the two legs are compared on the
    /// same axes rather than on whichever field each happened to read.
    #[cfg(test)]
    struct D2bObservation {
        rendered: String,
        closeout: Vec<(
            BTreeSet<StaticOriginId>,
            BTreeSet<StaticOriginId>,
            BTreeSet<StaticOriginId>,
        )>,
        entered: BTreeSet<StaticOriginId>,
        worker_calls: BTreeSet<StaticOriginId>,
    }

    /// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2b` — an abandoned `Let` body's joins are
    /// DISPOSITIONED, and the missing-join refusal is gone.**
    ///
    /// > **MEASURED:** under B-only exclusion row 3 no longer renders *"function
    /// > left planned source join … neither emitted nor statically unselected"*.
    /// > Suppressing **only** the new arm's disposition brings that exact refusal
    /// > back. **CLAIMED:** the abandoned body's joins are accounted at the arm that
    /// > abandons it. **THE GAP:** row 3 still does not compile — it advances to the
    /// > singular-specialization hard stop, which this deliverable keeps.
    ///
    /// ⛔ **The A/B is the whole control, because the repaired side is an ABSENCE.**
    /// Asserting only that the sentence is gone would pass for free if the sentence
    /// were deleted from production, or if the row failed earlier for an unrelated
    /// reason. The suppressed leg is what makes the absence attributable: it
    /// reproduces the exact refusal from the committed tree, with nothing else
    /// changed.
    ///
    /// ⛔ **And the advance is asserted positively**, not merely as "some other
    /// error": the row must reach the hard stop, which is strictly later than the
    /// join closeout. A row that started failing *earlier* would also stop
    /// rendering the join refusal, and that would be a regression wearing the
    /// shape of a fix.
    #[test]
    fn d2b_the_abandoned_let_body_joins_are_dispositioned_at_the_arm_that_abandons_it() {
        use crate::cranelift_backend::lowering::source::{
            set_lrc_d2b_let_disposition, LrcD2bLetDisposition,
        };
        use crate::cranelift_backend::lowering::{
            lrc_d2b_entered, lrc_d2b_join_observation, lrc_d2b_reset_observation, lrc_d2b_worker_calls,
        };
        struct Restore;
        impl Drop for Restore {
            fn drop(&mut self) {
                set_lrc_d2b_let_disposition(LrcD2bLetDisposition::Exact);
            }
        }
        const MISSING_JOIN: &str = "neither emitted nor statically unselected";
        const HARD_STOP: &str = "projects no worker for";

        let run = |mode: LrcD2bLetDisposition| -> D2bObservation {
            let _restore = Restore;
            let expression = host_result_closure_match(px8j_recursive_sibling_result(
                1,
                2,
                px8j_aggregate_result(),
            ));
            lrc_d2b_reset_observation();
            set_lrc_d2b_let_disposition(mode);
            let (result, _trace) = px8j_capture_source_trace(&expression, false, "ken_d2b_let");
            set_lrc_d2b_let_disposition(LrcD2bLetDisposition::Exact);
            D2bObservation {
                rendered: format!("{result:?}"),
                closeout: lrc_d2b_join_observation(),
                entered: lrc_d2b_entered(),
                worker_calls: lrc_d2b_worker_calls(),
            }
        };

        // ── REPAIRED ────────────────────────────────────────────────────────────
        let repaired = run(LrcD2bLetDisposition::Exact);
        assert!(
            !repaired.rendered.contains(MISSING_JOIN),
            "the abandoned body's join is still unaccounted: {}",
            repaired.rendered
        );
        assert!(
            repaired.rendered.contains(HARD_STOP),
            "the row did not reach the singular-specialization hard stop, so it is failing EARLIER \
             than the join closeout and the absence above is a regression rather than the repair: {}",
            repaired.rendered
        );

        // ⭐⭐ THE ACCOUNTING ITSELF, not inferred from the absence of a sentence.
        //
        // ⛔ `None` here would mean the closeout guard never ran, which the absence
        // assertion above cannot distinguish from "it ran and closed". That is the
        // reading this unwrap refuses.
        assert!(
            !repaired.closeout.is_empty(),
            "no join closeout ran at all, so the accounting below would be vacuous"
        );
        // ⛔ The guard runs once per function; select the close that actually
        // dispositioned something, and require it to be UNIQUE. Taking the last
        // close would record a different function's accounting.
        let dispositioning: Vec<_> = repaired
            .closeout
            .iter()
            .filter(|(_, _, dispositioned)| !dispositioned.is_empty())
            .collect();
        assert_eq!(
            dispositioning.len(),
            1,
            "expected exactly one function to disposition anything; got {} -- the arm is firing in \
             more places than the abandoned body",
            dispositioning.len()
        );
        let (required, consumed, dispositioned) = dispositioning[0].clone();
        let abandoned = *dispositioned
            .iter()
            .next()
            .expect("the abandoned body's join is dispositioned");

        assert_eq!(
            dispositioned.len(),
            1,
            "exactly the abandoned body's join is dispositioned; a wider subtree would swallow joins \
             that legitimately executed: {dispositioned:?}"
        );
        assert!(
            required.contains(&abandoned),
            "the dispositioned join is not in the required set, so it belongs to another function"
        );
        assert!(
            !consumed.contains(&abandoned),
            "the abandoned join is BOTH consumed and dispositioned; the two sets must stay disjoint"
        );
        assert!(
            consumed.is_disjoint(&dispositioned),
            "consumed and dispositioned overlap: {consumed:?} vs {dispositioned:?}"
        );
        let mut covered = consumed.clone();
        covered.extend(dispositioned.iter().copied());
        assert_eq!(
            covered, required,
            "the disjoint union of consumed and dispositioned does not close the required set"
        );
        assert!(
            !consumed.is_empty(),
            "nothing was consumed, so the closure above holds for the degenerate reason that \
             everything was dispositioned"
        );

        // THE BODY DID NOT EXECUTE, and that is asserted rather than assumed.
        assert!(
            !repaired.entered.contains(&abandoned),
            "the abandoned body's occurrence was ENTERED, so it did execute and dispositioning it is \
             the wrong accounting"
        );
        assert!(
            !repaired.worker_calls.contains(&abandoned),
            "a static-worker call was emitted for the abandoned body's origin, so it did execute"
        );
        assert!(
            !repaired.entered.is_empty(),
            "no occurrence was entered at all, so the two absences above are vacuous"
        );

        // ── SUPPRESSED — the pre-repair state, from the committed tree ──────────
        let suppressed = run(LrcD2bLetDisposition::Suppress);
        assert!(
            suppressed.rendered.contains(MISSING_JOIN),
            "suppressing ONLY the new arm's disposition did not recreate the missing-join refusal, \
             so that arm is not what accounts the join: {}",
            suppressed.rendered
        );
        assert!(
            suppressed
                .closeout
                .iter()
                .all(|(_, _, dispositioned)| !dispositioned.contains(&abandoned)),
            "the suppressed run still dispositioned the abandoned join, so the suppression did not \
             reach the arm and the A/B compares one state with itself"
        );
    }


    /// Row 3's exact shape with **only** the `Let` value changed to an ordinary
    /// constructor.
    ///
    /// ⛔ Derived from `px8j_recursive_sibling_result` by rewriting one node, not
    /// re-typed beside it. "Differs only in the `Let` value" is then true **by
    /// construction**: a copy would drift from row 3 the first time either is
    /// edited, and this control's whole meaning is that it is the same shape.
    ///
    /// The producer, its two recursive positions, the `Let` and the `Call Var(2)`
    /// body are row 3's own.
    fn px8j_sibling_result_with_ordinary_let_value() -> RuntimeExpr {
        let mut expression =
            px8j_recursive_sibling_result(1, 2, px8j_aggregate_result());
        let RuntimeExpr::ComputationalMatch { cases, .. } = &mut expression else {
            panic!("row 3's fixture is a computational match");
        };
        let node_case = cases
            .iter_mut()
            .find(|case| !case.recursive_positions.is_empty())
            .expect("row 3's producer has a recursive case");
        let RuntimeExpr::Let { value, .. } = &mut node_case.body else {
            panic!("row 3's recursive case body is a Let");
        };
        // The recursive `Call Var(0)` is what lowers to a backedge; an ordinary
        // constructor is what this row exists to contrast with.
        assert!(
            matches!(value.as_ref(), RuntimeExpr::Call { .. }),
            "row 3's Let value is the recursive call this fixture replaces"
        );
        *value = Box::new(RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        });
        expression
    }


    /// **`D2b` ROW A — CAPABILITY GATE ONLY. This row does NOT show a body running.**
    ///
    /// ⛔ Its whole subject is that on row 3's **two-position** shape the retained
    /// segment-1 refusal fires **before the case body**, so the body and its
    /// static-worker edge are **unreachable in both disposition modes**. It is
    /// named for that and must not be read as a live-body row: it does not show the
    /// body entering, its join consumed, or any global body-entry property. Row B
    /// below is the live one.
    ///
    /// > **MEASURED:** on row 3's two-position producer with only the `Let` value
    /// > changed to an ordinary constructor, **no static-worker edge is reached in
    /// > either mode**, `dispositioned` is empty in every close, the disjoint union
    /// > closes `required`, and both modes agree exactly. **CLAIMED:** the retained
    /// > singular refusal precedes the case body on this shape, and the new arm
    /// > dispositions nothing on a non-backedge value. **THE GAP:** this says
    /// > nothing about a body that *does* run — that is Row B's axis, not this
    /// > one's.
    ///
    /// ⛔ **An earlier revision of this row claimed to be the live half.** It was
    /// not: on this shape the body never lowers, so it could not testify to a body
    /// running. Splitting the two axes is what makes each row's claim true.
    ///
    /// ⛔ **The mode toggle must change NOTHING here**, and that is asserted rather
    /// than assumed. If `Suppress` altered this row, the mutation would not be
    /// confined to the backedge branch — and the backedge control's A/B would then
    /// be measuring two changes instead of one.
    #[test]
    fn d2b_capability_gate_the_two_position_shape_refuses_before_its_case_body() {
        use crate::cranelift_backend::lowering::source::{
            set_lrc_d2b_let_disposition, LrcD2bLetDisposition,
        };
        use crate::cranelift_backend::lowering::{
            lrc_d2b_entered, lrc_d2b_join_observation, lrc_d2b_reset_observation, lrc_d2b_worker_calls,
        };
        struct Restore;
        impl Drop for Restore {
            fn drop(&mut self) {
                set_lrc_d2b_let_disposition(LrcD2bLetDisposition::Exact);
            }
        }

        let run = |mode: LrcD2bLetDisposition| -> D2bObservation {
            let _restore = Restore;
            let expression = host_result_closure_match(px8j_sibling_result_with_ordinary_let_value());
            lrc_d2b_reset_observation();
            set_lrc_d2b_let_disposition(mode);
            let (result, _trace) =
                px8j_capture_source_trace(&expression, false, "ken_d2b_live_let");
            set_lrc_d2b_let_disposition(LrcD2bLetDisposition::Exact);
            D2bObservation {
                rendered: format!("{result:?}"),
                closeout: lrc_d2b_join_observation(),
                entered: lrc_d2b_entered(),
                worker_calls: lrc_d2b_worker_calls(),
            }
        };

        for mode in [LrcD2bLetDisposition::Exact, LrcD2bLetDisposition::Suppress] {
            let observed = run(mode);

            // ⛔ SOME source occurrence was entered -- NOT the body's, and this
            // comment says so because an earlier revision of it said "THE BODY
            // RAN", which is false on this shape and contradicted the very next
            // block.
            //
            // This is a NON-VACUITY clause and nothing more: without it, every
            // "nothing was dispositioned" below would hold because the compile did
            // nothing at all. It establishes that lowering happened, not that the
            // case body was reached -- Row B is the row that reaches a body, and it
            // asserts entry of the ARM-REPORTED origin rather than a non-empty set.
            assert!(
                !observed.entered.is_empty(),
                "{mode:?}: no source occurrence was entered, so this row measures nothing"
            );
            // ⛔ THE BODY'S `Call` IS NOT REACHED ON THIS FIXTURE, and that is a
            // measured property of the shape rather than a gap in this row.
            //
            // The re-cut asked this control to prove the body's `Call` reaches a
            // static-worker edge and its join is consumed. It cannot, and the
            // reason is structural: the producer keeps row 3's TWO recursive
            // positions, so `continuation_case_binder_run`'s segment-1 hard stop --
            // which this deliverable RETAINS -- fires during unit definition,
            // before the case body lowers. Changing only the `Let` value cannot get
            // past a refusal triggered by the producer's shape.
            //
            // ⇒ A fixture that DID reach the body would need a single-recursive-
            // position producer, which is no longer row 3's shape and would stop
            // being the comparison this row exists to make. The two requirements
            // are in tension on one fixture; the handback reports it.
            assert!(
                observed.worker_calls.is_empty(),
                "{mode:?}: a static-worker edge WAS reached, so the singular hard stop no longer \
                 precedes the body and the live half of this row is now constructible -- write it \
                 rather than keeping this assertion"
            );

            assert!(
                !observed.closeout.is_empty(),
                "{mode:?}: no join closeout ran, so the accounting below is vacuous"
            );

            // NOTHING IS DISPOSITIONED — the discriminator against the backedge row.
            for (required, consumed, dispositioned) in &observed.closeout {
                assert!(
                    dispositioned.is_empty(),
                    "{mode:?}: a live Let body's subtree was dispositioned {dispositioned:?}; the new \
                     arm is firing outside the backedge branch"
                );
                assert!(
                    consumed.is_disjoint(dispositioned),
                    "{mode:?}: consumed and dispositioned overlap"
                );
                let mut covered = consumed.clone();
                covered.extend(dispositioned.iter().copied());
                assert_eq!(
                    covered, *required,
                    "{mode:?}: the disjoint union does not close the required set"
                );
            }
            assert!(
                observed
                    .closeout
                    .iter()
                    .any(|(_, consumed, _)| !consumed.is_empty()),
                "{mode:?}: nothing was consumed anywhere, so the closures above are degenerate"
            );
        }

        // THE MODE TOGGLE CHANGES NOTHING on this shape.
        let exact = run(LrcD2bLetDisposition::Exact);
        let suppressed = run(LrcD2bLetDisposition::Suppress);
        assert_eq!(
            exact.rendered, suppressed.rendered,
            "the disposition mode changed a NON-backedge row's outcome, so the mutation is not \
             confined to the backedge branch and the other control's A/B measures two changes"
        );
        assert_eq!(
            exact.closeout, suppressed.closeout,
            "the disposition mode changed a non-backedge row's join accounting"
        );
        assert_eq!(
            exact.entered, suppressed.entered,
            "the disposition mode changed which occurrences a non-backedge row entered"
        );
    }


    /// Row 3's **single**-position shape with its recursive `Call` wrapped in a
    /// `Let` whose value is ordinary.
    ///
    /// ⛔ Derived by rewriting one node, like Row A's fixture, so the only
    /// difference from `px8j_recursive_sibling_result(1, 1, …)` is the wrapping.
    ///
    /// ⛔ **The de Bruijn shift is the load-bearing part.** Introducing the `Let`
    /// binder pushes every outer binding down one, so the original `Var(0)` callee
    /// becomes `Var(1)` in the body. Leaving it at `Var(0)` would silently call the
    /// `Let`'s own value — an ordinary `Unit` — and the row would fail for a reason
    /// having nothing to do with the arm under test.
    fn px8j_single_position_let_wrapped_recursive_call() -> RuntimeExpr {
        let mut expression = px8j_recursive_sibling_result(1, 1, px8j_aggregate_result());
        let RuntimeExpr::ComputationalMatch { cases, .. } = &mut expression else {
            panic!("the fixture is a computational match");
        };
        let node_case = cases
            .iter_mut()
            .find(|case| !case.recursive_positions.is_empty())
            .expect("the producer has a recursive case");
        assert_eq!(
            node_case.recursive_positions.len(),
            1,
            "Row B's axis is the SINGLE-position shape; two positions is Row A's"
        );
        let RuntimeExpr::Call { callee, args } = node_case.body.clone() else {
            panic!("the single-position case body is the recursive Call this row wraps");
        };
        assert!(
            matches!(callee.as_ref(), RuntimeExpr::Var(0)),
            "the recursive callee is Var(0) before the Let binder is introduced"
        );
        node_case.body = RuntimeExpr::Let {
            value: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:prelude::Unit::MkUnit".to_string(),
                args: Vec::new(),
            }),
            // The de Bruijn shift: Var(0) -> Var(1) under the new binder.
            body: Box::new(RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::Var(1)),
                args,
            }),
        };
        expression
    }


    /// **`D2b` ROW B — the REACHING non-backedge branch: the body runs, its join is
    /// consumed, and nothing is dispositioned.**
    ///
    /// > **MEASURED, and every clause is about THE ARM'S OWN body origin:** the arm
    /// > is reached with a **non-backedge** value; that body occurrence is
    /// > **entered**; a static-worker edge is reached **for that origin**; the
    /// > closeout whose `required` contains it is **unique**, and in that owning
    /// > closeout the origin is **consumed** and **not dispositioned**; every close
    /// > has empty `dispositioned` and a union that closes; and Exact and Suppress
    /// > agree on **all** of outcome, arrival, entry, calls and accounting. **CLAIMED:** the new arm's ordinary branch is untouched — a
    /// > live `Let` body runs and its joins are consumed, not dispositioned.
    /// > **THE GAP:** one source shape, and the single-position producer, which is
    /// > what makes the body reachable at all.
    ///
    /// ⛔ **The body occurrence is the ARM'S OWN `body.static_origin`**, reported by
    /// the arm-local observation, never a numeric origin. So each clause is a
    /// relation between what the arm saw and what the compile did, rather than a
    /// coincidence of two independently written constants.
    ///
    /// ⛔ **An earlier revision proved the arm's body was entered and then only that
    /// SOME worker call and SOME consumed join existed.** Those are existentials
    /// over unrelated occurrences — they hold even if the arm's body is entered and
    /// never lowered, while a different occurrence supplies the call and the
    /// consumption. Selecting the **owning** closeout by `required` membership, and
    /// requiring uniqueness, is what ties the accounting to *this* body instead of
    /// to whichever function happened to close last.
    ///
    /// ⛔ **Row A cannot make this claim** — on its two-position shape the retained
    /// refusal precedes the body entirely. That is why the pair is split.
    #[test]
    fn d2b_row_b_a_live_nonbackedge_let_runs_its_body_and_consumes_its_join() {
        use crate::cranelift_backend::lowering::source::{
            set_lrc_d2b_let_disposition, LrcD2bLetDisposition,
        };
        use crate::cranelift_backend::lowering::{
            lrc_d2b_entered, lrc_d2b_join_observation, lrc_d2b_let_arrivals, lrc_d2b_reset_observation,
            lrc_d2b_worker_calls,
        };
        struct Restore;
        impl Drop for Restore {
            fn drop(&mut self) {
                set_lrc_d2b_let_disposition(LrcD2bLetDisposition::Exact);
            }
        }

        let run = |mode: LrcD2bLetDisposition| {
            let _restore = Restore;
            let expression =
                host_result_closure_match(px8j_single_position_let_wrapped_recursive_call());
            lrc_d2b_reset_observation();
            set_lrc_d2b_let_disposition(mode);
            let (result, _trace) = px8j_capture_source_trace(&expression, false, "ken_d2b_row_b");
            set_lrc_d2b_let_disposition(LrcD2bLetDisposition::Exact);
            (
                format!("{result:?}"),
                lrc_d2b_let_arrivals(),
                lrc_d2b_entered(),
                lrc_d2b_worker_calls(),
                lrc_d2b_join_observation(),
            )
        };

        for mode in [LrcD2bLetDisposition::Exact, LrcD2bLetDisposition::Suppress] {
            let (_rendered, arrivals, entered, worker_calls, closeout) = run(mode);

            // 1. The arm was reached, with a NON-backedge value.
            let nonbackedge: Vec<_> = arrivals
                .iter()
                .filter(|(_, backedge)| !backedge)
                .map(|(origin, _)| *origin)
                .collect();
            assert!(
                !nonbackedge.is_empty(),
                "{mode:?}: the LetBody arm was never reached with a non-backedge value, so this row \
                 measures nothing"
            );
            assert!(
                arrivals.iter().all(|(_, backedge)| !backedge),
                "{mode:?}: a backedge arrival occurred, so this is not purely the ordinary branch"
            );

            // 2/3. ⭐⭐ EVERY ASSERTION IS ABOUT THE ARM'S OWN BODY ORIGIN.
            //
            // ⛔ An earlier revision asserted `entered.contains(body)` and then only
            // that SOME worker call and SOME consumed join existed anywhere in the
            // compile. Those are existentials over unrelated occurrences: they hold
            // just as well if the arm's body is entered and then never lowered,
            // while some other occurrence supplies the call and the consumption.
            // The relation below is per-body and closes that gap.
            assert!(!closeout.is_empty(), "{mode:?}: no join closeout ran");
            for body in &nonbackedge {
                assert!(
                    entered.contains(body),
                    "{mode:?}: the arm's own body occurrence {body:?} was never entered, so the live \
                     body did not run"
                );
                // THIS body reached a static-worker edge -- not merely some body.
                assert!(
                    worker_calls.contains(body),
                    "{mode:?}: no static-worker edge was reached for the arm's own body {body:?}; \
                     calls were emitted for {worker_calls:?}, which is a different occurrence"
                );
                // THIS body is consumed in ITS OWNING closeout. Selecting the close
                // by `required` membership is what ties the accounting to this
                // occurrence rather than to whichever function closed last.
                let owning: Vec<_> = closeout
                    .iter()
                    .filter(|(required, _, _)| required.contains(body))
                    .collect();
                assert_eq!(
                    owning.len(),
                    1,
                    "{mode:?}: the arm's body {body:?} is required by {} closeouts; its owning \
                     function must be unique or the assertions below name no particular accounting",
                    owning.len()
                );
                let (required, consumed, dispositioned) = owning[0];
                assert!(
                    consumed.contains(body),
                    "{mode:?}: the arm's own body {body:?} is not CONSUMED in its owning closeout, so \
                     it did not execute there"
                );
                assert!(
                    !dispositioned.contains(body),
                    "{mode:?}: the arm's own body {body:?} was DISPOSITIONED in its owning closeout, \
                     which is the abandoned-body accounting applied to a live body"
                );
                assert!(
                    consumed.is_disjoint(dispositioned),
                    "{mode:?}: consumed and dispositioned overlap in the owning closeout"
                );
                let mut covered = consumed.clone();
                covered.extend(dispositioned.iter().copied());
                assert_eq!(
                    covered, *required,
                    "{mode:?}: the owning closeout's disjoint union does not close its required set"
                );
            }

            // The whole-compile invariants, retained: no close dispositions
            // anything on this shape, and every close's union closes.
            for (required, consumed, dispositioned) in &closeout {
                assert!(
                    dispositioned.is_empty(),
                    "{mode:?}: a LIVE Let body's subtree was dispositioned {dispositioned:?}"
                );
                let mut covered = consumed.clone();
                covered.extend(dispositioned.iter().copied());
                assert_eq!(
                    covered, *required,
                    "{mode:?}: the disjoint union does not close the required set"
                );
            }
            assert!(
                closeout.iter().any(|(_, consumed, _)| !consumed.is_empty()),
                "{mode:?}: nothing was consumed, so the closures above are degenerate"
            );
        }

        // 4. THE MODE TOGGLE CHANGES NOTHING on the ordinary branch — every axis.
        let exact = run(LrcD2bLetDisposition::Exact);
        let suppressed = run(LrcD2bLetDisposition::Suppress);
        assert_eq!(exact.0, suppressed.0, "the mode changed a live row's outcome");
        assert_eq!(exact.1, suppressed.1, "the mode changed a live row's LetBody arrivals");
        assert_eq!(exact.2, suppressed.2, "the mode changed a live row's entered occurrences");
        assert_eq!(exact.3, suppressed.3, "the mode changed a live row's static-worker calls");
        assert_eq!(exact.4, suppressed.4, "the mode changed a live row's join accounting");
    }


}
