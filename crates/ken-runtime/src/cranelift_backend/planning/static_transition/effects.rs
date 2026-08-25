//! The planner-owned Effects domain -- the host-effect seat authority.
//!
//! `RT-PLANNER-EFFECTS-SPLIT` `D1` -- this module owns the host-effect seat
//! population: one `PlannedEffectSeat` per capability/argument seat of every
//! admitted `Effect` occurrence, its contract lookup, and its closed-form
//! rebuild-equality plus uniqueness validation. `StaticTransitionPlan` stays
//! in the parent; the impl block here reads ancestor-private root state
//! under the standing child-module pattern (item 4's `units.rs` precedent).
//!
//! The emitter-owned half (`EffectSeatGroupId`, `EffectSeatLedger`,
//! `EffectSeatClosure`, `EffectSeatVisitMutation`,
//! `EffectSeatDispatchMutation`) is a DIFFERENT thing entirely and stays in
//! `lowering/mod.rs` for item 16 -- see the `D0` ledger's boundary proposal
//! in `docs/program/issues/RT-PLANNER-EFFECTS-SPLIT.md`.

use std::collections::BTreeSet;

use super::{
    occurrence_authority, planner_capacity_error, planner_error, ConstructorIdentity,
    CraneliftBackendError, PredeclaredFunctionId, StaticOriginId, StaticTransitionPlan,
};
use super::aggregates::{collect_site_operand_ordinals, host_effect_recipe_tree};
use crate::RuntimeExpr;

/// **`RT-DECL-CLOSURE-PORT` `D7` — the phase one host-effect seat's value is
/// actually in.**
///
/// ⛔ Deliberately its own type rather than a reuse of the continuation-input
/// projection's former boundary-use vocabulary. That vocabulary was keyed on
/// ABI slots; this one is keyed on a semantic seat of a host operation. They
/// answer different questions about different populations, and one enum
/// spanning both is what lets an answer derived for one be read as authority
/// for the other.
///
/// ⭐ That separation is why this type survived a deletion that took the other
/// one. `RT-CONTSPEC-LEDGER` (Architect `evt_1v9m7t4m9dmj7`) retired the four
/// continuation-side boundary-use axes as an unowned schema fragment, having
/// established that nothing consumed them. The proposal that preceded it was to
/// populate them by projecting THIS record onto them — refused precisely
/// because the two populations are not one. Keep them apart.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum EffectSeatPhase {
    /// A compile-time `Lowered` template the emitter may read directly.
    SpecializedTemplate,
    /// A boundary-carrier word, observable only through emitted helpers.
    CarriedWord,
}

/// What the emitter DOES at one seat.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum EffectSeatOperation {
    /// Select one member of a closed constructor set (`Stream`, `CreatePolicy`,
    /// `ResourceOpenMode`) and write its wire tag.
    SelectClosedTag,
    /// Project a byte span to a `(pointer, length)` pair.
    ProjectBytesSpan,
    /// Observe an opaque resource handle as a scalar.
    ObserveResourceHandle,
    /// Observe the opaque invocation capability token as a scalar.
    ObserveCapabilityToken,
    /// Narrow an exact `Int` to a checked `u64`.
    NarrowExactInt,
}

/// **Which slot of one effect occurrence a seat is.**
///
/// ⛔ The conditional capability is NOT argument ordinal 0, and collapsing the
/// two is the exact confusion the post-capability offset exists to prevent.
/// `FsOpen`'s capability and `FsOpen`'s first semantic argument are both real
/// consumed seats with different needs; keyed on the structural position alone
/// they would be positions 0 and 1, and keyed on a bare ordinal they would
/// collide at 0. This carries the distinction in the key itself.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum EffectSeatSlot {
    /// The capability at structural position 0, when the occurrence has one.
    Capability,
    /// The semantic argument at this ordinal, AFTER the capability offset.
    Argument(u32),
}

/// **What a seat must be able to OBSERVE — derived FIRST, before any
/// representation is selected.**
///
/// ⭐⭐ **The direction is the whole point.** A `Need` read off a chosen
/// disposition reverses the equation: it makes whatever the representation
/// happens to offer into the definition of what the consumer wanted. Planning
/// derives this from the seat's own semantics — what the wire request requires
/// at that ordinal — and only then selects and validates an `Avail` that
/// satisfies it.
///
/// ⛔ Equality-bearing, together with the operation and the semantic ordinal. A
/// seat's identity is not its structural role: `BufferAllocate.capacity`,
/// `FsChangeMode.mode` and `FsReadAt.length` are all `EffectArgument`s holding
/// an `Int`, and they are three different seats.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum EffectSeatNeed {
    /// The member identity of a closed constructor set.
    ConstructorTag,
    /// A byte span's address and length.
    BytesPointerLength,
    /// An opaque resource handle's scalar word.
    ResourceScalar,
    /// The invocation capability token's scalar word.
    ///
    /// ⛔ Deliberately not [`Self::ResourceScalar`]. Both are opaque scalars and
    /// the emitter reads both through `emit_carrier_scalar`, but a capability
    /// token authorizes an operation while a resource handle names an object.
    /// One need spanning both would let a seat proved for one be read as proof
    /// for the other.
    CapabilityTokenScalar,
    /// An exact `Int`'s magnitude as a checked `u64`.
    ExactIntU64,
}

/// **The phases in which a seat's [`EffectSeatNeed`] can actually be
/// satisfied.**
///
/// ⛔ Per-SEAT, never per-need, and that is not redundancy. `BufferFreeze`'s
/// buffer and span-origin seats observe a resource handle in either phase
/// because their route already emits the helper; `ResourceRelease`'s and
/// `FsReadAt`'s observe the same `ResourceScalar` and have no such route. Same
/// need, different availability — a per-need table would have to answer one of
/// them wrongly.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct EffectSeatAvail {
    pub(in crate::cranelift_backend) specialized: bool,
    pub(in crate::cranelift_backend) carried: bool,
}

impl EffectSeatAvail {
    const SPECIALIZED_ONLY: Self = Self {
        specialized: true,
        carried: false,
    };
    const EITHER_PHASE: Self = Self {
        specialized: true,
        carried: true,
    };

    /// Whether a seat consumed in `phase` can satisfy its need.
    ///
    /// ⛔ This IS the `Need ⊆ Avail` test. It is a membership question, and the
    /// seat it is asked about carries its own coordinates — so a seat that
    /// fails it is refused as that exact seat of that exact operation, never as
    /// a generic specialized-only surface.
    pub(in crate::cranelift_backend) fn admits(self, phase: EffectSeatPhase) -> bool {
        match phase {
            EffectSeatPhase::SpecializedTemplate => self.specialized,
            EffectSeatPhase::CarriedWord => self.carried,
        }
    }
}

/// **One FULL semantic seat of one admitted host effect.**
///
/// ⭐ "Full" is the correction this record exists to make. A seat is not a
/// structural position and not a nominal role: it is the position *plus* the
/// operation it belongs to *plus* its post-capability-offset semantic ordinal.
/// Two seats agreeing on the first and differing on either of the others are
/// two records, because the wire request wants different things of them.
/// One artifact-static branch of the finite constructor dispatcher used by a
/// carried `ConstructorTag` seat.
///
/// Identity words are planner-issued [`ConstructorIdentity`] values. The
/// runtime carries only its ordinary constructor tag and positional fields;
/// neither variant adds a family/body/code tag to the carrier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum EffectSeatConstructorPath {
    /// A nullary constructor maps directly to one host-wire tag.
    Root {
        identity: ConstructorIdentity,
        field_count: u32,
        wire_tag: i64,
    },
    /// A constructor whose one positional field contains the discriminant that
    /// completes the host-wire tag. `ResourceWriteCreate(CreatePolicy)` is the
    /// current instance.
    PositionalChild {
        root_identity: ConstructorIdentity,
        root_field_count: u32,
        child_position: u32,
        child_identity: ConstructorIdentity,
        child_field_count: u32,
        wire_tag: i64,
    },
}

/// The closed source-constructor roles a host seat may dispatch.
///
/// Kept planner-private so lowering cannot submit a spelling and mint a second
/// identity authority. Lowering receives only [`EffectSeatConstructorPath`]
/// values resolved from the semantic plane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EffectSeatConstructorRole {
    StreamStdin,
    StreamStdout,
    StreamStderr,
    CreateNew,
    CreateOrTruncate,
    CreateOrKeep,
    ResourceRead,
    ResourceMetadata,
    ResourceWriteCreate,
}

impl EffectSeatConstructorRole {
    fn suffix(self) -> &'static str {
        match self {
            Self::StreamStdin => "::Stream::Stdin",
            Self::StreamStdout => "::Stream::Stdout",
            Self::StreamStderr => "::Stream::Stderr",
            Self::CreateNew => "::CreatePolicy::CreateNew",
            Self::CreateOrTruncate => "::CreatePolicy::CreateOrTruncate",
            Self::CreateOrKeep => "::CreatePolicy::CreateOrKeep",
            Self::ResourceRead => "::ResourceOpenMode::ResourceRead",
            Self::ResourceMetadata => "::ResourceOpenMode::ResourceMetadata",
            Self::ResourceWriteCreate => "::ResourceOpenMode::ResourceWriteCreate",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct PlannedEffectSeat {
    pub(in crate::cranelift_backend) effect_origin: StaticOriginId,
    /// The exact child occurrence that produces this seat's value.
    pub(in crate::cranelift_backend) child_origin: StaticOriginId,
    /// The child's STRUCTURAL position, capability included.
    pub(in crate::cranelift_backend) position: u32,
    pub(in crate::cranelift_backend) operation: ken_host::HostOpV1,
    /// The slot: the capability, or a semantic argument ordinal AFTER the
    /// conditional capability offset.
    ///
    /// ⛔ Not the structural position. An operation carrying a capability shifts
    /// every argument by one, so a seat keyed on the structural position alone
    /// names a different semantic argument depending on a fact about the
    /// operation's capability that the position does not carry.
    pub(in crate::cranelift_backend) slot: EffectSeatSlot,
    /// The owner of the occurrence that PRODUCES this seat's value.
    pub(in crate::cranelift_backend) producer_owner: PredeclaredFunctionId,
    /// The owner of the body that DISPATCHES the effect.
    ///
    /// ⚠ **No phase accompanies either owner, and that is a measured
    /// correction rather than an omission.** A derived `consumer_phase` was
    /// built first, from the child's planned join-result representation widened
    /// to `CarriedWord` across an owner boundary, and checked against the phase
    /// the emitter actually held. It was WRONG on real programs: `BufferFreeze`
    /// argument 0 and `FsReadFile`'s capability both arrive carried while their
    /// child occurrence has no `CarrierWord` join result and no owner crossing,
    /// because the value reaches the body through a declared ABI slot — a fact
    /// about the enclosing unit's parameters, not about the child. Rather than
    /// keep a prediction that is false, the phase is OBSERVED at the claim and
    /// `Need ⊆ Avail` is asked there, of the operand actually in hand. The
    /// membership question is unchanged; only the thing it is asked about is
    /// now a measurement instead of a guess.
    pub(in crate::cranelift_backend) consumer_owner: PredeclaredFunctionId,
    pub(in crate::cranelift_backend) semantic_operation: EffectSeatOperation,
    pub(in crate::cranelift_backend) need: EffectSeatNeed,
    pub(in crate::cranelift_backend) avail: EffectSeatAvail,
}

/// **Test-only seat construction for the `RT-CARRIER-BYTESPAN-OBSERVE`
/// `D4` observer control.**
///
/// ⛔ Gated on its own, and the gate is not decoration: an earlier draft of
/// this insertion sat between the mutation enum's `#[cfg(test)]` and its
/// `#[derive]`, capturing the attribute and shipping that enum into
/// production builds. The `--lib` test profile cannot observe that, which
/// is why the repair is validated by a production build.
#[cfg(test)]
impl PlannedEffectSeat {
    /// A seat record for a control, with a caller-chosen `need`.
    ///
    /// ⚠ Test-only scaffolding for `RT-CARRIER-BYTESPAN-OBSERVE` `D4`, whose
    /// observer consumes this record. The id newtypes are `pub(super)` here, so
    /// a control in the lowering cannot build one itself.
    ///
    /// ⛔ `avail` is `SPECIALIZED_ONLY` and stays that way: `D4` activates
    /// nothing, and a fixture handing itself `EITHER_PHASE` would be asserting
    /// `D5`'s outcome.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn for_observer_control(
        need: EffectSeatNeed,
    ) -> Self {
        PlannedEffectSeat {
            effect_origin: StaticOriginId(0),
            child_origin: StaticOriginId(1),
            position: 0,
            operation: ken_host::HostOpV1::FsReadFile,
            slot: EffectSeatSlot::Argument(0),
            producer_owner: PredeclaredFunctionId(0),
            consumer_owner: PredeclaredFunctionId(0),
            semantic_operation: EffectSeatOperation::ProjectBytesSpan,
            need,
            avail: EffectSeatAvail::SPECIALIZED_ONLY,
        }
    }
}

/// **Erase one axis of the seat key, or collapse every seat onto one
/// contract.**
///
/// ⛔ Applied ONLY inside [`build_host_effect_seat_plan`], never in the
/// re-derivation the close performs. That asymmetry is the whole mechanism: the
/// rebuild-equality validation mutates on both sides and so cannot see any of
/// these, which is correct — it checks the derivation is a function, not that
/// the function is right. What sees them is the independent recomputation of
/// the contract from the operation and the slot at the ledger close.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum EffectSeatPlanMutation {
    Exact,
    /// The operation stops being part of the key: every seat records the first
    /// admitted operation.
    EraseOperation,
    /// The ordinal stops being part of the key: every argument seat becomes
    /// argument 0.
    EraseOrdinal,
    /// The need stops being part of the authority: every seat records one need.
    EraseNeed,
    /// Every seat takes one contract, which is the "all argument seats are the
    /// same kind of thing" collapse the full-seat key exists to refuse.
    CollapseContract,
}

#[cfg(test)]
thread_local! {
    static EFFECT_SEAT_PLAN_MUTATION: std::cell::Cell<EffectSeatPlanMutation> =
        const { std::cell::Cell::new(EffectSeatPlanMutation::Exact) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn set_effect_seat_plan_mutation(
    mutation: EffectSeatPlanMutation,
) {
    EFFECT_SEAT_PLAN_MUTATION.with(|cell| cell.set(mutation));
}

/// **The host operations this backend represents as consumers.**
///
/// ⛔ It lives in PLANNING because the seat population is derived here and the
/// emitter's admission check reads the same list. A second copy on the lowering
/// side would be a second authority: the two could disagree about whether an
/// operation is admitted, and the disagreement would show up as a seat with no
/// planned record rather than as a contradiction anyone stated.
pub(in crate::cranelift_backend) const CRANELIFT_HOST_EFFECT_CONSUMERS_V1:
    [ken_host::HostOpV1; 13] = [
    ken_host::HostOpV1::ConsoleWrite,
    ken_host::HostOpV1::ConsoleFlush,
    ken_host::HostOpV1::ConsoleIsTerminal,
    ken_host::HostOpV1::FsReadFile,
    ken_host::HostOpV1::FsWriteFile,
    ken_host::HostOpV1::FsChangeMode,
    ken_host::HostOpV1::FsOpen,
    ken_host::HostOpV1::FsHandleMetadata,
    ken_host::HostOpV1::FsReadAt,
    ken_host::HostOpV1::FsWriteAt,
    ken_host::HostOpV1::ResourceRelease,
    ken_host::HostOpV1::BufferAllocate,
    ken_host::HostOpV1::BufferFreeze,
];

/// The seat contract of one admitted operation at one semantic ordinal.
///
/// ⛔ **Total over the 13 admitted operations, with no `_` arm**, so a new
/// admitted operation is a compile error here rather than an operation whose
/// seats silently have no contract. `None` means the operation has no seat at
/// that ordinal, which is an arity disagreement and is refused by the caller —
/// never a seat that is exempt.
///
/// ⚠ The `Avail` column is where this release's one new capability appears, and
/// nowhere else: `BufferAllocate.capacity` is the single seat whose exact `Int`
/// this release can observe through the carrier ABI. Every other `ExactIntU64`
/// seat stays specialized-only, which is why `Avail` is recorded per seat.
fn host_effect_seat_contract(
    operation: ken_host::HostOpV1,
    slot: EffectSeatSlot,
) -> Option<(EffectSeatOperation, EffectSeatNeed, EffectSeatAvail)> {
    use ken_host::HostOpV1 as Op;
    use EffectSeatAvail as Avail;
    use EffectSeatNeed as Need;
    use EffectSeatOperation as Semantic;
    // ⭐ The CAPABILITY half, kept ahead of the argument table because its
    // population is the exact complement: the four FS-path operations require
    // one, and every other admitted operation refuses one outright. A `None`
    // here is therefore not an arity gap but a capability the operation does
    // not admit, and the caller refuses it with the seat's own coordinates.
    let ordinal = match slot {
        EffectSeatSlot::Capability => {
            return match operation {
                Op::FsReadFile | Op::FsWriteFile | Op::FsChangeMode | Op::FsOpen => Some((
                    Semantic::ObserveCapabilityToken,
                    Need::CapabilityTokenScalar,
                    // Both phases: the emitter reads a specialized
                    // `CapabilityToken` template directly and a carried word
                    // through `emit_carrier_scalar`.
                    Avail::EITHER_PHASE,
                )),
                Op::ConsoleWrite
                | Op::ConsoleFlush
                | Op::ConsoleIsTerminal
                | Op::FsHandleMetadata
                | Op::FsReadAt
                | Op::FsWriteAt
                | Op::ResourceRelease
                | Op::BufferAllocate
                | Op::BufferFreeze
                | Op::ConsoleRead
                | Op::ClockWallNow
                | Op::ClockMonotonicNow
                | Op::ClockSleepUntil
                | Op::FsAppendFile
                | Op::FsMetadata
                | Op::FsReadDirectory
                | Op::FsCreateDirectory
                | Op::FsRemoveFile
                | Op::FsRemoveDirectory
                | Op::FsRename
                | Op::EntropyRandomBytes => None,
            };
        }
        EffectSeatSlot::Argument(ordinal) => ordinal,
    };
    let tag = (
        Semantic::SelectClosedTag,
        Need::ConstructorTag,
        Avail::SPECIALIZED_ONLY,
    );
    let bytes = (
        Semantic::ProjectBytesSpan,
        Need::BytesPointerLength,
        Avail::SPECIALIZED_ONLY,
    );
    // `RT-CARRIER-BYTESPAN-OBSERVE` `D5` — the byte-span seats whose carried
    // route is PROVED, per seat and each against its own measured witness.
    //
    // The tuple is shared by construction, so it is deliberately NOT the
    // discriminator: `bytes` and `carried_bytes` differ only in `Avail`, and a
    // seat moves between them only when a row was observed refusing at that
    // exact `(operation, ordinal)` and observed lowering afterwards. `AC-4`'s
    // disposition table in the node records the evidence per seat, including
    // the proof for each seat left on `bytes`.
    let carried_bytes = (
        Semantic::ProjectBytesSpan,
        Need::BytesPointerLength,
        Avail::EITHER_PHASE,
    );
    let resource = (
        Semantic::ObserveResourceHandle,
        Need::ResourceScalar,
        Avail::SPECIALIZED_ONLY,
    );
    let phase_bearing_resource = (
        Semantic::ObserveResourceHandle,
        Need::ResourceScalar,
        Avail::EITHER_PHASE,
    );
    let exact_int = (
        Semantic::NarrowExactInt,
        Need::ExactIntU64,
        Avail::SPECIALIZED_ONLY,
    );
    let carried_exact_int = (
        Semantic::NarrowExactInt,
        Need::ExactIntU64,
        Avail::EITHER_PHASE,
    );
    match (operation, ordinal) {
        (Op::ConsoleWrite, 0) | (Op::ConsoleFlush, 0) | (Op::ConsoleIsTerminal, 0) => Some(tag),
        // PROVED carried, per seat: `D5` measured a carried word reaching each
        // of these and the observer consuming it. Neither is site-bound.
        (Op::ConsoleWrite, 1) | (Op::FsWriteFile, 2) => Some(carried_bytes),
        // LEFT SPECIALIZED_ONLY for the direct operation consumer, and NOT
        // because the observer fails them — `D5` measured it succeeding at all
        // four. Each operation's synthesized `FileError` separately declares
        // `SiteOperand(0)`. `RT-SITEOP-CARRIED-WITNESS` projects that exact
        // second use through the emitted byte-span helper without widening the
        // seat-wide `Avail` relation.
        (Op::FsReadFile, 0) | (Op::FsWriteFile, 0) | (Op::FsChangeMode, 0) | (Op::FsOpen, 0) => {
            Some(bytes)
        }
        (Op::FsWriteFile, 1) | (Op::FsOpen, 1) => Some(tag),
        (Op::FsChangeMode, 1) => Some(exact_int),
        (Op::FsHandleMetadata, 0) | (Op::ResourceRelease, 0) => Some(resource),
        // ⭐ The one seat this release teaches the carrier to observe.
        (Op::BufferAllocate, 0) => Some(carried_exact_int),
        (Op::BufferFreeze, 0) | (Op::BufferFreeze, 3) => Some(phase_bearing_resource),
        (Op::BufferFreeze, 1) | (Op::BufferFreeze, 2) => Some(exact_int),
        (Op::FsReadAt, 0) | (Op::FsReadAt, 2) | (Op::FsWriteAt, 0) | (Op::FsWriteAt, 2) => {
            Some(resource)
        }
        (Op::FsWriteAt, 5) => Some(resource),
        // `RT-EXACTINT-CARRIED-OBSERVE` `D1` -- the positioned arm's exact-`Int`
        // seats join the EITHER_PHASE classification `BufferAllocate` `0`
        // already uses. Architect `evt_2kspreq08s3a` ruled AVAIL here rather
        // than a route, and the distinction is need-specific rather than a
        // reversal: route-not-Avail was required for `ResourceScalar` because
        // `emit_carrier_scalar` would read ANY word's bits as a scalar, so a
        // guard had to dominate the read. `narrow_carried_int_u64` is itself
        // FAIL-CLOSED -- it branches on the boundary tag and `require_i64`s the
        // viewed path's status, so a word that is not a decodable `Int` takes
        // the failure return and an out-of-range one returns `valid = 0` into
        // this operation's EXISTING narrow-failure lane. The accept path
        // re-runs the fail-closed consumer; it is simply the decoder rather
        // than a route's guards.
        //
        // ⇒ One mechanism for one need. A route here would leave
        // `BufferAllocate` `0` on `Avail` and these on a route -- two admission
        // mechanisms for a single need.
        //
        // These six move together because ONE reader edit covers them: they
        // share the positioned emitter arm. The Avail widening is inert for a
        // seat that never arrives carried (`EITHER_PHASE` still admits the
        // specialized phase through `Direct`), and the reader is total over
        // both phases.
        (Op::FsReadAt, 1)
        | (Op::FsReadAt, 3)
        | (Op::FsReadAt, 4)
        | (Op::FsWriteAt, 1)
        | (Op::FsWriteAt, 3)
        | (Op::FsWriteAt, 4) => Some(carried_exact_int),
        // An ADMITTED operation at an ordinal it does not have. `None` here is
        // an arity disagreement, refused by the caller with the seat's own
        // coordinates -- never a seat that is exempt from having a contract.
        (
            Op::ConsoleWrite
            | Op::ConsoleFlush
            | Op::ConsoleIsTerminal
            | Op::FsReadFile
            | Op::FsWriteFile
            | Op::FsChangeMode
            | Op::FsOpen
            | Op::FsHandleMetadata
            | Op::FsReadAt
            | Op::FsWriteAt
            | Op::ResourceRelease
            | Op::BufferAllocate
            | Op::BufferFreeze,
            _,
        ) => None,
        // ⛔ The represented-UNAVAILABLE lanes, named rather than wildcarded.
        // They are refused before any seat is derived, so they have no seat
        // contract at all -- and naming them is what makes promoting one to the
        // admitted set a compile error here rather than an operation whose
        // seats silently answer `None`.
        (
            Op::ConsoleRead
            | Op::ClockWallNow
            | Op::ClockMonotonicNow
            | Op::ClockSleepUntil
            | Op::FsAppendFile
            | Op::FsMetadata
            | Op::FsReadDirectory
            | Op::FsCreateDirectory
            | Op::FsRemoveFile
            | Op::FsRemoveDirectory
            | Op::FsRename
            | Op::EntropyRandomBytes,
            _,
        ) => None,
    }
}

/// **Derive one record for every capability/argument seat of every admitted
/// host effect occurrence.**
///
/// ⛔ **The population is every `Effect` source occurrence, not the ones some
/// reached trace visited**, and within one occurrence it is every slot the
/// operation actually has — not the slots the arm this compilation took
/// happened to read.
///
/// ⭐ The order of the derivation is the correction this record exists to make.
/// `Need` comes from [`host_effect_seat_contract`], which is keyed on the
/// operation and the slot and knows nothing about how the value will be
/// represented. Only then is the seat's `Avail` checked to admit the phase the
/// consumer will see it in. Reading the need off the representation instead is
/// what makes whatever the emitter happens to offer into the definition of what
/// the wire request wanted.
pub(in crate::cranelift_backend::planning::static_transition) fn build_host_effect_seat_plan(
    plan: &StaticTransitionPlan<'_>,
) -> Result<Vec<PlannedEffectSeat>, CraneliftBackendError> {
    let mut records = Vec::new();
    for occurrence in plan.source_occurrences.iter().flatten() {
        let RuntimeExpr::Effect {
            operation,
            capability,
            args,
            ..
        } = occurrence.expr
        else {
            continue;
        };
        // A represented-unavailable lane has no seats at all: it is refused
        // whole, before any slot of it is derived.
        if !CRANELIFT_HOST_EFFECT_CONSUMERS_V1.contains(operation) {
            continue;
        }
        let effect_origin = occurrence.static_origin;
        let authority = occurrence_authority(plan, effect_origin)?;
        let consumer_owner = authority.owner;
        let argument_base = u32::from(capability.is_some());
        let slots = capability
            .iter()
            .map(|_| EffectSeatSlot::Capability)
            .chain((0..args.len()).map(|ordinal| {
                EffectSeatSlot::Argument(
                    u32::try_from(ordinal).expect("an argument list shorter than u32::MAX"),
                )
            }));
        for slot in slots {
            let position = match slot {
                EffectSeatSlot::Capability => 0,
                EffectSeatSlot::Argument(ordinal) => argument_base
                    .checked_add(ordinal)
                    .ok_or_else(|| planner_capacity_error("effect seat position overflows"))?,
            };
            let child = authority
                .children
                .get(position as usize)
                .ok_or_else(|| planner_error("a host effect seat has no child occurrence"))?;
            // ⛔ `Need` FIRST, from the seat's own semantics.
            let Some((semantic_operation, need, avail)) =
                host_effect_seat_contract(*operation, slot)
            else {
                return Err(planner_error(format!(
                    "host operation {:?} has no seat contract at {slot:?}, so the occurrence's \
                     shape and the operation's wire request disagree",
                    operation
                )));
            };
            let record = PlannedEffectSeat {
                effect_origin,
                child_origin: child.origin,
                position,
                operation: *operation,
                slot,
                producer_owner: child.owner,
                consumer_owner,
                semantic_operation,
                need,
                avail,
            };
            #[cfg(test)]
            let record = mutate_planned_effect_seat(record);
            records.push(record);
        }
    }
    records.sort();
    Ok(records)
}

#[cfg(test)]
fn mutate_planned_effect_seat(record: PlannedEffectSeat) -> PlannedEffectSeat {
    let tag = (
        EffectSeatOperation::SelectClosedTag,
        EffectSeatNeed::ConstructorTag,
        EffectSeatAvail::SPECIALIZED_ONLY,
    );
    match EFFECT_SEAT_PLAN_MUTATION.with(std::cell::Cell::get) {
        EffectSeatPlanMutation::Exact => record,
        EffectSeatPlanMutation::EraseOperation => PlannedEffectSeat {
            operation: CRANELIFT_HOST_EFFECT_CONSUMERS_V1[0],
            ..record
        },
        EffectSeatPlanMutation::EraseOrdinal => PlannedEffectSeat {
            slot: match record.slot {
                EffectSeatSlot::Capability => EffectSeatSlot::Capability,
                EffectSeatSlot::Argument(_) => EffectSeatSlot::Argument(0),
            },
            ..record
        },
        EffectSeatPlanMutation::EraseNeed => PlannedEffectSeat {
            need: EffectSeatNeed::ConstructorTag,
            ..record
        },
        EffectSeatPlanMutation::CollapseContract => PlannedEffectSeat {
            semantic_operation: tag.0,
            need: tag.1,
            avail: tag.2,
            ..record
        },
    }
}

/// **The contract one operation/slot pair has, recomputed from nothing but the
/// pair.**
///
/// ⭐ This is the INDEPENDENT side of the seat authority's contract half. The
/// planned population records a semantic operation, a need and an availability;
/// this recomputes them at the close from the two key axes alone. Without it
/// `need` would be diagnostic text — nothing would read it, so erasing it would
/// change no decision and no gate could see the erasure.
pub(in crate::cranelift_backend) fn host_effect_seat_contract_of(
    operation: ken_host::HostOpV1,
    slot: EffectSeatSlot,
) -> Option<(EffectSeatOperation, EffectSeatNeed, EffectSeatAvail)> {
    host_effect_seat_contract(operation, slot)
}

/// Every record names a DISTINCT seat.
///
/// The non-aliasing law of the seat domain, in production rather than in a
/// test, for the same reason the aggregate producers have one: if two records
/// shared `(effect_origin, slot)`, one seat's contract could authorize
/// another's consumption.
fn validate_host_effect_seats_are_unique(
    records: &[PlannedEffectSeat],
) -> Result<(), CraneliftBackendError> {
    let mut seen = BTreeSet::new();
    for record in records {
        if !seen.insert((record.effect_origin, record.slot)) {
            return Err(planner_error(
                "two host effect seat records name the same occurrence slot, so a seat identity \
                 is not unique",
            ));
        }
    }
    Ok(())
}

pub(in crate::cranelift_backend::planning::static_transition) fn validate_host_effect_seat_plan(
    plan: &StaticTransitionPlan<'_>,
    records: &[PlannedEffectSeat],
) -> Result<(), CraneliftBackendError> {
    if records != build_host_effect_seat_plan(plan)? {
        return Err(planner_error(
            "the host effect seat population is not the exact closed seat-contract derivation",
        ));
    }
    validate_host_effect_seats_are_unique(records)
}

impl<'src> StaticTransitionPlan<'src> {
    /// The finite artifact-static dispatcher for an exact constructor-tag seat.
    ///
    /// `None` means the operation/slot is not a constructor-tag consumer. An
    /// empty vector means it is such a consumer but this artifact interned no
    /// matching constructor identity, so a carried value has no authority to
    /// enter the route. Every lookup is unique-or-error; there is no first-match
    /// or family fallback.
    pub(in crate::cranelift_backend) fn host_effect_constructor_dispatch(
        &self,
        operation: ken_host::HostOpV1,
        slot: EffectSeatSlot,
    ) -> Result<Option<Vec<EffectSeatConstructorPath>>, CraneliftBackendError> {
        use ken_host::HostOpV1 as Op;
        use EffectSeatConstructorPath::{PositionalChild, Root};
        use EffectSeatConstructorRole as Role;

        let identity = |role: Role| {
            self.semantic
                .source_constructor_identity_with_suffix(role.suffix())
        };
        let roots = |roles: &[(Role, i64)]| -> Result<Vec<_>, CraneliftBackendError> {
            let mut paths = Vec::new();
            for (role, wire_tag) in roles {
                if let Some(identity) = identity(*role)? {
                    paths.push(Root {
                        identity,
                        field_count: 0,
                        wire_tag: *wire_tag,
                    });
                }
            }
            Ok(paths)
        };

        let paths = match (operation, slot) {
            (
                Op::ConsoleWrite | Op::ConsoleFlush | Op::ConsoleIsTerminal,
                EffectSeatSlot::Argument(0),
            ) => roots(&[
                (Role::StreamStdin, 0),
                (Role::StreamStdout, 1),
                (Role::StreamStderr, 2),
            ])?,
            (Op::FsWriteFile, EffectSeatSlot::Argument(1)) => roots(&[
                (Role::CreateNew, 0),
                (Role::CreateOrTruncate, 1),
                (Role::CreateOrKeep, 2),
            ])?,
            (Op::FsOpen, EffectSeatSlot::Argument(1)) => {
                let mut paths = roots(&[(Role::ResourceRead, 0), (Role::ResourceMetadata, 1)])?;
                if let Some(root_identity) = identity(Role::ResourceWriteCreate)? {
                    for (role, wire_tag) in [
                        (Role::CreateNew, 2),
                        (Role::CreateOrTruncate, 3),
                        (Role::CreateOrKeep, 4),
                    ] {
                        if let Some(child_identity) = identity(role)? {
                            paths.push(PositionalChild {
                                root_identity,
                                root_field_count: 1,
                                child_position: 0,
                                child_identity,
                                child_field_count: 0,
                                wire_tag,
                            });
                        }
                    }
                }
                paths
            }
            _ => return Ok(None),
        };
        Ok(Some(paths))
    }

    /// The closed planned seat population, for the whole-pass seat closeout.
    pub(in crate::cranelift_backend) fn host_effect_seat_records(&self) -> &[PlannedEffectSeat] {
        &self.host_effect_seats
    }

    /// **The planned slot population of ONE effect occurrence.**
    ///
    /// ⛔ This is what a visit's completeness is measured against, so it is
    /// derived from the population rather than from the occurrence's argument
    /// list: a visit that read every argument it happened to lower would be
    /// complete by construction.
    pub(in crate::cranelift_backend) fn host_effect_seat_slots(
        &self,
        effect_origin: StaticOriginId,
    ) -> BTreeSet<EffectSeatSlot> {
        self.host_effect_seats
            .iter()
            .filter(|record| record.effect_origin == effect_origin)
            .map(|record| record.slot)
            .collect()
    }

    /// The exact argument slots that the operation's planned synthesized
    /// result tree consumes as site-bound operands.
    ///
    /// This is deliberately occurrence-keyed even though the recipe is chosen
    /// by operation: the emitter asks about the effect occurrence it is
    /// lowering, and a non-effect coordinate must refuse rather than borrow an
    /// operation from elsewhere.
    pub(in crate::cranelift_backend) fn host_effect_site_operand_slots(
        &self,
        effect_origin: StaticOriginId,
    ) -> Result<BTreeSet<EffectSeatSlot>, CraneliftBackendError> {
        let operation = self.host_effect_operation(effect_origin)?;
        let tree = host_effect_recipe_tree(operation);
        let mut ordinals = BTreeSet::new();
        collect_site_operand_ordinals(tree.error, &mut ordinals);
        collect_site_operand_ordinals(tree.ok, &mut ordinals);
        Ok(ordinals.into_iter().map(EffectSeatSlot::Argument).collect())
    }

    /// **Claim the ONE planned record for an exact seat.**
    ///
    /// ⛔ Keyed on the occurrence and the slot, never on the operation alone: a
    /// lookup by operation would answer for whichever occurrence of that
    /// operation came first, so one effect's proof would authorize another's
    /// consumption. A seat with no record is a loud refusal, not a fallback —
    /// it means the emitter reached a seat planning never derived.
    pub(in crate::cranelift_backend) fn host_effect_seat(
        &self,
        effect_origin: StaticOriginId,
        slot: EffectSeatSlot,
    ) -> Result<PlannedEffectSeat, CraneliftBackendError> {
        self.host_effect_seats
            .iter()
            .find(|record| record.effect_origin == effect_origin && record.slot == slot)
            .copied()
            .ok_or_else(|| {
                planner_error(format!(
                    "host effect occurrence {effect_origin:?} has no planned seat at {slot:?}"
                ))
            })
    }




    /// The host operation of one `Effect` seat.
    pub(in crate::cranelift_backend::planning::static_transition) fn host_effect_operation(
        &self,
        seat: StaticOriginId,
    ) -> Result<ken_host::HostOpV1, CraneliftBackendError> {
        let occurrence = self
            .source_occurrences
            .get(seat.0 as usize)
            .and_then(|slot| slot.as_ref())
            .ok_or_else(|| planner_error("synthesized aggregate seat is not an occurrence"))?;
        match occurrence.expr {
            RuntimeExpr::Effect { operation, .. } => Ok(*operation),
            _ => Err(planner_error(
                "synthesized aggregate seat is not a host effect",
            )),
        }
    }
}
