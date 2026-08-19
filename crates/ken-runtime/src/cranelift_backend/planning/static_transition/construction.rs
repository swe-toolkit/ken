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
#[cfg(test)]
#[allow(unused_imports)]
use super::aggregates::{
    aggregate_child_referent_owners, fixed_node_selected_owner, flatten_allocation_reachable_uses,
    host_effect_recipe_tree, node_referent_owners, validate_aggregate_producers_are_unique,
    SynthesizedAggregateStep,
};
#[allow(unused_imports)]
use super::aggregates::{
    build_aggregate_ownership_plan, lifetime_referent_affinity, validate_aggregate_ownership_plan,
    AggregateOccurrenceId, AggregateOccurrenceProducer, PlannedAggregateAllocation,
    PlannedAggregateOwnership, PlannedAggregateShape, SynthesizedAggregateNode,
    SynthesizedAggregatePath, SynthesizedAggregateRole, SynthesizedAggregateRoot,
    SynthesizedDynamicSet,
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
    build_semantic_plane, build_synthesized_constructor_inventory, SemanticMaterialArena,
    SemanticPlane, SemanticSourceSeed,
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
                host_effect_seats: Vec::new(),
                occurrence_authorities: Vec::new(),
                continuation_specializations: Vec::new(),
                continuation_specialization_calls: Vec::new(),
                required_consumer_projections: BTreeMap::new(),
                continuation_contexts: Vec::new(),
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
        // `D5a`: the generated contexts' own ABI, in its own arenas. ⛔ Installed
        // AFTER the specialization ABI and into separate vectors, never appended
        // to `continuation_descriptors` -- that population is exactly the
        // continuation-callee partition, and admitting a caller-side context
        // there would make one identity domain readable as the other.
        install_continuation_context_abi(&mut self.plan.abi, &self.plan.continuation_contexts)?;
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
        // ⛔ After `join_results` for the same reason the ownership plan is: a
        // seat's consumer phase is a fact about the child's planned result
        // representation, which does not exist until that line.
        self.plan.host_effect_seats = build_host_effect_seat_plan(&self.plan)?;
        validate_host_effect_seat_plan(&self.plan, &self.plan.host_effect_seats)?;
        #[cfg(test)]
        apply_static_worker_member_mutation(&mut self.plan);
        self.plan.validate()?;
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
