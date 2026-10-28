//! **`RT-FNSPLIT-B2F` `D1`/`D2` — the target code-unit population.**
//!
//! One closed Cranelift target function per `PredeclaredFunction` in the
//! validated `SemanticOwner` partition, forward-declared as a bundle and then
//! defined against `B2R`'s declared activation frame.
//!
//! ⛔ **This module does not derive the population and must never be made to.**
//! The unit set is `plan.entries` ∪ every `EdgeKind::StaticBody` **target**,
//! MINUS every declaration-owned pair (`RT-DECL-CLOSURE-PORT` `D2a`: a
//! closure-seed transparent declaration's entry seeds no function of its own),
//! seeded and validated by `B2O` and given one `AbiDescriptor` apiece by `B2R`.
//! `StaticTransitionPlan::emittable_units` projects that; this module consumes
//! the projection. In particular it never consults
//! `TransitionKind::ClosureBody`, which is a body's **return successor** and not
//! a unit head — the error that has appeared in successive drafts of the issue
//! file that warns against it.
//!
//! ⚠ The two shared exits (`SemanticOwner::Terminal`, `TrapTerminal`) are not
//! units and receive no target function. They are absent from
//! `emittable_units()` by construction, because `B2R` gives them no descriptor —
//! ⛔ **not** because this module filters them out. There is deliberately no
//! filter here to get wrong.

use super::core::{AmbientBodyAuthority, CheckedFrameFunctionScope, CheckedTokenFunctionScope};
use super::*;
use crate::cranelift_backend::surface::LoweringCause;
// `close_host_effect_seat_ledger`'s own `#[cfg(test)]` mutation check reaches
// these directly; moved to `effects.rs` at `RT-EMITTER-EFFECTS-SPLIT` `D1`.
#[cfg(test)]
use super::effects::{EffectSeatVisitMutation, effect_seat_visit_mutation};
use crate::cranelift_backend::planning::{
    FusionComposedEdge, FusionCompositionLayer, FusionOwnedOuterRealization,
};

use cranelift_module::FuncId;

/// **`RT-CHECKED-IH-CAPTURED-ENV-SCHEMA` tier 1 -- mechanically validate the
/// admitted ci<->oi bijection at its construction site.**
///
/// The mapping is READ from the planner's `WorkerCapture` roles, never rebuilt,
/// so this does not re-derive it. What it checks is that what the plan issued
/// really is a bijection over the capture run: exactly one entry per capture,
/// ordinals contiguous from zero, no ordinal repeated, and no source occurrence
/// serving two captures.
///
/// Deliberately checks ORDER AND MULTIPLICITY, not just the count. A count
/// agreement is the class of evidence that made a nine-versus-nine coincidence
/// look like confirmation during this slice's measurement; the whole point of a
/// bijection is the part a count cannot see.
/// **`RT-CHECKED-IH-CAPTURED-ENV-SCHEMA` tier 1 -- admit one capture's source
/// occurrence, or REFUSE.**
///
/// `Lexical` carries the planner-issued occurrence, which is the field-identity
/// authority. `Seed` carries none: a seed capture has no source occurrence, so
/// there is nothing to admit and a label would have to be invented. That is the
/// fabrication this slice exists to prevent, so it fails closed.
///
/// There is deliberately NO positional fallback. Falling back to "capture i gets
/// occurrence i" would compile, would pass on any population whose ordinals
/// happen to line up, and is exactly the plausible-face fabrication this
/// slice's measurement rejected.
fn admitted_capture_source(
    ordinal: u32,
    source: &ContinuationWorkerCaptureSource,
) -> Result<StaticOriginId, CraneliftBackendError> {
    match source {
        ContinuationWorkerCaptureSource::Lexical(origin) => Ok(*origin),
        ContinuationWorkerCaptureSource::Seed => Err(backend_module(
            LoweringCause::PlannerInvariant,
            format!(
                "checked-IH captured environment: capture {ordinal} is a seed capture with no \
             source occurrence, so its field identity cannot be admitted from the plan \
             and must not be invented",
            ),
        )),
    }
}

/// **`RT-CHECKED-IH-CAPTURED-ENV-SCHEMA` tier 1 -- the admitted run IS the
/// planner-issued one, field for field.**
///
/// [`validate_captured_environment_bijection`] proves the run is A bijection:
/// contiguous ordinals, no repeats, injective sources. It cannot prove it is
/// THE PLANNER'S bijection -- permuting only the source column preserves the
/// count, the ordinals, the multiplicity and the injectivity, so every one of
/// those predicates still holds while each capture now carries a DIFFERENT
/// capture's occurrence. That is the plausible-face fabrication this node
/// exists to prevent, and it was reachable in production until this check.
///
/// So this compares the admitted association against the authoritative role
/// sequence pairwise, in order: capture i must carry the ordinal AND the source
/// that the i-th `WorkerCapture` role in the envelope issued.
fn validate_admitted_matches_role_sequence(
    envelope: &[ContinuationOrdinaryEnvelopeRole],
    capture_sources: &[(u32, StaticOriginId)],
) -> Result<(), CraneliftBackendError> {
    let mut index = 0usize;
    for role in envelope {
        let ContinuationOrdinaryEnvelopeRole::WorkerCapture {
            ordinal, source, ..
        } = role
        else {
            continue;
        };
        let Some((admitted_ordinal, admitted_source)) = capture_sources.get(index) else {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "checked-IH captured environment: the envelope issues more worker captures \
                 than the admitted run holds, first missing at position {index}",
                ),
            ));
        };
        if admitted_ordinal != ordinal {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "checked-IH captured environment: capture at position {index} was admitted \
                 with ordinal {admitted_ordinal}, but the plan issues ordinal {ordinal} there",
                ),
            ));
        }
        let issued = admitted_capture_source(*ordinal, source)?;
        if *admitted_source != issued {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "checked-IH captured environment: capture {ordinal} was admitted with source \
                 {admitted_source:?}, but the plan issues {issued:?} for it -- the admitted \
                 association is not the planner-issued one",
                ),
            ));
        }
        index += 1;
    }
    if index != capture_sources.len() {
        return Err(backend_module(
            LoweringCause::PlannerInvariant,
            format!(
                "checked-IH captured environment: the admitted run holds {} captures but the \
             envelope issues {index}",
                capture_sources.len(),
            ),
        ));
    }
    Ok(())
}

fn validate_captured_environment_bijection(
    capture_sources: &[(u32, StaticOriginId)],
    capture_count: usize,
) -> Result<(), CraneliftBackendError> {
    // Unreachable from the single call site today: that loop pushes to
    // `capture_sources` and `worker_captures` in lockstep, so the two lengths
    // cannot diverge there. Retained as defence-in-depth for a real
    // function-level invariant -- this validator takes the run as an argument
    // and must not assume its caller built it that way. Noted so a later reader
    // does not mistake it for a live production guard and reason from its
    // reachability.
    if capture_sources.len() != capture_count {
        return Err(backend_module(
            LoweringCause::PlannerInvariant,
            format!(
                "checked-IH captured environment: {} sourced occurrences for {capture_count} \
             captures, so the admitted run is not a bijection",
                capture_sources.len(),
            ),
        ));
    }
    let mut seen_ordinals: Vec<u32> = Vec::with_capacity(capture_sources.len());
    let mut seen_sources: Vec<StaticOriginId> = Vec::with_capacity(capture_sources.len());
    for (index, (ordinal, source)) in capture_sources.iter().enumerate() {
        let expected = u32::try_from(index).map_err(|_| {
            backend_module(
                LoweringCause::Capacity,
                "checked-IH captured environment: capture ordinal exceeds addressable range"
                    .to_string(),
            )
        })?;
        if *ordinal != expected {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "checked-IH captured environment: capture at position {index} carries ordinal \
                 {ordinal}, so the plan's capture ordinals are not the contiguous run this \
                 environment is admitted over",
                ),
            ));
        }
        if seen_ordinals.contains(ordinal) {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!("checked-IH captured environment: capture ordinal {ordinal} appears twice",),
            ));
        }
        if seen_sources.contains(source) {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "checked-IH captured environment: source occurrence {source:?} is claimed by \
                 two captures, so the admitted mapping is not injective",
                ),
            ));
        }
        seen_ordinals.push(*ordinal);
        seen_sources.push(*source);
    }
    Ok(())
}

/// **`RT-FNSPLIT-B2F` `AC-2` — what the compiled module ACTUALLY contains.**
///
/// ⭐ **This exists because the census pin in `control.rs` cannot answer the
/// question `AC-2` is really about.** That pin counts how many times three
/// spellings occur in seven source files; it is a source-TEXT oracle, so a call
/// split across lines evades it and a mention in a comment inflates it, and in
/// no configuration does it observe an emitted function. The property `B2F` owes
/// is *"this node adds exactly the emitted units its design predicts"*, and the
/// only way to see an emitted unit is to count them at the point of emission.
///
/// Records `(declared, defined)` for the most recent compile on this thread.
/// ⚠ Two numbers rather than one: a bundle that declares `n` and defines `n-1`
/// leaves an undefined symbol, and a single counter cannot tell that from a
/// smaller correct population.
#[cfg(test)]
thread_local! {
    static B2F_UNIT_EMISSION: std::cell::Cell<(usize, usize)> =
        const { std::cell::Cell::new((0, 0)) };
}

/// The `(declared, defined)` unit counts from the most recent compile.
///
/// ⚠ **"Most recent compile" is the whole limitation.** This reading carries no
/// statement about *which* compile produced it, so a compile that fails before
/// reaching the emission seam leaves the previous compile's numbers standing and
/// reads exactly like one that reached the seam and declared that many. Use it
/// only where a single compile is known to have run to emission; for a timing
/// question about a *failing* compile, use the attempt epoch below.
#[cfg(test)]
pub(in crate::cranelift_backend) fn b2f_last_unit_emission() -> (usize, usize) {
    B2F_UNIT_EMISSION.with(std::cell::Cell::get)
}

/// **`RT-FNSPLIT-B2F` `AC-11` clause 3 — the compile-attempt epoch.**
///
/// ⛔⛔ **This exists because the first timing instrument could not distinguish
/// the two outcomes it was built to separate, and reported a confident number
/// for the wrong one.** That version compiled a successful sentinel to force
/// `B2F_UNIT_EMISSION` to a nonzero value, then compiled the failing fixture and
/// read the cell back. But nothing on a pre-emission refusal path *writes* that
/// cell — so the read returned the **sentinel's** `1`, and:
///
/// - "refused before `declare_unit_bundle` ran" (the wanted `0`), and
/// - "declared one unit, then refused during lowering" (the feared `1`)
///
/// ⇒ produce the **identical reading**. ⭐ The in-source comment claimed the
/// sentinel made those cases distinguishable; it made them indistinguishable.
/// A measured `1` was therefore evidence of nothing, in **either** direction.
///
/// ⭐ **The repair is to stamp the reading with the attempt it belongs to**, so
/// a stale value is *detectable as stale* rather than readable as a count.
/// Three outcomes, all distinct:
///
/// | reading | meaning |
/// |---|---|
/// | `None` | ⚠ the compile never reached the emission seam at all — refused earlier still, or never ran. **Not** a zero |
/// | `Some(0)` | ✅ reached the seam, refused **before** any unit was declared — what clause 3 asks for |
/// | `Some(n > 0)` | ⛔ `n` units were already declared when the refusal came — a *later* guarantee, not clause 3's |
///
/// ⛔ **The stamp is written in `core.rs` immediately before
/// `validate_emitted_transfers_are_representable`, NOT inside
/// `declare_unit_bundle`.** Stamping inside the bundle would make `Some(0)`
/// unreachable — the only way to observe the epoch would be to declare a unit,
/// which is the very event the reading is supposed to detect the absence of.
#[cfg(test)]
thread_local! {
    /// The epoch a test opened; bumped once per `b2f_open_compile_attempt`.
    static B2F_ATTEMPT: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    /// The epoch that was live when the emission seam was last reached.
    static B2F_ATTEMPT_AT_SEAM: std::cell::Cell<Option<u64>> =
        const { std::cell::Cell::new(None) };
}

/// Open a fresh compile attempt; the returned epoch identifies it.
///
/// ⚠ Deliberately does **not** clear `B2F_UNIT_EMISSION`: clearing it here would
/// hide a compile that never reached the seam behind a plausible `(0, 0)`, which
/// is the exact confusion this epoch exists to remove.
#[cfg(test)]
pub(in crate::cranelift_backend) fn b2f_open_compile_attempt() -> u64 {
    B2F_ATTEMPT.with(|cell| {
        let next = cell.get() + 1;
        cell.set(next);
        next
    })
}

/// Record that the emission seam was reached, and zero this attempt's counts.
#[cfg(test)]
pub(in crate::cranelift_backend) fn b2f_reached_emission_seam() {
    B2F_ATTEMPT_AT_SEAM.with(|cell| cell.set(Some(B2F_ATTEMPT.with(std::cell::Cell::get))));
    B2F_UNIT_EMISSION.with(|cell| cell.set((0, 0)));
}

/// How many units `epoch`'s compile had declared, or `None` if that compile
/// never reached the emission seam.
#[cfg(test)]
pub(in crate::cranelift_backend) fn b2f_units_declared_in_attempt(epoch: u64) -> Option<usize> {
    if B2F_ATTEMPT_AT_SEAM.with(std::cell::Cell::get) == Some(epoch) {
        Some(B2F_UNIT_EMISSION.with(std::cell::Cell::get).0)
    } else {
        None
    }
}

/// Every target function this artifact declares, keyed by its static identity.
///
/// ⭐ **Keyed by `PredeclaredFunctionId`, never by the iteration ordinal.** The
/// ordinal is an artifact of how this module happened to walk the plane; the id
/// is the planner's identity, and `D4`'s call edges resolve against *that*. A
/// map keyed on position would be an identity alias of exactly the kind `B2O`
/// removed from `SemanticDescriptor` (`semantic_ir.rs:651` records that
/// removal), reintroduced one layer out.
pub(in crate::cranelift_backend) struct UnitBundle {
    functions: BTreeMap<PredeclaredFunctionId, FuncId>,
    /// Typed generated-function scope for every declared target. `FuncId` is
    /// used only while declaring local `FuncRef`s; it never enters the finished
    /// grafted-spine graph.
    grafted_spine_scopes: BTreeMap<FuncId, GraftedSpineFunctionScope>,
    /// **`RT-CONTSPEC-ACTIVATE` `D2`** -- one declared target per planned
    /// continuation specialization, keyed by the planner's typed identity.
    ///
    /// Kept as its own map rather than folded into `functions`: a
    /// `ContinuationSpecializationId` is **not** a `PredeclaredFunctionId`, and
    /// admitting one there would alias two identities that the planner keeps
    /// apart. Nothing resolves a continuation by ordinal or by symbol name.
    continuations: BTreeMap<ContinuationSpecializationId, FuncId>,
    /// One declared target per compile-time response owner. Kept in its own
    /// identity domain: an owner is selected by one incoming causal edge and is
    /// neither that edge's K specialization nor the context it will later call.
    responses: BTreeMap<StaticResponseOwnerId, FuncId>,
    /// **`RT-DECL-CLOSURE-PORT` `D5a`** -- one declared target per planned
    /// generated producer execution context.
    ///
    /// A third map for the same reason there is a second: a
    /// `ContinuationContextId` is neither of the other two identities, and the
    /// ruling's "do not cast or alias one ID domain into the other" is enforced
    /// here by there being no map a caller could reach with the wrong key type.
    contexts: BTreeMap<ContinuationContextId, FuncId>,
    /// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2f`** -- one declared target per
    /// installed static continuation fusion region.
    ///
    /// A **fourth** map for the same reason there is a third: a
    /// `StaticContinuationFusionId` is none of the other three identities, and
    /// the standing "do not cast or alias one ID domain into the other" ruling
    /// is enforced here by there being no map a caller could reach with the
    /// wrong key type. In particular it is **not** keyed by the producer's
    /// `PredeclaredFunctionId`: the fused region is a third thing that owns
    /// itself, and keying it by the producer would make the redirect below
    /// resolve to whichever of the two the last writer meant.
    fusions: BTreeMap<StaticContinuationFusionId, FuncId>,
}

impl UnitBundle {
    /// The declared target function for one unit.
    ///
    /// ⛔ `None` is a real answer and the caller must not substitute one of its
    /// own: a unit absent here was never declared, and emitting a call to a
    /// fabricated `FuncId` is the failure this return type exists to make
    /// visible.
    pub(in crate::cranelift_backend) fn function(
        &self,
        unit: PredeclaredFunctionId,
    ) -> Option<FuncId> {
        self.functions.get(&unit).copied()
    }

    fn grafted_spine_scope(&self, function: FuncId) -> Option<GraftedSpineFunctionScope> {
        self.grafted_spine_scopes.get(&function).copied()
    }

    pub(super) fn grafted_spine_scope_population(
        &self,
    ) -> impl Iterator<Item = GraftedSpineFunctionScope> + '_ {
        self.grafted_spine_scopes.values().copied()
    }

    /// The declared target for one continuation specialization.
    ///
    /// `None` is a real answer and must not be substituted for: a
    /// specialization absent here was never declared, and resolving a causal
    /// identity to a fabricated `FuncId` is exactly what this return type
    /// exists to make visible.
    pub(in crate::cranelift_backend) fn continuation(
        &self,
        specialization: ContinuationSpecializationId,
    ) -> Option<FuncId> {
        self.continuations.get(&specialization).copied()
    }

    pub(in crate::cranelift_backend) fn response(
        &self,
        owner: StaticResponseOwnerId,
    ) -> Option<FuncId> {
        self.responses.get(&owner).copied()
    }

    pub(in crate::cranelift_backend) fn response_targets(
        &self,
    ) -> impl Iterator<Item = FuncId> + '_ {
        self.responses.values().copied()
    }

    /// The declared target for one generated producer execution context.
    ///
    /// `None` is a real answer, exactly as for the two maps above.
    pub(in crate::cranelift_backend) fn context(
        &self,
        context: ContinuationContextId,
    ) -> Option<FuncId> {
        self.contexts.get(&context).copied()
    }

    /// The declared target for one installed fused region.
    ///
    /// `None` is a real answer, exactly as for the three maps above.
    pub(in crate::cranelift_backend) fn fusion(
        &self,
        fusion: StaticContinuationFusionId,
    ) -> Option<FuncId> {
        self.fusions.get(&fusion).copied()
    }

    /// How many fused-region targets this bundle declares.
    pub(in crate::cranelift_backend) fn fusion_len(&self) -> usize {
        self.fusions.len()
    }

    /// How many continuation targets this bundle declares.
    pub(in crate::cranelift_backend) fn continuation_len(&self) -> usize {
        self.continuations.len()
    }

    /// How many target functions this bundle declares.
    ///
    /// ⚠ This is the **emitted-unit** count, not a source-spelling count. `D8`'s
    /// growth verdict is about this number; the census pin in `control.rs`
    /// counts spellings and cannot see it.
    pub(in crate::cranelift_backend) fn len(&self) -> usize {
        self.functions.len()
    }
}

impl Lowering<'_> {
    /// Verify the record's complete direct-store inventory against finalized
    /// CLIF. A producer capture must store the helper's exact returned word;
    /// copy-up has its own checked callee-frame provenance, and clearing is
    /// restricted to the member slot. No other store may address these slots.
    fn verify_pending_vis_slot_stores(&self, func: &Function) -> Result<(), CraneliftBackendError> {
        use cranelift_codegen::ir::{InstructionData, Opcode, ValueDef};
        let error = |why: &str| {
            backend_module(
                LoweringCause::FinishedCode,
                format!("pending-Vis record-store verifier: {why}",),
            )
        };
        let capacity_error = |why: &str| {
            backend_module(
                LoweringCause::Capacity,
                format!("pending-Vis record-store verifier: {why}",),
            )
        };
        let facts = &self.function_local.pending_vis_slot_stores;
        let sites = &self.function_local.pending_vis_copy_sites;
        let producers = &self.function_local.pending_vis_producer_sites;
        let Some(frame) = self.function_local.pending_vis_frame else {
            if !facts.is_empty() || !sites.is_empty() || !producers.is_empty() {
                return Err(error("a generated body wrote a record without its frame"));
            }
            return Ok(());
        };
        let cfg = ControlFlowGraph::with_function(func);
        let mut dom = cranelift_codegen::dominator_tree::DominatorTree::new();
        dom.compute(func, &cfg);
        let mut expected = BTreeMap::new();
        let mut copied = BTreeMap::new();
        for site in sites {
            if site.destination != frame.region
                || copied
                    .insert((site.call, site.copy_guard), BTreeSet::new())
                    .is_some()
            {
                return Err(error("a copy site has the wrong frame or repeats its call"));
            }
        }
        for fact in facts {
            if expected.insert(fact.inst, fact).is_some() {
                return Err(error("a record store was recorded twice"));
            }
        }
        let stack_word = |inst, slot, offset| {
            matches!(
                func.dfg.insts[inst],
                InstructionData::StackLoad { opcode: Opcode::StackLoad,
                    stack_slot, offset: actual } if stack_slot == slot && i32::from(actual) == offset
            )
        };
        let compare_nonzero = |guard, word| {
            if func.dfg.insts[guard].opcode() != Opcode::Brif {
                return false;
            }
            let Some(&condition) = func.dfg.inst_args(guard).first() else {
                return false;
            };
            let ValueDef::Result(compare, _) = func.dfg.value_def(condition) else {
                return false;
            };
            matches!(func.dfg.insts[compare],
                InstructionData::IntCompareImm { opcode: Opcode::IcmpImm, arg, imm, cond }
                if arg == word && imm.bits() == 0
                    && cond == cranelift_codegen::ir::condcodes::IntCC::NotEqual)
        };
        for inst in func
            .layout
            .blocks()
            .flat_map(|block| func.layout.block_insts(block))
        {
            let args = func.dfg.inst_args(inst);
            if !args.contains(&frame.slots) {
                continue;
            }
            match func.dfg.insts[inst] {
                InstructionData::Load { .. } => continue,
                InstructionData::Store { offset, .. } if args.get(1) == Some(&frame.slots) => {
                    let offset = i32::from(offset);
                    if offset < frame.region.discriminant
                        || i64::from(offset) >= i64::from(frame.region.frame_bytes)
                    {
                        continue;
                    }
                    let fact = expected.remove(&inst).ok_or_else(|| {
                        error("an unaccounted store addresses a pending record slot")
                    })?;
                    if fact.offset != offset {
                        return Err(error("a recorded store moved to a different record slot"));
                    }
                    let stored = args
                        .first()
                        .copied()
                        .ok_or_else(|| error("a pending record store has no value"))?;
                    match fact.source {
                        PendingVisStoreSource::Captured(word) => {
                            if offset == frame.region.discriminant || stored != word {
                                return Err(error(
                                    "a capture slot did not store the helper result",
                                ));
                            }
                        }
                        PendingVisStoreSource::Member(row) => {
                            let ValueDef::Result(mint, _) = func.dfg.value_def(stored) else {
                                return Err(error("a member was not minted at its producer"));
                            };
                            let exact = i64::from(row.ordinal()) + 1;
                            if offset != frame.region.discriminant
                                || !matches!(func.dfg.insts[mint],
                                    InstructionData::UnaryImm { opcode: Opcode::Iconst, imm }
                                    if imm.bits() == exact)
                            {
                                return Err(error("a member is not its producer's exact row"));
                            }
                        }
                        PendingVisStoreSource::Cleared => {
                            let ValueDef::Result(mint, _) = func.dfg.value_def(stored) else {
                                return Err(error("the consumed member was not cleared"));
                            };
                            if offset != frame.region.discriminant
                                || !matches!(func.dfg.insts[mint],
                                    InstructionData::UnaryImm { opcode: Opcode::Iconst, imm }
                                    if imm.bits() == 0)
                            {
                                return Err(error("a consumed member is not cleared to zero"));
                            }
                        }
                        PendingVisStoreSource::Copied {
                            load,
                            source,
                            source_region,
                            source_offset,
                            call,
                            status_guard,
                            trap_guard,
                            trap_load,
                            trap_offset,
                            member_load,
                            member_offset,
                            copy_guard,
                        } => {
                            let positions = copied
                                .get_mut(&(call, copy_guard))
                                .ok_or_else(|| error("a copy store has no declared call site"))?;
                            let site = sites
                                .iter()
                                .find(|site| site.call == call && site.copy_guard == copy_guard)
                                .ok_or_else(|| error("a copy store has no declared call site"))?;
                            if site.source != source_region || !positions.insert(offset) {
                                return Err(error(
                                    "a copy store repeats a field or changes its source region",
                                ));
                            }
                            // Position correspondence is checked against the
                            // two planner regions, not the emitter's recorded
                            // source offset. A same-shaped sibling capture
                            // cannot be substituted by changing only the load.
                            let expected_source = if offset == frame.region.discriminant {
                                Some(source_region.discriminant)
                            } else if offset < frame.region.continuation_inputs {
                                (offset - frame.region.captures).checked_add(source_region.captures)
                            } else {
                                (offset - frame.region.continuation_inputs)
                                    .checked_add(source_region.continuation_inputs)
                            };
                            if expected_source != Some(source_offset)
                                || member_offset != source_region.discriminant
                            {
                                return Err(error("copy-up changed a record field's position"));
                            }
                            let word = func.dfg.inst_results(load).first().copied();
                            let status = func.dfg.inst_results(call).first().copied();
                            let trap = func.dfg.inst_results(trap_load).first().copied();
                            let member = func.dfg.inst_results(member_load).first().copied();
                            if func.sized_stack_slots[source].size != source_region.frame_bytes {
                                return Err(error("a copied callee frame has the wrong tail size"));
                            }
                            if !stack_word(load, source, source_offset)
                                || !stack_word(trap_load, source, trap_offset)
                                || !stack_word(member_load, source, member_offset)
                                || word != Some(stored)
                            {
                                return Err(error(
                                    "a copy store did not use its callee-frame load",
                                ));
                            }
                            if !status.is_some_and(|status| compare_nonzero(status_guard, status))
                                || !trap.is_some_and(|trap| compare_nonzero(trap_guard, trap))
                                || !member.is_some_and(|member| compare_nonzero(copy_guard, member))
                                || !matches!(func.dfg.value_def(status.unwrap()),
                                    ValueDef::Result(producer, 0) if producer == call)
                            {
                                return Err(error(
                                    "copy-up lacks an exact status, Trap or member check",
                                ));
                            }
                            if !dom.dominates(call, status_guard, &func.layout)
                                || !dom.dominates(status_guard, trap_guard, &func.layout)
                                || !dom.dominates(trap_load, trap_guard, &func.layout)
                                || !dom.dominates(trap_guard, member_load, &func.layout)
                                || !dom.dominates(member_load, copy_guard, &func.layout)
                                || (load != member_load
                                    && !dom.dominates(copy_guard, load, &func.layout))
                                || !dom.dominates(copy_guard, inst, &func.layout)
                            {
                                return Err(error(
                                    "a copy store bypassed its checked callee result",
                                ));
                            }
                            let status_edges = func.dfg.insts[status_guard]
                                .branch_destination(&func.dfg.jump_tables);
                            let trap_edges = func.dfg.insts[trap_guard]
                                .branch_destination(&func.dfg.jump_tables);
                            let copy_edges = func.dfg.insts[copy_guard]
                                .branch_destination(&func.dfg.jump_tables);
                            if status_edges.get(1).is_none_or(|edge| {
                                !dom.dominates(
                                    edge.block(&func.dfg.value_lists),
                                    trap_load,
                                    &func.layout,
                                )
                            }) || trap_edges.get(1).is_none_or(|edge| {
                                !dom.dominates(
                                    edge.block(&func.dfg.value_lists),
                                    member_load,
                                    &func.layout,
                                )
                            }) || copy_edges.first().is_none_or(|edge| {
                                !dom.dominates(
                                    edge.block(&func.dfg.value_lists),
                                    inst,
                                    &func.layout,
                                )
                            }) {
                                return Err(error(
                                    "copy-up took a failed status, Trap or member branch",
                                ));
                            }
                        }
                    }
                }
                // An indirect pointer derived from the frame could hide a
                // record write from the offset scan. Refuse such a derivation.
                _ => return Err(error("the pending frame pointer has an untracked use")),
            }
        }
        if !expected.is_empty() {
            return Err(error("a recorded pending-Vis store was not emitted"));
        }
        // A ledger of stores alone cannot detect omitting both a store and
        // its fact. Start from each call site, then demand every common field
        // of its two distinct planner regions plus the member discriminant.
        let positions = |start: i32, bytes: i32| -> Result<BTreeSet<i32>, CraneliftBackendError> {
            if bytes < 0 || bytes % 8 != 0 {
                return Err(error("a copy site has an invalid record width"));
            }
            (0..bytes / 8)
                .map(|index| {
                    start
                        .checked_add(index * 8)
                        .ok_or_else(|| capacity_error("a copy site offset exhausted"))
                })
                .collect()
        };
        for site in sites {
            let source_end = i32::try_from(site.source.frame_bytes)
                .map_err(|_| capacity_error("a copy source frame exceeds addressable range"))?;
            let dest_end = i32::try_from(site.destination.frame_bytes).map_err(|_| {
                capacity_error("a copy destination frame exceeds addressable range")
            })?;
            let mut required = BTreeSet::from([site.destination.discriminant]);
            required.extend(positions(
                site.destination.captures,
                (site.source.continuation_inputs - site.source.captures)
                    .min(site.destination.continuation_inputs - site.destination.captures),
            )?);
            required.extend(positions(
                site.destination.continuation_inputs,
                (source_end - site.source.continuation_inputs)
                    .min(dest_end - site.destination.continuation_inputs),
            )?);
            if copied.get(&(site.call, site.copy_guard)) != Some(&required) {
                return Err(error("copy-up omitted or added a planned record field"));
            }
        }
        let protocol = self
            .pending_vis_record_protocol
            .as_ref()
            .ok_or_else(|| error("a producer has no checked returned-Vis plan"))?;
        let mut seen_members = BTreeSet::new();
        for site in producers {
            if !seen_members.insert(site.member) {
                return Err(error("a producer's member store was recorded twice"));
            }
            let mut rows = protocol.members.values().filter(|row| row.id() == site.row);
            let row = rows
                .next()
                .ok_or_else(|| error("a producer has no exact successor row"))?;
            if rows.next().is_some() {
                return Err(error("a producer's successor row is ambiguous"));
            }
            let member = facts
                .iter()
                .find(|fact| fact.inst == site.member)
                .ok_or_else(|| error("a producer omitted its member store"))?;
            if member.offset != frame.region.discriminant
                || !matches!(member.source, PendingVisStoreSource::Member(id) if id == site.row)
            {
                return Err(error("a producer changed its member store"));
            }
            let capture_offsets = row
                .captures()
                .iter()
                .enumerate()
                .map(|(index, _)| (frame.region.captures, index))
                .chain(
                    row.continuation_inputs()
                        .iter()
                        .enumerate()
                        .map(|(index, _)| (frame.region.continuation_inputs, index)),
                )
                .map(|(start, index)| {
                    let delta = index
                        .checked_mul(8)
                        .and_then(|value| i32::try_from(value).ok())
                        .ok_or_else(|| error("a producer capture offset exhausted"))?;
                    start
                        .checked_add(delta)
                        .ok_or_else(|| error("a producer capture offset exhausted"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            if site.capture_stores.len() != capture_offsets.len() {
                return Err(error("a producer omitted a planned record field"));
            }
            for (&inst, expected_offset) in site.capture_stores.iter().zip(capture_offsets) {
                let fact = facts
                    .iter()
                    .find(|fact| fact.inst == inst)
                    .ok_or_else(|| error("a producer omitted a planned record field"))?;
                if fact.offset != expected_offset
                    || !matches!(fact.source, PendingVisStoreSource::Captured(_))
                    || !dom.dominates(inst, site.member, &func.layout)
                {
                    return Err(error(
                        "a producer changed a planned field or wrote after its member",
                    ));
                }
            }
        }
        Ok(())
    }

    fn record_finished_grafted_spine_function(
        &mut self,
        function: &Function,
        bundle: &UnitBundle,
    ) -> Result<(), CraneliftBackendError> {
        self.verify_pending_vis_slot_stores(function)?;
        let scope = self.function_local.grafted_spine_scope.ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "a generated unit finished without a typed grafted-spine scope".to_string(),
            )
        })?;
        let calls = std::mem::take(&mut self.function_local.grafted_spine_calls);
        let terminals = std::mem::take(&mut self.function_local.grafted_spine_terminals);
        self.grafted_spine_builder
            .as_mut()
            .ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "a generated unit finished after its grafted-spine builder closed".to_string(),
                )
            })?
            .observe_emitted_function(scope, function, calls, terminals, |function| {
                bundle.grafted_spine_scope(function)
            })
    }

    pub(super) fn finish_grafted_spine_control_graph(
        &mut self,
    ) -> Result<(), CraneliftBackendError> {
        let builder = self.grafted_spine_builder.take().ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "the grafted-spine builder was already closed".to_string(),
            )
        })?;
        let (graph, validation_inputs) = builder.finish()?;
        #[cfg(any(test, feature = "px8-ds-test-support"))]
        graph.observe_actual_lowering_validation(validation_inputs);
        #[cfg(not(any(test, feature = "px8-ds-test-support")))]
        drop(validation_inputs);
        self.grafted_spine_graph = Some(graph);
        Ok(())
    }
}

/// **`RT-FNSPLIT-B2F` `D4` — every cross-owner call edge, resolved to the target
/// function the bundle declared for it.**
///
/// ⭐ **Keyed by the planner's `PredeclaredFunctionId`, resolved to a `FuncId`,
/// and derived from nothing else.** The ordinal `declare_unit_bundle` used to
/// spell a symbol name never enters here; a call edge names its callee by static
/// identity and the bundle answers with the declared target or with `None`.
pub(in crate::cranelift_backend) struct CallEdgeTargets {
    edges: Vec<(PredeclaredFunctionId, ResolvedUnitTarget)>,
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetainedUnitCallTargetMutation {
    Exact,
    SubstituteUnrelatedOwnerRoot,
    SuppressGraphClaims,
    SubstituteWrongTarget,
    DuplicateTargetClaim,
}

#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static RETAINED_UNIT_CALL_TARGET_MUTATION:
        std::cell::Cell<RetainedUnitCallTargetMutation> =
            const { std::cell::Cell::new(RetainedUnitCallTargetMutation::Exact) };
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_retained_unit_call_target_mutation<T>(
    mutation: RetainedUnitCallTargetMutation,
    f: impl FnOnce() -> T,
) -> T {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            RETAINED_UNIT_CALL_TARGET_MUTATION
                .with(|active| active.set(RetainedUnitCallTargetMutation::Exact));
        }
    }

    RETAINED_UNIT_CALL_TARGET_MUTATION.with(|active| active.set(mutation));
    let _restore = Restore;
    f()
}

#[cfg(feature = "px8-ds-test-support")]
pub fn retained_unit_call_target_mutation_is_exact() -> bool {
    RETAINED_UNIT_CALL_TARGET_MUTATION.with(std::cell::Cell::get)
        == RetainedUnitCallTargetMutation::Exact
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StaticResponseCallerRetargetMutation {
    RestoreSelectedKTarget,
    /// Only the exact pre-schema relay-promoted source edge loses its owner.
    RestoreRelayPromotedKTarget,
    RemoveSelectedCaller,
    RetargetToDifferentResponseOwner,
}

/// Test-only isolation of the generated-Result proof graph from the later
/// selected-incoming-owner closeout. No obligations or certificates are changed.
#[cfg(any(test, feature = "px8-ds-test-support"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResultContractLayerMutation {
    SuppressMissingContractRefusal,
}

/// Population-side mutations at the specialized response-owner body seam.
/// Each variant changes one emitted relation while leaving the typed plan
/// intact, so the finished-function verifier must reject it.
#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StaticResponseOwnerBodyMutation {
    SubstituteContextZero,
    ResponseWithOperation,
    ResponseWithPriorResponse,
    ResponseWithApplicationEnvironment,
    RawHostResultEscape,
    CallRawWorker,
    OmitKCall,
    DuplicateKCall,
    CallBeforeHostValidation,
    LoadPlaceholderIntoK,
    CallAfterAnswerCollapse,
    BypassTrapBeforeResult,
    VaryRet,
    /// Test-only: inject a Vis carrier past finished-body verification. A
    /// lawful returned member is driven by the owner loop; a separate unknown-
    /// member ingress is needed to witness the downstream C2 backstop.
    BypassRetValidationAndReturnVis,
    OmitOwnerDefinition,
}

/// Construct the same out-of-set member on both sides of the loop guard.
/// Only `BypassToC2` disables the loop's unknown-member exit; `Refuse` leaves
/// it intact and witnesses its native failure independently.
#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PendingVisNonmemberMutation {
    Refuse,
    BypassToC2,
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeneratedResultPathProofMutation {
    Exact,
    SubstituteTagHelper,
    SubstituteQueriedWord,
    SubstituteArena,
    SubstituteOutputSlot,
    OmitHelperStatusCheck,
    ClobberHeader,
    ClobberOutput,
    AddGenuineVisPredecessor,
    ComparisonConstant,
    ComparisonPolarity,
    DestinationOrdinal,
    FunctionIdentity,
    DropRootProof,
    BootstrapCycle,
    DisableCutDetector,
}

#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static GENERATED_RESULT_PATH_PROOF_MUTATION:
        std::cell::Cell<GeneratedResultPathProofMutation> =
        const { std::cell::Cell::new(GeneratedResultPathProofMutation::Exact) };
    static GENERATED_RESULT_PATH_PROOF_MUTATION_APPLICATIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
    static STATIC_RESPONSE_CALLER_RETARGET_MUTATION:
        std::cell::Cell<Option<StaticResponseCallerRetargetMutation>> =
            const { std::cell::Cell::new(None) };
    static STATIC_RESPONSE_CALLER_RETARGET_APPLICATIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

#[cfg(any(test, feature = "px8-ds-test-support"))]
thread_local! {
    static RESULT_CONTRACT_LAYER_MUTATION:
        std::cell::Cell<Option<ResultContractLayerMutation>> =
            const { std::cell::Cell::new(None) };
    static RESULT_CONTRACT_LAYER_MUTATION_APPLICATIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static STATIC_RESPONSE_OWNER_BODY_MUTATION:
        std::cell::Cell<Option<StaticResponseOwnerBodyMutation>> =
        const { std::cell::Cell::new(None) };
    static STATIC_RESPONSE_OWNER_BODY_MUTATION_APPLICATIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
    static PENDING_VIS_NONMEMBER_MUTATION:
        std::cell::Cell<Option<PendingVisNonmemberMutation>> =
        const { std::cell::Cell::new(None) };
    static PENDING_VIS_NONMEMBER_MUTATION_APPLICATIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_generated_result_path_proof_mutation<T>(
    mutation: GeneratedResultPathProofMutation,
    operation: impl FnOnce() -> T,
) -> (T, usize) {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            GENERATED_RESULT_PATH_PROOF_MUTATION
                .with(|slot| slot.set(GeneratedResultPathProofMutation::Exact));
        }
    }
    GENERATED_RESULT_PATH_PROOF_MUTATION.with(|slot| {
        assert_eq!(
            slot.replace(mutation),
            GeneratedResultPathProofMutation::Exact
        );
    });
    GENERATED_RESULT_PATH_PROOF_MUTATION_APPLICATIONS.with(|count| count.set(0));
    let _restore = Restore;
    let result = operation();
    let applications = GENERATED_RESULT_PATH_PROOF_MUTATION_APPLICATIONS.with(std::cell::Cell::get);
    (result, applications)
}

#[cfg(feature = "px8-ds-test-support")]
fn claim_generated_result_path_proof_mutation() -> Option<GeneratedResultPathProofMutation> {
    GENERATED_RESULT_PATH_PROOF_MUTATION.with(|slot| {
        let mutation = slot.get();
        if mutation == GeneratedResultPathProofMutation::Exact
            || GENERATED_RESULT_PATH_PROOF_MUTATION_APPLICATIONS.with(|count| count.get() != 0)
        {
            return None;
        }
        GENERATED_RESULT_PATH_PROOF_MUTATION_APPLICATIONS.with(|count| count.set(1));
        Some(mutation)
    })
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_static_response_caller_retarget_mutation<T>(
    mutation: StaticResponseCallerRetargetMutation,
    operation: impl FnOnce() -> T,
) -> (T, usize) {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            STATIC_RESPONSE_CALLER_RETARGET_MUTATION.with(|slot| slot.set(None));
        }
    }
    STATIC_RESPONSE_CALLER_RETARGET_MUTATION.with(|slot| {
        assert_eq!(slot.replace(Some(mutation)), None);
    });
    STATIC_RESPONSE_CALLER_RETARGET_APPLICATIONS.with(|count| count.set(0));
    let _restore = Restore;
    let result = operation();
    let applications = STATIC_RESPONSE_CALLER_RETARGET_APPLICATIONS.with(std::cell::Cell::get);
    (result, applications)
}

#[cfg(feature = "px8-ds-test-support")]
pub fn static_response_caller_retarget_mutation_is_exact() -> bool {
    STATIC_RESPONSE_CALLER_RETARGET_MUTATION
        .with(std::cell::Cell::get)
        .is_none()
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_result_contract_layer_mutation<T>(
    mutation: ResultContractLayerMutation,
    operation: impl FnOnce() -> T,
) -> (T, usize) {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            RESULT_CONTRACT_LAYER_MUTATION.with(|slot| slot.set(None));
        }
    }
    RESULT_CONTRACT_LAYER_MUTATION.with(|slot| assert_eq!(slot.replace(Some(mutation)), None));
    RESULT_CONTRACT_LAYER_MUTATION_APPLICATIONS.with(|count| count.set(0));
    let _restore = Restore;
    let result = operation();
    let applications = RESULT_CONTRACT_LAYER_MUTATION_APPLICATIONS.with(std::cell::Cell::get);
    (result, applications)
}

#[cfg(feature = "px8-ds-test-support")]
pub fn result_contract_layer_mutation_is_exact() -> bool {
    RESULT_CONTRACT_LAYER_MUTATION
        .with(std::cell::Cell::get)
        .is_none()
}

#[cfg(any(test, feature = "px8-ds-test-support"))]
fn suppress_missing_result_contract_refusal() -> bool {
    RESULT_CONTRACT_LAYER_MUTATION.with(|slot| match slot.get() {
        None => false,
        Some(ResultContractLayerMutation::SuppressMissingContractRefusal) => {
            RESULT_CONTRACT_LAYER_MUTATION_APPLICATIONS.with(|count| count.set(count.get() + 1));
            true
        }
    })
}

#[cfg(not(any(test, feature = "px8-ds-test-support")))]
fn suppress_missing_result_contract_refusal() -> bool {
    false
}

#[cfg(feature = "px8-ds-test-support")]
#[derive(Clone, Debug)]
pub struct RelayOwnerCoverageRefusalObservation {
    pub source: u32,
    pub source_lowered: bool,
    pub selected_caller: String,
}

#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static RELAY_OWNER_COVERAGE_REFUSALS:
        std::cell::RefCell<Option<Vec<RelayOwnerCoverageRefusalObservation>>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_relay_owner_coverage_refusal_observations<T>(
    operation: impl FnOnce() -> T,
) -> (T, Vec<RelayOwnerCoverageRefusalObservation>) {
    RELAY_OWNER_COVERAGE_REFUSALS.with(|slot| {
        assert!(
            slot.borrow().is_none(),
            "relay coverage observations cannot nest"
        );
        *slot.borrow_mut() = Some(Vec::new());
    });
    let result = operation();
    let observations = RELAY_OWNER_COVERAGE_REFUSALS.with(|slot| slot.borrow_mut().take().unwrap());
    (result, observations)
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_static_response_owner_body_mutation<T>(
    mutation: StaticResponseOwnerBodyMutation,
    operation: impl FnOnce() -> T,
) -> (T, usize) {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            STATIC_RESPONSE_OWNER_BODY_MUTATION.with(|slot| slot.set(None));
        }
    }
    STATIC_RESPONSE_OWNER_BODY_MUTATION.with(|slot| {
        assert_eq!(slot.replace(Some(mutation)), None);
    });
    STATIC_RESPONSE_OWNER_BODY_MUTATION_APPLICATIONS.with(|count| count.set(0));
    let _restore = Restore;
    let result = operation();
    let applications = STATIC_RESPONSE_OWNER_BODY_MUTATION_APPLICATIONS.with(std::cell::Cell::get);
    (result, applications)
}

#[cfg(feature = "px8-ds-test-support")]
pub fn static_response_owner_body_mutation_is_exact() -> bool {
    STATIC_RESPONSE_OWNER_BODY_MUTATION
        .with(std::cell::Cell::get)
        .is_none()
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_pending_vis_nonmember_mutation<T>(
    mutation: PendingVisNonmemberMutation,
    operation: impl FnOnce() -> T,
) -> (T, usize) {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            PENDING_VIS_NONMEMBER_MUTATION.with(|slot| slot.set(None));
        }
    }
    PENDING_VIS_NONMEMBER_MUTATION.with(|slot| {
        assert_eq!(slot.replace(Some(mutation)), None);
    });
    PENDING_VIS_NONMEMBER_MUTATION_APPLICATIONS.with(|count| count.set(0));
    let _restore = Restore;
    let result = operation();
    let applications = PENDING_VIS_NONMEMBER_MUTATION_APPLICATIONS.with(std::cell::Cell::get);
    (result, applications)
}

#[cfg(feature = "px8-ds-test-support")]
fn claim_pending_vis_nonmember_mutation(eligible: bool) -> Option<PendingVisNonmemberMutation> {
    PENDING_VIS_NONMEMBER_MUTATION.with(|slot| {
        let mutation = slot.get()?;
        if !eligible || PENDING_VIS_NONMEMBER_MUTATION_APPLICATIONS.with(|count| count.get() != 0) {
            return None;
        }
        PENDING_VIS_NONMEMBER_MUTATION_APPLICATIONS.with(|count| count.set(1));
        Some(mutation)
    })
}

// Test-only E4 observation: the response capture value is loaded by the
// declared owner-frame slot. Its source coordinate is metadata, not the key
// selecting the value. Record only pairs that reach a two-parameter source.
#[cfg(feature = "px8-ds-test-support")]
thread_local! {
    static RT_SEED_RESPONSE_OBSERVING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static RT_SEED_RESPONSE_PAIRS: std::cell::RefCell<Vec<(u32, u32, i32)>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(feature = "px8-ds-test-support")]
pub fn with_rt_seed_response_metadata_observations<T>(
    operation: impl FnOnce() -> T,
) -> (T, Vec<(u32, u32, i32)>) {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            RT_SEED_RESPONSE_OBSERVING.with(|enabled| enabled.set(false));
        }
    }
    RT_SEED_RESPONSE_OBSERVING.with(|enabled| assert!(!enabled.replace(true)));
    RT_SEED_RESPONSE_PAIRS.with(|pairs| pairs.borrow_mut().clear());
    let _restore = Restore;
    let result = operation();
    let pairs = RT_SEED_RESPONSE_PAIRS.with(|pairs| std::mem::take(&mut *pairs.borrow_mut()));
    (result, pairs)
}

#[cfg(feature = "px8-ds-test-support")]
fn claim_static_response_owner_body_mutation(
    predicate: impl FnOnce(StaticResponseOwnerBodyMutation) -> bool,
) -> Option<StaticResponseOwnerBodyMutation> {
    STATIC_RESPONSE_OWNER_BODY_MUTATION.with(|slot| {
        let mutation = slot.get()?;
        let already_applied =
            STATIC_RESPONSE_OWNER_BODY_MUTATION_APPLICATIONS.with(|count| count.get() != 0);
        if already_applied || !predicate(mutation) {
            return None;
        }
        STATIC_RESPONSE_OWNER_BODY_MUTATION_APPLICATIONS.with(|count| count.set(1));
        Some(mutation)
    })
}

#[derive(Clone)]
struct RetainedBodyTargetClaim {
    body: StaticOriginId,
    target: ResolvedUnitTarget,
}

impl CallEdgeTargets {
    /// The resolved targets of every call emitted **into** `caller`.
    ///
    /// ⚠ Returns an empty iterator for a unit with no outgoing call edges, which
    /// is the common case: most units are leaves.
    pub(in crate::cranelift_backend) fn targets_in(
        &self,
        caller: PredeclaredFunctionId,
    ) -> impl Iterator<Item = &ResolvedUnitTarget> + '_ {
        self.edges
            .iter()
            .filter(move |(from, _)| *from == caller)
            .map(|(_, target)| target)
    }

    /// How many call edges were resolved.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn len(&self) -> usize {
        self.edges.len()
    }
}

#[derive(Clone)]
pub(in crate::cranelift_backend) struct ResolvedUnitTarget {
    function: FuncId,
    /// The planner's typed callee identity. Retained so a generated context can
    /// follow the resolved call graph without recovering ownership from an
    /// origin or a declared function number.
    callee: PredeclaredFunctionId,
    origin: StaticOriginId,
    call_site_origin: StaticOriginId,
    kind: EmittableCallKind,
    header: AbiFrameHeader,
    slots: Vec<AbiSlot>,
    offsets: Vec<u32>,
}

/// A declared Result obligation for an existing typed unit. Presence means the
/// finished graph must prove this identity; it is never itself a certificate.
#[derive(Clone, Copy)]
pub(in crate::cranelift_backend) struct DeclaredResultContract {
    identity: ConstructorIdentity,
}

impl DeclaredResultContract {
    fn new(identity: ConstructorIdentity) -> Self {
        Self { identity }
    }

    pub(super) fn identity(self) -> ConstructorIdentity {
        self.identity
    }
}

#[derive(Clone)]
pub(in crate::cranelift_backend) struct DeclaredUnitCall {
    pub(in crate::cranelift_backend) function: FuncRef,
    /// The exact emitted function identity, never inferred from an origin or
    /// destination ABI. Only a planner-marked protocol chain has a frame tail.
    pub(in crate::cranelift_backend) frame_owner: Option<PendingVisFrameOwner>,
    /// The callee's scheduling entry -- `RT-CONTSPEC-ACTIVATE` `D1b` keeps
    /// this as the target origin.
    pub(in crate::cranelift_backend) origin: StaticOriginId,
    /// The callable source body this call is keyed by. Both ends are retained
    /// in the declared record, not just the map key, so a consumer can check
    /// the pair rather than infer it.
    pub(in crate::cranelift_backend) call_site_origin: StaticOriginId,
    pub(in crate::cranelift_backend) header: AbiFrameHeader,
    pub(in crate::cranelift_backend) slots: Vec<AbiSlot>,
    pub(in crate::cranelift_backend) offsets: Vec<u32>,
    pub(in crate::cranelift_backend) result_contract: Option<DeclaredResultContract>,
}

pub(in crate::cranelift_backend) struct DeclaredUnitCalls {
    pub(in crate::cranelift_backend) static_bodies: BTreeMap<StaticOriginId, DeclaredUnitCall>,
    pub(in crate::cranelift_backend) declarations: BTreeMap<StaticOriginId, DeclaredUnitCall>,
}

impl CallEdgeTargets {
    pub(in crate::cranelift_backend) fn declare_in_func<M: Module>(
        &self,
        caller: PredeclaredFunctionId,
        module: &mut M,
        func: &mut Function,
    ) -> Result<DeclaredUnitCalls, CraneliftBackendError> {
        let mut static_bodies = BTreeMap::new();
        let mut declarations = BTreeMap::new();
        for target in self.targets_in(caller) {
            let call = DeclaredUnitCall {
                function: module.declare_func_in_func(target.function, func),
                frame_owner: Some(PendingVisFrameOwner::Unit(target.callee)),
                origin: target.origin,
                call_site_origin: target.call_site_origin,
                header: target.header,
                slots: target.slots.clone(),
                offsets: target.offsets.clone(),
                result_contract: None,
            };
            let (calls, duplicate) = match target.kind {
                EmittableCallKind::StaticBody => (
                    &mut static_bodies,
                    "one caller has two static-body calls to the same body origin",
                ),
                EmittableCallKind::Declaration => (
                    &mut declarations,
                    "one declaration-reference occurrence has two planner-derived call targets",
                ),
            };
            if calls.insert(target.call_site_origin, call).is_some() {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    duplicate.to_string(),
                ));
            }
        }
        Ok(DeclaredUnitCalls {
            static_bodies,
            declarations,
        })
    }

    #[cfg(feature = "px8-ds-test-support")]
    fn substitute_unrelated_owner_roots(
        &self,
        root: PredeclaredFunctionId,
    ) -> Result<Vec<PredeclaredFunctionId>, CraneliftBackendError> {
        let mut pending = vec![root];
        let mut reachable = BTreeSet::new();
        while let Some(owner) = pending.pop() {
            if !reachable.insert(owner) {
                continue;
            }
            pending.extend(
                self.targets_in(owner)
                    .filter(|target| target.kind == EmittableCallKind::StaticBody)
                    .map(|target| target.callee),
            );
        }

        // Select by the typed graph relation, never by an id, iteration order,
        // retained-body shape, or a later lookup miss. More than one candidate is
        // ambiguity and stays a refusal (choosing by preference or order is
        // forbidden).
        let candidates = self
            .edges
            .iter()
            .filter_map(|(owner, target)| {
                (target.kind == EmittableCallKind::StaticBody && !reachable.contains(owner))
                    .then_some(*owner)
            })
            .collect::<BTreeSet<_>>();
        match candidates.len() {
            0 => {
                // RT-SSA HS2 relocation (Architect re-rule evt_2427xbynt1d2e /
                // evt_73h2ayt1acw9). A program whose every composed-return response
                // fell back to main lowering (the deferred D3 frontier, e.g.
                // checked writeAll) has NO specialization owner, so there is no
                // unrelated legal static-body owner to substitute. That is the
                // correct arm-(b) consequence of the Q1 fall-through, not a
                // production defect (this helper is px8-ds-test-support only). The
                // mutation is still exercised as its limiting case: an empty
                // traversal root is the maximally-unrelated substitute, and it
                // leaves the retained body with no graph-derived call target in the
                // generated unit exactly as a concrete unrelated root would (the
                // rejection fires downstream in `call_declared_unit`, byte-identical
                // to the sibling suppress-graph-claims control). It is NOT a bare
                // skip: the retained-body-target rejection is still proven.
                eprintln!(
                    "retained-unit root control replaced checked root {root:?} with an empty \
                     traversal: no unrelated legal static-body owner exists (every composed \
                     response fell back to main lowering)"
                );
                Ok(Vec::new())
            }
            1 => {
                let unrelated = candidates
                    .into_iter()
                    .next()
                    .expect("the singleton unrelated-owner set is nonempty");
                eprintln!(
                    "retained-unit root control replaced checked root {root:?} with unrelated \
                     legal static-body owner {unrelated:?}"
                );
                Ok(vec![unrelated])
            }
            found => Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "retained-unit root control requires at most one unrelated legal static-body \
                 owner, found {found}"
                ),
            )),
        }
    }

    /// Declare the exact static-body graph targets transitively reachable from
    /// a generated context's checked raw owner into its specialization-owned
    /// function.
    ///
    /// The source-body binding on each resolved graph edge is the authority for
    /// the map key, and the graph edge's typed callee is the only way the walk
    /// advances. The retained closure's body origin is not consulted here, so a
    /// later lookup miss cannot create a target. Multiple graph claims for one
    /// body, or a target that disagrees with its claim's body, reject before any
    /// body emission starts.
    fn declare_retained_body_targets_in_func<M: Module>(
        &self,
        root: PredeclaredFunctionId,
        module: &mut M,
        func: &mut Function,
        declared: &mut BTreeMap<StaticOriginId, DeclaredUnitCall>,
    ) -> Result<(), CraneliftBackendError> {
        // The generated context's checked raw owner is the root. Follow only
        // StaticBody edges transitively reachable from it, using the typed
        // callee carried by graph resolution. This is not a global body lookup:
        // an edge owned by an unrelated source subtree never enters the set.
        #[cfg(feature = "px8-ds-test-support")]
        let traversal_roots = match RETAINED_UNIT_CALL_TARGET_MUTATION.with(std::cell::Cell::get) {
            RetainedUnitCallTargetMutation::SubstituteUnrelatedOwnerRoot => {
                self.substitute_unrelated_owner_roots(root)?
            }
            _ => vec![root],
        };
        #[cfg(not(feature = "px8-ds-test-support"))]
        let traversal_roots = vec![root];
        let mut pending = traversal_roots;
        let mut visited = BTreeSet::new();
        #[cfg_attr(
            not(feature = "px8-ds-test-support"),
            expect(unused_mut, reason = "test-support controls mutate resolved claims")
        )]
        let mut claims = Vec::new();
        while let Some(owner) = pending.pop() {
            if !visited.insert(owner) {
                continue;
            }
            for target in self.targets_in(owner) {
                if target.kind != EmittableCallKind::StaticBody {
                    continue;
                }
                claims.push(RetainedBodyTargetClaim {
                    body: target.call_site_origin,
                    target: target.clone(),
                });
                pending.push(target.callee);
            }
        }

        // Controls perturb the already-resolved graph claims, before they are
        // declared into the generated function. They never inspect a lookup
        // miss and never alter the source plan.
        #[cfg(feature = "px8-ds-test-support")]
        match RETAINED_UNIT_CALL_TARGET_MUTATION.with(std::cell::Cell::get) {
            RetainedUnitCallTargetMutation::Exact
            | RetainedUnitCallTargetMutation::SubstituteUnrelatedOwnerRoot => {}
            RetainedUnitCallTargetMutation::SuppressGraphClaims => claims.clear(),
            RetainedUnitCallTargetMutation::SubstituteWrongTarget => {
                if claims.len() >= 2 {
                    let substitute = claims[1].target.clone();
                    claims[0].target = substitute;
                }
            }
            RetainedUnitCallTargetMutation::DuplicateTargetClaim => {
                if claims.len() >= 2 {
                    let mut duplicate = claims[1].target.clone();
                    duplicate.call_site_origin = claims[0].body;
                    claims.push(RetainedBodyTargetClaim {
                        body: claims[0].body,
                        target: duplicate,
                    });
                }
            }
        }

        let mut unique = BTreeMap::new();
        for claim in claims {
            if claim.body != claim.target.call_site_origin {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    format!(
                        "a retained-body graph claim for {:?} resolved to a target bound to {:?}; a \
                     target for another body cannot satisfy this claim",
                        claim.body, claim.target.call_site_origin
                    ),
                ));
            }
            if unique.insert(claim.body, claim.target).is_some() {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    format!(
                        "retained body {:?} has more than one graph-derived call target; choosing one \
                     by preference or iteration order is forbidden",
                        claim.body
                    ),
                ));
            }
        }

        for (body, target) in unique {
            if declared.contains_key(&body) {
                continue;
            }
            declared.insert(
                body,
                DeclaredUnitCall {
                    function: module.declare_func_in_func(target.function, func),
                    frame_owner: Some(PendingVisFrameOwner::Unit(target.callee)),
                    origin: target.origin,
                    call_site_origin: target.call_site_origin,
                    header: target.header,
                    slots: target.slots,
                    offsets: target.offsets,
                    result_contract: None,
                },
            );
        }
        Ok(())
    }
}

/// **`D4` — resolve the derived call edges against the declared bundle, BEFORE
/// any body is defined.**
///
/// ⭐ **This runs between `D1`'s declaration pass and `D2`'s definition pass on
/// purpose, and the position is the point.** A call edge whose callee was never
/// forward-declared is a program that cannot be emitted; discovering that while
/// half the bodies are already defined leaves a partially emitted artifact whose
/// failure mode is a link error or worse. ⇒ Resolving the whole edge set first
/// makes the bundle's completeness a **precondition** of definition rather than
/// a discovery during it.
///
/// ⛔ **`None` from the bundle is a hard error and must never be replaced by a
/// fabricated `FuncId`.** That substitution is the exact failure
/// `UnitBundle::function`'s `Option` return type exists to make visible, and it
/// would emit a call to whatever function happened to share the id.
///
/// **MEASURED:** every `StaticBody` edge the planner validated resolves to a
/// target function the bundle declared, and the resolved population equals the
/// derived population.
/// **CLAIMED:** call sites reference target functions by their **static**
/// identity, with no indirect dispatch on a dynamic property and no runtime
/// lookup that re-derives which code to run from a value.
/// **THE GAP:** ⛔ **this resolves the edges; it does not yet EMIT the call
/// instructions.** A unit body today loads its result slot and returns, because
/// body emission does not descend until `S6` switches `lower_expr`'s consumers
/// over. ⇒ Until then this carries **rejection** authority, not emission
/// authority, and ⛔ the claim above is discharged for *resolution* only. The
/// `direct call rather than call_indirect` half of it has no control yet and is
/// **not** claimed.
///
/// ⛔⛔ **AND THE GAP IS WIDER THAN "no call instruction" — MEASURED, NOT
/// ESTIMATED.** Replacing `bundle.function(edge.callee())` with
/// `bundle.function(edge.caller())` — i.e. resolving every call edge to the
/// **calling** unit instead of the called one, the identity-alias defect
/// `UnitBundle`'s doc comment warns against — leaves the **entire suite green**:
/// 498 + 26 + 14, zero failures.
///
/// ⇒ ⭐ **Which unit an edge resolves to is currently unpinned.** The `FuncRef`
/// is declared in the caller's `Function` and never called, so a wrong target is
/// a reference nobody follows. ⛔ **`S6` must not read this as covered.** The
/// control that closes it is a *behavioural* one — a program whose answer
/// depends on which unit ran — and it cannot exist until the call is emitted.
/// ⚠ `the_resolved_call_edge_population_moves_with_the_program` pins the edge
/// **count** and is blind to the edge's **destination**; those are different
/// claims and only the first has a defender today.
/// **`D4` -- the static-body units, projected by exact body origin.**
///
/// This is a **projection of `emittable_units`**, not a new unit or call-edge
/// population: every entry is a unit the planner already emitted and the
/// bundle already declared, re-keyed by the body origin a static worker
/// binding names. Nothing here mints a unit, an edge, or a descriptor.
///
/// A body origin that appears twice is rejected rather than resolved by
/// last-writer, because a duplicate means two units claim one body and the
/// binding could not name either unambiguously.
pub(in crate::cranelift_backend) struct WorkerTargets {
    /// The EXECUTABLE targets: bodies with a declared `Function` to call.
    by_origin: BTreeMap<StaticOriginId, ResolvedUnitTarget>,
    /// **`D5a` checkpoint 1 -- the TEMPLATE population, over every emittable
    /// unit including the template-only ones.**
    ///
    /// Architect ruling `evt_5a0q3m9tnkh8e`: *"the constructor's raw
    /// identity/arity validation must be separate from the generated context
    /// `FuncRef` used by the call."* This map is that separation made
    /// structural -- it carries the descriptor facts and **no `FuncRef` at
    /// all**, so a template-only body can be validated against its own raw
    /// contract by code that has no way to call it.
    templates: BTreeMap<StaticOriginId, WorkerTemplate>,
}

/// **`D5a` checkpoint 1 -- one raw worker body's descriptor contract.**
///
/// ⛔ Deliberately has no `function: FuncRef` field. `construct_static_worker_
/// binding` reads only `call_site_origin`, `header`, `slots` and `offsets`, so
/// removing the callee from the record it validates against makes "validated
/// the raw contract, called the generated context" a fact about the types
/// rather than a discipline someone has to remember.
#[derive(Clone)]
pub(in crate::cranelift_backend) struct WorkerTemplate {
    pub(in crate::cranelift_backend) origin: StaticOriginId,
    pub(in crate::cranelift_backend) call_site_origin: StaticOriginId,
    pub(in crate::cranelift_backend) header: AbiFrameHeader,
    pub(in crate::cranelift_backend) slots: Vec<AbiSlot>,
    pub(in crate::cranelift_backend) offsets: Vec<u32>,
}

impl WorkerTargets {
    /// Declare every projected target **into one generated function**, and
    /// hand back that function's own `DeclaredUnitCall`s.
    ///
    /// The `FuncRef`s produced here belong to `func` alone. They are minted
    /// per function and never copied between functions -- which is why the
    /// binding stores origins and not a `FuncRef` (`D4`).
    ///
    /// This is also the operation a separately emitted caller uses: it takes
    /// any `Function`, so a caller emitted outside the main loop declares its
    /// own refs through the same route rather than borrowing another's.
    pub(in crate::cranelift_backend) fn declare_in_func<M: Module>(
        &self,
        module: &mut M,
        func: &mut Function,
    ) -> BTreeMap<StaticOriginId, DeclaredUnitCall> {
        self.by_origin
            .iter()
            .map(|(origin, target)| {
                (
                    *origin,
                    DeclaredUnitCall {
                        function: module.declare_func_in_func(target.function, func),
                        frame_owner: Some(PendingVisFrameOwner::Unit(target.callee)),
                        origin: target.origin,
                        call_site_origin: target.call_site_origin,
                        header: target.header,
                        slots: target.slots.clone(),
                        offsets: target.offsets.clone(),
                        result_contract: None,
                    },
                )
            })
            .collect()
    }

    /// The raw template contract for every emittable body, executable or not.
    pub(in crate::cranelift_backend) fn templates(
        &self,
    ) -> &BTreeMap<StaticOriginId, WorkerTemplate> {
        &self.templates
    }
}

/// Project the already-validated emittable units by exact body origin.
pub(in crate::cranelift_backend) fn resolve_worker_targets(
    plan: &StaticTransitionPlan<'_>,
    bundle: &UnitBundle,
) -> Result<WorkerTargets, CraneliftBackendError> {
    let mut templates: BTreeMap<StaticOriginId, WorkerTemplate> = BTreeMap::new();
    // The TEMPLATE population is every emittable unit -- `D5a` checkpoint 1
    // keeps the raw worker's descriptor and source binding whether or not it
    // still receives a `Function`.
    // `D5a` checkpoint 4 step 3 -- the reaching mutation for "unchanged raw
    // worker ABI". Reading the EXECUTABLE population here instead of the
    // emittable one is the "template-only means deleted" mistake: it drops the
    // superseded body's descriptor while every consumer of that descriptor
    // remains. ⛔ The two populations are otherwise identical on a program with
    // no retarget, which is why this only became measurable at checkpoint 4.
    #[cfg(test)]
    let template_population = if crate::cranelift_backend::lowering::d5a_route_mutation()
        == crate::cranelift_backend::lowering::D5aRouteMutation::DropSupersededWorkerTemplates
    {
        crate::cranelift_backend::lowering::record_d5a_route_application();
        plan.executable_units()?
    } else {
        plan.emittable_units()?
    };
    #[cfg(not(test))]
    let template_population = plan.emittable_units()?;
    for unit in template_population {
        let (offsets, frame_bytes) = unit.slot_offsets()?;
        if frame_bytes != unit.header().frame_bytes {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "worker template frame size disagrees with its slot run".to_string(),
            ));
        }
        let origin = unit.body_occurrence();
        if templates
            .insert(
                origin,
                WorkerTemplate {
                    origin,
                    call_site_origin: origin,
                    header: unit.header(),
                    slots: unit.slots().to_vec(),
                    offsets,
                },
            )
            .is_some()
        {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "two emittable units claim the same body origin, so no worker template could \
                 name either unambiguously"
                    .to_string(),
            ));
        }
    }
    let mut by_origin: BTreeMap<StaticOriginId, ResolvedUnitTarget> = BTreeMap::new();
    for unit in plan.executable_units()? {
        let function = bundle.function(unit.function()).ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "a planned unit has no forward-declared function to project as a worker target"
                    .to_string(),
            )
        })?;
        let (offsets, frame_bytes) = unit.slot_offsets()?;
        if frame_bytes != unit.header().frame_bytes {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "worker target frame size disagrees with its slot run".to_string(),
            ));
        }
        let origin = unit.body_occurrence();
        let target = ResolvedUnitTarget {
            function,
            callee: unit.function(),
            origin,
            call_site_origin: origin,
            kind: EmittableCallKind::StaticBody,
            header: unit.header(),
            slots: unit.slots().to_vec(),
            offsets,
        };
        if by_origin.insert(origin, target).is_some() {
            return Err(backend_module(LoweringCause::PlannerInvariant,
                "two emittable units claim the same body origin, so no worker binding could                  name either unambiguously"
                    .to_string(),
            ));
        }
    }
    Ok(WorkerTargets {
        by_origin,
        templates,
    })
}

/// **`RT-DECL-CLOSURE-PORT` `D5` — the checked-call closeout ledger.**
///
/// One entry per checked same-SCC recursive call that reached a real direct
/// call to a declaration-owned unit, keyed by the `call_template_id` the
/// oriented plan issued.
///
/// ⭐ **The three populations are read independently.** `planned` is the
/// oriented plan's own `recursive_calls` identities; `consumed` is what the
/// affine marker machinery actually took; `emitted` is recorded only after the
/// `Inst` exists, from the emitted call itself. Set equality across the three
/// is what "every planned checked call became exactly one correct direct call,
/// and no other checked call was emitted" means.
///
/// ⛔ **Sets, not counts.** Two populations of the same size can differ, and a
/// length comparison would pass for one that swapped a template for another.
///
/// ⛔ **This closeout restates none of D5's other laws.** Interface, segment,
/// frame-template, occurrence-fingerprint, ABI descriptor, SCC, admission and
/// input-order checks all have their own authorities and keep them; duplicating
/// one here would put a second copy in a file where the two can disagree.
///
/// ⚠ **Scope, stated rather than left to be discovered.** The ledger is opened
/// and closed by `define_unit_bodies`, the sole production body-emission route.
/// It is live production code on the lane it guards, not a test hook.
#[derive(Debug, Default)]
pub(in crate::cranelift_backend) struct CheckedCallLedger {
    planned: BTreeSet<u64>,
    emitted: BTreeMap<u64, CheckedCallRecord>,
}

/// One emitted checked call, bound to the exact occurrence and target the
/// planner resolved for it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct CheckedCallRecord {
    pub(in crate::cranelift_backend) reference: StaticOriginId,
    pub(in crate::cranelift_backend) target: StaticOriginId,
    /// The callee **decoded out of the emitted instruction**.
    pub(in crate::cranelift_backend) callee: cranelift_codegen::ir::FuncRef,
    /// The callee the planner-resolved target record carries. ⚠ Both are
    /// `FuncRef`s minted into the same defining function, so comparing them is
    /// lawful and is a comparison of two independently produced facts — one
    /// read from the CLIF, one from the resolved `DeclaredUnitCall`.
    pub(in crate::cranelift_backend) resolved: cranelift_codegen::ir::FuncRef,
}

impl CheckedCallLedger {
    /// ⚠ `planned` is taken **directly** from `plan.recursive_calls` — that IS
    /// the exact domain of same-SCC checked calls, so no classifier and no
    /// whitelist stands between the plan and this set.
    pub(in crate::cranelift_backend) fn open(
        plan: Option<&crate::OrientedSubcontinuationPlanV1>,
    ) -> Self {
        Self {
            planned: plan
                .map(|plan| {
                    plan.recursive_calls
                        .iter()
                        .map(|call| call.call_template_id)
                        .collect()
                })
                .unwrap_or_default(),
            emitted: BTreeMap::new(),
        }
    }

    /// Record one emitted checked call. ⛔ Called only **after** the `Inst`
    /// exists, so a template reaches this set only once its call is real.
    pub(in crate::cranelift_backend) fn record_emitted(
        &mut self,
        call_template_id: u64,
        record: CheckedCallRecord,
    ) -> Result<(), CraneliftBackendError> {
        if self.emitted.insert(call_template_id, record).is_some() {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "one checked recursive call template emitted more than one declaration-unit call"
                    .to_string(),
            ));
        }
        Ok(())
    }

    /// **Planned = consumed = emitted, before the artifact is published.**
    ///
    /// `consumed` is the affine marker machinery's own set, passed in rather
    /// than mirrored here, so the two cannot drift.
    pub(in crate::cranelift_backend) fn close(
        self,
        consumed: &BTreeSet<u64>,
    ) -> Result<(), CraneliftBackendError> {
        let emitted = self.emitted.keys().copied().collect::<BTreeSet<_>>();
        for (name, set) in [("consumed", consumed), ("emitted", &emitted)] {
            if *set != self.planned {
                let missing = self.planned.difference(set).count();
                let extra = set.difference(&self.planned).count();
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    format!(
                        "the {name} checked recursive call population does not equal the planned one:                      {missing} planned templates absent, {extra} unplanned templates present"
                    ),
                ));
            }
        }
        // Each emitted call's ACTUAL callee against its exact resolved target.
        // ⛔ The callee is the one decoded from the emitted instruction; a
        // tuple that disagrees here is a call that went somewhere the planner
        // did not resolve.
        for (call_template_id, record) in &self.emitted {
            if record.callee != record.resolved {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    format!(
                        "checked recursive call template {call_template_id} emitted a call to \
                     {:?} but the target resolved for occurrence {:?} is {:?} ({:?})",
                        record.callee, record.reference, record.resolved, record.target
                    ),
                ));
            }
        }
        Ok(())
    }
}

pub(in crate::cranelift_backend) fn resolve_call_edges(
    plan: &StaticTransitionPlan<'_>,
    bundle: &UnitBundle,
) -> Result<CallEdgeTargets, CraneliftBackendError> {
    // `D5a` checkpoint 1: the edges that SURVIVE the retarget. An edge into a
    // template-only body is the seeding edge whose realization moved to a
    // generated context; resolving it would demand a `FuncId` for a unit with
    // no emitted `Function`, and `bundle.function`'s `None` is a real answer
    // rather than a prompt to fabricate one.
    let derived = plan.executable_call_edges()?;
    // `RT-CONTSPEC-ACTIVATE` `D1b`: the exact-set source-body binding, joined
    // on validated caller + callee scheduling entry. A `StaticBody` edge's
    // resolved `call_site_origin` becomes the callable SOURCE BODY; the
    // scheduling entry stays the target origin.
    let source_bindings: BTreeMap<(PredeclaredFunctionId, StaticOriginId), StaticOriginId> = plan
        .static_body_source_bindings()?
        .into_iter()
        .map(|(caller, source_body, entry)| ((caller, entry), source_body))
        .collect();
    let mut edges = Vec::with_capacity(derived.len());
    for edge in derived {
        let target = bundle.function(edge.callee()).ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "a call edge names a unit that was never forward-declared".to_string(),
            )
        })?;
        let unit = plan
            .emittable_units()?
            .into_iter()
            .find(|unit| unit.function() == edge.callee())
            .ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "call edge callee has no abi descriptor".to_string(),
                )
            })?;
        if unit.entry_origin() != edge.callee_origin() {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "call edge callee origin disagrees with its abi descriptor".to_string(),
            ));
        }
        let (offsets, frame_bytes) = unit.slot_offsets()?;
        if frame_bytes != unit.header().frame_bytes {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "call edge target frame size disagrees with its slot run".to_string(),
            ));
        }
        edges.push((
            edge.caller(),
            ResolvedUnitTarget {
                function: target,
                callee: edge.callee(),
                origin: edge.callee_origin(),
                call_site_origin: match edge.kind() {
                    EmittableCallKind::StaticBody => *source_bindings
                        .get(&(edge.caller(), edge.callee_origin()))
                        .ok_or_else(|| {
                            backend_module(
                                LoweringCause::PlannerInvariant,
                                "a static body call edge has no D1b source-body binding"
                                    .to_string(),
                            )
                        })?,
                    EmittableCallKind::Declaration => edge.call_site_origin(),
                },
                kind: edge.kind(),
                header: unit.header(),
                slots: unit.slots().to_vec(),
                offsets,
            },
        ));
    }
    #[cfg(test)]
    B2F_CALL_EDGE_RESOLUTION.with(|cell| cell.set(edges.len()));
    Ok(CallEdgeTargets { edges })
}

/// **`RT-DECL-CLOSURE-PORT` `D5`** — one mutation of the function-local
/// declared-call copy, per axis the ABI reconciliation claims to hold.
///
/// ⚠ Every variant leaves the plan's descriptor untouched. `Exact` is the
/// identity, and a control that ran only `Exact` would be asserting its own
/// setup ([[a-mutation-control-with-unwrap-or-exact-is-the-identity]]).
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D5DeclaredCallMutation {
    Exact,
    /// Phase: the word class a slot is carried in.
    Carrier,
    /// Owner: whether the callee owns or borrows the slot.
    Ownership,
    /// Owner: whose storage the slot addresses.
    StorageOwner,
    /// The slot's position within its own kind-run.
    Ordinal,
    /// The declared frame size.
    Header,
    /// Where a slot sits in the frame.
    Offsets,
    /// The call resolves some other unit's record — the wrong-target class.
    Retarget,
}

#[cfg(test)]
thread_local! {
    static D5_DECLARED_CALL_MUTATION: std::cell::Cell<D5DeclaredCallMutation> =
        const { std::cell::Cell::new(D5DeclaredCallMutation::Exact) };
}

/// Run `body` with one declared-call mutation installed, restoring `Exact` on
/// the way out **including on panic** — a control asserts inside, and a leak
/// would silently mutate every later compile on this thread.
#[cfg(test)]
pub(in crate::cranelift_backend) fn with_d5_declared_call_mutation<T>(
    mutation: D5DeclaredCallMutation,
    body: impl FnOnce() -> T,
) -> T {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            D5_DECLARED_CALL_MUTATION.with(|cell| cell.set(D5DeclaredCallMutation::Exact));
        }
    }
    D5_DECLARED_CALL_MUTATION.with(|cell| cell.set(mutation));
    let _restore = Restore;
    body()
}

#[cfg(test)]
fn d5_mutate_declared_calls(calls: &mut BTreeMap<StaticOriginId, DeclaredUnitCall>) {
    let mutation = D5_DECLARED_CALL_MUTATION.with(std::cell::Cell::get);
    if mutation == D5DeclaredCallMutation::Exact {
        return;
    }
    // ⚠ The retarget needs a DIFFERENT record to point at, so it is taken
    // before the loop below borrows the map mutably.
    let other = calls.values().next().cloned();
    for call in calls.values_mut() {
        match mutation {
            D5DeclaredCallMutation::Exact => {}
            D5DeclaredCallMutation::Carrier => {
                if let Some(slot) = call
                    .slots
                    .iter_mut()
                    .find(|slot| matches!(slot.kind, AbiSlotKind::Parameter | AbiSlotKind::Capture))
                {
                    slot.carrier = match slot.carrier {
                        AbiCarrier::ValueWord => AbiCarrier::GroundValueCarrier,
                        _ => AbiCarrier::ValueWord,
                    };
                }
            }
            D5DeclaredCallMutation::Ownership => {
                if let Some(slot) = call
                    .slots
                    .iter_mut()
                    .find(|slot| matches!(slot.kind, AbiSlotKind::Parameter | AbiSlotKind::Capture))
                {
                    slot.ownership = match slot.ownership {
                        AbiOwnership::OwnedByFrame => AbiOwnership::BorrowedForActivation,
                        _ => AbiOwnership::OwnedByFrame,
                    };
                }
            }
            D5DeclaredCallMutation::StorageOwner => {
                if let Some(slot) = call
                    .slots
                    .iter_mut()
                    .find(|slot| matches!(slot.kind, AbiSlotKind::Parameter | AbiSlotKind::Capture))
                {
                    slot.storage_owner = match slot.storage_owner {
                        AbiStorageOwner::ActivationFrame => AbiStorageOwner::PersistentStore,
                        _ => AbiStorageOwner::ActivationFrame,
                    };
                }
            }
            D5DeclaredCallMutation::Ordinal => {
                if let Some(slot) = call
                    .slots
                    .iter_mut()
                    .find(|slot| matches!(slot.kind, AbiSlotKind::Parameter | AbiSlotKind::Capture))
                {
                    slot.ordinal = slot.ordinal.wrapping_add(1);
                }
            }
            D5DeclaredCallMutation::Header => {
                call.header.frame_bytes = call.header.frame_bytes.wrapping_add(8);
            }
            D5DeclaredCallMutation::Offsets => {
                if let Some(offset) = call.offsets.first_mut() {
                    *offset = offset.wrapping_add(8);
                }
            }
            D5DeclaredCallMutation::Retarget => {
                if let Some(other) = other.clone() {
                    if other.origin != call.origin {
                        *call = other;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
thread_local! {
    /// How many call edges the most recent compile resolved.
    ///
    /// ⚠ Same limitation as [`b2f_last_unit_emission`]: it names no attempt, so
    /// read it only where one compile is known to have run to this seam.
    static B2F_CALL_EDGE_RESOLUTION: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The resolved call-edge count from the most recent compile.
#[cfg(test)]
pub(in crate::cranelift_backend) fn b2f_last_call_edge_resolution() -> usize {
    B2F_CALL_EDGE_RESOLUTION.with(std::cell::Cell::get)
}

/// The uniform internal call ABI for every target unit:
/// `(frame_ptr, services_ptr) -> i64`.
///
/// ⭐ **This is what "one fixed call-ABI scheme, not one fixed byte size"
/// means.** Every unit shares this signature; what varies per origin is the
/// *frame layout* the pointer addresses, which `B2R` declares per unit in
/// `AbiFrameHeader` + the slot run. ⛔ Reading "fixed frame" as one universal
/// byte size is the error that would reintroduce a boxed `Value` nobody asked
/// for, and `B2R` says so explicitly.
///
/// ⚠ The signature takes **no program-derived parameter**, which is the same
/// structural guarantee `AC-G0` accepts for `emit_native_int_local_graph`:
/// making a unit's *signature* vary with the program would require a visible
/// change here, so the compiler forbids that growth mode rather than a test
/// detecting it.
pub(super) fn unit_signature<M: Module>(module: &M) -> cranelift_codegen::ir::Signature {
    let mut sig = module.make_signature();
    sig.params
        .push(AbiParam::new(module.target_config().pointer_type()));
    sig.params
        .push(AbiParam::new(module.target_config().pointer_type()));
    sig.returns.push(AbiParam::new(types::I64));
    sig
}

/// **`D1` — forward-declare the whole bundle before any body is defined.**
///
/// ⭐ **The bundle is declared in one pass on purpose.** A unit's body may call
/// any other unit (that is what `D4`'s cross-owner call edges are), so a
/// declare-and-define-in-one-pass loop would be unable to emit a call to a unit
/// it has not reached yet. Declaring every signature first makes the call graph
/// order-independent, which is why the frame words `D1` as *"forward-declare the
/// whole bundle first, then define each body."*
pub(in crate::cranelift_backend) fn declare_unit_bundle<M: Module>(
    module: &mut M,
    plan: &StaticTransitionPlan<'_>,
) -> Result<UnitBundle, CraneliftBackendError> {
    let sig = unit_signature(module);
    let mut functions = BTreeMap::new();
    let mut grafted_spine_scopes = BTreeMap::new();
    // `D5a` checkpoint 1: the EXECUTABLE population, not the template one. ⛔ A
    // template-only raw worker must not be declared here -- declaring it and
    // then not defining it is the undefined phantom the ruling names, and it
    // would falsify the declared/defined census below.
    for (ordinal, unit) in plan.executable_units()?.into_iter().enumerate() {
        // The symbol carries the dense ordinal purely so the linker sees
        // distinct names. ⛔ It is NOT an identity: nothing resolves a unit by
        // parsing this string, and `functions` is keyed by the planner's id.
        let name = format!("ken_unit_{ordinal}");
        let id = module
            .declare_function(&name, Linkage::Local, &sig)
            .map_err(|err| backend_module(LoweringCause::Backend, err.to_string()))?;
        if grafted_spine_scopes
            .insert(id, GraftedSpineFunctionScope::Predeclared(unit.function()))
            .is_some()
        {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "two predeclared units share one module function target".to_string(),
            ));
        }
        if functions.insert(unit.function(), id).is_some() {
            // ⛔ Fails closed rather than overwriting. `B2R` gives exactly one
            // descriptor per `PredeclaredFunction`, so a duplicate here means
            // the plane disagrees with that invariant, and silently keeping the
            // last one would emit a bundle whose call edges resolve to the
            // wrong body.
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "two abi descriptors claim one predeclared function unit".to_string(),
            ));
        }
    }
    // `RT-CONTSPEC-ACTIVATE` `D2` -- forward-declare one target per planned
    // continuation specialization, before any body is defined. The symbol
    // carries a dense ordinal only so the linker sees distinct names; the map
    // is keyed by the planner's typed identity, never by that string.
    // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — over `O_t`, the ORDINARY residual
    // targets. A fusion-local target's selected body is lowered at its exact
    // call edge and never becomes a callable `Function`, so declaring one here
    // would mint a symbol nothing defines -- the undefined phantom the
    // executable/template split above exists to prevent, arriving by a second
    // route.
    let mut continuations = BTreeMap::new();
    for (ordinal, unit) in plan
        .ordinary_continuation_targets()?
        .into_iter()
        .enumerate()
    {
        let name = format!("ken_continuation_{ordinal}");
        let id = module
            .declare_function(&name, Linkage::Local, &sig)
            .map_err(|err| backend_module(LoweringCause::Backend, err.to_string()))?;
        if grafted_spine_scopes
            .insert(id, GraftedSpineFunctionScope::Continuation(unit))
            .is_some()
        {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "two continuation units share one module function target".to_string(),
            ));
        }
        if continuations.insert(unit, id).is_some() {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "two continuation descriptors claim one planned specialization".to_string(),
            ));
        }
    }
    // One response owner per fully validated response row, forward-declared
    // before the CP3 body pass defines its host seam and exact K-context call.
    // Every owner keeps the unit signature and one selected causal caller.
    let response_owners = plan
        .static_response_owner_specializations()?
        .map_err(|infeasible| {
            backend_module(
                LoweringCause::Fragment,
                format!(
                    "compile-time response specialization is infeasible at {:?}: {}",
                    infeasible.vis_origin(),
                    infeasible.reason(),
                ),
            )
        })?;
    let mut responses = BTreeMap::new();
    for owner in &response_owners {
        let name = format!("ken_static_response_{}", owner.id().ordinal());
        let id = module
            .declare_function(&name, Linkage::Local, &sig)
            .map_err(|err| backend_module(LoweringCause::Backend, err.to_string()))?;
        if grafted_spine_scopes
            .insert(id, GraftedSpineFunctionScope::Response(owner.id()))
            .is_some()
        {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "two response owners share one module function target".to_string(),
            ));
        }
        if responses.insert(owner.id(), id).is_some() {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "two response-owner descriptors claim one response-owner identity".to_string(),
            ));
        }
    }
    // `RT-DECL-CLOSURE-PORT` `D5a` -- forward-declare one target per planned
    // generated producer execution context, in the same pre-definition pass and
    // for the same reason: a context is called from the enclosing
    // specialization's body, which is defined below.
    let mut contexts = BTreeMap::new();
    for (ordinal, context) in plan.continuation_contexts()?.into_iter().enumerate() {
        let name = format!("ken_continuation_context_{ordinal}");
        let id = module
            .declare_function(&name, Linkage::Local, &sig)
            .map_err(|err| backend_module(LoweringCause::Backend, err.to_string()))?;
        if grafted_spine_scopes
            .insert(
                id,
                GraftedSpineFunctionScope::ContinuationContext(context.id()),
            )
            .is_some()
        {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "two continuation contexts share one module function target".to_string(),
            ));
        }
        if contexts.insert(context.id(), id).is_some() {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "two generated context descriptors claim one planned context".to_string(),
            ));
        }
    }
    // `RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` -- forward-declare one target per
    // installed fused region, in the same pre-definition pass and for the same
    // reason as the three above: the fused function is called from the
    // consumer's body through the redirected invocation, and that body is
    // defined below.
    //
    // Driven by `continuation_fusions()`, which is the join over the plane
    // AND its ABI arena. Iterating the plane alone would declare a target for a
    // region with no frame contract, and the definition pass would then have
    // nothing to lower it against.
    let mut fusions = BTreeMap::new();
    for (ordinal, fusion) in plan.continuation_fusions()?.into_iter().enumerate() {
        let name = format!("ken_static_continuation_fusion_{ordinal}");
        let id = module
            .declare_function(&name, Linkage::Local, &sig)
            .map_err(|err| backend_module(LoweringCause::Backend, err.to_string()))?;
        if grafted_spine_scopes
            .insert(id, GraftedSpineFunctionScope::Fusion(fusion.id()))
            .is_some()
        {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "two fusion regions share one module function target".to_string(),
            ));
        }
        if fusions.insert(fusion.id(), id).is_some() {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "two static continuation fusion descriptors claim one installed region".to_string(),
            ));
        }
    }
    #[cfg(test)]
    B2F_UNIT_EMISSION.with(|cell| cell.set((functions.len(), 0)));
    Ok(UnitBundle {
        functions,
        grafted_spine_scopes,
        continuations,
        responses,
        contexts,
        fusions,
    })
}

/// **`RT-CONTSPEC-ACTIVATE` `D2` — resolve every projected causal identity to
/// its typed declared target.**
///
/// Runs after declaration and before any body is defined, for the same reason
/// `resolve_call_edges` does: a causal edge whose target was never declared is
/// a program that cannot be emitted, and discovering that while half the
/// bodies exist leaves a partially emitted artifact.
///
/// The join is by the identity's `target()` alone. ⛔ Nothing here parses a
/// symbol name, indexes by ordinal, or aliases a `ContinuationSpecializationId`
/// to a `PredeclaredFunctionId` — a missing target rejects.
fn selected_response_owner_target(
    plan: &StaticTransitionPlan<'_>,
    bundle: &UnitBundle,
    identity: &ContinuationCallIdentity,
) -> Result<Option<FuncId>, CraneliftBackendError> {
    match plan.resolved_continuation_callee(identity)? {
        ResolvedContinuationCallee::StaticResponseOwner(owner) => bundle
            .response(owner)
            .ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "a selected response caller names an owner that was never forward-declared"
                        .to_string(),
                )
            })
            .map(Some),
        ResolvedContinuationCallee::OrdinarySpecialization(_) => Ok(None),
    }
}

pub(in crate::cranelift_backend) fn resolved_continuation_call_target(
    plan: &StaticTransitionPlan<'_>,
    bundle: &UnitBundle,
    identity: &ContinuationCallIdentity,
) -> Result<FuncId, CraneliftBackendError> {
    if let Some(response) = selected_response_owner_target(plan, bundle, identity)? {
        #[cfg(feature = "px8-ds-test-support")]
        if let Some(mutation) = STATIC_RESPONSE_CALLER_RETARGET_MUTATION.with(std::cell::Cell::get)
        {
            if mutation == StaticResponseCallerRetargetMutation::RestoreRelayPromotedKTarget
                && !plan
                    .has_pre_schema_relay_promotion(identity.producer_construct_origin(), identity)
            {
                return Ok(response);
            }
            STATIC_RESPONSE_CALLER_RETARGET_APPLICATIONS.with(|count| count.set(count.get() + 1));
            return match mutation {
                StaticResponseCallerRetargetMutation::RestoreSelectedKTarget
                | StaticResponseCallerRetargetMutation::RestoreRelayPromotedKTarget => {
                    bundle.continuation(identity.target()).ok_or_else(|| {
                        backend_module(
                            LoweringCause::PlannerInvariant,
                            "the response-caller mutation found no original K declaration"
                                .to_string(),
                        )
                    })
                }
                StaticResponseCallerRetargetMutation::RemoveSelectedCaller => Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    "the response-caller mutation removed one selected incoming caller".to_string(),
                )),
                StaticResponseCallerRetargetMutation::RetargetToDifferentResponseOwner => {
                    let owners = plan
                        .static_response_owner_specializations()?
                        .map_err(|infeasible| {
                            backend_module(LoweringCause::Fragment, format!(
                                "compile-time response specialization is infeasible at {:?}: {}",
                                infeasible.vis_origin(),
                                infeasible.reason(),
                            ))
                        })?;
                    let substitute = owners
                        .iter()
                        .find(|owner| owner.selected_caller() != identity)
                        .ok_or_else(|| {
                            backend_module(
                                LoweringCause::PlannerInvariant,
                                "the response-caller retarget control found no different owner"
                                    .to_string(),
                            )
                        })?;
                    bundle.response(substitute.id()).ok_or_else(|| {
                        backend_module(
                            LoweringCause::PlannerInvariant,
                            "the response-caller retarget control found no declared substitute"
                                .to_string(),
                        )
                    })
                }
            };
        }
        return Ok(response);
    }
    bundle.continuation(identity.target()).ok_or_else(|| {
        backend_module(
            LoweringCause::PlannerInvariant,
            "a projected causal identity names a continuation specialization that was never \
             forward-declared"
                .to_string(),
        )
    })
}

fn verified_response_owner_calls<'a>(
    plan: &StaticTransitionPlan<'_>,
    bundle: &UnitBundle,
    calls: impl IntoIterator<Item = &'a ContinuationCallIdentity>,
) -> Result<Vec<ContinuationCallIdentity>, CraneliftBackendError> {
    let mut verified = Vec::new();
    for identity in calls {
        let Some(expected) = selected_response_owner_target(plan, bundle, identity)? else {
            continue;
        };
        if resolved_continuation_call_target(plan, bundle, identity)? == expected {
            verified.push(identity.clone());
        }
    }
    Ok(verified)
}

pub(in crate::cranelift_backend) fn resolve_continuation_targets(
    plan: &StaticTransitionPlan<'_>,
    bundle: &UnitBundle,
) -> Result<BTreeMap<ContinuationCallIdentity, FuncId>, CraneliftBackendError> {
    let mut resolved = BTreeMap::new();
    // **`D3` — the ORDINARY residual domain `O`, from the one plan-authoritative
    // accessor.** A fusion-local identity omits its target resolution entirely
    // (Architect `evt_48rwarx25pj2p` §3), so it must not be looked up here: its
    // target has no forward-declared `Function` to resolve to, and asking would
    // raise the never-declared refusal below for an identity that is *lawfully*
    // absent. Narrowing the INPUT is what keeps that refusal meaningful for the
    // ordinary population instead of weakening it to tolerate an absence.
    for identity in plan.ordinary_continuation_call_identities()? {
        let target = resolved_continuation_call_target(plan, bundle, &identity)?;
        if resolved.insert(identity, target).is_some() {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "two projected causal identities collide; the planner mints one call token per \
                 ruled recursive position, so this is a key-arity defect rather than a \
                 double-resolution"
                    .to_string(),
            ));
        }
    }
    Ok(resolved)
}

/// **`RT-DECL-CLOSURE-PORT` `D5a`** — one entry of a continuation
/// specialization body's case environment, named by **where its operand comes
/// from** rather than by the operand.
///
/// ⭐ The whole property under repair here is an **order**, so the order is
/// what this type makes observable. Cranelift `Value`s cannot be synthesized
/// without a live `FunctionBuilder`, so a control written against the assembled
/// operands could only ever re-run the pipeline and read a refusal; a control
/// written against this plan states the binding law directly.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum ContinuationCaseBinderSource {
    /// The projected `StaticWorker`, standing for one recursive field's
    /// **induction hypothesis**.
    ///
    /// `D6a`: this binding takes the
    /// [`StaticWorkerCallRoute::GeneratedContext`] route -- the planner-issued
    /// execution context, which appends this frame's continuation-input suffix
    /// -- **iff** the planner issued such a context for this
    /// `(specialization, worker body)` pair and this unit resolved it.
    /// Otherwise it lawfully takes [`StaticWorkerCallRoute::RawWorker`], like
    /// every pre-`D5a` specialization, and appends nothing.
    InductionHypothesis,
    /// The ordinary-envelope operand at this index. ⛔ An index into the
    /// envelope, never a constructor source position -- the two coincide only
    /// when no `WorkerCapture` role precedes the field.
    Ordinary(usize),
    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D6a`** -- the selected **recursive
    /// constructor argument**, at its own constructor source position.
    ///
    /// ⭐ This is a compiler-only member. It is not a new source occurrence,
    /// continuation input, ABI slot, carrier, tag or runtime descriptor: it is
    /// the *same closure* the induction hypothesis names. The unit already
    /// carries every fact needed to build it -- the closure occurrence, body,
    /// declared arity, ordered capture provenance and the worker-capture
    /// operands -- so nothing crosses the ABI to represent it.
    ///
    /// It carries [`StaticWorkerCallRoute::RawWorker`] **unconditionally**: the
    /// source scope binds the closure itself, so there is nothing to condition
    /// on. ⛔ That is *not* a claim that it differs in route from the induction
    /// hypothesis beside it. In a unit that resolved no generated context the
    /// hypothesis lawfully carries `RawWorker` too, and the two members are
    /// then separated by their positions in the run rather than by their
    /// routes. See [`StaticWorkerCallRoute`] for the asymmetric law.
    ///
    /// ⛔ Before `D6a` this position was **skipped**, and the induction
    /// hypothesis silently stood in for the argument as well. That is a wrong
    /// program, not a missing one: every later binder shifted down by one, so
    /// the case body's outer-frame references landed one slot early.
    SelectedRecursiveArgument { source_position: u32 },
    /// The continuation input at this ordinal.
    ContinuationInput(usize),
}

/// **`RT-DECL-CLOSURE-PORT` `D5a` — the ruled computational-case binding law,
/// as a plan.**
///
/// The established law, identical in the specialized and carried paths
/// (`lower_computational_match_value_composed` builds the same three segments
/// from `induction_hypotheses`, `extend_specialized(args)`, then the frame
/// environment):
///
/// ```text
/// [IH bindings, recursive-position order REVERSED]
///   ++ [ALL constructor arguments, source order]
///   ++ [frame/tail environment]
/// ```
///
/// ⭐ **`RT-CONTSRC-PRODUCER-LOCAL` `D6a` -- "ALL" is load-bearing and was the
/// defect.** The middle segment covers **every** constructor argument in source
/// order, the selected recursive one included. The pre-`D6a` construction
/// *replaced* the selected recursive argument with its own induction
/// hypothesis: it emitted the IH in segment 1 and then skipped that position in
/// segment 2, so a case body with one recursive field and one outer reference
/// got a two-member run where the source scope has three. Every binder after
/// the skipped position was off by one, which is why the measured symptom was
/// an out-of-range `Var` at the *tail* (`Var: no runtime binding for index 2`)
/// rather than anything at the position actually omitted.
///
/// ⭐ **`recursive_position` is a constructor SOURCE-FIELD coordinate, not a
/// lexical environment index.** Reading it as the latter is the exact defect
/// this function exists to prevent: it placed the worker at environment slot
/// `recursive_position`, which for a nonzero position moves the induction
/// hypothesis out of its established lexical prefix and rebinds `Var(0)` to an
/// ordinary field. The measured consequence on `px8tr_nested_post_effect` was
/// `Unsupported(Call, "callee is not a closure")` -- `Var(0)` reading a `Unit`.
///
/// The specialization eliminates **one** selected recursive callable, so its
/// projected `StaticWorker` is that position's IH-prefix binding.
///
/// The selected recursive field is **absent from the ordinary envelope** --
/// [`ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField`] is by
/// construction the population that excludes it. ⛔ `D6a`: that is a fact about
/// the *envelope*, and reading it as a fact about the *binder run* is the
/// defect. The field has no envelope operand because it needs none: it is a
/// compiler-only [`ContinuationCaseBinderSource::SelectedRecursiveArgument`],
/// built from the unit's own worker provenance. So the run's length is
/// `argument_binders + recursive_positions.len()` -- one IH per recursive
/// position, then one member per constructor argument -- which is exactly the
/// binder count the planner's own demand walk uses for this case.
///
/// ⛔ Worker-capture `Parameter` slots are **not** case binders. They construct
/// the `StaticWorker`; they never enter this run.
///
/// Every gap is a hard stop rather than a hole: a gap-filled run would silently
/// shift every later binder, which is a wrong program rather than a refused one.
pub(super) fn continuation_case_binder_run(
    argument_binders: usize,
    recursive_positions: &[usize],
    worker_recursive_position: u32,
    envelope: &[ContinuationOrdinaryEnvelopeRole],
    continuation_inputs: usize,
) -> Result<Vec<ContinuationCaseBinderSource>, CraneliftBackendError> {
    for position in recursive_positions.iter().copied() {
        if position >= argument_binders {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "the selected case names a recursive position {position} outside its own \
                 {argument_binders}-binder run"
                ),
            ));
        }
    }
    let worker_position = usize::try_from(worker_recursive_position).map_err(|_| {
        backend_module(
            LoweringCause::Capacity,
            "the ruled recursive position exceeds this platform's index width".to_string(),
        )
    })?;
    if !recursive_positions.contains(&worker_position) {
        return Err(backend_module(
            LoweringCause::PlannerInvariant,
            format!(
                "the ruled recursive position {worker_position} is not among the selected case's \
             recursive positions, so the projected worker stands for no induction hypothesis"
            ),
        ));
    }

    // **`D6c` — THE CHECKED TOTAL, computed ONCE and reused.**
    //
    // ⛔ Chained `checked_add` with a typed refusal on overflow: no panic and no
    // wrapping. A wrapped total would understate the run's length and let the
    // sealed-run cardinality check below pass on a run it should refuse, which
    // is the one failure mode this postcondition exists to prevent.
    //
    // ⭐ The SAME value serves the allocation's capacity and the final
    // cardinality. Computing it twice would let the two drift, and the check
    // would then be comparing the run against a total the builder never used.
    let sealed_total = recursive_positions
        .len()
        .checked_add(argument_binders)
        .and_then(|partial| partial.checked_add(continuation_inputs))
        .ok_or_else(|| {
            backend_module(LoweringCause::Capacity, format!(
                "this case's binder run would hold {} induction hypotheses + {argument_binders} \
                 constructor arguments + {continuation_inputs} continuation inputs, which \
                 overflows this platform's index width",
                recursive_positions.len()
            ))
        })?;
    let mut run = Vec::with_capacity(sealed_total);

    // Segment 1 -- the IH prefix, recursive positions reversed.
    //
    // ⚠ **The reversal is not observable on any case this mechanism accepts.**
    // A specialization projects exactly one worker, so a second recursive
    // position has no IH to bind and hard-stops below; with one position,
    // reversed and forward order coincide. It is written as the law states it
    // rather than collapsed to the single-position case, so that admitting a
    // second worker later is a change to the projection and not to this order.
    // `D6c` — FABRICATED AVAILABILITY, under test only. A second recursive
    // position is claimed for this case, which the specialization projects no
    // worker for. ⛔ The claim is added to the segment's own input rather than
    // to its output: the loop below is untouched, so what refuses is the
    // production guard that owns availability, not a rewritten loop.
    #[cfg(test)]
    let fabricated: Vec<usize>;
    #[cfg(test)]
    let recursive_positions = if crate::cranelift_backend::lowering::d6c_selection_mutation()
        == crate::cranelift_backend::lowering::D6cSelectionMutation::FabricatedAvailability
    {
        crate::cranelift_backend::lowering::record_d6c_selection_application();
        fabricated = recursive_positions
            .iter()
            .copied()
            .chain(std::iter::once(worker_position.wrapping_add(1)))
            .collect();
        fabricated.as_slice()
    } else {
        recursive_positions
    };

    for position in recursive_positions.iter().rev().copied() {
        if position != worker_position {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "the selected case has a recursive position {position} that the continuation \
                 specialization projects no worker for, so its induction-hypothesis prefix cannot \
                 be built"
                ),
            ));
        }
        run.push(ContinuationCaseBinderSource::InductionHypothesis);
    }

    // Segment 2 -- ALL the constructor arguments in SOURCE order.
    //
    // A nonrecursive field takes its operand from its own envelope role. A
    // recursive field takes the compiler-only `SelectedRecursiveArgument`
    // member: it is the same closure the IH prefix names, bound at a second
    // environment position and reached by its own call route, so it needs no
    // envelope operand and no ABI slot.
    //
    // ⛔ "Differing only in call route" would overstate it. The routes differ
    // only where the planner issued a generated context; where it issued none,
    // both members carry `RawWorker` and the difference is *which position of
    // the run they occupy* -- which is precisely the difference this segment
    // exists to restore.
    //
    // ⛔ `D6a`: this loop used to `continue` on a recursive position. The IH
    // then stood in for the argument as well as for the hypothesis, and every
    // later binder shifted down one slot.
    for position in 0..argument_binders {
        let source_position = u32::try_from(position).map_err(|_| {
            backend_module(
                LoweringCause::Capacity,
                "a continuation case binder position exceeds the planner's field width".to_string(),
            )
        })?;
        if recursive_positions.contains(&position) {
            // ⛔ The hard stop for an unprojected recursive position is
            // segment 1's, and it has already fired: a position that is not the
            // ruled `worker_position` never reaches here. So this member is
            // always the *selected* recursive argument, and `D6a` deliberately
            // does not generalize to a multi-worker population.
            //
            // `D6c` — the four run-shape mutations, under test only. Each moves
            // ONE producer input of this run and leaves the rest of the segment
            // exactly as it was, so a refusal downstream is attributable to that
            // input rather than to a rewritten builder.
            #[cfg(test)]
            match crate::cranelift_backend::lowering::d6c_selection_mutation() {
                // The pre-`D6a` defect exactly: the position is skipped and the
                // IH stands in for the argument as well.
                crate::cranelift_backend::lowering::D6cSelectionMutation::OmitSelectedArgument => {
                    crate::cranelift_backend::lowering::record_d6c_selection_application();
                    continue;
                }
                // One run naming two selected arguments.
                crate::cranelift_backend::lowering::D6cSelectionMutation::DuplicateSelectedArgument => {
                    crate::cranelift_backend::lowering::record_d6c_selection_application();
                    run.push(ContinuationCaseBinderSource::SelectedRecursiveArgument {
                        source_position,
                    });
                }
                // A source position the unit projects no worker for. ⛔ Chosen
                // by arithmetic on the ruled position rather than from the
                // envelope, so the value is not one the plan named anywhere.
                crate::cranelift_backend::lowering::D6cSelectionMutation::WrongSourcePosition => {
                    crate::cranelift_backend::lowering::record_d6c_selection_application();
                    run.push(ContinuationCaseBinderSource::SelectedRecursiveArgument {
                        source_position: source_position.wrapping_add(1),
                    });
                    continue;
                }
                _ => {}
            }
            run.push(ContinuationCaseBinderSource::SelectedRecursiveArgument { source_position });
            continue;
        }
        let index = envelope
            .iter()
            .position(|role| {
                matches!(
                    role,
                    ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField {
                        source_position: candidate,
                    } if *candidate == source_position
                )
            })
            .ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    format!(
                        "the ordinary envelope has no nonrecursive field at source position \
                     {source_position}, so the selected case's binder run cannot be built"
                    ),
                )
            })?;
        run.push(ContinuationCaseBinderSource::Ordinary(index));
    }

    // `D6c` — WRONG ORDER, under test only. The IH prefix and the argument
    // segment are exchanged and nothing else moves: the same members, the same
    // count, the same continuation-input tail. ⛔ Applied before segment 3 so
    // the tail stays in its ruled place and the perturbation is confined to the
    // two segments whose order is the law under test.
    #[cfg(test)]
    if crate::cranelift_backend::lowering::d6c_selection_mutation()
        == crate::cranelift_backend::lowering::D6cSelectionMutation::WrongOrder
    {
        let hypotheses = run
            .iter()
            .filter(|source| matches!(source, ContinuationCaseBinderSource::InductionHypothesis))
            .count();
        if hypotheses > 0 && hypotheses < run.len() {
            crate::cranelift_backend::lowering::record_d6c_selection_application();
            run.rotate_left(hypotheses);
        }
    }

    // Segment 3 -- the tail environment: this frame's continuation inputs, in
    // their ruled ordinal order.
    for ordinal in 0..continuation_inputs {
        run.push(ContinuationCaseBinderSource::ContinuationInput(ordinal));
    }

    // ⭐⭐ **`RT-CONTSRC-PRODUCER-LOCAL` `D6c` — THE CANONICAL-RUN
    // POSTCONDITION. The run is SEALED here, and nothing leaves this function
    // unvalidated.**
    //
    // This function's doc has always claimed *"every gap is a hard stop rather
    // than a hole"*, and for the gaps it names that was true. It was NOT true of
    // the run's own SHAPE: `D6c` measured a member omitted, a member duplicated,
    // and the two segments permuted, and in each case the malformed run was
    // returned and lowered. Omission is the pre-`D6a` defect exactly, and on the
    // mixed witness it compiled clean -- the case body reads only `Var(0)`, so
    // every later binder shifted with nothing positioned to notice.
    //
    // ⛔ **Validated IN PLACE against this function's own inputs. There is no
    // second builder and no second population.** Constructing an expected run
    // and comparing would be a parallel authority able to reproduce the very
    // defect it checks, and the equality would prove only that two
    // constructions agree with each other.
    //
    // ⚠ **What this deliberately does NOT require:** that `Ordinary(index)`
    // values be numerically source-ordered, or any reconstruction of the
    // ordinary envelope's order. A self-consistent envelope permutation is
    // lawful -- each member is checked against the ROLE its own index names, so
    // the envelope may be laid out however the planner chose.
    let hypotheses = recursive_positions.len();
    if run.len() != sealed_total {
        return Err(backend_module(
            LoweringCause::PlannerInvariant,
            format!(
                "the sealed binder run holds {} members, but this case seals {hypotheses} induction \
             hypotheses + {argument_binders} constructor arguments + {continuation_inputs} \
             continuation inputs = {sealed_total}. A run of the wrong length shifts every later \
             binder, which is a wrong program rather than a refused one",
                run.len()
            ),
        ));
    }

    // Segment 1's exact extent. ⛔ Both directions: a non-hypothesis inside the
    // prefix and a hypothesis outside it are different defects and both are
    // caught, the second by the segment walks below.
    for (position, source) in run.iter().enumerate().take(hypotheses) {
        // ⛔ EVERY variant enumerated, no `matches!` and no `other` arm. This is
        // a load-bearing position: a future member kind must be a compile error
        // here, forcing a decision about whether it may lead the run, rather
        // than falling into a catch-all that happens to reject it today for a
        // reason nobody chose.
        match source {
            ContinuationCaseBinderSource::InductionHypothesis => {}
            ContinuationCaseBinderSource::SelectedRecursiveArgument { .. }
            | ContinuationCaseBinderSource::Ordinary(_)
            | ContinuationCaseBinderSource::ContinuationInput(_) => {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    format!(
                        "the sealed binder run holds {source:?} at position {position}, inside the \
                     {hypotheses}-member induction-hypothesis prefix. The IH prefix leads the run \
                     and the constructor arguments follow it; a member of another kind here is \
                     the two segments permuted"
                    ),
                ));
            }
        }
    }

    // Segment 2 -- every constructor argument at its own source position.
    //
    // ⛔ The match is EXHAUSTIVE over the closed source sum with no wildcard, so
    // a future variant is a compile error here rather than a silent acceptance.
    for position in 0..argument_binders {
        let index = hypotheses + position;
        let source_position = u32::try_from(position).map_err(|_| {
            backend_module(
                LoweringCause::Capacity,
                "a continuation case binder position exceeds the planner's field width".to_string(),
            )
        })?;
        let recursive = recursive_positions.contains(&position);
        match &run[index] {
            ContinuationCaseBinderSource::SelectedRecursiveArgument {
                source_position: named,
            } => {
                if !recursive {
                    return Err(backend_module(
                        LoweringCause::PlannerInvariant,
                        format!(
                            "the sealed binder run names a selected recursive argument at run \
                         position {index} for source position {source_position}, which this case \
                         does not list as recursive"
                        ),
                    ));
                }
                if *named != source_position {
                    return Err(backend_module(
                        LoweringCause::PlannerInvariant,
                        format!(
                            "the sealed binder run's argument segment holds a selected recursive \
                         argument for source position {named} at the slot belonging to source \
                         position {source_position}"
                        ),
                    ));
                }
                if position != worker_position {
                    return Err(backend_module(
                        LoweringCause::PlannerInvariant,
                        format!(
                            "the sealed binder run names a selected recursive argument at source \
                         position {source_position}, but this specialization projects a worker \
                         for position {worker_position}"
                        ),
                    ));
                }
            }
            ContinuationCaseBinderSource::Ordinary(role_index) => {
                if recursive {
                    return Err(backend_module(
                        LoweringCause::PlannerInvariant,
                        format!(
                            "the sealed binder run takes source position {source_position} from the \
                         ordinary envelope, but this case lists it as recursive -- the recursive \
                         field is a compiler-only member and has no envelope operand"
                        ),
                    ));
                }
                // ⚠ The ROLE the index names, never the index's own value. This
                // is what keeps a lawful envelope permutation lawful.
                let role = envelope.get(*role_index).ok_or_else(|| {
                    backend_module(
                        LoweringCause::PlannerInvariant,
                        format!(
                            "the sealed binder run points at ordinary-envelope index {role_index}, \
                         which this frame's {}-role envelope does not hold",
                            envelope.len()
                        ),
                    )
                })?;
                match role {
                    ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField {
                        source_position: candidate,
                    } if *candidate == source_position => {}
                    other => {
                        return Err(backend_module(
                            LoweringCause::PlannerInvariant,
                            format!(
                                "the sealed binder run takes source position {source_position} from \
                             ordinary-envelope index {role_index}, which names {other:?} instead"
                            ),
                        ));
                    }
                }
            }
            other @ (ContinuationCaseBinderSource::InductionHypothesis
            | ContinuationCaseBinderSource::ContinuationInput(_)) => {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    format!(
                        "the sealed binder run holds {other:?} at run position {index}, inside the \
                     constructor-argument segment for source position {source_position}"
                    ),
                ));
            }
        }
    }

    // Segment 3 -- the continuation-input tail, by exact ordinal.
    for ordinal in 0..continuation_inputs {
        let index = hypotheses + argument_binders + ordinal;
        // ⛔ EVERY variant enumerated, for the same reason as the IH prefix. The
        // ordinal mismatch is split out from the wrong-kind case so the two are
        // distinguishable in the diagnostic rather than merged into one arm.
        match &run[index] {
            ContinuationCaseBinderSource::ContinuationInput(named) => {
                if *named != ordinal {
                    return Err(backend_module(
                        LoweringCause::PlannerInvariant,
                        format!(
                            "the sealed binder run holds continuation input {named} at run position \
                         {index}, where this frame's continuation input {ordinal} belongs"
                        ),
                    ));
                }
            }
            source @ (ContinuationCaseBinderSource::InductionHypothesis
            | ContinuationCaseBinderSource::SelectedRecursiveArgument { .. }
            | ContinuationCaseBinderSource::Ordinary(_)) => {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    format!(
                        "the sealed binder run holds {source:?} at run position {index}, where this \
                     frame's continuation input {ordinal} belongs"
                    ),
                ));
            }
        }
    }

    Ok(run)
}

/// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the projected facts one
/// continuation specialization's SELECTED CASE BODY is lowered from.**
///
/// Exactly the members [`lower_continuation_selected_case_body`] reads, and no
/// more. The frame-shaped members of the definition pass's own projection --
/// slots, offsets, header, consumer owner -- are absent because a local
/// composition has no frame: it lowers the same body with no `Function`, no
/// descriptor and no ABI of its own.
pub(super) struct ContinuationSelectedCaseBody {
    pub(super) id: ContinuationSpecializationId,
    pub(super) continuation_origin: StaticOriginId,
    pub(super) producer_alternative: u32,
    pub(super) recursive_position: u32,
    pub(super) worker_closure_origin: StaticOriginId,
    pub(super) worker_body_origin: StaticOriginId,
    pub(super) worker_declared_arity: u32,
    pub(super) worker_capture_count: usize,
}

/// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — lower a continuation
/// specialization's exact selected case body, given its operands.**
///
/// **This is a factoring, not a new mechanism, and that is the point.**
/// Architect `evt_6kn9ckdnbf0ph` rules that a fusion-local identity replaces
/// **only its direct-call realization**: the body it would have executed, the
/// two static-worker bindings, the binder run and the environment are all
/// unchanged. So the local composition and the standalone definition must lower
/// **the same body from the same plan** -- and the only way to say that and
/// have it stay true is for there to be one function, called from both seats.
/// A second copy here would be a second authority over what the selected case
/// body IS, and the two would drift with nothing able to see it.
///
/// **What differs between the two callers is the OPERANDS, and nothing
/// else.** The definition pass loads `ordinary` and `carried_inputs` from its
/// own frame's `Parameter` and `Capture` slots at the descriptor's offsets; the
/// local composition receives the very same two runs assembled at the call
/// edge, from the planner's ordinary envelope and its continuation-input
/// projection. Both are the target specialization's own ordinary envelope in
/// its own order -- that is why the same body can consume either.
///
/// **It returns the phase-bearing [`LoweringOperand`], and writes no result
/// slot.** The definition pass stores it to its frame's `Result` offset; the
/// local composition hands it straight to the caller's existing eliminator.
/// Neither seat's disposal belongs here, and putting one here would give the
/// other a store it must then undo.
pub(super) fn lower_continuation_selected_case_body(
    compiler: &mut Lowering<'_>,
    builder: &mut FunctionBuilder<'_>,
    facts: &ContinuationSelectedCaseBody,
    envelope: &[ContinuationOrdinaryEnvelopeRole],
    ordinary: &[LoweringOperand],
    carried_inputs: &[LoweringOperand],
    // `D5a` — `Some` exactly when the CALLER resolved a generated execution
    // context for this specialization's worker body into its own function.
    // Supplied rather than re-asked of the planner: an issued context the
    // caller did not resolve is not a context the induction hypothesis may
    // name, and that is a fact about the caller's function, not about the plan.
    retargeted_worker_body: Option<StaticOriginId>,
) -> Result<LoweringOperand, CraneliftBackendError> {
    // The ordered capture segment for the selected worker: the
    // envelope's `WorkerCapture` roles, in capture-ordinal order,
    // taking each one's operand from its own Parameter position.
    let mut worker_captures = Vec::new();
    // **`RT-CHECKED-IH-CAPTURED-ENV-SCHEMA` tier 1 -- the ci<->oi bijection is
    // ADMITTED here, not rebuilt.**
    //
    // This loop used to match `WorkerCapture { .. }` and discard every field,
    // keeping only the operand. The planner had already issued the bijection on
    // that record -- `ordinal` IS ci and `source` IS oi -- so the mapping was
    // never missing, it was being dropped one line before the consumer that
    // needs it. Reading it here is sourced-from-source by construction: there is
    // no parallel map to drift and no positional inference to be right by luck.
    //
    // `Seed` is the genuinely unsourced case and is REFUSED. A seed capture
    // has no source occurrence, and inventing a label for it is the exact
    // fabrication this slice exists to prevent -- so it fails closed rather than
    // taking a positional fallback.
    let mut capture_sources: Vec<(u32, StaticOriginId)> = Vec::new();
    for (position, role) in envelope.iter().enumerate() {
        if let ContinuationOrdinaryEnvelopeRole::WorkerCapture {
            ordinal, source, ..
        } = role
        {
            capture_sources.push((*ordinal, admitted_capture_source(*ordinal, source)?));
            worker_captures.push(ordinary[position].clone());
        }
    }
    validate_captured_environment_bijection(&capture_sources, worker_captures.len())?;
    validate_admitted_matches_role_sequence(envelope, &capture_sources)?;
    if worker_captures.len() != facts.worker_capture_count {
        return Err(backend_module(
            LoweringCause::PlannerInvariant,
            "the ordinary envelope's worker-capture segment disagrees with the selected \
             worker's capture count"
                .to_string(),
        ));
    }

    // `D6a` -- THE INDUCTION HYPOTHESIS'S ROUTE, and only its.
    //
    // `retargeted_worker_body` is `Some` exactly when
    // `continuation_context_for` issued a generated execution context
    // for this `(specialization, worker body)` pair AND the retarget
    // below resolved it into this function. Both halves are required,
    // which is why this reads the retarget's own outcome rather than
    // re-asking the planner: an issued context this unit did not
    // resolve is not a context this binding can name.
    //
    // `None` is the ordinary, lawful answer -- every pre-`D5a`
    // specialization, and every unit in the governed-bracket witness.
    // The hypothesis then takes the raw route, appending nothing, and
    // the two bindings below are route-identical. That is a degenerate
    // route pair, not a collapsed one.
    //
    // Not re-derived at the call site. Both bindings below name the
    // same body origin, so no comparison available there can tell them
    // apart -- see `StaticWorkerCallRoute`.
    let induction_route = match retargeted_worker_body {
        Some(_) => StaticWorkerCallRoute::GeneratedContext,
        None => StaticWorkerCallRoute::RawWorker,
    };
    // `D6c` — CROSS-ROUTING, the hypothesis half. It takes the raw route
    // while this unit DID resolve a context; the argument below takes
    // the context route. Only where a context was actually resolved:
    // on a route-degenerate unit both members lawfully carry
    // `RawWorker`, so there is no crossing to make and the arm declines
    // rather than counting an application it did not perform.
    #[cfg(test)]
    let induction_route = if crate::cranelift_backend::lowering::d6c_selection_mutation()
        == crate::cranelift_backend::lowering::D6cSelectionMutation::CrossRouteTargets
        && retargeted_worker_body.is_some()
    {
        crate::cranelift_backend::lowering::record_d6c_selection_application();
        StaticWorkerCallRoute::RawWorker
    } else {
        induction_route
    };

    // The EXISTING constructor, with the projected identity and arity.
    let worker = compiler.construct_static_worker_binding(
        facts.worker_closure_origin,
        facts.worker_body_origin,
        facts.worker_declared_arity,
        facts.worker_capture_count,
        worker_captures.clone(),
        induction_route,
        // `D8i` — an induction hypothesis answers for no composed
        // source continuation. Stated explicitly: this is a positive
        // claim about the hypothesis's role, not the absence of one.
        ContinuationDischarge::DirectSpecializationCall,
    )?;

    // `D6a` -- the selected recursive constructor argument.
    //
    // The SAME closure occurrence, body origin, declared arity and
    // ordered capture operands as the induction hypothesis above, built
    // through the same constructor and validated against the same raw
    // template contract. What the two represent still differs: the
    // argument is the closure the source scope binds, while the
    // hypothesis is that closure as this specialization eliminates it.
    //
    // The ROUTE is `RawWorker` unconditionally here, and that is not
    // the same as saying it differs from the hypothesis's. When
    // `induction_route` above resolved to `RawWorker` -- no context
    // issued -- the two bindings are route-identical, and they are
    // still two bindings for two positions of the run. The route is
    // what will separate them at the call edge in `D6b` *where a
    // context exists*; it is not what makes them two.
    //
    // Nothing new crosses the ABI. This adds no slot, carrier, tag,
    // descriptor or source occurrence -- it is a second compiler-only
    // binding over operands this frame has already loaded.
    // `D6c` — the three binding-construction mutations, under test only.
    // Each moves ONE argument handed to the existing constructor; the
    // constructor itself, the hypothesis above and the run below are all
    // untouched, so the guard that refuses is the one that owns the
    // moved input.
    #[cfg(test)]
    let (argument_body_origin, argument_captures, argument_route) = {
        use crate::cranelift_backend::lowering::D6cSelectionMutation as Mutation;
        match crate::cranelift_backend::lowering::d6c_selection_mutation() {
            // A body this unit did not select. The substituted value
            // is a REAL planner-issued origin -- this continuation's own
            // frame occurrence -- rather than an arithmetic neighbour.
            // A fabricated id could be refused merely for being unknown;
            // a real origin naming the wrong role is the case the guard
            // actually has to catch. The control asserts it differs from
            // the selected body.
            Mutation::WrongClosureBody => {
                crate::cranelift_backend::lowering::record_d6c_selection_application();
                (
                    facts.continuation_origin,
                    worker_captures.clone(),
                    StaticWorkerCallRoute::RawWorker,
                )
            }
            // A capture run that is not the envelope's worker-capture
            // segment: drop an operand where there is one, otherwise add
            // one the envelope holds.
            //
            // The counter fires ONLY if the vector actually changed.
            // A unit with no captures and no ordinary operand to borrow
            // leaves this arm the IDENTITY, and counting an application
            // there would report a perturbation that never happened --
            // which is precisely how a control comes to prove the
            // opposite of what it claims.
            Mutation::WrongCaptureRun => {
                let mut perturbed = worker_captures.clone();
                if perturbed.pop().is_none() {
                    perturbed.extend(ordinary.first().cloned());
                }
                if perturbed.len() != worker_captures.len() {
                    crate::cranelift_backend::lowering::record_d6c_selection_application();
                }
                (
                    facts.worker_body_origin,
                    perturbed,
                    StaticWorkerCallRoute::RawWorker,
                )
            }
            // The argument takes the context route. Paired with the
            // hypothesis taking the raw route above, this is the
            // cross-routing the two members must never permit.
            //
            // Only where a context was actually resolved. On a
            // route-degenerate unit both members lawfully carry
            // `RawWorker`, so there is no crossing to perform; applying
            // it there would move a route no law distinguishes and count
            // an application for a perturbation with no content.
            Mutation::CrossRouteTargets if retargeted_worker_body.is_some() => {
                crate::cranelift_backend::lowering::record_d6c_selection_application();
                (
                    facts.worker_body_origin,
                    worker_captures.clone(),
                    StaticWorkerCallRoute::GeneratedContext,
                )
            }
            _ => (
                facts.worker_body_origin,
                worker_captures.clone(),
                StaticWorkerCallRoute::RawWorker,
            ),
        }
    };
    #[cfg(not(test))]
    let (argument_body_origin, argument_captures, argument_route) = (
        facts.worker_body_origin,
        worker_captures,
        StaticWorkerCallRoute::RawWorker,
    );
    let recursive_argument = compiler.construct_static_worker_binding(
        facts.worker_closure_origin,
        argument_body_origin,
        facts.worker_declared_arity,
        facts.worker_capture_count,
        argument_captures,
        argument_route,
        // `D8i` — the SPECIALIZATION's selected recursive argument.
        // Direct, and the contrast with `D8d`'s composed argument is
        // the point: the same source closure at the same position
        // carries an authority on the composed path and none here,
        // because only the composed consumption stands in for a causal
        // call the producer never made.
        ContinuationDischarge::DirectSpecializationCall,
    )?;

    // Exact body recovery: the selected case of the computational
    // frame this continuation belongs to, by its own alternative.
    let frame_occurrence = compiler.retained_body_occurrence(facts.continuation_origin)?;
    let RuntimeExpr::ComputationalMatch { cases, .. } = frame_occurrence.expr else {
        return Err(backend_module(
            LoweringCause::PlannerInvariant,
            "a continuation origin does not resolve to a computational frame".to_string(),
        ));
    };
    let alternative = facts.producer_alternative as usize;
    let case = cases.get(alternative).ok_or_else(|| {
        backend_module(
            LoweringCause::PlannerInvariant,
            "the projected producer alternative is outside the frame's case run".to_string(),
        )
    })?;
    let body =
        compiler.case_body_occurrence(frame_occurrence.static_origin, alternative, &case.body)?;
    // The semantic case environment, through the sole binding
    // authority, in the order `continuation_case_binder_run` states:
    // the IH prefix, then ALL the constructor arguments in source
    // order -- the selected recursive one included, as `D6a`'s
    // compiler-only member -- then this frame's continuation inputs.
    //
    // This site chooses nothing. It maps a plan onto operands; the
    // order is the plan's, and the plan is a pure function of the
    // planner's own coordinates.
    let plan = continuation_case_binder_run(
        case.argument_binders,
        &case.recursive_positions,
        facts.recursive_position,
        envelope,
        carried_inputs.len(),
    )?;
    let mut env: Vec<LoweringEnvironmentBinding> = Vec::with_capacity(plan.len());
    for source in &plan {
        let binding = match *source {
            ContinuationCaseBinderSource::InductionHypothesis => {
                LoweringEnvironmentBinding::StaticWorker(worker.clone())
            }
            ContinuationCaseBinderSource::SelectedRecursiveArgument { source_position } => {
                // The plan only ever names the ruled position here;
                // segment 1 hard-stops on any other. Re-checking it is
                // what keeps that a fact this site verifies rather than
                // one it inherits.
                if source_position != facts.recursive_position {
                    return Err(backend_module(
                        LoweringCause::PlannerInvariant,
                        format!(
                            "the binder run names a selected recursive argument at source \
                         position {source_position}, but this specialization projects a \
                         worker for position {}",
                            facts.recursive_position
                        ),
                    ));
                }
                LoweringEnvironmentBinding::StaticWorker(recursive_argument.clone())
            }
            ContinuationCaseBinderSource::Ordinary(index) => {
                let operand = ordinary.get(index).ok_or_else(|| {
                    backend_module(
                        LoweringCause::PlannerInvariant,
                        "the binder run names an ordinary-envelope index this frame loaded \
                         no operand for"
                            .to_string(),
                    )
                })?;
                LoweringEnvironmentBinding::Value(operand.clone())
            }
            ContinuationCaseBinderSource::ContinuationInput(ordinal) => {
                let operand = carried_inputs.get(ordinal).ok_or_else(|| {
                    backend_module(
                        LoweringCause::PlannerInvariant,
                        "the binder run names a continuation input ordinal this frame \
                         loaded no operand for"
                            .to_string(),
                    )
                })?;
                LoweringEnvironmentBinding::Value(operand.clone())
            }
        };
        env.push(binding);
    }

    // `D6b` — the same instant, structured. The trace below renders the
    // ROUTE of each static-worker member; this record carries the body
    // origin beside it, which is what lets a control ask whether the
    // mixed pair is over ONE body rather than merely mixed.
    #[cfg(test)]
    crate::cranelift_backend::lowering::record_d6b_specialization_body(
        crate::cranelift_backend::lowering::D6bSpecializationBody {
            unit: facts.id,
            worker_body_origin: facts.worker_body_origin,
            retargeted: retargeted_worker_body,
            worker_call_targets: compiler
                .function_local
                .worker_calls
                .keys()
                .copied()
                .collect(),
            raw_worker_call_targets: compiler
                .function_local
                .raw_worker_calls
                .keys()
                .copied()
                .collect(),
            members: env
                .iter()
                .enumerate()
                .filter_map(|(position, binding)| match binding {
                    LoweringEnvironmentBinding::StaticWorker(worker) => {
                        Some((position, worker.route, worker.body_origin))
                    }
                    LoweringEnvironmentBinding::Value(_) => None,
                })
                .collect(),
        },
    );
    #[cfg(test)]
    d5a_trace(format!(
        "  SPEC-BODY {:?} alt={} binders={} ordinary={} envelope={:?} env=[{}]",
        facts.id,
        facts.producer_alternative,
        case.argument_binders,
        ordinary.len(),
        envelope,
        env.iter()
            // `D6a` -- the ROUTE is printed, not just the arm. Both
            // static-worker members name the same closure, body and
            // arity, so an arm-only rendering shows two identical
            // entries and a change collapsing the two routes would be
            // invisible in this log.
            //
            // The converse does not hold, and a reader of this log
            // must not assume it: two entries rendering the SAME route
            // is the lawful route-degenerate case (no context issued),
            // not evidence that one binding was reused for both
            // members. Only a witness whose planner issues a context
            // renders a mixed pair, and only there does this log
            // discriminate the routes at all.
            .map(|binding| match binding {
                LoweringEnvironmentBinding::StaticWorker(worker) => match worker.route {
                    StaticWorkerCallRoute::RawWorker => "StaticWorker(RawWorker)",
                    StaticWorkerCallRoute::GeneratedContext => "StaticWorker(GeneratedContext)",
                },
                LoweringEnvironmentBinding::Value(LoweringOperand::Carried(_)) => "Carried",
                LoweringEnvironmentBinding::Value(LoweringOperand::Residual(_)) => "Residual",
                LoweringEnvironmentBinding::Value(LoweringOperand::Specialized(_)) => "Specialized",
            })
            .collect::<Vec<_>>()
            .join(", ")
    ));
    let lowered = compiler.lower_expr(builder, body, &env)?;
    // The selected continuation's Vis body is lowered in its own function,
    // not by replaying the enclosing Match there. Observe that completed
    // body beside the carried owner's trapped branch at the same origin.
    #[cfg(feature = "px8-ds-test-support")]
    if case.constructor.ends_with("::ITree::Vis")
        && compiler
            .static_transition_plan
            .pending_result_validated_owner(frame_occurrence.static_origin)?
    {
        let owner = compiler.defining_emission_owner.ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "a Vis continuation has no defining emission owner".to_string(),
            )
        })?;
        if compiler
            .static_transition_plan
            .pending_match_has_reentering_calls(frame_occurrence.static_origin, owner)?
            && compiler
                .static_transition_plan
                .owner_fed_match_population(frame_occurrence.static_origin, Some(owner))?
                .is_none()
        {
            crate::cranelift_backend::planning::record_selected_pending_match_emission(
                frame_occurrence.static_origin,
                owner,
                crate::cranelift_backend::planning::SelectedPendingMatchEmissionKind::OrdinaryVisBodyLowered,
            );
        }
    }

    Ok(lowered)
}

/// The response shape `lower_process_host_effect` materializes for one
/// operation. ConsoleIsTerminal is infallible and answers a bare Bool.
/// ClockWallNow answers an Instant constructor and stays refused here until a
/// fixture measures its owner path. Every other operation answers a
/// specialized HostResult.
pub(super) fn static_response_shape_matches(
    operation: ken_host::HostOpV1,
    response: &LoweringOperand,
) -> bool {
    match operation {
        ken_host::HostOpV1::ConsoleIsTerminal => {
            matches!(response, LoweringOperand::Specialized(Lowered::Bool { .. }))
        }
        ken_host::HostOpV1::ClockWallNow => false,
        _ => matches!(
            response,
            LoweringOperand::Specialized(Lowered::HostResult { .. })
        ),
    }
}

/// Emit one validated response effect from an existing response row. Initial
/// owners and pending-Vis successors use this same lowering, so a successor
/// does not acquire a second operation/effect or frame-input authority.
pub(super) fn lower_static_response_effect(
    compiler: &mut Lowering<'_>,
    builder: &mut FunctionBuilder<'_>,
    row: &StaticResponseContinuation,
    effect: &RuntimeExpr,
    operation_arguments: &BTreeMap<StaticOriginId, RuntimeExpr>,
    frame_inputs: &BTreeMap<StaticResponseFrameSource, LoweringOperand>,
) -> Result<(LoweringOperand, cranelift_codegen::ir::Inst), CraneliftBackendError> {
    let frame_operand = |binding: &StaticResponseEnvironmentBinding| {
        frame_inputs
            .get(&binding.frame_source())
            .cloned()
            .ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    format!(
                        "the response environment source {:?} names absent frame input {:?}",
                        binding.source(),
                        binding.frame_source(),
                    ),
                )
            })
    };
    let mut lowered_arguments = BTreeMap::new();
    let mut effect_environment = Vec::with_capacity(row.effect_environment().len());
    for input in row.effect_environment() {
        let operand = match input {
            StaticResponseEffectInput::Frame(binding) => frame_operand(binding)?,
            StaticResponseEffectInput::OperationArgument {
                origin,
                environment,
            } => {
                if let Some(lowered) = lowered_arguments.get(origin).cloned() {
                    lowered
                } else {
                    let argument = operation_arguments.get(origin).ok_or_else(|| {
                        backend_module(LoweringCause::PlannerInvariant,
                            "a mapped response operation argument has no retained source expression"
                                .to_string(),
                        )
                    })?;
                    let argument_environment = environment
                        .iter()
                        .map(|binding| {
                            frame_operand(binding).map(LoweringEnvironmentBinding::Value)
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    let lowered = compiler.lower_expr(
                        builder,
                        SourceOccurrence {
                            expr: argument,
                            static_origin: *origin,
                        },
                        &argument_environment,
                    )?;
                    if lowered_arguments.insert(*origin, lowered.clone()).is_some() {
                        return Err(backend_module(
                            LoweringCause::PlannerInvariant,
                            "one response operation argument was lowered twice".to_string(),
                        ));
                    }
                    lowered
                }
            }
        };
        effect_environment.push(LoweringEnvironmentBinding::Value(operand));
    }

    let response = compiler.lower_expr(
        builder,
        SourceOccurrence {
            expr: effect,
            static_origin: row.effect_origin(),
        },
        &effect_environment,
    )?;
    if !static_response_shape_matches(row.operation(), &response) {
        return Err(backend_module(
            LoweringCause::PlannerInvariant,
            format!(
                "a specialized response owner did not materialize the exact {:?} response",
                row.operation(),
            ),
        ));
    }
    let host_validation_block = builder.current_block().ok_or_else(|| {
        backend_module(
            LoweringCause::PlannerInvariant,
            "host response validation left no active response-owner block".to_string(),
        )
    })?;
    let host_validation_end = builder
        .func
        .layout
        .last_inst(host_validation_block)
        .ok_or_else(|| {
            backend_module(
                LoweringCause::FinishedCode,
                "host response validation emitted no finished instruction".to_string(),
            )
        })?;
    Ok((response, host_validation_end))
}

struct SuccessorEffect {
    row: StaticResponseContinuation,
    effect: RuntimeExpr,
    operation_arguments: BTreeMap<StaticOriginId, RuntimeExpr>,
}

struct PendingVisOwnerCall {
    row: StaticResponseContinuationId,
    call: cranelift_codegen::ir::Inst,
    expected_context: FuncId,
    host_validation_end: cranelift_codegen::ir::Inst,
    ret_identity: u64,
}

struct PendingVisFinishedBody {
    return_inst: cranelift_codegen::ir::Inst,
    initial_context_calls: usize,
    initial_response_is_current_host_result: bool,
    calls: Vec<PendingVisOwnerCall>,
    result_store: cranelift_codegen::ir::Inst,
    result_offset: i32,
    returned_word: cranelift_codegen::ir::Value,
    member_load: cranelift_codegen::ir::Inst,
    member_branches: Vec<cranelift_codegen::ir::Inst>,
    member_ids: Vec<StaticResponseContinuationId>,
    selected_blocks: Vec<cranelift_codegen::ir::Block>,
    payload_loads: Vec<Vec<cranelift_codegen::ir::Inst>>,
    placeholder_offset: i32,
    input_frame: cranelift_codegen::ir::Value,
    record_discriminant_offset: i32,
}

/// On the ordinary path the initial validated host effect and K call already
/// happened. A malformed test control may omit or duplicate that K call; the
/// finished-body verifier checks the recorded run against the emitted flow.
/// Every subsequent iteration is selected by the producer-minted response-row id;
/// no runtime tag, closure word or allocator chooses a successor target.
fn emit_pending_vis_owner_loop(
    compiler: &mut Lowering<'_>,
    builder: &mut FunctionBuilder<'_>,
    frame: cranelift_codegen::ir::Value,
    region: PendingVisFrameRegion,
    initial: &StaticResponseContinuation,
    initial_ret_abi_word: u64,
    initial_calls: &[cranelift_codegen::ir::Inst],
    initial_context: FuncId,
    initial_validation: cranelift_codegen::ir::Inst,
    initial_response_is_current_host_result: bool,
    returned: CarriedBoundaryWord,
    successors: &[SuccessorEffect],
    successor_targets: &BTreeMap<StaticResponseContinuationId, DeclaredUnitCall>,
    successor_contexts: &BTreeMap<StaticResponseContinuationId, FuncId>,
    result_offset: i32,
    placeholder_offset: i32,
    #[cfg(feature = "px8-ds-test-support")] raw_host_result_escape: Option<CarriedBoundaryWord>,
    force_unknown_member: bool,
    bypass_unknown_to_c2: bool,
) -> Result<PendingVisFinishedBody, CraneliftBackendError> {
    if successors.is_empty() {
        return Err(backend_module(
            LoweringCause::PlannerInvariant,
            "a pending-Vis owner has no successor set".to_string(),
        ));
    }
    // Retain malformed 0/2 initial-call controls through emission. The
    // issued-call handles belong to this same function as the successor loop;
    // the finished-body verifier checks them after the CLIF is finalized.
    let initial_ret_identity = initial.k_ret_identity().tag_abi_word()?;
    let mut calls = initial_calls
        .first()
        .map(|&call| {
            vec![PendingVisOwnerCall {
                row: initial.id(),
                call,
                expected_context: initial_context,
                host_validation_end: initial_validation,
                ret_identity: initial_ret_identity,
            }]
        })
        .unwrap_or_default();
    let result = builder.create_block();
    builder.append_block_param(result, types::I64);
    builder.append_block_param(result, types::I64);
    let first_ret = i64::try_from(initial_ret_abi_word).map_err(|_| {
        backend_module(
            LoweringCause::Capacity,
            "pending-Vis initial Ret tag exceeds range".to_string(),
        )
    })?;
    let first_ret = builder.ins().iconst(types::I64, first_ret);
    builder.ins().jump(result, &[returned.word, first_ret]);
    builder.switch_to_block(result);
    let returned_word = builder.block_params(result)[0];
    let expected_ret = builder.block_params(result)[1];
    let returned = CarriedBoundaryWord {
        word: returned_word,
    };
    let actual_tag = compiler.emit_carrier_tag(builder, returned)?;
    let is_ret = builder.ins().icmp(
        cranelift_codegen::ir::condcodes::IntCC::Equal,
        actual_tag,
        expected_ret,
    );
    let ret = builder.create_block();
    let vis = builder.create_block();
    builder.ins().brif(is_ret, ret, &[], vis, &[]);
    builder.switch_to_block(ret);
    let ret_fields = compiler.emit_carrier_field_count(builder, returned)?;
    Lowering::require_i64(builder, ret_fields, 1);
    #[cfg(feature = "px8-ds-test-support")]
    let stored_word = raw_host_result_escape.map_or(returned_word, |word| word.word);
    #[cfg(not(feature = "px8-ds-test-support"))]
    let stored_word = returned_word;
    let result_store = builder
        .ins()
        .store(MemFlags::trusted(), stored_word, frame, result_offset);
    let success = builder.ins().iconst(types::I64, 0);
    let return_inst = builder.ins().return_(&[success]);
    compiler.record_checked_frame_terminal(builder, FrameTerminalKind::Normal)?;
    builder.switch_to_block(vis);
    let member = builder
        .ins()
        .load(types::I64, MemFlags::trusted(), frame, region.discriminant);
    let cranelift_codegen::ir::ValueDef::Result(member_load, _) =
        builder.func.dfg.value_def(member)
    else {
        return Err(backend_module(
            LoweringCause::PlannerInvariant,
            "pending-Vis member is not a finished load".to_string(),
        ));
    };
    #[cfg(feature = "px8-ds-test-support")]
    let member = if force_unknown_member {
        builder.ins().iconst(types::I64, -99)
    } else {
        member
    };
    #[cfg(not(feature = "px8-ds-test-support"))]
    let _ = (force_unknown_member, bypass_unknown_to_c2);
    let mut member_branches = Vec::new();
    let mut member_ids = Vec::new();
    let mut member_blocks = Vec::new();
    for successor in successors {
        let selected = builder.create_block();
        let next = builder.create_block();
        let expected = i64::from(successor.row.id().ordinal()) + 1;
        let matches = builder.ins().icmp_imm(
            cranelift_codegen::ir::condcodes::IntCC::Equal,
            member,
            expected,
        );
        member_branches.push(builder.ins().brif(matches, selected, &[], next, &[]));
        member_ids.push(successor.row.id());
        member_blocks.push(selected);
        builder.switch_to_block(next);
    }
    #[cfg(feature = "px8-ds-test-support")]
    if bypass_unknown_to_c2 {
        // Test-only: deliberately cross the exact unknown-member exit with
        // the non-member Vis. The C2 match must trap on this owner-fed value.
        builder
            .ins()
            .store(MemFlags::trusted(), returned_word, frame, result_offset);
        let success = builder.ins().iconst(types::I64, 0);
        builder.ins().return_(&[success]);
        compiler.record_checked_frame_terminal(builder, FrameTerminalKind::Normal)?;
    } else {
        let invalid = builder.ins().iconst(types::I64, -1);
        builder.ins().return_(&[invalid]);
    }
    #[cfg(not(feature = "px8-ds-test-support"))]
    {
        let invalid = builder.ins().iconst(types::I64, -1);
        builder.ins().return_(&[invalid]);
    }
    let offset_at = |start: i32, ordinal: usize| -> Result<i32, CraneliftBackendError> {
        let delta = ordinal
            .checked_mul(8)
            .and_then(|bytes| i32::try_from(bytes).ok())
            .ok_or_else(|| {
                backend_module(
                    LoweringCause::Capacity,
                    "pending-Vis owner payload offset exhausted".to_string(),
                )
            })?;
        start.checked_add(delta).ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "pending-Vis owner payload exceeds its activation frame".to_string(),
            )
        })
    };
    let selected_blocks = member_blocks.clone();
    let mut payload_loads = Vec::new();
    for (successor, selected) in successors.iter().zip(member_blocks) {
        builder.switch_to_block(selected);
        let mut row_loads = Vec::new();
        let row = &successor.row;
        // A stale record for an unrelated non-Ret constructor is never an
        // effect permission. Both the member and its exact Vis identity hold.
        let vis_identity = compiler
            .static_transition_plan
            .constructor_symbol_identity(row.vis_origin())?
            .tag_abi_word()?;
        let vis_identity = i64::try_from(vis_identity).map_err(|_| {
            backend_module(
                LoweringCause::Capacity,
                "pending-Vis constructor tag exceeds range".to_string(),
            )
        })?;
        Lowering::require_i64(builder, actual_tag, vis_identity);
        let fields = compiler.emit_carrier_field_count(builder, returned)?;
        Lowering::require_i64(builder, fields, 2);
        let mut inputs = BTreeMap::new();
        let target = successor_targets.get(&row.id()).ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "a pending-Vis successor K context was not declared".to_string(),
            )
        })?;
        if target.header.parameters as usize != row.captures().len() + 1
            || target.header.captures as usize != row.continuation_inputs().len()
        {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "a pending-Vis successor disagrees with its K-context ABI".to_string(),
            ));
        }
        for (index, capture) in row.captures().iter().enumerate() {
            if capture.ordinal() as usize != index {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    "pending-Vis capture ordinals are not dense".to_string(),
                ));
            }
            let word = builder.ins().load(
                types::I64,
                MemFlags::trusted(),
                frame,
                offset_at(region.captures, index)?,
            );
            if let cranelift_codegen::ir::ValueDef::Result(inst, _) =
                builder.func.dfg.value_def(word)
            {
                row_loads.push(inst);
            }
            if inputs
                .insert(
                    StaticResponseFrameSource::Parameter(capture.producer_abi_slot()),
                    LoweringOperand::Carried(CarriedBoundaryWord { word }),
                )
                .is_some()
            {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    "pending-Vis capture source repeats a destination slot".to_string(),
                ));
            }
        }
        for (index, (ordinal, _, abi_position)) in row.continuation_inputs().iter().enumerate() {
            if *ordinal as usize != index
                || !target
                    .slots
                    .get(*abi_position as usize)
                    .is_some_and(|slot| {
                        slot.kind == AbiSlotKind::Capture && slot.ordinal == *ordinal
                    })
            {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    "pending-Vis continuation input disagrees with the successor ABI".to_string(),
                ));
            }
            let word = builder.ins().load(
                types::I64,
                MemFlags::trusted(),
                frame,
                offset_at(region.continuation_inputs, index)?,
            );
            if let cranelift_codegen::ir::ValueDef::Result(inst, _) =
                builder.func.dfg.value_def(word)
            {
                row_loads.push(inst);
            }
            if inputs
                .insert(
                    StaticResponseFrameSource::Capture(*ordinal),
                    LoweringOperand::Carried(CarriedBoundaryWord { word }),
                )
                .is_some()
            {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    "pending-Vis continuation input repeats a slot".to_string(),
                ));
            }
        }
        // Single-use record: the next K call starts with no pending member.
        // Its callee's newly minted member is copied up only after Trap checks.
        let zero = builder.ins().iconst(types::I64, 0);
        let cleared = builder
            .ins()
            .store(MemFlags::trusted(), zero, frame, region.discriminant);
        compiler
            .function_local
            .pending_vis_slot_stores
            .push(PendingVisSlotStore {
                inst: cleared,
                offset: region.discriminant,
                source: PendingVisStoreSource::Cleared,
            });
        let nested =
            AmbientBodyAuthority::bind(compiler, row.base_owner(), row.effect_source_owner());
        let effect = lower_static_response_effect(
            compiler,
            builder,
            row,
            &successor.effect,
            &successor.operation_arguments,
            &inputs,
        );
        nested.release(compiler);
        let (response, host_validation_end) = effect?;
        let mut call_inputs = vec![response];
        for capture in row.captures() {
            call_inputs.push(
                inputs
                    .get(&StaticResponseFrameSource::Parameter(
                        capture.producer_abi_slot(),
                    ))
                    .cloned()
                    .ok_or_else(|| {
                        backend_module(
                            LoweringCause::PlannerInvariant,
                            "pending-Vis K capture has no recorded parameter".to_string(),
                        )
                    })?,
            );
        }
        for (ordinal, _, _) in row.continuation_inputs() {
            call_inputs.push(
                inputs
                    .get(&StaticResponseFrameSource::Capture(*ordinal))
                    .cloned()
                    .ok_or_else(|| {
                        backend_module(
                            LoweringCause::PlannerInvariant,
                            "pending-Vis K continuation input has no recorded capture".to_string(),
                        )
                    })?,
            );
        }
        let (next_result, call) = compiler.call_declared_unit_target(
            builder,
            target.clone(),
            &call_inputs,
            None,
            #[cfg(test)]
            None,
        )?;
        let LoweringOperand::Carried(next_result) = next_result else {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "pending-Vis successor K returned a template".to_string(),
            ));
        };
        let expected_context = successor_contexts.get(&row.id()).copied().ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "pending-Vis successor has no exact generated K target".to_string(),
            )
        })?;
        calls.push(PendingVisOwnerCall {
            row: row.id(),
            call,
            expected_context,
            host_validation_end,
            ret_identity: row.k_ret_identity().tag_abi_word()?,
        });
        let expected_ret = i64::try_from(row.k_ret_identity().tag_abi_word()?).map_err(|_| {
            backend_module(
                LoweringCause::Capacity,
                "pending-Vis successor Ret tag exceeds range".to_string(),
            )
        })?;
        let expected_ret = builder.ins().iconst(types::I64, expected_ret);
        builder
            .ins()
            .jump(result, &[next_result.word, expected_ret]);
        payload_loads.push(row_loads);
    }
    Ok(PendingVisFinishedBody {
        return_inst,
        initial_context_calls: initial_calls.len(),
        initial_response_is_current_host_result,
        calls,
        result_store,
        result_offset,
        returned_word,
        member_load,
        member_branches,
        member_ids,
        selected_blocks,
        payload_loads,
        placeholder_offset,
        input_frame: frame,
        record_discriminant_offset: region.discriminant,
    })
}

struct StaticResponseFinishedBody {
    input_frame: cranelift_codegen::ir::Value,
    placeholder_offset: i32,
    context_calls: Vec<cranelift_codegen::ir::Inst>,
    host_validation_end: cranelift_codegen::ir::Inst,
    ret_validation_end: cranelift_codegen::ir::Inst,
    result_store: cranelift_codegen::ir::Inst,
    returned_word: cranelift_codegen::ir::Value,
    return_inst: cranelift_codegen::ir::Inst,
    frame: cranelift_codegen::ir::Value,
    result_offset: i32,
    trap_offset: i32,
    response_is_current_host_result: bool,
    ret_abi_word: u64,
}

// Both response-owner verifiers reject this finished-code violation through
// one typed refusal, regardless of the text that explains the failing edge.
fn response_k_missing_status_trap_refusal() -> CraneliftBackendError {
    backend_module(
        LoweringCause::FinishedCode,
        "a response K call read Result without the status then Trap-before-Result branches"
            .to_string(),
    )
}

/// Check the loop's FINISHED control flow, not just the call handles supplied
/// during emission. Each incoming edge to the result join must come from one
/// exact generated K call after its own status and Trap branches. Thus neither
/// ResultWord nor the pending record is read on an unchecked callee path.
fn verify_pending_vis_finished_body(
    func: &Function,
    facts: &PendingVisFinishedBody,
    initial: &StaticResponseContinuation,
    successors: &[SuccessorEffect],
    region: PendingVisFrameRegion,
) -> Result<(), CraneliftBackendError> {
    use cranelift_codegen::ir::{InstructionData, Opcode, ValueDef};
    let error = |reason: &str| {
        backend_module(
            LoweringCause::FinishedCode,
            format!("pending-Vis finished-body verifier: {reason}",),
        )
    };
    if facts.initial_context_calls != 1 {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            format!(
                "a response owner emitted {} K calls instead of exactly one",
                facts.initial_context_calls,
            ),
        ));
    }
    if facts.calls.len() != successors.len() + 1
        || facts.member_branches.len() != successors.len()
        || facts.member_ids.len() != successors.len()
        || facts.selected_blocks.len() != successors.len()
        || facts.payload_loads.len() != successors.len()
        || facts.calls[0].row != initial.id()
        || facts.calls[0].ret_identity != initial.k_ret_identity().tag_abi_word()?
    {
        return Err(error(
            "the returned successor set or initial K call is incomplete",
        ));
    }
    for inst in func
        .layout
        .blocks()
        .flat_map(|block| func.layout.block_insts(block))
    {
        if let InstructionData::Load { offset, .. } = func.dfg.insts[inst] {
            if i32::from(offset) == facts.placeholder_offset
                && func.dfg.inst_args(inst).first() == Some(&facts.input_frame)
            {
                return Err(backend_module(
                    LoweringCause::FinishedCode,
                    "a response owner loaded its deferred parameter-zero placeholder".to_string(),
                ));
            }
        }
    }
    let InstructionData::Load { offset, .. } = func.dfg.insts[facts.member_load] else {
        return Err(error("member discrimination is not a finished frame load"));
    };
    if i32::from(offset) != facts.record_discriminant_offset
        || func.dfg.inst_args(facts.member_load).first() != Some(&facts.input_frame)
    {
        return Err(error("member discrimination reads a non-record location"));
    }
    let Some(&member_word) = func.dfg.inst_results(facts.member_load).first() else {
        return Err(error("member load has no word"));
    };
    let ValueDef::Param(result_block, 0) = func.dfg.value_def(facts.returned_word) else {
        return Err(error(
            "the returned K word is not the result join's first parameter",
        ));
    };
    if func.dfg.inst_args(facts.result_store).first() != Some(&facts.returned_word)
        || !matches!(
            func.dfg.insts[facts.result_store],
            InstructionData::Store { .. }
        )
    {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a response owner let a raw HostResult or non-K value escape to Result".to_string(),
        ));
    }
    let cfg = ControlFlowGraph::with_function(func);
    let mut dom = cranelift_codegen::dominator_tree::DominatorTree::new();
    dom.compute(func, &cfg);
    // Neither the owner nor a malformed initial K may collapse Result before
    // the first checked call. Read the actual finished stores, not a test flag.
    if func
        .layout
        .blocks()
        .flat_map(|block| func.layout.block_insts(block))
        .any(|inst| {
            matches!(func.dfg.insts[inst], InstructionData::Store { offset, .. }
            if i32::from(offset) == facts.result_offset)
                && func.dfg.inst_args(inst).get(1) == Some(&facts.input_frame)
                && dom.dominates(inst, facts.calls[0].call, &func.layout)
        })
    {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a response owner called K after its answer was already collapsed".to_string(),
        ));
    }
    if !dom.dominates(result_block, facts.member_load, &func.layout)
        || !dom.dominates(result_block, facts.result_store, &func.layout)
    {
        return Err(error("a record or Result read bypasses the checked K join"));
    }
    let incoming = cfg.pred_iter(result_block).collect::<Vec<_>>();
    if incoming.len() != facts.calls.len() {
        return Err(error(
            "a K result reached the join without a selected-row call",
        ));
    }
    let mut reached = BTreeSet::new();
    for edge in incoming {
        if func.dfg.insts[edge.inst].opcode() != Opcode::Jump {
            return Err(error("a K result join has a non-call predecessor"));
        }
        let dominating = facts
            .calls
            .iter()
            .filter(|candidate| dom.dominates(candidate.call, edge.inst, &func.layout))
            .collect::<Vec<_>>();
        let selected = dominating
            .iter()
            .copied()
            .find(|candidate| {
                dominating.iter().all(|other| {
                    other.call == candidate.call
                        || dom.dominates(other.call, candidate.call, &func.layout)
                })
            })
            .ok_or_else(|| error("a K result predecessor has no unique latest call"))?;
        if !reached.insert(selected.row) {
            return Err(error("one response row entered the K join twice"));
        }
        if Lowering::decode_direct_callee(func, selected.call)? != selected.expected_context {
            if selected.row == initial.id() {
                return Err(backend_module(LoweringCause::FinishedCode,
                    "a response owner called a context or raw worker other than its exact K context".to_string(),
                ));
            }
            return Err(error(
                "a selected response row called a non-exact K context",
            ));
        }
        let positions = func
            .layout
            .blocks()
            .flat_map(|block| func.layout.block_insts(block))
            .enumerate()
            .map(|(index, inst)| (inst, index))
            .collect::<BTreeMap<_, _>>();
        if positions[&selected.call] <= positions[&selected.host_validation_end] {
            if selected.row == initial.id() {
                return Err(backend_module(
                    LoweringCause::FinishedCode,
                    "a response owner called K before host response validation completed"
                        .to_string(),
                ));
            }
            return Err(error(
                "a selected K was called before its host effect validation",
            ));
        }
        let guards = positions
            .iter()
            .filter(|(inst, _)| {
                func.dfg.insts[**inst].opcode() == Opcode::Brif
                    && dom.dominates(selected.call, **inst, &func.layout)
                    && dom.dominates(**inst, edge.inst, &func.layout)
            })
            .map(|(inst, _)| *inst)
            .collect::<Vec<_>>();
        if guards.len() < 2 {
            if selected.row == initial.id() {
                return Err(response_k_missing_status_trap_refusal());
            }
            return Err(error("a selected K result lacks status then Trap branches"));
        }
        let jumps = func.dfg.insts[edge.inst].branch_destination(&func.dfg.jump_tables);
        let Some(destination) = jumps.first() else {
            return Err(error("a K result predecessor has no join destination"));
        };
        if destination.block(&func.dfg.value_lists) != result_block {
            return Err(error(
                "a K result predecessor jumps outside the checked join",
            ));
        }
        let args = destination.args_slice(&func.dfg.value_lists);
        let Some((&word, &expected)) = args.first().zip(args.get(1)) else {
            return Err(error(
                "a K result join is missing its result or exact Ret identity",
            ));
        };
        let ValueDef::Result(word_load, _) = func.dfg.value_def(word) else {
            return Err(error("a K result was not read from its callee frame"));
        };
        if func.dfg.insts[word_load].opcode() != Opcode::StackLoad
            || !guards
                .iter()
                .all(|guard| dom.dominates(*guard, word_load, &func.layout))
        {
            return Err(error(
                "a K ResultWord was read before the status/Trap checks",
            ));
        }
        let ValueDef::Result(expected_inst, _) = func.dfg.value_def(expected) else {
            return Err(error("a K Ret identity was not minted for its exact row"));
        };
        let InstructionData::UnaryImm {
            opcode: Opcode::Iconst,
            imm,
        } = func.dfg.insts[expected_inst]
        else {
            return Err(error("a K Ret identity is not a literal from its row"));
        };
        if imm.bits()
            != i64::try_from(selected.ret_identity).map_err(|_| {
                backend_module(
                    LoweringCause::Capacity,
                    "pending-Vis finished-body verifier: a K Ret identity exceeds the word range"
                        .to_string(),
                )
            })?
        {
            if selected.row == initial.id() {
                return Err(backend_module(
                    LoweringCause::FinishedCode,
                    "a response owner validated a Ret identity other than its exact K Ret"
                        .to_string(),
                ));
            }
            return Err(error("a K Ret identity disagrees with the selected row"));
        }
    }
    if facts.calls.iter().any(|call| !reached.contains(&call.row)) {
        return Err(error("a selected-row K call has no checked result edge"));
    }
    if !facts.initial_response_is_current_host_result {
        return Err(backend_module(LoweringCause::FinishedCode,
            "a response owner substituted operation, prior-response, or application-environment authority for the current HostResult".to_string(),
        ));
    }
    for (index, successor) in successors.iter().enumerate() {
        let row = &successor.row;
        if facts.calls[index + 1].row != row.id()
            || facts.calls[index + 1].ret_identity != row.k_ret_identity().tag_abi_word()?
            || facts.member_ids[index] != row.id()
        {
            return Err(error("a successor's K, Ret and member identities disagree"));
        }
        let branch = facts.member_branches[index];
        let destinations = func.dfg.insts[branch].branch_destination(&func.dfg.jump_tables);
        if destinations
            .first()
            .map(|dest| dest.block(&func.dfg.value_lists))
            != Some(facts.selected_blocks[index])
        {
            return Err(error(
                "a member check does not select its recorded successor block",
            ));
        }
        let Some(&condition) = func.dfg.inst_args(branch).first() else {
            return Err(error("a successor branch has no condition"));
        };
        let ValueDef::Result(compare, _) = func.dfg.value_def(condition) else {
            return Err(error("a successor branch does not compare the member"));
        };
        let InstructionData::IntCompareImm {
            opcode: Opcode::IcmpImm,
            arg,
            imm,
            cond,
        } = func.dfg.insts[compare]
        else {
            return Err(error(
                "a successor branch is not an exact member comparison",
            ));
        };
        if arg != member_word
            || cond != cranelift_codegen::ir::condcodes::IntCC::Equal
            || imm.bits() != i64::from(row.id().ordinal()) + 1
        {
            return Err(error(
                "a successor branch did not check its exact nonzero member",
            ));
        }
        if facts.payload_loads[index].len()
            != row.captures().len() + row.continuation_inputs().len()
        {
            return Err(error(
                "a successor did not read its complete capture payload",
            ));
        }
        for (position, load) in facts.payload_loads[index].iter().enumerate() {
            if !dom.dominates(branch, *load, &func.layout)
                || !dom.dominates(facts.selected_blocks[index], *load, &func.layout)
            {
                return Err(error("a payload read bypassed its member check"));
            }
            let base = if position < row.captures().len() {
                region.captures
                    + i32::try_from(position * 8).map_err(|_| error("capture offset exhausted"))?
            } else {
                region.continuation_inputs
                    + i32::try_from((position - row.captures().len()) * 8)
                        .map_err(|_| error("continuation input offset exhausted"))?
            };
            let InstructionData::Load { offset, .. } = func.dfg.insts[*load] else {
                return Err(error(
                    "a successor payload did not come from its activation frame",
                ));
            };
            if i32::from(offset) != base
                || func.dfg.inst_args(*load).first() != Some(&facts.input_frame)
            {
                return Err(error(
                    "a successor payload load names an unplanned record slot",
                ));
            }
        }
    }
    Ok(())
}

/// Validate the response-owner relation from the finished Function rather than
/// from the declarations handed to its builder. The exact call target, call
/// cardinality, validation order, Trap-before-Result branch count, and the SSA
/// value stored to Result are all read back from finalized CLIF.
fn verify_static_response_finished_body(
    func: &Function,
    expected_context: FuncId,
    expected_ret_abi_word: u64,
    tag_helper: FuncId,
    field_count_helper: FuncId,
    boundary_arena: cranelift_codegen::ir::Value,
    facts: &StaticResponseFinishedBody,
    call_obligations: &[PendingCallResultObligation],
) -> Result<(), CraneliftBackendError> {
    let func = &IndexedFunction::new(func);
    if facts.context_calls.len() != 1 {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            format!(
                "a response owner emitted {} K calls instead of exactly one",
                facts.context_calls.len(),
            ),
        ));
    }
    let call = facts.context_calls[0];
    // Parameter zero is an inert placeholder, not a K input. Check the
    // finished function, not only the source-side frame_inputs construction:
    // any direct load of that exact input-frame slot could flow into K.
    for inst in func
        .layout
        .blocks()
        .flat_map(|block| func.layout.block_insts(block))
    {
        if let cranelift_codegen::ir::InstructionData::Load { offset, .. } = func.dfg.insts[inst] {
            if i32::from(offset) == facts.placeholder_offset
                && func.dfg.inst_args(inst).first() == Some(&facts.input_frame)
            {
                return Err(backend_module(
                    LoweringCause::FinishedCode,
                    "a response owner loaded its deferred parameter-zero placeholder".to_string(),
                ));
            }
        }
    }
    if Lowering::decode_direct_callee(func, call)? != expected_context {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a response owner called a context or raw worker other than its exact K context"
                .to_string(),
        ));
    }
    let finished_exact_calls = func
        .layout
        .blocks()
        .flat_map(|block| func.layout.block_insts(block))
        .filter(|inst| {
            matches!(
                func.dfg.insts[*inst],
                cranelift_codegen::ir::InstructionData::Call { .. }
            ) && Lowering::decode_direct_callee(func, *inst).ok() == Some(expected_context)
        })
        .count();
    if finished_exact_calls != 1 {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            format!(
                "finished CLIF contains {finished_exact_calls} exact K-context calls instead of one",
            ),
        ));
    }
    verify_all_paths_after_instruction(
        func,
        facts.host_validation_end,
        call,
        "a response owner called K before host response validation completed",
    )?;
    verify_all_paths_after_instruction(
        func,
        call,
        facts.result_store,
        "a response owner called K after its answer was already collapsed",
    )?;
    let matching_obligations = call_obligations
        .iter()
        .filter(|obligation| {
            obligation.call == call && obligation.result_word == facts.returned_word
        })
        .collect::<Vec<_>>();
    let [call_obligation] = matching_obligations.as_slice() else {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            format!(
                "a response K call has {} matching finished Result obligations instead of exactly one",
                matching_obligations.len(),
            ),
        ));
    };
    verify_call_result_obligation(func, call_obligation, expected_context)
        .map_err(|_| response_k_missing_status_trap_refusal())?;
    let stored = func
        .dfg
        .inst_args(facts.result_store)
        .first()
        .copied()
        .ok_or_else(|| {
            backend_module(
                LoweringCause::FinishedCode,
                "a response-owner Result store has no value operand".to_string(),
            )
        })?;
    if stored != facts.returned_word {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a response owner let a raw HostResult or non-K value escape to Result".to_string(),
        ));
    }
    if !facts.response_is_current_host_result {
        return Err(backend_module(LoweringCause::FinishedCode,
            "a response owner substituted operation, prior-response, or application-environment authority for the current HostResult"
                .to_string(),
        ));
    }
    if facts.ret_abi_word != expected_ret_abi_word {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a response owner validated a Ret identity other than its exact K Ret".to_string(),
        ));
    }
    match &func.dfg.insts[facts.result_store] {
        cranelift_codegen::ir::InstructionData::Store { args, offset, .. }
            if args[0] == facts.returned_word
                && args[1] == facts.frame
                && i32::from(*offset) == facts.result_offset => {}
        _ => {
            return Err(backend_module(
                LoweringCause::FinishedCode,
                "a response owner does not publish the validated word to its exact Result frame"
                    .to_string(),
            ));
        }
    }
    if func.dfg.insts[facts.return_inst].opcode() != cranelift_codegen::ir::Opcode::Return
        || func
            .dfg
            .inst_args(facts.return_inst)
            .first()
            .and_then(|word| iconst_value(func, *word))
            != Some(0)
    {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a response owner's certified Result publication does not return status zero"
                .to_string(),
        ));
    }
    let tag_queries = verify_carrier_queries(func, tag_helper, boundary_arena)?
        .into_iter()
        .filter(|query| query.word == facts.returned_word)
        .collect::<Vec<_>>();
    let [tag_query] = tag_queries.as_slice() else {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            format!(
                "a response owner has {} exact returned-word tag queries instead of one",
                tag_queries.len()
            ),
        ));
    };
    verify_exact_value_guard(
        func,
        tag_query.output,
        i64::try_from(expected_ret_abi_word).map_err(|_| {
            backend_module(
                LoweringCause::FinishedCode,
                "response Ret identity exceeds the runtime tag word".to_string(),
            )
        })?,
        facts.result_store,
    )?;
    let field_queries = verify_carrier_queries(func, field_count_helper, boundary_arena)?
        .into_iter()
        .filter(|query| query.word == facts.returned_word)
        .collect::<Vec<_>>();
    let [field_query] = field_queries.as_slice() else {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            format!(
                "a response owner has {} exact returned-word field-count queries instead of one",
                field_queries.len()
            ),
        ));
    };
    verify_exact_value_guard(func, field_query.output, 1, facts.result_store)?;
    verify_all_paths_after_instruction(
        func,
        tag_query.call,
        field_query.call,
        "a response owner's tag/arity proofs do not precede Result publication in order",
    )?;
    verify_all_paths_after_instruction(
        func,
        field_query.call,
        facts.result_store,
        "a response owner's tag/arity proofs do not precede Result publication in order",
    )?;
    let returned_inst = match func.dfg.value_def(facts.returned_word) {
        cranelift_codegen::ir::ValueDef::Result(inst, _) => inst,
        _ => {
            return Err(backend_module(
                LoweringCause::FinishedCode,
                "a response K Result is not defined by one finished CLIF instruction".to_string(),
            ));
        }
    };
    verify_all_paths_after_instruction(
        func,
        call,
        returned_inst,
        "a response K call Result producer",
    )?;
    verify_carrier_fact_current(
        func,
        &[CarrierFactGround {
            producer: returned_inst,
            guard: None,
        }],
        boundary_arena,
        &BTreeSet::from([facts.returned_word]),
        &BTreeSet::from([tag_helper, field_count_helper]),
        facts.result_store,
        "a response owner's returned constructor word",
    )?;
    Ok(())
}

fn verify_call_result_obligation(
    func: &IndexedFunction<'_>,
    obligation: &PendingCallResultObligation,
    certified_target: FuncId,
) -> Result<(), CraneliftBackendError> {
    if Lowering::decode_direct_callee(func, obligation.call)? != certified_target {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a call-result obligation's finished direct callee disagrees with its certificate"
                .to_string(),
        ));
    }
    if func.dfg.inst_results(obligation.call) != [obligation.status] {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a call-result obligation does not bind the call's exact status result".to_string(),
        ));
    }
    let call_args = func.dfg.inst_args(obligation.call);
    let [actual_header, _services] = call_args else {
        return Err(backend_module(LoweringCause::FinishedCode,
            "a call-result obligation's actual call does not carry the exact header and services arguments"
                .to_string(),
        ));
    };
    let Some((header_slot, header_base)) = resolve_stack_address(func, *actual_header)? else {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a call-result obligation's actual header is not a finalized stack address".to_string(),
        ));
    };
    if header_base != 0 {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a call-result obligation's actual header does not begin at its frame base".to_string(),
        ));
    }
    let header = &func.sized_stack_slots[header_slot];
    if header.size
        != u32::try_from(crate::activation_services::UNIT_CALL_FRAME_BYTES)
            .expect("the unit-call header size fits u32")
        || header.align_shift != 3
    {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a call-result obligation's actual header allocation disagrees with UnitBundle layout"
                .to_string(),
        ));
    }
    let header_payload = StackMemoryLocation {
        slot: header_slot,
        offset: i64::from(crate::activation_services::UNIT_CALL_FRAME_SLOTS),
        bytes: 8,
    };
    let actual_payload = verify_definite_stored_value(
        func,
        header_payload,
        obligation.call,
        "a call-result obligation's header payload pointer",
    )?;
    if resolve_stack_address(func, actual_payload)? != Some((obligation.payload, 0)) {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a call-result obligation's actual call header points to a foreign payload".to_string(),
        ));
    }

    let payload_slot = &func.sized_stack_slots[obligation.payload];
    if payload_slot.size != obligation.frame_bytes || payload_slot.align_shift != 3 {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a call-result obligation's actual frame allocation disagrees with its descriptor"
                .to_string(),
        ));
    }
    let trap_location = StackMemoryLocation {
        slot: obligation.payload,
        offset: i64::from(obligation.trap_offset),
        bytes: 8,
    };
    let result_location = StackMemoryLocation {
        slot: obligation.payload,
        offset: i64::from(obligation.result_offset),
        bytes: 8,
    };
    let initialized_trap = verify_definite_stored_value(
        func,
        trap_location,
        obligation.call,
        "a call-result obligation's Trap initializer",
    )?;
    if iconst_value(func, initialized_trap) != Some(0) {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a call-result obligation does not initialize Trap to zero on every reaching path"
                .to_string(),
        ));
    }

    if !compare_imm_is(
        func,
        obligation.status_compare,
        obligation.status,
        cranelift_codegen::ir::condcodes::IntCC::NotEqual,
        0,
    ) {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a call-result obligation lacks its exact status-nonzero comparison".to_string(),
        ));
    }
    let cranelift_codegen::ir::InstructionData::Brif {
        arg: status_arg,
        blocks: status_blocks,
        ..
    } = &func.dfg.insts[obligation.status_branch]
    else {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a call-result obligation's status locator is not a branch".to_string(),
        ));
    };
    if *status_arg != obligation.status_compare {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a call-result obligation's status branch uses another predicate".to_string(),
        ));
    }
    let status_guard = ExactProtocolGuard {
        branch: obligation.status_branch,
        success_ordinal: 1,
        success_target: status_blocks[1].block(&func.dfg.value_lists),
    };

    let (trap_load, actual_trap_location) = loaded_stack_location(func, obligation.trap_word)?
        .ok_or_else(|| {
            backend_module(
                LoweringCause::FinishedCode,
                "a call-result obligation's Trap word is not an exact stack-location load"
                    .to_string(),
            )
        })?;
    if actual_trap_location != trap_location {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a call-result obligation reads Trap from another frame or offset".to_string(),
        ));
    }
    verify_all_paths_guarded(
        func,
        obligation.call,
        status_guard,
        trap_load,
        "a call-result obligation's Trap load",
    )?;
    verify_all_paths_memory_current(
        func,
        obligation.call,
        trap_location,
        trap_load,
        "a call-result obligation's Trap load",
    )?;

    if !compare_imm_is(
        func,
        obligation.trap_compare,
        obligation.trap_word,
        cranelift_codegen::ir::condcodes::IntCC::NotEqual,
        0,
    ) {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a call-result obligation lacks its exact Trap-nonzero comparison".to_string(),
        ));
    }
    let cranelift_codegen::ir::InstructionData::Brif {
        arg: trap_arg,
        blocks: trap_blocks,
        ..
    } = &func.dfg.insts[obligation.trap_branch]
    else {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a call-result obligation's Trap locator is not a branch".to_string(),
        ));
    };
    if *trap_arg != obligation.trap_compare {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a call-result obligation's Trap branch uses another predicate".to_string(),
        ));
    }
    let trap_guard = ExactProtocolGuard {
        branch: obligation.trap_branch,
        success_ordinal: 1,
        success_target: trap_blocks[1].block(&func.dfg.value_lists),
    };

    let (result_load, actual_result_location) =
        loaded_stack_location(func, obligation.result_word)?.ok_or_else(|| {
            backend_module(
                LoweringCause::FinishedCode,
                "a call-result obligation's Result word is not an exact stack-location load"
                    .to_string(),
            )
        })?;
    if actual_result_location != result_location {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a call-result obligation reads Result from another frame or offset".to_string(),
        ));
    }
    verify_all_paths_guarded(
        func,
        trap_load,
        trap_guard,
        result_load,
        "a call-result obligation's Result load",
    )?;
    verify_all_paths_memory_current(
        func,
        obligation.call,
        result_location,
        result_load,
        "a call-result obligation's Result load",
    )?;
    Ok(())
}

/// Define the specialized owner at the host-response seam, then call its exact
/// K execution context with the validated response as operand zero.
///
/// The host-effect lowering is emitted directly into each owner Function. No
/// shared HostResult helper function or runtime selector exists: the planner's
/// typed response row chooses the effect occurrence, explicit frame inputs, and
/// one context target before code generation.
pub(super) fn stage_static_response_owner_bodies<M: Module>(
    module: &mut M,
    compiler: &mut Lowering<'_>,
    helpers: ArtifactHelpers<'_>,
    bundle: &UnitBundle,
    call_edges: &CallEdgeTargets,
) -> Result<Vec<StagedResultBody>, CraneliftBackendError> {
    let rows = compiler
        .static_transition_plan
        .static_response_feasibility_ledger_all()?
        .map_err(|infeasible| {
            backend_module(
                LoweringCause::Fragment,
                format!(
                    "compile-time response specialization is infeasible at {:?}: {}",
                    infeasible.vis_origin(),
                    infeasible.reason(),
                ),
            )
        })?;
    let owners = compiler
        .static_transition_plan
        .static_response_owner_specializations()?
        .map_err(|infeasible| {
            backend_module(
                LoweringCause::Fragment,
                format!(
                    "compile-time response specialization is infeasible at {:?}: {}",
                    infeasible.vis_origin(),
                    infeasible.reason(),
                ),
            )
        })?;
    if owners.len() != rows.len() {
        return Err(backend_module(
            LoweringCause::PlannerInvariant,
            "the response-owner definition population disagrees with the validated response rows"
                .to_string(),
        ));
    }

    struct OwnedResponseEmission {
        owner: StaticResponseOwnerSpecialization,
        row: StaticResponseContinuation,
        effect: RuntimeExpr,
        operation_arguments: BTreeMap<StaticOriginId, RuntimeExpr>,
        successors: Vec<SuccessorEffect>,
        offsets: Vec<u32>,
    }
    let mut emissions = Vec::with_capacity(owners.len());
    for (owner, row) in owners.into_iter().zip(rows) {
        if owner.response() != row.id()
            || owner.base_owner() != row.base_owner()
            || owner.selected_caller() != row.k_identity()
            || owner.k_context() != row.k_context()
        {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "a response-owner definition disagrees with its validated response row".to_string(),
            ));
        }
        let effect = compiler
            .retained_body_occurrence(row.effect_origin())?
            .expr
            .clone();
        let RuntimeExpr::Effect { operation, .. } = &effect else {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "a static response row's effect origin no longer names an Effect".to_string(),
            ));
        };
        if *operation != row.operation() {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "a static response row's effect occurrence changed operation".to_string(),
            ));
        }
        let mut operation_arguments = BTreeMap::new();
        for input in row.effect_environment() {
            if let StaticResponseEffectInput::OperationArgument { origin, .. } = input {
                operation_arguments
                    .entry(*origin)
                    .or_insert(compiler.retained_body_occurrence(*origin)?.expr.clone());
            }
        }
        let successors = compiler
            .pending_vis_record_protocol
            .as_ref()
            .and_then(|protocol| protocol.owners.get(&row.id()))
            .map(|protocol| {
                protocol
                    .contexts
                    .iter()
                    .flat_map(|context| &context.members)
                    .map(|member| {
                        member.successor.as_ref().ok_or_else(|| {
                            backend_module(
                                LoweringCause::PlannerInvariant,
                                "a pending-Vis member has no derived response row".to_string(),
                            )
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?
            .unwrap_or_default();
        let mut distinct_successors = BTreeMap::new();
        for successor in successors {
            distinct_successors
                .entry(successor.id())
                .or_insert_with(|| (*successor).clone());
        }
        let successors = distinct_successors.into_values().map(|row| {
            let effect = compiler.retained_body_occurrence(row.effect_origin())?.expr.clone();
            if !matches!(&effect, RuntimeExpr::Effect { operation, .. } if *operation == row.operation()) {
                return Err(backend_module(LoweringCause::PlannerInvariant, "a pending-Vis successor has no exact host effect".to_string()));
            }
            let mut operation_arguments = BTreeMap::new();
            for input in row.effect_environment() {
                if let StaticResponseEffectInput::OperationArgument { origin, .. } = input {
                    operation_arguments.entry(*origin).or_insert(
                        compiler.retained_body_occurrence(*origin)?.expr.clone(),
                    );
                }
            }
            Ok(SuccessorEffect { row, effect, operation_arguments })
        }).collect::<Result<Vec<_>, CraneliftBackendError>>()?;
        let (offsets, frame_bytes) = owner.slot_offsets()?;
        if frame_bytes != owner.header().frame_bytes {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "a response-owner frame size disagrees with its slot run".to_string(),
            ));
        }
        emissions.push(OwnedResponseEmission {
            owner,
            row,
            effect,
            operation_arguments,
            successors,
            offsets,
        });
    }

    let worker_targets = resolve_worker_targets(&compiler.static_transition_plan, bundle)?;
    let mut staged = Vec::new();
    for emission in emissions {
        // Derive this function's closeout from exactly the source inputs it
        // lowers, including every pending-Vis successor's own effect inputs.
        let mut required = BTreeSet::new();
        for root in std::iter::once(emission.row.effect_origin())
            .chain(emission.operation_arguments.keys().copied())
            .chain(emission.successors.iter().flat_map(|successor| {
                std::iter::once(successor.row.effect_origin())
                    .chain(successor.operation_arguments.keys().copied())
            }))
        {
            required.extend(
                compiler
                    .static_transition_plan
                    .source_join_origins_in_owner_subtree(root)?,
            );
        }
        let id = bundle.response(emission.owner.id()).ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "a response owner was never forward-declared".to_string(),
            )
        })?;
        let slots = emission.owner.slots();
        let offsets = emission.offsets.as_slice();
        if slots.len() != offsets.len() {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "a response-owner slot run disagrees with its offset walk".to_string(),
            ));
        }
        let parameter_count = slots
            .iter()
            .filter(|slot| slot.kind == AbiSlotKind::Parameter)
            .count();
        let capture_count = slots
            .iter()
            .filter(|slot| slot.kind == AbiSlotKind::Capture)
            .count();
        if parameter_count != emission.row.captures().len() + 1
            || capture_count != emission.row.continuation_inputs().len()
            || u32::try_from(parameter_count).ok() != Some(emission.owner.header().parameters)
            || u32::try_from(capture_count).ok() != Some(emission.owner.header().captures)
        {
            return Err(backend_module(LoweringCause::PlannerInvariant,
                "a response-owner frame does not partition as operation plus K captures plus continuation inputs"
                    .to_string(),
            ));
        }
        let result_offset = slots
            .iter()
            .zip(offsets)
            .find(|(slot, _)| slot.kind == AbiSlotKind::Result)
            .map(|(_, offset)| *offset)
            .ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "response-owner frame declares no result slot".to_string(),
                )
            })?;
        let placeholder_offset = slots
            .iter()
            .zip(offsets)
            .find(|(slot, _)| slot.kind == AbiSlotKind::Parameter && slot.ordinal == 0)
            .map(|(_, offset)| i32::try_from(*offset))
            .ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "response owner has no parameter-zero placeholder slot".to_string(),
                )
            })?
            .map_err(|_| {
                backend_module(
                    LoweringCause::Capacity,
                    "response-owner placeholder slot offset exceeds range".to_string(),
                )
            })?;
        let trap_offset = slots
            .iter()
            .zip(offsets)
            .find(|(slot, _)| slot.kind == AbiSlotKind::Trap)
            .map(|(_, offset)| *offset)
            .ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "response-owner frame declares no trap slot".to_string(),
                )
            })?;

        compiler.open_aggregate_events(id)?;
        let sig = unit_signature(module);
        let mut func = Function::with_name_signature(UserFuncName::user(5, id.as_u32()), sig);
        let mut function_local = helpers.declare_in_func(module, &mut func, None);
        function_local.grafted_spine_scope =
            Some(GraftedSpineFunctionScope::Response(emission.owner.id()));
        let declared_calls =
            call_edges.declare_in_func(emission.row.operation_source_owner(), module, &mut func)?;
        function_local.unit_calls = declared_calls.static_bodies;
        call_edges.declare_retained_body_targets_in_func(
            emission.row.operation_source_owner(),
            module,
            &mut func,
            &mut function_local.unit_calls,
        )?;
        function_local.declaration_calls = declared_calls.declarations;
        for successor in &emission.successors {
            let declared = call_edges.declare_in_func(
                successor.row.operation_source_owner(),
                module,
                &mut func,
            )?;
            for (site, call) in declared.static_bodies {
                if let Some(previous) = function_local.unit_calls.insert(site, call.clone()) {
                    if previous.origin != call.origin || previous.header != call.header {
                        return Err(backend_module(
                            LoweringCause::PlannerInvariant,
                            "two pending-Vis effect owners disagree on one generated call target"
                                .to_string(),
                        ));
                    }
                }
            }
            call_edges.declare_retained_body_targets_in_func(
                successor.row.operation_source_owner(),
                module,
                &mut func,
                &mut function_local.unit_calls,
            )?;
            for (site, call) in declared.declarations {
                if let Some(previous) = function_local.declaration_calls.insert(site, call.clone())
                {
                    if previous.origin != call.origin || previous.header != call.header {
                        return Err(backend_module(
                            LoweringCause::PlannerInvariant,
                            "two pending-Vis effect owners disagree on one declaration target"
                                .to_string(),
                        ));
                    }
                }
            }
        }
        function_local.worker_calls = worker_targets.declare_in_func(module, &mut func);
        function_local.raw_worker_calls = function_local.worker_calls.clone();
        function_local.worker_templates = worker_targets.templates().clone();
        let expected_context_target =
            bundle.context(emission.owner.k_context()).ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "a response owner's exact K context was never declared".to_string(),
                )
            })?;
        #[cfg(feature = "px8-ds-test-support")]
        let pending_vis_owner = compiler
            .static_transition_plan
            .selected_pending_response_at_vis(emission.row.vis_origin())?
            .is_some();
        #[cfg(feature = "px8-ds-test-support")]
        let body_mutation = claim_static_response_owner_body_mutation(|mutation| match mutation {
            StaticResponseOwnerBodyMutation::BypassRetValidationAndReturnVis => pending_vis_owner,
            StaticResponseOwnerBodyMutation::SubstituteContextZero => compiler
                .static_transition_plan
                .continuation_contexts()
                .is_ok_and(|contexts| {
                    contexts
                        .first()
                        .is_some_and(|context| context.id() != emission.owner.k_context())
                }),
            StaticResponseOwnerBodyMutation::ResponseWithPriorResponse => {
                emission.owner.id().ordinal() > 0
            }
            StaticResponseOwnerBodyMutation::ResponseWithApplicationEnvironment => {
                emission.row.operation() == crate::HostOpV1::BufferAllocate
                    && emission.owner.header().parameters > 1
            }
            _ => true,
        });
        #[cfg(feature = "px8-ds-test-support")]
        let selected_context = match body_mutation {
            Some(StaticResponseOwnerBodyMutation::SubstituteContextZero) => {
                let first = compiler
                    .static_transition_plan
                    .continuation_contexts()?
                    .first()
                    .map(|context| context.id())
                    .ok_or_else(|| {
                        backend_module(
                            LoweringCause::PlannerInvariant,
                            "the context-zero response mutation found no installed context"
                                .to_string(),
                        )
                    })?;
                let target = bundle.context(first).ok_or_else(|| {
                    backend_module(
                        LoweringCause::PlannerInvariant,
                        "the context-zero response mutation found no declared target".to_string(),
                    )
                })?;
                let mut call = declare_response_context_call_in_func(
                    module,
                    &mut func,
                    &compiler.static_transition_plan,
                    bundle,
                    emission.owner.k_context(),
                )?;
                call.function = module.declare_func_in_func(target, &mut func);
                call
            }
            Some(StaticResponseOwnerBodyMutation::CallRawWorker) => {
                let target = bundle
                    .continuation(emission.row.k_specialization())
                    .ok_or_else(|| {
                        backend_module(
                            LoweringCause::PlannerInvariant,
                            "the raw-worker response mutation found no K worker declaration"
                                .to_string(),
                        )
                    })?;
                DeclaredUnitCall {
                    function: module.declare_func_in_func(target, &mut func),
                    frame_owner: Some(PendingVisFrameOwner::Continuation(
                        emission.row.k_specialization(),
                    )),
                    origin: emission.row.k_body_origin(),
                    call_site_origin: emission.row.k_body_origin(),
                    header: emission.owner.header(),
                    slots: emission.owner.slots().to_vec(),
                    offsets: emission.offsets.clone(),
                    result_contract: None,
                }
            }
            _ => declare_response_context_call_in_func(
                module,
                &mut func,
                &compiler.static_transition_plan,
                bundle,
                emission.owner.k_context(),
            )?,
        };
        #[cfg(not(feature = "px8-ds-test-support"))]
        let selected_context = declare_response_context_call_in_func(
            module,
            &mut func,
            &compiler.static_transition_plan,
            bundle,
            emission.owner.k_context(),
        )?;
        function_local
            .context_calls
            .insert(emission.owner.k_context(), selected_context.clone());
        let successor_contexts = emission
            .successors
            .iter()
            .map(|successor| {
                let context = bundle.context(successor.row.k_context()).ok_or_else(|| {
                    backend_module(
                        LoweringCause::PlannerInvariant,
                        "a pending-Vis successor K context was never generated".to_string(),
                    )
                })?;
                Ok((successor.row.id(), context))
            })
            .collect::<Result<BTreeMap<_, _>, CraneliftBackendError>>()?;
        let successor_targets = emission
            .successors
            .iter()
            .map(|successor| {
                let target = declare_response_context_call_in_func(
                    module,
                    &mut func,
                    &compiler.static_transition_plan,
                    bundle,
                    successor.row.k_context(),
                )?;
                function_local
                    .context_calls
                    .insert(successor.row.k_context(), target.clone());
                Ok((successor.row.id(), target))
            })
            .collect::<Result<BTreeMap<_, _>, CraneliftBackendError>>()?;
        let frame_scope = CheckedFrameFunctionScope::open(compiler)?;
        let token_scope = CheckedTokenFunctionScope::open(compiler)?;
        let ambient = AmbientBodyAuthority::bind(
            compiler,
            emission.owner.base_owner(),
            emission.row.effect_source_owner(),
        );
        let mut func_ctx = FunctionBuilderContext::new();
        let mut finished_body = None;
        let mut pending_finished = None;
        {
            let mut builder = FunctionBuilder::new(&mut func, &mut func_ctx);
            let entry = builder.create_block();
            builder.append_block_params_for_function_params(entry);
            builder.switch_to_block(entry);
            let envelope = builder.block_params(entry)[0];
            let frame = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                envelope,
                crate::activation_services::UNIT_CALL_FRAME_SLOTS,
            );
            let host_dispatch_context = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                envelope,
                crate::activation_services::UNIT_CALL_FRAME_HOST_DISPATCH_CONTEXT,
            );
            let services = builder.block_params(entry)[1];
            let native_int_arena = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                services,
                crate::activation_services::SERVICES_NATIVE_INT_ARENA,
            );
            Lowering::require_nonzero(&mut builder, native_int_arena);
            let boundary_arena = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                services,
                crate::activation_services::SERVICES_BOUNDARY_ARENA,
            );
            Lowering::require_nonzero(&mut builder, boundary_arena);
            function_local.host_dispatch_context = Some(host_dispatch_context);
            function_local.native_int_arena = Some(native_int_arena);
            function_local.boundary_arena = Some(boundary_arena);
            function_local.services_pointer = Some(services);
            function_local.bind_unit_trap_frame(
                frame,
                i32::try_from(trap_offset).map_err(|_| {
                    backend_module(
                        LoweringCause::Capacity,
                        "response-owner trap slot offset exceeds range".to_string(),
                    )
                })?,
            )?;
            function_local.bind_pending_vis_frame(
                compiler.pending_vis_record_protocol.as_ref(),
                PendingVisFrameOwner::ResponseOwner(emission.row.id()),
                frame,
                emission.owner.header().frame_bytes,
            )?;
            #[cfg(feature = "px8-ds-test-support")]
            record_pending_vis_owner_frame_observation(
                emission.row.id(),
                emission.row.vis_origin(),
                emission.owner.header().frame_bytes,
                function_local.pending_vis_frame,
            );

            let mut frame_inputs = BTreeMap::new();
            let mut descriptor_inputs = Vec::new();
            for (slot, offset) in slots.iter().zip(offsets) {
                let frame_source = match slot.kind {
                    AbiSlotKind::Parameter if slot.ordinal == 0 => None,
                    AbiSlotKind::Parameter => {
                        Some(StaticResponseFrameSource::Parameter(slot.ordinal))
                    }
                    AbiSlotKind::Capture => Some(StaticResponseFrameSource::Capture(slot.ordinal)),
                    AbiSlotKind::Control
                    | AbiSlotKind::Store
                    | AbiSlotKind::Trap
                    | AbiSlotKind::Result => None,
                };
                let Some(frame_source) = frame_source else {
                    continue;
                };
                let offset = i32::try_from(*offset).map_err(|_| {
                    backend_module(
                        LoweringCause::Capacity,
                        "response-owner input slot offset exceeds range".to_string(),
                    )
                })?;
                let operand = LoweringOperand::Carried(CarriedBoundaryWord {
                    word: builder
                        .ins()
                        .load(types::I64, MemFlags::trusted(), frame, offset),
                });
                if frame_inputs.insert(frame_source, operand.clone()).is_some() {
                    return Err(backend_module(
                        LoweringCause::PlannerInvariant,
                        "a response-owner frame repeats one typed input slot".to_string(),
                    ));
                }
                descriptor_inputs.push(operand);
            }
            function_local.defining_abi_operands = descriptor_inputs;
            function_local.static_response_owner = Some(emission.owner.id());
            compiler.function_local = function_local;

            let mut context_suffix = Vec::with_capacity(
                emission.row.captures().len() + emission.row.continuation_inputs().len(),
            );
            for capture in emission.row.captures() {
                context_suffix.push(
                    frame_inputs
                        .get(&StaticResponseFrameSource::Parameter(
                            capture.producer_abi_slot(),
                        ))
                        .cloned()
                        .ok_or_else(|| {
                            backend_module(
                                LoweringCause::PlannerInvariant,
                                "a response K capture names an absent owner Parameter slot"
                                    .to_string(),
                            )
                        })?,
                );
                #[cfg(feature = "px8-ds-test-support")]
                if RT_SEED_RESPONSE_OBSERVING.with(std::cell::Cell::get) {
                    if let ContinuationSourceCoordinate::EntryAbi {
                        source_owner,
                        source_abi_position,
                        ..
                    } = capture.source()
                    {
                        let source = compiler
                            .static_transition_plan
                            .emittable_units()?
                            .into_iter()
                            .find(|unit| unit.function() == source_owner)
                            .expect("response EntryAbi source has an emittable owner");
                        if source.header().parameters >= 2
                            && source_abi_position < source.header().parameters
                        {
                            let source_slot = source
                                .slots()
                                .get(source_abi_position as usize)
                                .expect("source parameter has a descriptor slot");
                            assert_eq!(source_slot.kind, AbiSlotKind::Parameter);
                            let word = match context_suffix.last().expect("just appended capture") {
                                LoweringOperand::Carried(word) => word.word,
                                _ => panic!("response K capture was not an emitted word"),
                            };
                            let cranelift_codegen::ir::ValueDef::Result(inst, _) =
                                builder.func.dfg.value_def(word)
                            else {
                                panic!("response capture word has no emitted Load");
                            };
                            let cranelift_codegen::ir::InstructionData::Load { offset, .. } =
                                builder.func.dfg.insts[inst]
                            else {
                                panic!("response capture word was not loaded from its frame");
                            };
                            let emitted_offset = i32::from(offset);
                            let response_slot = slots
                                .iter()
                                .zip(offsets)
                                .find(|(_, offset)| {
                                    i32::try_from(**offset).ok() == Some(emitted_offset)
                                })
                                .map(|(slot, _)| slot)
                                .expect("emitted response Load offset names a descriptor slot");
                            assert_eq!(response_slot.kind, AbiSlotKind::Parameter);
                            assert_eq!(
                                response_slot.ordinal,
                                capture.producer_abi_slot(),
                                "response capture must read by producer's index-keyed frame slot"
                            );
                            assert_eq!(
                                response_slot.carrier, source_slot.carrier,
                                "response M: emitter-loaded slot and source descriptor carrier disagree"
                            );
                            assert_eq!(
                                response_slot.ownership, source_slot.ownership,
                                "response M: emitter-loaded slot and source descriptor ownership disagree"
                            );
                            assert_eq!(
                                response_slot.storage_owner, source_slot.storage_owner,
                                "response M: emitter-loaded slot and source descriptor storage owner disagree"
                            );
                            assert_eq!(
                                response_slot.carrier,
                                AbiCarrier::ValueWord,
                                "response M: ValueWord referent affinity must be revisited with a new parameter carrier"
                            );
                            RT_SEED_RESPONSE_PAIRS.with(|pairs| {
                                pairs.borrow_mut().push((
                                    source_abi_position,
                                    response_slot.ordinal,
                                    emitted_offset,
                                ))
                            });
                        }
                    }
                }
            }
            for (ordinal, _, _) in emission.row.continuation_inputs() {
                context_suffix.push(
                    frame_inputs
                        .get(&StaticResponseFrameSource::Capture(*ordinal))
                        .cloned()
                        .ok_or_else(|| {
                            backend_module(
                                LoweringCause::PlannerInvariant,
                                "a response continuation input names an absent owner Capture slot"
                                    .to_string(),
                            )
                        })?,
                );
            }
            let mut context_calls = Vec::new();
            let mut early_returned = None;
            #[cfg(feature = "px8-ds-test-support")]
            if body_mutation == Some(StaticResponseOwnerBodyMutation::CallBeforeHostValidation) {
                let mut inputs = Vec::with_capacity(1 + context_suffix.len());
                // The call-before-validation mutation must reach the finished
                // body verifier, not be intercepted by the sole placeholder mint.
                inputs.push(LoweringOperand::Carried(CarriedBoundaryWord {
                    word: builder.ins().iconst(types::I64, 0),
                }));
                inputs.extend(context_suffix.iter().cloned());
                let (returned, call) = compiler.call_declared_unit_target(
                    &mut builder,
                    selected_context.clone(),
                    &inputs,
                    None,
                    #[cfg(test)]
                    None,
                )?;
                context_calls.push(call);
                early_returned = Some(returned);
            }

            let (response, host_validation_end) = lower_static_response_effect(
                compiler,
                &mut builder,
                &emission.row,
                &emission.effect,
                &emission.operation_arguments,
                &frame_inputs,
            )?;
            #[cfg(feature = "px8-ds-test-support")]
            let raw_host_result_escape =
                if body_mutation == Some(StaticResponseOwnerBodyMutation::RawHostResultEscape) {
                    let LoweringOperand::Specialized(value) = &response else {
                        unreachable!("the HostResult shape was checked above")
                    };
                    Some(compiler.transfer_unit_result_into_carrier(
                        &mut builder,
                        emission.row.effect_origin(),
                        value,
                    )?)
                } else {
                    None
                };

            let mut response_input = response;
            let mut response_is_current_host_result = true;
            #[cfg(feature = "px8-ds-test-support")]
            match body_mutation {
                Some(StaticResponseOwnerBodyMutation::ResponseWithOperation) => {
                    // The operation's inert ABI representation is an ordinary
                    // carried zero here. Passing the deferred marker instead
                    // would test the new sole-mint refusal, not this verifier.
                    response_input = LoweringOperand::Carried(CarriedBoundaryWord {
                        word: builder.ins().iconst(types::I64, 0),
                    });
                    response_is_current_host_result = false;
                }
                Some(StaticResponseOwnerBodyMutation::ResponseWithPriorResponse) => {
                    response_input = frame_inputs
                        .iter()
                        .find(|(source, _)| matches!(source, StaticResponseFrameSource::Capture(_)))
                        .map(|(_, operand)| operand.clone())
                        .ok_or_else(|| {
                            backend_module(
                                LoweringCause::PlannerInvariant,
                                "the prior-response substitution found no carried prior lane"
                                    .to_string(),
                            )
                        })?;
                    response_is_current_host_result = false;
                }
                Some(StaticResponseOwnerBodyMutation::ResponseWithApplicationEnvironment) => {
                    response_input = frame_inputs
                        .get(&StaticResponseFrameSource::Parameter(1))
                        .cloned()
                        .ok_or_else(|| {
                            backend_module(
                                LoweringCause::PlannerInvariant,
                                "the application-environment substitution found no app capture"
                                    .to_string(),
                            )
                        })?;
                    response_is_current_host_result = false;
                }
                _ => {}
            }

            let mut context_inputs = Vec::with_capacity(1 + context_suffix.len());
            context_inputs.push(response_input);
            context_inputs.extend(context_suffix.iter().cloned());
            #[cfg(feature = "px8-ds-test-support")]
            if body_mutation == Some(StaticResponseOwnerBodyMutation::LoadPlaceholderIntoK) {
                context_inputs[0] = LoweringOperand::Carried(CarriedBoundaryWord {
                    word: builder.ins().load(
                        types::I64,
                        MemFlags::trusted(),
                        frame,
                        placeholder_offset,
                    ),
                });
            }
            let result_offset = i32::try_from(result_offset).map_err(|_| {
                backend_module(
                    LoweringCause::Capacity,
                    "response-owner result slot offset exceeds range".to_string(),
                )
            })?;
            let mut result_store = None;
            #[cfg(feature = "px8-ds-test-support")]
            if body_mutation == Some(StaticResponseOwnerBodyMutation::CallAfterAnswerCollapse) {
                let collapsed = builder.ins().iconst(types::I64, 0);
                result_store = Some(builder.ins().store(
                    MemFlags::trusted(),
                    collapsed,
                    frame,
                    result_offset,
                ));
            }
            let returned_operand = if let Some(returned) = early_returned {
                returned
            } else {
                #[cfg(feature = "px8-ds-test-support")]
                if body_mutation == Some(StaticResponseOwnerBodyMutation::OmitKCall) {
                    LoweringOperand::Carried(CarriedBoundaryWord {
                        word: builder.ins().iconst(types::I64, 0),
                    })
                } else {
                    #[cfg(feature = "px8-ds-test-support")]
                    if body_mutation
                        == Some(StaticResponseOwnerBodyMutation::BypassTrapBeforeResult)
                    {
                        super::calls::set_trap_caller_protocol_mutation(
                            super::calls::TrapCallerProtocolMutation::LoadResultBeforeTrapGuard,
                        );
                    }
                    let result = compiler.call_declared_unit_target(
                        &mut builder,
                        selected_context.clone(),
                        &context_inputs,
                        None,
                        #[cfg(test)]
                        None,
                    );
                    #[cfg(feature = "px8-ds-test-support")]
                    if body_mutation
                        == Some(StaticResponseOwnerBodyMutation::BypassTrapBeforeResult)
                    {
                        super::calls::set_trap_caller_protocol_mutation(
                            super::calls::TrapCallerProtocolMutation::Exact,
                        );
                    }
                    let (returned, call) = result?;
                    context_calls.push(call);
                    #[cfg(feature = "px8-ds-test-support")]
                    if body_mutation == Some(StaticResponseOwnerBodyMutation::DuplicateKCall) {
                        let (_duplicate_result, duplicate_call) = compiler
                            .call_declared_unit_target(
                                &mut builder,
                                selected_context.clone(),
                                &context_inputs,
                                None,
                                #[cfg(test)]
                                None,
                            )?;
                        context_calls.push(duplicate_call);
                    }
                    returned
                }
                #[cfg(not(feature = "px8-ds-test-support"))]
                {
                    let (returned, call) = compiler.call_declared_unit_target(
                        &mut builder,
                        selected_context.clone(),
                        &context_inputs,
                        None,
                        #[cfg(test)]
                        None,
                    )?;
                    context_calls.push(call);
                    returned
                }
            };
            let returned = match returned_operand {
                LoweringOperand::Carried(word) => word,
                // The pending-Vis response-owner Result frame carries K.
                LoweringOperand::Residual(residual) => {
                    #[cfg(any(test, feature = "px8-ds-test-support"))]
                    record_residual_abi_decode();
                    compiler.decode_residual_child(&mut builder, residual)?
                }
                LoweringOperand::Specialized(_) => {
                    return Err(backend_module(LoweringCause::PlannerInvariant,
                        "a response K context returned a specialized template instead of its Trap-checked runtime Result"
                            .to_string(),
                    ));
                }
            };
            // A bypass alone is inconclusive on a source whose real owner
            // returns Ret. This test-only mutation supplies a Vis carrier of
            // the source constructor's exact identity and field count. A
            // planned returned member is driven by the pending-Vis loop; only
            // the separate non-member ingress reaches the C2 backstop.
            #[cfg(feature = "px8-ds-test-support")]
            let bypass_ret_validation = body_mutation
                == Some(StaticResponseOwnerBodyMutation::BypassRetValidationAndReturnVis);
            #[cfg(feature = "px8-ds-test-support")]
            let returned = if bypass_ret_validation {
                let vis_origin = emission.row.vis_origin();
                let vis_expr = compiler.retained_body_occurrence(vis_origin)?.expr.clone();
                let RuntimeExpr::Construct { constructor, args } = vis_expr else {
                    return Err(backend_module(
                        LoweringCause::PlannerInvariant,
                        "the test-only owner bypass lost its source Vis constructor".to_string(),
                    ));
                };
                if !constructor.ends_with("::ITree::Vis") || args.len() != 2 {
                    return Err(backend_module(
                        LoweringCause::PlannerInvariant,
                        "the test-only owner bypass needs a binary Vis constructor".to_string(),
                    ));
                }
                compiler.transfer_constructor_operands(
                    &mut builder,
                    vis_origin,
                    &constructor,
                    &[
                        LoweringOperand::Carried(returned),
                        LoweringOperand::Carried(returned),
                    ],
                    None,
                )?
            } else {
                returned
            };
            if compiler
                .response_bracket_exits
                .contains_key(&emission.row.id())
            {
                compiler.bracket_ledger.close_owner(emission.row.id())?;
            }
            let pending_vis_owner = match compiler
                .response_owner_settlements
                .get(&emission.row.id())
            {
                Some(ResponseOwnerSettlement::Protocol) if !emission.successors.is_empty() => true,
                Some(ResponseOwnerSettlement::RetOnly) if emission.successors.is_empty() => false,
                // Baseline emission: the former pending-Vis protocol dropped
                // excluded owners, so these took the Ret-only arm. The
                // successor determines whether the handler-owned drive settles
                // a returned Vis or this route traps.
                Some(ResponseOwnerSettlement::Excluded { .. })
                    if emission.successors.is_empty() =>
                {
                    false
                }
                other => {
                    return Err(backend_module(
                        LoweringCause::PlannerInvariant,
                        format!(
                            "response owner {:?} reached emission with settlement {other:?} and {} successors",
                            emission.row.id(),
                            emission.successors.len(),
                        ),
                    ));
                }
            };
            if let Some(obligation) = compiler
                .function_local
                .pending_call_result_obligations
                .iter_mut()
                .find(|obligation| obligation.result_word == returned.word)
            {
                obligation.realization_required = true;
                // The exact tag and arity guards below refine the realized
                // word; a declared K target is not a constructor-identity proof.
                obligation.identity = None;
            }
            let exact_ret_identity = emission.row.k_ret_identity();
            let exact_ret_abi_word = exact_ret_identity.tag_abi_word()?;
            #[cfg(feature = "px8-ds-test-support")]
            let ret_abi_word = if body_mutation == Some(StaticResponseOwnerBodyMutation::VaryRet) {
                exact_ret_abi_word.checked_add(1).ok_or_else(|| {
                    backend_module(
                        LoweringCause::Capacity,
                        "the response Ret mutation exhausted the ABI word".to_string(),
                    )
                })?
            } else {
                exact_ret_abi_word
            };
            #[cfg(not(feature = "px8-ds-test-support"))]
            let ret_abi_word = exact_ret_abi_word;
            if pending_vis_owner {
                let pending = compiler.function_local.pending_vis_frame.ok_or_else(|| {
                    backend_module(
                        LoweringCause::PlannerInvariant,
                        "a pending-Vis owner has no caller-owned record".to_string(),
                    )
                })?;
                #[cfg(feature = "px8-ds-test-support")]
                let nonmember_mode = claim_pending_vis_nonmember_mutation(bypass_ret_validation);
                let (force_unknown_member, bypass_unknown_to_c2) = {
                    #[cfg(feature = "px8-ds-test-support")]
                    {
                        (
                            nonmember_mode.is_some(),
                            nonmember_mode == Some(PendingVisNonmemberMutation::BypassToC2),
                        )
                    }
                    #[cfg(not(feature = "px8-ds-test-support"))]
                    {
                        (false, false)
                    }
                };
                pending_finished = Some(emit_pending_vis_owner_loop(
                    compiler,
                    &mut builder,
                    frame,
                    pending.region,
                    &emission.row,
                    ret_abi_word,
                    &context_calls,
                    expected_context_target,
                    host_validation_end,
                    response_is_current_host_result,
                    returned,
                    &emission.successors,
                    &successor_targets,
                    &successor_contexts,
                    result_offset,
                    placeholder_offset,
                    #[cfg(feature = "px8-ds-test-support")]
                    raw_host_result_escape,
                    force_unknown_member,
                    bypass_unknown_to_c2,
                )?);
            } else {
                let ret_tag = compiler.emit_carrier_tag(&mut builder, returned)?;
                let expected_ret = i64::try_from(ret_abi_word).map_err(|_| {
                    backend_module(
                        LoweringCause::Capacity,
                        "response Ret identity exceeds the runtime tag word".to_string(),
                    )
                })?;
                #[cfg(feature = "px8-ds-test-support")]
                if !bypass_ret_validation {
                    Lowering::require_i64(&mut builder, ret_tag, expected_ret);
                }
                #[cfg(not(feature = "px8-ds-test-support"))]
                Lowering::require_i64(&mut builder, ret_tag, expected_ret);
                let ret_fields = compiler.emit_carrier_field_count(&mut builder, returned)?;
                #[cfg(feature = "px8-ds-test-support")]
                if !bypass_ret_validation {
                    Lowering::require_i64(&mut builder, ret_fields, 1);
                }
                #[cfg(not(feature = "px8-ds-test-support"))]
                Lowering::require_i64(&mut builder, ret_fields, 1);
                let ret_validation_end = builder
                    .func
                    .layout
                    .blocks()
                    .filter_map(|block| builder.func.layout.last_inst(block))
                    .last()
                    .ok_or_else(|| {
                        backend_module(
                            LoweringCause::FinishedCode,
                            "response Ret validation emitted no instruction".to_string(),
                        )
                    })?;
                let application = CheckedIhApplicationResult::from_declared_call(
                    LoweringOperand::Carried(returned),
                )?;
                let result_store = match result_store {
                    Some(store) => store,
                    None => {
                        #[cfg(feature = "px8-ds-test-support")]
                        let result_word =
                            raw_host_result_escape.map_or(application.word.word, |word| word.word);
                        #[cfg(not(feature = "px8-ds-test-support"))]
                        let result_word = application.word.word;
                        builder
                            .ins()
                            .store(MemFlags::trusted(), result_word, frame, result_offset)
                    }
                };
                let zero = builder.ins().iconst(types::I64, 0);
                let return_inst = builder.ins().return_(&[zero]);
                compiler.record_checked_frame_terminal(&builder, FrameTerminalKind::Normal)?;
                finished_body = Some(StaticResponseFinishedBody {
                    input_frame: frame,
                    placeholder_offset,
                    context_calls,
                    host_validation_end,
                    ret_validation_end,
                    result_store,
                    returned_word: application.word.word,
                    return_inst,
                    frame,
                    result_offset,
                    trap_offset: i32::try_from(trap_offset).map_err(|_| {
                        backend_module(
                            LoweringCause::Capacity,
                            "response-owner trap slot offset exceeds range".to_string(),
                        )
                    })?,
                    response_is_current_host_result,
                    ret_abi_word,
                });
            }
            compiler.validate_response_join_plan_consumption(&required)?;
            builder.seal_all_blocks();
            builder.finalize();
        }
        ambient.release(compiler);
        frame_scope.close(compiler, &func)?;
        token_scope.close(compiler, &func)?;
        compiler.record_finished_grafted_spine_function(&func, bundle)?;
        compiler.validate_materialized_dead_join_cfg_for(&required, &func)?;
        verify_cranelift_function(&func, module.isa())?;
        // The test-only Vis ingress bypasses finished-body verification as a
        // layer, whichever owner arm is emitted. No production build has this
        // predicate; every owner still runs its exact finished-body verifier.
        let verify_finished_body = || -> Result<(), CraneliftBackendError> {
            if let Some(pending) = pending_finished.as_ref() {
                let region = compiler
                    .pending_vis_record_protocol
                    .as_ref()
                    .ok_or_else(|| {
                        backend_module(
                            LoweringCause::PlannerInvariant,
                            "the pending-Vis verifier lost the closed plan".to_string(),
                        )
                    })?
                    .frame_region(
                        PendingVisFrameOwner::ResponseOwner(emission.row.id()),
                        emission.owner.header().frame_bytes,
                    )?
                    .ok_or_else(|| {
                        backend_module(
                            LoweringCause::PlannerInvariant,
                            "a pending-Vis owner has no planned record tail".to_string(),
                        )
                    })?;
                verify_pending_vis_finished_body(
                    &func,
                    pending,
                    &emission.row,
                    &emission.successors,
                    region,
                )?;
            } else {
                let ret_only = finished_body.as_ref().ok_or_else(|| {
                    backend_module(
                        LoweringCause::PlannerInvariant,
                        "a Ret-only response owner has no finished-body facts".to_string(),
                    )
                })?;
                verify_static_response_finished_body(
                    &func,
                    expected_context_target,
                    emission.row.k_ret_identity().tag_abi_word()?,
                    helpers.boundary_value_abi.tag,
                    helpers.boundary_value_abi.field_count,
                    compiler.function_local.boundary_arena.ok_or_else(|| {
                        backend_module(
                            LoweringCause::FinishedCode,
                            "a response-owner finished proof has no boundary arena".to_string(),
                        )
                    })?,
                    ret_only,
                    &compiler.function_local.pending_call_result_obligations,
                )?;
            }
            Ok(())
        };
        #[cfg(feature = "px8-ds-test-support")]
        if body_mutation != Some(StaticResponseOwnerBodyMutation::BypassRetValidationAndReturnVis) {
            verify_finished_body()?;
        }
        #[cfg(not(feature = "px8-ds-test-support"))]
        verify_finished_body()?;
        compiler.commit_aggregate_events()?;
        #[cfg(feature = "px8-ds-test-support")]
        if body_mutation == Some(StaticResponseOwnerBodyMutation::OmitOwnerDefinition) {
            continue;
        }
        let publication = if let Some(ret) = finished_body.as_ref() {
            FunctionResultPublication {
                frame: ret.frame,
                result_offset: ret.result_offset,
                trap_offset: ret.trap_offset,
                result_store: ret.result_store,
                returned_word: ret.returned_word,
                return_inst: ret.return_inst,
            }
        } else {
            let pending = pending_finished.as_ref().ok_or_else(|| {
                backend_module(
                    LoweringCause::FinishedCode,
                    "a response owner has no finished Result publication".to_string(),
                )
            })?;
            FunctionResultPublication {
                frame: pending.input_frame,
                result_offset: pending.result_offset,
                trap_offset: i32::try_from(trap_offset).map_err(|_| {
                    backend_module(
                        LoweringCause::Capacity,
                        "pending-Vis response-owner trap offset exceeds range".to_string(),
                    )
                })?,
                result_store: pending.result_store,
                returned_word: pending.returned_word,
                return_inst: pending.return_inst,
            }
        };
        // The bypass hook admits this owner past the entire finished-body
        // layer, which includes the staged Result proof. It publishes no
        // proven Result, so nothing can be realized or certified through it.
        #[cfg(feature = "px8-ds-test-support")]
        let publication = (body_mutation
            != Some(StaticResponseOwnerBodyMutation::BypassRetValidationAndReturnVis))
        .then_some(publication);
        #[cfg(not(feature = "px8-ds-test-support"))]
        let publication = Some(publication);
        staged.push(stage_result_body(
            compiler,
            ExistingResultUnitIdentity::Response(emission.owner.id()),
            id,
            func,
            publication,
            None,
            Some(emission.row.k_ret_identity()),
        ));
    }
    Ok(staged)
}

/// **`RT-CONTSPEC-ACTIVATE` `D2` — define each declared continuation target
/// from its own projected contract.**
///
/// Operands come from the **descriptor**: each `Parameter` and `Capture` slot
/// is loaded at the offset `slot_offsets` assigns it. ⛔ Function parameter 0
/// is not the payload, and the `Result` slot is never read -- it is
/// caller-initialized, and this body only writes it.
///
/// The partition is the ruled one: `Parameter` operands are the ordinary
/// envelope (nonrecursive producer fields, then selected worker captures in
/// capture-ordinal order), and `Capture` operands are the continuation inputs
/// by ordinal. The worker binding is built by the **existing** static-worker
/// constructor, and the semantic environment is the sole
/// `LoweringEnvironmentBinding` authority -- no parallel operand map and no
/// worker-body de Bruijn table.
pub(super) fn stage_continuation_bodies<M: Module>(
    module: &mut M,
    compiler: &mut Lowering<'_>,
    helpers: ArtifactHelpers<'_>,
    bundle: &UnitBundle,
) -> Result<Vec<StagedResultBody>, CraneliftBackendError> {
    // Own every projected fact BEFORE the loop: the projection borrows the
    // plan, and the definition below needs the compiler mutably.
    struct OwnedContinuationEmission {
        id: ContinuationSpecializationId,
        slots: Vec<AbiSlot>,
        offsets: Vec<u32>,
        envelope: Vec<ContinuationOrdinaryEnvelopeRole>,
        inputs: Vec<ContinuationInputView>,
        continuation_origin: StaticOriginId,
        producer_alternative: u32,
        recursive_position: u32,
        worker_closure_origin: StaticOriginId,
        worker_body_origin: StaticOriginId,
        worker_declared_arity: u32,
        worker_capture_count: usize,
        header_parameters: u32,
        header_captures: u32,
        frame_bytes: u32,
        /// `D8o` — the owner of the source body this specialization lowers: the
        /// continuation's own consumer. ⛔ Carried from the planner view rather
        /// than derived here; it is an existing planner fact reaching the site
        /// that needs it, not a new authority.
        consumer_owner: PredeclaredFunctionId,
    }
    // `RT-WORKER-BIND` `D4` exposes its local declaration operation for a
    // separately emitted caller; a continuation function is exactly that, so
    // it declares its own worker refs rather than borrowing another's.
    let worker_targets = resolve_worker_targets(&compiler.static_transition_plan, bundle)?;
    // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the ORDINARY residual targets
    // `O_t`, and the fusion-local complement recorded as this pass's own
    // omission.
    //
    // **The omission is recorded HERE, in the pass that would otherwise have
    // emitted the body**, and the closeout compares it with `F_t`. A statement
    // made elsewhere about what this loop does would agree with itself; this is
    // the loop's own decision, and if it ever stopped omitting, the recorded set
    // would shrink and the range equality would say so.
    let ordinary_targets = compiler
        .static_transition_plan
        .ordinary_continuation_targets()?;
    for omitted in compiler
        .static_transition_plan
        .continuation_units()?
        .iter()
        .map(|unit| unit.id())
        .filter(|id| !ordinary_targets.contains(id))
        .collect::<Vec<_>>()
    {
        compiler
            .fusion_compositions
            .as_mut()
            .ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "a fusion-local continuation target was omitted from the definition pass \
                     with no composition ledger open to record it; the omission would then be \
                     invisible to the range equality that is the only thing requiring it"
                        .to_string(),
                )
            })?
            .record_definition_omitted(omitted);
    }
    let emissions = compiler
        .static_transition_plan
        .continuation_units()?
        .into_iter()
        .filter(|unit| ordinary_targets.contains(&unit.id()))
        .map(|unit| {
            let (offsets, frame_bytes) = unit.slot_offsets()?;
            Ok(OwnedContinuationEmission {
                id: unit.id(),
                slots: unit.slots().to_vec(),
                offsets,
                envelope: unit.ordinary_envelope()?,
                inputs: unit.continuation_inputs()?,
                continuation_origin: unit.continuation_origin(),
                producer_alternative: unit.producer_alternative(),
                recursive_position: unit.recursive_position(),
                worker_closure_origin: unit.worker_closure_origin(),
                worker_body_origin: unit.worker_body_origin(),
                worker_declared_arity: unit.worker_declared_arity(),
                worker_capture_count: unit.worker_capture_count(),
                header_parameters: unit.header().parameters,
                header_captures: unit.header().captures,
                frame_bytes,
                consumer_owner: unit.consumer_owner(),
            })
        })
        .collect::<Result<Vec<_>, CraneliftBackendError>>()?;

    // Shape index for the `D7` definition-binding control: declared arity and
    // capture count per specialization, which is the only same-shaped
    // definition. Built BEFORE the loop because the loop consumes `emissions`.
    //
    // ⛔ Over the CALLABLE population, not over `emissions`. Measured, not
    // reasoned: this fixture plans exactly one continuation specialization, so
    // an index over specializations has a single entry and the control refuses
    // for want of a partner that was never going to be there. The two
    // same-shaped things are the worker BODIES, which is what `AC-9` means by
    // "a distinct same-shaped target".
    #[cfg(test)]
    let d7_callable_index: Vec<((usize, usize), StaticOriginId)> = compiler
        .static_transition_plan
        .emittable_units()?
        .iter()
        .map(|emittable| {
            (
                (
                    emittable
                        .slots()
                        .iter()
                        .filter(|slot| slot.kind == AbiSlotKind::Parameter)
                        .count(),
                    emittable
                        .slots()
                        .iter()
                        .filter(|slot| slot.kind == AbiSlotKind::Capture)
                        .count(),
                ),
                emittable.body_occurrence(),
            )
        })
        .collect();

    let mut staged = Vec::new();
    #[cfg_attr(
        not(test),
        expect(unused_mut, reason = "only the D7 control mutates it")
    )]
    for mut unit in emissions {
        // Resolve the EXACT target first; the control below perturbs only what
        // the definition is handed.
        let exact_id = bundle.continuation(unit.id).ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "a planned continuation specialization was never forward-declared".to_string(),
            )
        })?;
        let id = exact_id;

        // `RT-CONTSPEC-WITNESS` `D7`/`AC-9` — bind a DISTINCT same-shaped body
        // under this exact declared `FuncId`.
        //
        // ⛔ `id` is `exact_id` and stays that way: the declared function, the
        // specialization id, the causal token, the header, slots, offsets,
        // inputs, owner and the emitted call are all untouched, and
        // `verify_emitted_continuation_calls` remains enabled. The ONLY thing
        // that moves is which body this declared function executes -- which is
        // the residual the finished-CLIF equality gate explicitly cannot see.
        #[cfg(test)]
        if crate::cranelift_backend::lowering::CONTINUATION_EMISSION_MUTATION
            .with(std::cell::Cell::get)
            == ContinuationEmissionMutation::SubstituteContinuationBodyDefinition
        {
            let exact_shape = d7_callable_index
                .iter()
                .find(|(_, origin)| *origin == unit.worker_body_origin)
                .map(|(shape, _)| *shape)
                .ok_or_else(|| {
                    backend_module(
                        LoweringCause::PlannerInvariant,
                        "the D7 definition-binding control could not read the exact worker \
                         body's declared arity and capture count; a control that cannot \
                         establish the shape it matches on must refuse rather than bind an \
                         arbitrary body"
                            .to_string(),
                    )
                })?;
            let substitute = d7_callable_index
                .iter()
                .find(|(shape, origin)| *shape == exact_shape && *origin != unit.worker_body_origin)
                .map(|(_, origin)| *origin)
                .ok_or_else(|| {
                    backend_module(
                        LoweringCause::PlannerInvariant,
                        "the D7 definition-binding control found no distinct same-shaped \
                         callable body to bind under this declared function; that is a \
                         missing fixture precondition, not a discharge"
                            .to_string(),
                    )
                })?;
            unit.worker_body_origin = substitute;
            crate::cranelift_backend::lowering::record_d7_definition_binding_substitution();
        }

        let offsets = unit.offsets.as_slice();
        let envelope = &unit.envelope;
        let inputs = &unit.inputs;
        let slots = unit.slots.as_slice();
        if slots.len() != offsets.len() {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "a continuation slot run disagrees with its own offset walk".to_string(),
            ));
        }

        // Reject BEFORE definition on partition incompleteness: the ordinary
        // envelope must cover every Parameter slot, and the continuation
        // inputs must cover every Capture slot densely by ordinal.
        let parameter_slots: Vec<_> = slots
            .iter()
            .zip(offsets)
            .filter(|(slot, _)| slot.kind == AbiSlotKind::Parameter)
            .collect();
        let capture_slots: Vec<_> = slots
            .iter()
            .zip(offsets)
            .filter(|(slot, _)| slot.kind == AbiSlotKind::Capture)
            .collect();
        if parameter_slots.len() != envelope.len() {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "the ruled ordinary envelope does not cover the Parameter slot run".to_string(),
            ));
        }
        if capture_slots.len() != inputs.len() {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "the projected continuation inputs do not cover the Capture slot run".to_string(),
            ));
        }
        for (position, input) in inputs.iter().enumerate() {
            if input.ordinal as usize != position {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    "continuation inputs are not dense in ordinal order".to_string(),
                ));
            }
        }
        // Provenance: every worker capture role must name the worker this key
        // selected, so an envelope built against another closure rejects.
        for role in envelope.iter() {
            if let ContinuationOrdinaryEnvelopeRole::WorkerCapture { closure_origin, .. } = role {
                if *closure_origin != unit.worker_closure_origin {
                    return Err(backend_module(
                        LoweringCause::PlannerInvariant,
                        "an ordinary-envelope worker capture names a different closure than the \
                         selected worker"
                            .to_string(),
                    ));
                }
            }
        }

        compiler.open_aggregate_events(id)?;
        let sig = unit_signature(module);
        let mut func = Function::with_name_signature(UserFuncName::user(0, id.as_u32()), sig);
        // Set by the retarget below; `None` means this specialization calls the
        // raw worker unit directly, which is every pre-`D5a` case.
        let mut retargeted_worker_body: Option<StaticOriginId> = None;
        let result_offset = slots
            .iter()
            .zip(offsets)
            .find(|(slot, _)| slot.kind == AbiSlotKind::Result)
            .map(|(_, offset)| *offset)
            .ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "continuation frame declares no result slot".to_string(),
                )
            })?;
        let trap_offset = slots
            .iter()
            .zip(offsets)
            .find(|(slot, _)| slot.kind == AbiSlotKind::Trap)
            .map(|(_, offset)| *offset)
            .ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "continuation frame declares no trap slot".to_string(),
                )
            })?;

        let mut function_local = helpers.declare_in_func(module, &mut func, None);
        function_local.grafted_spine_scope = Some(GraftedSpineFunctionScope::Continuation(unit.id));
        // ONE lawful declaration per continuation `Function`, retained whole
        // and seated in BOTH existing roles. The worker constructor validates
        // through `unit_calls`; the later callee-only consumer resolves
        // through `worker_calls`. Seating only the second is what made the
        // constructor refuse -- the objects are the same, the roles are not.
        //
        // Declared here, into THIS function: no `FuncRef` crosses a function.
        let declared_workers = worker_targets.declare_in_func(module, &mut func);
        function_local.unit_calls = declared_workers.clone();
        // `D6b` -- the RAW route's table, captured before the retarget below
        // rewrites `worker_calls`. This is the only point at which the raw
        // callee for a retargeted body is still in hand.
        function_local.raw_worker_calls = declared_workers.clone();
        function_local.worker_templates = worker_targets.templates().clone();
        function_local.context_calls = declare_context_calls_in_func(
            module,
            &mut func,
            &compiler.static_transition_plan,
            bundle,
        )?;
        // `RT-DECL-CLOSURE-PORT` `D5a` -- THE RETARGET.
        //
        // If this specialization's worker body has a generated execution
        // context, the worker call resolves to that context instead of to the
        // raw unit. ⛔ Only `worker_calls` moves: `unit_calls` above keeps the
        // raw target, because the static-worker CONSTRUCTOR validates against
        // the raw body's own contract and this retarget does not change what
        // that body is.
        //
        // ⭐ Why this is what makes the whole thing work: the continuation
        // inputs live in THIS frame's Capture slots. The raw unit's ABI has
        // nowhere to put them, so calling the raw unit drops them before the
        // nested producer is reached -- which is exactly the measured defect.
        // The context's ABI has a capture run for them, so the call carries
        // them across the checked-IH worker execution.
        let mut worker_calls = declared_workers;
        // `D5a` checkpoint 4 step 3 -- the binding's three reaching mutations.
        //
        // ⛔ `Suppress` and `Transplant` perturb WHICH context the retarget is
        // handed; the exact lookup itself is untouched, so a refusal downstream
        // is attributable to the binding and not to a rewritten resolver.
        // `Transplant` declines when this unit has no foreign context to be
        // given -- it bumps the application counter when it does fire, so a
        // control can require the perturbation actually reached the seat rather
        // than reading a green as a defence.
        #[cfg(test)]
        let resolved_context = match crate::cranelift_backend::lowering::d5a_route_mutation() {
            crate::cranelift_backend::lowering::D5aRouteMutation::SuppressContextBinding => {
                crate::cranelift_backend::lowering::record_d5a_route_application();
                None
            }
            crate::cranelift_backend::lowering::D5aRouteMutation::TransplantContextBinding => {
                let foreign = compiler
                    .static_transition_plan
                    .continuation_contexts()?
                    .into_iter()
                    .find(|context| context.enclosing_specialization() != unit.id);
                match foreign {
                    Some(context) => {
                        crate::cranelift_backend::lowering::record_d5a_route_application();
                        Some(context)
                    }
                    None => compiler
                        .static_transition_plan
                        .continuation_context_for(unit.id, unit.worker_body_origin)?,
                }
            }
            _ => compiler
                .static_transition_plan
                .continuation_context_for(unit.id, unit.worker_body_origin)?,
        };
        #[cfg(not(test))]
        let resolved_context = compiler
            .static_transition_plan
            .continuation_context_for(unit.id, unit.worker_body_origin)?;
        if let Some(context) = resolved_context {
            // `D5a` checkpoint 4 step 3 -- THE TRANSPLANT STOP.
            //
            // ⭐⭐ Added because a transplant was **measured to compile**. The
            // resolved context used to be trusted wholesale and the record
            // below took its `origin` from `unit.worker_body_origin` -- the
            // asking unit's own value. `call_static_worker`'s
            // `target.origin != worker.body_origin` check therefore compared
            // that value with itself on this path and could not see a foreign
            // context at all. Handed one context in place of another, lowering
            // emitted a call that type-checked (the capture suffix made the
            // operand run agree) and transferred to a function executing a
            // DIFFERENT body.
            //
            // ⛔ Production never reaches that state: `continuation_context_for`
            // is keyed by `(enclosing, worker_body)` and is the only producer.
            // ⇒ But "unreachable by construction" was carrying the whole
            // guarantee here, with no check able to observe a violation, so the
            // ruling's transplanted-binding stop had nothing to name. These two
            // comparisons cost nothing and turn it into a fact the code checks.
            if context.enclosing_specialization() != unit.id {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    format!(
                        "the retarget resolved generated context {:?}, whose enclosing specialization \
                     is {:?} and not the {:?} now being defined; a context executes on behalf of \
                     the one identity that owns it, so this call would transfer another \
                     specialization's captures across this one's worker execution",
                        context.id(),
                        context.enclosing_specialization(),
                        unit.id,
                    ),
                ));
            }
            if context.worker_body_origin() != unit.worker_body_origin {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    format!(
                        "the retarget resolved a generated context executing body {:?}, but this \
                     specialization selected worker body {:?}",
                        context.worker_body_origin(),
                        unit.worker_body_origin,
                    ),
                ));
            }
            let target = bundle.context(context.id()).ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "a planned generated context was never forward-declared".to_string(),
                )
            })?;
            let (context_offsets, _frame_bytes) = context.slot_offsets()?;
            worker_calls.insert(
                unit.worker_body_origin,
                DeclaredUnitCall {
                    function: module.declare_func_in_func(target, &mut func),
                    frame_owner: Some(PendingVisFrameOwner::Context(context.id())),
                    // The context EXECUTES that body, so the origin it answers
                    // for is unchanged. ⛔ Read from the CONTEXT, not from the
                    // asking unit: taking it from `unit` is what made
                    // `call_static_worker`'s origin check self-referential
                    // here. The equality above is what lets both readings agree.
                    origin: context.worker_body_origin(),
                    call_site_origin: context.worker_body_origin(),
                    header: context.header(),
                    slots: context.slots().to_vec(),
                    offsets: context_offsets,
                    result_contract: compiler
                        .static_transition_plan
                        .checked_ih_generated_context_result_contract(context.id())?
                        .map(DeclaredResultContract::new),
                },
            );
            retargeted_worker_body = Some(unit.worker_body_origin);
        }
        // ⛔ Checked AFTER the retarget, not before it. `D5a` checkpoint 4
        // step 2 removes a fully retargeted raw worker from the emitted
        // `Function` population, so demanding an *emittable-unit* target here
        // would reject exactly the case the retarget exists to serve. What this
        // specialization actually needs is a declared callee for its worker
        // body -- raw or generated -- and that is what is required.
        if !worker_calls.contains_key(&unit.worker_body_origin) {
            return Err(backend_module(LoweringCause::PlannerInvariant,
                "the selected continuation worker body has no declared callee in this function,                  neither an emittable raw unit nor a generated execution context"
                    .to_string(),
            ));
        }
        function_local.worker_calls = worker_calls;
        // `D8n` — this generated Function's own checked-frame consumption
        // transaction, spanning the specialization body exactly. ⛔ Opened before the builder and
        // closed after it, so every branch scope inside nests within it.
        let frame_scope = CheckedFrameFunctionScope::open(compiler)?;
        let token_scope = CheckedTokenFunctionScope::open(compiler)?;
        // ⭐⭐ `D8o` — THE BINDING THIS PASS NEVER HAD. A specialization body
        // used to run with whatever the previously defined body left in both
        // ambient fields. The owner is exactly the planner's identity for this
        // specialization; the unit is the owner of the source body it lowers,
        // which is the continuation's own consumer.
        let ambient = AmbientBodyAuthority::bind(
            compiler,
            ContinuationEmissionOwner::Specialization(unit.id),
            unit.consumer_owner,
        );
        // `D8o` — the exact body key, supplied by the pass that knows it.
        #[cfg(test)]
        crate::cranelift_backend::lowering::record_d8o_body_key(
            compiler.defining_function_id,
            crate::cranelift_backend::lowering::D8oBodyKey::ContinuationSpecialization(unit.id),
        );
        let mut func_ctx = FunctionBuilderContext::new();
        let publication;
        {
            let mut builder = FunctionBuilder::new(&mut func, &mut func_ctx);
            let entry = builder.create_block();
            builder.append_block_params_for_function_params(entry);
            builder.switch_to_block(entry);
            let envelope_pointer = builder.block_params(entry)[0];
            let frame = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                envelope_pointer,
                crate::activation_services::UNIT_CALL_FRAME_SLOTS,
            );
            // The same per-function activation-services preamble every
            // generated unit body binds, from the same envelope and services
            // record. Omitting it is what left this function with no boundary
            // arena; substituting the native-`Int` arena for the boundary one
            // is the defect the `S6`/`D6` ruling exists to remove, so the
            // record is read for both rather than one standing in for the
            // other.
            let host_dispatch_context = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                envelope_pointer,
                crate::activation_services::UNIT_CALL_FRAME_HOST_DISPATCH_CONTEXT,
            );
            let services = builder.block_params(entry)[1];
            let native_int_arena = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                services,
                crate::activation_services::SERVICES_NATIVE_INT_ARENA,
            );
            Lowering::require_nonzero(&mut builder, native_int_arena);
            let boundary_arena = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                services,
                crate::activation_services::SERVICES_BOUNDARY_ARENA,
            );
            Lowering::require_nonzero(&mut builder, boundary_arena);
            function_local.host_dispatch_context = Some(host_dispatch_context);
            function_local.native_int_arena = Some(native_int_arena);
            function_local.boundary_arena = Some(boundary_arena);
            function_local.services_pointer = Some(services);
            function_local.bind_unit_trap_frame(
                frame,
                i32::try_from(trap_offset).map_err(|_| {
                    backend_module(
                        LoweringCause::Capacity,
                        "continuation trap slot offset exceeds range".to_string(),
                    )
                })?,
            )?;
            function_local.bind_pending_vis_frame(
                compiler.pending_vis_record_protocol.as_ref(),
                PendingVisFrameOwner::Continuation(unit.id),
                frame,
                unit.frame_bytes,
            )?;
            compiler.function_local = function_local;

            // Descriptor-only loads. Each operand is read from the slot the
            // descriptor assigns it, and from nowhere else.
            let load_at = |builder: &mut FunctionBuilder<'_>, offset: u32| {
                let offset = i32::try_from(offset).map_err(|_| {
                    backend_module(
                        LoweringCause::Capacity,
                        "continuation slot offset exceeds addressable range".to_string(),
                    )
                })?;
                Ok::<_, CraneliftBackendError>(LoweringOperand::Carried(CarriedBoundaryWord {
                    word: builder
                        .ins()
                        .load(types::I64, MemFlags::trusted(), frame, offset),
                }))
            };

            let mut ordinary = Vec::with_capacity(parameter_slots.len());
            for (_, offset) in &parameter_slots {
                ordinary.push(load_at(&mut builder, **offset)?);
            }
            let mut carried_inputs = Vec::with_capacity(capture_slots.len());
            for (_, offset) in &capture_slots {
                carried_inputs.push(load_at(&mut builder, **offset)?);
            }

            // `D5a` -- the operand suffix the retargeted worker call appends.
            //
            // These are THIS frame's own continuation inputs, in ordinal order,
            // which is exactly the capture run the generated context declares.
            // ⛔ Stashed rather than threaded through `construct_static_worker_
            // binding`: the worker binding is the raw body's contract and adding
            // a context's captures to it would make the raw contract vary with
            // its caller.
            if let Some(worker_body_origin) = retargeted_worker_body {
                compiler.function_local.generated_context_captures =
                    Some(GeneratedContextCaptures {
                        worker_body_origin,
                        operands: carried_inputs.clone(),
                    });
            }

            // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the selected case body,
            // through the ONE authority both realizations share.
            //
            // The operands are this frame's own: `ordinary` from the
            // `Parameter` slots and `carried_inputs` from the `Capture` slots,
            // each at the offset the descriptor assigns it. What the shared
            // function does with them is identical to what a fusion-local
            // composition does with the run assembled at its call edge, which
            // is exactly the property that makes the two realizations the same
            // body.
            let lowered = lower_continuation_selected_case_body(
                compiler,
                &mut builder,
                &ContinuationSelectedCaseBody {
                    id: unit.id,
                    continuation_origin: unit.continuation_origin,
                    producer_alternative: unit.producer_alternative,
                    recursive_position: unit.recursive_position,
                    worker_closure_origin: unit.worker_closure_origin,
                    worker_body_origin: unit.worker_body_origin,
                    worker_declared_arity: unit.worker_declared_arity,
                    worker_capture_count: unit.worker_capture_count,
                },
                envelope,
                &ordinary,
                &carried_inputs,
                retargeted_worker_body,
            )?;

            // The Result slot is WRITTEN here and never read.
            let word = match lowered {
                LoweringOperand::Carried(carried) => carried.word,
                // The continuation Result frame slot carries K.
                LoweringOperand::Residual(residual) => {
                    #[cfg(any(test, feature = "px8-ds-test-support"))]
                    record_residual_abi_decode();
                    compiler.decode_residual_child(&mut builder, residual)?.word
                }
                LoweringOperand::Specialized(value) => compiler.emit_result(&mut builder, value)?.0,
            };
            let result_offset = i32::try_from(result_offset).map_err(|_| {
                backend_module(
                    LoweringCause::Capacity,
                    "continuation result slot offset exceeds range".to_string(),
                )
            })?;
            let result_store = builder
                .ins()
                .store(MemFlags::trusted(), word, frame, result_offset);
            let zero = builder.ins().iconst(types::I64, 0);
            let return_inst = builder.ins().return_(&[zero]);
            publication = FunctionResultPublication {
                frame,
                result_offset,
                trap_offset: i32::try_from(trap_offset).map_err(|_| {
                    backend_module(
                        LoweringCause::Capacity,
                        "continuation trap slot offset exceeds range".to_string(),
                    )
                })?,
                result_store,
                returned_word: word,
                return_inst,
            };
            compiler.record_checked_frame_terminal(&builder, FrameTerminalKind::Normal)?;
            builder.seal_all_blocks();
            builder.finalize();
        }
        ambient.release(compiler);
        frame_scope.close(compiler, &func)?;
        token_scope.close(compiler, &func)?;
        compiler.record_finished_grafted_spine_function(&func, bundle)?;
        verify_cranelift_function(&func, module.isa())?;
        compiler.commit_aggregate_events()?;
        staged.push(stage_result_body(
            compiler,
            ExistingResultUnitIdentity::Continuation(unit.id),
            id,
            func,
            Some(publication),
            None,
            None,
        ));
    }
    Ok(staged)
}

/// Reconstruct a raw source body's semantic environment from a generated
/// context's two ABI runs.
///
/// The context descriptor deliberately coalesces the raw body's declared
/// arguments and raw captures into one `Parameter` run so its caller can append
/// the enclosing specialization's continuation inputs as `Capture`s. That ABI
/// layout is not the source binding order: de Bruijn conversion applies only to
/// the declared-argument prefix. The raw captures already carry their lexical
/// order, and the context captures are a separate planner-ordered suffix.
///
/// The split is validated against the raw owner's own descriptor. A count from
/// the context alone cannot distinguish declared arguments from raw captures,
/// which is the same-cardinality permutation this boundary must reject.
pub(in crate::cranelift_backend) fn generated_context_source_environment<T>(
    mut combined_parameters: Vec<T>,
    context_captures: Vec<T>,
    raw_parameters: u32,
    raw_captures: u32,
    converts_source_parameters: bool,
) -> Result<Vec<T>, CraneliftBackendError> {
    let raw_parameters = usize::try_from(raw_parameters).map_err(|_| {
        backend_module(
            LoweringCause::Capacity,
            "generated-context raw parameter count exceeds this platform".to_string(),
        )
    })?;
    let raw_captures = usize::try_from(raw_captures).map_err(|_| {
        backend_module(
            LoweringCause::Capacity,
            "generated-context raw capture count exceeds this platform".to_string(),
        )
    })?;
    let expected = raw_parameters.checked_add(raw_captures).ok_or_else(|| {
        backend_module(
            LoweringCause::Capacity,
            "generated-context raw input count overflowed".to_string(),
        )
    })?;
    if combined_parameters.len() != expected {
        return Err(backend_module(
            LoweringCause::PlannerInvariant,
            format!(
                "a generated context's Parameter run holds {} operands, but its raw owner declares \
             {raw_parameters} parameters plus {raw_captures} captures",
                combined_parameters.len()
            ),
        ));
    }

    let mut captures = combined_parameters.split_off(raw_parameters);
    if converts_source_parameters {
        combined_parameters.reverse();
    }
    combined_parameters.append(&mut captures);
    combined_parameters.extend(context_captures);
    Ok(combined_parameters)
}

/// **`RT-DECL-CLOSURE-PORT` `D5a` — define each generated producer execution
/// context.**
///
/// The body lowered here is the **raw worker body**, unchanged. What differs
/// from that body's own ordinary unit is only the environment it runs in: the
/// context's frame carries the raw parameter run *and* the enclosing
/// specialization's continuation inputs, so a nested producer inside that body
/// can reach operands raw `fn2` provably never receives.
///
/// ⛔ **Two different questions about the raw unit, and they are decided by
/// different authorities. Do not answer one with the other.**
///
/// 1. **Descriptor, provenance and source-binding authority are RETAINED.**
///    The raw `ClosureBody` keeps its own descriptor, its own ABI, and its
///    status as the source binding for this body — the template lowered here
///    *is* that body, unchanged. ⛔ Nothing about this retarget mutates,
///    unions, or suffixes the raw descriptor.
/// 2. **Executable `Function` declaration/definition membership is NOT
///    retained by default.** It is decided from the **post-retarget final
///    graph**, by `StaticTransitionPlan::template_only_worker_bodies`: a raw
///    worker every selecting specialization has retargeted — and whose carried
///    invocation also binds a generated context — is **template-only**, and is
///    absent from the emitted-`Function` population. A raw worker with any
///    remaining final raw call stays executable.
///
/// ⚠ **This paragraph previously said the raw unit "is still emitted and is
/// not retired", keeping its body and "simply losing one caller".** That was
/// the pre-`D5a` reading and it is superseded: it answers question 2 with
/// question 1's answer, so it is true of the descriptor and false of the
/// emitted population whenever the retarget is total. The surviving half of
/// the old claim is the useful one — *"a new owner for a body does not retire
/// the old owner's unit"* remains correct about **authority**, and says
/// nothing about **membership**.
///
/// ⛔ The emission owner bound here is `Specialization(enclosing)`, and
/// `defining_unit` stays the **raw** owner. Deriving one from the other is the
/// conflation `evt_609am4v7cdt5b` ruled against, and it is precisely because
/// this function lowers someone else's body that the two must be supplied
/// independently.
pub(super) fn stage_continuation_context_bodies<M: Module>(
    module: &mut M,
    compiler: &mut Lowering<'_>,
    helpers: ArtifactHelpers<'_>,
    bundle: &UnitBundle,
    call_edges: &CallEdgeTargets,
) -> Result<Vec<StagedResultBody>, CraneliftBackendError> {
    struct OwnedContext {
        id: ContinuationContextId,
        enclosing: ContinuationSpecializationId,
        worker_body_origin: StaticOriginId,
        raw_owner: PredeclaredFunctionId,
        /// `RT-SRCBODY-BIND-ORDER` `D2` — the RAW owner's definition arm, read
        /// from the raw owner's own descriptor rather than judged here.
        ///
        /// This context lowers someone else's body, so the binding order that
        /// body needs is decided by the unit that owns it. Taking the answer
        /// from that descriptor is what makes the claimed equivalence hold by
        /// construction; deciding it independently here would make it two
        /// judgments that happen to agree.
        raw_owner_definition: AbiUnitDefinition,
        /// The raw source body's declared parameter run. The generated context
        /// places this run and the raw capture run together in its `Parameter`
        /// slots, but only this prefix is a source binder run.
        raw_parameters: u32,
        /// The raw source body's capture run, following `raw_parameters` inside
        /// the generated context's `Parameter` slots.
        raw_captures: u32,
        slots: Vec<AbiSlot>,
        offsets: Vec<u32>,
        header_parameters: u32,
        header_captures: u32,
        frame_bytes: u32,
        checked_ih_generated_entry_access: Option<CheckedIhGeneratedEntryAccess>,
        result_contract: Option<ConstructorIdentity>,
    }
    // Own every projected fact before the loop: the projection borrows the plan
    // and definition below needs the compiler mutably.
    let unit_authorities = compiler
        .static_transition_plan
        .emittable_units()?
        .into_iter()
        .map(|unit| (unit.function(), unit.definition(), unit.header()))
        .collect::<Vec<_>>();
    let contexts = compiler
        .static_transition_plan
        .continuation_contexts()?
        .into_iter()
        .map(|context| {
            let (offsets, frame_bytes) = context.slot_offsets()?;
            let raw_owner = context.raw_owner();
            // `D2` — a context whose raw owner has no descriptor is a context
            // whose body has no declared ABI, which is not a binding-order
            // question to answer conservatively.
            let (raw_owner_definition, raw_header) = unit_authorities
                .iter()
                .find(|(function, _, _)| *function == raw_owner)
                .map(|(_, definition, header)| (*definition, *header))
                .ok_or_else(|| {
                    backend_module(
                        LoweringCause::PlannerInvariant,
                        "a generated context's raw owner has no ABI descriptor".to_string(),
                    )
                })?;
            let checked_ih_generated_entry_access = compiler
                .static_transition_plan
                .checked_ih_generated_entry_access_for_context(
                    context.id(),
                    context.enclosing_specialization(),
                    context.worker_body_origin(),
                )?;
            // Consumer controls perturb only the sanitized clone after the
            // planner has validated and published it. The malformed projection
            // therefore reaches the real terminal guard through the ordinary
            // generated-function installation and forwarding path.
            #[cfg(feature = "px8-ds-test-support")]
            let checked_ih_generated_entry_access = {
                let mut access = checked_ih_generated_entry_access;
                if let Some(access) = access.as_mut() {
                    use super::source::CheckedIhGeneratedEntryCapsuleMutation as Mutation;
                    use crate::CheckedIhGeneratedEntryConfluenceMutation as ProjectionMutation;
                    let active = super::source::checked_ih_generated_entry_capsule_mutation();
                    let direct_mutation = match active {
                        Mutation::WrongDestinationOwner => {
                            Some(ProjectionMutation::DestinationOwner)
                        }
                        Mutation::WrongDestinationBody => Some(ProjectionMutation::DestinationBody),
                        Mutation::WrongBinding => Some(ProjectionMutation::BindingFrame),
                        Mutation::WrongLocatorInvocation => {
                            Some(ProjectionMutation::LocatorInvocation)
                        }
                        Mutation::WrongLocatorCallee => Some(ProjectionMutation::LocatorCallee),
                        Mutation::WrongLocatorDomain => Some(ProjectionMutation::LocatorDomain),
                        Mutation::WrongLocatorIndex => Some(ProjectionMutation::LocatorIndex),
                        _ => None,
                    };
                    let tail_mutation = match active {
                        Mutation::RetainedAccessWrongDestinationOwner => {
                            Some(ProjectionMutation::DestinationOwner)
                        }
                        Mutation::RetainedAccessWrongDestinationBody => {
                            Some(ProjectionMutation::DestinationBody)
                        }
                        Mutation::RetainedAccessWrongBinding => {
                            Some(ProjectionMutation::BindingFrame)
                        }
                        Mutation::RetainedAccessWrongLocatorInvocation => {
                            Some(ProjectionMutation::LocatorInvocation)
                        }
                        Mutation::RetainedAccessWrongLocatorCallee => {
                            Some(ProjectionMutation::LocatorCallee)
                        }
                        Mutation::RetainedAccessWrongLocatorDomain => {
                            Some(ProjectionMutation::LocatorDomain)
                        }
                        Mutation::RetainedAccessWrongLocatorIndex => {
                            Some(ProjectionMutation::LocatorIndex)
                        }
                        _ => None,
                    };
                    if let Some(mutation) = direct_mutation {
                        access.mutate_published_direct_projection_for_control(mutation);
                    }
                    if let Some(mutation) = tail_mutation {
                        access.mutate_published_tail_projection_for_control(mutation);
                    }
                }
                access
            };
            Ok(OwnedContext {
                id: context.id(),
                enclosing: context.enclosing_specialization(),
                worker_body_origin: context.worker_body_origin(),
                raw_owner,
                raw_owner_definition,
                raw_parameters: raw_header.parameters,
                raw_captures: raw_header.captures,
                slots: context.slots().to_vec(),
                offsets,
                header_parameters: context.header().parameters,
                header_captures: context.header().captures,
                frame_bytes,
                checked_ih_generated_entry_access,
                result_contract: compiler
                    .static_transition_plan
                    .checked_ih_generated_context_result_contract(context.id())?,
            })
        })
        .collect::<Result<Vec<_>, CraneliftBackendError>>()?;

    let worker_targets = resolve_worker_targets(&compiler.static_transition_plan, bundle)?;
    let mut staged = Vec::new();
    for context in contexts {
        let id = bundle.context(context.id).ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "a planned generated context was never forward-declared".to_string(),
            )
        })?;
        let slots = context.slots.as_slice();
        let offsets = context.offsets.as_slice();
        if slots.len() != offsets.len() {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "a generated context slot run disagrees with its own offset walk".to_string(),
            ));
        }
        let parameter_count = slots
            .iter()
            .filter(|slot| slot.kind == AbiSlotKind::Parameter)
            .count();
        let capture_count = slots
            .iter()
            .filter(|slot| slot.kind == AbiSlotKind::Capture)
            .count();
        // The header is the declared contract and the slot run is what the body
        // will actually walk; a disagreement here means the environment this
        // body binds is not the one its caller passes operands for.
        if u32::try_from(parameter_count).ok() != Some(context.header_parameters)
            || u32::try_from(capture_count).ok() != Some(context.header_captures)
        {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "a generated context's slot run disagrees with its declared frame header"
                    .to_string(),
            ));
        }
        let result_offset = slots
            .iter()
            .zip(offsets)
            .find(|(slot, _)| slot.kind == AbiSlotKind::Result)
            .map(|(_, offset)| *offset)
            .ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "generated context frame declares no result slot".to_string(),
                )
            })?;
        let trap_offset = slots
            .iter()
            .zip(offsets)
            .find(|(slot, _)| slot.kind == AbiSlotKind::Trap)
            .map(|(_, offset)| *offset)
            .ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "generated context frame declares no trap slot".to_string(),
                )
            })?;

        let sig = unit_signature(module);
        let mut func = Function::with_name_signature(UserFuncName::user(3, id.as_u32()), sig);
        let mut function_local = helpers.declare_in_func(module, &mut func, None);
        function_local.grafted_spine_scope =
            Some(GraftedSpineFunctionScope::ContinuationContext(context.id));
        // The raw body's OWN call edges, declared into this function. They are
        // the raw owner's edges because the body is the raw owner's body; what
        // this context changes is the environment, never which callees the
        // source names.
        let declared_calls = call_edges.declare_in_func(context.raw_owner, module, &mut func)?;
        function_local.unit_calls = declared_calls.static_bodies;
        // This generated Function is owned by the enclosing specialization even
        // though it lowers the raw owner's body. Retained closures reaching that
        // body can therefore name static-body graph edges owned outside the raw
        // source subtree. Declare those targets from the already-resolved graph
        // claims before body emission; the later call lookup remains unchanged
        // and fail-closed.
        call_edges.declare_retained_body_targets_in_func(
            context.raw_owner,
            module,
            &mut func,
            &mut function_local.unit_calls,
        )?;
        function_local.declaration_calls = declared_calls.declarations;
        function_local.worker_calls = worker_targets.declare_in_func(module, &mut func);
        // `D6b` -- no retarget happens in a generated context body, so the two
        // tables agree here. Populated anyway rather than left empty: the raw
        // route must resolve from ITS OWN table in every function, or the
        // resolution silently depends on which function it runs in.
        function_local.raw_worker_calls = function_local.worker_calls.clone();
        function_local.worker_templates = worker_targets.templates().clone();
        function_local.context_calls = declare_context_calls_in_func(
            module,
            &mut func,
            &compiler.static_transition_plan,
            bundle,
        )?;
        // `D5a`: this context's own causal call refs, selected by the EMISSION
        // owner. ⛔ Not by `raw_owner` -- that is the filter that would hand this
        // function the raw unit's tokens and leave its own undeclared.
        let emission_owner = ContinuationEmissionOwner::Specialization(context.enclosing);
        function_local.continuation_calls = match compiler.continuation_claims.as_ref() {
            Some(ledger) => ledger.declare_owned_in_func(
                emission_owner,
                module,
                &mut func,
                &compiler.static_transition_plan,
            )?,
            None => BTreeMap::new(),
        };
        if let Some(ledger) = compiler.continuation_claims.as_mut() {
            ledger.record_declared(function_local.continuation_calls.keys().cloned())?;
        }
        // Contract 3, on this context: the projection is taken before the
        // function is defined and keyed on the emission owner this pass is about
        // to bind.
        let result_edges = compiler
            .static_transition_plan
            .continuation_result_edges_owned_by(emission_owner)?;

        // `D8n` — this generated Function's own checked-frame consumption
        // transaction, spanning the generated-context body exactly. ⛔ Opened before the builder and
        // closed after it, so every branch scope inside nests within it.
        let frame_scope = CheckedFrameFunctionScope::open(compiler)?;
        let token_scope = CheckedTokenFunctionScope::open(compiler)?;
        let mut func_ctx = FunctionBuilderContext::new();
        let publication;
        {
            let mut builder = FunctionBuilder::new(&mut func, &mut func_ctx);
            let entry = builder.create_block();
            builder.append_block_params_for_function_params(entry);
            builder.switch_to_block(entry);
            let envelope_pointer = builder.block_params(entry)[0];
            let frame = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                envelope_pointer,
                crate::activation_services::UNIT_CALL_FRAME_SLOTS,
            );
            let host_dispatch_context = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                envelope_pointer,
                crate::activation_services::UNIT_CALL_FRAME_HOST_DISPATCH_CONTEXT,
            );
            let services = builder.block_params(entry)[1];
            let native_int_arena = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                services,
                crate::activation_services::SERVICES_NATIVE_INT_ARENA,
            );
            Lowering::require_nonzero(&mut builder, native_int_arena);
            let boundary_arena = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                services,
                crate::activation_services::SERVICES_BOUNDARY_ARENA,
            );
            Lowering::require_nonzero(&mut builder, boundary_arena);
            function_local.host_dispatch_context = Some(host_dispatch_context);
            function_local.native_int_arena = Some(native_int_arena);
            function_local.boundary_arena = Some(boundary_arena);
            function_local.services_pointer = Some(services);
            function_local.bind_unit_trap_frame(
                frame,
                i32::try_from(trap_offset).map_err(|_| {
                    backend_module(
                        LoweringCause::Capacity,
                        "generated context trap slot offset exceeds range".to_string(),
                    )
                })?,
            )?;
            function_local.bind_pending_vis_frame(
                compiler.pending_vis_record_protocol.as_ref(),
                PendingVisFrameOwner::Context(context.id),
                frame,
                context.frame_bytes,
            )?;
            if let Some(access) = &context.checked_ih_generated_entry_access {
                if access.context() != context.id
                    || access.enclosing_specialization() != context.enclosing
                    || access.worker_body_origin() != context.worker_body_origin
                {
                    return Err(backend_module(LoweringCause::PlannerInvariant,
                        "a sanitized checked-IH generated-entry access disagrees with the exact context function being defined"
                            .to_string(),
                    ));
                }
            }
            function_local.checked_ih_generated_entry_access =
                context.checked_ih_generated_entry_access.clone();
            #[cfg(feature = "px8-ds-test-support")]
            if let Some(access) = &function_local.checked_ih_generated_entry_access {
                record_checked_ih_generated_entry_installed(access);
            }
            compiler.function_local = function_local;
            compiler.open_aggregate_events(id)?;
            // `D8o` — same binding, same unchanged domain: the emission owner is
            // the enclosing specialization and `defining_unit` stays the RAW
            // owner, which is the distinction the two fields exist to keep.
            // ⛔ After `open_aggregate_events`, so the observation this binding
            // writes is labelled with the Function it belongs to.
            let ambient = AmbientBodyAuthority::bind(compiler, emission_owner, context.raw_owner);
            // `D8o` — a generated CONTEXT body, whose owner is a Specialization
            // and whose kind is not.
            #[cfg(test)]
            crate::cranelift_backend::lowering::record_d8o_body_key(
                compiler.defining_function_id,
                crate::cranelift_backend::lowering::D8oBodyKey::GeneratedContext(context.id),
            );

            // The environment is the raw body's declared-argument prefix, then
            // its raw captures, then the generated context's continuation-input
            // captures. The context ABI coalesces the first two into one
            // `Parameter` run, so descriptor kind alone does not identify the
            // source binder boundary.
            //
            // `RT-SRCBODY-BIND-ORDER` `D2`: the raw owner's descriptor supplies
            // that boundary. Only its declared-argument prefix receives the
            // source de Bruijn conversion; both capture runs retain their own
            // planner order. Reversing the coalesced context `Parameter` run
            // instead is a same-cardinality permutation that moves the last raw
            // capture into the source parameter's semantic position.
            let mut context_parameters = Vec::new();
            let mut context_captures = Vec::new();
            #[cfg(test)]
            let mut context_parameter_ordinals = Vec::new();
            for (slot, offset) in slots.iter().zip(offsets) {
                if !matches!(slot.kind, AbiSlotKind::Parameter | AbiSlotKind::Capture) {
                    continue;
                }
                let offset = i32::try_from(*offset).map_err(|_| {
                    backend_module(
                        LoweringCause::Capacity,
                        "generated context slot offset exceeds addressable range".to_string(),
                    )
                })?;
                let word = builder
                    .ins()
                    .load(types::I64, MemFlags::trusted(), frame, offset);
                let operand = LoweringOperand::Carried(CarriedBoundaryWord { word });
                // RT-CAPTURE-PROJECTION-GROW D2 seam (i) -- record the SAME
                // operand by ABI position, mirroring the ordinary-unit push.
                //
                // The context body loads its parameter-then-capture run from its
                // own frame, but until now nothing wrote that run into
                // function_local.defining_abi_operands -- the vector every
                // capture-gather indexes. A gather reached under a context body
                // therefore read whatever run the enclosing function had left
                // there, which is the measured "slot 2 outside the emitting
                // function's 2 ABI operands".
                //
                // Pushed from THIS walk rather than rebuilt, so "index i is ABI
                // position i" holds by construction rather than by two walks
                // agreeing -- the same reason the ordinary-unit push is sited in
                // its walk.
                //
                // In DESCRIPTOR order, before any binding-order conversion: the
                // conversion below reorders the semantic environment only. That
                // separation is RT-SRCBODY-BIND-ORDER D1's invariant about this
                // exact vector, and it is preserved here rather than restated.
                compiler
                    .function_local
                    .defining_abi_operands
                    .push(operand.clone());
                #[cfg(test)]
                compiler
                    .function_local
                    .defining_abi_slot_kinds
                    .push(slot.kind);
                let binding = LoweringEnvironmentBinding::Value(operand);
                match slot.kind {
                    AbiSlotKind::Parameter => {
                        #[cfg(test)]
                        context_parameter_ordinals.push(slot.ordinal);
                        context_parameters.push(binding)
                    }
                    _ => context_captures.push(binding),
                }
            }
            let context_converts = source_body_binding_order(context.raw_owner_definition)?;
            let env = generated_context_source_environment(
                context_parameters,
                context_captures,
                context.raw_parameters,
                context.raw_captures,
                context_converts,
            )?;
            #[cfg(test)]
            let raw_parameter_count = usize::try_from(context.raw_parameters)
                .expect("raw parameter count fits this platform after environment construction");
            #[cfg(test)]
            let raw_capture_count = usize::try_from(context.raw_captures)
                .expect("raw capture count fits this platform after environment construction");
            #[cfg(test)]
            let mut source_parameter_ordinals =
                context_parameter_ordinals[..raw_parameter_count].to_vec();
            #[cfg(test)]
            if context_converts {
                source_parameter_ordinals.reverse();
            }
            #[cfg(test)]
            let source_capture_ordinals = context_parameter_ordinals
                [raw_parameter_count..raw_parameter_count + raw_capture_count]
                .iter()
                .map(|ordinal| ordinal - context.raw_parameters)
                .collect::<Vec<_>>();
            // `RT-SRCBODY-BIND-ORDER` `D3` control 4, amended (Architect,
            // producer-wide) -- the TRANSITION SENTINEL, sited at the producer
            // edge rather than inside one witness's test.
            //
            // **This gate is designed to go RED, and that red is its purpose.**
            // `D2` says a generated context binds its raw owner's parameter run
            // in the same order that owner's own unit would. `D1`'s conversion
            // reverses that run, and reversal is the IDENTITY on a run of
            // length one -- so at unary arity `D2` is inert and its cross-host
            // equivalence is not observable at all. The first generated-context
            // worker of arity two or more makes it observable, and at that
            // moment the equivalence obligation this node deferred becomes live
            // and UNMEASURED.
            //
            // **Sited here because a test can only watch the compiles it runs.**
            // The first cut asserted this bound inside the `px8tr` control, over
            // the observations of that one compile. A multi-parameter worker
            // introduced by any other program would never have entered that
            // observation vector, so the sentinel would have stayed green while
            // the obligation activated -- watching a witness while claiming a
            // population. Every generated context this crate builds passes
            // through this loop, so the gate is closed over the PRODUCER.
            //
            // **The residual, stated rather than implied.** `cfg(test)` here
            // means the `ken-runtime` lib-test build: a generated context built
            // while compiling an integration-test binary or a downstream crate
            // does not arm this gate. The fixture population that could
            // introduce a multi-parameter worker is itself `cfg(test)`
            // (`mod test_objects`), so it lies inside the reach; a worker
            // arising only from a real Ken program exercised solely by an
            // integration test lies outside it.
            //
            // Do NOT satisfy this by relaxing the bound -- a `<= 2` restates the
            // current population as the contract and destroys the gate. The
            // retiring event is the introduction of the arity-two worker, and
            // the deliverable then is the equivalence control, not a wider
            // bound. `d3_generated_context_arity_sentinel_edge_is_reached` is
            // the reaching positive control that keeps this non-vacuous.
            #[cfg(test)]
            assert!(
                raw_parameter_count <= 1,
                "a generated-context worker declares {raw_parameter_count} source parameters. \
                 D2's binding order is no longer inert, so its cross-host equivalence is now \
                 OBSERVABLE and UNMEASURED. The deliverable is the cross-host equivalence \
                 control -- that this body binds the same order in its own unit and in the \
                 context that lowers it -- not a wider bound at this gate. Recorded source \
                 ordinals: {source_parameter_ordinals:?}"
            );
            #[cfg(test)]
            srcbody_bind_order_record(SrcbodyBindOrderObservation {
                host: SrcbodyBindHost::GeneratedContext,
                definition: context.raw_owner_definition,
                converted: context_converts,
                body_origin: context.worker_body_origin,
                parameter_ordinals: source_parameter_ordinals,
                capture_ordinals: source_capture_ordinals,
            });

            let body = compiler.retained_body_occurrence(context.worker_body_origin)?;
            let lowered = compiler.lower_expr(&mut builder, body, &env)?;
            // The detached-result seat, live in this context. Every operand its
            // capture projection names is reachable from `env` above.
            let lowered = compiler.eliminate_detached_producer_continuation(
                &mut builder,
                &result_edges,
                lowered,
                &env,
            )?;
            let word = match lowered {
                LoweringOperand::Carried(word) => Some(word.word),
                LoweringOperand::Specialized(Lowered::Trap(trap)) => {
                    compiler.emit_current_trap(&mut builder, &trap)?;
                    None
                }
                LoweringOperand::Specialized(value) => Some(
                    compiler
                        .transfer_unit_result_into_carrier(
                            &mut builder,
                            context.worker_body_origin,
                            &value,
                        )?
                        .word,
                ),
                // The generated-context Result frame slot carries K.
                LoweringOperand::Residual(residual) => {
                    #[cfg(any(test, feature = "px8-ds-test-support"))]
                    record_residual_abi_decode();
                    Some(compiler.decode_residual_child(&mut builder, residual)?.word)
                }
            };
            let result_offset = i32::try_from(result_offset).map_err(|_| {
                backend_module(
                    LoweringCause::Capacity,
                    "generated context result slot offset exceeds range".to_string(),
                )
            })?;
            let trap_offset = i32::try_from(trap_offset).map_err(|_| {
                backend_module(
                    LoweringCause::Capacity,
                    "generated context trap slot offset exceeds range".to_string(),
                )
            })?;
            let pending_publication = if let Some(word) = word {
                if let Some(contract) = context.result_contract {
                    // Runtime validation remains defence in depth. It is not
                    // compiler authority: the staged finished proof below must
                    // establish the word before this function can be defined.
                    let actual =
                        compiler.emit_carrier_tag(&mut builder, CarriedBoundaryWord { word })?;
                    let expected = i64::try_from(contract.tag_abi_word()?).map_err(|_| {
                        backend_module(
                            LoweringCause::Capacity,
                            "generated context Result identity exceeds the runtime tag word"
                                .to_string(),
                        )
                    })?;
                    Lowering::require_i64(&mut builder, actual, expected);
                }
                let result_store =
                    builder
                        .ins()
                        .store(MemFlags::trusted(), word, frame, result_offset);
                Some((result_store, word))
            } else {
                None
            };
            let status = builder.ins().iconst(types::I64, 0);
            let return_inst = builder.ins().return_(&[status]);
            publication = pending_publication.map(|(result_store, returned_word)| {
                FunctionResultPublication {
                    frame,
                    result_offset,
                    trap_offset,
                    result_store,
                    returned_word,
                    return_inst,
                }
            });
            compiler.record_checked_frame_terminal(&builder, FrameTerminalKind::Normal)?;
            builder.seal_all_blocks();
            builder.finalize();
            ambient.release(compiler);
        }
        frame_scope.close(compiler, &func)?;
        // The same emission-seam gate every other generated function passes: the
        // callee of each recorded causal emission is decoded back out of THIS
        // finished CLIF and compared with the planner-issued target.
        compiler.verify_emitted_continuation_calls(&func, bundle)?;
        // `D8j` — the composed relation's own gate, beside the direct one and
        // never inside it: the two answer different questions about different
        // callees.
        compiler.verify_recorded_composed_discharges(&func, bundle)?;
        #[cfg(test)]
        crate::cranelift_backend::lowering::record_d8j_discharged(
            compiler.function_local.composed_discharges.keys().cloned(),
        );
        // `D8k` -- the composed half of the partition, accumulated from the
        // VERIFIED relation and never from the direct instruction map.
        if let Some(ledger) = compiler.continuation_claims.as_mut() {
            ledger.record_composed(
                compiler.function_local.composed_discharges.keys().cloned(),
                emission_owner,
            )?;
        }
        token_scope.close(compiler, &func)?;
        let response_owner_calls = verified_response_owner_calls(
            &compiler.static_transition_plan,
            bundle,
            compiler.function_local.continuation_emissions.keys().chain(
                compiler
                    .function_local
                    .checked_ih_transport_emissions
                    .iter()
                    .map(|(transport, _)| transport.source_call_identity()),
            ),
        )?;
        if let Some(ledger) = compiler.continuation_claims.as_mut() {
            ledger.record_emitted(
                compiler
                    .function_local
                    .continuation_emissions
                    .keys()
                    .cloned(),
            )?;
            ledger.record_response_owner_calls(response_owner_calls);
        }
        compiler.record_finished_grafted_spine_function(&func, bundle)?;
        verify_cranelift_function(&func, module.isa())?;
        compiler.commit_aggregate_events()?;
        staged.push(stage_result_body(
            compiler,
            ExistingResultUnitIdentity::Context(context.id),
            id,
            func,
            publication,
            context.result_contract,
            None,
        ));
    }
    Ok(staged)
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` — define one `Function` per installed
/// fused region: the producer's body and the consumer's suffix, in one frame.**
///
/// **What this actually binds, and it is PROVISIONAL: one authority, the
/// producer's, around the whole combined lowering.** The single
/// `AmbientBodyAuthority::bind` below takes
/// `Predeclared(producer_owner)`/`producer_owner` and spans the producer's body
/// *and* the consumer's suffix together. There is no consumer-phase switch in
/// this function.
///
/// **That is a known gap, not the design, and it is EXCLUDED from this cut.**
/// The consumer's case bodies are therefore lowered against the producer's
/// source-lookup authority. Armed and left this way it would be a wrong program
/// rather than a missing one, because the binders would resolve — to the wrong
/// terms.
///
/// **What makes that harmless today is the EMPTY FUSION POPULATION, and not
/// `D2F_EMITTER_ARMED`.** This function is called unconditionally on every
/// production compile; it is only the *installer* that the constant gates. So it
/// defines zero bodies because `continuation_fusions()` yields nothing to
/// iterate, which is a consequence of the gated install rather than a property
/// of this call site. An earlier wording named the constant as the reason, which
/// let an auditor confirm `false` and stop one hop short of the guarantee that
/// actually holds. **The reachable-empty path is deliberate** — guarding this
/// call as well would make the zero-fusion case take a different path from the
/// non-zero case, which is the shape the node is trying to keep exercised.
///
/// **The excluded repair, named so it is not re-derived:** the ambient authority
/// must move `Predeclared(producer) -> Predeclared(consumer) -> producer` across
/// the suffix, and the fused region needs its own checked-frame adoption,
/// because a `CheckedFrameFunctionScope` is a per-`Function` transaction while a
/// fused region's frame spans the producer's body and the consumer's suffix
/// across one. Both are later Architect-ruled wiring. Do not read the paragraph
/// above as a statement that either exists.
///
/// **The suffix is run ONCE, here.** Node `:650` measured that redirecting the
/// producer invocation alone leaves the consumer's own suffix live and executes
/// it twice. That is why the redirect, this definition, and the consumer takeover
/// are one irreducible core: any two of the three, without the third, is a wrong
/// artifact and not a partial one.
///
/// **No new elimination machinery.** The elimination is
/// `lower_computational_producer_expr` with a single-frame eliminator stack — the
/// same entry the producer dispatcher's own fallback arm uses. It handles the
/// carried and the specialized phase already, so nothing here re-derives case
/// selection, and nothing synthesizes an `ActiveContinuationFrame`.
///
/// **The claim is READ, not consumed.** The takeover at the consumer's seat is
/// the one consumption, and consuming here would make a definition spend the
/// affine right the redirect's seat still needs.
/// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — which origin keys the recursive self
/// edge and which origin records its call site.**
///
/// A two-line function because the choice is the whole content: the map key and
/// `origin` are the **callee body**, and `call_site_origin` is the **consuming
/// call**. They are different occurrences and this is the only place that says
/// so, so a control can assert the production decision instead of a copy of it.
///
/// **Why it is worth naming at all.** On the `R3` witness the claim's `seat`,
/// its `producer_body` and its redirect's callee entry ALL print `37` while the
/// consuming call is `17`. A fold of call site into body type-checks, and a
/// control whose expected values are every `37` still passes under the fold —
/// separating `17` from `37` is the only thing that catches it.
pub(super) fn fusion_self_edge_identities(
    producer_body: StaticOriginId,
    consuming_call: StaticOriginId,
) -> (StaticOriginId, StaticOriginId) {
    (producer_body, consuming_call)
}

pub(super) fn define_static_continuation_fusion_bodies<M: Module>(
    module: &mut M,
    compiler: &mut Lowering<'_>,
    helpers: ArtifactHelpers<'_>,
    bundle: &UnitBundle,
    call_edges: &CallEdgeTargets,
) -> Result<usize, CraneliftBackendError> {
    struct OwnedFusion {
        id: StaticContinuationFusionId,
        producer_owner: PredeclaredFunctionId,
        consumer_owner: PredeclaredFunctionId,
        producer_body: StaticOriginId,
        /// `RT-LEXICAL-R3-FUSION-EMITTER` `D3` -- the CALL SITE the moved
        /// suffix's claimed IH invocation is authorized at. Deliberately kept
        /// beside `producer_body` rather than folded into it: this is the
        /// occurrence that calls, that is the body called, and in this fixture
        /// they are different origins that a numeric coincidence elsewhere
        /// could hide.
        consuming_call: StaticOriginId,
        consuming_callee: StaticOriginId,
        redirect_callee: StaticOriginId,
        continuation_origin: StaticOriginId,
        /// `RT-LEXICAL-R3-FUSION-EMITTER` `D2` — the claim's consumer frame
        /// identity, re-entered locally inside the fused function.
        checked_frame_id: u64,
        slots: Vec<AbiSlot>,
        offsets: Vec<u32>,
        /// The INSTALLED fusion frame's whole header, carried rather than
        /// rebuilt from its parts. `D3`'s recursive self edge targets this
        /// definition, so it must present the frame contract the region was
        /// installed with; reassembling a header here is how it would come to
        /// differ in a field nobody thought to copy.
        header: AbiFrameHeader,
        header_parameters: u32,
        header_captures: u32,
    }
    // Own every projected fact before the loop, for the same reason the context
    // pass does: the projection borrows the plan and definition below needs the
    // compiler mutably.
    //
    // The two ORIGINS come from the claim, not from the view. The view carries
    // the plane's key; the claim carries the region the ledger actually admitted,
    // and those are the same fact only when preflight accepted. Reading the view
    // here would define a body for a region whose claim was refused.
    let fusions = {
        let claims = compiler.fusion_claims.as_ref();
        compiler
            .static_transition_plan
            .continuation_fusions()?
            .into_iter()
            .map(|fusion: StaticContinuationFusionView<'_>| {
                let (offsets, _frame_bytes) = fusion.slot_offsets()?;
                let claim = claims
                    .and_then(|ledger| ledger.claim(fusion.id()))
                    .ok_or_else(|| {
                        backend_module(
                            LoweringCause::PlannerInvariant,
                            "an installed static continuation fusion has no outstanding region \
                             claim, so its definition would take over a suffix nothing admitted"
                                .to_string(),
                        )
                    })?;
                Ok(OwnedFusion {
                    id: fusion.id(),
                    producer_owner: claim.producer_owner(),
                    consumer_owner: claim.consumer_owner(),
                    producer_body: claim.producer_body(),
                    consuming_call: claim.consuming_call(),
                    consuming_callee: claim.consuming_callee(),
                    redirect_callee: claim.redirect().callee_origin(),
                    continuation_origin: claim.continuation_origin(),
                    // `D2` -- the consumer frame identity the claim was
                    // preflighted against, carried so the fused body re-enters
                    // THAT frame rather than resolving one of its own.
                    checked_frame_id: claim.checked_transport().frame_id(),
                    slots: fusion.slots().to_vec(),
                    offsets,
                    header: fusion.header(),
                    header_parameters: fusion.header().parameters,
                    header_captures: fusion.header().captures,
                })
            })
            .collect::<Result<Vec<_>, CraneliftBackendError>>()?
    };

    let worker_targets = resolve_worker_targets(&compiler.static_transition_plan, bundle)?;
    let mut defined = 0usize;
    for fusion in fusions {
        let id = bundle.fusion(fusion.id).ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "an installed static continuation fusion was never forward-declared".to_string(),
            )
        })?;
        let slots = fusion.slots.as_slice();
        let offsets = fusion.offsets.as_slice();
        if slots.len() != offsets.len() {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "a fused region slot run disagrees with its own offset walk".to_string(),
            ));
        }
        let parameter_count = slots
            .iter()
            .filter(|slot| slot.kind == AbiSlotKind::Parameter)
            .count();
        let capture_count = slots
            .iter()
            .filter(|slot| slot.kind == AbiSlotKind::Capture)
            .count();
        // The header is the declared contract and the slot run is what this body
        // will actually walk; a disagreement means the environment this body
        // binds is not the one the redirected invocation passes operands for.
        if u32::try_from(parameter_count).ok() != Some(fusion.header_parameters)
            || u32::try_from(capture_count).ok() != Some(fusion.header_captures)
        {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "a fused region's slot run disagrees with its declared frame header".to_string(),
            ));
        }
        let result_offset = slots
            .iter()
            .zip(offsets)
            .find(|(slot, _)| slot.kind == AbiSlotKind::Result)
            .map(|(_, offset)| *offset)
            .ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "fused region frame declares no result slot".to_string(),
                )
            })?;
        let trap_offset = slots
            .iter()
            .zip(offsets)
            .find(|(slot, _)| slot.kind == AbiSlotKind::Trap)
            .map(|(_, offset)| *offset)
            .ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "fused region frame declares no trap slot".to_string(),
                )
            })?;

        let sig = unit_signature(module);
        let mut func = Function::with_name_signature(UserFuncName::user(4, id.as_u32()), sig);
        let mut function_local = helpers.declare_in_func(module, &mut func, None);
        function_local.grafted_spine_scope = Some(GraftedSpineFunctionScope::Fusion(fusion.id));
        // The PRODUCER's own call edges, declared into this function. This is
        // what carries the inherited producer edge: the producer's body still
        // names the callees the source named, and it is only its *host* that
        // changed. Not the consumer's edges — the consumer's redirected
        // invocation is the edge that reaches this function, not one inside it.
        let declared_calls =
            call_edges.declare_in_func(fusion.producer_owner, module, &mut func)?;
        function_local.unit_calls = declared_calls.static_bodies;
        // ---- `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — THE DEFINITION-LOCAL
        // ---- FUSION-RECURSIVE SELF EDGE (Architect, via `evt_6fzg11hpvfp4w`).
        //
        // The suffix moved into this function, and its claimed IH invocation is
        // a **recursive call to this same `Fusion(id)`** — not a no-op forward
        // of a current result, and not a call to the standalone producer, whose
        // body this definition now owns.
        //
        // Without this the fused body reaches
        // `call_declared_unit(producer_body)` against a table holding only the
        // producer's inherited edges and refuses with *"retained body ... has no
        // graph-derived call target in this unit"*. That refusal is correct and
        // stays: this supplies the edge the body genuinely contains rather than
        // relaxing the check.
        //
        // **Authorized from the preflighted claim, never from the failure.** The
        // ruling forbids inferring this edge from a lookup miss, from numeric
        // coincidence between origins, from body shape, or from ambient
        // "inside a fusion" state — so each fact below is read off `claim`.
        //
        // **Call-site identity is kept distinct from body identity.** The key
        // and `origin` are the callee body; `call_site_origin` is
        // `claim.consuming_call()`. In this fixture the body and the claim's
        // seat both print `37` while the consuming call is `17`; folding them
        // would compile and would make a wrong-call-site edge indistinguishable
        // from a right one.
        //
        // **The header and slots are the installed FUSION frame's**, never the
        // producer's — the callee is this fused definition, so the frame
        // contract is the one it was installed with.
        //
        // This is a **separate, definition-local obligation** from the external
        // consumer-to-fusion redirect, which remains the sole affine external
        // redirect. It consumes no region claim and records no redirect.
        // The half of the ruled verification that is genuinely derivable HERE:
        // the redirect's producer entry and the claim's producer body must be
        // the same occurrence. They reach the claim by different routes --
        // `producer_body` from `invocation_callee_entry` via the unique static
        // body triple, `redirect` from `fusion_redirect_target` -- so their
        // agreement is a real cross-check rather than a field compared with
        // itself.
        //
        // MEASURED, and stated rather than implied: `consuming_callee` does NOT
        // equal `producer_body` and must not be compared to it. On this witness
        // the consuming call is `17`, its callee occurrence `16`, and the
        // producer body `37`. The callee reaches the body through an IH
        // BINDING, and `CheckedIhBinding` names a `frame_origin` and a recursive
        // position -- not a body. An earlier draft of this guard compared the
        // two ids directly and refused every lawful region.
        //
        // ---- CORRECTION. An earlier revision of THIS comment said that
        // ---- resolution "is established at preflight". That was FALSE.
        //
        // Architect, relayed at `evt_45xd3px862ejs`: preflight's
        // `BinderAgreement` proves only marginal facts -- that the consuming
        // callee is an IH at the admitted frame and recursive position, and that
        // the result root equals the invocation callee entry. It does NOT prove
        // that this exact callee/binder resolves to that producer body.
        //
        // ---- AND THAT RELATION IS NOW CLOSED, PLANNER-SIDE. Ruled at
        // ---- `evt_2rw6vhq8xrqcm`; landed as
        // ---- `FusionClaimRefusal::BinderBodyResolution`.
        //
        // Preflight now re-resolves the consuming callee through the planner's
        // own binding authority and resolves that binder to a body, refusing
        // BEFORE the claim is issued unless it is the body being redirected. So
        // by the time a claim reaches this seat the relation is a checked
        // property of the certificate, not an assumption -- and `producer_body`
        // is that checked common result.
        //
        // ⇒ **What lowering keeps is still exactly the independent cross-check
        // below, and closing the relation upstream is not a licence to grow
        // it.** A second body authority here was ruled out in the same message:
        // re-deriving the binder relation in lowering would create a second
        // planner, and `ih_bindings` and `SemanticIr::child_origin` are
        // planner-private precisely so that cannot happen quietly.
        if fusion.redirect_callee != fusion.producer_body {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "a fused region's redirect names a producer entry other than the claim's producer \
                 body, so the definition-local recursive edge would target a body this claim \
                 never admitted"
                    .to_string(),
            ));
        }
        // The two identities, chosen by one shared function so the control that
        // separates them exercises the production decision rather than a copy of
        // it.
        let (self_edge_body, self_edge_call_site) =
            fusion_self_edge_identities(fusion.producer_body, fusion.consuming_call);
        let self_edge = DeclaredUnitCall {
            function: module.declare_func_in_func(id, &mut func),
            frame_owner: Some(PendingVisFrameOwner::Fusion(fusion.id)),
            origin: self_edge_body,
            call_site_origin: self_edge_call_site,
            header: fusion.header,
            slots: slots.to_vec(),
            offsets: offsets.to_vec(),
            result_contract: None,
        };
        // INSERTED, and its absence beforehand is required rather than assumed —
        // the producer's inherited edges must not already answer for this body,
        // or the suffix would have two answers to one lookup and the standalone
        // producer would stay reachable beside its fused definition.
        if function_local
            .unit_calls
            .insert(self_edge_body, self_edge)
            .is_some()
        {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "a fused definition already holds a call target for the body it owns, so its \
                 recursive suffix edge would be one of two answers to the same lookup"
                    .to_string(),
            ));
        }
        function_local.declaration_calls = declared_calls.declarations;
        function_local.worker_calls = worker_targets.declare_in_func(module, &mut func);
        // No retarget happens in a fused body, so the two tables agree. Populated
        // anyway rather than left empty, for the same reason as every other
        // generated function: the raw route must resolve from its own table here,
        // or resolution silently depends on which function it runs in.
        function_local.raw_worker_calls = function_local.worker_calls.clone();
        function_local.worker_templates = worker_targets.templates().clone();
        function_local.context_calls = declare_context_calls_in_func(
            module,
            &mut func,
            &compiler.static_transition_plan,
            bundle,
        )?;
        // The causal call refs declared here are the **PRODUCER's**, under
        // `Predeclared(producer_owner)`.
        //
        // The planner issues causal tokens for the producer's body under that
        // owner, and installing body ownership removed the producer's standalone
        // `Function` — so this is now the only `Function` that lowers that body,
        // and therefore the only one that can declare its refs.
        //
        // **There is deliberately no `Fusion(id)` owner in this function.** An
        // earlier cut minted `ContinuationEmissionOwner::Fusion(fusion.id)` here
        // and described it as the ambient authority. It was never bound to
        // anything: the sole `AmbientBodyAuthority::bind` below takes
        // `causal_owner`, and the planner issues no `Fusion`-owned tokens, so
        // asking for them returns the empty set and the producer's first causal
        // call refuses with *"the claimed continuation target was not declared
        // into this function"*. Whether the fused region ever becomes a causal
        // emission owner in its own right is later Architect-ruled wiring and is
        // excluded here; **it is not the case today, and this variable must not
        // be reintroduced to suggest otherwise.**
        //
        // NOT the union with the consumer's tokens. The consumer's own
        // `Function` still exists and still declares them; declaring them here
        // as well would be a second declaration of one token, which the claim
        // ledger's exact-set law refuses — correctly.
        let causal_owner = ContinuationEmissionOwner::Predeclared(fusion.producer_owner);
        function_local.continuation_calls = match compiler.continuation_claims.as_ref() {
            Some(ledger) => ledger.declare_owned_in_func(
                causal_owner,
                module,
                &mut func,
                &compiler.static_transition_plan,
            )?,
            None => BTreeMap::new(),
        };
        if let Some(ledger) = compiler.continuation_claims.as_mut() {
            ledger.record_declared(function_local.continuation_calls.keys().cloned())?;
        }
        let result_edges = compiler
            .static_transition_plan
            .continuation_result_edges_owned_by(causal_owner)?;

        let frame_scope = CheckedFrameFunctionScope::open(compiler)?;
        let token_scope = CheckedTokenFunctionScope::open(compiler)?;
        let mut func_ctx = FunctionBuilderContext::new();
        {
            let mut builder = FunctionBuilder::new(&mut func, &mut func_ctx);
            let entry = builder.create_block();
            builder.append_block_params_for_function_params(entry);
            builder.switch_to_block(entry);
            let envelope_pointer = builder.block_params(entry)[0];
            let frame = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                envelope_pointer,
                crate::activation_services::UNIT_CALL_FRAME_SLOTS,
            );
            let host_dispatch_context = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                envelope_pointer,
                crate::activation_services::UNIT_CALL_FRAME_HOST_DISPATCH_CONTEXT,
            );
            let services = builder.block_params(entry)[1];
            let native_int_arena = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                services,
                crate::activation_services::SERVICES_NATIVE_INT_ARENA,
            );
            Lowering::require_nonzero(&mut builder, native_int_arena);
            let boundary_arena = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                services,
                crate::activation_services::SERVICES_BOUNDARY_ARENA,
            );
            Lowering::require_nonzero(&mut builder, boundary_arena);
            function_local.host_dispatch_context = Some(host_dispatch_context);
            function_local.native_int_arena = Some(native_int_arena);
            function_local.boundary_arena = Some(boundary_arena);
            function_local.services_pointer = Some(services);
            function_local.bind_unit_trap_frame(
                frame,
                i32::try_from(trap_offset).map_err(|_| {
                    backend_module(
                        LoweringCause::Capacity,
                        "fused region trap slot offset exceeds range".to_string(),
                    )
                })?,
            )?;
            function_local.bind_pending_vis_frame(
                compiler.pending_vis_record_protocol.as_ref(),
                PendingVisFrameOwner::Fusion(fusion.id),
                frame,
                fusion.header.frame_bytes,
            )?;
            compiler.function_local = function_local;
            compiler.open_aggregate_events(id)?;

            // The two runs, walked once and kept SEPARATE. The context pass
            // concatenates them because one body binds both; here they are two
            // environments for two BODIES — the producer's and the consumer's
            // suffix — and concatenating would hand the producer the consumer's
            // captures.
            //
            // Separate ENVIRONMENTS, not separate authorities. The single
            // ambient authority below spans both; see the bind for what is
            // provisional about that.
            let mut parameters = Vec::new();
            let mut captures = Vec::new();
            for (slot, offset) in slots.iter().zip(offsets) {
                if !matches!(slot.kind, AbiSlotKind::Parameter | AbiSlotKind::Capture) {
                    continue;
                }
                let offset = i32::try_from(*offset).map_err(|_| {
                    backend_module(
                        LoweringCause::Capacity,
                        "fused region slot offset exceeds addressable range".to_string(),
                    )
                })?;
                let word = builder
                    .ins()
                    .load(types::I64, MemFlags::trusted(), frame, offset);
                let binding = LoweringEnvironmentBinding::Value(LoweringOperand::Carried(
                    CarriedBoundaryWord { word },
                ));
                // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the CURRENT FUSED
                // FRAME's ordered operand run, recorded in this same single
                // walk and in descriptor order.
                //
                // `fused_redirect_inputs` resolves `claim.inputs()` by indexing
                // `defining_abi_operands` of the function making the call. Until
                // the suffix moved in, that function was always the consumer's
                // predeclared unit, which populates the run in its own slot
                // walk. The fused definition never did, so the recursive self
                // edge above refused with *"names entry ABI position 0 outside
                // the calling function's 0 operands"* — an empty run, not a
                // wrong one.
                //
                // Written HERE rather than reassembled afterwards for the reason
                // the ordinary unit body states: one walk loads each slot once,
                // and a second pass that rebuilds the order is how the operand
                // run comes to disagree with the descriptor it was declared
                // from.
                compiler
                    .function_local
                    .defining_abi_operands
                    .push(LoweringOperand::Carried(CarriedBoundaryWord { word }));
                match slot.kind {
                    AbiSlotKind::Parameter => parameters.push(binding),
                    _ => captures.push(binding),
                }
            }

            // THE FUSION ITSELF: the producer's body is lowered THROUGH the
            // consumer's eliminator, in one pass, not to a value and then into
            // one.
            //
            // **Lowering the producer first and eliminating afterwards is
            // the defect, and it is the natural way to write this.** MEASURED:
            // that shape reaches the producer's own `"a computational recursor
            // closure names an in-flight activation, not a transferable value"`
            // refusal *inside the fused function* — the identical refusal fusion
            // exists to remove, merely relocated. The producer's body has no
            // value representation to hand anywhere; that it does not is the
            // whole reason the region is fused rather than called.
            //
            // ⇒ The eliminator goes ON THE STACK and the producer dispatcher
            // consumes it, so the producer's selected case body feeds the
            // consumer's suffix with no intermediate activation materialized.
            // This is the same entry the dispatcher's own nested arm uses to
            // fuse an inner eliminator ahead of an outer stack.
            //
            // **ONE authority spans BOTH, and that is provisional.** The bind
            // below is `Predeclared(producer_owner)`/`producer_owner` and it is
            // held across the whole combined lowering, so the consumer's case
            // bodies are lowered under the producer's source-lookup authority.
            // The ruled shape moves it
            // `Predeclared(producer) -> Predeclared(consumer) -> producer` across
            // the suffix; that switch, and the fused region's own checked-frame
            // adoption, are later Architect-ruled wiring and are **excluded from
            // this cut**. Fusing the two lowering steps is what removed the seam
            // the switch used to sit at, so it has to be reintroduced
            // deliberately rather than restored.
            let lowered = {
                let ambient =
                    AmbientBodyAuthority::bind(compiler, causal_owner, fusion.producer_owner);
                // `RT-LEXICAL-R3-FUSION-EMITTER` `D1` — arm the interior switch
                // for the extent of THIS region's body, and no longer.
                //
                // The key is the region's own `continuation_origin` and the fact
                // is its own `consumer_owner`; both come from the claim, so
                // nothing downstream infers either. Restored to whatever was
                // held on the way out rather than cleared, for the same reason
                // `AmbientBodyAuthority` restores: a nested definition pass must
                // not be handed `None` when its caller held a key.
                let enclosing_switch = compiler
                    .fused_consumer_authority
                    .replace((fusion.continuation_origin, fusion.consumer_owner));
                let lowered = fuse_producer_through_consumer_suffix(
                    compiler,
                    &mut builder,
                    fusion.producer_body,
                    fusion.continuation_origin,
                    fusion.checked_frame_id,
                    &parameters,
                    &captures,
                );
                compiler.fused_consumer_authority = enclosing_switch;
                ambient.release(compiler);
                lowered?
            };

            let lowered = compiler.eliminate_detached_producer_continuation(
                &mut builder,
                &result_edges,
                lowered,
                &captures,
            )?;
            let word = match lowered {
                LoweringOperand::Carried(word) => Some(word.word),
                LoweringOperand::Specialized(Lowered::Trap(trap)) => {
                    compiler.emit_current_trap(&mut builder, &trap)?;
                    None
                }
                LoweringOperand::Specialized(value) => Some(
                    compiler
                        .transfer_unit_result_into_carrier(
                            &mut builder,
                            fusion.continuation_origin,
                            &value,
                        )?
                        .word,
                ),
                // The fused-unit Result frame slot carries K.
                LoweringOperand::Residual(residual) => {
                    #[cfg(any(test, feature = "px8-ds-test-support"))]
                    record_residual_abi_decode();
                    Some(compiler.decode_residual_child(&mut builder, residual)?.word)
                }
            };
            if let Some(word) = word {
                builder.ins().store(
                    MemFlags::trusted(),
                    word,
                    frame,
                    i32::try_from(result_offset).map_err(|_| {
                        backend_module(
                            LoweringCause::Capacity,
                            "fused region result slot offset exceeds range".to_string(),
                        )
                    })?,
                );
            }
            let status = builder.ins().iconst(types::I64, 0);
            builder.ins().return_(&[status]);
            compiler.record_checked_frame_terminal(&builder, FrameTerminalKind::Normal)?;
            builder.seal_all_blocks();
            builder.finalize();
        }
        frame_scope.close(compiler, &func)?;
        compiler.verify_emitted_continuation_calls(&func, bundle)?;
        compiler.verify_recorded_composed_discharges(&func, bundle)?;
        #[cfg(test)]
        crate::cranelift_backend::lowering::record_d8j_discharged(
            compiler.function_local.composed_discharges.keys().cloned(),
        );
        if let Some(ledger) = compiler.continuation_claims.as_mut() {
            ledger.record_composed(
                compiler.function_local.composed_discharges.keys().cloned(),
                causal_owner,
            )?;
        }
        token_scope.close(compiler, &func)?;
        let response_owner_calls = verified_response_owner_calls(
            &compiler.static_transition_plan,
            bundle,
            compiler.function_local.continuation_emissions.keys().chain(
                compiler
                    .function_local
                    .checked_ih_transport_emissions
                    .iter()
                    .map(|(transport, _)| transport.source_call_identity()),
            ),
        )?;
        if let Some(ledger) = compiler.continuation_claims.as_mut() {
            ledger.record_emitted(
                compiler
                    .function_local
                    .continuation_emissions
                    .keys()
                    .cloned(),
            )?;
            ledger.record_response_owner_calls(response_owner_calls);
        }
        compiler.record_finished_grafted_spine_function(&func, bundle)?;
        verify_cranelift_function(&func, module.isa())?;
        compiler.commit_aggregate_events()?;
        if let Some(ledger) = compiler.fusion_claims.as_mut() {
            ledger.record_defined(fusion.id)?;
        }
        // ---- `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — `R` IS REALIZED HERE, and
        // ---- this body's emission is the whole of its realization.
        //
        // Architect `evt_6bm54j10w1n88`: the Outer consumer-binding identity
        // emits no call, reaches no call funnel and lowers no second selected
        // body. What realizes it is the fusion-owned body this loop just built,
        // whose selected case body IS the one that identity names -- the
        // planner closed that equality against the region claim before
        // installing the relation.
        //
        // ⛔ Recorded AFTER the body is verified and immediately before it is
        // defined, so a body that failed verification never claims to have
        // realized anything. ⛔ It is not a consumption: there is no call to
        // spend, and the record exists so a planned `R` whose owned body was
        // never emitted is refused at close rather than reading as success
        // because nothing happened.
        let realized = compiler
            .static_transition_plan
            .fusion_outer_realizations()
            .iter()
            .filter(|(_, realization)| realization.fusion() == fusion.id)
            .map(|(identity, realization)| (identity.clone(), realization.target()))
            .collect::<Vec<_>>();
        for (identity, target) in realized {
            compiler
                .fusion_compositions
                .as_mut()
                .ok_or_else(|| {
                    backend_module(
                        LoweringCause::PlannerInvariant,
                        "a fusion-owned body realized an outer identity with no composition \
                         ledger open to record it, so the realization would be unaccounted"
                            .to_string(),
                    )
                })?
                .record_outer_realized(&identity, fusion.id, target)?;
        }
        let mut ctx = module.make_context();
        std::mem::swap(&mut ctx.func, &mut func);
        module
            .define_function(id, &mut ctx)
            .map_err(|error| backend_module(LoweringCause::Backend, error.to_string()))?;
        defined += 1;
    }
    Ok(defined)
}

/// Lower the producer's body **through** the consumer's continuation occurrence.
///
/// Split out so the fused body reads as one step, and so the "the suffix must be
/// an elimination" refusal has exactly one site. It builds a **single-frame**
/// eliminator stack: this is the consumer's own suffix and nothing else, so
/// composing a second frame here would run a continuation the claim does not own.
///
/// The eliminator's environment is the **captures** and the producer's is the
/// **parameters**. They are two runs of one frame and are never concatenated:
/// the consumer's case bodies index its own continuation inputs, and handing
/// them the producer's parameter prefix would shift every one of those indices.
fn fuse_producer_through_consumer_suffix(
    compiler: &mut Lowering<'_>,
    builder: &mut FunctionBuilder<'_>,
    producer_body: StaticOriginId,
    continuation_origin: StaticOriginId,
    checked_frame_id: u64,
    parameters: &[LoweringEnvironmentBinding],
    captures: &[LoweringEnvironmentBinding],
) -> Result<LoweringOperand, CraneliftBackendError> {
    let suffix = compiler.retained_body_occurrence(continuation_origin)?;
    let RuntimeExpr::ComputationalMatch { cases, default, .. } = suffix.expr else {
        return Err(unsupported(
            LoweringCause::PlannerInvariant,
            "StaticContinuationFusion",
            "the claimed consumer continuation is not a computational match, so the fused region \
             has no suffix to run",
        ));
    };
    let body = compiler.retained_body_occurrence(producer_body)?;
    compiler.lower_fused_producer_through_suffix(
        builder,
        body,
        continuation_origin,
        checked_frame_id,
        cases,
        default,
        parameters,
        captures,
    )
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` — point every fused region's
/// redirected invocation at the fused `Function`, in the caller being defined.**
///
/// **Selection is by the CLAIM's own redirect edge and by the unit being
/// defined — nothing here searches.** The edge was chosen once, by
/// `fusion_redirect_target`, from the complete key, and validated by preflight
/// against the un-narrowed population. Re-deriving it here would search an edge
/// population that this region's own installation already narrowed, and could
/// not find the very edge whose supersession narrowed it.
///
/// **The frame contract installed is the FUSED region's, and that is not
/// optional.** The record drives the caller's stack-slot layout and its operand
/// walk; keeping the producer's frame while calling the fused body would write
/// the caller's operands at the wrong offsets. The `origin` field stays the
/// original edge's callee entry: it is the transfer coordinate the source
/// invocation already named, and the operand run it names is unchanged.
///
/// **`record_redirected` is affine and fires here**, so a second unit that
/// somehow claimed the same region refuses instead of silently installing a
/// second redirect.
fn redirect_fused_producer_invocations<M: Module>(
    module: &mut M,
    func: &mut Function,
    compiler: &mut Lowering<'_>,
    bundle: &UnitBundle,
    caller: PredeclaredFunctionId,
    unit_calls: &mut BTreeMap<StaticOriginId, DeclaredUnitCall>,
) -> Result<(), CraneliftBackendError> {
    // Own every fact before the ledger is borrowed mutably below.
    let redirects = {
        let Some(ledger) = compiler.fusion_claims.as_ref() else {
            return Ok(());
        };
        if ledger.is_empty() {
            return Ok(());
        }
        let contracts = compiler
            .static_transition_plan
            .continuation_fusions()?
            .into_iter()
            .map(|fusion| {
                let (offsets, _frame_bytes) = fusion.slot_offsets()?;
                Ok((
                    fusion.id(),
                    fusion.header(),
                    fusion.slots().to_vec(),
                    offsets,
                ))
            })
            .collect::<Result<Vec<_>, CraneliftBackendError>>()?;
        let mut redirects = Vec::new();
        for fusion in ledger.planned() {
            let Some(claim) = ledger.claim(*fusion) else {
                // Already consumed at its takeover. A claim is consumed exactly
                // once, and the takeover cannot run before the body it sits in
                // is defined, so reaching this in the declaration half of a
                // definition means the region was taken over by another seat.
                continue;
            };
            if claim.consumer_owner() != caller {
                continue;
            }
            let target = bundle.fusion(*fusion).ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "a claimed fused region has no forward-declared target to redirect to"
                        .to_string(),
                )
            })?;
            let (_, header, slots, offsets) = contracts
                .iter()
                .find(|(id, ..)| id == fusion)
                .ok_or_else(|| {
                    backend_module(
                        LoweringCause::PlannerInvariant,
                        "a claimed fused region has no installed frame contract".to_string(),
                    )
                })?;
            redirects.push((
                *fusion,
                claim.seat(),
                claim.redirect().callee_origin(),
                target,
                *header,
                slots.clone(),
                offsets.clone(),
            ));
        }
        redirects
    };
    for (fusion, seat, callee_origin, target, header, slots, offsets) in redirects {
        let call = DeclaredUnitCall {
            function: module.declare_func_in_func(target, func),
            frame_owner: Some(PendingVisFrameOwner::Fusion(fusion)),
            origin: callee_origin,
            call_site_origin: seat,
            header,
            slots,
            offsets,
            result_contract: None,
        };
        // The entry is INSERTED, and its absence beforehand is required rather
        // than assumed. Installing body ownership removed this seat's edge from
        // `executable_call_edges`, so an entry already standing here means the
        // producer is still reachable by its standalone route and the redirect
        // would be one of two answers to the same lookup.
        if unit_calls.insert(seat, call).is_some() {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "a fused region's redirected seat still holds a standalone producer call target, \
                 so the producer's body would remain reachable beside its fused definition"
                    .to_string(),
            ));
        }
        if let Some(ledger) = compiler.fusion_claims.as_mut() {
            ledger.record_redirected(fusion)?;
        }
    }
    Ok(())
}

pub(super) fn define_root_adapter<M: Module>(
    module: &mut M,
    compiler: &mut Lowering<'_>,
    helpers: ArtifactHelpers<'_>,
    bundle: &UnitBundle,
    adapter_id: FuncId,
    process_mode: bool,
    project_public_scalar_root: bool,
) -> Result<(), CraneliftBackendError> {
    compiler.open_aggregate_events(adapter_id)?;
    let root = compiler.static_transition_plan.root_emittable_unit()?;
    let root_id = bundle.function(root.function()).ok_or_else(|| {
        backend_module(
            LoweringCause::PlannerInvariant,
            "the recorded root unit was never forward-declared".to_string(),
        )
    })?;
    let (offsets, frame_bytes) = root.slot_offsets()?;
    if frame_bytes != root.header().frame_bytes {
        return Err(backend_module(
            LoweringCause::PlannerInvariant,
            "root adapter target frame size disagrees with its slot run".to_string(),
        ));
    }
    if process_mode {
        for role in [
            AbiProcessParameter::ProcessInput,
            AbiProcessParameter::Capability,
        ] {
            compiler
                .static_transition_plan
                .process_parameter_slot(role)?
                .ok_or_else(|| {
                    backend_module(
                        LoweringCause::PlannerInvariant,
                        "process root has no declared role-keyed ingress slot".to_string(),
                    )
                })?;
        }
    }

    let sig = unit_signature(module);
    let mut func = Function::with_name_signature(UserFuncName::user(0, adapter_id.as_u32()), sig);
    let mut function_local = helpers.declare_in_func(
        module,
        &mut func,
        Some(TrapExitAuthority::Root {
            process_sentinel: process_mode,
            source_authorized: false,
        }),
    );
    let root_origin = root.body_occurrence();
    function_local.unit_calls.insert(
        root_origin,
        DeclaredUnitCall {
            function: module.declare_func_in_func(root_id, &mut func),
            frame_owner: Some(PendingVisFrameOwner::Unit(root.function())),
            origin: root_origin,
            // The body occurrence, NOT the scheduling entry. They coincide
            // for an ordinary root and deliberately do not when the root body
            // schedules something before itself; `call_site_origin` is matched
            // against the `body_origin` the unit actually lowers, so naming the
            // entry here would disagree exactly on that case.
            call_site_origin: root_origin,
            header: root.header(),
            slots: root.slots().to_vec(),
            offsets,
            result_contract: None,
        },
    );

    let mut func_ctx = FunctionBuilderContext::new();
    {
        let mut builder = FunctionBuilder::new(&mut func, &mut func_ctx);
        let entry = builder.create_block();
        builder.append_block_params_for_function_params(entry);
        builder.switch_to_block(entry);
        let ingress = builder.block_params(entry)[0];
        let services = builder.block_params(entry)[1];
        let pointer_type = module.target_config().pointer_type();
        let native_int_arena = builder.ins().load(
            pointer_type,
            MemFlags::trusted(),
            services,
            crate::activation_services::SERVICES_NATIVE_INT_ARENA,
        );
        Lowering::require_nonzero(&mut builder, native_int_arena);
        let boundary_arena = builder.ins().load(
            pointer_type,
            MemFlags::trusted(),
            services,
            crate::activation_services::SERVICES_BOUNDARY_ARENA,
        );
        Lowering::require_nonzero(&mut builder, boundary_arena);
        function_local.services_pointer = Some(services);
        function_local.native_int_arena = Some(native_int_arena);
        function_local.boundary_arena = Some(boundary_arena);

        let mut inputs = Vec::new();
        if process_mode {
            let process_input = builder.ins().load(
                pointer_type,
                MemFlags::trusted(),
                ingress,
                crate::boundary_activation::ROOT_INGRESS_PROCESS_INPUT,
            );
            Lowering::require_nonzero(&mut builder, process_input);
            let host_dispatch_context = builder.ins().load(
                pointer_type,
                MemFlags::trusted(),
                ingress,
                crate::boundary_activation::ROOT_INGRESS_HOST_DISPATCH_CONTEXT,
            );
            Lowering::require_nonzero(&mut builder, host_dispatch_context);
            let capability = builder.ins().load(
                types::I64,
                MemFlags::trusted(),
                ingress,
                crate::boundary_activation::ROOT_INGRESS_CAPABILITY,
            );
            function_local.host_dispatch_context = Some(host_dispatch_context);
            inputs.push(LoweringOperand::Specialized(Lowered::BorrowedNativeValue {
                pointer: process_input,
            }));
            inputs.push(LoweringOperand::Specialized(Lowered::CapabilityToken {
                value: capability,
            }));
            #[cfg(test)]
            PROCESS_SLOT_MUTATION.with(|cell| match cell.get() {
                ProcessSlotMutation::Exact
                | ProcessSlotMutation::AttemptFixedContextOffsets
                | ProcessSlotMutation::ReintroduceLaunchIngress => {}
                ProcessSlotMutation::DeleteProcessInput => {
                    inputs.remove(0);
                }
                ProcessSlotMutation::DeleteCapability => {
                    inputs.pop();
                }
            });
        } else {
            function_local.host_dispatch_context = Some(builder.ins().iconst(pointer_type, 0));
        }

        compiler.function_local = function_local;
        let result = compiler.call_declared_unit(
            &mut builder,
            root_origin,
            &inputs,
            #[cfg(test)]
            Some(ingress),
        )?;
        let LoweringOperand::Carried(result) = result else {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "the internal root call did not return its result word".to_string(),
            ));
        };
        let public_result = if project_public_scalar_root {
            compiler.emit_public_carrier_scalar(&mut builder, result)?
        } else {
            result.word
        };
        builder.ins().return_(&[public_result]);
        builder.seal_all_blocks();
        builder.finalize();
    }
    verify_cranelift_function(&func, module.isa())?;
    compiler.commit_aggregate_events()?;
    #[cfg(test)]
    scale_b_record_functionized_root_adapter(&func);
    let mut ctx = module.make_context();
    std::mem::swap(&mut ctx.func, &mut func);
    module
        .define_function(adapter_id, &mut ctx)
        .map_err(|err| backend_module(LoweringCause::Backend, err.to_string()))
}

/// **`D2` — define every declared unit against its declared activation frame.**
///
/// ⛔ **Every dynamic value crosses into a unit through the declared
/// `AbiFrameHeader` + `AbiSlot` layout, never through capture-by-construction.**
/// The frame pointer is the unit's sole parameter; each slot is addressed at the
/// offset `B2R`'s own walk assigns it.
///
/// **MEASURED:** each emitted body addresses its slots at the offsets
/// `abi::slot_offsets` assigns, and reads its result from the slot `B2R` marks
/// `AbiSlotKind::Result`.
/// **CLAIMED:** the emitted code obeys the declared frame layout.
/// **THE GAP:** ⚠ this establishes *layout* agreement only. It says nothing
/// about the **ownership modes** (`AC-12`) or about whether every transfer into
/// a slot is **representable** (`AC-11`) — those are separate obligations with
/// their own controls, and ⛔ a body that addresses the right offset while
/// violating an ownership mode satisfies everything asserted here.
pub(super) struct RootUnitResult {
    pub(super) decoder: Option<ResultDecoder>,
    pub(super) trap: Option<RuntimeTrap>,
}

/// **`RT-CONTSPEC-ACTIVATE` `D3` — the artifact-wide claim set over exact
/// planned continuation-call tokens.**
///
/// Each projected causal identity is claimed during lowering by the exact
/// emission owner the token itself names. The separate
/// finished-Function token lattice checks path-wise affinity; this ledger
/// retains artifact-wide membership and the resolved target.
/// The causal token differs from `RT-WORKER-BIND`'s worker binding, and
/// nothing here changes that binding domain.
///
/// There is no lowering-minted arm token or second owner authority: the
/// defining unit is compared with the token's immutable `emission_owner`.
///
/// The key is the unchanged full `ContinuationCallIdentity`, including
/// `recursive_position`: weakening it would merge distinct causal tokens.
/// No traversal-order insertion rejection decides path-wise duplication.
pub(super) struct ContinuationClaimLedger {
    /// The RESOLVED target for each planned causal identity. Previously this
    /// kept only the keys and threw the `FuncId` away through `into_keys`,
    /// which is why no continuation target could ever be called: the join D1
    /// performed was discarded at the moment it became useful.
    resolved: BTreeMap<ContinuationCallIdentity, FuncId>,
    /// `None` until claimed; then the exact unit that claimed it, so owner
    /// agreement is a recorded fact rather than an inference.
    claims: BTreeMap<ContinuationCallIdentity, Option<ContinuationEmissionOwner>>,
    /// **`4b`** -- the PLANNED set, read straight off the plan's own causal call
    /// projection at open time and never derived from [`Self::resolved`].
    planned: BTreeSet<ContinuationCallIdentity>,
    /// **`4b`** -- every identity some generated function minted a `FuncRef`
    /// for, accumulated across all of them.
    declared: BTreeSet<ContinuationCallIdentity>,
    /// **`4b`** -- every identity a direct call was actually emitted for,
    /// accumulated across all generated functions after each one's CLIF has been
    /// checked.
    emitted: BTreeSet<ContinuationCallIdentity>,
    /// Every response-selected incoming edge whose direct call was decoded and
    /// matched after finished CLIF, including checked-IH transport calls that
    /// lawfully remain outside the ordinary direct/composed discharge partition.
    response_owner_calls: BTreeMap<ContinuationCallIdentity, usize>,
    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D8k`** -- every identity discharged by a
    /// VERIFIED composed source-continuation consumption, accumulated across
    /// every generated function after each one's CLIF has been checked.
    ///
    /// ⛔ Fed from `function_local.composed_discharges` and from nothing else.
    /// The direct instruction map is not a source of composed claims: its gate
    /// requires the recorded instruction to decode to `identity.target()`, and
    /// a composed instruction targets the raw worker, so an identity appearing
    /// in both would mean one of the two gates had been loosened.
    composed: BTreeSet<ContinuationCallIdentity>,
}

/// **`RT-CONTINUATION-EDGE-DISPOSITION` `D3` — the five mutations, one per
/// property the candidate/disposition layer promises.**
///
/// Each arms exactly one defect and must red for its OWN refusal. They are
/// deliberately separate variants rather than flags: a run arms at most one, so
/// a control cannot pass because some other mutation was still set.
///
/// **ONE declaration, not a `cfg(test)`/`cfg(not(test))` pair.** It was
/// authored as a pair, and the `cfg(not(test))` half carried only `None` while
/// the mutation *sites* in `core.rs` name all five variants unconditionally —
/// so the **production lib did not compile**, while the lib-**test** profile,
/// which is the only thing the seam checkpoint's `818/6/4` run built, compiled
/// perfectly. A production-only red no test profile can see.
///
/// ⇒ The variants are therefore declared once, for both profiles. What stays
/// `#[cfg(test)]` is the only thing that must: the **arming** — the
/// thread-local and its setter. In production `d3_mutation()` is a `const`
/// `None`, every site's comparison folds to `false`, and no mutation is
/// reachable by any caller.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D3Mutation {
    None,
    /// Withhold the static-worker binding the candidate authorizes.
    SuppressBindingInstallation,
    /// Settle `InlineNoCall` on bridge ENTRY rather than on a successful exit.
    MarkInlineBeforeBridgeCompletion,
    /// Drop the pending-composed half of the consumed test, so a candidate a
    /// composed call will claim is settled inline first.
    MarkInlineAfterComposedCall,
    /// Withhold the `DirectCall` settlement, leaving a candidate unsettled.
    OmitFinalDisposition,
    /// Settle one candidate twice.
    DoubleDisposition,
}

#[cfg(test)]
thread_local! {
    static D3_MUTATION: std::cell::Cell<D3Mutation> = const {
        std::cell::Cell::new(D3Mutation::None)
    };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn set_d3_mutation(mutation: D3Mutation) {
    D3_MUTATION.with(|cell| cell.set(mutation));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d3_mutation() -> D3Mutation {
    D3_MUTATION.with(std::cell::Cell::get)
}

/// **`RT-CONTINUATION-EDGE-DISPOSITION` `D3` — the seat a settlement was made
/// at.**
///
/// Two mutations (2 and 3) converge on the SAME terminal double-settlement
/// refusal, because they break the same invariant at different causal points.
/// The Architect's ruling is that a shared terminal string may corroborate both
/// rows but may not be either row's sole oracle. This enum is the
/// discriminator: it says **where** the offending settlement was made, which
/// the refusal text cannot.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D3Seat {
    /// The deferred bridge's ENTRY, before its scope has run.
    BridgeEntry,
    /// The deferred bridge's EXIT, after its scope completed.
    BridgeExit,
    /// The shared direct producer/call funnel.
    DirectFunnel,
    /// Finished-CLIF composed-discharge verification.
    ComposedPromotion,
}

/// **`RT-CONTINUATION-EDGE-DISPOSITION` `D3` — one ordered causal observation.**
///
/// Keyed by the live [`ContinuationCallIdentity`], so a row proves its
/// unmutated and armed arms reached the **same** seat for the **same** edge
/// rather than for two different ones that happen to look alike.
///
/// The two boolean halves on the bridge events are read **directly from the
/// ledger and the pending feed**, deliberately NOT through
/// `continuation_candidate_is_consumed`. That function is what mutation 3
/// mutates; routing the observation through it would make the instrument
/// inherit the defect it exists to detect, and the trace would agree with the
/// mutation instead of exposing it.
/// **`D3` — which environment binding an authorized position received.**
///
/// Mutation 1 substitutes one for the other, and the substitution is the whole
/// defect: the candidate still gets a binding, so a count sees nothing.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D3BindingKind {
    /// The capsule the candidate authorizes, callable at its recursive
    /// position.
    StaticWorker,
    /// A plain specialized value substituted in its place.
    Value,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D3Event {
    /// The bridge entered with this candidate bypassed.
    BridgeEntry {
        identity: ContinuationCallIdentity,
        settled: bool,
        pending_composed: bool,
    },
    /// A composed claim was RECORDED during lowering, which is strictly before
    /// finished-CLIF verification promotes it.
    ComposedRecorded { identity: ContinuationCallIdentity },
    /// **The binding seat** — which KIND of environment binding a composed
    /// candidate's authorized position actually received.
    ///
    /// Keyed by the identity the target's own coordinate selects, so row 1 can
    /// bind its downstream oracle to the same edge whose binding was
    /// suppressed rather than to "some binding somewhere in the program".
    BindingInstalled {
        identity: ContinuationCallIdentity,
        kind: D3BindingKind,
    },
    /// The shared direct funnel was REACHED and RETURNED an answer.
    ///
    /// Deliberately a separate event from [`Self::Settle`], and the
    /// separation was forced by a row that could not be written without it:
    /// row 4 withholds the settlement while preserving the call, so it must
    /// assert "the funnel returned AND nothing was settled". One event
    /// standing for both facts makes that sentence unstateable — the first
    /// draft overloaded them and row 4 failed against its own instrument.
    DirectFunnelReturned { identity: ContinuationCallIdentity },
    /// The bridge scope finished, and what it and the two feeds then said.
    BridgeExit {
        identity: ContinuationCallIdentity,
        completed: bool,
        settled: bool,
        pending_composed: bool,
    },
    /// A settlement was ATTEMPTED at a named seat. Recorded before the ledger
    /// call, so a conflicting settlement still leaves its seat in the trace.
    Settle {
        identity: ContinuationCallIdentity,
        disposition: CandidateDisposition,
        seat: D3Seat,
    },
}

#[cfg(test)]
impl D3Event {
    /// The live identity this observation is about.
    ///
    /// Clause 2 of the `AC-6` proof shape is that the unmutated and armed arms
    /// reach the mutation's seat **for the same derived identity**. Without an
    /// accessor a row can only compare event *kinds*, which two different
    /// edges would satisfy equally well.
    pub(in crate::cranelift_backend) fn identity(&self) -> &ContinuationCallIdentity {
        match self {
            Self::BridgeEntry { identity, .. }
            | Self::ComposedRecorded { identity }
            | Self::BindingInstalled { identity, .. }
            | Self::DirectFunnelReturned { identity }
            | Self::BridgeExit { identity, .. }
            | Self::Settle { identity, .. } => identity,
        }
    }

    /// The seat, when this observation is a settlement attempt.
    pub(in crate::cranelift_backend) fn settle_seat(&self) -> Option<D3Seat> {
        match self {
            Self::Settle { seat, .. } => Some(*seat),
            _ => None,
        }
    }
}

#[cfg(test)]
thread_local! {
    static D3_TRACE: std::cell::RefCell<Vec<D3Event>> =
        const { std::cell::RefCell::new(Vec::new()) };
    /// The candidate population the live plan projected at ledger open.
    static D3_PLAN_CANDIDATES: std::cell::RefCell<Vec<ContinuationCallIdentity>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

/// **The live plan's candidate population**, as `ContinuationCandidateLedger::
/// open` read it — typed identities, in plan order, not deduplicated by any
/// rendering.
#[cfg(test)]
pub(in crate::cranelift_backend) fn d3_plan_candidates() -> Vec<ContinuationCallIdentity> {
    D3_PLAN_CANDIDATES.with(|cell| cell.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d3_plan_candidates() {
    D3_PLAN_CANDIDATES.with(|cell| cell.borrow_mut().clear());
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d3_record(event: D3Event) {
    D3_TRACE.with(|cell| cell.borrow_mut().push(event));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d3_trace() -> Vec<D3Event> {
    D3_TRACE.with(|cell| cell.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d3_trace() {
    D3_TRACE.with(|cell| cell.borrow_mut().clear());
}

#[cfg(not(test))]
pub(in crate::cranelift_backend) fn d3_mutation() -> D3Mutation {
    D3Mutation::None
}

/// **`RT-CONTINUATION-EDGE-DISPOSITION` `D1` — what lowering settled a binding
/// candidate to.**
///
/// The three are settled at three different seats, each of which knows
/// something the other two do not, and none of them is a discharge on its own:
/// a discharge is what [`ContinuationClaimLedger`] records, and this enum sits
/// in front of it.
///
/// `InlineNoCall` is deliberately **not** a third arm of the discharge
/// partition. A third arm would let a program that made no call satisfy a law
/// whose entire purpose is to say a call was answered. It never enters the
/// equality, and `D2` derives the call-obligation subset from the other two.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum CandidateDisposition {
    /// The verified direct producer/call seat claimed and emitted.
    DirectCall,
    /// A raw-worker call was emitted AND passed finished-CLIF verification.
    ComposedCall,
    /// The exact deferred bridge scope completed successfully with this
    /// candidate still unconsumed.
    InlineNoCall,
    /// The planner issued an authorized checked-IH transport for this causal
    /// source, but no emitted runtime branch used it in this artifact. The edge
    /// has its own verifier and is not a causal-call obligation.
    TransportDormant,
}

/// **`RT-CONTINUATION-EDGE-DISPOSITION` `D1` — the binding-candidate ledger, a
/// SIBLING of the claim ledger on the same artifact lifetime.**
///
/// A candidate is minted by the planner and carries the exact worker provenance
/// and selector already present in [`ContinuationCallIdentity`] — deliberately
/// no new key, because a second key would be a second authority over which edge
/// is which.
///
/// **A candidate authorizes environment installation. It does not assert a
/// call.** That separation is the whole node: the same planner edge was
/// carrying both roles, and suppressing it to remove the obligation also
/// removed the binding.
///
/// This does not widen [`ContinuationClaimLedger`], add a per-owner close, or
/// traverse failed or non-selected compilations. It opens and closes in the
/// same two wrappers, beside the aggregate-allocation and effect-seat ledgers,
/// which is the established one-artifact-one-ledger idiom.
///
/// **The ordered closeout is `D2` and it has landed:** exact
/// one-disposition/disjointness first, then the derived
/// `DirectCall ∪ ComposedCall` subset, then the unchanged exact equality and
/// claim equality over that subset. An `InlineNoCall` member is therefore not
/// a call obligation and no longer reaches the closeout as an undischarged
/// token.
pub(super) struct ContinuationCandidateLedger {
    /// Every candidate the planner minted for this artifact.
    candidates: BTreeSet<ContinuationCallIdentity>,
    /// The disposition each settled candidate took, and the seat is the only
    /// writer of each variant.
    settled: BTreeMap<ContinuationCallIdentity, CandidateDisposition>,
    transport_candidates: BTreeSet<ContinuationCallIdentity>,
}

impl ContinuationCandidateLedger {
    fn open(plan: &StaticTransitionPlan<'_>) -> Result<Self, CraneliftBackendError> {
        // The SAME projection the claim ledger's `planned` is read from, so the
        // candidate population cannot drift from the population whose
        // obligations `D2` will derive out of it.
        // **`D3` — `O`, not `P`.** A fusion-local identity is settled in the
        // sibling composition ledger, never here, so admitting it as a candidate
        // would make totality-at-close demand a `CandidateDisposition` it must
        // not have. `evt_6kn9ckdnbf0ph` §2 forbids giving it one.
        let candidates = plan.ordinary_continuation_call_identities()?;
        // **`D3` — the candidate population AS THE LIVE PLAN PROJECTED IT.**
        //
        // Recorded here rather than rebuilt in a test, and the difference is
        // load-bearing: a test that re-plans its own witness proves a property
        // of the plan it just built, not of the plan the compile under
        // measurement actually used. Those can differ, and nothing would say
        // so. This is the projection `open` itself read.
        //
        // It is also a seat INDEPENDENT of every settlement seat the trace
        // observes, so an event carrying a wrong identity cannot match it by
        // construction -- the trace is not being validated against itself.
        #[cfg(test)]
        D3_PLAN_CANDIDATES.with(|cell| {
            *cell.borrow_mut() = candidates.iter().cloned().collect();
        });
        let transport_candidates = plan
            .checked_ih_environment_transport_source_identities()
            .intersection(&candidates)
            .cloned()
            .collect();
        Ok(Self {
            candidates,
            settled: BTreeMap::new(),
            transport_candidates,
        })
    }

    /// Settle one candidate to one artifact-wide disposition. Repeating an
    /// identical disposition is idempotent for exclusive paths; a conflicting
    /// disposition refuses here, naming both alternatives before closeout.
    pub(super) fn settle(
        &mut self,
        identity: &ContinuationCallIdentity,
        disposition: CandidateDisposition,
    ) -> Result<(), CraneliftBackendError> {
        if !self.candidates.contains(identity) {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "lowering settled a disposition for an identity the planner never minted a                  binding candidate for, so the candidate population and the settling seats                  disagree about which edges exist: {disposition:?}"
                ),
            ));
        }
        if let Some(existing) = self.settled.get(identity) {
            if *existing != disposition {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    format!(
                        "one binding candidate was settled twice, as {existing:?} and then as                      {disposition:?}; a candidate has exactly one disposition"
                    ),
                ));
            }
            // Two exclusive arms may settle the same disposition. The
            // function-local token lattice decides whether both can run.
            return Ok(());
        }
        self.settled.insert(identity.clone(), disposition);
        Ok(())
    }

    pub(super) fn is_settled(&self, identity: &ContinuationCallIdentity) -> bool {
        self.settled.contains_key(identity)
    }

    /// **`RT-CONTINUATION-EDGE-DISPOSITION` `D2` — totality first, then the
    /// derived call-obligation subset. The ORDER is the mechanism.**
    ///
    /// Disjointness is structural: [`Self::settle`] refuses a conflicting
    /// disposition, so a candidate cannot hold two forms and reach here. What is checked here is **totality** — that
    /// every minted candidate was settled by some seat.
    ///
    /// **Totality is checked BEFORE the subset is derived, and deriving
    /// first would defeat the whole check**: an unsettled candidate is in
    /// neither `DirectCall` nor `ComposedCall`, so it would simply fall out of
    /// the subset and pass silently — which is the exact failure the existing
    /// closeout refuses and this layer exists to make impossible.
    ///
    /// The domain needs no filtering. A candidate ledger only exists inside the
    /// selected `FunctionizedUnits` arm and only reaches this call on the
    /// success path, so plan-only rows and `Err` compilations are absent **by
    /// construction** rather than removed after the fact.
    fn close(mut self) -> Result<BTreeSet<ContinuationCallIdentity>, CraneliftBackendError> {
        for identity in &self.transport_candidates {
            self.settled
                .entry(identity.clone())
                .or_insert(CandidateDisposition::TransportDormant);
        }
        let unsettled = self
            .candidates
            .difference(&self.settled.keys().cloned().collect::<BTreeSet<_>>())
            .count();
        if unsettled > 0 {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "{unsettled} binding candidates reached the artifact closeout without a \
                 disposition; every candidate is settled exactly once by the seat that observed \
                 its event, so an unsettled candidate means a consumption path exists that no \
                 seat reports"
                ),
            ));
        }
        // The call-obligation subset. `InlineNoCall` is deliberately absent: it
        // is not a discharge and never enters the equality below.
        Ok(self
            .settled
            .into_iter()
            .filter(|(_, disposition)| match disposition {
                CandidateDisposition::DirectCall | CandidateDisposition::ComposedCall => true,
                CandidateDisposition::InlineNoCall | CandidateDisposition::TransportDormant => {
                    false
                }
            })
            .map(|(identity, _)| identity)
            .collect())
    }

    #[cfg(test)]
    pub(in crate::cranelift_backend) fn dispositions(
        &self,
    ) -> &BTreeMap<ContinuationCallIdentity, CandidateDisposition> {
        &self.settled
    }

    #[cfg(test)]
    pub(in crate::cranelift_backend) fn candidate_count(&self) -> usize {
        self.candidates.len()
    }
}

impl ContinuationClaimLedger {
    pub(super) fn open(
        plan: &StaticTransitionPlan<'_>,
        bundle: &UnitBundle,
    ) -> Result<Self, CraneliftBackendError> {
        let resolved = resolve_continuation_targets(plan, bundle)?;
        let claims = resolved
            .keys()
            .cloned()
            .map(|identity| (identity, None))
            .collect();
        // The PLANNED set, taken from the plan's causal call projection rather
        // than from `resolved`. ⚠ Honest note: `resolve_continuation_targets`
        // walks the same projection, so planned == resolved is structural today
        // and would only separate if resolution ever dropped or added a key. It
        // is recorded because `close()` asserts the four sets equal and a set
        // that is *implied* by another is not the same evidence as one that was
        // read independently -- the load-bearing pairs are declared and emitted.
        // **`D3` — `O`.** `close()` asserts `resolved = declared = planned` over
        // this set, and all three narrow together: a fusion-local identity is
        // absent from every one of them, so each law stays literally true over
        // its own complete domain rather than being widened to accept a gap.
        let planned = plan.ordinary_continuation_call_identities()?;
        Ok(Self {
            resolved,
            claims,
            planned,
            declared: BTreeSet::new(),
            emitted: BTreeSet::new(),
            response_owner_calls: BTreeMap::new(),
            composed: BTreeSet::new(),
        })
    }

    /// **`4b`** -- record the causal tokens one generated function minted call
    /// refs for.
    pub(super) fn record_declared(
        &mut self,
        declared: impl IntoIterator<Item = ContinuationCallIdentity>,
    ) -> Result<(), CraneliftBackendError> {
        for identity in declared {
            if !self.declared.insert(identity) {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    "a causal token was declared into more than one generated function".to_string(),
                ));
            }
        }
        Ok(())
    }

    /// **`4b`** -- record the causal tokens one generated function actually
    /// emitted a verified direct call for.
    ///
    /// ⛔ Called only *after* that function's CLIF has been checked, so a token
    /// reaches this set only once its emitted callee has been decoded from the
    /// instruction stream and matched against the planner-issued target.
    pub(super) fn record_emitted(
        &mut self,
        emitted: impl IntoIterator<Item = ContinuationCallIdentity>,
    ) -> Result<(), CraneliftBackendError> {
        for identity in emitted {
            if !self.emitted.insert(identity) {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    "a causal token emitted a direct call from more than one generated function"
                        .to_string(),
                ));
            }
        }
        Ok(())
    }

    fn record_response_owner_calls(
        &mut self,
        calls: impl IntoIterator<Item = ContinuationCallIdentity>,
    ) {
        for identity in calls {
            *self.response_owner_calls.entry(identity).or_default() += 1;
        }
    }

    /// **`D8k`** -- record the causal tokens one generated function discharged
    /// through a VERIFIED composed source-continuation consumption, and claim
    /// them.
    ///
    /// ⛔ Called only after `verify_recorded_composed_discharges` has promoted
    /// them, so an identity reaches this set only once its recorded raw-worker
    /// call was found in the finished CLIF, its decoded callee matched the
    /// `D8b`/`D8d` target, its operand run matched that target's declared run,
    /// and its result was shown to return downstream into the unchanged
    /// continuation.
    ///
    /// ⭐⭐ **The claim is made HERE and not at the seat, and that is what makes
    /// the partition disjoint.** A composed consumption claims the same
    /// `claims` slot a direct emission would, so an identity claimed both ways
    /// is rejected as a double claim rather than silently satisfying both
    /// halves of the union.
    pub(super) fn record_composed(
        &mut self,
        discharged: impl IntoIterator<Item = ContinuationCallIdentity>,
        defining: ContinuationEmissionOwner,
    ) -> Result<(), CraneliftBackendError> {
        for identity in discharged {
            if identity.emission_owner() != defining {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    "a composed source continuation was discharged by a function that is not its \
                     emission owner"
                        .to_string(),
                ));
            }
            let consumed = self.claims.get_mut(&identity).ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "a composed source continuation discharged a causal token this ledger never \
                     planned"
                        .to_string(),
                )
            })?;
            if let Some(previous) = consumed {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    format!(
                        "a causal token was claimed twice, first by {previous:?} and then by a \
                     composed source continuation; one causal obligation is discharged by one \
                     form, never by both"
                    ),
                ));
            }
            *consumed = Some(defining);
            if !self.composed.insert(identity) {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    "a causal token was discharged by a composed source continuation in more \
                     than one generated function"
                        .to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Declare THIS owning Function's own `FuncRef` for every causal token it
    /// owns, keyed by the complete four-field identity.
    ///
    /// Each `FuncRef` is minted into the `Function` passed here and belongs to
    /// it alone; ⛔ none is ever passed across functions. Declaring is not
    /// claiming -- a declared target is callable, and the affine claim happens
    /// later, at the exact producer occurrence.
    pub(super) fn declare_owned_in_func<M: Module>(
        &self,
        defining: ContinuationEmissionOwner,
        module: &mut M,
        func: &mut Function,
        plan: &StaticTransitionPlan<'_>,
    ) -> Result<BTreeMap<ContinuationCallIdentity, DeclaredUnitCall>, CraneliftBackendError> {
        // The declared target is the FULL contract, not a bare `FuncRef`:
        // `DeclaredUnitCall` already carries the function, the target origin,
        // and the projected header/slots/offsets, which is exactly what
        // `call_declared_unit_target` consumes. Reusing it is what keeps this
        // on the existing unit-call ABI instead of inventing a second one.
        let units = plan.continuation_units()?;
        let response_owners =
            plan.static_response_owner_specializations()?
                .map_err(|infeasible| {
                    backend_module(
                        LoweringCause::Fragment,
                        format!(
                            "compile-time response specialization is infeasible at {:?}: {}",
                            infeasible.vis_origin(),
                            infeasible.reason(),
                        ),
                    )
                })?;
        let transported = plan
            .checked_ih_environment_transports_owned_by(defining)
            .into_iter()
            .map(|transport| transport.source_call_identity().clone())
            .collect::<BTreeSet<_>>();
        self.resolved
            .iter()
            .filter(|(identity, _)| {
                identity.emission_owner() == defining || transported.contains(*identity)
            })
            .map(|(identity, target)| {
                let unit = units
                    .iter()
                    .find(|unit| unit.id() == identity.target())
                    .ok_or_else(|| {
                        backend_module(
                            LoweringCause::PlannerInvariant,
                            "a resolved causal identity names a specialization with no projected \
                             unit"
                                .to_string(),
                        )
                    })?;
                let (offsets, _frame_bytes) = unit.slot_offsets()?;
                // A selected continuation token calls the response OWNER, not
                // its original continuation specialization. The activation
                // tail belongs to that exact owner; the specialization has no
                // pending-Vis frame region. Read the same planner-selected
                // identity used to resolve the FuncId, never the destination
                // ABI slot or an emitter-local ordinal.
                let frame_owner = if let Some(owner) = response_owners
                    .iter()
                    .find(|owner| owner.selected_caller() == identity)
                {
                    if owner.header() != unit.header() || owner.slots() != unit.slots() {
                        return Err(backend_module(
                            LoweringCause::PlannerInvariant,
                            "a selected response owner changed its caller's activation ABI"
                                .to_string(),
                        ));
                    }
                    PendingVisFrameOwner::ResponseOwner(owner.response())
                } else {
                    PendingVisFrameOwner::Continuation(unit.id())
                };
                Ok((
                    identity.clone(),
                    DeclaredUnitCall {
                        function: module.declare_func_in_func(*target, func),
                        frame_owner: Some(frame_owner),
                        origin: unit.continuation_origin(),
                        call_site_origin: unit.continuation_origin(),
                        header: unit.header(),
                        slots: unit.slots().to_vec(),
                        offsets,
                        result_contract: plan
                            .static_response_forwarded_result_identity(identity)?
                            .map(DeclaredResultContract::new),
                    },
                ))
            })
            .collect()
    }

    /// Claim ONE exact token, at its producer occurrence.
    ///
    /// ⛔ This replaces a bulk discharge that consumed every token owned by a
    /// unit before any producer occurrence was reached. That was affine
    /// bookkeeping at the wrong seat: it could not tell a call that happened
    /// from one that did not, so it would have reported a clean ledger for a
    /// program that emitted no continuation call at all.
    ///
    /// Rejects an **absent** token and a **wrong owner**; duplicate claims
    /// are validated by the Function-local token lattice. The owner check is reachable
    /// here, because the caller supplies the unit currently being defined
    /// rather than the token's own owner being used to select it.
    ///
    /// ⚠ An affine red is not self-explaining: confirm the identity still
    /// carries all four fields including `recursive_position` before believing
    /// a double-consumption, because a collided key reports it against the
    /// right token.
    pub(super) fn claim_exact(
        &mut self,
        identity: &ContinuationCallIdentity,
        defining: ContinuationEmissionOwner,
    ) -> Result<FuncId, CraneliftBackendError> {
        if identity.emission_owner() != defining {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "a continuation call token was claimed by a context that is not its emission \
                 owner; note this compares the EMISSION owner, not the raw source-occurrence \
                 provenance owner beside it"
                    .to_string(),
            ));
        }
        let consumed = self.claims.get_mut(identity).ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "a continuation call token was claimed that this ledger never planned".to_string(),
            )
        })?;
        // The per-Function token lattice owns same-path duplicate claims (E1).
        // The whole-artifact set still records which planned identity was claimed.
        *consumed = Some(defining);
        self.resolved.get(identity).copied().ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "a claimed causal token has no resolved target".to_string(),
            )
        })
    }

    /// The closeout, over **two** populations since `D2`.
    ///
    /// `resolved` and `declared` are checked against the **full planned**
    /// population, because declaration is bulk over planned by `D8k`'s design.
    /// `discharged` and `claimed` are checked against **`call_obligations`**,
    /// the `DirectCall ∪ ComposedCall` subset the candidate ledger derives —
    /// so an `InlineNoCall` member is deliberately unclaimed and undischarged
    /// and is not an error, while an `InlineNoCall` member that WAS claimed
    /// still is.
    ///
    /// **Neither discharge set is bookkeeping.** An identity enters `emitted`
    /// only after its emitted callee was decoded from the finished CLIF and
    /// matched the planner-issued target, and enters `composed` only after a
    /// raw-worker call passed every clause of the finished-CLIF verification.
    ///
    /// **There is no equality against `emitted` alone.** The equality is over
    /// their **union** against `call_obligations`, and that is not a detail: a
    /// lawful `ComposedCall` obligation is answered by verified composed
    /// consumption and never becomes a direct call, so an "every obligation
    /// became one direct call" reading would exclude a legal member of the very
    /// representation this documents. Set equality says each obligation is
    /// answered in one of the two forms somewhere in the artifact, and no
    /// non-obligation is answered. The separate Function-local token lattice
    /// enforces at most one enter/receipt on each reachable runtime path.
    ///
    /// ⛔ Equality is asserted between sets, not between counts. Two sets of the
    /// same size can differ, and a length comparison here would pass for a
    /// population that swapped one token for another.
    /// `D2` — `call_obligations` is the derived `DirectCall ∪ ComposedCall`
    /// subset. **The discharge equality's FORM is unchanged**; only the set it
    /// ranges over is the derived subset rather than the full planned
    /// population, which is what lets a candidate that made no call stop being
    /// an obligation without any arm being added to the partition.
    ///
    /// **`resolved` and `declared` still range over the FULL planned set.**
    /// Declaration is bulk over planned by `D8k`'s own design, so narrowing
    /// those two would refuse every artifact with an `InlineNoCall` candidate
    /// for the opposite reason.
    /// Every response owner must be entered by its exact selected causal caller,
    /// and every response-issued context key must therefore have at least one
    /// such verified direct call. A forward declaration alone never satisfies
    /// this gate.
    fn validate_response_owner_call_coverage(
        &self,
        plan: &StaticTransitionPlan<'_>,
        dispositions: &BTreeMap<ContinuationCallIdentity, CandidateDisposition>,
        lowered_construct_origins: &BTreeSet<StaticOriginId>,
    ) -> Result<(), CraneliftBackendError> {
        let owners = plan
            .static_response_owner_specializations()?
            .map_err(|infeasible| {
                backend_module(
                    LoweringCause::Fragment,
                    format!(
                        "compile-time response specialization is infeasible at {:?}: {}",
                        infeasible.vis_origin(),
                        infeasible.reason(),
                    ),
                )
            })?;
        let response_rows =
            plan.static_response_feasibility_ledger_all()?
                .map_err(|infeasible| {
                    backend_module(
                        LoweringCause::Fragment,
                        format!(
                            "compile-time response specialization is infeasible at {:?}: {}",
                            infeasible.vis_origin(),
                            infeasible.reason(),
                        ),
                    )
                })?;
        let required_new_contexts = response_rows
            .iter()
            .filter(|row| !row.context_was_preexisting())
            .map(|row| row.k_context())
            .collect::<BTreeSet<_>>();
        let mut called_new_contexts = BTreeSet::new();
        for owner in owners {
            if !self
                .response_owner_calls
                .contains_key(owner.selected_caller())
            {
                let source = response_rows
                    .iter()
                    .find(|row| row.id() == owner.response())
                    .map(|row| row.vis_origin())
                    .ok_or_else(|| {
                        backend_module(
                            LoweringCause::PlannerInvariant,
                            "response owner has no installed response row".to_string(),
                        )
                    })?;
                // Only a relay-promoted owner may be unentered, and only when
                // no evaluator lowered its source Construct in ANY Function.
                // `disposition=None` alone proves neither of those facts.
                if !plan.has_pre_schema_relay_promotion(source, owner.selected_caller())
                    || lowered_construct_origins.contains(&source)
                {
                    #[cfg(feature = "px8-ds-test-support")]
                    if plan.has_pre_schema_relay_promotion(source, owner.selected_caller()) {
                        RELAY_OWNER_COVERAGE_REFUSALS.with(|slot| {
                            if let Some(rows) = slot.borrow_mut().as_mut() {
                                rows.push(RelayOwnerCoverageRefusalObservation {
                                    source: source.ticket_body_ordinal(),
                                    source_lowered: lowered_construct_origins.contains(&source),
                                    selected_caller: format!("{:?}", owner.selected_caller()),
                                });
                            }
                        });
                    }
                    return Err(backend_module(
                        LoweringCause::PlannerInvariant,
                        format!(
                            "a forward-declared response owner has no verified selected incoming call: \
                         owner={:?}, context={:?}, preexisting={}, caller={:?}, disposition={:?}, source_lowered={}",
                            owner.id(),
                            owner.k_context(),
                            owner.context_was_preexisting(),
                            owner.selected_caller(),
                            dispositions.get(owner.selected_caller()),
                            lowered_construct_origins.contains(&source),
                        ),
                    ));
                }
            }
            if self.composed.contains(owner.selected_caller()) {
                return Err(backend_module(LoweringCause::PlannerInvariant,
                    "a selected response caller was both directly retargeted and compositionally consumed"
                        .to_string(),
                ));
            }
            if !owner.context_was_preexisting() {
                called_new_contexts.insert(owner.k_context());
            }
        }
        if called_new_contexts != required_new_contexts {
            return Err(backend_module(LoweringCause::PlannerInvariant,
                "the response-issued context population is not covered by selected response-owner calls"
                    .to_string(),
            ));
        }
        Ok(())
    }

    pub(super) fn close(
        self,
        call_obligations: &BTreeSet<ContinuationCallIdentity>,
    ) -> Result<(), CraneliftBackendError> {
        // `D8k` -- DECLARATION may remain over the full planned set. An unused
        // declaration is a `FuncRef` nobody called, not an emitted call, so the
        // declared population stays equal to planned even where the discharge
        // took the composed form.
        for (name, set) in [
            (
                "resolved",
                self.resolved.keys().cloned().collect::<BTreeSet<_>>(),
            ),
            ("declared", self.declared.clone()),
        ] {
            if set != self.planned {
                let missing = self.planned.difference(&set).count();
                let extra = set.difference(&self.planned).count();
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    format!(
                        "the {name} continuation call population does not equal the planned one: \
                     {missing} planned tokens absent, {extra} unplanned tokens present"
                    ),
                ));
            }
        }
        // **`D8k` -- THE PARTITION**, since `D2` over the derived subset:
        // `call obligations = direct-emitted ⊎ composed-consumed`, asserted as
        // a disjoint union of two sets that
        // were accumulated from two different kinds of evidence: decoded direct
        // specialization emissions, and verified composed source-continuation
        // consumptions.
        //
        // ⛔ Not weakened to a count. Two sets of the right total size can still
        // be the wrong sets, and a program that emitted one token directly and
        // consumed a different one compositionally would satisfy any arithmetic
        // statement of this law.
        //
        // ⛔ Disjointness is asserted separately from coverage, because they
        // fail for different reasons: an overlap means one obligation was
        // answered twice, in two forms; a shortfall means one was never
        // answered at all.
        let both = self
            .emitted
            .intersection(&self.composed)
            .cloned()
            .collect::<BTreeSet<_>>();
        if !both.is_empty() {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "{} causal tokens were discharged BOTH by a decoded direct emission and by a \
                 verified composed consumption; the two forms partition the call-obligation \
                 population and an identity in both means one obligation was answered twice",
                    both.len()
                ),
            ));
        }
        let discharged = self
            .emitted
            .union(&self.composed)
            .cloned()
            .collect::<BTreeSet<_>>();
        // `D2` — the SAME equality, over the derived call-obligation subset.
        // `InlineNoCall` candidates are not in `call_obligations`, so they are
        // simply not obligations; nothing here treats them as discharged.
        if discharged != *call_obligations {
            let missing = call_obligations.difference(&discharged).count();
            let extra = discharged.difference(call_obligations).count();
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "the discharged continuation call population is not the call-obligation one: \
                 {missing} call obligations were neither directly emitted nor compositionally \
                 consumed, and {extra} discharged tokens are not call obligations. Direct: {}, \
                 composed: {}",
                    self.emitted.len(),
                    self.composed.len()
                ),
            ));
        }
        // `D2` — the claim law, as EXACT SET EQUALITY over the same derived
        // subset, not as a count of unclaimed slots.
        //
        // A count would have to be written as "ignore every unclaimed identity
        // outside the subset", and that hides the dual error: an `InlineNoCall`
        // candidate that was **accidentally claimed**. Equality catches both
        // directions and reports them separately, because they are opposite
        // defects — a missing obligation is a call nobody answered, an extra
        // claim is an inline non-call that answered for a call it never made.
        let claimed = self
            .claims
            .iter()
            .filter(|(_, consumed)| consumed.is_some())
            .map(|(identity, _)| identity.clone())
            .collect::<BTreeSet<_>>();
        if claimed != *call_obligations {
            let missing = call_obligations.difference(&claimed).count();
            let extra = claimed.difference(call_obligations).count();
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "the claimed continuation call population is not the call-obligation one: \
                 {missing} call obligations were never claimed by the unit that owns them, and \
                 {extra} claims were made for identities that are not call obligations"
                ),
            ));
        }
        // Owner agreement, asserted on the RECORDED consumer, over the
        // claimed/call-obligation members.
        //
        // Honest note: selection above is already by the token's own owner, so
        // this holds by construction today and cannot fire against the current
        // code. It is kept because it is the property `D3` actually promises,
        // and it is the check that would fire if selection were ever decoupled
        // from ownership.
        for (identity, consumed) in self
            .claims
            .iter()
            .filter(|(identity, _)| call_obligations.contains(*identity))
        {
            if *consumed != Some(identity.emission_owner()) {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    "a continuation call token was claimed by a unit that does not own it"
                        .to_string(),
                ));
            }
        }
        Ok(())
    }

    /// **`D3` — every identity this ordinary ledger has TOUCHED, in any role.**
    ///
    /// The union of claimed, declared, direct-emitted and composed-discharged.
    /// It exists for exactly one consumer: the fusion-local ledger's closeout,
    /// which asserts its own consumed population is disjoint from this one.
    ///
    /// **A union, not `planned`.** Asserting disjointness against `planned`
    /// would test the partition the planner already validated -- `O ∩ F = ∅` is
    /// checked at preflight and would be re-checked here against the same
    /// derivation. What has to be disjoint is what the two ledgers ACTUALLY
    /// recorded: a fusion-local identity that reached any ordinary role is a
    /// direct-plus-composed double realization, and it is that event, not the
    /// planner's arithmetic, that this makes visible.
    pub(super) fn touched_identities(&self) -> BTreeSet<ContinuationCallIdentity> {
        self.claims
            .iter()
            .filter(|(_, consumed)| consumed.is_some())
            .map(|(identity, _)| identity.clone())
            .chain(self.declared.iter().cloned())
            .chain(self.emitted.iter().cloned())
            .chain(self.composed.iter().cloned())
            .collect()
    }
}

/// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the SIBLING affine ledger for the
/// fusion-local realizations `F`.**
///
/// **A sibling, not a widening, and Architect `evt_6kn9ckdnbf0ph` is
/// explicit about why.** An ordinary identity's obligation is discharged by an
/// emitted call whose callee is decoded back out of the finished CLIF; a
/// fusion-local identity emits no call at all, so there is no instruction for
/// any such gate to read. Putting both in one ledger would mean either
/// weakening the direct laws to tolerate a member with no instruction, or
/// giving the composed member an instruction it does not have. Two ledgers over
/// two disjoint domains keeps each law literally true over its own complete
/// domain -- which is the same reason `O` was narrowed at the input rather than
/// the laws being relaxed.
///
/// **This is NOT the `FusionRegionClaimLedger` and does not touch it.** The
/// region claim carries the producer/consumer relation and is spent at the
/// redirect and takeover seats; consuming a composition here does not spend it,
/// and call `17/13` stays on the region claim and out of `dom(FusionComposedEdge)`
/// entirely.
///
/// ## The affine law, and what each half refuses
///
/// `dom(planned) = dom(consumed)`, with the target range equal to `F_t`:
///
/// - an identity with **no planned edge** is refused -- a composition seat was
///   reached for something the planner never composed;
/// - a **second** consumption of one identity is refused, whether in the same
///   function or another one, which is the replay case;
/// - a consumption whose **owner or target** disagrees with the planned edge is
///   refused, each named separately so the refusal says which fact moved;
/// - at close, each fusion's CONSUMED layers must be exactly one `Outer` and one
/// `Inner`. The layer is deliberately **not** checked at consumption: the
///   seat has no authority over it independent of the edge itself, so a
///   per-consumption comparison would read the edge's layer and compare it with
///   itself. What is real is the whole-artifact statement that both ruled layers
///   of each fusion were actually realized rather than merely planned, and that
///   is asserted where that population exists;
/// - a planned member **never consumed** is refused at close -- the case that
///   reads as success because nothing happened;
/// - a consumed identity that **also reached the ordinary ledger** in any role
///   is refused, which is the direct-plus-composed double realization.
/// **`D3` — which fusion class a specialization target belongs to.**
///
/// The omission sets are tagged with it so `I_t` and `R_t` close SEPARATELY.
/// One merged set is satisfied by either class alone, and the two omissions
/// fail differently: an undeclared `I_t` leaves a body nothing lowers locally,
/// while an undeclared `R_t` leaves one the fusion-owned body already emits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum FusionTargetClass {
    /// `I` — locally composed at its own call edge.
    LocallyComposed,
    /// `R` — realized by the fusion-owned body.
    FusionOwned,
}

fn fusion_target_classes(
    inner: &BTreeMap<ContinuationCallIdentity, FusionComposedEdge>,
    outer: &BTreeMap<ContinuationCallIdentity, FusionOwnedOuterRealization>,
) -> BTreeMap<ContinuationSpecializationId, FusionTargetClass> {
    inner
        .values()
        .map(|edge| (edge.target(), FusionTargetClass::LocallyComposed))
        .chain(
            outer
                .values()
                .map(|realization| (realization.target(), FusionTargetClass::FusionOwned)),
        )
        .collect()
}

pub(super) struct FusionCompositionLedger {
    /// `F` — the planned fusion-local realizations, keyed by exact identity.
    ///
    /// The WHOLE composed edge is retained, not the fields a consumption
    /// happens to check. A consumption is validated against the planner's own
    /// record; re-deriving any of owner, layer or target here would make this
    /// ledger a second authority over the composition it is supposed to audit.
    planned: BTreeMap<ContinuationCallIdentity, FusionComposedEdge>,
    /// `R` — the fusion-owned outer realizations. Held here for its TARGET
    /// RANGE and its planned-versus-realized equality only; the affine lifetime
    /// of an `R` identity belongs to the region claim and the owned body, and
    /// nothing here consumes one.
    planned_outer: BTreeMap<ContinuationCallIdentity, FusionOwnedOuterRealization>,
    /// `None` until the fusion-owned body realized it; then that body's fusion.
    realized_outer: BTreeMap<ContinuationCallIdentity, Option<StaticContinuationFusionId>>,
    /// `None` until consumed; then the emission owner that consumed it.
    consumed: BTreeMap<ContinuationCallIdentity, Option<ContinuationEmissionOwner>>,
    /// The specialization targets the DECLARATION pass actually omitted, read
    /// from that pass's own output rather than re-derived.
    ///
    /// Not derived from `planned` at close. Deriving it would compare the
    /// planner's `F_t` with itself and pass for a pass that omitted nothing --
    /// which is precisely the failure this range equality exists to catch.
    declaration_omitted: BTreeMap<ContinuationSpecializationId, FusionTargetClass>,
    /// The specialization targets the DEFINITION pass actually omitted.
    ///
    /// **Kept separate from the declaration set, and both are closed against
    /// `F_t`.** The two passes fail differently: a declaration that does not
    /// omit leaves an undefined phantom symbol, and a definition that does not
    /// omit emits a standalone `Function` for a body already lowered locally --
    /// a second realization of one edge. One merged set would be satisfied by
    /// either pass alone.
    definition_omitted: BTreeMap<ContinuationSpecializationId, FusionTargetClass>,
}

impl FusionCompositionLedger {
    /// Open over the planner's composed-edge relation, once per artifact.
    ///
    /// The declaration omission is seeded from the BUNDLE -- the declaration
    /// pass's own output -- rather than from the plan. That is the whole
    /// point: a `declare_unit_bundle` that stopped filtering would leave this
    /// set empty and the closeout would say so, where a plan-derived set would
    /// agree with the plan no matter what was declared.
    pub(super) fn open(
        plan: &StaticTransitionPlan<'_>,
        bundle: &UnitBundle,
    ) -> Result<Self, CraneliftBackendError> {
        let planned = plan.fusion_composed_edges().clone();
        let planned_outer = plan.fusion_outer_realizations().clone();
        let consumed = planned
            .keys()
            .cloned()
            .map(|identity| (identity, None))
            .collect();
        let realized_outer = planned_outer
            .keys()
            .cloned()
            .map(|identity| (identity, None))
            .collect();
        let classes = fusion_target_classes(&planned, &planned_outer);
        let declaration_omitted = plan
            .continuation_units()?
            .iter()
            .map(|unit| unit.id())
            .filter(|id| bundle.continuation(*id).is_none())
            .filter_map(|id| classes.get(&id).map(|class| (id, *class)))
            .collect();
        Ok(Self {
            planned,
            planned_outer,
            consumed,
            realized_outer,
            declaration_omitted,
            definition_omitted: BTreeMap::new(),
        })
    }

    /// Record that the DEFINITION pass omitted this specialization's body.
    ///
    /// Recorded where the omission happens, in the loop that would otherwise
    /// have emitted the `Function`, so the evidence is the pass's own decision
    /// rather than a statement about it made elsewhere.
    pub(super) fn record_definition_omitted(&mut self, target: ContinuationSpecializationId) {
        let classes = fusion_target_classes(&self.planned, &self.planned_outer);
        if let Some(class) = classes.get(&target) {
            self.definition_omitted.insert(target, *class);
        }
    }

    /// **`D3` — `R` was realized by the fusion-owned body that emits it.**
    ///
    /// ⛔ Not a consumption and not affine against `consumed`: an `R` identity
    /// never reaches a call seat, so there is nothing here to spend. What this
    /// records is that the owned body which realizes it was actually emitted,
    /// so a planned `R` with no emitted body is refused at close rather than
    /// reading as success because nothing happened.
    pub(super) fn record_outer_realized(
        &mut self,
        identity: &ContinuationCallIdentity,
        fusion: StaticContinuationFusionId,
        target: ContinuationSpecializationId,
    ) -> Result<(), CraneliftBackendError> {
        let planned = self.planned_outer.get(identity).ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "a fusion-owned body realized an outer identity the planner never recorded, so \
                 the realization is not one the R relation authorized"
                    .to_string(),
            )
        })?;
        if planned.fusion() != fusion {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "a fusion-owned outer identity planned for {:?} was realized by {fusion:?}; one \
                 region realizes its own outer identity and never another's",
                    planned.fusion()
                ),
            ));
        }
        if planned.target() != target {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "a fusion-owned outer identity planned for target {:?} was realized against \
                 {target:?}, so the body emitted is not the one the identity names",
                    planned.target()
                ),
            ));
        }
        let realized = self.realized_outer.get_mut(identity).ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "a planned outer realization has no realization slot; the two maps are built from \
                 one population and disagreeing means one was rebuilt"
                    .to_string(),
            )
        })?;
        if let Some(previous) = realized {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "a fusion-owned outer identity was realized twice, first by {previous:?}; one \
                 owned body emits its selected body once"
                ),
            ));
        }
        *realized = Some(fusion);
        Ok(())
    }

    /// Consume ONE fusion-local composition, at its exact call edge.
    ///
    /// Each fact is checked against the planned edge's own record with its own
    /// refusal, so a red names the fact that moved rather than reporting a
    /// generic mismatch.
    ///
    /// `target` is the CALL IDENTITY's own recorded target, supplied by the
    /// seat, not the edge's. The two are separate planner facts about one call,
    /// so this is a real comparison; passing the edge's own target back in would
    /// be an identity.
    pub(super) fn consume(
        &mut self,
        identity: &ContinuationCallIdentity,
        defining: ContinuationEmissionOwner,
        target: ContinuationSpecializationId,
    ) -> Result<(StaticContinuationFusionId, FusionCompositionLayer), CraneliftBackendError> {
        let edge = self.planned.get(identity).ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "a fusion-local composition was consumed for an identity the planner never \
                 composed; a composition seat reached for an unplanned identity is a lowering \
                 that decided for itself which edges are local"
                    .to_string(),
            )
        })?;
        if edge.emission_owner() != defining {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "a fusion-local composition planned for emission owner {:?} was consumed while \
                 defining {defining:?}; the composition is lowered into the consumer's own \
                 function and a foreign one would place the producer's body in a frame that \
                 never held its operands",
                    edge.emission_owner()
                ),
            ));
        }
        if edge.target() != target {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "a fusion-local composition planned for target {:?} was consumed against {target:?}; \
                 the composed edge names the exact specialization whose selected body is lowered \
                 locally, and another one is another body",
                    edge.target()
                ),
            ));
        }
        let fusion = edge.fusion();
        let layer = edge.layer();
        let consumed = self.consumed.get_mut(identity).ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "a fusion-local composition has a planned edge but no consumption slot; the two \
                 maps are built from one population and disagreeing means one was rebuilt"
                    .to_string(),
            )
        })?;
        if let Some(previous) = consumed {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "a fusion-local composition was consumed twice, first by {previous:?}; one \
                 composed call edge is realized once, and a second consumption is a replay \
                 whether it happens in this function or another"
                ),
            ));
        }
        *consumed = Some(defining);
        Ok((fusion, layer))
    }

    /// The closeout, over three populations.
    ///
    /// Every comparison is between SETS. Two populations of the same size can
    /// be the wrong two, and a count would pass for a pass that consumed one
    /// composition and omitted a different target.
    pub(super) fn close(
        self,
        ordinary_touched: &BTreeSet<ContinuationCallIdentity>,
    ) -> Result<(), CraneliftBackendError> {
        let planned = self.planned.keys().cloned().collect::<BTreeSet<_>>();
        let consumed = self
            .consumed
            .iter()
            .filter(|(_, consumed)| consumed.is_some())
            .map(|(identity, _)| identity.clone())
            .collect::<BTreeSet<_>>();
        if consumed != planned {
            let missing = planned.difference(&consumed).count();
            let extra = consumed.difference(&planned).count();
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "the consumed fusion-local composition population is not the planned one: \
                 {missing} planned compositions were never realized, and {extra} realizations \
                 name identities that were never planned. An unrealized composition is the case \
                 that reads as success because nothing was emitted for it"
                ),
            ));
        }
        // `I_t` and `R_t` — the two target ranges, closed SEPARATELY against
        // what each pass actually omitted, so one class cannot satisfy the
        // other's omission.
        let inner_targets = self
            .planned
            .values()
            .map(FusionComposedEdge::target)
            .collect::<BTreeSet<_>>();
        let outer_targets = self
            .planned_outer
            .values()
            .map(FusionOwnedOuterRealization::target)
            .collect::<BTreeSet<_>>();
        for (pass, omitted) in [
            ("declaration", &self.declaration_omitted),
            ("definition", &self.definition_omitted),
        ] {
            for (class, expected, name) in [
                (FusionTargetClass::LocallyComposed, &inner_targets, "I_t"),
                (FusionTargetClass::FusionOwned, &outer_targets, "R_t"),
            ] {
                let actual = omitted
                    .iter()
                    .filter(|(_, tag)| **tag == class)
                    .map(|(target, _)| *target)
                    .collect::<BTreeSet<_>>();
                if actual != *expected {
                    let missing = expected.difference(&actual).count();
                    let extra = actual.difference(expected).count();
                    return Err(backend_module(
                        LoweringCause::PlannerInvariant,
                        format!(
                            "the {pass} pass's omitted {name} population is not the planned one: \
                         {missing} were still {pass}ed, and {extra} were omitted without \
                         belonging to that class. A fusion-local target that keeps its standalone \
                         Function is a second realization of one edge"
                        ),
                    ));
                }
            }
        }
        // EXACTLY ONE CONSUMED `Inner` PER FUSION.
        //
        // ⚠ **Restated, not relaxed.** This row previously required `{Outer,
        // Inner}` per fusion, which was correct while both layers were composed
        // edges. Under `P = O ⊎ I ⊎ R` the `Outer` layer is NOT in this ledger
        // at all -- it is realized by the fusion-owned body -- so demanding it
        // here would demand a consumption that must never happen.
        let mut inner_per_fusion: BTreeMap<StaticContinuationFusionId, usize> = BTreeMap::new();
        for identity in &consumed {
            let edge = self.planned.get(identity).ok_or_else(|| {
                backend_module(
                    LoweringCause::PlannerInvariant,
                    "a consumed fusion-local composition has no planned edge at close; \
                     consumption cannot record an identity the planned map does not hold"
                        .to_string(),
                )
            })?;
            if edge.layer() != FusionCompositionLayer::Inner {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    "a composition ledger member is not at the Inner layer; the Outer layer is \
                     realized by the fusion-owned body and must never be consumed as a local \
                     composition"
                        .to_string(),
                ));
            }
            *inner_per_fusion.entry(edge.fusion()).or_default() += 1;
        }
        for (fusion, count) in &inner_per_fusion {
            if *count != 1 {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    format!(
                        "static continuation fusion {fusion:?} consumed {count} local compositions; \
                     exactly one Inner identity per fusion is composed at its call edge"
                    ),
                ));
            }
        }

        // `R` — planned versus REALIZED, so an outer identity whose owned body
        // was never emitted is refused rather than reading as success.
        let planned_outer = self.planned_outer.keys().cloned().collect::<BTreeSet<_>>();
        let realized_outer = self
            .realized_outer
            .iter()
            .filter(|(_, realized)| realized.is_some())
            .map(|(identity, _)| identity.clone())
            .collect::<BTreeSet<_>>();
        if realized_outer != planned_outer {
            let missing = planned_outer.difference(&realized_outer).count();
            let extra = realized_outer.difference(&planned_outer).count();
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "the realized fusion-owned outer population is not the planned one: {missing} \
                 planned outer identities were never realized by an emitted owned body, and \
                 {extra} realizations name identities that were never planned"
                ),
            ));
        }

        // THE DISJOINTNESS, against what the ordinary ledger RECORDED.
        //
        // Not against `planned`, which the planner's own partition already
        // settles. What this catches is an identity that was composed locally
        // AND reached an ordinary role -- claimed, declared, directly emitted or
        // composed-discharged -- which is one call edge realized twice.
        let both = consumed
            .union(&realized_outer)
            .cloned()
            .collect::<BTreeSet<_>>()
            .intersection(ordinary_touched)
            .count();
        if both > 0 {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                format!(
                    "{both} continuation call identities were realized both as a fusion-local \
                 composition and in an ordinary role; one call edge is realized exactly once, \
                 and an identity in both ledgers has been emitted twice under one obligation"
                ),
            ));
        }
        Ok(())
    }
}

/// **`RT-DECL-CLOSURE-PORT` `D5a` checkpoint 2 — open the ONE cross-pass causal
/// ledger.**
///
/// Called from the orchestration before the first generated `Function`, so
/// every pass that can declare, claim or emit a causal call accumulates into
/// this single object.
///
/// ⛔ There is exactly one of these and it must not be re-opened. A second open
/// would discard whatever the first had accumulated and leave a partial
/// equality reading as a global one, which is the failure mode checkpoint 2
/// exists to remove — so re-opening rejects rather than replacing.
/// **`D1`** — the disposition tally from the most recent artifact close, so a
/// witness can assert WHICH disposition its candidate settled to without the
/// ledger outliving the artifact it belongs to.
#[cfg(test)]
thread_local! {
    static D1_LAST_DISPOSITIONS: std::cell::RefCell<
        std::collections::BTreeMap<CandidateDisposition, usize>,
    > = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d1_last_dispositions()
-> std::collections::BTreeMap<CandidateDisposition, usize> {
    D1_LAST_DISPOSITIONS.with(|cell| cell.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d1_dispositions() {
    D1_LAST_DISPOSITIONS.with(|cell| cell.borrow_mut().clear());
}

pub(super) fn open_continuation_claim_ledger(
    compiler: &mut Lowering<'_>,
    bundle: &UnitBundle,
) -> Result<(), CraneliftBackendError> {
    if compiler.continuation_claims.is_some() {
        return Err(backend_module(LoweringCause::PlannerInvariant,
            "the continuation claim ledger is already open; one artifact has exactly one ledger              and re-opening would silently discard every token recorded so far"
                .to_string(),
        ));
    }
    compiler.continuation_claims = Some(ContinuationClaimLedger::open(
        &compiler.static_transition_plan,
        bundle,
    )?);
    // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the fusion-local sibling opens on
    // the SAME boundary and for the same reason: one artifact has exactly one
    // of it, and every composition seat consumes into it. Sharing the lifetime
    // is what makes the two ledgers siblings over disjoint domains rather than
    // one ledger with a second kind of member.
    compiler.fusion_compositions = Some(FusionCompositionLedger::open(
        &compiler.static_transition_plan,
        bundle,
    )?);
    // `RT-CONTINUATION-EDGE-DISPOSITION` `D1` — the candidate ledger opens on
    // the SAME boundary and for the same reason the two below do: one artifact
    // has exactly one of it, and every pass settles into it. Sharing the
    // lifetime is what makes it a sibling in front of the claim ledger rather
    // than a widening of it.
    compiler.continuation_candidates = Some(ContinuationCandidateLedger::open(
        &compiler.static_transition_plan,
    )?);
    // `D7` — the aggregate allocation relation opens on the same boundary and
    // for the same reason: one artifact has exactly one relation, and every
    // body's events commit into it.
    compiler.aggregate_allocations = Some(AggregateAllocationLedger::default());
    // `D7` — the host-effect seat ledger opens on the same boundary: one
    // artifact has one consumed-seat evidence, and every body claims into it.
    compiler.host_effect_seats = Some(EffectSeatLedger::default());
    Ok(())
}

/// **`D5a` checkpoint 2 — close it, once, after every generated `Function`.**
///
/// Since `D2`, two global laws rather than one: `resolved = declared =
/// planned` over the full planner population, and `discharged = claimed =
/// call_obligations` over the derived `DirectCall ∪ ComposedCall` subset.
///
/// **Not a per-pass partial:** a pass that discharges nothing is normal, and
/// only the whole-artifact sets answer whether **every call obligation** was
/// answered exactly once. Deliberately not "every planned token": an
/// `InlineNoCall` candidate is planned and is lawfully never discharged, which
/// is the whole point of the derived subset.
pub(super) fn close_continuation_claim_ledger(
    compiler: &mut Lowering<'_>,
) -> Result<(), CraneliftBackendError> {
    // The candidate ledger is taken on the same boundary as the claim ledger,
    // and `D2` closes it FIRST: totality, then the derived subset, then the
    // claim ledger's exact laws over that subset.
    let candidates = compiler.continuation_candidates.take().ok_or_else(|| {
        backend_module(
            LoweringCause::PlannerInvariant,
            "the continuation candidate ledger went missing".to_string(),
        )
    })?;
    #[cfg(test)]
    D1_LAST_DISPOSITIONS.with(|cell| {
        *cell.borrow_mut() = candidates.dispositions().values().copied().fold(
            std::collections::BTreeMap::new(),
            |mut acc, d| {
                *acc.entry(d).or_insert(0usize) += 1;
                acc
            },
        );
    });
    compiler
        .continuation_claims
        .as_ref()
        .ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "the continuation claim ledger went missing".to_string(),
            )
        })?
        .validate_response_owner_call_coverage(
            &compiler.static_transition_plan,
            &candidates.settled,
            &compiler.lowered_construct_origins,
        )?;
    // `D2` — the order, and it is the mechanism rather than a style choice.
    // Totality and disjointness FIRST, then the derived subset, then the
    // unchanged equality over it.
    let call_obligations = candidates.close()?;
    // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the fusion-local sibling closes
    // BEFORE the ordinary ledger is consumed, because its disjointness clause
    // reads what that ledger RECORDED and `close` takes it by value.
    //
    // The order also decides which red a reader sees first, and this is the
    // useful one: a fusion-local composition that was never realized, or one
    // realized twice, explains an ordinary population that then looks short. The
    // reverse order reports the symptom and consumes the evidence.
    let ordinary_touched = compiler
        .continuation_claims
        .as_ref()
        .ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "the continuation claim ledger went missing".to_string(),
            )
        })?
        .touched_identities();
    compiler
        .fusion_compositions
        .take()
        .ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "the fusion-local composition ledger went missing".to_string(),
            )
        })?
        .close(&ordinary_touched)?;
    compiler
        .continuation_claims
        .take()
        .ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "the continuation claim ledger went missing".to_string(),
            )
        })?
        .close(&call_obligations)
}

/// **`D7` — close the aggregate allocation relation once, over the whole
/// compilation.**
///
/// ⛔ Not a per-body partial. One planner record may govern events in several
/// bodies -- a synthesized role at a seat reached under both a predeclared unit
/// and a generated specialization allocates in both -- so `image(R_f) = P` is
/// false for every individual body and imposing it would refuse lawful
/// programs. Only the whole-artifact relation answers whether every planned
/// record was allocated and every event was planned.
pub(super) fn close_aggregate_allocation_ledger(
    compiler: &mut Lowering<'_>,
) -> Result<AggregateRelationClosure, CraneliftBackendError> {
    let planned = compiler
        .static_transition_plan
        .aggregate_ownership_records();
    compiler
        .aggregate_allocations
        .take()
        .ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "the aggregate allocation ledger went missing".to_string(),
            )
        })?
        .close(planned)
}

/// **`D7` — close the host-effect seat authority once, over the whole
/// compilation.**
///
/// ⛔ Whole-artifact rather than per-body, because a seat inside a worker body
/// is consumed in its predeclared unit and again in each specialization that
/// contains it; no single body's claims equal the population.
///
/// ⭐ **`image(claims) ⊆ P`, exactly as the aggregate relation states it — NOT
/// an equality.** This comment previously claimed the opposite, on the reasoning
/// that a seat population derived from the source's own `Effect` occurrences
/// must be fully reached. That was measured false: an occurrence sitting in a
/// declaration body a compilation never emits takes its seats with it, and
/// requiring equality refused such a program. `P` authorizes; it does not
/// oblige, and an unreached member is lawful and reported. A half-read
/// occurrence cannot hide in that gap, because completeness is a group-local
/// equality that has already run at each visit's close.
pub(super) fn close_host_effect_seat_ledger(
    compiler: &mut Lowering<'_>,
) -> Result<EffectSeatClosure, CraneliftBackendError> {
    let planned = compiler
        .static_transition_plan
        .host_effect_seat_records()
        .to_vec();
    #[cfg(test)]
    if effect_seat_visit_mutation() == EffectSeatVisitMutation::DropCommittedGroupBeforeGlobalClose
    {
        if let Some(ledger) = compiler.host_effect_seats.as_mut() {
            ledger.drop_one_committed_group_for_tests();
        }
    }
    let closure = compiler
        .host_effect_seats
        .take()
        .ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "the host effect seat ledger went missing".to_string(),
            )
        })?
        .close(&planned)?;
    #[cfg(test)]
    LAST_EFFECT_SEAT_CLOSURE.with(|cell| *cell.borrow_mut() = Some(closure.clone()));
    Ok(closure)
}

/// What the last completed seat closeout on this thread measured.
///
/// ⚠ Like `b2f_last_unit_emission`, this carries no statement about WHICH
/// compile produced it: a compile that fails before the closeout leaves the
/// previous reading standing. Read it only where one compile is known to have
/// closed.
#[cfg(test)]
thread_local! {
    static LAST_EFFECT_SEAT_CLOSURE: std::cell::RefCell<Option<EffectSeatClosure>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn last_effect_seat_closure() -> Option<EffectSeatClosure> {
    LAST_EFFECT_SEAT_CLOSURE.with(|cell| cell.borrow().clone())
}

/// How the `BufferAllocate` capacity seat was DISPATCHED, as `(specialized,
/// carried)`, since the last reset on this thread.
///
/// ⭐ **The premise a carried-capacity control cannot do without.** "The
/// compile succeeded and returned `InvalidBounds`" is the same green whether
/// the capacity took the carried route or the specialized one, so a fixture
/// that quietly stops carrying its capacity would leave the carried arm
/// untested and every assertion about it still passing. This is the only
/// instrument that separates those two worlds.
///
/// ⚠ It counts EMISSIONS, not executions: one compiled arm may run many times
/// or none. A control that wants "the carried route ran" needs the program's
/// own result, and this to know which route was compiled.
#[cfg(test)]
pub(in crate::cranelift_backend) fn capacity_phase_dispatch() -> (usize, usize) {
    super::effects::CAPACITY_PHASE_DISPATCH.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_capacity_phase_dispatch() {
    super::effects::CAPACITY_PHASE_DISPATCH.with(|cell| cell.set((0, 0)));
}

/// **`D5a` checkpoint 4 step 1 — declare every generated context into ONE
/// generated function.**
///
/// Same per-function discipline as `WorkerTargets::declare_in_func`: the
/// `FuncRef`s belong to `func` alone and are never copied between functions.
/// ⛔ Keyed by the planner's `ContinuationContextId`, never by the body origin
/// the context executes -- that key is what would let a consumer resolve a
/// context from a body origin, which is the reconstruction the ruling forbids.
fn declare_response_context_call_in_func<M: Module>(
    module: &mut M,
    func: &mut Function,
    plan: &StaticTransitionPlan<'_>,
    bundle: &UnitBundle,
    id: ContinuationContextId,
) -> Result<DeclaredUnitCall, CraneliftBackendError> {
    let context = plan
        .continuation_contexts()?
        .into_iter()
        .find(|context| context.id() == id)
        .ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "a response owner names no installed continuation context".to_string(),
            )
        })?;
    let target = bundle.context(id).ok_or_else(|| {
        backend_module(
            LoweringCause::PlannerInvariant,
            "a response context was never forward-declared".to_string(),
        )
    })?;
    let (offsets, _frame_bytes) = context.slot_offsets()?;
    Ok(DeclaredUnitCall {
        function: module.declare_func_in_func(target, func),
        frame_owner: Some(PendingVisFrameOwner::Context(context.id())),
        origin: context.worker_body_origin(),
        call_site_origin: context.worker_body_origin(),
        header: context.header(),
        slots: context.slots().to_vec(),
        offsets,
        result_contract: plan
            .checked_ih_generated_context_result_contract(id)?
            .map(DeclaredResultContract::new),
    })
}

pub(in crate::cranelift_backend) fn declare_context_calls_in_func<M: Module>(
    module: &mut M,
    func: &mut Function,
    plan: &StaticTransitionPlan<'_>,
    bundle: &UnitBundle,
) -> Result<BTreeMap<ContinuationContextId, DeclaredUnitCall>, CraneliftBackendError> {
    let mut calls = BTreeMap::new();
    for context in plan.continuation_contexts()? {
        let target = bundle.context(context.id()).ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "a planned generated context was never forward-declared".to_string(),
            )
        })?;
        let (offsets, _frame_bytes) = context.slot_offsets()?;
        calls.insert(
            context.id(),
            DeclaredUnitCall {
                function: module.declare_func_in_func(target, func),
                frame_owner: Some(PendingVisFrameOwner::Context(context.id())),
                // The context EXECUTES this body, so the origin it answers for
                // is unchanged and the source edge it serves is untouched.
                origin: context.worker_body_origin(),
                call_site_origin: context.worker_body_origin(),
                header: context.header(),
                slots: context.slots().to_vec(),
                offsets,
                result_contract: plan
                    .checked_ih_generated_context_result_contract(context.id())?
                    .map(DeclaredResultContract::new),
            },
        );
    }
    Ok(calls)
}

pub(super) fn define_unit_bodies<M: Module>(
    module: &mut M,
    compiler: &mut Lowering<'_>,
    helpers: ArtifactHelpers<'_>,
    bundle: &UnitBundle,
    call_edges: &CallEdgeTargets,
    staged_root_value: Option<&RuntimeValue>,
) -> Result<RootUnitResult, CraneliftBackendError> {
    let root = compiler
        .static_transition_plan
        .root_emittable_unit()?
        .function();
    // `D4`: projected once, declared afresh into each generated function below.
    let worker_targets = resolve_worker_targets(&compiler.static_transition_plan, bundle)?;
    // `RT-DECL-CLOSURE-PORT` `D5` — opened here because this bundle pass is the
    // only place a checked same-SCC call can reach a declaration-owned unit.
    compiler.checked_call_ledger = Some(CheckedCallLedger::open(
        compiler.oriented_subcontinuation_plan.as_ref(),
    ));
    let mut root_result = None;
    // `D5a` checkpoint 1: the SAME population `declare_unit_bundle` walked.
    // Both passes read `executable_units` so declared and defined cannot
    // disagree -- reading different methods here is exactly how a phantom
    // appears.
    let emissions = compiler
        .static_transition_plan
        .executable_units()?
        .into_iter()
        .map(|unit| {
            let (offsets, frame_bytes) = unit.slot_offsets()?;
            Ok(OwnedUnitEmission {
                function: unit.function(),
                body_occurrence: unit.body_occurrence(),
                definition: unit.definition(),
                header: unit.header(),
                slots: unit.slots().to_vec(),
                offsets,
                frame_bytes,
            })
        })
        .collect::<Result<Vec<_>, CraneliftBackendError>>()?;
    for unit in emissions {
        let id = bundle.function(unit.function).ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "a planned unit was never forward-declared".to_string(),
            )
        })?;
        let is_root = root == unit.function;
        let outcome = define_unit_body(
            module,
            compiler,
            helpers,
            unit,
            id,
            bundle,
            call_edges,
            &worker_targets,
            is_root,
            staged_root_value,
        )?;
        if let Some(outcome) = outcome {
            if root_result.replace(outcome).is_some() {
                return Err(backend_module(
                    LoweringCause::PlannerInvariant,
                    "more than one emitted unit claimed root result authority".to_string(),
                ));
            }
        }
    }
    // ⛔ The continuation ledger is NOT closed here. `D5a` checkpoint 2 moved
    // its lifetime out to the orchestration in `core.rs`, because two more
    // passes after this one declare, claim and emit causal calls and closing
    // here reported their tokens absent before they could exist.
    // `D5` closeout, before the artifact is published: planned = consumed =
    // emitted, and every emitted actual callee equals its exact resolved
    // target. ⛔ `consumed` is the affine machinery's OWN set, passed in rather
    // than mirrored, so the two cannot drift apart.
    compiler
        .checked_call_ledger
        .take()
        .ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "the checked call ledger went missing".to_string(),
            )
        })?
        .close(&compiler.consumed_recursive_call_templates)?;
    root_result.ok_or_else(|| {
        backend_module(
            LoweringCause::PlannerInvariant,
            "the emitted unit bundle did not define its recorded root".to_string(),
        )
    })
}

struct OwnedUnitEmission {
    function: PredeclaredFunctionId,
    /// The issued body occurrence this unit lowers, carried from the planner.
    body_occurrence: StaticOriginId,
    /// `RT-SRCBODY-BIND-ORDER` `D1` — carried because the ABI descriptor run
    /// and the source body's semantic environment are now two different orders,
    /// and only the definition arm says which units get the conversion.
    definition: AbiUnitDefinition,
    header: AbiFrameHeader,
    slots: Vec<AbiSlot>,
    offsets: Vec<u32>,
    frame_bytes: u32,
}

/// **`RT-SRCBODY-BIND-ORDER` `D1` — does this unit's body bind a SOURCE
/// parameter run?**
///
/// The ABI lays parameters out in declaration order; a source body indexes its
/// binders as de Bruijn levels from the innermost, which is the reverse. The
/// conversion is therefore owed exactly where the run being bound is a source
/// parameter run:
///
/// - `CallableDeclaration` and `ClosureBody` — yes. Both bind a written
///   parameter list, and `RT-DECL-CLOSURE-PORT` `D2` already treats the two
///   identically wherever the question is about their parameter/capture runs.
/// - `SchedulingEntry` — no. Its parameters are the closed process ingress
///   roles, resolved by `AbiProcessParameter` ordinal rather than by a binder
///   index, and reversing them would rename the two roles.
/// - `ContinuationSpecialization` — no. Its run is the planner's own ordered
///   input projection, positional by construction.
///
/// This is a question about the run's PROVENANCE, not its length: a one-
/// parameter source body is unaffected by the reversal but is still a source
/// body, and a two-role scheduling entry is affected but is still not one.
/// **`RT-SRCBODY-BIND-ORDER` `D3` — which host bound a semantic environment.**
///
/// The two seats that build one: a unit's own body, and a generated context
/// that lowers someone else's body. The equivalence `D2` owes is between them.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum SrcbodyBindHost {
    OrdinaryUnit,
    GeneratedContext,
}

/// **`RT-SRCBODY-BIND-ORDER` `D3` — one semantic environment, as PRODUCTION
/// built it.**
///
/// The row is deliberately raw: it records the ABI ordinals in the order they
/// were pushed into the environment, so a control re-derives the conversion
/// from the sequence rather than being told about it. `converted` is recorded
/// beside them precisely so a control can catch a build whose classification
/// is right and whose environment does not follow it — the failure a control
/// that reads the predicate alone is blind to.
#[cfg(test)]
#[derive(Clone, Debug)]
pub(in crate::cranelift_backend) struct SrcbodyBindOrderObservation {
    pub(in crate::cranelift_backend) host: SrcbodyBindHost,
    pub(in crate::cranelift_backend) definition: AbiUnitDefinition,
    /// What `source_body_binding_order` answered for `definition`.
    pub(in crate::cranelift_backend) converted: bool,
    /// The body this environment was built for. The join key for `D2`'s
    /// cross-host equivalence: one body, two hosts.
    pub(in crate::cranelift_backend) body_origin: StaticOriginId,
    /// Parameter ABI ordinals, in SEMANTIC ENVIRONMENT order.
    pub(in crate::cranelift_backend) parameter_ordinals: Vec<u32>,
    /// Capture ABI ordinals, in SEMANTIC ENVIRONMENT order.
    pub(in crate::cranelift_backend) capture_ordinals: Vec<u32>,
}

#[cfg(test)]
thread_local! {
    static SRCBODY_BIND_ORDER: std::cell::RefCell<Vec<SrcbodyBindOrderObservation>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn srcbody_bind_order_record(
    observation: SrcbodyBindOrderObservation,
) {
    SRCBODY_BIND_ORDER.with(|cell| cell.borrow_mut().push(observation));
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct SrcbodyParameterLoad {
    pub(in crate::cranelift_backend) body_origin: StaticOriginId,
    pub(in crate::cranelift_backend) abi_ordinal: u32,
    pub(in crate::cranelift_backend) word: cranelift_codegen::ir::Value,
    pub(in crate::cranelift_backend) emitted_load_offset: i32,
}

#[cfg(test)]
thread_local! {
    static SRCBODY_PARAMETER_LOADS: std::cell::RefCell<Vec<SrcbodyParameterLoad>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn srcbody_parameter_loads_take() -> Vec<SrcbodyParameterLoad> {
    SRCBODY_PARAMETER_LOADS.with(|cell| std::mem::take(&mut *cell.borrow_mut()))
}

/// Drains every environment built on this thread since the last take.
#[cfg(test)]
pub(in crate::cranelift_backend) fn srcbody_bind_order_take() -> Vec<SrcbodyBindOrderObservation> {
    SRCBODY_BIND_ORDER.with(|cell| std::mem::take(&mut *cell.borrow_mut()))
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` — this became fallible for the
/// fusion arm, and `false` would have been the wrong answer rather than a safe
/// one.**
///
/// The other four arms answer from a property of their class that is already
/// settled. A static continuation fusion's binding order is not: it is decided
/// by how the emitter builds the generated definition's environment, and
/// `D2f`'s scoped source-body authorities are the successor seam. Answering
/// `false` here would pick one of the two orders for a class whose environment
/// nothing constructs yet, and the wrong pick is silent — it reverses an
/// operand run rather than failing.
pub(in crate::cranelift_backend) fn source_body_binding_order(
    definition: AbiUnitDefinition,
) -> Result<bool, CraneliftBackendError> {
    Ok(match definition {
        AbiUnitDefinition::CallableDeclaration { .. } | AbiUnitDefinition::ClosureBody { .. } => {
            true
        }
        AbiUnitDefinition::SchedulingEntry { .. }
        | AbiUnitDefinition::ContinuationSpecialization { .. } => false,
        AbiUnitDefinition::StaticContinuationFusion { .. } => {
            return Err(unsupported(
                LoweringCause::PlannerInvariant,
                "StaticContinuationFusion",
                "a static continuation fusion unit reached environment construction, but its \
                 source-body binding order is the emitter's to establish and no generated \
                 definition exists; RT-LEXICAL-RECURSOR-CONSUMERS D2f refuses rather than \
                 choosing an operand order for it",
            ));
        }
    })
}

fn define_unit_body<M: Module>(
    module: &mut M,
    compiler: &mut Lowering<'_>,
    helpers: ArtifactHelpers<'_>,
    unit: OwnedUnitEmission,
    id: FuncId,
    bundle: &UnitBundle,
    call_edges: &CallEdgeTargets,
    worker_targets: &WorkerTargets,
    is_root: bool,
    staged_root_value: Option<&RuntimeValue>,
) -> Result<Option<RootUnitResult>, CraneliftBackendError> {
    // ⭐ The declared size and the walked size must agree. They are the same
    // walk by construction (`abi::slot_offsets` totals for both), so this
    // rejects a corrupted descriptor rather than a divergent derivation.
    if unit.frame_bytes != unit.header.frame_bytes {
        return Err(backend_module(
            LoweringCause::PlannerInvariant,
            "abi frame size disagrees with its own slot run".to_string(),
        ));
    }
    let result_offset = unit
        .slots
        .iter()
        .zip(&unit.offsets)
        .find(|(slot, _)| slot.kind == AbiSlotKind::Result)
        .map(|(_, offset)| *offset)
        .ok_or_else(|| {
            // ⛔ Fails closed. `CONVENTION_SLOTS` puts a `Result` slot in every
            // unit, so its absence means the descriptor is not the one `B2R`
            // built, and returning a default word would fabricate a result.
            backend_module(
                LoweringCause::PlannerInvariant,
                "unit frame declares no result slot".to_string(),
            )
        })?;
    let trap_offset = unit
        .slots
        .iter()
        .zip(&unit.offsets)
        .find(|(slot, _)| slot.kind == AbiSlotKind::Trap)
        .map(|(_, offset)| *offset)
        .ok_or_else(|| {
            backend_module(
                LoweringCause::PlannerInvariant,
                "unit frame declares no trap slot".to_string(),
            )
        })?;

    let sig = unit_signature(module);
    let mut func = Function::with_name_signature(UserFuncName::user(2, id.as_u32()), sig);
    // ⭐ `D4` — this unit's callees are referenced HERE, by the static identity
    // the planner assigned, before the body exists to call them.
    //
    // ⛔ **The call instructions themselves are `S6`'s**, because a unit body
    // does not descend into its own expression until `lower_expr`'s consumers
    // switch over. ⇒ What is live today is the **reference**: a `FuncRef`
    // resolved from a validated call edge through the declared bundle, with no
    // ordinal, no name parsing and no dynamic lookup anywhere on the path. ⚠ An
    // emitted `call` is not claimed and no control here asserts one.
    #[cfg(test)]
    let unit_trap_authority = match TRAP_FRAME_BINDING_MUTATION.with(std::cell::Cell::get) {
        TrapFrameBindingMutation::MisclassifyUnitAsRoot => Some(TrapExitAuthority::Root {
            process_sentinel: false,
            source_authorized: false,
        }),
        TrapFrameBindingMutation::Exact | TrapFrameBindingMutation::DeleteUnitLane => None,
    };
    #[cfg(not(test))]
    let unit_trap_authority = None;
    let mut function_local = helpers.declare_in_func(module, &mut func, unit_trap_authority);
    function_local.grafted_spine_scope =
        Some(GraftedSpineFunctionScope::Predeclared(unit.function));
    let declared_calls = call_edges.declare_in_func(unit.function, module, &mut func)?;
    // `D3` — this ordinary Function declares its OWN `FuncRef` for every causal
    // token it owns, keyed by the four-field identity. Minted here, into this
    // `Function`; never passed across functions.
    function_local.continuation_calls = match compiler.continuation_claims.as_ref() {
        Some(ledger) => ledger.declare_owned_in_func(
            ContinuationEmissionOwner::Predeclared(unit.function),
            module,
            &mut func,
            &compiler.static_transition_plan,
        )?,
        None => BTreeMap::new(),
    };
    // `4b`: the DECLARED half of the four-set equality, recorded where the refs
    // are actually minted.
    if let Some(ledger) = compiler.continuation_claims.as_mut() {
        ledger.record_declared(function_local.continuation_calls.keys().cloned())?;
    }
    // `RT-DECL-CLOSURE-PORT` `D5a` contract 3 — AUTHORITY BEFORE EMISSION.
    //
    // Projected here, beside the declaration of this unit's own call refs and
    // **before the function is defined**. ⛔ It is a projection of authority the
    // planner already issued, keyed on the owner this pass is about to define;
    // lowering supplies only that owner and never searches, reverse-derives a
    // consumer, or reads an owner back off anything it emitted.
    let result_edges = compiler
        .static_transition_plan
        .continuation_result_edges_owned_by(ContinuationEmissionOwner::Predeclared(
            unit.function,
        ))?;
    // `D3`: the owner operand for the claim, supplied independently of any
    // token -- this is the ordinary producer unit currently being defined.
    #[cfg(test)]
    d5a_trace(format!(
        "UNIT-BODY entry function={:?} origin={:?}",
        unit.function, unit.body_occurrence
    ));
    compiler.open_aggregate_events(id)?;
    // `D8o` — bound for this body's lifetime and released on exit, so no later
    // Function inherits it. ⛔ The domain is unchanged: `D5a`'s rule that an
    // ordinary predeclared unit body emits as itself.
    let ambient = AmbientBodyAuthority::bind(
        compiler,
        ContinuationEmissionOwner::Predeclared(unit.function),
        unit.function,
    );
    #[cfg(test)]
    crate::cranelift_backend::lowering::record_d8o_body_key(
        compiler.defining_function_id,
        crate::cranelift_backend::lowering::D8oBodyKey::OrdinaryUnit(unit.function),
    );
    function_local.unit_calls = declared_calls.static_bodies;
    function_local.declaration_calls = declared_calls.declarations;
    // `RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` — THE REDIRECT.
    //
    // Installing body ownership removed the producer's invocation from
    // `executable_call_edges`, so the seat this unit calls the producer at has no
    // entry in the table above. That absence is not the redirect — it is the hole
    // the redirect fills, and leaving it is the `"retained body ... has no
    // graph-derived call target in this unit"` refusal.
    //
    // Applied to the function-local COPY, after the table is populated and
    // before any body is defined. Never to the plan's descriptor: the plane and
    // its ABI arena are the authority the frame contract below is checked
    // against, and mutating them would leave both sides agreeing and prove
    // nothing.
    redirect_fused_producer_invocations(
        module,
        &mut func,
        compiler,
        bundle,
        unit.function,
        &mut function_local.unit_calls,
    )?;
    // `RT-DECL-CLOSURE-PORT` `D5` — the causal control on the ABI half.
    //
    // ⛔⛔ **Injected HERE, on the function-local COPY, and never on the plan's
    // descriptor.** That asymmetry is the entire point: `D5`'s ABI
    // reconciliation claims the declared call record still agrees with the
    // immutable descriptor `D3` validated. A mutation applied to both sides
    // would leave them agreeing and prove nothing; a mutation applied to the
    // descriptor would trip the ABI plane upstream instead. ⇒ Only a mutation
    // of the copy measures the reconciliation itself.
    #[cfg(test)]
    d5_mutate_declared_calls(&mut function_local.declaration_calls);
    // `D4`: this function's own worker refs, minted here and never copied.
    function_local.worker_calls = worker_targets.declare_in_func(module, &mut func);
    // `D6b` -- an ordinary unit body performs no retarget, so the two tables
    // agree. Populated anyway, for the same reason as the context body above.
    function_local.raw_worker_calls = function_local.worker_calls.clone();
    // `D5a` checkpoint 1: the raw template contracts, beside the call targets
    // and deliberately not derived from them.
    function_local.worker_templates = worker_targets.templates().clone();
    // `D5a` checkpoint 4 step 1: this function's own context call targets.
    function_local.context_calls =
        declare_context_calls_in_func(module, &mut func, &compiler.static_transition_plan, bundle)?;
    // `D8n` — this generated Function's own checked-frame consumption
    // transaction, spanning the ordinary unit body exactly. ⛔ Opened before the builder and
    // closed after it, so every branch scope inside nests within it.
    let frame_scope = CheckedFrameFunctionScope::open(compiler)?;
    let token_scope = CheckedTokenFunctionScope::open(compiler)?;
    let mut func_ctx = FunctionBuilderContext::new();
    let root_outcome;
    {
        let mut builder = FunctionBuilder::new(&mut func, &mut func_ctx);
        let entry = builder.create_block();
        builder.append_block_params_for_function_params(entry);
        builder.switch_to_block(entry);
        let envelope = builder.block_params(entry)[0];
        let slots = builder.ins().load(
            module.target_config().pointer_type(),
            MemFlags::trusted(),
            envelope,
            crate::activation_services::UNIT_CALL_FRAME_SLOTS,
        );
        let host_dispatch_context = builder.ins().load(
            module.target_config().pointer_type(),
            MemFlags::trusted(),
            envelope,
            crate::activation_services::UNIT_CALL_FRAME_HOST_DISPATCH_CONTEXT,
        );
        let services = builder.block_params(entry)[1];
        let native_int_arena = builder.ins().load(
            module.target_config().pointer_type(),
            MemFlags::trusted(),
            services,
            crate::activation_services::SERVICES_NATIVE_INT_ARENA,
        );
        Lowering::require_nonzero(&mut builder, native_int_arena);
        let boundary_arena = builder.ins().load(
            module.target_config().pointer_type(),
            MemFlags::trusted(),
            services,
            crate::activation_services::SERVICES_BOUNDARY_ARENA,
        );
        Lowering::require_nonzero(&mut builder, boundary_arena);
        function_local.host_dispatch_context = Some(host_dispatch_context);
        function_local.native_int_arena = Some(native_int_arena);
        function_local.boundary_arena = Some(boundary_arena);
        function_local.services_pointer = Some(services);
        // The two fixed envelope loads are unconditional. Semantic frame
        // accesses below are relative only to the B2R payload base.
        #[cfg(test)]
        let bind_unit_trap_frame = TRAP_FRAME_BINDING_MUTATION.with(std::cell::Cell::get)
            != TrapFrameBindingMutation::DeleteUnitLane;
        #[cfg(not(test))]
        let bind_unit_trap_frame = true;
        if bind_unit_trap_frame {
            function_local.bind_unit_trap_frame(
                slots,
                i32::try_from(trap_offset).map_err(|_| {
                    backend_module(
                        LoweringCause::Capacity,
                        "abi trap slot offset exceeds addressable range".to_string(),
                    )
                })?,
            )?;
            function_local.bind_pending_vis_frame(
                compiler.pending_vis_record_protocol.as_ref(),
                PendingVisFrameOwner::Unit(unit.function),
                slots,
                unit.frame_bytes,
            )?;
        }
        compiler.function_local = function_local;
        #[cfg(test)]
        if is_root
            && PROCESS_SLOT_MUTATION.with(std::cell::Cell::get)
                == ProcessSlotMutation::AttemptFixedContextOffsets
        {
            return Err(backend_module(
                LoweringCause::PlannerInvariant,
                "fixed host-dispatch context is not semantic process-pair storage".to_string(),
            ));
        }
        // `RT-SRCBODY-BIND-ORDER` `D1` — ONE walk, TWO orders.
        //
        // The walk below is still the single load of each Parameter/Capture
        // slot, and `defining_abi_operands` still receives them in descriptor
        // order, unchanged. What is no longer the same order is the **semantic
        // environment** the body is lowered against: `lower_expr` resolves
        // `Var(i)` as a de Bruijn index (`RuntimeExpr::Let` prepends at 0), so
        // the innermost binder is position 0 — while the ABI lays parameters
        // out in declaration order, ordinal 0 first. For a source body those
        // two are reverses of each other, and binding the descriptor run
        // directly delivered parameter 0 where the body asked for the last
        // parameter.
        //
        // The conversion applies only where the body IS a source body with
        // declared parameters — `CallableDeclaration` and `ClosureBody`. A
        // `SchedulingEntry`'s root ingress pair is a closed ABI role, not a
        // source binder run, and a `ContinuationSpecialization`'s projection is
        // the planner's own; both keep the descriptor order they had.
        let mut parameters = Vec::new();
        let mut captures = Vec::new();
        #[cfg(test)]
        let mut parameter_ordinals = Vec::new();
        #[cfg(test)]
        let mut capture_ordinals = Vec::new();
        #[cfg(test)]
        let mut parameter_loads = Vec::new();
        for (slot, offset) in unit.slots.iter().zip(&unit.offsets) {
            if matches!(slot.kind, AbiSlotKind::Parameter | AbiSlotKind::Capture) {
                #[cfg(test)]
                let (base, offset) = if is_root
                    && slot.kind == AbiSlotKind::Parameter
                    && PROCESS_SLOT_MUTATION.with(std::cell::Cell::get)
                        == ProcessSlotMutation::ReintroduceLaunchIngress
                {
                    let offset = match slot.ordinal {
                        0 => crate::boundary_activation::ROOT_INGRESS_PROCESS_INPUT,
                        1 => crate::boundary_activation::ROOT_INGRESS_CAPABILITY,
                        _ => {
                            return Err(backend_module(
                                LoweringCause::PlannerInvariant,
                                "the process-root mutation found an unknown parameter role"
                                    .to_string(),
                            ));
                        }
                    };
                    // The root adapter's companion mutation explicitly put
                    // launch ingress here. Without that producer-side change,
                    // the retained direct host context is not an admissible
                    // semantic-slot source.
                    (host_dispatch_context, offset)
                } else {
                    (
                        slots,
                        i32::try_from(*offset).map_err(|_| {
                            backend_module(
                                LoweringCause::Capacity,
                                "abi input slot offset exceeds addressable range".to_string(),
                            )
                        })?,
                    )
                };
                #[cfg(not(test))]
                let (base, offset) = (
                    slots,
                    i32::try_from(*offset).map_err(|_| {
                        backend_module(
                            LoweringCause::Capacity,
                            "abi input slot offset exceeds addressable range".to_string(),
                        )
                    })?,
                );
                let word = builder
                    .ins()
                    .load(types::I64, MemFlags::trusted(), base, offset);
                #[cfg(test)]
                if slot.kind == AbiSlotKind::Parameter {
                    parameter_loads.push((slot.ordinal, word));
                }
                let carried = CarriedBoundaryWord { word };
                // The process root's two ABI ordinals are closed semantic
                // roles, not generic ValueWord inputs. Recovering them here
                // prevents a borrowed process-input body from being emitted
                // twice behind a runtime carried-representation split.
                let operand =
                    if is_root && compiler.process_object && slot.kind == AbiSlotKind::Parameter {
                        let value = compiler.emit_carrier_scalar(&mut builder, carried)?;
                        match slot.ordinal {
                            ordinal if ordinal == AbiProcessParameter::ProcessInput.ordinal() => {
                                LoweringOperand::Specialized(Lowered::BorrowedNativeValue {
                                    pointer: value,
                                })
                            }
                            ordinal if ordinal == AbiProcessParameter::Capability.ordinal() => {
                                LoweringOperand::Specialized(Lowered::CapabilityToken { value })
                            }
                            _ => {
                                return Err(backend_module(
                                    LoweringCause::PlannerInvariant,
                                    "the process root has an unknown parameter role".to_string(),
                                ));
                            }
                        }
                    } else {
                        LoweringOperand::Carried(carried)
                    };
                // `D5a` checkpoint 4 step 1b: the SAME operand, recorded by ABI
                // position. Taken from this one walk rather than rebuilt, so
                // "index i is ABI position i" holds by construction instead of
                // by two walks agreeing.
                //
                // `RT-SRCBODY-BIND-ORDER` `D1` — that invariant is about THIS
                // vector and is unchanged: the push below is still in
                // descriptor order and is the only thing that writes it. The
                // semantic environment is now built separately, from the same
                // operands, and the two orders agree only where the definition
                // arm takes no conversion. Do not restore the old reading that
                // one push served both jobs.
                compiler
                    .function_local
                    .defining_abi_operands
                    .push(operand.clone());
                // `RT-SRCBODY-BIND-ORDER` `D3c` -- the kind at the same ABI
                // position, from the same walk, so the observatory can derive a
                // semantic position from the descriptor instead of searching
                // the environment for the operand.
                #[cfg(test)]
                compiler
                    .function_local
                    .defining_abi_slot_kinds
                    .push(slot.kind);
                match slot.kind {
                    AbiSlotKind::Parameter => {
                        #[cfg(test)]
                        parameter_ordinals.push(slot.ordinal);
                        parameters.push(LoweringEnvironmentBinding::Value(operand))
                    }
                    _ => {
                        #[cfg(test)]
                        capture_ordinals.push(slot.ordinal);
                        captures.push(LoweringEnvironmentBinding::Value(operand))
                    }
                }
            }
        }
        // `validate_slot_run` proves the Parameter run is a contiguous prefix
        // of the Capture run, so concatenating the two IS the descriptor order
        // — which is what the non-source arms below must keep byte-identically.
        let converts = source_body_binding_order(unit.definition)?;
        if converts {
            parameters.reverse();
            #[cfg(test)]
            parameter_ordinals.reverse();
        }
        let mut env = parameters;
        env.extend(captures);
        // The in-process validation API historically stages one ground
        // `RuntimeValue` as the root environment.  It is compile-time material,
        // not launch ingress and not a generated-call transfer, so it does not
        // acquire an ABI slot.  The value is lowered exactly once inside the
        // selected root unit; descendants can receive it only through their
        // ordinary declared captures.
        if is_root {
            if let Some(value) = staged_root_value {
                env.push(LoweringEnvironmentBinding::Value(
                    LoweringOperand::Specialized(compiler.lower_value(&mut builder, value)?),
                ));
            }
        }
        if is_root {
            compiler.root_terminal_authority =
                compiler.take_distinguished_root_answer_authority()?;
        }
        // **No root special case.** The planner issues every unit's body
        // occurrence at the visit that registered its scheduling entry, so the
        // carried value is already the right one for the root and for every
        // declaration alike.
        //
        // This conditional existed because `unit.origin` was an ALIAS of the
        // scheduling entry, which is wrong for any body that schedules
        // something before itself — the root was simply the one case that had
        // been noticed and patched. Substituting at one arm left every non-root
        // unit entering its entry and never reaching its body occurrence or the
        // join subtree beneath it. Reinstating a branch here would restore that
        // defect for whichever arm it did not cover.
        let body_origin = unit.body_occurrence;
        #[cfg(test)]
        for ordinal in parameter_ordinals.iter().copied() {
            let word = parameter_loads
                .iter()
                .find(|(abi_ordinal, _)| *abi_ordinal == ordinal)
                .expect("source binder ordinal was loaded from the declared parameter run")
                .1;
            let offset = match builder.func.dfg.value_def(word) {
                cranelift_codegen::ir::ValueDef::Result(inst, _) => {
                    match builder.func.dfg.insts[inst] {
                        cranelift_codegen::ir::InstructionData::Load { offset, .. } => {
                            i32::from(offset)
                        }
                        ref other => panic!("a source binder has no emitted Load: {other:?}"),
                    }
                }
                other => {
                    panic!("a source binder is not defined by an emitted instruction: {other:?}")
                }
            };
            SRCBODY_PARAMETER_LOADS.with(|cell| {
                cell.borrow_mut().push(SrcbodyParameterLoad {
                    body_origin,
                    abi_ordinal: ordinal,
                    word,
                    emitted_load_offset: offset,
                })
            });
        }
        #[cfg(test)]
        srcbody_bind_order_record(SrcbodyBindOrderObservation {
            host: SrcbodyBindHost::OrdinaryUnit,
            definition: unit.definition,
            converted: converts,
            body_origin,
            parameter_ordinals,
            capture_ordinals,
        });
        let body = compiler.retained_body_occurrence(body_origin)?;
        compiler.select_terminal_result_origins(body_origin, body.expr)?;
        let lowered = compiler.lower_expr(&mut builder, body, &env)?;
        compiler.validate_join_plan_consumption(unit.function, unit.body_occurrence)?;
        let (result, outcome) = if is_root {
            let lowered = match lowered {
                LoweringOperand::Residual(residual) => LoweringOperand::Carried(
                    compiler.decode_residual_child(&mut builder, residual)?,
                ),
                other @ (LoweringOperand::Specialized(_) | LoweringOperand::Carried(_)) => other,
            };
            match lowered {
                LoweringOperand::Carried(word) if !compiler.process_object => (
                    Some(word.word),
                    Some(RootUnitResult {
                        decoder: Some(ResultDecoder::Boundary),
                        trap: None,
                    }),
                ),
                LoweringOperand::Carried(word) => {
                    let tag = builder
                        .ins()
                        .band_imm(word.word, crate::boundary_value::BOUNDARY_TAG_MASK as i64);
                    let immediate = builder.create_block();
                    let persistent = builder.create_block();
                    let decoded = builder.create_block();
                    builder.append_block_param(decoded, types::I64);
                    let is_immediate = builder.ins().icmp_imm(
                        cranelift_codegen::ir::condcodes::IntCC::Equal,
                        tag,
                        BoundaryTag::ImmediateExitStatus as i64,
                    );
                    builder
                        .ins()
                        .brif(is_immediate, immediate, &[], persistent, &[]);

                    builder.switch_to_block(immediate);
                    let status = compiler.emit_carrier_scalar(&mut builder, word)?;
                    builder.ins().jump(decoded, &[status.into()]);

                    builder.switch_to_block(persistent);
                    // Decode a checked ExitCode by constructor shape, not by the
                    // allocation lifetime of its fields. The root reads either
                    // persistent or invocation-owned nodes in this invocation.
                    let class = compiler.emit_carrier_class(&mut builder, word)?;
                    Lowering::require_i64(&mut builder, class, BoundaryClass::Constructor as i64);
                    // The non-root Ret result edge retains its constructor field.
                    // Only the checked process root projects that carried ExitCode.
                    let constructor = compiler.emit_carrier_tag(&mut builder, word)?;
                    let fields = compiler.emit_carrier_field_count(&mut builder, word)?;
                    let success_identity = compiler
                        .static_transition_plan
                        .synthesized_constructor_identity(SynthesizedConstructorRole::Fixed(
                            SynthesizedFixedConstructorRole::ExitSuccess,
                        ))?;
                    let failure_identity = compiler
                        .static_transition_plan
                        .synthesized_constructor_identity(SynthesizedConstructorRole::Fixed(
                            SynthesizedFixedConstructorRole::ExitFailure,
                        ))?;
                    let success_tag = Lowering::carrier_identity_immediate(
                        &mut builder,
                        success_identity.tag_abi_word()?,
                    );
                    let is_success = builder.ins().icmp(
                        cranelift_codegen::ir::condcodes::IntCC::Equal,
                        constructor,
                        success_tag,
                    );
                    let success = builder.create_block();
                    let failure = builder.create_block();
                    builder.ins().brif(is_success, success, &[], failure, &[]);

                    builder.switch_to_block(success);
                    Lowering::require_i64(&mut builder, fields, 0);
                    let zero = builder.ins().iconst(types::I64, 0);
                    builder.ins().jump(decoded, &[zero.into()]);

                    builder.switch_to_block(failure);
                    let failure_tag = Lowering::carrier_identity_immediate(
                        &mut builder,
                        failure_identity.tag_abi_word()?,
                    );
                    let is_failure = builder.ins().icmp(
                        cranelift_codegen::ir::condcodes::IntCC::Equal,
                        constructor,
                        failure_tag,
                    );
                    Lowering::require_i64(&mut builder, is_failure, 1);
                    Lowering::require_i64(&mut builder, fields, 1);
                    let field = compiler.emit_carrier_field(&mut builder, word, 0)?;
                    let field_tag = builder
                        .ins()
                        .band_imm(field.word, crate::boundary_value::BOUNDARY_TAG_MASK as i64);
                    Lowering::require_i64(
                        &mut builder,
                        field_tag,
                        BoundaryTag::ImmediateInt as i64,
                    );
                    let code = compiler.emit_carrier_scalar(&mut builder, field)?;
                    let small = Lowering::carrier_small_marker(&mut builder);
                    compiler.function_local.native_int_tags.insert(code, small);
                    let status = compiler.emit_process_exit_status(
                        &mut builder,
                        Lowered::Constructor {
                            constructor: compiler.process_symbols.exit_failure.clone(),
                            synthesized_identity: Some(failure_identity),
                            occurrence: None,
                            args: vec![ConstructorField::specialized(Lowered::Int {
                                value: code,
                                known: None,
                            })],
                        },
                    );
                    builder.ins().jump(decoded, &[status.into()]);
                    builder.switch_to_block(decoded);
                    let status = builder.block_params(decoded)[0];
                    (
                        Some(status),
                        Some(RootUnitResult {
                            decoder: Some(ResultDecoder::ProcessStatus),
                            trap: None,
                        }),
                    )
                }
                LoweringOperand::Specialized(Lowered::Trap(trap)) => {
                    #[cfg(test)]
                    if compiler.process_object {
                        px8tr_record_trap_provenance(
                            Px8trTrapProvenanceEvent::FinalProcessObjectTrap { trap: trap.clone() },
                        );
                    }
                    compiler.emit_current_trap(&mut builder, &trap)?;
                    (
                        None,
                        Some(RootUnitResult {
                            decoder: Some(ResultDecoder::TrapOnly),
                            trap: None,
                        }),
                    )
                }
                LoweringOperand::Specialized(value) => {
                    let (token, decoder) = compiler.emit_result(&mut builder, value)?;
                    (
                        Some(token),
                        Some(RootUnitResult {
                            decoder: Some(decoder),
                            trap: None,
                        }),
                    )
                }
                LoweringOperand::Residual(_) => {
                    unreachable!("root R was decoded before result dispatch")
                }
            }
        } else {
            // `RT-DECL-CLOSURE-PORT` `D5a` — THE DETACHED-RESULT SEAT.
            //
            // The exact retained result is lowered; nothing has been
            // transferred into a carrier, allocated, published or joined. This
            // is where the landed object fixture was measured to refuse, and it
            // is the seat for every producer owner the fixed point detached as
            // an ordinary unit result.
            //
            // ⛔ Applied on the non-root path only, because this is the only
            // result surface that reaches `transfer_unit_result_into_carrier`.
            // A root that owned an undischarged projected call would be caught
            // by the whole-pass claim closure, not silently dropped.
            let lowered = compiler.eliminate_detached_producer_continuation(
                &mut builder,
                &result_edges,
                lowered,
                &env,
            )?;
            let word = match lowered {
                LoweringOperand::Carried(word) => Some(word.word),
                LoweringOperand::Specialized(Lowered::Trap(trap)) => {
                    compiler.emit_current_trap(&mut builder, &trap)?;
                    None
                }
                LoweringOperand::Specialized(value) => Some(
                    compiler
                        .transfer_unit_result_into_carrier(
                            &mut builder,
                            unit.body_occurrence,
                            &value,
                        )?
                        .word,
                ),
                // The ordinary-unit Result frame slot carries K.
                LoweringOperand::Residual(residual) => {
                    #[cfg(any(test, feature = "px8-ds-test-support"))]
                    record_residual_abi_decode();
                    Some(compiler.decode_residual_child(&mut builder, residual)?.word)
                }
            };
            (word, None)
        };
        root_outcome = outcome;
        if let Some(result) = result {
            builder.ins().store(
                MemFlags::trusted(),
                result,
                slots,
                i32::try_from(result_offset).map_err(|_| {
                    backend_module(
                        LoweringCause::Capacity,
                        "abi result slot offset exceeds addressable range".to_string(),
                    )
                })?,
            );
        }
        let status = builder.ins().iconst(types::I64, 0);
        builder.ins().return_(&[status]);
        compiler.record_checked_frame_terminal(&builder, FrameTerminalKind::Normal)?;
        builder.seal_all_blocks();
        builder.finalize();
    }
    ambient.release(compiler);
    frame_scope.close(compiler, &func)?;
    #[cfg(test)]
    d5a_trace(format!(
        "UNIT-BODY done function={:?} origin={:?} root={:?}",
        unit.function,
        unit.body_occurrence,
        root_outcome.as_ref().map(|_| "root")
    ));
    compiler.validate_materialized_dead_join_cfg(unit.function, unit.body_occurrence, &func)?;
    // `4b` -- the emission-seam equality gate, on the FINISHED function and
    // before it is defined into the module. The callee of every recorded causal
    // emission is decoded out of this CLIF and compared with the planner-issued
    // target; a disagreement rejects here rather than being emitted.
    compiler.verify_emitted_continuation_calls(&func, bundle)?;
    // `D8j` — same placement in the ordinary unit-body pass. ⛔ Both passes
    // gate, because a composed discharge can be claimed in either.
    compiler.verify_recorded_composed_discharges(&func, bundle)?;
    #[cfg(test)]
    crate::cranelift_backend::lowering::record_d8j_discharged(
        compiler.function_local.composed_discharges.keys().cloned(),
    );
    // `D8k` -- same accumulation on the ordinary pass. ⛔ The owner supplied is
    // this unit's own emission owner, so `record_composed` can hold the
    // discharge to the identity it claims rather than trusting the seat.
    if let Some(ledger) = compiler.continuation_claims.as_mut() {
        ledger.record_composed(
            compiler.function_local.composed_discharges.keys().cloned(),
            ContinuationEmissionOwner::Predeclared(unit.function),
        )?;
    }
    token_scope.close(compiler, &func)?;
    // `4b` closeout control: verify this function's emissions but never
    // accumulate them, so whole-pass set equality has a population to miss.
    #[cfg(test)]
    let accumulate = CONTINUATION_EMISSION_MUTATION.with(std::cell::Cell::get)
        != ContinuationEmissionMutation::SuppressEmissionAccumulation;
    #[cfg(not(test))]
    let accumulate = true;
    let response_owner_calls = verified_response_owner_calls(
        &compiler.static_transition_plan,
        bundle,
        compiler.function_local.continuation_emissions.keys().chain(
            compiler
                .function_local
                .checked_ih_transport_emissions
                .iter()
                .map(|(transport, _)| transport.source_call_identity()),
        ),
    )?;
    if let Some(ledger) = compiler.continuation_claims.as_mut() {
        if accumulate {
            ledger.record_emitted(
                compiler
                    .function_local
                    .continuation_emissions
                    .keys()
                    .cloned(),
            )?;
        }
        ledger.record_response_owner_calls(response_owner_calls);
    }
    compiler.record_finished_grafted_spine_function(&func, bundle)?;
    verify_cranelift_function(&func, module.isa())?;
    compiler.commit_aggregate_events()?;
    #[cfg(test)]
    scale_b_record_unit_body(&func);
    #[cfg(feature = "px8-ds-test-support")]
    record_plain_native_unit_ir(id, &func);
    let mut ctx = module.make_context();
    std::mem::swap(&mut ctx.func, &mut func);
    module
        .define_function(id, &mut ctx)
        .map_err(|err| backend_module(LoweringCause::Backend, err.to_string()))?;
    // ⛔ Counted HERE, adjacent to `define_function`, and NOT at the call site
    // in the loop above -- where it was first written and where it was
    // worthless.
    //
    // ⭐ A mutation gating the `define_unit_body` call left this test GREEN,
    // because a counter incremented once per loop iteration compares the
    // bundle's length to the length of the collection the loop walks, which are
    // equal by construction. It proved the loop ran and CLAIMED bodies were
    // defined. Only an increment on the emitting path can tell those apart.
    #[cfg(test)]
    B2F_UNIT_EMISSION.with(|cell| {
        let (declared, defined) = cell.get();
        cell.set((declared, defined + 1));
    });
    Ok(root_outcome)
}

#[cfg(test)]
mod diagnostic_cause_tests {
    use super::*;

    #[test]
    fn response_owner_finished_verifiers_share_one_code_even_if_message_changes() {
        // Both pending-Vis and static response-owner verifiers call this same
        // refusal on the missing status/Trap branch, so neither can silently
        // drift to a different family while retaining the other message.
        let finished = response_k_missing_status_trap_refusal();
        assert_eq!(finished.code().as_str(), "N0004");
        let reworded = backend_module(
            LoweringCause::FinishedCode,
            "test-only reworded: Result was read before two required guards".to_string(),
        );
        assert_ne!(finished.to_string(), reworded.to_string());
        assert_eq!(finished.code(), reworded.code());
        assert_ne!(
            finished.code(),
            backend_module(LoweringCause::PlannerInvariant, "same carrier".to_string()).code(),
        );
        assert_ne!(
            finished.code(),
            backend_module(LoweringCause::Capacity, "same carrier".to_string()).code(),
        );
    }
}

#[cfg(test)]
mod captured_environment_bijection {
    use super::*;

    fn origin(id: u32) -> StaticOriginId {
        StaticOriginId::for_test(id)
    }

    /// The admitted run measured on the checked family: nine captures, ordinals
    /// contiguous from zero, each with its own sourced occurrence.
    fn measured_run() -> Vec<(u32, StaticOriginId)> {
        (0..9u32).map(|i| (i, origin(703 - i))).collect()
    }

    #[test]
    fn the_admitted_run_measured_on_the_checked_family_validates() {
        validate_captured_environment_bijection(&measured_run(), 9)
            .expect("the plan-issued run is a bijection over the capture list");
    }

    /// Swapping whole (ordinal, source) PAIRS is refused.
    ///
    /// CORRECTED LABEL: this swaps entire tuples, so what refuses it is the
    /// moved ORDINALS, not the sources -- each source stays paired with its own
    /// ordinal. It was previously described as reordering sources "while
    /// leaving the ordinals alone", which it never did, and that mislabel hid a
    /// real gap: a permutation of the SOURCE COLUMN ALONE preserves count,
    /// ordinals, multiplicity and injectivity, so this validator cannot see it.
    /// That case is covered by
    /// [`super::validate_admitted_matches_role_sequence`] and its own control
    /// below, not here.
    #[test]
    fn swapping_whole_ordinal_source_pairs_is_refused() {
        let mut run = measured_run();
        run.swap(0, 8);
        let refusal = validate_captured_environment_bijection(&run, 9)
            .expect_err("a reordered source run is not the plan's bijection");
        assert!(
            format!("{refusal:?}").contains("ordinal"),
            "the refusal must name the ordinal disagreement: {refusal:?}"
        );
    }

    /// A fabricated source -- one occurrence claimed by two captures -- breaks
    /// injectivity, which a count check cannot see.
    #[test]
    fn a_duplicated_source_occurrence_is_refused() {
        let mut run = measured_run();
        run[3].1 = run[4].1;
        let refusal = validate_captured_environment_bijection(&run, 9)
            .expect_err("two captures may not claim one occurrence");
        assert!(
            format!("{refusal:?}").contains("injective"),
            "the refusal must name the injectivity failure: {refusal:?}"
        );
    }

    /// Order and multiplicity, not just the count: a run whose length matches
    /// but whose ordinals are not the contiguous plan run must still refuse.
    #[test]
    fn a_gapped_ordinal_run_is_refused_even_at_the_right_length() {
        let mut run = measured_run();
        run[5].0 = 42;
        validate_captured_environment_bijection(&run, 9)
            .expect_err("a gapped ordinal run is not the plan's contiguous capture run");
    }

    #[test]
    fn a_short_run_is_refused_against_the_capture_count() {
        let mut run = measured_run();
        run.pop();
        validate_captured_environment_bijection(&run, 9)
            .expect_err("fewer sourced occurrences than captures is not a bijection");
    }

    /// **The fail-closed pin.** A seed capture has no source occurrence, so it
    /// is REFUSED rather than given a positional label.
    #[test]
    fn a_seed_capture_is_refused_rather_than_labelled() {
        let refusal = admitted_capture_source(4, &ContinuationWorkerCaptureSource::Seed)
            .expect_err("a seed capture has no admissible source occurrence");
        let text = format!("{refusal:?}");
        assert!(
            text.contains("seed capture") && text.contains("must not be invented"),
            "the refusal must name the seed case and the prohibition: {text}"
        );
    }

    /// The source value here is deliberately NOT `703 - ordinal`.
    ///
    /// On the measured family the run happens to be descending, so an
    /// implementation that computed `703 - ordinal` instead of reading the
    /// plan's source would satisfy a positive control drawn from that run and
    /// look correct. Choosing an origin outside the pattern makes THIS control
    /// independently discriminating, rather than leaning on the reorder and
    /// structural ACs to carry the anti-positional property alone.
    #[test]
    fn a_lexical_capture_admits_its_planner_issued_occurrence() {
        let admitted =
            admitted_capture_source(4, &ContinuationWorkerCaptureSource::Lexical(origin(42)))
                .expect("a lexical capture carries its source occurrence");
        assert_eq!(
            admitted,
            origin(42),
            "the admitted occurrence must be the plan's own, never derived from the ordinal"
        );
    }
}

#[cfg(test)]
mod admitted_matches_role_sequence {
    use super::*;

    fn origin(id: u32) -> StaticOriginId {
        StaticOriginId::for_test(id)
    }

    /// Two captures whose sources are NOT related by any offset or ordering
    /// pattern, so agreement cannot be satisfied by a formula.
    fn roles() -> Vec<ContinuationOrdinaryEnvelopeRole> {
        vec![
            ContinuationOrdinaryEnvelopeRole::WorkerCapture {
                ordinal: 0,
                owner: PredeclaredFunctionId::for_test(1),
                closure_origin: origin(900),
                source: ContinuationWorkerCaptureSource::Lexical(origin(17)),
                lifetime: PlannedReferentLifetime::ActivationOwned,
            },
            ContinuationOrdinaryEnvelopeRole::WorkerCapture {
                ordinal: 1,
                owner: PredeclaredFunctionId::for_test(1),
                closure_origin: origin(900),
                source: ContinuationWorkerCaptureSource::Lexical(origin(4)),
                lifetime: PlannedReferentLifetime::ActivationOwned,
            },
        ]
    }

    fn admitted() -> Vec<(u32, StaticOriginId)> {
        vec![(0, origin(17)), (1, origin(4))]
    }

    #[test]
    fn the_planner_issued_association_agrees() {
        validate_admitted_matches_role_sequence(&roles(), &admitted())
            .expect("the admitted run is the planner-issued association");
    }

    /// **The gap CV found.** Permuting ONLY the source column preserves the
    /// count, the ordinals, the multiplicity and the injectivity, so the
    /// bijection validator accepts it -- every capture now carries another
    /// capture's occurrence. This control is the one that must see it.
    #[test]
    fn a_source_only_permutation_is_refused() {
        let mut run = admitted();
        let first = run[0].1;
        run[0].1 = run[1].1;
        run[1].1 = first;
        // The permuted run is still A bijection: ordinals intact, sources distinct.
        validate_captured_environment_bijection(&run, 2)
            .expect("the permuted run still satisfies the weaker bijection predicate");
        let refusal = validate_admitted_matches_role_sequence(&roles(), &run)
            .expect_err("a source-only permutation is not the planner-issued association");
        assert!(
            format!("{refusal:?}").contains("not the planner-issued one"),
            "the refusal must name the association, not a shape property: {refusal:?}"
        );
    }

    #[test]
    fn a_short_admitted_run_is_refused_against_the_roles() {
        validate_admitted_matches_role_sequence(&roles(), &admitted()[..1])
            .expect_err("fewer admitted captures than the envelope issues");
    }

    #[test]
    fn an_ordinal_disagreement_with_the_roles_is_refused() {
        let mut run = admitted();
        run[1].0 = 7;
        validate_admitted_matches_role_sequence(&roles(), &run)
            .expect_err("an ordinal the plan does not issue at that position");
    }
}

#[derive(Clone, Copy)]
struct FunctionResultPublication {
    frame: cranelift_codegen::ir::Value,
    result_offset: i32,
    trap_offset: i32,
    result_store: cranelift_codegen::ir::Inst,
    returned_word: cranelift_codegen::ir::Value,
    return_inst: cranelift_codegen::ir::Inst,
}

/// The existing typed UnitBundle identity whose already-declared function is
/// staged. This is proof provenance over the bundle's three disjoint maps, not
/// a new emitted-function class or an alternate registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ExistingResultUnitIdentity {
    Continuation(ContinuationSpecializationId),
    Context(ContinuationContextId),
    Response(StaticResponseOwnerId),
}

#[derive(Default)]
struct ForwardingProof {
    valid: bool,
    grounded: bool,
    sources: BTreeSet<cranelift_codegen::ir::Value>,
}

#[derive(Clone, Copy)]
struct FinishedUnitResultRealization {
    unit: ExistingResultUnitIdentity,
    target: FuncId,
}

#[derive(Clone, Copy)]
struct FinishedUnitResultContract {
    unit: ExistingResultUnitIdentity,
    target: FuncId,
    identity: ConstructorIdentity,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CertifiedInfeasibleEdge {
    terminator: cranelift_codegen::ir::Inst,
    destination_ordinal: u8,
    target: cranelift_codegen::ir::Block,
}

#[derive(Clone, Debug)]
struct CertifiedCutProvenance {
    unit: ExistingResultUnitIdentity,
    function: FuncId,
    identity: ConstructorIdentity,
    query_call: cranelift_codegen::ir::Inst,
    queried_word: cranelift_codegen::ir::Value,
    observed_tag: cranelift_codegen::ir::Value,
    field_count_call: cranelift_codegen::ir::Inst,
    comparison: cranelift_codegen::ir::Inst,
    sources: BTreeSet<cranelift_codegen::ir::Value>,
    dependencies: BTreeSet<CertifiedInfeasibleEdge>,
}

#[derive(Default)]
struct CertifiedCuts {
    edges: BTreeSet<CertifiedInfeasibleEdge>,
    provenance: BTreeMap<CertifiedInfeasibleEdge, CertifiedCutProvenance>,
}

#[derive(Clone, Debug)]
struct VerifiedCarrierQuery {
    call: cranelift_codegen::ir::Inst,
    word: cranelift_codegen::ir::Value,
    output: cranelift_codegen::ir::Value,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct StackMemoryLocation {
    slot: cranelift_codegen::ir::StackSlot,
    offset: i64,
    bytes: u32,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ExactProtocolGuard {
    branch: cranelift_codegen::ir::Inst,
    success_ordinal: u8,
    success_target: Block,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ProtocolFactState {
    Absent,
    Produced,
    Guarded,
}

#[derive(Clone, Copy, Debug)]
enum StackWriteEffect {
    None,
    Exact {
        location: StackMemoryLocation,
        value: cranelift_codegen::ir::Value,
    },
    Unknown,
}

struct LocationAccess {
    containers: BTreeSet<cranelift_codegen::ir::StackSlot>,
    escaped: bool,
    /// Calls that receive a container's address. Once one returns, its callee
    /// may hold the pointer, and any later call may write through it.
    retaining_calls: BTreeSet<cranelift_codegen::ir::Inst>,
    /// Blocks entered on some path after a retaining call has returned.
    retained_blocks: std::cell::OnceCell<BTreeSet<Block>>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ReachingMemoryState {
    Uninitialized,
    Exact(cranelift_codegen::ir::Value),
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum MemoryFlowState {
    Absent,
    Current,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CarrierFactGround {
    producer: cranelift_codegen::ir::Inst,
    guard: Option<ExactProtocolGuard>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum CarrierFlowState {
    Absent,
    Pending(u16),
    Current,
}

/// One finalized function plus the facts every staged-Result proof query
/// reads. Each fact is a pure function of the borrowed CLIF and is built at
/// its first use; the shared borrow keeps every cached fact current. With it a
/// query costs its own backward slice and lookups, never a rescan of the whole
/// function, so verification stays near-linear in body size.
struct IndexedFunction<'f> {
    func: &'f Function,
    cfg: std::cell::OnceCell<ControlFlowGraph>,
    edges: std::cell::OnceCell<ReachableEdges>,
    stack_addresses: std::cell::RefCell<StackAddressCache>,
    stored_slots: std::cell::OnceCell<
        BTreeMap<cranelift_codegen::ir::StackSlot, Vec<Option<cranelift_codegen::ir::StackSlot>>>,
    >,
    calls: std::cell::OnceCell<Vec<(cranelift_codegen::ir::Inst, Option<FuncId>)>>,
    comparisons:
        std::cell::OnceCell<BTreeMap<cranelift_codegen::ir::Value, Vec<ConstantComparison>>>,
    branches_on: std::cell::OnceCell<
        BTreeMap<cranelift_codegen::ir::Value, Vec<cranelift_codegen::ir::Inst>>,
    >,
    branches_comparing: std::cell::OnceCell<
        BTreeMap<cranelift_codegen::ir::Value, Vec<cranelift_codegen::ir::Inst>>,
    >,
    loads: std::cell::OnceCell<
        BTreeMap<
            StackMemoryLocation,
            Vec<(cranelift_codegen::ir::Inst, cranelift_codegen::ir::Value)>,
        >,
    >,
    call_address_arguments: std::cell::OnceCell<
        BTreeMap<cranelift_codegen::ir::StackSlot, Vec<cranelift_codegen::ir::Inst>>,
    >,
    location_accesses:
        std::cell::RefCell<BTreeMap<cranelift_codegen::ir::StackSlot, std::rc::Rc<LocationAccess>>>,
}

/// The finalized CFG restricted to blocks reachable from the entry, with each
/// block's incoming edges in `successor_edges` form.
struct ReachableEdges {
    entry: Block,
    reachable: BTreeSet<Block>,
    predecessors: BTreeMap<Block, Vec<(cranelift_codegen::ir::Inst, u8, Block)>>,
}

#[derive(Default)]
struct StackAddressCache {
    roots: BTreeMap<cranelift_codegen::ir::Value, Option<(cranelift_codegen::ir::StackSlot, i64)>>,
    cut_free:
        BTreeMap<cranelift_codegen::ir::Value, Option<(cranelift_codegen::ir::StackSlot, i64)>>,
}

/// One single-result comparison against a constant, as
/// `comparison_with_constant` reads it. `immediate` marks the `icmp_imm` form
/// that `compare_imm_is` also accepts.
#[derive(Clone, Copy, Debug)]
struct ConstantComparison {
    comparison: cranelift_codegen::ir::Value,
    operand: cranelift_codegen::ir::Value,
    condition: cranelift_codegen::ir::condcodes::IntCC,
    constant: i64,
    immediate: bool,
}

impl std::ops::Deref for IndexedFunction<'_> {
    type Target = Function;

    fn deref(&self) -> &Function {
        self.func
    }
}

impl<'f> IndexedFunction<'f> {
    fn new(func: &'f Function) -> Self {
        Self {
            func,
            cfg: std::cell::OnceCell::new(),
            edges: std::cell::OnceCell::new(),
            stack_addresses: std::cell::RefCell::default(),
            stored_slots: std::cell::OnceCell::new(),
            calls: std::cell::OnceCell::new(),
            comparisons: std::cell::OnceCell::new(),
            branches_on: std::cell::OnceCell::new(),
            branches_comparing: std::cell::OnceCell::new(),
            loads: std::cell::OnceCell::new(),
            call_address_arguments: std::cell::OnceCell::new(),
            location_accesses: std::cell::RefCell::default(),
        }
    }

    /// Every call, direct or indirect, keyed by each stack slot whose address
    /// it receives as an argument.
    fn call_address_arguments(
        &self,
    ) -> Result<
        &BTreeMap<cranelift_codegen::ir::StackSlot, Vec<cranelift_codegen::ir::Inst>>,
        CraneliftBackendError,
    > {
        if let Some(calls) = self.call_address_arguments.get() {
            return Ok(calls);
        }
        let mut calls = BTreeMap::<_, Vec<_>>::new();
        for inst in self.insts() {
            if !self.func.dfg.insts[inst].opcode().is_call() {
                continue;
            }
            for argument in self.func.dfg.inst_args(inst) {
                if let Some((slot, _)) = resolve_stack_address(self, *argument)? {
                    let receivers = calls.entry(slot).or_default();
                    if receivers.last() != Some(&inst) {
                        receivers.push(inst);
                    }
                }
            }
        }
        Ok(self.call_address_arguments.get_or_init(|| calls))
    }

    fn insts(&self) -> impl Iterator<Item = cranelift_codegen::ir::Inst> + '_ {
        self.func
            .layout
            .blocks()
            .flat_map(|block| self.func.layout.block_insts(block))
    }

    fn cfg(&self) -> &ControlFlowGraph {
        self.cfg
            .get_or_init(|| ControlFlowGraph::with_function(self.func))
    }

    fn edges(&self) -> Result<&ReachableEdges, CraneliftBackendError> {
        if let Some(edges) = self.edges.get() {
            return Ok(edges);
        }
        let func = self.func;
        let entry = func.layout.entry_block().ok_or_else(|| {
            backend_module(
                LoweringCause::FinishedCode,
                "a staged Result function has no entry block".to_string(),
            )
        })?;
        let mut reachable = BTreeSet::from([entry]);
        let mut predecessors = BTreeMap::<Block, Vec<_>>::new();
        let mut pending = vec![entry];
        while let Some(block) = pending.pop() {
            if !func
                .layout
                .block_insts(block)
                .any(|inst| func.dfg.insts[inst].opcode().is_terminator())
            {
                return Err(backend_module(
                    LoweringCause::FinishedCode,
                    "a staged Result proof reaches a CFG block without a supported terminator"
                        .to_string(),
                ));
            }
            for (terminator, ordinal, target) in successor_edges(func, block)? {
                predecessors
                    .entry(target)
                    .or_default()
                    .push((terminator, ordinal, block));
                if reachable.insert(target) {
                    pending.push(target);
                }
            }
        }
        Ok(self.edges.get_or_init(|| ReachableEdges {
            entry,
            reachable,
            predecessors,
        }))
    }

    fn calls(&self) -> &[(cranelift_codegen::ir::Inst, Option<FuncId>)] {
        self.calls.get_or_init(|| {
            self.insts()
                .filter(|inst| {
                    self.func.dfg.insts[*inst].opcode() == cranelift_codegen::ir::Opcode::Call
                })
                .map(|call| (call, Lowering::decode_direct_callee(self.func, call).ok()))
                .collect()
        })
    }

    /// Constant comparisons whose compared operand resolves to `operand`.
    fn comparisons_of(&self, operand: cranelift_codegen::ir::Value) -> &[ConstantComparison] {
        self.comparisons
            .get_or_init(|| {
                let mut comparisons = BTreeMap::<_, Vec<_>>::new();
                for inst in self.insts() {
                    let [comparison] = self.func.dfg.inst_results(inst) else {
                        continue;
                    };
                    let Some((condition, operand, constant)) =
                        comparison_with_constant(self.func, *comparison)
                    else {
                        continue;
                    };
                    comparisons
                        .entry(self.func.dfg.resolve_aliases(operand))
                        .or_default()
                        .push(ConstantComparison {
                            comparison: *comparison,
                            operand,
                            condition,
                            constant,
                            immediate: matches!(
                                &self.func.dfg.insts[inst],
                                cranelift_codegen::ir::InstructionData::IntCompareImm { opcode, .. }
                                    if *opcode == cranelift_codegen::ir::Opcode::IcmpImm
                            ),
                        });
                }
                comparisons
            })
            .get(&operand)
            .map_or(&[], Vec::as_slice)
    }

    /// `brif` instructions whose condition is exactly `condition`.
    fn branches_on(
        &self,
        condition: cranelift_codegen::ir::Value,
    ) -> &[cranelift_codegen::ir::Inst] {
        self.branches_on
            .get_or_init(|| {
                let mut branches = BTreeMap::<_, Vec<_>>::new();
                for inst in self.insts() {
                    if let cranelift_codegen::ir::InstructionData::Brif { arg, .. } =
                        &self.func.dfg.insts[inst]
                    {
                        branches.entry(*arg).or_default().push(inst);
                    }
                }
                branches
            })
            .get(&condition)
            .map_or(&[], Vec::as_slice)
    }

    /// `brif` instructions whose condition compares exactly `compared` with a
    /// constant.
    fn branches_comparing(
        &self,
        compared: cranelift_codegen::ir::Value,
    ) -> &[cranelift_codegen::ir::Inst] {
        self.branches_comparing
            .get_or_init(|| {
                let mut branches = BTreeMap::<_, Vec<_>>::new();
                for inst in self.insts() {
                    if let cranelift_codegen::ir::InstructionData::Brif { arg, .. } =
                        &self.func.dfg.insts[inst]
                    {
                        if let Some((_, compared, _)) = comparison_with_constant(self.func, *arg) {
                            branches.entry(compared).or_default().push(inst);
                        }
                    }
                }
                branches
            })
            .get(&compared)
            .map_or(&[], Vec::as_slice)
    }

    /// Every load result that reads exactly `location`, in layout order.
    fn loads_of(
        &self,
        location: StackMemoryLocation,
    ) -> Result<&[(cranelift_codegen::ir::Inst, cranelift_codegen::ir::Value)], CraneliftBackendError>
    {
        if self.loads.get().is_none() {
            let mut loads = BTreeMap::<_, Vec<_>>::new();
            for inst in self.insts() {
                for output in self.func.dfg.inst_results(inst) {
                    if let Some((load, location)) = loaded_stack_location(self, *output)? {
                        loads.entry(location).or_default().push((load, *output));
                    }
                }
            }
            let _ = self.loads.set(loads);
        }
        Ok(self
            .loads
            .get()
            .and_then(|loads| loads.get(&location))
            .map_or(&[], Vec::as_slice))
    }

    /// For each stack slot whose address is stored, the slot of each store's
    /// destination (`None` when the destination is not an exact stack
    /// address).
    fn stored_slots(
        &self,
    ) -> Result<
        &BTreeMap<cranelift_codegen::ir::StackSlot, Vec<Option<cranelift_codegen::ir::StackSlot>>>,
        CraneliftBackendError,
    > {
        if let Some(stored) = self.stored_slots.get() {
            return Ok(stored);
        }
        let mut stored_slots = BTreeMap::<_, Vec<_>>::new();
        for inst in self.insts() {
            let (stored, destination) = match &self.func.dfg.insts[inst] {
                cranelift_codegen::ir::InstructionData::StackStore {
                    arg, stack_slot, ..
                } => (*arg, Some(*stack_slot)),
                cranelift_codegen::ir::InstructionData::Store { args, .. }
                | cranelift_codegen::ir::InstructionData::StoreNoOffset { args, .. } => (
                    args[0],
                    resolve_stack_address(self, args[1])?.map(|(slot, _)| slot),
                ),
                data if data.opcode().can_store() => {
                    // An atomic or other undecoded store may write any stack
                    // address among its operands somewhere this index cannot
                    // name, so each such address escapes.
                    for &argument in self.func.dfg.inst_args(inst) {
                        if let Some((slot, _)) = resolve_stack_address(self, argument)? {
                            stored_slots.entry(slot).or_default().push(None);
                        }
                    }
                    continue;
                }
                _ => continue,
            };
            let Some((stored_slot, _)) = resolve_stack_address(self, stored)? else {
                continue;
            };
            stored_slots
                .entry(stored_slot)
                .or_default()
                .push(destination);
        }
        Ok(self.stored_slots.get_or_init(|| stored_slots))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AllPathsVerdict {
    Holds,
    Violated,
    Unreachable,
}

/// Decides whether every finalized path from the entry to `dependent` leaves
/// a forward state machine in `accepted` just after `dependent` executes. The
/// machine starts in `entry_state` and has the finite state set `states`;
/// `through_inst` and `through_edge` are its transfer functions.
///
/// The walk runs backward from `dependent`, replacing the set of acceptable
/// states by its preimage at each instruction and edge. A path is settled once
/// every state is acceptable (no earlier history can matter) and violated once
/// none is, or when it reaches the entry with `entry_state` unacceptable. A
/// violating forward path exists exactly when this walk finds one, so the
/// verdict equals a forward all-paths walk from the entry, while each query
/// visits only the blocks between `dependent` and its deciding events.
fn verify_all_paths_backward<S: Copy + Ord>(
    func: &IndexedFunction<'_>,
    states: &[S],
    entry_state: S,
    dependent: cranelift_codegen::ir::Inst,
    accepted: S,
    mut through_inst: impl FnMut(S, cranelift_codegen::ir::Inst) -> Result<S, CraneliftBackendError>,
    mut through_edge: impl FnMut(
        S,
        cranelift_codegen::ir::Inst,
        u8,
        Block,
    ) -> Result<S, CraneliftBackendError>,
) -> Result<AllPathsVerdict, CraneliftBackendError> {
    let edges = func.edges()?;
    let Some(block) = func
        .layout
        .inst_block(dependent)
        .filter(|block| edges.reachable.contains(block))
    else {
        return Ok(AllPathsVerdict::Unreachable);
    };
    let mut seen = BTreeSet::new();
    let mut pending = vec![(block, Some(dependent), BTreeSet::from([accepted]))];
    'paths: while let Some((block, mut cursor, mut acceptable)) = pending.pop() {
        while let Some(inst) = cursor {
            let mut preimage = BTreeSet::new();
            for state in states {
                if acceptable.contains(&through_inst(*state, inst)?) {
                    preimage.insert(*state);
                }
            }
            if preimage.is_empty() {
                return Ok(AllPathsVerdict::Violated);
            }
            if preimage.len() == states.len() {
                continue 'paths;
            }
            acceptable = preimage;
            cursor = func.layout.prev_inst(inst);
        }
        if block == edges.entry && !acceptable.contains(&entry_state) {
            return Ok(AllPathsVerdict::Violated);
        }
        for &(terminator, ordinal, source) in edges.predecessors.get(&block).into_iter().flatten() {
            let mut preimage = BTreeSet::new();
            for state in states {
                if acceptable.contains(&through_edge(*state, terminator, ordinal, block)?) {
                    preimage.insert(*state);
                }
            }
            if preimage.is_empty() {
                return Ok(AllPathsVerdict::Violated);
            }
            if preimage.len() != states.len() && seen.insert((source, preimage.clone())) {
                pending.push((source, func.layout.last_inst(source), preimage));
            }
        }
    }
    Ok(AllPathsVerdict::Holds)
}

fn all_paths_result(
    verdict: AllPathsVerdict,
    label: &str,
    violation: impl FnOnce() -> String,
) -> Result<(), CraneliftBackendError> {
    match verdict {
        AllPathsVerdict::Holds => Ok(()),
        AllPathsVerdict::Violated => Err(backend_module(LoweringCause::FinishedCode, violation())),
        AllPathsVerdict::Unreachable => Err(backend_module(
            LoweringCause::FinishedCode,
            format!("{label} is not reachable from the finalized function entry"),
        )),
    }
}

/// A finalized CLIF body held outside the object until its pending Result
/// obligations close. `target` and `unit` are the two existing identities
/// already joined by UnitBundle at the typed definition pass.
pub(super) struct StagedResultBody {
    unit: ExistingResultUnitIdentity,
    target: FuncId,
    func: Function,
    authorities: BTreeMap<cranelift_codegen::ir::Value, GeneratedConstructorAuthority>,
    call_obligations: Vec<PendingCallResultObligation>,
    boundary_arena: Option<cranelift_codegen::ir::Value>,
    publication: Option<FunctionResultPublication>,
    required_contract: Option<ConstructorIdentity>,
    independent_contract: Option<ConstructorIdentity>,
}

fn address_depends_on_slots(
    func: &IndexedFunction<'_>,
    value: cranelift_codegen::ir::Value,
    slots: &BTreeSet<cranelift_codegen::ir::StackSlot>,
) -> Result<bool, CraneliftBackendError> {
    let cfg = func.cfg();
    let entry = func.layout.entry_block();
    let mut seen = BTreeSet::new();
    let mut pending = vec![value];
    while let Some(value) = pending.pop() {
        let value = func.dfg.resolve_aliases(value);
        if !seen.insert(value) {
            continue;
        }
        if resolve_stack_address(func, value)?.is_some_and(|(slot, _)| slots.contains(&slot)) {
            return Ok(true);
        }
        match func.dfg.value_def(value) {
            cranelift_codegen::ir::ValueDef::Result(inst, _) => {
                pending.extend(func.dfg.inst_args(inst).iter().copied());
            }
            cranelift_codegen::ir::ValueDef::Param(block, index) => {
                if entry != Some(block) {
                    for predecessor in cfg.pred_iter(block) {
                        pending.extend(incoming_arguments_with_cuts(
                            func,
                            predecessor.inst,
                            block,
                            index,
                            &BTreeSet::new(),
                        )?);
                    }
                }
            }
            cranelift_codegen::ir::ValueDef::Union(left, right) => {
                pending.push(left);
                pending.push(right);
            }
        }
    }
    Ok(false)
}

fn call_may_access_location(
    func: &IndexedFunction<'_>,
    call: cranelift_codegen::ir::Inst,
    access: &LocationAccess,
) -> Result<bool, CraneliftBackendError> {
    for argument in func.dfg.inst_args(call) {
        match resolve_stack_address(func, *argument)? {
            Some((slot, _)) if access.containers.contains(&slot) => return Ok(true),
            None if access.escaped => return Ok(true),
            _ => {}
        }
    }
    Ok(call_follows_retention(func, call, access))
}

pub(super) fn close_and_define_staged_result_bodies<M: Module>(
    module: &mut M,
    helpers: ArtifactHelpers<'_>,
    mut staged: Vec<StagedResultBody>,
) -> Result<(), CraneliftBackendError> {
    let mut required_realizations = Vec::<FuncId>::new();
    for body in &staged {
        for obligation in &body.call_obligations {
            if !obligation.realization_required {
                continue;
            }
            let target = Lowering::decode_direct_callee(&body.func, obligation.call)?;
            let _typed_target = exact_staged_unit(&staged, target)?;
            if !required_realizations.contains(&target) {
                required_realizations.push(target);
            }
        }
    }
    // One analysis per finalized body, shared by every proof query on it; the
    // borrow ends before definition moves the functions out.
    let indexed = staged
        .iter()
        .map(|body| IndexedFunction::new(&body.func))
        .collect::<Vec<_>>();
    // Protocol verification depends only on finalized CLIF and the exact typed
    // call target, not on fixed-point certificate availability. Verify each
    // call/publication once before either graph iteration so a later SCC pass
    // can consume only already-audited locators without re-running the CFG and
    // memory analyses for every dependency round.
    for (body, func) in staged.iter().zip(&indexed) {
        for obligation in &body.call_obligations {
            let target = Lowering::decode_direct_callee(func, obligation.call)?;
            verify_call_result_obligation(func, obligation, target)?;
        }
        if let Some(publication) = body.publication {
            verify_publication_and_terminals(func, publication, true)?;
        }
    }
    let mut realizations = Vec::<FinishedUnitResultRealization>::new();
    loop {
        let mut progress = false;
        for (body, func) in staged.iter().zip(&indexed) {
            if realizations
                .iter()
                .any(|realization| realization.target == body.target)
            {
                continue;
            }
            let Some(publication) = body.publication else {
                continue;
            };
            let mut realized_call_words = BTreeSet::new();
            let mut dependencies_ready = true;
            for obligation in &body.call_obligations {
                let target = Lowering::decode_direct_callee(func, obligation.call)?;
                // A callee's realization is a fact about its own finished
                // body, so it grounds every protocol-verified call of it,
                // demanded or not. Only a demanded call must wait for it.
                let target_unit = if obligation.realization_required {
                    Some(exact_staged_unit(&staged, target)?)
                } else {
                    staged_unit(&staged, target)?
                };
                let realized = target_unit.is_some_and(|target_unit| {
                    realizations.iter().any(|realization| {
                        realization.target == target && realization.unit == target_unit
                    })
                });
                if obligation.realization_required && !realized {
                    dependencies_ready = false;
                    break;
                }
                if realized {
                    realized_call_words.insert(obligation.result_word);
                }
            }
            if !dependencies_ready {
                continue;
            }
            let reachable = reachable_with_cuts(func, &BTreeSet::new())?;
            let cfg = func.cfg();
            let (valid, grounded) = prove_realized_value(
                body,
                func,
                &cfg,
                &reachable,
                &realized_call_words,
                publication.returned_word,
            )?;
            if valid && grounded {
                realizations.push(FinishedUnitResultRealization {
                    unit: body.unit,
                    target: body.target,
                });
                progress = true;
            }
        }
        if !progress {
            break;
        }
    }
    let missing_realizations = required_realizations
        .iter()
        .filter(|target| {
            let target_unit = exact_staged_unit(&staged, **target)
                .expect("required realization targets were validated above");
            !realizations.iter().any(|realization| {
                realization.target == **target && realization.unit == target_unit
            })
        })
        .copied()
        .collect::<Vec<_>>();
    if !missing_realizations.is_empty() {
        let details = missing_realizations
            .iter()
            .filter_map(|target| {
                staged
                    .iter()
                    .find(|body| body.target == *target)
                    .map(|body| {
                        (
                            body.unit,
                            *target,
                            body.publication.map(|publication| {
                                (
                                    publication.returned_word,
                                    body.func.dfg.value_def(publication.returned_word),
                                )
                            }),
                            body.call_obligations.len(),
                            body.call_obligations
                                .iter()
                                .take(8)
                                .map(|obligation| {
                                    (
                                        obligation.result_word,
                                        obligation.identity,
                                        obligation.realization_required,
                                    )
                                })
                                .collect::<Vec<_>>(),
                        )
                    })
            })
            .collect::<Vec<_>>();
        return Err(backend_module(
            LoweringCause::FinishedCode,
            format!(
                "the finished generated-Result realization graph is not closed: \
             missing={missing_realizations:?}, details={details:?}"
            ),
        ));
    }

    let mut required = Vec::<(FuncId, ConstructorIdentity)>::new();
    for (body, func) in staged.iter().zip(&indexed) {
        if let Some(identity) = body.required_contract {
            if !required.contains(&(body.target, identity)) {
                required.push((body.target, identity));
            }
        }
        for obligation in &body.call_obligations {
            let Some(identity) = obligation.identity else {
                continue;
            };
            let target = Lowering::decode_direct_callee(func, obligation.call)?;
            let _typed_target = exact_staged_unit(&staged, target)?;
            if !required.contains(&(target, identity)) {
                required.push((target, identity));
            }
        }
    }
    // A demand at an undeclared call is derived from its consumer: the callee
    // must prove the same identity from its own finished body. Only the
    // declared demands are owed; derived ones only seed their callers.
    let declared_required = required.clone();
    let mut finished = Vec::<FinishedUnitResultContract>::new();
    loop {
        let mut progress = false;
        let mut derived = Vec::<(FuncId, ConstructorIdentity)>::new();
        for (body, func) in staged.iter().zip(&indexed) {
            let demanded = required
                .iter()
                .filter(|(target, _)| *target == body.target)
                .map(|(_, identity)| *identity)
                .collect::<Vec<_>>();
            for identity in demanded {
                if finished.iter().any(|certificate| {
                    certificate.unit == body.unit
                        && certificate.target == body.target
                        && certificate.identity == identity
                }) {
                    continue;
                }
                let mut call_seeds = BTreeMap::new();
                let mut realized_call_words = BTreeSet::new();
                let mut all_calls_finished = true;
                for obligation in &body.call_obligations {
                    let target = Lowering::decode_direct_callee(func, obligation.call)?;
                    if obligation.realization_required {
                        let target_unit = exact_staged_unit(&staged, target)?;
                        if !realizations.iter().any(|realization| {
                            realization.unit == target_unit && realization.target == target
                        }) {
                            all_calls_finished = false;
                            break;
                        }
                        realized_call_words.insert(obligation.result_word);
                    }
                    let certificate = match obligation.identity {
                        Some(identity) => {
                            let target_unit = exact_staged_unit(&staged, target)?;
                            let Some(certificate) = finished.iter().find(|certificate| {
                                certificate.unit == target_unit
                                    && certificate.target == target
                                    && certificate.identity == identity
                            }) else {
                                all_calls_finished = false;
                                break;
                            };
                            Some(*certificate)
                        }
                        None => match staged_unit(&staged, target)? {
                            Some(target_unit) => {
                                if !required.contains(&(target, identity))
                                    && !derived.contains(&(target, identity))
                                {
                                    derived.push((target, identity));
                                }
                                finished
                                    .iter()
                                    .find(|certificate| {
                                        certificate.unit == target_unit
                                            && certificate.target == target
                                            && certificate.identity == identity
                                    })
                                    .copied()
                            }
                            None => None,
                        },
                    };
                    let certified_target =
                        certificate.map_or(target, |certificate| certificate.target);
                    if certified_target != target {
                        return Err(backend_module(
                            LoweringCause::FinishedCode,
                            "a call-result certificate changed the already-verified direct target"
                                .to_string(),
                        ));
                    }
                    if let Some(certificate) = certificate {
                        if call_seeds
                            .insert(obligation.result_word, certificate.identity)
                            .is_some()
                        {
                            return Err(backend_module(
                                LoweringCause::FinishedCode,
                                "two call-result obligations claim one function-local SSA word"
                                    .to_string(),
                            ));
                        }
                    }
                }
                if !all_calls_finished {
                    continue;
                }
                let Some(publication) = body.publication else {
                    continue;
                };
                let proven = if body.independent_contract == Some(identity) {
                    // The response-owner verifier may refine only an actually
                    // initialized returned word. The exact call seed is a
                    // separate dependency; the tag/arity guard cannot conjure
                    // realization from an uninitialized Result slot.
                    realized_call_words.contains(&publication.returned_word)
                } else {
                    let cuts = derive_certified_cuts(
                        body,
                        func,
                        identity,
                        &call_seeds,
                        helpers.boundary_value_abi,
                    )?;
                    let reachable = reachable_with_cuts(func, &cuts.edges)?;
                    let cfg = func.cfg();
                    let proof = prove_forwarded_value(
                        func,
                        &cfg,
                        &reachable,
                        &cuts.edges,
                        &body.authorities,
                        &call_seeds,
                        identity,
                        publication.returned_word,
                    )?;
                    proof.valid && proof.grounded
                };
                if proven {
                    finished.push(FinishedUnitResultContract {
                        unit: body.unit,
                        target: body.target,
                        identity,
                    });
                    progress = true;
                }
            }
        }
        if !derived.is_empty() {
            required.extend(derived);
            progress = true;
        }
        if !progress {
            break;
        }
    }
    let missing = declared_required
        .iter()
        .filter(|(target, identity)| {
            let target_unit = exact_staged_unit(&staged, *target)
                .expect("required contract targets were validated above");
            !finished.iter().any(|certificate| {
                certificate.unit == target_unit
                    && certificate.target == *target
                    && certificate.identity == *identity
            })
        })
        .copied()
        .collect::<Vec<_>>();
    // The test-only layer-isolation switch is read only when a missing
    // certificate would otherwise trigger this exact refusal block.
    if !missing.is_empty() && !suppress_missing_result_contract_refusal() {
        for (target, demanded) in &missing {
            let Some(body) = staged.iter().find(|body| body.target == *target) else {
                continue;
            };
            let Some(publication) = body.publication else {
                continue;
            };
            let Some(actual) = body.authorities.get(&publication.returned_word) else {
                continue;
            };
            if actual.identity != *demanded {
                return Err(backend_module(
                    LoweringCause::FinishedCode,
                    format!(
                        "finished target {target:?} publishes actual constructor identity {:?} at \
                     {:?}, but its pending Result obligation demands {demanded:?}",
                        actual.identity, publication.returned_word,
                    ),
                ));
            }
        }
        let details = missing
            .iter()
            .filter_map(|(target, identity)| {
                staged
                    .iter()
                    .find(|body| body.target == *target)
                    .map(|body| {
                        (
                            *target,
                            *identity,
                            body.publication.map(|publication| {
                                (
                                    publication.returned_word,
                                    body.func.dfg.value_def(publication.returned_word),
                                )
                            }),
                            body.publication.and_then(|publication| {
                                body.authorities
                                    .get(&publication.returned_word)
                                    .map(|authority| (authority.word, authority.identity))
                            }),
                            body.authorities.len(),
                            body.authorities
                                .values()
                                .filter(|authority| authority.identity == *identity)
                                .take(8)
                                .map(|authority| authority.word)
                                .collect::<Vec<_>>(),
                            body.call_obligations.len(),
                            body.call_obligations
                                .iter()
                                .filter(|obligation| obligation.identity == Some(*identity))
                                .take(8)
                                .map(|obligation| obligation.result_word)
                                .collect::<Vec<_>>(),
                        )
                    })
            })
            .collect::<Vec<_>>();
        return Err(backend_module(
            LoweringCause::FinishedCode,
            format!(
                "the finished generated-Result proof graph is not closed: \
             missing={missing:?}, details={details:?}"
            ),
        ));
    }
    for body in &staged {
        if let Some(identity) = body.required_contract {
            if !finished.iter().any(|certificate| {
                certificate.unit == body.unit
                    && certificate.target == body.target
                    && certificate.identity == identity
            }) {
                return Err(backend_module(
                    LoweringCause::FinishedCode,
                    "a generated context's terminal Result contract lacks a finished certificate"
                        .to_string(),
                ));
            }
        }
    }
    drop(indexed);
    for mut body in staged.drain(..) {
        let mut ctx = module.make_context();
        std::mem::swap(&mut ctx.func, &mut body.func);
        module
            .define_function(body.target, &mut ctx)
            .map_err(|error| backend_module(LoweringCause::Backend, error.to_string()))?;
    }
    Ok(())
}

fn compare_imm_is(
    func: &Function,
    value: cranelift_codegen::ir::Value,
    argument: cranelift_codegen::ir::Value,
    condition: cranelift_codegen::ir::condcodes::IntCC,
    immediate: i64,
) -> bool {
    let cranelift_codegen::ir::ValueDef::Result(inst, 0) = func.dfg.value_def(value) else {
        return false;
    };
    matches!(
        &func.dfg.insts[inst],
        cranelift_codegen::ir::InstructionData::IntCompareImm {
            opcode,
            arg,
            cond,
            imm,
        } if *opcode == cranelift_codegen::ir::Opcode::IcmpImm
            && *arg == argument
            && *cond == condition
            && imm.bits() == immediate
    )
}

fn comparison_with_constant(
    func: &Function,
    value: cranelift_codegen::ir::Value,
) -> Option<(
    cranelift_codegen::ir::condcodes::IntCC,
    cranelift_codegen::ir::Value,
    i64,
)> {
    let cranelift_codegen::ir::ValueDef::Result(inst, 0) = func.dfg.value_def(value) else {
        return None;
    };
    match &func.dfg.insts[inst] {
        cranelift_codegen::ir::InstructionData::IntCompareImm {
            opcode,
            arg,
            cond,
            imm,
        } if *opcode == cranelift_codegen::ir::Opcode::IcmpImm => Some((*cond, *arg, imm.bits())),
        cranelift_codegen::ir::InstructionData::IntCompare { opcode, args, cond }
            if *opcode == cranelift_codegen::ir::Opcode::Icmp =>
        {
            if let Some(constant) = iconst_value(func, args[1]) {
                Some((*cond, args[0], constant))
            } else {
                iconst_value(func, args[0]).map(|constant| (*cond, args[1], constant))
            }
        }
        _ => None,
    }
}

fn derive_certified_cuts(
    body: &StagedResultBody,
    func: &IndexedFunction<'_>,
    identity: ConstructorIdentity,
    call_seeds: &BTreeMap<cranelift_codegen::ir::Value, ConstructorIdentity>,
    helpers: &crate::boundary_value_clif::BoundaryLocalFuncs,
) -> Result<CertifiedCuts, CraneliftBackendError> {
    let Some(arena) = body.boundary_arena else {
        return Ok(CertifiedCuts::default());
    };
    #[cfg(feature = "px8-ds-test-support")]
    let mutation = body
        .authorities
        .values()
        .any(|authority| authority.identity != identity)
        .then(claim_generated_result_path_proof_mutation)
        .flatten();
    #[cfg(not(feature = "px8-ds-test-support"))]
    let mutation: Option<()> = None;
    #[cfg(feature = "px8-ds-test-support")]
    if matches!(
        mutation,
        Some(
            GeneratedResultPathProofMutation::FunctionIdentity
                | GeneratedResultPathProofMutation::DisableCutDetector
        )
    ) {
        return Ok(CertifiedCuts::default());
    }

    #[cfg(feature = "px8-ds-test-support")]
    let query_arena = if mutation == Some(GeneratedResultPathProofMutation::SubstituteArena) {
        body.publication
            .map_or(arena, |publication| publication.frame)
    } else {
        arena
    };
    #[cfg(not(feature = "px8-ds-test-support"))]
    let query_arena = arena;
    #[cfg(feature = "px8-ds-test-support")]
    let selected_tag_helper =
        if mutation == Some(GeneratedResultPathProofMutation::SubstituteTagHelper) {
            helpers.field_count
        } else {
            helpers.tag
        };
    #[cfg(not(feature = "px8-ds-test-support"))]
    let selected_tag_helper = helpers.tag;
    let mut tag_queries = verify_carrier_queries(func, selected_tag_helper, query_arena)?;
    let field_queries = verify_carrier_queries(func, helpers.field_count, arena)?;
    #[cfg(feature = "px8-ds-test-support")]
    if mutation == Some(GeneratedResultPathProofMutation::OmitHelperStatusCheck) {
        tag_queries.clear();
    }
    #[cfg(feature = "px8-ds-test-support")]
    if mutation == Some(GeneratedResultPathProofMutation::SubstituteQueriedWord) {
        if let Some(foreign) = body
            .authorities
            .values()
            .find(|authority| authority.identity != identity)
            .map(|authority| authority.word)
        {
            for query in &mut tag_queries {
                query.word = foreign;
            }
        }
    }
    #[cfg(feature = "px8-ds-test-support")]
    if mutation == Some(GeneratedResultPathProofMutation::SubstituteOutputSlot) {
        for query in &mut tag_queries {
            query.output = query.word;
        }
    }
    #[cfg(feature = "px8-ds-test-support")]
    let empty_call_seeds = BTreeMap::new();
    #[cfg(feature = "px8-ds-test-support")]
    let proof_call_seeds = if matches!(
        mutation,
        Some(
            GeneratedResultPathProofMutation::DropRootProof
                | GeneratedResultPathProofMutation::BootstrapCycle
        )
    ) {
        &empty_call_seeds
    } else {
        call_seeds
    };
    #[cfg(not(feature = "px8-ds-test-support"))]
    let proof_call_seeds = call_seeds;

    let tag_preserving_helpers =
        BTreeSet::from([helpers.tag, helpers.field_count, helpers.store_field]);
    let authority_grounds = body
        .authorities
        .iter()
        .filter(|(_, authority)| authority.identity == identity)
        .map(|(word, authority)| {
            verify_constructor_authority_ground(body, func, authority, helpers)
                .map(|ground| (*word, ground))
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    let mut cuts = CertifiedCuts::default();
    loop {
        let reachable = reachable_with_cuts(func, &cuts.edges)?;
        let cfg = func.cfg();
        let mut added = false;
        for tag_query in &tag_queries {
            let proof = prove_forwarded_value(
                func,
                &cfg,
                &reachable,
                &cuts.edges,
                &body.authorities,
                proof_call_seeds,
                identity,
                tag_query.word,
            )?;
            if !proof.valid || !proof.grounded {
                continue;
            }
            #[cfg(feature = "px8-ds-test-support")]
            if matches!(
                mutation,
                Some(
                    GeneratedResultPathProofMutation::AddGenuineVisPredecessor
                        | GeneratedResultPathProofMutation::ClobberHeader
                        | GeneratedResultPathProofMutation::ClobberOutput
                )
            ) {
                continue;
            }

            let mut grounds = Vec::new();
            let mut tracked_words = BTreeSet::from([tag_query.word]);
            for source in &proof.sources {
                tracked_words.insert(*source);
                if let Some(authority) = body.authorities.get(source) {
                    if authority.identity != identity || authority.word != *source {
                        grounds.clear();
                        break;
                    }
                    let Some(ground) = authority_grounds.get(source).copied() else {
                        grounds.clear();
                        break;
                    };
                    grounds.push(ground);
                } else if proof_call_seeds.get(source) == Some(&identity) {
                    let cranelift_codegen::ir::ValueDef::Result(producer, _) =
                        func.dfg.value_def(*source)
                    else {
                        grounds.clear();
                        break;
                    };
                    grounds.push(CarrierFactGround {
                        producer,
                        guard: None,
                    });
                } else {
                    grounds.clear();
                    break;
                }
            }
            if grounds.is_empty() {
                continue;
            }
            if verify_carrier_fact_current(
                func,
                &grounds,
                arena,
                &tracked_words,
                &tag_preserving_helpers,
                tag_query.call,
                "a certified tag query",
            )
            .is_err()
            {
                continue;
            }

            for &branch in func.branches_comparing(tag_query.output) {
                let cranelift_codegen::ir::InstructionData::Brif { arg, blocks, .. } =
                    &func.dfg.insts[branch]
                else {
                    continue;
                };
                if cuts.edges.iter().any(|cut| cut.terminator == branch) {
                    continue;
                }
                let Some((condition, compared, constant)) = comparison_with_constant(func, *arg)
                else {
                    continue;
                };
                let mut expected_constant =
                    i64::try_from(identity.tag_abi_word()?).map_err(|_| {
                        backend_module(
                            LoweringCause::FinishedCode,
                            "a certified Result identity exceeds the runtime tag word".to_string(),
                        )
                    })?;
                #[cfg(feature = "px8-ds-test-support")]
                if mutation == Some(GeneratedResultPathProofMutation::ComparisonConstant) {
                    expected_constant = expected_constant.wrapping_add(1);
                }
                if compared != tag_query.output || constant != expected_constant {
                    continue;
                }
                let cranelift_codegen::ir::ValueDef::Result(comparison_inst, 0) =
                    func.dfg.value_def(*arg)
                else {
                    continue;
                };
                let Some((tag_output_load, _)) = loaded_stack_location(func, tag_query.output)?
                else {
                    continue;
                };
                if verify_all_paths_after_instruction(
                    func,
                    tag_output_load,
                    comparison_inst,
                    "a certified tag comparison",
                )
                .is_err()
                {
                    continue;
                }
                let matching_field_queries = field_queries
                    .iter()
                    .filter(|query| {
                        let field_output_load = loaded_stack_location(func, query.output)
                            .ok()
                            .flatten()
                            .map(|(load, _)| load);
                        query.word == tag_query.word
                            && verify_all_paths_after_instruction(
                                func,
                                tag_output_load,
                                query.call,
                                "a certified field-count query",
                            )
                            .is_ok()
                            && field_output_load.is_some_and(|load| {
                                verify_all_paths_after_instruction(
                                    func,
                                    load,
                                    comparison_inst,
                                    "a certified field-count query",
                                )
                                .is_ok()
                            })
                    })
                    .collect::<Vec<_>>();
                if matching_field_queries.len() != 1 {
                    continue;
                }
                if verify_carrier_fact_current(
                    func,
                    &grounds,
                    arena,
                    &tracked_words,
                    &tag_preserving_helpers,
                    comparison_inst,
                    "a certified tag comparison",
                )
                .is_err()
                {
                    continue;
                }

                let mut destination_ordinal = match condition {
                    cranelift_codegen::ir::condcodes::IntCC::Equal => 1,
                    cranelift_codegen::ir::condcodes::IntCC::NotEqual => 0,
                    _ => continue,
                };
                #[cfg(feature = "px8-ds-test-support")]
                if mutation == Some(GeneratedResultPathProofMutation::ComparisonPolarity) {
                    destination_ordinal ^= 1;
                }
                let mut target = blocks[destination_ordinal as usize].block(&func.dfg.value_lists);
                #[cfg(feature = "px8-ds-test-support")]
                if mutation == Some(GeneratedResultPathProofMutation::DestinationOrdinal) {
                    target =
                        blocks[(destination_ordinal ^ 1) as usize].block(&func.dfg.value_lists);
                }
                let cut = CertifiedInfeasibleEdge {
                    terminator: branch,
                    destination_ordinal,
                    target,
                };
                exact_guard_edge(
                    func,
                    ExactProtocolGuard {
                        branch,
                        success_ordinal: destination_ordinal,
                        success_target: target,
                    },
                )?;
                if cuts.edges.insert(cut) {
                    let field_query = matching_field_queries[0];
                    let prior_dependencies = cuts
                        .edges
                        .iter()
                        .copied()
                        .filter(|dependency| *dependency != cut)
                        .collect();
                    let prior = cuts.provenance.insert(
                        cut,
                        CertifiedCutProvenance {
                            unit: body.unit,
                            function: body.target,
                            identity,
                            query_call: tag_query.call,
                            queried_word: tag_query.word,
                            observed_tag: tag_query.output,
                            field_count_call: field_query.call,
                            comparison: comparison_inst,
                            sources: proof.sources.clone(),
                            dependencies: prior_dependencies,
                        },
                    );
                    if prior.is_some() {
                        return Err(backend_module(
                            LoweringCause::FinishedCode,
                            "one certified edge acquired two proof provenances".to_string(),
                        ));
                    }
                    added = true;
                }
            }
        }
        if !added {
            break;
        }
    }
    verify_cut_provenance(body, func, &cuts)?;
    Ok(cuts)
}

fn exact_guard_edge(
    func: &Function,
    guard: ExactProtocolGuard,
) -> Result<(), CraneliftBackendError> {
    let block = func.layout.inst_block(guard.branch).ok_or_else(|| {
        backend_module(
            LoweringCause::FinishedCode,
            "a protocol guard branch is absent from finalized CLIF".to_string(),
        )
    })?;
    let edges = successor_edges(func, block)?;
    if edges.iter().any(|(terminator, ordinal, target)| {
        *terminator == guard.branch
            && *ordinal == guard.success_ordinal
            && *target == guard.success_target
    }) {
        Ok(())
    } else {
        Err(backend_module(
            LoweringCause::FinishedCode,
            "a protocol guard's success edge disagrees with finalized CLIF".to_string(),
        ))
    }
}

fn exact_staged_unit(
    staged: &[StagedResultBody],
    target: FuncId,
) -> Result<ExistingResultUnitIdentity, CraneliftBackendError> {
    staged_unit(staged, target)?.ok_or_else(|| {
        backend_module(
            LoweringCause::FinishedCode,
            format!("a generated-Result proof dependency targets unstaged function {target:?}"),
        )
    })
}

/// The one typed unit staged under `target`, or `None` when the call leaves
/// the staged set.
fn staged_unit(
    staged: &[StagedResultBody],
    target: FuncId,
) -> Result<Option<ExistingResultUnitIdentity>, CraneliftBackendError> {
    let matches = staged
        .iter()
        .filter(|body| body.target == target)
        .map(|body| body.unit)
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [] => Ok(None),
        [unit] => Ok(Some(*unit)),
        _ => Err(backend_module(
            LoweringCause::FinishedCode,
            format!("more than one typed unit is staged under function {target:?}"),
        )),
    }
}

fn iconst_value(func: &Function, value: cranelift_codegen::ir::Value) -> Option<i64> {
    let cranelift_codegen::ir::ValueDef::Result(inst, 0) = func.dfg.value_def(value) else {
        return None;
    };
    let cranelift_codegen::ir::InstructionData::UnaryImm { opcode, imm } = &func.dfg.insts[inst]
    else {
        return None;
    };
    (*opcode == cranelift_codegen::ir::Opcode::Iconst).then(|| imm.bits())
}

fn incoming_arguments_with_cuts(
    func: &Function,
    inst: cranelift_codegen::ir::Inst,
    target: Block,
    index: usize,
    cuts: &BTreeSet<CertifiedInfeasibleEdge>,
) -> Result<Vec<cranelift_codegen::ir::Value>, CraneliftBackendError> {
    let mut incoming = Vec::new();
    let mut append = |ordinal: u8, destination: &cranelift_codegen::ir::BlockCall| {
        if destination.block(&func.dfg.value_lists) != target
            || cuts.contains(&CertifiedInfeasibleEdge {
                terminator: inst,
                destination_ordinal: ordinal,
                target,
            })
        {
            return Ok(());
        }
        incoming.push(
            destination
                .args_slice(&func.dfg.value_lists)
                .get(index)
                .copied()
                .ok_or_else(|| {
                    backend_module(
                        LoweringCause::FinishedCode,
                        "a certified Result forwarding edge omits its block argument".to_string(),
                    )
                })?,
        );
        Ok::<_, CraneliftBackendError>(())
    };
    let data = &func.dfg.insts[inst];
    if !data.opcode().is_branch() {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a certified Result block parameter has an unsupported incoming control edge"
                .to_string(),
        ));
    }
    for (ordinal, destination) in data
        .branch_destination(&func.dfg.jump_tables)
        .iter()
        .enumerate()
    {
        append(
            u8::try_from(ordinal).map_err(|_| {
                backend_module(
                    LoweringCause::FinishedCode,
                    "a certified Result incoming branch has more than 256 successors".to_string(),
                )
            })?,
            destination,
        )?;
    }
    Ok(incoming)
}

fn instruction_clobbers_location(
    func: &IndexedFunction<'_>,
    inst: cranelift_codegen::ir::Inst,
    location: StackMemoryLocation,
    access: &LocationAccess,
) -> Result<bool, CraneliftBackendError> {
    match stack_write_effect(func, inst)? {
        StackWriteEffect::None => {}
        StackWriteEffect::Exact {
            location: written, ..
        } => return Ok(locations_overlap(location, written)),
        StackWriteEffect::Unknown => return unknown_write_may_alias(func, inst, access),
    }
    if func.dfg.insts[inst].opcode().is_call() {
        return call_may_access_location(func, inst, access);
    }
    Ok(false)
}

fn loaded_stack_location(
    func: &IndexedFunction<'_>,
    value: cranelift_codegen::ir::Value,
) -> Result<Option<(cranelift_codegen::ir::Inst, StackMemoryLocation)>, CraneliftBackendError> {
    let value = func.dfg.resolve_aliases(value);
    let cranelift_codegen::ir::ValueDef::Result(inst, 0) = func.dfg.value_def(value) else {
        return Ok(None);
    };
    let bytes = func.dfg.value_type(value).bytes();
    let location = match &func.dfg.insts[inst] {
        cranelift_codegen::ir::InstructionData::StackLoad {
            opcode,
            stack_slot,
            offset,
        } if *opcode == cranelift_codegen::ir::Opcode::StackLoad => Some(StackMemoryLocation {
            slot: *stack_slot,
            offset: i64::from(i32::from(*offset)),
            bytes,
        }),
        cranelift_codegen::ir::InstructionData::Load {
            opcode,
            arg,
            offset,
            ..
        } if *opcode == cranelift_codegen::ir::Opcode::Load => resolve_stack_address(func, *arg)?
            .and_then(|(slot, base)| {
                base.checked_add(i64::from(i32::from(*offset)))
                    .map(|offset| StackMemoryLocation {
                        slot,
                        offset,
                        bytes,
                    })
            }),
        cranelift_codegen::ir::InstructionData::LoadNoOffset { opcode, arg, .. }
            if *opcode == cranelift_codegen::ir::Opcode::Load =>
        {
            resolve_stack_address(func, *arg)?.map(|(slot, offset)| StackMemoryLocation {
                slot,
                offset,
                bytes,
            })
        }
        _ => None,
    };
    Ok(location.map(|location| (inst, location)))
}

fn location_access(
    func: &IndexedFunction<'_>,
    location: StackMemoryLocation,
) -> Result<std::rc::Rc<LocationAccess>, CraneliftBackendError> {
    if let Some(access) = func.location_accesses.borrow().get(&location.slot) {
        return Ok(access.clone());
    }
    // The containers are the slots reachable from `location.slot` through
    // "this slot's address is stored into that slot" edges.
    let stored_slots = func.stored_slots()?;
    let mut containers = BTreeSet::from([location.slot]);
    let mut escaped = false;
    let mut pending = vec![location.slot];
    while let Some(slot) = pending.pop() {
        for destination in stored_slots.get(&slot).into_iter().flatten() {
            match destination {
                Some(destination) => {
                    if containers.insert(*destination) {
                        pending.push(*destination);
                    }
                }
                None => escaped = true,
            }
        }
    }
    let call_arguments = func.call_address_arguments()?;
    let retaining_calls = containers
        .iter()
        .flat_map(|slot| call_arguments.get(slot).into_iter().flatten().copied())
        .collect();
    let access = std::rc::Rc::new(LocationAccess {
        containers,
        escaped,
        retaining_calls,
        retained_blocks: std::cell::OnceCell::new(),
    });
    func.location_accesses
        .borrow_mut()
        .insert(location.slot, access.clone());
    Ok(access)
}

/// Whether some retaining call of `access` can return before `call` runs.
fn call_follows_retention(
    func: &IndexedFunction<'_>,
    call: cranelift_codegen::ir::Inst,
    access: &LocationAccess,
) -> bool {
    if access.retaining_calls.is_empty() {
        return false;
    }
    let Some(block) = func.layout.inst_block(call) else {
        return true;
    };
    let retained = access.retained_blocks.get_or_init(|| {
        let cfg = func.cfg();
        let mut retained = BTreeSet::new();
        let mut pending = access
            .retaining_calls
            .iter()
            .filter_map(|retaining| func.layout.inst_block(*retaining))
            .flat_map(|block| cfg.succ_iter(block))
            .collect::<Vec<_>>();
        while let Some(block) = pending.pop() {
            if retained.insert(block) {
                pending.extend(cfg.succ_iter(block));
            }
        }
        retained
    });
    if retained.contains(&block) {
        return true;
    }
    // Same block: a retaining call strictly earlier in the layout.
    for inst in func.layout.block_insts(block) {
        if inst == call {
            return false;
        }
        if access.retaining_calls.contains(&inst) {
            return true;
        }
    }
    false
}

fn locations_overlap(left: StackMemoryLocation, right: StackMemoryLocation) -> bool {
    if left.slot != right.slot {
        return false;
    }
    let left_end = left.offset.saturating_add(i64::from(left.bytes));
    let right_end = right.offset.saturating_add(i64::from(right.bytes));
    left.offset < right_end && right.offset < left_end
}

fn prove_forwarded_value(
    func: &Function,
    cfg: &ControlFlowGraph,
    reachable: &BTreeSet<Block>,
    cuts: &BTreeSet<CertifiedInfeasibleEdge>,
    authorities: &BTreeMap<cranelift_codegen::ir::Value, GeneratedConstructorAuthority>,
    call_seeds: &BTreeMap<cranelift_codegen::ir::Value, ConstructorIdentity>,
    identity: ConstructorIdentity,
    value: cranelift_codegen::ir::Value,
) -> Result<ForwardingProof, CraneliftBackendError> {
    // Leaf validity, groundedness and sources depend on the reachable leaf set,
    // not on the path through a reconvergent block-param chain.
    let mut proof = ForwardingProof {
        valid: true,
        grounded: false,
        sources: BTreeSet::new(),
    };
    let mut seen = BTreeSet::new();
    let mut pending = vec![value];
    while let Some(value) = pending.pop() {
        if !seen.insert(value) {
            continue;
        }
        if authorities
            .get(&value)
            .is_some_and(|authority| authority.identity == identity && authority.word == value)
            || call_seeds.get(&value) == Some(&identity)
        {
            proof.grounded = true;
            proof.sources.insert(value);
            continue;
        }
        match func.dfg.value_def(value) {
            cranelift_codegen::ir::ValueDef::Param(block, index) => {
                let predecessors = cfg
                    .pred_iter(block)
                    .filter(|predecessor| reachable.contains(&predecessor.block))
                    .collect::<Vec<_>>();
                if predecessors.is_empty() {
                    proof.valid = false;
                }
                for predecessor in predecessors {
                    pending.extend(incoming_arguments_with_cuts(
                        func,
                        predecessor.inst,
                        block,
                        index,
                        cuts,
                    )?);
                }
            }
            cranelift_codegen::ir::ValueDef::Union(left, right) => {
                pending.push(left);
                pending.push(right);
            }
            cranelift_codegen::ir::ValueDef::Result(_, _) => proof.valid = false,
        }
    }
    if !proof.valid {
        proof.grounded = false;
    }
    Ok(proof)
}

fn prove_realized_value(
    body: &StagedResultBody,
    func: &IndexedFunction<'_>,
    cfg: &ControlFlowGraph,
    reachable: &BTreeSet<Block>,
    realized_call_words: &BTreeSet<cranelift_codegen::ir::Value>,
    value: cranelift_codegen::ir::Value,
) -> Result<(bool, bool), CraneliftBackendError> {
    // Validity and groundedness depend only on the backward-reachable leaves.
    let entry = func.layout.entry_block().ok_or_else(|| {
        backend_module(
            LoweringCause::FinishedCode,
            "a staged Result function has no entry block".to_string(),
        )
    })?;
    let mut valid = true;
    let mut grounded = false;
    let mut seen = BTreeSet::new();
    let mut pending = vec![value];
    while let Some(value) = pending.pop() {
        if !seen.insert(value) {
            continue;
        }
        if body.authorities.contains_key(&value) || realized_call_words.contains(&value) {
            grounded = true;
            continue;
        }
        match func.dfg.value_def(value) {
            cranelift_codegen::ir::ValueDef::Param(block, _) if block == entry => grounded = true,
            cranelift_codegen::ir::ValueDef::Param(block, index) => {
                let predecessors = cfg
                    .pred_iter(block)
                    .filter(|predecessor| reachable.contains(&predecessor.block))
                    .collect::<Vec<_>>();
                if predecessors.is_empty() {
                    valid = false;
                }
                for predecessor in predecessors {
                    pending.extend(incoming_arguments_with_cuts(
                        func,
                        predecessor.inst,
                        block,
                        index,
                        &BTreeSet::new(),
                    )?);
                }
            }
            cranelift_codegen::ir::ValueDef::Union(left, right) => {
                pending.push(left);
                pending.push(right);
            }
            cranelift_codegen::ir::ValueDef::Result(_, _) => {
                if body
                    .call_obligations
                    .iter()
                    .any(|obligation| obligation.result_word == value)
                {
                    valid = false;
                } else {
                    grounded = true;
                }
            }
        }
    }
    Ok((valid, grounded))
}

fn reachable_with_cuts(
    func: &Function,
    cuts: &BTreeSet<CertifiedInfeasibleEdge>,
) -> Result<BTreeSet<Block>, CraneliftBackendError> {
    let entry = func.layout.entry_block().ok_or_else(|| {
        backend_module(
            LoweringCause::FinishedCode,
            "a staged Result function has no entry block".to_string(),
        )
    })?;
    let mut reachable = BTreeSet::from([entry]);
    let mut pending = vec![entry];
    while let Some(block) = pending.pop() {
        for (terminator, destination_ordinal, target) in successor_edges(func, block)? {
            if cuts.contains(&CertifiedInfeasibleEdge {
                terminator,
                destination_ordinal,
                target,
            }) {
                continue;
            }
            if reachable.insert(target) {
                pending.push(target);
            }
        }
    }
    Ok(reachable)
}

/// The reaching state `inst` itself establishes for `location`, or `None`
/// when `inst` leaves the incoming state unchanged.
fn reaching_memory_effect(
    func: &IndexedFunction<'_>,
    inst: cranelift_codegen::ir::Inst,
    location: StackMemoryLocation,
    access: &LocationAccess,
) -> Result<Option<ReachingMemoryState>, CraneliftBackendError> {
    Ok(match stack_write_effect(func, inst)? {
        StackWriteEffect::None => (func.dfg.insts[inst].opcode().is_call()
            && call_may_access_location(func, inst, access)?)
        .then_some(ReachingMemoryState::Unknown),
        StackWriteEffect::Exact {
            location: written,
            value,
        } if written == location => {
            Some(ReachingMemoryState::Exact(func.dfg.resolve_aliases(value)))
        }
        StackWriteEffect::Exact {
            location: written, ..
        } if locations_overlap(location, written) => Some(ReachingMemoryState::Unknown),
        StackWriteEffect::Unknown if unknown_write_may_alias(func, inst, access)? => {
            Some(ReachingMemoryState::Unknown)
        }
        StackWriteEffect::Unknown | StackWriteEffect::Exact { .. } => None,
    })
}

fn reaching_memory_states_before(
    func: &IndexedFunction<'_>,
    location: StackMemoryLocation,
    dependent: cranelift_codegen::ir::Inst,
) -> Result<BTreeSet<ReachingMemoryState>, CraneliftBackendError> {
    // Each state is set outright by the last effect before `dependent`, so the
    // states reaching it are the first effects met walking backward along
    // every path, plus Uninitialized wherever a path reaches the entry first.
    let edges = func.edges()?;
    let access = location_access(func, location)?;
    let block = func
        .layout
        .inst_block(dependent)
        .filter(|block| edges.reachable.contains(block))
        .ok_or_else(|| {
            backend_module(
                LoweringCause::FinishedCode,
                "a memory proof's dependent instruction is unreachable".to_string(),
            )
        })?;
    let mut observations = BTreeSet::new();
    let mut seen = BTreeSet::new();
    let mut pending = vec![(block, func.layout.prev_inst(dependent))];
    while let Some((block, mut cursor)) = pending.pop() {
        let mut decided = None;
        while let Some(inst) = cursor {
            decided = reaching_memory_effect(func, inst, location, &access)?;
            if decided.is_some() {
                break;
            }
            cursor = func.layout.prev_inst(inst);
        }
        if let Some(state) = decided {
            observations.insert(state);
            continue;
        }
        if block == edges.entry {
            observations.insert(ReachingMemoryState::Uninitialized);
        }
        for &(_, _, source) in edges.predecessors.get(&block).into_iter().flatten() {
            if seen.insert(source) {
                pending.push((source, func.layout.last_inst(source)));
            }
        }
    }
    Ok(observations)
}

fn resolve_stack_address(
    func: &IndexedFunction<'_>,
    value: cranelift_codegen::ir::Value,
) -> Result<Option<(cranelift_codegen::ir::StackSlot, i64)>, CraneliftBackendError> {
    if let Some(address) = resolve_stack_address_direct(func, value, &mut BTreeSet::new()) {
        return Ok(Some(address));
    }
    let value = func.dfg.resolve_aliases(value);
    let needs_cfg = match func.dfg.value_def(value) {
        cranelift_codegen::ir::ValueDef::Param(block, _) => {
            func.layout.entry_block() != Some(block)
        }
        cranelift_codegen::ir::ValueDef::Union(_, _) => true,
        cranelift_codegen::ir::ValueDef::Result(inst, 0) => {
            matches!(
                &func.dfg.insts[inst],
                cranelift_codegen::ir::InstructionData::BinaryImm64 { opcode, .. }
                    if *opcode == cranelift_codegen::ir::Opcode::IaddImm
            ) || matches!(
                &func.dfg.insts[inst],
                cranelift_codegen::ir::InstructionData::Binary { opcode, .. }
                    if *opcode == cranelift_codegen::ir::Opcode::Iadd
            )
        }
        cranelift_codegen::ir::ValueDef::Result(_, _) => false,
    };
    if !needs_cfg {
        return Ok(None);
    }
    if let Some(resolved) = func.stack_addresses.borrow().roots.get(&value) {
        return Ok(*resolved);
    }
    // A result whose subtree hit no cycle cut is path-independent, so the
    // memo of such results is shared by every query on this function.
    let mut search = StackAddressSearch {
        resolved: std::mem::take(&mut func.stack_addresses.borrow_mut().cut_free),
        ..StackAddressSearch::default()
    };
    let resolved = resolve_stack_address_inner(func, func.cfg(), value, &mut search);
    let mut cache = func.stack_addresses.borrow_mut();
    cache.cut_free = search.resolved;
    let resolved = resolved?;
    cache.roots.insert(value, resolved);
    Ok(resolved)
}

fn resolve_stack_address_direct(
    func: &Function,
    value: cranelift_codegen::ir::Value,
    visiting: &mut BTreeSet<cranelift_codegen::ir::Value>,
) -> Option<(cranelift_codegen::ir::StackSlot, i64)> {
    let value = func.dfg.resolve_aliases(value);
    if !visiting.insert(value) {
        return None;
    }
    let result = match func.dfg.value_def(value) {
        cranelift_codegen::ir::ValueDef::Result(inst, 0) => match &func.dfg.insts[inst] {
            cranelift_codegen::ir::InstructionData::StackLoad {
                opcode,
                stack_slot,
                offset,
            } if *opcode == cranelift_codegen::ir::Opcode::StackAddr => {
                Some((*stack_slot, i64::from(i32::from(*offset))))
            }
            cranelift_codegen::ir::InstructionData::BinaryImm64 { opcode, arg, imm }
                if *opcode == cranelift_codegen::ir::Opcode::IaddImm =>
            {
                resolve_stack_address_direct(func, *arg, visiting).and_then(|(slot, base)| {
                    base.checked_add(imm.bits()).map(|offset| (slot, offset))
                })
            }
            cranelift_codegen::ir::InstructionData::Binary { opcode, args }
                if *opcode == cranelift_codegen::ir::Opcode::Iadd =>
            {
                if let Some(offset) = iconst_value(func, args[1]) {
                    resolve_stack_address_direct(func, args[0], visiting)
                        .and_then(|(slot, base)| base.checked_add(offset).map(|sum| (slot, sum)))
                } else if let Some(offset) = iconst_value(func, args[0]) {
                    resolve_stack_address_direct(func, args[1], visiting)
                        .and_then(|(slot, base)| base.checked_add(offset).map(|sum| (slot, sum)))
                } else {
                    None
                }
            }
            _ => None,
        },
        _ => None,
    };
    visiting.remove(&value);
    result
}

#[derive(Default)]
struct StackAddressSearch {
    visiting: BTreeSet<cranelift_codegen::ir::Value>,
    resolved:
        BTreeMap<cranelift_codegen::ir::Value, Option<(cranelift_codegen::ir::StackSlot, i64)>>,
    cycle_cut: bool,
}

fn resolve_stack_address_inner(
    func: &Function,
    cfg: &ControlFlowGraph,
    value: cranelift_codegen::ir::Value,
    search: &mut StackAddressSearch,
) -> Result<Option<(cranelift_codegen::ir::StackSlot, i64)>, CraneliftBackendError> {
    let value = func.dfg.resolve_aliases(value);
    if let Some(resolved) = search.resolved.get(&value) {
        return Ok(*resolved);
    }
    if !search.visiting.insert(value) {
        search.cycle_cut = true;
        return Ok(None);
    }
    let outer_cut = std::mem::replace(&mut search.cycle_cut, false);
    let resolved = resolve_stack_address_uncached(func, cfg, value, search);
    search.visiting.remove(&value);
    let resolved = resolved?;
    if !search.cycle_cut {
        search.resolved.insert(value, resolved);
    }
    search.cycle_cut |= outer_cut;
    Ok(resolved)
}

fn resolve_stack_address_uncached(
    func: &Function,
    cfg: &ControlFlowGraph,
    value: cranelift_codegen::ir::Value,
    search: &mut StackAddressSearch,
) -> Result<Option<(cranelift_codegen::ir::StackSlot, i64)>, CraneliftBackendError> {
    let resolved = match func.dfg.value_def(value) {
        cranelift_codegen::ir::ValueDef::Result(inst, 0) => match &func.dfg.insts[inst] {
            cranelift_codegen::ir::InstructionData::StackLoad {
                opcode,
                stack_slot,
                offset,
            } if *opcode == cranelift_codegen::ir::Opcode::StackAddr => {
                Some((*stack_slot, i64::from(i32::from(*offset))))
            }
            cranelift_codegen::ir::InstructionData::BinaryImm64 { opcode, arg, imm }
                if *opcode == cranelift_codegen::ir::Opcode::IaddImm =>
            {
                resolve_stack_address_inner(func, cfg, *arg, search)?.and_then(|(slot, offset)| {
                    offset.checked_add(imm.bits()).map(|offset| (slot, offset))
                })
            }
            cranelift_codegen::ir::InstructionData::Binary { opcode, args }
                if *opcode == cranelift_codegen::ir::Opcode::Iadd =>
            {
                if let Some(offset) = iconst_value(func, args[1]) {
                    resolve_stack_address_inner(func, cfg, args[0], search)?
                        .and_then(|(slot, base)| base.checked_add(offset).map(|sum| (slot, sum)))
                } else if let Some(offset) = iconst_value(func, args[0]) {
                    resolve_stack_address_inner(func, cfg, args[1], search)?
                        .and_then(|(slot, base)| base.checked_add(offset).map(|sum| (slot, sum)))
                } else {
                    None
                }
            }
            _ => None,
        },
        cranelift_codegen::ir::ValueDef::Param(block, index) => {
            let Some(entry) = func.layout.entry_block() else {
                return Ok(None);
            };
            if block == entry {
                None
            } else {
                let mut candidate = None;
                let mut grounded = false;
                for predecessor in cfg.pred_iter(block) {
                    let incoming = incoming_arguments_with_cuts(
                        func,
                        predecessor.inst,
                        block,
                        index,
                        &BTreeSet::new(),
                    )?;
                    if incoming.is_empty() {
                        return Ok(None);
                    }
                    for incoming in incoming {
                        let next = resolve_stack_address_inner(func, cfg, incoming, search)?;
                        let Some(next) = next else {
                            return Ok(None);
                        };
                        grounded = true;
                        if candidate.is_some_and(|held| held != next) {
                            return Ok(None);
                        }
                        candidate = Some(next);
                    }
                }
                grounded.then_some(candidate).flatten()
            }
        }
        cranelift_codegen::ir::ValueDef::Union(left, right) => {
            let left = resolve_stack_address_inner(func, cfg, left, search)?;
            let right = resolve_stack_address_inner(func, cfg, right, search)?;
            (left.is_some() && left == right).then_some(left).flatten()
        }
        cranelift_codegen::ir::ValueDef::Result(_, _) => None,
    };
    Ok(resolved)
}

fn stack_write_effect(
    func: &IndexedFunction<'_>,
    inst: cranelift_codegen::ir::Inst,
) -> Result<StackWriteEffect, CraneliftBackendError> {
    let effect = match &func.dfg.insts[inst] {
        cranelift_codegen::ir::InstructionData::StackStore {
            arg,
            stack_slot,
            offset,
            ..
        } => StackWriteEffect::Exact {
            location: StackMemoryLocation {
                slot: *stack_slot,
                offset: i64::from(i32::from(*offset)),
                bytes: func.dfg.value_type(*arg).bytes(),
            },
            value: *arg,
        },
        cranelift_codegen::ir::InstructionData::Store {
            opcode,
            args,
            offset,
            ..
        } if *opcode == cranelift_codegen::ir::Opcode::Store => {
            match resolve_stack_address(func, args[1])? {
                Some((slot, base)) => StackWriteEffect::Exact {
                    location: StackMemoryLocation {
                        slot,
                        offset: base.saturating_add(i64::from(i32::from(*offset))),
                        bytes: func.dfg.value_type(args[0]).bytes(),
                    },
                    value: args[0],
                },
                None => StackWriteEffect::Unknown,
            }
        }
        cranelift_codegen::ir::InstructionData::StoreNoOffset { args, .. } => {
            match resolve_stack_address(func, args[1])? {
                Some((slot, offset)) => StackWriteEffect::Exact {
                    location: StackMemoryLocation {
                        slot,
                        offset,
                        bytes: func.dfg.value_type(args[0]).bytes(),
                    },
                    value: args[0],
                },
                None => StackWriteEffect::Unknown,
            }
        }
        // A narrow store writes part of its operand and an atomic write stores
        // a word no operand names, so neither establishes exact contents.
        data if data.opcode().can_store() => StackWriteEffect::Unknown,
        _ => StackWriteEffect::None,
    };
    Ok(effect)
}

fn status_zero_guards(
    func: &IndexedFunction<'_>,
    status: cranelift_codegen::ir::Value,
) -> Result<Vec<ExactProtocolGuard>, CraneliftBackendError> {
    let mut guards = Vec::new();
    for comparison in func.comparisons_of(func.dfg.resolve_aliases(status)) {
        if !(comparison.immediate
            && comparison.operand == status
            && comparison.condition == cranelift_codegen::ir::condcodes::IntCC::Equal
            && comparison.constant == 0)
        {
            continue;
        }
        for &branch in func.branches_on(comparison.comparison) {
            let cranelift_codegen::ir::InstructionData::Brif { blocks, .. } =
                &func.dfg.insts[branch]
            else {
                continue;
            };
            guards.push(ExactProtocolGuard {
                branch,
                success_ordinal: 0,
                success_target: blocks[0].block(&func.dfg.value_lists),
            });
        }
    }
    if guards.is_empty() {
        Err(backend_module(
            LoweringCause::FinishedCode,
            "a helper result has no exact status-zero guard".to_string(),
        ))
    } else {
        Ok(guards)
    }
}

fn successor_edges(
    func: &Function,
    block: Block,
) -> Result<Vec<(cranelift_codegen::ir::Inst, u8, Block)>, CraneliftBackendError> {
    let terminator = func.layout.last_inst(block).ok_or_else(|| {
        backend_module(
            LoweringCause::FinishedCode,
            "a staged Result CFG block has no terminator".to_string(),
        )
    })?;
    let data = &func.dfg.insts[terminator];
    if data.opcode().is_branch() {
        return data
            .branch_destination(&func.dfg.jump_tables)
            .iter()
            .enumerate()
            .map(|(ordinal, destination)| {
                Ok((
                    terminator,
                    u8::try_from(ordinal).map_err(|_| {
                        backend_module(
                            LoweringCause::FinishedCode,
                            "a staged Result branch has more than 256 successors".to_string(),
                        )
                    })?,
                    destination.block(&func.dfg.value_lists),
                ))
            })
            .collect();
    }
    if data.opcode().is_terminator() {
        return Ok(Vec::new());
    }
    Err(backend_module(
        LoweringCause::FinishedCode,
        format!(
            "a staged Result CFG block ends in unsupported non-terminator {:?}",
            data.opcode()
        ),
    ))
}

fn unknown_write_may_alias(
    func: &IndexedFunction<'_>,
    inst: cranelift_codegen::ir::Inst,
    access: &LocationAccess,
) -> Result<bool, CraneliftBackendError> {
    let address = match &func.dfg.insts[inst] {
        cranelift_codegen::ir::InstructionData::Store { args, .. }
        | cranelift_codegen::ir::InstructionData::StoreNoOffset { args, .. } => Some(args[1]),
        cranelift_codegen::ir::InstructionData::AtomicCas { args, .. } => Some(args[0]),
        cranelift_codegen::ir::InstructionData::AtomicRmw { args, .. } => Some(args[0]),
        _ => None,
    };
    let Some(address) = address else {
        return Ok(true);
    };
    Ok(access.escaped || address_depends_on_slots(func, address, &access.containers)?)
}

fn value_is_guarded_nonzero_at_inst(
    func: &IndexedFunction<'_>,
    value: cranelift_codegen::ir::Value,
    dependent: cranelift_codegen::ir::Inst,
) -> bool {
    let value = func.dfg.resolve_aliases(value);
    let cranelift_codegen::ir::ValueDef::Result(producer, 0) = func.dfg.value_def(value) else {
        return false;
    };
    func.comparisons_of(value).iter().any(|comparison| {
        let nonzero_guard = (comparison.immediate
            && comparison.operand == value
            && comparison.condition == cranelift_codegen::ir::condcodes::IntCC::NotEqual
            && comparison.constant == 0)
            || (comparison.condition == cranelift_codegen::ir::condcodes::IntCC::Equal
                && comparison.constant != 0);
        nonzero_guard
            && func
                .branches_on(comparison.comparison)
                .iter()
                .any(|&branch| {
                    let cranelift_codegen::ir::InstructionData::Brif { blocks, .. } =
                        &func.dfg.insts[branch]
                    else {
                        return false;
                    };
                    verify_all_paths_guarded(
                        func,
                        producer,
                        ExactProtocolGuard {
                            branch,
                            success_ordinal: 0,
                            success_target: blocks[0].block(&func.dfg.value_lists),
                        },
                        dependent,
                        "a nonzero guarded terminal",
                    )
                    .is_ok()
                })
    })
}

fn value_is_provably_nonzero(func: &Function, value: cranelift_codegen::ir::Value) -> bool {
    if iconst_value(func, value).is_some_and(|constant| constant != 0) {
        return true;
    }
    let cranelift_codegen::ir::ValueDef::Result(inst, 0) = func.dfg.value_def(value) else {
        return false;
    };
    match &func.dfg.insts[inst] {
        cranelift_codegen::ir::InstructionData::BinaryImm64 { opcode, imm, .. }
            if *opcode == cranelift_codegen::ir::Opcode::BorImm && imm.bits() != 0 =>
        {
            true
        }
        cranelift_codegen::ir::InstructionData::Unary { opcode, arg }
            if *opcode == cranelift_codegen::ir::Opcode::Ineg =>
        {
            value_is_provably_nonzero(func, *arg)
        }
        _ => false,
    }
}

fn verify_all_paths_after_instruction(
    func: &IndexedFunction<'_>,
    producer: cranelift_codegen::ir::Inst,
    dependent: cranelift_codegen::ir::Inst,
    label: &str,
) -> Result<(), CraneliftBackendError> {
    let verdict = verify_all_paths_backward(
        func,
        &[false, true],
        false,
        dependent,
        true,
        |current, inst| Ok(current || inst == producer),
        |current, _, _, _| Ok(current),
    )?;
    all_paths_result(verdict, label, || {
        format!("{label} is reachable before its required finalized instruction")
    })
}

fn verify_all_paths_guarded(
    func: &IndexedFunction<'_>,
    producer: cranelift_codegen::ir::Inst,
    guard: ExactProtocolGuard,
    dependent: cranelift_codegen::ir::Inst,
    label: &str,
) -> Result<(), CraneliftBackendError> {
    exact_guard_edge(func, guard)?;
    let verdict = verify_all_paths_backward(
        func,
        &[
            ProtocolFactState::Absent,
            ProtocolFactState::Produced,
            ProtocolFactState::Guarded,
        ],
        ProtocolFactState::Absent,
        dependent,
        ProtocolFactState::Guarded,
        |state, inst| {
            Ok(if inst == producer {
                ProtocolFactState::Produced
            } else {
                state
            })
        },
        |state, terminator, ordinal, target| {
            Ok(if terminator != guard.branch {
                state
            } else if ordinal == guard.success_ordinal && target == guard.success_target {
                match state {
                    ProtocolFactState::Produced | ProtocolFactState::Guarded => {
                        ProtocolFactState::Guarded
                    }
                    ProtocolFactState::Absent => ProtocolFactState::Absent,
                }
            } else {
                ProtocolFactState::Absent
            })
        },
    )?;
    all_paths_result(verdict, label, || {
        format!("{label} is reachable without traversing its current successful guard")
    })
}

fn verify_all_paths_memory_current(
    func: &IndexedFunction<'_>,
    producer: cranelift_codegen::ir::Inst,
    location: StackMemoryLocation,
    dependent: cranelift_codegen::ir::Inst,
    label: &str,
) -> Result<(), CraneliftBackendError> {
    let access = location_access(func, location)?;
    let mut clobbers = BTreeMap::new();
    let verdict = verify_all_paths_backward(
        func,
        &[MemoryFlowState::Absent, MemoryFlowState::Current],
        MemoryFlowState::Absent,
        dependent,
        MemoryFlowState::Current,
        |state, inst| {
            if inst == producer {
                return Ok(MemoryFlowState::Current);
            }
            let clobbered = match clobbers.get(&inst) {
                Some(clobbered) => *clobbered,
                None => {
                    let clobbered = instruction_clobbers_location(func, inst, location, &access)?;
                    clobbers.insert(inst, clobbered);
                    clobbered
                }
            };
            Ok(if clobbered {
                MemoryFlowState::Absent
            } else {
                state
            })
        },
        |state, _, _, _| Ok(state),
    )?;
    all_paths_result(verdict, label, || {
        format!("{label} is reachable without the current producer or after a clobber")
    })
}

fn carrier_pending_state(index: usize) -> Result<CarrierFlowState, CraneliftBackendError> {
    u16::try_from(index)
        .map(CarrierFlowState::Pending)
        .map_err(|_| {
            backend_module(
                LoweringCause::FinishedCode,
                "a carrier proof has more than 65536 grounds".to_string(),
            )
        })
}

fn verify_carrier_fact_current(
    func: &IndexedFunction<'_>,
    grounds: &[CarrierFactGround],
    arena: cranelift_codegen::ir::Value,
    tracked_words: &BTreeSet<cranelift_codegen::ir::Value>,
    tag_preserving_helpers: &BTreeSet<FuncId>,
    dependent: cranelift_codegen::ir::Inst,
    label: &str,
) -> Result<(), CraneliftBackendError> {
    if grounds.is_empty() {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            format!("{label} has no finalized ground"),
        ));
    }
    let mut states = vec![CarrierFlowState::Absent, CarrierFlowState::Current];
    states.extend(
        (0..=u16::MAX)
            .take(grounds.len())
            .map(CarrierFlowState::Pending),
    );
    let verdict = verify_all_paths_backward(
        func,
        &states,
        CarrierFlowState::Absent,
        dependent,
        CarrierFlowState::Current,
        |state, inst| {
            if let Some(index) = grounds.iter().position(|ground| ground.producer == inst) {
                return if grounds[index].guard.is_some() {
                    carrier_pending_state(index)
                } else {
                    Ok(CarrierFlowState::Current)
                };
            }
            if func.dfg.insts[inst].opcode().is_call()
                && func
                    .dfg
                    .inst_args(inst)
                    .iter()
                    .any(|argument| *argument == arena || tracked_words.contains(argument))
            {
                let target = Lowering::decode_direct_callee(func, inst).ok();
                if !target.is_some_and(|target| tag_preserving_helpers.contains(&target)) {
                    return Ok(CarrierFlowState::Absent);
                }
            }
            Ok(state)
        },
        |state, terminator, ordinal, target| {
            let mut outgoing = state;
            for (index, ground) in grounds.iter().enumerate() {
                let Some(guard) = ground.guard else {
                    continue;
                };
                if guard.branch != terminator {
                    continue;
                }
                let pending_index = carrier_pending_state(index)?;
                if state == pending_index
                    && ordinal == guard.success_ordinal
                    && target == guard.success_target
                {
                    outgoing = CarrierFlowState::Current;
                } else if state == pending_index {
                    outgoing = CarrierFlowState::Absent;
                }
            }
            Ok(outgoing)
        },
    )?;
    all_paths_result(verdict, label, || {
        format!("{label} is reachable without a current finalized constructor fact")
    })
}

fn verify_carrier_queries(
    func: &IndexedFunction<'_>,
    helper: FuncId,
    arena: cranelift_codegen::ir::Value,
) -> Result<Vec<VerifiedCarrierQuery>, CraneliftBackendError> {
    let mut verified = Vec::new();
    for &(call, callee) in func.calls() {
        let callee = match callee {
            Some(callee) => callee,
            None => Lowering::decode_direct_callee(func, call)?,
        };
        if callee != helper {
            continue;
        }
        let [actual_arena, word, out] = func.dfg.inst_args(call) else {
            return Err(backend_module(
                LoweringCause::FinishedCode,
                "a certified carrier query does not have exactly arena, word and out operands"
                    .to_string(),
            ));
        };
        if *actual_arena != arena {
            return Err(backend_module(
                LoweringCause::FinishedCode,
                "a certified carrier query uses a different boundary arena".to_string(),
            ));
        }
        let Some((output_slot, output_offset)) = resolve_stack_address(func, *out)? else {
            return Err(backend_module(
                LoweringCause::FinishedCode,
                "a certified carrier query's out operand is not an exact stack address".to_string(),
            ));
        };
        if output_offset != 0 {
            return Err(backend_module(
                LoweringCause::FinishedCode,
                "a certified carrier query's out address is not the start of its slot".to_string(),
            ));
        }
        let output_location = StackMemoryLocation {
            slot: output_slot,
            offset: 0,
            bytes: 8,
        };
        let [status] = func.dfg.inst_results(call) else {
            return Err(backend_module(
                LoweringCause::FinishedCode,
                "a certified carrier query does not return exactly one status".to_string(),
            ));
        };
        let guards = status_zero_guards(func, *status)?;
        let mut outputs = Vec::new();
        for &(load, output) in func.loads_of(output_location)? {
            let applicable = guards
                .iter()
                .copied()
                .filter(|guard| {
                    verify_all_paths_guarded(
                        func,
                        call,
                        *guard,
                        load,
                        "a certified carrier query output load",
                    )
                    .is_ok()
                })
                .collect::<Vec<_>>();
            if applicable.len() != 1 {
                return Err(backend_module(
                    LoweringCause::FinishedCode,
                    format!(
                        "a certified carrier query output load has {} all-path status guards instead of one",
                        applicable.len()
                    ),
                ));
            }
            verify_all_paths_memory_current(
                func,
                call,
                output_location,
                load,
                "a certified carrier query output load",
            )?;
            outputs.push(output);
        }
        let [output] = outputs.as_slice() else {
            return Err(backend_module(
                LoweringCause::FinishedCode,
                format!(
                    "a certified carrier query has {} all-path guarded output loads instead of one",
                    outputs.len()
                ),
            ));
        };
        verified.push(VerifiedCarrierQuery {
            call,
            word: *word,
            output: *output,
        });
    }
    Ok(verified)
}

fn verify_constructor_authority_ground(
    body: &StagedResultBody,
    func: &IndexedFunction<'_>,
    authority: &GeneratedConstructorAuthority,
    helpers: &crate::boundary_value_clif::BoundaryLocalFuncs,
) -> Result<CarrierFactGround, CraneliftBackendError> {
    let arena = body.boundary_arena.ok_or_else(|| {
        backend_module(
            LoweringCause::FinishedCode,
            "a generated constructor authority has no boundary arena".to_string(),
        )
    })?;
    let (word_load, word_location) =
        loaded_stack_location(func, authority.word)?.ok_or_else(|| {
            backend_module(
                LoweringCause::FinishedCode,
                "a generated constructor authority's word is not an actual helper-output load"
                    .to_string(),
            )
        })?;
    let mut allocation_calls = Vec::new();
    for &(call, callee) in func.calls() {
        if callee != Some(helpers.alloc) {
            continue;
        }
        let args = func.dfg.inst_args(call);
        let Some(out) = args.last().copied() else {
            continue;
        };
        if args.first().copied() != Some(arena)
            || resolve_stack_address(func, out)? != Some((word_location.slot, word_location.offset))
        {
            continue;
        }
        let [status] = func.dfg.inst_results(call) else {
            continue;
        };
        let applicable = status_zero_guards(func, *status)?
            .into_iter()
            .filter(|guard| {
                verify_all_paths_guarded(
                    func,
                    call,
                    *guard,
                    word_load,
                    "a generated constructor allocation result",
                )
                .is_ok()
            })
            .collect::<Vec<_>>();
        if applicable.len() == 1
            && verify_all_paths_memory_current(
                func,
                call,
                word_location,
                word_load,
                "a generated constructor allocation result",
            )
            .is_ok()
        {
            allocation_calls.push(call);
        }
    }
    if allocation_calls.len() != 1 {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            format!(
                "a generated constructor authority has {} exact finalized allocation producers instead of one",
                allocation_calls.len()
            ),
        ));
    }

    let expected_tag = i64::try_from(authority.identity.tag_abi_word()?).map_err(|_| {
        backend_module(
            LoweringCause::FinishedCode,
            "a generated constructor identity exceeds the runtime tag word".to_string(),
        )
    })?;
    let mut tag_grounds = Vec::new();
    for &(call, callee) in func.calls() {
        if callee != Some(helpers.store_tag_id) {
            continue;
        }
        let args = func.dfg.inst_args(call);
        let [actual_arena, word, tag] = args else {
            continue;
        };
        if *actual_arena != arena
            || *word != authority.word
            || iconst_value(func, *tag) != Some(expected_tag)
        {
            continue;
        }
        verify_all_paths_after_instruction(
            func,
            word_load,
            call,
            "a generated constructor tag store",
        )?;
        let [status] = func.dfg.inst_results(call) else {
            continue;
        };
        for guard in status_zero_guards(func, *status)? {
            tag_grounds.push(CarrierFactGround {
                producer: call,
                guard: Some(guard),
            });
        }
    }
    let [ground] = tag_grounds.as_slice() else {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            format!(
                "a generated constructor authority has {} exact finalized tag-store guards instead of one",
                tag_grounds.len()
            ),
        ));
    };
    Ok(*ground)
}

fn verify_cut_provenance(
    body: &StagedResultBody,
    func: &IndexedFunction<'_>,
    cuts: &CertifiedCuts,
) -> Result<(), CraneliftBackendError> {
    if cuts.edges.len() != cuts.provenance.len()
        || cuts
            .edges
            .iter()
            .any(|edge| !cuts.provenance.contains_key(edge))
    {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "the certified cut edge set disagrees with its proof provenance".to_string(),
        ));
    }
    for (edge, proof) in &cuts.provenance {
        if proof.unit != body.unit
            || proof.function != body.target
            || proof.sources.is_empty()
            || proof.dependencies.contains(edge)
            || proof
                .dependencies
                .iter()
                .any(|dependency| !cuts.provenance.contains_key(dependency))
        {
            return Err(backend_module(
                LoweringCause::FinishedCode,
                "a certified cut carries foreign, empty, circular, or missing provenance"
                    .to_string(),
            ));
        }
        exact_guard_edge(
            func,
            ExactProtocolGuard {
                branch: edge.terminator,
                success_ordinal: edge.destination_ordinal,
                success_target: edge.target,
            },
        )?;
        for inst in [
            proof.query_call,
            proof.field_count_call,
            proof.comparison,
            edge.terminator,
        ] {
            if func.layout.inst_block(inst).is_none() {
                return Err(backend_module(
                    LoweringCause::FinishedCode,
                    "a certified cut proof names an instruction absent from finalized CLIF"
                        .to_string(),
                ));
            }
        }
        let cranelift_codegen::ir::InstructionData::Brif { arg, .. } =
            &func.dfg.insts[edge.terminator]
        else {
            return Err(backend_module(
                LoweringCause::FinishedCode,
                "a certified cut terminator is not its finalized conditional branch".to_string(),
            ));
        };
        let Some((condition, compared, constant)) = comparison_with_constant(func, *arg) else {
            return Err(backend_module(
                LoweringCause::FinishedCode,
                "a certified cut no longer has its exact tag comparison".to_string(),
            ));
        };
        let expected = i64::try_from(proof.identity.tag_abi_word()?).map_err(|_| {
            backend_module(
                LoweringCause::FinishedCode,
                "a certified cut identity exceeds the runtime tag word".to_string(),
            )
        })?;
        let expected_infeasible_ordinal = match condition {
            cranelift_codegen::ir::condcodes::IntCC::Equal => 1,
            cranelift_codegen::ir::condcodes::IntCC::NotEqual => 0,
            _ => u8::MAX,
        };
        if compared != proof.observed_tag
            || constant != expected
            || edge.destination_ordinal != expected_infeasible_ordinal
            || func.dfg.inst_args(proof.query_call).get(1).copied() != Some(proof.queried_word)
            || func.dfg.inst_args(proof.field_count_call).get(1).copied()
                != Some(proof.queried_word)
        {
            return Err(backend_module(
                LoweringCause::FinishedCode,
                "a certified cut's finalized word/query/comparison provenance disagrees"
                    .to_string(),
            ));
        }
    }

    fn visit(
        edge: CertifiedInfeasibleEdge,
        cuts: &CertifiedCuts,
        visiting: &mut BTreeSet<CertifiedInfeasibleEdge>,
        done: &mut BTreeSet<CertifiedInfeasibleEdge>,
    ) -> bool {
        if done.contains(&edge) {
            return true;
        }
        if !visiting.insert(edge) {
            return false;
        }
        let valid = cuts.provenance[&edge]
            .dependencies
            .iter()
            .copied()
            .all(|dependency| visit(dependency, cuts, visiting, done));
        visiting.remove(&edge);
        if valid {
            done.insert(edge);
        }
        valid
    }
    let mut done = BTreeSet::new();
    for edge in &cuts.edges {
        if !visit(*edge, cuts, &mut BTreeSet::new(), &mut done) {
            return Err(backend_module(
                LoweringCause::FinishedCode,
                "a certified cut proof contains a pruning dependency cycle".to_string(),
            ));
        }
    }
    Ok(())
}

fn verify_definite_stored_value(
    func: &IndexedFunction<'_>,
    location: StackMemoryLocation,
    dependent: cranelift_codegen::ir::Inst,
    label: &str,
) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
    let states = reaching_memory_states_before(func, location, dependent)?;
    if states.len() != 1 {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            format!("{label} has no single reaching initialized value on every path: {states:?}"),
        ));
    }
    match states.iter().next().copied() {
        Some(ReachingMemoryState::Exact(value)) => Ok(value),
        _ => Err(backend_module(
            LoweringCause::FinishedCode,
            format!("{label} has no single reaching initialized value on every path: {states:?}"),
        )),
    }
}

fn verify_exact_value_guard(
    func: &IndexedFunction<'_>,
    value: cranelift_codegen::ir::Value,
    expected: i64,
    dependent: cranelift_codegen::ir::Inst,
) -> Result<(), CraneliftBackendError> {
    let value = func.dfg.resolve_aliases(value);
    let cranelift_codegen::ir::ValueDef::Result(producer, 0) = func.dfg.value_def(value) else {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "an exact-value proof's observed word has no finalized producer instruction"
                .to_string(),
        ));
    };
    let mut applicable = 0usize;
    for comparison in func.comparisons_of(value) {
        if !(comparison.immediate
            && comparison.operand == value
            && comparison.condition == cranelift_codegen::ir::condcodes::IntCC::Equal
            && comparison.constant == expected)
        {
            continue;
        }
        for &branch in func.branches_on(comparison.comparison) {
            let cranelift_codegen::ir::InstructionData::Brif { blocks, .. } =
                &func.dfg.insts[branch]
            else {
                continue;
            };
            let guard = ExactProtocolGuard {
                branch,
                success_ordinal: 0,
                success_target: blocks[0].block(&func.dfg.value_lists),
            };
            if verify_all_paths_guarded(
                func,
                producer,
                guard,
                dependent,
                "a finished Result exact-value dependent",
            )
            .is_ok()
            {
                applicable += 1;
            }
        }
    }
    if applicable == 1 {
        Ok(())
    } else {
        Err(backend_module(
            LoweringCause::FinishedCode,
            format!(
                "a finished Result publication has {applicable} all-path exact guards for expected word {expected} instead of one"
            ),
        ))
    }
}

fn verify_no_memory_effect_after_instruction(
    func: &IndexedFunction<'_>,
    producer: cranelift_codegen::ir::Inst,
    dependent: cranelift_codegen::ir::Inst,
    label: &str,
) -> Result<(), CraneliftBackendError> {
    let verdict = verify_all_paths_backward(
        func,
        &[false, true],
        false,
        dependent,
        true,
        |current, inst| {
            Ok(if inst == producer {
                true
            } else if current
                && (func.dfg.insts[inst].opcode().is_call()
                    || !matches!(stack_write_effect(func, inst)?, StackWriteEffect::None))
            {
                false
            } else {
                current
            })
        },
        |current, _, _, _| Ok(current),
    )?;
    all_paths_result(verdict, label, || {
        format!("{label} is reachable after an intervening memory effect")
    })
}

fn verify_publication_and_terminals(
    func: &IndexedFunction<'_>,
    publication: FunctionResultPublication,
    _call_obligations_verified: bool,
) -> Result<(), CraneliftBackendError> {
    match &func.dfg.insts[publication.result_store] {
        cranelift_codegen::ir::InstructionData::Store {
            opcode,
            args,
            offset,
            ..
        } if *opcode == cranelift_codegen::ir::Opcode::Store
            && args[0] == publication.returned_word
            && args[1] == publication.frame
            && i32::from(*offset) == publication.result_offset => {}
        _ => {
            return Err(backend_module(
                LoweringCause::FinishedCode,
                "a finished Result certificate does not bind its exact word/frame/store"
                    .to_string(),
            ));
        }
    }
    verify_all_paths_after_instruction(
        func,
        publication.result_store,
        publication.return_inst,
        "a finished Result status-zero publication",
    )?;
    verify_no_memory_effect_after_instruction(
        func,
        publication.result_store,
        publication.return_inst,
        "a finished Result status-zero publication",
    )?;
    if func.dfg.insts[publication.return_inst].opcode() != cranelift_codegen::ir::Opcode::Return
        || func
            .dfg
            .inst_args(publication.return_inst)
            .first()
            .and_then(|word| iconst_value(func, *word))
            != Some(0)
    {
        return Err(backend_module(
            LoweringCause::FinishedCode,
            "a finished Result publication does not return status zero".to_string(),
        ));
    }
    for return_inst in func
        .layout
        .blocks()
        .flat_map(|block| func.layout.block_insts(block))
        .filter(|inst| func.dfg.insts[*inst].opcode() == cranelift_codegen::ir::Opcode::Return)
    {
        if return_inst == publication.return_inst {
            continue;
        }
        let Some(returned) = func.dfg.inst_args(return_inst).first().copied() else {
            return Err(backend_module(
                LoweringCause::FinishedCode,
                "a finished Result function has a terminal with no status".to_string(),
            ));
        };
        if value_is_provably_nonzero(func, returned) {
            continue;
        }
        let return_block = func.layout.inst_block(return_inst).ok_or_else(|| {
            backend_module(
                LoweringCause::FinishedCode,
                "a finished Result terminal is absent from the layout".to_string(),
            )
        })?;
        if value_is_guarded_nonzero_at_inst(func, returned, return_inst) {
            continue;
        }
        let explicit_trap = iconst_value(func, returned) == Some(0)
            && func.layout.block_insts(return_block).any(|inst| {
                let cranelift_codegen::ir::InstructionData::Store {
                    opcode,
                    args,
                    offset,
                    ..
                } = &func.dfg.insts[inst]
                else {
                    return false;
                };
                *opcode == cranelift_codegen::ir::Opcode::Store
                    && args[1] == publication.frame
                    && i32::from(*offset) == publication.trap_offset
                    && (value_is_provably_nonzero(func, args[0])
                        || value_is_guarded_nonzero_at_inst(func, args[0], inst))
                    && verify_all_paths_after_instruction(
                        func,
                        inst,
                        return_inst,
                        "a finished Result Trap publication",
                    )
                    .is_ok()
            });
        if !explicit_trap {
            let block = func
                .layout
                .inst_block(return_inst)
                .expect("the return block was checked above");
            let instructions = func
                .layout
                .block_insts(block)
                .map(|inst| format!("{inst:?}:{:?}", func.dfg.insts[inst]))
                .collect::<Vec<_>>();
            return Err(backend_module(
                LoweringCause::FinishedCode,
                format!(
                    "a finished Result function has an uncertified status-zero terminal \
                 {return_inst:?} in {block:?}, returned={returned:?}, \
                 definition={:?}, producer={:?}: {instructions:?}",
                    func.dfg.value_def(returned),
                    match func.dfg.value_def(returned) {
                        cranelift_codegen::ir::ValueDef::Result(inst, _) =>
                            Some(&func.dfg.insts[inst]),
                        _ => None,
                    },
                ),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod generated_result_protocol_verifier {
    use super::*;
    use cranelift_codegen::ir::{
        AbiParam, BlockCall, ExtFuncData, ExternalName, JumpTableData, Signature, UserExternalName,
        condcodes::IntCC,
    };
    use cranelift_codegen::isa::CallConv;

    #[derive(Clone, Copy)]
    enum CallProtocolMutation {
        Exact,
        TrapStatusBypass,
        ResultTrapBypass,
        ForeignHeaderPayload,
        InitializerBypass,
        RawHeaderClobber,
        RawTrapClobber,
        RawResultClobber,
    }

    fn direct_callee(func: &mut Function, target: FuncId, parameters: usize) -> FuncRef {
        let mut signature = Signature::new(CallConv::SystemV);
        signature
            .params
            .extend((0..parameters).map(|_| AbiParam::new(types::I64)));
        signature.returns.push(AbiParam::new(types::I64));
        let signature = func.import_signature(signature);
        let name = func.params.ensure_user_func_name(UserExternalName {
            namespace: 0,
            index: target.as_u32(),
        });
        func.import_function(ExtFuncData {
            name: ExternalName::User(name),
            signature,
            colocated: true,
        })
    }

    fn finish_return(builder: &mut FunctionBuilder<'_>, value: cranelift_codegen::ir::Value) {
        builder.ins().return_(&[value]);
    }

    fn call_protocol_fixture(
        mutation: CallProtocolMutation,
    ) -> (Function, PendingCallResultObligation, FuncId) {
        let target = FuncId::from_u32(77);
        let mut func = Function::new();
        func.signature.call_conv = CallConv::SystemV;
        func.signature.returns.push(AbiParam::new(types::I64));
        let callee = direct_callee(&mut func, target, 2);
        let payload =
            func.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 24, 3));
        let foreign_payload =
            func.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 24, 3));
        let header = func.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            u32::try_from(crate::activation_services::UNIT_CALL_FRAME_BYTES).unwrap(),
            3,
        ));
        let mut context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut func, &mut context);
        let entry = builder.create_block();
        let initialize = builder.create_block();
        let call_block = builder.create_block();
        let status_gate = builder.create_block();
        let status_failure = builder.create_block();
        let trap_entry = builder.create_block();
        let trap_read = builder.create_block();
        let trap_failure = builder.create_block();
        let result_block = builder.create_block();
        builder.switch_to_block(entry);
        let selector = builder.ins().iconst(types::I64, 0);
        if matches!(mutation, CallProtocolMutation::InitializerBypass) {
            builder
                .ins()
                .brif(selector, call_block, &[], initialize, &[]);
        } else {
            let zero = builder.ins().iconst(types::I64, 0);
            builder.ins().stack_store(zero, payload, 0);
            builder.ins().jump(call_block, &[]);
        }
        builder.switch_to_block(initialize);
        let zero = builder.ins().iconst(types::I64, 0);
        builder.ins().stack_store(zero, payload, 0);
        builder.ins().jump(call_block, &[]);

        builder.switch_to_block(call_block);
        let payload_address = builder.ins().stack_addr(types::I64, payload, 0);
        let foreign_address = builder.ins().stack_addr(types::I64, foreign_payload, 0);
        let stored_payload = if matches!(mutation, CallProtocolMutation::ForeignHeaderPayload) {
            foreign_address
        } else {
            payload_address
        };
        builder.ins().stack_store(
            stored_payload,
            header,
            crate::activation_services::UNIT_CALL_FRAME_SLOTS,
        );
        let header_address = builder.ins().stack_addr(types::I64, header, 0);
        if matches!(mutation, CallProtocolMutation::RawHeaderClobber) {
            builder.ins().store(
                MemFlags::trusted(),
                foreign_address,
                header_address,
                crate::activation_services::UNIT_CALL_FRAME_SLOTS,
            );
        }
        let services = builder.ins().iconst(types::I64, 0);
        let call = builder.ins().call(callee, &[header_address, services]);
        let status = builder.inst_results(call)[0];
        let status_compare = builder.ins().icmp_imm(IntCC::NotEqual, status, 0);
        if matches!(mutation, CallProtocolMutation::TrapStatusBypass) {
            builder
                .ins()
                .brif(selector, trap_read, &[], status_gate, &[]);
        } else {
            builder.ins().jump(status_gate, &[]);
        }

        builder.switch_to_block(status_gate);
        let status_branch =
            builder
                .ins()
                .brif(status_compare, status_failure, &[], trap_entry, &[]);
        builder.switch_to_block(status_failure);
        finish_return(&mut builder, status);

        builder.switch_to_block(trap_entry);
        if matches!(mutation, CallProtocolMutation::ResultTrapBypass) {
            builder
                .ins()
                .brif(selector, result_block, &[], trap_read, &[]);
        } else {
            builder.ins().jump(trap_read, &[]);
        }
        builder.switch_to_block(trap_read);
        if matches!(mutation, CallProtocolMutation::RawTrapClobber) {
            let stale = builder.ins().iconst(types::I64, 9);
            builder
                .ins()
                .store(MemFlags::trusted(), stale, payload_address, 0);
        }
        let trap_word = builder.ins().stack_load(types::I64, payload, 0);
        let trap_compare = builder.ins().icmp_imm(IntCC::NotEqual, trap_word, 0);
        let trap_branch = builder
            .ins()
            .brif(trap_compare, trap_failure, &[], result_block, &[]);
        builder.switch_to_block(trap_failure);
        let trapped = builder.ins().iconst(types::I64, 0);
        finish_return(&mut builder, trapped);

        builder.switch_to_block(result_block);
        if matches!(mutation, CallProtocolMutation::RawResultClobber) {
            let stale = builder.ins().iconst(types::I64, 9);
            builder
                .ins()
                .store(MemFlags::trusted(), stale, payload_address, 8);
        }
        let result_word = builder.ins().stack_load(types::I64, payload, 8);
        finish_return(&mut builder, result_word);
        builder.seal_all_blocks();
        builder.finalize();
        let obligation = PendingCallResultObligation {
            identity: None,
            realization_required: false,
            call,
            payload,
            status,
            status_compare,
            status_branch,
            trap_word,
            trap_compare,
            trap_branch,
            result_word,
            frame_bytes: 24,
            trap_offset: 0,
            result_offset: 8,
        };
        (func, obligation, target)
    }

    #[derive(Clone, Copy)]
    enum QueryMutation {
        Exact,
        GuardBypass,
        BranchTableBypass,
        RawOutputClobber,
        LaterProducerBypass,
        Subdivided,
    }

    fn query_fixture(mutation: QueryMutation) -> (Function, FuncId, cranelift_codegen::ir::Value) {
        let helper = FuncId::from_u32(88);
        let mut func = Function::new();
        func.signature.call_conv = CallConv::SystemV;
        func.signature.returns.push(AbiParam::new(types::I64));
        let callee = direct_callee(&mut func, helper, 3);
        let output =
            func.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 8, 3));
        let mut context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut func, &mut context);
        let entry = builder.create_block();
        let guard = builder.create_block();
        let success = builder.create_block();
        let middle = builder.create_block();
        let load = builder.create_block();
        let failure = builder.create_block();
        let later_producer = builder.create_block();
        builder.switch_to_block(entry);
        let arena = builder.ins().iconst(types::I64, 11);
        let word = builder.ins().iconst(types::I64, 22);
        let out = builder.ins().stack_addr(types::I64, output, 0);
        let call = builder.ins().call(callee, &[arena, word, out]);
        let status = builder.inst_results(call)[0];
        let selector = builder.ins().iconst(types::I64, 0);
        match mutation {
            QueryMutation::GuardBypass => {
                builder.ins().brif(selector, load, &[], guard, &[]);
            }
            QueryMutation::BranchTableBypass => {
                let default = BlockCall::new(guard, &[], &mut builder.func.dfg.value_lists);
                let bypass = BlockCall::new(load, &[], &mut builder.func.dfg.value_lists);
                let table = builder.create_jump_table(JumpTableData::new(default, &[bypass]));
                builder.ins().br_table(selector, table);
            }
            _ => {
                builder.ins().jump(guard, &[]);
            }
        }
        builder.switch_to_block(guard);
        let comparison = builder.ins().icmp_imm(IntCC::Equal, status, 0);
        builder.ins().brif(comparison, success, &[], failure, &[]);
        builder.switch_to_block(failure);
        let failed = builder.ins().iconst(types::I64, -1);
        finish_return(&mut builder, failed);

        builder.switch_to_block(success);
        match mutation {
            QueryMutation::Subdivided => builder.ins().jump(middle, &[]),
            QueryMutation::LaterProducerBypass => builder.ins().jump(later_producer, &[]),
            _ => builder.ins().jump(load, &[]),
        };
        builder.switch_to_block(middle);
        builder.ins().jump(load, &[]);
        builder.switch_to_block(later_producer);
        if matches!(mutation, QueryMutation::LaterProducerBypass) {
            let _second = builder.ins().call(callee, &[arena, word, out]);
            builder.ins().jump(load, &[]);
        } else {
            let unreachable_status = builder.ins().iconst(types::I64, -1);
            finish_return(&mut builder, unreachable_status);
        }

        builder.switch_to_block(load);
        if matches!(mutation, QueryMutation::RawOutputClobber) {
            let stale = builder.ins().iconst(types::I64, 7);
            builder.ins().store(MemFlags::trusted(), stale, out, 0);
        }
        let observed = builder.ins().stack_load(types::I64, output, 0);
        finish_return(&mut builder, observed);
        builder.seal_all_blocks();
        builder.finalize();
        (func, helper, arena)
    }

    fn backedge_guard_fixture(
        stale_backedge: bool,
    ) -> (
        Function,
        cranelift_codegen::ir::Inst,
        ExactProtocolGuard,
        cranelift_codegen::ir::Inst,
    ) {
        let mut func = Function::new();
        func.signature.call_conv = CallConv::SystemV;
        func.signature.returns.push(AbiParam::new(types::I64));
        let mut context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut func, &mut context);
        let entry = builder.create_block();
        let produce = builder.create_block();
        builder.append_block_param(produce, types::I64);
        let guard_block = builder.create_block();
        let after_guard = builder.create_block();
        let dependent_block = builder.create_block();
        let failure = builder.create_block();
        builder.switch_to_block(entry);
        let first = builder.ins().iconst(types::I64, 0);
        builder.ins().jump(produce, &[first.into()]);
        builder.switch_to_block(produce);
        let phase = builder.block_params(produce)[0];
        let producer_value = builder.ins().iconst(types::I64, 0);
        let cranelift_codegen::ir::ValueDef::Result(producer, 0) =
            builder.func.dfg.value_def(producer_value)
        else {
            unreachable!()
        };
        if stale_backedge {
            let later = builder.ins().icmp_imm(IntCC::NotEqual, phase, 0);
            builder
                .ins()
                .brif(later, dependent_block, &[], guard_block, &[]);
        } else {
            builder.ins().jump(guard_block, &[]);
        }
        builder.switch_to_block(guard_block);
        let matches = builder.ins().icmp_imm(IntCC::Equal, producer_value, 0);
        let branch = builder.ins().brif(matches, after_guard, &[], failure, &[]);
        builder.switch_to_block(failure);
        let failed = builder.ins().iconst(types::I64, -1);
        finish_return(&mut builder, failed);
        builder.switch_to_block(after_guard);
        if stale_backedge {
            let later = builder.ins().iconst(types::I64, 1);
            builder.ins().jump(produce, &[later.into()]);
        } else {
            builder.ins().jump(dependent_block, &[]);
        }
        builder.switch_to_block(dependent_block);
        let value = builder.ins().iconst(types::I64, 0);
        let cranelift_codegen::ir::ValueDef::Result(dependent, 0) =
            builder.func.dfg.value_def(value)
        else {
            unreachable!()
        };
        finish_return(&mut builder, value);
        builder.seal_all_blocks();
        builder.finalize();
        (
            func,
            producer,
            ExactProtocolGuard {
                branch,
                success_ordinal: 0,
                success_target: after_guard,
            },
            dependent,
        )
    }

    #[derive(Clone, Copy)]
    enum PublicationMutation {
        Exact,
        Bypass,
        Clobber,
    }

    fn publication_fixture(
        mutation: PublicationMutation,
    ) -> (
        Function,
        cranelift_codegen::ir::Inst,
        cranelift_codegen::ir::Inst,
    ) {
        let mut func = Function::new();
        func.signature.call_conv = CallConv::SystemV;
        func.signature.returns.push(AbiParam::new(types::I64));
        let slot =
            func.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 8, 3));
        let mut context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut func, &mut context);
        let entry = builder.create_block();
        let publish = builder.create_block();
        let finish = builder.create_block();
        builder.switch_to_block(entry);
        let selector = builder.ins().iconst(types::I64, 0);
        if matches!(mutation, PublicationMutation::Bypass) {
            builder.ins().brif(selector, finish, &[], publish, &[]);
        } else {
            builder.ins().jump(publish, &[]);
        }
        builder.switch_to_block(publish);
        let word = builder.ins().iconst(types::I64, 41);
        let result_store = builder.ins().stack_store(word, slot, 0);
        if matches!(mutation, PublicationMutation::Clobber) {
            let stale = builder.ins().iconst(types::I64, 42);
            builder.ins().stack_store(stale, slot, 0);
        }
        builder.ins().jump(finish, &[]);
        builder.switch_to_block(finish);
        let zero = builder.ins().iconst(types::I64, 0);
        let returned = builder.ins().return_(&[zero]);
        builder.seal_all_blocks();
        builder.finalize();
        (func, result_store, returned)
    }

    fn call_refusal(mutation: CallProtocolMutation) -> String {
        let (func, obligation, target) = call_protocol_fixture(mutation);
        format!(
            "{:?}",
            verify_call_result_obligation(&IndexedFunction::new(&func), &obligation, target)
                .expect_err("the malformed call protocol must refuse")
        )
    }

    /// Promise class: durable invariant.
    /// MEASURED: finalized Brif CFGs with real status/Trap bypass edges refuse,
    /// while the exact call protocol certifies. CLAIMED: every dependent load
    /// traverses the current producer's successful guard. THE GAP: callee body
    /// correctness is a separate finished-certificate obligation.
    #[test]
    fn all_path_call_guards_reject_status_and_trap_bypasses() {
        let (func, obligation, target) = call_protocol_fixture(CallProtocolMutation::Exact);
        verify_call_result_obligation(&IndexedFunction::new(&func), &obligation, target)
            .expect("the exact call protocol is certified");
        assert!(
            call_refusal(CallProtocolMutation::TrapStatusBypass)
                .contains("Trap load is reachable without traversing")
        );
        assert!(
            call_refusal(CallProtocolMutation::ResultTrapBypass)
                .contains("Result load is reachable without traversing")
        );
    }

    /// Promise class: durable invariant.
    /// MEASURED: the exact one-pass CFG certifies, while an actual backedge that
    /// re-executes the producer and bypasses its later guard refuses.
    /// CLAIMED: a successful prior iteration cannot authorize a later producer
    /// execution. THE GAP: call/query fixtures separately bind memory and helper
    /// identities.
    #[test]
    fn a_backedge_cannot_reuse_a_prior_successful_guard() {
        let (exact, producer, guard, dependent) = backedge_guard_fixture(false);
        verify_all_paths_guarded(
            &IndexedFunction::new(&exact),
            producer,
            guard,
            dependent,
            "exact loop control",
        )
        .expect("the one-pass exact control is guarded");
        let (stale, producer, guard, dependent) = backedge_guard_fixture(true);
        let refusal = verify_all_paths_guarded(
            &IndexedFunction::new(&stale),
            producer,
            guard,
            dependent,
            "backedge-dependent observation",
        )
        .expect_err("the second producer execution bypasses its current guard");
        assert!(format!("{refusal:?}").contains("current successful guard"));
    }

    /// Promise class: durable invariant.
    /// MEASURED: exact publication dominates its return without another memory
    /// effect; a bypass and a same-slot clobber independently refuse.
    /// CLAIMED: final status-zero publication has the same all-path/no-clobber
    /// floor as calls and queries. THE GAP: opaque external alias effects are
    /// conservatively refused by the production publication scan.
    #[test]
    fn publication_requires_all_path_store_and_no_later_memory_effect() {
        let (exact, store, returned) = publication_fixture(PublicationMutation::Exact);
        verify_all_paths_after_instruction(
            &IndexedFunction::new(&exact),
            store,
            returned,
            "exact publication",
        )
        .expect("the exact store dominates its return");
        verify_no_memory_effect_after_instruction(
            &IndexedFunction::new(&exact),
            store,
            returned,
            "exact publication",
        )
        .expect("the exact store reaches return unclobbered");

        let (bypass, store, returned) = publication_fixture(PublicationMutation::Bypass);
        assert!(
            format!(
                "{:?}",
                verify_all_paths_after_instruction(
                    &IndexedFunction::new(&bypass),
                    store,
                    returned,
                    "bypass publication"
                )
                .expect_err("the return bypasses publication")
            )
            .contains("before its required finalized instruction")
        );

        let (clobber, store, returned) = publication_fixture(PublicationMutation::Clobber);
        assert!(
            format!(
                "{:?}",
                verify_no_memory_effect_after_instruction(
                    &IndexedFunction::new(&clobber),
                    store,
                    returned,
                    "clobbered publication",
                )
                .expect_err("the later store clobbers publication")
            )
            .contains("intervening memory effect")
        );
    }

    /// Promise class: durable invariant.
    /// MEASURED: foreign payload pointers, an initializer bypass, and a raw
    /// header overwrite each refuse at reaching-memory replay. CLAIMED: the
    /// actual call receives the exact initialized UnitBundle payload. THE GAP:
    /// the callee's Result contract remains independently certified.
    #[test]
    fn actual_header_and_reaching_initializer_are_independent_obligations() {
        assert!(
            call_refusal(CallProtocolMutation::ForeignHeaderPayload).contains("foreign payload")
        );
        assert!(call_refusal(CallProtocolMutation::InitializerBypass).contains("Trap initializer"));
        assert!(call_refusal(CallProtocolMutation::RawHeaderClobber).contains("foreign payload"));
    }

    /// Promise class: durable invariant.
    /// MEASURED: raw-address writes to Trap and Result invalidate the exact
    /// call generation before their loads. CLAIMED: no aliasing store can
    /// preserve a stale call-result fact. THE GAP: unsupported escaped-address
    /// provenance refuses conservatively rather than being modeled.
    #[test]
    fn raw_alias_clobber_invalidates_unit_memory_flow() {
        assert!(call_refusal(CallProtocolMutation::RawTrapClobber).contains("after a clobber"));
        assert!(call_refusal(CallProtocolMutation::RawResultClobber).contains("after a clobber"));
    }

    #[derive(Clone, Copy, Debug)]
    enum StoreForm {
        Full,
        Narrow8,
        Narrow16,
        Narrow32,
        Atomic,
    }

    fn store_form(
        builder: &mut FunctionBuilder<'_>,
        form: StoreForm,
        word: cranelift_codegen::ir::Value,
        address: cranelift_codegen::ir::Value,
        offset: i32,
    ) {
        let flags = cranelift_codegen::ir::MemFlags::trusted();
        match form {
            StoreForm::Full => {
                builder.ins().store(flags, word, address, offset);
            }
            StoreForm::Narrow8 => {
                builder.ins().istore8(flags, word, address, offset);
            }
            StoreForm::Narrow16 => {
                builder.ins().istore16(flags, word, address, offset);
            }
            StoreForm::Narrow32 => {
                builder.ins().istore32(flags, word, address, offset);
            }
            StoreForm::Atomic => {
                let address = builder.ins().iadd_imm(address, i64::from(offset));
                builder.ins().atomic_rmw(
                    types::I64,
                    flags,
                    cranelift_codegen::ir::AtomicRmwOp::Xchg,
                    address,
                    word,
                );
            }
        }
    }

    /// Promise class: durable invariant.
    /// MEASURED: on one fixture, a full store establishes the exact word;
    /// istore8, istore16, istore32 and atomic exchange of the same word to the
    /// same address leave the eight-byte location Unknown.
    /// CLAIMED: only a full-width store names the contents it leaves behind.
    /// THE GAP: other storing opcodes rely on the can_store fail-closed arm;
    /// this fixture pins all three narrow stores and one atomic exchange.
    #[test]
    fn only_a_full_store_establishes_exact_contents() {
        for form in [
            StoreForm::Full,
            StoreForm::Narrow8,
            StoreForm::Narrow16,
            StoreForm::Narrow32,
            StoreForm::Atomic,
        ] {
            let mut func = Function::new();
            func.signature.call_conv = CallConv::SystemV;
            func.signature.returns.push(AbiParam::new(types::I64));
            let slot =
                func.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 8, 3));
            let mut context = FunctionBuilderContext::new();
            let mut builder = FunctionBuilder::new(&mut func, &mut context);
            let entry = builder.create_block();
            builder.switch_to_block(entry);
            builder.seal_block(entry);
            let earlier = builder.ins().iconst(types::I64, 7);
            builder.ins().stack_store(earlier, slot, 0);
            let word = builder.ins().iconst(types::I64, 0x100);
            let address = builder.ins().stack_addr(types::I64, slot, 0);
            store_form(&mut builder, form, word, address, 0);
            let loaded = builder.ins().stack_load(types::I64, slot, 0);
            let returned = builder.ins().return_(&[loaded]);
            builder.finalize();
            let states = reaching_memory_states_before(
                &IndexedFunction::new(&func),
                StackMemoryLocation {
                    slot,
                    offset: 0,
                    bytes: 8,
                },
                returned,
            )
            .expect("reaching contents are decidable");
            let expected = match form {
                StoreForm::Full => ReachingMemoryState::Exact(word),
                StoreForm::Narrow8
                | StoreForm::Narrow16
                | StoreForm::Narrow32
                | StoreForm::Atomic => ReachingMemoryState::Unknown,
            };
            assert_eq!(states, BTreeSet::from([expected]), "{form:?}");
        }
    }

    /// Promise class: durable invariant.
    /// MEASURED: an atomic exchange that stores a slot's address to a
    /// non-stack destination escapes the slot; exchanging an ordinary word
    /// through the same instruction does not.
    /// CLAIMED: an atomic exchange cannot hide a stored stack address from
    /// the escape index. THE GAP: other storing opcodes rely on can_store;
    /// this fixture pins the atomic exchange route.
    #[test]
    fn an_atomic_store_of_a_slot_address_escapes_it() {
        for store_address in [true, false] {
            let mut func = Function::new();
            func.signature.call_conv = CallConv::SystemV;
            func.signature.returns.push(AbiParam::new(types::I64));
            let slot =
                func.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 8, 3));
            let mut context = FunctionBuilderContext::new();
            let mut builder = FunctionBuilder::new(&mut func, &mut context);
            let entry = builder.create_block();
            builder.switch_to_block(entry);
            builder.seal_block(entry);
            let word = if store_address {
                builder.ins().stack_addr(types::I64, slot, 0)
            } else {
                builder.ins().iconst(types::I64, 0)
            };
            let heap = builder.ins().iconst(types::I64, 0x1000);
            store_form(&mut builder, StoreForm::Atomic, word, heap, 0);
            let zero = builder.ins().iconst(types::I64, 0);
            builder.ins().return_(&[zero]);
            builder.finalize();
            let access = location_access(
                &IndexedFunction::new(&func),
                StackMemoryLocation {
                    slot,
                    offset: 0,
                    bytes: 8,
                },
            )
            .expect("slot escape is decidable");
            assert_eq!(access.escaped, store_address);
        }
    }

    fn certificate_refusal(result_form: StoreForm, trap_form: StoreForm) -> Option<String> {
        let mut func = Function::new();
        func.signature.call_conv = CallConv::SystemV;
        func.signature.params.push(AbiParam::new(types::I64));
        func.signature.returns.push(AbiParam::new(types::I64));
        let slot =
            func.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 16, 3));
        let mut context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut func, &mut context);
        let entry = builder.create_block();
        let publish = builder.create_block();
        let trap = builder.create_block();
        builder.append_block_params_for_function_params(entry);
        builder.switch_to_block(entry);
        let selector = builder.block_params(entry)[0];
        let frame = builder.ins().stack_addr(types::I64, slot, 0);
        builder.ins().brif(selector, publish, &[], trap, &[]);
        builder.switch_to_block(publish);
        let returned_word = builder.ins().iconst(types::I64, 41);
        store_form(&mut builder, result_form, returned_word, frame, 0);
        let zero = builder.ins().iconst(types::I64, 0);
        let return_inst = builder.ins().return_(&[zero]);
        builder.switch_to_block(trap);
        let trap_word = builder.ins().iconst(types::I64, 0x100);
        store_form(&mut builder, trap_form, trap_word, frame, 8);
        let trap_zero = builder.ins().iconst(types::I64, 0);
        builder.ins().return_(&[trap_zero]);
        builder.seal_all_blocks();
        builder.finalize();
        let result_store = func
            .layout
            .block_insts(publish)
            .find(|inst| func.dfg.insts[*inst].opcode().can_store())
            .expect("the publish block stores");
        verify_publication_and_terminals(
            &IndexedFunction::new(&func),
            FunctionResultPublication {
                frame,
                result_offset: 0,
                trap_offset: 8,
                result_store,
                returned_word,
                return_inst,
            },
            true,
        )
        .err()
        .map(|error| format!("{error:?}"))
    }

    /// Promise class: durable invariant.
    /// MEASURED: with full stores the Result and Trap certificates hold; each
    /// of istore8, istore16 and istore32 refuses a Result certificate and an
    /// explicit-Trap terminal when substituted for the full store.
    /// CLAIMED: Result and Trap words are certified only from full stores.
    /// THE GAP: undecoded storing opcodes rely on the opcode guard; the
    /// fixture independently pins all three narrow widths at both sites.
    #[test]
    fn result_and_trap_certificates_require_full_stores() {
        assert_eq!(certificate_refusal(StoreForm::Full, StoreForm::Full), None);
        for narrow in [StoreForm::Narrow8, StoreForm::Narrow16, StoreForm::Narrow32] {
            let result = certificate_refusal(narrow, StoreForm::Full)
                .expect("a narrow Result store is refused");
            assert!(
                result.contains("does not bind its exact word/frame/store"),
                "unexpected {narrow:?} Result refusal: {result}"
            );
            let trap = certificate_refusal(StoreForm::Full, narrow)
                .expect("a narrow Trap store is refused");
            assert!(
                trap.contains("uncertified status-zero terminal"),
                "unexpected {narrow:?} Trap refusal: {trap}"
            );
        }
    }

    /// Promise class: durable invariant.
    /// MEASURED: a retained stack address reaches later zero-argument calls
    /// both in the same block and a successor block, but not an earlier call;
    /// an ordinary word retains no address on either path.
    /// CLAIMED: call aliasing respects the retaining call's program order.
    /// THE GAP: non-stack-destination store retention is a separate obligation.
    #[test]
    fn retained_call_argument_reaches_only_later_calls() {
        fn fixture(
            expose_slot: bool,
        ) -> (
            Function,
            cranelift_codegen::ir::StackSlot,
            cranelift_codegen::ir::Inst,
            cranelift_codegen::ir::Inst,
            cranelift_codegen::ir::Inst,
        ) {
            let mut func = Function::new();
            func.signature.call_conv = CallConv::SystemV;
            func.signature.returns.push(AbiParam::new(types::I64));
            let exposure = direct_callee(&mut func, FuncId::from_u32(91), 1);
            let other = direct_callee(&mut func, FuncId::from_u32(92), 0);
            let slot =
                func.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 8, 3));
            let mut context = FunctionBuilderContext::new();
            let mut builder = FunctionBuilder::new(&mut func, &mut context);
            let entry = builder.create_block();
            let after = builder.create_block();
            builder.switch_to_block(entry);
            builder.seal_block(entry);
            let earlier_call = builder.ins().call(other, &[]);
            let argument = if expose_slot {
                builder.ins().stack_addr(types::I64, slot, 0)
            } else {
                builder.ins().iconst(types::I64, 0)
            };
            builder.ins().call(exposure, &[argument]);
            let same_block_later_call = builder.ins().call(other, &[]);
            builder.ins().jump(after, &[]);
            builder.switch_to_block(after);
            builder.seal_block(after);
            let later_call = builder.ins().call(other, &[]);
            let zero = builder.ins().iconst(types::I64, 0);
            finish_return(&mut builder, zero);
            builder.finalize();
            (func, slot, earlier_call, same_block_later_call, later_call)
        }
        for exposed in [true, false] {
            let (func, slot, earlier, same_block_later, later) = fixture(exposed);
            let indexed = IndexedFunction::new(&func);
            let access = location_access(
                &indexed,
                StackMemoryLocation {
                    slot,
                    offset: 0,
                    bytes: 8,
                },
            )
            .expect("slot retention is decidable from the finished call operands");
            assert_eq!(
                call_may_access_location(&indexed, same_block_later, &access)
                    .expect("same-block call aliasing is decidable"),
                exposed,
                "a later call in the retaining call's block can use its pointer"
            );
            assert_eq!(
                call_may_access_location(&indexed, later, &access)
                    .expect("successor-block call aliasing is decidable"),
                exposed,
                "a call need not pass the retained pointer again to write through it"
            );
            assert!(
                !call_may_access_location(&indexed, earlier, &access)
                    .expect("earlier call aliasing is decidable"),
                "a call that runs before the retaining call cannot use its pointer"
            );
        }
    }

    fn query_refusal(mutation: QueryMutation) -> String {
        let (func, helper, arena) = query_fixture(mutation);
        format!(
            "{:?}",
            verify_carrier_queries(&IndexedFunction::new(&func), helper, arena)
                .expect_err("the malformed helper protocol must refuse")
        )
    }

    /// Promise class: durable invariant.
    /// MEASURED: extra Brif and BrTable successors into the output load both
    /// refuse. CLAIMED: every supported CFG successor participates in helper
    /// guard domination. THE GAP: terminal instructions have no successors and
    /// are checked as exits rather than enumerated edges.
    #[test]
    fn helper_guard_rejects_each_supported_extra_successor() {
        assert!(query_refusal(QueryMutation::GuardBypass).contains("status guards"));
        assert!(query_refusal(QueryMutation::BranchTableBypass).contains("status guards"));
    }

    /// Promise class: durable invariant.
    /// MEASURED: a later helper call and a raw output-slot overwrite each
    /// refuse before observation. CLAIMED: one iteration's guard/memory fact
    /// cannot certify a later producer. THE GAP: helper semantic purity beyond
    /// the audited output protocol remains outside this verifier.
    #[test]
    fn helper_memory_rejects_stale_producer_and_raw_output_clobber() {
        let stale = query_refusal(QueryMutation::LaterProducerBypass);
        assert!(
            stale.contains("current producer or after a clobber"),
            "unexpected stale-producer refusal: {stale}"
        );
        let clobber = query_refusal(QueryMutation::RawOutputClobber);
        assert!(
            clobber.contains("after a clobber"),
            "unexpected output-clobber refusal: {clobber}"
        );
    }

    /// Promise class: durable invariant.
    /// MEASURED: exact and success-edge-subdivided helper CFGs both certify one
    /// query. CLAIMED: certification is CFG-semantic rather than layout-
    /// positional. THE GAP: this positive controls subdivision, not arbitrary
    /// semantics-changing rewrites.
    #[test]
    fn layout_equivalent_helper_subdivision_remains_certified() {
        for mutation in [QueryMutation::Exact, QueryMutation::Subdivided] {
            let (func, helper, arena) = query_fixture(mutation);
            assert_eq!(
                verify_carrier_queries(&IndexedFunction::new(&func), helper, arena)
                    .expect("layout-equivalent helper protocol must certify")
                    .len(),
                1
            );
        }
    }
}

fn stage_result_body(
    compiler: &mut Lowering<'_>,
    unit: ExistingResultUnitIdentity,
    target: FuncId,
    func: Function,
    publication: Option<FunctionResultPublication>,
    required_contract: Option<ConstructorIdentity>,
    independent_contract: Option<ConstructorIdentity>,
) -> StagedResultBody {
    StagedResultBody {
        unit,
        target,
        func,
        authorities: std::mem::take(&mut compiler.function_local.generated_constructor_authorities),
        call_obligations: std::mem::take(
            &mut compiler.function_local.pending_call_result_obligations,
        ),
        boundary_arena: compiler.function_local.boundary_arena,
        publication,
        required_contract,
        independent_contract,
    }
}
