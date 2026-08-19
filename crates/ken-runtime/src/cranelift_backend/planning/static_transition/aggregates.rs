//! Aggregate allocation events, ownership records, and the planner-side
//! aggregate lifecycle -- the synthesized-tree recipe, the lifetime-meet
//! derivation, and the closed ownership population.
//!
//! `RT-PLANNER-AGGREGATES-SPLIT` `D1` -- this module owns the aggregates
//! domain moved from the parent (`AggregateOccurrenceId`,
//! `AggregateOccurrenceProducer`, the shape/role/path/step/node vocabulary,
//! and `PlannedAggregateOwnership` + its read-only view). `StaticTransitionPlan`
//! stays in the parent; the impl block here reads ancestor-private root state
//! under the standing child-module pattern (item 4's `units.rs` precedent).
//!
//! The lowering-owned half (`AggregateAllocationEvent`,
//! `AggregateAllocationLedger`, `AggregateRelationClosure`) is a DIFFERENT
//! thing entirely and stays in `lowering/mod.rs` for item 15 -- see the D0
//! ledger's boundary proposal in
//! `docs/program/issues/RT-PLANNER-AGGREGATES-SPLIT.md`.

use std::collections::{BTreeMap, BTreeSet};

use super::{
    occurrence_authority, planner_capacity_error, planner_error,
    synthesized_seat_emission_owners, BoundaryReferentOwner,
    ContinuationEmissionOwner, CraneliftBackendError, FieldIdentity, JoinResultRepresentation,
    PlannedOccurrenceChildAuthority, PlannedReferentLifetime, PredeclaredFunctionId,
    StaticOriginId, StaticTransitionPlan, SynthesizedConstructorRole,
    SynthesizedFixedConstructorRole,
};
use super::closure::{derive_case_producer_fact, CaseProducerSet};
use crate::boundary_value::{BoundaryClass, BoundaryTag};
use crate::RuntimeExpr;

/// **`RT-DECL-CLOSURE-PORT` `D7` — the opaque identity of one aggregate
/// emission occurrence.**
///
/// Issued by the planner and only by the planner. The field is private to this
/// module, so lowering **cannot construct one** — it can only receive an
/// identity from an accessor that already interned it. That is the mechanical
/// form of "lowering does not construct identities"; a doc comment saying so
/// would be advisory, and this is not.
///
/// ## Why an identity rather than the emission origin
///
/// A `Lowered::Constructor` template outlives the occurrence that produced it.
/// Lowering builds the template at the `Construct` occurrence and may transfer
/// it into the carrier much later, at a `Let`, `Match`, `Call` or `Effect`
/// origin reached through nested producer traversal. The identity the emitter
/// needs is the **producer's**, and by then the emission origin is a different
/// occurrence entirely.
///
/// That is not a hypothesis: the sibling `synthesized_identity` field on the
/// template exists for exactly this reason and says so in its own comment —
/// *"the caller occurrence is not the constructor occurrence and therefore
/// cannot lawfully re-query its atom."* The allocation lane is the second fact
/// with that property, and it travels the same way.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub(in crate::cranelift_backend) struct AggregateOccurrenceId(
    pub(in crate::cranelift_backend::planning::static_transition) u32,
);
/// Which producer an aggregate occurrence record is about.
///
/// The two arms are the two ways an aggregate comes to exist, and they are
/// named by different authorities on purpose. A source aggregate is named by
/// its own occurrence in the program. A synthesized one has no occurrence to be
/// named by, so it is named by the closed compiler role that builds it — never
/// by the origin it happens to be emitted at, which belongs to whatever
/// expression the emission was reached through.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum AggregateOccurrenceProducer {
    /// A `Construct`/`Record` written in the program.
    Source(StaticOriginId),
    /// One exact compiler-synthesized producer USE.
    ///
    /// A role is a schema, not an identity, and neither is a role at a seat.
    /// Two uses of `PrivateTransferCount` at two seats are two occurrences even
    /// though their schema is one; two uses of `ResourceKind` at ONE seat --
    /// `ResourceKindMismatch` fields 0 and 1 -- are likewise two occurrences,
    /// and no seat-and-role key can separate them. The path is what does.
    SynthesizedUse {
        /// The exact `D5a` emission owner, `Predeclared` or `Specialization`.
        ///
        /// It is in the KEY, not merely on the record. One seat's body may be
        /// lowered under a predeclared unit and again inside a generated
        /// specialization context; those are different emissions and their
        /// records must not alias. Deriving this from the seat's provenance
        /// owner would collapse exactly the distinction `D5a` exists to keep.
        owner: ContinuationEmissionOwner,
        /// The source occurrence that anchors this synthesized use. Host-result
        /// trees use their `Effect`; a unit-boundary environment uses the exact
        /// source constructor that owns the closure-valued field.
        seat: StaticOriginId,
        /// Where in the seat's synthesized tree this use sits.
        ///
        /// ⛔ Not an ordinal. An ordinal would count emissions in lowering's
        /// control flow, which the planner does not execute; a path is measured
        /// structure that both sides state independently and can be checked
        /// against each other at construction.
        path: SynthesizedAggregatePath,
        /// The closed compiler role that builds this use.
        role: SynthesizedAggregateRole,
    },
}
/// The closed compiler role that builds one synthesized aggregate.
///
/// Constructor roles retain the semantic plane's existing constructor
/// identity. The environment arm names the record introduced when a
/// closure-valued source-constructor field is carried as a generated-unit call
/// input; it has no constructor identity because its shape is
/// [`PlannedAggregateShape::Record`].
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum SynthesizedAggregateRole {
    /// The full semantic constructor-role sum, fixed and `IOError`. Every
    /// `IOError` alternative is a real allocation with its own path, so this
    /// cannot be narrowed to the fixed half.
    Constructor(SynthesizedConstructorRole),
    /// The captured-environment Record introduced at a generated-unit input.
    UnitBoundaryEnvironment,
}
/// Which aggregate shape one producer occurrence builds.
///
/// ⛔ Deliberately its own two-member enum rather than a reuse of
/// [`crate::boundary_value::BoundaryClass`]. That type is the *node* class and
/// admits five ground shapes; the population here is exactly the shapes that
/// **have children to take a lifetime meet over**, and spelling it as its own
/// type is what makes a `Bytes` occurrence a type error here instead of a
/// record nothing ever consumes.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum PlannedAggregateShape {
    Constructor,
    Record,
}
/// A read-only view of one planned aggregate ownership record.
///
/// ⭐ Every accessor answers from the ONE record the occurrence names. Nothing
/// here searches, and nothing takes a coordinate — a consumer that could pass a
/// coordinate could pass the wrong one, which is the defect this projection
/// exists to make unspellable.
pub(in crate::cranelift_backend) struct PlannedAggregateView<'plan> {
    record: &'plan PlannedAggregateOwnership,
}
impl<'plan> PlannedAggregateView<'plan> {
    pub(in crate::cranelift_backend) fn id(&self) -> AggregateOccurrenceId {
        self.record.id
    }

    pub(in crate::cranelift_backend) fn producer(&self) -> &'plan AggregateOccurrenceProducer {
        &self.record.producer
    }

    /// The producer's own source occurrence, for a source aggregate.
    ///
    /// `None` for a compiler-synthesized use, which has no occurrence in the
    /// program — an absence, not a coordinate to fall back on.
    pub(in crate::cranelift_backend) fn producer_origin(&self) -> Option<StaticOriginId> {
        match &self.record.producer {
            AggregateOccurrenceProducer::Source(origin) => Some(*origin),
            AggregateOccurrenceProducer::SynthesizedUse { .. } => None,
        }
    }

    pub(in crate::cranelift_backend) fn owner(&self) -> Option<PredeclaredFunctionId> {
        self.record.owner
    }

    pub(in crate::cranelift_backend) fn shape(&self) -> PlannedAggregateShape {
        self.record.shape
    }

    /// The ruled allocation lane. ⛔ Read here, never re-derived from the value.
    pub(in crate::cranelift_backend) fn allocation(&self) -> PlannedAggregateAllocation {
        self.record.allocation
    }

    pub(in crate::cranelift_backend) fn meet(&self) -> PlannedReferentLifetime {
        self.record.meet
    }

    /// The ordered children, with each one's position, source occurrence,
    /// lifetime and possible referent owners.
    pub(in crate::cranelift_backend) fn children(&self) -> &'plan [PlannedAggregateChild] {
        &self.record.children
    }
}
/// The allocation lane the ruled lifetime meet selects for one aggregate.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum PlannedAggregateAllocation {
    /// Every child's possible-owner set excludes the invocation arena.
    PersistentGround,
    /// At least one child has an invocation-owned alternative, so the
    /// aggregate's own lifetime is the invocation.
    InvocationAggregate,
}
/// One child of an aggregate producer, with the exact facts the meet is taken
/// over.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct PlannedAggregateChild {
    pub(in crate::cranelift_backend) position: u32,
    /// The child's own source occurrence.
    ///
    /// `None` for a child of a compiler-synthesized aggregate, which has no
    /// occurrence in the program. Recorded as an absence rather than filled
    /// with the parent's origin -- the aliasing that made a synthesized
    /// subtree indistinguishable from the expression it was emitted under.
    pub(in crate::cranelift_backend) origin: Option<StaticOriginId>,
    /// **`RT-DECL-CLOSURE-PORT` `D7` — the ordered field identity a RECORD
    /// producer plans at this position.**
    ///
    /// ⭐ Issued here, at the producer, and read nowhere else. A record's field
    /// names are a producer fact in exactly the way its ownership record is, so
    /// they travel with the template rather than being re-resolved at whatever
    /// coordinate the record is finally transferred at.
    ///
    /// ⛔ `None` for a constructor child and for a synthesized child — an
    /// absence, never a name to fall back on. A consumer comparing a record's
    /// carried identity against `None` must refuse rather than skip: two
    /// absences agreeing is the shape that let a grafted schema pass.
    pub(in crate::cranelift_backend) field_identity: Option<FieldIdentity>,
    pub(in crate::cranelift_backend) lifetime: PlannedReferentLifetime,
    /// The **possible** referent owners of this child, never a determination.
    ///
    /// ⚠ Read the emptiness rule before the membership rule: a child whose set
    /// is empty is not a child that owns nothing, it is a child whose
    /// representation the planner could not derive, and the builder refuses it
    /// rather than letting an empty set satisfy "contains no invocation owner"
    /// vacuously.
    pub(in crate::cranelift_backend) owners: Vec<BoundaryReferentOwner>,
}
/// **`RT-DECL-CLOSURE-PORT` `D7` — one exact ownership record per aggregate
/// producer occurrence.**
///
/// ⭐ **The lifetime of an aggregate is a MEET over its children, and no
/// per-value shape can compute it.** `Construct` and `Record` are persistable
/// shapes, so the value-shape disposition reaches for a persistent lane for
/// every one of them. That is right exactly when every child outlives the
/// parent, and it is the dangling edge otherwise — which is why the tag may not
/// be chosen at the allocation site from the value in hand.
///
/// ⛔ The consumer reads `allocation` and nothing else. It may not re-derive the
/// meet, inspect a runtime tag, or search lifetimes in lowering.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct PlannedAggregateOwnership {
    /// The opaque identity lowering carries on the template and hands back at
    /// emission. Dense, and equal to this record's index in the population.
    pub(in crate::cranelift_backend) id: AggregateOccurrenceId,
    /// Which producer this record is about, and the population's sort key.
    pub(in crate::cranelift_backend) producer: AggregateOccurrenceProducer,
    /// The function unit that emits a source producer.
    ///
    /// `None` for a synthesized role, which has no source occurrence and
    /// therefore no function owner. Spelled as an absence rather than a
    /// borrowed owner so nothing can read a synthesized record as if it were
    /// owned by whichever unit happened to emit it.
    pub(in crate::cranelift_backend) owner: Option<PredeclaredFunctionId>,
    pub(in crate::cranelift_backend) shape: PlannedAggregateShape,
    /// The tree's own child model for a synthesized use, handed to the emitter
    /// so it can check that each operand it holds is the KIND the meet was
    /// taken over.
    ///
    /// `None` for a source producer, whose children are occurrences in the
    /// program rather than nodes in a tree.
    pub(in crate::cranelift_backend) declared_children:
        Option<&'static [SynthesizedAggregateNode]>,
    pub(in crate::cranelift_backend) children: Vec<PlannedAggregateChild>,
    /// The meet itself, retained beside the lane it selects so a reader can
    /// see the derivation rather than only its verdict.
    pub(in crate::cranelift_backend) meet: PlannedReferentLifetime,
    pub(in crate::cranelift_backend) allocation: PlannedAggregateAllocation,
}
/// The **possible** referent owners of one aggregate child.
///
/// Two authorities bound this set and the answer is their intersection:
///
/// - **Lifetime** ([`lifetime_referent_affinity`]) — how long the referent may
///   live. `ActivationOwned` admits the invocation arena; `Persistent` does not.
/// - **Representation** ([`JoinResultRepresentation`]) — whether there is a
///   referent to own at all. A child the emitter materializes as a
///   `NativeScalarPair` is an immediate: it has no heap node, so no owner but
///   [`BoundaryReferentOwner::NoReferent`] is possible for it.
///
/// Reading only the first is what makes every call-shaped child look
/// arena-owned. `derive_occurrence_lifetime` answers `ActivationOwned` for
/// every `Call`, `Effect` and `PrimitiveCall` unconditionally — not because
/// their results are arena-owned, but because it does not look through them.
/// That is a sound floor on the LIFETIME axis and says nothing about the
/// REFERENT axis, and treating it as if it did forces an aggregate over two
/// integer-returning calls into the invocation lane. Such an aggregate is then
/// refused at the process root, which cannot accept an arena-owned answer — so
/// the over-approximation does not merely cost a lane, it rejects a program
/// that is sound and that ran before the lane existed.
///
/// This is a narrowing of "possible", not a relaxation of the escape rule. A
/// child with no referent cannot dangle, so it cannot be the reason a parent
/// must die with the invocation. Where the representation is unknown the
/// lifetime answer stands unnarrowed, which is the conservative direction.
pub(in crate::cranelift_backend::planning::static_transition) fn aggregate_child_referent_owners(
    plan: &StaticTransitionPlan<'_>,
    child: &PlannedOccurrenceChildAuthority,
) -> Result<Vec<BoundaryReferentOwner>, CraneliftBackendError> {
    let by_lifetime = lifetime_referent_affinity(child.lifetime);
    let representation = plan
        .join_results
        .get(child.origin.0 as usize)
        .and_then(|slot| slot.as_ref())
        .map(|result| result.representation);
    match representation {
        // The emitter will produce a native scalar pair here. There is no
        // boundary node, so there is nothing for an arena or a store to own.
        Some(JoinResultRepresentation::NativeScalarPair) => {
            Ok(vec![BoundaryReferentOwner::NoReferent])
        }
        // A carrier word may name a node, and an occurrence with no planned
        // join result tells us nothing. Both keep the lifetime's own answer.
        Some(JoinResultRepresentation::CarrierWord) | None => Ok(by_lifetime),
    }
}
/// Which compiler-built tree one synthesized aggregate path is rooted at.
///
/// A host operation synthesizes two independent values — the `error` arm and
/// the `ok` arm — and they are separate trees, not two halves of one. Rooting a
/// path at one of them is what keeps `FsWriteAt`'s `PrivateTransferCount`
/// (which lives under `ok`, inside `Wrote`) distinct from `FsReadAt`'s
/// error-side machinery, without either arm having to know the other's shape.
/// The unit-boundary root instead names the environment record at one exact
/// source-constructor field.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum SynthesizedAggregateRoot {
    HostResultError,
    HostResultOk,
    /// The environment record nested at one field of a source constructor
    /// whose closed result reaches a generated-unit call input.
    UnitBoundaryEnvironment,
}
/// One step from a synthesized aggregate to one of its ordered children.
///
/// ⛔ The two constructors are deliberately different steps rather than one
/// integer. A fixed constructor's field 0 and a dynamic constructor's
/// alternative 0 are positions in different structures — one is a child that is
/// always present, the other is a child that exists only when the discriminator
/// selects it. Collapsing them to a bare index would let a path name a node it
/// does not reach and still compare equal to the one that does.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend::planning::static_transition) enum SynthesizedAggregateStep {
    /// The ordered field of a fixed constructor, by position.
    Field(u32),
    /// One alternative of a dynamic constructor, by its ordered position in the
    /// closed alternative list.
    ///
    /// ⚠ The POSITION, not the ABI discriminator tag. `ResourceKind`'s two tags
    /// are wire-schema facts (`wire.resource_kind_fs_handle`), so a path keyed
    /// on them would depend on a value this planner does not own and could not
    /// state without importing the host's wire layout. The position is the same
    /// fact on both sides, and the emitter's own alternative list is checked
    /// against it at construction.
    Alternative(u32),
}
/// The exact position of one synthesized aggregate in its compiler-built tree.
///
/// ⭐ **This is the fact a role alone cannot supply.** Six of the measured
/// construction sites build a repeated role at one seat: `ResourceKind` appears
/// under `ResourceReleaseFailed` field 0 and `ResourceKindMismatch` fields 0
/// and 1, and the `IOError` alternative set appears under `ResourceHostIo`
/// field 0, `ResourceReleaseFailed` field 2 and `FileError` field 2. A
/// role-keyed record cannot tell those apart, so one row would have to serve
/// three allocations.
///
/// ⛔ The separator is **where the node sits**, never an issued ordinal. An
/// ordinal would have to count emissions in lowering's control flow, which the
/// planner does not execute and therefore cannot compute; the path is measured
/// structure and both sides can state it independently.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct SynthesizedAggregatePath {
    pub(in crate::cranelift_backend::planning::static_transition) root: SynthesizedAggregateRoot,
    pub(in crate::cranelift_backend::planning::static_transition)
        steps: Vec<SynthesizedAggregateStep>,
}
impl SynthesizedAggregatePath {
    /// The empty path at one of a host result's two arms.
    pub(in crate::cranelift_backend) fn root(root: SynthesizedAggregateRoot) -> Self {
        Self {
            root,
            steps: Vec::new(),
        }
    }

    /// This path extended by one ordered field of a fixed constructor.
    pub(in crate::cranelift_backend) fn field(&self, position: u32) -> Self {
        self.extend(SynthesizedAggregateStep::Field(position))
    }

    /// This path extended by one alternative of a dynamic constructor.
    pub(in crate::cranelift_backend) fn alternative(&self, position: u32) -> Self {
        self.extend(SynthesizedAggregateStep::Alternative(position))
    }

    fn extend(&self, step: SynthesizedAggregateStep) -> Self {
        let mut steps = self.steps.clone();
        steps.push(step);
        Self {
            root: self.root,
            steps,
        }
    }
}
/// One node of a host operation's closed synthesized aggregate tree.
///
/// ⭐ **The tree is the recipe.** The previous spelling was a flat per-role
/// child list plus a flat per-operation use list, and the two together could
/// not state *where* a use sits — which is exactly the fact that separates the
/// six repeated-role sites above.
///
/// ## Acyclicity is a compile-time property here, not a runtime colouring
///
/// The children are `&'static` slices built from `const` items, and a `const`
/// that transitively references itself is an evaluation cycle rustc rejects. So
/// the walk over this tree terminates because the tree is finite, and there is
/// no back-edge check to get wrong. The previous role-graph spelling needed a
/// visiting/done colouring precisely because a role could name itself; a value
/// tree cannot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum SynthesizedAggregateNode {
    /// A fixed constructor with ordered children, built through
    /// `Lowered::Constructor`.
    Fixed {
        role: SynthesizedFixedConstructorRole,
        children: &'static [SynthesizedAggregateNode],
    },
    /// A dynamic constructor: exactly one alternative is selected at runtime.
    Dynamic(SynthesizedDynamicSet),
    /// A scalar, named by the **exact** closed immediate disposition the
    /// emitter must produce for it.
    ///
    /// `spill` is the whole reason this is not one class. A
    /// `RepresentedImmediate` does NOT mean "no boundary node": when a runtime
    /// magnitude test finds the payload too wide for the immediate field, a
    /// `spill: Some(_)` value becomes a handle of that class, which is a
    /// persistent-store referent. So
    ///
    /// - `spill: None` (`Bool`) is exactly `{NoReferent}`;
    /// - `spill: Some(_)` (`Int`, `BoundedNat`, `StructuralNat`,
    ///   `ProcessExitStatus`) is `{NoReferent, PersistentStore}`.
    ///
    /// These are the disposition authority's own two fields, not a second tag
    /// table. Recording only the broad `RepresentedImmediate` family, or
    /// widening every scalar to the larger set, is a safe LANE answer and a
    /// false statement about the child -- and a record's owner sets are its
    /// stated evidence, not merely a means to a verdict.
    Scalar {
        tag: BoundaryTag,
        spill: Option<BoundaryClass>,
    },
    /// A value the **Effect seat itself** supplies, named by the ordered
    /// position of the operand it comes from.
    ///
    /// ⭐ **Site-dependence is not non-reachability.** `OptionSome` wraps the
    /// seat's path operand and `PrivateBufferSpan` carries the seat's buffer
    /// `ResourceToken`; both are real allocations that production emits. What
    /// is unavailable for them is a *role-invariant* meet — not a meet. The
    /// evidence is exact and it is already in the plan: the operand is a child
    /// occurrence of this very seat, with its own lifetime and join
    /// representation.
    ///
    /// ⛔ So this is resolved against the seat, never defaulted and never
    /// pruned. Omitting the node from `P` because no role-invariant answer
    /// exists is not the fail-closed direction once production can emit the
    /// allocation — it leaves a real allocation with no record. If the seat's
    /// operand evidence cannot be derived, **planning fails**.
    ///
    /// The index is into the seat's `args`, before the capability offset that
    /// `RuntimeExpr::Effect` applies to its semantic children.
    SiteOperand(u32),
    /// This arm of the host result synthesizes no aggregate at all.
    ///
    /// `FsReadFile`'s `ResponseBytes`, `FsOpen`'s `ResourceToken`,
    /// `FsHandleMetadata`'s `Int`. Distinct from [`Self::SiteOperand`] on
    /// purpose: that one is a child whose evidence the seat supplies, this one
    /// is a position where the tree governs nothing because no aggregate is
    /// built. Collapsing them would let "no allocation here" and "an allocation
    /// whose child comes from the site" share an arm.
    Absent,
}
/// The closed alternative set of one dynamic constructor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum SynthesizedDynamicSet {
    /// The `IOError` alternative set, whose population is the process symbol
    /// inventory rather than a list this module can spell.
    ///
    /// Named rather than enumerated because its arity is
    /// `NativeProcessSymbols::io_errors.len()` and its roles are minted by
    /// [`StaticTransitionPlan::synthesized_io_error_roles`]. Its alternatives
    /// carry at most one `Int` payload, on the last alternative only.
    IoErrors,
    /// An alternative list this module states, indexed by position.
    Alternatives(&'static [SynthesizedAggregateNode]),
}
impl SynthesizedAggregateNode {
    /// A `BoundedNat` scalar, as the reply-validation lowering produces it.
    pub(in crate::cranelift_backend::planning::static_transition) const fn bounded_nat() -> Self {
        Self::Scalar {
            tag: BoundaryTag::ImmediateBoundedNat,
            spill: Some(BoundaryClass::Int),
        }
    }

    /// A native `Int` scalar.
    pub(in crate::cranelift_backend::planning::static_transition) const fn native_int() -> Self {
        Self::Scalar {
            tag: BoundaryTag::ImmediateInt,
            spill: Some(BoundaryClass::Int),
        }
    }

    /// A fixed constructor with no children.
    pub(in crate::cranelift_backend::planning::static_transition) const fn nullary(role: SynthesizedFixedConstructorRole) -> Self {
        Self::Fixed {
            role,
            children: &[],
        }
    }
}
/// **`RT-DECL-CLOSURE-PORT` `D7` — the closed synthesized aggregate tree of one
/// host operation, both arms.**
///
/// ## It was MEASURED, not transcribed
///
/// This structure lives in `lower_process_host_effect` as roughly four hundred
/// lines of imperative code interleaved with `builder.ins()` calls. It was
/// derived by instrumenting every `synthesized_constructor` and
/// `synthesized_dynamic_alternative` call to print the KINDS of its own
/// children, running the suite **single-threaded**, and reading the edges off
/// the log.
///
/// The single-threaded run is not incidental: `--nocapture` output from
/// concurrent tests interleaves, and a parallel run manufactured a phantom in
/// which one seat built its error tree twice. That phantom would have forced a
/// planner-issued repetition ordinal into the key — an ordinal the planner
/// cannot compute, because it depends on lowering's control flow.
///
/// ## Two things the measurement found that a transcription would have smoothed
///
/// - `OptionSome -> [Bytes]` and `PrivateBufferSpan -> [ResourceToken, …]` are
///   genuine **site-dependent leaves**. A `ResourceToken` is a handle, not a
///   scalar, so neither can take a role-invariant child model.
/// - `Wrote -> [Bool]` occurs exactly once in the whole suite, in the `c2_ac4`
///   fixture, and disagrees with every production construction of `Wrote`. It
///   is recorded here as the fixture's disagreement, not the tree's.
///
/// ## The eager `IOError` template is ABANDONED for the resource-surface ops
///
/// `lower_process_host_effect` builds one `IOError` dynamic constructor before
/// it knows which branch it is in. The file operations use it as `FileError`
/// field 2 and the console operations use it as the whole error; the six
/// resource-surface operations build their **own** `surface_io_error` and never
/// reference it. So the trees below do not contain it at those roots — an
/// abandoned template is not a semantic use, and giving it a path would plan a
/// record for an allocation that never happens.
pub(in crate::cranelift_backend::planning::static_transition) fn host_effect_recipe_tree(operation: ken_host::HostOpV1) -> SynthesizedHostResultTree {
    use ken_host::HostOpV1 as Op;
    use SynthesizedAggregateNode as N;
    use SynthesizedFixedConstructorRole as R;

    const NAT2: &[SynthesizedAggregateNode] = &[N::bounded_nat(), N::bounded_nat()];
    const INT2: &[SynthesizedAggregateNode] = &[N::native_int(), N::native_int()];

    /// `PrivateTransferCount(BoundedNat, BoundedNat)`.
    const TRANSFER_COUNT: SynthesizedAggregateNode = N::Fixed {
        role: R::PrivateTransferCount,
        children: NAT2,
    };
    /// `ResourceTraceIdentity(Int, Int)`.
    const TRACE_IDENTITY: SynthesizedAggregateNode = N::Fixed {
        role: R::ResourceTraceIdentity,
        children: INT2,
    };
    /// The two-alternative `ResourceKind` set, at wire tags this module does
    /// not spell — reached by POSITION, per [`SynthesizedAggregateStep`].
    const RESOURCE_KIND: SynthesizedAggregateNode =
        N::Dynamic(SynthesizedDynamicSet::Alternatives(&[
            N::nullary(R::ResourceKindFsHandle),
            N::nullary(R::ResourceKindBuffer),
        ]));
    const IO_ERRORS: SynthesizedAggregateNode = N::Dynamic(SynthesizedDynamicSet::IoErrors);

    /// The eleven-alternative resource surface, in the emitter's own order.
    const RESOURCE_SURFACE: SynthesizedAggregateNode =
        N::Dynamic(SynthesizedDynamicSet::Alternatives(&[
            N::Fixed {
                role: R::ResourceHostIo,
                children: &[IO_ERRORS],
            },
            N::nullary(R::ResourceClosed),
            N::nullary(R::ResourceMalformed),
            N::Fixed {
                role: R::ResourceRightNotHeld,
                children: INT2,
            },
            N::Fixed {
                role: R::ResourceReleaseFailed,
                children: &[RESOURCE_KIND, TRACE_IDENTITY, IO_ERRORS],
            },
            N::Fixed {
                role: R::ResourceKindMismatch,
                children: &[RESOURCE_KIND, RESOURCE_KIND],
            },
            N::nullary(R::ResourceBufferLimit),
            N::nullary(R::ResourceAllocationFailed),
            N::nullary(R::ResourceInvalidOffset),
            N::nullary(R::ResourceInvalidBounds),
            N::nullary(R::ResourceNoProgress),
        ]));

    /// `Option::Some(<the site's path operand>)`.
    ///
    /// Its child is site-bound, which bounds the ROLE-INVARIANT meet and not
    /// the meet: this node and every parent of it gets an exact seat-bound
    /// record, derived from the seat's own operand authority.
    const SOME_SITE_PATH: SynthesizedAggregateNode = N::Fixed {
        role: R::OptionSome,
        // The seat's operand 0 — the path the caller passed.
        children: &[N::SiteOperand(0)],
    };
    /// `FileError(FileOperation*, Option::Some(<site path>), IOError)`.
    const READ_FILE_ERROR_CHILDREN: &[SynthesizedAggregateNode] = &[
        N::nullary(R::FileOperationRead),
        SOME_SITE_PATH,
        IO_ERRORS,
    ];
    const WRITE_FILE_ERROR_CHILDREN: &[SynthesizedAggregateNode] = &[
        N::nullary(R::FileOperationWrite),
        SOME_SITE_PATH,
        IO_ERRORS,
    ];
    const CHANGE_MODE_ERROR_CHILDREN: &[SynthesizedAggregateNode] = &[
        N::nullary(R::FileOperationChangeMode),
        SOME_SITE_PATH,
        IO_ERRORS,
    ];
    const READ_FILE_ERROR: SynthesizedAggregateNode = N::Fixed {
        role: R::FileError,
        children: READ_FILE_ERROR_CHILDREN,
    };
    const WRITE_FILE_ERROR: SynthesizedAggregateNode = N::Fixed {
        role: R::FileError,
        children: WRITE_FILE_ERROR_CHILDREN,
    };
    const CHANGE_MODE_ERROR: SynthesizedAggregateNode = N::Fixed {
        role: R::FileError,
        children: CHANGE_MODE_ERROR_CHILDREN,
    };

    /// The `FsReadAt` success value: `ReadEof` or `ReadSome(span, transferred)`.
    const READ_PROGRESS: SynthesizedAggregateNode =
        N::Dynamic(SynthesizedDynamicSet::Alternatives(&[
            N::nullary(R::ReadEof),
            N::Fixed {
                role: R::ReadSome,
                children: &[
                    // `PrivateBufferSpan(ResourceToken, Int, BoundedNat)` — the
                    // token is the site's buffer operand, so this whole node is
                    // site-dependent.
                    N::Fixed {
                        role: R::PrivateBufferSpan,
                        // The seat's operand 2 — the buffer `ResourceToken`
                        // this span is bound to (`PX8-SPAN-PROV`).
                        children: &[N::SiteOperand(2), N::native_int(), N::bounded_nat()],
                    },
                    TRANSFER_COUNT,
                ],
            },
        ]));
    const WROTE: SynthesizedAggregateNode = N::Fixed {
        role: R::Wrote,
        children: &[TRANSFER_COUNT],
    };
    const UNIT: SynthesizedAggregateNode = N::nullary(R::Unit);

    let (error, ok) = match operation {
        // Returns a `Bool` before any synthesized producer runs, so neither arm
        // exists. Not a gap: the early return is above the synthesis entirely.
        Op::ConsoleIsTerminal => (N::Absent, N::Absent),
        Op::ConsoleWrite | Op::ConsoleFlush => (IO_ERRORS, UNIT),
        Op::FsReadFile => (READ_FILE_ERROR, N::Absent),
        Op::FsOpen => (READ_FILE_ERROR, N::Absent),
        // ⚠ The `ok` arm here is the emitter's `else` branch, which is `Unit`.
        // The flat use table this tree replaces derived these two rows from the
        // operation match and MISSED that branch, so it planned no `Unit`
        // record for them. No fixture exercises either operation, which is why
        // the omission was invisible; the tree states both arms from the same
        // match the emitter uses, so an arm cannot be dropped by inattention.
        Op::FsWriteFile => (WRITE_FILE_ERROR, UNIT),
        Op::FsChangeMode => (CHANGE_MODE_ERROR, UNIT),
        Op::BufferAllocate | Op::BufferFreeze => (RESOURCE_SURFACE, N::Absent),
        Op::FsHandleMetadata => (RESOURCE_SURFACE, N::Absent),
        Op::ResourceRelease => (RESOURCE_SURFACE, UNIT),
        Op::FsReadAt => (RESOURCE_SURFACE, READ_PROGRESS),
        Op::FsWriteAt => (RESOURCE_SURFACE, WROTE),
        // Not an admitted consumer; `lower_process_host_effect` refuses it
        // before any synthesized producer runs.
        _ => (N::Absent, N::Absent),
    };
    SynthesizedHostResultTree { error, ok }
}
/// The two synthesized aggregate trees of one host operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend::planning::static_transition) struct SynthesizedHostResultTree {
    pub(in crate::cranelift_backend::planning::static_transition) error: SynthesizedAggregateNode,
    pub(in crate::cranelift_backend::planning::static_transition) ok: SynthesizedAggregateNode,
}
impl SynthesizedHostResultTree {
    pub(in crate::cranelift_backend::planning::static_transition) fn node(&self, root: SynthesizedAggregateRoot) -> SynthesizedAggregateNode {
        match root {
            SynthesizedAggregateRoot::HostResultError => self.error,
            SynthesizedAggregateRoot::HostResultOk => self.ok,
            // Environment records are derived from source call-input results,
            // not from a host operation's synthesized tree. Returning the
            // absent node keeps the host-tree resolver fail-closed if the two
            // domains are ever accidentally mixed.
            SynthesizedAggregateRoot::UnitBoundaryEnvironment => {
                SynthesizedAggregateNode::Absent
            }
        }
    }
}
/// Collect the site-bound operand ordinals named anywhere under one
/// compiler-synthesized result node.
///
/// This walks the same closed recipe that plans aggregate children. It is not
/// a second operation table: adding or removing a `SiteOperand` in the recipe
/// changes both the planned child relation and this population together.
pub(in crate::cranelift_backend::planning::static_transition) fn collect_site_operand_ordinals(node: SynthesizedAggregateNode, ordinals: &mut BTreeSet<u32>) {
    match node {
        SynthesizedAggregateNode::Fixed { children, .. } => {
            for child in children {
                collect_site_operand_ordinals(*child, ordinals);
            }
        }
        SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::Alternatives(alternatives)) => {
            for alternative in alternatives {
                collect_site_operand_ordinals(*alternative, ordinals);
            }
        }
        // The closed IOError alternatives contain only scalar payloads. They
        // cannot introduce a site operand behind this dynamic node.
        SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::IoErrors)
        | SynthesizedAggregateNode::Scalar { .. }
        | SynthesizedAggregateNode::Absent => {}
        SynthesizedAggregateNode::SiteOperand(index) => {
            ordinals.insert(index);
        }
    }
}
/// What a path walk arrived at.
///
/// The `IOError` alternatives are not nodes in the static tree — they are
/// minted from the process symbol inventory — so a walk that reaches one
/// reports its position rather than a node it cannot produce.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SynthesizedTreeResolution {
    Node(SynthesizedAggregateNode),
    IoErrorAlternative(u32),
}
/// One semantic use the flattening found: a node, and where it sits.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend::planning::static_transition) struct FlattenedSynthesizedUse {
    pub(in crate::cranelift_backend::planning::static_transition) path: SynthesizedAggregatePath,
    pub(in crate::cranelift_backend::planning::static_transition) role: SynthesizedConstructorRole,
    pub(in crate::cranelift_backend::planning::static_transition)
        children: &'static [SynthesizedAggregateNode],
}
/// Flatten one operation's trees to every **allocation-reachable** use.
///
/// ⭐ **A dynamic SET is not an allocation; every selected ALTERNATIVE is.**
/// That is the boundary, ruled by the Architect after an earlier spelling drew
/// it at "a `Fixed` node that is not itself a dynamic alternative". That
/// predicate was wrong in a specific way worth recording: it was about *which
/// authority supplies the lane today* (`emit_carrier_dynamic_constructor` read
/// the value-shape disposition) rather than about *what allocates*. An
/// alternative calls `emit_carrier_alloc` exactly as a fixed constructor does —
/// dynamic allocations are measured live — so reconciling one against the tree
/// without interning an occurrence proves only that lowering named the expected
/// schema. It supplies no lifetime record, cannot enter `R`, and cannot satisfy
/// "every event has exactly one record".
///
/// The traversal is therefore total over constructor-valued nodes:
///
/// ```text
/// Fixed(role, children):
///   intern (owner, seat, path, Fixed(role)) with its ordered child model
///   recurse into every aggregate-valued child at path.Field(position)
///
/// Dynamic(Alternatives(alts)):
///   visit each alternative at path.Alternative(index)
///
/// Dynamic(IoErrors):
///   intern each planner-issued role at path.Alternative(index),
///   keyed IoError(role), with that role's exact children
/// ```
///
/// ⛔ Nothing is pruned for want of a role-invariant meet. A node whose child
/// comes from the emission site gets an exact **site-bound** record derived
/// from the seat's own operand authority, or planning fails.
pub(in crate::cranelift_backend::planning::static_transition) fn flatten_allocation_reachable_uses(
    plan: &StaticTransitionPlan<'_>,
    operation: ken_host::HostOpV1,
) -> Vec<FlattenedSynthesizedUse> {
    let tree = host_effect_recipe_tree(operation);
    let io_errors = plan.semantic.synthesized_io_error_roles().len();
    let mut uses = Vec::new();
    for root in [
        SynthesizedAggregateRoot::HostResultError,
        SynthesizedAggregateRoot::HostResultOk,
    ] {
        collect_reachable_uses(
            tree.node(root),
            &SynthesizedAggregatePath::root(root),
            io_errors,
            plan,
            &mut uses,
        );
    }
    uses
}
/// The ordered children of one `IOError` alternative.
///
/// The set is nullary but for its **last** alternative, which carries the
/// decoded payload as a native `Int`. That is the emitter's own shape in
/// `synthesized_io_error_alternatives`, restated here so the record's child
/// model is the one the allocation actually has.
fn io_error_alternative_children(
    index: usize,
    count: usize,
) -> &'static [SynthesizedAggregateNode] {
    const PAYLOAD: &[SynthesizedAggregateNode] = &[SynthesizedAggregateNode::native_int()];
    if count > 0 && index + 1 == count {
        PAYLOAD
    } else {
        &[]
    }
}
/// The flattening's walk. Terminates because the tree is a finite `&'static`
/// value; see [`SynthesizedAggregateNode`] on why it cannot be cyclic.
fn collect_reachable_uses(
    node: SynthesizedAggregateNode,
    path: &SynthesizedAggregatePath,
    io_errors: usize,
    plan: &StaticTransitionPlan<'_>,
    uses: &mut Vec<FlattenedSynthesizedUse>,
) {
    match node {
        SynthesizedAggregateNode::Fixed { role, children } => {
            uses.push(FlattenedSynthesizedUse {
                path: path.clone(),
                role: SynthesizedConstructorRole::Fixed(role),
                children,
            });
            for (position, child) in children.iter().enumerate() {
                collect_reachable_uses(
                    *child,
                    &path.field(position as u32),
                    io_errors,
                    plan,
                    uses,
                );
            }
        }
        // The set is not an allocation. Each alternative is, and each is
        // visited at its own ordered position -- which is what separates the
        // three `ResourceKind` uses and the repeated `IOError` sets.
        SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::Alternatives(alternatives)) => {
            for (position, alternative) in alternatives.iter().enumerate() {
                collect_reachable_uses(
                    *alternative,
                    &path.alternative(position as u32),
                    io_errors,
                    plan,
                    uses,
                );
            }
        }
        // The closed `IOError` inventory supplies both the alternative token
        // and the role. The ABI tag remains non-authoritative: the path step is
        // the ordered position, as everywhere else.
        SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::IoErrors) => {
            for (position, role) in plan
                .semantic
                .synthesized_io_error_roles()
                .iter()
                .enumerate()
            {
                uses.push(FlattenedSynthesizedUse {
                    path: path.alternative(position as u32),
                    role: SynthesizedConstructorRole::IoError(*role),
                    children: io_error_alternative_children(position, io_errors),
                });
            }
        }
        SynthesizedAggregateNode::Scalar { .. }
        | SynthesizedAggregateNode::SiteOperand(_)
        | SynthesizedAggregateNode::Absent => {}
    }
}
/// The possible referent owners of one tree node, at one exact effect seat.
///
/// ⚠ Exactness matters here independently of the verdict. These sets are the
/// record's stated evidence, and a set that is merely *sufficient* to reach the
/// right lane is still a false statement about what the child can be.
///
/// ⛔ **There is no "not derivable" answer.** A node whose child comes from the
/// emission site is resolved against that site — the operand is a child
/// occurrence of this very seat, and its lifetime and join representation are
/// already planned. If that evidence cannot be read, planning FAILS. Returning
/// an absence here is what previously pruned four real constructors out of `P`,
/// leaving allocations production emits with no record at all.
pub(in crate::cranelift_backend::planning::static_transition) fn node_referent_owners(
    plan: &StaticTransitionPlan<'_>,
    seat: StaticOriginId,
    node: SynthesizedAggregateNode,
) -> Result<Vec<BoundaryReferentOwner>, CraneliftBackendError> {
    match node {
        // A scalar is NOT `NoReferent` alone. `Int`, `BoundedNat`,
        // `StructuralNat` and `ProcessExitStatus` each have a declared
        // persistent SPILL arm, so a scalar child may be a persistent-store
        // referent. It can never be arena-owned, which is why the lane verdict
        // is the same either way -- and why recording `{NoReferent}` was a
        // false statement that happened not to cost anything yet.
        SynthesizedAggregateNode::Scalar { spill: None, .. } => {
            Ok(vec![BoundaryReferentOwner::NoReferent])
        }
        SynthesizedAggregateNode::Scalar { spill: Some(_), .. } => Ok(vec![
            BoundaryReferentOwner::NoReferent,
            BoundaryReferentOwner::PersistentStore,
        ]),
        // A nested fixed constructor IS a referent, and its owner is
        // determined -- it is the lane its own children select. It is never
        // `NoReferent`, and listing alternatives it cannot take would describe
        // a different node than the one being allocated.
        SynthesizedAggregateNode::Fixed { children, .. } => {
            Ok(vec![fixed_node_selected_owner(plan, seat, children)?])
        }
        // ⭐ A dynamic child is the UNION of its alternatives' selected owners.
        //
        // ⛔ Not `None`, and not a flat `{PersistentStore}` read off the lane
        // `emit_carrier_dynamic_constructor` happens to take. The value at this
        // position is whichever alternative the discriminator selects, so the
        // parent must survive every one of them: a single invocation-capable
        // alternative makes the parent invocation-owned. Answering persistent
        // because the *set* is shaped persistently would allocate the parent
        // over a child that can be shorter-lived than it.
        SynthesizedAggregateNode::Dynamic(set) => {
            let mut owners = BTreeSet::new();
            for alternative in dynamic_alternative_nodes(plan, set) {
                owners.insert(fixed_node_selected_owner_of(plan, seat, alternative)?);
            }
            if owners.is_empty() {
                return Err(planner_error(
                    "a dynamic aggregate child has no alternatives, so its owner set is empty \
                     and would satisfy the escape test vacuously",
                ));
            }
            Ok(owners.into_iter().collect())
        }
        // The seat's own operand. Its evidence is the child occurrence's
        // lifetime narrowed by its join representation -- the same two
        // authorities a source aggregate's children are read through, applied
        // to the exact operand this node names.
        SynthesizedAggregateNode::SiteOperand(index) => {
            site_operand_referent_owners(plan, seat, index)
        }
        // ⛔ Never a child. `Absent` marks a host-result arm that builds no
        // aggregate; reaching it as a child means the tree claims an allocation
        // has a child at a position where nothing is built.
        SynthesizedAggregateNode::Absent => Err(planner_error(
            "a synthesized aggregate child is marked absent, so the tree describes an \
             allocation whose operand is not built",
        )),
    }
}
/// The possible owners of the operand one [`SynthesizedAggregateNode::SiteOperand`]
/// names, read from the seat's own child occurrence.
///
/// The index is into the Effect's `args`; its semantic child position is offset
/// by the capability operand, exactly as `lower_process_host_effect` offsets it
/// when it lowers the same operand.
fn site_operand_referent_owners(
    plan: &StaticTransitionPlan<'_>,
    seat: StaticOriginId,
    index: u32,
) -> Result<Vec<BoundaryReferentOwner>, CraneliftBackendError> {
    let occurrence = plan
        .source_occurrences
        .get(seat.0 as usize)
        .and_then(|slot| slot.as_ref())
        .ok_or_else(|| planner_error("synthesized aggregate seat is not an occurrence"))?;
    let RuntimeExpr::Effect { capability, .. } = occurrence.expr else {
        return Err(planner_error(
            "a site-bound synthesized aggregate child names a seat that is not a host effect",
        ));
    };
    let position = usize::from(capability.is_some())
        .checked_add(index as usize)
        .ok_or_else(|| planner_capacity_error("site operand position overflows"))?;
    let authority = occurrence_authority(plan, seat)?;
    let child = authority.children.get(position).ok_or_else(|| {
        planner_error(
            "a site-bound synthesized aggregate child names an operand the seat does not have",
        )
    })?;
    let owners = aggregate_child_referent_owners(plan, child)?;
    if owners.is_empty() {
        return Err(planner_error(
            "a site-bound synthesized aggregate child has no derivable referent owner",
        ));
    }
    Ok(owners)
}
/// The alternative nodes of a dynamic set, as owner derivation must see them.
fn dynamic_alternative_nodes(
    plan: &StaticTransitionPlan<'_>,
    set: SynthesizedDynamicSet,
) -> Vec<SynthesizedAggregateNode> {
    match set {
        SynthesizedDynamicSet::Alternatives(alternatives) => alternatives.to_vec(),
        // Every `IOError` alternative is nullary but the last, which carries an
        // `Int`. Both shapes are enumerated rather than collapsed to the widest,
        // so the union is over the alternatives that actually exist.
        SynthesizedDynamicSet::IoErrors => {
            let count = plan.semantic.synthesized_io_error_roles().len();
            (0..count)
                .map(|index| SynthesizedAggregateNode::Fixed {
                    // The role names an alternative for shape purposes only;
                    // owner derivation never keys on it, only on the children.
                    role: SynthesizedFixedConstructorRole::ResourceHostIo,
                    children: io_error_alternative_children(index, count),
                })
                .collect()
        }
    }
}
/// The exact owner one fixed node's allocation takes, given its children.
///
/// `Wrote` is persistent **because** `PrivateTransferCount` is, which is
/// persistent because neither of its scalar children can be arena-owned. The
/// chain is computed rather than asserted per role, so a verdict cannot
/// disagree with the tree it is supposed to follow.
pub(in crate::cranelift_backend::planning::static_transition) fn fixed_node_selected_owner(
    plan: &StaticTransitionPlan<'_>,
    seat: StaticOriginId,
    children: &'static [SynthesizedAggregateNode],
) -> Result<BoundaryReferentOwner, CraneliftBackendError> {
    let mut escapes = false;
    for child in children {
        if node_referent_owners(plan, seat, *child)?
            .contains(&BoundaryReferentOwner::InvocationArena)
        {
            escapes = true;
        }
    }
    Ok(if escapes {
        BoundaryReferentOwner::InvocationArena
    } else {
        BoundaryReferentOwner::PersistentStore
    })
}
/// [`fixed_node_selected_owner`] for a node rather than a child list.
fn fixed_node_selected_owner_of(
    plan: &StaticTransitionPlan<'_>,
    seat: StaticOriginId,
    node: SynthesizedAggregateNode,
) -> Result<BoundaryReferentOwner, CraneliftBackendError> {
    match node {
        SynthesizedAggregateNode::Fixed { children, .. } => {
            fixed_node_selected_owner(plan, seat, children)
        }
        // A dynamic set nested directly inside a dynamic set is not a shape the
        // measured tree has; it would be an alternative that is itself a
        // choice, with no constructor to allocate.
        other => {
            let _ = other;
            Err(planner_error(
                "a dynamic aggregate alternative is not a constructor, so it allocates nothing",
            ))
        }
    }
}
/// Source-constructor fields whose empty lexical environment is carried into a
/// generated-unit call.
///
/// The key is derived from source structure on both sides: the direct lexical
/// callee fixes the generated-unit boundary, the call argument fixes the result
/// root, and the closed producer analysis fixes each concrete constructor field.
/// No lowering-order ordinal participates.
fn unit_boundary_environment_fields(
    plan: &StaticTransitionPlan<'_>,
) -> Result<BTreeSet<(StaticOriginId, u32)>, CraneliftBackendError> {
    let mut fields = BTreeSet::new();
    for occurrence in plan.source_occurrences.iter().flatten() {
        let RuntimeExpr::Call { args, .. } = occurrence.expr else {
            continue;
        };
        let callee = plan
            .semantic
            .child_origin(occurrence.static_origin, 0)?;
        if !matches!(
            plan.planned_occurrence_expr(callee)?,
            RuntimeExpr::LexicalClosure { .. }
        ) {
            continue;
        }
        for argument_position in 0..args.len() {
            let argument = plan
                .semantic
                .child_origin(occurrence.static_origin, 1 + argument_position)?;
            let mut match_scrutinees = BTreeMap::new();
            let fact = derive_case_producer_fact(
                plan,
                argument,
                &[],
                &mut match_scrutinees,
            )?;
            let CaseProducerSet::Closed(_) = fact.producers else {
                continue;
            };
            for (_, origins) in fact.producer_origins {
                for producer in origins {
                    let RuntimeExpr::Construct { args, .. } =
                        plan.planned_occurrence_expr(producer)?
                    else {
                        return Err(planner_error(
                            "closed constructor-result authority names a \
                             non-Construct producer",
                        ));
                    };
                    for (position, field) in args.iter().enumerate() {
                        if matches!(
                            field,
                            RuntimeExpr::LexicalClosure { captures, .. }
                                if captures.is_empty()
                        ) {
                            fields.insert((
                                producer,
                                u32::try_from(position).map_err(|_| {
                                    planner_capacity_error(
                                        "unit-boundary environment field exceeds the \
                                         position space",
                                    )
                                })?,
                            ));
                        }
                    }
                }
            }
        }
    }
    Ok(fields)
}
/// Derive one ownership record for every aggregate producer occurrence.
///
/// ⛔ **The population is every `Construct`/`Record` source occurrence, not the
/// ones some reached trace visited.** A lane chosen from the branch this
/// execution happened to take is exactly the row-driven discovery the frame
/// forbids.
///
/// The synthesized population below adds records for compiler-built trees; it
/// does not remove any source producer from this population.
pub(in crate::cranelift_backend::planning::static_transition) fn build_aggregate_ownership_plan(
    plan: &StaticTransitionPlan<'_>,
) -> Result<Vec<PlannedAggregateOwnership>, CraneliftBackendError> {
    let mut records = Vec::new();
    for occurrence in plan.source_occurrences.iter().flatten() {
        let shape = match occurrence.expr {
            RuntimeExpr::Construct { .. } => PlannedAggregateShape::Constructor,
            RuntimeExpr::Record { .. } => PlannedAggregateShape::Record,
            _ => continue,
        };
        let origin = occurrence.static_origin;
        let authority = occurrence_authority(plan, origin)?;
        let mut children = Vec::with_capacity(authority.children.len());
        for child in &authority.children {
            let owners = aggregate_child_referent_owners(plan, child)?;
            if owners.is_empty() {
                return Err(planner_error(
                    "aggregate producer child has no derivable referent owner",
                ));
            }
            // ⭐ The RECORD half of the producer schema, issued once beside the
            // ownership record it belongs to. ⛔ Gated on the shape rather than
            // attempted-and-recovered: a `Construct` occurrence has no field
            // names at all, so asking for one and swallowing the failure would
            // make "this producer plans no name here" and "the lookup did not
            // work" the same answer.
            let field_identity = match shape {
                PlannedAggregateShape::Record => Some(
                    plan.record_field_identity(origin, child.position as usize)?,
                ),
                PlannedAggregateShape::Constructor => None,
            };
            children.push(PlannedAggregateChild {
                position: child.position,
                origin: Some(child.origin),
                field_identity,
                lifetime: child.lifetime,
                owners,
            });
        }
        // ⭐ The ruled meet, stated once. "Any invocation-owned ALTERNATIVE"
        // is membership in the possible set, not a proof that the child *is*
        // invocation-owned — an aggregate is only persistable when no child
        // could be shorter-lived than it.
        let escapes = children
            .iter()
            .any(|child| child.owners.contains(&BoundaryReferentOwner::InvocationArena));
        let (meet, allocation) = if escapes {
            (
                PlannedReferentLifetime::ActivationOwned,
                PlannedAggregateAllocation::InvocationAggregate,
            )
        } else {
            (
                PlannedReferentLifetime::Persistent,
                PlannedAggregateAllocation::PersistentGround,
            )
        };
        records.push(PlannedAggregateOwnership {
            // Renumbered below. The identity is the record's index in the
            // sorted population, so it cannot be assigned before the order is
            // final.
            id: AggregateOccurrenceId(0),
            producer: AggregateOccurrenceProducer::Source(origin),
            owner: Some(
                plan.semantic
                    .function_owner(origin)?
                    .ok_or_else(|| planner_error("aggregate producer has no function owner"))?,
            ),
            shape,
            declared_children: None,
            children,
            meet,
            allocation,
        });
    }

    // The synthesized half: ONE record per exact allocation-reachable use in
    // the operation's tree, keyed by WHERE that use sits.
    //
    // The population is (every `Effect` source occurrence) x (its emission
    // owners) x (the allocation-reachable uses its operation's tree flattens
    // to). Two seats using one role get two records, and two uses of one role
    // at one seat -- `ResourceKind` under `ResourceKindMismatch` fields 0 and
    // 1, say -- get two records because their paths differ.
    for occurrence in plan.source_occurrences.iter().flatten() {
        let RuntimeExpr::Effect { operation, .. } = occurrence.expr else {
            continue;
        };
        let seat = occurrence.static_origin;
        for owner in synthesized_seat_emission_owners(plan, seat)? {
            for semantic_use in flatten_allocation_reachable_uses(plan, *operation) {
                let mut children = Vec::with_capacity(semantic_use.children.len());
                for (position, child) in semantic_use.children.iter().enumerate() {
                    // ⛔ No pruning. A child the emission site supplies is
                    // resolved AGAINST that site; a child that cannot be
                    // resolved fails planning. Skipping the use here is what
                    // left four real constructors -- `OptionSome`, `FileError`,
                    // `PrivateBufferSpan`, `ReadSome` -- allocating with no
                    // record.
                    let owners = node_referent_owners(plan, seat, *child)?;
                    children.push(PlannedAggregateChild {
                        position: u32::try_from(position).map_err(|_| {
                            planner_capacity_error(
                                "synthesized aggregate arity exceeds the position space",
                            )
                        })?,
                        // A synthesized child has no source occurrence of its own.
                        origin: None,
                        // Every synthesized aggregate this population reaches is
                        // a constructor node, so there is no field name to plan.
                        field_identity: None,
                        lifetime: if owners.contains(&BoundaryReferentOwner::InvocationArena) {
                            PlannedReferentLifetime::ActivationOwned
                        } else {
                            PlannedReferentLifetime::Persistent
                        },
                        owners,
                    });
                }
                let escapes = children
                    .iter()
                    .any(|child| child.owners.contains(&BoundaryReferentOwner::InvocationArena));
                let (meet, allocation) = if escapes {
                    (
                        PlannedReferentLifetime::ActivationOwned,
                        PlannedAggregateAllocation::InvocationAggregate,
                    )
                } else {
                    (
                        PlannedReferentLifetime::Persistent,
                        PlannedAggregateAllocation::PersistentGround,
                    )
                };
                records.push(PlannedAggregateOwnership {
                    id: AggregateOccurrenceId(0),
                    producer: AggregateOccurrenceProducer::SynthesizedUse {
                        owner,
                        seat,
                        path: semantic_use.path.clone(),
                        role: SynthesizedAggregateRole::Constructor(
                            semantic_use.role,
                        ),
                    },
                    // Provenance only, kept for readers. The emission owner that
                    // confers authority is in the key above.
                    owner: plan.semantic.function_owner(seat)?,
                    shape: PlannedAggregateShape::Constructor,
                    declared_children: Some(semantic_use.children),
                    children,
                    meet,
                    allocation,
                });
            }
        }
    }
    // The unit-boundary environment half. Each record is rooted in one exact
    // source producer and field selected by the closed call-input result
    // analysis above. Empty captures are the bounded first population: the
    // record has no fields, so no compiler-created field-name authority is
    // needed or inferred.
    for (seat, position) in unit_boundary_environment_fields(plan)? {
        for owner in synthesized_seat_emission_owners(plan, seat)? {
            records.push(PlannedAggregateOwnership {
                id: AggregateOccurrenceId(0),
                producer: AggregateOccurrenceProducer::SynthesizedUse {
                    owner,
                    seat,
                    path: SynthesizedAggregatePath::root(
                        SynthesizedAggregateRoot::UnitBoundaryEnvironment,
                    )
                    .field(position),
                    role: SynthesizedAggregateRole::UnitBoundaryEnvironment,
                },
                owner: plan.semantic.function_owner(seat)?,
                shape: PlannedAggregateShape::Record,
                declared_children: Some(&[]),
                children: Vec::new(),
                meet: PlannedReferentLifetime::Persistent,
                allocation: PlannedAggregateAllocation::PersistentGround,
            });
        }
    }
    records.sort_by(|left, right| left.producer.cmp(&right.producer));
    for (index, record) in records.iter_mut().enumerate() {
        record.id = AggregateOccurrenceId(u32::try_from(index).map_err(|_| {
            planner_capacity_error("the aggregate occurrence population exceeds the identity space")
        })?);
    }
    Ok(records)
}
/// Every record names a DISTINCT producer.
///
/// This is the non-aliasing law of the occurrence domain, and it is production
/// code rather than a test because it is what makes an identity an identity: if
/// two records shared a producer, one seat's record could authorize another
/// seat's allocation and the lane chosen for one node would govern a different
/// one. Two uses of a role at two seats must be two occurrences; two records for
/// ONE use is the same failure seen from the other side.
pub(in crate::cranelift_backend::planning::static_transition) fn validate_aggregate_producers_are_unique(
    records: &[PlannedAggregateOwnership],
) -> Result<(), CraneliftBackendError> {
    let mut seen = BTreeSet::new();
    for record in records {
        if !seen.insert(record.producer.clone()) {
            return Err(planner_error(
                "two aggregate ownership records name the same producer, so an occurrence \
                 identity is not unique",
            ));
        }
    }
    Ok(())
}
pub(in crate::cranelift_backend::planning::static_transition) fn validate_aggregate_ownership_plan(
    plan: &StaticTransitionPlan<'_>,
    records: &[PlannedAggregateOwnership],
) -> Result<(), CraneliftBackendError> {
    if records != build_aggregate_ownership_plan(plan)? {
        return Err(planner_error(
            "aggregate ownership is not the exact closed lifetime-meet derivation",
        ));
    }
    validate_aggregate_producers_are_unique(records)?;
    // ⛔ A second, independent check on the same records, because the
    // re-derivation above only proves the builder agrees with itself. This one
    // states the PROPERTY: the persistent lane is issued only where no child
    // has an invocation-owned alternative.
    // The identity is only opaque to its consumers if it is exact here: a
    // record whose id is not its own index would resolve to a *different*
    // record's lane, which is the one failure this domain has to be incapable
    // of. Stated as its own law rather than left to the rebuild comparison
    // above, which would agree with a builder that numbered every record zero.
    for (index, record) in records.iter().enumerate() {
        if record.id.0 as usize != index {
            return Err(planner_error(
                "aggregate occurrence identities are not the dense index of their own population",
            ));
        }
    }
    for record in records {
        let escapes = record
            .children
            .iter()
            .any(|child| child.owners.contains(&BoundaryReferentOwner::InvocationArena));
        let expected = if escapes {
            PlannedAggregateAllocation::InvocationAggregate
        } else {
            PlannedAggregateAllocation::PersistentGround
        };
        if record.allocation != expected {
            return Err(planner_error(
                "aggregate allocation lane disagrees with its own children's owner sets",
            ));
        }
    }
    Ok(())
}
pub(in crate::cranelift_backend::planning::static_transition) fn lifetime_referent_affinity(
    lifetime: PlannedReferentLifetime,
) -> Vec<BoundaryReferentOwner> {
    match lifetime {
        PlannedReferentLifetime::Persistent => vec![
            BoundaryReferentOwner::NoReferent,
            BoundaryReferentOwner::PersistentStore,
        ],
        PlannedReferentLifetime::ActivationOwned => vec![
            BoundaryReferentOwner::NoReferent,
            BoundaryReferentOwner::PersistentStore,
            BoundaryReferentOwner::InvocationArena,
        ],
    }
}

impl<'src> StaticTransitionPlan<'src> {
    /// **`D7` — the ruled allocation lane for one aggregate producer.**
    ///
    /// ⛔ **Absence is a loud failure, never a default.** An aggregate the
    /// planner never issued a record for is one whose lifetime meet was never
    /// taken, and answering `PersistentGround` for it would reinstate exactly
    /// the unproven persistent lane this record exists to replace — silently,
    /// and only for the occurrences the population happened to miss.
    ///
    /// ⚠ The `shape` argument is a cross-check, not a lookup key. The caller
    /// knows which aggregate it is emitting; if that disagrees with the record
    /// at this origin, one of the two is reading the wrong occurrence and the
    /// lane is meaningless either way.
    pub(in crate::cranelift_backend::planning::static_transition) fn aggregate_allocation(
        &self,
        origin: StaticOriginId,
        shape: PlannedAggregateShape,
    ) -> Result<PlannedAggregateAllocation, CraneliftBackendError> {
        let record = self
            .aggregate_ownership
            .iter()
            .find(|record| record.producer == AggregateOccurrenceProducer::Source(origin))
            .ok_or_else(|| {
                planner_error("aggregate producer has no planned ownership record")
            })?;
        if record.shape != shape {
            return Err(planner_error(
                "aggregate producer disagrees with its planned ownership shape",
            ));
        }
        Ok(record.allocation)
    }
    /// The occurrence identity of one **source** aggregate producer.
    ///
    /// This is the only way lowering obtains an identity for a source
    /// `Construct`/`Record`, and it is asked at the producer occurrence — where
    /// the answer is well defined — not at the emission site, where it is not.
    ///
    /// Absence is a loud failure for the same reason the lane's absence is: an
    /// occurrence the planner never interned is one whose meet was never taken.
    pub(in crate::cranelift_backend) fn source_aggregate_occurrence(
        &self,
        origin: StaticOriginId,
        shape: PlannedAggregateShape,
    ) -> Result<AggregateOccurrenceId, CraneliftBackendError> {
        let record = self
            .aggregate_ownership
            .iter()
            .find(|record| record.producer == AggregateOccurrenceProducer::Source(origin))
            .ok_or_else(|| planner_error("aggregate producer has no planned ownership record"))?;
        if record.shape != shape {
            return Err(planner_error(
                "aggregate producer disagrees with its planned ownership shape",
            ));
        }
        Ok(record.id)
    }
    /// The occurrence identity of one **compiler-synthesized** aggregate use.
    ///
    /// ⛔ The key is `owner + seat + path + full role`, never the role alone. A
    /// synthesized aggregate has no occurrence in the program to be keyed by,
    /// and a role repeats within one seat's tree — `ResourceKind` three times —
    /// so the path is what separates the uses.
    ///
    /// Every allocation-reachable use has a record, site-bound ones included.
    /// Absence here is a loud failure, not the ordinary answer for a role whose
    /// children come from the emission site.
    pub(in crate::cranelift_backend) fn synthesized_aggregate_occurrence(
        &self,
        owner: ContinuationEmissionOwner,
        seat: StaticOriginId,
        path: &SynthesizedAggregatePath,
        role: SynthesizedConstructorRole,
    ) -> Result<AggregateOccurrenceId, CraneliftBackendError> {
        self.synthesized_aggregate_record(
            owner,
            seat,
            path,
            SynthesizedAggregateRole::Constructor(role),
        )
        .map(|record| record.id)
    }
    /// The occurrence of the empty environment record nested in one exact
    /// source-constructor field that the closed result analysis routes to a
    /// generated-unit call input.
    ///
    /// Absence is ordinary for every other closure-valued field. The full key
    /// remains owner + producer seat + structural field path + compiler role;
    /// no lowering-order ordinal is accepted by this interface.
    pub(in crate::cranelift_backend) fn unit_boundary_environment_occurrence(
        &self,
        owner: ContinuationEmissionOwner,
        seat: StaticOriginId,
        position: u32,
    ) -> Option<AggregateOccurrenceId> {
        let path = SynthesizedAggregatePath::root(
            SynthesizedAggregateRoot::UnitBoundaryEnvironment,
        )
        .field(position);
        self.aggregate_ownership
            .iter()
            .find(|record| {
                record.producer
                    == AggregateOccurrenceProducer::SynthesizedUse {
                        owner,
                        seat,
                        path: path.clone(),
                        role: SynthesizedAggregateRole::UnitBoundaryEnvironment,
                    }
            })
            .map(|record| record.id)
    }
    /// The record of one synthesized use, found by the full four-part key.
    ///
    /// ⛔ The path is part of the LOOKUP, not a field checked afterwards. A
    /// lookup that matched on owner/seat/role and then verified the path would
    /// find the first of three `ResourceKind` uses and reject the other two;
    /// matching on all four finds each one's own record.
    pub(in crate::cranelift_backend::planning::static_transition) fn synthesized_aggregate_record(
        &self,
        owner: ContinuationEmissionOwner,
        seat: StaticOriginId,
        path: &SynthesizedAggregatePath,
        role: SynthesizedAggregateRole,
    ) -> Result<&PlannedAggregateOwnership, CraneliftBackendError> {
        self.aggregate_ownership
            .iter()
            .find(|record| match &record.producer {
                AggregateOccurrenceProducer::SynthesizedUse {
                    owner: record_owner,
                    seat: record_seat,
                    path: record_path,
                    role: record_role,
                } => {
                    *record_owner == owner
                        && *record_seat == seat
                        && record_path == path
                        && *record_role == role
                }
                AggregateOccurrenceProducer::Source(_) => false,
            })
            .ok_or_else(|| {
                planner_error("synthesized aggregate use has no planned ownership record")
            })
    }
    /// The declared child model of one modelled synthesized role.
    ///
    /// The recipe and the lowering code that builds these aggregates are two
    /// statements of one shape. Handing the emitter the model -- rather than
    /// only its length -- is what lets it check that each operand it actually
    /// holds is the KIND the recipe assumed when it took the meet.
    ///
    /// Arity alone is not sufficient and was not claimed to be: a recipe that
    /// says `Immediate` where the emitter passes a referent-bearing child has
    /// the right count and the wrong lane, and the aggregate is allocated
    /// persistent over an operand that can be arena-owned.
    /// The tree node one path names at one effect seat, whether or not it is
    /// allocation-reachable.
    ///
    /// ⭐ This is what lets a **dynamic alternative** be reconciled against the
    /// tree. An alternative HAS its own path-keyed ownership record and takes
    /// its allocation lane from it, exactly as a fixed constructor does; what
    /// it does not have is a parent's declared child model to be reached
    /// through, because a dynamic set's members are not ordered fields of a
    /// constructor. So its ordered fields are read from the tree here, and an
    /// emitter that put `ResourceTraceIdentity` at `ResourceReleaseFailed`
    /// field 2 instead of field 1 would otherwise pass unchallenged while
    /// carrying the wrong occurrence.
    ///
    /// A path that names no node, or names one that is not a fixed
    /// constructor, is a loud failure: the emitter and the tree disagree about
    /// the shape of the thing being built.
    pub(in crate::cranelift_backend) fn synthesized_tree_node(
        &self,
        seat: StaticOriginId,
        path: &SynthesizedAggregatePath,
    ) -> Result<(SynthesizedConstructorRole, &'static [SynthesizedAggregateNode]),
        CraneliftBackendError>
    {
        let operation = self.host_effect_operation(seat)?;
        let roles = self.semantic.synthesized_io_error_roles();
        match self.synthesized_tree_walk(operation, path)? {
            SynthesizedTreeResolution::Node(SynthesizedAggregateNode::Fixed {
                role,
                children,
            }) => Ok((SynthesizedConstructorRole::Fixed(role), children)),
            SynthesizedTreeResolution::IoErrorAlternative(position) => {
                let role = roles.get(position as usize).copied().ok_or_else(|| {
                    planner_error(
                        "synthesized aggregate path names an IOError alternative the closed \
                         inventory does not have",
                    )
                })?;
                Ok((
                    SynthesizedConstructorRole::IoError(role),
                    io_error_alternative_children(position as usize, roles.len()),
                ))
            }
            SynthesizedTreeResolution::Node(_) => Err(planner_error(
                "synthesized aggregate path does not name a constructor node",
            )),
        }
    }
    /// Walk one path from an operation's tree root to the node it names.
    ///
    /// Split out from [`Self::synthesized_tree_node`] because two callers need
    /// different things at the end of the same walk: one wants the constructor
    /// at the path, the other wants the alternative POPULATION at it. Sharing
    /// the walk is what keeps the step-kind law stated once.
    fn synthesized_tree_walk(
        &self,
        operation: ken_host::HostOpV1,
        path: &SynthesizedAggregatePath,
    ) -> Result<SynthesizedTreeResolution, CraneliftBackendError> {
        let mut node = host_effect_recipe_tree(operation).node(path.root);
        for (depth, step) in path.steps.iter().enumerate() {
            // The `IOError` set's alternatives are minted by the planner, so
            // they are resolved from the inventory rather than from a static
            // child list. This is a terminal step: an `IOError` alternative is
            // nullary or carries one scalar, and neither is a node a further
            // step can enter.
            if let SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::IoErrors) = node {
                let SynthesizedAggregateStep::Alternative(position) = step else {
                    return Err(planner_error(
                        "synthesized aggregate path takes a field step into the IOError set",
                    ));
                };
                if depth + 1 != path.steps.len() {
                    return Err(planner_error(
                        "synthesized aggregate path continues past an IOError alternative, \
                         which has no constructor-valued child",
                    ));
                }
                return Ok(SynthesizedTreeResolution::IoErrorAlternative(*position));
            }
            node = match (node, step) {
                (
                    SynthesizedAggregateNode::Fixed { children, .. },
                    SynthesizedAggregateStep::Field(position),
                ) => *children.get(*position as usize).ok_or_else(|| {
                    planner_error("synthesized aggregate path names a field the tree does not have")
                })?,
                (
                    SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::Alternatives(
                        alternatives,
                    )),
                    SynthesizedAggregateStep::Alternative(position),
                ) => *alternatives.get(*position as usize).ok_or_else(|| {
                    planner_error(
                        "synthesized aggregate path names an alternative the tree does not have",
                    )
                })?,
                // ⛔ A field step into a dynamic set, or an alternative step
                // into a fixed constructor, is not a path this tree has. The
                // step kinds are what make that a refusal rather than an index
                // that happens to be in range.
                _ => {
                    return Err(planner_error(
                        "synthesized aggregate path takes a step the node it is at cannot take",
                    ));
                }
            };
        }
        Ok(SynthesizedTreeResolution::Node(node))
    }
    /// **A DIFFERENT live effect seat running the SAME host operation.**
    ///
    /// ⭐ Same operation means the same synthesized recipe tree, so the sibling
    /// shares this seat's roles, paths and shapes exactly. That is what makes
    /// an A/B out of it: the only coordinate that differs between the two is
    /// which occurrence in the program is being lowered, and every other input
    /// to the record lookup is identical by construction.
    ///
    /// ⛔ Never an invalid or non-`Effect` origin. A refusal driven by one of
    /// those would be a refusal about seat VALIDITY, which is a different and
    /// much weaker claim than the one the discriminator makes.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn sibling_effect_seat(
        &self,
        seat: StaticOriginId,
    ) -> Option<StaticOriginId> {
        let operation = match self.source_occurrence(seat) {
            Ok(RuntimeExpr::Effect { operation, .. }) => operation.clone(),
            _ => return None,
        };
        let mut stack = vec![self.root_static_origin().ok()?];
        let mut seen = 0usize;
        while let Some(origin) = stack.pop() {
            seen += 1;
            if seen > 4096 {
                return None;
            }
            if origin != seat
                && matches!(
                    self.source_occurrence(origin),
                    Ok(RuntimeExpr::Effect { operation: other, .. }) if *other == operation
                )
            {
                return Some(origin);
            }
            let mut position = 0;
            while let Ok(child) = self.child_static_origin(origin, position) {
                stack.push(child);
                position += 1;
            }
        }
        None
    }
    /// **`RT-DECL-CLOSURE-PORT` `D7` — a READ-ONLY projection of one planned
    /// aggregate ownership record, reached by its opaque occurrence identity.**
    ///
    /// ⭐ The key is the occurrence, never a coordinate. That is the whole
    /// point: a consumer that holds a template holds its producer's identity,
    /// and this turns that identity into the planner's own facts without the
    /// consumer knowing where the producer sat or being able to search for it.
    ///
    /// ⛔ Read-only, and deliberately not a `&PlannedAggregateOwnership`. The
    /// record is the planner's; handing out a reference to it would let a
    /// consumer pattern-match its way to facts this projection has not chosen
    /// to publish, and a later field would silently become emitter-visible.
    pub(in crate::cranelift_backend) fn aggregate_record_view(
        &self,
        id: AggregateOccurrenceId,
    ) -> Result<PlannedAggregateView<'_>, CraneliftBackendError> {
        let record = self.aggregate_ownership.get(id.0 as usize).ok_or_else(|| {
            planner_error("aggregate occurrence identity is outside this plan's population")
        })?;
        // ⚠ The identity indexes the arena, so the record found must AGREE
        // that it is the one asked for. A record whose own `id` differs would
        // mean the arena's order and its contents had diverged, which nothing
        // downstream could see.
        if record.id != id {
            return Err(planner_error(
                "planned aggregate record disagrees with the identity it was found by",
            ));
        }
        Ok(PlannedAggregateView { record })
    }
    /// The closed planner population `P`, for the whole-pass relation closeout.
    pub(in crate::cranelift_backend) fn aggregate_ownership_records(
        &self,
    ) -> &[PlannedAggregateOwnership] {
        &self.aggregate_ownership
    }
    /// **The planner's closed, ordered alternative population at one path.**
    ///
    /// ⭐ This exists so the emitter can be checked for **equality** rather than
    /// for prefix agreement. Iterating the emitter's own alternative vector and
    /// resolving each position proves only that the alternatives it *did* build
    /// are the right ones; a vector missing its last alternative — or empty —
    /// agrees with every prefix of the planned population and passes. A planner
    /// tree with two `ResourceKind` alternatives then accepts an emitter
    /// carrying only alternative 0, and the missing allocation is invisible
    /// everywhere.
    ///
    /// ⛔ **Not "invisible until a later closeout" — the earlier text said
    /// that, and it was wrong.** The whole-pass close states `image(R) ⊆ P`,
    /// not equality, because `P` authorizes rather than obliges and an unused
    /// record is lawful. So the ledger cannot distinguish a truncated emitter
    /// from a record this compilation simply had no body for, and the exact
    /// cardinality can never be deferred to it.
    ///
    /// ⛔ So the count comes from HERE and never from the emitter.
    ///
    /// The path must name a dynamic node; anything else is a shape
    /// disagreement rather than a population one and is refused as such.
    pub(in crate::cranelift_backend) fn synthesized_dynamic_alternatives(
        &self,
        seat: StaticOriginId,
        path: &SynthesizedAggregatePath,
    ) -> Result<Vec<SynthesizedConstructorRole>, CraneliftBackendError> {
        self.synthesized_alternative_population(seat, path)?
            .ok_or_else(|| {
                planner_error("synthesized aggregate path does not name a dynamic alternative set")
            })
    }
    /// **The alternative population at a path, with ABSENCE typed apart from
    /// FAILURE.**
    ///
    /// ⭐ `Ok(None)` means the path **lawfully resolved** to a node that is not
    /// a dynamic set — a constructor, a scalar, a site operand, or an absent
    /// arm. `Err` means the question could not be answered at all: the seat is
    /// missing or is not an `Effect`, the walk left the tree, an `IOError`
    /// position is outside the closed inventory, or the population is
    /// malformed.
    ///
    /// ⛔ **Those are not the same answer and a caller may not merge them.** A
    /// root reconciliation that wrote `.ok()` here turned every one of those
    /// failures into "the planner plans no set at this root", so a non-dynamic
    /// emitted root then matched the absent case and was accepted. That is a
    /// missing-authority default in a function whose whole contract is that
    /// neither direction may be defaulted — and no shape or truncation mutation
    /// can find it, because both of those keep the lookup working.
    fn synthesized_alternative_population(
        &self,
        seat: StaticOriginId,
        path: &SynthesizedAggregatePath,
    ) -> Result<Option<Vec<SynthesizedConstructorRole>>, CraneliftBackendError> {
        let operation = self.host_effect_operation(seat)?;
        match self.synthesized_tree_walk(operation, path)? {
            SynthesizedTreeResolution::Node(node) => match node {
                SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::Alternatives(
                    alternatives,
                )) => alternatives
                    .iter()
                    .map(|alternative| match alternative {
                        SynthesizedAggregateNode::Fixed { role, .. } => {
                            Ok(SynthesizedConstructorRole::Fixed(*role))
                        }
                        // ⛔ A malformed population is a FAILURE, not an
                        // absence: an alternative that is not a constructor
                        // allocates nothing and the set cannot be stated.
                        _ => Err(planner_error(
                            "a dynamic aggregate alternative is not a constructor, so it \
                             allocates nothing",
                        )),
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map(Some),
                SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::IoErrors) => Ok(Some(
                    self.semantic
                        .synthesized_io_error_roles()
                        .iter()
                        .map(|role| SynthesizedConstructorRole::IoError(*role))
                        .collect(),
                )),
                // ⭐ A LAWFUL non-set. The path resolved; the node it named
                // simply is not a dynamic set. This is the only absence, and it
                // is the one a caller may act on.
                SynthesizedAggregateNode::Fixed { .. }
                | SynthesizedAggregateNode::Scalar { .. }
                | SynthesizedAggregateNode::SiteOperand(_)
                | SynthesizedAggregateNode::Absent => Ok(None),
            },
            // An alternative is a member of a set, not a set. A path that names
            // one where a set was asked for is a disagreement about the shape
            // of the tree, not a lawful absence.
            SynthesizedTreeResolution::IoErrorAlternative(_) => Err(planner_error(
                "synthesized aggregate path names an IOError alternative, not a set",
            )),
        }
    }
    /// [`Self::synthesized_alternative_population`] at a host-result ROOT.
    ///
    /// Named separately because the two callers want different things from the
    /// same answer: a dynamic CHILD is declared dynamic by its parent's child
    /// model, so `Ok(None)` there is a tree inconsistency and
    /// `synthesized_dynamic_alternatives` turns it into an error. A ROOT has
    /// nothing above it declaring its kind, so `Ok(None)` is the ordinary
    /// answer for the arms that build a constructor or nothing at all.
    pub(in crate::cranelift_backend) fn synthesized_root_alternative_population(
        &self,
        seat: StaticOriginId,
        path: &SynthesizedAggregatePath,
    ) -> Result<Option<Vec<SynthesizedConstructorRole>>, CraneliftBackendError> {
        self.synthesized_alternative_population(seat, path)
    }
    pub(in crate::cranelift_backend) fn synthesized_aggregate_children(
        &self,
        owner: ContinuationEmissionOwner,
        seat: StaticOriginId,
        path: &SynthesizedAggregatePath,
        role: SynthesizedConstructorRole,
    ) -> Result<&'static [SynthesizedAggregateNode], CraneliftBackendError> {
        self.synthesized_aggregate_record(
            owner,
            seat,
            path,
            SynthesizedAggregateRole::Constructor(role),
        )?
            .declared_children
            .ok_or_else(|| {
                planner_error("synthesized aggregate use has a record but no child model")
            })
    }
    /// The ruled allocation lane of an already-interned aggregate occurrence.
    ///
    /// The identity carries the answer from the producer to the emitter across
    /// a traversal that loses the origin. There is deliberately no fallible
    /// lookup by emission origin here: an identity this plan issued always
    /// resolves, and an identity it did not issue cannot be constructed.
    pub(in crate::cranelift_backend) fn aggregate_allocation_at(
        &self,
        occurrence: AggregateOccurrenceId,
        shape: PlannedAggregateShape,
    ) -> Result<PlannedAggregateAllocation, CraneliftBackendError> {
        let record = self
            .aggregate_ownership
            .get(occurrence.0 as usize)
            .ok_or_else(|| {
                planner_error("aggregate occurrence identity is outside this plan's population")
            })?;
        if record.shape != shape {
            return Err(planner_error(
                "aggregate producer disagrees with its planned ownership shape",
            ));
        }
        Ok(record.allocation)
    }
}


#[cfg(test)]
mod tests {
    use super::super::*;
    use super::super::tests::unit;
    use crate::RuntimeValue;

    /// A seat-bearing fixture and its first emission owner.
    ///
    /// Every `D7` row below needs a real `Effect` occurrence, because the
    /// corrected population resolves site-bound children against the seat's own
    /// operand authority. A hand-built key cannot stand in for that.
    pub(super) fn d7_seat_fixture(
        operation: ken_host::HostOpV1,
        args: Vec<RuntimeExpr>,
    ) -> (StaticTransitionPlan<'static>, StaticOriginId, ContinuationEmissionOwner) {
        let expr = Box::leak(Box::new(RuntimeExpr::Effect {
            family: "FS".to_string(),
            operation,
            capability: None,
            args,
        }));
        let plan = plan_static_transition_graph(expr, &BTreeMap::new()).expect("plans");
        let seat = plan
            .source_occurrences
            .iter()
            .flatten()
            .find(|occurrence| matches!(occurrence.expr, RuntimeExpr::Effect { .. }))
            .expect("the fixture has an effect seat")
            .static_origin;
        let owner = *synthesized_seat_emission_owners(&plan, seat)
            .expect("the seat has emission owners")
            .first()
            .expect("a seat is emitted by at least its own predeclared unit");
        (plan, seat, owner)
    }
    /// Three `Int` operands — enough for `FsReadAt`'s buffer at position 2.
    pub(super) fn d7_three_operands() -> Vec<RuntimeExpr> {
        (1..=3)
            .map(|value| RuntimeExpr::Value(RuntimeValue::Int(value.into())))
            .collect()
    }
    /// `D7` — a scalar node's owner set is derived from its EXACT disposition.
    ///
    /// MEASURED: a `spill: None` scalar yields exactly `{NoReferent}`; a
    /// `spill: Some(_)` scalar yields `{NoReferent, PersistentStore}`; and
    /// every scalar in the production tree agrees with the disposition its own
    /// `Lowered` variant declares.
    ///
    /// CLAIMED: the recorded owner set is the child's exact evidence, read off
    /// the spill field of the sole disposition authority — not a family-wide
    /// answer that happens to reach the right lane.
    ///
    /// THE GAP: neither set contains `InvocationArena`, so **this distinction
    /// changes no lane verdict today**. It is asserted because the sets are the
    /// record's stated evidence and a merely-sufficient set is a false one.
    #[test]
    fn a_scalar_nodes_owner_set_comes_from_its_exact_spill_disposition() {
        let (plan, seat, _) = d7_seat_fixture(ken_host::HostOpV1::FsWriteAt, d7_three_operands());

        let spill_free = SynthesizedAggregateNode::Scalar {
            tag: BoundaryTag::ImmediateBool,
            spill: None,
        };
        assert_eq!(
            node_referent_owners(&plan, seat, spill_free).expect("a spill-free scalar resolves"),
            vec![BoundaryReferentOwner::NoReferent],
            "a `Bool` never becomes a node at any magnitude, so `PersistentStore` \
             is not among its possible owners"
        );

        for spilling in [
            SynthesizedAggregateNode::native_int(),
            SynthesizedAggregateNode::bounded_nat(),
        ] {
            assert_eq!(
                node_referent_owners(&plan, seat, spilling)
                    .expect("a spill-capable scalar resolves"),
                vec![
                    BoundaryReferentOwner::NoReferent,
                    BoundaryReferentOwner::PersistentStore,
                ],
                "a wide {spilling:?} spills to a handle of its spill class, which \
                 is a persistent-store referent"
            );
        }

        // The two sets must actually differ, or the row is vacuous and would
        // pass against the single family-wide answer it exists to reject.
        assert_ne!(
            node_referent_owners(&plan, seat, spill_free).expect("resolves"),
            node_referent_owners(&plan, seat, SynthesizedAggregateNode::native_int())
                .expect("resolves"),
            "spill presence must change the recorded owner set, or recording the \
             disposition bought nothing over the broad immediate family"
        );

        // Every scalar the production tree declares must name a disposition an
        // emitter variant actually produces. This is the anti-drift half: the
        // tree is a second statement of a shape the disposition authority owns.
        for operation in measured_tree_operations() {
            for node in every_tree_node(operation) {
                if let SynthesizedAggregateNode::Scalar { tag, spill } = node {
                    assert!(
                        matches!(
                            (tag, spill),
                            (BoundaryTag::ImmediateBoundedNat, Some(BoundaryClass::Int))
                                | (BoundaryTag::ImmediateInt, Some(BoundaryClass::Int))
                        ),
                        "{operation:?} declares a scalar {tag:?}/{spill:?} that no \
                         emitter variant produces"
                    );
                }
            }
        }
    }
    /// **A site-bound child is RESOLVED against the seat, never pruned.**
    ///
    /// MEASURED: at a real `FsReadFile` seat, `OptionSome` — whose only child
    /// is the seat's path operand — has a record, and so does `FileError`,
    /// which contains it. At `FsReadAt`, `PrivateBufferSpan` and `ReadSome`
    /// have records. The owner set recorded for the site-bound child equals the
    /// one the seat's own operand occurrence yields.
    ///
    /// CLAIMED: site-dependence bounds the *role-invariant* meet, not the meet.
    /// Every one of these is a real allocation production emits, so each gets
    /// an exact site-bound record.
    ///
    /// THE GAP — this is the correction's whole subject, so it is worth saying
    /// plainly what the previous spelling did. It pruned these four
    /// constructors from `P` on the grounds that no role-invariant answer
    /// existed. That is not the fail-closed direction: it left four
    /// allocations that production emits with **no record at all**, which reads
    /// as "refuses at allocation" and was in fact "allocates with the unproven
    /// persistent lane, via the value-shape disposition".
    #[test]
    fn a_site_bound_child_is_resolved_against_the_seat_not_pruned() {
        use SynthesizedAggregateRoot::{HostResultError as ERR, HostResultOk as OK};
        use SynthesizedConstructorRole::Fixed;
        use SynthesizedFixedConstructorRole as R;

        // `OptionSome` and its parent `FileError`, at a file-operation seat.
        let (plan, seat, owner) = d7_seat_fixture(
            ken_host::HostOpV1::FsReadFile,
            vec![RuntimeExpr::Value(RuntimeValue::Int(1.into()))],
        );
        let err = SynthesizedAggregatePath::root(ERR);
        for (path, role) in [
            (err.clone(), R::FileError),
            (err.field(0), R::FileOperationRead),
            (err.field(1), R::OptionSome),
        ] {
            plan.synthesized_aggregate_occurrence(owner, seat, &path, Fixed(role))
                .unwrap_or_else(|error| {
                    panic!("{role:?} at {path:?} must have a record, not be pruned: {error:?}")
                });
        }

        // The site-bound child's recorded owners ARE the seat's operand's.
        let record = plan
            .synthesized_aggregate_record(
                owner,
                seat,
                &err.field(1),
                SynthesizedAggregateRole::Constructor(Fixed(R::OptionSome)),
            )
            .expect("`OptionSome` has a record");
        let authority = occurrence_authority(&plan, seat).expect("the seat has an authority");
        let operand = authority
            .children
            .first()
            .expect("the seat has its path operand");
        assert_eq!(
            record.children[0].owners,
            aggregate_child_referent_owners(&plan, operand).expect("the operand resolves"),
            "the site-bound child's evidence must BE the seat operand's, not a \
             conservative stand-in for it"
        );

        // `PrivateBufferSpan` and `ReadSome`, at a positioned-read seat.
        let (plan, seat, owner) =
            d7_seat_fixture(ken_host::HostOpV1::FsReadAt, d7_three_operands());
        let ok = SynthesizedAggregatePath::root(OK);
        for (path, role) in [
            (ok.alternative(1), R::ReadSome),
            (ok.alternative(1).field(0), R::PrivateBufferSpan),
            (ok.alternative(1).field(1), R::PrivateTransferCount),
            (ok.alternative(0), R::ReadEof),
        ] {
            plan.synthesized_aggregate_occurrence(owner, seat, &path, Fixed(role))
                .unwrap_or_else(|error| {
                    panic!("{role:?} at {path:?} must have a record: {error:?}")
                });
        }
    }
    /// **An `Absent` node is never a child.**
    ///
    /// MEASURED: deriving owners for `Absent` in a child position is a planner
    /// refusal, while the derivable node in the same position resolves.
    ///
    /// CLAIMED: `Absent` marks a host-result arm that builds no aggregate, and
    /// it is a distinct arm from `SiteOperand` precisely so "nothing is built
    /// here" cannot be read as "a child the site supplies".
    ///
    /// THE GAP: this drives the derivation directly; that the production tree
    /// puts `Absent` only at roots is the tree's own shape.
    #[test]
    fn an_absent_node_is_never_a_child() {
        let (plan, seat, _) = d7_seat_fixture(ken_host::HostOpV1::FsWriteAt, d7_three_operands());
        assert!(
            node_referent_owners(&plan, seat, SynthesizedAggregateNode::Absent).is_err(),
            "an absent child means the tree claims an allocation whose operand \
             is not built"
        );
        // The positive control: the same position with a real node resolves,
        // so the refusal above is not "children never resolve".
        assert!(
            node_referent_owners(&plan, seat, SynthesizedAggregateNode::native_int()).is_ok(),
            "a scalar in the same position must resolve, or the refusal is vacuous"
        );
    }
    /// **A dynamic child's owners are the UNION of its alternatives'.**
    ///
    /// MEASURED: a two-alternative set whose members are both persistent yields
    /// `{PersistentStore}`. The SAME set with one arena-capable alternative
    /// yields a set containing `InvocationArena`, and a parent holding it is
    /// `InvocationAggregate` rather than `PersistentGround`.
    ///
    /// CLAIMED: the value at a dynamic position is whichever alternative the
    /// discriminator selects, so the parent must survive every one of them —
    /// one invocation-capable alternative makes the parent invocation-owned.
    ///
    /// THE GAP: the arena-capable alternative is reached through a seat operand
    /// that is a closure, because **no alternative in the production tree is
    /// arena-capable today**. So this proves the UNION rule and the lane it
    /// selects, not that any production parent takes the invocation lane
    /// through a dynamic child. Without the escaping half the row would pass
    /// equally for the flat `{PersistentStore}` answer it replaced, which is
    /// why both halves are asserted against the same set shape.
    #[test]
    fn a_dynamic_childs_owners_are_the_union_of_its_alternatives() {
        use SynthesizedAggregateNode as N;
        use SynthesizedFixedConstructorRole as R;

        // Operand 0 is a closure, which `derive_occurrence_lifetime` answers
        // `ActivationOwned` for -- so `SiteOperand(0)` at this seat is the
        // arena-capable leaf the production tree does not have.
        let (plan, seat, _) = d7_seat_fixture(
            ken_host::HostOpV1::FsWriteAt,
            vec![
                RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: vec!["x".to_string()],
                    body: Box::new(unit()),
                },
                RuntimeExpr::Value(RuntimeValue::Int(2.into())),
                RuntimeExpr::Value(RuntimeValue::Int(3.into())),
            ],
        );
        assert!(
            node_referent_owners(&plan, seat, N::SiteOperand(0))
                .expect("the closure operand resolves")
                .contains(&BoundaryReferentOwner::InvocationArena),
            "the fixture's operand 0 must be arena-capable, or the escaping half              below is not testing anything"
        );

        // Both alternatives persistent.
        const PERSISTENT: &[SynthesizedAggregateNode] = &[
            N::nullary(R::ReadEof),
            N::Fixed {
                role: R::PrivateTransferCount,
                children: &[N::bounded_nat(), N::bounded_nat()],
            },
        ];
        assert_eq!(
            node_referent_owners(
                &plan,
                seat,
                N::Dynamic(SynthesizedDynamicSet::Alternatives(PERSISTENT))
            )
            .expect("an all-persistent set resolves"),
            vec![BoundaryReferentOwner::PersistentStore],
            "no alternative can be arena-owned, so the union is persistent"
        );

        // The SAME shape with one arena-capable alternative.
        const ESCAPING: &[SynthesizedAggregateNode] = &[
            N::nullary(R::ReadEof),
            N::Fixed {
                role: R::PrivateTransferCount,
                children: &[N::SiteOperand(0)],
            },
        ];
        let union = node_referent_owners(
            &plan,
            seat,
            N::Dynamic(SynthesizedDynamicSet::Alternatives(ESCAPING)),
        )
        .expect("a set with an escaping alternative resolves");
        assert!(
            union.contains(&BoundaryReferentOwner::InvocationArena),
            "one arena-capable alternative must reach the union, got {union:?}"
        );

        // And it must change the PARENT's lane, not merely its owner set.
        const PARENT_PERSISTENT: &[SynthesizedAggregateNode] =
            &[N::Dynamic(SynthesizedDynamicSet::Alternatives(PERSISTENT))];
        const PARENT_ESCAPING: &[SynthesizedAggregateNode] =
            &[N::Dynamic(SynthesizedDynamicSet::Alternatives(ESCAPING))];
        assert_eq!(
            fixed_node_selected_owner(&plan, seat, PARENT_PERSISTENT)
                .expect("the persistent parent resolves"),
            BoundaryReferentOwner::PersistentStore
        );
        assert_eq!(
            fixed_node_selected_owner(&plan, seat, PARENT_ESCAPING)
                .expect("the escaping parent resolves"),
            BoundaryReferentOwner::InvocationArena,
            "a parent whose dynamic child has ONE invocation-capable alternative              must take the invocation lane; answering persistent because the set              is shaped persistently would allocate it over a child that can be              shorter-lived than it"
        );

        // An empty alternative list is a refusal, not a vacuous persistent
        // answer -- an empty owner set satisfies the escape test trivially.
        assert!(
            node_referent_owners(
                &plan,
                seat,
                N::Dynamic(SynthesizedDynamicSet::Alternatives(&[]))
            )
            .is_err(),
            "an alternative-less dynamic child must refuse rather than resolve              persistent by having nothing to check"
        );
    }
    /// **`D7` — the flattening reproduces the MEASURED tree, alternatives
    /// included.**
    ///
    /// MEASURED: instrumenting every `synthesized_constructor` and
    /// `synthesized_dynamic_alternative` call with the kinds of its own
    /// children, single-threaded, produced the edge set restated below. The
    /// corrected flattening interns every constructor-valued node: fixed nodes,
    /// every dynamic alternative at its ordered position, and every
    /// planner-issued `IOError` role.
    ///
    /// CLAIMED: the tree in `host_effect_recipe_tree` is the structure the
    /// emitter actually builds, the paths it flattens to are the positions
    /// those uses occupy, and `P` now covers every allocation.
    ///
    /// THE GAP: this pins the flattening against **the measurement**, not
    /// against the emitter. What keeps the emitter honest is the
    /// per-construction reconciliation. `FsWriteFile` and `FsChangeMode` have
    /// no fixture, so their rows are derived from the emitter's own operation
    /// match rather than observed, and are marked below.
    #[test]
    fn the_flattening_reproduces_the_measured_tree() {
        use ken_host::HostOpV1 as Op;
        use SynthesizedAggregateRoot::{HostResultError as ERR, HostResultOk as OK};
        use SynthesizedConstructorRole::Fixed;
        use SynthesizedFixedConstructorRole as R;

        let (plan, _, _) = d7_seat_fixture(Op::FsWriteAt, d7_three_operands());
        let io_errors = plan.semantic.synthesized_io_error_roles().len();
        assert!(
            io_errors > 1,
            "the closed IOError inventory must be non-trivial, or the alternative \
             rows below are vacuous"
        );

        let path = |root, steps: &[SynthesizedAggregateStep]| SynthesizedAggregatePath {
            root,
            steps: steps.to_vec(),
        };
        let field = SynthesizedAggregateStep::Field;
        let alt = SynthesizedAggregateStep::Alternative;

        // The eleven resource-surface alternatives, in the emitter's order.
        let surface: Vec<(SynthesizedAggregatePath, SynthesizedConstructorRole)> = [
            R::ResourceHostIo,
            R::ResourceClosed,
            R::ResourceMalformed,
            R::ResourceRightNotHeld,
            R::ResourceReleaseFailed,
            R::ResourceKindMismatch,
            R::ResourceBufferLimit,
            R::ResourceAllocationFailed,
            R::ResourceInvalidOffset,
            R::ResourceInvalidBounds,
            R::ResourceNoProgress,
        ]
        .into_iter()
        .enumerate()
        .map(|(index, role)| (path(ERR, &[alt(index as u32)]), Fixed(role)))
        .chain(
            // `ResourceKind` at its THREE distinct parent paths, each with its
            // own two alternatives. These are the repeated-role sites.
            [
                (4_u32, 0_u32),
                (5, 0),
                (5, 1),
            ]
            .into_iter()
            .flat_map(|(alternative, position)| {
                [R::ResourceKindFsHandle, R::ResourceKindBuffer]
                    .into_iter()
                    .enumerate()
                    .map(move |(index, role)| {
                        (
                            path(
                                ERR,
                                &[alt(alternative), field(position), alt(index as u32)],
                            ),
                            Fixed(role),
                        )
                    })
            }),
        )
        .chain(std::iter::once((
            path(ERR, &[alt(4), field(1)]),
            Fixed(R::ResourceTraceIdentity),
        )))
        .collect();

        // The two `IOError` sets the resource surface reaches.
        let surface_io: Vec<(SynthesizedAggregatePath, SynthesizedConstructorRole)> =
            [(0_u32, 0_u32), (4, 2)]
                .into_iter()
                .flat_map(|(alternative, position)| {
                    plan.semantic
                        .synthesized_io_error_roles()
                        .iter()
                        .enumerate()
                        .map(move |(index, role)| {
                            (
                                path(
                                    ERR,
                                    &[alt(alternative), field(position), alt(index as u32)],
                                ),
                                SynthesizedConstructorRole::IoError(*role),
                            )
                        })
                })
                .collect();

        let file_error = |operation: R| -> Vec<(SynthesizedAggregatePath, SynthesizedConstructorRole)> {
            let mut rows = vec![
                (path(ERR, &[]), Fixed(R::FileError)),
                (path(ERR, &[field(0)]), Fixed(operation)),
                (path(ERR, &[field(1)]), Fixed(R::OptionSome)),
            ];
            rows.extend(
                plan.semantic
                    .synthesized_io_error_roles()
                    .iter()
                    .enumerate()
                    .map(|(index, role)| {
                        (
                            path(ERR, &[field(2), alt(index as u32)]),
                            SynthesizedConstructorRole::IoError(*role),
                        )
                    }),
            );
            rows
        };
        let console_error = || -> Vec<(SynthesizedAggregatePath, SynthesizedConstructorRole)> {
            plan.semantic
                .synthesized_io_error_roles()
                .iter()
                .enumerate()
                .map(|(index, role)| {
                    (
                        path(ERR, &[alt(index as u32)]),
                        SynthesizedConstructorRole::IoError(*role),
                    )
                })
                .collect()
        };
        let unit = || vec![(path(OK, &[]), Fixed(R::Unit))];

        let expected: Vec<(Op, Vec<(SynthesizedAggregatePath, SynthesizedConstructorRole)>)> = vec![
            // Returns a `Bool` above the synthesis entirely.
            (Op::ConsoleIsTerminal, vec![]),
            (
                Op::ConsoleWrite,
                console_error().into_iter().chain(unit()).collect(),
            ),
            (
                Op::ConsoleFlush,
                console_error().into_iter().chain(unit()).collect(),
            ),
            (Op::FsReadFile, file_error(R::FileOperationRead)),
            (Op::FsOpen, file_error(R::FileOperationRead)),
            // ⚠ DERIVED, not observed — no fixture exercises these two. The
            // `Unit` row is the emitter's `else` branch, which the flat use
            // table this tree replaced had MISSED.
            (
                Op::FsWriteFile,
                file_error(R::FileOperationWrite)
                    .into_iter()
                    .chain(unit())
                    .collect(),
            ),
            (
                Op::FsChangeMode,
                file_error(R::FileOperationChangeMode)
                    .into_iter()
                    .chain(unit())
                    .collect(),
            ),
            (
                Op::BufferAllocate,
                surface.iter().cloned().chain(surface_io.clone()).collect(),
            ),
            (
                Op::BufferFreeze,
                surface.iter().cloned().chain(surface_io.clone()).collect(),
            ),
            (
                Op::FsHandleMetadata,
                surface.iter().cloned().chain(surface_io.clone()).collect(),
            ),
            (
                Op::ResourceRelease,
                surface
                    .iter()
                    .cloned()
                    .chain(surface_io.clone())
                    .chain(unit())
                    .collect(),
            ),
            (
                Op::FsReadAt,
                surface
                    .iter()
                    .cloned()
                    .chain(surface_io.clone())
                    .chain([
                        (path(OK, &[alt(0)]), Fixed(R::ReadEof)),
                        (path(OK, &[alt(1)]), Fixed(R::ReadSome)),
                        (path(OK, &[alt(1), field(0)]), Fixed(R::PrivateBufferSpan)),
                        (
                            path(OK, &[alt(1), field(1)]),
                            Fixed(R::PrivateTransferCount),
                        ),
                    ])
                    .collect(),
            ),
            (
                Op::FsWriteAt,
                surface
                    .iter()
                    .cloned()
                    .chain(surface_io.clone())
                    .chain([
                        (path(OK, &[]), Fixed(R::Wrote)),
                        (path(OK, &[field(0)]), Fixed(R::PrivateTransferCount)),
                    ])
                    .collect(),
            ),
        ];

        for (operation, rows) in &expected {
            let flattened = flatten_allocation_reachable_uses(&plan, *operation)
                .into_iter()
                .map(|semantic_use| (semantic_use.path, semantic_use.role))
                .collect::<BTreeSet<_>>();
            let wanted = rows.iter().cloned().collect::<BTreeSet<_>>();
            assert_eq!(
                flattened, wanted,
                "{operation:?}'s allocation-reachable population disagrees with \
                 the measured tree"
            );
        }
    }
    /// **A repeated role at ONE seat gets DISTINCT REAL RECORDS.**
    ///
    /// MEASURED: at one `FsWriteAt` seat, `ResourceKindFsHandle` has three
    /// records — under `ResourceReleaseFailed` field 0 and `ResourceKindMismatch`
    /// fields 0 and 1 — with three distinct occurrence identities. The `IOError`
    /// set likewise has two reachable positions on that tree, each with a full
    /// set of per-role records.
    ///
    /// CLAIMED: `(owner, seat, path, role)` is injective where
    /// `(owner, seat, role)` is not, and the separation now produces **records**
    /// rather than only distinguishable keys.
    ///
    /// THE GAP: the previous spelling of this row asserted only that the six
    /// sites had distinct *paths*, and said so — because under the fixed-only
    /// population none of them had a record. That is exactly what the Architect
    /// rejected. This row now reads the real population.
    #[test]
    fn a_repeated_role_at_one_seat_gets_distinct_real_records() {
        use SynthesizedAggregateRoot::HostResultError as ERR;
        use SynthesizedConstructorRole::Fixed;
        use SynthesizedFixedConstructorRole as R;

        let (plan, seat, owner) =
            d7_seat_fixture(ken_host::HostOpV1::FsWriteAt, d7_three_operands());
        let err = SynthesizedAggregatePath::root(ERR);

        // The three `ResourceKind` parent paths, each interning its own two
        // alternatives.
        let kind_parents = [
            err.alternative(4).field(0),
            err.alternative(5).field(0),
            err.alternative(5).field(1),
        ];
        let mut identities = BTreeSet::new();
        for parent in &kind_parents {
            for (position, role) in
                [R::ResourceKindFsHandle, R::ResourceKindBuffer].iter().enumerate()
            {
                let occurrence = plan
                    .synthesized_aggregate_occurrence(
                        owner,
                        seat,
                        &parent.alternative(position as u32),
                        Fixed(*role),
                    )
                    .unwrap_or_else(|error| {
                        panic!("{role:?} under {parent:?} must have a REAL record: {error:?}")
                    });
                assert!(
                    identities.insert(occurrence),
                    "{role:?} under {parent:?} reused an identity, so one record \
                     is authorizing two allocations"
                );
            }
        }
        assert_eq!(
            identities.len(),
            6,
            "three parent paths x two alternatives is six non-aliasing records"
        );

        // Each reachable `IOError` set produces path-keyed `IoError(role)`
        // records, and the two sets do not share one.
        let roles = plan.semantic.synthesized_io_error_roles().to_vec();
        let mut io_identities = BTreeSet::new();
        for parent in [err.alternative(0).field(0), err.alternative(4).field(2)] {
            for (position, role) in roles.iter().enumerate() {
                let occurrence = plan
                    .synthesized_aggregate_occurrence(
                        owner,
                        seat,
                        &parent.alternative(position as u32),
                        SynthesizedConstructorRole::IoError(*role),
                    )
                    .unwrap_or_else(|error| {
                        panic!("IOError {position} under {parent:?} must have a record: {error:?}")
                    });
                assert!(
                    io_identities.insert(occurrence),
                    "IOError {position} under {parent:?} reused an identity"
                );
            }
        }
        assert_eq!(
            io_identities.len(),
            roles.len() * 2,
            "two reachable IOError sets x the closed inventory is that many records"
        );
    }
    /// **A path is not an index: the STEP KINDS carry information.**
    ///
    /// MEASURED: taking a `Field` step where the tree has a dynamic set, or an
    /// `Alternative` step where it has a fixed constructor, is refused — even
    /// when the position is in range. Collapsing, dropping or swapping a step
    /// resolves to a different node or to nothing. A path continuing past an
    /// `IOError` alternative is refused.
    ///
    /// CLAIMED: `SynthesizedAggregatePath` names at most one node, and the
    /// mutations a hand-written path in `core.rs` could plausibly contain are
    /// each caught rather than silently landing on a neighbour.
    ///
    /// THE GAP: this drives the resolver directly. That the *emitter's* paths
    /// are right is a different claim, held by the construction-time
    /// reconciliation.
    #[test]
    fn a_path_step_kind_is_load_bearing_not_an_index() {
        use SynthesizedAggregateRoot::{HostResultError as ERR, HostResultOk as OK};
        use SynthesizedConstructorRole::Fixed;
        use SynthesizedFixedConstructorRole as R;

        let (plan, seat, _) = d7_seat_fixture(ken_host::HostOpV1::FsWriteAt, d7_three_operands());
        let err = SynthesizedAggregatePath::root(ERR);
        let ok = SynthesizedAggregatePath::root(OK);

        // The truth: `ResourceTraceIdentity` at alternative 4, field 1.
        assert_eq!(
            plan.synthesized_tree_node(seat, &err.alternative(4).field(1))
                .expect("the measured path resolves")
                .0,
            Fixed(R::ResourceTraceIdentity)
        );

        // COLLAPSE — the same two positions with the alternative step dropped.
        // Field 4 of a dynamic set is not a step that set can take.
        plan.synthesized_tree_node(seat, &err.field(4).field(1))
            .expect_err("a field step into a dynamic set must refuse");

        // REMOVE — one step short lands on the alternative itself, which is a
        // constructor, so this resolves to a DIFFERENT role rather than
        // failing. That is exactly why the role is compared too.
        assert_eq!(
            plan.synthesized_tree_node(seat, &err.alternative(4))
                .expect("the alternative itself is a node")
                .0,
            Fixed(R::ResourceReleaseFailed),
            "a dropped step must not silently keep the same role"
        );

        // SWAP — the two step kinds exchanged.
        plan.synthesized_tree_node(seat, &err.field(4).alternative(1))
            .expect_err("an alternative step into a constructor must refuse");

        // A sibling FIELD is a different node: field 0 of that alternative is
        // the `ResourceKind` dynamic set, which is not a constructor itself.
        plan.synthesized_tree_node(seat, &err.alternative(4).field(0))
            .expect_err("a dynamic set is not a constructor node");

        // UNPLANNED — a position past the end of the alternative list.
        plan.synthesized_tree_node(
            seat,
            &err.alternative(
                u32::try_from(
                    plan.synthesized_dynamic_alternatives(seat, &err)
                        .expect("the error root is the resource surface")
                        .len(),
                )
                .expect("the inventory fits"),
            ),
        )
        .expect_err("a position past the closed resource-error inventory must refuse");

        // An `IOError` alternative is terminal: nothing below it is a
        // constructor, so a path continuing past one names no node.
        let io = err.alternative(0).field(0);
        plan.synthesized_tree_node(seat, &io.alternative(0))
            .expect("an IOError alternative is a node");
        plan.synthesized_tree_node(seat, &io.alternative(0).field(0))
            .expect_err("a path may not continue past an IOError alternative");
        plan.synthesized_tree_node(
            seat,
            &io.alternative(u32::try_from(plan.semantic.synthesized_io_error_roles().len())
                .expect("the inventory fits")),
        )
        .expect_err("a position past the closed IOError inventory must refuse");

        // The two ROOTS are not interchangeable.
        assert_eq!(
            plan.synthesized_tree_node(seat, &ok)
                .expect("the ok root of `FsWriteAt` is `Wrote`")
                .0,
            Fixed(R::Wrote)
        );
        assert_ne!(
            plan.synthesized_tree_node(seat, &ok).map(|node| node.0).ok(),
            plan.synthesized_tree_node(seat, &err).map(|node| node.0).ok(),
            "the two arms must not resolve to the same node"
        );
    }
    /// **The ABANDONED eager `IOError` template contributes neither `P` nor an
    /// allocation — because it is no longer built.**
    ///
    /// MEASURED: no operation's tree names an `IOError` set at the bare error
    /// root except the console operations, whose whole error value IS that set.
    /// For the six resource-surface operations, `P` contains no record at the
    /// error root, and `lower_process_host_effect` constructs the generic
    /// template only inside the two branches that use it.
    ///
    /// CLAIMED: the template contributes no record and no event, and does so by
    /// **not existing** rather than by being planned and proven unreachable.
    ///
    /// THE GAP: this is a statement about the tree and about `P`. That the
    /// emitter no longer builds it eagerly is a source fact the reconciliation
    /// enforces indirectly — an eagerly-built template would have to be given
    /// some path, and the resource operations' trees have no `IOError` node at
    /// the root to give it.
    #[test]
    fn an_abandoned_eager_template_contributes_neither_records_nor_allocation() {
        use ken_host::HostOpV1 as Op;
        use SynthesizedAggregateNode as N;
        use SynthesizedAggregateRoot::HostResultError as ERR;

        let (plan, _, _) = d7_seat_fixture(Op::FsWriteAt, d7_three_operands());
        for operation in [
            Op::FsHandleMetadata,
            Op::ResourceRelease,
            Op::BufferAllocate,
            Op::BufferFreeze,
            Op::FsReadAt,
            Op::FsWriteAt,
        ] {
            let root = host_effect_recipe_tree(operation).node(ERR);
            assert!(
                !matches!(root, N::Dynamic(SynthesizedDynamicSet::IoErrors)),
                "{operation:?} builds its own surface error, so the eager \
                 template is abandoned and must not be at its error root"
            );
            assert!(
                flatten_allocation_reachable_uses(&plan, operation)
                    .iter()
                    .all(|semantic_use| {
                        semantic_use.path != SynthesizedAggregatePath::root(ERR)
                    }),
                "{operation:?} plans a record at the error root, which is the \
                 abandoned template's position"
            );
        }

        // The positive control: the operations that DO use the generic template
        // have it exactly where the measurement put it. Without this, the rows
        // above would pass for an implementation that never emits an `IOError`
        // set at all.
        assert!(
            matches!(
                host_effect_recipe_tree(Op::ConsoleWrite).node(ERR),
                N::Dynamic(SynthesizedDynamicSet::IoErrors)
            ),
            "`ConsoleWrite`'s whole error value IS the generic template"
        );
        let file_children = match host_effect_recipe_tree(Op::FsReadFile).node(ERR) {
            N::Fixed { children, .. } => children,
            other => panic!("`FsReadFile`'s error root is `FileError`, got {other:?}"),
        };
        assert!(
            matches!(
                file_children[2],
                N::Dynamic(SynthesizedDynamicSet::IoErrors)
            ),
            "`FileError` field 2 IS the generic template"
        );
    }
    /// **A record's lookup key includes its PATH, not only its role.**
    ///
    /// MEASURED: at a real `FsWriteAt` seat, `ResourceKindFsHandle` resolves at
    /// `error.alternative(4).field(0).alternative(0)` and refuses at every
    /// other path — with the owner, seat and role held identical. The
    /// population now genuinely COLLIDES on `(owner, seat, role)`: that role
    /// has three records at one seat.
    ///
    /// CLAIMED: `synthesized_aggregate_record` matches on all four key parts,
    /// so a path-keyed population cannot silently degrade to a role-keyed one.
    ///
    /// THE GAP — worth recording because it changed under the correction. Under
    /// the earlier fixed-only population this row had to be driven by a
    /// hand-built key, because no two records shared `(owner, seat, role)` and
    /// dropping the path from the lookup left the suite green. With every
    /// alternative interned, the collision is real and the production
    /// population itself discriminates.
    #[test]
    fn a_records_lookup_key_includes_its_path_not_only_its_role() {
        use SynthesizedAggregateRoot::{HostResultError as ERR, HostResultOk as OK};
        use SynthesizedConstructorRole::Fixed;
        use SynthesizedFixedConstructorRole as R;

        let (plan, seat, owner) =
            d7_seat_fixture(ken_host::HostOpV1::FsWriteAt, d7_three_operands());
        let err = SynthesizedAggregatePath::root(ERR);

        // The real collision: one role, one seat, three records.
        let colliding = [
            err.alternative(4).field(0).alternative(0),
            err.alternative(5).field(0).alternative(0),
            err.alternative(5).field(1).alternative(0),
        ];
        let resolved = colliding
            .iter()
            .map(|path| {
                plan.synthesized_aggregate_occurrence(
                    owner,
                    seat,
                    path,
                    Fixed(R::ResourceKindFsHandle),
                )
                .expect("each of the three uses has its own record")
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(
            resolved.len(),
            3,
            "one role at one seat resolves to three DIFFERENT occurrences, which \
             is only possible if the path is in the key"
        );

        // And paths that name no node refuse rather than falling back.
        for wrong in [
            err.alternative(4).field(0).alternative(7),
            err.clone(),
            SynthesizedAggregatePath::root(OK),
        ] {
            assert!(
                plan.synthesized_aggregate_occurrence(
                    owner,
                    seat,
                    &wrong,
                    Fixed(R::ResourceKindFsHandle)
                )
                .is_err(),
                "{wrong:?} must not resolve; owner, seat and role are identical, \
                 so only the path can refuse it"
            );
        }

        // The role is still part of the key too: the right path with the wrong
        // role refuses. Without this, the rows above would pass for a lookup
        // that matched on the path alone.
        assert!(
            plan.synthesized_aggregate_occurrence(owner, seat, &colliding[0], Fixed(R::Wrote))
                .is_err(),
            "the right path with the wrong role must refuse"
        );
    }
    /// **The PLANNER owns the alternative population, count included.**
    ///
    /// MEASURED: at a real `FsWriteAt` seat the resource surface reports eleven
    /// ordered roles, `ResourceKind` reports its two, and a reachable `IOError`
    /// set reports the whole closed inventory. A path that names a constructor,
    /// an `IOError` alternative, or nothing at all is refused rather than
    /// answering an empty population.
    ///
    /// CLAIMED: an emitter's alternative vector can be compared for EQUALITY
    /// against this, so a truncated set is caught at construction.
    ///
    /// THE GAP — this is the correction's subject, so it is worth stating what
    /// the previous spelling did. It iterated the emitter's own vector and
    /// resolved each position, which rejects an EXTRA alternative (its path
    /// does not exist) but accepts a MISSING final one, because a prefix agrees
    /// with every position it has. The empty vector agreed with everything.
    ///
    /// ⛔ The gap statement that stood here added that a whole-pass
    /// `image(R) = P` closeout *"would eventually surface the unused records"*.
    /// It would not, and there is no such closeout: `P` is an authorization
    /// population, the whole-pass close states `image(R) ⊆ P`, and a planned
    /// record no event related is lawful. A truncated emitter and a lawfully
    /// unused record are the same shape at the ledger, so this boundary has to
    /// establish the equality itself rather than borrow it from a later pass.
    ///
    /// ⚠ The refusal on an empty population is what makes the count usable: a
    /// zero-length expectation would make the emitter's own emptiness agree.
    #[test]
    fn the_planner_owns_the_ordered_alternative_population() {
        use SynthesizedAggregateRoot::{HostResultError as ERR, HostResultOk as OK};
        use SynthesizedConstructorRole::Fixed;
        use SynthesizedFixedConstructorRole as R;

        let (plan, seat, _) = d7_seat_fixture(ken_host::HostOpV1::FsWriteAt, d7_three_operands());
        let err = SynthesizedAggregatePath::root(ERR);

        assert_eq!(
            plan.synthesized_dynamic_alternatives(seat, &err)
                .expect("the error root is the resource surface"),
            vec![
                Fixed(R::ResourceHostIo),
                Fixed(R::ResourceClosed),
                Fixed(R::ResourceMalformed),
                Fixed(R::ResourceRightNotHeld),
                Fixed(R::ResourceReleaseFailed),
                Fixed(R::ResourceKindMismatch),
                Fixed(R::ResourceBufferLimit),
                Fixed(R::ResourceAllocationFailed),
                Fixed(R::ResourceInvalidOffset),
                Fixed(R::ResourceInvalidBounds),
                Fixed(R::ResourceNoProgress),
            ],
            "the surface population is ordered and closed, and its COUNT is the \
             planner's rather than whatever the emitter built"
        );

        assert_eq!(
            plan.synthesized_dynamic_alternatives(seat, &err.alternative(4).field(0))
                .expect("`ResourceReleaseFailed` field 0 is the `ResourceKind` set"),
            vec![Fixed(R::ResourceKindFsHandle), Fixed(R::ResourceKindBuffer)],
        );

        let roles = plan.semantic.synthesized_io_error_roles().to_vec();
        assert!(roles.len() > 1, "the closed inventory must be non-trivial");
        assert_eq!(
            plan.synthesized_dynamic_alternatives(seat, &err.alternative(0).field(0))
                .expect("`ResourceHostIo` field 0 is a reachable IOError set"),
            roles
                .iter()
                .map(|role| SynthesizedConstructorRole::IoError(*role))
                .collect::<Vec<_>>(),
            "a reachable IOError set reports its whole closed inventory"
        );

        // ⛔ Every non-set path REFUSES rather than reporting an empty
        // population. An empty expectation would make an emitter's own
        // emptiness agree, which is the failure this row exists to exclude.
        for wrong in [
            err.alternative(4),                          // a constructor
            err.alternative(4).field(1),                 // a constructor
            err.alternative(0).field(0).alternative(0),  // an IOError ALTERNATIVE
            err.alternative(12),                         // no node at all
            SynthesizedAggregatePath::root(OK),          // `Wrote`, a constructor
        ] {
            assert!(
                plan.synthesized_dynamic_alternatives(seat, &wrong).is_err(),
                "{wrong:?} does not name an alternative set and must refuse, not \
                 report an empty population"
            );
        }
    }
    /// **A lawful non-dynamic root and a FAILED lookup are different answers.**
    ///
    /// MEASURED: at a real `FsWriteAt` seat the error root reports a population
    /// and the `ok` root — `Wrote`, a constructor — reports `Ok(None)`. At an
    /// `FsReadFile` seat the `ok` root is `Absent` and also reports `Ok(None)`.
    /// A seat that is not an `Effect` occurrence, and a path that leaves the
    /// tree or names an `IOError` position outside the closed inventory, each
    /// report `Err`.
    ///
    /// CLAIMED: absence is typed apart from failure, so a caller cannot act on
    /// "the planner plans no set here" when what actually happened is that the
    /// question could not be answered.
    ///
    /// THE GAP — and it is the reason this row exists. The root reconciliation
    /// wrote `.ok()` on this query, which merged every failure into absence: a
    /// non-dynamic emitted root then matched the absent case and was accepted,
    /// a missing-authority default inside a function whose stated contract is
    /// that neither direction may be defaulted. **No shape or truncation
    /// mutation can find that**, because both keep the lookup working — which
    /// is why the five root mutations were all green against it. The
    /// distinguishing evidence is exactly the two halves below, and the
    /// positive half matters as much as the negative: without it, the
    /// correction would be satisfied by making every non-dynamic root fail.
    #[test]
    fn a_lawful_non_dynamic_root_is_not_a_failed_lookup() {
        use SynthesizedAggregateRoot::{HostResultError as ERR, HostResultOk as OK};

        let (plan, seat, _) = d7_seat_fixture(ken_host::HostOpV1::FsWriteAt, d7_three_operands());
        let err = SynthesizedAggregatePath::root(ERR);
        let ok = SynthesizedAggregatePath::root(OK);

        // Dynamic root: a population.
        assert_eq!(
            plan.synthesized_root_alternative_population(seat, &err)
                .expect("the error root resolves")
                .expect("the error root is the resource surface")
                .len(),
            11
        );

        // ⭐ LAWFULLY non-dynamic: `Wrote` is a constructor, so the answer is a
        // resolved absence rather than a failure.
        assert_eq!(
            plan.synthesized_root_alternative_population(seat, &ok)
                .expect("the ok root resolves lawfully"),
            None,
            "a constructor root is a resolved non-set, not a failed lookup"
        );

        // The same for an arm that builds no aggregate at all.
        let (absent_plan, absent_seat, _) = d7_seat_fixture(
            ken_host::HostOpV1::FsReadFile,
            vec![RuntimeExpr::Value(RuntimeValue::Int(1.into()))],
        );
        assert_eq!(
            absent_plan
                .synthesized_root_alternative_population(
                    absent_seat,
                    &SynthesizedAggregatePath::root(OK)
                )
                .expect("an absent ok arm resolves lawfully"),
            None,
            "`FsReadFile`'s `ok` builds no aggregate, which is still a RESOLVED \
             answer"
        );

        // ⛔ And every way the question can fail to be answerable stays `Err`.
        let non_effect = plan
            .source_occurrences
            .iter()
            .flatten()
            .find(|occurrence| !matches!(occurrence.expr, RuntimeExpr::Effect { .. }))
            .expect("the fixture has a non-effect occurrence")
            .static_origin;
        assert!(
            plan.synthesized_root_alternative_population(non_effect, &err)
                .is_err(),
            "a seat that is not a host effect cannot answer, and must not read \
             as a root with no planned set"
        );
        assert!(
            plan.synthesized_root_alternative_population(
                StaticOriginId(u32::MAX),
                &err
            )
            .is_err(),
            "an origin outside the occurrence population must fail, not resolve \
             to an absence"
        );
        assert!(
            plan.synthesized_root_alternative_population(seat, &err.alternative(12))
                .is_err(),
            "a path that leaves the tree must fail, not resolve to an absence"
        );
        let inventory = plan.semantic.synthesized_io_error_roles().len();
        assert!(
            plan.synthesized_root_alternative_population(
                seat,
                &err.alternative(0).field(0).alternative(
                    u32::try_from(inventory).expect("the inventory fits")
                )
            )
            .is_err(),
            "an IOError position outside the closed inventory must fail, not \
             resolve to an absence"
        );
    }
    #[test]
    fn substrate_same_shape_aggregates_keep_distinct_lifetimes() {
        let wrapper = |child| RuntimeExpr::Construct {
            constructor: "ctor:fixture::Substrate::Wrapper".to_string(),
            args: vec![child],
        };
        let expr = RuntimeExpr::Record {
            fields: vec![
                ("persistent".to_string(), wrapper(unit())),
                (
                    "activation".to_string(),
                    wrapper(RuntimeExpr::LexicalClosure {
                        captures: Vec::new(),
                        params: vec!["x".to_string()],
                        body: Box::new(unit()),
                    }),
                ),
            ],
        };
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).expect("plans");
        let wrappers = plan
            .occurrence_authorities
            .iter()
            .filter(|record| {
                matches!(
                    plan.planned_occurrence_expr(record.origin),
                    Ok(RuntimeExpr::Construct { constructor, .. })
                        if constructor == "ctor:fixture::Substrate::Wrapper"
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(wrappers.len(), 2);
        assert_ne!(wrappers[0].origin, wrappers[1].origin);
        assert_eq!(
            plan.constructor_symbol_identity(wrappers[0].origin)
                .unwrap(),
            plan.constructor_symbol_identity(wrappers[1].origin)
                .unwrap()
        );
        assert_eq!(
            wrappers
                .iter()
                .map(|record| record.lifetime)
                .collect::<BTreeSet<_>>(),
            BTreeSet::from([
                PlannedReferentLifetime::Persistent,
                PlannedReferentLifetime::ActivationOwned,
            ])
        );
    }













    /// Every operation whose tree this module states.
    pub(super) fn measured_tree_operations() -> Vec<ken_host::HostOpV1> {
        use ken_host::HostOpV1 as Op;
        vec![
            Op::ConsoleIsTerminal,
            Op::ConsoleWrite,
            Op::ConsoleFlush,
            Op::FsReadFile,
            Op::FsOpen,
            Op::FsWriteFile,
            Op::FsChangeMode,
            Op::BufferAllocate,
            Op::BufferFreeze,
            Op::FsHandleMetadata,
            Op::ResourceRelease,
            Op::FsReadAt,
            Op::FsWriteAt,
        ]
    }

    /// Every node in both of an operation's trees, in no particular order.
    pub(super) fn every_tree_node(operation: ken_host::HostOpV1) -> Vec<SynthesizedAggregateNode> {
        let tree = host_effect_recipe_tree(operation);
        let mut nodes = Vec::new();
        for root in [
            SynthesizedAggregateRoot::HostResultError,
            SynthesizedAggregateRoot::HostResultOk,
        ] {
            walk_tree_with_paths(
                tree.node(root),
                &SynthesizedAggregatePath::root(root),
                &mut |node, _| nodes.push(node),
            );
        }
        nodes
    }

    /// Visit every node of a tree with the path that reaches it.
    pub(super) fn walk_tree_with_paths(
        node: SynthesizedAggregateNode,
        path: &SynthesizedAggregatePath,
        visit: &mut dyn FnMut(SynthesizedAggregateNode, &SynthesizedAggregatePath),
    ) {
        visit(node, path);
        match node {
            SynthesizedAggregateNode::Fixed { children, .. } => {
                for (position, child) in children.iter().enumerate() {
                    walk_tree_with_paths(*child, &path.field(position as u32), visit);
                }
            }
            SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::Alternatives(
                alternatives,
            )) => {
                for (position, alternative) in alternatives.iter().enumerate() {
                    walk_tree_with_paths(
                        *alternative,
                        &path.alternative(position as u32),
                        visit,
                    );
                }
            }
            SynthesizedAggregateNode::Dynamic(SynthesizedDynamicSet::IoErrors)
            | SynthesizedAggregateNode::Scalar { .. }
            | SynthesizedAggregateNode::SiteOperand(_)
            | SynthesizedAggregateNode::Absent => {}
        }
    }
}
