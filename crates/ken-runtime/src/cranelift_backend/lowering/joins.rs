//! The control and joins emitter -- branch/match emission, join emission
//! (consumption, disposition, validation, scalar-merge completion), and
//! block/terminator construction (trap exits) on the emitter side.
//!
//! `RT-EMITTER-CONTROL-JOINS-SPLIT` `D1`. Extends the `boundary.rs`/
//! `source.rs`/`calls.rs` seam (items 11-13): the owner traced in this
//! item's D0 ledger (7 addenda, `docs/program/issues/
//! RT-EMITTER-CONTROL-JOINS-SPLIT.md`, Addendum 7 the corrected transport
//! manifest) relocates here from `core.rs` and `mod.rs`, moved verbatim --
//! each moved method sits in its own small `impl` block, matching `source.
//! rs`/`calls.rs`'s own shape. Every other type the moving methods merely
//! manipulate (`ScalarMergeKind`, `merge_planned_scalar_branch`, `lowered_
//! from_scalar_pair`, `TrapExitAuthority`, `TrapFrameBindingMutation`,
//! `Px8trTrapProvenanceEvent`, `PlannedTrapSeat`, `NativeScalarPairV1`,
//! `JoinConsumptionMutation`, `specialized_at`/`specialized_ref_at`/
//! `effect_seat_phase`, and siblings) stays declared at the `mod.rs` hub --
//! hub-stays/methods-move, the same shape items 10/12/13 established.
//! `ScalarMergeKind` in particular was corrected MOVE -> RETAIN mid-D0
//! (Addendum 7, Architect Finding 1): it is a field of the retained `source.
//! rs` type `SourceJoinTarget`, so it stays at the hub even though every one
//! of its use sites sits inside a method that moved here.
//!
//! `pub(super)` widenings, each load-bearing (named in the `D1` ledger
//! addendum, not silent): `jump_planned_join_arm` (retained core.rs callers
//! outside the moving match-dispatch cluster), `TrapIdentityMutation` and
//! `set_trap_identity_mutation` (both constructed/called directly by the
//! not-yet-moved tests in `core/tests/control.rs`, a descendant of `lowering
//! ::core`, a sibling of this module -- reachable before the move only
//! because the callee sat in the `lowering` module itself). Other movers
//! (`carried_join_arm`, `append_planned_join_params`, `finish_planned_join`)
//! already carried `pub(super)` from item 13's own D1 and need no change.
//! Any further widening this D1 required beyond what was traced here is
//! named in the handback, not silently introduced.

use super::*;

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TrapIdentityMutation {
    Exact,
    Zero,
    Substitute,
}

#[cfg(test)]
thread_local! {
    pub(super) static TRAP_IDENTITY_MUTATION: std::cell::Cell<TrapIdentityMutation> =
        const { std::cell::Cell::new(TrapIdentityMutation::Exact) };
}

#[cfg(test)]
pub(super) fn set_trap_identity_mutation(mutation: TrapIdentityMutation) {
    TRAP_IDENTITY_MUTATION.with(|cell| cell.set(mutation));
}


/// One general scalar-merge decision observed by the governed
/// `RT-DYNAMIC-ARM-SCALAR-MERGE` control.
///
/// Feature-scoped and doc-hidden: this is diagnostic machinery for the real D5
/// package path, not a supported production API. `operand_kind` is read from
/// the exhaustive [`lowered_value_kind`] classification at the merge seat;
/// `constructor` preserves exact identity when that operand is still a
/// constructor.
#[cfg(any(test, feature = "dasm-c2-observation"))]
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DasmC2ScalarMergeObservation {
    pub construct: &'static str,
    pub operand_kind: &'static str,
    pub constructor: Option<String>,
    pub admitted: bool,
}

#[cfg(any(test, feature = "dasm-c2-observation"))]
thread_local! {
    static DASM_C2_SCALAR_MERGE_OBSERVATIONS:
        std::cell::RefCell<Vec<DasmC2ScalarMergeObservation>> =
        const { std::cell::RefCell::new(Vec::new()) };
    static DASM_C2_SCALAR_MERGE_OBSERVATION_ENABLED: std::cell::Cell<bool> =
        const { std::cell::Cell::new(false) };
}

#[cfg(any(test, feature = "dasm-c2-observation"))]
fn dasm_c2_record_scalar_merge(observation: DasmC2ScalarMergeObservation) {
    if DASM_C2_SCALAR_MERGE_OBSERVATION_ENABLED.get() {
        DASM_C2_SCALAR_MERGE_OBSERVATIONS.with(|cell| cell.borrow_mut().push(observation));
    }
}

#[cfg(any(test, feature = "dasm-c2-observation"))]
fn dasm_c2_take_scalar_merge_observations() -> Vec<DasmC2ScalarMergeObservation> {
    DASM_C2_SCALAR_MERGE_OBSERVATIONS
        .with(|cell| std::mem::take(&mut *cell.borrow_mut()))
}

/// Feature-scoped handle for the D5 scalar-merge observation.
#[cfg(feature = "dasm-c2-observation")]
#[doc(hidden)]
pub struct DasmC2ScalarMergeObservationScope {
    previous_enabled: bool,
    previous_observations: Option<Box<Vec<DasmC2ScalarMergeObservation>>>,
}

#[cfg(feature = "dasm-c2-observation")]
impl DasmC2ScalarMergeObservationScope {
    fn restore(&mut self) {
        let Some(previous_observations) = self.previous_observations.take() else {
            return;
        };
        DASM_C2_SCALAR_MERGE_OBSERVATION_ENABLED.set(self.previous_enabled);
        DASM_C2_SCALAR_MERGE_OBSERVATIONS.with(|cell| {
            *cell.borrow_mut() = *previous_observations;
        });
    }

    /// Close this scope and return its recorded merge decisions.
    pub fn finish(mut self) -> Vec<DasmC2ScalarMergeObservation> {
        let observations = dasm_c2_take_scalar_merge_observations();
        self.restore();
        observations
    }
}

#[cfg(feature = "dasm-c2-observation")]
impl Drop for DasmC2ScalarMergeObservationScope {
    fn drop(&mut self) {
        self.restore();
    }
}

/// Observe general scalar-merge decisions on this thread until the returned
/// scope is finished or dropped.
#[cfg(feature = "dasm-c2-observation")]
#[doc(hidden)]
pub fn dasm_c2_scalar_merge_observation_scope() -> DasmC2ScalarMergeObservationScope {
    let previous_observations = dasm_c2_take_scalar_merge_observations();
    let previous_enabled = DASM_C2_SCALAR_MERGE_OBSERVATION_ENABLED.replace(true);
    DasmC2ScalarMergeObservationScope {
        previous_enabled,
        previous_observations: Some(Box::new(previous_observations)),
    }
}


impl<'a> Lowering<'a> {
        /// ⭐ **The dual of [`LoweringOperand::specialized_join_arm`]** — a join
        /// whose single lane is the **carrier word**, not a native scalar pair.
        ///
        /// ⚠⚠ **Read the two together: they refuse in OPPOSITE directions, and the
        /// asymmetry is the point.** `specialized_join_arm` guards a join that has
        /// no carried lane, so its `Carried` arm fails closed. This guards a join
        /// that has *only* a carried lane, so a **specialized** arm must cross into
        /// it. ⛔ Neither is a `Carried -> Lowered` conversion; this one moves
        /// `Lowered -> CarriedBoundaryWord`, which is precisely the direction `§2g`
        /// rules as the producer's one-way seam.
        ///
        /// ⭐ **Why a carried match's join has one lane and it is this one.** An arm
        /// of a carried `Match` may return a **projected child**, which `§2g`
        /// requires to stay `Carried` and which has no compile-time template to
        /// re-specialize. So the merge cannot be a `Lowered` join, and every arm
        /// must arrive as a carrier word.
        ///
        /// ⚠ **The producer's coverage is partial and this inherits that
        /// deliberately.** An arm whose value is a form `transfer_into_carrier`
        /// defers — a spillable `Int`, a `String`, borrowed ingress — fails closed
        /// with **the producer's own message**, ⛔ never a second refusal invented
        /// here. ⇒ The carried match's arm coverage widens exactly when the
        /// producer's does, with no list to keep in sync and no second authority to
        /// let drift.
        pub(super) fn carried_join_arm(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            origin: StaticOriginId,
            lowered: LoweringOperand,
            required_kind: Option<ScalarMergeKind>,
            join: &'static str,
        ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
            match lowered {
                LoweringOperand::Carried(word) => {
                    #[cfg(test)]
                    D8_CARRIED_JOIN_UNCHANGED.with(|count| count.set(count.get() + 1));
                    Ok(word)
                }
                // ── ⛔ DEFERRED, said plainly ──────────────────────────────────
                //
                // ⚠ A deferral is honest; a deferral that reads as delivery is not.
                // A compile-time trap arm must **not** reach the merge at all — it
                // returns instead (`seal_source_trap_branch`) — so the merge block
                // would have fewer predecessors than the case chain has arms. That
                // is a control-flow shape this route does not build yet, and
                // refusing is strictly better than emitting a half-formed merge.
                LoweringOperand::Specialized(Lowered::Trap(trap)) => Err(unsupported(
                    "BoundaryCarrier",
                    format!(
                        "{join} resolves at compile time to a trap ({}), and the carried join \
                         does not yet build a merge with a trapping predecessor",
                        trap.message
                    ),
                )),
                // **`RT-MATCH-RECURSOR-CONSUMERS` `D2` — REPRESENT THE ARM THAT HAS
                // ALREADY LEFT, HERE, BEFORE THE GUARD.**
                //
                // `Lowered::RecursiveBackedge` is not a value and never becomes one.
                // It says the tail-recursive edge was ALREADY emitted as a CFG jump
                // and this block is predecessor-free, so the arm contributes no
                // value to the merge -- only a predecessor edge that is itself
                // unreachable. The caller still jumps, so the merge keeps its
                // predecessor count and its single `I64` block parameter, and the
                // word below is never read at run time because no control ever
                // arrives here.
                //
                // This does NOT relax `emit_carrier_transfer`. That guard is
                // correct and untouched: protocol machinery is still never a source
                // value at a boundary. What was wrong was ASKING it -- the arm was
                // reaching a value transfer it should never have entered, because
                // this match keyed on the operand's REPRESENTATION (`Carried` vs
                // `Specialized`) when the property that decides whether an arm can
                // be a join predecessor is whether control already departed.
                //
                // This is the carried mirror of a landed representation, not a
                // new mechanism. The scalar lane already does exactly this: a
                // backedge arm yields an inert pair and then ABSTAINS from result
                // kind agreement (`record_scalar_merge_kind` returns early), and the
                // `JumpToJoin` scalar branch exempts it from the planned-kind check
                // before jumping. The carried lane has one word and no kind to
                // agree on, so representing the arm is the whole of its share.
                //
                // ⇒ No reduced-predecessor merge is built or needed. The `Trap` arm
                // above still refuses because a trap RETURNS instead of jumping and
                // so genuinely removes a predecessor; a backedge still jumps.
                LoweringOperand::Specialized(Lowered::RecursiveBackedge) => {
                    // A null word, deliberately: if this ever were read, a zero is a
                    // fail-fast null rather than a plausible arena address.
                    Ok(CarriedBoundaryWord {
                        word: builder.ins().iconst(types::I64, 0),
                    })
                }
                LoweringOperand::Specialized(lowered) => {
                    #[cfg(test)]
                    D8_SPECIALIZED_JOIN_PRODUCTIONS.with(|count| count.set(count.get() + 1));
                    let terminal_exit = self.process_object
                        && (required_kind == Some(ScalarMergeKind::ExitCode)
                            || self
                                .function_local
                                .terminal_result_origins
                                .contains(&origin))
                        && matches!(
                            &lowered,
                            Lowered::Constructor { constructor, .. }
                                if constructor == &self.process_symbols.exit_success
                                    || constructor == &self.process_symbols.exit_failure
                        );
                    if terminal_exit {
                        let status = self.emit_process_exit_status(builder, lowered);
                        self.emit_carrier_immediate(builder, BoundaryTag::ImmediateExitStatus, status)
                    } else {
                        self.transfer_into_carrier(builder, origin, &lowered)
                    }
                }
            }
        }
}

impl<'a> Lowering<'a> {
        /// Give one already-planned join exactly the lanes named by its D8 token.
        pub(super) fn append_planned_join_params(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            merge: cranelift_codegen::ir::Block,
            join_plan: &JoinPlanToken,
        ) {
            self.function_local
                .materialized_join_blocks
                .entry(join_plan.origin)
                .or_default()
                .insert(merge);
            builder.append_block_param(merge, types::I64);
            if join_plan.representation == JoinResultRepresentation::NativeScalarPair {
                builder.append_block_param(merge, types::I64);
            }
        }
}

impl<'a> Lowering<'a> {
        /// Send one continuing predecessor through the representation selected
        /// before CFG emission. Source traps are sealed by the caller and never
        /// reach this value-only operation.
        pub(super) fn jump_planned_join_arm(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            merge: cranelift_codegen::ir::Block,
            join_plan: &JoinPlanToken,
            origin: StaticOriginId,
            lowered: LoweringOperand,
            merge_kind: &mut Option<ScalarMergeKind>,
            join: &'static str,
        ) -> Result<(), CraneliftBackendError> {
            match join_plan.representation {
                JoinResultRepresentation::NativeScalarPair => {
                    if matches!(lowered, LoweringOperand::Carried(_)) {
                        let planned = self.retained_body_occurrence(join_plan.origin)?;
                        return Err(backend_module(format!(
                            "{join} source join {:?} ({:?}) planned native scalar lanes but \
                             lowering produced a carried boundary word",
                            join_plan.origin, planned.expr
                        )));
                    }
                    let (value, kind) = self.merge_scalar_branch(builder, join_plan, lowered, join)?;
                    Self::record_scalar_merge_kind(join, merge_kind, kind)?;
                    builder
                        .ins()
                        .jump(merge, &[value.tag.into(), value.payload.into()]);
                }
                JoinResultRepresentation::CarrierWord => {
                    let word = self.carried_join_arm(builder, origin, lowered, None, join)?;
                    builder.ins().jump(merge, &[word.word.into()]);
                }
            }
            Ok(())
        }
}

impl<'a> Lowering<'a> {
        /// Recover the typed result of a planned join after all continuing
        /// predecessors have been emitted.
        pub(super) fn finish_planned_join(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            merge: cranelift_codegen::ir::Block,
            join_plan: &JoinPlanToken,
            merge_kind: Option<ScalarMergeKind>,
            join: &'static str,
        ) -> Result<LoweringOperand, CraneliftBackendError> {
            builder.switch_to_block(merge);
            match join_plan.representation {
                JoinResultRepresentation::NativeScalarPair => {
                    let pair = NativeScalarPairV1 {
                        tag: builder.block_params(merge)[0],
                        payload: builder.block_params(merge)[1],
                    };
                    let kind = merge_kind.ok_or_else(|| {
                        backend_module(format!(
                            "{join} has a continuing native predecessor without a result kind"
                        ))
                    })?;
                    Ok(LoweringOperand::Specialized(
                        self.lowered_from_scalar_pair(kind, pair),
                    ))
                }
                JoinResultRepresentation::CarrierWord => {
                    Ok(LoweringOperand::Carried(CarriedBoundaryWord {
                        word: builder.block_params(merge)[0],
                    }))
                }
            }
        }
}

impl<'a> Lowering<'a> {
        /// ⭐⭐ **`D3` — `Match` eliminating a value that has NO compile-time
        /// template.** This is the second, *executable* route the whole node exists
        /// to build.
        ///
        /// The specialized route answers *"which constructor?"* by reading a
        /// `Lowered::Constructor`'s own `constructor` field while compiling. Here
        /// there is no such value and no such field — only a boundary word — so
        /// **every** question becomes a call into the emitted carrier ABI:
        ///
        /// | question | specialized route | this route |
        /// |---|---|---|
        /// | which Result carrier representation? | compile-time `Lowered` variant | `class(word)` |
        /// | which constructor? | `case.constructor == constructor` | `tag(word)` vs `case_constructor_identity` |
        /// | how many children? | `args.len()` | `field_count(word)` |
        /// | child *i*? | `args[i]` | `field(word, i)` — ⭐ **stays `Carried`** |
        /// | nothing matched? | a compile-time `Lowered::Trap` | a **runtime** closed default |
        ///
        /// ⭐ **Both columns read ONE identity authority** (`D2`). The producer
        /// wrote either `constructor_symbol_identity(..)` for a source occurrence
        /// or `synthesized_constructor_identity(..)` for a closed compiler role;
        /// this compares against `case_constructor_identity(..).tag_abi_word()`.
        /// Equal spellings intern to one canonical span, so the two agree **because
        /// they are the same number**, not because two derivations happen to
        /// coincide.
        /// ⛔ There is no decode step and no reverse table: the comparison is word
        /// against word, ⛔ never word against a reconstructed name.
        ///
        /// ⚠ **This changes no production behaviour today.** Nothing in production
        /// emits a `Carried` scrutinee (`AC-C10` — zero `B2F` activation), so this
        /// route is reached only by a test that seeds one. Stated here so the
        /// reachability is not overclaimed by a later reader.
        pub(super) fn lower_carried_match(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            scrutinee: CarriedBoundaryWord,
            cases: &[crate::RuntimeMatchCase],
            default: &RuntimeTrap,
            static_origin: StaticOriginId,
            env: &[LoweringEnvironmentBinding],
        ) -> Result<LoweringOperand, CraneliftBackendError> {
            let join_plan = self.consumed_join_plan_token(static_origin)?;
            if cases.is_empty() {
                return Ok(LoweringOperand::Specialized(Lowered::Trap(default.clone())));
            }
            // Only the closed process-input constructor family can arrive through
            // the borrowed-opaque lane.  Do not materialize that branch for an
            // ordinary carried match: Cranelift must compile both successors, so a
            // runtime class test alone would incorrectly require borrowed
            // identities for unrelated source constructors.
            let admits_borrowed_input = self.process_object
                && cases.iter().all(|case| {
                    borrowed_constructor_identity(&self.process_symbols, &case.constructor).is_some()
                });
            if !admits_borrowed_input {
                return self.lower_nonborrowed_carried_match(
                    builder,
                    scrutinee,
                    cases,
                    default,
                    static_origin,
                    env,
                    &join_plan,
                );
            }
            let class = self.emit_carrier_class(builder, scrutinee)?;
            let is_borrowed = builder.ins().icmp_imm(
                cranelift_codegen::ir::condcodes::IntCC::Equal,
                class,
                BoundaryClass::BorrowedOpaque as i64,
            );
            let borrowed = builder.create_block();
            let represented = builder.create_block();
            let merge = join_plan
                .has_continuing_predecessor
                .then(|| builder.create_block());
            if let Some(merge) = merge {
                self.append_planned_join_params(builder, merge, &join_plan);
            }
            let mut merge_kind = None;
            builder
                .ins()
                .brif(is_borrowed, borrowed, &[], represented, &[]);

            builder.switch_to_block(borrowed);
            let pointer = self.emit_carrier_scalar(builder, scrutinee)?;
            let borrowed_result = self.lower_borrowed_match(
                builder,
                pointer,
                cases,
                default,
                static_origin,
                env,
                &join_plan,
            )?;
            if self.seal_source_trap_branch(builder, &borrowed_result)? {
                // This runtime representation has no continuing predecessor.
            } else {
                let merge = merge.ok_or_else(|| {
                    backend_module(
                        "join plan omitted a merge despite a continuing predecessor".to_string(),
                    )
                })?;
                self.jump_planned_join_arm(
                    builder,
                    merge,
                    &join_plan,
                    static_origin,
                    borrowed_result,
                    &mut merge_kind,
                    "a carried borrowed-input match",
                )?;
            }

            builder.switch_to_block(represented);
            let represented_result = self.lower_nonborrowed_carried_match(
                builder,
                scrutinee,
                cases,
                default,
                static_origin,
                env,
                &join_plan,
            )?;
            if self.seal_source_trap_branch(builder, &represented_result)? {
                // This runtime representation has no continuing predecessor.
            } else {
                let merge = merge.ok_or_else(|| {
                    backend_module(
                        "join plan omitted a merge despite a continuing predecessor".to_string(),
                    )
                })?;
                self.jump_planned_join_arm(
                    builder,
                    merge,
                    &join_plan,
                    static_origin,
                    represented_result,
                    &mut merge_kind,
                    "a carried represented-value match",
                )?;
            }

            let Some(merge) = merge else {
                let unreachable_continuation = builder.create_block();
                builder.switch_to_block(unreachable_continuation);
                return Ok(LoweringOperand::Specialized(Lowered::Trap(default.clone())));
            };
            self.finish_planned_join(
                builder,
                merge,
                &join_plan,
                merge_kind,
                "a carried representation split",
            )
        }
}

impl<'a> Lowering<'a> {
        fn lower_nonborrowed_carried_match(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            scrutinee: CarriedBoundaryWord,
            cases: &[crate::RuntimeMatchCase],
            default: &RuntimeTrap,
            static_origin: StaticOriginId,
            env: &[LoweringEnvironmentBinding],
            join_plan: &JoinPlanToken,
        ) -> Result<LoweringOperand, CraneliftBackendError> {
            // ⭐ Handled before any block is created, and that ordering matters: a
            // case-free match reaches the default unconditionally, so building a
            // merge block for it would leave one with no predecessor.
            if cases.is_empty() {
                return Ok(LoweringOperand::Specialized(Lowered::Trap(default.clone())));
            }
            let ok_case = cases
                .iter()
                .enumerate()
                .find(|(_, case)| case.constructor == self.process_symbols.result_ok);
            let err_case = cases
                .iter()
                .enumerate()
                .find(|(_, case)| case.constructor == self.process_symbols.result_err);
            if ok_case.is_some() || err_case.is_some() {
                let (Some(ok_case), Some(err_case)) = (ok_case, err_case) else {
                    return Err(unsupported(
                        "HostResult",
                        "a carried HostResult match requires both closed Result cases",
                    ));
                };
                if ok_case.1.binders != 1 || err_case.1.binders != 1 {
                    return Err(unsupported(
                        "HostResult",
                        "carried Result cases must each bind exactly one selected payload",
                    ));
                }
                // Dispatch both carried representations into one pair of source
                // case blocks. A nested source join is therefore emitted exactly
                // once even though either representation can select its owner.
                let ok_body = builder.create_block();
                builder.append_block_param(ok_body, types::I64);
                let err_body = builder.create_block();
                builder.append_block_param(err_body, types::I64);
                let merge = join_plan
                    .has_continuing_predecessor
                    .then(|| builder.create_block());
                if let Some(merge) = merge {
                    self.append_planned_join_params(builder, merge, join_plan);
                }
                let mut merge_kind = None;

                let class = self.emit_carrier_class(builder, scrutinee)?;
                let is_host_result = builder.ins().icmp_imm(
                    cranelift_codegen::ir::condcodes::IntCC::Equal,
                    class,
                    BoundaryClass::HostResult as i64,
                );
                let host_result = builder.create_block();
                let constructor = builder.create_block();
                builder
                    .ins()
                    .brif(is_host_result, host_result, &[], constructor, &[]);

                builder.switch_to_block(host_result);
                let success = self.emit_carrier_host_success(builder, scrutinee)?;
                let payload = self.emit_carrier_host_payload(builder, scrutinee)?;
                builder.ins().brif(
                    success,
                    ok_body,
                    &[payload.word.into()],
                    err_body,
                    &[payload.word.into()],
                );

                builder.switch_to_block(constructor);
                let tag = self.emit_carrier_tag(builder, scrutinee)?;
                let field_count = self.emit_carrier_field_count(builder, scrutinee)?;
                for (body_block, (index, _case)) in [(ok_body, ok_case), (err_body, err_case)] {
                    let identity = self
                        .static_transition_plan
                        .case_constructor_identity(static_origin, index)?
                        .tag_abi_word()?;
                    let identity = Self::carrier_identity_immediate(builder, identity);
                    let selected = builder.create_block();
                    let next = builder.create_block();
                    let matched = builder.ins().icmp(
                        cranelift_codegen::ir::condcodes::IntCC::Equal,
                        tag,
                        identity,
                    );
                    builder.ins().brif(matched, selected, &[], next, &[]);
                    builder.switch_to_block(selected);
                    Self::require_i64(builder, field_count, 1);
                    let payload = self.emit_carrier_field(builder, scrutinee, 0)?;
                    builder.ins().jump(body_block, &[payload.word.into()]);
                    builder.switch_to_block(next);
                }
                let defaulted = LoweringOperand::Specialized(Lowered::Trap(default.clone()));
                if !self.seal_source_trap_branch(builder, &defaulted)? {
                    return Err(unsupported(
                        "Match",
                        "the carried Result match's closed default did not seal its branch",
                    ));
                }

                for (block, (index, case)) in [(ok_body, ok_case), (err_body, err_case)] {
                    builder.switch_to_block(block);
                    let payload = CarriedBoundaryWord {
                        word: builder.block_params(block)[0],
                    };
                    let case_env = env_with_operands([LoweringOperand::Carried(payload)], env);
                    let body = self.case_body_occurrence(static_origin, index, &case.body)?;
                    let body_origin = body.static_origin;
                    let lowered = self.lower_expr(builder, body, &case_env)?;
                    if self.seal_source_trap_branch(builder, &lowered)? {
                        continue;
                    }
                    let merge = merge.ok_or_else(|| {
                        backend_module(
                            "join plan omitted a merge despite a continuing predecessor".to_string(),
                        )
                    })?;
                    self.jump_planned_join_arm(
                        builder,
                        merge,
                        join_plan,
                        body_origin,
                        lowered,
                        &mut merge_kind,
                        "a carried Result arm",
                    )?;
                }

                let Some(merge) = merge else {
                    let unreachable_continuation = builder.create_block();
                    builder.switch_to_block(unreachable_continuation);
                    return Ok(LoweringOperand::Specialized(Lowered::Trap(default.clone())));
                };
                return self.finish_planned_join(
                    builder,
                    merge,
                    join_plan,
                    merge_kind,
                    "a carried Result join",
                );
            }

            self.lower_carried_constructor_match(
                builder,
                scrutinee,
                cases,
                default,
                static_origin,
                env,
                join_plan,
            )
        }
}

impl<'a> Lowering<'a> {
        fn lower_carried_constructor_match(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            scrutinee: CarriedBoundaryWord,
            cases: &[crate::RuntimeMatchCase],
            default: &RuntimeTrap,
            static_origin: StaticOriginId,
            env: &[LoweringEnvironmentBinding],
            join_plan: &JoinPlanToken,
        ) -> Result<LoweringOperand, CraneliftBackendError> {
            // Read identity and arity ONCE, ahead of the chain: both are properties
            // of the scrutinee, not of any case, and re-reading per case would be a
            // second answer to a question that has one.
            let tag = self.emit_carrier_tag(builder, scrutinee)?;
            let field_count = self.emit_carrier_field_count(builder, scrutinee)?;

            let merge = join_plan
                .has_continuing_predecessor
                .then(|| builder.create_block());
            if let Some(merge) = merge {
                self.append_planned_join_params(builder, merge, join_plan);
            }
            let mut merge_kind = None;

            for (index, case) in cases.iter().enumerate() {
                // ⭐ `D1` — the case's identity, keyed on this `Match` occurrence's
                // origin and the case's ordinal. ⚠ `case.constructor`, the
                // **string**, is deliberately not the key: keying on the spelling
                // would be the second derivation `D2` forbids.
                let identity = self
                    .static_transition_plan
                    .case_constructor_identity(static_origin, index)?
                    .tag_abi_word()?;
                let identity = Self::carrier_identity_immediate(builder, identity);
                let selected = builder.create_block();
                let next = builder.create_block();
                let matched = builder.ins().icmp(
                    cranelift_codegen::ir::condcodes::IntCC::Equal,
                    tag,
                    identity,
                );
                builder.ins().brif(matched, selected, &[], next, &[]);

                builder.switch_to_block(selected);
                // ⚠ **The arity check the specialized route performs while
                // compiling has to be EMITTED here**, because neither operand is
                // known until the value exists. It is a real guard, not ceremony:
                // binding *n* binders over a node with fewer children would read
                // past the node. A mismatch means the producer's `field_count` and
                // the elaborator's binder count disagree — corruption, not an input
                // condition — so it takes the same failure status as every other
                // carrier ABI violation.
                let binders = i64::try_from(case.binders).map_err(|_| {
                    unsupported(
                        "BoundaryCarrier",
                        "a case binds more binders than the carrier ABI can count",
                    )
                })?;
                Self::require_i64(builder, field_count, binders);

                // ⭐ `§2g`: *"projected children remain `Carried`."* Each binder is
                // a runtime projection, and it enters `case_env` **in the carried
                // phase** — which is the exact clause `§2h`'s control demands.
                let mut bindings = Vec::with_capacity(case.binders);
                for position in 0..case.binders {
                    bindings.push(LoweringOperand::Carried(
                        self.emit_carrier_field(builder, scrutinee, position)?,
                    ));
                }
                let case_env = env_with_operands(bindings, env);
                let body = self.case_body_occurrence(static_origin, index, &case.body)?;
                let body_origin = body.static_origin;
                let lowered = self.lower_expr(builder, body, &case_env)?;
                if self.seal_source_trap_branch(builder, &lowered)? {
                    builder.switch_to_block(next);
                    continue;
                }
                let merge = merge.ok_or_else(|| {
                    backend_module(
                        "join plan omitted a merge despite a continuing predecessor".to_string(),
                    )
                })?;
                self.jump_planned_join_arm(
                    builder,
                    merge,
                    join_plan,
                    body_origin,
                    lowered,
                    &mut merge_kind,
                    "a carried `Match` arm",
                )?;

                builder.switch_to_block(next);
            }

            // ── ⛔ THE CLOSED DEFAULT — `AC-C3`'s negative arm ─────────────────
            //
            // ⭐ Routed through the existing [`Self::seal_source_trap_branch`]
            // rather than spelling the trap encoding a second time: if the encoding
            // ever changes, both move together. A constructor outside the
            // artifact-static case set lands here, at runtime, and returns.
            let defaulted = LoweringOperand::Specialized(Lowered::Trap(default.clone()));
            if !self.seal_source_trap_branch(builder, &defaulted)? {
                return Err(unsupported(
                    "Match",
                    "the carried match's closed default did not seal its branch",
                ));
            }

            let Some(merge) = merge else {
                let unreachable_continuation = builder.create_block();
                builder.switch_to_block(unreachable_continuation);
                return Ok(LoweringOperand::Specialized(Lowered::Trap(default.clone())));
            };
            self.finish_planned_join(
                builder,
                merge,
                join_plan,
                merge_kind,
                "a carried `Match` join",
            )
        }
}

impl<'a> Lowering<'a> {
        /// `static_origin` is the origin of the **match occurrence** whose cases
        /// these are; case *i*'s body is `child(static_origin, 1 + i)`.
        pub(super) fn lower_borrowed_match(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            pointer: cranelift_codegen::ir::Value,
            cases: &[crate::RuntimeMatchCase],
            default: &RuntimeTrap,
            static_origin: StaticOriginId,
            env: &[LoweringEnvironmentBinding],
            join_plan: &JoinPlanToken,
        ) -> Result<LoweringOperand, CraneliftBackendError> {
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
            let pointer_type = builder.func.dfg.value_type(pointer);
            let fields = builder
                .ins()
                .load(pointer_type, MemFlags::trusted(), pointer, 16);
            if let [case] = cases {
                let (expected_tag, expected_arity) =
                    borrowed_constructor_identity(&self.process_symbols, &case.constructor)
                        .ok_or_else(|| {
                            unsupported(
                                "Match",
                                format!("{} has no borrowed constructor identity", case.constructor),
                            )
                        })?;
                if case.binders != expected_arity {
                    return Err(unsupported(
                        "Match",
                        format!("{} borrowed arity mismatch", case.constructor),
                    ));
                }
                let arm = builder.create_block();
                let rejected = builder.create_block();
                let selected = builder.ins().icmp_imm(
                    cranelift_codegen::ir::condcodes::IntCC::Equal,
                    tag,
                    expected_tag,
                );
                builder.ins().brif(selected, arm, &[], rejected, &[]);
                builder.switch_to_block(rejected);
                let failure = builder.ins().iconst(types::I64, -1);
                builder.ins().return_(&[failure]);
                builder.switch_to_block(arm);
                Self::require_i64(builder, arity, expected_arity as i64);
                if expected_arity != 0 {
                    Self::require_nonzero(builder, fields);
                }
                let arm_env = env_with_operands(
                    (0..expected_arity).map(|index| {
                        let field = builder.ins().iadd_imm(fields, (index * 32) as i64);
                        LoweringOperand::Specialized(Lowered::BorrowedNativeValue { pointer: field })
                    }),
                    env,
                );
                // The single-case fast path is still case 0 of this match.
                let body = self.case_body_occurrence(static_origin, 0, &case.body)?;
                return self.lower_expr(builder, body, &arm_env);
            }
            let merge = join_plan
                .has_continuing_predecessor
                .then(|| builder.create_block());
            if let Some(merge) = merge {
                self.append_planned_join_params(builder, merge, join_plan);
            }
            let mut test_block = builder.current_block().expect("borrowed match block");
            let mut merge_kind = None;
            for (index, case) in cases.iter().enumerate() {
                let (expected_tag, expected_arity) =
                    borrowed_constructor_identity(&self.process_symbols, &case.constructor)
                        .ok_or_else(|| {
                            unsupported(
                                "Match",
                                format!("{} has no borrowed constructor identity", case.constructor),
                            )
                        })?;
                if case.binders != expected_arity {
                    return Err(unsupported(
                        "Match",
                        format!("{} borrowed arity mismatch", case.constructor),
                    ));
                }
                let arm = builder.create_block();
                let next = builder.create_block();
                if builder.current_block() != Some(test_block) {
                    builder.switch_to_block(test_block);
                }
                let selected = builder.ins().icmp_imm(
                    cranelift_codegen::ir::condcodes::IntCC::Equal,
                    tag,
                    expected_tag,
                );
                builder.ins().brif(selected, arm, &[], next, &[]);
                builder.switch_to_block(arm);
                Self::require_i64(builder, arity, expected_arity as i64);
                if expected_arity != 0 {
                    Self::require_nonzero(builder, fields);
                }
                let arm_env = env_with_operands(
                    (0..expected_arity).map(|index| {
                        let field = builder.ins().iadd_imm(fields, (index * 32) as i64);
                        LoweringOperand::Specialized(Lowered::BorrowedNativeValue { pointer: field })
                    }),
                    env,
                );
                let body = self.case_body_occurrence(static_origin, index, &case.body)?;
                let lowered = self.lower_expr(builder, body, &arm_env)?;
                if !self.seal_source_trap_branch(builder, &lowered)? {
                    let merge = merge.ok_or_else(|| {
                        backend_module(
                            "join plan omitted a merge despite a continuing predecessor".to_string(),
                        )
                    })?;
                    self.jump_planned_join_arm(
                        builder,
                        merge,
                        join_plan,
                        body.static_origin,
                        lowered,
                        &mut merge_kind,
                        "a borrowed `Match` arm",
                    )?;
                }
                test_block = next;
            }
            builder.switch_to_block(test_block);
            let failure = builder.ins().iconst(types::I64, -1);
            builder.ins().return_(&[failure]);
            let Some(merge) = merge else {
                let unreachable_continuation = builder.create_block();
                builder.switch_to_block(unreachable_continuation);
                return Ok(LoweringOperand::Specialized(Lowered::Trap(default.clone())));
            };
            self.finish_planned_join(
                builder,
                merge,
                join_plan,
                merge_kind,
                "a borrowed `Match` join",
            )
        }
}

impl<'a> Lowering<'a> {
        #[allow(clippy::too_many_arguments)]
        pub(super) fn lower_borrowed_option_match(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            present: cranelift_codegen::ir::Value,
            value: cranelift_codegen::ir::Value,
            none: &str,
            some: &str,
            cases: &[crate::RuntimeMatchCase],
            _default: &RuntimeTrap,
            static_origin: StaticOriginId,
            env: &[LoweringEnvironmentBinding],
        ) -> Result<LoweringOperand, CraneliftBackendError> {
            let join_plan = self.consumed_join_plan_token(static_origin)?;
            let merge = join_plan
                .has_continuing_predecessor
                .then(|| builder.create_block());
            if let Some(merge) = merge {
                self.append_planned_join_params(builder, merge, &join_plan);
            }
            let some_block = builder.create_block();
            let none_block = builder.create_block();
            let mut merge_kind = None;
            let mut terminal_trap = None;
            builder
                .ins()
                .brif(present, some_block, &[], none_block, &[]);
            for (block, symbol, fields) in [
                (some_block, some, vec![Lowered::Int { value, known: None }]),
                (none_block, none, Vec::new()),
            ] {
                builder.switch_to_block(block);
                let case = cases
                    .iter()
                    .enumerate()
                    .find(|(_, case)| case.constructor == symbol);
                let Some((index, case)) = case else {
                    let failure = builder.ins().iconst(types::I64, -1);
                    builder.ins().return_(&[failure]);
                    continue;
                };
                if case.binders != fields.len() {
                    return Err(unsupported("Match", "borrowed Option arity mismatch"));
                }
                let arm_env = env_with(fields, env);
                let body = self.case_body_occurrence(static_origin, index, &case.body)?;
                let lowered = self.lower_expr(builder, body, &arm_env)?;
                if let LoweringOperand::Specialized(Lowered::Trap(trap)) = &lowered {
                    terminal_trap.get_or_insert_with(|| trap.clone());
                }
                if self.seal_source_trap_branch(builder, &lowered)? {
                    continue;
                }
                let merge = merge.ok_or_else(|| {
                    backend_module(
                        "borrowed Option join omitted a merge despite a continuing predecessor"
                            .to_string(),
                    )
                })?;
                self.jump_planned_join_arm(
                    builder,
                    merge,
                    &join_plan,
                    body.static_origin,
                    lowered,
                    &mut merge_kind,
                    "Match",
                )?;
            }
            let Some(merge) = merge else {
                let unreachable_continuation = builder.create_block();
                builder.switch_to_block(unreachable_continuation);
                let trap = terminal_trap.ok_or_else(|| {
                    backend_module(
                        "borrowed Option join omitted both a continuing predecessor and a source \
                         trap"
                            .to_string(),
                    )
                })?;
                return Ok(LoweringOperand::Specialized(Lowered::Trap(trap)));
            };
            self.finish_planned_join(builder, merge, &join_plan, merge_kind, "Match")
        }
}

impl<'a> Lowering<'a> {
        #[allow(clippy::too_many_arguments)]
        pub(super) fn lower_dynamic_host_result_match(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            success: cranelift_codegen::ir::Value,
            error: Lowered,
            ok: Lowered,
            err_constructor: &str,
            ok_constructor: &str,
            cases: &[crate::RuntimeMatchCase],
            default: &RuntimeTrap,
            static_origin: StaticOriginId,
            env: &[LoweringEnvironmentBinding],
        ) -> Result<LoweringOperand, CraneliftBackendError> {
            // D8: the source traversal consumed the origin-keyed contract before
            // reaching this helper. Reborrow it before creating a block or lowering
            // either arm. The specialized HostResult scrutinee is not a selector
            // for the result representation.
            let join_plan = self.consumed_join_plan_token(static_origin)?;
            let merge = join_plan
                .has_continuing_predecessor
                .then(|| builder.create_block());
            if let Some(merge) = merge {
                #[cfg(test)]
                D8_JOIN_MERGES_CREATED.with(|count| count.set(count.get() + 1));
                self.append_planned_join_params(builder, merge, &join_plan);
                #[cfg(test)]
                if D8_JOIN_CONSUMPTION_MUTATION.with(std::cell::Cell::get)
                    == JoinConsumptionMutation::DispositionDynamicHostResultMerge
                {
                    self.function_local
                        .dispositioned_join_origins
                        .insert(static_origin);
                }
            }
            let ok_block = builder.create_block();
            let err_block = builder.create_block();
            let mut merge_kind = None;
            builder.ins().brif(success, ok_block, &[], err_block, &[]);
            for (block, constructor, payload) in [
                (ok_block, ok_constructor, ok),
                (err_block, err_constructor, error),
            ] {
                builder.switch_to_block(block);
                let Some((index, case)) = cases
                    .iter()
                    .enumerate()
                    .find(|(_, case)| case.constructor == constructor && case.binders == 1)
                else {
                    let failure = builder.ins().iconst(types::I64, -1);
                    builder.ins().return_(&[failure]);
                    continue;
                };
                let arm_env = env_with([payload], env);
                let body = self.case_body_occurrence(static_origin, index, &case.body)?;
                let lowered = self.lower_expr(builder, body, &arm_env)?;
                if self.seal_source_trap_branch(builder, &lowered)? {
                    continue;
                }
                let merge = merge.ok_or_else(|| {
                    backend_module(
                        "join plan omitted a merge despite a continuing predecessor".to_string(),
                    )
                })?;
                match join_plan.representation {
                    JoinResultRepresentation::NativeScalarPair => {
                        let (value, branch_kind) =
                            self.merge_scalar_branch(builder, &join_plan, lowered, "Match")?;
                        Self::record_scalar_merge_kind("Match", &mut merge_kind, branch_kind)?;
                        builder
                            .ins()
                            .jump(merge, &[value.tag.into(), value.payload.into()]);
                    }
                    JoinResultRepresentation::CarrierWord => {
                        let word =
                            self.carried_join_arm(builder, body.static_origin, lowered, None, "Match")?;
                        builder.ins().jump(merge, &[word.word.into()]);
                    }
                }
            }
            let Some(merge) = merge else {
                let unreachable_continuation = builder.create_block();
                builder.switch_to_block(unreachable_continuation);
                return Ok(LoweringOperand::Specialized(Lowered::Trap(default.clone())));
            };
            builder.switch_to_block(merge);
            match join_plan.representation {
                JoinResultRepresentation::NativeScalarPair => {
                    let pair = NativeScalarPairV1 {
                        tag: builder.block_params(merge)[0],
                        payload: builder.block_params(merge)[1],
                    };
                    Ok(LoweringOperand::Specialized(self.lowered_from_scalar_pair(
                        merge_kind.expect("HostResult emits a continuing closed alternative"),
                        pair,
                    )))
                }
                JoinResultRepresentation::CarrierWord => {
                    Ok(LoweringOperand::Carried(CarriedBoundaryWord {
                        word: builder.block_params(merge)[0],
                    }))
                }
            }
        }
}

impl<'a> Lowering<'a> {
        #[allow(clippy::too_many_arguments)]
        pub(super) fn lower_bounded_nat_match(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            nat: BoundedNatV1,
            structural: bool,
            cases: &[crate::RuntimeMatchCase],
            _default: &RuntimeTrap,
            static_origin: StaticOriginId,
            env: &[LoweringEnvironmentBinding],
        ) -> Result<LoweringOperand, CraneliftBackendError> {
            let join_plan = self.consumed_join_plan_token(static_origin)?;
            let zero = cases.iter().enumerate().find(|(_, case)| {
                case.constructor == self.process_symbols.nat_zero && case.binders == 0
            });
            let suc = cases.iter().enumerate().find(|(_, case)| {
                case.constructor == self.process_symbols.nat_suc && case.binders == 1
            });
            let (Some(zero), Some(suc)) = (zero, suc) else {
                return Err(unsupported(
                    "BoundedNat",
                    "structural Nat match requires exact Zero and Suc predecessor arms",
                ));
            };
            let zero_block = builder.create_block();
            let suc_block = builder.create_block();
            let merge = join_plan
                .has_continuing_predecessor
                .then(|| builder.create_block());
            if let Some(merge) = merge {
                self.append_planned_join_params(builder, merge, &join_plan);
            }
            let predecessor = nat.predecessor(builder);
            let is_zero =
                builder
                    .ins()
                    .icmp_imm(cranelift_codegen::ir::condcodes::IntCC::Equal, nat.value, 0);
            builder.ins().brif(is_zero, zero_block, &[], suc_block, &[]);
            let mut merge_kind = None;
            let mut terminal_trap = None;
            for (block, (index, case), predecessor) in [
                (zero_block, zero, None),
                (suc_block, suc, Some(predecessor)),
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
                let body = self.case_body_occurrence(static_origin, index, &case.body)?;
                let lowered = self.lower_expr(builder, body, &arm_env)?;
                if let LoweringOperand::Specialized(Lowered::Trap(trap)) = &lowered {
                    terminal_trap.get_or_insert_with(|| trap.clone());
                }
                if self.seal_source_trap_branch(builder, &lowered)? {
                    continue;
                }
                let merge = merge.ok_or_else(|| {
                    backend_module(
                        "join plan omitted a BoundedNat merge despite a continuing predecessor"
                            .to_string(),
                    )
                })?;
                self.jump_planned_join_arm(
                    builder,
                    merge,
                    &join_plan,
                    body.static_origin,
                    lowered,
                    &mut merge_kind,
                    "BoundedNat",
                )?;
            }
            let Some(merge) = merge else {
                let unreachable = builder.create_block();
                builder.switch_to_block(unreachable);
                let trap = terminal_trap.ok_or_else(|| {
                    backend_module(
                        "BoundedNat join omitted both a continuing predecessor and a source trap"
                            .to_string(),
                    )
                })?;
                return Ok(LoweringOperand::Specialized(Lowered::Trap(trap)));
            };
            self.finish_planned_join(builder, merge, &join_plan, merge_kind, "BoundedNat")
        }
}

impl<'a> Lowering<'a> {
        pub(super) fn lower_dynamic_constructor_match(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            dynamic: DynamicConstructorV1,
            continuation: DynamicConstructorContinuation<'_>,
        ) -> Result<LoweringOperand, CraneliftBackendError> {
            validate_dynamic_constructor_alternatives(
                dynamic
                    .alternatives
                    .iter()
                    .map(|alternative| (alternative.tag, alternative.constructor.as_str())),
            )?;

            let source_default = match continuation {
                DynamicConstructorContinuation::Ordinary { default, .. }
                | DynamicConstructorContinuation::Producer { default, .. } => default,
            };
            let static_origin = match continuation {
                DynamicConstructorContinuation::Ordinary { static_origin, .. }
                | DynamicConstructorContinuation::Producer { static_origin, .. } => static_origin,
            };
            let join_plan = self.consumed_join_plan_token(static_origin)?;
            let merge = join_plan.has_continuing_predecessor.then(|| {
                let merge = builder.create_block();
                self.append_planned_join_params(builder, merge, &join_plan);
                merge
            });
            let mut test_block = builder
                .current_block()
                .expect("dynamic constructor match block");
            let mut merge_kind = None;
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
                let (cases, default, env, static_origin) = match continuation {
                    DynamicConstructorContinuation::Ordinary {
                        cases,
                        default,
                        env,
                        static_origin,
                    }
                    | DynamicConstructorContinuation::Producer {
                        cases,
                        default,
                        env,
                        static_origin,
                        ..
                    } => (cases, default, env, static_origin),
                };
                let (index, case) = match select_dynamic_constructor_case(cases, &alternative, default)?
                {
                    Ok(selected) => selected,
                    Err(_owned_default) => {
                        let failure = builder.ins().iconst(types::I64, -4);
                        builder.ins().return_(&[failure]);
                        test_block = next;
                        continue;
                    }
                };
                let arm_env = materialize_dynamic_constructor_env(&alternative, env);
                let body = self.case_body_occurrence(static_origin, index, &case.body)?;
                let lowered = match continuation {
                    DynamicConstructorContinuation::Ordinary { .. } => {
                        self.lower_expr(builder, body, &arm_env)?
                    }
                    DynamicConstructorContinuation::Producer { eliminators, .. } => {
                        self.lower_computational_producer_expr(builder, body, &arm_env, eliminators)?
                    }
                };
                if self.seal_source_trap_branch(builder, &lowered)? {
                    test_block = next;
                    continue;
                }
                let merge = merge.ok_or_else(|| {
                    backend_module(
                        "join plan omitted a DynamicConstructor merge despite a continuing \
                         predecessor"
                            .to_string(),
                    )
                })?;
                self.jump_planned_join_arm(
                    builder,
                    merge,
                    &join_plan,
                    body.static_origin,
                    lowered,
                    &mut merge_kind,
                    "DynamicConstructor",
                )?;
                test_block = next;
            }
            builder.switch_to_block(test_block);
            let malformed = builder
                .ins()
                .iconst(types::I64, MALFORMED_DYNAMIC_CONSTRUCTOR_STATUS);
            builder.ins().return_(&[malformed]);
            let Some(merge) = merge else {
                let unreachable_continuation = builder.create_block();
                builder.switch_to_block(unreachable_continuation);
                return Ok(LoweringOperand::Specialized(Lowered::Trap(
                    source_default.clone(),
                )));
            };
            self.finish_planned_join(builder, merge, &join_plan, merge_kind, "DynamicConstructor")
        }
}

impl<'a> Lowering<'a> {
        /// Take the pre-emission result contract for this exact source join.
        ///
        /// Consumption is recorded before a merge block can be created. A second
        /// call to this method in one generated function is therefore a planner /
        /// lowering disagreement. Legitimate traversal re-entry goes through
        /// [`Self::enter_source_occurrence_plan`] and only reborrows the token.
        pub(super) fn consume_join_plan(
            &mut self,
            origin: StaticOriginId,
        ) -> Result<JoinPlanToken, CraneliftBackendError> {
            let token = self.static_transition_plan.join_plan_token(origin)?;
            if token.origin != origin {
                return Err(backend_module(
                    "source join consumed a result plan for a different origin".to_string(),
                ));
            }
            #[cfg(test)]
            match D8_JOIN_CONSUMPTION_MUTATION.with(std::cell::Cell::get) {
                JoinConsumptionMutation::Exact => {}
                JoinConsumptionMutation::SkipFirst
                    if self.function_local.consumed_join_origins.is_empty() =>
                {
                    return Ok(token);
                }
                JoinConsumptionMutation::DuplicateFirst
                    if self.function_local.consumed_join_origins.is_empty() =>
                {
                    self.function_local.consumed_join_origins.insert(origin);
                }
                JoinConsumptionMutation::SkipFirst
                | JoinConsumptionMutation::DuplicateFirst
                | JoinConsumptionMutation::IncludeStaticallyUnselected
                | JoinConsumptionMutation::OmitFirstStaticallyUnselectedMatchCase
                | JoinConsumptionMutation::OmitSourceMachineComputationalMatchSelection
                | JoinConsumptionMutation::MaterializeFirstUnselectedMatchJoin
                | JoinConsumptionMutation::AttachEntryToFirstMaterializedDead
                | JoinConsumptionMutation::ForceMaterializedDeadOverlapWithEntry
                | JoinConsumptionMutation::DispositionDynamicHostResultMerge => {}
            }
            if !self.function_local.consumed_join_origins.insert(origin) {
                return Err(backend_module(
                    "one source join consumed its static result plan more than once".to_string(),
                ));
            }
            Ok(token)
        }
}

impl<'a> Lowering<'a> {
        /// Disposition every planned join in a statically unselected source branch.
        ///
        /// The planner derives the subtree from its validated positional-child
        /// inventory and stops at declared-unit owner boundaries. Lowering supplies
        /// only the exact branch root it proved dead; it maintains no second source
        /// spelling inventory.
        pub(super) fn disposition_statically_unselected_source_subtree(
            &mut self,
            root: StaticOriginId,
        ) -> Result<(), CraneliftBackendError> {
            #[cfg(test)]
            if D8_JOIN_CONSUMPTION_MUTATION.with(std::cell::Cell::get)
                == JoinConsumptionMutation::IncludeStaticallyUnselected
            {
                return Ok(());
            }
            let joins = self
                .static_transition_plan
                .source_join_origins_in_owner_subtree(root)?;
            for origin in joins {
                // **`RT-MATERIALIZED-DEAD-JOIN-RECONCILE` `D1` -- a join lowering
                // actually CONSUMED is never dispositioned dead.**
                //
                // `finalize_join_disposition` requires `consumed united with
                // dispositioned` to cover `required` and to be DISJOINT, so the
                // two sets are one partition. An origin that lowering emitted and
                // this walk also dispositions lands in both halves: the overlap
                // check passes only because the CFG half runs later, where it
                // surfaces as *"materialized-but-dead source join ... retained a
                // reachable block"* -- a contradiction reported one plane away
                // from where it was created.
                //
                // The reachability of the arm is NOT the question here, and
                // treating it as one is what made two successive re-entry
                // predicates fail in opposite directions. Widening a re-entry
                // predicate to retain this arm removes it from `dispositioned`
                // without adding it to `consumed`, so the SAME partition breaks
                // through the other face -- *"left planned source join ... neither
                // emitted nor statically unselected"* -- and it does so for every
                // genuinely unselected arm in the program, not just this one.
                // MEASURED: the narrowest such widening cost 12 lib regressions
                // and still did not close the witness.
                //
                // Consumption is the authority because it is a FACT lowering
                // already established by emitting the block, not an inference
                // about control flow. Deciding from it makes the partition
                // consistent by construction.
                #[cfg(test)]
                let force_overlap = D8_JOIN_CONSUMPTION_MUTATION.with(std::cell::Cell::get)
                    == JoinConsumptionMutation::ForceMaterializedDeadOverlapWithEntry;
                #[cfg(not(test))]
                let force_overlap = false;
                if !force_overlap && self.function_local.consumed_join_origins.contains(&origin) {
                    continue;
                }
                self.function_local
                    .dispositioned_join_origins
                    .insert(origin);
            }
            Ok(())
        }
}

impl<'a> Lowering<'a> {
        /// Record one case reached by static `Match` selection.
        ///
        /// `Match` lays its validated positional children out as the scrutinee
        /// followed by every case body. The complete root population comes from the
        /// planner's checked child inventory; lowering supplies only the reached
        /// case index. An empty selection records the default/no-match route.
        ///
        /// This deliberately defers disposition until generated-function closure.
        /// A recursive producer can revisit the same source occurrence and select a
        /// second case, so the emission-reachable population is the union of every
        /// observed selection, not the first constructor seen.
        pub(super) fn disposition_statically_unselected_match_cases(
            &mut self,
            match_origin: StaticOriginId,
            selected_case: Option<usize>,
        ) -> Result<(), CraneliftBackendError> {
            let case_bodies = self
                .static_transition_plan
                .source_match_case_body_origins(match_origin)?;
            let case_count = case_bodies.len();
            if selected_case.is_some_and(|index| index >= case_count) {
                return Err(backend_module(
                    "selected source Match case is outside the validated child population".to_string(),
                ));
            }
            #[cfg(test)]
            if matches!(
                D8_JOIN_CONSUMPTION_MUTATION.with(std::cell::Cell::get),
                JoinConsumptionMutation::MaterializeFirstUnselectedMatchJoin
                    | JoinConsumptionMutation::AttachEntryToFirstMaterializedDead
                    | JoinConsumptionMutation::ForceMaterializedDeadOverlapWithEntry
            ) {
                let mut materialized = None;
                for (index, root) in case_bodies.iter().copied().enumerate() {
                    if selected_case == Some(index) {
                        continue;
                    }
                    if let Some(origin) = self
                        .static_transition_plan
                        .source_join_origins_in_owner_subtree(root)?
                        .into_iter()
                        .next()
                    {
                        materialized = Some(origin);
                        break;
                    }
                }
                if let Some(origin) = materialized {
                    if !self.function_local.consumed_join_origins.contains(&origin) {
                        self.consume_join_plan(origin)?;
                    }
                }
            }
            let reached = self
                .function_local
                .emission_reachable_match_cases
                .entry(match_origin)
                .or_default();
            if let Some(index) = selected_case {
                reached.insert(index);
            }
            Ok(())
        }
}

impl<'a> Lowering<'a> {
        /// Close every recorded static `Match` selection against its validated
        /// positional-child population.
        fn close_statically_unselected_match_cases(&mut self) -> Result<(), CraneliftBackendError> {
            let reached = self
                .function_local
                .emission_reachable_match_cases
                .iter()
                .map(|(origin, cases)| (*origin, cases.clone()))
                .collect::<Vec<_>>();
            // `RT-DECL-CLOSURE-PORT` `D5a` checkpoint 3 — the planner-issued
            // recursive predecessors. Read once, outside the loop, and from the
            // planner rather than from anything observed here.
            let recursive_predecessors = self
                .static_transition_plan
                .source_machine_recursive_predecessor_origins()?;
            for (match_origin, reached_cases) in reached {
                let case_bodies = self
                    .static_transition_plan
                    .source_match_case_body_origins(match_origin)?;
                // ⭐ **THE UNION `D5a` checkpoint 3 repairs.** Final reachability is
                // the initial selection PLUS every planner-issued source-machine
                // recursive predecessor -- not the initial selection alone.
                //
                // A predecessor's contribution here is the planner's **closed case
                // population**, because what re-enters the match is the *return* of
                // a generated call: a carried word with no compile-time constructor
                // template, so no case can be ruled out for it. Concretely, the
                // initial scrutinee selecting `Vis` used to disposition the `Ret`
                // arm's whole subtree, while the emitted causal call's return edge
                // makes `Ret` genuinely reachable -- lowering then both materialized
                // that arm's join and dispositioned it, and the finished CFG
                // correctly exposed the contradiction.
                //
                // ⚠ The ruling also says *"specialized re-entry keeps its exact
                // selected case"*. That describes the specialization **body**, which
                // lowers its own selected alternative directly and never re-enters
                // this match; it contributes to that generated function's own
                // population, not to this one. ⇒ There is deliberately **no**
                // exact-alternative narrowing at this seat: a narrowing that can
                // never fire is a branch that rots, and adding one here would read
                // as covering a case it could not reach.
                //
                // ⛔ The validator is untouched and this does not force any block
                // dead or delete origin 25. The repair is to stop asserting a
                // deadness that was never true.
                let final_reachable: BTreeSet<usize> =
                    if recursive_predecessors.contains(&match_origin) {
                        (0..case_bodies.len()).collect()
                    } else {
                        reached_cases
                    };
                #[cfg(test)]
                let mut omitted_for_mutation = false;
                for (index, root) in case_bodies.into_iter().enumerate() {
                    if final_reachable.contains(&index) {
                        continue;
                    }
                    #[cfg(test)]
                    if !omitted_for_mutation
                        && D8_JOIN_CONSUMPTION_MUTATION.with(std::cell::Cell::get)
                            == JoinConsumptionMutation::OmitFirstStaticallyUnselectedMatchCase
                    {
                        omitted_for_mutation = true;
                        continue;
                    }
                    self.disposition_statically_unselected_source_subtree(root)?;
                }
            }
            Ok(())
        }
}

impl<'a> Lowering<'a> {
        /// Reborrow a contract after the source traversal has consumed it.
        ///
        /// Merge helpers may be reached long after a computational eliminator was
        /// installed. They do not constitute another source occurrence and must
        /// therefore neither mint nor consume a second contract.
        pub(super) fn consumed_join_plan_token(
            &self,
            origin: StaticOriginId,
        ) -> Result<JoinPlanToken, CraneliftBackendError> {
            if !self.function_local.consumed_join_origins.contains(&origin) {
                return Err(backend_module(format!(
                    "source join {origin:?} requested its static result plan before consumption",
                )));
            }
            self.static_transition_plan.join_plan_token(origin)
        }
}

impl<'a> Lowering<'a> {
        /// Close AC-14 at the generated-function boundary.
        ///
        /// Duplicate consumption already rejects at [`Self::consume_join_plan`].
        /// This equality supplies the missing other direction: every join in the
        /// planner's closed owner partition must either be reached exactly once by
        /// emission or be structurally dispositioned under a statically unselected
        /// branch, and no join owned by another function may appear here.
        pub(super) fn validate_join_plan_consumption(
            &mut self,
            function: PredeclaredFunctionId,
        ) -> Result<(), CraneliftBackendError> {
            self.close_statically_unselected_match_cases()?;
            let required = self
                .static_transition_plan
                .required_join_origins(function)?;
            self.finalize_join_disposition(&required)
        }
}

impl<'a> Lowering<'a> {
        fn finalize_join_disposition(
            &mut self,
            required: &BTreeSet<StaticOriginId>,
        ) -> Result<(), CraneliftBackendError> {
            // `D2b` — OBSERVATION ONLY. Records the three sets at the instant the
            // guard reads them, so a control can assert the accounting rather than
            // infer it from a refusal message. It removes nothing, adds nothing and
            // changes no result.
            #[cfg(test)]
            LRC_D2B_JOIN_OBSERVATION.with(|cell| {
                cell.borrow_mut().push((
                    required.clone(),
                    self.function_local.consumed_join_origins.clone(),
                    self.function_local.dispositioned_join_origins.clone(),
                ));
            });
            #[cfg(test)]
            {
                let mutation = D8_JOIN_CONSUMPTION_MUTATION.with(std::cell::Cell::get);
                if matches!(
                    mutation,
                    JoinConsumptionMutation::OmitSourceMachineComputationalMatchSelection
                ) && self
                    .function_local
                    .consumed_join_origins
                    .is_disjoint(&self.function_local.dispositioned_join_origins)
                {
                    if let Some(origin) = self
                        .function_local
                        .consumed_join_origins
                        .iter()
                        .next()
                        .copied()
                    {
                        self.function_local
                            .dispositioned_join_origins
                            .insert(origin);
                    }
                }
            }

            let mut covered = self.function_local.consumed_join_origins.clone();
            covered.extend(
                self.function_local
                    .dispositioned_join_origins
                    .iter()
                    .copied(),
            );
            if let Some(origin) = covered.difference(required).next() {
                return Err(backend_module(format!(
                    "source join {origin:?} was classified outside its owning function",
                )));
            }
            if let Some(origin) = required.difference(&covered).next() {
                return Err(backend_module(format!(
                    "function left planned source join {origin:?} neither emitted nor statically unselected",
                )));
            }
            if self.function_local.join_disposition_finalized {
                return Err(backend_module(
                    "generated function finalized its source join disposition more than once"
                        .to_string(),
                ));
            }
            self.function_local.final_reachable_join_origins = required
                .difference(&self.function_local.dispositioned_join_origins)
                .copied()
                .collect();
            self.function_local.join_disposition_finalized = true;
            Ok(())
        }
}

impl<'a> Lowering<'a> {
        /// Validate the materialized-but-dead half of the final join disposition
        /// against the completed function CFG.
        ///
        /// A consumed token with no recorded merge block is metadata-only
        /// materialization and has no CFG repair obligation. Every recorded block
        /// for an origin later classified dead must be unreachable from entry,
        /// have no live predecessor, and contribute no block parameter to a
        /// reachable instruction. The ordinary Cranelift verifier subsequently
        /// closes the remaining SSA dominance and use-def obligations.
        pub(super) fn validate_materialized_dead_join_cfg(
            &self,
            function: PredeclaredFunctionId,
            func: &Function,
        ) -> Result<(), CraneliftBackendError> {
            let required = self
                .static_transition_plan
                .required_join_origins(function)?;
            self.validate_materialized_dead_join_cfg_for(&required, func)
        }
}

impl<'a> Lowering<'a> {
        fn validate_materialized_dead_join_cfg_for(
            &self,
            required: &BTreeSet<StaticOriginId>,
            func: &Function,
        ) -> Result<(), CraneliftBackendError> {
            if !self.function_local.join_disposition_finalized {
                return Err(backend_module(
                    "generated function checked materialized joins before final disposition"
                        .to_string(),
                ));
            }
            let mut final_covered = self.function_local.final_reachable_join_origins.clone();
            final_covered.extend(
                self.function_local
                    .dispositioned_join_origins
                    .iter()
                    .copied(),
            );
            if &final_covered != required
                || !self
                    .function_local
                    .final_reachable_join_origins
                    .is_disjoint(&self.function_local.dispositioned_join_origins)
            {
                return Err(backend_module(
                    "generated function has an incomplete or overlapping final join disposition"
                        .to_string(),
                ));
            }
            let cfg = ControlFlowGraph::with_function(func);
            let entry = func
                .layout
                .entry_block()
                .ok_or_else(|| backend_module("generated function has no entry block".to_string()))?;
            let mut reachable = BTreeSet::from([entry]);
            let mut pending = vec![entry];
            while let Some(block) = pending.pop() {
                for successor in cfg.succ_iter(block) {
                    if reachable.insert(successor) {
                        pending.push(successor);
                    }
                }
            }

            let overlap = self
                .function_local
                .consumed_join_origins
                .intersection(&self.function_local.dispositioned_join_origins)
                .copied()
                .collect::<Vec<_>>();
            for origin in overlap {
                if !required.contains(&origin) {
                    return Err(backend_module(format!(
                        "materialized-but-dead source join {origin:?} escaped its owning function",
                    )));
                }
                let blocks = self
                    .function_local
                    .materialized_join_blocks
                    .get(&origin)
                    .into_iter()
                    .flat_map(|blocks| blocks.iter().copied())
                    .collect::<Vec<_>>();
                #[cfg(test)]
                let blocks = match D8_JOIN_CONSUMPTION_MUTATION.with(std::cell::Cell::get) {
                    JoinConsumptionMutation::AttachEntryToFirstMaterializedDead
                    | JoinConsumptionMutation::ForceMaterializedDeadOverlapWithEntry
                    | JoinConsumptionMutation::OmitSourceMachineComputationalMatchSelection => {
                        let mut blocks = blocks;
                        blocks.push(entry);
                        blocks
                    }
                    JoinConsumptionMutation::Exact
                    | JoinConsumptionMutation::SkipFirst
                    | JoinConsumptionMutation::DuplicateFirst
                    | JoinConsumptionMutation::IncludeStaticallyUnselected
                    | JoinConsumptionMutation::OmitFirstStaticallyUnselectedMatchCase
                    | JoinConsumptionMutation::MaterializeFirstUnselectedMatchJoin
                    | JoinConsumptionMutation::DispositionDynamicHostResultMerge => blocks,
                };
                for block in blocks {
                    if reachable.contains(&block) {
                        return Err(backend_module(format!(
                            "materialized-but-dead source join {origin:?} retained a reachable block",
                        )));
                    }
                    if cfg
                        .pred_iter(block)
                        .any(|predecessor| reachable.contains(&predecessor.block))
                    {
                        return Err(backend_module(format!(
                            "materialized-but-dead source join {origin:?} retained a live predecessor",
                        )));
                    }
                    let params = func.dfg.block_params(block);
                    for reachable_block in &reachable {
                        for inst in func.layout.block_insts(*reachable_block) {
                            if func
                                .dfg
                                .inst_args(inst)
                                .iter()
                                .any(|argument| params.contains(argument))
                            {
                                return Err(backend_module(format!(
                                    "materialized-but-dead source join {origin:?} retained a reachable use",
                                )));
                            }
                        }
                    }
                }
            }
            Ok(())
        }
}

impl<'a> Lowering<'a> {
        /// ⭐ **A JOIN — `§2h` calls branch/join forwarding phase-bearing, so the
        /// arm arrives as a [`LoweringOperand`] and the phase boundary is taken
        /// HERE, once, rather than at each of the callers.**
        ///
        /// ⚠ the tagged native scalar join merges `(tag, payload)` lanes of a native scalar. A carried
        /// boundary word has no such pair, so it fails closed via
        /// [`LoweringOperand::specialized_join_arm`] — ⛔ a *pending* boundary, not
        /// a final one; see that method for why the distinction is kept.
        pub(super) fn merge_scalar_branch(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            join_plan: &JoinPlanToken,
            lowered: LoweringOperand,
            construct: &'static str,
        ) -> Result<(NativeScalarPairV1, ScalarMergeKind), CraneliftBackendError> {
            if join_plan.representation != JoinResultRepresentation::NativeScalarPair {
                return Err(backend_module(
                    "carrier-result join reached a native-only scalar merge consumer".to_string(),
                ));
            }
            self.merge_scalar_operand(builder, lowered, None, construct)
        }
}

impl<'a> Lowering<'a> {
        /// Consume one scalar-valued operand at a private typed CFG boundary.
        ///
        /// The surrounding source join may use `CarrierWord` storage. Once the
        /// consumer has established the scalar kind from a specialized arm, a
        /// carried sibling can be decoded back to that exact kind without changing
        /// constructor meaning. This is intentionally separate from
        /// [`Self::merge_scalar_branch`]: callers that own an ordinary source join
        /// must still obey its planned representation.
        pub(super) fn merge_scalar_operand(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            lowered: LoweringOperand,
            required_kind: Option<ScalarMergeKind>,
            construct: &'static str,
        ) -> Result<(NativeScalarPairV1, ScalarMergeKind), CraneliftBackendError> {
            if let LoweringOperand::Carried(word) = lowered {
                let required_kind = required_kind.ok_or_else(|| {
                    backend_module(
                        "a carried scalar reached an untyped private merge consumer".to_string(),
                    )
                })?;
                let boundary_tag = builder
                    .ins()
                    .band_imm(word.word, crate::boundary_value::BOUNDARY_TAG_MASK as i64);
                let (expected_tag, native_tag) = match required_kind {
                    ScalarMergeKind::Int => (
                        BoundaryTag::ImmediateInt,
                        Self::carrier_small_marker(builder),
                    ),
                    ScalarMergeKind::Bool => (
                        BoundaryTag::ImmediateBool,
                        builder.ins().iconst(types::I64, 0),
                    ),
                    ScalarMergeKind::StructuralNat => (
                        BoundaryTag::ImmediateStructuralNat,
                        builder.ins().iconst(types::I64, 0),
                    ),
                    ScalarMergeKind::ExitCode => (
                        BoundaryTag::ImmediateExitStatus,
                        builder.ins().iconst(types::I64, 0),
                    ),
                    ScalarMergeKind::RecursiveBackedge => {
                        return Err(unsupported(
                            construct,
                            "a carried word cannot mint a recursive-backedge control marker",
                        ));
                    }
                };
                Self::require_i64(builder, boundary_tag, expected_tag as i64);
                let payload = self.emit_carrier_scalar(builder, word)?;
                return Ok((
                    NativeScalarPairV1 {
                        tag: native_tag,
                        payload,
                    },
                    required_kind,
                ));
            }
            let lowered = lowered.specialized_join_arm(construct)?;
            if required_kind == Some(ScalarMergeKind::ExitCode) {
                let lowered = Self::unwrap_terminal_ret(lowered);
                let zero_tag = builder.ins().iconst(types::I64, 0);
                return match lowered {
                    Lowered::RecursiveBackedge => Ok((
                        NativeScalarPairV1 {
                            tag: zero_tag,
                            payload: builder.ins().iconst(types::I64, 0),
                        },
                        ScalarMergeKind::RecursiveBackedge,
                    )),
                    Lowered::ProcessExitStatus { value } => Ok((
                        NativeScalarPairV1 {
                            tag: zero_tag,
                            payload: value,
                        },
                        ScalarMergeKind::ExitCode,
                    )),
                    lowered if self.process_object => Ok((
                        NativeScalarPairV1 {
                            tag: zero_tag,
                            payload: self.emit_process_exit_status(builder, lowered),
                        },
                        ScalarMergeKind::ExitCode,
                    )),
                    _ => Err(unsupported(
                        construct,
                        "checked ExitCode join is unavailable outside process-object lowering",
                    )),
                };
            }
            let checked_root_exit_representation = self.has_checked_root_exit_representation();
            let lowered = if checked_root_exit_representation {
                Self::unwrap_terminal_ret(lowered)
            } else {
                lowered
            };
            let zero_tag = builder.ins().iconst(types::I64, 0);
            #[cfg(any(test, feature = "dasm-c2-observation"))]
            let observation = if DASM_C2_SCALAR_MERGE_OBSERVATION_ENABLED.get() {
                let observed_operand_kind = lowered_value_kind(&lowered);
                let observed_constructor = match &lowered {
                    Lowered::Constructor { constructor, .. } => Some(constructor.clone()),
                    _ => None,
                };
                Some((observed_operand_kind, observed_constructor))
            } else {
                None
            };
            let result = match lowered {
                Lowered::RecursiveBackedge => Ok((
                    NativeScalarPairV1 {
                        tag: zero_tag,
                        payload: builder.ins().iconst(types::I64, 0),
                    },
                    ScalarMergeKind::RecursiveBackedge,
                )),
                Lowered::Int { value, known } => Ok((
                    NativeScalarPairV1 {
                        tag: self.native_int_tag(builder, value, known)?,
                        payload: value,
                    },
                    ScalarMergeKind::Int,
                )),
                Lowered::Bool { value, .. } => Ok((
                    NativeScalarPairV1 {
                        tag: zero_tag,
                        payload: value,
                    },
                    ScalarMergeKind::Bool,
                )),
                Lowered::StructuralNat(nat) => Ok((
                    NativeScalarPairV1 {
                        tag: zero_tag,
                        payload: nat.value,
                    },
                    ScalarMergeKind::StructuralNat,
                )),
                Lowered::Constructor {
                    constructor, args, ..
                } if args.is_empty()
                    && (constructor == self.process_symbols.bool_true
                        || constructor == self.process_symbols.bool_false) =>
                {
                    Ok((
                        NativeScalarPairV1 {
                            tag: zero_tag,
                            payload: builder.ins().iconst(
                                types::I64,
                                i64::from(constructor == self.process_symbols.bool_true),
                            ),
                        },
                        ScalarMergeKind::Bool,
                    ))
                }
                Lowered::ProcessExitStatus { value } => Ok((
                    NativeScalarPairV1 {
                        tag: zero_tag,
                        payload: value,
                    },
                    ScalarMergeKind::ExitCode,
                )),
                lowered if checked_root_exit_representation => Ok((
                    NativeScalarPairV1 {
                        tag: zero_tag,
                        payload: self.emit_process_exit_status(builder, lowered),
                    },
                    ScalarMergeKind::ExitCode,
                )),
                _ => Err(unsupported(
                    construct,
                    "dynamic arms must produce scalar Int or Bool values",
                )),
            };
            #[cfg(any(test, feature = "dasm-c2-observation"))]
            if let Some((observed_operand_kind, observed_constructor)) = observation {
                dasm_c2_record_scalar_merge(DasmC2ScalarMergeObservation {
                    construct,
                    operand_kind: observed_operand_kind,
                    constructor: observed_constructor,
                    admitted: result.is_ok(),
                });
            }
            result
        }
}

impl<'a> Lowering<'a> {
        fn record_scalar_merge_kind(
            construct: &'static str,
            expected: &mut Option<ScalarMergeKind>,
            kind: ScalarMergeKind,
        ) -> Result<(), CraneliftBackendError> {
            if kind == ScalarMergeKind::RecursiveBackedge {
                return Ok(());
            }
            match expected {
                Some(expected) if *expected != kind => Err(unsupported(
                    construct,
                    "dynamic native arms disagree on scalar result kind",
                )),
                Some(_) => Ok(()),
                None => {
                    *expected = Some(kind);
                    Ok(())
                }
            }
        }
}

impl LoweringOperand {
        /// ⭐ **A BRANCH/JOIN ARM's typed phase boundary — deliberately a distinct
        /// method from [`Self::specialized_at`], because it records a distinct
        /// fact.**
        ///
        /// `§2h` names *"branch/join forwarding"* phase-bearing, so a join is ⛔
        /// **not** a specialized-only leaf: the reason a `Carried` fails closed here
        /// is not *"this surface reads a template"* but *"this join merges native
        /// scalar lanes and `C1` has not built a carried lane for it."* ⇒ Every call
        /// of this method is an **inventory entry** for the join work, and
        /// `grep`ping the name is how that inventory is read back.
        ///
        /// ⚠ Collapsing the two into one helper would be the cheaper diff and the
        /// worse artifact: it would erase, at exactly the sites that need it, the
        /// difference between a boundary that is *final* and one that is *pending*.
        fn specialized_join_arm(self, join: &'static str) -> Result<Lowered, CraneliftBackendError> {
            match self {
                LoweringOperand::Specialized(lowered) => Ok(lowered),
                LoweringOperand::Carried(_) => Err(unsupported(
                    "BoundaryCarrier",
                    format!(
                        "{join} merges native scalar lanes and has no carried lane; a boundary word \
                         cannot cross it until that join carries the phase"
                    ),
                )),
            }
        }
}

impl<'a> Lowering<'a> {
        /// ⭐ Takes the **operand**, not a template, and needs no phase boundary:
        /// *"is this branch a trap?"* has a total answer in both phases. A carried
        /// boundary word is never a trap — `Lowered::Trap` is a compile-time
        /// refusal, and the producer refuses to transfer one — so the `Carried`
        /// case answers `false` and the branch is left unsealed, which is the same
        /// answer any non-trap specialized value gets.
        pub(super) fn seal_source_trap_branch(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            lowered: &LoweringOperand,
        ) -> Result<bool, CraneliftBackendError> {
            let LoweringOperand::Specialized(Lowered::Trap(trap)) = lowered else {
                return Ok(false);
            };
            let status = self.emit_current_trap(builder, trap)?;
            builder.ins().return_(&[status]);
            Ok(true)
        }
}

impl<'a> Lowering<'a> {
        pub(super) fn emit_current_trap(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            trap: &RuntimeTrap,
        ) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
            let identity = self.static_transition_plan.trap_identity(trap)?;
            match self.function_local.trap_exit {
                Some(TrapExitAuthority::UnitFrame { slots, trap_offset }) => {
                    #[cfg(test)]
                    let identity_word =
                        match TRAP_IDENTITY_MUTATION.with(std::cell::Cell::get) {
                            TrapIdentityMutation::Exact => identity.abi_word(),
                            TrapIdentityMutation::Zero => 0,
                            TrapIdentityMutation::Substitute => identity
                                .abi_word()
                                .checked_add(1)
                                .expect("planner trap identity fits below i64::MAX"),
                        };
                    #[cfg(not(test))]
                    let identity_word = identity.abi_word();
                    #[cfg(test)]
                    px8tr_record_trap_provenance(Px8trTrapProvenanceEvent::PlannedTrapEmitted {
                        trap: trap.clone(),
                        seat: PlannedTrapSeat::UnitTrapWord,
                        planned_identity: identity.abi_word(),
                        emitted_word: identity_word,
                    });
                    let word = builder.ins().iconst(types::I64, identity_word);
                    builder
                        .ins()
                        .store(MemFlags::trusted(), word, slots, trap_offset);
                    Ok(builder.ins().iconst(types::I64, 0))
                }
                Some(TrapExitAuthority::Root {
                    process_sentinel,
                    source_authorized: true,
                }) => {
                    if process_sentinel {
                        #[cfg(test)]
                        px8tr_record_trap_provenance(Px8trTrapProvenanceEvent::PlannedTrapEmitted {
                            trap: trap.clone(),
                            seat: PlannedTrapSeat::RootProcessSentinel,
                            planned_identity: identity.abi_word(),
                            emitted_word: -4,
                        });
                        Ok(builder.ins().iconst(types::I64, -4))
                    } else {
                        let token = (identity.abi_word()
                            << crate::cranelift_backend::compiled::ROOT_TRAP_TOKEN_SHIFT)
                            | crate::cranelift_backend::compiled::ROOT_TRAP_TOKEN_TAG;
                        #[cfg(test)]
                        px8tr_record_trap_provenance(Px8trTrapProvenanceEvent::PlannedTrapEmitted {
                            trap: trap.clone(),
                            seat: PlannedTrapSeat::RootTrapToken,
                            planned_identity: identity.abi_word(),
                            emitted_word: token,
                        });
                        let word = builder.ins().iconst(types::I64, identity.abi_word());
                        let shifted = builder.ins().ishl_imm(
                            word,
                            crate::cranelift_backend::compiled::ROOT_TRAP_TOKEN_SHIFT,
                        );
                        Ok(builder.ins().bor_imm(
                            shifted,
                            crate::cranelift_backend::compiled::ROOT_TRAP_TOKEN_TAG,
                        ))
                    }
                }
                None => Err(backend_module(
                    "trap branch has no generated-unit TrapWord lane".to_string(),
                )),
                Some(TrapExitAuthority::Root {
                    source_authorized: false,
                    ..
                }) => Err(backend_module(
                    "generated function has no source-trap authority".to_string(),
                )),
            }
        }
}

impl FunctionLocalRefs {
    pub(super) fn bind_unit_trap_frame(
        &mut self,
        slots: cranelift_codegen::ir::Value,
        trap_offset: i32,
    ) -> Result<(), CraneliftBackendError> {
        if self.trap_exit.is_some() {
            return Err(backend_module(
                "unit trap frame was bound to a function without unit authority".to_string(),
            ));
        }
        self.trap_exit = Some(TrapExitAuthority::UnitFrame { slots, trap_offset });
        Ok(())
    }
}

