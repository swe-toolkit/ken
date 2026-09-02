//! The planner-owned construction lifecycle -- minting, relation and seat
//! construction.
//!
//! `RT-PLANNER-ROOT-CLOSURE-SPLIT` `D1` -- this module owns
//! `Planner<'src>`, the raw graph-construction machine (node/edge/store
//! allocation, the recursive `plan_expr` descent, declaration-call-target
//! resolution, and `finish()`, which sequences every domain's own builder
//! and then closes the plan through `closure.rs`'s `validate()`).
//! `StaticTransitionPlan` itself stays declared in the parent (it is the
//! LCA of this module and `closure.rs`); this module's own `impl
//! StaticTransitionPlan` fragment holds only the two registration methods
//! (`register_scheduling_entry`, `record_planned_entry_body`) that are
//! `Planner`'s own write-path, under the standing child-module pattern
//! (item 4's `units.rs` precedent, already extended to `Planner` itself
//! in item 9's `intern_trap`).
//!
//! The closure/validation/read-only-projection half of `StaticTransitionPlan`
//! -- `validate`, `census`, `semantic_census`, and the case-emission /
//! case-producer / static-worker-member / substrate-preallocation families
//! that back `validate` -- is a DIFFERENT lifecycle entirely and lives in
//! `closure.rs`; see the `D0` ledger's outcome-2 determination in
//! `docs/program/issues/RT-PLANNER-ROOT-CLOSURE-SPLIT.md`.

#[cfg(test)]
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};

use super::abi::{
    build_abi_plane, install_continuation_context_abi, install_continuation_specialization_abi,
    AbiPlane, AbiRootIngress,
};
#[cfg(feature = "px8-ds-test-support")]
use super::aggregates::{
    apply_checked_ih_continuation_inheritance_mutation,
    checked_ih_intervening_binder_population_control_is_active,
    record_checked_ih_continuation_inheritances,
    record_checked_ih_generated_entry_admissions,
    record_checked_ih_generated_entry_confluences,
    run_checked_ih_intervening_binder_population_control,
};
#[cfg(test)]
#[allow(unused_imports)]
use super::aggregates::{
    aggregate_child_referent_owners, fixed_node_selected_owner, flatten_allocation_reachable_uses,
    host_effect_recipe_tree, node_referent_owners, validate_aggregate_producers_are_unique,
    SynthesizedAggregateStep,
};
#[allow(unused_imports)]
use super::aggregates::{
    build_aggregate_ownership_plan, build_checked_ih_continuation_inheritances,
    build_checked_ih_environment_transports, build_checked_ih_generated_entry_accesses,
    build_checked_ih_generated_entry_confluences,
    lifetime_referent_affinity, validate_aggregate_ownership_plan,
    validate_checked_ih_continuation_inheritances, validate_checked_ih_environment_transports,
    validate_checked_ih_generated_entry_accesses,
    validate_checked_ih_generated_entry_confluences, AggregateOccurrenceId,
    AggregateOccurrenceProducer,
    PlannedAggregateAllocation, PlannedAggregateOwnership, PlannedAggregateShape,
    SynthesizedAggregateNode, SynthesizedAggregatePath, SynthesizedAggregateRole,
    SynthesizedAggregateRoot, SynthesizedDynamicSet,
};
#[cfg(test)]
use super::closure::apply_static_worker_member_mutation;
use super::closure::{
    build_case_emission_plan, validate_case_emission_plan, validate_substrate_preallocation_closure,
};
use super::continuations::{
    build_continuation_specialization_plan, finalize_continuation_availability_plan,
    validate_continuation_specialization_plan,
};
#[allow(unused_imports)]
use super::continuations::{
    build_static_continuation_fusion_plan, fusion_redirect_target,
    verify_current_lexical_availability, verify_predeclared_entry_frame_membership,
    AdmittedContinuationDiscovery, BodyEmissionDisposition, CheckedCaseBinderLayout,
    CheckedCaseBinderRole, CheckedIhBinding, CheckedTransportCoordinate, ComposedCallTarget,
    ComposedWorkerRouteEligibility, ComposedWorkerView, ContinuationAvailabilityDraft,
    ContinuationAvailabilityOver, ContinuationAvailabilityViews, ContinuationCallIdentity,
    ContinuationCallView, ContinuationConsumingOccurrence, ContinuationContextId,
    ContinuationContextView, ContinuationEmissionOwner, ContinuationEnvironmentClaim,
    ContinuationEnvironmentClaimOver, ContinuationEnvironmentDraft, ContinuationFrameIdentity,
    ContinuationFrameRequirement, ContinuationInputSource, ContinuationInputView,
    ContinuationOrdinaryEnvelopeRole, ContinuationResultEdge, ContinuationSourceCoordinate,
    ContinuationSourceSlotAuthority, ContinuationSpecializationId, ContinuationUnitView,
    ContinuationWorkerCaptureProvenance, ContinuationWorkerCaptureSource, FusionClaimRefusal,
    FusionComposedEdge, FusionCompositionLayer, FusionOwnedBody, FusionOwnedOuterRealization,
    FusionRegionClaim, FusionRegionClaimLedger, PlannedContinuationContext, ProducerLocalBinding,
    ProducerLocalLocator, RequiredConsumerProjection, StaticContinuationFusionCandidate,
    StaticContinuationFusionDescriptor, StaticContinuationFusionId, StaticContinuationFusionKey,
    StaticContinuationFusionPlan, StaticContinuationFusionView,
};
#[cfg(test)]
#[allow(unused_imports)]
use super::continuations::{
    validate_continuation_specialization_closure, ContinuationInternMutation,
    ContinuationProductionMutation, ContinuationProjectionOmission, COMPOSED_CALL_TARGET_DEFECT,
    CONTINUATION_INTERN_MUTATION, CONTINUATION_PRODUCTION_MUTATION, DUPLICATE_STATIC_BODY_TRIPLE,
    ENVELOPE_DEFECT, SUPPRESS_POST_SPECIALIZATION_DESCENT, WEAKEN_CONTINUATION_DECREASING_MEASURE,
};
#[allow(unused_imports)]
use super::effects::{
    build_host_effect_seat_plan, host_effect_seat_contract_of, validate_host_effect_seat_plan,
    EffectSeatAvail, EffectSeatNeed, EffectSeatOperation, EffectSeatPhase, EffectSeatSlot,
    PlannedEffectSeat, CRANELIFT_HOST_EFFECT_CONSUMERS_V1,
};
#[allow(unused_imports)]
use super::joins_traps::{
    build_join_result_plan, planned_partiality_trap, JoinPlanToken, JoinResultRepresentation,
    PlannedJoinResult,
};
use super::occurrences::{
    build_occurrence_authority_plan, origin_of, validate_occurrence_authority_plan, StaticOriginId,
};
use super::semantic_ir::{
    build_bool_constructor_inventory, build_semantic_plane,
    build_synthesized_constructor_inventory, SemanticMaterialArena, SemanticPlane,
    SemanticSourceSeed,
};
use super::{
    planner_capacity_error, planner_error, CraneliftBackendError, DeclarationCallTargetClass,
    DynamicActivationFrame, EdgeEvidence, EdgeKind, PersistentNodeId, PersistentStoreNode,
    PlanContext, PlannedEntryBody, PlannedHelperKey, StaticEdge, StaticEdgeId, StaticNode,
    StaticNodeId, StaticSourceId, StaticTransitionPlan, StoreKind, TransitionKind,
};
use crate::RuntimeExpr;

#[cfg(test)]
thread_local! {
    static ACTIVE_RECURSIVE_LOWERING_FRAMES: Cell<usize> = const { Cell::new(0) };
    static MAX_RECURSIVE_LOWERING_FRAMES: Cell<usize> = const { Cell::new(0) };
}

/// Test-only observation of the actual `plan_expr` call stack.
///
/// The guard is entered inside `plan_expr`, so `Drop` runs on every `?` return
/// as well as the ordinary path. This measures production recursion rather than
/// deriving a proxy from bracket depth or expression-node counts.
#[cfg(test)]
struct RecursiveLoweringFrameGuard;

#[cfg(test)]
impl RecursiveLoweringFrameGuard {
    fn enter() -> Self {
        ACTIVE_RECURSIVE_LOWERING_FRAMES.with(|active| {
            let depth = active
                .get()
                .checked_add(1)
                .expect("recursive lowering frame count fits usize");
            active.set(depth);
            MAX_RECURSIVE_LOWERING_FRAMES.with(|maximum| {
                maximum.set(maximum.get().max(depth));
            });
        });
        Self
    }
}

#[cfg(test)]
impl Drop for RecursiveLoweringFrameGuard {
    fn drop(&mut self) {
        ACTIVE_RECURSIVE_LOWERING_FRAMES.with(|active| {
            active.set(
                active
                    .get()
                    .checked_sub(1)
                    .expect("recursive lowering frame guard is balanced"),
            );
        });
    }
}

#[cfg(test)]
pub(super) fn reset_recursive_lowering_frame_count() {
    ACTIVE_RECURSIVE_LOWERING_FRAMES.with(|active| active.set(0));
    MAX_RECURSIVE_LOWERING_FRAMES.with(|maximum| maximum.set(0));
}

#[cfg(test)]
pub(super) fn max_recursive_lowering_frame_count() -> usize {
    MAX_RECURSIVE_LOWERING_FRAMES.with(Cell::get)
}

/// ⭐ The dual result of planning one expression.
///
/// One `StaticNodeId` was previously made to mean two different things:
///
/// - **`entry`** — the first node the transfer graph *schedules* for the
///   expression;
/// - **`occurrence`** — the node on which `SemanticSourceSeed::expression`
///   registered that `RuntimeExpr`, and from which its positional child-origin
///   record is read.
///
/// They coincide for every ordinary form and **deliberately do not** for
/// `ComputationalMatch`, whose occurrence is registered on its
/// `SourceReturnResume` while the parent must still schedule its scrutinee.
/// Returning one value for both made a parent record the scrutinee's identity as
/// its child's origin — a category error, not an off-by-one.
///
/// ⛔ **The two fields have disjoint consumers, and that is the whole mechanism.**
/// Transfer topology consumes **only `.entry`**; source correspondence consumes
/// **only `.occurrence`**. This adds no node, no origin, no search and no
/// arithmetic: both values are outputs of the same recursive visit, and
/// `occurrence` is the origin already assigned to the already-existing semantic
/// seed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct PlannedExpr {
    pub(super) entry: StaticNodeId,
    pub(super) occurrence: StaticOriginId,
}

/// **`RT-DECL-CLOSURE-PORT` `D4` causal control — the two ways the selective
/// retarget can be wrong, both compile-preserving.**
///
/// ⭐ Two settings and not one, because a retarget can be wrong by not moving
/// **and** by moving to the wrong place, and no single mutation shows both.
///
/// `NeverRetarget` is the pre-`D4` world: the closure-seed reference keeps its
/// zero-input scheduling entry, which is the wrong-arity target `D4` removes.
/// `AnyStaticBody` is the ruled-out *reverse body search*: it takes the first
/// static-body target in the graph instead of the one leaving this
/// declaration's own entry. ⛔ The second is the one a positive assertion is
/// most likely to agree with by accident, because a program with one closure
/// gives both spellings the same answer — which is why the fixture carries a
/// second, anonymous closure.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum D4DeclarationTargetMutation {
    Exact,
    /// Never follow the static-body edge: the pre-`D4` behaviour.
    NeverRetarget,
    /// Follow the first static-body edge in the graph, whoever it leaves.
    AnyStaticBody,
}

#[cfg(test)]
thread_local! {
    pub(super) static D4_DECLARATION_TARGET_MUTATION: std::cell::Cell<D4DeclarationTargetMutation> =
        const { std::cell::Cell::new(D4DeclarationTargetMutation::Exact) };
}

pub(super) struct Planner<'src> {
    pub(super) plan: StaticTransitionPlan<'src>,
    store_interner: BTreeMap<PersistentStoreNode, PersistentNodeId>,
    next_source: u32,
    pub(super) terminal: StaticNodeId,
    trap_terminal: StaticNodeId,
}

impl<'src> Planner<'src> {
    pub(super) fn new() -> Result<Self, CraneliftBackendError> {
        let empty = PersistentNodeId(0);
        let frame = DynamicActivationFrame {
            syntax: empty,
            environment: empty,
            normal: empty,
            abrupt: empty,
            path: empty,
            cleanup: empty,
            affine: empty,
            source_return: empty,
        };
        let mut planner = Self {
            plan: StaticTransitionPlan {
                entries: Vec::new(),
                planned_entry_bodies: Vec::new(),
                nodes: Vec::new(),
                edges: Vec::new(),
                stores: Vec::new(),
                store_depths: Vec::new(),
                evidence: Vec::new(),
                planned_helpers: Vec::new(),
                semantic_sources: Vec::new(),
                semantic_material: SemanticMaterialArena::default(),
                abi: AbiPlane::default(),
                root_entry: None,
                root_ingress: AbiRootIngress::Value,
                semantic: SemanticPlane::default(),
                root_occurrence: None,
                declaration_occurrences: BTreeMap::new(),
                declaration_entries: BTreeMap::new(),
                declaration_call_targets: BTreeMap::new(),
                trap_catalog: Vec::new(),
                source_occurrences: Vec::new(),
                join_results: Vec::new(),
                case_emissions: Vec::new(),
                aggregate_ownership: Vec::new(),
                checked_ih_environment_transports: Vec::new(),
                checked_ih_continuation_inheritances: Vec::new(),
                checked_ih_generated_entry_confluences: BTreeMap::new(),
                checked_ih_generated_entry_accesses: BTreeMap::new(),
                host_effect_seats: Vec::new(),
                occurrence_authorities: Vec::new(),
                continuation_specializations: Vec::new(),
                continuation_specialization_calls: Vec::new(),
                required_consumer_projections: BTreeMap::new(),
                continuation_contexts: Vec::new(),
                static_response_continuations: Vec::new(),
                static_response_plan_installed: false,
                static_response_infeasible: None,
                static_response_deferred: Vec::new(),
                static_response_phase_a: None,
                // Empty by construction: the planner has no oriented plan, so a
                // fusion identity cannot exist yet. `D2f`'s post-planner
                // installer is the only writer.
                static_continuation_fusions: StaticContinuationFusionPlan::default(),
                // Empty by construction: body ownership is installed only from
                // validated claims, which cannot exist before the plan does.
                fusion_owned_bodies: BTreeMap::new(),
                fusion_composed_calls: BTreeMap::new(),
                fusion_outer_realizations: BTreeMap::new(),
                fusion_bodies_installed: false,
            },
            store_interner: BTreeMap::new(),
            next_source: 0,
            terminal: StaticNodeId(0),
            trap_terminal: StaticNodeId(0),
        };
        let terminal_owner = planner.source()?;
        planner.terminal = planner.control_node(TransitionKind::Terminal, terminal_owner, frame)?;
        let trap_owner = planner.source()?;
        planner.trap_terminal =
            planner.control_node(TransitionKind::TrapTerminal, trap_owner, frame)?;
        Ok(planner)
    }

    fn source(&mut self) -> Result<StaticSourceId, CraneliftBackendError> {
        let id = self.next_source;
        self.next_source = self
            .next_source
            .checked_add(1)
            .ok_or_else(|| planner_capacity_error("static source identity exhausted"))?;
        Ok(StaticSourceId(id))
    }

    fn push_node(
        &mut self,
        kind: TransitionKind,
        owner: StaticSourceId,
        frame: DynamicActivationFrame,
    ) -> Result<StaticNodeId, CraneliftBackendError> {
        let id = u32::try_from(self.plan.nodes.len())
            .map_err(|_| planner_capacity_error("static node identity exhausted"))?;
        let id = StaticNodeId(id);
        self.plan.nodes.push(StaticNode {
            id,
            transition: kind,
            owner,
            frame,
        });
        self.plan
            .planned_helpers
            .push(PlannedHelperKey::node(kind, id));
        Ok(id)
    }

    fn control_node(
        &mut self,
        kind: TransitionKind,
        owner: StaticSourceId,
        frame: DynamicActivationFrame,
    ) -> Result<StaticNodeId, CraneliftBackendError> {
        let node = self.push_node(kind, owner, frame)?;
        self.plan
            .semantic_sources
            .push(SemanticSourceSeed::control(node, kind));
        Ok(node)
    }

    /// Registers an expression occurrence whose syntax children are already
    /// planned. `children` is in source position order, and holds each child's
    /// **occurrence** origin — never its scheduling entry (D9).
    ///
    /// The returned `PlannedExpr` has `entry == occurrence.node`: an ordinary
    /// form is scheduled at the very node its occurrence is registered on.
    /// `ComputationalMatch` is the sole variant that does not go through here for
    /// that reason.
    fn expression_node(
        &mut self,
        kind: TransitionKind,
        owner: StaticSourceId,
        frame: DynamicActivationFrame,
        expr: &'src RuntimeExpr,
        children: &[StaticOriginId],
    ) -> Result<PlannedExpr, CraneliftBackendError> {
        let node = self.push_node(kind, owner, frame)?;
        self.expression_seed(node, expr, children)?;
        Ok(PlannedExpr {
            entry: node,
            occurrence: origin_of(node),
        })
    }

    /// Emits an already-pushed node's semantic material. Split out for the one
    /// occurrence whose node must exist before its children are planned (a
    /// computational match's source-return resume owns the outer edges).
    fn expression_seed(
        &mut self,
        node: StaticNodeId,
        expr: &'src RuntimeExpr,
        children: &[StaticOriginId],
    ) -> Result<(), CraneliftBackendError> {
        match expr {
            RuntimeExpr::Trap(trap)
            | RuntimeExpr::Match { default: trap, .. }
            | RuntimeExpr::ComputationalMatch { default: trap, .. } => {
                self.intern_trap(trap)?;
            }
            RuntimeExpr::PrimitiveCall { primitive, .. } => {
                if let Some(trap) = planned_partiality_trap(primitive) {
                    self.intern_trap(&trap)?;
                }
            }
            // `RT-DEAD-ARM-EFFECT-LOWERING` `D1`: every effect occurrence gets
            // its dead-arm trap interned here, unconditionally. Deadness is a
            // LOWERING-time verdict, so the planner cannot know which effects
            // will need one -- and an identity minted only for the effects some
            // earlier pass guessed at would fail exactly on the occurrence that
            // turned out to need it. An interned trap nothing emits is lawful;
            // the catalog authorizes, it does not oblige.
            RuntimeExpr::Effect {
                family, operation, ..
            } => {
                self.intern_trap(&super::joins_traps::dead_arm_effect_trap(family, *operation))?;
            }
            _ => {}
        }
        let seed =
            SemanticSourceSeed::expression(node, expr, children, &mut self.plan.semantic_material)?;
        self.plan.semantic_sources.push(seed);
        self.record_source_occurrence(node, expr)
    }

    /// Add the cross-owner edges represented by source `DeclarationRef`
    /// occurrences after all transparent declaration entries exist.
    ///
    /// These are deliberately not `StaticBody` edges: that edge kind is the
    /// closure-body owner boundary and also seeds a function unit. A transparent
    /// declaration entry is already a scheduling-entry seed.
    pub(super) fn connect_declaration_calls(
        &mut self,
        declaration_entries: &BTreeMap<String, StaticNodeId>,
    ) -> Result<(), CraneliftBackendError> {
        // ⭐ `D4`: resolve each declaration's target ONCE, before any edge is
        // added. The class is a property of the **declaration**, not of a
        // reference to it, so two references to one symbol cannot disagree —
        // and a symbol with no reference at all is still resolved, so a
        // malformed closure seed fails in planning rather than only when
        // somebody happens to call it.
        let mut targets: BTreeMap<&str, (StaticNodeId, DeclarationCallTargetClass)> =
            BTreeMap::new();
        for (symbol, entry) in declaration_entries {
            let resolved = self.declaration_call_target(symbol, *entry)?;
            targets.insert(symbol.as_str(), resolved);
        }
        let calls = self
            .plan
            .source_occurrences
            .iter()
            .flatten()
            .filter_map(|occurrence| match occurrence.expr {
                RuntimeExpr::DeclarationRef { symbol } => targets
                    .get(symbol.as_str())
                    .copied()
                    .map(|(target, class)| {
                        (StaticNodeId(occurrence.static_origin.0), target, class)
                    }),
                _ => None,
            })
            .collect::<Vec<_>>();
        for (caller, callee, class) in calls {
            self.edge(caller, callee, EdgeKind::DeclarationCall)?;
            // Keyed by the REFERENCE occurrence, which is the same key the
            // resolved call record is looked up by in lowering. One reference
            // cannot acquire two classes: `source_occurrences` is dense by
            // origin, so each reference is visited once.
            if self
                .plan
                .declaration_call_targets
                .insert(StaticOriginId(caller.0), class)
                .is_some()
            {
                return Err(planner_error(
                    "one declaration-reference occurrence resolved two call target classes",
                ));
            }
        }
        Ok(())
    }

    /// **`RT-DECL-CLOSURE-PORT` `D4` — the selective retarget, and the only
    /// place it is decided.**
    ///
    /// ⛔ **This is not a blanket retarget, and the difference is load-bearing.**
    /// A non-closure transparent declaration's scheduling entry *is* its unit —
    /// a legitimately zero-input thunk — so moving its `DeclarationCall` edge
    /// would break every declaration call the corpus already makes. Only a
    /// `Closure` / `LexicalClosure` seed owns a second, declaration-owned unit
    /// to move to.
    ///
    /// The discriminator is read from the declaration's **planned occurrence**,
    /// not from the entry's shape or from a reverse search for a body: those
    /// two `RuntimeExpr` arms are exactly the pair that emits an
    /// `EdgeKind::StaticBody` edge, which is what makes "is a closure seed" and
    /// "has one forward static-body edge" the same fact stated twice. ⇒ Both are
    /// asserted, so a plan where they disagree fails in planning.
    fn declaration_call_target(
        &self,
        symbol: &str,
        entry: StaticNodeId,
    ) -> Result<(StaticNodeId, DeclarationCallTargetClass), CraneliftBackendError> {
        let occurrence = self
            .plan
            .declaration_occurrences
            .get(symbol)
            .copied()
            .ok_or_else(|| {
                planner_error("a planned transparent declaration has no occurrence origin")
            })?;
        let seed = self
            .plan
            .source_occurrences
            .get(occurrence.0 as usize)
            .and_then(Option::as_ref)
            .map(|planned| {
                matches!(
                    planned.expr,
                    RuntimeExpr::Closure { .. } | RuntimeExpr::LexicalClosure { .. }
                )
            })
            .ok_or_else(|| {
                planner_error("a planned transparent declaration has no source occurrence")
            })?;

        let mut body = None;
        for edge in &self.plan.edges {
            if edge.kind != EdgeKind::StaticBody || edge.from != entry {
                continue;
            }
            if body.is_some() {
                return Err(planner_error(
                    "a transparent declaration entry has two forward static body edges",
                ));
            }
            body = Some(edge.to);
        }
        match (seed, body) {
            (true, Some(body)) => {
                // The mutations act HERE, on a decision the consistency law
                // above has already accepted -- a mutation applied to the
                // discriminator itself would only ever trip that law, and would
                // measure the law instead of the retarget.
                #[cfg(test)]
                match D4_DECLARATION_TARGET_MUTATION.with(std::cell::Cell::get) {
                    D4DeclarationTargetMutation::Exact => {}
                    D4DeclarationTargetMutation::NeverRetarget => {
                        return Ok((entry, DeclarationCallTargetClass::SchedulingEntry));
                    }
                    D4DeclarationTargetMutation::AnyStaticBody => {
                        let any = self
                            .plan
                            .edges
                            .iter()
                            .find(|edge| edge.kind == EdgeKind::StaticBody)
                            .map(|edge| edge.to)
                            .expect("this arm has at least one static body edge");
                        return Ok((any, DeclarationCallTargetClass::CallableDeclaration));
                    }
                }
                Ok((body, DeclarationCallTargetClass::CallableDeclaration))
            }
            (false, None) => Ok((entry, DeclarationCallTargetClass::SchedulingEntry)),
            (true, None) => Err(planner_error(
                "a closure-seed transparent declaration has no forward static body edge to its \
                 callable unit",
            )),
            (false, Some(_)) => Err(planner_error(
                "a non-closure transparent declaration entry has a forward static body edge",
            )),
        }
    }

    fn edge(
        &mut self,
        from: StaticNodeId,
        to: StaticNodeId,
        kind: EdgeKind,
    ) -> Result<(), CraneliftBackendError> {
        let edge = u32::try_from(self.plan.edges.len())
            .map(StaticEdgeId)
            .map_err(|_| planner_capacity_error("static edge identity exhausted"))?;
        let owner = self.plan.nodes[from.0 as usize].owner;
        self.plan.edges.push(StaticEdge {
            id: edge,
            from,
            to,
            kind,
        });
        self.plan.evidence.push(EdgeEvidence {
            edge: edge.0,
            owner,
            from,
            to,
            kind,
        });
        self.plan
            .planned_helpers
            .push(PlannedHelperKey::edge(kind, edge));
        Ok(())
    }

    fn store(
        &mut self,
        kind: StoreKind,
        local: u32,
        aux: u32,
        child: PersistentNodeId,
    ) -> Result<PersistentNodeId, CraneliftBackendError> {
        let node = PersistentStoreNode {
            kind,
            local,
            aux,
            child,
        };
        if let Some(id) = self.store_interner.get(&node) {
            return Ok(*id);
        }
        let id = u32::try_from(self.plan.stores.len() + 1)
            .map(PersistentNodeId)
            .map_err(|_| planner_capacity_error("persistent store identity exhausted"))?;
        let child_depth = if child.0 == 0 {
            0
        } else {
            *self
                .plan
                .store_depths
                .get(child.0 as usize - 1)
                .ok_or_else(|| planner_error("persistent store child is not closed"))?
        };
        self.plan.stores.push(node);
        self.plan.store_depths.push(
            child_depth
                .checked_add(1)
                .ok_or_else(|| planner_capacity_error("persistent chain depth exhausted"))?,
        );
        self.store_interner.insert(node, id);
        Ok(id)
    }

    fn frame(
        &mut self,
        tag: u32,
        ordinal: u32,
        ctx: PlanContext,
        successor: StaticNodeId,
    ) -> Result<DynamicActivationFrame, CraneliftBackendError> {
        let syntax = self.store(StoreKind::Syntax, tag, ordinal, PersistentNodeId(0))?;
        let path = self.store(StoreKind::Path, ordinal, 0, ctx.path)?;
        let normal = self.store(StoreKind::Continuation, successor.0, 0, ctx.continuation)?;
        Ok(DynamicActivationFrame {
            syntax,
            environment: ctx.environment,
            normal,
            abrupt: PersistentNodeId(0),
            path,
            cleanup: ctx.cleanup,
            affine: ctx.affine,
            source_return: ctx.source_return,
        })
    }

    /// Plans a positional operand sequence. Returns the chain **entry** — what the
    /// parent schedules — and each element's **occurrence** origin indexed by its
    /// source position, which is what the parent records as its positional child
    ///. The two are different values for a
    /// `ComputationalMatch` element, and mixing them is a category error.
    fn plan_sequence(
        &mut self,
        expressions: &[&'src RuntimeExpr],
        ctx: PlanContext,
        successor: StaticNodeId,
        exit_kind: EdgeKind,
    ) -> Result<(StaticNodeId, Vec<StaticOriginId>), CraneliftBackendError> {
        let mut next = successor;
        let mut next_kind = exit_kind;
        let mut occurrences = vec![None; expressions.len()];
        for (ordinal, expression) in expressions.iter().enumerate().rev() {
            let planned = self.plan_expr(expression, ctx, next, next_kind, ordinal as u32)?;
            next = planned.entry;
            // Fusion-claim parameter order depends on this original-ordinal
            // write-back: reverse planning changes the execution chain, never
            // the positional child slice that reaches the semantic arena.
            occurrences[ordinal] = Some(planned.occurrence);
            next_kind = EdgeKind::Continue;
        }
        let occurrences = occurrences
            .into_iter()
            .map(|occurrence| {
                occurrence.ok_or_else(|| {
                    planner_error("operand sequence position has no planned occurrence")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok((next, occurrences))
    }

    /// Plans one eliminator's case bodies. Returns the dispatch **entry** and each
    /// body's **occurrence** origin by source position (D9): the case test edges
    /// to the body's `entry`, while the parent records the body's `occurrence`.
    fn plan_cases(
        &mut self,
        bodies: &[(&'src RuntimeExpr, usize)],
        ctx: PlanContext,
        successor: StaticNodeId,
        exit_kind: EdgeKind,
        default: StaticNodeId,
    ) -> Result<(StaticNodeId, Vec<StaticOriginId>), CraneliftBackendError> {
        let mut reject = default;
        let mut occurrences = vec![None; bodies.len()];
        for (ordinal, (body, binders)) in bodies.iter().enumerate().rev() {
            let mut body_ctx = ctx;
            for binder in 0..*binders {
                body_ctx.environment = self.store(
                    StoreKind::Environment,
                    binder as u32,
                    0,
                    body_ctx.environment,
                )?;
            }
            let planned = self.plan_expr(body, body_ctx, successor, exit_kind, ordinal as u32)?;
            occurrences[ordinal] = Some(planned.occurrence);
            let owner = self.source()?;
            let frame = self.frame(0x80, ordinal as u32, ctx, reject)?;
            let test = self.control_node(TransitionKind::CaseTest, owner, frame)?;
            // Topology: the test selects the body's SCHEDULING entry.
            self.edge(test, planned.entry, EdgeKind::Select)?;
            self.edge(test, reject, EdgeKind::Reject)?;
            reject = test;
        }
        let occurrences = occurrences
            .into_iter()
            .map(|occurrence| {
                occurrence.ok_or_else(|| planner_error("case position has no planned occurrence"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok((reject, occurrences))
    }

    /// Plans one expression occurrence and returns **both** of its identities
    /// (D9): the `entry` the parent schedules and the `occurrence` the parent
    /// records at its source position. Every arm but `ComputationalMatch` returns
    /// them equal, by going through `expression_node`.
    /// **Issuance seat 2 — register a static body.**
    ///
    /// Atomically emits `source -> body.entry` as [`EdgeKind::StaticBody`] and
    /// records `body.entry -> body.occurrence` in the pairing relation. It
    /// replaces every bare `StaticBody` edge write, so a body edge without its
    /// issued pair is unconstructible rather than merely rejected — the same
    /// property `register_scheduling_entry` gives seat 1.
    ///
    /// **Generic in the planned form.** It takes the body's returned
    /// [`PlannedExpr`] and names no expression shape. At both call sites the
    /// planner already holds `body.entry` and `body.occurrence` together, so
    /// recording the pair here is issued identity; selecting child 0 afterwards,
    /// scanning the occurrence graph, or using `origin_of(edge.to)` all discard
    /// an identity that was in hand and reconstruct it.
    fn register_static_body(
        &mut self,
        source: StaticNodeId,
        body: PlannedExpr,
    ) -> Result<(), CraneliftBackendError> {
        self.edge(source, body.entry, EdgeKind::StaticBody)?;
        self.plan.record_planned_entry_body(body);
        Ok(())
    }

    pub(super) fn plan_expr(
        &mut self,
        expr: &'src RuntimeExpr,
        ctx: PlanContext,
        successor: StaticNodeId,
        exit_kind: EdgeKind,
        ordinal: u32,
    ) -> Result<PlannedExpr, CraneliftBackendError> {
        #[cfg(test)]
        let _recursive_lowering_frame = RecursiveLoweringFrameGuard::enter();
        let owner = self.source()?;
        let tag = runtime_expr_tag(expr);
        let frame = self.frame(tag, ordinal, ctx, successor)?;
        let ctx = PlanContext {
            continuation: frame.normal,
            path: frame.path,
            ..ctx
        };
        match expr {
            RuntimeExpr::Trap(_) => {
                let node =
                    self.expression_node(TransitionKind::Evaluate, owner, frame, expr, &[])?;
                self.edge(node.entry, self.trap_terminal, EdgeKind::Trap)?;
                Ok(node)
            }
            RuntimeExpr::Value(_)
            | RuntimeExpr::Var(_)
            | RuntimeExpr::DeclarationRef { .. }
            | RuntimeExpr::ImportedDeclarationRef { .. } => {
                let node =
                    self.expression_node(TransitionKind::Evaluate, owner, frame, expr, &[])?;
                self.edge(node.entry, successor, exit_kind)?;
                Ok(node)
            }
            RuntimeExpr::CheckedJoinSite { body, .. }
            | RuntimeExpr::CheckedSubcontinuationFrame { body, .. }
            | RuntimeExpr::CheckedRecursiveInvocation { body, .. }
            | RuntimeExpr::CheckedComputationalIHSlots { body, .. }
            | RuntimeExpr::CheckedComputationalIHInvocation { body, .. }
            | RuntimeExpr::Project { record: body, .. } => {
                let body = self.plan_expr(body, ctx, successor, exit_kind, 0)?;
                let node = self.expression_node(
                    TransitionKind::Sequence,
                    owner,
                    frame,
                    expr,
                    &[body.occurrence],
                )?;
                self.edge(node.entry, body.entry, EdgeKind::Continue)?;
                Ok(node)
            }
            RuntimeExpr::Let { value, body } => {
                let environment = self.store(StoreKind::Environment, 0, 0, ctx.environment)?;
                let body = self.plan_expr(
                    body,
                    PlanContext { environment, ..ctx },
                    successor,
                    exit_kind,
                    1,
                )?;
                let value = self.plan_expr(value, ctx, body.entry, EdgeKind::Continue, 0)?;
                let node = self.expression_node(
                    TransitionKind::Sequence,
                    owner,
                    frame,
                    expr,
                    &[value.occurrence, body.occurrence],
                )?;
                self.edge(node.entry, value.entry, EdgeKind::Continue)?;
                Ok(node)
            }
            RuntimeExpr::If {
                scrutinee,
                then_expr,
                else_expr,
            } => {
                let then_entry = self.plan_expr(then_expr, ctx, successor, exit_kind, 1)?;
                let else_entry = self.plan_expr(else_expr, ctx, successor, exit_kind, 2)?;
                let branch_owner = self.source()?;
                let branch = self.control_node(TransitionKind::Branch, branch_owner, frame)?;
                self.edge(branch, then_entry.entry, EdgeKind::Select)?;
                self.edge(branch, else_entry.entry, EdgeKind::Reject)?;
                let scrutinee = self.plan_expr(scrutinee, ctx, branch, EdgeKind::Continue, 0)?;
                let node = self.expression_node(
                    TransitionKind::Evaluate,
                    owner,
                    frame,
                    expr,
                    &[
                        scrutinee.occurrence,
                        then_entry.occurrence,
                        else_entry.occurrence,
                    ],
                )?;
                self.edge(node.entry, scrutinee.entry, EdgeKind::Continue)?;
                Ok(node)
            }
            RuntimeExpr::Match {
                scrutinee, cases, ..
            } => {
                let default_owner = self.source()?;
                let default = self.control_node(TransitionKind::Evaluate, default_owner, frame)?;
                self.edge(default, self.trap_terminal, EdgeKind::Trap)?;
                let bodies = cases
                    .iter()
                    .map(|case| (&case.body, case.binders))
                    .collect::<Vec<_>>();
                let (dispatch, case_bodies) =
                    self.plan_cases(&bodies, ctx, successor, exit_kind, default)?;
                let scrutinee = self.plan_expr(scrutinee, ctx, dispatch, EdgeKind::Continue, 0)?;
                let mut children = Vec::with_capacity(1 + case_bodies.len());
                children.push(scrutinee.occurrence);
                children.extend(case_bodies);
                let node =
                    self.expression_node(TransitionKind::Evaluate, owner, frame, expr, &children)?;
                self.edge(node.entry, scrutinee.entry, EdgeKind::Continue)?;
                Ok(node)
            }
            RuntimeExpr::ComputationalMatch {
                scrutinee, cases, ..
            } => {
                let cleanup = self.store(StoreKind::Cleanup, owner.0, 0, ctx.cleanup)?;
                let affine = self.store(StoreKind::Affine, owner.0, 0, ctx.affine)?;
                let control_ctx = PlanContext {
                    cleanup,
                    affine,
                    ..ctx
                };
                let completed = self.control_node(TransitionKind::CompletedTail, owner, frame)?;
                let tail = self.control_node(TransitionKind::ProducerTail, owner, frame)?;
                let wrapper = self.control_node(TransitionKind::ProducerWrapper, owner, frame)?;
                let resume = self.push_node(TransitionKind::SourceReturnResume, owner, frame)?;
                self.edge(resume, wrapper, EdgeKind::InvokeProducerWrapper)?;
                self.edge(wrapper, tail, EdgeKind::InvokeProducerTail)?;
                self.edge(tail, completed, EdgeKind::CompleteProducerTail)?;
                self.edge(completed, successor, exit_kind)?;
                let source_return = self.store(
                    StoreKind::SourceReturn,
                    wrapper.0,
                    tail.0,
                    ctx.source_return,
                )?;
                let control_ctx = PlanContext {
                    source_return,
                    ..control_ctx
                };
                for id in [completed, tail, wrapper, resume] {
                    self.plan.nodes[id.0 as usize].frame.source_return = source_return;
                    self.plan.nodes[id.0 as usize].frame.cleanup = cleanup;
                    self.plan.nodes[id.0 as usize].frame.affine = affine;
                }
                let default_owner = self.source()?;
                let default = self.control_node(TransitionKind::Evaluate, default_owner, frame)?;
                self.edge(default, self.trap_terminal, EdgeKind::Trap)?;
                let bodies = cases
                    .iter()
                    .map(|case| {
                        (
                            &case.body,
                            case.argument_binders + case.recursive_positions.len(),
                        )
                    })
                    .collect::<Vec<_>>();
                let (dispatch, case_bodies) = self.plan_cases(
                    &bodies,
                    control_ctx,
                    resume,
                    EdgeKind::SourceReturnOwnedResume,
                    default,
                )?;
                let scrutinee =
                    self.plan_expr(scrutinee, control_ctx, dispatch, EdgeKind::Continue, 0)?;
                let mut children = Vec::with_capacity(1 + case_bodies.len());
                children.push(scrutinee.occurrence);
                children.extend(case_bodies);
                self.expression_seed(resume, expr, &children)?;
                // ⭐ THE SOLE SPLIT. This occurrence's record
                // lives on `resume`, because the resume owns the outer edges and
                // must exist before the cases are planned — but the transfer
                // graph still schedules the SCRUTINEE, exactly as before. So the
                // two identities genuinely differ here, and returning one value
                // for both is what made a parent record the scrutinee's identity
                // as this match's. ⛔ Do not "fix" this by returning `resume` as
                // the entry: that would change the approved Boundary-A topology.
                Ok(PlannedExpr {
                    entry: scrutinee.entry,
                    occurrence: origin_of(resume),
                })
            }
            RuntimeExpr::Closure { body, .. } => {
                let body_return_owner = self.source()?;
                let body_return =
                    self.control_node(TransitionKind::ClosureBody, body_return_owner, frame)?;
                self.edge(body_return, self.terminal, EdgeKind::Continue)?;
                let body = self.plan_expr(body, ctx, body_return, EdgeKind::Continue, 0)?;
                let node = self.expression_node(
                    TransitionKind::Evaluate,
                    owner,
                    frame,
                    expr,
                    &[body.occurrence],
                )?;
                self.edge(node.entry, successor, exit_kind)?;
                self.register_static_body(node.entry, body)?;
                Ok(node)
            }
            RuntimeExpr::LexicalClosure { captures, body, .. } => {
                let body_return_owner = self.source()?;
                let body_return =
                    self.control_node(TransitionKind::ClosureBody, body_return_owner, frame)?;
                self.edge(body_return, self.terminal, EdgeKind::Continue)?;
                let body = self.plan_expr(body, ctx, body_return, EdgeKind::Continue, 0)?;
                let captures = captures.iter().collect::<Vec<_>>();
                let (capture_entry, capture_occurrences) =
                    self.plan_sequence(&captures, ctx, successor, exit_kind)?;
                let mut children = Vec::with_capacity(1 + capture_occurrences.len());
                children.push(body.occurrence);
                children.extend(capture_occurrences);
                let node =
                    self.expression_node(TransitionKind::Evaluate, owner, frame, expr, &children)?;
                self.edge(
                    node.entry,
                    capture_entry,
                    if captures.is_empty() {
                        exit_kind
                    } else {
                        EdgeKind::Continue
                    },
                )?;
                self.register_static_body(node.entry, body)?;
                Ok(node)
            }
            RuntimeExpr::PrimitiveCall { args, .. } | RuntimeExpr::Construct { args, .. } => {
                let expressions = args.iter().collect::<Vec<_>>();
                let (first, operand_occurrences) =
                    self.plan_sequence(&expressions, ctx, successor, exit_kind)?;
                let node = self.expression_node(
                    TransitionKind::Sequence,
                    owner,
                    frame,
                    expr,
                    &operand_occurrences,
                )?;
                self.edge(
                    node.entry,
                    first,
                    if expressions.is_empty() {
                        exit_kind
                    } else {
                        EdgeKind::Continue
                    },
                )?;
                Ok(node)
            }
            RuntimeExpr::Record { fields } => {
                let expressions = fields.iter().map(|(_, value)| value).collect::<Vec<_>>();
                let (first, operand_occurrences) =
                    self.plan_sequence(&expressions, ctx, successor, exit_kind)?;
                let node = self.expression_node(
                    TransitionKind::Sequence,
                    owner,
                    frame,
                    expr,
                    &operand_occurrences,
                )?;
                self.edge(
                    node.entry,
                    first,
                    if expressions.is_empty() {
                        exit_kind
                    } else {
                        EdgeKind::Continue
                    },
                )?;
                Ok(node)
            }
            RuntimeExpr::Call { callee, args } => {
                let mut expressions = Vec::with_capacity(args.len() + 1);
                expressions.push(callee.as_ref());
                expressions.extend(args);
                let (first, operand_occurrences) =
                    self.plan_sequence(&expressions, ctx, successor, exit_kind)?;
                let node = self.expression_node(
                    TransitionKind::Sequence,
                    owner,
                    frame,
                    expr,
                    &operand_occurrences,
                )?;
                self.edge(
                    node.entry,
                    first,
                    if expressions.is_empty() {
                        exit_kind
                    } else {
                        EdgeKind::Continue
                    },
                )?;
                Ok(node)
            }
            RuntimeExpr::Effect {
                capability, args, ..
            } => {
                let mut expressions =
                    Vec::with_capacity(args.len() + usize::from(capability.is_some()));
                if let Some(capability) = capability {
                    expressions.push(capability.value.as_ref());
                }
                expressions.extend(args);
                let (first, operand_occurrences) =
                    self.plan_sequence(&expressions, ctx, successor, exit_kind)?;
                let node = self.expression_node(
                    TransitionKind::Sequence,
                    owner,
                    frame,
                    expr,
                    &operand_occurrences,
                )?;
                self.edge(
                    node.entry,
                    first,
                    if expressions.is_empty() {
                        exit_kind
                    } else {
                        EdgeKind::Continue
                    },
                )?;
                Ok(node)
            }
        }
    }

    pub(super) fn finish(
        mut self,
        symbols: &crate::NativeProcessSymbols,
        root_ingress: AbiRootIngress,
        functionized_units: bool,
    ) -> Result<StaticTransitionPlan<'src>, CraneliftBackendError> {
        let (synthesized_identities, synthesized_io_roles) =
            build_synthesized_constructor_inventory(&mut self.plan.semantic_material, symbols)?;
        let bool_identities =
            build_bool_constructor_inventory(&mut self.plan.semantic_material, symbols)?;
        let planned_entry_bodies = self.plan.planned_entry_bodies.clone();
        let entry_bodies = |entry: StaticNodeId| {
            planned_entry_bodies
                .iter()
                .find(|pair| pair.entry == entry)
                .map(|pair| pair.body_occurrence)
        };
        self.plan.semantic = build_semantic_plane(
            &self.plan.nodes,
            &self.plan.edges,
            &self.plan.entries,
            &entry_bodies,
            self.plan.root_entry,
            &self.plan.semantic_sources,
            &self.plan.semantic_material,
        )?;
        self.plan
            .semantic
            .install_synthesized_constructor_inventory(
                synthesized_identities,
                synthesized_io_roles,
            );
        self.plan
            .semantic
            .validate_synthesized_constructor_inventory()?;
        self.plan
            .semantic
            .install_bool_constructor_inventory(bool_identities);
        self.plan.semantic.validate_bool_constructor_inventory()?;
        // Slice 0's substrate closes before ABI descriptors or any emitted
        // allocation can exist. The two populations are independently
        // re-derived and checked; their cross-population closure is a third,
        // separately named gate.
        self.plan.case_emissions = build_case_emission_plan(&self.plan)?;
        validate_case_emission_plan(&self.plan, &self.plan.case_emissions)?;
        self.plan.occurrence_authorities = build_occurrence_authority_plan(&self.plan)?;
        validate_occurrence_authority_plan(&self.plan, &self.plan.occurrence_authorities)?;
        validate_substrate_preallocation_closure(
            &self.plan,
            &self.plan.case_emissions,
            &self.plan.occurrence_authorities,
        )?;
        // `B2R` — the representation contract is built from the owner partition
        // the line above just validated, and it fails **before** anything is
        // emitted. It is deliberately not deferred to lowering: a contract that
        // is only checked at emission time cannot be a *pre*-emission gate.
        let root_entry = self
            .plan
            .root_entry
            .ok_or_else(|| planner_error("plan has no root scheduling entry"))?;
        self.plan.root_ingress = root_ingress;
        // `RT-DECL-CLOSURE-PORT` `D2` — the declaration occurrences are the
        // discriminator that splits a `StaticBody` target into a callable
        // declaration unit or an anonymous closure body. They come from the one
        // loop that plans transparent declarations, so the ABI plane cannot
        // classify a unit as declaration-owned unless the planner actually
        // planned that declaration.
        let declaration_origins: BTreeSet<StaticOriginId> = self
            .plan
            .declaration_occurrences
            .values()
            .copied()
            .collect();
        self.plan.abi = build_abi_plane(
            &self.plan.semantic,
            &self.plan.nodes,
            &self.plan.semantic_sources,
            &self.plan.edges,
            &self.plan.entries,
            &declaration_origins,
            root_entry,
            root_ingress,
        )?;
        let (
            continuation_specializations,
            continuation_specialization_calls,
            required_consumer_projections,
            continuation_contexts,
            _admitted_discoveries,
        ) = build_continuation_specialization_plan(&self.plan)?;
        self.plan.continuation_specializations = continuation_specializations;
        self.plan.continuation_specialization_calls = continuation_specialization_calls;
        self.plan.required_consumer_projections = required_consumer_projections;
        self.plan.continuation_contexts = continuation_contexts;
        validate_continuation_specialization_plan(&self.plan)?;
        install_continuation_specialization_abi(
            &mut self.plan.abi,
            &self.plan.continuation_specializations,
        )?;
        // CP2 installs the complete response demand population into the SAME
        // ordinary context identity plane, after causal specialization planning
        // has closed but before any context ABI exists. The scratch causal ABI
        // below is an independent prefix oracle: appending response-only keys
        // must leave every prior descriptor, slot, input and affinity byte
        // unchanged, not merely leave the context count plausible.
        let causal_contexts = self.plan.continuation_contexts.clone();
        self.plan.install_static_response_context_plan()?;
        let mut causal_abi = self.plan.abi.clone();
        install_continuation_context_abi(&mut causal_abi, &causal_contexts)?;
        // `D5a`: the generated contexts' own ABI, in its own arenas. ⛔ Installed
        // AFTER the specialization ABI and into separate vectors, never appended
        // to `continuation_descriptors` -- that population is exactly the
        // continuation-callee partition, and admitting a caller-side context
        // there would make one identity domain readable as the other.
        install_continuation_context_abi(&mut self.plan.abi, &self.plan.continuation_contexts)?;
        validate_causal_context_abi_prefix(&self.plan.abi, &causal_abi)?;
        // `D3b` STAGE 2 — every context id now exists, so every structural frame
        // requirement can be resolved to exactly one identity.
        //
        // ⛔ **Placed AFTER validation deliberately.** The validator re-derives
        // the whole continuation plan and compares it for exact equality; a
        // finalized sibling field stamped before that runs is state the
        // re-derivation cannot produce, so the comparison would fail on the
        // stamping rather than on any real disagreement. Finalization is a
        // post-derivation publication step, not part of the derivation.
        //
        // ⭐ Still before anything can publish a view: `continuation_units` and
        // `continuation_contexts` both require the ABI installed above, so the
        // earliest possible view is built after this line.
        finalize_continuation_availability_plan(&mut self.plan)?;
        self.plan.join_results = build_join_result_plan(&self.plan, functionized_units)?;
        // `D7` — the aggregate occurrence population is built HERE, last, and
        // deliberately not beside the occurrence authorities it also reads.
        //
        // Two inputs make the position load-bearing, and only one of them was
        // obvious. The meet is taken over the per-child lifetimes in
        // `occurrence_authorities`, so it cannot precede those. It is ALSO
        // taken over each child's planned result representation, which is
        // `join_results` and is not built until this line — a child the
        // emitter will materialize as a native scalar pair has no referent at
        // all, and reading the lifetime without the representation makes every
        // call-shaped child look arena-owned.
        self.plan.aggregate_ownership = build_aggregate_ownership_plan(&self.plan)?;
        validate_aggregate_ownership_plan(&self.plan, &self.plan.aggregate_ownership)?;
        self.plan.checked_ih_environment_transports =
            build_checked_ih_environment_transports(&self.plan)?;
        validate_checked_ih_environment_transports(
            &self.plan,
            &self.plan.checked_ih_environment_transports,
        )?;
        // PHASE B of the response context install (RECUT 2, HS5, Architect
        // evt_7eh84c8n6w08e). aggregate_ownership and the transport records are
        // now final, so the exact record-derived transport-source set -- the real
        // Deferred/Specialized discriminator -- exists. Phase A (:1213) minted the
        // owner-less context entries the context ABI (:1221) already covers; this
        // assigns owners to Specialized and seals the P1 UNION P2 residual. The
        // determination is genuinely post-install (the z1315 cycle), so it cannot
        // run at :1213; owner-additive, so phase A's entries are untouched.
        self.plan.install_static_response_context_plan_phase_b()?;
        self.plan.checked_ih_continuation_inheritances =
            build_checked_ih_continuation_inheritances(&self.plan)?;
        #[cfg(feature = "px8-ds-test-support")]
        apply_checked_ih_continuation_inheritance_mutation(
            &mut self.plan.checked_ih_continuation_inheritances,
        );
        validate_checked_ih_continuation_inheritances(
            &self.plan,
            &self.plan.checked_ih_continuation_inheritances,
        )?;
        self.plan.checked_ih_generated_entry_confluences =
            build_checked_ih_generated_entry_confluences(&self.plan)?;
        validate_checked_ih_generated_entry_confluences(
            &self.plan,
            &self.plan.checked_ih_generated_entry_confluences,
        )?;
        self.plan.checked_ih_generated_entry_accesses =
            build_checked_ih_generated_entry_accesses(
                &self.plan,
                &self.plan.checked_ih_generated_entry_confluences,
            )?;
        validate_checked_ih_generated_entry_accesses(
            &self.plan,
            &self.plan.checked_ih_generated_entry_confluences,
            &self.plan.checked_ih_generated_entry_accesses,
        )?;
        #[cfg(feature = "px8-ds-test-support")]
        record_checked_ih_generated_entry_confluences(
            &self.plan.checked_ih_generated_entry_confluences,
        );
        #[cfg(feature = "px8-ds-test-support")]
        record_checked_ih_generated_entry_admissions(
            &self.plan,
            &self.plan.checked_ih_generated_entry_accesses,
        )?;
        #[cfg(feature = "px8-ds-test-support")]
        if !checked_ih_intervening_binder_population_control_is_active() {
            record_checked_ih_continuation_inheritances(
                &self.plan,
                &self.plan.checked_ih_continuation_inheritances,
            );
        }
        // ⛔ After `join_results` for the same reason the ownership plan is: a
        // seat's consumer phase is a fact about the child's planned result
        // representation, which does not exist until that line.
        self.plan.host_effect_seats = build_host_effect_seat_plan(&self.plan)?;
        validate_host_effect_seat_plan(&self.plan, &self.plan.host_effect_seats)?;
        // This compiler-generated residual is authorized by the same catalog as
        // source traps. Append it only after every source-derived entry so
        // existing planned identities remain stable; lowering can then replace
        // the residual scalar with the catalog identity rather than minting a
        // second sentinel namespace.
        self.intern_trap(&super::joins_traps::malformed_dynamic_constructor_trap())?;
        #[cfg(test)]
        apply_static_worker_member_mutation(&mut self.plan);
        self.plan.validate()?;
        #[cfg(feature = "px8-ds-test-support")]
        run_checked_ih_intervening_binder_population_control(&self.plan)?;
        Ok(self.plan)
    }
}

// [from the StaticTransitionPlan impl -- registration write-path]
impl<'src> StaticTransitionPlan<'src> {
    pub(super) fn register_scheduling_entry(&mut self, planned: PlannedExpr) {
        self.entries.push(planned.entry);
        self.record_planned_entry_body(planned);
    }

    fn record_planned_entry_body(&mut self, planned: PlannedExpr) {
        self.planned_entry_bodies.push(PlannedEntryBody {
            entry: planned.entry,
            body_occurrence: planned.occurrence,
        });
    }
}

/// Prove that adding response-issued contexts appended to, rather than rebuilt,
/// every causal context ABI arena. Comparing the complete causal-only arenas to
/// exact installed prefixes covers descriptor IDs and byte layout together.
fn validate_causal_context_abi_prefix(
    installed: &AbiPlane,
    causal: &AbiPlane,
) -> Result<(), CraneliftBackendError> {
    let descriptors = installed
        .context_descriptors
        .get(..causal.context_descriptors.len());
    let slots = installed.context_slots.get(..causal.context_slots.len());
    let inputs = installed.context_inputs.get(..causal.context_inputs.len());
    let affinities = installed
        .context_affinities
        .get(..causal.context_affinities.len());
    if descriptors != Some(causal.context_descriptors.as_slice())
        || slots != Some(causal.context_slots.as_slice())
        || inputs != Some(causal.context_inputs.as_slice())
        || affinities != Some(causal.context_affinities.as_slice())
    {
        return Err(planner_error(
            "installing response contexts changed a causal context ABI prefix",
        ));
    }
    Ok(())
}

fn runtime_expr_tag(expr: &RuntimeExpr) -> u32 {
    match expr {
        RuntimeExpr::CheckedJoinSite { .. } => 0,
        RuntimeExpr::CheckedSubcontinuationFrame { .. } => 1,
        RuntimeExpr::CheckedRecursiveInvocation { .. } => 2,
        RuntimeExpr::CheckedComputationalIHSlots { .. } => 3,
        RuntimeExpr::CheckedComputationalIHInvocation { .. } => 4,
        RuntimeExpr::Value(_) => 5,
        RuntimeExpr::Var(_) => 6,
        RuntimeExpr::Let { .. } => 7,
        RuntimeExpr::If { .. } => 8,
        RuntimeExpr::PrimitiveCall { .. } => 9,
        RuntimeExpr::Construct { .. } => 10,
        RuntimeExpr::Match { .. } => 11,
        RuntimeExpr::ComputationalMatch { .. } => 12,
        RuntimeExpr::Record { .. } => 13,
        RuntimeExpr::Project { .. } => 14,
        RuntimeExpr::Closure { .. } => 15,
        RuntimeExpr::LexicalClosure { .. } => 16,
        RuntimeExpr::DeclarationRef { .. } => 17,
        RuntimeExpr::ImportedDeclarationRef { .. } => 18,
        RuntimeExpr::Call { .. } => 19,
        RuntimeExpr::Effect { .. } => 20,
        RuntimeExpr::Trap(_) => 21,
    }
}

#[cfg(test)]
mod tests {

    use super::super::*;
    use crate::RuntimeValue;
    use super::super::tests::{
        b2ac_topology_fixtures, b2o_transparent_declaration, d2_declaration_and_anonymous_closure,
    };


    /// A canonical digest of the Boundary-A transfer graph: node transitions in
    /// order, then every edge as `(from, to, kind)` in order.
    #[cfg(test)]
    pub(in crate::cranelift_backend::planning::static_transition) fn b2ac_topology_digest(expr: &RuntimeExpr) -> String {
        let plan = plan_static_transition_graph(expr, &BTreeMap::new()).expect("plannable");
        let mut digest = String::new();
        digest.push_str(&format!(
            "nodes={} edges={}",
            plan.nodes.len(),
            plan.edges.len()
        ));
        for node in &plan.nodes {
            digest.push_str(&format!("|n{}:{:?}", node.id.0, node.transition));
        }
        for edge in &plan.edges {
            digest.push_str(&format!(
                "|e{}:{}->{}:{:?}",
                edge.id.0, edge.from.0, edge.to.0, edge.kind
            ));
        }
        digest.push_str(&format!("|entries={:?}", plan.entries));
        digest
    }

    /// **AC-11 — every transfer edge is unchanged and consumes `.entry`.**
    ///
    /// These digests were captured by running the identical probe against the
    /// WP's base commit `70bd2c74` — before `PlannedExpr` existed — in a scratch
    /// worktree, and are asserted here against the post-D9 planner. Equality is
    /// the mechanical proof that the Boundary-A graph is topologically
    /// identical: same nodes in the same order, same edges with the same
    /// `(from, to, kind)`, same scheduling entries.
    ///
    /// ## ⚠ Reproducing the baseline — the recipe, because equality hides
    /// its own provenance
    ///
    /// ⛔ The asserted property is *equality against committed constants*, so a
    /// re-capture taken **after** the change would have produced byte-identical
    /// values. **Nothing in this file distinguishes a genuine pre-change baseline
    /// from a re-recording**, and the scratch worktree it was taken in is gone. So
    /// the binding is demonstrated here rather than testified to — anyone can
    /// redo it in about two minutes:
    ///
    /// ```text
    /// git worktree add --detach /tmp/b2ac-base 70bd2c74
    /// # port these two functions into that tree's test module verbatim:
    /// #   `b2ac_topology_fixtures`  (the seven fixtures, by name)
    /// #   `b2ac_topology_digest`    (nodes, edges, entries -- it reads nothing
    /// #                              that postdates the base, which is why it
    /// #                              compiles there at all)
    /// cd /tmp/b2ac-base
    /// scripts/ken-cargo test -p ken-runtime --lib -- b2ac_topology
    /// git worktree remove /tmp/b2ac-base
    /// ```
    ///
    /// ⛔ `scripts/ken-cargo`, never raw `cargo` — `COORDINATION §12`, and it
    /// binds inside a copied recipe exactly as it binds anywhere else. A recipe
    /// that
    /// spells the raw command teaches the next reader to bypass the build lock.
    ///
    /// Verified this way by the adversary on `2db29abe`: **all seven rows
    /// reproduce byte-for-byte** from `70bd2c74`, including
    /// `computational-under-let`, which is the row carrying the load.
    ///
    /// ⭐ Read `computational-under-let`: the parent `Sequence` (n12) edges to
    /// **n11**, the computational match's scrutinee, and *not* to the
    /// `SourceReturnResume` (n6). That is D9's promise — the occurrence moved to
    /// the resume while the schedule stayed on the scrutinee — and this row is
    /// what would redden if a future change returned the resume as the entry.
    #[cfg(test)]
    const B2AC_BASE_TOPOLOGY: &[(&str, &str)] = &[
        ("leaf", "nodes=3 edges=1|n0:Terminal|n1:TrapTerminal|n2:Evaluate|e0:2->0:Continue|entries=[StaticNodeId(2)]"),
        ("let-if", "nodes=9 edges=8|n0:Terminal|n1:TrapTerminal|n2:Evaluate|n3:Evaluate|n4:Evaluate|n5:Branch|n6:Evaluate|n7:Evaluate|n8:Sequence|e0:2->0:Continue|e1:3->2:Continue|e2:4->2:Continue|e3:5->3:Select|e4:5->4:Reject|e5:6->5:Continue|e6:7->6:Continue|e7:8->7:Continue|entries=[StaticNodeId(8)]"),
        ("match", "nodes=7 edges=6|n0:Terminal|n1:TrapTerminal|n2:Evaluate|n3:Evaluate|n4:CaseTest|n5:Evaluate|n6:Evaluate|e0:2->1:Trap|e1:3->0:Continue|e2:4->3:Select|e3:4->2:Reject|e4:5->4:Continue|e5:6->5:Continue|entries=[StaticNodeId(6)]"),
        ("lexical-closure-call", "nodes=8 edges=7|n0:Terminal|n1:TrapTerminal|n2:Evaluate|n3:ClosureBody|n4:Evaluate|n5:Evaluate|n6:Evaluate|n7:Sequence|e0:2->0:Continue|e1:3->0:Continue|e2:4->3:Continue|e3:5->2:Continue|e4:6->5:Continue|e5:6->4:StaticBody|e6:7->6:Continue|entries=[StaticNodeId(7)]"),
        ("computational", "nodes=11 edges=10|n0:Terminal|n1:TrapTerminal|n2:CompletedTail|n3:ProducerTail|n4:ProducerWrapper|n5:SourceReturnResume|n6:Evaluate|n7:Evaluate|n8:CaseTest|n9:Evaluate|n10:Sequence|e0:5->4:InvokeProducerWrapper|e1:4->3:InvokeProducerTail|e2:3->2:CompleteProducerTail|e3:2->0:Continue|e4:6->1:Trap|e5:7->5:SourceReturnOwnedResume|e6:8->7:Select|e7:8->6:Reject|e8:9->8:Continue|e9:10->9:Continue|entries=[StaticNodeId(10)]"),
        ("computational-nested", "nodes=19 edges=19|n0:Terminal|n1:TrapTerminal|n2:CompletedTail|n3:ProducerTail|n4:ProducerWrapper|n5:SourceReturnResume|n6:Evaluate|n7:CompletedTail|n8:ProducerTail|n9:ProducerWrapper|n10:SourceReturnResume|n11:Evaluate|n12:Evaluate|n13:CaseTest|n14:Evaluate|n15:Sequence|n16:CaseTest|n17:Evaluate|n18:Sequence|e0:5->4:InvokeProducerWrapper|e1:4->3:InvokeProducerTail|e2:3->2:CompleteProducerTail|e3:2->0:Continue|e4:6->1:Trap|e5:10->9:InvokeProducerWrapper|e6:9->8:InvokeProducerTail|e7:8->7:CompleteProducerTail|e8:7->5:SourceReturnOwnedResume|e9:11->1:Trap|e10:12->10:SourceReturnOwnedResume|e11:13->12:Select|e12:13->11:Reject|e13:14->13:Continue|e14:15->14:Continue|e15:16->15:Select|e16:16->6:Reject|e17:17->16:Continue|e18:18->17:Continue|entries=[StaticNodeId(18)]"),
        ("computational-under-let", "nodes=13 edges=12|n0:Terminal|n1:TrapTerminal|n2:Evaluate|n3:CompletedTail|n4:ProducerTail|n5:ProducerWrapper|n6:SourceReturnResume|n7:Evaluate|n8:Evaluate|n9:CaseTest|n10:Evaluate|n11:Sequence|n12:Sequence|e0:2->0:Continue|e1:6->5:InvokeProducerWrapper|e2:5->4:InvokeProducerTail|e3:4->3:CompleteProducerTail|e4:3->2:Continue|e5:7->1:Trap|e6:8->6:SourceReturnOwnedResume|e7:9->8:Select|e8:9->7:Reject|e9:10->9:Continue|e10:11->10:Continue|e11:12->11:Continue|entries=[StaticNodeId(12)]"),
    ];

    #[test]
    fn boundary_a_topology_is_identical_to_the_pre_d9_planner() {
        let expected: BTreeMap<&str, &str> = B2AC_BASE_TOPOLOGY.iter().copied().collect();
        for (name, expr) in b2ac_topology_fixtures() {
            let digest = b2ac_topology_digest(&expr);
            let base = expected
                .get(name)
                .expect("every fixture has a recorded base digest");
            assert_eq!(
                &digest.as_str(),
                base,
                "AC-11: `{name}` changed the Boundary-A transfer graph. D9 must move \
                 only which identity is RECORDED at a source position, never which \
                 node is SCHEDULED."
            );
        }
    }

    /// **AC-13 — the split is exactly one variant.**
    #[test]
    fn computational_match_is_the_sole_entry_occurrence_split() {
        let mut split = Vec::new();
        for (name, expr) in b2ac_topology_fixtures() {
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
            let planned = planner
                .plan_expr(&expr, context, planner.terminal, EdgeKind::Continue, 0)
                .expect("plannable");
            if planned.occurrence != origin_of(planned.entry) {
                split.push(name);
            }
        }
        assert_eq!(
            split,
            vec!["computational", "computational-nested"],
            "AC-13: only a `ComputationalMatch` result may split entry from \
             occurrence, and every such result must. `computational-under-let` is \
             a `Let` at the root, so its own result does not split."
        );
    }


    /// Emit one fixture end to end, returning the failure text if it refuses.
    #[cfg(test)]
    pub(in crate::cranelift_backend::planning::static_transition) fn ac3_emit(
        root: &RuntimeExpr,
        declarations: &BTreeMap<&str, &RuntimeDeclaration>,
    ) -> Result<(), String> {
        use crate::cranelift_backend::artifact::new_object_module_for_lowering_tests;
        use crate::cranelift_backend::lowering::core::{
            compile_expr_into_object_module, NativeSeedEnvironment,
        };
        let seed_env = NativeSeedEnvironment::empty();
        compile_expr_into_object_module(
            new_object_module_for_lowering_tests("ac3")
                .map_err(|error| format!("{error:?}"))?,
            "ac3_entry",
            cranelift_module::Linkage::Export,
            root,
            &seed_env,
            declarations.clone(),
            None,
            true,
            None,
            Some(crate::cranelift_backend::test_support::test_only_distinguished_root_join_plan()),
            None,
        )
        .map(|_| ())
        .map_err(|error| format!("{error:?}"))
    }

    /// **`RT-BODY-OCCURRENCE-PROVENANCE` `AC-3` — collapsing the issued body
    /// back to the scheduling entry recreates the traversal/closeout failure.**
    ///
    /// > **PROPERTY:** the issued body occurrence is what makes the source
    /// > traversal reach the unit's join subtree.
    /// > **OPERAND THAT MOVED:** the **population** — the value the planner
    /// > issues for a `SchedulingEntry` seed, mutated at the seat that resolves
    /// > it, back to the pre-correction `StaticOriginId(seed.0)` alias. No
    /// > detector, assertion or validator arm was touched.
    /// > **OBSERVED BOUNDARY:** `finalize_join_disposition` refuses with
    /// > *"function left planned source join … neither emitted nor statically
    /// > unselected"* — a required join reached by neither consumption nor
    /// > disposition, which is the attribution record's failure exactly.
    ///
    /// **Population-side is the whole point.** `AC-3` asserts REACH. A
    /// detector-side mutation would redden this same test name while the
    /// carried value never moved, and would keep reddening for the entire life
    /// of a correction that reached nothing.
    ///
    /// **The `Exact` arm is not a fourth assertion — it is what validates the
    /// other three.** A refusal control only has to reach its own guard; the
    /// success arm has to traverse every guard, so it is the only arm that
    /// establishes the fixture could have lowered at all. Without it, a fixture
    /// broken upstream would refuse under both settings and read as a discharge.
    ///
    /// **Honest scope of the two arms.** The root arm exercises the arm that
    /// previously carried a *workaround* (`define_unit_body`'s `is_root`
    /// substitution), so it demonstrates the mechanism rather than the shipped
    /// defect. The **non-root** arm is the population that was actually broken:
    /// nothing compensated for it, which is why the defect shipped. Both are
    /// asserted; neither is offered as the other.
    #[test]
    fn collapsing_the_body_to_its_scheduling_entry_recreates_the_closeout_failure() {
        use super::super::semantic_ir::{with_body_occurrence_mutation, BodyOccurrenceMutation};

        let (_, computational) = b2ac_topology_fixtures()
            .into_iter()
            .find(|(name, _)| *name == "computational")
            .expect("the computational fixture");

        // Arm 1: the ROOT unit.
        let empty = BTreeMap::new();
        assert_eq!(
            ac3_emit(&computational, &empty),
            Ok(()),
            "AC-3 positive control: the root fixture must lower under the exact \
             pairing, or the refusals below prove nothing about the pairing"
        );
        let collapsed_root = with_body_occurrence_mutation(
            BodyOccurrenceMutation::CollapseSchedulingEntryBody,
            || ac3_emit(&computational, &empty),
        );
        assert_eq!(
            collapsed_root,
            Err(
                "Backend(Module(\"function left planned source join StaticOriginId(5) \
                 neither emitted nor statically unselected\"))"
                    .to_string()
            ),
            "AC-3: with the body collapsed to the scheduling entry the traversal \
             enters the entry and never reaches the join subtree, so closeout \
             finds a required join neither consumed nor dispositioned"
        );

        // Arm 2: a NON-ROOT unit — the population that actually shipped broken.
        let declaration = b2o_transparent_declaration(computational.clone());
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::b2o", &declaration);
        let root = RuntimeExpr::Value(RuntimeValue::Bool(true));
        assert_eq!(
            ac3_emit(&root, &declarations),
            Ok(()),
            "AC-3 positive control: the non-root fixture must lower under the \
             exact pairing"
        );
        let collapsed_declaration = with_body_occurrence_mutation(
            BodyOccurrenceMutation::CollapseSchedulingEntryBody,
            || ac3_emit(&root, &declarations),
        );
        assert!(
            collapsed_declaration
                .as_ref()
                .err()
                .is_some_and(|message| message.contains("planned source join")),
            "AC-3: the non-root unit is the population the removed root-only \
             substitution never covered; collapsing its body must recreate the \
             same closeout failure. got {collapsed_declaration:?}"
        );
    }


    /// **`RT-BODY-OCCURRENCE-PROVENANCE` `AC-1b` — the `StaticBodyTarget` class
    /// takes its ISSUED pair, not its seed's own ordinal.**
    ///
    /// > **MEASURED:** for a closure whose body is a computational match, the
    /// > unit seeded on the `StaticBody` target carries a body occurrence that
    /// > differs from `origin_of(seed)`, and equals the pair issued when that
    /// > body's `StaticBody` edge was registered.
    /// > **CLAIMED:** the retired `StaticOriginId(edge.to.0)` fallback is gone
    /// > and this class reads the one relation.
    /// > **THE GAP:** the fixture's closure body must genuinely schedule
    /// > something before itself. For an ordinary body the seed's own ordinal
    /// > IS its occurrence, so the fallback and the relation agree and the test
    /// > passes under both -- which is exactly how the carve-out survived
    /// > review the first time.
    ///
    /// This is the class the original bounded contract exempted as
    /// already-grounded. On venue 4 that exemption issued `SOI(58)` to a unit
    /// whose real body was `SOI(26)`, and its four planned joins were never
    /// entered.
    #[test]
    fn a_static_body_target_whose_body_is_computational_takes_its_issued_pair() {
        let (_, computational) = b2ac_topology_fixtures()
            .into_iter()
            .find(|(name, _)| *name == "computational")
            .expect("the computational fixture");
        let expr = RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["x".to_string()],
                body: Box::new(computational),
            }),
            args: vec![RuntimeExpr::Value(RuntimeValue::Bool(true))],
        };
        let plan = plan_static_transition_graph(&expr, &BTreeMap::new()).expect("plannable");

        let body_edge = plan
            .edges
            .iter()
            .find(|edge| edge.kind == EdgeKind::StaticBody)
            .expect("precondition: the fixture must carry a static body edge");
        let unit = plan
            .semantic
            .functions
            .iter()
            .find(|function| function.planned_node == body_edge.to)
            .expect("the static body target seeded a function unit");

        assert_ne!(
            unit.body_occurrence,
            origin_of(unit.planned_node),
            "AC-1b precondition AND claim: the closure body must schedule \
             something before itself, so the retired fallback \
             `StaticOriginId(edge.to.0)` and the issued pair DISAGREE here. If \
             they agreed, this test would pass under the carve-out too"
        );
        assert_eq!(
            plan.planned_entry_body(body_edge.to),
            Some(unit.body_occurrence),
            "AC-1b: the unit reads the row issued when its static body edge was \
             registered -- one relation, not a per-class rule"
        );
    }

    /// **`AC-3` `StaticBodyTarget` arm — the CLASS-SELECTIVE collapse.**
    ///
    /// > **OPERAND THAT MOVED:** the population, restricted to the
    /// > `StaticBodyTarget` class -- the retired `StaticOriginId(edge.to.0)`
    /// > fallback restored for that class ONLY.
    ///
    /// A global collapse reddens first through the `SchedulingEntry` class and
    /// therefore says nothing about this one. The informative side is the arm
    /// that would still green if this class were left on the fallback, which is
    /// why the mutation has to be class-selective rather than plan-wide.
    #[test]
    fn collapsing_only_the_static_body_target_class_is_refused() {
        use super::super::semantic_ir::{with_body_occurrence_mutation, BodyOccurrenceMutation};
        let (_, computational) = b2ac_topology_fixtures()
            .into_iter()
            .find(|(name, _)| *name == "computational")
            .expect("the computational fixture");
        let expr = RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["x".to_string()],
                body: Box::new(computational),
            }),
            args: vec![RuntimeExpr::Value(RuntimeValue::Bool(true))],
        };
        let empty = BTreeMap::new();
        assert_eq!(
            ac3_emit(&expr, &empty),
            Ok(()),
            "AC-3 positive control: the fixture must lower under the exact \
             relation, or the refusal below proves nothing about the relation"
        );
        let collapsed = with_body_occurrence_mutation(
            BodyOccurrenceMutation::CollapseStaticBodyTargetBody,
            || ac3_emit(&expr, &empty),
        );
        assert!(
            collapsed
                .as_ref()
                .err()
                .is_some_and(|message| message.contains("planned source join")),
            "AC-3: restoring the retired fallback for this class alone must \
             recreate the traversal/closeout failure. got {collapsed:?}"
        );
    }

    /// **`RT-BODY-OCCURRENCE-PROVENANCE` `AC-4` — call identity is the ENTRY
    /// axis and did not move with the body axis.**
    ///
    /// > **MEASURED:** across every `b2ac` fixture, each call edge's
    /// > `callee_origin` equals `origin_of(callee_unit.planned_node)`.
    /// > **CLAIMED:** the correction changed the BODY axis only; call identity
    /// > is invariant under it.
    /// > **THE GAP:** at least one fixture must have a unit whose two axes
    /// > DIFFER, or the equality holds for both readings and the pin cannot
    /// > tell which axis it measured.
    ///
    /// **This is the invariant most easily broken by accident, and the one a
    /// green suite is least likely to catch.** The old `origin` field was an
    /// alias of `planned_node`, so every consumer read the entry axis whether or
    /// not it meant to. Renaming that field in bulk would have silently moved
    /// call identity onto the body axis for exactly the units where the two
    /// differ — the same units the correction targets — and every fixture whose
    /// axes coincide would have stayed green.
    #[test]
    fn call_identity_stays_on_the_entry_axis_after_the_body_axis_moved() {
        let mut fixtures_with_split_axes = 0usize;
        let mut checked_edges = 0usize;

        for (name, expr) in b2ac_topology_fixtures() {
            let plan = plan_static_transition_graph(&expr, &BTreeMap::new())
                .unwrap_or_else(|error| panic!("{name} must plan: {error:?}"));

            fixtures_with_split_axes += usize::from(
                plan.semantic
                    .functions
                    .iter()
                    .any(|function| function.body_occurrence != origin_of(function.planned_node)),
            );

            for edge in plan.emittable_call_edges().expect("call edges") {
                let callee = plan
                    .semantic
                    .functions
                    .iter()
                    .find(|function| function.id == edge.callee())
                    .expect("a call edge names a planned unit");
                assert_eq!(
                    edge.callee_origin(),
                    origin_of(callee.planned_node),
                    "AC-4 [{name}]: a call names the unit it ENTERS. The body \
                     occurrence is where that unit's traversal begins once \
                     inside, and moving call identity onto it would change which \
                     unit a call resolves to"
                );
                checked_edges += 1;
            }
        }

        // Non-vacuity, both axes.
        assert!(
            fixtures_with_split_axes > 0,
            "AC-4 precondition: at least one fixture must have a unit whose entry \
             and body DIFFER, or this test passes under either reading and \
             measures nothing"
        );
        assert!(
            checked_edges > 0,
            "AC-4 precondition: the fixture set must actually produce call edges"
        );
    }

    /// **`RT-BODY-OCCURRENCE-PROVENANCE` `AC-1` — the issued pair is `n18 ->
    /// n5`, and `n10` is not registered as that unit's body.**
    ///
    /// > **MEASURED:** on the frozen `computational-nested` fixture, the sole
    /// > row of the pairing authority is `(n18, origin_of(n5))`, and no
    /// > `PredeclaredFunction` carries `origin_of(n10)` as its body.
    /// > **CLAIMED:** the registration binds the OUTER scheduling entry to the
    /// > outer body occurrence its own visit returned, and excludes the nested
    /// > call's occurrence.
    /// > **THE GAP:** `n10` must actually EXIST and be the inner match's
    /// > occurrence. "`n10` is not registered" is vacuously true of a node the
    /// > fixture never planned, so the exclusion carries no information until
    /// > the excluded thing is shown to be the real, competing candidate.
    ///
    /// **The non-vacuity arms are the test.** Both are asserted here rather
    /// than assumed: `n5 != n10` (two distinct resumes exist under one entry, so
    /// there is a genuine choice to get wrong) and `origin_of(n18) != n5` (the
    /// entry is not the body, so a pin that read the entry would differ). On a
    /// fixture where the axes coincide this test would pass while measuring
    /// nothing.
    ///
    /// The exact node identities are a **normative compatibility vector**:
    /// they are the frozen `B2AC_BASE_TOPOLOGY` row for this fixture, which
    /// pins `nodes=19`, `n5`/`n10` as the two `SourceReturnResume` nodes and
    /// `entries=[StaticNodeId(18)]`. A topology change reddens that row first.
    #[test]
    fn nested_registration_issues_the_outer_pair_and_excludes_the_inner_resume() {
        let (_, nested) = b2ac_topology_fixtures()
            .into_iter()
            .find(|(name, _)| *name == "computational-nested")
            .expect("the nested computational fixture");
        let plan = plan_static_transition_graph(&nested, &BTreeMap::new()).expect("plannable");

        let n18 = StaticNodeId(18);
        let n5 = origin_of(StaticNodeId(5));
        let n10 = origin_of(StaticNodeId(10));

        // Non-vacuity, before anything is concluded from an absence.
        assert_ne!(
            n5, n10,
            "AC-1 precondition: the fixture must supply TWO distinct resumes \
             under one entry, or there is no wrong answer available to exclude"
        );
        assert_ne!(
            origin_of(n18),
            n5,
            "AC-1 precondition: entry and body must differ on this fixture, or \
             reading the entry would be indistinguishable from reading the body"
        );
        let outer = plan.root_static_origin().expect("root occurrence");
        assert_eq!(outer, n5, "the outer occurrence is the outer resume");
        assert_eq!(
            plan.child_static_origin(outer, 1)
                .expect("the outer match's case body resolves"),
            n10,
            "AC-1 precondition: `n10` is the INNER match's occurrence — a real, \
             competing candidate, not an absent node"
        );

        // The issued pairing itself.
        assert_eq!(
            plan.planned_entry_bodies,
            vec![PlannedEntryBody {
                entry: n18,
                body_occurrence: n5,
            }],
            "AC-1: the sole issued pair binds the outer scheduling entry to the \
             outer body occurrence its own visit returned"
        );

        // And no unit claims the nested call's occurrence as its body.
        assert!(
            plan.semantic
                .functions
                .iter()
                .all(|function| function.body_occurrence != n10),
            "AC-1: `n10` is the occurrence the NESTED call returned to its \
             parent; recovering an `outermost` resume by graph shape is exactly \
             what would select it"
        );
    }

    /// **`RT-BODY-OCCURRENCE-PROVENANCE` supporting discrimination — a NON-ROOT
    /// unit whose body schedules something before itself has entry != body.**
    ///
    /// **This is NOT the node's `AC-2`, and it must not be read as discharging
    /// it.** The node's `AC-2` is the exact `LiftRose` synthetic-venue result:
    /// owner 2's required `{26, 33, 39, 53}` reached and closed. This test is
    /// obligation 2 of the LEADER'S DISPATCH list, which numbers differently
    /// from the node's acceptance table — an earlier revision of this file
    /// labelled it `AC-2` and thereby claimed a gate it does not touch. The
    /// node's table is the authority; a dispatch's ordering is not.
    ///
    /// > **MEASURED:** for a transparent declaration whose body is a
    /// > computational match, the unit's `body_occurrence` differs from
    /// > `origin_of(planned_node)`, and equals the declaration occurrence the
    /// > planner recorded for that symbol.
    /// > **CLAIMED:** the correction reaches NON-ROOT units — the population the
    /// > removed root-only substitution never covered.
    /// > **THE GAP:** the unit must genuinely be non-root. `AC-1`'s fixture has
    /// > exactly one entry and it IS the root, so it cannot discharge this;
    /// > passing it off as coverage would leave the entire defect population
    /// > unmeasured.
    ///
    /// This is the discriminating pair the old code could not produce: before
    /// the correction `body_occurrence` was `StaticOriginId(seed.0)`, so the
    /// first assertion below was an identity and could not fail.
    #[test]
    fn a_non_root_computational_declaration_body_differs_from_its_entry() {
        // Reuse the frozen `computational` fixture shape as the DECLARATION
        // body, so the only thing varying from `AC-1` is root-ness.
        let (_, computational) = b2ac_topology_fixtures()
            .into_iter()
            .find(|(name, _)| *name == "computational")
            .expect("the computational fixture");
        let declaration = b2o_transparent_declaration(computational);
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::b2o", &declaration);
        let root = RuntimeExpr::Value(RuntimeValue::Bool(true));
        let plan = plan_static_transition_graph(&root, &declarations).expect("plannable");

        let root_entry = plan.root_entry.expect("a root entry");
        let declaration_body = plan
            .declaration_occurrence_origin("decl:fixture::b2o")
            .expect("the declaration was planned");

        let pair = plan
            .planned_entry_bodies
            .iter()
            .find(|pair| pair.entry != root_entry)
            .expect("precondition: a non-root scheduling entry exists");

        assert_ne!(
            origin_of(pair.entry),
            pair.body_occurrence,
            "a non-root unit whose body schedules its scrutinee first must \
             not have its entry aliased as its body — this equality is what the \
             correction removed"
        );
        assert_eq!(
            pair.body_occurrence, declaration_body,
            "the issued body is the occurrence the declaration's own visit \
             returned"
        );

        // The unit built from that seed carries the issued value, not the alias.
        let unit = plan
            .semantic
            .functions
            .iter()
            .find(|function| function.planned_node == pair.entry)
            .expect("the non-root seed built a function unit");
        assert_eq!(
            unit.body_occurrence, declaration_body,
            "the carried field is the issued body occurrence"
        );
        assert_ne!(
            unit.body_occurrence,
            origin_of(unit.planned_node),
            "and it is NOT an alias of the scheduling entry"
        );
    }

    /// **`RT-FNSPLIT-B2A-S` AC-5 — keying selection by the scheduling ENTRY
    /// resolves to the WRONG body. Demonstrated, not forbidden by a grep.**
    ///
    /// ⛔ The first candidate discharged AC-5 by scanning for four container
    /// spellings keyed by `StaticNodeId`. The Architect rejected that
    /// (`evt_6sq2tq3v9jcd0`) and was right: a `Vec` indexed by `planned.entry.0`, a
    /// type alias, or a bespoke collection all violate the ruled property while
    /// such a scan stays green. **The property is about which value selects a body,
    /// so the control has to be about that too.**
    #[test]
    fn keying_selection_by_the_scheduling_entry_does_not_resolve_the_body() {
        // Promise class: durable invariant.
        let (_, computational) = b2ac_topology_fixtures()
            .into_iter()
            .find(|(name, _)| *name == "computational")
            .expect("the computational fixture");
        let plan =
            plan_static_transition_graph(&computational, &BTreeMap::new()).expect("plannable");

        let occurrence = plan.root_static_origin().expect("root occurrence");
        let entry = *plan.entries.first().expect("a root entry");
        assert_ne!(
            occurrence,
            origin_of(entry),
            "AC-5: the fixture must actually exhibit the split, or this test is vacuous"
        );

        // What the TAG resolves to: this match.
        let by_tag = plan
            .source_occurrence(occurrence)
            .expect("the occurrence resolves its own body");
        assert!(
            matches!(by_tag, RuntimeExpr::ComputationalMatch { .. }),
            "AC-5: the occurrence must resolve to the match itself"
        );

        // What an ENTRY-keyed lookup would resolve to: anything but this body. It
        // is either a different term or no source occurrence at all -- both are
        // wrong answers for "the body of this match", which is the point.
        let by_entry = plan.source_occurrence(origin_of(entry));
        assert!(
            !matches!(by_entry, Ok(term) if std::ptr::eq(term, by_tag)),
            "AC-5: the scheduling entry must not resolve to the occurrence's body; \
             if it does, entry and occurrence have been conflated again and \
             hard-stop #8 is back"
        );
    }

    /// The definition arms of a `D2` plan, paired with each unit's declared
    /// `(parameters, captures)`.
    pub(in crate::cranelift_backend::planning::static_transition) fn d2_units(plan: &StaticTransitionPlan<'_>) -> Vec<(AbiUnitDefinition, (u32, u32))> {
        plan.emittable_units()
            .expect("validated units")
            .into_iter()
            .map(|unit| {
                (
                    unit.definition(),
                    (unit.header().parameters, unit.header().captures),
                )
            })
            .collect()
    }

    /// **`D2` — a transparent closure-seed declaration owns its own callable
    /// unit, and an anonymous closure in the same program does not.**
    #[test]
    fn d2_a_transparent_closure_seed_declaration_owns_a_callable_unit() {
        let (root, declaration) = d2_declaration_and_anonymous_closure();
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::d2", &declaration);
        let plan = plan_static_transition_graph(&root, &declarations).expect("plannable");

        let declaration_origin = plan
            .declaration_occurrence_origin("decl:fixture::d2")
            .expect("the transparent declaration has an occurrence origin");
        let units = d2_units(&plan);

        // The declaration's callable unit: owned by the DECLARATION's occurrence,
        // carrying the closure's own arity. Asserted as an exact member rather
        // than "some unit is a CallableDeclaration", so a unit owned by the
        // wrong occurrence cannot satisfy it.
        assert!(
            units.contains(&(
                AbiUnitDefinition::CallableDeclaration {
                    declaration_origin,
                    provenance: AbiCaptureProvenance::Lexical,
                },
                (1, 2),
            )),
            "D2: the declaration's closure-seed body must be a callable unit \
             owned by the declaration, with the closure's own arity; got {units:?}"
        );

        // The discriminator: the anonymous closure stays an anonymous body. ⛔ Its
        // defining origin is NOT the declaration's, and that is the whole content
        // of "separately owned".
        let anonymous = units
            .iter()
            .filter_map(|(definition, arity)| match definition {
                AbiUnitDefinition::ClosureBody {
                    defining_origin, ..
                } => Some((*defining_origin, *arity)),
                _ => None,
            })
            .collect::<Vec<_>>();
        let [(anonymous_origin, anonymous_arity)] = anonymous[..] else {
            panic!("D2: expected exactly one anonymous ClosureBody unit, got {anonymous:?}");
        };
        assert_eq!(
            anonymous_arity,
            (0, 1),
            "D2: the anonymous closure must keep its own arity, distinct from \
             the declaration's -- reading the wrong unit's header would agree \
             with the declaration's (1, 2) instead"
        );
        assert_ne!(
            anonymous_origin, declaration_origin,
            "D2: the anonymous closure body must not be owned by the declaration"
        );
    }

    /// **`D2` causal control — the derivation, not the fixture, decides.**
    ///
    /// ⭐ Two mutations, and **each catches a defect the other cannot**. Ignoring
    /// ownership reds only the positive arm; claiming universal ownership reds
    /// only the discriminator. A single control here would leave one of the two
    /// wrong derivations green, and both wrong derivations still compile.
    #[test]
    fn d2_the_owner_split_is_causal_in_both_directions() {
        let (root, declaration) = d2_declaration_and_anonymous_closure();
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::d2", &declaration);

        let arms = |units: &[(AbiUnitDefinition, (u32, u32))]| {
            let callable = units
                .iter()
                .filter(|(definition, _)| {
                    matches!(definition, AbiUnitDefinition::CallableDeclaration { .. })
                })
                .count();
            let bodies = units
                .iter()
                .filter(|(definition, _)| {
                    matches!(definition, AbiUnitDefinition::ClosureBody { .. })
                })
                .count();
            (callable, bodies)
        };

        let plan = plan_static_transition_graph(&root, &declarations).expect("plannable");
        assert_eq!(
            arms(&d2_units(&plan)),
            (1, 1),
            "D2: the fixture must hold exactly one unit of each owner"
        );

        // Mutation 1 -- the pre-port derivation. The declaration loses its unit.
        let ignored = super::super::abi::D2_IGNORE_DECLARATION_OWNERSHIP.with(|flag| {
            flag.set(true);
            let plan = plan_static_transition_graph(&root, &declarations).expect("plannable");
            let observed = arms(&d2_units(&plan));
            flag.set(false);
            observed
        });
        assert_eq!(
            ignored,
            (0, 2),
            "D2: ignoring the owner discriminator must restore the pre-port \
             classification -- if this stays (1, 1) the split is not derived \
             from declaration ownership at all"
        );

        // Mutation 2 -- the opposite defect. The anonymous closure is captured
        // by the new arm, which the positive assertion alone accepts happily.
        let claimed = super::super::abi::D2_CLAIM_ALL_BODIES_DECLARATION_OWNED.with(|flag| {
            flag.set(true);
            let plan = plan_static_transition_graph(&root, &declarations).expect("plannable");
            let observed = arms(&d2_units(&plan));
            flag.set(false);
            observed
        });
        assert_eq!(
            claimed,
            (2, 0),
            "D2: claiming universal declaration ownership must swallow the \
             anonymous closure -- the discriminator is what rejects this"
        );
    }


    /// **`RT-DECL-CLOSURE-PORT` `D4` fixture — one program holding BOTH target
    /// classes, each actually referenced.**
    ///
    /// ⭐ A closure-seed declaration and a non-closure transparent declaration,
    /// with a `DeclarationRef` to each, because `D4`'s property is a
    /// **partition**. A fixture carrying only the closure seed cannot tell
    /// "retargeted by seed class" apart from "retargeted unconditionally" — and
    /// the unconditional reading is the hazard: a non-closure declaration's
    /// entry *is* its unit, so moving its call breaks every declaration call
    /// the corpus already makes.
    ///
    /// The two declarations carry different arities on purpose, so an assertion
    /// cannot be satisfied by reading the wrong unit's header and still agree.
    pub(in crate::cranelift_backend::planning::static_transition) fn d4_both_target_classes() -> (RuntimeExpr, RuntimeDeclaration, RuntimeDeclaration) {
        let closure_seed = RuntimeDeclaration {
            symbol: "decl:fixture::d4::callable".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::LexicalClosure {
                    captures: vec![RuntimeExpr::Var(0), RuntimeExpr::Var(1)],
                    params: vec!["arg0".to_string()],
                    body: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
                },
            },
            metadata: crate::RuntimeSymbolMetadata::empty(),
        };
        let thunk = RuntimeDeclaration {
            symbol: "decl:fixture::d4::thunk".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::Value(RuntimeValue::Int(73.into())),
            },
            metadata: crate::RuntimeSymbolMetadata::empty(),
        };
        // ⭐ A THIRD closure, anonymous and at the root, carrying an arity that
        // matches neither declaration. It is what makes "the static-body edge
        // leaving THIS declaration's entry" distinguishable from "some
        // static-body edge": with one closure in the program those two
        // derivations agree, and a reverse body search would pass unnoticed.
        let root = RuntimeExpr::Let {
            value: Box::new(RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: Vec::new(),
                    body: Box::new(RuntimeExpr::Value(RuntimeValue::Int(7.into()))),
                }),
                args: Vec::new(),
            }),
            body: Box::new(RuntimeExpr::Let {
                value: Box::new(RuntimeExpr::DeclarationRef {
                    symbol: "decl:fixture::d4::callable".to_string(),
                }),
                body: Box::new(RuntimeExpr::DeclarationRef {
                    symbol: "decl:fixture::d4::thunk".to_string(),
                }),
            }),
        };
        (root, closure_seed, thunk)
    }

    /// Every planned declaration call, as
    /// `(recorded class, callee definition, callee (parameters, captures))`.
    ///
    /// ⭐ Joined through the **resolved call edge**, not through the recorded
    /// class alone: the class is what the planner decided, and the descriptor is
    /// where the decision landed. Reading only the class would let a correct
    /// record sit above an edge that went somewhere else entirely.
    pub(in crate::cranelift_backend::planning::static_transition) fn d4_declaration_calls(
        plan: &StaticTransitionPlan<'_>,
    ) -> Vec<(
        DeclarationCallTargetClass,
        AbiUnitDefinition,
        (u32, u32),
    )> {
        let units = plan.emittable_units().expect("validated units");
        let mut calls = plan
            .emittable_call_edges()
            .expect("validated call edges")
            .into_iter()
            .filter(|edge| edge.kind() == EmittableCallKind::Declaration)
            .map(|edge| {
                let unit = units
                    .iter()
                    .find(|unit| unit.function() == edge.callee())
                    .expect("a declaration call edge names an emittable unit");
                (
                    plan.declaration_call_target_class(edge.call_site_origin())
                        .expect("a planned declaration call records its target class"),
                    unit.definition(),
                    (unit.header().parameters, unit.header().captures),
                )
            })
            .collect::<Vec<_>>();
        calls.sort_by_key(|(class, _, _)| *class);
        calls
    }

    /// **`D4` — a closure-seed declaration's call reaches its declaration-owned
    /// callable unit, and a non-closure declaration's call does NOT move.**
    ///
    /// Promise class: durable invariant. It is asserted as an equality over the
    /// whole declaration-call population, so a third class, a lost call, or a
    /// duplicated one all red — none of which a per-call `contains` would see.
    #[test]
    fn d4_the_declaration_call_partition_follows_the_seed_class() {
        let (root, closure_seed, thunk) = d4_both_target_classes();
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::d4::callable", &closure_seed);
        declarations.insert("decl:fixture::d4::thunk", &thunk);
        let plan = plan_static_transition_graph(&root, &declarations).expect("plannable");

        let callable_origin = plan
            .declaration_occurrence_origin("decl:fixture::d4::callable")
            .expect("the closure-seed declaration has an occurrence origin");

        assert_eq!(
            d4_declaration_calls(&plan),
            vec![
                (
                    DeclarationCallTargetClass::SchedulingEntry,
                    AbiUnitDefinition::SchedulingEntry {
                        ingress: abi::AbiSchedulingIngress::Empty,
                    },
                    (0, 0),
                ),
                (
                    DeclarationCallTargetClass::CallableDeclaration,
                    AbiUnitDefinition::CallableDeclaration {
                        declaration_origin: callable_origin,
                        provenance: AbiCaptureProvenance::Lexical,
                    },
                    (1, 2),
                ),
            ],
            "D4: the closure-seed declaration's call must reach the unit that \
             declares its one parameter and two captures, and the non-closure \
             declaration's call must still reach its own zero-input scheduling \
             entry"
        );
    }

    // ─── RT-DECL-CLOSURE-PORT D2a — the function-unit population ───
    //
    // ⭐⭐ **One source declaration contributes ONE function.** Before `D2a` a
    // closure-seed transparent declaration contributed two: its `StaticBody`
    // target (the `D2` callable unit) and its own zero-input `SchedulingEntry`
    // at the closure occurrence. The second has no lawful runtime meaning — it
    // cannot call the callable unit without the missing parameters and
    // captures, cannot return the closure, and cannot be a no-op without
    // changing program meaning.

    /// Every class in the ruled partition, in one program.
    ///
    /// ⚠ All four are present **and distinguishable**: the two declaration
    /// closure forms differ in arity, and the anonymous closure differs from
    /// both. A fixture carrying one closure cannot tell "the relation leaving
    /// THIS declaration" from "some relation".
    #[cfg(test)]
    pub(in crate::cranelift_backend::planning::static_transition) fn d2a_every_partition_class() -> (RuntimeExpr, Vec<RuntimeDeclaration>) {
        let lexical = RuntimeDeclaration {
            symbol: "decl:fixture::d2a::lexical".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::LexicalClosure {
                    captures: vec![RuntimeExpr::Value(RuntimeValue::Int(1.into()))],
                    params: vec!["a".to_string()],
                    body: Box::new(RuntimeExpr::Var(0)),
                },
            },
            metadata: crate::RuntimeSymbolMetadata::empty(),
        };
        let seed = RuntimeDeclaration {
            symbol: "decl:fixture::d2a::seed".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::Closure {
                    captures: Vec::new(),
                    params: vec!["p".to_string(), "q".to_string()],
                    body: Box::new(RuntimeExpr::Var(0)),
                },
            },
            metadata: crate::RuntimeSymbolMetadata::empty(),
        };
        let thunk = RuntimeDeclaration {
            symbol: "decl:fixture::d2a::thunk".to_string(),
            kind: RuntimeDeclarationKind::Transparent {
                body: RuntimeExpr::Value(RuntimeValue::Int(73.into())),
            },
            metadata: crate::RuntimeSymbolMetadata::empty(),
        };
        // The root, carrying an ANONYMOUS closure that must keep its own
        // `ClosureBody` unit and its own emitted `StaticBody` call.
        let root = RuntimeExpr::Let {
            value: Box::new(RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: Vec::new(),
                    body: Box::new(RuntimeExpr::Value(RuntimeValue::Int(7.into()))),
                }),
                args: Vec::new(),
            }),
            body: Box::new(RuntimeExpr::Let {
                value: Box::new(RuntimeExpr::DeclarationRef {
                    symbol: "decl:fixture::d2a::lexical".to_string(),
                }),
                body: Box::new(RuntimeExpr::Let {
                    value: Box::new(RuntimeExpr::DeclarationRef {
                        symbol: "decl:fixture::d2a::seed".to_string(),
                    }),
                    body: Box::new(RuntimeExpr::DeclarationRef {
                        symbol: "decl:fixture::d2a::thunk".to_string(),
                    }),
                }),
            }),
        };
        (root, vec![lexical, seed, thunk])
    }

    /// The unit population as a sorted class census, plus the count of emitted
    /// `StaticBody` **calls**.
    #[cfg(test)]
    pub(in crate::cranelift_backend::planning::static_transition) fn d2a_population(plan: &StaticTransitionPlan<'_>) -> (Vec<&'static str>, usize) {
        let mut classes = plan
            .emittable_units()
            .expect("validated units")
            .into_iter()
            .map(|unit| match unit.definition() {
                AbiUnitDefinition::SchedulingEntry { .. } => "SchedulingEntry",
                AbiUnitDefinition::CallableDeclaration { .. } => "CallableDeclaration",
                AbiUnitDefinition::ClosureBody { .. } => "ClosureBody",
                AbiUnitDefinition::ContinuationSpecialization { .. } => {
                    "ContinuationSpecialization"
                }
                // `D2f`: named rather than filtered, so the census stays a
                // TOTAL classification of the planned population. Absorbing the
                // class into another label is how a new unit class becomes
                // invisible to the very control that measures the population.
                AbiUnitDefinition::StaticContinuationFusion { .. } => {
                    "StaticContinuationFusion"
                }
            })
            .collect::<Vec<_>>();
        classes.sort_unstable();
        let static_body_calls = plan
            .emittable_call_edges()
            .expect("validated call edges")
            .into_iter()
            .filter(|edge| edge.kind() == EmittableCallKind::StaticBody)
            .count();
        (classes, static_body_calls)
    }

    /// **`D2a` — the closed partition, stated as a population.**
    #[test]
    fn d2a_one_source_declaration_contributes_exactly_one_function() {
        let (root, declarations) = d2a_every_partition_class();
        let declarations = declarations
            .iter()
            .map(|declaration| (declaration.symbol.as_str(), declaration))
            .collect::<BTreeMap<_, _>>();
        let plan = plan_static_transition_graph_with_symbols(
            &root,
            &declarations,
            &crate::NativeProcessSymbols::legacy_prelude(),
            abi::AbiRootIngress::Value,
            true,
        )
        .expect("the D2a fixture plans");
        let (classes, static_body_calls) = d2a_population(&plan);
        assert_eq!(
            classes,
            vec![
                // the two closure declarations
                "CallableDeclaration",
                "CallableDeclaration",
                // the anonymous closure at the root
                "ClosureBody",
                // the root, and the non-closure thunk declaration
                "SchedulingEntry",
                "SchedulingEntry",
            ],
            "D2a: root + thunk are the ONLY scheduling entries; each closure \
             declaration contributes exactly one callable unit and no separate \
             scheduling entry; the anonymous closure keeps its ClosureBody"
        );
        assert_eq!(
            static_body_calls, 1,
            "D2a: only the ANONYMOUS closure's static-body relation is an \
             emitted call. A declaration-owned pair's relation is a \
             definition/signature relation inside one unit, and emitting it as \
             a call would reintroduce the phantom from the other side"
        );
        // Cross-plane one-for-one: the semantic partition, the ABI descriptors,
        // and the declared function population must state the same result.
        // ⛔ A repair that only skipped `emittable_units` would leave a phantom
        // owner here, which is one of the four explicitly rejected half-measures.
        assert_eq!(
            plan.semantic.functions.len(),
            classes.len(),
            "D2a: the semantic function population must equal the ABI \
             descriptor population exactly"
        );
    }

    /// **`D2a` — the substitution is CAUSAL.**
    #[test]
    fn d2a_retaining_the_obsolete_scheduling_unit_restores_the_phantom() {
        let (root, declarations) = d2a_every_partition_class();
        let declarations = declarations
            .iter()
            .map(|declaration| (declaration.symbol.as_str(), declaration))
            .collect::<BTreeMap<_, _>>();
        let retained = with_d2a_population_mutation(
            D2aPopulationMutation::RetainObsoleteSchedulingUnit,
            || {
                plan_static_transition_graph_with_symbols(
                    &root,
                    &declarations,
                    &crate::NativeProcessSymbols::legacy_prelude(),
                    abi::AbiRootIngress::Value,
                    true,
                )
                .map(|plan| d2a_population(&plan).0)
            },
        );
        let retained = retained.expect("the pre-D2a population still plans");
        assert_eq!(
            retained.iter().filter(|class| **class == "SchedulingEntry").count(),
            4,
            "D2a: with the substitution suppressed, BOTH closure declarations \
             get their obsolete zero-input scheduling entry back — 4 entries \
             where the ruled partition has 2. If this count did not move, the \
             partition assertion above is not caused by D2a: {retained:?}"
        );
    }

    /// **`D4` — the partition is CAUSAL in both directions.**
    ///
    /// ⭐ Two mutations, because one cannot defeat a split. Without
    /// `NeverRetarget` the positive assertion above is consistent with the
    /// retarget never having been installed on a plan that happened to agree;
    /// without `AlwaysRetarget` it is consistent with a blanket retarget that
    /// drags the thunk along and is only accidentally right about the closure
    /// seed.
    #[test]
    fn d4_the_declaration_call_partition_is_causal_in_both_directions() {
        let (root, closure_seed, thunk) = d4_both_target_classes();
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::d4::callable", &closure_seed);
        declarations.insert("decl:fixture::d4::thunk", &thunk);

        let under = |mutation: D4DeclarationTargetMutation| {
            D4_DECLARATION_TARGET_MUTATION.with(|cell| {
                cell.set(mutation);
                let outcome = plan_static_transition_graph(&root, &declarations)
                    .map(|plan| d4_declaration_calls(&plan));
                cell.set(D4DeclarationTargetMutation::Exact);
                outcome
            })
        };

        // Direction 1 -- the pre-`D4` world, and `D2a` has made it STRICTLY
        // unreachable rather than merely wrong.
        //
        // ⭐ Before `D2a` this mutation planned: both calls landed on a
        // zero-input scheduling entry, so the closure-seed reference called a
        // thunk declaring none of its parameters or captures. That wrong-arity
        // target was the thing `D4` removed. `D2a` removes the target itself —
        // a closure-seed declaration's scheduling entry is no longer a function
        // unit — so the same mutation now cannot be planned at all.
        //
        // ⚠ The assertion was updated because the mechanism got stronger, not
        // because the control was re-fit to whatever the code now does: the
        // direction being measured is unchanged (suppress the retarget, prove
        // the positive partition was caused by the seed discriminator), and it
        // is now measured by a refusal instead of by an arity.
        let never = under(D4DeclarationTargetMutation::NeverRetarget);
        let Err(CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(reason))) = never
        else {
            panic!(
                "D4/D2a: with the retarget suppressed the closure-seed call \
                 targets a scheduling entry that D2a no longer seeds as a unit, \
                 so planning must refuse it: {never:?}"
            );
        };
        assert!(
            reason.contains("declaration call edge target is not its function unit's seed"),
            "D4/D2a: the refusal must name the missing unit seed. Any other \
             invariant would leave the suppressed retarget unpinned: {reason}"
        );

        // Direction 2 -- the ruled-out reverse body search. It lands on the
        // anonymous closure's body, which is a `ClosureBody` unit owned by
        // nobody's declaration. ⛔ It must not merely produce a different
        // partition: a declaration call to an anonymous closure body is not a
        // lawful target at all, so planning refuses it.
        let any = under(D4DeclarationTargetMutation::AnyStaticBody);
        let Err(CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(reason))) = any
        else {
            panic!(
                "D4: a declaration call retargeted by reverse body search reaches \
                 an anonymous closure body, which must be refused rather than \
                 planned: {any:?}"
            );
        };
        assert!(
            reason.contains("neither a scheduling entry nor a callable declaration unit"),
            "D4: the refusal must name the target CLASS -- a refusal for some \
             other invariant would leave the wrong-owner target unpinned: {reason}"
        );
    }
}
