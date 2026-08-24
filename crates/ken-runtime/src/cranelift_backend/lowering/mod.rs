//! Lowering state, the acyclic support methods, and the lowered-value,
//! continuation/control, source-machine, bounded-Nat and dynamic-constructor
//! data model, plus their free helpers (RT-SPLIT §10.1/§10.2).
//!
//! The indivisible 29-method lowering SCC lives in the child module `core`,
//! which consumes this module's private items as its ancestor and therefore
//! needs no widening (§10.4, "hierarchy is load-bearing").
//!
//! Imports below name their **owning** module, crate root, or external
//! dependency directly. This module must never import through the facade:
//! §10.3 forbids an implementation module doing so, and an omnibus
//! `use super::*` would hide the real `compiled` / `planning` / `surface`
//! edges behind a namespace. The `pub(in crate::cranelift_backend)` on each
//! is namespace wiring, not a widening — it re-exports names at their existing
//! visibility so `core` and its subject tests inherit them, and it cannot
//! escape `crate::cranelift_backend`.

pub(in crate::cranelift_backend) mod core;

// `RT-FNSPLIT-B2F` `D1`/`D2` — the target code-unit population. A sibling of
// `core` rather than a region inside it: `core.rs` is the module whose recursive
// whole-configuration authority `D6` removes, and putting the replacement
// population in the same file would make the census that measures the removal
// unable to distinguish the two.
pub(in crate::cranelift_backend) mod units;

// `RT-FNSPLIT-B2F` `D3` — the artifact-static seed material. A sibling of
// `units` rather than a region inside it, because the two mint **different
// populations**: `units` mints code (`Theta(n)` in the program) and this mints
// data (`Theta(|seed environment|)`, independent of the program). ⛔ Folding
// them into one file would put two growth axes behind one census row, which is
// the population conflation this node has already had to repair three times.
pub(in crate::cranelift_backend) mod seed_material;

// `RT-LOWERING-VALUES-BOUNDARY-SPLIT` `D1` — the values-boundary domain's
// facade-portable disposition/classification/lifecycle-phase vocabulary. A
// sibling of `core`/`units`/`seed_material` rather than a region inside any
// of them, since `Lowered`/`LoweringOperand` and the carrier-emission
// machinery that consumes this vocabulary stay here, SCC-pinned.
pub(in crate::cranelift_backend) mod boundary;

// This module's own retained carrier-emission code references
// `BoundaryDisposition` and `LoweredVariant` directly (deciding and naming a
// value's representation), and its `D2k` test tracer holds a
// `BoundaryTransferInvokingSite`. The rest of the vocabulary is `core`'s and
// the `tests` subtree's to import from `boundary` directly (see `core.rs`),
// matching how `units.rs` imports `AmbientBodyAuthority`/
// `CheckedFrameFunctionScope` from `core` rather than through a blanket
// re-export here.
pub(in crate::cranelift_backend) use boundary::{BoundaryDisposition, LoweredVariant};
#[cfg(test)]
pub(in crate::cranelift_backend) use boundary::BoundaryTransferInvokingSite;

// `RT-SOURCE-MACHINE-TYPES-SPLIT` `D1` — the source machine's own state types
// and dispatch control. A sibling of `core`/`units`/`seed_material`/`boundary`
// rather than a region inside any of them; the types the moving methods
// merely manipulate (`SourcePrefixTemplate`, `SourceControl`, and siblings)
// stay declared here, shared with retained checked-invocation/continuation-
// frame machinery — matching how `boundary.rs` left `Lowered`/`LoweringOperand`
// at this hub and moved only their facade-qualified methods.
pub(in crate::cranelift_backend) mod source;

// This module's own retained `SourceControl` (the checked-invocation lineage
// carrier) holds the moved `SourceContinuation` by value, so the hub needs it
// in scope in every build.
use source::SourceContinuation;
// `SourceContinuationTerminal` has no production hub consumer -- only test
// code constructs it directly -- but this `use` (like the `boundary` one
// above) is what the `core`/`tests` glob chain re-exports downward; the
// alternative is a scattered per-test-file import for every direct
// construction site, which the boundary.rs precedent avoids.
#[cfg(test)]
use source::{SourceCarriedControlMutation, SourceContinuationTerminal, with_source_carried_control_mutation};

// `RT-EMITTER-CALLS-RETURNS-SPLIT` `D1` — the calls and returns emitter:
// declared-call emission, residual and recursor call lowering, return
// emission, and the callee-side checks. A sibling of
// `core`/`units`/`seed_material`/`boundary`/`source` rather than a region
// inside any of them, matching how the earlier emitter/vocabulary slices
// left their hub types here and moved only the mutating methods.
pub(in crate::cranelift_backend) mod calls;

// The test-glob chain (`core.rs`'s `use super::*`, then `core/tests/mod.rs`'s
// own `use super::*`) re-exports these downward to `core/tests/control.rs`,
// which references each by bare name — the same mechanism the `source`
// imports above already rely on.
#[cfg(test)]
use calls::{
    d5_emitted_declaration_calls, reset_d5_emitted_declaration_calls,
    set_trap_caller_protocol_mutation, with_d5_closeout_mutation, D5CloseoutMutation,
    TrapCallerProtocolMutation,
};

// `RT-EMITTER-CONTROL-JOINS-SPLIT` `D1` — the control and joins emitter:
// branch/match emission, join emission, and block/terminator construction
// (trap exits). A sibling of `core`/`units`/`seed_material`/`boundary`/
// `source`/`calls` rather than a region inside any of them, matching how the
// earlier emitter slices left their hub types here and moved only the
// mutating methods. `ScalarMergeKind`, `merge_planned_scalar_branch`, and
// `lowered_from_scalar_pair` stay at this hub (Addendum 7): a field of the
// retained `SourceJoinTarget` and two retained-caller delegating helpers,
// not exclusive to this slice.
pub(in crate::cranelift_backend) mod joins;

// The not-yet-moved tests in `core/tests/control.rs` construct
// `TrapIdentityMutation` and call `set_trap_identity_mutation` directly; the
// test-glob chain (`core.rs`'s `use super::*`, then `core/tests/mod.rs`'s
// own `use super::*`) re-exports these downward, the same mechanism the
// `source`/`calls` imports above already rely on.
#[cfg(test)]
use joins::{set_trap_identity_mutation, TrapIdentityMutation};

// `RT-EMITTER-AGGREGATES-SPLIT` `D1` — the aggregates emitter: aggregate
// construction and projection emission, allocation emission, and the
// governed-allocation surfaces. A sibling of
// `core`/`units`/`seed_material`/`boundary`/`source`/`calls`/`joins` rather
// than a region inside any of them. `AggregateAllocationLedger`/
// `AggregateAllocationEvent`/`AggregateRelationClosure` were already
// `pub(in crate::cranelift_backend)` before the move (the Architect's D0
// carry) and move verbatim, zero widening — the retained `Lowering` hub
// struct's own `aggregate_allocations` field is updated to the qualified
// `aggregates::AggregateAllocationLedger`, the same pattern `units.rs`'s
// own moved-ledger types already use as `Lowering` fields.
pub(in crate::cranelift_backend) mod aggregates;

// `RT-EMITTER-EFFECTS-SPLIT` `D1` — the effects emitter: effect-seat
// emission, host-call emission, and the effect-side operand construction.
// A sibling of `core`/`units`/`seed_material`/`boundary`/`source`/
// `calls`/`joins`/`aggregates` rather than a region inside any of them.
// `EffectSeatLedger`/`EffectSeatClosure` were already `pub(in
// crate::cranelift_backend)` before the move and move verbatim, zero
// widening — the retained `Lowering` hub struct's own `host_effect_seats`
// field is updated to the qualified `effects::EffectSeatLedger`, the same
// pattern `aggregates.rs`'s own `aggregate_allocations` field already
// uses. `ClaimedEffectSeats<'a>` and `SiteOperandWitness`/
// `site_operand_witness` both stay at this hub (Architect's D0 vote,
// `evt_7nzxad9y75crk`) — zero-widening RETAIN, not movers.
pub(in crate::cranelift_backend) mod effects;

// `core.rs`'s retained `transfer_constructor_operands` constructs
// `GovernedAllocationSite::CarriedConstructor` directly — a production
// (not test-gated) reference, unlike every other name this item moved.
use aggregates::GovernedAllocationSite;

// `core.rs`'s retained `lower_process_host_effect` constructs
// `SynthesizedArgument::{Scalar,Nested,Dynamic}` directly — despite its own
// doc comment calling it "private to synthesized construction," its sole
// production caller sits outside the moved cluster.
use aggregates::SynthesizedArgument;

// `units.rs`'s retained ledger-lifecycle wrappers reference these two by
// bare name via the same glob-inheritance mechanism as the `boundary`/
// `source` imports above.
use aggregates::{AggregateAllocationLedger, AggregateRelationClosure};

// `units.rs`'s retained `open_continuation_claim_ledger`/`close_host_
// effect_seat_ledger`/`last_effect_seat_closure` reference these two by
// bare name via the same glob-inheritance mechanism as above.
use effects::{EffectSeatClosure, EffectSeatLedger};

// `core/tests/constructors.rs`'s residual `d7_ownership_run` (the one
// multi-leaf fixture that stays there, `RT-EMITTER-AGGREGATES-SPLIT` `D2`)
// names `GovernedAllocationMutation` in its own signature and body; the same
// test-glob-chain mechanism as `joins`'s own `#[cfg(test)]` re-export above.
// `CarrierAllocationRequest` no longer needs re-exporting here: its only
// test consumer moved into `aggregates::tests` itself at `D2`.
// `GovernedAllocationMutationGuard`/`SiblingProducerSubstitution` -- narrowed
// away at `RT-BACKEND-SPLIT-CLOSURE` (item 18): their own only-ever consumer
// (a test formerly in `constructors.rs`) relocated into `aggregates::tests`
// itself at item 15's own `D2`, where both names are already in scope
// without any re-export; the compiler confirmed zero remaining consumers
// crate-wide before this narrowing (`unused import` warning, independently
// re-verified by direct grep, not applied on the warning alone).
#[cfg(test)]
use aggregates::GovernedAllocationMutation;

// `calls.rs`'s retained `call_declared_unit_target` reads
// `SELF_AUTHORIZED_FALLBACK_REACHES` directly, under `#[cfg(test)]`, as part
// of the self-authorizing-aggregate control's own measurement.
#[cfg(test)]
use aggregates::SELF_AUTHORIZED_FALLBACK_REACHES;

// --- external dependencies -------------------------------------------------
pub(in crate::cranelift_backend) use std::collections::{BTreeMap, BTreeSet};

// `RT-FNSPLIT-B2V` `D4`. Re-exported at facade scope like every other import in
// this header so the `tests` subtree inherits the names.
pub(in crate::cranelift_backend) use crate::boundary_value::{
    BoundaryClass, BoundaryReferentOwner, BoundaryTag, BOUNDARY_ERR_BOUNDS, BOUNDARY_OK,
};

pub(in crate::cranelift_backend) use cranelift_codegen::flowgraph::ControlFlowGraph;
pub(in crate::cranelift_backend) use cranelift_codegen::ir::{
    types, AbiParam, Block, FuncRef, Function, InstBuilder, MemFlags, StackSlotData, StackSlotKind,
    UserFuncName,
};
pub(in crate::cranelift_backend) use cranelift_codegen::verify_function;
pub(in crate::cranelift_backend) use cranelift_frontend::{
    FunctionBuilder, FunctionBuilderContext,
};
pub(in crate::cranelift_backend) use cranelift_module::{FuncId, Linkage, Module};

pub(in crate::cranelift_backend) use safe_byte_span::SafeByteSpan;

// --- crate root ------------------------------------------------------------
pub(in crate::cranelift_backend) use crate::{
    RuntimeDeclaration, RuntimeDeclarationKind, RuntimeExpr, RuntimeGroundValue, RuntimePartiality,
    RuntimePrimitive, RuntimeSymbol, RuntimeTrap, RuntimeTrapCode, RuntimeValue,
};

// --- sibling backend modules, named at their OWNERS -----------------------
// §10.3 DAG: `lowering support -> compiled, planning, surface`.
//
// These are SEMANTIC support edges, not namespace wiring for `core`:
// `Lowering::emit_result` returns and constructs `compiled::ResultDecoder`,
// and `source_case_has_no_checked_control_markers` calls the `planning`
// collectors and constructs `CheckedOrientedMarkerSets`. Acyclic, because
// `compiled` and `planning` each depend only on `surface` and neither imports
// lowering. No `artifact` / `artifact::api` edge, and no reverse edge.
// (Architect `evt_8vhe6rd6r80c`; the landed §10.3 line said support -> surface
// only, which these four imports and two production bodies refute.)
pub(in crate::cranelift_backend) use super::compiled::{CompiledModule, ResultDecoder};
#[cfg(any(test, feature = "r3-4b-observation"))]
pub(in crate::cranelift_backend) use super::planning::{
    StaticContinuationFusionDescriptor, StaticContinuationFusionKey, StaticContinuationFusionPlan,
};
pub(in crate::cranelift_backend) use super::planning::{
    collect_checked_oriented_markers, collect_checked_subcontinuation_frames,
    build_static_continuation_fusion_plan, plan_static_transition_graph_with_symbols,
    FusionCompositionLayer, FusionRegionClaim, FusionRegionClaimLedger,
    StaticContinuationFusionId, StaticContinuationFusionView,
    validate_oriented_subcontinuation_transport,
    AbiCaptureProvenance, AbiCarrier, AbiFrameHeader, AbiOwnership, AbiProcessParameter,
    AbiRootIngress, AbiSlot, AbiSlotKind, AbiStorageOwner, AbiUnitDefinition,
    expected_capture_slot,
    // `RT-LEXICAL-RECURSOR-CONSUMERS` `D2e` — the checked binder layout, now
    // reaching PRODUCTION rather than only lowering's test targets: the composed
    // eliminator checks its assembled run against it. ⛔ Ungated here and in
    // `planning.rs`, because a `cfg(test)` re-export of an item production reads
    // is an unresolved import the test profile cannot see.
    CheckedCaseBinderLayout, CheckedCaseBinderRole,
    CheckedOrientedMarkerSets, ConstructorIdentity, ContinuationCallIdentity, ContinuationCallView,
    DeclarationCallTargetClass,
    ContinuationContextId, ContinuationEmissionOwner,
    ContinuationInputView, ContinuationOrdinaryEnvelopeRole, ContinuationResultEdge,
    ContinuationWorkerCaptureSource,
    ContinuationAvailabilityViews, ContinuationEnvironmentClaim, ContinuationFrameIdentity,
    ContinuationSourceCoordinate,
    ContinuationSourceSlotAuthority,
    ContinuationSpecializationId,
    ContinuationUnitView, RequiredConsumerProjection, EmittableCallKind,
    FieldIdentity, JoinPlanToken,
    CaseEmissionStatus, PlannedReferentLifetime,
    host_effect_seat_contract_of, EffectSeatNeed, EffectSeatOperation, EffectSeatPhase,
    EffectSeatSlot, PlannedEffectSeat,
    AggregateOccurrenceId, PlannedAggregateAllocation, PlannedAggregateShape,
    SynthesizedAggregateNode, SynthesizedAggregatePath, SynthesizedAggregateRoot, PlannedAggregateOwnership,
    dead_arm_effect_trap,
    JoinResultRepresentation, PredeclaredFunctionId, StaticOriginId, StaticTransitionPlan,
    verify_current_lexical_availability, verify_predeclared_entry_frame_membership,
    SynthesizedConstructorRole, SynthesizedFixedConstructorRole,
};
#[cfg(test)]
pub(in crate::cranelift_backend) use super::planning::{
    plan_static_transition_graph, with_last_io_error_role_omitted, ScaleBPlanCensus,
};
pub(in crate::cranelift_backend) use super::surface::{
    backend, backend_module, unsupported, BackendFailure, CraneliftBackendError,
    NativeSeedEnvironment,
};

// `#[cfg(test)]`-only: an unconditional `use` of this breaks the non-test
// build, which the test build cannot show you.
#[cfg(test)]
pub(in crate::cranelift_backend) use crate::RuntimeMatchCase;

/// One completed FunctionizedUnits emission row for RT-SCALE-B.
///
/// The fixed native-Int and boundary-value graphs, every unit body, and the
/// public root adapter all record through the same function seam.  Imports and
/// test-only probes never call that seam.
#[cfg(test)]
#[derive(Clone, Debug)]
pub(in crate::cranelift_backend) struct ScaleBEmissionMetrics {
    pub(in crate::cranelift_backend) plan: ScaleBPlanCensus,
    pub(in crate::cranelift_backend) authority_functionized: bool,
    pub(in crate::cranelift_backend) emitted_helpers: usize,
    pub(in crate::cranelift_backend) production_functions: usize,
    pub(in crate::cranelift_backend) native_int_functions: usize,
    pub(in crate::cranelift_backend) boundary_value_functions: usize,
    pub(in crate::cranelift_backend) functionized_root_adapters: usize,
    pub(in crate::cranelift_backend) functionized_unit_bodies: usize,
    pub(in crate::cranelift_backend) clif_instructions: usize,
    pub(in crate::cranelift_backend) clif_bytes: usize,
    pub(in crate::cranelift_backend) total_dfg_values: usize,
    pub(in crate::cranelift_backend) total_instructions: usize,
    pub(in crate::cranelift_backend) total_blocks: usize,
}

#[cfg(test)]
#[derive(Clone, Debug)]
struct ScaleBEmissionAttempt {
    metrics: ScaleBEmissionMetrics,
    complete: bool,
}

#[cfg(test)]
thread_local! {
    static SCALE_B_EMISSION_ATTEMPT:
        std::cell::RefCell<Option<ScaleBEmissionAttempt>> =
            const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
fn scale_b_reset_emission_attempt() {
    SCALE_B_EMISSION_ATTEMPT.with(|attempt| *attempt.borrow_mut() = None);
}

#[cfg(test)]
fn scale_b_begin_emission_attempt(plan: &StaticTransitionPlan<'_>, authority_functionized: bool) {
    SCALE_B_EMISSION_ATTEMPT.with(|attempt| {
        *attempt.borrow_mut() = Some(ScaleBEmissionAttempt {
            metrics: ScaleBEmissionMetrics {
                plan: plan.scale_b_census(),
                authority_functionized,
                emitted_helpers: 0,
                production_functions: 0,
                native_int_functions: 0,
                boundary_value_functions: 0,
                functionized_root_adapters: 0,
                functionized_unit_bodies: 0,
                clif_instructions: 0,
                clif_bytes: 0,
                total_dfg_values: 0,
                total_instructions: 0,
                total_blocks: 0,
            },
            complete: false,
        });
    });
}

#[cfg(test)]
fn scale_b_finish_emission_attempt() {
    SCALE_B_EMISSION_ATTEMPT.with(|attempt| {
        if let Some(attempt) = attempt.borrow_mut().as_mut() {
            attempt.complete = true;
        }
    });
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn scale_b_last_emission_metrics() -> Option<ScaleBEmissionMetrics>
{
    SCALE_B_EMISSION_ATTEMPT.with(|attempt| {
        attempt
            .borrow()
            .as_ref()
            .filter(|attempt| attempt.complete)
            .map(|attempt| attempt.metrics.clone())
    })
}

#[cfg(test)]
#[derive(Clone, Copy)]
enum ScaleBEmitter {
    NativeInt,
    BoundaryValue,
    FunctionizedRootAdapter,
    FunctionizedUnitBody,
}

#[cfg(test)]
fn scale_b_record_function(function: &Function, emitter: ScaleBEmitter) {
    SCALE_B_EMISSION_ATTEMPT.with(|attempt| {
        let mut attempt = attempt.borrow_mut();
        let Some(attempt) = attempt.as_mut() else {
            return;
        };
        if attempt.complete {
            return;
        }
        let instructions = function.dfg.num_insts();
        attempt.metrics.production_functions += 1;
        attempt.metrics.clif_instructions += instructions;
        attempt.metrics.clif_bytes += function.display().to_string().len();
        attempt.metrics.total_dfg_values += function.dfg.num_values();
        attempt.metrics.total_instructions += instructions;
        attempt.metrics.total_blocks += function.dfg.num_blocks();
        match emitter {
            ScaleBEmitter::NativeInt => attempt.metrics.native_int_functions += 1,
            ScaleBEmitter::BoundaryValue => attempt.metrics.boundary_value_functions += 1,
            ScaleBEmitter::FunctionizedRootAdapter => {
                attempt.metrics.functionized_root_adapters += 1;
            }
            ScaleBEmitter::FunctionizedUnitBody => {
                attempt.metrics.functionized_unit_bodies += 1;
                attempt.metrics.emitted_helpers += 1;
            }
        }
    });
}

#[cfg(test)]
pub(crate) fn scale_b_record_native_int(function: &Function) {
    scale_b_record_function(function, ScaleBEmitter::NativeInt);
}

#[cfg(test)]
pub(crate) fn scale_b_record_boundary_value(function: &Function) {
    scale_b_record_function(function, ScaleBEmitter::BoundaryValue);
}

#[cfg(test)]
fn scale_b_record_functionized_root_adapter(function: &Function) {
    scale_b_record_function(function, ScaleBEmitter::FunctionizedRootAdapter);
}

#[cfg(test)]
fn scale_b_record_unit_body(function: &Function) {
    scale_b_record_function(function, ScaleBEmitter::FunctionizedUnitBody);
}

// ⭐ The admitted set now has ONE definition, in planning, because the seat
// population is derived from it there. This is a namespace re-export so both
// the emitter's admission check and the planner's population read the same
// list; a local copy could disagree with it silently.
use crate::cranelift_backend::planning::CRANELIFT_HOST_EFFECT_CONSUMERS_V1;









#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BoundedNatLoweringMutation {
    Exact,
    BrokenDecrement,
    RawScalarPredecessor,
}
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Px8jProducerPath {
    Composed,
    DeferredConstructor,
    SourceMachine,
}
/// ⭐ **The phase of an induction hypothesis's residual, with the carried word
/// when there is one** (`RT-FNSPLIT-C1` `AC-C4`, `§2g-i`).
///
/// ⚠ **The raw `ir::Value` is recorded rather than a `CarriedBoundaryWord`, and
/// that is deliberate.** Recording the struct would require giving it
/// `PartialEq` — which would hand *production* a compile-time way to ask
/// whether two carried values are the same word. ⛔ That is exactly the
/// capability `CarriedBoundaryWord`'s emptiness exists to deny (see its doc
/// comment: every question about a carried value is answered by an emitted
/// helper at runtime). ⇒ The observation stays test-only in its **capability**,
/// not merely in its `#[cfg]`.
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Px8jResidualPhase {
    /// The carried phase, together with the exact boundary word held.
    Carried(cranelift_codegen::ir::Value),
    /// The specialized phase. ⛔ Recorded as a phase only: `§2g-i`'s clause
    /// constrains the **carried** arm, and a specialized residual is a
    /// different route entirely.
    Specialized,
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq)]
enum Px8jSourceTraceEvent {
    Mint {
        path: Px8jProducerPath,
        origin: RecursorProducerOriginId,
        cursor: ContinuationCursorId,
        siblings: usize,
        parent_scope: Option<RecursorProducerOriginId>,
    },
    Carrier {
        path: Px8jProducerPath,
        origin: RecursorProducerOriginId,
        cursor: ContinuationCursorId,
        sibling_position: usize,
        /// ⭐⭐ **The RESIDUAL edge — `§2g-i`'s actual sentence.**
        ///
        /// ⛔ Every other field of this event observes the **metadata** edge:
        /// `origin`, `cursor` and `sibling_position` all say *who owns* the
        /// hypothesis. ⛔ **None of them says what is INSIDE it.** Substituting
        /// one projected child for another leaves all three byte-identical —
        /// which is precisely the compile-preserving evasion that defeated this
        /// control (`children[position]` → `children[0]`, `runtime-qa` on
        /// `b8d2922f`).
        residual: Px8jResidualPhase,
    },
    /// ⭐ **A child projected out of a carried scrutinee, as
    /// `(position, word)`.**
    ///
    /// ⭐⭐ **This is the INDEPENDENT ORACLE for the residual edge, and the
    /// independence is structural, not a matter of care.** It is written by the
    /// projection loop itself, keyed on **that loop's own counter** — so it
    /// records which field each word actually came from, and it is written
    /// *before* any selection among the children happens. ⇒ A test may name a
    /// position on its **fixture's** authority and ask this record which word
    /// that field produced, without ever reading the index the production path
    /// selected with.
    CarrierFieldProjection {
        path: Px8jProducerPath,
        position: usize,
        word: cranelift_codegen::ir::Value,
    },
    Install {
        origin: RecursorProducerOriginId,
        selection_cursor: ContinuationCursorId,
        sibling_position: usize,
        exits: Vec<(RecursorProducerOriginId, Option<RecursorProducerOriginId>)>,
    },
    DirectConsume {
        origin: RecursorProducerOriginId,
        selection_cursor: ContinuationCursorId,
        sibling_position: usize,
        exits: Vec<(RecursorProducerOriginId, Option<RecursorProducerOriginId>)>,
    },
    Selection {
        origin: RecursorProducerOriginId,
    },
    Exit {
        origin: RecursorProducerOriginId,
        scope_origin: RecursorProducerOriginId,
        parent_scope: Option<RecursorProducerOriginId>,
    },
    ReturnHole {
        cursor: ContinuationCursorId,
    },
    ResumeOuter {
        cursor: ContinuationCursorId,
    },
}
#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Px8trTrapProvenanceEvent {
    CheckedRecursorDefault {
        checked_frame_id: u64,
        actual_constructor: Option<RuntimeSymbol>,
        trap: RuntimeTrap,
    },
    DeforestedAnswerResumed {
        checked_frame_id: u64,
        actual_constructor: Option<RuntimeSymbol>,
        return_constructor: RuntimeSymbol,
    },
    /// **`RT-DECL-CLOSURE-PORT` `D6a` — the CARRIED answer route was emitted.**
    ///
    /// ⛔⛔ A separate event from [`Self::DeforestedAnswerResumed`], and the
    /// separation is the point. That one is recorded while lowering the
    /// **specialized** branch, where the scrutinee is a compile-time
    /// `Lowered::Constructor` and `actual_constructor` can name it. On the
    /// carried branch there is no such name to record and nothing at compile
    /// time knows which way a runtime word will go — so reusing that event here
    /// would dress a compile-time fact as runtime evidence.
    ///
    /// ⚠ This event says exactly one thing: **the carried route was emitted**
    /// into this frame's return case. It is an *emission* discriminator. The
    /// runtime half is the linked artifact's exit status, and the two are
    /// reported as a pair — neither substitutes for the other.
    CarriedAnswerRouteEmitted {
        checked_frame_id: u64,
        return_constructor: RuntimeSymbol,
    },
    FinalProcessObjectTrap {
        trap: RuntimeTrap,
    },
    /// **`RT-DECL-CLOSURE-PORT` `D6a` — EXACT trap provenance, recorded at the
    /// seat that emits the trap rather than at the seat that decided it.**
    ///
    /// ⭐ The frame requires the disabled checked-answer route to be proven
    /// through *"the planner trap identity at the unit `TrapWord` and root
    /// propagation seat"*, and states why: **the generic process `-4` string
    /// alone is not exact provenance.** That is not a stylistic preference.
    /// `-4` is the root adapter's single process-trap sentinel — it is the
    /// **same word for every trap in the program**, so a row asserting it
    /// cannot tell the checked-`ITree` default apart from any other trap the
    /// fixture could have reached, including one reached by a bug.
    ///
    /// ⚠ Both words are recorded, and the pair is the point:
    ///
    /// - `planned_identity` — what `StaticTransitionPlan::trap_identity` issued
    ///   for this exact `RuntimeTrap`. The authority.
    /// - `emitted_word` — what this seat actually put in the instruction
    ///   stream. Read according to `seat`.
    ///
    /// ⛔ Recording only the planned word would make this an assertion that the
    /// planner agrees with itself: `TrapIdentityMutation::{Zero,Substitute}`
    /// perturbs the *emitted* word and would leave such an event untouched.
    /// Recording the pair is what makes that existing mutation reach this row.
    PlannedTrapEmitted {
        trap: RuntimeTrap,
        seat: PlannedTrapSeat,
        planned_identity: i64,
        emitted_word: i64,
    },
    /// **`RT-DECL-CLOSURE-PORT` `D6a` — the ROOT PROPAGATION seat.**
    ///
    /// Recorded where a caller reads a callee unit's `TrapWord` and forwards
    /// it. ⛔ **Deliberately carries no identity**, and that is not an omission:
    /// the word here is a `stack_load`, a *runtime* value, so the compiler
    /// genuinely does not know which trap is flowing through. Recording a
    /// planned identity at this seat would be inventing one.
    ///
    /// ⭐ What the compiler *does* know is the authority, and therefore whether
    /// the callee's exact word survives the hop. That is the fact this event
    /// reports, and it is what makes the frame's rule legible rather than
    /// merely asserted: the identity is exact at [`PlannedTrapSeat::UnitTrapWord`]
    /// and is **collapsed** at [`PlannedTrapSeat::RootProcessSentinel`], which is
    /// exactly why the process `-4` string cannot stand in for provenance.
    UnitTrapWordPropagated {
        seat: PlannedTrapSeat,
        /// Whether the callee's exact trap word reaches the next frame
        /// recoverably. `false` only for the identity-free process sentinel.
        identity_preserved: bool,
    },
}
/// The three authorities [`Lowering::emit_current_trap`] can emit a trap under.
///
/// ⚠ **They are not interchangeable, and only the first two carry identity.**
/// `RootProcessSentinel` collapses every trap to one word by construction —
/// naming the seat is what stops a reader mistaking that collapse for a
/// measurement.
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PlannedTrapSeat {
    /// The generated unit's `TrapWord` lane. ⭐ The exact planner-issued
    /// identity word is stored here, so this is the seat where the checked
    /// default's provenance is genuinely exact.
    UnitTrapWord,
    /// The root adapter's process-object lane. ⛔ Identity-free: every trap
    /// emits the `-4` process sentinel.
    RootProcessSentinel,
    /// The root adapter's value lane, which shifts and tags the exact identity
    /// into a boundary trap token.
    RootTrapToken,
}
#[cfg(test)]
fn px8j_record_source_event(event: Px8jSourceTraceEvent) {
    PX8J_SOURCE_TRACE.with(|trace| trace.borrow_mut().push(event));
}
#[cfg(test)]
fn px8tr_record_trap_provenance(event: Px8trTrapProvenanceEvent) {
    PX8TR_TRAP_PROVENANCE.with(|trace| trace.borrow_mut().push(event));
}
#[cfg(test)]
fn px8tr_deforested_answer_route_enabled() -> bool {
    !PX8TR_DISABLE_DEFORESTED_ANSWER_ROUTE.get()
}

#[cfg(not(test))]
fn px8tr_deforested_answer_route_enabled() -> bool {
    true
}
/// Record which child a carried scrutinee's projection loop produced at each
/// position. ⭐ See `Px8jSourceTraceEvent::CarrierFieldProjection` — this is the
/// residual edge's independent oracle.
#[cfg(test)]
fn px8j_record_carrier_field_projection(
    path: Px8jProducerPath,
    position: usize,
    word: CarriedBoundaryWord,
) {
    px8j_record_source_event(Px8jSourceTraceEvent::CarrierFieldProjection {
        path,
        position,
        word: word.word,
    });
}

#[cfg(test)]
fn px8j_record_recursor_carrier(path: Px8jProducerPath, value: &LoweringOperand) {
    // ⭐ A trace probe, so it is total over both phases and takes neither
    // boundary: a carried operand is simply not a recursor carrier, which is the
    // same "nothing to record" answer any other non-recursor value gets.
    let LoweringOperand::Specialized(Lowered::ComputationalRecursorClosure {
        residual,
        invocation,
        ..
    }) = value
    else {
        return;
    };
    // ⭐ Total over both phases here too, and wildcard-free: the residual's
    // phase is part of what is observed, so an unclassifiable arm would be a
    // silent hole in exactly the edge this field exists to expose.
    let residual = match residual.as_ref() {
        LoweringOperand::Carried(word) => Px8jResidualPhase::Carried(word.word),
        LoweringOperand::Specialized(_) => Px8jResidualPhase::Specialized,
    };
    px8j_record_source_event(Px8jSourceTraceEvent::Carrier {
        path,
        origin: invocation.origin,
        cursor: invocation.resume_cursor,
        sibling_position: invocation.sibling_position,
        residual,
    });
}
fn verify_cranelift_function(
    func: &Function,
    isa: &dyn cranelift_codegen::isa::TargetIsa,
) -> Result<(), CraneliftBackendError> {
    verify_function(func, isa).map_err(|err| backend(BackendFailure::Verifier(err.to_string())))
}

// RT-SPLIT slice 5 (Architect `evt_3tgaw9ws44fqg`): test-only adapter letting
// the two artifact-subject `px8i_*` tests reach the private original across the
// ownership boundary. No ISA flags, validation, defaults, transformation, or
// error remapping — a single delegating call. Test scaffolding: absent from
// production builds, zero AC-7 production seams.
// Same shape, same rationale: the artifact-subject `px8i_local_helpers_*` test
// discriminates this lowering-private helper. Single delegating call, no policy.
#[cfg(test)]
pub(super) fn require_i64_for_artifact_tests(
    builder: &mut FunctionBuilder<'_>,
    actual: cranelift_codegen::ir::Value,
    expected: i64,
) {
    Lowering::require_i64(builder, actual, expected)
}

#[cfg(test)]
pub(super) fn verify_cranelift_function_for_artifact_tests(
    func: &Function,
    isa: &dyn cranelift_codegen::isa::TargetIsa,
) -> Result<(), CraneliftBackendError> {
    verify_cranelift_function(func, isa)
}
/// One source occurrence the lowering is working on: the expression the planner
/// walked, paired with the `StaticOriginId` the planner preallocated for it.
///
/// The pair exists so that an occurrence's static name travels **with** the term
/// rather than beside it, so a child's origin can be derived positionally as the
/// walk descends.
///
/// ⚠ Since `RT-FNSPLIT-B2A-S` the origin is **no longer provenance only** — for a
/// retained closure body it is the *selector*, and `retained_body_occurrence` is
/// the one place a term is recovered from one. Be precise about what is still
/// true, because the previous blanket claim here is now half wrong:
///
/// - it **does** select a retained closure body (that is the point of the unit);
/// - since `RT-CONTSPEC-ACTIVATE` `D3` it **also keys a collection**: the
///   producer `Construct` occurrence's origin is the first field of the
///   four-field causal selector that resolves a continuation call binding. The
///   earlier "never keys a collection" clause is retired, and it is retired
///   rather than narrowed because a reader who trusted it would look for the
///   binding lookup somewhere other than the origin;
/// - it still **never** alters a branch or reaches emitted code by comparison,
///   ordering, or arithmetic — the binding lookup is an exact equality on a
///   planner-issued identity, not a decision computed from an origin's value.
#[derive(Clone, Copy)]
struct SourceOccurrence<'a> {
    expr: &'a RuntimeExpr,
    static_origin: StaticOriginId,
}

/// An owned source occurrence, for the points where the lowering **clones** a
/// `RuntimeExpr` into a pending frame or a cloneable prefix template.
///
/// ⛔ **No longer a retained closure**: `RT-FNSPLIT-B2A-S` removed that, and
/// `Lowered::Closure`/`DeclarationClosure` now name their body by origin alone. The
/// source machine's in-flight frames still own their terms, and that is a forced
/// boundary rather than a leftover — `lower_source_forked_match` synthesizes a
/// `Trap` that exists nowhere in the source tree, so it has no planned occurrence
/// to be resolved from and cannot be represented by a tag.
///
/// ⭐ The pair is one value on purpose: `SourcePrefixTemplate` is `Clone`, and a
/// clone that copied the term while dropping its origin would silently
/// reintroduce exactly the recoverability vacancy the pair exists to close.
/// Making them one field makes that drop unspellable.
#[derive(Clone)]
struct OwnedSourceOccurrence {
    expr: RuntimeExpr,
    static_origin: StaticOriginId,
}

impl OwnedSourceOccurrence {
    /// Clones a borrowed occurrence into an owned one, carrying the origin in
    /// the same constructor as the clone.
    fn cloned(occurrence: SourceOccurrence<'_>) -> Self {
        Self {
            expr: occurrence.expr.clone(),
            static_origin: occurrence.static_origin,
        }
    }
}


/// **Everything that is resolved into ONE generated `Function` and is
/// meaningless in any other.**
///
/// ⛔⛔ **Nothing in here is portable across functions, and the three kinds fail
/// differently.** `FuncRef`, `GlobalValue` and `ir::Value` are all
/// function-scoped entity references in Cranelift:
///
/// | field kind | what it is | moving it to another function |
/// |---|---|---|
/// | `FuncRef`s, `SeedMaterialRefs`' `GlobalValue`s, `BoundaryCarrierRefs` | an identity **resolved into** a function | ⚠ must be **re-resolved** — the identity survives, the handle does not |
/// | `host_dispatch_context`, `native_int_arena` | a **result of this function's own dataflow** | ⛔ must be **re-derived** — there is no identity to re-resolve |
/// | `native_int_tags` | a map **keyed on `ir::Value`** | ⛔⛔ **silently aliases** |
///
/// ⭐ **The `native_int_tags` row is the dangerous one, and it is why this
/// struct exists.** Entity references restart per `Function` — `v0`, `v1`, …
/// are reused — so a tag map carried from one function into another answers a
/// lookup for an unrelated value **that happens to share the numeric handle**.
/// ⛔ No type error, no panic, no verifier complaint: a wrong tag on a path that
/// still compiles.
///
/// ⇒ ⭐ **`RT-FNSPLIT-B2F` `S6` gives each unit body its own function, so this
/// state must be partitioned by construction.** ⛔ A `reset()` someone has to
/// remember to call is the same defect with an extra step. Gathering the fields
/// under one name is what makes *"is this per-function state?"* answerable by
/// reading one struct instead of auditing a hundred-field one.
///
/// ⚠ **This is the identity-alias class in a third substrate** — after `B2O`
/// removed it from `SemanticDescriptor` and `D1` kept it out of `UnitBundle`,
/// here it is again in Cranelift's own entity references.
///
/// ⛔ **NOT CLAIMED: no control demonstrates the alias.** Producing one requires
/// the `S6` switch-over that does not exist yet, so this is a read of Cranelift's
/// entity scoping, not a measurement. ⚠ What *is* structural is that a second
/// function cannot silently inherit this state without a visible second
/// construction of this struct.
/// **`RT-FNSPLIT-B2F` `S6`** — the module-level identities every generated
/// function must resolve for itself, and the sole producer of a
/// [`FunctionLocalRefs`].
///
/// ⭐ **This is the OTHER side of the construction boundary, and naming it is
/// what makes the boundary checkable.** [`FunctionLocalRefs`] says what is *not*
/// portable; this says what *is*. A `FuncId` and a `DataId` are module-scoped:
/// they mean the same thing in every function of the artifact, and
/// [`Self::declare_in_func`] is the one operation that turns them into the
/// function-scoped handles a body can actually reference.
///
/// ⛔ **There is deliberately no way to build a `FunctionLocalRefs` in
/// production except through [`Self::declare_in_func`].** The root and every
/// future unit body therefore cannot drift: a helper added here is resolved into
/// *every* generated function or into none, and *"the root has a carrier ref and
/// the unit does not"* stops being expressible by forgetting to copy a line.
/// ⚠ The `#[cfg(test)]` fixtures build the struct directly and are the stated
/// exception; they emit into no module and hold `None` throughout.
///
/// **MEASURED:** every resolved-handle field of `FunctionLocalRefs` is produced
/// by one function, from module-scoped identities held here.
/// **CLAIMED:** any second generated function emits against the same helper
/// surface the root does.
/// **THE GAP:** ⛔ **there is no second generated function yet** — `S6`'s
/// switch-over is not landed, so today this has exactly one caller and the claim
/// is about a population of one. ⚠ It is *not* claimed that this suffices for a
/// unit body: the two `ir::Value` fields are **not** resolvable from here,
/// because they are results of a function's own dataflow rather than identities,
/// and `declare_in_func` leaves them `None` for each caller to derive.
///
/// **THE TRAP DATAFLOW BOUNDARY:** a root's closed [`TrapExitAuthority`] is
/// supplied at construction. A unit's `slots` value is loaded from that
/// function's own envelope, so its authority is absent until
/// [`FunctionLocalRefs::bind_unit_trap_frame`] binds the exact frame.
#[derive(Clone, Copy)]
struct ArtifactHelpers<'h> {
    seed_material: &'h seed_material::SeedMaterial,
    host_dispatch: Option<FuncId>,
    native_int: &'h crate::native_int_clif::NativeIntLocalFuncs,
    boundary_value_abi: &'h crate::boundary_value_clif::BoundaryLocalFuncs,
}

impl ArtifactHelpers<'_> {
    /// Resolve every module-level identity into `func`.
    ///
    /// ⛔ **Call this once per `Function`, and never reuse the result.** The
    /// returned handles are function-scoped; see [`FunctionLocalRefs`] for what
    /// each of the three kinds does when it crosses a function boundary.
    fn declare_in_func<M: Module>(
        self,
        module: &mut M,
        func: &mut Function,
        trap_exit: Option<TrapExitAuthority>,
    ) -> FunctionLocalRefs {
        FunctionLocalRefs {
            seed_material: self.seed_material.declare_in_func(module, func),
            host_dispatch: self
                .host_dispatch
                .map(|id| module.declare_func_in_func(id, func)),
            // ⛔ Dataflow results, not identities: `None` here is correct, and
            // each function derives its own from its entry block.
            host_dispatch_context: None,
            services_pointer: None,
            native_int_arena: None,
            // ⛔ Sourced by the activation-services record, which the `S6`/`D6`
            // reland introduces. Fail closed until then.
            boundary_arena: None,
            native_int_binop: Some(module.declare_func_in_func(self.native_int.binop, func)),
            native_int_compare: Some(module.declare_func_in_func(self.native_int.compare, func)),
            native_int_intern: Some(module.declare_func_in_func(self.native_int.intern, func)),
            native_int_narrow: Some(module.declare_func_in_func(self.native_int.narrow, func)),
            native_int_export: Some(module.declare_func_in_func(self.native_int.export, func)),
            native_int_export_parts: Some(
                module.declare_func_in_func(self.native_int.export_parts, func),
            ),
            // ⭐ The **one** exact-`Int` decoder, resolved here so the
            // region-limbed spill copies through the landed representation
            // instead of growing a second one.
            native_int_resolve: Some(module.declare_func_in_func(self.native_int.resolve, func)),
            // ⛔ Empty, never inherited. This is the map whose `ir::Value` keys
            // alias across functions; starting it empty per function is why the
            // two structs are separate types rather than one with a `reset()`.
            native_int_tags: BTreeMap::new(),
            unit_calls: BTreeMap::new(),
            worker_calls: BTreeMap::new(),
            raw_worker_calls: BTreeMap::new(),
            worker_templates: BTreeMap::new(),
            context_calls: BTreeMap::new(),
            defining_abi_operands: Vec::new(),
            #[cfg(test)]
            defining_abi_slot_kinds: Vec::new(),
            generated_context_captures: None,
            constructed_context_frame: None,
            continuation_calls: BTreeMap::new(),
            continuation_emissions: BTreeMap::new(),
            pending_composed_discharges: Vec::new(),
            composed_discharges: BTreeMap::new(),
            declaration_calls: BTreeMap::new(),
            trap_exit,
            terminal_result_origins: BTreeSet::new(),
            consumed_join_origins: BTreeSet::new(),
            dispositioned_join_origins: BTreeSet::new(),
            join_disposition_finalized: false,
            final_reachable_join_origins: BTreeSet::new(),
            materialized_join_blocks: BTreeMap::new(),
            emission_reachable_match_cases: BTreeMap::new(),
            boundary_carrier: Some(BoundaryCarrierRefs {
                class: module.declare_func_in_func(self.boundary_value_abi.class, func),
                tag: module.declare_func_in_func(self.boundary_value_abi.tag, func),
                field_count: module.declare_func_in_func(self.boundary_value_abi.field_count, func),
                field: module.declare_func_in_func(self.boundary_value_abi.field, func),
                record_field: module
                    .declare_func_in_func(self.boundary_value_abi.record_field, func),
                scalar: module.declare_func_in_func(self.boundary_value_abi.scalar, func),
                host_success: module
                    .declare_func_in_func(self.boundary_value_abi.host_success, func),
                host_payload: module
                    .declare_func_in_func(self.boundary_value_abi.host_payload, func),
                alloc: module.declare_func_in_func(self.boundary_value_abi.alloc, func),
                store_tag_id: module
                    .declare_func_in_func(self.boundary_value_abi.store_tag_id, func),
                store_scalar: module
                    .declare_func_in_func(self.boundary_value_abi.store_scalar, func),
                store_field: module.declare_func_in_func(self.boundary_value_abi.store_field, func),
                store_name: module.declare_func_in_func(self.boundary_value_abi.store_name, func),
                make_immediate: module
                    .declare_func_in_func(self.boundary_value_abi.make_immediate, func),
                store_int_tag: module
                    .declare_func_in_func(self.boundary_value_abi.store_int_tag, func),
                store_bytes_len: module
                    .declare_func_in_func(self.boundary_value_abi.store_bytes_len, func),
                store_byte: module.declare_func_in_func(self.boundary_value_abi.store_byte, func),
                store_int_limbs: module
                    .declare_func_in_func(self.boundary_value_abi.store_int_limbs, func),
                store_int_limb: module
                    .declare_func_in_func(self.boundary_value_abi.store_int_limb, func),
                seal_int: module.declare_func_in_func(self.boundary_value_abi.seal_int, func),
                int_view: module.declare_func_in_func(self.boundary_value_abi.int_view, func),
                bytes_view: module
                    .declare_func_in_func(self.boundary_value_abi.bytes_view, func),
            }),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TrapExitAuthority {
    UnitFrame {
        slots: cranelift_codegen::ir::Value,
        trap_offset: i32,
    },
    Root {
        process_sentinel: bool,
        source_authorized: bool,
    },
}

/// **`RT-DECL-CLOSURE-PORT` `D5a`** -- the continuation-input operands one
/// enclosing specialization passes across a retargeted worker call.
/// **`RT-CAPTURE-CONTEXT-FRAME-EMIT` `D2` -- the generated context's own frame,
/// CONSTRUCTED at the creation site from the producer's live environment.**
///
/// The closure conversion the Architect ruled (`evt_7vh5nccb9gcqy`). A carried
/// recursive-position invocation retargeted onto a generated context must
/// supply that context's whole declared frame, and two of its runs cannot be
/// gathered where the retarget happens:
///
/// - the **worker-capture tail of the `Parameter` run** -- the carried
///   invocation supplies only the raw body's declared arguments, so the
///   selected closure's captures are simply absent there;
/// - the **`Capture` run** -- the enclosing specialization's continuation
///   inputs, whose producer-local members live in the producer's semantic
///   environment and are **not ABI operands at all**, so
///   `function_local.defining_abi_operands` structurally cannot hold them.
///
/// Both runs ARE in hand at `assemble_continuation_call_operands`, which runs
/// in the enclosing function's body with `producer_env` live and resolves every
/// member through the planner's own projections. This carries them from there
/// to the retarget, so the frame is **materialized where its free variables are
/// live** rather than re-derived where they are not.
///
/// **This supplies members; it relaxes no check.** The operands are presented
/// in the context's declared order and the consumer re-verifies both
/// cardinalities against the context's own frame header before using them, so a
/// mis-ordered or mis-counted frame still refuses. `verify_entry_frame`'s
/// membership and slot re-derivation guard is untouched: the context body still
/// walks its own declared run.
///
/// Keyed by the **complete planner-issued coordinate triple**, never by body
/// origin alone. One function can reach two retargets over one body, and a
/// frame consumed at the wrong one would be an arity-correct call carrying
/// another occurrence's values -- the exact silent shape `D6a` and
/// `generated_context_captures` both guard against by retaining their key.
struct ConstructedContextFrame {
    /// The eliminator occurrence whose recursive position this frame serves.
    continuation_origin: StaticOriginId,
    /// That position, as the planner issued it.
    recursive_position: u32,
    /// The selected worker body the context executes.
    worker_body_origin: StaticOriginId,
    /// The selected closure's captures, in **capture-ordinal** order -- the
    /// tail of the context's `Parameter` run, after the declared arguments the
    /// carried invocation itself supplies.
    worker_captures: Vec<LoweringOperand>,
    /// The enclosing specialization's continuation inputs, in ordinal order --
    /// the context's own `Capture` run.
    context_captures: Vec<LoweringOperand>,
}

struct GeneratedContextCaptures {
    /// The exact worker body origin whose call carries this suffix.
    worker_body_origin: StaticOriginId,
    /// The enclosing specialization's continuation inputs, in ordinal order.
    operands: Vec<LoweringOperand>,
}

struct FunctionLocalRefs {
    /// **`RT-FNSPLIT-B2F` `D3`** — the artifact-static seed material, resolved
    /// into this generated function.
    ///
    /// ⭐ Held **alongside** `Lowering::seed_env`, not instead of it, and the
    /// division is the point: `seed_env` answers *which* `RuntimeGroundValue` a
    /// symbol denotes — a compile-time question — while this answers *where the
    /// running artifact reads it from*. ⛔ Collapsing the two would either put a
    /// compilation-only borrow in the artifact (which does not typecheck) or
    /// re-fold the value into the instruction stream (which is the authority
    /// `D3` removes).
    seed_material: seed_material::SeedMaterialRefs,
    host_dispatch: Option<FuncRef>,
    host_dispatch_context: Option<cranelift_codegen::ir::Value>,
    services_pointer: Option<cranelift_codegen::ir::Value>,
    native_int_arena: Option<cranelift_codegen::ir::Value>,
    /// ⭐ **The BOUNDARY arena, and it is a different pointer from
    /// [`Self::native_int_arena`]** — Architect ruling via `evt_e300y2kjeb6k`.
    /// The two answer different questions and were wrongly merged into one
    /// field; see [`Lowering::carrier_arena`] for the retraction.
    ///
    /// ⚠ **`None` in production, because NOTHING PUBLISHES A BOUNDARY ARENA
    /// YET** — measured, not assumed: every `BoundaryRegion::reserve` /
    /// `reserve_persistent` call site in the crate is a test, so the activation
    /// owner the ruling assigns this to does not exist on either launcher path.
    /// ⇒ Every boundary-carrier call fails closed until then — ⭐ which is
    /// strictly better than the native arena it used to be handed silently.
    boundary_arena: Option<cranelift_codegen::ir::Value>,
    native_int_binop: Option<FuncRef>,
    native_int_compare: Option<FuncRef>,
    native_int_intern: Option<FuncRef>,
    native_int_narrow: Option<FuncRef>,
    native_int_export: Option<FuncRef>,
    native_int_export_parts: Option<FuncRef>,
    /// `(arena, tag, payload, out_view) -> status` — the sole exact-`Int`
    /// decoder, `ken_native_int_resolve_local`.
    native_int_resolve: Option<FuncRef>,
    native_int_tags: BTreeMap<cranelift_codegen::ir::Value, cranelift_codegen::ir::Value>,
    /// The boundary-carrier helpers, made callable inside **this** generated
    /// function (`RT-FNSPLIT-C1` `D3`).
    ///
    /// ⛔ `FuncRef`s, not `FuncId`s — the ruling requires the helper IDs to be
    /// *"declared into each generated function as callable refs and actually
    /// called by all three routes"*, and a `FuncId` held here would be exactly
    /// the inert threading the node forbids: present, plausible, and never
    /// reaching an emitted call.
    boundary_carrier: Option<BoundaryCarrierRefs>,
    unit_calls: BTreeMap<StaticOriginId, units::DeclaredUnitCall>,
    /// **`D4`** -- this function's own static-worker call targets, keyed by
    /// exact body origin. Minted per generated function; a `FuncRef` here
    /// belongs to that function and is never copied to another.
    worker_calls: BTreeMap<StaticOriginId, units::DeclaredUnitCall>,
    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D6b`** -- this function's own call targets
    /// for the **raw worker bodies**, keyed by exact body origin, as declared
    /// *before* any retarget.
    ///
    /// ⛔ A fifth map rather than a flag on `worker_calls`, because the two
    /// answer different questions for the **same** key. In a retargeted
    /// specialization `worker_calls[body]` has been overwritten with the
    /// generated context that executes `body`, so the raw callee is no longer
    /// reachable from that map at all -- and a
    /// [`StaticWorkerCallRoute::RawWorker`] binding for that same body still
    /// needs it. Keeping one map and choosing by route is impossible when one
    /// entry has been replaced; keeping both is the whole mechanism.
    ///
    /// ⛔ Same per-function discipline as every table here: minted by
    /// `WorkerTargets::declare_in_func` into *this* `Function`, never copied
    /// between functions, so no `FuncRef` crosses a boundary.
    ///
    /// ⚠ **A body absent here is a body with no emitted `Function`.** This map
    /// is declared from the *executable* population, so a body that a total
    /// retarget made template-only is legitimately missing. `call_static_worker`
    /// fails closed on the miss rather than falling back to `worker_calls` --
    /// a fallback would silently route a raw call through a generated context
    /// whose ABI expects a capture suffix this caller does not supply.
    raw_worker_calls: BTreeMap<StaticOriginId, units::DeclaredUnitCall>,
    /// **`RT-DECL-CLOSURE-PORT` `D5a` checkpoint 1** -- the RAW descriptor
    /// contract for every worker body, executable or template-only.
    ///
    /// ⛔ Separate from `worker_calls` on purpose, and carrying no `FuncRef`.
    /// `construct_static_worker_binding` validates identity and arity against
    /// **this**, while the call is emitted through `worker_calls`, which may
    /// have been retargeted to a generated context. Validating against the
    /// thing you are about to call would make the retarget invisible to the
    /// check; validating against the raw template is what "unchanged ordinary
    /// `fn2` ABI" actually means.
    worker_templates: BTreeMap<StaticOriginId, units::WorkerTemplate>,
    /// **`RT-DECL-CLOSURE-PORT` `D5a` checkpoint 4 step 1** -- this function's
    /// own call targets for the generated execution contexts, keyed by the
    /// planner's context identity.
    ///
    /// ⛔ A fourth map rather than an entry in `worker_calls` or `unit_calls`:
    /// a `ContinuationContextId` is its own identity domain, and keying a
    /// context by the body origin it executes is precisely the "reconstruct the
    /// binding from body origin" the ruling forbids. Minted per function; no
    /// `FuncRef` crosses a function.
    context_calls: BTreeMap<ContinuationContextId, units::DeclaredUnitCall>,
    /// **`RT-DECL-CLOSURE-PORT` `D5a` checkpoint 4 step 1b** -- this function's
    /// own ABI-slot operands, indexed by ABI position.
    ///
    /// The `Parameter` run followed by the `Capture` run, in descriptor order,
    /// from the one slot walk -- so index `i` here is ABI position `i` of the
    /// function being defined.
    ///
    /// **`RT-SRCBODY-BIND-ORDER` `D1`: this is no longer the body's
    /// environment order.** The same walk still produces both, but a source
    /// body's semantic environment is `reverse(Parameter run) ++ Capture run`,
    /// because `lower_expr` resolves `Var(i)` as a de Bruijn index. THIS vector
    /// keeps descriptor order and is the ABI-position authority; do not read an
    /// environment index out of it or an ABI position out of an environment.
    ///
    /// ⭐ A retargeted carried invocation reads its context's capture suffix
    /// from here, at the **immediate slots** the planner assigned. It is stored
    /// rather than threaded because the call seam is six frames below the body
    /// walk; the alternative was passing an environment through every
    /// intermediate, which is how a seat ends up reading whichever environment
    /// happened to be nearest.
    ///
    /// ⛔ Reset per function, like every other field here: these are `ir::Value`
    /// operands of one `Function` and mean nothing in another.
    defining_abi_operands: Vec<LoweringOperand>,
    /// **`RT-SRCBODY-BIND-ORDER` `D3c`** -- the slot KIND at each ABI position
    /// of [`Self::defining_abi_operands`], recorded in the same walk.
    ///
    /// Independent source-descriptor authority for the `D3c` observatory,
    /// and that independence is the point. Deriving where an ABI position lands
    /// in the semantic environment needs two descriptor facts -- whether the
    /// position is a `Parameter` and how long the `Parameter` run is -- and
    /// both must come from the DESCRIPTOR, never from searching the environment
    /// for the operand. A search would make the instrument agree with whatever
    /// production did.
    ///
    /// `cfg(test)`: nothing in production needs a slot's kind after the walk.
    #[cfg(test)]
    defining_abi_slot_kinds: Vec<AbiSlotKind>,
    /// **`RT-DECL-CLOSURE-PORT` `D5a`** -- the operand suffix a **retargeted**
    /// worker call must append, and the one body origin it applies to.
    ///
    /// `None` in every function that calls raw worker units directly, which is
    /// every pre-`D5a` case. When present it is set by the enclosing
    /// specialization's own body, from that frame's Capture slots, so the
    /// operands are `ir::Value`s of *this* function and cannot be reused
    /// elsewhere -- the same per-function discipline `worker_calls` follows.
    ///
    /// ⛔ The body origin is retained beside the operands rather than being
    /// implicit: a suffix appended to the wrong worker call would be a silent
    /// arity error at a frame that happened to be big enough.
    generated_context_captures: Option<GeneratedContextCaptures>,
    /// **`RT-CAPTURE-CONTEXT-FRAME-EMIT` `D2`** -- see
    /// [`ConstructedContextFrame`]. Written at the creation site, consumed at
    /// the carried-invocation retarget, and per-function for the same reason
    /// `generated_context_captures` is: the operands are `ir::Value`s of this
    /// Function.
    constructed_context_frame: Option<ConstructedContextFrame>,
    /// **`RT-CONTSPEC-ACTIVATE` `D3`** -- this Function's own `FuncRef` per
    /// causal token it owns, keyed by the complete four-field identity.
    /// Minted into this `Function`; never passed across functions.
    continuation_calls: BTreeMap<ContinuationCallIdentity, units::DeclaredUnitCall>,
    /// **`RT-CONTSPEC-ACTIVATE` `4b`** -- the exact `Inst` this Function emitted
    /// for each causal token, recorded at the `builder.ins().call` that produced
    /// it.
    ///
    /// ⭐ This is an **anchor, not an answer**. It records *where* a call was
    /// emitted, never *what* it calls: the callee is decoded back out of the
    /// finished CLIF at
    /// [`Lowering::verify_emitted_continuation_calls`]. Recording the target
    /// here instead would make the gate compare the value it was handed with
    /// the value it was handed.
    ///
    /// ⛔ An entry exists only because a call instruction exists, so a token
    /// that was claimed and never called leaves no entry -- which is the whole
    /// reason the emission set is kept separately from the claim ledger.
    continuation_emissions: BTreeMap<ContinuationCallIdentity, cranelift_codegen::ir::Inst>,
    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D8j`** — composed discharges this function
    /// has CLAIMED but not yet verified.
    ///
    /// ⛔ A claim is not a discharge. Entries land here at the source-machine
    /// seat, after the raw-worker call is emitted and its result has returned
    /// to unchanged source-machine control, and they move into
    /// [`Self::composed_discharges`] only once the finished CLIF has been
    /// consulted. Nothing outside `verify_recorded_composed_discharges` may
    /// read this as a discharge.
    pending_composed_discharges: Vec<PendingComposedDischarge>,
    /// **`D8j` — the verified composed-discharge relation for this function.**
    ///
    /// ⛔⛔ **Deliberately NOT `continuation_emissions`.** That map's gate
    /// requires the recorded instruction to decode to `identity.target()` — the
    /// specialization the causal call names. A lawful composed instruction
    /// targets the **raw worker**, so putting a composed record there would
    /// either fail a gate it was never about or force that gate to be loosened
    /// for every direct emission. Two relations, two contracts.
    ///
    /// ⛔ Populated only by `verify_recorded_composed_discharges`, only from
    /// [`Self::pending_composed_discharges`], and only after all five
    /// verifications pass. `D8k` owns whatever global closure reads it.
    composed_discharges: BTreeMap<ContinuationCallIdentity, cranelift_codegen::ir::Inst>,
    declaration_calls: BTreeMap<StaticOriginId, units::DeclaredUnitCall>,
    /// The current function's closed trap-exit authority. Absence is an error
    /// state, never an implicit Root.
    trap_exit: Option<TrapExitAuthority>,
    /// Source occurrences reached only through the current unit's result
    /// position. Process-exit constructors are normalized only at these
    /// occurrences, never merely because an exit-shaped value appears nested.
    terminal_result_origins: BTreeSet<StaticOriginId>,
    /// Join-plan entries consumed while defining this function.  Each
    /// `FunctionLocalRefs` is freshly declared for one generated function, so
    /// this set cannot alias consumption across unit bodies.
    consumed_join_origins: BTreeSet<StaticOriginId>,
    /// Planned joins under source branches proven statically unselected while
    /// defining this function.
    ///
    /// Keeping these separate from consumed joins makes both mismatches loud:
    /// entering a dead join rejects, while failing to disposition one leaves
    /// the generated-function closure check red.
    dispositioned_join_origins: BTreeSet<StaticOriginId>,
    /// Whether the owner-bound reachable/dead partition has been closed.
    ///
    /// The boolean is separate from either partition because a generated
    /// function may lawfully own no reachable joins.
    join_disposition_finalized: bool,
    /// The final semantically reachable half of the owner-bound join
    /// population. This is derived only after every reached-case union closes;
    /// it is not inferred from structural token consumption.
    final_reachable_join_origins: BTreeSet<StaticOriginId>,
    /// Actual CLIF merge blocks materialized for each planned source join.
    ///
    /// Token consumption can precede semantic selection without producing a
    /// merge block. Keeping the CFG population separate is what lets closure
    /// distinguish metadata materialization from a live SSA join.
    materialized_join_blocks: BTreeMap<StaticOriginId, BTreeSet<Block>>,
    /// Case indices actually reached while emitting each statically selected
    /// source `Match`.
    ///
    /// Selection may revisit one source occurrence through a recursive
    /// producer and reach a different case later. We therefore close the dead
    /// population only after emission, from the union of reached indices,
    /// rather than treating the first observed constructor as globally final.
    emission_reachable_match_cases: BTreeMap<StaticOriginId, BTreeSet<usize>>,
}


#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HostContextPropagationMutation {
    Exact,
    ServicesPointer,
    NativeIntArena,
    BoundaryArena,
    Null,
    LaunchIngress,
}

#[cfg(test)]
thread_local! {
    static HOST_CONTEXT_PROPAGATION_MUTATION:
        std::cell::Cell<HostContextPropagationMutation> =
        const { std::cell::Cell::new(HostContextPropagationMutation::Exact) };
}

#[cfg(test)]
thread_local! {
    static D8_CARRIED_JOIN_UNCHANGED: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
    static D8_SPECIALIZED_JOIN_PRODUCTIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
    static D8_JOIN_MERGES_CREATED: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
    static D8_JOIN_CONSUMPTION_MUTATION: std::cell::Cell<JoinConsumptionMutation> =
        const { std::cell::Cell::new(JoinConsumptionMutation::Exact) };
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum JoinConsumptionMutation {
    Exact,
    SkipFirst,
    DuplicateFirst,
    IncludeStaticallyUnselected,
    OmitFirstStaticallyUnselectedMatchCase,
    OmitSourceMachineComputationalMatchSelection,
    MaterializeFirstUnselectedMatchJoin,
    AttachEntryToFirstMaterializedDead,
    /// **`RT-MATERIALIZED-DEAD-JOIN-RECONCILE` `D1` -- reconstructs the
    /// consumed-AND-dispositioned overlap SYNTHETICALLY.**
    ///
    /// The disposition path can no longer build that state: a consumed origin
    /// is never dispositioned dead, so `consumed intersect dispositioned` is
    /// empty by construction and every guard downstream of it is unreachable
    /// from that producer. This mutation forces the overlap anyway, and
    /// attaches an entry-reachable block to it, so
    /// `validate_materialized_dead_join_cfg` keeps a LIVE test of its own
    /// contract.
    ///
    /// Without it the validator is defence-in-depth in name only: byte-untouched,
    /// permanently green, and free to rot against some future path that
    /// reintroduces the state it exists to catch.
    ForceMaterializedDeadOverlapWithEntry,
    DispositionDynamicHostResultMerge,
}

#[cfg(test)]
fn reset_d8_join_conversion_counts() {
    D8_CARRIED_JOIN_UNCHANGED.with(|count| count.set(0));
    D8_SPECIALIZED_JOIN_PRODUCTIONS.with(|count| count.set(0));
    D8_JOIN_MERGES_CREATED.with(|count| count.set(0));
}

#[cfg(test)]
fn d8_join_conversion_counts() -> (usize, usize) {
    (
        D8_CARRIED_JOIN_UNCHANGED.with(std::cell::Cell::get),
        D8_SPECIALIZED_JOIN_PRODUCTIONS.with(std::cell::Cell::get),
    )
}

#[cfg(test)]
fn d8_join_merge_count() -> usize {
    D8_JOIN_MERGES_CREATED.with(std::cell::Cell::get)
}

#[cfg(test)]
fn set_d8_join_consumption_mutation(mutation: JoinConsumptionMutation) {
    D8_JOIN_CONSUMPTION_MUTATION.with(|cell| cell.set(mutation));
}

#[cfg(test)]
fn set_host_context_propagation_mutation(mutation: HostContextPropagationMutation) {
    HOST_CONTEXT_PROPAGATION_MUTATION.with(|cell| cell.set(mutation));
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProcessSlotMutation {
    Exact,
    DeleteProcessInput,
    DeleteCapability,
    AttemptFixedContextOffsets,
    ReintroduceLaunchIngress,
}

#[cfg(test)]
thread_local! {
    static PROCESS_SLOT_MUTATION: std::cell::Cell<ProcessSlotMutation> =
        const { std::cell::Cell::new(ProcessSlotMutation::Exact) };
}

#[cfg(test)]
fn set_process_slot_mutation(mutation: ProcessSlotMutation) {
    PROCESS_SLOT_MUTATION.with(|cell| cell.set(mutation));
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TrapFrameBindingMutation {
    Exact,
    DeleteUnitLane,
    MisclassifyUnitAsRoot,
}



/// **`RT-CONTSPEC-ACTIVATE` `D4` — the three executable controls for the
/// continuation emission seam.**
///
/// All `#[cfg(test)]`; production compiles as if they did not exist. Each sits
/// on the exact production branch it perturbs, so its red reproduces from the
/// committed tree by flipping the switch.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ContinuationEmissionMutation {
    Exact,
    /// `D4` emission seam: substitute **only the function-local emitted
    /// `FuncRef`** with another callable already declared in this same
    /// function, retaining the continuation's header, slots, offsets, inputs,
    /// identity, and owner.
    ///
    /// ⭐ **This replaces the same-shaped two-target redirect, which was
    /// unreachable here and was therefore never a control at all.** That one
    /// searched `continuation_calls` for a *distinct same-shaped* entry; this
    /// generated function holds exactly one entry, so it refused with "found no
    /// distinct same-shaped call target" **before reaching the call seam**. A
    /// pre-call refusal proves a missing fixture precondition, not emitted-
    /// target equality. Architect `evt_6bf2mmehjzy3k`.
    ///
    /// ⛔ Substituting the ref and nothing else is what isolates the `ACTIVATE`
    /// property: the call is still emitted, on the same ABI, with the same
    /// inputs -- only the callee identity moves, so the finished-CLIF oracle
    /// must reject for exactly one reason.
    ///
    /// ⛔ No fall back to exact. If no other callable is declared in this
    /// function the control fails loudly, because a control that silently
    /// becomes the identity is vacuous.
    ///
    /// The **two-target same-shaped** redirect and its observable behavioural
    /// consequence live wholly in `RT-CONTSPEC-WITNESS` `D7`/`AC-9`, whose
    /// integrated fixture must supply two distinct same-shaped targets in one
    /// lawful callable population.
    SubstituteEmittedFuncRef,
    /// **`RT-CONTSPEC-WITNESS` `D7` — the two-target same-shaped call-site
    /// redirect.**
    ///
    /// Selects a **distinct** target whose **declared arity and capture count**
    /// equal the exact target's, per `RT-WORKER-BIND`'s definition of
    /// same-shaped. Distinctness is on the emitted callable, never on origin
    /// inequality; sameness is on the two declared counts, never on widths,
    /// alignments, offsets, carriers, ownership or the frame header.
    ///
    /// ⛔ **REACHABILITY EVIDENCE ONLY. This control's mutated arm never
    /// executes, and it is not an executed-result oracle.** The finished-CLIF
    /// equality gate compares the emitted callee against
    /// `bundle.continuation(identity.target())`, so moving only the emitted
    /// `FuncRef` is rejected before the program runs. That is
    /// `RT-CONTSPEC-ACTIVATE`'s static face.
    ///
    /// ⭐ **What it does prove, which is worth keeping.** In `ACTIVATE` the
    /// same-shaped redirect refused with "found no distinct same-shaped call
    /// target" **before** reaching the call, so it was not a control at all.
    /// With a two-target population it resolves a distinct same-shaped target
    /// and the seam is entered. That is a statement about **reach**, and it must
    /// not be read as one about behaviour.
    ///
    /// ⇒ `AC-9`'s behavioural obligation is discharged elsewhere, by
    /// [`ContinuationEmissionMutation::SubstituteContinuationBodyDefinition`],
    /// which perturbs the declaration-to-body binding the equality gate cannot
    /// see and yields a changed **executed** answer.
    ///
    /// ⛔ No fall back to exact. If this function declares no distinct
    /// same-shaped target the control fails loudly rather than silently
    /// becoming the identity.
    RedirectToDistinctSameShapedTarget,
    /// **`RT-CONTSPEC-WITNESS` `D7`/`AC-9` — the behavioural witness, at the
    /// continuation DEFINITION-BINDING seat.** Architect ruling 2026-08-08.
    ///
    /// ⭐ **Why this seat and not the call site.** A call-site `FuncRef`
    /// redirect is *structurally* unable to execute while the equality gate is
    /// present: finished CLIF is compared against
    /// `bundle.continuation(identity.target())`, so moving only the emitted
    /// callee must reject before anything runs. But that left-hand side is
    /// **not** a planner population — `UnitBundle::continuation` is the
    /// lowering forward-declaration naming authority, and
    /// `define_continuation_bodies` is the producer that binds each declared
    /// continuation function to the body it executes.
    ///
    /// ⇒ The gate proves planner-identity to emitted-callee routing. It does
    /// **not** prove that the declaration-to-body binding is right, or what the
    /// bound body computes. That residual is `RT-CONTSPEC-ACTIVATE`'s own
    /// stated one, and it is exactly what `AC-9` needs.
    ///
    /// This mutation substitutes the **body authority defined under the exact
    /// continuation `FuncId`**, selected by the same declared-arity and
    /// capture-count predicate as
    /// [`ContinuationEmissionMutation::RedirectToDistinctSameShapedTarget`].
    /// The causal token, specialization id, declared `FuncId`, header, slots,
    /// offsets, inputs, owner and emitted call are all preserved, and
    /// `verify_emitted_continuation_calls` stays enabled and **green
    /// naturally** — so a red here cannot be the static gate firing.
    ///
    /// ⛔ No fall back to exact, and the application is counted: a control that
    /// silently becomes a no-op would pass while measuring nothing.
    SubstituteContinuationBodyDefinition,
    /// `4b` closure seam: emit the direct call but do not record it against its
    /// causal token, so the finished-CLIF sweep must notice an emission the
    /// records do not account for.
    SuppressEmissionRecord,
    /// `4b` closeout seam: verify each function's emissions but never
    /// accumulate them, so the whole-pass set equality must notice the missing
    /// population.
    SuppressEmissionAccumulation,
    /// `D3` affine seam: claim the same causal token twice.
    ClaimTokenTwice,
    /// `D3` owner seam: claim under a producer owner that does not own the
    /// token.
    ClaimUnderWrongOwner,
}

#[cfg(test)]
fn set_continuation_emission_mutation(mutation: ContinuationEmissionMutation) {
    CONTINUATION_EMISSION_MUTATION.with(|cell| cell.set(mutation));
}

#[cfg(test)]
thread_local! {
    static CONTINUATION_EMISSION_MUTATION: std::cell::Cell<ContinuationEmissionMutation> =
        const { std::cell::Cell::new(ContinuationEmissionMutation::Exact) };
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D3b` — the CONSUMER mutations.**
///
/// ⭐ `D4a`'s mutations proved the *instrument*: that the nearest-alias slot and
/// the locator slot hold different operands. These prove the *consumer*: that
/// the production emission seam refuses when it reads the wrong one. The
/// Architect's gate `evt_65xkzqppdqdaj` requires both, and passing the first
/// does not discharge the second.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D3bConsumerMutation {
    Exact,
    /// Consume the locator's scope-relative introduction index instead of the
    /// projection's nearest-alias index — the exact defect `D2b` reopened `D2` for,
    /// now expressed at the consumption boundary.
    ConsumeLocatorIndex,
    /// Move the resolved slot by one. ⛔ Distinct from the above rather than
    /// redundant with it: on a fixture whose locator and nearest-alias indices are
    /// adjacent the two coincide, but this one also perturbs an emission with a
    /// single producer-local input, where no collision exists to catch it.
    ShiftProducerLocalSlot,
}

/// **`D4b`** — how the generated-frame consumer's identity revalidation is
/// perturbed on the BEHAVIOURAL path. Test-only.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D4bFrameMutation {
    Exact,
    /// Present a claimed `ContinuationContextId` that is not the one this
    /// claim's own `(specialization, worker body)` key resolves to.
    WrongClaimedContext,
}

#[cfg(test)]
thread_local! {
    static D4B_FRAME_MUTATION: std::cell::Cell<D4bFrameMutation> =
        const { std::cell::Cell::new(D4bFrameMutation::Exact) };
    /// How many times the GENERATED-frame arm of `verify_entry_frame` was
    /// actually taken. ⛔ The non-vacuity counter: a behavioural control that
    /// only asserts a successful compile cannot tell "the generated route ran"
    /// from "the fixture never took it".
    static D4B_GENERATED_FRAME_CONSUMPTIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn set_d4b_frame_mutation(mutation: D4bFrameMutation) {
    D4B_FRAME_MUTATION.with(|cell| cell.set(mutation));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d4b_frame_mutation() -> D4bFrameMutation {
    D4B_FRAME_MUTATION.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d4b_generated_frame_consumption() {
    D4B_GENERATED_FRAME_CONSUMPTIONS.with(|cell| cell.set(cell.get() + 1));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d4b_generated_frame_consumptions() -> usize {
    D4B_GENERATED_FRAME_CONSUMPTIONS.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d4b_generated_frame_consumptions() {
    D4B_GENERATED_FRAME_CONSUMPTIONS.with(|cell| cell.set(0));
}

#[cfg(test)]
thread_local! {
    static D3B_CONSUMER_MUTATION: std::cell::Cell<D3bConsumerMutation> =
        const { std::cell::Cell::new(D3bConsumerMutation::Exact) };
    static D3B_CONSUMER_APPLIED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn set_d3b_consumer_mutation(mutation: D3bConsumerMutation) {
    D3B_CONSUMER_MUTATION.with(|cell| cell.set(mutation));
    D3B_CONSUMER_APPLIED.with(|cell| cell.set(0));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d3b_consumer_mutation() -> D3bConsumerMutation {
    D3B_CONSUMER_MUTATION.with(std::cell::Cell::get)
}

/// ⛔ A mutation that never fired is not a control. This counter is what lets a
/// row assert the perturbation actually reached the seam, rather than passing
/// because nothing happened.
#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d3b_consumer_application() {
    D3B_CONSUMER_APPLIED.with(|cell| cell.set(cell.get() + 1));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d3b_consumer_applications() -> usize {
    D3B_CONSUMER_APPLIED.with(std::cell::Cell::get)
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D4a` — the lowering-side operand observatory.**
///
/// ⭐ **This is an INSTRUMENT, not a mechanism.** It is `#[cfg(test)]` in every
/// part; production compiles as if it did not exist, and nothing here is
/// consulted by any lowering decision. Its whole job is to answer one question
/// the planner cannot be asked without becoming its own oracle: *which actual
/// operand does the emitting context's environment hold at the nearest-alias
/// index, and is it the operand lowering built for that exact binding?*
///
/// ⛔ **The identity it reports is the Cranelift SSA `Value`, deliberately.**
/// A carrier, a phase, a length or a planner coordinate would all agree between
/// a correct index and a wrong one on a same-shaped decoy — which is exactly the
/// population this checkpoint's fixture supplies. The SSA word is the one thing
/// that cannot agree, so it is the only honest discriminator here.
///
/// ⛔ **No planner re-walk, no index arithmetic, no fixture-authored expected
/// index.** The creation half is keyed by the LOWERING's own occurrence id at
/// the seat where it constructs the binder; the seam half is keyed by index.
/// The two are joined by `binding_origin`, and a wrong index breaks the join.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D4aSlotSelection {
    /// Production's own answer: the projection's `nearest_alias_index`.
    Exact,
    /// `D4a` mutation 1 — consume the locator's scope-relative introduction
    /// index instead. This is the defect `D2b` reopened `D2` for, expressed at
    /// the lowering slot.
    UseLocatorIndex,
    /// `D4a` mutation 2 — read both slots, then exchange them. Distinct from
    /// mutation 1: it perturbs the *pairing* while both indices stay lawful, so
    /// it survives any repair that merely bounds-checks the index.
    SwapSlots,
}

#[cfg(test)]
thread_local! {
    /// ⛔ The observatory is **disarmed by default**, so every other test in
    /// this binary pays nothing and records nothing. Only `D4a`'s own control
    /// arms it, and it disarms again when that control takes its readings.
    static D4A_ARMED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static D4A_SLOT_SELECTION: std::cell::Cell<D4aSlotSelection> =
        const { std::cell::Cell::new(D4aSlotSelection::Exact) };
    static D4A_SEAM: std::cell::RefCell<Vec<D4aSeamObservation>> =
        const { std::cell::RefCell::new(Vec::new()) };
    static D4A_CREATED: std::cell::RefCell<Vec<(StaticOriginId, String)>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

/// One reaching producer-local continuation input, as lowering actually sees it.
#[cfg(test)]
#[derive(Clone, Debug)]
pub(in crate::cranelift_backend) struct D4aSeamObservation {
    /// The binding this input names — the join key into the creation record.
    pub(in crate::cranelift_backend) binding_origin: StaticOriginId,
    /// The projection's nearest-alias index into the emitting environment.
    pub(in crate::cranelift_backend) nearest_alias_index: u32,
    /// The locator's scope-relative introduction index.
    pub(in crate::cranelift_backend) locator_index: u32,
    /// The actual operand at the selected nearest-alias slot.
    pub(in crate::cranelift_backend) nearest_alias_operand: String,
    /// The actual operand at the locator slot — the decoy.
    pub(in crate::cranelift_backend) locator_operand: String,
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d4a_set_slot_selection(selection: D4aSlotSelection) {
    D4A_SLOT_SELECTION.with(|cell| cell.set(selection));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d4a_set_armed(armed: bool) {
    D4A_ARMED.with(|cell| cell.set(armed));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d4a_armed() -> bool {
    D4A_ARMED.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d4a_slot_selection() -> D4aSlotSelection {
    D4A_SLOT_SELECTION.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d4a_record_seam(observation: D4aSeamObservation) {
    if !d4a_armed() {
        return;
    }
    D4A_SEAM.with(|cell| cell.borrow_mut().push(observation));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d4a_take_seam() -> Vec<D4aSeamObservation> {
    D4A_SEAM.with(|cell| std::mem::take(&mut *cell.borrow_mut()))
}

/// The **binder-creation seat**: lowering has just built the operand for the
/// value bound at `origin`. ⭐ Recorded here and nowhere else, because this is
/// the only point at which the operand and the occurrence that creates it are
/// both in hand without consulting an environment index.
#[cfg(test)]
pub(in crate::cranelift_backend) fn d4a_record_created(
    origin: StaticOriginId,
    operand: String,
) {
    if !d4a_armed() {
        return;
    }
    D4A_CREATED.with(|cell| cell.borrow_mut().push((origin, operand)));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d4a_take_created() -> Vec<(StaticOriginId, String)> {
    D4A_CREATED.with(|cell| std::mem::take(&mut *cell.borrow_mut()))
}

/// A lowering-side description of one environment slot, carrying the Cranelift
/// SSA `Value` wherever the operand has one.
///
/// ⛔ Deliberately **not** a `Debug` impl on the operand types. `Lowered` has no
/// `Debug` on purpose (`RT-FNSPLIT-C1`), and adding one would be a second way to
/// read a compile-time template out of an operand. This is a test-only
/// projection to a string, and it reaches only the fields it names.
#[cfg(test)]
pub(in crate::cranelift_backend) fn d4a_describe_binding(
    binding: Option<&LoweringEnvironmentBinding>,
) -> String {
    match binding {
        None => "none".to_string(),
        Some(LoweringEnvironmentBinding::StaticWorker(..)) => "worker".to_string(),
        Some(LoweringEnvironmentBinding::Value(LoweringOperand::Carried(word))) => {
            format!("carried({:?})", word.word)
        }
        Some(LoweringEnvironmentBinding::Value(LoweringOperand::Specialized(lowered))) => {
            match lowered {
                Lowered::Int { value, .. }
                | Lowered::Bool { value, .. }
                | Lowered::ProcessExitStatus { value }
                | Lowered::CapabilityToken { value }
                | Lowered::ResourceToken { value }
                | Lowered::BorrowedNativeValue { pointer: value } => {
                    format!("specialized-scalar({value:?})")
                }
                Lowered::Constructor { constructor, .. } => {
                    format!("specialized-ctor({constructor})")
                }
                Lowered::Record { .. } => "specialized-record".to_string(),
                Lowered::Closure { .. } => "specialized-closure".to_string(),
                Lowered::Trap(..) => "specialized-trap".to_string(),
                Lowered::HostResult {
                    success,
                    ok_constructor,
                    err_constructor,
                    ..
                } => format!("specialized-hostresult({success:?},{ok_constructor},{err_constructor})"),
                Lowered::ResponseBytes(span) => {
                    let (pointer, len) = (span.pointer(), span.len());
                    format!("specialized-responsebytes({pointer:?},{len:?})")
                }
                Lowered::Bytes(bytes) => format!("specialized-bytes({bytes:?})"),
                Lowered::String(text) => format!("specialized-string({text})"),
                _ => "specialized-other".to_string(),
            }
        }
    }
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D3c` — the entry-ABI immediate-availability
/// observatory.**
///
/// ⭐ **An INSTRUMENT, not a mechanism**, on the same terms as `D4a` above:
/// `#[cfg(test)]` throughout, disarmed by default, consulted by no lowering
/// decision. ⛔ **`D3c` authorizes no production edit**, and nothing here is one.
///
/// **The question, as it was originally posed.**
/// `exact_continuation_projection`'s `RootIsImmediate` arm copied an `EntryAbi`
/// root's `source_abi_position` straight into `immediate_slot`, and the
/// emission seam then read `producer_env` at that slot. `D4a` established that
/// `producer_env` at a predeclared seat is the **current lexical environment,
/// with intervening binders prepended** — not the entry ABI operand run. So the
/// copy was only sound at zero binder depth, and every population before `D4a`
/// was at zero binder depth.
///
/// **`RootIsImmediate` is RETIRED, on parent and candidate alike.** What
/// determines direct-emission availability today is
/// `resolve_direct_emission_claim` reading `nearest_alias_index` off a
/// `ContinuationEnvironmentClaim::CurrentLexical` — an index into the emitting
/// environment. A root ABI position no longer decides it.
///
/// **`source_abi_position` is NOT retired, and this paragraph previously said
/// it was.** It remains lawful production provenance: the root component of
/// `ContinuationSourceCoordinate::EntryAbi`, consumed outside any `cfg(test)`
/// gate — the planner's exact-source re-derivation locates an owner's entry
/// slot by matching on it. What retired is the **substitution**: using that
/// root coordinate as an index into the emitter's own environment. Only the
/// substitution is `cfg(test)` mutation material, and it is the only thing this
/// observatory reconstructs. The distinction has to be stated, in both
/// directions: a reader who takes the retirement to cover the field goes
/// looking for dead production code that is load-bearing, and a reader who
/// takes the substitution as live files a production defect that does not
/// exist.
///
/// **The oracle, and why it is independent.** Production already records the
/// entry ABI operands, in ABI-position order, at unit entry: `D5a` built
/// `defining_abi_operands` from the same single slot walk that seeds the entry
/// environment, so "index `i` is ABI position `i`" holds there by construction.
/// `RT-SRCBODY-BIND-ORDER` `D1`: that walk now yields TWO orders, and this
/// record keeps the descriptor one. The environment a source body is lowered
/// against is `reverse(Parameter run) ++ Capture run`, so a comparison between
/// the two must go through the derived mapping recorded beside each seat, not
/// through a shared index.
/// That record is keyed by ABI position and never by an environment index, so
/// comparing it against the emission environment **at the position derived from
/// the descriptor** is a comparison of two independently-derived answers to
/// "which value is this", not a walk checked against itself. Reading the
/// environment at `source_abi_position` directly is the retired substitution,
/// and it is present here only as the mutation below.
///
/// ⛔ **The identity reported is the Cranelift SSA `Value`**, for `D4a`'s reason:
/// the carrier, the phase and the lowering shape all agree between the entry
/// parameter and the local binding that displaces it, so only the SSA word can
/// discriminate.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D3cPositionSelection {
    /// Read the emission environment where the entry value **actually is**,
    /// located by operand identity against the entry oracle.
    MeasuredImmediate,
    /// `D3c` mutation — read the emission environment at `source_abi_position`
    /// instead. This reconstructs the **retired** `RootIsImmediate`
    /// substitution; it is not what production does today, and the name
    /// describes the retired shape rather than a live one. This is the
    /// Architect's condition 4: the substitution must flip the observed
    /// operand, and flip it because the two positions hold **different
    /// values**, not because one is out of bounds or of a different shape.
    SourceAbiPosition,
}

#[cfg(test)]
thread_local! {
    /// ⛔ Disarmed by default. Only `D3c`'s own control arms it.
    static D3C_ARMED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static D3C_POSITION_SELECTION: std::cell::Cell<D3cPositionSelection> =
        const { std::cell::Cell::new(D3cPositionSelection::MeasuredImmediate) };
    static D3C_SEAT: std::cell::RefCell<Vec<D3cSeatObservation>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

/// One entry-ABI continuation input at one real predeclared emission seat.
///
/// ⭐ The row is deliberately **raw**: it carries the whole emission environment
/// and the entry operand, and does no locating of its own beyond the selection
/// the mutation drives. The control re-derives the measured position from these
/// fields, so the instrument cannot quietly become the oracle.
#[cfg(test)]
#[derive(Clone, Debug)]
pub(in crate::cranelift_backend) struct D3cSeatObservation {
    /// How many inputs of this seat's required vector carry an `EntryAbi` root.
    pub(in crate::cranelift_backend) entry_abi_inputs: usize,
    /// How many carry a `ProducerLocal` root — the Architect's condition 2 needs
    /// at least one of each **in the same vector**.
    pub(in crate::cranelift_backend) producer_local_inputs: usize,
    /// This input's root ABI position, read from
    /// `ContinuationSourceCoordinate::EntryAbi`. Production still carries and
    /// consumes it there as root provenance. What the retired `RootIsImmediate`
    /// arm did **in addition**, and no longer does, was copy it into
    /// `immediate_slot` and index the emitter's environment with it.
    pub(in crate::cranelift_backend) source_abi_position: u32,
    /// Production's own entry-walk record at that ABI position — the oracle.
    pub(in crate::cranelift_backend) entry_operand: String,
    /// Length of the entry ABI operand run.
    pub(in crate::cranelift_backend) abi_operands: usize,
    /// **`RT-SRCBODY-BIND-ORDER` `D3c`** -- the DESCRIPTOR's kind at
    /// `source_abi_position`, and the length of the descriptor's `Parameter`
    /// run. Read from `defining_abi_slot_kinds`, never from the environment.
    ///
    /// These two are what let the control derive the exact semantic position
    /// an ABI position maps to. Without them the only available check is
    /// membership, which every unique-operand permutation satisfies and which
    /// therefore cannot tell the intended conversion from arbitrary
    /// misalignment. `None` when the position is outside the recorded run,
    /// which is itself a finding rather than a reason to fall back to a search.
    pub(in crate::cranelift_backend) source_slot_kind: Option<AbiSlotKind>,
    pub(in crate::cranelift_backend) source_parameter_run: usize,
    /// The whole emission-seat environment, in order.
    pub(in crate::cranelift_backend) emission_environment: Vec<String>,
    /// The position this instrument read, under the active selection.
    pub(in crate::cranelift_backend) observed_position: Option<u32>,
    /// The operand found there.
    pub(in crate::cranelift_backend) observed_operand: String,
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d3c_set_armed(armed: bool) {
    D3C_ARMED.with(|cell| cell.set(armed));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d3c_armed() -> bool {
    D3C_ARMED.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d3c_set_position_selection(selection: D3cPositionSelection) {
    D3C_POSITION_SELECTION.with(|cell| cell.set(selection));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d3c_position_selection() -> D3cPositionSelection {
    D3C_POSITION_SELECTION.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d3c_record_seat(observation: D3cSeatObservation) {
    if !d3c_armed() {
        return;
    }
    D3C_SEAT.with(|cell| cell.borrow_mut().push(observation));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d3c_take_seat() -> Vec<D3cSeatObservation> {
    D3C_SEAT.with(|cell| std::mem::take(&mut *cell.borrow_mut()))
}

/// **`AC-5` -- the two executable mutation controls for the static-worker
/// substrate.**
///
/// These perturb **production resolution**, at the two seams the substrate
/// actually depends on, and they are committed rather than applied by hand so
/// the proof does not evaporate when a reviewer closes their terminal. Both
/// arms are `#[cfg(test)]`; production compiles as if they did not exist.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StaticWorkerMutation {
    Exact,
    /// `D2` seam: restore the pre-node carried-capture narrowing in
    /// `lower_binder`, so a carried capture again goes to the specialized-only
    /// fold instead of installing a `StaticWorker`.
    RestoreCarriedCaptureNarrowing,
    /// `D4` transport seam: hand the consumer a **distinct** already-resolved
    /// target from this function's own `worker_calls`, selected by `AC-6`'s
    /// same-shape definition -- same declared arity, same capture count -- and
    /// by nothing else.
    ///
    /// `AC-5` clause (c): this switch runs on the **two-same-shape-worker
    /// program and only there**. The ordinary single-worker witness has no
    /// distinct same-shape candidate at all, which is a property of that
    /// fixture's population; it discharges the carried-capture arm and nothing
    /// more. Widening the predicate back toward mere difference to make it
    /// find something there is clause (b)'s defect reintroduced as a repair.
    ///
    /// The binding and its construction are untouched -- this is a redirect
    /// performed *after* the worker was constructed, which is the only place
    /// it proves anything.
    RedirectResolvedWorkerTarget,
}

#[cfg(test)]
fn set_static_worker_mutation(mutation: StaticWorkerMutation) {
    STATIC_WORKER_MUTATION.with(|cell| cell.set(mutation));
}

#[cfg(test)]
thread_local! {
    static STATIC_WORKER_MUTATION: std::cell::Cell<StaticWorkerMutation> =
        const { std::cell::Cell::new(StaticWorkerMutation::Exact) };
    static TRAP_FRAME_BINDING_MUTATION: std::cell::Cell<TrapFrameBindingMutation> =
        const { std::cell::Cell::new(TrapFrameBindingMutation::Exact) };
}



/// **`RT-DECL-CLOSURE-PORT` `D5a` — the outcome-complete localization trace.**
///
/// ⛔⛔ **Outcome-complete on purpose.** The previous localization instrumented
/// only `claim_and_call_continuation`'s two EARLY RETURNS and read zero hits as
/// "the helper is never entered". That inference is wrong: a *successful*
/// claim-and-call reaches neither early return, so zero is equally consistent
/// with the claim succeeding. A negative check passes for any reason, and that
/// one had no positive control. ⇒ Every path through the instrumented sites
/// records a terminal outcome here, so silence in the trace means the site was
/// not reached and nothing else.
#[cfg(test)]
thread_local! {
    static D5A_TRACE: std::cell::RefCell<Vec<String>> = const {
        std::cell::RefCell::new(Vec::new())
    };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d5a_trace(entry: String) {
    D5A_TRACE.with(|trace| trace.borrow_mut().push(entry));
}

/// **`RT-DECL-CLOSURE-PORT` `D5a` — the checked-IH marker's ORDERED event log.**
///
/// ⭐ Two facts have to be separable and one of them is an **ordering**: the
/// marker is consumed *at the call edge, before emission*. A pair of counters
/// cannot say that -- both are `1` whether the consumption happened before or
/// after the call was written. One ordered log can, so this is one log.
///
/// ⚠ Accumulates across a thread; read it through [`d5a_marker_events`] after
/// [`reset_d5a_marker_events`], or an earlier compile's events are attributed
/// to this one.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D5aMarkerEvent {
    /// A checked-IH marker was consumed at the exact static-worker call edge,
    /// carrying every identity that had to agree for it to be consumed.
    Consumed {
        call_template_id: u64,
        slot_template_id: u64,
        binder_index: u64,
        arity: u64,
    },
    /// A static worker call instruction was actually written.
    ///
    /// `raw_operands` is the run the raw worker's own contract accounts for —
    /// source arguments plus stored captures — and `supplied_operands` is what
    /// the call actually carried. ⭐ Recording both separately is what makes the
    /// generated-context capture suffix a **measurable prefix relation** rather
    /// than a claim: a retargeted call is the raw run plus the enclosing
    /// frame's continuation inputs, and an unretargeted one is the raw run
    /// exactly. One total would conflate "no suffix" with "a suffix of zero".
    WorkerCallEmitted {
        body_origin: StaticOriginId,
        raw_operands: usize,
        supplied_operands: usize,
        /// **`RT-CONTSRC-PRODUCER-LOCAL` `D6b`** — the route this call was
        /// emitted on.
        ///
        /// ⭐ Recorded beside the operand counts because together they are the
        /// **call semantics**, and `D6b`'s claim is about the pair. `D6a` binds
        /// two workers over one `body_origin`, so the three pre-`D6b` fields
        /// cannot say which binding an event belongs to — two events for one
        /// body are indistinguishable without this. With it, "the raw route
        /// appends nothing and the context route appends the suffix" is one
        /// readable relation per event rather than an inference across events.
        route: StaticWorkerCallRoute,
    },
}

#[cfg(test)]
thread_local! {
    static D5A_MARKER_EVENTS: std::cell::RefCell<Vec<D5aMarkerEvent>> = const {
        std::cell::RefCell::new(Vec::new())
    };
    static D5A_MARKER_MUTATION: std::cell::Cell<D5aMarkerMutation> =
        const { std::cell::Cell::new(D5aMarkerMutation::Exact) };
}

/// **`RT-DECL-CLOSURE-PORT` `D5a`** — the two causal mutations on the checked-IH
/// marker seam. Each defeats exactly one claim the seam makes, and each is a
/// perturbation the plan alone cannot express.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D5aMarkerMutation {
    Exact,
    /// Skip the consumption without an error, leaving the call itself lawful
    /// and emitted. ⛔ The only way to ask whether the positive route *depends*
    /// on the consumption — every plan-level perturbation refuses at the
    /// consumer instead of reaching closeout.
    SuppressConsumption,
    /// Skip the ENTRY arity check only.
    ///
    /// ⚠ Without this, the consumer's own arity guard is **unreachable on this
    /// witness**: the marker wraps the very call that reaches the consumer, so
    /// entry and the consumer read the same two numbers and entry always
    /// refuses first. The guard is still ruled and still load-bearing where a
    /// marker's wrapped call is not the one that reaches a static worker — this
    /// switch is what lets a control red it rather than leave it asserted.
    RelaxEntryArity,
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn with_d5a_marker_mutation<T>(
    mutation: D5aMarkerMutation,
    body: impl FnOnce() -> T,
) -> T {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            D5A_MARKER_MUTATION.with(|cell| cell.set(D5aMarkerMutation::Exact));
        }
    }
    D5A_MARKER_MUTATION.with(|cell| cell.set(mutation));
    let _restore = Restore;
    body()
}

/// **`RT-DECL-CLOSURE-PORT` `D5a` checkpoint 4 step 3 — the REACHING mutations
/// for the ruled discriminators.**
///
/// ⭐⭐ Every variant here exists because the route is now **positive**. Before
/// checkpoint 4 the only fixture that reaches these seats refused further along,
/// so a control written then would have compared a red against a red and passed
/// for the wrong reason — which is why the frame forbids red-versus-red
/// evidence and why the detached seat carried an explicit *"UNEXERCISED
/// GUARDS — do not read these as tested"* block. Each variant below moves the
/// green compile to one named refusal.
///
/// ⛔ Each perturbs **what a seat is handed**, never the guard that inspects it.
/// A mutation of the guard would ask whether the guard agrees with itself.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D5aRouteMutation {
    Exact,
    // ── the detached-result seat's five formerly unexercised guards ─────────
    /// Present the one residual causal edge twice, so the seat sees a
    /// multi-member projection onto one unit result.
    DuplicateResidualEdge,
    /// Hand the seat a carried word instead of the specialized constructor the
    /// planned producer edge requires.
    CarryNonConstructorResult,
    /// Strip the lowered constructor's synthesized identity, so the result is
    /// no longer the planner's own constructor for that edge's construct
    /// origin.
    StripLoweredConstructorIdentity,
    /// Move the ruled recursive position past the planned constructor's field
    /// run.
    PerturbRecursivePosition,
    /// Perturb the continuation's declared ordinary-parameter count, so the
    /// field run and the declared run no longer differ by exactly the one
    /// omitted recursive field.
    PerturbOrdinaryParameterCount,
    // ── the generated-context binding (checkpoint 4 step 1) ────────────────
    /// Withhold the retarget's context lookup, leaving the specialization to
    /// call the raw worker unit that checkpoint 4 step 2 removed from the
    /// executable population.
    SuppressContextBinding,
    /// Bind a context the *enclosing specialization does not own* — the
    /// transplant the ruling's key exists to prevent.
    TransplantContextBinding,
    /// Make the identity-and-body key resolve TWICE, so the lookup sees two
    /// contexts claiming one specialization and worker body.
    ///
    /// ⚠ Applied inside the lookup because the planner derives its context
    /// population deterministically and interns on that same key: a duplicate
    /// is unreachable through any plan the planner will build, so this
    /// perturbs the guard's INPUT — the population it walks — which is the
    /// only way to ask the question at all.
    DuplicateContextBinding,
    // ── the capture projection: root provenance versus immediate slot ──────
    /// Index the emitting environment with the ROOT ABI position instead of the
    /// immediate slot. ⭐ On a specialization-owned edge those are different
    /// environments, which is the whole reason the two coordinates are kept
    /// apart.
    ///
    /// ⚠ Read the row that uses this before trusting it: on the witness both
    /// coordinates are IN RANGE, so the swap binds different operands without
    /// any refusal. That is a measured limit of the evidence, not a defence.
    ReadRootPositionAsImmediateSlot,
    /// Push the immediate slot one past the emitting environment, so the bounds
    /// guard that makes the planner's resolution *answerable here* has to fire.
    PerturbImmediateSlotOutOfRange,
    /// Break the predeclared emitter's consistency law: for an owner that IS
    /// its inputs' root provenance, move the immediate slot off the root ABI
    /// position it must equal.
    PerturbPredeclaredImmediateSlot,
    /// `RT-CONTSRC-PRODUCER-LOCAL` `D1` — present a **producer-local**
    /// coordinate at both emission seams.
    ///
    /// ⚠ Perturbs the seam's INPUT, deliberately: `D1` represents the
    /// producer-local domain but nothing constructs one yet, so no plan the
    /// planner will build reaches these arms. Without this the two refusals are
    /// unmeasured code, which is indistinguishable from absent code.
    PresentProducerLocalCoordinate,
    // ── the generated-context capture suffix ──────────────────────────────
    // ── the carried-invocation binding's retained source coordinates ───────
    /// Perturb the recursive position the carried invocation presents, so the
    /// lookup is asked for a coordinate the planner never issued.
    PerturbCarriedInvocationCoordinates,
    // ── the raw worker's unchanged descriptor authority ────────────────────
    /// Drop the superseded bodies from the raw template population, keeping
    /// only the executable ones — the "template-only means deleted" reading
    /// checkpoint 1 exists to refuse.
    DropSupersededWorkerTemplates,
    // ── the one cross-pass causal ledger's lifetime ────────────────────────
    /// Close the ledger after the FIRST definition pass, before any generated
    /// `Function` exists. ⭐ This is the checkpoint-2 defect reproduced
    /// deliberately: the equality is unchanged and only the window it is taken
    /// over moves, which is what makes the refusal attributable to the
    /// lifetime.
    CloseLedgerAfterTheFirstPass,
}

#[cfg(test)]
thread_local! {
    static D5A_ROUTE_MUTATION: std::cell::Cell<D5aRouteMutation> =
        const { std::cell::Cell::new(D5aRouteMutation::Exact) };
    /// How many times the live mutation actually **fired**.
    ///
    /// ⛔ A mutation whose precondition never holds on this fixture is
    /// indistinguishable from an inert one: the compile stays green and the
    /// control reads as "the mechanism defended itself". Every seat that can
    /// decline to apply its mutation bumps this, so a control can require that
    /// its perturbation reached the code at all.
    static D5A_ROUTE_APPLICATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d5a_route_mutation() -> D5aRouteMutation {
    D5A_ROUTE_MUTATION.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d5a_route_application() {
    D5A_ROUTE_APPLICATIONS.with(|cell| cell.set(cell.get() + 1));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d5a_route_applications() -> usize {
    D5A_ROUTE_APPLICATIONS.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn with_d5a_route_mutation<T>(
    mutation: D5aRouteMutation,
    body: impl FnOnce() -> T,
) -> T {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            D5A_ROUTE_MUTATION.with(|cell| cell.set(D5aRouteMutation::Exact));
        }
    }
    D5A_ROUTE_MUTATION.with(|cell| cell.set(mutation));
    D5A_ROUTE_APPLICATIONS.with(|cell| cell.set(0));
    let _restore = Restore;
    body()
}

/// **`RT-DECL-CLOSURE-PORT` `D6a` upstream — what the routed answer did.**
///
/// ⭐ The route is a **predecessor-edge fact**, so every control over it has to
/// name an *edge*: which producer raised it, and which consumer received it.
/// A control that could only see the consumer's final value would be unable to
/// distinguish the two producers, and the whole point of this checkpoint is
/// that they are distinct authorities.
///
/// ⛔ These carry the exact planner identities they observed, never counts —
/// *"planned, claimed and emitted call identity agree on the exact identity,
/// not merely on counts"* is one of the discriminators this checkpoint owes.
#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::cranelift_backend) enum D6aRouteEvent {
    /// **PRODUCER 2.** The result of an exactly claimed and emitted
    /// continuation-specialization call was raised to
    /// `CheckedSelectedRecursor`. Recorded only after the owner/affine claim
    /// succeeded and the emitted callee was checked against
    /// `identity.target()`, so `target` is the authority's own identity and
    /// not a guess about it.
    CallResultRaised {
        target: ContinuationSpecializationId,
    },
    /// **PRODUCER 1.** An exact recursor layer supplied a route.
    ///
    /// ⚠ Both answers are recorded, and `selects_occurrence` is recorded beside
    /// the answer, so a reader can see the decision rather than infer it. ⛔ On
    /// the governed witness this producer only ever fires **positively**
    /// (`checked_frame_id: Some(7)`, `selects_occurrence: true`), so the
    /// negative arm is not exercised here — the mutation
    /// [`D6aRouteMutation::DropRecursorLayerRoute`] is what supplies a
    /// `DirectScrutinee` answer at this seat.
    RecursorLayerSupplied {
        checked_frame_id: Option<u64>,
        selects_occurrence: bool,
        route: SourceComputationalAnswerRoute,
    },
    /// **THE CONSUMER.** The carried computational-match seat, with the route
    /// its predecessor handed it, the frame's own recursor-layer field, and
    /// the join of the two.
    ///
    /// ⚠ All three are recorded because the join is the mechanism: `incoming`
    /// alone cannot show that the frame's field failed to overwrite it, which
    /// is exactly the drop measured at `ae45e804`.
    ConsumerRoute {
        seat: D6aConsumerSeat,
        static_origin: StaticOriginId,
        incoming: SourceComputationalAnswerRoute,
        frame_field: SourceComputationalAnswerRoute,
        joined: SourceComputationalAnswerRoute,
    },
    /// The carried elimination this consumer handed its frame to was actually
    /// entered. ⭐ Distinct from `ConsumerRoute`, which records only that a
    /// seat computed a route — a seat can do that on a path the eliminator
    /// never runs, and a control that conflated the two would credit a route
    /// with an emission it had nothing to do with.
    CarriedEliminationEntered {
        static_origin: StaticOriginId,
        route: SourceComputationalAnswerRoute,
        cases: usize,
    },
    /// The carried checked-answer fallback was actually emitted, at this
    /// consumer. ⭐ Recorded into the *route* trace as well as the trap
    /// provenance so that one ordered sequence carries the whole edge —
    /// producer, consumer, emission — and a row does not have to correlate two
    /// traces to say which consumer acted.
    CarriedFallbackEmitted {
        static_origin: StaticOriginId,
    },
    /// The carried consumer took its closed default instead. ⛔ Recorded for
    /// **every** reason it can do so, including a `DirectScrutinee` route, so
    /// the trace can never show a consumer that neither emitted nor defaulted.
    CarriedDefaultSealed {
        static_origin: StaticOriginId,
        route: SourceComputationalAnswerRoute,
    },
}

/// Which lowering edge a carried computational-match consumer sits on.
///
/// ⚠ Both edges exist on the governed witness and they arrive at the **same**
/// `StaticOriginId`, which is why the seat has to be recorded: an assertion
/// keyed on the origin alone cannot say which predecessor it is about, and the
/// whole premise of this checkpoint is that the origin does not determine the
/// route.
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::cranelift_backend) enum D6aConsumerSeat {
    /// The composed eliminator edge in `lower_computational_match_value_composed`.
    Composed,
    /// The source machine's computational-scrutinee edge.
    SourceMachine,
}

/// Perturbations of the `D6a` upstream transport, **one producer at a time**.
///
/// ⭐ Separating them is the requirement, not a convenience: the frame owes a
/// control showing the recursor-layer producer *stays green independently* of
/// the call-result producer, and a mutation that disabled both could not
/// distinguish "independent" from "jointly dead".
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::cranelift_backend) enum D6aRouteMutation {
    Exact,
    /// Drop **only** producer 2: the exact claimed call result returns
    /// `DirectScrutinee`. ⛔ The recursor-layer producer is untouched.
    DropCallResultRoute,
    /// Drop **only** producer 1: an exact selecting recursor layer supplies
    /// `DirectScrutinee`. ⛔ The call-result producer is untouched.
    DropRecursorLayerRoute,
    /// Let the frame's own field **overwrite** the incoming route instead of
    /// joining with it. ⚠ This reproduces the exact defect measured at
    /// `ae45e804`, and it is the control that the join is load-bearing.
    OverwriteIncomingWithFrameField,
}

#[cfg(test)]
thread_local! {
    static D6A_ROUTE_MUTATION: std::cell::Cell<D6aRouteMutation> =
        const { std::cell::Cell::new(D6aRouteMutation::Exact) };
    static D6A_ROUTE_TRACE: std::cell::RefCell<Vec<D6aRouteEvent>> =
        const { std::cell::RefCell::new(Vec::new()) };
    /// How many times the live `D6a` mutation actually fired — the same
    /// inertness guard `D5A_ROUTE_APPLICATIONS` provides, for the same reason.
    static D6A_ROUTE_APPLICATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d6a_route_mutation() -> D6aRouteMutation {
    D6A_ROUTE_MUTATION.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d6a_route_application() {
    D6A_ROUTE_APPLICATIONS.with(|cell| cell.set(cell.get() + 1));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d6a_route_applications() -> usize {
    D6A_ROUTE_APPLICATIONS.with(std::cell::Cell::get)
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2k-1a` — the owner discriminator, and it
/// is deliberately independent of the refusal message.**
///
/// `D2k-0` established that the five walls share one edge, but the edge is
/// parsed out of the refusal text and `mod.rs`'s environment-value read
/// **forwards its caller's edge** — so five distinct routes converging through
/// that forward would present identically. Sameness is the claim, and a
/// discriminator used to establish identity has to be able to distinguish.
///
/// These events are what distinguishes: the **enclosing `lower_expr` arm**, and
/// **which `value_at` caller** actually ran. Neither reads the refusal string.
///
/// **`value_at` itself is not instrumented and must stay byte-identical**
/// (`AC-2`). Every tag sits on a *caller* or on an enclosing arm, which is why
/// this discriminator is constructible at all.
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::cranelift_backend) enum GeneratedUnitCallInputCaller {
    StaticMatchCaseParameter,
    StaticMatchCaseCapture,
    SourceClosureArgument,
    SourceLexicalClosureArgument,
    SourceLexicalClosureCapture,
    SourceMachineDeclaredUnit,
}

/// The planner level named by a generated-unit call-input diagnostic.
///
/// Five callers enter through a closure and therefore name its planned body.
/// The source-machine declared-unit route starts from a scheduling entry; it
/// names the same body when child zero exists, and otherwise states explicitly
/// that the entry is the only available identity. A missing diagnostic child
/// is data, never a reason for the test build to refuse compilation.
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::cranelift_backend) enum GeneratedUnitCallInputCallee {
    Body(StaticOriginId),
    Entry(StaticOriginId),
    MissingBodyChild { entry: StaticOriginId },
    MissingBodyChildByMutation { entry: StaticOriginId },
}

#[cfg(test)]
thread_local! {
    static CALL_INPUT_CALLEE_CHILD_MISSING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static CALL_INPUT_CALLEE_CHILD_MISSING_HITS: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// RAII scope for the missing diagnostic-child control.
#[cfg(test)]
pub(in crate::cranelift_backend) struct CallInputCalleeDiagnosticMutationGuard {
    previous: bool,
    previous_hits: u32,
}

#[cfg(test)]
impl CallInputCalleeDiagnosticMutationGuard {
    pub(in crate::cranelift_backend) fn install() -> Self {
        let previous = CALL_INPUT_CALLEE_CHILD_MISSING.with(|cell| cell.replace(true));
        let previous_hits = CALL_INPUT_CALLEE_CHILD_MISSING_HITS.with(|cell| cell.replace(0));
        Self {
            previous,
            previous_hits,
        }
    }

    pub(in crate::cranelift_backend) fn hits(&self) -> u32 {
        CALL_INPUT_CALLEE_CHILD_MISSING_HITS.with(std::cell::Cell::get)
    }
}

#[cfg(test)]
impl Drop for CallInputCalleeDiagnosticMutationGuard {
    fn drop(&mut self) {
        CALL_INPUT_CALLEE_CHILD_MISSING.with(|cell| cell.set(self.previous));
        CALL_INPUT_CALLEE_CHILD_MISSING_HITS.with(|cell| cell.set(self.previous_hits));
    }
}

#[cfg(test)]
fn call_input_callee_child_missing() -> bool {
    CALL_INPUT_CALLEE_CHILD_MISSING.with(std::cell::Cell::get)
}

#[cfg(test)]
fn note_call_input_callee_child_missing() {
    CALL_INPUT_CALLEE_CHILD_MISSING_HITS.with(|cell| cell.set(cell.get().saturating_add(1)));
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::cranelift_backend) enum D2kOwnerEvent {
    /// A `Construct` arm was entered, with the constructor it is building.
    ConstructEntered {
        origin: StaticOriginId,
        constructor: String,
    },
    /// A `value_at` call was reached, tagged by which caller it was.
    ValueAtCaller { site: &'static str },
    /// A `Var` in value position resolved to a static worker binding -- the
    /// read that is about to refuse -- **paired with the caller that is about
    /// to take it**.
    ///
    /// **The two facts are ONE event on purpose.** They were briefly two, and
    /// the caller was then recovered by scanning backwards from the read. That
    /// scan could only ever find an EARLIER caller event, because this path
    /// records the read first and the caller second -- so the caller it
    /// reported was a stale predecessor from an unrelated successful read, and
    /// the assertion was green without ever establishing adjacency. Pairing
    /// removes the question rather than answering it: there is no ordering to
    /// get wrong when one event carries both halves.
    StaticWorkerRead {
        origin: StaticOriginId,
        site: &'static str,
    },
    /// `D2k-1b-i` — a `Construct` argument was **recognized** as a static
    /// worker and retained as a compiler-only field, instead of being read as
    /// a value.
    ///
    /// **The owner/argument relation this carries is the PLANNER's, not
    /// emission order.** `field_origin` is
    /// `child_static_origin(owner, position)`, so a consumer of this trace
    /// learns which constructor owns the field from the planner's own
    /// positional child-origin range rather than from which `ConstructEntered`
    /// happened to be emitted nearest. That distinction is not cosmetic: a
    /// constructor argument may itself be a `Construct`, and a
    /// nearest-preceding rule names the **inner** one while agreeing with the
    /// planner on every non-nested fixture.
    StaticWorkerField {
        owner: StaticOriginId,
        position: usize,
        field_origin: StaticOriginId,
        constructor: String,
    },
    /// `D2k-1b-i` — a static `Match` elimination installed a constructor's
    /// static-worker field into the lexical binding authority **without
    /// erasing its kind**, so the existing exact-`Var` call arm can consume it.
    ///
    /// This is the consumption half of the total: every recorded
    /// [`D2kOwnerEvent::StaticWorkerField`] must be answered by one of these
    /// or by a refusal, and a row that compiles with none is a failure.
    StaticWorkerBinderInstalled {
        field_origin: StaticOriginId,
        position: usize,
    },
    /// `D2k-1b-i` — the exact-`Var` call arm consumed a static worker that
    /// reached it through a constructor field installed by a static `Match`
    /// elimination.
    StaticWorkerCallConsumed { origin: StaticOriginId },
    /// `D2k-1c-0` — a static `Match` elimination descended over a constructor's
    /// fields, tagged with **which** site and with the planner origin of the
    /// eliminating match occurrence.
    ///
    /// **THE DURABLE POPULATION IS TWO REBIND CALLERS AND SIX DESCENT SITES**:
    /// four reach [`Lowering::rebind`] through
    /// [`Lowering::bound_constructor_fields`] and two through
    /// [`Lowering::extend_constructor_fields`], both by way of the single
    /// [`Lowering::constructor_field_bindings`]. The deciding read was posed
    /// over the four `bound_constructor_fields` sites; the two
    /// `extend_constructor_fields` sites reach the same chokepoint and the
    /// witness landed on one of them, so a four-site reading of this instrument
    /// is short by exactly the sites that mattered.
    ///
    /// **This is a ROUTE instrument, not a worker one.**
    /// [`StaticWorkerBinderInstalled`] fires only for a worker field, and on
    /// today's population that is never — every row sits at zero installs
    /// behind the route gap. This one fires for every constructor field of
    /// every kind, so descent multiplicity is measurable on the rows as they
    /// stand. **Descent multiplicity is not field multiplicity**: repeated
    /// descents of one eliminating match occurrence may traverse different
    /// constructors, and rows 4 and 5 do.
    ///
    /// [`StaticWorkerBinderInstalled`]: D2kOwnerEvent::StaticWorkerBinderInstalled
    StaticMatchBinderDescent {
        /// **A stable function-route name**, e.g.
        /// `extend_constructor_fields@composed` — never a `file:line`. A line
        /// label re-aims itself at an unrelated site on any edit above it and is
        /// then green for the wrong reason, which is the defect this node
        /// already found in `AC-2`'s own control. `1c-0c`.
        site: &'static str,
        eliminated_origin: StaticOriginId,
    },
    /// `RT-REQUIRED-CONSUMER-REACH-CENSUS` `D5` -- the real one-way carrier
    /// entry, recorded before its admissibility walk can refuse.
    ///
    /// Event presence records that the crossing was reached; `closure_path`
    /// separately records whether the presented graph contains an ordinary
    /// closure. Keeping those facts separate leaves the frame's third branch
    /// representable: a crossing may be absent independently of closure shape.
    BoundaryTransferEntered {
        origin: StaticOriginId,
        root_kind: &'static str,
        closure_path: Option<String>,
        invoking_site: BoundaryTransferInvokingSite,
    },
}

#[cfg(test)]
thread_local! {
    static D2K_OWNER_TRACE: std::cell::RefCell<Vec<D2kOwnerEvent>> =
        const { std::cell::RefCell::new(Vec::new()) };
    static D2K_BOUNDARY_TRANSFER_INVOKING_SITE:
        std::cell::Cell<BoundaryTransferInvokingSite> =
        const { std::cell::Cell::new(BoundaryTransferInvokingSite::Direct) };
}

#[cfg(test)]
struct BoundaryTransferInvokingSiteGuard {
    previous: BoundaryTransferInvokingSite,
}

#[cfg(test)]
impl BoundaryTransferInvokingSiteGuard {
    fn enter(site: BoundaryTransferInvokingSite) -> Self {
        let previous = D2K_BOUNDARY_TRANSFER_INVOKING_SITE.with(|current| {
            current.replace(site)
        });
        Self { previous }
    }
}

#[cfg(test)]
impl Drop for BoundaryTransferInvokingSiteGuard {
    fn drop(&mut self) {
        D2K_BOUNDARY_TRANSFER_INVOKING_SITE.with(|current| {
            current.set(self.previous);
        });
    }
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d2k_owner_event(event: D2kOwnerEvent) {
    D2K_OWNER_TRACE.with(|trace| trace.borrow_mut().push(event));
}

/// Take and clear the trace, so one compile's events are never read as another's.
#[cfg(test)]
pub(in crate::cranelift_backend) fn d2k_owner_trace_take() -> Vec<D2kOwnerEvent> {
    D2K_OWNER_TRACE.with(|trace| std::mem::take(&mut *trace.borrow_mut()))
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d6a_route_event(event: D6aRouteEvent) {
    D6A_ROUTE_TRACE.with(|trace| trace.borrow_mut().push(event));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d6a_route_trace() -> Vec<D6aRouteEvent> {
    D6A_ROUTE_TRACE.with(|trace| trace.borrow().clone())
}

/// Runs `body` under `mutation` with a **cleared** route trace, restoring the
/// exact route on the way out even if `body` unwinds.
#[cfg(test)]
pub(in crate::cranelift_backend) fn with_d6a_route_mutation<T>(
    mutation: D6aRouteMutation,
    body: impl FnOnce() -> T,
) -> T {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            D6A_ROUTE_MUTATION.with(|cell| cell.set(D6aRouteMutation::Exact));
        }
    }
    D6A_ROUTE_MUTATION.with(|cell| cell.set(mutation));
    D6A_ROUTE_APPLICATIONS.with(|cell| cell.set(0));
    D6A_ROUTE_TRACE.with(|trace| trace.borrow_mut().clear());
    let _restore = Restore;
    body()
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d5a_marker_event(event: D5aMarkerEvent) {
    D5A_MARKER_EVENTS.with(|events| events.borrow_mut().push(event));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d5a_marker_events() {
    D5A_MARKER_EVENTS.with(|events| events.borrow_mut().clear());
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d5a_marker_events() -> Vec<D5aMarkerEvent> {
    D5A_MARKER_EVENTS.with(|events| events.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d5a_trace() {
    D5A_TRACE.with(|trace| trace.borrow_mut().clear());
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn take_d5a_trace() -> Vec<String> {
    D5A_TRACE.with(|trace| trace.borrow().clone())
}



#[cfg(test)]
fn set_trap_frame_binding_mutation(mutation: TrapFrameBindingMutation) {
    TRAP_FRAME_BINDING_MUTATION.with(|cell| cell.set(mutation));
}



struct Lowering<'a> {
    seed_env: &'a NativeSeedEnvironment,
    /// Everything resolved into the ONE generated function this `Lowering`
    /// emits into. ⛔ See [`FunctionLocalRefs`] — none of it is portable.
    function_local: FunctionLocalRefs,
    declarations: BTreeMap<&'a str, &'a RuntimeDeclaration>,
    /// The closed static plan for this compilation.
    ///
    /// It lives here, rather than as a local of `compile_expr_into_module`,
    /// because every descent needs the checked positional child-origin table to
    /// derive the child's static name, and because a retained closure body is now
    /// resolved *out of this plan* by its static origin.
    ///
    /// ⚠ **The plan borrows the source trees** — `StaticTransitionPlan<'a>` holds
    /// each planned occurrence's term by reference, which is what lets a tag be
    /// resolved back to a body without any site retaining a cloned one. So the
    /// non-escape property is now **load-bearing** rather than incidental, and it
    /// is not argued from the absence of borrows:
    ///
    /// `CompiledModule<M>` has **no lifetime parameter** and takes only owned
    /// data, so nothing borrowed can be stored in it — the compiler rejects it.
    /// `escaping_a_source_borrow_into_the_compiled_artifact_does_not_typecheck`
    /// pins exactly that, by requiring `CompiledModule: 'static`; give the
    /// artifact a borrowed field and the pin stops compiling.
    static_transition_plan: StaticTransitionPlan<'a>,
    result_table: BTreeMap<i64, RuntimeGroundValue>,
    next_token: i64,
    next_recursor_frame_provenance: u64,
    next_recursor_producer_origin: u64,
    next_continuation_activation: u64,
    next_continuation_cursor: u64,
    next_source_join: u64,
    next_source_predecessor: u64,
    live_source_continuations: usize,
    /// `RT-CARRIED-ORDINARY-COMPOSITION` `D2` — re-entry depth for continuing a
    /// composed suffix behind a carried ordinary elimination.
    ///
    /// The termination argument is lexicographic on
    /// `(active.pending.len(), eliminators.len())`: a composed re-entry leaves
    /// every pending suffix untouched and consumes one eliminator, and a resume
    /// splits `active.pending` into head plus a strictly shorter tail. So both
    /// components are non-increasing and one strictly decreases at every step.
    ///
    /// This counter does not rest on that argument. Every measured member of
    /// this node's population has a suffix of length one, so depth two was never
    /// exercised, and a termination argument that is only true is still a
    /// termination argument nobody ran. The bound fails closed instead, and is
    /// expected never to bind.
    carried_suffix_reentries: usize,
    source_control_root: Option<ContinuationCursorId>,
    active_oriented_semantic_regions: usize,
    /// ⛔⛔ **`AC-C4`'s TERMINATION GUARD — the carried computational
    /// eliminations currently being emitted, by their frame's static origin.**
    ///
    /// ⚠ **Why a guard is needed at all, and it is not a scope choice.** A
    /// *specialized* recursive elimination terminates because its residual is a
    /// compile-time value that gets strictly smaller — `Suc(Suc(Zero))` reaches
    /// `Zero`. A **carried** residual is a runtime word, so nothing shrinks at
    /// compile time: emitting the recursive case emits its induction-hypothesis
    /// invocation, which re-enters the same eliminator, which emits the
    /// recursive case again. ⇒ Inlining a carried recursion **cannot
    /// terminate**, for any recursive datatype, and without this the compiler
    /// does not error — it **hangs and then overflows its stack**.
    ///
    /// ⭐ Keyed on the frame's own `static_origin` rather than a depth count, so
    /// it refuses exactly self-resumption and ⛔ still permits a case body to
    /// eliminate a *different* carried value. A bare depth bound would refuse
    /// legitimate nesting — an over-strengthened guard manufacturing defects.
    ///
    /// ⛔⛔ **This is a TRANSITION SENTINEL, ⛔ not closure — and the successor
    /// that retires it is named.** Steward decision 2026-07-28 (`C1 §2g-i`'s
    /// amendment block): `AC-C4` splits, `C1` keeps the representation half, and
    /// **`RT-FNSPLIT-B2F` owns the runtime invocation** inside its existing
    /// atomic target/switch boundary. ⇒ `B2F` emits one closed, recursively
    /// callable target per static computational-eliminator origin and turns a
    /// zero-argument structural IH into a **direct call to that same target**;
    /// at that point this stack, and the refusal it guards, come out.
    ///
    /// ⚠ **`B2F`'s termination premise is a DIFFERENT argument from this
    /// guard's, and that is the point.** This refuses because nothing shrinks at
    /// **compile** time. `B2F` may call because the producer→validator boundary
    /// has already established a **finite acyclic carrier graph** and the call
    /// rides a **declared recursive child edge** — so its measure is strict
    /// descent in that validated graph, ⛔ never compile-time shrinkage.
    active_carried_computational_eliminations: Vec<(StaticOriginId, cranelift_codegen::ir::Block)>,
    native_join_plan: Option<crate::NativeJoinPlanV1>,
    consumed_join_sites: BTreeSet<u64>,
    root_terminal_authority: Option<RootTerminalAnswerAuthority>,
    active_join_site: Option<u64>,
    oriented_subcontinuation_plan: Option<crate::OrientedSubcontinuationPlanV1>,
    consumed_subcontinuation_frames: BTreeSet<(u64, u64)>,
    active_subcontinuation_frame: Option<u64>,
    consumed_recursive_call_templates: BTreeSet<u64>,
    pending_recursive_call: Option<CheckedRecursiveInvocationInstance>,
    pending_computational_ih_call: Option<PendingCheckedIhCall>,
    active_recursive_invocations: Vec<CheckedRecursiveInvocationInstance>,
    next_recursive_invocation_instance: u64,
    dynamic_splice_edges: BTreeMap<DynamicSpliceEdgeId, DynamicSpliceEdge>,
    next_dynamic_splice_edge: u64,
    assumptions: BTreeSet<String>,
    unsupported: Vec<String>,
    /// **`RT-CONTSPEC-ACTIVATE` `D3`** -- the affine claim ledger, held across
    /// the whole unit-definition pass so a token claimed at one producer
    /// occurrence cannot be claimed again at another.
    continuation_claims: Option<units::ContinuationClaimLedger>,
    /// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3`** — the SIBLING affine ledger for
    /// the fusion-local realizations `F`, held over the same span as
    /// `continuation_claims` and opened and closed on the same boundary.
    ///
    /// A sibling, not an arm of the ledger above. `O` and `F` are disjoint
    /// domains with different evidence: an ordinary obligation is discharged by
    /// a call decoded back out of the finished CLIF, and a fusion-local one
    /// emits no call for any such gate to read. One ledger over both would have
    /// to weaken the direct laws to tolerate a member with no instruction.
    fusion_compositions: Option<units::FusionCompositionLedger>,
    /// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2k-1b-i`** — the conservation ledger
    /// for compiler-only static-worker constructor fields. See
    /// [`StaticWorkerFieldLedger`]; closed by
    /// [`Lowering::require_complete_static_worker_disposition`].
    ///
    /// **Always present, never `Option`**, unlike the two ledgers above. Those
    /// are opened by an authority that may not run, and their absence is a real
    /// state meaning *"no claim regime here"*. This one has no such state: a
    /// recognized worker field must reach a disposition on **every** route,
    /// including a direct lowering harness that opens no unit-definition pass.
    /// An `Option` would give the drop a way back in — an unopened ledger that
    /// closes vacuously.
    static_worker_fields: StaticWorkerFieldLedger,
    /// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2f`** — the fused-region claim
    /// ledger, held across the whole unit-definition pass for the same reason
    /// the continuation ledger above is: the redirect happens while defining the
    /// consumer's Function and the takeover happens inside that same body, so a
    /// region claimed at one seat cannot be claimed again at another.
    ///
    /// A separate ledger rather than an arm of `continuation_claims`. The two
    /// are affine over **different** things — a causal call token versus a fused
    /// region — and a single ledger would make "this token was spent" and "this
    /// region was taken over" one exhaustion.
    ///
    /// `None` on a compile that never reached the emission seam; **empty rather
    /// than absent** on a compile that reached it with no fused region, which is
    /// what keeps the zero case on the same path as the non-zero one.
    fusion_claims: Option<FusionRegionClaimLedger>,
    /// **`RT-LEXICAL-R3-FUSION-EMITTER` `D1` — the per-phase authority switch's
    /// only state, and it is deliberately a KEY plus a FACT, not a mode.**
    ///
    /// Set for the extent of one fused region's body definition, to that
    /// region's `(continuation_origin, consumer_owner)`. The interior point in
    /// the composed eliminator reads it and switches authority for exactly the
    /// case body whose frame's `static_origin` equals the key.
    ///
    /// **Why a field rather than a frame member.** The fact is needed at the
    /// consumer's case-body lowering, which is reached through the shared
    /// eliminator path; carrying it on `ComputationalEliminatorFrame` would
    /// touch every construction site of that struct across the whole lowering,
    /// for a fact only the fused definition pass can supply. One scoped field
    /// keeps the ripple inside the fusion entry path.
    ///
    /// **Never `ContinuationEmissionOwner::Fusion`.** `Fusion` is region and
    /// definition identity only (Architect `evt_4vqey13cxxjqs`); the authority
    /// that runs here is the *consumer's* `Predeclared` owner, and it is
    /// restored to the producer's on the way out.
    fused_consumer_authority: Option<(StaticOriginId, PredeclaredFunctionId)>,
    continuation_candidates: Option<units::ContinuationCandidateLedger>,
    /// **`RT-DECL-CLOSURE-PORT` `D5a`** — the emission owner of the context
    /// currently being defined, in the generalized domain.
    ///
    /// ⛔ Deliberately a separate field from `defining_unit` rather than a
    /// function of it. A generated specialization context lowers a raw body, so
    /// deriving one from the other would make the raw body's predeclared owner
    /// stand in for the generated context — the exact conflation
    /// `evt_609am4v7cdt5b` ruled against.
    defining_emission_owner: Option<ContinuationEmissionOwner>,
    /// **`RT-DECL-CLOSURE-PORT` `D7`** — the exact declared function whose body
    /// is being emitted, which scopes this body's allocation EVENTS.
    ///
    /// ⛔ Evidence scope only, never planner authority. It is a separate field
    /// from `defining_unit` and `defining_emission_owner` because it answers a
    /// different question: those name who the planner says owns the emission,
    /// this names the module definition the events belong to. One
    /// `PredeclaredFunctionId` can be built as more than one `FuncId` body.
    defining_function_id: Option<FuncId>,
    /// **`RT-DECL-CLOSURE-PORT` `D7`** — the aggregate-allocation event ledger,
    /// held across the whole emission pass.
    ///
    /// `None` outside that pass, which is the only place an aggregate
    /// allocation can be attributed to a declared function.
    aggregate_allocations: Option<aggregates::AggregateAllocationLedger>,
    /// `D7` — the consumed side of the host-effect seat authority. `None`
    /// outside the emission pass, where a bare rig defines no function and there
    /// is no population to close against.
    host_effect_seats: Option<effects::EffectSeatLedger>,
    /// **`RT-DECL-CLOSURE-PORT` `D5`** — the checked-call closeout ledger.
    /// `None` outside the functionized unit-bundle pass, which is the only
    /// place a checked call can reach a declaration-owned unit.
    checked_call_ledger: Option<units::CheckedCallLedger>,
    /// The exact unit currently being defined, so `D3`'s owner check compares
    /// against a fact supplied independently of the token.
    defining_unit: Option<PredeclaredFunctionId>,
    process_object: bool,
    process_symbols: crate::NativeProcessSymbols,
    #[cfg(test)]
    native_int_mutation: NativeIntLoweringMutation,
    #[cfg(test)]
    bounded_nat_mutation: BoundedNatLoweringMutation,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RecursorFrameProvenance(u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InvocationTemplateRef {
    SameSccCall(u64),
    ComputationalIHCall(u64),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CheckedRecursiveInvocationInstance {
    source: InvocationTemplateRef,
    invocation_instance_id: u64,
    semantic_depth: usize,
    dynamic_splice_edge: Option<DynamicSpliceEdgeId>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DynamicSpliceEdgeId(u64);
/// The unique compiler-owned authority to splice one completed dynamic child
/// invocation into one exact open parent occurrence. Lowered values retain
/// only the inert `DynamicSpliceEdgeId`; this non-`Clone` ledger entry is
/// removed and consumed before any CFG is emitted.
struct DynamicSpliceEdge {
    edge_id: DynamicSpliceEdgeId,
    child_invocation_instance_id: u64,
    parent_invocation_instance_id: u64,
    checked_call_template_id: u64,
    parent_frame_template_id: u64,
    segment_site_id: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ContinuationActivationId(u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ContinuationCursorId(u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RecursorProducerOriginId(u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RecursorLayerRole {
    SelectsOccurrence {
        origin: RecursorProducerOriginId,
    },
    ExitsScope {
        origin: RecursorProducerOriginId,
        scope_origin: RecursorProducerOriginId,
        parent_scope: Option<RecursorProducerOriginId>,
    },
}
#[derive(Clone)]
struct ComputationalRecursorFramePayload {
    cases: Vec<crate::RuntimeComputationalMatchCase>,
    default: RuntimeTrap,
    outer_env: Vec<LoweringEnvironmentBinding>,
    /// The origin of the computational-match occurrence these cases came from,
    /// cloned into this payload **in the same constructor as the cases** so a
    /// later resumption can still derive a case body's origin positionally
    ///.
    static_origin: StaticOriginId,
    provenance: RecursorFrameProvenance,
    checked_frame_id: Option<u64>,
    checked_invocation_id: Option<u64>,
    checked_invocation_source: Option<InvocationTemplateRef>,
    checked_invocation_depth: usize,
}
#[derive(Clone)]
struct OwnedSelectedScope {
    scope_origin: RecursorProducerOriginId,
    parent_scope: Option<RecursorProducerOriginId>,
    frame: ComputationalRecursorFramePayload,
}
#[derive(Clone, Copy)]
struct NativeScalarPairV1 {
    tag: cranelift_codegen::ir::Value,
    payload: cranelift_codegen::ir::Value,
}
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NativeIntLoweringMutation {
    Exact,
    Wrapping,
    Trap,
    SuppressTerminalExport,
    CorruptTerminalExport,
}
/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2k-1b-i0` — one field of a specialized
/// [`Lowered::Constructor`] template.**
///
/// A specialized constructor is a **compiler template, not necessarily a
/// materialized runtime aggregate**. Its fields used to be `Vec<Lowered>`,
/// which baked in the assumption that every statically eliminated field is an
/// ordinary value. The five `#6d` walls measured by `D2k-0` are exactly the
/// population that assumption excludes: a lexical static worker reaching a
/// constructor argument, which is a value-producing position **by
/// construction** and therefore refuses at
/// [`LoweringEnvironmentBinding::value_at`].
///
/// This sum is the closed compiler-only distinction the Architect ruled
/// (`evt_4krvq67427n5z`): **an ordinary specialized field versus a static-worker
/// field**, and nothing wider. It is narrower than [`LoweringOperand`] on
/// purpose — a `LoweringEnvironmentBinding`-as-payload arm would admit
/// `Carried` values into a template and widen the contract past the measured
/// need, which the ruling forbids.
///
/// **There is deliberately no wildcard or default conversion.** A future
/// third arm must be a compile error at every field reader, exactly as
/// `value_at`'s own exhaustive match is an asset rather than an obstacle.
#[derive(Clone)]
enum ConstructorField {
    /// An ordinary specialized field: a value this constructor materializes.
    /// Every field the lowering engines build today is this arm.
    Specialized(Lowered),
    /// A statically-bound worker transported through the constructor template
    /// without becoming a value.
    ///
    /// **THIS ARM IS NOW CONSTRUCTED**, at the one compiler-only template:
    /// `Lowering::static_worker_constructor_template`, reached from both the
    /// direct-descent and source-machine `RuntimeExpr::Construct` arms after
    /// `recognized_constructor_worker_fields` answers ahead of any
    /// value-producing read. The `never constructed` warning that stood here
    /// as `D2k-1b-i0`'s open checkpoint is gone, and its disappearance is the
    /// compiler's own announcement that the checkpoint closed.
    ///
    /// **It is also READ**, by the kind-preserving static `Match` binder
    /// [`bound_constructor_fields`], which installs it into the one lexical
    /// binding authority as [`LoweringEnvironmentBinding::StaticWorker`] rather
    /// than converting it to a value — so the pre-existing exact-`Var` call arm
    /// is the thing that consumes it. Both `1b-i0` warnings (`never
    /// constructed`, then `field 0 is never read`) are therefore gone, and
    /// neither was silenced with an `#[allow(dead_code)]`.
    ///
    /// > **MEASURED, and it is why arming the producer ALONE does not land:**
    /// > with this arm constructed and nothing consuming it, four of `D2k`'s
    /// > five expressions stop refusing and **compile with the worker silently
    /// > dropped** (`739cfde3`, preserved as evidence). That is the forbidden
    /// > fourth state — constructed, neither consumed nor authoritatively
    /// > erased, then forgotten — and it is what
    /// > [`StaticWorkerFieldLedger`] exists to make impossible.
    ///
    /// **The field readers' arms below are TYPE COMPLETENESS, NOT the
    /// boundary.** They are local refusals reached *during* descent, and the
    /// ruling requires a **whole-graph** refusal *before the first allocation
    /// or emitted transfer*. That boundary is
    /// `Lowering::source_aggregate_preflight` together with
    /// [`Lowered::boundary_transfer_admissibility`], both of which run ahead of
    /// every emission; a count of green reader arms is never evidence about it,
    /// because the two differ in **when**, not in **how many**.
    StaticWorker {
        binding: StaticWorkerBinding,
        /// **The planner-owned pairing key**, `child_static_origin(owner,
        /// position)` at the producing `Construct`.
        ///
        /// Architect `evt_5etamwj8tp2fh` requires the recognized field to be
        /// paired to its later static elimination **by planner origin/position
        /// — never by constructor spelling and never by trace proximity.** A
        /// constructor argument may itself be a `Construct`, so a
        /// nearest-preceding rule names the inner constructor and agrees with
        /// the planner on every non-nested fixture. Carrying the key on the
        /// field is what lets the elimination and the conservation ledger name
        /// the same occurrence without either of them inferring the relation.
        ///
        /// **Provenance, not identity.** It names the *occurrence*, and one
        /// occurrence can be constructed more than once in a single compile —
        /// `D2k-1c-0` measured `row1` doing it. `recognition` below is what
        /// distinguishes those constructions.
        field_origin: StaticOriginId,
        /// **`D2k-1c-0` — THIS constructed field, distinct from every other
        /// construction of the same occurrence.**
        ///
        /// The template carries it so the static elimination that rebinds this
        /// field transitions **this** recognition rather than "some recognition
        /// with that origin". Without it, two constructions of one occurrence
        /// were one ledger record, and a single rebind-and-consume closed green
        /// with the second worker constructed and forgotten — the forbidden
        /// fourth state, one link earlier in the chain than the transport
        /// identity can see. Architect `evt_3manpp82emcq6`.
        recognition: StaticWorkerRecognitionId,
    },
}

impl ConstructorField {
    /// The ordinary field a producer builds. Every construction site in the
    /// tree goes through this arm **except** the one armed producer,
    /// `Lowering::static_worker_constructor_template`, which is the sole
    /// builder of the worker arm.
    fn specialized(value: Lowered) -> Self {
        ConstructorField::Specialized(value)
    }

    /// The refusal a field reader raises when it reaches a static-worker field
    /// in a position that requires a value.
    ///
    /// **This is a shared REFUSAL TEXT, not a shared decision.** Each reader
    /// still decides what its own worker case does and supplies its own `edge`;
    /// this only spares one sentence from being copied at every site. A reader
    /// that should do something other than refuse must not call it.
    fn static_worker_refusal(edge: &str) -> CraneliftBackendError {
        unsupported(
            "StaticWorkerBinding",
            format!(
                "{edge} requires an ordinary specialized constructor field, and this field \
                 transports a static worker binding, which has no value representation"
            ),
        )
    }

    /// The ordinary field behind this kind, or a refusal naming the reader.
    ///
    /// **`edge` is the reader's own, exactly as it is for
    /// [`LoweringEnvironmentBinding::value_at`].** That is what makes a refusal
    /// identify *which* field read was taken rather than merely reporting that
    /// one was — the property section 2 of the frame records as an asset of the
    /// existing chokepoint. Passing a generic edge here throws that away.
    ///
    /// **The match is exhaustive with no wildcard**, so a future third field
    /// kind is a compile error at this accessor rather than a silent escape
    /// through it. The returned `Lowered` may contain runtime SSA values;
    /// "specialized" names the field form, not compile-time-known content.
    fn specialized_at(&self, edge: &str) -> Result<&Lowered, CraneliftBackendError> {
        match self {
            ConstructorField::Specialized(value) => Ok(value),
            ConstructorField::StaticWorker { .. } => Err(Self::static_worker_refusal(edge)),
        }
    }

    /// The owned ordinary field behind this kind, or a refusal naming the
    /// reader. The by-value twin of [`ConstructorField::specialized_at`], for
    /// readers that consume the lowered payload rather than borrow it. A
    /// `Lowered` payload may contain runtime SSA values; "specialized" names
    /// the field form, not compile-time-known content.
    fn into_specialized_at(self, edge: &str) -> Result<Lowered, CraneliftBackendError> {
        match self {
            ConstructorField::Specialized(value) => Ok(value),
            ConstructorField::StaticWorker { .. } => Err(Self::static_worker_refusal(edge)),
        }
    }
}

#[derive(Clone)]
enum Lowered {
    Int {
        value: cranelift_codegen::ir::Value,
        known: Option<i64>,
    },
    Bool {
        value: cranelift_codegen::ir::Value,
        known: Option<bool>,
    },
    ProcessExitStatus {
        value: cranelift_codegen::ir::Value,
    },
    CapabilityToken {
        value: cranelift_codegen::ir::Value,
    },
    ResourceToken {
        value: cranelift_codegen::ir::Value,
    },
    BoundedNat(BoundedNatV1),
    /// A structural `Nat` constructed by checked Ken. Unlike `BoundedNat`,
    /// this value is not a host-reply proof carrier; it is the ordinary unary
    /// constructor representation deforested to one native scalar.
    StructuralNat(StructuralNatV1),
    /// A runtime byte span that will be dereferenced and copied. The payload is
    /// a [`SafeByteSpan`], so the braced literal is not constructible outside
    /// `safe_byte_span` — `AC-10`.
    ResponseBytes(SafeByteSpan),
    HostResult {
        success: cranelift_codegen::ir::Value,
        error: Box<Lowered>,
        ok: Box<Lowered>,
        err_constructor: String,
        ok_constructor: String,
    },
    DynamicConstructor(DynamicConstructorV1),
    Bytes(Vec<u8>),
    BorrowedNativeValue {
        pointer: cranelift_codegen::ir::Value,
    },
    BorrowedOption {
        present: cranelift_codegen::ir::Value,
        value: cranelift_codegen::ir::Value,
        none: String,
        some: String,
    },
    String(String),
    Constructor {
        constructor: String,
        /// The already-resolved constructor identity, carried with the template.
        ///
        /// `Some` for a compiler-synthesized constructor whose identity came
        /// from the semantic plane's closed role capability, **and also for a
        /// source constructor whose template may outlive its own occurrence** --
        /// which is every source constructor the `Construct` lowering arms
        /// build. The name is narrower than the population it now has; read the
        /// field as "identity resolved at the producer", not "this constructor
        /// is synthesized".
        synthesized_identity: Option<ConstructorIdentity>,
        /// **`RT-DECL-CLOSURE-PORT` `D7` -- the planner-issued aggregate
        /// occurrence this template will become.**
        ///
        /// Carried for exactly the reason `synthesized_identity` is, stated in
        /// that field's own words: a later unit boundary may receive this result
        /// after nested producer traversal, where the caller occurrence is not
        /// the constructor occurrence and therefore cannot lawfully re-query the
        /// plan. The allocation lane is the second fact with that property, so
        /// it travels the same way. Same shape as
        /// [`Lowered::DeclarationClosure::reference`].
        ///
        /// `None` is a REFUSAL, never a default. An emitter reaching an
        /// aggregate with no interned occurrence must fail as loudly as one
        /// reaching an origin with no ownership record: answering
        /// `PersistentGround` would reinstate the unproven persistent lane for
        /// precisely the producers the population still misses.
        occurrence: Option<AggregateOccurrenceId>,
        /// **`D2k-1b-i0` — the fields are a closed compiler-only kind, not bare
        /// [`Lowered`].** See [`ConstructorField`] for why the old
        /// `Vec<Lowered>` was the assumption the five `#6d` walls violate.
        args: Vec<ConstructorField>,
    },
    Record {
        /// **`RT-DECL-CLOSURE-PORT` `D7` — the planner-issued aggregate
        /// occurrence this template will become.**
        ///
        /// ⭐ Exactly what `Lowered::Constructor::occurrence` is, and here for
        /// the same reason: **the aggregate's authority belongs to its
        /// PRODUCER, not to whatever coordinate happens to be in scope where it
        /// is used.** A record forwarded through a `Var` or handed to a call
        /// arrives somewhere that cannot lawfully re-query the plan for it.
        ///
        /// ⛔ Its absence is what made a source `Record` resolve its ownership
        /// record at the coordinate of its *use* — a callee's scheduling entry,
        /// a call argument's slot — and be refused on the shape cross-check
        /// against whatever record lived there. The `Constructor` variant never
        /// had that defect because it already carried this.
        ///
        /// `None` is a REFUSAL, never a default, exactly as on `Constructor`:
        /// an aggregate with no interned occurrence has no lifetime meet, and
        /// answering `PersistentGround` for it would reinstate the unproven
        /// persistent lane. A value-domain record — one built from a
        /// `RuntimeValue`, with no occurrence in the program — carries `None`
        /// and fails closed at the allocation rather than borrowing a lane.
        occurrence: Option<AggregateOccurrenceId>,
        fields: Vec<LoweredRecordField>,
    },
    /// A retained closure. ⭐ **The body is the static origin and nothing else.**
    ///
    /// `body` used to be an `OwnedSourceOccurrence` — a cloned `RuntimeExpr`
    /// carried beside its origin. That is the shape `RT-NATIVE-FNSPLIT` exists to
    /// remove: it let a *dynamic* property (which term this value happens to hold)
    /// name *static* code, so two authorities described one body and the cloned
    /// one won.
    ///
    /// Now the value names its body the way the planner already named it, and the
    /// term is recovered from the plan by that name alone
    /// (`Lowering::retained_body_occurrence`). ⛔ Do not reintroduce a term here,
    /// not even "for convenience": a carrier holding both would restore the two
    /// authorities, and the origin would be back to decorating a body rather than
    /// selecting one.
    ///
    /// ⚠ This does **not** change *when* a body is lowered. Each call site still
    /// re-lowers the resolved term in its own whole configuration — that is
    /// symptom-inventory entry 2, and it stays open for `RT-FNSPLIT-B2F`.
    Closure {
        /// **`D7` — the PHASE-BEARING capture edge.**
        ///
        /// A retained callable is an invocation-local compiler control capsule:
        /// the capsule itself is specialized, but the values it closes over
        /// reached it at their own phases, and a capture that arrived through a
        /// declared ABI slot is `Carried`. Storing `Lowered` here forced every
        /// such capture to be read as a compile-time template, which is why a
        /// lawfully mixed environment could not be represented at all.
        ///
        /// ⛔ **A carried capture does not become a `Lowered` value.** It gains
        /// no [`LoweredVariant`], no [`BoundaryDisposition`], no encoding
        /// policy, no inverse conversion, no carrier tag, no durable slot and no
        /// independent callable identity. The governing line is **phase
        /// identity, not transitive Rust containment**: the word stays typed as
        /// a [`LoweringOperand`] end to end.
        ///
        /// ⛔ **The capsule stays unconditionally non-transferable**, and
        /// [`Lowering::boundary_transfer_admissibility`] refuses it **before**
        /// descending into these captures — a carried capture must never become
        /// a way to reach the carrier through a callable that is itself refused.
        captures: Vec<LoweringOperand>,
        params: Vec<String>,
        body: StaticOriginId,
    },
    DeclarationClosure {
        /// **`RT-DECL-CLOSURE-PORT` `D4` — the planner-issued
        /// `DeclarationRef` occurrence this binding was produced at.**
        ///
        /// ⭐ Not a decoration and not derivable from `symbol`: the resolved
        /// declaration-call record is keyed by the **reference** occurrence, so
        /// two references to one declaration are two distinct call sites with
        /// their own targets. Carrying it here is what lets the `Call` consumer
        /// emit *this* reference's call instead of looking one up by name.
        ///
        /// ⛔ It is not a second body authority — `body` remains the sole
        /// callable-body identity, exactly as `AC-1` requires.
        reference: StaticOriginId,
        symbol: RuntimeSymbol,
        /// **`D7` — the same phase-bearing capture edge as
        /// [`Lowered::Closure::captures`], for the same reason.** A declaration
        /// closure's *lexical* captures reach it at their own phases; its *seed*
        /// captures are resolved to JIT-time ground values and are therefore
        /// always constructed as explicit `Specialized`.
        captures: Vec<LoweringOperand>,
        params: Vec<String>,
        body: StaticOriginId,
    },
    /// ⭐⭐ **The one `Lowered` child position that is a [`LoweringOperand`], by
    /// the Architect's `AC-C4` SINGLE-FIELD LICENSE — ⛔ not a precedent.**
    ///
    /// `residual` is the value the saved recursor **continues on**. `§2h`'s
    /// phase closure therefore requires this edge to preserve `Carried`:
    /// eliminating a carried scrutinee whose case declares a recursive position
    /// builds an induction hypothesis over a **carried** child, and treating
    /// that child as a compile-time template is precisely the defect that
    /// closure exists to forbid.
    ///
    /// ⭐ **Why this is not the `§2g` violation it looks like.** The governing
    /// line is **phase identity, not transitive Rust containment.** `§2g`
    /// forbids a carried word from *becoming* a `Lowered` inhabitant or
    /// acquiring a [`LoweredVariant`], a [`BoundaryDisposition`], an encoding
    /// policy, or an inverse conversion. None of those happens here: the word
    /// stays typed as a `LoweringOperand` end to end, and the outer object
    /// remains a specialization-only, in-flight **control capsule**.
    /// `Specialized` classifies how *that capsule* is consumed; it never
    /// asserted that every operand edge the capsule owns is itself specialized.
    ///
    /// ⛔ **The license is this field and nothing else.** `Constructor`,
    /// `Record`, `Closure`, `DeclarationClosure` and every other child position
    /// stay `Lowered`. ⛔ No third `LoweringOperand` variant, ⛔ no
    /// `Lowered::Boundary`, ⛔ no `Carried -> Lowered` conversion, ⛔ no durable
    /// closure lane, ⛔ no encoder/decoder row, ⛔ no carrier tag.
    ///
    /// ⚠ **The capsule itself stays unconditionally non-transferable, and the
    /// ORDERING is part of the ruling.** The admission walk must reject a
    /// `ComputationalRecursorClosure` *before* inspecting or emitting its
    /// residual — see [`Lowering::boundary_transfer_admissibility`] and the
    /// [`LoweredVariant::ComputationalRecursorClosure`] `FailClosedForbidden`
    /// row, both of which match `{ .. }` and refuse without descending. A
    /// carried residual must not become a way to reach the carrier through a
    /// capsule that is otherwise refused.
    ComputationalRecursorClosure {
        residual: Box<LoweringOperand>,
        activation: ContinuationActivationId,
        invocation: RecursorInvocationSegment,
    },
    /// A tail-recursive edge already emitted as a CFG jump. The current block
    /// is predecessor-free; enclosing scalar combinators propagate this
    /// marker so it cannot be confused with an ordinary or terminal value.
    RecursiveBackedge,
    Trap(RuntimeTrap),
}

/// **`RT-DECL-CLOSURE-PORT` `D7` — one field of a lowered record, with its
/// producer-issued typed identity beside the value it names.**
///
/// ⭐ **The identity and the value are ONE member, never two lists.** A record's
/// field names are a producer fact — resolved where the record is built and
/// meaningless at any later coordinate — exactly as its ownership record is. A
/// parallel `Vec<FieldIdentity>` beside the values would carry that fact, and
/// would also make a desync spellable: a schema one element short, or ordered
/// differently from the values, still resolves real identities and still emits.
/// Pairing them makes the desync a type error instead of a wrong name.
///
/// ⛔ **`name` is NOT the identity, and the two may never be interchanged.** It
/// is the compile-time spelling, retained because value-domain records and
/// specialized projection are keyed on it; `identity` is the artifact-static
/// word the carrier ABI stores and `Project` looks up (`D1`/`D2`). Deriving one
/// from the other in either direction is the second derivation `D2` forbids —
/// there is no `&str -> FieldIdentity` mapping and none may be added.
#[derive(Clone)]
pub(in crate::cranelift_backend) struct LoweredRecordField {
    /// The compile-time spelling. ⛔ Never an identity.
    pub(in crate::cranelift_backend) name: String,
    /// The planner-issued typed identity, resolved at the record's producer.
    ///
    /// ⛔ `None` is a REFUSAL, never a default, for the same reason
    /// [`Lowered::Record::occurrence`]'s absence is: a value-domain record has
    /// no occurrence in the program and therefore no planned schema, and
    /// emitting a name for it would mean inventing one. It fails closed at the
    /// preflight rather than borrowing a name from the coordinate it is
    /// transferred at.
    pub(in crate::cranelift_backend) identity: Option<FieldIdentity>,
    pub(in crate::cranelift_backend) value: Lowered,
}

/// The boundary-carrier helpers this generated function may call.
///
/// ⛔ **Exactly the helpers the four carrier routes use, and no more.** A ref
/// declared here that no route calls is inert threading — the defect this node
/// exists to avoid — so the set is kept minimal and every member is reached by
/// the one-way producer ([`Lowering::transfer_into_carrier`]), the
/// `Match`/`ComputationalMatch` route, or the `Project` route.
///
/// ⚠ **The set grew by two when the producer landed, and the reason is worth
/// recording rather than absorbing.** The original comment said *"exactly the
/// helpers the three **eliminators** use"* — which under-counts by exactly the
/// producer, because `§2g` names it as a fourth route (*"the boundary producer
/// has a one-way typed seam ... it consumes the sole `BoundaryLocalFuncs`
/// authority"*). `make_immediate` and `store_name` are producer-only: an
/// eliminator never mints an immediate and never writes a field name.
#[derive(Clone, Copy, Debug)]
struct BoundaryCarrierRefs {
    /// `(arena, word, out) -> status` — the representation class that selects
    /// representation-specific consumers such as `HostResult`.
    class: FuncRef,
    /// `(arena, word, out) -> status` — the runtime constructor/record identity
    /// that `Match` discriminates against the artifact-static case set.
    tag: FuncRef,
    /// `(arena, word, out) -> status` — the child count a case's binder arity
    /// is checked against **at runtime**.
    field_count: FuncRef,
    /// `(arena, word, index, out) -> status` — positional child projection.
    /// Its result stays [`LoweringOperand::Carried`].
    field: FuncRef,
    /// `(arena, word, name_id, out) -> status` — `Project` by artifact-static
    /// field identity.
    record_field: FuncRef,
    /// Runtime scalar extraction for statically scalar consumer positions.
    scalar: FuncRef,
    /// HostResult runtime discriminant and selected payload.
    host_success: FuncRef,
    host_payload: FuncRef,
    /// `(arena, tag, class, field_count, out) -> status` — the one-way
    /// producer's allocation step.
    alloc: FuncRef,
    /// `(arena, word, tag_id) -> status` — the producer records the identity.
    store_tag_id: FuncRef,
    /// The producer records an inline scalar or HostResult discriminant.
    store_scalar: FuncRef,
    /// `(arena, word, index, child) -> status` — the producer writes children.
    store_field: FuncRef,
    /// `(arena, word, index, name_id) -> status` — the producer writes a
    /// record's field names, so [`Self::record_field`] can find one by
    /// artifact-static identity. ⛔ No `arena`-free shortcut: the name a
    /// `Project` looks up and the name the producer wrote must be the same
    /// word, which is `D2` at the field-identity namespace.
    store_name: FuncRef,
    /// `(tag, payload, out) -> status` — the producer's leaf step for a value
    /// whose payload rides in the tagged word. ⚠ Note the **absent `arena`**:
    /// an immediate names no referent, so there is nothing for an arena to own.
    make_immediate: FuncRef,
    /// `(arena, word, native_tag) -> status` — the spill arm records **how** the
    /// magnitude word is to be read, as a `NativeIntV1` marker.
    ///
    /// ⛔ **This is also the spill arm's region guard, and it is deliberately
    /// not re-derived here.** `BOUNDARY_INT_MARKER_OWNER` says a
    /// `NATIVE_INT_BIG_TAG_V1` payload is a slot in the *invocation's* native
    /// arena and therefore inadmissible on a persistent node. The helper
    /// enforces that from the one table; the producer neither restates the rule
    /// nor pre-empts it — see [`Lowering::emit_carrier_spillable_immediate`] for
    /// the residual that leaves.
    store_int_tag: FuncRef,
    /// `(arena, word, len, out) -> status` — claim `len` content bytes in the
    /// node's own region. ⭐ A **claim-then-fill** protocol: the span exists
    /// before a byte of it is written, so a length the region cannot satisfy
    /// fails before any address is formed.
    store_bytes_len: FuncRef,
    /// `(arena, word, index, byte) -> status` — write one content byte.
    store_byte: FuncRef,
    /// `(arena, word, sign, len, out) -> status` — claim `len` magnitude limbs
    /// in the node's **own** region for a region-limbed `Int`.
    store_int_limbs: FuncRef,
    /// `(arena, word, index, limb) -> status` — write one magnitude limb.
    store_int_limb: FuncRef,
    /// `(arena, word) -> status` — check a region-limbed `Int`'s magnitude
    /// canonical and seal it. ⛔ **Until this succeeds the node denotes
    /// nothing**, so it is the last step of the copy and never optional.
    seal_int: FuncRef,
    /// `(arena, word, out_view) -> status` — canonical exact-`Int` view.
    int_view: FuncRef,
    bytes_view: FuncRef,
}

/// A value that has crossed into the **operational carrier** — nothing but the
/// Cranelift SSA boundary word (`RT-FNSPLIT-C1` `D3`).
///
/// ⛔ **It holds the word and NOTHING ELSE, and the emptiness is the point.**
/// No constructor string, no field list, no body or template, no tag/class, no
/// reverse-decoding data. Every question about this value — which constructor,
/// how many fields, which child — is answered by **calling an emitted helper at
/// runtime**, never by reading a field of this struct. ⇒ The struct having room
/// for a compile-time answer is exactly how the wall would grow back.
#[derive(Clone, Copy, Debug)]
struct CarriedBoundaryWord {
    word: cranelift_codegen::ir::Value,
}

/// ⭐ **The closed PHASE sum — which phase a lowering operand is in, not what
/// kind of value it is** (`RT-FNSPLIT-C1` `D3`).
///
/// ⛔ **THE RULING IS `§2g` OF THE C1 FRAME — cite it, do not restate it.**
/// `docs/program/wp/RT-FNSPLIT-C1-operational-carrier.md §2g` carries Architect
/// Decision `dec_4te25repm33ph`'s resolution verbatim. Where the frame and any
/// restatement disagree, **the frame governs**.
///
/// ⚠ An earlier revision of this comment pasted the ruling text here. That was
/// a **second authority** — the exact defect this chain keeps paying for — and
/// it is deleted rather than kept "for convenience": two copies of a ruling
/// drift silently, and the copy nearest the code is the one a reader trusts.
///
/// ⭐ The clause that shapes the surface census, and the one a
/// three-eliminator reading misses: **environments and result surfaces that can
/// receive a transferred value carry this wrapper.** Every other clause is
/// prohibitive; this one *adds* obligated surface.
///
/// ⚠ The **name** `LoweringOperand` is this implementation's choice and is not
/// ruled; the **shape** is.
///
/// ⛔ **NOT a variant of [`Lowered`].** `Lowered` is a compile-time
/// specialization lattice (`§2f`); a `Lowered::Boundary` inhabitant is the
/// `B2E` shape the inertness rule rejects — the inhabitant is the easy half and
/// the three executable eliminations are the node.
///
/// ⛔ **One-way.** There is a producer into [`Self::Carried`] and deliberately
/// **no** inverse: a `Carried → Lowered` conversion would let a consumer
/// recover a compile-time template from a runtime value, which is the wall
/// itself wearing a different name.
// ⛔ No `Debug`: `Lowered` has none, and deriving one here would be a new,
// second way to read a compile-time template out of an operand.
#[derive(Clone)]
enum LoweringOperand {
    /// The compile-time specialization lattice — every route that existed
    /// before this node. ⛔ Kept as an **explicit** arm, never a fallback:
    /// a fallback arm is a wildcard with better manners.
    Specialized(Lowered),
    /// A runtime boundary word, eliminated only by emitted helpers.
    Carried(CarriedBoundaryWord),
}

/// **THE ONE BINDING AUTHORITY** for a lexical environment (`RT-WORKER-BIND`
/// judgment 4). Every lexical environment reaching [`Lowering::lower_expr`] is
/// a slice of these -- saved ordinary and computational eliminator
/// environments, pending-`Let` environments, and recursor outer environments
/// alike.
///
/// There is deliberately **no** parallel operand environment and **no**
/// de-Bruijn side map. Either would create a second binding authority, and then
/// the question *"what is bound here"* would have two answers.
///
/// This sum is compiler-only. It is **not** a [`Lowered`] variant, not a third
/// [`LoweringOperand`] arm, and it never becomes a runtime value.
#[derive(Clone)]
enum LoweringEnvironmentBinding {
    /// An ordinary bound value. Every binder that existed before this node
    /// installs this arm, and the outer spine forwards it unchanged.
    Value(LoweringOperand),
    /// A statically-bound worker: a lexical callable whose body is a declared
    /// static-body unit. Its sole admissible use is as the callee of a `Call`
    /// with an exact `Var` callee; every value-producing position rejects it.
    ///
    /// `D2` constructs this arm from a lexical closure's retained occurrence.
    /// The callee-only consumer is `D3`, so nothing *reads* the binding's
    /// fields in production yet -- see the note on [`StaticWorkerBinding`].
    StaticWorker(StaticWorkerBinding),
}

/// The compiler-only description of a static worker binding.
///
/// It carries no runtime word, tag, layout, vtable, descriptor or environment
/// pointer, and no callable identity. `captures` stay [`LoweringOperand`], so a
/// carried capture stays carried -- the binding never becomes a value.
///
/// A `FuncRef` deliberately does **not** live here: it belongs to one Cranelift
/// `Function`, and the target is declared afresh into each generated function
/// (`D4`).
///
/// `D3` reads `body_origin`, `declared_arity` and `captures`. It does **not**
/// read `closure_origin`, so the allowance is narrowed to that one field
/// rather than blanketing the struct.
///
/// `closure_origin` is preserved because judgment 1 rules these four fields
/// and the separation may not vary; it is the binding's own occurrence
/// identity, which no consumer needs today. Narrowing the allowance is what
/// keeps a future genuinely-unread field visible instead of silently covered.
#[derive(Clone)]
struct StaticWorkerBinding {
    #[allow(dead_code)]
    closure_origin: StaticOriginId,
    body_origin: StaticOriginId,
    declared_arity: u32,
    captures: Vec<LoweringOperand>,
    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D6a`** — which callee this binding's call
    /// goes to, carried rather than reconstructed. See
    /// [`StaticWorkerCallRoute`].
    ///
    /// ⛔ **Deliberately NOT `#[allow(dead_code)]`.** `D6a` is the
    /// representation; the route's consumption at the call edge is `D6b`, so
    /// production reads nothing here yet and the compiler says so. That warning
    /// is the obligation staying visible, exactly as the narrowed allowance on
    /// `closure_origin` above intends — silencing it would turn an open
    /// checkpoint into an invisible one.
    route: StaticWorkerCallRoute,
    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D8i`** — which causal obligation this
    /// binding's consumption may satisfy. See [`ContinuationDischarge`].
    ///
    /// ⛔ **A separate facet from `route`, deliberately.** The route decides the
    /// callee and the operand run; the discharge decides which causal call, if
    /// any, a consumption of this binding is allowed to answer for. They are
    /// independent: the composed selected recursive argument is `RawWorker` and
    /// carries an authority, while an ordinary `RawWorker` induction hypothesis
    /// carries none. Folding the authority into the route would make "which
    /// callee" and "which obligation" one field, and `D6b` already showed what
    /// happens when two contracts share one discriminator.
    ///
    /// ⛔ **The open checkpoint is visible on the READER, not on this field.**
    /// The field is read by [`StaticWorkerBinding::
    /// composed_continuation_authority`], so the compiler does not name it
    /// unread — and it would be wrong to claim it does. That accessor is what
    /// carries the narrowed allowance until `D8j` supplies its production
    /// consumer, and it is the thing to check when asking whether `D8i` is
    /// still transport-only.
    discharge: ContinuationDischarge,
    /// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2k-1c-0`** — the TRANSPORT this
    /// binding was rebound as, or `None` if it was bound directly.
    ///
    /// **This is the identity of one transport, NOT of the field.** It was a
    /// `field_origin` through `D2k-1b-i`, which was already the second tally in
    /// a row: the planner origin identifies the *occurrence*, and `D2k-1c-0`
    /// measured that one occurrence is descended more than once in a single
    /// compile. So a field-keyed binding still could not say *which* of that
    /// occurrence's transports a call discharged, and a per-origin balance let
    /// transport #1 be consumed twice while transport #2 was dropped. The
    /// planner origin survives as provenance inside the ledger; the thing the
    /// binding carries is the transport. Architect `evt_2npnrzesz3t65`.
    ///
    /// **`None` is a real state, not a missing value:** every pre-existing
    /// construction site produces it, and a call consuming such a binding
    /// discharges nothing at all. That is what keeps an ordinary worker call
    /// from paying a transported field's debt.
    ///
    /// ⇒ Compiler-side bookkeeping only. It is an opaque compile-local
    /// identity, **not** a runtime word, tag, descriptor, carrier or callable
    /// identity, it is never emitted, and recording it creates no planner
    /// population.
    transport: Option<StaticWorkerTransportId>,
}

impl StaticWorkerBinding {
    /// **`D8i` — the composed causal authority this binding carries, or a
    /// refusal.**
    ///
    /// ⛔ **An ordinary binding is REJECTED, not answered with `None`.** A
    /// caller asking this question is asking "which composed obligation may I
    /// discharge with this", and for a [`ContinuationDischarge::
    /// DirectSpecializationCall`] binding the answer is not "none available" —
    /// it is that the question does not apply to it. `None` would let a caller
    /// treat the two cases alike with `unwrap_or_default`, `is_some`, or an
    /// `if let` that silently skips; a `Result` makes ignoring it a written
    /// decision.
    ///
    /// ⭐ Returned by reference and still opaque. A caller can compare it, key
    /// on it, and read its target specialization and emission owner. It cannot
    /// read the call-site sequence and cannot construct one.
    ///
    /// `D8j` is this method's production consumer, and the narrowed allowance
    /// that stood here is DELETED rather than widened: the composed discharge
    /// obtains its authority through this accessor, so a refusal is on the live
    /// path rather than behind a `cfg`.
    fn composed_continuation_authority(
        &self,
    ) -> Result<&ContinuationCallIdentity, CraneliftBackendError> {
        match &self.discharge {
            ContinuationDischarge::ComposedSourceContinuation(identity) => Ok(identity),
            ContinuationDischarge::DirectSpecializationCall => Err(unsupported(
                "StaticWorkerBinding",
                "an ordinary static-worker binding carries no composed causal authority: its \
                 discharge is a direct specialization call, which answers for no composed \
                 source continuation. This refuses rather than reporting an absence, because a \
                 consumption that reached here has already decided it is discharging a composed \
                 obligation and there is none to discharge",
            )),
        }
    }
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8f` — a pending checked-IH marker, and the
/// exact application occurrence it denotes.**
///
/// ⛔⛔ **The occurrence is the whole point of this type existing.** The marker
/// used to be a bare template id, so "a marker is pending" was the only fact a
/// consumption site had — and the site is the static-worker call arm, which
/// every static-worker call reaches. A pending marker therefore meant *the next
/// static-worker call consumes it*, and inside one checked wrapper the next
/// call need not be the checked one: the wrapped application's own ARGUMENTS
/// are evaluated first, and an argument can be an ordinary call on the selected
/// recursive argument.
///
/// ⭐ `application_origin` is the occurrence of the expression the marker
/// wraps, taken from the same `child_origin(marker, 0)` the lowering already
/// derives to lower it. That is the existing checked occurrence authority
/// projected faithfully — the marker denotes the complete application
/// occurrence, which this file already states — and it is not a route, an
/// arity, a binder index, a call ordinal, or any inferred shape.
#[derive(Clone, Copy)]
struct PendingCheckedIhCall {
    call_template_id: u64,
    /// The compiler-private checked-IH use shape. This is consumed by a static
    /// exhaustive dispatcher and never enters the crossing value.
    kind: crate::CheckedComputationalIHInvocationKind,
    /// Explicit source-to-runtime binder map for this invocation. The plan's
    /// IH-subsequence ordinal is translated through it before comparison with
    /// the emitted runtime `Var`.
    binder_morphism: crate::CheckedComputationalIHBinderMorphism,
    /// The occurrence of the application this marker denotes. Only a call being
    /// lowered AT this occurrence may consume the marker.
    application_origin: StaticOriginId,
}




// `RT-EMITTER-CALLS-RETURNS-SPLIT` `D1` — RETAINED at the hub, not moved to
// `calls`. The D0 ledger traced these as "exclusive to `call_static_worker`/
// `call_static_worker_with_inputs`" (both of which DO move); execution-time
// tracing found that RETAINED `dispatch_fused_consuming_call` also
// constructs `StaticWorkerCallOutcome` directly and RETAINED
// `claim_composed_discharge` destructures `StaticWorkerEmission`'s fields
// directly — genuinely shared between a moving and a staying consumer, the
// same hub-stays/methods-move shape item 12 established, not the exclusive
// population the ledger recorded. Corrected here per the standing carry
// (treat a D0 ledger's silence on a manipulated type's own disposition as an
// unclosed AC-1 gap, closed by usage-tracing at `D1` execution).
/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8j` — what the shared static-worker emitter
/// hands back beside the call's value.**
///
/// ⭐ The instruction is returned rather than recorded by the emitter, because
/// the emitter serves BOTH consumers — direct descent and the source machine —
/// and only one of them may discharge a composed obligation. An emitter that
/// recorded would be deciding a question that belongs to its caller.
#[derive(Clone, Copy)]
struct StaticWorkerEmission {
    /// The raw-worker call instruction actually written.
    inst: cranelift_codegen::ir::Inst,
    /// The operand run supplied to it: explicit arguments plus stored captures
    /// plus any route suffix. Compared against the `D8b`/`D8d` target's own
    /// declared contract at verification.
    supplied_operands: usize,
}

/// The shared static-worker seam either emits its call or, under the R3
/// post-field control only, hands the already-validated call back to the outer
/// seam. Production has no deferred variant.
enum StaticWorkerCallOutcome {
    Emitted(LoweringOperand, StaticWorkerEmission),
    #[cfg(test)]
    DeferredPostField(LoweringOperand),
}

impl StaticWorkerCallOutcome {
    fn into_operand(self) -> LoweringOperand {
        match self {
            Self::Emitted(operand, _) => operand,
            #[cfg(test)]
            Self::DeferredPostField(operand) => operand,
        }
    }

    fn into_emitted(
        self,
    ) -> Result<(LoweringOperand, StaticWorkerEmission), CraneliftBackendError> {
        match self {
            Self::Emitted(operand, emission) => Ok((operand, emission)),
            #[cfg(test)]
            Self::DeferredPostField(_) => Err(backend_module(
                "a deferred post-field fused call reached a source-machine consumer"
                    .to_string(),
            )),
        }
    }
}

/// A call whose exact consuming occurrence supplied the target and operand run,
/// deferred solely so the test mutation can attempt it at the forbidden outer
/// post-field seam without re-deriving either input.
#[cfg(test)]
struct D2fDeferredPostFieldDirectCall {
    fusion: StaticContinuationFusionId,
    seat: StaticOriginId,
    consuming_call: StaticOriginId,
    target: units::DeclaredUnitCall,
    operands: Vec<LoweringOperand>,
}

/// **`D8j` — one CLAIMED composed discharge, awaiting the finished CLIF.**
///
/// ⛔ Every field is recorded at the seat from a value the seat already held.
/// Nothing here is re-derived at verification time from another field, because
/// then the verification would be comparing the record with itself.
struct PendingComposedDischarge {
    /// The opaque authority the binding transported.
    identity: ContinuationCallIdentity,
    /// The raw-worker call this discharge answers with.
    inst: cranelift_codegen::ir::Inst,
    /// The worker body the `D8b`/`D8d` target names — the callee the decoded
    /// instruction must resolve to.
    worker_body_origin: StaticOriginId,
    /// The target's declared contract: arity plus stored captures.
    declared_operands: usize,
    /// What the emitter actually supplied.
    supplied_operands: usize,
    /// The call's result word, as it was handed back to source-machine control.
    /// `None` when the result was not a carried word, which is itself a
    /// verification failure — recorded rather than rejected at the seat so the
    /// refusal happens in one place.
    result: Option<cranelift_codegen::ir::Value>,
    /// Live source continuations immediately before the emitter ran and
    /// immediately after the result was placed. Equal means the result returned
    /// into the continuation that was already in force.
    source_control: (usize, usize),
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8i` — the closed causal-discharge facet.**
///
/// ```text
/// ContinuationDischarge =
///   DirectSpecializationCall
/// | ComposedSourceContinuation(opaque ContinuationCallIdentity)
/// ```
///
/// ⭐ **Two arms, both explicit, no default and no `Option`.** Every
/// [`StaticWorkerBinding`] states its arm at construction because
/// [`Lowering::construct_static_worker_binding`] takes it as a required
/// argument — omission is a compile error, not a silent
/// [`Self::DirectSpecializationCall`]. That is the whole reason it is a
/// parameter rather than a field the composed path patches afterwards: a
/// defaulted facet would make "this binding carries no authority" and "nobody
/// said" the same value, and those are the two states this type exists to keep
/// apart.
///
/// ⛔ **Transport only, at this checkpoint.** Nothing here consumes, records or
/// closes the authority. The authority is carried opaquely — a
/// [`ContinuationCallIdentity`] has no sequence accessor and no lowering
/// constructor — so a binding can hand one on but can never mint one, and no
/// path can invent an obligation it is entitled to discharge.
#[derive(Clone, Debug, Eq, PartialEq)]
enum ContinuationDischarge {
    /// The ordinary binding: its call is a direct specialization call and
    /// answers for **no** composed causal obligation.
    ///
    /// ⛔ This is a positive statement, not an absence. Induction hypotheses,
    /// selected recursive arguments built by a specialization context, and
    /// lexical-closure capsules all carry it explicitly.
    DirectSpecializationCall,
    /// **The composed selected recursive argument.** Carries the exact opaque
    /// identity `D8h` paired with the [`ComposedCallTarget`] this binding was
    /// derived from — the identity that target's own five-field coordinate
    /// selects, transported unchanged.
    ///
    /// [`ComposedCallTarget`]: crate::cranelift_backend::planning::ComposedCallTarget
    ComposedSourceContinuation(ContinuationCallIdentity),
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D6a` — the closed compiler-only call route.**
///
/// Two bindings can name the **same** closure occurrence, body origin, declared
/// arity and ordered captures and still be different callables, because they
/// reach that body by different routes. A specialization's induction hypothesis
/// and its selected recursive constructor argument are exactly that pair: the
/// same closure, distinguished by where the call goes.
///
/// ⭐ **THE ROUTE LAW, and it is not symmetric.**
///
/// - [`Self::RawWorker`] is what a **`SelectedRecursiveArgument`** carries,
///   **always**. The source scope binds the closure itself, so there is no
///   condition to evaluate and no case in which it carries the other arm.
/// - A **`InductionHypothesis`** carries [`Self::GeneratedContext`] **iff** the
///   planner issued a generated execution context for this
///   `(specialization, worker body)` pair *and* this unit resolved it.
///   Otherwise it **lawfully carries `RawWorker` too**.
///
/// ⛔ **So equal routes on the two members is a LAWFUL state, not a defect.** A
/// unit whose planner issued no context is *route-degenerate*: both members
/// render `RawWorker`, and they remain two distinct bindings for two distinct
/// environment positions. Reading "the two rendered routes are equal" as "one
/// binding was reused for both members" is an invalid inference, and prose or a
/// control that draws it is wrong about the mechanism rather than strict about
/// it. The membership-and-order facts are what separate the members in the
/// degenerate case; the route separates them only where a context exists.
///
/// ⛔ **It is carried, never derived at the call site.** The pre-`D6a` code
/// decided the suffix by comparing `generated_context_captures.worker_body_
/// origin` against the binding's `body_origin`, and selected the callee by
/// whichever entry the retarget had left in `worker_calls`. Both readings are
/// *blind here by construction*: the two bindings share a body origin, so a
/// body-origin comparison answers the same for both, and one map entry cannot
/// name two routes. Body shape, declared arity, use site, environment length
/// and "whichever target exists" are all equally blind for the same reason —
/// which is why this is a field and not a predicate.
///
/// The route is fixed at construction, from the role the binding is built for
/// together with the planner's own issuance — the planner either issued a
/// generated execution context for this `(specialization, worker body)` pair
/// and this unit resolved it, or it did not. Neither input is recoverable at
/// the call edge, which is why both are read here.
///
/// ⛔ No `FuncRef` lives here. The route names *which declared-call table* a
/// caller resolves its target from; the target itself is still minted into the
/// calling function, so nothing crosses a function boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StaticWorkerCallRoute {
    /// The exact **raw worker body**, at its own unretargeted contract.
    ///
    /// Its operand run is the raw run and nothing else: explicit arguments,
    /// then stored captures. It appends **no** continuation-input suffix.
    ///
    /// Three populations take it: every binding built for an ordinary lexical
    /// closure; `D6a`'s selected recursive constructor argument, **without
    /// exception**; and an induction hypothesis in a unit that resolved no
    /// generated execution context — the third being the route-degenerate case
    /// described on the type above, and lawful.
    RawWorker,
    /// The **planner-issued generated execution context** that executes that
    /// same body on behalf of one enclosing specialization.
    ///
    /// The context's ABI has a capture run for the enclosing specialization's
    /// continuation inputs, so a call on this route appends that suffix.
    ///
    /// ⛔ Reached by an induction hypothesis **only when a context was issued
    /// and resolved**, and by nothing else — never by a selected recursive
    /// constructor argument. "The route an induction hypothesis takes" states
    /// the implication in the wrong direction: this arm implies an induction
    /// hypothesis, an induction hypothesis does not imply this arm.
    GeneratedContext,
}

impl LoweringEnvironmentBinding {
    /// **THE FAIL-CLOSED READ** for a value-producing position.
    ///
    /// `Var` in every value-producing position accepts **only** [`Self::Value`]
    /// (`D3`). A [`Self::StaticWorker`] used as a result, aggregate field,
    /// primitive or effect argument, stored value, projection subject, match
    /// scrutinee, or ordinary call argument fails closed **here**, before any
    /// carrier transfer.
    ///
    /// The match is exhaustive with no wildcard, and that is the mechanism: a
    /// future third arm is a compile error at every value-producing read rather
    /// than a silent escape. `edge` names the call site in the diagnostic.
    fn value_at(&self, edge: &'static str) -> Result<&LoweringOperand, CraneliftBackendError> {
        match self {
            LoweringEnvironmentBinding::Value(operand) => Ok(operand),
            LoweringEnvironmentBinding::StaticWorker(_) => Err(unsupported(
                "StaticWorkerBinding",
                format!(
                    "{edge} is a value-producing position and a static worker binding has no \
                     value representation; its only admissible use is as the callee of a call \
                     with an exact Var callee"
                ),
            )),
        }
    }
}

/// A freshly bound operand entering the one binding authority. Every binder
/// that predates this node goes through here, which is what makes *"existing
/// binders install `Value`"* structural rather than tested.
impl From<LoweringOperand> for LoweringEnvironmentBinding {
    fn from(operand: LoweringOperand) -> Self {
        LoweringEnvironmentBinding::Value(operand)
    }
}

impl LoweringOperand {
    /// ⭐ **THE RULED TYPED PHASE BOUNDARY** in front of a specialized-only
    /// helper — `§2h` ¶2, verbatim: *"Every edge from `LoweringOperand` into
    /// such a specialized-only helper must first exhaustively classify both
    /// variants with no wildcard; `Carried` must take its ruled emitted-helper
    /// route **or fail closed**."*
    ///
    /// ⛔ **This is NOT the forbidden `Carried -> Lowered` conversion, and the
    /// difference is the whole point.** A conversion would *answer* a
    /// compile-time question about a runtime value; this **refuses** to. The
    /// `Carried` arm produces an error, never a `Lowered`, so no caller can
    /// recover a template through it.
    ///
    /// ⚠⚠ **THE MISUSE THIS INVITES, NAMED SO A REVIEWER CAN HUNT IT.** Nothing
    /// in production emits a `Carried` operand yet, so **every** call of this
    /// method is currently unreachable in its failing arm — which means putting
    /// it on a *phase-bearing* edge compiles, passes the whole suite, and still
    /// destroys the closure `§2h` mandates. ⇒ It is legal **only** where the
    /// callee inspects or destructures the **compile-time template** (a
    /// primitive's operands, a shape comparison, a constructor-name probe). It
    /// is ⛔ **illegal** on the three forwarding roles, which must stay
    /// `LoweringOperand` end to end:
    ///
    /// | role | why it may not fail closed here |
    /// |---|---|
    /// | environment insertion | `§2h`: a projected `Carried` child must survive `case_env` |
    /// | recursive lowering call | the callee is in the phase-bearing component, not a leaf |
    /// | result / join forwarding | a refused join is a lost `Carried`, not a rejected one |
    ///
    /// `edge` names the call site in the diagnostic. ⚠ It is a **label, not a
    /// mechanism** — it makes a misplaced boundary legible in a failure, and
    /// nothing more.
    /// **The phase this operand is ACTUALLY in.**
    ///
    /// ⛔ An observation, not a decision. It is the one fact the planner's
    /// derived `consumer_phase` is checked against, so it must read the operand
    /// in hand and nothing else — a version of this that consulted the plan
    /// would make the agreement true by construction.
    fn effect_seat_phase(&self) -> EffectSeatPhase {
        match self {
            LoweringOperand::Specialized(_) => EffectSeatPhase::SpecializedTemplate,
            LoweringOperand::Carried(_) => EffectSeatPhase::CarriedWord,
        }
    }

    fn specialized_at(self, edge: &'static str) -> Result<Lowered, CraneliftBackendError> {
        match self {
            LoweringOperand::Specialized(lowered) => Ok(lowered),
            LoweringOperand::Carried(_) => Err(unsupported(
                "BoundaryCarrier",
                format!(
                    "{edge} is a specialized-only surface and a carried boundary word has no \
                     compile-time template for it to read; the carrier's ruled route is an \
                     emitted helper call"
                ),
            )),
        }
    }


    /// [`Self::specialized_at`] without consuming the operand — same ruling,
    /// same prohibitions, for a callee that borrows its template.
    fn specialized_ref_at(&self, edge: &'static str) -> Result<&Lowered, CraneliftBackendError> {
        match self {
            LoweringOperand::Specialized(lowered) => Ok(lowered),
            LoweringOperand::Carried(_) => Err(unsupported(
                "BoundaryCarrier",
                format!(
                    "{edge} is a specialized-only surface and a carried boundary word has no \
                     compile-time template for it to read; the carrier's ruled route is an \
                     emitted helper call"
                ),
            )),
        }
    }
}

/// The spine's bulk phase boundary -- a **list of operands** rendered as the
/// specialized templates a specialized-only helper reads: a constructor's
/// arguments, a closure's captures, a primitive's operands.
///
/// Same ruling and the same prohibitions as
/// [`LoweringOperand::specialized_at`]; this exists because several leaves take
/// a whole `&[Lowered]` rather than one, and hand-writing the fold at each
/// would multiply the classification instead of sharing it.
///
/// This is deliberately **not** the environment form. An operand list has no
/// binding to classify, so there is nothing here to fail closed on; a lexical
/// environment goes through [`specialized_bindings_at`] instead, which crosses
/// the binding authority first. The two are named apart so that passing an
/// environment to the operand form is a type error rather than a silent
/// bypass of the value-producing rule.
fn specialized_operands_at(
    operands: &[LoweringOperand],
    edge: &'static str,
) -> Result<Vec<Lowered>, CraneliftBackendError> {
    operands
        .iter()
        .map(|operand| operand.specialized_ref_at(edge).cloned())
        .collect()
}

/// [`specialized_operands_at`] for the arguments of a **constructor template**,
/// producing that template's closed [`ConstructorField`] kinds.
///
/// **Every operand becomes [`ConstructorField::Specialized`], with no
/// exception, and that stays true with the producer armed.** A
/// [`LoweringOperand`] is a lowered value, and recognition happens *before* an
/// argument is lowered — so a static worker never becomes an operand and can
/// never arrive here. An operand reaching this function that names a worker has
/// already refused upstream at [`LoweringEnvironmentBinding::value_at`].
///
/// **The armed seam is NOT this function**, which is the correction worth
/// leaving: `D2k-1b-i0` predicted that `D2k-1b-i` would arm it, and arming it
/// would have been arming the wrong end. Recognition has to happen where the
/// *source argument* is still in hand, not where its lowered operand is — by
/// then the value-producing read has already been taken. The armed producer is
/// `Lowering::static_worker_constructor_template`, which builds the mixed
/// template itself and never routes through here.
fn specialized_constructor_fields_at(
    operands: &[LoweringOperand],
    edge: &'static str,
) -> Result<Vec<ConstructorField>, CraneliftBackendError> {
    Ok(specialized_operands_at(operands, edge)?
        .into_iter()
        .map(ConstructorField::specialized)
        .collect())
}

/// The ordinary values behind a constructor template's fields, or a refusal
/// naming the reader.
///
/// **The read direction of [`specialized_constructor_fields_at`]**, for readers
/// that genuinely need a value out of every field.
///
/// **The static `Match` elimination sites are NO LONGER among them.** They took
/// this function until `D2k-1b-i`, and converting was exactly what a worker
/// field could not survive: it has no value representation, so conversion could
/// only refuse. They call [`Lowering::bound_constructor_fields`] instead, which
/// installs each field into the one lexical binding authority *without erasing
/// its kind* — a site that must preserve the worker stops converting rather
/// than converting more cleverly.
///
/// **This is a per-reader refusal, and it is NOT the boundary.** It is reached
/// *during* descent, which is exactly the *"descends partway and then refuses"*
/// shape the ruling forbids as a substitute for whole-graph preflight. The
/// boundary is `Lowering::source_aggregate_preflight` with
/// [`Lowered::boundary_transfer_admissibility`] ahead of every allocation, and
/// [`Lowering::require_complete_static_worker_disposition`] ahead of emission.
fn specialized_fields_at(
    fields: &[ConstructorField],
    edge: &'static str,
) -> Result<Vec<Lowered>, CraneliftBackendError> {
    fields
        .iter()
        .map(|field| field.specialized_at(edge).cloned())
        .collect()
}


/// [`specialized_fields_at`] without the clone, for readers that only borrow
/// the fields — a preflight walk, a shape comparison, a tag read.
fn specialized_field_refs_at<'a>(
    fields: &'a [ConstructorField],
    edge: &'static str,
) -> Result<Vec<&'a Lowered>, CraneliftBackendError> {
    fields
        .iter()
        .map(|field| field.specialized_at(edge))
        .collect()
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2k-1b-i` — the CONSERVATION LEDGER for
/// compiler-only static-worker constructor fields.**
///
/// **The invariant is conservation, and exact-`Var` consumption is a terminal
/// disposition rather than the invariant itself** (Architect
/// `evt_5etamwj8tp2fh`). Every recognized worker at constructor field
/// `(owner, position)` receives **exactly one** disposition before any
/// runtime-value boundary:
///
/// 1. **Consume** — a static elimination of that exact field rebinds the same
///    `StaticWorkerBinding` through [`bound_constructor_fields`], and the
///    pre-existing exact-`Var` callee call consumes it exactly once.
/// 2. **Erase as proven unobservable** — lawful only under a whole-graph,
///    origin-keyed proof, **at or before construction**. ⇒ **No such authority
///    exists in this increment, so this ledger never records one.** A field
///    already built and then ignored is not erasure and earns no consumed
///    credit; erasure would have to prevent the construction.
/// 3. **Refuse** — neither holds, so compilation refuses before emission.
///
/// **The forbidden fourth state is constructed-then-forgotten**, which is what
/// the producer-alone cut shipped: four rows compiled with the worker dropped,
/// and an enumeration of forbidden *uses* was satisfied vacuously because a
/// drop is not a use. [`Self::close`] is the total that closes it — it does not
/// ask what happened to the field, it asks whether **every** recognized field
/// reached a disposition, and refuses on the complement.
///
/// **Why a ledger and not a local check.** The recognition and the elimination
/// are in different descents, and on the measured population they are in
/// different *routes* — so no site local to either can see both. This is the
/// same shape as the `D3` continuation claim ledger and the `D2f` fused-region
/// ledger, and it closes beside them.
/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2k-1b-i` — the CONSERVATION LEDGER for
/// compiler-only static-worker constructor fields.**
///
/// **The invariant is conservation, and exact-`Var` consumption is a terminal
/// disposition rather than the invariant itself** (Architect
/// `evt_5etamwj8tp2fh`). Every recognized worker at constructor field
/// `(owner, position)` receives **exactly one** disposition before any
/// runtime-value boundary:
///
/// 1. **Consume** — a static elimination of that exact field rebinds the same
///    `StaticWorkerBinding` through [`Lowering::bound_constructor_fields`], and
///    the pre-existing exact-`Var` callee call consumes it exactly once.
/// 2. **Erase as proven unobservable** — lawful only under a whole-graph,
///    origin-keyed proof, **at or before construction**. ⇒ **No such authority
///    exists in this increment, so this ledger never records one.** A field
///    already built and then ignored is not erasure and earns no consumed
///    credit; erasure would have to prevent the construction.
/// 3. **Refuse** — neither holds, so compilation refuses before emission.
///
/// **The forbidden fourth state is constructed-then-forgotten**, which is what
/// the producer-alone cut shipped: four rows compiled with the worker dropped,
/// and an enumeration of forbidden *uses* was satisfied vacuously because a
/// drop is not a use.
///
/// **Every relation here is per occurrence, keyed by the planner's
/// `child_static_origin(owner, position)`, and never an aggregate.** The first
/// attempt kept the recognized origins but tallied consuming calls in one
/// compile-wide scalar, accepting whenever the tally reached the entry count.
/// Architect `dec_2xxj1zrwmgjdb` rejected it: that accepts a compile in which
/// one transported field is dropped while another is called twice, or in which
/// an unrelated pre-existing worker call supplies the count, and it cannot see
/// excess consumption at all. **A count over a population is not a pairing
/// within it** — and a documented limit is not a discharged claim.
///
/// **Why a ledger and not a local check.** The recognition and the elimination
/// are in different descents, and on the measured population they are in
/// different *routes* — so no site local to either can see both. This is the
/// same shape as the `D3` continuation claim ledger and the `D2f` fused-region
/// ledger, and it closes beside them.
/// **`D2k-1c-0` REPLACED THE COUNTERS WITH RELATIONS, and the reason is that
/// narrowing a count's scope never turns a tally into a pairing.** The rejected
/// ledger counted compile-wide; its successor counted per `field_origin`; both
/// are tallies. `D2k-1c-0` measured that one static occurrence **is** descended
/// more than once in a single compile — see
/// `d2k_1c_0_one_static_occurrence_is_descended_more_than_once_in_one_compile`
/// — so at `rebinds = 2`, transport #1 consumed twice with transport #2
/// dropped balanced at `2 == 2` and closed green. **A pairing needs a fact
/// saying WHICH transport was paid, and two `usize`s cannot carry one.**
/// Architect `evt_2npnrzesz3t65` ruled the representation below.
#[derive(Default)]
struct StaticWorkerFieldLedger {
    /// **The recognitions.** One entry per *constructed* worker field, keyed by
    /// a fresh [`StaticWorkerRecognitionId`].
    ///
    /// **NOT keyed by `field_origin`.** It was, with `or_insert`, and that
    /// silently dropped the second construction of one occurrence: `recognize`,
    /// `recognize`, one `rebind`, one consumption, close **green** — with a
    /// constructed worker forgotten before any transport existed to owe for it.
    /// The candidate's own `row1` measurement said "two instances" while its
    /// ledger held one. Architect `evt_3manpp82emcq6`.
    ///
    /// **The CAUSE of the repeated construction is unmeasured, and nothing
    /// here should be read as naming one.** An earlier revision attributed it to
    /// *"a speculative descent and the descent that keeps its result"*; that is
    /// one shape that fits and it was never established. What is measured is
    /// that it happens. **The ledger is correct either way** — that is the point
    /// of accounting per instance rather than per cause — and Architect
    /// `evt_2npnrzesz3t65` closed the question a cause would have borne on:
    /// identity is minted at the transition, a discarded descent does not erase
    /// a constructed worker, and there is no implicit rollback.
    recognized: BTreeMap<StaticWorkerRecognitionId, RecognizedStaticWorkerField>,
    /// **The transitions**: which recognition became which transport. The law
    /// is `dom(transitioned) = dom(recognized)`, so a constructed worker that
    /// never entered binding authority is caught here rather than being
    /// unrepresentable. **Its VALUES are governed too, since `D2k-1c-1`** —
    /// this map and `minted` must name each other, which is the join the chain
    /// argument rests on and the one link that used to have no law.
    transitioned: BTreeMap<StaticWorkerRecognitionId, StaticWorkerTransportId>,
    /// **The transports.** One entry per *dynamic* successful `rebind`, keyed
    /// by a fresh [`StaticWorkerTransportId`]. Two rebinds of one
    /// `field_origin` are two entries here, which is exactly the distinction
    /// the counters could not make.
    minted: BTreeMap<StaticWorkerTransportId, MintedStaticWorkerTransport>,
    /// **The consumptions**, keyed by the transport they discharge and valued
    /// by the exact consumer occurrence. A second consumption of one key is
    /// refused at the call rather than absorbed into a total.
    consumed: BTreeMap<StaticWorkerTransportId, StaticOriginId>,
    issuer: transport_identity::TransportIdIssuer,
    recognitions: transport_identity::RecognitionIdIssuer,
}

/// One constructed constructor field carrying a static worker.
struct RecognizedStaticWorkerField {
    /// Provenance only. **Never the identity** — two constructions of one
    /// occurrence share this value and are still two constructed fields.
    field_origin: StaticOriginId,
    owner: StaticOriginId,
    position: usize,
    constructor: String,
    /// The generated function body this field was constructed into. A
    /// transition from another body is a provenance failure, on the same
    /// reading as the transport's own scope.
    scope: Option<FuncId>,
}

/// One transport: a recognized field entering lexical binding authority.
struct MintedStaticWorkerTransport {
    /// The recognition this transport discharges. One transport per
    /// transitioned recognition, so the chain is
    /// **construct -> transition -> consume** with an identity at each link.
    recognition: StaticWorkerRecognitionId,
    /// Provenance only. **Never the identity** — two transports of one
    /// occurrence share this value and are still two transports.
    field_origin: StaticOriginId,
    owner: StaticOriginId,
    position: usize,
    constructor: String,
    /// **The evidence scope: the generated function body this transport was
    /// minted into.** A consumption from a different body is a provenance
    /// failure, not a discharge — repeated lowering in different generated
    /// functions mints distinct IDs, and carrying one across that boundary
    /// without a new rebind is precisely what this refuses. Same field, and
    /// same reading of it, as the `D7` aggregate-allocation ledger: evidence
    /// scope, never planner authority.
    scope: Option<FuncId>,
}

/// **The transport identity, opaque BECAUSE the field is private to an inner
/// module and for no weaker reason.**
///
/// A private-field newtype declared *beside* its users refuses nothing: every
/// sibling in the same module can still write `StaticWorkerTransportId(7)`, so
/// "only the issuer mints" would be a comment rather than a rule. The ledger and
/// the binder both live in this module, so the type and its issuer are moved
/// **inside** one — where the privacy is load bearing and `mint` is the only
/// reachable constructor.
mod transport_identity {
    /// An opaque compiler-only transport identity. **Not a runtime word, tag,
    /// descriptor, carrier or callable identity**, and never emitted.
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub(super) struct StaticWorkerTransportId(u64);

    /// An opaque compiler-only **recognition** identity: one per successful
    /// `ConstructorField::StaticWorker` construction.
    ///
    /// **A SEPARATE identity from the transport, and separate from the
    /// planner origin, because it answers a third question.** The planner origin
    /// names the *occurrence*; the transport names one entry into lexical
    /// binding authority; this names one **constructed field**. Keying
    /// recognition by origin collapsed two constructed workers of one occurrence
    /// into one record, so the second disappeared before any transport identity
    /// existed to account for it — green close, worker forgotten, which is the
    /// forbidden fourth state one step earlier than the counters allowed it.
    /// Architect `evt_3manpp82emcq6`.
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub(super) struct StaticWorkerRecognitionId(u64);

    /// **The checked monotone issuer.** Fresh for every dynamic successful
    /// rebind, including repeated rebinds of the same `field_origin`.
    #[derive(Default)]
    pub(super) struct TransportIdIssuer {
        next: u64,
    }

    impl TransportIdIssuer {
        /// `None` on exhaustion rather than a wrap, so the one way to get a
        /// duplicate identity is a refusal instead of a silent collision.
        pub(super) fn mint(&mut self) -> Option<StaticWorkerTransportId> {
            let issued = StaticWorkerTransportId(self.next);
            self.next = self.next.checked_add(1)?;
            Some(issued)
        }
    }

    /// **The checked monotone issuer for recognitions.** Fresh for every
    /// constructed worker field, including repeated constructions of the same
    /// `field_origin` — which `D2k-1c-0` measured happening on `row1`.
    #[derive(Default)]
    pub(super) struct RecognitionIdIssuer {
        next: u64,
    }

    impl RecognitionIdIssuer {
        pub(super) fn mint(&mut self) -> Option<StaticWorkerRecognitionId> {
            let issued = StaticWorkerRecognitionId(self.next);
            self.next = self.next.checked_add(1)?;
            Some(issued)
        }
    }
}

use transport_identity::{StaticWorkerRecognitionId, StaticWorkerTransportId};

impl StaticWorkerFieldLedger {
    /// A producer recognized a worker and built the field.
    /// **A producer constructed a worker field.** One recognition identity per
    /// construction, minted here and carried by that exact compiler template.
    ///
    /// **Never deduplicated by `field_origin`.** Two constructions of one
    /// occurrence are two constructed workers, each owing its own transition
    /// and its own consumption; collapsing them loses the second before any
    /// transport identity exists to be short.
    fn recognize(
        &mut self,
        owner: StaticOriginId,
        position: usize,
        field_origin: StaticOriginId,
        constructor: &str,
        scope: Option<FuncId>,
    ) -> Result<StaticWorkerRecognitionId, CraneliftBackendError> {
        let Some(recognition) = self.recognitions.mint() else {
            return Err(unsupported(
                "StaticWorkerBinding",
                "the static worker recognition issuer is exhausted; refusing rather than reusing \
                 an identity, because a reused identity would let one constructed field stand in \
                 for another"
                    .to_string(),
            ));
        };
        self.recognized.insert(
            recognition,
            RecognizedStaticWorkerField {
                field_origin,
                owner,
                position,
                constructor: constructor.to_string(),
                scope,
            },
        );
        Ok(recognition)
    }

    /// **Disposition 1, first half — the TRANSPORT EVENT.** A static
    /// elimination is putting this recognized field into lexical binding
    /// authority, so a fresh transport identity is minted here and returned for
    /// the rebound binding to carry.
    ///
    /// **Minted at the rebind, not at some later notion of retention**
    /// (Architect `evt_2npnrzesz3t65`). `rebind` *is* the moment the field
    /// enters the authority; deferring identity until a caller declares the
    /// descent kept would reconstruct it after the binding may already have been
    /// consumed, and would make retention a second authority over what happened.
    /// The converse also holds: **calling a descent speculative does not erase a
    /// worker after construction** — erasure is lawful only under positive
    /// authority at or before construction, and there is none here.
    ///
    /// ⇒ **There is no implicit rollback and no discard API.** On normal
    /// lowering every minted transport must be consumed even if a caller later
    /// ignores the result; on error the compilation aborts. Annulling a mint
    /// would be lawful only through an explicit transaction rolling back the
    /// binding, the emitted/control state and this evidence together, and
    /// nothing measured shows a rollback-and-continue path that would need one.
    ///
    /// **A rebind of a field no producer recognized fails closed.** It cannot
    /// happen while the armed producer is the only builder of the worker arm,
    /// which is precisely why it is worth refusing rather than ignoring: the
    /// day a second builder appears, an unrecognized transport must not be able
    /// to enter the binding authority unaccounted.
    fn rebind(
        &mut self,
        recognition: StaticWorkerRecognitionId,
        scope: Option<FuncId>,
    ) -> Result<StaticWorkerTransportId, CraneliftBackendError> {
        let Some(recognized) = self.recognized.get(&recognition) else {
            return Err(unsupported(
                "StaticWorkerBinding",
                format!(
                    "a static elimination rebound a static worker field carrying recognition \
                     {recognition:?} that this compilation never constructed"
                ),
            ));
        };
        if recognized.scope != scope {
            return Err(unsupported(
                "StaticWorkerBinding",
                format!(
                    "constructor {} at origin {:?} field {} was constructed into one generated \
                     function body and rebound while defining another; a recognized field \
                     carried across a body boundary is provenance failure, not a transition",
                    recognized.constructor, recognized.owner, recognized.position
                ),
            ));
        }
        if let Some(already) = self.transitioned.get(&recognition) {
            return Err(unsupported(
                "StaticWorkerBinding",
                format!(
                    "constructor {} at origin {:?} field {} was constructed once and rebound \
                     twice; its first transition already minted transport {already:?}, and one \
                     constructed field cannot enter binding authority as two transports",
                    recognized.constructor, recognized.owner, recognized.position
                ),
            ));
        }
        let Some(transport) = self.issuer.mint() else {
            return Err(unsupported(
                "StaticWorkerBinding",
                "the static worker transport issuer is exhausted; refusing rather than reusing \
                 an identity, because a reused identity would let one transport discharge \
                 another"
                    .to_string(),
            ));
        };
        self.minted.insert(
            transport,
            MintedStaticWorkerTransport {
                recognition,
                field_origin: recognized.field_origin,
                owner: recognized.owner,
                position: recognized.position,
                constructor: recognized.constructor.clone(),
                scope,
            },
        );
        self.transitioned.insert(recognition, transport);
        Ok(transport)
    }

    /// **Disposition 1, second half — an exact-`Var` call consumes ONE named
    /// transport**, recorded against the exact consumer occurrence.
    ///
    /// Three refusals, and each answers a way the counters were fail-open:
    ///
    /// 1. **Unknown identity** — a binding naming a transport this ledger never
    ///    minted. Nothing outside the issuer can produce one, so this is the
    ///    fail-closed backstop for the day a second minting site appears.
    /// 2. **Cross-scope** — minted while defining one generated function body
    ///    and consumed while defining another. Repeated lowering in different
    ///    bodies mints distinct identities; carrying one across that boundary
    ///    without a new rebind is a provenance failure, **not** a licence to
    ///    consume it again.
    /// 3. **Already consumed** — the exact hole in the per-origin count.
    ///    Consuming transport `A` twice used to cover an outstanding `B`
    ///    because only the *total* was read. Here `A` is already in the
    ///    relation, so the second consumption refuses **at the call**, and `B`
    ///    is still outstanding at close.
    ///
    /// **`None` is a real state, not a missing value.** A binding that never
    /// came out of a constructor field carries no transport and discharges
    /// nothing, so an ordinary pre-existing worker call cannot pay a
    /// transported field's debt.
    fn note_consuming_call(
        &mut self,
        transport: Option<StaticWorkerTransportId>,
        consumer: StaticOriginId,
        scope: Option<FuncId>,
    ) -> Result<(), CraneliftBackendError> {
        let Some(transport) = transport else {
            return Ok(());
        };
        let Some(minted) = self.minted.get(&transport) else {
            return Err(unsupported(
                "StaticWorkerBinding",
                format!(
                    "an exact-Var call at origin {consumer:?} consumed a static worker naming \
                     transport {transport:?}, which this compilation never minted"
                ),
            ));
        };
        if minted.scope != scope {
            return Err(unsupported(
                "StaticWorkerBinding",
                format!(
                    "constructor {} at origin {:?} field {} was transported into one generated \
                     function body and consumed at origin {consumer:?} while defining another; a \
                     transport identity carried across a body boundary without a new rebind is \
                     provenance failure, not a discharge",
                    minted.constructor, minted.owner, minted.position
                ),
            ));
        }
        if let Some(first) = self.consumed.get(&transport) {
            return Err(unsupported(
                "StaticWorkerBinding",
                format!(
                    "constructor {} at origin {:?} field {} was transported once and consumed \
                     twice, at origins {first:?} and {consumer:?}; a second consumption of one \
                     transport cannot discharge a different transport's obligation",
                    minted.constructor, minted.owner, minted.position
                ),
            ));
        }
        self.consumed.insert(transport, consumer);
        Ok(())
    }

    /// **The total.** Every recognized worker is consumed exactly once, erased
    /// before construction under positive unobservability authority, or refused
    /// before emission; none is dropped.
    ///
    /// Erasure is structurally absent here — a recognition exists only because
    /// the field was built — so the two reachable outcomes are consume and
    /// refuse.
    ///
    /// **The chain has three links and an identity at each one:**
    /// `dom(transitioned) = dom(recognized)`, then the JOIN — `transitioned` and
    /// `minted` are mutually inverse — then `dom(consumed) = dom(minted)`.
    /// ⇒ **construct -> transition -> consume**, each step per instance. A
    /// constructed field that never transitions is caught by the first law, and
    /// it is the state a recognition map keyed by `field_origin` could not even
    /// represent.
    ///
    /// **`D2k-1c-1` MADE THE JOIN A LAW. It was a claim about `rebind`'s body,
    /// and the chain argument rests on it.** This doc said *"`minted` in
    /// bijection with `transitioned` because one transition mints exactly one
    /// transport"* while every assertion below was keyed on **its own map's
    /// keys** — so nothing checked that a `transitioned` VALUE is a key of
    /// `minted`. `transitioned[r] = T` with `T ∉ minted` passed all four:
    /// `r` has a transition, `r` is recognized, and the two loops over
    /// `minted`/`consumed` never see `T` at all. ⇒ **`close()` returned `Ok`
    /// with a recognized field whose transport was never consumed** — the
    /// constructed-then-forgotten state this ledger exists to make impossible,
    /// admitted by the one link that had no law. Adversary `evt_733esjz2t4bn8`.
    ///
    /// **The containment alone is NOT the fix, and that is the part the finding
    /// did not reach.** `range(transitioned) ⊆ dom(minted)` still admits
    /// `transitioned[r1] = transitioned[r2] = T`: two constructions sharing one
    /// transport, discharged by that transport's single consumption, with the
    /// containment satisfied. The law is therefore the **agreeing** bijection —
    /// `minted[transitioned[r]].recognition == r`, and back — which makes
    /// injectivity a consequence rather than a second check. Both directions are
    /// stated for the same reason the other two pairs are, below.
    ///
    /// **Every law here is a relation and never an equality of cardinalities.**
    /// `consumed ⊆ minted` is enforced at the call: `note_consuming_call`
    /// refuses an identity it never minted, so nothing can enter `consumed` that
    /// is absent from `minted`. `⊇` is enforced here. Both directions are
    /// checked below anyway, because a law worth stating is worth being able to
    /// fail — and the `⊆` re-check is what catches a future second writer of
    /// `consumed` that skips the call-side guard. **That standard is the whole
    /// argument for `D2k-1c-1`:** it was adopted deliberately for a case the
    /// call site already enforced, and not applied to the join, which nothing
    /// enforced anywhere.
    fn close(&self) -> Result<(), CraneliftBackendError> {
        // Link one: every CONSTRUCTED field transitioned into binding
        // authority. Per recognition instance, never "some minted transport
        // shares this origin" -- that is what let a second construction of one
        // occurrence vanish while the first covered for it.
        for (recognition, recognized) in &self.recognized {
            if !self.transitioned.contains_key(recognition) {
                return Err(unsupported(
                    "StaticWorkerBinding",
                    format!(
                        "constructor {} at origin {:?} transports a static worker in field {} \
                         (field origin {:?}, recognition {recognition:?}) that no static \
                         elimination rebinds, so this recognition's own transport never reaches \
                         a consumer at an exact-Var call and is not erased; a constructor carrying \
                         an unconsumed static worker denotes a value containing the callable and \
                         has no runtime representation",
                        recognized.constructor,
                        recognized.owner,
                        recognized.position,
                        recognized.field_origin
                    ),
                ));
            }
        }
        for (recognition, transport) in &self.transitioned {
            if !self.recognized.contains_key(recognition) {
                return Err(unsupported(
                    "StaticWorkerBinding",
                    format!(
                        "transport {transport:?} is recorded as the transition of recognition \
                         {recognition:?}, which this compilation never constructed"
                    ),
                ));
            }
            // THE JOIN, forward. The transition is the only record that a
            // construction owes anything, and the two laws around it are keyed
            // on their own maps -- so a transport named here and absent from
            // `minted` is an obligation no later loop can see.
            let Some(minted) = self.minted.get(transport) else {
                return Err(unsupported(
                    "StaticWorkerBinding",
                    format!(
                        "recognition {recognition:?} is recorded as transitioning into transport \
                         {transport:?}, which this compilation never minted; the transitions and \
                         the transports must name each other, or a constructed field's obligation \
                         is recorded against an identity nothing else quantifies over"
                    ),
                ));
            };
            // THE JOIN, agreeing -- and this is the half a containment misses.
            // `range(transitioned) subset dom(minted)` still admits two
            // recognitions naming ONE transport, whose single consumption then
            // discharges both: constructed-then-forgotten with the containment
            // satisfied. Requiring the transport to name THIS recognition back
            // makes injectivity a consequence rather than another check.
            if minted.recognition != *recognition {
                return Err(unsupported(
                    "StaticWorkerBinding",
                    format!(
                        "recognition {recognition:?} is recorded as transitioning into transport \
                         {transport:?}, which was minted for recognition {:?}; one transport \
                         cannot be the transition of two constructed fields, because its single \
                         consumption would discharge both and leave one forgotten",
                        minted.recognition
                    ),
                ));
            }
        }
        for (transport, minted) in &self.minted {
            // THE JOIN, back. A transport whose recognition does not transition
            // to it is a construction that entered binding authority twice --
            // `rebind` refuses the second, and this is that refusal restated as
            // a law the close can fail on rather than a property of one writer.
            if self.transitioned.get(&minted.recognition) != Some(transport) {
                return Err(unsupported(
                    "StaticWorkerBinding",
                    format!(
                        "transport {transport:?} was minted for recognition {:?}, which does not \
                         transition to it; a constructed field enters binding authority exactly \
                         once, so a second transport standing behind one recognition is an \
                         obligation with no construction behind it",
                        minted.recognition
                    ),
                ));
            }
            // ---- `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — THE ARMED COMPILE NOW
            // ---- STOPS HERE, and this arm had never been evaluated on this
            // ---- witness before.
            //
            // Until the two-member binder wiring landed at the composed
            // eliminator (`core.rs`, the reversed IH prefix), the armed compile
            // refused *earlier* -- at `specialized_at`, before it ever assembled
            // the case run -- so no transport was minted on this witness and this
            // close was never reached with a live obligation. **The red is the
            // increment working, not a regression it introduced.**
            //
            // MEASURED, identically on both armed roots (`Exact`, `ReHomed`),
            // by instrumenting the assembled run and every exact-`Var` call:
            //
            // ```text
            // env[0] = Worker(body=34, transport=None)      <- induction hypothesis
            // env[1] = Worker(body=34, transport=Some(0))   <- selected recursive field
            // the ONLY exact-Var call in the case body: Var(0)  -> transport None
            // ```
            //
            // ⇒ Two distinct members over the **same** joined worker, exactly as
            // ruled, with the single transport on segment 2 alone -- and the case
            // body's own recursive call names the *hypothesis*. So the transport
            // is minted and nothing consumes it, and this close fires.
            //
            // **CLAIMED by this refusal:** the worker is dropped. **THE GAP:** on
            // this witness it is not. The identical body is bound at `env[0]` and
            // *is* called; what no consumption exists for is this particular
            // transport identity. The refusal's own last clause -- *"no other
            // transport's consumption discharges it"* -- is true and is precisely
            // the thing in question, because the sibling member carries no
            // transport at all rather than a second one.
            //
            // ⛔ **Two landed laws collide here and neither is wrong.** `D6a`
            // requires ALL constructor arguments to be bound, the selected
            // recursive one included, or every later binder shifts by one. The
            // conservation ledger requires every rebound transport to be consumed,
            // or a worker is silently discarded. A body that reaches its recursive
            // producer through the hypothesis satisfies the first and violates the
            // second. The specialization sibling in `units.rs` cannot see this: it
            // builds both members through `construct_static_worker_binding`, so
            // NEITHER carries a transport and no obligation is ever opened.
            //
            // ⛔ **Do not repair this by moving the transport onto the hypothesis.**
            // It makes this witness green and mirrors the same red onto any body
            // that does reference the field, and the ruling assigns the
            // recognition-to-rebind transport to segment 2 alone. Whether a
            // recognition that produced two members is discharged by one
            // consumption among them is a ledger-shape question, and it is routed
            // rather than taken here.
            if !self.consumed.contains_key(transport) {
                return Err(unsupported(
                    "StaticWorkerBinding",
                    format!(
                        "constructor {} at origin {:?} field {} (field origin {:?}) was rebound \
                         into the binding authority as transport {transport:?} and never \
                         consumed at an exact-Var call; a transported static worker that is not \
                         called is dropped, and no other transport's consumption discharges it",
                        minted.constructor, minted.owner, minted.position, minted.field_origin
                    ),
                ));
            }
        }
        for (transport, consumer) in &self.consumed {
            if !self.minted.contains_key(transport) {
                return Err(unsupported(
                    "StaticWorkerBinding",
                    format!(
                        "an exact-Var call at origin {consumer:?} is recorded as consuming \
                         transport {transport:?}, which this compilation never minted"
                    ),
                ));
            }
        }
        Ok(())
    }
}

/// **The KIND-PRESERVING static `Match` binder** — a constructor template's
/// fields entering the one lexical binding authority without losing the
/// distinction the template drew.
///
/// **This is the consumer half of `D2k-1b-i`, and it is deliberately not a
/// cleverer [`specialized_fields_at`].** That function converts every field to
/// a [`Lowered`], which is precisely what a static-worker field cannot survive:
/// it has no value representation, so conversion can only refuse. The repair is
/// to stop converting — an ordinary field becomes
/// `Value(Specialized(..))` exactly as it always did, and a worker field
/// becomes the **same** [`LoweringEnvironmentBinding::StaticWorker`] it was
/// bound as before it entered the constructor.
///
/// ⇒ **The consumer is not new.** Once the worker is back in the binding
/// authority, the pre-existing exact-`Var` call arm is the only thing that can
/// read it, and every other use still fails closed at
/// [`LoweringEnvironmentBinding::value_at`] — which this function does not
/// touch. The repair installs the binding; it does not widen what may consume
/// one.
impl Lowering<'_> {
    fn bound_constructor_fields(
        &mut self,
        fields: &[ConstructorField],
        outer: &[LoweringEnvironmentBinding],
    ) -> Result<Vec<LoweringEnvironmentBinding>, CraneliftBackendError> {
        let mut bindings = self.constructor_field_bindings(fields)?;
        bindings.extend(outer.iter().cloned());
        Ok(bindings)
    }

    /// [`Self::bound_constructor_fields`] for the sites that append to an
    /// environment they are already building, rather than building one in front
    /// of a spine.
    fn extend_constructor_fields(
        &mut self,
        env: &mut Vec<LoweringEnvironmentBinding>,
        fields: &[ConstructorField],
    ) -> Result<(), CraneliftBackendError> {
        let bindings = self.constructor_field_bindings(fields)?;
        env.extend(bindings);
        Ok(())
    }

    /// The one place a [`ConstructorField`] becomes a
    /// [`LoweringEnvironmentBinding`], so both binder shapes above spell the
    /// kind-preservation identically rather than each re-deriving it, and so
    /// **the conservation ledger is marked exactly where the rebinding happens**
    /// rather than at each caller.
    ///
    /// **The match is exhaustive with no wildcard**, for the same reason every
    /// other field reader's is: a future third field kind must be a compile
    /// error at this binder rather than silently taking the ordinary arm.
    fn constructor_field_bindings(
        &mut self,
        fields: &[ConstructorField],
    ) -> Result<Vec<LoweringEnvironmentBinding>, CraneliftBackendError> {
        let mut bindings = Vec::with_capacity(fields.len());
        for (position, field) in fields.iter().enumerate() {
            bindings.push(match field {
                ConstructorField::Specialized(value) => {
                    LoweringEnvironmentBinding::Value(LoweringOperand::Specialized(value.clone()))
                }
                ConstructorField::StaticWorker {
                    binding,
                    field_origin,
                    recognition,
                } => {
                    // **THE TRANSPORT EVENT.** The field enters lexical
                    // binding authority here, so the identity is minted here —
                    // scoped to the generated function body being defined, so a
                    // consumption from another body is a provenance failure
                    // rather than a discharge.
                    let transport = self
                        .static_worker_fields
                        .rebind(*recognition, self.defining_function_id)?;
                    #[cfg(test)]
                    record_d2k_owner_event(D2kOwnerEvent::StaticWorkerBinderInstalled {
                        field_origin: *field_origin,
                        position,
                    });
                    // **The transport identity travels WITH the binding**, so
                    // the exact-`Var` call that consumes it names the one
                    // transport it discharges. A field key here would name the
                    // occurrence, which is one transport too coarse: the same
                    // occurrence is rebound more than once in a single compile.
                    LoweringEnvironmentBinding::StaticWorker(StaticWorkerBinding {
                        transport: Some(transport),
                        ..binding.clone()
                    })
                }
            });
        }
        Ok(bindings)
    }

    /// **The conservation close** — run at the end of the unit-definition pass,
    /// beside the `D3` causal and `D2f` fused-region ledgers and for the same
    /// reason: the disposition of a field recognized in one descent can only be
    /// known once every descent is done.
    ///
    /// On the retired monolithic route it ran before emission of the root
    /// answer. On the surviving functionized route it does not, deliberately:
    /// `define_root_adapter` is called above it (`core.rs:2581` versus
    /// `:2614`), because the adapter is itself a generated `Function` and
    /// closing the causal ledgers before it would make a ref declared there
    /// invisible to the laws rather than caught by them. **The ordering there
    /// belongs to the causal ledger and is not this close's to move.**
    ///
    /// **The refusal's lawfulness does not rest on that ordering — the sentence
    /// below carries it, and it holds on both authorities.** A dropped field
    /// allocates nothing by definition, and a field that would have escaped into
    /// a runtime aggregate has already been refused ahead of the first
    /// allocation by `source_aggregate_preflight` and
    /// [`Lowered::boundary_transfer_admissibility`]. This close covers the
    /// remaining state those two cannot see — built, never used, never refused.
    /// The unqualified claim was a **wrong reason standing beside a right one**,
    /// which is the shape the next auditor inherits (adversary sweep, Steward
    /// `evt_55rzfnc1gkekq`).
    fn require_complete_static_worker_disposition(&self) -> Result<(), CraneliftBackendError> {
        self.static_worker_fields.close()
    }
}

/// [`specialized_operands_at`] for a **lexical environment** rather than a bare
/// operand list.
///
/// The two are deliberately distinct functions. This one crosses the binding
/// authority first, so a [`LoweringEnvironmentBinding::StaticWorker`] fails
/// closed before any template is read; [`specialized_operands_at`] takes an operand
/// list -- a constructor's arguments, a closure's captures, a primitive's
/// operands -- which is not an environment and has no binding to classify.
/// Collapsing them would put a value-producing read on a surface that has no
/// binding to reject.
fn specialized_bindings_at(
    env: &[LoweringEnvironmentBinding],
    edge: &'static str,
) -> Result<Vec<Lowered>, CraneliftBackendError> {
    env.iter()
        .map(|binding| {
            #[cfg(test)]
            record_d2k_owner_event(D2kOwnerEvent::ValueAtCaller {
                site: "mod.rs environment values (forwards its caller's edge)",
            });
            binding.value_at(edge)?.specialized_ref_at(edge).cloned()
        })
        .collect()
}

/// ⭐ **THE ONE WAY A LOWERING ENVIRONMENT IS BUILT** — freshly bound
/// specialized values in front of the enclosing spine.
///
/// ⭐ **The spine is forwarded UNCHANGED, and that is the `§2h` property, not
/// an implementation detail.** The ruling's control clause asks that *"a
/// projected `Carried` child remains `Carried` through `case_env` and nested
/// lowering"*; that holds here **structurally** — `outer` is already
/// `[LoweringOperand]` and every element is cloned, so there is no arm in which
/// a carried operand could be re-specialized or dropped on the way in.
///
/// `bindings` are `Lowered` because a **binder introduces a compile-time
/// value**: a case's constructor fields, a `Let`'s value, a closure's captures.
/// ⚠ When a binder can be `Carried` — the projected child of an eliminated
/// carrier — it does **not** come through this parameter; it is already an
/// operand and is prepended as one.
fn env_with(
    bindings: impl IntoIterator<Item = Lowered>,
    outer: &[LoweringEnvironmentBinding],
) -> Vec<LoweringEnvironmentBinding> {
    bindings
        .into_iter()
        .map(|lowered| LoweringEnvironmentBinding::Value(LoweringOperand::Specialized(lowered)))
        .chain(outer.iter().cloned())
        .collect()
}

/// ⭐ **The phase-PRESERVING environment constructor** — for bindings that are
/// already operands.
///
/// ⚠ This is the one that matters for `§2h`'s control clause. [`env_with`]'s
/// bindings are templates a binder just introduced; **these** are values the
/// lowering produced, and a projected `Carried` child arrives here. There is no
/// arm that re-specializes or drops one — it is moved into the environment as
/// it stands, which is what makes *"remains `Carried` through `case_env` and
/// nested lowering"* a structural fact rather than a tested one.
/// Bind already-lowered operands into the one binding authority, in order.
///
/// This is the phase-preserving entry every pre-existing binder takes: an
/// operand becomes [`LoweringEnvironmentBinding::Value`] and nothing else, so
/// a projected `Carried` child stays carried on the way in. It exists so the
/// many sites that build an environment out of lowered operands all spell the
/// installation the same way rather than each re-deriving it.
fn bound_values(
    operands: impl IntoIterator<Item = LoweringOperand>,
) -> Vec<LoweringEnvironmentBinding> {
    operands
        .into_iter()
        .map(LoweringEnvironmentBinding::Value)
        .collect()
}

fn env_with_operands(
    bindings: impl IntoIterator<Item = LoweringOperand>,
    outer: &[LoweringEnvironmentBinding],
) -> Vec<LoweringEnvironmentBinding> {
    bindings
        .into_iter()
        .map(LoweringEnvironmentBinding::Value)
        .chain(outer.iter().cloned())
        .collect()
}

/// Append specialized bindings **after** operands already in an environment —
/// the same rule as [`env_with`], for the sites whose leading bindings are
/// themselves lowered operands (a call's arguments) and whose trailing ones are
/// a closure's captured templates.
/// **`D8d` — how many target-derived static-worker bindings were installed.**
///
/// ⛔ An INSTALLATION counter, not a consumption one. `D8e` owns consumption and
/// this says nothing about it. It exists because "the binding is intentionally
/// unreadable" and "the binding was never built" are indistinguishable from the
/// outside, and only one of them is the checkpoint.
#[cfg(test)]
thread_local! {
    static D8D_STATIC_WORKER_BINDINGS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// **`D8d` — how many times the composed site reached a RECURSIVE position.**
///
/// The denominator to `D8D_STATIC_WORKER_BINDINGS`' numerator. Without it,
/// "no binding installed" cannot be told apart from "the site was never
/// reached", and those have completely different consequences for `D8e`.
#[cfg(test)]
thread_local! {
    static D8D_RECURSIVE_SITES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8d_record_site() {
    D8D_RECURSIVE_SITES.with(|count| count.set(count.get() + 1));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8d_recursive_sites() -> usize {
    D8D_RECURSIVE_SITES.with(std::cell::Cell::get)
}

/// **`D8e` — how many source-machine `Var` callees resolved to a `D8d`
/// binding and were consumed through the shared emitter.**
///
/// The third counter in the pair `D8d` established, and it exists for the same
/// reason: "the consumer is correct but unreached" and "the consumer is wrong"
/// are indistinguishable from a green suite.
#[cfg(test)]
thread_local! {
    static D8E_CONSUMPTIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8e_record_consumption() {
    D8E_CONSUMPTIONS.with(|count| count.set(count.get() + 1));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8e_consumptions() -> usize {
    D8E_CONSUMPTIONS.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8d_record_binding() {
    D8D_STATIC_WORKER_BINDINGS.with(|count| count.set(count.get() + 1));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d8d_bindings() {
    D8D_STATIC_WORKER_BINDINGS.with(|count| count.set(0));
    D8D_RECURSIVE_SITES.with(|count| count.set(0));
    D8E_CONSUMPTIONS.with(|count| count.set(0));
    D8I_DISCHARGES.with(|log| log.borrow_mut().clear());
    D8L2_CONSUMED_FACETS.with(|log| log.borrow_mut().clear());
}

/// **`D8i` — what discharge facet each constructed binding actually carried.**
///
/// ⛔ An OBSERVATION of the transported value, not a second authority: the
/// record is written inside the constructor, from the argument the call site
/// supplied, after every validation has passed. It exists because `D8i` is
/// transport with no production reader until `D8j`, so "both arms occur, each
/// at the site its role dictates" is otherwise unobservable — and a facet that
/// were defaulted or inferred rather than stated would be indistinguishable
/// from one that is stated correctly.
///
/// `None` is [`ContinuationDischarge::DirectSpecializationCall`]. The composed
/// arm records the identity's two readable facts; the call-site sequence stays
/// unread here as everywhere else.
#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct D8iDischargeRecord {
    pub(in crate::cranelift_backend) body_origin: StaticOriginId,
    pub(in crate::cranelift_backend) composed:
        Option<(ContinuationEmissionOwner, ContinuationSpecializationId)>,
}

#[cfg(test)]
thread_local! {
    static D8I_DISCHARGES: std::cell::RefCell<Vec<D8iDischargeRecord>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8i_discharges() -> Vec<D8iDischargeRecord> {
    D8I_DISCHARGES.with(|log| log.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d8i_discharge(record: D8iDischargeRecord) {
    D8I_DISCHARGES.with(|log| log.borrow_mut().push(record));
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8j` — the composed-discharge defects.**
///
/// ⛔ Every one perturbs an INPUT the seat or the verifier is handed, never the
/// check itself. Where an identity is substituted it is a real one taken from
/// the planned population, because `ContinuationCallIdentity` has no
/// constructor outside planning.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D8jMutation {
    Exact,
    /// Emit the raw call and record nothing. The relation stays empty on a
    /// program that should have populated it.
    SuppressDischargeAfterRealCall,
    /// Present a different exact identity from the same population — the answer
    /// a same-symbol shortcut would give, since the witness's targets share one
    /// constructor symbol.
    SubstituteAnotherExactIdentity,
    /// Record some other instruction than the raw-worker call that was written.
    RedirectRecordedInstruction,
    /// Claim from a function that is not the identity's emission owner.
    WrongClaimingOwner,
    /// Attempt the composed discharge on an ordinary binding.
    DischargeFromOrdinaryBinding,
    /// **Verification 4b's discriminator.** Move the emitter's reported operand
    /// run away from the run its `D8b`/`D8d` target declares, and move nothing
    /// else.
    ///
    /// ⛔ Applied to `StaticWorkerEmission.supplied_operands` AFTER the call has
    /// assembled and emitted its operand vector, so the vector that was written
    /// is the real one and only the evidence about it disagrees. Identity,
    /// paired target, owner, recorded instruction, decoded callee, downstream
    /// result and source control all stay correct, which is what makes 4b the
    /// FIRST refusal rather than a consequence of an earlier one.
    ///
    /// ⛔ A whole-target substitution is NOT a control for 4b: verification 1 or
    /// 4a would refuse first and mask it. That is why the perturbation is of the
    /// evidence and not of the target.
    ///
    /// ⚠ **The delta is arbitrary, and that is a property of this witness, not
    /// a choice.** Both of its workers declare arity 1 with no captures, so no
    /// adjacent real quantity differs from the true run to perturb toward. What
    /// carries the control is the isolation -- one field moves and every other
    /// verifier input stays exact -- not the value.
    SupplyOperandCountDisagreesWithTarget,
    /// Record a result value defined BEFORE the call, so the value the
    /// continuation is said to have received cannot have come from it.
    ///
    /// ⛔ A real value of the finished function, taken from the first
    /// instruction that defines one -- not a fabricated `Value`, which would be
    /// caught by a bounds check rather than by the downstream relation.
    RecordResultDefinedBeforeTheCall,
}

#[cfg(test)]
thread_local! {
    static D8J_MUTATION: std::cell::Cell<D8jMutation> =
        const { std::cell::Cell::new(D8jMutation::Exact) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn set_d8j_mutation(mutation: D8jMutation) {
    D8J_MUTATION.with(|cell| cell.set(mutation));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8j_mutation() -> D8jMutation {
    D8J_MUTATION.with(std::cell::Cell::get)
}

/// **`D8j`** — the verified composed-discharge relation of the function most
/// recently finalized, for the rows that assert it was populated.
#[cfg(test)]
thread_local! {
    static D8J_DISCHARGED: std::cell::RefCell<Vec<ContinuationCallIdentity>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

/// **`D8l2` — the discharge facet each source-machine consumption carried.**
///
/// ⛔ Recorded at the CONSUMPTION seat, not at construction. `D8I_DISCHARGES`
/// already says what each binding was built with; this says what was actually
/// consumed, and the two populations differ -- a binding can be built and never
/// consumed, and a program can reach the seat with bindings from several
/// sources. `true` is a composed authority.
#[cfg(test)]
thread_local! {
    static D8L2_CONSUMED_FACETS: std::cell::RefCell<Vec<bool>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d8l2_consumed_facet(composed: bool) {
    D8L2_CONSUMED_FACETS.with(|log| log.borrow_mut().push(composed));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8l2_consumed_facets() -> Vec<bool> {
    D8L2_CONSUMED_FACETS.with(|log| log.borrow().clone())
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8n` — what the two checked seams actually
/// saw, per compilation.**
///
/// ⛔ Written AT the seams from state production already holds there, never
/// rebuilt from a fixture or an expected plan. The point of the split-body
/// witness is that one source body is lowered under two different defining
/// Functions; an observation that reconstructed the identity would be a second
/// authority for the very fact under test. Each record carries the **defining
/// `FuncId`** -- never an emission owner.
///
/// ⚠ The identity is `defining_function_id`, set by `open_aggregate_events` at
/// the START of every body -- the one per-`Function` fact in scope at both
/// seams.
///
/// **Pre-`D8o` history, retained because it is why this field was chosen:**
/// `defining_emission_owner` was then set only by the ordinary unit-body and
/// generated-context passes, so it was **stale** inside a specialization body,
/// and both consumptions reported the same owner while genuinely occurring in
/// two different Functions. ⭐ `D8o` repaired that: all three source-bearing
/// body kinds now bind both ambient fields for their own lifetime and restore
/// on exit, so the current invariant is that the owner is the planner's for the
/// body being emitted. `defining_function_id` remains the right identity here
/// regardless -- it answers "which module definition", which is the question
/// these two seams ask.
#[cfg(test)]
thread_local! {
    static D8N_FRAME_CONSUMPTIONS: std::cell::RefCell<Vec<(Option<FuncId>, u64, u64)>> =
        const { std::cell::RefCell::new(Vec::new()) };
    static D8N_SLOT_RECONCILIATIONS: std::cell::RefCell<Vec<(Option<FuncId>, u64)>> =
        const { std::cell::RefCell::new(Vec::new()) };
    /// `D8m` — the pair the slot seam actually reconciled: the checked frame id
    /// the bridge transported, and the plan slot template it was held to. Kept
    /// separate from `D8N_SLOT_RECONCILIATIONS` so `D8n`'s accepted evidence
    /// keeps the exact tuple it was reviewed with.
    ///
    /// This is the relation a witness with ONE checked occurrence cannot
    /// discriminate: there, frame and slot are both singletons and any pairing
    /// of them is the same pairing. With two distinct occurrences the two
    /// subjects hold distinct values, so exchanging them is visible here and
    /// invisible to a bag of frames beside a bag of slots.
    static D8M_SLOT_FRAME_PAIRS: std::cell::RefCell<Vec<(Option<FuncId>, u64, u64)>> =
        const { std::cell::RefCell::new(Vec::new()) };
    /// `D8m` — which arm of the closed bridge descriptor each composed site
    /// actually took. The three arms are disjoint by source constructor, so this
    /// is how "the ordinary and unwrapped populations are unchanged" becomes a
    /// measurement rather than a reading of the match.
    static D8M_BRIDGE_ARMS: std::cell::RefCell<Vec<(Option<FuncId>, D8mBridgeArm)>> =
        const { std::cell::RefCell::new(Vec::new()) };
    /// `D8p` — the PLAN side of each checked-application binding, written at the
    /// seam the moment it binds: the exact defining body, the exact application
    /// occurrence, and the call/slot/binder/arity the plan holds it to.
    static D8P_APPLICATION_BINDINGS: std::cell::RefCell<Vec<D8pApplicationBinding>> =
        const { std::cell::RefCell::new(Vec::new()) };
    /// `D8p` — the TARGET side, written at the emission seat under the same key.
    /// Kept as a separate record on purpose: joining the two on
    /// `(defining body, application occurrence)` is what makes the agreement a
    /// relation rather than one site agreeing with itself.
    static D8P_EMITTED_TARGETS: std::cell::RefCell<Vec<D8pEmittedTarget>> =
        const { std::cell::RefCell::new(Vec::new()) };
    /// `D8f` — the disposition each static-worker call edge was given, written
    /// at the boundary that acts on it.
    static D8F_DISPOSITIONS: std::cell::RefCell<
        Vec<(Option<FuncId>, StaticOriginId, CheckedApplicationDisposition)>,
    > = const { std::cell::RefCell::new(Vec::new()) };
    /// `D8g` — every static-worker call, written at the ONE emitter both
    /// populations reach, after the instruction exists.
    ///
    /// The functionized and composed populations are different programs and
    /// different ingresses; this is the seat they share, so a relation keyed
    /// here compares like with like rather than two logs that happen to be
    /// shaped alike.
    static D8G_EMISSIONS: std::cell::RefCell<Vec<D8gEmission>> =
        const { std::cell::RefCell::new(Vec::new()) };
    /// `D6b` — one specialization body's two declared call tables and the
    /// static-worker members its case environment installed.
    static D6B_BODIES: std::cell::RefCell<Vec<D6bSpecializationBody>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D6b` — one specialization body's route/table
/// situation, recorded where the body is defined.**
///
/// The subject is the ASYMMETRIC law's precondition, which no other family
/// observes: whether this body was actually retargeted, and — separately — what
/// each of its two declared call tables can answer for. Clauses 3 and 4 of the
/// law both turn on that pair, and until this record existed the only evidence
/// for it was the text of a refusal message.
#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct D6bSpecializationBody {
    pub(in crate::cranelift_backend) unit: ContinuationSpecializationId,
    /// The worker body this specialization selected.
    pub(in crate::cranelift_backend) worker_body_origin: StaticOriginId,
    /// The retarget's own outcome: `Some` iff a generated execution context
    /// replaced this body's entry in `worker_calls`.
    pub(in crate::cranelift_backend) retargeted: Option<StaticOriginId>,
    /// Which body origins `worker_calls` can answer for in this function.
    pub(in crate::cranelift_backend) worker_call_targets: BTreeSet<StaticOriginId>,
    /// Which body origins `raw_worker_calls` can answer for in this function.
    ///
    /// ⛔ Recorded as its OWN set rather than as a difference against the one
    /// above. A retargeted body is present in one and absent from the other, and
    /// that asymmetry is the whole subject -- deriving either side from the
    /// other would make it true by construction.
    pub(in crate::cranelift_backend) raw_worker_call_targets: BTreeSet<StaticOriginId>,
    /// The case environment's static-worker members, in binder-run order:
    /// `(run position, route, the body origin the member names)`.
    ///
    /// ⛔ The body origin travels WITH the route. `D6a` binds both members over
    /// one worker body by design, so a pair of routes with the origins dropped
    /// cannot say whether the mixed pair is over one body or two.
    pub(in crate::cranelift_backend) members: Vec<(usize, StaticWorkerCallRoute, StaticOriginId)>,
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d6b_specialization_body(body: D6bSpecializationBody) {
    D6B_BODIES.with(|log| log.borrow_mut().push(body));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d6b_specialization_bodies() -> Vec<D6bSpecializationBody> {
    D6B_BODIES.with(|log| log.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d6b_specialization_bodies() {
    D6B_BODIES.with(|log| log.borrow_mut().clear());
}

/// `D8g` — one static-worker call, as the shared emitter wrote it.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct D8gEmission {
    pub(in crate::cranelift_backend) function: Option<FuncId>,
    /// The occurrence of the call being lowered.
    pub(in crate::cranelift_backend) call_origin: StaticOriginId,
    /// The decoded raw callee this binding names.
    pub(in crate::cranelift_backend) target_body_origin: StaticOriginId,
    pub(in crate::cranelift_backend) declared_arity: u32,
    pub(in crate::cranelift_backend) captures: usize,
    pub(in crate::cranelift_backend) route: StaticWorkerCallRoute,
    /// The raw run the worker's own contract accounts for.
    pub(in crate::cranelift_backend) raw_operands: usize,
    /// What the instruction actually carried.
    pub(in crate::cranelift_backend) supplied_operands: usize,
    /// Whether this binding carries a composed causal authority.
    pub(in crate::cranelift_backend) composed_discharge: bool,
    /// **The emitted callee IDENTITY**: the declared `FuncRef` the route's own
    /// table answered with, which is the callee the instruction is written
    /// against.
    ///
    /// ⛔ NOT the target origin. The raw and generated-context routes
    /// intentionally SHARE a worker body origin -- that is the whole of `D6a` --
    /// so an origin recorded here is identical on both routes and a wrong-table
    /// mutation cannot move it. The `FuncRef` is the one fact that does.
    pub(in crate::cranelift_backend) emitted_callee: u32,
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8g`** — the durable producer-input mutations,
/// with application counters.
///
/// ⛔ Each moves ONE producer input at the emitter and nothing else. The
/// counters are what make "the mutation fired" a measurement rather than an
/// assumption: a control that silently never applied is a green that proves the
/// opposite of what it claims.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D8gMutation {
    Exact,
    /// Each route reads the OTHER route's target table. The call key, the
    /// binding, the operand run and the route field all stay exact; only which
    /// table answers moves.
    WrongTable,
    /// The sole producer of the generated-context capture suffix is withheld.
    /// The raw run and the call itself stay exact.
    WithholdContextSuffix,
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D9b` — the ordinary-envelope perturbations.**
///
/// ⛔ Each moves ONE fact of the planner's own role sequence. The assembler, the
/// lowered field run and the selected closure are untouched, so a refusal is
/// attributable to the moved role and not to a rewritten assembly.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D9EnvelopeMutation {
    Exact,
    /// Exchange the ordinals of the first two capture roles, leaving both roles
    /// at their own positions. ⭐ The required discriminator: a multiset of
    /// capture values cannot see this, because the same five values are still
    /// present.
    SwapCaptureOrdinals,
    /// Move the leading nonrecursive role behind the capture run, so the ruled
    /// prefix-then-captures order is violated with the same roles present.
    NonrecursiveAfterCaptures,
    /// Point one capture role at another closure occurrence, leaving its ordinal
    /// and position alone.
    ForeignCaptureClosure,
    /// Drop the last capture role, so the selected closure's run is consumed
    /// short.
    DropLastCaptureRole,
}

#[cfg(test)]
thread_local! {
    static D9_ENVELOPE_MUTATION: std::cell::Cell<D9EnvelopeMutation> =
        const { std::cell::Cell::new(D9EnvelopeMutation::Exact) };
    static D9_ENVELOPE_APPLICATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static D9_ASSEMBLIES: std::cell::RefCell<Vec<D9Assembly>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

/// One assembled ordinary run, keyed by role position.
///
/// ⭐ **Both sides are TYPED.** The roles are the planner's own sum, and the
/// operands are exact identities — not descriptions. The string forms this
/// replaced were a *collapsed shape tag*, and the collapse was measured rather
/// than argued: on the two-capture witness both distinct `Int` captures encoded
/// to the single string `"specialized:other"`, so a swap of their ordinals was
/// an observational identity and the discriminator could not have failed.
#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct D9Assembly {
    pub(in crate::cranelift_backend) unit: ContinuationSpecializationId,
    /// The role sequence the assembler consumed, in order.
    pub(in crate::cranelift_backend) roles: Vec<D9RoleKey>,
    /// What it put at each of those positions, in the same order.
    pub(in crate::cranelift_backend) operands: Vec<D9OperandIdentity>,
    /// The assembler's INPUT authorities, recorded so the expectation can be
    /// derived without consulting the assembled run.
    ///
    /// ⛔ These are inputs, never the output: `fields` is the producer
    /// constructor's whole lowered field run and `captures` is the selected
    /// closure's own ordered capture vector. The independent side maps a role
    /// onto one of these; comparing the result with `operands` is then a real
    /// relation rather than an identity.
    pub(in crate::cranelift_backend) fields: Vec<D9OperandIdentity>,
    pub(in crate::cranelift_backend) captures: Vec<D9OperandIdentity>,
}

/// One role of the planner's ordinary envelope, typed and exact.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D9RoleKey {
    NonrecursiveField {
        source_position: u32,
    },
    WorkerCapture {
        ordinal: u32,
        closure_origin: StaticOriginId,
    },
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d9_role_key(
    role: &ContinuationOrdinaryEnvelopeRole,
) -> D9RoleKey {
    match role {
        ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField { source_position } => {
            D9RoleKey::NonrecursiveField {
                source_position: *source_position,
            }
        }
        ContinuationOrdinaryEnvelopeRole::WorkerCapture {
            ordinal,
            closure_origin,
            ..
        } => D9RoleKey::WorkerCapture {
            ordinal: *ordinal,
            closure_origin: *closure_origin,
        },
    }
}

/// The PHASE of an operand — the closed sum, never inferred from its content.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D9OperandPhase {
    Specialized,
    Carried,
}

/// **The exact, comparison-only identity of one lowering operand.**
///
/// ⛔ **This is NOT a `Debug` for `LoweringOperand` and not an inverse.** It
/// recovers no compile-time template: it names the operand's phase, the
/// already-public [`LoweredVariant`] classification, and the SSA words and
/// planner-issued origins the operand holds. An SSA `Value` is an opaque index
/// — it is precisely what [`CarriedBoundaryWord`] itself holds — and a
/// [`StaticOriginId`] is a planner name. No constructor spelling, field name,
/// byte string, or literal is read, so nothing here reconstitutes a template
/// from a word. The `§2g` wall is about *becoming a `Lowered` inhabitant*, and
/// an equality token is not that.
///
/// ⭐ **Exact where it must be, and honest where it cannot be.** Two operands
/// with different SSA words compare unequal. Two that hold no word at all (a
/// `Bytes`, a `String`, a nullary `Constructor`) compare equal on content, so
/// this type does **not** claim global injectivity. The row that depends on
/// telling two operands apart asserts their identities differ **as its own
/// premise** rather than assuming it — that assertion is what makes the
/// swapped-ordinal discriminator a real red instead of a no-op.
#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct D9OperandIdentity {
    pub(in crate::cranelift_backend) phase: D9OperandPhase,
    /// `None` for a carried word. ⛔ Not a gap: `§2g` is explicit that a carried
    /// word gains **no** [`LoweredVariant`], so asking for one would be the
    /// inverse conversion the phase sum exists to forbid.
    pub(in crate::cranelift_backend) variant: Option<LoweredVariant>,
    /// Every SSA word the operand holds, in structural order.
    pub(in crate::cranelift_backend) words: Vec<cranelift_codegen::ir::Value>,
    /// Every planner-issued static origin it names, in structural order.
    pub(in crate::cranelift_backend) origins: Vec<StaticOriginId>,
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d9_operand_identity(
    operand: &LoweringOperand,
) -> D9OperandIdentity {
    let mut words = Vec::new();
    let mut origins = Vec::new();
    let (phase, variant) = match operand {
        LoweringOperand::Carried(word) => {
            words.push(word.word);
            (D9OperandPhase::Carried, None)
        }
        LoweringOperand::Specialized(lowered) => {
            d9_collect(lowered, &mut words, &mut origins);
            (D9OperandPhase::Specialized, Some(lowered.variant()))
        }
    };
    D9OperandIdentity {
        phase,
        variant,
        words,
        origins,
    }
}

/// ⛔ **Exhaustive, with NO wildcard arm.** A `_ =>` is how the encoder this
/// replaced collapsed `Int` — and every other unnamed arm — to one tag. Adding a
/// [`Lowered`] variant must be a compile error here, not a silently coarser
/// identity.
#[cfg(test)]
fn d9_collect(
    lowered: &Lowered,
    words: &mut Vec<cranelift_codegen::ir::Value>,
    origins: &mut Vec<StaticOriginId>,
) {
    match lowered {
        Lowered::Int { value, .. }
        | Lowered::Bool { value, .. }
        | Lowered::ProcessExitStatus { value }
        | Lowered::CapabilityToken { value }
        | Lowered::ResourceToken { value }
        | Lowered::BorrowedNativeValue { pointer: value } => words.push(*value),
        Lowered::BoundedNat(nat) => words.push(nat.value),
        Lowered::StructuralNat(nat) => words.push(nat.value),
        Lowered::ResponseBytes(span) => {
            words.push(span.pointer());
            words.push(span.len());
        }
        Lowered::HostResult {
            success, error, ok, ..
        } => {
            words.push(*success);
            d9_collect(error, words, origins);
            d9_collect(ok, words, origins);
        }
        Lowered::DynamicConstructor(dynamic) => {
            words.push(dynamic.discriminator);
            for alternative in &dynamic.alternatives {
                for field in &alternative.fields {
                    d9_collect(field, words, origins);
                }
            }
        }
        Lowered::BorrowedOption { present, value, .. } => {
            words.push(*present);
            words.push(*value);
        }
        Lowered::Constructor { args, .. } => {
            for arg in args {
                match arg {
                    ConstructorField::Specialized(value) => d9_collect(value, words, origins),
                    // A worker field contributes **no runtime word** — that is
                    // what "no value representation" means, and it is why this
                    // arm cannot push onto `words`.
                    //
                    // **It does contribute an ORIGIN, and that is a
                    // re-derivation rather than the original decision.** This
                    // arm previously contributed nothing at all, justified by
                    // *"this walk is an infallible observation"* — sound only
                    // while nothing constructed a worker. Now that the producer
                    // is armed, contributing nothing would make a template
                    // carrying a worker observationally **identical** to one
                    // missing that field entirely, and distinguishing operands
                    // is this walk's whole purpose. The binding's `body_origin`
                    // is the same fact the `Closure` arm below records, so the
                    // observation stays faithful without inventing a word the
                    // field does not have.
                    ConstructorField::StaticWorker { binding, .. } => {
                        origins.push(binding.body_origin)
                    }
                }
            }
        }
        Lowered::Record { fields, .. } => {
            for field in fields {
                d9_collect(&field.value, words, origins);
            }
        }
        Lowered::Closure { captures, body, .. } => {
            origins.push(*body);
            for capture in captures {
                let nested = d9_operand_identity(capture);
                words.extend(nested.words);
                origins.extend(nested.origins);
            }
        }
        Lowered::DeclarationClosure {
            reference,
            captures,
            body,
            ..
        } => {
            origins.push(*reference);
            origins.push(*body);
            for capture in captures {
                let nested = d9_operand_identity(capture);
                words.extend(nested.words);
                origins.extend(nested.origins);
            }
        }
        Lowered::ComputationalRecursorClosure { residual, .. } => {
            let nested = d9_operand_identity(residual);
            words.extend(nested.words);
            origins.extend(nested.origins);
        }
        // ⛔ Named explicitly, not swept into a wildcard. Each holds no SSA word
        // and no origin, so its identity is its variant alone — which the doc
        // comment above states as a boundary rather than hiding.
        Lowered::Bytes(_)
        | Lowered::String(_)
        | Lowered::RecursiveBackedge
        | Lowered::Trap(_) => {}
    }
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d9_perturb_envelope(
    envelope: Vec<ContinuationOrdinaryEnvelopeRole>,
) -> Vec<ContinuationOrdinaryEnvelopeRole> {
    let mutation = D9_ENVELOPE_MUTATION.with(std::cell::Cell::get);
    if mutation == D9EnvelopeMutation::Exact {
        return envelope;
    }
    let capture_positions = envelope
        .iter()
        .enumerate()
        .filter(|(_, role)| {
            matches!(role, ContinuationOrdinaryEnvelopeRole::WorkerCapture { .. })
        })
        .map(|(position, _)| position)
        .collect::<Vec<_>>();
    let mut perturbed = envelope;
    match mutation {
        D9EnvelopeMutation::Exact => {}
        // ⛔ Declines unless there are TWO captures to exchange. With fewer the
        // swap is the identity, and counting an application for it would let a
        // control read a green as a defence.
        D9EnvelopeMutation::SwapCaptureOrdinals => {
            if capture_positions.len() >= 2 {
                let (first, second) = (capture_positions[0], capture_positions[1]);
                let (a, b) = (
                    d9_role_ordinal(&perturbed[first]),
                    d9_role_ordinal(&perturbed[second]),
                );
                if let (Some(a), Some(b)) = (a, b) {
                    if a != b {
                        d9_set_role_ordinal(&mut perturbed[first], b);
                        d9_set_role_ordinal(&mut perturbed[second], a);
                        D9_ENVELOPE_APPLICATIONS.with(|cell| cell.set(cell.get() + 1));
                    }
                }
            }
        }
        D9EnvelopeMutation::NonrecursiveAfterCaptures => {
            if let Some(position) = perturbed.iter().position(|role| {
                matches!(
                    role,
                    ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField { .. }
                )
            }) {
                if !capture_positions.is_empty() {
                    let role = perturbed.remove(position);
                    perturbed.push(role);
                    D9_ENVELOPE_APPLICATIONS.with(|cell| cell.set(cell.get() + 1));
                }
            }
        }
        D9EnvelopeMutation::ForeignCaptureClosure => {
            if let Some(position) = capture_positions.first().copied() {
                if let ContinuationOrdinaryEnvelopeRole::WorkerCapture {
                    closure_origin, ..
                } = &mut perturbed[position]
                {
                    // A REAL origin naming the wrong role, not a fabricated id:
                    // an unknown id could be refused merely for being unknown.
                    if let Some(other) = d9_other_origin(*closure_origin) {
                        *closure_origin = other;
                        D9_ENVELOPE_APPLICATIONS.with(|cell| cell.set(cell.get() + 1));
                    }
                }
            }
        }
        D9EnvelopeMutation::DropLastCaptureRole => {
            if let Some(position) = capture_positions.last().copied() {
                perturbed.remove(position);
                D9_ENVELOPE_APPLICATIONS.with(|cell| cell.set(cell.get() + 1));
            }
        }
    }
    perturbed
}

#[cfg(test)]
fn d9_role_ordinal(role: &ContinuationOrdinaryEnvelopeRole) -> Option<u32> {
    match role {
        ContinuationOrdinaryEnvelopeRole::WorkerCapture { ordinal, .. } => Some(*ordinal),
        ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField { .. } => None,
    }
}

#[cfg(test)]
fn d9_set_role_ordinal(role: &mut ContinuationOrdinaryEnvelopeRole, value: u32) {
    if let ContinuationOrdinaryEnvelopeRole::WorkerCapture { ordinal, .. } = role {
        *ordinal = value;
    }
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d9_set_foreign_origin(origin: Option<StaticOriginId>) {
    D9_FOREIGN_ORIGIN.with(|cell| cell.set(origin));
}

#[cfg(test)]
thread_local! {
    static D9_FOREIGN_ORIGIN: std::cell::Cell<Option<StaticOriginId>> =
        const { std::cell::Cell::new(None) };
}

#[cfg(test)]
fn d9_other_origin(current: StaticOriginId) -> Option<StaticOriginId> {
    D9_FOREIGN_ORIGIN
        .with(std::cell::Cell::get)
        .filter(|candidate| *candidate != current)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d9_assembly(assembly: D9Assembly) {
    D9_ASSEMBLIES.with(|log| log.borrow_mut().push(assembly));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d9_assemblies() -> Vec<D9Assembly> {
    D9_ASSEMBLIES.with(|log| log.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d9_assemblies() {
    D9_ASSEMBLIES.with(|log| log.borrow_mut().clear());
}

/// Arm one `D9b` envelope perturbation for `body`, restoring however it leaves.
#[cfg(test)]
pub(in crate::cranelift_backend) fn with_d9_envelope_mutation<T>(
    mutation: D9EnvelopeMutation,
    body: impl FnOnce() -> T,
) -> (T, usize) {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            D9_ENVELOPE_MUTATION.with(|cell| cell.set(D9EnvelopeMutation::Exact));
        }
    }
    D9_ENVELOPE_MUTATION.with(|cell| cell.set(mutation));
    D9_ENVELOPE_APPLICATIONS.with(|cell| cell.set(0));
    let _restore = Restore;
    let result = body();
    (
        result,
        D9_ENVELOPE_APPLICATIONS.with(std::cell::Cell::get),
    )
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D6c` — the pre-emission SELECTION refusal set.**
///
/// ⛔ **Not `D8f`'s set, despite sharing vocabulary.** `D8f` is about which call
/// consumes a pending checked-IH marker. These are about **selecting the raw/IH
/// target and its `SelectedRecursiveArgument` member**, before any instruction
/// exists. Say which you mean whenever you write "the omission refusal" here.
///
/// Each variant moves the smallest thing its law is about and nothing else, so a
/// refusal is attributable to that perturbation rather than to a rewritten
/// resolver. Every arm bumps the application counter **only when it actually
/// changes something**, which is what makes "the mutation reached the seat" a
/// measurement instead of an assumption.
///
/// ⛔ **They are not all single inputs.** [`Self::CrossRouteTargets`] is a
/// **paired route exchange** — a law about crossing two routes cannot be violated
/// by moving one of them — and [`Self::WrongOrder`] is a **segment permutation**
/// rather than an input at all. The other six move one thing each.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D6cSelectionMutation {
    Exact,
    /// Skip the `SelectedRecursiveArgument` member — the pre-`D6a` defect, where
    /// the induction hypothesis silently stood in for the argument too.
    OmitSelectedArgument,
    /// Push the member twice, so one run names two selected arguments.
    DuplicateSelectedArgument,
    /// Name a source position other than the one the unit projects a worker for.
    WrongSourcePosition,
    /// Permute the run: the IH prefix and the argument segment are exchanged,
    /// with the same members, the same count and the same tail. ⛔ A permutation,
    /// not a moved input.
    WrongOrder,
    /// Claim a member for a recursive position this specialization projects no
    /// worker for — availability the plan does not grant.
    FabricatedAvailability,
    /// Build the argument binding over a different closure/body origin than the
    /// one the unit selected.
    WrongClosureBody,
    /// Build the argument binding with a capture run that is not the envelope's
    /// worker-capture segment.
    WrongCaptureRun,
    /// Exchange the two routes as a PAIR: the hypothesis takes the raw route
    /// while a context was resolved, and the argument takes the context route.
    /// ⛔ Both halves together -- crossing is a relation between the two members,
    /// so moving only one would be a different (and lawful) perturbation.
    CrossRouteTargets,
}

#[cfg(test)]
thread_local! {
    static D6C_SELECTION_MUTATION: std::cell::Cell<D6cSelectionMutation> =
        const { std::cell::Cell::new(D6cSelectionMutation::Exact) };
    static D6C_SELECTION_APPLICATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d6c_selection_mutation() -> D6cSelectionMutation {
    D6C_SELECTION_MUTATION.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d6c_selection_application() {
    D6C_SELECTION_APPLICATIONS.with(|cell| cell.set(cell.get() + 1));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d6c_selection_applications() -> usize {
    D6C_SELECTION_APPLICATIONS.with(std::cell::Cell::get)
}

/// Arm one `D6c` selection mutation for the duration of `body`, restoring on the
/// way out however `body` leaves. Returns `(result, applications)`.
#[cfg(test)]
pub(in crate::cranelift_backend) fn with_d6c_selection_mutation<T>(
    mutation: D6cSelectionMutation,
    body: impl FnOnce() -> T,
) -> (T, usize) {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            D6C_SELECTION_MUTATION.with(|cell| cell.set(D6cSelectionMutation::Exact));
        }
    }
    D6C_SELECTION_MUTATION.with(|cell| cell.set(mutation));
    D6C_SELECTION_APPLICATIONS.with(|cell| cell.set(0));
    let _restore = Restore;
    let result = body();
    (result, d6c_selection_applications())
}

#[cfg(test)]
thread_local! {
    static D8G_MUTATION: std::cell::Cell<D8gMutation> =
        const { std::cell::Cell::new(D8gMutation::Exact) };
    static D8G_MUTATION_APPLICATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8g_mutation() -> D8gMutation {
    D8G_MUTATION.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d8g_mutation_application() {
    D8G_MUTATION_APPLICATIONS.with(|cell| cell.set(cell.get() + 1));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8g_mutation_applications() -> usize {
    D8G_MUTATION_APPLICATIONS.with(std::cell::Cell::get)
}

/// Arm a `D8g` mutation for the duration of `body`, restoring on the way out
/// however `body` leaves. Returns `(result, applications)`.
#[cfg(test)]
pub(in crate::cranelift_backend) fn with_d8g_mutation<T>(
    mutation: D8gMutation,
    body: impl FnOnce() -> T,
) -> (T, usize) {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            D8G_MUTATION.with(|cell| cell.set(D8gMutation::Exact));
        }
    }
    D8G_MUTATION.with(|cell| cell.set(mutation));
    D8G_MUTATION_APPLICATIONS.with(|cell| cell.set(0));
    let _restore = Restore;
    let result = body();
    (result, d8g_mutation_applications())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d8g_emission(emission: D8gEmission) {
    D8G_EMISSIONS.with(|log| log.borrow_mut().push(emission));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8g_emissions() -> Vec<D8gEmission> {
    D8G_EMISSIONS.with(|log| log.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d8g_emissions() {
    D8G_EMISSIONS.with(|log| log.borrow_mut().clear());
}

/// `D8p` — what the plan bound at one checked application.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct D8pApplicationBinding {
    pub(in crate::cranelift_backend) function: Option<FuncId>,
    pub(in crate::cranelift_backend) application_origin: StaticOriginId,
    pub(in crate::cranelift_backend) call_template_id: u64,
    pub(in crate::cranelift_backend) slot_template_id: u64,
    pub(in crate::cranelift_backend) binder_index: u64,
    pub(in crate::cranelift_backend) arity: u64,
}

/// `D8p` — what was actually called there.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct D8pEmittedTarget {
    pub(in crate::cranelift_backend) function: Option<FuncId>,
    pub(in crate::cranelift_backend) application_origin: StaticOriginId,
    pub(in crate::cranelift_backend) target_body_origin: StaticOriginId,
    pub(in crate::cranelift_backend) declared_arity: u32,
    pub(in crate::cranelift_backend) captures: usize,
    /// The operand run the call instruction ACTUALLY carried, read off the
    /// emission rather than from the binding.
    ///
    /// Written only after `call_static_worker_with_inputs` has returned, so this
    /// record exists exactly when a call instruction does. Recorded before the
    /// emitter it would be a claim about a call that had not been made, and a
    /// widened run assembled inside the emitter would be invisible to it.
    pub(in crate::cranelift_backend) supplied_operands: usize,
}

/// The arm of the closed immediate-binder bridge descriptor a composed site took.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum D8mBridgeArm {
    /// A bare `ComputationalMatch` -- the unwrapped bridge, all-None.
    Computational,
    /// A `CheckedSubcontinuationFrame` wrapping a `ComputationalMatch` -- the
    /// form `D8m` added.
    CheckedComputational,
    /// A bare `Match` -- the ordinary bridge.
    Ordinary,
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d8n_frame_consumption(
    defining_function: Option<FuncId>,
    invocation_id: u64,
    frame_id: u64,
) {
    D8N_FRAME_CONSUMPTIONS
        .with(|log| log.borrow_mut().push((defining_function, invocation_id, frame_id)));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d8n_slot_reconciliation(
    defining_function: Option<FuncId>,
    slot_template_id: u64,
) {
    D8N_SLOT_RECONCILIATIONS
        .with(|log| log.borrow_mut().push((defining_function, slot_template_id)));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d8m_slot_frame_pair(
    defining_function: Option<FuncId>,
    checked_frame_id: u64,
    slot_template_id: u64,
) {
    D8M_SLOT_FRAME_PAIRS.with(|log| {
        log.borrow_mut()
            .push((defining_function, checked_frame_id, slot_template_id))
    });
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8m_slot_frame_pairs() -> Vec<(Option<FuncId>, u64, u64)> {
    D8M_SLOT_FRAME_PAIRS.with(|log| log.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d8p_application_binding(binding: D8pApplicationBinding) {
    D8P_APPLICATION_BINDINGS.with(|log| log.borrow_mut().push(binding));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d8p_emitted_target(target: D8pEmittedTarget) {
    D8P_EMITTED_TARGETS.with(|log| log.borrow_mut().push(target));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8p_application_bindings() -> Vec<D8pApplicationBinding> {
    D8P_APPLICATION_BINDINGS.with(|log| log.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8p_emitted_targets() -> Vec<D8pEmittedTarget> {
    D8P_EMITTED_TARGETS.with(|log| log.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d8f_disposition(
    function: Option<FuncId>,
    application_origin: StaticOriginId,
    disposition: CheckedApplicationDisposition,
) {
    D8F_DISPOSITIONS.with(|log| {
        log.borrow_mut()
            .push((function, application_origin, disposition))
    });
}

#[cfg(test)]
#[allow(clippy::type_complexity)]
pub(in crate::cranelift_backend) fn d8f_dispositions(
) -> Vec<(Option<FuncId>, StaticOriginId, CheckedApplicationDisposition)> {
    D8F_DISPOSITIONS.with(|log| log.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d8m_bridge_arm(
    defining_function: Option<FuncId>,
    arm: D8mBridgeArm,
) {
    D8M_BRIDGE_ARMS.with(|log| log.borrow_mut().push((defining_function, arm)));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8m_bridge_arms() -> Vec<(Option<FuncId>, D8mBridgeArm)> {
    D8M_BRIDGE_ARMS.with(|log| log.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8n_frame_consumptions() -> Vec<(Option<FuncId>, u64, u64)> {
    D8N_FRAME_CONSUMPTIONS.with(|log| log.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8n_slot_reconciliations() -> Vec<(Option<FuncId>, u64)> {
    D8N_SLOT_RECONCILIATIONS.with(|log| log.borrow().clone())
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8o`** — what each emitted body was BOUND
/// with, and what it INHERITED.
///
/// ⛔ Written at the binding from the facts the caller supplied, which are the
/// planner's own for that body kind. The inherited pair is the half that proves
/// the release: with it in place every body inherits `None`.
#[cfg(test)]
#[allow(clippy::type_complexity)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct D8oBodyAuthority {
    pub(in crate::cranelift_backend) function: Option<FuncId>,
    pub(in crate::cranelift_backend) owner: ContinuationEmissionOwner,
    pub(in crate::cranelift_backend) unit: PredeclaredFunctionId,
    pub(in crate::cranelift_backend) inherited_owner: Option<ContinuationEmissionOwner>,
    pub(in crate::cranelift_backend) inherited_unit: Option<PredeclaredFunctionId>,
}

#[cfg(test)]
thread_local! {
    static D8O_BODY_AUTHORITIES: std::cell::RefCell<Vec<D8oBodyAuthority>> =
        const { std::cell::RefCell::new(Vec::new()) };
    static D8O_INHERIT_RESIDUE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d8o_body_authority(
    function: Option<FuncId>,
    owner: ContinuationEmissionOwner,
    unit: PredeclaredFunctionId,
    inherited_owner: Option<ContinuationEmissionOwner>,
    inherited_unit: Option<PredeclaredFunctionId>,
) {
    D8O_BODY_AUTHORITIES.with(|log| {
        log.borrow_mut().push(D8oBodyAuthority {
            function,
            owner,
            unit,
            inherited_owner,
            inherited_unit,
        })
    });
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8o_body_authorities() -> Vec<D8oBodyAuthority> {
    D8O_BODY_AUTHORITIES.with(|log| log.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d8o_body_authorities() {
    D8O_BODY_AUTHORITIES.with(|log| log.borrow_mut().clear());
    D8O_COMPOSED_CLAIM_BODIES.with(|log| log.borrow_mut().clear());
    D8O_BODY_KEYS.with(|log| log.borrow_mut().clear());
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8o` — the exact body key.**
///
/// ⛔⛔ **Closed, and independently meaningful.** Each variant names the body by
/// the planner descriptor identity that pass was handed -- not by the ambient
/// owner, not by the `FuncId` alone, not by a raw origin, and not by a selected
/// composed identity. Supplied at each of the three body sites, which is the
/// only place that knows which kind it is without inferring it.
///
/// ⭐ **A generated context carries a `Specialization` OWNER and is not a
/// specialization BODY.** That distinction is the whole reason this key exists
/// separately from the owner: an owner-variant filter counts a context body as
/// a specialization body, which is exactly the mistake the previous evidence
/// made.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum D8oBodyKey {
    OrdinaryUnit(PredeclaredFunctionId),
    ContinuationSpecialization(ContinuationSpecializationId),
    GeneratedContext(ContinuationContextId),
}

/// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the fusion-local compositions this
/// compile actually REALIZED, in order.**
///
/// Recorded at the composition seat AFTER the affine consumption succeeded,
/// so a refused composition contributes nothing. The ledger already knows what
/// was planned; what no ledger can report — because the compile refuses before
/// its closeout — is which planned members a given artifact managed to reach.
/// That is what this observes and it is the only thing it observes.
#[cfg(test)]
thread_local! {
    static R3_LOCAL_COMPOSITIONS: std::cell::RefCell<
        Vec<(ContinuationSpecializationId, FusionCompositionLayer)>,
    > = const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_r3_local_composition(
    target: ContinuationSpecializationId,
    layer: FusionCompositionLayer,
) {
    R3_LOCAL_COMPOSITIONS.with(|cell| cell.borrow_mut().push((target, layer)));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn r3_local_compositions()
-> Vec<(ContinuationSpecializationId, FusionCompositionLayer)> {
    R3_LOCAL_COMPOSITIONS.with(|cell| cell.borrow().clone())
}

/// **`D3` — the fusion-owned outer realizations this compile dispatched.**
#[cfg(test)]
thread_local! {
    static R3_OUTER_DISPATCHES: std::cell::RefCell<
        Vec<(StaticContinuationFusionId, ContinuationSpecializationId)>,
    > = const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_r3_outer_dispatch(
    fusion: StaticContinuationFusionId,
    target: ContinuationSpecializationId,
) {
    R3_OUTER_DISPATCHES.with(|cell| cell.borrow_mut().push((fusion, target)));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn r3_outer_dispatches()
-> Vec<(StaticContinuationFusionId, ContinuationSpecializationId)> {
    R3_OUTER_DISPATCHES.with(|cell| cell.borrow().clone())
}

/// **`D3` — THE SHARING RELATION, one row per assembled composed-eliminator
/// case run.**
///
/// Each row holds that run's static-worker members as
/// `(binder slot, transport)`. This is the only observable that can state
/// `evt_37715knv356yp`'s coordinate — *one recognized source field, one
/// transport, two authorized binder projections* — because the transport
/// identity is opaque and privately-fielded, so a control cannot forge one and
/// cannot recover it from anywhere else.
///
/// ⛔ Rows, not a flat list. Whether TWO members of ONE run share a transport is
/// a different question from whether two transports exist in a compile, and a
/// flattened recording answers only the second.
#[cfg(test)]
thread_local! {
    static R3_RUN_WORKER_MEMBERS: std::cell::RefCell<
        Vec<Vec<(usize, Option<StaticWorkerTransportId>)>>,
    > = const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_r3_run_worker_members(
    run: &[LoweringEnvironmentBinding],
) {
    let row: Vec<_> = run
        .iter()
        .enumerate()
        .filter_map(|(slot, binding)| match binding {
            LoweringEnvironmentBinding::StaticWorker(worker) => Some((slot, worker.transport)),
            LoweringEnvironmentBinding::Value(_) => None,
        })
        .collect();
    R3_RUN_WORKER_MEMBERS.with(|cell| cell.borrow_mut().push(row));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn r3_run_worker_members()
-> Vec<Vec<(usize, Option<StaticWorkerTransportId>)>> {
    R3_RUN_WORKER_MEMBERS.with(|cell| cell.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_r3_run_worker_members() {
    R3_RUN_WORKER_MEMBERS.with(|cell| cell.borrow_mut().clear());
}

/// **`D3` — the fused invocations emitted at claims' exact consuming calls.**
#[cfg(test)]
thread_local! {
    static R3_FUSED_INVOCATIONS: std::cell::RefCell<
        Vec<(StaticContinuationFusionId, StaticOriginId)>,
    > = const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_r3_fused_invocation(
    fusion: StaticContinuationFusionId,
    consuming_call: StaticOriginId,
) {
    R3_FUSED_INVOCATIONS.with(|cell| cell.borrow_mut().push((fusion, consuming_call)));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn r3_fused_invocations()
-> Vec<(StaticContinuationFusionId, StaticOriginId)> {
    R3_FUSED_INVOCATIONS.with(|cell| cell.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_r3_fused_invocations() {
    R3_FUSED_INVOCATIONS.with(|cell| cell.borrow_mut().clear());
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_r3_outer_dispatches() {
    R3_OUTER_DISPATCHES.with(|cell| cell.borrow_mut().clear());
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_r3_local_compositions() {
    R3_LOCAL_COMPOSITIONS.with(|cell| cell.borrow_mut().clear());
}

#[cfg(test)]
thread_local! {
    static D8O_BODY_KEYS: std::cell::RefCell<Vec<(Option<FuncId>, D8oBodyKey)>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d8o_body_key(
    function: Option<FuncId>,
    key: D8oBodyKey,
) {
    D8O_BODY_KEYS.with(|log| log.borrow_mut().push((function, key)));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8o_body_keys() -> Vec<(Option<FuncId>, D8oBodyKey)> {
    D8O_BODY_KEYS.with(|log| log.borrow().clone())
}

/// **`D7`/`AC-9` positive application counter.**
///
/// ⛔ Required by the Architect's ruling, and it is not bookkeeping: the
/// definition-binding mutation's whole claim is that a *different body* was
/// bound under the exact declared `FuncId`. A mutation that quietly applied to
/// nothing would leave the program executing its exact answer, and the test
/// would then read an unchanged result as "the substitution had no effect"
/// rather than as "no substitution happened". Those are opposite conclusions
/// from identical evidence, so the count is what separates them.
#[cfg(test)]
thread_local! {
    static D7_DEFINITION_BINDING_SUBSTITUTIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d7_definition_binding_substitution() {
    D7_DEFINITION_BINDING_SUBSTITUTIONS.with(|count| count.set(count.get() + 1));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d7_definition_binding_substitutions() -> usize {
    D7_DEFINITION_BINDING_SUBSTITUTIONS.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d7_definition_binding_substitutions() {
    D7_DEFINITION_BINDING_SUBSTITUTIONS.with(|count| count.set(0));
}

/// **`D8o`** — the emitter body of every composed claim actually reached.
#[cfg(test)]
thread_local! {
    static D8O_COMPOSED_CLAIM_BODIES: std::cell::RefCell<
        Vec<(Option<FuncId>, Option<ContinuationEmissionOwner>)>,
    > = const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d8o_composed_claim_body(
    function: Option<FuncId>,
    body_owner: Option<ContinuationEmissionOwner>,
) {
    D8O_COMPOSED_CLAIM_BODIES.with(|log| log.borrow_mut().push((function, body_owner)));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8o_composed_claim_bodies(
) -> Vec<(Option<FuncId>, Option<ContinuationEmissionOwner>)> {
    D8O_COMPOSED_CLAIM_BODIES.with(|log| log.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn set_d8o_inherit_residue(armed: bool) {
    D8O_INHERIT_RESIDUE.with(|cell| cell.set(armed));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8o_inherit_residue() -> bool {
    D8O_INHERIT_RESIDUE.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d8n_observations() {
    D8N_FRAME_CONSUMPTIONS.with(|log| log.borrow_mut().clear());
    D8N_SLOT_RECONCILIATIONS.with(|log| log.borrow_mut().clear());
    D8M_SLOT_FRAME_PAIRS.with(|log| log.borrow_mut().clear());
    D8M_BRIDGE_ARMS.with(|log| log.borrow_mut().clear());
    D8P_APPLICATION_BINDINGS.with(|log| log.borrow_mut().clear());
    D8P_EMITTED_TARGETS.with(|log| log.borrow_mut().clear());
    D8F_DISPOSITIONS.with(|log| log.borrow_mut().clear());
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8j_discharged() -> Vec<ContinuationCallIdentity> {
    D8J_DISCHARGED.with(|log| log.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn record_d8j_discharged(
    identities: impl IntoIterator<Item = ContinuationCallIdentity>,
) {
    D8J_DISCHARGED.with(|log| log.borrow_mut().extend(identities));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d8j_discharged() {
    D8J_DISCHARGED.with(|log| log.borrow_mut().clear());
}

/// **`D8i` — hand an ordinary binding site a REAL composed authority whose
/// emission owner is not this unit's.**
///
/// ⛔ The authority is taken from the plan's own target population, searched
/// for an emission owner that differs from the defining one. It is not
/// fabricated, and it could not be: `ContinuationCallIdentity` has no
/// constructor outside planning, which is exactly why this switch has to find a
/// real one rather than build a wrong one.
///
/// ⭐ It perturbs the constructor's INPUT — the facet a call site supplies —
/// and leaves the guard untouched, so the refusal is attributable to the guard
/// rather than to the mutation being the guard.
#[cfg(test)]
thread_local! {
    static D8I_FOREIGN_AUTHORITY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn set_d8i_foreign_authority(armed: bool) {
    D8I_FOREIGN_AUTHORITY.with(|cell| cell.set(armed));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8i_foreign_authority() -> bool {
    D8I_FOREIGN_AUTHORITY.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d8d_bindings() -> usize {
    D8D_STATIC_WORKER_BINDINGS.with(std::cell::Cell::get)
}

fn extend_specialized(
    env: &mut Vec<LoweringEnvironmentBinding>,
    bindings: impl IntoIterator<Item = Lowered>,
) {
    env.extend(
        bindings
            .into_iter()
            .map(|lowered| LoweringEnvironmentBinding::Value(LoweringOperand::Specialized(lowered))),
    );
}

/// Append a retained callable's **phase-bearing captures** after operands
/// already in an environment.
///
/// ⭐ **The counterpart to [`extend_specialized`] for the `D7` capture edge, and
/// the distinction is the point.** `extend_specialized` takes templates and
/// *asserts* a phase by wrapping each one `Specialized`. A capture edge already
/// carries its own phase, so asserting one here would be exactly the narrowing
/// that made a mixed environment unrepresentable: a `Carried` capture would
/// either be refused or, worse, re-labelled as a template it has no bytes for.
///
/// ⛔ There is no classification to perform. An environment binding holds a
/// [`LoweringOperand`], so this is a total, phase-preserving move — **not** a
/// wildcard that silently absorbs the carried case.
fn extend_captures(
    env: &mut Vec<LoweringEnvironmentBinding>,
    captures: impl IntoIterator<Item = LoweringOperand>,
) {
    env.extend(captures.into_iter().map(LoweringEnvironmentBinding::Value));
}

/// ⭐ The inverse direction, which needs **no** boundary at all: a freshly
/// constructed specialized value entering the spine.
///
/// ⛔ There is deliberately no `From<CarriedBoundaryWord>` counterpart taking a
/// short cut around [`Lowering::transfer_into_carrier`] — the producer is the
/// one way in, and it screens admissibility first.
impl From<Lowered> for LoweringOperand {
    fn from(lowered: Lowered) -> Self {
        LoweringOperand::Specialized(lowered)
    }
}

/// ⭐ **The ONE-WAY PRODUCER** — `Lowered -> CarriedBoundaryWord`
/// (`RT-FNSPLIT-C1` `D3`; the seam is ruled in frame `§2g`, its authority for
/// `(tag, class)` in `§2h` ¶4).
///
/// ⛔ **There is deliberately no inverse in this block, and none may be added.**
/// A `Carried -> Lowered` conversion would let a consumer recover a compile-time
/// template from a runtime value, which is the wall this whole node exists to
/// remove, wearing a different name.
impl<'a> Lowering<'a> {
    /// Transfer a compile-time [`Lowered`] into the operational carrier.
    ///
    /// ⚠⚠ **The admissibility walk runs HERE and exactly once, before the first
    /// allocation.** [`Lowered::boundary_transfer_admissibility`]'s own contract
    /// says the ordering is load-bearing: a walk performed after the first child
    /// is published rejects a transfer it has already half-emitted, which is a
    /// **partial publication**, not a rejection. That is why the recursion lives
    /// in a separate private [`Self::emit_carrier_transfer`] — the entry point
    /// screens the whole graph, and the recursion never re-screens a subgraph
    /// whose parent is already allocated.
    fn transfer_into_carrier(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        origin: StaticOriginId,
        value: &Lowered,
    ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
        #[cfg(test)]
        record_d2k_owner_event(D2kOwnerEvent::BoundaryTransferEntered {
            origin,
            root_kind: lowered_value_kind(value),
            closure_path: value.first_boundary_closure_path(),
            invoking_site: D2K_BOUNDARY_TRANSFER_INVOKING_SITE
                .with(std::cell::Cell::get),
        });
        if let Err(error) = value.boundary_transfer_admissibility() {
            return Err(unsupported(
                "BoundaryTransferDiagnostic",
                format!(
                    "origin={origin:?} root_kind={}; inner={error}",
                    lowered_value_kind(value)
                ),
            ));
        }
        self.source_aggregate_preflight(value)?;
        self.emit_carrier_transfer(builder, origin, value)
    }













    /// Enter one planned source occurrence on any lowering traversal.
    ///
    /// A recursive source-machine route may revisit the same occurrence while
    /// emitting one function. That is a reborrow of its already-consumed
    /// contract, not a second population member.
    fn enter_source_occurrence_plan(
        &mut self,
        origin: StaticOriginId,
    ) -> Result<(), CraneliftBackendError> {
        #[cfg(test)]
        LRC_D2B_ENTERED.with(|cell| {
            cell.borrow_mut().insert(origin);
        });
        if self
            .static_transition_plan
            .join_plan_token_if_planned(origin)?
            .is_none()
        {
            return Ok(());
        }
        if self.function_local.consumed_join_origins.contains(&origin) {
            self.consumed_join_plan_token(origin)?;
        } else {
            self.consume_join_plan(origin)?;
        }
        Ok(())
    }






    /// Carry one generated-unit call input across the boundary.
    ///
    /// ⭐ A `Carried` input is already across and is returned untouched; a
    /// `Specialized` one crosses here.
    ///
    /// ⛔ **The origin is no longer the input's own, and that is a measured
    /// simplification rather than a regression.** It used to be, because an
    /// aggregate input's ownership record was resolved at whatever coordinate
    /// it was transferred at. It is not any more: every source aggregate now
    /// carries its producer occurrence and recovers its own schema from it, and
    /// the whole-graph preflight refuses one that does not. MEASURED: forcing
    /// every source-call input to cross at the program ROOT -- the maximally
    /// wrong coordinate -- leaves the suite at its exact baseline.
    fn carry_call_input(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        origin: StaticOriginId,
        input: LoweringOperand,
        #[cfg(test)] caller: GeneratedUnitCallInputCaller,
        #[cfg(test)] callee: GeneratedUnitCallInputCallee,
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        #[cfg(test)]
        let _invoking_site = BoundaryTransferInvokingSiteGuard::enter(
            BoundaryTransferInvokingSite::GeneratedUnitCallInput { caller, callee },
        );
        match input {
            LoweringOperand::Carried(word) => Ok(LoweringOperand::Carried(word)),
            LoweringOperand::Specialized(value) => {
                let value = self.unit_boundary_environment_record(value)?;
                Ok(LoweringOperand::Carried(
                    self.transfer_into_carrier(builder, origin, &value)?,
                ))
            }
        }
    }


    /// Resolve the body-level identity used only by the call-input diagnostic.
    ///
    /// Under the missing-child mutation the real planner lookup is redirected
    /// to a position that cannot exist. The failure becomes an explicit tag;
    /// it cannot become an early return from the compile it observes.
    #[cfg(test)]
    fn generated_unit_call_body_callee(
        &self,
        entry: StaticOriginId,
    ) -> GeneratedUnitCallInputCallee {
        let mutated = call_input_callee_child_missing();
        let unmutated = self
            .static_transition_plan
            .child_static_origin(entry, 0)
            .ok();
        let selected = if mutated {
            self.static_transition_plan
                .child_static_origin(entry, usize::MAX)
                .ok()
        } else {
            unmutated
        };
        if mutated && selected != unmutated {
            note_call_input_callee_child_missing();
            return selected.map_or(
                GeneratedUnitCallInputCallee::MissingBodyChildByMutation { entry },
                GeneratedUnitCallInputCallee::Body,
            );
        }
        selected.map_or(
            GeneratedUnitCallInputCallee::MissingBodyChild { entry },
            GeneratedUnitCallInputCallee::Body,
        )
    }

    /// Resolve the source-machine declared unit's diagnostic identity.
    ///
    /// This route begins with a scheduling entry rather than a closure. Child
    /// zero is still the comparable body-level identity when the plan has one;
    /// otherwise the unmutated entry is retained and its different level is
    /// represented in the tag.
    #[cfg(test)]
    fn generated_unit_call_entry_callee(
        &self,
        entry: StaticOriginId,
    ) -> GeneratedUnitCallInputCallee {
        let mutated = call_input_callee_child_missing();
        let unmutated = self
            .static_transition_plan
            .child_static_origin(entry, 0)
            .ok();
        let selected = if mutated {
            self.static_transition_plan
                .child_static_origin(entry, usize::MAX)
                .ok()
        } else {
            unmutated
        };
        if mutated && selected != unmutated {
            note_call_input_callee_child_missing();
            return selected.map_or(
                GeneratedUnitCallInputCallee::MissingBodyChildByMutation { entry },
                GeneratedUnitCallInputCallee::Body,
            );
        }
        selected.map_or(
            GeneratedUnitCallInputCallee::Entry(entry),
            GeneratedUnitCallInputCallee::Body,
        )
    }





    /// **`D2f` — the ordered continuation-input operands one redirected
    /// invocation must append, resolved in the caller being defined.**
    ///
    /// **Each input is resolved through the LANDED entry-frame membership
    /// gate**, not through a rule respelled here.
    /// `verify_predeclared_entry_frame_membership` is `D3b`'s authority on
    /// whether a predeclared frame really declares a member for a coordinate at
    /// a slot; calling it means the fused call and every other consumer of an
    /// entry-frame coordinate answer that question the same way, and a later
    /// change to the rule reaches this seat too.
    ///
    /// **Both non-serviceable shapes refuse rather than defaulting.** A
    /// `ProducerLocal` coordinate names a mid-body value, and this seat holds an
    /// ABI operand run and no lexical environment — the same refusal
    /// `resolve_context_capture_claim` makes, for the same reason. An `EntryAbi`
    /// coordinate naming a frame other than the one being defined names an
    /// operand run this caller does not hold. Substituting a plausible operand
    /// for either would pass the callee a word from the wrong frame.
    fn fused_redirect_inputs(
        &mut self,
        seat: StaticOriginId,
    ) -> Result<Option<Vec<LoweringOperand>>, CraneliftBackendError> {
        let Some(defining) = self.defining_unit else {
            return Ok(None);
        };
        let Some(ledger) = self.fusion_claims.as_ref() else {
            return Ok(None);
        };
        let mut matched = ledger.planned().iter().copied().filter(|fusion| {
            ledger
                .claim(*fusion)
                .is_some_and(|claim| claim.seat() == seat && claim.consumer_owner() == defining)
        });
        let Some(fusion) = matched.next() else {
            return Ok(None);
        };
        if matched.next().is_some() {
            return Err(unsupported(
                "StaticContinuationFusion",
                "two installed fused regions claim one redirected invocation seat in one unit, so \
                 which region's continuation inputs this call passes is undetermined",
            ));
        }
        let authorities = ledger
            .claim(fusion)
            .expect("the claim was present at the filter above")
            .inputs()
            .to_vec();
        let mut resolved = Vec::with_capacity(authorities.len());
        for authority in authorities {
            let ContinuationSourceCoordinate::EntryAbi {
                source_owner,
                source_abi_position,
                ..
            } = authority.coordinate
            else {
                return Err(unsupported(
                    "StaticContinuationFusion",
                    "a fused region's continuation input names a producer-local coordinate; the \
                     redirected call seat holds an entry ABI operand run and no lexical \
                     environment, so there is no operand to pass and this refuses rather than \
                     indexing one run with the other's index",
                ));
            };
            if source_owner != defining {
                return Err(unsupported(
                    "StaticContinuationFusion",
                    format!(
                        "a fused region's continuation input names entry frame {source_owner:?}, \
                         which is not the frame making the redirected call ({defining:?}); its \
                         declared position indexes an operand run this caller does not hold"
                    ),
                ));
            }
            verify_predeclared_entry_frame_membership(
                &self.static_transition_plan,
                source_owner,
                authority.coordinate,
                source_abi_position,
            )?;
            let operand = self
                .function_local
                .defining_abi_operands
                .get(source_abi_position as usize)
                .ok_or_else(|| {
                    unsupported(
                        "StaticContinuationFusion",
                        format!(
                            "a fused region's continuation input names entry ABI position {} \
                             outside the calling function's {} operands",
                            source_abi_position,
                            self.function_local.defining_abi_operands.len(),
                        ),
                    )
                })?
                .clone();
            resolved.push(operand);
        }
        Ok(Some(resolved))
    }




    /// **`RT-CONTSPEC-ACTIVATE` `4b` — the emission-seam equality gate for one
    /// generated function.**
    ///
    /// For every causal token this function emitted a call for, prove
    ///
    /// ```text
    /// bundle.continuation(identity.target())  ==  callee decoded from the CLIF
    /// ```
    ///
    /// **The two sides come from different producers.** The left is the
    /// planner's own four-field projection (`continuation_call_binding_for`)
    /// carried through the forward-declaration table; the right is the finished
    /// instruction stream. ⛔ Neither is `continuation_calls` and neither is the
    /// ledger's `resolved` map -- those are what the emitter *was handed*, and a
    /// gate built on them would agree with a redirected emission.
    ///
    /// ⚠ **The honest residual:** `bundle` is the naming authority that decides
    /// which `FuncId` a specialization *is*, so a gate cannot get behind it --
    /// if the forward declaration itself named the wrong function, both sides
    /// move together. What this proves is that the routing from planned identity
    /// to emitted callee is exact, not that the declaration table is right.
    /// What the callee's **body** then computes is deferred to
    /// `RT-CONTSPEC-WITNESS` `D7`/`AC-9` and is not claimed here.
    ///
    /// The second half is closure: every direct call in the function whose
    /// callee is *any* planned continuation specialization must be one of the
    /// recorded ones. ⛔ Without it the gate would be complete only over the set
    /// it built itself -- an unrecorded emission would be invisible, and the
    /// records would look exhaustive because nothing disagreed with them.
    fn verify_emitted_continuation_calls(
        &self,
        func: &Function,
        bundle: &units::UnitBundle,
    ) -> Result<(), CraneliftBackendError> {
        let mut expected_by_callee: BTreeMap<FuncId, usize> = BTreeMap::new();
        for (identity, inst) in &self.function_local.continuation_emissions {
            let planned = bundle.continuation(identity.target()).ok_or_else(|| {
                backend_module(
                    "an emitted causal token names a specialization that was never \
                     forward-declared"
                        .to_string(),
                )
            })?;
            let emitted = Self::decode_direct_callee(func, *inst)?;
            if emitted != planned {
                return Err(backend_module(format!(
                    "the emitted direct-call target {emitted:?} disagrees with the planner-issued \
                     continuation target {planned:?} for a causal token; the call that was built \
                     is not the call that was planned"
                )));
            }
            *expected_by_callee.entry(planned).or_default() += 1;
        }

        // Closure: no continuation call may be emitted that was not recorded.
        // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — over `O_t`, the ORDINARY
        // targets. `I_t` and `R_t` have no declared `Function` by construction:
        // an `I` target's selected body is lowered at its call edge and an `R`
        // target's is emitted once as the fusion-owned body. Demanding a
        // `FuncId` for either asks the bundle for a symbol the declaration pass
        // deliberately did not mint.
        //
        // ⛔ This is a narrowing of the CALLEE population the emission scan
        // recognises, not a weakening of the closure. The scan still refuses any
        // recorded emission naming a specialization callee, and a fusion-local
        // target emits no call at all -- so an emission for one would have to
        // name a callee this set no longer contains, and the reverse direction
        // above catches it.
        let mut specialization_callees = BTreeSet::new();
        for unit in self.static_transition_plan.ordinary_continuation_targets()? {
            let id = bundle.continuation(unit).ok_or_else(|| {
                backend_module(
                    "a planned ordinary continuation specialization was never forward-declared"
                        .to_string(),
                )
            })?;
            specialization_callees.insert(id);
        }
        // ⛔ Not a fast path around the check: with no planned specialization
        // there is no callee the scan could recognise, and the loop above has
        // already rejected any recorded emission naming one -- so `expected` is
        // necessarily empty here too. A program with no continuations skips an
        // instruction walk it could only ever conclude nothing from.
        if specialization_callees.is_empty() {
            return Ok(());
        }
        let mut observed_by_callee: BTreeMap<FuncId, usize> = BTreeMap::new();
        for block in func.layout.blocks() {
            for inst in func.layout.block_insts(block) {
                let cranelift_codegen::ir::InstructionData::Call { func_ref, .. } =
                    func.dfg.insts[inst]
                else {
                    continue;
                };
                let cranelift_codegen::ir::ExternalName::User(name_ref) =
                    func.dfg.ext_funcs[func_ref].name
                else {
                    continue;
                };
                let user = &func.params.user_named_funcs()[name_ref];
                if user.namespace != 0 {
                    continue;
                }
                let callee = FuncId::from_u32(user.index);
                if specialization_callees.contains(&callee) {
                    *observed_by_callee.entry(callee).or_default() += 1;
                }
            }
        }
        if observed_by_callee != expected_by_callee {
            return Err(backend_module(format!(
                "the continuation calls this function actually emitted {observed_by_callee:?} are \
                 not the ones recorded against planned causal tokens {expected_by_callee:?}"
            )));
        }
        Ok(())
    }

    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D8j` — verifications 3, 4 and 5, against
    /// the FINISHED CLIF, and the only route into the composed relation.**
    ///
    /// A claim recorded at the source-machine seat proved what could be proved
    /// then: the authority came from the exact paired planner target, and the
    /// claiming function is that authority's own emission owner. Everything
    /// remaining is a fact about instructions, and instructions are not
    /// readable until the function is built.
    ///
    /// 3. **The finished stream contains the recorded call.** A record naming
    ///    an instruction that is not in the layout is a record about a call
    ///    this function did not make.
    /// 4. **The decoded callee and the operand contract agree with the exact
    ///    `D8b`/`D8d` target.** ⭐ The callee is decoded from the instruction
    ///    and compared against the `FuncId` the BUNDLE gives the emittable unit
    ///    whose origin is the target's worker body — two different producers.
    ///    ⚠ The operand half is **not** decoded: this call passes its run
    ///    through a frame slot rather than as call arguments, so what is
    ///    compared is the run the emitter reported supplying against the run
    ///    the planner target declares. That is still two producers, but it is
    ///    not a CLIF fact and this comment is where that limit is written down.
    /// 5. **The result returns into the unchanged source-machine
    ///    continuation.** The value handed on must be a result of the recorded
    ///    instruction — a CLIF fact — and the live source-continuation depth
    ///    must be what it was before the emitter ran.
    ///
    /// ⛔ Records are promoted one at a time and a duplicate identity refuses:
    /// one causal obligation cannot be answered twice.
    fn verify_recorded_composed_discharges(
        &mut self,
        func: &Function,
        bundle: &units::UnitBundle,
    ) -> Result<(), CraneliftBackendError> {
        let pending = std::mem::take(&mut self.function_local.pending_composed_discharges);
        if pending.is_empty() {
            return Ok(());
        }
        let mut emitted_insts = BTreeSet::new();
        for block in func.layout.blocks() {
            for inst in func.layout.block_insts(block) {
                emitted_insts.insert(inst);
            }
        }
        for record in pending {
            // ⛔ `D8j` — the REDIRECT switch, applied to the verifier's input
            // and only where a genuinely different call exists in the finished
            // function. It moves the record onto another real instruction,
            // which is what an attribution mistake would look like; a
            // fabricated `Inst` would be caught by clause 3 instead and would
            // prove the wrong guard.
            #[cfg(test)]
            let record = if d8j_mutation() == D8jMutation::RedirectRecordedInstruction {
                let mut other = None;
                'redirect: for block in func.layout.blocks() {
                    for inst in func.layout.block_insts(block) {
                        if inst == record.inst {
                            continue;
                        }
                        if matches!(
                            func.dfg.insts[inst],
                            cranelift_codegen::ir::InstructionData::Call { .. }
                        ) {
                            other = Some(inst);
                            break 'redirect;
                        }
                    }
                }
                match other {
                    Some(inst) => PendingComposedDischarge { inst, ..record },
                    None => record,
                }
            } else {
                record
            };
            // 3 — the recorded call is in the finished stream.
            if !emitted_insts.contains(&record.inst) {
                return Err(backend_module(
                    "a claimed composed discharge names an instruction the finished function \
                     does not contain, so it answers with a call this function never made"
                        .to_string(),
                ));
            }
            // 4a — the decoded callee is the planner's own raw worker.
            let unit = self
                .static_transition_plan
                .emittable_units()?
                .into_iter()
                .find(|unit| unit.body_occurrence() == record.worker_body_origin)
                .ok_or_else(|| {
                    backend_module(
                        "a claimed composed discharge names a worker body with no emittable unit"
                            .to_string(),
                    )
                })?;
            let planned = bundle.function(unit.function()).ok_or_else(|| {
                backend_module(
                    "a claimed composed discharge names a worker unit that was never \
                     forward-declared"
                        .to_string(),
                )
            })?;
            let decoded = Self::decode_direct_callee(func, record.inst)?;
            if decoded != planned {
                return Err(backend_module(format!(
                    "a composed discharge's recorded instruction calls {decoded:?}, but the \
                     D8b/D8d target's raw worker is {planned:?}; the call that was made is not \
                     the call the authority stands for"
                )));
            }
            // 4b — the operand contract, reported against declared.
            if record.supplied_operands != record.declared_operands {
                return Err(backend_module(format!(
                    "a composed discharge supplied {} operands but its D8b/D8d target declares \
                     {}; the raw worker's contract and the call that answers for it disagree",
                    record.supplied_operands, record.declared_operands
                )));
            }
            // 5 — the result returned into the unchanged continuation.
            #[cfg(test)]
            let record = if d8j_mutation() == D8jMutation::RecordResultDefinedBeforeTheCall {
                let mut earliest = None;
                'earliest: for block in func.layout.blocks() {
                    for inst in func.layout.block_insts(block) {
                        if inst == record.inst {
                            break 'earliest;
                        }
                        if let [value, ..] = func.dfg.inst_results(inst) {
                            earliest = Some(*value);
                            break 'earliest;
                        }
                    }
                }
                match earliest {
                    Some(value) => PendingComposedDischarge {
                        result: Some(value),
                        ..record
                    },
                    None => record,
                }
            } else {
                record
            };
            let result = record.result.ok_or_else(|| {
                backend_module(
                    "a composed discharge's call result was not a carried word, so nothing \
                     returned into source-machine control for it to be"
                        .to_string(),
                )
            })?;
            // ⚠ **Downstream, not the call's own SSA result, and the
            // difference is written here rather than papered over.** A declared
            // unit returns through its callee frame: the emitter writes the
            // operand run into a stack slot, calls, and LOADS the answer back.
            // So there is no SSA result on the call instruction to compare
            // against, and a check demanding one would assert a CLIF shape this
            // ABI does not have.
            //
            // ⭐ What IS a CLIF fact, and what is checked: the value handed to
            // source-machine control is defined at or after the recorded call,
            // in the function's block layout. ⚠ NOT in the call's own block:
            // the emitter branches on the callee's status word and loads the
            // answer in a successor it creates for that purpose, so demanding
            // the same block would refuse every lawful call. Nothing before the
            // call can have produced the value, which is the property.
            let mut reached = false;
            let mut downstream = false;
            'blocks: for block in func.layout.blocks() {
                for inst in func.layout.block_insts(block) {
                    if inst == record.inst {
                        reached = true;
                    }
                    if reached && func.dfg.inst_results(inst).contains(&result) {
                        downstream = true;
                        break 'blocks;
                    }
                }
            }
            if !downstream {
                return Err(backend_module(
                    "a composed discharge's recorded result is not defined at or after its \
                     recorded call in the finished block layout, so the value handed to \
                     source-machine control cannot have come from the call that answered the \
                     obligation"
                        .to_string(),
                ));
            }
            let (before, after) = record.source_control;
            if before != after {
                return Err(backend_module(format!(
                    "a composed discharge ran with {before} live source continuations and \
                     returned into {after}; the result did not return into the continuation \
                     that was in force"
                )));
            }
            let settled_identity = record.identity.clone();
            if self
                .function_local
                .composed_discharges
                .insert(record.identity, record.inst)
                .is_some()
            {
                return Err(backend_module(
                    "one causal identity was discharged twice in a single function".to_string(),
                ));
            }
            // `RT-CONTINUATION-EDGE-DISPOSITION` `D1` — `ComposedCall`, settled
            // at the ONE seat where a composed claim has passed every clause and
            // is admitted to the verified population, so the disposition is
            // downstream of finished-CLIF verification by construction and the
            // composed feed itself is untouched.
            //
            // ⇒ **AFTER the existing double-discharge refusal, and the order is
            // load-bearing.** Settling first made this layer refuse a second
            // arrival before the law did, replacing `d8f`'s expected
            // "discharged twice in a single function" with a candidate-ledger
            // message. A layer in front of the law must not preempt the law's
            // own refusals: it derives from them, it does not speak for them.
            // `D3` — the promotion seat, recorded before the ledger call so a
            // REFUSED promotion still leaves its seat in the trace. That is the
            // terminal event both mutation 2's and mutation 3's chains end on,
            // and the seat is what tells the two chains apart.
            #[cfg(test)]
            units::d3_record(units::D3Event::Settle {
                identity: settled_identity.clone(),
                disposition: units::CandidateDisposition::ComposedCall,
                seat: units::D3Seat::ComposedPromotion,
            });
            if let Some(ledger) = self.continuation_candidates.as_mut() {
                ledger.settle(&settled_identity, units::CandidateDisposition::ComposedCall)?;
            }
        }
        Ok(())
    }







    /// The carrier helpers, as refs callable inside **this** generated function.
    fn carrier_refs(&self) -> Result<BoundaryCarrierRefs, CraneliftBackendError> {
        self.function_local.boundary_carrier.ok_or_else(|| {
            unsupported(
                "BoundaryCarrier",
                "this generated function has no boundary-carrier helper refs",
            )
        })
    }

    /// The **boundary** arena the carrier helpers take as their first argument.
    ///
    /// ⛔⛔ **A CLAIM THAT STOOD HERE WAS FALSE, and it is retracted rather than
    /// deleted.** The retracted text asserted that the boundary arena and the
    /// native-`Int` arena *"are the same SSA value, and that is a fact about the
    /// ABI rather than a shortcut."* They are not — `CompiledModule::run` passed
    /// a **`NativeIntArenaV1`** as parameter 0, and in process mode the field was
    /// re-read from `invocation[24]`, which is the native arena again. ⇒ Every
    /// boundary-carrier helper reached through here was handed a native arena.
    /// ⚠ **It never fired only because the carrier was inert** — which is
    /// exactly why it must not be discovered by `S6` making it live.
    ///
    /// ⭐ **Repaired under the Architect's ruling (relayed `evt_e300y2kjeb6k`):**
    /// one runtime-owned [`crate::activation_services::GeneratedActivationServicesV1`]
    /// with **distinct typed fields**, a uniform `(frame_ptr, services_ptr) -> i64`
    /// signature for the root and every unit, `FunctionLocalRefs` split to match,
    /// and this accessor returning **only** `boundary_arena`. ⛔ Not a second
    /// answer to one question — two different questions that were wrongly merged
    /// into one.
    ///
    /// ⚠ A reader who met the false claim and believed it needs to see that it
    /// was withdrawn, and the next person to ask *"why two arena fields?"* needs
    /// the reason here.
    fn carrier_arena(&self) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
        self.function_local.boundary_arena.ok_or_else(|| {
            unsupported(
                "BoundaryCarrier",
                "this generated function has no boundary arena: the activation \
                 -services record that sources it is `S6`/`D6` reland work, and \
                 substituting the native-`Int` arena is the defect that ruling \
                 exists to remove",
            )
        })
    }

    /// ⭐ **THE ONE SIGNEDNESS VIEW over `pack_identity`'s `u64`.**
    ///
    /// The planner's `pack_identity` is *"the ONE injective encoding"* and it
    /// yields a `u64`; Cranelift's `iconst` takes an `i64`. ⛔ This bit-preserving
    /// reinterpretation is a **view over that one encoding, never a sibling
    /// beside it** — a second packing spelled at a call site is precisely how
    /// `D2`'s single authority would be undone at the bridge rather than at the
    /// source.
    ///
    /// ⚠ It is deliberately spelled once, so there is exactly one `as i64` to
    /// review and exactly one to get wrong.
    fn carrier_identity_immediate(
        builder: &mut FunctionBuilder<'_>,
        identity: u64,
    ) -> cranelift_codegen::ir::Value {
        builder.ins().iconst(types::I64, identity as i64)
    }

    /// A one-word output slot plus its address, for a helper's `out` parameter.
    fn carrier_out_slot(
        builder: &mut FunctionBuilder<'_>,
        pointer_type: types::Type,
    ) -> (
        cranelift_codegen::ir::StackSlot,
        cranelift_codegen::ir::Value,
    ) {
        let slot =
            builder.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 8, 3));
        let address = builder.ins().stack_addr(pointer_type, slot, 0);
        (slot, address)
    }

    /// A child ordinal as an ABI immediate.
    ///
    /// `RT-EMITTER-AGGREGATES-SPLIT` `D1` -- RETAIN, hub-stays, the same
    /// disposition as `carrier_out_slot` above: called from both the
    /// `aggregates.rs` mover cluster and this hub's own retained
    /// `emit_carrier_field`, discovered only by the compiler at `D1` (not
    /// predicted at `D0`, since `D0`'s census was scoped to the two
    /// cluster line ranges).
    fn carrier_position_immediate(
        builder: &mut FunctionBuilder<'_>,
        position: usize,
    ) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
        let position = i64::try_from(position).map_err(|_| {
            unsupported(
                "BoundaryCarrier",
                "a transferred aggregate's child ordinal is outside the ABI's range",
            )
        })?;
        Ok(builder.ins().iconst(types::I64, position))
    }










    /// `make_immediate(tag, payload, out) -> status`. ⚠ No arena: an immediate
    /// names no referent.
    fn emit_carrier_immediate(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        tag: BoundaryTag,
        payload: cranelift_codegen::ir::Value,
    ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
        let refs = self.carrier_refs()?;
        let arena = self.carrier_arena()?;
        let pointer_type = builder.func.dfg.value_type(arena);
        let (slot, out) = Self::carrier_out_slot(builder, pointer_type);
        let tag = builder.ins().iconst(types::I64, i64::from(tag as u8));
        let call = builder
            .ins()
            .call(refs.make_immediate, &[tag, payload, out]);
        Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
        Ok(CarriedBoundaryWord {
            word: builder.ins().stack_load(types::I64, slot, 0),
        })
    }






    /// The `NativeIntV1` marker for a spillable immediate whose magnitude **is**
    /// its payload word.
    ///
    /// ⭐ `ProcessExitStatus`, `BoundedNat` and `StructuralNat` are one native
    /// scalar each — there is no second word and no arena slot — so `Small` is
    /// not a default for them, it is the only true answer.
    /// ⛔ `Lowered::Int` does **not** come here: its marker is carried alongside
    /// the payload by [`Self::native_int_tag`], and substituting a constant would
    /// be the producer out-voting the native-`Int` representation.
    fn carrier_small_marker(builder: &mut FunctionBuilder<'_>) -> cranelift_codegen::ir::Value {
        builder
            .ins()
            .iconst(types::I64, crate::NATIVE_INT_SMALL_TAG_V1 as i64)
    }






    // ── the CONSUMER half of the carrier ABI (`D3` / `D4`) ──────────────
    //
    // ⭐ **Every question these answer is answered AT RUNTIME, by a call.**
    // That is the whole content of the node: a carried value has no
    // compile-time template, so *"which constructor"*, *"how many children"*
    // and *"which child"* cannot be read off a struct field — there is no
    // struct field to read. ⛔ Adding one is how the wall grows back
    // (see [`CarriedBoundaryWord`]).
    //
    // ⚠ Each of these emits a `require_i64(status, BOUNDARY_OK)`, which
    // **splits the current block**. A caller assembling its own control flow
    // must therefore take `builder`'s *current* block after the call, never
    // the block it switched to before it.

    /// `tag(arena, word, out) -> status` — the runtime constructor identity.
    ///
    /// ⭐ The returned word is comparable **only** against a
    /// `ConstructorIdentity::tag_abi_word()` from the same artifact's plane —
    /// it is the very word the producer wrote with `store_tag_id`, and that
    /// shared authority is `D2`. ⛔ It is not an ordinal, not a `LoweredVariant`
    /// discriminant, and not portable across artifacts.
    fn emit_carrier_tag(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        target: CarriedBoundaryWord,
    ) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
        let refs = self.carrier_refs()?;
        let arena = self.carrier_arena()?;
        let pointer_type = builder.func.dfg.value_type(arena);
        let (slot, out) = Self::carrier_out_slot(builder, pointer_type);
        let call = builder.ins().call(refs.tag, &[arena, target.word, out]);
        Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
        Ok(builder.ins().stack_load(types::I64, slot, 0))
    }

    fn emit_carrier_class(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        target: CarriedBoundaryWord,
    ) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
        let refs = self.carrier_refs()?;
        let arena = self.carrier_arena()?;
        let pointer_type = builder.func.dfg.value_type(arena);
        let (slot, out) = Self::carrier_out_slot(builder, pointer_type);
        let call = builder.ins().call(refs.class, &[arena, target.word, out]);
        Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
        Ok(builder.ins().stack_load(types::I64, slot, 0))
    }

    fn emit_carrier_host_success(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        target: CarriedBoundaryWord,
    ) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
        let refs = self.carrier_refs()?;
        let arena = self.carrier_arena()?;
        let pointer_type = builder.func.dfg.value_type(arena);
        let (slot, out) = Self::carrier_out_slot(builder, pointer_type);
        let call = builder
            .ins()
            .call(refs.host_success, &[arena, target.word, out]);
        Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
        Ok(builder.ins().stack_load(types::I64, slot, 0))
    }

    fn emit_carrier_host_payload(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        target: CarriedBoundaryWord,
    ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
        let refs = self.carrier_refs()?;
        let arena = self.carrier_arena()?;
        let pointer_type = builder.func.dfg.value_type(arena);
        let (slot, out) = Self::carrier_out_slot(builder, pointer_type);
        let call = builder
            .ins()
            .call(refs.host_payload, &[arena, target.word, out]);
        Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
        Ok(CarriedBoundaryWord {
            word: builder.ins().stack_load(types::I64, slot, 0),
        })
    }

    pub(super) fn emit_carrier_scalar(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        target: CarriedBoundaryWord,
    ) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
        let refs = self.carrier_refs()?;
        let arena = self.carrier_arena()?;
        let pointer_type = builder.func.dfg.value_type(arena);
        let (slot, out) = Self::carrier_out_slot(builder, pointer_type);
        let call = builder.ins().call(refs.scalar, &[arena, target.word, out]);
        Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
        Ok(builder.ins().stack_load(types::I64, slot, 0))
    }

    /// Project the object entry's public scalar result.
    ///
    /// Immediate carrier words expose their payload through the ordinary
    /// scalar helper. A persistent exact `Int` instead owns a magnitude in the
    /// boundary region; export that magnitude through the native-`Int`
    /// interner/exporter pair so the object launcher observes the same decimal
    /// value as the direct lowering path.
    pub(super) fn emit_public_carrier_scalar(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        target: CarriedBoundaryWord,
    ) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
        let refs = self.carrier_refs()?;
        let boundary_arena = self.carrier_arena()?;
        let native_arena = self.function_local.native_int_arena.ok_or_else(|| {
            unsupported("NativeResult", "carried Int result has no invocation arena")
        })?;
        let export_parts = self
            .function_local
            .native_int_export_parts
            .ok_or_else(|| unsupported("NativeResult", "carried Int has no export function"))?;
        let pointer_type = builder.func.dfg.value_type(boundary_arena);

        let tag = builder.ins().band_imm(
            target.word,
            crate::boundary_value::BOUNDARY_TAG_MASK as i64,
        );
        let persistent = builder.ins().icmp_imm(
            cranelift_codegen::ir::condcodes::IntCC::Equal,
            tag,
            crate::boundary_value::BoundaryTag::PersistentGround as i64,
        );
        let exact_int = builder.create_block();
        let immediate = builder.create_block();
        let done = builder.create_block();
        builder.append_block_param(done, types::I64);
        builder
            .ins()
            .brif(persistent, exact_int, &[], immediate, &[]);

        builder.switch_to_block(exact_int);
        let view_slot = builder.create_sized_stack_slot(
            cranelift_codegen::ir::StackSlotData::new(
                cranelift_codegen::ir::StackSlotKind::ExplicitSlot,
                24,
                3,
            ),
        );
        let view = builder.ins().stack_addr(pointer_type, view_slot, 0);
        let call = builder
            .ins()
            .call(refs.int_view, &[boundary_arena, target.word, view]);
        Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
        let sign = builder.ins().stack_load(types::I64, view_slot, 0);
        let len = builder.ins().stack_load(types::I64, view_slot, 8);
        let limbs = builder.ins().stack_load(pointer_type, view_slot, 16);
        let call = builder
            .ins()
            .call(export_parts, &[native_arena, sign, limbs, len]);
        Self::require_i64(builder, builder.inst_results(call)[0], 0);
        let zero = builder.ins().iconst(types::I64, 0);
        builder.ins().jump(done, &[zero.into()]);

        builder.switch_to_block(immediate);
        let scalar = self.emit_carrier_scalar(builder, target)?;
        builder.ins().jump(done, &[scalar.into()]);

        builder.switch_to_block(done);
        Ok(builder.block_params(done)[0])
    }

    /// `field_count(arena, word, out) -> status` — the child count a case's
    /// binder arity is checked against **at runtime**.
    ///
    /// ⚠ **Why this is a runtime check and not a compile-time one.** For a
    /// specialized scrutinee the arity check compares `case.binders` against
    /// `args.len()`, both known while compiling. A carried word knows neither,
    /// so the same guard has to be emitted — and it must still *be* a guard:
    /// binding *n* binders over a value with fewer children would read past the
    /// node.
    fn emit_carrier_field_count(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        target: CarriedBoundaryWord,
    ) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
        let refs = self.carrier_refs()?;
        let arena = self.carrier_arena()?;
        let pointer_type = builder.func.dfg.value_type(arena);
        let (slot, out) = Self::carrier_out_slot(builder, pointer_type);
        let call = builder
            .ins()
            .call(refs.field_count, &[arena, target.word, out]);
        Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
        Ok(builder.ins().stack_load(types::I64, slot, 0))
    }

    /// `field(arena, word, index, out) -> status` — positional child projection.
    ///
    /// ⭐⭐ **The result is a [`CarriedBoundaryWord`], and that return type is
    /// the `§2g` property, not a convenience.** *"Projected children remain
    /// `Carried`."* A signature returning [`Lowered`] here would be the
    /// forbidden inverse conversion wearing the name of a projection — so the
    /// prohibition is carried by the **type**, where it cannot be forgotten at a
    /// call site.
    fn emit_carrier_field(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        target: CarriedBoundaryWord,
        position: usize,
    ) -> Result<CarriedBoundaryWord, CraneliftBackendError> {
        let refs = self.carrier_refs()?;
        let arena = self.carrier_arena()?;
        let pointer_type = builder.func.dfg.value_type(arena);
        let (slot, out) = Self::carrier_out_slot(builder, pointer_type);
        let index = Self::carrier_position_immediate(builder, position)?;
        let call = builder
            .ins()
            .call(refs.field, &[arena, target.word, index, out]);
        Self::require_i64(builder, builder.inst_results(call)[0], BOUNDARY_OK);
        Ok(CarriedBoundaryWord {
            word: builder.ins().stack_load(types::I64, slot, 0),
        })
    }


}

impl Lowered {
    /// Test-only diagnostic path to the first ordinary closure in the same
    /// whole-value graph screened by [`Self::boundary_transfer_admissibility`].
    ///
    /// This is not an admission predicate. It records the root-to-child path
    /// needed to distinguish a closure result from a closure nested in an
    /// enclosing aggregate, while the production admissibility walk remains
    /// the sole decision.
    #[cfg(test)]
    fn first_boundary_closure_path(&self) -> Option<String> {
        fn descend(value: &Lowered, path: String) -> Option<String> {
            match value {
                Lowered::Closure { .. } | Lowered::DeclarationClosure { .. } => Some(path),
                Lowered::Constructor { args, .. } => {
                    for (position, field) in args.iter().enumerate() {
                        let ConstructorField::Specialized(child) = field else {
                            // `RT-CROSSING-CALL-SITE-ATTRIBUTION` `D3`: the
                            // production walk refuses this field through
                            // `specialized_field_refs_at`; this diagnostic has
                            // no child value to inspect. Its eventual `None`
                            // therefore means only "no ordinary closure was
                            // located on the inspectable specialized run", not
                            // "the whole constructor graph contains no
                            // closure".
                            continue;
                        };
                        if let Some(found) = descend(
                            child,
                            format!("{path}.arg[{position}].{}", lowered_value_kind(child)),
                        ) {
                            return Some(found);
                        }
                    }
                    None
                }
                Lowered::Record { fields, .. } => {
                    for (position, field) in fields.iter().enumerate() {
                        if let Some(found) = descend(
                            &field.value,
                            format!(
                                "{path}.field[{position}].{}",
                                lowered_value_kind(&field.value)
                            ),
                        ) {
                            return Some(found);
                        }
                    }
                    None
                }
                Lowered::HostResult { error, ok, .. } => descend(
                    error,
                    format!("{path}.error.{}", lowered_value_kind(error)),
                )
                .or_else(|| descend(ok, format!("{path}.ok.{}", lowered_value_kind(ok)))),
                Lowered::DynamicConstructor(dynamic) => {
                    for (alternative, branch) in dynamic.alternatives.iter().enumerate() {
                        for (position, field) in branch.fields.iter().enumerate() {
                            if let Some(found) = descend(
                                field,
                                format!(
                                    "{path}.alternative[{alternative}].field[{position}].{}",
                                    lowered_value_kind(field)
                                ),
                            ) {
                                return Some(found);
                            }
                        }
                    }
                    None
                }
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
                | Lowered::ComputationalRecursorClosure { .. }
                | Lowered::RecursiveBackedge
                | Lowered::Trap(_) => None,
            }
        }

        descend(self, lowered_value_kind(self).to_string())
    }

    }

#[derive(Clone, Copy)]
struct StructuralNatV1 {
    value: cranelift_codegen::ir::Value,
}
/// Compact private observation of a structural Nat minted from a checked host
/// reply. The scalar never enters Runtime IR or the Ken surface: only the
/// Zero/Suc eliminators below can observe it.
#[derive(Clone, Copy)]
struct BoundedNatV1 {
    value: cranelift_codegen::ir::Value,
}
impl BoundedNatV1 {
    fn mint_after_reply_validation(value: cranelift_codegen::ir::Value) -> Self {
        Self { value }
    }

    fn predecessor(self, builder: &mut FunctionBuilder<'_>) -> Self {
        Self::derived_from_validated(builder.ins().iadd_imm(self.value, -1))
    }

    fn derived_from_validated(value: cranelift_codegen::ir::Value) -> Self {
        Self { value }
    }
}
/// **`RT-CARRIER-BYTESPAN-OBSERVE` `D4b` / `AC-10` — the `ResponseBytes`
/// validity invariant, made STRUCTURAL.**
///
/// Since `D2`, [`Lowered::ResponseBytes`] means *a span that will be
/// dereferenced and copied*, so every instance must independently be a valid
/// span. Before `D4b` that invariant was carried by two call sites and a doc
/// comment: any code in this crate could write the braced literal and mint an
/// unestablished span, and nothing would say so.
///
/// **The submodule is the mechanism, and its depth is load-bearing.** A private
/// field is visible to the declaring module *and every descendant of it*. Every
/// construction site — the producer in `core`, the rebuild below, and both
/// `core::tests` controls — is a descendant of `lowering`, so a newtype declared
/// at `lowering` scope would be constructible by all of them and would refuse
/// nothing. Declaring it one level DOWN inverts that: `lowering` is the parent
/// of `safe_byte_span`, not a descendant, so the braced literal is `E0451`
/// everywhere outside these braces and the two constructors below are the only
/// way in.
///
/// **Field privacy alone was NOT enough, and the first `D4b` candidate proved
/// it** (Architect, on `450fff8b`). Closing the braced literal closes a
/// *spelling*; it does not close *provenance*. That candidate paired it with an
/// unconditional `established_by_caller(pointer, len)` visible to all of
/// `cranelift_backend`, so any production descendant could still mint a fresh
/// span from arbitrary SSA values without ever masking — and the candidate's own
/// compile-positive sibling was exactly that construction. Named and greppable
/// is an auditable convention, not a mechanism.
///
/// **The production census, which is the actual `AC-10` claim.** Exactly one
/// production entry point creates a span from nothing, and it masks:
///
/// - [`SafeByteSpan::masked_at_producer`] — emits the `select` pair itself.
/// - [`SafeByteSpan::rebuild_from_collected`] — takes `self`, so it is
///   unreachable without an existing span. That is a **bearer condition on the
///   caller**, not a constraint on the values it returns; see its own doc.
/// - `for_control` — `#[cfg(test)]`, so it does not exist in a production
///   build at all.
///
/// ⇒ a newly added production construction has no raw mint to reach for: it
/// must either mask, or already hold a span that did.
///
/// **What this does and does not establish — narrowed by Architect
/// `dec_5ghh87fvg7skn`, and the narrowing is the point.** `pointer` and `len`
/// are Cranelift SSA values, opaque at Rust compile time, so no mechanism here
/// can *verify* that a span points at live memory. What is structural is the
/// closure of the **construction authority**: reaching a `SafeByteSpan` at all
/// requires either the masking mint or possession of a prior span, and that
/// chain the compiler does enforce.
///
/// **It is NOT a claim about the VALUES.** `rebuild_from_collected` discards
/// its receiver, so a holder of any span can wrap arbitrary SSA values in a new
/// one. The provenance of a rebuilt `pointer, len` pair is guarded by the single
/// current call site and by review — **not by mechanism.** The `⇒` above is the
/// exact and complete statement; anything stronger is false.
mod safe_byte_span {
    use super::{types, FunctionBuilder, InstBuilder};

    /// A `{pointer, len}` byte span whose CONSTRUCTION is rooted in a masking
    /// mint. The fields are private to this module by design — see the
    /// module-level note, which also states what the type does not carry.
    #[derive(Clone, Copy)]
    pub(in crate::cranelift_backend) struct SafeByteSpan {
        pointer: cranelift_codegen::ir::Value,
        len: cranelift_codegen::ir::Value,
    }

    impl SafeByteSpan {
        /// **Self-establishing mint: the mask is emitted HERE, not at the call
        /// site** (Architect `dec_12s3j2gj67c66`).
        ///
        /// The unselected arm becomes the canonical empty span `{null, 0}`,
        /// whose copy loop runs zero times. Because the `select` pair lives
        /// inside this constructor, a producer that holds a `success`
        /// discriminant cannot obtain a span that skips it: there is no
        /// argument ordering, and no later edit to the call site, that yields
        /// an unmasked value from this function.
        pub(in crate::cranelift_backend) fn masked_at_producer(
            builder: &mut FunctionBuilder<'_>,
            pointer_type: cranelift_codegen::ir::Type,
            pointer: cranelift_codegen::ir::Value,
            len: cranelift_codegen::ir::Value,
            success: cranelift_codegen::ir::Value,
        ) -> Self {
            let null = builder.ins().iconst(pointer_type, 0);
            let empty = builder.ins().iconst(types::I64, 0);
            Self {
                pointer: builder.ins().select(success, pointer, null),
                len: builder.ins().select(success, len, empty),
            }
        }

        /// **A BEARER CONDITION, not a dataflow proof** (Architect
        /// `dec_5ghh87fvg7skn`). Taking `self` means a caller that does not
        /// already hold a span cannot reach this at all — that much the compiler
        /// enforces, and it is what lets production keep the rebuild without
        /// granting every producer a fresh raw mint.
        ///
        /// **The receiver is then DISCARDED.** The returned span's values are
        /// this call's two arguments and nothing else, so a holder of any span
        /// can wrap arbitrary SSA values in a new one. Read `self` as a warrant
        /// proving the caller was already inside the construction surface — it
        /// does not make the result derived from the old span.
        ///
        /// `d9_collect` takes a span apart into a flat value list and
        /// `rebuild_recursive_argument` puts it back together. **That single
        /// caller is correct because it consumes the loop-header parameters in
        /// the same structural order the original span was flattened in — a
        /// local call-site fact, verified by review, that this signature does
        /// not carry.** A second caller would inherit none of it.
        ///
        /// The values cannot be re-derived from `self` here, and that is not a
        /// deferred repair: at the rebuild site `self` holds the *preheader* SSA
        /// handles while the arguments are freshly created *loop-header block
        /// parameters*. Reusing the receiver would discard the phi-like
        /// recursive values and can break dominance. A real value-provenance
        /// mechanism would have to own the whole flatten → block-parameter →
        /// rebuild mapping, which is a separate design question and not a
        /// comment-sized change.
        pub(in crate::cranelift_backend) fn rebuild_from_collected(
            self,
            pointer: cranelift_codegen::ir::Value,
            len: cranelift_codegen::ir::Value,
        ) -> Self {
            Self { pointer, len }
        }

        /// **Test-only control mint, deliberately unconstrained.**
        ///
        /// The two legitimate direct constructions in `core::tests` hand-build a
        /// span with no `success` discriminant to mask against. This does NOT
        /// constrain `len` against any source length: the `D2` edge control
        /// varies the declared length away from the true one precisely so the
        /// emitted guards are reachable, and a mint that clamped it would
        /// silently disarm that control.
        ///
        /// It is `#[cfg(test)]` so that this freedom cannot leak into a
        /// production build — that gating is what keeps the census above true.
        #[cfg(test)]
        pub(in crate::cranelift_backend) fn for_control(
            pointer: cranelift_codegen::ir::Value,
            len: cranelift_codegen::ir::Value,
        ) -> Self {
            Self { pointer, len }
        }

        pub(in crate::cranelift_backend) fn pointer(self) -> cranelift_codegen::ir::Value {
            self.pointer
        }

        pub(in crate::cranelift_backend) fn len(self) -> cranelift_codegen::ir::Value {
            self.len
        }
    }
}
/// **`AC-10`'s PRODUCTION-position probe: the census above, witnessed rather
/// than asserted.**
///
/// The sibling probe in `core::tests::constructors` witnesses that the braced
/// literal is refused. It cannot witness the census claim that *production has
/// no raw mint*, because it lives under `#[cfg(test)]`, where `for_control`
/// exists by construction. This module sits in production code, so it sees the
/// surface a new producer would actually see:
///
/// ```text
/// RUSTFLAGS='--cfg ken_ac10_production_mint_probe' \
///   ./scripts/ken-cargo build -p ken-runtime
/// ```
///
/// **Expected, and MEASURED on rustc 1.96.0 at this SHA:** exactly ONE error,
/// `E0599: no associated function ... named `for_control` ... in the current
/// scope`, on `refused_raw_mint`. `warranted_rebuild` compiles silently.
///
/// ⇒ **MEASURED:** production cannot reach the unconstrained mint.
/// **CLAIMED:** every production span is therefore *constructed* either by
/// [`SafeByteSpan::masked_at_producer`] or by a caller already holding one,
/// since those are the only two production routes.
/// **THE GAP — and `warranted_rebuild` is BOTH SIDES OF IT.** It proves the
/// refusal is about *minting* rather than about the fixture: it builds a
/// `ResponseBytes` in this same module and compiles, because it is handed a span
/// that already exists. The same three lines are the counterexample to the
/// stronger reading — its `pointer` and `len` are locally constructed and
/// unrelated to `existing`, and it compiles anyway. **So this probe witnesses
/// closure of the construction authority and simultaneously shows that the
/// VALUES are not constrained.** A positive control and a counterexample can be
/// the same code; do not read this probe as evidence of value provenance.
///
/// **This probe carries ONE refusal on purpose — measured, not assumed.** It
/// first also carried the braced literal, and enabling it reported only the
/// `E0599`: the resolution failure aborts the compilation before the privacy
/// pass runs, so the `E0451` never appeared and a reader counting errors would
/// have concluded the braced literal was *accepted* here. One refusal per probe
/// is the only shape that cannot mask its sibling. The braced literal is
/// witnessed separately by `ac10_evasion_probe`, and the privacy that refuses it
/// is module-scoped — `lowering` and its descendants are alike outside
/// `safe_byte_span` — so that probe covers this position too.
///
/// **The production profile is the only one that can see this.** Under
/// `--all-targets` the lib-test target enables `cfg(test)`, `for_control`
/// resolves, and `refused_raw_mint` would compile there — the same blindness
/// that let a `#[cfg(test)]` defect reach production past three approvals
/// earlier in this WP.
#[cfg(ken_ac10_production_mint_probe)]
mod ac10_production_mint_probe {
    use super::{Lowered, SafeByteSpan};

    type ProbeValue = cranelift_codegen::ir::Value;

    /// The test-only mint, reached from production. Must not compile: `E0599`.
    pub(super) fn refused_raw_mint(pointer: ProbeValue, len: ProbeValue) -> Lowered {
        Lowered::ResponseBytes(SafeByteSpan::for_control(pointer, len))
    }

    /// Non-vacuity: rebuilding through a span that already exists must compile.
    pub(super) fn warranted_rebuild(
        existing: SafeByteSpan,
        pointer: ProbeValue,
        len: ProbeValue,
    ) -> Lowered {
        Lowered::ResponseBytes(existing.rebuild_from_collected(pointer, len))
    }
}
#[derive(Clone)]
struct DynamicConstructorV1 {
    discriminator: cranelift_codegen::ir::Value,
    alternatives: Vec<DynamicConstructorAlternativeV1>,
}
#[derive(Clone)]
struct DynamicConstructorAlternativeV1 {
    tag: i64,
    constructor: RuntimeSymbol,
    identity: ConstructorIdentity,
    /// **`D7` — the planner's occurrence for this exact alternative.**
    ///
    /// ⭐ A dynamic SET is not an allocation; a selected ALTERNATIVE is. It
    /// calls `emit_carrier_alloc` exactly as a fixed constructor does, so it
    /// needs the same path-keyed record — otherwise its allocation has no
    /// lifetime meet and cannot enter the event-to-record relation.
    ///
    /// `None` is a refusal, never a default: it means no context was being
    /// defined, and the allocation fails loudly rather than borrowing a lane.
    occurrence: Option<AggregateOccurrenceId>,
    fields: Vec<Lowered>,
}




























/// **`RT-DECL-CLOSURE-PORT` `D7` — the seats of ONE visit, bound to the operands
/// they were claimed from.**
///
/// ⭐ **This is what replaces the bulk pre-operation conversion.** That
/// conversion crossed every operand to a specialized template before the
/// operation was known, so a seat that could not be read specialized failed as
/// "a host-effect operand" — a generic surface naming neither the operation nor
/// the seat. Here each arm names the exact slot it is reading, and a refusal
/// carries that seat's operation and need.
///
/// ⛔ The record comes from the claim, not from a fresh lookup. Re-resolving it
/// here would let the arm read a seat the visit never claimed, which is exactly
/// the binding the claim group exists to establish.
struct ClaimedEffectSeats<'a> {
    claimed: &'a BTreeMap<EffectSeatSlot, PlannedEffectSeat>,
    capability: Option<&'a LoweringOperand>,
    arguments: &'a [LoweringOperand],
}

impl<'a> ClaimedEffectSeats<'a> {
    /// The claimed record and the operand it was claimed from.
    fn operand(
        &self,
        slot: EffectSeatSlot,
    ) -> Result<(PlannedEffectSeat, &'a LoweringOperand), CraneliftBackendError> {
        let record = *self.claimed.get(&slot).ok_or_else(|| {
            unsupported(
                "Effect",
                format!("{slot:?} was not claimed in this visit, so nothing authorizes reading it"),
            )
        })?;
        let operand = match slot {
            EffectSeatSlot::Capability => self.capability.ok_or_else(|| {
                unsupported(
                    "Effect",
                    "the capability seat was claimed but no capability operand was lowered",
                )
            })?,
            EffectSeatSlot::Argument(ordinal) => {
                self.arguments.get(ordinal as usize).ok_or_else(|| {
                    unsupported(
                        "Effect",
                        format!("{slot:?} was claimed but no operand was lowered at it"),
                    )
                })?
            }
        };
        Ok((record, operand))
    }

    /// A view that authorizes NOTHING.
    ///
    /// ⭐ For the constructor-reconciliation tests, which build a synthesized
    /// node directly rather than through a host-effect visit. It is the honest
    /// spelling of what `&[]` used to say: this caller claimed no seat, so any
    /// declared `SiteOperand` child refuses. A tree with no site-bound child —
    /// which is every tree these tests build — asks it for nothing.
    fn none() -> ClaimedEffectSeats<'static> {
        static NONE: BTreeMap<EffectSeatSlot, PlannedEffectSeat> = BTreeMap::new();
        ClaimedEffectSeats { claimed: &NONE, capability: None, arguments: &[] }
    }

    /// Read one seat's compile-time template.
    ///
    /// ⛔ **Exhaustive over the two phases, and the refusal names the SEAT.**
    /// The carried arm is not dead code that could be a wildcard: it is the arm
    /// that would fire if a seat's `Avail` were ever widened without a carried
    /// route being written for it, and it must say which seat and which need
    /// rather than "a host-effect operand".
    fn specialized(&self, slot: EffectSeatSlot) -> Result<&'a Lowered, CraneliftBackendError> {
        let (record, operand) = self.operand(slot)?;
        match operand {
            LoweringOperand::Specialized(lowered) => Ok(lowered),
            LoweringOperand::Carried(_) => Err(unsupported(
                "Effect",
                format!(
                    "seat {:?} of {:?} needs {:?}, which this release can observe only in a \
                     specialized template, but this visit holds a carried word",
                    record.slot, record.operation, record.need
                ),
            )),
        }
    }
}





/// What makes one lowered value distinguishable from another at a site.
///
/// ⛔ **`None` is a refusal.** A value this cannot witness is one whose identity
/// the reconciliation cannot establish, so a site-bound child holding it fails
/// rather than being accepted on the strength of its shape. The alternative —
/// a permissive fallback — would reopen the substitution for exactly the
/// variants nobody thought about.
#[derive(Clone, Debug, Eq, PartialEq)]
enum SiteOperandWitness {
    Values(Vec<cranelift_codegen::ir::Value>),
    Bytes(Vec<u8>),
}

/// The witness of one lowered value, or `None` if it has none.
///
/// The CLIF `Value` numbering is per function, which is precisely what makes it
/// a discriminator here: two different operands of one seat, in one function
/// body, hold different values even when their shapes agree.
fn site_operand_witness(value: &Lowered) -> Option<SiteOperandWitness> {
    use SiteOperandWitness::{Bytes, Values};
    match value {
        Lowered::ResourceToken { value } | Lowered::CapabilityToken { value } => {
            Some(Values(vec![*value]))
        }
        Lowered::BorrowedNativeValue { pointer } => Some(Values(vec![*pointer])),
        Lowered::ResponseBytes(span) => Some(Values(vec![span.pointer(), span.len()])),
        Lowered::Int { value, .. } | Lowered::Bool { value, .. } => Some(Values(vec![*value])),
        // The nat wrappers carry a validated payload rather than a bare CLIF
        // value; their inner value is what distinguishes two of them.
        Lowered::BoundedNat(nat) => Some(Values(vec![nat.value])),
        Lowered::StructuralNat(nat) => Some(Values(vec![nat.value])),
        Lowered::Bytes(content) => Some(Bytes(content.clone())),
        Lowered::String(text) => Some(Bytes(text.as_bytes().to_vec())),
        _ => None,
    }
}


const MALFORMED_DYNAMIC_CONSTRUCTOR_STATUS: i64 = -3;
fn validate_dynamic_constructor_alternatives<'a>(
    alternatives: impl IntoIterator<Item = (i64, &'a str)>,
) -> Result<(), CraneliftBackendError> {
    let mut tags = BTreeSet::new();
    let mut constructors = BTreeSet::new();
    let mut count = 0;
    for (tag, constructor) in alternatives {
        count += 1;
        if !tags.insert(tag) {
            return Err(unsupported(
                "DynamicConstructor",
                format!("duplicate alternative tag {tag}"),
            ));
        }
        if !constructors.insert(constructor) {
            return Err(unsupported(
                "DynamicConstructor",
                format!("duplicate alternative constructor {constructor}"),
            ));
        }
    }
    if count == 0 {
        return Err(unsupported(
            "DynamicConstructor",
            "closed alternative table is empty",
        ));
    }
    Ok(())
}
/// Selects the source case for one dynamic-constructor alternative.
///
/// ⭐ Returns the case's **index** alongside it: the selection is a search by
/// constructor name, and a search recovers no position. The caller needs the
/// index to derive the body's static origin positionally
///.
fn select_dynamic_constructor_case<'a>(
    cases: &'a [crate::RuntimeMatchCase],
    alternative: &DynamicConstructorAlternativeV1,
    default: &'a RuntimeTrap,
) -> Result<Result<(usize, &'a crate::RuntimeMatchCase), &'a RuntimeTrap>, CraneliftBackendError> {
    let mut selected = cases
        .iter()
        .enumerate()
        .filter(|(_, case)| case.constructor == alternative.constructor);
    let Some((index, case)) = selected.next() else {
        return Ok(Err(default));
    };
    if selected.next().is_some() {
        return Err(unsupported(
            "DynamicConstructor",
            format!(
                "source match duplicates constructor {}",
                alternative.constructor
            ),
        ));
    }
    if case.binders != alternative.fields.len() {
        return Err(unsupported(
            "DynamicConstructor",
            format!(
                "case {} expects {} binders but alternative has {} fields",
                case.constructor,
                case.binders,
                alternative.fields.len()
            ),
        ));
    }
    Ok(Ok((index, case)))
}
fn materialize_dynamic_constructor_env(
    alternative: &DynamicConstructorAlternativeV1,
    env: &[LoweringEnvironmentBinding],
) -> Vec<LoweringEnvironmentBinding> {
    env_with(alternative.fields.clone(), env)
}
fn console_stream_tag(value: &Lowered) -> Option<i64> {
    let Lowered::Constructor {
        constructor, args, ..
    } = value
    else {
        return None;
    };
    if !args.is_empty() {
        return None;
    }
    if constructor.ends_with("::Stdin") {
        Some(0)
    } else if constructor.ends_with("::Stdout") {
        Some(1)
    } else if constructor.ends_with("::Stderr") {
        Some(2)
    } else {
        None
    }
}
fn create_policy_tag(value: &Lowered) -> Option<i64> {
    let Lowered::Constructor {
        constructor, args, ..
    } = value
    else {
        return None;
    };
    if !args.is_empty() {
        return None;
    }
    if constructor.ends_with("::CreateNew") {
        Some(0)
    } else if constructor.ends_with("::CreateOrTruncate") {
        Some(1)
    } else if constructor.ends_with("::CreateOrKeep") {
        Some(2)
    } else {
        None
    }
}
fn resource_open_mode_tag(value: &Lowered) -> Option<i64> {
    let Lowered::Constructor {
        constructor, args, ..
    } = value
    else {
        return None;
    };
    if constructor.ends_with("::ResourceRead") && args.is_empty() {
        Some(0)
    } else if constructor.ends_with("::ResourceMetadata") && args.is_empty() {
        Some(1)
    } else if constructor.ends_with("::ResourceWriteCreate") && args.len() == 1 {
        // A worker field means this is not a recognized open mode. `None` is
        // the conservative answer for an `Option`-returning classifier: the
        // caller treats it as unrecognized rather than as a known tag.
        create_policy_tag(args[0].specialized_at("a resource open mode field").ok()?)
            .map(|tag| tag + 2)
    } else {
        None
    }
}
fn dynamic_host_result_producer_case<'a>(
    cases: &'a [crate::RuntimeMatchCase],
    constructor: &str,
) -> Result<Option<(usize, &'a crate::RuntimeMatchCase)>, CraneliftBackendError> {
    let Some((index, case)) = cases
        .iter()
        .enumerate()
        .find(|(_, case)| case.constructor == constructor)
    else {
        return Ok(None);
    };
    if case.binders != 1 {
        return Err(unsupported(
            "ComputationalMatch",
            format!(
                "dynamic HostResult tree producer case {} expects exactly one binder, got {}",
                case.constructor, case.binders
            ),
        ));
    }
    Ok(Some((index, case)))
}
/// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the affine splice capability's
/// identity.**
///

#[derive(Clone, Copy)]
struct ComputationalEliminatorFrame<'a> {
    /// `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the affine splice capability this
    /// edge carries, `None` on every edge that is not the splice's own.
    ///
    /// The id travels on the frame because the frame IS the splice's pending
    /// semantic edge; the outstanding-set on `Lowering` is what makes it affine,
    /// because this struct is `Copy` and a token on a `Copy` value can be
    /// duplicated. Copying the frame copies the id; only one copy can spend it.
    cases: &'a [crate::RuntimeComputationalMatchCase],
    default: &'a RuntimeTrap,
    env: &'a [LoweringEnvironmentBinding],
    /// The origin of the computational-match occurrence these cases belong to.
    /// Case *i*'s body is `child(static_origin, 1 + i)`.
    static_origin: StaticOriginId,
    retained_scrutinee_index: Option<usize>,
    deferred_constructor_case: Option<&'a DeferredConstructorCaseEnvironment<'a>>,
    provenance: RecursorFrameProvenance,
    checked_frame_id: Option<u64>,
    checked_invocation_id: Option<u64>,
    checked_invocation_source: Option<InvocationTemplateRef>,
    checked_invocation_depth: usize,
    /// **`RT-DECL-CLOSURE-PORT` `D6a` — the closed answer-route fact, carried
    /// rather than re-derived.**
    ///
    /// ⭐⭐ This field is the whole checkpoint. `SourceContinuation::
    /// ComputationalMatchScrutinee` already holds it, the **specialized** arm
    /// already reads it, and the **carried** arm used to build this frame
    /// without it — so a checked recursive answer arriving as a carried word was
    /// asked only whether its tag matched an ordinary case, and took the closed
    /// default when it did not. `D6` activation was the first execution of the
    /// functionized artifact and therefore the first time that drop could be
    /// observed at all.
    ///
    /// ⛔ **Threaded, never inferred.** It is not to be recovered from a tag, a
    /// body, a constructor spelling, a frame id, or the presence of a
    /// continuation unit. Every construction site that is not that source
    /// continuation supplies [`SourceComputationalAnswerRoute::DirectScrutinee`],
    /// which keeps the existing closed default exactly as it was.
    answer_route: SourceComputationalAnswerRoute,
}
/// **`RT-DECL-CLOSURE-PORT` `D5a` checkpoint 4 step 1 — one carried
/// invocation's RETAINED SOURCE COORDINATES.**
///
/// The exact computational-match occurrence the invocation resumes and the
/// ruled recursive position it occupies, both read straight off the invocation
/// segment that is already in scope at the call seam.
///
/// ⛔ Deliberately carries no body origin, no ABI shape and no context id.
/// Those are what the binding must NOT be keyed on, and a coordinate record
/// that cannot hold them is a stronger guarantee than a rule saying not to use
/// them.
#[derive(Clone, Copy)]
struct CarriedInvocationCoordinates {
    continuation_origin: StaticOriginId,
    recursive_position: u32,
}

impl CarriedInvocationCoordinates {
    /// Read the coordinates off the invocation segment.
    ///
    /// ⛔ Fails closed on a sibling position that does not fit the planner's
    /// width rather than truncating: a truncated position would silently select
    /// a different recursive field's binding.
    fn of(segment: &RecursorInvocationSegment) -> Result<Self, CraneliftBackendError> {
        Ok(Self {
            continuation_origin: segment.selection.static_origin,
            recursive_position: u32::try_from(segment.sibling_position).map_err(|_| {
                unsupported(
                    "ContinuationSpecialization",
                    "a carried invocation's sibling position exceeds the planner's recursive                      position width",
                )
            })?,
        })
    }
}

#[derive(Clone, Copy)]
struct OrdinaryEliminatorFrame<'a> {
    cases: &'a [crate::RuntimeMatchCase],
    default: &'a RuntimeTrap,
    env: &'a [LoweringEnvironmentBinding],
    /// The origin of the **match occurrence these cases belong to**. Case *i*'s
    /// body is `child(static_origin, 1 + i)`; see `SourceContinuation::
    /// MatchScrutinee` for why one parent origin beats a per-case vector.
    static_origin: StaticOriginId,
    retained_scrutinee_index: Option<usize>,
    deferred_constructor_case: Option<&'a DeferredConstructorCaseEnvironment<'a>>,
}
#[derive(Clone, Copy)]
struct PendingLetContinuationFrame<'a> {
    /// ⭐ The same phase-bearing edge as
    /// [`Lowered::ComputationalRecursorClosure::residual`], borrowed: a pending
    /// `Let` resumes on exactly the value the capsule it came from carries, so
    /// narrowing it back to `&Lowered` here would reintroduce the boundary one
    /// frame later.
    residual: &'a LoweringOperand,
    args: &'a [RuntimeExpr],
    /// The origin of the `Call` occurrence `args` belong to; argument *i* is
    /// `child(call_origin, 1 + i)`.
    call_origin: StaticOriginId,
    env: &'a [LoweringEnvironmentBinding],
    recursive_unit_body: Option<StaticOriginId>,
}
#[derive(Clone, Copy)]
struct ActiveContinuationFrame<'a> {
    activation: ContinuationActivationId,
    cursor: ContinuationCursorId,
    parent: Option<&'a ActiveContinuationFrame<'a>>,
    pending: &'a [EliminatorFrame<'a>],
    selected_ancestry: &'a [RecursorFrameProvenance],
    source_lineage: &'a [SourceSelectedContinuation<'a>],
    source_selected_cursor: Option<ContinuationCursorId>,
    selected_scope: Option<&'a OwnedSelectedScope>,
}
#[derive(Clone)]
struct ComputationalRecursorLayer {
    cases: Vec<crate::RuntimeComputationalMatchCase>,
    default: RuntimeTrap,
    outer_env: Vec<LoweringEnvironmentBinding>,
    /// The origin of the computational-match occurrence these cases came from,
    /// carried with the clone so a resumed selection can still derive a case
    /// body's origin positionally.
    static_origin: StaticOriginId,
    provenance: RecursorFrameProvenance,
    role: RecursorLayerRole,
    checked_frame_id: Option<u64>,
    checked_invocation_id: Option<u64>,
    checked_invocation_source: Option<InvocationTemplateRef>,
    checked_invocation_depth: usize,
    semantic_pending: bool,
}
#[derive(Clone)]
struct RecursorInvocationSegment {
    /// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — whether THIS segment consumed
    /// the splice's capability.**
    ///
    /// Recorded where the capability is spent, not asked for later. It is the
    /// receipt for an affine consumption, never an observation of the segment's
    /// own shape: a segment does not become `Composed` by presenting an extra
    /// layer, and one that does so without the capability is refused for exact
    /// coverage exactly as before.
    origin: RecursorProducerOriginId,
    /// Declaration-order field position inside the one selected constructor
    /// case. Siblings share `origin`; this position distinguishes their
    /// immutable carriers through the consumer boundary.
    sibling_position: usize,
    selection: ComputationalRecursorLayer,
    unwind: RecursorUnwindStack,
    resume_cursor: ContinuationCursorId,
    checked_invocation: Option<CheckedRecursiveInvocationInstance>,
    computational_ih_slot_template_id: Option<u64>,
    /// The declared retained-body unit for a callable recursive position.
    ///
    /// `None` is the ordinary structural-data IH: it resumes the eliminator
    /// over its carried value and accepts no source arguments.
    recursive_unit_body: Option<StaticOriginId>,
    /// Inert handles into `Lowering::dynamic_splice_edges`. Cloning a lowered
    /// recursor can copy a handle, but only one clone can consume the unique
    /// compiler-owned edge; every replay rejects before CFG.
    dynamic_splice_edges: Vec<DynamicSpliceEdgeId>,
    /// Immutable mint-time witness for every already-open control extent.
    /// Qualification may attach a fresh invocation identity later, but it may
    /// not delete, duplicate, reorder, or transplant an exit obligation.
    open_control_obligations: Vec<OpenControlObligation>,
}
#[derive(Clone)]
struct RecursorUnwindStack {
    later_wrappers_in_construction_order: Vec<ComputationalRecursorLayer>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct OpenControlObligation {
    scope_origin: RecursorProducerOriginId,
    parent_scope: Option<RecursorProducerOriginId>,
    checked_frame_id: Option<u64>,
    semantic_pending: bool,
}
fn open_control_obligations(unwind: &RecursorUnwindStack) -> Vec<OpenControlObligation> {
    unwind
        .later_wrappers_in_construction_order
        .iter()
        .filter_map(|layer| match layer.role {
            RecursorLayerRole::ExitsScope {
                scope_origin,
                parent_scope,
                ..
            } => Some(OpenControlObligation {
                scope_origin,
                parent_scope,
                checked_frame_id: layer.checked_frame_id,
                semantic_pending: layer.semantic_pending,
            }),
            RecursorLayerRole::SelectsOccurrence { .. } => None,
        })
        .collect()
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AffineSpliceState {
    Open,
    Consumed,
}
/// Move-only compiler capability for one oriented segment splice.  It is
/// deliberately not `Clone`; validation consumes the sole open state before
/// any CFG or consumer lowering begins.
struct AffineSpliceCapability {
    state: AffineSpliceState,
}
impl AffineSpliceCapability {
    fn consume(&mut self) -> Result<(), CraneliftBackendError> {
        if std::mem::replace(&mut self.state, AffineSpliceState::Consumed)
            == AffineSpliceState::Consumed
        {
            return Err(unsupported(
                "OrientedSubcontinuation",
                "affine splice capability was consumed more than once",
            ));
        }
        Ok(())
    }
}
#[derive(Clone)]
struct OrientedControlLedgerEntry {
    frame_id: Option<u64>,
    invocation_id: Option<u64>,
    role: RecursorLayerRole,
    checked_witness: Option<crate::OrientedControlWitnessV1>,
}
fn oriented_layer_is_pending_semantic(layer: &ComputationalRecursorLayer) -> bool {
    layer.semantic_pending
}
fn validate_oriented_control_projection(
    producer_origin: RecursorProducerOriginId,
    layers: &[ComputationalRecursorLayer],
) -> Result<(), CraneliftBackendError> {
    let mut invocation_sources = BTreeMap::new();
    let mut open_scopes = BTreeMap::new();
    for layer in layers {
        let role_origin = match layer.role {
            RecursorLayerRole::SelectsOccurrence { origin }
            | RecursorLayerRole::ExitsScope { origin, .. } => origin,
        };
        if role_origin != producer_origin {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "control occurrence was transplanted across producer regions",
            ));
        }
        match (layer.checked_invocation_id, layer.checked_invocation_source) {
            (Some(instance), Some(source)) => {
                if invocation_sources
                    .insert(instance, source)
                    .is_some_and(|old| old != source)
                {
                    return Err(unsupported(
                        "OrientedSubcontinuationPlanV1",
                        "one invocation instance is shared by distinct checked templates",
                    ));
                }
            }
            (None, Some(_)) => {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "checked invocation source has no affine instance identity",
                ));
            }
            _ => {}
        }
        match (layer.role, layer.semantic_pending) {
            (RecursorLayerRole::SelectsOccurrence { .. }, false) => {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "pending selection was misclassified as control-only",
                ));
            }
            _ => {}
        }
        if let RecursorLayerRole::ExitsScope {
            scope_origin,
            parent_scope,
            ..
        } = layer.role
        {
            if layer.checked_invocation_id.is_some() {
                if open_scopes.insert(scope_origin, parent_scope).is_some() {
                    return Err(unsupported(
                        "OrientedSubcontinuationPlanV1",
                        "open control obligation is duplicated",
                    ));
                }
            }
        }
    }
    for parent in open_scopes.values().flatten() {
        if !open_scopes.contains_key(parent) {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "open control obligation has a stale or cross-region parent",
            ));
        }
    }
    Ok(())
}
struct OwnedOrientedSubcontinuationSegment {
    producer_origin: RecursorProducerOriginId,
    sibling_position: usize,
    activation: ContinuationActivationId,
    segment_site_id: Option<u64>,
    input_interface: Option<crate::CheckedAnswerInterfaceV1>,
    output_interface: Option<crate::CheckedAnswerInterfaceV1>,
    semantic_frames: Vec<ComputationalRecursorLayer>,
    control_ledger: Vec<OrientedControlLedgerEntry>,
    resume_cursor: ContinuationCursorId,
    capability: AffineSpliceCapability,
}
struct InstalledOrientedSubcontinuationSegment {
    checked: bool,
    producer_origin: RecursorProducerOriginId,
    sibling_position: usize,
    activation: ContinuationActivationId,
    semantic_frames: Vec<ComputationalRecursorLayer>,
    control_ledger: Vec<OrientedControlLedgerEntry>,
    resume_cursor: ContinuationCursorId,
}
impl RecursorInvocationSegment {
    fn new(
        origin: RecursorProducerOriginId,
        sibling_position: usize,
        selection: ComputationalRecursorLayer,
        unwind: RecursorUnwindStack,
        resume_cursor: ContinuationCursorId,
        checked_invocation: Option<CheckedRecursiveInvocationInstance>,
        computational_ih_slot_template_id: Option<u64>,
    ) -> Self {
        let open_control_obligations = open_control_obligations(&unwind);
        Self {
            origin,
            sibling_position,
            selection,
            unwind,
            resume_cursor,
            checked_invocation,
            computational_ih_slot_template_id,
            recursive_unit_body: None,
            dynamic_splice_edges: Vec::new(),
            open_control_obligations,
        }
    }

    fn validate_open_control_obligations(&self) -> Result<(), CraneliftBackendError> {
        if open_control_obligations(&self.unwind) != self.open_control_obligations {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "open control obligation set changed after affine mint",
            ));
        }
        Ok(())
    }
}
/// Splits a recursor capsule into the value it continues on and its control
/// payload.
///
/// ⭐ **`AC-C4`: both halves of the signature are phase-bearing.** The input is
/// an operand because a capsule may itself arrive carried-adjacent; the output
/// is an operand because the residual may now be a carried word. ⛔ Every
/// invocation site must classify the returned base **exhaustively, with no
/// wildcard** — `Specialized(BoundedNat | Closure)` keep their existing
/// meanings byte-for-byte, and `Carried` installs the already-checked
/// invocation segment and resumes the same computational eliminator over that
/// word. ⛔ Never `specialized_at`, never a reconstructed `Lowered`, never back
/// through the producer.
fn decompose_computational_recursor(
    value: LoweringOperand,
) -> (
    LoweringOperand,
    Option<(ContinuationActivationId, RecursorInvocationSegment)>,
) {
    match value {
        LoweringOperand::Specialized(Lowered::ComputationalRecursorClosure {
            residual,
            activation,
            invocation,
        }) => (*residual, Some((activation, invocation))),
        // ⛔ Spelled rather than collapsed into a wildcard: a non-capsule
        // operand passes through in whichever phase it arrived, and the phase
        // set stays visibly closed.
        LoweringOperand::Specialized(value) => (LoweringOperand::Specialized(value), None),
        LoweringOperand::Carried(word) => (LoweringOperand::Carried(word), None),
    }
}
/// **`RT-LEXICAL-R3-FUSION-EMITTER` — the checked sequence a dynamic invocation
/// segment is measured against.**
///
/// The `SegmentComposition` selector and its `Composed` arm were retired with
/// the splice capability (Architect `evt_6bm54j10w1n88`): the capability was
/// their only producer, so after its removal no caller could ever have selected
/// the composed sequence. `composed_frame_templates` itself is NOT retired — it
/// is an encoded, decoded and validated field of `OrientedSubcontinuationPlanV1`
/// authored by the checked source, so it has a live producer and a live
/// validator, and the ruling's second branch applies: it is preserved, and this
/// path no longer reads it.
fn checked_invocation_frame_templates(
    plan: &crate::OrientedSubcontinuationPlanV1,
    source: InvocationTemplateRef,
) -> Result<Vec<u64>, CraneliftBackendError> {
    match source {
        // A same-SCC template carries no composition-time population, so a
        // composed segment on one covers exactly the ordinary sequence. That is
        // deliberate and fail-closed rather than an omission: an extra layer
        // there is refused by the exact-coverage check below, never absorbed.
        InvocationTemplateRef::SameSccCall(call_template_id) => plan
            .recursive_call(call_template_id)
            .map(|call| call.callee_frame_templates.clone())
            .ok_or_else(|| {
                unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "dynamic invocation has no checked same-SCC call template",
                )
            }),
        InvocationTemplateRef::ComputationalIHCall(call_template_id) => plan
            .computational_ih_call(call_template_id)
            .map(|call| call.callee_frame_templates.clone())
            .ok_or_else(|| {
                unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "dynamic invocation has no checked computational IH call template",
                )
            }),
    }
}
fn instantiate_checked_invocation_segment(
    plan: &crate::OrientedSubcontinuationPlanV1,
    invocation: CheckedRecursiveInvocationInstance,
    segment: &mut RecursorInvocationSegment,
) -> Result<(), CraneliftBackendError> {
    let frame_templates = checked_invocation_frame_templates(plan, invocation.source)?;
    let expected = frame_templates.iter().copied().collect::<BTreeSet<_>>();
    let mut instantiated = BTreeSet::new();
    // ⭐ Set equality alone cannot see a PERMUTATION (`dec_s30rdnb1dvgk` item
    // 5): two header-identical frames of the same callee declaration can be
    // exchanged and still cover `expected` exactly. Record the ORDER in which
    // identities are instantiated so the exact occurrence binding — not merely
    // the set — is checked below.
    let mut instantiated_order: Vec<u64> = Vec::new();
    // ⚠ A layer carrying no checked identity is not an error *here*: a segment
    // may legitimately contain unchecked layers. It becomes one only if the
    // callee's expected templates are then not covered, which is what
    // distinguishes "this layer need not instantiate" from "this layer must
    // and cannot say which occurrence it is."
    let mut layers_without_identity = 0usize;
    let mut visit = |layer: &mut ComputationalRecursorLayer| {
        let Some(frame_id) = layer.checked_frame_id else {
            layers_without_identity += 1;
            return Ok(());
        };
        let frame = plan.frame(frame_id).ok_or_else(|| {
            unsupported(
                "OrientedSubcontinuationPlanV1",
                "dynamic recursive layer has no checked frame entry",
            )
        })?;
        if frame.runtime_frame_fingerprint
            != crate::compiler_private_computational_match_frame_fingerprint(
                &layer.cases,
                &layer.default,
            )
        {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "dynamic recursive layer does not match its checked frame template",
            ));
        }
        if !expected.contains(&frame_id) {
            return Ok(());
        }
        match layer.checked_invocation_id {
            None => {
                layer.checked_invocation_id = Some(invocation.invocation_instance_id);
                layer.checked_invocation_source = Some(invocation.source);
                layer.checked_invocation_depth = invocation.semantic_depth;
            }
            Some(existing) if existing == invocation.invocation_instance_id => {
                if layer.checked_invocation_source != Some(invocation.source) {
                    return Err(unsupported(
                        "OrientedSubcontinuationPlanV1",
                        "dynamic invocation source changed after qualification",
                    ));
                }
            }
            Some(_) => return Ok(()),
        }
        if !instantiated.insert(frame_id) {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "one invocation instantiates a checked frame template more than once",
            ));
        }
        instantiated_order.push(frame_id);
        Ok(())
    };
    visit(&mut segment.selection)?;
    for layer in &mut segment.unwind.later_wrappers_in_construction_order {
        visit(layer)?;
    }
    if instantiated != expected {
        let actual = std::iter::once(&segment.selection)
            .chain(segment.unwind.later_wrappers_in_construction_order.iter())
            .map(|layer| (layer.checked_frame_id, layer.checked_invocation_id))
            .collect::<Vec<_>>();
        // ⭐ Name the missing-identity cause explicitly rather than letting it
        // read as ordinary coverage drift. Since `dec_s30rdnb1dvgk` removed the
        // fingerprint recovery in `make_computational_recursor`, an absent
        // `checked_frame_id` is the expected shape of a layer whose transported
        // marker was dropped — and it must never be recovered by inference.
        if layers_without_identity > 0 {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                format!(
                    "computational invocation {:?} has {layers_without_identity} layer(s) with no checked frame identity and does not cover its expected templates: expected={expected:?} instantiated={instantiated:?} actual={actual:?}",
                    invocation.source,
                ),
            ));
        }
        return Err(unsupported(
            "OrientedSubcontinuationPlanV1",
            format!(
                "computational invocation {:?} does not carry its exact checked frame sequence: expected={expected:?} instantiated={instantiated:?} actual={actual:?}",
                invocation.source,
            ),
        ));
    }
    // ⛔ **The permutation net.** `instantiated == expected` is set equality and
    // is therefore blind to an exchange of two header-identical frames — the
    // exact hazard `AC-F1` creates by design. Requiring the *sequence* to agree
    // pins each occurrence to its own identity.
    //
    // ⭐ The expected sequence is re-derived here from the plan's
    // `semantic_position`, deliberately **not** inherited from the order
    // `erasure.rs:1149` happened to emit — an ordering claim taken from its own
    // producer cannot detect that producer drifting.
    //
    // ⚠ Direction: `finish_match` assigns `semantic_position` in checked
    // postorder, and a segment is traversed **selection first**, which is the
    // reverse of that order. `oriented_test_plan` documents exactly this — its
    // frames are postorder `p2, p1, p0` while composition visits `0, 1, 2` and
    // `oriented_segment_keeps_semantic_and_control_axes_independent` asserts
    // the resulting semantic order is `[2, 1, 0]`.
    //
    // ⚠ This uses Runtime traversal order only to VALIDATE the checked
    // plan/marker binding. It does not mint semantic order from Runtime: the
    // plan's `semantic_position` is the authority and Runtime order is the
    // thing being checked against it.
    let mut planned_visit_order = frame_templates.clone();
    planned_visit_order.sort_by_key(|frame_id| {
        std::cmp::Reverse(plan.frame(*frame_id).map(|frame| frame.semantic_position))
    });
    if instantiated_order != planned_visit_order {
        return Err(unsupported(
            "OrientedSubcontinuationPlanV1",
            format!(
                "computational invocation {:?} instantiates its checked frames out of their planned occurrence order: planned={planned_visit_order:?} instantiated={instantiated_order:?}",
                invocation.source,
            ),
        ));
    }
    segment.checked_invocation = None;
    Ok(())
}
/// Test-only causal switch for the retired cross-instance flat ordering.
///
/// This is feature-gated so ordinary Runtime and CLI artifacts cannot select
/// the invalid ordering. PX8-DS integration tests use it to drive the exact
/// checked source through the former production consumer.
#[cfg(feature = "px8-ds-test-support")]
#[doc(hidden)]
pub fn with_px8ds_retired_flat_order<R>(run: impl FnOnce() -> R) -> R {
    struct Restore(bool);

    impl Drop for Restore {
        fn drop(&mut self) {
            PX8DS_RETIRED_FLAT_ORDER.with(|enabled| enabled.set(self.0));
        }
    }

    let previous = PX8DS_RETIRED_FLAT_ORDER.with(|enabled| enabled.replace(true));
    let _restore = Restore(previous);
    run()
}
fn px8ds_retired_flat_order_enabled() -> bool {
    #[cfg(any(test, feature = "px8-ds-test-support"))]
    {
        return PX8DS_RETIRED_FLAT_ORDER.with(std::cell::Cell::get);
    }
    #[cfg(not(any(test, feature = "px8-ds-test-support")))]
    {
        false
    }
}
fn compose_oriented_subcontinuation(
    plan: Option<&crate::OrientedSubcontinuationPlanV1>,
    invocation: Option<CheckedRecursiveInvocationInstance>,
    activation: ContinuationActivationId,
    mut segment: RecursorInvocationSegment,
    dynamic_splice_edges: Vec<DynamicSpliceEdge>,
) -> Result<InstalledOrientedSubcontinuationSegment, CraneliftBackendError> {
    // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the receipt travels ON the segment,
    // so composition is decided where the capability was spent rather than
    // re-derived here from ambient state. Nothing in this function can make a
    // segment composed.
    segment.validate_open_control_obligations()?;
    let invocation = invocation.or(segment.checked_invocation);
    if let Some(invocation) = invocation {
        let plan = plan.ok_or_else(|| {
            unsupported(
                "OrientedSubcontinuationPlanV1",
                "dynamic invocation has no checked oriented plan",
            )
        })?;
        instantiate_checked_invocation_segment(plan, invocation, &mut segment)?;
    }
    let producer_origin = segment.origin;
    let sibling_position = segment.sibling_position;
    let resume_cursor = segment.resume_cursor;
    let mut control_layers =
        Vec::with_capacity(1 + segment.unwind.later_wrappers_in_construction_order.len());
    control_layers.push(segment.selection);
    control_layers.extend(
        segment
            .unwind
            .later_wrappers_in_construction_order
            .into_iter()
            .rev(),
    );
    let mut control_ledger = control_layers
        .iter()
        .map(|layer| OrientedControlLedgerEntry {
            frame_id: layer.checked_frame_id,
            invocation_id: layer.checked_invocation_id,
            role: layer.role,
            checked_witness: None,
        })
        .collect::<Vec<_>>();
    validate_oriented_control_projection(producer_origin, &control_layers)?;
    #[cfg(test)]
    px8j_record_source_event(Px8jSourceTraceEvent::DirectConsume {
        origin: segment.origin,
        selection_cursor: segment.resume_cursor,
        sibling_position: segment.sibling_position,
        exits: control_layers
            .iter()
            .rev()
            .filter_map(|layer| match layer.role {
                RecursorLayerRole::ExitsScope {
                    scope_origin,
                    parent_scope,
                    ..
                } => Some((scope_origin, parent_scope)),
                RecursorLayerRole::SelectsOccurrence { .. } => None,
            })
            .collect(),
    });

    // A selected frame is pending semantic work. A freshly instantiated IH
    // layer remains semantic even when its control projection is already in
    // ExitsScope phase. Inherited exit rows carry no fresh invocation source:
    // their transformer was consumed at selection and they remain only as
    // affine open-extent obligations in the control ledger.
    let semantic_layers = control_layers
        .iter()
        .filter(|layer| oriented_layer_is_pending_semantic(layer))
        .cloned()
        .collect::<Vec<_>>();

    let planned = semantic_layers
        .iter()
        .map(|layer| (layer.checked_invocation_id, layer.checked_frame_id))
        .collect::<Vec<_>>();
    let has_planned = planned.iter().any(|(_, frame)| frame.is_some());
    if has_planned
        && planned
            .iter()
            .any(|(invocation, frame)| invocation.is_none() || frame.is_none())
    {
        let detail = semantic_layers
            .iter()
            .map(|layer| {
                (
                    layer.checked_frame_id,
                    layer.checked_invocation_id,
                    layer.checked_invocation_depth,
                    layer.provenance.0,
                    layer
                        .cases
                        .iter()
                        .map(|case| case.constructor.as_str())
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        return Err(unsupported(
            "OrientedSubcontinuationPlanV1",
            format!(
                "oriented segment mixes checked and inferred computational frames: {detail:?}; recursive templates: {:?}",
                plan.map(|plan| (
                    plan.recursive_calls
                        .iter()
                        .map(|call| (call.call_template_id, call.declaration.as_str(), call.callee.as_str()))
                        .collect::<Vec<_>>(),
                    plan.computational_ih_calls
                        .iter()
                        .map(|call| (call.call_template_id, call.declaration.as_str(), call.slot_template_id))
                        .collect::<Vec<_>>()
                ))
            ),
        ));
    }

    let (segment_site_id, input_interface, output_interface, semantic_frames) = if has_planned {
        let plan = plan.ok_or_else(|| {
            unsupported(
                "OrientedSubcontinuationPlanV1",
                "oriented segment has no checked plan metadata",
            )
        })?;
        plan.validate()
            .map_err(|reason| unsupported("OrientedSubcontinuationPlanV1", reason))?;
        for entry in &mut control_ledger {
            if entry.invocation_id.is_none() {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "checked control occurrence has no affine invocation identity",
                ));
            }
            let frame_id = entry.frame_id.expect("all control entries are checked");
            entry.checked_witness = Some(
                plan.frame(frame_id)
                    .expect("checked control entry has a validated plan row")
                    .control_witness
                    .clone(),
            );
        }
        let mut by_id = BTreeMap::<u64, Vec<u64>>::new();
        let mut layers_by_key = BTreeMap::new();
        for layer in semantic_layers {
            let frame_id = layer.checked_frame_id.expect("all frames are checked");
            let invocation_id = layer
                .checked_invocation_id
                .expect("all checked frames have an invocation instance");
            if layers_by_key
                .insert((invocation_id, frame_id), layer)
                .is_some()
            {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "oriented segment repeats a checked dynamic frame key",
                ));
            }
            by_id.entry(invocation_id).or_default().push(frame_id);
        }
        for frame_ids in by_id.values_mut() {
            frame_ids.sort_by_key(|frame_id| {
                plan.frame(*frame_id)
                    .expect("checked frame exists after plan validation")
                    .semantic_position
            });
            for pair in frame_ids.windows(2) {
                let left = plan.frame(pair[0]).expect("validated frame");
                let right = plan.frame(pair[1]).expect("validated frame");
                if left.segment_site_id != right.segment_site_id {
                    return Err(unsupported(
                        "OrientedSubcontinuationPlanV1",
                        "invocation-local oriented segment crosses checked prompt regions",
                    ));
                }
                if left.output_interface != right.input_interface {
                    return Err(unsupported(
                        "OrientedSubcontinuationPlanV1",
                        "invocation-local oriented segment endpoints do not compose",
                    ));
                }
            }
        }

        if px8ds_retired_flat_order_enabled() {
            let mut retired = layers_by_key
                .iter()
                .map(|((invocation_id, frame_id), layer)| {
                    (
                        *invocation_id,
                        plan.frame(*frame_id).expect("validated checked frame"),
                        layer,
                    )
                })
                .collect::<Vec<_>>();
            retired.sort_by_key(|(_, frame, layer)| {
                (
                    std::cmp::Reverse(layer.checked_invocation_depth),
                    frame.semantic_position,
                )
            });
            for pair in retired.windows(2) {
                if pair[0].1.output_interface != pair[1].1.input_interface {
                    return Err(unsupported(
                        "OrientedSubcontinuationPlanV1",
                        format!(
                            "retired flat oriented splice answer endpoints do not compose: left=(instance={}, frame={}, depth={}) right=(instance={}, frame={}, depth={})",
                            pair[0].0,
                            pair[0].1.frame_id,
                            pair[0].2.checked_invocation_depth,
                            pair[1].0,
                            pair[1].1.frame_id,
                            pair[1].2.checked_invocation_depth,
                        ),
                    ));
                }
            }
        }

        let mut edges_by_child = BTreeMap::new();
        let mut child_by_parent_frame = BTreeMap::new();
        for edge in dynamic_splice_edges {
            if edge.child_invocation_instance_id == edge.parent_invocation_instance_id {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "dynamic splice edge forms a self-parent cycle",
                ));
            }
            let child_frames = by_id
                .get(&edge.child_invocation_instance_id)
                .ok_or_else(|| {
                    unsupported(
                        "OrientedSubcontinuationPlanV1",
                        "dynamic splice edge names a stale child invocation",
                    )
                })?;
            let parent_frames = by_id.get(&edge.parent_invocation_instance_id);
            if parent_frames.is_some_and(|frames| !frames.contains(&edge.parent_frame_template_id))
            {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "dynamic splice edge names the wrong static parent frame",
                ));
            }
            let call = plan
                .computational_ih_call(edge.checked_call_template_id)
                .ok_or_else(|| {
                    unsupported(
                        "OrientedSubcontinuationPlanV1",
                        "dynamic splice edge names a stale checked call template",
                    )
                })?;
            // ---- `RT-LEXICAL-R3-FUSION-EMITTER` `DP` — the SECOND consumer of
            // ---- the authored membership, and it was invisible from the first.
            //
            // `child_frames` is the child invocation's actual frame sequence,
            // normalised to ascending `semantic_position` above. The plan-side
            // operand has to describe the same segment shape, so a composed
            // child is compared against the composed sequence.
            //
            // The ordinary operand is left EXACTLY as it was — the authored
            // sequence, unsorted, compared verbatim. Only the composed branch
            // derives and normalises, because only it concatenates two authored
            // sequences and cannot assume the result is already in position
            // order. Sorting the ordinary one too would have changed a landed
            // comparison for no reason `DP` needs.
            //
            // **Found by an advancing refusal, not by reading.** Closing the
            // mixed-frame guard moved the compile to *"dynamic splice edge
            // disagrees with its checked static parent"* here. One membership
            // law, two sites that encode it; the first refusal named only one.
            let expected_child_frames = call.callee_frame_templates.clone();
            if call.parent_frame_template_id != Some(edge.parent_frame_template_id)
                || call.parent_segment_site_id != Some(edge.segment_site_id)
                || call.callee_segment_site_id != edge.segment_site_id
                || expected_child_frames != *child_frames
            {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "dynamic splice edge disagrees with its checked static parent",
                ));
            }
            if call.result_interface != call.caller_interface {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "dynamic splice call result does not match its checked caller interface",
                ));
            }
            if edges_by_child
                .insert(edge.child_invocation_instance_id, edge)
                .is_some()
            {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "dynamic child invocation carries duplicate affine splice edges",
                ));
            }
        }
        let mut external_children = BTreeMap::new();
        for edge in edges_by_child.values() {
            if by_id.contains_key(&edge.parent_invocation_instance_id) {
                let key = (
                    edge.parent_invocation_instance_id,
                    edge.parent_frame_template_id,
                );
                if child_by_parent_frame
                    .insert(key, edge.child_invocation_instance_id)
                    .is_some()
                {
                    return Err(unsupported(
                        "OrientedSubcontinuationPlanV1",
                        "sibling dynamic invocations contend for one affine parent edge",
                    ));
                }
            } else {
                if edge.parent_invocation_instance_id != 0 {
                    return Err(unsupported(
                        "OrientedSubcontinuationPlanV1",
                        "dynamic splice edge names a stale parent invocation",
                    ));
                }
                if external_children
                    .insert(
                        edge.parent_frame_template_id,
                        edge.child_invocation_instance_id,
                    )
                    .is_some()
                {
                    return Err(unsupported(
                        "OrientedSubcontinuationPlanV1",
                        "sibling dynamic invocations compete for one external parent edge",
                    ));
                }
            }
        }
        let roots = if !external_children.is_empty() {
            if edges_by_child.len() != by_id.len() {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "dynamic splice edge deletion leaves an unparented invocation-local segment",
                ));
            }
            let mut roots = external_children.into_iter().collect::<Vec<_>>();
            roots.sort_by_key(|(parent_frame, _)| {
                plan.frame(*parent_frame)
                    .expect("validated external parent frame")
                    .semantic_position
            });
            roots.into_iter().map(|(_, child)| child).collect()
        } else {
            by_id
                .keys()
                .filter(|instance| !edges_by_child.contains_key(instance))
                .copied()
                .collect::<Vec<_>>()
        };
        if roots.is_empty() || (edges_by_child.len() < by_id.len() && roots.len() != 1) {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "dynamic splice edges do not form one exact invocation-local tree",
            ));
        }
        fn append_invocation_local_segment(
            invocation_id: u64,
            by_id: &BTreeMap<u64, Vec<u64>>,
            child_by_parent_frame: &BTreeMap<(u64, u64), u64>,
            visiting: &mut BTreeSet<u64>,
            completed: &mut BTreeSet<u64>,
            order: &mut Vec<(u64, u64)>,
        ) -> Result<(), CraneliftBackendError> {
            if completed.contains(&invocation_id) {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "dynamic splice edge is shared across sibling invocation paths",
                ));
            }
            if !visiting.insert(invocation_id) {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "dynamic splice edges form a parent cycle",
                ));
            }
            for frame_id in by_id
                .get(&invocation_id)
                .expect("validated invocation-local segment exists")
            {
                if let Some(child) = child_by_parent_frame.get(&(invocation_id, *frame_id)) {
                    append_invocation_local_segment(
                        *child,
                        by_id,
                        child_by_parent_frame,
                        visiting,
                        completed,
                        order,
                    )?;
                }
                order.push((invocation_id, *frame_id));
            }
            visiting.remove(&invocation_id);
            completed.insert(invocation_id);
            Ok(())
        }
        let mut order = Vec::new();
        let mut visiting = BTreeSet::new();
        let mut completed = BTreeSet::new();
        for root in roots {
            append_invocation_local_segment(
                root,
                &by_id,
                &child_by_parent_frame,
                &mut visiting,
                &mut completed,
                &mut order,
            )?;
        }
        if completed.len() != by_id.len() {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "dynamic splice tree leaves an invocation-local segment unreachable",
            ));
        }
        let mut ordered = order
            .into_iter()
            .map(|key| {
                let layer = layers_by_key
                    .remove(&key)
                    .expect("validated dynamic frame key exists");
                let frame = plan.frame(key.1).expect("validated checked frame exists");
                (key.0, frame, layer)
            })
            .collect::<Vec<_>>();
        let site = ordered
            .first()
            .expect("checked oriented segment is nonempty")
            .1
            .segment_site_id;
        if ordered
            .iter()
            .any(|(_, frame, _)| frame.segment_site_id != site)
        {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "oriented dynamic splice crosses checked prompt regions",
            ));
        }
        let input = ordered.first().unwrap().1.input_interface.clone();
        let output = ordered.last().unwrap().1.output_interface.clone();
        (
            Some(site),
            Some(input),
            Some(output),
            ordered.drain(..).map(|(_, _, layer)| layer).collect(),
        )
    } else {
        (None, None, None, semantic_layers)
    };

    let mut owned = OwnedOrientedSubcontinuationSegment {
        producer_origin,
        sibling_position,
        activation,
        segment_site_id,
        input_interface,
        output_interface,
        semantic_frames,
        control_ledger,
        resume_cursor,
        capability: AffineSpliceCapability {
            state: AffineSpliceState::Open,
        },
    };
    owned.capability.consume()?;
    debug_assert_eq!(owned.capability.state, AffineSpliceState::Consumed);
    debug_assert!(owned.control_ledger.len() >= owned.semantic_frames.len());
    debug_assert_eq!(
        owned.segment_site_id.is_some(),
        owned.input_interface.is_some()
    );
    debug_assert_eq!(
        owned.segment_site_id.is_some(),
        owned.output_interface.is_some()
    );
    Ok(InstalledOrientedSubcontinuationSegment {
        checked: owned.segment_site_id.is_some(),
        producer_origin: owned.producer_origin,
        sibling_position: owned.sibling_position,
        activation: owned.activation,
        semantic_frames: owned.semantic_frames,
        control_ledger: owned.control_ledger,
        resume_cursor: owned.resume_cursor,
    })
}
fn recursor_invocation_is_checked(segment: &RecursorInvocationSegment) -> bool {
    segment.selection.checked_frame_id.is_some()
        || segment
            .unwind
            .later_wrappers_in_construction_order
            .iter()
            .any(|layer| layer.checked_frame_id.is_some())
}
fn installed_oriented_eliminator_frames(
    segment: &InstalledOrientedSubcontinuationSegment,
) -> Vec<EliminatorFrame<'_>> {
    segment
        .semantic_frames
        .iter()
        .map(|layer| {
            EliminatorFrame::Computational(ComputationalEliminatorFrame {
                // `D3` — an INSTALLED segment's frames carry no capability: the
                // splice's edge already spent it to build this segment.
                cases: &layer.cases,
                default: &layer.default,
                env: &layer.outer_env,
                static_origin: layer.static_origin,
                retained_scrutinee_index: None,
                deferred_constructor_case: None,
                provenance: layer.provenance,
                checked_frame_id: layer.checked_frame_id,
                checked_invocation_id: layer.checked_invocation_id,
                checked_invocation_source: layer.checked_invocation_source,
                checked_invocation_depth: layer.checked_invocation_depth,
                answer_route: SourceComputationalAnswerRoute::DirectScrutinee,
            })
        })
        .collect()
}
/// Validate the control shape available at source-machine installation.
/// Parent adjacency is established by the return-hole continuation and belongs
/// only to the flattened-consumer validator below.
fn validate_recursor_invocation_install_shape(
    segment: &RecursorInvocationSegment,
) -> Result<(), CraneliftBackendError> {
    if !matches!(
        segment.selection.role,
        RecursorLayerRole::SelectsOccurrence { origin } if origin == segment.origin
    ) {
        return Err(unsupported(
            "ComputationalRecursor",
            "recursor selection role does not select the invocation origin",
        ));
    }
    let mut scope_origins = BTreeSet::new();
    for layer in &segment.unwind.later_wrappers_in_construction_order {
        let RecursorLayerRole::ExitsScope {
            origin,
            scope_origin,
            ..
        } = layer.role
        else {
            return Err(unsupported(
                "ComputationalRecursor",
                "recursor unwind role does not exit the invocation origin",
            ));
        };
        if origin != segment.origin {
            return Err(unsupported(
                "ComputationalRecursor",
                "recursor unwind role does not exit the invocation origin",
            ));
        }
        if !scope_origins.insert(scope_origin) {
            return Err(unsupported(
                "ComputationalRecursor",
                "recursor unwind repeats a selected scope identity",
            ));
        }
    }
    Ok(())
}
fn validate_recursor_invocation_segment(
    segment: &RecursorInvocationSegment,
) -> Result<(), CraneliftBackendError> {
    if !matches!(
        segment.selection.role,
        RecursorLayerRole::SelectsOccurrence { origin } if origin == segment.origin
    ) {
        return Err(unsupported(
            "ComputationalRecursor",
            "recursor selection role does not select the invocation origin",
        ));
    }
    // Construction order is outer-to-inner, while execution pops the vector
    // inner-to-outer. An outermost scope may name a parent owned by the caller;
    // every carried successor must link to the immediately preceding scope.
    let mut scope_origins = BTreeSet::new();
    let mut previous_scope = None;
    for layer in &segment.unwind.later_wrappers_in_construction_order {
        let RecursorLayerRole::ExitsScope {
            origin,
            scope_origin,
            parent_scope,
        } = layer.role
        else {
            return Err(unsupported(
                "ComputationalRecursor",
                "recursor unwind role does not exit the invocation origin",
            ));
        };
        if origin != segment.origin {
            return Err(unsupported(
                "ComputationalRecursor",
                "recursor unwind role does not exit the invocation origin",
            ));
        }
        if !scope_origins.insert(scope_origin) {
            return Err(unsupported(
                "ComputationalRecursor",
                "recursor unwind repeats a selected scope identity",
            ));
        }
        if let Some(previous_scope) = previous_scope {
            if parent_scope != Some(previous_scope) {
                return Err(unsupported(
                    "ComputationalRecursor",
                    "recursor unwind has a broken selected-scope parent link",
                ));
            }
        }
        previous_scope = Some(scope_origin);
    }
    Ok(())
}
fn active_recursor_frame<'a>(
    eliminators: &'a [EliminatorFrame<'a>],
) -> Option<&'a ActiveContinuationFrame<'a>> {
    eliminators.iter().find_map(|eliminator| match eliminator {
        EliminatorFrame::Active(frame) => Some(frame),
        EliminatorFrame::Computational(_)
        | EliminatorFrame::Ordinary(_)
        | EliminatorFrame::PendingLet(_)
        | EliminatorFrame::InvocationReturn => None,
    })
}
fn find_continuation_cursor<'a>(
    active: &'a ActiveContinuationFrame<'a>,
    cursor: ContinuationCursorId,
) -> Option<&'a ActiveContinuationFrame<'a>> {
    if active.cursor == cursor {
        Some(active)
    } else {
        active
            .parent
            .and_then(|parent| find_continuation_cursor(parent, cursor))
    }
}
fn active_context_contains_cursor(
    active: &ActiveContinuationFrame<'_>,
    cursor: ContinuationCursorId,
) -> bool {
    find_continuation_cursor(active, cursor).is_some()
        || active.source_selected_cursor == Some(cursor)
        || active.source_lineage.iter().rev().any(|candidate| {
            let candidate = candidate.as_active(active.source_lineage);
            find_continuation_cursor(&candidate, cursor).is_some()
        })
}
#[derive(Clone, Copy)]
enum EliminatorFrame<'a> {
    Computational(ComputationalEliminatorFrame<'a>),
    Ordinary(OrdinaryEliminatorFrame<'a>),
    PendingLet(PendingLetContinuationFrame<'a>),
    InvocationReturn,
    Active(ActiveContinuationFrame<'a>),
}
/// The source-evaluation continuation above a recursive-IH invocation.  This
/// is deliberately distinct from `EliminatorFrame`: source evaluation drains
/// this owned chain before its terminal may resume the outer eliminator cursor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SourceComputationalAnswerRoute {
    DirectScrutinee,
    CheckedSelectedRecursor,
}

/// **`RT-DECL-CLOSURE-PORT` `D6a` upstream half — one lowering predecessor's
/// operand paired with the route it arrived by.**
///
/// ⭐⭐ **The route is a property of the exact predecessor that supplied the
/// answer, not of the `ComputationalMatch` occurrence, its checked frame, or
/// its owner.** The census on the governed witness is what settles that:
/// `CSId(0)` and `CSId(1)` sit at the *same* continuation origin 10 and coexist
/// with the ordinary direct scrutinee of that origin, so any occurrence-global
/// projection marks the ordinary direct predecessor checked too.
///
/// ⛔ Compiler-path metadata only. It is **not** a field in the runtime word,
/// not a carrier lane, and not a runtime discriminator — nothing here reaches
/// the emitted CFG.
/// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — one continuation target's operands,
/// assembled at its exact call edge.**
///
/// The two runs are the target specialization's own `Parameter` and `Capture`
/// runs, in the planner's order. Separate fields rather than one operand
/// vector: a direct call concatenates them, a local composition consumes them
/// at two different seats, and a pre-concatenated run would force the second
/// caller to split on a length that neither seat owns.
struct ContinuationCallOperands {
    /// The facts the target's selected case body is lowered from, projected
    /// once here so the composition seat never re-derives them.
    body: units::ContinuationSelectedCaseBody,
    /// The owner of the source body this specialization lowers — the
    /// continuation's own consumer. Carried because the local composition binds
    /// it as the ambient defining unit, exactly as the definition pass does.
    consumer_owner: PredeclaredFunctionId,
    /// The planner's ordinary envelope, as the assembly consumed it. Carried
    /// so the body lowering reads the SAME sequence the operands were assembled
    /// from, including any test perturbation, rather than re-projecting it.
    envelope: Vec<ContinuationOrdinaryEnvelopeRole>,
    ordinary: Vec<LoweringOperand>,
    continuation_inputs: Vec<LoweringOperand>,
}

/// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the SECOND axis of a routed answer:
/// what role the value plays at the eliminator that receives it.**
///
/// Architect `evt_43ng4f578mdvv`. `SourceComputationalAnswerRoute` says how
/// checked the value is; this says whether the eliminator should ELIMINATE it or
/// has already been discharged by whoever produced it. The two are independent
/// and neither is derivable from the other, which is why this is a second field
/// rather than a third route variant.
///
/// ⛔ **This is not a phase and never converts one.** A `Specialized` operand
/// stays specialized and a `Carried` one stays carried across the whole
/// disposition; what changes is which frame consumes it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EliminatorRole {
    /// The ordinary role: the receiving eliminator eliminates this value.
    /// **Every value and every call result is this**, and the exhaustive match
    /// at the shared consumer has no wildcard so a future third role must be a
    /// compile error there rather than silently taking this arm.
    Scrutinee,
    /// The value is the ANSWER of the computational frame at
    /// `continuation_origin`, which has therefore already been discharged by
    /// the producer that issued this.
    ///
    /// ⛔ **Issuable by ONE authority only** — an already-selected `Inner`
    /// `FusionComposedEdge`, carrying that edge's planner-authored consumer
    /// continuation. Nothing else may mint it, and it is never inferred from
    /// the operand's shape.
    AnswerAfterComputationalFrame { continuation_origin: StaticOriginId },
}

#[derive(Clone)]
struct RoutedAnswer {
    value: LoweringOperand,
    route: SourceComputationalAnswerRoute,
    /// `D3` — the eliminator-role axis. `Scrutinee` on every constructor except
    /// the one Inner composition seat.
    role: EliminatorRole,
}

impl RoutedAnswer {
    /// Ordinary source evaluation **starts** here.
    fn direct(value: LoweringOperand) -> Self {
        Self {
            value,
            route: SourceComputationalAnswerRoute::DirectScrutinee,
            role: EliminatorRole::Scrutinee,
        }
    }

    /// An exact producer **raises** it.
    ///
    /// ⛔ There are exactly two callers of this and there must not be a third
    /// without a ruling: the exact selecting recursor-layer path, and the
    /// result of an actually claimed and emitted continuation-specialization
    /// call. A static-worker call, a raw unit call, an ordinary expression
    /// result, or a merely matching continuation origin is **not** an exact
    /// producer.
    fn checked(value: LoweringOperand) -> Self {
        Self {
            value,
            route: SourceComputationalAnswerRoute::CheckedSelectedRecursor,
            role: EliminatorRole::Scrutinee,
        }
    }

    /// **`D3` — the Inner composition's answer, and the ONLY producer of the
    /// non-`Scrutinee` role.** The continuation origin is the composed edge's
    /// planner-authored consumer continuation, supplied by the seat that
    /// already selected that edge; this constructor derives nothing.
    fn composed_answer(value: LoweringOperand, continuation_origin: StaticOriginId) -> Self {
        Self {
            value,
            route: SourceComputationalAnswerRoute::CheckedSelectedRecursor,
            role: EliminatorRole::AnswerAfterComputationalFrame { continuation_origin },
        }
    }

    /// Raise a frame's starting route with this predecessor's, never lower it.
    ///
    /// ⛔ The existing `SourceContinuation::ComputationalMatchScrutinee` field
    /// stays the recursor-layer producer's authority, but it is **not the sole
    /// authority** and must not overwrite a checked route carried in by an
    /// exact call result. That asymmetry is why this is a join and not an
    /// assignment.
    fn raise(self, existing: SourceComputationalAnswerRoute) -> SourceComputationalAnswerRoute {
        match (self.route, existing) {
            (SourceComputationalAnswerRoute::CheckedSelectedRecursor, _)
            | (_, SourceComputationalAnswerRoute::CheckedSelectedRecursor) => {
                SourceComputationalAnswerRoute::CheckedSelectedRecursor
            }
            _ => SourceComputationalAnswerRoute::DirectScrutinee,
        }
    }
}
impl SourceComputationalAnswerRoute {
    /// **`D6a` upstream — PRODUCER 1.** An exact recursor layer, and only the
    /// `SelectsOccurrence` role on a checked frame, supplies the checked route.
    fn for_recursor_layer(layer: &ComputationalRecursorLayer) -> Self {
        let selects_occurrence = matches!(layer.role, RecursorLayerRole::SelectsOccurrence { .. });
        let route = if layer.checked_frame_id.is_some() && selects_occurrence {
            Self::CheckedSelectedRecursor
        } else {
            Self::DirectScrutinee
        };
        #[cfg(test)]
        let route = if d6a_route_mutation() == D6aRouteMutation::DropRecursorLayerRoute
            && route == Self::CheckedSelectedRecursor
        {
            record_d6a_route_application();
            Self::DirectScrutinee
        } else {
            route
        };
        #[cfg(test)]
        record_d6a_route_event(D6aRouteEvent::RecursorLayerSupplied {
            checked_frame_id: layer.checked_frame_id,
            selects_occurrence,
            route,
        });
        route
    }
}
fn source_case_has_no_checked_control_markers(expr: &RuntimeExpr) -> bool {
    let mut frames = BTreeMap::new();
    if collect_checked_subcontinuation_frames(expr, &mut frames).is_err() || !frames.is_empty() {
        return false;
    }
    let mut markers = CheckedOrientedMarkerSets::default();
    collect_checked_oriented_markers(expr, &mut markers, "<source-case>", &mut Vec::new()).is_ok()
        && markers.recursive_calls.is_empty()
        && markers.computational_ih_slots.is_empty()
        && markers.computational_ih_calls.is_empty()
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8e` — what a source-machine call is calling.**
///
/// ⛔ A sum rather than an operand, because a static worker **is not a value**.
/// The capsule has no value representation, so it cannot be carried in the
/// operand slot the value route uses, and widening that slot to hold it would
/// undo exactly the fail-closed property `D8d` installed it for.
///
/// ⭐ The two arms differ only in where the argument run goes once it is
/// evaluated. Argument evaluation itself is identical and happens under the
/// machine's own control and phase for both, which is why the split is at the
/// completion and not at the entry.
#[derive(Clone)]
enum SourceCallee {
    /// The pre-existing route: a lowered callee consumed by
    /// `source_call_state`.
    Value(LoweringOperand),
    /// **`D8e`** — an exact `Var` that resolved to a `D8d` target-derived
    /// binding. ⛔ Resolved once, at the `Call` occurrence, before the callee
    /// would otherwise have been evaluated as a value; there is no second
    /// lookup and no planner query on this path.
    StaticWorker {
        worker: StaticWorkerBinding,
        static_origin: StaticOriginId,
        /// **`RT-CONTSRC-PRODUCER-LOCAL` `D8p`** — the exact binder index the
        /// callee `Var` resolved at.
        ///
        /// ⛔ Carried from the occurrence that resolved it, never re-derived at
        /// the emission seat. The checked-IH seam holds the emitted call to the
        /// binder ordinal the plan seated the hypothesis at, and a second
        /// derivation here could disagree with the one that actually chose the
        /// binding -- which is the shape `D8f` already rules out for the
        /// application origin.
        binder_index: u64,
    },
}
/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8f` — what a static-worker call edge found at
/// the checked-application seam.**
///
/// ⛔ **Closed, and deliberately three cases rather than a Boolean.** "No
/// checked application is pending" and "one is pending, at another occurrence"
/// are different facts with different consequences for the composed causal
/// claim, and a `bool` spelled them the same. The exhaustive match at the
/// integration boundary is what makes a fourth case a compile error instead of
/// a silent fall-through into one of these.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum CheckedApplicationDisposition {
    /// No checked application is pending at this edge. The call is an ordinary
    /// one and the seam did not touch it.
    ///
    /// ⭐ A composed binding claims its causal identity here exactly as it did
    /// before `D8f` -- this is the `D8j` population, and it is the larger one.
    NoPendingApplication,
    /// A checked application IS pending, and this call is **not** it.
    ///
    /// The marker stays pending for the occurrence that owns it, the call is
    /// emitted unchanged -- and its composed causal identity is **not** claimed.
    /// That last part is the whole of this variant: the identity belongs to the
    /// checked application the planner issued it for, and letting an ordinary
    /// selected-argument call answer for it is a second discharge of one
    /// obligation.
    PendingAtAnotherOccurrence,
    /// This call **is** the pending checked application, and the marker has been
    /// consumed for it. It claims that same planner-issued identity, once.
    ConsumedHere,
}
#[derive(Clone)]
struct SourceJoinTarget<'a> {
    join_id: u64,
    block: cranelift_codegen::ir::Block,
    expected_outer: ContinuationCursorId,
    required_kind: ScalarMergeKind,
    join_plan: std::rc::Rc<JoinPlanToken>,
    result_origin: StaticOriginId,
    terminal_active_prefix: Vec<EliminatorFrame<'a>>,
}
/// An affine capability for one mutually exclusive predecessor of a checked
/// source join. The target description is shareable; this edge deliberately is
/// not `Clone`, so a predecessor can either seal its edge or consume it into a
/// branch fan-out, never replay it.
struct SourcePredecessorEdge<'a> {
    target: SourceJoinTarget<'a>,
    predecessor_identity: u64,
}
/// A cloneable source-evaluation prefix with its terminal edge removed. A
/// branch fan-out may materialize this prefix once per mutually exclusive CFG
/// arm, but the post-cut suffix and executable predecessor edge never live in
/// the template.
#[derive(Clone)]
enum SourcePrefixTemplate {
    Terminal {
        expected_outer: ContinuationCursorId,
    },
    CheckedRecursiveInvocationReturn {
        instance: CheckedRecursiveInvocationInstance,
        next: Box<SourcePrefixTemplate>,
    },
    CheckedComputationalIHInvocationReturn {
        call_template_id: u64,
        next: Box<SourcePrefixTemplate>,
    },
    ReturnFromSelectedCase {
        delimiter: SelectedCaseReturnDelimiter,
        next: Box<SourcePrefixTemplate>,
    },
    LetBody {
        body: OwnedSourceOccurrence,
        env: Vec<LoweringEnvironmentBinding>,
        next: Box<SourcePrefixTemplate>,
    },
    ApplyRecursorSelection {
        layer: ComputationalRecursorLayer,
        next: Box<SourcePrefixTemplate>,
    },
    UnwindRecursorSegment {
        stack: RecursorUnwindStack,
        resume_cursor: ContinuationCursorId,
        next: Box<SourcePrefixTemplate>,
    },
    IfScrutinee {
        then_expr: OwnedSourceOccurrence,
        else_expr: OwnedSourceOccurrence,
        env: Vec<LoweringEnvironmentBinding>,
        next: Box<SourcePrefixTemplate>,
    },
    ConstructArgument {
        constructor: RuntimeSymbol,
        static_origin: StaticOriginId,
        remaining: Vec<OwnedSourceOccurrence>,
        lowered: Vec<Lowered>,
        env: Vec<LoweringEnvironmentBinding>,
        next: Box<SourcePrefixTemplate>,
    },
    MatchScrutinee {
        cases: Vec<crate::RuntimeMatchCase>,
        default: RuntimeTrap,
        env: Vec<LoweringEnvironmentBinding>,
        static_origin: StaticOriginId,
        next: Box<SourcePrefixTemplate>,
    },
    ComputationalMatchScrutinee {
        cases: Vec<crate::RuntimeComputationalMatchCase>,
        default: RuntimeTrap,
        env: Vec<LoweringEnvironmentBinding>,
        static_origin: StaticOriginId,
        provenance: RecursorFrameProvenance,
        checked_frame_id: Option<u64>,
        answer_route: SourceComputationalAnswerRoute,
        next: Box<SourcePrefixTemplate>,
    },
    ProjectRecord {
        field: String,
        next: Box<SourcePrefixTemplate>,
    },
    CallCallee {
        args: Vec<OwnedSourceOccurrence>,
        env: Vec<LoweringEnvironmentBinding>,
        next: Box<SourcePrefixTemplate>,
    },
    CallArgument {
        callee: SourceCallee,
        remaining: Vec<OwnedSourceOccurrence>,
        lowered: Vec<LoweringOperand>,
        env: Vec<LoweringEnvironmentBinding>,
        next: Box<SourcePrefixTemplate>,
    },
}
enum SourcePrefixTerminal<'a> {
    ResumeOuter {
        root_authority: Option<RootTerminalAnswerAuthority>,
    },
    Join(SourcePredecessorEdge<'a>),
}
struct SourceBranchFanout<'a> {
    source_prefix_template: SourcePrefixTemplate,
    inherited_edge: SourcePredecessorEdge<'a>,
}
struct ArmedInvocation<'a> {
    suspended: SourceControl<'a>,
    expected_selected: ContinuationCursorId,
}
#[derive(Clone)]
struct SourceSelectedContinuation<'a> {
    activation: ContinuationActivationId,
    cursor: ContinuationCursorId,
    parent: Option<&'a ActiveContinuationFrame<'a>>,
    pending: Vec<EliminatorFrame<'a>>,
    selected_ancestry: Vec<RecursorFrameProvenance>,
    selected_scope: Option<OwnedSelectedScope>,
}
impl<'a> SourceSelectedContinuation<'a> {
    fn as_active<'b>(
        &'b self,
        source_lineage: &'b [SourceSelectedContinuation<'a>],
    ) -> ActiveContinuationFrame<'b>
    where
        'a: 'b,
    {
        ActiveContinuationFrame {
            activation: self.activation,
            cursor: self.cursor,
            parent: self.parent,
            pending: &self.pending,
            selected_ancestry: &self.selected_ancestry,
            source_lineage,
            source_selected_cursor: Some(self.cursor),
            selected_scope: self.selected_scope.as_ref(),
        }
    }
}
fn source_active_cursor<'a: 'b, 'b>(
    selected: &'b SourceSelectedContinuation<'a>,
    lineage: &'b [SourceSelectedContinuation<'a>],
    cursor: ContinuationCursorId,
) -> Option<ActiveContinuationFrame<'b>> {
    std::iter::once(selected)
        .chain(lineage.iter().rev())
        .find_map(|candidate| {
            let mut active = candidate.as_active(lineage);
            active.source_selected_cursor = Some(selected.cursor);
            if active.cursor == cursor {
                Some(active)
            } else {
                let mut parent = active.parent;
                while let Some(frame) = parent {
                    if frame.cursor == cursor {
                        let mut frame = *frame;
                        frame.source_lineage = lineage;
                        frame.source_selected_cursor = Some(selected.cursor);
                        return Some(frame);
                    }
                    parent = frame.parent;
                }
                None
            }
        })
}
struct SourceControl<'a> {
    continuation: SourceContinuation<'a>,
    selected: SourceSelectedContinuation<'a>,
    selected_lineage: Vec<SourceSelectedContinuation<'a>>,
    terminal_outer: ContinuationCursorId,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SelectedCaseReturnDelimiter {
    activation: ContinuationActivationId,
    cursor: ContinuationCursorId,
    scope_origin: RecursorProducerOriginId,
    frame_id: Option<u64>,
    invocation_id: Option<u64>,
}
#[derive(Clone, Copy)]
enum DynamicConstructorContinuation<'a> {
    Ordinary {
        cases: &'a [crate::RuntimeMatchCase],
        default: &'a RuntimeTrap,
        env: &'a [LoweringEnvironmentBinding],
        static_origin: StaticOriginId,
    },
    Producer {
        cases: &'a [crate::RuntimeMatchCase],
        default: &'a RuntimeTrap,
        env: &'a [LoweringEnvironmentBinding],
        static_origin: StaticOriginId,
        eliminators: &'a [EliminatorFrame<'a>],
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ScalarMergeKind {
    Int,
    Bool,
    StructuralNat,
    ExitCode,
    RecursiveBackedge,
}


/// Proof token for the legacy closed-expression merge sites. It can only be
/// minted when source evaluation has no live continuation. Checked source joins
/// use their explicit `SourceJoinTarget.required_kind` instead.
/// Move-only proof that the native lowering machine has reached the checked
/// invocation root with no semantic or control continuation left to consume
/// the value.
struct RootTerminalAnswerAuthority {
    site_id: u64,
    checked_result_type_fingerprint: u64,
    occurrence_binding_fingerprint: u64,
    outer_cursor: Option<ContinuationCursorId>,
}
struct TerminalAnswerAuthority;
struct DeferredConstructorCaseEnvironment<'a> {
    constructor: &'a str,
    lowered_prefix: &'a [Lowered],
    selected_field: usize,
    trailing_fields: &'a [RuntimeExpr],
    /// The origin of the `Construct` occurrence the fields belong to. Field *i*
    /// of that constructor is its child *i*, so `trailing_fields[j]` is
    /// `child(construct_origin, selected_field + 1 + j)`.
    construct_origin: StaticOriginId,
    producer_env: &'a [LoweringEnvironmentBinding],
    outer_eliminator: EliminatorFrame<'a>,
    splice_caller: Option<&'a ActiveContinuationFrame<'a>>,
    selected_active: ActiveContinuationFrame<'a>,
}
/// **`D8m`** — the four checked facts a source `ComputationalMatch` frame
/// carries, kept together so no site can supply three of them.
#[derive(Clone, Copy)]
struct CheckedComputationalFrame {
    id: Option<u64>,
    invocation_id: Option<u64>,
    invocation_source: Option<InvocationTemplateRef>,
    invocation_depth: usize,
}

#[derive(Clone, Copy)]
/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8m` — the closed bridge descriptor.**
///
/// Exactly three admissible shapes for the case body an
/// `immediate_binder_eliminator` bridge is built from, and no fourth:
///
/// 1. a direct [`RuntimeExpr::ComputationalMatch`];
/// 2. a direct ordinary [`RuntimeExpr::Match`];
/// 3. exactly `CheckedSubcontinuationFrame { frame_id, body: ComputationalMatch }`.
///
/// ⭐⭐ **The third exists because the bridge is an OPTIMIZATION of the source
/// match, not a new semantic frame.** The source declared one checked frame
/// there; deforesting the producer into it does not create a second frame and
/// must not lose the first. Before `D8m` the bridge always carried
/// `checked_frame_id: None`, so a checked IH slot inside a composed case body
/// refused as "detached from its checked frame" — the `D8f` hard stop.
///
/// ⛔ **Nothing here mints, borrows or infers a frame identity.** The id is the
/// one the source marker carries and is reached only by matching that exact
/// shape. There is deliberately no fingerprint lookup, no body-shape match, no
/// origin coincidence, no "the only frame in the plan", and no generic wrapper
/// peeling: a marker around anything but a `ComputationalMatch`, or any other
/// checked wrapper kind, simply is not a bridge.
enum ImmediateBinderEliminator<'a> {
    Computational {
        cases: &'a [crate::RuntimeComputationalMatchCase],
        default: &'a RuntimeTrap,
    },
    Ordinary {
        cases: &'a [crate::RuntimeMatchCase],
        default: &'a RuntimeTrap,
    },
    /// The wrapped form. ⛔ The match's own occurrence is **child 0 of the
    /// marker occurrence**, never the wrapper's origin — the wrapper is not the
    /// frame, it names it.
    CheckedComputational {
        frame_id: u64,
        cases: &'a [crate::RuntimeComputationalMatchCase],
        default: &'a RuntimeTrap,
    },
}
fn immediate_binder_eliminator(
    body: &RuntimeExpr,
    argument_binder_offset: usize,
    argument_binders: usize,
) -> Option<(usize, ImmediateBinderEliminator<'_>)> {
    let (scrutinee, eliminator) = match body {
        RuntimeExpr::ComputationalMatch {
            scrutinee,
            cases,
            default,
        } => (
            scrutinee.as_ref(),
            ImmediateBinderEliminator::Computational { cases, default },
        ),
        // `D8m` — the EXACT wrapped shape, and only it. ⛔ Not a loop, not a
        // helper that strips any checked wrapper: a `CheckedRecursiveInvocation`
        // or a `CheckedJoinSite` around a match is a different construct with a
        // different consumption law, and peeling it here would silently give the
        // bridge an identity nobody transported for it.
        RuntimeExpr::CheckedSubcontinuationFrame { frame_id, body } => {
            let RuntimeExpr::ComputationalMatch {
                scrutinee,
                cases,
                default,
            } = body.as_ref()
            else {
                return None;
            };
            (
                scrutinee.as_ref(),
                ImmediateBinderEliminator::CheckedComputational {
                    frame_id: *frame_id,
                    cases,
                    default,
                },
            )
        }
        RuntimeExpr::Match {
            scrutinee,
            cases,
            default,
        } => (
            scrutinee.as_ref(),
            ImmediateBinderEliminator::Ordinary { cases, default },
        ),
        _ => return None,
    };
    let RuntimeExpr::Var(index) = scrutinee else {
        return None;
    };
    let index = usize::try_from(*index).ok()?;
    let field = index.checked_sub(argument_binder_offset)?;
    (field < argument_binders).then_some((field, eliminator))
}
fn requires_heterogeneous_deforestation(expr: &RuntimeExpr) -> bool {
    matches!(
        expr,
        RuntimeExpr::Match { .. }
            | RuntimeExpr::ComputationalMatch { .. }
            | RuntimeExpr::If { .. }
            | RuntimeExpr::Call { .. }
    ) && produces_deforestable_aggregate_with_ih(expr, &BTreeSet::new())
}
fn reaches_environment_computational_recursor(
    expr: &RuntimeExpr,
    env: &[LoweringEnvironmentBinding],
    introduced_binders: usize,
) -> bool {
    let recursive_hypotheses = env
        .iter()
        .enumerate()
        .filter_map(|(index, binding)| {
            // Exhaustive with no wildcard. A static worker is a lexical
            // callable, never a computational recursor closure, so it is not a
            // recursive hypothesis -- this is a classification, not a
            // value-producing read, so it answers rather than fails closed.
            let is_recursor = match binding {
                LoweringEnvironmentBinding::Value(LoweringOperand::Specialized(
                    Lowered::ComputationalRecursorClosure { .. },
                )) => true,
                LoweringEnvironmentBinding::Value(_) => false,
                LoweringEnvironmentBinding::StaticWorker(_) => false,
            };
            is_recursor.then_some(index + introduced_binders)
        })
        .collect();
    produces_deforestable_aggregate_with_ih(expr, &recursive_hypotheses)
        && !produces_deforestable_aggregate_with_ih(expr, &BTreeSet::new())
}
fn shifted_aggregate_ihs(aggregate_ihs: &BTreeSet<usize>, by: usize) -> BTreeSet<usize> {
    aggregate_ihs.iter().map(|index| index + by).collect()
}
fn produces_deforestable_aggregate_with_ih(
    expr: &RuntimeExpr,
    aggregate_ihs: &BTreeSet<usize>,
) -> bool {
    match expr {
        RuntimeExpr::CheckedJoinSite { body, .. } => {
            produces_deforestable_aggregate_with_ih(body, aggregate_ihs)
        }
        RuntimeExpr::Construct { .. } => true,
        RuntimeExpr::Let { body, .. } => {
            produces_deforestable_aggregate_with_ih(body, &shifted_aggregate_ihs(aggregate_ihs, 1))
        }
        RuntimeExpr::Match { cases, .. } => {
            !cases.is_empty()
                && cases.iter().all(|case| {
                    produces_deforestable_aggregate_with_ih(
                        &case.body,
                        &shifted_aggregate_ihs(aggregate_ihs, case.binders),
                    )
                })
        }
        RuntimeExpr::ComputationalMatch { cases, .. } => {
            !cases.is_empty()
                && cases.iter().all(|case| {
                    let mut case_ihs = (0..case.recursive_positions.len()).collect::<BTreeSet<_>>();
                    case_ihs.extend(aggregate_ihs.iter().map(|index| {
                        index + case.recursive_positions.len() + case.argument_binders
                    }));
                    produces_deforestable_aggregate_with_ih(&case.body, &case_ihs)
                })
        }
        RuntimeExpr::If {
            then_expr,
            else_expr,
            ..
        } => {
            produces_deforestable_aggregate_with_ih(then_expr, aggregate_ihs)
                && produces_deforestable_aggregate_with_ih(else_expr, aggregate_ihs)
        }
        RuntimeExpr::Call { callee, .. } => {
            if let RuntimeExpr::Var(index) = callee.as_ref() {
                return usize::try_from(*index).is_ok_and(|index| aggregate_ihs.contains(&index));
            }
            match callee.as_ref() {
                RuntimeExpr::Closure {
                    captures,
                    params,
                    body,
                } => produces_deforestable_aggregate_with_ih(
                    body,
                    &shifted_aggregate_ihs(aggregate_ihs, params.len() + captures.len()),
                ),
                RuntimeExpr::LexicalClosure {
                    captures,
                    params,
                    body,
                } => produces_deforestable_aggregate_with_ih(
                    body,
                    &shifted_aggregate_ihs(aggregate_ihs, params.len() + captures.len()),
                ),
                _ => false,
            }
        }
        _ => false,
    }
}
fn produces_recursive_deforestable_aggregate(expr: &RuntimeExpr, symbol: &str) -> bool {
    match expr {
        RuntimeExpr::Construct { .. } => true,
        RuntimeExpr::Let { body, .. } => produces_recursive_deforestable_aggregate(body, symbol),
        RuntimeExpr::Match { cases, .. } => {
            !cases.is_empty()
                && cases
                    .iter()
                    .all(|case| produces_recursive_deforestable_aggregate(&case.body, symbol))
        }
        RuntimeExpr::ComputationalMatch { cases, .. } => {
            !cases.is_empty()
                && cases
                    .iter()
                    .all(|case| produces_recursive_deforestable_aggregate(&case.body, symbol))
        }
        RuntimeExpr::If {
            then_expr,
            else_expr,
            ..
        } => {
            produces_recursive_deforestable_aggregate(then_expr, symbol)
                && produces_recursive_deforestable_aggregate(else_expr, symbol)
        }
        RuntimeExpr::Call { callee, .. } => {
            matches!(callee.as_ref(), RuntimeExpr::DeclarationRef { symbol: callee } if callee == symbol)
        }
        _ => false,
    }
}
/// Selects an ordinary case by constructor, **with its index**.
///
/// The index is what makes the selected body's origin derivable: the search
/// itself recovers no position.
fn select_ordinary_case<'a>(
    eliminator: OrdinaryEliminatorFrame<'a>,
    constructor: &str,
) -> Result<(usize, &'a crate::RuntimeMatchCase), RuntimeTrap> {
    eliminator
        .cases
        .iter()
        .enumerate()
        .find(|(_, case)| case.constructor == constructor)
        .ok_or_else(|| eliminator.default.clone())
}
/// Selects a computational case by constructor, **with its index**, plus the
/// remaining frames.
///
/// The index is load-bearing: the selected body's origin is `child(the frame's
/// `static_origin`, 1 + index)`, and the search alone recovers no position.
fn select_computational_case<'frames, 'data>(
    eliminators: &'frames [ComputationalEliminatorFrame<'data>],
    constructor: &str,
) -> Result<
    (
        usize,
        &'data crate::RuntimeComputationalMatchCase,
        &'frames [ComputationalEliminatorFrame<'data>],
    ),
    RuntimeTrap,
> {
    let Some(eliminator) = eliminators.first() else {
        return Err(RuntimeTrap {
            code: RuntimeTrapCode::UnsupportedErasure,
            message: "nested computational producer has no eliminator".to_string(),
        });
    };
    eliminator
        .cases
        .iter()
        .enumerate()
        .find(|(_, case)| case.constructor == constructor)
        .map(|(index, case)| (index, case, &eliminators[1..]))
        .ok_or_else(|| eliminator.default.clone())
}
impl<'a> Lowering<'a> {
    fn mint_recursor_producer_origin(&mut self) -> RecursorProducerOriginId {
        let origin = RecursorProducerOriginId(self.next_recursor_producer_origin);
        self.next_recursor_producer_origin = self
            .next_recursor_producer_origin
            .checked_add(1)
            .expect("compiler-private recursor producer origin exhausted");
        origin
    }

    fn mint_recursor_frame_provenance(&mut self) -> RecursorFrameProvenance {
        let provenance = RecursorFrameProvenance(self.next_recursor_frame_provenance);
        self.next_recursor_frame_provenance = self
            .next_recursor_frame_provenance
            .checked_add(1)
            .expect("compiler-private recursor provenance exhausted");
        provenance
    }

    fn mint_continuation_activation(&mut self) -> ContinuationActivationId {
        let activation = ContinuationActivationId(self.next_continuation_activation);
        self.next_continuation_activation = self
            .next_continuation_activation
            .checked_add(1)
            .expect("compiler-private continuation activation exhausted");
        activation
    }

    fn mint_continuation_cursor(&mut self) -> ContinuationCursorId {
        let cursor = ContinuationCursorId(self.next_continuation_cursor);
        self.next_continuation_cursor = self
            .next_continuation_cursor
            .checked_add(1)
            .expect("compiler-private continuation cursor exhausted");
        cursor
    }

    fn enter_checked_subcontinuation_frame(
        &mut self,
        frame_id: u64,
    ) -> Result<(), CraneliftBackendError> {
        if self
            .active_subcontinuation_frame
            .replace(frame_id)
            .is_some()
        {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "nested checked subcontinuation occurrence marker",
            ));
        }
        Ok(())
    }

    fn enter_checked_recursive_invocation(
        &mut self,
        call_template_id: u64,
        body: &RuntimeExpr,
    ) -> Result<CheckedRecursiveInvocationInstance, CraneliftBackendError> {
        if self.pending_recursive_call.is_some() {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "nested unchecked recursive invocation marker",
            ));
        }
        let call = self
            .oriented_subcontinuation_plan
            .as_ref()
            .and_then(|plan| plan.recursive_call(call_template_id))
            .ok_or_else(|| {
                unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "recursive invocation marker has no checked call template",
                )
            })?;
        let RuntimeExpr::Call { callee, args } = body else {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "recursive invocation marker does not wrap one complete call",
            ));
        };
        if !matches!(callee.as_ref(), RuntimeExpr::DeclarationRef { symbol } if symbol == &call.callee)
            || args.len() as u64 != call.arity
        {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "recursive invocation marker callee or arity is stale",
            ));
        }
        if !self
            .consumed_recursive_call_templates
            .insert(call_template_id)
        {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "recursive invocation occurrence marker was consumed twice",
            ));
        }
        let instance = CheckedRecursiveInvocationInstance {
            source: InvocationTemplateRef::SameSccCall(call_template_id),
            invocation_instance_id: self.next_recursive_invocation_instance,
            semantic_depth: self.active_recursive_invocations.len() + 1,
            dynamic_splice_edge: None,
        };
        self.next_recursive_invocation_instance = self
            .next_recursive_invocation_instance
            .checked_add(1)
            .expect("compiler-private recursive invocation identity exhausted");
        self.pending_recursive_call = Some(instance);
        self.active_recursive_invocations.push(instance);
        Ok(instance)
    }

    fn leave_checked_recursive_invocation(
        &mut self,
        instance: CheckedRecursiveInvocationInstance,
    ) -> Result<(), CraneliftBackendError> {
        if self.pending_recursive_call == Some(instance) {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "recursive invocation marker was not consumed by its call",
            ));
        }
        if self.active_recursive_invocations.pop() != Some(instance) {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "recursive invocation instance stack is not affine",
            ));
        }
        Ok(())
    }

    /// **`RT-DECL-CLOSURE-PORT` `D5a` — the marker denotes the COMPLETE
    /// APPLICATION OCCURRENCE.**
    ///
    /// ⭐ That is what `CheckedComputationalIHCallTemplateV1::arity` is *about*:
    /// a call template describes one complete application, so the marker's
    /// wrapped expression must be exactly one `RuntimeExpr::Call` of that
    /// arity. Entry required only that a template exist, which admitted a marker
    /// wrapping anything at all -- and then the only place that could notice was
    /// closeout, on the produced *value*, long after emission.
    ///
    /// ⛔ Checked at ENTRY, so a marker that does not denote an application is
    /// refused before a single instruction is emitted for it.
    fn enter_checked_computational_ih_invocation(
        &mut self,
        call_template_id: u64,
        kind: crate::CheckedComputationalIHInvocationKind,
        binder_morphism: crate::CheckedComputationalIHBinderMorphism,
        body: &RuntimeExpr,
        // `D8f` — the occurrence of the application this marker denotes,
        // supplied by the caller from the same `child_origin(marker, 0)` it
        // already derives to lower the body. ⛔ Not recomputed here: a second
        // derivation could disagree with the one the lowering actually uses,
        // and then the marker would be pending against an occurrence nobody
        // visits.
        application_origin: StaticOriginId,
    ) -> Result<(), CraneliftBackendError> {
        if self
            .pending_computational_ih_call
            .replace(PendingCheckedIhCall {
                call_template_id,
                kind,
                binder_morphism,
                application_origin,
            })
            .is_some()
        {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "nested computational IH invocation marker",
            ));
        }
        let plan = self.oriented_subcontinuation_plan.as_ref().ok_or_else(|| {
            unsupported(
                "OrientedSubcontinuationPlanV1",
                "computational IH invocation marker has no checked plan",
            )
        })?;
        let call = plan
            .computational_ih_call(call_template_id)
            .ok_or_else(|| {
                unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "computational IH invocation marker has no checked call template",
                )
            })?;
        let RuntimeExpr::Call { args, .. } = body else {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "computational IH invocation marker does not wrap a complete application",
            ));
        };
        let supplied = u64::try_from(args.len()).map_err(|_| {
            unsupported(
                "OrientedSubcontinuationPlanV1",
                "computational IH invocation argument count exceeds addressable range",
            )
        })?;
        #[cfg(test)]
        let relaxed =
            D5A_MARKER_MUTATION.with(std::cell::Cell::get) == D5aMarkerMutation::RelaxEntryArity;
        #[cfg(not(test))]
        let relaxed = false;
        if !relaxed && supplied != call.arity {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                format!(
                    "computational IH invocation marker wraps a call of {supplied} arguments but \
                     its checked template names arity {}",
                    call.arity
                ),
            ));
        }
        Ok(())
    }

    /// **`RT-DECL-CLOSURE-PORT` `D5a` — consume the checked-IH marker at the
    /// exact static-worker call edge.**
    ///
    /// ⭐⭐ **The marker denotes the application, not the applied value.** On the
    /// ported route the induction hypothesis *is* an emitted call, so its result
    /// is a boundary word and no compile-time recursor-closure template exists
    /// for closeout to read. Consuming here -- **before emission** -- is what
    /// makes the marker mean the same thing on both routes; consuming on the
    /// returned word would be a carrier decode, which is exactly what `§2h`
    /// forbids.
    ///
    /// Three independent identities must agree, and all three are read from the
    /// checked plan rather than from the operand:
    ///
    /// | requirement | what a mismatch would mean |
    /// |---|---|
    /// | the pending template resolves, and its slot template resolves | the marker names a plan this lowering does not hold |
    /// | `call.arity == args.len()` | the emitted call is not the application the template describes |
    /// | `slot.method_binder_ordinal == Var index` | the callee is not the binder the checked plan seated the IH at |
    ///
    /// ⭐ The third is an **independent oracle for the binder run**. The slot
    /// template carries `recursive_position` and `method_binder_ordinal` as
    /// *separate* fields -- the constructor source coordinate and the lexical
    /// one -- so a lowering that conflated them (as this checkpoint's
    /// predecessor did) disagrees with the plan here and refuses.
    ///
    /// ⛔ Returns a closed [`CheckedApplicationDisposition`]. `D8f`'s three
    /// cases are not interchangeable: "nothing pending" and "pending, but not
    /// here" both leave the call unchanged and differ in whether its composed
    /// causal identity may be claimed.
    fn consume_checked_ih_marker_at_static_worker_call(
        &mut self,
        binder_index: u64,
        supplied_arguments: usize,
        // `D8f` — the occurrence of the call being lowered.
        static_origin: StaticOriginId,
    ) -> Result<CheckedApplicationDisposition, CraneliftBackendError> {
        #[cfg(test)]
        if D5A_MARKER_MUTATION.with(std::cell::Cell::get) == D5aMarkerMutation::SuppressConsumption
        {
            // ⛔ The call below is still emitted, lawfully and unchanged; only
            // the consumption is withheld. That is the whole point — closeout
            // must notice a real application that no consumption accounts for.
            //
            // ⚠ Reported as PENDING-AT-ANOTHER-OCCURRENCE, not as
            // nothing-pending: a marker genuinely is pending here, and the
            // mutation withholds this call's claim on it. Reporting the wrong
            // case would let the mutation quietly change the causal claim too,
            // and then the row it feeds would be measuring two changes at once.
            return Ok(CheckedApplicationDisposition::PendingAtAnotherOccurrence);
        }
        let Some(pending) = self.pending_computational_ih_call else {
            return Ok(CheckedApplicationDisposition::NoPendingApplication);
        };
        // ⭐⭐ `D8f` — OCCUPANCY. A pending marker does not mean "the next
        // static-worker call consumes it".
        //
        // Inside one checked wrapper the arguments of the wrapped application
        // are evaluated before the application itself, and an argument can be
        // an ordinary call on the selected recursive argument. That call
        // reaches this seat with a marker pending and must leave it pending:
        // it is not the occurrence the plan issued the marker for.
        //
        // ⛔ A disposition, not a refusal, and the difference is the checkpoint.
        // An ordinary call is *untouched* by this seam -- exactly as it is when
        // no marker is pending at all -- so the marker survives for the
        // occurrence that owns it. Refusing here would make a lawful program
        // fail; consuming here would attribute the checked application to a
        // call the planner never issued a template for.
        //
        // ⛔ The comparison is on the OCCURRENCE and on nothing else. Route,
        // arity, binder index, first-call order and callee shape are all
        // properties an ordinary selected-argument call can share with the
        // checked one -- they are the same worker at the same arity in the same
        // frame -- so every one of them is blind here by construction. The
        // arity and binder-ordinal agreements below stay, but as checks on the
        // call that has already been identified, never as the identification.
        // ⭐⭐ **WITNESSED SINCE `D8p`.** Mutating this comparison to never
        // admit reds `D5a` rows and two `D8p` rows. Mutating it to ALWAYS admit
        // -- the pre-`D8f` behaviour -- now reds
        // `d8p_binding_is_zero_one_or_declined_per_application_occurrence`,
        // whose witness puts two static-worker calls under one pending marker:
        // the same worker, the same arity, the same frame, so route, arity,
        // binder index and call order are all blind and the occurrence is the
        // only thing that decides. The first call emits with no consumption; the
        // second consumes at the plan's exact occurrence.
        //
        // ⚠ This paragraph previously read "UNWITNESSED, and measured to be so
        // -- mutating it to ALWAYS admit leaves the whole suite green". That was
        // true until `D8p` put the checked-application seam on the source
        // machine's call edge, which is what made a program with two candidate
        // calls reachable at all.
        //
        // ⭐ **ROUTE 2 IS NOW OPEN; `D8f`'s remaining question is elsewhere.**
        // The history below is retained because each entry says what a route
        // costs, and two of the three are still closed.
        //
        // Route 2's stated reason has been restated twice. It first read that a
        // composed bridge frame can never carry a checked identity, because
        // `immediate_binder_eliminator` synthesized it with
        // `checked_frame_id: None` -- `D8m` retired that. It then read that the
        // ordinary unit body has no static-worker seat at the application --
        // `D8p` retired that too, by putting this seam on the source machine's
        // call edge. The composed route now hosts a checked-IH invocation
        // marker, binds it, and emits the call: see the `D8p` rows.
        //
        // ⛔ `D8f` is still not discharged, and the reason has moved off this
        // seat. Its two-call witness now reaches the affine causal law -- "one
        // causal identity was discharged twice in a single function" -- because
        // the ordinary call re-uses a binding that carries a composed authority.
        // That is a question about the ordinary call's discharge, not about
        // occupancy, and this gate's answer is no longer what blocks it.
        //
        // Route 1 -- a NON-COMPOSED checked wrapper (`px8tr`), which hosts the
        // invocation marker green. Nesting an ordinary call on the same recursor
        // binder inside the marker's application refuses in plan validation with
        // "oriented segment mixes checked and inferred computational frames",
        // detail `(Some(7), None, ...)`. The ordinary call instantiates a
        // semantic IH layer, and a segment carrying any checked frame requires
        // every semantic layer to carry a checked invocation id. ⇒ **The call
        // that must leave the marker pending is exactly the call that would have
        // to have consumed one.**
        //
        // Route 2 -- a COMPOSED checked wrapper. OPEN since `D8p`. It refused at
        // `finish_checked_computational_ih_marker` with "a checked
        // computational-IH marker is a specialized-only surface", because the
        // ordinary unit body that lowers the case body reached the application
        // through the source machine, whose call edge did not consult this seam.
        // It does now, so the marker is consumed there and the route carries the
        // occupancy witness.
        //
        // Route 3 -- a COMPOSED static-worker call as the checked application's
        // argument, so the seat is reached without instantiating an IH layer.
        // Refuses with "source open occurrence disagrees with the
        // closure-selected dynamic parent".
        //
        // ⇒ The occupancy property is witnessed on route 2. Routes 1 and 3 stay
        // recorded because each names a law that a future witness would have to
        // satisfy rather than route around.
        if static_origin != pending.application_origin {
            return Ok(CheckedApplicationDisposition::PendingAtAnotherOccurrence);
        }
        let call_template_id = pending.call_template_id;
        let plan = self.oriented_subcontinuation_plan.as_ref().ok_or_else(|| {
            unsupported(
                "OrientedSubcontinuationPlanV1",
                "computational IH invocation has no checked plan",
            )
        })?;
        let call = plan
            .computational_ih_call(call_template_id)
            .ok_or_else(|| {
                unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "computational IH invocation has no checked call template",
                )
            })?;
        let slot = plan
            .computational_ih_slot(call.slot_template_id)
            .ok_or_else(|| {
                unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "computational IH call template names a slot template the plan does not hold",
                )
            })?;
        let supplied = u64::try_from(supplied_arguments).map_err(|_| {
            unsupported(
                "OrientedSubcontinuationPlanV1",
                "static worker call argument count exceeds addressable range",
            )
        })?;
        if supplied != call.arity {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                format!(
                    "the checked computational-IH call template names arity {} but the static \
                     worker call applies {supplied} arguments",
                    call.arity
                ),
            ));
        }
        let planned_runtime_index = pending
            .binder_morphism
            .runtime_index(slot.method_binder_ordinal)
            .ok_or_else(|| {
                unsupported(
                    "OrientedSubcontinuationPlanV1",
                    format!(
                        "the checked computational-IH slot's method ordinal {} is outside the \
                         invocation's source-to-runtime binder map {:#?}",
                        slot.method_binder_ordinal, pending.binder_morphism
                    ),
                )
            })?;
        if planned_runtime_index != binder_index {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                format!(
                    "the checked computational-IH slot maps method ordinal {} to runtime \
                     `Var({planned_runtime_index})` but the consuming call reads \
                     `Var({binder_index})`",
                    slot.method_binder_ordinal
                ),
            ));
        }
        // ⛔ Taken LAST. Every refusal above leaves the marker pending, so a
        // rejected consumption still reaches closeout's fail-closed arm rather
        // than silently becoming an unmarked call.
        self.pending_computational_ih_call = None;
        #[cfg(test)]
        record_d5a_marker_event(D5aMarkerEvent::Consumed {
            call_template_id,
            slot_template_id: call.slot_template_id,
            binder_index,
            arity: call.arity,
        });
        // `D8p` — the plan side of the binding, keyed on the pair the
        // checkpoint is about: the exact defining body and the exact
        // application occurrence. Written here because this is where the
        // binding happens, not reconstructed by a reader afterwards.
        #[cfg(test)]
        record_d8p_application_binding(D8pApplicationBinding {
            function: self.defining_function_id,
            application_origin: static_origin,
            call_template_id,
            slot_template_id: call.slot_template_id,
            binder_index,
            arity: call.arity,
        });
        Ok(CheckedApplicationDisposition::ConsumedHere)
    }

    fn mint_checked_computational_ih_instance(
        &mut self,
        value: &mut Lowered,
    ) -> Result<Option<CheckedRecursiveInvocationInstance>, CraneliftBackendError> {
        let Some(pending) = self.pending_computational_ih_call.take() else {
            return Ok(None);
        };
        let call_template_id = pending.call_template_id;
        let Lowered::ComputationalRecursorClosure { invocation, .. } = value else {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "computational IH marker was applied to an ordinary value",
            ));
        };
        let plan = self.oriented_subcontinuation_plan.as_ref().ok_or_else(|| {
            unsupported(
                "OrientedSubcontinuationPlanV1",
                "computational IH invocation has no checked plan",
            )
        })?;
        let call = plan
            .computational_ih_call(call_template_id)
            .ok_or_else(|| {
                unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "computational IH invocation has no checked call template",
                )
            })?;
        if invocation.computational_ih_slot_template_id != Some(call.slot_template_id) {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "computational IH invocation marker names a different slot",
            ));
        }
        let parent_frame_template_id = call.parent_frame_template_id.ok_or_else(|| {
            unsupported(
                "OrientedSubcontinuationPlanV1",
                "computational IH invocation has no checked static parent",
            )
        })?;
        let segment_site_id = call.parent_segment_site_id.ok_or_else(|| {
            unsupported(
                "OrientedSubcontinuationPlanV1",
                "computational IH invocation has no checked parent segment",
            )
        })?;
        let mut parents = std::iter::once(&invocation.selection)
            .chain(
                invocation
                    .unwind
                    .later_wrappers_in_construction_order
                    .iter(),
            )
            .filter(|layer| {
                layer.semantic_pending && layer.checked_frame_id == Some(parent_frame_template_id)
            });
        let selected = parents.next().ok_or_else(|| {
            unsupported(
                "OrientedSubcontinuationPlanV1",
                "computational IH closure has no exact checked open parent occurrence",
            )
        })?;
        if parents.next().is_some() {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "computational IH closure has multiple candidate dynamic parent occurrences",
            ));
        }
        let parent_invocation_instance_id = match selected.checked_invocation_id {
            Some(instance_id) => instance_id,
            None if selected.checked_invocation_source.is_none() => 0,
            None => {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    format!(
                        "computational IH closure-selected occurrence has no dynamic parent identity: frame={:?} source={:?} depth={} handles={:?}",
                        selected.checked_frame_id,
                        selected.checked_invocation_source,
                        selected.checked_invocation_depth,
                        invocation.dynamic_splice_edges,
                    ),
                ))
            }
        };
        let selected_site = plan
            .frame(parent_frame_template_id)
            .map(|frame| frame.segment_site_id)
            .ok_or_else(|| {
                unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "computational IH closure-selected occurrence names a stale parent frame",
                )
            })?;
        if selected_site != segment_site_id {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "computational IH closure-selected occurrence crosses its checked segment",
            ));
        }
        let edge_id = DynamicSpliceEdgeId(self.next_dynamic_splice_edge);
        self.next_dynamic_splice_edge = self
            .next_dynamic_splice_edge
            .checked_add(1)
            .expect("compiler-private dynamic splice edge identity exhausted");
        let instance = CheckedRecursiveInvocationInstance {
            source: InvocationTemplateRef::ComputationalIHCall(call_template_id),
            invocation_instance_id: self.next_recursive_invocation_instance,
            semantic_depth: self.active_recursive_invocations.len() + 1,
            dynamic_splice_edge: Some(edge_id),
        };
        self.next_recursive_invocation_instance = self
            .next_recursive_invocation_instance
            .checked_add(1)
            .expect("compiler-private invocation identity exhausted");
        if self
            .dynamic_splice_edges
            .insert(
                edge_id,
                DynamicSpliceEdge {
                    edge_id,
                    child_invocation_instance_id: instance.invocation_instance_id,
                    parent_invocation_instance_id,
                    checked_call_template_id: call_template_id,
                    parent_frame_template_id,
                    segment_site_id,
                },
            )
            .is_some()
        {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "dynamic splice edge identity was minted twice",
            ));
        }
        invocation.dynamic_splice_edges.push(edge_id);
        Ok(Some(instance))
    }

    fn validate_source_dynamic_splice_parent(
        &self,
        instance: CheckedRecursiveInvocationInstance,
        open: &OwnedSelectedScope,
    ) -> Result<(), CraneliftBackendError> {
        let edge_id = instance.dynamic_splice_edge.ok_or_else(|| {
            unsupported(
                "OrientedSubcontinuationPlanV1",
                "source IH invocation has no affine dynamic splice edge",
            )
        })?;
        let edge = self.dynamic_splice_edges.get(&edge_id).ok_or_else(|| {
            unsupported(
                "OrientedSubcontinuationPlanV1",
                "source IH invocation names a deleted or already-consumed dynamic splice edge",
            )
        })?;
        if edge.child_invocation_instance_id != instance.invocation_instance_id
            || edge.parent_invocation_instance_id != open.frame.checked_invocation_id.unwrap_or(0)
            || Some(edge.parent_frame_template_id) != open.frame.checked_frame_id
        {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "source open occurrence disagrees with the closure-selected dynamic parent",
            ));
        }
        Ok(())
    }

    fn take_dynamic_splice_edges(
        &mut self,
        segment: &RecursorInvocationSegment,
    ) -> Result<Vec<DynamicSpliceEdge>, CraneliftBackendError> {
        let mut seen = BTreeSet::new();
        let mut edges = Vec::with_capacity(segment.dynamic_splice_edges.len());
        for edge_id in &segment.dynamic_splice_edges {
            if !seen.insert(*edge_id) {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "dynamic splice edge handle is duplicated in one invocation carrier",
                ));
            }
            let edge = self.dynamic_splice_edges.remove(edge_id).ok_or_else(|| {
                unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "dynamic splice edge was deleted, replayed, or consumed by a sibling",
                )
            })?;
            if edge.edge_id != *edge_id {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "dynamic splice edge ledger identity is stale",
                ));
            }
            edges.push(edge);
        }
        Ok(edges)
    }

    fn finish_checked_computational_ih_marker(
        &mut self,
        value: LoweringOperand,
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        // **`RT-DECL-CLOSURE-PORT` `D5a` — closeout is keyed on WHETHER THE
        // MARKER WAS CONSUMED, not on what the body produced.**
        //
        // ⭐ When it was already consumed -- at the exact static-worker call
        // edge, or by a recursor-closure callee -- the marker's obligations are
        // discharged and this operand is simply the application's result. It is
        // forwarded **unchanged, including `Carried`**: the marker denotes the
        // application, so once the application is bound there is nothing left
        // here to read a template for. ⛔ Refusing a `Carried` here would refuse
        // the ported route's *result* for a template it was never supposed to
        // carry.
        if self.pending_computational_ih_call.is_none() {
            return Ok(value);
        }
        // ⭐ A still-PENDING marker keeps the specialized-template path exactly
        // as it was: the marker consumes a **recursor closure template**, a
        // carried boundary word is not one and never becomes one, so this stays
        // a specialized-only surface with the ruled fail-closed arm.
        let mut value = value.specialized_at("a checked computational-IH marker")?;
        let Some(instance) = self.mint_checked_computational_ih_instance(&mut value)? else {
            return Ok(LoweringOperand::Specialized(value));
        };
        let Lowered::ComputationalRecursorClosure { invocation, .. } = &mut value else {
            unreachable!("IH instance mint validates one recursor closure")
        };
        let plan = self.oriented_subcontinuation_plan.as_ref().ok_or_else(|| {
            unsupported(
                "OrientedSubcontinuationPlanV1",
                "computational IH invocation has no checked plan",
            )
        })?;
        // Qualify the exact reusable template sequence at marker consumption.
        // Existing child-qualified layers remain untouched when later parent
        // wrappers are added to the same flattened carrier.
        // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the segment's own receipt, the
        // same operand `compose_oriented_subcontinuation` uses. Nothing ambient
        // is consulted, so marker consumption and composition cannot disagree
        // about which segment the splice built.
        instantiate_checked_invocation_segment(plan, instance, invocation)?;
        Ok(LoweringOperand::Specialized(value))
    }

    fn consume_checked_recursive_invocation_call(
        &mut self,
        symbol: &RuntimeSymbol,
    ) -> Result<Option<CheckedRecursiveInvocationInstance>, CraneliftBackendError> {
        let Some(instance) = self.pending_recursive_call.take() else {
            return Ok(None);
        };
        let InvocationTemplateRef::SameSccCall(call_template_id) = instance.source else {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "same-SCC call consumer received a computational IH invocation",
            ));
        };
        let call = self
            .oriented_subcontinuation_plan
            .as_ref()
            .and_then(|plan| plan.recursive_call(call_template_id))
            .ok_or_else(|| {
                unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "active recursive invocation has no checked template",
                )
            })?;
        if &call.callee != symbol {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "recursive invocation marker was transplanted to another callee",
            ));
        }
        Ok(Some(instance))
    }

    fn consume_checked_subcontinuation_frame(
        &mut self,
        cases: &[crate::RuntimeComputationalMatchCase],
        default: &RuntimeTrap,
    ) -> Result<Option<u64>, CraneliftBackendError> {
        let Some(frame_id) = self.active_subcontinuation_frame.take() else {
            return Ok(None);
        };
        let frame = self
            .oriented_subcontinuation_plan
            .as_ref()
            .and_then(|plan| plan.frame(frame_id))
            .ok_or_else(|| {
                unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "checked Runtime marker has no transported frame entry",
                )
            })?;
        if frame.runtime_frame_fingerprint
            != crate::compiler_private_computational_match_frame_fingerprint(cases, default)
        {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "checked Runtime marker no longer denotes its planned frame",
            ));
        }
        let invocation_id = self
            .active_recursive_invocations
            .last()
            .map_or(0, |instance| instance.invocation_instance_id);
        if !self
            .consumed_subcontinuation_frames
            .insert((invocation_id, frame_id))
        {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "checked Runtime frame marker was consumed more than once",
            ));
        }
        // `D8n` — the observation, at the REAL consumption seam and written from
        // production state that is in hand here: the pair the ledger just
        // accepted, and the defining `FuncId` of the function being emitted. ⛔ Nothing is reconstructed from a fixture or looked up in an
        // expected plan; a reader that rebuilt either side would agree with
        // itself and say nothing about what production did.
        #[cfg(test)]
        record_d8n_frame_consumption(self.defining_function_id, invocation_id, frame_id);
        Ok(Some(frame_id))
    }

    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D8m` — the checked computational frame
    /// tuple, derived ONCE.**
    ///
    /// ⭐⭐ Every consumer of a source `ComputationalMatch` needs the same four
    /// facts, and they are not independent: the invocation id exists only when
    /// a frame id does, and the source and depth both come from the innermost
    /// active recursive invocation. Deriving them at each site is how they
    /// drift.
    ///
    /// ⛔ `D8m`'s first draft hard-coded three of the four at the checked bridge
    /// -- `checked_invocation_id: None`, `checked_invocation_source: None`,
    /// `checked_invocation_depth: 0` -- which made the bridge's tuple a
    /// different object from the one the direct path builds for the SAME source
    /// match. Sharing the derivation is what makes "the bridge is an
    /// optimization of the source match" true of the whole tuple rather than of
    /// its first field.
    ///
    /// ⛔ This is not a validator. It consumes the marker through the existing
    /// entry/consumption pair and reads the invocation stack; it checks nothing
    /// the callers did not already have checked for them.
    fn checked_computational_frame(
        &mut self,
        cases: &[crate::RuntimeComputationalMatchCase],
        default: &RuntimeTrap,
    ) -> Result<CheckedComputationalFrame, CraneliftBackendError> {
        let id = self.consume_checked_subcontinuation_frame(cases, default)?;
        Ok(CheckedComputationalFrame {
            id,
            invocation_id: id.map(|_| {
                self.active_recursive_invocations
                    .last()
                    .map_or(0, |instance| instance.invocation_instance_id)
            }),
            invocation_source: self
                .active_recursive_invocations
                .last()
                .map(|instance| instance.source),
            invocation_depth: self
                .active_recursive_invocations
                .last()
                .map_or(0, |instance| instance.semantic_depth),
        })
    }

    fn computational_ih_slots_for_case(
        &self,
        case: &crate::RuntimeComputationalMatchCase,
        checked_frame_id: Option<u64>,
    ) -> Result<Vec<Option<u64>>, CraneliftBackendError> {
        let RuntimeExpr::CheckedComputationalIHSlots {
            slot_template_ids, ..
        } = &case.body
        else {
            if checked_frame_id.is_some() && !case.recursive_positions.is_empty() {
                return Err(unsupported(
                    "OrientedSubcontinuationPlanV1",
                    "checked computational case is missing its IH slot marker",
                ));
            }
            return Ok(vec![None; case.recursive_positions.len()]);
        };
        let frame_id = checked_frame_id.ok_or_else(|| {
            unsupported(
                "OrientedSubcontinuationPlanV1",
                "computational IH slot marker is detached from its checked frame",
            )
        })?;
        if slot_template_ids.len() != case.recursive_positions.len() {
            return Err(unsupported(
                "OrientedSubcontinuationPlanV1",
                "computational IH slot marker is not bijective with recursive positions",
            ));
        }
        let plan = self.oriented_subcontinuation_plan.as_ref().ok_or_else(|| {
            unsupported(
                "OrientedSubcontinuationPlanV1",
                "computational IH slot marker has no checked plan",
            )
        })?;
        let mut seen = BTreeSet::new();
        slot_template_ids
            .iter()
            .copied()
            .zip(case.recursive_positions.iter().copied())
            .map(|(slot_template_id, recursive_position)| {
                if !seen.insert(slot_template_id) {
                    return Err(unsupported(
                        "OrientedSubcontinuationPlanV1",
                        "computational IH case repeats a checked slot template",
                    ));
                }
                let slot = plan
                    .computational_ih_slot(slot_template_id)
                    .ok_or_else(|| {
                        unsupported(
                            "OrientedSubcontinuationPlanV1",
                            "computational IH case names a stale slot template",
                        )
                    })?;
                // `D8m` — the PAIR, recorded BEFORE the binding law runs.
                //
                // Recorded after it, the two components could not disagree: the
                // law is exactly `slot.frame_template_id == frame_id`, so an
                // observation taken past it agrees with the plan by
                // construction and a test comparing them would be agreeing with
                // itself. Taken here it reports what the bridge actually
                // transported beside what the plan named for that slot, and the
                // transplant control can show the disagreement rather than only
                // the refusal it causes.
                #[cfg(test)]
                record_d8m_slot_frame_pair(
                    self.defining_function_id,
                    frame_id,
                    slot.slot_template_id,
                );
                if slot.frame_template_id != frame_id
                    || slot.constructor != case.constructor
                    || slot.recursive_position != recursive_position as u64
                {
                    return Err(unsupported(
                        "OrientedSubcontinuationPlanV1",
                        "computational IH slot constructor/position/frame binding is stale",
                    ));
                }
                // `D8n` — the observation at the REAL slot-reconciliation seam,
                // recorded only once the plan-named slot has been resolved and
                // held to its frame, constructor and position. The slot id is
                // the plan's own; the identity is the defining `FuncId`.
                #[cfg(test)]
                record_d8n_slot_reconciliation(self.defining_function_id, slot.slot_template_id);
                Ok(Some(slot_template_id))
            })
            .collect()
    }

    fn enter_oriented_semantic_region(&mut self, checked: bool) {
        if checked {
            self.active_oriented_semantic_regions = self
                .active_oriented_semantic_regions
                .checked_add(1)
                .expect("compiler-private oriented segment depth exhausted");
        }
    }

    fn leave_oriented_semantic_region(&mut self, checked: bool) {
        if checked {
            self.active_oriented_semantic_regions = self
                .active_oriented_semantic_regions
                .checked_sub(1)
                .expect("oriented semantic region must be entered exactly once");
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn make_computational_recursor(
        &mut self,
        recursive: LoweringOperand,
        cases: Vec<crate::RuntimeComputationalMatchCase>,
        default: RuntimeTrap,
        outer_env: Vec<LoweringEnvironmentBinding>,
        static_origin: StaticOriginId,
        provenance: RecursorFrameProvenance,
        checked_frame_id: Option<u64>,
        computational_ih_slot_template_id: Option<u64>,
        origin: RecursorProducerOriginId,
        sibling_position: usize,
        role: RecursorLayerRole,
        activation: ContinuationActivationId,
        resume_cursor: ContinuationCursorId,
        splice_caller: Option<&ActiveContinuationFrame<'_>>,
        source_control: Option<(
            &SourceSelectedContinuation<'_>,
            &[SourceSelectedContinuation<'_>],
        )>,
        recursive_unit_body: Option<StaticOriginId>,
    ) -> Result<LoweringOperand, CraneliftBackendError> {
        // ---- `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — THE CONSUMPTION POINT.
        //
        // This is where the splice's pending semantic edge becomes a layer of a
        // dynamic invocation segment, so this is where its capability is spent.
        // Spending it here, rather than at composition, is what binds the
        // receipt to the ONE segment the edge joined: every other segment built
        // in this same fused body carries no capability, gets `Ordinary`, and is
        // measured against the ordinary sequence.
        //
        // Taken BEFORE any of the construction below can fail, so a refusal
        // downstream cannot leave the capability outstanding and silently
        // re-spendable by a later edge.
        let recursive_unit_body = recursive_unit_body.or_else(|| match &recursive {
            LoweringOperand::Specialized(Lowered::Closure { body, .. }) => Some(*body),
            LoweringOperand::Specialized(_) | LoweringOperand::Carried(_) => None,
        });
        let (residual, payload) = decompose_computational_recursor(recursive);
        let active_instance = self.active_recursive_invocations.last().copied();
        // ⛔ **The frame identity is TRANSPORTED, never inferred**
        // (`dec_s30rdnb1dvgk`). This site used to fall back, when
        // `checked_frame_id` was `None`, to `find`ing a `callee_frame_templates`
        // entry whose `runtime_frame_fingerprint` equalled one recomputed from
        // `cases`/`default`. `AC-F1` deliberately makes body-only differences
        // share a header fingerprint, so that `find` cannot discriminate a
        // callee declaration's two same-family computational frames — it
        // returns the first, silently.
        //
        // ⛔ Do not restore any recovery here, in any spelling: not header
        // equality, not body equality, not `StaticOriginId`, not vector
        // position, and not "the only remaining match." Each of those is
        // Runtime *inference*; the oriented plan's checked identity is the
        // authority. A missing identity is rejected in
        // `instantiate_checked_invocation_segment`, before CFG.
        let exact_frame_id = checked_frame_id;
        let invocation_id = exact_frame_id
            .and_then(|_| active_instance.map(|instance| instance.invocation_instance_id));
        let invocation_source = active_instance.map(|instance| instance.source);
        let invocation_depth = active_instance.map_or(0, |instance| instance.semantic_depth);
        let mut current_layer = ComputationalRecursorLayer {
            cases,
            default,
            outer_env,
            static_origin,
            provenance,
            role,
            checked_frame_id: exact_frame_id,
            checked_invocation_id: invocation_id,
            checked_invocation_source: invocation_source,
            checked_invocation_depth: invocation_depth,
            semantic_pending: true,
        };
        let segment_origin = payload
            .as_ref()
            .map(|(_, invocation)| invocation.origin)
            .unwrap_or(origin);
        let segment_sibling_position = payload
            .as_ref()
            .map(|(_, invocation)| invocation.sibling_position)
            .unwrap_or(sibling_position);
        let segment_checked_invocation = payload
            .as_ref()
            .and_then(|(_, invocation)| invocation.checked_invocation)
            .or(active_instance);
        let segment_recursive_unit_body = payload
            .as_ref()
            .and_then(|(_, invocation)| invocation.recursive_unit_body)
            .or(recursive_unit_body);
        let segment_dynamic_splice_edges = payload
            .as_ref()
            .map(|(_, invocation)| invocation.dynamic_splice_edges.clone())
            .unwrap_or_default();
        let (selection, unwind) =
            if let Some((_, invocation)) = payload {
                let splice_caller = splice_caller.ok_or_else(|| {
                    unsupported(
                        "ComputationalRecursor",
                        "recursive payload splice has no active continuation",
                    )
                })?;
                let source_cursor_is_live = source_control.is_some_and(|(selected, lineage)| {
                    source_active_cursor(selected, lineage, invocation.resume_cursor).is_some()
                });
                if !active_context_contains_cursor(splice_caller, invocation.resume_cursor)
                    && !source_cursor_is_live
                    && !recursor_invocation_is_checked(&invocation)
                {
                    return Err(unsupported(
                        "ComputationalRecursor",
                        "recursive payload resume cursor is not active",
                    ));
                }
                let mut unwind = invocation.unwind;
                let parent_scope = unwind.later_wrappers_in_construction_order.last().and_then(
                    |layer| match layer.role {
                        RecursorLayerRole::ExitsScope { scope_origin, .. } => Some(scope_origin),
                        RecursorLayerRole::SelectsOccurrence { .. } => None,
                    },
                );
                let unwind_role = match role {
                    RecursorLayerRole::SelectsOccurrence { origin: _ } => {
                        RecursorLayerRole::ExitsScope {
                            origin: segment_origin,
                            scope_origin: origin,
                            parent_scope,
                        }
                    }
                    RecursorLayerRole::ExitsScope {
                        origin: _,
                        scope_origin,
                        parent_scope,
                    } => RecursorLayerRole::ExitsScope {
                        origin: segment_origin,
                        scope_origin,
                        parent_scope,
                    },
                };
                current_layer.role = unwind_role;
                unwind
                    .later_wrappers_in_construction_order
                    .push(current_layer);
                if let Some((selected, lineage)) = source_control {
                    if selected.selected_scope.is_none() {
                        return Err(unsupported(
                            "ComputationalRecursor",
                            "source recursor invocation is missing its owned selected scope",
                        ));
                    }
                    for scope in lineage
                        .iter()
                        .filter_map(|selected| selected.selected_scope.as_ref())
                        .chain(selected.selected_scope.iter())
                    {
                        if unwind
                            .later_wrappers_in_construction_order
                            .iter()
                            .any(|layer| {
                                matches!(
                                    layer.role,
                                    RecursorLayerRole::ExitsScope { scope_origin, .. }
                                        if scope_origin == scope.scope_origin
                                )
                            })
                        {
                            continue;
                        }
                        unwind.later_wrappers_in_construction_order.push(
                            ComputationalRecursorLayer {
                                cases: scope.frame.cases.clone(),
                                default: scope.frame.default.clone(),
                                outer_env: scope.frame.outer_env.clone(),
                                static_origin: scope.frame.static_origin,
                                provenance: scope.frame.provenance,
                                checked_frame_id: scope.frame.checked_frame_id,
                                checked_invocation_id: scope.frame.checked_invocation_id,
                                checked_invocation_source: scope.frame.checked_invocation_source,
                                checked_invocation_depth: scope.frame.checked_invocation_depth,
                                semantic_pending: false,
                                role: RecursorLayerRole::ExitsScope {
                                    origin: segment_origin,
                                    scope_origin: scope.scope_origin,
                                    parent_scope: scope.parent_scope,
                                },
                            },
                        );
                    }
                }
                (invocation.selection, unwind)
            } else {
                (
                    current_layer,
                    RecursorUnwindStack {
                        later_wrappers_in_construction_order: Vec::new(),
                    },
                )
            };
        // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the receipt, joined with any the
        // wrapped payload already carried.
        //
        // Wrapping an existing segment keeps its receipt: the inner edge already
        // spent the capability and this outer layer joins that same segment. Two
        // capabilities meeting on one segment would be two splices composing one
        // construction, which is not a shape the checked plan can describe, so it
        // refuses rather than picking one.
        let mut invocation = RecursorInvocationSegment::new(
            segment_origin,
            segment_sibling_position,
            selection,
            unwind,
            resume_cursor,
            segment_checked_invocation,
            computational_ih_slot_template_id,
        );
        invocation.recursive_unit_body = segment_recursive_unit_body;
        invocation.dynamic_splice_edges = segment_dynamic_splice_edges;
        Ok(LoweringOperand::Specialized(
            Lowered::ComputationalRecursorClosure {
                residual: Box::new(residual),
                activation,
                invocation,
            },
        ))
    }



    fn restore_root_terminal_authority(
        &mut self,
        authority: Option<RootTerminalAnswerAuthority>,
        expected_outer: ContinuationCursorId,
    ) -> Result<(), CraneliftBackendError> {
        let Some(mut authority) = authority else {
            return Ok(());
        };
        if authority.outer_cursor != Some(expected_outer) {
            return Err(backend(BackendFailure::PlannerInvariant(
                "checked root answer authority returned through the wrong outer cursor"
                    .to_string(),
            )));
        }
        // The exact source-machine delimiter consumes this cursor binding.
        // A later source-machine episode may bind the same affine root token
        // to its own exact outer cursor; retaining the old cursor would turn a
        // lawful sequential episode into an apparent transplant.
        authority.outer_cursor = None;
        if self.root_terminal_authority.replace(authority).is_some() {
            return Err(backend(BackendFailure::PlannerInvariant(
                "checked root answer authority was duplicated across source control".to_string(),
            )));
        }
        Ok(())
    }

    /// The checked root cut determines the temporary scalar representation
    /// used at internal CFG joins. This is validation metadata only: it cannot
    /// mint or consume terminal authority, which remains affine in
    /// `RootTerminalAnswerAuthority` until `emit_result`.
    fn has_checked_root_exit_representation(&self) -> bool {
        self.process_object
            && self.native_join_plan.as_ref().is_some_and(|plan| {
                plan.sites.iter().any(|site| {
                    site.runtime_frame_fingerprint == crate::NATIVE_JOIN_INVOCATION_RETURN_FRAME_V1
                        && site.checked_occurrence_path == [0]
                        && site.answer_kind == crate::NativeJoinAnswerKindV1::ExitCode
                        && self.consumed_join_sites.contains(&site.site_id)
                })
            })
    }

    fn mint_terminal_answer_authority(
        &mut self,
    ) -> Result<TerminalAnswerAuthority, CraneliftBackendError> {
        debug_assert_eq!(
            self.live_source_continuations == 0,
            self.source_control_root.is_none(),
            "source-control ownership and diagnostic depth must agree"
        );
        let authority = self.root_terminal_authority.take().ok_or_else(|| {
            backend(BackendFailure::PlannerInvariant(
                "terminal answer has no affine checked-root authority".to_string(),
            ))
        })?;
        let site = self
            .native_join_plan
            .as_ref()
            .and_then(|plan| {
                plan.sites
                    .iter()
                    .find(|site| site.site_id == authority.site_id)
            })
            .ok_or_else(|| {
                unsupported(
                    "NativeJoinPlanV1",
                    "terminal answer authority names a missing checked-root site",
                )
            })?;
        if !self.process_object
            || site.runtime_frame_fingerprint != crate::NATIVE_JOIN_INVOCATION_RETURN_FRAME_V1
            || site.checked_occurrence_path != [0]
            || site.answer_kind != crate::NativeJoinAnswerKindV1::ExitCode
            || site.checked_result_type_fingerprint != authority.checked_result_type_fingerprint
            || site.occurrence_binding_fingerprint != authority.occurrence_binding_fingerprint
            || !self.consumed_join_sites.contains(&authority.site_id)
            || authority.outer_cursor.is_some()
            || self.source_control_root.is_some()
            || self.active_oriented_semantic_regions != 0
            || self.active_subcontinuation_frame.is_some()
            || self.active_join_site.is_some()
        {
            return Err(unsupported(
                "NativeJoinPlanV1",
                "terminal answer authority does not match the exhausted checked root",
            ));
        }
        Ok(TerminalAnswerAuthority)
    }


    /// Scalarize only under the answer kind carried by an already-consumed
    /// checked join site. In particular, process-object mode is not evidence
    /// that an arbitrary constructor is terminal: only an `ExitCode` plan may
    /// invoke the terminal process decoder.
    /// ⭐ A **planned** join — same phase-bearing role as
    /// [`Self::merge_scalar_branch`], same pending boundary, and named the same
    /// way so `grep specialized_join_arm` reaches all three.
    fn merge_planned_scalar_branch(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        join_plan: &JoinPlanToken,
        lowered: LoweringOperand,
        required_kind: ScalarMergeKind,
        construct: &'static str,
    ) -> Result<(NativeScalarPairV1, ScalarMergeKind), CraneliftBackendError> {
        if join_plan.representation != JoinResultRepresentation::NativeScalarPair {
            return Err(backend_module(
                "carrier-result join reached a native checked-plan merge consumer".to_string(),
            ));
        }
        self.merge_scalar_operand(builder, lowered, Some(required_kind), construct)
    }

    fn lowered_from_scalar_pair(
        &mut self,
        kind: ScalarMergeKind,
        pair: NativeScalarPairV1,
    ) -> Lowered {
        match kind {
            ScalarMergeKind::Int => {
                self.function_local
                    .native_int_tags
                    .insert(pair.payload, pair.tag);
                Lowered::Int {
                    value: pair.payload,
                    known: None,
                }
            }
            ScalarMergeKind::Bool => Lowered::Bool {
                value: pair.payload,
                known: None,
            },
            ScalarMergeKind::StructuralNat => Lowered::StructuralNat(StructuralNatV1 {
                value: pair.payload,
            }),
            ScalarMergeKind::ExitCode => Lowered::ProcessExitStatus {
                value: pair.payload,
            },
            ScalarMergeKind::RecursiveBackedge => {
                unreachable!("backedges do not establish a merge result kind")
            }
        }
    }


    fn planned_join_site_for_frame(
        &mut self,
        frame: EliminatorFrame<'_>,
    ) -> Result<Option<crate::NativeJoinPlanSiteV1>, CraneliftBackendError> {
        let fingerprint = match frame {
            EliminatorFrame::Computational(frame) => {
                crate::compiler_private_computational_match_frame_fingerprint(
                    frame.cases,
                    frame.default,
                )
            }
            EliminatorFrame::Ordinary(frame) => {
                crate::compiler_private_ordinary_match_frame_fingerprint(frame.cases, frame.default)
            }
            EliminatorFrame::InvocationReturn => crate::NATIVE_JOIN_INVOCATION_RETURN_FRAME_V1,
            EliminatorFrame::PendingLet(_) | EliminatorFrame::Active(_) => return Ok(None),
        };
        let Some(plan) = &self.native_join_plan else {
            return Ok(None);
        };
        if matches!(frame, EliminatorFrame::InvocationReturn) && self.active_join_site.is_some() {
            return Err(unsupported(
                "NativeJoinPlanV1",
                "distinguished root cannot consume an active match occurrence marker",
            ));
        }
        let matches = match frame {
            EliminatorFrame::InvocationReturn => plan
                .sites
                .iter()
                .filter(|site| {
                    site.runtime_frame_fingerprint == crate::NATIVE_JOIN_INVOCATION_RETURN_FRAME_V1
                        && site.checked_occurrence_path == [0]
                        && site.answer_kind == crate::NativeJoinAnswerKindV1::ExitCode
                })
                .cloned()
                .collect::<Vec<_>>(),
            EliminatorFrame::Computational(_) | EliminatorFrame::Ordinary(_) => {
                let Some(site_id) = self.active_join_site else {
                    return Ok(None);
                };
                plan.sites
                    .iter()
                    .filter(|site| site.site_id == site_id)
                    .cloned()
                    .collect::<Vec<_>>()
            }
            EliminatorFrame::PendingLet(_) | EliminatorFrame::Active(_) => unreachable!(),
        };
        match matches.as_slice() {
            [] if self.active_join_site.is_some() => Err(unsupported(
                "NativeJoinPlanV1",
                "runtime occurrence has no exact checked join site",
            )),
            [] => Ok(None),
            [site] => {
                if site.runtime_frame_fingerprint != fingerprint
                    || site.occurrence_binding_fingerprint
                        != crate::compiler_private_join_occurrence_binding_fingerprint(
                            site.site_id,
                            &site.declaration,
                            &site.checked_occurrence_path,
                            site.checked_result_type_fingerprint,
                        )
                {
                    return Err(unsupported(
                        "NativeJoinPlanV1",
                        "checked join occurrence binding is stale or inconsistent",
                    ));
                }
                if !self.consumed_join_sites.insert(site.site_id)
                    && !matches!(frame, EliminatorFrame::InvocationReturn)
                {
                    return Err(unsupported(
                        "NativeJoinPlanV1",
                        "checked join occurrence was consumed twice",
                    ));
                }
                if !matches!(frame, EliminatorFrame::InvocationReturn) {
                    self.active_join_site = None;
                }
                Ok(Some(site.clone()))
            }
            _ => Err(unsupported(
                "NativeJoinPlanV1",
                "checked cut identity resolves to multiple plan sites",
            )),
        }
    }

    fn require_complete_join_plan_consumption(&self) -> Result<(), CraneliftBackendError> {
        let Some(plan) = &self.native_join_plan else {
            return Ok(());
        };
        let planned = plan
            .sites
            .iter()
            .map(|site| site.site_id)
            .collect::<BTreeSet<_>>();
        if planned != self.consumed_join_sites {
            return Err(unsupported(
                "NativeJoinPlanV1",
                format!(
                    "checked join plan contains an unconsumed or orphan site: planned {planned:?}, consumed {:?}",
                    self.consumed_join_sites
                ),
            ));
        }
        Ok(())
    }

    fn require_complete_dynamic_splice_edge_consumption(
        &self,
    ) -> Result<(), CraneliftBackendError> {
        if self.dynamic_splice_edges.is_empty() {
            return Ok(());
        }
        Err(unsupported(
            "OrientedSubcontinuationPlanV1",
            format!(
                "checked lowering left affine dynamic splice edges unconsumed: {:?}",
                self.dynamic_splice_edges.keys().collect::<Vec<_>>(),
            ),
        ))
    }

    fn take_distinguished_root_answer_authority(
        &mut self,
    ) -> Result<Option<RootTerminalAnswerAuthority>, CraneliftBackendError> {
        let Some(plan) = &self.native_join_plan else {
            return if self.process_object {
                Err(unsupported(
                    "NativeJoinPlanV1",
                    "process-object lowering has no checked distinguished-root answer authority",
                ))
            } else {
                Ok(None)
            };
        };
        let roots = plan
            .sites
            .iter()
            .filter(|site| {
                site.runtime_frame_fingerprint == crate::NATIVE_JOIN_INVOCATION_RETURN_FRAME_V1
                    && site.checked_occurrence_path == [0]
                    && site.answer_kind == crate::NativeJoinAnswerKindV1::ExitCode
            })
            .cloned()
            .collect::<Vec<_>>();
        let site = match roots.as_slice() {
            [] if !self.process_object => return Ok(None),
            [] => {
                return Err(unsupported(
                    "NativeJoinPlanV1",
                    "process-object lowering has no checked distinguished-root answer authority",
                ));
            }
            [site] => site,
            _ => {
                return Err(unsupported(
                    "NativeJoinPlanV1",
                    "checked package contains multiple distinguished root join sites",
                ));
            }
        };
        if site.occurrence_binding_fingerprint
            != crate::compiler_private_join_occurrence_binding_fingerprint(
                site.site_id,
                &site.declaration,
                &site.checked_occurrence_path,
                site.checked_result_type_fingerprint,
            )
        {
            return Err(unsupported(
                "NativeJoinPlanV1",
                "distinguished root join occurrence binding is stale or inconsistent",
            ));
        }
        if !self.consumed_join_sites.insert(site.site_id) {
            return Err(unsupported(
                "NativeJoinPlanV1",
                "checked distinguished-root answer authority was consumed more than once",
            ));
        }
        Ok(Some(RootTerminalAnswerAuthority {
            site_id: site.site_id,
            checked_result_type_fingerprint: site.checked_result_type_fingerprint,
            occurrence_binding_fingerprint: site.occurrence_binding_fingerprint,
            outer_cursor: None,
        }))
    }

    fn scalar_kind_from_plan(kind: crate::NativeJoinAnswerKindV1) -> ScalarMergeKind {
        match kind {
            crate::NativeJoinAnswerKindV1::Int => ScalarMergeKind::Int,
            crate::NativeJoinAnswerKindV1::Bool => ScalarMergeKind::Bool,
            crate::NativeJoinAnswerKindV1::StructuralNat => ScalarMergeKind::StructuralNat,
            crate::NativeJoinAnswerKindV1::ExitCode => ScalarMergeKind::ExitCode,
        }
    }

    fn declaration_call_produces_deforestable_aggregate(&self, expr: &RuntimeExpr) -> bool {
        let RuntimeExpr::Call { callee, .. } = expr else {
            return false;
        };
        let RuntimeExpr::DeclarationRef { symbol } = callee.as_ref() else {
            return false;
        };
        let Some(declaration) = self.declarations.get(symbol.as_str()).copied() else {
            return false;
        };
        let RuntimeDeclarationKind::Transparent {
            body:
                RuntimeExpr::Closure {
                    body: declaration_body,
                    ..
                },
        } = &declaration.kind
        else {
            return false;
        };
        produces_recursive_deforestable_aggregate(declaration_body, symbol)
    }




















    fn require_i64(
        builder: &mut FunctionBuilder<'_>,
        actual: cranelift_codegen::ir::Value,
        expected: i64,
    ) {
        let valid = builder.create_block();
        let invalid = builder.create_block();
        let matches = builder.ins().icmp_imm(
            cranelift_codegen::ir::condcodes::IntCC::Equal,
            actual,
            expected,
        );
        builder.ins().brif(matches, valid, &[], invalid, &[]);
        builder.switch_to_block(invalid);
        let failure = builder.ins().iconst(types::I64, -1);
        builder.ins().return_(&[failure]);
        builder.switch_to_block(valid);
    }

    fn require_one_of_i64(
        builder: &mut FunctionBuilder<'_>,
        actual: cranelift_codegen::ir::Value,
        expected: &[i64],
    ) {
        let valid = builder.create_block();
        let invalid = builder.create_block();
        let mut matches = builder.ins().icmp_imm(
            cranelift_codegen::ir::condcodes::IntCC::Equal,
            actual,
            expected[0],
        );
        for expected in &expected[1..] {
            let next = builder.ins().icmp_imm(
                cranelift_codegen::ir::condcodes::IntCC::Equal,
                actual,
                *expected,
            );
            matches = builder.ins().bor(matches, next);
        }
        builder.ins().brif(matches, valid, &[], invalid, &[]);
        builder.switch_to_block(invalid);
        let failure = builder.ins().iconst(types::I64, -1);
        builder.ins().return_(&[failure]);
        builder.switch_to_block(valid);
    }

    fn require_nonzero(builder: &mut FunctionBuilder<'_>, value: cranelift_codegen::ir::Value) {
        let valid = builder.create_block();
        let invalid = builder.create_block();
        let present =
            builder
                .ins()
                .icmp_imm(cranelift_codegen::ir::condcodes::IntCC::NotEqual, value, 0);
        builder.ins().brif(present, valid, &[], invalid, &[]);
        builder.switch_to_block(invalid);
        let failure = builder.ins().iconst(types::I64, -1);
        builder.ins().return_(&[failure]);
        builder.switch_to_block(valid);
    }







    fn lower_value(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        value: &RuntimeValue,
    ) -> Result<Lowered, CraneliftBackendError> {
        match value {
            RuntimeValue::Bool(value) => Ok(Lowered::Bool {
                value: builder.ins().iconst(types::I64, i64::from(*value)),
                known: Some(*value),
            }),
            RuntimeValue::Int(crate::RuntimeIntV1::Small(value)) => Ok(Lowered::Int {
                value: builder.ins().iconst(types::I64, *value),
                known: Some(*value),
            }),
            RuntimeValue::Int(value @ crate::RuntimeIntV1::Big { .. }) => {
                self.lower_big_int_constant(builder, value)
            }
            RuntimeValue::Bytes(value) => Ok(Lowered::Bytes(value.clone())),
            RuntimeValue::String(value) => Ok(Lowered::String(value.clone())),
            RuntimeValue::Constructor { constructor, args } => Ok(Lowered::Constructor {
                constructor: constructor.clone(),
                synthesized_identity: None,
                occurrence: None,
                args: args
                    .iter()
                    .map(|arg| {
                        self.lower_value(builder, arg)
                            .map(ConstructorField::specialized)
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            }),
            // ⚠ A VALUE-domain record has no occurrence in the program, so it
            // has no producer authority. Stated as an absence rather than
            // filled in: it fails closed at the allocation.
            RuntimeValue::Record { fields } => Ok(Lowered::Record {
                occurrence: None,
                fields: fields
                    .iter()
                    .map(|(name, value)| {
                        Ok(LoweredRecordField {
                            name: name.clone(),
                            // No occurrence, so no planned schema either. Both
                            // absences are stated at the same place.
                            identity: None,
                            value: self.lower_value(builder, value)?,
                        })
                    })
                    .collect::<Result<Vec<_>, CraneliftBackendError>>()?,
            }),
            RuntimeValue::ClosureRef { .. } => Err(unsupported(
                "ClosureRef",
                "pre-existing closure references are not lowered by the native backend",
            )),
            RuntimeValue::Unknown => Err(unsupported(
                "Unknown",
                "unknown runtime values must reject before backend lowering",
            )),
        }
    }

    fn lower_seed_capture(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        symbol: &str,
    ) -> Result<Lowered, CraneliftBackendError> {
        let value = self.seed_env.values.get(symbol).ok_or_else(|| {
            unsupported(
                "Closure",
                format!("capture {symbol} has no runtime value in the seed environment"),
            )
        })?;
        // ⛔ Wildcard-free, because this match **is** the boundary between the
        // represented path and the compiler-only one. A `_` arm here would let
        // a future ground-value variant fall silently onto whichever side
        // happened to be last.
        match value {
            RuntimeGroundValue::Bool(flag) => Ok(Lowered::Bool {
                value: self.artifact_static_payload(builder, symbol)?,
                // ⚠ `known` is retained deliberately and it is NOT a second
                // authority: it is the *compile-time* answer used for
                // specialization decisions, while `value` is the word the
                // running artifact actually reads. ⛔ If a specialization ever
                // substitutes `known` for `value` in emitted code, the borrow
                // becomes unobservable — which is why the control for this is a
                // mutation of the minted bytes, not a count of loads.
                known: Some(*flag),
            }),
            RuntimeGroundValue::Int(crate::RuntimeIntV1::Small(small)) => Ok(Lowered::Int {
                value: self.artifact_static_payload(builder, symbol)?,
                known: Some(*small),
            }),
            // ⚠ **The stated boundary of `D3`'s represented path.** These five
            // still lower through the compiler-side specialization lattice:
            // `lower_ground_value` returns `Lowered::Bytes`/`String`/
            // `Constructor`/`Record`, which hold the compiler's own Rust values
            // and carry no `ir::Value` at all, and a big integer goes through
            // the interning helper rather than a frame word.
            //
            // ⛔ **This is a boundary, not a second authority.** No value has two
            // paths: a scalar has exactly the artifact-static one, and these
            // have exactly the specialization one. ⚠ Giving them an
            // artifact-static representation needs a *reader* for the encoded
            // aggregate — the encoding exists (`seed_material`), the consumer
            // does not — and the runtime-`alloc` carrier is not a substitute,
            // because it produces activation-time storage for a slot declared
            // `AbiStorageOwner::ArtifactStatic`.
            RuntimeGroundValue::Int(big @ crate::RuntimeIntV1::Big { .. }) => {
                self.lower_big_int_constant(builder, big)
            }
            RuntimeGroundValue::Bytes(_)
            | RuntimeGroundValue::String(_)
            | RuntimeGroundValue::Constructor { .. }
            | RuntimeGroundValue::Record { .. } => self.lower_ground_value(builder, value),
        }
    }

    /// Load a seed symbol's scalar payload out of artifact-static material.
    ///
    /// ⛔ Fails closed. A symbol present in the environment but absent from the
    /// minted material means the two populations disagree, and folding the
    /// compile-time value in as a fallback would silently restore exactly the
    /// authority `D3` removes — with nothing to observe that it had.
    fn artifact_static_payload(
        &self,
        builder: &mut FunctionBuilder<'_>,
        symbol: &str,
    ) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
        self.function_local
            .seed_material
            .payload_word(builder, symbol)
            .ok_or_else(|| {
                unsupported(
                    "Closure",
                    format!("seed capture {symbol} has no artifact-static material minted for it"),
                )
            })
    }

    fn lower_ground_value(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        value: &RuntimeGroundValue,
    ) -> Result<Lowered, CraneliftBackendError> {
        match value {
            RuntimeGroundValue::Bool(value) => Ok(Lowered::Bool {
                value: builder.ins().iconst(types::I64, i64::from(*value)),
                known: Some(*value),
            }),
            RuntimeGroundValue::Int(crate::RuntimeIntV1::Small(value)) => Ok(Lowered::Int {
                value: builder.ins().iconst(types::I64, *value),
                known: Some(*value),
            }),
            RuntimeGroundValue::Int(value @ crate::RuntimeIntV1::Big { .. }) => {
                self.lower_big_int_constant(builder, value)
            }
            RuntimeGroundValue::Bytes(value) => Ok(Lowered::Bytes(value.clone())),
            RuntimeGroundValue::String(value) => Ok(Lowered::String(value.clone())),
            RuntimeGroundValue::Constructor { constructor, args } => Ok(Lowered::Constructor {
                constructor: constructor.clone(),
                synthesized_identity: None,
                occurrence: None,
                args: args
                    .iter()
                    .map(|arg| {
                        self.lower_ground_value(builder, arg)
                            .map(ConstructorField::specialized)
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            }),
            // ⚠ Likewise a ground-value record: no occurrence, stated.
            RuntimeGroundValue::Record { fields } => Ok(Lowered::Record {
                occurrence: None,
                fields: fields
                    .iter()
                    .map(|(name, value)| {
                        Ok(LoweredRecordField {
                            name: name.clone(),
                            identity: None,
                            value: self.lower_ground_value(builder, value)?,
                        })
                    })
                    .collect::<Result<Vec<_>, CraneliftBackendError>>()?,
            }),
        }
    }

    fn lower_big_int_constant(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        value: &crate::RuntimeIntV1,
    ) -> Result<Lowered, CraneliftBackendError> {
        let crate::RuntimeIntV1::Big { sign, limbs } = value else {
            unreachable!("Big constant lowering is called only for Big Int values")
        };
        let limb_count = limbs.len();
        let byte_len = u32::try_from(limbs.len().saturating_mul(std::mem::size_of::<u64>()))
            .map_err(|_| unsupported("RuntimeValue::Int", "Big Int literal is too large"))?;
        let limbs_slot = builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            byte_len,
            3,
        ));
        for (index, limb) in limbs.iter().enumerate() {
            let limb = builder.ins().iconst(types::I64, *limb as i64);
            builder.ins().stack_store(
                limb,
                limbs_slot,
                i32::try_from(index * std::mem::size_of::<u64>()).expect("Big limb offset is u32"),
            );
        }
        let output =
            builder.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 16, 3));
        let pointer_type = builder.func.dfg.value_type(
            self.function_local
                .native_int_arena
                .ok_or_else(|| unsupported("RuntimeValue::Int", "Big Int has no arena"))?,
        );
        let arena = self
            .function_local
            .native_int_arena
            .expect("Big Int arena was checked");
        let helper = self.function_local.native_int_intern.ok_or_else(|| {
            unsupported("RuntimeValue::Int", "Big Int has no local intern helper")
        })?;
        let sign = builder
            .ins()
            .iconst(types::I64, i64::from(matches!(sign, crate::Sign::Negative)));
        let limbs = builder.ins().stack_addr(pointer_type, limbs_slot, 0);
        let len = builder.ins().iconst(
            types::I64,
            i64::try_from(limb_count).expect("Big limb count fits i64"),
        );
        let output_ptr = builder.ins().stack_addr(pointer_type, output, 0);
        let call = builder
            .ins()
            .call(helper, &[arena, sign, limbs, len, output_ptr]);
        Self::require_i64(builder, builder.inst_results(call)[0], 0);
        let pair = NativeScalarPairV1 {
            tag: builder.ins().stack_load(types::I64, output, 0),
            payload: builder.ins().stack_load(types::I64, output, 8),
        };
        Ok(self.lowered_from_scalar_pair(ScalarMergeKind::Int, pair))
    }

    /// Reify a host-owned unsigned word into the exact native Int carrier.
    /// The shared local interner chooses Small or canonical Big; callers never
    /// reinterpret the raw `u64` bits as a signed scalar.
    fn lower_unsigned_u64_int(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        value: cranelift_codegen::ir::Value,
    ) -> Result<Lowered, CraneliftBackendError> {
        let arena = self.function_local.native_int_arena.ok_or_else(|| {
            unsupported("NativeInt", "unsigned Int producer has no invocation arena")
        })?;
        let helper = self.function_local.native_int_intern.ok_or_else(|| {
            unsupported(
                "NativeInt",
                "unsigned Int producer has no local intern helper",
            )
        })?;
        let limb =
            builder.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 8, 3));
        builder.ins().stack_store(value, limb, 0);
        let output =
            builder.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 16, 3));
        let pointer_type = builder.func.dfg.value_type(arena);
        let limb = builder.ins().stack_addr(pointer_type, limb, 0);
        let output_pointer = builder.ins().stack_addr(pointer_type, output, 0);
        let zero = builder.ins().iconst(types::I64, 0);
        let one = builder.ins().iconst(types::I64, 1);
        let call = builder
            .ins()
            .call(helper, &[arena, zero, limb, one, output_pointer]);
        Self::require_i64(builder, builder.inst_results(call)[0], 0);
        let pair = NativeScalarPairV1 {
            tag: builder.ins().stack_load(types::I64, output, 0),
            payload: builder.ins().stack_load(types::I64, output, 8),
        };
        Self::require_one_of_i64(
            builder,
            pair.tag,
            &[
                crate::NATIVE_INT_SMALL_TAG_V1 as i64,
                crate::NATIVE_INT_BIG_TAG_V1 as i64,
            ],
        );
        Ok(self.lowered_from_scalar_pair(ScalarMergeKind::Int, pair))
    }

    fn native_int_tag(
        &mut self,
        builder: &mut FunctionBuilder<'_>,
        payload: cranelift_codegen::ir::Value,
        known: Option<i64>,
    ) -> Result<cranelift_codegen::ir::Value, CraneliftBackendError> {
        if let Some(tag) = self.function_local.native_int_tags.get(&payload).copied() {
            return Ok(tag);
        }
        if known.is_some() {
            return Ok(builder
                .ins()
                .iconst(types::I64, crate::NATIVE_INT_SMALL_TAG_V1 as i64));
        }
        Err(unsupported(
            "NativeInt",
            "dynamic Int value lost its two-word tag transport",
        ))
    }




    /// ⛔ **A typed boundary: raw [`Lowered`] only, and STRUCTURALLY so**
    /// (`RT-FNSPLIT-C1` frame `§2h` ¶2).
    ///
    /// `§2h` admits a raw-`Lowered` helper only where it is **structurally
    /// incapable** of receiving, forwarding or returning a
    /// [`LoweringOperand::Carried`]. ⛔ *"Nothing passes it one today"* is
    /// explicitly **not** that property — present-corpus reachability is the
    /// argument the ruling was written to reject. This helper is closed on all
    /// three counts **by signature**, not by census:
    ///
    /// - **cannot receive** — the parameter is `Lowered`, so handing it a
    ///   [`LoweringOperand`] is `E0308` at every call site, present or future;
    /// - **cannot return** — [`RuntimeGroundValue`] is a closed **compile-time
    ///   constant** domain (bool / int / bytes / string, and aggregates of
    ///   itself). Transitively it has no arm able to hold a Cranelift SSA word;
    /// - **cannot mint or forward** — ⚠ `&mut self` *does* reach
    ///   [`Lowering::boundary_carrier`], so the refs are in scope. What closes
    ///   the mint is that this takes **no `FunctionBuilder`**: it cannot emit
    ///   CLIF, so it cannot run the one-way producer, and a `FuncRef` with
    ///   nowhere to emit is inert. Its recursion descends into `Vec<Lowered>`
    ///   child positions only.
    ///
    /// ⭐ **The edge's fail-closed disposition is FORCED, not chosen.** This
    /// materializes a *compile-time constant*; a `Carried` is by construction a
    /// runtime word with **no** compile-time value. So when the caller's
    /// scrutinee becomes a `LoweringOperand`, the `Carried` arm has no sound
    /// answer but `Err` — the return type's domain settles it, not a preference.
    fn ground_value(
        &mut self,
        value: Lowered,
    ) -> Result<RuntimeGroundValue, CraneliftBackendError> {
        match value {
            Lowered::Int {
                known: Some(value), ..
            } => Ok(RuntimeGroundValue::Int((value).into())),
            Lowered::Int { known: None, .. } => Err(unsupported(
                "Result",
                "native aggregate result contains a non-constant Int field",
            )),
            Lowered::Bool {
                known: Some(value), ..
            } => Ok(RuntimeGroundValue::Bool(value)),
            Lowered::Bool { known: None, .. } => Err(unsupported(
                "Result",
                "native aggregate result contains a non-constant Bool field",
            )),
            Lowered::ProcessExitStatus { .. } => Err(unsupported(
                "Result",
                "process exit status cannot escape a native process call",
            )),
            Lowered::Bytes(value) => Ok(RuntimeGroundValue::Bytes(value)),
            Lowered::BorrowedNativeValue { .. }
            | Lowered::BorrowedOption { .. }
            | Lowered::ResponseBytes { .. }
            | Lowered::CapabilityToken { .. }
            | Lowered::ResourceToken { .. }
            | Lowered::BoundedNat(_)
            | Lowered::StructuralNat(_)
            | Lowered::HostResult { .. }
            | Lowered::DynamicConstructor(_) => Err(unsupported(
                "Result",
                "borrowed ingress values cannot escape the native call",
            )),
            Lowered::String(value) => Ok(RuntimeGroundValue::String(value)),
            Lowered::Constructor {
                constructor, args, ..
            } => Ok(RuntimeGroundValue::Constructor {
                constructor,
                args: args
                    .into_iter()
                    .map(|arg| {
                        self.ground_value(
                            arg.into_specialized_at(
                                "a constructor field escaping to a ground value",
                            )?,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            }),
            Lowered::Record { fields, .. } => Ok(RuntimeGroundValue::Record {
                fields: fields
                    .into_iter()
                    .map(|field| Ok((field.name, self.ground_value(field.value)?)))
                    .collect::<Result<Vec<_>, CraneliftBackendError>>()?,
            }),
            Lowered::Closure { .. } | Lowered::DeclarationClosure { .. } => Err(unsupported(
                "Closure",
                "closures are callable but not observable ground values in native lowering",
            )),
            Lowered::ComputationalRecursorClosure { .. } => Err(unsupported(
                "ComputationalMatch",
                "recursive hypotheses are callable but not observable ground values",
            )),
            Lowered::RecursiveBackedge => Err(unsupported(
                "DeclarationRef",
                "a recursive CFG edge cannot escape as a ground value",
            )),
            Lowered::Trap(trap) => Err(unsupported(
                "Trap",
                format!("trap result must be reported as trapped: {}", trap.message),
            )),
        }
    }

    fn intern_result(&mut self, ground: RuntimeGroundValue) -> i64 {
        let token = self.next_token;
        self.next_token += 1;
        self.result_table.insert(token, ground);
        token
    }
}
/// ⛔ **A typed boundary: raw [`Lowered`] only, and STRUCTURALLY so**
/// (`RT-FNSPLIT-C1` frame `§2h` ¶2).
///
/// Closed **more tightly** than [`Lowering::ground_value`], and worth stating
/// because the two were priced as one class and are not: this is a **free
/// function with no `self`**, so it cannot reach
/// [`Lowering::boundary_carrier`] at all — there is no `FunctionBuilder`
/// argument *and* no receiver through which one could be found. Its parameter
/// is `&Lowered` (⇒ `E0308` on a [`LoweringOperand`]) and it returns
/// `&'static str` (⇒ no arm can hold a Cranelift SSA word). It is a
/// one-per-variant dispatch table over the specialization lattice and nothing
/// else, so there is no reachable surface on which to attempt an evasion.
fn lowered_value_kind(value: &Lowered) -> &'static str {
    match value {
        Lowered::Int { .. } => "Int",
        Lowered::Bool { .. } => "Bool",
        Lowered::ProcessExitStatus { .. } => "ProcessExitStatus",
        Lowered::CapabilityToken { .. } => "CapabilityToken",
        Lowered::ResourceToken { .. } => "ResourceToken",
        Lowered::BoundedNat(_) => "BoundedNat",
        Lowered::StructuralNat(_) => "StructuralNat",
        Lowered::ResponseBytes { .. } => "ResponseBytes",
        Lowered::HostResult { .. } => "HostResult",
        Lowered::DynamicConstructor(_) => "DynamicConstructor",
        Lowered::Bytes(_) => "Bytes",
        Lowered::BorrowedNativeValue { .. } => "BorrowedNativeValue",
        Lowered::BorrowedOption { .. } => "BorrowedOption",
        Lowered::String(_) => "String",
        Lowered::Constructor { .. } => "Constructor",
        Lowered::Record { .. } => "Record",
        Lowered::Closure { .. } => "Closure",
        Lowered::DeclarationClosure { .. } => "DeclarationClosure",
        Lowered::ComputationalRecursorClosure { .. } => "ComputationalRecursorClosure",
        Lowered::RecursiveBackedge => "RecursiveBackedge",
        Lowered::Trap(_) => "Trap",
    }
}
fn expect_two_args(
    symbol: &'static str,
    args: Vec<Lowered>,
) -> Result<(Lowered, Lowered), CraneliftBackendError> {
    let [lhs, rhs]: [Lowered; 2] = args.try_into().map_err(|args: Vec<Lowered>| {
        unsupported(
            "PrimitiveCall",
            format!("{symbol} expects 2 args, got {}", args.len()),
        )
    })?;
    Ok((lhs, rhs))
}
fn borrowed_constructor_identity(
    symbols: &crate::NativeProcessSymbols,
    symbol: &str,
) -> Option<(i64, usize)> {
    if symbol == symbols.process_input {
        Some((1, 3))
    } else if symbol == symbols.list_nil {
        Some((2, 0))
    } else if symbol == symbols.list_cons {
        Some((3, 2))
    } else if symbol == symbols.prod {
        Some((4, 2))
    } else {
        None
    }
}

#[cfg(test)]
thread_local! {
    static PX8J_SOURCE_TRACE: std::cell::RefCell<Vec<Px8jSourceTraceEvent>> =
        const { std::cell::RefCell::new(Vec::new()) };
    static PX8J_DELETE_OWNED_SELECTED_SCOPE: std::cell::Cell<bool> =
        const { std::cell::Cell::new(false) };
    pub(super) static PX8TR_TRAP_PROVENANCE: std::cell::RefCell<Vec<Px8trTrapProvenanceEvent>> =
        const { std::cell::RefCell::new(Vec::new()) };
    pub(super) static PX8TR_DISABLE_DEFORESTED_ANSWER_ROUTE: std::cell::Cell<bool> =
        const { std::cell::Cell::new(false) };
}
#[cfg(test)]
thread_local! {
    pub(crate) static NATIVE_INT_LOWERING_MUTATION: std::cell::Cell<NativeIntLoweringMutation> =
        const { std::cell::Cell::new(NativeIntLoweringMutation::Exact) };
}
#[cfg(any(test, feature = "px8-ds-test-support"))]
thread_local! {
    static PX8DS_RETIRED_FLAT_ORDER: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}


/// `RT-LEXICAL-RECURSOR-CONSUMERS` `D2b` — OBSERVATION ONLY.
///
/// ⛔ Recorders, never deciders. Each is written at a seam and read by a
/// control; none removes a join, chooses a disposition, or affects a result.
#[cfg(test)]
thread_local! {
    /// `(required, consumed, dispositioned)` per closeout, in call order.
    ///
    /// ⛔ A `Vec`, not a slot. The guard runs ONCE PER FUNCTION, so a slot
    /// records the last close and silently discards the one a control is
    /// asking about -- which reads as "nothing was dispositioned".
    static LRC_D2B_JOIN_OBSERVATION: std::cell::RefCell<
        Vec<(BTreeSet<StaticOriginId>, BTreeSet<StaticOriginId>, BTreeSet<StaticOriginId>)>,
    > = const { std::cell::RefCell::new(Vec::new()) };
    /// Every source occurrence entered on this thread.
    static LRC_D2B_ENTERED: std::cell::RefCell<BTreeSet<StaticOriginId>> =
        const { std::cell::RefCell::new(BTreeSet::new()) };
    /// Every origin a static-worker call was emitted for.
    static LRC_D2B_WORKER_CALLS: std::cell::RefCell<BTreeSet<StaticOriginId>> =
        const { std::cell::RefCell::new(BTreeSet::new()) };
    /// Arm-local: every `LetBody` arrival, as `(body.static_origin, backedge?)`.
    ///
    /// ⛔ This is how a control names the body occurrence WITHOUT a numeric
    /// origin: the arm reports its own `body.static_origin`, which is the
    /// planner's, so the control asserts a relation between what the arm saw
    /// and what the traversal entered.
    static LRC_D2B_LET_ARRIVALS: std::cell::RefCell<Vec<(StaticOriginId, bool)>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn lrc_d2b_reset_observation() {
    LRC_D2B_JOIN_OBSERVATION.with(|cell| cell.borrow_mut().clear());
    LRC_D2B_ENTERED.with(|cell| cell.borrow_mut().clear());
    LRC_D2B_WORKER_CALLS.with(|cell| cell.borrow_mut().clear());
    LRC_D2B_LET_ARRIVALS.with(|cell| cell.borrow_mut().clear());
}

/// Every closeout's three sets, in call order.
///
/// ⛔ An empty `Vec` and a `Vec` of empty sets are deliberately distinct: "no
/// closeout ran" and "one ran and saw nothing" are readings a control must not
/// conflate.
#[cfg(test)]
pub(in crate::cranelift_backend) fn lrc_d2b_join_observation(
) -> Vec<(BTreeSet<StaticOriginId>, BTreeSet<StaticOriginId>, BTreeSet<StaticOriginId>)> {
    LRC_D2B_JOIN_OBSERVATION.with(|cell| cell.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn lrc_d2b_record_let_arrival(
    body_origin: StaticOriginId,
    backedge: bool,
) {
    LRC_D2B_LET_ARRIVALS.with(|cell| cell.borrow_mut().push((body_origin, backedge)));
}

/// Every `LetBody` arrival on this thread, in order.
#[cfg(test)]
pub(in crate::cranelift_backend) fn lrc_d2b_let_arrivals() -> Vec<(StaticOriginId, bool)> {
    LRC_D2B_LET_ARRIVALS.with(|cell| cell.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn lrc_d2b_record_worker_call(origin: StaticOriginId) {
    LRC_D2B_WORKER_CALLS.with(|cell| {
        cell.borrow_mut().insert(origin);
    });
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn lrc_d2b_entered() -> BTreeSet<StaticOriginId> {
    LRC_D2B_ENTERED.with(|cell| cell.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn lrc_d2b_worker_calls() -> BTreeSet<StaticOriginId> {
    LRC_D2B_WORKER_CALLS.with(|cell| cell.borrow().clone())
}
