//! The effects emitter -- effect-seat emission, host-call emission, and
//! the effect-side operand construction.
//!
//! `RT-EMITTER-EFFECTS-SPLIT` `D1`. Extends the `boundary.rs`/`source.rs`/
//! `calls.rs`/`joins.rs`/`aggregates.rs` seam (items 11-15): the owner
//! traced in this item's D0 ledger (`docs/program/issues/
//! RT-EMITTER-EFFECTS-SPLIT.md`) relocates here from `mod.rs` and
//! `core.rs`, moved verbatim -- the seat-group claim/close methods and the
//! `EffectSeatLedger` type family from `mod.rs`, plus `core.rs`'s
//! `RuntimeExpr::Effect` dispatch entry (`lower_process_host_effect`, the
//! campaign's largest single mover at 1,340 lines), matching item 14's
//! own core.rs-dispatch-entry-moves-with-its-domain precedent.
//!
//! Every other type the moving methods merely manipulate stays declared at
//! the `mod.rs` hub -- hub-stays/methods-move, the same shape items
//! 10/12/13/14/15 established. Two judgment calls, both resolved by the
//! Architect's D0 vote (`evt_7nzxad9y75crk`): `ClaimedEffectSeats<'a>`
//! stays RETAIN (a parameter type genuinely shared across this module,
//! `aggregates.rs`, and item 15's `constructors.rs` tests -- zero-widening
//! at the `mod.rs` hub); `SiteOperandWitness`/`site_operand_witness` also
//! stay RETAIN (zero Effects consumers -- the sole embedding/caller is
//! `aggregates.rs`, an aggregates-adjacent symbol despite its name).
//!
//! `EffectSeatLedger`/`EffectSeatClosure` were already `pub(in
//! crate::cranelift_backend)` before the move -- they move verbatim, zero
//! widening. `mod.rs`'s own `host_effect_seats: Option<EffectSeatLedger>`
//! field on the retained `Lowering` hub struct is updated to the qualified
//! `Option<effects::EffectSeatLedger>`, the same pattern `aggregates.rs`'s
//! own `aggregate_allocations` field already uses.
//!
//! `pub(super)` widening: `lower_process_host_effect` and
//! `lower_buffer_freeze_resource_seat`, whose sole external caller
//! (`lower_expr`'s `RuntimeExpr::Effect` arm) stays in the retained
//! `core.rs`.
//!
//! Carried non-move transport hunk (named, not silent): `units.rs`
//! (out of scope for this item) holds `#[cfg(test)]` accessor fns
//! (`capacity_phase_dispatch`/`reset_capacity_phase_dispatch`) that read
//! the moving `CAPACITY_PHASE_DISPATCH` thread_local via
//! `super::CAPACITY_PHASE_DISPATCH` -- updated to `super::effects::
//! CAPACITY_PHASE_DISPATCH` as part of this transport.

use super::*;

// `masked_reply_response_bytes` stays in the retained `core.rs` (a sibling
// consumer, `core/tests/constructors.rs`'s `super::masked_reply_response_
// bytes`, keeps it there); widened to `pub(super)` for this module's own
// `lower_process_host_effect` to reach it.
use super::core::masked_reply_response_bytes;
// `EffectSeatGroupId` is declared in this module's own nested
// `effect_seat_group` submodule (moved verbatim below); resolved by its
// own relative path, not inherited from `use super::*`.
use effect_seat_group::EffectSeatGroupId;

/// **`D7` — perturbations of one VISIT, as distinct from perturbations of the
/// planned population.**
///
/// ⛔ These act on the emitter's side of the authority. The population
/// mutations ([`EffectSeatPlanMutation`]) act on the planner's. Keeping them
/// separate is what lets a control say which side a gate is actually reading —
/// a single enum spanning both would let a green row be attributed to either.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum EffectSeatVisitMutation {
    Exact,
    /// Omit one slot per visit, alternating which one across successive visits.
    /// ⭐ The masking discriminator: the omissions are COMPLEMENTARY, so a
    /// ledger that accumulated claims per occurrence rather than per visit would
    /// see a complete union and accept.
    OmitComplementary,
    /// Claim the visit's first slot a second time.
    DuplicateWithinVisit,
    /// Drop the open group instead of closing it.
    DiscardGroup,
    /// Report the opposite phase of the one the operand is actually in.
    PerturbObservedPhase,
    /// Drop one COMMITTED group after every body close has passed and before
    /// the whole-pass close. ⭐ The only way to ask whether the whole-pass
    /// backstop is still doing work now that the body close catches the same
    /// condition earlier — every ordinary route to a discarded group is now
    /// stopped before it can reach the backstop.
    DropCommittedGroupBeforeGlobalClose,
}
#[cfg(test)]
thread_local! {
    static EFFECT_SEAT_VISIT_MUTATION: std::cell::Cell<EffectSeatVisitMutation> =
        const { std::cell::Cell::new(EffectSeatVisitMutation::Exact) };
    /// Which visit this is, so `OmitComplementary` can alternate.
    static EFFECT_SEAT_VISIT_INDEX: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}
#[cfg(test)]
pub(in crate::cranelift_backend) fn set_effect_seat_visit_mutation(
    mutation: EffectSeatVisitMutation,
) {
    EFFECT_SEAT_VISIT_MUTATION.with(|cell| cell.set(mutation));
    EFFECT_SEAT_VISIT_INDEX.with(|cell| cell.set(0));
}
#[cfg(test)]
pub(super) fn effect_seat_visit_mutation() -> EffectSeatVisitMutation {
    EFFECT_SEAT_VISIT_MUTATION.with(std::cell::Cell::get)
}
#[cfg(test)]
fn effect_seat_next_visit_index() -> usize {
    EFFECT_SEAT_VISIT_INDEX.with(|cell| {
        let index = cell.get();
        cell.set(index + 1);
        index
    })
}
/// **`RT-DECL-CLOSURE-PORT` `D7` — the two framed lowering-closure mutations.**
///
/// ⛔ Both name a REMOVAL of something this release added, not a corruption of
/// an input. That is what makes them closure evidence: each restores the state
/// the frame says must refuse, and the control asserts the refusal is the exact
/// one the frame names rather than any refusal.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum EffectSeatDispatchMutation {
    Exact,
    /// Delete the carried arm of the capacity seat, leaving the specialized
    /// read the whole route -- the state that produced the exact
    /// `264 -> 262 / position 1` refusal.
    RemoveCarriedCapacityArm,
    /// Restore the eager all-argument projection in reply synthesis, so every
    /// argument is demanded as a template whether or not a synthesized node
    /// declares a use for it.
    RestoreBulkConversion,
    /// **`RT-CARRIER-BYTESPAN-OBSERVE` `AC-2`.** Put every `BytesPointerLength`
    /// seat back to `SPECIALIZED_ONLY`, which is exactly the state `D5`
    /// activated out of.
    ///
    /// It withdraws the AVAILABILITY rather than deleting the observer call, so
    /// the refusal is raised by the real `Need ⊆ Avail` gate and carries the
    /// real message. A mutation that stubbed the observer instead would
    /// manufacture a message that merely resembled the original.
    RemoveCarriedByteSpanAvailability,
    /// Force the byte-span observer's outcome to `1` — a well-formed span that
    /// failed a bounds rule — at the point the lowering reads it.
    ///
    /// It injects AFTER the observer boundary on purpose. It is not a claim
    /// that any rig witnesses `D3` producing this status; it isolates the
    /// propagation layer between the observer and the program, which is the
    /// only layer these controls are about.
    ForceByteSpanOutcomeBounds,
    /// The same injection for outcome `2` — a word that never denoted a
    /// viewable byte span.
    ForceByteSpanOutcomeNotASpan,
    /// Replace a projected site-bound operand after its source witness has
    /// been captured. The reconciliation must reject the substitution rather
    /// than lending the original operand's owner proof to the replacement.
    SubstituteSiteOperandValue,
    /// Give the production `ReadEof`/`ReadSome` constructor producer a
    /// discriminator outside its closed 0/1 alternatives. This mutates the
    /// producer, before transfer, rather than injecting a post-validation
    /// `DynamicConstructorV1` in a test.
    ForceReadProgressOutsideAlternatives,
    /// Withdraw only M3's carried-constructor route. `Avail` remains
    /// specialized-only, so the original exact seat refusal must return.
    RemoveCarriedConstructorDispatch,
}
#[cfg(test)]
thread_local! {
    static EFFECT_SEAT_DISPATCH_MUTATION: std::cell::Cell<EffectSeatDispatchMutation> =
        const { std::cell::Cell::new(EffectSeatDispatchMutation::Exact) };
    pub(super) static SITE_OPERAND_SUBSTITUTION_HITS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}
#[cfg(test)]
pub(in crate::cranelift_backend) fn set_effect_seat_dispatch_mutation(
    mutation: EffectSeatDispatchMutation,
) {
    EFFECT_SEAT_DISPATCH_MUTATION.with(|cell| cell.set(mutation));
    SITE_OPERAND_SUBSTITUTION_HITS.with(|cell| cell.set(0));
}
#[cfg(test)]
pub(super) fn effect_seat_dispatch_mutation() -> EffectSeatDispatchMutation {
    EFFECT_SEAT_DISPATCH_MUTATION.with(std::cell::Cell::get)
}
#[cfg(test)]
pub(super) fn site_operand_substitution_hits() -> usize {
    SITE_OPERAND_SUBSTITUTION_HITS.with(std::cell::Cell::get)
}

/// **`RT-DEAD-ARM-EFFECT-LOWERING` `D1` conjunct (2) -- every constructor the
/// RUNTIME can produce, as opposed to the program's own syntax.**
///
/// This is the set whose absence made the first cut of the deadness predicate
/// unsound in the LIVE direction (Architect `evt_4hcny7ae7h9sb`): an effect
/// RESPONSE such as `Result::Ok` is synthesized by the host, appears in no
/// `Construct` and in no literal, and so read as never-constructed. Unioned in
/// as LIVE, it keeps the program-constructed `FSOp` REQUEST arms provably dead
/// while the host-produced response arms stay strict.
///
/// **EXHAUSTIVE BY CONSTRUCTION, and that is the soundness gate, not a style
/// choice** (`COORDINATION` section 7). The struct is destructured with NO
/// `..` rest pattern, so a symbol added to the runtime's vocabulary is a
/// COMPILE ERROR here until someone classifies it. The alternative -- a field
/// list that silently omits a new origin -- would trap a live path, which is a
/// working program broken, and is exactly the regression conjunct (2) exists to
/// prevent. Do not add a rest pattern to make this compile.
fn runtime_producible_constructors(
    symbols: &crate::NativeProcessSymbols,
) -> BTreeSet<crate::RuntimeSymbol> {
    let crate::NativeProcessSymbols {
        process_input,
        list_nil,
        list_cons,
        prod,
        exit_success,
        exit_failure,
        result_err,
        result_ok,
        option_some,
        file_error,
        file_operation_read,
        file_operation_write,
        file_operation_change_mode,
        io_errors,
        resource_host_io,
        resource_closed,
        resource_malformed,
        resource_right_not_held,
        resource_release_failed,
        resource_kind_mismatch,
        resource_buffer_limit,
        resource_allocation_failed,
        resource_invalid_offset,
        resource_invalid_bounds,
        resource_no_progress,
        resource_kind_fs_handle,
        resource_kind_buffer,
        resource_trace_identity,
        nat_zero,
        nat_suc,
        private_buffer_span,
        private_transfer_count,
        read_some,
        read_eof,
        wrote,
        unit,
        bool_false,
        bool_true,
    } = symbols;
    // Every field is a constructor the native runtime can put in front of a
    // match: host-effect responses, process-entry inputs, and the primitive
    // result vocabulary alike. None is classified as program-only.
    let mut producible = [
        process_input,
        list_nil,
        list_cons,
        prod,
        exit_success,
        exit_failure,
        result_err,
        result_ok,
        option_some,
        file_error,
        file_operation_read,
        file_operation_write,
        file_operation_change_mode,
        resource_host_io,
        resource_closed,
        resource_malformed,
        resource_right_not_held,
        resource_release_failed,
        resource_kind_mismatch,
        resource_buffer_limit,
        resource_allocation_failed,
        resource_invalid_offset,
        resource_invalid_bounds,
        resource_no_progress,
        resource_kind_fs_handle,
        resource_kind_buffer,
        resource_trace_identity,
        nat_zero,
        nat_suc,
        private_buffer_span,
        private_transfer_count,
        read_some,
        read_eof,
        wrote,
        unit,
        bool_false,
        bool_true,
    ]
    .into_iter()
    .cloned()
    .collect::<BTreeSet<_>>();
    // The `IOError` alternatives are a run rather than a single field, and the
    // whole run is host-produced.
    producible.extend(io_errors.iter().cloned());
    producible
}

/// **`RT-DEAD-ARM-EFFECT-LOWERING` `D1` -- what claiming one seat produced.**
///
/// Two outcomes rather than a `Result` with a stringly-matched error, because
/// the caller must be able to tell "this seat refused" from "this seat refused
/// AND its arm is provably unreachable" WITHOUT re-deriving the second fact or
/// matching on a message. The decision belongs at the seat, which is the only
/// place that holds both the membership verdict and the effect's origin.
#[derive(Clone, Copy, Debug)]
enum SeatClaimOutcome {
    Claimed(PlannedEffectSeat),
    /// Today's refusal, on an arm PROVEN never-constructed program-wide. The
    /// caller lowers the whole effect to a trap instead of a wire request.
    UnreachableArm,
}

impl<'a> Lowering<'a> {
    /// **`D7` — open the claim group for one visit to one effect occurrence.**
    ///
    /// ⛔ Called BEFORE any seat of the occurrence is observed, and after every
    /// operand has been lowered — an operand's own lowering may itself visit a
    /// nested effect, and a group open across that would take the nested
    /// visit's claims.
    ///
    /// ⚠ `None` outside the emission pass. A bare rig defines no function, so
    /// there is no body for a visit to belong to; the per-seat `Avail`
    /// membership below still runs there, because that is a property of the
    /// seat rather than of the ledger.
    fn open_host_effect_seat_group(
        &mut self,
        effect_origin: StaticOriginId,
        operation: ken_host::HostOpV1,
    ) -> Result<Option<EffectSeatGroupId>, CraneliftBackendError> {
        let planned = self
            .static_transition_plan
            .host_effect_seat_slots(effect_origin);
        let function = self.defining_function_id;
        let Some(ledger) = self.host_effect_seats.as_mut() else {
            return Ok(None);
        };
        let function = function.ok_or_else(|| {
            backend_module(
                "a host effect occurrence was visited inside the emission pass with no declared \
                 function open, so its claim group has no body to be scoped by"
                    .to_string(),
            )
        })?;
        ledger
            .open_group(function, effect_origin, operation, planned)
            .map(Some)
    }
    /// **`D7` — claim the ONE planned record for a seat, in the phase the
    /// operand is ACTUALLY in.**
    ///
    /// ⭐ **This is where `Need ⊆ Avail` is asked.** The need and the
    /// availability are the planner's, derived from the operation and the slot
    /// with no reference to any representation; the phase is read off the
    /// operand in hand, and cannot be reverse-derived from a child occurrence or
    /// an ABI result. A seat that fails the membership is refused as that exact
    /// seat of that exact operation — not as a generic specialized-only surface,
    /// which is the whole point of the record.
    ///
    /// ⚠ The returned record is bound to `operand` by construction: the phase it
    /// was proved against was read from that operand and no other. Binding it to
    /// the operation-specific ARM that performs the read needs the arms to take
    /// the claim in place of the bulk conversion, which is the next release —
    /// today the claim is made and the arms still read the bulk vector.
    /// **`RT-DEAD-ARM-EFFECT-LOWERING` `D1` -- the ONE deadness question, asked
    /// identically at every refusal site.**
    ///
    /// There are two sites that fail object emission on an arm no execution
    /// reaches -- the seat's `Need`-subset-`Avail` membership test, and the
    /// represented-unavailable-lane check at the top of
    /// `lower_process_host_effect`. Both consult THIS, so the predicate cannot
    /// drift between them and there is exactly one place it can be wrong
    /// (Architect `evt_4hcny7ae7h9sb`: compute once, consult at each site).
    fn effect_arm_is_provably_dead(
        &self,
        effect_origin: StaticOriginId,
    ) -> Result<bool, CraneliftBackendError> {
        self.static_transition_plan.origin_is_in_provably_dead_arm(
            effect_origin,
            &runtime_producible_constructors(&self.process_symbols),
        )
    }

    /// **`RT-DEAD-ARM-JOIN-DISPOSITION` -- when the trap fires, the arm's
    /// planned source joins are DISPOSITIONED as statically unselected.**
    ///
    /// Trapping a dead arm skips lowering its body, so that body's planned
    /// joins are neither emitted nor statically unselected and the
    /// join-consumption invariant refuses the whole function. Latent since the
    /// trap landed: every compile refused in the effect-seat layer before
    /// reaching that invariant. Measured on the governed witness -- 19 of 19
    /// unconsumed joins were inside provably dead arms, no exceptions.
    ///
    /// **DISPOSITION FOLLOWS DEADNESS, never the reverse.** The arm body comes
    /// from the deadness predicate's OWN witness, so the only joins this can
    /// disposition are those of an arm that predicate already proved dead. If a
    /// join cannot be dispositioned under the existing predicate that is a
    /// finding to report -- never a reason to widen the predicate until it can,
    /// which is the over-accept this shape invites.
    ///
    /// Reuses `RT-LEXICAL-RECURSOR-CONSUMERS` `D2b`'s abandoned-region
    /// mechanism unchanged: a trapped dead arm is the same category of region as
    /// an abandoned `Let` body, and that mechanism's own doc records it removing
    /// this identical refusal.
    ///
    /// The `"neither emitted nor statically unselected"` refusal stays as the
    /// fail-closed backstop, and `validate_materialized_dead_join_cfg` still
    /// runs afterwards -- it additionally requires the emitted and dispositioned
    /// sets to be DISJOINT, so dispositioning a join that was in fact emitted
    /// refuses rather than passing.
    fn disposition_dead_arm_joins(
        &mut self,
        effect_origin: StaticOriginId,
    ) -> Result<(), CraneliftBackendError> {
        let Some(arm_body) = self
            .static_transition_plan
            .provably_dead_arm_body_containing(
                effect_origin,
                &runtime_producible_constructors(&self.process_symbols),
            )?
        else {
            return Ok(());
        };
        let joins = self
            .static_transition_plan
            .source_join_origins_in_owner_subtree(arm_body)?;
        for origin in joins {
            self.function_local.dispositioned_join_origins.insert(origin);
        }
        Ok(())
    }

    fn claim_host_effect_seat(
        &mut self,
        group: Option<EffectSeatGroupId>,
        effect_origin: StaticOriginId,
        slot: EffectSeatSlot,
        operand: &LoweringOperand,
    ) -> Result<SeatClaimOutcome, CraneliftBackendError> {
        let record = self
            .static_transition_plan
            .host_effect_seat(effect_origin, slot)?;
        let observed = operand.effect_seat_phase();
        #[cfg(test)]
        let observed = match effect_seat_visit_mutation() {
            EffectSeatVisitMutation::PerturbObservedPhase => match observed {
                EffectSeatPhase::SpecializedTemplate => EffectSeatPhase::CarriedWord,
                EffectSeatPhase::CarriedWord => EffectSeatPhase::SpecializedTemplate,
            },
            _ => observed,
        };
        // A planned `SiteOperand(index)` is a second, exact consumer of the
        // same source operand. Its carried route is the emitted-helper
        // projection in `site_operand_argument`, not a widening of this seat's
        // direct `Avail` partition. Derive that exception from the planner's
        // existing recipe relation so an operation with no declared
        // site-bound child retains the ordinary `Need ⊆ Avail` refusal.
        let carried_site_operand = observed == EffectSeatPhase::CarriedWord
            && record.need == EffectSeatNeed::BytesPointerLength
            && self
                .static_transition_plan
                .host_effect_site_operand_slots(effect_origin)?
                .contains(&slot);
        // **`RT-RESOURCE-RELEASE-CARRIED-OBSERVE` `D1` -- keyed on the
        // (need, phase) PAIR, closing over the whole `ResourceScalar` family.**
        //
        // Architect `evt_3dnd21pjg193g`, on a measured guard-uniformity result.
        // The key began as the (operation, slot, need) triple naming only
        // `ResourceRelease` `Argument(0)`; measuring past it showed the SAME
        // observation is wanted at `FsHandleMetadata` `Argument(0)` and
        // `FsReadAt` `Argument(0)` too. Enumerating one operation at a time is
        // an unbounded chain of near-identical rulings for one predicate, so
        // where entries share a predicate the closure is over the predicate.
        //
        // **Sound ONLY because the guards are UNIFORM across those seats, and
        // that is structural rather than sampled.** A `Lowered`'s boundary
        // representation is chosen by ONE `match` on its `LoweredVariant`
        // (`boundary.rs`, the `CapabilityToken | ResourceToken` arm), with no
        // consuming operation in scope -- so a resource token carries
        // `InvocationBorrowed` / `BorrowedOpaque` no matter which seat later
        // reads it. Had the tag/class varied by consumer, widening with fixed
        // guards would send a VALID handle down the runtime failure path:
        // a loud compile refusal traded for a silent runtime failure on a
        // well-typed program, which is worse than the refusal it replaces.
        //
        // The guards prove "a borrowed-opaque invocation handle", NOT
        // "specifically a resource token" -- `CapabilityToken` and the borrowed
        // native/option variants share that tag/class pair. That is the
        // PRECEDENT's existing property, inherited unchanged rather than
        // introduced here: what keeps a capability from arriving at a
        // `ResourceScalar` seat is the seat's own contract and Ken's typing, not
        // these guards. Stated so a reader does not over-read them.
        //
        // Conditioned on the CARRIED phase, so the route is the exact
        // complement of where `Direct` serves. In the specialized phase
        // `avail.admits` is true and `Direct` takes it, unchanged -- a new route
        // beside an old one must fire only where the old one cannot serve, or
        // it masks the strict gate on inputs the strict gate was handling
        // correctly.
        //
        // NOT `host_effect_site_operand_slots`, measured the wrong key for this
        // seat shape on the sibling. NOT widened across NEEDS: `ExactIntU64`
        // is a different observation with its own carried precedent
        // (`carried_exact_int`), and one key spanning two observations would
        // conflate them.
        let carried_resource_token = observed == EffectSeatPhase::CarriedWord
            && record.need == EffectSeatNeed::ResourceScalar;
        // `RT-CARRIED-IH-DISPATCH-SITEOP` M3 -- admission is permission to
        // attempt one guarded observation, not a widening of `Avail`. The
        // planner must issue at least one artifact-static constructor path for
        // this exact operation/slot; an empty or non-constructor table retains
        // the ordinary refusal below.
        let carried_constructor_dispatch = observed == EffectSeatPhase::CarriedWord
            && record.need == EffectSeatNeed::ConstructorTag
            && self
                .static_transition_plan
                .host_effect_constructor_dispatch(record.operation, record.slot)?
                .is_some_and(|paths| !paths.is_empty());
        #[cfg(test)]
        let carried_constructor_dispatch = carried_constructor_dispatch
            && effect_seat_dispatch_mutation()
                != EffectSeatDispatchMutation::RemoveCarriedConstructorDispatch;
        let route = if record.avail.admits(observed) {
            EffectSeatClaimRoute::Direct
        } else if carried_site_operand {
            EffectSeatClaimRoute::SiteOperandProjection
        } else if carried_resource_token {
            EffectSeatClaimRoute::CarriedResourceObservation
        } else if carried_constructor_dispatch {
            EffectSeatClaimRoute::CarriedConstructorDispatch
        } else {
            EffectSeatClaimRoute::Direct
        };
        let admits = route != EffectSeatClaimRoute::Direct || record.avail.admits(observed);
        // `AC-2` — withdraw exactly what `D5` granted, at the membership test
        // itself, so the refusal below is the PRODUCTION refusal rather than a
        // manufactured lookalike. Only the byte-span need and only the carried
        // phase are affected; every other seat answers as it always did.
        #[cfg(test)]
        let admits = admits
            && !(effect_seat_dispatch_mutation()
                == EffectSeatDispatchMutation::RemoveCarriedByteSpanAvailability
                && record.need == EffectSeatNeed::BytesPointerLength
                && observed == EffectSeatPhase::CarriedWord);
        if !admits {
            // **`RT-DEAD-ARM-EFFECT-LOWERING` `D1` -- the narrowest trigger.**
            //
            // Reached ONLY where today's lowering already refuses, so every
            // currently-compiling program lowers exactly as it did. The seat's
            // `Need`-subset-`Avail` partition above is untouched: this does not
            // widen what a seat may observe, it asks whether this seat can be
            // reached at all.
            //
            // **This is a substitute-on-refusal shape, and that is normally the
            // fail-OPEN smell this backend refuses. It is sound here for one
            // specific reason and not as a general pattern: the substitute is a
            // TRAP.** An arm wrongly reported dead HALTS -- it never yields a
            // wrong result, never skips a capability gate, never returns a
            // value the seat could not observe. So the census's completeness
            // buys LIVENESS, and correctness does not depend on it. Substituting
            // anything that could succeed here would be unsound, and the
            // distinction is the whole justification.
            //
            // The predicate is conservative in the safe direction: an arm not
            // PROVEN never-constructed answers `false` and takes the refusal
            // below, unchanged.
            if self.effect_arm_is_provably_dead(effect_origin)? {
                if let (Some(group), Some(ledger)) = (group, self.host_effect_seats.as_mut()) {
                    ledger.record_unreachable_seat(group, slot)?;
                }
                self.disposition_dead_arm_joins(effect_origin)?;
                return Ok(SeatClaimOutcome::UnreachableArm);
            }
            return Err(unsupported(
                "Effect",
                format!(
                    "seat {slot:?} of {:?} needs {:?}, which it cannot observe in {observed:?}",
                    record.operation, record.need
                ),
            ));
        }
        let Some(group) = group else {
            return Ok(SeatClaimOutcome::Claimed(record));
        };
        let Some(ledger) = self.host_effect_seats.as_mut() else {
            return Ok(SeatClaimOutcome::Claimed(record));
        };
        ledger.claim(group, record, observed, route)?;
        Ok(SeatClaimOutcome::Claimed(record))
    }
    /// **`D7` — close the visit, before host dispatch or any successful exit.**
    fn close_host_effect_seat_group(
        &mut self,
        group: Option<EffectSeatGroupId>,
    ) -> Result<(), CraneliftBackendError> {
        let Some(group) = group else {
            return Ok(());
        };
        let Some(ledger) = self.host_effect_seats.as_mut() else {
            return Ok(());
        };
        #[cfg(test)]
        if effect_seat_visit_mutation() == EffectSeatVisitMutation::DiscardGroup {
            ledger.discard_open_group_for_tests();
            return Ok(());
        }
        ledger.close_group(group)
    }
}

/// **`RT-DECL-CLOSURE-PORT` `D7` — the claim-group identity.**
///
/// ⛔ **Its own module with a private field, so lowering cannot CONSTRUCT one.**
/// But construction is not where the guarantee lives — a fresh id minted out of
/// band names no open group, and every operation below requires an id the
/// ledger itself opened and is still holding. ⇒ The closure is REGISTRATION,
/// not opacity; opacity only removes the shortcut.
mod effect_seat_group {
    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub(in crate::cranelift_backend) struct EffectSeatGroupId(u64);

    /// Mint the next identity. ⛔ Callable only with the ledger's own counter.
    pub(super) fn mint(counter: &mut u64) -> EffectSeatGroupId {
        *counter += 1;
        EffectSeatGroupId(*counter)
    }
}
/// One seat, as the emitter actually found it.
///
/// ⭐ **The observed phase is RETAINED, not checked and discarded.** Checking it
/// against `Avail` and then dropping it leaves the ledger unable to say what the
/// emitter saw — so a later reader has only the planner's admissible set, and
/// the one fact that distinguishes a specialized read from a carried one is
/// gone at exactly the point a reviewer would ask for it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ClaimedEffectSeat {
    record: PlannedEffectSeat,
    observed: EffectSeatPhase,
    route: EffectSeatClaimRoute,
}
/// Why one observed phase is admissible at a claimed seat.
///
/// `SiteOperandProjection` is deliberately separate from `Avail`: the direct
/// operation consumer remains specialized-only, while the compiler-authored
/// result tree names an exact second use whose carried word is projected by an
/// emitted helper.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EffectSeatClaimRoute {
    Direct,
    SiteOperandProjection,
    /// **`RT-RESOURCE-RELEASE-CARRIED-OBSERVE` `D1` -- the carried
    /// resource-token observation.**
    ///
    /// Admitted through the same door the byte-span route uses -- `route !=
    /// Direct` bypasses `avail` -- so the seat's `Need`-subset-`Avail`
    /// partition is BYTE-UNTOUCHED: `(ResourceRelease, 0)` stays
    /// `Avail::SPECIALIZED_ONLY` in the `resource` row and is NOT moved to
    /// `phase_bearing_resource`. A new route PROVES observability; it does not
    /// relax membership (Architect `evt_48m6xvb59wnyg`).
    ///
    /// Bypassing `avail` is sound ONLY because the accept path re-runs a
    /// fail-closed consumer: the emitter arm reads this seat through
    /// `lower_resource_token_seat`, whose carried arm requires the boundary tag
    /// AND the boundary class before it reads the scalar. Admission here is a
    /// permission; the guarded observation is the authority.
    CarriedResourceObservation,
    /// A carried constructor value at an exact `ConstructorTag` seat with a
    /// non-empty planner-issued finite dispatch table.
    ///
    /// Like the resource route, this is permission only. The accept/refuse
    /// authority is the consumer: it checks the carrier tag, exact arity and
    /// every positional child tag before producing a host-wire value. Every
    /// mismatch returns the deterministic failure value before host dispatch.
    CarriedConstructorDispatch,
}
/// One compiler-side lowering VISIT to one effect occurrence.
///
/// ⭐ **The group is the unit of completeness, and that is the whole point.** A
/// ledger that accumulated claims per (body, occurrence) across visits would
/// accept two visits that each read half the seats, because their union is
/// complete — and two half-reads are exactly the defect. Completeness is asked
/// of each visit alone.
#[derive(Clone, Debug)]
struct OpenEffectSeatGroup {
    id: EffectSeatGroupId,
    function: FuncId,
    effect_origin: StaticOriginId,
    operation: ken_host::HostOpV1,
    /// The occurrence's planned slot population, bound at open so a later
    /// change to the plan cannot move the target the group closes against.
    planned: BTreeSet<EffectSeatSlot>,
    claims: BTreeMap<EffectSeatSlot, ClaimedEffectSeat>,
    /// **`RT-DEAD-ARM-EFFECT-LOWERING` `D1` -- planned seats this visit REACHED
    /// but could not observe, on an arm proven never-constructed.**
    ///
    /// Held apart from `claims` deliberately, and this is the load-bearing
    /// half. The ledger is an ATTESTATION of what the emitter actually
    /// consumed, so recording an unobservable seat as a claim would enter a
    /// falsehood into the record that every later reader trusts. The visit is
    /// still COMPLETE -- every planned seat was reached -- and `close_group`
    /// asks for exactly that, over the union.
    unreachable: BTreeSet<EffectSeatSlot>,
}
/// A visit that closed complete.
#[derive(Clone, Debug)]
struct CommittedEffectSeatGroup {
    function: FuncId,
    effect_origin: StaticOriginId,
    claims: BTreeMap<EffectSeatSlot, ClaimedEffectSeat>,
    /// See [`OpenEffectSeatGroup::unreachable`]. Carried through the commit so
    /// the whole-pass closeout can report reached-but-unobservable seats
    /// separately from claims rather than silently counting them as neither.
    unreachable: BTreeSet<EffectSeatSlot>,
}
/// **`RT-DECL-CLOSURE-PORT` `D7` — what the emitter ACTUALLY consumed, per
/// visit.**
///
/// ⭐ This is the independent second side of the seat authority. The planner
/// derives a population of seats and the admissible phases of each; this records
/// which seats a concrete visit reached and the phase it actually found them in.
/// A single structure holding both would make the agreement true by construction.
#[derive(Clone, Debug, Default)]
pub(in crate::cranelift_backend) struct EffectSeatLedger {
    next_group: u64,
    /// ⛔ At most ONE group open at a time. An effect's operands are lowered
    /// before its group opens, so a second open means a visit began inside
    /// another visit's window and their claims could interleave.
    open: Option<OpenEffectSeatGroup>,
    /// ⛔ Every opened group, keyed to the EXACT body it was opened for. A bare
    /// set of ids cannot answer "was every group this body opened committed
    /// before this body was defined" -- it can only answer that question over
    /// the whole compilation, which is too late: the body is already in the
    /// module. The `FuncId` is what makes the question askable per body.
    opened: BTreeMap<EffectSeatGroupId, FuncId>,
    committed: BTreeMap<EffectSeatGroupId, CommittedEffectSeatGroup>,
}
/// What the whole-pass seat closeout measured.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct EffectSeatClosure {
    /// Visits that closed complete.
    pub(in crate::cranelift_backend) groups: usize,
    /// Claims across all visits.
    pub(in crate::cranelift_backend) claims: usize,
    /// Distinct planned seats some visit reached — `image(claims)`.
    pub(in crate::cranelift_backend) image: usize,
    pub(in crate::cranelift_backend) population: usize,
    /// Members of `P` no visit reached. **Lawful** — `P` authorizes, it does not
    /// oblige, exactly as the aggregate relation's `P` does. A declaration body
    /// this compilation never emitted takes its occurrence's seats with it.
    /// Reported, never a failure condition.
    pub(in crate::cranelift_backend) unreached: usize,
}
impl EffectSeatLedger {
    /// Open the group for one visit, BEFORE any seat of it is observed.
    fn open_group(
        &mut self,
        function: FuncId,
        effect_origin: StaticOriginId,
        operation: ken_host::HostOpV1,
        planned: BTreeSet<EffectSeatSlot>,
    ) -> Result<EffectSeatGroupId, CraneliftBackendError> {
        if let Some(open) = &self.open {
            return Err(backend_module(format!(
                "host effect seat ledger: {:?} is still open while a visit to {effect_origin:?} \
                 starts, so a seat could be claimed into the wrong visit",
                open.effect_origin
            )));
        }
        if planned.is_empty() {
            return Err(backend_module(format!(
                "host effect seat ledger: {effect_origin:?} is visited but plans no seat at all"
            )));
        }
        let id = effect_seat_group::mint(&mut self.next_group);
        self.opened.insert(id, function);
        self.open = Some(OpenEffectSeatGroup {
            id,
            function,
            effect_origin,
            operation,
            planned,
            claims: BTreeMap::new(),
            unreachable: BTreeSet::new(),
        });
        Ok(id)
    }

    /// The open group, checked against the id the caller believes it holds.
    fn open_group_mut(
        &mut self,
        group: EffectSeatGroupId,
    ) -> Result<&mut OpenEffectSeatGroup, CraneliftBackendError> {
        let open = self.open.as_mut().ok_or_else(|| {
            backend_module(
                "host effect seat ledger: a seat was claimed with no open visit, so it belongs \
                 to no group"
                    .to_string(),
            )
        })?;
        if open.id != group {
            return Err(backend_module(format!(
                "host effect seat ledger: a claim names {group:?} while {:?} is open",
                open.id
            )));
        }
        Ok(open)
    }

    /// Claim one seat into the open visit.
    fn claim(
        &mut self,
        group: EffectSeatGroupId,
        record: PlannedEffectSeat,
        observed: EffectSeatPhase,
        route: EffectSeatClaimRoute,
    ) -> Result<(), CraneliftBackendError> {
        // ⛔ The contract, recomputed from the record's own operation and slot
        // and nothing else, before the record is admitted to a group. This is
        // what makes operation, ordinal and need load-bearing rather than
        // recorded and unread.
        let recomputed = host_effect_seat_contract_of(record.operation, record.slot);
        if recomputed != Some((record.semantic_operation, record.need, record.avail)) {
            return Err(backend_module(format!(
                "host effect seat ledger: {record:?} recomputes from its own operation and slot \
                 to {recomputed:?}, so its recorded contract is not the one its key names"
            )));
        }
        // ⛔ The direct route proves `observed ∈ Avail`. The sole second
        // route is a carried byte-span value at a planner-declared
        // `SiteOperand`; its exact recipe membership was proved by the caller.
        // Keeping the two routes distinct prevents the exception from
        // widening every consumer of the seat.
        let admissible = match route {
            EffectSeatClaimRoute::Direct => record.avail.admits(observed),
            EffectSeatClaimRoute::SiteOperandProjection => {
                observed == EffectSeatPhase::CarriedWord
                    && record.need == EffectSeatNeed::BytesPointerLength
                    && matches!(record.slot, EffectSeatSlot::Argument(_))
            }
            // `RT-RESOURCE-RELEASE-CARRIED-OBSERVE` `D1`. Re-derived HERE from
            // the record's own fields, independently of the caller that chose
            // the route -- which is the point of this second check: a route
            // selected on one set of facts and admitted on another would let a
            // mis-keyed route in. The triple is the same narrow key, spelled
            // again rather than shared, so the two must agree.
            EffectSeatClaimRoute::CarriedResourceObservation => {
                observed == EffectSeatPhase::CarriedWord
                    && record.need == EffectSeatNeed::ResourceScalar
            }
            EffectSeatClaimRoute::CarriedConstructorDispatch => {
                observed == EffectSeatPhase::CarriedWord
                    && record.need == EffectSeatNeed::ConstructorTag
            }
        };
        if !admissible {
            return Err(backend_module(format!(
                "host effect seat ledger: {:?} seat {:?} of {:?} was observed as {observed:?} \
                 through {route:?}, which its planned contract does not admit",
                record.effect_origin, record.slot, record.operation,
            )));
        }
        let open = self.open_group_mut(group)?;
        if record.effect_origin != open.effect_origin || record.operation != open.operation {
            return Err(backend_module(format!(
                "host effect seat ledger: {record:?} was claimed into the visit to {:?} {:?}, so \
                 one occurrence's seat carries another's authority",
                open.effect_origin, open.operation
            )));
        }
        if !open.planned.contains(&record.slot) {
            return Err(backend_module(format!(
                "host effect seat ledger: {:?} is not a planned slot of {:?}",
                record.slot, open.effect_origin
            )));
        }
        if let Some(previous) = open.claims.insert(
            record.slot,
            ClaimedEffectSeat {
                record,
                observed,
                route,
            },
        ) {
            return Err(backend_module(format!(
                "host effect seat ledger: {:?} of {:?} is claimed twice in one visit (first as \
                 {previous:?})",
                record.slot, open.effect_origin
            )));
        }
        Ok(())
    }

    /// **`RT-DEAD-ARM-EFFECT-LOWERING` `D1` -- record a planned seat this visit
    /// REACHED but could not observe, because its arm is proven unreachable.**
    ///
    /// Deliberately NOT `claim`: nothing is attested about the operand's phase
    /// or its route, because nothing was observed. This only keeps the visit's
    /// completeness accounting total.
    fn record_unreachable_seat(
        &mut self,
        group: EffectSeatGroupId,
        slot: EffectSeatSlot,
    ) -> Result<(), CraneliftBackendError> {
        let open = self.open_group_mut(group)?;
        if open.claims.contains_key(&slot) {
            return Err(backend_module(format!(
                "host effect seat ledger: seat {slot:?} of {:?} was already claimed, so recording \
                 it as unobservable would put one seat on both sides of the visit's accounting",
                open.effect_origin
            )));
        }
        open.unreachable.insert(slot);
        Ok(())
    }

    /// Close the visit, before host dispatch or any successful exit.
    ///
    /// ⛔ Group-local slot EQUALITY. Not "at least the ones it read", and not
    /// accumulated with any other visit.
    fn close_group(&mut self, group: EffectSeatGroupId) -> Result<(), CraneliftBackendError> {
        let open = self.open_group_mut(group)?.clone();
        let claimed = open.claims.keys().copied().collect::<BTreeSet<_>>();
        // `RT-DEAD-ARM-EFFECT-LOWERING` `D1`: completeness is asked over the
        // UNION of seats claimed and seats reached-but-unobservable, so the
        // property this gate enforces is unchanged -- every planned seat of the
        // visit was reached. What changed is that a seat on a provably dead arm
        // is accounted for HONESTLY, as reached and not observed, instead of
        // being either claimed falsely or silently dropped. The two sets are
        // disjoint by construction: a seat takes exactly one of the two paths in
        // `claim_host_effect_seat`.
        let reached = claimed
            .union(&open.unreachable)
            .copied()
            .collect::<BTreeSet<_>>();
        if reached != open.planned {
            return Err(backend_module(format!(
                "host effect seat ledger: the visit to {:?} reached {reached:?} but its planned \
                 population is {:?}, so the occurrence was read incompletely",
                open.effect_origin, open.planned
            )));
        }
        self.committed.insert(
            open.id,
            CommittedEffectSeatGroup {
                function: open.function,
                effect_origin: open.effect_origin,
                claims: open.claims,
                unreachable: open.unreachable,
            },
        );
        self.open = None;
        Ok(())
    }

    /// Drop the open visit without closing it, for the discarded-group control.
    /// No production path does this: a visit either closes or the pass fails.
    #[cfg(test)]
    fn discard_open_group_for_tests(&mut self) {
        self.open = None;
    }

    /// Drop one committed group, leaving its `opened` entry, so the whole-pass
    /// `opened = committed` backstop can be asked whether it still fires on its
    /// own. No production path does this either.
    #[cfg(test)]
    pub(super) fn drop_one_committed_group_for_tests(&mut self) {
        if let Some(id) = self.committed.keys().next().copied() {
            self.committed.remove(&id);
        }
    }

    /// **Close one BODY, before it is defined.**
    ///
    /// ⭐ **This is the gate the whole-pass close cannot be.** The whole-pass
    /// version states the same law over the compilation, but it runs after every
    /// `define_function` — so a body that discarded a visit's claims is already
    /// in the module when the contradiction is noticed. The artifact is refused
    /// either way; what changes is whether the defective body was ever defined.
    ///
    /// ⛔ Two clauses, and the second needs the `FuncId` association: no group
    /// for THIS body may still be open, and every group this body opened must be
    /// committed AND committed with this same body. A group opened here and
    /// committed under another `FuncId` would satisfy a bare id comparison.
    pub(super) fn commit_body(&mut self, function: FuncId) -> Result<(), CraneliftBackendError> {
        if let Some(open) = &self.open {
            if open.function == function {
                return Err(backend_module(format!(
                    "host effect seat ledger: the visit to {:?} is still open as function \
                     {function} is defined, so its claims were never closed",
                    open.effect_origin
                )));
            }
        }
        for (id, opened_for) in &self.opened {
            if *opened_for != function {
                continue;
            }
            match self.committed.get(id) {
                Some(committed) if committed.function == function => {}
                Some(committed) => {
                    return Err(backend_module(format!(
                        "host effect seat ledger: {id:?} was opened for function {function} but \
                         committed under {}, so a visit's claims belong to a body that did not \
                         make them",
                        committed.function
                    )));
                }
                None => {
                    return Err(backend_module(format!(
                        "host effect seat ledger: {id:?} was opened for function {function} and \
                         never committed, so a visit's claims were discarded before the body was \
                         defined"
                    )));
                }
            }
        }
        Ok(())
    }

    /// **Close the whole compilation.**
    ///
    /// ⛔ Every opened group committed, and `image(claims) ⊆ P`.
    ///
    /// ⚠ **Deliberately NOT a group per member of `P`.** `P` is an
    /// authorization population — the same law the aggregate relation carries in
    /// this frame — so an unreached member is lawful and reported. It cannot
    /// hide a half-read occurrence, because completeness is a group-local
    /// equality that has already run.
    pub(super) fn close(
        &mut self,
        planned: &[PlannedEffectSeat],
    ) -> Result<EffectSeatClosure, CraneliftBackendError> {
        if let Some(open) = &self.open {
            return Err(backend_module(format!(
                "host effect seat ledger: the visit to {:?} is still open at the close, so it \
                 was never committed",
                open.effect_origin
            )));
        }
        let committed = self.committed.keys().copied().collect::<BTreeSet<_>>();
        let opened = self.opened.keys().copied().collect::<BTreeSet<_>>();
        if committed != opened {
            return Err(backend_module(format!(
                "host effect seat ledger: {} visits opened but {} committed, so a visit's claims \
                 were discarded",
                opened.len(),
                committed.len()
            )));
        }
        let population = planned
            .iter()
            .map(|record| (record.effect_origin, record.slot))
            .collect::<BTreeSet<_>>();
        let mut image = BTreeSet::new();
        let mut claims = 0usize;
        for group in self.committed.values() {
            for claimed in group.claims.values() {
                claims += 1;
                let key = (claimed.record.effect_origin, claimed.record.slot);
                if !population.contains(&key) {
                    return Err(backend_module(format!(
                        "host effect seat ledger: function {} claimed {key:?}, which is not in \
                         the planned population",
                        group.function
                    )));
                }
                image.insert(key);
            }
        }
        Ok(EffectSeatClosure {
            groups: self.committed.len(),
            claims,
            image: image.len(),
            population: population.len(),
            unreached: population.difference(&image).count(),
        })
    }
}
/// The `IoErrorIdentityV1::Other` discriminator, as `io_error_tag`
/// (`ken-host/src/abi_v1.rs`) encodes it: `(payload as u32 as u64) << 32 | 11`.
///
/// It is the only `IOError` variant carrying an integer whose meaning is its
/// payload rather than its discriminator, which is what lets a synthesized
/// pre-dispatch refusal be represented on an `IOError` surface without minting
/// a constructor the host would never produce.
const IO_ERROR_OTHER_DISCRIMINATOR: i64 = 11;
/// `ResourceErrorV1::MalformedResource`, as the wire reply's `detail` field
/// spells it (`ken-host/src/abi_v1.rs`).
///
/// **`RT-CARRIER-BYTESPAN-OBSERVE` `D5`** uses it for a carried word that never
/// denoted a viewable byte span — the observer's outcome `2`.
pub(super) const RESOURCE_ERROR_MALFORMED_RESOURCE: i64 = 1;
/// `ResourceErrorV1::InvalidOffset`, as the wire reply's `detail` field spells
/// it. Named here rather than written as a bare `6` at its call site.
const RESOURCE_ERROR_INVALID_OFFSET: i64 = 6;
/// `ResourceErrorV1::InvalidBounds`, as the wire reply's `detail` field spells
/// it.
///
/// **`RT-CARRIER-BYTESPAN-OBSERVE` `D5`** uses it for a well-formed byte span
/// that failed a containment rule — the observer's outcome `1`. That is the
/// same answer an out-of-range narrowing already gives, and it is the correct
/// one: the value is a real span whose extent is not admissible.
pub(super) const RESOURCE_ERROR_INVALID_BOUNDS: i64 = 7;
/// **`RT-CARRIER-BYTESPAN-OBSERVE` `D5` — one byte-span seat, read in whichever
/// phase its operand actually arrived in.**
///
/// The `(pointer, len)` pair is what the wire request wants either way, so the
/// two phases converge here and the arm that stores them does not know which
/// route produced them — the same shape `BufferAllocate`'s capacity seat already
/// uses.
///
/// `refusal` is the one asymmetry, and it is not an accident of the encoding: a
/// SPECIALIZED template was decided at compile time, so it has no run-time way
/// to fail. A CARRIED word is decided at run time by the helper's guards, so it
/// carries `Some((invalid, resource_code))` — the predicate, and which of the
/// two refusals it is. Folding that into the existing narrow-failure lane is
/// what makes a refusal a typed pre-dispatch reply with **zero host dispatch**,
/// rather than a lowering error or a null read.
///
/// **The second element is a `ResourceErrorV1` CODE, not a finished `detail`,
/// and the distinction is load-bearing.** It names *which* refusal occurred;
/// how that becomes a value depends on the surface the operation declares, and
/// only the caller knows that. An earlier revision returned a finished detail
/// and wrote it straight to the reply, which put a raw resource code on an
/// `IOError` surface — where `1` and `7` decode as `PermissionDenied` and
/// `IsDirectory`. The refusal never reached Ken at all: the reply tag was
/// rejected first and the whole compiled function failed generically.
struct ObservedBytesSeat {
    pointer: cranelift_codegen::ir::Value,
    len: cranelift_codegen::ir::Value,
    refusal: Option<(cranelift_codegen::ir::Value, cranelift_codegen::ir::Value)>,
}
#[cfg(test)]
thread_local! {
    /// `(specialized, carried)` emissions of the `BufferAllocate` capacity arm.
    ///
    /// `pub(super)`: `units.rs`'s own `#[cfg(test)]` accessor fns
    /// (`capacity_phase_dispatch`/`reset_capacity_phase_dispatch`) read this
    /// directly via a qualified `super::effects::CAPACITY_PHASE_DISPATCH`
    /// path -- a sibling-file reach, not a descendant one, so private
    /// (module-local) visibility does not reach it.
    pub(super) static CAPACITY_PHASE_DISPATCH: std::cell::Cell<(usize, usize)> =
        const { std::cell::Cell::new((0, 0)) };
}

impl<'a> Lowering<'a> {
    /// **`RT-CARRIER-BYTESPAN-OBSERVE` `D5` — THE per-seat activation, and the
    /// only place a `BytesPointerLength` seat's phase is dispatched on.**
    ///
    /// Exhaustive over the two phases with no wildcard, for the reason
    /// [`ClaimedEffectSeats::specialized`] gives: the arm that would fire if a
    /// seat's `Avail` were widened without a route being written must name the
    /// seat, not fall into a catch-all. Here both arms have a route, so neither
    /// is the refusal — but the shape is kept so a THIRD phase would break
    /// compilation.
    ///
    /// The seat record handed to the observer is the CLAIMED one, so the
    /// observer's own `need` check is asking about the seat this visit proved,
    /// not one re-resolved behind the claim.
    ///
    /// # `AC-11` — immediate consumption, discharged STRUCTURALLY
    ///
    /// The gate (Architect `dec_5zjh9675253pj`) is that the view must be
    /// consumed before any invalidating operation and never stored or
    /// transported across one. `D5` discharges it by showing the invalidating
    /// operation **is not expressible in the window**, which is stronger than
    /// ordering the emitted calls carefully:
    ///
    /// 1. **What invalidates is Rust-side and takes `&mut BoundaryValueStore`.**
    ///    `BoundaryRegion::reserve` (`boundary_value.rs`) is what resizes
    ///    `data`, and a resize is what moves the table under the pointer.
    ///    `publish_persistent`'s own note — *"invalidated by any later
    ///    materialization or reservation"* — is about those methods.
    /// 2. **Emitted code cannot reach them.** A compiled body holds a raw arena
    ///    pointer and may call only the CLOSED, pinned `BOUNDARY_LOCAL_HELPERS`
    ///    inventory. That inventory has no reserve, grow, resize or publish
    ///    entry, and its allocator refuses at `ARENA_NODE_CAPACITY` /
    ///    `ARENA_DATA_CAPACITY` rather than reallocating.
    /// 3. ⇒ **Within one emitted host-effect body no reservation or
    ///    materialization of the persistent image can occur at all**, so the
    ///    pointer this returns cannot outlive one. It is stored into the wire
    ///    request and consumed by the `host_dispatch` call in the same body.
    ///
    /// The ordering is worth stating exactly, because this lowering *does*
    /// allocate into the carrier — just never in the window. Operand lowering
    /// allocates BEFORE the claim group opens, and reply decoding allocates
    /// AFTER dispatch returns. Between the observation and the dispatch the arm
    /// emits only stack stores, constants, comparisons and other read-only
    /// observers. **Even so, the window argument is the weaker half:** point 2
    /// is what makes an invalidating operation unspellable there, and it holds
    /// however the arm is later reordered.
    ///
    /// **The residual, stated rather than buried:** this is a proof about the
    /// EMITTED window, not about the Rust harness around it. A test rig that
    /// holds a returned pointer in Rust across a `reserve_persistent` is still
    /// reading a moved table — `d4_observe` documents exactly that trap and
    /// copies while the store is alive. **Widening this inventory with a
    /// growing helper would retire the proof**, which is why it rests on the
    /// inventory being closed rather than on a survey of today's call sites.
    fn wire_bytes_seat(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        seats: &ClaimedEffectSeats<'_>,
        slot: EffectSeatSlot,
    ) -> Result<ObservedBytesSeat, CraneliftBackendError> {
        use cranelift_codegen::ir::condcodes::IntCC;
        let (record, operand) = seats.operand(slot)?;
        match operand {
            LoweringOperand::Specialized(lowered) => {
                let lowered = lowered.clone();
                let (pointer, len) = self.wire_bytes(builder, &lowered)?;
                Ok(ObservedBytesSeat { pointer, len, refusal: None })
            }
            LoweringOperand::Carried(word) => {
                let word = *word;
                let (pointer, len, outcome) =
                    self.observe_carried_bytes_span(builder, record, word)?;
                #[cfg(test)]
                let outcome = match effect_seat_dispatch_mutation() {
                    EffectSeatDispatchMutation::ForceByteSpanOutcomeBounds => {
                        builder.ins().iconst(types::I64, 1)
                    }
                    EffectSeatDispatchMutation::ForceByteSpanOutcomeNotASpan => {
                        builder.ins().iconst(types::I64, 2)
                    }
                    _ => outcome,
                };
                // The three-valued outcome is preserved ACROSS this boundary
                // rather than collapsed into one failure: outcome 1 and outcome
                // 2 select different `ResourceErrorV1` codes, so a program can
                // still tell "a real span whose extent is inadmissible" from
                // "this word was never a viewable span". See the observer's doc
                // for what outcome 2 itself already merges.
                //
                // The code is handed UP rather than encoded here, so the arm
                // that knows the operation's declared error surface is the one
                // that decides how it is represented on it.
                let invalid = builder.ins().icmp_imm(IntCC::NotEqual, outcome, 0);
                let bounds = builder.ins().icmp_imm(IntCC::Equal, outcome, 1);
                let out_of_bounds = builder
                    .ins()
                    .iconst(types::I64, RESOURCE_ERROR_INVALID_BOUNDS);
                let malformed = builder
                    .ins()
                    .iconst(types::I64, RESOURCE_ERROR_MALFORMED_RESOURCE);
                let resource_code = builder.ins().select(bounds, out_of_bounds, malformed);
                Ok(ObservedBytesSeat {
                    pointer,
                    len,
                    refusal: Some((invalid, resource_code)),
                })
            }
        }
    }
    fn wire_bytes(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        value: &Lowered,
    ) -> Result<(cranelift_codegen::ir::Value, cranelift_codegen::ir::Value), CraneliftBackendError>
    {
        let pointer_type = builder.func.dfg.value_type(
            self.function_local
                .host_dispatch_context
                .expect("process byte lowering owns a direct host context"),
        );
        match value {
            Lowered::BorrowedNativeValue { pointer } => {
                let kind = builder
                    .ins()
                    .load(types::I64, MemFlags::trusted(), *pointer, 0);
                Self::require_i64(builder, kind, 1);
                Ok((
                    builder
                        .ins()
                        .load(pointer_type, MemFlags::trusted(), *pointer, 16),
                    builder
                        .ins()
                        .load(types::I64, MemFlags::trusted(), *pointer, 24),
                ))
            }
            Lowered::ResponseBytes(span) => Ok((span.pointer(), span.len())),
            Lowered::Bytes(bytes) => {
                if bytes.is_empty() {
                    return Ok((
                        builder.ins().iconst(pointer_type, 0),
                        builder.ins().iconst(types::I64, 0),
                    ));
                }
                let size = u32::try_from(bytes.len())
                    .map_err(|_| unsupported("Effect", "Bytes exceed native stack slot"))?;
                let slot = builder.create_sized_stack_slot(StackSlotData::new(
                    StackSlotKind::ExplicitSlot,
                    size,
                    0,
                ));
                for (offset, byte) in bytes.iter().enumerate() {
                    let byte = builder.ins().iconst(types::I8, i64::from(*byte));
                    builder.ins().stack_store(byte, slot, offset as i32);
                }
                Ok((
                    builder.ins().stack_addr(pointer_type, slot, 0),
                    builder.ins().iconst(types::I64, bytes.len() as i64),
                ))
            }
            _ => Err(unsupported("Effect", "operand is not a Bytes value")),
        }
    }
    pub(super) fn narrow_native_int_u64(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        value: &Lowered,
    ) -> Result<(cranelift_codegen::ir::Value, cranelift_codegen::ir::Value), CraneliftBackendError>
    {
        let Lowered::Int { value, known } = value else {
            return Err(unsupported("Effect", "host-width operand is not Int"));
        };
        let arena = self
            .function_local
            .native_int_arena
            .ok_or_else(|| unsupported("Effect", "host-width Int has no invocation arena"))?;
        let helper = self.function_local.native_int_narrow.ok_or_else(|| {
            unsupported("Effect", "host-width Int has no checked narrowing helper")
        })?;
        let tag = self.native_int_tag(builder, *value, *known)?;
        let output_slot =
            builder.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 8, 3));
        let pointer_type = builder.func.dfg.value_type(arena);
        let output = builder.ins().stack_addr(pointer_type, output_slot, 0);
        let call = builder.ins().call(helper, &[arena, tag, *value, output]);
        let status = builder.inst_results(call)[0];
        Self::require_one_of_i64(builder, status, &[0, 1]);
        let valid =
            builder
                .ins()
                .icmp_imm(cranelift_codegen::ir::condcodes::IntCC::Equal, status, 0);
        let value = builder
            .ins()
            .load(types::I64, MemFlags::trusted(), output, 0);
        Ok((value, valid))
    }
    /// **`RT-DECL-CLOSURE-PORT` `D7` — the CARRIED exact-`Int` narrowing, over
    /// the existing carrier ABI.** `(u64, valid)`, emitted, for a capacity that
    /// reaches its seat as a boundary word rather than a compile-time template.
    ///
    /// ⭐ **One range rule, stated ONCE, over both carried representations.**
    /// The two decoders converge on `narrowed` as the same
    /// `(sign, len, limbs)` triple — the identical shape
    /// `ken_boundary_int_view_local` converges on internally — and the rule
    /// `sign == 0 && len == 1` is applied after they merge. An immediate is
    /// given a one-limb table of its own scalar rather than a second magnitude
    /// encoding, which is what lets the merged code be representation-blind.
    /// Testing the range twice would let the two spellings drift, and the drift
    /// would be invisible: each arm is exercised by different magnitudes, so a
    /// suite can be green with one of them wrong.
    ///
    /// ⛔ **`sign` is a BIT, not a number.** `0` is non-negative and `1` is
    /// negative — `ken_boundary_store_int_limbs_local` refuses anything else,
    /// and the native decoder writes `uextend(payload < 0)`. So the test is
    /// `sign == 0`; a signed `sign >= 0` is **always true** and would admit
    /// every negative `Int` at its magnitude. `len >= 1` always holds for the
    /// same reason — an empty magnitude denotes no integer — so `len == 1` is
    /// the exact "fits one unsigned limb" test rather than a bound.
    ///
    /// ⛔ **Limb 0 is loaded ONLY on the valid path.** A wide magnitude is
    /// refused on `len` before its table is read at all.
    ///
    /// ⛔ **`valid == 0` is the ONLY `InvalidBounds` outcome, and it means
    /// exactly one thing: a well-formed exact `Int` whose value does not fit
    /// `u64`.** Everything else — a word that is not an `Int`, a wrong class or
    /// owner, an unsealed magnitude, a helper that fails — leaves through
    /// `require_i64(.., BOUNDARY_OK)` as a carrier error. That separation is the
    /// framed contract: a caller must not be able to read "out of range" off a
    /// word that never denoted a number.
    ///
    /// ⚠ **The tag branch is a discrimination, not a validation.** It selects
    /// which decoder can read the word; it does not decide the word is good.
    /// Every non-`ImmediateInt` word goes to `int_view`, whose own guards are
    /// the authority — `resolve` rejects a wrong tag or owner, the class guard
    /// rejects a non-`Int` node, and the region path rejects an unsealed
    /// magnitude. Re-deriving any of those here would be a second copy of a rule
    /// that already has one.
    #[cfg(test)]
    fn record_capacity_phase_dispatch(carried: bool) {
        CAPACITY_PHASE_DISPATCH.with(|cell| {
            let (specialized, carried_count) = cell.get();
            cell.set(if carried {
                (specialized, carried_count + 1)
            } else {
                (specialized + 1, carried_count)
            });
        });
    }
    #[cfg(not(test))]
    fn record_capacity_phase_dispatch(_carried: bool) {}
    /// **`RT-CARRIER-BYTESPAN-OBSERVE` `D4` — the lowering-side byte-span
    /// observer.**
    ///
    /// Consumes the exact [`PlannedEffectSeat`] record, emits one
    /// `ken_boundary_bytes_view_local` call, and returns SSA
    /// `(pointer, length, outcome)`.
    ///
    /// ⛔ **It never constructs a [`Lowered`] and never decodes at Rust or JIT
    /// time.** Everything it learns it learns from the helper at run time; the
    /// only compile-time facts it reads are the planner's, off the record it
    /// was handed.
    ///
    /// ⭐⭐ **THE OUTCOME IS THREE-VALUED, and that is the whole point.** `D3`
    /// answers a word that never denoted a byte span and a well-formed span
    /// that fails containment with DIFFERENT statuses, and a caller must not be
    /// able to read one off the other. So this does **not** funnel the status
    /// through [`Self::require_i64`] — that collapses every refusal into one
    /// failure return and would destroy the distinction the helper exists to
    /// make. The discriminant is:
    ///
    /// | outcome | meaning |
    /// |---|---|
    /// | `0` | the span is observable; pointer and length are live |
    /// | `1` | a WELL-FORMED byte span that failed a bounds rule |
    /// | `2` | the word never denoted a byte span at all |
    ///
    /// ⚠ On any non-zero outcome the pointer and length are `0`, so a caller
    /// that ignores the discriminant reads a null span rather than a plausible
    /// one — the failure is loud rather than silently wrong.
    ///
    /// **THE OUTCOME-`2` COLLAPSE, DECIDED BY `D5` RATHER THAN INHERITED.**
    /// `D3` minted four statuses; outcome `2` merges three of them — `ERR_TAG`,
    /// `ERR_CLASS` and `ERR_ESCAPE` — and the row label above is loose for the
    /// last, since an invocation-owned byte span *is* a byte span, just not one
    /// this helper may safely view.
    ///
    /// **`D5` keeps the collapse, deliberately, and the reason is that nothing
    /// downstream can express the distinction.** Per-seat activation maps a
    /// refusal onto a `ResourceErrorV1` code in the wire reply, and Ken's
    /// surface has no constructor that separates "wrong tag" from "wrong class"
    /// from "invocation-owned". All three mean the same thing to a program: this
    /// carried word is not a span this operation may read, decided before any
    /// host dispatch. Splitting them would need a fourth outcome here, a fourth
    /// reply code, and a Ken-visible constructor to receive it — three changes
    /// to carry a distinction no consumer can currently observe.
    ///
    /// ⇒ **What `D5` does NOT collapse is outcome `1` against outcome `2`.**
    /// Those two select *different* reply codes
    /// ([`RESOURCE_ERROR_INVALID_BOUNDS`] and
    /// [`RESOURCE_ERROR_MALFORMED_RESOURCE`]), each reaching the program as the
    /// payload of an `IOError::Other` on the operation's own declared error
    /// surface, so the separation `D3` built the bounds status for survives all
    /// the way into a value a Ken program can match on. That last clause is
    /// witnessed, not asserted:
    /// `d5_the_two_byte_span_refusals_are_distinct_typed_values_without_dispatch`
    /// observes the two codes and reddens if either collapses.
    ///
    /// **If a later node needs an escape refusal diagnosed distinctly, the
    /// information is gone by this point and the change belongs in `D3`'s status
    /// set, not here.**
    ///
    /// ⚠ **ADDRESS-STABILITY CONTRACT (`AC-11`, Architect `dec_5zjh9675253pj`).**
    ///
    /// The returned pointer is an ephemeral view into the persistent image's
    /// current published data table. It remains valid only until the next
    /// materialization or reservation of that image. `PersistentStore` ownership
    /// guarantees the referent's lifetime, not the stability of this interior
    /// address. A consumer must use the pointer and length before any such
    /// operation and must not store or transport the pair across one.
    ///
    /// ⛔ **The SSA pair is a BORROWED VIEW, never a new persistent
    /// representation.** `D5` owns the per-seat proof that the host-marshalling
    /// consumer uses it before any materialization or reservation; retaining it
    /// across one is a hard stop and a separate mechanism decision, not
    /// something this observer may paper over.
    ///
    /// **WIRED by `D5`, which is why there is no `#[allow(dead_code)]` here.**
    /// `D4` landed this dormant, with every `BytesPointerLength` seat still
    /// `SPECIALIZED_ONLY`. [`Self::wire_bytes_seat`] is now the sole caller, and
    /// it is reached from the byte-span seats whose `Avail` `D5` widened. An
    /// observer that still needed the attribute would be an observer nothing
    /// called, so its absence is the evidence the activation is real rather
    /// than a note.
    pub(super) fn observe_carried_bytes_span(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        seat: PlannedEffectSeat,
        target: CarriedBoundaryWord,
    ) -> Result<
        (
            cranelift_codegen::ir::Value,
            cranelift_codegen::ir::Value,
            cranelift_codegen::ir::Value,
        ),
        CraneliftBackendError,
    > {
        use cranelift_codegen::ir::condcodes::IntCC;
        // ⛔ The record is CONSUMED, not decorative. An observer emitted for a
        // seat whose need is not a byte span would be reading a value the
        // planner never said was one, and that is a representation decision
        // taken by the caller rather than by the authority.
        if seat.need != EffectSeatNeed::BytesPointerLength {
            return Err(unsupported(
                "Effect",
                format!(
                    "the byte-span observer was asked for seat {:?} of {:?}, whose need is \
                     {:?} rather than BytesPointerLength",
                    seat.slot, seat.operation, seat.need
                ),
            ));
        }
        let refs = self.carrier_refs()?;
        let boundary_arena = self.carrier_arena()?;
        let pointer_type = builder.func.dfg.value_type(boundary_arena);

        let view_slot =
            builder.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 16, 3));
        let view = builder.ins().stack_addr(pointer_type, view_slot, 0);
        let call = builder
            .ins()
            .call(refs.bytes_view, &[boundary_arena, target.word, view]);
        let status = builder.inst_results(call)[0];

        let observed = builder.create_block();
        let refused = builder.create_block();
        let done = builder.create_block();
        builder.append_block_param(done, pointer_type);
        builder.append_block_param(done, types::I64);
        builder.append_block_param(done, types::I64);
        let ok = builder.ins().icmp_imm(IntCC::Equal, status, BOUNDARY_OK);
        builder.ins().brif(ok, observed, &[], refused, &[]);

        builder.switch_to_block(observed);
        let pointer = builder.ins().stack_load(pointer_type, view_slot, 0);
        let length = builder.ins().stack_load(types::I64, view_slot, 8);
        let good = builder.ins().iconst(types::I64, 0);
        builder
            .ins()
            .jump(done, &[pointer.into(), length.into(), good.into()]);

        // ⛔ The two refusals are separated HERE, from the helper's own status,
        // rather than re-derived from the word. Re-deriving would be a second
        // authority on a question `D3`'s guards already answer.
        builder.switch_to_block(refused);
        let bounded = builder.ins().icmp_imm(
            IntCC::Equal,
            status,
            crate::boundary_value::BOUNDARY_ERR_BOUNDS,
        );
        let null = builder.ins().iconst(pointer_type, 0);
        let empty = builder.ins().iconst(types::I64, 0);
        let out_of_bounds = builder.ins().iconst(types::I64, 1);
        let not_a_span = builder.ins().iconst(types::I64, 2);
        let outcome = builder.ins().select(bounded, out_of_bounds, not_a_span);
        builder
            .ins()
            .jump(done, &[null.into(), empty.into(), outcome.into()]);

        builder.switch_to_block(done);
        let p = builder.block_params(done);
        Ok((p[0], p[1], p[2]))
    }
    /// **`RT-EXACTINT-CARRIED-OBSERVE` `D1` -- one positioned exact-`Int` seat,
    /// read in whichever phase it arrives in.**
    ///
    /// The paired half of moving these seats to `carried_exact_int`. The two
    /// decoders are the existing ones, one per phase, and this is the only
    /// place the pairing is spelled -- so a seat cannot be admitted in a phase
    /// this cannot decode, and no second carried `Int` decode exists to drift
    /// from `narrow_carried_int_u64`.
    ///
    /// Both arms return `(value, valid)`. `valid = 0` is a LAWFUL outcome, not
    /// an error: it feeds the operation's existing narrow-failure lane
    /// (`InvalidBounds` / `InvalidOffset`) exactly as the specialized phase
    /// already did for an out-of-range `Int`. The carried arm additionally
    /// fail-closes on a word that is not a decodable `Int` at all -- the tag
    /// branch's viewed path `require_i64`s its status -- so there is no misread
    /// path and no route-level guard is needed on top.
    fn narrow_positioned_int_seat(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        seats: &ClaimedEffectSeats<'_>,
        index: u32,
        name: &'static str,
    ) -> Result<(cranelift_codegen::ir::Value, cranelift_codegen::ir::Value), CraneliftBackendError>
    {
        let (_, operand) = seats.operand(EffectSeatSlot::Argument(index))?;
        match operand {
            LoweringOperand::Specialized(value @ Lowered::Int { .. }) => {
                let value = value.clone();
                self.narrow_native_int_u64(builder, &value)
            }
            LoweringOperand::Specialized(_) => Err(unsupported(
                "Effect",
                format!("positioned {name} operand is not Int"),
            )),
            LoweringOperand::Carried(word) => {
                let word = *word;
                self.narrow_carried_int_u64(builder, word)
            }
        }
    }

    fn narrow_carried_int_u64(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        target: CarriedBoundaryWord,
    ) -> Result<(cranelift_codegen::ir::Value, cranelift_codegen::ir::Value), CraneliftBackendError>
    {
        use cranelift_codegen::ir::condcodes::IntCC;
        let refs = self.carrier_refs()?;
        let boundary_arena = self.carrier_arena()?;
        let pointer_type = builder.func.dfg.value_type(boundary_arena);

        let tag = builder
            .ins()
            .band_imm(target.word, crate::boundary_value::BOUNDARY_TAG_MASK as i64);
        let is_immediate_int = builder.ins().icmp_imm(
            IntCC::Equal,
            tag,
            crate::boundary_value::BoundaryTag::ImmediateInt as i64,
        );
        let immediate = builder.create_block();
        let viewed = builder.create_block();
        // `(sign, len, limbs)` — the canonical triple, from either decoder.
        let narrowed = builder.create_block();
        builder.append_block_param(narrowed, types::I64);
        builder.append_block_param(narrowed, types::I64);
        builder.append_block_param(narrowed, pointer_type);
        builder
            .ins()
            .brif(is_immediate_int, immediate, &[], viewed, &[]);

        // ── the immediate payload ────────────────────────────────────────
        //
        // The exact tag is validated by the branch above; the scalar comes from
        // the carrier's own helper rather than a shift written here, so the
        // immediate decode has one implementation and this is a caller of it.
        builder.switch_to_block(immediate);
        let scalar = self.emit_carrier_scalar(builder, target)?;
        // The sign BIT, spelled as the decoder spells it.
        let negative = builder.ins().icmp_imm(IntCC::SignedLessThan, scalar, 0);
        let immediate_sign = builder.ins().uextend(types::I64, negative);
        let immediate_len = builder.ins().iconst(types::I64, 1);
        // ⭐ A one-limb table holding the scalar. An immediate's whole
        // magnitude IS one limb, so giving it a table makes the merged rule
        // read it exactly as it reads a persistent one — no second encoding,
        // and no branch below that has to know which arm it came from.
        let immediate_slot = builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            8,
            3,
        ));
        builder.ins().stack_store(scalar, immediate_slot, 0);
        let immediate_limbs = builder.ins().stack_addr(pointer_type, immediate_slot, 0);
        builder.ins().jump(
            narrowed,
            &[immediate_sign.into(), immediate_len.into(), immediate_limbs.into()],
        );

        // ── the sealed persistent / native `Int` view ────────────────────
        builder.switch_to_block(viewed);
        let view_slot = builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            24,
            3,
        ));
        let view = builder.ins().stack_addr(pointer_type, view_slot, 0);
        let call = builder
            .ins()
            .call(refs.int_view, &[boundary_arena, target.word, view]);
        Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
        let view_sign = builder.ins().stack_load(types::I64, view_slot, 0);
        let view_len = builder.ins().stack_load(types::I64, view_slot, 8);
        let view_limbs = builder.ins().stack_load(pointer_type, view_slot, 16);
        builder.ins().jump(
            narrowed,
            &[view_sign.into(), view_len.into(), view_limbs.into()],
        );

        // ── the shared rule ──────────────────────────────────────────────
        builder.switch_to_block(narrowed);
        let sign = builder.block_params(narrowed)[0];
        let len = builder.block_params(narrowed)[1];
        let limbs = builder.block_params(narrowed)[2];
        let non_negative = builder.ins().icmp_imm(IntCC::Equal, sign, 0);
        let one_limb = builder.ins().icmp_imm(IntCC::Equal, len, 1);
        let in_range = builder.ins().band(non_negative, one_limb);
        let read_limb = builder.create_block();
        let out_of_range = builder.create_block();
        let done = builder.create_block();
        builder.append_block_param(done, types::I64);
        builder.append_block_param(done, types::I8);
        builder
            .ins()
            .brif(in_range, read_limb, &[], out_of_range, &[]);

        builder.switch_to_block(read_limb);
        let value = builder
            .ins()
            .load(types::I64, MemFlags::trusted(), limbs, 0);
        let valid = builder.ins().iconst(types::I8, 1);
        builder.ins().jump(done, &[value.into(), valid.into()]);

        // ⛔ The magnitude is NOT read here. A wide `Int` is refused on its
        // length before its limb table is touched, which is what makes "loading
        // limb 0 only on the valid path" a property of the emitted code rather
        // than of the values a fixture happens to pass.
        builder.switch_to_block(out_of_range);
        let absent = builder.ins().iconst(types::I64, 0);
        let invalid = builder.ins().iconst(types::I8, 0);
        builder.ins().jump(done, &[absent.into(), invalid.into()]);

        builder.switch_to_block(done);
        Ok((builder.block_params(done)[0], builder.block_params(done)[1]))
    }
    pub(super) fn lower_dynamic_small_int(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        value: cranelift_codegen::ir::Value,
    ) -> Lowered {
        let tag = builder
            .ins()
            .iconst(types::I64, crate::NATIVE_INT_SMALL_TAG_V1 as i64);
        self.function_local.native_int_tags.insert(value, tag);
        Lowered::Int { value, known: None }
    }
    fn require_u8(builder: &mut FunctionBuilder<'_>, value: cranelift_codegen::ir::Value) {
        let valid = builder.create_block();
        let invalid = builder.create_block();
        let in_range = builder.ins().icmp_imm(
            cranelift_codegen::ir::condcodes::IntCC::UnsignedLessThanOrEqual,
            value,
            i64::from(u8::MAX),
        );
        builder.ins().brif(in_range, valid, &[], invalid, &[]);
        builder.switch_to_block(invalid);
        let failure = builder.ins().iconst(types::I64, -1);
        builder.ins().return_(&[failure]);
        builder.switch_to_block(valid);
    }
    fn require_true(builder: &mut FunctionBuilder<'_>, condition: cranelift_codegen::ir::Value) {
        let valid = builder.create_block();
        let invalid = builder.create_block();
        builder.ins().brif(condition, valid, &[], invalid, &[]);
        builder.switch_to_block(invalid);
        let failure = builder.ins().iconst(types::I64, -1);
        builder.ins().return_(&[failure]);
        builder.switch_to_block(valid);
    }
    fn require_when(
        builder: &mut FunctionBuilder<'_>,
        enabled: cranelift_codegen::ir::Value,
        condition: cranelift_codegen::ir::Value,
    ) {
        let validate = builder.create_block();
        let done = builder.create_block();
        builder.ins().brif(enabled, validate, &[], done, &[]);
        builder.switch_to_block(validate);
        Self::require_true(builder, condition);
        builder.ins().jump(done, &[]);
        builder.switch_to_block(done);
    }
    /// `request_length` is the RAW pre-clamp request length — an outer
    /// consistency ceiling and audit input only, never a progress bound
    /// (`BUDGET-EFF`, Architect ruling `dec_1m6xdwjp2ttyn`, boundary
    /// constraint 3). `effective_request` is the host's post-clamp bound,
    /// carried in the reply; range/no-wrap/span-containment and `remaining`
    /// are all derived from it, and `0 < count <= effective_request <=
    /// request_length` is asserted before minting.
    pub(super) fn mint_validated_progress_nat(
        builder: &mut FunctionBuilder<'_>,
        success: cranelift_codegen::ir::Value,
        count: cranelift_codegen::ir::Value,
        request_start: cranelift_codegen::ir::Value,
        request_length: cranelift_codegen::ir::Value,
        effective_request: cranelift_codegen::ir::Value,
        reply_start: Option<cranelift_codegen::ir::Value>,
    ) -> (BoundedNatV1, BoundedNatV1, BoundedNatV1) {
        let positive = builder.ins().icmp_imm(
            cranelift_codegen::ir::condcodes::IntCC::UnsignedGreaterThan,
            count,
            0,
        );
        let bounded = builder.ins().icmp(
            cranelift_codegen::ir::condcodes::IntCC::UnsignedLessThanOrEqual,
            count,
            effective_request,
        );
        let effective_within_raw = builder.ins().icmp(
            cranelift_codegen::ir::condcodes::IntCC::UnsignedLessThanOrEqual,
            effective_request,
            request_length,
        );
        let effective_end = builder.ins().iadd(request_start, effective_request);
        let effective_no_wrap = builder.ins().icmp(
            cranelift_codegen::ir::condcodes::IntCC::UnsignedGreaterThanOrEqual,
            effective_end,
            request_start,
        );
        let span_start = reply_start.unwrap_or(request_start);
        let span_end = builder.ins().iadd(span_start, count);
        let span_no_wrap = builder.ins().icmp(
            cranelift_codegen::ir::condcodes::IntCC::UnsignedGreaterThanOrEqual,
            span_end,
            span_start,
        );
        let starts_at_request = builder.ins().icmp(
            cranelift_codegen::ir::condcodes::IntCC::Equal,
            span_start,
            request_start,
        );
        let inside = builder.ins().icmp(
            cranelift_codegen::ir::condcodes::IntCC::UnsignedLessThanOrEqual,
            span_end,
            effective_end,
        );
        let valid = [
            positive,
            bounded,
            effective_within_raw,
            effective_no_wrap,
            span_no_wrap,
            starts_at_request,
            inside,
        ]
        .into_iter()
        .reduce(|left, right| builder.ins().band(left, right))
        .expect("progress validation has fixed clauses");
        Self::require_when(builder, success, valid);

        let minted = BoundedNatV1::mint_after_reply_validation(count);
        let predecessor = minted.predecessor(builder);
        let remaining =
            BoundedNatV1::derived_from_validated(builder.ins().isub(effective_request, count));
        (minted, predecessor, remaining)
    }
    fn validate_resource_io(
        builder: &mut FunctionBuilder<'_>,
        encoded: cranelift_codegen::ir::Value,
    ) {
        let discriminator = builder.ins().band_imm(encoded, 0xff);
        let other = builder.create_block();
        let ordinary = builder.create_block();
        let valid = builder.create_block();
        let is_other = builder.ins().icmp_imm(
            cranelift_codegen::ir::condcodes::IntCC::Equal,
            discriminator,
            11,
        );
        builder.ins().brif(is_other, other, &[], ordinary, &[]);
        builder.switch_to_block(other);
        let middle = builder
            .ins()
            .band_imm(encoded, 0x0000_0000_ffff_ff00u64 as i64);
        Self::require_i64(builder, middle, 0);
        builder.ins().jump(valid, &[]);
        builder.switch_to_block(ordinary);
        let upper = builder.ins().ushr_imm(encoded, 8);
        Self::require_i64(builder, upper, 0);
        Self::require_one_of_i64(builder, discriminator, &[0, 1, 3, 4, 5, 6, 7, 8, 9, 10]);
        builder.ins().jump(valid, &[]);
        builder.switch_to_block(valid);
    }
    #[allow(clippy::too_many_arguments)]
    fn validate_resource_error_reply(
        builder: &mut FunctionBuilder<'_>,
        reply_tag: cranelift_codegen::ir::Value,
        resource_reply_tag: u64,
        discriminator: cranelift_codegen::ir::Value,
        schema: cranelift_codegen::ir::Value,
        kind: cranelift_codegen::ir::Value,
        identity: cranelift_codegen::ir::Value,
        io: cranelift_codegen::ir::Value,
        required: cranelift_codegen::ir::Value,
        held: cranelift_codegen::ir::Value,
        actual_expected_kind: cranelift_codegen::ir::Value,
        actual_actual_kind: cranelift_codegen::ir::Value,
        resource_error_tags_in_payload_shape_order: [u64; 10],
        expected_schema: u64,
        expected_kind: u64,
        buffer_kind: u64,
    ) {
        let resource = builder.create_block();
        let done = builder.create_block();
        let is_resource = builder.ins().icmp_imm(
            cranelift_codegen::ir::condcodes::IntCC::Equal,
            reply_tag,
            resource_reply_tag as i64,
        );
        builder.ins().brif(is_resource, resource, &[], done, &[]);
        builder.switch_to_block(resource);
        let mut resource_error_tags = resource_error_tags_in_payload_shape_order.into_iter();
        let mut next_resource_error_tag = || {
            resource_error_tags
                .next()
                .expect("resource error payload shape is complete")
        };
        let closed_tag = next_resource_error_tag();
        let malformed_reply_tag = next_resource_error_tag();
        let right_not_held_tag = next_resource_error_tag();
        let release_failed_tag = next_resource_error_tag();
        let kind_mismatch_tag = next_resource_error_tag();
        let buffer_limit_tag = next_resource_error_tag();
        let invalid_offset_tag = next_resource_error_tag();
        let invalid_bounds_tag = next_resource_error_tag();
        let no_progress_tag = next_resource_error_tag();
        let allocation_failed_tag = next_resource_error_tag();
        let arms = [
            closed_tag,
            malformed_reply_tag,
            right_not_held_tag,
            release_failed_tag,
            kind_mismatch_tag,
        ]
        .map(|tag| (tag, builder.create_block()));
        let mut test = builder
            .current_block()
            .expect("resource reply validation block");
        for (index, (discriminator_tag, arm)) in arms.into_iter().enumerate() {
            let next = builder.create_block();
            if builder.current_block() != Some(test) {
                builder.switch_to_block(test);
            }
            let selected = builder.ins().icmp_imm(
                cranelift_codegen::ir::condcodes::IntCC::Equal,
                discriminator,
                i64::try_from(discriminator_tag).expect("resource error tag fits i64"),
            );
            builder.ins().brif(selected, arm, &[], next, &[]);
            builder.switch_to_block(arm);
            match index {
                0 | 1 => {
                    for field in [
                        schema,
                        kind,
                        identity,
                        io,
                        required,
                        held,
                        actual_expected_kind,
                        actual_actual_kind,
                    ] {
                        Self::require_i64(builder, field, 0);
                    }
                }
                2 => {
                    Self::require_i64(builder, schema, expected_schema as i64);
                    Self::require_i64(builder, kind, 0);
                    Self::require_i64(builder, identity, 0);
                    Self::require_i64(builder, io, 0);
                    Self::require_i64(builder, actual_expected_kind, 0);
                    Self::require_i64(builder, actual_actual_kind, 0);
                    Self::require_u8(builder, required);
                    Self::require_u8(builder, held);
                }
                3 => {
                    Self::require_i64(builder, schema, expected_schema as i64);
                    Self::require_one_of_i64(
                        builder,
                        kind,
                        &[expected_kind as i64, buffer_kind as i64],
                    );
                    Self::require_i64(builder, required, 0);
                    Self::require_i64(builder, held, 0);
                    Self::require_i64(builder, actual_expected_kind, 0);
                    Self::require_i64(builder, actual_actual_kind, 0);
                    Self::validate_resource_io(builder, io);
                }
                4 => {
                    for field in [schema, kind, identity, io, required, held] {
                        Self::require_i64(builder, field, 0);
                    }
                    Self::require_one_of_i64(
                        builder,
                        actual_expected_kind,
                        &[expected_kind as i64, buffer_kind as i64],
                    );
                    Self::require_one_of_i64(
                        builder,
                        actual_actual_kind,
                        &[expected_kind as i64, buffer_kind as i64],
                    );
                    let distinct = builder.ins().icmp(
                        cranelift_codegen::ir::condcodes::IntCC::NotEqual,
                        actual_expected_kind,
                        actual_actual_kind,
                    );
                    Self::require_true(builder, distinct);
                }
                _ => unreachable!(),
            }
            builder.ins().jump(done, &[]);
            test = next;
        }
        builder.switch_to_block(test);
        Self::require_one_of_i64(
            builder,
            discriminator,
            &[
                buffer_limit_tag,
                invalid_offset_tag,
                invalid_bounds_tag,
                no_progress_tag,
                allocation_failed_tag,
            ]
            .map(|tag| i64::try_from(tag).expect("resource error tag fits i64")),
        );
        for field in [
            schema,
            kind,
            identity,
            io,
            required,
            held,
            actual_expected_kind,
            actual_actual_kind,
        ] {
            Self::require_i64(builder, field, 0);
        }
        builder.ins().jump(done, &[]);
        builder.switch_to_block(done);
    }

    /// Emit the one-sided finite dispatcher for a carried constructor seat.
    ///
    /// Every success edge is named by a planner-issued constructor identity,
    /// exact field count, and (where present) exact positional child identity
    /// and arity. The unmatched edge returns the compiled function's
    /// deterministic failure value. No host request is dispatched before this
    /// method returns a wire tag, so a non-matching word cannot be marshalled.
    fn emit_carried_constructor_dispatch(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        word: CarriedBoundaryWord,
        paths: &[EffectSeatConstructorPath],
    ) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
        if paths.is_empty() {
            return Err(unsupported(
                "Effect",
                "a carried constructor seat has no artifact-static dispatch path",
            ));
        }
        let root_tag = self.emit_carrier_tag(builder, word)?;
        let root_field_count = self.emit_carrier_field_count(builder, word)?;
        let done = builder.create_block();
        builder.append_block_param(done, types::I64);

        for path in paths {
            let selected = builder.create_block();
            let next = builder.create_block();
            let root_identity = match path {
                EffectSeatConstructorPath::Root { identity, .. } => *identity,
                EffectSeatConstructorPath::PositionalChild { root_identity, .. } => *root_identity,
            };
            let expected_root =
                Self::carrier_identity_immediate(builder, root_identity.tag_abi_word()?);
            let root_matches = builder.ins().icmp(
                cranelift_codegen::ir::condcodes::IntCC::Equal,
                root_tag,
                expected_root,
            );
            builder.ins().brif(root_matches, selected, &[], next, &[]);
            builder.switch_to_block(selected);

            match path {
                EffectSeatConstructorPath::Root {
                    field_count,
                    wire_tag,
                    ..
                } => {
                    Self::require_i64(builder, root_field_count, i64::from(*field_count));
                    let wire_tag = builder.ins().iconst(types::I64, *wire_tag);
                    builder.ins().jump(done, &[wire_tag]);
                }
                EffectSeatConstructorPath::PositionalChild {
                    root_field_count: expected_root_fields,
                    child_position,
                    child_identity,
                    child_field_count,
                    wire_tag,
                    ..
                } => {
                    Self::require_i64(builder, root_field_count, i64::from(*expected_root_fields));
                    let child_position = usize::try_from(*child_position).map_err(|_| {
                        unsupported(
                            "Effect",
                            "a constructor child position exceeds the target index space",
                        )
                    })?;
                    let child = self.emit_carrier_field(builder, word, child_position)?;
                    let child_tag = self.emit_carrier_tag(builder, child)?;
                    let child_fields = self.emit_carrier_field_count(builder, child)?;
                    Self::require_i64(builder, child_fields, i64::from(*child_field_count));
                    let expected_child =
                        Self::carrier_identity_immediate(builder, child_identity.tag_abi_word()?);
                    let child_matches = builder.ins().icmp(
                        cranelift_codegen::ir::condcodes::IntCC::Equal,
                        child_tag,
                        expected_child,
                    );
                    let matched = builder.create_block();
                    builder.ins().brif(child_matches, matched, &[], next, &[]);
                    builder.switch_to_block(matched);
                    let wire_tag = builder.ins().iconst(types::I64, *wire_tag);
                    builder.ins().jump(done, &[wire_tag]);
                }
            }
            builder.switch_to_block(next);
        }

        // The closed table had no exact member. This is the accept/refuse
        // authority for Route A: return before any host dispatch rather than
        // coerce the word to a convenient tag.
        let failure = builder.ins().iconst(types::I64, -1);
        builder.ins().return_(&[failure]);
        builder.switch_to_block(done);
        Ok(builder.block_params(done)[0])
    }

    /// Read one constructor-tag seat in either phase. Specialized templates
    /// retain their established classifier. A carried word must pass the exact
    /// planner-issued finite dispatcher above; no `Avail` widening is involved.
    fn wire_constructor_tag_seat(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        seats: &ClaimedEffectSeats<'_>,
        slot: EffectSeatSlot,
        classify_specialized: fn(&Lowered) -> Option<i64>,
        malformed: &'static str,
    ) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
        let (record, operand) = seats.operand(slot)?;
        match operand {
            LoweringOperand::Specialized(lowered) => classify_specialized(lowered)
                .map(|wire_tag| builder.ins().iconst(types::I64, wire_tag))
                .ok_or_else(|| unsupported("Effect", malformed)),
            LoweringOperand::Carried(word) => {
                let paths = self
                    .static_transition_plan
                    .host_effect_constructor_dispatch(record.operation, record.slot)?
                    .ok_or_else(|| {
                        unsupported(
                            "Effect",
                            format!(
                                "seat {:?} of {:?} has no constructor dispatcher",
                                record.slot, record.operation
                            ),
                        )
                    })?;
                self.emit_carried_constructor_dispatch(builder, *word, &paths)
            }
        }
    }

    ///
    /// **`RT-RESOURCE-RELEASE-CARRIED-OBSERVE` `D1` -- generalized to `owner` so
    /// there is ONE guarded resource-token observation, not two copies.** The
    /// carried route this node adds for `ResourceRelease` performs the SAME
    /// guards and the SAME read; duplicating them would create a second
    /// authority over what proves a carried word is a resource token, and the
    /// two would drift. `owner` names the operation only for the refusal
    /// message.
    pub(super) fn lower_resource_token_seat(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        operand: &LoweringOperand,
        owner: &'static str,
        seat: &'static str,
    ) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
        match operand {
            LoweringOperand::Specialized(Lowered::ResourceToken { value }) => Ok(*value),
            LoweringOperand::Specialized(_) => Err(unsupported(
                "Effect",
                format!("{owner} {seat} is not a resource"),
            )),
            LoweringOperand::Carried(word) => {
                let tag = builder.ins().band_imm(
                    word.word,
                    crate::boundary_value::BOUNDARY_TAG_MASK as i64,
                );
                Self::require_i64(
                    builder,
                    tag,
                    crate::boundary_value::BoundaryTag::InvocationBorrowed as i64,
                );
                let class = self.emit_carrier_class(builder, *word)?;
                Self::require_i64(builder, class, BoundaryClass::BorrowedOpaque as i64);
                self.emit_carrier_scalar(builder, *word)
            }
        }
    }
    /// `static_origin` is the `Effect` occurrence's own origin.
    ///
    /// ⚠ HAZARD 2 (D3): the planner plans `capability.value` **first when it is
    /// present**, so the argument base is `1` with a capability and `0` without
    /// one. `argument_base` below is that conditional offset, computed once from
    /// the same `Option` the planner tested (`static_transition.rs` `Effect`
    /// arm) rather than assumed.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn lower_process_host_effect(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        family: &RuntimeSymbol,
        operation: ken_host::HostOpV1,
        capability: Option<&crate::RuntimeCapabilityUse>,
        args: &[RuntimeExpr],
        static_origin: StaticOriginId,
        env: &[LoweringEnvironmentBinding],
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        if !CRANELIFT_HOST_EFFECT_CONSUMERS_V1.contains(&operation) {
            // `RT-DEAD-ARM-EFFECT-LOWERING` `D1` -- the SECOND refusal site, and
            // the same question as the seat's.
            //
            // A total handler names every operation of its request type, so an
            // arm for an operation this backend does not represent refuses here
            // BEFORE any seat is claimed. On a provably dead arm that refusal
            // fails the whole object emission for a lane no execution reaches --
            // the identical defect, one check earlier. Measured on the governed
            // fixtures: seven arms (FsAppendFile, FsMetadata, FsReadDirectory,
            // FsCreateDirectory, FsRemoveFile, FsRemoveDirectory, FsRename).
            //
            // Same fail-closed substitute for the same reason: an arm wrongly
            // proven dead HALTS rather than issuing an operation this backend
            // cannot represent.
            if self.effect_arm_is_provably_dead(static_origin)? {
                self.disposition_dead_arm_joins(static_origin)?;
                return Ok(LoweringOperand::Specialized(Lowered::Trap(
                    dead_arm_effect_trap(family, operation),
                )));
            }
            return Err(unsupported(
                "Effect",
                format!(
                    "effect {family}.{} is a represented unavailable lane",
                    operation as u16
                ),
            ));
        }
        let argument_base = usize::from(capability.is_some());
        let lowered = args
            .iter()
            .enumerate()
            .map(|(position, argument)| {
                let argument =
                    self.child_occurrence(static_origin, argument_base + position, argument)?;
                self.lower_expr(builder, argument, env)
            })
            .collect::<Result<Vec<_>, _>>()?;
        // ⛔ The capability is lowered HERE rather than inside the FS arm, and
        // the reason is the claim group's window. An operand's own lowering can
        // visit a nested effect, so every operand must be in hand before the
        // group opens — a capability lowered inside the arm would lower while
        // this occurrence's group was open and its nested claims would land in
        // the wrong visit.
        let capability_operand = match capability {
            Some(capability) => {
                // Present ⇒ the capability value is child 0 of this occurrence.
                let value = self.child_occurrence(static_origin, 0, &capability.value)?;
                Some(self.lower_expr(builder, value, env)?)
            }
            None => None,
        };
        // ⭐ `D7` — ONE claim group per compiler-side visit to this occurrence,
        // opened before any seat of it is observed and closed before dispatch.
        // Completeness is asked of this visit alone: a ledger accumulating
        // claims per occurrence would accept two visits that each read half the
        // seats, and two half-reads are the defect.
        let group = self.open_host_effect_seat_group(static_origin, operation)?;
        #[cfg(test)]
        let omitted = match effect_seat_visit_mutation() {
            // Alternate which slot this visit drops, so successive visits'
            // omissions are complementary and their union is complete.
            EffectSeatVisitMutation::OmitComplementary => {
                let slots = self
                    .static_transition_plan
                    .host_effect_seat_slots(static_origin)
                    .into_iter()
                    .collect::<Vec<_>>();
                let index = effect_seat_next_visit_index();
                slots.get(index % slots.len().max(1)).copied()
            }
            _ => None,
        };
        #[cfg(not(test))]
        let omitted: Option<EffectSeatSlot> = None;
        let mut claimed = BTreeMap::new();
        // `RT-DEAD-ARM-EFFECT-LOWERING` `D1`: set when any seat of this
        // occurrence reports its arm provably unreachable. Recorded rather than
        // returned early, so the seat visit still completes and the claim group
        // still closes -- an early return here would leave `D7`'s group open and
        // trade one refusal for a different, misattributed one.
        let mut unreachable_arm = false;
        let mut claim = |lowering: &mut Self,
                         claimed: &mut BTreeMap<EffectSeatSlot, PlannedEffectSeat>,
                         unreachable_arm: &mut bool,
                         slot,
                         operand: &LoweringOperand| {
            if omitted == Some(slot) {
                return Ok(());
            }
            match lowering.claim_host_effect_seat(group, static_origin, slot, operand)? {
                SeatClaimOutcome::Claimed(record) => {
                    claimed.insert(slot, record);
                }
                SeatClaimOutcome::UnreachableArm => *unreachable_arm = true,
            }
            Ok(())
        };
        if let Some(operand) = &capability_operand {
            claim(
                self,
                &mut claimed,
                &mut unreachable_arm,
                EffectSeatSlot::Capability,
                operand,
            )?;
        }
        for (ordinal, operand) in lowered.iter().enumerate() {
            let ordinal = u32::try_from(ordinal).map_err(|_| {
                unsupported(
                    "Effect",
                    "host effect argument ordinal exceeds the seat space",
                )
            })?;
            claim(
                self,
                &mut claimed,
                &mut unreachable_arm,
                EffectSeatSlot::Argument(ordinal),
                operand,
            )?;
        }
        #[cfg(test)]
        if effect_seat_visit_mutation() == EffectSeatVisitMutation::DuplicateWithinVisit {
            if let Some(operand) = lowered.first() {
                claim(
                    self,
                    &mut claimed,
                    &mut unreachable_arm,
                    EffectSeatSlot::Argument(0),
                    operand,
                )?;
            }
        }
        self.close_host_effect_seat_group(group)?;
        // **`RT-DEAD-ARM-EFFECT-LOWERING` `D1` -- the substitution, after the
        // claim group closes.**
        //
        // The arm this effect sits in is PROVEN never-constructed program-wide,
        // so no execution selects it. Lower the whole effect to a trap rather
        // than a wire request: the arm KEEPS its place in the match, so totality
        // and control flow are unchanged -- nothing is elided -- and if a request
        // value ever reaches here from outside the census's view the program
        // halts instead of issuing a host operation it could not observe a seat
        // for.
        if unreachable_arm {
            return Ok(LoweringOperand::Specialized(Lowered::Trap(
                dead_arm_effect_trap(family, operation),
            )));
        }
        // ⭐ **The bulk pre-operation conversion is gone.** It crossed every
        // operand to a specialized template BEFORE the operation was known, so a
        // seat that could not be read that way failed as "a host-effect operand"
        // -- naming neither the operation nor the seat, and answering for seats
        // the arm never reads. Each arm now reads its own claimed seats.
        let seats = ClaimedEffectSeats {
            claimed: &claimed,
            capability: capability_operand.as_ref(),
            arguments: &lowered,
        };
        // The four ordinals any arm below reads, named once. ⛔ These are
        // SEMANTIC ordinals: the capability offset is already applied, so
        // `SEAT_0` is the first semantic argument whether or not the operation
        // carries a capability.
        const SEAT_0: EffectSeatSlot = EffectSeatSlot::Argument(0);
        const SEAT_1: EffectSeatSlot = EffectSeatSlot::Argument(1);
        const SEAT_2: EffectSeatSlot = EffectSeatSlot::Argument(2);
        const SEAT_3: EffectSeatSlot = EffectSeatSlot::Argument(3);
        let pointer_type = builder.func.dfg.value_type(
            self.function_local
                .host_dispatch_context
                .expect("process effect lowering owns a direct host context"),
        );
        let wire = ken_host::host_effect_wire_layout_v1(operation).map_err(|error| {
            unsupported(
                "Effect",
                format!("generated HostEffectAbiV1 layout rejected: {error:?}"),
            )
        })?;
        let request_offset = |index: usize| {
            i32::try_from(wire.request_offsets[index])
                .expect("C-probed request offset was checked as u32")
        };
        let request = builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            wire.request_size,
            wire.request_align_shift,
        ));
        // The two reply surfaces a synthesized pre-dispatch failure can land on,
        // materialized once so no arm re-derives one.
        let resource_error_reply_tag = builder
            .ins()
            .iconst(types::I64, wire.reply_resource_error_tag as i64);
        let error_reply_tag = builder
            .ins()
            .iconst(types::I64, wire.reply_error_tag as i64);
        // Wrap a `ResourceErrorV1` code as `IOError::Other <code>`, in the exact
        // encoding `ken_host::abi_v1::io_error_tag` uses: the payload in the
        // high 32 bits, discriminator `11` in the low byte, which the decoder
        // below recovers with `sshr_imm(detail, 32)`.
        //
        // `ConsoleWrite` and `FsWriteFile` declare `IOError` surfaces, so their
        // `detail` is read as an `IOError` discriminator. Handing them a RAW
        // resource code would silently reinterpret `1` and `7` as
        // `PermissionDenied` and `IsDirectory` — two real, wrong errors. `Other`
        // is the one variant that carries an integer whose meaning is the
        // payload rather than the discriminator, which is why the observer's
        // refusal can be represented on this surface at all without inventing a
        // constructor.
        let io_error_other_detail =
            |builder: &mut FunctionBuilder<'_>, code: cranelift_codegen::ir::Value| {
                let payload = builder.ins().ishl_imm(code, 32);
                builder.ins().bor_imm(payload, IO_ERROR_OTHER_DISCRIMINATOR)
            };
        // `(invalid, reply tag, detail)`. The TAG is carried rather than assumed
        // because a synthesized pre-dispatch failure must land on the surface
        // the operation actually declares: the resource operations accept
        // `reply_resource_error_tag`, and `ConsoleWrite` / `FsWriteFile` accept
        // only `success` or `reply_error_tag`. Writing the wrong one is not a
        // mis-labelled error — `require_one_of_i64` refuses the reply outright
        // and the whole compiled function fails generically, which is the defect
        // this triple exists to make unspellable.
        let mut narrow_failure: Option<(
            cranelift_codegen::ir::Value,
            cranelift_codegen::ir::Value,
            cranelift_codegen::ir::Value,
        )> = None;
        let mut positioned_bounds: Option<(
            cranelift_codegen::ir::Value,
            cranelift_codegen::ir::Value,
        )> = None;
        // `RT-CARRIER-BYTESPAN-OBSERVE` `D5` — `detail` is a VALUE rather than a
        // constant, because a byte-span refusal chooses its `ResourceErrorV1`
        // code at run time from the observer's outcome. Every pre-existing
        // caller still passes a constant and is unchanged in meaning.
        let mut record_narrow_failure =
            |builder: &mut FunctionBuilder<'_>,
             invalid,
             tag: cranelift_codegen::ir::Value,
             detail: cranelift_codegen::ir::Value| {
                narrow_failure = Some(match narrow_failure.take() {
                    Some((prior_invalid, prior_tag, prior_detail)) => (
                        builder.ins().bor(prior_invalid, invalid),
                        builder.ins().select(prior_invalid, prior_tag, tag),
                        builder.ins().select(prior_invalid, prior_detail, detail),
                    ),
                    None => (invalid, tag, detail),
                });
            };
        match operation {
            ken_host::HostOpV1::ConsoleWrite
            | ken_host::HostOpV1::ConsoleFlush
            | ken_host::HostOpV1::ConsoleIsTerminal => {
                if capability.is_some() {
                    return Err(unsupported(
                        "Effect",
                        "ambient Console carried a capability",
                    ));
                }
                let stream = self.wire_constructor_tag_seat(
                    builder,
                    &seats,
                    SEAT_0,
                    console_stream_tag,
                    "Console operation has a malformed Stream operand",
                )?;
                builder
                    .ins()
                    .stack_store(stream, request, request_offset(0));
                if operation == ken_host::HostOpV1::ConsoleWrite {
                    let span = self.wire_bytes_seat(builder, &seats, SEAT_1)?;
                    if let Some((invalid, resource_code)) = span.refusal {
                        let detail = io_error_other_detail(builder, resource_code);
                        record_narrow_failure(builder, invalid, error_reply_tag, detail);
                    }
                    builder
                        .ins()
                        .stack_store(span.pointer, request, request_offset(1));
                    builder
                        .ins()
                        .stack_store(span.len, request, request_offset(2));
                }
            }
            ken_host::HostOpV1::FsReadFile
            | ken_host::HostOpV1::FsWriteFile
            | ken_host::HostOpV1::FsChangeMode
            | ken_host::HostOpV1::FsOpen => {
                // Lowered and claimed above, with every other operand, so the
                // claim group's window contains no operand lowering.
                let (capability_record, capability_operand) =
                    seats.operand(EffectSeatSlot::Capability)?;
                // ⛔ Exhaustive over both phases with no wildcard. The capability
                // is an either-phase seat: a specialized `CapabilityToken`
                // template is read directly, a carried word through the emitted
                // scalar read. A specialized template that is neither is the
                // third case, and it names the seat rather than falling into a
                // catch-all.
                let token = match capability_operand {
                    LoweringOperand::Specialized(Lowered::CapabilityToken { value }) => *value,
                    LoweringOperand::Carried(word) => self.emit_carrier_scalar(builder, *word)?,
                    LoweringOperand::Specialized(other) => {
                        return Err(unsupported(
                            "Effect",
                            format!(
                                "seat {:?} of {:?} needs {:?}, but the specialized template at \
                                 it is a {} rather than the opaque invocation token",
                                capability_record.slot,
                                capability_record.operation,
                                capability_record.need,
                                lowered_value_kind(other)
                            ),
                        ));
                    }
                };
                builder.ins().stack_store(token, request, request_offset(0));
                let path = self.wire_bytes_seat(builder, &seats, SEAT_0)?;
                if let Some((invalid, resource_code)) = path.refusal {
                    let detail = io_error_other_detail(builder, resource_code);
                    record_narrow_failure(builder, invalid, error_reply_tag, detail);
                }
                builder
                    .ins()
                    .stack_store(path.pointer, request, request_offset(1));
                builder
                    .ins()
                    .stack_store(path.len, request, request_offset(2));
                if operation == ken_host::HostOpV1::FsWriteFile {
                    let policy = self.wire_constructor_tag_seat(
                        builder,
                        &seats,
                        SEAT_1,
                        create_policy_tag,
                        "FS.WriteFile has a malformed CreatePolicy",
                    )?;
                    let contents = self.wire_bytes_seat(builder, &seats, SEAT_2)?;
                    if let Some((invalid, resource_code)) = contents.refusal {
                        let detail = io_error_other_detail(builder, resource_code);
                        record_narrow_failure(builder, invalid, error_reply_tag, detail);
                    }
                    builder
                        .ins()
                        .stack_store(policy, request, request_offset(3));
                    builder
                        .ins()
                        .stack_store(contents.pointer, request, request_offset(4));
                    builder
                        .ins()
                        .stack_store(contents.len, request, request_offset(5));
                } else if operation == ken_host::HostOpV1::FsChangeMode {
                    let mode = seats.specialized(SEAT_1)?;
                    let (mode, valid_int) = self.narrow_native_int_u64(builder, mode)?;
                    let in_range = builder.ins().icmp_imm(
                        cranelift_codegen::ir::condcodes::IntCC::UnsignedLessThanOrEqual,
                        mode,
                        0o7777,
                    );
                    let in_range = builder.ins().band(valid_int, in_range);
                    let narrowed = builder.ins().ireduce(types::I16, mode);
                    let invalid = builder.ins().iconst(types::I16, 0xffff);
                    let mode = builder.ins().select(in_range, narrowed, invalid);
                    builder.ins().stack_store(mode, request, request_offset(3));
                } else if operation == ken_host::HostOpV1::FsOpen {
                    let mode = self.wire_constructor_tag_seat(
                        builder,
                        &seats,
                        SEAT_1,
                        resource_open_mode_tag,
                        "FS.Open has a malformed ResourceOpenMode",
                    )?;
                    builder.ins().stack_store(mode, request, request_offset(3));
                }
            }
            ken_host::HostOpV1::FsHandleMetadata | ken_host::HostOpV1::ResourceRelease => {
                if capability.is_some() {
                    return Err(unsupported(
                        "Effect",
                        "resource operation carried a capability",
                    ));
                }
                // **`RT-RESOURCE-RELEASE-CARRIED-OBSERVE` `D1` -- the carried
                // route's OBSERVATION, and it is where the route's admission is
                // PAID FOR.**
                //
                // `claim_host_effect_seat` admitted this seat through
                // `EffectSeatClaimRoute::CarriedResourceObservation`, which
                // bypasses `avail` by construction. That is sound only because
                // the accept path re-runs a fail-closed consumer, and this is
                // it: `lower_resource_token_seat`'s carried arm requires
                // `BoundaryTag::InvocationBorrowed` AND
                // `BoundaryClass::BorrowedOpaque` BEFORE reading the scalar, so
                // a carried word not proven a resource token REFUSES rather
                // than yielding a garbage read.
                //
                // ⇒ The route does not weaken the gate; it moves the proof from
                // the phase table to a guarded observation, and the observation
                // is the one `BufferFreeze` already trusts and ships.
                //
                // `FsHandleMetadata` shares this arm and, since the key was
                // widened to the (need, phase) pair, is covered by the same
                // route -- its `Argument(0)` is a `ResourceScalar` seat too, and
                // the guards it passes are the same ones, for the same reason:
                // the tag/class a resource token carries is a function of its
                // `LoweredVariant`, not of the operation that reads it.
                let token = self.lower_resource_token_seat(
                    builder,
                    seats.operand(SEAT_0)?.1,
                    "resource operation",
                    "operand",
                )?;
                builder
                    .ins()
                    .stack_store(token, request, request_offset(0));
            }
            ken_host::HostOpV1::BufferAllocate => {
                if capability.is_some() {
                    return Err(unsupported(
                        "Effect",
                        "buffer allocation carried a capability",
                    ));
                }
                // ⭐ **The one seat whose `Avail` admits BOTH phases and now
                // has a route for each.** The claim is consumed here and the
                // observed phase selects the decoder, so the dispatch is bound
                // to the exact operand this arm reads rather than to a
                // conversion performed on its behalf.
                //
                // ⛔ Exhaustive, no wildcard. Both arms return `(u64, valid)`
                // and feed the SAME `InvalidBounds` lane below, so a capacity
                // that does not fit is one outcome regardless of how it
                // arrived — which is the point: the phase is a fact about how
                // the value reached the seat, never about what the program
                // means.
                // ⛔ The mutation deletes the CARRIED arm, leaving the
                // specialized read the whole route -- the exact state that
                // produced the `264 -> 262 / position 1` refusal the frame
                // names.
                #[cfg(test)]
                let carried_arm_removed = effect_seat_dispatch_mutation()
                    == EffectSeatDispatchMutation::RemoveCarriedCapacityArm;
                #[cfg(not(test))]
                let carried_arm_removed = false;
                let (_, capacity_operand) = seats.operand(SEAT_0)?;
                let (capacity, valid) = if carried_arm_removed {
                    let capacity = seats.specialized(SEAT_0)?.clone();
                    Self::record_capacity_phase_dispatch(false);
                    self.narrow_native_int_u64(builder, &capacity)?
                } else {
                    match capacity_operand {
                        LoweringOperand::Specialized(lowered) => {
                            let lowered = lowered.clone();
                            Self::record_capacity_phase_dispatch(false);
                            self.narrow_native_int_u64(builder, &lowered)?
                        }
                        LoweringOperand::Carried(word) => {
                            let word = *word;
                            Self::record_capacity_phase_dispatch(true);
                            self.narrow_carried_int_u64(builder, word)?
                        }
                    }
                };
                let invalid = builder.ins().icmp_imm(
                    cranelift_codegen::ir::condcodes::IntCC::Equal,
                    valid,
                    0,
                );
                let detail = builder
                    .ins()
                    .iconst(types::I64, RESOURCE_ERROR_INVALID_BOUNDS);
                record_narrow_failure(builder, invalid, resource_error_reply_tag, detail);
                builder
                    .ins()
                    .stack_store(capacity, request, request_offset(0));
            }
            ken_host::HostOpV1::BufferFreeze => {
                if capability.is_some() {
                    return Err(unsupported("Effect", "BufferFreeze carried a capability"));
                }
                let token = self.lower_resource_token_seat(
                    builder,
                    seats.operand(SEAT_0)?.1,
                    "BufferFreeze",
                    "buffer",
                )?;
                let start = seats.specialized(SEAT_1)?;
                let length = seats.specialized(SEAT_2)?;
                let (start, start_valid) = self.narrow_native_int_u64(builder, start)?;
                let (length, length_valid) = self.narrow_native_int_u64(builder, length)?;
                let valid = builder.ins().band(start_valid, length_valid);
                let invalid = builder.ins().icmp_imm(
                    cranelift_codegen::ir::condcodes::IntCC::Equal,
                    valid,
                    0,
                );
                let detail = builder
                    .ins()
                    .iconst(types::I64, RESOURCE_ERROR_INVALID_BOUNDS);
                record_narrow_failure(builder, invalid, resource_error_reply_tag, detail);
                // PX8-SPAN-PROV: trailing `span_origin` acquisition token.
                let span_origin = self.lower_resource_token_seat(
                    builder,
                    seats.operand(SEAT_3)?.1,
                    "BufferFreeze",
                    "span origin",
                )?;
                for (index, value) in [token, start, length, span_origin].into_iter().enumerate() {
                    builder
                        .ins()
                        .stack_store(value, request, request_offset(index));
                }
            }
            ken_host::HostOpV1::FsReadAt | ken_host::HostOpV1::FsWriteAt => {
                if capability.is_some() {
                    return Err(unsupported(
                        "Effect",
                        "positioned resource operation carried a capability",
                    ));
                }
                // `RT-RESOURCE-RELEASE-CARRIED-OBSERVE` `D1`: these resource
                // seats read through the SHARED guarded observation, because the
                // widened (need, phase) key can now claim them in the carried
                // phase. A specialized-only read here would leave the claim
                // admitted and the read refusing -- the route would have moved
                // the refusal rather than closed it.
                let file = self.lower_resource_token_seat(
                    builder,
                    seats.operand(EffectSeatSlot::Argument(0))?.1,
                    "positioned resource operation",
                    "file",
                )?;
                // `RT-EXACTINT-CARRIED-OBSERVE` `D1` -- these seats decode in
                // EITHER phase, paired ATOMICALLY with the `carried_exact_int`
                // move in the seat table. A widened `Avail` without its reader
                // is the claim-admitted-read-refuses shape: the seat passes the
                // gate and dies at the read, which moves a refusal instead of
                // closing one.
                //
                // `narrow_native_int_u64` and `narrow_carried_int_u64` are the
                // ONLY two decoders here, one per phase, both returning
                // `(value, valid)` into the operation's existing
                // narrow-failure lane below. No second carried `Int` decode is
                // spelled.
                let (file_offset, file_offset_valid) =
                    self.narrow_positioned_int_seat(builder, &seats, 1, "file offset")?;
                let buffer = self.lower_resource_token_seat(
                    builder,
                    seats.operand(EffectSeatSlot::Argument(2))?.1,
                    "positioned resource operation",
                    "buffer",
                )?;
                let (buffer_start, buffer_start_valid) =
                    self.narrow_positioned_int_seat(builder, &seats, 3, "buffer start")?;
                let (length, length_valid) =
                    self.narrow_positioned_int_seat(builder, &seats, 4, "length")?;
                positioned_bounds = Some((buffer_start, length));
                let file_offset_invalid = builder.ins().icmp_imm(
                    cranelift_codegen::ir::condcodes::IntCC::Equal,
                    file_offset_valid,
                    0,
                );
                let detail = builder
                    .ins()
                    .iconst(types::I64, RESOURCE_ERROR_INVALID_OFFSET);
                record_narrow_failure(
                    builder,
                    file_offset_invalid,
                    resource_error_reply_tag,
                    detail,
                );
                let bounds_valid = builder.ins().band(buffer_start_valid, length_valid);
                let bounds_invalid = builder.ins().icmp_imm(
                    cranelift_codegen::ir::condcodes::IntCC::Equal,
                    bounds_valid,
                    0,
                );
                let detail = builder
                    .ins()
                    .iconst(types::I64, RESOURCE_ERROR_INVALID_BOUNDS);
                record_narrow_failure(builder, bounds_invalid, resource_error_reply_tag, detail);
                if operation == ken_host::HostOpV1::FsWriteAt {
                    // PX8-SPAN-PROV: `FsWriteAt` carries the trailing
                    // `span_origin` acquisition token; `FsReadAt` mints the span
                    // and has no origin operand.
                    let span_origin = self.lower_resource_token_seat(
                        builder,
                        seats.operand(EffectSeatSlot::Argument(5))?.1,
                        "positioned resource operation",
                        "span origin",
                    )?;
                    for (index, value) in
                        [file, buffer, file_offset, buffer_start, length, span_origin]
                            .into_iter()
                            .enumerate()
                    {
                        builder
                            .ins()
                            .stack_store(value, request, request_offset(index));
                    }
                } else {
                    for (index, value) in [file, buffer, file_offset, buffer_start, length]
                        .into_iter()
                        .enumerate()
                    {
                        builder
                            .ins()
                            .stack_store(value, request, request_offset(index));
                    }
                }
            }
            _ => unreachable!("availability was checked above"),
        }
        let reply = builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            wire.reply_size,
            wire.reply_align_shift,
        ));
        let host_context = self
            .function_local
            .host_dispatch_context
            .expect("process effect lowering owns a direct host context");
        let op = builder.ins().iconst(types::I64, operation as i64);
        let request_pointer = builder.ins().stack_addr(pointer_type, request, 0);
        let request_size = builder
            .ins()
            .iconst(types::I64, i64::from(wire.request_size));
        let reply_pointer = builder.ins().stack_addr(pointer_type, reply, 0);
        if let Some((invalid, failure_tag, detail)) = narrow_failure {
            let dispatch = builder.create_block();
            let synthesize = builder.create_block();
            let decoded = builder.create_block();
            builder.ins().brif(invalid, synthesize, &[], dispatch, &[]);

            builder.switch_to_block(dispatch);
            let call = builder.ins().call(
                self.function_local
                    .host_dispatch
                    .expect("process effect lowering owns one host dispatch import"),
                &[
                    host_context,
                    op,
                    request_pointer,
                    request_size,
                    reply_pointer,
                ],
            );
            let status = builder.inst_results(call)[0];
            Self::require_i64(builder, status, 0);
            builder.ins().jump(decoded, &[]);

            builder.switch_to_block(synthesize);
            let zero = builder.ins().iconst(types::I64, 0);
            for offset in [
                wire.reply_resource_error_schema_offset,
                wire.reply_resource_error_kind_offset,
                wire.reply_resource_error_identity_offset,
                wire.reply_resource_error_io_offset,
                wire.reply_resource_error_required_offset,
                wire.reply_resource_error_held_offset,
                wire.reply_resource_error_expected_kind_offset,
                wire.reply_resource_error_actual_kind_offset,
                wire.reply_bytes_data_offset,
                wire.reply_bytes_len_offset,
                wire.reply_effective_request_offset,
            ] {
                builder.ins().stack_store(
                    zero,
                    reply,
                    i32::try_from(offset).expect("reply field offset is u32"),
                );
            }
            // The tag comes from whoever recorded the failure, because only they
            // know which surface this operation declares. Hardcoding the
            // resource-error tag here is what made a byte-span refusal on
            // `ConsoleWrite` / `FsWriteFile` fail `require_one_of_i64` below and
            // collapse into the generic compiled-function failure instead of
            // reaching Ken as a value.
            builder.ins().stack_store(
                failure_tag,
                reply,
                i32::try_from(wire.reply_tag_offset).expect("reply tag offset is u32"),
            );
            builder.ins().stack_store(
                detail,
                reply,
                i32::try_from(wire.reply_detail_offset).expect("reply detail offset is u32"),
            );
            builder.ins().jump(decoded, &[]);
            builder.switch_to_block(decoded);
        } else {
            let call = builder.ins().call(
                self.function_local
                    .host_dispatch
                    .expect("process effect lowering owns one host dispatch import"),
                &[
                    host_context,
                    op,
                    request_pointer,
                    request_size,
                    reply_pointer,
                ],
            );
            let status = builder.inst_results(call)[0];
            Self::require_i64(builder, status, 0);
        }
        let tag = builder.ins().stack_load(
            types::I64,
            reply,
            i32::try_from(wire.reply_tag_offset).expect("reply tag offset is u32"),
        );
        let detail = builder.ins().stack_load(
            types::I64,
            reply,
            i32::try_from(wire.reply_detail_offset).expect("reply detail offset is u32"),
        );
        if operation == ken_host::HostOpV1::ConsoleIsTerminal {
            Self::require_i64(builder, tag, wire.reply_bool_tag as i64);
            Ok(LoweringOperand::Specialized(Lowered::Bool {
                value: detail,
                known: None,
            }))
        } else {
            let success_tag = match operation {
                ken_host::HostOpV1::FsReadFile => wire.reply_bytes_tag,
                ken_host::HostOpV1::FsOpen => wire.reply_resource_tag,
                ken_host::HostOpV1::FsHandleMetadata => wire.reply_metadata_tag,
                ken_host::HostOpV1::BufferAllocate => wire.reply_resource_tag,
                ken_host::HostOpV1::BufferFreeze => wire.reply_bytes_tag,
                ken_host::HostOpV1::FsReadAt => wire.reply_read_progress_tag,
                ken_host::HostOpV1::FsWriteAt => wire.reply_write_progress_tag,
                _ => wire.reply_unit_tag,
            } as i64;
            let accepted_tags = match operation {
                ken_host::HostOpV1::FsHandleMetadata => vec![
                    success_tag,
                    wire.reply_error_tag as i64,
                    wire.reply_resource_error_tag as i64,
                ],
                ken_host::HostOpV1::ResourceRelease => {
                    vec![success_tag, wire.reply_resource_error_tag as i64]
                }
                ken_host::HostOpV1::BufferAllocate | ken_host::HostOpV1::BufferFreeze => {
                    vec![success_tag, wire.reply_resource_error_tag as i64]
                }
                ken_host::HostOpV1::FsReadAt | ken_host::HostOpV1::FsWriteAt => vec![
                    success_tag,
                    wire.reply_error_tag as i64,
                    wire.reply_resource_error_tag as i64,
                ],
                _ => vec![success_tag, wire.reply_error_tag as i64],
            };
            Self::require_one_of_i64(builder, tag, &accepted_tags);
            let resource_schema = builder.ins().stack_load(
                types::I64,
                reply,
                i32::try_from(wire.reply_resource_error_schema_offset)
                    .expect("resource error schema offset is u32"),
            );
            let resource_kind = builder.ins().stack_load(
                types::I64,
                reply,
                i32::try_from(wire.reply_resource_error_kind_offset)
                    .expect("resource error kind offset is u32"),
            );
            let resource_identity = builder.ins().stack_load(
                types::I64,
                reply,
                i32::try_from(wire.reply_resource_error_identity_offset)
                    .expect("resource error identity offset is u32"),
            );
            let resource_io = builder.ins().stack_load(
                types::I64,
                reply,
                i32::try_from(wire.reply_resource_error_io_offset)
                    .expect("resource error io offset is u32"),
            );
            let resource_required = builder.ins().stack_load(
                types::I64,
                reply,
                i32::try_from(wire.reply_resource_error_required_offset)
                    .expect("resource error required offset is u32"),
            );
            let resource_held = builder.ins().stack_load(
                types::I64,
                reply,
                i32::try_from(wire.reply_resource_error_held_offset)
                    .expect("resource error held offset is u32"),
            );
            let resource_expected_kind = builder.ins().stack_load(
                types::I64,
                reply,
                i32::try_from(wire.reply_resource_error_expected_kind_offset)
                    .expect("resource error expected-kind offset is u32"),
            );
            let resource_actual_kind = builder.ins().stack_load(
                types::I64,
                reply,
                i32::try_from(wire.reply_resource_error_actual_kind_offset)
                    .expect("resource error actual-kind offset is u32"),
            );
            Self::validate_resource_error_reply(
                builder,
                tag,
                wire.reply_resource_error_tag,
                detail,
                resource_schema,
                resource_kind,
                resource_identity,
                resource_io,
                resource_required,
                resource_held,
                resource_expected_kind,
                resource_actual_kind,
                [
                    wire.resource_error_closed,
                    wire.resource_error_malformed,
                    wire.resource_error_right_not_held,
                    wire.resource_error_release_failed,
                    wire.resource_error_kind_mismatch,
                    wire.resource_error_buffer_limit,
                    wire.resource_error_invalid_offset,
                    wire.resource_error_invalid_bounds,
                    wire.resource_error_no_progress,
                    wire.resource_error_allocation_failed,
                ],
                wire.resource_error_reply_schema,
                wire.resource_kind_fs_handle,
                wire.resource_kind_buffer,
            );
            let payload = builder.ins().sshr_imm(detail, 32);
            let payload_int = self.lower_dynamic_small_int(builder, payload);
            // `D7` — the two roots of this operation's synthesized aggregate
            // trees. Every synthesized producer below states its own path from
            // one of them explicitly; nothing is rebound as the walk descends,
            // so a path in this file names exactly one node in the planner's
            // tree and the two can be compared.
            // ⭐ **The claim-backed view itself is what reply synthesis
            // carries** -- positional semantics, projected LAZILY.
            //
            // ⛔ A dense `Vec<Lowered>` used to be realized here by demanding a
            // specialized template for every argument the operation has. That
            // was the removed pre-operation bulk conversion RELOCATED after
            // dispatch: knowing the operation narrowed the diagnostic but did
            // not authorize reading a seat no synthesized node uses. Only two
            // site-bound children exist in the measured trees -- the file-error
            // path's `SiteOperand(0)` and `FsReadAt`'s private-buffer-span
            // `SiteOperand(2)` -- so an operation with no `SiteOperand` child
            // must project no template at all. `BufferAllocate`'s carried
            // capacity is the case that made this load-bearing: its own arm
            // consumes the seat, and the dense vector then refused it again on
            // behalf of a consumer that never wanted it.
            //
            // ⚠ The `BufferFreeze` special case is gone rather than moved. It
            // existed only to keep the dense realization off an operation whose
            // seats are not templates; with the projection driven by declared
            // uses, an operation with no site-bound child asks for nothing
            // without needing to be named.
            //
            // ⛔ The mutation puts the dense realization back, exactly as it
            // stood: every argument demanded as a template, before any declared
            // use has said which ones it wants.
            #[cfg(test)]
            if effect_seat_dispatch_mutation() == EffectSeatDispatchMutation::RestoreBulkConversion
                && operation != ken_host::HostOpV1::BufferFreeze
            {
                for ordinal in 0..lowered.len() as u32 {
                    let _ = seats.specialized(EffectSeatSlot::Argument(ordinal))?;
                }
            }
            let error_root =
                SynthesizedAggregatePath::root(SynthesizedAggregateRoot::HostResultError);
            let ok_root = SynthesizedAggregatePath::root(SynthesizedAggregateRoot::HostResultOk);
            // ⭐ `D7` — the generic `IOError` value is built **per branch, at
            // the path that branch puts it at**, not once up front.
            //
            // ⛔ It used to be constructed eagerly, before the operation match
            // knew which arm it was in. The six resource-surface operations
            // build their own `surface_io_error` and never referenced it, so
            // that template was ABANDONED — an allocation-shaped value with no
            // semantic use. Now it is never built for them at all, which is
            // strictly better than planning it and proving it unreachable: it
            // contributes neither a record nor an event because it does not
            // exist.
            let generic_io_error =
                |this: &Self,
                 builder: &mut FunctionBuilder<'_>,
                 payload: Lowered,
                 node: &SynthesizedAggregatePath| {
                    Ok::<_, CraneliftBackendError>(Lowered::DynamicConstructor(
                        DynamicConstructorV1 {
                            discriminator: builder.ins().band_imm(detail, 0xff),
                            alternatives: this.synthesized_io_error_alternatives(
                                static_origin,
                                node,
                                payload,
                                &seats,
                            )?,
                        },
                    ))
                };
            let error = if matches!(
                operation,
                ken_host::HostOpV1::FsReadFile
                    | ken_host::HostOpV1::FsWriteFile
                    | ken_host::HostOpV1::FsChangeMode
                    | ken_host::HostOpV1::FsOpen
            ) {
                // ⛔ No eager read of the path seat here. It used to be
                // demanded as a template and then dropped -- the same
                // unauthorized eager demand at a smaller scale. The one thing
                // that legitimately needs it is the `OptionSome` child below,
                // which projects it through `site_operand_argument` at its
                // declared `SiteOperand(0)`.
                let (operation_role, operation_symbol) = match operation {
                    ken_host::HostOpV1::FsReadFile | ken_host::HostOpV1::FsOpen => (
                        SynthesizedFixedConstructorRole::FileOperationRead,
                        self.process_symbols.file_operation_read.clone(),
                    ),
                    ken_host::HostOpV1::FsWriteFile => (
                        SynthesizedFixedConstructorRole::FileOperationWrite,
                        self.process_symbols.file_operation_write.clone(),
                    ),
                    ken_host::HostOpV1::FsChangeMode => (
                        SynthesizedFixedConstructorRole::FileOperationChangeMode,
                        self.process_symbols.file_operation_change_mode.clone(),
                    ),
                    _ => unreachable!("validated FS result operation"),
                };
                let operation = self.synthesized_constructor(
                    static_origin,
                    &error_root.field(0),
                    operation_role,
                    operation_symbol,
                    Vec::new(),
                    &seats,
                )?;
                let path_argument =
                    self.site_operand_argument(builder, static_origin, 0, &seats)?;
                let path = self.synthesized_constructor(
                    static_origin,
                    &error_root.field(1),
                    SynthesizedFixedConstructorRole::OptionSome,
                    self.process_symbols.option_some.clone(),
                    // The seat's operand 0 — projected, not passed.
                    vec![path_argument],
                    &seats,
                )?;
                let io_error = generic_io_error(self, builder, payload_int, &error_root.field(2))?;
                self.synthesized_constructor(
                    static_origin,
                    &error_root,
                    SynthesizedFixedConstructorRole::FileError,
                    self.process_symbols.file_error.clone(),
                    vec![
                        SynthesizedArgument::Nested(operation),
                        SynthesizedArgument::Nested(path),
                        SynthesizedArgument::Dynamic(io_error),
                    ],
                    &seats,
                )?
            } else if matches!(
                operation,
                ken_host::HostOpV1::FsHandleMetadata
                    | ken_host::HostOpV1::ResourceRelease
                    | ken_host::HostOpV1::BufferAllocate
                    | ken_host::HostOpV1::BufferFreeze
                    | ken_host::HostOpV1::FsReadAt
                    | ken_host::HostOpV1::FsWriteAt
            ) {
                let generic = builder.ins().icmp_imm(
                    cranelift_codegen::ir::condcodes::IntCC::Equal,
                    tag,
                    wire.reply_error_tag as i64,
                );
                let zero = builder.ins().iconst(types::I64, 0);
                let resource_surface_tag = builder.ins().iadd_imm(detail, 1);
                let surface_tag = builder.ins().select(generic, zero, resource_surface_tag);
                let surface_io = builder.ins().select(generic, detail, resource_io);
                let surface_io_payload = builder.ins().sshr_imm(surface_io, 32);
                let surface_io_payload_int =
                    self.lower_dynamic_small_int(builder, surface_io_payload);
                let resource_required_int =
                    self.lower_unsigned_u64_int(builder, resource_required)?;
                let resource_held_int = self.lower_unsigned_u64_int(builder, resource_held)?;
                // ⭐ Built ONCE PER SEMANTIC USE. `ResourceHostIo` field 0 and
                // `ResourceReleaseFailed` field 2 are two allocations at two
                // paths; cloning one template into both would carry one
                // occurrence to two allocations, which is exactly the aliasing
                // the path key exists to prevent.
                let surface_io_error =
                    |this: &Self,
                     builder: &mut FunctionBuilder<'_>,
                     node: &SynthesizedAggregatePath| {
                        Ok::<_, CraneliftBackendError>(Lowered::DynamicConstructor(
                            DynamicConstructorV1 {
                                discriminator: builder.ins().band_imm(surface_io, 0xff),
                                alternatives: this.synthesized_io_error_alternatives(
                                    static_origin,
                                    node,
                                    surface_io_payload_int.clone(),
                                    &seats,
                                )?,
                            },
                        ))
                    };
                let identity_low = builder.ins().band_imm(resource_identity, 0xffff_ffff);
                let identity_high = builder.ins().ushr_imm(resource_identity, 32);
                let identity_low_int = self.lower_dynamic_small_int(builder, identity_low);
                let identity_high_int = self.lower_dynamic_small_int(builder, identity_high);
                // ⭐ `ResourceKind` is built THREE times at this one seat, so
                // the closure takes the path of the node it is building rather
                // than closing over one. Passing a path in is what makes the
                // three uses three occurrences; a closure that knew its own
                // path could only ever describe one of them.
                let resource_kind_value =
                    |this: &Self, discriminator, node: &SynthesizedAggregatePath| {
                        Ok::<_, CraneliftBackendError>(Lowered::DynamicConstructor(
                            DynamicConstructorV1 {
                                discriminator,
                                alternatives: vec![
                                    this.synthesized_dynamic_alternative(
                                        static_origin,
                                        node,
                                        0,
                                        wire.resource_kind_fs_handle as i64,
                                        SynthesizedFixedConstructorRole::ResourceKindFsHandle,
                                        this.process_symbols.resource_kind_fs_handle.clone(),
                                        Vec::new(),
                                        &seats,
                                    )?,
                                    this.synthesized_dynamic_alternative(
                                        static_origin,
                                        node,
                                        1,
                                        wire.resource_kind_buffer as i64,
                                        SynthesizedFixedConstructorRole::ResourceKindBuffer,
                                        this.process_symbols.resource_kind_buffer.clone(),
                                        Vec::new(),
                                        &seats,
                                    )?,
                                ],
                            },
                        ))
                    };
                // `ResourceReleaseFailed` field 1, alternative 4 of the
                // resource surface -- measured, not assumed.
                let checked_resource_tag = |wire_tag: u64| {
                    i64::try_from(wire_tag).expect("resource error tag fits i64") + 1
                };
                let trace_identity = self.synthesized_constructor(
                    static_origin,
                    &error_root.alternative(4).field(1),
                    SynthesizedFixedConstructorRole::ResourceTraceIdentity,
                    self.process_symbols.resource_trace_identity.clone(),
                    vec![
                        SynthesizedArgument::Scalar(identity_low_int),
                        SynthesizedArgument::Scalar(identity_high_int),
                    ],
                    &seats,
                )?;
                Lowered::DynamicConstructor(DynamicConstructorV1 {
                    discriminator: surface_tag,
                    alternatives: vec![
                        self.synthesized_dynamic_alternative(
                            static_origin,
                            &error_root,
                            0,
                            0,
                            SynthesizedFixedConstructorRole::ResourceHostIo,
                            self.process_symbols.resource_host_io.clone(),
                            vec![SynthesizedArgument::Dynamic(surface_io_error(
                                self,
                                builder,
                                &error_root.alternative(0).field(0),
                            )?)],
                            &seats,
                        )?,
                        self.synthesized_dynamic_alternative(
                            static_origin,
                            &error_root,
                            1,
                            checked_resource_tag(wire.resource_error_closed),
                            SynthesizedFixedConstructorRole::ResourceClosed,
                            self.process_symbols.resource_closed.clone(),
                            Vec::new(),
                            &seats,
                        )?,
                        self.synthesized_dynamic_alternative(
                            static_origin,
                            &error_root,
                            2,
                            checked_resource_tag(wire.resource_error_malformed),
                            SynthesizedFixedConstructorRole::ResourceMalformed,
                            self.process_symbols.resource_malformed.clone(),
                            Vec::new(),
                            &seats,
                        )?,
                        self.synthesized_dynamic_alternative(
                            static_origin,
                            &error_root,
                            3,
                            checked_resource_tag(wire.resource_error_right_not_held),
                            SynthesizedFixedConstructorRole::ResourceRightNotHeld,
                            self.process_symbols.resource_right_not_held.clone(),
                            vec![
                                SynthesizedArgument::Scalar(resource_required_int),
                                SynthesizedArgument::Scalar(resource_held_int),
                            ],
                            &seats,
                        )?,
                        self.synthesized_dynamic_alternative(
                            static_origin,
                            &error_root,
                            4,
                            checked_resource_tag(wire.resource_error_release_failed),
                            SynthesizedFixedConstructorRole::ResourceReleaseFailed,
                            self.process_symbols.resource_release_failed.clone(),
                            vec![
                                SynthesizedArgument::Dynamic(resource_kind_value(
                                    self,
                                    resource_kind,
                                    &error_root.alternative(4).field(0),
                                )?),
                                SynthesizedArgument::Nested(trace_identity),
                                SynthesizedArgument::Dynamic(surface_io_error(
                                    self,
                                    builder,
                                    &error_root.alternative(4).field(2),
                                )?),
                            ],
                            &seats,
                        )?,
                        self.synthesized_dynamic_alternative(
                            static_origin,
                            &error_root,
                            5,
                            checked_resource_tag(wire.resource_error_kind_mismatch),
                            SynthesizedFixedConstructorRole::ResourceKindMismatch,
                            self.process_symbols.resource_kind_mismatch.clone(),
                            vec![
                                SynthesizedArgument::Dynamic(resource_kind_value(
                                    self,
                                    resource_expected_kind,
                                    &error_root.alternative(5).field(0),
                                )?),
                                SynthesizedArgument::Dynamic(resource_kind_value(
                                    self,
                                    resource_actual_kind,
                                    &error_root.alternative(5).field(1),
                                )?),
                            ],
                            &seats,
                        )?,
                        self.synthesized_dynamic_alternative(
                            static_origin,
                            &error_root,
                            6,
                            checked_resource_tag(wire.resource_error_buffer_limit),
                            SynthesizedFixedConstructorRole::ResourceBufferLimit,
                            self.process_symbols.resource_buffer_limit.clone(),
                            Vec::new(),
                            &seats,
                        )?,
                        self.synthesized_dynamic_alternative(
                            static_origin,
                            &error_root,
                            7,
                            checked_resource_tag(wire.resource_error_allocation_failed),
                            SynthesizedFixedConstructorRole::ResourceAllocationFailed,
                            self.process_symbols.resource_allocation_failed.clone(),
                            Vec::new(),
                            &seats,
                        )?,
                        self.synthesized_dynamic_alternative(
                            static_origin,
                            &error_root,
                            8,
                            checked_resource_tag(wire.resource_error_invalid_offset),
                            SynthesizedFixedConstructorRole::ResourceInvalidOffset,
                            self.process_symbols.resource_invalid_offset.clone(),
                            Vec::new(),
                            &seats,
                        )?,
                        self.synthesized_dynamic_alternative(
                            static_origin,
                            &error_root,
                            9,
                            checked_resource_tag(wire.resource_error_invalid_bounds),
                            SynthesizedFixedConstructorRole::ResourceInvalidBounds,
                            self.process_symbols.resource_invalid_bounds.clone(),
                            Vec::new(),
                            &seats,
                        )?,
                        self.synthesized_dynamic_alternative(
                            static_origin,
                            &error_root,
                            10,
                            checked_resource_tag(wire.resource_error_no_progress),
                            SynthesizedFixedConstructorRole::ResourceNoProgress,
                            self.process_symbols.resource_no_progress.clone(),
                            Vec::new(),
                            &seats,
                        )?,
                    ],
                })
            } else {
                generic_io_error(self, builder, payload_int, &error_root)?
            };
            let success = builder.ins().icmp_imm(
                cranelift_codegen::ir::condcodes::IntCC::Equal,
                tag,
                success_tag,
            );
            let ok = if operation == ken_host::HostOpV1::FsReadFile {
                // `D2` — self-validating span; see `masked_reply_response_bytes`.
                masked_reply_response_bytes(
                    builder,
                    pointer_type,
                    reply,
                    wire.reply_bytes_data_offset,
                    wire.reply_bytes_len_offset,
                    success,
                )
            } else if operation == ken_host::HostOpV1::FsOpen {
                Lowered::ResourceToken { value: detail }
            } else if operation == ken_host::HostOpV1::BufferAllocate {
                Lowered::ResourceToken { value: detail }
            } else if operation == ken_host::HostOpV1::BufferFreeze {
                // `D2` — the SECOND site, and it needs the mask for the same
                // reason. ⛔ Not a copy of the arm above by accident: both
                // construct a `ResponseBytes` from the reply span, so a mask at
                // only one of them leaves the other dereferencing an
                // unestablished pointer on its failure path.
                masked_reply_response_bytes(
                    builder,
                    pointer_type,
                    reply,
                    wire.reply_bytes_data_offset,
                    wire.reply_bytes_len_offset,
                    success,
                )
            } else if operation == ken_host::HostOpV1::FsReadAt {
                let reply_data = builder.ins().stack_load(
                    pointer_type,
                    reply,
                    i32::try_from(wire.reply_bytes_data_offset)
                        .expect("reply bytes data offset is u32"),
                );
                let reply_start = builder.ins().stack_load(
                    types::I64,
                    reply,
                    i32::try_from(wire.reply_bytes_len_offset)
                        .expect("reply bytes len offset is u32"),
                );
                let nonzero = builder.ins().icmp_imm(
                    cranelift_codegen::ir::condcodes::IntCC::NotEqual,
                    detail,
                    0,
                );
                let read_some = builder.ins().band(success, nonzero);
                let zero = builder.ins().iconst(types::I64, 0);
                let eof_data = builder.ins().icmp(
                    cranelift_codegen::ir::condcodes::IntCC::Equal,
                    reply_data,
                    zero,
                );
                let eof_start = builder.ins().icmp_imm(
                    cranelift_codegen::ir::condcodes::IntCC::Equal,
                    reply_start,
                    0,
                );
                let eof_valid = builder.ins().band(eof_data, eof_start);
                let is_zero = builder.ins().bnot(nonzero);
                let read_eof = builder.ins().band(success, is_zero);
                Self::require_when(builder, read_eof, eof_valid);
                Self::require_when(builder, read_some, eof_data);
                let (request_start, request_length) = positioned_bounds
                    .expect("positioned request bounds were narrowed before dispatch");
                let effective_request = builder.ins().stack_load(
                    types::I64,
                    reply,
                    i32::try_from(wire.reply_effective_request_offset)
                        .expect("reply effective request offset is u32"),
                );
                let (count, predecessor, remaining) = Self::mint_validated_progress_nat(
                    builder,
                    read_some,
                    detail,
                    request_start,
                    request_length,
                    effective_request,
                    Some(reply_start),
                );
                let reply_start_int = self.lower_unsigned_u64_int(builder, reply_start)?;
                // PX8-SPAN-PROV: bind the minted span to this `readAt`'s buffer
                // operand acquisition (lowered arg 2, the request seat).
                // `RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL` `D1` -- the dead
                // reply-path gate is REMOVED here, and it is removed WITH the
                // projector fix rather than before it.
                //
                // It destructured a `span_origin` that nothing consumed (the
                // span is projected from the operand list below), so as a
                // BINDING it was vestigial. But its refusal was not: deleting
                // it alone only moved the refusal into the projector, which
                // could not handle a carried buffer either. Measured both ways
                // before this node widened. The two edits are one semantic unit;
                // splitting them ships a diff that greens nothing.
                let span_argument =
                    self.site_operand_argument(builder, static_origin, 2, &seats)?;
                let span = self.synthesized_constructor(
                    static_origin,
                    &ok_root.alternative(1).field(0),
                    SynthesizedFixedConstructorRole::PrivateBufferSpan,
                    self.process_symbols.private_buffer_span.clone(),
                    vec![
                        // The seat's operand 2 — the buffer this span is bound
                        // to (`PX8-SPAN-PROV`), projected from the operand list
                        // rather than rebuilt from its destructured payload.
                        span_argument,
                        SynthesizedArgument::Scalar(reply_start_int),
                        SynthesizedArgument::Scalar(Lowered::BoundedNat(count)),
                    ],
                    &seats,
                )?;
                let transferred = self.synthesized_constructor(
                    static_origin,
                    &ok_root.alternative(1).field(1),
                    SynthesizedFixedConstructorRole::PrivateTransferCount,
                    self.process_symbols.private_transfer_count.clone(),
                    vec![
                        SynthesizedArgument::Scalar(Lowered::BoundedNat(predecessor)),
                        SynthesizedArgument::Scalar(Lowered::BoundedNat(remaining)),
                    ],
                    &seats,
                )?;
                #[cfg(test)]
                let read_progress_discriminator = if effect_seat_dispatch_mutation()
                    == EffectSeatDispatchMutation::ForceReadProgressOutsideAlternatives
                {
                    builder.ins().iconst(types::I64, 2)
                } else {
                    builder.ins().uextend(types::I64, nonzero)
                };
                #[cfg(not(test))]
                let read_progress_discriminator =
                    builder.ins().uextend(types::I64, nonzero);
                Lowered::DynamicConstructor(DynamicConstructorV1 {
                    discriminator: read_progress_discriminator,
                    alternatives: vec![
                        self.synthesized_dynamic_alternative(
                            static_origin,
                            &ok_root,
                            0,
                            0,
                            SynthesizedFixedConstructorRole::ReadEof,
                            self.process_symbols.read_eof.clone(),
                            Vec::new(),
                            &seats,
                        )?,
                        self.synthesized_dynamic_alternative(
                            static_origin,
                            &ok_root,
                            1,
                            1,
                            SynthesizedFixedConstructorRole::ReadSome,
                            self.process_symbols.read_some.clone(),
                            vec![
                                SynthesizedArgument::Nested(span),
                                SynthesizedArgument::Nested(transferred),
                            ],
                            &seats,
                        )?,
                    ],
                })
            } else if operation == ken_host::HostOpV1::FsWriteAt {
                let (request_start, request_length) = positioned_bounds
                    .expect("positioned request bounds were narrowed before dispatch");
                let effective_request = builder.ins().stack_load(
                    types::I64,
                    reply,
                    i32::try_from(wire.reply_effective_request_offset)
                        .expect("reply effective request offset is u32"),
                );
                let (_count, predecessor, remaining) = Self::mint_validated_progress_nat(
                    builder,
                    success,
                    detail,
                    request_start,
                    request_length,
                    effective_request,
                    None,
                );
                let transferred = self.synthesized_constructor(
                    static_origin,
                    &ok_root.field(0),
                    SynthesizedFixedConstructorRole::PrivateTransferCount,
                    self.process_symbols.private_transfer_count.clone(),
                    vec![
                        SynthesizedArgument::Scalar(Lowered::BoundedNat(predecessor)),
                        SynthesizedArgument::Scalar(Lowered::BoundedNat(remaining)),
                    ],
                    &seats,
                )?;
                self.synthesized_constructor(
                    static_origin,
                    &ok_root,
                    SynthesizedFixedConstructorRole::Wrote,
                    self.process_symbols.wrote.clone(),
                    vec![SynthesizedArgument::Nested(transferred)],
                    &seats,
                )?
            } else if operation == ken_host::HostOpV1::FsHandleMetadata {
                self.lower_unsigned_u64_int(builder, detail)?
            } else {
                self.synthesized_constructor(
                    static_origin,
                    &ok_root,
                    SynthesizedFixedConstructorRole::Unit,
                    self.process_symbols.unit.clone(),
                    Vec::new(),
                    &seats,
                )?
            };
            // `D7` — the two ROOTS, which no node declares. Every other
            // synthesized allocation is reached through a parent's ordered
            // child model and reconciled on the way down; these two are
            // returned straight into the host result with nothing above them,
            // so the population equality is asked for here explicitly.
            self.reconcile_host_result_root(static_origin, &error_root, &error)?;
            self.reconcile_host_result_root(static_origin, &ok_root, &ok)?;
            Ok(LoweringOperand::Specialized(Lowered::HostResult {
                success,
                error: Box::new(error),
                ok: Box::new(ok),
                err_constructor: self.process_symbols.result_err.clone(),
                ok_constructor: self.process_symbols.result_ok.clone(),
            }))
        }
    }
}
