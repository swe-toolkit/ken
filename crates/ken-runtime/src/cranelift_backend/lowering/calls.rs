//! The calls and returns emitter -- declared-call emission, residual and
//! recursor call lowering, return emission, and the callee-side checks.
//!
//! `RT-EMITTER-CALLS-RETURNS-SPLIT` `D1`. Extends the `boundary.rs`/`source.rs`
//! seam (items 11/12): the owner traced in this item's D0 ledger (15 addenda,
//! `docs/program/issues/RT-EMITTER-CALLS-RETURNS-SPLIT.md`) relocates here from
//! `core.rs` and `mod.rs`, moved verbatim -- each moved method now sits in its
//! own small `impl<'a> Lowering<'a>` block, matching `source.rs`'s own shape,
//! since Rust does not require a type's inherent methods to share one `impl`
//! block or one file. Every other type the moving methods merely manipulate
//! (`Lowering`, `Lowered`, `LoweringOperand`, `FunctionLocalRefs`, and
//! siblings) stays declared at the `mod.rs` hub -- hub-stays/methods-move,
//! the same shape item 10/12 established.
//!
//! Eleven widenings from private to `pub(super)`, all load-bearing rather
//! than cosmetic (named in the `D1` ledger addendum, not silent): `call_static_
//! worker`, `validate_retained_callable_capture_contract`, `call_declared_
//! unit_target`, `call_declared_declaration_unit`, `decode_direct_callee`,
//! `unwrap_terminal_ret`, `emit_process_exit_status`, the `RECURSIVE_POSITION_
//! UNIT_CALLS` static, `recursive_position_unit_calls`, `TrapCallerProtocol
//! Mutation`, and `set_trap_caller_protocol_mutation` each had a RETAINED
//! caller (`lower_expr`, `lower_declaration_ref`, `merge_scalar_operand`,
//! `dispatch_fusion_owned_outer_realization`, `claim_and_call_resolved_
//! continuation_inner`, `dispatch_fused_consuming_call`, `carried_join_arm`,
//! `construct_static_worker_binding`, or the test-glob chain reaching
//! `core/tests/control.rs`) that was previously reached only because the
//! callee sat in an ANCESTOR module (`lowering` or `lowering::core`); as a
//! SIBLING module the same reachable set requires the visibility spelled out
//! rather than inherited. The reachable population is unchanged from before
//! the move -- this is the ordinary sibling-module recompute item 11/12 also
//! needed, not new scaffolding, so it carries no `AC-5` ledger entry.

use super::*;

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TrapCallerProtocolMutation {
    Exact,
    LeaveStaleTrap,
    ReadResultBeforeTrap,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D5CloseoutMutation {
    Exact,
    /// Emit the lawful call, then suppress its ledger entry.
    SuppressLedgerEntry,
    /// Record the entry twice under one template.
    DuplicateLedgerEntry,
    /// Record an entry under a template the plan never issued.
    ExtraLedgerEntry,
    /// Record a callee that is not the one the instruction actually calls.
    SubstituteEmittedCallee,
}

#[cfg(test)]
thread_local! {
    pub(super) static RECURSIVE_POSITION_UNIT_CALLS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

#[cfg(test)]
thread_local! {
    static TRAP_CALLER_PROTOCOL_MUTATION: std::cell::Cell<TrapCallerProtocolMutation> =
        const { std::cell::Cell::new(TrapCallerProtocolMutation::Exact) };
    /// **`RT-DECL-CLOSURE-PORT` `D5` — every declaration-owned unit call this
    /// thread actually emitted**, as `(reference occurrence, target origin,
    /// emitted callee)`.
    ///
    /// ⛔ Appended at the emission site from the emitted `Inst` itself. Its
    /// point is to be an authority *independent of* `declaration_calls`, so a
    /// control can compare the planner-resolved target against what was really
    /// called. ⚠ It accumulates across a thread, so read it through
    /// [`d5_emitted_declaration_calls`] after
    /// [`reset_d5_emitted_declaration_calls`] — a bare read attributes an
    /// earlier compile's calls to the current one.
    /// **`RT-DECL-CLOSURE-PORT` `D5`** — the causal controls on the checked-call
    /// closeout. Each defeats exactly one of the three things the closeout
    /// claims: that every lawful emission is recorded, that no template records
    /// twice, and that the recorded callee is the one actually emitted.
    static D5_CLOSEOUT_MUTATION: std::cell::Cell<D5CloseoutMutation> =
        const { std::cell::Cell::new(D5CloseoutMutation::Exact) };
    static D5_EMITTED_DECLARATION_CALLS: std::cell::RefCell<
        Vec<(StaticOriginId, StaticOriginId, cranelift_codegen::ir::FuncRef)>,
    > = const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(super) fn recursive_position_unit_calls() -> usize {
    RECURSIVE_POSITION_UNIT_CALLS.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn with_d5_closeout_mutation<T>(
    mutation: D5CloseoutMutation,
    body: impl FnOnce() -> T,
) -> T {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            D5_CLOSEOUT_MUTATION.with(|cell| cell.set(D5CloseoutMutation::Exact));
        }
    }
    D5_CLOSEOUT_MUTATION.with(|cell| cell.set(mutation));
    let _restore = Restore;
    body()
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d5_emitted_declaration_calls() {
    D5_EMITTED_DECLARATION_CALLS.with(|calls| calls.borrow_mut().clear());
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d5_emitted_declaration_calls()
-> Vec<(StaticOriginId, StaticOriginId, cranelift_codegen::ir::FuncRef)> {
    D5_EMITTED_DECLARATION_CALLS.with(|calls| calls.borrow().clone())
}

#[cfg(test)]
pub(super) fn set_trap_caller_protocol_mutation(mutation: TrapCallerProtocolMutation) {
    TRAP_CALLER_PROTOCOL_MUTATION.with(|cell| cell.set(mutation));
}

impl<'a> Lowering<'a> {
        pub(super) fn call_static_worker(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            worker: &StaticWorkerBinding,
            args: &[RuntimeExpr],
            static_origin: StaticOriginId,
            env: &[LoweringEnvironmentBinding],
        ) -> Result<LoweringOperand, CraneliftBackendError> {
            // Explicit arguments in source order: argument `i` is child `1 + i` of
            // the `Call` occurrence, the callee being child `0`.
            //
            // ⛔ **This is the direct descent's argument phase and only its own.**
            // `D8e`'s source-machine consumer evaluates its arguments under the
            // machine's control and phase instead, and enters at
            // [`Self::call_static_worker_with_inputs`] below. The two share every
            // line after this point, which is the whole reason the split is here and
            // not lower: captures, the route's suffix, the route's table and the
            // emitted call are one assembly with one owner.
            // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the VISITED argument origins,
            // recorded as this descent walks them rather than re-derived later.
            //
            // The fusion seam below compares them against the claim's ordered
            // parameter projection. Recomputing the children there would be a source
            // walk in lowering, which `evt_5edhqyyhw4585` forbids; recording what
            // was actually visited is a different fact and is the one the equality
            // is about.
            let mut visited = Vec::with_capacity(args.len());
            let inputs = args
                .iter()
                .enumerate()
                .map(|(position, argument)| {
                    let argument = self.child_occurrence(static_origin, 1 + position, argument)?;
                    visited.push(argument.static_origin);
                    self.lower_expr(builder, argument, env)
                })
                .collect::<Result<Vec<_>, _>>()?;
            self.call_static_worker_with_inputs(builder, worker, inputs, static_origin, Some(&visited))
                // `D8j` — direct descent DISCARDS the emission handle. It is not a
                // composed consumption and has no causal obligation to answer for;
                // dropping the handle here is that statement, made where the
                // decision belongs.
                .map(StaticWorkerCallOutcome::into_operand)
        }
}

impl<'a> Lowering<'a> {
        /// **The route-selected static-worker emitter, from evaluated arguments
        /// onward.** Shared verbatim by the direct descent and by `D8e`'s
        /// source-machine consumer; neither reassembles any part of it.
        pub(super) fn call_static_worker_with_inputs(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            worker: &StaticWorkerBinding,
            mut inputs: Vec<LoweringOperand>,
            static_origin: StaticOriginId,
            // `D3` — the argument origins this call's own descent VISITED, in source
            // order. `None` from a consumer that evaluates arguments under another
            // control and cannot report them; the fusion seam then refuses rather
            // than skipping its equality.
            visited_arguments: Option<&[StaticOriginId]>,
        ) -> Result<StaticWorkerCallOutcome, CraneliftBackendError> {
            // ---- `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — THE FUSED INVOCATION, at
            // ---- the exact checked consuming call. Architect `evt_5edhqyyhw4585`.
            //
            // Seated at the shared seam both static-worker consumers pass through,
            // not at the direct descent above. This node has paid four times for an
            // instrument placed at one consumer while another reached the same
            // machinery by a different route.
            //
            // ⛔ Selected ONLY by `claim.consuming_call == static_origin`. Every
            // other equality below is a closure check after selection.
            if let Some(realized) = self.dispatch_fused_consuming_call(
                builder,
                worker,
                &inputs,
                static_origin,
                visited_arguments,
            )? {
                return Ok(realized);
            }
            // `D2b` OBSERVATION ONLY, at the seam BOTH consumers share -- the direct
            // descent and the source-machine consumer. Recording at one caller
            // would miss whichever path a fixture happens to take, which is exactly
            // how a "no worker call" reading can be an instrument gap rather than a
            // fact about the program.
            #[cfg(test)]
            crate::cranelift_backend::lowering::lrc_d2b_record_worker_call(static_origin);
            // ⛔ Arity is checked HERE, not in either caller. The explicit-argument
            // run is exactly `inputs` at entry -- captures are appended below -- so
            // this is the one place both consumers can be held to the declared
            // arity without either restating it.
            let supplied = u32::try_from(inputs.len())
                .map_err(|_| unsupported("Call", "call argument count exceeds addressable range"))?;
            if supplied != worker.declared_arity {
                return Err(unsupported(
                    "Call",
                    format!(
                        "static worker expects {} arguments but call provides {supplied}",
                        worker.declared_arity
                    ),
                ));
            }

            // Stored captures follow, unchanged. A carried capture stays carried:
            // there is deliberately no conversion on this edge, which is the whole
            // reason the binding holds operands rather than templates.
            inputs.extend(worker.captures.iter().cloned());

            // `RT-DECL-CLOSURE-PORT` `D5a` -- the generated-context capture suffix.
            //
            // ⭐ The prefix above is untouched, which is the point: the raw worker's
            // operand run is exactly what it always was, and a context call is that
            // run plus the enclosing specialization's continuation inputs. That is
            // why "keep raw fn2's ABI unchanged" and "carry the inputs across the
            // worker execution" are not in tension -- one is a prefix of the other.
            //
            // ⛔ Guarded on the exact body origin. A suffix appended to some other
            // worker call would be an arity error against a frame that might be
            // large enough to absorb it silently.
            // `D5a` checkpoint 4 step 3 -- the raw run, captured before the suffix
            // so the two are separable in the event log below. ⭐ "No suffix" and
            // "a suffix of length zero" are different facts and one total cannot
            // tell them apart.
            #[cfg(test)]
            let raw_operands = inputs.len();
            // ⭐ **`RT-CONTSRC-PRODUCER-LOCAL` `D6b` — THE ROUTE IS CONSUMED HERE,
            // and it decides BOTH halves of the call: the operand run and the
            // callee.** The two must be decided by one fact, because they are one
            // contract -- a generated context's ABI has a capture run the raw body
            // does not, so appending the suffix to a raw target, or omitting it from
            // a context target, is an arity disagreement against a frame that may be
            // large enough to absorb it silently.
            //
            // ⛔ The pre-`D6b` reading -- "append iff
            // `generated_context_captures.worker_body_origin == worker.body_origin`"
            // -- is *blind by construction* now that `D6a` binds two workers over
            // one body origin. It answers the same for the induction hypothesis and
            // for the selected recursive argument, which are exactly the two the
            // suffix must separate. The body-origin equality is retained below, but
            // demoted to a consistency check on the route's own choice.
            match worker.route {
                StaticWorkerCallRoute::GeneratedContext => {
                    // The enclosing specialization's continuation inputs, carried
                    // across this worker execution. Absent is a hard stop: a
                    // context-routed call with no suffix to append means the
                    // binding named a context this frame never stashed operands
                    // for, and calling it would underflow the context's ABI.
                    let extra = self
                        .function_local
                        .generated_context_captures
                        .as_ref()
                        .ok_or_else(|| {
                            unsupported(
                                "Call",
                                format!(
                                    "a static worker binding for body origin {:?} is routed to a \
                                     generated context, but this frame stashed no continuation-input \
                                     suffix for any body",
                                    worker.body_origin
                                ),
                            )
                        })?;
                    if extra.worker_body_origin != worker.body_origin {
                        return Err(unsupported(
                            "Call",
                            format!(
                                "a context-routed static worker binding names body origin {:?}, but \
                                 the stashed continuation-input suffix belongs to body {:?}",
                                worker.body_origin, extra.worker_body_origin
                            ),
                        ));
                    }
                    // `D8g` — WITHHOLD THE SUFFIX, under test only. The sole
                    // producer of the generated-context capture run is withheld and
                    // the raw run and the call itself stay exact, so the difference
                    // this isolates is the suffix and nothing else.
                    #[cfg(test)]
                    let withhold = crate::cranelift_backend::lowering::d8g_mutation()
                        == crate::cranelift_backend::lowering::D8gMutation::WithholdContextSuffix;
                    #[cfg(not(test))]
                    let withhold = false;
                    if withhold {
                        #[cfg(test)]
                        crate::cranelift_backend::lowering::record_d8g_mutation_application();
                    } else {
                        inputs.extend(extra.operands.iter().cloned());
                    }
                }
                StaticWorkerCallRoute::RawWorker => {
                    // ⛔ Appends NOTHING, unconditionally -- not even when this
                    // frame does hold a suffix for this very body origin, which is
                    // precisely the `D6a` case where the induction hypothesis
                    // beside this binding is context-routed over the same body.
                    // "No suffix" and "a suffix of length zero" stay different
                    // facts, and this arm is the first.
                }
            }

            // Resolve the exact target FIRST. The mutation below perturbs only what
            // the consumer is handed, never what the binding named.
            //
            // `D6b` -- from the route's OWN table. In a retargeted specialization
            // `worker_calls[body]` is the generated context and the raw callee
            // survives only in `raw_worker_calls`; taking whichever entry exists is
            // the inference `D6a` made impossible.
            // `D8g` — WRONG TABLE, under test only. Each route reads the other's
            // table. The call key, the binding, the route field and the operand run
            // are all untouched; only which table answers moves, which is the one
            // producer input the table-choice relation is about.
            #[cfg(test)]
            let swap_tables = if crate::cranelift_backend::lowering::d8g_mutation()
                == crate::cranelift_backend::lowering::D8gMutation::WrongTable
            {
                crate::cranelift_backend::lowering::record_d8g_mutation_application();
                true
            } else {
                false
            };
            #[cfg(not(test))]
            let swap_tables = false;
            let (table, table_name) = match (worker.route, swap_tables) {
                (StaticWorkerCallRoute::GeneratedContext, false)
                | (StaticWorkerCallRoute::RawWorker, true) => {
                    (&self.function_local.worker_calls, "worker_calls")
                }
                (StaticWorkerCallRoute::RawWorker, false)
                | (StaticWorkerCallRoute::GeneratedContext, true) => {
                    (&self.function_local.raw_worker_calls, "raw_worker_calls")
                }
            };
            let exact = table.get(&worker.body_origin).cloned().ok_or_else(|| {
                unsupported(
                    "Call",
                    format!(
                        "no {table_name} target for body origin {:?} was declared into this \
                             function, so the {:?} route has no callee. ⛔ This never falls back to \
                             the other route's table: a raw call answered by a generated context \
                             would underflow that context's capture run, and a context call answered \
                             by the raw body would drop the continuation inputs the context exists \
                             to carry",
                        worker.body_origin, worker.route
                    ),
                )
            })?;
            // `D8g` — the emitted callee identity, captured where the table
            // answered. The FuncRef, not the origin: the two routes share a worker
            // body origin by design, so only this separates them.
            #[cfg(test)]
            let emitted_callee = {
                use cranelift_codegen::entity::EntityRef;
                exact.function.index() as u32
            };

            // `AC-5` clause (b): the redirect selects a **distinct** target by
            // `AC-6`'s definition of same-shape -- **same declared arity and same
            // capture count** -- and by nothing else.
            //
            // Selecting on `origin != body_origin` would establish only
            // difference. The target map is populated from every projected
            // emittable unit, so over a heterogeneous fixture that picks an
            // arbitrary unrelated unit and the red proves nothing about the origin
            // check. Requiring full header/slot/offset equality is the opposite
            // error: it over-constrains past the shape `AC-6` actually names.
            //
            // No candidate is a loud failure, never a fall back to exact: a silent
            // fallback would make this control vacuously green.
            // `D6b` -- the redirect searches the SAME table the exact resolution
            // used. Searching `worker_calls` while the exact answer came from
            // `raw_worker_calls` would make the mutation a route swap rather than
            // the same-shape redirect `AC-5` names, and the resulting red would be
            // attributable to the wrong mechanism.
            #[cfg(test)]
            let target = if STATIC_WORKER_MUTATION.with(std::cell::Cell::get)
                == StaticWorkerMutation::RedirectResolvedWorkerTarget
            {
                table
                    .iter()
                    .find(|(origin, call)| {
                        **origin != worker.body_origin
                            && call.header.parameters == exact.header.parameters
                            && call.header.captures == exact.header.captures
                    })
                    .map(|(_, call)| call.clone())
                    .ok_or_else(|| {
                        unsupported(
                            "StaticWorkerMutation",
                            "the AC-5 redirect found no DISTINCT target of the same declared arity \
                             and capture count; it never falls back to exact, because a fallback \
                             would make this control vacuously green. Clause (c): run this switch \
                             on the two-same-shape-worker program only",
                        )
                    })?
            } else {
                exact
            };
            #[cfg(not(test))]
            let target = exact;

            if target.origin != worker.body_origin {
                return Err(unsupported(
                    "Call",
                    format!(
                        "worker call target carries origin {:?} but the binding names body origin \
                         {:?}",
                        target.origin, worker.body_origin
                    ),
                ));
            }

            let emitted = self.call_declared_unit_target(
                builder,
                target,
                &inputs,
                #[cfg(test)]
                None,
            )?;
            // `D5a` -- the emission half of the marker's ordered log. ⛔ Recorded
            // AFTER the instruction exists, so "consumed before emitted" is a fact
            // about the log's order rather than about where the line was written.
            #[cfg(test)]
            record_d5a_marker_event(D5aMarkerEvent::WorkerCallEmitted {
                body_origin: worker.body_origin,
                raw_operands,
                supplied_operands: inputs.len(),
                route: worker.route,
            });
            // `D8g` — the same instant, recorded with the facts a keyed relation
            // needs: which body is emitting, which call occurrence, the decoded raw
            // callee, its declared contract, the route that chose the table, the run
            // the instruction carried, and whether the binding answers for a
            // composed causal obligation.
            //
            // ⛔ This is the ONE emitter the functionized and composed populations
            // share. Recording here is what lets `D8g` relate two different programs
            // at one seat rather than compare two logs that merely look alike.
            #[cfg(test)]
            crate::cranelift_backend::lowering::record_d8g_emission(
                crate::cranelift_backend::lowering::D8gEmission {
                    function: self.defining_function_id,
                    call_origin: static_origin,
                    target_body_origin: worker.body_origin,
                    declared_arity: worker.declared_arity,
                    captures: worker.captures.len(),
                    route: worker.route,
                    raw_operands,
                    supplied_operands: inputs.len(),
                    composed_discharge: worker.composed_continuation_authority().is_ok(),
                    emitted_callee,
                },
            );
            // `D8j` — the instruction is HANDED BACK, not recorded. Which consumer
            // may answer for a causal obligation with it is the caller's question,
            // and both this emitter's callers reach here.
            // ⭐ The operand run is reported from the vector that was just written,
            // read adjacent to the call that consumed it. ⛔ Not recomputed from the
            // binding's declared arity and captures: that is the quantity
            // verification 4b compares this against, and deriving both from one
            // source would make the comparison an identity.
            let emission = StaticWorkerEmission {
                inst: emitted.1,
                supplied_operands: inputs.len(),
            };
            // `D8j` verification 4b's discriminator, applied AFTER the real vector
            // was assembled and emitted. ⛔ It moves the EVIDENCE about the run, not
            // the run: the call that was written is unchanged, and every other input
            // to the verifier stays exact.
            #[cfg(test)]
            let emission = if d8j_mutation() == D8jMutation::SupplyOperandCountDisagreesWithTarget {
                StaticWorkerEmission {
                    supplied_operands: emission.supplied_operands.saturating_add(1),
                    ..emission
                }
            } else {
                emission
            };
            Ok(StaticWorkerCallOutcome::Emitted(emitted.0, emission))
        }
}

impl<'a> Lowering<'a> {
        /// **`D7` -- THE PRE-EMISSION CAPTURE-CONTRACT GATE for a retained callable
        /// whose environment is mixed-phase.**
        ///
        /// ⭐⭐ **Membership in `worker_templates` is necessary and INSUFFICIENT, and
        /// that is the whole reason this exists.** The measured stop that opened this
        /// checkpoint had a template for its body origin and still could not be
        /// represented; "the planner knows this closure" answers *whether* a contract
        /// was issued, never *what it says*. A capsule that closes over a carried
        /// word commits the callee to reading that word out of an activation-frame
        /// slot at an exact ordinal, so every one of those facts is checked **before
        /// any function definition or object emission**, not discovered when the
        /// callee loads the wrong offset.
        ///
        /// ⛔ **No policy is invented here.** The expected capture slot is projected
        /// from [`expected_capture_slot`], the same authority that laid the
        /// descriptor, so carrier, ownership, storage owner, width, alignment and
        /// ordinal are compared against the planner's own answer rather than against
        /// a second copy of the rule that could drift from it.
        ///
        /// The one fact this layer adds is **phase admissibility**, which the ABI
        /// plane cannot state because it has no operands: a `Carried` capture is an
        /// invocation-time SSA word, so it is lawful only in a slot the **activation
        /// frame** owns. `ArtifactStatic` material is minted before execution begins
        /// -- a seed capture's lane -- and no word this activation computes can be
        /// that, so a carried capture in a seed slot is refused rather than stored.
        pub(super) fn validate_retained_callable_capture_contract(
            &self,
            closure_origin: StaticOriginId,
            body_origin: StaticOriginId,
            provenance: AbiCaptureProvenance,
            declared_arity: usize,
            captures: &[LoweringOperand],
        ) -> Result<(), CraneliftBackendError> {
            // 1. Exactly one planner-issued template for this exact body.
            //
            // ⭐ Uniqueness is upstream and structural: `worker_templates` is keyed
            // by body origin, and the population walk that fills it already refuses
            // when "two emittable units claim the same body origin". So the only
            // failure this lookup can still see is OMISSION -- which is the half a
            // keyed map cannot make unrepresentable.
            let target = self
                .function_local
                .worker_templates
                .get(&body_origin)
                .ok_or_else(|| {
                    unsupported(
                        "RetainedCallableCaptureContract",
                        format!(
                            "a mixed-phase retained callable at {closure_origin:?} has no planner-issued \
                             worker template for body origin {body_origin:?} in this function"
                        ),
                    )
                })?;

            // 2. The record is keyed by the callable SOURCE BODY, so a disagreement
            //    here is a wrong-body contract rather than a lookup miss.
            if target.call_site_origin != body_origin {
                return Err(unsupported(
                    "RetainedCallableCaptureContract",
                    format!(
                        "the worker template reached for body origin {body_origin:?} is keyed by \
                         source body {:?}",
                        target.call_site_origin
                    ),
                ));
            }

            // 3. Declared arity and capture count, against the descriptor header.
            if target.header.parameters as usize != declared_arity {
                return Err(unsupported(
                    "RetainedCallableCaptureContract",
                    format!(
                        "worker descriptor declares {} parameters but the retained callable declares \
                         {declared_arity}",
                        target.header.parameters
                    ),
                ));
            }
            if target.header.captures as usize != captures.len() {
                return Err(unsupported(
                    "RetainedCallableCaptureContract",
                    format!(
                        "worker descriptor declares {} captures but {} were projected from the \
                         retained definition",
                        target.header.captures,
                        captures.len()
                    ),
                ));
            }

            // 4. The ORDERED capture run, taken in slot order. Its length must agree
            //    with the header independently -- the header and the slot run are two
            //    recorded facts, and a gate that trusted one to speak for the other
            //    would be blind to exactly the descriptor it exists to reject.
            let capture_slots = target
                .slots
                .iter()
                .filter(|slot| slot.kind == AbiSlotKind::Capture)
                .collect::<Vec<_>>();
            if capture_slots.len() != captures.len() {
                return Err(unsupported(
                    "RetainedCallableCaptureContract",
                    format!(
                        "worker descriptor's slot run declares {} capture slots against a header of \
                         {} and {} projected captures",
                        capture_slots.len(),
                        target.header.captures,
                        captures.len()
                    ),
                ));
            }

            for (position, (slot, capture)) in capture_slots.iter().zip(captures).enumerate() {
                let ordinal = u32::try_from(position).map_err(|_| {
                    unsupported(
                        "RetainedCallableCaptureContract",
                        "retained callable capture count exceeds addressable range",
                    )
                })?;
                // 5. Phase admissibility, then ordinal/provenance/owner/lifetime in
                //    ONE comparison
                //    against the planner's own projection. ⛔ Comparing field by
                //    field here would let a field added to `AbiSlot` later go
                //    unchecked; whole-slot equality cannot.
                //
                //    Ordinal density falls out of this rather than needing its own
                //    pass: slot *i* of the capture run must carry ordinal *i*, so a
                //    duplicated, permuted or gapped ordinal fails here.
                //    ⚠ **The phase check runs FIRST, and the order is load-bearing
                //    rather than stylistic.** Behind the whole-slot equality below
                //    it would be DEAD for the only provenance production calls this
                //    with: equality already forces a lexical capture's slot to be
                //    `ValueWord`, whose storage owner is the activation frame, so no
                //    descriptor could ever reach the phase arm. Asking whether the
                //    refusal could be provoked at all is what surfaced that -- a
                //    guard nothing can trip is not a guard.
                match capture {
                    // A compile-time template is lawful in any capture slot: it is
                    // read where the descriptor says, whatever owns that storage.
                    LoweringOperand::Specialized(_) => {}
                    // ⛔ An invocation-time word cannot inhabit storage minted
                    // before execution began, nor the persistent store.
                    LoweringOperand::Carried(_) => {
                        if slot.storage_owner != AbiStorageOwner::ActivationFrame {
                            return Err(unsupported(
                                "RetainedCallableCaptureContract",
                                format!(
                                    "capture {position} of the retained callable at \
                                     {closure_origin:?} arrived carried, but its slot's storage is \
                                     owned by {:?} -- an invocation-time word cannot inhabit storage \
                                     that outlives the activation that computes it",
                                    slot.storage_owner
                                ),
                            ));
                        }
                    }
                }
                let expected = expected_capture_slot(provenance, ordinal);
                if **slot != expected {
                    return Err(unsupported(
                        "RetainedCallableCaptureContract",
                        format!(
                            "capture {position} of the retained callable at {closure_origin:?} \
                             declares slot {slot:?} but its {provenance:?} provenance projects \
                             {expected:?}"
                        ),
                    ));
                }
            }
            Ok(())
        }
}

impl<'a> Lowering<'a> {
        pub(super) fn call_declared_recursive_position_unit(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            body_origin: StaticOriginId,
            inputs: &[LoweringOperand],
            coordinates: Option<CarriedInvocationCoordinates>,
        ) -> Result<LoweringOperand, CraneliftBackendError> {
            // `RT-DECL-CLOSURE-PORT` `D5a` checkpoint 4 step 1 — THE CARRIED
            // INVOCATION BINDING.
            //
            // The binding is resolved from the invocation's retained source
            // coordinates through planner authority. ⛔ Lowering supplies only the
            // coordinates it already holds; it does not reconstruct the binding
            // from `body_origin`, from the callee's ABI shape, from the existence
            // of a context, or by taking a first match — the planner rejects an
            // ambiguous resolution rather than choosing.
            // `D5a` checkpoint 4 step 3 -- perturb the RETAINED SOURCE COORDINATES
            // the invocation presents, so the lookup is asked for a coordinate the
            // planner never issued. ⛔ The lookup is untouched: what moves is the
            // key, which is the thing the ruling says the binding must be resolved
            // by.
            #[cfg(test)]
            let coordinates = coordinates.map(|mut coordinates| {
                if d5a_route_mutation() == D5aRouteMutation::PerturbCarriedInvocationCoordinates {
                    record_d5a_route_application();
                    coordinates.recursive_position = coordinates.recursive_position.wrapping_add(1);
                }
                coordinates
            });
            let context = match coordinates {
                Some(coordinates) => self.static_transition_plan.carried_invocation_context(
                    coordinates.continuation_origin,
                    coordinates.recursive_position,
                    body_origin,
                )?,
                None => None,
            };
            // ⛔ FAIL CLOSED, over BOTH ways of arriving at "no context".
            //
            // A site that cannot supply the coordinates cannot be *asked* whether
            // it should retarget; a site whose coordinates resolve to nothing was
            // asked and got no answer. Either way, emitting the raw call is a miss
            // that looks exactly like a lawful ordinary call. It is only safe when
            // no context exists for this body at all.
            //
            // ⭐⭐ The second half is new. This test used to guard the missing-
            // coordinates arm ALONE, so perturbed-but-present coordinates fell
            // straight through to the raw target. On this witness that still
            // refused — but only incidentally, because the superseded body has no
            // `Function` left to call. Had it remained executable, the retarget
            // would have been dropped in silence. ⇒ The guard belongs to the
            // outcome, not to one of the two routes into it.
            if context.is_none()
                && self
                    .static_transition_plan
                    .continuation_contexts()?
                    .iter()
                    .any(|context| context.worker_body_origin() == body_origin)
            {
                return Err(unsupported(
                    "ContinuationSpecialization",
                    format!(
                        "a carried recursive-position invocation of body {body_origin:?} resolved no \
                         generated execution context, and that body has one; emitting the raw target \
                         here would drop the retarget silently rather than refuse it. Retained source \
                         coordinates presented: {:?}",
                        coordinates.map(|c| (c.continuation_origin, c.recursive_position)),
                    ),
                ));
            }
            // `RT-RECURSOR-TRANSPORT` `D2` trace: the CONTEXT POPULATION this
            // resolution was answered from, so "raw target" can be read as "no
            // context exists for this body" rather than merely "none was returned".
            #[cfg(test)]
            d5a_trace(format!(
                "  RT-D2 CONTEXT-POPULATION body={body_origin:?} contexts={:?}",
                self.static_transition_plan
                    .continuation_contexts()?
                    .iter()
                    .map(|context| (
                        context.id(),
                        context.worker_body_origin(),
                        context.enclosing_specialization()
                    ))
                    .collect::<Vec<_>>()
            ));
            #[cfg(test)]
            d5a_trace(format!(
                "  CARRIED-INVOCATION body={body_origin:?} coords={:?} -> {}",
                coordinates.map(|c| (c.continuation_origin, c.recursive_position)),
                match context {
                    Some(context) => format!("context {context:?}"),
                    None => "raw target".to_string(),
                }
            ));
            let result = match context {
                Some(context) => self.call_declared_context(builder, context, body_origin, inputs)?,
                None => self.call_declared_unit(
                    builder,
                    body_origin,
                    inputs,
                    #[cfg(test)]
                    None,
                )?,
            };
            #[cfg(test)]
            RECURSIVE_POSITION_UNIT_CALLS.with(|calls| calls.set(calls.get() + 1));
            Ok(result)
        }
}

impl<'a> Lowering<'a> {
        /// Emit the one exact retargeted callee for a carried invocation.
        ///
        /// ⭐ Only the **callee** moves. The call is the same already-planned
        /// emitted call, at the same site, with the same operand prefix and the
        /// same causal ancestry; its source edge, its predecessor and its
        /// provenance are untouched. That is what makes this a retarget rather than
        /// a deletion.
        fn call_declared_context(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            context: ContinuationContextId,
            body_origin: StaticOriginId,
            inputs: &[LoweringOperand],
        ) -> Result<LoweringOperand, CraneliftBackendError> {
            let target = self
                .function_local
                .context_calls
                .get(&context)
                .cloned()
                .ok_or_else(|| {
                    unsupported(
                        "ContinuationSpecialization",
                        format!(
                            "the generated execution context bound to body {body_origin:?} was not                          declared into this function"
                        ),
                    )
                })?;
            // `D5a` checkpoint 4 step 1b — THE EXACT CAPTURE SUFFIX.
            //
            // The context declares a frame of `parameters + captures`; the carried
            // invocation supplies the parameter run, so retargeting its callee
            // without appending the captures is a call that does not match the
            // frame it now names. ⛔ Appended in the context's DECLARED ORDER, and
            // taken only from the immediate slots the planner assigned -- nothing
            // is reconstructed from the raw worker, chosen by shape, or routed
            // through a runtime transport.
            let view = self
                .static_transition_plan
                .continuation_contexts()?
                .into_iter()
                .find(|candidate| candidate.id() == context)
                .ok_or_else(|| {
                    unsupported(
                        "ContinuationSpecialization",
                        "the bound generated context has no projected view",
                    )
                })?;
            let defining_owner = self.defining_emission_owner.ok_or_else(|| {
                unsupported(
                    "ContinuationSpecialization",
                    "a carried invocation retarget was reached with no emission owner bound for the                  context currently being defined",
                )
            })?;
            let mut inputs = inputs.to_vec();
            for capture in view.captures()? {
                // `RT-CONTSRC-PRODUCER-LOCAL` `D1` — present a producer-local
                // coordinate to this seam, so its refusal is measured rather than
                // merely written. ⛔ Applied BEFORE the domain match, because the
                // question is what the match does with a domain it cannot locate.
                #[cfg(test)]
                let capture = {
                    let mut capture = capture;
                    if d5a_route_mutation() == D5aRouteMutation::PresentProducerLocalCoordinate {
                        record_d5a_route_application();
                        capture.coordinate = ContinuationSourceCoordinate::producer_local_probe();
                    }
                    capture
                };
                // `RT-CONTSRC-PRODUCER-LOCAL` `D1` `D3` consumer 3 of 3, context
                // half. ⛔ Exhaustive over the coordinate domains with no default,
                // for the same reason as the specialization seam: this reads an ABI
                // operand run, which a producer-local value is not in.
                // `RT-CONTSRC-PRODUCER-LOCAL` `D3b` — the same pairing resolver as
                // the emission seam, told what THIS seam is holding.
                //
                // ⛔ `ContinuationImmediateSeat::AbiOperandRun` is not a weaker
                // `Emission`: this seam holds an ABI operand run and no semantic
                // environment at all, so a current-lexical nearest-alias index has
                // nothing here to index and is refused rather than read as an ABI
                // position. The generated-context capture arm IS resolvable here,
                // because a capture slot is a position in exactly this run.
                let immediate_slot = self.resolve_context_capture_claim(
                    capture.coordinate,
                    capture.availability,
                    defining_owner,
                )?;
                // `RT-CONTSRC-PRODUCER-LOCAL` `D2b` — the availability domain, matched
                // exhaustively with no wildcard exactly as the coordinate domain is
                // above. `D3` teaches this seam the two producer-local arms; until
                // then it must not index `defining_abi_operands` — an ABI operand run
                // — with a lexical environment index.
                let operand = self
                    .function_local
                    .defining_abi_operands
                    .get(immediate_slot as usize)
                    .ok_or_else(|| {
                        unsupported(
                            "ContinuationSpecialization",
                            format!(
                                "a generated context capture names immediate slot {} outside the                              emitting function's {} ABI operands; note this is the IMMEDIATE                              slot, whose meaning is fixed by the availability domain {:?}",
                                immediate_slot,
                                self.function_local.defining_abi_operands.len(),
                                capture.availability,
                            ),
                        )
                    })?
                    .clone();
                inputs.push(operand);
            }
            self.call_declared_unit_target(
                builder,
                target,
                &inputs,
                #[cfg(test)]
                None,
            )
            .map(|(operand, _inst)| operand)
        }
}

impl<'a> Lowering<'a> {
        /// **`RT-DECL-CLOSURE-PORT` `D4` — the complete call to a declaration-owned
        /// callable unit, and the SOLE place its input order is decided.**
        ///
        /// Every `Call` consumer that can reach a [`Lowered::DeclarationClosure`]
        /// routes through here, because the input slice is the one thing they must
        /// not each remember for themselves: the callee descriptor declares its
        /// `Parameter` slots and then its `Capture` slots, so
        ///
        /// ```text
        /// inputs = actual arguments in PARAMETER order ++ retained captures in D3 order
        /// ```
        ///
        /// is a property of the ABI, not of any one call site. ⛔ A consumer that
        /// assembled its own slice would be a second ordering authority, and a
        /// swapped one still type-checks — every input is a word.
        ///
        /// ⚠ `args` arrive already lowered because the three consumers lower them
        /// differently (ordinary child occurrences, producer-env child occurrences,
        /// source-machine operands). What they must NOT differ on is what happens
        /// afterwards.
        pub(super) fn call_declaration_closure_unit(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            reference: StaticOriginId,
            symbol: &RuntimeSymbol,
            params: &[String],
            captures: Vec<LoweringOperand>,
            args: Vec<LoweringOperand>,
        ) -> Result<LoweringOperand, CraneliftBackendError> {
            // ⛔⛔ **The pending checked-recursion marker is CONSUMED here.**
            //
            // ⚠ Every consumer this route diverts from reached
            // `lower_recursive_declaration_call`, which consumes the marker on
            // entry. A branch that returns before it would leave the marker set —
            // to be picked up by whatever call came next, which
            // `consume_checked_recursive_invocation_call` would then report as a
            // marker "transplanted to another callee". That is a silent
            // mis-attribution, not a missing feature. ⇒ Taking it here is not
            // optional bookkeeping; it is what keeps the diversion from corrupting
            // an unrelated call site.
            //
            // `D5` replaces `D4`'s conservative refusal of a present marker with
            // the split-domain validation below. The marker remains **validation
            // authority only**: nothing it carries becomes a runtime operand, an
            // identity word, a selector, a capsule, a capture, an alternate ABI, a
            // copied body, or a fallback.
            let checked = self.consume_checked_recursive_invocation_call(symbol)?;
            // The declared arity, checked against the call before anything is
            // emitted. The descriptor rejects a mismatched slice too, but it can
            // only say "the frame is missing an input"; this says which call.
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
            self.validate_declaration_unit_call(
                reference,
                symbol,
                checked,
                params.len(),
                captures.len(),
            )?;
            // ⭐ The consumed template id is carried from the validation that
            // accepted it to the emission that realizes it — one value, one path.
            // ⛔ Re-deriving it at the emission site would be a second authority on
            // "which template is this call", and the two could disagree silently.
            let checked_template = match checked.map(|instance| instance.source) {
                Some(InvocationTemplateRef::SameSccCall(id)) => Some(id),
                // `validate_declaration_unit_call` already refused every other
                // source, so this arm is unreachable-by-validation rather than a
                // fallback. Spelled, not wildcarded.
                Some(InvocationTemplateRef::ComputationalIHCall(_)) => None,
                None => None,
            };
            let mut inputs = args;
            // `D7` — the captures are already phase-bearing: a carried capture
            // passes as its existing word and a specialized one is unchanged. The
            // former `map(Specialized)` here asserted a phase the edge now owns.
            inputs.extend(captures);
            self.call_declared_declaration_unit(builder, reference, &inputs, checked_template)
        }
}

impl<'a> Lowering<'a> {
        /// **`RT-DECL-CLOSURE-PORT` `D5` — the split-domain validation that stands
        /// between a call occurrence and the declaration-owned unit it reaches.**
        ///
        /// Two authorities are joined on **one exact occurrence and one exact
        /// target**, and neither is derived from the other:
        ///
        /// 1. **the checked-plan domain** validates the facts it actually owns —
        ///    same-SCC identity, admission, application arity, and the canonical
        ///    local telescope;
        /// 2. **the identity join** binds the consumed checked occurrence to the
        ///    planner-issued `DeclarationCall` reference: exact callee symbol,
        ///    exact application arity, exact resolved `CallableDeclaration` target.
        ///    These three equalities are the *only* lawful shared facts between the
        ///    two domains;
        /// 3. **the ABI domain** validates that same target's `D3`-established
        ///    `Parameter ++ Capture` descriptor run and `D4` input order.
        ///
        /// ⛔⛔ **There is no checked-interface → ABI projection, and D5 must not
        /// invent one** (Architect ruling, 2026-08-04). `CheckedAnswerInterfaceV1`
        /// is canonical *semantic* bytes plus a fingerprint of those bytes: it
        /// encodes no [`AbiCarrier`], no [`AbiOwnership`], no [`AbiStorageOwner`],
        /// no slot ordinal and no capture provenance. So owner and phase are read
        /// where they actually live — on the resolved descriptor — and never parsed
        /// out of canonical bytes, surrogated by a fingerprint comparison, or
        /// assigned by telescope position.
        ///
        /// ⛔ **The ABI half compares two independently produced copies**, which is
        /// the whole reason it is not tautological. `resolve_call_edges` copies the
        /// already-validated unit header/slots/offsets into the function-local
        /// [`units::DeclaredUnitCall`] record; the plan's own descriptor is
        /// immutable and was independently checked by `D3` against the caller-side
        /// `StaticBody` signature. Reading `slot.carrier` from the declared record
        /// and then deriving that same record's expected owner from it would be a
        /// check with one operand. ⇒ The chain preserved here is
        /// `caller-side D3 signature ↔ validated target descriptor ↔ exact declared
        /// call record`.
        ///
        /// ⚠ **Axes deliberately NOT re-checked here, because an earlier authority
        /// already fails closed on them.** `OrientedSubcontinuationPlanV1::validate`
        /// runs on the compile path (`planning.rs`, inside
        /// `oriented_subcontinuation_plan_for_program`) and there establishes, for
        /// every recursive-call template: the callee segment site and the exact
        /// callee frame-template set against the plan's own frames; the composition
        /// of the last callee frame's output interface with `result_interface` and
        /// `caller_interface`; and `occurrence_binding_fingerprint` over **every**
        /// field of the template. `planning.rs` additionally reconciles the exact
        /// per-declaration marker locations, which is what binds `declaration` to
        /// the body the marker actually sits in. Restating those here would put a
        /// second copy of each law in a file where the two can disagree, and would
        /// let a control mis-attribute an upstream refusal to `D5`.
        fn validate_declaration_unit_call(
            &self,
            reference: StaticOriginId,
            symbol: &RuntimeSymbol,
            checked: Option<CheckedRecursiveInvocationInstance>,
            params: usize,
            captures: usize,
        ) -> Result<(), CraneliftBackendError> {
            // ── The identity join, half one: the planner's resolved target class.
            //
            // ⛔ Every call through this route is a call to a declaration-OWNED
            // unit, so a planner that resolved this same reference to the
            // declaration's zero-arity scheduling entry disagrees with the lowering
            // about what the declaration is. That fails closed rather than emitting
            // a call whose arity the descriptor would then have to reject.
            let class = self
                .static_transition_plan
                .declaration_call_target_class(reference)
                .ok_or_else(|| {
                    backend(BackendFailure::PlannerInvariant(format!(
                        "declaration reference to {symbol} has no planned call target class"
                    )))
                })?;
            if class != DeclarationCallTargetClass::CallableDeclaration {
                return Err(backend(BackendFailure::PlannerInvariant(format!(
                    "declaration {symbol} lowers as a declaration-owned unit call but its planned \
                     call targets the declaration's scheduling entry"
                ))));
            }
            let declared = self
                .function_local
                .declaration_calls
                .get(&reference)
                .ok_or_else(|| {
                    backend_module(
                        "DeclarationRef has no planner-derived declaration call target".to_string(),
                    )
                })?;
            // ── The ABI domain, on that exact target.
            let mut found = None;
            for unit in self.static_transition_plan.emittable_units()? {
                // `declared.origin` is the callee's SCHEDULING ENTRY by its own
                // contract, filled from `edge.callee_origin()` in
                // `resolve_call_edges`. So the resolution must compare the entry
                // axis. Reading the body axis happened to agree because a
                // callable-declaration unit is seeded on its body node, where the
                // two coincide -- but agreement on the current population is not
                // authority, and a split-axis unit elsewhere in the same plan could
                // false-match a call that names an entry.
                if unit.entry_origin() != declared.origin {
                    continue;
                }
                if found.is_some() {
                    return Err(backend(BackendFailure::PlannerInvariant(
                        "two abi descriptors claim one declaration call target origin".to_string(),
                    )));
                }
                found = Some(unit);
            }
            let unit = found.ok_or_else(|| {
                backend(BackendFailure::PlannerInvariant(
                    "declaration call target has no abi descriptor".to_string(),
                ))
            })?;
            if !matches!(
                unit.definition(),
                AbiUnitDefinition::CallableDeclaration { .. }
            ) {
                return Err(backend(BackendFailure::PlannerInvariant(format!(
                    "declaration call to {symbol} resolves a target that is not a declaration-owned \
                     callable unit"
                ))));
            }
            // The declared call record against the plan's immutable descriptor:
            // header, the whole slot run (carrier, ownership, storage owner,
            // width/alignment, kind and ordinal), and the offsets.
            if unit.header() != declared.header {
                return Err(backend(BackendFailure::PlannerInvariant(
                    "declaration call record header disagrees with its validated descriptor"
                        .to_string(),
                )));
            }
            if unit.slots() != declared.slots.as_slice() {
                return Err(backend(BackendFailure::PlannerInvariant(
                    "declaration call record slot run disagrees with its validated descriptor"
                        .to_string(),
                )));
            }
            let (offsets, frame_bytes) = unit.slot_offsets()?;
            if offsets != declared.offsets || frame_bytes != declared.header.frame_bytes {
                return Err(backend(BackendFailure::PlannerInvariant(
                    "declaration call record offsets disagree with its validated descriptor"
                        .to_string(),
                )));
            }
            // ── The `D4` input order, stated against the descriptor that receives
            // it. `inputs = args in PARAMETER order ++ captures in D3 order` is only
            // meaningful if the descriptor's leading run is exactly that many
            // `Parameter` slots followed by exactly that many `Capture` slots, each
            // densely ordinalled within its own kind-run.
            let mut expected = (0..params)
                .map(|ordinal| (AbiSlotKind::Parameter, ordinal as u32))
                .chain((0..captures).map(|ordinal| (AbiSlotKind::Capture, ordinal as u32)));
            for slot in unit.slots() {
                match slot.kind {
                    AbiSlotKind::Parameter | AbiSlotKind::Capture => {
                        if expected.next() != Some((slot.kind, slot.ordinal)) {
                            return Err(backend(BackendFailure::PlannerInvariant(format!(
                                "declaration call to {symbol} does not match its callable unit's \
                                 parameter-then-capture input run"
                            ))));
                        }
                    }
                    AbiSlotKind::Result
                    | AbiSlotKind::Control
                    | AbiSlotKind::Trap
                    | AbiSlotKind::Store => {}
                }
            }
            if expected.next().is_some() {
                return Err(backend(BackendFailure::PlannerInvariant(format!(
                    "declaration call to {symbol} supplies more inputs than its callable unit declares"
                ))));
            }
            // ── The checked-plan domain. An unchecked call has nothing further to
            // reconcile: it carries no same-SCC obligation, and the ABI half above
            // is unconditional.
            let Some(instance) = checked else {
                return Ok(());
            };
            let InvocationTemplateRef::SameSccCall(call_template_id) = instance.source else {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "declaration-owned unit call received a computational IH invocation",
                ));
            };
            let plan = self.oriented_subcontinuation_plan.as_ref().ok_or_else(|| {
                unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "checked declaration-owned unit call has no checked plan",
                )
            })?;
            let call = plan.recursive_call(call_template_id).ok_or_else(|| {
                unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "checked declaration-owned unit call has no checked template",
                )
            })?;
            // ── The identity join, half two: callee symbol and application arity.
            // These are the lawful shared equalities — the callee the checked plan
            // names is the callee the emitter is about to call, applied to exactly
            // the arity the plan admitted.
            if &call.callee != symbol {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "checked declaration-owned unit call names another callee",
                ));
            }
            if call.arity != params as u64 {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    format!(
                        "checked recursive invocation of {symbol} is admitted at arity {} but the \
                         declaration-owned unit call applies {params}",
                        call.arity
                    ),
                ));
            }
            if call.local_telescope.len() as u64 != call.arity {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "checked recursive invocation's local telescope does not cover its arity",
                ));
            }
            // ── Same-SCC identity and admission.
            //
            // ⭐ **What makes this call *same-SCC* is that its callee is itself a
            // recursive member of the caller's group** — not that the template says
            // so. The independent side is the rest of the plan: some template in
            // this recursion group must be the callee's OWN call template, and the
            // group must be internally consistent in `scc_index` and `admission`.
            //
            // ⚠ For a self-call the witness is this very template, so `recursion_
            // group` alone is not discriminated by a single-declaration fixture;
            // the mutual same-SCC fixture is what closes that. Stated rather than
            // left to be discovered — the rule is identical either way, and there
            // is deliberately no `caller == callee` shortcut.
            let group = plan
                .recursive_calls
                .iter()
                .filter(|other| other.recursion_group == call.recursion_group)
                .collect::<Vec<_>>();
            if !group.iter().any(|other| other.declaration == call.callee) {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "checked recursive invocation's callee is not a recursive member of its own \
                     recursion group",
                ));
            }
            // ⚠ Split, so each axis gets its OWN first refusal. One composite
            // predicate is discharged by either disagreement holding, so it could
            // not tell the `scc_index` mutation from the `admission` one.
            if group.iter().any(|other| other.scc_index != call.scc_index) {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "checked recursion group disagrees about its scc index",
                ));
            }
            if group.iter().any(|other| other.admission != call.admission) {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "checked recursion group disagrees about its admission",
                ));
            }
            if plan.recursive_calls.iter().any(|other| {
                other.scc_index == call.scc_index && other.recursion_group != call.recursion_group
            }) {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "two checked recursion groups claim one scc index",
                ));
            }
            Ok(())
        }
}

impl<'a> Lowering<'a> {
        /// Transfer the terminal value returned by one declared generated unit.
        ///
        /// Process exit constructors are the one result-edge representation that
        /// differs from their nested carrier form: the root consumes a closed
        /// `ImmediateExitStatus`, not a constructor node. Keeping the conversion at
        /// this result surface prevents an ordinary nested exit-shaped constructor
        /// from being mistaken for the process answer.
        pub(super) fn transfer_unit_result_into_carrier(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            origin: StaticOriginId,
            value: &Lowered,
        ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
            #[cfg(test)]
            d5a_trace(format!(
                "  UNIT-RESULT transfer origin={origin:?} value={}",
                lowered_value_kind(value)
            ));
            let process_exit = self.process_object
                && matches!(
                    value,
                    Lowered::Constructor { constructor, .. }
                        if constructor == &self.process_symbols.exit_success
                            || constructor == &self.process_symbols.exit_failure
                );
            if process_exit {
                let status = self.emit_process_exit_status(builder, value.clone());
                self.emit_carrier_immediate(builder, BoundaryTag::ImmediateExitStatus, status)
            } else {
                self.transfer_into_carrier(builder, origin, value)
            }
        }
}

impl<'a> Lowering<'a> {
        /// Select the exact source occurrences evaluated in result position for
        /// the generated unit currently being defined.
        pub(super) fn select_terminal_result_origins(
            &mut self,
            origin: StaticOriginId,
            _expr: &RuntimeExpr,
        ) -> Result<(), CraneliftBackendError> {
            self.function_local.terminal_result_origins = self
                .static_transition_plan
                .source_result_origins_in_owner_subtree(origin)?;
            Ok(())
        }
}

impl<'a> Lowering<'a> {
        pub(super) fn call_declared_unit(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            body_origin: StaticOriginId,
            inputs: &[LoweringOperand],
            #[cfg(test)] launch_ingress: Option<cranelift_codegen::ir::Value>,
        ) -> Result<LoweringOperand, CraneliftBackendError> {
            let target = self
                .function_local
                .unit_calls
                .get(&body_origin)
                .cloned()
                .ok_or_else(|| {
                    backend_module(format!(
                        "retained body {body_origin:?} has no graph-derived call target in this unit"
                    ))
                })?;
            // `RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` — the redirected invocation's
            // EXTRA operands.
            //
            // The fused frame is the producer's parameter run **plus** the
            // consumer's ordered continuation inputs, because the suffix now runs
            // inside the callee and needs the environment it used to have in the
            // caller. The source invocation passes only the producer's run, so the
            // remainder is supplied here — at the one seat that knows the call was
            // redirected.
            //
            // `None` is the ordinary path and is not the same as `Some(vec![])`:
            // the first means this seat has no fused region, the second means it has
            // one whose suffix needs nothing beyond the producer's operands. Both
            // occur, and collapsing them would let a claim's arity failure read as
            // an unfused call.
            match self.fused_redirect_inputs(body_origin)? {
                None => self
                    .call_declared_unit_target(
                        builder,
                        target,
                        inputs,
                        #[cfg(test)]
                        launch_ingress,
                    )
                    .map(|(operand, _inst)| operand),
                Some(continuation_inputs) => {
                    let mut all = inputs.to_vec();
                    all.extend(continuation_inputs);
                    self.call_declared_unit_target(
                        builder,
                        target,
                        &all,
                        #[cfg(test)]
                        launch_ingress,
                    )
                    .map(|(operand, _inst)| operand)
                }
            }
        }
}

impl<'a> Lowering<'a> {
        /// **`RT-DECL-CLOSURE-PORT` `D4` — the call at a `DeclarationRef`, with its
        /// real inputs.**
        ///
        /// ⭐ `inputs` is the caller's ordered slice: the declaration's actual
        /// arguments in **parameter order**, followed by its retained captures in
        /// `D3` order. It is passed straight to the descriptor-driven emission
        /// below, which remains the sole authority for the exact
        /// `Parameter` + `Capture` slot run and rejects a slice that does not match
        /// it in either direction.
        ///
        /// ⛔ Nothing here re-derives the target: no callable identity word, no
        /// runtime lookup, no name parsing. The reference occurrence selects a
        /// record the planner already resolved and the bundle already declared.
        pub(super) fn call_declared_declaration_unit(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            reference_origin: StaticOriginId,
            inputs: &[LoweringOperand],
            checked_template: Option<u64>,
        ) -> Result<LoweringOperand, CraneliftBackendError> {
            let target = self
                .function_local
                .declaration_calls
                .get(&reference_origin)
                .cloned()
                .ok_or_else(|| {
                    backend_module(
                        "DeclarationRef has no planner-derived declaration call target".to_string(),
                    )
                })?;
            let target_origin = target.origin;
            let target_function = target.function;
            let (operand, call) = self.call_declared_unit_target(
                builder,
                target,
                inputs,
                #[cfg(test)]
                None,
            )?;
            // `RT-DECL-CLOSURE-PORT` `D5` — the emitted-target oracle.
            //
            // ⭐ The callee is read back out of the **instruction that was actually
            // emitted**, not out of the declared map a second time. A control that
            // compared two reads of `declaration_calls` would agree with itself
            // whatever the emitter did; this disagrees the moment the emitted call
            // and the planner-resolved target diverge.
            // ⛔⛔ The callee is decoded out of the instruction that was ACTUALLY
            // emitted, never read back out of the declared map. That is what makes
            // the closeout's target comparison a comparison of two independently
            // produced facts.
            let emitted_callee = match builder.func.dfg.insts[call] {
                cranelift_codegen::ir::InstructionData::Call { func_ref, .. } => func_ref,
                _ => {
                    return Err(backend_module(
                        "a declared unit call was not emitted as a direct call instruction".to_string(),
                    ));
                }
            };
            #[cfg(test)]
            D5_EMITTED_DECLARATION_CALLS.with(|calls| {
                calls
                    .borrow_mut()
                    .push((reference_origin, target_origin, emitted_callee))
            });
            // `RT-DECL-CLOSURE-PORT` `D5` — one ledger entry per CHECKED call,
            // keyed by its template and bound to the exact reference occurrence and
            // resolved target. ⚠ An unchecked entry call carries no template id and
            // is deliberately outside this set.
            if let Some(call_template_id) = checked_template {
                #[cfg(test)]
                let mutation = D5_CLOSEOUT_MUTATION.with(std::cell::Cell::get);
                #[cfg(test)]
                if mutation == D5CloseoutMutation::SuppressLedgerEntry {
                    // ⛔ The call itself is already emitted and lawful; only its
                    // record is withheld. That is the whole point — the closeout
                    // must notice a real call that no entry accounts for.
                    return Ok(operand);
                }
                let ledger = self.checked_call_ledger.as_mut().ok_or_else(|| {
                    backend_module(
                        "a checked declaration-unit call was emitted outside the unit bundle pass"
                            .to_string(),
                    )
                })?;
                #[cfg(test)]
                let record = units::CheckedCallRecord {
                    reference: reference_origin,
                    target: target_origin,
                    callee: if mutation == D5CloseoutMutation::SubstituteEmittedCallee {
                        target_function
                    } else {
                        emitted_callee
                    },
                    resolved: if mutation == D5CloseoutMutation::SubstituteEmittedCallee {
                        // A ref this function certainly did not call.
                        builder
                            .func
                            .dfg
                            .ext_funcs
                            .keys()
                            .find(|candidate| *candidate != emitted_callee)
                            .unwrap_or(target_function)
                    } else {
                        target_function
                    },
                };
                #[cfg(not(test))]
                let record = units::CheckedCallRecord {
                    reference: reference_origin,
                    target: target_origin,
                    callee: emitted_callee,
                    resolved: target_function,
                };
                ledger.record_emitted(call_template_id, record)?;
                #[cfg(test)]
                match mutation {
                    D5CloseoutMutation::DuplicateLedgerEntry => {
                        ledger.record_emitted(call_template_id, record)?;
                    }
                    D5CloseoutMutation::ExtraLedgerEntry => {
                        // ⚠ Keyed off the real template so each call site adds a
                        // DISTINCT unplanned entry. A single shared key would trip
                        // the duplicate check at the second call site instead, and
                        // this row would measure duplication rather than the
                        // planned-set membership it names.
                        ledger.record_emitted(call_template_id ^ u64::MAX, record)?;
                    }
                    D5CloseoutMutation::Exact
                    | D5CloseoutMutation::SuppressLedgerEntry
                    | D5CloseoutMutation::SubstituteEmittedCallee => {}
                }
            }
            Ok(operand)
        }
}

impl<'a> Lowering<'a> {
        /// Emit the direct call to a declared unit target.
        ///
        /// Returns the produced operand **and the exact `Inst` emitted for the
        /// call**. ⭐ The `Inst` is returned rather than kept in a `last_call` field
        /// (see also [`D5_EMITTED_DECLARATION_CALLS`])
        /// so that a caller which needs to attribute the emitted instruction has to
        /// take it from the emission itself; a stale side-channel would attribute
        /// one call site's instruction to another's token.
        pub(super) fn call_declared_unit_target(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            target: units::DeclaredUnitCall,
            inputs: &[LoweringOperand],
            #[cfg(test)] launch_ingress: Option<cranelift_codegen::ir::Value>,
        ) -> Result<(LoweringOperand, cranelift_codegen::ir::Inst), CraneliftBackendError> {
            let payload = builder.create_sized_stack_slot(StackSlotData::new(
                StackSlotKind::ExplicitSlot,
                target.header.frame_bytes,
                3,
            ));
            let mut input = 0usize;
            let mut result_offset = None;
            let mut trap_offset = None;
            for (slot, offset) in target.slots.iter().zip(&target.offsets) {
                let offset = i32::try_from(*offset).map_err(|_| {
                    backend_module("callee slot offset exceeds addressable range".to_string())
                })?;
                match slot.kind {
                    AbiSlotKind::Parameter | AbiSlotKind::Capture => {
                        let value = inputs.get(input).ok_or_else(|| {
                            backend_module("callee frame is missing a declared input".to_string())
                        })?;
                        let word = match value {
                            LoweringOperand::Carried(word) => word.word,
                            LoweringOperand::Specialized(value) => {
                                // ⚠ **`target.origin` is the CALLEE's scheduling
                                // entry**, and what still arrives specialized here
                                // is what no earlier crossing took.
                                //
                                // ⛔ **The two earlier crossings are NOT the same
                                // mechanism, and conflating them is what this
                                // comment used to do.** `lower_expr`'s
                                // direct-closure-callee arm carries each input at
                                // its exact caller-side occurrence. The source
                                // machine's call path carries its inputs at ONE
                                // common transfer coordinate with no per-argument
                                // pairing — inert, because an aggregate carries and
                                // is preflighted against its own producer
                                // authority, and a non-aggregate queries no
                                // aggregate ownership.
                                //
                                // MEASURED after both, `--nocapture
                                // --test-threads=1` over the whole suite: 137
                                // `BorrowedNativeValue`, 137 `CapabilityToken`, 42
                                // `Int` and 1 `Bool` `Parameter`s, plus 55 `Int`
                                // `Capture`s — all non-aggregate, so a
                                // `NonAggregate` request takes the caller's tag,
                                // consults no planned record and enters neither `E`
                                // nor `R`. The origin is not load-bearing for any
                                // of them. **No aggregate `Capture` reaches here at
                                // all**, so the capture-authority witness does not
                                // exist.
                                //
                                // ⛔ The one remaining aggregate population is
                                // **`Constructor` `Parameter`s from
                                // `call_static_worker`** (traced by backtrace, not
                                // inferred). They reach this fallback and
                                // **self-authorize**: each carries its own producer
                                // occurrence, so the coordinate below is not the
                                // authority its ownership record is resolved at.
                                //
                                // ⚠ No guard here refusing aggregates: measured, it
                                // would refuse those 97 inputs, which compile today.
                                #[cfg(test)]
                                if value.source_aggregate_producer().is_some() {
                                    SELF_AUTHORIZED_FALLBACK_REACHES
                                        .with(|n| n.set(n.get().saturating_add(1)));
                                }
                                self.transfer_into_carrier(
                                    builder,
                                    self.callee_scheduling_origin_under_mutation(target.origin),
                                    value,
                                )?
                                .word
                            }
                        };
                        builder.ins().stack_store(word, payload, offset);
                        input += 1;
                    }
                    AbiSlotKind::Control | AbiSlotKind::Store => {
                        let zero = builder.ins().iconst(types::I64, 0);
                        builder.ins().stack_store(zero, payload, offset);
                    }
                    AbiSlotKind::Trap => {
                        #[cfg(test)]
                        let zero = match TRAP_CALLER_PROTOCOL_MUTATION
                            .with(std::cell::Cell::get)
                        {
                            TrapCallerProtocolMutation::LeaveStaleTrap => {
                                builder.ins().iconst(types::I64, 1)
                            }
                            TrapCallerProtocolMutation::Exact
                            | TrapCallerProtocolMutation::ReadResultBeforeTrap => {
                                builder.ins().iconst(types::I64, 0)
                            }
                        };
                        #[cfg(not(test))]
                        let zero = builder.ins().iconst(types::I64, 0);
                        builder.ins().stack_store(zero, payload, offset);
                        trap_offset = Some(offset);
                    }
                    AbiSlotKind::Result => {
                        #[cfg(test)]
                        if TRAP_CALLER_PROTOCOL_MUTATION.with(std::cell::Cell::get)
                            == TrapCallerProtocolMutation::ReadResultBeforeTrap
                        {
                            let false_word = builder.ins().iconst(types::I64, 0);
                            builder.ins().stack_store(false_word, payload, offset);
                        }
                        result_offset = Some(offset);
                    }
                }
            }
            if input != inputs.len() {
                return Err(backend_module(
                    "caller supplied inputs absent from the callee descriptor".to_string(),
                ));
            }
            let pointer_type = builder.func.dfg.value_type(
                self.function_local
                    .services_pointer
                    .ok_or_else(|| backend_module("unit call has no services pointer".to_string()))?,
            );
            let slots = builder.ins().stack_addr(pointer_type, payload, 0);
            let envelope = builder.create_sized_stack_slot(StackSlotData::new(
                StackSlotKind::ExplicitSlot,
                u32::try_from(crate::activation_services::UNIT_CALL_FRAME_BYTES)
                    .expect("unit call frame byte count fits u32"),
                3,
            ));
            builder.ins().stack_store(
                slots,
                envelope,
                crate::activation_services::UNIT_CALL_FRAME_SLOTS,
            );
            let services = self
                .function_local
                .services_pointer
                .expect("services pointer checked above");
            let exact_host_dispatch_context =
                self.function_local.host_dispatch_context.ok_or_else(|| {
                    backend_module("unit call has no direct host-dispatch context".to_string())
                })?;
            #[cfg(test)]
            let host_dispatch_context = if launch_ingress.is_some()
                && PROCESS_SLOT_MUTATION.with(std::cell::Cell::get)
                    == ProcessSlotMutation::ReintroduceLaunchIngress
            {
                // This is the deliberately forbidden half of the AC-14 control:
                // unlike the retained direct context, this value is explicitly
                // sourced from the root adapter's launch-ingress parameter.
                launch_ingress.expect("the root adapter supplied launch ingress")
            } else {
                HOST_CONTEXT_PROPAGATION_MUTATION.with(|cell| match cell.get() {
                    HostContextPropagationMutation::Exact => exact_host_dispatch_context,
                    HostContextPropagationMutation::ServicesPointer if launch_ingress.is_none() => {
                        services
                    }
                    HostContextPropagationMutation::NativeIntArena if launch_ingress.is_none() => self
                        .function_local
                        .native_int_arena
                        .expect("unit native-int arena is bound"),
                    HostContextPropagationMutation::BoundaryArena if launch_ingress.is_none() => self
                        .function_local
                        .boundary_arena
                        .expect("unit boundary arena is bound"),
                    HostContextPropagationMutation::Null if launch_ingress.is_none() => {
                        builder.ins().iconst(pointer_type, 0)
                    }
                    HostContextPropagationMutation::LaunchIngress => {
                        launch_ingress.unwrap_or(exact_host_dispatch_context)
                    }
                    HostContextPropagationMutation::ServicesPointer
                    | HostContextPropagationMutation::NativeIntArena
                    | HostContextPropagationMutation::BoundaryArena
                    | HostContextPropagationMutation::Null => exact_host_dispatch_context,
                })
            };
            #[cfg(not(test))]
            let host_dispatch_context = exact_host_dispatch_context;
            builder.ins().stack_store(
                host_dispatch_context,
                envelope,
                crate::activation_services::UNIT_CALL_FRAME_HOST_DISPATCH_CONTEXT,
            );
            let envelope = builder.ins().stack_addr(pointer_type, envelope, 0);
            let call = builder.ins().call(target.function, &[envelope, services]);
            let [unit_status] = builder.inst_results(call) else {
                return Err(backend_module(
                    "internal unit call did not return exactly one word".to_string(),
                ));
            };
            let unit_status = *unit_status;
            let failed = builder.ins().icmp_imm(
                cranelift_codegen::ir::condcodes::IntCC::NotEqual,
                unit_status,
                0,
            );
            let failure_block = builder.create_block();
            let trap_check_block = builder.create_block();
            builder
                .ins()
                .brif(failed, failure_block, &[], trap_check_block, &[]);
            builder.switch_to_block(failure_block);
            builder.ins().return_(&[unit_status]);
            builder.seal_block(failure_block);
            builder.switch_to_block(trap_check_block);
            builder.seal_block(trap_check_block);
            let trap_offset = trap_offset.ok_or_else(|| {
                backend_module("callee frame declares no trap slot".to_string())
            })?;
            let result_offset = result_offset.ok_or_else(|| {
                backend_module("callee frame declares no result slot".to_string())
            })?;
            #[cfg(test)]
            if TRAP_CALLER_PROTOCOL_MUTATION.with(std::cell::Cell::get)
                == TrapCallerProtocolMutation::ReadResultBeforeTrap
            {
                let word = builder.ins().stack_load(types::I64, payload, result_offset);
                return Ok((LoweringOperand::Carried(CarriedBoundaryWord { word }), call));
            }
            let trap_word = builder.ins().stack_load(types::I64, payload, trap_offset);
            let trapped = builder.ins().icmp_imm(
                cranelift_codegen::ir::condcodes::IntCC::NotEqual,
                trap_word,
                0,
            );
            let trap_block = builder.create_block();
            let result_block = builder.create_block();
            builder.ins().brif(trapped, trap_block, &[], result_block, &[]);
            builder.switch_to_block(trap_block);
            match self.function_local.trap_exit {
                Some(TrapExitAuthority::UnitFrame { slots, trap_offset }) => {
                    #[cfg(test)]
                    px8tr_record_trap_provenance(Px8trTrapProvenanceEvent::UnitTrapWordPropagated {
                        seat: PlannedTrapSeat::UnitTrapWord,
                        identity_preserved: true,
                    });
                    builder
                        .ins()
                        .store(MemFlags::trusted(), trap_word, slots, trap_offset);
                    let no_result = builder.ins().iconst(types::I64, 0);
                    builder.ins().return_(&[no_result]);
                }
                Some(TrapExitAuthority::Root {
                    process_sentinel: true,
                    ..
                }) => {
                    #[cfg(test)]
                    px8tr_record_trap_provenance(Px8trTrapProvenanceEvent::UnitTrapWordPropagated {
                        seat: PlannedTrapSeat::RootProcessSentinel,
                        identity_preserved: false,
                    });
                    let process_trap = builder.ins().iconst(types::I64, -4);
                    builder.ins().return_(&[process_trap]);
                }
                Some(TrapExitAuthority::Root {
                    process_sentinel: false,
                    ..
                }) => {
                    #[cfg(test)]
                    px8tr_record_trap_provenance(Px8trTrapProvenanceEvent::UnitTrapWordPropagated {
                        seat: PlannedTrapSeat::RootTrapToken,
                        identity_preserved: true,
                    });
                    let shifted = builder.ins().ishl_imm(
                        trap_word,
                        crate::cranelift_backend::compiled::ROOT_TRAP_TOKEN_SHIFT,
                    );
                    let root_token = builder.ins().bor_imm(
                        shifted,
                        crate::cranelift_backend::compiled::ROOT_TRAP_TOKEN_TAG,
                    );
                    builder.ins().return_(&[root_token]);
                }
                None => {
                    return Err(backend_module(
                        "trap branch has no generated-unit TrapWord lane".to_string(),
                    ));
                }
            }
            builder.seal_block(trap_block);
            builder.switch_to_block(result_block);
            builder.seal_block(result_block);
            let word = builder.ins().stack_load(types::I64, payload, result_offset);
            Ok((LoweringOperand::Carried(CarriedBoundaryWord { word }), call))
        }
}

impl<'a> Lowering<'a> {
        /// **`RT-CONTSPEC-ACTIVATE` `4b` — decode the callee of an emitted direct
        /// call out of the finished CLIF.**
        ///
        /// ⭐ **This is the independent side of the emission gate.** It reads the
        /// instruction stream that was actually built: the instruction's
        /// `func_ref`, that ref's `ExtFuncData` name, and the function's own
        /// imported-user-name table, which `Module::declare_func_in_func` populates
        /// with `UserExternalName { namespace: 0, index: func_id }`. ⛔ Nothing here
        /// consults `continuation_calls`, the claim ledger's `resolved` map, or the
        /// `DeclaredUnitCall` that was handed to the emitter -- those are all
        /// downstream of the same resolution and comparing against one of them
        /// would be a re-run of the builder under test.
        ///
        /// ⛔ A non-direct call, a non-user name, or a foreign namespace is a
        /// rejection rather than a skip: an unattributable callee must not read as
        /// agreement.
        pub(super) fn decode_direct_callee(
            func: &Function,
            inst: cranelift_codegen::ir::Inst,
        ) -> Result<FuncId, CraneliftBackendError> {
            let cranelift_codegen::ir::InstructionData::Call { func_ref, .. } = func.dfg.insts[inst]
            else {
                return Err(backend_module(
                    "an emitted continuation call site does not hold a direct call instruction"
                        .to_string(),
                ));
            };
            let cranelift_codegen::ir::ExternalName::User(name_ref) = func.dfg.ext_funcs[func_ref].name
            else {
                return Err(backend_module(
                    "an emitted continuation call names a callee that is not a user function"
                        .to_string(),
                ));
            };
            let user = &func.params.user_named_funcs()[name_ref];
            if user.namespace != 0 {
                return Err(backend_module(
                    "an emitted continuation call names a callee outside the module function namespace"
                        .to_string(),
                ));
            }
            Ok(FuncId::from_u32(user.index))
        }
}

impl<'a> Lowering<'a> {
        pub(super) fn unwrap_terminal_ret(mut lowered: Lowered) -> Lowered {
            loop {
                match lowered {
                    Lowered::Constructor {
                        constructor,
                        mut args,
                        synthesized_identity,
                        occurrence,
                    } if constructor.ends_with("::ITree::Ret") && args.len() == 1 => {
                        match args.remove(0) {
                            ConstructorField::Specialized(inner) => lowered = inner,
                            // This function is infallible, so it cannot refuse; the
                            // decision it CAN take is to leave the wrapper on and
                            // keep the refusal with a consumer that is able to make
                            // one. Unwrapping would hand a caller expecting a value
                            // something with no value representation.
                            //
                            // **Re-derived now that a worker really can arrive
                            // here.** The original justification was that the read
                            // was infallible because nothing constructed a worker;
                            // that premise is gone, so the claim has to rest on
                            // where the intact constructor actually lands. Both
                            // reachable consumers fail closed on it:
                            // `emit_process_exit_status` answers its `-3`
                            // malformed-payload sentinel, and the scalar-pair join
                            // either routes into that same decoder or refuses with
                            // *"dynamic native arms must produce scalar Int
                            // values"*. Neither can mistake the wrapper for a
                            // value, which is what makes handing it back the
                            // conservative move rather than merely the unchanged
                            // one.
                            //
                            // **And the conservation close is behind both of
                            // them.** A worker field that nothing rebinds refuses
                            // before the root answer is emitted, so an intact
                            // wrapper carrying an unconsumed worker cannot appear
                            // in a shipped object at all — the two decoder
                            // refusals above are what happens on the way there,
                            // not the last line of defence.
                            field @ ConstructorField::StaticWorker { .. } => {
                                return Lowered::Constructor {
                                    constructor,
                                    synthesized_identity,
                                    occurrence,
                                    args: vec![field],
                                };
                            }
                        }
                    }
                    lowered => return lowered,
                }
            }
        }
}

impl<'a> Lowering<'a> {
        pub(super) fn emit_result(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            value: Lowered,
        ) -> Result<(cranelift_codegen::ir::Value, ResultDecoder), CraneliftBackendError> {
            if self.process_object {
                let _authority = self.mint_terminal_answer_authority()?;
                let value = Self::unwrap_terminal_ret(value);
                let value = match value {
                    Lowered::ProcessExitStatus { value } => value,
                    value => self.emit_process_exit_status(builder, value),
                };
                return Ok((value, ResultDecoder::ProcessStatus));
            }
            match value {
                Lowered::Int { value, known } => {
                    let tag = self.native_int_tag(builder, value, known)?;
                    let arena = self.function_local.native_int_arena.ok_or_else(|| {
                        unsupported("NativeResult", "Int result has no invocation arena")
                    })?;
                    let export = self.function_local.native_int_export.ok_or_else(|| {
                        unsupported("NativeResult", "Int result has no export support function")
                    })?;
                    #[cfg(test)]
                    if self.native_int_mutation == NativeIntLoweringMutation::SuppressTerminalExport {
                        return Ok((value, ResultDecoder::Int));
                    }
                    let call = builder.ins().call(export, &[arena, tag, value]);
                    Self::require_i64(builder, builder.inst_results(call)[0], 0);
                    #[cfg(test)]
                    if self.native_int_mutation == NativeIntLoweringMutation::CorruptTerminalExport {
                        let invalid = builder.ins().iconst(types::I64, 7);
                        builder.ins().store(
                            MemFlags::trusted(),
                            invalid,
                            arena,
                            crate::native_int_clif::ARENA_FINAL_TAG,
                        );
                    }
                    Ok((value, ResultDecoder::Int))
                }
                Lowered::Bool { value, .. } => Ok((value, ResultDecoder::Bool)),
                value => {
                    let ground = self.ground_value(value)?;
                    let token = self.intern_result(ground);
                    Ok((
                        builder.ins().iconst(types::I64, token),
                        ResultDecoder::Table,
                    ))
                }
            }
        }
}

impl<'a> Lowering<'a> {
        pub(super) fn emit_process_exit_status(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            value: Lowered,
        ) -> cranelift_codegen::ir::Value {
            let Lowered::Constructor {
                constructor, args, ..
            } = value
            else {
                return builder.ins().iconst(types::I64, -2);
            };
            if constructor == self.process_symbols.exit_success {
                return if args.is_empty() {
                    builder.ins().iconst(types::I64, 0)
                } else {
                    builder.ins().iconst(types::I64, -2)
                };
            }
            if constructor != self.process_symbols.exit_failure {
                return builder.ins().iconst(types::I64, -2);
            }
            // A worker field joins the malformed-payload sentinel rather than
            // getting its own: this decoder answers with a status word and has no
            // way to refuse, so the conservative move is the value that already
            // means "this is not a decodable exit status".
            let Ok(args) = specialized_fields_at(&args, "an exit status payload field") else {
                return builder.ins().iconst(types::I64, -3);
            };
            let Ok([payload]) = <Vec<Lowered> as TryInto<[Lowered; 1]>>::try_into(args) else {
                return builder.ins().iconst(types::I64, -3);
            };
            let Lowered::Int { known, .. } = &payload else {
                return builder.ins().iconst(types::I64, -3);
            };
            if let Some(code) = *known {
                let mapping = crate::process_exit_status(crate::ProcessExitCode::Failure(code));
                return builder.ins().iconst(
                    types::I64,
                    if mapping.trap_report.is_some() {
                        -3
                    } else {
                        i64::from(mapping.status)
                    },
                );
            }
            let Ok((value, valid_int)) = self.narrow_native_int_u64(builder, &payload) else {
                return builder.ins().iconst(types::I64, -3);
            };
            let zero = builder.ins().iconst(types::I64, 0);
            let one = builder.ins().iconst(types::I64, 1);
            let max = builder.ins().iconst(types::I64, 255);
            let malformed = builder.ins().iconst(types::I64, -3);
            let is_zero =
                builder
                    .ins()
                    .icmp(cranelift_codegen::ir::condcodes::IntCC::Equal, value, zero);
            let positive = builder.ins().icmp(
                cranelift_codegen::ir::condcodes::IntCC::UnsignedGreaterThan,
                value,
                zero,
            );
            let within_max = builder.ins().icmp(
                cranelift_codegen::ir::condcodes::IntCC::UnsignedLessThanOrEqual,
                value,
                max,
            );
            let valid = builder.ins().band(valid_int, positive);
            let valid = builder.ins().band(valid, within_max);
            let nonzero = builder.ins().select(valid, value, malformed);
            builder.ins().select(is_zero, one, nonzero)
        }
}
