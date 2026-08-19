//! Occurrence minting, source/child correspondence, and the
//! occurrence-keyed read views over the static transition plan.
//!
//! `RT-PLANNER-OCCURRENCES-SPLIT` `D1` — this module owns
//! `StaticOriginId` (moved from `semantic_ir`) and the planner-side
//! occurrence records, validations and read views (moved from the
//! parent). `StaticTransitionPlan` stays in the parent; the inherent
//! impls here read ancestor-private root state under the standing
//! child-module pattern (item 4's `units.rs` precedent).

use std::collections::BTreeSet;

use super::semantic_ir::SemanticSourceKind;
use super::{
    planner_capacity_error, planner_error, runtime_value_lifetime,
    CraneliftBackendError, PlannedReferentLifetime, Planner, PredeclaredFunctionId,
    StaticNodeId, StaticTransitionPlan,
};
#[cfg(test)]
use super::AC4_RESOLUTIONS;
use crate::RuntimeExpr;

/// The preallocated positional identity of one planned occurrence.
///
/// Widened to `pub(in crate::cranelift_backend)` so the
/// lowering can carry an occurrence's static name to the site that lowers it.
/// The wrapped ordinal stays `pub(super)` deliberately: a consumer outside this
/// planner can hold, compare, and pass an origin, but **cannot mint one** from
/// an arbitrary integer, so the tag population can only ever be the planner's
/// own.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub(in crate::cranelift_backend) struct StaticOriginId(pub(super) u32);

/// One planned source occurrence: the borrowed term, paired with the origin the
/// planner gave it in the very same visit.
///
/// ⭐ The origin is stored **beside** the term rather than left implicit in the
/// table position. A dense table whose entries only ever say "whatever lives at
/// this index" cannot detect an entry written at the wrong index; storing the
/// origin makes that failure observable, and `source_occurrence` rejects it
/// instead of returning a plausible wrong body.
#[derive(Clone, Copy)]
pub(super) struct PlannedOccurrence<'src> {
    pub(super) static_origin: StaticOriginId,
    pub(super) expr: &'src RuntimeExpr,
}

/// `D2`: exact occurrence, owner and referent-lifetime authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PlannedOccurrenceChildAuthority {
    pub(super) origin: StaticOriginId,
    pub(super) position: u32,
    pub(super) owner: PredeclaredFunctionId,
    pub(super) lifetime: PlannedReferentLifetime,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PlannedOccurrenceAuthority {
    pub(super) origin: StaticOriginId,
    pub(super) owner: PredeclaredFunctionId,
    pub(super) lifetime: PlannedReferentLifetime,
    pub(super) children: Vec<PlannedOccurrenceChildAuthority>,
}

/// The occurrence origin of a node the planner has just allocated a semantic
/// seed for.
///
/// ⛔ Formed **only** inside the planner. `StaticOriginId`'s ordinal is
/// planner-private precisely so no consumer outside this module can mint one, and
/// this is the single function that does.
pub(super) fn origin_of(node: StaticNodeId) -> StaticOriginId {
    StaticOriginId(node.0)
}

pub(super) fn derive_occurrence_lifetime(
    plan: &StaticTransitionPlan<'_>,
    origin: StaticOriginId,
    environment: &[PlannedReferentLifetime],
) -> Result<PlannedReferentLifetime, CraneliftBackendError> {
    let expr = plan.planned_occurrence_expr(origin)?;
    let child = |position| plan.semantic.child_origin(origin, position);
    let lifetime = match expr {
        RuntimeExpr::CheckedJoinSite { .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { .. }
        | RuntimeExpr::CheckedRecursiveInvocation { .. }
        | RuntimeExpr::CheckedComputationalIHSlots { .. }
        | RuntimeExpr::CheckedComputationalIHInvocation { .. } => {
            derive_occurrence_lifetime(plan, child(0)?, environment)?
        }
        RuntimeExpr::Value(value) => runtime_value_lifetime(value),
        RuntimeExpr::Var(index) => environment
            .get(*index as usize)
            .copied()
            .unwrap_or(PlannedReferentLifetime::ActivationOwned),
        RuntimeExpr::Let { .. } => {
            let value = derive_occurrence_lifetime(plan, child(0)?, environment)?;
            let mut nested = Vec::with_capacity(environment.len() + 1);
            nested.push(value);
            nested.extend_from_slice(environment);
            derive_occurrence_lifetime(plan, child(1)?, &nested)?
        }
        RuntimeExpr::If { .. } => derive_occurrence_lifetime(plan, child(1)?, environment)?
            .max(derive_occurrence_lifetime(plan, child(2)?, environment)?),
        RuntimeExpr::Construct { args, .. } => (0..args.len()).try_fold(
            PlannedReferentLifetime::Persistent,
            |lifetime, position| {
                Ok(lifetime.max(derive_occurrence_lifetime(
                    plan,
                    child(position)?,
                    environment,
                )?))
            },
        )?,
        RuntimeExpr::Record { fields } => (0..fields.len()).try_fold(
            PlannedReferentLifetime::Persistent,
            |lifetime, position| {
                Ok(lifetime.max(derive_occurrence_lifetime(
                    plan,
                    child(position)?,
                    environment,
                )?))
            },
        )?,
        RuntimeExpr::Match { cases, .. } => {
            let scrutinee = derive_occurrence_lifetime(plan, child(0)?, environment)?;
            let mut lifetime = PlannedReferentLifetime::Persistent;
            for (index, case) in cases.iter().enumerate() {
                let mut nested = Vec::with_capacity(case.binders + environment.len());
                nested.extend((0..case.binders).map(|_| scrutinee));
                nested.extend_from_slice(environment);
                lifetime = lifetime.max(derive_occurrence_lifetime(
                    plan,
                    child(1 + index)?,
                    &nested,
                )?);
            }
            lifetime
        }
        RuntimeExpr::ComputationalMatch { cases, .. } => {
            let scrutinee = derive_occurrence_lifetime(plan, child(0)?, environment)?;
            let mut lifetime = PlannedReferentLifetime::Persistent;
            for (index, case) in cases.iter().enumerate() {
                let mut nested = Vec::with_capacity(
                    case.argument_binders + case.recursive_positions.len() + environment.len(),
                );
                nested.extend(
                    (0..case.recursive_positions.len())
                        .map(|_| PlannedReferentLifetime::ActivationOwned),
                );
                nested.extend((0..case.argument_binders).map(|_| scrutinee));
                nested.extend_from_slice(environment);
                lifetime = lifetime.max(derive_occurrence_lifetime(
                    plan,
                    child(1 + index)?,
                    &nested,
                )?);
            }
            lifetime
        }
        RuntimeExpr::Project { .. } => derive_occurrence_lifetime(plan, child(0)?, environment)?,
        RuntimeExpr::Trap(_) => PlannedReferentLifetime::Persistent,
        RuntimeExpr::PrimitiveCall { .. }
        | RuntimeExpr::Closure { .. }
        | RuntimeExpr::LexicalClosure { .. }
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. }
        | RuntimeExpr::Call { .. }
        | RuntimeExpr::Effect { .. } => PlannedReferentLifetime::ActivationOwned,
    };
    Ok(lifetime)
}

pub(super) fn build_occurrence_authority_plan(
    plan: &StaticTransitionPlan<'_>,
) -> Result<Vec<PlannedOccurrenceAuthority>, CraneliftBackendError> {
    let mut records = Vec::new();
    for occurrence in plan.source_occurrences.iter().flatten() {
        let origin = occurrence.static_origin;
        let owner = plan
            .semantic
            .function_owner(origin)?
            .ok_or_else(|| planner_error("source occurrence has no function owner"))?;
        let children = plan
            .semantic
            .child_origins(origin)?
            .iter()
            .copied()
            .enumerate()
            .map(|(position, child_origin)| {
                Ok(PlannedOccurrenceChildAuthority {
                    origin: child_origin,
                    position: u32::try_from(position).map_err(|_| {
                        planner_capacity_error("occurrence child position exhausted")
                    })?,
                    owner: plan
                        .semantic
                        .function_owner(child_origin)?
                        .ok_or_else(|| planner_error("source child has no function owner"))?,
                    lifetime: derive_occurrence_lifetime(plan, child_origin, &[])?,
                })
            })
            .collect::<Result<Vec<_>, CraneliftBackendError>>()?;
        records.push(PlannedOccurrenceAuthority {
            origin,
            owner,
            lifetime: derive_occurrence_lifetime(plan, origin, &[])?,
            children,
        });
    }
    records.sort_by_key(|record| record.origin);
    Ok(records)
}

pub(super) fn validate_occurrence_authority_plan(
    plan: &StaticTransitionPlan<'_>,
    records: &[PlannedOccurrenceAuthority],
) -> Result<(), CraneliftBackendError> {
    if records != build_occurrence_authority_plan(plan)? {
        return Err(planner_error(
            "dormant occurrence authority is not exact for origin, owner and lifetime",
        ));
    }
    Ok(())
}

/// Whether `needle` lies in the occurrence subtree rooted at `root`.
///
/// Bounded by the occurrence population rather than by a depth budget: the
/// walk visits each origin at most once, so a malformed cyclic child relation
/// terminates instead of recurring.
pub(super) fn occurrence_subtree_contains(
    plan: &StaticTransitionPlan<'_>,
    root: StaticOriginId,
    needle: StaticOriginId,
) -> Result<bool, CraneliftBackendError> {
    let mut seen = BTreeSet::new();
    let mut stack = vec![root];
    while let Some(origin) = stack.pop() {
        if origin == needle {
            return Ok(true);
        }
        if !seen.insert(origin) {
            continue;
        }
        if let Ok(children) = plan.semantic.child_origins(origin) {
            stack.extend(children.iter().copied());
        }
    }
    Ok(false)
}

pub(super) fn occurrence_authority<'plan>(
    plan: &'plan StaticTransitionPlan<'_>,
    origin: StaticOriginId,
) -> Result<&'plan PlannedOccurrenceAuthority, CraneliftBackendError> {
    plan.occurrence_authorities
        .iter()
        .find(|authority| authority.origin == origin)
        .ok_or_else(|| planner_error("continuation input has no occurrence authority"))
}

impl<'src> Planner<'src> {

    /// Files this occurrence's term under the origin the planner just gave it.
    ///
    /// ⭐ This is deliberately the *same* function that emits the semantic seed,
    /// not a companion pass: the term and its static name are recorded in one
    /// step, so no ordering between two walks can put them out of agreement. Every
    /// planned occurrence reaches `expression_seed`, so the table is total over
    /// the occurrence population by construction.
    pub(super) fn record_source_occurrence(
        &mut self,
        node: StaticNodeId,
        expr: &'src RuntimeExpr,
    ) -> Result<(), CraneliftBackendError> {
        let index = node.0 as usize;
        if self.plan.source_occurrences.len() <= index {
            self.plan.source_occurrences.resize(index + 1, None);
        }
        // A second occurrence filed under one origin would make selection
        // ambiguous, and ambiguity here is a compiler bug rather than a program
        // the backend cannot handle — so it is a `PlannerInvariant`, not a
        // capacity refusal (`RT-PLANNER-ATTRIB-K`).
        if self.plan.source_occurrences[index].is_some() {
            return Err(planner_error(
                "static origin was given more than one source occurrence",
            ));
        }
        self.plan.source_occurrences[index] = Some(PlannedOccurrence {
            static_origin: origin_of(node),
            expr,
        });
        Ok(())
    }

}

impl<'src> StaticTransitionPlan<'src> {

    /// Planner-private source lookup for pre-allocation derivations.
    ///
    /// This intentionally does not increment `AC4_RESOLUTIONS`: that counter
    /// measures the capability crossing into lowering, while Slice 0 runs
    /// wholly inside the planner before such a consumer exists.
    pub(super) fn planned_occurrence_expr(
        &self,
        static_origin: StaticOriginId,
    ) -> Result<&'src RuntimeExpr, CraneliftBackendError> {
        let index = static_origin.0 as usize;
        let occurrence = self
            .source_occurrences
            .get(index)
            .and_then(Option::as_ref)
            .ok_or_else(|| planner_error("planner derivation names no source occurrence"))?;
        if occurrence.static_origin != static_origin {
            return Err(planner_error(
                "planner derivation occurrence disagrees with its table position",
            ));
        }
        Ok(occurrence.expr)
    }

    /// Resolves a static origin to the source term the planner filed under it.
    ///
    /// ⭐ **This is the sole `origin -> expression` route in the backend**, and it
    /// is what `RT-FNSPLIT-B2A-C`'s N3 pin asserted did not exist. B2A-S retires
    /// that pin deliberately: the count goes from zero to **exactly one**, so a
    /// retained body is selected by its static name and by nothing else.
    ///
    /// Three distinct ways to be wrong, each rejected separately so a mutation
    /// that breaks one is distinguishable from a mutation that breaks another:
    ///
    /// 1. an origin past the end of the table — outside the planned population;
    /// 2. an origin naming a planned node that is **not** a source occurrence
    ///    (a control node), whose slot is legitimately empty;
    /// 3. an entry whose **stored** origin disagrees with the index it was found
    ///    at — the table itself is corrupt, and returning a term from a
    ///    mis-indexed entry is exactly the wrong-body substitution this WP
    ///    exists to make impossible.
    ///
    /// ⛔ The returned lifetime is the **plan's** `'src`, not `&self`'s: the
    /// borrow outlives this call, which is what lets a `&mut self` lowering step
    /// resolve a tag and then lower the result. That is also why the plan cannot
    /// escape — see `Lowering::static_transition_plan`.
    pub(in crate::cranelift_backend) fn source_occurrence(
        &self,
        static_origin: StaticOriginId,
    ) -> Result<&'src RuntimeExpr, CraneliftBackendError> {
        // ⭐ Counted at ENTRY, not on the success path. A resolution that fails
        // is still a resolution *attempted through this route*, and `AC-4` is a
        // claim about routes, not about outcomes — counting only successes would
        // let a second caller hide behind a bad origin.
        #[cfg(test)]
        AC4_RESOLUTIONS.with(|cell| cell.set(cell.get() + 1));
        let index = static_origin.0 as usize;
        let slot = self.source_occurrences.get(index).ok_or_else(|| {
            planner_error("static origin is outside the planned occurrence table")
        })?;
        let occurrence = slot
            .as_ref()
            .ok_or_else(|| planner_error("static origin names no planned source occurrence"))?;
        if occurrence.static_origin != static_origin {
            return Err(planner_error(
                "planned occurrence's stored origin disagrees with its table position",
            ));
        }
        Ok(occurrence.expr)
    }

    /// Test-only typed recovery of one source occurrence by its asserted
    /// ordinal. The production lowering cannot mint a [`StaticOriginId`] from
    /// an integer; this diagnostic likewise returns only an identity the plan
    /// already contains.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn source_occurrence_origin_at_ordinal_for_test(
        &self,
        ordinal: usize,
    ) -> Option<StaticOriginId> {
        let occurrence = self.source_occurrences.get(ordinal)?.as_ref()?;
        (occurrence.static_origin.0 as usize == ordinal).then_some(occurrence.static_origin)
    }

    /// The preallocated origin of one positional syntax child of `parent`.
    ///
    /// This is the **sole** production point for a child's static name, and the
    /// only admissible one: the position is the child's source-field ordinal and
    /// the value comes out of B1R's checked positional child-origin range. There
    /// is deliberately no pointer, content, hash, clone-order, or visit-order
    /// route to an origin, and no arithmetic that mints one
    ///.
    pub(in crate::cranelift_backend) fn child_static_origin(
        &self,
        parent: StaticOriginId,
        position: usize,
    ) -> Result<StaticOriginId, CraneliftBackendError> {
        self.semantic.child_origin(parent, position)
    }

    /// The **occurrence** origin of the whole program's root.
    ///
    /// Read from the value stored during the root's own planning visit (D9), not
    /// derived from `entries.first()` — that is a *scheduling* entry, and for a
    /// root whose body is a `ComputationalMatch` it names the scrutinee.
    pub(in crate::cranelift_backend) fn root_static_origin(
        &self,
    ) -> Result<StaticOriginId, CraneliftBackendError> {
        self.root_occurrence
            .ok_or_else(|| planner_error("plan has no root occurrence"))
    }

    /// The occurrence table's three properties, each as its own failure.
    ///
    /// ⛔ Deliberately **not** one composite check. A single "the table is fine"
    /// assertion is discharged by any one of these holding, so a mutation that
    /// breaks exactly one would still be reported as the same failure; three
    /// named failures make three different mutations distinguishable.
    ///
    /// The cross-check is against `semantic_sources`, a population produced by a
    /// *different* mechanism in the same visit. Checking the table against itself
    /// could only ever confirm its internal shape — it could not notice that an
    /// occurrence the planner registered is missing from it.
    pub(super) fn validate_source_occurrence_table(&self) -> Result<(), CraneliftBackendError> {
        // 1. Self-consistency: an entry's stored origin is the index it sits at.
        for (index, slot) in self.source_occurrences.iter().enumerate() {
            let Some(occurrence) = slot else {
                continue;
            };
            if occurrence.static_origin.0 as usize != index {
                return Err(planner_error(
                    "occurrence table entry is filed under an origin that is not its index",
                ));
            }
        }

        // 2. Totality over the occurrence population: every expression seed the
        //    walk registered has an entry, filed under that seed's own origin.
        let mut expression_seeds = 0usize;
        for seed in &self.semantic_sources {
            if !matches!(seed.source, SemanticSourceKind::Expression(_)) {
                continue;
            }
            expression_seeds += 1;
            let filed = self
                .source_occurrences
                .get(seed.origin.0 as usize)
                .and_then(|slot| slot.as_ref())
                .ok_or_else(|| {
                    planner_error("planned source occurrence is missing from the occurrence table")
                })?;
            if filed.static_origin != seed.origin {
                return Err(planner_error(
                    "occurrence table entry does not match its semantic seed's origin",
                ));
            }
        }

        // 3. No surplus: the table holds nothing no seed accounts for. With (2)
        //    this is injectivity — one entry per registered occurrence, and no
        //    entry without one.
        let filed = self
            .source_occurrences
            .iter()
            .filter(|slot| slot.is_some())
            .count();
        if filed != expression_seeds {
            return Err(planner_error(
                "occurrence table holds an entry no semantic seed accounts for",
            ));
        }
        Ok(())
    }

}
