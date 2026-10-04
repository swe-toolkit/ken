//! Ken runtime — production content-addressed value store (K3).
//!
//! Implements `spec/40-runtime/41-values.md` + `44-capacity.md` and
//! `docs/design/content-addressing.md` at production resolution.
//!
//! Differences from the F4 design-validation crate (`ken-foundation`):
//! - NFC string normalization at construction time (`41 §3a`, design doc §1.4)
//! - Space-scoped arena separation + reclamation (`44 §3`)
//! - Arena page chaining beyond a single flat Vec (`44 §1b`)
//! - `unknown` propagation (Kleene/Heyting logic, `41 §6`)

// Preserve every consumer's module path while the run-time implementation
// lives in the compiler-independent support crate.
pub use ken_runtime_support::{
    activation_abi, activation_services, artifact_validation, boundary_activation,
    boundary_resource_profile, boundary_value, canonical, hash, invocation_tickets, ir,
    native_int, store, values,
};
/// `RT-FNSPLIT-B2V` — the emitted-code half of the boundary-value ABI.
///
/// Private, exactly like `native_int_clif`: the CLIF graph is compiler-internal
/// and is reached through [`boundary_value`]'s published layout constants.
mod boundary_value_clif;
mod boundary_emission_plan;
pub mod cranelift_backend;
pub mod executable_artifact_contract;
pub mod executable_entrypoint_packaging;
#[cfg(test)]
mod native_effect_v1;
pub mod native_execution_differential;
mod native_int_clif;

#[doc(hidden)]
pub mod native_join_plan;
pub mod native_process_authority;
pub mod native_process_entrypoint;
pub mod object_linker_packaging;
#[doc(hidden)]
pub mod oriented_subcontinuation_plan;
pub mod platform_runtime_support;
pub mod runtime_ir_evaluator;
pub mod target_abi;
pub mod unknown;

pub use ken_runtime_support::{
    activation_abi::*, activation_services::*, artifact_validation::*, boundary_activation::*,
    boundary_resource_profile::*, canonical::Canonical,
};
pub use cranelift_backend::*;
pub use executable_artifact_contract::*;
pub use executable_entrypoint_packaging::*;
pub use ken_runtime_support::{hash::fnv1a_64, ir::*};
pub use ken_host::{
    CanonicalOutcomeV1, CanonicalReplyV1, CanonicalRequestV1, CapacityExhaustedV1,
    CapacityResourceV1, CapacityScopeV1, ConsoleStreamV1, EffectEvent, EffectObservation,
    EffectiveUidSnapshotV1, FsDeltaV1, FsNodeKindV1, FsNodeObservationV1, HOST_EFFECT_ABI_V1_HASH,
    HomeRootResolutionFailureV1, HostOpV1, IoErrorIdentityV1, LinkedEffectTrace, ReadProgressV1,
    ResourceBindingRole, ResourceErrorV1, RootExecutionDeniedV1, RuntimeTrapProvenanceV1,
    SemanticErrorV1, TARGET_ABI_MANIFEST_HASH, TerminalErrorV1, TerminalExitClass, TransferCountV1,
    WriteProgressV1, admit_root_execution, decode_linked_effect_trace, encode_linked_effect_trace,
    observe_effective_uid_v1,
};
pub use native_execution_differential::*;
pub use ken_runtime_support::native_int::*;
#[doc(hidden)]
pub use native_join_plan::*;
pub use native_process_authority::*;
pub use native_process_entrypoint::*;
pub use object_linker_packaging::*;
#[doc(hidden)]
pub use oriented_subcontinuation_plan::*;
pub use platform_runtime_support::*;
pub use runtime_ir_evaluator::*;
pub use ken_runtime_support::store::{InternResult, Space, Store, StoreStats};
pub use target_abi::*;
pub use unknown::Unknown;
pub use ken_runtime_support::values::{Sign, Value};

// This cross-crate layout check needs the compiler's generated helper layout;
// unlike the run-time arithmetic tests it cannot live in runtime-support.
#[cfg(test)]
mod support_layout_tests {
    #[test]
    fn arena_header_matches_the_generated_local_helper_layout() {
        assert_eq!(
            std::mem::size_of::<crate::native_int::NativeIntArenaV1>(),
            crate::native_int_clif::ARENA_BYTES
        );
    }
}
