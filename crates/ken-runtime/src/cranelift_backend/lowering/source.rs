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
//! `pub(super)` on `SourceContinuation`, `SourceContinuationTerminal`,
//! `install_recursor_invocation` and `lower_source_machine` restores exactly
//! their pre-move reachability (mod.rs-private items were already visible
//! to every `lowering` descendant; core.rs-private items are called from
//! retained core.rs code and, until `D2` relocates them, from the ten
//! not-yet-moved tests in `core/tests/control.rs`) — not a widening beyond
//! what already existed.

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
        lowered: Vec<Lowered>,
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
                        body,
                        ..
                    } => {
                        // `D8f` — the machine's own child derivation, taken
                        // BEFORE the marker is entered so the same occurrence is
                        // recorded and evaluated.
                        let body = self.owned_child_occurrence(static_origin, 0, *body)?;
                        self.enter_checked_computational_ih_invocation(
                            call_template_id,
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
                                let disposition = self
                                    .consume_checked_ih_marker_at_static_worker_call(
                                        binder_index,
                                        0,
                                        static_origin,
                                    )?;
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
                            // ⭐ A source `Construct` builds a constructor
                            // **template**, so its arguments take the ruled
                            // fail-closed boundary here.
                            lowered.push(
                                value.specialized_at("a source constructor argument")?,
                            );
                            control.continuation = *next;
                            if remaining.is_empty() {
                                SourceMachineState::Value {
                                    value: RoutedAnswer::direct(LoweringOperand::Specialized(self.finish_source_constructor(
                                        builder,
                                        constructor,
                                        static_origin,
                                        lowered,
                                    )?)),
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

    pub(super) fn install_recursor_invocation<'b>(
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
