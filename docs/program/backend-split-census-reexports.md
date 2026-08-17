# Backend split census: re-export surface

Measurement SHA: `4de48651434dd6340f81ec9b1b7a5ac2ec8c0199`

This inventory records every explicit Rust re-export statement in the backend
facade and measured backend scope. Library and test profiles are listed
independently rather than collapsed into one surface.

## Boundary and selector

The domain is `crates/ken-runtime/src/cranelift_backend.rs`, every Rust
file below `crates/ken-runtime/src/cranelift_backend/`, and
`crates/ken-runtime/src/boundary_value_clif.rs`. The statement selector is:

```text
^\s*pub(?:\([^)]*\))?\s+use\b
```

It selects 57 explicit statements. It cannot see glob exports
introduced by macros, inherent public items, module declarations, or the
crate-root `pub use cranelift_backend::*` edge; that root edge is recorded
separately below. Profile classification reads the nearest governing
`cfg` attribute and cannot model a feature enabled indirectly by Cargo
feature unification.

## Profile totals

| profile | statements | visibility totals |
|---|---:|---|
| default library and test | 29 | `pub` 3, `pub(crate)` 2, `pub(in crate::cranelift_backend)` 17, `pub(super)` 7 |
| named feature only | 4 | `pub` 4 |
| test default; library with named feature | 2 | `pub(in crate::cranelift_backend)` 2 |
| test only | 22 | `pub(crate)` 4, `pub(in crate::cranelift_backend)` 13, `pub(super)` 5 |

The default library build sees only `default library and test` rows.
The default test build sees those rows plus `test only` and the test leg
of `test default; library with named feature`. Feature-only rows exist
in a library or test build only when their named feature is enabled.

The crate root also contains `pub use cranelift_backend::*` at
`crates/ken-runtime/src/lib.rs:65`. It exposes the facade's bare-public rows at
`ken_runtime::<name>` in every profile where the underlying facade row exists;
it does not widen restricted rows.

## Statement ledger

### default library and test

- `cranelift_backend.rs:58`; `pub(crate)`.

  ```rust
  pub(crate) use artifact::api::{ emit_bound_process_program_object_with_cranelift, emit_runtime_ir_object_with_authority, run_process_expr_with_cranelift, run_runtime_ir_report_with_authority, };
  ```

- `cranelift_backend.rs:65`; `pub`.

  ```rust
  pub use artifact::api::{ emit_runtime_ir_object_with_cranelift, reject_program_blockers, run_example_with_interpreter_observation, run_example_with_seed_observation, run_ken_checked_proof_erasure_example_with_interpreter_observation, run_nc6_seed_examples, run_nc8_validated_seed_examples, run_runtime_ir_report_with_cranelift, run_validated_example_with_interpreter_observation, };
  ```

- `cranelift_backend.rs:128`; `pub`.

  ```rust
  pub use lowering::core::with_child_match_recursor_census;
  ```

- `cranelift_backend.rs:146`; `pub(crate)`.

  ```rust
  pub(crate) use surface::backend_module;
  ```

- `cranelift_backend.rs:147`; `pub`.

  ```rust
  pub use surface::{ BackendFailure, CraneliftBackendError, CraneliftObjectArtifact, CraneliftRunReport, InterpreterOracleObservation, NativeArtifactIdentity, NativeDifferentialReport, NativeDifferentialStage, NativeDifferentialVerdict, NativeEvidenceFact, NativeFidelity, NativeRunEvidence, NativeRuntimeIrComparisonReport, NativeRuntimeIrComparisonVerdict, NativeSeedEnvironment, NativeToolchainReport, NativeTrustReport, UnsupportedLowering, ValidatedNativeRunError, };
  ```

- `cranelift_backend/lowering/core.rs:9`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use super::*;
  ```

- `cranelift_backend/lowering/mod.rs:36`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use std::collections::{BTreeMap, BTreeSet};
  ```

- `cranelift_backend/lowering/mod.rs:40`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use crate::boundary_value::{ BoundaryClass, BoundaryReferentOwner, BoundaryTag, BOUNDARY_ERR_BOUNDS, BOUNDARY_OK, };
  ```

- `cranelift_backend/lowering/mod.rs:44`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use cranelift_codegen::flowgraph::ControlFlowGraph;
  ```

- `cranelift_backend/lowering/mod.rs:45`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use cranelift_codegen::ir::{ types, AbiParam, Block, FuncRef, Function, InstBuilder, MemFlags, StackSlotData, StackSlotKind, UserFuncName, };
  ```

- `cranelift_backend/lowering/mod.rs:49`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use cranelift_codegen::verify_function;
  ```

- `cranelift_backend/lowering/mod.rs:50`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use cranelift_frontend::{ FunctionBuilder, FunctionBuilderContext, };
  ```

- `cranelift_backend/lowering/mod.rs:53`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use cranelift_module::{FuncId, Linkage, Module};
  ```

- `cranelift_backend/lowering/mod.rs:55`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use safe_byte_span::SafeByteSpan;
  ```

- `cranelift_backend/lowering/mod.rs:58`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use crate::{ RuntimeDeclaration, RuntimeDeclarationKind, RuntimeExpr, RuntimeGroundValue, RuntimePartiality, RuntimePrimitive, RuntimeSymbol, RuntimeTrap, RuntimeTrapCode, RuntimeValue, };
  ```

- `cranelift_backend/lowering/mod.rs:74`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use super::compiled::{CompiledModule, ResultDecoder};
  ```

- `cranelift_backend/lowering/mod.rs:79`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use super::planning::{ collect_checked_oriented_markers, collect_checked_subcontinuation_frames, build_static_continuation_fusion_plan, plan_static_transition_graph_with_symbols, FusionCompositionLayer, FusionRegionClaim, FusionRegionClaimLedger, StaticContinuationFusionId, StaticContinuationFusionView, validate_oriented_subcontinuation_transport, AbiCaptureProvenance, AbiCarrier, AbiFrameHeader, AbiOwnership, AbiProcessParameter, AbiRootIngress, AbiSlot, AbiSlotKind, AbiStorageOwner, AbiUnitDefinition, expected_capture_slot, // `RT-LEXICAL-RECURSOR-CONSUMERS` `D2e` — the checked binder layout, now // reaching PRODUCTION rather than only lowering's test targets: the composed // eliminator checks its assembled run against it. ⛔ Ungated here and in // `planning.rs`, because a `cfg(test)` re-export of an item production reads // is an unresolved import the test profile cannot see. CheckedCaseBinderLayout, CheckedCaseBinderRole, CheckedOrientedMarkerSets, ConstructorIdentity, ContinuationCallIdentity, ContinuationCallView, DeclarationCallTargetClass, ContinuationContextId, ContinuationEmissionOwner, ContinuationInputView, ContinuationOrdinaryEnvelopeRole, ContinuationResultEdge, ContinuationAvailabilityViews, ContinuationEnvironmentClaim, ContinuationFrameIdentity, ContinuationSourceCoordinate, ContinuationSourceSlotAuthority, ContinuationSpecializationId, ContinuationUnitView, RequiredConsumerProjection, EmittableCallKind, FieldIdentity, JoinPlanToken, CaseEmissionStatus, PlannedReferentLifetime, host_effect_seat_contract_of, EffectSeatNeed, EffectSeatOperation, EffectSeatPhase, EffectSeatSlot, PlannedEffectSeat, AggregateOccurrenceId, PlannedAggregateAllocation, PlannedAggregateShape, SynthesizedAggregateNode, SynthesizedAggregatePath, SynthesizedAggregateRoot, PlannedAggregateOwnership, JoinResultRepresentation, PredeclaredFunctionId, StaticOriginId, StaticTransitionPlan, verify_current_lexical_availability, verify_predeclared_entry_frame_membership, SynthesizedConstructorRole, SynthesizedFixedConstructorRole, };
  ```

- `cranelift_backend/lowering/mod.rs:117`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use super::surface::{ backend, backend_module, unsupported, BackendFailure, CraneliftBackendError, NativeSeedEnvironment, };
  ```

- `cranelift_backend/planning/static_transition.rs:39`;
  `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use abi::{ AbiCaptureProvenance, AbiCarrier, AbiFrameHeader, AbiOwnership, AbiProcessParameter, AbiRootIngress, AbiSchedulingIngress, AbiSlot, AbiSlotKind, AbiStorageOwner, AbiUnitDefinition, expected_capture_slot, };
  ```

- `cranelift_backend/planning/static_transition.rs:50`;
  `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use semantic_ir::{ ConstructorIdentity, FieldIdentity, PredeclaredFunctionId, StaticOriginId, SynthesizedConstructorRole, SynthesizedFixedConstructorRole, SynthesizedIoErrorRole, };
  ```

- `cranelift_backend/planning.rs:16`; `pub(super)`.

  ```rust
  pub(super) use static_transition::build_static_continuation_fusion_plan;
  ```

- `cranelift_backend/planning.rs:17`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use static_transition::{ FusionComposedEdge, FusionCompositionLayer, FusionOwnedOuterRealization, FusionRegionClaim, FusionRegionClaimLedger, };
  ```

- `cranelift_backend/planning.rs:30`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use static_transition::{ StaticContinuationFusionId, StaticContinuationFusionView, };
  ```

- `cranelift_backend/planning.rs:47`; `pub(super)`.

  ```rust
  pub(super) use static_transition::plan_static_transition_graph_with_symbols;
  ```

- `cranelift_backend/planning.rs:54`; `pub(super)`.

  ```rust
  pub(super) use static_transition::{ planned_partiality_trap, CaseEmissionStatus, ConstructorIdentity, DeclarationCallTargetClass, JoinPlanToken, JoinResultRepresentation, StaticOriginId, StaticTransitionPlan, SynthesizedConstructorRole, SynthesizedFixedConstructorRole, };
  ```

- `cranelift_backend/planning.rs:64`; `pub(super)`.

  ```rust
  pub(super) use static_transition::{ AggregateOccurrenceId, FieldIdentity, PlannedAggregateAllocation, PlannedAggregateShape, PlannedAggregateOwnership, PlannedReferentLifetime, SynthesizedAggregateNode, SynthesizedAggregatePath, SynthesizedAggregateRoot, };
  ```

- `cranelift_backend/planning.rs:71`; `pub(super)`.

  ```rust
  pub(super) use static_transition::{ host_effect_seat_contract_of, EffectSeatNeed, EffectSeatOperation, EffectSeatPhase, EffectSeatSlot, PlannedEffectSeat, CRANELIFT_HOST_EFFECT_CONSUMERS_V1, };
  ```

- `cranelift_backend/planning.rs:77`; `pub(super)`.

  ```rust
  pub(super) use static_transition::{ ContinuationCallIdentity, ContinuationCallView, ContinuationContextId, ContinuationConsumingOccurrence, ContinuationEmissionOwner, ContinuationInputView, RequiredConsumerProjection, ContinuationOrdinaryEnvelopeRole, ContinuationResultEdge, // `RT-CONTSRC-PRODUCER-LOCAL` `D1` — the closed coordinate sum reaches // lowering because the emission resolver must MATCH on it; there is no // accessor that answers "which ABI position" without the domain. ContinuationAvailabilityViews, ContinuationEnvironmentClaim, ContinuationFrameIdentity, ContinuationSourceCoordinate, ContinuationSourceSlotAuthority, ContinuationSpecializationId, // `RT-LEXICAL-RECURSOR-CONSUMERS` `D2e` — the checked binder layout reaches // lowering's test targets so its control can compare the authority against // the prefix production actually assembled, rather than against its own // recomputation of the same rule. Namespace re-export only. CheckedCaseBinderLayout, CheckedCaseBinderRole, // `RT-CONTSRC-PRODUCER-LOCAL` `D3b` — the emission consumer's fail-closed // check that it is indexing where the coordinate actually sits. verify_current_lexical_availability, verify_predeclared_entry_frame_membership, ContinuationUnitView, // `RT-CONTSRC-PRODUCER-LOCAL` `D7a` — the planner-issued composed worker // view, reached from a computational frame's own coordinates. Namespace // re-export only: no wrapper, no derivation, no second authority. ComposedWorkerRouteEligibility, };
  ```

- `cranelift_backend/planning.rs:141`; `pub(super)`.

  ```rust
  pub(super) use static_transition::{ AbiCaptureProvenance, AbiCarrier, AbiFrameHeader, AbiOwnership, AbiProcessParameter, AbiRootIngress, AbiSlot, AbiSlotKind, AbiStorageOwner, AbiUnitDefinition, expected_capture_slot, EmittableCallKind, PredeclaredFunctionId, };
  ```

### named feature only

- `cranelift_backend.rs:82`; `pub`.

  ```rust
  pub use lowering::with_px8ds_retired_flat_order;
  ```

- `cranelift_backend.rs:90`; `pub`.

  ```rust
  pub use lowering::core::{with_match_recursor_census, MatchRecursorCensusRow};
  ```

- `cranelift_backend.rs:97`; `pub`.

  ```rust
  pub use lowering::core::{ d2f_gate_observation_scope, D2fGateArrival, D2fGateObservationScope, };
  ```

- `cranelift_backend.rs:113`; `pub`.

  ```rust
  pub use lowering::{ dasm_c2_scalar_merge_observation_scope, DasmC2ScalarMergeObservation, DasmC2ScalarMergeObservationScope, };
  ```

### test default; library with named feature

- `cranelift_backend/lowering/mod.rs:76`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use super::planning::{ StaticContinuationFusionDescriptor, StaticContinuationFusionKey, StaticContinuationFusionPlan, };
  ```

- `cranelift_backend/planning.rs:36`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use static_transition::{ StaticContinuationFusionDescriptor, StaticContinuationFusionKey, StaticContinuationFusionPlan, };
  ```

### test only

- `cranelift_backend.rs:73`; `pub(crate)`.

  ```rust
  pub(crate) use artifact::api::{ emit_synthetic_runtime_ir_object_with_cranelift, run_synthetic_runtime_ir_report_with_cranelift, };
  ```

- `cranelift_backend.rs:167`; `pub(crate)`.

  ```rust
  pub(crate) use lowering::{ scale_b_record_boundary_value, scale_b_record_native_int, NativeIntLoweringMutation, PlannedTrapSeat, Px8trTrapProvenanceEvent, NATIVE_INT_LOWERING_MUTATION, };
  ```

- `cranelift_backend.rs:172`; `pub(crate)`.

  ```rust
  pub(crate) use test_objects::{ emit_process_entrypoint_object_with_cranelift, emit_px8tr_nested_post_effect_object, };
  ```

- `cranelift_backend.rs:186`; `pub(crate)`.

  ```rust
  pub(crate) use test_objects::Px8trNestedRouteObject;
  ```

- `cranelift_backend/lowering/core/tests/mod.rs:9`;
  `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use super::*;
  ```

- `cranelift_backend/lowering/core/tests/mod.rs:30`;
  `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use super::super::super::{ emit_process_entrypoint_object_with_cranelift, run_example_with_seed_observation, };
  ```

- `cranelift_backend/lowering/core/tests/mod.rs:35`;
  `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use crate::{ CraneliftObjectArtifact, NativeFidelity, RuntimeExample, RuntimeLowerabilityStatus, RuntimeObservation, UnsupportedLowering, };
  ```

- `cranelift_backend/lowering/core/tests/mod.rs:41`;
  `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use crate::cranelift_backend::test_support::{ test_only_distinguished_root_join_plan, total_primitive, };
  ```

- `cranelift_backend/lowering/core/tests/mod.rs:53`;
  `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use crate::cranelift_backend::artifact::{ compile_expr_for_lowering_tests as compile_expr, new_jit_module_for_lowering_tests as new_jit_module, new_object_module_for_lowering_tests as new_object_module, };
  ```

- `cranelift_backend/lowering/mod.rs:114`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use super::planning::{ plan_static_transition_graph, with_last_io_error_role_omitted, ScaleBPlanCensus, };
  ```

- `cranelift_backend/lowering/mod.rs:125`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use crate::RuntimeMatchCase;
  ```

- `cranelift_backend/planning/static_transition.rs:45`;
  `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use semantic_ir::with_last_io_error_role_omitted;
  ```

- `cranelift_backend/planning/static_transition.rs:47`;
  `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use semantic_ir::{ with_d2a_population_mutation, D2aPopulationMutation, };
  ```

- `cranelift_backend/planning/static_transition.rs:18371`;
  `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use tests::contspec_nested_fixture;
  ```

- `cranelift_backend/planning/static_transition.rs:18376`;
  `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use tests::{ d2j_checked_fixture_under, d2j_installed_plan_under, D2jCause, D2J_DECLARATION, };
  ```

- `cranelift_backend/planning.rs:41`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use static_transition::{ d2j_checked_fixture_under, d2j_installed_plan_under, r3_fusion_claim_consumptions, reset_r3_fusion_claim_consumptions, with_fusion_claim_parameter_mutation, with_fusion_producer_capture_mutation, D2jCause, FusionClaimParameterMutation, FusionProducerCaptureMutation, D2J_DECLARATION, };
  ```

- `cranelift_backend/planning.rs:49`; `pub(super)`.

  ```rust
  pub(super) use static_transition::{ contspec_nested_fixture, governed_nested_resource_bracket, plan_static_transition_graph, take_continuation_required_consumer_observations, PlannedResultFieldKindForTest, RequiredConsumerProjectionDisposition, ScaleBPlanCensus, };
  ```

- `cranelift_backend/planning.rs:76`; `pub(super)`.

  ```rust
  pub(super) use static_transition::{set_effect_seat_plan_mutation, EffectSeatPlanMutation};
  ```

- `cranelift_backend/planning.rs:109`; `pub(super)`.

  ```rust
  pub(super) use static_transition::{ set_composed_call_target_defect, set_continuation_descent_owner_duplication, set_envelope_defect, EnvelopeDefect, with_continuation_consuming_eliminator_seed_mutated, with_continuation_consuming_occurrence_seed_mutated, with_required_consumer_projection_mutation, ComposedCallTargetDefect, RequiredConsumerProjectionMutation, };
  ```

- `cranelift_backend/planning.rs:122`; `pub(in crate::cranelift_backend)`.

  ```rust
  pub(in crate::cranelift_backend) use static_transition::{ d3b_publish_without_finalization, d3b_refinalize, D3bFinalizationPerturbation, };
  ```

- `cranelift_backend/planning.rs:126`; `pub(super)`.

  ```rust
  pub(super) use static_transition::with_last_io_error_role_omitted;
  ```

- `cranelift_backend/planning.rs:133`; `pub(super)`.

  ```rust
  pub(super) use static_transition::{ ac4_note_route_invocation, ac4_open_route_window, ac4_route_counts, };
  ```
