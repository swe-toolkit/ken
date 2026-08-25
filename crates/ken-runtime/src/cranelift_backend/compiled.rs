//! The compiled container: `CompiledModule`, its JIT specialization
//! `CompiledExpr`, the result decoder, result-table ownership, and JIT
//! execution with result decoding.
//!
//! RT-SPLIT slice 3 of 7. Pure move out of the flat `cranelift_backend`
//! module. This module does NOT own compilation policy -- it owns the
//! artifact of compilation and how its result is read back. Depends only on
//! `surface`.

use std::collections::{BTreeMap, BTreeSet};
use std::mem;

use cranelift_jit::JITModule;
use cranelift_module::FuncId;

use super::surface::{backend, backend_module, BackendFailure, CraneliftBackendError};
use crate::{RuntimeGroundValue, RuntimeObservation, RuntimeTrap};

pub(super) struct CompiledModule<M> {
    pub(super) module: M,
    func_id: FuncId,
    decoder: Option<ResultDecoder>,
    result_table: BTreeMap<i64, RuntimeGroundValue>,
    trap: Option<RuntimeTrap>,
    trap_catalog: Vec<RuntimeTrap>,
    carrier_identity_catalog: Vec<(String, u64)>,
    pub(super) verifier_passed: bool,
    pub(super) assumptions: BTreeSet<String>,
    pub(super) unsupported: Vec<String>,
}

pub(super) type CompiledExpr = CompiledModule<JITModule>;

#[derive(Clone, Copy)]
pub(super) enum ResultDecoder {
    Int,
    ProcessStatus,
    Bool,
    Boundary,
    Table,
    TrapOnly,
}

/// Root/native trap reporting is outside the source-value carrier vocabulary.
/// The invalid low-byte tag keeps this token disjoint from every BoundaryWord;
/// the payload is the planner-bound, nonzero trap identity.
pub(crate) const ROOT_TRAP_TOKEN_TAG: i64 = 0xff;
pub(crate) const ROOT_TRAP_TOKEN_SHIFT: i64 = 8;

/// Decode the magnitude shared by JIT root tokens and linked signed root
/// tokens into the zero-based planner-catalog index it names.
///
/// The sign belongs to the surrounding ABI and is removed before this call.
/// Identity zero, a malformed tag, and an index wider than the host address
/// space all refuse rather than becoming a generic trap.
pub(crate) fn root_trap_catalog_index(magnitude: u64) -> Option<usize> {
    (magnitude & ROOT_TRAP_TOKEN_TAG as u64 == ROOT_TRAP_TOKEN_TAG as u64)
        .then_some(magnitude >> ROOT_TRAP_TOKEN_SHIFT)
        .filter(|identity| *identity != 0)
        .and_then(|identity| usize::try_from(identity - 1).ok())
}

impl<M> CompiledModule<M> {
    /// Transparent one-to-one packing seam (RT-SPLIT §10.4a). Exists so the
    /// four construction-only fields (`func_id`, `decoder`, `result_table`,
    /// `trap`) can stay private to this module while the three existing
    /// construction sites live outside it. No validation, no defaults, no
    /// clones, no reordering, no policy -- adding any would make this a
    /// behavior change rather than wiring.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn from_parts(
        module: M,
        func_id: FuncId,
        decoder: Option<ResultDecoder>,
        result_table: BTreeMap<i64, RuntimeGroundValue>,
        trap: Option<RuntimeTrap>,
        trap_catalog: Vec<RuntimeTrap>,
        carrier_identity_catalog: Vec<(String, u64)>,
        verifier_passed: bool,
        assumptions: BTreeSet<String>,
        unsupported: Vec<String>,
    ) -> Self {
        Self {
            module,
            func_id,
            decoder,
            result_table,
            trap,
            trap_catalog,
            carrier_identity_catalog,
            verifier_passed,
            assumptions,
            unsupported,
        }
    }

    pub(super) fn trap_catalog(&self) -> &[RuntimeTrap] {
        &self.trap_catalog
    }
}

impl CompiledModule<JITModule> {
    pub(super) fn run(
        mut self,
        process_root: Option<*const std::ffi::c_void>,
    ) -> Result<(RuntimeObservation, Option<i64>), CraneliftBackendError> {
        if let Some(trap) = self.trap {
            return Ok((RuntimeObservation::Trapped(trap), None));
        }

        self.module
            .finalize_definitions()
            .map_err(|err| backend_module(err.to_string()))?;
        let code = self.module.get_finalized_function(self.func_id);
        // Named native-code-execution boundary. This is tested/validated JIT
        // execution, never a proof and never a host-ABI syscall boundary.
        let mut store = crate::boundary_value::BoundaryValueStore::default();
        for (symbol, identity) in &self.carrier_identity_catalog {
            if !store.issue_carrier_identity(symbol, *identity) {
                return Err(backend_module(
                    "compiled carrier identity catalog disagrees with itself".to_string(),
                ));
            }
        }
        let binding = crate::boundary_activation::BoundaryStoreBindingV1::open(
            &mut store,
            crate::boundary_resource_profile::starter_smoke_profile(),
        );
        let mut activation = crate::boundary_activation::BoundaryActivationV1::begin(&binding);
        let process_root = process_root
            .or_else(|| activation.native_frame_ptr())
            .ok_or_else(|| {
                backend_module("activation did not publish a launch pointer".to_string())
            })?;
        let services = activation
            .services_ptr()
            .ok_or_else(|| {
                backend_module("activation did not publish its services".to_string())
            })?;
        let native = unsafe {
            mem::transmute::<
                _,
                extern "C" fn(
                    *const std::ffi::c_void,
                    *const std::ffi::c_void,
                ) -> i64,
            >(code)
        };
        let token = native(process_root, services);
        let decoder = self
            .decoder
            .ok_or_else(|| backend(BackendFailure::NativeResultDecode { token }))?;
        let trap_identity = || {
            (token > 0)
                .then(|| root_trap_catalog_index(token as u64))
                .flatten()
        };
        let decode_trap = |identity: usize| {
            self.trap_catalog
                .get(identity)
                .cloned()
                .map(|trap| (RuntimeObservation::Trapped(trap), Some(token)))
        };
        match decoder {
            ResultDecoder::Boundary | ResultDecoder::ProcessStatus | ResultDecoder::TrapOnly => {
                if let Some(trapped) = trap_identity().and_then(decode_trap) {
                    return Ok(trapped);
                }
            }
            ResultDecoder::Table if !self.result_table.contains_key(&token) => {
                if let Some(trapped) = trap_identity().and_then(decode_trap) {
                    return Ok(trapped);
                }
            }
            ResultDecoder::Int | ResultDecoder::Bool | ResultDecoder::Table => {}
        }
        let ground = match decoder {
            ResultDecoder::Int => RuntimeGroundValue::Int(
                activation
                    .native_int_arena()
                    .decode_final_export()
                    .ok_or_else(|| backend(BackendFailure::NativeResultDecode { token }))?,
            ),
            ResultDecoder::ProcessStatus => RuntimeGroundValue::Int(token.into()),
            ResultDecoder::Bool => RuntimeGroundValue::Bool(token != 0),
            ResultDecoder::Boundary => match crate::boundary_value::BoundaryWord(token as u64)
                .tag()
            {
                Some(crate::boundary_value::BoundaryTag::ImmediateBool) => {
                    RuntimeGroundValue::Bool(
                        crate::boundary_value::BoundaryWord(token as u64).payload() != 0,
                    )
                }
                Some(crate::boundary_value::BoundaryTag::ImmediateInt) => {
                    RuntimeGroundValue::Int(
                        crate::boundary_value::BoundaryWord(token as u64)
                            .signed_payload()
                        .into(),
                    )
                }
                Some(crate::boundary_value::BoundaryTag::PersistentGround) => {
                    let adopted = activation
                        .finish(
                            &mut store,
                            Some(crate::boundary_value::BoundaryWord(token as u64)),
                        )
                        .map_err(|_| {
                            backend(BackendFailure::NativeResultDecode { token })
                        })?
                        .ok_or_else(|| {
                            backend(BackendFailure::NativeResultDecode { token })
                        })?;
                    store.observe_adopted_ground(adopted).ok_or_else(|| {
                        backend(BackendFailure::NativeResultDecode { token })
                    })?
                }
                // `RT-FNUNIT-RESULT-TOKEN` `D3`. The one arm this node adds.
                //
                // Seal FIRST: withdraw writers and freeze persistent state.
                // The activation still owns the invocation arena, which stays
                // read-only decode input. The aggregate word is passed to the
                // decoder and is NEVER root-adopted -- `finish` is called with
                // `None`, so nothing arena-backed can become persistent, and
                // the value that comes back is owned.
                Some(crate::boundary_value::BoundaryTag::InvocationAggregate) => {
                    activation
                        .finish(&mut store, None)
                        .map_err(|_| backend(BackendFailure::NativeResultDecode { token }))?;
                    crate::boundary_value::decode_invocation_ground(
                        activation.arena(),
                        &mut store,
                        crate::boundary_value::BoundaryWord(token as u64),
                    )
                    .map_err(|_| backend(BackendFailure::NativeResultDecode { token }))?
                }
                // THE CLOSED REFUSAL SET, SPELLED RATHER THAN WILDCARDED.
                //
                // Every one of these is a real tag this decoder does not read,
                // and `None` is a tag byte outside the closed set. Writing them
                // out makes a new `BoundaryTag` a compile error here instead of
                // silently joining the refusal -- and it is why widening this
                // arm cannot happen by accident, which `AC-4` names as the
                // cheap wrong repair.
                None
                | Some(crate::boundary_value::BoundaryTag::ImmediateExitStatus)
                | Some(crate::boundary_value::BoundaryTag::ImmediateBoundedNat)
                | Some(crate::boundary_value::BoundaryTag::ImmediateStructuralNat)
                | Some(crate::boundary_value::BoundaryTag::PersistentClosure)
                | Some(crate::boundary_value::BoundaryTag::InvocationBorrowed)
                | Some(crate::boundary_value::BoundaryTag::InvocationHostResult) => {
                    return Err(backend(BackendFailure::NativeResultDecode { token }));
                }
            },
            ResultDecoder::Table => self
                .result_table
                .get(&token)
                .cloned()
                .ok_or_else(|| backend(BackendFailure::NativeResultDecode { token }))?,
            ResultDecoder::TrapOnly => {
                return Err(backend(BackendFailure::NativeResultDecode { token }));
            }
        };
        Ok((RuntimeObservation::Returned(ground), Some(token)))
    }
}
