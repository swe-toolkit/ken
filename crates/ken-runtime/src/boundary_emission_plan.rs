//! Compiler-private plan derived from the lowering representation authority.

use crate::boundary_value::{
    BoundaryClass, BoundaryReferentOwner, BoundaryStorageShape, BoundaryTag, BOUNDARY_RETIRED_LANES,
};

/// The emission plan — the representation authority, reduced to what the
/// emitter needs in order to *generate* the helper bodies.
///
/// ⛔ **Computed once at the `lowering/core` → `emit_boundary_value_local_graph`
/// seam and passed in.** It is deliberately data-only: the derivation lives
/// with the compiler's `LoweredVariant`/`BoundaryInput` authority, which is
/// the only place that can see it; the emitter consumes the plan but cannot
/// restate it.
///
/// ⚠ **It carries no seed value and no sampled runtime value** — only the
/// finite class sets the partition admits. A representation chosen by
/// inspecting a value describes a program that cannot be written (`D1`/`AC-2`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BoundaryEmissionPlan {
    int_magnitude_classes: Vec<BoundaryClass>,
    byte_span_classes: Vec<BoundaryClass>,
    tags: BoundaryTagAdmission,
}

/// The tag half of the plan: which tags the partition admits, split by the
/// distinctions the emitted helpers actually branch on.
///
/// ⛔ **Sets, never ordinal bands.** The emitter previously asked *"is this tag
/// numerically at or below `LAST_PERSISTENT_TAG`"*, which is a second authority
/// derived by hand from [`BoundaryTag`]'s declaration order: reordering the
/// enum leaves both constants well-formed and silently re-points every
/// persistent word at the invocation arena. A set has no such failure mode, and
/// it does not require the admitted tags to be contiguous in the first place.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BoundaryTagAdmission {
    admitted: Vec<BoundaryTag>,
    immediate: Vec<BoundaryTag>,
    handle: Vec<BoundaryTag>,
    owner_bands: Vec<(BoundaryReferentOwner, Vec<BoundaryTag>)>,
    immediate_value_classes: Vec<(BoundaryTag, BoundaryClass)>,
    handle_class_relation: Vec<(BoundaryTag, Vec<BoundaryClass>)>,
}

impl BoundaryTagAdmission {
    /// Build the tag admission from already-derived sets.
    pub(crate) fn new(
        admitted: Vec<BoundaryTag>,
        immediate: Vec<BoundaryTag>,
        handle: Vec<BoundaryTag>,
        owner_bands: Vec<(BoundaryReferentOwner, Vec<BoundaryTag>)>,
        immediate_value_classes: Vec<(BoundaryTag, BoundaryClass)>,
        handle_class_relation: Vec<(BoundaryTag, Vec<BoundaryClass>)>,
    ) -> Self {
        BoundaryTagAdmission {
            admitted,
            immediate,
            handle,
            owner_bands,
            immediate_value_classes,
            handle_class_relation,
        }
    }

    /// Every tag any admitted outcome can carry. A tag outside this set is the
    /// third outcome that fails, never a fall-through.
    pub(crate) fn admitted(&self) -> &[BoundaryTag] {
        &self.admitted
    }

    /// The tags whose payload is the value itself.
    pub(crate) fn immediate(&self) -> &[BoundaryTag] {
        &self.immediate
    }

    /// The tags whose payload indexes a node.
    pub(crate) fn handle(&self) -> &[BoundaryTag] {
        &self.handle
    }

    /// Each referent owner the partition publishes handles for, paired with
    /// exactly the tags it publishes under that owner.
    ///
    /// ⛔ **A relation, not a two-way split.** The emitter used to assume there
    /// are exactly two handle owners and discriminate them with one threshold;
    /// an owner the partition started admitting would have been silently folded
    /// into whichever side of that threshold its tag landed on.
    pub(crate) fn owner_bands(&self) -> &[(BoundaryReferentOwner, Vec<BoundaryTag>)] {
        &self.owner_bands
    }

    /// Each admitted immediate tag paired with the class the `class` helper
    /// must report for it.
    ///
    /// ⛔ **Not a node class.** See [`BoundaryTag::immediate_value_class`] for
    /// why this is a separate contract from [`BOUNDARY_TAG_CLASS_RELATION`]. A
    /// tag absent from this relation has no classification, and the emitted
    /// helper fails closed on it rather than defaulting.
    pub(crate) fn immediate_value_classes(&self) -> &[(BoundaryTag, BoundaryClass)] {
        &self.immediate_value_classes
    }

    /// The normalized handle `BoundaryTag → set<BoundaryClass>` relation — what
    /// may be written into a **node's** `NODE_CLASS`.
    ///
    /// ⛔ **This is the emitted allocator's sole authority for the relation**,
    /// and it is derived from `BoundaryOutcome::HandleWord` in the one
    /// partition sweep. `ImmediateWord` is excluded by construction: an
    /// immediate has no node, so it has no node class. A tag with no row here
    /// admits nothing, and the allocator's fold is seeded with the empty mask so
    /// that absence is `BOUNDARY_ERR_RELATION` on its own rather than something
    /// an earlier guard has to make unreachable.
    pub(crate) fn handle_class_relation(&self) -> &[(BoundaryTag, Vec<BoundaryClass>)] {
        &self.handle_class_relation
    }

    /// The tags published under one owner — empty if the partition publishes
    /// none, which is a legitimate answer and not a missing entry.
    pub(crate) fn tags_owned_by(&self, owner: BoundaryReferentOwner) -> &[BoundaryTag] {
        self.owner_bands
            .iter()
            .find(|(band, _)| *band == owner)
            .map(|(_, tags)| tags.as_slice())
            .unwrap_or(&[])
    }
}

impl BoundaryEmissionPlan {
    /// Build a plan from already-derived class and tag sets.
    ///
    /// ⛔ There is deliberately **no** whole-admitted-class set here. One was
    /// carried for a while and no emitted helper ever read it — `rustc` said so
    /// on every lib build (`method admitted_classes is never used`). A derived
    /// set with no production consumer is a declaration, and `RULING R3` is
    /// explicit that a declaration does not discharge the predicate. The
    /// per-storage-shape sets below are the ones the emitter actually uses.
    ///
    /// The sole production caller is the derivation in the compiler's lowering
    /// module. This constructor crosses the crate boundary; no run-time path
    /// synthesizes an emission plan.
    pub(crate) fn new(
        int_magnitude_classes: Vec<BoundaryClass>,
        byte_span_classes: Vec<BoundaryClass>,
        tags: BoundaryTagAdmission,
    ) -> Self {
        BoundaryEmissionPlan {
            int_magnitude_classes,
            byte_span_classes,
            tags,
        }
    }

    /// The tag sets the emitted helpers branch on.
    pub(crate) fn tags(&self) -> &BoundaryTagAdmission {
        &self.tags
    }

    /// The classes a limb-storage helper may touch.
    pub(crate) fn int_magnitude_classes(&self) -> &[BoundaryClass] {
        &self.int_magnitude_classes
    }

    /// The classes a byte-span helper may touch.
    pub(crate) fn byte_span_classes(&self) -> &[BoundaryClass] {
        &self.byte_span_classes
    }
}

/// The tags that are **recognized but carry no admitted lane**, given the
/// partition's admitted tag set (`RT-FNSPLIT-C1` `D5`).
///
/// ⛔ **Derived from BOTH authorities at every call site, never written down.**
/// A tag is retired exactly when it names a retired lane *and* the live
/// partition admits it nowhere — so a tag that still has one surviving admitted
/// lane is **not** reported here, because such a tag is genuinely admitted and
/// refusing it by name would be the inverse error.
///
/// ⭐ **Why this is a function of the plan and not a seventh field on
/// [`BoundaryTagAdmission`].** Every emitted helper already holds the plan's
/// admitted set; taking it as an argument means each mutation fixture derives
/// its retired set from *its own* admitted set rather than from a hand-written
/// list that would drift into a second authority — which is the defect the
/// whole partition-derived plan exists to avoid.
pub(crate) fn boundary_retired_tags(admitted: &[BoundaryTag]) -> Vec<BoundaryTag> {
    let mut tags: Vec<BoundaryTag> = Vec::new();
    for (retired_tag, _) in BOUNDARY_RETIRED_LANES {
        if !admitted.contains(retired_tag) && !tags.contains(retired_tag) {
            tags.push(*retired_tag);
        }
    }
    tags
}
