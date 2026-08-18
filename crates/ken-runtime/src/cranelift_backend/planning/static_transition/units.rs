//! The emitter's read-only view of one validated function unit, and the
//! cross-owner call edges between them.
//!
//! `RT-PLANNER-UNITS-ABI-SPLIT` `D1` — this module owns the `Emittable*`
//! vocabulary and the `StaticTransitionPlan` projections that derive it. The
//! types are unmintable in `lowering` (private fields, no constructors), and
//! every method is a read-only projection of facts the validated plane already
//! established. `StaticTransitionPlan` itself stays in the parent; the inherent
//! impls here read ancestor-private root state under the standing child-module
//! pattern.

use std::collections::BTreeMap;

use super::abi::{self, AbiFrameHeader, AbiSlot, AbiUnitDefinition};
use super::semantic_ir::StaticOriginId;
use super::{
    origin_of, planner_error, CraneliftBackendError, StaticNodeId, StaticTransitionPlan,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub(in crate::cranelift_backend) struct PredeclaredFunctionId(pub(super) u32);

/// **`RT-FNSPLIT-B2F` `D1` — the emitter's read-only view of ONE validated
/// function unit.**
///
/// ⭐ **This is the `case_constructor_identity` precedent, not a widened
/// field.** What crosses into `crate::cranelift_backend` is a *question about
/// a unit* and an answer the asker cannot mint: `AbiPlane`, `AbiDescriptor`,
/// `build_abi_plane` and `AbiPlane::validate` all stay `pub(super)`, so the
/// emitter can neither construct a plane, mutate a descriptor, nor reach the
/// pre-emission validator to bypass it.
///
/// ⛔ **The fields are private and there is no constructor**, so a unit
/// cannot be forged in `lowering`. That is the load-bearing half: `B2F`
/// drives emission from units, so an unmintable unit means emission cannot
/// be driven from anything but the validated plane.
///
/// **MEASURED:** `lowering` can read a unit's declared identity, origin,
/// definition, header and slot run, and can construct none of them.
/// **CLAIMED:** emission is driven by `B2R`'s validated authority rather than
/// by a second table `B2F` derives for itself.
/// **THE GAP:** ⚠ `AbiSlot` and `AbiFrameHeader` are plain `Copy` data whose
/// fields are now readable in `cranelift_backend`, so `lowering` **can**
/// spell a *local* `AbiSlot` literal — Rust cannot forbid struct-literal
/// construction inside one crate. ⛔ **This is not claimed to be detected.**
/// What closes it is that a forged slot has no route into a unit: the only
/// producer of an `EmittableUnit` is [`Self::emittable_units`], which reads
/// `self.abi`. A control that emission consumes only unit-supplied slots is
/// `AC-12`'s, and it is not discharged here.
/// **One cross-owner call edge, as the emitter is allowed to see it.**
///
/// ⭐ **Both ends are `PredeclaredFunctionId`s and nothing else.** There is no
/// node id, no origin and no expression here, because a call edge's whole
/// content at emission time is *which unit calls which unit* — and resolving a
/// callee to a target function must go through the planner's identity, never
/// through the ordinal some emission loop happened to assign.
///
/// ⛔ **Unmintable in `lowering`:** the fields are private and the sole producer
/// is [`StaticTransitionPlan::emittable_call_edges`]. ⇒ The emitter cannot
/// invent a call to a unit the planner did not connect, which is the property
/// that makes "no indirect dispatch on a dynamic property" structural rather
/// than a coding convention.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct EmittableCallEdge {
    caller: PredeclaredFunctionId,
    callee: PredeclaredFunctionId,
    callee_origin: StaticOriginId,
    call_site_origin: StaticOriginId,
    kind: EmittableCallKind,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum EmittableCallKind {
    StaticBody,
    Declaration,
}

impl EmittableCallEdge {
    /// The unit this call is emitted **into**.
    pub(in crate::cranelift_backend) fn caller(self) -> PredeclaredFunctionId {
        self.caller
    }

    /// The unit this call transfers **to**. ⛔ Resolve it through
    /// `UnitBundle::function`, whose `None` is a real answer.
    pub(in crate::cranelift_backend) fn callee(self) -> PredeclaredFunctionId {
        self.callee
    }

    pub(in crate::cranelift_backend) fn callee_origin(self) -> StaticOriginId {
        self.callee_origin
    }

    /// The source occurrence which owns this call operation.
    ///
    /// For a closure-body call this is the body target, preserving the
    /// established lookup. For a declaration call it is the exact
    /// `DeclarationRef` occurrence, so two references to one declaration remain
    /// distinct typed edges without emitter-side symbol lookup.
    pub(in crate::cranelift_backend) fn call_site_origin(self) -> StaticOriginId {
        self.call_site_origin
    }

    pub(in crate::cranelift_backend) fn kind(self) -> EmittableCallKind {
        self.kind
    }
}

#[derive(Clone, Copy, Debug)]
pub(in crate::cranelift_backend) struct EmittableUnit<'plan> {
    function: PredeclaredFunctionId,
    body_occurrence: StaticOriginId,
    /// This unit's scheduling entry — the axis a CALL names.
    planned_node: StaticNodeId,
    definition: AbiUnitDefinition,
    header: AbiFrameHeader,
    slots: &'plan [AbiSlot],
}

impl<'plan> EmittableUnit<'plan> {
    /// This unit's static identity. ⛔ Unmintable in `lowering`: the newtype's
    /// field stays `pub(super)`, so the emitter can key and compare an id but
    /// cannot fabricate one or do arithmetic on it.
    pub(in crate::cranelift_backend) fn function(self) -> PredeclaredFunctionId {
        self.function
    }

    /// The occurrence origin of this unit's body, for
    /// [`StaticTransitionPlan::source_occurrence`].
    ///
    /// The **issued** body occurrence, carried from the planner. It is not
    /// this unit's entry and must not be substituted with one.
    pub(in crate::cranelift_backend) fn body_occurrence(self) -> StaticOriginId {
        self.body_occurrence
    }

    /// This unit's **scheduling-entry** origin — the axis a call edge's
    /// `callee_origin` names.
    ///
    /// Exists so a call-identity consumer can say which axis it means
    /// instead of inheriting whichever one the carrier happens to hold. The two
    /// coincide for every unit whose body does not schedule something before
    /// itself, so a site that reads the wrong one is green on most fixtures.
    pub(in crate::cranelift_backend) fn entry_origin(self) -> StaticOriginId {
        origin_of(self.planned_node)
    }

    /// Whether this unit is a scheduling entry or a retained closure body,
    /// with the closure body's defining origin and capture provenance.
    pub(in crate::cranelift_backend) fn definition(self) -> AbiUnitDefinition {
        self.definition
    }

    /// The declared activation-frame header. ⚠ `frame_bytes` is derived from
    /// the slot run by `B2R`; do not recompute it from [`Self::slots`].
    pub(in crate::cranelift_backend) fn header(self) -> AbiFrameHeader {
        self.header
    }

    /// This unit's declared slots, in `B2R`'s layout order: parameters,
    /// captures, result, control, trap, store.
    pub(in crate::cranelift_backend) fn slots(self) -> &'plan [AbiSlot] {
        self.slots
    }

    /// Each slot's byte offset in this unit's activation frame, paired with the
    /// frame's total size.
    ///
    /// ⛔ **Delegated to `abi::slot_offsets`, never re-derived here.** The
    /// emitter needs offsets to load and store slots, and prefix-summing the
    /// widths at the emission site would put the same arithmetic in a second
    /// file where the two can silently disagree. `AbiFrameHeader::frame_bytes`
    /// is totalled *through* the same walk, so the offsets the emitter uses and
    /// the size the ABI declares cannot diverge.
    ///
    /// ⚠ The returned total is checked against [`Self::header`]'s `frame_bytes`
    /// by `AC-3`, not here — this accessor's job is to have one walk, not to
    /// assert about it.
    pub(in crate::cranelift_backend) fn slot_offsets(
        self,
    ) -> Result<(Vec<u32>, u32), CraneliftBackendError> {
        abi::slot_offsets(self.slots)
    }
}

impl StaticTransitionPlan<'_> {
    /// **`RT-FNSPLIT-B2F` `D1` — every function unit this artifact must emit, in
    /// unit order.**
    ///
    /// ⛔ **This does not derive the population and must never be made to.** The
    /// set is `plan.entries` ∪ every `EdgeKind::StaticBody` **target** minus
    /// every `D2a` declaration-owned pair, already
    /// seeded and validated by `B2O` (`semantic_ir.rs`
    /// `validate_function_units`) and already given one descriptor apiece by
    /// `B2R`. This walks `self.abi.descriptors` and projects; it re-seeds
    /// nothing, and in particular it does **not** consult
    /// `TransitionKind::ClosureBody`, which is a body's *return successor* and
    /// not a unit head.
    ///
    /// ⚠ The two shared exits (`SemanticOwner::Terminal`, `TrapTerminal`) are not
    /// units and are absent here by construction — they never receive a
    /// descriptor.
    /// **`RT-FNSPLIT-B2F` `AC-11` — prove every transfer this node will emit is
    /// representable, BEFORE any unit is declared, defined or called.**
    ///
    /// ⛔ Exposed as a **verdict**, not as the plane: the semantic plane and its
    /// source seeds stay private, so an emitter can obtain the answer and cannot
    /// re-derive a different one. ⭐ That is what keeps this a single authority
    /// rather than a check the emitter could route around.
    ///
    /// ⛔ Clause 3 is discharged by the CALL SITE's position, not by this
    /// method's contents: it runs before `declare_unit_bundle` in
    /// `compile_expr_into_module`. Moving the call after emission would satisfy
    /// every assertion inside it and discharge nothing.
    pub(in crate::cranelift_backend) fn validate_emitted_transfers_are_representable(
        &self,
    ) -> Result<(), CraneliftBackendError> {
        abi::validate_emitted_transfers(
            &self.semantic,
            &self.nodes,
            &self.semantic_sources,
            &self.abi.descriptors,
            &self.abi.slots,
        )
    }

    pub(in crate::cranelift_backend) fn emittable_call_edges(
        &self,
    ) -> Result<Vec<EmittableCallEdge>, CraneliftBackendError> {
        let mut calls = self
            .semantic
            .static_body_call_edges(&self.edges)?
            .into_iter()
            .map(|(caller, callee, callee_origin)| EmittableCallEdge {
                caller,
                callee,
                callee_origin,
                call_site_origin: callee_origin,
                kind: EmittableCallKind::StaticBody,
            })
            .collect::<Vec<_>>();
        calls.extend(
            self.semantic
                .declaration_call_edges(&self.edges)?
                .into_iter()
                .map(
                    |(caller, callee, callee_origin, call_site_origin)| EmittableCallEdge {
                        caller,
                        callee,
                        callee_origin,
                        call_site_origin,
                        kind: EmittableCallKind::Declaration,
                    },
                ),
        );
        Ok(calls)
    }

    /// **`D5a` checkpoint 1 — the units that receive a declared and defined
    /// `Function`.**
    ///
    /// [`Self::emittable_units`] stays the **descriptor / provenance / template**
    /// population and is unchanged — `D2f` keeps it complete, so a fusion-owned
    /// producer remains the source, ABI and template authority for the body its
    /// fused definition lowers. This is the **executable** subset. The two must
    /// not be conflated in the other direction either: declaring from here and
    /// defining from `emittable_units` (or the reverse) is exactly the
    /// undefined-phantom the ruling forbids, so both the declaration pass and the
    /// definition pass read this one method.
    pub(in crate::cranelift_backend) fn executable_units(
        &self,
    ) -> Result<Vec<EmittableUnit<'_>>, CraneliftBackendError> {
        let dispositions = self.body_dispositions()?;
        Ok(self
            .emittable_units()?
            .into_iter()
            // The disposition map is keyed by BODY origin — `D5a`'s candidates
            // come from `context.worker_body_origin()` and `D2f`'s from
            // `claim.producer_body()` — so the membership test names the body
            // axis, exactly as `unit.body_occurrence()` does.
            .filter(|unit| !dispositions.contains_key(&unit.body_occurrence()))
            .collect())
    }

    /// **`D5a` checkpoint 1 — the call edges that survive the retarget.**
    ///
    /// An edge into a template-only body is the seeding edge whose realization
    /// retargeted; resolving it would demand a `FuncId` for a unit that has no
    /// emitted `Function`, and fabricating one is the failure
    /// `UnitBundle::function`'s `Option` exists to expose.
    /// ⛔ **The probe names the BODY axis, because the set does.**
    /// `template_only` is a set of worker body origins — its candidates come
    /// from `context.worker_body_origin()` — and the sibling `executable_units`
    /// probes it with `unit.body_occurrence()` for that reason.
    /// `edge.callee_origin()` is the **scheduling entry**, a different axis:
    /// `resolve_call_edges` enforces `unit.entry_origin() == edge.callee_origin()`,
    /// so reading it here would ask an executability question with a call-identity
    /// key. The two coincide for every unit whose body does not schedule
    /// something before itself, which is why probing the wrong one stays green
    /// on most fixtures; the invariant this file states at the composed-selector
    /// refusal is that *executability is a function of the body alone*.
    pub(in crate::cranelift_backend) fn executable_call_edges(
        &self,
    ) -> Result<Vec<EmittableCallEdge>, CraneliftBackendError> {
        let dispositions = self.body_dispositions()?;
        let body_axis: BTreeMap<PredeclaredFunctionId, StaticOriginId> = self
            .emittable_units()?
            .into_iter()
            .map(|unit| (unit.function(), unit.body_occurrence()))
            .collect();
        Ok(self
            .emittable_call_edges()?
            .into_iter()
            .filter(|edge| match body_axis.get(&edge.callee()) {
                Some(body) => !dispositions.contains_key(body),
                // A callee with no descriptor is a planner contradiction this
                // filter does not own. Retaining the edge hands it downstream
                // for rejection rather than silently suppressing it here.
                //
                // ⛔ Which rejection is NOT promised. `resolve_call_edges`
                // resolves `bundle.function(edge.callee())` BEFORE it looks the
                // descriptor up, so an ordinary forward-declaration failure can
                // preempt the descriptor diagnostic. The guarantee is only that
                // the edge is rejected, never suppressed.
                None => true,
            })
            .collect())
    }

    pub(in crate::cranelift_backend) fn root_emittable_unit(
        &self,
    ) -> Result<EmittableUnit<'_>, CraneliftBackendError> {
        let root_entry = self
            .root_entry
            .ok_or_else(|| planner_error("plan has no recorded root entry"))?;
        let root_function = self.semantic.function_for_node(root_entry)?;
        self.emittable_units()?
            .into_iter()
            .find(|unit| unit.function() == root_function)
            .ok_or_else(|| planner_error("recorded root has no abi descriptor"))
    }

    pub(in crate::cranelift_backend) fn emittable_units(
        &self,
    ) -> Result<Vec<EmittableUnit<'_>>, CraneliftBackendError> {
        self.abi
            .descriptors
            .iter()
            .map(|descriptor| {
                let start = descriptor.slots.start as usize;
                let end = start
                    .checked_add(descriptor.slots.len as usize)
                    .ok_or_else(|| planner_error("abi slot range overflows"))?;
                let slots = self
                    .abi
                    .slots
                    .get(start..end)
                    .ok_or_else(|| planner_error("abi slot range is outside the plane"))?;
                Ok(EmittableUnit {
                    function: descriptor.function,
                    body_occurrence: descriptor.body_occurrence,
                    planned_node: descriptor.planned_node,
                    definition: descriptor.definition,
                    header: descriptor.header,
                    slots,
                })
            })
            .collect()
    }
}
