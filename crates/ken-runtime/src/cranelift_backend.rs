//! Cranelift backend for the runtime IR seed.
//!
//! This module deliberately keeps the native boundary narrow. Cranelift code
//! returns scalar `i64` values directly and aggregate observations through an
//! opaque token table decoded by this Rust layer. Native addresses, object
//! layout, allocation order, ABI details, and Cranelift internals never become
//! Ken-observable meaning.
//!
//! This file holds no test material — that lives in the two gated child
//! modules below. What remains is facade surface (module declarations and
//! re-exports) plus the two inherent `impl` blocks at the foot of the file,
//! which are production code on `surface`'s own types and are reached from
//! `artifact::api`.
//!
//! # Test-only helpers are not public production API
//!
//! `emit_process_entrypoint_object_with_cranelift` is a test-only helper, and
//! that it is not reachable from outside this crate is asserted by
//! `ken-cli/tests/px4b_native_production.rs`, in
//! `naked_process_ir_helpers_are_not_public_production_api`. That test asks
//! the *compiler* the question, by compiling probe snippets against this
//! crate's built rlib; a source-text assertion on the declaration would only
//! test its spelling and would break on every relocation.
//!
//! Both reachable paths are covered there. `lib.rs` re-exports this module
//! with a `pub use cranelift_backend::*`, so a widened item would surface at
//! the crate root as well as under the module path, and a check on one path
//! alone would miss the other:
//!
//! ```text
//! use ken_runtime::cranelift_backend::emit_process_entrypoint_object_with_cranelift as _;   // must not compile
//! use ken_runtime::emit_process_entrypoint_object_with_cranelift as _;                      // must not compile
//! use ken_runtime::cranelift_backend::emit_runtime_ir_object_with_cranelift as _;           // positive control
//! ```
//!
//! These fences are `text` on purpose: they remain illustrations and do not
//! appear in CI's workspace doctest population. The executable form lives in
//! the test named above, which runs in the sharded lane.

use std::collections::{BTreeMap, BTreeSet};

use crate::RuntimeProgram;

pub(crate) mod artifact;
pub(crate) mod compiled;
mod lowering;
pub(crate) mod planning;
pub(crate) mod surface;

#[cfg(test)]
mod test_objects;
#[cfg(test)]
mod test_support;

// The facade preserves the exact pre-existing `ken_runtime::<name>` surface.
// Re-exporting an already-exported name at its established visibility does not
// widen it.
pub(crate) use artifact::api::{
    emit_bound_process_program_object_with_cranelift, emit_runtime_ir_object_with_authority,
    run_process_expr_with_cranelift, run_runtime_ir_report_with_authority,
};
// `RT-DYNAMIC-ARM-SCALAR-MERGE` `D1b-role-c1` — the synthetic compilation
// entrypoints. ⛔ `#[cfg(test)]`, so these names do not exist in a production
// build and no production path can reach lowering through them.
pub use artifact::api::{
    emit_runtime_ir_object_with_cranelift, reject_program_blockers,
    run_example_with_interpreter_observation, run_example_with_seed_observation,
    run_ken_checked_proof_erasure_example_with_interpreter_observation, run_nc6_seed_examples,
    run_nc8_validated_seed_examples, run_runtime_ir_report_with_cranelift,
    run_validated_example_with_interpreter_observation,
};
#[cfg(test)]
pub(crate) use artifact::api::{
    emit_synthetic_runtime_ir_object_with_cranelift, run_synthetic_runtime_ir_report_with_cranelift,
};

// `with_px8ds_retired_flat_order` is bare `pub` and reached cross-CRATE as
// `ken_runtime::with_px8ds_retired_flat_order` through `lib.rs:39`. Moving it
// into the private `lowering` module severs that path, and neither
// `-p ken-runtime` build config can observe the break — only the consumer can.
#[cfg(feature = "px8-ds-test-support")]
pub use lowering::with_px8ds_retired_flat_order;

// `RT-MATCH-RECURSOR-CONSUMERS` 4a: the cross-crate census surface, reached as
// `ken_runtime::{with_match_recursor_census, MatchRecursorCensusRow}` through
// `lib.rs`. Same reachability caveat as the item above -- only a consumer can
// observe a break in this path, so the `ken-cli`/`ken-verify`/`ken-elaborator`
// census suites are what keep it honest.
// `RT-CAPTURE-PROJECTION-GROW` `D1` — the worker-prefix deferral ledger.
#[cfg(feature = "px8-ds-test-support")]
pub use planning::{with_worker_prefix_deferrals, WorkerPrefixDeferral};

#[cfg(feature = "px8-ds-test-support")]
pub use lowering::core::{
    with_branched_scrutinee_unit_body_match_branch_entry_suppressed,
    with_branched_scrutinee_unit_body_route1, with_match_recursor_census,
    BranchedScrutineeUnitBodyRoute1, MatchRecursorCensusRow,
};

// `RT-4B-OBSERVATION-FEATURE-GATE`: the existing D2f observer, reachable by
// governed cross-crate controls only when its isolated default-off feature is
// enabled. These items are doc-hidden and explicitly unsupported as production
// API at their declarations.
#[cfg(feature = "r3-4b-observation")]
pub use lowering::core::{
    d2f_gate_observation_scope, D2fGateArrival, D2fGateObservationScope,
};

// `RT-DYNAMIC-ARM-SCALAR-MERGE` `c2`: the real D5 package control's isolated,
// default-off view of general scalar-merge decisions. The declarations are
// doc-hidden and explicitly unsupported as production API.
//
// Keep this compiled-feature fact adjacent to the feature-gated re-export it
// mirrors: one `ken-runtime` feature resolution feeds both predicates here.
// Moving either into another crate would reopen cross-crate feature-resolution
// drift.
/// Whether this `ken-runtime` build includes the D5 observation entry point.
///
/// This fact stays available in both configurations so cross-crate controls
/// can detect dependency feature unification without widening the gated
/// observation facade.
#[doc(hidden)]
pub const DASM_C2_OBSERVATION_COMPILED: bool = cfg!(feature = "dasm-c2-observation");

#[cfg(feature = "dasm-c2-observation")]
pub use lowering::joins::{
    dasm_c2_scalar_merge_observation_scope, DasmC2ScalarMergeObservation,
    DasmC2ScalarMergeObservationScope,
};

// `RT-MATCH-RECURSOR-CONSUMERS` 4a.1: the child-process transport of that same
// recorder, and the one item in this pair that is deliberately NOT gated.
//
// The gate is on this crate's feature and the call site is in `ken-cli`'s
// binary, which has no feature of its own to test -- it receives
// `px8-ds-test-support` only through a `[dev-dependencies]` edge, which enables
// the feature on this crate's unit without defining any `cfg` visible to
// `ken-cli`'s sources. A gated re-export would leave the call site unwritable in
// the default build. The behaviour, not the item, is what the feature gates:
// with it off the function is a direct call to its argument.
pub use lowering::core::with_child_match_recursor_census;

// ⛔ This list is DERIVED, not authored: it is exactly the set of module-level
// bare-`pub` items in `surface.rs`, enumerated mechanically and checked for set
// equality in BOTH directions. A name dropped here vanishes from
// `ken_runtime::*` via `lib.rs:39` and can still compile green across the whole
// workspace — most of these names have no in-repo consumer at all, so the
// compiler is not a net for this edit. `NativeSeedEnvironment::{empty,
// nc5_seed, insert}` are impl METHODS, not module items, and correctly do not
// appear.
//
// `backend_module` is listed separately because a glob re-exports
// restricted-visibility items too: it is `pub(crate)` in `surface`, so
// enumerating only bare-`pub` declarations silently drops it.
// `native_int_clif.rs:14`, a SIBLING of this module, imports it as
// `crate::cranelift_backend::backend_module` — the compiler caught that one
// only because it happens to have an in-crate consumer, which is luck, not a
// net.
pub(crate) use surface::backend_module;
pub use surface::{
    BackendFailure, CraneliftBackendError, CraneliftObjectArtifact, CraneliftRunReport,
    InterpreterOracleObservation, NativeArtifactIdentity, NativeDifferentialReport,
    NativeDifferentialStage, NativeDifferentialVerdict, NativeEvidenceFact, NativeFidelity,
    NativeRunEvidence, NativeRuntimeIrComparisonReport, NativeRuntimeIrComparisonVerdict,
    NativeSeedEnvironment, NativeToolchainReport, NativeTrustReport, UnsupportedLowering,
    ValidatedNativeRunError,
};

// Test-only facade surface. These re-exports carry names to consumers OUTSIDE
// this module — `object_linker_packaging`, a crate-root sibling, reaches every
// one of them — so they are surface, not test content, and they cannot move
// into the gated child modules that declare them.
//
// ⛔ The `#[cfg(test)]` gate is deliberate, not an artifact to clean up. The
// build asymmetry here is bidirectional: an UNGATED `use` warns as unused in
// the library build, and deleting it breaks the test build. Neither
// configuration's diagnostics are authority on their own, so the decision is
// taken over the INTERSECTION of both — gate rather than choose a side.
#[cfg(test)]
pub(crate) use lowering::{
    scale_b_record_boundary_value, scale_b_record_native_int, CarriedComputationalLoopEdge,
    NativeIntLoweringMutation, PlannedTrapSeat, Px8trTrapProvenanceEvent,
    NATIVE_INT_LOWERING_MUTATION,
};
#[cfg(test)]
pub(crate) use test_objects::{
    emit_process_entrypoint_object_with_cranelift, emit_px8tr_nested_post_effect_object,
};
// `Px8trNestedRouteObject` was nameable at `crate::cranelift_backend::
// Px8trNestedRouteObject` before the move. Spelling the declaration
// `pub(crate)` at its new home does NOT preserve that reach, because the module
// holding it is private — the type surface would narrow even though the
// visibility keyword is unchanged. This re-export preserves the pre-existing
// path. It is kept separate from the block above so the `allow` covers only
// this one intentionally path-preserving import: the absence of a named
// consumer today is evidence that nothing currently reaches it, not authority
// to remove the way anything could.
#[cfg(test)]
#[allow(unused_imports)] // Preserve the pre-existing nameable crate-private type path.
pub(crate) use test_objects::Px8trNestedRouteObject;

impl NativeArtifactIdentity {
    fn from_program(program: &RuntimeProgram) -> Self {
        Self {
            package_identity: program.package_identity.clone(),
            core_semantic_hash: program.core_semantic_hash,
            runtime_artifact_hash: program.artifact_hash,
        }
    }
}

impl NativeRunEvidence {
    fn seed_example() -> Self {
        let mut evidence = Self::default();
        evidence.unavailable.insert(
            "package/core/runtime artifact identity unavailable for standalone seed example"
                .to_string(),
        );
        evidence.evidence_sources.insert(
            "backend".to_string(),
            "compiled Cranelift JIT run".to_string(),
        );
        evidence
    }

    fn from_program(program: &RuntimeProgram) -> Self {
        let mut evidence = Self {
            package_identity: Some(program.package_identity.clone()),
            core_semantic_hash: Some(program.core_semantic_hash),
            runtime_artifact_hash: Some(program.artifact_hash),
            evidence_sources: BTreeMap::new(),
            unavailable: BTreeSet::new(),
        };
        evidence.evidence_sources.insert(
            "package_identity".to_string(),
            "RuntimeProgram.package_identity from the exact runtime artifact".to_string(),
        );
        evidence.evidence_sources.insert(
            "core_semantic_hash".to_string(),
            "RuntimeProgram.core_semantic_hash from the exact runtime artifact".to_string(),
        );
        evidence.evidence_sources.insert(
            "runtime_artifact_hash".to_string(),
            "RuntimeProgram.artifact_hash from the exact runtime artifact".to_string(),
        );
        evidence.evidence_sources.insert(
            "backend".to_string(),
            "compiled Cranelift JIT run".to_string(),
        );
        evidence
    }
}
