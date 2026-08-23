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
    CraneliftBackendError, PlannedReferentLifetime, PredeclaredFunctionId,
    StaticNodeId, StaticTransitionPlan,
};
use super::construction::Planner;
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

impl StaticOriginId {
    /// Construct an origin for a TEST only.
    ///
    /// The field stays `pub(super)` so production code outside this module
    /// cannot mint an origin the planner never issued -- that restriction is
    /// the point, and this does not relax it: the constructor is `cfg(test)`
    /// and compiled out of the real artifact.
    #[cfg(test)]
    pub(in crate::cranelift_backend) const fn for_test(id: u32) -> Self {
        Self(id)
    }
}

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
/// Does this literal value carry `constructor` anywhere inside it?
///
/// Recurses through every nesting a `RuntimeValue` admits, because a census
/// that stopped at the outermost value would miss a constructor buried in an
/// argument, a record field, or a closure capture -- and missing one is the
/// unsound direction for a deadness proof.
fn runtime_value_constructs(value: &crate::RuntimeValue, constructor: &crate::RuntimeSymbol) -> bool {
    match value {
        crate::RuntimeValue::Constructor {
            constructor: constructed,
            args,
        } => {
            constructed == constructor
                || args
                    .iter()
                    .any(|arg| runtime_value_constructs(arg, constructor))
        }
        crate::RuntimeValue::Record { fields } => fields
            .iter()
            .any(|(_, field)| runtime_value_constructs(field, constructor)),
        crate::RuntimeValue::ClosureRef { captured, .. } => captured
            .iter()
            .any(|capture| runtime_value_constructs(capture, constructor)),
        crate::RuntimeValue::Bool(_)
        | crate::RuntimeValue::Int(_)
        | crate::RuntimeValue::Bytes(_)
        | crate::RuntimeValue::String(_)
        // An opaque value is one this census cannot see into. It is part of the
        // residual the consumer's TRAP covers, not something to read as absence.
        | crate::RuntimeValue::Unknown => false,
    }
}

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

    /// **`RT-DEAD-ARM-EFFECT-LOWERING` `D1` -- the whole-program
    /// construction-site census: is this constructor EVER constructed?**
    ///
    /// Derived from `source_occurrences`, which is the same table lowering
    /// reads, so the census's coverage equals what the backend will lower **by
    /// construction** rather than by agreement between two walks. Every planned
    /// occurrence reaches `record_source_occurrence` (`construction.rs:393`,
    /// via `plan_expr`'s recursion over every child and every arm body), so the
    /// table is total over the occurrence population -- the property
    /// `validate_source_occurrence_table` already relies on.
    ///
    /// A hand-rolled parallel traversal is precisely where an
    /// **under**-approximation of the construction sites would hide, and an
    /// under-approximation is the unsound direction here: it would let an arm
    /// be called dead that a construction elsewhere makes live.
    ///
    /// **Keyed on the `RuntimeSymbol` itself.** `RuntimeMatchCase.constructor`
    /// and `RuntimeExpr::Construct.constructor` are the same type carrying the
    /// same spelling (`ir.rs:588`, `ir.rs:650`), so this is an exact match with
    /// no translation step that could disagree.
    pub(in crate::cranelift_backend) fn constructor_is_ever_constructed(
        &self,
        constructor: &crate::RuntimeSymbol,
    ) -> bool {
        self.source_occurrences
            .iter()
            .flatten()
            .any(|occurrence| match occurrence.expr {
                RuntimeExpr::Construct {
                    constructor: constructed,
                    ..
                } => constructed == constructor,
                // A constructor value does NOT only arise from `Construct`. It
                // can be a LITERAL, and literals nest: a constructor's own
                // arguments, a record's fields, and a closure reference's
                // captured values are all `RuntimeValue`s that may carry one.
                //
                // Missing this class was a real defect in the first cut of this
                // census, and it failed in the UNSOUND direction -- fewer
                // constructions seen means more arms wrongly proven dead. It was
                // caught by an existing lowering control whose fixture builds its
                // scrutinee as a literal, which is exactly the shape a
                // `Construct`-only walk cannot see.
                RuntimeExpr::Value(value) => runtime_value_constructs(value, constructor),
                _ => false,
            })
    }

    /// **Is `needle` inside a match arm whose constructor is never constructed
    /// program-wide?**
    ///
    /// The reachability half of `RT-DEAD-ARM-EFFECT-LOWERING`. An arm of a
    /// total handler is entered only when its constructor is the scrutinee's,
    /// so a constructor no occurrence anywhere constructs cannot select its
    /// arm, and nothing inside that arm can execute.
    ///
    /// **EXISTENTIAL over enclosing arms, deliberately, and it does not need
    /// the innermost one.** If ANY enclosing arm is dead then everything nested
    /// inside it is unreachable, so taking the first witness is sound and more
    /// enclosing arms can only help. Asking for the innermost would add a
    /// nesting order this predicate does not need and could get wrong.
    ///
    /// **TWO CONJUNCTS, and the second is the correction.** An arm is dead only
    /// when its constructor is BOTH (1) never built by program syntax AND (2)
    /// not producible by the runtime. Architect `evt_4hcny7ae7h9sb`.
    ///
    /// The first cut had only (1) and was **unsound in the LIVE direction**: it
    /// proved `ctor:prelude::Result::Ok` dead, because an effect RESPONSE is
    /// synthesized by the host and appears in no `Construct` and no literal. It
    /// would have trapped the success continuation of every effect. The
    /// request/response axis is what decides whether a syntactic census is
    /// sound at all: an `FSOp` REQUEST is program-built and stays in (1)'s
    /// scope; a RESPONSE is runtime-built and only (2) can see it.
    ///
    /// `runtime_producible` is supplied by the consumer because the runtime's
    /// symbol vocabulary lives there, and it is enumerated
    /// exhaustively-by-construction so a symbol added to that vocabulary is a
    /// COMPILE error until it is classified rather than a silent unsound trap.
    ///
    /// **Conservative direction.** Both conjuncts over-approximate liveness, so
    /// this under-approximates deadness: anything not proven dead answers
    /// `false` and keeps today's strict behaviour. The residual it still cannot
    /// see is why the substitute at the consumer is a TRAP -- an arm wrongly
    /// reported dead HALTS rather than yielding a wrong result.
    ///
    /// Walked on demand rather than cached. This runs only on a path that is
    /// already failing today, so the cost is paid once per refusal and there is
    /// no cached set that could go stale against the table it was derived from.
    pub(in crate::cranelift_backend) fn origin_is_in_provably_dead_arm(
        &self,
        needle: StaticOriginId,
        runtime_producible: &BTreeSet<crate::RuntimeSymbol>,
    ) -> Result<bool, CraneliftBackendError> {
        Ok(self
            .provably_dead_arm_body_containing(needle, runtime_producible)?
            .is_some())
    }

    /// **`RT-DEAD-ARM-JOIN-DISPOSITION` -- the same question, returning its
    /// WITNESS: the body of the dead arm that contains `needle`.**
    ///
    /// The predicate above is this, asked for existence. One scan, one set of
    /// conjuncts, one answer -- a second traversal that agreed "usually" is
    /// exactly how disposition and deadness would drift apart.
    ///
    /// The caller needs the witness, not the verdict: dispositioning a dead
    /// arm's joins requires knowing WHICH arm was proved dead, and re-deriving
    /// it at the consumer would be a second authority over deadness. Returning
    /// it here keeps `disposition-follows-deadness` true BY CONSTRUCTION -- the
    /// only arm whose joins can be dispositioned is one this predicate already
    /// proved dead, because it is the only arm the caller is ever handed.
    pub(in crate::cranelift_backend) fn provably_dead_arm_body_containing(
        &self,
        needle: StaticOriginId,
        runtime_producible: &BTreeSet<crate::RuntimeSymbol>,
    ) -> Result<Option<StaticOriginId>, CraneliftBackendError> {
        for occurrence in self.source_occurrences.iter().flatten() {
            let cases = match occurrence.expr {
                RuntimeExpr::Match { cases, .. } => cases
                    .iter()
                    .map(|case| &case.constructor)
                    .collect::<Vec<_>>(),
                RuntimeExpr::ComputationalMatch { cases, .. } => cases
                    .iter()
                    .map(|case| &case.constructor)
                    .collect::<Vec<_>>(),
                _ => continue,
            };
            // Child 0 is the scrutinee and child `1 + i` is case `i`'s body,
            // the ordering `plan_expr` builds for both match forms
            // (`construction.rs`, `children.push(scrutinee.occurrence)` then the
            // case bodies). Read through `child_origins` rather than restated.
            let Ok(children) = self.semantic.child_origins(occurrence.static_origin) else {
                continue;
            };
            for (index, constructor) in cases.into_iter().enumerate() {
                // CONJUNCT (2), and it is the one whose absence refuted the
                // first cut. A constructor the RUNTIME can produce is live no
                // matter what the program's syntax does: an effect RESPONSE
                // like `Result::Ok` is synthesized by the host, reaches the
                // scrutinee, and selects this arm without any `Construct` or
                // literal anywhere in the program. Checked FIRST because it is
                // the cheaper test and because it is the one that must never be
                // skipped.
                if runtime_producible.contains(constructor) {
                    continue;
                }
                // CONJUNCT (1): not built by program syntax either.
                if self.constructor_is_ever_constructed(constructor) {
                    continue;
                }
                let Some(body) = children.get(1 + index).copied() else {
                    continue;
                };
                if occurrence_subtree_contains(self, body, needle)? {
                    return Ok(Some(body));
                }
            }
        }
        Ok(None)
    }

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

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::super::tests::{
        b2ac_topology_fixtures, b2o_transparent_declaration, equal_shaped_atom_fixture,
        equal_shaped_child_fixture, fixture_witness, nested_resource_bracket, nodes_of_shape, substrate_case, substrate_constructor, trap,
        unit,
    };
    use crate::RuntimeValue;

    /// Three occurrences that are **equal as terms**. A content or hash lookup
    /// cannot tell them apart; their origins can.
    fn content_equal_occurrences() -> RuntimeExpr {
        RuntimeExpr::If {
            scrutinee: Box::new(unit()),
            then_expr: Box::new(unit()),
            else_expr: Box::new(unit()),
        }
    }

    /// `RT-FNSPLIT-B2A-S` D6 — the occurrence table's negative controls, each red
    /// at its **own named artifact**.
    ///
    /// ⛔ The four expected errors are deliberately distinct. A single "the table
    /// is invalid" verdict would be discharged by any one of these mutations, so
    /// it could not tell a swapped entry from a missing one from a surplus one —
    /// and the whole point of storing each entry's origin beside its term is that
    /// those failures are distinguishable.
    #[test]
    fn occurrence_table_negative_controls_fail_at_named_artifacts() {
        // Promise class: durable mutation proof.
        let expr = equal_shaped_atom_fixture();
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();
        let vars = nodes_of_shape(&plan, RuntimeExprShape::Var);
        assert_eq!(
            vars.len(),
            2,
            "fixture must hold two equal-shaped occurrences"
        );

        // Control 1 — SWAP two equal-shaped entries. Each now sits at an index
        // that is not its stored origin, so the wrong-body substitution is
        // REFUSED rather than performed. This is the control that matters: the
        // pair agrees on shape and counts, so nothing but the stored origin
        // distinguishes them.
        let mut swapped = plan.clone();
        swapped
            .source_occurrences
            .swap(vars[0].0 as usize, vars[1].0 as usize);
        assert_eq!(
            swapped.validate_source_occurrence_table().unwrap_err(),
            planner_error("occurrence table entry is filed under an origin that is not its index")
        );
        // ⭐ And the lookup refuses on its own, not only via the whole-plan
        // validator — a consumer cannot reach a swapped body even in a plan that
        // was never re-validated.
        assert_eq!(
            swapped.source_occurrence(origin_of(vars[0])).unwrap_err(),
            planner_error("planned occurrence's stored origin disagrees with its table position")
        );

        // Control 2 — MISSING: a control node is a planned node with no source
        // term, so its slot is legitimately empty and a lookup on it is loud
        // rather than a substituted neighbour.
        assert_eq!(
            plan.source_occurrence(origin_of(plan.terminal_id()))
                .unwrap_err(),
            planner_error("static origin names no planned source occurrence")
        );

        // Control 3 — OUT OF RANGE: past the end of the planned population.
        assert_eq!(
            plan.source_occurrence(StaticOriginId(plan.nodes.len() as u32 + 7))
                .unwrap_err(),
            planner_error("static origin is outside the planned occurrence table")
        );

        // Control 4 — SURPLUS/DUPLICATE: an entry no semantic seed accounts for.
        // Well-formed in isolation (its stored origin *is* its index), so only the
        // cross-check against the independently produced seed population sees it.
        let mut surplus = plan.clone();
        let terminal = plan.terminal_id();
        surplus.source_occurrences[terminal.0 as usize] = Some(PlannedOccurrence {
            static_origin: origin_of(terminal),
            expr: &expr,
        });
        assert_eq!(
            surplus.validate_source_occurrence_table().unwrap_err(),
            planner_error("occurrence table holds an entry no semantic seed accounts for")
        );
    }

    /// `RT-FNSPLIT-B2A-S` D6/AC-3 — **identity is the ordinal, not the content.**
    ///
    /// ⭐ This is the chain's predicate as an executable test. The fixture's three
    /// occurrences are equal as terms, so a content or hash lookup would have to
    /// pick one of them arbitrarily — while the tag resolves each to its own
    /// occurrence. If this test ever passes with the origins compared equal, a
    /// dynamic property has started naming static code again.
    #[test]
    fn content_equal_occurrences_resolve_to_distinct_occurrences() {
        // Promise class: durable invariant.
        let expr = content_equal_occurrences();
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();
        let units = nodes_of_shape(&plan, RuntimeExprShape::Construct);
        // ⚠ Load-bearing: without it the loop below is vacuous and this test
        // passes while checking nothing at all.
        assert_eq!(units.len(), 3, "fixture must hold three equal terms");

        let resolved = units
            .iter()
            .map(|node| {
                (
                    origin_of(*node),
                    plan.source_occurrence(origin_of(*node)).unwrap(),
                )
            })
            .collect::<Vec<_>>();

        for (index, (origin, term)) in resolved.iter().enumerate() {
            for (other_origin, other_term) in resolved.iter().skip(index + 1) {
                assert_ne!(origin, other_origin, "each occurrence has its own origin");
                // ⛔ `RuntimeExpr: PartialEq` is gone (`D2`), and a `Debug`-text
                // proxy is barred. The claim here does not need term-to-term
                // comparison at all: the fixture builds all three occurrences
                // from `unit()`, so *"a CONTENT lookup could not have told them
                // apart"* is established by asserting each resolves to that one
                // known content — a direct property, checked against a value
                // this test states rather than against its sibling.
                for resolved in [term, other_term] {
                    let RuntimeExpr::Construct { constructor, args } = resolved else {
                        panic!("occurrence resolved to {resolved:?}, not a Construct");
                    };
                    assert_eq!(constructor, "ctor:prelude::Unit::MkUnit");
                    assert!(args.is_empty(), "unit takes no arguments");
                }
                assert!(
                    !std::ptr::eq(*term, *other_term),
                    "distinct occurrences resolve to distinct subterms"
                );
            }
        }
    }

    /// `RT-FNSPLIT-B2A-S` D6/AC-3 — perturbing the borrowed **address** while the
    /// ordinal mapping is unchanged does not move any identity.
    ///
    /// Two structurally equal source trees at different addresses plan to the same
    /// origins, and each plan resolves its own origins into its own tree. So the
    /// table's key is the planner's ordinal and the borrow is payload — which is
    /// exactly what makes a lifetime admissible here rather than dangerous.
    #[test]
    fn a_source_tree_at_a_different_address_yields_identical_origins() {
        // Promise class: durable invariant.
        let first = equal_shaped_child_fixture();
        let second = equal_shaped_child_fixture();
        assert!(
            !std::ptr::eq(&first, &second),
            "the two fixtures must live at different addresses"
        );

        let first_plan = plan_static_transition_graph(&first, &BTreeMap::new()).unwrap();
        let second_plan = plan_static_transition_graph(&second, &BTreeMap::new()).unwrap();

        let origins = |plan: &StaticTransitionPlan<'_>| {
            plan.semantic_sources
                .iter()
                .map(|seed| (seed.origin, seed.source))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            origins(&first_plan),
            origins(&second_plan),
            "identity must not depend on where the source tree happens to live"
        );

        // And the payload follows the borrow it was planned from, rather than
        // leaking across plans.
        for (origin, _) in origins(&first_plan) {
            if let Ok(term) = first_plan.source_occurrence(origin) {
                let other = second_plan.source_occurrence(origin).unwrap();
                // ⛔ This asserted only `discriminant` equality, which CANNOT
                // establish the property: it passes if `Var(0)` were resolved
                // as `Var(3)`, or if the two equal-shaped `Let` children were
                // exchanged — the exact occurrence-identity defects this
                // fixture exists to catch. Recursive comparison is genuinely
                // required here, through the closure-refusing witness.
                let (lhs, rhs) = (fixture_witness(term), fixture_witness(other));
                assert!(
                    lhs.is_some() && rhs.is_some(),
                    "both occurrences must lie in the fixture grammar; a \
                     refusal is a failure, not a skip"
                );
                assert_eq!(
                    lhs, rhs,
                    "equal trees resolve to structurally identical terms, \
                     including every Var index and child position"
                );
                assert!(
                    !std::ptr::eq(term, other),
                    "but each plan resolves into its own tree"
                );
            }
        }
    }

    /// `RT-FNSPLIT-B2A-S` AC-2 — the table is **total** over the planned expression
    /// population, positively and not by the absence of a failure.
    #[test]
    fn the_occurrence_table_is_total_over_every_planned_expression() {
        // Promise class: durable invariant.
        let expr = nested_resource_bracket(3);
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).unwrap();

        let mut expressions = 0usize;
        for seed in &plan.semantic_sources {
            match seed.source {
                SemanticSourceKind::Expression(_) => {
                    expressions += 1;
                    plan.source_occurrence(seed.origin).expect(
                        "every planned expression occurrence resolves through its own origin",
                    );
                }
                SemanticSourceKind::Control(_) => {
                    plan.source_occurrence(seed.origin)
                        .expect_err("a control node has no source term to resolve");
                }
            }
        }
        assert!(
            expressions > 1,
            "the fixture must plan more than one expression for totality to mean anything"
        );
        assert_eq!(
            expressions,
            plan.source_occurrences
                .iter()
                .filter(|slot| slot.is_some())
                .count(),
            "the table holds exactly one entry per planned expression occurrence"
        );
    }

    /// **AC-14 — nested computational matches stay INJECTIVE even when several
    /// occurrences share a scheduling entry.**
    ///
    /// ⭐ This is the row a shallow test omits. In `computational-nested` the
    /// outer and inner matches are scheduled through the same chain, so a key
    /// taken from the *entry* would look unique while naming the wrong
    /// occurrence. The occurrences must differ, and each must resolve its own
    /// children.
    #[test]
    fn nested_computational_occurrences_stay_injective_under_a_shared_entry() {
        let (_, nested) = b2ac_topology_fixtures()
            .into_iter()
            .find(|(name, _)| *name == "computational-nested")
            .expect("the nested computational fixture");
        let plan = plan_static_transition_graph(&nested, &BTreeMap::new()).expect("plannable");
        let outer = plan.root_static_origin().expect("root occurrence");
        // The outer match's case body IS the inner match: child `1 + 0`.
        let inner = plan
            .child_static_origin(outer, 1)
            .expect("the outer match's case body resolves");
        assert_ne!(
            outer, inner,
            "AC-14: two computational occurrences must not share an origin"
        );
        // Each resolves its OWN children -- scrutinee at 0, case body at 1.
        for occurrence in [outer, inner] {
            plan.child_static_origin(occurrence, 0)
                .expect("scrutinee position resolves");
            plan.child_static_origin(occurrence, 1)
                .expect("case-body position resolves");
        }
        // And the shared scheduling entry is genuinely shared: the plan's single
        // root entry is a scrutinee chain node, not either occurrence.
        let entry = *plan.entries.first().expect("a root entry");
        assert_ne!(origin_of(entry), outer, "the entry is not the occurrence");
    }

    /// **Fail-closed law: a body occurrence must be a SOURCE occurrence, and an
    /// in-range control node is not one.**
    ///
    /// > **MEASURED:** the plan validator rejects a pairing whose body names an
    /// > in-range slot holding `None`.
    /// > **CLAIMED:** the law refuses a body occurrence that is not a planned
    /// > source occurrence.
    /// > **THE GAP:** `source_occurrences` is `Vec<Option<..>>`. An
    /// > out-of-range ordinal and an in-range `None` are DIFFERENT refusals, and
    /// > `.get(..).is_none()` only performs the first. The mutation here is
    /// > deliberately the in-range one, because the out-of-range case passes
    /// > under both the broken and the fixed predicate and so discriminates
    /// > nothing.
    ///
    /// The owner check does not subsume this: descriptors exist for control
    /// nodes too, so a control node owned by the same unit satisfies it.
    #[test]
    fn a_body_occurrence_naming_an_in_range_control_node_is_refused() {
        let (_, computational) = b2ac_topology_fixtures()
            .into_iter()
            .find(|(name, _)| *name == "computational")
            .expect("the computational fixture");
        let plan = plan_static_transition_graph(&computational, &BTreeMap::new())
            .expect("plannable");

        // A control node: in range, and its source slot is empty.
        let (control_index, _) = plan
            .source_occurrences
            .iter()
            .enumerate()
            .find(|(_, slot)| slot.is_none())
            .expect("precondition: the fixture must contain a control node");
        assert!(
            control_index < plan.source_occurrences.len(),
            "precondition: the mutation must be IN RANGE, or it tests the \
             out-of-range arm instead"
        );

        let mut mutated = plan.clone();
        mutated.planned_entry_bodies[0].body_occurrence =
            StaticOriginId(control_index as u32);
        assert_eq!(
            mutated.validate().unwrap_err(),
            planner_error("scheduling entry body occurrence is not a planned source occurrence"),
            "an in-range control node carries no source term and must not be \
             accepted as a unit body"
        );
    }

    /// **Fail-closed law: the declaration view is EQUALITY-checked per symbol,
    /// not merely a member of the issued set.**
    ///
    /// > **MEASURED:** swapping two declarations' recorded occurrences is
    /// > refused.
    /// > **CLAIMED:** the surviving view agrees with the pairing authority.
    /// > **THE GAP:** a swap preserves the SET exactly — same elements, every
    /// > one still issued to some entry — so a membership test passes on it.
    /// > Only a per-symbol comparison can see it, which is what makes this an
    /// > equality-checked projection rather than a co-existing table.
    #[test]
    fn swapping_two_declaration_occurrences_is_refused() {
        let (_, computational) = b2ac_topology_fixtures()
            .into_iter()
            .find(|(name, _)| *name == "computational")
            .expect("the computational fixture");
        let first = b2o_transparent_declaration(computational.clone());
        let second = b2o_transparent_declaration(RuntimeExpr::Value(RuntimeValue::Bool(false)));
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::one", &first);
        declarations.insert("decl:fixture::two", &second);
        let root = RuntimeExpr::Value(RuntimeValue::Bool(true));
        let plan = plan_static_transition_graph(&root, &declarations).expect("plannable");

        let one = plan
            .declaration_occurrence_origin("decl:fixture::one")
            .expect("first declaration planned");
        let two = plan
            .declaration_occurrence_origin("decl:fixture::two")
            .expect("second declaration planned");
        assert_ne!(
            one, two,
            "precondition: the two declarations must have DISTINCT occurrences, \
             or a swap is the identity and the control is vacuous"
        );

        let mut mutated = plan.clone();
        mutated
            .declaration_occurrences
            .insert("decl:fixture::one".to_owned(), two);
        mutated
            .declaration_occurrences
            .insert("decl:fixture::two".to_owned(), one);

        // The set is provably unchanged, which is why membership cannot see it.
        assert_eq!(
            plan.declaration_occurrences
                .values()
                .copied()
                .collect::<BTreeSet<_>>(),
            mutated
                .declaration_occurrences
                .values()
                .copied()
                .collect::<BTreeSet<_>>(),
            "the swap must preserve the value SET, or this control would be \
             discharged by a membership test and would not pin keyed equality"
        );

        assert_eq!(
            mutated.validate().unwrap_err(),
            planner_error("declaration occurrence projection disagrees with the issued pairing"),
            "each symbol's recorded occurrence must equal the body issued to \
             THAT symbol's own scheduling entry"
        );
    }

    /// **`RT-FNSPLIT-B2A-S` AC-5 — and entry-keying cannot be introduced QUIETLY,
    /// because filing two occurrences under one origin is refused.**
    ///
    /// ⭐ This is the mechanism that makes the property enforceable rather than
    /// merely stated. A `ComputationalMatch` shares its scheduling entry with its
    /// scrutinee chain, so a table keyed by `.entry` files two terms under one
    /// index — and `record_source_occurrence` rejects that outright.
    ///
    /// **Measured, not assumed:** replacing `expression_seed(resume, …)` with
    /// `expression_seed(scrutinee.entry, …)` — a compile-preserving mutation, and
    /// exactly the "key selection by `.entry`" change the Architect asked for —
    /// reddens **48** tests, **36** of them naming this invariant.
    #[test]
    fn filing_two_occurrences_under_one_origin_is_refused() {
        // Promise class: durable mutation proof.
        let (_, computational) = b2ac_topology_fixtures()
            .into_iter()
            .find(|(name, _)| *name == "computational")
            .expect("the computational fixture");

        let mut planner = Planner::new().expect("planner");
        let empty = PersistentNodeId(0);
        let context = PlanContext {
            environment: empty,
            continuation: empty,
            path: empty,
            cleanup: empty,
            affine: empty,
            source_return: empty,
        };
        planner
            .plan_expr(
                &computational,
                context,
                planner.terminal,
                EdgeKind::Continue,
                0,
            )
            .expect("plannable");

        // Any node that already owns an occurrence: re-filing it is the collision
        // an entry-keyed table would produce.
        let taken = planner
            .plan
            .semantic_sources
            .iter()
            .find_map(|seed| {
                matches!(seed.source, SemanticSourceKind::Expression(_))
                    .then_some(seed.planned_node)
            })
            .expect("the fixture plans at least one expression occurrence");
        assert_eq!(
            planner
                .record_source_occurrence(taken, &computational)
                .unwrap_err(),
            planner_error("static origin was given more than one source occurrence"),
            "AC-5: a second occurrence under one origin must be a loud planner \
             invariant, since that is what silently merges two bodies"
        );
    }

    /// **AC-15 — a root or transparent-declaration `ComputationalMatch` body
    /// receives the RESUME occurrence, not the scrutinee origin.**
    #[test]
    fn root_and_declaration_computational_bodies_take_the_resume_occurrence() {
        let (_, computational) = b2ac_topology_fixtures()
            .into_iter()
            .find(|(name, _)| *name == "computational")
            .expect("the computational fixture");

        // Root: the stored occurrence is the resume seed, and the scheduling
        // entry is the scrutinee -- so they must differ, and the occurrence must
        // resolve its own positional children.
        let plan =
            plan_static_transition_graph(&computational, &BTreeMap::new()).expect("plannable");
        let root = plan.root_static_origin().expect("root occurrence");
        let entry = *plan.entries.first().expect("a root entry");
        assert_ne!(
            root,
            origin_of(entry),
            "AC-15: a root computational match must not take its scrutinee's origin"
        );
        plan.child_static_origin(root, 0)
            .expect("the root occurrence resolves its scrutinee position");

        // Transparent declaration: same discriminator, by symbol.
        let declaration = RuntimeDeclaration {
            symbol: "decl:fixture::b2ac".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: computational.clone(),
            },
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
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::b2ac", &declaration);
        let plan =
            plan_static_transition_graph(&RuntimeExpr::Var(0), &declarations).expect("plannable");
        let occurrence = plan
            .declaration_occurrence_origin("decl:fixture::b2ac")
            .expect("the transparent declaration has an occurrence origin");
        let declaration_entry = plan.entries[1];
        assert_ne!(
            occurrence,
            origin_of(declaration_entry),
            "AC-15: a declaration whose body is a computational match must not \
             take its scrutinee's origin"
        );
        plan.child_static_origin(occurrence, 0)
            .expect("the declaration occurrence resolves its scrutinee position");
    }

    /// **AC-12 — every semantic child position consumes `.occurrence`.**
    ///
    /// Pinned at the type rather than by auditing call sites: both seed entry
    /// points take `&[StaticOriginId]`, and `StaticOriginId` can only be formed
    /// by `origin_of` inside this module, so a `StaticNodeId` cannot reach a
    /// child position at all.
    #[test]
    fn the_semantic_seed_api_accepts_only_occurrence_origins() {
        // `RT-PLANNER-ROOT-CLOSURE-SPLIT` `D1` — `expression_node` and
        // `expression_seed` moved into `construction.rs` with the rest of
        // `Planner`'s own impl; this oracle follows them.
        let source = include_str!("../static_transition/construction.rs");
        // ⚠ Count DECLARATION lines, not substring hits: this test's own
        // assertion text mentions both spellings, and a substring oracle would
        // fire on the prose that denies them.
        let declarations = source
            .lines()
            .filter(|line| line.trim() == "children: &[StaticOriginId],")
            .count();
        assert_eq!(
            declarations, 2,
            "AC-12: `expression_node` and `expression_seed` must both take \
             occurrence origins; a `&[StaticNodeId]` parameter here is the exact \
             conflation this parameter type exists to prevent"
        );
        assert!(
            !source
                .lines()
                .any(|line| line.trim() == "children: &[StaticNodeId],"),
            "AC-12: no semantic child list may be typed as scheduling nodes"
        );
    }

    /// MEASURED: equal case spellings at two distinct Match occurrences retain
    /// opposite partitions when their exact scrutinees produce Left and Right.
    ///
    /// CLAIMED: constructor family or case-list equality cannot substitute for
    /// match and scrutinee occurrence identity.
    ///
    /// GAP: owner separation is carried in each record and covered by D3's
    /// owner cross-check; this fixture keeps both matches in one owner to isolate
    /// the occurrence axis.
    #[test]
    fn substrate_equal_case_lists_keep_distinct_occurrence_partitions() {
        let one_match = |producer: &str| RuntimeExpr::Match {
            scrutinee: Box::new(substrate_constructor(producer)),
            cases: ["Left", "Right"].into_iter().map(substrate_case).collect(),
            default: trap("occurrence discriminator default"),
        };
        let expr = RuntimeExpr::Record {
            fields: vec![
                ("left".to_string(), one_match("Left")),
                ("right".to_string(), one_match("Right")),
            ],
        };
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).expect("plans");
        let mut by_match = BTreeMap::<StaticOriginId, Vec<CaseEmissionStatus>>::new();
        for record in &plan.case_emissions {
            by_match
                .entry(record.match_origin)
                .or_default()
                .push(record.status);
        }
        assert_eq!(by_match.len(), 2);
        let partitions = by_match.into_values().collect::<Vec<_>>();
        assert!(partitions.contains(&vec![
            CaseEmissionStatus::Reachable,
            CaseEmissionStatus::Eliminated,
        ]));
        assert!(partitions.contains(&vec![
            CaseEmissionStatus::Eliminated,
            CaseEmissionStatus::Reachable,
        ]));
    }

    /// D2 pin.
    ///
    /// MEASURED: every source occurrence has exactly one record carrying its
    /// preallocated origin, semantic function owner, exact positional child
    /// list and conservative referent lifetime. A lexical closure child makes
    /// its aggregate parent activation-owned; changing that answer to
    /// Persistent is rejected.
    ///
    /// CLAIMED: later planner slices can consume one closed per-occurrence
    /// authority instead of independently re-deriving owner or promoting a
    /// referent lifetime.
    ///
    /// GAP: this slice exposes no capability to lowering; activation remains a
    /// later slice's independently reviewed change.
    #[test]
    fn substrate_occurrence_owner_and_lifetime_are_exact() {
        let expr = RuntimeExpr::Record {
            fields: vec![
                ("durable".to_string(), substrate_constructor("Left")),
                (
                    "activation".to_string(),
                    RuntimeExpr::LexicalClosure {
                        captures: Vec::new(),
                        params: vec!["x".to_string()],
                        body: Box::new(unit()),
                    },
                ),
            ],
        };
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).expect("plans");
        let root = plan.root_static_origin().expect("root occurrence");
        let root_authority = plan
            .occurrence_authorities
            .iter()
            .find(|record| record.origin == root)
            .expect("root authority");
        assert_eq!(
            root_authority.lifetime,
            PlannedReferentLifetime::ActivationOwned
        );
        assert_eq!(
            root_authority
                .children
                .iter()
                .map(|child| child.origin)
                .collect::<Vec<_>>(),
            plan.semantic.child_origins(root).unwrap()
        );
        assert_eq!(
            Some(root_authority.owner),
            plan.semantic.function_owner(root).unwrap()
        );
        assert_eq!(
            plan.occurrence_authorities.len(),
            plan.source_occurrences.iter().flatten().count()
        );

        let mut wrong_lifetime = plan.occurrence_authorities.clone();
        wrong_lifetime
            .iter_mut()
            .find(|record| record.origin == root)
            .expect("mutable root authority")
            .lifetime = PlannedReferentLifetime::Persistent;
        assert_eq!(
            validate_occurrence_authority_plan(&plan, &wrong_lifetime).unwrap_err(),
            planner_error(
                "dormant occurrence authority is not exact for origin, owner and lifetime"
            )
        );
    }

    /// MEASURED: two occurrences of the same constructor identity and arity
    /// receive different lifetime authority solely because one child is
    /// persistent and the other is activation-owned.
    ///
    /// CLAIMED: aggregate lifetime is an occurrence-keyed transitive meet, not
    /// a property of constructor spelling or shape.
    ///
    /// GAP: the invocation aggregate carrier row is deliberately absent from
    /// Slice 0; this control proves only the dormant authority it will consume.
    /// `D7` — one role at two producer seats is TWO non-aliasing occurrences.
    ///
    /// MEASURED: two records for one role at different seats are accepted and
    /// carry different identities; two records naming the same `(seat, role)`
    /// are rejected by name.
    ///
    /// CLAIMED: the occurrence domain is non-aliasing, so one seat's record
    /// cannot authorize another seat's allocation.
    ///
    /// THE GAP: this drives hand-built records, so it proves the LAW and not
    /// that the builder populates it correctly. The builder's own agreement is
    /// the rebuild comparison in `validate_aggregate_ownership_plan`; what this
    /// adds is that a violation would be caught rather than silently indexed.
    #[test]
    fn one_role_at_two_seats_is_two_non_aliasing_occurrences() {
        let at = |id: u32,
                  owner: ContinuationEmissionOwner,
                  seat: u32,
                  path: SynthesizedAggregatePath| {
            PlannedAggregateOwnership {
                id: AggregateOccurrenceId(id),
                producer: AggregateOccurrenceProducer::SynthesizedUse {
                    owner,
                    seat: StaticOriginId(seat),
                    path,
                    role: SynthesizedAggregateRole::Constructor(
                        SynthesizedConstructorRole::Fixed(
                            SynthesizedFixedConstructorRole::Unit,
                        ),
                    ),
                },
                owner: None,
                shape: PlannedAggregateShape::Constructor,
                declared_children: Some(&[]),
                children: Vec::new(),
                meet: PlannedReferentLifetime::Persistent,
                allocation: PlannedAggregateAllocation::PersistentGround,
            }
        };
        let ok_root = SynthesizedAggregatePath::root(SynthesizedAggregateRoot::HostResultOk);
        let record = |id: u32, owner: ContinuationEmissionOwner, seat: u32| {
            at(id, owner, seat, ok_root.clone())
        };
        let unit_a = ContinuationEmissionOwner::Predeclared(PredeclaredFunctionId(0));
        let unit_b = ContinuationEmissionOwner::Predeclared(PredeclaredFunctionId(1));
        let generated = ContinuationEmissionOwner::Specialization(ContinuationSpecializationId(0));

        // Same ROLE, different SEAT: two lawful occurrences.
        let distinct = [record(0, unit_a, 11), record(1, unit_a, 12)];
        validate_aggregate_producers_are_unique(&distinct)
            .expect("one role at two seats is two occurrences, not a collision");
        assert_ne!(
            distinct[0].id, distinct[1].id,
            "two occurrences must not share an identity, or the per-use key \
             bought nothing over the per-role one it replaced"
        );

        // Same ROLE and same SEAT, different EMISSION OWNER: also two lawful
        // occurrences. This is the `D5a` axis -- one body lowered by its
        // predeclared unit and again inside a generated context is two
        // emissions, and a key without the owner would alias them.
        validate_aggregate_producers_are_unique(&[record(0, unit_a, 11), record(1, unit_b, 11)])
            .expect("one seat under two predeclared owners is two occurrences");
        validate_aggregate_producers_are_unique(&[record(0, unit_a, 11), record(1, generated, 11)])
            .expect("predeclared and specialization emissions of one seat are distinct");

        // ⭐ Same ROLE, same SEAT, same OWNER, different PATH: two lawful
        // occurrences. This is the axis the path key exists for — three
        // `ResourceKind` uses at one seat differ in nothing else, and without
        // the path one record would have to serve all three.
        validate_aggregate_producers_are_unique(&[
            at(0, unit_a, 11, ok_root.alternative(4).field(0)),
            at(1, unit_a, 11, ok_root.alternative(5).field(0)),
            at(2, unit_a, 11, ok_root.alternative(5).field(1)),
        ])
        .expect("one role at three positions in one seat's tree is three occurrences");

        // And the step KIND separates, not just the position: `field(0)` and
        // `alternative(0)` are different paths, so a key that collapsed the two
        // step kinds to a bare index would alias these two.
        validate_aggregate_producers_are_unique(&[
            at(0, unit_a, 11, ok_root.field(0)),
            at(1, unit_a, 11, ok_root.alternative(0)),
        ])
        .expect("a field step and an alternative step at position 0 are distinct paths");

        // The new Record role shares owner, seat, and field position with an
        // existing host-result constructor use. Its root and role are both in
        // the producer key, so the pair remains distinct.
        let host = at(0, unit_a, 11, ok_root.field(0));
        let mut environment = host.clone();
        environment.id = AggregateOccurrenceId(1);
        environment.producer = AggregateOccurrenceProducer::SynthesizedUse {
            owner: unit_a,
            seat: StaticOriginId(11),
            path: SynthesizedAggregatePath::root(
                SynthesizedAggregateRoot::UnitBoundaryEnvironment,
            )
            .field(0),
            role: SynthesizedAggregateRole::UnitBoundaryEnvironment,
        };
        environment.shape = PlannedAggregateShape::Record;
        assert_ne!(host.producer, environment.producer);
        validate_aggregate_producers_are_unique(&[host.clone(), environment.clone()])
            .expect("the environment root and role cannot alias a host-result use");

        // Nearest-neighbour mutation: collapse only the new key onto the host
        // key. The production validator must make that one-field perturbation
        // red.
        environment.producer = host.producer.clone();
        let alias =
            validate_aggregate_producers_are_unique(&[host, environment])
                .expect_err(
                    "collapsing the environment key onto a host key must refuse",
                );
        assert!(format!("{alias:?}").contains("same producer"));

        // Same SEAT, same role, same owner, SAME PATH: one use, so a second is
        // a collision.
        let collided = [record(0, unit_a, 11), record(1, unit_a, 11)];
        let error = validate_aggregate_producers_are_unique(&collided)
            .expect_err("two records for one use must reject");
        assert!(
            format!("{error:?}").contains("same producer"),
            "the refusal must be the aliasing stop itself: {error:?}"
        );
    }

    /// `RT-SYNTHESIZED-ENV-RECORD-OCCURRENCE` D0 / AC-2 / AC-3.
    ///
    /// MEASURED: a direct lexical-closure call whose closed result producer is
    /// a constructor with an empty lexical-closure field receives a synthesized
    /// Record occurrence keyed by emission owner, that exact producer, the
    /// unit-boundary root followed by field position zero, and the environment
    /// role.
    ///
    /// CLAIMED: the planner-visible producer and source field position form the
    /// same structural key lowering can recover from the carried constructor;
    /// no lowering-order ordinal participates. The distinct root and role keep
    /// the new use disjoint from every host-result constructor use.
    ///
    /// GAP: this is the empty-environment first population. Naming captured
    /// record fields requires field-identity and referent-owner authority that
    /// this node does not infer.
    ///
    /// Promise class: durable invariant. The mutation duplicates the actual
    /// environment producer key and must be refused by the production
    /// non-aliasing validator.
    #[test]
    fn unit_boundary_environment_record_has_a_structural_non_aliasing_occurrence() {
        let expression = RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["value".to_string()],
                body: Box::new(RuntimeExpr::Var(0)),
            }),
            args: vec![RuntimeExpr::Construct {
                constructor: "ctor:fixture::Environment::Wrap".to_string(),
                args: vec![RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: vec!["unit".to_string()],
                    body: Box::new(RuntimeExpr::Construct {
                        constructor: "ctor:fixture::Environment::Leaf".to_string(),
                        args: Vec::new(),
                    }),
                }],
            }],
        };
        let plan = plan_static_transition_graph(&expression, &BTreeMap::new())
            .expect("the direct unit-boundary fixture must plan");
        let producer = plan
            .source_occurrences
            .iter()
            .flatten()
            .find_map(|occurrence| match occurrence.expr {
                RuntimeExpr::Construct { args, .. }
                    if matches!(
                        args.as_slice(),
                        [RuntimeExpr::LexicalClosure { .. }]
                    ) =>
                {
                    Some(occurrence.static_origin)
                }
                _ => None,
            })
            .expect("the fixture has one closure-bearing constructor producer");
        let expected_path = SynthesizedAggregatePath::root(
            SynthesizedAggregateRoot::UnitBoundaryEnvironment,
        )
        .field(0);
        let environment_records = plan
            .aggregate_ownership
            .iter()
            .filter(|record| {
                matches!(
                    &record.producer,
                    AggregateOccurrenceProducer::SynthesizedUse {
                        seat,
                        path,
                        role: SynthesizedAggregateRole::UnitBoundaryEnvironment,
                        ..
                    } if *seat == producer && path == &expected_path
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            environment_records.len(),
            1,
            "the one producer field under one emission owner must mint one \
             occurrence"
        );
        let environment = environment_records[0];
        assert_eq!(environment.shape, PlannedAggregateShape::Record);
        assert_eq!(environment.declared_children, Some(&[][..]));
        assert!(environment.children.is_empty());
        assert_eq!(environment.meet, PlannedReferentLifetime::Persistent);
        assert_eq!(
            environment.allocation,
            PlannedAggregateAllocation::PersistentGround
        );

        let mut duplicate = environment.clone();
        duplicate.id = AggregateOccurrenceId(environment.id.0 + 1);
        let refusal = validate_aggregate_producers_are_unique(&[
            environment.clone(),
            duplicate,
        ])
        .expect_err("duplicating the real environment key must refuse");
        assert!(
            format!("{refusal:?}").contains("same producer"),
            "the production non-aliasing law must own the refusal: {refusal:?}"
        );
    }
}
