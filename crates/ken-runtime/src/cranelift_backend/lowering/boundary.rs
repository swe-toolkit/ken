//! Values-and-boundary vocabulary — the disposition/classification and
//! lifecycle-phase machinery for a lowered value crossing a boundary.
//!
//! `RT-LOWERING-VALUES-BOUNDARY-SPLIT` `D1`. This module holds the
//! facade-portable half of the values-boundary domain: `pub(in
//! crate::cranelift_backend)` on every moved type AND its fields/variants
//! throughout, so relocating it here from `lowering/mod.rs` changes nothing
//! for any caller anywhere in the facade — SCC and test-tree included. The
//! carrier-emission half (`transfer_into_carrier`, the `emit_carrier_*`
//! family, `BoundaryCarrierRefs`) stays in `mod.rs`, pinned to the LCA by
//! direct consumption from the indivisible SCC in `core.rs` and by
//! not-yet-relocated `core/tests/*` construction (`RT-LOWERING-FUNCTION-STATE-
//! SPLIT`'s and this node's own `D0`).
//!
//! `Lowered` and `LoweringOperand` themselves stay declared in `mod.rs`
//! (SCC-pinned); only their facade-qualified methods move here, matching
//! `Lowered`'s bare-private variants via descendant-of-`mod.rs` visibility —
//! zero widening, the item-10 hub-stays/methods-move shape.

use super::*;

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::cranelift_backend) enum BoundaryTransferInvokingSite {
    /// A transfer invoked from a site not yet given a narrower diagnostic tag.
    Direct,
    /// [`Lowering::carry_call_input`], the ordinary generated-unit input route.
    GeneratedUnitCallInput {
        caller: GeneratedUnitCallInputCaller,
        callee: GeneratedUnitCallInputCallee,
    },
}

/// ⛔ **The `Lowered` variant TAG, without a value.**
///
/// `D4`'s policies are claims about a **whole variant**, never about a sampled
/// value — the frame says so in as many words, because assigning *immediate-only*
/// to a variant that has a spill arm is the vacuity route `AC-10` exists to
/// close. A disposition that takes `&Lowered` cannot be swept over the variants
/// without constructing 21 values, and a control that samples one value per
/// variant would be asserting the variant-level claim from value-level evidence.
///
/// ⭐ So the disposition is a function of **this** — the tag alone — and the tag
/// set is enumerable. `Lowered::variant` and
/// `LoweredVariant::boundary_disposition` are both `match`es with **no `_`
/// arm**, so a 23rd `Lowered` variant is a compile error in both.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum LoweredVariant {
    Int,
    Bool,
    ProcessExitStatus,
    CapabilityToken,
    ResourceToken,
    BoundedNat,
    StructuralNat,
    ResponseBytes,
    HostResult,
    StaticResponseDeferred,
    DynamicConstructor,
    Bytes,
    BorrowedNativeValue,
    BorrowedOption,
    String,
    Constructor,
    Record,
    Closure,
    DeclarationClosure,
    ComputationalRecursorClosure,
    RecursiveBackedge,
    Trap,
}

impl LoweredVariant {
    /// Every variant, in declaration order.
    pub(in crate::cranelift_backend) const ALL: [LoweredVariant; 22] = [
        LoweredVariant::Int,
        LoweredVariant::Bool,
        LoweredVariant::ProcessExitStatus,
        LoweredVariant::CapabilityToken,
        LoweredVariant::ResourceToken,
        LoweredVariant::BoundedNat,
        LoweredVariant::StructuralNat,
        LoweredVariant::ResponseBytes,
        LoweredVariant::HostResult,
        LoweredVariant::StaticResponseDeferred,
        LoweredVariant::DynamicConstructor,
        LoweredVariant::Bytes,
        LoweredVariant::BorrowedNativeValue,
        LoweredVariant::BorrowedOption,
        LoweredVariant::String,
        LoweredVariant::Constructor,
        LoweredVariant::Record,
        LoweredVariant::Closure,
        LoweredVariant::DeclarationClosure,
        LoweredVariant::ComputationalRecursorClosure,
        LoweredVariant::RecursiveBackedge,
        LoweredVariant::Trap,
    ];
}

impl Lowered {
    /// This value's variant tag. ⛔ Exhaustive, no `_` arm.
    pub(in crate::cranelift_backend) fn variant(&self) -> LoweredVariant {
        match self {
            Lowered::Int { .. } => LoweredVariant::Int,
            Lowered::Bool { .. } => LoweredVariant::Bool,
            Lowered::ProcessExitStatus { .. } => LoweredVariant::ProcessExitStatus,
            Lowered::CapabilityToken { .. } => LoweredVariant::CapabilityToken,
            Lowered::ResourceToken { .. } => LoweredVariant::ResourceToken,
            Lowered::BoundedNat(_) => LoweredVariant::BoundedNat,
            Lowered::StructuralNat(_) => LoweredVariant::StructuralNat,
            Lowered::ResponseBytes { .. } => LoweredVariant::ResponseBytes,
            Lowered::HostResult { .. } => LoweredVariant::HostResult,
            Lowered::StaticResponseDeferred => LoweredVariant::StaticResponseDeferred,
            Lowered::DynamicConstructor(_) => LoweredVariant::DynamicConstructor,
            Lowered::Bytes(_) => LoweredVariant::Bytes,
            Lowered::BorrowedNativeValue { .. } => LoweredVariant::BorrowedNativeValue,
            Lowered::BorrowedOption { .. } => LoweredVariant::BorrowedOption,
            Lowered::String(_) => LoweredVariant::String,
            Lowered::Constructor { .. } => LoweredVariant::Constructor,
            Lowered::Record { .. } => LoweredVariant::Record,
            Lowered::Closure { .. } => LoweredVariant::Closure,
            Lowered::DeclarationClosure { .. } => LoweredVariant::DeclarationClosure,
            Lowered::ComputationalRecursorClosure { .. } => {
                LoweredVariant::ComputationalRecursorClosure
            }
            Lowered::RecursiveBackedge => LoweredVariant::RecursiveBackedge,
            Lowered::Trap(_) => LoweredVariant::Trap,
        }
    }
}

/// `RT-FNSPLIT-B2V` `D4` — the five STATIC ENCODING POLICIES, as a closed set.
///
/// ⛔ **Five policies, and the type says five.** They were previously readable
/// only by inspecting a `BoundaryDisposition`: *immediate-only* and
/// *immediate-with-declared-handle-spill* are the same constructor distinguished
/// by an `Option` field, so "every variant has exactly one of five" was a
/// **reading** of the type rather than a fact about it. `AC-3` requires the
/// assignment, and a claim a type cannot express is a claim a control has to
/// restate — which is how the misassignment it names would survive.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum StaticEncodingPolicy {
    /// Every value encodes in the tagged word; **no spill arm exists**.
    ImmediateOnly,
    /// Every value encodes as an opaque handle, with explicit lifetime and
    /// referent owner.
    HandleOnly,
    /// Values encode immediate **or**, on a declared closed condition, as a
    /// handle carrying the same lifetime/referent-owner obligations.
    ImmediateWithDeclaredHandleSpill,
    /// Never a source value at a boundary.
    ProtocolOnly,
    /// Rejected before emission, with an exact error.
    FailClosedForbidden,
}

impl StaticEncodingPolicy {
    /// Every policy, in the frame's order.
    pub(in crate::cranelift_backend) const ALL: [StaticEncodingPolicy; 5] = [
        StaticEncodingPolicy::ImmediateOnly,
        StaticEncodingPolicy::HandleOnly,
        StaticEncodingPolicy::ImmediateWithDeclaredHandleSpill,
        StaticEncodingPolicy::ProtocolOnly,
        StaticEncodingPolicy::FailClosedForbidden,
    ];
}

impl BoundaryDisposition {
    /// The static encoding policy this disposition declares.
    ///
    /// ⛔ **A declared spill is the THIRD policy, not the first.**
    /// `RepresentedImmediate { spill: Some(_) }` claims that a value encodes
    /// immediate *or* spills to a handle on a declared condition — it does not
    /// claim every value of the variant is immediate, and calling it
    /// *immediate-only* would let a proof attach handle evidence to one sampled
    /// spill while never establishing the handle obligations for the partition.
    pub(in crate::cranelift_backend) fn policy(self) -> StaticEncodingPolicy {
        match self {
            BoundaryDisposition::RepresentedImmediate { spill: None, .. } => {
                StaticEncodingPolicy::ImmediateOnly
            }
            BoundaryDisposition::RepresentedImmediate { spill: Some(_), .. } => {
                StaticEncodingPolicy::ImmediateWithDeclaredHandleSpill
            }
            BoundaryDisposition::RepresentedHandle { .. } => StaticEncodingPolicy::HandleOnly,
            BoundaryDisposition::ProtocolOnly { .. } => StaticEncodingPolicy::ProtocolOnly,
            BoundaryDisposition::FailClosedForbidden { .. } => {
                StaticEncodingPolicy::FailClosedForbidden
            }
        }
    }
}

// ---------------------------------------------------------------------------
// `AC-10` — total classified-domain closure
// ---------------------------------------------------------------------------
//
// ⛔ **"One control total over every value" is not an executable oracle**, and
// the frame says so: the admitted domains include unbounded integers, arbitrary
// byte contents, ownership states, and recursive parent → child reachability. A
// finite runtime sweep dressed as a universal claim is worse than an honest
// sweep, because it reads as total.
//
// ⭐ **So totality is proved STRUCTURALLY, in two layers.** The sealed
// wildcard-free disposition closes the *variant* layer. Below it, every
// **value-dependent representation discriminator** is a closed finite partition,
// and the classifier is a total function from a cell of that product to exactly
// one actual outcome. A value reaches its cell through a *total* projection
// (`int_fits_immediate`, `referent_owner`, "does this aggregate hold an
// invocation-owned child") — so the infinite domain is covered by construction
// rather than by enumeration, and only the finitely many CELLS need controls.

/// Magnitude / shape — the discriminator an immediate-with-spill policy names.
///
/// The projection from a value is total: `BoundaryWord::int_fits_immediate`
/// answers for every `i64`, and there is no third answer.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum MagnitudePartition {
    /// The payload encodes in the tagged word's 56-bit field.
    WithinImmediateField,
    /// The payload does not, so a declared spill arm must carry it.
    BeyondImmediateField,
}

/// Parent → child reachability — the discriminator that decides whether an
/// aggregate can be represented at all.
///
/// ⛔ **Total over nodes is not closed under parent → child reachability**, which
/// is why this is its own partition rather than a property of the parent's
/// variant: a persistent aggregate holding an invocation-owned child is a
/// surviving parent naming storage that dies first.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum ReachabilityPartition {
    /// No children — nothing to reach.
    Leaf,
    /// Every reachable child outlives the parent.
    ChildrenOutliveParent,
    /// Some reachable child dies before the parent.
    ChildDiesBeforeParent,
}

/// Whether a handle's referent carries the store's identity of record.
///
/// ⛔ **`NoStoreIdentity` is NOT a valid outcome for a persistent handle**, and
/// classifying it as one was the defect the Architect ruled on. A consumer can
/// recover the *absence* of an identity; it cannot thereby recover the same
/// identity **intact**. Worse, this ABI's own node contract says a null
/// `NODE_SLOT` denotes *invocation-arena* ownership — so a word claiming
/// `PersistentStore` over a null slot contradicts the layout it is written in.
/// Reserving persistent-region storage is storage governance, never adoption.
///
/// ⭐ An emitted-constructed persistent node is therefore a **pending** internal
/// state, not a published outcome: [`crate::boundary_value::BoundaryValueStore::adopt`]
/// validates the reachable graph, interns it, and mints or reuses the real
/// `SlotId` before the word can escape. `NoStoreIdentity` remains correct for an
/// **invocation** handle, where there is no store identity to have.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum HandleIdentity {
    /// The store minted or reused the referent's `SlotId` and the node names it.
    StoreMinted,
    /// An invocation-owned referent, which has no store identity by design.
    NoStoreIdentity,
}

/// Whether a persistent node has passed the store-owned adoption boundary.
///
/// ⛔ A closed partition, and the one that decides whether a persistent handle is
/// **published at all**. Emitted construction alone leaves
/// `PendingStoreAdoption`; only the store's `adopt` moves a node to
/// `StoreAdopted`, and the emitted escape gate refuses to let a pending word
/// cross a generated-function boundary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum AdoptionPartition {
    /// The store has minted or reused this referent's `SlotId`.
    StoreAdopted,
    /// Constructed and sealed by emitted code, but not adopted by the store.
    PendingStoreAdoption,
}

/// The **actual outcome** a boundary input receives — the closed set `AC-10`
/// quantifies over.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum BoundaryOutcome {
    /// The value rides in the tagged word.
    ///
    /// ⛔ `value_class` is what the uniform `class` helper must report for such
    /// a word — a *boundary-value* classification, deliberately NOT a node
    /// class (an immediate has no node). See
    /// [`BoundaryTag::immediate_value_class`].
    ImmediateWord {
        tag: BoundaryTag,
        value_class: Option<BoundaryClass>,
    },
    /// A handle, with every obligation the frame names discharged: class,
    /// referent owner, identity, and lifetime (the owner *is* the lifetime).
    HandleWord {
        tag: BoundaryTag,
        class: BoundaryClass,
        owner: BoundaryReferentOwner,
        identity: HandleIdentity,
    },
    /// Never a source value at a boundary.
    ProtocolOnly,
    /// Rejected before emission or publication, with an exact status.
    FailClosedForbidden,
}

/// One cell of the closed discriminator product — a boundary **input**, reduced
/// to the finitely many things its representation can depend on.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct BoundaryInput {
    pub(in crate::cranelift_backend) variant: LoweredVariant,
    pub(in crate::cranelift_backend) magnitude: MagnitudePartition,
    pub(in crate::cranelift_backend) reachability: ReachabilityPartition,
    pub(in crate::cranelift_backend) adoption: AdoptionPartition,
}

impl BoundaryInput {
    /// Every cell of the product, in a fixed order.
    pub(in crate::cranelift_backend) fn all() -> Vec<BoundaryInput> {
        let mut cells = Vec::new();
        for variant in LoweredVariant::ALL {
            for magnitude in [
                MagnitudePartition::WithinImmediateField,
                MagnitudePartition::BeyondImmediateField,
            ] {
                for reachability in [
                    ReachabilityPartition::Leaf,
                    ReachabilityPartition::ChildrenOutliveParent,
                    ReachabilityPartition::ChildDiesBeforeParent,
                ] {
                    for adoption in [
                        AdoptionPartition::StoreAdopted,
                        AdoptionPartition::PendingStoreAdoption,
                    ] {
                        cells.push(BoundaryInput {
                            variant,
                            magnitude,
                            reachability,
                            adoption,
                        });
                    }
                }
            }
        }
        cells
    }

    /// The actual outcome this input receives.
    ///
    /// ⛔ **Classification happens FIRST and the behaviour is entailed by the
    /// class.** The failure arm belongs to the *unrepresentable* class, never
    /// inside the admitted one — a predicate reading *"either round-trip or fail
    /// closed"* over the admitted set is satisfied vacuously by an
    /// implementation that rejects everything.
    ///
    /// ⛔ **No `_` arm anywhere below**, so a new variant, a new policy, or a new
    /// partition value is a compile error rather than a silent default.
    pub(in crate::cranelift_backend) fn outcome(self) -> BoundaryOutcome {
        let disposition = self.variant.boundary_disposition();
        match disposition {
            BoundaryDisposition::ProtocolOnly { .. } => BoundaryOutcome::ProtocolOnly,
            BoundaryDisposition::FailClosedForbidden { .. } => BoundaryOutcome::FailClosedForbidden,
            BoundaryDisposition::RepresentedImmediate { tag, spill } => {
                match (spill, self.magnitude) {
                    // Immediate-only: the outcome does not depend on magnitude,
                    // and that constancy is asserted rather than assumed.
                    (None, MagnitudePartition::WithinImmediateField)
                    | (None, MagnitudePartition::BeyondImmediateField) => {
                        BoundaryOutcome::ImmediateWord {
                            tag,
                            value_class: tag.immediate_value_class(),
                        }
                    }
                    (Some(_), MagnitudePartition::WithinImmediateField) => {
                        BoundaryOutcome::ImmediateWord {
                            tag,
                            value_class: tag.immediate_value_class(),
                        }
                    }
                    // ⛔ **The SPILL ARM is a handle outcome**, so it discharges the
                    // same class / owner / identity / lifetime obligations as
                    // handle-only. This is the arm the frame says a proof may not
                    // attach to one sampled value.
                    // ⛔ The spill arm is a PERSISTENT handle, so it publishes
                    // only once the store owns its identity.
                    (Some(class), MagnitudePartition::BeyondImmediateField) => {
                        match self.adoption {
                            AdoptionPartition::PendingStoreAdoption => {
                                BoundaryOutcome::FailClosedForbidden
                            }
                            AdoptionPartition::StoreAdopted => BoundaryOutcome::HandleWord {
                                tag: BoundaryTag::PersistentGround,
                                class,
                                owner: BoundaryReferentOwner::PersistentStore,
                                identity: Self::handle_identity(
                                    BoundaryReferentOwner::PersistentStore,
                                ),
                            },
                        }
                    }
                }
            }
            BoundaryDisposition::RepresentedHandle { tag, class } => {
                let owner = tag.referent_owner();
                match (owner, self.reachability) {
                    // ⭐⭐ **`RT-DECL-CLOSURE-PORT` `D7` — the aggregate lifetime
                    // MEET.** A `Constructor`/`Record` whose value shape is
                    // persistable but which has a child that dies first is not
                    // an error; it is an aggregate whose lifetime is the
                    // invocation. The parent takes
                    // [`BoundaryTag::InvocationAggregate`] and the whole edge
                    // becomes sound — an invocation-owned parent naming an
                    // invocation-owned child dangles nothing.
                    //
                    // ⛔ This is NOT a relaxation of the escape rule. It is the
                    // missing lane the rule was standing in for: the refusal
                    // below still fires for every non-aggregate shape, and this
                    // arm's own referent owner is the arena, so every escape
                    // check downstream governs it unchanged.
                    //
                    // ⚠ Keyed on the CLASS, not on the tag, and deliberately:
                    // the incoming `tag` is whatever the value-shape
                    // disposition reached for, and the question here is whether
                    // this shape has children to take a meet over. `Bytes`,
                    // `String` and `Int` do not, so they keep the refusal.
                    (
                        BoundaryReferentOwner::PersistentStore,
                        ReachabilityPartition::ChildDiesBeforeParent,
                    ) if matches!(
                        class,
                        BoundaryClass::Constructor | BoundaryClass::Record
                    ) =>
                    {
                        BoundaryOutcome::HandleWord {
                            tag: BoundaryTag::InvocationAggregate,
                            class,
                            owner: BoundaryReferentOwner::InvocationArena,
                            identity: Self::handle_identity(
                                BoundaryReferentOwner::InvocationArena,
                            ),
                        }
                    }
                    // ⛔ A surviving parent may not name storage that dies
                    // first. Rejected before publication, with `ERR_ESCAPE`.
                    (
                        BoundaryReferentOwner::PersistentStore,
                        ReachabilityPartition::ChildDiesBeforeParent,
                    ) => BoundaryOutcome::FailClosedForbidden,
                    // ⛔ A persistent handle publishes only after the store has
                    // adopted it. Until then the node carries `NULL_SLOT`, which
                    // this ABI reads as invocation ownership — a word claiming
                    // otherwise contradicts its own layout.
                    (BoundaryReferentOwner::PersistentStore, ReachabilityPartition::Leaf)
                    | (
                        BoundaryReferentOwner::PersistentStore,
                        ReachabilityPartition::ChildrenOutliveParent,
                    ) => match self.adoption {
                        AdoptionPartition::PendingStoreAdoption => {
                            BoundaryOutcome::FailClosedForbidden
                        }
                        AdoptionPartition::StoreAdopted => BoundaryOutcome::HandleWord {
                            tag,
                            class,
                            owner,
                            identity: Self::handle_identity(owner),
                        },
                    },
                    // An invocation handle has no store identity to have, and
                    // adoption is not its boundary.
                    (BoundaryReferentOwner::InvocationArena, ReachabilityPartition::Leaf)
                    | (
                        BoundaryReferentOwner::InvocationArena,
                        ReachabilityPartition::ChildrenOutliveParent,
                    )
                    | (
                        BoundaryReferentOwner::InvocationArena,
                        ReachabilityPartition::ChildDiesBeforeParent,
                    ) => BoundaryOutcome::HandleWord {
                        tag,
                        class,
                        owner,
                        identity: Self::handle_identity(owner),
                    },
                    // A handle whose referent nothing owns is not representable.
                    (BoundaryReferentOwner::NoReferent, ReachabilityPartition::Leaf)
                    | (
                        BoundaryReferentOwner::NoReferent,
                        ReachabilityPartition::ChildrenOutliveParent,
                    )
                    | (
                        BoundaryReferentOwner::NoReferent,
                        ReachabilityPartition::ChildDiesBeforeParent,
                    ) => BoundaryOutcome::FailClosedForbidden,
                }
            }
        }
    }

    /// The identity a **published** handle of this owner carries.
    ///
    /// ⛔ A persistent handle is only ever published `StoreMinted` — a pending
    /// one is not a handle outcome at all, and `outcome` routes it to
    /// `FailClosedForbidden` before reaching here.
    fn handle_identity(owner: BoundaryReferentOwner) -> HandleIdentity {
        match owner {
            BoundaryReferentOwner::PersistentStore => HandleIdentity::StoreMinted,
            BoundaryReferentOwner::InvocationArena | BoundaryReferentOwner::NoReferent => {
                HandleIdentity::NoStoreIdentity
            }
        }
    }
}

impl BoundaryOutcome {
    /// Whether this outcome is one the static policy permits.
    ///
    /// ⛔ The entailment `AC-10` requires: the outcome is not merely *some*
    /// classification, it is one the variant's declared policy allows. An
    /// immediate-only policy yielding a handle is the misassignment the frame
    /// names, seen from the value level.
    pub(in crate::cranelift_backend) fn permitted_by(self, policy: StaticEncodingPolicy) -> bool {
        match (policy, self) {
            (StaticEncodingPolicy::ImmediateOnly, BoundaryOutcome::ImmediateWord { .. }) => true,
            (StaticEncodingPolicy::HandleOnly, BoundaryOutcome::HandleWord { .. })
            | (StaticEncodingPolicy::HandleOnly, BoundaryOutcome::FailClosedForbidden) => true,
            (
                StaticEncodingPolicy::ImmediateWithDeclaredHandleSpill,
                BoundaryOutcome::ImmediateWord { .. },
            )
            | (
                StaticEncodingPolicy::ImmediateWithDeclaredHandleSpill,
                BoundaryOutcome::HandleWord { .. },
            )
            // ⛔ A spill arm is a PERSISTENT handle, so an unadopted one fails
            // closed before publication. That is an *unrepresentable-input*
            // outcome, not admission of a represented value — the vacuity guard
            // is that all four outcomes stay inhabited and that magnitude still
            // changes this policy's outcome.
            | (
                StaticEncodingPolicy::ImmediateWithDeclaredHandleSpill,
                BoundaryOutcome::FailClosedForbidden,
            ) => true,
            (StaticEncodingPolicy::ProtocolOnly, BoundaryOutcome::ProtocolOnly) => true,
            (StaticEncodingPolicy::FailClosedForbidden, BoundaryOutcome::FailClosedForbidden) => {
                true
            }
            (StaticEncodingPolicy::ImmediateOnly, BoundaryOutcome::HandleWord { .. })
            | (StaticEncodingPolicy::ImmediateOnly, BoundaryOutcome::ProtocolOnly)
            | (StaticEncodingPolicy::ImmediateOnly, BoundaryOutcome::FailClosedForbidden)
            | (StaticEncodingPolicy::HandleOnly, BoundaryOutcome::ImmediateWord { .. })
            | (StaticEncodingPolicy::HandleOnly, BoundaryOutcome::ProtocolOnly)
            | (
                StaticEncodingPolicy::ImmediateWithDeclaredHandleSpill,
                BoundaryOutcome::ProtocolOnly,
            )
            | (StaticEncodingPolicy::ProtocolOnly, BoundaryOutcome::ImmediateWord { .. })
            | (StaticEncodingPolicy::ProtocolOnly, BoundaryOutcome::HandleWord { .. })
            | (StaticEncodingPolicy::ProtocolOnly, BoundaryOutcome::FailClosedForbidden)
            | (StaticEncodingPolicy::FailClosedForbidden, BoundaryOutcome::ImmediateWord { .. })
            | (StaticEncodingPolicy::FailClosedForbidden, BoundaryOutcome::HandleWord { .. })
            | (StaticEncodingPolicy::FailClosedForbidden, BoundaryOutcome::ProtocolOnly) => false,
        }
    }
}

// ---------------------------------------------------------------------------
// `RECUT 2` — representation authority-to-execution closure
// ---------------------------------------------------------------------------
//
// ⛔ **`AC-10` closes the CLASSIFICATION; this closes the EXECUTION.** The
// partition above proves every boundary input reaches exactly one outcome
// permitted by its variant's static policy. It does **not** ask whether the
// outcome's lifecycle is executable end to end — and that is the predicate the
// Architect named across blocks `#1`–`#6`: *every representation authority must
// be the sole authority actually consumed by the production path it governs,
// and every admitted partition must have one total executable lifecycle.*
//
// ⭐ **Why this is a type and not a table.** The proof shape RECUT 2 retires is
// a hand-maintained matrix that can drift from the production enums. So the row
// set here is not written down: it is **derived** by iterating
// [`BoundaryInput::all`] and classifying, and the phases are **struct fields
// with no default**. A row that cannot say what closes a phase does not
// compile, and a new outcome or a new phase is a compile error rather than a
// row nobody added.
//
// ⚠ **The honest boundary, stated once here rather than implied:** the
// compiler closes *completeness* — that every required phase is bound. It does
// **not** close *identity* — that the bound anchor is the real production item
// rather than a lookalike. Identity is closed by named causal controls, and
// [`ProductionAnchor::derived_witness`] exists so that the subset of anchors
// which can be evaluated without a JIT are checked against production values
// rather than against their own spelling.

/// One phase of the lifecycle `RECUT 2` requires every admitted row to close.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum LifecyclePhase {
    /// The authority that decides the representation.
    Authority,
    /// Emitted code that constructs a word of this outcome.
    Producer,
    /// The check that rejects a malformed or unrepresentable input.
    Validator,
    /// Canonicalization and store adoption / identity mint.
    CanonicalizerAdopter,
    /// The step that makes the word visible past the producer.
    Publisher,
    /// A separately compiled reader that recovers the value.
    Consumer,
}

impl LifecyclePhase {
    /// Every phase, in lifecycle order.
    ///
    /// ⛔ Bound to the enum by [`LifecyclePhase::index`]'s wildcard-free match,
    /// so a seventh phase cannot be added without extending this array.
    pub(in crate::cranelift_backend) const ALL: [LifecyclePhase; 6] = [
        LifecyclePhase::Authority,
        LifecyclePhase::Producer,
        LifecyclePhase::Validator,
        LifecyclePhase::CanonicalizerAdopter,
        LifecyclePhase::Publisher,
        LifecyclePhase::Consumer,
    ];

    /// This phase's position in [`LifecyclePhase::ALL`].
    ///
    /// ⛔ **This is the pin that binds `ALL`'s length to the type.** A seventh
    /// variant is a non-exhaustive-match compile error here, and the control
    /// `recut2_the_phase_inventory_is_bound_to_the_type` checks that every
    /// index round-trips through `ALL` — so `ALL` cannot silently omit one.
    pub(in crate::cranelift_backend) fn index(self) -> usize {
        match self {
            LifecyclePhase::Authority => 0,
            LifecyclePhase::Producer => 1,
            LifecyclePhase::Validator => 2,
            LifecyclePhase::CanonicalizerAdopter => 3,
            LifecyclePhase::Publisher => 4,
            LifecyclePhase::Consumer => 5,
        }
    }
}

/// The production item that closes a phase.
///
/// ⛔ **Every variant names a real item on the production path**, not a
/// description of one. The `derived_witness` below is what keeps that honest
/// for the anchors it can reach: it returns a value **computed by** the named
/// authority, so deleting or rewiring the authority changes the witness.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum ProductionAnchor {
    /// `boundary_value::NodeField` / `RegionHeaderField` — the sole layout
    /// authority, from which every extent is derived (`AC-1` layout closure).
    LayoutFieldInventory,
    /// `boundary_value::boundary_int_magnitude_is_canonical` — the canonical
    /// sign/limb contract, authoritative wherever a word is built.
    IntNormalizationAuthority,
    /// `boundary_value_clif::emit_boundary_value_local_graph` — the emitted
    /// producer helpers.
    EmittedProducerGraph,
    /// The emitted escape gate refusing a pending or invocation-owned word.
    EmittedEscapeGate,
    /// `BoundaryValueStore::adopt`'s iterative tri-colour reachable-graph walk.
    ReachableGraphValidator,
    /// `BoundaryValueStore::adopt` — postorder canonicalization and the
    /// store-only identity mint.
    StoreAdoption,
    /// `BoundaryArenaBuilder::publish` / `BoundaryValueStore::publish_persistent`.
    RegionPublication,
    /// `boundary_value_clif::capture_boundary_value_local_graph` — a separately
    /// compiled consumer.
    SeparatelyCompiledConsumer,
}

impl ProductionAnchor {
    /// A value **computed by the named production item**, where that is
    /// possible without a JIT module.
    ///
    /// ⛔ **`None` is not a waiver and not a residual** — it says this anchor's
    /// identity is closed by a named causal control instead, and
    /// `recut2_every_anchor_is_closed_by_a_witness_or_a_named_control` requires
    /// each `None` anchor to appear in [`ProductionAnchor::CONTROL_CLOSED`].
    /// Making "cannot determine" a third outcome that must be *accounted for*,
    /// rather than one that falls through to pass, is the point.
    pub(in crate::cranelift_backend) fn derived_witness(self) -> Option<i64> {
        match self {
            // Derived from the field inventory: if a field is added, removed or
            // reordered, this value moves.
            ProductionAnchor::LayoutFieldInventory => {
                Some(crate::boundary_value::NODE_EXTENT as i64)
            }
            // Computed by calling the normalization authority on a magnitude it
            // must reject — a leading-zero limb is non-canonical by contract.
            ProductionAnchor::IntNormalizationAuthority => Some(i64::from(
                !crate::boundary_value::boundary_int_magnitude_is_canonical(0, &[1, 0]),
            )),
            // Derived from the escape gate's exact status constant.
            ProductionAnchor::EmittedEscapeGate => Some(crate::boundary_value::BOUNDARY_ERR_ESCAPE),
            // Derived from the validator's exact malformed-shape status.
            ProductionAnchor::ReachableGraphValidator => {
                Some(crate::boundary_value::BOUNDARY_ERR_CYCLE)
            }
            // Derived from the seal/quiescence handoff's exact status.
            ProductionAnchor::RegionPublication => Some(crate::boundary_value::BOUNDARY_ERR_SEALED),
            // ⛔ These three need a live JIT module to evaluate, so their
            // identity is control-closed rather than witness-closed.
            ProductionAnchor::EmittedProducerGraph
            | ProductionAnchor::StoreAdoption
            | ProductionAnchor::SeparatelyCompiledConsumer => None,
        }
    }

    /// The anchors whose identity is closed by a named causal control rather
    /// than by a derived witness, each paired with that control.
    ///
    /// ⛔ **This list is the residual given a cell**, in the frame's sense: it
    /// records what is control-enforced instead of letting the absence read as
    /// enforcement.
    pub(in crate::cranelift_backend) const CONTROL_CLOSED: &'static [(
        ProductionAnchor,
        &'static str,
    )] = &[
        (
            ProductionAnchor::EmittedProducerGraph,
            "b2v_the_helper_inventory_is_closed_and_named",
        ),
        (
            ProductionAnchor::StoreAdoption,
            "b2v_adoption_mints_a_real_slot_and_equal_values_converge",
        ),
        (
            ProductionAnchor::SeparatelyCompiledConsumer,
            "b2v_a_separately_compiled_consumer_recovers_the_value",
        ),
    ];
}

/// How one phase is closed for one row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum PhaseBinding {
    /// Closed by a named production item.
    Closed(ProductionAnchor),
    /// The outcome class structurally has no such phase.
    ///
    /// ⛔ **Derived from the outcome by [`BoundaryOutcome::requires`], never
    /// chosen per row.** If a row could declare a phase absent on its own
    /// authority, this enum would be the drift-prone matrix again with an
    /// escape hatch — every uncomfortable cell would become `StructurallyAbsent`
    /// and the artifact would close vacuously.
    StructurallyAbsent,
}

/// One row of the closure artifact — all six phases, none optional.
///
/// ⛔ **There is no `Default` and no `Option`.** Omitting a field is a
/// missing-field compile error, which is RECUT 2's *"a missing lifecycle phase
/// must be a construction failure"* discharged by construction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct PhaseClosure {
    authority: PhaseBinding,
    producer: PhaseBinding,
    validator: PhaseBinding,
    canonicalizer_adopter: PhaseBinding,
    publisher: PhaseBinding,
    consumer: PhaseBinding,
}

impl PhaseClosure {
    /// This row's binding for one phase.
    pub(in crate::cranelift_backend) fn binding(&self, phase: LifecyclePhase) -> PhaseBinding {
        match phase {
            LifecyclePhase::Authority => self.authority,
            LifecyclePhase::Producer => self.producer,
            LifecyclePhase::Validator => self.validator,
            LifecyclePhase::CanonicalizerAdopter => self.canonicalizer_adopter,
            LifecyclePhase::Publisher => self.publisher,
            LifecyclePhase::Consumer => self.consumer,
        }
    }
}

impl BoundaryOutcome {
    /// Whether this outcome's class **requires** a phase.
    ///
    /// ⛔ **Derived from the outcome, so a row cannot excuse itself.** A
    /// `ProtocolOnly` value never reaches a boundary, so it has no producer,
    /// adopter, publisher or consumer — but it still has an authority that says
    /// so and a validator that enforces it. An invocation handle has no
    /// *canonicalizer/adopter* because store adoption must **reject** it
    /// (Ruling B item 6); that absence is a contract, not a gap.
    pub(in crate::cranelift_backend) fn requires(self, phase: LifecyclePhase) -> bool {
        match (self, phase) {
            // An immediate rides in the word: produced, validated, published and
            // read, but never canonicalized or adopted by the store.
            (BoundaryOutcome::ImmediateWord { .. }, LifecyclePhase::Authority)
            | (BoundaryOutcome::ImmediateWord { .. }, LifecyclePhase::Producer)
            | (BoundaryOutcome::ImmediateWord { .. }, LifecyclePhase::Validator)
            | (BoundaryOutcome::ImmediateWord { .. }, LifecyclePhase::Publisher)
            | (BoundaryOutcome::ImmediateWord { .. }, LifecyclePhase::Consumer) => true,
            (BoundaryOutcome::ImmediateWord { .. }, LifecyclePhase::CanonicalizerAdopter) => false,

            // ⛔ A store-minted handle is the only outcome that requires ALL
            // SIX. This is the row the six blocks kept failing.
            (
                BoundaryOutcome::HandleWord {
                    identity: HandleIdentity::StoreMinted,
                    ..
                },
                _,
            ) => true,

            // An invocation handle has no store identity to mint, by design.
            (
                BoundaryOutcome::HandleWord {
                    identity: HandleIdentity::NoStoreIdentity,
                    ..
                },
                LifecyclePhase::CanonicalizerAdopter,
            ) => false,
            (
                BoundaryOutcome::HandleWord {
                    identity: HandleIdentity::NoStoreIdentity,
                    ..
                },
                LifecyclePhase::Authority,
            )
            | (
                BoundaryOutcome::HandleWord {
                    identity: HandleIdentity::NoStoreIdentity,
                    ..
                },
                LifecyclePhase::Producer,
            )
            | (
                BoundaryOutcome::HandleWord {
                    identity: HandleIdentity::NoStoreIdentity,
                    ..
                },
                LifecyclePhase::Validator,
            )
            | (
                BoundaryOutcome::HandleWord {
                    identity: HandleIdentity::NoStoreIdentity,
                    ..
                },
                LifecyclePhase::Publisher,
            )
            | (
                BoundaryOutcome::HandleWord {
                    identity: HandleIdentity::NoStoreIdentity,
                    ..
                },
                LifecyclePhase::Consumer,
            ) => true,

            // Never a source value at a boundary, and rejected before emission:
            // both are closed by an authority plus the validator that enforces
            // it, and neither ever produces, adopts, publishes or is read.
            (BoundaryOutcome::ProtocolOnly, LifecyclePhase::Authority)
            | (BoundaryOutcome::ProtocolOnly, LifecyclePhase::Validator)
            | (BoundaryOutcome::FailClosedForbidden, LifecyclePhase::Authority)
            | (BoundaryOutcome::FailClosedForbidden, LifecyclePhase::Validator) => true,
            (BoundaryOutcome::ProtocolOnly, LifecyclePhase::Producer)
            | (BoundaryOutcome::ProtocolOnly, LifecyclePhase::CanonicalizerAdopter)
            | (BoundaryOutcome::ProtocolOnly, LifecyclePhase::Publisher)
            | (BoundaryOutcome::ProtocolOnly, LifecyclePhase::Consumer)
            | (BoundaryOutcome::FailClosedForbidden, LifecyclePhase::Producer)
            | (BoundaryOutcome::FailClosedForbidden, LifecyclePhase::CanonicalizerAdopter)
            | (BoundaryOutcome::FailClosedForbidden, LifecyclePhase::Publisher)
            | (BoundaryOutcome::FailClosedForbidden, LifecyclePhase::Consumer) => false,
        }
    }

    /// The lifecycle closure for this outcome.
    ///
    /// ⛔ **Wildcard-free over the outcome, and every field is mandatory**, so
    /// a new outcome variant is a compile error and an existing one cannot ship
    /// a hole.
    pub(in crate::cranelift_backend) fn phase_closure(self) -> PhaseClosure {
        match self {
            BoundaryOutcome::ImmediateWord { .. } => PhaseClosure {
                authority: PhaseBinding::Closed(ProductionAnchor::LayoutFieldInventory),
                producer: PhaseBinding::Closed(ProductionAnchor::EmittedProducerGraph),
                validator: PhaseBinding::Closed(ProductionAnchor::IntNormalizationAuthority),
                canonicalizer_adopter: PhaseBinding::StructurallyAbsent,
                publisher: PhaseBinding::Closed(ProductionAnchor::RegionPublication),
                consumer: PhaseBinding::Closed(ProductionAnchor::SeparatelyCompiledConsumer),
            },
            BoundaryOutcome::HandleWord {
                identity: HandleIdentity::StoreMinted,
                ..
            } => PhaseClosure {
                authority: PhaseBinding::Closed(ProductionAnchor::LayoutFieldInventory),
                producer: PhaseBinding::Closed(ProductionAnchor::EmittedProducerGraph),
                validator: PhaseBinding::Closed(ProductionAnchor::ReachableGraphValidator),
                canonicalizer_adopter: PhaseBinding::Closed(ProductionAnchor::StoreAdoption),
                publisher: PhaseBinding::Closed(ProductionAnchor::RegionPublication),
                consumer: PhaseBinding::Closed(ProductionAnchor::SeparatelyCompiledConsumer),
            },
            BoundaryOutcome::HandleWord {
                identity: HandleIdentity::NoStoreIdentity,
                ..
            } => PhaseClosure {
                authority: PhaseBinding::Closed(ProductionAnchor::LayoutFieldInventory),
                producer: PhaseBinding::Closed(ProductionAnchor::EmittedProducerGraph),
                validator: PhaseBinding::Closed(ProductionAnchor::EmittedEscapeGate),
                canonicalizer_adopter: PhaseBinding::StructurallyAbsent,
                publisher: PhaseBinding::Closed(ProductionAnchor::RegionPublication),
                consumer: PhaseBinding::Closed(ProductionAnchor::SeparatelyCompiledConsumer),
            },
            BoundaryOutcome::ProtocolOnly => PhaseClosure {
                authority: PhaseBinding::Closed(ProductionAnchor::LayoutFieldInventory),
                producer: PhaseBinding::StructurallyAbsent,
                validator: PhaseBinding::Closed(ProductionAnchor::EmittedEscapeGate),
                canonicalizer_adopter: PhaseBinding::StructurallyAbsent,
                publisher: PhaseBinding::StructurallyAbsent,
                consumer: PhaseBinding::StructurallyAbsent,
            },
            BoundaryOutcome::FailClosedForbidden => PhaseClosure {
                authority: PhaseBinding::Closed(ProductionAnchor::LayoutFieldInventory),
                producer: PhaseBinding::StructurallyAbsent,
                validator: PhaseBinding::Closed(ProductionAnchor::ReachableGraphValidator),
                canonicalizer_adopter: PhaseBinding::StructurallyAbsent,
                publisher: PhaseBinding::StructurallyAbsent,
                consumer: PhaseBinding::StructurallyAbsent,
            },
        }
    }
}

/// `RT-FNSPLIT-B2V` `D4` — what a `Lowered` becomes when it crosses a boundary.
///
/// ⛔ **The population is closed by the compiler, not by a histogram.** The
/// `#10` evidence measured 41 source-valued transfers and 26-of-154 aggregate
/// root results; those numbers are *corroboration*. The proof is
/// [`Lowered::boundary_disposition`]'s exhaustive, wildcard-free `match` over
/// the 22 landed variants: a 23rd variant is a **compile error** until someone
/// dispositions it, never a silent default into `ValueWord`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum BoundaryDisposition {
    /// The payload rides in the tagged word itself.
    ///
    /// `spill` names the handle class used when a **runtime** magnitude test
    /// finds the payload too wide for the immediate field. ⭐ Spelling the
    /// spill out is the point: without it "represented immediate" would quietly
    /// claim that every `Int` fits 56 bits, which is false for exactly the
    /// values a bignum language exists to carry.
    RepresentedImmediate {
        tag: BoundaryTag,
        spill: Option<BoundaryClass>,
    },
    /// An opaque handle. The **referent** owner is part of the disposition
    /// because it is a different question from who owns the frame slot (`D2`).
    RepresentedHandle {
        tag: BoundaryTag,
        class: BoundaryClass,
    },
    /// Never a source value at a boundary — protocol machinery only.
    ProtocolOnly { why: &'static str },
    /// Rejected **before** emission, with an exact error.
    FailClosedForbidden { why: &'static str },
}

impl Lowered {
    /// The boundary disposition of this value.
    ///
    /// ⛔ **No `_` arm, by construction.** Every variant is named.
    /// The boundary disposition of this value's VARIANT.
    ///
    /// ⛔ A policy is a claim about the whole variant, so it is a function of
    /// the variant TAG and of nothing else — see [`LoweredVariant`]. Delegating
    /// makes that structural: this cannot come to depend on a payload without
    /// someone changing the signature, and the tag set is enumerable, so the
    /// `AC-3` assignment can be swept without constructing 21 values.
    pub(in crate::cranelift_backend) fn boundary_disposition(&self) -> BoundaryDisposition {
        self.variant().boundary_disposition()
    }

    /// `RT-FNSPLIT-C1` `D5` — whether this **whole value graph** may cross the
    /// boundary, decided before anything is allocated, written or published.
    ///
    /// ⭐⭐ **The root variant table is not sufficient, and that is the finding
    /// this walk exists to encode.** `boundary_disposition` is a function of the
    /// root tag alone, so it reports `RepresentedHandle` for a `Constructor`
    /// whose arguments contain a closure. Nothing in the lowering excludes that
    /// shape: `lower_expr`'s `Construct` arm lowers each argument through
    /// `lower_expr` and screens only for `RecursiveBackedge`, so a closure
    /// nested inside a constructor is constructible on the live path.
    ///
    /// ⇒ Admissibility is a property of the **graph**, not of the root.
    ///
    /// ⛔ **Total and wildcard-free by construction.** Every variant is named,
    /// so a 23rd `Lowered` inhabitant is a compile error here as well as in
    /// `variant()` — a new carrier of children cannot be added without deciding
    /// whether it can hide a closure.
    ///
    /// ⚠ **Ordering is load-bearing: this runs BEFORE any allocation, store
    /// write, adoption or publication.** A walk performed after the first child
    /// is published would reject the transfer having already emitted part of
    /// it, which is a partial publication rather than a rejection.
    ///
    /// ⚠ **The completeness cost, stated honestly and in its true size:** this
    /// rejects only graphs that *actually contain* a closure. ⛔ It does **not**
    /// reject the `Constructor` variant, and it does **not** follow that the 29
    /// of 41 measured `Constructor` transfers fail — only those whose actual
    /// argument graph holds a closure do. ⭐ And the measured zero-closure
    /// transfer census proves nothing either way, because the carrier is inert:
    /// that zero holds for every variant and cannot distinguish "closures never
    /// transfer" from "nothing transfers yet."
    /// **The producer-issued occurrence of a source aggregate, whichever shape
    /// it is.**
    ///
    /// ⭐ One reader over both aggregate variants, so a consumer asks *"what is
    /// this template's producer authority?"* without branching on shape and
    /// without a shape-specific spelling drifting from its sibling. `None` for
    /// every non-aggregate, and for an aggregate whose occurrence is absent —
    /// which is an explicit fail-closed absence at the allocation, never a
    /// signal to fall back to a use coordinate.
    pub(in crate::cranelift_backend) fn source_aggregate_producer(
        &self,
    ) -> Option<AggregateOccurrenceId> {
        match self {
            Lowered::Constructor { occurrence, .. } | Lowered::Record { occurrence, .. } => {
                *occurrence
            }
            _ => None,
        }
    }

    pub(in crate::cranelift_backend) fn boundary_transfer_admissibility(
        &self,
    ) -> Result<(), CraneliftBackendError> {
        match self {
            // ── closures: the rejection this walk exists for ──────────────
            //
            // ⛔ One exact typed error at every depth, so a nested rejection is
            // not reported as some enclosing variant's failure.
            Lowered::Closure { .. } | Lowered::DeclarationClosure { .. } => {
                #[cfg(test)]
                d5a_trace(format!(
                    "  BOUNDARY-REFUSAL first closure child variant={}",
                    lowered_value_kind(self)
                ));
                Err(unsupported(
                    "Closure",
                    "a closure cannot cross the boundary: it is runtime-local and \
                     live-domain only, and it has no durable lane",
                ))
            }
            Lowered::ComputationalRecursorClosure { .. } => Err(unsupported(
                "ComputationalMatch",
                "a computational recursor closure names an in-flight activation, \
                 not a transferable value",
            )),
            Lowered::StaticResponseDeferred => Err(unsupported(
                "StaticResponseDeferred",
                "a deferred host response is compiler control and can only enter its exact response owner",
            )),

            // ── recursive carriers: recurse into EVERY child position ─────
            Lowered::Constructor { args, .. } => {
                for arg in
                    specialized_field_refs_at(args, "a constructor field crossing the boundary")?
                {
                    arg.boundary_transfer_admissibility()?;
                }
                Ok(())
            }
            Lowered::Record { fields, .. } => {
                for field in fields {
                    field.value.boundary_transfer_admissibility()?;
                }
                Ok(())
            }
            Lowered::HostResult { error, ok, .. } => {
                error.boundary_transfer_admissibility()?;
                ok.boundary_transfer_admissibility()
            }
            // ⚠ **The child position most easily missed.** `DynamicConstructor`
            // looks like a leaf: its payload is a struct, and the children are
            // two levels down, in a `Vec` of alternative structs. Treating it
            // as a leaf would leave a closure nested in a dynamic alternative
            // completely unguarded while every other arm was correct.
            Lowered::DynamicConstructor(dynamic) => {
                for alternative in &dynamic.alternatives {
                    for field in &alternative.fields {
                        field.boundary_transfer_admissibility()?;
                    }
                }
                Ok(())
            }

            // ── leaves: no `Lowered` child position exists ────────────────
            //
            // ⛔ Admitted here means "holds no closure", NOT "is transferable".
            // Whether a leaf has a boundary representation at all is
            // `boundary_disposition`'s question and is decided separately; a
            // `ProtocolOnly` or otherwise forbidden leaf is still refused
            // there. Conflating the two would let this walk read as a transfer
            // authorization it is not.
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

impl LoweredVariant {
    /// The boundary disposition of this variant.
    ///
    /// ⛔ **No `_` arm, by construction.** Every variant is named.
    pub(in crate::cranelift_backend) fn boundary_disposition(self) -> BoundaryDisposition {
        use BoundaryDisposition::{
            FailClosedForbidden, ProtocolOnly, RepresentedHandle, RepresentedImmediate,
        };
        match self {
            // ─── represented immediates ──────────────────────────────────
            //
            // Ken's `Int` is arbitrary precision, so the immediate field is a
            // fast path and the spill is the general case. The choice between
            // them is made by emitted code from the value's magnitude at
            // RUNTIME; nothing inspects a JIT-time value to pick a layout,
            // which is `AC-2`.
            LoweredVariant::Int => RepresentedImmediate {
                tag: BoundaryTag::ImmediateInt,
                spill: Some(BoundaryClass::Int),
            },
            // One bit. The only immediate that cannot overflow its field.
            LoweredVariant::Bool => RepresentedImmediate {
                tag: BoundaryTag::ImmediateBool,
                spill: None,
            },
            LoweredVariant::ProcessExitStatus => RepresentedImmediate {
                tag: BoundaryTag::ImmediateExitStatus,
                spill: Some(BoundaryClass::Int),
            },
            LoweredVariant::BoundedNat => RepresentedImmediate {
                tag: BoundaryTag::ImmediateBoundedNat,
                spill: Some(BoundaryClass::Int),
            },
            LoweredVariant::StructuralNat => RepresentedImmediate {
                tag: BoundaryTag::ImmediateStructuralNat,
                spill: Some(BoundaryClass::Int),
            },

            // ─── tokens: handles, NOT immediates ─────────────────────────
            //
            // ⛔ A capability or resource token is an opaque 64-bit identity,
            // and the immediate field is 56 bits. Truncating it would let two
            // distinct tokens compare equal — an authority forgery, not a
            // rounding error — so these take a handle whose node payload holds
            // the full word. Their owner is the invocation because that is
            // already the extent over which the token is valid.
            LoweredVariant::CapabilityToken | LoweredVariant::ResourceToken => RepresentedHandle {
                tag: BoundaryTag::InvocationBorrowed,
                class: BoundaryClass::BorrowedOpaque,
            },

            // ─── persistable ground values ───────────────────────────────
            //
            // `Constructor` is a REQUIRED live arm: 29 of the 41 measured
            // source-valued transfers are `Constructor` parameters, and a
            // disposition that parked it in `FailClosedForbidden` would reject
            // the dominant population — sound, and unable to satisfy `B2F`'s
            // `D6`/`D7`. That is the whole finding of `#10`.
            LoweredVariant::Constructor | LoweredVariant::DynamicConstructor => RepresentedHandle {
                tag: BoundaryTag::PersistentGround,
                class: BoundaryClass::Constructor,
            },
            LoweredVariant::Record => RepresentedHandle {
                tag: BoundaryTag::PersistentGround,
                class: BoundaryClass::Record,
            },
            LoweredVariant::String => RepresentedHandle {
                tag: BoundaryTag::PersistentGround,
                class: BoundaryClass::String,
            },
            LoweredVariant::Bytes => RepresentedHandle {
                tag: BoundaryTag::PersistentGround,
                class: BoundaryClass::Bytes,
            },

            // ─── borrowed ingress ────────────────────────────────────────
            //
            // ⛔ Invocation-owned: the referent is host storage that dies with
            // the native invocation. `AC-7`'s escape check keys on exactly this
            // owner, so a word naming one cannot silently outlive its buffer.
            //
            // `HostResult` is the second REQUIRED live arm. At compile time,
            // `Lowered::HostResult` retains a runtime success word plus two
            // candidate payload templates, but emitted transfer branches on the
            // normalized success word before recursive transfer and materializes
            // exactly one selected payload in canonical field zero. Carried
            // consumers read that representation through `host_success` and
            // field-zero `host_payload`.
            LoweredVariant::HostResult => RepresentedHandle {
                tag: BoundaryTag::InvocationHostResult,
                class: BoundaryClass::HostResult,
            },
            // `RT-CARRIER-BYTESPAN-OBSERVE` `D2`, Architect `dec_6qmstfn6tjqdt`
            // — normalization by COPY into the one existing lawful byte-span
            // row. `ResponseBytes` is an EXPLICITLY bytes-typed runtime
            // `{pointer, len}`, so its content can be copied into a
            // persistent-region `Bytes` node while the host span is still
            // valid, at the one-way producer.
            //
            // ⛔ This is NOT a retag of the borrowed word and NOT a new lane:
            // `(PersistentGround, Bytes)` is already in
            // `BOUNDARY_TAG_CLASS_RELATION`, and the producer copies the bytes
            // rather than publishing the host pointer. The referent after the
            // copy is region storage the store adopts, which is why the owner
            // may be `PersistentStore` without the escape rule weakening.
            //
            // ⚠ Its two former companions stay put, and the split is the
            // point. `BorrowedNativeValue` and `BorrowedOption` are opaque by
            // CLASS, not merely un-copied: neither carries a typed extent, so
            // there is nothing to copy without dereferencing a pointer whose
            // length this ABI does not know. Moving them here would be exactly
            // the confused-deputy hole the node's Banned section names.
            LoweredVariant::ResponseBytes => RepresentedHandle {
                tag: BoundaryTag::PersistentGround,
                class: BoundaryClass::Bytes,
            },
            LoweredVariant::BorrowedNativeValue | LoweredVariant::BorrowedOption => {
                RepresentedHandle {
                    tag: BoundaryTag::InvocationBorrowed,
                    class: BoundaryClass::BorrowedOpaque,
                }
            }

            // ─── closures: FAIL CLOSED for `C1` ──────────────────────────
            //
            // ⛔ **Changed by `RT-FNSPLIT-C1` under Architect Decision
            // `dec_21aa95jbsznfh`, and the history is the point.**
            //
            // `B2V` landed this arm as `RepresentedHandle { tag:
            // BoundaryTag::PersistentClosure, class: BoundaryClass::Closure }`,
            // deliberately, reasoning that *"a `FailClosedForbidden` here would
            // guarantee that wall for `B2F`."* That reasoning was recorded and
            // never executed — the whole disposition was inert.
            //
            // ⭐ The conflict it hid: `PersistentClosure` is the DURABLE lane
            // (`referent_owner() == PersistentStore`; the word outlives the
            // invocation that minted it), and `C1`'s settled input is that
            // ordinary closures stay **runtime-local and live-domain only**.
            // Making the landed disposition execute would have restored exactly
            // the lane the `#11` ruling forbids.
            //
            // ⛔ So this is conditional rejection of a VALUE SHAPE, not of the
            // closure concept and not of `Constructor`: a closure-free
            // constructor is still admitted and has its own positive control.
            // ⛔ Do not "fix" this by adding a third closure tag or by
            // disguising a closure as `InvocationBorrowed` / `BorrowedOpaque` —
            // both were considered and refused; they violate the ownership and
            // self-evidence boundaries rather than respecting them.
            //
            // ⚠ A live-domain closure carrier is a real and expected future
            // mechanism, but it is **`B2F`'s design**: it needs invocation
            // ownership, static origin, captured `BoundaryWord`s, callable
            // dispatch and non-escape enforcement. ⛔ `C1` may not invent it,
            // and this arm is not the place to smuggle it in.
            LoweredVariant::Closure | LoweredVariant::DeclarationClosure => FailClosedForbidden {
                why: "an ordinary closure is runtime-local and live-domain only; it has \
                      no durable boundary lane, and a callable cross-owner carrier is \
                      B2F's design rather than this node's",
            },

            // ─── fail-closed ─────────────────────────────────────────────
            //
            // ⛔ Not a value: it names a `ContinuationActivationId` and a
            // `RecursorInvocationSegment`, which identify ONE in-flight
            // activation of the enclosing recursor. Transferring it to another
            // unit would hand over a cursor into a frame that unit does not
            // have. Rejected before emission, with an exact error.
            LoweredVariant::ComputationalRecursorClosure => FailClosedForbidden {
                why: "a computational recursor closure names an in-flight activation, \
                      not a transferable value",
            },
            LoweredVariant::StaticResponseDeferred => FailClosedForbidden {
                why: "a deferred host response is compiler control and can only enter its exact response owner",
            },

            // ─── protocol-only ───────────────────────────────────────────
            LoweredVariant::RecursiveBackedge => ProtocolOnly {
                why: "a tail-recursive edge is already a CFG jump; the block is \
                      predecessor-free and there is no value to transfer",
            },
            // The trap word is its own `AbiCarrier`, written by the protocol.
            // ⛔ `result_carrier` is not its producer — the `AC-11` correction
            // on `B2F` says exactly this, and it holds here too.
            LoweredVariant::Trap => ProtocolOnly {
                why: "a trap is written to the activation's trap word, which is a \
                      protocol carrier and not a source-expression result",
            },
        }
    }
}

impl crate::boundary_value::BoundaryEmissionPlan {
    /// Derive the emission plan from the representation authority.
    ///
    /// ⛔ **Nothing here is written down.** The admitted class set is collected
    /// by sweeping [`BoundaryInput::all`] through the wildcard-free classifier
    /// and keeping the classes that reach a published `HandleWord`; the
    /// per-storage sets are that set filtered by
    /// [`BoundaryClass::storage_shape`]. So a class the disposition stops
    /// admitting, or a class whose storage changes, changes what the emitter
    /// generates — which is the causal edge `RECUT 2` requires.
    ///
    /// ⚠ This lives in `lowering` rather than beside the struct because
    /// [`BoundaryInput`] is `pub(in crate::cranelift_backend)`: the authority is
    /// only visible here, which is precisely why the emitter cannot restate it.
    pub(crate) fn derive() -> Self {
        use crate::boundary_value::{
            BoundaryClass, BoundaryReferentOwner, BoundaryStorageShape, BoundaryTag,
            BoundaryTagAdmission,
        };
        use std::collections::{BTreeMap, BTreeSet};

        let mut admitted: BTreeSet<BoundaryClass> = BTreeSet::new();
        let mut immediate_tags: BTreeSet<BoundaryTag> = BTreeSet::new();
        let mut handle_tags: BTreeSet<BoundaryTag> = BTreeSet::new();
        let mut owner_bands: BTreeMap<BoundaryReferentOwner, BTreeSet<BoundaryTag>> =
            BTreeMap::new();
        let mut immediate_value_classes: BTreeMap<BoundaryTag, BoundaryClass> = BTreeMap::new();
        let mut handle_class_relation: BTreeMap<BoundaryTag, BTreeSet<BoundaryClass>> =
            BTreeMap::new();
        for cell in BoundaryInput::all() {
            // ⛔ Wildcard-free: a new outcome variant must decide here whether
            // its tag is admitted, rather than defaulting to "not emitted".
            match cell.outcome() {
                BoundaryOutcome::ImmediateWord { tag, value_class } => {
                    immediate_tags.insert(tag);
                    // ⛔ An immediate the authority cannot classify gets NO
                    // entry, so the emitted helper fails closed on it rather
                    // than inheriting a default arm.
                    if let Some(class) = value_class {
                        immediate_value_classes.insert(tag, class);
                    }
                }
                BoundaryOutcome::HandleWord {
                    tag, class, owner, ..
                } => {
                    admitted.insert(class);
                    handle_tags.insert(tag);
                    owner_bands.entry(owner).or_default().insert(tag);
                    // ⛔ Node-class legality, from `HandleWord` only. An
                    // `ImmediateWord` has no node, so it contributes no row.
                    handle_class_relation.entry(tag).or_default().insert(class);
                }
                BoundaryOutcome::ProtocolOnly | BoundaryOutcome::FailClosedForbidden => {}
            }
        }
        let of_shape = |shape: BoundaryStorageShape| -> Vec<BoundaryClass> {
            admitted
                .iter()
                .copied()
                .filter(|class| class.storage_shape() == shape)
                .collect()
        };
        let int_magnitude = of_shape(BoundaryStorageShape::IntMagnitude);
        let byte_span = of_shape(BoundaryStorageShape::ByteSpan);
        // The admitted set is the union, not a range: a tag admitted as an
        // immediate and a tag admitted as a handle are both legal words, and
        // nothing requires the two groups to be numerically adjacent.
        let admitted_tags: Vec<BoundaryTag> = immediate_tags
            .union(&handle_tags)
            .copied()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        crate::boundary_value::BoundaryEmissionPlan::new(
            int_magnitude,
            byte_span,
            BoundaryTagAdmission::new(
                admitted_tags,
                immediate_tags.into_iter().collect(),
                handle_tags.into_iter().collect(),
                owner_bands
                    .into_iter()
                    .map(|(owner, tags)| (owner, tags.into_iter().collect()))
                    .collect(),
                immediate_value_classes.into_iter().collect(),
                handle_class_relation
                    .into_iter()
                    .map(|(tag, classes)| (tag, classes.into_iter().collect()))
                    .collect(),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // `RT-BACKEND-SPLIT-CLOSURE` (item 18) -- needed by the relocated
    // `refusal_pins_rehomed_computational_match_without_selector_exclusion`
    // below; not otherwise ambient in this module.
    use crate::cranelift_backend::lowering::core::tests::inert_test_static_origin;
    use crate::UnsupportedLowering;

    // ─── RT-FNSPLIT-B2V AC-3 — the Lowered disposition is exhaustive, no wildcard ─
    //
    // **MEASURED:** the `boundary_disposition` region of `lowering/mod.rs` contains
    // no `_ =>` arm and names all 21 `Lowered` variants.
    // **CLAIMED:** adding a 23rd variant is a COMPILE ERROR, so the transfer
    // population is closed structurally rather than by the `#10` histogram.
    // **THE GAP:** the compiler already guarantees exhaustiveness — what it cannot
    // guarantee is that nobody *silences* it. A `_ =>` arm would make a new variant
    // compile straight into whatever that arm returned. This pin exists for that
    // one job, and it also checks the dispatch is single, since a second
    // wildcarded dispatch elsewhere would be outside the compiler's guarantee too.

    #[test]
    fn b2v_ac3_the_lowered_boundary_disposition_has_no_wildcard_arm() {
        // `RT-LOWERING-VALUES-BOUNDARY-SPLIT` `D1`: `LoweredVariant` and its
        // `boundary_disposition` moved from `mod.rs` into this sibling `boundary`
        // module. `D2` then moved this test itself into that same module, so the
        // oracle now reads its own file rather than a relative sibling path.
        let source = include_str!("boundary.rs");
        let region = source
            .split_once("fn boundary_disposition(self) -> BoundaryDisposition {")
            .map(|(_, after)| after)
            .and_then(|after| {
                after
                    .split_once("}\n}\n\nimpl crate::boundary_value::BoundaryEmissionPlan")
                    .map(|(body, _)| body)
            })
            .expect("AC-3: the disposition region was not found, so every check below is vacuous");

        // ⚠ POSITIVE CONTROL FIRST. A negative check passes for any reason,
        // including an extractor that returned an empty region.
        assert!(
            region.contains("LoweredVariant::Constructor"),
            "AC-3: the extracted region does not contain a token that is certainly \
             in it, so its silence about `_ =>` means nothing"
        );

        // ⛔ Every arm head must name a `Lowered` variant.
        //
        // ⚠ This started as `!region.contains("_ =>")` and a compile-preserving
        // evasion DEFEATED it in one line: `unhandled => ...` is a binding
        // catch-all, so it silences exhaustiveness exactly like `_` while matching
        // no `_ =>` substring. The pin was green with the catch-all in place. What
        // the two evasions share is a GRANULARITY error — the check was a claim
        // about one spelling where the property is about the SHAPE of every arm —
        // so the repair is to enumerate arm heads rather than to add the second
        // spelling to a forbidden list that is open at the top.
        for line in region.lines() {
            let trimmed = line.trim();
            if !trimmed.contains("=>") {
                continue;
            }
            assert!(
                trimmed.starts_with("LoweredVariant::") || trimmed.starts_with('|'),
                "AC-3: `{trimmed}` is a match arm whose head does not name a \
                 `LoweredVariant`. A catch-all — `_` or a binding — silences \
                 exhaustiveness, so a new variant would compile into it instead of \
                 failing until someone dispositions it."
            );
        }

        // Every one of the 22 landed variants is named. Pinned as the ALLOWED
        // inventory: a variant renamed or removed reddens here with its own name in
        // the message, where a bare count would only say that something moved.
        for variant in [
            "Int",
            "Bool",
            "ProcessExitStatus",
            "CapabilityToken",
            "ResourceToken",
            "BoundedNat",
            "StructuralNat",
            "ResponseBytes",
            "HostResult",
            "DynamicConstructor",
            "Bytes",
            "BorrowedNativeValue",
            "BorrowedOption",
            "String",
            "Constructor",
            "Record",
            "Closure",
            "DeclarationClosure",
            "ComputationalRecursorClosure",
            "RecursiveBackedge",
            "Trap",
        ] {
            assert!(
                region.contains(&format!("LoweredVariant::{variant}")),
                "AC-3: `LoweredVariant::{variant}` has no disposition"
            );
        }

        // ⛔ `Constructor` and `HostResult` are REQUIRED LIVE ARMS. A disposition
        // that parked either in `FailClosedForbidden` would reject the dominant
        // measured population — sound, and unable to satisfy `B2F`'s `D6`/`D7`.
        // Checked positionally, so moving one into the forbidden block reddens.
        let forbidden_block = region
            .split_once("FailClosedForbidden {")
            .map(|(_, after)| after)
            .expect("AC-3: there is no fail-closed arm at all, which is itself wrong");
        for required_live in ["Constructor", "HostResult"] {
            assert!(
                !forbidden_block.contains(&format!("LoweredVariant::{required_live}")),
                "AC-3: `LoweredVariant::{required_live}` is a REQUIRED LIVE ARM and has \
                 been moved behind the fail-closed boundary"
            );
        }

        // The dispatch is single: one definition, so the compiler's exhaustiveness
        // guarantee covers the whole question and not just this copy of it.
        //
        // `D2` moved this test into the very file `include_str!` reads, so the
        // whole `source` now also contains this test's OWN two literal mentions
        // of the search phrase (the `split_once` needle above and this
        // assertion's own message-adjacent argument). Scoping the count to the
        // production text BEFORE `mod tests {` excludes those self-mentions
        // without weakening what the assertion actually checks: a second
        // PRODUCTION definition is still counted, wherever in the production
        // region it appears.
        let (production_source, _) = source
            .split_once("\nmod tests {\n")
            .expect("AC-3: the test module marker was not found in its own source");
        assert_eq!(
            production_source
                .matches("fn boundary_disposition(self)")
                .count(),
            1,
            "AC-3: a second disposition exists, and the compiler cannot promise \
             the two agree"
        );
    }

    // ─── RT-FNSPLIT-C1 D5 — closure admissibility is a property of the GRAPH ───

    /// A real `StaticOriginId` for a closure fixture.
    ///
    /// ⭐ One cannot be minted outside the planner — its ordinal is `pub(super)`,
    /// which is exactly the unmintability `D1`/`D2` rely on. So a control that needs
    /// a closure value must source a genuine origin from a genuine plan rather than
    /// fabricating one, and that constraint is a feature reaching into the tests.
    fn c1_closure_fixture_origin() -> StaticOriginId {
        let expr = RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        };
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new())
            .expect("the fixture expression plans");
        plan.root_static_origin()
            .expect("the plan has a root occurrence origin")
    }

    fn c1_closure(origin: StaticOriginId) -> Lowered {
        Lowered::Closure {
            captures: Vec::new(),
            params: Vec::new(),
            body: origin,
            boundary_environment: None,
        }
    }

    /// **`RT-FNSPLIT-C1` `D5` — a closure is inadmissible at the root and at every
    /// depth, and the rejection is the same exact typed error at each.**
    ///
    /// **MEASURED:** `boundary_transfer_admissibility` returns the closure-transfer
    /// error for a bare closure, a bare declaration closure, a closure nested one
    /// level inside a `Constructor`, and one nested two levels inside a
    /// `Constructor` -> `Record`.
    /// **CLAIMED:** admissibility is a property of the whole value graph.
    /// **THE GAP:** the root variant table cannot see any of the nested cases —
    /// `boundary_disposition` reports `RepresentedHandle` for every one of the
    /// nested fixtures below, because it is a function of the root tag alone. That
    /// disagreement is asserted here rather than described, so the walk cannot be
    /// deleted in favour of the table without reddening.
    #[test]
    fn c1_d5_a_closure_is_inadmissible_at_the_root_and_at_every_depth() {
        // Promise class: durable invariant.
        let origin = c1_closure_fixture_origin();
        let expected = unsupported(
            "Closure",
            "a closure cannot cross the boundary: it is runtime-local and \
             live-domain only, and it has no durable lane",
        );

        let bare = c1_closure(origin);
        let bare_declaration = Lowered::DeclarationClosure {
            reference: origin,
            symbol: "decl:fixture::f".to_string(),
            captures: Vec::new(),
            params: Vec::new(),
            body: origin,
        };
        let depth_1 = Lowered::Constructor {
            constructor: "ctor:fixture::Box::MkBox".to_string(),
            synthesized_identity: None,
            occurrence: None,
            args: vec![ConstructorField::specialized(c1_closure(origin))],
        };
        let depth_2 = Lowered::Constructor {
            constructor: "ctor:fixture::Box::MkBox".to_string(),
            synthesized_identity: None,
            occurrence: None,
            args: vec![ConstructorField::specialized(Lowered::Record {
                occurrence: None,
                fields: vec![LoweredRecordField {
                    name: "field:held".to_string(),
                    // This rig exercises `boundary_transfer_admissibility`, which
                    // walks for closure PRESENCE and reads no schema at all.
                    identity: None,
                    value: c1_closure(origin),
                }],
            })],
        };

        for (label, value) in [
            ("bare closure", &bare),
            ("bare declaration closure", &bare_declaration),
            ("closure nested at depth 1", &depth_1),
            ("closure nested at depth 2", &depth_2),
        ] {
            assert_eq!(
                value.boundary_transfer_admissibility().unwrap_err(),
                expected,
                "{label}: the graph holds a closure and must be refused with the \
                 exact closure-transfer error"
            );
        }

        // ⭐ THE GAP, asserted. The two nested fixtures are exactly the cases the
        // root table cannot see, and it must be shown to disagree — otherwise this
        // whole walk could be replaced by `boundary_disposition` and nothing would
        // redden.
        for (label, value) in [
            ("closure nested at depth 1", &depth_1),
            ("closure nested at depth 2", &depth_2),
        ] {
            assert!(
                matches!(
                    value.boundary_disposition(),
                    BoundaryDisposition::RepresentedHandle { .. }
                ),
                "{label}: the ROOT table already refuses this, so the graph walk is \
                 not what is catching it and this control proves nothing about depth"
            );
        }
    }

    /// **`RT-FNSPLIT-C1` `D5` — the positive path: a closure-free constructor is
    /// still admitted.**
    ///
    /// ⛔ This is the control that keeps the rejection **conditional**. Without it,
    /// an implementation that refused every `Constructor` outright would satisfy
    /// every negative control above — and it would be a capability removal wearing
    /// a soundness fix's clothing.
    ///
    /// ⚠ Admitted here means *"this graph holds no closure"*, **not** *"this value
    /// is transferable"*. Whether the root has a boundary representation at all is
    /// `boundary_disposition`'s separate question.
    #[test]
    fn c1_d5_a_closure_free_constructor_is_admissible() {
        // Promise class: durable invariant.
        let closure_free = Lowered::Constructor {
            constructor: "ctor:fixture::Pair::MkPair".to_string(),
            synthesized_identity: None,
            occurrence: None,
            args: vec![
                ConstructorField::specialized(Lowered::String("left".to_string())),
                ConstructorField::specialized(Lowered::Record {
                    occurrence: None,
                    fields: vec![LoweredRecordField {
                        name: "field:right".to_string(),
                        identity: None,
                        value: Lowered::Bytes(vec![7, 8]),
                    }],
                }),
            ],
        };
        assert!(
            closure_free.boundary_transfer_admissibility().is_ok(),
            "a constructor whose graph holds no closure must remain admissible; \
             D5 rejects closure-bearing GRAPHS, not the Constructor variant"
        );

        // Non-vacuity: the same shape with one leaf swapped for a closure must be
        // refused, so the `is_ok` above is attributable to the absence of a closure
        // rather than to the walk admitting everything it is handed.
        let origin = c1_closure_fixture_origin();
        let closure_bearing = Lowered::Constructor {
            constructor: "ctor:fixture::Pair::MkPair".to_string(),
            synthesized_identity: None,
            occurrence: None,
            args: vec![
                ConstructorField::specialized(Lowered::String("left".to_string())),
                ConstructorField::specialized(Lowered::Record {
                    occurrence: None,
                    fields: vec![LoweredRecordField {
                        name: "field:right".to_string(),
                        identity: None,
                        value: c1_closure(origin),
                    }],
                }),
            ],
        };
        assert!(
            closure_bearing.boundary_transfer_admissibility().is_err(),
            "NON-VACUITY: the walk admits a graph differing only by a closure in one \
             leaf position, so it is not discriminating on closures at all"
        );
    }

    // ─── RT-FNSPLIT-B2V AC-3 — exactly one of the FIVE static encoding policies ───

    /// **`AC-3` — every `Lowered` variant carries exactly one of `D4`'s five static
    /// encoding policies, and a declared spill is the SPILL policy.**
    ///
    /// ⛔ The prior control proved wildcard-freedom and nothing else. Exhaustiveness
    /// says every variant has *a* disposition; it says nothing about *which*, and
    /// the frame names the misassignment it cares about: a variant with a declared
    /// spill arm assigned *immediate-only* would let a proof attach handle evidence
    /// to one sampled spill while never establishing the handle obligations for the
    /// whole partition. That is the vacuity route `AC-10` exists to close, and no
    /// amount of "no `_` arm" detects it.
    ///
    /// ⚠ MEASURED: the policy of every one of the 22 variant **tags**. CLAIMED:
    /// each variant has exactly one of five policies. THE GAP: that a policy is a
    /// claim about the *variant* and not about a sampled value — closed structurally
    /// rather than asserted, because `boundary_disposition` now takes
    /// `LoweredVariant` and has no value to sample.
    #[test]
    fn b2v_ac3_every_variant_carries_exactly_one_of_the_five_static_policies() {
        use std::collections::{BTreeMap, BTreeSet};

        // ⛔ The sweep is over the tag set, so it is TOTAL by construction — there
        // are no 22 values to build and therefore no sampling to get wrong.
        let assigned: BTreeMap<LoweredVariant, StaticEncodingPolicy> = LoweredVariant::ALL
            .iter()
            .map(|variant| (*variant, variant.boundary_disposition().policy()))
            .collect();
        assert_eq!(
            assigned.len(),
            LoweredVariant::ALL.len(),
            "AC-3: a variant is listed twice, so the sweep is not over the tag set"
        );
        assert_eq!(
            assigned.len(),
            22,
            "AC-3: the landed variant count has moved"
        );

        // Every assigned policy is one of the five, and the five are the closed set.
        let five: BTreeSet<StaticEncodingPolicy> = StaticEncodingPolicy::ALL.iter().copied().collect();
        assert_eq!(
            five.len(),
            5,
            "AC-3: the policy set is not five distinct policies"
        );
        for (variant, policy) in &assigned {
            assert!(
                five.contains(policy),
                "AC-3: {variant:?} carries a policy outside the closed set"
            );
        }

        // ⛔ **THE misassignment the frame names.** A disposition that declares a
        // spill must be the third policy, never the first.
        for variant in LoweredVariant::ALL {
            let disposition = variant.boundary_disposition();
            if let BoundaryDisposition::RepresentedImmediate { spill, .. } = disposition {
                let expected = if spill.is_some() {
                    StaticEncodingPolicy::ImmediateWithDeclaredHandleSpill
                } else {
                    StaticEncodingPolicy::ImmediateOnly
                };
                assert_eq!(
                    disposition.policy(),
                    expected,
                    "AC-3: {variant:?} declares spill {spill:?} and must carry the \
                     matching policy — assigning immediate-only to a variant with a \
                     spill arm is the vacuity route AC-10 exists to close"
                );
            }
        }
        // ⚠ NON-DEGENERATE PAIR on that exact boundary: `Int` declares a spill and
        // `Bool` does not, and they must land in DIFFERENT policies. A checker that
        // ignored `spill` would put both in one and pass the loop above.
        assert_eq!(
            assigned[&LoweredVariant::Int],
            StaticEncodingPolicy::ImmediateWithDeclaredHandleSpill,
            "AC-3: Int declares a PersistentGround/Int spill, so it is the third policy"
        );
        assert_eq!(
            assigned[&LoweredVariant::Bool],
            StaticEncodingPolicy::ImmediateOnly,
            "AC-3: Bool has no spill arm, so it is the first policy"
        );
        assert_ne!(
            assigned[&LoweredVariant::Int],
            assigned[&LoweredVariant::Bool],
            "AC-3: the spill boundary must separate them, or neither assertion means \
             anything"
        );

        // `Constructor` and `HostResult` are REQUIRED LIVE arms — represented, in
        // policy terms, not merely absent from the forbidden block.
        for required in [LoweredVariant::Constructor, LoweredVariant::HostResult] {
            assert_eq!(
                assigned[&required],
                StaticEncodingPolicy::HandleOnly,
                "AC-3: {required:?} is a required LIVE represented arm"
            );
        }

        // ⚠ POSITIVE CONTROL over the policy set: every policy the frame declares
        // must actually be inhabited. A policy nobody uses is unreachable surface
        // that reads as supported — the same defect that removed `ImmediateCapability`
        // from the tag set — and a policy holding all 21 would make every check
        // above vacuous.
        let mut population: BTreeMap<StaticEncodingPolicy, usize> = BTreeMap::new();
        for policy in assigned.values() {
            *population.entry(*policy).or_default() += 1;
        }
        for policy in StaticEncodingPolicy::ALL {
            let count = population.get(&policy).copied().unwrap_or(0);
            assert!(
                count > 0,
                "AC-3: no variant carries {policy:?}, so it is unreachable surface"
            );
            assert!(
                count < LoweredVariant::ALL.len(),
                "AC-3: {policy:?} holds every variant, so the assignment is degenerate"
            );
        }

        // ⛔ Every fail-closed arm names an EXACT reason, never a bare rejection.
        for variant in LoweredVariant::ALL {
            match variant.boundary_disposition() {
                BoundaryDisposition::FailClosedForbidden { why }
                | BoundaryDisposition::ProtocolOnly { why } => assert!(
                    !why.is_empty(),
                    "AC-3: {variant:?} rejects without an exact reason"
                ),
                BoundaryDisposition::RepresentedImmediate { .. }
                | BoundaryDisposition::RepresentedHandle { .. } => {}
            }
        }
    }

    // ─── RT-FNSPLIT-B2V AC-10 — total classified-domain closure ──────────────────

    /// **`AC-10` — every boundary input receives exactly one actual outcome, and
    /// that outcome is entailed by its variant's static policy.**
    ///
    /// ⛔ **This is a STRUCTURAL totality proof, and it is not one dynamic test
    /// pretending to enumerate an infinite domain.** The admitted domains include
    /// unbounded integers, arbitrary byte contents, ownership states and recursive
    /// parent → child reachability; no finite runtime sweep covers them, and one
    /// wearing a universal name would be worse than an honest sweep. The closure has
    /// two layers:
    ///
    /// 1. the sealed wildcard-free disposition closes the **variant** layer
    ///    (`b2v_ac3_…`), and
    /// 2. every **value-dependent discriminator** is a closed finite partition —
    ///    magnitude/shape, lifetime/owner, parent → child reachability, and the
    ///    producer that minted the referent — reached from a value by a **total**
    ///    projection (`int_fits_immediate`, `referent_owner`, "does this aggregate
    ///    hold an invocation-owned child").
    ///
    /// ⭐ **So the infinite domain is covered by construction and only the finitely
    /// many CELLS need controls.** This sweeps the whole product.
    ///
    /// ⚠ MEASURED: every cell maps to exactly one outcome, permitted by its policy.
    /// CLAIMED: no input or encoding outcome is unclassified. THE GAP: that a value
    /// reaches its cell — which is the totality of the projections named above, and
    /// is why they are named rather than implied.
    #[test]
    fn b2v_ac10_every_boundary_input_receives_one_policy_entailed_outcome() {
        use std::collections::BTreeSet;

        let cells = BoundaryInput::all();
        // The product is closed and finite: every variant x 2 magnitudes x 3
        // reachabilities x 2 producers.
        assert_eq!(
            cells.len(),
            LoweredVariant::ALL.len() * 2 * 3 * 2,
            "AC-10: the cell product has moved"
        );
        assert_eq!(
            cells.iter().collect::<BTreeSet<_>>().len(),
            cells.len(),
            "AC-10: a cell is enumerated twice, so the sweep is not over the product"
        );

        let mut outcomes = BTreeSet::new();
        for cell in &cells {
            let policy = cell.variant.boundary_disposition().policy();
            let outcome = cell.outcome();
            // ⛔ **Entailment, not merely classification.** An outcome the policy
            // does not permit is the misassignment AC-3 names, seen from the value
            // level.
            assert!(
                outcome.permitted_by(policy),
                "AC-10: {cell:?} receives {outcome:?}, which {policy:?} does not permit"
            );
            // ⛔ Every handle outcome discharges class, referent owner, identity and
            // lifetime — including the SPILL ARM of an immediate policy, which is
            // the arm a proof may not attach to one sampled value.
            if let BoundaryOutcome::HandleWord { tag, owner, .. } = outcome {
                assert_eq!(
                    tag.referent_owner(),
                    owner,
                    "AC-10: {cell:?} declares an owner its tag does not carry — the \
                     lifetime obligation is the owner"
                );
                assert_ne!(
                    owner,
                    BoundaryReferentOwner::NoReferent,
                    "AC-10: a handle whose referent nothing owns has no lifetime"
                );
            }
            outcomes.insert(outcome);
        }

        // ⚠ POSITIVE CONTROL over the outcome set: all four actual outcomes must be
        // inhabited. A classifier that answered `FailClosedForbidden` everywhere
        // satisfies "exactly one outcome" and every entailment above — that is the
        // vacuity the frame's own AC-10 wording was rewritten to exclude.
        let kinds: BTreeSet<&str> = outcomes
            .iter()
            .map(|outcome| match outcome {
                BoundaryOutcome::ImmediateWord { .. } => "immediate",
                BoundaryOutcome::HandleWord { .. } => "handle",
                BoundaryOutcome::ProtocolOnly => "protocol-only",
                BoundaryOutcome::FailClosedForbidden => "fail-closed",
            })
            .collect();
        assert_eq!(
            kinds,
            ["fail-closed", "handle", "immediate", "protocol-only"]
                .into_iter()
                .collect::<BTreeSet<_>>(),
            "AC-10: an actual outcome is uninhabited, so the classification is degenerate"
        );

        // ⛔ **A policy's outcome varies ONLY in the discriminators it declares.**
        // An immediate-only policy whose outcome moved with magnitude would be a
        // spill arm nobody declared; a handle policy indifferent to reachability
        // would be admitting the parent → child escape.
        for variant in LoweredVariant::ALL {
            let policy = variant.boundary_disposition().policy();
            let at = |magnitude, reachability| {
                BoundaryInput {
                    variant,
                    magnitude,
                    reachability,
                    adoption: AdoptionPartition::StoreAdopted,
                }
                .outcome()
            };
            let within = at(
                MagnitudePartition::WithinImmediateField,
                ReachabilityPartition::Leaf,
            );
            let beyond = at(
                MagnitudePartition::BeyondImmediateField,
                ReachabilityPartition::Leaf,
            );
            match policy {
                StaticEncodingPolicy::ImmediateWithDeclaredHandleSpill => assert_ne!(
                    within, beyond,
                    "AC-10: {variant:?} declares a spill, so magnitude MUST change \
                     its outcome — a constant one is a spill arm that never fires"
                ),
                StaticEncodingPolicy::ImmediateOnly
                | StaticEncodingPolicy::HandleOnly
                | StaticEncodingPolicy::ProtocolOnly
                | StaticEncodingPolicy::FailClosedForbidden => assert_eq!(
                    within, beyond,
                    "AC-10: {variant:?} declares no spill, so magnitude must NOT \
                     change its outcome"
                ),
            }
        }

        // ⛔ Parent → child reachability is a real discriminator: at least one
        // persistent aggregate must reject the child that dies first, and the same
        // variant must be admitted when its children outlive it. A nondegenerate
        // pair on one variant, so "rejects everything" cannot pass.
        // ⛔ `Bytes`, not `Constructor`, is the variant that must still REJECT, and
        // the substitution is `RT-DECL-CLOSURE-PORT` `D7`'s, not a weakening.
        //
        // The rule being pinned is *"no surviving parent may name storage that dies
        // first."* A `Bytes` handle has **no children to take a lifetime meet
        // over**, so the only way to satisfy that rule on the
        // `ChildDiesBeforeParent` cell is to refuse — and it still does.
        let escaping = BoundaryInput {
            variant: LoweredVariant::Bytes,
            magnitude: MagnitudePartition::WithinImmediateField,
            reachability: ReachabilityPartition::ChildDiesBeforeParent,
            adoption: AdoptionPartition::StoreAdopted,
        };
        let sound = BoundaryInput {
            reachability: ReachabilityPartition::ChildrenOutliveParent,
            ..escaping
        };
        assert_eq!(
            escaping.outcome(),
            BoundaryOutcome::FailClosedForbidden,
            "AC-10: a persistent parent naming a child that dies first must reject"
        );
        assert!(
            matches!(sound.outcome(), BoundaryOutcome::HandleWord { .. }),
            "AC-10: the same variant with sound children must be admitted, or the \
             rejection above is about the variant and not about reachability"
        );

        // ⭐⭐ **`D7` — the aggregate lane, and why re-tagging is not relaxation.**
        //
        // An aggregate DOES have children, so the same rule has a second lawful
        // discharge: stop claiming the parent survives. `Constructor` at
        // `ChildDiesBeforeParent` is admitted, but **as an invocation-owned
        // parent** — so "no surviving parent names storage that dies first" holds
        // because there is no surviving parent, not because the check was dropped.
        //
        // ⛔ The owner assertion is the load-bearing half. Admitting the cell while
        // leaving `owner == PersistentStore` would be exactly the unsound edge this
        // node exists to remove, and it would still satisfy a bare
        // `matches!(.., HandleWord { .. })`.
        for aggregate in [LoweredVariant::Constructor, LoweredVariant::Record] {
            let meet = BoundaryInput {
                variant: aggregate,
                magnitude: MagnitudePartition::WithinImmediateField,
                reachability: ReachabilityPartition::ChildDiesBeforeParent,
                adoption: AdoptionPartition::StoreAdopted,
            };
            let BoundaryOutcome::HandleWord { tag, owner, .. } = meet.outcome() else {
                panic!("D7: {aggregate:?} with a shorter-lived child must take the aggregate lane");
            };
            assert_eq!(
                tag,
                BoundaryTag::InvocationAggregate,
                "D7: the aggregate meet must select the invocation aggregate lane"
            );
            assert_eq!(
                owner,
                BoundaryReferentOwner::InvocationArena,
                "D7: the aggregate parent must be invocation-owned, or admitting \
                 this cell restores the dangling edge instead of removing it"
            );

            // ⛔ And the lane is REACHED BY THE MEET, not by the shape: the same
            // aggregate whose children outlive it must still be persistent. Without
            // this, the assertions above would pass on an implementation that had
            // simply moved every aggregate onto the invocation lane.
            let outlives = BoundaryInput {
                reachability: ReachabilityPartition::ChildrenOutliveParent,
                ..meet
            };
            let BoundaryOutcome::HandleWord { tag, owner, .. } = outlives.outcome() else {
                panic!("D7: {aggregate:?} with surviving children must stay a handle");
            };
            assert_eq!(
                tag,
                BoundaryTag::PersistentGround,
                "D7: an aggregate whose children outlive it keeps the persistent lane"
            );
            assert_eq!(
                owner,
                BoundaryReferentOwner::PersistentStore,
                "D7: reachability, not the aggregate shape, selects the lane"
            );
        }

        // ⛔ Identity is CLASSIFIED, not assumed. A store-materialized persistent
        // handle carries the store's identity; an emitted-constructed one carries
        // none, by AC-6's design — and recording which is what makes identity
        // recoverable rather than unasked.
        let adopted = BoundaryInput {
            variant: LoweredVariant::Constructor,
            magnitude: MagnitudePartition::WithinImmediateField,
            reachability: ReachabilityPartition::Leaf,
            adoption: AdoptionPartition::StoreAdopted,
        };
        let pending = BoundaryInput {
            adoption: AdoptionPartition::PendingStoreAdoption,
            ..adopted
        };
        assert!(
            matches!(
                adopted.outcome(),
                BoundaryOutcome::HandleWord {
                    identity: HandleIdentity::StoreMinted,
                    ..
                }
            ),
            "AC-10: an adopted persistent handle carries the store's identity"
        );
        // ⛔ **A pending node is not a published handle at all.** Classifying it as
        // one with `NoStoreIdentity` was the defect: a consumer recovering the
        // ABSENCE of an identity has not recovered the same identity intact, and a
        // null `NODE_SLOT` denotes invocation ownership in this very layout.
        assert_eq!(
            pending.outcome(),
            BoundaryOutcome::FailClosedForbidden,
            "AC-10: a persistent node the store has not adopted must not publish"
        );

        // ⛔ **owner ⟺ identity, over the WHOLE product**: every published handle
        // declaring `PersistentStore` carries a real store identity, and no other
        // owner does.
        for cell in &cells {
            if let BoundaryOutcome::HandleWord {
                owner, identity, ..
            } = cell.outcome()
            {
                assert_eq!(
                    owner == BoundaryReferentOwner::PersistentStore,
                    identity == HandleIdentity::StoreMinted,
                    "AC-10: {cell:?} publishes owner {owner:?} with identity \
                     {identity:?} — a persistent handle has a store identity and \
                     nothing else does"
                );
            }
        }
    }

    // ---------------------------------------------------------------------------
    // `RECUT 2` — representation authority-to-execution closure
    // ---------------------------------------------------------------------------

    /// **`RECUT 2`.** Every admitted row closes every phase its outcome requires.
    ///
    /// ⛔ **This is the artifact RECUT 2 demands, and its row set is DERIVED.**
    /// Nothing here enumerates rows: the cells come from [`BoundaryInput::all`], the
    /// outcome from the wildcard-free classifier, the required phases from the
    /// outcome's own class, and the bindings from a struct with six mandatory
    /// fields. A hand-maintained matrix can drift from the production enums; this
    /// cannot, because there is no matrix to maintain.
    ///
    /// ⚠ MEASURED: for every derived cell, each required outcome phase is
    /// bound to a named production anchor, and every phase it does not require is
    /// `StructurallyAbsent`. CLAIMED: every admitted partition has one total
    /// executable lifecycle. THE GAP: that each anchor **is** the production item it
    /// names — closed for five anchors by `derived_witness` below, and for the other
    /// three by the named controls in `ProductionAnchor::CONTROL_CLOSED`.
    #[test]
    fn recut2_every_admitted_row_closes_every_required_phase() {
        use std::collections::BTreeSet;

        let cells = BoundaryInput::all();
        // Positive control FIRST: an empty sweep satisfies every `for` below.
        assert_eq!(
            cells.len(),
            LoweredVariant::ALL.len() * 2 * 3 * 2,
            "RECUT 2: the cell product moved, so this sweep is not over the partition"
        );

        let mut bound_anchors = BTreeSet::new();
        let mut absent_seen = false;
        for cell in &cells {
            let outcome = cell.outcome();
            let closure = outcome.phase_closure();
            for phase in LifecyclePhase::ALL {
                match (outcome.requires(phase), closure.binding(phase)) {
                    // Required and bound — the row closes this phase.
                    (true, PhaseBinding::Closed(anchor)) => {
                        bound_anchors.insert(anchor);
                    }
                    // Not required and absent — a contract, derived from the class.
                    (false, PhaseBinding::StructurallyAbsent) => absent_seen = true,
                    // ⛔ The two failures RECUT 2 exists to make loud.
                    (true, PhaseBinding::StructurallyAbsent) => panic!(
                        "RECUT 2: {cell:?} receives {outcome:?}, whose class REQUIRES \
                         {phase:?}, but the row declares it structurally absent — an \
                         authority with no closed execution is the named predicate's \
                         failure"
                    ),
                    (false, PhaseBinding::Closed(anchor)) => panic!(
                        "RECUT 2: {cell:?} receives {outcome:?}, whose class does not \
                         require {phase:?}, yet the row binds {anchor:?} — a phase \
                         nothing entails is a claim with no authority behind it"
                    ),
                }
            }
        }

        // ⚠ TWO-SIDED. Without these, a `requires` that answered `false` everywhere
        // would pass the loop above with every row structurally absent, and a
        // `requires` that answered `true` everywhere would pass a row that bound one
        // anchor to all six phases. Both must be inhabited.
        assert!(
            absent_seen,
            "RECUT 2: no phase is ever structurally absent, so `requires` is constant \
             and the derivation is degenerate"
        );
        assert!(
            bound_anchors.len() >= 5,
            "RECUT 2: only {} distinct anchors are reachable across the whole \
             partition, so most of the lifecycle is closed by one item standing in \
             for the rest",
            bound_anchors.len()
        );
    }

    /// **`RECUT 2`.** The phase inventory is bound to the type, not restated beside it.
    ///
    /// ⛔ **This is the `AC-V1b` defect, refused.** That pin froze `25` next to an
    /// enum and was invariant under adding a variant by construction. Here the
    /// index is produced by a wildcard-free match on the enum, so a seventh phase
    /// is a compile error there, and this control proves `ALL` did not silently
    /// drop one.
    #[test]
    fn recut2_the_phase_inventory_is_bound_to_the_type() {
        for (position, phase) in LifecyclePhase::ALL.into_iter().enumerate() {
            assert_eq!(
                phase.index(),
                position,
                "RECUT 2: {phase:?} sits at {position} in ALL but indexes {}",
                phase.index()
            );
        }
        let distinct: std::collections::BTreeSet<_> = LifecyclePhase::ALL.into_iter().collect();
        assert_eq!(
            distinct.len(),
            LifecyclePhase::ALL.len(),
            "RECUT 2: ALL repeats a phase, so a missing one is masked by a duplicate"
        );
    }

    /// **`RECUT 2`.** No anchor is silently unclosed.
    ///
    /// ⛔ **"Cannot determine" is a third outcome that must be ACCOUNTED FOR**, not
    /// one that falls through to pass. An anchor with no derived witness must name
    /// the causal control that closes its identity instead; an anchor that does
    /// neither is the residual with no cell.
    #[test]
    fn recut2_every_anchor_is_closed_by_a_witness_or_a_named_control() {
        use std::collections::BTreeSet;

        let mut anchors = BTreeSet::new();
        for cell in BoundaryInput::all() {
            let closure = cell.outcome().phase_closure();
            for phase in LifecyclePhase::ALL {
                if let PhaseBinding::Closed(anchor) = closure.binding(phase) {
                    anchors.insert(anchor);
                }
            }
        }
        assert!(
            !anchors.is_empty(),
            "RECUT 2: no anchor is bound anywhere, so every check below is vacuous"
        );

        let control_closed: BTreeSet<_> = ProductionAnchor::CONTROL_CLOSED
            .iter()
            .map(|(anchor, _)| *anchor)
            .collect();
        for anchor in &anchors {
            match anchor.derived_witness() {
                Some(_) => assert!(
                    !control_closed.contains(anchor),
                    "RECUT 2: {anchor:?} has BOTH a derived witness and a control \
                     row — one of the two is not describing this anchor"
                ),
                None => assert!(
                    control_closed.contains(anchor),
                    "RECUT 2: {anchor:?} has no derived witness and names no causal \
                     control, so nothing closes its identity — this is exactly the \
                     residual that reads as enforcement"
                ),
            }
        }

        // ⛔ Every declared control row must correspond to an anchor the partition
        // actually reaches. A control for a dead anchor is a row that can never fail.
        for (anchor, control) in ProductionAnchor::CONTROL_CLOSED {
            assert!(
                anchors.contains(anchor),
                "RECUT 2: {anchor:?} names control `{control}` but is bound by no row \
                 in the partition, so that control guards nothing"
            );
        }
    }

    /// **`RECUT 2`.** The derived witnesses are computed by production, not restated.
    ///
    /// ⛔ **A witness that agrees with a constant written beside it proves nothing**
    /// — that is two hand-maintained authorities agreeing, the `AC-1` defect. Each
    /// value below is checked against the *behaviour* of the authority that
    /// produces it, so rewiring the authority moves the witness.
    #[test]
    fn recut2_derived_witnesses_come_from_the_production_authority() {
        // The layout witness is the node extent DERIVED from the field inventory.
        // ⛔ Asserted as a relation to the inventory, never as a frozen byte count:
        // a reviewed layout delta is predicate delta, not a regression.
        let extent = ProductionAnchor::LayoutFieldInventory
            .derived_witness()
            .expect("the layout authority is evaluable without a JIT");
        assert!(
            extent > 0,
            "RECUT 2: the derived node extent is {extent}, so the field inventory \
             computes nothing and the layout authority has no content"
        );

        // ⛔ **The expected value is COMPUTED BY the authority here, not written as
        // a literal.** Mutation `M-E` deleted the production call inside
        // `derived_witness` and replaced it with `Some(1)`; against a frozen `1`
        // that evasion stayed green, because a hardcoded constant and a live call
        // are indistinguishable while they happen to agree. Computing the expected
        // side from the authority does not make them distinguishable *today* — see
        // the residual below — but it does mean the two diverge the moment the
        // contract moves, which is the drift a frozen literal cannot see.
        let normalization_rejects_leading_zero =
            !crate::boundary_value::boundary_int_magnitude_is_canonical(0, &[1, 0]);
        assert_eq!(
            ProductionAnchor::IntNormalizationAuthority.derived_witness(),
            Some(i64::from(normalization_rejects_leading_zero)),
            "RECUT 2: the normalization witness no longer tracks the canonical \
             sign/limb authority, so it is measuring its own spelling"
        );
        // ⚠ The two-sided half: the same authority must ACCEPT a canonical
        // magnitude, or "rejects a leading zero" is just "rejects everything" and
        // the witness above is `1` for a reason that has nothing to do with the
        // contract.
        assert!(
            crate::boundary_value::boundary_int_magnitude_is_canonical(0, &[1]),
            "RECUT 2: the normalization authority rejects a canonical magnitude too, \
             so its rejection above is not discriminating"
        );
        assert!(
            normalization_rejects_leading_zero,
            "RECUT 2: the authority now ACCEPTS a leading-zero magnitude — the \
             canonical sign/limb contract has changed and this anchor's meaning \
             changed with it"
        );

        // The status witnesses are the exact codes the production paths return.
        assert_eq!(
            ProductionAnchor::EmittedEscapeGate.derived_witness(),
            Some(crate::boundary_value::BOUNDARY_ERR_ESCAPE)
        );
        assert_eq!(
            ProductionAnchor::ReachableGraphValidator.derived_witness(),
            Some(crate::boundary_value::BOUNDARY_ERR_CYCLE)
        );
        assert_eq!(
            ProductionAnchor::RegionPublication.derived_witness(),
            Some(crate::boundary_value::BOUNDARY_ERR_SEALED)
        );
        // ⛔ And the statuses must be DISTINCT: three phases sharing one code would
        // make a control that fires for one indistinguishable from the others.
        let statuses = [
            crate::boundary_value::BOUNDARY_ERR_ESCAPE,
            crate::boundary_value::BOUNDARY_ERR_CYCLE,
            crate::boundary_value::BOUNDARY_ERR_SEALED,
        ];
        let distinct: std::collections::BTreeSet<_> = statuses.iter().collect();
        assert_eq!(
            distinct.len(),
            statuses.len(),
            "RECUT 2: two lifecycle phases report the same exact status, so a \
             control cannot tell which one fired"
        );
    }

    /// **`RECUT 2`.** The store-minted handle is the only outcome requiring all six.
    ///
    /// ⛔ **This is the row the six blocks kept failing**, so it is pinned as a
    /// relation to the phase inventory rather than as the number six — the count is
    /// derived from `LifecyclePhase::ALL`, so adding a phase strengthens this
    /// automatically instead of leaving a stale literal behind.
    #[test]
    fn recut2_only_the_store_minted_handle_requires_the_whole_lifecycle() {
        let full: Vec<BoundaryOutcome> = BoundaryInput::all()
            .into_iter()
            .map(|cell| cell.outcome())
            .filter(|outcome| {
                LifecyclePhase::ALL
                    .into_iter()
                    .all(|phase| outcome.requires(phase))
            })
            .collect();
        assert!(
            !full.is_empty(),
            "RECUT 2: no outcome requires the whole lifecycle, so the artifact never \
             asks the question the six blocks failed"
        );
        for outcome in full {
            assert!(
                matches!(
                    outcome,
                    BoundaryOutcome::HandleWord {
                        identity: HandleIdentity::StoreMinted,
                        ..
                    }
                ),
                "RECUT 2: {outcome:?} requires every phase, but only a store-minted \
                 handle should — a non-persistent outcome demanding adoption means \
                 the requirement is not derived from the class"
            );
        }
    }

    /// **`RECUT 2`, causal.** The emitter CONSUMES the representation authority.
    ///
    /// ⛔ **The ruling's bar, literally:** *"mutate or bypass the authority and the
    /// captured/emitted helper graph must change or reject; an emitter that ignores
    /// the plan must redden."* A test that only checks the plan is *passed* would be
    /// the `let _ = plan` the ruling excludes, so this feeds a **perturbed plan**
    /// and requires the emitted CLIF to differ.
    ///
    /// ⚠ MEASURED: the emitted helper graph under the derived plan differs from the
    /// graph under a plan whose class sets differ. CLAIMED: the helper bodies are
    /// generated from the authority. THE GAP: that the derived plan is the
    /// authority's real answer — closed by
    /// `recut2_the_plan_is_derived_from_the_partition_not_restated` below.
    #[test]
    fn recut2_the_emitted_helper_graph_changes_when_the_authority_changes() {
        use crate::boundary_value::{BoundaryClass, BoundaryEmissionPlan};

        let derived = BoundaryEmissionPlan::derive();
        let real = crate::boundary_value_clif::tests::capture_with_plan(&derived);

        // Positive control FIRST: the capture must be non-empty, or every
        // comparison below is between two empty strings and means nothing.
        assert!(
            real.contains("function"),
            "RECUT 2: the capture is empty, so the difference below is not evidence"
        );

        // ⛔ Perturb ONLY the axis the plan names: the class set a limb helper may
        // touch. Nothing else about the emitter or the module changes.
        let perturbed = BoundaryEmissionPlan::new(
            vec![BoundaryClass::Record],
            derived.byte_span_classes().to_vec(),
            derived.tags().clone(),
        );
        let other = crate::boundary_value_clif::tests::capture_with_plan(&perturbed);
        assert_ne!(
            real, other,
            "RECUT 2: the emitted helper graph is IDENTICAL under a plan whose \
             int-magnitude class set is different — the emitter is not consuming \
             the authority, it is only receiving it"
        );

        // ⛔ And the difference must be the CLASS COMPARISON, not incidental. The
        // real graph compares against `Int`; the perturbed one against `Record`.
        assert!(
            real.contains(&format!("{}", BoundaryClass::Int as i64))
                && other.contains(&format!("{}", BoundaryClass::Record as i64)),
            "RECUT 2: the graphs differ, but not in the class constant the plan \
             supplies — the difference is not attributable to the authority"
        );

        // ⚠ Two-sided: the SAME plan must produce the SAME graph, or `assert_ne!`
        // above would pass for any two captures and prove nothing about the plan.
        let again = crate::boundary_value_clif::tests::capture_with_plan(&derived);
        assert_eq!(
            real, again,
            "RECUT 2: two captures under the same plan differ, so emission is not \
             a function of the plan and the inequality above is noise"
        );
    }

    /// **`RECUT 2`.** The plan is derived from the partition, not restated beside it.
    ///
    /// ⛔ This is the half that keeps the causal test above honest: it would still
    /// pass if `derive()` returned a hand-written set. Here the expected sets are
    /// recomputed from the authority *in the test*, by the same two total
    /// projections — the classifier and `storage_shape` — so a `derive()` that
    /// stopped consulting either reddens.
    #[test]
    fn recut2_the_plan_is_derived_from_the_partition_not_restated() {
        use crate::boundary_value::{BoundaryClass, BoundaryEmissionPlan, BoundaryStorageShape};
        use std::collections::BTreeSet;

        let mut admitted: BTreeSet<BoundaryClass> = BTreeSet::new();
        for cell in BoundaryInput::all() {
            if let BoundaryOutcome::HandleWord { class, .. } = cell.outcome() {
                admitted.insert(class);
            }
        }
        assert!(
            !admitted.is_empty(),
            "RECUT 2: the partition admits no handle class at all, so the plan is \
             vacuous and every set below is trivially equal"
        );

        let plan = BoundaryEmissionPlan::derive();
        // ⛔ There is no whole-admitted-class assertion here, because the plan no
        // longer carries that set: no emitted helper ever read it. The per-shape
        // sets below are recomputed from `admitted` in this test, so the classifier
        // is still the thing being pinned — dropping an unconsumed accessor removed
        // a declaration, not a control.
        for (shape, got) in [
            (
                BoundaryStorageShape::IntMagnitude,
                plan.int_magnitude_classes(),
            ),
            (BoundaryStorageShape::ByteSpan, plan.byte_span_classes()),
        ] {
            let want: Vec<BoundaryClass> = admitted
                .iter()
                .copied()
                .filter(|class| class.storage_shape() == shape)
                .collect();
            assert_eq!(
                got, want,
                "RECUT 2: the plan's {shape:?} set is not the admitted classes of \
                 that storage shape"
            );
            assert!(
                !want.is_empty(),
                "RECUT 2: no admitted class has storage shape {shape:?}, so the \
                 guard built from it would name nothing"
            );
        }
    }

    /// **`RECUT 2`, causal — the TAG axis.** The emitted helpers branch on the
    /// plan's derived tag sets, not on an ordinal band.
    ///
    /// ⛔ **The ruling's bar is causation, not agreement:** *"mutate or bypass the
    /// authority and the captured/emitted helper graph must change or reject."* An
    /// assertion that the plan and the emitted bytes agree is explicitly listed as
    /// not counting, so this feeds a **perturbed tag admission** and requires the
    /// CLIF to differ.
    ///
    /// ⚠ MEASURED: the emitted graph under the derived tag sets differs from the
    /// graph under tag sets that admit a different set of tags. CLAIMED: the
    /// emitted validity, handle-ness and immediacy tests are generated from the
    /// authority. THE GAP: that the derived sets are the authority's real answer —
    /// closed by `recut2_the_tag_admission_is_derived_from_the_partition_not_restated`.
    #[test]
    fn recut2_the_emitted_helper_graph_changes_when_the_tag_sets_change() {
        use crate::boundary_value::{BoundaryEmissionPlan, BoundaryTag, BoundaryTagAdmission};

        let derived = BoundaryEmissionPlan::derive();
        let real = crate::boundary_value_clif::tests::capture_with_plan(&derived);
        assert!(
            real.contains("function"),
            "RECUT 2: the capture is empty, so the difference below is not evidence"
        );

        // ⛔ Perturb ONLY the tag axis: drop one admitted tag. Nothing about the
        // class sets, the emitter, or the module changes.
        let dropped = *derived
            .tags()
            .admitted()
            .last()
            .expect("the partition admits at least one tag");
        let thinner: Vec<BoundaryTag> = derived
            .tags()
            .admitted()
            .iter()
            .copied()
            .filter(|tag| *tag != dropped)
            .collect();
        assert!(
            thinner.len() + 1 == derived.tags().admitted().len(),
            "RECUT 2: the perturbation removed nothing, so the comparison below is \
             between two identical plans"
        );
        let perturbed = BoundaryEmissionPlan::new(
            derived.int_magnitude_classes().to_vec(),
            derived.byte_span_classes().to_vec(),
            BoundaryTagAdmission::new(
                thinner,
                derived.tags().immediate().to_vec(),
                derived.tags().handle().to_vec(),
                derived.tags().owner_bands().to_vec(),
                derived.tags().immediate_value_classes().to_vec(),
                derived.tags().handle_class_relation().to_vec(),
            ),
        );
        let other = crate::boundary_value_clif::tests::capture_with_plan(&perturbed);
        assert_ne!(
            real, other,
            "RECUT 2: the emitted helper graph is IDENTICAL under a plan admitting \
             one fewer tag — the emitter is not consuming the tag admission, it is \
             only receiving it"
        );

        // ⛔ And the difference must be the DROPPED TAG's own membership tests,
        // not something incidental. Presence/absence is the wrong needle: tag 8 is
        // also in the handle set and in an owner band, so it keeps being compared
        // for those. What must move is the COUNT — dropping it from exactly one
        // derived set removes exactly the comparisons that set generates, and
        // nothing in the perturbation can add one.
        let compares_for = |clif: &str| {
            let suffix = format!(", {}", dropped as i64);
            clif.lines()
                .filter(|line| line.contains("icmp_imm") && line.trim_end().ends_with(&suffix))
                .count()
        };
        let (before, after) = (compares_for(&real), compares_for(&other));
        assert!(
            before > 0,
            "RECUT 2: the real graph never compares against {dropped:?} at all, so \
             its disappearance below would not be evidence"
        );
        assert!(
            after < before,
            "RECUT 2: the graphs differ, but the emitted membership tests for \
             {dropped:?} did not decrease ({before} -> {after}) — the difference \
             is not attributable to the tag the plan stopped admitting"
        );

        // ⚠ Two-sided: the SAME plan must produce the SAME graph, or `assert_ne!`
        // above would pass for any two captures.
        let again = crate::boundary_value_clif::tests::capture_with_plan(&derived);
        assert_eq!(
            real, again,
            "RECUT 2: two captures under the same plan differ, so emission is not \
             a function of the plan and the inequality above is noise"
        );
    }

    /// **`RECUT 2`, causal — the OWNER axis.** The emitted region selection, the
    /// node's recorded owner, and the escape gate all branch on the plan's owner
    /// bands.
    ///
    /// ⚠ MEASURED: moving a tag from one owner band to another changes the emitted
    /// CLIF. CLAIMED: the emitted owner decisions are generated from the authority
    /// rather than from a threshold on tag order. THE GAP: that the bands are the
    /// partition's real answer — closed by the derivation test below.
    ///
    /// ⛔ This axis needs its own perturbation because the tag test above holds the
    /// bands fixed: a plan could consume the admitted set and still decide
    /// ownership from a hardcoded threshold, and every assertion there would pass.
    #[test]
    fn recut2_the_emitted_helper_graph_changes_when_the_owner_bands_change() {
        use crate::boundary_value::{BoundaryEmissionPlan, BoundaryTagAdmission};

        let derived = BoundaryEmissionPlan::derive();
        let real = crate::boundary_value_clif::tests::capture_with_plan(&derived);
        assert!(
            real.contains("function"),
            "RECUT 2: the capture is empty, so the difference below is not evidence"
        );

        // Move the first band's first tag into the second band — the owners keep
        // their identities, only the tag-to-owner assignment moves.
        let mut bands = derived.tags().owner_bands().to_vec();
        assert!(
            bands.len() >= 2 && !bands[0].1.is_empty(),
            "RECUT 2: fewer than two non-empty owner bands, so a reassignment \
             cannot be expressed and this test proves nothing"
        );
        let moved = bands[0].1.remove(0);
        bands[1].1.push(moved);
        bands[1].1.sort();
        let perturbed = BoundaryEmissionPlan::new(
            derived.int_magnitude_classes().to_vec(),
            derived.byte_span_classes().to_vec(),
            BoundaryTagAdmission::new(
                derived.tags().admitted().to_vec(),
                derived.tags().immediate().to_vec(),
                derived.tags().handle().to_vec(),
                bands,
                derived.tags().immediate_value_classes().to_vec(),
                derived.tags().handle_class_relation().to_vec(),
            ),
        );
        let other = crate::boundary_value_clif::tests::capture_with_plan(&perturbed);
        assert_ne!(
            real, other,
            "RECUT 2: the emitted helper graph is IDENTICAL after moving {moved:?} \
             to another owner band — the owner decisions are not derived from the \
             bands"
        );

        let again = crate::boundary_value_clif::tests::capture_with_plan(&derived);
        assert_eq!(
            real, again,
            "RECUT 2: two captures under the same plan differ, so the inequality \
             above is noise"
        );
    }

    /// **`RECUT 2`.** The tag admission is derived from the partition, not restated
    /// beside it.
    ///
    /// ⛔ The half that keeps the two causal tests honest: they would both still
    /// pass if `derive()` returned hand-written sets. Here the expected sets are
    /// recomputed from the authority *in the test*, by the same total projection —
    /// sweeping `BoundaryInput::all()` through the wildcard-free classifier — so a
    /// `derive()` that stopped consulting it reddens.
    #[test]
    fn recut2_the_tag_admission_is_derived_from_the_partition_not_restated() {
        use crate::boundary_value::{BoundaryEmissionPlan, BoundaryReferentOwner, BoundaryTag};
        use std::collections::{BTreeMap, BTreeSet};

        let mut immediate: BTreeSet<BoundaryTag> = BTreeSet::new();
        let mut handle: BTreeSet<BoundaryTag> = BTreeSet::new();
        let mut bands: BTreeMap<BoundaryReferentOwner, BTreeSet<BoundaryTag>> = BTreeMap::new();
        let mut value_classes: BTreeMap<BoundaryTag, crate::boundary_value::BoundaryClass> =
            BTreeMap::new();
        for cell in BoundaryInput::all() {
            match cell.outcome() {
                BoundaryOutcome::ImmediateWord { tag, value_class } => {
                    immediate.insert(tag);
                    if let Some(class) = value_class {
                        value_classes.insert(tag, class);
                    }
                }
                BoundaryOutcome::HandleWord { tag, owner, .. } => {
                    handle.insert(tag);
                    bands.entry(owner).or_default().insert(tag);
                }
                BoundaryOutcome::ProtocolOnly | BoundaryOutcome::FailClosedForbidden => {}
            }
        }
        // Positive controls: each population is non-empty, so the equalities below
        // are not agreements between empty sets.
        assert!(
            !immediate.is_empty() && !handle.is_empty() && bands.len() >= 2,
            "RECUT 2: the partition yields immediate={}, handle={}, bands={} — a \
             plan derived from it would be vacuous",
            immediate.len(),
            handle.len(),
            bands.len()
        );

        let plan = BoundaryEmissionPlan::derive();
        assert_eq!(
            plan.tags().immediate(),
            immediate.iter().copied().collect::<Vec<_>>(),
            "RECUT 2: the plan's immediate tag set is not the partition's"
        );
        assert_eq!(
            plan.tags().handle(),
            handle.iter().copied().collect::<Vec<_>>(),
            "RECUT 2: the plan's handle tag set is not the partition's"
        );
        assert_eq!(
            plan.tags().admitted(),
            immediate
                .union(&handle)
                .copied()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>(),
            "RECUT 2: the plan's admitted tag set is not the union of the two"
        );
        assert_eq!(
            plan.tags().owner_bands(),
            bands
                .into_iter()
                .map(|(owner, tags)| (owner, tags.into_iter().collect::<Vec<_>>()))
                .collect::<Vec<_>>(),
            "RECUT 2: the plan's owner bands are not the partition's"
        );
        // ⛔ The immediate-class projection, swept from the same outcomes. Kept
        // separate from the node-class relation on purpose: this is what the
        // `class` helper reports for an immediate word, not a node's `NODE_CLASS`.
        assert!(
            !value_classes.is_empty(),
            "RECUT 2: the partition classifies no immediate, so the equality below \
             is between empty relations"
        );
        assert_eq!(
            plan.tags().immediate_value_classes(),
            value_classes.into_iter().collect::<Vec<_>>(),
            "RECUT 2: the plan's immediate value-class relation is not the \
             partition's"
        );
    }

    /// **`RECUT 2`, identity.** The predicted-and-then-measured half: emitted code
    /// makes no identity decision the authority does not supply.
    ///
    /// ⚠ **The prediction, stated before it was measured** (recorded in the
    /// evidence doc at `ab11a3d2`): `HandleIdentity` is computed by
    /// `BoundaryInput::handle_identity` **from the owner alone**, so once the owner
    /// bands are derived, identity needs no separate wiring.
    ///
    /// ⚠ MEASURED: identity is a total function of owner across every admitted
    /// handle outcome, and the sole identity the emitted graph can mint is the
    /// absent one — every `alloc`ed node is written `NULL_SLOT`, which this ABI
    /// reads as "no store identity". CLAIMED: no emitted decision assigns identity.
    /// ⛔ **THE GAP, stated rather than closed:** this shows emitted code cannot
    /// mint a *store* identity, not that no future helper could. The residual is
    /// review-enforced — `escape_check`'s adoption gate is the mechanism that keeps
    /// it honest at runtime, and it is tested separately.
    #[test]
    fn recut2_identity_is_a_function_of_owner_and_needs_no_second_wiring() {
        use std::collections::BTreeMap;

        let mut by_owner: BTreeMap<BoundaryReferentOwner, BTreeSet<HandleIdentity>> = BTreeMap::new();
        for cell in BoundaryInput::all() {
            if let BoundaryOutcome::HandleWord {
                owner, identity, ..
            } = cell.outcome()
            {
                by_owner.entry(owner).or_default().insert(identity);
            }
        }
        assert!(
            by_owner.len() >= 2,
            "RECUT 2: fewer than two owners publish handles, so 'identity is a \
             function of owner' cannot be distinguished from 'identity is constant'"
        );
        for (owner, identities) in &by_owner {
            assert_eq!(
                identities.len(),
                1,
                "RECUT 2: {owner:?} publishes {} distinct identities, so identity \
                 is NOT a function of owner and the emitted side would need a \
                 decision the bands cannot supply",
                identities.len()
            );
        }
        // ⛔ Non-vacuity: the function must actually distinguish, or a constant
        // identity would satisfy every assertion above.
        let distinct: BTreeSet<HandleIdentity> = by_owner.values().flatten().copied().collect();
        assert!(
            distinct.len() >= 2,
            "RECUT 2: every owner yields the same identity, so the agreement above \
             is vacuous"
        );
    }

    // `RT-BACKEND-SPLIT-CLOSURE` (item 18) -- relocated verbatim from
    // `control.rs` (its own discriminated property, constructing a
    // `Lowered::ComputationalRecursorClosure` directly and asserting
    // `boundary_transfer_admissibility`'s exact refusal, belongs to this
    // module's own domain, matching the sibling tests immediately above).
    /// `RT-REFUSAL-PINS-REHOMED` D1-D3: the in-flight computational capsule is
    /// refused by its value-transfer policy without selecting either body-emission
    /// lane.
    ///
    /// MEASURED: the real `boundary_transfer_admissibility` method returns the
    /// `ComputationalMatch` refusal and its exact in-flight-activation reason.
    /// CLAIMED: a computational recursor capsule is control state, not a
    /// transferable value, regardless of which body-emission lane is present.
    /// THE GAP: this pins the construct-level refusal, not the four fixture-only
    /// programs that previously reached it through selector exclusion.
    ///
    /// Promise class: durable invariant. Removing the retiring lane leaves this
    /// value-transfer rule unchanged.
    #[test]
    fn refusal_pins_rehomed_computational_match_without_selector_exclusion() {
        let origin = RecursorProducerOriginId(41);
        let capsule = Lowered::ComputationalRecursorClosure {
            residual: Box::new(LoweringOperand::Specialized(Lowered::Trap(RuntimeTrap {
                code: RuntimeTrapCode::ExplicitTrap,
                message: "refusal pin inert residual".to_string(),
            }))),
            activation: ContinuationActivationId(43),
            invocation: RecursorInvocationSegment::new(
                origin,
                0,
                ComputationalRecursorLayer {
                    cases: Vec::new(),
                    default: RuntimeTrap {
                        code: RuntimeTrapCode::ExplicitTrap,
                        message: "refusal pin inert layer".to_string(),
                    },
                    outer_env: Vec::new(),
                    static_origin: inert_test_static_origin(),
                    provenance: RecursorFrameProvenance(44),
                    role: RecursorLayerRole::SelectsOccurrence { origin },
                    checked_frame_id: None,
                    checked_invocation_id: None,
                    checked_invocation_source: None,
                    checked_invocation_depth: 0,
                    semantic_pending: true,
                },
                RecursorUnwindStack {
                    later_wrappers_in_construction_order: Vec::new(),
                },
                ContinuationCursorId(42),
                None,
                None,
            ),
        };

        let refused = capsule
            .boundary_transfer_admissibility()
            .expect_err("an in-flight computational capsule must not be transferable");
        assert!(matches!(
            refused,
            CraneliftBackendError::Unsupported(UnsupportedLowering {
                construct: "ComputationalMatch",
                reason,
            }) if reason == "a computational recursor closure names an in-flight activation, not a transferable value"
        ));
    }
}
