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

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::super::tests::{
        b2ac_topology_fixtures, b2o_transparent_declaration, b2o_two_closure_fixture,
        b2r_plan, contspec_nested_fixture, contspec_parameter_match, contspec_plan, unit,
    };
    use crate::cranelift_backend::surface::NativeSeedEnvironment;
    use crate::RuntimeGroundValue;

    /// A closure whose captures arrive by the **seed** provenance: the captures
    /// are symbols resolved against the seed environment at JIT time.
    fn b2r_seed_closure(captures: &[&str], body: RuntimeExpr) -> RuntimeExpr {
        RuntimeExpr::Closure {
            captures: captures.iter().map(|c| (*c).to_string()).collect(),
            params: vec!["x".to_string()],
            body: Box::new(body),
        }
    }

    /// A closure whose captures arrive by the **lexical** provenance: each
    /// capture is an arbitrary source expression, planned as a syntax child.
    fn b2r_lexical_closure(captures: Vec<RuntimeExpr>, body: RuntimeExpr) -> RuntimeExpr {
        RuntimeExpr::LexicalClosure {
            captures,
            params: vec!["x".to_string()],
            body: Box::new(body),
        }
    }

    /// The single capture slot of the fixture's one closure unit.
    fn b2r_only_capture_slot(plan: &StaticTransitionPlan<'_>) -> AbiSlot {
        let mut found = Vec::new();
        for descriptor in &plan.abi.descriptors {
            let start = descriptor.slots.start as usize;
            let end = start + descriptor.slots.len as usize;
            for slot in &plan.abi.slots[start..end] {
                if slot.kind == AbiSlotKind::Capture {
                    found.push(*slot);
                }
            }
        }
        assert_eq!(
            found.len(),
            1,
            "the fixture must declare exactly one capture slot, or this helper \
             is silently picking one of several"
        );
        found[0]
    }

    fn contspec_persistent_result_with_parameter_fixture() -> RuntimeExpr {
        RuntimeExpr::Let {
            // Rebind process parameter 1 at de Bruijn ordinal 0. The
            // consumer result remains persistent while its inputs retain
            // invocation-arena affinity.
            value: Box::new(RuntimeExpr::Var(1)),
            body: Box::new(contspec_parameter_match(unit())),
        }
    }

    /// `AC-1` — descriptor totality over the owner partition, **both
    /// directions**.
    ///
    /// ⚠ A one-directional check passes happily on an orphan, so both are
    /// asserted: every unit has exactly one descriptor, and every descriptor
    /// names a member of the partition.
    ///
    /// Promise class: **durable invariant** — a relation between two populations,
    /// not a frozen count.
    #[test]
    fn b2r_ac1_every_function_unit_has_exactly_one_descriptor_and_conversely() {
        let expr = b2r_lexical_closure(Vec::new(), RuntimeExpr::Var(0));
        let plan = b2r_plan(&expr);

        // Non-vacuity FIRST: a plane with one unit would make both directions
        // true for the wrong reason, and every claim below would be green on a
        // fixture that never exercised a boundary.
        assert!(
            plan.semantic.functions.len() > 1,
            "the fixture has only one function unit, so totality is trivially \
             true and this control observes nothing"
        );

        // Direction 1 — every unit is covered.
        assert_eq!(
            plan.abi.descriptors.len(),
            plan.semantic.functions.len(),
            "AC-1: the descriptor population is not exact for the function unit \
             partition"
        );
        // Direction 2 — every descriptor names a member, positionally.
        for (ordinal, descriptor) in plan.abi.descriptors.iter().enumerate() {
            let function = &plan.semantic.functions[ordinal];
            assert_eq!(descriptor.function, function.id, "AC-1: descriptor/unit id");
            assert_eq!(
                descriptor.planned_node, function.planned_node,
                "AC-1: a descriptor names a node that is not its unit's seed"
            );
        }

        // And an ORPHAN must be refused, so direction 2 is a real detector
        // rather than a restatement of how the builder happens to loop.
        let mut orphaned = plan.abi.clone();
        orphaned.descriptors.pop();
        let err = orphaned
            .validate(
                &plan.semantic,
                &plan.nodes,
                &plan.semantic_sources,
                &plan.edges,
                &plan.entries,
                &plan.declaration_occurrences.values().copied().collect(),
                plan.root_entry.expect("root entry"),
                plan.root_ingress,
            )
            .expect_err("AC-1: dropping a descriptor must be refused");
        // ⛔ The EXACT failure, not `is_err()`. A control that reddens does not
        // confirm which detector caught it, and `is_err()` would stay green if
        // some unrelated law started firing first.
        assert!(
            format!("{err:?}").contains("not exact for the function unit partition"),
            "AC-1: the orphan was refused, but not by the totality law. Got: {err:?}"
        );
    }

    /// `AC-2` / `C1` — **a descriptor post-condition, not a census of the 44
    /// caller-environment append sites.**
    ///
    /// ⭐ The site census is a spelling standing in for a population: the frame
    /// measured 44 sites across two spellings, and the site `C1` names is in the
    /// spelling a sweep written against the other one **excludes**. This control
    /// is mechanism-independent instead — it holds whether the environment is
    /// appended, cloned, threaded or restructured, and it still holds at the
    /// 45th site.
    ///
    /// Promise class: **durable invariant.**
    #[test]
    fn b2r_ac2_an_irrelevant_caller_binding_does_not_change_the_callee_descriptor() {
        let inner = b2r_lexical_closure(Vec::new(), RuntimeExpr::Var(0));
        let wrapped = RuntimeExpr::Let {
            value: Box::new(unit()),
            body: Box::new(b2r_lexical_closure(Vec::new(), RuntimeExpr::Var(0))),
        };

        let bare = b2r_plan(&inner);
        let deeper = b2r_plan(&wrapped);

        // ⚠ Non-vacuity: the extra binding must actually have changed the plan.
        // Comparing two identical plans would pass for the wrong reason.
        assert!(
            deeper.nodes.len() > bare.nodes.len(),
            "AC-2: the irrelevant binding did not change the plan, so descriptor \
             invariance is being asserted against an unchanged input"
        );

        // The unit count is unchanged: an irrelevant binding adds no scheduling
        // entry and no static body edge.
        assert_eq!(
            deeper.semantic.functions.len(),
            bare.semantic.functions.len(),
            "AC-2: an irrelevant caller binding changed the function unit count"
        );

        // ⭐ SHAPE, not identity. `planned_node`/`origin` are positional over the
        // node table and legitimately move when the table grows; the LAYOUT must
        // not. This narrowing was recorded in the predictions file (`P2`) BEFORE
        // measuring, so it is a stated design choice rather than a red assertion
        // trimmed until it passed.
        assert_eq!(
            deeper.abi.shapes().expect("shapes"),
            bare.abi.shapes().expect("shapes"),
            "AC-2/C1: adding an irrelevant caller binding changed a callee \
             descriptor's slot count or layout, which is the caller-depth \
             dependence this node exists to remove"
        );
    }

    /// `AC-3` / `C2` — **both** capture provenances produce a declared slot with
    /// a declared layout, and they are a **non-degenerate discriminator pair.**
    ///
    /// ⚠ A single positive case is green-vs-green under the exact swap it should
    /// catch. The two provenances are exercised on the **same** closure shape,
    /// differing only in how their captures arrive, so a collapse of the two into
    /// one carrier fails **both** sides rather than neither.
    ///
    /// ⭐ **Where the real enforcement lives.** That a seed layout is not chosen
    /// by inspecting the particular runtime value is enforced by the
    /// **signature**: `AbiCaptureProvenance::carrier` takes no value, and
    /// `build_abi_plane`'s inputs contain no `RuntimeGroundValue` and no
    /// `Lowered`. There is nothing to inspect. This test is a positive control
    /// that the mechanism is reachable and discriminating — not the enforcement.
    ///
    /// Promise class: **durable invariant.**
    #[test]
    fn b2r_ac3_both_capture_provenances_declare_slots_and_select_distinct_carriers() {
        let seeded_expr = b2r_seed_closure(&["c"], RuntimeExpr::Var(0));
        let lexical_expr = b2r_lexical_closure(vec![unit()], RuntimeExpr::Var(0));
        let seeded = b2r_plan(&seeded_expr);
        let lexical = b2r_plan(&lexical_expr);

        let seed_capture = b2r_only_capture_slot(&seeded);
        let lexical_capture = b2r_only_capture_slot(&lexical);

        // Both declare a slot with a declared layout — kind, carrier, ownership,
        // width and alignment, none of them absent or defaulted.
        for (label, slot) in [("seed", seed_capture), ("lexical", lexical_capture)] {
            assert_eq!(slot.kind, AbiSlotKind::Capture, "AC-3: {label} slot kind");
            assert_eq!(slot.width_bytes, 8, "AC-3: {label} declared width");
            assert_eq!(slot.align_bytes, 8, "AC-3: {label} declared alignment");
        }

        // ⭐ The discriminator: the two provenances select DIFFERENT carriers on
        // the same closure shape. Collapsing them would fail this, and a swap of
        // the two would fail it too.
        assert_eq!(
            seed_capture.carrier,
            AbiCarrier::GroundValueCarrier,
            "AC-3/C2: a seed capture must travel in the fixed closed carrier for \
             the permitted ground-value family"
        );
        assert_eq!(
            lexical_capture.carrier,
            AbiCarrier::ValueWord,
            "AC-3/C2: a lexical capture travels in the ordinary value carrier"
        );
        assert_ne!(
            seed_capture.carrier, lexical_capture.carrier,
            "AC-3/C2: the two provenances collapsed to one carrier, so a pin \
             keyed to either one would be a spelling standing in for the \
             population"
        );
    }

    /// `AC-4` / `C3` — *the transported payload may change; the ABI may not.*
    ///
    /// Two required controls, plus the positive rejection control that stops the
    /// negative half from passing because nothing reached the checker.
    ///
    /// Promise class: **durable invariant** for the invariance halves; **durable
    /// mutation proof** for the rejection half.
    #[test]
    fn b2r_ac4_the_abi_is_invariant_under_payload_and_depth_and_rejects_an_implicit_tail() {
        // Control 1 — caller DEPTH changes, per-origin descriptor is identical.
        let shallow_expr = b2r_seed_closure(&["c"], RuntimeExpr::Var(0));
        let deep_expr = RuntimeExpr::Let {
            value: Box::new(unit()),
            body: Box::new(RuntimeExpr::Let {
                value: Box::new(unit()),
                body: Box::new(b2r_seed_closure(&["c"], RuntimeExpr::Var(0))),
            }),
        };
        let shallow = b2r_plan(&shallow_expr);
        let deep = b2r_plan(&deep_expr);
        assert!(
            deep.nodes.len() > shallow.nodes.len() + 1,
            "AC-4: the depth control did not actually deepen the caller"
        );
        assert_eq!(
            deep.abi.shapes().expect("shapes"),
            shallow.abi.shapes().expect("shapes"),
            "AC-4/C3: the per-origin descriptor varied with CALLER DEPTH"
        );

        // Control 2 — the seed capture's payload changes within its declared
        // carrier class. ⭐ Renaming the captured symbol changes WHICH ground
        // value the seed environment will supply at JIT time; the descriptor's
        // shape must not move. The carrier cannot vary with the value because no
        // value is in scope to vary it — this control observes that the arity and
        // layout are likewise untouched.
        let other_payload_expr = b2r_seed_closure(&["a-different-capture"], RuntimeExpr::Var(0));
        let other_payload = b2r_plan(&other_payload_expr);
        assert_eq!(
            other_payload.abi.shapes().expect("shapes"),
            shallow.abi.shapes().expect("shapes"),
            "AC-4/C3: the descriptor shape moved when the transported payload did"
        );

        // ⚠ Control 3 — the POSITIVE control. "The validator rejects an implicit
        // caller-env tail" passes for any reason, including that nothing ever
        // reached the checker. So construct one and observe the rejection.
        let mut tailed = shallow.abi.clone();
        let last = tailed
            .descriptors
            .last_mut()
            .expect("the fixture has at least one descriptor");
        last.slots.len += 1;
        let tail_slot = *tailed.slots.last().expect("the fixture has slots");
        tailed.slots.push(tail_slot);
        let err = tailed
            .validate(
                &shallow.semantic,
                &shallow.nodes,
                &shallow.semantic_sources,
                &shallow.edges,
                &shallow.entries,
                &shallow.declaration_occurrences.values().copied().collect(),
                shallow.root_entry.expect("root entry"),
                shallow.root_ingress,
            )
            .expect_err("AC-4/C3: an implicit caller-environment tail must be REFUSED");
        assert!(
            format!("{err:?}").contains("implicit caller-environment tail"),
            "AC-4/C3: the tail was refused, but not by the tail law -- a control \
             that reddens does not confirm WHICH detector caught it. Got: {err:?}"
        );
    }

    /// `AC-5` / `C4` — cross-module linking is a **checked** exclusion, paired
    /// with a positive intra-module control so the exclusion is distinguishable
    /// from a gap.
    ///
    /// Promise class: **durable mutation proof** plus a positive control.
    #[test]
    fn b2r_ac5_an_imported_capture_edge_is_refused_and_intra_module_recursion_is_not() {
        // The exclusion. A lexical closure's captures are arbitrary source
        // expressions, so this is a real plan in which an imported value would
        // have to cross into a frame and be given a carrier.
        let imported = b2r_lexical_closure(
            vec![RuntimeExpr::ImportedDeclarationRef {
                symbol: "decl:other::thing".to_string(),
                dependency: "other".to_string(),
                dependency_semantic_hash: "hash".to_string(),
            }],
            RuntimeExpr::Var(0),
        );
        let declarations = BTreeMap::new();
        let err = match plan_static_transition_graph(&imported, &declarations) {
            Ok(_) => panic!(
                "AC-5/C4: an imported capture edge must be REFUSED before emission, \
                 and it planned green instead"
            ),
            Err(err) => err,
        };
        assert!(
            matches!(err, CraneliftBackendError::Unsupported(ref u) if u.construct == "ImportedDeclarationRef"),
            "AC-5/C4: the refusal must be the EXISTING dependency-linking \
             unsupported result, not a generic planner error. Got: {err:?}"
        );

        // ⚠ The positive control. Without it, the assertion above is
        // indistinguishable from a planner that refuses closures generally.
        let intra = b2r_lexical_closure(
            vec![RuntimeExpr::DeclarationRef {
                symbol: "decl:fixture::b2o".to_string(),
            }],
            RuntimeExpr::Var(0),
        );
        let plan = plan_static_transition_graph(&intra, &declarations)
            .expect("AC-5/C4: an INTRA-module declaration capture must plan green");
        assert!(
            plan.abi.descriptors.len() > 1,
            "AC-5/C4: the positive control produced no boundary, so it does not \
             discriminate"
        );
    }

    /// `AC-10` — the **predicted** descriptor population, measured.
    ///
    /// ⭐ The numbers below were written into
    /// `docs/program/rt-fnsplit-b2r-predictions.md` (`P1`) and committed at
    /// `b7aacd03`, **before** `abi.rs` existed. A count re-fit to what the code
    /// happens to produce measures nothing; the commit graph orders the
    /// prediction ahead of the measurement so a miss stays legible as a miss.
    ///
    /// Promise class: **durable invariant** — the assertion is the RELATION
    /// `descriptors == entries + StaticBody edges`, which survives any change
    /// preserving the contract. The per-fixture table beside it is a
    /// **transition sentinel**: it is a snapshot of these seven fixtures and is
    /// retired when the fixture set changes.
    #[test]
    fn b2r_ac10_the_descriptor_population_matches_the_prediction_on_every_fixture() {
        let mut measured = Vec::new();
        for (name, expr) in b2ac_topology_fixtures() {
            let declarations = BTreeMap::new();
            let plan = plan_static_transition_graph(&expr, &declarations).expect("plannable");

            // The durable relation, asserted per fixture.
            let static_body = plan
                .edges
                .iter()
                .filter(|edge| edge.kind == EdgeKind::StaticBody)
                .count();
            // ⚠ `RT-DECL-CLOSURE-PORT` `D2a` subtracts the declaration-owned
            // pairs from this relation. It is omitted here because every
            // fixture in this set is planned with NO declarations
            // (`declarations` is empty above), so there are no such pairs. ⛔
            // The general form is enforced by
            // `SemanticPlane::validate_function_units`, and the fixture that
            // exercises the subtraction is
            // `d2a_one_source_declaration_contributes_exactly_one_function`. If
            // a declaration ever enters this fixture set, this assertion is
            // where it will red — and the subtraction is the fix, not a
            // re-baselined count.
            assert!(
                declarations.is_empty(),
                "AC-10/AC-1: `{name}` -- this relation omits D2a's \
                 declaration-owned-pair subtraction because the fixture set \
                 carries no declarations"
            );
            assert_eq!(
                plan.abi.descriptors.len(),
                plan.entries.len() + static_body,
                "AC-10/AC-1: `{name}` -- descriptors are not the scheduling \
                 entries plus the static body targets"
            );
            measured.push((name, plan.abi.descriptors.len()));
        }

        assert_eq!(
            measured,
            vec![
                ("leaf", 1),
                ("let-if", 1),
                ("match", 1),
                ("lexical-closure-call", 2),
                ("computational", 1),
                ("computational-nested", 1),
                ("computational-under-let", 1),
            ],
            "AC-10: the measured descriptor population differs from the value \
             predicted at `b7aacd03` before this module was written"
        );
        assert_eq!(
            measured.iter().map(|(_, n)| n).sum::<usize>(),
            8,
            "AC-10: predicted 8 descriptors over the seven fixtures"
        );

        // And the richer B2O fixture: 2 scheduling entries (root + the
        // transparent declaration) and 2 static body edges.
        let declaration = b2o_transparent_declaration(unit());
        let mut declarations = BTreeMap::new();
        declarations.insert("decl:fixture::b2o", &declaration);
        let expr = b2o_two_closure_fixture();
        let plan = plan_static_transition_graph(&expr, &declarations).expect("plannable");
        assert_eq!(
            plan.abi.descriptors.len(),
            4,
            "AC-10: predicted 4 descriptors on the two-closure fixture"
        );
    }

    /// `AC-11` — **every rejection class `D5` advertises has a witness that
    /// reaches THAT arm.**
    ///
    /// ⛔ This is not `AC-4`'s positive control and `AC-4` does not cover it.
    /// `AC-4` proves *the checker was reached*; `AC-11` proves *which arm
    /// rejected*. In the failure mode the input **is** constructed, the validator
    /// **does** reject, the test **is** green — and an **earlier** arm returned
    /// the error while the arm you meant to exercise is unreachable code.
    ///
    /// ⭐ Asserting the **exact** message rather than `is_err`/`expect_err` is
    /// the entire mechanism. With `expect_err` every row below reads green and
    /// teaches nothing.
    ///
    /// Promise class: **durable mutation proof.** Each row names the arm that
    /// actually fired, so a re-ordering of the validator reddens here rather than
    /// silently changing which law is load-bearing.
    #[test]
    fn b2r_ac11_every_advertised_d5_rejection_class_names_the_arm_that_actually_fires() {
        let expr = b2r_seed_closure(&["c"], RuntimeExpr::Var(0));
        let plan = b2r_plan(&expr);
        let base = &plan.abi;

        let check = |abi: &AbiPlane| -> String {
            match abi.validate(
                &plan.semantic,
                &plan.nodes,
                &plan.semantic_sources,
                &plan.edges,
                &plan.entries,
                &plan.declaration_occurrences.values().copied().collect(),
                plan.root_entry.expect("root entry"),
                plan.root_ingress,
            ) {
                Ok(()) => "NO WITNESS -- the mutation was accepted".to_string(),
                Err(err) => format!("{err:?}"),
            }
        };

        let closure_unit = base
            .descriptors
            .iter()
            .position(|d| d.header.captures == 1)
            .expect("the fixture must have a unit with exactly one capture");

        let mut measured = Vec::new();

        // D5 class 1 -- a MISSING capture slot.
        let mut missing = base.clone();
        missing.descriptors[closure_unit].header.captures = 0;
        measured.push(("missing capture slot", check(&missing)));

        // D5 class 2 -- an EXTRA capture slot.
        let mut extra = base.clone();
        extra.descriptors[closure_unit].header.captures = 2;
        measured.push(("extra capture slot", check(&extra)));

        // D5 class 3 -- an implicit caller-environment TAIL.
        let mut tailed = base.clone();
        let tail = *tailed.slots.last().expect("slots");
        tailed
            .descriptors
            .last_mut()
            .expect("descriptors")
            .slots
            .len += 1;
        tailed.slots.push(tail);
        measured.push(("implicit caller-env tail", check(&tailed)));

        // D5 class 4 -- caller/callee dynamic-edge LAYOUT DISAGREEMENT.
        //
        // ⛔ An earlier revision mutated `planned_node` here, which tests TARGET
        // IDENTITY while naming layout agreement -- the Architect's finding. A
        // real witness must leave identity intact and make the CALLER-side
        // transfer layout disagree with the callee's declared frame.
        //
        // This grows the defining occurrence's capture-child count in the graph
        // while leaving its recorded `capture_slots` alone. The per-descriptor
        // checks compare against `capture_slots` and so still pass; only the
        // boundary comparison, which counts capture children caller-side, can
        // see the divergence. That is exactly the independence the signature
        // claims.
        let lexical_expr = b2r_lexical_closure(vec![unit()], RuntimeExpr::Var(0));
        let lexical_plan = b2r_plan(&lexical_expr);
        let mut skewed_plane = lexical_plan.semantic.clone();
        let defining = lexical_plan
            .edges
            .iter()
            .find(|edge| edge.kind == EdgeKind::StaticBody)
            .map(|edge| edge.from.0 as usize)
            .expect("the lexical fixture has a static body boundary");
        let program = skewed_plane.descriptors[defining].program.0 as usize;
        let record = skewed_plane.programs[program].records.start as usize;
        let extra = skewed_plane.records[record].child_origins;
        let borrowed = skewed_plane.child_origins[extra.start as usize];
        skewed_plane
            .child_origins
            .insert((extra.start + extra.len) as usize, borrowed);
        skewed_plane.records[record].child_origins.len += 1;
        let layout_arm = match lexical_plan.abi.validate(
            &skewed_plane,
            &lexical_plan.nodes,
            &lexical_plan.semantic_sources,
            &lexical_plan.edges,
            &lexical_plan.entries,
            &lexical_plan.declaration_occurrences.values().copied().collect(),
            lexical_plan.root_entry.expect("root entry"),
            lexical_plan.root_ingress,
        ) {
            Ok(()) => "NO WITNESS -- the layout skew was accepted".to_string(),
            Err(err) => format!("{err:?}"),
        };
        measured.push(("edge layout disagreement", layout_arm));

        // D5 class 5 -- a recursive-bundle member that is NOT forward-declared.
        let mut unforward = base.clone();
        unforward.descriptors.truncate(closure_unit);
        measured.push(("callee not forward-declared", check(&unforward)));

        // D5 class 6 -- representability / the imported-edge exclusion. This one
        // is a GRAPH witness, not a plane mutation: it is checked during
        // construction, before any descriptor is minted.
        let imported = b2r_lexical_closure(
            vec![RuntimeExpr::ImportedDeclarationRef {
                symbol: "decl:other::thing".to_string(),
                dependency: "other".to_string(),
                dependency_semantic_hash: "hash".to_string(),
            }],
            RuntimeExpr::Var(0),
        );
        let declarations = BTreeMap::new();
        let imported_arm = match plan_static_transition_graph(&imported, &declarations) {
            Ok(_) => "NO WITNESS -- the imported capture edge planned green".to_string(),
            Err(err) => format!("{err:?}"),
        };
        measured.push(("imported capture edge", imported_arm));

        let report = measured
            .iter()
            .map(|(class, arm)| format!("{class} => {arm}"))
            .collect::<Vec<_>>();
        // ⭐ MEASURED, not predicted-then-fitted. The arm named on each row is
        // the one that actually returned. **Five of the six classes reach an arm
        // of this validator's own; exactly ONE is enforced by an earlier arm**
        // and is recorded as such rather than counted as a law of its own.
        //
        // ⚠ **Row 5 — recursive-bundle forward declaration — is the subsumed
        // one.** Descriptors are dense and complete over the partition before any
        // edge resolves, which *is* forward-declaration, so the dense population
        // check sees a gap first and the class never reaches an arm of its own.
        // It is reported as subsumed, not counted.
        //
        // ⭐ **Row 4 — edge-layout disagreement — reaches its OWN arm**,
        // `"boundary signature ... transferred capture count"`, supplied by
        // `AbiBoundarySignature` / `validate_boundary_layouts` in `abi.rs`.
        //
        // ⛔ **This paragraph previously said rows 4 AND 5 were subsumed and that
        // the edge-agreement code had been deleted. That was true of an earlier
        // revision and false of this one**, and the assertion directly below
        // proved it false while the comment still said it. The deletion was
        // reverted once the Architect established that the composition I cited
        // proves target IDENTITY and never layout AGREEMENT; a real caller-side
        // boundary check now exists. `B2F` is told to read this validator as its
        // guarantee, so a stale count of live laws here is exactly the
        // silently-inherited defect `AC-11` exists to prevent -- which makes a
        // stale governing comment the same defect one layer up from the code.
        assert_eq!(
            report,
            vec![
                // -- reach an arm of their own --
                "missing capture slot => Backend(PlannerInvariant(\"abi descriptor \
                 is missing a declared capture slot\"))"
                    .to_string(),
                "extra capture slot => Backend(PlannerInvariant(\"abi descriptor \
                 declares a capture slot its origin does not have\"))"
                    .to_string(),
                "implicit caller-env tail => Backend(PlannerInvariant(\"abi frame \
                 carries an implicit caller-environment tail\"))"
                    .to_string(),
                // -- reaches the boundary-layout arm --
                "edge layout disagreement => Backend(PlannerInvariant(\"boundary \
                 signature and callee descriptor disagree on the transferred \
                 capture count\"))"
                    .to_string(),
                // -- SUBSUMED: descriptors are dense over the partition before
                //    any edge resolves, which IS forward-declaration --
                "callee not forward-declared => Backend(PlannerInvariant(\"abi \
                 descriptor population is not exact for the function unit \
                 partition\"))"
                    .to_string(),
                // -- reaches its own arm, with the EXISTING unsupported result --
                "imported capture edge => Unsupported(UnsupportedLowering { \
                 construct: \"ImportedDeclarationRef\", reason: \"imported \
                 declaration requires dependency linking, so it receives no \
                 callable descriptor in the intra-module representation \
                 contract\" })"
                    .to_string(),
            ],
            "AC-11: the arm that actually fired differs from the one recorded \
             for this class. Either the validator was re-ordered -- in which \
             case which law is load-bearing has changed and that is the point of \
             this test -- or a previously subsumed arm became reachable."
        );
    }

    /// `AC-3` positive control, as the frame words it — **a seed capture whose
    /// ground value is a `Constructor`, a `Record`, or a `String` must still
    /// yield one FIXED carrier, and the descriptor must not vary with the
    /// value.**
    ///
    /// ⛔ An earlier revision discharged this by renaming a capture symbol. That
    /// is not the discriminator the frame asks for: it never constructs a value
    /// from the family, so it cannot observe representability across it. The
    /// Architect's finding, and this is the repair.
    ///
    /// ⭐ **Two mechanisms, and they answer different questions.**
    ///
    /// 1. The **closed-family map** below is exhaustive over `RuntimeGroundValue`
    ///    with no `_ =>` arm, so a seventh variant is a **compile error** here
    ///    rather than a value that silently acquires a carrier. That is the
    ///    representability half.
    /// 2. Planning the same closure against three seed environments — each
    ///    binding the capture to a different variant of the family — must give
    ///    **byte-identical descriptors**. That is the invariance half.
    ///
    /// ⚠ **Why the second is stronger than it looks, stated honestly.** The
    /// descriptors are identical because the planner never receives a seed
    /// environment at all: `build_abi_plane`'s inputs contain no
    /// `RuntimeGroundValue`. So this control does not *discover* invariance — it
    /// **exhibits** that the family is real, constructible, and inert to the
    /// contract. The enforcement remains the signature. Recorded this way rather
    /// than presented as a measurement that could have come out otherwise.
    ///
    /// Promise class: **durable invariant.**
    #[test]
    fn b2r_ac3_the_closed_ground_value_family_yields_one_fixed_carrier() {
        // (1) Representability across the closed family. No `_ =>` arm.
        fn carrier_for(value: &RuntimeGroundValue) -> AbiCarrier {
            match value {
                RuntimeGroundValue::Bool(_)
                | RuntimeGroundValue::Int(_)
                | RuntimeGroundValue::Bytes(_)
                | RuntimeGroundValue::String(_)
                | RuntimeGroundValue::Constructor { .. }
                | RuntimeGroundValue::Record { .. } => AbiCarrier::GroundValueCarrier,
            }
        }

        let family = vec![
            ("String", RuntimeGroundValue::String("seeded".to_string())),
            (
                "Constructor",
                RuntimeGroundValue::Constructor {
                    constructor: "ctor:prelude::Unit::MkUnit".to_string(),
                    args: Vec::new(),
                },
            ),
            (
                "Record",
                RuntimeGroundValue::Record {
                    fields: vec![("f".to_string(), RuntimeGroundValue::Bool(true))],
                },
            ),
        ];

        // Every member of the family maps to the ONE carrier.
        for (label, value) in &family {
            assert_eq!(
                carrier_for(value),
                AbiCarrier::GroundValueCarrier,
                "AC-3/C2: a seed ground value of kind {label} did not land on the \
                 single fixed carrier"
            );
        }

        // (2) Invariance: the same closure, seeded with each member in turn.
        let expr = b2r_seed_closure(&["c"], RuntimeExpr::Var(0));
        let mut shapes = Vec::new();
        for (label, value) in &family {
            let mut seed_env = NativeSeedEnvironment::default();
            seed_env.insert("c", value.clone());
            // ⚠ The environment is constructed and bound, and is deliberately
            // NOT threaded into planning -- because planning has no parameter to
            // thread it into. That absence IS the contract.
            assert!(
                seed_env.values.contains_key("c"),
                "AC-3: the {label} seed binding did not materialise, so the \
                 invariance rows below would compare three empty environments"
            );
            let plan = b2r_plan(&expr);
            shapes.push(plan.abi.shapes().expect("shapes"));
        }

        assert_eq!(shapes.len(), 3, "AC-3: the family must have three members");
        assert_eq!(
            shapes[0], shapes[1],
            "AC-3/C3: the descriptor differed between a String and a Constructor \
             seed capture"
        );
        assert_eq!(
            shapes[1], shapes[2],
            "AC-3/C3: the descriptor differed between a Constructor and a Record \
             seed capture"
        );

        // Non-vacuity: the shapes being compared must actually contain a seed
        // capture slot carrying the fixed carrier, or all three are equal
        // because all three are empty.
        let seeded = b2r_plan(&expr);
        assert_eq!(
            b2r_only_capture_slot(&seeded).carrier,
            AbiCarrier::GroundValueCarrier,
            "AC-3: the fixture declares no seed capture slot, so the invariance \
             rows above compare descriptors that never exercised the carrier"
        );
    }

    /// MEASURED: one extra exact continuation input adds one capture slot and
    /// no other slot. A count-only or provenance-wide projection would leave
    /// the slot count unchanged or add a caller-environment tail.
    ///
    /// CLAIMED: AC-2's frame length is exactly ordinary parameters plus exact
    /// continuation captures plus the fixed convention.
    ///
    /// GAP: this is the dormant descriptor projection; Slice 3 still owns
    /// transferring the values at a call site.
    ///
    /// Promise class: durable invariant.
    #[test]
    fn contspec_abi_added_capture_moves_slot_count_by_exactly_one() {
        let plan = contspec_plan();
        let mut base_units = vec![plan.continuation_specializations[0].clone()];
        base_units[0].id = ContinuationSpecializationId(0);
        let mut base = AbiPlane::default();
        install_continuation_specialization_abi(&mut base, &base_units)
            .expect("base descriptor plans");

        let mut added_units = base_units.clone();
        let added_unit = &mut added_units[0];
        let mut added = added_unit
            .key
            .continuation_inputs
            .last()
            .expect("fixture has a continuation input")
            .clone();
        added.ordinal = u32::try_from(added_unit.key.continuation_inputs.len())
            .expect("fixture input count fits");
        added.ordinary_abi_position = added_unit
            .key
            .ordinary_parameters
            .checked_add(added.ordinal)
            .expect("fixture ABI position fits");
        added_unit.key.continuation_inputs.push(added);
        let mut extended = AbiPlane::default();
        install_continuation_specialization_abi(&mut extended, &added_units)
            .expect("extended descriptor plans");

        let base_descriptor = base.continuation_descriptors[0];
        let extended_descriptor = extended.continuation_descriptors[0];
        assert_eq!(
            extended_descriptor.header.captures,
            base_descriptor.header.captures + 1,
            "AC-2: added capture did not add exactly one capture slot"
        );
        assert_eq!(
            extended_descriptor.slots.len,
            base_descriptor.slots.len + 1,
            "AC-2: broken projection produced the wrong unchanged/tail slot count"
        );
        assert_eq!(
            extended_descriptor.header.parameters, base_descriptor.header.parameters,
            "AC-2: adding a capture changed the ordinary parameter prefix"
        );
    }

    /// MEASURED: three independent compile-valid corruptions change the exact
    /// source owner, the ownership/storage lifetime pair, or the closed
    /// referent-affinity set of a runtime-reaching continuation input. Each is
    /// refused by its named D3 comparison.
    ///
    /// CLAIMED: the descriptor cannot launder a planner-projected input into a
    /// plausible slot with different owner, lifetime, or affinity.
    ///
    /// GAP: this checks descriptor authority before activation; Slice 3 must
    /// still prove that lowering consumes this descriptor.
    ///
    /// Promise class: durable mutation proof.
    #[test]
    fn contspec_abi_refuses_owner_lifetime_and_affinity_disagreement() {
        let plan = contspec_plan();

        let mut wrong_owner = plan.abi.clone();
        wrong_owner.continuation_inputs[0].provenance =
            abi::AbiContinuationInputProvenance::EntryAbi {
                source_owner: PredeclaredFunctionId(u32::MAX),
            };
        assert_eq!(
            wrong_owner
                .validate_continuation_specializations(&plan.continuation_specializations)
                .unwrap_err(),
            planner_error("continuation ABI input provenance disagrees with the planner projection")
        );

        // `RT-CONTSRC-PRODUCER-LOCAL` `D3a` — the corruption the bare
        // `source_owner` field could not express, let alone refuse.
        //
        // ⭐ The owner is carried across UNCHANGED and only the domain moves,
        // so ordinal, owner and affinity all still agree. Before `D3a` this
        // record held one `PredeclaredFunctionId` and both domains projected
        // onto it, so this substitution was **not representable** — an ABI
        // authority for an entry-ABI value and one for a producer-local
        // binding in the same owner were the same value. The tag is what makes
        // the swap both expressible and refusable.
        let mut crossed_domain = plan.abi.clone();
        let carried_owner = match crossed_domain.continuation_inputs[0].provenance {
            abi::AbiContinuationInputProvenance::EntryAbi { source_owner } => source_owner,
            abi::AbiContinuationInputProvenance::ProducerLocal { binding_owner } => binding_owner,
        };
        crossed_domain.continuation_inputs[0].provenance =
            abi::AbiContinuationInputProvenance::ProducerLocal {
                binding_owner: carried_owner,
            };
        assert_ne!(
            crossed_domain.continuation_inputs[0].provenance,
            plan.abi.continuation_inputs[0].provenance,
            "the domain swap must change the recorded provenance, or this row measures nothing"
        );
        assert_eq!(
            crossed_domain
                .validate_continuation_specializations(&plan.continuation_specializations)
                .unwrap_err(),
            planner_error("continuation ABI input provenance disagrees with the planner projection")
        );

        let mut wrong_lifetime = plan.abi.clone();
        let descriptor = wrong_lifetime.continuation_descriptors[0];
        let capture = descriptor.slots.start as usize + descriptor.header.parameters as usize;
        wrong_lifetime.continuation_slots[capture].ownership = AbiOwnership::BorrowedForActivation;
        wrong_lifetime.continuation_slots[capture].storage_owner = AbiStorageOwner::ArtifactStatic;
        assert_eq!(
            wrong_lifetime
                .validate_continuation_specializations(&plan.continuation_specializations)
                .unwrap_err(),
            planner_error("continuation ABI input lifetime disagrees with the planner projection"),
            "D3: a runtime-reaching input was accepted with a contradictory durable borrow"
        );

        let mut wrong_affinity = plan.abi.clone();
        let affinity = wrong_affinity.continuation_inputs[0]
            .referent_affinity
            .start as usize;
        let original = wrong_affinity.continuation_affinities[affinity];
        wrong_affinity.continuation_affinities[affinity] = match original {
            BoundaryReferentOwner::NoReferent => BoundaryReferentOwner::InvocationArena,
            BoundaryReferentOwner::PersistentStore | BoundaryReferentOwner::InvocationArena => {
                BoundaryReferentOwner::NoReferent
            }
        };
        assert_eq!(
            wrong_affinity
                .validate_continuation_specializations(&plan.continuation_specializations)
                .unwrap_err(),
            planner_error(
                "continuation ABI input referent affinity disagrees with the planner projection"
            )
        );
    }

    /// MEASURED: exact preflight leaves every continuation-ABI backing vector's
    /// capacity unchanged while each descriptor is appended. Skipping preflight
    /// makes the first append grow storage and reaches the exact D4 refusal.
    ///
    /// CLAIMED: construction and validation add no allocation on the individual
    /// boundary path; all compiler-side backing storage is reserved before that
    /// path starts.
    ///
    /// GAP: capacity growth observes allocations owned by this representation,
    /// not unrelated allocator traffic elsewhere in planning.
    ///
    /// Promise class: durable mutation proof.
    #[test]
    fn contspec_abi_preflight_makes_boundary_construction_allocation_free() {
        let exact = contspec_plan();
        exact
            .abi
            .validate_continuation_specializations(&exact.continuation_specializations)
            .expect("preflighted construction and allocation-free validation pass");

        let expr = Box::leak(Box::new(contspec_nested_fixture()));
        abi::SKIP_CONTINUATION_ABI_PREFLIGHT.with(|mutation| mutation.set(true));
        let allocating = plan_static_transition_graph(expr, &BTreeMap::new());
        abi::SKIP_CONTINUATION_ABI_PREFLIGHT.with(|mutation| mutation.set(false));
        let error = match allocating {
            Ok(_) => panic!("D4: allocating continuation ABI unexpectedly planned"),
            Err(error) => error,
        };
        assert_eq!(
            error,
            planner_error("continuation ABI descriptor construction allocated after preflight"),
            "D4: the compile-valid allocating choice did not make the gate red"
        );
    }

    /// MEASURED: a process root whose result is persistent still projects its
    /// `ValueWord` parameters from exact source owner/positions with
    /// invocation-arena affinity. The compile-valid historical mutation instead
    /// narrows those inputs from the body-result lifetime.
    ///
    /// CLAIMED: D1 projection consumes the exact source-slot environment on the
    /// producer edge; aggregate result lifetime is not a value-flow authority.
    ///
    /// GAP: this is dormant planner data. Slice 2 declares the unit ABI and
    /// Slice 3 transports the planned values.
    #[test]
    fn contspec_parameter_affinity_comes_from_its_exact_source_slot() {
        let expr = Box::leak(Box::new(
            contspec_persistent_result_with_parameter_fixture(),
        ));
        let symbols = crate::NativeProcessSymbols::legacy_prelude();
        let plan = plan_static_transition_graph_with_symbols(
            expr,
            &BTreeMap::new(),
            &symbols,
            AbiRootIngress::Process,
            false,
        )
        .expect("plans");
        let unit = plan
            .continuation_specializations
            .first()
            .expect("one continuation specialization");
        assert_eq!(plan.continuation_specializations.len(), 1);
        let descriptor = plan
            .abi
            .descriptors
            .iter()
            .find(|descriptor| descriptor.function == unit.key.consumer_owner)
            .expect("consumer descriptor");
        assert_eq!(
            occurrence_authority(&plan, descriptor.body_occurrence)
                .expect("consumer result authority")
                .lifetime,
            PlannedReferentLifetime::Persistent,
            "fixture must distinguish body-result lifetime from input affinity",
        );
        assert_eq!(unit.key.continuation_inputs.len(), 2);
        let input = &unit.key.continuation_inputs[0];
        let (source_owner, source_abi_position, source) =
            input.coordinate.expect_entry_abi();
        assert_eq!(source_owner, unit.key.consumer_owner);
        assert_eq!(source_abi_position, 1);
        assert_eq!(source, ContinuationInputSource::Parameter);
        assert_eq!(
            input.referent_affinity,
            vec![
                BoundaryReferentOwner::NoReferent,
                BoundaryReferentOwner::PersistentStore,
                BoundaryReferentOwner::InvocationArena,
            ]
        );

        // ⭐⭐ **`D3b` alias repair — this row's mutation is now REFUSED, not
        // merely observable, and that is a strengthening rather than a change of
        // subject.**
        //
        // The mutation replaces every input's referent affinity with a proxy
        // derived from the consumer's body-result lifetime, narrowing this
        // input's affinity from `[NoReferent, PersistentStore, InvocationArena]`
        // to `[NoReferent, PersistentStore]`. Until the alias repair the row
        // could only *watch* that narrowing reach the projection: the lexical
        // search matched on coordinate alone, so a wrong contract was never
        // consulted and flowed through unchallenged.
        //
        // Eligibility is now exact equality of the **complete** source-slot
        // authority, so the narrowed record no longer matches the authority the
        // seat actually holds, and planning fails closed.
        //
        // ⛔ This is the Architect's required alias control 5 — "the same
        // coordinate with a different carrier, ownership, storage owner, or
        // affinity does not qualify" — discharged by a REAL production
        // perturbation on a landed fixture rather than a synthetic environment.
        // A value whose affinity omits `InvocationArena` genuinely cannot point
        // into invocation-owned storage, so it is a different value wearing the
        // same root identity; indexing it would emit a call carrying an operand
        // of the wrong lifetime class.
        CONTINUATION_PRODUCTION_MUTATION
            .with(|mutation| mutation.set(ContinuationProductionMutation::ResultLifetimeProxy));
        let refusal = plan_static_transition_graph_with_symbols(
            expr,
            &BTreeMap::new(),
            &symbols,
            AbiRootIngress::Process,
            false,
        )
        .map(|_| ())
        .expect_err(
            "the result-lifetime proxy narrows this input's referent affinity, and the D3b \
             alias rule must refuse it; a successful plan means eligibility fell back to \
             matching the coordinate alone",
        );
        CONTINUATION_PRODUCTION_MUTATION
            .with(|mutation| mutation.set(ContinuationProductionMutation::Exact));
        assert!(
            format!("{refusal:?}").contains(
                "under a different carrier, ownership, storage owner or referent affinity"
            ),
            "the refusal must be the contract-mismatch one, not the absent-member or ambiguous \
             one -- those name different defects and would let this row pass while measuring \
             something else: {refusal:?}"
        );
    }

}
