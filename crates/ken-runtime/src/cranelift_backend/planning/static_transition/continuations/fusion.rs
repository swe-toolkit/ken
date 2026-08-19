//! The static-continuation fusion identity plane: fusion keys, ids,
//! descriptors, plans, views, claims, and the mint/derive/validate free fns.
//!
//! `RT-PLANNER-CONTINUATIONS-SPLIT` `D1` sub-split (operator ruling
//! `evt_43kptchmrpzc7`) — this child holds the fusion sub-domain of the
//! continuations owner. `ContinuationEmissionOwner` (the seat identity enum)
//! stays whole in the parent `continuations.rs`; its `Fusion` variant reaches
//! this module's `StaticContinuationFusionId` via `super::fusion::...`.

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};

#[allow(unused_imports)]
use super::*;
use super::super::{planner_capacity_error, planner_error, StaticTransitionPlan, StaticOriginId};
use crate::RuntimeExpr;

/// **`D3` — one exact call edge a fusion composes, with the coordinates the
/// emitter needs and no others.**
///
/// Ruled at `evt_1t3f4e8100rb5`. Every member is copied from a relation that
/// already resolved it; nothing here is re-derived at the emitter, and nothing
/// is a coincidence key.
///
/// `layer` records WHICH of the two ruled checked bindings selected this edge.
/// It is provenance, not a selector: the map is keyed by call identity, and two
/// layers of one fusion are two records rather than one record with a flag.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct FusionComposedEdge {
    pub(in crate::cranelift_backend::planning::static_transition) fusion: StaticContinuationFusionId,
    pub(in crate::cranelift_backend::planning::static_transition) target: ContinuationSpecializationId,
    pub(in crate::cranelift_backend::planning::static_transition) emission_owner: ContinuationEmissionOwner,
    pub(in crate::cranelift_backend::planning::static_transition) consumer_continuation_origin: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) producer_construct_origin: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) layer: FusionCompositionLayer,
}

/// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — `R`: the ONE consumer-binding
/// identity per body-owning fusion, realized by the fusion-owned body itself.**
///
/// Architect `evt_6bm54j10w1n88`. Named for a REALIZATION, not a call, because
/// that is what it is: `R` emits no call, enters no call funnel, and lowers no
/// second selected body. The fusion-owned body already emits that body exactly
/// once, and this record says *which planned continuation identity that body
/// realized* -- it neither moves nor duplicates the `FusionRegionClaim`, which
/// remains the authority for the producer construct, field, consuming call,
/// callee, producer body and redirect.
///
/// **Selection is the checked consumer binding plus the exact call-target
/// bijection -- the same authority `I` is selected by. The coordinate equalities
/// below are a CLOSURE CHECK APPLIED AFTER SELECTION, never a selector.**
/// Selecting on origin or body coincidence is the aliasing this node has
/// refused throughout: two unrelated identities that happened to agree would be
/// admitted, and the one that mattered would be admitted for the wrong reason.
#[derive(Clone, Debug)]
pub(in crate::cranelift_backend) struct FusionOwnedOuterRealization {
    pub(in crate::cranelift_backend::planning::static_transition) fusion: StaticContinuationFusionId,
    pub(in crate::cranelift_backend::planning::static_transition) target: ContinuationSpecializationId,
    pub(in crate::cranelift_backend::planning::static_transition) emission_owner: ContinuationEmissionOwner,
    pub(in crate::cranelift_backend::planning::static_transition) consumer_continuation_origin: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) selected_case_body: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) producer_body: StaticOriginId,
}

#[cfg_attr(not(test), allow(dead_code))]
impl FusionOwnedOuterRealization {
    pub(in crate::cranelift_backend) fn fusion(&self) -> StaticContinuationFusionId {
        self.fusion
    }
    pub(in crate::cranelift_backend) fn target(&self) -> ContinuationSpecializationId {
        self.target
    }
    pub(in crate::cranelift_backend) fn emission_owner(&self) -> ContinuationEmissionOwner {
        self.emission_owner
    }
    pub(in crate::cranelift_backend) fn consumer_continuation_origin(&self) -> StaticOriginId {
        self.consumer_continuation_origin
    }
    pub(in crate::cranelift_backend) fn selected_case_body(&self) -> StaticOriginId {
        self.selected_case_body
    }
    pub(in crate::cranelift_backend) fn producer_body(&self) -> StaticOriginId {
        self.producer_body
    }
}

/// Which ruled checked binding selected a composed edge.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum FusionCompositionLayer {
    /// Selected by the fusion key's checked CONSUMER binding.
    Outer,
    /// Selected by the fusion key's checked PRODUCER-ARGUMENT binding.
    Inner,
}

#[cfg_attr(not(test), allow(dead_code))]
impl FusionComposedEdge {
    pub(in crate::cranelift_backend) fn fusion(&self) -> StaticContinuationFusionId {
        self.fusion
    }
    pub(in crate::cranelift_backend) fn target(&self) -> ContinuationSpecializationId {
        self.target
    }
    pub(in crate::cranelift_backend) fn emission_owner(&self) -> ContinuationEmissionOwner {
        self.emission_owner
    }
    pub(in crate::cranelift_backend) fn layer(&self) -> FusionCompositionLayer {
        self.layer
    }
    /// **`D3` — the planner-authored consumer continuation this edge composes
    /// at.** The one authority for the frame an Inner composition may answer
    /// for; lowering closes it against the actual stack head and derives
    /// nothing.
    pub(in crate::cranelift_backend) fn consumer_continuation_origin(&self) -> StaticOriginId {
        self.consumer_continuation_origin
    }
}


thread_local! {
    static MUTATE_PRIMARY_FUSION_KEY_DERIVATION: Cell<bool> = const { Cell::new(false) };
}

/// `D2h` — perturb the PRIMARY key derivation, so the independent re-derivation
/// has something to catch.
#[cfg(test)]
pub(in crate::cranelift_backend) fn set_primary_fusion_key_derivation_mutated(armed: bool) {
    MUTATE_PRIMARY_FUSION_KEY_DERIVATION.with(|cell| cell.set(armed));
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2h` — dense identity of one interned
/// static continuation fusion.**
///
/// A fourth id domain, never cast into any of the other three. It is not a
/// `PredeclaredFunctionId` (it has no source occurrence of its own), not a
/// `ContinuationSpecializationId` (that class is defined over a real static
/// worker and this one has none), and not a `ContinuationContextId`.
///
/// Assigned from the complete key and from nothing else. There is no
/// constructor from an integer, so nothing can mint one and have it read as
/// planner-issued.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct StaticContinuationFusionId(pub(in crate::cranelift_backend::planning::static_transition) u32);

/// **The complete immutable identity of one static continuation fusion.**
///
/// Exactly the Architect's seven facts, in their order. Every member is a
/// planner-issued identity: none is a constructor spelling, a type, a row
/// number, a runtime tag, "the only continuation", or "the only marker".
/// **Distinct in any member means a distinct fusion.**
///
/// `checked_transport` is **required and is never an `Option`**. Absence does
/// not denote a smaller-but-valid identity: a producer whose transport cannot
/// be resolved from all three wrapper authorities is not a candidate, so it
/// never reaches a key.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct StaticContinuationFusionKey {
    pub(in crate::cranelift_backend::planning::static_transition) admitted: AdmittedContinuationDiscovery,
    pub(in crate::cranelift_backend::planning::static_transition) producer_construct_origin: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) producer_owner: PredeclaredFunctionId,
    pub(in crate::cranelift_backend::planning::static_transition) producer_alternative: u32,
    pub(in crate::cranelift_backend::planning::static_transition) recursive_position: u32,
    pub(in crate::cranelift_backend::planning::static_transition) producer_argument_origin: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) producer_argument_binding: CheckedIhBinding,
    pub(in crate::cranelift_backend::planning::static_transition) selected_case_body: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) consuming_call: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) consuming_callee: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) consumer_binding: CheckedIhBinding,
    pub(in crate::cranelift_backend::planning::static_transition) checked_transport: CheckedTransportCoordinate,
    pub(in crate::cranelift_backend::planning::static_transition) invocation_caller: PredeclaredFunctionId,
    pub(in crate::cranelift_backend::planning::static_transition) invocation_callee: PredeclaredFunctionId,
    pub(in crate::cranelift_backend::planning::static_transition) invocation_callee_entry: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) consumer_owner: PredeclaredFunctionId,
    pub(in crate::cranelift_backend::planning::static_transition) continuation_inputs: Vec<ContinuationSourceSlotAuthority>,
}

/// The PRIMARY derivation: the key a candidate determines.
///
/// Total, because a candidate has already passed every gate.
#[cfg_attr(not(test), allow(dead_code))]
pub(in crate::cranelift_backend::planning::static_transition) fn primary_fusion_key(candidate: &StaticContinuationFusionCandidate) -> StaticContinuationFusionKey {
    let mut key = StaticContinuationFusionKey {
        admitted: candidate.admitted,
        producer_construct_origin: candidate.producer_construct_origin,
        producer_owner: candidate.producer_owner,
        producer_alternative: candidate.producer_alternative,
        recursive_position: candidate.recursive_position,
        producer_argument_origin: candidate.producer_argument_origin,
        producer_argument_binding: candidate.producer_argument_binding,
        selected_case_body: candidate.selected_case_body,
        consuming_call: candidate.consuming_call,
        consuming_callee: candidate.consuming_callee,
        consumer_binding: candidate.consumer_binding,
        checked_transport: candidate.checked_transport.clone(),
        invocation_caller: candidate.invocation_caller,
        invocation_callee: candidate.invocation_callee,
        invocation_callee_entry: candidate.invocation_callee_entry,
        consumer_owner: candidate.consumer_owner,
        continuation_inputs: candidate.continuation_inputs.clone(),
    };
    // The mutation the independent re-derivation must catch. It perturbs the
    // PRIMARY derivation only; the second route below re-reads planner facts and
    // is untouched by it, which is what makes the comparison a real check rather
    // than a function agreeing with itself.
    // Perturbs a LOCATOR -- the admitted root the whole re-derivation hangs off
    // -- rather than a downstream member the second path recomputes anyway.
    // Mutating a recomputed field only shows that one field is recomputed; the
    // claim is that the locators are established, so the control has to attack
    // one of those.
    #[cfg(test)]
    if MUTATE_PRIMARY_FUSION_KEY_DERIVATION.with(Cell::get) {
        key.admitted.result_root = StaticOriginId(key.admitted.result_root.0 + 1);
    }
    key
}

/// The INDEPENDENT re-derivation: rebuild every member from planner facts,
/// reached by a second route.
///
/// This does not re-run the enumerator and does not read the candidate. It
/// takes the admitted discovery -- the one thing the ledger issues -- and
/// re-resolves the rest: the case's own declaration for the alternative and
/// position, the semantic child inventory for the argument and the case body,
/// the checked-IH authority for both bindings, the transport map for the
/// coordinate, the `StaticBody` edges for the triple, and the producer
/// environment for the ordered projection.
///
/// A disagreement with [`primary_fusion_key`] is a planner error, not a smaller
/// population: two routes to one identity must not differ.
#[cfg_attr(not(test), allow(dead_code))]
pub(in crate::cranelift_backend::planning::static_transition) fn rederive_fusion_key(
    plan: &StaticTransitionPlan<'_>,
    oriented: &crate::OrientedSubcontinuationPlanV1,
    key: &StaticContinuationFusionKey,
) -> Result<StaticContinuationFusionKey, CraneliftBackendError> {
    let transport = build_checked_transport(plan, oriented)?;
    let ih_bindings = build_checked_ih_bindings(plan)?;

    // THE LOCATORS ARE ESTABLISHED, NOT COPIED. An earlier revision took
    // `admitted`, the construct origin, the alternative and the position from
    // the primary key and re-derived only what hangs off them -- so a
    // correlated primary error in a selector reproduced itself one layer up and
    // the comparison agreed.
    //
    // THREE of the four are now independently justified against production
    // authority before anything downstream is derived: the admitted discovery
    // against the ledger, the construct origin against that root's result
    // population, and the alternative against constructor identity.
    //
    // THE POSITION IS THE EXCEPTION and is qualified at its own site below. It
    // is checked against the case's declaration but starts from a value read off
    // the key, so its independence is conditional on `consumer_binding` being
    // re-established and compared. Saying "each of the four" would overstate it,
    // and this comment did.
    let admitted = fusion_root_source_for_future_enumerator(plan)?
        .into_iter()
        .find(|entry| *entry == key.admitted)
        .ok_or_else(|| {
            planner_error("a fusion key's admitted discovery is not in the production ledger")
        })?;
    let continuation_origin = admitted.continuation_origin;

    let RuntimeExpr::ComputationalMatch { cases, .. } =
        plan.planned_occurrence_expr(continuation_origin)?
    else {
        return Err(planner_error("a fusion key names a non-computational consumer"));
    };

    // The construct origin must belong to the result population of the admitted
    // root, which is where a producer is allowed to come from at all.
    if !continuation_result_origins(plan, admitted.result_root)?
        .contains(&key.producer_construct_origin)
    {
        return Err(planner_error(
            "a fusion key's producer construct is not in its admitted root's result population",
        ));
    }

    // The alternative is DERIVED from constructor identity, not read off the
    // key: exactly one case may match, and multiplicity is a refusal.
    let identity = plan.constructor_symbol_identity(key.producer_construct_origin)?;
    let mut matching = Vec::new();
    for alternative in 0..cases.len() {
        if plan.case_constructor_identity(continuation_origin, alternative)? == identity {
            matching.push(alternative);
        }
    }
    let [alternative] = matching[..] else {
        return Err(planner_error(
            "a fusion key's producer constructor does not select exactly one consumer case",
        ));
    };
    let producer_alternative = u32::try_from(alternative)
        .map_err(|_| planner_capacity_error("fusion alternative exhausted"))?;
    let case = cases
        .get(alternative)
        .ok_or_else(|| planner_error("a fusion key names an absent consumer alternative"))?;

    // WHAT THIS ESTABLISHES, AND WHAT IT DOES NOT. It checks that the position
    // the key's consumer binding names is DECLARED ON THE CASE. It does not
    // establish that position independently of the key, because the value it
    // starts from is read off `key.consumer_binding` before that binding has
    // itself been rebuilt below.
    //
    // Independence for this member is CONDITIONAL: it comes from
    // `consumer_binding` being re-established from `ih_bindings` further down
    // and then compared in the caller's final whole-key equality. Reading this
    // check alone as an independent derivation is the overclaim -- an earlier
    // revision of this comment said "derived ... not the one the key asserts",
    // which is exactly the reading the conditionality rules out.
    let consuming_binding_position = key.consumer_binding.recursive_position as usize;
    let position = *case
        .recursive_positions
        .iter()
        .find(|position| **position == consuming_binding_position)
        .ok_or_else(|| planner_error("a fusion key names an undeclared recursive position"))?;
    let recursive_position = u32::try_from(position)
        .map_err(|_| planner_capacity_error("fusion position exhausted"))?;

    let selected_case_body = plan
        .semantic
        .child_origin(continuation_origin, 1 + alternative)?;
    let consuming_call = fusion_through_checked_wrappers(plan, selected_case_body)?;
    let consuming_callee = plan.semantic.child_origin(consuming_call, 0)?;
    let producer_argument_origin = plan
        .semantic
        .child_origins(key.producer_construct_origin)?
        .get(position)
        .copied()
        .ok_or_else(|| planner_error("a fusion key names an absent producer argument"))?;

    let producer_argument_binding = ih_bindings
        .get(&producer_argument_origin)
        .copied()
        .ok_or_else(|| planner_error("a fusion key's producer argument is not a hypothesis"))?;
    let consumer_binding = ih_bindings
        .get(&consuming_callee)
        .copied()
        .ok_or_else(|| planner_error("a fusion key's consuming callee is not a hypothesis"))?;
    let checked_transport = transport
        .get(&consuming_call)
        .cloned()
        .ok_or_else(|| planner_error("a fusion key's transport does not re-resolve"))?;

    let producer_owner = occurrence_authority(plan, key.producer_construct_origin)?.owner;
    let consumer_owner = occurrence_authority(plan, continuation_origin)?.owner;
    let (invocation_caller, invocation_callee, invocation_callee_entry) =
        fusion_unique_static_body_triple(plan, producer_owner)?
            .ok_or_else(|| planner_error("a fusion key's invocation edge does not re-resolve"))?;
    let environment = exact_continuation_source_environment(
        plan,
        producer_owner,
        key.admitted.result_root,
        key.producer_construct_origin,
        consumer_owner,
        continuation_origin,
    )?
    .ok_or_else(|| planner_error("a fusion key's input projection does not re-resolve"))?;

    Ok(StaticContinuationFusionKey {
        admitted,
        producer_construct_origin: key.producer_construct_origin,
        producer_owner,
        producer_alternative,
        recursive_position,
        producer_argument_origin,
        producer_argument_binding,
        selected_case_body,
        consuming_call,
        consuming_callee,
        consumer_binding,
        checked_transport,
        invocation_caller,
        invocation_callee,
        invocation_callee_entry,
        consumer_owner,
        continuation_inputs: environment.inputs.clone(),
    })
}

/// The planner-side descriptor, and the third leg of the bijection.
///
/// Deliberately **not** an `AbiUnitDefinition` arm and not an emission
/// descriptor -- those are `D2f`. It records only what the identity determines.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct StaticContinuationFusionDescriptor {
    pub(in crate::cranelift_backend::planning::static_transition) id: StaticContinuationFusionId,
    pub(in crate::cranelift_backend::planning::static_transition) continuation_inputs: usize,
    pub(in crate::cranelift_backend::planning::static_transition) recursive_position: u32,
    pub(in crate::cranelift_backend::planning::static_transition) consumer_owner: PredeclaredFunctionId,
    pub(in crate::cranelift_backend::planning::static_transition) producer_owner: PredeclaredFunctionId,
}

/// The interned fusion population: production planner state, and `D2f`'s fixed
/// input.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct StaticContinuationFusionPlan {
    pub(in crate::cranelift_backend::planning::static_transition) keys: Vec<StaticContinuationFusionKey>,
    pub(in crate::cranelift_backend::planning::static_transition) descriptors: Vec<StaticContinuationFusionDescriptor>,
    // Observation-only structural footprint. This plan derives `Eq` and
    // `PartialEq`, so feature-on and test builds include `walked` in equality;
    // feature-off builds do not carry the field. The containing
    // `StaticTransitionPlan` derives only `Clone`, and no production branch
    // compares two fusion plans. The feature therefore changes this private
    // plan's structural equality in its enabled build, but not plan decisions
    // or emitted artifacts.
    #[cfg(any(test, feature = "r3-4b-observation"))]
    pub(in crate::cranelift_backend::planning::static_transition) walked_admitted_continuation_discoveries: usize,
}

#[cfg_attr(not(test), allow(dead_code))]
impl StaticContinuationFusionPlan {
    /// **The production interning seam, keyed by the COMPLETE structural key.**
    ///
    /// Submitting a key either reuses the id an equal key already has or mints a
    /// fresh one. Equality is over every member, so two keys differing anywhere
    /// are two fusions and receive two ids -- there is no equivalence class that
    /// ignores a member and no fallback identity.
    ///
    /// This is what makes the bijection a property of the interner rather than
    /// of a lookup table: `id_for` below only reads, and a read cannot show that
    /// an unequal key would be given its own identity.
    pub(in crate::cranelift_backend::planning::static_transition) fn intern(
        &mut self,
        key: StaticContinuationFusionKey,
    ) -> Result<StaticContinuationFusionId, CraneliftBackendError> {
        if let Some(existing) = self.id_for(&key) {
            return Ok(existing);
        }
        let id = StaticContinuationFusionId(u32::try_from(self.keys.len()).map_err(|_| {
            planner_capacity_error("static continuation fusion identity exhausted")
        })?);
        self.descriptors.push(StaticContinuationFusionDescriptor {
            id,
            continuation_inputs: key.continuation_inputs.len(),
            recursive_position: key.recursive_position,
            consumer_owner: key.consumer_owner,
            producer_owner: key.producer_owner,
        });
        self.keys.push(key);
        Ok(id)
    }

    /// key -> ID, on WHOLE-key equality: a key differing in any member is a
    /// different key.
    pub(in crate::cranelift_backend::planning::static_transition) fn id_for(&self, key: &StaticContinuationFusionKey) -> Option<StaticContinuationFusionId> {
        self.keys
            .iter()
            .position(|candidate| candidate == key)
            .and_then(|index| u32::try_from(index).ok())
            .map(StaticContinuationFusionId)
    }

    /// ID -> key.
    pub(in crate::cranelift_backend::planning::static_transition) fn key_for(&self, id: StaticContinuationFusionId) -> Option<&StaticContinuationFusionKey> {
        self.keys.get(id.0 as usize)
    }

    /// ID -> descriptor.
    pub(in crate::cranelift_backend::planning::static_transition) fn descriptor_for(
        &self,
        id: StaticContinuationFusionId,
    ) -> Option<&StaticContinuationFusionDescriptor> {
        self.descriptors.iter().find(|entry| entry.id == id)
    }

    pub(in crate::cranelift_backend) fn len(&self) -> usize {
        self.keys.len()
    }

    /// `D2f` Deliverable 0 — READ-ONLY observation of what this plane resolved.
    ///
    /// **Not a re-opening of the identity plane.** Key derivation, the slot-role
    /// derivation, interning and the re-derivation validator are untouched;
    /// these hand back what `intern` already stored so a control downstream of
    /// a production compile can state *what* resolved rather than only *how
    /// many*. The gate needs the key itself: "exactly one key" and "the
    /// production path resolved the same key the planner controls derive" are
    /// claims a count cannot carry.
    #[cfg(any(test, feature = "r3-4b-observation"))]
    pub(in crate::cranelift_backend) fn observed_keys(&self) -> &[StaticContinuationFusionKey] {
        &self.keys
    }

    #[cfg(any(test, feature = "r3-4b-observation"))]
    pub(in crate::cranelift_backend) fn observed_descriptors(
        &self,
    ) -> &[StaticContinuationFusionDescriptor] {
        &self.descriptors
    }

    #[cfg(any(test, feature = "r3-4b-observation"))]
    pub(in crate::cranelift_backend) fn observed_walked_admitted_continuation_discoveries(
        &self,
    ) -> usize {
        self.walked_admitted_continuation_discoveries
    }

    pub(in crate::cranelift_backend::planning::static_transition) fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// The interned keys, in interning order — the production reader `D2f`'s
    /// installer and its ABI join both consume.
    ///
    /// Distinct from [`Self::observed_keys`], which is `#[cfg(test)]` and exists
    /// for a control to state what a compile resolved. Sharing one accessor
    /// would have made the production path depend on a test-only item, which
    /// compiles under `cfg(test)` and is a production-only red.
    pub(in crate::cranelift_backend::planning::static_transition) fn installed_keys(&self) -> &[StaticContinuationFusionKey] {
        &self.keys
    }
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` — one installed fused region, joined
/// from the fusion plane and its own ABI arena.**
///
/// Every field is borrowed from exactly one authority, so nothing here is a
/// second copy that could disagree: the identity and the complete key come from
/// the plane, the frame contract from the arena, and the redirect target is
/// derived on demand from the key rather than stored.
#[cfg_attr(not(test), allow(dead_code))]
pub(in crate::cranelift_backend) struct StaticContinuationFusionView<'plan> {
    pub(in crate::cranelift_backend::planning::static_transition) id: StaticContinuationFusionId,
    pub(in crate::cranelift_backend::planning::static_transition) key: &'plan StaticContinuationFusionKey,
    pub(in crate::cranelift_backend::planning::static_transition) planned: &'plan StaticContinuationFusionDescriptor,
    pub(in crate::cranelift_backend::planning::static_transition) header: AbiFrameHeader,
    pub(in crate::cranelift_backend::planning::static_transition) slots: &'plan [AbiSlot],
    pub(in crate::cranelift_backend::planning::static_transition) inputs: &'plan [abi::AbiContinuationInputAuthority],
}

#[cfg_attr(not(test), allow(dead_code))]
impl<'plan> StaticContinuationFusionView<'plan> {
    pub(in crate::cranelift_backend) fn id(&self) -> StaticContinuationFusionId {
        self.id
    }

    pub(in crate::cranelift_backend) fn key(&self) -> &'plan StaticContinuationFusionKey {
        self.key
    }

    /// The emission owner this fused region carries. **`Fusion(local id)` and
    /// never a `PredeclaredFunctionId`** — `D2f` Deliverable 2's whole point is
    /// that the fused region is a third thing that owns itself, not the original
    /// producer or consumer.
    pub(in crate::cranelift_backend) fn emission_owner(&self) -> ContinuationEmissionOwner {
        ContinuationEmissionOwner::Fusion(self.id)
    }

    /// The producer's source-body authority: the unit whose body computes the
    /// activation.
    pub(in crate::cranelift_backend) fn producer_owner(&self) -> PredeclaredFunctionId {
        self.planned.producer_owner
    }

    /// The suffix's source-body authority, **separately carried**. `D2f`
    /// Deliverable 3 requires the suffix to be lowered under its own validated
    /// authority rather than under the producer's, so the two are two fields
    /// here and never one.
    pub(in crate::cranelift_backend) fn consumer_owner(&self) -> PredeclaredFunctionId {
        self.planned.consumer_owner
    }

    pub(in crate::cranelift_backend) fn header(&self) -> AbiFrameHeader {
        self.header
    }

    pub(in crate::cranelift_backend) fn slots(&self) -> &'plan [AbiSlot] {
        self.slots
    }

    pub(in crate::cranelift_backend) fn inputs(&self) -> &'plan [abi::AbiContinuationInputAuthority] {
        self.inputs
    }

    pub(in crate::cranelift_backend) fn slot_offsets(
        &self,
    ) -> Result<(Vec<u32>, u32), CraneliftBackendError> {
        abi::slot_offsets(self.slots)
    }

    /// The one producer invocation this fused region may redirect, derived from
    /// the complete key by `D2f` Deliverable 5's landed selector.
    ///
    /// **Derived here rather than stored at install time**: storing it would make
    /// the redirect a fact about when the plane was installed, and the selector
    /// is the authority on which edge the key names.
    pub(in crate::cranelift_backend) fn redirect_target(
        &self,
        plan: &StaticTransitionPlan<'_>,
    ) -> Result<EmittableCallEdge, CraneliftBackendError> {
        fusion_redirect_target(plan, self.key)
    }
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` Deliverable 5 — the ONE producer
/// invocation edge a fusion may redirect, selected by the complete key.**
///
/// The frame's wording is *"redirect only the exact original producer
/// invocation ... not every edge to that callee, and not a search for a
/// plausible one"*. That is a statement about **how the edge is chosen**, so the
/// choosing is done here, once, from key members and from nothing else:
/// `invocation_caller`, `invocation_callee`, and `invocation_callee_entry`.
///
/// **Selection is by the key; the edge kind is VALIDATED, never selected on.**
/// Filtering the population by [`EmittableCallKind::StaticBody`] first would be
/// a criterion the key does not contain. If the three key members already
/// determine one edge, the filter is redundant; if they do not, the filter
/// silently resolves an ambiguity that a redirection is not allowed to have. So
/// the population is narrowed by the key alone, the count is required to be
/// exactly one, and the kind is then checked on the survivor.
///
/// **Both failure directions are named.** Zero matches means the key describes
/// an invocation this plan does not contain; more than one means the key does
/// not identify an invocation at all. Neither is recoverable by picking, and a
/// redirection built on either would move a call the source never made.
///
/// **No coordinate is written into this derivation.** An earlier statement of
/// this deliverable named the edge `0 -> 2`; that is the retired `px8j`
/// witness's coordinate and no edge of that shape exists on the checked twin,
/// whose invocation is `3 -> 2`. Naming either here would make the selector
/// agree with one witness by construction. The observed coordinate belongs in a
/// control, where it is a measurement.
#[cfg_attr(not(test), allow(dead_code))]
pub(in crate::cranelift_backend) fn fusion_redirect_target(
    plan: &StaticTransitionPlan<'_>,
    key: &StaticContinuationFusionKey,
) -> Result<EmittableCallEdge, CraneliftBackendError> {
    let mut matched = plan.executable_call_edges()?.into_iter().filter(|edge| {
        edge.caller() == key.invocation_caller
            && edge.callee() == key.invocation_callee
            && edge.callee_origin() == key.invocation_callee_entry
    });
    let selected = matched.next().ok_or_else(|| {
        planner_error(
            "the fusion key names a producer invocation this plan does not emit, so there is \
             no edge to redirect",
        )
    })?;
    if matched.next().is_some() {
        return Err(planner_error(
            "the fusion key's producer invocation identity selects more than one emittable \
             call edge, so it does not identify the invocation to redirect",
        ));
    }
    if selected.kind() != EmittableCallKind::StaticBody {
        return Err(planner_error(
            "the fusion key's producer invocation resolves to a declaration call rather than a \
             static body edge, which is not a producer invocation",
        ));
    }
    Ok(selected)
}


/// One fusion-owned body's installed record.
///
/// Keyed in the plan by the **producer body origin**; the producer function
/// is carried beside the identity so a consumer never has to re-derive which
/// unit the body belonged to from a call edge.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct FusionOwnedBody {
    pub(in crate::cranelift_backend::planning::static_transition) producer: PredeclaredFunctionId,
    pub(in crate::cranelift_backend::planning::static_transition) fusion: StaticContinuationFusionId,
}

impl FusionOwnedBody {
    pub(in crate::cranelift_backend) fn producer(self) -> PredeclaredFunctionId {
        self.producer
    }

    pub(in crate::cranelift_backend) fn fusion(self) -> StaticContinuationFusionId {
        self.fusion
    }
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` — one preflighted, move-only claim on
/// exactly one fused source region.**
///
/// The Architect's ruled emitter object. It is **compiler-only**: nothing here
/// reaches a runtime value, a `Lowered`, a Cranelift entity or a boundary word.
/// It is **move-only** by construction — no `Clone`, no `Copy` — so a second
/// consumer of one region cannot exist by copying the permit, which is the
/// property the affine discipline actually needs. `PartialEq` is deliberately
/// absent too: two claims are never compared, they are held or moved.
///
/// **Every field is copied out of the complete production key, or derived from
/// the immutable plan by the landed selector.** Nothing here is a witness
/// coordinate, a search for a plausible edge, or a re-measurement: a claim that
/// re-derived any member from the program text would be a second authority that
/// could disagree with the key its identity came from.
///
/// **What holding one authorizes, stated exactly, because it is narrower than
/// "the fused region is mine":** emit one `Fusion(id)` definition, redirect the
/// one named invocation, and — at that original call seat and once — replace the
/// claimed continuation prefix with its stored successor. It authorizes no
/// excision of any other origin, and it is not a licence to suppress a source
/// occurrence generally.
pub(in crate::cranelift_backend) struct FusionRegionClaim {
    pub(in crate::cranelift_backend::planning::static_transition) fusion: StaticContinuationFusionId,
    /// `Fusion(id)` — carried rather than recomputed at the emission seat, so
    /// the owner a body binds is the one preflight validated.
    pub(in crate::cranelift_backend::planning::static_transition) emission_owner: ContinuationEmissionOwner,
    /// The two source authorities, **separately**. `D2f` Deliverable 3 requires
    /// the producer's body and the suffix to be lowered under their own
    /// validated authorities; collapsing them to one field is the defect the
    /// pair exists to prevent.
    pub(in crate::cranelift_backend::planning::static_transition) producer_owner: PredeclaredFunctionId,
    pub(in crate::cranelift_backend::planning::static_transition) consumer_owner: PredeclaredFunctionId,
    /// The unique landed `StaticBody` edge this claim redirects, from
    /// [`fusion_redirect_target`] and from nothing else.
    pub(in crate::cranelift_backend::planning::static_transition) redirect: EmittableCallEdge,
    /// The producer's entry occurrence — the body the fused definition lowers
    /// first, under `producer_owner`.
    pub(in crate::cranelift_backend::planning::static_transition) producer_body: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) producer_construct_origin: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) producer_argument_origin: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) producer_alternative: u32,
    pub(in crate::cranelift_backend::planning::static_transition) recursive_position: u32,
    /// The claimed suffix: the selected case body and the consuming `Call`
    /// inside it, which `D2f` Deliverable 4 makes the sole consumer of the
    /// producer's activation.
    pub(in crate::cranelift_backend::planning::static_transition) selected_case_body: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) consuming_call: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) consuming_callee: StaticOriginId,
    /// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the ordered invocation-parameter
    /// projection.** Architect `evt_5edhqyyhw4585`.
    ///
    /// One entry per fused `Parameter` ABI slot, in source order: the positional
    /// argument child of this claim's exact checked consuming `Call`. It is the
    /// ORDINARY parameter run of the fused invocation, and it is a DIFFERENT
    /// AXIS from the producer constructor's recursive field run -- that field is
    /// the callee/worker binding, is compiler-only, and must never cross the
    /// ABI. Confusing the two is the false assembly this projection replaces,
    /// which was measured to refuse at the boundary before it could be emitted.
    ///
    /// Closure carried by the claim: not a fusion identity member, not a second
    /// candidate relation. Lowering receives it read-only and does not call
    /// `child_origin`, scan the source, or rebuild the list.
    pub(in crate::cranelift_backend::planning::static_transition) invocation_parameters: Vec<StaticOriginId>,
    /// The claimed continuation prefix's own origin — the
    /// `ComputationalMatchScrutinee` seat whose stored `next` replaces it when
    /// this claim is consumed.
    pub(in crate::cranelift_backend::planning::static_transition) continuation_origin: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) result_root: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) enclosing_specialization: Option<ContinuationSpecializationId>,
    pub(in crate::cranelift_backend::planning::static_transition) checked_transport: CheckedTransportCoordinate,
    /// The ordered projection the fused frame takes as its `Capture` run,
    /// carried so the seat passes exactly the operands the ABI declared.
    pub(in crate::cranelift_backend::planning::static_transition) inputs: Vec<ContinuationSourceSlotAuthority>,
}

#[cfg_attr(not(test), allow(dead_code))]
impl FusionRegionClaim {
    pub(in crate::cranelift_backend) fn fusion(&self) -> StaticContinuationFusionId {
        self.fusion
    }

    pub(in crate::cranelift_backend) fn emission_owner(&self) -> ContinuationEmissionOwner {
        self.emission_owner
    }

    pub(in crate::cranelift_backend) fn producer_owner(&self) -> PredeclaredFunctionId {
        self.producer_owner
    }

    pub(in crate::cranelift_backend) fn consumer_owner(&self) -> PredeclaredFunctionId {
        self.consumer_owner
    }

    pub(in crate::cranelift_backend) fn redirect(&self) -> EmittableCallEdge {
        self.redirect
    }

    pub(in crate::cranelift_backend) fn producer_body(&self) -> StaticOriginId {
        self.producer_body
    }

    pub(in crate::cranelift_backend) fn producer_construct_origin(&self) -> StaticOriginId {
        self.producer_construct_origin
    }

    pub(in crate::cranelift_backend) fn producer_argument_origin(&self) -> StaticOriginId {
        self.producer_argument_origin
    }

    pub(in crate::cranelift_backend) fn producer_alternative(&self) -> u32 {
        self.producer_alternative
    }

    pub(in crate::cranelift_backend) fn recursive_position(&self) -> u32 {
        self.recursive_position
    }

    pub(in crate::cranelift_backend) fn selected_case_body(&self) -> StaticOriginId {
        self.selected_case_body
    }

    pub(in crate::cranelift_backend) fn consuming_call(&self) -> StaticOriginId {
        self.consuming_call
    }

    pub(in crate::cranelift_backend) fn consuming_callee(&self) -> StaticOriginId {
        self.consuming_callee
    }

    /// **`D3` — the ordered invocation-parameter projection**, read-only.
    pub(in crate::cranelift_backend) fn invocation_parameters(&self) -> &[StaticOriginId] {
        &self.invocation_parameters
    }

    pub(in crate::cranelift_backend) fn continuation_origin(&self) -> StaticOriginId {
        self.continuation_origin
    }

    pub(in crate::cranelift_backend) fn result_root(&self) -> StaticOriginId {
        self.result_root
    }

    pub(in crate::cranelift_backend) fn enclosing_specialization(
        &self,
    ) -> Option<ContinuationSpecializationId> {
        self.enclosing_specialization
    }

    pub(in crate::cranelift_backend) fn checked_transport(&self) -> &CheckedTransportCoordinate {
        &self.checked_transport
    }

    pub(in crate::cranelift_backend) fn inputs(&self) -> &[ContinuationSourceSlotAuthority] {
        &self.inputs
    }

    /// The exact call seat this claim may be consumed at.
    ///
    /// **The seat is the redirected edge's own call-site origin, in the
    /// consumer's body.** **Not** the consuming `Call` and not the construct
    /// origin: the consumption replaces the continuation prefix that the
    /// *producer invocation* returns into, so the seat is where that invocation
    /// is, and matching on anything else would let the claim be consumed at a
    /// sibling occurrence that merely resembles it.
    pub(in crate::cranelift_backend) fn seat(&self) -> StaticOriginId {
        self.redirect.call_site_origin()
    }
}

/// Why one fused region's claim could not be issued.
///
/// A named cause per ruled refusal, so a control asserts *which* preflight rule
/// fired rather than that some string contains a word. ⇒ A refusal that
/// regressed into a different rule cannot pass a test written for the first.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum FusionClaimRefusal {
    /// The plane/ABI/id join does not agree with the complete key.
    Identity,
    /// The named producer invocation is absent, ambiguous, or not a static body
    /// edge. (Raised by [`fusion_redirect_target`] itself.)
    SelectorEdge,
    /// The edge's caller is not the consumer owner, its callee is not the
    /// producer owner, or its callee entry is not the producer body.
    InvocationTriple,
    /// The redirect would target the unit it is emitted into.
    ///
    /// **No witness in the `D2j` family reaches this, and no control claims
    /// one.** It is a fail-closed guard against a self-recursive shape this
    /// fixture family cannot produce, kept because the alternative is emitting a
    /// region fused with itself. Do not write a row asserting it fires.
    SelfRedirection,
    /// A checked binder or the admitted ledger root disagrees with the key.
    BinderAgreement,
    /// The consuming callee is not the binder the key names, or that binder's
    /// hypothesis does not resolve to the producer body.
    ///
    /// **This is the relation BETWEEN the two facts [`Self::BinderAgreement`]
    /// checks, and it is why that rule is not sufficient on its own.**
    /// `BinderAgreement` establishes two MARGINAL facts — that the key's
    /// consuming binder sits at the admitted frame and recursive position, and
    /// that the admitted result root equals the invocation callee entry. Each
    /// is a statement about one operand. Neither says that the hypothesis THAT
    /// binder names is a recursive invocation of THAT body, which is the fact
    /// the fused self edge is emitted against.
    BinderBodyResolution,
    /// The ordered input projection is unavailable or disagrees with the frame
    /// the ABI declared for it.
    InputAvailability,
    /// The fused frame does not declare exactly one ordinary result lane, so
    /// what it exports is not closure-free final data by its own contract.
    ResultLane,
    /// Two claims name one edge, one continuation frame, or one suffix.
    OverlappingClaim,
}

impl FusionClaimRefusal {
    pub(in crate::cranelift_backend::planning::static_transition) fn detail(self) -> &'static str {
        match self {
            Self::Identity => {
                "a static continuation fusion claim's identity join disagrees with its complete \
                 key, so the region it names is not the one the planner interned"
            }
            Self::SelectorEdge => {
                "a static continuation fusion claim has no unique landed static body invocation \
                 to redirect"
            }
            Self::InvocationTriple => {
                "a static continuation fusion claim's redirect edge is not the producer \
                 invocation its key names: caller must be the consumer owner, callee the \
                 producer owner, and callee entry the producer body"
            }
            Self::SelfRedirection => {
                "a static continuation fusion claim would redirect an invocation into the unit it \
                 is emitted from, which fuses a region with itself"
            }
            Self::BinderAgreement => {
                "a static continuation fusion claim's checked binders do not agree with the key's \
                 recursive position, admitted continuation origin, or admitted result root"
            }
            Self::BinderBodyResolution => {
                "a static continuation fusion claim's consuming callee is not the checked binder \
                 its key names, or that binder's hypothesis does not resolve to the producer body \
                 the claim redirects into"
            }
            Self::InputAvailability => {
                "a static continuation fusion claim's ordered input projection is unavailable or \
                 disagrees with the capture run its own ABI frame declares"
            }
            Self::ResultLane => {
                "a static continuation fusion claim's frame does not declare exactly one result \
                 lane, so the fused region has no single closure-free lane to export through"
            }
            Self::OverlappingClaim => {
                "two static continuation fusion claims name one invocation edge, one continuation \
                 frame, or one consuming suffix, so the regions are not disjoint"
            }
        }
    }
}

pub(in crate::cranelift_backend::planning::static_transition) fn fusion_claim_error(cause: FusionClaimRefusal) -> CraneliftBackendError {
    planner_error(cause.detail())
}

/// **`D2f` — the affine ledger over every installed fusion's region claim.**
///
/// Built once by [`Self::preflight`], **before any unit is declared and before
/// any body is defined**, which is the ordering the ruling fixes: a refusal
/// after the first definition exists is a partially emitted module rather than a
/// rejection.
///
/// Affine on the fusion identity. [`Self::consume`] **moves** the claim out, so
/// a second consumption of one region is not a policy check that could be
/// forgotten — there is no longer a claim to consume.
pub(in crate::cranelift_backend) struct FusionRegionClaimLedger {
    /// Every installed fusion, read once at preflight. **Never** derived from
    /// [`Self::claims`], which shrinks as claims are consumed: a planned set
    /// that emptied alongside its claims could not detect an unconsumed one.
    pub(in crate::cranelift_backend::planning::static_transition) planned: BTreeSet<StaticContinuationFusionId>,
    pub(in crate::cranelift_backend::planning::static_transition) claims: BTreeMap<StaticContinuationFusionId, FusionRegionClaim>,
    /// The regions whose claim has been consumed at its seat, and the seat it
    /// was consumed at — recorded rather than inferred, so closeout states
    /// where the takeover happened.
    pub(in crate::cranelift_backend::planning::static_transition) consumed: BTreeMap<StaticContinuationFusionId, StaticOriginId>,
    /// The regions for which a `Fusion(id)` definition was emitted.
    pub(in crate::cranelift_backend::planning::static_transition) defined: BTreeSet<StaticContinuationFusionId>,
    /// The regions whose named invocation was actually redirected.
    pub(in crate::cranelift_backend::planning::static_transition) redirected: BTreeSet<StaticContinuationFusionId>,
    /// `D2f` producer side — the regions whose producer body the plan now says
    /// a fused definition owns. Recorded by the atomic install and by nothing
    /// else, so the closeout bijects against a fact the plan actually holds.
    pub(in crate::cranelift_backend::planning::static_transition) body_owned: BTreeSet<StaticContinuationFusionId>,
    /// Whether ownership has been recorded at all. **Separate from
    /// `body_owned.is_empty()` on purpose:** a compile with zero fused regions
    /// records an empty set, and an emptiness sentinel cannot tell that apart
    /// from never having recorded, so exact-once would silently become
    /// at-most-once-if-non-empty.
    pub(in crate::cranelift_backend::planning::static_transition) body_owned_recorded: bool,
}

/// Test-only corruption of the claim's ordered invocation-parameter
/// projection at its production derivation site.
///
/// The three variants move different properties. `MoveFirstToCallee` preserves
/// the declared arity and therefore reaches lowering's independent
/// visited-origin comparison. `DropLast` and `AppendCallee` change the arity
/// and must be refused by preflight before a claim is issued.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum FusionClaimParameterMutation {
    Exact,
    MoveFirstToCallee,
    DropLast,
    AppendCallee,
}

/// Test-only corruption of the producer-capture population after the fusion
/// key has selected its real producer descriptor.
///
/// This moves only the selected descriptor's capture count presented to fusion
/// admission. It does not invent a claim, source relation, capture authority,
/// or ABI input; the production detector must refuse the new non-empty
/// disposition before the fusion ABI is installed.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum FusionProducerCaptureMutation {
    Exact,
    ForceNonEmptyAfterSelection,
}

#[cfg(test)]
thread_local! {
    pub(in crate::cranelift_backend::planning::static_transition) static FUSION_CLAIM_PARAMETER_MUTATION: Cell<FusionClaimParameterMutation> =
        const { Cell::new(FusionClaimParameterMutation::Exact) };
    pub(in crate::cranelift_backend::planning::static_transition) static FUSION_PRODUCER_CAPTURE_MUTATION: Cell<FusionProducerCaptureMutation> =
        const { Cell::new(FusionProducerCaptureMutation::Exact) };
    pub(in crate::cranelift_backend::planning::static_transition) static R3_FUSION_CLAIM_CONSUMPTIONS: std::cell::RefCell<
        Vec<(StaticContinuationFusionId, StaticOriginId)>,
    > = const { std::cell::RefCell::new(Vec::new()) };
}

/// Run one production compile with the selected producer's capture population
/// changed from empty to non-empty. The guard restores exact behaviour even if
/// planning returns early or unwinds.
#[cfg(test)]
pub(in crate::cranelift_backend) fn with_fusion_producer_capture_mutation<R>(
    mutation: FusionProducerCaptureMutation,
    run: impl FnOnce() -> R,
) -> R {
    struct Restore(FusionProducerCaptureMutation);

    impl Drop for Restore {
        fn drop(&mut self) {
            FUSION_PRODUCER_CAPTURE_MUTATION.with(|cell| cell.set(self.0));
        }
    }

    let previous = FUSION_PRODUCER_CAPTURE_MUTATION.with(|cell| cell.replace(mutation));
    let _restore = Restore(previous);
    run()
}

/// Run one production compile with a single claim-parameter defect installed.
/// The guard restores exact behaviour even if the compile unwinds.
#[cfg(test)]
pub(in crate::cranelift_backend) fn with_fusion_claim_parameter_mutation<R>(
    mutation: FusionClaimParameterMutation,
    run: impl FnOnce() -> R,
) -> R {
    struct Restore(FusionClaimParameterMutation);

    impl Drop for Restore {
        fn drop(&mut self) {
            FUSION_CLAIM_PARAMETER_MUTATION.with(|cell| cell.set(self.0));
        }
    }

    let previous = FUSION_CLAIM_PARAMETER_MUTATION.with(|cell| cell.replace(mutation));
    let _restore = Restore(previous);
    run()
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_r3_fusion_claim_consumptions() {
    R3_FUSION_CLAIM_CONSUMPTIONS.with(|cell| cell.borrow_mut().clear());
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn r3_fusion_claim_consumptions(
) -> Vec<(StaticContinuationFusionId, StaticOriginId)> {
    R3_FUSION_CLAIM_CONSUMPTIONS.with(|cell| cell.borrow().clone())
}

#[cfg_attr(not(test), allow(dead_code))]
impl FusionRegionClaimLedger {
    /// Derive one claim per installed fusion, refusing before anything is
    /// declared or defined.
    ///
    /// **Every check here is a relation between two authorities that were
    /// established separately** — the complete key, the installed ABI arena, and
    /// the plan's own emittable-edge projection. A check comparing a value
    /// with the thing that produced it is a restatement, not a gate, and none is
    /// written below.
    pub(in crate::cranelift_backend) fn preflight(
        plan: &StaticTransitionPlan<'_>,
    ) -> Result<Self, CraneliftBackendError> {
        let mut planned = BTreeSet::new();
        let mut claims = BTreeMap::new();
        // The three disjointness domains, accumulated across fusions. Separate
        // sets rather than one tuple set: two claims sharing a suffix while
        // differing in their edge is a real overlap, and a tuple key would
        // admit it.
        let mut claimed_edges = BTreeSet::new();
        let mut claimed_frames = BTreeSet::new();
        let mut claimed_suffixes = BTreeSet::new();

        let views = plan.continuation_fusions()?;
        // The planner's own binding authority, derived ONCE and only when there
        // is a claim to check with it. A compile with no installed fusion pays
        // nothing, which keeps this rule off the cost of every other compile.
        let ih_bindings = if views.is_empty() {
            BTreeMap::new()
        } else {
            build_checked_ih_bindings(plan)?
        };

        for view in views {
            let id = view.id();
            let key = view.key();

            // Identity: the ABI join's owners must be the key's owners. The
            // join in `continuation_fusions` matches descriptors to keys by
            // position; this is the independent half that makes the pairing
            // mean something.
            if view.producer_owner() != key.producer_owner
                || view.consumer_owner() != key.consumer_owner
            {
                return Err(fusion_claim_error(FusionClaimRefusal::Identity));
            }
            if view.emission_owner() != ContinuationEmissionOwner::Fusion(id) {
                return Err(fusion_claim_error(FusionClaimRefusal::Identity));
            }

            // The unique landed selector edge. `redirect_target` raises its own
            // absent/ambiguous/declaration-kind refusals; this preflight adds
            // the triple and the self-redirection rule on top of it.
            let redirect = view.redirect_target(plan)?;
            if redirect.caller() != key.consumer_owner
                || redirect.callee() != key.producer_owner
                || redirect.callee_origin() != key.invocation_callee_entry
            {
                return Err(fusion_claim_error(FusionClaimRefusal::InvocationTriple));
            }
            if redirect.caller() == redirect.callee() {
                return Err(fusion_claim_error(FusionClaimRefusal::SelfRedirection));
            }
            // ⇒ **The two source authorities are now known distinct, and there
            // is deliberately no separate check for it.** The triple above
            // established `caller == consumer_owner` and `callee ==
            // producer_owner`, and self-redirection established `caller !=
            // callee`; together those entail `producer_owner !=
            // consumer_owner`. A third `if` restating it could not fail, and a
            // refusal branch that cannot fail is worse than none — nothing
            // prompts a reader to check it, and it reads as a gate.

            // Binder and admitted-ledger agreement.
            //
            // **`producer_argument_binding.frame_origin ==
            // consumer_binding.frame_origin` is NOT among these, and its absence
            // is deliberate.** The two are the same type, sit side by side in
            // one key, and read as an obvious coherence check — but they name
            // different checked frames by design (measured 25 and 10 on the
            // canonical `Exact` witness), so asserting them equal would refuse
            // the very witness this class exists for. The recursive positions
            // beside them *do* agree, which is what makes the wrong row look
            // confirmatory if it is added partially.
            if key.producer_argument_binding.recursive_position != key.recursive_position
                || key.consumer_binding.recursive_position != key.recursive_position
                || key.consumer_binding.frame_origin != key.admitted.continuation_origin
                || key.admitted.result_root != key.invocation_callee_entry
            {
                return Err(fusion_claim_error(FusionClaimRefusal::BinderAgreement));
            }

            // The binder-to-body relation, which the two rules above do NOT
            // entail. Ruled at `evt_2rw6vhq8xrqcm`.
            //
            // `BinderAgreement` proves two MARGINAL facts: the key's consuming
            // binder sits at the admitted frame and recursive position, and the
            // admitted result root equals the invocation callee entry. Both are
            // statements about a single operand, and a key can satisfy both
            // while the hypothesis its binder names invokes some OTHER body --
            // which is precisely the fact `D3` emits the definition-local fused
            // self edge against.
            //
            // Two steps, because "the exact consuming callee" is half the
            // claim. First the callee is re-resolved through the planner's own
            // binding authority rather than taken from the key -- the key
            // ASSERTS a `consumer_binding`, and nothing above re-derives it.
            // Then that binder is resolved to a body and required to be the one
            // the claim redirects into.
            //
            // ⇒ **Only ONE comparison is written, and the other two are
            // entailed.** `InvocationTriple` above already forced
            // `redirect.callee_origin() == key.invocation_callee_entry`, and
            // the claim below is constructed with `producer_body:
            // key.invocation_callee_entry`. So comparing the resolved body
            // against all three would be one gate and two restatements, and a
            // branch that cannot fail reads as a check while being none.
            #[cfg(test)]
            let resolution_armed = !SUPPRESS_BINDER_BODY_RESOLUTION.with(Cell::get);
            #[cfg(not(test))]
            let resolution_armed = true;
            if resolution_armed {
                if ih_bindings.get(&key.consuming_callee).copied() != Some(key.consumer_binding) {
                    return Err(fusion_claim_error(FusionClaimRefusal::BinderBodyResolution));
                }
                if fusion_resolved_binder_body(plan, key.consumer_binding)?
                    != Some(key.invocation_callee_entry)
                {
                    return Err(fusion_claim_error(FusionClaimRefusal::BinderBodyResolution));
                }
            }

            // Ordinary input availability: the ordered projection the key
            // carries must be exactly the capture run the installed frame
            // declares for it, and every one of them must be an ordinary
            // carrier.
            //
            // **Both halves are SUBSUMED on every path that exists today, and
            // saying so is the honest form.** Installation necessarily precedes
            // preflight — a claim is derived from an *installed* fusion — and the
            // installer both builds the capture run from this same projection
            // and applies `AC-4`'s carrier gate to it. So no reachable input can
            // make either condition false, and **no control below claims to
            // exercise them.** They are kept as defence in depth against a
            // future caller that preflights against an arena some other writer
            // filled, which is the only way they could ever fire.
            // ---- `D3` — THE ORDERED INVOCATION-PARAMETER PROJECTION.
            // ---- Architect `evt_5edhqyyhw4585`, derived here and nowhere else.
            //
            // Derived only AFTER the fusion key has selected its exact consuming
            // `Call` and its checked callee binding, so this is closure over an
            // already-selected edge rather than a search. Lowering gets it
            // read-only: it never walks the source for these operands.
            let call_children = plan.semantic.child_origins(key.consuming_call)?;
            // 1. Child 0 is the claim's own consuming callee, and its checked
            //    binding resolves to the claim's producer body. Both halves,
            //    because the first alone would admit a call whose callee merely
            //    shares an origin with a binding that resolves elsewhere.
            // ⛔ **Validation 1 is NOT re-run here, and that is measured rather
            // than assumed.** Both halves -- child 0 being the claim's consuming
            // callee, and its checked binding resolving to the producer body --
            // belong to the `resolution_armed` block above, which is ONE rule
            // that `d3_the_consuming_binder_must_resolve_to_the_redirected_producer_body`
            // suppresses to prove its refusals are not an earlier proxy. Adding
            // an unconditional copy here refused under suppression too, reddening
            // exactly that control: the copy WAS the earlier proxy the control
            // exists to exclude. So validation 1 is discharged there, once.
            // 3. Every recorded argument origin is an actual positional child of
            //    THAT SAME call. Taken by construction from `call_children`, so
            //    there is no origin here the call does not own.
            //
            // Ordering is structural rather than selected. The `Call` planning
            // arm constructs `[callee] ++ args` by extending from the source
            // argument slice. `plan_sequence` may plan that run in reverse, but
            // writes every occurrence back at its original `enumerate` ordinal;
            // `SemanticSourceSeed::expression` then copies that positional slice
            // into the child-origin arena, and `child_origins` returns the same
            // validated slice. Therefore this `skip(1)` preserves source argument
            // order. No map, sort, keyed lookup, or second ordering decision
            // exists between the source walk and this projection.
            let mut invocation_parameters =
                call_children.iter().skip(1).copied().collect::<Vec<_>>();
            #[cfg(test)]
            match FUSION_CLAIM_PARAMETER_MUTATION.with(Cell::get) {
                FusionClaimParameterMutation::Exact => {}
                FusionClaimParameterMutation::MoveFirstToCallee => {
                    let first = invocation_parameters.first_mut().ok_or_else(|| {
                        planner_error(
                            "the claim-parameter mutation requires a non-empty argument run",
                        )
                    })?;
                    *first = key.consuming_callee;
                }
                FusionClaimParameterMutation::DropLast => {
                    // Both governed roots are unary, so this produces the same
                    // empty `Vec` as an absent parameter projection. The claim
                    // representation has no presence bit: the two states are
                    // deliberately indistinguishable and reach the same count
                    // check below.
                    invocation_parameters.pop().ok_or_else(|| {
                        planner_error(
                            "the claim-parameter mutation requires a non-empty argument run",
                        )
                    })?;
                }
                FusionClaimParameterMutation::AppendCallee => {
                    invocation_parameters.push(key.consuming_callee);
                }
            }
            // 2. The ordered argument count equals the fused header's ordinary
            //    parameter count. ⛔ Against the DESCRIPTOR's own slot walk, not
            //    against the vector's length, which would compare the projection
            //    with itself.
            let parameter_slots = view
                .slots()
                .iter()
                .filter(|slot| slot.kind == AbiSlotKind::Parameter)
                .count();
            if invocation_parameters.len() != parameter_slots {
                return Err(fusion_claim_error(FusionClaimRefusal::InputAvailability));
            }
            // 4. Producer-capture emptiness was already closed at
            //    `install_static_continuation_fusions`, against the selected
            //    producer descriptor. The captures below are the DISTINCT
            //    continuation-input suffix that becomes `claim.inputs()`;
            //    reading them as producer captures would conflate the two ABI
            //    axes the admission check exists to keep separate.
            if view.header().captures as usize != key.continuation_inputs.len() {
                return Err(fusion_claim_error(FusionClaimRefusal::InputAvailability));
            }
            let captures = view
                .slots()
                .iter()
                .filter(|slot| slot.kind == AbiSlotKind::Capture)
                .count();
            if captures != key.continuation_inputs.len()
                || view.inputs().len() != key.continuation_inputs.len()
            {
                return Err(fusion_claim_error(FusionClaimRefusal::InputAvailability));
            }
            for (ordinal, input) in key.continuation_inputs.iter().enumerate() {
                let ordinal = u32::try_from(ordinal).map_err(|_| {
                    planner_capacity_error("static continuation fusion input ordinal exhausted")
                })?;
                abi::fusion_input_carrier_admissibility(input.carrier, ordinal)?;
            }

            // Exactly one ordinary result lane. `CONVENTION_SLOTS` gives every
            // frame one, so a count other than one means this is not the frame
            // the arena built and the fused region has no single lane to export
            // closure-free final data through.
            if view
                .slots()
                .iter()
                .filter(|slot| slot.kind == AbiSlotKind::Result)
                .count()
                != 1
            {
                return Err(fusion_claim_error(FusionClaimRefusal::ResultLane));
            }

            // Pairwise disjointness, across every claim issued so far.
            let edge = (
                redirect.caller(),
                redirect.callee(),
                redirect.callee_origin(),
                redirect.call_site_origin(),
            );
            if !claimed_edges.insert(edge)
                || !claimed_frames.insert(key.admitted.continuation_origin)
                || !claimed_suffixes.insert(key.consuming_call)
            {
                return Err(fusion_claim_error(FusionClaimRefusal::OverlappingClaim));
            }

            planned.insert(id);
            let previous = claims.insert(
                id,
                FusionRegionClaim {
                    fusion: id,
                    emission_owner: ContinuationEmissionOwner::Fusion(id),
                    producer_owner: key.producer_owner,
                    consumer_owner: key.consumer_owner,
                    redirect,
                    producer_body: key.invocation_callee_entry,
                    producer_construct_origin: key.producer_construct_origin,
                    producer_argument_origin: key.producer_argument_origin,
                    producer_alternative: key.producer_alternative,
                    recursive_position: key.recursive_position,
                    selected_case_body: key.selected_case_body,
                    consuming_call: key.consuming_call,
                    consuming_callee: key.consuming_callee,
                    invocation_parameters,
                    continuation_origin: key.admitted.continuation_origin,
                    result_root: key.admitted.result_root,
                    enclosing_specialization: key.admitted.enclosing_specialization,
                    checked_transport: key.checked_transport.clone(),
                    inputs: key.continuation_inputs.clone(),
                },
            );
            if previous.is_some() {
                return Err(fusion_claim_error(FusionClaimRefusal::Identity));
            }
        }

        Ok(Self {
            planned,
            claims,
            consumed: BTreeMap::new(),
            defined: BTreeSet::new(),
            redirected: BTreeSet::new(),
            body_owned: BTreeSet::new(),
            body_owned_recorded: false,
        })
    }

    pub(in crate::cranelift_backend) fn planned(&self) -> &BTreeSet<StaticContinuationFusionId> {
        &self.planned
    }

    pub(in crate::cranelift_backend) fn is_empty(&self) -> bool {
        self.planned.is_empty()
    }

    /// Read a still-unconsumed claim without taking it.
    ///
    /// The definition pass needs the region's authorities and origins while the
    /// claim must stay outstanding — a definition emitted for a region whose
    /// claim was already consumed at the seat is exactly the double-takeover
    /// this ledger exists to refuse.
    pub(in crate::cranelift_backend) fn claim(
        &self,
        fusion: StaticContinuationFusionId,
    ) -> Option<&FusionRegionClaim> {
        self.claims.get(&fusion)
    }

    /// Move the exact selected claim out of the outstanding population without
    /// recording a successful consumption.
    ///
    /// Test-only: this represents a claim that escaped after the outer selector
    /// closed against it. It neither constructs a second claim nor writes the
    /// consumed ledger, so the production outstanding check remains the sole
    /// detector for the corrupted state.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn escape_selected_claim_for_test(
        &mut self,
        fusion: StaticContinuationFusionId,
    ) -> bool {
        self.claims.remove(&fusion).is_some()
    }

    /// **Consume the claim for `fusion` at `seat`, atomically and exactly once.**
    ///
    /// The seat is checked against the claim's own [`FusionRegionClaim::seat`]
    /// *before* the claim moves out, so a consumption at the wrong occurrence
    /// leaves the claim outstanding rather than spending it on the wrong
    /// takeover. A refusal here is therefore recoverable state, not a hole.
    pub(in crate::cranelift_backend) fn consume(
        &mut self,
        fusion: StaticContinuationFusionId,
        seat: StaticOriginId,
    ) -> Result<FusionRegionClaim, CraneliftBackendError> {
        let Some(claim) = self.claims.get(&fusion) else {
            return Err(planner_error(
                "a static continuation fusion region claim was consumed twice, or consumed for a \
                 fusion this compile never preflighted; the fused region has exactly one takeover",
            ));
        };
        if claim.seat() != seat {
            return Err(planner_error(
                "a static continuation fusion region claim was offered a call seat other than the \
                 one its redirected invocation names, so the takeover would replace a \
                 continuation prefix this claim does not own",
            ));
        }
        let claim = self
            .claims
            .remove(&fusion)
            .expect("the claim was present at the borrow above");
        self.consumed.insert(fusion, seat);
        #[cfg(test)]
        R3_FUSION_CLAIM_CONSUMPTIONS.with(|cell| cell.borrow_mut().push((fusion, seat)));
        Ok(claim)
    }

    pub(in crate::cranelift_backend) fn record_defined(
        &mut self,
        fusion: StaticContinuationFusionId,
    ) -> Result<(), CraneliftBackendError> {
        if !self.planned.contains(&fusion) {
            return Err(planner_error(
                "a static continuation fusion definition was emitted for a region this compile \
                 never preflighted",
            ));
        }
        if !self.defined.insert(fusion) {
            return Err(planner_error(
                "two static continuation fusion definitions were emitted for one installed region",
            ));
        }
        Ok(())
    }

    pub(in crate::cranelift_backend) fn record_redirected(
        &mut self,
        fusion: StaticContinuationFusionId,
    ) -> Result<(), CraneliftBackendError> {
        if !self.planned.contains(&fusion) {
            return Err(planner_error(
                "a producer invocation was redirected to a region this compile never preflighted",
            ));
        }
        if !self.redirected.insert(fusion) {
            return Err(planner_error(
                "one installed fused region's producer invocation was redirected twice",
            ));
        }
        Ok(())
    }

    /// Record, in one move, the regions whose producer body the plan now owns.
    ///
    /// Called only by `StaticTransitionPlan::install_fusion_owned_bodies`, and
    /// only after that method has validated its whole scratch map. Taking the
    /// set rather than one identity at a time is what keeps the ledger's view
    /// and the plan's map the same atomic fact.
    /// **Validate a proposed ownership record WITHOUT mutating anything.**
    ///
    /// Split from the commit below because the plan's install is a two-object
    /// transaction: the plan's map and this ledger's set must both move or
    /// neither must. A single fallible `record` forced the plan to mutate first
    /// and then discover a ledger refusal, which left the plan owning bodies
    /// after returning `Err`. Every refusal now happens here, before either
    /// object has changed.
    pub(in crate::cranelift_backend::planning::static_transition) fn check_body_owned(
        &self,
        owned: &BTreeSet<StaticContinuationFusionId>,
    ) -> Result<(), CraneliftBackendError> {
        if self.body_owned_recorded {
            return Err(planner_error(
                "static continuation fusion body ownership was recorded twice on one claim \
                 ledger; a ledger already spent on one plan cannot install ownership into another",
            ));
        }
        if !owned.is_subset(&self.planned) {
            return Err(planner_error(
                "a fusion-owned producer body names a region this compile never preflighted",
            ));
        }
        Ok(())
    }

    /// Commit the record. **Infallible by construction** — every condition was
    /// decided by [`Self::check_body_owned`], so there is no branch here that
    /// could reject after the plan has already moved its map.
    pub(in crate::cranelift_backend::planning::static_transition) fn commit_body_owned(&mut self, owned: BTreeSet<StaticContinuationFusionId>) {
        self.body_owned = owned;
        self.body_owned_recorded = true;
    }

    /// **The ruled closeout bijection: installed ↔ fusion-owned body ↔
    /// definition ↔ redirect ↔ consumed claim.**
    ///
    /// Written as four set equalities against `planned`, **not** as four counts.
    /// Equal counts hold vacuously at zero and hold wrongly when one region is
    /// defined twice while another is skipped; the sets do not.
    ///
    /// An **unconsumed** claim fails this, which is the affine half: a region
    /// whose definition was emitted and whose seat never took it over is a
    /// program in which the suffix runs twice.
    pub(in crate::cranelift_backend) fn close(self) -> Result<usize, CraneliftBackendError> {
        if !self.claims.is_empty() {
            return Err(planner_error(
                "a static continuation fusion region claim was never consumed: its definition was \
                 emitted but no call seat took the region over, so the delegated suffix is still \
                 lowered by its original consumer and would execute twice",
            ));
        }
        let consumed: BTreeSet<StaticContinuationFusionId> =
            self.consumed.keys().copied().collect();
        if self.body_owned != self.planned {
            return Err(planner_error(
                "the fusion-owned producer bodies are not exactly the installed fused regions: a                  producer whose body was taken over with no fused region, or a region whose                  producer still emits its own definition, is not a takeover",
            ));
        }
        if self.defined != self.planned {
            return Err(planner_error(
                "the emitted static continuation fusion definitions are not exactly the installed \
                 fused regions",
            ));
        }
        if self.redirected != self.planned {
            return Err(planner_error(
                "the redirected producer invocations are not exactly the installed fused regions",
            ));
        }
        if consumed != self.planned {
            return Err(planner_error(
                "the consumed static continuation fusion region claims are not exactly the \
                 installed fused regions",
            ));
        }
        Ok(self.planned.len())
    }
}


/// **`D2h` — build the fusion identity plane, in the ruled fail-closed order.**
///
/// Steps 1 to 3 are [`enumerate_live_fusion_candidates`]. Step 4 derives each
/// key twice by two routes -- [`primary_fusion_key`] from the candidate, and
/// [`rederive_fusion_key`] from planner facts -- and requires them equal. Step 5
/// interns, and **only then** does an id or descriptor exist.
///
/// **`D2f` — this is now REACHED on the production compile path.** Until the
/// emitter increment it had twelve call sites and every one was inside
/// `#[cfg(test)]`, so the plane was landed production state that no real compile
/// ever built. `lowering/core.rs` calls it at the single production planner call
/// site, which is the only scope where the static transition plan and the
/// oriented plan are both authoritative at once.
pub(in crate::cranelift_backend) fn build_static_continuation_fusion_plan(
    plan: &StaticTransitionPlan<'_>,
    entry: &RuntimeExpr,
    declarations: &BTreeMap<&str, &RuntimeDeclaration>,
    oriented: Option<&crate::OrientedSubcontinuationPlanV1>,
) -> Result<StaticContinuationFusionPlan, CraneliftBackendError> {
    let (candidates, _walked_admitted_continuation_discoveries) =
        enumerate_live_fusion_candidates_with_input_size(plan, entry, declarations, oriented)?;
    let mut fusion = StaticContinuationFusionPlan::default();
    #[cfg(any(test, feature = "r3-4b-observation"))]
    {
        fusion.walked_admitted_continuation_discoveries =
            _walked_admitted_continuation_discoveries;
    }
    // On this same `oriented = None` branch the helper above returns
    // `(Vec::new(), 0)`, so `fusion` is exactly `default()` in every build,
    // including test builds.
    let Some(oriented) = oriented else {
        return Ok(fusion);
    };

    for candidate in &candidates {
        let key = primary_fusion_key(candidate);
        let rederived = rederive_fusion_key(plan, oriented, &key)?;
        if rederived != key {
            return Err(planner_error(
                "a static continuation fusion key does not re-derive exactly from planner facts",
            ));
        }
        fusion.intern(key)?;
    }
    Ok(fusion)
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2i` — one fusion CANDIDATE, established
/// and not interned.**
///
/// Carries only facts from the Architect's closed seven. There is no id, no
/// descriptor, no key and no interning here, and none may be added: those are
/// `D2h`.
///
/// PRODUCTION planner state, consumed by the `D2h` key plane above it. The
/// `allow` covers the chain's top -- `D2f` has not been built -- and records
/// nothing about this type being test scaffolding, which it is not.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct StaticContinuationFusionCandidate {
    /// 1. the admitted discovery context, taken whole from the ledger.
    pub(in crate::cranelift_backend::planning::static_transition) admitted: AdmittedContinuationDiscovery,
    /// 2. the producer.
    pub(in crate::cranelift_backend::planning::static_transition) producer_construct_origin: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) producer_owner: PredeclaredFunctionId,
    pub(in crate::cranelift_backend::planning::static_transition) producer_alternative: u32,
    pub(in crate::cranelift_backend::planning::static_transition) recursive_position: u32,
    pub(in crate::cranelift_backend::planning::static_transition) producer_argument_origin: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) producer_argument_binding: CheckedIhBinding,
    /// 3. the selected case body and its exact consuming call.
    pub(in crate::cranelift_backend::planning::static_transition) selected_case_body: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) consuming_call: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) consuming_callee: StaticOriginId,
    pub(in crate::cranelift_backend::planning::static_transition) consumer_binding: CheckedIhBinding,
    /// 4. the required transport coordinate.
    pub(in crate::cranelift_backend::planning::static_transition) checked_transport: CheckedTransportCoordinate,
    /// 5. the unique `StaticBody` triple.
    pub(in crate::cranelift_backend::planning::static_transition) invocation_caller: PredeclaredFunctionId,
    pub(in crate::cranelift_backend::planning::static_transition) invocation_callee: PredeclaredFunctionId,
    pub(in crate::cranelift_backend::planning::static_transition) invocation_callee_entry: StaticOriginId,
    /// 6. the owner split.
    pub(in crate::cranelift_backend::planning::static_transition) consumer_owner: PredeclaredFunctionId,
    /// 7. the complete ordered input projection.
    pub(in crate::cranelift_backend::planning::static_transition) continuation_inputs: Vec<ContinuationSourceSlotAuthority>,
}

/// The exact producer invocation edge, or a refusal.
///
/// Requires **exactly one** `StaticBody` call edge into the producer unit.
/// Absence and multiplicity are both refusals: "the only edge" would be an
/// existential and choosing among several would be a guess.
pub(in crate::cranelift_backend::planning::static_transition) fn fusion_unique_static_body_triple(
    plan: &StaticTransitionPlan<'_>,
    producer_owner: PredeclaredFunctionId,
) -> Result<
    Option<(PredeclaredFunctionId, PredeclaredFunctionId, StaticOriginId)>,
    CraneliftBackendError,
> {
    let mut matching = Vec::new();
    for edge in plan.semantic.static_body_call_edges(&plan.edges)? {
        if edge.1 == producer_owner {
            matching.push(edge);
        }
    }
    // `D2i` `AC-3` multiplicity control. Arming this presents a SECOND matching
    // edge to the uniqueness decision and changes nothing else: the transport
    // gate, the bindings, the exact consuming suffix and the input projection
    // have all already been satisfied by the time this runs, so a candidate that
    // disappears here disappeared at the uniqueness gate specifically.
    #[cfg(test)]
    if DUPLICATE_STATIC_BODY_TRIPLE.with(Cell::get) {
        if let Some(first) = matching.first().copied() {
            matching.push(first);
        }
    }
    if matching.len() != 1 {
        // Absence and multiplicity are both refusals: "the only edge" would be
        // an existential and choosing among several would be a guess.
        return Ok(None);
    }
    Ok(matching.into_iter().next())
}

/// Resolve one checked induction hypothesis to the body it invokes.
///
/// ## The relation, and why it needs its own derivation
///
/// A [`CheckedIhBinding`] names a **frame and a recursive position** and no
/// body. The body it denotes is reached through the frame's own scrutinee: the
/// hypothesis at position `p` is the recursive result for the scrutinee's
/// argument at `p`, and that argument carries the producer as its body. So the
/// route is scrutinee, then the argument at the binder's recursive position,
/// then that argument's body.
///
/// ⇒ **The recursive position is USED here, not merely compared.** Preflight's
/// [`FusionClaimRefusal::BinderAgreement`] only checks the position against the
/// key's own copy of it, which two equal numbers satisfy without either naming
/// an argument that exists. Indexing the scrutinee's arguments by it is what
/// makes the position select something.
///
/// ## MEASURED
///
/// On the three `D2j` causes that install a key, the consumer binding resolves
/// to exactly the key's `invocation_callee_entry` — `Exact` (10, 0) to 37,
/// `ReHomed` (6, 0) to 33, `ProducerArity` (10, 0) to 38.
///
/// **And the route is not landing there by construction.** The same resolution
/// applied to each key's PRODUCER argument binding lands on a DIFFERENT body
/// every time — 34, 30 and 35 respectively, which is the producer's own
/// outgoing edge rather than the one being redirected. A derivation that
/// returned the redirect target whatever it was given could not do that.
///
/// ## The closure step, and why a non-closure argument REFUSES
///
/// On all three witnesses the argument is a [`RuntimeExpr::LexicalClosure`] and
/// the body is its child. An argument that is not a closure has no body, so
/// there is no hypothesis-invoked body to compare and the relation is
/// **unproved rather than false**. Returning the argument itself would make the
/// comparison answer a question it was not asked, so this refuses instead. The
/// population is three witness families, and a shape outside them is refused,
/// not guessed at.
pub(in crate::cranelift_backend::planning::static_transition) fn fusion_resolved_binder_body(
    plan: &StaticTransitionPlan<'_>,
    binding: CheckedIhBinding,
) -> Result<Option<StaticOriginId>, CraneliftBackendError> {
    let scrutinee = plan.semantic.child_origin(binding.frame_origin, 0)?;
    let Some(argument) = plan
        .semantic
        .child_origins(scrutinee)?
        .get(binding.recursive_position as usize)
        .copied()
    else {
        return Ok(None);
    };
    match plan.planned_occurrence_expr(argument)? {
        RuntimeExpr::LexicalClosure { .. } => {
            Ok(Some(plan.semantic.child_origin(argument, 0)?))
        }
        _ => Ok(None),
    }
}

/// Descend the checked wrappers to the occurrence they carry.
pub(in crate::cranelift_backend::planning::static_transition) fn fusion_through_checked_wrappers(
    plan: &StaticTransitionPlan<'_>,
    mut origin: StaticOriginId,
) -> Result<StaticOriginId, CraneliftBackendError> {
    loop {
        match plan.planned_occurrence_expr(origin)? {
            RuntimeExpr::CheckedSubcontinuationFrame { .. }
            | RuntimeExpr::CheckedComputationalIHSlots { .. }
            | RuntimeExpr::CheckedComputationalIHInvocation { .. }
            | RuntimeExpr::CheckedRecursiveInvocation { .. }
            | RuntimeExpr::CheckedJoinSite { .. } => {
                origin = plan.semantic.child_origin(origin, 0)?;
            }
            _ => return Ok(origin),
        }
    }
}

/// **`D2i` — the first LIVE fusion enumerator.**
///
/// ## What it now does
///
/// It consumes [`fusion_root_source_for_future_enumerator`], so the roots are
/// the production-admitted ledger with its complete identity. Nothing here
/// reconstructs a seed, scans a worker body, runs a parallel fixed point, or
/// changes terminal traversal.
///
/// ## Who consumes it
///
/// [`build_static_continuation_fusion_plan`] -- the `D2h` production key plane
/// -- consumes it. Both are production planner state and neither is
/// `#[cfg(test)]`, so this compiles into a non-test Runtime build.
///
/// ## What is still absent
///
/// **It still mints nothing itself.** The id, the descriptor and the interning
/// belong to the plane above it; a candidate here is established evidence
/// rather than an identity, and that split is deliberate.
///
/// **No emission, ABI or edge redirection exists** -- those are `D2f`, and the
/// plane is their fixed input rather than their beginning. No `R3` row is
/// claimed green.
///
/// ## The gates, each declining rather than guessing
///
/// Every fact comes from the Architect's closed seven. If a gate ever needs a
/// fact outside them, that is a closed-contract failure to report rather than
/// plumbing to add.
///
/// PRODUCTION planner state, for the same reason as the candidate it builds.
#[cfg_attr(not(test), allow(dead_code))]
pub(in crate::cranelift_backend::planning::static_transition) fn enumerate_live_fusion_candidates(
    plan: &StaticTransitionPlan<'_>,
    entry: &RuntimeExpr,
    declarations: &BTreeMap<&str, &RuntimeDeclaration>,
    oriented: Option<&crate::OrientedSubcontinuationPlanV1>,
) -> Result<Vec<StaticContinuationFusionCandidate>, CraneliftBackendError> {
    enumerate_live_fusion_candidates_with_input_size(plan, entry, declarations, oriented)
        .map(|(candidates, _)| candidates)
}

pub(in crate::cranelift_backend::planning::static_transition) fn enumerate_live_fusion_candidates_with_input_size(
    plan: &StaticTransitionPlan<'_>,
    entry: &RuntimeExpr,
    declarations: &BTreeMap<&str, &RuntimeDeclaration>,
    oriented: Option<&crate::OrientedSubcontinuationPlanV1>,
) -> Result<(Vec<StaticContinuationFusionCandidate>, usize), CraneliftBackendError> {
    crate::cranelift_backend::planning::validate_oriented_subcontinuation_transport(
        entry,
        declarations,
        oriented,
    )?;
    let Some(oriented) = oriented else {
        return Ok((Vec::new(), 0));
    };
    let transport = build_checked_transport(plan, oriented)?;
    let ih_bindings = build_checked_ih_bindings(plan)?;
    let mut candidates = Vec::new();
    let admitted_continuation_discoveries = fusion_root_source_for_future_enumerator(plan)?;
    let walked_admitted_continuation_discoveries = admitted_continuation_discoveries.len();

    for admitted in admitted_continuation_discoveries {
        let continuation_origin = admitted.continuation_origin;
        let RuntimeExpr::ComputationalMatch { cases, .. } =
            plan.planned_occurrence_expr(continuation_origin)?
        else {
            continue;
        };
        let consumer_owner = occurrence_authority(plan, continuation_origin)?.owner;

        for producer_construct_origin in continuation_result_origins(plan, admitted.result_root)? {
            let producer = plan.planned_occurrence_expr(producer_construct_origin)?;
            let RuntimeExpr::Construct { args, .. } = producer else {
                continue;
            };
            let identity = plan.constructor_symbol_identity(producer_construct_origin)?;
            let producer_owner = occurrence_authority(plan, producer_construct_origin)?.owner;

            for (alternative, case) in cases.iter().enumerate() {
                if plan.case_constructor_identity(continuation_origin, alternative)? != identity {
                    continue;
                }
                let producer_alternative = u32::try_from(alternative)
                    .map_err(|_| planner_capacity_error("fusion alternative exhausted"))?;

                for position in case.recursive_positions.iter().copied() {
                    let recursive_position = u32::try_from(position)
                        .map_err(|_| planner_capacity_error("fusion position exhausted"))?;
                    if args.get(position).is_none() {
                        continue;
                    }
                    let Some(producer_argument_origin) = plan
                        .semantic
                        .child_origins(producer_construct_origin)?
                        .get(position)
                        .copied()
                    else {
                        continue;
                    };
                    let Some(producer_argument_binding) =
                        ih_bindings.get(&producer_argument_origin).copied()
                    else {
                        continue;
                    };

                    let selected_case_body = plan
                        .semantic
                        .child_origin(continuation_origin, 1 + alternative)?;
                    let consuming_call =
                        fusion_through_checked_wrappers(plan, selected_case_body)?;
                    if !matches!(
                        plan.planned_occurrence_expr(consuming_call)?,
                        RuntimeExpr::Call { .. }
                    ) {
                        continue;
                    }
                    let consuming_callee = plan.semantic.child_origin(consuming_call, 0)?;
                    let expected = CheckedIhBinding {
                        frame_origin: continuation_origin,
                        recursive_position,
                    };
                    let Some(consumer_binding) = ih_bindings.get(&consuming_callee).copied() else {
                        continue;
                    };
                    if consumer_binding != expected {
                        continue;
                    }
                    let Some(checked_transport) = transport.get(&consuming_call).cloned() else {
                        continue;
                    };
                    let Some((invocation_caller, invocation_callee, invocation_callee_entry)) =
                        fusion_unique_static_body_triple(plan, producer_owner)?
                    else {
                        continue;
                    };
                    let Some(environment) = exact_continuation_source_environment(
                        plan,
                        producer_owner,
                        admitted.result_root,
                        producer_construct_origin,
                        consumer_owner,
                        continuation_origin,
                    )?
                    else {
                        continue;
                    };

                    candidates.push(StaticContinuationFusionCandidate {
                        admitted,
                        producer_construct_origin,
                        producer_owner,
                        producer_alternative,
                        recursive_position,
                        producer_argument_origin,
                        producer_argument_binding,
                        selected_case_body,
                        consuming_call,
                        consuming_callee,
                        consumer_binding,
                        checked_transport,
                        invocation_caller,
                        invocation_callee,
                        invocation_callee_entry,
                        consumer_owner,
                        continuation_inputs: environment.inputs.clone(),
                    });
                }
            }
        }
    }
    Ok((candidates, walked_admitted_continuation_discoveries))
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2i` — the root source fusion enumeration
/// consumes.**
///
/// [`enumerate_live_fusion_candidates`] consumes this, and the `D2h` production
/// key plane consumes that. All three are production planner state, so this
/// chain compiles into a non-test Runtime build.
///
/// What has no consumer yet is the plane at the top: `D2f` is its fixed input
/// and has not been built.
///
/// No seed-only path has been removed, because no such path exists on this
/// branch to remove -- the seed frontier is excluded here by construction
/// rather than deleted from live code.
///
/// What it fixes in advance is the root source. The alternative -- rebuilding
/// `child(consumer, 0)` over every planned `ComputationalMatch` -- reconstructs
/// the SEED frontier, and the fixed point admits discoveries past it that no
/// seed scan can name. An enumerator keyed on the seeds would walk a different
/// population from the one production works over, which is the defect this
/// exists to keep out of the enumerator when it is written.
///
/// The complete identity travels, `enclosing_specialization` included: it is
/// the immediate emission context and cannot be recovered from a worker body's
/// raw occurrence owner. One invocation, no parallel discovery, no seed
/// reconstruction, no worker-body scan.
#[cfg_attr(not(test), allow(dead_code))]
pub(in crate::cranelift_backend::planning::static_transition) fn fusion_root_source_for_future_enumerator(
    plan: &StaticTransitionPlan<'_>,
) -> Result<Vec<AdmittedContinuationDiscovery>, CraneliftBackendError> {
    admitted_continuation_discoveries(plan)
}

/// The admitted-discovery ledger for this plan, from the production fixed point.
///
/// One invocation, one ledger. Nothing here reconstructs a seed, scans worker
/// bodies, or runs a parallel fixed point -- the entries are exactly what
/// `build_continuation_specialization_plan` admitted, returned from that same
/// call.
#[cfg_attr(not(test), allow(dead_code))]
pub(in crate::cranelift_backend::planning::static_transition) fn admitted_continuation_discoveries(
    plan: &StaticTransitionPlan<'_>,
) -> Result<Vec<AdmittedContinuationDiscovery>, CraneliftBackendError> {
    let (_, _, _, _, admitted) = build_continuation_specialization_plan(plan)?;
    Ok(admitted)
}


impl<'src> super::StaticTransitionPlan<'src> {
    /// `D2f` Deliverable 0 — the generated-definition population, observed.
    ///
    /// **Zero until the emitter exists**, and that is exactly what the gate
    /// pins: the checked witness reaches plane `1` while this stays `0`, so the
    /// later `0 -> 1` movement is a statement about emission rather than about
    /// the plane.
    ///
    /// > ### REPOINTED — it used to read a population this class cannot enter
    /// >
    /// > This counted descriptors carrying the fusion definition arm in
    /// > [`AbiPlane::descriptors`], described as "the independent read -- it
    /// > comes from the ABI plane, not from the fusion plane the same compile
    /// > resolved." **The independence was real and the population was wrong.**
    /// > `descriptors` is built positionally over the semantic plane's function
    /// > partition and refuses a descriptor whose id is not its ordinal; its
    /// > builder has no arm constructing the fusion variant; and it is closed
    /// > before a fusion identity exists at all. So the zero was not "no
    /// > emission yet" — it was the only value that read could ever hold, and a
    /// > pre-movement baseline is the one measurement that cannot tell a resting
    /// > zero from an unreachable one.
    /// >
    /// > The direction was the saving grace rather than the design: an assertion
    /// > against an unmovable counter **fails** when emission lands, so the cost
    /// > was a wasted emitter turn and not a shipped false green.
    /// >
    /// > It now reads `fusion_descriptors` — the arena the generated definitions
    /// > are actually installed into. That arena is empty on every compile until
    /// > `D2f`'s installer is wired to the production path, so **this still reads
    /// > `0` and the gate is unchanged.** What changed is that the zero is now a
    /// > population the mover can enter.
    #[cfg(any(test, feature = "r3-4b-observation"))]
    pub(in crate::cranelift_backend) fn observed_fusion_definition_count(&self) -> usize {
        self.abi.fusion_descriptors.len()
    }






    /// The installed fusion-owned bodies, for the closeout's bijection.
    pub(in crate::cranelift_backend) fn fusion_owned_bodies(
        &self,
    ) -> &BTreeMap<StaticOriginId, FusionOwnedBody> {
        &self.fusion_owned_bodies
    }


    /// **`D2f` — install body ownership for every successfully claimed fused
    /// region, atomically, before any unit is declared or defined.**
    ///
    /// **The ordering is the mechanism, not a convenience.** Every row below is
    /// validated against the executable population *as it stands with no fusion
    /// ownership installed* — the complete one. Validating against a population
    /// already narrowed by a partial install would let the second row's
    /// "leaves no standalone route" become true *because the first row removed
    /// the route*, which is a self-fulfilling check. The scratch map is
    /// therefore built and validated in full, and only then moved into the plan.
    ///
    /// The redirect is the one **already validated by preflight and carried on
    /// the claim**. `fusion_redirect_target` is deliberately not re-run: after
    /// installation it would search a narrowed edge population and could not
    /// find the very edge whose supersession is the reason the population
    /// narrowed.
    pub(in crate::cranelift_backend) fn install_fusion_owned_bodies(
        &mut self,
        ledger: &mut FusionRegionClaimLedger,
    ) -> Result<(), CraneliftBackendError> {
        if self.fusion_bodies_installed {
            return Err(planner_error(
                "static continuation fusion body ownership may be installed exactly once",
            ));
        }
        let units = self.executable_units()?;
        let template_only = self.template_only_worker_bodies()?;
        let edges = self.emittable_call_edges()?;
        let mut scratch: BTreeMap<StaticOriginId, FusionOwnedBody> = BTreeMap::new();
        let mut owned = BTreeSet::new();

        for fusion in ledger.planned().iter().copied() {
            // Solely from a SUCCESSFUL claim. A planned identity whose claim was
            // refused or already consumed has no validated redirect, and
            // deriving ownership from it would take a body out of the emitted
            // population on the strength of a region nobody may take over.
            let claim = ledger.claim(fusion).ok_or_else(|| {
                planner_error(
                    "static continuation fusion body ownership was derived from an identity with \
                     no outstanding claim, so no validated redirect authorizes it",
                )
            })?;
            let body = claim.producer_body();
            let producer = claim.producer_owner();

            // Exactly one emittable unit matches BOTH the producer function and
            // the producer body. Matching on either alone would let a unit that
            // shares a body with another, or another unit of the same function,
            // stand in for the one whose body is being taken over.
            let mut matching = units
                .iter()
                .filter(|unit| unit.function() == producer && unit.body_occurrence() == body);
            let Some(unit) = matching.next() else {
                return Err(planner_error(
                    "a static continuation fusion claims a producer body that no executable unit \
                     of that producer declares",
                ));
            };
            if matching.next().is_some() {
                return Err(planner_error(
                    "two executable units declare one static continuation fusion's producer body \
                     and function, so the body being taken over is ambiguous",
                ));
            }

            // A retained closure body, and nothing else. This is also what
            // refuses the scheduling-entry route: `SchedulingEntry` and
            // `CallableDeclaration` are distinct arms of the same closed enum,
            // so an entry unit cannot pass here and no second branch restating
            // that could ever fire.
            if !matches!(unit.definition(), AbiUnitDefinition::ClosureBody { .. }) {
                return Err(planner_error(
                    "a static continuation fusion's producer body is not a retained closure body, \
                     so it is a scheduling entry or callable declaration whose standalone \
                     definition nothing replaces",
                ));
            }

            // No continuation-template disposition already owns it.
            if template_only.contains(&body) {
                return Err(planner_error(
                    "a static continuation fusion claims a producer body a generated continuation \
                     context already supersedes",
                ));
            }

            // Exactly one claim owns this body and producer.
            if scratch
                .insert(body, FusionOwnedBody { producer, fusion })
                .is_some()
            {
                return Err(planner_error(
                    "two static continuation fusion claims own one producer body, so which fused \
                     definition lowers it is undetermined",
                ));
            }
            owned.insert(fusion);

            // Treating the claim's already-validated redirect as superseded must
            // leave NO standalone route into this producer. A surviving second
            // static-body edge, an ordinary declaration call, an unretargeted
            // raw-worker route, or an entry route all mean the body still has a
            // caller that the fused definition does not serve, and removing its
            // standalone definition would leave that caller unresolvable.
            let redirect = claim.redirect();
            let surviving = edges.iter().any(|edge| {
                edge.callee() == producer
                    && !(edge.caller() == redirect.caller()
                        && edge.callee() == redirect.callee()
                        && edge.callee_origin() == redirect.callee_origin()
                        && edge.call_site_origin() == redirect.call_site_origin()
                        && edge.kind() == redirect.kind())
            });
            if surviving {
                return Err(planner_error(
                    "a static continuation fusion's producer keeps a standalone route after its \
                     claimed invocation is superseded, so removing the producer's own definition \
                     would leave that route unresolvable",
                ));
            }
        }

        // **The transaction. Every fallible refusal is above this line.**
        //
        // The ledger is checked BEFORE the plan's map moves, because the two
        // objects are not type-bound to each other: a ledger already spent on
        // one plan can be handed to an equivalent second plan whose own scratch
        // map validates perfectly. Recording after the move meant that case
        // mutated the second plan and then returned `Err` — leaving it with a
        // populated ownership map, a narrowed executable-unit population and a
        // narrowed edge population, all from a call that reported failure.
        //
        // No caller convention can exclude that, so it is excluded here by
        // ordering: the check below is the last thing that can fail, and the two
        // commits after it are infallible.
        // ---- `D3` — MINT THE COMPOSED EDGES. Ruled `evt_1t3f4e8100rb5`.
        //
        // Two layers per fusion, each selected by one of the key's checked IH
        // bindings and by nothing else. The bindings are the relation the
        // grounding turn established: the consumer binding names the outer
        // frame, the producer-argument binding names the inner one, and they
        // differ precisely BECAUSE they are the two composition layers.
        //
        // Every refusal is above the transaction line, and each names a
        // distinct way the join can be wrong rather than collapsing them into
        // one "could not resolve".
        let mut composed: BTreeMap<ContinuationCallIdentity, FusionComposedEdge> = BTreeMap::new();
        let mut outer_realizations: BTreeMap<
            ContinuationCallIdentity,
            FusionOwnedOuterRealization,
        > = BTreeMap::new();
        let specializations = self.continuation_units()?;
        let specialization_calls = self.continuation_calls()?;
        for view in self.continuation_fusions()? {
            let fusion = view.id();
            if !owned.contains(&fusion) {
                continue;
            }
            let key = view.key();
            for (layer, frame, owner) in [
                (
                    FusionCompositionLayer::Outer,
                    key.consumer_binding.frame_origin,
                    key.consumer_owner,
                ),
                (
                    FusionCompositionLayer::Inner,
                    key.producer_argument_binding.frame_origin,
                    key.producer_owner,
                ),
            ] {
                let mut matching = specializations
                    .iter()
                    .filter(|unit| {
                        unit.continuation_origin() == frame && unit.consumer_owner() == owner
                    });
                let Some(unit) = matching.next() else {
                    return Err(planner_error(
                        "a static continuation fusion's checked binding names a continuation \
                         frame that no generated specialization eliminates, so the edge it would \
                         compose does not exist",
                    ));
                };
                if matching.next().is_some() {
                    return Err(planner_error(
                        "two generated continuation specializations answer one static \
                         continuation fusion's checked binding frame and owner, so which edge it \
                         composes is ambiguous",
                    ));
                }

                // The target's UNIQUE edge. Uniqueness is the injective
                // call-target law above, so this reads a fact the closure
                // validator already refused any violation of -- it does not
                // re-derive it.
                let mut edges = specialization_calls
                    .iter()
                    .filter(|call| call.target() == unit.id());
                let Some(call) = edges.next() else {
                    return Err(planner_error(
                        "a static continuation fusion composes a specialization no exact call \
                         reaches, so there is no edge to compose at",
                    ));
                };
                if edges.next().is_some() {
                    return Err(planner_error(
                        "a composed continuation specialization has more than one exact planned \
                         edge, which the injective call-target law forbids",
                    ));
                }
                if call.emission_owner() != ContinuationEmissionOwner::Predeclared(owner) {
                    return Err(planner_error(
                        "a composed continuation edge is emitted by an owner other than the one \
                         the fusion's checked binding names, so composing it would lower a \
                         selected body in a function that does not hold its operands",
                    ));
                }
                let identity = self
                    .continuation_call_binding_for(
                        call.producer_construct_origin(),
                        call.continuation_origin(),
                        call.producer_alternative(),
                        call.recursive_position(),
                    )?
                    .ok_or_else(|| {
                        planner_error(
                            "a composed continuation edge has no binding under its own four-field \
                             selector, so the identity the composition is keyed by cannot be named",
                        )
                    })?;
                if identity.target() != unit.id() {
                    return Err(planner_error(
                        "a composed continuation edge's re-resolved identity names a different \
                         specialization than the edge it was read from",
                    ));
                }
                // ---- `D3` — THE TERNARY ROUTE. `evt_6bm54j10w1n88`.
                //
                // Both layers are SELECTED identically, by the checked binding
                // and the exact call-target bijection above. What differs is
                // what realizes them: `Inner` is composed locally at its call
                // edge, and `Outer` is already realized by the fusion-owned body
                // and must never reach a call seat at all.
                match layer {
                    FusionCompositionLayer::Inner => {
                        if composed
                            .insert(
                                identity,
                                FusionComposedEdge {
                                    fusion,
                                    target: unit.id(),
                                    emission_owner: call.emission_owner(),
                                    consumer_continuation_origin: frame,
                                    producer_construct_origin: call.producer_construct_origin(),
                                    layer,
                                },
                            )
                            .is_some()
                        {
                            return Err(planner_error(
                                "one exact continuation call identity is composed twice, so which \
                                 fused region owns the edge is undetermined",
                            ));
                        }
                    }
                    FusionCompositionLayer::Outer => {
                        // ---- THE CLOSURE TRIPLE, APPLIED AFTER SELECTION.
                        //
                        // Every equality below is checked against the region
                        // claim's own recorded coordinates. None of them chose
                        // this identity -- the checked consumer binding and the
                        // call-target bijection did that, above -- so a
                        // disagreement here is a real refusal rather than a
                        // filter that quietly admitted the wrong edge.
                        let claim = ledger.claim(fusion).ok_or_else(|| {
                            planner_error(
                                "a body-owning static continuation fusion has no region claim to \
                                 close its outer realization against, so nothing says which body \
                                 realized the outer identity",
                            )
                        })?;
                        if unit.continuation_origin() != claim.continuation_origin() {
                            return Err(planner_error(
                                "a fusion-owned outer realization names a continuation origin the \
                                 region claim does not, so the body that would realize it \
                                 eliminates a different frame",
                            ));
                        }
                        let selected_case_body = self.semantic.child_origin(
                            unit.continuation_origin(),
                            1 + unit.producer_alternative() as usize,
                        )?;
                        if selected_case_body != claim.selected_case_body() {
                            return Err(planner_error(
                                "a fusion-owned outer realization's selected case body is not the \
                                 one the region claim records, so the owned body emits a different \
                                 body than the identity names",
                            ));
                        }
                        if unit.worker_body_origin() != claim.producer_body()
                            || unit.worker_body_origin() != claim.redirect().callee_origin()
                        {
                            return Err(planner_error(
                                "a fusion-owned outer realization's worker body disagrees with the \
                                 region claim's producer body or its redirect callee, so the \
                                 identity is not the one that body realizes",
                            ));
                        }
                        if call.emission_owner()
                            != ContinuationEmissionOwner::Predeclared(claim.consumer_owner())
                        {
                            return Err(planner_error(
                                "a fusion-owned outer realization is emitted by an owner other \
                                 than the region claim's consumer, so it would be realized in a \
                                 function that does not hold the fused region",
                            ));
                        }
                        // The producer body is actually FUSION-OWNED, and by
                        // THIS fusion. Read from the scratch map this call is
                        // building, which is the population about to be
                        // installed -- not from the already-installed one, which
                        // would answer for a previous compile.
                        match scratch.get(&claim.producer_body()) {
                            Some(owned_body) if owned_body.fusion == fusion => {}
                            Some(_) => {
                                return Err(planner_error(
                                    "a fusion-owned outer realization's producer body is owned by \
                                     a different fusion, so two regions would each claim to \
                                     realize it",
                                ));
                            }
                            None => {
                                return Err(planner_error(
                                    "a fusion-owned outer realization names a producer body no \
                                     fusion owns, so nothing emits the body the identity is \
                                     supposed to be realized by",
                                ));
                            }
                        }
                        if outer_realizations
                            .insert(
                                identity,
                                FusionOwnedOuterRealization {
                                    fusion,
                                    target: unit.id(),
                                    emission_owner: call.emission_owner(),
                                    consumer_continuation_origin: frame,
                                    selected_case_body,
                                    producer_body: claim.producer_body(),
                                },
                            )
                            .is_some()
                        {
                            return Err(planner_error(
                                "one exact continuation call identity is realized as a \
                                 fusion-owned outer twice, so which region realized it is \
                                 undetermined",
                            ));
                        }
                    }
                }
            }
        }

        // ---- `D3` — THE FUSION-SCOPED JOIN, validated as ONE structure rather
        // ---- than by origin coincidence. `evt_6kn9ckdnbf0ph` §2, carried to the
        // ---- ternary population by `evt_6bm54j10w1n88`.
        //
        // Each object carries one part and none is widened to hold another's:
        // `FusionComposedEdge` says which exact edge is locally composed (`I`),
        // `FusionOwnedOuterRealization` says which planned identity the owned
        // body already realized (`R`), and `FusionRegionClaim` says which checked
        // call consumes the worker produced inside the composition. The ordinary
        // consuming call is deliberately in none of them -- it has no
        // `ContinuationCallIdentity` and is not a specialization target.
        //
        // So the only join asserted is `fusion == claim.fusion`, exactly once per
        // layer, from both sides.
        for fusion in composed
            .values()
            .map(|edge| edge.fusion)
            .chain(outer_realizations.values().map(|realization| realization.fusion))
        {
            if !owned.contains(&fusion) {
                return Err(planner_error(
                    "a fusion-local realization names a fusion that owns no producer body, so the                      claim half of the composition join does not exist",
                ));
            }
        }
        for (name, population) in [
            ("inner", composed.values().map(|edge| edge.fusion).collect::<Vec<_>>()),
            (
                "outer",
                outer_realizations.values().map(|r| r.fusion).collect::<Vec<_>>(),
            ),
        ] {
            let mut seen = BTreeSet::new();
            for fusion in &population {
                if !seen.insert(*fusion) {
                    return Err(planner_error(format!(
                        "one static continuation fusion has two {name} realizations, so its two                          ruled selections are not distinct"
                    )));
                }
            }
            for fusion in owned.iter() {
                if !seen.contains(fusion) {
                    return Err(planner_error(format!(
                        "a body-owning static continuation fusion has no {name} realization, so                          only one of its two ruled layers is realized and the other's edge is                          left expecting a call that was omitted"
                    )));
                }
            }
        }

        // ---- `D3` — THE EXACT TERNARY PARTITIONS `P = O ⊎ I ⊎ R` AND
        // ---- `T = O_t ⊎ I_t ⊎ R_t`. `evt_6bm54j10w1n88`.
        //
        // ⚠ **The binary form this replaces was VACUOUS, and the repair is not
        // the ternary rewrite alone.** It defined `residual = planned \ fused`
        // four lines above and then asserted `residual ∩ fused = ∅` and
        // `residual ∪ fused = planned`. Set difference makes the first true by
        // construction, and the second is exactly `fused ⊆ planned`, already
        // refused above -- so **neither `Err` was reachable**, and nothing read
        // `residual` afterwards, so nothing checked that the blessed population
        // was the consumed one. Found by the adversary, not by this seat.
        //
        // Two things fix it, and both are needed:
        //
        // 1. **`O` is obtained from the CONSUMERS' OWN FUNCTION**, not
        //    re-derived here. `ordinary_identities_of` is the single derivation
        //    `ordinary_continuation_call_identities` also calls, so "validated"
        //    and "consumed" cannot drift -- not because a check compares them,
        //    but because there is one derivation to disagree with.
        // 2. **`I ∩ R = ∅` is now LIVE.** Under the binary form there was one
        //    fused map and its disjointness from its own complement was
        //    algebra. `I` and `R` are built by two selectors over two layers and
        //    can name one identity; this is the refusal that catches it.
        //
        // **Stated honestly: the COVERAGE half remains implied** by the two
        // subset refusals above plus the definition of `O`. It is asserted
        // because it is the law, and because the subset refusals are what it
        // depends on -- if either is ever relaxed, this becomes the check that
        // still names the property.
        let planned_identities = self.continuation_call_identities()?;
        let inner_identities = composed.keys().cloned().collect::<BTreeSet<_>>();
        let outer_identities = outer_realizations.keys().cloned().collect::<BTreeSet<_>>();
        for (name, population) in [("inner", &inner_identities), ("outer", &outer_identities)] {
            if !population.is_subset(&planned_identities) {
                return Err(planner_error(format!(
                    "an {name} fusion-local identity is not in the exact planned call population,                      so that class is not a subset of the population it partitions"
                )));
            }
        }
        if !inner_identities.is_disjoint(&outer_identities) {
            return Err(planner_error(
                "a continuation call identity is both locally composed and realized by a                  fusion-owned body, so one planned edge would be realized twice by two different                  mechanisms",
            ));
        }
        let ordinary_identities =
            Self::ordinary_identities_of(&planned_identities, &composed, &outer_realizations);
        if ordinary_identities
            .union(&inner_identities)
            .cloned()
            .collect::<BTreeSet<_>>()
            .union(&outer_identities)
            .cloned()
            .collect::<BTreeSet<_>>()
            != planned_identities
        {
            return Err(planner_error(
                "the ordinary, locally composed and fusion-owned identity populations are not a                  partition of the exact planned population",
            ));
        }

        let planned_targets = specializations.iter().map(|unit| unit.id()).collect::<BTreeSet<_>>();
        let inner_targets = composed.values().map(|edge| edge.target).collect::<BTreeSet<_>>();
        let outer_targets = outer_realizations
            .values()
            .map(|realization| realization.target)
            .collect::<BTreeSet<_>>();
        for (name, population) in [("inner", &inner_targets), ("outer", &outer_targets)] {
            if !population.is_subset(&planned_targets) {
                return Err(planner_error(format!(
                    "an {name} fusion-local target is outside the planned unit population, so                      that class's target range is not a subset of the population it partitions"
                )));
            }
        }
        if !inner_targets.is_disjoint(&outer_targets) {
            return Err(planner_error(
                "a continuation specialization target is both a local composition target and a                  fusion-owned realization target, so one unit's selected body would be emitted                  twice",
            ));
        }
        let ordinary_targets =
            Self::ordinary_targets_of(&planned_targets, &composed, &outer_realizations);
        if ordinary_targets
            .union(&inner_targets)
            .cloned()
            .collect::<BTreeSet<_>>()
            .union(&outer_targets)
            .cloned()
            .collect::<BTreeSet<_>>()
            != planned_targets
        {
            return Err(planner_error(
                "the ordinary, locally composed and fusion-owned target populations are not a                  partition of the planned specialization population",
            ));
        }

        ledger.check_body_owned(&owned)?;
        self.fusion_owned_bodies = scratch;
        self.fusion_composed_calls = composed;
        self.fusion_outer_realizations = outer_realizations;
        self.fusion_bodies_installed = true;
        ledger.commit_body_owned(owned);
        Ok(())
    }



    /// **`D3` — `R`, the fusion-owned outer realizations.**
    pub(in crate::cranelift_backend) fn fusion_outer_realizations(
        &self,
    ) -> &BTreeMap<ContinuationCallIdentity, FusionOwnedOuterRealization> {
        &self.fusion_outer_realizations
    }


    /// **`D3` — the composed edge for this exact call identity, if any.**
    ///
    /// **Probed by WHOLE identity.** An identity with no record takes the
    /// existing `DirectCall` path unchanged; there is no target, body, owner or
    /// origin question asked here, and no incoming-domain scan anywhere.
    pub(in crate::cranelift_backend) fn fusion_composed_edge(
        &self,
        identity: &ContinuationCallIdentity,
    ) -> Option<&FusionComposedEdge> {
        self.fusion_composed_calls.get(identity)
    }


    /// Every composed edge the planner minted, for the transport-instance
    /// closeout to check consumption against.
    pub(in crate::cranelift_backend) fn fusion_composed_edges(
        &self,
    ) -> &BTreeMap<ContinuationCallIdentity, FusionComposedEdge> {
        &self.fusion_composed_calls
    }



    /// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` — install one compile's fusion
    /// identity plane and the generated-definition ABI it determines.**
    ///
    /// The one writer of both, and it writes them together on purpose: a plane
    /// without its arena and an arena without its plane are each individually
    /// consistent and jointly meaningless, and [`Self::continuation_fusions`]
    /// below is a join over both.
    ///
    /// **The producer's parameter run is READ, never chosen.** The redirected
    /// invocation keeps passing exactly the operands it passed before, so the
    /// producer unit's own descriptor is the authority for how many there are.
    /// A fusion whose producer owner has no descriptor is a fusion whose
    /// producer has no declared ABI, which is refused rather than defaulted.
    pub(in crate::cranelift_backend) fn install_static_continuation_fusions(
        &mut self,
        fusions: StaticContinuationFusionPlan,
    ) -> Result<(), CraneliftBackendError> {
        if !self.static_continuation_fusions.is_empty() {
            return Err(planner_error(
                "a static continuation fusion plane may be installed exactly once",
            ));
        }
        let mut projections = Vec::new();
        projections.try_reserve_exact(fusions.len()).map_err(|_| {
            planner_capacity_error("static continuation fusion projection allocation failed")
        })?;
        for (position, key) in fusions.installed_keys().iter().enumerate() {
            let id = StaticContinuationFusionId(u32::try_from(position).map_err(|_| {
                planner_capacity_error("static continuation fusion identity exhausted")
            })?);
            let producer = self
                .abi
                .descriptors
                .iter()
                .find(|descriptor| descriptor.function == key.producer_owner)
                .ok_or_else(|| {
                    planner_error(
                        "a static continuation fusion's producer owner has no ABI descriptor, so \
                         the operand run its redirected invocation already passes is unknown",
                    )
                })?;
            // `R3` -- the producer-capture run and the continuation-capture
            // suffix are different axes. The former is read from the selected
            // producer descriptor here; the latter comes from
            // `key.continuation_inputs` below and becomes `claim.inputs()`.
            //
            // This admitted population has no producer captures. A non-empty
            // run is a new ABI disposition: it refuses before fusion-ABI
            // installation rather than being folded into the ordinary
            // parameter run or the continuation-capture suffix.
            let producer_captures = producer.header.captures;
            #[cfg(test)]
            let producer_captures = if FUSION_PRODUCER_CAPTURE_MUTATION.with(Cell::get)
                == FusionProducerCaptureMutation::ForceNonEmptyAfterSelection
            {
                // Population-side mutation only. The real producer descriptor
                // has already been selected; no input or authority is added.
                producer_captures.max(1)
            } else {
                producer_captures
            };
            if producer_captures != 0 {
                return Err(planner_error(
                    "a static continuation fusion's producer capture run is non-empty; this ABI \
                     disposition cannot be folded into the fused invocation's ordinary \
                     parameters or continuation-input capture suffix",
                ));
            }
            projections.push(abi::PlannedStaticContinuationFusionAbi {
                id,
                producer_parameters: producer.header.parameters,
                continuation_inputs: &key.continuation_inputs,
            });
        }
        abi::install_static_continuation_fusion_abi(&mut self.abi, &projections)?;
        drop(projections);
        self.static_continuation_fusions = fusions;
        Ok(())
    }


    /// **`D2f` — every installed fused region, joined ABI-to-plane BY IDENTITY.**
    ///
    /// Revalidates plane/ABI agreement the way [`Self::continuation_contexts`]
    /// does and for the same reason: the join indexes descriptors by the id they
    /// declare, so an identical reordering of both sides does not pass.
    pub(in crate::cranelift_backend) fn continuation_fusions(
        &self,
    ) -> Result<Vec<StaticContinuationFusionView<'_>>, CraneliftBackendError> {
        if self.abi.fusion_descriptors.len() != self.static_continuation_fusions.len() {
            return Err(planner_error(
                "static continuation fusion ABI descriptor count disagrees with the installed \
                 fusion plane",
            ));
        }
        let mut by_id: BTreeMap<
            StaticContinuationFusionId,
            &abi::AbiStaticContinuationFusionDescriptor,
        > = BTreeMap::new();
        for descriptor in &self.abi.fusion_descriptors {
            let AbiUnitDefinition::StaticContinuationFusion { fusion } = descriptor.definition
            else {
                return Err(planner_error(
                    "a static continuation fusion ABI descriptor declares another class's unit \
                     definition",
                ));
            };
            if by_id.insert(fusion, descriptor).is_some() {
                return Err(planner_error(
                    "two static continuation fusion ABI descriptors declare the same identity",
                ));
            }
        }
        self.static_continuation_fusions
            .installed_keys()
            .iter()
            .enumerate()
            .map(|(position, key)| {
                let id = StaticContinuationFusionId(u32::try_from(position).map_err(|_| {
                    planner_capacity_error("static continuation fusion identity exhausted")
                })?);
                let descriptor = *by_id.get(&id).ok_or_else(|| {
                    planner_error(
                        "an installed static continuation fusion has no ABI descriptor declaring \
                         its identity",
                    )
                })?;
                let planned = self
                    .static_continuation_fusions
                    .descriptor_for(id)
                    .ok_or_else(|| {
                        planner_error(
                            "an installed static continuation fusion has no planner descriptor",
                        )
                    })?;
                let slots = dense_slice(&self.abi.fusion_slots, descriptor.slots).ok_or_else(
                    || planner_error("static continuation fusion slot range is outside the plane"),
                )?;
                let inputs = dense_slice(&self.abi.fusion_inputs, descriptor.inputs).ok_or_else(
                    || planner_error("static continuation fusion input range is outside the plane"),
                )?;
                if inputs.len() != key.continuation_inputs.len() {
                    return Err(planner_error(
                        "static continuation fusion input authority count disagrees with the \
                         complete key's ordered projection",
                    ));
                }
                Ok(StaticContinuationFusionView {
                    id,
                    key,
                    planned,
                    header: descriptor.header,
                    slots,
                    inputs,
                })
            })
            .collect()
    }

}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::super::super::tests::*;
    use super::super::super::*;
    use super::super::*;
    use super::*;
    #[allow(unused_imports)]
    use crate::{RuntimeComputationalMatchCase, RuntimeMatchCase, RuntimeTrap, RuntimeTrapCode, RuntimeValue};

    /// Walk down through any checked wrappers to the occurrence they carry.
    ///
    /// The wrappers are real occurrences in the semantic tree, so the checked
    /// form's coordinates are NOT the unmarked form's. This descends by the
    /// wrapper's own body edge rather than assuming a depth.
    #[cfg(test)]
    fn d2g_through_wrappers(
        plan: &StaticTransitionPlan<'_>,
        mut origin: StaticOriginId,
    ) -> Result<StaticOriginId, CraneliftBackendError> {
        loop {
            match plan.planned_occurrence_expr(origin)? {
                RuntimeExpr::CheckedSubcontinuationFrame { .. }
                | RuntimeExpr::CheckedComputationalIHSlots { .. }
                | RuntimeExpr::CheckedComputationalIHInvocation { .. }
                | RuntimeExpr::CheckedRecursiveInvocation { .. }
                | RuntimeExpr::CheckedJoinSite { .. } => {
                    origin = plan.semantic.child_origin(origin, 0)?;
                }
                _ => return Ok(origin),
            }
        }
    }

    /// Build the plane for one cause: mutated source, correct plan.
    #[cfg(test)]
    fn d2j_plane_under(
        cause: D2jCause,
    ) -> Result<StaticContinuationFusionPlan, CraneliftBackendError> {
        // Through the SHARED constructor, so this plane and the full-compile
        // gate are measurements of one witness rather than two that resemble
        // each other.
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let mut declarations = BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
        build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
    }

    /// Which ruled preflight rule a refusal reached, by its own message.
    ///
    /// Classified rather than reduced to `is_err`: every row below moves one
    /// operand and claims a **specific** rule fired, so a row that accepted any
    /// planner invariant would pass when a different and possibly weaker rule
    /// answered — including one that regressed into a coarser refusal upstream.
    #[cfg(test)]
    fn d2f_refusal_of(result: Result<FusionRegionClaimLedger, CraneliftBackendError>) -> String {
        match result {
            Ok(_) => "issued".to_string(),
            Err(CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(message))) => {
                for cause in [
                    FusionClaimRefusal::Identity,
                    FusionClaimRefusal::SelectorEdge,
                    FusionClaimRefusal::InvocationTriple,
                    FusionClaimRefusal::SelfRedirection,
                    FusionClaimRefusal::BinderAgreement,
                    FusionClaimRefusal::BinderBodyResolution,
                    FusionClaimRefusal::InputAvailability,
                    FusionClaimRefusal::ResultLane,
                    FusionClaimRefusal::OverlappingClaim,
                ] {
                    if message == cause.detail() {
                        return format!("{cause:?}");
                    }
                }
                // The selector's own refusals are raised by
                // `fusion_redirect_target` and are not spelled by this enum;
                // they are the absent/ambiguous/declaration-kind family and are
                // reported under the rule they serve.
                if message.contains("no edge to redirect")
                    || message.contains("selects more than one emittable")
                    || message.contains("rather than a static body edge")
                {
                    return format!("{:?}", FusionClaimRefusal::SelectorEdge);
                }
                format!("other planner invariant: {message}")
            }
            Err(error) => format!("other error: {error:?}"),
        }
    }

    /// Preflight the witness with the key perturbed and the binder-to-body rule
    /// suppressed, so a row can read what EVERY OTHER rule did with that key.
    #[cfg(test)]
    fn d2f_preflight_exact_without_resolution(
        perturb: impl FnOnce(&mut Vec<StaticContinuationFusionKey>),
    ) -> Result<FusionRegionClaimLedger, CraneliftBackendError> {
        set_binder_body_resolution_suppressed(true);
        let result = d2f_preflight_exact(perturb);
        set_binder_body_resolution_suppressed(false);
        result
    }

    /// The one fixture for `AC-1` and `AC-2`: a hypothesis reached three ways
    /// and an ordinary child sitting beside it in the same scope.
    ///
    /// Inside the `Node` case the environment is `[IH, child0, ..]`. The `Let`
    /// pushes one binder and the nested `Match` case pushes another, so the same
    /// hypothesis is `Var(0)`, then `Var(1)`, then `Var(2)` at three different
    /// depths -- and `Var(3)` beside the last one is the ordinary child.
    ///
    /// One fixture rather than three, so the ordinary child is drawn from the
    /// same declaration as the hypotheses and the pair cannot differ by
    /// something other than its role.
    #[cfg(test)]
    fn d2e_indirection_fixture() -> RuntimeExpr {
        let unit = || RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        };
        let trap = || RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "D2e indirection fixture".to_string(),
        };
        RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::D2eIn::Node".to_string(),
                args: vec![RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: vec!["unit".to_string()],
                    body: Box::new(RuntimeExpr::Construct {
                        constructor: "ctor:fixture::D2eIn::Leaf".to_string(),
                        args: Vec::new(),
                    }),
                }],
            }),
            cases: vec![
                RuntimeComputationalMatchCase {
                    constructor: "ctor:fixture::D2eIn::Node".to_string(),
                    argument_binders: 1,
                    recursive_positions: vec![0],
                    body: RuntimeExpr::Let {
                        // env here is [IH, child0, ..] -- the DIRECT reference.
                        value: Box::new(RuntimeExpr::Var(0)),
                        body: Box::new(RuntimeExpr::Match {
                            // env is now [let, IH, child0, ..] -- through a Let.
                            scrutinee: Box::new(RuntimeExpr::Var(1)),
                            cases: vec![RuntimeMatchCase {
                                constructor: "ctor:fixture::D2eIn::Wrap".to_string(),
                                binders: 1,
                                // env is now [match, let, IH, child0, ..].
                                body: RuntimeExpr::Construct {
                                    constructor: "ctor:fixture::D2eIn::Pair".to_string(),
                                    args: vec![RuntimeExpr::Var(2), RuntimeExpr::Var(3)],
                                },
                            }],
                            default: trap(),
                        }),
                    },
                },
                RuntimeComputationalMatchCase {
                    constructor: "ctor:fixture::D2eIn::Leaf".to_string(),
                    argument_binders: 0,
                    recursive_positions: Vec::new(),
                    body: unit(),
                },
            ],
            default: trap(),
        }
    }

    /// `D2j` — a fusion-reaching witness with a genuinely NON-EMPTY ordered
    /// input projection, and its count.
    ///
    /// `D2h`'s witness reaches a fusion candidate and projects **zero** ordered
    /// inputs, which is why its `continuation_inputs` identity class could not
    /// be exercised there. This witness is the same body inside a
    /// two-parameter `LexicalClosure`, so the consumer's owning unit has an
    /// entry ABI to project -- one structural difference, and it is the reason
    /// the projection becomes non-empty.
    ///
    /// Both are measured in one test, so the count is a comparison rather than
    /// a bare number: **0 on the bare witness, 2 on this one.** A lone "2"
    /// would not show that the parameterisation is what produced it.
    ///
    /// The projection's members are pinned, not just its length -- a count
    /// could be reached by an unrelated projection of the same size.
    ///
    /// The earlier generic census (2360 projections across the corpus) is
    /// negative evidence only: it showed the machinery yields non-empty runs and
    /// is **not** promoted to a fusion witness here.
    /// **`D2f` Deliverables 1 and `AC-4` — the fused region's frame is derived
    /// from the producer's declared operands and the key's projected run, and
    /// an activation carrier arriving as an input is REFUSED.**
    ///
    /// The frame contract and the refusal are one control because `AC-4`'s named
    /// failure mode is a check that passes because nothing reached it. The
    /// mutated row's refusal is asserted in the **same** assertion as the
    /// unmutated row's non-zero descriptor and its frame, so "the activation
    /// carrier was refused" cannot be read off a compile that installed nothing.
    ///
    /// The mutation is compile-preserving and reaching: it moves one operand —
    /// the carrier on the key's first projected input — and enters through the
    /// production installer, not through the carrier gate directly.
    ///
    /// **MEASURED:** on the checked applied `Exact` witness the installer builds
    /// one fusion descriptor whose frame is one parameter (the producer unit's
    /// own declared run), two captures (the key's projected inputs), and the
    /// four convention slots; and the same installer refuses when one projected
    /// input's carrier is an activation word.
    /// **CLAIMED:** no activation, cursor, selection or unwind carrier can enter
    /// a fused region's input lane, and the fused frame's parameter run is the
    /// producer's rather than a count chosen here.
    /// **THE GAP:** this pins the **arena and its gate**. It pins no emission:
    /// no generated definition is built, no edge is redirected, and the
    /// production path does not call this installer, so
    /// `observed_fusion_definition_count` still reads `0` on every compile.
    #[test]
    fn d2f_1_the_fused_frame_is_the_producers_run_and_refuses_an_activation_input() {
        /// Install one plane into the witness's own plan and report the frame.
        fn install(
            mutate: Option<AbiCarrier>,
        ) -> Result<(usize, u32, u32, Vec<(AbiSlotKind, AbiCarrier)>), CraneliftBackendError>
        {
            let (entry, declaration, oriented) = d2j_checked_fixture_under(D2jCause::Exact);
            let mut declarations = BTreeMap::new();
            declarations.insert(D2J_DECLARATION, &declaration);
            let mut plan =
                plan_static_transition_graph(&entry, &declarations).expect("the twin plans");
            let resolved =
                build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
                    .expect("the twin resolves a plane");
            let plane = match mutate {
                None => resolved,
                Some(carrier) => {
                    // Within the tuple returned below, the operand moved and
                    // nothing else. The mutated plan itself is synthetic
                    // `::default()` and carries no enumerator input count; a
                    // `walked` difference would be attributable to that
                    // synthetic branch, not to the carrier. The key is otherwise
                    // the one production derived, so a refusal here is
                    // attributable to the carrier and not to a hand-built key.
                    let mut key = resolved.installed_keys()[0].clone();
                    key.continuation_inputs[0].carrier = carrier;
                    let mut mutated = StaticContinuationFusionPlan::default();
                    mutated.intern(key).expect("the mutated key interns");
                    mutated
                }
            };
            plan.install_static_continuation_fusions(plane)?;
            let views = plan.continuation_fusions()?;
            let view = &views[0];
            Ok((
                views.len(),
                view.header().parameters,
                view.header().captures,
                view.slots()
                    .iter()
                    .map(|slot| (slot.kind, slot.carrier))
                    .collect(),
            ))
        }

        let unmutated = install(None).expect("the unmutated twin installs");
        // Every activation carrier, and the store handle, each refused on its
        // own row: a single `ControlWord` row would leave the other three
        // admitted-by-omission, which is the shape the exhaustive match exists
        // to make impossible.
        //
        // Each row reports WHICH refusal it reached, not merely that it failed.
        // The two reasons are different claims — an activation word crossing
        // inward, and a durable lane arriving as an input — so a row that
        // collapsed them would pass if the gate answered the wrong one, and
        // would also pass on an unrelated planner invariant.
        let classify = |carrier: AbiCarrier| match install(Some(carrier)) {
            Ok(_) => "admitted",
            Err(CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(message))) => {
                if message.contains("names an activation carrier") {
                    "activation"
                } else if message.contains("names a persistent store handle") {
                    "store"
                } else {
                    "other planner invariant"
                }
            }
            Err(_) => "other error",
        };
        let refused: Vec<(AbiCarrier, &str)> = [
            AbiCarrier::ResultWord,
            AbiCarrier::ControlWord,
            AbiCarrier::TrapWord,
            AbiCarrier::StoreHandle,
        ]
        .into_iter()
        .map(|carrier| (carrier, classify(carrier)))
        .collect();
        // And the admitted sibling, so the refusals above are not merely "this
        // installer refuses everything".
        let admitted = install(Some(AbiCarrier::GroundValueCarrier))
            .expect("an ordinary ground-value input is admitted");

        assert_eq!(
            (
                unmutated,
                refused,
                (admitted.0, admitted.1, admitted.2),
            ),
            (
                (
                    1,
                    1,
                    2,
                    vec![
                        (AbiSlotKind::Parameter, AbiCarrier::ValueWord),
                        (AbiSlotKind::Capture, AbiCarrier::ValueWord),
                        (AbiSlotKind::Capture, AbiCarrier::ValueWord),
                        (AbiSlotKind::Result, AbiCarrier::ResultWord),
                        (AbiSlotKind::Control, AbiCarrier::ControlWord),
                        (AbiSlotKind::Trap, AbiCarrier::TrapWord),
                        (AbiSlotKind::Store, AbiCarrier::StoreHandle),
                    ],
                ),
                vec![
                    (AbiCarrier::ResultWord, "activation"),
                    (AbiCarrier::ControlWord, "activation"),
                    (AbiCarrier::TrapWord, "activation"),
                    (AbiCarrier::StoreHandle, "store"),
                ],
                (1, 1, 2),
            ),
            "the fused frame is the producer unit's one declared parameter plus the key's two \
             projected inputs plus the convention tail; each activation carrier and the store \
             handle is refused as an input while an ordinary ground-value carrier is admitted -- \
             and the refusals share this assertion with a non-zero installed descriptor so none \
             of them can hold because nothing was installed"
        );
    }

    /// **`D2f` — the region claim is issued exactly once for the canonical
    /// witness, and each ruled preflight rule REFUSES on its own moved operand.**
    ///
    /// The positive and the refusals share one assertion for the reason `AC-4`'s
    /// control does: "the triple was checked" must not be readable off a run
    /// that issued no claim at all.
    ///
    /// **MEASURED:** on the checked applied `Exact` witness, preflight issues one
    /// claim whose members are the key's own — producer owner 2, consumer owner
    /// 3, producer body 37, and a seat that is the redirected edge's call site;
    /// and independently moving the consumer owner, the callee entry, the
    /// admitted result root, and the recursive position each refuses at its
    /// named rule, as does a second key sharing this one's suffix.
    /// **CLAIMED:** no fused region is claimed whose redirect edge is not the
    /// producer invocation its key names, whose checked binders disagree with
    /// that key, or whose region overlaps another claim's.
    /// **THE GAP:** this pins **preflight**. It pins no emission: no definition
    /// is built, no edge is redirected, and no production compile calls this, so
    /// nothing here establishes that a claim is ever consumed by real codegen.
    /// The affine and closeout behaviour is pinned separately below, on the
    /// ledger rather than on a compile.
    #[test]
    fn d2f_2_the_region_claim_is_issued_once_and_each_ruled_rule_refuses() {
        let issued = d2f_preflight_exact(|_| ()).expect("the unperturbed witness issues a claim");
        assert_eq!(issued.planned().len(), 1, "one installed region, one claim");
        let id = *issued.planned().iter().next().expect("the identity");
        let claim = issued.claim(id).expect("its claim is outstanding");

        // The claim's members are the key's, and the seat is the redirected
        // edge's call site rather than the consuming call — the distinction the
        // accessor exists to keep.
        let members = (
            claim.producer_owner(),
            claim.consumer_owner(),
            claim.producer_body(),
            claim.emission_owner(),
            claim.seat() == claim.redirect().call_site_origin(),
            claim.seat() != claim.consuming_call(),
            claim.inputs().len(),
        );

        // ⇒ The two source authorities are distinct here as a CONSEQUENCE of the
        // triple and the self-redirection rule, not as a separate gate. Asserted
        // as the entailment it is, so a reader does not look for the missing
        // `if` and conclude the property is unchecked.
        assert_ne!(
            claim.producer_owner(),
            claim.consumer_owner(),
            "distinct authorities, entailed by caller==consumer, callee==producer, caller!=callee"
        );

        let refusals = vec![
            // The edge is findable but its caller is no longer the consumer
            // owner: the triple's load-bearing row.
            (
                "consumer owner moved",
                d2f_refusal_of(d2f_preflight_exact(|keys| {
                    keys[0].consumer_owner = keys[0].producer_owner;
                })),
            ),
            // No edge enters that entry at all, so there is nothing to redirect.
            (
                "callee entry moved",
                d2f_refusal_of(d2f_preflight_exact(|keys| {
                    keys[0].invocation_callee_entry =
                        StaticOriginId(keys[0].invocation_callee_entry.0 + 1);
                })),
            ),
            // The admitted ledger root no longer ties back to the invocation.
            (
                "admitted result root moved",
                d2f_refusal_of(d2f_preflight_exact(|keys| {
                    keys[0].admitted.result_root = StaticOriginId(keys[0].admitted.result_root.0 + 1);
                })),
            ),
            // A checked binder no longer names the key's recursive position.
            (
                "consumer binder position moved",
                d2f_refusal_of(d2f_preflight_exact(|keys| {
                    keys[0].consumer_binding.recursive_position += 1;
                })),
            ),
            // Two distinct identities claiming one suffix, one continuation
            // frame and one edge.
            (
                "second key shares the region",
                d2f_refusal_of(d2f_preflight_exact(|keys| {
                    let mut twin = keys[0].clone();
                    twin.producer_alternative += 1;
                    keys.push(twin);
                })),
            ),
        ];

        assert_eq!(
            (members, refusals),
            (
                (
                    PredeclaredFunctionId(2),
                    PredeclaredFunctionId(3),
                    StaticOriginId(37),
                    ContinuationEmissionOwner::Fusion(id),
                    true,
                    true,
                    2,
                ),
                vec![
                    ("consumer owner moved", "InvocationTriple".to_string()),
                    ("callee entry moved", "SelectorEdge".to_string()),
                    ("admitted result root moved", "BinderAgreement".to_string()),
                    ("consumer binder position moved", "BinderAgreement".to_string()),
                    ("second key shares the region", "OverlappingClaim".to_string()),
                ],
            ),
            "one claim is issued for the canonical witness with the key's own members, and each \
             ruled rule refuses on its own moved operand -- the refusals share this assertion \
             with the issued claim so none of them can hold because preflight issued nothing"
        );
    }

    /// **`D2f` — the claim is AFFINE and the closeout bijects.**
    ///
    /// Separate from the preflight control because it is a different question:
    /// preflight asks which regions may be claimed, this asks that a claimed
    /// region is taken over exactly once and that an untaken one fails the
    /// closeout rather than passing quietly.
    ///
    /// **MEASURED:** consuming the claim at its own seat succeeds and leaves
    /// nothing to consume; a second consumption refuses; consuming at another
    /// occurrence refuses **and leaves the claim outstanding**, so the failed
    /// takeover is recoverable rather than a spent permit; a closeout with the
    /// claim unconsumed fails; and the closeout succeeds only when definition,
    /// redirect and consumption are all recorded for the installed region.
    /// **CLAIMED:** a fused region cannot be taken over twice, cannot be taken
    /// over at an occurrence its redirected invocation does not name, and cannot
    /// be left with its suffix still lowered by its original consumer.
    /// **THE GAP:** the ledger is exercised directly. Nothing here shows that
    /// real codegen calls `consume`, because no emitter does yet.
    #[test]
    fn d2f_3_one_region_is_taken_over_exactly_once_or_the_closeout_fails() {
        let id_of = |ledger: &FusionRegionClaimLedger| {
            *ledger.planned().iter().next().expect("the identity")
        };

        // Wrong seat: refuses, and the claim SURVIVES.
        let mut ledger = d2f_preflight_exact_owned(|_| (), true).expect("claim");
        let id = id_of(&ledger);
        let seat = ledger.claim(id).expect("outstanding").seat();
        let wrong_seat = ledger
            .consume(id, StaticOriginId(seat.0 + 1))
            .err()
            .is_some();
        let survives_wrong_seat = ledger.claim(id).is_some();

        // Right seat: consumes, and there is nothing left to consume.
        let consumed_once = ledger.consume(id, seat).is_ok();
        let outstanding_after = ledger.claim(id).is_some();
        let consumed_twice_refuses = ledger.consume(id, seat).is_err();

        // Closeout with the region defined and redirected: bijects.
        ledger.record_defined(id).expect("one definition");
        ledger.record_redirected(id).expect("one redirect");
        let bijects = ledger.close().expect("the closeout bijects");

        // Closeout with the claim never consumed: fails on the unconsumed claim.
        let mut unconsumed = d2f_preflight_exact_owned(|_| (), true).expect("claim");
        let other = id_of(&unconsumed);
        unconsumed.record_defined(other).expect("one definition");
        unconsumed.record_redirected(other).expect("one redirect");
        let unconsumed_fails = unconsumed.close().is_err();

        // Closeout with the claim consumed but no definition emitted: fails on
        // the definition set, not on the claim. A separate row because these are
        // two different halves of the bijection and a single "close errored"
        // could be satisfied by either.
        let mut undefined = d2f_preflight_exact_owned(|_| (), true).expect("claim");
        let third = id_of(&undefined);
        let third_seat = undefined.claim(third).expect("outstanding").seat();
        undefined.consume(third, third_seat).expect("consumed");
        undefined.record_redirected(third).expect("one redirect");
        let undefined_fails = undefined.close().is_err();

        // A second definition for one region refuses at the recorder rather than
        // at the closeout, so the double emission is caught where it happens.
        let mut twice = d2f_preflight_exact_owned(|_| (), true).expect("claim");
        let fourth = id_of(&twice);
        twice.record_defined(fourth).expect("one definition");
        let second_definition_refuses = twice.record_defined(fourth).is_err();

        assert_eq!(
            (
                wrong_seat,
                survives_wrong_seat,
                consumed_once,
                outstanding_after,
                consumed_twice_refuses,
                bijects,
                unconsumed_fails,
                undefined_fails,
                second_definition_refuses,
            ),
            (true, true, true, false, true, 1, true, true, true),
            "a claim offered the wrong seat refuses and survives; consumed at its own seat it is \
             spent exactly once; the closeout bijects only when definition, redirect and \
             consumption are all present, and an unconsumed claim or a missing definition each \
             fails it"
        );
    }

    /// **`D2f` producer side — the claimed producer's body leaves the executable
    /// population, and only that body and only that edge.**
    ///
    /// **MEASURED** on the checked applied `Exact` witness: body 37 of unit 2
    /// becomes `FusionOwned(0)`; unit 2 stays in `emittable_units` as the source,
    /// ABI and template authority while leaving `executable_units`; the claimed
    /// incoming edge `3 -> 2 @37` leaves `executable_call_edges` while the
    /// producer's own outgoing `2 -> 1 @34` survives; a second install refuses;
    /// and ownership derived from an identity whose claim is no longer
    /// outstanding refuses.
    /// **CLAIMED:** body ownership removes exactly the producer's standalone
    /// definition and exactly its claimed invocation, leaving every other route
    /// and every other unit alone.
    /// **THE GAP:** un-wired. No production compile installs ownership, so this
    /// says nothing about a real compile; and the axis row below is measured
    /// degenerate on this witness -- see its own comment.
    #[test]
    fn d2f_4_the_claimed_producer_body_leaves_the_executable_population() {
        let (entry, declaration, oriented) = d2j_checked_fixture_under(D2jCause::Exact);
        let mut declarations = BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        let mut plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
        let plane =
            build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
                .expect("plane");
        plan.install_static_continuation_fusions(plane)
            .expect("installs");
        let mut ledger = FusionRegionClaimLedger::preflight(&plan).expect("claims");

        // BEFORE: the population is complete, which is the denominator every row
        // below is measured against. A "unit 2 is absent" that was already true
        // would carry nothing.
        let before_units: Vec<PredeclaredFunctionId> = plan
            .executable_units()
            .expect("units")
            .iter()
            .map(|unit| unit.function())
            .collect();
        let before_edges = plan.executable_call_edges().expect("edges").len();

        plan.install_fusion_owned_bodies(&mut ledger)
            .expect("ownership installs");

        let disposition = plan
            .body_dispositions()
            .expect("dispositions")
            .get(&StaticOriginId(37))
            .copied();
        let after_units: Vec<PredeclaredFunctionId> = plan
            .executable_units()
            .expect("units")
            .iter()
            .map(|unit| unit.function())
            .collect();
        // `emittable_units` must be UNCHANGED: the ruling keeps the producer as
        // the source/ABI/template authority for the body its fused definition
        // lowers, so a row that let it shrink would be the AST excision the
        // Architect forbade rather than the bounded disposition.
        let emittable_intact = plan
            .emittable_units()
            .expect("emittable")
            .iter()
            .any(|unit| unit.function() == PredeclaredFunctionId(2));
        let after_edges: Vec<(u32, u32, u32)> = plan
            .executable_call_edges()
            .expect("edges")
            .iter()
            .map(|edge| (edge.caller().0, edge.callee().0, edge.callee_origin().0))
            .collect();

        // A second install refuses rather than overwriting.
        let second_install = plan.install_fusion_owned_bodies(&mut ledger).is_err();

        // Ownership derived from an identity with no outstanding claim refuses:
        // a consumed claim has already spent its takeover, so nothing
        // authorizes removing the producer's definition on its behalf.
        let mut spent = d2f_preflight_exact(|_| ()).expect("claims");
        let only = *spent.planned().iter().next().expect("identity");
        let seat = spent.claim(only).expect("outstanding").seat();
        spent.consume(only, seat).expect("consumed");
        let mut fresh_plan = {
            let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
            let plane =
                build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
                    .expect("plane");
            let mut plan = plan;
            plan.install_static_continuation_fusions(plane).expect("installs");
            plan
        };
        let consumed_claim_refuses = fresh_plan.install_fusion_owned_bodies(&mut spent).is_err();

        assert_eq!(
            (
                before_units,
                before_edges,
                disposition,
                after_units,
                emittable_intact,
                after_edges,
                second_install,
                consumed_claim_refuses,
            ),
            (
                vec![
                    PredeclaredFunctionId(0),
                    PredeclaredFunctionId(1),
                    PredeclaredFunctionId(2),
                    PredeclaredFunctionId(3),
                ],
                // MEASURED, not chosen: three emittable edges before, and the
                // third is what makes "only the claimed one left" a real
                // statement rather than "the only edge left".
                3,
                Some(BodyEmissionDisposition::FusionOwned(
                    StaticContinuationFusionId(0)
                )),
                vec![
                    PredeclaredFunctionId(0),
                    PredeclaredFunctionId(1),
                    PredeclaredFunctionId(3),
                ],
                true,
                // The producer's own outgoing edge AND the root's call into the
                // consumer both survive; exactly `3 -> 2 @37` is gone.
                vec![(2, 1, 34), (0, 3, 40)],
                true,
                true,
            ),
            "installing ownership removes exactly unit 2's standalone definition and exactly its \
             claimed incoming edge 3 -> 2 @37, leaves the producer's own outgoing 2 -> 1 @34 and \
             the root's 0 -> 3 @40 and every other unit alone, keeps unit 2 emittable as the source/ABI/template authority, \
             and refuses both a second install and an install derived from a spent claim"
        );

        // THE AXIS ROW, AND IT IS MEASURED DEGENERATE HERE -- stated rather than
        // written as a passing check. The ruling asks for a control proving the
        // disposition is keyed by `body_occurrence()` and not by a call
        // identity. On this witness unit 2's `entry_origin()` and
        // `body_occurrence()` are BOTH 37, because `resolve_call_edges` enforces
        // `unit.entry_origin() == edge.callee_origin()` and the claimed edge's
        // callee origin is 37. So substituting either axis for the other selects
        // the same body and no assertion over this fixture can tell them apart.
        // A row asserting "keyed by the body" would pass for the wrong reason.
        // The discriminating witness is a unit whose body schedules something
        // before itself, which this family does not contain.
        let unit_two = plan
            .emittable_units()
            .expect("emittable")
            .into_iter()
            .find(|unit| unit.function() == PredeclaredFunctionId(2))
            .expect("unit 2");
        assert_eq!(
            (unit_two.entry_origin(), unit_two.body_occurrence()),
            (StaticOriginId(37), StaticOriginId(37)),
            "the two axes COINCIDE on this witness, so the body-vs-call-identity row is \
             degenerate here and is deliberately not asserted as if it discriminated"
        );
    }

    /// **`D2f` Deliverable 5 — the redirect target is DERIVED from the complete
    /// key, and every invocation member of that key is load-bearing.**
    ///
    /// The deliverable's own words are that the redirection names the exact
    /// original producer invocation and is *"not a search for a plausible
    /// one"*. A control that only showed the right edge coming out would be
    /// satisfied by a selector that returned the sole `StaticBody` edge, or the
    /// first one, or the one whose callee happens to match — so the discriminator
    /// is written first and it is per member: each of the three invocation
    /// members is independently repointed at **another identity this same plan
    /// really contains**, and each repointing must refuse.
    ///
    /// Repointing at a real sibling rather than an invented id is what makes
    /// each row say "this member is consulted" instead of "an unknown id is not
    /// found" — a selector that ignored a member entirely would still find its
    /// edge under that member's mutation.
    ///
    /// **MEASURED:** on the checked applied `Exact` witness the key's invocation
    /// triple selects exactly one of the three emittable call edges, it is
    /// `3 -> 2` entering origin `37`, and each of three sibling repointings
    /// selects none.
    /// **CLAIMED:** the edge a fusion redirects is determined by the key.
    /// **THE GAP:** this pins **selection**. It pins no emission: nothing here
    /// redirects anything, and the fused region does not exist. `3 -> 2` is an
    /// observed coordinate of this witness and is deliberately absent from
    /// [`fusion_redirect_target`], which reads key members only.
    #[test]
    fn d2f_5_the_redirect_target_is_selected_by_the_complete_key() {
        let (entry, declaration, oriented) = d2j_checked_fixture_under(D2jCause::Exact);
        let mut declarations = BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
        let fusion =
            build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
                .expect("the plane builds");
        let key = fusion
            .key_for(StaticContinuationFusionId(0))
            .expect("the witness resolves one key")
            .clone();

        let selected = fusion_redirect_target(&plan, &key).expect("the key selects its edge");

        // The population the selection discriminates WITHIN is an operand, not a
        // message argument. Selecting one of one would be a restatement of the
        // plan's shape; selecting one of three is a discrimination.
        let population = plan.executable_call_edges().expect("edges").len();
        assert_eq!(
            (
                population,
                selected.caller(),
                selected.callee(),
                selected.callee_origin(),
                selected.kind(),
            ),
            (
                3,
                key.invocation_caller,
                key.invocation_callee,
                key.invocation_callee_entry,
                EmittableCallKind::StaticBody,
            ),
            "the key must pick exactly its own invocation out of a population that \
             contains alternatives: {:?}",
            plan.executable_call_edges().expect("edges"),
        );
        // Restated once against the measured coordinates, so a future plan whose
        // numbering moves is caught here rather than silently agreeing with a key
        // that moved with it. These literals are this witness's, never the
        // selector's.
        assert_eq!(
            (
                selected.caller().0,
                selected.callee().0,
                selected.callee_origin().0
            ),
            (3, 2, 37),
            "the checked twin's producer invocation is 3 -> 2 entering origin 37; the retired \
             px8j coordinate 0 -> 2 is not an edge of this plan"
        );

        // ---- the discriminator, one row per invocation member.
        //
        // Each repointing names an identity this plan really has: unit 2 is a
        // real caller, unit 1 is a real callee, and origin 34 is a real callee
        // entry -- they simply are not THIS invocation's.
        let repointings: [(&str, &dyn Fn(&mut StaticContinuationFusionKey)); 3] = [
            ("invocation_caller", &|key| {
                key.invocation_caller = PredeclaredFunctionId(2);
            }),
            ("invocation_callee", &|key| {
                key.invocation_callee = PredeclaredFunctionId(1);
            }),
            ("invocation_callee_entry", &|key| {
                key.invocation_callee_entry = StaticOriginId(34);
            }),
        ];
        let refused: Vec<(&str, bool)> = repointings
            .iter()
            .map(|(member, repoint)| {
                let mut moved = key.clone();
                repoint(&mut moved);
                assert_ne!(
                    moved, key,
                    "{member}: the repointing must actually change the key"
                );
                (*member, fusion_redirect_target(&plan, &moved).is_err())
            })
            .collect();
        assert_eq!(
            refused,
            vec![
                ("invocation_caller", true),
                ("invocation_callee", true),
                ("invocation_callee_entry", true),
            ],
            "every invocation member must be consulted: a member the selector ignored would \
             still find this edge after that member was repointed at a real sibling"
        );

        // ---- the kind is VALIDATED, not selected on.
        //
        // Repointed at the plan's declaration call edge, the triple matches --
        // so this row reaches the survivor and is refused for its kind. Without
        // it, a selector that pre-filtered on `StaticBody` would be
        // indistinguishable from one that checks afterwards, and the difference
        // is whether an ambiguity can be silently resolved.
        let mut declaration_call = key.clone();
        declaration_call.invocation_caller = PredeclaredFunctionId(0);
        declaration_call.invocation_callee = PredeclaredFunctionId(3);
        declaration_call.invocation_callee_entry = StaticOriginId(40);
        let error = fusion_redirect_target(&plan, &declaration_call)
            .expect_err("a declaration call is not a producer invocation");
        assert!(
            format!("{error:?}").contains("declaration call"),
            "the survivor must be refused for its KIND, which is a different failure from \
             matching nothing: {error:?}"
        );
    }

    /// **`D2f` — a ledger spent on one plan cannot mutate a second, and the
    /// refusal leaves that second plan byte-for-byte at baseline.**
    ///
    /// The Architect's block on `21455ec4`. The install writes two objects that
    /// are **not type-bound to each other** — the plan's ownership map and the
    /// ledger's recorded set — so a ledger already spent on plan A can be handed
    /// to an equivalent plan B. B's own scratch map validates perfectly, because
    /// B is an equivalent plan; the refusal comes only from the ledger. With the
    /// plan mutated first, that case left B owning bodies, with a narrowed
    /// executable-unit population and a narrowed edge population, **after a call
    /// that returned `Err`**.
    ///
    /// **MEASURED:** installing into A succeeds; the same ledger against an
    /// equivalent B is rejected; and B's ownership map, executable-unit
    /// population and executable-edge population are each identical to the
    /// baseline captured before the attempt. A second install into A is also
    /// rejected, and a ledger whose region set is empty still refuses a second
    /// record rather than reading as never-recorded.
    /// **CLAIMED:** no failed install leaves a plan partially owned.
    /// **THE GAP:** this pins the ordering of the transaction. It does not show
    /// that any production caller reuses a ledger — none does today, which is
    /// exactly why the defect was reachable only by construction and not by a
    /// failing compile.
    #[test]
    fn d2f_6_a_spent_ledger_cannot_half_install_into_a_second_plan() {
        let (entry, declaration, oriented) = d2j_checked_fixture_under(D2jCause::Exact);
        let mut declarations = BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);

        let build = || {
            let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
            let plane =
                build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
                    .expect("plane");
            let mut plan = plan;
            plan.install_static_continuation_fusions(plane).expect("installs");
            plan
        };
        // Two equivalent plans over one witness. B is a real second plan, not a
        // clone of A: if it were a clone, "B was not mutated" could hold because
        // nothing ever could mutate it.
        let mut plan_a = build();
        let mut plan_b = build();
        let mut ledger = FusionRegionClaimLedger::preflight(&plan_a).expect("claims");

        let snapshot = |plan: &StaticTransitionPlan<'_>| {
            (
                plan.fusion_owned_bodies().clone(),
                plan.executable_units()
                    .expect("units")
                    .iter()
                    .map(|unit| (unit.function(), unit.body_occurrence()))
                    .collect::<Vec<_>>(),
                plan.executable_call_edges()
                    .expect("edges")
                    .iter()
                    .map(|edge| {
                        (
                            edge.caller(),
                            edge.callee(),
                            edge.callee_origin(),
                            edge.call_site_origin(),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        };
        let b_baseline = snapshot(&plan_b);

        let a_installs = plan_a.install_fusion_owned_bodies(&mut ledger).is_ok();
        // A genuinely moved, which is the denominator: "B is unchanged" says
        // nothing unless an install of this ledger demonstrably changes a plan.
        let a_moved = snapshot(&plan_a) != b_baseline;

        let b_rejected = plan_b.install_fusion_owned_bodies(&mut ledger).is_err();
        let b_intact = snapshot(&plan_b) == b_baseline;

        // A second install into A refuses on the explicit flag.
        let a_second_refuses = plan_a.install_fusion_owned_bodies(&mut ledger).is_err();

        // The zero-region half of exact-once is `d2f_7`, on a plan that
        // genuinely installs no regions. It is NOT tested here: feeding an
        // artificial empty subset to a one-region ledger exercises neither
        // flag against a real zero-region install.

        assert_eq!(
            (
                a_installs,
                a_moved,
                b_rejected,
                b_intact,
                a_second_refuses,
            ),
            (true, true, true, true, true),
            "installing a ledger into one plan moves it; reusing that spent ledger against an \
             equivalent second plan is rejected and leaves that plan's ownership map, executable \
             units and executable edges exactly at baseline; and a second install refuses"
        );
    }

    /// **`D2f` — exact-once holds on a plan that installs NO fused regions, on
    /// both flags independently.**
    ///
    /// The Architect's second block. `d2f_6` fed an artificial empty owned
    /// subset to a **one-region** ledger, which exercised neither flag against a
    /// real zero-region install — so reverting the plan's `fusion_bodies_installed`
    /// to the old empty-map sentinel left that control green. The population a
    /// row selects from has to be the real one; an empty set handed to a
    /// non-empty ledger is a constructed input, not a zero-region compile.
    ///
    /// `D2jCause::ExactSuffix` is the genuine article: it reaches the plane
    /// builder and resolves **zero** keys, so its plan installs an empty fusion
    /// arena and its preflight ledger has an empty `planned` set. Both flags are
    /// then the only thing that can distinguish never-installed from
    /// installed-empty, because every emptiness test in sight answers "empty"
    /// either way.
    ///
    /// **MEASURED:** the plane resolves zero regions; the first install on that
    /// empty plan succeeds; a second install on the same plan is rejected; the
    /// recorded empty ledger reused against an equivalent empty plan is
    /// rejected without mutating it; and that plan remains installable by a
    /// fresh empty ledger, which is what proves the rejection left no residue.
    /// **CLAIMED:** exact-once is a property of whether the transaction ran, not
    /// of whether its payload was non-empty.
    /// **THE GAP:** un-wired, as with every control on this branch.
    #[test]
    fn d2f_7_the_zero_region_install_is_exact_once_on_both_flags() {
        let (entry, declaration, oriented) = d2j_checked_fixture_under(D2jCause::ExactSuffix);
        let mut declarations = BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);

        let build = || {
            let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
            let plane =
                build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
                    .expect("plane");
            let regions = plane.len();
            let mut plan = plan;
            plan.install_static_continuation_fusions(plane).expect("installs");
            (plan, regions)
        };

        let (mut empty_a, regions) = build();
        let mut ledger = FusionRegionClaimLedger::preflight(&empty_a).expect("claims");
        // The population this control depends on, asserted rather than assumed:
        // a cause that quietly began resolving one region would turn every row
        // below into the one-region case under a zero-region name.
        let ledger_empty = ledger.is_empty();

        // First install on a genuinely zero-region plan SUCCEEDS. Under the old
        // emptiness sentinel this was indistinguishable from never having run.
        let first_install = empty_a.install_fusion_owned_bodies(&mut ledger).is_ok();
        // Second install on the SAME plan must reject — with a **FRESH** ledger.
        //
        // The fresh ledger is the whole point of this row and it was wrong on
        // the first cut. Reusing `ledger` here rejects on the LEDGER's flag,
        // which is set, so the row passed even with the plan's flag reverted to
        // an emptiness sentinel — a check that could not fail, in a control
        // written specifically to prove it could. A ledger that has never
        // recorded leaves the plan's own flag as the only thing able to refuse.
        let mut second_ledger = FusionRegionClaimLedger::preflight(&empty_a).expect("claims");
        let second_install_rejects = empty_a
            .install_fusion_owned_bodies(&mut second_ledger)
            .is_err();

        // The recorded empty ledger against an EQUIVALENT empty plan: rejected,
        // and B unchanged. This is the row that pins the ledger's flag, and it
        // is a separate object from the row above so a single flag cannot carry
        // both.
        let (mut empty_b, _) = build();
        let b_baseline = (
            empty_b.fusion_owned_bodies().clone(),
            empty_b
                .executable_units()
                .expect("units")
                .iter()
                .map(|unit| (unit.function(), unit.body_occurrence()))
                .collect::<Vec<_>>(),
            empty_b.executable_call_edges().expect("edges").len(),
        );
        let b_rejects_spent_ledger = empty_b.install_fusion_owned_bodies(&mut ledger).is_err();
        let b_intact = (
            empty_b.fusion_owned_bodies().clone(),
            empty_b
                .executable_units()
                .expect("units")
                .iter()
                .map(|unit| (unit.function(), unit.body_occurrence()))
                .collect::<Vec<_>>(),
            empty_b.executable_call_edges().expect("edges").len(),
        ) == b_baseline;
        // And B is still INSTALLABLE by a fresh empty ledger. Without this the
        // rejection above is consistent with B having been quietly consumed:
        // "unchanged and dead" and "unchanged and live" look identical from a
        // snapshot alone.
        let mut fresh = FusionRegionClaimLedger::preflight(&empty_b).expect("claims");
        let b_still_installable = empty_b.install_fusion_owned_bodies(&mut fresh).is_ok();

        assert_eq!(
            (
                regions,
                ledger_empty,
                first_install,
                second_install_rejects,
                b_rejects_spent_ledger,
                b_intact,
                b_still_installable,
            ),
            (0, true, true, true, true, true, true),
            "on a plan that genuinely installs zero fused regions, the first ownership install \
             succeeds, a second on the same plan is rejected by the plan's own flag, the recorded \
             empty ledger is rejected against an equivalent plan without mutating it, and that \
             plan is still installable by a fresh ledger -- so exact-once is a property of \
             whether the transaction ran and not of whether its payload was non-empty"
        );
    }

    /// `D2i` — the enumerator is REACHED, and on ledger roots the terminal twin
    /// yields EXACTLY ONE candidate.
    ///
    /// MEASURED, and it is not what the earlier seed-keyed reading concluded.
    /// `D2h` reported that this fixture could present no producer/consumer pair,
    /// and that was true of the SEED root `child(consumer, 0)` only. The
    /// admitted ledger also carries a DESCENT root -- one the fixed point
    /// admitted with `Some(enclosing_specialization)`, which no seed
    /// reconstruction can name -- and from that root the walk reaches the
    /// producer.
    ///
    /// So consuming the ledger is what makes the fixture productive. Every gate
    /// then passes on facts inside the Architect's seven: the producer's
    /// argument binds a hypothesis, the selected case body's exact `Call`
    /// resolves to this consumer frame at this position, the transport
    /// coordinate resolves, the `StaticBody` triple is unique, and the owners
    /// split.
    ///
    /// The candidate's members are pinned rather than counted, because a count
    /// would pass on a candidate assembled from the wrong root.
    #[test]
    fn d2i_the_enumerator_is_reached_and_yields_one_candidate_from_a_descent_root() {
        let declaration = d2g_declaration(true);
        let entry = d2g_entry();
        let mut declarations = BTreeMap::new();
        declarations.insert(D2G_DECLARATION, &declaration);
        let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
        let oriented = d2g_oriented_plan();

        let roots = fusion_root_source_for_future_enumerator(&plan).expect("root source");
        assert!(
            !roots.is_empty(),
            "the enumerator must have had roots to walk"
        );

        let candidates =
            enumerate_live_fusion_candidates(&plan, &entry, &declarations, Some(&oriented))
                .expect("the enumerator runs to completion");
        assert_eq!(
            candidates.len(),
            1,
            "exactly one candidate, and multiplicity would be a refusal rather than a \
             selection: {candidates:?}"
        );
        let candidate = &candidates[0];

        // THE LOAD-BEARING MEMBER: the root is a descent, not the seed. A seed
        // reconstruction carries `None` here by construction, so this member is
        // what shows the ledger is why the fixture is productive at all.
        assert!(
            candidate.admitted.enclosing_specialization.is_some(),
            "the candidate must come from an admitted DESCENT root, which no seed \
             reconstruction can name: {:?}",
            candidate.admitted
        );
        assert_ne!(
            candidate.admitted.result_root,
            plan.semantic
                .child_origin(candidate.admitted.continuation_origin, 0)
                .expect("scrutinee"),
            "and that root must differ from the consumer's own seed root"
        );

        // The consumer binding names this frame and position, not merely some
        // hypothesis.
        assert_eq!(
            candidate.consumer_binding,
            CheckedIhBinding {
                frame_origin: candidate.admitted.continuation_origin,
                recursive_position: candidate.recursive_position,
            },
            "the consuming Call's callee resolves to THIS consumer frame and position"
        );
        assert_ne!(
            candidate.producer_owner, candidate.consumer_owner,
            "producer and consumer are in different units, which is the split the \
             fusion exists to close"
        );
        // The COMPLETE triple, not just its callee. Pinning one member leaves the
        // other two free to be anything, and the caller is the emission owner the
        // redirection in `D2f` will belong to.
        assert_eq!(
            candidate.invocation_caller,
            PredeclaredFunctionId(1),
            "the measured emission owner of the producer invocation"
        );
        assert_eq!(
            candidate.invocation_callee, candidate.producer_owner,
            "the triple's callee is the producer's own unit"
        );
        assert_eq!(
            candidate.invocation_callee,
            PredeclaredFunctionId(3),
            "and that unit is the measured one"
        );
        // Tied to the candidate's own admitted root rather than to a literal:
        // the entry the invocation enters IS the root the descent admitted, and
        // stating it as `== 33` alone would still hold if the two drifted apart.
        assert_eq!(
            candidate.invocation_callee_entry, candidate.admitted.result_root,
            "the invocation's callee entry is the candidate's admitted root"
        );
        assert_eq!(
            candidate.invocation_callee_entry,
            StaticOriginId(33),
            "and that root is the measured one"
        );

        // The converse direction, so one candidate is not the only outcome this
        // control can produce: with no oriented plan the required transport
        // member cannot resolve and the enumerator refuses at that gate.
        let absent = enumerate_live_fusion_candidates(&plan, &entry, &declarations, None)
            .expect_err("markers present with no plan must refuse at the transport gate");
        assert!(
            format!("{absent:?}").contains("checked subcontinuation markers have no checked plan"),
            "and it must refuse for the transport reason: {absent:?}"
        );
    }

    /// `D2i` `AC-2` — suppressing ONLY the post-specialization descent takes the
    /// candidate count 1 to 0, and the initial terminal root survives.
    ///
    /// This is the causal control for the finding. The candidate exists because
    /// the ledger carries a descent root; removing exactly that push must remove
    /// exactly that candidate. Both halves are asserted, because a count going
    /// to zero would also be produced by discovery collapsing entirely -- and
    /// that would prove nothing about the descent.
    #[test]
    fn d2i_ac2_suppressing_only_the_descent_takes_the_candidate_to_zero() {
        struct Restore;
        impl Drop for Restore {
            fn drop(&mut self) {
                set_post_specialization_descent_suppressed(false);
            }
        }

        let declaration = d2g_declaration(true);
        let entry = d2g_entry();
        let mut declarations = BTreeMap::new();
        declarations.insert(D2G_DECLARATION, &declaration);
        let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
        let oriented = d2g_oriented_plan();

        let before = enumerate_live_fusion_candidates(&plan, &entry, &declarations, Some(&oriented))
            .expect("enumerates")
            .len();
        assert_eq!(before, 1, "the unsuppressed baseline is one candidate");
        let seed_root = plan
            .semantic
            .child_origin(
                fusion_root_source_for_future_enumerator(&plan).expect("roots")[0]
                    .continuation_origin,
                0,
            )
            .expect("scrutinee");

        let _restore = Restore;
        set_post_specialization_descent_suppressed(true);
        let after = enumerate_live_fusion_candidates(&plan, &entry, &declarations, Some(&oriented))
            .expect("enumerates")
            .len();
        let suppressed_roots =
            fusion_root_source_for_future_enumerator(&plan).expect("roots under suppression");

        assert_eq!(
            after, 0,
            "suppressing only the descent must remove exactly the candidate it fed"
        );
        assert!(
            suppressed_roots
                .iter()
                .any(|root| root.result_root == seed_root),
            "and the initial terminal root must still be admitted, or the count fell \
             because discovery collapsed rather than because the descent went: \
             {suppressed_roots:?}"
        );
        assert!(
            suppressed_roots
                .iter()
                .all(|root| root.enclosing_specialization.is_none()),
            "with no descent admitted, no admitted root carries an enclosing \
             specialization: {suppressed_roots:?}"
        );
    }

    /// `D2i` `AC-3` — MULTIPLICITY is refused, and it is executed rather than
    /// claimed.
    ///
    /// The previous candidate asserted only that "multiplicity would be a
    /// refusal" in a message. `fusion_unique_static_body_triple` declines on a
    /// second matching edge, but nothing reached that branch, so the claim was
    /// unexecuted.
    ///
    /// Arming the control presents a second matching edge at the uniqueness
    /// decision and changes nothing else. Every earlier gate has already been
    /// satisfied by the time it runs -- the transport coordinate still resolves
    /// and the admitted roots are unchanged, both asserted below -- so the
    /// candidate that disappears did so at the uniqueness gate specifically and
    /// not because something upstream broke.
    #[test]
    fn d2i_ac3_a_second_matching_static_body_edge_is_refused() {
        struct Restore;
        impl Drop for Restore {
            fn drop(&mut self) {
                set_static_body_triple_duplicated(false);
            }
        }

        let declaration = d2g_declaration(true);
        let entry = d2g_entry();
        let mut declarations = BTreeMap::new();
        declarations.insert(D2G_DECLARATION, &declaration);
        let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
        let oriented = d2g_oriented_plan();

        let before = enumerate_live_fusion_candidates(&plan, &entry, &declarations, Some(&oriented))
            .expect("enumerates")
            .len();
        assert_eq!(before, 1, "the unsuppressed baseline is one candidate");
        let roots_before = fusion_root_source_for_future_enumerator(&plan).expect("roots");
        let transport_before = build_checked_transport(&plan, &oriented).expect("transport");

        let _restore = Restore;
        set_static_body_triple_duplicated(true);
        let after = enumerate_live_fusion_candidates(&plan, &entry, &declarations, Some(&oriented))
            .expect("enumerates")
            .len();

        assert_eq!(
            after, 0,
            "a second matching StaticBody edge must refuse rather than select among them"
        );
        assert_eq!(
            fusion_root_source_for_future_enumerator(&plan).expect("roots"),
            roots_before,
            "and the admitted roots are untouched, so discovery did not change"
        );
        assert_eq!(
            build_checked_transport(&plan, &oriented).expect("transport"),
            transport_before,
            "and the transport coordinates still resolve, so the earlier gates still pass"
        );
    }

    /// `D2i` — the ROOT SOURCE fusion enumeration consumes is the admitted
    /// ledger, and it is strictly richer than the seed frontier.
    ///
    /// [`enumerate_live_fusion_candidates`] consumes this helper, and the `D2h`
    /// production key plane consumes that; all three are production planner
    /// state. No seed-only path was ever removed, because none existed on this
    /// branch to delete -- the seed frontier is excluded by construction. What
    /// is established here is which roots the enumerator walks, and why the
    /// obvious alternative is wrong.
    ///
    /// The equality below is an ALIAS OBSERVATION, not a causal control: the
    /// helper is defined as the ledger, so it can only agree with it. It is kept
    /// because it is the statement of what the root source IS, and it is
    /// labelled so nobody reads it as evidence.
    ///
    /// The two causal claims are the ones after it:
    ///
    /// - the admitted population is **strictly larger** than the seed
    ///   reconstruction can name, so keying an enumerator on the seeds would
    ///   miss admitted roots;
    /// - at least one admitted root carries **`Some(enclosing_specialization)`**,
    ///   which no seed reconstruction can produce and which cannot be recovered
    ///   downstream from a worker body's raw occurrence owner.
    ///
    /// Measured on the landed terminal twin; the claim is about that fixture.
    #[test]
    fn d2i_the_future_enumerators_root_source_is_richer_than_the_seed_frontier() {
        let declaration = d2g_declaration(true);
        let entry = d2g_entry();
        let mut declarations = BTreeMap::new();
        declarations.insert(D2G_DECLARATION, &declaration);
        let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");

        let root_source: BTreeSet<_> = fusion_root_source_for_future_enumerator(&plan)
            .expect("root source")
            .into_iter()
            .collect();
        let ledger: BTreeSet<_> = admitted_continuation_discoveries(&plan)
            .expect("ledger")
            .into_iter()
            .collect();
        assert_eq!(
            root_source, ledger,
            "alias observation only: the root source is DEFINED as the admitted ledger, \
             so this states what it is and is not evidence that anything consumes it"
        );

        let mut seeds = BTreeSet::new();
        for occurrence in plan.source_occurrences.iter().flatten() {
            if matches!(occurrence.expr, RuntimeExpr::ComputationalMatch { .. }) {
                seeds.insert(AdmittedContinuationDiscovery {
                    continuation_origin: occurrence.static_origin,
                    result_root: plan
                        .semantic
                        .child_origin(occurrence.static_origin, 0)
                        .expect("scrutinee"),
                    enclosing_specialization: None,
                });
            }
        }
        // CARDINALITY IS NOT CONTAINMENT. `len() >` alone permits a ledger that
        // LOSES seed pairs while gaining unrelated roots, which is a strictly
        // worse failure than the one the count was meant to catch.
        //
        // The containment claim belongs on `(continuation_origin, result_root)`,
        // because a reconstructed seed carries `None` for its enclosing
        // specialization while the admitted entry for the same syntactic pair
        // may carry one -- so full-identity containment would fail for a reason
        // that is not a defect. Strict extension stays on the full identity.
        let projection = |set: &BTreeSet<AdmittedContinuationDiscovery>| {
            set.iter()
                .map(|entry| (entry.continuation_origin, entry.result_root))
                .collect::<BTreeSet<_>>()
        };
        let seed_pairs = projection(&seeds);
        let admitted_pairs = projection(&root_source);
        assert!(
            seed_pairs.is_subset(&admitted_pairs),
            "every independently reconstructed syntactic seed pair must occur in the \
             admitted projection, or the ledger has dropped a root the frontier names: \
             missing={:?}",
            seed_pairs.difference(&admitted_pairs).collect::<Vec<_>>()
        );
        assert!(
            root_source.len() > seeds.len(),
            "and the admitted population must strictly extend the seed frontier on the \
             FULL identity, or fixing this root source in advance buys a future \
             enumerator nothing: root_source={root_source:?} seeds={seeds:?}"
        );
        assert!(
            root_source
                .iter()
                .any(|root| root.enclosing_specialization.is_some()),
            "and at least one admitted root must carry its enclosing specialization, \
             the field a seed reconstruction cannot produce: root_source={root_source:?}"
        );
    }

    /// `D2i` — the discovery LEDGER is what the production fixed point admitted,
    /// and it is not reconstructible from the seeds.
    ///
    /// Production does derive the initial frontier as `child(consumer, 0)` over
    /// every planned `ComputationalMatch`; a **reconstructed top-level seed
    /// carries `None`** for its enclosing specialization, because a top-level
    /// computational frame has no enclosing generated context. The fixed point
    /// then descends into selected worker bodies and admits discoveries that do
    /// carry one.
    ///
    /// Three claims, and the third is the one that matters:
    ///
    /// 1. every reconstructed seed is admitted;
    /// 2. the ledger is a strict extension of them;
    /// 3. **at least one admitted entry is a descent** -- it carries
    ///    `Some(enclosing_specialization)`.
    ///
    /// A count-only extra pair would satisfy (2) and prove nothing: two entries
    /// differing only in a field the ledger projected away would look like an
    /// extension while actually being the same discovery recorded twice.
    /// Requiring a `Some` positively is what shows the ledger carries the
    /// emission context, which cannot be reconstructed downstream from a worker
    /// body's raw occurrence owner.
    #[test]
    fn d2i_the_discovery_ledger_is_richer_than_the_seed_reconstruction() {
        let declaration = d2g_declaration(true);
        let entry = d2g_entry();
        let mut declarations = BTreeMap::new();
        declarations.insert(D2G_DECLARATION, &declaration);
        let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");

        let ledger = admitted_continuation_discoveries(&plan).expect("ledger");
        assert!(!ledger.is_empty(), "the fixed point admitted nothing at all");

        // The seed reconstruction, spelled out only so the two can be compared.
        // Nothing in production derives the admitted population this way.
        let mut seeds = BTreeSet::new();
        for occurrence in plan.source_occurrences.iter().flatten() {
            if matches!(occurrence.expr, RuntimeExpr::ComputationalMatch { .. }) {
                seeds.insert(AdmittedContinuationDiscovery {
                    continuation_origin: occurrence.static_origin,
                    result_root: plan
                        .semantic
                        .child_origin(occurrence.static_origin, 0)
                        .expect("scrutinee"),
                    // A top-level computational frame has no enclosing
                    // generated context.
                    enclosing_specialization: None,
                });
            }
        }
        let admitted: BTreeSet<_> = ledger.iter().copied().collect();

        assert!(
            seeds.is_subset(&admitted),
            "every reconstructed seed must be admitted, or the ledger is missing the \
             frontier it started from: seeds={seeds:?} admitted={admitted:?}"
        );
        assert!(
            admitted.len() > seeds.len(),
            "the ledger must admit pairs the seeds cannot name: seeds={seeds:?} \
             admitted={admitted:?}"
        );
        assert!(
            admitted
                .iter()
                .any(|entry| entry.enclosing_specialization.is_some()),
            "at least one admitted entry must be a DESCENT carrying its enclosing \
             specialization. Without this the extension could be a pair distinguished \
             only by a field the ledger dropped, which is the defect this field exists \
             to close: admitted={admitted:?}"
        );
    }

    /// `D2j` — the per-member provenance matrix, one executable assertion per
    /// row.
    ///
    /// Each closed-seven member is checked against its authoritative planner
    /// fact on the non-empty witness. These are assertions, not a table in a
    /// record: a row that stopped holding would red here.
    ///
    /// `recursive_position` is the one row that is NOT an independent
    /// derivation, and it is tested as exactly what it is -- see below.
    #[test]
    fn d2j_every_member_matches_its_authoritative_planner_fact() {
        // REBASELINED onto the cause-selected root. Every coordinate below is
        // re-derived from THIS entry; none is transported from the bare-root
        // revision of this control.
        let (entry, declaration, oriented) = d2j_checked_fixture_under(D2jCause::Exact);
        let mut declarations = BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
        let candidates =
            enumerate_live_fusion_candidates(&plan, &entry, &declarations, Some(&oriented))
                .expect("enumerates");
        assert_eq!(candidates.len(), 1);
        let c = &candidates[0];

        // 1. admitted discovery context -> the production ledger.
        //
        // POPULATION: the ledger holds four admitted discoveries, so membership
        // selects one of four rather than restating a singleton.
        let ledger = fusion_root_source_for_future_enumerator(&plan).expect("ledger");
        assert!(
            ledger.len() > 1,
            "a one-entry ledger would make membership degenerate: {ledger:?}"
        );
        assert!(
            ledger.contains(&c.admitted),
            "the admitted context must be an entry of the production ledger"
        );

        // 2. producer construct -> the admitted root's result population.
        //
        // POPULATION: five origins sit in that root's result position, so
        // membership is again a selection and not a restatement.
        let results = continuation_result_origins(&plan, c.admitted.result_root).expect("results");
        assert!(
            results.len() > 1,
            "a one-origin result population would make membership degenerate: {results:?}"
        );
        assert!(
            results.contains(&c.producer_construct_origin),
            "the producer construct must lie in its admitted root's result population"
        );
        // producer owner -> occurrence authority.
        assert_eq!(
            c.producer_owner,
            occurrence_authority(&plan, c.producer_construct_origin)
                .expect("authority")
                .owner
        );
        // alternative -> constructor identity, and exactly one case matches.
        let identity = plan
            .constructor_symbol_identity(c.producer_construct_origin)
            .expect("identity");
        let case_count = {
            let RuntimeExpr::ComputationalMatch { cases, .. } = plan
                .planned_occurrence_expr(c.admitted.continuation_origin)
                .expect("consumer")
            else {
                panic!("the consumer is not a computational match")
            };
            cases.len()
        };
        // POPULATION: two cases, so "unique" is a discrimination between them.
        assert_eq!(
            case_count, 2,
            "a single-case match would make uniqueness degenerate"
        );
        let matching: Vec<usize> = (0..case_count)
            .filter(|alternative| {
                plan.case_constructor_identity(c.admitted.continuation_origin, *alternative)
                    .expect("case identity")
                    == identity
            })
            .collect();
        assert_eq!(
            matching,
            vec![c.producer_alternative as usize],
            "the alternative is the unique case the producer's constructor selects"
        );

        // producer argument -> the semantic child inventory at that position.
        //
        // ON THIS WITNESS THE INVENTORY IS A SINGLETON, WHICH IS DEGENERATE.
        // The exact witness's producer construct has exactly one child, so
        // indexing it at the recursive position cannot distinguish that position
        // from "the only child" -- `AC-1`'s degenerate-witness case exactly. The
        // assertion is still made here, and then discharged non-degenerately on
        // the widened witness below; the singleton is asserted rather than
        // described so it cannot silently stop being one.
        let children = plan
            .semantic
            .child_origins(c.producer_construct_origin)
            .expect("children");
        assert_eq!(
            children.len(),
            1,
            "the exact witness's inventory is a singleton, which is why the widened one exists"
        );
        assert_eq!(
            c.producer_argument_origin, children[c.recursive_position as usize],
            "the argument is the construct's child at the recursive position"
        );

        // both bindings -> the checked-IH authority.
        //
        // POPULATION: the authority holds two bindings and the two members
        // resolve to DIFFERENT ones -- they differ in `frame_origin`, which is
        // what stops this from being one lookup asserted twice. They do share a
        // `recursive_position` of 0, so that field carries no discrimination
        // here and the row does not claim it does.
        let ih = build_checked_ih_bindings(&plan).expect("bindings");
        assert_eq!(ih.len(), 2, "two bindings, so neither lookup is forced");
        assert_eq!(ih.get(&c.producer_argument_origin), Some(&c.producer_argument_binding));
        assert_eq!(ih.get(&c.consuming_callee), Some(&c.consumer_binding));
        assert_ne!(
            c.producer_argument_binding.frame_origin, c.consumer_binding.frame_origin,
            "and the two bindings are genuinely different entries"
        );

        // RECURSIVE POSITION -- the conditional row, tested as conditional.
        //
        // Membership is SELECTED FROM `key.consumer_binding`: the position is
        // checked to be declared on the case, starting from a value the key
        // carries. That is all this row establishes on its own.
        let RuntimeExpr::ComputationalMatch { cases, .. } = plan
            .planned_occurrence_expr(c.admitted.continuation_origin)
            .expect("consumer")
        else {
            panic!("the consumer is not a computational match")
        };
        //
        // POPULATION, AND IT IS THE SECOND HALF OF WHY THIS ROW IS WEAK: the
        // selected case declares exactly ONE recursive position and its sibling
        // declares none. So membership discriminates between the two CASES and
        // not among positions, and no witness in this family can make it do the
        // latter. That bound is asserted, not described.
        let declared: Vec<Vec<usize>> = cases
            .iter()
            .map(|case| case.recursive_positions.clone())
            .collect();
        assert_eq!(
            declared,
            vec![vec![0usize], Vec::new()],
            "the declaration population this row selects from"
        );
        assert!(
            cases[c.producer_alternative as usize]
                .recursive_positions
                .contains(&(c.consumer_binding.recursive_position as usize)),
            "the position the consumer binding names is DECLARED on the case"
        );
        // Independence arrives only through the consumer binding being rebuilt
        // from the plan and compared in the final whole-key equality, which is
        // asserted here as the two together rather than claimed of the row.
        assert_eq!(
            ih.get(&c.consuming_callee).map(|b| b.recursive_position),
            Some(c.recursive_position),
            "and the rebuilt consumer binding independently names that position"
        );

        // 3. selected case body, consuming Call and callee.
        assert_eq!(
            c.selected_case_body,
            plan.semantic
                .child_origin(c.admitted.continuation_origin, 1 + c.producer_alternative as usize)
                .expect("case body")
        );
        assert_eq!(
            c.consuming_call,
            fusion_through_checked_wrappers(&plan, c.selected_case_body).expect("call")
        );
        assert_eq!(
            c.consuming_callee,
            plan.semantic.child_origin(c.consuming_call, 0).expect("callee")
        );
        assert_eq!(
            c.consumer_binding,
            CheckedIhBinding {
                frame_origin: c.admitted.continuation_origin,
                recursive_position: c.recursive_position,
            },
            "the consumer binding names THIS frame and position"
        );

        // 4. transport -> the resolved coordinate at that exact Call.
        //
        // POPULATION: the transport map resolves four Calls, so this is a lookup
        // that could have returned another coordinate.
        let transport = build_checked_transport(&plan, &oriented).expect("transport");
        assert!(
            transport.len() > 1,
            "a one-entry transport map would make the lookup degenerate: {transport:?}"
        );
        assert_eq!(transport.get(&c.consuming_call), Some(&c.checked_transport));

        // 5. the unique StaticBody triple.
        //
        // WHAT THIS ROW ACTUALLY ESTABLISHES, because two of the triple's three
        // members carry less than they appear to:
        //
        // - `invocation_callee` equals `producer_owner` BY CONSTRUCTION -- the
        //   edge search filters on exactly that -- so its agreement is a
        //   tautology and is asserted below as one rather than as evidence;
        // - `invocation_caller` COINCIDES with `consumer_owner` on this witness,
        //   which nothing in the mechanism requires;
        // - `invocation_callee_entry` COINCIDES with `admitted.result_root`.
        //
        // The informative content is the UNIQUENESS: two `StaticBody` edges
        // exist and exactly one enters the producer's unit.
        let edges = plan
            .semantic
            .static_body_call_edges(&plan.edges)
            .expect("edges");
        assert!(
            edges.len() > 1,
            "one edge overall would make uniqueness degenerate: {edges:?}"
        );
        assert_eq!(
            edges
                .iter()
                .filter(|edge| edge.1 == c.producer_owner)
                .count(),
            1,
            "exactly one of them enters the producer's unit"
        );
        assert_eq!(
            fusion_unique_static_body_triple(&plan, c.producer_owner).expect("triple"),
            Some((c.invocation_caller, c.invocation_callee, c.invocation_callee_entry)),
            "the triple is the unique StaticBody edge into the producer's unit"
        );
        assert_eq!(
            c.invocation_callee, c.producer_owner,
            "and the callee is the producer's unit BY CONSTRUCTION, not by measurement"
        );
        assert_eq!(
            (c.invocation_caller, c.invocation_callee_entry),
            (c.consumer_owner, c.admitted.result_root),
            "the two coincidences, pinned so they read as coincidences rather than as facts"
        );

        // 6. the owner split.
        assert_eq!(
            c.consumer_owner,
            occurrence_authority(&plan, c.admitted.continuation_origin)
                .expect("authority")
                .owner
        );
        assert_ne!(c.producer_owner, c.consumer_owner, "and the owners differ");

        // 7. the ordered input projection.
        //
        // `AC-2`: THE COUNT IS PART OF THE ROW. Without this the equality below
        // holds just as well when both sides are empty, which is the exact shape
        // the node exists to close -- an empty-vector agreement that reads as
        // coverage. Two ordered inputs, and the witness that produced them is
        // paired against a zero-input one in
        // `d2j_the_witness_projects_a_non_empty_ordered_input_run`.
        assert_eq!(
            c.continuation_inputs.len(),
            2,
            "the row rests on a TWO-input projection, not on an empty one"
        );
        assert_eq!(
            exact_continuation_source_environment(
                &plan,
                c.producer_owner,
                c.admitted.result_root,
                c.producer_construct_origin,
                c.consumer_owner,
                c.admitted.continuation_origin,
            )
            .expect("projection")
            .map(|environment| environment.inputs),
            Some(c.continuation_inputs.clone()),
            "the projection is the producer environment's own ordered inputs"
        );

        // THE ARGUMENT ROW, DISCHARGED NON-DEGENERATELY.
        //
        // Same family, one knob: a second nullary argument on the producer
        // construct. The inventory becomes two, so indexing it at the recursive
        // position now DISCRIMINATES -- the other child is a real alternative
        // the assertion would pick up if the position were derived wrongly. The
        // widened plan describes the widened source, because this is a positive
        // witness rather than a refusal control.
        let widened_declaration = d2j_declaration_under(D2jCause::ProducerArity);
        let mut widened_declarations = BTreeMap::new();
        widened_declarations.insert(D2J_DECLARATION, &widened_declaration);
        let widened_plan =
            plan_static_transition_graph(&entry, &widened_declarations).expect("plannable");
        let widened_oriented = d2j_oriented_plan_under(D2jCause::ProducerArity);
        let widened = enumerate_live_fusion_candidates(
            &widened_plan,
            &entry,
            &widened_declarations,
            Some(&widened_oriented),
        )
        .expect("enumerates");
        assert_eq!(widened.len(), 1, "widening must not cost the candidate");
        let w = &widened[0];
        // `RuntimeExpr` has no structural equality by design, so the shape is
        // asserted piecewise rather than against a constructed twin.
        let RuntimeExpr::Construct {
            constructor: widened_constructor,
            args: widened_args,
        } = widened_plan
            .planned_occurrence_expr(w.producer_construct_origin)
            .expect("producer")
        else {
            panic!("the widened producer is not a construct")
        };
        assert_eq!(widened_constructor, D2J_PRODUCER_CONSTRUCTOR);
        assert_eq!(widened_args.len(), 2, "the producer gained a second argument");
        assert!(
            matches!(
                &widened_args[1],
                RuntimeExpr::Construct { constructor, args }
                    if constructor == "ctor:prelude::Unit::MkUnit" && args.is_empty()
            ),
            "the added child is nullary, so it contributes no result origin and no marker edge"
        );

        // THE CENSUS -- BECAUSE THE SYMBOL IS NOT A SELECTOR.
        //
        // Two constructs in this fixture carry `D2J_PRODUCER_CONSTRUCTOR`: the
        // case-body producer and the OUTER match's scrutinee. An earlier
        // revision keyed the widening on the symbol and moved BOTH, so the
        // "producer-only" claim beside it was false and the mutation was not
        // the one the row attributes its discrimination to.
        //
        // Both occurrences are enumerated on both plans. The exact plan holds
        // them at arity one; the widened plan moves EXACTLY the producer.
        let arity_census = |census_plan: &StaticTransitionPlan<'_>| -> Vec<(StaticOriginId, usize)> {
            let mut found: Vec<(StaticOriginId, usize)> = census_plan
                .occurrence_authorities
                .iter()
                .map(|authority| authority.origin)
                .filter_map(|origin| match census_plan.planned_occurrence_expr(origin) {
                    Ok(RuntimeExpr::Construct { constructor, args })
                        if constructor == D2J_PRODUCER_CONSTRUCTOR =>
                    {
                        Some((origin, args.len()))
                    }
                    _ => None,
                })
                .collect();
            found.sort();
            found
        };
        let exact_census = arity_census(&plan);
        let widened_census = arity_census(&widened_plan);
        assert_eq!(
            exact_census.len(),
            2,
            "the symbol names TWO occurrences, which is the whole reason for this census: {exact_census:?}"
        );
        assert!(
            exact_census.iter().all(|(_, arity)| *arity == 1),
            "both are arity one before the knob: {exact_census:?}"
        );
        assert_eq!(
            widened_census.len(),
            2,
            "the knob adds no occurrence of the symbol: {widened_census:?}"
        );
        assert_eq!(
            widened_census
                .iter()
                .filter(|(_, arity)| *arity == 2)
                .map(|(origin, _)| *origin)
                .collect::<Vec<_>>(),
            vec![w.producer_construct_origin],
            "exactly one occurrence moved, and it is the producer the row is about"
        );
        assert_eq!(
            widened_census
                .iter()
                .filter(|(origin, _)| *origin != w.producer_construct_origin)
                .map(|(_, arity)| *arity)
                .collect::<Vec<_>>(),
            vec![1],
            "and the other same-symbol construct stayed at arity one"
        );
        let widened_children = widened_plan
            .semantic
            .child_origins(w.producer_construct_origin)
            .expect("children");
        assert_eq!(
            widened_children.len(),
            2,
            "TWO children, so the index is now a choice"
        );
        assert_eq!(
            w.producer_argument_origin, widened_children[w.recursive_position as usize],
            "the argument is the child AT the recursive position"
        );
        assert_ne!(
            w.producer_argument_origin, widened_children[1],
            "and not the other child, which is what the singleton could not say"
        );
    }

    /// `D2j` — five source-side causes, each refusing before any ID or
    /// descriptor exists.
    ///
    /// Every row is an executable assertion at the PLANE, where an id and a
    /// descriptor would exist if anything had been minted. The baseline mints
    /// exactly one first, so each refusal is a change and not the family's
    /// resting state.
    ///
    /// Each cause mutates the SOURCE and leaves the plan alone -- the plan is
    /// the correct description, and deriving it from the mutated body would move
    /// the description along with the artifact and leave nothing to catch.
    ///
    /// The refusals are attributed, not merely counted: each is required to name
    /// its own gate, so a cause that happened to break something upstream would
    /// fail rather than pass as coverage.
    #[test]
    fn d2j_the_source_side_causes_refuse_before_any_id_exists() {
        let baseline = d2j_plane_under(D2jCause::Exact).expect("the exact witness builds");
        assert_eq!(baseline.len(), 1, "the baseline mints exactly one identity");
        assert!(baseline
            .descriptor_for(StaticContinuationFusionId(0))
            .is_some());

        // Causes that refuse at the transport boundary, each naming its own
        // authority.
        for (cause, expected) in [
            (D2jCause::Frame, "checked plan frame marker is missing or transplanted"),
            (
                D2jCause::SelectedSlot,
                "checked computational-IH slot Runtime occurrences differ",
            ),
            (
                D2jCause::Invocation,
                "checked computational-IH call Runtime occurrences differ",
            ),
        ] {
            let refusal = d2j_plane_under(cause).expect_err("{cause:?} must refuse");
            assert!(
                format!("{refusal:?}").contains(expected),
                "{cause:?} must refuse at its OWN authority, not merely refuse: {refusal:?}"
            );
        }

        // Causes that form no candidate at all: nothing is minted, and there is
        // no id to inspect because none was created.
        for cause in [D2jCause::ExactSuffix, D2jCause::CallIdentity] {
            let plane = d2j_plane_under(cause).expect("still builds, with nothing to intern");
            assert!(
                plane.is_empty(),
                "{cause:?} must mint no key, id or descriptor: {plane:?}"
            );
            assert_eq!(plane.descriptor_for(StaticContinuationFusionId(0)), None);
        }
    }

    /// `D2j` — the SEGMENT-OWNER category, as a provenance and non-aliasing
    /// disposition rather than a sixth refusal.
    ///
    /// Removing the outer parameterisation does not refuse. It removes the
    /// consumer's two-entry ABI floor while leaving the producer inside its
    /// inner closure, so the checked transport, the exact suffix and the
    /// consuming `Call` all stay valid and a candidate is still formed. The
    /// owners it is formed with are different ones: the whole fusion is
    /// RE-HOMED.
    ///
    /// So the claim this control carries is not "the owner authority refuses"
    /// but the two things that are true and are worth pinning:
    ///
    /// 1. **Provenance** — each owner in each key equals the occurrence
    ///    authority of ITS OWN plan, so a moved owner is a real re-home and not
    ///    a relabelling.
    /// 2. **Non-aliasing** — a coherent source-side re-home yields a
    ///    structurally different complete key, and it differs in more than its
    ///    owners.
    ///
    /// The two planes number their identities independently, so nothing here
    /// compares an id across them.
    #[test]
    fn d2j_the_segment_owner_re_home_is_provenance_and_non_aliasing() {
        // REBASELINED, and the two sides no longer share a root. The sharing
        // invariant is PER CAUSE: `Exact` is the ABI-applied program, `ReHomed`
        // is the bare one, because that cause removes the outer closure and has
        // zero ABI inputs. Sharing one root here would have measured one of the
        // two against a program it does not describe.
        let (exact_entry, exact_declaration, exact_oriented) =
            d2j_checked_fixture_under(D2jCause::Exact);
        let mut exact_declarations = BTreeMap::new();
        exact_declarations.insert(D2J_DECLARATION, &exact_declaration);
        let exact_plan =
            plan_static_transition_graph(&exact_entry, &exact_declarations).expect("plannable");
        let exact_plane = build_static_continuation_fusion_plan(
            &exact_plan,
            &exact_entry,
            &exact_declarations,
            Some(&exact_oriented),
        )
        .expect("the exact witness builds");

        let (entry, rehomed_declaration, rehomed_oriented) =
            d2j_checked_fixture_under(D2jCause::ReHomed);
        let mut rehomed_declarations = BTreeMap::new();
        rehomed_declarations.insert(D2J_DECLARATION, &rehomed_declaration);
        let rehomed_plan =
            plan_static_transition_graph(&entry, &rehomed_declarations).expect("plannable");
        let rehomed_plane = build_static_continuation_fusion_plan(
            &rehomed_plan,
            &entry,
            &rehomed_declarations,
            Some(&rehomed_oriented),
        )
        .expect("the re-home still builds; nothing about transport changed");

        assert_eq!(exact_plane.len(), 1, "the exact plane mints one identity");
        assert_eq!(
            rehomed_plane.len(),
            1,
            "and the candidate SURVIVES the re-home, which is why this is not a refusal"
        );
        // Taken as each plane's sole key. The two planes number from zero
        // independently, so naming an id would be comparing two unrelated
        // counters rather than two keys.
        let exact = exact_plane.keys.first().expect("the exact key");
        let rehomed = rehomed_plane.keys.first().expect("the re-homed key");

        // 1. PROVENANCE — each owner against its own plan's authority.
        for (plan, key, side) in [
            (&exact_plan, exact, "exact"),
            (&rehomed_plan, rehomed, "re-homed"),
        ] {
            assert_eq!(
                key.producer_owner,
                occurrence_authority(plan, key.producer_construct_origin)
                    .expect("authority")
                    .owner,
                "{side}: the producer owner is ITS OWN plan's occurrence authority"
            );
            assert_eq!(
                key.consumer_owner,
                occurrence_authority(plan, key.admitted.continuation_origin)
                    .expect("authority")
                    .owner,
                "{side}: the consumer owner is ITS OWN plan's occurrence authority"
            );
            assert_ne!(
                key.producer_owner, key.consumer_owner,
                "{side}: and the split is intact, which is why neither side refuses"
            );
        }

        // 2. NON-ALIASING — the owner pair moves, and the key differs by more.
        assert_ne!(
            (exact.producer_owner, exact.consumer_owner),
            (rehomed.producer_owner, rehomed.consumer_owner),
            "a re-home that left the owner pair alone would not be one"
        );
        // At least one owner member must move; MEASURED, all four do, and the
        // invocation pair moves with them rather than trailing the split.
        for (label, before, after) in [
            ("producer", exact.producer_owner, rehomed.producer_owner),
            ("consumer", exact.consumer_owner, rehomed.consumer_owner),
            ("caller", exact.invocation_caller, rehomed.invocation_caller),
            ("callee", exact.invocation_callee, rehomed.invocation_callee),
        ] {
            assert_ne!(before, after, "the {label} owner moved");
        }
        assert_ne!(
            exact, rehomed,
            "the complete keys are two identities, not one seen twice"
        );

        // And the difference is NOT confined to the owners: the ordered input
        // run collapses with the ABI floor that produced it. This is the
        // measurement the category is retained for.
        assert_eq!(
            exact.continuation_inputs.len(),
            2,
            "the exact witness projects two ordered inputs"
        );
        assert!(
            rehomed.continuation_inputs.is_empty(),
            "and the re-home collapses the run to nothing: {:?}",
            rehomed.continuation_inputs
        );

        // The transport coordinate is UNCHANGED across the two, which is what
        // makes the difference attributable to the re-home rather than to
        // transport having moved underneath it.
        assert_eq!(
            exact.checked_transport, rehomed.checked_transport,
            "transport is the constant here, not the variable"
        );
    }

    #[test]
    fn d2j_the_witness_projects_a_non_empty_ordered_input_run() {
        // The bare witness: a fusion candidate, and an empty projection.
        let bare_declaration = d2g_declaration(true);
        let bare_entry = d2g_entry();
        let mut bare_declarations = BTreeMap::new();
        bare_declarations.insert(D2G_DECLARATION, &bare_declaration);
        let bare_plan =
            plan_static_transition_graph(&bare_entry, &bare_declarations).expect("plannable");
        let bare_oriented = d2g_oriented_plan();
        let bare = enumerate_live_fusion_candidates(
            &bare_plan,
            &bare_entry,
            &bare_declarations,
            Some(&bare_oriented),
        )
        .expect("enumerates");
        assert_eq!(bare.len(), 1, "the bare witness still reaches one candidate");
        assert_eq!(
            bare[0].continuation_inputs.len(),
            0,
            "and still projects nothing, which is what this witness exists to fix"
        );

        // The parameterised witness: the same candidate, with inputs to project.
        // REBASELINED onto the cause-selected applied root.
        let (entry, declaration, oriented) = d2j_checked_fixture_under(D2jCause::Exact);
        let mut declarations = BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
        let candidates =
            enumerate_live_fusion_candidates(&plan, &entry, &declarations, Some(&oriented))
                .expect("enumerates");
        assert_eq!(
            candidates.len(),
            1,
            "wrapping must not cost the candidate: {candidates:?}"
        );
        let inputs = &candidates[0].continuation_inputs;
        assert_eq!(
            inputs.len(),
            2,
            "THE COUNT: two ordered inputs, one per declared parameter: {inputs:?}"
        );

        // The members, so the count cannot be met by an unrelated projection.
        let consumer_owner = candidates[0].consumer_owner;
        let ContinuationSourceCoordinate::EntryAbi {
            source_owner: first_owner,
            ..
        } = inputs[0].coordinate
        else {
            panic!("the first input is not an entry-ABI coordinate: {:?}", inputs[0]);
        };
        for (ordinal, input) in inputs.iter().enumerate() {
            match &input.coordinate {
                ContinuationSourceCoordinate::EntryAbi {
                    source_owner,
                    source_abi_position,
                    source,
                } => {
                    assert_eq!(
                        *source_owner, first_owner,
                        "every input in the run comes from ONE unit's entry ABI"
                    );
                    assert_eq!(
                        *source_abi_position as usize, ordinal,
                        "and they are ORDERED, position matching ordinal"
                    );
                    assert!(
                        matches!(source, ContinuationInputSource::Parameter),
                        "and sourced from a parameter: {source:?}"
                    );
                }
                other => panic!("an input projected from an unexpected coordinate: {other:?}"),
            }
        }
        assert_ne!(
            candidates[0].producer_owner, consumer_owner,
            "and the candidate is still the cross-unit one"
        );

        // THE OWNER RELATION, ASSERTED RATHER THAN OBSERVED, AND IT FALSIFIED
        // TWO SUCCESSIVE GUESSES OF MINE.
        //
        // `first_owner` is read out of the run being checked, so "every input
        // shares it" is a self-consistency check and cannot constrain WHICH unit
        // it is. Two revisions carried a prose claim about that unit while
        // asserting nothing that could fail if the claim were wrong: first that
        // the inputs are parameters of the PRODUCER's unit, then that they are
        // of neither fusion side. Both were wrong.
        //
        // MEASURED: the inputs are entry-ABI parameters of the CONSUMER's own
        // unit, which is also what the `R3` before-hole measurement recorded
        // (`source_owner` 0 against a consumer owner of 0). The continuation
        // inputs are what the consuming frame supplies, so this is the relation
        // that was there all along.
        assert_eq!(
            first_owner, consumer_owner,
            "the entry-ABI owner is the CONSUMER's unit"
        );
        assert_ne!(
            first_owner, candidates[0].producer_owner,
            "and not the producer's, which was the first wrong guess"
        );
        assert_eq!(
            first_owner,
            PredeclaredFunctionId(3),
            "and it is the measured unit, pinned so a shift in the fixture is visible"
        );
    }

    /// `D2g` `AC-1` — the checked twin reaches the SAME producer to IH-consumer
    /// relation as the unmarked witness, through the landed authority.
    ///
    /// The relation: the consumer's selected case body is the exact suffix iff
    /// its callee `Var` resolves to
    /// `CheckedIhBinding { frame_origin: <the consumer frame>, recursive_position }`.
    /// Read out of `build_checked_ih_bindings`, the authority that landed in
    /// `D2e`, and not re-derived here.
    ///
    /// Both forms are measured in one loop, so a twin that lost the relation
    /// could not pass on the unmarked side alone. Measured on this one fixture
    /// pair; the claim is about it, not about checked wrapping in general.
    #[test]
    fn d2g_ac1_checked_twin_reaches_the_same_producer_to_ih_consumer_relation() {
        for checked in [false, true] {
            let declaration = d2g_declaration(checked);
            let entry = d2g_entry();
            let mut declarations = BTreeMap::new();
            declarations.insert(D2G_DECLARATION, &declaration);
            let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
            let bindings = build_checked_ih_bindings(&plan).expect("bindings derive");

            let body = *plan
                .declaration_occurrences
                .get(D2G_DECLARATION)
                .expect("the declaration is planned");
            let outer = d2g_through_wrappers(&plan, body).expect("outer frame");
            let case_body = plan
                .semantic
                .child_origin(outer, 1)
                .expect("the OutNode case body");
            let call = d2g_through_wrappers(&plan, case_body).expect("the consuming Call");
            assert!(
                matches!(
                    plan.planned_occurrence_expr(call).expect("call expr"),
                    RuntimeExpr::Call { .. }
                ),
                "checked={checked}: the selected case body must reach a Call"
            );
            let callee = plan.semantic.child_origin(call, 0).expect("callee");
            assert_eq!(
                bindings.get(&callee).copied(),
                Some(CheckedIhBinding {
                    frame_origin: outer,
                    recursive_position: 0,
                }),
                "checked={checked}: the consuming Call's callee must resolve to THIS \
                 consumer frame's hypothesis at position 0"
            );
        }
    }

    /// `D2g` `AC-2` POSITIVE — a complete independently authored plan authorizes
    /// this fixture, and the check is not circular.
    ///
    /// The plan's `runtime_marker_locations` are hand-derived from the
    /// collector's edge convention rather than collected from the fixture, so
    /// this compares two independently produced descriptions of the same tree.
    /// It covers all three marker populations, the frame fingerprints, the
    /// declaration paths, the checked occurrence paths and the exact runtime
    /// locations, because `validate_oriented_subcontinuation_transport` requires
    /// every one of them.
    ///
    /// Alone this proves only that the two agree; the mutation below is what
    /// makes the agreement causal.
    #[test]
    fn d2g_ac2_a_complete_independent_plan_positively_validates_the_twin() {
        use crate::cranelift_backend::planning::validate_oriented_subcontinuation_transport;
        let declaration = d2g_declaration(true);
        let entry = d2g_entry();
        let mut declarations = BTreeMap::new();
        declarations.insert(D2G_DECLARATION, &declaration);
        let oriented = d2g_oriented_plan();
        validate_oriented_subcontinuation_transport(&entry, &declarations, Some(&oriented))
            .expect("the independently authored plan must authorize this fixture");
    }

    /// `D2g` `AC-2` DISCRIMINATOR — the plan is held FIXED and the RUNTIME moves.
    ///
    /// An earlier revision mutated the plan's own marker location and re-sealed
    /// its fingerprint. That shows only that the validator notices when its
    /// description is edited; it says nothing about whether it detects a change
    /// in the artifact. **Here the plan is the same object the positive
    /// accepted, byte for byte, including every fingerprint.**
    ///
    /// What moves is the Runtime declaration: the outer slot wrapper stops
    /// wrapping the selected case body and wraps the sibling `OutLeaf` case
    /// instead. That is a real case body, so the refusal cannot be about
    /// malformedness. The invocation marker stays on the consuming `Call`.
    ///
    /// The actual collected location is asserted BEFORE validation, to show the
    /// mutation landed where intended -- and it is used only as an observation,
    /// never to build or feed the plan.
    #[test]
    fn d2g_ac2_relocating_the_runtime_marker_against_a_fixed_plan_is_refused() {
        use crate::cranelift_backend::planning::{
            collect_checked_oriented_markers, validate_oriented_subcontinuation_transport,
            CheckedOrientedMarkerSets,
        };

        // The plan the positive used, unmodified.
        let oriented = d2g_oriented_plan();
        assert_eq!(
            oriented, d2g_oriented_plan(),
            "the plan must be the positive's plan; nothing here may reseal or edit it"
        );

        let mutated_body = d2g_declaration_body_relocated(true, true);
        let mut markers = CheckedOrientedMarkerSets::default();
        collect_checked_oriented_markers(
            &mutated_body,
            &mut markers,
            D2G_DECLARATION,
            &mut Vec::new(),
        )
        .expect("markers collect");
        let observed = markers
            .computational_ih_slots
            .get(&(D2G_OUTER_SLOT, vec![20, 0]))
            .expect("the outer slot marker is still present, at its new home");
        assert_eq!(
            observed.iter().cloned().collect::<Vec<_>>(),
            vec![vec![0, 2]],
            "the mutation must have moved the outer slot marker to the sibling case"
        );
        assert_ne!(
            observed.iter().cloned().collect::<Vec<_>>(),
            vec![d2g_outer_slot_location()],
            "and it must no longer sit where the fixed plan says it does"
        );

        let declaration = RuntimeDeclaration {
            symbol: D2G_DECLARATION.to_string(),
            kind: RuntimeDeclarationKind::Transparent { body: mutated_body },
            metadata: crate::RuntimeSymbolMetadata {
                obligations: Default::default(),
                obligation_metadata: Default::default(),
                assumptions: Default::default(),
                assumption_trust_metadata: Default::default(),
                trusted_base_delta: Default::default(),
                lowerability: None,
                unsupported: None,
                runtime_checks: Default::default(),
                capabilities: Default::default(),
                effects: Default::default(),
            },
        };
        let entry = d2g_entry();
        let mut declarations = BTreeMap::new();
        declarations.insert(D2G_DECLARATION, &declaration);

        let refusal =
            validate_oriented_subcontinuation_transport(&entry, &declarations, Some(&oriented))
                .expect_err("a relocated runtime marker must be refused against the fixed plan");
        let rendered = format!("{refusal:?}");
        assert!(
            rendered.contains("checked computational-IH slot Runtime occurrences differ"),
            "the refusal must name the exact slot location mismatch: {rendered}"
        );
    }

    /// `D2g` — each slot's constructor fact is its OWN frame's, and both are
    /// pinned because the validator cannot tell them apart.
    ///
    /// The outer frame eliminates `D2gOut` and the inner one eliminates
    /// `D2gIn`. An earlier revision wrote the outer constructor into both slot
    /// templates. `validate_oriented_subcontinuation_transport` checks
    /// identities, paths, locations and fingerprints -- **not** whether a slot's
    /// constructor matches the case it stands for -- so the mismatch validated
    /// cleanly and would have travelled into the key as a wrong fact.
    ///
    /// A pin is the only thing that catches it, so the two facts are asserted
    /// separately and asserted to DIFFER; equal values would mean the
    /// hardcoding came back.
    #[test]
    fn d2g_each_slot_carries_its_own_frames_constructor() {
        let oriented = d2g_oriented_plan();
        let constructor = |slot_template_id: u64| {
            oriented
                .computational_ih_slots
                .iter()
                .find(|slot| slot.slot_template_id == slot_template_id)
                .map(|slot| slot.constructor.clone())
                .expect("the slot is planned")
        };
        assert_eq!(constructor(D2G_OUTER_SLOT), D2G_OUTER_SLOT_CONSTRUCTOR);
        assert_eq!(constructor(D2G_INNER_SLOT), D2G_INNER_SLOT_CONSTRUCTOR);
        assert_ne!(
            constructor(D2G_OUTER_SLOT),
            constructor(D2G_INNER_SLOT),
            "the two slots eliminate different constructors; equal values here means \
             one was hardcoded for both again"
        );

        // And the facts agree with the fixture, so the pin is about the program
        // rather than about two constants agreeing with each other.
        let body = d2g_declaration_body(true);
        let rendered = format!("{body:?}");
        assert!(
            rendered.contains(D2G_OUTER_SLOT_CONSTRUCTOR)
                && rendered.contains(D2G_INNER_SLOT_CONSTRUCTOR),
            "both constructors must actually occur in the fixture"
        );
    }

    /// `D2g` — the capture helper resolves the exact PLAN-BACKED triple on the
    /// twin, and resolves nothing on the unmarked witness.
    ///
    /// A raw wrapper value is not authority. Each member is resolved against the
    /// plan by its id AND the checked occurrence path it was declared at, and
    /// the three must be related -- the slot belongs to the frame, the call is
    /// that slot's call -- before a coordinate is recorded.
    #[test]
    fn d2g_ac2_transport_resolves_plan_backed_on_the_twin_and_is_absent_on_the_witness() {
        let oriented = d2g_oriented_plan();
        for (checked, expect_some) in [(true, true), (false, false)] {
            let declaration = d2g_declaration(checked);
            let entry = d2g_entry();
            let mut declarations = BTreeMap::new();
            declarations.insert(D2G_DECLARATION, &declaration);
            let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
            let transport = build_checked_transport(&plan, &oriented).expect("transport derives");

            let body = *plan
                .declaration_occurrences
                .get(D2G_DECLARATION)
                .expect("declaration planned");
            let outer = d2g_through_wrappers(&plan, body).expect("outer frame");
            let case_body = plan.semantic.child_origin(outer, 1).expect("case body");
            let call = d2g_through_wrappers(&plan, case_body).expect("consuming Call");

            if expect_some {
                assert_eq!(
                    transport.get(&call),
                    Some(&CheckedTransportCoordinate {
                        frame_id: D2G_OUTER_FRAME,
                        slot_template_id: D2G_OUTER_SLOT,
                        slot_occurrence_path: vec![20, 0],
                        call_template_id: D2G_CALL,
                        call_occurrence_path: vec![30, 0],
                    }),
                    "the consuming Call carries the resolved, related triple"
                );
            } else {
                assert!(
                    transport.is_empty(),
                    "the unmarked witness resolves no coordinate anywhere: {transport:?}"
                );
            }
        }
    }

    /// `D2g` — a resolved id with a BROKEN relationship is not authority.
    ///
    /// Every id still resolves; only the slot-to-frame relation is severed. A
    /// helper that recorded "three markers were in scope and all three ids exist"
    /// would still answer here, which is what this rules out.
    #[test]
    fn d2g_ac2_a_broken_slot_to_frame_relation_resolves_nothing() {
        let declaration = d2g_declaration(true);
        let entry = d2g_entry();
        let mut declarations = BTreeMap::new();
        declarations.insert(D2G_DECLARATION, &declaration);
        let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");

        let mut oriented = d2g_oriented_plan();
        let slot = oriented
            .computational_ih_slots
            .iter_mut()
            .find(|slot| slot.slot_template_id == D2G_OUTER_SLOT)
            .expect("outer slot");
        // The slot now claims the INNER frame while the marker sits inside the
        // outer one. Both frames are real and both ids resolve.
        slot.frame_template_id = D2G_INNER_FRAME;

        let transport = build_checked_transport(&plan, &oriented).expect("transport derives");
        assert!(
            transport.is_empty(),
            "a slot that does not belong to the frame in scope must resolve nothing: \
             {transport:?}"
        );
    }

    /// `D2g` `AC-4` — the absence control is exercised in the direction that can
    /// FAIL.
    ///
    /// "No markers and no plan is accepted" passes whether or not the validator
    /// can see anything, because that is its trivially-accepting case. The
    /// discriminating half is markers present with NO plan, which must be
    /// refused -- and that is what would have caught a validator wired to accept
    /// everything.
    #[test]
    fn d2g_ac4_transport_validator_refuses_markers_without_a_plan() {
        use crate::cranelift_backend::planning::validate_oriented_subcontinuation_transport;
        let unmarked = d2g_declaration(false);
        let checked = d2g_declaration(true);
        let entry = d2g_entry();

        let mut unmarked_declarations = BTreeMap::new();
        unmarked_declarations.insert(D2G_DECLARATION, &unmarked);
        validate_oriented_subcontinuation_transport(&entry, &unmarked_declarations, None)
            .expect("no markers and no plan is the lawful absent case");

        let mut checked_declarations = BTreeMap::new();
        checked_declarations.insert(D2G_DECLARATION, &checked);
        let refusal =
            validate_oriented_subcontinuation_transport(&entry, &checked_declarations, None)
                .expect_err("markers present with no plan must be refused");
        let rendered = format!("{refusal:?}");
        assert!(
            rendered.contains("checked subcontinuation markers have no checked plan metadata"),
            "the refusal must be the transport one: {rendered}"
        );
    }

    /// `D2g` `AC-2` census — the marker populations, per fixture.
    ///
    /// Stated per fixture because that is the shape of the fact: this unmarked
    /// body has none and this twin has all three. Neither is a property of
    /// checked fixtures in general.
    #[test]
    fn d2g_ac2_marker_census_per_fixture() {
        use crate::cranelift_backend::planning::{
            collect_checked_oriented_markers, CheckedOrientedMarkerSets,
        };
        let census = |expr: &RuntimeExpr| {
            let mut markers = CheckedOrientedMarkerSets::default();
            collect_checked_oriented_markers(expr, &mut markers, D2G_DECLARATION, &mut Vec::new())
                .expect("markers collect");
            (
                markers.computational_ih_slots.len(),
                markers.computational_ih_calls.len(),
                markers.recursive_calls.len(),
            )
        };
        assert_eq!(
            census(&d2g_declaration_body(false)),
            (0, 0, 0),
            "the unmarked body: no slot, no invocation, no recursive-call marker"
        );
        assert_eq!(
            census(&d2g_declaration_body(true)),
            (2, 1, 0),
            "the twin: two IH-slot markers, one per recursive case, and one \
             IH-invocation marker at the consuming Call"
        );
    }

    /// `D2g` `AC-3` — the twin's coordinates are FRESH, and some COINCIDE.
    ///
    /// Checked markers are real occurrences, so coordinates below the first
    /// wrapper shift. Measured, and not what I predicted: the declaration body,
    /// the selected case body and the scrutinee move, while others land on the
    /// same origin in both forms.
    ///
    /// The coinciding coordinate is the dangerous one. A number carried over
    /// from the unmarked witness is then right BY ACCIDENT -- it survives
    /// review, survives a spot check, and is wrong the moment the fixture
    /// changes.
    #[test]
    fn d2g_ac3_the_twins_coordinates_are_fresh_and_some_coincide() {
        let roles = |checked: bool| {
            let declaration = d2g_declaration(checked);
            let entry = d2g_entry();
            let mut declarations = BTreeMap::new();
            declarations.insert(D2G_DECLARATION, &declaration);
            let plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
            let body = *plan
                .declaration_occurrences
                .get(D2G_DECLARATION)
                .expect("declaration planned");
            let outer = d2g_through_wrappers(&plan, body).expect("outer frame");
            let case_body = plan.semantic.child_origin(outer, 1).expect("case body");
            let call = d2g_through_wrappers(&plan, case_body).expect("consuming Call");
            let scrutinee = plan.semantic.child_origin(outer, 0).expect("scrutinee");
            vec![body, outer, case_body, call, scrutinee]
        };
        let unmarked = roles(false);
        let twin = roles(true);
        println!("D2G unmarked (body, outer, case_body, call, scrutinee) = {unmarked:?}");
        println!("D2G twin     (body, outer, case_body, call, scrutinee) = {twin:?}");

        let agreeing = unmarked
            .iter()
            .zip(&twin)
            .filter(|(left, right)| left == right)
            .count();
        assert!(
            agreeing < unmarked.len(),
            "the wrappers must move at least one coordinate, or nothing here is fresh: \
             unmarked={unmarked:?} twin={twin:?}"
        );
        assert!(
            agreeing > 0,
            "and at least one must coincide, or this fixture cannot demonstrate that \
             agreement is uninformative: unmarked={unmarked:?} twin={twin:?}"
        );
    }

    /// **`D3` grounding — THE TWO COMPOSED CONTINUATION EDGES ARE NAMED BY THE
    /// FUSION KEY'S TWO CHECKED IH BINDINGS, and each target's incoming call
    /// domain is closed.**
    ///
    /// Ruled at `evt_1t3f4e8100rb5`: before any body change, the exact two-edge
    /// composition population must be grounded, and if the planner cannot derive
    /// both exact identities and their exact consumer edges **without a new
    /// source fact**, the implementer must stop and report the missing relation.
    /// This is that grounding, made durable rather than left in a probe.
    ///
    /// **THE DERIVATION CLOSES, and the relation is already checked.** The key's
    /// two `CheckedIhBinding`s name the two layers exactly:
    ///
    /// - `consumer_binding.frame_origin` is the OUTER specialization's
    ///   continuation origin -- 10 on `Exact`, 6 on `ReHomed`;
    /// - `producer_argument_binding.frame_origin` is the INNER one's -- 25 and
    ///   21.
    ///
    /// ⇒ **This is the same pair whose NON-equality the preflight comment above
    /// documents.** That comment records them as "different checked frames by
    /// design (measured 25 and 10)" and warns that asserting them equal would
    /// refuse the witness. The two frames are the two composition layers, which
    /// is why they differ -- so the fact that already forbids one check is what
    /// supplies this one.
    ///
    /// A structural substitute exists and is deliberately NOT used: on both
    /// witnesses `child_origins(producer_body)` is a one-element run holding the
    /// inner frame. It agrees here, but it is a positional read of the body's
    /// shape rather than a checked relation, and it would stop agreeing the
    /// moment a producer body carried more than one child.
    ///
    /// **MEASURED:** both witnesses, fusion plane installed, claims preflighted.
    /// Each specialization's complete incoming `ContinuationCallIdentity` domain
    /// is enumerated; the outer and inner classifications are disjoint and each
    /// selects exactly one specialization.
    /// **CLAIMED:** the composition population the ruled relation must mint is
    /// derivable from relations that already exist, so no new source fact is
    /// required at the minting seat.
    /// **THE GAP:** this pins the DERIVATION of the two edges. It pins no
    /// emitter behaviour and no composition disposition.
    ///
    /// ---- SUPERSEDED NOTE, kept because it recorded a real finding and its
    /// ---- conclusion was overtaken. Ruled at `evt_7akh94dvqeqap`.
    ///
    /// This section used to read that every incoming domain here is a singleton
    /// and that the ruled "every incoming identity is composed" partition was
    /// therefore satisfied vacuously, with two source fixtures owed to fix it.
    /// **The singleton measurement stands; the conclusion drawn from it does
    /// not.** The residual-direct-caller population is now REJECTED as a planner
    /// alias by [`validate_continuation_specialization_closure`]'s injective
    /// call-target law, so it is not a lawful state to preserve and no source
    /// fixture is owed for it. The same-body sibling has a planner-relation
    /// control of its own. **A specialization's liveness is decided by its own
    /// unique edge**, so there is no incoming-domain scan left for a singleton
    /// domain to make vacuous.
    #[test]
    fn d3_the_two_composed_edges_are_named_by_the_keys_checked_bindings() {
        let mut rows = Vec::new();
        for cause in [D2jCause::Exact, D2jCause::ReHomed] {
            let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
            let mut declarations = BTreeMap::new();
            declarations.insert(D2J_DECLARATION, &declaration);
            let mut plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
            let resolved =
                build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
                    .expect("the witness resolves a plane");
            let mut plane = StaticContinuationFusionPlan::default();
            for key in resolved.installed_keys().to_vec() {
                plane.intern(key).expect("interns");
            }
            plan.install_static_continuation_fusions(plane)
                .expect("installs");
            let ledger = FusionRegionClaimLedger::preflight(&plan).expect("claims");
            let id = *ledger.planned().iter().next().expect("one region");
            let claim = ledger.claim(id).expect("outstanding");

            let key = plan
                .continuation_fusions()
                .expect("views")
                .into_iter()
                .find(|view| view.id() == id)
                .expect("the installed view")
                .key()
                .clone();

            let specialization_at = |frame: StaticOriginId, owner: PredeclaredFunctionId| {
                let matching = plan
                    .continuation_units()
                    .expect("units")
                    .into_iter()
                    .filter(|unit| {
                        unit.continuation_origin() == frame && unit.consumer_owner() == owner
                    })
                    .map(|unit| unit.id())
                    .collect::<Vec<_>>();
                matching
            };
            let outer = specialization_at(key.consumer_binding.frame_origin, claim.consumer_owner());
            let inner = specialization_at(
                key.producer_argument_binding.frame_origin,
                claim.producer_owner(),
            );
            let incoming = |target: ContinuationSpecializationId| {
                plan.continuation_calls()
                    .expect("calls")
                    .iter()
                    .filter(|call| call.target() == target)
                    .count()
            };

            rows.push((
                cause,
                outer.len(),
                inner.len(),
                outer != inner,
                outer.first().map(|id| incoming(*id)),
                inner.first().map(|id| incoming(*id)),
            ));
        }

        assert_eq!(
            rows,
            vec![
                (D2jCause::Exact, 1, 1, true, Some(1), Some(1)),
                (D2jCause::ReHomed, 1, 1, true, Some(1), Some(1)),
            ],
            "each checked binding selects EXACTLY ONE specialization, the outer and inner \
             selections are disjoint, and each target's complete incoming call domain is a \
             singleton -- so the composition population derives from existing relations, and the \
             composed-vs-residual partition is measured DEGENERATE on this family"
        );
    }

    /// **`D3` — the fusion mints exactly TWO composed edges, one per ruled
    /// checked binding, keyed by whole call identity.**
    ///
    /// Ruled at `evt_1t3f4e8100rb5`. The outer layer is selected by the key's
    /// checked consumer binding, the inner by its checked producer-argument
    /// binding, each conjoined with the owner that binding belongs to. Every
    /// other edge in the compile is unrecorded and keeps the byte-identical
    /// `DirectCall` path.
    ///
    /// **MEASURED** on both witnesses: two records, distinct identities,
    /// distinct targets, one `Outer` and one `Inner`, and the emission owner of
    /// each is the `Predeclared` owner its binding names -- `Exact` outer at
    /// frame 10 under unit 3 and inner at frame 25 under unit 2; `ReHomed`
    /// outer at 6 under unit 1 and inner at 21 under unit 3.
    /// **CLAIMED:** the composition population is exactly the two ruled layers,
    /// so no third edge can be composed and neither layer can be composed twice.
    /// **THE GAP:** this pins the PLANNER relation. No edge is consumed here --
    /// the funnel, the local selected-body lowering, `ComposedCall` and the
    /// transport-instance closeout are owed, and until they land these records
    /// are minted and unread.
    #[test]
    fn d3_the_fusion_mints_exactly_two_composed_edges_one_per_checked_binding() {
        let mut rows = Vec::new();
        for cause in [D2jCause::Exact, D2jCause::ReHomed] {
            let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
            let mut declarations = BTreeMap::new();
            declarations.insert(D2J_DECLARATION, &declaration);
            let mut plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
            let resolved =
                build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
                    .expect("the witness resolves a plane");
            let mut plane = StaticContinuationFusionPlan::default();
            for key in resolved.installed_keys().to_vec() {
                plane.intern(key).expect("interns");
            }
            plan.install_static_continuation_fusions(plane)
                .expect("installs");
            let mut claims = FusionRegionClaimLedger::preflight(&plan).expect("claims");
            plan.install_fusion_owned_bodies(&mut claims)
                .expect("ownership installs");

            // `evt_6bm54j10w1n88` — the two ruled selections are still made,
            // but they land in DIFFERENT relations: `Inner` in the composed-edge
            // map, `Outer` in the fusion-owned realization map.
            let edges = plan.fusion_composed_edges();
            let outer = plan.fusion_outer_realizations();
            let mut layers = edges
                .values()
                .map(|edge| (edge.layer(), edge.target(), edge.emission_owner()))
                .chain(outer.values().map(|realization| {
                    (
                        FusionCompositionLayer::Outer,
                        realization.target(),
                        realization.emission_owner(),
                    )
                }))
                .collect::<Vec<_>>();
            layers.sort_by_key(|(layer, _, _)| match layer {
                FusionCompositionLayer::Outer => 0,
                FusionCompositionLayer::Inner => 1,
            });
            let distinct_targets = edges
                .values()
                .map(|edge| edge.target())
                .chain(outer.values().map(|realization| realization.target()))
                .collect::<BTreeSet<_>>()
                .len();
            rows.push((cause, edges.len(), outer.len(), distinct_targets, layers));
        }

        assert_eq!(
            rows,
            vec![
                (
                    D2jCause::Exact,
                    1,
                    1,
                    2,
                    vec![
                        (
                            FusionCompositionLayer::Outer,
                            ContinuationSpecializationId(1),
                            ContinuationEmissionOwner::Predeclared(PredeclaredFunctionId(3)),
                        ),
                        (
                            FusionCompositionLayer::Inner,
                            ContinuationSpecializationId(0),
                            ContinuationEmissionOwner::Predeclared(PredeclaredFunctionId(2)),
                        ),
                    ],
                ),
                (
                    D2jCause::ReHomed,
                    1,
                    1,
                    2,
                    vec![
                        (
                            FusionCompositionLayer::Outer,
                            ContinuationSpecializationId(1),
                            ContinuationEmissionOwner::Predeclared(PredeclaredFunctionId(1)),
                        ),
                        (
                            FusionCompositionLayer::Inner,
                            ContinuationSpecializationId(0),
                            ContinuationEmissionOwner::Predeclared(PredeclaredFunctionId(3)),
                        ),
                    ],
                ),
            ],
            "exactly two composed edges per fusion, one per ruled checked binding, on DISTINCT \
             targets, each emitted by the owner its own binding names"
        );
    }

    /// **`D3` — `P = O ⊎ F` and `T = O_t ⊎ F_t`, as exact SET relations, and
    /// the fusion-scoped join between the two composition objects.**
    ///
    /// Ruled `evt_48rwarx25pj2p` §3 (the single residual projection) and
    /// `evt_6kn9ckdnbf0ph` §2 (the join is `edge.fusion == claim.fusion`, and
    /// the ordinary consuming call is deliberately NOT in
    /// `dom(FusionComposedEdge)`).
    ///
    /// **MEASURED** on both witnesses: the exact planned identity population is
    /// the two composed edges and nothing else, so `O` is empty and `F = P`;
    /// the target population is likewise wholly fused. Each fusion carries
    /// exactly one `Outer` and one `Inner` layer, and each names a fusion that
    /// owns a producer body.
    /// **CLAIMED:** the ordinary and fusion-local populations partition the
    /// planned ones exactly -- disjoint and covering -- on identities and on
    /// targets, so a consumer reading `O`/`O_t` sees every ordinary member and
    /// no fused one.
    /// **THE GAP, stated because a wholly-fused family cannot exercise the
    /// residual half:** `O` and `O_t` are EMPTY here, so this row measures the
    /// partition's shape and the join, not the residual path's behaviour. The
    /// population that makes `O` non-empty is the same-body composed/ordinary
    /// discriminator (`evt_6kn9ckdnbf0ph` §5), which is owed with the local
    /// lowering; until it lands, nothing here should be read as evidence that
    /// an ordinary identity survives beside a fused one.
    #[test]
    fn d3_the_ordinary_and_fusion_local_populations_partition_the_planned_ones() {
        let mut rows = Vec::new();
        for cause in [D2jCause::Exact, D2jCause::ReHomed] {
            let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
            let mut declarations = BTreeMap::new();
            declarations.insert(D2J_DECLARATION, &declaration);
            let mut plan = plan_static_transition_graph(&entry, &declarations).expect("plannable");
            let resolved =
                build_static_continuation_fusion_plan(&plan, &entry, &declarations, Some(&oriented))
                    .expect("the witness resolves a plane");
            let mut plane = StaticContinuationFusionPlan::default();
            for key in resolved.installed_keys().to_vec() {
                plane.intern(key).expect("interns");
            }
            plan.install_static_continuation_fusions(plane)
                .expect("installs");
            let mut claims = FusionRegionClaimLedger::preflight(&plan).expect("claims");
            plan.install_fusion_owned_bodies(&mut claims)
                .expect("ownership installs");

            let planned = plan.continuation_call_identities().expect("planned identities");
            let ordinary = plan
                .ordinary_continuation_call_identities()
                .expect("ordinary identities");
            // `evt_6bm54j10w1n88` — the fused half is now TWO classes, and
            // this control ranges over their union because the partition law
            // does. ⛔ Their mutual disjointness is asserted separately below:
            // folding them together first and then testing the union against
            // `planned` would hide an identity that is in both, which is the one
            // failure the binary form could not have.
            let inner = plan
                .fusion_composed_edges()
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>();
            let outer = plan
                .fusion_outer_realizations()
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>();
            assert!(
                inner.is_disjoint(&outer),
                "{cause:?}: an identity is both locally composed and fusion-owned"
            );
            let fused = inner.union(&outer).cloned().collect::<BTreeSet<_>>();
            let planned_targets = plan
                .continuation_units()
                .expect("units")
                .iter()
                .map(|unit| unit.id())
                .collect::<BTreeSet<_>>();
            let ordinary_targets = plan
                .ordinary_continuation_targets()
                .expect("ordinary targets");
            let inner_targets = plan
                .fusion_composed_edges()
                .values()
                .map(|edge| edge.target())
                .collect::<BTreeSet<_>>();
            let outer_targets = plan
                .fusion_outer_realizations()
                .values()
                .map(|realization| realization.target())
                .collect::<BTreeSet<_>>();
            assert!(
                inner_targets.is_disjoint(&outer_targets),
                "{cause:?}: a target is both a local composition and a fusion-owned realization"
            );
            let fused_targets = inner_targets
                .union(&outer_targets)
                .cloned()
                .collect::<BTreeSet<_>>();

            // The partition, asserted as SETS in both directions rather than as
            // sizes: disjoint, and covering.
            assert!(
                ordinary.is_disjoint(&fused),
                "{cause:?}: an identity is both ordinary and fusion-local"
            );
            assert_eq!(
                ordinary.union(&fused).cloned().collect::<BTreeSet<_>>(),
                planned,
                "{cause:?}: O union F must be exactly the planned identity population"
            );
            assert!(
                ordinary_targets.is_disjoint(&fused_targets),
                "{cause:?}: a target is both ordinary and fusion-local"
            );
            assert_eq!(
                ordinary_targets
                    .union(&fused_targets)
                    .cloned()
                    .collect::<BTreeSet<_>>(),
                planned_targets,
                "{cause:?}: O_t union F_t must be exactly the planned target population"
            );

            // The join, per fusion, from the edge side and the claim side.
            let mut layers: BTreeMap<StaticContinuationFusionId, Vec<FusionCompositionLayer>> =
                BTreeMap::new();
            for edge in plan.fusion_composed_edges().values() {
                assert!(
                    claims.claim(edge.fusion()).is_some(),
                    "{cause:?}: a composed edge names a fusion with no claim"
                );
                layers.entry(edge.fusion()).or_default().push(edge.layer());
            }
            let mut join = layers.into_values().collect::<Vec<_>>();
            for entry in &mut join {
                entry.sort();
            }

            rows.push((
                cause,
                planned.len(),
                ordinary.len(),
                fused.len(),
                planned_targets.len(),
                ordinary_targets.len(),
                fused_targets.len(),
                join,
            ));
        }

        assert_eq!(
            rows,
            vec![
                (
                    D2jCause::Exact,
                    2,
                    0,
                    2,
                    2,
                    0,
                    2,
                    // `evt_6bm54j10w1n88` — the composed-edge map now holds
                    // the Inner layer ONLY. The Outer layer did not disappear;
                    // it moved to `fusion_outer_realizations`, where the control
                    // above asserts exactly one per fusion.
                    vec![vec![FusionCompositionLayer::Inner]],
                ),
                (
                    D2jCause::ReHomed,
                    2,
                    0,
                    2,
                    2,
                    0,
                    2,
                    // `evt_6bm54j10w1n88` — the composed-edge map now holds
                    // the Inner layer ONLY. The Outer layer did not disappear;
                    // it moved to `fusion_outer_realizations`, where the control
                    // above asserts exactly one per fusion.
                    vec![vec![FusionCompositionLayer::Inner]],
                ),
            ],
            "both witnesses are WHOLLY fused: every planned identity and every target is \
             fusion-local, the residual halves are empty, and the single fusion carries exactly \
             one outer and one inner layer"
        );
    }

    /// **`D3` — the consuming callee's binder must RESOLVE to the producer body,
    /// and neither marginal binder fact establishes that.**
    ///
    /// Ruled at `evt_2rw6vhq8xrqcm`: `BinderAgreement` proves the key's
    /// consuming binder sits at the admitted frame and recursive position, and
    /// that the admitted result root equals the invocation callee entry. It does
    /// not prove the relation BETWEEN them — that the hypothesis that binder
    /// names invokes that body — which is the fact `D3`'s definition-local fused
    /// self edge is emitted against.
    ///
    /// ## Why each row is paired with a SUPPRESSED twin
    ///
    /// A row asserting only "the perturbed key refused" cannot distinguish this
    /// rule from an earlier proxy: a perturbation that also trips
    /// `BinderAgreement` refuses either way, and the classified cause would then
    /// be reporting which rule ran FIRST rather than which rule was needed.
    /// Suppressing this one rule and observing the SAME key **issue a claim** is
    /// what proves the four marginal checks — frame identity, recursive
    /// position, admitted result root, and the redirect triple — all stayed
    /// green under that perturbation. The pairs are the control; the refusals
    /// alone are not.
    ///
    /// **MEASURED** on the checked applied `Exact` witness. Moving the admitted
    /// frame and the consuming binder's frame TOGETHER to the producer's own
    /// `ComputationalMatch` keeps every marginal check satisfied — the two
    /// frames still agree with each other, the positions are untouched, and the
    /// result root and redirect are untouched — and the binder then resolves to
    /// body 34, the producer's OWN outgoing edge, rather than to the redirected
    /// body 37. Independently, pointing `consuming_callee` at the producer's
    /// hypothesis occurrence 29, whose binder is a real binding and a DIFFERENT
    /// one, refuses at the same rule while suppressing it issues.
    /// **CLAIMED:** no fused region is claimed whose consuming callee is not the
    /// binder its key names, or whose binder's hypothesis invokes a body other
    /// than the one the claim redirects into.
    /// **THE GAP:** this pins **preflight** and the planner's binding authority.
    /// It pins no emission — no self edge is built here — and the resolution's
    /// closure step is measured on the three `D2j` causes that install a key,
    /// so a constructor argument outside that population is refused rather than
    /// shown correct.
    #[test]
    fn d3_the_consuming_binder_must_resolve_to_the_redirected_producer_body() {
        // The lawful witness still issues WITH the rule armed. Without this the
        // refusals below would be equally consistent with a rule that refuses
        // everything -- which is the shape that already cost this node one
        // guard.
        let lawful = d2f_refusal_of(d2f_preflight_exact(|_| ()));

        // The EXACT-CALLEE half: the key keeps its own binder but points
        // `consuming_callee` at the producer's hypothesis occurrence, whose
        // binding is real and DIFFERENT. So this fails because the binders
        // disagree, not because one is absent.
        let moved_callee = |keys: &mut Vec<StaticContinuationFusionKey>| {
            keys[0].consuming_callee = keys[0].producer_argument_origin;
        };

        // The BODY half, and it has to be a COHERENT relabel to reach the
        // resolution at all.
        //
        // ⇒ **Moving the binder's frame alone does NOT test this.** That was
        // the first shape written here, and a mutation proof killed it: with
        // only the frame moved, `consuming_callee` no longer matches its own
        // binding, so the callee half above answers first and the resolution
        // never runs. The comparison it was meant to exercise could be replaced
        // by a tautology with the row still green.
        //
        // So all three members move together onto the PRODUCER's hypothesis:
        // the callee occurrence, its true binding, and the admitted frame that
        // binding must agree with. The key is now internally consistent and
        // still wrong -- it names a hypothesis that invokes body 34, the
        // producer's own outgoing edge, while claiming to redirect body 37.
        let relabelled_binder = |keys: &mut Vec<StaticContinuationFusionKey>| {
            keys[0].consuming_callee = keys[0].producer_argument_origin;
            keys[0].consumer_binding = keys[0].producer_argument_binding;
            keys[0].admitted.continuation_origin =
                keys[0].producer_argument_binding.frame_origin;
        };

        let rows = vec![
            ("lawful witness, rule armed", lawful),
            (
                "consuming callee moved, rule armed",
                d2f_refusal_of(d2f_preflight_exact(moved_callee)),
            ),
            (
                "consuming callee moved, rule suppressed",
                d2f_refusal_of(d2f_preflight_exact_without_resolution(moved_callee)),
            ),
            (
                "binder relabelled to the producer's, rule armed",
                d2f_refusal_of(d2f_preflight_exact(relabelled_binder)),
            ),
            (
                "binder relabelled to the producer's, rule suppressed",
                d2f_refusal_of(d2f_preflight_exact_without_resolution(relabelled_binder)),
            ),
        ];

        assert_eq!(
            rows,
            vec![
                ("lawful witness, rule armed", "issued".to_string()),
                (
                    "consuming callee moved, rule armed",
                    "BinderBodyResolution".to_string()
                ),
                (
                    "consuming callee moved, rule suppressed",
                    "issued".to_string()
                ),
                (
                    "binder relabelled to the producer's, rule armed",
                    "BinderBodyResolution".to_string()
                ),
                (
                    "binder relabelled to the producer's, rule suppressed",
                    "issued".to_string()
                ),
            ],
            "each perturbation refuses AT the binder-to-body relation, and the same key issues \
             once that one rule is suppressed -- so the four marginal checks stayed green and \
             the refusal is not an earlier proxy answering for them"
        );
    }

    /// `D2e` `AC-2`/`AC-3` — indirection does not lose the role, and an ordinary
    /// binder beside it does not acquire one.
    ///
    /// The three hypothesis references sit at de Bruijn indices 0, 1 and 2 and
    /// all resolve to the SAME frame origin and recursive position. A classifier
    /// keyed on depth or on index would answer differently at each; threading
    /// answers the same, which is the property.
    ///
    /// `Var(3)` is the discriminator: it is an ordinary constructor child in the
    /// same scope as the third hypothesis reference, so a classifier that
    /// answered "hypothesis" for everything in a recursive case fails here.
    #[test]
    fn d2e_ih_binding_survives_let_and_nested_match_indirection() {
        let expr = d2e_indirection_fixture();
        let plan = b2r_plan(&expr);
        let bindings = build_checked_ih_bindings(&plan).expect("bindings derive");

        let root = plan.root_occurrence.expect("the fixture has a root occurrence");
        let case_body = plan.semantic.child_origin(root, 1).expect("Node case body");
        let let_value = plan.semantic.child_origin(case_body, 0).expect("Let value");
        let inner_match = plan.semantic.child_origin(case_body, 1).expect("Let body");
        let match_scrutinee = plan
            .semantic
            .child_origin(inner_match, 0)
            .expect("Match scrutinee");
        let pair = plan
            .semantic
            .child_origin(inner_match, 1)
            .expect("Wrap case body");
        let pair_hypothesis = plan.semantic.child_origin(pair, 0).expect("Pair arg 0");
        let pair_child = plan.semantic.child_origin(pair, 1).expect("Pair arg 1");

        let expected = CheckedIhBinding {
            frame_origin: root,
            recursive_position: 0,
        };
        assert_eq!(
            bindings.get(&let_value).copied(),
            Some(expected),
            "the direct reference resolves to the frame's hypothesis"
        );
        assert_eq!(
            bindings.get(&match_scrutinee).copied(),
            Some(expected),
            "a Let between the binder and the use shifts the index, not the role"
        );
        assert_eq!(
            bindings.get(&pair_hypothesis).copied(),
            Some(expected),
            "a Let AND a nested Match case binder still leave the role intact"
        );
        assert_eq!(
            bindings.get(&pair_child).copied(),
            None,
            "the ordinary constructor child beside it acquires no role, so the \
             classifier is not answering `hypothesis` for everything in scope"
        );
    }

    /// `D2e` `AC-3` — a closure body does not inherit the enclosing hypothesis.
    ///
    /// The lowering gives a closure body a FRESH environment, so a `Var(0)`
    /// inside one is that closure's own parameter and not the hypothesis that
    /// happens to be `Var(0)` outside it. This is the case a walk that extended
    /// the outer environment would get wrong, and it is silent -- the classifier
    /// would simply claim a hypothesis that is not there.
    #[test]
    fn d2e_ih_binding_does_not_leak_into_a_closure_body() {
        let expr = d2e_indirection_fixture();
        let plan = b2r_plan(&expr);
        let bindings = build_checked_ih_bindings(&plan).expect("bindings derive");
        let root = plan.root_occurrence.expect("the fixture has a root occurrence");
        let scrutinee = plan.semantic.child_origin(root, 0).expect("scrutinee");
        let closure = plan
            .semantic
            .child_origin(scrutinee, 0)
            .expect("the constructor's closure argument");
        let closure_body = plan
            .semantic
            .child_origin(closure, 0)
            .expect("the closure body");
        assert_eq!(
            bindings.get(&closure_body).copied(),
            None,
            "nothing inside a closure body resolves to the enclosing hypothesis"
        );
        assert!(
            !bindings.is_empty(),
            "and the sweep is not vacuously empty -- the fixture does produce bindings"
        );
    }

    /// `D2e` `AC-1` — the role DISCRIMINATES, in both directions on one shape.
    ///
    /// A classifier that answered `InductionHypothesis` for everything would
    /// pass an IH-only assertion, and one that answered `ConstructorChild` for
    /// everything would pass a child-only assertion. So the same case supplies
    /// both, at adjacent de Bruijn indices, and each is named exactly.
    ///
    /// The case declares TWO binders of which ONE is recursive, so the two
    /// roles are genuinely a pair drawn from one declaration rather than from
    /// two fixtures whose difference could be anything.
    #[test]
    fn d2e_binder_role_separates_a_hypothesis_from_an_ordinary_child() {
        let case = RuntimeComputationalMatchCase {
            constructor: "ctor:fixture::D2eShape::Node".to_string(),
            argument_binders: 2,
            recursive_positions: vec![1],
            body: RuntimeExpr::Var(0),
        };
        let layout = CheckedCaseBinderLayout::for_case(&case).expect("the layout derives");

        assert_eq!(
            layout.role_at(0),
            CheckedCaseBinderRole::InductionHypothesis {
                recursive_position: 1
            },
            "the leading slot is the hypothesis, and it names the RECURSIVE POSITION \\
             the case declared -- not its own de Bruijn index"
        );
        assert_eq!(
            layout.role_at(1),
            CheckedCaseBinderRole::ConstructorChild { field_position: 0 },
            "the binder immediately after the hypothesis prefix is an ordinary child"
        );
        assert_eq!(
            layout.role_at(2),
            CheckedCaseBinderRole::ConstructorChild { field_position: 1 }
        );
    }

    /// `D2e` — the hypothesis prefix runs in REVERSE declaration order, and this
    /// is the half a single-recursive-position witness cannot see.
    ///
    /// **MEASURED, not assumed.** At the specialized composed arm, a
    /// `PX8JSiblingTree::Node` case with `recursive_positions=[0, 1]` put the
    /// hypothesis for `sibling_position=1` at `Var(0)` and the one for
    /// `sibling_position=0` at `Var(1)`.
    ///
    /// **This test is the whole reason the reversal is not a guess.** Forward
    /// and reversed coincide at length one, so every one-position fixture in the
    /// corpus agrees with BOTH spellings; only a case with two positions can
    /// tell them apart. Deleting this test would leave the order pinned by
    /// nothing.
    #[test]
    fn d2e_binder_role_hypothesis_prefix_is_reverse_declaration_order() {
        let case = RuntimeComputationalMatchCase {
            constructor: "ctor:fixture::D2eShape::Pair".to_string(),
            argument_binders: 2,
            recursive_positions: vec![0, 1],
            body: RuntimeExpr::Var(0),
        };
        let layout = CheckedCaseBinderLayout::for_case(&case).expect("the layout derives");

        assert_eq!(
            layout.role_at(0),
            CheckedCaseBinderRole::InductionHypothesis {
                recursive_position: 1
            },
            "Var(0) carries the LAST declared recursive position"
        );
        assert_eq!(
            layout.role_at(1),
            CheckedCaseBinderRole::InductionHypothesis {
                recursive_position: 0
            },
            "Var(1) carries the FIRST declared recursive position"
        );
        assert_ne!(
            layout.role_at(0),
            layout.role_at(1),
            "the two hypotheses must not collapse to one role, or the reversal is unobservable"
        );
    }

    /// `D2e` — the run ENDS, and what follows is the enclosing frame.
    ///
    /// Total rather than fallible: a `Var` reaching past this case's binders
    /// is ordinary, and refusing it here would turn every outer-scope reference
    /// into a planner error.
    #[test]
    fn d2e_binder_role_past_the_case_run_is_the_frame_environment() {
        let case = RuntimeComputationalMatchCase {
            constructor: "ctor:fixture::D2eShape::Node".to_string(),
            argument_binders: 1,
            recursive_positions: vec![0],
            body: RuntimeExpr::Var(0),
        };
        let layout = CheckedCaseBinderLayout::for_case(&case).expect("the layout derives");
        assert_eq!(layout.binder_count(), 2);
        assert_eq!(
            layout.role_at(2),
            CheckedCaseBinderRole::FrameEnvironment,
            "the first index past the run belongs to the enclosing frame"
        );
        assert_eq!(
            layout.role_at(97),
            CheckedCaseBinderRole::FrameEnvironment,
            "and so does every index beyond it"
        );
    }

    /// `D2e` — a case with NO recursive positions has no hypothesis slot at all.
    ///
    /// The population that must not acquire a role by accident: every
    /// non-recursive case in the corpus. If the prefix were computed from a
    /// count that could be nonzero here, `Var(0)` would misclassify.
    #[test]
    fn d2e_binder_role_a_nonrecursive_case_has_no_hypothesis_slot() {
        let case = RuntimeComputationalMatchCase {
            constructor: "ctor:fixture::D2eShape::Leaf".to_string(),
            argument_binders: 1,
            recursive_positions: Vec::new(),
            body: RuntimeExpr::Var(0),
        };
        let layout = CheckedCaseBinderLayout::for_case(&case).expect("the layout derives");
        assert_eq!(
            layout.role_at(0),
            CheckedCaseBinderRole::ConstructorChild { field_position: 0 },
            "with nothing recursive declared, the leading binder is the child itself"
        );
        assert_eq!(layout.role_at(1), CheckedCaseBinderRole::FrameEnvironment);
    }

    #[test]
    fn governed_nested_bracket_uses_canonical_four_seat_binder_roles() {
        for depth in 3..=7 {
            assert_governed_bracket_shape(&nested_resource_bracket(depth), depth);
        }
    }
}
