//! The aggregates emitter -- aggregate construction and projection
//! emission, allocation emission, and the governed-allocation surfaces.
//!
//! `RT-EMITTER-AGGREGATES-SPLIT` `D1`. Extends the `boundary.rs`/
//! `source.rs`/`calls.rs`/`joins.rs` seam (items 11-14): the owner traced
//! in this item's D0 ledger (`docs/program/issues/
//! RT-EMITTER-AGGREGATES-SPLIT.md`) relocates here from `mod.rs`, moved
//! verbatim from two separate impl-block clusters -- each moved method
//! sits in its own small `impl` block (or, for cluster 2, one `impl`
//! block since no RETAIN method was interleaved there), matching
//! `joins.rs`'s own shape. Every other type the moving methods merely
//! manipulate (`ScalarMergeKind`-adjacent planner types, `BoundaryCarrierRefs`,
//! `ClaimedEffectSeats`, the `emit_carrier_*` decode family, `carrier_out_slot`,
//! and siblings) stays declared at the `mod.rs` hub -- hub-stays/methods-move,
//! the same shape items 10/12/13/14 established.
//!
//! `AggregateAllocationLedger`/`AggregateAllocationEvent`/
//! `AggregateRelationClosure` were already `pub(in crate::cranelift_backend)`
//! before the move (the Architect's D0 carry) -- they move verbatim, zero
//! widening, matching the precedent `units.rs`'s own moved-ledger types
//! (`CheckedCallLedger`/`ContinuationCandidateLedger`/
//! `ContinuationClaimLedger`/`FusionCompositionLedger`) already set as
//! fields of the retained `Lowering` hub struct. `mod.rs`'s own
//! `aggregate_allocations: Option<AggregateAllocationLedger>` field is
//! updated to the qualified `Option<aggregates::AggregateAllocationLedger>`,
//! the exact same pattern those `units::` fields already use.
//!
//! `pub(super)` widenings, each load-bearing (named in the `D1` ledger
//! addendum, not silent): every mover with a named retained caller in
//! `core.rs`/`units.rs`/`mod.rs` itself. `GovernedAllocationSite` in
//! particular needed it for a construction site in `core.rs`'s retained
//! `transfer_constructor_operands` (`GovernedAllocationSite::
//! CarriedConstructor`) -- an enum-variant construction from a retained
//! file, the same missed-site shape as a delegating-wrapper call, found
//! only by a crate-wide reference sweep during `D1`, not predicted at
//! `D0`.

use super::*;

/// **`RT-DECL-CLOSURE-PORT` `D7` — the four sites that construct a governed
/// allocation request.**
///
/// ⭐ Named in production, not only under test. The domain is a real fact about
/// the emitter — these four are exactly the places an aggregate governed by a
/// planned record is allocated — and naming them is what lets a control act at
/// ONE of them while the other three stay honest.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum GovernedAllocationSite {
    /// A source `Construct`, carried through `emit_carrier_transfer`.
    SourceConstructor,
    /// A source `Record`, likewise.
    SourceRecord,
    /// The SELECTED alternative of a compiler-synthesized dynamic constructor.
    /// The set is not an allocation; the alternative is.
    DynamicAlternative,
    /// A constructor built from already-lowered operands at the process
    /// boundary (`transfer_constructor_operands`).
    CarriedConstructor,
}

/// **`D7` — the closed mutation surface for the governed-allocation controls.**
///
/// ⛔ `#[cfg(test)]`, so none of it exists in a shipped compiler. It is a
/// closed sum rather than a set of booleans for the same reason
/// [`CarrierAllocationRequest`] is: at most one perturbation can be installed
/// at a time, and "two bypasses at once" is not a state a control should be
/// able to reach by accident.
///
/// ⭐ Each variant acts at exactly ONE seam and increments the hit counter when
/// it does. A control that asserts only the refusal cannot tell "the site
/// bypassed and the choke caught it" from "the fixture never reached the site
/// and something else failed" — the hit count is what separates those, and it
/// is why every variant is required to prove it fired.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GovernedAllocationMutation {
    None,
    /// Hand the choke a `NonAggregate` request at the named site.
    Bypass(GovernedAllocationSite),
    /// Select the planned occurrence and record at a DIFFERENT live effect seat
    /// running the same host operation, retaining this seat's construction and
    /// operands. The A/B seat discriminator.
    SiblingEffectSeat,
    /// Transfer every source-call input at the program ROOT instead of at the
    /// coordinate the call supplies, keeping every value, callee, parameter
    /// slot, shape, lane and order. The call-USE coordinate discriminator.
    CallInputTransferOrigin,
    /// Give one source-call argument's TEMPLATE a sibling argument's
    /// planner-issued **producer occurrence**, keeping its own value, args,
    /// constructor symbol, call use, callee, parameter slot, shape, lane and
    /// order. The A/B aggregate-ownership discriminator.
    ///
    /// ⛔ **Not the same axis as [`Self::CallInputTransferOrigin`], and it may
    /// not be cited as this control.** That one moves the coordinate a value is
    /// *transferred at*; this one moves the certificate the value *carries*.
    /// Since `aggregate_carrier_authority` prefers the carried occurrence, a
    /// use-coordinate substitution is inert for exactly the templates that
    /// authorize themselves — which is every aggregate on this route.
    ///
    /// ⭐ The occurrence is taken from a **live sibling argument's own
    /// template**, never constructed. A hand-made identity would test that the
    /// record lookup rejects nonsense; taking a real one tests that ownership
    /// discriminates between two aggregates the plan considers equally real.
    SiblingAggregateProducer,
    /// Transfer a still-specialized call input at the PROGRAM ROOT's occurrence
    /// instead of the callee's scheduling entry, at the one call-input site that
    /// has no caller-side occurrence to carry.
    ///
    /// ⭐ The self-authority probe. A template that authorizes itself is
    /// unaffected by which coordinate it is transferred at; one that does not
    /// resolves a different record, or none at all.
    CalleeSchedulingOrigin,
}

#[cfg(test)]
thread_local! {
    static GOVERNED_ALLOCATION_MUTATION: std::cell::Cell<GovernedAllocationMutation> =
        const { std::cell::Cell::new(GovernedAllocationMutation::None) };
    static GOVERNED_ALLOCATION_HITS: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
    /// How many RAW carrier allocations have actually been emitted.
    ///
    /// ⭐ Counted at the `alloc` call itself, not at the choke's entry and not
    /// at the ledger. "Refused before any allocation" is a claim about emitted
    /// instructions, so the instrument has to sit where the instruction is
    /// emitted -- a counter one frame earlier would be satisfied by a refusal
    /// that had already allocated.
    static CARRIER_RAW_ALLOCATIONS: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
    /// What the ownership substitution actually moved, recorded at the seam.
    ///
    /// ⭐ Both records are read from the LIVE plan, at the moment of the swap.
    /// "The two arguments agree on shape and lane" is the premise that makes the
    /// negative a statement about IDENTITY rather than about shape, and a
    /// premise re-derived afterwards from a separately built plan is a premise
    /// about a different object.
    static SIBLING_PRODUCER_SUBSTITUTION: std::cell::Cell<Option<SiblingProducerSubstitution>> =
        const { std::cell::Cell::new(None) };
    /// How many SELF-AUTHORIZING aggregates reached the callee-scheduling-entry
    /// fallback -- the one call-input site with no caller-side occurrence.
    pub(super) static SELF_AUTHORIZED_FALLBACK_REACHES: std::cell::Cell<u32> =
        const { std::cell::Cell::new(0) };
    /// The last `(passed in, actually used)` coordinate pair the self-authority
    /// probe returned.
    ///
    /// ⛔ Recorded at the RETURN, not at the decision. A hit counter proves the
    /// seam decided to substitute; only this proves it substituted. Measured:
    /// with the seam's return value reverted to its argument while the counter
    /// still fired, the control stayed green -- a no-op substitution is
    /// otherwise indistinguishable from a well-defended one.
    static CALLEE_SCHEDULING_ORIGIN_USED: std::cell::Cell<
        Option<(StaticOriginId, StaticOriginId)>,
    > = const { std::cell::Cell::new(None) };
}

/// The exact ownership substitution one A/B run performed.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SiblingProducerSubstitution {
    /// The certificate the first argument carried before the substitution.
    pub(in crate::cranelift_backend) from: Option<AggregateOccurrenceId>,
    /// The sibling certificate it was given.
    pub(in crate::cranelift_backend) to: AggregateOccurrenceId,
    /// Whether the two records agree on planned shape.
    pub(crate) same_shape: bool,
    /// Whether the two records agree on planned allocation lane.
    pub(crate) same_lane: bool,
}

/// RAII installation of one mutation.
///
/// ⛔ The restore is in `Drop` rather than at the end of the control, because a
/// control that asserts a refusal is a control whose happy path can `panic!`
/// mid-way. A hand-written reset after the assertion would be skipped exactly
/// when the assertion fails, leaving the mutation installed for every test that
/// runs afterwards on this thread — a whole-suite corruption produced by the
/// one failure you were trying to diagnose.
#[cfg(test)]
pub(crate) struct GovernedAllocationMutationGuard {
    previous: GovernedAllocationMutation,
    previous_hits: u32,
    previous_allocations: u32,
    previous_substitution: Option<SiblingProducerSubstitution>,
    previous_reaches: u32,
    previous_origin_used: Option<(StaticOriginId, StaticOriginId)>,
}

#[cfg(test)]
impl GovernedAllocationMutationGuard {
    pub(crate) fn install(mutation: GovernedAllocationMutation) -> Self {
        let guard = Self {
            previous: GOVERNED_ALLOCATION_MUTATION.with(std::cell::Cell::get),
            previous_hits: GOVERNED_ALLOCATION_HITS.with(std::cell::Cell::get),
            previous_allocations: CARRIER_RAW_ALLOCATIONS.with(std::cell::Cell::get),
            previous_substitution: SIBLING_PRODUCER_SUBSTITUTION.with(std::cell::Cell::get),
            previous_reaches: SELF_AUTHORIZED_FALLBACK_REACHES.with(std::cell::Cell::get),
            previous_origin_used: CALLEE_SCHEDULING_ORIGIN_USED.with(std::cell::Cell::get),
        };
        GOVERNED_ALLOCATION_MUTATION.with(|cell| cell.set(mutation));
        GOVERNED_ALLOCATION_HITS.with(|cell| cell.set(0));
        CARRIER_RAW_ALLOCATIONS.with(|cell| cell.set(0));
        SIBLING_PRODUCER_SUBSTITUTION.with(|cell| cell.set(None));
        SELF_AUTHORIZED_FALLBACK_REACHES.with(|cell| cell.set(0));
        CALLEE_SCHEDULING_ORIGIN_USED.with(|cell| cell.set(None));
        guard
    }

    /// What the ownership substitution moved, if one fired.
    pub(crate) fn substitution(&self) -> Option<SiblingProducerSubstitution> {
        SIBLING_PRODUCER_SUBSTITUTION.with(std::cell::Cell::get)
    }

    /// How many self-authorizing aggregates reached the callee-scheduling-entry
    /// fallback.
    pub(crate) fn self_authorized_fallback_reaches(&self) -> u32 {
        SELF_AUTHORIZED_FALLBACK_REACHES.with(std::cell::Cell::get)
    }

    /// The `(passed in, actually used)` coordinate pair the self-authority probe
    /// last returned.
    pub(in crate::cranelift_backend) fn callee_scheduling_origin_used(
        &self,
    ) -> Option<(StaticOriginId, StaticOriginId)> {
        CALLEE_SCHEDULING_ORIGIN_USED.with(std::cell::Cell::get)
    }

    /// How many times this mutation's seam actually fired.
    pub(crate) fn hits(&self) -> u32 {
        GOVERNED_ALLOCATION_HITS.with(std::cell::Cell::get)
    }

    /// How many raw carrier allocations were emitted since this guard installed.
    ///
    /// ⚠ Zero is only meaningful beside a baseline that is NON-zero. On its
    /// own it is equally consistent with "refused before allocating" and with
    /// "this fixture never allocates", and those are different claims.
    pub(crate) fn raw_allocations(&self) -> u32 {
        CARRIER_RAW_ALLOCATIONS.with(std::cell::Cell::get)
    }
}

#[cfg(test)]
impl Drop for GovernedAllocationMutationGuard {
    fn drop(&mut self) {
        GOVERNED_ALLOCATION_MUTATION.with(|cell| cell.set(self.previous));
        GOVERNED_ALLOCATION_HITS.with(|cell| cell.set(self.previous_hits));
        CARRIER_RAW_ALLOCATIONS.with(|cell| cell.set(self.previous_allocations));
        SIBLING_PRODUCER_SUBSTITUTION.with(|cell| cell.set(self.previous_substitution));
        SELF_AUTHORIZED_FALLBACK_REACHES.with(|cell| cell.set(self.previous_reaches));
        CALLEE_SCHEDULING_ORIGIN_USED.with(|cell| cell.set(self.previous_origin_used));
    }
}

#[cfg(test)]
fn governed_allocation_hit() {
    GOVERNED_ALLOCATION_HITS.with(|hits| hits.set(hits.get().saturating_add(1)));
}


/// **`RT-DECL-CLOSURE-PORT` `D7` — the CLOSED request the deepest carrier
/// allocator accepts.**
///
/// ⭐ There is no way to hand the allocator a bare tag for an aggregate. A
/// caller either names a **planned record** — and the lane, the event and the
/// relation pair all follow from that record — or it declares the allocation
/// **non-aggregate**, in which case a `Constructor`/`Record` class is refused
/// before anything is emitted.
///
/// ⛔ This is what makes *"every governed allocation is in `E`"* a property of
/// the ALLOCATOR rather than of caller discipline. The predecessor paired a
/// checked wrapper that recorded evidence with a raw helper beside it that did
/// not, so the law held exactly as long as every future caller remembered which
/// one to reach for — an obligation nothing enforced and nothing measured. A
/// bypass was one plausible-looking line, and it would have produced an
/// allocation in no body's event set with no diagnostic anywhere.
///
/// ⛔ Not a `bool`, not an `Option<AggregateOccurrenceId>`. Both spellings
/// admit "governed, but with no record" and "ungoverned, but at an aggregate
/// class"; the sum admits neither, so the choke's two refusals are total over
/// the domain rather than over the cases someone enumerated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CarrierAllocationRequest {
    /// A scalar, spill, byte-bodied, `HostResult` or borrowed allocation. It
    /// names no record and never enters `E`.
    ///
    /// ⛔ The tag is the caller's precisely because there is no record to take
    /// one from — which is why this variant may not carry an aggregate class.
    NonAggregate { tag: BoundaryTag },
    /// An aggregate governed by a planned ownership record.
    ///
    /// ⛔ Carries no tag. The lane is read from the record at `occurrence`,
    /// never from the value in hand.
    PlannedAggregate {
        occurrence: AggregateOccurrenceId,
        shape: PlannedAggregateShape,
    },
}

impl CarrierAllocationRequest {
    /// The one node class an aggregate of this shape may be carried at.
    ///
    /// ⚠ Total over `PlannedAggregateShape`, so a third shape is a compile
    /// error here rather than a class that silently defaults.
    fn aggregate_class(shape: PlannedAggregateShape) -> BoundaryClass {
        match shape {
            PlannedAggregateShape::Constructor => BoundaryClass::Constructor,
            PlannedAggregateShape::Record => BoundaryClass::Record,
        }
    }
}


/// **`RT-DECL-CLOSURE-PORT` `D7` — the identity of one aggregate allocation
/// EVENT within one compilation.**
///
/// ⭐ `FuncId` is the exact declared function handed to `define_function`, and
/// it scopes **event evidence only** — never planner authority. The planner
/// keys records by `owner + seat + path + role`; this keys the *emissions* of
/// those records, which is a different question with a different answer.
///
/// ## Why the result `Value` alone is not an identity
///
/// A CLIF `Value` is numbered **per function**, so two bodies allocate at
/// `v12` routinely. A first prototype keyed on the value alone and refused six
/// lawful allocations. Adding the emission owner and the raw defining unit did
/// not fix it either — the same function is *built more than once*, so those
/// two coordinates still aliased. `FuncId` is the coordinate that actually
/// separates them, because it is what the module identifies a definition by.
///
/// ⛔ Not a build counter. A counter would make the identity depend on the
/// order bodies happen to be emitted in, which is exactly the row-driven
/// discovery this domain refuses.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct AggregateAllocationEvent {
    function: FuncId,
    result: cranelift_codegen::ir::Value,
}

/// The events of one body, open while that body is being emitted.
///
/// ⭐ **`events` and `relation` are INDEPENDENTLY MUTABLE, on purpose.** `E` is
/// what actually allocated; `R` is what those allocations were related to. A
/// single map cannot tell the two apart — its keys would BE its domain by
/// construction — so "an event was recorded but never related" and "a relation
/// entry exists for no event" would both be unstateable, and the local close
/// below would have nothing to compare. The law is `dom(R) = E`, and a law
/// needs two sides.
#[derive(Clone, Debug)]
struct LocalAggregateEvents {
    function: FuncId,
    /// `E` — one entry per governed raw allocation that actually happened.
    events: BTreeSet<cranelift_codegen::ir::Value>,
    /// `R` — the pairing of those events to the records that govern them.
    relation: BTreeMap<cranelift_codegen::ir::Value, AggregateOccurrenceId>,
}

/// **The compilation's event-to-record relation `R`.**
///
/// Each emitted body opens a fresh local set bound to its `FuncId`; the checked
/// wrapper records one pair per allocation; the set is committed after the
/// function is finalized and verified and **before** `define_function`. The
/// whole-pass closeout then states the relation's laws once.
#[derive(Clone, Debug, Default)]
pub(in crate::cranelift_backend) struct AggregateAllocationLedger {
    local: Option<LocalAggregateEvents>,
    /// `E` over the compilation, appended at each body commit.
    committed_events: BTreeSet<AggregateAllocationEvent>,
    /// `R` over the compilation.
    committed: BTreeMap<AggregateAllocationEvent, AggregateOccurrenceId>,
    /// Bodies whose event set was opened, and those whose commit landed. The
    /// two are compared at the close, so a discarded commit cannot pass as a
    /// body that simply allocated nothing.
    opened_functions: BTreeSet<FuncId>,
    committed_functions: BTreeSet<FuncId>,
}

impl AggregateAllocationLedger {
    /// Open a fresh local set for one body.
    ///
    /// ⛔ A second build of one `FuncId` rejects. There is no
    /// rollback-and-continue: a body that is built twice has emitted its
    /// allocations twice, and the relation cannot say which emission the
    /// records govern.
    pub(super) fn open(&mut self, function: FuncId) -> Result<(), CraneliftBackendError> {
        if self.committed_functions.contains(&function) {
            return Err(backend_module(format!(
                "aggregate allocation ledger: function {function} is built a second time, so \
                 its events would be recorded twice"
            )));
        }
        if let Some(open) = &self.local {
            return Err(backend_module(format!(
                "aggregate allocation ledger: function {} is still open while {function} \
                 starts, so an allocation could be attributed to the wrong body",
                open.function
            )));
        }
        if !self.opened_functions.insert(function) {
            return Err(backend_module(format!(
                "aggregate allocation ledger: function {function} opens a second time"
            )));
        }
        self.local = Some(LocalAggregateEvents {
            function,
            events: BTreeSet::new(),
            relation: BTreeMap::new(),
        });
        Ok(())
    }

    /// The open body, checked against the function the caller believes is open.
    fn open_body(
        &mut self,
        function: FuncId,
    ) -> Result<&mut LocalAggregateEvents, CraneliftBackendError> {
        let local = self.local.as_mut().ok_or_else(|| {
            backend_module(
                "aggregate allocation ledger: an allocation was emitted with no open body, so \
                 it belongs to no function's event set"
                    .to_string(),
            )
        })?;
        if local.function != function {
            return Err(backend_module(format!(
                "aggregate allocation ledger: an allocation in function {function} was recorded \
                 while {} is open",
                local.function
            )));
        }
        Ok(local)
    }

    /// Record that a governed allocation happened. This is `E`, and it is taken
    /// from the allocation itself — **never** derived from relation keys.
    pub(super) fn record_event(
        &mut self,
        function: FuncId,
        result: cranelift_codegen::ir::Value,
    ) -> Result<(), CraneliftBackendError> {
        let local = self.open_body(function)?;
        if !local.events.insert(result) {
            return Err(backend_module(format!(
                "aggregate allocation ledger: function {function} value {result} is already an \
                 event, so one raw allocation produced two"
            )));
        }
        Ok(())
    }

    /// Relate one event to the record that governs it.
    pub(super) fn relate(
        &mut self,
        function: FuncId,
        result: cranelift_codegen::ir::Value,
        occurrence: AggregateOccurrenceId,
    ) -> Result<(), CraneliftBackendError> {
        let local = self.open_body(function)?;
        match local.relation.insert(result, occurrence) {
            // ⛔ Both a duplicate and a conflict reject. One raw allocation
            // yields one result value, so a second pair at that value means
            // either the wrapper ran twice for one allocation or two
            // allocations share a result -- and neither can be reconciled.
            Some(previous) => Err(backend_module(format!(
                "aggregate allocation ledger: function {function} value {result} already maps to \
                 {previous:?}, so a second pair to {occurrence:?} is a duplicate or a conflict"
            ))),
            None => Ok(()),
        }
    }

    /// Commit the open body's pairs into the compilation relation.
    pub(super) fn commit(&mut self) -> Result<(), CraneliftBackendError> {
        let local = self.local.take().ok_or_else(|| {
            backend_module(
                "aggregate allocation ledger: a body commit ran with no open event set"
                    .to_string(),
            )
        })?;
        if !self.committed_functions.insert(local.function) {
            return Err(backend_module(format!(
                "aggregate allocation ledger: function {} commits a second time",
                local.function
            )));
        }
        // ⛔ `dom(R) = E` at the LOCAL close, before anything is committed.
        // An event with no relation entry is an allocation nothing authorized;
        // a relation entry with no event is an authorization nothing allocated.
        // Both are refusals, and neither is visible from one side alone.
        let related = local.relation.keys().copied().collect::<BTreeSet<_>>();
        if related != local.events {
            return Err(backend_module(format!(
                "aggregate allocation ledger: function {} has {} events and {} relation keys, \
                 so dom(R) is not E",
                local.function,
                local.events.len(),
                related.len()
            )));
        }
        // Both evidences are appended, so the whole-pass close can restate the
        // same law over the compilation rather than trusting each body's.
        for result in local.events {
            self.committed_events.insert(AggregateAllocationEvent {
                function: local.function,
                result,
            });
        }
        for (result, occurrence) in local.relation {
            let event = AggregateAllocationEvent {
                function: local.function,
                result,
            };
            if let Some(previous) = self.committed.insert(event, occurrence) {
                return Err(backend_module(format!(
                    "aggregate allocation ledger: {event:?} already maps to {previous:?}, so a \
                     committed pair is not unique"
                )));
            }
        }
        Ok(())
    }

    /// Drop the open body's evidence without committing it, for the
    /// discarded-commit control. There is no production path that does this:
    /// a body either commits or the pass fails.
    #[cfg(test)]
    pub(super) fn discard_open_body_for_tests(&mut self) {
        self.local = None;
    }

    /// Clear committed relation entries while leaving event evidence, for the
    /// cleared-relation control. No production path does this either.
    #[cfg(test)]
    pub(super) fn clear_committed_relation_for_tests(&mut self) {
        self.committed.clear();
    }

    /// **Close the relation once, after every body is emitted.**
    ///
    /// ⛔ The laws are stated over the WHOLE compilation, never per function.
    /// One record may govern many function-local events — a synthesized role at
    /// a seat reached under both a predeclared unit and a generated
    /// specialization allocates in both bodies — so `image(R_f) = P` is false
    /// for every individual `f` and imposing it would refuse lawful programs.
    pub(super) fn close(
        &mut self,
        planned: &[PlannedAggregateOwnership],
    ) -> Result<AggregateRelationClosure, CraneliftBackendError> {
        if let Some(open) = &self.local {
            return Err(backend_module(format!(
                "aggregate allocation ledger: function {} is still open at the whole-pass \
                 closeout, so its events were never committed",
                open.function
            )));
        }
        // ⭐ Every body that OPENED must have COMMITTED. A discarded commit
        // leaves its events uncommitted, and without this the artifact would
        // look like a body that simply allocated nothing.
        if self.opened_functions != self.committed_functions {
            return Err(backend_module(format!(
                "aggregate allocation ledger: {} bodies opened but {} committed, so a body's \
                 events were discarded",
                self.opened_functions.len(),
                self.committed_functions.len()
            )));
        }
        // ⭐ `dom(R) = E` over the whole compilation, restated from the two
        // independently accumulated evidences rather than trusted from the
        // per-body closes. Clearing committed relation entries between bodies
        // leaves the event evidence behind, and only this comparison sees it.
        let related = self.committed.keys().copied().collect::<BTreeSet<_>>();
        if related != self.committed_events {
            return Err(backend_module(format!(
                "aggregate allocation ledger: the compilation has {} events and {} relation \
                 keys, so dom(R) is not E",
                self.committed_events.len(),
                related.len()
            )));
        }
        let population = planned
            .iter()
            .map(|record| record.id)
            .collect::<BTreeSet<_>>();
        let image = self
            .committed
            .values()
            .copied()
            .collect::<BTreeSet<_>>();
        // ⭐ `image(R) ⊆ P`, and deliberately NOT equality.
        //
        // `P` is a closed AUTHORIZATION population, not an execution
        // obligation: it plans a record for every allocation-reachable node of
        // every seat's tree under every emission owner the seat may be lowered
        // by, while one compilation emits only the bodies it has. An unused
        // record is lawful. Measured before this was ruled: requiring equality
        // refused ordinary programs by 1 to 132 records.
        for occurrence in &image {
            if !population.contains(occurrence) {
                return Err(backend_module(format!(
                    "aggregate allocation ledger: {occurrence:?} is related by an event but is \
                     not in the planned population"
                )));
            }
        }
        Ok(AggregateRelationClosure {
            events: self.committed_events.len(),
            image: image.len(),
            population: population.len(),
            unused: population.difference(&image).count(),
        })
    }
}

/// What the whole-pass closeout measured.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct AggregateRelationClosure {
    pub(in crate::cranelift_backend) events: usize,
    pub(in crate::cranelift_backend) image: usize,
    pub(in crate::cranelift_backend) population: usize,
    /// Planned records no event related. **Lawful** — `P` authorizes, it does
    /// not oblige. Retained as a measurement, never as a failure condition.
    pub(in crate::cranelift_backend) unused: usize,
}


/// One argument to a compiler-synthesized constructor, in the FORM the tree
/// declares it.
///
/// ⭐ **The four forms are disjoint and the reconciliation matches on the pair
/// `(declared node, argument form)`.** A bare `Vec<Lowered>` cannot state which
/// form an operand is meant to be, so a site-bound child could only be checked
/// for position — and `SynthesizedAggregateNode::SiteOperand(_) => true` was
/// exactly that: arity proved the parent field position and nothing proved the
/// value in it was the operand whose lifetime justified the record. A different
/// value of the same shape and the same boundary disposition could inherit
/// operand `i`'s owner proof, which is the authority substitution `D7` exists
/// to prevent.
///
/// ⛔ Private to synthesized construction. Not a `Lowered` variant, not a
/// runtime tag, and nothing downstream sees it: the provenance is consumed by
/// the reconciliation and discarded, and the ordinary `Lowered` child is what
/// reaches the ordinary constructor field.
pub(super) enum SynthesizedArgument {
    /// A scalar the emitter materialized, for a `Scalar` node.
    Scalar(Lowered),
    /// A nested synthesized constructor, for a `Fixed` node.
    Nested(Lowered),
    /// A dynamic constructor, for a `Dynamic` node.
    Dynamic(Lowered),
    /// A value **projected from the seat's indexed operand**, for a
    /// `SiteOperand` node.
    ///
    /// All three axes are carried because all three are reconciled: the seat
    /// must be the one being lowered, the index must be the one the tree
    /// declares, and the value must still witness as the operand at that index.
    SiteOperand {
        seat: StaticOriginId,
        index: u32,
        value: Lowered,
        source: SiteOperandSource,
    },
}

/// The phase-bearing source that authorized a site-bound projection.
///
/// A carried projection necessarily creates fresh CLIF values, so
/// reconciliation cannot re-run the helper and compare SSA identities: a
/// second call would produce a second pair. Instead the projection records the
/// exact carried word plus the witness minted by the first call. The claimed
/// seat is re-read during reconciliation and must still hold that word.
pub(super) enum SiteOperandSource {
    Specialized,
    Carried {
        word: cranelift_codegen::ir::Value,
        projected: SiteOperandWitness,
    },
}

impl SynthesizedArgument {
    /// The `Lowered` child this argument becomes once its provenance has been
    /// reconciled and discarded.
    fn into_lowered(self) -> Lowered {
        match self {
            Self::Scalar(value) | Self::Nested(value) | Self::Dynamic(value) => value,
            Self::SiteOperand { value, .. } => value,
        }
    }

    fn lowered(&self) -> &Lowered {
        match self {
            Self::Scalar(value) | Self::Nested(value) | Self::Dynamic(value) => value,
            Self::SiteOperand { value, .. } => value,
        }
    }
}


impl<'a> Lowering<'a> {
        /// **`RT-DECL-CLOSURE-PORT` `D7` — reconcile every aggregate in a template
        /// against its OWN producer's planned ownership record, before anything is
        /// allocated.**
        ///
        /// ⭐ **It takes no origin, and that absence is the mechanism.** Every other
        /// reconciliation on this path is handed a coordinate and is therefore only
        /// as right as the coordinate it was handed — which is precisely the defect
        /// class this subclosure exists to close. Here there is no coordinate to
        /// pass, so there is no wrong one to pass: each node is checked against the
        /// record its own producer occurrence names, and a template can only be
        /// admitted by agreeing with the plan about itself.
        ///
        /// ⛔ **A missing producer is a REFUSAL, never a fallback.** An aggregate
        /// with no interned occurrence has no lifetime meet, so "resolve it at
        /// wherever it is being transferred" would reinstate exactly the
        /// use-coordinate authority the `occurrence` fields were added to retire.
        /// The fallback in [`Self::aggregate_carrier_authority`] survives only for
        /// values this preflight never sees.
        ///
        /// ⚠ **Whole-graph, and it runs BEFORE `emit_carrier_transfer`.** A nested
        /// child is allocated during its parent's transfer, so a check that fired
        /// only at each node's own allocation would already have allocated the
        /// parent by the time a child was refused. Walking the spine up front is
        /// what makes "refuses before any allocation" true of the whole tree rather
        /// than of its root.
        pub(super) fn source_aggregate_preflight(&self, value: &Lowered) -> Result<(), CraneliftBackendError> {
            match value {
                // ── the two source aggregates: reconciled AT THIS NODE ────────
                Lowered::Constructor { args, .. } => {
                    let children =
                        specialized_field_refs_at(args, "a constructor field crossing the boundary")?;
                    self.reconcile_source_aggregate(
                        value,
                        PlannedAggregateShape::Constructor,
                        &children,
                        None,
                    )?;
                    for arg in &children {
                        self.source_aggregate_preflight(arg)?;
                    }
                    Ok(())
                }
                Lowered::Record { fields, .. } => {
                    let children: Vec<&Lowered> = fields.iter().map(|field| &field.value).collect();
                    self.reconcile_source_aggregate(
                        value,
                        PlannedAggregateShape::Record,
                        &children,
                        Some(fields),
                    )?;
                    for field in fields {
                        self.source_aggregate_preflight(&field.value)?;
                    }
                    Ok(())
                }

                // ── recursive carriers that are not themselves aggregates ─────
                //
                // ⛔ These have no ownership record of their own and they are NOT
                // leaves. A walk that stopped here would leave every aggregate
                // below them unreconciled while reporting the tree admitted -- and
                // both of these positions are reached: a host result's two arms are
                // separate trees, and a dynamic alternative's fields are two levels
                // down inside a `Vec` of alternative structs.
                Lowered::HostResult { error, ok, .. } => {
                    self.source_aggregate_preflight(error)?;
                    self.source_aggregate_preflight(ok)
                }
                Lowered::DynamicConstructor(dynamic) => {
                    for alternative in &dynamic.alternatives {
                        for field in &alternative.fields {
                            self.source_aggregate_preflight(field)?;
                        }
                    }
                    Ok(())
                }

                // ── values that cannot cross at all ───────────────────────────
                //
                // ⛔ Admitted HERE means "this walk has nothing to reconcile", not
                // "this value may cross". Whether a closure has a boundary
                // representation is `boundary_transfer_admissibility`'s question,
                // it is decided before this walk runs on the same value, and it
                // refuses all three. Re-deciding it here would put a second, weaker
                // authority on a question that already has one -- and would report
                // a nested closure as an ownership failure.
                Lowered::Closure { .. }
                | Lowered::DeclarationClosure { .. }
                | Lowered::ComputationalRecursorClosure { .. } => Ok(()),

                // ── true leaves: no `Lowered` child position exists ───────────
                //
                // ⛔ No `_` arm, by construction. A new variant with a child
                // position is a compile error here rather than a subtree that
                // silently stops being reconciled -- which is exactly how
                // `HostResult` and `DynamicConstructor` were missed.
                Lowered::Int { .. }
                | Lowered::Bool { .. }
                | Lowered::ProcessExitStatus { .. }
                | Lowered::CapabilityToken { .. }
                | Lowered::ResourceToken { .. }
                | Lowered::BoundedNat(_)
                | Lowered::StructuralNat(_)
                | Lowered::ResponseBytes { .. }
                | Lowered::Bytes(_)
                | Lowered::BorrowedNativeValue { .. }
                | Lowered::BorrowedOption { .. }
                | Lowered::String(_)
                | Lowered::RecursiveBackedge
                | Lowered::Trap(_) => Ok(()),
            }
        }
}

impl<'a> Lowering<'a> {
        /// Reconcile ONE source aggregate node against the ownership record its own
        /// producer occurrence names.
        ///
        /// ⛔ Takes no origin, for the reason [`Self::source_aggregate_preflight`]
        /// states: there is no coordinate to pass, so there is no wrong one.
        fn reconcile_source_aggregate(
            &self,
            value: &Lowered,
            shape: PlannedAggregateShape,
            children: &[&Lowered],
            record_fields: Option<&[LoweredRecordField]>,
        ) -> Result<(), CraneliftBackendError> {
            let Some(occurrence) = value.source_aggregate_producer() else {
                return Err(unsupported(
                    lowered_value_kind(value),
                    "a source aggregate reached the carrier with no planner-issued producer \
                     occurrence, so it would name no ownership record and could only be given \
                     the authority of wherever it happened to be transferred",
                ));
            };
            let planned = self
                .static_transition_plan
                .aggregate_record_view(occurrence)?;
            if planned.shape() != shape {
                return Err(unsupported(
                    lowered_value_kind(value),
                    format!(
                        "the template is a {shape:?} but its own producer occurrence names a \
                         {:?} ownership record",
                        planned.shape()
                    ),
                ));
            }
            // ⭐ **The CLASS is a second, independent reading of the same fact.**
            // The variant match above is what the template *is*; this is what the
            // sole disposition authority says it *represents*. They are derived by
            // different code from different fields, so a template whose variant and
            // disposition ever disagree is caught here rather than allocating under
            // one and being emitted under the other.
            let (_, class) = Self::carrier_handle_disposition(value)?;
            let planned_class = CarrierAllocationRequest::aggregate_class(planned.shape());
            if class != planned_class {
                return Err(unsupported(
                    lowered_value_kind(value),
                    format!(
                        "the template's boundary class is {class:?} but its own producer \
                         occurrence names a {planned_class:?} ownership record"
                    ),
                ));
            }
            if planned.children().len() != children.len() {
                return Err(unsupported(
                    lowered_value_kind(value),
                    format!(
                        "the template has {} children but its own producer occurrence names an \
                         ownership record planned with {}",
                        children.len(),
                        planned.children().len()
                    ),
                ));
            }
            // ⭐ **The constructor SCHEMA, against the producer's own origin.**
            // A source constructor's `synthesized_identity` was resolved at the
            // producer and travels with the template for the same reason the
            // occurrence does; here the two carried facts are made to agree with
            // each other through the plan. A template that acquired one producer's
            // occurrence and another's symbol -- the exact shape of a grafted or
            // substituted certificate -- cannot satisfy both.
            //
            // ⚠ Gated to a SOURCE producer: a compiler-synthesized constructor's
            // identity comes from the semantic plane's closed role capability and
            // has no source origin to resolve against.
            if let Some(producer_origin) = planned.producer_origin() {
                if let Lowered::Constructor {
                    synthesized_identity: Some(carried),
                    ..
                } = value
                {
                    let planned_identity = self
                        .static_transition_plan
                        .constructor_symbol_identity(producer_origin)?;
                    if *carried != planned_identity {
                        return Err(unsupported(
                            lowered_value_kind(value),
                            "the template's carried constructor identity is not the one the \
                             planner resolved at its own producer occurrence's origin",
                        ));
                    }
                }
            }
            for (position, (child, planned_child)) in
                children.iter().zip(planned.children()).enumerate()
            {
                // ⭐ **The RECORD half of the schema, compared EXACTLY and by
                // TYPE.** The template carries the identity its producer was issued
                // and the plan states the identity it planned at this position;
                // both are `FieldIdentity`, so the comparison is the identity
                // itself rather than a spelling. ⛔ There is no `&str ->
                // FieldIdentity` direction and none may be added: comparing the
                // template's field STRING against the plan would be the second
                // derivation `D2` forbids, and it is what left field naming
                // unreconciled while order and arity were covered.
                if let Some(fields) = record_fields {
                    let held = fields[position].identity;
                    match (held, planned_child.field_identity) {
                        // ⛔ Not `held != planned`. Two absences comparing equal is
                        // the shape that admits a record with no schema at all
                        // against a record the planner planned no schema for.
                        (Some(held), Some(planned)) if held == planned => {}
                        (held, planned) => {
                            return Err(unsupported(
                                lowered_value_kind(value),
                                format!(
                                    "record field {position} carries identity {held:?} but its own \
                                     producer occurrence names a record planned with {planned:?} \
                                     at that position"
                                ),
                            ));
                        }
                    }
                }
                // ⭐ **The child's own possible-owner set, against the set the meet
                // was taken over.** The planner's set is what the parent's lane was
                // DERIVED from; this is what the sole disposition authority and the
                // child's own static encoding say the emitter will actually build.
                // A child that can be owned by something the meet never considered
                // makes the parent's lane a conclusion from the wrong premises --
                // and when that something is the invocation arena, a persistent
                // parent ends up naming storage that dies first.
                // ⭐ **The planner's TWO fields about this position must agree,
                // before either is used.** `owners` is the set the meet was taken
                // over and `lifetime` is the lifetime it was taken under, and the
                // law relating them is one-directional: the invocation arena is a
                // possible owner ONLY IF the position is activation-owned.
                //
                // ⛔ **Not an equivalence, and the asymmetry is a measured fact
                // about the planner rather than a hedge.** `owners` is the
                // lifetime's affinity INTERSECTED with the child's representation,
                // so a child the emitter materializes as a native scalar pair is
                // recorded `ActivationOwned` over `[NoReferent]` -- it may be
                // short-lived and still have no boundary node for anything to own.
                // Stating this as `==` refuses that lawful record, measured on the
                // very fixture the owner control is built from.
                //
                // ⚠ Stated as a law rather than assumed, because everything below
                // reads whichever field answers its question: a record whose owners
                // admitted the arena under a persistent lifetime would let a
                // containment pass under one field while the lane was concluded
                // from the other.
                if planned_child
                    .owners
                    .contains(&BoundaryReferentOwner::InvocationArena)
                    && planned_child.lifetime != PlannedReferentLifetime::ActivationOwned
                {
                    return Err(unsupported(
                        lowered_value_kind(child),
                        format!(
                            "the ownership record's own fields disagree at child {}: it plans a \
                             {:?} referent lifetime and possible owners {:?}",
                            planned_child.position, planned_child.lifetime, planned_child.owners,
                        ),
                    ));
                }
                // ⛔ **Containment, and the DIRECTION is the whole content.** A held
                // child may be LONGER-lived than the position planned for: a
                // persistent child sitting where the meet allowed an
                // activation-owned one dangles nothing. Requiring the two sets to be
                // EQUAL reds every spillable immediate, whose closed set is
                // `{NoReferent, PersistentStore}` at a position the planner
                // legitimately plans `ActivationOwned` -- measured on two fixtures
                // before the direction was fixed. It may never be SHORTER: that is
                // the parent naming storage that dies first, which is the edge this
                // subclosure exists to close.
                //
                // ⚠ The held set is read from the disposition and, for an aggregate,
                // from its own ruled lane -- never from the value's shape.
                // `Constructor` and `Record` are persistable SHAPES, which is
                // precisely why the shape cannot answer this.
                let held_owners = self.child_possible_referent_owners(child)?;
                if let Some(escaped) = held_owners
                    .iter()
                    .find(|owner| !planned_child.owners.contains(owner))
                {
                    let held_lifetime = Self::possible_owners_lifetime(&held_owners);
                    return Err(unsupported(
                        lowered_value_kind(child),
                        format!(
                            "child {} is held with a {held_lifetime:?} referent lifetime and can \
                             be owned by {escaped:?}, which its own producer occurrence's \
                             ownership record did not plan for that position (planned {:?} over \
                             {:?})",
                            planned_child.position, planned_child.lifetime, planned_child.owners,
                        ),
                    ));
                }
                // ⚠ Gated to a SOURCE producer. A compiler-synthesized aggregate's
                // children have no occurrence in the program, and their agreement
                // with the plan is already established -- by path, role and
                // disposition -- in [`Self::reconcile_declared_children`].
                // Re-deriving it here from source origins the planner deliberately
                // recorded as absent would be a second, weaker authority for a
                // question that already has one.
                if planned.producer_origin().is_none() {
                    continue;
                }
                let Some(child_shape) = Self::lowered_aggregate_shape(child) else {
                    continue;
                };
                let Some(child_occurrence) = child.source_aggregate_producer() else {
                    return Err(unsupported(
                        lowered_value_kind(child),
                        "an aggregate child reached the carrier with no planner-issued producer \
                         occurrence, so its producer class cannot be established",
                    ));
                };
                // Producer class, not aggregate shape, decides whether a child has
                // source coordinates to validate. Source-produced children retain
                // the exact per-position lookup below. A synthesized child has no
                // source occurrence by construction; its generic ownership record
                // was already checked above and is checked again when the recursive
                // preflight visits that child. The source-child certificate control
                // reaches this arm and still refuses a sibling occurrence, while
                // the unit-boundary Record reaches the synthesized branch.
                if self
                    .static_transition_plan
                    .aggregate_record_view(child_occurrence)?
                    .producer_origin()
                    .is_none()
                {
                    continue;
                }
                let Some(child_origin) = planned_child.origin else {
                    return Err(unsupported(
                        lowered_value_kind(child),
                        "a source aggregate's child is an aggregate, but the planner recorded \
                         no source occurrence for it at that position",
                    ));
                };
                let expected = self
                    .static_transition_plan
                    .source_aggregate_occurrence(child_origin, child_shape)?;
                // ⭐ The child must carry the occurrence the planner planned AT
                // THAT POSITION -- not merely some record of the same shape, and
                // not the same producer's record reached from elsewhere in the
                // tree. Swap two children, graft a sibling's subtree in, or hand a
                // forwarded aggregate a neighbour's certificate, and the carried
                // occurrence stops matching the position it sits at.
                if child.source_aggregate_producer() != Some(expected) {
                    return Err(unsupported(
                        lowered_value_kind(child),
                        format!(
                            "child {} carries producer occurrence {:?} but the planner planned \
                             {expected:?} at that position",
                            planned_child.position,
                            child.source_aggregate_producer(),
                        ),
                    ));
                }
            }
            Ok(())
        }
}

impl<'a> Lowering<'a> {
        /// The **closed** set of referent owners one held child can have.
        ///
        /// ⭐ Derived from the two authorities that already own the question and
        /// from nothing else: [`Lowered::boundary_disposition`] for what the value
        /// represents, and -- for an aggregate -- its OWN planned allocation lane
        /// for what it will be allocated in. ⛔ Never re-derived from a runtime tag
        /// and never guessed from the value's shape: `Constructor` and `Record` are
        /// persistable shapes, which is precisely why the shape cannot answer this.
        fn child_possible_referent_owners(
            &self,
            child: &Lowered,
        ) -> Result<Vec<BoundaryReferentOwner>, CraneliftBackendError> {
            match child.boundary_disposition() {
                // An immediate with no spill class has no boundary node at any
                // magnitude, so there is nothing for an arena or a store to own.
                BoundaryDisposition::RepresentedImmediate { spill: None, .. } => {
                    Ok(vec![BoundaryReferentOwner::NoReferent])
                }
                // ⚠ A spillable immediate has TWO representations and the choice is
                // a runtime magnitude, so both owners are possible and the set says
                // so. Collapsing it to either one would be a determination this
                // static walk is not entitled to make.
                BoundaryDisposition::RepresentedImmediate { spill: Some(_), .. } => Ok(vec![
                    BoundaryReferentOwner::NoReferent,
                    BoundaryReferentOwner::PersistentStore,
                ]),
                BoundaryDisposition::RepresentedHandle { tag, .. } => {
                    let Some(_) = Self::lowered_aggregate_shape(child) else {
                        // A non-aggregate handle's owner IS its tag's, exactly as
                        // the closed discriminator product reads it.
                        return Ok(vec![tag.referent_owner()]);
                    };
                    // ⭐ An aggregate's owner comes from its OWN ownership record's
                    // ruled lane, never from the tag its shape reached for. This is
                    // the same read the allocation itself makes, so a child whose
                    // lane disagrees with its position in the parent is caught here
                    // rather than after both are allocated.
                    let Some(occurrence) = child.source_aggregate_producer() else {
                        return Err(unsupported(
                            lowered_value_kind(child),
                            "a source aggregate child reached the carrier with no planner-issued \
                             producer occurrence, so its allocation lane -- and therefore its \
                             referent owner -- has no authority",
                        ));
                    };
                    let allocation = self
                        .static_transition_plan
                        .aggregate_record_view(occurrence)?
                        .allocation();
                    Ok(vec![match allocation {
                        PlannedAggregateAllocation::PersistentGround => {
                            BoundaryReferentOwner::PersistentStore
                        }
                        PlannedAggregateAllocation::InvocationAggregate => {
                            BoundaryReferentOwner::InvocationArena
                        }
                    }])
                }
                BoundaryDisposition::ProtocolOnly { why }
                | BoundaryDisposition::FailClosedForbidden { why } => {
                    Err(unsupported(lowered_value_kind(child), why))
                }
            }
        }
}

impl<'a> Lowering<'a> {
        /// The referent lifetime a closed possible-owner set encodes.
        ///
        /// ⛔ Membership, not a determination: a child is activation-owned exactly
        /// when the invocation arena is a possible owner of it, which is the same
        /// rule the planner's own meet is taken under.
        fn possible_owners_lifetime(owners: &[BoundaryReferentOwner]) -> PlannedReferentLifetime {
            if owners.contains(&BoundaryReferentOwner::InvocationArena) {
                PlannedReferentLifetime::ActivationOwned
            } else {
                PlannedReferentLifetime::Persistent
            }
        }
}

impl<'a> Lowering<'a> {
        /// The planned aggregate shape a template is, if it is an aggregate at all.
        fn lowered_aggregate_shape(value: &Lowered) -> Option<PlannedAggregateShape> {
            match value {
                Lowered::Constructor { .. } => Some(PlannedAggregateShape::Constructor),
                Lowered::Record { .. } => Some(PlannedAggregateShape::Record),
                _ => None,
            }
        }
}

impl<'a> Lowering<'a> {
        /// Replace one directly carried, empty lexical environment with the
        /// planner-issued synthesized record that names it.
        ///
        /// The planner and lowering derive the key independently. The planner
        /// follows the closed result of a direct lexical-closure call argument to
        /// one source constructor field. Lowering recovers that constructor's
        /// source origin from the occurrence carried on the template and enumerates
        /// the actual field position here, at the generated-unit boundary. The
        /// lookup succeeds only when owner, producer, structural path, and role all
        /// agree.
        ///
        /// An absent key leaves the closure unchanged, so the existing whole-graph
        /// refusal remains the authority for every unplanned occurrence.
        pub(super) fn unit_boundary_environment_record(
            &self,
            value: Lowered,
        ) -> Result<Lowered, CraneliftBackendError> {
            let Some(owner) = self.defining_emission_owner else {
                return Ok(value);
            };
            let (constructor, synthesized_identity, occurrence, args) = match value {
                Lowered::Constructor {
                    constructor,
                    synthesized_identity,
                    occurrence,
                    args,
                } => (constructor, synthesized_identity, occurrence, args),
                other => return Ok(other),
            };
            let Some(aggregate_occurrence) = occurrence else {
                return Ok(Lowered::Constructor {
                    constructor,
                    synthesized_identity,
                    occurrence,
                    args,
                });
            };
            let Some(producer) = self
                .static_transition_plan
                .aggregate_record_view(aggregate_occurrence)?
                .producer_origin()
            else {
                return Ok(Lowered::Constructor {
                    constructor,
                    synthesized_identity,
                    occurrence,
                    args,
                });
            };
            let args = args
                .into_iter()
                .enumerate()
                .map(|(position, field)| {
                    let ConstructorField::Specialized(Lowered::Closure {
                        captures,
                        ..
                    }) = &field
                    else {
                        return Ok(field);
                    };
                    if !captures.is_empty() {
                        return Ok(field);
                    }
                    let position = u32::try_from(position).map_err(|_| {
                        backend_module(
                            "unit-boundary environment field exceeds the position space"
                                .to_string(),
                        )
                    })?;
                    let Some(occurrence) = self
                        .static_transition_plan
                        .unit_boundary_environment_occurrence(owner, producer, position)
                    else {
                        return Ok(field);
                    };
                    Ok(ConstructorField::specialized(Lowered::Record {
                        occurrence: Some(occurrence),
                        fields: Vec::new(),
                    }))
                })
                .collect::<Result<Vec<_>, CraneliftBackendError>>()?;
            Ok(Lowered::Constructor {
                constructor,
                synthesized_identity,
                occurrence,
                args,
            })
        }
}

impl<'a> Lowering<'a> {
        /// The coordinate source-call inputs are transferred at, under the
        /// call-use mutation only.
        ///
        /// ⚠ **The hit is counted only when the coordinate actually CHANGES.** A
        /// call already made at the root would otherwise report a substitution that
        /// substituted nothing, which is indistinguishable from a well-defended one.
        #[cfg(test)]
        pub(super) fn call_input_transfer_origin_under_mutation(
            &self,
            origin: StaticOriginId,
        ) -> Result<StaticOriginId, CraneliftBackendError> {
            if GOVERNED_ALLOCATION_MUTATION.with(std::cell::Cell::get)
                != GovernedAllocationMutation::CallInputTransferOrigin
            {
                return Ok(origin);
            }
            let root = self.static_transition_plan.root_static_origin()?;
            if root != origin {
                governed_allocation_hit();
            }
            Ok(root)
        }
}

impl<'a> Lowering<'a> {
        /// Replace the FIRST argument's carried **producer occurrence** with the
        /// second's, under the A/B ownership mutation only.
        ///
        /// ⛔ Only the certificate moves. The template keeps its own constructor
        /// symbol, its own children, its own resolved identity, its own call use and
        /// its own parameter slot, so the emitter builds exactly what it built
        /// before and only the ownership record it claims differs.
        ///
        /// ⚠ **The hit is counted only when the occurrence actually CHANGES.** A
        /// call whose two arguments already share a record, or whose first argument
        /// is not a specialized aggregate, leaves the list untouched -- and a
        /// substitution that substitutes nothing is indistinguishable from a
        /// well-defended one if the counter fires anyway.
        #[cfg(test)]
        pub(super) fn substitute_sibling_aggregate_producer(
            &self,
            mut inputs: Vec<LoweringOperand>,
        ) -> Vec<LoweringOperand> {
            if GOVERNED_ALLOCATION_MUTATION.with(std::cell::Cell::get)
                != GovernedAllocationMutation::SiblingAggregateProducer
            {
                return inputs;
            }
            if inputs.len() < 2 {
                return inputs;
            }
            let LoweringOperand::Specialized(sibling) = &inputs[1] else {
                return inputs;
            };
            let Some(sibling) = sibling.source_aggregate_producer() else {
                return inputs;
            };
            let LoweringOperand::Specialized(target) = &mut inputs[0] else {
                return inputs;
            };
            let carried = match target {
                Lowered::Constructor { occurrence, .. } | Lowered::Record { occurrence, .. } => {
                    occurrence
                }
                _ => return inputs,
            };
            if *carried == Some(sibling) {
                return inputs;
            }
            let replaced = *carried;
            *carried = Some(sibling);
            let agreement = replaced.and_then(|replaced| {
                let before = self
                    .static_transition_plan
                    .aggregate_record_view(replaced)
                    .ok()?;
                let after = self
                    .static_transition_plan
                    .aggregate_record_view(sibling)
                    .ok()?;
                Some((
                    before.shape() == after.shape(),
                    before.allocation() == after.allocation(),
                ))
            });
            SIBLING_PRODUCER_SUBSTITUTION.with(|cell| {
                cell.set(Some(SiblingProducerSubstitution {
                    from: replaced,
                    to: sibling,
                    same_shape: agreement.is_some_and(|(shape, _)| shape),
                    same_lane: agreement.is_some_and(|(_, lane)| lane),
                }));
            });
            governed_allocation_hit();
            inputs
        }
}

impl<'a> Lowering<'a> {
        /// The recursive emission step. ⛔ Private, and ⛔ never the entry point —
        /// see [`Self::transfer_into_carrier`] for why the split is not stylistic.
        ///
        /// ⭐ **The dispatch is an exhaustive `match` on the variant, and the
        /// `(tag, class)` comes from [`Lowered::boundary_disposition`].** Both
        /// halves are load-bearing and they answer different questions: the
        /// disposition is the **sole authority** for how a value is represented
        /// (`§2h` ¶4 makes reading it *required*), while the variant match is what
        /// supplies the **payload and children**, which a disposition cannot carry
        /// because it is a function of the variant tag alone.
        ///
        /// ⛔ **No wildcard arm.** A 22nd `Lowered` inhabitant is a compile error
        /// here, exactly as it is in `variant()` and
        /// `boundary_transfer_admissibility` — so a new carrier of children cannot
        /// be added without someone deciding whether it can cross.
        pub(super) fn emit_carrier_transfer(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            origin: StaticOriginId,
            value: &Lowered,
        ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
            match value {
                // ── the supported transfer surface ───────────────────────────
                Lowered::Bool { value: word, .. } => {
                    let tag = Self::carrier_immediate_tag(value)?;
                    self.emit_carrier_immediate(builder, tag, *word)
                }
                // ── the magnitude dispatch: `spill: Some(_)` immediates ───────
                //
                // ⭐ Four variants, ONE mechanism — which is the corrected `D9`
                // partition. ⛔ They are not four pieces of work and must not be
                // spelled as four arms with four bodies: the disposition supplies
                // the tag and the spill class, so the only thing that differs
                // between them is where the payload word and its `NativeIntV1`
                // marker come from.
                Lowered::Int {
                    value: payload,
                    known,
                } => {
                    let (tag, spill) = Self::carrier_spillable_disposition(value)?;
                    // ⛔ The marker travels with the payload; see
                    // `carrier_small_marker` for why this is not a constant.
                    let marker = self.native_int_tag(builder, *payload, *known)?;
                    self.emit_carrier_native_int(builder, tag, spill, *payload, marker)
                }
                Lowered::ProcessExitStatus { value: payload }
                | Lowered::BoundedNat(BoundedNatV1 { value: payload })
                | Lowered::StructuralNat(StructuralNatV1 { value: payload }) => {
                    let (tag, spill) = Self::carrier_spillable_disposition(value)?;
                    let marker = Self::carrier_small_marker(builder);
                    self.emit_carrier_spillable_immediate(builder, tag, spill, *payload, marker)
                }
                // ── byte-bodied handles ───────────────────────────────────────
                //
                // ⛔ Two arms, ONE emitter, and the class comes from the
                // disposition rather than from which arm we are in — see
                // `emit_carrier_bytes` for why a shared body driven by the class is
                // the thing that makes `String`'s guard reachable at all.
                Lowered::String(text) => {
                    let (tag, class) = Self::carrier_handle_disposition(value)?;
                    self.emit_carrier_bytes(builder, tag, class, text.as_bytes())
                }
                Lowered::Bytes(content) => {
                    let (tag, class) = Self::carrier_handle_disposition(value)?;
                    self.emit_carrier_bytes(builder, tag, class, content)
                }
                Lowered::Constructor {
                    constructor,
                    synthesized_identity,
                    // Read from `value` by the disposition below, which needs the
                    // whole template. Bound explicitly rather than swallowed by a
                    // `..` so a further field is a compile error here.
                    occurrence: _,
                    args,
                } => {
                    let (occurrence, class) = self.aggregate_carrier_authority(
                        origin,
                        value,
                        PlannedAggregateShape::Constructor,
                    )?;
                    let schema_origin = self.aggregate_schema_origin(occurrence, origin)?;
                    // ⭐ `D2` — the identity comes from the ONE artifact-static
                    // authority, via the typed newtype's own ABI-word method. ⛔ Not
                    // `intern_symbol`, which is dense insertion-order numbering over
                    // one store instance and therefore a *different* number in a
                    // different store (`§2e`).
                    let identity = match synthesized_identity {
                        Some(identity) => *identity,
                        None => self
                            .static_transition_plan
                            .constructor_symbol_identity(schema_origin)
                            .map_err(|error| {
                                backend_module(format!(
                                    "constructor transfer for {constructor} at {:?} has no \
                                     resolved identity: {error}",
                                    schema_origin
                                ))
                            })?,
                    }
                    .tag_abi_word()?;
                    // The fields are read BEFORE the allocation, deliberately. This
                    // arm materializes the constructor, so a field read placed
                    // inside the store loop below would refuse only after
                    // `emit_checked_aggregate_alloc` had already run — the
                    // *"descends partway and then refuses"* shape the ruling
                    // forbids.
                    //
                    // **Emitted instruction order is still unchanged now that
                    // this read CAN fail, and that is measured rather than assumed
                    // from unconstructibility.** The premise used to be that
                    // nothing constructed a worker; with the producer armed, the
                    // claim rests instead on reachability: every route into this
                    // function is screened whole-graph first.
                    // `transfer_into_carrier` runs `boundary_transfer_admissibility`
                    // and `source_aggregate_preflight` before calling it,
                    // `transfer_constructor_operands` runs the same pair ahead of
                    // its own allocation, this function's recursion only descends
                    // into a subgraph its parent already screened, and
                    // `emit_carrier_dynamic_constructor` is reached only from here.
                    // ⇒ A worker-bearing template cannot arrive, so this read
                    // cannot fail in production and no instruction is emitted
                    // before it either way.
                    //
                    // This is local ordering, not the whole-graph boundary — the
                    // boundary is the screening pair named above, and no count of
                    // arms like this one is ever evidence about it.
                    let arguments =
                        specialized_field_refs_at(args, "a constructor field being materialized")?;
                    let word = self.emit_checked_aggregate_alloc(
                        builder,
                        GovernedAllocationSite::SourceConstructor,
                        occurrence,
                        PlannedAggregateShape::Constructor,
                        class,
                        arguments.len(),
                    )?;
                    self.emit_carrier_store_tag_id(builder, word, identity)?;
                    for (position, argument) in arguments.into_iter().enumerate() {
                        let child = self.emit_carrier_transfer(builder, origin, argument)?;
                        self.emit_carrier_store_field(builder, word, position, child)?;
                    }
                    Ok(word)
                }
                Lowered::Record { fields, .. } => {
                    let (occurrence, class) = self.aggregate_carrier_authority(
                        origin,
                        value,
                        PlannedAggregateShape::Record,
                    )?;
                    // ⛔ No schema coordinate here, and its absence is the point:
                    // a record's field names now travel on the template, so this
                    // arm resolves NOTHING at the coordinate it is transferred at.
                    let word = self.emit_checked_aggregate_alloc(
                        builder,
                        GovernedAllocationSite::SourceRecord,
                        occurrence,
                        PlannedAggregateShape::Record,
                        class,
                        fields.len(),
                    )?;
                    for (position, field) in fields.iter().enumerate() {
                        // ⭐ `D2` at the field-identity namespace: the name written
                        // here and the name `Project` looks up are the same word
                        // from the same authority. ⚠ The `String` key on the field
                        // is deliberately NOT the identity — it is the compile-time
                        // spelling, and using it would be the second derivation
                        // `D2` forbids.
                        //
                        // ⭐⭐ **The identity EMITTED is the one the preflight
                        // COMPARED.** It travels on the template from the producer
                        // that was issued it, and the whole-graph walk has already
                        // made it agree with the plan at this exact position. ⛔ Not
                        // a fresh `record_field_identity` lookup: a second read at
                        // emission is a second authority, and it answers from
                        // whatever coordinate is in scope here rather than from the
                        // fact that was checked.
                        let Some(identity) = field.identity else {
                            return Err(unsupported(
                                "Record",
                                format!(
                                    "record field {position} reached the carrier with no                                  planner-issued identity, so its name would have to be                                  invented at the coordinate it is transferred at"
                                ),
                            ));
                        };
                        self.emit_carrier_store_name(
                            builder,
                            word,
                            position,
                            identity.name_abi_word()?,
                        )?;
                        let child = self.emit_carrier_transfer(builder, origin, &field.value)?;
                        self.emit_carrier_store_field(builder, word, position, child)?;
                    }
                    Ok(word)
                }

                // ── ⛔ FAIL CLOSED — and these are DEFERRALS, said plainly ────
                //
                // ⚠ A deferral is honest; a deferral that reads as delivery is not.
                // Each arm below is a form the carrier ABI *can* represent and this
                // producer does not yet emit. ⛔ Do not read the fail-closed status
                // as "the boundary refuses this" — `boundary_disposition` admits
                // most of them. The refusal is **this producer's**, and it is
                // conservative rather than silent precisely so the gap cannot be
                // mistaken for coverage.
                Lowered::HostResult {
                    success, error, ok, ..
                } => {
                    let (tag, class) = Self::carrier_handle_disposition(value)?;
                    let ok = self.emit_carrier_transfer(builder, origin, ok)?;
                    let error = self.emit_carrier_transfer(builder, origin, error)?;
                    let word = self.emit_carrier_alloc(
                        builder,
                        CarrierAllocationRequest::NonAggregate { tag },
                        class,
                        2,
                    )?;
                    let success = if builder.func.dfg.value_type(*success) == types::I64 {
                        *success
                    } else {
                        builder.ins().uextend(types::I64, *success)
                    };
                    self.emit_carrier_store_scalar(builder, word, success)?;
                    self.emit_carrier_store_field(builder, word, 0, ok)?;
                    self.emit_carrier_store_field(builder, word, 1, error)?;
                    Ok(word)
                }
                Lowered::DynamicConstructor(dynamic) => {
                    self.emit_carrier_dynamic_constructor(builder, origin, dynamic)
                }
                Lowered::ResourceToken { value: payload } => {
                    let (tag, class) = Self::carrier_handle_disposition(value)?;
                    let word = self.emit_carrier_alloc(
                        builder,
                        CarrierAllocationRequest::NonAggregate { tag },
                        class,
                        0,
                    )?;
                    self.emit_carrier_store_scalar(builder, word, *payload)?;
                    Ok(word)
                }
                Lowered::CapabilityToken { value: payload } => {
                    let (tag, class) = Self::carrier_handle_disposition(value)?;
                    let word = self.emit_carrier_alloc(
                        builder,
                        CarrierAllocationRequest::NonAggregate { tag },
                        class,
                        0,
                    )?;
                    self.emit_carrier_store_scalar(builder, word, *payload)?;
                    Ok(word)
                }
                Lowered::BorrowedNativeValue { pointer } => {
                    let (tag, class) = Self::carrier_handle_disposition(value)?;
                    let word = self.emit_carrier_alloc(
                        builder,
                        CarrierAllocationRequest::NonAggregate { tag },
                        class,
                        0,
                    )?;
                    self.emit_carrier_store_scalar(builder, word, *pointer)?;
                    Ok(word)
                }
                // ⭐ `RT-CARRIER-BYTESPAN-OBSERVE` `D2` — NORMALIZED AT THE
                // PRODUCER, by copy, per Architect `dec_6qmstfn6tjqdt`.
                //
                // ⛔ This arm used to publish the HOST POINTER as a
                // `BorrowedOpaque` scalar with the length beside it as a child
                // word. That word died with the invocation and no consumer could
                // lawfully dereference it, which is why every `BytesPointerLength`
                // seat refused a carried byte source. The content is now copied
                // into the one existing lawful byte-span row while the host span is
                // still valid, so what crosses the boundary is region storage
                // rather than a borrowed address.
                //
                // The `(tag, class)` still comes from the sole disposition
                // authority; only the disposition's ANSWER for this variant moved.
                Lowered::ResponseBytes(span) => {
                    let (tag, class) = Self::carrier_handle_disposition(value)?;
                    self.emit_carrier_bytes_runtime_span(
                        builder,
                        tag,
                        class,
                        span.pointer(),
                        span.len(),
                    )
                }
                Lowered::BorrowedOption { .. } => Err(unsupported(
                    lowered_value_kind(value),
                    "the carrier producer does not yet emit borrowed ingress: an \
                     `InvocationBorrowed` handle is arena-owned and must clear \
                     `escape_check` before it may be written into a parent",
                )),

                // ── ⛔ REFUSED, not deferred — and structurally required here ──
                //
                // ⚠ Stated honestly: these arms are **unreachable in practice**,
                // because `boundary_transfer_admissibility` rejects the three
                // closure forms at the entry point and `boundary_disposition`
                // classifies the last two as `ProtocolOnly`. They are spelled
                // anyway because exhaustiveness is the mechanism that makes a 22nd
                // variant a compile error — ⛔ collapsing them into a `_` arm would
                // buy three lines and spend the whole closure property.
                Lowered::Closure { .. }
                | Lowered::DeclarationClosure { .. }
                | Lowered::ComputationalRecursorClosure { .. } => Err(unsupported(
                    lowered_value_kind(value),
                    "a closure has no durable lane and cannot cross the boundary; \
                     this arm is unreachable because the admissibility walk already \
                     refused the graph",
                )),
                Lowered::RecursiveBackedge | Lowered::Trap(_) => Err(unsupported(
                    lowered_value_kind(value),
                    "protocol machinery is never a source value at a boundary",
                )),
            }
        }
}

impl<'a> Lowering<'a> {
        /// The `(tag, class)` of a **handle**-represented value, read from the sole
        /// disposition authority (`§2h` ¶4).
        ///
        /// ⭐ **This is the typed boundary in front of the emission step**, and it
        /// is wildcard-free over [`BoundaryDisposition`] on purpose: a fifth
        /// disposition would break compilation here rather than silently taking
        /// whichever arm a `_` had swallowed.
        /// **`D7` — the aggregate allocation tag, taken from the planner record.**
        ///
        /// ⛔ The value-shape disposition answers a DIFFERENT question. It reports
        /// the lane a `Constructor`/`Record` takes *considered alone*, which is
        /// always the persistent one — the shape is persistable. Whether this
        /// particular aggregate may take it depends on its children's lifetimes,
        /// which the value in hand does not carry and this producer may not go
        /// looking for.
        ///
        /// ⚠ So this deliberately keeps the disposition's CLASS and replaces only
        /// its TAG. The class is a fact about the shape and the disposition is its
        /// authority; the lane is a fact about the meet and the planner is its.
        /// The record identity and class of one aggregate about to be allocated.
        ///
        /// ⭐ Returns the OCCURRENCE, not a lane. The lane is the checked wrapper's
        /// to read, so there is exactly one place a planned record becomes a
        /// `BoundaryTag` and exactly one place an event is recorded — a caller
        /// cannot obtain the lane and then allocate without leaving a pair.
        /// **`RT-DECL-CLOSURE-PORT` `D7` — the coordinate an aggregate's SCHEMA is
        /// resolved at, recovered from its own ownership record.**
        ///
        /// ⭐⭐ **Ownership was only half the defect.** Carrying the occurrence fixed
        /// which record an aggregate names; it did not fix where its constructor
        /// symbol, its field NAMES and its child positions are looked up, and those
        /// are keyed on a coordinate. A source record forwarded through a `Var` and
        /// handed to a call was still asking for its field names at the `Var` --
        /// measured, as `"static origin ... has no RecordFieldName atom at
        /// occurrence 0"`, on the released forwarded-record row.
        ///
        /// ⛔ The producer origin comes from the ownership record the template
        /// names, so it is recovered rather than transported. Nothing here searches
        /// for it, and no caller may pass one.
        ///
        /// ⚠ A compiler-synthesized aggregate has no source origin at all, so it
        /// keeps the transfer coordinate -- which for a synthesized subtree is the
        /// seat its whole tree is rooted at, and is the coordinate its children are
        /// reached under.
        ///
        /// ⛔ **There is no longer a `synthesized` flag here, and its removal is a
        /// measurement rather than a tidy-up.** It selected between two child
        /// coordinates that were genuinely DIFFERENT values -- 152 of 152 reached
        /// emissions took the two arms to different origins, 38 of them with a
        /// synthesized producer -- and the whole suite was green under either arm.
        /// The coordinate a child is transferred at is inert: a leaf never reads
        /// it, and an aggregate recovers its own from the record it carries. A
        /// decision whose two answers are indistinguishable is not a decision, and
        /// keeping it would have kept a `child_static_origin` lookup on the
        /// emission path that nothing consumed.
        fn aggregate_schema_origin(
            &self,
            occurrence: AggregateOccurrenceId,
            transfer: StaticOriginId,
        ) -> Result<StaticOriginId, CraneliftBackendError> {
            Ok(self
                .static_transition_plan
                .aggregate_record_view(occurrence)?
                .producer_origin()
                .unwrap_or(transfer))
        }
}

impl<'a> Lowering<'a> {
        pub(super) fn aggregate_carrier_authority(
            &self,
            origin: StaticOriginId,
            value: &Lowered,
            shape: PlannedAggregateShape,
        ) -> Result<(AggregateOccurrenceId, BoundaryClass), CraneliftBackendError> {
            let (_, class) = Self::carrier_handle_disposition(value)?;
            // `D7` -- the carried occurrence is the authority whenever the template
            // has one, because it names the PRODUCER. `origin` names wherever the
            // template happened to be transferred, which after nested producer
            // traversal is a `Let`, `Match`, `Call` or `Effect` occurrence that
            // never built an aggregate at all.
            // ⭐ **The PRODUCER's occurrence, whichever aggregate shape this is.**
            // Both variants now carry one, so this reads the same answer for a
            // record as for a constructor rather than having a shape-shaped hole
            // that fell through to the use coordinate.
            let occurrence = match value.source_aggregate_producer() {
                Some(occurrence) => occurrence,
                // ⚠ Reached only by an aggregate with NO producer — a value-domain
                // record or constructor built from a `RuntimeValue`, which has no
                // occurrence in the program. `source_aggregate_occurrence` then
                // fails closed unless the transfer coordinate genuinely is a
                // producer, which for those is the rig that built them.
                None => self
                    .static_transition_plan
                    .source_aggregate_occurrence(origin, shape)?,
            };
            Ok((occurrence, class))
        }
}

impl<'a> Lowering<'a> {
        fn carrier_handle_disposition(
            value: &Lowered,
        ) -> Result<(BoundaryTag, BoundaryClass), CraneliftBackendError> {
            match value.boundary_disposition() {
                BoundaryDisposition::RepresentedHandle { tag, class } => Ok((tag, class)),
                // ⚠ Not dead defensive code: it fires if a variant's disposition is
                // ever retuned from handle to immediate while this arm still
                // allocates. The disposition is the authority, so the producer must
                // fail rather than out-vote it.
                BoundaryDisposition::RepresentedImmediate { .. } => Err(unsupported(
                    lowered_value_kind(value),
                    "the producer would allocate a handle for a value the sole \
                     disposition authority represents as an immediate",
                )),
                BoundaryDisposition::ProtocolOnly { why }
                | BoundaryDisposition::FailClosedForbidden { why } => {
                    Err(unsupported(lowered_value_kind(value), why))
                }
            }
        }
}

impl<'a> Lowering<'a> {
        /// The tag of a **spill-free immediate**, read from the sole disposition
        /// authority.
        ///
        /// ⛔ **`spill: Some(_)` is still refused HERE, and that is not a leftover.**
        /// The refusal did not become unnecessary when the dispatch landed — it
        /// moved. A spillable payload has two possible representations, so a caller
        /// asking this question about one is asking a question with two answers;
        /// [`Self::carrier_spillable_disposition`] is the one that returns both, and
        /// this arm is what stops a spillable value reaching a bare `make_immediate`
        /// through an arm that forgot. ⚠ Deleting it would not reintroduce a
        /// truncation *today* — every spillable arm routes to the dispatch — which
        /// is exactly why it must stay: the next `RepresentedImmediate` variant is
        /// added by someone who copies the `Bool` arm.
        fn carrier_immediate_tag(value: &Lowered) -> Result<BoundaryTag, CraneliftBackendError> {
            match value.boundary_disposition() {
                BoundaryDisposition::RepresentedImmediate { tag, spill: None } => Ok(tag),
                BoundaryDisposition::RepresentedImmediate { spill: Some(_), .. } => Err(unsupported(
                    lowered_value_kind(value),
                    "a spillable immediate needs the runtime magnitude dispatch, \
                     not a single `make_immediate`",
                )),
                BoundaryDisposition::RepresentedHandle { .. } => Err(unsupported(
                    lowered_value_kind(value),
                    "the producer would mint an immediate for a value the sole \
                     disposition authority represents as a handle",
                )),
                BoundaryDisposition::ProtocolOnly { why }
                | BoundaryDisposition::FailClosedForbidden { why } => {
                    Err(unsupported(lowered_value_kind(value), why))
                }
            }
        }
}

impl<'a> Lowering<'a> {
        /// The `(immediate tag, spill class)` of a **spillable** immediate, read
        /// from the sole disposition authority (`§2h` ¶4).
        ///
        /// ⛔ **`spill: None` is refused, and the refusal is the mirror of the one
        /// on [`Self::carrier_immediate_tag`].** Between them the two readers
        /// partition `RepresentedImmediate` on the `spill` field, so neither the
        /// dispatch nor the single-`make_immediate` path can be reached for a value
        /// the authority classified the other way — and a value with **no** reader
        /// is a compile error at the `match` in `emit_carrier_transfer`, not a
        /// silent default.
        fn carrier_spillable_disposition(
            value: &Lowered,
        ) -> Result<(BoundaryTag, BoundaryClass), CraneliftBackendError> {
            match value.boundary_disposition() {
                BoundaryDisposition::RepresentedImmediate {
                    tag,
                    spill: Some(class),
                } => Ok((tag, class)),
                BoundaryDisposition::RepresentedImmediate { spill: None, .. } => Err(unsupported(
                    lowered_value_kind(value),
                    "the producer would emit a magnitude dispatch for a value the \
                     sole disposition authority declares cannot overflow its field",
                )),
                BoundaryDisposition::RepresentedHandle { .. } => Err(unsupported(
                    lowered_value_kind(value),
                    "the producer would mint an immediate for a value the sole \
                     disposition authority represents as a handle",
                )),
                BoundaryDisposition::ProtocolOnly { why }
                | BoundaryDisposition::FailClosedForbidden { why } => {
                    Err(unsupported(lowered_value_kind(value), why))
                }
            }
        }
}

impl<'a> Lowering<'a> {
        /// `alloc(arena, tag, class, field_count, out) -> status`.
        /// Open a fresh local event set for the body about to be emitted.
        ///
        /// ⛔ Called at the START of a body, before any allocation, so an
        /// allocation cannot be attributed to whichever body happened to be open
        /// last. A missing open is a loud failure at the first allocation, not a
        /// silently unattributed event.
        pub(super) fn open_aggregate_events(
            &mut self,
            function: cranelift_module::FuncId,
        ) -> Result<(), CraneliftBackendError> {
            self.defining_function_id = Some(function);
            match self.aggregate_allocations.as_mut() {
                Some(ledger) => ledger.open(function),
                // Outside the emission pass there is no relation to open into.
                None => Ok(()),
            }
        }
}

impl<'a> Lowering<'a> {
        /// Commit the open body's pairs, after finalization and verification and
        /// **before** `define_function`.
        /// ⭐ **`D7` — the effect-seat body close runs HERE, and this is the only
        /// place it can.** All four emitters reach this one boundary after
        /// finalization and verification and before `define_function`; a close
        /// installed in any single emitter would leave the other three ungated, and
        /// one installed at the whole-pass closeout would notice a discarded visit
        /// only after its body was already in the module.
        ///
        /// ⛔ `defining_function_id` is cleared only after BOTH closes succeed. It
        /// is the body the closes are asked about, so clearing it first would make
        /// the question unaskable at exactly the moment it is due.
        pub(super) fn commit_aggregate_events(&mut self) -> Result<(), CraneliftBackendError> {
            if let Some(function) = self.defining_function_id {
                if let Some(ledger) = self.host_effect_seats.as_mut() {
                    ledger.commit_body(function)?;
                }
            }
            let committed = match self.aggregate_allocations.as_mut() {
                Some(ledger) => ledger.commit(),
                None => Ok(()),
            };
            committed?;
            self.defining_function_id = None;
            Ok(())
        }
}

impl<'a> Lowering<'a> {
        /// **`D7` — ergonomic sugar over a `PlannedAggregate` request.**
        ///
        /// ⚠ **The governance no longer lives here.** The class/shape agreement,
        /// the lane derivation and the event-and-relation recording all happen at
        /// [`Self::emit_carrier_alloc`], because a check that lives in a wrapper
        /// holds only as long as every future caller remembers to reach for the
        /// wrapper rather than the raw helper beside it — an obligation nothing
        /// enforced and nothing measured. This function exists so the construction
        /// seats read the way they did.
        pub(super) fn emit_checked_aggregate_alloc(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            site: GovernedAllocationSite,
            occurrence: AggregateOccurrenceId,
            shape: PlannedAggregateShape,
            class: BoundaryClass,
            field_count: usize,
        ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
            let request = Self::governed_request(site, occurrence, shape);
            self.emit_carrier_alloc(builder, request, class, field_count)
        }
}

impl<'a> Lowering<'a> {
        /// The request one governed site hands the choke.
        ///
        /// ⛔ In a shipped compiler this is the `PlannedAggregate` construction and
        /// nothing else — the `#[cfg(test)]` arm compiles out entirely. It exists
        /// so a control can perturb ONE named site's request, which is the only way
        /// to show that site reaches the choke GOVERNED. Asserting the choke's
        /// refusal on a hand-built request proves the choke; it says nothing about
        /// whether the emitter's four real sites arrive there.
        fn governed_request(
            site: GovernedAllocationSite,
            occurrence: AggregateOccurrenceId,
            shape: PlannedAggregateShape,
        ) -> CarrierAllocationRequest {
            #[cfg(test)]
            if GOVERNED_ALLOCATION_MUTATION.with(std::cell::Cell::get)
                == GovernedAllocationMutation::Bypass(site)
            {
                governed_allocation_hit();
                return CarrierAllocationRequest::NonAggregate {
                    tag: BoundaryTag::PersistentGround,
                };
            }
            let _ = site;
            CarrierAllocationRequest::PlannedAggregate { occurrence, shape }
        }
}

impl<'a> Lowering<'a> {
        /// Enter one governed allocation into `E`, then into `R`.
        ///
        /// ⚠ Outside the emission pass there is no relation and no declared
        /// function, so there is no event to record. That is not a bypass: a bare
        /// rig emits no artifact, and the relation's laws are about an artifact's
        /// bodies. Inside the pass the `FuncId` is REQUIRED, and its absence is a
        /// loud failure rather than an unattributed event.
        fn record_governed_allocation(
            &mut self,
            result: cranelift_codegen::ir::Value,
            occurrence: AggregateOccurrenceId,
        ) -> Result<(), CraneliftBackendError> {
            let function = self.defining_function_id;
            let Some(ledger) = self.aggregate_allocations.as_mut() else {
                return Ok(());
            };
            let function = function.ok_or_else(|| {
                backend_module(
                    "a governed aggregate allocation ran inside the emission pass with no declared \
                     function open, so its event has no FuncId to be scoped by"
                        .to_string(),
                )
            })?;
            // ⛔ Event evidence FIRST, then the relation pair. `E` is what
            // allocated; deriving it from the relation would make `dom(R) = E` true
            // by construction and the law unstateable.
            ledger.record_event(function, result)?;
            ledger.relate(function, result, occurrence)
        }
}

impl<'a> Lowering<'a> {
        /// **`D7` — THE choke point. Every carrier allocation in the backend is
        /// this call, and the REQUEST decides whether it is governed.**
        ///
        /// ⭐ The two arms are not two spellings of one thing:
        ///
        /// | request | class | lane | evidence |
        /// |---|---|---|---|
        /// | `NonAggregate` | must NOT be `Constructor`/`Record` | the caller's tag | none |
        /// | `PlannedAggregate` | must MATCH the shape | `aggregate_allocation_at` | `E` then `R` |
        ///
        /// ⛔ Both refusals happen **before the raw `alloc` call**, so a bypass
        /// cannot allocate and then fail: the arena's allocation count does not
        /// move and no artifact is ever defined. Refusing afterwards would leave
        /// the very half-governed state the request exists to make unspellable.
        ///
        /// ⛔ The event, by contrast, is recorded AFTER the raw allocation,
        /// because the result `Value` is half the event's identity and does not
        /// exist before it. That ordering is what makes "one allocation, one pair"
        /// checkable at all.
        pub(super) fn emit_carrier_alloc(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            request: CarrierAllocationRequest,
            class: BoundaryClass,
            field_count: usize,
        ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
            // ── The request is settled before a single instruction is emitted ──
            let tag = match request {
                CarrierAllocationRequest::NonAggregate { tag } => {
                    if matches!(class, BoundaryClass::Constructor | BoundaryClass::Record) {
                        return Err(backend_module(format!(
                            "a {class:?} carrier was allocated as non-aggregate, so it would name no \
                             planned ownership record and enter neither E nor R"
                        )));
                    }
                    tag
                }
                CarrierAllocationRequest::PlannedAggregate { occurrence, shape } => {
                    let expected = CarrierAllocationRequest::aggregate_class(shape);
                    if class != expected {
                        return Err(backend_module(format!(
                            "a planned {shape:?} aggregate was allocated at class {class:?} rather \
                             than {expected:?}"
                        )));
                    }
                    // ⛔ The LANE is read from the record, never from the caller.
                    match self
                        .static_transition_plan
                        .aggregate_allocation_at(occurrence, shape)?
                    {
                        PlannedAggregateAllocation::PersistentGround => BoundaryTag::PersistentGround,
                        PlannedAggregateAllocation::InvocationAggregate => {
                            BoundaryTag::InvocationAggregate
                        }
                    }
                }
            };
            let refs = self.carrier_refs()?;
            let arena = self.carrier_arena()?;
            let pointer_type = builder.func.dfg.value_type(arena);
            let (slot, out) = Self::carrier_out_slot(builder, pointer_type);
            let tag = builder.ins().iconst(types::I64, i64::from(tag as u8));
            let class = builder.ins().iconst(types::I64, class as i64);
            let count = builder.ins().iconst(
                types::I64,
                i64::try_from(field_count).map_err(|_| {
                    unsupported(
                        "BoundaryCarrier",
                        "a transferred aggregate has more fields than the ABI can name",
                    )
                })?,
            );
            #[cfg(test)]
            CARRIER_RAW_ALLOCATIONS.with(|n| n.set(n.get().saturating_add(1)));
            let call = builder
                .ins()
                .call(refs.alloc, &[arena, tag, class, count, out]);
            Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
            let word = CarriedBoundaryWord {
                word: builder.ins().stack_load(types::I64, slot, 0),
            };
            if let CarrierAllocationRequest::PlannedAggregate { occurrence, .. } = request {
                self.record_governed_allocation(word.word, occurrence)?;
            }
            Ok(word)
        }
}

impl<'a> Lowering<'a> {
        /// ⭐⭐ **THE MAGNITUDE DISPATCH** — the producer arm for a value whose
        /// disposition carries `spill: Some(_)` (`RT-FNSPLIT-B2F` `D9`; Architect
        /// ruling on the corrected producer partition).
        ///
        /// ⛔⛔ **The predicate is READ, never re-derived.**
        /// `ken_boundary_make_immediate_local` already tests the payload against the
        /// one `BOUNDARY_IMMEDIATE_DOMAIN` table and already reports the answer
        /// distinguishably — its own source says the errors are kept distinct *"so a
        /// control can tell which rule refused without reading the payload back"*.
        /// ⇒ A shift-and-compare here would be a **second answer to a question that
        /// already has one**, free to drift from the table silently. That is the
        /// second-representation-authority defect one layer down, and it is the same
        /// objection [`Self::carrier_identity_immediate`] raises about `pack_identity`.
        ///
        /// ⭐ **Three outcomes, ⛔ not two:**
        ///
        /// | status | outcome |
        /// |---|---|
        /// | `BOUNDARY_OK` | the immediate word `make_immediate` wrote |
        /// | `BOUNDARY_ERR_BOUNDS` | **the spill** — a handle of the declared class |
        /// | anything else | fail closed, via the same `require_i64` every other helper status takes |
        ///
        /// ⛔ Collapsing *"anything else"* into the spill would turn a shape, tag or
        /// capacity error into a **silent allocation** of a value nobody asked for.
        /// The third outcome is spelled as `require_i64(status, BOUNDARY_ERR_BOUNDS)`
        /// on the not-OK edge precisely so it cannot be written as a two-way branch
        /// by accident.
        ///
        /// ⭐ **`AC-2` — this is emitted code reading a RUNTIME value.** Nothing here
        /// inspects a JIT-time constant to choose a layout: one compiled body takes
        /// either arm depending on the payload it is handed. That is why the
        /// partition is a property of the value rather than of the compilation.
        ///
        /// ⛔⛔ **THIS ARM IS ONLY SOUND FOR A `Small`-MARKED PAYLOAD, and it is
        /// [`Self::emit_carrier_native_int`]'s job to guarantee that.** The payload
        /// of a `NativeIntV1` pair means different things under different markers —
        /// a `Big` payload is a **slot identity**, and asking `make_immediate` a
        /// magnitude question about a slot number is answered `OK` for a low slot.
        /// ⇒ Calling this directly on an unpartitioned `Lowered::Int` payload is a
        /// **silent corruption**, not a fail-closed gap.
        ///
        /// ⚠ An earlier revision of this comment claimed such a value would be
        /// refused by `store_int_tag`'s owner guard. **It never reaches that guard**
        /// — corrected under the Architect's ruling `evt_79xcj70p0qxjj`.
        ///
        /// **MEASURED:** the emitted body branches on `make_immediate`'s status and
        /// builds a `BoundaryClass::Int` handle on the bounds edge.
        /// **CLAIMED:** a `Small`-marked spillable value crosses without truncation.
        /// **THE GAP:** ⚠ **the marker partition is the caller's**, so this
        /// function's soundness is conditional on it. The non-`Int` spillables reach
        /// here directly because their payload *is* their magnitude with no pair and
        /// no second reading — see [`Self::carrier_small_marker`].
        ///
        /// ⚠ **A second residual, review-caught rather than mechanically detected:**
        /// swapping the status branch below for a hand-written magnitude test still
        /// round-trips every value, so no test in this suite would redden. ⛔ Its
        /// absence from a green run is not evidence about it.
        fn emit_carrier_spillable_immediate(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            tag: BoundaryTag,
            spill: BoundaryClass,
            payload: cranelift_codegen::ir::Value,
            native_marker: cranelift_codegen::ir::Value,
        ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
            let refs = self.carrier_refs()?;
            let arena = self.carrier_arena()?;
            let pointer_type = builder.func.dfg.value_type(arena);
            let (slot, out) = Self::carrier_out_slot(builder, pointer_type);
            let immediate_tag = builder.ins().iconst(types::I64, i64::from(tag as u8));
            let call = builder
                .ins()
                .call(refs.make_immediate, &[immediate_tag, payload, out]);
            let status = builder.inst_results(call)[0];

            // ⛔ The ONE comparison this function makes, and it is against a status,
            // not against a magnitude.
            let fits = builder.ins().icmp_imm(
                cranelift_codegen::ir::condcodes::IntCC::Equal,
                status,
                BOUNDARY_OK,
            );
            let immediate_block = builder.create_block();
            let spill_block = builder.create_block();
            let join = builder.create_block();
            builder.append_block_param(join, types::I64);
            builder
                .ins()
                .brif(fits, immediate_block, &[], spill_block, &[]);

            builder.switch_to_block(immediate_block);
            let word = builder.ins().stack_load(types::I64, slot, 0);
            builder.ins().jump(join, &[word.into()]);

            builder.switch_to_block(spill_block);
            // ⭐ The third outcome, spelled as a requirement rather than an `else`:
            // reaching here means the status was not `OK`, and anything that is also
            // not `ERR_BOUNDS` leaves the function fail-closed right here.
            Self::require_i64(builder, status, BOUNDARY_ERR_BOUNDS);
            // ⚠ `require_i64` splits the block; from here the builder is in its
            // `valid` successor, which is where the allocation belongs.
            let spilled = self.emit_carrier_alloc(
                builder,
                CarrierAllocationRequest::NonAggregate {
                    tag: BoundaryTag::PersistentGround,
                },
                spill,
                0,
            )?;
            let store = builder
                .ins()
                .call(refs.store_scalar, &[arena, spilled.word, payload]);
            Self::require_i64(builder, builder.inst_results(store)[0], BOUNDARY_OK);
            let mark = builder
                .ins()
                .call(refs.store_int_tag, &[arena, spilled.word, native_marker]);
            Self::require_i64(builder, builder.inst_results(mark)[0], BOUNDARY_OK);
            builder.ins().jump(join, &[spilled.word.into()]);

            builder.switch_to_block(join);
            Ok(CarriedBoundaryWord {
                word: builder.block_params(join)[0],
            })
        }
}

impl<'a> Lowering<'a> {
        /// ⭐⭐ **THE `NativeIntV1` MARKER PARTITION** — the entry point for
        /// `Lowered::Int`, and the thing that must happen **before** any magnitude
        /// question is asked (Architect ruling, `evt_79xcj70p0qxjj`).
        ///
        /// ⛔⛔ **Why the marker comes first, and why the obvious order is a silent
        /// corruption rather than a residual.** `Lowered::Int`'s `value` is the
        /// **payload half of a `NativeIntV1` pair**, and what that word *means*
        /// depends on the marker: for `Small` it is the magnitude; for `Big` it is a
        /// **slot identity in the invocation's native arena**, and slots begin at
        /// `1`. ⇒ Calling `make_immediate` on a `Big` payload asks a magnitude
        /// question about a slot number — and a low slot **satisfies** the immediate
        /// domain, so the value crosses on the apparent-success arm encoded as the
        /// integer `1`. ⚠ Not a fail-closed gap: a wrong answer that looks like a
        /// right one.
        ///
        /// ⚠ **This corrects a residual I previously stated as fail-closed.** The
        /// earlier claim was that a `Big` would be refused by `store_int_tag`'s
        /// owner guard. It never reaches that guard.
        ///
        /// ⭐ **The branch is a read of the canonical transport tag, ⛔ not a
        /// sibling magnitude predicate**, so it does not weaken the ban on
        /// re-deriving the immediate-domain test: within the `Small` arm the ruled
        /// status-derived dispatch is unchanged.
        ///
        /// | marker | path |
        /// |---|---|
        /// | `NATIVE_INT_SMALL_TAG_V1` | the payload **is** the magnitude → [`Self::emit_carrier_spillable_immediate`] |
        /// | `NATIVE_INT_BIG_TAG_V1` | the payload is a slot → resolve, then an **owned deep copy** into the persistent region |
        /// | anything else | ⛔ fail closed |
        fn emit_carrier_native_int(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            tag: BoundaryTag,
            spill: BoundaryClass,
            payload: cranelift_codegen::ir::Value,
            marker: cranelift_codegen::ir::Value,
        ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
            let small = builder.ins().icmp_imm(
                cranelift_codegen::ir::condcodes::IntCC::Equal,
                marker,
                i64::try_from(crate::NATIVE_INT_SMALL_TAG_V1).map_err(|_| {
                    unsupported(
                        "BoundaryCarrier",
                        "the native `Small` marker is not an ABI word",
                    )
                })?,
            );
            let small_block = builder.create_block();
            let wide_block = builder.create_block();
            let join = builder.create_block();
            builder.append_block_param(join, types::I64);
            builder.ins().brif(small, small_block, &[], wide_block, &[]);

            builder.switch_to_block(small_block);
            let immediate =
                self.emit_carrier_spillable_immediate(builder, tag, spill, payload, marker)?;
            builder.ins().jump(join, &[immediate.word.into()]);

            builder.switch_to_block(wide_block);
            let wide = self.emit_carrier_region_limbed_int(builder, spill, payload, marker)?;
            builder.ins().jump(join, &[wide.word.into()]);

            builder.switch_to_block(join);
            Ok(CarriedBoundaryWord {
                word: builder.block_params(join)[0],
            })
        }
}

impl<'a> Lowering<'a> {
        /// ⭐ **The owned deep copy** — a region-limbed `Int` crossing into the
        /// persistent region (Architect ruling, `evt_79xcj70p0qxjj`).
        ///
        /// ⛔ **No represented-unavailable lane, and no new error identity.** A valid
        /// wide `Int` crosses **successfully**; `ERR_ESCAPE` is not an admissible
        /// terminal result for one. The copy is *owned*, so nothing borrows the
        /// invocation arena past its extent and the escape question does not arise.
        ///
        /// ⭐ **The decode is `ken_native_int_resolve_local`'s, never ours.** It
        /// already yields canonical `sign`, `len` and `limbs` from the one native
        /// representation. ⛔ Deriving them here would be a second exact-integer
        /// decoder beside the first — the proliferation `docs/PRINCIPLES.md` forbids
        /// — and `boundary_value_clif`'s own int readers make the identical choice.
        ///
        /// ⛔ **The order is load-bearing and is the established wide-`Int`
        /// producer's:** allocate → region marker → claim → copy → **seal**. The
        /// marker written is [`BOUNDARY_INT_REGION_LIMBS`], ⛔ never the native
        /// `Big` marker: that marker names a slot in storage that dies with the
        /// invocation, which is exactly what `BOUNDARY_INT_MARKER_OWNER` refuses on
        /// a persistent node. And until `seal_int` succeeds **the node denotes
        /// nothing**, so the seal is the last step rather than an optional check.
        ///
        /// ⚠ The limb loop is over a **runtime** length: nothing about the magnitude
        /// is known when this body is compiled, which is `AC-2` at the wide arm.
        fn emit_carrier_region_limbed_int(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            spill: BoundaryClass,
            payload: cranelift_codegen::ir::Value,
            marker: cranelift_codegen::ir::Value,
        ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
            // ⛔ Any marker that is not `Big` fails closed HERE — the closed set is
            // `{Small, Big}` and `Small` was taken by the caller's branch.
            Self::require_i64(
                builder,
                marker,
                i64::try_from(crate::NATIVE_INT_BIG_TAG_V1).map_err(|_| {
                    unsupported(
                        "BoundaryCarrier",
                        "the native `Big` marker is not an ABI word",
                    )
                })?,
            );

            let refs = self.carrier_refs()?;
            let arena = self.carrier_arena()?;
            let decoder = self.function_local.native_int_resolve.ok_or_else(|| {
                unsupported(
                    "BoundaryCarrier",
                    "this generated function has no exact-Int decoder",
                )
            })?;
            let pointer_type = builder.func.dfg.value_type(arena);

            // ⭐⭐ **The native arena comes from the BOUNDARY arena's own binding
            // slot, and that choice is intrinsic rather than convenient.** The node
            // being built is read back by `int_sign` / `int_len` / `int_limb`, and
            // each of those decodes with exactly `load(arena, ARENA_NATIVE_INT)`.
            // ⇒ Reading the same slot makes producer and consumer agree **by
            // construction**; taking the pointer from anywhere else would let the
            // two decode a pair against different arenas, which is the drift the
            // one-decoder rule exists to prevent.
            //
            // ⛔ Not native-arena layout: this is the boundary arena's binding
            // field, read exactly as `boundary_value_clif` reads it, and the value
            // is handed straight to the decoder rather than walked.
            let native_arena = builder.ins().load(
                pointer_type,
                MemFlags::trusted(),
                arena,
                crate::boundary_value::ARENA_NATIVE_INT,
            );
            Self::require_nonzero(builder, native_arena);

            // The decoder's `{sign, len, limbs, small}` view.
            let view_slot =
                builder.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 32, 3));
            let view = builder.ins().stack_addr(pointer_type, view_slot, 0);
            let decoded = builder
                .ins()
                .call(decoder, &[native_arena, marker, payload, view]);
            Self::require_i64(builder, builder.inst_results(decoded)[0], 0);
            let sign = builder.ins().load(
                types::I64,
                MemFlags::trusted(),
                view,
                crate::native_int_clif::VIEW_SIGN,
            );
            let length = builder.ins().load(
                types::I64,
                MemFlags::trusted(),
                view,
                crate::native_int_clif::VIEW_LEN,
            );
            let source = builder.ins().load(
                pointer_type,
                MemFlags::trusted(),
                view,
                crate::native_int_clif::VIEW_LIMBS,
            );

            // allocate → region marker → claim → copy → seal.
            let word = self.emit_carrier_alloc(
                builder,
                CarrierAllocationRequest::NonAggregate {
                    tag: BoundaryTag::PersistentGround,
                },
                spill,
                0,
            )?;
            let region = builder.ins().iconst(
                types::I64,
                i64::try_from(crate::boundary_value::BOUNDARY_INT_REGION_LIMBS).map_err(|_| {
                    unsupported(
                        "BoundaryCarrier",
                        "the region-limbs marker is not an ABI word",
                    )
                })?,
            );
            let marked = builder
                .ins()
                .call(refs.store_int_tag, &[arena, word.word, region]);
            Self::require_i64(builder, builder.inst_results(marked)[0], BOUNDARY_OK);
            let (_span_slot, span) = Self::carrier_out_slot(builder, pointer_type);
            let claim = builder.ins().call(
                refs.store_int_limbs,
                &[arena, word.word, sign, length, span],
            );
            Self::require_i64(builder, builder.inst_results(claim)[0], BOUNDARY_OK);

            let head = builder.create_block();
            builder.append_block_param(head, types::I64);
            let body = builder.create_block();
            let done = builder.create_block();
            let zero = builder.ins().iconst(types::I64, 0);
            builder.ins().jump(head, &[zero.into()]);

            builder.switch_to_block(head);
            let index = builder.block_params(head)[0];
            let more = builder.ins().icmp(
                cranelift_codegen::ir::condcodes::IntCC::UnsignedLessThan,
                index,
                length,
            );
            builder.ins().brif(more, body, &[], done, &[]);

            builder.switch_to_block(body);
            let offset = builder.ins().imul_imm(index, 8);
            let address = builder.ins().iadd(source, offset);
            let limb = builder
                .ins()
                .load(types::I64, MemFlags::trusted(), address, 0);
            let write = builder
                .ins()
                .call(refs.store_int_limb, &[arena, word.word, index, limb]);
            Self::require_i64(builder, builder.inst_results(write)[0], BOUNDARY_OK);
            // ⚠ `require_i64` split the block, so the back edge is emitted from the
            // block the builder is in NOW, not from `body`.
            let next = builder.ins().iadd_imm(index, 1);
            builder.ins().jump(head, &[next.into()]);

            builder.switch_to_block(done);
            let sealed = builder.ins().call(refs.seal_int, &[arena, word.word]);
            Self::require_i64(builder, builder.inst_results(sealed)[0], BOUNDARY_OK);
            Ok(word)
        }
}

impl<'a> Lowering<'a> {
        /// ⭐ **The byte-bodied handle producer** — the `String` / `Bytes` arm of
        /// `RT-FNSPLIT-B2F` `D9`.
        ///
        /// ⭐ **ONE body, driven with the class the disposition supplies.** ⛔ Not
        /// two emitters and ⛔ not a `Bytes` emitter a `String` "shares every code
        /// path but the class" with — the class is exactly the axis `store_bytes_len`
        /// and `store_byte` guard on, so it is the one path the two do **not** share.
        /// `boundary_value_clif`'s own history records a `class_guard` narrowed to
        /// `Bytes` alone staying green because no test had ever asked emitted code to
        /// *build* a `String`.
        ///
        /// ⭐ **Claim-then-fill.** `store_bytes_len` reserves the whole span before a
        /// byte is written, so a length the region cannot satisfy fails **before any
        /// address is formed** rather than part-way through the content.
        ///
        /// **MEASURED:** the emitted body allocates a node of the declared class,
        /// claims a span of the literal's length, and writes every byte of it.
        /// **CLAIMED:** a byte-bodied literal crosses the boundary with its content.
        /// **THE GAP:** ⚠ the content is a **compile-time literal**, so this arm says
        /// nothing about a runtime source. ⛔ Do not read it as coverage of the
        /// byte-bodied class in general.
        ///
        /// ⚠ **The former wording of that gap — *"there is no `Lowered` variant
        /// that carries one"* — is FALSE since `RT-CARRIER-BYTESPAN-OBSERVE` `D2`,
        /// and it was the sentence a reader would have built on.**
        /// [`Lowered::ResponseBytes`] carries a runtime `{pointer, len}` and is
        /// copied by [`Self::emit_carrier_bytes_runtime_span`], which is the
        /// separate control the old wording asked for. The gap this arm still has
        /// is real and narrower: **it is the LITERAL arm, and it covers literals.**
        fn emit_carrier_bytes(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            tag: BoundaryTag,
            class: BoundaryClass,
            content: &[u8],
        ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
            let refs = self.carrier_refs()?;
            let arena = self.carrier_arena()?;
            let pointer_type = builder.func.dfg.value_type(arena);
            let word = self.emit_carrier_alloc(
                builder,
                CarrierAllocationRequest::NonAggregate { tag },
                class,
                0,
            )?;
            let (_span_slot, span) = Self::carrier_out_slot(builder, pointer_type);
            let length = builder.ins().iconst(
                types::I64,
                i64::try_from(content.len()).map_err(|_| {
                    unsupported(
                        "BoundaryCarrier",
                        "a transferred literal is longer than the ABI can name",
                    )
                })?,
            );
            let claim = builder
                .ins()
                .call(refs.store_bytes_len, &[arena, word.word, length, span]);
            Self::require_i64(builder, builder.inst_results(claim)[0], BOUNDARY_OK);
            for (index, byte) in content.iter().enumerate() {
                let position = Self::carrier_position_immediate(builder, index)?;
                let byte = builder.ins().iconst(types::I64, i64::from(*byte));
                let write = builder
                    .ins()
                    .call(refs.store_byte, &[arena, word.word, position, byte]);
                Self::require_i64(builder, builder.inst_results(write)[0], BOUNDARY_OK);
            }
            Ok(word)
        }
}

impl<'a> Lowering<'a> {
        /// **`RT-CARRIER-BYTESPAN-OBSERVE` `D2` — the RUNTIME-SPAN analogue of
        /// [`Self::emit_carrier_bytes`]**, under Architect `dec_6qmstfn6tjqdt`.
        ///
        /// Same claim-then-fill shape and the same two guarded helpers; the only
        /// difference is where the content comes from. The literal arm unrolls over
        /// a `&[u8]` the compiler holds; this one emits a loop that copies `len`
        /// bytes from a runtime `pointer` **while the host span is still valid**,
        /// which is what makes the result outlive the invocation.
        ///
        /// ⭐ **Normalization by COPY, never a retag.** The word this returns names
        /// region storage the copy filled, not the caller's buffer. That is the
        /// whole reason the referent owner may be `PersistentStore`: nothing here
        /// republishes the host pointer, so `AC-7`'s escape rule is untouched.
        ///
        /// ⛔ **Only an EXPLICITLY bytes-typed source may reach here.** The extent
        /// is the caller's typed `len`, never a length this ABI inferred from an
        /// opaque word — dereferencing a `BorrowedOpaque` scalar is the
        /// confused-deputy hole the node's Banned section names, and it is refused
        /// one layer up by the disposition rather than here.
        ///
        /// **Every failure is closed BEFORE publication.** `store_bytes_len`
        /// reserves the whole span first, so a length the region cannot satisfy
        /// fails before any address is formed; and each `store_byte` is bounds-
        /// checked against the length just recorded. Every status goes through
        /// [`Self::require_i64`], which returns failure from the emitted function,
        /// so a partially-filled node is never adopted and therefore never
        /// published — store adoption is the identity boundary, and it is
        /// downstream of every check here.
        ///
        /// ⚠ **A negative or absurd `len` fails CLOSED rather than looping.**
        /// `store_bytes_len` compares UNSIGNED against the data capacity, so a
        /// negative length reads as an enormous unsigned one and is refused by the
        /// capacity guard before the loop is reached; the loop's own bound is the
        /// same unsigned comparison. **Zero length is a legal span**: the capacity
        /// check admits it and the loop body simply never runs.
        fn emit_carrier_bytes_runtime_span(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            tag: BoundaryTag,
            class: BoundaryClass,
            pointer: cranelift_codegen::ir::Value,
            len: cranelift_codegen::ir::Value,
        ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
            let refs = self.carrier_refs()?;
            let arena = self.carrier_arena()?;
            let pointer_type = builder.func.dfg.value_type(arena);
            let word = self.emit_carrier_alloc(
                builder,
                CarrierAllocationRequest::NonAggregate { tag },
                class,
                0,
            )?;
            let (_span_slot, span) = Self::carrier_out_slot(builder, pointer_type);
            let claim = builder
                .ins()
                .call(refs.store_bytes_len, &[arena, word.word, len, span]);
            Self::require_i64(builder, builder.inst_results(claim)[0], BOUNDARY_OK);

            let head = builder.create_block();
            builder.append_block_param(head, types::I64);
            let body = builder.create_block();
            let done = builder.create_block();
            let zero = builder.ins().iconst(types::I64, 0);
            builder.ins().jump(head, &[zero.into()]);

            builder.switch_to_block(head);
            let index = builder.block_params(head)[0];
            let more = builder.ins().icmp(
                cranelift_codegen::ir::condcodes::IntCC::UnsignedLessThan,
                index,
                len,
            );
            builder.ins().brif(more, body, &[], done, &[]);

            builder.switch_to_block(body);
            let address = builder.ins().iadd(pointer, index);
            let byte = builder
                .ins()
                .load(types::I8, MemFlags::trusted(), address, 0);
            let widened = builder.ins().uextend(types::I64, byte);
            let write = builder
                .ins()
                .call(refs.store_byte, &[arena, word.word, index, widened]);
            Self::require_i64(builder, builder.inst_results(write)[0], BOUNDARY_OK);
            // ⚠ `require_i64` split the block, so the back edge is emitted from the
            // block the builder is in NOW, not from `body`.
            let next = builder.ins().iadd_imm(index, 1);
            builder.ins().jump(head, &[next.into()]);

            builder.switch_to_block(done);
            Ok(word)
        }
}

impl<'a> Lowering<'a> {
        /// `store_tag_id(arena, word, tag_id) -> status`.
        pub(super) fn emit_carrier_store_tag_id(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            target: CarriedBoundaryWord,
            identity: u64,
        ) -> Result<(), CraneliftBackendError> {
            let refs = self.carrier_refs()?;
            let arena = self.carrier_arena()?;
            let identity = Self::carrier_identity_immediate(builder, identity);
            let call = builder
                .ins()
                .call(refs.store_tag_id, &[arena, target.word, identity]);
            Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
            Ok(())
        }
}

impl<'a> Lowering<'a> {
        /// `store_scalar(arena, word, value) -> status`.
        fn emit_carrier_store_scalar(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            target: CarriedBoundaryWord,
            payload: cranelift_codegen::ir::Value,
        ) -> Result<(), CraneliftBackendError> {
            let refs = self.carrier_refs()?;
            let arena = self.carrier_arena()?;
            let call = builder
                .ins()
                .call(refs.store_scalar, &[arena, target.word, payload]);
            Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
            Ok(())
        }
}

impl<'a> Lowering<'a> {
        fn emit_carrier_dynamic_constructor(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            origin: StaticOriginId,
            dynamic: &DynamicConstructorV1,
        ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
            validate_dynamic_constructor_alternatives(
                dynamic
                    .alternatives
                    .iter()
                    .map(|alternative| (alternative.tag, alternative.constructor.as_str())),
            )?;
            let merge = builder.create_block();
            builder.append_block_param(merge, types::I64);

            for alternative in &dynamic.alternatives {
                let selected = builder.create_block();
                let next = builder.create_block();
                let matches = builder.ins().icmp_imm(
                    cranelift_codegen::ir::condcodes::IntCC::Equal,
                    dynamic.discriminator,
                    alternative.tag,
                );
                builder.ins().brif(matches, selected, &[], next, &[]);

                builder.switch_to_block(selected);
                let disposition = Lowered::DynamicConstructor(DynamicConstructorV1 {
                    discriminator: dynamic.discriminator,
                    alternatives: vec![alternative.clone()],
                });
                // ⭐ **`D7` — the selected alternative's lane comes from ITS OWN
                // planner record.** The set is not an allocation; this is. The
                // value-shape disposition answers `PersistentGround` for every
                // `DynamicConstructor` because the shape is persistable, which is
                // the same unproven persistent lane the fixed-constructor arm
                // already stopped taking. Whether this alternative may take it
                // depends on its children's lifetimes, which the value in hand does
                // not carry.
                //
                // ⚠ The CLASS still comes from the disposition and only the TAG is
                // replaced — the class is a fact about the shape and the
                // disposition is its authority; the lane is a fact about the meet
                // and the planner is its.
                let (_, class) = Self::carrier_handle_disposition(&disposition)?;
                let occurrence = match alternative.occurrence {
                    Some(occurrence) => occurrence,
                    // ⛔ A refusal, not a default. An alternative with no carried
                    // occurrence is one whose lifetime meet was never taken, and
                    // answering `PersistentGround` for it would reinstate exactly
                    // the unproven lane the record exists to replace — silently,
                    // and only for the alternatives the population happened to
                    // miss.
                    None => {
                        return Err(unsupported(
                            "DynamicConstructor",
                            format!(
                                "the selected alternative {} carries no planned occurrence, so its \
                                 allocation has no lifetime meet",
                                alternative.constructor
                            ),
                        ));
                    }
                };
                let word = self.emit_checked_aggregate_alloc(
                    builder,
                    GovernedAllocationSite::DynamicAlternative,
                    occurrence,
                    PlannedAggregateShape::Constructor,
                    class,
                    alternative.fields.len(),
                )?;
                self.emit_carrier_store_tag_id(builder, word, alternative.identity.tag_abi_word()?)?;
                for (position, field) in alternative.fields.iter().enumerate() {
                    let field = self.emit_carrier_transfer(builder, origin, field)?;
                    self.emit_carrier_store_field(builder, word, position, field)?;
                }
                builder.ins().jump(merge, &[word.word.into()]);
                builder.switch_to_block(next);
            }

            let malformed = builder
                .ins()
                .iconst(types::I64, MALFORMED_DYNAMIC_CONSTRUCTOR_STATUS);
            builder.ins().return_(&[malformed]);
            builder.switch_to_block(merge);
            Ok(CarriedBoundaryWord {
                word: builder.block_params(merge)[0],
            })
        }
}

impl<'a> Lowering<'a> {
        /// `store_field(arena, word, index, child) -> status`.
        pub(super) fn emit_carrier_store_field(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            target: CarriedBoundaryWord,
            position: usize,
            child: CarriedBoundaryWord,
        ) -> Result<(), CraneliftBackendError> {
            let refs = self.carrier_refs()?;
            let arena = self.carrier_arena()?;
            let index = Self::carrier_position_immediate(builder, position)?;
            let call = builder
                .ins()
                .call(refs.store_field, &[arena, target.word, index, child.word]);
            Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
            Ok(())
        }
}

impl<'a> Lowering<'a> {
        /// `store_name(arena, word, index, name_id) -> status`.
        fn emit_carrier_store_name(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            target: CarriedBoundaryWord,
            position: usize,
            identity: u64,
        ) -> Result<(), CraneliftBackendError> {
            let refs = self.carrier_refs()?;
            let arena = self.carrier_arena()?;
            let index = Self::carrier_position_immediate(builder, position)?;
            let identity = Self::carrier_identity_immediate(builder, identity);
            let call = builder
                .ins()
                .call(refs.store_name, &[arena, target.word, index, identity]);
            Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
            Ok(())
        }
}

impl<'a> Lowering<'a> {
        /// `record_field(arena, word, name_id, out) -> status` — `Project` by
        /// **artifact-static field identity**.
        ///
        /// ⭐ The `name_id` is the same word the producer wrote with `store_name`,
        /// from the same `D1` authority — which is exactly why `AC-C5`'s reordered
        /// record still projects correctly: the lookup is keyed on the interned
        /// name, ⛔ never on declaration position.
        ///
        /// ⭐ Result stays carried, for the reason spelled out on
        /// [`Self::emit_carrier_field`].
        pub(super) fn emit_carrier_record_field(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            target: CarriedBoundaryWord,
            identity: u64,
        ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
            let refs = self.carrier_refs()?;
            let arena = self.carrier_arena()?;
            let pointer_type = builder.func.dfg.value_type(arena);
            let (slot, out) = Self::carrier_out_slot(builder, pointer_type);
            let identity = Self::carrier_identity_immediate(builder, identity);
            let call = builder
                .ins()
                .call(refs.record_field, &[arena, target.word, identity, out]);
            Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
            Ok(CarriedBoundaryWord {
                word: builder.ins().stack_load(types::I64, slot, 0),
            })
        }
}

impl<'a> Lowering<'a> {
        /// Project the seat's operand at `index` into a site-bound argument.
        ///
        /// ⭐ The only way to build a [`SynthesizedArgument::SiteOperand`] in
        /// ordinary lowering, and it reads the visit's CLAIMED seat rather than
        /// accepting a value — so the emitter states *which operand* it means and
        /// cannot hand over a substitute by mistake.
        ///
        /// ⛔ **The sole site-operand projection, driven by an exact declared
        /// `SiteOperand(index)` use.** It used to read a dense `Vec<Lowered>` that
        /// the caller built by demanding a specialized template for *every*
        /// argument the operation has. That vector was the prohibited pre-operation
        /// bulk conversion relocated after dispatch: operation knowledge made the
        /// diagnostic narrower but did not authorize reading an unrelated seat, so
        /// `BufferAllocate`'s capacity — which no synthesized node uses — was
        /// re-read as a template here after its own arm had already consumed it,
        /// and a carried capacity was refused by a consumer that never wanted it.
        /// Projecting only the seat a declared child names is what makes the route
        /// exact-use-driven rather than dense. A specialized operand is cloned
        /// opaquely; a carried byte-span operand is observed through the emitted
        /// helper and becomes a runtime-valued `Lowered::ResponseBytes`.
        pub(super) fn site_operand_argument(
            &mut self,
            builder: &mut FunctionBuilder<'_>,
            seat: StaticOriginId,
            index: u32,
            seats: &ClaimedEffectSeats<'_>,
        ) -> Result<SynthesizedArgument, CraneliftBackendError> {
            let (record, operand) = seats.operand(EffectSeatSlot::Argument(index))?;
            let (mut value, source) = match operand {
                LoweringOperand::Specialized(value) => (value.clone(), SiteOperandSource::Specialized),
                LoweringOperand::Carried(word) => {
                    let (pointer, len, outcome) =
                        self.observe_carried_bytes_span(builder, record, *word)?;
                    let valid = builder.ins().icmp_imm(
                        cranelift_codegen::ir::condcodes::IntCC::Equal,
                        outcome,
                        0,
                    );
                    let pointer_type = builder.func.dfg.value_type(pointer);
                    let value = Lowered::ResponseBytes(SafeByteSpan::masked_at_producer(
                        builder,
                        pointer_type,
                        pointer,
                        len,
                        valid,
                    ));
                    let projected = site_operand_witness(&value).ok_or_else(|| {
                        unsupported(
                            "Effect",
                            "the carried site-operand projection produced no value witness",
                        )
                    })?;
                    (
                        value,
                        SiteOperandSource::Carried {
                            word: word.word,
                            projected,
                        },
                    )
                }
            };
            #[cfg(test)]
            if effect_seat_dispatch_mutation() == EffectSeatDispatchMutation::SubstituteSiteOperandValue
            {
                SITE_OPERAND_SUBSTITUTION_HITS.with(|cell| cell.set(cell.get() + 1));
                value = Lowered::Int {
                    value: builder.ins().iconst(types::I64, 0),
                    known: Some(0),
                };
            }
            Ok(SynthesizedArgument::SiteOperand {
                seat,
                index,
                value,
                source,
            })
        }

        pub(super) fn synthesized_fixed_identity(
            &self,
            role: SynthesizedFixedConstructorRole,
        ) -> Result<ConstructorIdentity, CraneliftBackendError> {
            self.static_transition_plan
                .synthesized_constructor_identity(SynthesizedConstructorRole::Fixed(role))
        }

        /// Build one compiler-synthesized aggregate template at an exact producer
        /// seat.
        ///
        /// `seat` is the `Effect` occurrence whose lowering is making this use. It
        /// is passed explicitly rather than read from ambient state so that the
        /// occurrence this template carries is bound to the exact use — a role
        /// alone cannot select one, which is the whole point of the per-use key.
        pub(super) fn synthesized_constructor(
            &self,
            seat: StaticOriginId,
            path: &SynthesizedAggregatePath,
            role: SynthesizedFixedConstructorRole,
            constructor: RuntimeSymbol,
            args: Vec<SynthesizedArgument>,
            seats: &ClaimedEffectSeats<'_>,
        ) -> Result<Lowered, CraneliftBackendError> {
            // ⚠ Every allocation-reachable use in an operation's tree HAS a record,
            // site-bound ones included -- `OptionSome`, `FileError`,
            // `PrivateBufferSpan`, `ReadSome`. None of them is lawfully unmodelled.
            // The `None` below is reached only when no context is being defined,
            // which is not an emission this population covers at all.
            // The exact `D5a` emission owner of the context doing the lowering.
            // Absent means no context is being defined, which is not an emission
            // this population covers -- so no occurrence, and the loud refusal at
            // the allocation stands rather than a borrowed owner being invented.
            let Some(owner) = self.defining_emission_owner else {
                return Ok(Lowered::Constructor {
                    constructor,
                    synthesized_identity: Some(self.synthesized_fixed_identity(role)?),
                    occurrence: None,
                    args: args
                        .into_iter()
                        .map(SynthesizedArgument::into_lowered)
                        .map(ConstructorField::specialized)
                        .collect(),
                });
            };
            // ⛔ **`?`, never `.ok()`.** With a live emission owner, every
            // allocation-reachable synthesized use HAS a record — that is the rule
            // this checkpoint closed. So a lookup that fails here is a missing or
            // wrong authority, not an absence to route around, and mapping it to
            // `None` silently skipped the child reconciliation below and emitted a
            // template that would then refuse only at its allocation.
            //
            // `None` survives on exactly one branch: the explicit
            // no-emission-owner early return above.
            // ⭐ **`D7` — the A/B seat discriminator's ONLY seam.** Under the
            // `SiblingEffectSeat` mutation this becomes a DIFFERENT live effect
            // seat running the same host operation, while the arguments and
            // operands already built for the real seat are retained unchanged. So
            // a refusal below is attributable to the seat coordinate and to
            // nothing else — not to an invalid seat, not to a different program.
            #[cfg(test)]
            let seat = self.sibling_effect_seat_under_mutation(seat);
            let occurrence = Some(self.static_transition_plan.synthesized_aggregate_occurrence(
                owner,
                seat,
                path,
                SynthesizedConstructorRole::Fixed(role),
            )?);
            // The recipe and this call site are two statements of one shape, so
            // they are cross-checked rather than trusted to agree. A recipe that
            // drifts from the code that builds the aggregate would otherwise pick
            // the lane for a different node than the one being allocated, and
            // nothing downstream could tell.
            {
                let declared = self
                    .static_transition_plan
                    .synthesized_aggregate_children(
                        owner,
                        seat,
                        path,
                        SynthesizedConstructorRole::Fixed(role),
                    )?;
                self.reconcile_declared_children(owner, seat, path, declared, &args, seats)?;
            }
            Ok(Lowered::Constructor {
                constructor,
                synthesized_identity: Some(self.synthesized_fixed_identity(role)?),
                // `D7` — the planner's occurrence for this role, resolved here and
                // carried, exactly as a source constructor's is.
                occurrence,
                // The provenance has done its work; what the template holds is the
                // ordinary child, so nothing downstream sees a second carrier.
                args: args
                        .into_iter()
                        .map(SynthesizedArgument::into_lowered)
                        .map(ConstructorField::specialized)
                        .collect(),
            })
        }

        /// The program ROOT's occurrence in place of the callee's scheduling entry,
        /// under the self-authority probe only.
        ///
        /// ⚠ Returns the coordinate unchanged when the plan has no root, or when the
        /// root IS the callee's entry -- so a control must assert the hit count.
        ///
        /// ⛔ The root is a real, live, planned occurrence, never a fabricated one.
        /// A refusal driven by an unusable coordinate would be a claim about
        /// coordinate VALIDITY; the claim here is that a self-authorizing aggregate
        /// does not care WHICH live coordinate it crosses at.
        #[cfg(test)]
        pub(super) fn callee_scheduling_origin_under_mutation(
            &self,
            origin: StaticOriginId,
        ) -> StaticOriginId {
            if GOVERNED_ALLOCATION_MUTATION.with(std::cell::Cell::get)
                != GovernedAllocationMutation::CalleeSchedulingOrigin
            {
                return origin;
            }
            let Ok(root) = self.static_transition_plan.root_static_origin() else {
                return origin;
            };
            if root == origin {
                return origin;
            }
            governed_allocation_hit();
            let used = root;
            CALLEE_SCHEDULING_ORIGIN_USED.with(|cell| cell.set(Some((origin, used))));
            used
        }

        #[cfg(not(test))]
        pub(super) fn callee_scheduling_origin_under_mutation(
            &self,
            origin: StaticOriginId,
        ) -> StaticOriginId {
            origin
        }

        /// Swap in a sibling effect seat, under the A/B mutation only.
        ///
        /// ⚠ Returns the seat unchanged when no sibling exists. A control must
        /// therefore assert the HIT COUNT rather than the refusal alone: without
        /// it, "the fixture has no sibling seat so nothing was swapped" and "the
        /// swap happened and was caught" are the same green.
        #[cfg(test)]
        fn sibling_effect_seat_under_mutation(&self, seat: StaticOriginId) -> StaticOriginId {
            if GOVERNED_ALLOCATION_MUTATION.with(std::cell::Cell::get)
                != GovernedAllocationMutation::SiblingEffectSeat
            {
                return seat;
            }
            match self.static_transition_plan.sibling_effect_seat(seat) {
                Some(sibling) => {
                    governed_allocation_hit();
                    sibling
                }
                None => seat,
            }
        }

        /// Every operand must be the KIND the tree assumed when it took the meet.
        ///
        /// ⛔ Arity agreement is not sufficient and never was: a model that says
        /// `Scalar` where a referent-bearing child is passed has the right count
        /// and the wrong lane, and the aggregate is then allocated persistent over
        /// an operand that can be arena-owned -- the dangling parent this whole
        /// record exists to prevent.
        pub(super) fn reconcile_declared_children(
            &self,
            owner: ContinuationEmissionOwner,
            seat: StaticOriginId,
            path: &SynthesizedAggregatePath,
            declared: &'static [SynthesizedAggregateNode],
            args: &[SynthesizedArgument],
            seats: &ClaimedEffectSeats<'_>,
        ) -> Result<(), CraneliftBackendError> {
            if declared.len() != args.len() {
                return Err(unsupported(
                    "Constructor",
                    format!(
                        "synthesized aggregate node is planned with {} children but the emitter \
                         built {}",
                        declared.len(),
                        args.len()
                    ),
                ));
            }
            for (position, (child, argument)) in declared.iter().zip(args).enumerate() {
                let position = u32::try_from(position).map_err(|_| {
                    unsupported(
                        "Constructor",
                        "synthesized aggregate arity exceeds the path step space",
                    )
                })?;
                let agrees = match (child, argument) {
                    // The EXACT planned disposition, spill class and presence
                    // included -- not the broad `RepresentedImmediate` family.
                    //
                    // The family is not enough because it does not distinguish the
                    // two owner sets the planner derived from: `spill: None` has no
                    // boundary node at any magnitude, while `spill: Some(_)`
                    // becomes a persistent-store handle at wide ones. Accepting any
                    // immediate here would let a record justified by one of those
                    // govern an operand that is the other.
                    (
                        SynthesizedAggregateNode::Scalar { tag, spill },
                        SynthesizedArgument::Scalar(value),
                    ) => matches!(
                        value.boundary_disposition(),
                        BoundaryDisposition::RepresentedImmediate {
                            tag: emitted_tag,
                            spill: emitted_spill,
                        } if emitted_tag == *tag && emitted_spill == *spill
                    ),
                    // ⭐ **A nested child's path EXTENDS its parent's.** The operand
                    // must be the exact occurrence interned at
                    // `path.field(position)` -- not merely a constructor of the same
                    // role, and not that same role's occurrence somewhere else in
                    // the tree. This is what makes the path key CHECKED rather than
                    // merely declared: the emitter states where it put the child,
                    // the planner states where it planned it, and the two are
                    // compared. Collapse a step, drop one, or swap two, and the
                    // occurrence resolved here stops matching the operand.
                    //
                    // ⛔ Every allocation-reachable nested child HAS a record --
                    // `ReadSome`'s `PrivateBufferSpan` included, whose site-bound
                    // `ResourceToken` gets an exact seat-derived one. So a failed
                    // lookup here is missing authority, not a lawful absence, and
                    // it propagates. The comparison is against `Some(expected)`;
                    // under an `.ok()` mapping a missing record compared EQUAL to a
                    // child carrying no occurrence and the pair passed on two
                    // absences agreeing.
                    (
                        SynthesizedAggregateNode::Fixed { role: inner, .. },
                        SynthesizedArgument::Nested(value),
                    ) => {
                        // ⛔ Propagated, not mapped to `None`. Under `.ok()` a
                        // missing expected record compared EQUAL to a child
                        // carrying no occurrence, so two absences agreed and the
                        // pair passed. The comparison is against `Some(expected)`.
                        let expected = self.static_transition_plan.synthesized_aggregate_occurrence(
                            owner,
                            seat,
                            &path.field(position),
                            SynthesizedConstructorRole::Fixed(*inner),
                        )?;
                        matches!(
                            value,
                            Lowered::Constructor { occurrence, .. } if *occurrence == Some(expected)
                        )
                    }
                    // ⭐ **A dynamic child is checked ALTERNATIVE BY ALTERNATIVE.**
                    //
                    // ⛔ Not merely "is it a dynamic constructor". That weaker
                    // check is what let the three `ResourceKind` uses be built at
                    // ONE path with no complaint: the parent's own reconciliation
                    // passed on shape, and because a `Lowered::DynamicConstructor`
                    // carries no single occurrence, nothing compared the identities
                    // its alternatives were holding. Three distinct allocations
                    // then shared one record — exactly the aliasing the path key
                    // exists to prevent, and invisible.
                    //
                    // The set has no occurrence, but each alternative does, and
                    // each sits at `child_path.alternative(index)`. Comparing those
                    // is what makes the parent-to-child path law hold through a
                    // dynamic position as it already does through a fixed one.
                    (
                        SynthesizedAggregateNode::Dynamic(_),
                        SynthesizedArgument::Dynamic(Lowered::DynamicConstructor(dynamic)),
                    ) => self.dynamic_alternatives_agree(
                        owner,
                        seat,
                        &path.field(position),
                        dynamic,
                    )?,
                    // ⭐ **All three provenance axes, against the CLAIMED seat.**
                    //
                    // The planner derived this child's owners from the seat's
                    // operand `index`. So the value here must BE that operand:
                    // the seat must match, the index must be the declared one, and
                    // the value must still witness as this visit's claimed
                    // `Argument(index)`. Arity alone proves only the parent field
                    // position, and a value of the same shape and the same boundary
                    // disposition would otherwise inherit that operand's owner
                    // proof.
                    //
                    // ⛔ **The projection happens HERE, at a declared use, and
                    // nowhere else.** The two coordinate checks run first so that a
                    // wrong seat or a wrong index is the ordinary child mismatch
                    // below rather than a projection error about the seat the
                    // emitter did not name.
                    //
                    // ⛔ A carried `SiteOperand` is admitted only through the
                    // emitted byte-span helper. Reconciliation re-reads the exact
                    // claimed seat and requires both its carried word and the
                    // projected runtime-value witness to remain unchanged. It does
                    // not reconstruct a template, widen the carrier, borrow a
                    // sibling, or fall back.
                    (
                        SynthesizedAggregateNode::SiteOperand(declared_index),
                        SynthesizedArgument::SiteOperand {
                            seat: bound,
                            index,
                            value,
                            source,
                        },
                    ) => {
                        if *bound != seat || index != declared_index {
                            false
                        } else {
                            let (_, operand) =
                                seats.operand(EffectSeatSlot::Argument(*declared_index))?;
                            match (source, operand) {
                                (
                                    SiteOperandSource::Specialized,
                                    LoweringOperand::Specialized(projected),
                                ) => {
                                    site_operand_witness(value).is_some()
                                        && site_operand_witness(value)
                                            == site_operand_witness(projected)
                                }
                                (
                                    SiteOperandSource::Carried { word, projected },
                                    LoweringOperand::Carried(actual),
                                ) => {
                                    *word == actual.word
                                        && site_operand_witness(value).as_ref() == Some(projected)
                                }
                                _ => false,
                            }
                        }
                    }
                    // ⛔ `Absent` marks a host-result arm that builds no aggregate,
                    // so it is never a child of a planned record.
                    (SynthesizedAggregateNode::Absent, _) => {
                        return Err(unsupported(
                            "Constructor",
                            "a synthesized aggregate is planned with an absent child, so the \
                             tree describes an allocation whose operand is not built",
                        ));
                    }
                    // ⛔ The FORMS ARE DISJOINT. A mismatched pair is a refusal, not
                    // a fallthrough to a weaker check: passing a bare scalar where
                    // the tree declares a site-bound operand is precisely the
                    // substitution this typing exists to make unstateable.
                    _ => false,
                };
                if !agrees {
                    return Err(unsupported(
                        "Constructor",
                        format!(
                            "synthesized aggregate child {position} is planned as {child:?} but the \
                             emitter built a {}, so the meet was taken over a different node than \
                             the one being allocated",
                            lowered_value_kind(argument.lowered())
                        ),
                    ));
                }
            }
            Ok(())
        }

        /// Build one alternative of a compiler-synthesized dynamic constructor.
        ///
        /// ⭐ An alternative IS an allocation and has its own path-keyed ownership
        /// record; `emit_carrier_dynamic_constructor` takes its lane from that
        /// record rather than from the value-shape disposition. So this reconciles
        /// against the tree AND resolves the occurrence the emitter will carry: the
        /// node at `parent.alternative(position)` must be this exact role with this
        /// exact ordered child model.
        pub(super) fn synthesized_dynamic_alternative(
            &self,
            seat: StaticOriginId,
            parent: &SynthesizedAggregatePath,
            position: u32,
            tag: i64,
            role: SynthesizedFixedConstructorRole,
            constructor: RuntimeSymbol,
            fields: Vec<SynthesizedArgument>,
            seats: &ClaimedEffectSeats<'_>,
        ) -> Result<DynamicConstructorAlternativeV1, CraneliftBackendError> {
            // Absent means no context is being defined, which is not an emission
            // this population covers -- the same boundary `synthesized_constructor`
            // draws, and for the same reason.
            let role = SynthesizedConstructorRole::Fixed(role);
            let occurrence = self.reconcile_dynamic_alternative(
                seat,
                parent,
                position,
                role,
                &fields,
                seats,
            )?;
            Ok(DynamicConstructorAlternativeV1 {
                tag,
                constructor,
                identity: self.static_transition_plan.synthesized_constructor_identity(role)?,
                occurrence,
                fields: fields.into_iter().map(SynthesizedArgument::into_lowered).collect(),
            })
        }

        /// Whether the dynamic alternative population at a path EQUALS the
        /// planner's.
        ///
        /// Used for a dynamic CHILD at `parent.field(i)` and for a dynamic ROOT at
        /// the bare root path. The two are the same contract at different seats,
        /// which is why they share one function rather than one being a weaker
        /// spelling of the other.
        ///
        /// ⭐ **Equality, not prefix agreement.** The expected population comes from
        /// `synthesized_dynamic_alternatives` — the planner's own ordered roles at
        /// this exact `seat + child_path` — and the emitter's cardinality is
        /// compared to it before anything else.
        ///
        /// ⛔ **The count is never inferred from the emitter's vector.** An earlier
        /// spelling iterated `dynamic.alternatives` and resolved each position. That
        /// rejects an EXTRA alternative, because its path does not exist — but a
        /// vector missing its last alternative, or an empty one, agrees with every
        /// prefix and returns true. A planner tree with two `ResourceKind`
        /// alternatives then accepted an emitter carrying only alternative 0, and
        /// the missing allocation would never surface at all.
        ///
        /// ⛔ **The earlier text here said it would surface at a future whole-pass
        /// `image(R) = P` closeout. There is no such closeout and there will not
        /// be** — `P` is an authorization population, so a record no event related
        /// is LAWFUL and the whole-pass close states `image(R) ⊆ P`. Exact
        /// construction cardinality therefore cannot defer to the ledger under any
        /// later WP: the ledger cannot tell a truncated emitter from a lawfully
        /// unused record. It has to be established here, which is what this
        /// function does.
        ///
        /// The set itself has no record — it is not an allocation. Its alternatives
        /// are, and each one's role and identity are checked at its own position.
        pub(super) fn dynamic_alternatives_agree(
            &self,
            owner: ContinuationEmissionOwner,
            seat: StaticOriginId,
            child_path: &SynthesizedAggregatePath,
            dynamic: &DynamicConstructorV1,
        ) -> Result<bool, CraneliftBackendError> {
            let planned = self
                .static_transition_plan
                .synthesized_dynamic_alternatives(seat, child_path)?;
            if planned.len() != dynamic.alternatives.len() {
                return Ok(false);
            }
            for (index, (role, alternative)) in
                planned.iter().zip(&dynamic.alternatives).enumerate()
            {
                let index = u32::try_from(index).map_err(|_| {
                    unsupported(
                        "DynamicConstructor",
                        "the alternative population exceeds the path step space",
                    )
                })?;
                let position = child_path.alternative(index);
                // ⛔ Same fail-closed rule: missing planner authority must not
                // compare equal to an alternative carrying no occurrence.
                let expected = self.static_transition_plan.synthesized_aggregate_occurrence(
                    owner,
                    seat,
                    &position,
                    *role,
                )?;
                if alternative.occurrence != Some(expected) {
                    return Ok(false);
                }
            }
            Ok(true)
        }

        /// **Reconcile a host-result ROOT against the planner's tree.**
        ///
        /// ⭐ A root dynamic set is an allocation population exactly as a child one
        /// is — the resource surface at `HostResultError`, read progress at
        /// `HostResultOk`, the console `IOError` root. What makes it easy to miss
        /// is that **no node declares it**: a child is reached through its parent's
        /// ordered child model and is checked on the way, while a root is returned
        /// straight into `Lowered::HostResult` with nothing above it to compare
        /// against. The population equality had to be asked for here explicitly.
        ///
        /// ⛔ The check is BIDIRECTIONAL, and neither direction may be defaulted. A
        /// root the planner gives a dynamic set to must receive one; a root it does
        /// not must not. Treating a missing planned population as "no declaration,
        /// so nothing to check" would make an unplanned root set pass, which is the
        /// same shape as treating a short emitter vector as a prefix.
        pub(super) fn reconcile_host_result_root(
            &self,
            seat: StaticOriginId,
            root: &SynthesizedAggregatePath,
            value: &Lowered,
        ) -> Result<(), CraneliftBackendError> {
            // Absent means no context is being defined, which is not an emission
            // this population covers -- the same boundary every other synthesized
            // reconciliation draws.
            let Some(owner) = self.defining_emission_owner else {
                return Ok(());
            };
            // ⛔ **`?`, never `.ok()`.** A lookup FAILURE and a lawful non-dynamic
            // root are different answers, and the planner types them apart. Merging
            // them here -- an absent or non-`Effect` seat, a walk that leaves the
            // tree, an `IOError` position outside the closed inventory, a malformed
            // population -- would make every one of those read as "the planner
            // plans no set at this root", and a non-dynamic emitted root would then
            // match the absent case and be accepted. That is the missing-authority
            // default this function's contract forbids, and no shape or truncation
            // mutation can find it: both of those keep the lookup working.
            let planned = self
                .static_transition_plan
                .synthesized_root_alternative_population(seat, root)?;
            match (planned, value) {
                (Some(_), Lowered::DynamicConstructor(dynamic)) => {
                    if self.dynamic_alternatives_agree(owner, seat, root, dynamic)? {
                        Ok(())
                    } else {
                        Err(unsupported(
                            "DynamicConstructor",
                            format!(
                                "the host-result root at {root:?} disagrees with the planner's \
                                 closed alternative population, so an allocation at that root has \
                                 no record or a record has no allocation"
                            ),
                        ))
                    }
                }
                (Some(_), _) => Err(unsupported(
                    "DynamicConstructor",
                    format!(
                        "the planner plans a dynamic alternative set at {root:?} but the emitter \
                         built a {}",
                        lowered_value_kind(value)
                    ),
                )),
                (None, Lowered::DynamicConstructor(_)) => Err(unsupported(
                    "DynamicConstructor",
                    format!(
                        "the emitter built a dynamic alternative set at {root:?}, where the \
                         planner plans none, so its alternatives allocate with no records"
                    ),
                )),
                (None, _) => Ok(()),
            }
        }

        /// Reconcile one dynamic alternative against the tree and return its exact
        /// path-keyed occurrence.
        ///
        /// ⭐ The alternative allocates, so it needs a record — not merely a schema
        /// agreement. This resolves `owner + seat + parent.alternative(position) +
        /// role`, checks the role the tree has at that path and the ordered
        /// children the emitter built, and hands back the occurrence the emitter
        /// carries to its allocation.
        ///
        /// `None` when no context is being defined, which is the same boundary
        /// `synthesized_constructor` draws and for the same reason: that is not an
        /// emission this population covers, so the allocation refuses loudly rather
        /// than borrowing a lane.
        fn reconcile_dynamic_alternative(
            &self,
            seat: StaticOriginId,
            parent: &SynthesizedAggregatePath,
            position: u32,
            role: SynthesizedConstructorRole,
            fields: &[SynthesizedArgument],
            seats: &ClaimedEffectSeats<'_>,
        ) -> Result<Option<AggregateOccurrenceId>, CraneliftBackendError> {
            let Some(owner) = self.defining_emission_owner else {
                return Ok(None);
            };
            let path = parent.alternative(position);
            let (declared_role, declared) =
                self.static_transition_plan.synthesized_tree_node(seat, &path)?;
            if declared_role != role {
                return Err(unsupported(
                    "DynamicConstructor",
                    format!(
                        "alternative {position} is planned as {declared_role:?} but the emitter \
                         built {role:?}, so the path names a different node than the one being \
                         constructed"
                    ),
                ));
            }
            self.reconcile_declared_children(owner, seat, &path, declared, fields, seats)?;
            Ok(Some(self.static_transition_plan.synthesized_aggregate_occurrence(
                owner,
                seat,
                &path,
                role,
            )?))
        }

        /// The closed `IOError` alternative set at one exact position in the tree.
        ///
        /// ⭐ Every alternative here is a real allocation and takes its own
        /// path-keyed record, keyed `IoError(role)`. The set is built at a `parent`
        /// path rather than a bare role because the same inventory appears three
        /// times in the measured trees — `FileError` field 2, `ResourceHostIo`
        /// field 0, `ResourceReleaseFailed` field 2 — and those are different
        /// allocations.
        pub(super) fn synthesized_io_error_alternatives(
            &self,
            seat: StaticOriginId,
            parent: &SynthesizedAggregatePath,
            payload: Lowered,
            seats: &ClaimedEffectSeats<'_>,
        ) -> Result<Vec<DynamicConstructorAlternativeV1>, CraneliftBackendError> {
            let roles = self.static_transition_plan.synthesized_io_error_roles();
            if roles.len() != self.process_symbols.io_errors.len() {
                return Err(unsupported(
                    "DynamicConstructor",
                    "the closed IOError role inventory does not match the effect symbol population",
                ));
            }
            let last = roles.len().saturating_sub(1);
            self.process_symbols
                .io_errors
                .iter()
                .zip(roles)
                .enumerate()
                .map(|(position, (constructor, role))| {
                    let role = SynthesizedConstructorRole::IoError(*role);
                    let fields = (position == last)
                        .then(|| vec![SynthesizedArgument::Scalar(payload.clone())])
                        .unwrap_or_default();
                    let occurrence = self.reconcile_dynamic_alternative(
                        seat,
                        parent,
                        u32::try_from(position).map_err(|_| {
                            unsupported(
                                "DynamicConstructor",
                                "the IOError alternative population exceeds the path step space",
                            )
                        })?,
                        role,
                        &fields,
                        seats,
                    )?;
                    Ok(DynamicConstructorAlternativeV1 {
                        tag: i64::try_from(position).map_err(|_| {
                            unsupported(
                                "DynamicConstructor",
                                "the IOError alternative population exceeds the ABI discriminator",
                            )
                        })?,
                        constructor: constructor.clone(),
                        identity: self
                            .static_transition_plan
                            .synthesized_constructor_identity(role)?,
                        occurrence,
                        fields: fields.into_iter().map(SynthesizedArgument::into_lowered).collect(),
                    })
                })
                .collect()
        }
}

