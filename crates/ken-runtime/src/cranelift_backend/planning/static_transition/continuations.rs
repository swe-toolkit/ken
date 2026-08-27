//! Continuation keys, continuation-seat construction, the evidence surfaces
//! keyed on them, and the static-continuation fusion identity plane.
//!
//! `RT-PLANNER-CONTINUATIONS-SPLIT` `D1` — this module owns the continuation
//! domain moved from the parent (`ContinuationSpecializationId`,
//! `ContinuationEmissionOwner`, the seat/environment/availability records, the
//! checked-binder/checked-transport surface, and
//! `StaticContinuationFusion*`/`Fusion*`). `StaticTransitionPlan` stays in the
//! parent; the impls here read ancestor-private root state under the standing
//! child-module pattern (item 4's `units.rs` precedent).

use std::collections::{BTreeMap, BTreeSet};
#[cfg(test)]
use std::cell::Cell;

use super::abi;
#[cfg(feature = "px8-ds-test-support")]
use super::aggregates::checked_ih_generated_entry_context_permutation_is_active;
use super::abi::{
    AbiCaptureProvenance, AbiCarrier, AbiFrameHeader, AbiOwnership, AbiSlot, AbiSlotKind,
    AbiStorageOwner, AbiUnitDefinition,
};
use super::occurrences::{occurrence_authority, origin_of, StaticOriginId};
use super::semantic_ir::{RuntimeExprShape, SemanticSourceKind};
use super::units::{EmittableCallEdge, EmittableCallKind};
use super::{
    CaseEmissionStatus, ConstructorIdentity, CraneliftBackendError, EdgeKind,
    PlannedReferentLifetime, PredeclaredFunctionId, RuntimeDeclaration, StaticTransitionPlan,
    dense_slice, lifetime_referent_affinity, planner_capacity_error, planner_error,
};
use crate::boundary_value::BoundaryReferentOwner;
#[cfg(test)]
use super::closure::{D4B_ADMISSION, D4B_ADMISSION_ARMED, D4bVerdict};
use crate::RuntimeExpr;

mod fusion;

pub(in crate::cranelift_backend) use fusion::{
    FusionClaimRefusal, FusionComposedEdge, FusionCompositionLayer, FusionOwnedBody,
    FusionOwnedOuterRealization, FusionRegionClaim, FusionRegionClaimLedger,
    StaticContinuationFusionCandidate, StaticContinuationFusionDescriptor,
    StaticContinuationFusionId, StaticContinuationFusionKey, StaticContinuationFusionPlan,
    StaticContinuationFusionView, build_static_continuation_fusion_plan, fusion_redirect_target,
};
#[cfg(test)]
pub(in crate::cranelift_backend) use fusion::{FusionClaimParameterMutation, FusionProducerCaptureMutation};
#[cfg(test)]
pub(in crate::cranelift_backend) use fusion::{
    r3_fusion_claim_consumptions, reset_r3_fusion_claim_consumptions,
    set_primary_fusion_key_derivation_mutated, with_fusion_claim_parameter_mutation,
    with_fusion_producer_capture_mutation,
};

/// Dense identity of one planner-interned continuation specialization.
///
/// The identity is compiler-only. It is assigned from the full immutable key;
/// capture values and runtime selectors never participate.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub(in crate::cranelift_backend) struct ContinuationSpecializationId(pub(super) u32);

/// **`RT-DECL-CLOSURE-PORT` `D5a` — the generalized emission-owner domain.**
///
/// Architect ruling `evt_609am4v7cdt5b`. The planner had been conflating three
/// authorities, and the one that decides *who can emit a causal call* is
/// neither of the two it was reading:
///
/// | authority | for the measured witness |
/// |---|---|
/// | source-occurrence provenance owner | raw `fn2` — the nested producer is textually in body 36 |
/// | root input provenance owner | `fn3` — its parameters populate the continuation environment |
/// | **immediate emission and availability owner** | the interned specialization that selected and invoked `fn2` |
///
/// ⛔ **The two variants are distinct ID domains and are never cast or aliased
/// into one another.** That is the whole point of making this an enum rather
/// than widening `PredeclaredFunctionId`: a specialization context is not a
/// predeclared unit, and code that treats one as the other reintroduces exactly
/// the conflation this type exists to remove.
///
/// ⚠ `Predeclared` is the *only* variant the population had before `D5a`, and
/// every same-owner edge still lands there with identical behaviour. A
/// `Specialization` owner appears only where the fixed point descended into a
/// worker body from an interned specialization.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum ContinuationEmissionOwner {
    /// An ordinary predeclared function unit emits this call from its own body.
    Predeclared(PredeclaredFunctionId),
    /// The generated execution context of an interned specialization emits this
    /// call. The raw body's predeclared owner remains **provenance only**.
    Specialization(ContinuationSpecializationId),
    /// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2f`.** A planner-interned static
    /// continuation fusion emits this call from the generated region it owns.
    ///
    /// The fused region is a **third thing**: not the original producer's owner
    /// and not the consumer's. `D2d`'s grounding record measured those two, and
    /// naming either of them here would make the generated region's emissions
    /// indistinguishable from the source units it was fused out of.
    ///
    /// The domain rule above binds this variant too: a
    /// [`StaticContinuationFusionId`] is never cast into, or aliased with, a
    /// `PredeclaredFunctionId` or a `ContinuationSpecializationId`. The three
    /// interners are independent and each may lawfully issue local id `0`, so a
    /// bare numeric comparison across them is not an identity test.
    Fusion(StaticContinuationFusionId),
}

/// Dense identity of one planner-interned generated producer execution context.
///
/// ⛔ **A third ID domain, and it is never cast into either of the other two.**
/// A context is not a `PredeclaredFunctionId` (it has no source occurrence of
/// its own) and it is not a `ContinuationSpecializationId` (it is not a
/// continuation callee — it is the *caller* side, the execution in which one
/// specialization's selected worker body runs with that specialization's
/// continuation inputs still live).
///
/// ⚠ [`ContinuationEmissionOwner`] deliberately does **not** gain a variant for
/// **this** class. `Specialization(id)` already names the emitting context
/// uniquely: a specialization has exactly one selected worker body, so
/// `(specialization, worker_body_origin)` — the context key — is determined by
/// the specialization alone. An owner variant for a context would give the same
/// context two names, and the claim ledger's affinity is keyed on the owner.
///
/// ⚠ **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` — read the paragraph above as
/// scoped to `ContinuationContextId`, which is all it ever established.**
/// [`ContinuationEmissionOwner`] now carries a third variant,
/// [`ContinuationEmissionOwner::Fusion`], and that is not in tension with it:
/// the argument here is *"a context is already determined by its
/// specialization"*, and a static continuation fusion is not. Its identity is
/// the complete [`StaticContinuationFusionKey`], no member of which is derivable
/// from a specialization — the fused region is neither the producer's owner nor
/// the consumer's, so no existing variant names it.
///
/// **The stated consequence survives the correction and is real work.** Ledger
/// affinity keys on the owner, so a third variant widens what that key ranges
/// over: every affinity comparison must be over the whole
/// [`ContinuationEmissionOwner`] value, never over an extracted inner id, since
/// the four id domains are independent interners that each issue local `0`.
/// Comparing owners as enum values is what keeps affinity total across the
/// widened domain; comparing inner integers would silently identify a fusion
/// with a predeclared unit.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub(in crate::cranelift_backend) struct ContinuationContextId(pub(super) u32);

impl ContinuationContextId {
    /// A DIFFERENT context identity. **Mutation support only** — used to present
    /// a claimed id that disagrees with the one the claim's own key resolves to,
    /// so the consumer's agreement check has something to catch.
    ///
    /// ⛔ Not a way to construct an identity from an integer: it can only
    /// displace an id that already exists, so no test can mint one out of thin
    /// air and have it read as planner-issued.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn d4b_displaced(self) -> Self {
        Self(self.0.wrapping_add(1))
    }
}

/// **`RT-DECL-CLOSURE-PORT` `D5a` — one planner-interned generated producer
/// execution context.**
///
/// Architect ruling `evt_609am4v7cdt5b`: *"Materialize this as an explicit
/// planner-interned, continuation-specialized producer execution context."*
///
/// ## Where these come from, and why not at descent
///
/// Derived **after** the fixed point, from the calls whose `emission_owner` is a
/// `Specialization`. ⛔ Interning at descent instead would mint a context for
/// every worker body the fixed point walks into, including the ones whose bodies
/// contain no nested producer at all — dead definitions the emitter would then
/// have to declare and define. The post-hoc derivation mints exactly the
/// contexts something actually emits from.
///
/// ## Its ABI, and what it must not do to the raw body
///
/// - **Parameters** = the raw worker's declared arity plus its capture count, in
///   that order. That is exactly the operand run the enclosing specialization
///   already passes when it calls the worker, so the call **keeps its shape**
///   and only grows a suffix.
/// - **Captures** = the enclosing specialization's continuation inputs, in
///   ordinal order. These are the values raw `fn2` provably never receives.
///
/// ⛔ The raw `ClosureBody`'s own descriptor is **untouched** — not mutated, not
/// unioned, and no runtime suffix is fused into it. The raw unit keeps its
/// ordinary ABI, its provenance, and its authority as this body's source
/// binding.
///
/// ⚠ **That is a claim about the descriptor, and it settles nothing about
/// whether an executable `Function` is emitted for the raw worker.** Those are
/// separate questions with separate authorities, and this comment used to
/// conflate them by adding *"and simply loses this one caller"*.
///
/// Executable membership is decided from the **post-retarget final graph**, by
/// [`StaticTransitionPlan::template_only_worker_bodies`]: when **every**
/// specialization selecting a body has retargeted to a generated context, and
/// that body's carried invocation also binds one, the raw worker is
/// **template-only** — it retains everything above and is absent from the
/// emitted-`Function` population. If any final raw call remains, the body
/// stays executable. ⛔ So "loses one caller" describes only the mixed case;
/// under a total retarget the raw worker loses its last one.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct PlannedContinuationContext {
    pub(super) id: ContinuationContextId,
    /// **Stage 2** — the finalized availability of each capture, in capture
    /// order. Empty until [`finalize_continuation_availability_plan`] runs; the
    /// only reader is [`ContinuationContextView::captures`], which refuses when
    /// an entry is missing rather than publishing a draft.
    pub(super) finalized_availability: Vec<ContinuationAvailabilityViews>,
    /// The specialization whose selected worker body this context executes, and
    /// whose continuation inputs it keeps live across that execution.
    pub(super) enclosing_specialization: ContinuationSpecializationId,
    /// The raw worker body lowered inside this context.
    pub(super) worker_body_origin: StaticOriginId,
    /// ⚠ **PROVENANCE ONLY.** The predeclared unit the worker body textually
    /// belongs to. Retained so a reader can see which raw body this context
    /// executes; it confers no emission authority — that is the whole point.
    pub(super) raw_owner: PredeclaredFunctionId,
    /// Raw declared arity + raw capture count: the context's `Parameter` run.
    pub(super) parameters: u32,
    /// The enclosing specialization's continuation inputs: the `Capture` run.
    pub(super) captures: Vec<ContinuationInputProjection>,
}

impl ContinuationContextId {
    pub(super) fn from_position(position: usize) -> Result<Self, CraneliftBackendError> {
        Ok(Self(u32::try_from(position).map_err(|_| {
            planner_capacity_error("generated context identity exhausted")
        })?))
    }
}

impl PlannedContinuationContext {
    pub(in crate::cranelift_backend) fn id(&self) -> ContinuationContextId {
        self.id
    }
    /// The specialization whose worker body this context executes.
    ///
    /// ⭐ This is also the **emission owner** every call emitted from inside
    /// this context carries: `ContinuationEmissionOwner::Specialization(_)` of
    /// exactly this id. The two are the same fact seen from the two sides, which
    /// is why the owner enum needs no context variant.
    pub(in crate::cranelift_backend) fn enclosing_specialization(
        &self,
    ) -> ContinuationSpecializationId {
        self.enclosing_specialization
    }
    pub(in crate::cranelift_backend) fn worker_body_origin(&self) -> StaticOriginId {
        self.worker_body_origin
    }
    /// ⚠ Provenance only — the raw predeclared unit this body textually is.
    pub(in crate::cranelift_backend) fn raw_owner(&self) -> PredeclaredFunctionId {
        self.raw_owner
    }
    pub(in crate::cranelift_backend) fn parameters(&self) -> u32 {
        self.parameters
    }
    pub(super) fn captures(&self) -> &[ContinuationInputProjection] {
        &self.captures
    }
}

/// A read-only view of one already-validated generated producer execution
/// context: its identity, the specialization it belongs to, and its validated
/// ABI descriptor, slots and input authority.
pub(in crate::cranelift_backend) struct ContinuationContextView<'plan> {
    pub(super) planned: &'plan PlannedContinuationContext,
    /// Stage-2 availability, one per capture. ⛔ The view cannot reach the
    /// drafts on `planned` -- `captures()` publishes from here or refuses.
    pub(super) finalized: &'plan [ContinuationAvailabilityViews],
    pub(super) header: AbiFrameHeader,
    pub(super) slots: &'plan [AbiSlot],
    pub(super) inputs: &'plan [abi::AbiContinuationInputAuthority],
}

impl<'plan> ContinuationContextView<'plan> {
    pub(in crate::cranelift_backend) fn id(&self) -> ContinuationContextId {
        self.planned.id
    }
    pub(in crate::cranelift_backend) fn enclosing_specialization(
        &self,
    ) -> ContinuationSpecializationId {
        self.planned.enclosing_specialization
    }
    pub(in crate::cranelift_backend) fn worker_body_origin(&self) -> StaticOriginId {
        self.planned.worker_body_origin
    }
    pub(in crate::cranelift_backend) fn raw_owner(&self) -> PredeclaredFunctionId {
        self.planned.raw_owner
    }
    /// `D3b` — this context's declared parameter count, so a consumer can check
    /// a capture slot against the run it actually names. ⛔ A read of existing
    /// authority: the capture run begins after the parameters, and that offset
    /// is the planner's rather than this accessor's.
    pub(in crate::cranelift_backend) fn parameters(&self) -> u32 {
        self.planned.parameters
    }
    pub(in crate::cranelift_backend) fn header(&self) -> AbiFrameHeader {
        self.header
    }
    pub(in crate::cranelift_backend) fn slots(&self) -> &'plan [AbiSlot] {
        self.slots
    }

    /// This context's own capture run, as continuation-input views.
    ///
    /// Each view's `source_owner`/`source_abi_position` is **root** provenance
    /// naming the enclosing specialization's own root owner, and its
    /// `immediate_slot` is that value's position in the *enclosing* environment.
    /// ⛔ Neither is this context's own slot position — for that, use the
    /// capture's `ordinal` against this descriptor's `Capture` run. Keeping the
    /// two apart is the whole reason the record carries both.
    pub(in crate::cranelift_backend) fn captures(
        &self,
    ) -> Result<Vec<ContinuationInputView>, CraneliftBackendError> {
        self.planned
            .captures
            .iter()
            .zip(self.inputs)
            .enumerate()
            .map(|(position, (projection, authority))| {
                // `D3a` — the ABI-plane input authority records a DOMAIN-TAGGED
                // provenance owner, so agreement is checked as the complete
                // tagged value. ⛔ Both domains are now recordable here, and
                // neither can satisfy the other's comparison by carrying the
                // same owner id.
                if projection.ordinal != authority.ordinal
                    || abi::AbiContinuationInputProvenance::of(projection.coordinate)
                        != authority.provenance
                {
                    return Err(planner_error(
                        "a generated context capture disagrees with its validated ABI input \
                         authority",
                    ));
                }
                continuation_input_view(projection, self.finalized.get(position))
            })
            .collect()
    }

    /// Byte offsets for this context's slot run, from the one offset walk.
    pub(in crate::cranelift_backend) fn slot_offsets(
        &self,
    ) -> Result<(Vec<u32>, u32), CraneliftBackendError> {
        let (offsets, frame_bytes) = abi::slot_offsets(self.slots)?;
        if frame_bytes != self.header.frame_bytes {
            return Err(planner_error(
                "a generated context descriptor's frame size disagrees with its own slot run",
            ));
        }
        Ok((offsets, frame_bytes))
    }
}

/// The source ABI provenance class of one continuation input.
///
/// ⛔ This labels provenance **inside one coordinate space** — a position in the
/// source owner's entry ABI input run. It is not the place to name a value that
/// has no such position; see [`ContinuationSourceCoordinate`].
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum ContinuationInputSource {
    Parameter,
    LexicalCapture { source_origin: StaticOriginId },
    SeedCapture { defining_origin: StaticOriginId },
}

/// `RT-CONTSRC-PRODUCER-LOCAL` `D1` — the exact structural identity of a value
/// the producer **creates mid-body**.
///
/// ⛔ Deliberately carries no ABI position and is not convertible to one. The
/// value does not exist at its owner's function entry, so `parameters +
/// captures` has no position for it, and inventing one is the first of the five
/// exits the Architect closed at `evt_75k8cydbj5127`.
///
/// The identity is the occurrence that introduces the binding **plus which
/// binding of that occurrence it is**: an occurrence such as a `Match` case
/// introduces several at once, and an origin alone would name the set rather
/// than the value.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct ProducerLocalBinding {
    /// The unit whose body contains the binding.
    pub(in crate::cranelift_backend) binding_owner: PredeclaredFunctionId,
    /// The occurrence that introduces it.
    pub(in crate::cranelift_backend) binding_origin: StaticOriginId,
    /// Which binding that occurrence introduces. Zero when it introduces one.
    pub(in crate::cranelift_backend) binding_ordinal: u32,
}

/// `RT-CONTSRC-PRODUCER-LOCAL` `D1` — where a producer-local value is found at
/// the moment the continuation call is emitted.
///
/// ⛔ Deliberately its own type rather than a second `u32` sitting beside an
/// entry ABI position. An environment index and an ABI position are different
/// coordinate spaces; the whole reason [`ContinuationSourceCoordinate`] is a
/// closed sum is that no consumer can read one as the other by forgetting to
/// ask which it holds.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct ProducerLocalLocator {
    /// The occurrence whose value environment contains the binding at the point
    /// the call is emitted.
    pub(in crate::cranelift_backend) environment_origin: StaticOriginId,
    /// The binding's index in that environment.
    pub(in crate::cranelift_backend) environment_index: u32,
}

/// `RT-CONTSRC-PRODUCER-LOCAL` `D1` — which coordinate **domain** names one
/// continuation input's value in its producer.
///
/// ⛔ A closed sum over two coordinate *spaces*, and ⛔ **not** a fourth
/// [`ContinuationInputSource`] arm. The Architect rejected that shape
/// explicitly (`evt_75k8cydbj5127`): appending a case to an enum whose
/// enclosing record still requires an entry-ABI coordinate produces a truthful
/// provenance label with an untruthful `source_abi_position` beside it.
///
/// ⛔ **No default or wildcard arm anywhere this is matched.** A third domain
/// must fail to compile at every consumer until each one assigns it — that is
/// `AC-2`, and it is the property that makes this a type rather than a
/// convention.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum ContinuationSourceCoordinate {
    /// A value that exists at the source owner's function entry: one position
    /// in its `parameters + captures` ABI input run, carrying the contract that
    /// `continuation_owner_entry_sources` reads off that exact `AbiSlot`.
    ///
    /// ⭐ Unchanged from before `D1` in both content and meaning. The three
    /// components were previously inline fields of the records below; moving
    /// them into an arm is what makes the other domain expressible without
    /// making this one ambiguous.
    EntryAbi {
        source_owner: PredeclaredFunctionId,
        source_abi_position: u32,
        source: ContinuationInputSource,
    },
    /// A value created after entry by the producer body. Named by its exact
    /// structural binding identity and located, at emission time, in the
    /// environment that actually contains it. Its carrier / ownership /
    /// storage-owner / affinity are planner-derived and live on the enclosing
    /// record, exactly as the entry arm's slot-derived ones do.
    ///
    /// ⭐ `D4a` removed this arm's `dead_code` allowance. `D1` represented it,
    /// `D2` populated it in the walk, and `D4a` admits it — so it is now
    /// constructed and read on the ordinary planning path, not only by the
    /// controls that prove each consumer handles it.
    ProducerLocal {
        binding: ProducerLocalBinding,
        locator: ProducerLocalLocator,
    },
}

impl ContinuationSourceCoordinate {

    /// A producer-local coordinate for a control that must **reach** one.
    ///
    /// ⛔ Test-only, and it exists because `D1` represents this domain while
    /// nothing constructs it yet: without a way to present one, every refusal
    /// written for it is unmeasured code, which reads exactly like absent code.
    /// The identifiers are sentinels — the controls assert on the refusal, not
    /// on what the binding names.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn producer_local_probe() -> Self {
        Self::ProducerLocal {
            binding: ProducerLocalBinding {
                binding_owner: PredeclaredFunctionId(u32::MAX),
                binding_origin: StaticOriginId(u32::MAX),
                binding_ordinal: 0,
            },
            locator: ProducerLocalLocator {
                environment_origin: StaticOriginId(u32::MAX),
                environment_index: 0,
            },
        }
    }

    /// The entry-ABI components, for a test that is asserting *about* them.
    ///
    /// ⛔ Test-only, and it panics rather than defaulting: a test that meant to
    /// read an entry position and got a producer-local coordinate has measured
    /// a different value than it names, which is the failure mode this whole
    /// separation exists to prevent.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn expect_entry_abi(
        self,
    ) -> (PredeclaredFunctionId, u32, ContinuationInputSource) {
        match self {
            Self::EntryAbi {
                source_owner,
                source_abi_position,
                source,
            } => (source_owner, source_abi_position, source),
            Self::ProducerLocal { binding, locator } => panic!(
                "this assertion names an entry ABI coordinate but reached the producer-local \
                 binding {binding:?} at {locator:?}"
            ),
        }
    }
}

/// One exact ordered input projection into a dormant continuation unit.
///
/// Every field is static planner provenance. In particular, the value carried
/// by the source slot is deliberately absent from this type and from the key
/// that owns it.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct ContinuationInputProjection {
    pub(super) producer_owner: PredeclaredFunctionId,
    pub(super) consumer_owner: PredeclaredFunctionId,
    /// `D1` — the closed coordinate domain naming this value in its producer.
    /// ⛔ Replaces the inline `source_owner`/`source_abi_position`/`source`
    /// triple; those are the `EntryAbi` arm's components and are unchanged
    /// there.
    pub(super) coordinate: ContinuationSourceCoordinate,
    pub(super) ordinal: u32,
    pub(super) carrier: AbiCarrier,
    pub(super) ownership: AbiOwnership,
    pub(super) storage_owner: AbiStorageOwner,
    pub(super) referent_affinity: Vec<BoundaryReferentOwner>,
    pub(super) ordinary_abi_position: u32,
    /// **`D5a` — where this value is IMMEDIATELY available to the emitter.**
    ///
    /// `source_owner`/`source_abi_position` above are **root provenance**: the
    /// predeclared unit whose parameter or capture originally supplied this
    /// value. That is retained and never substituted for. But the function that
    /// actually emits the call is the key's `emission_owner`, and for a
    /// generated specialization context that is **not** the root owner — raw
    /// `fn2` never receives `fn3`'s parameters.
    ///
    /// This field is the position in the **emitting context's own environment**
    /// where the value sits. For a `Predeclared` emission owner the emitter *is*
    /// the root owner, so it equals `source_abi_position` and that equality is
    /// checked at the emission seam rather than assumed. For a `Specialization`
    /// emission owner it names one of the generated context's own capture
    /// positions.
    ///
    /// ⛔ A root position used as an immediate one is exactly the reverse-map
    /// `evt_609am4v7cdt5b` forbids; the two are kept apart here so no consumer
    /// has to know which it holds.
    ///
    /// **`D3b` re-cut** — the two consumer-specific claims, as STAGE 1 drafts.
    ///
    /// ⛔ There is no single "immediate slot" here, because the two consumers do
    /// not hold the same environment. ⛔ And these are **drafts**: a generated
    /// frame is still a structural requirement, because the context ids that
    /// would resolve it are minted after this record is interned. Nothing reads
    /// this field directly — [`continuation_input_view`] publishes only the
    /// finalized form.
    pub(super) availability: ContinuationAvailabilityDraft,
}

impl ContinuationAvailabilityViews {
    /// The direct-emission claim's index, for a test that has already
    /// established the claim exists. ⛔ Test-only and panicking on purpose:
    /// production consumers must go through the fail-closed resolvers, never
    /// assert their way past the "which environment" question.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn expect_direct_emission_slot(self) -> u32 {
        match self.direct_emission {
            Some(ContinuationEnvironmentClaim::CurrentLexical {
                nearest_alias_index, ..
            }) => nearest_alias_index,
            Some(ContinuationEnvironmentClaim::EntryFrame { declared_slot, .. }) => declared_slot,
            None => panic!("expected a direct-emission availability claim, found none"),
        }
    }
}

/// **`D3b` re-cut — which frame will emit this continuation call.**
///
/// ⛔ Deliberately an enum over the two emitting-frame classes rather than an
/// `Option` with a defaulting arm: a generated context that declares no member
/// for a value must **reject**, and a default would silently emit a call reading
/// whatever sat at the root position.
///
/// ⭐ This replaces `ContinuationImmediateResolution`, whose `RootIsImmediate`
/// arm carried the retired premise in its name: that when the emitter is the
/// root owner, root position *is* immediate position. `D3c` measured otherwise.
pub(super) enum ContinuationEmitterFrame<'plan> {
    /// The emitting frame is the predeclared producer owner itself, and its
    /// direct-emission consumer stands in that owner's retained lexical
    /// environment.
    Predeclared(PredeclaredFunctionId),
    /// The emitting frame is the generated execution context of an enclosing
    /// specialization. Values are reachable only as that context's declared
    /// captures, which are exactly the enclosing specialization's continuation
    /// inputs laid out after its parameter run.
    GeneratedContext {
        enclosing: ContinuationSpecializationId,
        worker_body_origin: StaticOriginId,
        context_parameters: u32,
        enclosing_inputs: &'plan [ContinuationInputProjection],
    },
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D3b` (re-cut)** — WHERE ONE NAMED CONSUMER
/// holds this value, as a closed sum over **environments**.
///
/// `D1` gave every continuation input a root coordinate
/// ([`ContinuationSourceCoordinate`]): *which value is this, and in whose
/// terms*. That answers identity and is never rewritten. This answers a
/// different question — *where does the consumer about to read it find it* —
/// and ⛔ **neither determines the other.**
///
/// ⛔⛔ **The retired law, named so it is not reconstructed.** `D3b` first
/// landed a product over `(root coordinate, availability)` with three lawful
/// pairings and three crossed, resting on the premise that root provenance
/// constrains availability. `D3c` measured that premise false: at a predeclared
/// seat under one intervening binder an entry root's ABI position 0 is not its
/// immediate position, which is 1. So `EntryAbi` + `CurrentLexical` is not a
/// crossed pair — **it is the lawful answer at nonzero lexical depth.**
/// `RootIsImmediate`, the pairing table and the equality
/// `immediate_slot == source_abi_position` are all retired.
///
/// The two arms are two genuinely different environments:
///
/// - [`Self::CurrentLexical`] — the *semantic* environment in force at one
///   exact predeclared emission occurrence, with intervening binders counted.
///   **Either root arm may take it.**
/// - [`Self::EntryFrame`] — a declared slot in one exactly identified frame's
///   operand run. **Either root arm may take it**, but only where that frame
///   really does declare a member for the full coordinate.
///
/// ⛔ There is no accessor that answers "which index" without first answering
/// "which environment, and whose". Reading a lexical index as a frame slot is
/// the conflation this sum exists to make unrepresentable.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum ContinuationEnvironmentClaimOver<Frame> {
    /// The value sits in the retained lexical environment at one exact
    /// predeclared emission seat.
    ///
    /// ⭐ The index is the **nearest exact alias**: the minimum de Bruijn index
    /// among the positions whose held authority is exactly `Closed([S])` for the
    /// complete requested source slot `S`. See [`nearest_exact_alias`], which is
    /// the single definition of that rule and is re-run by the consumer.
    ///
    /// ⛔ It is **not** a "post-shift index", and that retired spelling is not a
    /// synonym. Post-shift named a count of binders pushed between the value's
    /// scope and the seat, which presumes the value occupies exactly one
    /// position; `let y = x` makes that false. The number here is selected from a
    /// set of proved aliases, and a reader who thinks of it as a shift will
    /// reconstruct the exactly-once law this replaced.
    CurrentLexical {
        emission_owner: PredeclaredFunctionId,
        producer_result_origin: StaticOriginId,
        emission_origin: StaticOriginId,
        lexical_environment_origin: StaticOriginId,
        nearest_alias_index: u32,
    },
    /// The value arrives as a declared slot of one exactly identified frame's
    /// operand run.
    ///
    /// ⭐ **This subsumes the old `GeneratedContextCapture`.** A generated
    /// context's capture run and a predeclared function's entry run are the same
    /// *kind* of environment — a declared operand run — differing only in which
    /// frame declares it. Two names for one environment class is what let the
    /// old law read a frame identity off a root domain.
    EntryFrame {
        frame: Frame,
        declared_slot: u32,
    },
}

/// **Stage 2** — a claim whose frame is an exact, resolved identity. This is the
/// only form any consumer ever sees.
pub(in crate::cranelift_backend) type ContinuationEnvironmentClaim =
    ContinuationEnvironmentClaimOver<ContinuationFrameIdentity>;

/// **Stage 1** — a claim whose frame is still a structural requirement.
/// ⛔ Never published: [`continuation_input_view`] has no way to expose one.
pub(in crate::cranelift_backend) type ContinuationEnvironmentDraft =
    ContinuationEnvironmentClaimOver<ContinuationFrameRequirement>;

/// **STAGE 1 — the STRUCTURAL frame requirement a projection can state while the
/// fixed point is still running.**
///
/// ⛔⛔ **A distinct type from [`ContinuationFrameIdentity`], and that is the
/// mechanism rather than a style choice.** Specializations are interned first,
/// each key carrying these very projections, and `ContinuationContextId`s are
/// minted only afterwards from `enclosing_unit.key.continuation_inputs`. So a
/// claim built during projection *cannot* carry a context id: it does not exist
/// yet. Making the two stages one type would leave a field that is sometimes
/// filled and sometimes not — a half-stamped claim — and every consumer would
/// have to be trusted to check which it holds. Here the requirement simply
/// **cannot be presented to a consumer**: nothing converts one into an identity
/// except [`finalize_continuation_availability`], which resolves it.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum ContinuationFrameRequirement {
    /// A predeclared function's own entry ABI run. ⭐ Already exact at stage 1 —
    /// a predeclared function id is not minted by context interning, so this arm
    /// carries no less evidence than its finalized twin.
    Predeclared(PredeclaredFunctionId),
    /// A generated execution context's frame, named by the pair contexts are
    /// **provisionally interned on**. ⛔ This is a key, not an identity: it says
    /// which context *should* exist, and finalization is what proves exactly one
    /// does.
    GeneratedContext {
        enclosing: ContinuationSpecializationId,
        worker_body_origin: StaticOriginId,
    },
}

/// **STAGE 2 — which frame's operand run an
/// [`ContinuationEnvironmentClaim::EntryFrame`] speaks for**, as an exact
/// identity, published only after every generated context has been minted.
///
/// ⛔ A frame identity is never inferred from a root coordinate. It names the
/// one environment whose declared members can discharge the claim, and a
/// consumer holding a different frame must refuse rather than index its own.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum ContinuationFrameIdentity {
    /// A predeclared function's own entry ABI run.
    Predeclared(PredeclaredFunctionId),
    /// A generated execution context's frame: its parameter run followed by its
    /// declared capture run.
    ///
    /// ⭐ **All three sides are recorded, and the consumer revalidates all
    /// three.** `context` is the resolved identity; `specialization` and
    /// `worker_body_origin` are the key it resolved *from*. Keeping the key
    /// beside the id is what lets a consumer check that the id it holds is the
    /// one this key names, rather than taking the id on trust — the two could
    /// only disagree if finalization resolved against a different plan, which is
    /// exactly the failure worth being unable to hide.
    GeneratedContext {
        context: ContinuationContextId,
        specialization: ContinuationSpecializationId,
        worker_body_origin: StaticOriginId,
    },
}

/// **STAGE 2 — resolve one structural frame requirement to an exact identity.**
///
/// ⛔ Exactly one match, or refuse. Zero means the plan names a frame that was
/// never interned; more than one means the provisional key is not a key at all.
/// Both are refused **here, at finalization**, rather than at whatever seam
/// happens to reach the claim first — which is the difference between a plan
/// that cannot be built and a plan that is accepted and fails later.
pub(super) fn finalize_continuation_frame(
    contexts: &[PlannedContinuationContext],
    requirement: ContinuationFrameRequirement,
) -> Result<ContinuationFrameIdentity, CraneliftBackendError> {
    match requirement {
        // ⭐ Already exact: a predeclared function id is not minted by context
        // interning, so there is nothing to resolve and nothing that could fail.
        ContinuationFrameRequirement::Predeclared(owner) => {
            Ok(ContinuationFrameIdentity::Predeclared(owner))
        }
        ContinuationFrameRequirement::GeneratedContext {
            enclosing,
            worker_body_origin,
        } => {
            let mut found = None;
            for context in contexts {
                if context.enclosing_specialization != enclosing
                    || context.worker_body_origin != worker_body_origin
                {
                    continue;
                }
                if found.is_some() {
                    return Err(planner_error(
                        "two generated execution contexts share one (enclosing specialization, \
                         worker body) pair, so the frame a continuation input names is ambiguous; \
                         RT-CONTSRC-PRODUCER-LOCAL D3b refuses at finalization rather than \
                         publishing a claim that resolves differently depending on which \
                         consumer looks it up",
                    ));
                }
                found = Some(context.id);
            }
            let context = found.ok_or_else(|| {
                planner_error(
                    "a continuation input names a generated execution context frame that was \
                     never interned, so no declared operand run exists to discharge it; \
                     RT-CONTSRC-PRODUCER-LOCAL D3b refuses at finalization rather than \
                     publishing an unresolvable claim",
                )
            })?;
            Ok(ContinuationFrameIdentity::GeneratedContext {
                context,
                specialization: enclosing,
                worker_body_origin,
            })
        }
    }
}

/// Finalize one draft claim. `CurrentLexical` carries no frame and passes
/// through unchanged — it was already exact at stage 1.
pub(super) fn finalize_continuation_claim(
    contexts: &[PlannedContinuationContext],
    draft: ContinuationEnvironmentDraft,
) -> Result<ContinuationEnvironmentClaim, CraneliftBackendError> {
    Ok(match draft {
        ContinuationEnvironmentDraft::CurrentLexical {
            emission_owner,
            producer_result_origin,
            emission_origin,
            lexical_environment_origin,
            nearest_alias_index,
        } => ContinuationEnvironmentClaim::CurrentLexical {
            emission_owner,
            producer_result_origin,
            emission_origin,
            lexical_environment_origin,
            nearest_alias_index,
        },
        ContinuationEnvironmentDraft::EntryFrame {
            frame,
            declared_slot,
        } => ContinuationEnvironmentClaim::EntryFrame {
            frame: finalize_continuation_frame(contexts, frame)?,
            declared_slot,
        },
    })
}

/// Finalize both consumer views of one input.
pub(super) fn finalize_continuation_availability(
    contexts: &[PlannedContinuationContext],
    draft: ContinuationAvailabilityDraft,
) -> Result<ContinuationAvailabilityViews, CraneliftBackendError> {
    Ok(ContinuationAvailabilityViews {
        direct_emission: draft
            .direct_emission
            .map(|claim| finalize_continuation_claim(contexts, claim))
            .transpose()?,
        context_capture: draft
            .context_capture
            .map(|claim| finalize_continuation_claim(contexts, claim))
            .transpose()?,
    })
}

/// **STAGE 2, the pass** — run once, after every generated context has been
/// minted, over every specialization input and every context capture.
///
/// ⛔⛔ **Whole-plan, and that is the obligation rather than a convenience.**
/// Finalizing lazily — resolving each claim the first time some consumer asks
/// for it — would leave a plan carrying an unresolvable frame *accepted*, and
/// refused only if and when something happened to reach it. Every claim is
/// resolved here, whether or not anything will ever read it.
///
/// ⛔ **The obligation does not rest on how reachable the route currently is,
/// and must not be restated as if it did.** An accepted-but-unresolvable claim
/// is the wrong state to publish whatever today's consumption looks like: a
/// claim nothing reaches now may be reached by the next checkpoint, and a plan
/// that cannot be built should fail when it is built. Reachability changes with
/// fixtures; this refusal does not.
pub(super) fn finalize_continuation_availability_plan(
    plan: &mut StaticTransitionPlan<'_>,
) -> Result<(), CraneliftBackendError> {
    let contexts = &plan.continuation_contexts;
    let mut specialization_views = Vec::with_capacity(plan.continuation_specializations.len());
    for unit in &plan.continuation_specializations {
        specialization_views.push(
            unit.key
                .continuation_inputs
                .iter()
                .map(|input| finalize_continuation_availability(contexts, input.availability))
                .collect::<Result<Vec<_>, _>>()?,
        );
    }
    let mut context_views = Vec::with_capacity(contexts.len());
    for context in contexts {
        context_views.push(
            context
                .captures
                .iter()
                .map(|capture| finalize_continuation_availability(contexts, capture.availability))
                .collect::<Result<Vec<_>, _>>()?,
        );
    }
    for (unit, views) in plan
        .continuation_specializations
        .iter_mut()
        .zip(specialization_views)
    {
        unit.finalized_availability = views;
    }
    for (context, views) in plan.continuation_contexts.iter_mut().zip(context_views) {
        context.finalized_availability = views;
    }
    Ok(())
}

/// **`D3b` stage-2 controls** — how the context population is perturbed before
/// finalization is re-run. Test-only.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D3bFinalizationPerturbation {
    Exact,
    /// No context was interned at all: every generated requirement resolves to
    /// ZERO.
    DropContexts,
    /// Every context appears twice under its own key: every generated
    /// requirement resolves to MORE THAN ONE.
    DuplicateContexts,
}

/// Re-run stage-2 finalization over a perturbed context population and report
/// `(generated requirements resolved, claims finalized)`.
///
/// ⛔ The first number is the **non-vacuity** counter, and its premise is
/// arithmetic rather than empirical: a zero-or-multiple perturbation over an
/// **empty** generated-requirement population succeeds trivially — there is
/// nothing to resolve, so nothing can fail to resolve. A control that only
/// asserted the two refusals would therefore pass on a plan carrying no
/// generated requirement at all.
///
/// ⭐ So this counter is what proves that population **nonempty**, and that is
/// the whole of its job. ⛔ It is **not** premised on how reachable the
/// generated-frame route is at any consumer — that is a different question,
/// owned by `D4b`'s behavioural control, and a reachability figure must not be
/// reintroduced here as this control's justification.
#[cfg(test)]
pub(in crate::cranelift_backend) fn d3b_refinalize(
    plan: &StaticTransitionPlan<'_>,
    perturbation: D3bFinalizationPerturbation,
) -> Result<(usize, usize), CraneliftBackendError> {
    let contexts = match perturbation {
        D3bFinalizationPerturbation::Exact => plan.continuation_contexts.clone(),
        D3bFinalizationPerturbation::DropContexts => Vec::new(),
        D3bFinalizationPerturbation::DuplicateContexts => {
            let mut doubled = plan.continuation_contexts.clone();
            doubled.extend(plan.continuation_contexts.iter().cloned());
            doubled
        }
    };
    let mut generated = 0;
    let mut total = 0;
    let mut count = |draft: ContinuationAvailabilityDraft| {
        for claim in [draft.direct_emission, draft.context_capture].into_iter().flatten() {
            total += 1;
            if matches!(
                claim,
                ContinuationEnvironmentDraft::EntryFrame {
                    frame: ContinuationFrameRequirement::GeneratedContext { .. },
                    ..
                }
            ) {
                generated += 1;
            }
        }
    };
    for unit in &plan.continuation_specializations {
        for input in &unit.key.continuation_inputs {
            count(input.availability);
            finalize_continuation_availability(&contexts, input.availability)?;
        }
    }
    for context in &plan.continuation_contexts {
        for capture in &context.captures {
            count(capture.availability);
            finalize_continuation_availability(&contexts, capture.availability)?;
        }
    }
    Ok((generated, total))
}

/// Attempt to publish a view with no finalized entry — the publication gate.
/// Test-only.
#[cfg(test)]
pub(in crate::cranelift_backend) fn d3b_publish_without_finalization(
    plan: &StaticTransitionPlan<'_>,
) -> Result<(), CraneliftBackendError> {
    let projection = plan
        .continuation_specializations
        .first()
        .and_then(|unit| unit.key.continuation_inputs.first())
        .ok_or_else(|| planner_error("the fixture plans no continuation input"))?;
    continuation_input_view(projection, None).map(|_| ())
}

/// **The two consumer-specific availability views of one continuation input.**
///
/// ⛔⛔ **One unqualified index cannot be authority for both consumers, and this
/// is measured rather than argued.** The direct continuation-call emission reads
/// `producer_env` — the exact lexical environment standing at the producer's
/// emission seat. The generated-context capture append reads
/// `function_local.defining_abi_operands` — an entry-frame operand run. `D3c`
/// showed those two disagree at nonzero binder depth, so a single `availability`
/// field repaired for one consumer silently mis-serves the other.
///
/// ⭐ The split is not artificial: the two consumers already read **two physical
/// copies** of these records — the specialization's `continuation_inputs`, and
/// the context's `captures`, which is a clone of them.
///
/// ⛔ A closed record keyed by consumer kind, deliberately **not** an unkeyed
/// vector and **not** a "first matching availability" search. A consumer takes
/// its own field or refuses; there is no arm that lets it fall back to the
/// other's.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct ContinuationAvailabilityOver<Frame> {
    /// For the direct continuation-call emission consumer.
    pub(in crate::cranelift_backend) direct_emission:
        Option<ContinuationEnvironmentClaimOver<Frame>>,
    /// For the generated-context capture-append consumer.
    pub(in crate::cranelift_backend) context_capture:
        Option<ContinuationEnvironmentClaimOver<Frame>>,
}

/// **Stage 2** — the published, immutable views. Both consumers read this form.
pub(in crate::cranelift_backend) type ContinuationAvailabilityViews =
    ContinuationAvailabilityOver<ContinuationFrameIdentity>;

/// **Stage 1** — what a projection carries while the fixed point runs.
pub(in crate::cranelift_backend) type ContinuationAvailabilityDraft =
    ContinuationAvailabilityOver<ContinuationFrameRequirement>;

/// One exact source-slot value in the environment carried by a producer edge
/// into a computational continuation.
///
/// This is deliberately distinct from occurrence/result lifetime authority.
/// A persistent function result does not narrow a `ValueWord` input that may
/// still name invocation-owned storage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct ContinuationSourceSlotAuthority {
    /// `D1` — the closed coordinate domain. The contract fields below are
    /// slot-derived for the `EntryAbi` arm and planner-derived for the
    /// `ProducerLocal` one; that difference is in how they are *obtained*, not
    /// in what they mean, so they stay beside the coordinate rather than
    /// inside it.
    pub(in crate::cranelift_backend) coordinate: ContinuationSourceCoordinate,
    pub(in crate::cranelift_backend) carrier: AbiCarrier,
    pub(in crate::cranelift_backend) ownership: AbiOwnership,
    pub(in crate::cranelift_backend) storage_owner: AbiStorageOwner,
    pub(in crate::cranelift_backend) referent_affinity: Vec<BoundaryReferentOwner>,
}

/// One semantic value's closed source-ABI provenance while walking the exact
/// source environment of a continuation owner.
///
/// `Open` is deliberately contagious. An opaque producer, an unbound local,
/// or two distinct ABI sources cannot be converted into one plausible slot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum ContinuationValueSourceAuthority {
    Closed(Vec<ContinuationSourceSlotAuthority>),
    Open,
}

impl ContinuationValueSourceAuthority {
    pub(super) fn source(source: ContinuationSourceSlotAuthority) -> Self {
        Self::Closed(vec![source])
    }

    pub(super) fn join(self, other: Self) -> Self {
        match (self, other) {
            (Self::Closed(mut left), Self::Closed(right)) => {
                for source in right {
                    if !left.contains(&source) {
                        left.push(source);
                    }
                }
                Self::Closed(left)
            }
            (Self::Open, _) | (_, Self::Open) => Self::Open,
        }
    }
}

/// The exact producer-to-continuation environment consumed by D1 projection.
///
/// The edge identity stays beside the ordered source slots so the projection
/// cannot be derived from a consumer descriptor in isolation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ContinuationProducerEnvironment {
    pub(super) producer_owner: PredeclaredFunctionId,
    pub(super) producer_result_origin: StaticOriginId,
    pub(super) producer_construct_origin: StaticOriginId,
    pub(super) consumer_owner: PredeclaredFunctionId,
    pub(super) inputs: Vec<ContinuationSourceSlotAuthority>,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ContinuationProjectionOmission {
    ProducerOwner,
    ConsumerOwner,
    SourceOwner,
    SourceAbiPosition,
    Source,
    Ordinal,
    Carrier,
    Ownership,
    StorageOwner,
    ReferentAffinity,
    OrdinaryAbiPosition,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ContinuationInternMutation {
    Exact,
    OmitProjection(ContinuationProjectionOmission),
    PrefixOnly,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ContinuationProductionMutation {
    Exact,
    ResultLifetimeProxy,
    ConstructorFieldCountPrefix,
    DescriptorOrdinalSources,
    DescriptorInputCountTruncation,
}

#[cfg(test)]
thread_local! {
    pub(super) static CONTINUATION_INTERN_MUTATION: Cell<ContinuationInternMutation> =
        const { Cell::new(ContinuationInternMutation::Exact) };
    pub(super) static CONTINUATION_PRODUCTION_MUTATION: Cell<ContinuationProductionMutation> =
        const { Cell::new(ContinuationProductionMutation::Exact) };
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum ContinuationWorkerCaptureSource {
    Seed,
    Lexical(StaticOriginId),
}

/// One selected worker capture's exact provenance.
///
/// **`RT-CONTSRC-PRODUCER-LOCAL` `D7a`** widened this from module-private to
/// backend-visible so [`ComposedWorkerView`] can carry the *same* record the
/// specialization key holds rather than a second copy of its shape. The fields
/// stay private with read-only accessors: the point is that a consumer can read
/// this provenance and cannot construct one, which a `pub`-field mirror would
/// have given away.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct ContinuationWorkerCaptureProvenance {
    pub(super) ordinal: u32,
    pub(super) owner: PredeclaredFunctionId,
    pub(super) closure_origin: StaticOriginId,
    pub(super) source: ContinuationWorkerCaptureSource,
    pub(super) lifetime: PlannedReferentLifetime,
}

// Read by this node's tests; `D7b`/`D7c` are the held production consumers.
#[cfg_attr(not(test), allow(dead_code))]
impl ContinuationWorkerCaptureProvenance {
    pub(in crate::cranelift_backend) fn ordinal(&self) -> u32 {
        self.ordinal
    }
    pub(in crate::cranelift_backend) fn owner(&self) -> PredeclaredFunctionId {
        self.owner
    }
    pub(in crate::cranelift_backend) fn closure_origin(&self) -> StaticOriginId {
        self.closure_origin
    }
    pub(in crate::cranelift_backend) fn source(&self) -> ContinuationWorkerCaptureSource {
        self.source
    }
    pub(in crate::cranelift_backend) fn lifetime(&self) -> PlannedReferentLifetime {
        self.lifetime
    }
}

/// Exact source provenance of the static worker whose result enters a return
/// hole. This remains planner data; Slice 1 emits no worker or call.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct ContinuationWorkerProvenance {
    pub(super) parent_origin: StaticOriginId,
    pub(super) producer_origin: StaticOriginId,
    pub(super) sibling_position: u32,
    pub(super) closure_origin: StaticOriginId,
    pub(super) body_origin: StaticOriginId,
    pub(super) declared_arity: u32,
    pub(super) captures: Vec<ContinuationWorkerCaptureProvenance>,
}

/// The exact outer occurrence that consumes one continuation result.
///
/// The body is the selected outer case body. The eliminator is retained beside
/// it because the body alone does not identify the path through which the
/// result was consumed. Both coordinates are minted by the forward outer-match
/// walk; neither is recovered from an owner or by searching backwards from the
/// continuation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct ContinuationConsumingOccurrence {
    pub(super) body_origin: StaticOriginId,
    pub(super) eliminator_origin: StaticOriginId,
}

impl ContinuationConsumingOccurrence {
    pub(in crate::cranelift_backend) fn body_origin(self) -> StaticOriginId {
        self.body_origin
    }

    pub(in crate::cranelift_backend) fn eliminator_origin(self) -> StaticOriginId {
        self.eliminator_origin
    }
}

/// A continuation call's independently derived consumer-level occurrence.
///
/// This is deliberately separate from
/// [`ContinuationSpecializationKey::consuming_occurrence`]. That key field is
/// the source-level certificate whose position-zero child is the target
/// continuation. This projection names the occurrence required after the
/// target body is realized: the same occurrence at depth one, and the unique
/// outer consumer from depth two onward.
///
/// The fields are private and there is no constructor outside planning.
/// Lowering can only receive a value that the whole-plan validator has matched
/// against [`derive_required_consumer_occurrence`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct RequiredConsumerProjection {
    pub(super) source: ContinuationConsumingOccurrence,
    pub(super) required: ContinuationConsumingOccurrence,
}

impl RequiredConsumerProjection {
    pub(in crate::cranelift_backend) fn source(self) -> ContinuationConsumingOccurrence {
        self.source
    }

    pub(in crate::cranelift_backend) fn body_origin(self) -> StaticOriginId {
        self.required.body_origin
    }

    pub(in crate::cranelift_backend) fn eliminator_origin(self) -> StaticOriginId {
        self.required.eliminator_origin
    }
}

/// The complete immutable identity of one continuation specialization.
///
/// The ordered projection vector is owned directly. There is no parallel
/// count, summary, or post-assignment widening operation.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct ContinuationSpecializationKey {
    /// ⚠ **`D5a`: PROVENANCE ONLY.** This is the raw source-occurrence owner —
    /// the unit the producer `Construct` is textually in. It does **not** confer
    /// emission authority; see `emission_owner` beside it.
    pub(super) producer_owner: PredeclaredFunctionId,
    /// **`D5a` — the immediate emission and availability owner.**
    ///
    /// Who can actually emit this call and possess its operands. Equal to
    /// `Predeclared(producer_owner)` for every edge discovered at a top-level
    /// computational frame, which is the entire pre-`D5a` population.
    pub(super) emission_owner: ContinuationEmissionOwner,
    pub(super) producer_result_origin: StaticOriginId,
    pub(super) producer_construct_origin: StaticOriginId,
    pub(super) producer_alternative: u32,
    /// The owner of the continuation occurrence itself.
    ///
    /// This is provenance only for the consumer half; it does not name the
    /// outer case that consumes the continuation's answer. Widening it is
    /// closed: [`exact_continuation_source_environment`] validates that the
    /// continuation occurrence has this exact owner and fails closed on an
    /// inequality.
    pub(super) consumer_owner: PredeclaredFunctionId,
    /// The exact outer selected case body and eliminator that consume this
    /// specialization's result.
    ///
    /// This confers occurrence-level consuming authority. `None` means the
    /// forward outer-match walk did not establish one exact consuming case; it
    /// never falls back to `consumer_owner` or a reverse lookup.
    pub(super) consuming_occurrence: Option<ContinuationConsumingOccurrence>,
    pub(super) continuation_origin: StaticOriginId,
    pub(super) recursive_position: u32,
    /// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2b` — THE CLOSED PROJECTION.**
    ///
    /// Every recursive source position of this producer construct, not just the
    /// one this unit specializes. ⛔ `recursive_position` above is **singular**
    /// and always will be — one interned unit per position — so it cannot
    /// answer *"is field `k` recursive?"*, only *"is field `k` MINE?"*. The
    /// envelope needs the former, and asking the singular field for it calls
    /// every sibling recursive position nonrecursive by construction.
    ///
    /// ⛔ Set-equal to the case's checked `recursive_positions` by construction:
    /// it is copied from that set, not derived from a body, a lowered shape, an
    /// arity or a constructor symbol. Uniqueness is structural — it is a set.
    pub(super) recursive_positions: BTreeSet<u32>,
    pub(super) worker: ContinuationWorkerProvenance,
    pub(super) ordinary_parameters: u32,
    pub(super) continuation_inputs: Vec<ContinuationInputProjection>,
}

/// **`RT-CONTSPEC-ACTIVATE` `D1` — the opaque causal identity.**
///
/// Four fields: the token's `(producer_construct_origin, producer_alternative,
/// call_site_sequence)` plus the `recursive_position` read from the resolved
/// target's planner key.
///
/// Every field is private and there is **no lowering constructor** — the only
/// way to obtain one is
/// [`StaticTransitionPlan::continuation_call_binding_for`], which resolves it
/// from already-validated planner facts. There is deliberately **no sequence
/// accessor**: the call-site sequence stays opaque *inside* the identity, so
/// lowering can neither supply it nor derive it. Full `Eq`/`Ord` so the
/// identity can key a map without any field being readable.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct ContinuationCallIdentity {
    pub(super) token: ContinuationSpecializationCallToken,
    pub(super) recursive_position: u32,
}

impl ContinuationCallIdentity {
    /// The exact specialization this causal edge targets.
    ///
    /// This is the only fact the identity exposes: it is what a later slice
    /// resolves to a declared target, and it reveals nothing about sequence.
    pub(in crate::cranelift_backend) fn target(&self) -> ContinuationSpecializationId {
        self.token.target
    }

    /// The producer unit that owns this causal edge.
    ///
    /// `D3` compares this against the exact unit currently being defined. It
    /// is the token's own immutable fact, re-exposed and not reconstructed --
    /// there is no second owner authority, and nothing derives an owner from
    /// emission position.
    pub(in crate::cranelift_backend) fn producer_owner(&self) -> PredeclaredFunctionId {
        self.token.producer_owner
    }

    /// **`D5a` — who emits this call.**
    ///
    /// ⛔ `D3`'s owner comparison moves to THIS accessor. `producer_owner`
    /// above is provenance and answering the claim check with it is the exact
    /// conflation `evt_609am4v7cdt5b` ruled against.
    pub(in crate::cranelift_backend) fn emission_owner(&self) -> ContinuationEmissionOwner {
        self.token.emission_owner
    }
}

/// `D1` — a read-only view of one already-validated continuation
/// specialization: its exact identity, immutable planner key facts, and
/// validated ABI descriptor, slots and input authority.
pub(in crate::cranelift_backend) struct ContinuationUnitView<'plan> {
    pub(super) id: ContinuationSpecializationId,
    pub(super) key: &'plan ContinuationSpecializationKey,
    /// Stage-2 availability, one per continuation input. ⛔ See
    /// [`ContinuationContextView::finalized`].
    pub(super) finalized: &'plan [ContinuationAvailabilityViews],
    pub(super) header: AbiFrameHeader,
    pub(super) slots: &'plan [AbiSlot],
    pub(super) inputs: &'plan [abi::AbiContinuationInputAuthority],
}

impl<'plan> ContinuationUnitView<'plan> {
    pub(in crate::cranelift_backend) fn id(&self) -> ContinuationSpecializationId {
        self.id
    }
    pub(in crate::cranelift_backend) fn producer_owner(&self) -> PredeclaredFunctionId {
        self.key.producer_owner
    }
    /// **`D5a`** — the immediate emission and availability owner.
    pub(in crate::cranelift_backend) fn emission_owner(&self) -> ContinuationEmissionOwner {
        self.key.emission_owner
    }
    pub(in crate::cranelift_backend) fn consumer_owner(&self) -> PredeclaredFunctionId {
        self.key.consumer_owner
    }
    pub(in crate::cranelift_backend) fn consuming_occurrence(
        &self,
    ) -> Option<ContinuationConsumingOccurrence> {
        self.key.consuming_occurrence
    }
    pub(in crate::cranelift_backend) fn producer_construct_origin(&self) -> StaticOriginId {
        self.key.producer_construct_origin
    }
    pub(in crate::cranelift_backend) fn producer_result_origin(&self) -> StaticOriginId {
        self.key.producer_result_origin
    }
    pub(in crate::cranelift_backend) fn producer_alternative(&self) -> u32 {
        self.key.producer_alternative
    }
    pub(in crate::cranelift_backend) fn continuation_origin(&self) -> StaticOriginId {
        self.key.continuation_origin
    }
    pub(in crate::cranelift_backend) fn recursive_position(&self) -> u32 {
        self.key.recursive_position
    }

    /// **`D2b` — the closed projection.** Every recursive source position of
    /// this producer construct. ⛔ Read-only: lowering may consult it and must
    /// never reconstruct membership from a lowered shape, arity, body or
    /// constructor symbol.
    pub(in crate::cranelift_backend) fn recursive_positions(&self) -> &BTreeSet<u32> {
        &self.key.recursive_positions
    }
    pub(in crate::cranelift_backend) fn ordinary_parameters(&self) -> u32 {
        self.key.ordinary_parameters
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

    /// **The ruled ordinary envelope**, one role per `Parameter` ABI slot, in
    /// slot order.
    ///
    /// `nonrecursive_field_count = ordinary_parameters - worker_capture_count`,
    /// computed with checked arithmetic. Positions
    /// `0..nonrecursive_field_count` are nonrecursive producer-`Construct`
    /// fields in producer source order with **every** recursive position
    /// omitted; worker capture `ordinal` occupies
    /// `nonrecursive_field_count + ordinal`.
    ///
    /// ⛔ **Every projected recursive position, not just the selected one** —
    /// `D2b`. This contract previously said "with the selected recursive
    /// position omitted", which is the singular model that let a SIBLING
    /// recursive field through as an ordinary ABI parameter. The field count is
    /// `nonrecursive_field_count + |recursive_positions|` for the same reason:
    /// `+ 1` was the same assumption written a second time.
    ///
    /// Each role is recompared against the validated slot run before it is
    /// returned: the count of `Parameter` slots must equal
    /// `header.parameters`, which must equal the envelope length.
    /// ⛔ **The absent nonrecursive prefix is `Ok(None)`, not `Err`, and that
    /// distinction is the whole point of this method existing beside
    /// [`Self::ordinary_envelope`].** A caller that has already established a
    /// unit is enveloped wants the failure; a caller deciding **membership**
    /// needs the two apart. "Fewer ordinary parameters than the selected worker
    /// has captures" is the ruled boundary of a domain — no prefix means no
    /// capture-to-ordinary-parameter correspondence, hence no coordinate run —
    /// while every other failure here is an integrity defect about a unit that
    /// IS in the domain. Collapsing them makes a malformed envelope read as a
    /// non-member and silently issue nothing, which is fail-open.
    ///
    /// There is deliberately no second copy of the capture subtraction at the
    /// membership call site: this is the one authority, and a caller that
    /// re-derived the predicate could drift from it.
    pub(in crate::cranelift_backend) fn ruled_ordinary_envelope(
        &self,
    ) -> Result<Option<Vec<ContinuationOrdinaryEnvelopeRole>>, CraneliftBackendError> {
        let captures = u32::try_from(self.key.worker.captures.len()).map_err(|_| {
            planner_error("worker capture count exceeds addressable range")
        })?;
        let Some(nonrecursive_field_count) = self.key.ordinary_parameters.checked_sub(captures)
        else {
            return Ok(None);
        };

        let parameter_slots = self
            .slots
            .iter()
            .filter(|slot| slot.kind == AbiSlotKind::Parameter)
            .count();
        if parameter_slots != self.header.parameters as usize {
            return Err(planner_error(
                "the continuation slot run's Parameter count disagrees with its own header",
            ));
        }

        // ⭐⭐ `RT-CONTSRC-PRODUCER-LOCAL` `D8l2` — THE NONRECURSIVE POPULATION
        // IS SOURCE POSITIONS, NOT ENVELOPE INDICES.
        //
        // The producer `Construct` has `N + |recursive_positions|` fields: `N`
        // nonrecursive ones and the whole recursive run. The roles are those
        // `N` fields at their OWN source positions, in source order, with
        // **every** recursive position skipped.
        //
        // ⛔ What stood here emitted `0..N` — the envelope index — and called it
        // `source_position`. The two coincide only while every nonrecursive
        // field precedes every recursive position, because omitting a later
        // position does not renumber the earlier ones but omitting an earlier
        // one renumbers every later one. `px8tr` selects its last field, so the
        // defect was unreachable on every landed fixture and the method's own
        // doc comment — "in producer source order with the recursive positions
        // omitted" — described the rule the loop did not implement. `D8l1`
        // measured it with two witnesses differing only in field order.
        //
        // ⛔ This is neither a reverse source walk nor a new identity: the
        // producer's field count is `N + |recursive_positions|` by construction
        // from the checked capture subtraction above plus the closed
        // projection's own size, and the positions omitted are that
        // projection's members. Nothing is inferred from a body, a shape, or an
        // ABI slot.
        //
        // ⛔ `D2b` — THE COUNT AND THE SKIP ARE BOTH PLURAL, and they were both
        // singular. This paragraph read `N + 1` and "the selected recursive
        // one", and the loop matched it. That model is correct exactly when a
        // producer has one recursive position, which every landed fixture had —
        // so a sibling recursive field was counted nonrecursive, became an
        // ordinary ABI parameter, and carried a `Specialized(Closure)` the
        // boundary walk then correctly refused. `D8l2`'s SOURCE-ORDER lesson is
        // untouched by that correction and is the reason this paragraph is
        // rewritten rather than deleted.
        //
        // ⛔ Self-membership is required, not defensive: `nonrecursive_field_count`
        // is derived from an arity that excluded every projected position, so if
        // this unit's own position were absent from the projection the two
        // derivations would disagree and the count below would be silently short.
        if !self
            .key
            .recursive_positions
            .contains(&self.key.recursive_position)
        {
            return Err(planner_error(
                "a continuation's own recursive position is absent from its closed projection,                  so the envelope's field count cannot be derived from it",
            ));
        }
        let recursive_run = u32::try_from(self.key.recursive_positions.len())
            .map_err(|_| planner_error("the recursive position run exceeds addressable range"))?;
        let field_count = nonrecursive_field_count
            .checked_add(recursive_run)
            .ok_or_else(|| {
                planner_error("the producer constructor's field count overflows the envelope")
            })?;
        // ⛔ The range refusal is REQUIRED, not defensive, and its reason is an
        // IMPOSSIBLE KEY STATE rather than any arity consequence: a selected
        // position at or past the field count names a field the producer
        // construct does not have, so this unit's identity does not identify
        // anything in the run it claims to specialize.
        //
        // ⛔ It does NOT cause a surplus role or a slot-length mismatch, and
        // saying so would be false under the repaired model. The loop below
        // filters on the CLOSED PROJECTION, which removes every real recursive
        // position whatever `selected` is; an out-of-range `selected` simply
        // matches nothing, and the emitted roles remain the normal nonrecursive
        // run. That consequence belonged to the singular model, where `selected`
        // was the only thing omitted -- it does not survive the plural one, and
        // it is not this guard's rationale.
        if self.key.recursive_position >= field_count {
            return Err(planner_error(
                "a continuation selects a recursive position outside its producer constructor's \
                 field run, so the ordinary envelope cannot omit it and the remaining fields do \
                 not name a population",
            ));
        }
        // ⛔ `D8l2` — the defect switch perturbs the SELECTION the population is
        // built from, never the range check or the loop. Out-of-range selection
        // is unreachable through any plan the planner builds -- the key's
        // recursive position and the field count are derived from one producer
        // -- so the refusal would otherwise ship unexercised.
        #[cfg(test)]
        let selected = match envelope_defect() {
            EnvelopeDefect::SelectionOutOfRange => field_count,
            _ => self.key.recursive_position,
        };
        #[cfg(not(test))]
        let selected = self.key.recursive_position;
        // ⛔ The range refusal is REQUIRED, not defensive, and its reason is an
        // IMPOSSIBLE KEY STATE rather than any arity consequence: a selected
        // position at or past the field count names a field the producer
        // construct does not have, so this unit's identity does not identify
        // anything in the run it claims to specialize.
        //
        // ⛔ It does NOT cause a surplus role or a slot-length mismatch, and
        // saying so would be false under the repaired model. The loop below
        // filters on the CLOSED PROJECTION, which removes every real recursive
        // position whatever `selected` is; an out-of-range `selected` simply
        // matches nothing, and the emitted roles remain the normal nonrecursive
        // run. That consequence belonged to the singular model, where `selected`
        // was the only thing omitted -- it does not survive the plural one, and
        // it is not this guard's rationale.
        if selected >= field_count {
            return Err(planner_error(
                "a continuation selects a recursive position outside its producer constructor's \
                 field run, so the ordinary envelope cannot omit it and the remaining fields do \
                 not name a population",
            ));
        }
        let mut envelope = Vec::with_capacity(parameter_slots);
        // ⭐⭐ `D2b` — omit EVERY recursive position, from the closed
        // projection, not only `selected`. `selected` is this unit's own
        // position; a sibling recursive field is equally not a runtime value,
        // and calling it nonrecursive is what put a `Specialized(Closure)` into
        // the ordinary run. The two agree whenever a producer has exactly one
        // recursive position, which is why this was green on every landed
        // fixture until a sibling shape appeared.
        let mut nonrecursive = (0..field_count)
            .filter(|position| {
                *position != selected
                    && !u32::try_from(*position)
                        .ok()
                        .is_some_and(|encoded| self.key.recursive_positions.contains(&encoded))
            })
            .collect::<Vec<_>>();
        // ⛔ `D8l2` — the four population defects, applied to the built
        // population. Each is a shape a wrong derivation would actually
        // produce: the dense prefix is exactly what stood here before this
        // repair.
        #[cfg(test)]
        match envelope_defect() {
            EnvelopeDefect::Exact | EnvelopeDefect::SelectionOutOfRange => {}
            EnvelopeDefect::Omit => {
                nonrecursive.pop();
            }
            EnvelopeDefect::Duplicate => {
                if let Some(first) = nonrecursive.first().copied() {
                    if let Some(last) = nonrecursive.last_mut() {
                        *last = first;
                    }
                }
            }
            EnvelopeDefect::DensePrefix => {
                nonrecursive = (0..nonrecursive_field_count).collect();
            }
            EnvelopeDefect::WrongOrder => nonrecursive.reverse(),
        }
        for source_position in nonrecursive {
            envelope.push(ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField {
                source_position,
            });
        }
        for capture in &self.key.worker.captures {
            let position = nonrecursive_field_count
                .checked_add(capture.ordinal)
                .ok_or_else(|| planner_error("worker capture position overflows the envelope"))?;
            if position as usize != envelope.len() {
                return Err(planner_error(
                    "worker captures are not dense in capture-ordinal order, so the ruled \
                     envelope position cannot be assigned",
                ));
            }
            envelope.push(ContinuationOrdinaryEnvelopeRole::WorkerCapture {
                ordinal: capture.ordinal,
                owner: capture.owner,
                closure_origin: capture.closure_origin,
                source: capture.source,
                lifetime: capture.lifetime,
            });
        }
        if envelope.len() != parameter_slots {
            return Err(planner_error(
                "the ruled ordinary envelope does not cover its Parameter slot run exactly",
            ));
        }
        Ok(Some(envelope))
    }

    /// The ruled envelope of a unit already established to have one.
    ///
    /// Delegates to [`Self::ruled_ordinary_envelope`] and restates its one
    /// non-membership disposition as the error every existing caller already
    /// expects. The arithmetic, the slot recomparison and the message all live
    /// there; this is the projection, not a second derivation.
    pub(in crate::cranelift_backend) fn ordinary_envelope(
        &self,
    ) -> Result<Vec<ContinuationOrdinaryEnvelopeRole>, CraneliftBackendError> {
        self.ruled_ordinary_envelope()?.ok_or_else(|| {
            planner_error(
                "a continuation declares fewer ordinary parameters than its selected worker \
                 has captures, so the ruled envelope has no nonrecursive prefix",
            )
        })
    }

    /// **The ordered continuation inputs**, re-exposed from the immutable key
    /// and recompared against the validated ABI input authority.
    ///
    /// Each projected input must agree with the authority at its position on
    /// ordinal and source owner, and the input's `ordinary_abi_position` must
    /// name a real `Parameter` slot, so a projection cannot point outside the
    /// envelope it is supposed to index.
    pub(in crate::cranelift_backend) fn continuation_inputs(
        &self,
    ) -> Result<Vec<ContinuationInputView>, CraneliftBackendError> {
        self.key
            .continuation_inputs
            .iter()
            .zip(self.inputs)
            .enumerate()
            .map(|(position, (projection, authority))| {
                // `D3a` — domain-tagged provenance agreement, as at the
                // generated-context capture consumer above.
                if projection.ordinal != authority.ordinal
                    || abi::AbiContinuationInputProvenance::of(projection.coordinate)
                        != authority.provenance
                {
                    return Err(planner_error(
                        "a continuation input projection disagrees with its validated ABI input \
                         authority",
                    ));
                }
                continuation_input_view(projection, self.finalized.get(position))
            })
            .collect()
    }

    /// The static worker whose result enters this continuation's return hole.
    ///
    /// These are the already-validated planner facts a `D2` definition needs
    /// to bind the body through `RT-WORKER-BIND`'s environment: the exact body
    /// origin to lower, the declared arity, and the ordered capture count.
    /// They are read, never re-derived -- lowering may not walk source syntax
    /// to rediscover them.
    pub(in crate::cranelift_backend) fn worker_body_origin(&self) -> StaticOriginId {
        self.key.worker.body_origin
    }
    pub(in crate::cranelift_backend) fn worker_closure_origin(&self) -> StaticOriginId {
        self.key.worker.closure_origin
    }
    pub(in crate::cranelift_backend) fn worker_declared_arity(&self) -> u32 {
        self.key.worker.declared_arity
    }
    pub(in crate::cranelift_backend) fn worker_capture_count(&self) -> usize {
        self.key.worker.captures.len()
    }

    /// Byte offsets for this unit's slot run, and the frame size, from the
    /// **one** offset walk `B2F` owns.
    ///
    /// This is the same `abi::slot_offsets` the emittable path uses, not a
    /// second derivation: a continuation body must load each declared operand
    /// from its own frame position, and the alternative -- letting the emitter
    /// prefix-sum widths itself -- is precisely the second layout authority
    /// that walk exists to prevent.
    ///
    /// Fails closed when the walked frame size disagrees with the descriptor's
    /// declared `frame_bytes`, so a corrupted descriptor is rejected rather
    /// than silently emitted against.
    pub(in crate::cranelift_backend) fn slot_offsets(
        &self,
    ) -> Result<(Vec<u32>, u32), CraneliftBackendError> {
        let (offsets, frame_bytes) = abi::slot_offsets(self.slots)?;
        if frame_bytes != self.header.frame_bytes {
            return Err(planner_error(
                "a continuation descriptor's frame size disagrees with its own slot run",
            ));
        }
        Ok((offsets, frame_bytes))
    }
}

/// **The ruled ordinary-envelope role of one `Parameter` ABI slot.**
///
/// The Architect's ruling: the Parameter prefix is
/// `[nonrecursive producer-Construct fields in source order]
///  ++ [selected worker captures in capture-ordinal order]`,
/// with **every** recursive field omitted.
///
/// ⛔ `D2b`: this read "with the selected recursive field omitted". A producer
/// with one recursive position makes the two readings identical, which is why
/// the singular wording survived every landed fixture.
///
/// This is a **role projection**, not a worker-body environment map. The
/// continuation descriptor's contract and the worker's `arity + captures`
/// contract are distinct, and nothing here relates a slot to a lexical
/// position in the worker body.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum ContinuationOrdinaryEnvelopeRole {
    /// A nonrecursive field of the producer `Construct`, at its **source**
    /// position.
    ///
    /// ⛔ **NO recursive field is in this population** — `D2b`. This doc
    /// previously said "the selected recursive field is not in this
    /// population", which was the singular reading that let a sibling recursive
    /// field in as an ordinary ABI parameter.
    NonrecursiveConstructorField { source_position: u32 },
    /// One selected worker capture, at its capture ordinal.
    WorkerCapture {
        ordinal: u32,
        owner: PredeclaredFunctionId,
        closure_origin: StaticOriginId,
        source: ContinuationWorkerCaptureSource,
        lifetime: PlannedReferentLifetime,
    },
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D7a` — whether the planner issued a generated
/// execution context for the composed frame's worker body.**
///
/// ⛔ **This is eligibility, not a route.** The `D6a` route law is asymmetric
/// and this enum does not restate half of it:
///
/// - the **selected recursive constructor argument** calls
///   [`StaticWorkerCallRoute::RawWorker`] **unconditionally**, at every value of
///   this enum. `GeneratedContextIssued` is not a licence to route it anywhere
///   else;
/// - an **induction hypothesis** at this frame carries
///   `GeneratedContext` **iff** this is `GeneratedContextIssued` *and* the unit
///   defining it resolved that exact context. `RawOnly` means every route out of
///   this frame is raw, and equal routes are then a lawful, route-degenerate
///   state — never evidence that one binding was reused for both.
///
/// [`StaticWorkerCallRoute::RawWorker`]:
///     crate::cranelift_backend::lowering::StaticWorkerCallRoute
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum ComposedWorkerRouteEligibility {
    /// The planner interned exactly this context for `(specialization, worker
    /// body)`. The identity is carried, not merely the fact, so a consumer that
    /// later resolves a context has something to compare against rather than a
    /// boolean it must re-derive.
    GeneratedContextIssued(ContinuationContextId),
    /// No generated context names this specialization and this worker body, so
    /// the raw worker is the only target. ⛔ Not "none found yet": the lookup
    /// this is read from is a singleton resolution that refuses rather than
    /// choosing.
    RawOnly,
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D7a` — one composed frame's static worker, as
/// the planner already knows it.**
///
/// ## Why this exists
///
/// The composed eliminator path holds the *causal* coordinates — the producer
/// `Construct` occurrence it is building, the computational-frame origin, the
/// selected alternative, and the ruled recursive source position — and does
/// **not** hold the continuation unit that carries the worker those coordinates
/// select. Every fact below already sits in an interned
/// [`ContinuationSpecializationKey`]; without this projection the only route
/// from those coordinates to these facts is to walk the closure occurrence and
/// read its shape, which is the second authority `RT-WORKER-BIND` exists to
/// prevent.
///
/// ⛔ **Exposure, not discovery.** Nothing here is computed from a lowered
/// value, an emitted shape, a body arity, an environment length, or from which
/// of the two targets happens to exist. Every field is copied out of the key
/// and re-checked against an independent planner fact before it is returned.
///
/// ⛔ **Unmintable.** Every field is private, there is no public constructor,
/// and the only way to obtain one is
/// [`StaticTransitionPlan::composed_worker_view`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct ComposedWorkerView {
    pub(super) closure_origin: StaticOriginId,
    pub(super) body_origin: StaticOriginId,
    pub(super) declared_arity: u32,
    pub(super) captures: Vec<ContinuationWorkerCaptureProvenance>,
    pub(super) route_eligibility: ComposedWorkerRouteEligibility,
    pub(super) recursive_position: u32,
}

// Read by this node's tests; `D7b`/`D7c` are the held production consumers.
#[cfg_attr(not(test), allow(dead_code))]
impl ComposedWorkerView {
    /// The exact closure occurrence the specialization selected.
    pub(in crate::cranelift_backend) fn closure_origin(&self) -> StaticOriginId {
        self.closure_origin
    }
    /// The **raw** worker body — the closure occurrence's own body child, and
    /// the origin a raw-route call resolves. It is re-derived through the sole
    /// child-origin production point before this view is built, so it is not
    /// read back off the same children list that recorded it.
    pub(in crate::cranelift_backend) fn body_origin(&self) -> StaticOriginId {
        self.body_origin
    }
    /// The worker's declared arity, from the key. ⛔ Not a body parameter count
    /// re-read at the call site.
    pub(in crate::cranelift_backend) fn declared_arity(&self) -> u32 {
        self.declared_arity
    }
    /// The ordered capture provenance, dense in capture ordinal. Its agreement
    /// with the ABI-validated ordinary envelope is checked before this view is
    /// built.
    pub(in crate::cranelift_backend) fn captures(&self) -> &[ContinuationWorkerCaptureProvenance] {
        &self.captures
    }
    /// See [`ComposedWorkerRouteEligibility`] — eligibility, and specifically
    /// **not** the selected recursive argument's route, which is always raw.
    pub(in crate::cranelift_backend) fn route_eligibility(&self) -> ComposedWorkerRouteEligibility {
        self.route_eligibility
    }
    /// The selector's own recursive source position, echoed back after the
    /// worker's `sibling_position` was checked to equal it.
    pub(in crate::cranelift_backend) fn recursive_position(&self) -> u32 {
        self.recursive_position
    }
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8b` — one planner-issued composed-call
/// target.**
///
/// ## What it is
///
/// The callee a composed eliminator frame will call at one exact `D8a`
/// selector, with the complete provenance needed to call it: closure
/// occurrence, raw body, declared arity, ordered capture provenance, and route
/// eligibility, all carried as the whole [`ComposedWorkerView`] rather than as
/// a bare origin beside it.
///
/// ⛔ **It is a representation, not a route decision and not a population
/// claim.** It does not say which of the two callees is taken — the view
/// carries route eligibility and `D8c` owns consumption. It mints no
/// `EmittableCallEdge`, forces no declaration, demands no `Function`, and
/// asserts nothing about the executable population.
///
/// ⭐ **This replaces `D7a2`'s raw-target requirement, and the difference is the
/// whole point.** That object was a *demand on the population* — "this raw body
/// must be a declared-and-defined `Function`" — and it was withdrawn because
/// honouring it re-opened the permanently-refused raw closure route. This one
/// makes no such demand. The shape survives because the shape was right: one per
/// exact selector, unconstructible outside planning, whole view, derived from an
/// interned specialization fact.
///
/// ## Which side of the split each fact sits on
///
/// | fact | side |
/// |---|---|
/// | the selector resolves to exactly one worker | **unreconciled** — [`StaticTransitionPlan::composed_worker_view_unreconciled`] |
/// | provenance re-checks: position, body child, ordered captures | **unreconciled** |
/// | selector agreement, the law this object is validated by | **unreconciled** |
/// | is the callee reachable as an emitted `Function` | **reconciled** — `D8c` owns it, and nothing here asks it |
///
/// ⛔ Stating this is not decoration. `D7a2` derived its requirements from a
/// population question that the requirements themselves decided, and the split
/// exists so that cannot recur: a target is minted from resolution alone, so no
/// later executability answer can depend on an earlier one that assumed it.
///
/// ⛔ **Unconstructible.** Every field is private, there is no constructor
/// outside this module, and the only way to obtain one is
/// [`StaticTransitionPlan::composed_call_targets`].
///
/// ⛔ **There is no body accessor.** The callee is read as
/// `target.worker().body_origin()`. A `body_origin()` on this type would be a
/// second spelling of one field, and the check that compared the two was
/// deleted in `D7a2` for comparing a value with itself.
///
/// ## `D8h` — the paired causal identity
///
/// The target additionally carries the exact opaque [`ContinuationCallIdentity`]
/// that its **own** five-field causal coordinate selects, resolved through
/// [`StaticTransitionPlan::continuation_call_binding_for`] — the planner lookup
/// that already existed for this coordinate — and never rebuilt here.
///
/// ⛔ **The identity stays opaque and planner-owned.** It has no sequence
/// accessor and no lowering constructor, so pairing it here adds no way to
/// fabricate one: this type hands out a value it could not have made. Nothing
/// in the pairing consults the worker's body, the constructor's symbol, the
/// declared arity, the source position, or a same-shaped constructor. On the
/// witness population **constructor-symbol equality cannot discriminate at
/// all** — both targets name one symbol — which is what makes that exclusion a
/// measured fact rather than a stated intention.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct ComposedCallTarget {
    pub(super) emission_owner: ContinuationEmissionOwner,
    pub(super) producer_construct_origin: StaticOriginId,
    pub(super) continuation_origin: StaticOriginId,
    pub(super) producer_alternative: u32,
    pub(super) recursive_position: u32,
    pub(super) worker: ComposedWorkerView,
    pub(super) call_identity: ContinuationCallIdentity,
}

// `D8i` is the production consumer: `composed_recursive_argument_binding` reads
// `selector()`, `worker()` and `call_identity()` to build and to authorize the
// composed binding. ⛔ The dead-code allowance that stood here is DELETED rather
// than narrowed -- it existed while nothing outside tests read this type, and
// keeping it now would hide the next accessor that ships unread.
impl ComposedCallTarget {
    /// The exact `D8a` five-field selector this target was minted under.
    pub(in crate::cranelift_backend) fn selector(
        &self,
    ) -> (
        ContinuationEmissionOwner,
        StaticOriginId,
        StaticOriginId,
        u32,
        u32,
    ) {
        (
            self.emission_owner,
            self.producer_construct_origin,
            self.continuation_origin,
            self.producer_alternative,
            self.recursive_position,
        )
    }
    /// The full worker provenance — the callee, and everything needed to call
    /// it. ⛔ The only route to the body origin, deliberately.
    pub(in crate::cranelift_backend) fn worker(&self) -> &ComposedWorkerView {
        &self.worker
    }
    /// **`D8h`** — the opaque causal identity this target's own coordinate
    /// selects.
    ///
    /// ⛔ Returned by reference and still opaque: a consumer can compare it,
    /// key a map on it, and ask it for its target specialization and emission
    /// owner. It cannot read the call-site sequence, and it cannot construct
    /// one — which is what keeps the identity planner-owned across the pairing.
    pub(in crate::cranelift_backend) fn call_identity(&self) -> &ContinuationCallIdentity {
        &self.call_identity
    }
}

/// `D8b` target-minting defects, for the controls that no well-formed plan can
/// reach on its own.
///
/// ⛔ `#[cfg(test)]`. Targets are derived from interned planner facts, so a
/// wrong body or a transplanted construct origin is unreachable through any plan
/// the planner will build — which is exactly why the law that catches them would
/// otherwise ship unexercised.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum ComposedCallTargetDefect {
    Exact,
    /// Mint the first target carrying a body that is not its own worker's.
    WrongBody,
    /// Mint the first target under a sibling layer's construct origin while
    /// keeping its own worker provenance.
    TransplantConstruct,
    /// **`D8h`** — pair the first target with the causal identity belonging to
    /// a **different** target whose producer constructor carries the **same
    /// symbol identity**.
    ///
    /// ⛔ This is not a fabricated identity and not an arbitrary swap: it is
    /// exactly the value a pairing rule keyed on constructor-symbol equality
    /// would have produced, taken from the real population by searching for
    /// that equality. So the refusal below is attributable to the pairing rule
    /// the release forbids, rather than to the identity being wrong in some
    /// unrelated way. The selector and the carried worker are untouched, so
    /// selector agreement still passes and only the pairing law can see it.
    ///
    /// ⚠ If no same-symbol sibling exists on the plan under test this leaves
    /// the population exact, and the row that arms it asserts the sibling's
    /// existence separately — so a population that cannot exhibit the defect
    /// fails loudly instead of passing vacuously.
    SameSymbolIdentity,
}

#[cfg(test)]
thread_local! {
    pub(super) static COMPOSED_CALL_TARGET_DEFECT: Cell<ComposedCallTargetDefect> =
        const { Cell::new(ComposedCallTargetDefect::Exact) };
}

/// Arm one `D8b` target-minting defect for the current thread.
#[cfg(test)]
pub(in crate::cranelift_backend) fn set_composed_call_target_defect(
    defect: ComposedCallTargetDefect,
) {
    COMPOSED_CALL_TARGET_DEFECT.with(|cell| cell.set(defect));
}

/// Re-expose one immutable input projection as a view.
///
/// ⭐ One constructor for both the specialization and the generated-context
/// callers: the two populations hold the *same* projection records (a context's
/// captures literally are its enclosing specialization's continuation inputs),
/// so a second hand-written copy would be the place the two drift apart.
pub(super) fn continuation_input_view(
    projection: &ContinuationInputProjection,
    finalized: Option<&ContinuationAvailabilityViews>,
) -> Result<ContinuationInputView, CraneliftBackendError> {
    // ⛔⛔ **THE PUBLICATION GATE.** This is the single conversion both
    // populations pass through, so it is the one place that can guarantee no
    // consumer ever holds a draft. A missing entry means stage 2 did not run for
    // this record, and the honest answer is a refusal -- never the draft, which
    // would be a half-stamped claim, and never a default frame, which would be
    // an invented one.
    let availability = *finalized.ok_or_else(|| {
        planner_error(
            "a continuation input has no finalized availability, so its generated frame \
             requirement was never resolved to an exact context identity; \
             RT-CONTSRC-PRODUCER-LOCAL D3b refuses to publish an unfinalized claim",
        )
    })?;
    Ok(ContinuationInputView {
        ordinal: projection.ordinal,
        coordinate: projection.coordinate,
        carrier: projection.carrier,
        ownership: projection.ownership,
        storage_owner: projection.storage_owner,
        referent_affinity: projection.referent_affinity.clone(),
        ordinary_abi_position: projection.ordinary_abi_position,
        availability,
    })
}

/// A read-only view of one already-validated continuation input.
///
/// Every field is existing immutable `ContinuationInputProjection` material,
/// re-exposed rather than re-derived.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct ContinuationInputView {
    pub(in crate::cranelift_backend) ordinal: u32,
    /// `D1` — the closed coordinate domain naming this value in its producer.
    /// ⛔ The emission seam must **match** on it; there is no field here that
    /// answers "which ABI position" without first answering "which domain".
    pub(in crate::cranelift_backend) coordinate: ContinuationSourceCoordinate,
    pub(in crate::cranelift_backend) carrier: AbiCarrier,
    pub(in crate::cranelift_backend) ownership: AbiOwnership,
    pub(in crate::cranelift_backend) storage_owner: AbiStorageOwner,
    pub(in crate::cranelift_backend) referent_affinity: Vec<BoundaryReferentOwner>,
    pub(in crate::cranelift_backend) ordinary_abi_position: u32,
    /// **`D3b` re-cut** — the two consumer-specific claims. See
    /// [`ContinuationAvailabilityViews`]. ⛔ Each consumer must take its OWN
    /// field and validate the frame or seat it names against the environment it
    /// actually holds; there is no field here that answers "which index"
    /// without first answering "which environment, and whose".
    pub(in crate::cranelift_backend) availability: ContinuationAvailabilityViews,
}

impl ContinuationInputView {
    /// **The complete requested source slot `S` this input names.**
    ///
    /// ⭐ One derivation, on the view, because the alias rule is stated over the
    /// WHOLE record and a consumer that rebuilt it field-by-field at the seam
    /// would be a second definition able to drift from the planner's. The
    /// eligibility test in [`nearest_exact_alias`] is exact equality against
    /// this, so a field omitted here would silently widen it.
    pub(in crate::cranelift_backend) fn requested_source_slot(
        &self,
    ) -> ContinuationSourceSlotAuthority {
        ContinuationSourceSlotAuthority {
            coordinate: self.coordinate,
            carrier: self.carrier,
            ownership: self.ownership,
            storage_owner: self.storage_owner,
            referent_affinity: self.referent_affinity.clone(),
        }
    }
}

/// `D1` — a read-only view of one already-validated continuation call token,
/// carrying the full producer tuple and the exact target.
///
/// `continuation_origin` and `recursive_position` are read from the **resolved
/// target's key**, never from the token; the token owns the sequence.
pub(in crate::cranelift_backend) struct ContinuationCallView<'plan> {
    pub(super) token: &'plan ContinuationSpecializationCallToken,
    pub(super) continuation_origin: StaticOriginId,
    pub(super) recursive_position: u32,
    pub(super) target: ContinuationSpecializationId,
}

impl ContinuationCallView<'_> {
    pub(in crate::cranelift_backend) fn producer_owner(&self) -> PredeclaredFunctionId {
        self.token.producer_owner
    }
    /// **`D5a`** — the immediate emission and availability owner.
    pub(in crate::cranelift_backend) fn emission_owner(&self) -> ContinuationEmissionOwner {
        self.token.emission_owner
    }
    pub(in crate::cranelift_backend) fn producer_result_origin(&self) -> StaticOriginId {
        self.token.producer_result_origin
    }
    pub(in crate::cranelift_backend) fn producer_construct_origin(&self) -> StaticOriginId {
        self.token.producer_construct_origin
    }
    pub(in crate::cranelift_backend) fn producer_alternative(&self) -> u32 {
        self.token.producer_alternative
    }
    pub(in crate::cranelift_backend) fn continuation_origin(&self) -> StaticOriginId {
        self.continuation_origin
    }
    pub(in crate::cranelift_backend) fn recursive_position(&self) -> u32 {
        self.recursive_position
    }
    pub(in crate::cranelift_backend) fn target(&self) -> ContinuationSpecializationId {
        self.target
    }
}

/// **`RT-DECL-CLOSURE-PORT` `D5a`** — one already-issued causal call, projected
/// onto the exact result edge it belongs to.
///
/// The three edge fields are the ruled key
/// `(producer_owner, producer_result_origin, producer_construct_origin)`; the
/// owner is implicit in how the projection is requested, so it cannot disagree
/// with the unit asking. `recursive_position` rides along because the detached
/// seat has to omit exactly that field of the planned constructor and has no
/// other lawful way to learn which one it is.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct ContinuationResultEdge {
    pub(in crate::cranelift_backend) producer_result_origin: StaticOriginId,
    pub(in crate::cranelift_backend) producer_construct_origin: StaticOriginId,
    pub(in crate::cranelift_backend) recursive_position: u32,
    pub(in crate::cranelift_backend) identity: ContinuationCallIdentity,
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PlannedContinuationSpecialization {
    pub(super) id: ContinuationSpecializationId,
    pub(super) key: ContinuationSpecializationKey,
    /// **Stage 2** — the finalized availability of each continuation input, in
    /// ordinal order. ⛔ Deliberately a SIBLING of `key`, never inside it:
    /// `key` is the interning identity, and stamping a resolved context id into
    /// it after interning would rewrite the identity every dedup decision was
    /// already made against.
    pub(super) finalized_availability: Vec<ContinuationAvailabilityViews>,
}

/// Exact causal identity for one direct producer edge into an interned target.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct ContinuationSpecializationCallToken {
    /// ⚠ **`D5a`: PROVENANCE ONLY** — the raw source-occurrence owner. Kept
    /// because it is still the right answer to "where is this producer written";
    /// it is no longer the answer to "who emits this call".
    pub(super) producer_owner: PredeclaredFunctionId,
    /// **`D5a` — who emits this call and holds its operands.**
    pub(super) emission_owner: ContinuationEmissionOwner,
    pub(super) producer_result_origin: StaticOriginId,
    pub(super) producer_construct_origin: StaticOriginId,
    pub(super) producer_alternative: u32,
    pub(super) call_site_sequence: u32,
    pub(super) target: ContinuationSpecializationId,
    pub(super) worker: ContinuationWorkerProvenance,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct PlannedContinuationSpecializationCall {
    pub(super) token: ContinuationSpecializationCallToken,
}


/// The exact result-position population below `root`, bounded to its source
/// owner. Ordinary `Match` branches are selected only through Slice 0's D1
/// verdicts; owner and lifetime checks come only from Slice 0's D2 population.

/// **`RT-CAPTURE-PROJECTION-GROW` `D1` — the binder depth from a producer result
/// root down to one of its result origins.**
///
/// ⭐ The recursive-position worker closure is a DIRECT CHILD of the producer
/// `Construct`, so no binder separates the two. The binders that matter sit
/// **above** the construct — the producer `Match` arm whose body it is — which
/// is why a walk from the closure to the construct is structurally always zero
/// and the depth has to be measured on this path instead.
///
/// ⛔ The descent mirrors [`continuation_result_origins`]' child threading and
/// the binder arities mirror `shift_runtime_vars`, so no binding form is crossed
/// silently. A closure is a LEAF here: the walk never descends through one, so
/// a construct inside a closure body returns `None` rather than a depth that
/// ignored the closure's own binders.
///
/// ⛔ `None` means "not found on a binder-bearing path" and callers must treat
/// it as *do not join*, never as depth zero — joining at a wrong-but-small depth
/// inflates the demand and can veto the whole continuation.
fn producer_binder_depth(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
    target: StaticOriginId,
    depth: usize,
) -> Result<Option<usize>, CraneliftBackendError> {
    if origin == target {
        return Ok(Some(depth));
    }
    let expr = plan.planned_occurrence_expr(origin)?;
    let child = |position| plan.semantic.child_origin(origin, position);
    match expr {
        RuntimeExpr::CheckedJoinSite { .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { .. }
        | RuntimeExpr::CheckedRecursiveInvocation { .. }
        | RuntimeExpr::CheckedComputationalIHSlots { .. }
        | RuntimeExpr::CheckedComputationalIHInvocation { .. } => {
            producer_binder_depth(plan, child(0)?, target, depth)
        }
        RuntimeExpr::Let { .. } => producer_binder_depth(plan, child(1)?, target, depth + 1),
        RuntimeExpr::If { .. } => {
            if let Some(found) = producer_binder_depth(plan, child(1)?, target, depth)? {
                return Ok(Some(found));
            }
            producer_binder_depth(plan, child(2)?, target, depth)
        }
        RuntimeExpr::Match { cases, .. } => {
            for (index, case) in cases.iter().enumerate() {
                if let Some(found) =
                    producer_binder_depth(plan, child(1 + index)?, target, depth + case.binders)?
                {
                    return Ok(Some(found));
                }
            }
            Ok(None)
        }
        RuntimeExpr::ComputationalMatch { cases, .. } => {
            for (index, case) in cases.iter().enumerate() {
                let binders = case
                    .argument_binders
                    .checked_add(case.recursive_positions.len())
                    .ok_or_else(|| planner_capacity_error("producer binder depth exhausted"))?;
                if let Some(found) =
                    producer_binder_depth(plan, child(1 + index)?, target, depth + binders)?
                {
                    return Ok(Some(found));
                }
            }
            Ok(None)
        }
        RuntimeExpr::Value(_)
        | RuntimeExpr::Var(_)
        | RuntimeExpr::PrimitiveCall { .. }
        | RuntimeExpr::Construct { .. }
        | RuntimeExpr::Record { .. }
        | RuntimeExpr::Project { .. }
        | RuntimeExpr::Closure { .. }
        | RuntimeExpr::LexicalClosure { .. }
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. }
        | RuntimeExpr::Call { .. }
        | RuntimeExpr::Effect { .. }
        | RuntimeExpr::Trap(_) => Ok(None),
    }
}


/// **`RT-CAPTURE-PROJECTION-GROW` `D1` — one edge whose worker prefix was NOT
/// joined, with the numbers that explain why.**
///
/// A deferral is a `D2` handoff, not a silent drop: the worker references values
/// outside the continuation's entry environment, which is the producer-local
/// population the entry-frame widening exists to seat.
#[cfg(any(test, feature = "px8-ds-test-support"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkerPrefixDeferral {
    /// ⚠ Crate-visible, not `pub`: `StaticOriginId` is private to this crate, so
    /// a `pub` field of that type cannot be read by a consumer at all. Consumers
    /// identify the edge through [`WorkerPrefixDeferral::edge`], which commits
    /// them to no id value.
    pub(crate) producer_construct_origin: StaticOriginId,
    pub depth: usize,
    pub demand: usize,
    pub reached: usize,
}

#[cfg(any(test, feature = "px8-ds-test-support"))]
impl WorkerPrefixDeferral {
    /// The deferred edge's identity, as an opaque string.
    ///
    /// ⭐ Deliberately opaque. It is enough to ask whether two rows name the
    /// same edge, which is the only question a consumer has, and it commits no
    /// caller to a planner-assigned ordinal that legitimately moves when an
    /// unrelated binding renumbers the occurrence table.
    pub fn edge(&self) -> String {
        format!("{:?}", self.producer_construct_origin)
    }
}

#[cfg(any(test, feature = "px8-ds-test-support"))]
thread_local! {
    static WORKER_PREFIX_DEFERRALS: std::cell::RefCell<Option<Vec<WorkerPrefixDeferral>>> =
        const { std::cell::RefCell::new(None) };
}

/// Run `body` while recording every deferred worker prefix on this thread.
///
/// Hidden and default-off: inert unless a scope installs the ledger, and it does
/// not affect the projection it observes.
#[cfg(any(test, feature = "px8-ds-test-support"))]
#[doc(hidden)]
pub fn with_worker_prefix_deferrals<R>(body: impl FnOnce() -> R) -> (R, Vec<WorkerPrefixDeferral>) {
    struct Restore(Option<Vec<WorkerPrefixDeferral>>);
    impl Drop for Restore {
        fn drop(&mut self) {
            WORKER_PREFIX_DEFERRALS.with(|cell| *cell.borrow_mut() = self.0.take());
        }
    }
    let previous = WORKER_PREFIX_DEFERRALS.with(|cell| cell.borrow_mut().replace(Vec::new()));
    let restore = Restore(previous);
    let value = body();
    let rows = WORKER_PREFIX_DEFERRALS
        .with(|cell| cell.borrow_mut().take())
        .unwrap_or_default();
    drop(restore);
    (value, rows)
}

#[cfg(any(test, feature = "px8-ds-test-support"))]
fn record_worker_prefix_deferral(row: WorkerPrefixDeferral) {
    WORKER_PREFIX_DEFERRALS.with(|cell| {
        if let Some(rows) = cell.borrow_mut().as_mut() {
            rows.push(row);
        }
    });
}

pub(super) fn continuation_result_origins(
    plan: &StaticTransitionPlan<'_>,
    root: StaticOriginId,
) -> Result<BTreeSet<StaticOriginId>, CraneliftBackendError> {
    let owner = occurrence_authority(plan, root)?.owner;
    let mut pending = vec![root];
    let mut results = BTreeSet::new();
    while let Some(origin) = pending.pop() {
        if results.contains(&origin) {
            continue;
        }
        let authority = occurrence_authority(plan, origin)?;
        if authority.owner != owner {
            continue;
        }
        results.insert(origin);
        let expr = plan.planned_occurrence_expr(origin)?;
        let child = |position| plan.semantic.child_origin(origin, position);
        match expr {
            RuntimeExpr::CheckedJoinSite { .. }
            | RuntimeExpr::CheckedSubcontinuationFrame { .. }
            | RuntimeExpr::CheckedRecursiveInvocation { .. }
            | RuntimeExpr::CheckedComputationalIHSlots { .. }
            | RuntimeExpr::CheckedComputationalIHInvocation { .. } => {
                pending.push(child(0)?);
            }
            RuntimeExpr::Let { .. } => pending.push(child(1)?),
            RuntimeExpr::If { .. } => {
                pending.push(child(1)?);
                pending.push(child(2)?);
            }
            RuntimeExpr::Match { cases, .. } => {
                let records = plan
                    .case_emissions
                    .iter()
                    .filter(|record| record.match_origin == origin)
                    .collect::<Vec<_>>();
                if records.len() != cases.len() {
                    return Err(planner_error(
                        "continuation result flow has no exact D1 case population",
                    ));
                }
                for record in records {
                    if record.status == CaseEmissionStatus::Reachable {
                        pending.push(record.body_origin);
                    }
                }
            }
            RuntimeExpr::ComputationalMatch { cases, .. } => {
                for index in 0..cases.len() {
                    pending.push(child(1 + index)?);
                }
            }
            RuntimeExpr::Value(_)
            | RuntimeExpr::Var(_)
            | RuntimeExpr::PrimitiveCall { .. }
            | RuntimeExpr::Construct { .. }
            | RuntimeExpr::Record { .. }
            | RuntimeExpr::Project { .. }
            | RuntimeExpr::Closure { .. }
            | RuntimeExpr::LexicalClosure { .. }
            | RuntimeExpr::DeclarationRef { .. }
            | RuntimeExpr::ImportedDeclarationRef { .. }
            | RuntimeExpr::Call { .. }
            | RuntimeExpr::Effect { .. }
            | RuntimeExpr::Trap(_) => {}
        }
    }
    Ok(results)
}

pub(super) fn build_continuation_worker_provenance(
    plan: &StaticTransitionPlan<'_>,
    parent_origin: StaticOriginId,
    producer_origin: StaticOriginId,
    sibling_position: usize,
    closure_origin: StaticOriginId,
) -> Result<ContinuationWorkerProvenance, CraneliftBackendError> {
    let closure_authority = occurrence_authority(plan, closure_origin)?;
    let closure = plan.planned_occurrence_expr(closure_origin)?;
    let body = closure_authority
        .children
        .iter()
        .find(|child| child.position == 0)
        .ok_or_else(|| planner_error("continuation worker closure has no body authority"))?;
    let (parameters, captures) = match closure {
        RuntimeExpr::Closure {
            captures, params, ..
        } => {
            let captures = captures
                .iter()
                .enumerate()
                .map(|(ordinal, _)| {
                    Ok(ContinuationWorkerCaptureProvenance {
                        ordinal: u32::try_from(ordinal).map_err(|_| {
                            planner_capacity_error("continuation worker capture exhausted")
                        })?,
                        owner: closure_authority.owner,
                        closure_origin,
                        source: ContinuationWorkerCaptureSource::Seed,
                        lifetime: PlannedReferentLifetime::Persistent,
                    })
                })
                .collect::<Result<Vec<_>, CraneliftBackendError>>()?;
            (params.len(), captures)
        }
        RuntimeExpr::LexicalClosure {
            captures, params, ..
        } => {
            let captures = captures
                .iter()
                .enumerate()
                .map(|(ordinal, _)| {
                    let position = u32::try_from(1 + ordinal).map_err(|_| {
                        planner_capacity_error("continuation worker capture position exhausted")
                    })?;
                    let source = closure_authority
                        .children
                        .iter()
                        .find(|child| child.position == position)
                        .ok_or_else(|| {
                            planner_error(
                                "continuation worker capture has no exact D2 authority",
                            )
                        })?;
                    Ok(ContinuationWorkerCaptureProvenance {
                        ordinal: u32::try_from(ordinal).map_err(|_| {
                            planner_capacity_error("continuation worker capture exhausted")
                        })?,
                        owner: source.owner,
                        closure_origin,
                        source: ContinuationWorkerCaptureSource::Lexical(source.origin),
                        lifetime: source.lifetime,
                    })
                })
                .collect::<Result<Vec<_>, CraneliftBackendError>>()?;
            (params.len(), captures)
        }
        _ => {
            return Err(planner_error(
                "continuation worker provenance names a non-closure occurrence",
            ));
        }
    };
    Ok(ContinuationWorkerProvenance {
        parent_origin,
        producer_origin,
        sibling_position: u32::try_from(sibling_position)
            .map_err(|_| planner_capacity_error("continuation worker position exhausted"))?,
        closure_origin,
        body_origin: body.origin,
        declared_arity: u32::try_from(parameters)
            .map_err(|_| planner_capacity_error("continuation worker arity exhausted"))?,
        captures,
    })
}

pub(super) fn slot_referent_affinity(
    carrier: AbiCarrier,
) -> Result<Vec<BoundaryReferentOwner>, CraneliftBackendError> {
    match carrier {
        AbiCarrier::GroundValueCarrier => Ok(vec![
            BoundaryReferentOwner::NoReferent,
            BoundaryReferentOwner::PersistentStore,
        ]),
        AbiCarrier::ValueWord => Ok(vec![
            BoundaryReferentOwner::NoReferent,
            BoundaryReferentOwner::PersistentStore,
            BoundaryReferentOwner::InvocationArena,
        ]),
        AbiCarrier::ResultWord
        | AbiCarrier::ControlWord
        | AbiCarrier::TrapWord
        | AbiCarrier::StoreHandle => Err(planner_error(
            "continuation source environment names a convention slot",
        )),
    }
}


pub(super) fn continuation_owner_entry_sources(
    plan: &StaticTransitionPlan<'_>,
    owner: PredeclaredFunctionId,
) -> Result<Vec<ContinuationSourceSlotAuthority>, CraneliftBackendError> {
    let descriptor = plan
        .abi
        .descriptors
        .iter()
        .find(|descriptor| descriptor.function == owner)
        .ok_or_else(|| planner_error("continuation source owner has no ABI descriptor"))?;
    let input_count = descriptor
        .header
        .parameters
        .checked_add(descriptor.header.captures)
        .ok_or_else(|| planner_capacity_error("continuation input population exhausted"))?;
    let start = usize::try_from(descriptor.slots.start)
        .map_err(|_| planner_capacity_error("continuation ABI range exhausted"))?;
    let end = start
        .checked_add(
            usize::try_from(input_count)
                .map_err(|_| planner_capacity_error("continuation ABI range exhausted"))?,
        )
        .ok_or_else(|| planner_capacity_error("continuation ABI range exhausted"))?;
    let slots = plan
        .abi
        .slots
        .get(start..end)
        .ok_or_else(|| planner_error("continuation input range is outside the ABI plane"))?;
    let mut sources = Vec::with_capacity(slots.len());
    for slot in slots {
        let (source_abi_position, source, referent_affinity) = match slot.kind {
            AbiSlotKind::Parameter => (
                slot.ordinal,
                ContinuationInputSource::Parameter,
                slot_referent_affinity(slot.carrier)?,
            ),
            AbiSlotKind::Capture => {
                let source_abi_position = descriptor
                    .header
                    .parameters
                    .checked_add(slot.ordinal)
                    .ok_or_else(|| {
                        planner_capacity_error("continuation capture ABI position exhausted")
                    })?;
                // ⭐ `RT-DECL-CLOSURE-PORT` `D2`: `CallableDeclaration` is
                // matched **beside** `ClosureBody`, with the identical sourcing,
                // because it occupies the identical graph position — same
                // defining closure occurrence, same `[body, captures..]` child
                // layout. Before `D2` these very nodes WERE `ClosureBody`, so
                // anything else here (an error arm, a fresh derivation) would be
                // a behaviour change smuggled in under a reclassification.
                match descriptor.definition {
                    AbiUnitDefinition::ClosureBody {
                        defining_origin,
                        provenance: AbiCaptureProvenance::Lexical,
                    }
                    | AbiUnitDefinition::CallableDeclaration {
                        declaration_origin: defining_origin,
                        provenance: AbiCaptureProvenance::Lexical,
                    } => {
                        let defining = occurrence_authority(plan, defining_origin)?;
                        let child_position = slot.ordinal.checked_add(1).ok_or_else(|| {
                            planner_capacity_error(
                                "continuation capture source position exhausted",
                            )
                        })?;
                        let child = defining
                            .children
                            .iter()
                            .find(|child| child.position == child_position)
                            .ok_or_else(|| {
                                planner_error(
                                    "lexical continuation capture has no D2 source",
                                )
                            })?;
                        (
                            source_abi_position,
                            ContinuationInputSource::LexicalCapture {
                                source_origin: child.origin,
                            },
                            lifetime_referent_affinity(child.lifetime),
                        )
                    }
                    AbiUnitDefinition::ClosureBody {
                        defining_origin,
                        provenance: AbiCaptureProvenance::Seed,
                    }
                    | AbiUnitDefinition::CallableDeclaration {
                        declaration_origin: defining_origin,
                        provenance: AbiCaptureProvenance::Seed,
                    } => {
                        occurrence_authority(plan, defining_origin)?;
                        (
                            source_abi_position,
                            ContinuationInputSource::SeedCapture { defining_origin },
                            lifetime_referent_affinity(
                                PlannedReferentLifetime::Persistent,
                            ),
                        )
                    }
                    AbiUnitDefinition::SchedulingEntry { .. } => {
                        return Err(planner_error(
                            "scheduling entry declares a continuation capture",
                        ));
                    }
                    AbiUnitDefinition::ContinuationSpecialization { .. } => {
                        return Err(planner_error(
                            "a dormant continuation specialization cannot source another planner environment",
                        ));
                    }
                    // `D2f`: this arm asks a fusion region to be the SOURCE of
                    // another unit's continuation environment. That is the lane
                    // the ruling's stop condition names — a fused region's
                    // locals are its activation, and sourcing them out would be
                    // activation state crossing the descriptor boundary. It
                    // refuses on the class, not on the absence of an emitter.
                    AbiUnitDefinition::StaticContinuationFusion { .. } => {
                        return Err(planner_error(
                            "a static continuation fusion cannot source another planner \
                             environment; its region keeps its activation local",
                        ));
                    }
                }
            }
            AbiSlotKind::Result
            | AbiSlotKind::Control
            | AbiSlotKind::Trap
            | AbiSlotKind::Store => {
                return Err(planner_error(
                    "continuation source environment names a convention slot",
                ));
            }
        };
        sources.push(ContinuationSourceSlotAuthority {
            // ⭐ This function enumerates the ENTRY ABI input run and nothing
            // else, so it produces exactly the `EntryAbi` arm. That is not an
            // omission: a value with no entry position is by construction not
            // in the population this function walks.
            coordinate: ContinuationSourceCoordinate::EntryAbi {
                source_owner: owner,
                source_abi_position,
                source,
            },
            carrier: slot.carrier,
            ownership: slot.ownership,
            storage_owner: slot.storage_owner,
            referent_affinity,
        });
    }
    let entry_position = |source: &ContinuationSourceSlotAuthority| match source.coordinate {
        ContinuationSourceCoordinate::EntryAbi {
            source_abi_position, ..
        } => source_abi_position,
        // ⛔ Unreachable by construction — every push above is `EntryAbi` — and
        // deliberately mapped to a position no exact run can hold, so a future
        // arm smuggled into this walk fails the exactness check below instead
        // of sorting silently into it.
        ContinuationSourceCoordinate::ProducerLocal { .. } => u32::MAX,
    };
    sources.sort_by_key(entry_position);
    if sources.len() != input_count as usize
        || sources
            .iter()
            .enumerate()
            .any(|(position, source)| entry_position(source) as usize != position)
    {
        return Err(planner_error(
            "continuation owner entry environment is not exact for its ABI slots",
        ));
    }
    Ok(sources)
}

/// `RT-CONTSRC-PRODUCER-LOCAL` `D2` — which producer-local binding kind a
/// coordinate names.
///
/// ⛔ **Not part of the coordinate's identity.** `binding_origin` +
/// `binding_ordinal` already distinguish every binding, including two of
/// different kinds, so adding a kind tag to the representation would be a
/// second statement of a fact the identity already carries. This selects the
/// two facts whose *derivation* differs, and nothing else.
///
/// ⛔ Closed, no default arm: a third binding kind must state its carrier and
/// its referent lifetime explicitly rather than inherit either.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProducerLocalKind {
    /// The result of a host effect. The `Effect` occurrence owns the identity,
    /// and its own occurrence authority owns the referent lifetime.
    HostEffectResult,
    /// One **constructor argument** binder of one `Match` /
    /// `ComputationalMatch` case. Its representation and referent both come
    /// from the scrutinee, named here by the match occurrence whose child 0 it
    /// is.
    ///
    /// ⛔ **Not "any case binder".** A `ComputationalMatch` case environment is
    /// `[recursive IH binders, constructor argument binders, outer
    /// environment]`, and the two runs are **not homogeneous** —
    /// `derive_occurrence_lifetime` gives every IH `ActivationOwned` while
    /// argument binders take the scrutinee's, and the result-phase pass gives
    /// IHs a declared-unit-result contract while argument binders preserve the
    /// scrutinee's representation. `a5a6ce9b` looped over the combined count
    /// and stamped one contract across both, which silently misclassified the
    /// IH prefix; the Architect blocked it at `evt_9krmbv834z9p`. There is
    /// deliberately no `RecursiveIhBinder` arm here — see the walk.
    CaseArgumentBinder { match_origin: StaticOriginId },
}

/// `RT-CONTSRC-PRODUCER-LOCAL` `D2` — the planner contract for one
/// producer-local binding.
///
/// ⭐ **One authority for both kinds.** Only the carrier and the referent
/// lifetime are derived per kind; ownership and storage owner are read off
/// `AbiCarrier`'s existing methods — the same two the entry plane's `abi::slot`
/// reads — so this record cannot disagree with the entry plane about what a
/// carrier implies. ⛔ Nothing here restates a fact another record already
/// holds.
///
/// **Where each fact comes from, stated because "planner-derived" is not an
/// answer:**
///
/// | fact | host-effect result | constructor **argument** binder |
/// |---|---|---|
/// | carrier | `abi::result_carrier` on the `Effect` shape — the existing "carrier an occurrence's result travels in" authority, and this binding *is* that result | `abi::result_carrier` on the **scrutinee's** shape, gated by `slot_referent_affinity` |
/// | ownership | `AbiCarrier::ownership` | `AbiCarrier::ownership` |
/// | storage owner | `AbiCarrier::storage_owner` | `AbiCarrier::storage_owner` |
/// | referent affinity | the `Effect` occurrence's own lifetime authority | the scrutinee child's lifetime authority |
///
/// ⛔ **Recursive IH binders are not in this table and take no contract here.**
/// They are a separate subrun of a `ComputationalMatch` case and stay `Open`;
/// see the walk for why none can be read. ⛔ There is no blanket carrier for
/// "a case binder": a constructor argument's carrier is the **scrutinee's**,
/// read from the existing authority, and nothing else is claimed.
///
/// ⭐ The binder's referent lifetime is **not** conservatively floored, because
/// it does not have to be: `PlannedReferentLifetime::Persistent` is issued only
/// when the complete source result is closed over persistent children, so a
/// field of a persistent scrutinee is persistent by that type's own definition
/// and reading the scrutinee's lifetime is not a promotion.
fn producer_local_source(
    plan: &StaticTransitionPlan<'_>,
    kind: ProducerLocalKind,
    binding: ProducerLocalBinding,
    locator: ProducerLocalLocator,
) -> Result<ContinuationSourceSlotAuthority, CraneliftBackendError> {
    let (carrier, lifetime) = match kind {
        ProducerLocalKind::HostEffectResult => (
            abi::result_carrier(SemanticSourceKind::Expression(RuntimeExprShape::Effect))?,
            occurrence_authority(plan, binding.binding_origin)?.lifetime,
        ),
        ProducerLocalKind::CaseArgumentBinder { match_origin } => {
            let scrutinee = occurrence_authority(plan, match_origin)?
                .children
                .iter()
                .find(|child| child.position == 0)
                .ok_or_else(|| {
                    planner_error("a case argument binder names a match with no scrutinee child")
                })?;
            // ⭐ The carrier is READ, not chosen. A constructor argument binder
            // preserves the scrutinee's representation — that is the existing
            // result-phase rule, stated at the `Match` and `ComputationalMatch`
            // arms of `summarize_result_phase` — so the binder's carrier is the
            // carrier the scrutinee's result travels in, and `abi::result_carrier`
            // is the sole authority for that. ⛔ This replaces the blanket
            // `ValueWord` of `a5a6ce9b`, which was a `D2` invention.
            let seed = plan
                .semantic_sources
                .iter()
                .find(|seed| seed.origin == scrutinee.origin)
                .ok_or_else(|| {
                    planner_error("a case argument binder's scrutinee has no semantic seed")
                })?;
            let carrier = abi::result_carrier(seed.source)?;
            // ⛔ Admissibility, from the same authority that gates an entry
            // slot: a continuation source environment admits `ValueWord` and
            // `GroundValueCarrier` and refuses every convention carrier. A
            // scrutinee whose result travels in one fails closed HERE rather
            // than being silently narrowed to an ordinary value word.
            slot_referent_affinity(carrier)?;
            (carrier, scrutinee.lifetime)
        }
    };
    Ok(ContinuationSourceSlotAuthority {
        coordinate: ContinuationSourceCoordinate::ProducerLocal { binding, locator },
        carrier,
        ownership: carrier.ownership(),
        storage_owner: carrier.storage_owner(),
        referent_affinity: lifetime_referent_affinity(lifetime),
    })
}

/// `D2` — one producer-local value, as the walk's value authority.
///
/// `binding_origin` is what **creates** the value; `locator.environment_origin`
/// is the scope whose environment **contains** it. For a case binder the two
/// coincide; for a `Let`-bound effect result they deliberately do not, which is
/// the separation `D1` exists to express.
fn producer_local_value(
    plan: &StaticTransitionPlan<'_>,
    kind: ProducerLocalKind,
    binding_origin: StaticOriginId,
    binding_ordinal: u32,
    environment_origin: StaticOriginId,
    environment_index: u32,
) -> Result<ContinuationValueSourceAuthority, CraneliftBackendError> {
    let binding = ProducerLocalBinding {
        binding_owner: occurrence_authority(plan, binding_origin)?.owner,
        binding_origin,
        binding_ordinal,
    };
    let locator = ProducerLocalLocator {
        environment_origin,
        environment_index,
    };
    Ok(ContinuationValueSourceAuthority::source(
        producer_local_source(plan, kind, binding, locator)?,
    ))
}

pub(super) fn walk_continuation_value_environment(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
    target: StaticOriginId,
    environment: &[ContinuationValueSourceAuthority],
) -> Result<
    (
        ContinuationValueSourceAuthority,
        Option<Vec<ContinuationValueSourceAuthority>>,
    ),
    CraneliftBackendError,
> {
    if origin == target {
        return Ok((
            ContinuationValueSourceAuthority::Open,
            Some(environment.to_vec()),
        ));
    }
    let expr = plan.planned_occurrence_expr(origin)?;
    let child = |position| plan.semantic.child_origin(origin, position);
    let walk = |position, environment: &[ContinuationValueSourceAuthority]| {
        walk_continuation_value_environment(plan, child(position)?, target, environment)
    };
    let result = match expr {
        RuntimeExpr::CheckedJoinSite { .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { .. }
        | RuntimeExpr::CheckedRecursiveInvocation { .. }
        | RuntimeExpr::CheckedComputationalIHSlots { .. }
        | RuntimeExpr::CheckedComputationalIHInvocation { .. } => walk(0, environment)?,
        RuntimeExpr::Var(index) => (
            environment
                .get(*index as usize)
                .cloned()
                .unwrap_or(ContinuationValueSourceAuthority::Open),
            None,
        ),
        RuntimeExpr::Let { .. } => {
            let (value, found) = walk(0, environment)?;
            if found.is_some() {
                return Ok((ContinuationValueSourceAuthority::Open, found));
            }
            // `RT-CONTSRC-PRODUCER-LOCAL` `D2` — a `Let`-bound **host-effect
            // result** is a producer-local binding, not an opaque value.
            //
            // ⭐ Minted HERE rather than in the `Effect` arm, because this is
            // where the value enters an environment and therefore where it
            // acquires a locator. An effect result consumed without a binder
            // never enters one, and correctly stays `Open`: there is no
            // environment position to name.
            //
            // ⛔ The binding origin is the `Effect` occurrence — what creates
            // the value — while the locator names this `Let`'s body, whose
            // environment holds it at index 0. The two differ on purpose.
            let bound_origin = child(0)?;
            let value = match plan.planned_occurrence_expr(bound_origin)? {
                RuntimeExpr::Effect { .. } => producer_local_value(
                    plan,
                    ProducerLocalKind::HostEffectResult,
                    bound_origin,
                    0,
                    child(1)?,
                    0,
                )?,
                _ => value,
            };
            let mut nested = Vec::with_capacity(environment.len() + 1);
            nested.push(value);
            nested.extend_from_slice(environment);
            walk(1, &nested)?
        }
        RuntimeExpr::If { .. } => {
            let (_, found) = walk(0, environment)?;
            if found.is_some() {
                return Ok((ContinuationValueSourceAuthority::Open, found));
            }
            let (then_value, found) = walk(1, environment)?;
            if found.is_some() {
                return Ok((ContinuationValueSourceAuthority::Open, found));
            }
            let (else_value, found) = walk(2, environment)?;
            if found.is_some() {
                return Ok((ContinuationValueSourceAuthority::Open, found));
            }
            (then_value.join(else_value), None)
        }
        RuntimeExpr::Match { cases, .. } => {
            let (_, found) = walk(0, environment)?;
            if found.is_some() {
                return Ok((ContinuationValueSourceAuthority::Open, found));
            }
            let mut value = ContinuationValueSourceAuthority::Closed(Vec::new());
            for (index, case) in cases.iter().enumerate() {
                // `RT-CONTSRC-PRODUCER-LOCAL` `D2` — each case binder is a
                // producer-local binding. ⛔ Its identity is the case BODY's
                // occurrence plus the binder ordinal, never the match
                // occurrence plus an encoded pair: two numbers packed into one
                // is the aliasing this plane refuses everywhere else, and the
                // case body is already the exact static identity of that
                // binder's scope.
                let case_body = child(1 + index)?;
                let mut nested = Vec::with_capacity(case.binders + environment.len());
                for binder in 0..case.binders {
                    nested.push(producer_local_value(
                        plan,
                        ProducerLocalKind::CaseArgumentBinder {
                            match_origin: origin,
                        },
                        case_body,
                        u32::try_from(binder).map_err(|_| {
                            planner_capacity_error("continuation case binder ordinal exhausted")
                        })?,
                        case_body,
                        u32::try_from(binder).map_err(|_| {
                            planner_capacity_error("continuation case binder ordinal exhausted")
                        })?,
                    )?);
                }
                nested.extend_from_slice(environment);
                let (case_value, found) = walk(1 + index, &nested)?;
                if found.is_some() {
                    return Ok((ContinuationValueSourceAuthority::Open, found));
                }
                value = value.join(case_value);
            }
            (value, None)
        }
        RuntimeExpr::ComputationalMatch { cases, .. } => {
            let (_, found) = walk(0, environment)?;
            if found.is_some() {
                return Ok((ContinuationValueSourceAuthority::Open, found));
            }
            let mut value = ContinuationValueSourceAuthority::Closed(Vec::new());
            for (index, case) in cases.iter().enumerate() {
                let binders = case
                    .argument_binders
                    .checked_add(case.recursive_positions.len())
                    .ok_or_else(|| {
                        planner_capacity_error("continuation case binder count exhausted")
                    })?;
                // `D2` corrected — this case environment is
                // `[recursive IH binders, constructor argument binders, outer
                // environment]`, and the two runs are **not homogeneous**.
                // `derive_occurrence_lifetime` gives every IH `ActivationOwned`
                // and every argument binder the scrutinee's lifetime; the
                // result-phase pass gives IHs a declared-unit-result contract
                // and argument binders the scrutinee's representation.
                //
                // ⛔ The IH prefix stays `Open`. **No contract is claimed for
                // it**, because none can be read: nothing maps a `ResultPhase`
                // to an `AbiCarrier`; the IH's phase depends on
                // `functionized_units`, a whole-plan argument that is not a
                // field of `StaticTransitionPlan` and so is not edge-local; and
                // an IH is a *callable*, and the continuation-input projection
                // has no callable domain at all — its source vocabulary
                // (`AbiCarrier` / `AbiOwnership` / `AbiStorageOwner`) carries
                // values, with no representation for a static callable, so
                // there is nothing here to name an IH with. Supplying one is
                // [[RT-CONTSRC-CALLABLE-CONTRACT]]'s, as a closed sum beside
                // `ContinuationSourceSlotAuthority`. Leaving it `Open` is the
                // pre-`D2` behaviour
                // and refuses to claim what it cannot derive; a default carrier
                // here is exactly what `evt_9krmbv834z9p` forbids.
                //
                // ⭐ The ordinal still spans the whole run, so identity stays
                // `(case body, binder ordinal)` with no new tag.
                let case_body = child(1 + index)?;
                let recursive_binders = case.recursive_positions.len();
                let mut nested = Vec::with_capacity(binders + environment.len());
                for binder in 0..binders {
                    if binder < recursive_binders {
                        nested.push(ContinuationValueSourceAuthority::Open);
                        continue;
                    }
                    nested.push(producer_local_value(
                        plan,
                        ProducerLocalKind::CaseArgumentBinder {
                            match_origin: origin,
                        },
                        case_body,
                        u32::try_from(binder).map_err(|_| {
                            planner_capacity_error("continuation case binder ordinal exhausted")
                        })?,
                        case_body,
                        u32::try_from(binder).map_err(|_| {
                            planner_capacity_error("continuation case binder ordinal exhausted")
                        })?,
                    )?);
                }
                nested.extend_from_slice(environment);
                let (case_value, found) = walk(1 + index, &nested)?;
                if found.is_some() {
                    return Ok((ContinuationValueSourceAuthority::Open, found));
                }
                value = value.join(case_value);
            }
            (value, None)
        }
        RuntimeExpr::PrimitiveCall { args, .. } | RuntimeExpr::Construct { args, .. } => {
            for position in 0..args.len() {
                let (_, found) = walk(position, environment)?;
                if found.is_some() {
                    return Ok((ContinuationValueSourceAuthority::Open, found));
                }
            }
            (ContinuationValueSourceAuthority::Open, None)
        }
        RuntimeExpr::Record { fields } => {
            for position in 0..fields.len() {
                let (_, found) = walk(position, environment)?;
                if found.is_some() {
                    return Ok((ContinuationValueSourceAuthority::Open, found));
                }
            }
            (ContinuationValueSourceAuthority::Open, None)
        }
        RuntimeExpr::Project { .. } => {
            let (_, found) = walk(0, environment)?;
            (ContinuationValueSourceAuthority::Open, found)
        }
        RuntimeExpr::LexicalClosure { captures, .. } => {
            for position in 0..captures.len() {
                let (_, found) = walk(1 + position, environment)?;
                if found.is_some() {
                    return Ok((ContinuationValueSourceAuthority::Open, found));
                }
            }
            (ContinuationValueSourceAuthority::Open, None)
        }
        RuntimeExpr::Call { args, .. } => {
            let (_, found) = walk(0, environment)?;
            if found.is_some() {
                return Ok((ContinuationValueSourceAuthority::Open, found));
            }
            for position in 0..args.len() {
                let (_, found) = walk(1 + position, environment)?;
                if found.is_some() {
                    return Ok((ContinuationValueSourceAuthority::Open, found));
                }
            }
            (ContinuationValueSourceAuthority::Open, None)
        }
        RuntimeExpr::Effect {
            capability, args, ..
        } => {
            let child_count = args.len() + usize::from(capability.is_some());
            for position in 0..child_count {
                let (_, found) = walk(position, environment)?;
                if found.is_some() {
                    return Ok((ContinuationValueSourceAuthority::Open, found));
                }
            }
            (ContinuationValueSourceAuthority::Open, None)
        }
        RuntimeExpr::Value(_)
        | RuntimeExpr::Closure { .. }
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. }
        | RuntimeExpr::Trap(_) => (ContinuationValueSourceAuthority::Open, None),
    };
    Ok(result)
}

pub(super) fn validate_continuation_source_slot(
    plan: &StaticTransitionPlan<'_>,
    source: &ContinuationSourceSlotAuthority,
) -> Result<(), CraneliftBackendError> {
    // `D1` `D3a` consumer 1 of 3 — the slot's only exact validator.
    //
    // ⛔ Exhaustive over the coordinate domains with no wildcard, and **neither
    // arm is exempted**: each RE-DERIVES its own authority and compares the
    // whole record. The entry arm re-reads the owner's entry slot run; the
    // producer-local arm re-runs the very walk that mints these values. A
    // domain added later gets its own derivation here or does not compile.
    match source.coordinate {
        ContinuationSourceCoordinate::EntryAbi {
            source_owner,
            source_abi_position,
            ..
        } => {
            let expected = continuation_owner_entry_sources(plan, source_owner)?
                .into_iter()
                .find(|candidate| {
                    matches!(
                        candidate.coordinate,
                        ContinuationSourceCoordinate::EntryAbi {
                            source_abi_position: candidate_position,
                            ..
                        } if candidate_position == source_abi_position
                    )
                })
                .ok_or_else(|| {
                    planner_error("continuation value names no exact source ABI slot")
                })?;
            if expected != *source || source.referent_affinity.is_empty() {
                return Err(planner_error(
                    "continuation value disagrees with its exact source ABI provenance",
                ));
            }
        }
        ContinuationSourceCoordinate::ProducerLocal { binding, locator } => {
            // `D3a` — the producer-local re-derivation, the exact analogue of
            // the entry arm above.
            //
            // ⭐ The independent authority is the **walk itself**: re-run it
            // from this binding owner's own source root to the scope the
            // locator names, and the value it places at the locator's index
            // must be this one. That re-derives carrier, ownership, storage
            // owner and affinity from `producer_local_source`, rather than
            // trusting the fields the projection arrived carrying.
            //
            // ⛔ Rooted at `binding.binding_owner`, not at the consumer or the
            // emitting owner: a binding's scope belongs to the function whose
            // body created it, and walking from anywhere else would reach a
            // different environment and index it with this locator's number.
            let owner_root = continuation_owner_source_root(plan, binding.binding_owner)?;
            let entry = continuation_owner_entry_sources(plan, binding.binding_owner)?
                .into_iter()
                .map(ContinuationValueSourceAuthority::source)
                .collect::<Vec<_>>();
            let (_, reached) = walk_continuation_value_environment(
                plan,
                owner_root,
                locator.environment_origin,
                &entry,
            )?;
            let reached = reached.ok_or_else(|| {
                planner_error(
                    "a producer-local continuation value names a scope outside its own binding \
                     owner's source subtree, so the forward walk never reaches it and no \
                     environment holds the value it claims",
                )
            })?;
            let index = usize::try_from(locator.environment_index).map_err(|_| {
                planner_capacity_error("continuation producer-local environment index exhausted")
            })?;
            let held = reached.get(index).ok_or_else(|| {
                planner_error(
                    "a producer-local continuation value names an environment index past the end \
                     of the environment in force at its own locator scope",
                )
            })?;
            let ContinuationValueSourceAuthority::Closed(sources) = held else {
                return Err(planner_error(
                    "a producer-local continuation value's locator names an open environment \
                     position, which carries no exact source authority to agree with; this \
                     fails closed rather than accepting a position whose contents are unknown",
                ));
            };
            // ⛔ Whole-record containment, not a coordinate match. The position
            // may legitimately hold several joined sources, so membership is
            // the right relation — but the member must agree in every field,
            // which is what makes this a re-derivation rather than a lookup.
            if !sources.contains(source) || source.referent_affinity.is_empty() {
                return Err(planner_error(
                    "continuation value disagrees with its exact producer-local source provenance",
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn continuation_owner_source_root(
    plan: &StaticTransitionPlan<'_>,
    owner: PredeclaredFunctionId,
) -> Result<StaticOriginId, CraneliftBackendError> {
    let origins = plan
        .occurrence_authorities
        .iter()
        .filter(|authority| authority.owner == owner)
        .map(|authority| authority.origin)
        .collect::<BTreeSet<_>>();
    let nested = plan
        .occurrence_authorities
        .iter()
        .filter(|authority| authority.owner == owner)
        .flat_map(|authority| {
            authority
                .children
                .iter()
                .filter(move |child| child.owner == owner)
                .map(|child| child.origin)
        })
        .collect::<BTreeSet<_>>();
    let roots = origins.difference(&nested).copied().collect::<Vec<_>>();
    // ⭐⭐ `RT-DECL-CLOSURE-PORT` `D2a` — the seed's origin is the source root,
    // and the set difference is a PROXY for it.
    //
    // The two agree for every unit whose owned occurrences form one tree: the
    // un-nested root is the occurrence the seed node was planned from. `D2a` is
    // exactly where the proxy diverges. A declaration-owned callable unit now
    // owns **two** occurrence trees — the declaration occurrence (its ownership,
    // provenance and `D3` signature authority) and the closure body it actually
    // emits — and the body occurrence is a *child* of the declaration
    // occurrence, so the difference collapses to the declaration and the walk
    // starts one level above the code this unit contains.
    //
    // ⇒ Take the property, not the proxy: the root is the origin of this unit's
    // seed node. ⚠ The difference derivation is retained as a **cross-check**
    // wherever it still yields exactly one root, so its existing failure mode
    // (an owner with no single occurrence tree) is not silently deleted.
    // ⛔ Narrowed to the exact declaration-owned pair — the unit's seed carries
    // an incoming `StaticBody` edge from a scheduling entry THIS SAME unit owns.
    // That conjunction is only true of the `D2a` pair: an anonymous closure
    // body's `StaticBody` source is owned by a different unit, and every other
    // unit's seed has no incoming `StaticBody` edge at all. Applying the
    // preference any wider re-roots units whose proxy was correct.
    let seed = plan
        .semantic
        .functions
        .get(owner.0 as usize)
        .map(|function| function.planned_node);
    let declaration_owned_seed = seed.is_some_and(|seed| {
        plan.edges.iter().any(|edge| {
            edge.kind == EdgeKind::StaticBody
                && edge.to == seed
                && plan.entries.contains(&edge.from)
                // ⚠ Ownership is read off `origins` — the occurrences this
                // owner already holds — rather than by spelling
                // `SemanticOwner` here, which this file deliberately does not.
                && origins.contains(&origin_of(edge.from))
        })
    });
    if declaration_owned_seed {
        let seed = origin_of(seed.expect("checked above"));
        if origins.contains(&seed) {
            return Ok(seed);
        }
    }
    let [root] = roots.as_slice() else {
        return Err(planner_error(
            "continuation owner does not have one exact source-occurrence root",
        ));
    };
    Ok(*root)
}

/// The prefix of an enclosing value environment that one expression can
/// actually observe after accounting for binders introduced inside it.
///
/// Closure bodies are separate function owners. Creating a lexical closure
/// evaluates its capture expressions here, but its body consumes only the
/// closure's explicit capture/parameter ABI and therefore is not an enclosing
/// environment use at this continuation edge.
pub(super) fn required_surrounding_environment_prefix(
    expr: &RuntimeExpr,
    local_binders: usize,
) -> Result<usize, CraneliftBackendError> {
    let with_binders = |additional: usize| {
        local_binders
            .checked_add(additional)
            .ok_or_else(|| {
                planner_capacity_error("continuation environment binder count exhausted")
            })
    };
    let mut maximum = 0usize;
    let mut include = |required: usize| maximum = maximum.max(required);
    match expr {
        RuntimeExpr::CheckedJoinSite { body, .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { body, .. }
        | RuntimeExpr::CheckedRecursiveInvocation { body, .. }
        | RuntimeExpr::CheckedComputationalIHSlots { body, .. }
        | RuntimeExpr::CheckedComputationalIHInvocation { body, .. } => {
            include(required_surrounding_environment_prefix(
                body,
                local_binders,
            )?);
        }
        RuntimeExpr::Var(index) => {
            let index = usize::try_from(*index).map_err(|_| {
                planner_capacity_error("continuation environment variable index exhausted")
            })?;
            if let Some(outer_ordinal) = index.checked_sub(local_binders) {
                include(outer_ordinal.checked_add(1).ok_or_else(|| {
                    planner_capacity_error("continuation environment prefix exhausted")
                })?);
            }
        }
        RuntimeExpr::Let { value, body } => {
            include(required_surrounding_environment_prefix(
                value,
                local_binders,
            )?);
            include(required_surrounding_environment_prefix(
                body,
                with_binders(1)?,
            )?);
        }
        RuntimeExpr::If {
            scrutinee,
            then_expr,
            else_expr,
        } => {
            for child in [scrutinee.as_ref(), then_expr.as_ref(), else_expr.as_ref()] {
                include(required_surrounding_environment_prefix(
                    child,
                    local_binders,
                )?);
            }
        }
        RuntimeExpr::PrimitiveCall { args, .. } | RuntimeExpr::Construct { args, .. } => {
            for argument in args {
                include(required_surrounding_environment_prefix(
                    argument,
                    local_binders,
                )?);
            }
        }
        RuntimeExpr::Match {
            scrutinee, cases, ..
        } => {
            include(required_surrounding_environment_prefix(
                scrutinee,
                local_binders,
            )?);
            for case in cases {
                include(required_surrounding_environment_prefix(
                    &case.body,
                    with_binders(case.binders)?,
                )?);
            }
        }
        RuntimeExpr::ComputationalMatch {
            scrutinee, cases, ..
        } => {
            include(required_surrounding_environment_prefix(
                scrutinee,
                local_binders,
            )?);
            for case in cases {
                let binders = case
                    .argument_binders
                    .checked_add(case.recursive_positions.len())
                    .ok_or_else(|| {
                        planner_capacity_error(
                            "continuation environment case binder count exhausted",
                        )
                    })?;
                include(required_surrounding_environment_prefix(
                    &case.body,
                    with_binders(binders)?,
                )?);
            }
        }
        RuntimeExpr::Record { fields } => {
            for (_, field) in fields {
                include(required_surrounding_environment_prefix(
                    field,
                    local_binders,
                )?);
            }
        }
        RuntimeExpr::Project { record, .. } => {
            include(required_surrounding_environment_prefix(
                record,
                local_binders,
            )?);
        }
        RuntimeExpr::LexicalClosure { captures, .. } => {
            for capture in captures {
                include(required_surrounding_environment_prefix(
                    capture,
                    local_binders,
                )?);
            }
        }
        RuntimeExpr::Call { callee, args } => {
            include(required_surrounding_environment_prefix(
                callee,
                local_binders,
            )?);
            for argument in args {
                include(required_surrounding_environment_prefix(
                    argument,
                    local_binders,
                )?);
            }
        }
        RuntimeExpr::Effect {
            capability, args, ..
        } => {
            if let Some(capability) = capability {
                include(required_surrounding_environment_prefix(
                    &capability.value,
                    local_binders,
                )?);
            }
            for argument in args {
                include(required_surrounding_environment_prefix(
                    argument,
                    local_binders,
                )?);
            }
        }
        RuntimeExpr::Value(_)
        | RuntimeExpr::Closure { .. }
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. }
        | RuntimeExpr::Trap(_) => {}
    }
    Ok(maximum)
}


pub(super) fn exact_continuation_source_environment(
    plan: &StaticTransitionPlan<'_>,
    producer_owner: PredeclaredFunctionId,
    producer_result_origin: StaticOriginId,
    producer_construct_origin: StaticOriginId,
    consumer_owner: PredeclaredFunctionId,
    continuation_origin: StaticOriginId,
) -> Result<Option<ContinuationProducerEnvironment>, CraneliftBackendError> {
    if occurrence_authority(plan, producer_construct_origin)?.owner != producer_owner
        || !continuation_result_origins(plan, producer_result_origin)?
            .contains(&producer_construct_origin)
    {
        return Err(planner_error(
            "continuation producer environment is not bound to its exact result edge",
        ));
    }
    let continuation = plan.planned_occurrence_expr(continuation_origin)?;
    let RuntimeExpr::ComputationalMatch { cases, .. } = continuation else {
        return Err(planner_error(
            "continuation source environment names no exact computational continuation",
        ));
    };
    if occurrence_authority(plan, continuation_origin)?.owner != consumer_owner {
        return Err(planner_error(
            "continuation source environment names no exact computational continuation",
        ));
    }
    let entry_sources = continuation_owner_entry_sources(plan, consumer_owner)?;
    let entry_environment = entry_sources
        .iter()
        .cloned()
        .map(ContinuationValueSourceAuthority::source)
        .collect::<Vec<_>>();
    let source_root = continuation_owner_source_root(plan, consumer_owner)?;
    let (_, reached) = walk_continuation_value_environment(
        plan,
        source_root,
        continuation_origin,
        &entry_environment,
    )?;
    let reached = reached.ok_or_else(|| {
        planner_error("computational continuation is outside its source owner subtree")
    })?;
    // The exact entry authority is the intrinsic consumer-environment floor;
    // case bodies may require a longer prefix after local rebinding.
    let intrinsic_environment_floor = entry_sources.len();
    let mut required_input_count = intrinsic_environment_floor;
    for case in cases {
        let binders = case
            .argument_binders
            .checked_add(case.recursive_positions.len())
            .ok_or_else(|| {
                planner_capacity_error("continuation environment case binder count exhausted")
            })?;
        required_input_count = required_input_count.max(required_surrounding_environment_prefix(
            &case.body,
            binders,
        )?);
    }
    // ⭐⭐ `RT-CAPTURE-PROJECTION-GROW` `D1` — THE WORKER CLOSURE'S OWN PREFIX,
    // JOINED CONDITIONALLY, PER EDGE.
    //
    // The loop above ranges over the ELIMINATOR's case bodies. The
    // recursive-position worker closure is a separate node in the PRODUCER's
    // `Construct`, outside every one of those cases, so nothing above ever
    // counted its captures — measured at `<=2` against a 3-5 capture set
    // (`RT-CAPTURE-CARDINALITY-GAP` `D0`).
    //
    // ⛔ **The join is CONDITIONAL and it must be.** `required_input_count` is a
    // single max for the WHOLE continuation gating a fail-closed refusal, so
    // joining an edge whose worker demands more than `reached` supplies does not
    // just fail that edge — it vetoes the continuation and takes down edges that
    // were coverable. Measured on `px7f`: one edge fits at 4 while a sibling
    // demands 6 against `reached = 4`; an unconditional join refuses both.
    //
    // ⛔ Never force `required_input_count > reached.len()`.
    //
    // ⛔ Positions are read in the SELECTED CONSTRUCTOR'S bucket only.
    // `recursive_positions` index the ELIMINATOR case's constructor while `args`
    // belongs to the PRODUCER's construct; indexing one with the other is
    // meaningful only when they are the same constructor — the same rule
    // `RT-BRANCH-LOCAL-DECLARED-CALLABLE` `D1` established one layer up.
    // Measured: without this guard an unrelated argument was read at the
    // eliminator's position.
    let producer_construct = plan.planned_occurrence_expr(producer_construct_origin)?;
    if let RuntimeExpr::Construct {
        constructor: produced,
        args,
    } = producer_construct
    {
        if let Some(depth) =
            producer_binder_depth(plan, producer_result_origin, producer_construct_origin, 0)?
        {
            for case in cases {
                if case.constructor != *produced {
                    continue;
                }
                for position in &case.recursive_positions {
                    let Some(argument) = args.get(*position) else {
                        continue;
                    };
                    let demand = required_surrounding_environment_prefix(argument, depth)?;
                    if demand <= reached.len() {
                        required_input_count = required_input_count.max(demand);
                    } else {
                        // DEFERRED to `D2`, not dropped. The worker references
                        // values outside the continuation's entry environment —
                        // the producer-local population — which the entry-frame
                        // widening is what seats. Recorded so `D2` can assert
                        // this edge greens once that lands.
                        #[cfg(any(test, feature = "px8-ds-test-support"))]
                        record_worker_prefix_deferral(WorkerPrefixDeferral {
                            producer_construct_origin,
                            depth,
                            demand,
                            reached: reached.len(),
                        });
                    }
                }
            }
        }
    }
    #[cfg(test)]
    let required_input_count = if CONTINUATION_PRODUCTION_MUTATION.with(Cell::get)
        == ContinuationProductionMutation::DescriptorInputCountTruncation
    {
        intrinsic_environment_floor
    } else {
        required_input_count
    };
    if reached.len() < required_input_count {
        return Err(planner_error(
            "computational continuation lacks its complete semantic value environment",
        ));
    }
    // `D4b` — record this candidate's FULL required vector, and let the
    // take-loop below decide the outcome.
    //
    // ⭐ **The mechanism, exactly.** The vector is captured here, before the
    // loop runs, and the record is pushed carrying `false` — NOT admitted. The
    // only thing that ever flips it to `true` is the assignment after the loop
    // (below), which is reachable only when the loop has fallen through every
    // required position without returning. So the recorded outcome is
    // **production's own control flow**: reaching that line is what "admitted"
    // means, and the loop's two `return Ok(None)` clauses simply leave the
    // record as it was pushed.
    //
    // ⛔⛔ **This is deliberately NOT a predicate written beside the loop, and
    // the earlier form was.** That form recomputed the outcome as
    // "every position closed and unambiguous" right here — which made the
    // control compare the instrument with itself, so an extra route modality
    // installed in the real loop passed unnoticed. That was measured, not
    // supposed: a mutation admitting an all-`Open` vector survived until this
    // was restructured. The control's subject is that admission *equals* that
    // predicate, so the instrument must not be allowed to assume it.
    #[cfg(test)]
    let recorded = if D4B_ADMISSION_ARMED.with(std::cell::Cell::get) {
        let verdicts = reached
            .iter()
            .take(required_input_count)
            .map(|value| match value {
                ContinuationValueSourceAuthority::Open => D4bVerdict::Open,
                ContinuationValueSourceAuthority::Closed(sources) if sources.len() == 1 => {
                    D4bVerdict::Closed
                }
                ContinuationValueSourceAuthority::Closed(sources) => {
                    D4bVerdict::Ambiguous(sources.len())
                }
            })
            .collect::<Vec<_>>();
        // Pushed as NOT admitted. ⛔ The `false` is load-bearing, not a
        // placeholder -- see the mechanism note above; nothing here may compute
        // the outcome.
        Some(D4B_ADMISSION.with(|ledger| {
            let mut ledger = ledger.borrow_mut();
            ledger.push((verdicts, false));
            ledger.len() - 1
        }))
    } else {
        None
    };
    let mut exact_inputs = Vec::with_capacity(required_input_count);
    for value in reached.into_iter().take(required_input_count) {
        let ContinuationValueSourceAuthority::Closed(mut sources) = value else {
            // An open value refuses this dormant specialization candidate; it
            // is not authority to reject the enclosing source program.
            return Ok(None);
        };
        if sources.len() != 1 {
            // Distinct exact sources are deliberately not collapsed to a
            // descriptor ordinal. Ambiguity refuses only this candidate.
            return Ok(None);
        }
        exact_inputs.push(sources.remove(0));
    }
    // The take-loop fell through: this candidate is admitted. ⭐ This assignment
    // IS the observation -- reaching this line is what "admitted" means.
    #[cfg(test)]
    if let Some(index) = recorded {
        D4B_ADMISSION.with(|ledger| ledger.borrow_mut()[index].1 = true);
    }
    // `RT-CONTSRC-PRODUCER-LOCAL` `D4a` — ADMISSION. The `D2` transition
    // sentinel that declined every producer-local candidate stood exactly here
    // and is now deleted, which is the event its own promise class named.
    //
    // ⭐ **Nothing replaces it, and that is the point.** The declined set `R`
    // is refused **upstream** by the take-loop above, on the authority that was
    // always there: an `Open` value declines, and a position carrying more than
    // one exact source declines as ambiguous. Those two clauses are precisely
    // the census's three non-closed positions — `OPEN[ih-binder]`,
    // `OPEN[let-value:Construct]`, `AMBIG2[let-value:If]`. So admitting `V` is
    // *removing* a filter, never adding a selector.
    //
    // ⛔ No corpus, closure identity, planned-member status, first-`Open`
    // classification or edge predicate is consulted here or anywhere below.
    // The full required vector is walked — `required_input_count` positions,
    // every one of which must be closed and unambiguous — and that walk remains
    // the sole authority.
    //
    // ⚠ The consumers below are what make this safe: `D3a` taught
    // `validate_continuation_source_slot` to RE-DERIVE a producer-local source
    // rather than refuse it, so admitting the domain no longer walks into a
    // planner error. The two lowering emission seams still refuse both local
    // availability arms; `D3b` assigns them, against the emissions this
    // checkpoint creates.
    for source in &exact_inputs {
        validate_continuation_source_slot(plan, source)?;
    }
    #[cfg(test)]
    let inputs = {
        let mut inputs = exact_inputs;
        match CONTINUATION_PRODUCTION_MUTATION.with(Cell::get) {
            ContinuationProductionMutation::ResultLifetimeProxy => {
                let descriptor = plan
                    .abi
                    .descriptors
                    .iter()
                    .find(|descriptor| descriptor.function == consumer_owner)
                    .ok_or_else(|| planner_error("continuation consumer has no ABI descriptor"))?;
                let affinity = lifetime_referent_affinity(
                    occurrence_authority(plan, descriptor.body_occurrence)?.lifetime,
                );
                for source in &mut inputs {
                    source.referent_affinity = affinity.clone();
                }
            }
            ContinuationProductionMutation::DescriptorOrdinalSources => {
                inputs = entry_sources
                    .into_iter()
                    .take(required_input_count)
                    .collect();
            }
            ContinuationProductionMutation::Exact
            | ContinuationProductionMutation::ConstructorFieldCountPrefix
            | ContinuationProductionMutation::DescriptorInputCountTruncation => {}
        }
        inputs
    };
    #[cfg(not(test))]
    let inputs = exact_inputs;
    Ok(Some(ContinuationProducerEnvironment {
        producer_owner,
        producer_result_origin,
        producer_construct_origin,
        consumer_owner,
        inputs,
    }))
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D2b` arm 1** — the exact semantic environment
/// in force at the emission seat.
///
/// ⭐ This is the *same* forward walk `D1`/`D2` already use to build a
/// continuation's source environment, pointed at a different target. The walk
/// returns the environment it is holding when it reaches its target, so
/// targeting the emission occurrence yields the environment the emitter is
/// standing in — binders pushed by every intervening `Let`, `Match` case and
/// `ComputationalMatch` case already applied. That is precisely what
/// "nearest-alias" names, and it is why no new authority is needed: the binder
/// push rules are read off the walk rather than restated here.
///
/// ⛔ **Fail-closed path 1 of 5 — wrong emission origin.** The seat must be an
/// occurrence of the producer owner *and* one of this exact result edge's
/// construct origins. A seat that is neither has no defined lexical environment
/// and gets an error, never an empty one.
pub(super) fn continuation_emission_seat_environment(
    plan: &StaticTransitionPlan<'_>,
    environment: &ContinuationProducerEnvironment,
) -> Result<(StaticOriginId, Vec<ContinuationValueSourceAuthority>), CraneliftBackendError> {
    if occurrence_authority(plan, environment.producer_construct_origin)?.owner
        != environment.producer_owner
        || !continuation_result_origins(plan, environment.producer_result_origin)?
            .contains(&environment.producer_construct_origin)
    {
        return Err(planner_error(
            "a continuation emission seat is not an occurrence of its own producer owner on its \
             own result edge, so no exact lexical environment holds there; RT-CONTSRC-PRODUCER-\
             LOCAL D2b refuses rather than walking to a seat that belongs to another edge",
        ));
    }
    let source_root = continuation_owner_source_root(plan, environment.producer_owner)?;
    let entry_environment = continuation_owner_entry_sources(plan, environment.producer_owner)?
        .into_iter()
        .map(ContinuationValueSourceAuthority::source)
        .collect::<Vec<_>>();
    let (_, reached) = walk_continuation_value_environment(
        plan,
        source_root,
        environment.producer_construct_origin,
        &entry_environment,
    )?;
    let reached = reached.ok_or_else(|| {
        planner_error(
            "a continuation emission seat is outside its producer owner's source subtree, so the \
             forward semantic environment walk never reaches it and no nearest-alias index exists",
        )
    })?;
    Ok((source_root, reached))
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D3b`** — VERIFY that one producer-local
/// coordinate really occupies the nearest-alias index a `CurrentLexical`
/// availability names, at the emission seat that availability is keyed to.
///
/// ⭐ **This is verification, not derivation, and the direction is the whole
/// point.** The index arrives from the projection; this walks the emission
/// seat's own environment and asks whether that index is where the coordinate
/// actually sits. Deriving the index *from* the environment here would be the
/// reverse map `evt_609am4v7cdt5b` forbids — lowering would be re-answering a
/// question the planner owns, and the two answers could then disagree with
/// nothing to arbitrate them.
///
/// ⛔ **Why the emission consumer needs it at all.** A `CurrentLexical`
/// availability is consumed by *indexing an environment*, and every incidental
/// discriminator a consumer could otherwise check — carrier, ownership, storage
/// owner, referent affinity, lowering shape — is **equal** across the positions
/// of one seat environment in the measured population (`D4a`, exact
/// `ac897a08`). So a consumer that indexed with the wrong number would read a
/// well-formed operand of exactly the right contract and emit a call with the
/// wrong value in it, silently. This is the check that makes that
/// unrepresentable.
///
/// ⚠ **THE GAP, stated because it is easy to over-read.** This proves the
/// consumer indexes with the number the planner assigned. It does **not**
/// re-prove the assignment: it re-runs the planner's own walk, so a defect in
/// that walk would be reproduced here rather than caught. `D2b`'s discriminator
/// and `D3a`'s validator own that half.
pub(in crate::cranelift_backend) fn verify_current_lexical_availability(
    plan: &StaticTransitionPlan<'_>,
    emission_owner: PredeclaredFunctionId,
    producer_result_origin: StaticOriginId,
    emission_origin: StaticOriginId,
    lexical_environment_origin: StaticOriginId,
    requested: &ContinuationSourceSlotAuthority,
    nearest_alias_index: u32,
) -> Result<(), CraneliftBackendError> {
    let producer_owner = emission_owner;
    let environment = ContinuationProducerEnvironment {
        producer_owner,
        producer_result_origin,
        producer_construct_origin: emission_origin,
        consumer_owner: producer_owner,
        inputs: Vec::new(),
    };
    let (source_root, seat) = continuation_emission_seat_environment(plan, &environment)?;
    if source_root != lexical_environment_origin {
        return Err(planner_error(
            "a current-lexical availability names a lexical environment origin that is not the              emitting owner's own source root, so the index it carries counts binders in an              environment this seat never stands in",
        ));
    }
    // ⛔ Reuses the projection's OWN locator, so "where the coordinate is" has
    // exactly one definition in this plane. A second search written here would
    // be a second authority that could drift from the first.
    let derived = current_lexical_availability(
        requested,
        emission_owner,
        producer_result_origin,
        source_root,
        emission_origin,
        &seat,
    )?;
    if derived
        != (ContinuationEnvironmentDraft::CurrentLexical {
            emission_owner,
            producer_result_origin,
            emission_origin,
            lexical_environment_origin,
            nearest_alias_index,
        })
    {
        return Err(planner_error(
            "a continuation input is being consumed at an index the emission seat's own lexical              environment does not hold that coordinate at; RT-CONTSRC-PRODUCER-LOCAL D3b refuses              rather than emitting a call carrying a well-formed operand of the right contract and              the wrong value",
        ));
    }
    Ok(())
}

/// **`D3b` re-cut — VERIFY that one predeclared frame really declares a member
/// for a full coordinate, at exactly the declared slot.**
///
/// ⭐ **Membership, not numeric agreement.** The retired law asked whether
/// `immediate_slot == source_abi_position` — a comparison between a frame
/// position and a ROOT position, which is the coupling `D3c` falsified. This
/// asks the only question that is actually about the frame: *does this
/// descriptor's own entry run declare this coordinate here?*
///
/// ⛔ **A `ProducerLocal` member cannot be invented.** The entry source
/// enumeration produces exactly the entry ABI input run, so a mid-body value is
/// simply absent from it and this refuses. That is the ruled law: a
/// producer-local member remains unavailable at a predeclared entry frame unless
/// a separately authorized substrate later declares one.
pub(in crate::cranelift_backend) fn verify_predeclared_entry_frame_membership(
    plan: &StaticTransitionPlan<'_>,
    frame: PredeclaredFunctionId,
    coordinate: ContinuationSourceCoordinate,
    declared_slot: u32,
) -> Result<(), CraneliftBackendError> {
    let members = continuation_owner_entry_sources(plan, frame)?;
    let mut found = None;
    for (position, member) in members.iter().enumerate() {
        if member.coordinate != coordinate {
            continue;
        }
        if found.is_some() {
            return Err(planner_error(
                "a predeclared entry frame declares two members for one continuation \
                 coordinate, so its declared slot is ambiguous; RT-CONTSRC-PRODUCER-LOCAL D3b \
                 refuses rather than taking the first",
            ));
        }
        found = Some(position);
    }
    let position = found.ok_or_else(|| {
        planner_error(
            "a predeclared entry frame declares no member for this continuation coordinate, so \
             its entry run cannot make that value available; RT-CONTSRC-PRODUCER-LOCAL D3b fails \
             closed rather than reading whichever operand sits at a plausible position",
        )
    })?;
    let position = u32::try_from(position)
        .map_err(|_| planner_capacity_error("predeclared entry frame slot exhausted"))?;
    if position != declared_slot {
        return Err(planner_error(
            "an entry-frame claim's declared slot is not the position at which that predeclared \
             frame declares the coordinate, so the slot names a different member",
        ));
    }
    Ok(())
}

/// **`D3b` re-cut — the position at which one predeclared frame's ENTRY RUN
/// declares a coordinate**, or `None` when it declares no member for it.
///
/// ⭐⭐ **This is the measured shape of the capture consumer's source frame, and
/// the measurement is what put it here.** The generated-context capture append
/// indexes `function_local.defining_abi_operands` — the operand run of the frame
/// *currently being defined*. Over the corpus that frame was, in every
/// observation, the **emission owner of the enclosing specialization**, and in
/// every observation it was a predeclared function rather than that
/// specialization's own generated context. So the capture consumer's claim is an
/// entry-frame claim against a PREDECLARED frame, and its declared slot is the
/// coordinate's position in that frame's entry ABI run.
///
/// ⛔ `None` is the honest answer, not a failure to look. A `ProducerLocal`
/// coordinate is a mid-body value with no position in any entry run, so no
/// capture claim can be built for it and the consumer refuses rather than
/// indexing an operand run the value was never in. `D4b` owns making such a value
/// capturable; until then the boundary fails closed.
///
/// ⛔ Membership is by the **whole coordinate, exactly once**. Two members for
/// one coordinate is an ambiguous slot, refused rather than resolved by taking
/// the first.
pub(super) fn predeclared_entry_frame_slot(
    plan: &StaticTransitionPlan<'_>,
    frame: PredeclaredFunctionId,
    coordinate: ContinuationSourceCoordinate,
) -> Result<Option<u32>, CraneliftBackendError> {
    let members = continuation_owner_entry_sources(plan, frame)?;
    let mut found = None;
    for (position, member) in members.iter().enumerate() {
        if member.coordinate != coordinate {
            continue;
        }
        if found.is_some() {
            return Err(planner_error(
                "a predeclared entry frame declares two members for one continuation coordinate, \
                 so the capture slot it would supply is ambiguous; \
                 RT-CONTSRC-PRODUCER-LOCAL D3b refuses rather than taking the first",
            ));
        }
        found = Some(position);
    }
    found
        .map(|position| {
            u32::try_from(position)
                .map_err(|_| planner_capacity_error("predeclared entry frame slot exhausted"))
        })
        .transpose()
}

/// **`D3b` (alias repair) — THE NEAREST EXACT ALIAS**, the one total rule that
/// selects a lexical position, shared by the planner that issues the claim and
/// the consumer that revalidates it.
///
/// ⭐⭐ **Why this is not the banned "first match", stated where the rule lives.**
/// The ban exists because choosing among candidates *never proved equivalent*
/// silently picks one of several different values. Here every candidate is
/// proved to be the same semantic value **before** ordering is consulted at all:
/// eligibility is exact equality of the complete
/// [`ContinuationSourceSlotAuthority`], and the discriminator is **eligibility,
/// not ordering**. Ordering only canonicalizes among proved aliases.
///
/// The proof comes from the authority's own algebra, not from an assumption:
/// [`ContinuationValueSourceAuthority::join`] unions and **deduplicates complete
/// records**, so
///
/// - `Closed([S])` means every represented path yields exactly source slot `S`;
/// - `Closed([S, T])` means the value is ambiguous between distinct sources and
///   is **not** an exact alias, even though it contains `S`;
/// - two positions each holding `Closed([S])` are proved aliases of one semantic
///   source, whatever names lowering later assigns them.
///
/// ⛔ **The retired law, named so it is not reconstructed.** This previously
/// required the coordinate to occur **exactly once** and refused two positions as
/// ambiguous. That was measured false against `let y = x`: a non-`Effect` `Let`
/// pushes the bound expression's own authority, so one root identity lawfully
/// occupies two bindings. The old law conflated *"does this position certainly
/// hold `S`"* with *"is it the only position that does"*; `D3b` needs the first
/// and never the second.
///
/// ⛔ Eligibility is the **complete** record — coordinate, carrier, ownership,
/// storage owner and referent affinity. A position carrying the same coordinate
/// under a different contract is a different value and does not qualify.
pub(super) fn nearest_exact_alias(
    requested: &ContinuationSourceSlotAuthority,
    seat_environment: &[ContinuationValueSourceAuthority],
) -> Result<u32, CraneliftBackendError> {
    let mut eligible: Vec<u32> = Vec::new();
    // Two witnesses kept apart on purpose: they make the three refusals below
    // distinguishable, so a control naming one cannot pass by tripping another.
    let mut ambiguous = false;
    let mut contract_mismatch = false;
    for (index, value) in seat_environment.iter().enumerate() {
        let ContinuationValueSourceAuthority::Closed(sources) = value else {
            continue;
        };
        let index = u32::try_from(index).map_err(|_| {
            planner_capacity_error("continuation lexical environment index exhausted")
        })?;
        match sources.as_slice() {
            // ⭐ Exactly `Closed([S])`, compared as the WHOLE record.
            [only] if only == requested => eligible.push(index),
            [only] if only.coordinate == requested.coordinate => contract_mismatch = true,
            many if many.iter().any(|source| source.coordinate == requested.coordinate) => {
                ambiguous = true;
            }
            _ => {}
        }
    }
    // ⛔ `min`, written as a fold over the whole eligible set rather than as an
    // early exit from the loop above. The two agree today because the scan is
    // ascending -- and that is exactly why the total rule is spelled out here:
    // an early `break` would read as "take the first", and a later reordering of
    // the scan would silently change the answer.
    if let Some(selected) = eligible.iter().copied().min() {
        return Ok(selected);
    }
    if ambiguous {
        return Err(planner_error(
            "the emission seat holds this continuation coordinate only inside an ambiguous \
             source set (a Closed([S, T]) join), which does not prove any position certainly \
             yields the requested value; RT-CONTSRC-PRODUCER-LOCAL D3b requires an exact \
             singleton and refuses rather than selecting a position that may yield another \
             source",
        ));
    }
    if contract_mismatch {
        return Err(planner_error(
            "the emission seat holds this continuation coordinate under a different carrier, \
             ownership, storage owner or referent affinity, so it is a different value with the \
             same root identity; RT-CONTSRC-PRODUCER-LOCAL D3b matches the complete source-slot \
             authority and refuses rather than indexing on the coordinate alone",
        ));
    }
    Err(planner_error(
        "a continuation coordinate is not present in the lexical environment in force at the \
         emission seat, so the value is not immediately available there; this fails closed \
         rather than reverse-searching for a position that happens to hold a similar value",
    ))
}

/// **`D3b` re-cut, the `CurrentLexical` arm** — select the nearest exact alias of
/// one requested source slot in the emission seat's environment.
///
/// ⭐ **Either root arm reaches here, and that is the `D3c` correction.** The
/// seat environment is seeded by `continuation_owner_entry_sources`, whose
/// members carry `EntryAbi` coordinates, and then walked forward with binders
/// prepended. So an entry value's lexical position is found by the same rule
/// that finds a producer-local one — which is exactly why the old "an entry root
/// takes its ABI position" shortcut was wrong.
pub(super) fn current_lexical_availability(
    requested: &ContinuationSourceSlotAuthority,
    emission_owner: PredeclaredFunctionId,
    producer_result_origin: StaticOriginId,
    lexical_environment_origin: StaticOriginId,
    emission_origin: StaticOriginId,
    seat_environment: &[ContinuationValueSourceAuthority],
) -> Result<ContinuationEnvironmentDraft, CraneliftBackendError> {
    Ok(ContinuationEnvironmentDraft::CurrentLexical {
        emission_owner,
        producer_result_origin,
        emission_origin,
        lexical_environment_origin,
        nearest_alias_index: nearest_exact_alias(requested, seat_environment)?,
    })
}

/// **`D3b` re-cut — build the two consumer-specific availability views.**
///
/// ⛔ The `emitter` argument names the frame that will *emit* this call, and it
/// is the only thing consulted. Nothing here reads a root coordinate to decide
/// which environment a value lives in — that coupling is precisely what `D3c`
/// falsified.
pub(super) fn exact_continuation_projection(
    plan: &StaticTransitionPlan<'_>,
    environment: &ContinuationProducerEnvironment,
    ordinary_parameters: u32,
    emitter: &ContinuationEmitterFrame<'_>,
) -> Result<Vec<ContinuationInputProjection>, CraneliftBackendError> {
    // The emission seat's lexical environment is derived at most once per edge,
    // and only where the direct-emission consumer will actually read it.
    let mut seat_environment: Option<(StaticOriginId, Vec<ContinuationValueSourceAuthority>)> =
        None;
    environment
        .inputs
        .iter()
        .enumerate()
        .map(|(ordinal, input)| {
            let ordinal = u32::try_from(ordinal).map_err(|_| {
                planner_capacity_error("continuation projection ordinal exhausted")
            })?;
            let availability = match emitter {
                // ⭐⭐ **The `D3c` correction, and the whole point of the re-cut.**
                // A predeclared emitter's direct-emission consumer reads the
                // retained lexical environment at the seat. So EVERY input --
                // entry-rooted or producer-local alike -- takes a
                // `CurrentLexical` claim whose index comes from the forward
                // walk. ⛔ An entry root does NOT take its ABI position here;
                // that shortcut was measured wrong at nonzero binder depth.
                ContinuationEmitterFrame::Predeclared(emission_owner) => {
                    let (lexical_environment_origin, seat) = match &seat_environment {
                        Some(seat) => seat,
                        None => seat_environment.insert(
                            continuation_emission_seat_environment(plan, environment)?,
                        ),
                    };
                    let claim = current_lexical_availability(
                        input,
                        *emission_owner,
                        environment.producer_result_origin,
                        *lexical_environment_origin,
                        environment.producer_construct_origin,
                        seat,
                    )?;
                    // ⭐⭐ **The capture view, built against a DIFFERENT
                    // environment of the same frame — this is the re-cut's whole
                    // claim, made concrete.**
                    //
                    // The direct-emission consumer above reads this predeclared
                    // frame's retained LEXICAL environment, so it takes a
                    // nearest-alias index. The capture-append consumer reads the
                    // same frame's ENTRY ABI RUN, so it takes that run's
                    // position. `D3c` measured those two numbers diverging at
                    // nonzero binder depth — which is exactly why one field could
                    // not serve both, and why each is derived from its own
                    // environment here rather than one being computed from the
                    // other.
                    //
                    // ⛔ `None` when the frame declares no member: fails closed.
                    // Nothing invents a position, and no fallback reads the
                    // direct-emission index as a frame slot.
                    let capture = predeclared_entry_frame_slot(
                        plan,
                        *emission_owner,
                        input.coordinate,
                    )?
                    .map(|declared_slot| ContinuationEnvironmentDraft::EntryFrame {
                        frame: ContinuationFrameRequirement::Predeclared(*emission_owner),
                        declared_slot,
                    });
                    ContinuationAvailabilityDraft {
                        direct_emission: Some(claim),
                        context_capture: capture,
                    }
                }
                // The emitting frame is a generated execution context. Its
                // captures ARE the enclosing specialization's continuation
                // inputs, in ordinal order, laid out after its parameter run.
                //
                // ⛔ Membership is by the **whole coordinate**, exactly once.
                // Matching on a position, or on an owner/position pair, would
                // let a local binding and an entry position collide by carrying
                // the same integer.
                ContinuationEmitterFrame::GeneratedContext {
                    enclosing,
                    worker_body_origin,
                    context_parameters,
                    enclosing_inputs,
                } => {
                    let position = enclosing_inputs
                        .iter()
                        .position(|enclosing| enclosing.coordinate == input.coordinate)
                        .ok_or_else(|| {
                            planner_error(
                                "a continuation input's coordinate is not among the enclosing \
                                 specialization's continuation inputs, so the generated emission \
                                 context declares no member for it; this fails closed rather than \
                                 falling back to a root position, which would read whatever the \
                                 raw body happened to hold there",
                            )
                        })?;
                    if enclosing_inputs
                        .iter()
                        .filter(|enclosing| enclosing.coordinate == input.coordinate)
                        .count()
                        != 1
                    {
                        return Err(planner_error(
                            "a generated emission context declares two members for one \
                             continuation coordinate, so its declared slot is ambiguous; \
                             RT-CONTSRC-PRODUCER-LOCAL D3b refuses rather than taking the first",
                        ));
                    }
                    let position = u32::try_from(position).map_err(|_| {
                        planner_capacity_error("continuation immediate slot exhausted")
                    })?;
                    let declared_slot =
                        context_parameters.checked_add(position).ok_or_else(|| {
                            planner_capacity_error(
                                "continuation immediate slot position exhausted",
                            )
                        })?;
                    let claim = ContinuationEnvironmentDraft::EntryFrame {
                        frame: ContinuationFrameRequirement::GeneratedContext {
                            enclosing: *enclosing,
                            worker_body_origin: *worker_body_origin,
                        },
                        declared_slot,
                    };
                    // ⭐ The SAME claim serves both consumers here, and that is
                    // sound for one reason only: a generated context's direct
                    // emission and its capture append read the same frame -- its
                    // own operand run. ⛔ It is written twice rather than shared
                    // through one field, so a later divergence between the two
                    // consumers is a local edit and not a silent reinterpretation.
                    ContinuationAvailabilityDraft {
                        direct_emission: Some(claim),
                        context_capture: Some(claim),
                    }
                }
            };
            Ok(ContinuationInputProjection {
                availability,
                producer_owner: environment.producer_owner,
                consumer_owner: environment.consumer_owner,
                coordinate: input.coordinate,
                ordinal,
                carrier: input.carrier,
                ownership: input.ownership,
                storage_owner: input.storage_owner,
                referent_affinity: input.referent_affinity.clone(),
                ordinary_abi_position: ordinary_parameters
                    .checked_add(ordinal)
                    .ok_or_else(|| {
                        planner_capacity_error(
                            "continuation projection ABI position exhausted",
                        )
                    })?,
            })
        })
        .collect()
}

pub(super) fn exact_continuation_ordinary_parameters(
    plan: &StaticTransitionPlan<'_>,
    producer_construct_origin: StaticOriginId,
    arguments: &[RuntimeExpr],
    recursive_position: usize,
    recursive_positions: &BTreeSet<u32>,
    worker: &ContinuationWorkerProvenance,
) -> Result<u32, CraneliftBackendError> {
    if worker.producer_origin != producer_construct_origin
        || usize::try_from(worker.sibling_position).ok() != Some(recursive_position)
        || arguments.get(recursive_position).is_none()
    {
        return Err(planner_error(
            "continuation ordinary envelope disagrees with its static worker",
        ));
    }
    let mut ordinary_fields = 0usize;
    for (position, _) in arguments.iter().enumerate() {
        // ⭐⭐ `D2b` — EVERY recursive position is excluded, not just this
        // unit's own. `recursive_position` answers "is this field MINE?"; the
        // runtime envelope needs "is this field RECURSIVE?", and a sibling
        // recursive field counted here becomes an ordinary ABI parameter
        // carrying a `Specialized(Closure)` the boundary correctly refuses.
        if u32::try_from(position)
            .ok()
            .is_some_and(|encoded| recursive_positions.contains(&encoded))
        {
            continue;
        }
        debug_assert!(
            position != recursive_position,
            "this unit's own recursive position must be a member of the closed projection"
        );
        let field_origin = plan
            .semantic
            .child_origin(producer_construct_origin, position)?;
        occurrence_authority(plan, field_origin)?;
        ordinary_fields = ordinary_fields.checked_add(1).ok_or_else(|| {
            planner_capacity_error("continuation ordinary field population exhausted")
        })?;
    }
    for (ordinal, capture) in worker.captures.iter().enumerate() {
        if usize::try_from(capture.ordinal).ok() != Some(ordinal)
            || capture.closure_origin != worker.closure_origin
        {
            return Err(planner_error(
                "continuation worker captures are not one exact ordered envelope",
            ));
        }
        ordinary_fields = ordinary_fields.checked_add(1).ok_or_else(|| {
            planner_capacity_error("continuation ordinary capture population exhausted")
        })?;
    }
    #[cfg(test)]
    if CONTINUATION_PRODUCTION_MUTATION.with(Cell::get)
        == ContinuationProductionMutation::ConstructorFieldCountPrefix
    {
        ordinary_fields = arguments.len();
    }
    u32::try_from(ordinary_fields)
        .map_err(|_| planner_capacity_error("continuation ordinary arity exhausted"))
}

/// Copy exactly one component of an `EntryAbi` coordinate from `source` into
/// `target`, for the `AC-2` omission matrix.
///
/// ⛔ Panics on a producer-local coordinate. The matrix proves a field is
/// load-bearing by neutralizing it and observing the two keys become equal; a
/// silent no-op would leave them unequal and be read as proof.
#[cfg(test)]
fn copy_entry_coordinate_component(
    target: &mut ContinuationSourceCoordinate,
    source: &ContinuationSourceCoordinate,
    component: ContinuationProjectionOmission,
) {
    let (
        ContinuationSourceCoordinate::EntryAbi {
            source_owner: from_owner,
            source_abi_position: from_position,
            source: from_source,
        },
        ContinuationSourceCoordinate::EntryAbi {
            source_owner: to_owner,
            source_abi_position: to_position,
            source: to_source,
        },
    ) = (source, target)
    else {
        panic!(
            "the AC-2 omission matrix reached a producer-local coordinate; it would have \
             neutralized nothing and reported the field load-bearing anyway"
        );
    };
    match component {
        ContinuationProjectionOmission::SourceOwner => *to_owner = *from_owner,
        ContinuationProjectionOmission::SourceAbiPosition => *to_position = *from_position,
        ContinuationProjectionOmission::Source => *to_source = *from_source,
        other => panic!("{other:?} is not a coordinate component"),
    }
}

#[cfg(test)]
fn continuation_keys_equal_under_mutation(
    left: &ContinuationSpecializationKey,
    right: &ContinuationSpecializationKey,
    mutation: ContinuationInternMutation,
) -> bool {
    match mutation {
        ContinuationInternMutation::Exact => left == right,
        ContinuationInternMutation::PrefixOnly => {
            left.consumer_owner == right.consumer_owner
                && left.continuation_origin == right.continuation_origin
        }
        ContinuationInternMutation::OmitProjection(field) => {
            if left.continuation_inputs.len() != right.continuation_inputs.len() {
                return false;
            }
            let mut normalized = right.clone();
            for (source, target) in left
                .continuation_inputs
                .iter()
                .zip(&mut normalized.continuation_inputs)
            {
                match field {
                    ContinuationProjectionOmission::ProducerOwner => {
                        target.producer_owner = source.producer_owner
                    }
                    ContinuationProjectionOmission::ConsumerOwner => {
                        target.consumer_owner = source.consumer_owner
                    }
                    // `D1` — the three source components now live inside the
                    // `EntryAbi` arm, so the copy-back reaches into it. ⛔ A
                    // producer-local coordinate PANICS rather than silently
                    // copying nothing: a no-op here would leave the two keys
                    // unequal and the control would report "distinguished"
                    // while having applied no mutation at all.
                    ContinuationProjectionOmission::SourceOwner
                    | ContinuationProjectionOmission::SourceAbiPosition
                    | ContinuationProjectionOmission::Source => {
                        copy_entry_coordinate_component(
                            &mut target.coordinate,
                            &source.coordinate,
                            field,
                        )
                    }
                    ContinuationProjectionOmission::Ordinal => target.ordinal = source.ordinal,
                    ContinuationProjectionOmission::Carrier => target.carrier = source.carrier,
                    ContinuationProjectionOmission::Ownership => {
                        target.ownership = source.ownership
                    }
                    ContinuationProjectionOmission::StorageOwner => {
                        target.storage_owner = source.storage_owner
                    }
                    ContinuationProjectionOmission::ReferentAffinity => {
                        target.referent_affinity = source.referent_affinity.clone()
                    }
                    ContinuationProjectionOmission::OrdinaryAbiPosition => {
                        target.ordinary_abi_position = source.ordinary_abi_position
                    }
                }
            }
            left == &normalized
        }
    }
}


pub(super) fn intern_specialization(
    interned: &mut BTreeMap<ContinuationSpecializationKey, ContinuationSpecializationId>,
    units: &mut Vec<PlannedContinuationSpecialization>,
    key: ContinuationSpecializationKey,
) -> Result<(ContinuationSpecializationId, bool), CraneliftBackendError> {
    #[cfg(test)]
    {
        let mutation = CONTINUATION_INTERN_MUTATION.with(Cell::get);
        if mutation != ContinuationInternMutation::Exact {
            if let Some(unit) = units
                .iter()
                .find(|unit| continuation_keys_equal_under_mutation(&unit.key, &key, mutation))
            {
                return Ok((unit.id, false));
            }
        }
    }
    if let Some(id) = interned.get(&key).copied() {
        let unit = units
            .get(id.0 as usize)
            .ok_or_else(|| planner_error("interned continuation has no unit"))?;
        if unit.key != key {
            return Err(planner_error(
                "interned continuation identity is not full-key exact",
            ));
        }
        return Ok((id, false));
    }
    let id = ContinuationSpecializationId(
        u32::try_from(units.len()).map_err(|_| {
            planner_capacity_error("continuation specialization identity exhausted")
        })?,
    );
    // The immutable key is installed before the caller performs any recursive
    // discovery. This ordering is the fixed point's decreasing measure.
    interned.insert(key.clone(), id);
    units.push(PlannedContinuationSpecialization {
        id,
        key,
        finalized_availability: Vec::new(),
    });
    Ok((id, true))
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2i` — one discovery the production fixed
/// point ADMITTED.**
///
/// Recorded only after `visited.insert` accepts the item, and returned from the
/// same `build_continuation_specialization_plan` invocation that produced it.
///
/// Production does derive the INITIAL frontier as `child(consumer, 0)` over
/// every planned `ComputationalMatch`. What it must not do is mistake that
/// frontier for the complete admitted population: the fixed point descends into
/// selected worker bodies and admits further discoveries that no seed scan can
/// name.
///
/// All three admitted identity fields are carried. The outer consuming
/// occurrence is separate traversal authority: it continues to the
/// specialization key but is not projected into this identity ledger.
/// `enclosing_specialization` is the immediate emission context, and it
/// **cannot be reconstructed downstream** --
/// a worker body's raw occurrence owner does not name the specialization that
/// selected and invoked it, which is the conflation `D5a` removed. Projecting it
/// away would collapse two genuinely distinct admitted discoveries into one
/// entry, so it is copied straight off the discovery at the moment of
/// admission.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct AdmittedContinuationDiscovery {
    pub(super) continuation_origin: StaticOriginId,
    pub(super) result_root: StaticOriginId,
    /// Copied from the admitted discovery, never re-derived.
    pub(super) enclosing_specialization: Option<ContinuationSpecializationId>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct ContinuationConsumingOccurrenceSeed {
    pub(super) alternative: u32,
    pub(super) occurrence: ContinuationConsumingOccurrence,
}

/// Forward outer-match facts carried with a continuation discovery.
///
/// Every candidate was written while the eliminator and its ordinal-selected
/// case body were both in hand. Selection waits until the inner specialization
/// alternative is known, but no later step searches for a parent or indexes a
/// relation by continuation origin.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct ContinuationConsumingOccurrenceSeeds {
    pub(super) candidates: Vec<ContinuationConsumingOccurrenceSeed>,
}

/// The exact outer consumer a discovery's producers must use.
///
/// A source discovery retains the prior outer-match candidates until its
/// specialization alternative selects one. A generated descent already has
/// that selection in its newly interned target, so it carries the exact
/// occurrence directly. These are two construction phases of one relation,
/// not two lookup authorities.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum ContinuationRequiredConsumingOccurrence {
    /// `RT-CONSUMER-CARRY-CONTROL-DEBT` `C4` disposition: retained as the
    /// source-side phase of the relation. The current depth fixtures take the
    /// outermost fallback instead of exercising an inherited nested-Source
    /// population, and this variant is resolved only by the test observer.
    /// It is therefore not evidence of a production consumer or route; a
    /// successor that relies on this arm owes its own reaching control.
    Source(ContinuationConsumingOccurrenceSeeds),
    Exact(ContinuationConsumingOccurrence),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct ContinuationDiscovery {
    pub(super) continuation_origin: StaticOriginId,
    pub(super) result_root: StaticOriginId,
    /// **`D5a` — the enclosing generated emission context, retained across
    /// descent.**
    ///
    /// ⛔ This field is the correction. The fixed point used to descend into
    /// `worker.body_origin` carrying only the two origins, so the next
    /// iteration re-read the raw occurrence owner and lost the specialization
    /// that had selected and invoked that worker. Retaining it here is what
    /// makes "descending into the selected worker body from an interned
    /// specialization retains that specialization as the immediate emission
    /// owner" a fact of the traversal rather than something a later lookup has
    /// to reconstruct — which it provably cannot do.
    ///
    /// `None` at a top-level `ComputationalMatch` root: there is no enclosing
    /// generated context, so the emission owner is the raw occurrence owner.
    pub(super) enclosing_specialization: Option<ContinuationSpecializationId>,
    /// Forward-seeded outer consuming-case candidates, carried across the
    /// fixed-point descent. `None` means no direct outer computational
    /// eliminator selected this continuation as its scrutinee.
    pub(super) consuming_occurrences: Option<ContinuationConsumingOccurrenceSeeds>,
    /// `RT-CONTKEY-CONSUMER-DESCENT-CARRY` -- the consumer this discovery's
    /// producers must use. At the outermost source discovery, the fallback
    /// installs the consumer selected by that same match. The one-level lag
    /// begins at depth 2: a generated descent carries the consumer established
    /// by its parent specialization.
    ///
    /// This is traversal state, not specialization identity. It therefore
    /// remains beside `ContinuationSpecializationKey`: widening that key would
    /// change interning even though two walks that reach the same immutable
    /// specialization do not become different units merely because their
    /// outstanding outer consumer differs.
    pub(super) required_consuming_occurrence: Option<ContinuationRequiredConsumingOccurrence>,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum RequiredConsumerProjectionDisposition {
    Minted,
    SkippedRequiredEqualsSource,
    AbsentNoRequiredConsumer,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct ContinuationRequiredConsumerObservation {
    pub(super) continuation_origin: StaticOriginId,
    pub(super) result_root: StaticOriginId,
    pub(super) required: Option<ContinuationConsumingOccurrence>,
    pub(super) derived_at_consumer: Option<ContinuationConsumingOccurrence>,
    pub(super) child_push: bool,
    pub(super) projection_disposition: Option<RequiredConsumerProjectionDisposition>,
}

#[cfg(test)]
impl ContinuationRequiredConsumerObservation {
    pub(in crate::cranelift_backend) fn continuation_origin(self) -> StaticOriginId {
        self.continuation_origin
    }

    pub(in crate::cranelift_backend) fn result_root(self) -> StaticOriginId {
        self.result_root
    }

    pub(in crate::cranelift_backend) fn required(
        self,
    ) -> Option<ContinuationConsumingOccurrence> {
        self.required
    }

    pub(in crate::cranelift_backend) fn derived_at_consumer(
        self,
    ) -> Option<ContinuationConsumingOccurrence> {
        self.derived_at_consumer
    }

    pub(in crate::cranelift_backend) fn is_child_push(self) -> bool {
        self.child_push
    }

    pub(in crate::cranelift_backend) fn projection_disposition(
        self,
    ) -> Option<RequiredConsumerProjectionDisposition> {
        self.projection_disposition
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ContinuationConsumingOccurrenceSeedMutation {
    BodyOrigin,
    EliminatorOrigin,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum RequiredConsumerProjectionMutation {
    BodyOrigin,
    EliminatorOrigin,
}

#[cfg(test)]
thread_local! {
    pub(super) static WEAKEN_CONTINUATION_DECREASING_MEASURE: Cell<bool> = const { Cell::new(false) };
    pub(super) static SUPPRESS_POST_SPECIALIZATION_DESCENT: Cell<bool> = const { Cell::new(false) };
    pub(super) static DUPLICATE_STATIC_BODY_TRIPLE: Cell<bool> = const { Cell::new(false) };
    /// Suppress ONLY the binder-to-body resolution rule in preflight.
    ///
    /// This exists so a control can show that its perturbation left every OTHER
    /// preflight rule green. Asserting "the new rule refused" alone cannot say
    /// that: a perturbed key that also trips `BinderAgreement` would refuse
    /// under either rule, and a reader could not tell the discriminator from a
    /// proxy. Suppressing the one rule and observing the SAME perturbed key
    /// issue a claim is what makes the attribution exact.
    pub(super) static SUPPRESS_BINDER_BODY_RESOLUTION: Cell<bool> = const { Cell::new(false) };
    pub(super) static DUPLICATE_DESCENT_AS_TOP_LEVEL: Cell<bool> = const { Cell::new(false) };
    pub(super) static MUTATE_CONTINUATION_CONSUMING_OCCURRENCE_SEED: Cell<Option<ContinuationConsumingOccurrenceSeedMutation>> = const { Cell::new(None) };
    pub(super) static REQUIRED_CONSUMER_PROJECTION_MUTATION:
        Cell<Option<RequiredConsumerProjectionMutation>> = const { Cell::new(None) };
    pub(super) static REQUIRED_CONSUMER_PROJECTION_MUTATION_APPLICATIONS: Cell<usize> = const { Cell::new(0) };
    pub(super) static CONTINUATION_REQUIRED_CONSUMER_OBSERVATIONS:
        std::cell::RefCell<Vec<ContinuationRequiredConsumerObservation>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn with_required_consumer_projection_mutation<T>(
    mutation: RequiredConsumerProjectionMutation,
    run: impl FnOnce() -> T,
) -> (T, usize) {
    struct Restore(Option<RequiredConsumerProjectionMutation>);
    impl Drop for Restore {
        fn drop(&mut self) {
            REQUIRED_CONSUMER_PROJECTION_MUTATION.with(|cell| cell.set(self.0));
        }
    }
    let previous = REQUIRED_CONSUMER_PROJECTION_MUTATION
        .with(|cell| cell.replace(Some(mutation)));
    REQUIRED_CONSUMER_PROJECTION_MUTATION_APPLICATIONS.with(|cell| cell.set(0));
    let restore = Restore(previous);
    let result = run();
    let applications =
        REQUIRED_CONSUMER_PROJECTION_MUTATION_APPLICATIONS.with(Cell::get);
    drop(restore);
    (result, applications)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn take_continuation_required_consumer_observations(
) -> Vec<ContinuationRequiredConsumerObservation> {
    CONTINUATION_REQUIRED_CONSUMER_OBSERVATIONS.with(|observations| {
        std::mem::take(&mut *observations.borrow_mut())
    })
}

#[cfg(test)]
fn with_continuation_consuming_occurrence_seed_mutation<T>(
    mutation: ContinuationConsumingOccurrenceSeedMutation,
    run: impl FnOnce() -> T,
) -> T {
    struct Restore(Option<ContinuationConsumingOccurrenceSeedMutation>);
    impl Drop for Restore {
        fn drop(&mut self) {
            MUTATE_CONTINUATION_CONSUMING_OCCURRENCE_SEED.with(|cell| cell.set(self.0));
        }
    }

    let previous = MUTATE_CONTINUATION_CONSUMING_OCCURRENCE_SEED
        .with(|cell| cell.replace(Some(mutation)));
    let _restore = Restore(previous);
    run()
}

/// Run a control with only the forward consuming-body seed replaced by the
/// continuation's own occurrence.
#[cfg(test)]
pub(in crate::cranelift_backend) fn with_continuation_consuming_occurrence_seed_mutated<T>(
    run: impl FnOnce() -> T,
) -> T {
    with_continuation_consuming_occurrence_seed_mutation(
        ContinuationConsumingOccurrenceSeedMutation::BodyOrigin,
        run,
    )
}

/// Run a control with only the forward consuming-eliminator seed replaced by
/// the continuation's own match occurrence.
#[cfg(test)]
pub(in crate::cranelift_backend) fn with_continuation_consuming_eliminator_seed_mutated<T>(
    run: impl FnOnce() -> T,
) -> T {
    with_continuation_consuming_occurrence_seed_mutation(
        ContinuationConsumingOccurrenceSeedMutation::EliminatorOrigin,
        run,
    )
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8l2` — the ordinary-envelope population
/// defects.**
///
/// ⛔ Every one is a shape a wrong derivation would actually produce, not an
/// invented corruption. [`Self::DensePrefix`] is exactly what this method
/// emitted before `D8l2`, so the control that reds on it is a regression test
/// for the defect `D8l1` measured.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum EnvelopeDefect {
    Exact,
    /// Select a recursive position at the field count, which no plan produces.
    SelectionOutOfRange,
    /// Drop the last nonrecursive field.
    Omit,
    /// Repeat the first nonrecursive field in the last slot, so the length is
    /// right and the set is not.
    Duplicate,
    /// The pre-`D8l2` derivation: envelope indices emitted as source positions.
    DensePrefix,
    /// Every field present, in reverse source order.
    WrongOrder,
}

#[cfg(test)]
thread_local! {
    pub(super) static ENVELOPE_DEFECT: Cell<EnvelopeDefect> = const { Cell::new(EnvelopeDefect::Exact) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn set_envelope_defect(defect: EnvelopeDefect) {
    ENVELOPE_DEFECT.with(|cell| cell.set(defect));
}

#[cfg(test)]
fn envelope_defect() -> EnvelopeDefect {
    ENVELOPE_DEFECT.with(Cell::get)
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8a` — instantiate the second emission owner.**
///
/// Armed, every descent into a selected worker body is pushed a **second** time
/// with `enclosing_specialization: None`, so the same nested producer
/// `Construct` occurrences are also discovered as though they sat at a top-level
/// computational frame. Those discoveries intern real specializations, through
/// the real interning path, whose four source coordinates are identical to the
/// genuine ones and whose emission owner is `Predeclared` instead of
/// `Specialization`.
///
/// ⛔ This is the only way to instantiate the second owner, and that is the
/// measurement rather than a limitation of the instrument: production cannot
/// produce it. `continuation_result_origins` does not descend into
/// `Closure`/`LexicalClosure`, and every descent root is a closure's body child,
/// so for one `continuation_origin` the seed walk and each descent walk cover
/// **disjoint** occurrence subtrees. A producer `Construct` inside a worker body
/// is therefore reachable from exactly one discovery, and its emission owner is
/// a function of its source coordinates. Arming this hook removes precisely that
/// disjointness — nothing else — which is what makes the resulting refusal
/// attributable to the owner and not to some other damage.
/// `D2i` `AC-2` — suppress ONLY the post-ordinary-specialization descent.
///
/// Nothing else changes: the initial frontier is still seeded and admitted, so
/// a candidate that vanishes under this hook vanished because the descent root
/// is gone and not because discovery stopped.
/// `D2i` `AC-3` — present a second matching `StaticBody` edge to the uniqueness
/// decision, and change nothing else.

/// `D3` — suppress the binder-to-body resolution rule, and nothing else, so a
/// control can attribute its refusal to that rule rather than to a proxy.
#[cfg(test)]
pub(in crate::cranelift_backend) fn set_binder_body_resolution_suppressed(armed: bool) {
    SUPPRESS_BINDER_BODY_RESOLUTION.with(|cell| cell.set(armed));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn set_static_body_triple_duplicated(armed: bool) {
    DUPLICATE_STATIC_BODY_TRIPLE.with(|cell| cell.set(armed));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn set_post_specialization_descent_suppressed(armed: bool) {
    SUPPRESS_POST_SPECIALIZATION_DESCENT.with(|cell| cell.set(armed));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn set_continuation_descent_owner_duplication(armed: bool) {
    DUPLICATE_DESCENT_AS_TOP_LEVEL.with(|cell| cell.set(armed));
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2e` — what one binder of a checked
/// computational-match case IS.**
///
/// **The role a slot plays, which the derived environment did not carry.**
/// [`derive_case_producer_fact`]'s `ComputationalMatch` arm pushes
/// `argument_binders + recursive_positions.len()` entries and makes every one of
/// them `CaseProducerFact::open(origin)` — the **count** is right and the
/// **role** is absent. `RuntimeExpr::Var(index)` then indexes that environment,
/// so an induction hypothesis, an ordinary constructor child and a frame value
/// are indistinguishable in the derived fact.
///
/// That absence is why `build_continuation_specialization_plan` falls back to
/// the *syntactic* predicate — the argument at a recursive position must be a
/// `Closure` or `LexicalClosure` — and skips a `Var` that names the
/// compiler-minted hypothesis. This enum is what lets the role be **derived**
/// instead of searched for.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum CheckedCaseBinderRole {
    /// The compiler-minted induction hypothesis over the child at this
    /// **recursive source position**.
    ///
    /// The position is the one the *case* declared, never the binder's de
    /// Bruijn index — the two differ whenever a case has more than one
    /// recursive position, which is exactly the population a single-position
    /// witness cannot discriminate.
    InductionHypothesis { recursive_position: u32 },
    /// An ordinary constructor child binder, at its declaration-order field
    /// position.
    ConstructorChild { field_position: u32 },
    /// Outside this case's own binder run: the enclosing frame environment.
    FrameEnvironment,
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2e` — the checked case binder layout,
/// MEASURED rather than remembered.**
///
/// ## The measurement, and its population
///
/// The frame declined to pin this order, because which range occupies which de
/// Bruijn prefix is a property of the lowering rather than of the frame. It was
/// measured at **two independent induction-hypothesis construction sites** — the
/// carried computational-match arm and the specialized composed arm — by
/// instrumenting the assembled case environment and reading back the kind of
/// each binding:
///
/// ```text
/// carried arm, R3 before-hole under B-only exclusion, origins 5 and 18:
///   argument_binders=1 recursive_positions=[0]
///   Var(0) -> ComputationalRecursorClosure      Var(1) -> Carried child
///
/// specialized composed arm, PX8JSiblingTree::Node:
///   argument_binders=2 recursive_positions=[0, 1]
///   Var(0) -> IH for sibling_position=1         Var(1) -> IH for sibling_position=0
/// ```
///
/// ⇒ **Induction hypotheses occupy the LEADING prefix, and within it they run in
/// REVERSE `recursive_positions` order.** The children follow in declaration
/// order, then the frame environment.
///
/// **The reversal is the half a one-position witness cannot see**, because
/// forward and reversed coincide at length one. It is measured here only because
/// the two-sibling fixture exists; the `R3` witness alone would have left it
/// unmeasured and a "remembered" order would have had an even chance.
///
/// ## Single ownership is the GOAL, and it is not true yet
///
/// [`Self::for_case`] performs its `.rev()` once and stores the result **in de
/// Bruijn order**, so every consumer *of this type* indexes that vector and
/// none re-derives the order.
///
/// **That is a claim about this type's consumers, and it must not be read as a
/// claim about the program.** Production assembles the hypothesis prefix at
/// **four** sites in `cranelift_backend/lowering/core.rs`, each with its own
/// `case.recursive_positions.iter().rev()`:
///
/// | site | arm |
/// |---|---|
/// | `4939` | specialized composed |
/// | `5496` | recursor-layer construction |
/// | `7094` | source machine |
/// | `13708` | carried computational match |
///
/// All four predate this type. So there are five reversals, and the one here is
/// the only one nothing in production depends on.
///
/// **Do not read this as an instruction not to look.** A lowering change that
/// moves the prefix becomes a single-site correction **once the identity plane
/// adopts this type at those four sites** — until then it is a five-site
/// change, and the four above are where to start. They are named so the
/// relation is findable from either end; `core.rs:4939` carries the pointer
/// back.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct CheckedCaseBinderLayout {
    /// The recursive source position each induction-hypothesis slot stands for,
    /// **indexed by de Bruijn position**. Element `i` is the role of `Var(i)`.
    pub(super) induction_hypotheses: Vec<u32>,
    pub(super) argument_binders: u32,
}

#[cfg_attr(not(test), allow(dead_code))]
impl CheckedCaseBinderLayout {
    /// Derive the layout from the case's own checked declaration.
    ///
    /// Reads `recursive_positions` and `argument_binders` and nothing else —
    /// not the body, not a lowered shape, not a constructor spelling, not an
    /// arity recovered from a value.
    pub(in crate::cranelift_backend) fn for_case(
        case: &crate::RuntimeComputationalMatchCase,
    ) -> Result<Self, CraneliftBackendError> {
        let mut induction_hypotheses = Vec::with_capacity(case.recursive_positions.len());
        // THE MEASURED ORDER, and the only place it is spelled: the hypothesis
        // prefix runs in reverse declaration order.
        for position in case.recursive_positions.iter().rev().copied() {
            induction_hypotheses.push(u32::try_from(position).map_err(|_| {
                planner_capacity_error("checked case recursive position exhausted")
            })?);
        }
        Ok(Self {
            induction_hypotheses,
            argument_binders: u32::try_from(case.argument_binders).map_err(|_| {
                planner_capacity_error("checked case argument binder run exhausted")
            })?,
        })
    }

    /// What the binder at this de Bruijn index is.
    ///
    /// Total by construction: an index past this case's own binder run is the
    /// enclosing frame environment, which is a **role**, not an error. A `Var`
    /// reaching past the run is ordinary and must not be refused here.
    pub(in crate::cranelift_backend) fn role_at(&self, index: usize) -> CheckedCaseBinderRole {
        if let Some(recursive_position) = self.induction_hypotheses.get(index).copied() {
            return CheckedCaseBinderRole::InductionHypothesis { recursive_position };
        }
        let past_hypotheses = index - self.induction_hypotheses.len();
        if past_hypotheses < self.argument_binders as usize {
            return CheckedCaseBinderRole::ConstructorChild {
                field_position: past_hypotheses as u32,
            };
        }
        CheckedCaseBinderRole::FrameEnvironment
    }

    /// How many binders this case introduces before the enclosing environment.
    pub(in crate::cranelift_backend) fn binder_count(&self) -> usize {
        self.induction_hypotheses.len() + self.argument_binders as usize
    }
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2e` — the induction hypothesis a `Var`
/// occurrence names.**
///
/// Both members are planner-issued: the frame's own occurrence origin, and the
/// recursive source position the case declared. Neither is a constructor
/// spelling, a type, a row number, a runtime tag, nor a position recovered from
/// a lowered value.
///
/// The recursive position is the one the case declared and NOT the binder's de
/// Bruijn index. The two differ whenever a case declares more than one
/// recursive position, because the hypothesis prefix runs reversed -- see
/// [`CheckedCaseBinderLayout`], which is the only place that order is spelled.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct CheckedIhBinding {
    pub(super) frame_origin: StaticOriginId,
    pub(super) recursive_position: u32,
}

impl CheckedIhBinding {
    pub(in crate::cranelift_backend) fn new(
        frame_origin: StaticOriginId,
        recursive_position: u32,
    ) -> Self {
        Self {
            frame_origin,
            recursive_position,
        }
    }

    pub(in crate::cranelift_backend) fn frame_origin(self) -> StaticOriginId {
        self.frame_origin
    }

    pub(in crate::cranelift_backend) fn recursive_position(self) -> u32 {
        self.recursive_position
    }
}

/// What one binder in scope is, threaded down the occurrence tree.
///
/// This is the environment element [`derive_case_producer_fact`] never had. It
/// pushes `argument_binders + recursive_positions.len()` entries and makes every
/// one of them `CaseProducerFact::open(origin)`, so the count is right and the
/// role is absent. Carrying the role is what lets a `Var` be **resolved** to the
/// hypothesis it names rather than **recognised by shape**.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum CheckedBinderProvenance {
    InductionHypothesis(CheckedIhBinding),
    /// One ordinary constructor field introduced by a checked computational
    /// case. The frame and declared field position come directly from
    /// [`CheckedCaseBinderLayout`]; neither is inferred from the `Var` that
    /// later resolves to it.
    ConstructorChild {
        frame_origin: StaticOriginId,
        field_position: u32,
    },
    /// One parameter of an ordinary lexical closure.
    LexicalClosureParameter {
        closure_origin: StaticOriginId,
        parameter_ordinal: u32,
    },
    /// One captured value in an ordinary lexical closure body. The source
    /// occurrence is the capture expression evaluated outside the body.
    LexicalClosureCapture {
        closure_origin: StaticOriginId,
        capture_ordinal: u32,
        source_origin: StaticOriginId,
    },
    /// Every other binder: a `Let` value, an ordinary `Match` case binder, or
    /// a symbolic `Closure` parameter/capture whose source occurrence is not
    /// represented in the lexical child plane.
    Ordinary,
}

/// The result of resolving one `Var` through the exact forward binder walk.
///
/// `provenance` says which semantic binder the occurrence names. The immediate
/// environment index says where this occurrence reads that binder now. Keeping
/// both facts in one resolution prevents a later consumer from reconstructing
/// availability from semantic identity or from running a second binder walk.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct CheckedBinderResolution {
    pub(super) provenance: CheckedBinderProvenance,
    pub(super) immediate_environment_index: u32,
    #[cfg(feature = "px8-ds-test-support")]
    pub(super) preceding_environment_provenance: Option<CheckedBinderProvenance>,
}

fn resolve_checked_binder(
    environment: &[CheckedBinderProvenance],
    index: u32,
) -> Option<CheckedBinderResolution> {
    let immediate_environment_index = usize::try_from(index).ok()?;
    let provenance = environment.get(immediate_environment_index).copied()?;
    Some(CheckedBinderResolution {
        provenance,
        immediate_environment_index: index,
        #[cfg(feature = "px8-ds-test-support")]
        preceding_environment_provenance: immediate_environment_index
            .checked_sub(1)
            .and_then(|preceding| environment.get(preceding).copied()),
    })
}

/// Resolve every `Var` occurrence that names a compiler-minted induction
/// hypothesis, by threading the binder environment the lowering builds.
///
/// ## Why this is a derivation and not a search
///
/// The walk never asks what an expression *looks like*. It carries a binder
/// environment down the tree exactly as [`derive_case_producer_fact`] does --
/// same variants, same child positions, same fresh-environment rule at a
/// closure body -- and a `Var` is answered by **indexing** that environment. A
/// `Var` is classified as a hypothesis if and only if the binder it resolves to
/// was pushed as one by [`CheckedCaseBinderLayout`].
///
/// So the population is closed by construction: nothing is matched on
/// `RuntimeExpr::Var(0)`, on an argument position, or on a constructor symbol,
/// and an expression that merely resembles the witness cannot enter.
///
/// ## The two things a role-blind classifier gets wrong
///
/// A classifier keyed on de Bruijn *depth* misses an occurrence reached through
/// a `Let` or a nested `Match`, because those push binders and shift every
/// index below them. A classifier keyed on *position* misses a case with more
/// than one recursive position, because the hypothesis prefix is reversed.
/// Threading answers both without either key existing.
pub(super) fn derive_checked_ih_bindings(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
    environment: &[CheckedBinderProvenance],
    out: &mut BTreeMap<StaticOriginId, CheckedBinderResolution>,
) -> Result<(), CraneliftBackendError> {
    let expr = plan.planned_occurrence_expr(origin)?;
    let child = |position| plan.semantic.child_origin(origin, position);
    match expr {
        RuntimeExpr::Var(index) => {
            if let Some(resolution) = resolve_checked_binder(environment, *index) {
                out.insert(origin, resolution);
            }
        }
        RuntimeExpr::CheckedJoinSite { .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { .. }
        | RuntimeExpr::CheckedRecursiveInvocation { .. }
        | RuntimeExpr::CheckedComputationalIHSlots { .. }
        | RuntimeExpr::CheckedComputationalIHInvocation { .. }
        | RuntimeExpr::Project { .. } => {
            derive_checked_ih_bindings(plan, child(0)?, environment, out)?;
        }
        RuntimeExpr::Construct { args, .. } => {
            for position in 0..args.len() {
                derive_checked_ih_bindings(plan, child(position)?, environment, out)?;
            }
        }
        RuntimeExpr::PrimitiveCall { args, .. } => {
            for position in 0..args.len() {
                derive_checked_ih_bindings(plan, child(position)?, environment, out)?;
            }
        }
        RuntimeExpr::Record { fields } => {
            for position in 0..fields.len() {
                derive_checked_ih_bindings(plan, child(position)?, environment, out)?;
            }
        }
        RuntimeExpr::Let { .. } => {
            derive_checked_ih_bindings(plan, child(0)?, environment, out)?;
            // The value's binder shifts every index below it. This push is why a
            // hypothesis reached through a `Let` keeps its role.
            let mut nested = Vec::with_capacity(environment.len() + 1);
            nested.push(CheckedBinderProvenance::Ordinary);
            nested.extend_from_slice(environment);
            derive_checked_ih_bindings(plan, child(1)?, &nested, out)?;
        }
        RuntimeExpr::If { .. } => {
            for position in 0..3 {
                derive_checked_ih_bindings(plan, child(position)?, environment, out)?;
            }
        }
        RuntimeExpr::Match { cases, .. } => {
            derive_checked_ih_bindings(plan, child(0)?, environment, out)?;
            for (index, case) in cases.iter().enumerate() {
                let mut nested = Vec::with_capacity(case.binders + environment.len());
                nested.extend((0..case.binders).map(|_| CheckedBinderProvenance::Ordinary));
                nested.extend_from_slice(environment);
                derive_checked_ih_bindings(plan, child(1 + index)?, &nested, out)?;
            }
        }
        RuntimeExpr::ComputationalMatch { cases, .. } => {
            derive_checked_ih_bindings(plan, child(0)?, environment, out)?;
            for (index, case) in cases.iter().enumerate() {
                // The layout is the sole authority for which prefix is which and
                // for the order inside it. Nothing here re-derives either.
                let layout = CheckedCaseBinderLayout::for_case(case)?;
                let mut nested = Vec::with_capacity(layout.binder_count() + environment.len());
                for binder in 0..layout.binder_count() {
                    nested.push(match layout.role_at(binder) {
                        CheckedCaseBinderRole::InductionHypothesis { recursive_position } => {
                            CheckedBinderProvenance::InductionHypothesis(CheckedIhBinding {
                                frame_origin: origin,
                                recursive_position,
                            })
                        }
                        CheckedCaseBinderRole::ConstructorChild { field_position } => {
                            CheckedBinderProvenance::ConstructorChild {
                                frame_origin: origin,
                                field_position,
                            }
                        }
                        CheckedCaseBinderRole::FrameEnvironment => {
                            CheckedBinderProvenance::Ordinary
                        }
                    });
                }
                nested.extend_from_slice(environment);
                derive_checked_ih_bindings(plan, child(1 + index)?, &nested, out)?;
            }
        }
        RuntimeExpr::Closure {
            captures, params, ..
        } => {
            // A closure body does NOT see the enclosing environment by de Bruijn
            // index, so a hypothesis does not leak across the boundary. Building
            // a fresh environment here is what keeps that true; extending the
            // outer one would classify an unrelated parameter as a hypothesis.
            let mut body_environment = Vec::with_capacity(captures.len() + params.len());
            body_environment.extend(
                (0..captures.len() + params.len()).map(|_| CheckedBinderProvenance::Ordinary),
            );
            derive_checked_ih_bindings(plan, child(0)?, &body_environment, out)?;
        }
        RuntimeExpr::LexicalClosure {
            captures, params, ..
        } => {
            for position in 0..captures.len() {
                derive_checked_ih_bindings(plan, child(1 + position)?, environment, out)?;
            }
            let mut body_environment = Vec::with_capacity(captures.len() + params.len());
            for parameter_ordinal in 0..params.len() {
                body_environment.push(CheckedBinderProvenance::LexicalClosureParameter {
                    closure_origin: origin,
                    parameter_ordinal: u32::try_from(parameter_ordinal).map_err(|_| {
                        planner_capacity_error("lexical closure parameter ordinal exhausted")
                    })?,
                });
            }
            for capture_ordinal in 0..captures.len() {
                body_environment.push(CheckedBinderProvenance::LexicalClosureCapture {
                    closure_origin: origin,
                    capture_ordinal: u32::try_from(capture_ordinal).map_err(|_| {
                        planner_capacity_error("lexical closure capture ordinal exhausted")
                    })?,
                    source_origin: child(1 + capture_ordinal)?,
                });
            }
            derive_checked_ih_bindings(plan, child(0)?, &body_environment, out)?;
        }
        RuntimeExpr::Call { args, .. } => {
            derive_checked_ih_bindings(plan, child(0)?, environment, out)?;
            for position in 0..args.len() {
                derive_checked_ih_bindings(plan, child(1 + position)?, environment, out)?;
            }
        }
        RuntimeExpr::Effect {
            capability, args, ..
        } => {
            let child_count = args.len() + usize::from(capability.is_some());
            for position in 0..child_count {
                derive_checked_ih_bindings(plan, child(position)?, environment, out)?;
            }
        }
        RuntimeExpr::Trap(_)
        | RuntimeExpr::Value(_)
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. } => {}
    }
    Ok(())
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2g` — the resolved checked transport
/// coordinate at one occurrence.**
///
/// The Architect ruled this coordinate **required and never optional**, and
/// ruled it an exact resolved triple rather than a raw id: the enclosing
/// `CheckedSubcontinuationFrame`'s frame, the selected
/// `CheckedComputationalIHSlots` template with its checked occurrence path, and
/// the `CheckedComputationalIHInvocation` template with its path.
///
/// Every member is read off a marker the walk descended through. Nothing is
/// inferred from the Runtime shape, nothing selects "the only marker", and a
/// raw id is never accepted on its own -- the path travels with the id it was
/// declared beside, so a template used at the wrong location cannot present as
/// the right one.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct CheckedTransportCoordinate {
    pub(super) frame_id: u64,
    pub(super) slot_template_id: u64,
    pub(super) slot_occurrence_path: Vec<u64>,
    pub(super) call_template_id: u64,
    pub(super) call_occurrence_path: Vec<u64>,
}

impl CheckedTransportCoordinate {
    /// **`RT-LEXICAL-R3-FUSION-EMITTER` `D2`** — the consumer frame identity
    /// this coordinate already carries, exposed read-only.
    ///
    /// Read, never re-derived. The fused body re-enters **this** frame rather
    /// than looking one up, so the identity the claim was preflighted against
    /// is the identity the checked consumer then validates. Nothing new is
    /// planned for it and no second authority is introduced: the coordinate is
    /// already a member of the complete key the claim's identity came from.
    pub(in crate::cranelift_backend) fn frame_id(&self) -> u64 {
        self.frame_id
    }
}

/// What is in scope while descending; a member stays `None` until its marker is
/// crossed, and a coordinate is recorded only when all three are present.
#[derive(Clone, Debug, Default)]
pub(super) struct CheckedTransportScope {
    pub(super) frame_id: Option<u64>,
    pub(super) slot: Option<(u64, Vec<u64>)>,
    pub(super) invocation: Option<(u64, Vec<u64>)>,
}

/// Thread the checked wrapper authorities down the occurrence tree and
/// **resolve them through a validated oriented plan**.
///
/// The coordinate is **inherited**, not binder-scoped, so this descends every
/// child uniformly through the closed child inventory rather than spelling the
/// per-variant child positions a second time. Only the three marker variants
/// change what is in scope.
///
/// A raw wrapper value is NOT authority. A marker names a template id and the
/// checked occurrence path it was declared at; this resolves that pair against
/// the plan's own entries and then requires the three to be **related**: the
/// slot must belong to the frame in scope, and the call must be the call of
/// that slot. A marker whose id resolves but whose relationships do not is left
/// unresolved rather than recorded, so coincidence cannot present as authority.
///
/// A `CheckedComputationalIHSlots` marker declares one template per recursive
/// position. This threads the **selected** one -- the entry whose index matches
/// the recursive position being consumed -- rather than "the only" entry, which
/// is the existential shape the ruling forbids. With one declared position the
/// two coincide, which is exactly why the selection is written positionally
/// instead of by `first()`.
pub(super) fn derive_checked_transport(
    plan: &StaticTransitionPlan<'_>,
    oriented: &crate::OrientedSubcontinuationPlanV1,
    origin: StaticOriginId,
    scope: &CheckedTransportScope,
    out: &mut BTreeMap<StaticOriginId, CheckedTransportCoordinate>,
) -> Result<(), CraneliftBackendError> {
    let expr = plan.planned_occurrence_expr(origin)?;
    let mut scope = scope.clone();
    match expr {
        RuntimeExpr::CheckedSubcontinuationFrame { frame_id, .. } => {
            scope.frame_id = Some(*frame_id);
        }
        RuntimeExpr::CheckedComputationalIHSlots {
            slot_template_ids,
            checked_occurrence_paths,
            ..
        } => {
            if slot_template_ids.len() != checked_occurrence_paths.len() {
                return Err(planner_error(
                    "checked computational-IH slot marker identity and location arity differ",
                ));
            }
            // Positional selection, and only a single-entry marker is resolved
            // here: a marker declaring several templates needs the consuming
            // position to choose among them, and that arrives with the key in
            // `D2h`. Leaving it unresolved refuses rather than guesses.
            if let ([slot_template_id], [checked_occurrence_path]) =
                (&slot_template_ids[..], &checked_occurrence_paths[..])
            {
                scope.slot = Some((*slot_template_id, checked_occurrence_path.clone()));
            } else {
                scope.slot = None;
            }
        }
        RuntimeExpr::CheckedComputationalIHInvocation {
            call_template_id,
            checked_occurrence_path,
            ..
        } => {
            scope.invocation = Some((*call_template_id, checked_occurrence_path.clone()));
        }
        _ => {}
    }
    if let (Some(frame_id), Some((slot_id, slot_path)), Some((call_id, call_path))) =
        (&scope.frame_id, &scope.slot, &scope.invocation)
    {
        // Each member must resolve to a plan entry BY ITS PAIR -- id and the
        // checked occurrence path it was declared at -- so a template used at
        // the wrong location cannot answer for the right one.
        let frame = oriented
            .frames
            .iter()
            .find(|frame| frame.frame_id == *frame_id);
        let slot = oriented.computational_ih_slots.iter().find(|slot| {
            slot.slot_template_id == *slot_id && slot.checked_occurrence_path == *slot_path
        });
        let call = oriented.computational_ih_calls.iter().find(|call| {
            call.call_template_id == *call_id && call.checked_occurrence_path == *call_path
        });
        if let (Some(frame), Some(slot), Some(call)) = (frame, slot, call) {
            // The relationships, which are what make three resolved entries one
            // coordinate rather than three coincidences.
            let related = slot.frame_template_id == frame.frame_id
                && call.slot_template_id == slot.slot_template_id;
            if related {
                out.insert(
                    origin,
                    CheckedTransportCoordinate {
                        frame_id: frame.frame_id,
                        slot_template_id: slot.slot_template_id,
                        slot_occurrence_path: slot.checked_occurrence_path.clone(),
                        call_template_id: call.call_template_id,
                        call_occurrence_path: call.checked_occurrence_path.clone(),
                    },
                );
            }
        }
    }
    for child in plan.semantic.child_origins(origin)?.to_vec() {
        derive_checked_transport(plan, oriented, child, &scope, out)?;
    }
    Ok(())
}

/// Every occurrence at which a complete, plan-resolved checked transport
/// coordinate is in scope, from the same roots the IH binding uses.
#[cfg_attr(not(test), allow(dead_code))]
pub(super) fn build_checked_transport(
    plan: &StaticTransitionPlan<'_>,
    oriented: &crate::OrientedSubcontinuationPlanV1,
) -> Result<BTreeMap<StaticOriginId, CheckedTransportCoordinate>, CraneliftBackendError> {
    let mut out = BTreeMap::new();
    let scope = CheckedTransportScope::default();
    if let Some(root) = plan.root_occurrence {
        derive_checked_transport(plan, oriented, root, &scope, &mut out)?;
    }
    for origin in plan.declaration_occurrences.values().copied() {
        derive_checked_transport(plan, oriented, origin, &scope, &mut out)?;
    }
    Ok(out)
}

/// Every `Var` occurrence in the program that names an induction hypothesis.
///
/// Walked from the same roots [`build_case_emission_plan`] uses -- the program
/// root and every transparent declaration -- each with an empty environment,
/// because neither has binders in scope at its own occurrence.
#[cfg_attr(not(test), allow(dead_code))]
pub(super) fn build_checked_binder_provenance(
    plan: &StaticTransitionPlan<'_>,
) -> Result<BTreeMap<StaticOriginId, CheckedBinderResolution>, CraneliftBackendError> {
    let mut out = BTreeMap::new();
    if let Some(root) = plan.root_occurrence {
        derive_checked_ih_bindings(plan, root, &[], &mut out)?;
    }
    for origin in plan.declaration_occurrences.values().copied() {
        derive_checked_ih_bindings(plan, origin, &[], &mut out)?;
    }
    Ok(out)
}

pub(super) fn build_checked_ih_bindings(
    plan: &StaticTransitionPlan<'_>,
) -> Result<BTreeMap<StaticOriginId, CheckedIhBinding>, CraneliftBackendError> {
    Ok(build_checked_binder_provenance(plan)?
        .into_iter()
        .filter_map(|(origin, resolution)| match resolution.provenance {
            CheckedBinderProvenance::InductionHypothesis(binding) => Some((origin, binding)),
            CheckedBinderProvenance::ConstructorChild { .. }
            | CheckedBinderProvenance::LexicalClosureParameter { .. }
            | CheckedBinderProvenance::LexicalClosureCapture { .. }
            | CheckedBinderProvenance::Ordinary => None,
        })
        .collect())
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` — why one body receives no
/// standalone `Function`, as a closed typed reason rather than membership of
/// a bare origin set.**
///
/// The two reasons are **not interchangeable and must never merge**. A
/// continuation template's body is superseded by a *generated context* that
/// lowers the same body under a different environment; a fusion-owned body
/// is lowered inside a *fused definition* under the producer's own source
/// authority alongside a second unit's suffix. They differ in who lowers the
/// body, in what replaces the incoming edge, and in what a closeout must
/// biject against. Collapsing them into one `BTreeSet<StaticOriginId>` would
/// make "why is this body absent" unanswerable at exactly the point a
/// closeout has to answer it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum BodyEmissionDisposition {
    /// `D5a` — every selecting specialization retargeted to a generated
    /// context, so the raw worker body survives as a template only.
    ContinuationTemplate,
    /// `D2f` — this body is lowered inside the named fused definition. The
    /// identity is carried so the closeout can biject fusion-owned bodies
    /// against installed fusions rather than merely counting them.
    FusionOwned(StaticContinuationFusionId),
}

/// Read a computational match's scrutinee from its forward child authority.
pub(super) fn forward_match_scrutinee(
    plan: &StaticTransitionPlan<'_>,
    match_origin: StaticOriginId,
) -> Result<StaticOriginId, CraneliftBackendError> {
    let mut position_zero = occurrence_authority(plan, match_origin)?
        .children
        .iter()
        .filter(|child| child.position == 0);
    let Some(scrutinee) = position_zero.next() else {
        return Err(planner_error(
            "a computational match has no position-zero occurrence authority",
        ));
    };
    if position_zero.next().is_some() {
        return Err(planner_error(
            "a computational match has two position-zero occurrence authorities",
        ));
    }
    Ok(scrutinee.origin)
}

/// Seed every initial discovery by walking source children forward from the
/// root and declarations.
///
/// When a computational match's scrutinee is itself a computational match, the
/// outer occurrence is still in hand. That is the only moment this function
/// records the outer eliminator and each ordinal-selected case body. The seed
/// then travels on the discovery; there is no continuation-keyed table, parent
/// map, or later occurrence scan.
pub(super) fn initial_continuation_discoveries(
    plan: &StaticTransitionPlan<'_>,
) -> Result<Vec<ContinuationDiscovery>, CraneliftBackendError> {
    let mut roots = plan
        .declaration_occurrences
        .values()
        .copied()
        .map(|origin| (origin, None, None))
        .collect::<Vec<_>>();
    if let Some(root) = plan.root_occurrence {
        roots.push((root, None, None));
    }

    let mut walked = BTreeSet::new();
    let mut pending = Vec::new();
    while let Some((origin, consuming_occurrences, required_consuming_occurrence)) = roots.pop() {
        if !walked.insert(origin) {
            return Err(planner_error(
                "the forward continuation seed walk reached one source occurrence twice",
            ));
        }
        let expr = plan.planned_occurrence_expr(origin)?;
        let children = plan.semantic.child_origins(origin)?.to_vec();
        if let RuntimeExpr::ComputationalMatch { cases, .. } = expr {
            let scrutinee = forward_match_scrutinee(plan, origin)?;
            if children.first().copied() != Some(scrutinee) {
                return Err(planner_error(
                    "a computational match's semantic scrutinee disagrees with its position-zero \
                     occurrence authority",
                ));
            }
            pending.push(ContinuationDiscovery {
                continuation_origin: origin,
                result_root: scrutinee,
                enclosing_specialization: None,
                consuming_occurrences: consuming_occurrences.clone(),
                required_consuming_occurrence,
            });

            let mut candidates = Vec::with_capacity(cases.len());
            for alternative in 0..cases.len() {
                let body_origin = plan.semantic.child_origin(origin, 1 + alternative)?;
                #[cfg(test)]
                let body_origin = if MUTATE_CONTINUATION_CONSUMING_OCCURRENCE_SEED
                    .with(Cell::get)
                    == Some(ContinuationConsumingOccurrenceSeedMutation::BodyOrigin)
                {
                    // The exact wrong relation from AC-2: the continuation's
                    // own occurrence in place of the outer selected case body.
                    scrutinee
                } else {
                    body_origin
                };
                candidates.push(ContinuationConsumingOccurrenceSeed {
                    alternative: u32::try_from(alternative).map_err(|_| {
                        planner_capacity_error("outer consuming alternative exhausted")
                    })?,
                    occurrence: ContinuationConsumingOccurrence {
                        body_origin,
                        eliminator_origin: origin,
                    },
                });
            }

            // Only the direct position-zero child receives this outer
            // relation. Every case body is walked independently with no
            // inherited parent relation.
            let required_consuming_occurrence = consuming_occurrences
                .map(ContinuationRequiredConsumingOccurrence::Source)
                .or_else(|| {
                    Some(ContinuationRequiredConsumingOccurrence::Source(
                        ContinuationConsumingOccurrenceSeeds {
                            candidates: candidates.clone(),
                        },
                    ))
                });
            roots.push((
                scrutinee,
                Some(ContinuationConsumingOccurrenceSeeds { candidates }),
                required_consuming_occurrence,
            ));
            for child in children.into_iter().skip(1) {
                roots.push((child, None, None));
            }
        } else {
            roots.extend(children.into_iter().map(|child| (child, None, None)));
        }
    }

    pending.sort();
    Ok(pending)
}

pub(super) fn continuation_result_constructor_identities(
    plan: &StaticTransitionPlan<'_>,
    result_root: StaticOriginId,
) -> Result<Vec<ConstructorIdentity>, CraneliftBackendError> {
    let mut identities = Vec::new();
    for origin in continuation_result_origins(plan, result_root)? {
        if !matches!(
            plan.planned_occurrence_expr(origin)?,
            RuntimeExpr::Construct { .. }
        ) {
            continue;
        }
        let identity = plan.constructor_symbol_identity(origin)?;
        if !identities.contains(&identity) {
            identities.push(identity);
        }
    }
    Ok(identities)
}

/// Select the one forward-seeded outer case consumed by this inner
/// specialization alternative.
pub(super) fn consuming_occurrence_from_seed(
    plan: &StaticTransitionPlan<'_>,
    discovery: &ContinuationDiscovery,
    alternative: usize,
) -> Result<Option<ContinuationConsumingOccurrence>, CraneliftBackendError> {
    let Some(seeds) = &discovery.consuming_occurrences else {
        return Ok(None);
    };
    let selected_case_body = plan
        .semantic
        .child_origin(discovery.continuation_origin, 1 + alternative)?;
    let produced = continuation_result_constructor_identities(plan, selected_case_body)?;
    let mut matching = Vec::new();
    for candidate in &seeds.candidates {
        let identity = plan.case_constructor_identity(
            candidate.occurrence.eliminator_origin,
            candidate.alternative as usize,
        )?;
        if produced.contains(&identity) && !matching.contains(&candidate.occurrence) {
            matching.push(candidate.occurrence);
        }
    }
    #[cfg(test)]
    if MUTATE_CONTINUATION_CONSUMING_OCCURRENCE_SEED.with(Cell::get)
        == Some(ContinuationConsumingOccurrenceSeedMutation::EliminatorOrigin)
    {
        let selected = match matching.as_slice() {
            [only] => Some(*only),
            _ => None,
        };
        return Ok(selected.map(|occurrence| ContinuationConsumingOccurrence {
            body_origin: occurrence.body_origin,
            // Selection has already closed against the real outer match.
            // Replacing only its coordinate with the real inner match keeps
            // the body and selection axes fixed and fires step 1's guard.
            eliminator_origin: discovery.continuation_origin,
        }));
    }
    Ok(match matching.as_slice() {
        [only] => Some(*only),
        _ => None,
    })
}

/// Select the outer consumer already carried for this discovery's producers.
///
/// Source candidates are selected through their own eliminator's forward
/// position-zero continuation. Generated descents have already performed that
/// selection and therefore carry an exact occurrence.
pub(super) fn required_consuming_occurrence_for_alternative(
    plan: &StaticTransitionPlan<'_>,
    discovery: &ContinuationDiscovery,
    alternative: usize,
) -> Result<Option<ContinuationConsumingOccurrence>, CraneliftBackendError> {
    let Some(required) = &discovery.required_consuming_occurrence else {
        return Ok(None);
    };
    let seeds = match required {
        ContinuationRequiredConsumingOccurrence::Source(seeds) => seeds,
        ContinuationRequiredConsumingOccurrence::Exact(occurrence) => {
            return Ok(Some(*occurrence));
        }
    };
    let Some(first) = seeds.candidates.first() else {
        return Ok(None);
    };
    let eliminator_origin = first.occurrence.eliminator_origin;
    if seeds
        .candidates
        .iter()
        .any(|candidate| candidate.occurrence.eliminator_origin != eliminator_origin)
    {
        return Err(planner_error(
            "one required-consumer source relation names two outer eliminators",
        ));
    }
    let continuation_origin = forward_match_scrutinee(plan, eliminator_origin)?;
    let selected_case_body = plan
        .semantic
        .child_origin(continuation_origin, 1 + alternative)?;
    let produced = continuation_result_constructor_identities(plan, selected_case_body)?;
    let mut matching = Vec::new();
    for candidate in &seeds.candidates {
        let identity = plan.case_constructor_identity(
            candidate.occurrence.eliminator_origin,
            candidate.alternative as usize,
        )?;
        if produced.contains(&identity) && !matching.contains(&candidate.occurrence) {
            matching.push(candidate.occurrence);
        }
    }
    Ok(match matching.as_slice() {
        [only] => Some(*only),
        _ => None,
    })
}

pub(super) fn derive_required_consumer_occurrence(
    plan: &StaticTransitionPlan<'_>,
    key: &ContinuationSpecializationKey,
) -> Result<Option<ContinuationConsumingOccurrence>, CraneliftBackendError> {
    let Some(source_level) = key.consuming_occurrence else {
        return Ok(None);
    };
    if rederive_consuming_occurrence(plan, key, source_level)? != Some(source_level) {
        return Err(planner_error(
            "a required-consumer derivation starts from an invalid source-level occurrence",
        ));
    }
    let produced = continuation_result_constructor_identities(plan, source_level.body_origin)?;
    let mut matching = Vec::new();
    for occurrence in plan.source_occurrences.iter().flatten() {
        let RuntimeExpr::ComputationalMatch { cases, .. } = occurrence.expr else {
            continue;
        };
        if forward_match_scrutinee(plan, occurrence.static_origin)?
            != source_level.eliminator_origin
        {
            continue;
        }
        for alternative in 0..cases.len() {
            let identity = plan.case_constructor_identity(occurrence.static_origin, alternative)?;
            if produced.contains(&identity) {
                let candidate = ContinuationConsumingOccurrence {
                    body_origin: plan
                        .semantic
                        .child_origin(occurrence.static_origin, 1 + alternative)?,
                    eliminator_origin: occurrence.static_origin,
                };
                if !matching.contains(&candidate) {
                    matching.push(candidate);
                }
            }
        }
    }
    Ok(match matching.as_slice() {
        [only] => Some(*only),
        [] => Some(source_level),
        _ => {
            return Err(planner_error(
                "more than one outer consumer is derivable for one continuation call level",
            ));
        }
    })
}

/// Re-derive a claimed consuming occurrence without reading the forward seed.
///
/// The claim supplies only the outer eliminator coordinate. Its position-zero
/// child must be this continuation, and the selected outer body is read again
/// by ordinal from that eliminator after matching the inner selected body's
/// result constructor. This is the independent half of the relation check.
///
/// At this base, the position-zero relation is injective by construction:
/// `plan_expr` plans every source child separately, and every visit mints a
/// fresh append-only node identity through `push_node`. A second match therefore
/// cannot reuse this continuation occurrence as its own position-zero child.
pub(super) fn rederive_consuming_occurrence(
    plan: &StaticTransitionPlan<'_>,
    key: &ContinuationSpecializationKey,
    claimed: ContinuationConsumingOccurrence,
) -> Result<Option<ContinuationConsumingOccurrence>, CraneliftBackendError> {
    if forward_match_scrutinee(plan, claimed.eliminator_origin)? != key.continuation_origin {
        return Ok(None);
    }
    let selected_case_body = plan.semantic.child_origin(
        key.continuation_origin,
        1 + key.producer_alternative as usize,
    )?;
    let produced = continuation_result_constructor_identities(plan, selected_case_body)?;
    let RuntimeExpr::ComputationalMatch { cases, .. } =
        plan.planned_occurrence_expr(claimed.eliminator_origin)?
    else {
        return Ok(None);
    };
    let mut matching = Vec::new();
    for alternative in 0..cases.len() {
        let identity =
            plan.case_constructor_identity(claimed.eliminator_origin, alternative)?;
        if produced.contains(&identity) {
            let occurrence = ContinuationConsumingOccurrence {
                body_origin: plan
                    .semantic
                    .child_origin(claimed.eliminator_origin, 1 + alternative)?,
                eliminator_origin: claimed.eliminator_origin,
            };
            if !matching.contains(&occurrence) {
                matching.push(occurrence);
            }
        }
    }
    Ok(match matching.as_slice() {
        [only] => Some(*only),
        _ => None,
    })
}

pub(super) fn validate_continuation_consuming_occurrences(
    plan: &StaticTransitionPlan<'_>,
    units: &[PlannedContinuationSpecialization],
) -> Result<(), CraneliftBackendError> {
    for unit in units {
        let Some(claimed) = unit.key.consuming_occurrence else {
            continue;
        };
        if rederive_consuming_occurrence(plan, &unit.key, claimed)? != Some(claimed) {
            #[cfg(test)]
            {
                let reason = if forward_match_scrutinee(plan, claimed.eliminator_origin)?
                    != unit.key.continuation_origin
                {
                    "a continuation specialization's consuming occurrence has a mismatched \
                     eliminator_origin: it does not select the continuation as its position-zero \
                     child"
                } else {
                    "a continuation specialization's consuming occurrence has a mismatched \
                     body_origin: it is not the exact outer selected case body derived from its \
                     eliminator"
                };
                return Err(planner_error(reason));
            }
            #[cfg(not(test))]
            return Err(planner_error(
                "a continuation specialization's consuming occurrence is not the exact outer \
                 selected case body derived from its eliminator",
            ));
        }
    }
    Ok(())
}

pub(super) fn build_continuation_specialization_plan(
    plan: &StaticTransitionPlan<'_>,
) -> Result<
    (
        Vec<PlannedContinuationSpecialization>,
        Vec<PlannedContinuationSpecializationCall>,
        BTreeMap<ContinuationCallIdentity, RequiredConsumerProjection>,
        Vec<PlannedContinuationContext>,
        Vec<AdmittedContinuationDiscovery>,
    ),
    CraneliftBackendError,
> {
    let mut admitted: Vec<AdmittedContinuationDiscovery> = Vec::new();
    let mut pending = initial_continuation_discoveries(plan)?;
    let computational_count = pending.len();
    let bound = plan
        .source_occurrences
        .len()
        .checked_mul(computational_count.saturating_add(1))
        .and_then(|count| count.checked_add(1))
        .ok_or_else(|| planner_capacity_error("continuation fixed-point bound exhausted"))?;
    let mut steps = 0usize;
    let mut visited = BTreeSet::new();
    let mut interned = BTreeMap::new();
    let mut units: Vec<PlannedContinuationSpecialization> = Vec::new();
    let mut calls = BTreeSet::new();
    let mut required_consumer_projections = BTreeMap::new();
    let mut pending_required_consumer_projections = Vec::new();
    let mut sequences = BTreeMap::<
        (PredeclaredFunctionId, StaticOriginId, StaticOriginId),
        u32,
    >::new();
    while let Some(discovery) = pending.pop() {
        steps = steps
            .checked_add(1)
            .ok_or_else(|| planner_capacity_error("continuation fixed point exhausted"))?;
        if steps > bound {
            return Err(planner_error(
                "continuation specialization fixed point did not terminate",
            ));
        }
        #[cfg(test)]
        if WEAKEN_CONTINUATION_DECREASING_MEASURE.with(Cell::get) {
            // Compile-preserving AC-5 mutation: the active item is returned to
            // the frontier without entering the finite seen set.
            pending.push(discovery.clone());
        } else if !visited.insert(discovery.clone()) {
            continue;
        }
        #[cfg(not(test))]
        if !visited.insert(discovery.clone()) {
            continue;
        }
        // The ledger entry, written only where the production fixed point has
        // already admitted this item. Nothing else writes here.
        admitted.push(AdmittedContinuationDiscovery {
            continuation_origin: discovery.continuation_origin,
            result_root: discovery.result_root,
            enclosing_specialization: discovery.enclosing_specialization,
        });

        let continuation = plan.planned_occurrence_expr(discovery.continuation_origin)?;
        let RuntimeExpr::ComputationalMatch { cases, .. } = continuation else {
            return Err(planner_error(
                "continuation discovery names a non-computational match",
            ));
        };
        let consumer_owner = occurrence_authority(plan, discovery.continuation_origin)?.owner;
        for producer_construct_origin in
            continuation_result_origins(plan, discovery.result_root)?
        {
            let producer = plan.planned_occurrence_expr(producer_construct_origin)?;
            let RuntimeExpr::Construct { args, .. } = producer else {
                continue;
            };
            let identity = plan.constructor_symbol_identity(producer_construct_origin)?;
            let producer_owner = occurrence_authority(plan, producer_construct_origin)?.owner;
            for (alternative, case) in cases.iter().enumerate() {
                if plan.case_constructor_identity(discovery.continuation_origin, alternative)?
                    != identity
                {
                    continue;
                }
                // `D2b` — the closed projection, built ONCE from the checked
                // set and validated before any unit is interned. Every member
                // must name a real constructor field; a position outside the
                // argument run would leave the envelope omitting a field that
                // does not exist and the slot reconciliation reporting a length
                // disagreement that says nothing about the real fault.
                let mut checked_recursive_positions = BTreeSet::new();
                for position in case.recursive_positions.iter().copied() {
                    if position >= args.len() {
                        // The computational-match validator owns the malformed
                        // -position diagnostic; dormant planning must not
                        // preempt it, so this position is not projected.
                        continue;
                    }
                    let encoded = u32::try_from(position).map_err(|_| {
                        planner_capacity_error("continuation recursive position exhausted")
                    })?;
                    if !checked_recursive_positions.insert(encoded) {
                        return Err(planner_error(
                            "a checked case names one recursive source position twice, so the \
                             closed projection is not unique by source position",
                        ));
                    }
                }
                for position in case.recursive_positions.iter().copied() {
                    let Some(candidate) = args.get(position) else {
                        // This is not a specialization candidate. The existing
                        // computational-match validator remains the authority
                        // for the malformed-position diagnostic; dormant
                        // planning must not preempt it.
                        continue;
                    };
                    if !matches!(
                        candidate,
                        RuntimeExpr::Closure { .. } | RuntimeExpr::LexicalClosure { .. }
                    ) {
                        continue;
                    }
                    let closure_origin = plan
                        .semantic
                        .child_origin(producer_construct_origin, position)?;
                    let worker = build_continuation_worker_provenance(
                        plan,
                        discovery.continuation_origin,
                        producer_construct_origin,
                        position,
                        closure_origin,
                    )?;
                    let ordinary_parameters = exact_continuation_ordinary_parameters(
                        plan,
                        producer_construct_origin,
                        args,
                        position,
                        &checked_recursive_positions,
                        &worker,
                    )?;
                    let Some(producer_environment) = exact_continuation_source_environment(
                        plan,
                        producer_owner,
                        discovery.result_root,
                        producer_construct_origin,
                        consumer_owner,
                        discovery.continuation_origin,
                    )?
                    else {
                        // ⛔⛔ **`D7` checkpoint `1c` HARD STOP -- this `continue`
                        // is the omission site, and it is NOT closed.**
                        //
                        // The candidate above is already a derived
                        // recursive-position worker with a built provenance.
                        // Dropping it here removes it from the `#26`
                        // result-flow population, so its closure has nothing to
                        // be claimed by and reaches the late generic `Closure`
                        // arm -- which is exactly the fall-through the frame's
                        // matrix-omission law forbids, and exactly the framed
                        // witness (closure `381` / body `375`, `Vis` parent
                        // `386`, constructor-field position `1`).
                        //
                        // ⛔ **Refusing here with existing authority is not
                        // possible, and that is a measurement, not a
                        // preference.** Scoping the refusal to a real planned
                        // member -- the only planner-issued discriminator there
                        // is, `AbiUnitDefinition::ClosureBody`'s defining
                        // occurrence -- reds 23 green `ken-runtime` fixtures.
                        // Every one of them is in the SAME state as the witness:
                        // a real member, at a recursive position of an exact
                        // continuation producer, declined for an open source
                        // value environment, with zero interned
                        // specializations. The one candidate that IS separable
                        // -- `contspec_open_and_ambiguous_sources_refuse_only_
                        // the_candidate`'s closure `13`, declined on one edge
                        // and interned on another -- is separable by a test that
                        // does not separate the other twenty-two.
                        //
                        // ⇒ The narrowing that would admit the green fixtures
                        // and refuse `381` needs a predicate this plane does not
                        // have. The frame calls a third production predicate a
                        // hard stop; the concrete edge is returned instead of
                        // widened. The member-population law below IS enforced,
                        // and it is the converse direction of the same law.
                        continue;
                    };
                    let emission_owner = match discovery.enclosing_specialization {
                        Some(enclosing) => ContinuationEmissionOwner::Specialization(enclosing),
                        None => ContinuationEmissionOwner::Predeclared(
                            producer_environment.producer_owner,
                        ),
                    };
                    // The immediate-availability resolution, taken from the
                    // SAME enclosing specialization that settled the emission
                    // owner above. ⛔ Owned before the key is built: the
                    // enclosing unit is read out of `units` here so the
                    // immutable borrow ends before `intern_specialization`
                    // takes it mutably.
                    let enclosing_context = match discovery.enclosing_specialization {
                        None => None,
                        Some(enclosing) => {
                            let enclosing_unit =
                                units.get(enclosing.0 as usize).ok_or_else(|| {
                                    planner_error(
                                        "a descent names an enclosing specialization that was \
                                         never interned",
                                    )
                                })?;
                            Some((
                                enclosing,
                                // ⭐ The body origin half of the pair contexts
                                // are interned on, taken from the enclosing
                                // unit's own key so the frame identity is the
                                // interning key rather than a restatement of it.
                                enclosing_unit.key.worker.body_origin,
                                generated_context_parameters(&enclosing_unit.key.worker)?,
                                enclosing_unit.key.continuation_inputs.clone(),
                            ))
                        }
                    };
                    let emitter = match &enclosing_context {
                        None => ContinuationEmitterFrame::Predeclared(
                            producer_environment.producer_owner,
                        ),
                        Some((
                            enclosing,
                            worker_body_origin,
                            context_parameters,
                            enclosing_inputs,
                        )) => ContinuationEmitterFrame::GeneratedContext {
                            enclosing: *enclosing,
                            worker_body_origin: *worker_body_origin,
                            context_parameters: *context_parameters,
                            enclosing_inputs,
                        },
                    };
                    let required_consuming_occurrence =
                        required_consuming_occurrence_for_alternative(
                            plan,
                            &discovery,
                            alternative,
                        )?;
                    let key = ContinuationSpecializationKey {
                        producer_owner: producer_environment.producer_owner,
                        emission_owner,
                        producer_result_origin: producer_environment.producer_result_origin,
                        producer_construct_origin: producer_environment.producer_construct_origin,
                        producer_alternative: u32::try_from(alternative).map_err(|_| {
                            planner_capacity_error("continuation alternative exhausted")
                        })?,
                        consumer_owner: producer_environment.consumer_owner,
                        consuming_occurrence: consuming_occurrence_from_seed(
                            plan,
                            &discovery,
                            alternative,
                        )?,
                        continuation_origin: discovery.continuation_origin,
                        recursive_position: u32::try_from(position).map_err(|_| {
                            planner_capacity_error("continuation recursive position exhausted")
                        })?,
                        recursive_positions: checked_recursive_positions.clone(),
                        worker: worker.clone(),
                        ordinary_parameters,
                        continuation_inputs: exact_continuation_projection(
                            plan,
                            &producer_environment,
                            ordinary_parameters,
                            &emitter,
                        )?,
                    };
                    let (target, inserted) =
                        intern_specialization(&mut interned, &mut units, key)?;
                    let sequence_key = (
                        producer_owner,
                        discovery.result_root,
                        producer_construct_origin,
                    );
                    let sequence = sequences.entry(sequence_key).or_insert(0);
                    let token = ContinuationSpecializationCallToken {
                        producer_owner,
                        emission_owner,
                        producer_result_origin: discovery.result_root,
                        producer_construct_origin,
                        producer_alternative: u32::try_from(alternative).map_err(|_| {
                            planner_capacity_error("continuation alternative exhausted")
                        })?,
                        call_site_sequence: *sequence,
                        target,
                        worker: worker.clone(),
                    };
                    let identity = ContinuationCallIdentity {
                        token: token.clone(),
                        recursive_position: u32::try_from(position).map_err(|_| {
                            planner_capacity_error("continuation recursive position exhausted")
                        })?,
                    };
                    #[cfg(test)]
                    if required_consuming_occurrence.is_none() {
                        CONTINUATION_REQUIRED_CONSUMER_OBSERVATIONS.with(|observations| {
                            observations.borrow_mut().push(
                                ContinuationRequiredConsumerObservation {
                                    continuation_origin: discovery.continuation_origin,
                                    result_root: discovery.result_root,
                                    required: None,
                                    derived_at_consumer: None,
                                    child_push: false,
                                    projection_disposition: Some(
                                        RequiredConsumerProjectionDisposition::
                                            AbsentNoRequiredConsumer,
                                    ),
                                },
                            );
                        });
                    }
                    if let Some(required) = required_consuming_occurrence {
                        pending_required_consumer_projections.push((
                            identity,
                            required,
                            discovery.continuation_origin,
                            discovery.result_root,
                        ));
                    }
                    let call = PlannedContinuationSpecializationCall { token };
                    if calls.insert(call) {
                        *sequence = sequence.checked_add(1).ok_or_else(|| {
                            planner_capacity_error("continuation call sequence exhausted")
                        })?;
                    }
                    if inserted {
                        // The key is already interned when its body can add work.
                        //
                        // ⭐ `D5a`: the descent now carries `target` as the
                        // enclosing generated emission context. Everything the
                        // nested producer will need — the continuation inputs
                        // this specialization already holds — lives in that
                        // context, and the raw worker body's own owner cannot
                        // reach them. Dropping it here is exactly the defect
                        // `evt_609am4v7cdt5b` ruled on.
                        // `D2i` `AC-2` causal control: suppressing ONLY this
                        // descent must take the fusion candidate count 1 -> 0
                        // while the initial terminal root stays admitted. See
                        // `set_continuation_post_specialization_descent_suppressed`.
                        #[cfg(test)]
                        let descend = !SUPPRESS_POST_SPECIALIZATION_DESCENT.with(Cell::get);
                        #[cfg(not(test))]
                        let descend = true;
                        if descend {
                            // `RT-CONSUMER-CARRY-CONTROL-DEBT` `C1`: these are
                            // constructional pins, not checks against a second
                            // authority. Production interning either returns a
                            // full-key-equal installed unit or inserts this key
                            // at the returned index. Test-only aliasing returns
                            // `inserted = false`, so it cannot enter this block.
                            // Neither refusal is reachable through the current
                            // interner; they protect only a future writer change
                            // that breaks that construction contract.
                            let target_unit = units.get(target.0 as usize).ok_or_else(|| {
                                planner_error(
                                    "a descent target was not installed before its child",
                                )
                            })?;
                            if target_unit.key.worker != worker {
                                return Err(planner_error(
                                    "a descent target names a different worker than the child push",
                                ));
                            }
                            let required_consuming_occurrence = target_unit
                                .key
                                .consuming_occurrence
                                .map(ContinuationRequiredConsumingOccurrence::Exact);
                            #[cfg(test)]
                            if let Some(ContinuationRequiredConsumingOccurrence::Exact(required)) =
                                required_consuming_occurrence.as_ref()
                            {
                                CONTINUATION_REQUIRED_CONSUMER_OBSERVATIONS.with(
                                    |observations| {
                                        observations.borrow_mut().push(
                                            ContinuationRequiredConsumerObservation {
                                                continuation_origin: discovery.continuation_origin,
                                                result_root: worker.body_origin,
                                                required: Some(*required),
                                                derived_at_consumer: None,
                                                child_push: true,
                                                projection_disposition: None,
                                            },
                                        );
                                    },
                                );
                            }
                            pending.push(ContinuationDiscovery {
                                continuation_origin: discovery.continuation_origin,
                                result_root: worker.body_origin,
                                enclosing_specialization: Some(target),
                                consuming_occurrences: discovery.consuming_occurrences.clone(),
                                required_consuming_occurrence,
                            });
                        }
                        // `D8a` — the same descent, as though it were top level.
                        // See `set_continuation_descent_owner_duplication`.
                        //
                        // `RT-CONSUMER-CARRY-CONTROL-DEBT` `C5` disposition:
                        // leave the inherited required consumer unchanged. This
                        // test-only twin mutates the emission-owner axis; unlike
                        // the real descent above, it does not advance the carry.
                        // It is not evidence for descent-carry behaviour, and
                        // coupling that second mutation axis into this twin
                        // would weaken its original discriminator.
                        #[cfg(test)]
                        if DUPLICATE_DESCENT_AS_TOP_LEVEL.with(Cell::get) {
                            pending.push(ContinuationDiscovery {
                                continuation_origin: discovery.continuation_origin,
                                result_root: worker.body_origin,
                                enclosing_specialization: None,
                                consuming_occurrences: discovery.consuming_occurrences.clone(),
                                required_consuming_occurrence: discovery
                                    .required_consuming_occurrence
                                    .clone(),
                            });
                        }
                    }
                }
            }
        }
    }
    let calls = calls.into_iter().collect::<Vec<_>>();
    // Preserve the source-level certificate's established refusal priority.
    // The required relation is derived only after that independently checked
    // certificate has passed; otherwise a source mutation would be mislabeled
    // as a required-consumer failure.
    validate_continuation_consuming_occurrences(plan, &units)?;
    validate_continuation_specialization_closure(&interned, &units, &calls)?;
    for (identity, required, _continuation_origin, _result_root) in
        pending_required_consumer_projections
    {
        let target_unit = units.get(identity.target().0 as usize).ok_or_else(|| {
            planner_error("a required-consumer projection names an uninstalled target")
        })?;
        let derived_at_consumer = derive_required_consumer_occurrence(plan, &target_unit.key)?;
        if derived_at_consumer != Some(required) {
            return Err(planner_error(
                "a continuation call's required consumer does not match the consumer-level \
                 occurrence independently derived from its target",
            ));
        }
        let source = target_unit.key.consuming_occurrence.ok_or_else(|| {
            planner_error(
                "a required-consumer projection's target has no source-level consuming occurrence",
            )
        })?;
        #[cfg(test)]
        let projection_minted = required != source;
        if required != source {
            let projection = RequiredConsumerProjection { source, required };
            if required_consumer_projections
                .insert(identity, projection)
                .is_some_and(|prior| prior != projection)
            {
                return Err(planner_error(
                    "one continuation call identity claims two required consumers",
                ));
            }
        }
        #[cfg(test)]
        CONTINUATION_REQUIRED_CONSUMER_OBSERVATIONS.with(|observations| {
            let projection_disposition = if projection_minted {
                RequiredConsumerProjectionDisposition::Minted
            } else {
                RequiredConsumerProjectionDisposition::SkippedRequiredEqualsSource
            };
            observations
                .borrow_mut()
                .push(ContinuationRequiredConsumerObservation {
                    continuation_origin: _continuation_origin,
                    result_root: _result_root,
                    required: Some(required),
                    derived_at_consumer,
                    child_push: false,
                    projection_disposition: Some(projection_disposition),
                });
        });
    }
    #[cfg(test)]
    if let Some(mutation) = REQUIRED_CONSUMER_PROJECTION_MUTATION.with(Cell::get) {
        if let Some(projection) = required_consumer_projections.values_mut().next() {
            match mutation {
                RequiredConsumerProjectionMutation::BodyOrigin => {
                    projection.required.body_origin = projection.source.body_origin;
                }
                RequiredConsumerProjectionMutation::EliminatorOrigin => {
                    projection.required.eliminator_origin = projection.source.eliminator_origin;
                }
            }
            REQUIRED_CONSUMER_PROJECTION_MUTATION_APPLICATIONS
                .with(|applications| applications.set(applications.get() + 1));
        }
    }
    validate_required_consumer_projections(
        plan,
        &units,
        &calls,
        &required_consumer_projections,
    )?;
    #[cfg(feature = "px8-ds-test-support")]
    let contexts = if checked_ih_generated_entry_context_permutation_is_active() {
        let mut context_calls = calls.clone();
        context_calls.reverse();
        intern_generated_contexts(&units, &context_calls)?
    } else {
        intern_generated_contexts(&units, &calls)?
    };
    #[cfg(not(feature = "px8-ds-test-support"))]
    let contexts = intern_generated_contexts(&units, &calls)?;
    Ok((
        units,
        calls,
        required_consumer_projections,
        contexts,
        admitted,
    ))
}

/// The `Parameter` run of the generated context that executes one specialization
/// worker: the raw worker's declared arity plus its capture count.
///
/// ⭐ Read off the **worker provenance**, not off the raw unit's ABI descriptor.
/// That is deliberate and it is the same pair the caller already supplies:
/// `call_static_worker` builds its operands as `declared_arity` explicit
/// arguments followed by the stored captures, so a context whose parameter run
/// is derived from the same two numbers accepts that exact operand prefix by
/// construction. Deriving it from the raw descriptor instead would make the
/// context's shape depend on a second authority that the call site never reads.
pub(super) fn generated_context_parameters(
    worker: &ContinuationWorkerProvenance,
) -> Result<u32, CraneliftBackendError> {
    let captures = u32::try_from(worker.captures.len())
        .map_err(|_| planner_capacity_error("continuation worker capture count exhausted"))?;
    worker
        .declared_arity
        .checked_add(captures)
        .ok_or_else(|| planner_capacity_error("generated context parameter run exhausted"))
}

/// **`D5a` — intern the generated producer execution contexts, AFTER the fixed
/// point.**
///
/// One context per `(enclosing_specialization, worker_body_origin)` reached by a
/// call whose emission owner is a `Specialization`. ⛔ Not one per descent: a
/// worker body the fixed point walked into but that emits nothing gets no
/// context, because nothing would ever call it.
///
/// **The same raw worker reached under two continuation identities yields two
/// distinct contexts** — the key leads with the enclosing specialization, so two
/// identities cannot collapse onto one widened definition. That is the ruling's
/// requirement stated as a key, not as a check.
pub(super) fn intern_generated_contexts(
    units: &[PlannedContinuationSpecialization],
    calls: &[PlannedContinuationSpecializationCall],
) -> Result<Vec<PlannedContinuationContext>, CraneliftBackendError> {
    let mut contexts: Vec<PlannedContinuationContext> = Vec::new();
    let mut interned = BTreeMap::new();
    for call in calls {
        let ContinuationEmissionOwner::Specialization(enclosing) = call.token.emission_owner
        else {
            continue;
        };
        let worker_body_origin = call.token.producer_result_origin;
        if interned.contains_key(&(enclosing, worker_body_origin)) {
            continue;
        }
        let enclosing_unit = units.get(enclosing.0 as usize).ok_or_else(|| {
            planner_error("a causal call names an emission owner that was never interned")
        })?;
        // The context executes the enclosing specialization's OWN selected
        // worker body. A call claiming otherwise would be asking a context to
        // supply captures for a body it does not run, so this rejects rather
        // than interning a context nothing can lawfully call.
        if enclosing_unit.key.worker.body_origin != worker_body_origin {
            return Err(planner_error(
                "a causal call's producer result origin is not the enclosing specialization's \
                 selected worker body",
            ));
        }
        let id = ContinuationContextId(
            u32::try_from(contexts.len())
                .map_err(|_| planner_capacity_error("generated context identity exhausted"))?,
        );
        contexts.push(PlannedContinuationContext {
            id,
            finalized_availability: Vec::new(),
            enclosing_specialization: enclosing,
            worker_body_origin,
            raw_owner: call.token.producer_owner,
            parameters: generated_context_parameters(&enclosing_unit.key.worker)?,
            captures: enclosing_unit.key.continuation_inputs.clone(),
        });
        interned.insert((enclosing, worker_body_origin), id);
    }
    Ok(contexts)
}

pub(super) fn validate_continuation_specialization_closure(
    interned: &BTreeMap<ContinuationSpecializationKey, ContinuationSpecializationId>,
    units: &[PlannedContinuationSpecialization],
    calls: &[PlannedContinuationSpecializationCall],
) -> Result<(), CraneliftBackendError> {
    if interned.len() != units.len() {
        return Err(planner_error(
            "continuation key and unit populations are not bijective",
        ));
    }
    for (index, unit) in units.iter().enumerate() {
        if unit.id.0 as usize != index || interned.get(&unit.key) != Some(&unit.id) {
            return Err(planner_error(
                "continuation key and unit populations are not exact",
            ));
        }
    }
    let mut reached = BTreeSet::new();
    let mut identities = BTreeSet::new();
    for call in calls {
        if !identities.insert(call.token.clone()) {
            return Err(planner_error(
                "continuation planned-edge population contains a duplicate",
            ));
        }
        let target = units
            .get(call.token.target.0 as usize)
            .ok_or_else(|| planner_error("continuation edge names no target unit"))?;
        if target.id != call.token.target
            || target.key.producer_owner != call.token.producer_owner
            || target.key.producer_result_origin != call.token.producer_result_origin
            || target.key.producer_construct_origin != call.token.producer_construct_origin
            || target.key.producer_alternative != call.token.producer_alternative
            || target.key.worker != call.token.worker
        {
            return Err(planner_error(
                "continuation edge token disagrees with its exact target",
            ));
        }
        // ---- `D3` — CALL TARGET IS INJECTIVE. Ruled at `evt_7akh94dvqeqap`.
        //
        // The checks above prove key/unit bijection, unique tokens, token/target
        // agreement, and that every unit is REACHED. Reachability is surjective
        // and says nothing about the other direction: two distinct tokens may
        // name one unit and every check above still passes.
        //
        // **That gap is what pushed a liveness rule into the emitter.** Without
        // injectivity here, "may this specialization stop being declared?" is a
        // question about a call POPULATION, and the emitter grew an
        // all-incoming-calls scan to answer it -- a scan no lawful source can
        // make fail, defending an invalid planner state late. With injectivity,
        // the question is answered by one identity's own disposition and the
        // scan is unnecessary rather than merely unreachable.
        //
        // ⇒ Deliberately a SEPARATE refusal from the duplicate-token check
        // above. A repeated token is one edge planned twice; two distinct
        // tokens on one target is an ALIAS -- two edges the planner believes are
        // different, resolving to one unit. They are different defects and a
        // control for one must not pass by tripping the other.
        if !reached.insert(target.id) {
            return Err(planner_error(
                "two distinct continuation planned edges name one specialization unit, so the \
                 planner's call and unit populations are not bijective and a specialization's \
                 liveness is not decided by its own edge",
            ));
        }
    }
    if reached.len() != units.len() {
        return Err(planner_error(
            "continuation planned-edge closure does not reach every unit",
        ));
    }
    Ok(())
}

pub(super) fn validate_required_consumer_projections(
    plan: &StaticTransitionPlan<'_>,
    units: &[PlannedContinuationSpecialization],
    calls: &[PlannedContinuationSpecializationCall],
    projections: &BTreeMap<ContinuationCallIdentity, RequiredConsumerProjection>,
) -> Result<(), CraneliftBackendError> {
    let mut call_identities = BTreeSet::new();
    for call in calls {
        let target = units
            .get(call.token.target.0 as usize)
            .ok_or_else(|| planner_error("a continuation call names no target unit"))?;
        call_identities.insert(ContinuationCallIdentity {
            token: call.token.clone(),
            recursive_position: target.key.recursive_position,
        });
    }
    for (identity, projection) in projections {
        if !call_identities.contains(identity) {
            return Err(planner_error(
                "a required-consumer projection names no planned continuation call",
            ));
        }
        let target = units
            .get(identity.target().0 as usize)
            .ok_or_else(|| planner_error("a required-consumer projection names no target unit"))?;
        if identity.recursive_position != target.key.recursive_position {
            return Err(planner_error(
                "a required-consumer projection's call position disagrees with its target",
            ));
        }
        let derived = derive_required_consumer_occurrence(plan, &target.key)?;
        let source = rederive_consuming_occurrence(plan, &target.key, projection.source)?;
        if source != Some(projection.source) {
            return Err(planner_error(
                "a required-consumer projection's source occurrence does not match the exact \
                 source-level occurrence independently derived from its target",
            ));
        }
        if derived != Some(projection.required) {
            #[cfg(test)]
            {
                let reason = match derived {
                    Some(expected)
                        if expected.eliminator_origin != projection.required.eliminator_origin =>
                    {
                        "a required-consumer projection has a mismatched eliminator_origin"
                    }
                    _ => "a required-consumer projection has a mismatched body_origin",
                };
                return Err(planner_error(reason));
            }
            #[cfg(not(test))]
            return Err(planner_error(
                "a required-consumer projection is not the exact consumer-level occurrence \
                 independently derived from its target",
            ));
        }
    }
    Ok(())
}


pub(super) fn validate_continuation_specialization_plan(
    plan: &StaticTransitionPlan<'_>,
) -> Result<(), CraneliftBackendError> {
    let (
        expected_units,
        expected_calls,
        expected_required_consumers,
        expected_contexts,
        _admitted,
    ) =
        build_continuation_specialization_plan(plan)?;
    // ⛔⛔ **The comparison is against the DERIVATION, and `D3b`'s stage-2
    // finalization is not part of it.**
    //
    // This validator's whole strength is exact equality against a fresh
    // re-derivation, and that must not be weakened. But `finalized_availability`
    // is stamped *after* the derivation closes, from the interned contexts —
    // a re-derivation cannot reproduce it and is not supposed to. Comparing it
    // would make this fire on the stamping rather than on any disagreement about
    // what was derived, which is exactly what it did when finalization first
    // landed: 83 tests red, none of them about the plan being wrong.
    //
    // ⭐ Cleared on a clone rather than skipped field-by-field, so a field added
    // to either record later is compared by default. Only what is explicitly
    // named here is exempt.
    let mut landed_units = plan.continuation_specializations.clone();
    for unit in &mut landed_units {
        unit.finalized_availability.clear();
    }
    let mut landed_contexts = plan.continuation_contexts.clone();
    for context in &mut landed_contexts {
        context.finalized_availability.clear();
    }
    if landed_units != expected_units
        || plan.continuation_specialization_calls != expected_calls
        || plan.required_consumer_projections != expected_required_consumers
        || landed_contexts != expected_contexts
    {
        return Err(planner_error(
            "continuation specialization plan is not the exact closed derivation",
        ));
    }
    Ok(())
}


impl<'src> StaticTransitionPlan<'src> {
    /// **`D2f` — the closed body-emission disposition, keyed by body occurrence.**
    ///
    /// The single authority every downstream projection reads: declarations,
    /// definitions, the worker-target projection and the surviving-call-edge
    /// projection. Keyed **exactly** by `unit.body_occurrence()` and never by a
    /// call identity or a callee entry — `executable_call_edges` states at
    /// length why those are a different axis that merely coincides on most
    /// fixtures.
    ///
    /// A body claimed by both reasons refuses here rather than letting whichever
    /// insert ran second win silently.
    pub(in crate::cranelift_backend) fn body_dispositions(
        &self,
    ) -> Result<BTreeMap<StaticOriginId, BodyEmissionDisposition>, CraneliftBackendError> {
        let mut dispositions = BTreeMap::new();
        for body in self.template_only_worker_bodies()? {
            dispositions.insert(body, BodyEmissionDisposition::ContinuationTemplate);
        }
        for (body, owned) in &self.fusion_owned_bodies {
            if dispositions
                .insert(*body, BodyEmissionDisposition::FusionOwned(owned.fusion))
                .is_some()
            {
                return Err(planner_error(
                    "one body is claimed by both a continuation template and a static \
                     continuation fusion, so which definition lowers it is undetermined",
                ));
            }
        }
        Ok(dispositions)
    }

    /// **`D3` — every exact planned continuation call identity, `P`.**
    ///
    /// The one projection both halves of the partition are derived from, so
    /// `O` and `F` cannot disagree about which population they partition.
    pub(in crate::cranelift_backend) fn continuation_call_identities(
        &self,
    ) -> Result<BTreeSet<ContinuationCallIdentity>, CraneliftBackendError> {
        self.continuation_calls()?
            .iter()
            .map(|call| {
                self.continuation_call_binding_for(
                    call.producer_construct_origin(),
                    call.continuation_origin(),
                    call.producer_alternative(),
                    call.recursive_position(),
                )?
                .ok_or_else(|| {
                    planner_error(
                        "a planned continuation call has no binding under its own four-field \
                         selector, so the exact planned identity population cannot be built",
                    )
                })
            })
            .collect()
    }

    /// **`D3` — the ORDINARY residual identities `O = P \ F`.**
    ///
    /// **This is the single plan-authoritative narrowing, and it exists so
    /// that no consumer filters for itself.** Ruled `evt_48rwarx25pj2p` §3:
    /// the candidate ledger, the claim ledger, declaration, definition,
    /// resolution, direct-call verification and the `D8` composed-discharge
    /// machinery all read *this*, so every landed ordinary law stays literally
    /// true over its own complete domain rather than being weakened to tolerate
    /// an absence.
    ///
    /// Repeated filtering at each consumer would be a second authority over
    /// which edges are ordinary, and the two would drift silently.
    pub(in crate::cranelift_backend) fn ordinary_continuation_call_identities(
        &self,
    ) -> Result<BTreeSet<ContinuationCallIdentity>, CraneliftBackendError> {
        Ok(Self::ordinary_identities_of(
            &self.continuation_call_identities()?,
            &self.fusion_composed_calls,
            &self.fusion_outer_realizations,
        ))
    }

    /// **`D3` — the ONE derivation of `O`, over populations supplied by the
    /// caller.**
    ///
    /// ⚠ **This exists so that "validated" and "consumed" cannot be two
    /// derivations.** Preflight calls it on the CANDIDATE `I` and `R` before
    /// either is installed, and the accessor above calls it on the installed
    /// ones. The partition law therefore ranges over exactly the function every
    /// consumer reads, rather than over a residual the validator computed for
    /// itself and nothing afterwards used -- which is what made the binary form
    /// vacuous.
    pub(super) fn ordinary_identities_of(
        planned: &BTreeSet<ContinuationCallIdentity>,
        inner: &BTreeMap<ContinuationCallIdentity, FusionComposedEdge>,
        outer: &BTreeMap<ContinuationCallIdentity, FusionOwnedOuterRealization>,
    ) -> BTreeSet<ContinuationCallIdentity> {
        planned
            .iter()
            .filter(|identity| {
                !inner.contains_key(*identity) && !outer.contains_key(*identity)
            })
            .cloned()
            .collect()
    }

    /// The target-side twin of [`Self::ordinary_identities_of`], and it exists
    /// for the same reason.
    pub(super) fn ordinary_targets_of(
        planned: &BTreeSet<ContinuationSpecializationId>,
        inner: &BTreeMap<ContinuationCallIdentity, FusionComposedEdge>,
        outer: &BTreeMap<ContinuationCallIdentity, FusionOwnedOuterRealization>,
    ) -> BTreeSet<ContinuationSpecializationId> {
        let fused = inner
            .values()
            .map(|edge| edge.target)
            .chain(outer.values().map(|realization| realization.target))
            .collect::<BTreeSet<_>>();
        planned.difference(&fused).copied().collect()
    }

    /// **`D3` — the ORDINARY residual targets `O_t = T \ F_t`.**
    ///
    /// The target-side twin of [`Self::ordinary_continuation_call_identities`].
    /// A fusion-local target omits its declaration, definition and resolution,
    /// so this is the population those three passes range over.
    pub(in crate::cranelift_backend) fn ordinary_continuation_targets(
        &self,
    ) -> Result<BTreeSet<ContinuationSpecializationId>, CraneliftBackendError> {
        let planned = self
            .continuation_units()?
            .iter()
            .map(|unit| unit.id())
            .collect::<BTreeSet<_>>();
        Ok(Self::ordinary_targets_of(
            &planned,
            &self.fusion_composed_calls,
            &self.fusion_outer_realizations,
        ))
    }

    /// Test-side independent read of the consuming occurrence recorded on one
    /// unit. Production validation uses the same derivation before returning a
    /// plan; exposing it here lets the row controls print both authorities.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn rederive_continuation_consuming_occurrence(
        &self,
        unit: &ContinuationUnitView<'_>,
    ) -> Result<Option<ContinuationConsumingOccurrence>, CraneliftBackendError> {
        let Some(claimed) = unit.key.consuming_occurrence else {
            return Ok(None);
        };
        rederive_consuming_occurrence(self, unit.key, claimed)
    }

    /// Test-side check of the unchanged `consumer_owner` validator relation.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn continuation_consumer_owner_is_exact(
        &self,
        unit: &ContinuationUnitView<'_>,
    ) -> Result<bool, CraneliftBackendError> {
        Ok(occurrence_authority(self, unit.continuation_origin())?.owner
            == unit.consumer_owner())
    }

    /// Every already-validated continuation specialization, with its exact
    /// identity, its immutable planner key facts, and its validated ABI
    /// descriptor, slots and input authority.
    ///
    /// Revalidates plan/ABI agreement: the two populations must be the same
    /// size, each descriptor must name a `ContinuationSpecialization` whose id
    /// is the planner's id at that position, and every dense range must lie
    /// inside its plane.
    pub(in crate::cranelift_backend) fn continuation_units(
        &self,
    ) -> Result<Vec<ContinuationUnitView<'_>>, CraneliftBackendError> {
        if self.abi.continuation_descriptors.len() != self.continuation_specializations.len() {
            return Err(planner_error(
                "continuation ABI descriptor count disagrees with the planned specialization \
                 population",
            ));
        }
        // The join is BY IDENTITY, not by position.
        //
        // An earlier form zipped the two populations and then checked that the
        // descriptor's id equalled the planner's id at that index. That catches
        // a reordering of one side, but agrees with an *identical* reordering
        // of both, so it was not an independent check. Indexing the descriptors
        // by the id they declare, and then resolving each planned
        // specialization through that index, removes position from the join
        // entirely: a descriptor is found by the identity it names or not at
        // all.
        let mut by_id: BTreeMap<ContinuationSpecializationId, &abi::AbiContinuationDescriptor> =
            BTreeMap::new();
        for descriptor in &self.abi.continuation_descriptors {
            let AbiUnitDefinition::ContinuationSpecialization { specialization } =
                descriptor.definition
            else {
                return Err(planner_error(
                    "a continuation ABI descriptor does not define a continuation specialization",
                ));
            };
            if by_id.insert(specialization, descriptor).is_some() {
                return Err(planner_error(
                    "two continuation ABI descriptors declare the same specialization identity",
                ));
            }
        }
        self.continuation_specializations
            .iter()
            .map(|planned| {
                let descriptor = *by_id.get(&planned.id).ok_or_else(|| {
                    planner_error(
                        "a planned continuation specialization has no ABI descriptor declaring \
                         its identity",
                    )
                })?;
                let slots = dense_slice(&self.abi.continuation_slots, descriptor.slots)
                    .ok_or_else(|| {
                        planner_error("continuation slot range is outside the plane")
                    })?;
                let inputs = dense_slice(&self.abi.continuation_inputs, descriptor.inputs)
                    .ok_or_else(|| {
                        planner_error("continuation input range is outside the plane")
                    })?;
                if inputs.len() != planned.key.continuation_inputs.len() {
                    return Err(planner_error(
                        "continuation input authority count disagrees with the planner key's \
                         ordered input projection",
                    ));
                }
                Ok(ContinuationUnitView {
                    id: planned.id,
                    key: &planned.key,
                    finalized: &planned.finalized_availability,
                    header: descriptor.header,
                    slots,
                    inputs,
                })
            })
            .collect()
    }

    /// **`D5a` — every planner-interned generated producer execution context.**
    ///
    /// Revalidates plan/ABI agreement the same way [`Self::continuation_units`]
    /// does, and by the same argument: the join is **by identity**, indexing the
    /// descriptors by the id they declare, so an identical reordering of both
    /// sides does not pass.
    pub(in crate::cranelift_backend) fn continuation_contexts(
        &self,
    ) -> Result<Vec<ContinuationContextView<'_>>, CraneliftBackendError> {
        if self.abi.context_descriptors.len() != self.continuation_contexts.len() {
            return Err(planner_error(
                "generated context ABI descriptor count disagrees with the planned context \
                 population",
            ));
        }
        let mut by_id: BTreeMap<
            ContinuationContextId,
            &abi::AbiContinuationContextDescriptor,
        > = BTreeMap::new();
        for descriptor in &self.abi.context_descriptors {
            if by_id.insert(descriptor.context, descriptor).is_some() {
                return Err(planner_error(
                    "two generated context ABI descriptors declare the same context identity",
                ));
            }
        }
        self.continuation_contexts
            .iter()
            .map(|planned| {
                let descriptor = *by_id.get(&planned.id).ok_or_else(|| {
                    planner_error(
                        "a planned generated context has no ABI descriptor declaring its identity",
                    )
                })?;
                let slots = dense_slice(&self.abi.context_slots, descriptor.slots)
                    .ok_or_else(|| {
                        planner_error("generated context slot range is outside the plane")
                    })?;
                let inputs = dense_slice(&self.abi.context_inputs, descriptor.inputs)
                    .ok_or_else(|| {
                        planner_error("generated context input range is outside the plane")
                    })?;
                if inputs.len() != planned.captures.len() {
                    return Err(planner_error(
                        "generated context input authority count disagrees with the planner's \
                         ordered capture projection",
                    ));
                }
                Ok(ContinuationContextView {
                    planned,
                    finalized: &planned.finalized_availability,
                    header: descriptor.header,
                    slots,
                    inputs,
                })
            })
            .collect()
    }

    /// The generated context that emits on behalf of one specialization, if that
    /// specialization's worker body has one.
    ///
    /// ⛔ Selected by the **enclosing specialization and the exact body origin**,
    /// both supplied by the caller. Nothing here searches for a plausible
    /// context, and `None` means the ordinary raw-worker target is correct — it
    /// is not a licence to pick one.
    pub(in crate::cranelift_backend) fn continuation_context_for(
        &self,
        enclosing: ContinuationSpecializationId,
        worker_body_origin: StaticOriginId,
    ) -> Result<Option<ContinuationContextView<'_>>, CraneliftBackendError> {
        let contexts = self.continuation_contexts()?;
        // `D5a` checkpoint 4 step 3 -- the duplicate reaching mutation, applied
        // to the POPULATION this loop walks and not to the stop below.
        //
        // ⚠ It has to be applied here. The planner interns contexts on this
        // very key, so a duplicate is unreachable through any plan it will
        // build; the only way to ask whether the collision stop works is to
        // present the population it is written for. ⛔ Re-emitting the stop's
        // own error under the mutation would have been a control that proves a
        // hardcoded string propagates.
        let mut order = (0..contexts.len()).collect::<Vec<_>>();
        #[cfg(test)]
        if crate::cranelift_backend::lowering::d5a_route_mutation()
            == crate::cranelift_backend::lowering::D5aRouteMutation::DuplicateContextBinding
        {
            crate::cranelift_backend::lowering::record_d5a_route_application();
            order.extend(0..contexts.len());
        }
        let mut found = None;
        for index in order {
            let context = &contexts[index];
            if context.enclosing_specialization() == enclosing
                && context.worker_body_origin() == worker_body_origin
            {
                if found.is_some() {
                    return Err(planner_error(
                        "two generated contexts claim one specialization and worker body",
                    ));
                }
                found = Some(index);
            }
        }
        Ok(found.map(|index| {
            contexts
                .into_iter()
                .nth(index)
                .expect("an index taken from this same population")
        }))
    }

    /// Every already-validated continuation call token, with the full producer
    /// tuple and the exact target it names.
    ///
    /// The join to a specialization is **by `token.target` only**. Continuation
    /// origin and recursive position are read from the resolved target's key;
    /// the call-site sequence is read only from the token.
    pub(in crate::cranelift_backend) fn continuation_calls(
        &self,
    ) -> Result<Vec<ContinuationCallView<'_>>, CraneliftBackendError> {
        let units = self.continuation_units()?;
        self.continuation_specialization_calls
            .iter()
            .map(|planned| {
                let token = &planned.token;
                let unit = units
                    .iter()
                    .find(|unit| unit.id == token.target)
                    .ok_or_else(|| {
                        planner_error(
                            "a continuation call token names a target with no planned \
                             specialization",
                        )
                    })?;
                if unit.key.producer_construct_origin != token.producer_construct_origin
                    || unit.key.producer_alternative != token.producer_alternative
                    || unit.key.producer_owner != token.producer_owner
                    || unit.key.emission_owner != token.emission_owner
                    || unit.key.producer_result_origin != token.producer_result_origin
                {
                    return Err(planner_error(
                        "a continuation call token's producer tuple disagrees with its resolved \
                         target's planner key",
                    ));
                }
                Ok(ContinuationCallView {
                    token,
                    // From the RESOLVED TARGET KEY, never from the token.
                    continuation_origin: unit.key.continuation_origin,
                    recursive_position: unit.key.recursive_position,
                    target: token.target,
                })
            })
            .collect()
    }

    /// **The only lowering lookup selector**, keyed by the four fields the
    /// later producer path can actually supply: the actual `Construct` origin,
    /// the active computational-frame origin, the selected alternative, and one
    /// member of that case's ruled recursive positions.
    ///
    /// Zero matches is `None` — the caller takes its existing nonspecialized
    /// path. More than one match is a **planner invariant failure**, never a
    /// choice: selecting the first, lowest or any other sequence here would
    /// make lowering the authority for a fact the planner owns.
    pub(in crate::cranelift_backend) fn continuation_call_binding_for(
        &self,
        producer_construct_origin: StaticOriginId,
        continuation_origin: StaticOriginId,
        producer_alternative: u32,
        recursive_position: u32,
    ) -> Result<Option<ContinuationCallIdentity>, CraneliftBackendError> {
        let mut found: Option<ContinuationCallIdentity> = None;
        for call in self.continuation_calls()? {
            if call.token.producer_construct_origin == producer_construct_origin
                && call.continuation_origin == continuation_origin
                && call.token.producer_alternative == producer_alternative
                && call.recursive_position == recursive_position
            {
                if found.is_some() {
                    return Err(planner_error(
                        "more than one continuation call binding matches one exact selector; the \
                         planner mints one call token per ruled recursive position, so this is a \
                         planner invariant failure and never a choice between sequences",
                    ));
                }
                found = Some(ContinuationCallIdentity {
                    token: call.token.clone(),
                    recursive_position: call.recursive_position,
                });
            }
        }
        Ok(found)
    }

    /// The independently validated consumer-level occurrence for one exact
    /// continuation call.
    ///
    /// The map is built and checked in planning. Lowering receives only this
    /// opaque value; it neither sees the derivation nor manufactures a fallback
    /// from the source-level specialization key.
    pub(in crate::cranelift_backend) fn required_consumer_projection_for(
        &self,
        identity: &ContinuationCallIdentity,
    ) -> Option<RequiredConsumerProjection> {
        self.required_consumer_projections.get(identity).copied()
    }

    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D7a` — the planner-issued composed worker
    /// view.**
    ///
    /// Keyed by the four facts a composed eliminator frame actually holds: the
    /// **producer `Construct` occurrence** it is building, the
    /// **computational-frame origin** it is lowering, the **selected
    /// alternative**, and one member of that case's **ruled recursive
    /// positions**. It answers with the full worker provenance the continuation
    /// unit uses — closure occurrence, raw body, declared arity, ordered capture
    /// provenance, and route eligibility.
    ///
    /// ⭐ **This is the same causal coordinate tuple
    /// [`Self::continuation_call_binding_for`] already selects on, and that is
    /// the point.** The producer `Construct` origin is not a tag, a sequence, an
    /// owner heuristic, a specialization id, or a second identity bolted on to
    /// break a tie; it is the field that says *which occurrence is being
    /// built*, and the composed path supplies it directly as
    /// `deferred.construct_origin`.
    ///
    /// ⛔ **It is load-bearing, not belt-and-braces.** The other three fields are
    /// all properties of the **source text**: one source computational match
    /// specialized at two recursion layers shares its origin, its selected
    /// alternative and its ruled recursive position across both layers, and the
    /// producer `Construct` occurrence is the only thing that separates them.
    /// Without this field the selector collides on every plan in this crate that
    /// interns continuation specializations, which
    /// [`d7a_the_three_field_selector_collides_where_the_four_field_selector_resolves`]
    /// measures directly.
    ///
    /// ⇒ **Different workers under distinct construct origins are distinct
    /// questions, not a conflict.** Two layers naming two workers is the
    /// ordinary, lawful shape of nested specialization. Conflict is reserved for
    /// two specializations answering *one* four-field selector with different
    /// workers.
    ///
    /// [`d7a_the_three_field_selector_collides_where_the_four_field_selector_resolves`]:
    ///     crate::cranelift_backend::lowering::core::tests::control
    ///
    /// ⛔ **This method is the whole point of `D7a`, so read what it forbids.**
    /// A consumer that has this cannot need to walk the closure occurrence, read
    /// a body's parameter count, measure an environment's length, or decide from
    /// *whichever of the two targets happens to exist*. Every one of those is a
    /// second authority for a fact the planner already interned, and each was
    /// available before this method was and is what it retires.
    ///
    /// ## The four refusals, and why none of them is an `Option`
    ///
    /// Unlike [`Self::continuation_call_binding_for`], **zero answers is a
    /// refusal, not a `None`.** The difference is what the two selectors mean.
    /// That one asks *is this producer occurrence specialized at all*, and "no"
    /// is the ordinary pre-specialization path taken by every program in the
    /// pre-`D5a` population. This one is asked only about a position the planner
    /// **already ruled recursive on a frame it already specialized**, so zero is
    /// a contradiction between two planner facts and there is no lawful
    /// alternative route for a caller to fall back to.
    ///
    /// | refusal | what it means |
    /// |---|---|
    /// | zero answers | no specialization claims this construct occurrence, frame, alternative and position |
    /// | conflicting full identities | two do, and they disagree about the worker |
    /// | wrong position / body / capture provenance | a key's own worker record fails its independent re-check |
    /// | unexecutable raw target | the raw body is superseded, so a raw-route call has no `Function` to reach |
    ///
    /// ⚠ **The fourth is not defensive padding.** The selected recursive
    /// argument's route is `RawWorker` unconditionally, so a superseded raw body
    /// means this view would hand a consumer a target that has a descriptor and
    /// no emitted `Function`. Refusing here is the same fail-closed direction
    /// [`Self::executable_units`] takes, applied at the projection rather than
    /// at the emission. ⛔ That refusal and the executable-unit population it
    /// reads are `D7a2`'s subject and are deliberately untouched here.
    ///
    /// ⛔ **Agreement between several answers is lawful; equality is by full
    /// identity.** Two specializations may still share all four fields — they
    /// would differ only in emission owner, which this selector deliberately
    /// does not carry — and if they name the *same* worker, the same eligibility
    /// included, there is one answer and it is theirs. Only disagreement
    /// refuses. Picking the first, the lowest, or any other sequence would make
    /// the caller the authority for a fact the planner owns.
    // `D7b` (one environment authority) and `D7c` (the callee consumer) are the
    // production consumers, and both are held. Until one lands the only callers
    // are this node's own tests, so the attribute below is load-bearing rather
    // than habitual — and `D7b` is what retires it.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(in crate::cranelift_backend) fn composed_worker_view(
        &self,
        emission_owner: ContinuationEmissionOwner,
        producer_construct_origin: StaticOriginId,
        continuation_origin: StaticOriginId,
        producer_alternative: u32,
        recursive_position: u32,
    ) -> Result<ComposedWorkerView, CraneliftBackendError> {
        let answer = self.composed_worker_view_unreconciled(
            emission_owner,
            producer_construct_origin,
            continuation_origin,
            producer_alternative,
            recursive_position,
        )?;

        // ⭐ Executability is checked on the RESOLVED answer, not per candidate,
        // and the order is the design rather than a convenience. A group whose
        // members disagree has no worker to ask this question about, so
        // "ambiguous" is the primary defect and must be what a reader is told;
        // asking each candidate first would report whichever member happened to
        // be superseded and bury the ambiguity. Nothing is lost: agreement fixes
        // the body origin, and executability is a function of the body alone.
        //
        // ⭐ `D7a2`: the population this reads is the **reconciled** one. The
        // refusal itself is unchanged and still truthful — it fires exactly when
        // the raw route has no `Function` to reach — but a body that a minted
        // raw-target requirement retains is no longer superseded, so the same
        // question now gets a different, and correct, answer.
        // `D2f`: the disposition map, not the `D5a` set alone. A fusion-owned
        // body leaves the executable population for a different reason and the
        // raw route it projects is equally unreachable, so the refusal names
        // whichever disposition actually applies rather than reporting the one
        // reason it happens to know about.
        match self.body_dispositions()?.get(&answer.body_origin) {
            None => {}
            Some(BodyEmissionDisposition::ContinuationTemplate) => {
                return Err(planner_error(
                    "the composed selector's raw worker body is template-only: every \
                     specialization selecting it retargeted, so it keeps its descriptor and \
                     leaves the executable population, and the raw route this view projects has \
                     no Function to reach",
                ));
            }
            Some(BodyEmissionDisposition::FusionOwned(_)) => {
                return Err(planner_error(
                    "the composed selector's raw worker body is owned by a static continuation \
                     fusion: it is lowered inside that fused definition, so it leaves the \
                     executable population and the raw route this view projects has no Function \
                     to reach",
                ));
            }
        }
        Ok(answer)
    }

    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D8b` — every planner-issued composed-call
    /// target, one per `D8a` selector the planner interned.**
    ///
    /// Demand is derived from the **existence of a specialization at a
    /// selector**, an already-interned planner fact. Nothing here reads a
    /// reached lowerer, walks source, or consults an emitted shape.
    ///
    /// ⛔ **Unreconciled by construction, and that is the no-circularity rule.**
    /// The view is taken from
    /// [`Self::composed_worker_view_unreconciled`], so no target depends on an
    /// executability answer. `D8c` owns that question, and it must be free to
    /// answer it without the answer having already been assumed here — which is
    /// exactly the loop `D7a2` closed and was withdrawn for.
    ///
    /// ⛔ A selector whose group does not resolve — conflicting workers, two
    /// emission owners, failed provenance — mints **no** target and propagates
    /// its refusal. A target names one exact callee; there is none to name when
    /// the planner cannot say which.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(in crate::cranelift_backend) fn composed_call_targets(
        &self,
    ) -> Result<Vec<ComposedCallTarget>, CraneliftBackendError> {
        let units = self.continuation_units()?;
        let mut selectors = units
            .iter()
            .map(|unit| {
                (
                    unit.emission_owner(),
                    unit.producer_construct_origin(),
                    unit.continuation_origin(),
                    unit.producer_alternative(),
                    unit.recursive_position(),
                )
            })
            .collect::<Vec<_>>();
        selectors.sort_unstable();
        selectors.dedup();

        let mut targets = Vec::with_capacity(selectors.len());
        for (owner, construct, frame, alternative, position) in selectors {
            let worker = self.composed_worker_view_unreconciled(
                owner,
                construct,
                frame,
                alternative,
                position,
            )?;
            // `D8h` — THE PAIRING, through the planner's own lookup.
            //
            // ⛔ The four arguments are this target's own causal coordinate and
            // nothing else. No body, symbol, arity, source position or
            // same-shaped constructor participates, and the sequence inside the
            // identity is never seen here: `continuation_call_binding_for`
            // returns the whole opaque value or refuses.
            let call_identity = self
                .continuation_call_binding_for(construct, frame, alternative, position)?
                .ok_or_else(|| {
                    planner_error(
                        "a composed-call target's own causal coordinate selects no \
                         continuation call binding, so there is no planner-issued identity to \
                         pair it with. This fails closed rather than minting an unpaired \
                         target: the two populations are interned together -- one call token \
                         per interned specialization, at the same coordinate -- so a target \
                         without a binding means those two have drifted, and a consumer handed \
                         an unpaired target would have to invent the identity that is missing",
                    )
                })?;
            // The FIFTH field, held rather than looked up. The lookup above is
            // keyed on the four causal fields; the emission owner is the
            // coordinate's remaining component, and the identity carries its
            // own. ⭐ Comparing them is what makes the pairing five-field: two
            // independently derived answers to "who emits this", from the
            // interned unit and from the call token, must agree.
            if call_identity.emission_owner() != owner {
                return Err(planner_error(
                    "a composed-call target's paired causal identity names a different \
                     emission owner than the selector it was minted under, so the call token \
                     and the interned unit disagree about who emits this call",
                ));
            }
            targets.push(ComposedCallTarget {
                emission_owner: owner,
                producer_construct_origin: construct,
                continuation_origin: frame,
                producer_alternative: alternative,
                recursive_position: position,
                worker,
                call_identity,
            });
        }

        #[cfg(test)]
        {
            match COMPOSED_CALL_TARGET_DEFECT.with(Cell::get) {
                ComposedCallTargetDefect::Exact => {}
                ComposedCallTargetDefect::WrongBody => {
                    // Point the first target at another target's body. ⛔ A
                    // fabricated origin would be caught by the plane bounds
                    // rather than by the law below, which would prove the wrong
                    // guard.
                    let other = targets
                        .iter()
                        .map(|target| target.worker.body_origin)
                        .find(|body| *body != targets[0].worker.body_origin);
                    if let Some(other) = other {
                        targets[0].worker.body_origin = other;
                    }
                }
                ComposedCallTargetDefect::TransplantConstruct => {
                    let other = targets
                        .iter()
                        .map(|target| target.producer_construct_origin)
                        .find(|construct| *construct != targets[0].producer_construct_origin);
                    if let Some(other) = other {
                        targets[0].producer_construct_origin = other;
                    }
                }
                ComposedCallTargetDefect::SameSymbolIdentity => {
                    // ⛔ SEARCHED, not hand-picked: the sibling is selected by
                    // the very equality the forbidden rule would key on, so the
                    // switch installs that rule's own answer.
                    let symbol =
                        self.constructor_symbol_identity(targets[0].producer_construct_origin)?;
                    let mut sibling = None;
                    for target in targets.iter().skip(1) {
                        if self.constructor_symbol_identity(target.producer_construct_origin)?
                            == symbol
                        {
                            sibling = Some(target.call_identity.clone());
                            break;
                        }
                    }
                    if let Some(sibling) = sibling {
                        targets[0].call_identity = sibling;
                    }
                }
            }
        }
        Ok(targets)
    }

    /// **`D8b` — the composed-call target law: selector agreement.**
    ///
    /// Returns the number of targets checked, so a caller can tell a real
    /// population from an empty one.
    ///
    /// **One law, and it is on the unreconciled side.** Re-resolving a target's
    /// own five-field selector must return the worker it carries. That catches
    /// both minting defects, because both are one defect seen from two sides: a
    /// callee attributed to a selector that does not resolve to it. Each names
    /// real origins, so neither is visible to a check that only asks whether the
    /// body exists.
    ///
    /// ⛔ **Three checks that were here are gone, and none of them should come
    /// back in this form:**
    ///
    /// - *the carried body must be its worker's body* — read the field back
    ///   through an accessor defined as that field, so it compared a value with
    ///   itself. Killed by its own control in `D7a2`; the accessor is deleted
    ///   too, so it cannot be written again by accident;
    /// - *exact-set equality against the executable population* — an
    ///   executability question, which is `D8c`'s and whose presence here was
    ///   the circularity;
    /// - *declaration and definition agree* — unexercised twice, measured both
    ///   times, and deleted rather than carried a third time. Every body on
    ///   these plans has an emittable descriptor by construction, and the only
    ///   perturbation that would reach it is a fabricated origin, which the
    ///   plane bounds catch instead.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(in crate::cranelift_backend) fn verify_composed_call_targets(
        &self,
    ) -> Result<usize, CraneliftBackendError> {
        let targets = self.composed_call_targets()?;
        for target in &targets {
            let (owner, construct, frame, alternative, position) = target.selector();
            let resolved = self.composed_worker_view_unreconciled(
                owner,
                construct,
                frame,
                alternative,
                position,
            )?;
            if resolved != target.worker {
                return Err(planner_error(
                    "a composed-call target's own selector resolves to a different worker than \
                     the target carries, so the callee was minted for one layer and attributed \
                     to another",
                ));
            }
            // `D8h` — the pairing law, the second half of the same claim.
            //
            // ⭐ Re-resolved from the target's OWN coordinate through the same
            // planner lookup that minted it, and compared whole. ⛔ The
            // comparison is on the opaque identity, so it holds the call-site
            // sequence too without this code ever seeing it -- which is the
            // property a sequence accessor would have destroyed.
            let paired = self
                .continuation_call_binding_for(construct, frame, alternative, position)?
                .ok_or_else(|| {
                    planner_error(
                        "a composed-call target's own causal coordinate selects no continuation \
                         call binding at verification, so the identity it carries cannot be the \
                         one that coordinate names",
                    )
                })?;
            if paired != target.call_identity {
                return Err(planner_error(
                    "a composed-call target carries a causal identity its own five-field \
                     coordinate does not select, so the identity was attributed by something \
                     other than the coordinate -- a constructor symbol, a worker body, an arity \
                     or a source position would each produce exactly this",
                ));
            }
            if target.call_identity.emission_owner() != owner {
                return Err(planner_error(
                    "a composed-call target's paired causal identity names a different emission \
                     owner than its own selector",
                ));
            }
        }
        Ok(targets.len())
    }

    /// [`Self::composed_worker_view`] up to but **not including** the
    /// executability check.
    ///
    /// ⛔ The split exists because `D7a2`'s raw-target requirements are derived
    /// from these views, and the retained population is derived from those
    /// requirements. Asking the reconciled question here would be a cycle:
    /// *is this body executable* would depend on a requirement that depends on
    /// this answer. Everything that makes the view **correct** — the selector,
    /// the conflict rule, all three provenance re-checks — is on this side of
    /// the split; only the population question is deferred.
    pub(super) fn composed_worker_view_unreconciled(
        &self,
        emission_owner: ContinuationEmissionOwner,
        producer_construct_origin: StaticOriginId,
        continuation_origin: StaticOriginId,
        producer_alternative: u32,
        recursive_position: u32,
    ) -> Result<ComposedWorkerView, CraneliftBackendError> {
        let units = self.continuation_units()?;

        // ⛔ `D8b` amendment — an owner-collision refusal stood here and is
        // deleted. `D8a` measured its population **impossible**, twice over:
        // `continuation_result_origins` does not descend into closures and every
        // descent root is a closure's body child, so the seed and descent walks
        // cover disjoint subtrees; and removing that disjointness does not
        // produce two owners, it produces the availability law's refusal at
        // planning. A guard whose population the planner proves cannot exist is
        // not a residual to carry -- it is a check that can never fail, and one
        // more of those was already deleted from this node for the same reason.
        //
        // ⛔ The `emission_owner` field stays and its selector role is live:
        // supplying an owner no unit carries reaches the zero-answer refusal
        // below, and dropping it from the filter reds two rows.
        let mut answer: Option<ComposedWorkerView> = None;
        for unit in &units {
            if unit.emission_owner() != emission_owner
                || unit.producer_construct_origin() != producer_construct_origin
                || unit.continuation_origin() != continuation_origin
                || unit.producer_alternative() != producer_alternative
                || unit.recursive_position() != recursive_position
            {
                continue;
            }
            let candidate = self.composed_worker_view_of(unit, recursive_position)?;
            match &answer {
                None => answer = Some(candidate),
                Some(established) if *established == candidate => {}
                Some(_) => {
                    return Err(planner_error(
                        "two continuation specializations answer one composed worker selector \
                         with different full worker identities; one producer Construct \
                         occurrence at one frame position has one static worker, and choosing \
                         between them would make the caller the authority for a fact planning \
                         owns",
                    ));
                }
            }
        }
        let answer = answer.ok_or_else(|| {
            planner_error(
                "no continuation specialization claims this emission owner, producer Construct \
                 occurrence, computational frame, selected alternative and ruled recursive \
                 position, so there is no planner-issued worker provenance to project and \
                 nothing may be reconstructed from the closure occurrence's shape instead",
            )
        })?;
        Ok(answer)
    }

    /// One unit's contribution to [`Self::composed_worker_view`], with the three
    /// provenance re-checks that method documents. ⛔ The executability check is
    /// deliberately **not** here — it belongs to the resolved answer, for the
    /// reason recorded at its site.
    ///
    /// ⭐ Each check is against a fact derived **independently** of the one it
    /// validates, which is what makes it a check rather than a restatement:
    /// the position against the selector the caller supplied, the body through
    /// the sole child-origin production point rather than the children list that
    /// recorded it, and the captures against the ABI-validated ordinary
    /// envelope rather than against themselves.
    pub(super) fn composed_worker_view_of(
        &self,
        unit: &ContinuationUnitView<'_>,
        recursive_position: u32,
    ) -> Result<ComposedWorkerView, CraneliftBackendError> {
        let worker = &unit.key.worker;

        // Position provenance. The intern path already required this against
        // the producer's argument list; here it is re-checked against the
        // selector the caller supplied, which is a different source for the
        // same number.
        if worker.sibling_position != recursive_position {
            return Err(planner_error(
                "a continuation specialization's static worker sits at a different constructor \
                 sibling position than the composed selector names",
            ));
        }

        // Body provenance. `child_static_origin` is the sole production point
        // for a child's static name; the key's `body_origin` was taken from the
        // closure's recorded child authority. Comparing them is a real join, not
        // a value compared with itself.
        if self.child_static_origin(worker.closure_origin, 0)? != worker.body_origin {
            return Err(planner_error(
                "a continuation specialization's static worker body is not its own closure \
                 occurrence's body child, so the raw target this view would project is not the \
                 body that closure denotes",
            ));
        }

        // Capture provenance, against the ruled ordinary envelope — which is
        // itself recompared against the validated `Parameter` slot run before it
        // is returned, so this reaches the ABI rather than stopping at the key.
        let envelope = unit.ordinary_envelope()?;
        let carried = envelope
            .iter()
            .filter_map(|role| match role {
                ContinuationOrdinaryEnvelopeRole::WorkerCapture {
                    ordinal,
                    owner,
                    closure_origin,
                    source,
                    lifetime,
                } => Some((*ordinal, *owner, *closure_origin, *source, *lifetime)),
                ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField { .. } => None,
            })
            .collect::<Vec<_>>();
        if carried.len() != worker.captures.len() {
            return Err(planner_error(
                "a continuation specialization's worker captures and its ruled ordinary \
                 envelope's capture run are different lengths",
            ));
        }
        for (position, (capture, carried)) in worker.captures.iter().zip(&carried).enumerate() {
            if u32::try_from(position).ok() != Some(capture.ordinal)
                || capture.closure_origin != worker.closure_origin
                || (
                    capture.ordinal,
                    capture.owner,
                    capture.closure_origin,
                    capture.source,
                    capture.lifetime,
                ) != *carried
            {
                return Err(planner_error(
                    "a continuation specialization's worker captures are not one exact ordered \
                     envelope of its own closure occurrence",
                ));
            }
        }

        let route_eligibility = match self.continuation_context_for(unit.id(), worker.body_origin)? {
            Some(context) => ComposedWorkerRouteEligibility::GeneratedContextIssued(context.id()),
            None => ComposedWorkerRouteEligibility::RawOnly,
        };

        Ok(ComposedWorkerView {
            closure_origin: worker.closure_origin,
            body_origin: worker.body_origin,
            declared_arity: worker.declared_arity,
            captures: worker.captures.clone(),
            route_eligibility,
            recursive_position,
        })
    }

    /// **`RT-DECL-CLOSURE-PORT` `D5a` — project already-issued causal authority
    /// onto this owner's exact result edges.**
    ///
    /// This is **exposure, not discovery**. Every edge returned is a call the
    /// planner already minted; the identity on it is resolved through the
    /// existing four-field selector, so nothing here can create a binding that
    /// [`Self::continuation_call_binding_for`] would not also return.
    ///
    /// ⛔ The owner is supplied by the caller and is the unit it is about to
    /// define — it is never read back off a lowered value, a reached
    /// occurrence, or an emitted shape. ⛔ There is no ordering preference and
    /// no "first match": the population is returned whole, and a consumer that
    /// cannot resolve it to one member must fail rather than choose (`D5a`
    /// contract 3).
    pub(in crate::cranelift_backend) fn continuation_result_edges_owned_by(
        &self,
        emission_owner: ContinuationEmissionOwner,
    ) -> Result<Vec<ContinuationResultEdge>, CraneliftBackendError> {
        let mut edges = Vec::new();
        for call in self.continuation_calls()? {
            // `D5a`: keyed on the EMISSION owner, never on provenance. A unit
            // that merely contains the producer text is not thereby able to
            // emit the call.
            if call.emission_owner() != emission_owner {
                continue;
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
                        "a projected causal call has no binding under its own four-field \
                         selector, so its result edge cannot be projected",
                    )
                })?;
            // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — `O` ONLY.
            //
            // `evt_6bm54j10w1n88`: an `R` identity is realized once by the
            // fusion-owned body and must never reach a call seat at all, and an
            // `I` identity is realized at its own call edge by
            // `compose_continuation_locally`. Projecting either as a residual
            // result edge hands the detached-result seat a causal call for
            // something that will never be a call -- which is exactly the
            // refusal the armed compile stopped at.
            //
            // ⛔ Narrowed at the projection, not filtered at the seat. The seat
            // rejects a multi-member projection outright, so a class it must
            // ignore has to be absent rather than skipped, and one authority
            // over which edges are ordinary is the whole point of `O`.
            if self.fusion_composed_calls.contains_key(&identity)
                || self.fusion_outer_realizations.contains_key(&identity)
            {
                continue;
            }
            edges.push(ContinuationResultEdge {
                producer_result_origin: call.producer_result_origin(),
                producer_construct_origin: call.producer_construct_origin(),
                recursive_position: call.recursive_position(),
                identity,
            });
        }
        Ok(edges)
    }

}

#[cfg(test)]
pub(in crate::cranelift_backend) mod tests {
    #[allow(unused_imports)]
    use super::super::tests::*;
    use super::super::*;
    use super::*;
    #[allow(unused_imports)]
    use crate::{RuntimeComputationalMatchCase, RuntimeMatchCase, RuntimeTrap, RuntimeTrapCode, RuntimeValue};

    /// Supplementary local resolution law. The governing discriminator is the
    /// population-side `Let` insertion exercised through the full planner.
    #[test]
    fn checked_binder_resolution_rederives_an_intervening_binder_shift() {
        let binding = CheckedIhBinding {
            frame_origin: StaticOriginId(41),
            recursive_position: 3,
        };
        let unshifted_environment = [CheckedBinderProvenance::InductionHypothesis(binding)];
        let shifted_environment = [
            CheckedBinderProvenance::Ordinary,
            CheckedBinderProvenance::InductionHypothesis(binding),
        ];

        let unshifted = resolve_checked_binder(&unshifted_environment, 0)
            .expect("the checked binder is immediately available");
        let shifted = resolve_checked_binder(&shifted_environment, 1)
            .expect("the same checked binder remains available past one binder");

        assert_eq!(unshifted.provenance, shifted.provenance);
        assert_eq!(
            unshifted.provenance,
            CheckedBinderProvenance::InductionHypothesis(binding)
        );
        assert_eq!(unshifted.immediate_environment_index, 0);
        assert_eq!(shifted.immediate_environment_index, 1);
    }

pub(in crate::cranelift_backend::planning::static_transition)     fn contspec_seed_capture_worker_fixture() -> RuntimeExpr {
        // `Closure` rather than `LexicalClosure`: its captures are SYMBOLS, so
        // the ruled run's capture sources are `Seed`, not `Lexical`. That is the
        // one arm of the run derivation that refuses rather than admitting, and
        // without a fixture that reaches it the refusal is only ever read, never
        // exercised -- changing it to a `continue` leaves the suite green.
        let worker = RuntimeExpr::Closure {
            captures: vec!["seed_a".to_string(), "seed_b".to_string()],
            params: vec!["worker".to_string()],
            body: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::Contspec::Leaf".to_string(),
                args: Vec::new(),
            }),
        };
        RuntimeExpr::LexicalClosure {
            captures: Vec::new(),
            params: vec!["continuation_input".to_string()],
            body: Box::new(RuntimeExpr::ComputationalMatch {
                scrutinee: Box::new(RuntimeExpr::Construct {
                    constructor: "ctor:fixture::Contspec::Node".to_string(),
                    args: vec![worker],
                }),
                cases: vec![RuntimeComputationalMatchCase {
                    constructor: "ctor:fixture::Contspec::Node".to_string(),
                    argument_binders: 1,
                    recursive_positions: vec![0],
                    body: unit(),
                }],
                default: trap("seed capture worker"),
            }),
        }
    }

pub(in crate::cranelift_backend::planning::static_transition)     fn contspec_capture_free_worker_fixture() -> RuntimeExpr {
        // Identical to the captured fixtures EXCEPT the worker captures
        // nothing. This is the negative half of the discriminating pair, and it
        // is non-degenerate on purpose: it still builds a continuation unit with
        // a ruled envelope, so "no checked-IH record" here is a statement about
        // the CAPTURE-FREE population rather than about a program that produced
        // no units at all.
        let worker = RuntimeExpr::LexicalClosure {
            captures: Vec::new(),
            params: vec!["worker".to_string()],
            body: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::Contspec::Leaf".to_string(),
                args: Vec::new(),
            }),
        };
        RuntimeExpr::LexicalClosure {
            captures: Vec::new(),
            params: vec!["continuation_input".to_string()],
            body: Box::new(RuntimeExpr::ComputationalMatch {
                scrutinee: Box::new(RuntimeExpr::Construct {
                    constructor: "ctor:fixture::Contspec::Node".to_string(),
                    args: vec![worker],
                }),
                cases: vec![RuntimeComputationalMatchCase {
                    constructor: "ctor:fixture::Contspec::Node".to_string(),
                    argument_binders: 1,
                    recursive_positions: vec![0],
                    body: unit(),
                }],
                default: trap("capture free worker"),
            }),
        }
    }

pub(in crate::cranelift_backend)     fn contspec_activation_owned_worker_captures_fixture() -> RuntimeExpr {
        // ⛔ The captures are `Construct`s WITH A FIELD, not `unit()`, and that
        // is the entire point of this fixture existing beside
        // `contspec_multiple_worker_captures_fixture`. A `unit()` capture is a
        // persistent ground value, so every child of the record it produces is
        // `Persistent` and the record is `PersistentGround` -- which exercises
        // exactly one side of the lifetime/allocation derivation and leaves the
        // escaping side, the side the checked-IH population actually lives on,
        // untested. An allocated aggregate is owned by the invocation arena.
        // The run ALTERNATES the two lifetime arms rather than committing to
        // one. An all-escaping run would test the escaping side but would still
        // be satisfied by a record-wide constant; a MIXED run is refuted by any
        // constant, in either direction, because the record must carry both
        // answers at once. `Effect` is `ActivationOwned` by the occurrence
        // plane's own expression-kind rule; `unit()` is persistent ground.
        let capture = |tag: usize| {
            if tag % 2 == 0 {
                RuntimeExpr::Effect {
                    family: "Console".to_string(),
                    operation: ken_host::HostOpV1::ConsoleRead,
                    capability: None,
                    args: Vec::new(),
                }
            } else {
                unit()
            }
        };
        let worker = RuntimeExpr::LexicalClosure {
            captures: (0..9).map(capture).collect(),
            params: vec!["worker".to_string()],
            body: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::Contspec::Leaf".to_string(),
                args: Vec::new(),
            }),
        };
        RuntimeExpr::LexicalClosure {
            captures: Vec::new(),
            params: vec!["continuation_input".to_string()],
            body: Box::new(RuntimeExpr::ComputationalMatch {
                scrutinee: Box::new(RuntimeExpr::Construct {
                    constructor: "ctor:fixture::Contspec::Node".to_string(),
                    args: vec![worker],
                }),
                cases: vec![RuntimeComputationalMatchCase {
                    constructor: "ctor:fixture::Contspec::Node".to_string(),
                    argument_binders: 1,
                    recursive_positions: vec![0],
                    body: unit(),
                }],
                default: trap("activation owned worker captures"),
            }),
        }
    }

pub(in crate::cranelift_backend::planning::static_transition)     fn contspec_multiple_worker_captures_fixture() -> RuntimeExpr {
        let worker = RuntimeExpr::LexicalClosure {
            captures: vec![unit(), unit()],
            params: vec!["worker".to_string()],
            body: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::Contspec::Leaf".to_string(),
                args: Vec::new(),
            }),
        };
        RuntimeExpr::LexicalClosure {
            captures: Vec::new(),
            params: vec!["continuation_input".to_string()],
            body: Box::new(RuntimeExpr::ComputationalMatch {
                scrutinee: Box::new(RuntimeExpr::Construct {
                    constructor: "ctor:fixture::Contspec::Node".to_string(),
                    args: vec![worker],
                }),
                cases: vec![RuntimeComputationalMatchCase {
                    constructor: "ctor:fixture::Contspec::Node".to_string(),
                    argument_binders: 1,
                    recursive_positions: vec![0],
                    body: unit(),
                }],
                default: trap("multiple worker captures"),
            }),
        }
    }

    /// A `ComputationalMatch` consumer sitting inside a `Match` case body, with
    /// a `Let`-bound host-effect result above both.
    ///
    /// The environment reaching that consumer therefore holds **both** `D2`
    /// binding kinds and an entry parameter, in one vector, which is what lets
    /// one walk distinguish them instead of three fixtures each proving its own
    /// half.
    fn contsrc_d2_both_binding_kinds_fixture() -> RuntimeExpr {
        RuntimeExpr::Let {
            value: Box::new(RuntimeExpr::Effect {
                family: "Console".to_string(),
                operation: ken_host::HostOpV1::ConsoleRead,
                capability: None,
                args: Vec::new(),
            }),
            body: Box::new(RuntimeExpr::Match {
                scrutinee: Box::new(RuntimeExpr::Construct {
                    constructor: "ctor:fixture::Contspec::Node".to_string(),
                    args: vec![unit()],
                }),
                cases: vec![RuntimeMatchCase {
                    constructor: "ctor:fixture::Contspec::Node".to_string(),
                    binders: 1,
                    // ⛔ `Var(3)` is load-bearing, not decoration. The case
                    // body must REACH past the two computational binders to the
                    // surrounding environment, or `required_input_count` is
                    // zero, `exact_inputs` is empty, and every assertion below
                    // about the gate is satisfied by a fixture that never
                    // reached it. Index 3 = the two `ComputationalMatch`
                    // binders, then the enclosing `Match` binder.
                    body: contspec_parameter_match(RuntimeExpr::Var(3)),
                }],
                default: trap("d2 both binding kinds"),
            }),
        }
    }

    /// The environment the walk reaches at `target`, and that target's owner.
    fn contsrc_d2_reached_environment(
        plan: &StaticTransitionPlan<'_>,
        target: StaticOriginId,
    ) -> Vec<ContinuationValueSourceAuthority> {
        let owner = occurrence_authority(plan, target)
            .expect("the target has an occurrence authority")
            .owner;
        let entry_sources = continuation_owner_entry_sources(plan, owner)
            .expect("the owner has an exact entry environment");
        let entry_environment = entry_sources
            .into_iter()
            .map(ContinuationValueSourceAuthority::source)
            .collect::<Vec<_>>();
        let root = continuation_owner_source_root(plan, owner).expect("one source root");
        let (_, reached) =
            walk_continuation_value_environment(plan, root, target, &entry_environment)
                .expect("the walk reaches the target");
        reached.expect("the target is inside its owner's subtree")
    }

    /// The first occurrence of the given expression shape, in origin order.
    fn contsrc_d2_first_origin(
        plan: &StaticTransitionPlan<'_>,
        matches_shape: impl Fn(&RuntimeExpr) -> bool,
    ) -> StaticOriginId {
        let mut origins = plan
            .occurrence_authorities
            .iter()
            .map(|authority| authority.origin)
            .collect::<Vec<_>>();
        origins.sort();
        origins
            .into_iter()
            .find(|origin| {
                plan.planned_occurrence_expr(*origin)
                    .is_ok_and(&matches_shape)
            })
            .expect("the fixture contains that shape")
    }

    /// `D2b` — find a **lawful** emission seat whose lexical environment holds
    /// `coordinate` at an index the introduction index does not predict.
    ///
    /// ⭐ The seat is *searched for among real occurrences*, never fabricated:
    /// it must be an occurrence of `owner` and a genuine construct origin of
    /// some result edge, which is exactly what
    /// [`continuation_emission_seat_environment`] demands. A hand-picked origin
    /// would be the place this row stopped measuring the production check.
    ///
    /// Returns `(result_origin, construct_origin, index_at_that_seat)`.
    fn contsrc_d2b_shifted_emission_seat(
        plan: &StaticTransitionPlan<'_>,
        owner: PredeclaredFunctionId,
        coordinate: ContinuationSourceCoordinate,
        introduction_index: u32,
    ) -> (StaticOriginId, StaticOriginId, u32) {
        let mut origins = plan
            .occurrence_authorities
            .iter()
            .map(|authority| authority.origin)
            .collect::<Vec<_>>();
        origins.sort();
        for result_origin in origins.iter().copied() {
            let Ok(constructs) = continuation_result_origins(plan, result_origin) else {
                continue;
            };
            for construct_origin in constructs.iter().copied() {
                let Ok(authority) = occurrence_authority(plan, construct_origin) else {
                    continue;
                };
                if authority.owner != owner {
                    continue;
                }
                let environment = ContinuationProducerEnvironment {
                    producer_owner: owner,
                    producer_result_origin: result_origin,
                    producer_construct_origin: construct_origin,
                    consumer_owner: owner,
                    inputs: Vec::new(),
                };
                let Ok((_, seat)) = continuation_emission_seat_environment(plan, &environment)
                else {
                    continue;
                };
                let found = seat.iter().position(|value| {
                    matches!(
                        value,
                        ContinuationValueSourceAuthority::Closed(sources)
                            if sources.iter().any(|source| source.coordinate == coordinate)
                    )
                });
                let Some(index) = found else { continue };
                let index = u32::try_from(index).expect("a fixture environment index fits");
                if index > introduction_index {
                    return (result_origin, construct_origin, index);
                }
            }
        }
        panic!(
            "the fixture has no lawful emission seat holding {coordinate:?} past an intervening \
             binder; without one this row would measure the unshifted case and could not tell a \
             real nearest-alias selection from returning the introduction index"
        );
    }

    fn contsrc_d2_local(
        value: &ContinuationValueSourceAuthority,
    ) -> (&ContinuationSourceSlotAuthority, ProducerLocalBinding, ProducerLocalLocator) {
        let ContinuationValueSourceAuthority::Closed(sources) = value else {
            panic!("expected an exactly-sourced value, got {value:?}");
        };
        let [source] = sources.as_slice() else {
            panic!("expected exactly one source, got {sources:?}");
        };
        let ContinuationSourceCoordinate::ProducerLocal { binding, locator } = source.coordinate
        else {
            panic!("expected a producer-local coordinate, got {:?}", source.coordinate);
        };
        (source, binding, locator)
    }

    fn mutate_projection_field(
        projection: &mut ContinuationInputProjection,
        field: ContinuationProjectionOmission,
    ) {
        match field {
            ContinuationProjectionOmission::ProducerOwner => {
                projection.producer_owner = PredeclaredFunctionId(u32::MAX)
            }
            ContinuationProjectionOmission::ConsumerOwner => {
                projection.consumer_owner = PredeclaredFunctionId(u32::MAX)
            }
            // `D1` — same three components, now inside the `EntryAbi` arm.
            // ⛔ The sentinel is written INTO the arm, never by replacing the
            // arm: swapping the whole coordinate for a producer-local one would
            // make every row of the matrix pass on the domain tag alone.
            ContinuationProjectionOmission::SourceOwner
            | ContinuationProjectionOmission::SourceAbiPosition
            | ContinuationProjectionOmission::Source => {
                let ContinuationSourceCoordinate::EntryAbi {
                    source_owner,
                    source_abi_position,
                    source,
                } = &mut projection.coordinate
                else {
                    panic!("the AC-2 omission matrix reached a producer-local coordinate");
                };
                match field {
                    ContinuationProjectionOmission::SourceOwner => {
                        *source_owner = PredeclaredFunctionId(u32::MAX)
                    }
                    ContinuationProjectionOmission::SourceAbiPosition => {
                        *source_abi_position = u32::MAX
                    }
                    ContinuationProjectionOmission::Source => {
                        *source = ContinuationInputSource::SeedCapture {
                            defining_origin: StaticOriginId(u32::MAX),
                        }
                    }
                    other => panic!("{other:?} is not a coordinate component"),
                }
            }
            ContinuationProjectionOmission::Ordinal => projection.ordinal = u32::MAX,
            ContinuationProjectionOmission::Carrier => {
                projection.carrier = AbiCarrier::GroundValueCarrier
            }
            ContinuationProjectionOmission::Ownership => {
                projection.ownership = AbiOwnership::TransferredToCaller
            }
            ContinuationProjectionOmission::StorageOwner => {
                projection.storage_owner = AbiStorageOwner::ArtifactStatic
            }
            ContinuationProjectionOmission::ReferentAffinity => {
                projection.referent_affinity = vec![BoundaryReferentOwner::PersistentStore]
            }
            ContinuationProjectionOmission::OrdinaryAbiPosition => {
                projection.ordinary_abi_position = u32::MAX
            }
        }
    }

    /// Build one planned edge for a target, with the token fields the closure
    /// validator checks taken from that target's own key.
    ///
    /// Taking them from the key is what makes the negative row below reach the
    /// NEW check: a hand-picked token would trip "edge token disagrees with its
    /// exact target" first, and a control that refuses at an earlier rule has
    /// not exercised the rule it names.
    #[cfg(test)]
    fn contspec_edge_for(
        unit: &PlannedContinuationSpecialization,
        call_site_sequence: u32,
    ) -> PlannedContinuationSpecializationCall {
        PlannedContinuationSpecializationCall {
            token: ContinuationSpecializationCallToken {
                producer_owner: unit.key.producer_owner,
                emission_owner: unit.key.emission_owner,
                producer_result_origin: unit.key.producer_result_origin,
                producer_construct_origin: unit.key.producer_construct_origin,
                producer_alternative: unit.key.producer_alternative,
                call_site_sequence,
                target: unit.id,
                worker: unit.key.worker.clone(),
            },
        }
    }

    /// **`RT-DECL-CLOSURE-PORT` `D3` — the four transport classes are present
    /// and typed at the declaration-owned callable-unit boundary.**
    ///
    /// Capture, parameter, result and trap, asserted as the **exact ordered slot
    /// run** rather than as counts: a count agrees with a run that has the right
    /// number of wrong slots.
    #[test]
    fn d3_the_callable_declaration_boundary_carries_typed_transport() {
        let (root, declaration) = d2_declaration_and_anonymous_closure();
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::d2", &declaration);
        let plan = plan_static_transition_graph(&root, &declarations).expect("plannable");
        let declaration_origin = plan
            .declaration_occurrence_origin("decl:fixture::d2")
            .expect("occurrence");

        let unit = plan
            .emittable_units()
            .expect("validated units")
            .into_iter()
            .find(|unit| {
                matches!(
                    unit.definition(),
                    AbiUnitDefinition::CallableDeclaration { declaration_origin: origin, .. }
                        if origin == declaration_origin
                )
            })
            .expect("the declaration owns a callable unit");

        let run = unit
            .slots()
            .iter()
            .map(|slot| (slot.kind, slot.carrier, slot.ordinal))
            .collect::<Vec<_>>();
        assert_eq!(
            run,
            vec![
                // the declared parameter
                (AbiSlotKind::Parameter, AbiCarrier::ValueWord, 0),
                // the lifted captures, in capture declaration order
                (AbiSlotKind::Capture, AbiCarrier::ValueWord, 0),
                (AbiSlotKind::Capture, AbiCarrier::ValueWord, 1),
                // the result / control / trap / store convention
                (AbiSlotKind::Result, AbiCarrier::ResultWord, 0),
                (AbiSlotKind::Control, AbiCarrier::ControlWord, 0),
                (AbiSlotKind::Trap, AbiCarrier::TrapWord, 0),
                (AbiSlotKind::Store, AbiCarrier::StoreHandle, 0),
            ],
            "D3: the callable declaration boundary must carry typed parameter, \
             capture, result and trap transport in ABI order"
        );

        // Ownership and storage owner are part of "typed", not decoration: a
        // capture that transferred to the caller, or lived in the persistent
        // store, would be a different transport with the same slot kinds.
        for slot in unit.slots() {
            let expected = match slot.kind {
                AbiSlotKind::Parameter | AbiSlotKind::Capture | AbiSlotKind::Control
                | AbiSlotKind::Trap => (AbiOwnership::OwnedByFrame, AbiStorageOwner::ActivationFrame),
                AbiSlotKind::Result => (
                    AbiOwnership::TransferredToCaller,
                    AbiStorageOwner::ActivationFrame,
                ),
                AbiSlotKind::Store => (
                    AbiOwnership::BorrowedForActivation,
                    AbiStorageOwner::PersistentStore,
                ),
            };
            assert_eq!(
                (slot.ownership, slot.storage_owner),
                expected,
                "D3: {:?} slot crosses with the wrong ownership/storage owner",
                slot.kind
            );
        }
    }

    /// **`RT-DECL-CLOSURE-PORT` `D3` — the SEED producer transports too, with
    /// its own carrier.**
    ///
    /// ⭐ There are **two** `StaticBody` producers, `Closure` and
    /// `LexicalClosure`, and they differ in exactly the axis under test: a seed
    /// capture crosses as `GroundValueCarrier`, a lexical one as `ValueWord`. A
    /// fixture exercising one producer leaves the other's transport unmeasured,
    /// and "the ported declaration works" would then be a claim about half the
    /// population.
    #[test]
    fn d3_a_seed_provenance_declaration_transports_with_its_own_carrier() {
        let declaration = RuntimeDeclaration {
            symbol: "decl:fixture::d3seed".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::Closure {
                    captures: vec!["decl:fixture::cap".to_string()],
                    params: vec!["arg0".to_string()],
                    body: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
                },
            },
            metadata: crate::RuntimeSymbolMetadata::empty(),
        };
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::d3seed", &declaration);
        let plan = plan_static_transition_graph(&RuntimeExpr::Value(RuntimeValue::Bool(true)), &declarations)
            .expect("plannable");
        let declaration_origin = plan
            .declaration_occurrence_origin("decl:fixture::d3seed")
            .expect("occurrence");

        let unit = plan
            .emittable_units()
            .expect("validated units")
            .into_iter()
            .find(|unit| {
                matches!(
                    unit.definition(),
                    AbiUnitDefinition::CallableDeclaration { declaration_origin: origin, .. }
                        if origin == declaration_origin
                )
            })
            .expect("the seed declaration owns a callable unit");

        assert_eq!(
            unit.definition(),
            AbiUnitDefinition::CallableDeclaration {
                declaration_origin,
                provenance: AbiCaptureProvenance::Seed,
            },
            "D3: a Closure-bodied declaration must own a SEED-provenance callable unit"
        );
        let captures = unit
            .slots()
            .iter()
            .filter(|slot| slot.kind == AbiSlotKind::Capture)
            .map(|slot| slot.carrier)
            .collect::<Vec<_>>();
        assert_eq!(
            captures,
            vec![AbiCarrier::GroundValueCarrier],
            "D3: a seed capture must cross as GroundValueCarrier, not as the \
             lexical ValueWord -- the carrier is a function of provenance"
        );
    }

    /// **`RT-DECL-CLOSURE-PORT` `D3` — `C4` actually REJECTS on a
    /// declaration-owned unit, and the control proves that is not free.**
    ///
    /// ⭐ This is the one the leader named: routing `C4` through the shared
    /// predicate in `D2` is only worth something if the exclusion genuinely
    /// fires for the new arm. ⛔ A green suite cannot establish that — an
    /// exclusion that silently stopped seeing these units also reports no
    /// violation.
    #[test]
    fn d3_an_imported_capture_on_a_declaration_owned_unit_is_refused() {
        // The imported EDGE: an imported value in a capture position, which is
        // where it would have to cross a frame boundary and be given a carrier.
        let declaration = RuntimeDeclaration {
            symbol: "decl:fixture::d3".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::LexicalClosure {
                    captures: vec![RuntimeExpr::ImportedDeclarationRef {
                        symbol: "decl:other::imported".to_string(),
                        dependency: "pkg:other".to_string(),
                        dependency_semantic_hash: "hash".to_string(),
                    }],
                    params: Vec::new(),
                    body: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
                },
            },
            metadata: crate::RuntimeSymbolMetadata::empty(),
        };
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::d3", &declaration);
        let root = RuntimeExpr::Value(RuntimeValue::Bool(true));

        let refused = plan_static_transition_graph(&root, &declarations);
        assert!(
            refused.is_err(),
            "D3/C4: an imported capture edge on a declaration-owned callable \
             unit must be refused before it receives a callable descriptor"
        );

        // ⭐ The control. With `C4` restored to matching `ClosureBody` alone,
        // the SAME program is accepted -- so the assertion above is caused by
        // the shared predicate reaching the new arm, not by some other refusal
        // that would have fired anyway.
        let accepted_under_mutation =
            super::abi::D3_C4_MATCHES_CLOSURE_BODY_ONLY.with(|flag| {
                flag.set(true);
                let outcome = plan_static_transition_graph(&root, &declarations).is_ok();
                flag.set(false);
                outcome
            });
        assert!(
            accepted_under_mutation,
            "D3/C4: the refusal must be CAUSED by C4 reaching the declaration-owned \
             arm -- if the program is still refused with C4 narrowed back to \
             ClosureBody, this test is measuring a different rejection and proves \
             nothing about the population shrink"
        );
    }

    /// **`D3` — CALL TARGET IS INJECTIVE, and an interning ALIAS refuses at
    /// planner closure rather than being defended late in the emitter.**
    ///
    /// Ruled at `evt_7akh94dvqeqap`. The closure validator already proved
    /// key/unit bijection, unique tokens, token/target agreement and surjective
    /// reachability. It did **not** prove that two distinct edges cannot name
    /// one unit, and that gap is what had pushed an all-incoming-calls liveness
    /// scan into lowering -- a scan no lawful source can make fail.
    ///
    /// **The negative reaches the NEW rule and not an older one, which is the
    /// whole difficulty of this row.** The two keys differ **only** in a field
    /// inside `continuation_inputs`, and no such field appears in the call
    /// token; both edges therefore still agree with their target on every field
    /// the existing check compares. Under exact interning they are two units and
    /// the population is bijective; under `OmitProjection` they conflate to one
    /// unit and the two distinct edges become an alias.
    ///
    /// **MEASURED:** exact interning gives two units, two distinct edges, two
    /// distinct targets, and closure passes. The same two keys and the same two
    /// edges under `OmitProjection` give one unit and refuse at the duplicate
    /// TARGET rule -- not at the duplicate-token rule, which is a separate
    /// defect and is left free to fire on its own row.
    /// **CLAIMED:** a specialization's liveness is decided by its own unique
    /// edge, because the planner refuses any state in which it would not be.
    /// **THE GAP:** this is planner closure only. It pins no emitter behaviour
    /// and no composition disposition; the `ComposedCall`/`DirectCall` outcome
    /// this law makes well-defined is owed with the relation.
    #[test]
    fn d3_two_distinct_planned_edges_may_not_name_one_specialization() {
        let plan = contspec_plan();
        let base_key = plan.continuation_specializations[0].key.clone();
        let field = ContinuationProjectionOmission::Ordinal;

        let mut aliasing_key = base_key.clone();
        mutate_projection_field(&mut aliasing_key.continuation_inputs[0], field);
        assert_ne!(
            base_key, aliasing_key,
            "the two keys must be exactly distinct, or the row tests nothing"
        );

        // Exact interning: two units, two edges, bijective.
        let mut interned = BTreeMap::new();
        let mut units = Vec::new();
        intern_specialization(&mut interned, &mut units, base_key.clone()).expect("interns");
        intern_specialization(&mut interned, &mut units, aliasing_key.clone()).expect("interns");
        let exact_units = units.len();
        let edges = vec![
            contspec_edge_for(&units[0], 0),
            contspec_edge_for(&units[1], 0),
        ];
        let exact = validate_continuation_specialization_closure(&interned, &units, &edges);

        // The SAME two keys under projection-omitting interning: one unit.
        let mut aliased_interned = BTreeMap::new();
        let mut aliased_units = Vec::new();
        CONTINUATION_INTERN_MUTATION
            .with(|mutation| mutation.set(ContinuationInternMutation::OmitProjection(field)));
        intern_specialization(&mut aliased_interned, &mut aliased_units, base_key.clone())
            .expect("interns");
        intern_specialization(&mut aliased_interned, &mut aliased_units, aliasing_key)
            .expect("interns");
        CONTINUATION_INTERN_MUTATION
            .with(|mutation| mutation.set(ContinuationInternMutation::Exact));
        let aliased_unit_count = aliased_units.len();
        // Two DISTINCT edges -- different call-site sequences, so the
        // duplicate-token rule cannot be what answers -- both landing on the one
        // conflated unit.
        let aliased_edges = vec![
            contspec_edge_for(&aliased_units[0], 0),
            contspec_edge_for(&aliased_units[0], 1),
        ];
        assert_ne!(
            aliased_edges[0].token, aliased_edges[1].token,
            "the two edges must be distinct tokens, or this row would refuse at the duplicate-\
             token rule instead of the one it names"
        );
        let aliased = validate_continuation_specialization_closure(
            &aliased_interned,
            &aliased_units,
            &aliased_edges,
        );

        let refusal = |result: Result<(), CraneliftBackendError>| match result {
            Ok(()) => "closed".to_string(),
            Err(CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(message))) => {
                message
            }
            Err(other) => format!("other: {other:?}"),
        };

        assert_eq!(
            (
                exact_units,
                refusal(exact),
                aliased_unit_count,
                refusal(aliased)
            ),
            (
                2,
                "closed".to_string(),
                1,
                "two distinct continuation planned edges name one specialization unit, so the \
                 planner's call and unit populations are not bijective and a specialization's \
                 liveness is not decided by its own edge"
                    .to_string()
            ),
            "exact interning closes with two units and two edges; the same two keys conflated by \
             projection omission refuse at the duplicate-TARGET rule, with both edges still \
             agreeing with their target on every field the older checks compare"
        );
    }

    /// **`D3` — A SAME-BODY SIBLING IS TWO UNITS WITH TWO EDGES, and body
    /// equality is never liveness authority.**
    ///
    /// Ruled at `evt_7akh94dvqeqap` point 3. Two exact, closure-valid full keys
    /// that share the worker body and provenance but differ on one legitimate
    /// identity coordinate must intern to distinct units with distinct edges, so
    /// that composing one can never suppress the other.
    ///
    /// **This is a PLANNER-RELATION row and deliberately not a source program.**
    /// It says nothing about whether a Ken program can produce this population --
    /// ten measured configurations did not -- and it must not be read as source
    /// reachability. It uses the exact interning path, never a coarsening
    /// mutation.
    ///
    /// **MEASURED:** worker body and full worker provenance equal, one identity
    /// coordinate differs, two distinct units, two distinct edges, closure
    /// passes.
    /// **CLAIMED:** shared body cannot alias two specializations, so a rule that
    /// keyed liveness on body would be deciding the sibling's fate too.
    /// **THE GAP:** the composed/direct halves of this row -- that only the
    /// composed unit leaves the executable population -- need the composition
    /// relation and are owed with it.
    #[test]
    fn d3_a_same_body_sibling_interns_as_two_units_with_two_edges() {
        let plan = contspec_plan();
        let left = plan.continuation_specializations[0].key.clone();
        let mut right = left.clone();
        // One legitimate identity coordinate, chosen because it is NOT part of
        // the worker provenance: the sibling stays same-body by construction.
        right.recursive_position += 1;
        right.recursive_positions.insert(right.recursive_position);

        let mut interned = BTreeMap::new();
        let mut units = Vec::new();
        let (left_id, _) =
            intern_specialization(&mut interned, &mut units, left.clone()).expect("interns");
        let (right_id, _) =
            intern_specialization(&mut interned, &mut units, right.clone()).expect("interns");
        let edges = vec![
            contspec_edge_for(&units[0], 0),
            contspec_edge_for(&units[1], 0),
        ];
        let closed =
            validate_continuation_specialization_closure(&interned, &units, &edges).is_ok();

        assert_eq!(
            (
                left.worker == right.worker,
                left.worker.body_origin == right.worker.body_origin,
                left != right,
                left_id != right_id,
                edges[0].token.target != edges[1].token.target,
                closed,
            ),
            (true, true, true, true, true, true),
            "the sibling shares worker provenance AND body origin, differs on one identity \
             coordinate, and still interns to a DISTINCT unit with a distinct edge -- so no \
             body-keyed rule could compose one without deciding the other"
        );
    }

    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D3b` alias controls 3, 4 and 5 — the rule
    /// selects by ELIGIBILITY, and order only canonicalizes among proved aliases.**
    ///
    /// ⭐⭐ **Controls 1 and 3 select OPPOSITE ENDS of the environment, and that
    /// is the whole point of having both.** A suite carrying only the
    /// nearest-alias case passes just as well under a positional shortcut —
    /// "take the first member containing the coordinate" — because on that
    /// fixture the first member *is* the answer. Control 3 puts an ambiguous
    /// `Closed([S, T])` at the inner position and the exact singleton at the
    /// outer one, so a positional shortcut answers `0` and the ruled rule answers
    /// `2`. Only the pair distinguishes them.
    ///
    /// ⚠ **MEASURED**: eligibility is exact equality of the complete source-slot
    /// authority, and selection among eligible positions is the minimum index.
    /// **CLAIMED**: the rule is total over the environment — every position is
    /// classified before any is chosen. **THE GAP**: this exercises the rule as a
    /// function; that the planner and the consumer both *call* it is
    /// `d3b_the_duplicated_entry_source_selects_the_nearest_alias` below.
    ///
    /// **Promise class: durable invariant.**
    #[test]
    fn d3b_alias_eligibility_not_position_decides() {
        let expr = Box::leak(Box::new(contspec_complete_environment_fixture()));
        let symbols = crate::NativeProcessSymbols::legacy_prelude();
        let plan = plan_static_transition_graph_with_symbols(
            expr,
            &BTreeMap::new(),
            &symbols,
            AbiRootIngress::Process,
            false,
        )
        .expect("the complete-environment fixture plans");
        let owner = plan.continuation_specializations[0].key.consumer_owner;
        // ⛔ Real authorities, not hand-built ones: eligibility is whole-record
        // equality, so a synthetic S could differ from anything production ever
        // produces and the row would prove nothing about the live rule.
        let sources = continuation_owner_entry_sources(&plan, owner).expect("entry sources");
        assert!(
            sources.len() >= 2,
            "the fixture must supply two distinct source slots, or S and T cannot be told apart"
        );
        let s = sources[0].clone();
        let t = sources[1].clone();
        assert_ne!(s, t, "S and T must be distinct records");
        assert_ne!(
            s.coordinate, t.coordinate,
            "S and T must differ in COORDINATE, or the contract-mismatch arm would fire where \
             the rows below expect the ambiguity or absence one"
        );
        let closed = |sources: Vec<ContinuationSourceSlotAuthority>| {
            ContinuationValueSourceAuthority::Closed(sources)
        };

        // Control 1's shape, as a unit: two exact aliases, the nearer wins.
        let both = vec![closed(vec![s.clone()]), closed(vec![t.clone()]), closed(vec![s.clone()])];
        assert_eq!(
            nearest_exact_alias(&s, &both).expect("two exact aliases are eligible"),
            0,
            "among proved aliases the minimum de Bruijn index is selected"
        );

        // ⛔ Control 3 — inner ambiguous, outer exact. The OUTER is selected.
        let outer = vec![
            closed(vec![s.clone(), t.clone()]),
            closed(vec![t.clone()]),
            closed(vec![s.clone()]),
        ];
        assert_eq!(
            nearest_exact_alias(&s, &outer).expect("the outer singleton is eligible"),
            2,
            "an ambiguous Closed([S, T]) at the inner position is NOT an alias, so the rule must \
             reach past it; answering 0 here would mean it is selecting the first member that \
             merely contains the coordinate"
        );

        // ⛔ Control 4 — ambiguous with no singleton anywhere refuses.
        let ambiguous = vec![closed(vec![s.clone(), t.clone()])];
        let refusal = nearest_exact_alias(&s, &ambiguous)
            .expect_err("an ambiguous source set proves nothing and must refuse");
        assert!(
            format!("{refusal:?}").contains("ambiguous source set"),
            "the refusal must be the ambiguity one, not absence: {refusal:?}"
        );

        // ⛔ Control 5, unit form — same coordinate, different contract.
        let mut narrowed = s.clone();
        narrowed.referent_affinity = Vec::new();
        assert_ne!(narrowed, s, "the narrowing must actually change the record");
        assert_eq!(narrowed.coordinate, s.coordinate, "and must keep the coordinate");
        let mismatched = vec![closed(vec![narrowed])];
        let refusal = nearest_exact_alias(&s, &mismatched)
            .expect_err("a different contract under the same coordinate must refuse");
        assert!(
            format!("{refusal:?}").contains("different carrier, ownership, storage owner"),
            "the refusal must be the contract-mismatch one: {refusal:?}"
        );

        // ⛔ Absent entirely — the third refusal, kept distinguishable.
        let absent = vec![closed(vec![t])];
        let refusal =
            nearest_exact_alias(&s, &absent).expect_err("an absent coordinate must refuse");
        assert!(
            format!("{refusal:?}").contains("not present in the lexical environment"),
            "the refusal must be the absence one: {refusal:?}"
        );
    }

    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D3b` alias controls 1, 2 and 6 — the
    /// measured duplicate selects index 0, and the real consumer rederives it.**
    ///
    /// ⭐ This is the exact environment that produced the hard stop: `let y = x`
    /// forwards process parameter 1, so `EntryAbi { .., 1, Parameter }` occupies
    /// lexical indices **0 and 2**. The old exact-once law refused it outright.
    ///
    /// ⛔ **Control 2 is the canonicality half, and it is deliberately NOT stated
    /// as "index 2 holds a different value".** It does not — index 2 is a proved
    /// alias holding the same semantic source. What the consumer refuses is a
    /// claim that is not the *canonical* selection, and that distinction is the
    /// reason this rule is safe to share between two planes: planner and consumer
    /// run one function, so a claim either is what that function returns or is
    /// rejected.
    ///
    /// ⚠ **MEASURED**: the issued claim carries index 0; the production consumer
    /// accepts it and refuses index 2. **CLAIMED**: the claim a consumer indexes
    /// with is the planner's own answer, re-derived rather than trusted. **THE
    /// GAP**: this re-runs the planner's rule, so a defect in the rule itself
    /// would be reproduced rather than caught — `d3b_alias_eligibility_not_position_decides`
    /// owns that half.
    ///
    /// **Promise class: durable invariant.**
    #[test]
    fn d3b_the_duplicated_entry_source_selects_the_nearest_alias() {
        let expr = Box::leak(Box::new(contspec_complete_environment_fixture()));
        let symbols = crate::NativeProcessSymbols::legacy_prelude();
        let plan = plan_static_transition_graph_with_symbols(
            expr,
            &BTreeMap::new(),
            &symbols,
            AbiRootIngress::Process,
            false,
        )
        .expect("the complete-environment fixture plans");
        let unit = &plan.continuation_specializations[0];
        let input = &unit.key.continuation_inputs[0];
        let requested = ContinuationSourceSlotAuthority {
            coordinate: input.coordinate,
            carrier: input.carrier,
            ownership: input.ownership,
            storage_owner: input.storage_owner,
            referent_affinity: input.referent_affinity.clone(),
        };

        // The duplicate is real and MEASURED here, not assumed from the fixture's
        // name. ⛔ Without this the row could pass on an environment holding the
        // coordinate once, where nearest-alias and exact-once agree.
        let seat_environment = continuation_emission_seat_environment(
            &plan,
            &ContinuationProducerEnvironment {
                producer_owner: unit.key.producer_owner,
                producer_result_origin: unit.key.producer_result_origin,
                producer_construct_origin: unit.key.producer_construct_origin,
                consumer_owner: unit.key.consumer_owner,
                inputs: Vec::new(),
            },
        )
        .expect("the emission seat has an environment")
        .1;
        let exact_positions = seat_environment
            .iter()
            .enumerate()
            .filter(|(_, value)| {
                matches!(value, ContinuationValueSourceAuthority::Closed(sources)
                    if sources.as_slice() == [requested.clone()])
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        assert_eq!(
            exact_positions,
            vec![0, 2],
            "this row's whole subject is a coordinate at TWO exact-alias positions; one position \
             would make nearest-alias and the retired exact-once law indistinguishable"
        );

        let Some(ContinuationEnvironmentDraft::CurrentLexical {
            emission_owner,
            producer_result_origin,
            emission_origin,
            lexical_environment_origin,
            nearest_alias_index,
        }) = input.availability.direct_emission
        else {
            panic!("a predeclared emitter must issue a current-lexical direct-emission claim");
        };
        // ⛔ Control 1 — the nearer of the two proved aliases.
        assert_eq!(
            nearest_alias_index, 0,
            "the issued claim must carry the minimum eligible index"
        );

        // The REAL consumer accepts it.
        verify_current_lexical_availability(
            &plan,
            emission_owner,
            producer_result_origin,
            emission_origin,
            lexical_environment_origin,
            &requested,
            nearest_alias_index,
        )
        .expect("the production consumer must accept the planner's own answer");

        // ⛔ Control 2 — the outer alias is refused as non-canonical.
        let refusal = verify_current_lexical_availability(
            &plan,
            emission_owner,
            producer_result_origin,
            emission_origin,
            lexical_environment_origin,
            &requested,
            2,
        )
        .expect_err(
            "a claim naming the outer alias must be refused; accepting it would mean the \
             consumer indexes with whatever number it is handed as long as something lives there",
        );
        assert!(
            format!("{refusal:?}").contains("does not hold that coordinate at"),
            "the refusal must be the seat revalidation: {refusal:?}"
        );
    }

    /// MEASURED: the nested fixture produces two units and two exact causal
    /// edges; every key owns the full ordered two-input projection.
    ///
    /// CLAIMED: D1-D5 are a closed planner population before any consumer is
    /// exposed. GAP: Slice 2 still has to declare the ABI unit arm, and Slice 3
    /// still has to lower a call; this test claims neither.
    #[test]
    fn contspec_planner_closes_ordered_keys_units_and_causal_edges_dormantly() {
        let plan = contspec_plan();
        assert_eq!(plan.continuation_specializations.len(), 2);
        assert_eq!(plan.continuation_specialization_calls.len(), 2);
        for (index, unit) in plan.continuation_specializations.iter().enumerate() {
            assert_eq!(unit.id.0 as usize, index);
            assert_eq!(unit.key.continuation_inputs.len(), 2);
            assert_eq!(
                unit.key
                    .continuation_inputs
                    .iter()
                    .map(|input| input.ordinal)
                    .collect::<Vec<_>>(),
                vec![0, 1]
            );
            assert_eq!(
                unit.key
                    .continuation_inputs
                    .iter()
                    .map(|input| {
                        let (owner, position, _) = input.coordinate.expect_entry_abi();
                        (owner, position)
                    })
                    .collect::<Vec<_>>(),
                vec![(unit.key.consumer_owner, 0), (unit.key.consumer_owner, 1)]
            );
            assert!(matches!(
                unit.key.continuation_inputs[1].coordinate.expect_entry_abi().2,
                ContinuationInputSource::LexicalCapture { .. }
            ));
            assert_eq!(
                unit.key
                    .continuation_inputs
                    .iter()
                    .map(|input| input.ordinary_abi_position)
                    .collect::<Vec<_>>(),
                vec![1, 2]
            );
        }
        let targets = plan
            .continuation_specialization_calls
            .iter()
            .map(|call| call.token.target)
            .collect::<BTreeSet<_>>();
        assert_eq!(targets.len(), 2, "D5: one target was orphaned or conflated");
        validate_continuation_specialization_plan(&plan).expect("exact closure");
        plan.abi
            .validate_continuation_specializations(&plan.continuation_specializations)
            .expect("exact dormant ABI");
        assert_eq!(
            plan.abi.continuation_descriptors.len(),
            plan.continuation_specializations.len(),
            "D1: every interned specialization needs one explicit descriptor"
        );
        for (index, descriptor) in plan.abi.continuation_descriptors.iter().enumerate() {
            assert_eq!(
                descriptor.definition,
                AbiUnitDefinition::ContinuationSpecialization {
                    specialization: ContinuationSpecializationId(index as u32),
                },
                "D1: a continuation descriptor masqueraded as an existing unit arm"
            );
        }

        // Dormancy is a capability property: the existing emission population
        // is still exactly the pre-existing ABI descriptors. No accessor above
        // can project a continuation unit into this population.
        assert_eq!(
            plan.emittable_units().expect("existing units").len(),
            plan.abi.descriptors.len()
        );
    }

    /// MEASURED: `Let` forwards process parameter 1 into semantic environment
    /// ordinal 0. The first case uses no surrounding value, while the second
    /// case consumes outer ordinal 2, so the case maximum requires three
    /// inputs. Exact production retains source ABI positions `[1, 0, 1]`; the
    /// descriptor-count mutation truncates that population to `[1, 0]`, while
    /// descriptor restatement produces `[0, 1]`.
    ///
    /// CLAIMED: D1 obtains owner/position/provenance from the value that reaches
    /// each continuation-environment ordinal, not from that ordinal itself.
    ///
    /// GAP: this remains dormant planner authority. No continuation unit or
    /// call is emitted in Slice 1.
    #[test]
    fn contspec_locally_forwarded_parameter_retains_exact_source_position() {
        let expr = Box::leak(Box::new(
            contspec_complete_environment_fixture(),
        ));
        let symbols = crate::NativeProcessSymbols::legacy_prelude();
        let plan = plan_static_transition_graph_with_symbols(
            expr,
            &BTreeMap::new(),
            &symbols,
            AbiRootIngress::Process,
            false,
        )
        .expect("exact semantic source environment plans");
        assert_eq!(
            plan.continuation_specializations[0]
                .key
                .continuation_inputs
                .iter()
                .map(|input| input.coordinate.expect_entry_abi().1)
                .collect::<Vec<_>>(),
            vec![1, 0, 1],
        );

        CONTINUATION_PRODUCTION_MUTATION.with(|mutation| {
            mutation.set(ContinuationProductionMutation::DescriptorInputCountTruncation)
        });
        let truncated = plan_static_transition_graph_with_symbols(
            expr,
            &BTreeMap::new(),
            &symbols,
            AbiRootIngress::Process,
            false,
        )
        .expect("compile-valid descriptor-count truncation plans");
        CONTINUATION_PRODUCTION_MUTATION
            .with(|mutation| mutation.set(ContinuationProductionMutation::Exact));
        assert_eq!(
            truncated.continuation_specializations[0]
                .key
                .continuation_inputs
                .iter()
                .map(|input| input.coordinate.expect_entry_abi().1)
                .collect::<Vec<_>>(),
            vec![1, 0],
            "the production mutation must discard the required tail",
        );

        CONTINUATION_PRODUCTION_MUTATION.with(|mutation| {
            mutation.set(ContinuationProductionMutation::DescriptorOrdinalSources)
        });
        let wrong = plan_static_transition_graph_with_symbols(
            expr,
            &BTreeMap::new(),
            &symbols,
            AbiRootIngress::Process,
            false,
        )
        .expect("compile-valid descriptor-ordinal restatement plans");
        CONTINUATION_PRODUCTION_MUTATION
            .with(|mutation| mutation.set(ContinuationProductionMutation::Exact));
        assert_eq!(
            wrong.continuation_specializations[0]
                .key
                .continuation_inputs
                .iter()
                .map(|input| input.coordinate.expect_entry_abi().1)
                .collect::<Vec<_>>(),
            vec![0, 1],
            "the production mutation must restate descriptor ordinals",
        );
    }

    /// MEASURED: placing an opaque value, or a join of two distinct parameter
    /// sources, at required surrounding-environment ordinal 2 leaves the
    /// enclosing source program valid while producing no dormant continuation
    /// specialization. Ordinals 0 and 1 remain closed in both fixtures.
    ///
    /// CLAIMED: D1 refuses open and ambiguous source provenance at the
    /// candidate boundary; it neither invents a descriptor slot nor rejects
    /// an otherwise valid program.
    ///
    /// GAP: Slice 1 remains planner-only and emits neither refused candidate.
    #[test]
    fn contspec_open_and_ambiguous_sources_refuse_only_the_candidate() {
        let symbols = crate::NativeProcessSymbols::legacy_prelude();
        let open = Box::leak(Box::new(contspec_required_tail_fixture(unit())));
        let open_plan = plan_static_transition_graph_with_symbols(
            &*open,
            &BTreeMap::new(),
            &symbols,
            AbiRootIngress::Process,
            false,
        )
        .expect("open provenance refuses only the candidate");
        assert!(open_plan.continuation_specializations.is_empty());
        assert!(open_plan.continuation_specialization_calls.is_empty());

        CONTINUATION_PRODUCTION_MUTATION.with(|mutation| {
            mutation.set(ContinuationProductionMutation::DescriptorInputCountTruncation)
        });
        let truncated_open = plan_static_transition_graph_with_symbols(
            &*open,
            &BTreeMap::new(),
            &symbols,
            AbiRootIngress::Process,
            false,
        )
        .expect("descriptor-count truncation discards the open required tail");
        CONTINUATION_PRODUCTION_MUTATION
            .with(|mutation| mutation.set(ContinuationProductionMutation::Exact));
        assert_eq!(truncated_open.continuation_specializations.len(), 1);
        assert_eq!(
            truncated_open.continuation_specializations[0]
                .key
                .continuation_inputs
                .len(),
            2,
            "the mutation must admit the candidate by discarding ordinal 2",
        );

        let ambiguous_tail = RuntimeExpr::If {
            scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
            then_expr: Box::new(RuntimeExpr::Var(0)),
            else_expr: Box::new(RuntimeExpr::Var(1)),
        };
        let ambiguous = Box::leak(Box::new(contspec_required_tail_fixture(
            ambiguous_tail,
        )));
        let ambiguous_plan = plan_static_transition_graph_with_symbols(
            ambiguous,
            &BTreeMap::new(),
            &symbols,
            AbiRootIngress::Process,
            false,
        )
        .expect("ambiguous provenance refuses only the candidate");
        assert!(ambiguous_plan.continuation_specializations.is_empty());
        assert!(ambiguous_plan.continuation_specialization_calls.is_empty());
    }

    /// MEASURED: one static worker field and two ordered worker capture fields
    /// produce ABI prefix two. The static worker identity itself is excluded.
    /// The compile-valid constructor-field-count mutation produces prefix one.
    ///
    /// CLAIMED: D1 derives the prefix by walking the runtime worker envelope,
    /// not from constructor arity or a closure-count proxy.
    ///
    /// GAP: capture values remain intentionally absent from the immutable key;
    /// only their ordered static provenance participates.
    #[test]
    fn contspec_ordinary_prefix_uses_the_ordered_worker_envelope() {
        let expr = Box::leak(Box::new(contspec_multiple_worker_captures_fixture()));
        let plan = plan_static_transition_graph(expr, &BTreeMap::new()).expect("plans");
        let unit = plan
            .continuation_specializations
            .first()
            .expect("one continuation specialization");
        assert_eq!(plan.continuation_specializations.len(), 1);
        assert_eq!(unit.key.recursive_position, 0);
        assert_eq!(
            unit.key
                .worker
                .captures
                .iter()
                .map(|capture| capture.ordinal)
                .collect::<Vec<_>>(),
            vec![0, 1],
        );
        assert_eq!(unit.key.ordinary_parameters, 2);
        assert_eq!(
            unit.key.continuation_inputs[0].ordinary_abi_position,
            2,
        );

        CONTINUATION_PRODUCTION_MUTATION.with(|mutation| {
            mutation.set(ContinuationProductionMutation::ConstructorFieldCountPrefix)
        });
        let wrong = plan_static_transition_graph(expr, &BTreeMap::new())
            .expect("compile-valid constructor-field count plans");
        CONTINUATION_PRODUCTION_MUTATION
            .with(|mutation| mutation.set(ContinuationProductionMutation::Exact));
        assert_eq!(
            wrong.continuation_specializations[0].key.ordinary_parameters,
            1,
            "the production mutation must count the worker and omit both captures",
        );
        assert_eq!(
            wrong.continuation_specializations[0].key.continuation_inputs[0]
                .ordinary_abi_position,
            1,
        );
    }

    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D1`/`D2b` — the two planner-side consumers
    /// of the coordinate refuse the producer-local domain instead of reading an
    /// entry ABI position it does not have.**
    ///
    /// `D1` represents the domain; `D3` teaches the *emission* consumers to
    /// assign it. Between the two, the honest behaviour is a refusal, and a
    /// refusal nobody exercises is indistinguishable from a missing one — so
    /// this presents a producer-local coordinate directly.
    ///
    /// ⚠ **`D2b` changed the second half of this row, and the change is a
    /// strengthening rather than a weakening.** `exact_continuation_projection`
    /// no longer refuses *on the domain* — `D2b` gives the producer-local domain
    /// a real availability derivation. It refuses on the harder question the
    /// derivation asks: whether the coordinate is genuinely present in the
    /// lexical environment in force at the emission seat. A **fabricated**
    /// coordinate like [`ContinuationSourceCoordinate::producer_local_probe`] is
    /// present nowhere, so it is still refused — but now because no walk places
    /// it, which is the property `D2b` actually owes.
    ///
    /// ⚠ **`D3a` moved the FIRST half the same way, for the same reason.**
    /// `validate_continuation_source_slot` no longer refuses on the domain
    /// either — it re-derives producer-local coordinates by re-running the
    /// forward walk. A **fabricated** coordinate is still refused, but now
    /// because the re-derivation cannot reach it: this row's probe names
    /// `PredeclaredFunctionId(u32::MAX)` as its binding owner, and no such
    /// owner has a source root to walk from.
    ///
    /// ⛔ That refusal is real but *shallow* — it rejects the owner before any
    /// environment is consulted. The deeper property, that a coordinate with a
    /// genuine owner and a genuine scope is refused when the walk does not
    /// place it **there**, is deliberately not claimed here: it needs a
    /// coordinate this probe cannot express, and
    /// `contsrc_d3a_validator_rederives_a_producer_local_source` measures it
    /// with real owners, real scopes and a positive control.
    ///
    /// MEASURED: `validate_continuation_source_slot` returns `Err` on a
    /// producer-local coordinate whose binding owner does not exist;
    /// `exact_continuation_projection` returns `Err` on a producer-local
    /// coordinate that the forward semantic walk does not find at the emission
    /// seat; each returns `Ok` on the same record carrying its original entry
    /// coordinate. CLAIMED: neither consumer has a path that silently reads an
    /// entry position out of the local domain. THE GAP: this says nothing about
    /// whether either derivation assigns the *right* answer when the coordinate
    /// IS present — the `D2b` discriminators below and the `D3a` row above own
    /// that, and it is deliberately not claimed here.
    ///
    /// ⭐ The positive control is the load-bearing half. `Err` is satisfied by a
    /// record that was malformed for some unrelated reason, so each row proves
    /// the same record validates when its coordinate is the entry one.
    ///
    /// **Promise class: durable invariant.**
    #[test]
    fn contsrc_producer_local_coordinate_is_refused_by_both_planner_consumers() {
        let plan = contspec_plan();
        let unit = &plan.continuation_specializations[0];
        let entry_sources = continuation_owner_entry_sources(&plan, unit.key.consumer_owner)
            .expect("the consumer owner has an exact entry environment");
        let exact = entry_sources
            .first()
            .expect("the fixture consumer has at least one entry input")
            .clone();

        // Positive control: the untouched record validates.
        validate_continuation_source_slot(&plan, &exact)
            .expect("the exact entry-ABI authority must validate, or this row proves nothing");

        let mut local = exact.clone();
        local.coordinate = ContinuationSourceCoordinate::producer_local_probe();
        let refusal = validate_continuation_source_slot(&plan, &local)
            .expect_err("the exact slot validator must refuse a producer-local coordinate");
        assert_eq!(
            refusal,
            planner_error("continuation owner does not have one exact source-occurrence root"),
            "the validator must refuse with a message from its OWN re-derivation rather than \
             incidentally"
        );

        // `D3b` re-cut — the predeclared emitter's arm. BOTH coordinate domains
        // now take the forward lexical derivation; a fabricated local coordinate
        // is not in the seat environment, so it refuses.
        let entry_environment = ContinuationProducerEnvironment {
            producer_owner: unit.key.producer_owner,
            producer_result_origin: unit.key.producer_result_origin,
            producer_construct_origin: unit.key.producer_construct_origin,
            consumer_owner: unit.key.consumer_owner,
            inputs: vec![exact],
        };
        exact_continuation_projection(
            &plan,
            &entry_environment,
            unit.key.ordinary_parameters,
            &ContinuationEmitterFrame::Predeclared(entry_environment.producer_owner),
        )
        .expect("the entry-coordinate projection must succeed, or this row proves nothing");

        let local_environment = ContinuationProducerEnvironment {
            inputs: vec![local],
            ..entry_environment
        };
        let refusal = exact_continuation_projection(
            &plan,
            &local_environment,
            unit.key.ordinary_parameters,
            &ContinuationEmitterFrame::Predeclared(local_environment.producer_owner),
        )
        .expect_err("the projection must refuse a coordinate the seat environment does not hold");
        assert!(
            format!("{refusal:?}").contains("not present in the lexical environment"),
            "the projection must refuse because the forward walk does not place this binding at \
             the emission seat -- NOT incidentally, and not on the domain alone: {refusal:?}"
        );
    }

    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D1` `AC-2` — the coordinate is a CLOSED
    /// sum, and an entry position is not representable as a local binding.**
    ///
    /// The property `AC-2` actually needs is compile-time: a third domain must
    /// not compile until every consumer assigns it. That is carried by there
    /// being no wildcard arm at any of the matches, which no runtime assertion
    /// can observe. What *is* observable, and is the reason the sum exists, is
    /// that the two domains never compare equal — so an entry coordinate can
    /// never be mistaken for a local binding by a consumer comparing
    /// coordinates.
    ///
    /// ⛔ This is deliberately NOT an assertion about the source text of the
    /// matches. It tests the behaviour the type buys.
    ///
    /// **Promise class: durable invariant.**
    #[test]
    fn contsrc_the_two_coordinate_domains_never_compare_equal() {
        let plan = contspec_plan();
        let entry_sources = continuation_owner_entry_sources(
            &plan,
            plan.continuation_specializations[0].key.consumer_owner,
        )
        .expect("the consumer owner has an exact entry environment");
        let local = ContinuationSourceCoordinate::producer_local_probe();
        assert!(
            !entry_sources.is_empty(),
            "the fixture must supply at least one entry coordinate to compare against"
        );
        for source in &entry_sources {
            assert!(
                matches!(
                    source.coordinate,
                    ContinuationSourceCoordinate::EntryAbi { .. }
                ),
                "the entry walk must produce only entry coordinates"
            );
            assert_ne!(
                source.coordinate, local,
                "an entry coordinate compared equal to a producer-local one, so the \
                 generated-context capture lookup could resolve one as the other"
            );
        }
    }

    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D2` corrected — a `ComputationalMatch`
    /// case run is `[IH, argument]` and the two subruns are NOT contracted
    /// alike.**
    ///
    /// This is the discriminator `a5a6ce9b` lacked. That checkpoint looped over
    /// the combined binder count and stamped one contract across both subruns;
    /// its positive row targeted an inner `ComputationalMatch` but inspected
    /// that occurrence's *incoming* environment, so it observed an outer
    /// ordinary-`Match` binder and a host-effect result — never an IH.
    ///
    /// MEASURED: the outer case's environment is ordered
    /// `[Open, producer-local argument binder, ...]`. The IH prefix carries no
    /// contract at all; the argument binder carries the scrutinee's carrier and
    /// lifetime. CLAIMED: the two subruns are contracted from their own
    /// authorities, and the IH's is not invented. THE GAP: the IH's real
    /// contract is not derived here — it is refused. That is the hard stop
    /// reported with this correction, not a claim that `Open` is its answer.
    ///
    /// ⭐ **This rejects both stampings, which is the point.** Stamping the
    /// argument contract across the run makes position 0 `Closed`; stamping the
    /// IH treatment across it makes position 1 `Open`. The row asserts each
    /// position's domain exactly, so either stamp reds it — and the two
    /// assertions are what a single `binders`-wide loop cannot satisfy.
    ///
    /// **Promise class: durable invariant.**
    #[test]
    fn contsrc_d2_a_computational_case_run_separates_its_ih_prefix_from_its_arguments() {
        let expr = Box::leak(Box::new(contsrc_d2_ih_and_argument_case_fixture()));
        let plan = plan_static_transition_graph(expr, &BTreeMap::new())
            .expect("the IH/argument fixture plans");
        let mut computational = plan
            .occurrence_authorities
            .iter()
            .map(|authority| authority.origin)
            .filter(|origin| {
                plan.planned_occurrence_expr(*origin)
                    .is_ok_and(|expr| matches!(expr, RuntimeExpr::ComputationalMatch { .. }))
            })
            .collect::<Vec<_>>();
        computational.sort();
        let [outer, inner, ..] = computational.as_slice() else {
            panic!("the fixture must contain an outer and an inner ComputationalMatch");
        };
        let reached = contsrc_d2_reached_environment(&plan, *inner);
        assert!(
            reached.len() >= 2,
            "the walk must land on the outer case's own binder run: {reached:?}"
        );

        // ⛔ Position 0 is the recursive IH. No contract is claimed for it.
        // Stamping the argument contract across the run would make this
        // `Closed`, which is precisely the defect this row exists to catch.
        assert_eq!(
            reached[0],
            ContinuationValueSourceAuthority::Open,
            "the recursive IH prefix must carry NO contract; a producer-local value here is \
             the a5a6ce9b misclassification"
        );

        // Position 1 is the ordinary constructor argument binder, contracted
        // from the scrutinee. Stamping the IH treatment across the run would
        // make this `Open`.
        let (argument, binding, locator) = contsrc_d2_local(&reached[1]);
        assert_eq!(
            binding.binding_ordinal, 1,
            "the ordinal must span the whole run so identity stays (case body, ordinal) \
             with no new tag"
        );
        assert_eq!(binding.binding_origin, locator.environment_origin);
        assert_eq!(locator.environment_index, 1);

        // The contract is READ from the scrutinee, not chosen. The scrutinee is
        // a constructor of persistent children, so its lifetime is persistent
        // and its affinity is the two-element set -- which an IH's
        // activation-owned treatment could not produce.
        let scrutinee_lifetime = occurrence_authority(&plan, *outer)
            .expect("the outer match has an occurrence authority")
            .children
            .iter()
            .find(|child| child.position == 0)
            .expect("the outer match has a scrutinee child")
            .lifetime;
        // ⛔ Non-vacuity: the fixture's scrutinee must actually be PERSISTENT.
        // An activation-owned scrutinee would give the argument binder the same
        // affinity an IH's activation-owned treatment produces, and the
        // comparison below would hold for the wrong reason.
        assert_eq!(
            scrutinee_lifetime,
            PlannedReferentLifetime::Persistent,
            "the discriminator needs a persistent scrutinee, or the affinity assertion cannot \
             tell a scrutinee-derived contract from a stamped activation-owned one"
        );
        assert_eq!(
            argument.referent_affinity,
            lifetime_referent_affinity(scrutinee_lifetime),
            "the argument binder's affinity must be the scrutinee's, not a stamped constant"
        );
        assert_eq!(
            argument.ownership,
            argument.carrier.ownership(),
            "ownership must remain the carrier's projection"
        );
        assert_eq!(argument.storage_owner, argument.carrier.storage_owner());
    }

    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D2` — both binding kinds are populated as
    /// DISTINCT structural bindings, with a planner-derived contract.**
    ///
    /// One fixture, one walk, one environment vector holding a `Match` case
    /// binder, a `Let`-bound host-effect result and an entry parameter — so the
    /// row can tell the two local kinds apart from each other *and* from the
    /// entry domain, which three separate fixtures could not.
    ///
    /// ⛔ Both local values here are an **ordinary `Match` case's constructor
    /// argument binder** and a host-effect result. No recursive IH binder is in
    /// this fixture's environment; the IH/argument split is a separate row.
    ///
    /// MEASURED: at the consumer inside the case body the environment is
    /// `[argument binder, effect result, entry ...]`; the two local bindings
    /// differ in `binding_origin`; each carries the carrier its own authority
    /// supplies — the scrutinee's for the argument binder, the `Effect` shape's
    /// for the effect result — with the ownership and storage owner
    /// `AbiCarrier` derives from it, and a non-empty referent affinity.
    /// CLAIMED: `D2` populates both kinds through one derivation that restates
    /// no other record's fact. THE GAP: nothing here admits either binding —
    /// the candidate still declines, which is the next row.
    ///
    /// ⭐ The locator/binding split is asserted, not assumed: for the effect
    /// result the two origins **differ** (the `Effect` creates the value, the
    /// `Let` body holds it), and for the argument binder they coincide. If a
    /// future change collapsed the two fields, the effect row would red.
    ///
    /// **Promise class: durable invariant.**
    #[test]
    fn contsrc_d2_populates_both_producer_local_binding_kinds() {
        let expr = Box::leak(Box::new(contsrc_d2_both_binding_kinds_fixture()));
        let plan = plan_static_transition_graph(expr, &BTreeMap::new())
            .expect("the D2 fixture plans");
        let target = contsrc_d2_first_origin(&plan, |expr| {
            matches!(expr, RuntimeExpr::ComputationalMatch { .. })
        });
        let effect_origin = contsrc_d2_first_origin(&plan, |expr| {
            matches!(expr, RuntimeExpr::Effect { .. })
        });
        let reached = contsrc_d2_reached_environment(&plan, target);
        assert!(
            reached.len() >= 2,
            "the fixture must reach the consumer with both local bindings: {reached:?}"
        );

        // Position 0 -- the `Match` case binder. Its scope introduces it and
        // holds it, so binding and locator name the same occurrence.
        let (binder_source, binder_binding, binder_locator) = contsrc_d2_local(&reached[0]);
        assert_eq!(binder_binding.binding_ordinal, 0);
        assert_eq!(
            binder_binding.binding_origin, binder_locator.environment_origin,
            "a case binder is introduced by, and held in, the same scope"
        );
        assert_eq!(binder_locator.environment_index, 0);

        // Position 1 -- the `Let`-bound host-effect result. The `Effect`
        // creates it; the `Let` body holds it. ⛔ Different origins.
        let (effect_source, effect_binding, effect_locator) = contsrc_d2_local(&reached[1]);
        assert_eq!(
            effect_binding.binding_origin, effect_origin,
            "the host-effect result must be identified by the Effect occurrence itself"
        );
        assert_ne!(
            effect_binding.binding_origin, effect_locator.environment_origin,
            "the binding identity and the emission-time locator must not collapse into one"
        );
        assert_eq!(effect_locator.environment_index, 0);

        // The two kinds are distinct bindings, which is the deliverable.
        assert_ne!(
            binder_binding, effect_binding,
            "the two D2 binding kinds must not compare equal"
        );

        // The contract, derived once and consistent with the entry plane's
        // reading of the same carrier. ⛔ The carrier is asserted against the
        // authority that supplies it, never against a literal: on this fixture
        // both authorities answer `ValueWord`, and writing that constant here
        // would state a blanket rule the derivation deliberately does not have.
        let expected_argument_carrier = abi::result_carrier(SemanticSourceKind::Expression(
            RuntimeExprShape::Construct,
        ))
        .expect("the fixture's Match scrutinee is a Construct");
        let expected_effect_carrier =
            abi::result_carrier(SemanticSourceKind::Expression(RuntimeExprShape::Effect))
                .expect("the Effect shape has a result carrier");
        assert_eq!(binder_source.carrier, expected_argument_carrier);
        assert_eq!(effect_source.carrier, expected_effect_carrier);
        for (label, source) in [
            ("argument binder", binder_source),
            ("effect result", effect_source),
        ] {
            assert_eq!(
                source.ownership,
                source.carrier.ownership(),
                "{label} ownership must be the carrier's, not a second statement of it"
            );
            assert_eq!(
                source.storage_owner,
                source.carrier.storage_owner(),
                "{label} storage owner must be the carrier's"
            );
            assert!(
                !source.referent_affinity.is_empty(),
                "{label} must carry a real referent affinity; an empty one is what \
                 validate_continuation_source_slot rejects for the entry domain"
            );
        }

        // ⭐ The referent affinity is derived PER BINDING, not stamped from one
        // constant. The case binder's scrutinee is a persistent constructor and
        // the host effect's result is activation-owned, so the two affinities
        // must differ on this fixture. A single hardcoded affinity — the
        // easiest wrong implementation — reds here.
        assert_ne!(
            binder_source.referent_affinity, effect_source.referent_affinity,
            "both kinds received the same referent affinity, so the derivation is not \
             reading each binding's own lifetime authority"
        );
    }

    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D2b` DISCRIMINATOR 1 — current-lexical
    /// availability counts the binders actually pushed between the binding and
    /// the emission seat.**
    ///
    /// The defect `evt_44k69b55vhek2` reopened `D2` for is that `D1`'s locator
    /// is *scope-relative* — `environment_index` is where the value sits in the
    /// scope that introduced it — while the emitter stands somewhere else
    /// entirely. Any implementation that hands the locator index straight
    /// through looks correct on every fixture where nothing intervenes.
    ///
    /// ⭐ **So the fixture is chosen for the intervening binder.** The
    /// host-effect result is introduced at index 0 of the `Let` body, and the
    /// enclosing `Match` case pushes its own binder before the emission seat, so
    /// the value has moved by the time it is emitted. `nearest_alias_index` and
    /// `environment_index` are therefore *different numbers on this row* —
    /// which is what makes "introduction index equals emission index" a failing
    /// answer here rather than an indistinguishable one.
    ///
    /// MEASURED: the projection's `CurrentLexical` arm returns the index at
    /// which the emission seat's own environment holds this exact coordinate;
    /// that index differs from the binding's `environment_index`; the position
    /// the introduction index names at that seat carries a **different** value;
    /// and the arm is keyed to the exact emission occurrence and lexical
    /// environment origin. CLAIMED: the availability is derived by the forward
    /// semantic walk rather than restated from the locator. THE GAP: this says
    /// nothing about lowering *consuming* the arm — `D3` owns that, and the
    /// emission seams still refuse it.
    ///
    /// **Promise class: durable invariant.**
    #[test]
    fn contsrc_d2b_current_lexical_availability_counts_the_intervening_binder() {
        let expr = Box::leak(Box::new(contsrc_d2_both_binding_kinds_fixture()));
        let plan =
            plan_static_transition_graph(expr, &BTreeMap::new()).expect("the D2 fixture plans");
        let target = contsrc_d2_first_origin(&plan, |expr| {
            matches!(expr, RuntimeExpr::ComputationalMatch { .. })
        });
        let owner = occurrence_authority(&plan, target)
            .expect("the target has an occurrence authority")
            .owner;
        let reached = contsrc_d2_reached_environment(&plan, target);
        let (effect_source, _, effect_locator) = contsrc_d2_local(&reached[1]);
        let coordinate = effect_source.coordinate;
        let introduction_index = effect_locator.environment_index;

        let (result_origin, construct_origin, seat_index) =
            contsrc_d2b_shifted_emission_seat(&plan, owner, coordinate, introduction_index);
        let environment = ContinuationProducerEnvironment {
            producer_owner: owner,
            producer_result_origin: result_origin,
            producer_construct_origin: construct_origin,
            consumer_owner: owner,
            inputs: vec![effect_source.clone()],
        };

        // The independent oracle: the seat's own environment, read directly
        // rather than through the projection under test.
        let (source_root, seat) = continuation_emission_seat_environment(&plan, &environment)
            .expect("the searched seat is lawful by construction");
        let holds = |index: u32| {
            matches!(
                seat.get(index as usize),
                Some(ContinuationValueSourceAuthority::Closed(sources))
                    if sources.iter().any(|source| source.coordinate == coordinate)
            )
        };
        assert!(
            holds(seat_index),
            "the oracle must place this binding at {seat_index} or the row proves nothing"
        );
        // ⛔ THE vacuity kill. Had the derivation returned the locator's index,
        // it would have named a position holding a DIFFERENT value — so this
        // asserts the wrong answer is wrong, not merely that it is unequal.
        assert!(
            !holds(introduction_index),
            "the introduction index still holds this binding at the emission seat, so nothing \
             here distinguishes a real nearest-alias selection from passing the locator through"
        );

        let projected = exact_continuation_projection(
            &plan,
            &environment,
            0,
            &ContinuationEmitterFrame::Predeclared(environment.producer_owner),
        )
        .expect("a producer-local coordinate present at the seat must project");
        assert_eq!(
            projected[0].availability,
            ContinuationAvailabilityDraft {
                direct_emission: Some(ContinuationEnvironmentDraft::CurrentLexical {
                    emission_owner: environment.producer_owner,
                    producer_result_origin: result_origin,
                    emission_origin: construct_origin,
                    lexical_environment_origin: source_root,
                    nearest_alias_index: seat_index,
                }),
                // ⛔ The re-cut's load-bearing half of this row. A predeclared
                // emitter projects NO context-capture claim, so the capture
                // consumer cannot reach this value at all. Asserting the whole
                // record rather than the direct view is what makes that a
                // measured absence instead of an unexamined field.
                context_capture: None,
            },
            "the direct-emission claim must be keyed to the exact emission occurrence, owner and \
             lexical environment and carry the nearest-alias index, and a predeclared emitter must \
             project no capture claim at all"
        );
        assert_ne!(
            seat_index, introduction_index,
            "this fixture must exercise a genuine shift; equal indices would make every \
             assertion above satisfiable by the identity"
        );

        // ⛔ Fail-closed 1 of 5 — wrong emission origin. A seat that is not a
        // construct origin of this result edge has no defined environment.
        let off_edge = {
            let lawful = continuation_result_origins(&plan, result_origin)
                .expect("the result edge resolves");
            plan.occurrence_authorities
                .iter()
                .map(|authority| authority.origin)
                .find(|origin| !lawful.contains(origin))
                .expect("the fixture has an occurrence off this result edge")
        };
        let wrong_seat = ContinuationProducerEnvironment {
            producer_construct_origin: off_edge,
            ..environment.clone()
        };
        let refusal = exact_continuation_projection(
            &plan,
            &wrong_seat,
            0,
            &ContinuationEmitterFrame::Predeclared(environment.producer_owner),
        )
        .expect_err("an emission seat off its own result edge must refuse");
        assert!(
            format!("{refusal:?}").contains("not an occurrence of its own producer owner"),
            "the refusal must be the emission-origin one, not an incidental error: {refusal:?}"
        );

        // ⛔ Fail-closed 2 of 5 — wrong nearest-alias index. A binding the walk
        // does not place at the seat gets no index at all.
        let mut absent = effect_source.clone();
        absent.coordinate = ContinuationSourceCoordinate::producer_local_probe();
        let refusal = exact_continuation_projection(
            &plan,
            &ContinuationProducerEnvironment {
                inputs: vec![absent],
                ..environment
            },
            0,
            &ContinuationEmitterFrame::Predeclared(environment.producer_owner),
        )
        .expect_err("a binding absent from the seat environment must refuse");
        assert!(
            format!("{refusal:?}").contains("not present in the lexical environment"),
            "the refusal must name the absent binding rather than fall through: {refusal:?}"
        );
    }

    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D2b` DISCRIMINATOR 2 — generated-context
    /// capture availability, with the root and immediate positions DIFFERENT.**
    ///
    /// A generated context reaches a producer-local value only as one of its own
    /// declared captures, laid out after its parameter run. So the immediate
    /// capture slot is a third number, distinct from both the binding's
    /// introduction index and its position in the capture projection — and a
    /// row where any two of those coincide cannot tell a real lookup from an
    /// index that happened to line up.
    ///
    /// ⭐ The enclosing capture projection is built with a decoy ahead of the
    /// value, and the context declares parameters, so the three numbers here are
    /// `0` (introduction), `1` (capture position) and `3` (immediate slot).
    ///
    /// MEASURED: the arm resolves to the exact context/owner it is keyed to with
    /// `immediate_capture_slot = context_parameters + capture position`; and it
    /// refuses when the caller's proof is absent, when the coordinate is not in
    /// the capture projection, when the owner is crossed with another unit's
    /// captures, and when the slot arithmetic would overflow. CLAIMED: a
    /// producer-local capture exists only where the caller's own current-lexical
    /// availability proves the value was there. THE GAP: `D3` still owns
    /// consuming this at emission; nothing here lowers it.
    ///
    /// **Promise class: durable invariant.**
    #[test]
    fn contsrc_d2b_generated_context_capture_separates_root_from_immediate() {
        let expr = Box::leak(Box::new(contsrc_d2_both_binding_kinds_fixture()));
        let plan =
            plan_static_transition_graph(expr, &BTreeMap::new()).expect("the D2 fixture plans");
        let target = contsrc_d2_first_origin(&plan, |expr| {
            matches!(expr, RuntimeExpr::ComputationalMatch { .. })
        });
        let owner = occurrence_authority(&plan, target)
            .expect("the target has an occurrence authority")
            .owner;
        let reached = contsrc_d2_reached_environment(&plan, target);
        let (effect_source, _, effect_locator) = contsrc_d2_local(&reached[1]);
        let coordinate = effect_source.coordinate;
        let introduction_index = effect_locator.environment_index;

        let (result_origin, construct_origin, seat_index) =
            contsrc_d2b_shifted_emission_seat(&plan, owner, coordinate, introduction_index);
        let source_root = continuation_owner_source_root(&plan, owner).expect("one source root");

        // The CALLER's own projection of this value. ⭐ **The re-cut relocates
        // this row's caller-proof authority rather than dropping it.** Under the
        // retired law the projection inspected this record's availability and
        // refused anything but a current-lexical one. That check is gone, because
        // a nested generated context's member lawfully carries an entry-frame
        // claim, so the old refusal was not a law. What replaces it is stronger
        // and structural: **exact-once membership by whole coordinate in the
        // enclosing specialization's own continuation inputs** — and those inputs
        // were themselves projected and validated when that specialization was
        // interned. A capture cannot be fabricated because there is nothing to
        // find unless the caller really declares the value.
        let caller_proof = ContinuationInputProjection {
            availability: ContinuationAvailabilityDraft {
                direct_emission: Some(ContinuationEnvironmentDraft::CurrentLexical {
                    emission_owner: owner,
                    producer_result_origin: result_origin,
                    emission_origin: construct_origin,
                    lexical_environment_origin: source_root,
                    nearest_alias_index: seat_index,
                }),
                context_capture: None,
            },
            producer_owner: owner,
            consumer_owner: owner,
            coordinate,
            ordinal: 1,
            carrier: effect_source.carrier,
            ownership: effect_source.ownership,
            storage_owner: effect_source.storage_owner,
            referent_affinity: effect_source.referent_affinity.clone(),
            ordinary_abi_position: 7,
        };
        // A decoy ahead of it, so the capture POSITION is not zero and cannot
        // be confused with the introduction index.
        let decoy = ContinuationInputProjection {
            availability: ContinuationAvailabilityDraft {
                direct_emission: Some(ContinuationEnvironmentDraft::EntryFrame {
                    frame: ContinuationFrameRequirement::Predeclared(owner),
                    declared_slot: 0,
                }),
                context_capture: None,
            },
            coordinate: ContinuationSourceCoordinate::EntryAbi {
                source_owner: owner,
                source_abi_position: 0,
                source: ContinuationInputSource::Parameter,
            },
            ordinal: 0,
            ..caller_proof.clone()
        };
        let enclosing_inputs = vec![decoy, caller_proof.clone()];
        const CONTEXT_PARAMETERS: u32 = 2;
        let context = ContinuationSpecializationId(0);
        // ⛔ Two DISTINCT worker bodies. The frame identity is a pair, and a row
        // that only ever supplies one body origin cannot tell a recorded identity
        // from a defaulted one.
        let body_origin = target;
        let other_body_origin = plan
            .occurrence_authorities
            .iter()
            .map(|authority| authority.origin)
            .find(|origin| *origin != body_origin)
            .expect("the fixture has a second static origin");
        let environment = ContinuationProducerEnvironment {
            producer_owner: owner,
            producer_result_origin: result_origin,
            producer_construct_origin: construct_origin,
            consumer_owner: owner,
            inputs: vec![effect_source.clone()],
        };
        fn resolution<'plan>(
            context: ContinuationSpecializationId,
            inputs: &'plan [ContinuationInputProjection],
            worker_body_origin: StaticOriginId,
            parameters: u32,
        ) -> ContinuationEmitterFrame<'plan> {
            ContinuationEmitterFrame::GeneratedContext {
                enclosing: context,
                worker_body_origin,
                context_parameters: parameters,
                enclosing_inputs: inputs,
            }
        }

        let projected = exact_continuation_projection(
            &plan,
            &environment,
            0,
            &resolution(context, &enclosing_inputs, body_origin, CONTEXT_PARAMETERS),
        )
        .expect("a captured producer-local value with a caller proof must project");
        let declared = ContinuationEnvironmentDraft::EntryFrame {
            frame: ContinuationFrameRequirement::GeneratedContext {
                enclosing: context,
                worker_body_origin: body_origin,
            },
            declared_slot: CONTEXT_PARAMETERS + 1,
        };
        assert_eq!(
            projected[0].availability,
            // ⭐ BOTH views carry the same claim here, and only here. A generated
            // context's direct emission and its capture append read one frame --
            // its own operand run -- so the two consumers agree by identity of
            // environment rather than by convention. ⛔ Asserting the whole record
            // is what makes that agreement measured; asserting one view would
            // leave the other unexamined.
            ContinuationAvailabilityDraft {
                direct_emission: Some(declared),
                context_capture: Some(declared),
            },
        );
        // The three numbers are pairwise distinct, which is what makes the
        // lookup answerable rather than a coincidence of index.
        assert_ne!(CONTEXT_PARAMETERS + 1, introduction_index);
        assert_ne!(CONTEXT_PARAMETERS + 1, 1, "the capture position is not the slot");

        // ⛔ Fail-closed 3 of 5 — missing full-coordinate capture membership.
        let refusal = exact_continuation_projection(
            &plan,
            &environment,
            0,
            &resolution(context, &enclosing_inputs[..1], body_origin, CONTEXT_PARAMETERS),
        )
        .expect_err("a value absent from the capture projection must refuse");
        assert!(
            format!("{refusal:?}").contains("not among the"),
            "the refusal must be the membership one: {refusal:?}"
        );

        // ⛔ 4 of 5 — the FRAME IDENTITY is recorded, not defaulted.
        //
        // ⭐ **This is the re-cut of the retired "crossed owner/context" refusal,
        // and the relocation is the point.** That refusal fired in the planner,
        // which was the wrong plane for it: the planner is handed the emitting
        // frame and has no second frame to cross it against. What it can be held
        // to is FAITHFULNESS — that the claim names the frame it was actually
        // given. The refusal itself now lives at the consumer, in
        // `verify_entry_frame`, which is the only place both the claimed frame
        // and the held frame exist. A row asserting a defaulted identity would
        // make that consumer check unreachable in principle.
        let crossed = exact_continuation_projection(
            &plan,
            &environment,
            0,
            &resolution(context, &enclosing_inputs, other_body_origin, CONTEXT_PARAMETERS),
        )
        .expect("a different worker body is still a well-formed emitting frame")[0]
            .availability;
        assert_eq!(
            crossed,
            ContinuationAvailabilityDraft {
                direct_emission: Some(ContinuationEnvironmentDraft::EntryFrame {
                    frame: ContinuationFrameRequirement::GeneratedContext {
                        enclosing: context,
                        worker_body_origin: other_body_origin,
                    },
                    declared_slot: CONTEXT_PARAMETERS + 1,
                }),
                context_capture: Some(ContinuationEnvironmentDraft::EntryFrame {
                    frame: ContinuationFrameRequirement::GeneratedContext {
                        enclosing: context,
                        worker_body_origin: other_body_origin,
                    },
                    declared_slot: CONTEXT_PARAMETERS + 1,
                }),
            },
            "the claim must name the worker body it was handed; a frame identity that ignores it \
             would make two different frames indistinguishable to the consumer that has to refuse \
             one of them"
        );
        assert_ne!(
            crossed, projected[0].availability,
            "two distinct emitting frames must not project the same claim, or the identity \
             carries no information"
        );

        // ⛔ 5 of 5 — the slot arithmetic refuses rather than wraps. The capture
        // run starts after the parameter run, and `u32::MAX` parameters leaves no
        // representable slot for a capture at position 1.
        let refusal = exact_continuation_projection(
            &plan,
            &environment,
            0,
            &resolution(context, &enclosing_inputs, body_origin, u32::MAX),
        )
        .expect_err("an immediate slot past the representable range must refuse");
        assert!(
            format!("{refusal:?}").contains("immediate slot position exhausted"),
            "the refusal must be the slot-arithmetic one: {refusal:?}"
        );

        // ⛔ 6 — membership is EXACTLY ONCE. A duplicated member makes the
        // declared slot ambiguous, and taking the first would silently pick one
        // of two positions holding the same coordinate.
        let refusal = exact_continuation_projection(
            &plan,
            &environment,
            0,
            &resolution(
                context,
                &[
                    enclosing_inputs[0].clone(),
                    caller_proof.clone(),
                    caller_proof,
                ],
                body_origin,
                CONTEXT_PARAMETERS,
            ),
        )
        .expect_err("a duplicated capture member must refuse rather than take the first");
        assert!(
            format!("{refusal:?}").contains("two members for one"),
            "the refusal must be the ambiguity one: {refusal:?}"
        );
    }

    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D4a` — the binding is ADMITTED, and the
    /// declined set is refused by the authority that was always there.**
    ///
    /// ⭐ **This row is the `D2` sentinel, fired and restated — not deleted.**
    /// Its predecessor asserted the exact opposite: that no interned
    /// specialization names a producer-local coordinate. That was `D2`'s law
    /// and its promise class named `D4` as the event that retires it. `D4a` is
    /// that event, so the assertion is **inverted rather than dropped**, which
    /// keeps the transition itself measured instead of leaving a gap where a
    /// law used to be.
    ///
    /// MEASURED: the fixture's environment holds producer-local bindings, and
    /// at least one interned specialization now names a producer-local
    /// coordinate, against a nonzero interned-input total. CLAIMED: the `D2`
    /// filter is gone and the domain reaches interning. THE GAP: this row does
    /// **not** measure `R`'s decline — this fixture is fully closed, so nothing
    /// in it reaches either decline clause, and the corpus-level
    /// `interned = V` / `declined = R` partition is `D4b`'s. It also says
    /// nothing about lowering, which still refuses both local availability
    /// arms; that is `D3b`'s, against the emissions this checkpoint creates.
    ///
    /// **Promise class: durable invariant** — the admission law itself. It goes
    /// red if a future change re-filters the domain or weakens either decline
    /// clause.
    #[test]
    fn contsrc_d4a_a_producer_local_environment_is_admitted_and_r_still_declines() {
        let expr = Box::leak(Box::new(contsrc_d2_both_binding_kinds_fixture()));
        let plan = plan_static_transition_graph(expr, &BTreeMap::new())
            .expect("a fixture whose environment names producer-local bindings still PLANS");
        let target = contsrc_d2_first_origin(&plan, |expr| {
            matches!(expr, RuntimeExpr::ComputationalMatch { .. })
        });
        let reached = contsrc_d2_reached_environment(&plan, target);
        assert!(
            reached.iter().any(|value| matches!(
                value,
                ContinuationValueSourceAuthority::Closed(sources)
                    if sources.iter().any(|source| matches!(
                        source.coordinate,
                        ContinuationSourceCoordinate::ProducerLocal { .. }
                    ))
            )),
            "the fixture must actually reach the gate, or this row measures nothing: {reached:?}"
        );

        // ADMISSION, the inverted law. ⛔ The count is asserted alongside, so an
        // empty interned population cannot satisfy this vacuously — which is
        // the shape that would have let a still-filtering gate pass.
        let mut interned_inputs = 0usize;
        let mut interned_local = 0usize;
        for plan in [&plan, &contspec_plan()] {
            for unit in &plan.continuation_specializations {
                for input in &unit.key.continuation_inputs {
                    interned_inputs += 1;
                    if matches!(
                        input.coordinate,
                        ContinuationSourceCoordinate::ProducerLocal { .. }
                    ) {
                        interned_local += 1;
                    }
                }
            }
        }
        assert!(
            interned_inputs > 0,
            "no specialization interned any input at all, so this row holds vacuously"
        );
        assert!(
            interned_local > 0,
            "D4a admits the producer-local domain, so at least one interned specialization must \
             name one; zero means the gate is still filtering and the deletion did not take"
        );

        // ⛔ `R`'s decline is deliberately NOT asserted here, and the reason is
        // that it cannot be asserted honestly from this fixture. This
        // environment is fully closed — that is precisely why it is admitted —
        // so nothing in it reaches either decline clause. A row that
        // constructed an `Open` value locally and matched on it would be a
        // tautology about the enum, not a measurement of the take-loop.
        //
        // What keeps `R` declined is that `D4a` deleted a block *below* the
        // take-loop and changed nothing in it. The corpus-level partition
        // proof, `interned = V` and `declined = R` over all 83 instances, is
        // `D4b`'s deliverable and needs the census harness, not this row.
    }

    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D3a` — the validator RE-DERIVES a
    /// producer-local source instead of refusing its domain.**
    ///
    /// ⭐ The positive control is the deliverable: before `D3a` this call was
    /// an `Err` for every producer-local coordinate, so `Ok(())` here is the
    /// consumer actually being assigned rather than merely stopping.
    ///
    /// ⛔ The three discriminators exist because a lookup that merely *finds*
    /// the coordinate would pass the positive control. Each perturbs one thing
    /// the re-derivation must check and asserts the refusal by its own message.
    ///
    /// MEASURED: a walk-derived producer-local source validates; relocating it
    /// to an index the same environment genuinely holds a **different** value
    /// at is refused; changing the structural binding identity while keeping
    /// the locator is refused; and corrupting a contract field the coordinate
    /// does not name — the referent affinity — is refused. CLAIMED: the arm
    /// re-runs the derivation and compares the whole record, rather than
    /// confirming the coordinate appears somewhere. THE GAP: production still
    /// declines every producer-local candidate at the `D2` gate, so this arm is
    /// reached here by direct construction and not by any fixture compile.
    ///
    /// **Promise class: durable invariant.**
    #[test]
    fn contsrc_d3a_validator_rederives_a_producer_local_source() {
        let expr = Box::leak(Box::new(contsrc_d2_both_binding_kinds_fixture()));
        let plan =
            plan_static_transition_graph(expr, &BTreeMap::new()).expect("the D2 fixture plans");
        let target = contsrc_d2_first_origin(&plan, |expr| {
            matches!(expr, RuntimeExpr::ComputationalMatch { .. })
        });
        let reached = contsrc_d2_reached_environment(&plan, target);
        let (effect_source, _, effect_locator) = contsrc_d2_local(&reached[1]);

        // The positive control, and the whole point of the deliverable.
        validate_continuation_source_slot(&plan, effect_source)
            .expect("D3a re-derives a producer-local source rather than refusing its domain");

        // Discriminator 1 — the locator must name the position that actually
        // holds THIS value.
        //
        // ⭐ The decoy is the fixture's OTHER producer-local binding's own
        // locator, so it is a real occupied position in a real scope that holds
        // a genuinely different value. That is what makes "walk somewhere and
        // accept whatever sits at the index" a FAILING answer here rather than
        // an indistinguishable one — an out-of-range index would only have
        // measured the bounds guard.
        let (binder_source, _, binder_locator) = contsrc_d2_local(&reached[0]);
        assert_ne!(
            (binder_locator.environment_origin, binder_locator.environment_index),
            (effect_locator.environment_origin, effect_locator.environment_index),
            "the two fixture bindings must occupy different positions, or the decoy is the seat"
        );
        let mut relocated = effect_source.clone();
        if let ContinuationSourceCoordinate::ProducerLocal { locator, .. } =
            &mut relocated.coordinate
        {
            *locator = binder_locator;
        }
        // The oracle: that position is occupied, and by something else.
        let decoy_scope = contsrc_d2_reached_environment(&plan, binder_locator.environment_origin);
        assert!(
            matches!(
                decoy_scope.get(binder_locator.environment_index as usize),
                Some(ContinuationValueSourceAuthority::Closed(sources))
                    if sources.contains(binder_source) && !sources.contains(&relocated)
            ),
            "the decoy position must hold the OTHER binding and not the relocated one, or this \
             discriminator cannot fail for the intended reason"
        );
        assert_eq!(
            validate_continuation_source_slot(&plan, &relocated).unwrap_err(),
            planner_error(
                "continuation value disagrees with its exact producer-local source provenance"
            )
        );

        // Discriminator 2 — the structural binding identity is re-derived, not
        // carried. Keeping the locator exact and moving only the ordinal makes
        // the coordinate name a binding the walk never places there.
        let mut crossed_binding = effect_source.clone();
        if let ContinuationSourceCoordinate::ProducerLocal { binding, .. } =
            &mut crossed_binding.coordinate
        {
            binding.binding_ordinal = binding.binding_ordinal.wrapping_add(1);
        }
        assert_eq!(
            validate_continuation_source_slot(&plan, &crossed_binding).unwrap_err(),
            planner_error(
                "continuation value disagrees with its exact producer-local source provenance"
            )
        );

        // Discriminator 3 — the CONTRACT, not just the coordinate. Affinity is
        // re-derived by `producer_local_source` from the binding's lifetime, so
        // a projection arriving with a different one disagrees even though its
        // coordinate is exact.
        //
        // ⛔ The corrupted affinity is deliberately kept NON-EMPTY. Clearing it
        // was the first draft, and it passed against a coordinate-only
        // comparison: the sibling `is_empty()` clause refused it, so the row
        // named the whole-record comparison while measuring the emptiness
        // guard. Measured, not reasoned — the mutation to a coordinate-only
        // match survived until this became a different non-empty value.
        let mut wrong_affinity = effect_source.clone();
        assert!(
            !wrong_affinity.referent_affinity.is_empty(),
            "the fixture source must carry an affinity to corrupt, or this row is vacuous"
        );
        wrong_affinity
            .referent_affinity
            .push(wrong_affinity.referent_affinity[0]);
        assert!(
            !wrong_affinity.referent_affinity.is_empty()
                && wrong_affinity.referent_affinity != effect_source.referent_affinity,
            "the corrupted affinity must be non-empty AND different, or discriminator 3 measures \
             the emptiness guard instead of the whole-record comparison"
        );
        assert_eq!(
            validate_continuation_source_slot(&plan, &wrong_affinity).unwrap_err(),
            planner_error(
                "continuation value disagrees with its exact producer-local source provenance"
            )
        );

        // Discriminator 4 — the locator's index is BOUNDED by the environment
        // that scope actually has, and running off it refuses by its own
        // message rather than folding into the disagreement above.
        //
        // ⚠ This is the one refusal the coordinate comparison cannot subsume.
        // The locator lives *inside* the coordinate, so a wrong index is
        // normally caught as a coordinate disagreement — an index past the end
        // has no position to compare against at all, and reaches a different
        // guard.
        let mut past_end = effect_source.clone();
        if let ContinuationSourceCoordinate::ProducerLocal { locator, .. } =
            &mut past_end.coordinate
        {
            locator.environment_index = u32::MAX;
        }
        assert_eq!(
            validate_continuation_source_slot(&plan, &past_end).unwrap_err(),
            planner_error(
                "a producer-local continuation value names an environment index past the end of \
                 the environment in force at its own locator scope"
            )
        );
    }

    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D3a` — the ABI provenance construction is
    /// DOMAIN-PRESERVING, on the one input where a domain-total helper looks
    /// identical.**
    ///
    /// ⭐ The whole ruling is about *one* case: the same owner id reached
    /// through two different coordinate domains. Any helper — including the
    /// domain-total `provenance_owner()` this replaced — agrees with this one
    /// on every other input, so a row that varied the owner would pass against
    /// the rejected design. This holds the owner FIXED and varies only the
    /// domain.
    ///
    /// MEASURED: two coordinates carrying the identical `PredeclaredFunctionId`
    /// in different domains produce provenance values that are not equal, and
    /// each lands in its own arm. CLAIMED: an entry-ABI authority and a
    /// producer-local authority in the same owner are distinguishable at this
    /// plane. THE GAP: this is the constructor's law only; that the ABI
    /// cross-check actually *uses* it is
    /// `contspec_abi_refuses_owner_lifetime_and_affinity_disagreement`.
    ///
    /// **Promise class: durable invariant.**
    #[test]
    fn contsrc_d3a_abi_provenance_separates_the_domains_at_one_owner() {
        let owner = PredeclaredFunctionId(11);
        let entry = abi::AbiContinuationInputProvenance::of(
            ContinuationSourceCoordinate::EntryAbi {
                source_owner: owner,
                source_abi_position: 0,
                source: ContinuationInputSource::Parameter,
            },
        );
        let local = abi::AbiContinuationInputProvenance::of(
            ContinuationSourceCoordinate::ProducerLocal {
                binding: ProducerLocalBinding {
                    binding_owner: owner,
                    binding_origin: StaticOriginId(4),
                    binding_ordinal: 0,
                },
                locator: ProducerLocalLocator {
                    environment_origin: StaticOriginId(4),
                    environment_index: 0,
                },
            },
        );
        assert_eq!(
            entry,
            abi::AbiContinuationInputProvenance::EntryAbi {
                source_owner: owner
            }
        );
        assert_eq!(
            local,
            abi::AbiContinuationInputProvenance::ProducerLocal {
                binding_owner: owner
            }
        );
        // ⛔ The kill: a domain-total owner projection makes these equal.
        assert_ne!(
            entry, local,
            "the same owner reached through two domains must not collapse to one provenance"
        );
    }

    /// AC-2 omission matrix. Each row is compile-valid and produces one named
    /// wrong answer: if that field is omitted, two distinct units conflate.
    #[test]
    fn contspec_each_projection_field_prevents_one_compile_valid_collision() {
        let plan = contspec_plan();
        let base_key = plan.continuation_specializations[0].key.clone();
        let fields = [
            ContinuationProjectionOmission::ProducerOwner,
            ContinuationProjectionOmission::ConsumerOwner,
            ContinuationProjectionOmission::SourceOwner,
            ContinuationProjectionOmission::SourceAbiPosition,
            ContinuationProjectionOmission::Source,
            ContinuationProjectionOmission::Ordinal,
            ContinuationProjectionOmission::Carrier,
            ContinuationProjectionOmission::Ownership,
            ContinuationProjectionOmission::StorageOwner,
            ContinuationProjectionOmission::ReferentAffinity,
            ContinuationProjectionOmission::OrdinaryAbiPosition,
        ];
        for field in fields {
            let mut distinct = base_key.clone();
            mutate_projection_field(&mut distinct.continuation_inputs[0], field);
            assert_ne!(
                base_key, distinct,
                "AC-2 {field:?}: the exact key selected the wrong existing unit"
            );

            let mut interned = BTreeMap::new();
            let mut units = Vec::new();
            let (left, _) = intern_specialization(
                &mut interned,
                &mut units,
                base_key.clone(),
            )
            .unwrap();
            let (right, _) =
                intern_specialization(&mut interned, &mut units, distinct).unwrap();
            assert_ne!(
                left, right,
                "AC-2 {field:?}: two units differing only in this field conflated"
            );

            let mut mutated_interned = BTreeMap::new();
            let mut mutated_units = Vec::new();
            CONTINUATION_INTERN_MUTATION.with(|mutation| {
                mutation.set(ContinuationInternMutation::OmitProjection(field))
            });
            let (wrong_left, _) = intern_specialization(
                &mut mutated_interned,
                &mut mutated_units,
                base_key.clone(),
            )
            .unwrap();
            let (wrong_right, _) = intern_specialization(
                &mut mutated_interned,
                &mut mutated_units,
                {
                    let mut key = base_key.clone();
                    mutate_projection_field(&mut key.continuation_inputs[0], field);
                    key
                },
            )
            .unwrap();
            CONTINUATION_INTERN_MUTATION
                .with(|mutation| mutation.set(ContinuationInternMutation::Exact));
            assert_eq!(
                wrong_left, wrong_right,
                "AC-2 {field:?}: omission did not produce the named wrong-unit conflation"
            );
        }
    }

    /// AC-3 prefix collision. The prefix is deliberately equal while the exact
    /// worker/result tail differs; full-key interning must select two targets.
    #[test]
    fn contspec_prefix_only_interning_would_select_the_wrong_target() {
        let plan = contspec_plan();
        let left = plan.continuation_specializations[0].key.clone();
        let mut right = left.clone();
        right.worker.body_origin = StaticOriginId(u32::MAX);
        let prefix = |key: &ContinuationSpecializationKey| {
            (
                key.producer_owner,
                key.consumer_owner,
                key.continuation_origin,
            )
        };
        assert_eq!(prefix(&left), prefix(&right), "fixture has no prefix collision");
        assert_ne!(left, right, "fixture has no exact-key discriminator");
        let mut interned = BTreeMap::new();
        let mut units = Vec::new();
        let (left_id, _) =
            intern_specialization(&mut interned, &mut units, left.clone()).unwrap();
        let (right_id, _) =
            intern_specialization(&mut interned, &mut units, right.clone()).unwrap();
        assert_ne!(
            left_id, right_id,
            "AC-3: prefix-only equality conflated two exact worker targets"
        );

        let mut wrong_interned = BTreeMap::new();
        let mut wrong_units = Vec::new();
        CONTINUATION_INTERN_MUTATION
            .with(|mutation| mutation.set(ContinuationInternMutation::PrefixOnly));
        let (wrong_left, _) =
            intern_specialization(&mut wrong_interned, &mut wrong_units, left).unwrap();
        let (wrong_right, _) =
            intern_specialization(&mut wrong_interned, &mut wrong_units, right).unwrap();
        CONTINUATION_INTERN_MUTATION
            .with(|mutation| mutation.set(ContinuationInternMutation::Exact));
        assert_eq!(
            wrong_left, wrong_right,
            "AC-3: prefix mutation did not select the named wrong existing target"
        );
    }

    /// AC-4 assigned-key mutation. The mutation is compile-valid and changes
    /// the edge's selected alternative; exact re-derivation must reject it.
    #[test]
    fn contspec_assigned_key_mutation_plans_an_edge_to_the_wrong_alternative() {
        let mut plan = contspec_plan();
        let assigned = &mut plan.continuation_specializations[0].key;
        assigned.producer_alternative = assigned.producer_alternative.max(1);
        assert_eq!(
            plan.validate().unwrap_err(),
            planner_error("continuation specialization plan is not the exact closed derivation")
        );
    }

    /// AC-5. The normal nested inner-to-outer population terminates with two
    /// keys. Returning the active item to the frontier without first interning
    /// it destroys the finite unseen-key measure and reaches the exact bound.
    #[test]
    fn contspec_nested_fixed_point_requires_interning_before_discovery() {
        let normal = contspec_plan();
        assert_eq!(normal.continuation_specializations.len(), 2);

        let expr = contspec_nested_fixture();
        WEAKEN_CONTINUATION_DECREASING_MEASURE.with(|mutation| mutation.set(true));
        let weakened = plan_static_transition_graph(&expr, &BTreeMap::new());
        WEAKEN_CONTINUATION_DECREASING_MEASURE.with(|mutation| mutation.set(false));
        let error = match weakened {
            Ok(_) => panic!("AC-5: weakened measure unexpectedly terminated"),
            Err(error) => error,
        };
        assert_eq!(
            error,
            planner_error("continuation specialization fixed point did not terminate")
        );
    }

    /// D5 planned-edge closure is independently load-bearing: losing a call or
    /// redirecting its target cannot be hidden by a still-complete unit vector.
    #[test]
    fn contspec_planned_edge_closure_rejects_omission_and_redirection() {
        let plan = contspec_plan();
        let mut omitted = plan.clone();
        omitted.continuation_specialization_calls.pop();
        assert_eq!(
            omitted.validate().unwrap_err(),
            planner_error("continuation specialization plan is not the exact closed derivation")
        );

        let mut redirected = plan;
        let current = redirected.continuation_specialization_calls[0]
            .token
            .target
            .0;
        redirected.continuation_specialization_calls[0].token.target =
            ContinuationSpecializationId(1 - current);
        assert_eq!(
            validate_continuation_specialization_closure(
                &redirected
                    .continuation_specializations
                    .iter()
                    .map(|unit| (unit.key.clone(), unit.id))
                    .collect(),
                &redirected.continuation_specializations,
                &redirected.continuation_specialization_calls,
            )
            .unwrap_err(),
            planner_error("continuation edge token disagrees with its exact target")
        );
    }
}
