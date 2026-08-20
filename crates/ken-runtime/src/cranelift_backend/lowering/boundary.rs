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
/// arm**, so a 22nd `Lowered` variant is a compile error in both.
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
    pub(in crate::cranelift_backend) const ALL: [LoweredVariant; 21] = [
        LoweredVariant::Int,
        LoweredVariant::Bool,
        LoweredVariant::ProcessExitStatus,
        LoweredVariant::CapabilityToken,
        LoweredVariant::ResourceToken,
        LoweredVariant::BoundedNat,
        LoweredVariant::StructuralNat,
        LoweredVariant::ResponseBytes,
        LoweredVariant::HostResult,
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
/// the 21 landed variants: a 22nd variant is a **compile error** until someone
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
    /// so a 22nd `Lowered` inhabitant is a compile error here as well as in
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
            // `HostResult` is the second REQUIRED live arm. It carries a
            // RUNTIME success discriminant plus the two payloads it selects
            // between; the landed lowering holds those payloads as compile-time
            // templates, which is why a compiled-once callee cannot consume one
            // today.
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
