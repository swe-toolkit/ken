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
