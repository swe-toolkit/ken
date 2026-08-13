//! **`RT-FNSPLIT-B2F` `D1`/`D2` — the target code-unit population.**
//!
//! One closed Cranelift target function per `PredeclaredFunction` in the
//! validated `SemanticOwner` partition, forward-declared as a bundle and then
//! defined against `B2R`'s declared activation frame.
//!
//! ⛔ **This module does not derive the population and must never be made to.**
//! The unit set is `plan.entries` ∪ every `EdgeKind::StaticBody` **target**,
//! MINUS every declaration-owned pair (`RT-DECL-CLOSURE-PORT` `D2a`: a
//! closure-seed transparent declaration's entry seeds no function of its own),
//! seeded and validated by `B2O` and given one `AbiDescriptor` apiece by `B2R`.
//! `StaticTransitionPlan::emittable_units` projects that; this module consumes
//! the projection. In particular it never consults
//! `TransitionKind::ClosureBody`, which is a body's **return successor** and not
//! a unit head — the error that has appeared in successive drafts of the issue
//! file that warns against it.
//!
//! ⚠ The two shared exits (`SemanticOwner::Terminal`, `TrapTerminal`) are not
//! units and receive no target function. They are absent from
//! `emittable_units()` by construction, because `B2R` gives them no descriptor —
//! ⛔ **not** because this module filters them out. There is deliberately no
//! filter here to get wrong.

use super::*;
use super::core::{AmbientBodyAuthority, CheckedFrameFunctionScope};
use crate::cranelift_backend::planning::{FusionComposedEdge, FusionCompositionLayer};

use cranelift_module::FuncId;

/// **`RT-FNSPLIT-B2F` `AC-2` — what the compiled module ACTUALLY contains.**
///
/// ⭐ **This exists because the census pin in `control.rs` cannot answer the
/// question `AC-2` is really about.** That pin counts how many times three
/// spellings occur in seven source files; it is a source-TEXT oracle, so a call
/// split across lines evades it and a mention in a comment inflates it, and in
/// no configuration does it observe an emitted function. The property `B2F` owes
/// is *"this node adds exactly the emitted units its design predicts"*, and the
/// only way to see an emitted unit is to count them at the point of emission.
///
/// Records `(declared, defined)` for the most recent compile on this thread.
/// ⚠ Two numbers rather than one: a bundle that declares `n` and defines `n-1`
/// leaves an undefined symbol, and a single counter cannot tell that from a
/// smaller correct population.
#[cfg(test)]
thread_local! {
    static B2F_UNIT_EMISSION: std::cell::Cell<(usize, usize)> =
        const { std::cell::Cell::new((0, 0)) };
}

/// The `(declared, defined)` unit counts from the most recent compile.
///
/// ⚠ **"Most recent compile" is the whole limitation.** This reading carries no
/// statement about *which* compile produced it, so a compile that fails before
/// reaching the emission seam leaves the previous compile's numbers standing and
/// reads exactly like one that reached the seam and declared that many. Use it
/// only where a single compile is known to have run to emission; for a timing
/// question about a *failing* compile, use the attempt epoch below.
#[cfg(test)]
pub(in crate::cranelift_backend) fn b2f_last_unit_emission() -> (usize, usize) {
    B2F_UNIT_EMISSION.with(std::cell::Cell::get)
}

/// **`RT-FNSPLIT-B2F` `AC-11` clause 3 — the compile-attempt epoch.**
///
/// ⛔⛔ **This exists because the first timing instrument could not distinguish
/// the two outcomes it was built to separate, and reported a confident number
/// for the wrong one.** That version compiled a successful sentinel to force
/// `B2F_UNIT_EMISSION` to a nonzero value, then compiled the failing fixture and
/// read the cell back. But nothing on a pre-emission refusal path *writes* that
/// cell — so the read returned the **sentinel's** `1`, and:
///
/// - "refused before `declare_unit_bundle` ran" (the wanted `0`), and
/// - "declared one unit, then refused during lowering" (the feared `1`)
///
/// ⇒ produce the **identical reading**. ⭐ The in-source comment claimed the
/// sentinel made those cases distinguishable; it made them indistinguishable.
/// A measured `1` was therefore evidence of nothing, in **either** direction.
///
/// ⭐ **The repair is to stamp the reading with the attempt it belongs to**, so
/// a stale value is *detectable as stale* rather than readable as a count.
/// Three outcomes, all distinct:
///
/// | reading | meaning |
/// |---|---|
/// | `None` | ⚠ the compile never reached the emission seam at all — refused earlier still, or never ran. **Not** a zero |
/// | `Some(0)` | ✅ reached the seam, refused **before** any unit was declared — what clause 3 asks for |
/// | `Some(n > 0)` | ⛔ `n` units were already declared when the refusal came — a *later* guarantee, not clause 3's |
///
/// ⛔ **The stamp is written in `core.rs` immediately before
/// `validate_emitted_transfers_are_representable`, NOT inside
/// `declare_unit_bundle`.** Stamping inside the bundle would make `Some(0)`
/// unreachable — the only way to observe the epoch would be to declare a unit,
/// which is the very event the reading is supposed to detect the absence of.
#[cfg(test)]
thread_local! {
    /// The epoch a test opened; bumped once per `b2f_open_compile_attempt`.
    static B2F_ATTEMPT: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    /// The epoch that was live when the emission seam was last reached.
    static B2F_ATTEMPT_AT_SEAM: std::cell::Cell<Option<u64>> =
        const { std::cell::Cell::new(None) };
}

/// Open a fresh compile attempt; the returned epoch identifies it.
///
/// ⚠ Deliberately does **not** clear `B2F_UNIT_EMISSION`: clearing it here would
/// hide a compile that never reached the seam behind a plausible `(0, 0)`, which
/// is the exact confusion this epoch exists to remove.
#[cfg(test)]
pub(in crate::cranelift_backend) fn b2f_open_compile_attempt() -> u64 {
    B2F_ATTEMPT.with(|cell| {
        let next = cell.get() + 1;
        cell.set(next);
        next
    })
}

/// Record that the emission seam was reached, and zero this attempt's counts.
#[cfg(test)]
pub(in crate::cranelift_backend) fn b2f_reached_emission_seam() {
    B2F_ATTEMPT_AT_SEAM.with(|cell| cell.set(Some(B2F_ATTEMPT.with(std::cell::Cell::get))));
    B2F_UNIT_EMISSION.with(|cell| cell.set((0, 0)));
}

/// How many units `epoch`'s compile had declared, or `None` if that compile
/// never reached the emission seam.
#[cfg(test)]
pub(in crate::cranelift_backend) fn b2f_units_declared_in_attempt(epoch: u64) -> Option<usize> {
    if B2F_ATTEMPT_AT_SEAM.with(std::cell::Cell::get) == Some(epoch) {
        Some(B2F_UNIT_EMISSION.with(std::cell::Cell::get).0)
    } else {
        None
    }
}

/// Every target function this artifact declares, keyed by its static identity.
///
/// ⭐ **Keyed by `PredeclaredFunctionId`, never by the iteration ordinal.** The
/// ordinal is an artifact of how this module happened to walk the plane; the id
/// is the planner's identity, and `D4`'s call edges resolve against *that*. A
/// map keyed on position would be an identity alias of exactly the kind `B2O`
/// removed from `SemanticDescriptor` (`semantic_ir.rs:651` records that
/// removal), reintroduced one layer out.
pub(in crate::cranelift_backend) struct UnitBundle {
    functions: BTreeMap<PredeclaredFunctionId, FuncId>,
    /// **`RT-CONTSPEC-ACTIVATE` `D2`** -- one declared target per planned
    /// continuation specialization, keyed by the planner's typed identity.
    ///
    /// Kept as its own map rather than folded into `functions`: a
    /// `ContinuationSpecializationId` is **not** a `PredeclaredFunctionId`, and
    /// admitting one there would alias two identities that the planner keeps
    /// apart. Nothing resolves a continuation by ordinal or by symbol name.
    continuations: BTreeMap<ContinuationSpecializationId, FuncId>,
    /// **`RT-DECL-CLOSURE-PORT` `D5a`** -- one declared target per planned
    /// generated producer execution context.
    ///
    /// A third map for the same reason there is a second: a
    /// `ContinuationContextId` is neither of the other two identities, and the
    /// ruling's "do not cast or alias one ID domain into the other" is enforced
    /// here by there being no map a caller could reach with the wrong key type.
    contexts: BTreeMap<ContinuationContextId, FuncId>,
    /// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2f`** -- one declared target per
    /// installed static continuation fusion region.
    ///
    /// A **fourth** map for the same reason there is a third: a
    /// `StaticContinuationFusionId` is none of the other three identities, and
    /// the standing "do not cast or alias one ID domain into the other" ruling
    /// is enforced here by there being no map a caller could reach with the
    /// wrong key type. In particular it is **not** keyed by the producer's
    /// `PredeclaredFunctionId`: the fused region is a third thing that owns
    /// itself, and keying it by the producer would make the redirect below
    /// resolve to whichever of the two the last writer meant.
    fusions: BTreeMap<StaticContinuationFusionId, FuncId>,
}

impl UnitBundle {
    /// The declared target function for one unit.
    ///
    /// ⛔ `None` is a real answer and the caller must not substitute one of its
    /// own: a unit absent here was never declared, and emitting a call to a
    /// fabricated `FuncId` is the failure this return type exists to make
    /// visible.
    pub(in crate::cranelift_backend) fn function(
        &self,
        unit: PredeclaredFunctionId,
    ) -> Option<FuncId> {
        self.functions.get(&unit).copied()
    }

    /// The declared target for one continuation specialization.
    ///
    /// `None` is a real answer and must not be substituted for: a
    /// specialization absent here was never declared, and resolving a causal
    /// identity to a fabricated `FuncId` is exactly what this return type
    /// exists to make visible.
    pub(in crate::cranelift_backend) fn continuation(
        &self,
        specialization: ContinuationSpecializationId,
    ) -> Option<FuncId> {
        self.continuations.get(&specialization).copied()
    }

    /// The declared target for one generated producer execution context.
    ///
    /// `None` is a real answer, exactly as for the two maps above.
    pub(in crate::cranelift_backend) fn context(
        &self,
        context: ContinuationContextId,
    ) -> Option<FuncId> {
        self.contexts.get(&context).copied()
    }

    /// The declared target for one installed fused region.
    ///
    /// `None` is a real answer, exactly as for the three maps above.
    pub(in crate::cranelift_backend) fn fusion(
        &self,
        fusion: StaticContinuationFusionId,
    ) -> Option<FuncId> {
        self.fusions.get(&fusion).copied()
    }

    /// How many fused-region targets this bundle declares.
    pub(in crate::cranelift_backend) fn fusion_len(&self) -> usize {
        self.fusions.len()
    }

    /// How many continuation targets this bundle declares.
    pub(in crate::cranelift_backend) fn continuation_len(&self) -> usize {
        self.continuations.len()
    }

    /// How many target functions this bundle declares.
    ///
    /// ⚠ This is the **emitted-unit** count, not a source-spelling count. `D8`'s
    /// growth verdict is about this number; the census pin in `control.rs`
    /// counts spellings and cannot see it.
    pub(in crate::cranelift_backend) fn len(&self) -> usize {
        self.functions.len()
    }
}

/// **`RT-FNSPLIT-B2F` `D4` — every cross-owner call edge, resolved to the target
/// function the bundle declared for it.**
///
/// ⭐ **Keyed by the planner's `PredeclaredFunctionId`, resolved to a `FuncId`,
/// and derived from nothing else.** The ordinal `declare_unit_bundle` used to
/// spell a symbol name never enters here; a call edge names its callee by static
/// identity and the bundle answers with the declared target or with `None`.
pub(in crate::cranelift_backend) struct CallEdgeTargets {
    edges: Vec<(PredeclaredFunctionId, ResolvedUnitTarget)>,
}

impl CallEdgeTargets {
    /// The resolved targets of every call emitted **into** `caller`.
    ///
    /// ⚠ Returns an empty iterator for a unit with no outgoing call edges, which
    /// is the common case: most units are leaves.
    pub(in crate::cranelift_backend) fn targets_in(
        &self,
        caller: PredeclaredFunctionId,
    ) -> impl Iterator<Item = &ResolvedUnitTarget> + '_ {
        self.edges
            .iter()
            .filter(move |(from, _)| *from == caller)
            .map(|(_, target)| target)
    }

    /// How many call edges were resolved.
    #[cfg(test)]
    pub(in crate::cranelift_backend) fn len(&self) -> usize {
        self.edges.len()
    }
}

#[derive(Clone)]
pub(in crate::cranelift_backend) struct ResolvedUnitTarget {
    function: FuncId,
    origin: StaticOriginId,
    call_site_origin: StaticOriginId,
    kind: EmittableCallKind,
    header: AbiFrameHeader,
    slots: Vec<AbiSlot>,
    offsets: Vec<u32>,
}

#[derive(Clone)]
pub(in crate::cranelift_backend) struct DeclaredUnitCall {
    pub(in crate::cranelift_backend) function: FuncRef,
    /// The callee's scheduling entry -- `RT-CONTSPEC-ACTIVATE` `D1b` keeps
    /// this as the target origin.
    pub(in crate::cranelift_backend) origin: StaticOriginId,
    /// The callable source body this call is keyed by. Both ends are retained
    /// in the declared record, not just the map key, so a consumer can check
    /// the pair rather than infer it.
    pub(in crate::cranelift_backend) call_site_origin: StaticOriginId,
    pub(in crate::cranelift_backend) header: AbiFrameHeader,
    pub(in crate::cranelift_backend) slots: Vec<AbiSlot>,
    pub(in crate::cranelift_backend) offsets: Vec<u32>,
}

pub(in crate::cranelift_backend) struct DeclaredUnitCalls {
    pub(in crate::cranelift_backend) static_bodies:
        BTreeMap<StaticOriginId, DeclaredUnitCall>,
    pub(in crate::cranelift_backend) declarations:
        BTreeMap<StaticOriginId, DeclaredUnitCall>,
}

impl CallEdgeTargets {
    pub(in crate::cranelift_backend) fn declare_in_func<M: Module>(
        &self,
        caller: PredeclaredFunctionId,
        module: &mut M,
        func: &mut Function,
    ) -> Result<DeclaredUnitCalls, CraneliftBackendError> {
        let mut static_bodies = BTreeMap::new();
        let mut declarations = BTreeMap::new();
        for target in self.targets_in(caller) {
            let call = DeclaredUnitCall {
                function: module.declare_func_in_func(target.function, func),
                origin: target.origin,
                call_site_origin: target.call_site_origin,
                header: target.header,
                slots: target.slots.clone(),
                offsets: target.offsets.clone(),
            };
            let (calls, duplicate) = match target.kind {
                EmittableCallKind::StaticBody => (
                    &mut static_bodies,
                    "one caller has two static-body calls to the same body origin",
                ),
                EmittableCallKind::Declaration => (
                    &mut declarations,
                    "one declaration-reference occurrence has two planner-derived call targets",
                ),
            };
            if calls.insert(target.call_site_origin, call).is_some() {
                return Err(backend_module(duplicate.to_string()));
            }
        }
        Ok(DeclaredUnitCalls {
            static_bodies,
            declarations,
        })
    }
}

/// **`D4` — resolve the derived call edges against the declared bundle, BEFORE
/// any body is defined.**
///
/// ⭐ **This runs between `D1`'s declaration pass and `D2`'s definition pass on
/// purpose, and the position is the point.** A call edge whose callee was never
/// forward-declared is a program that cannot be emitted; discovering that while
/// half the bodies are already defined leaves a partially emitted artifact whose
/// failure mode is a link error or worse. ⇒ Resolving the whole edge set first
/// makes the bundle's completeness a **precondition** of definition rather than
/// a discovery during it.
///
/// ⛔ **`None` from the bundle is a hard error and must never be replaced by a
/// fabricated `FuncId`.** That substitution is the exact failure
/// `UnitBundle::function`'s `Option` return type exists to make visible, and it
/// would emit a call to whatever function happened to share the id.
///
/// **MEASURED:** every `StaticBody` edge the planner validated resolves to a
/// target function the bundle declared, and the resolved population equals the
/// derived population.
/// **CLAIMED:** call sites reference target functions by their **static**
/// identity, with no indirect dispatch on a dynamic property and no runtime
/// lookup that re-derives which code to run from a value.
/// **THE GAP:** ⛔ **this resolves the edges; it does not yet EMIT the call
/// instructions.** A unit body today loads its result slot and returns, because
/// body emission does not descend until `S6` switches `lower_expr`'s consumers
/// over. ⇒ Until then this carries **rejection** authority, not emission
/// authority, and ⛔ the claim above is discharged for *resolution* only. The
/// `direct call rather than call_indirect` half of it has no control yet and is
/// **not** claimed.
///
/// ⛔⛔ **AND THE GAP IS WIDER THAN "no call instruction" — MEASURED, NOT
/// ESTIMATED.** Replacing `bundle.function(edge.callee())` with
/// `bundle.function(edge.caller())` — i.e. resolving every call edge to the
/// **calling** unit instead of the called one, the identity-alias defect
/// `UnitBundle`'s doc comment warns against — leaves the **entire suite green**:
/// 498 + 26 + 14, zero failures.
///
/// ⇒ ⭐ **Which unit an edge resolves to is currently unpinned.** The `FuncRef`
/// is declared in the caller's `Function` and never called, so a wrong target is
/// a reference nobody follows. ⛔ **`S6` must not read this as covered.** The
/// control that closes it is a *behavioural* one — a program whose answer
/// depends on which unit ran — and it cannot exist until the call is emitted.
/// ⚠ `the_resolved_call_edge_population_moves_with_the_program` pins the edge
/// **count** and is blind to the edge's **destination**; those are different
/// claims and only the first has a defender today.
/// **`D4` -- the static-body units, projected by exact body origin.**
///
/// This is a **projection of `emittable_units`**, not a new unit or call-edge
/// population: every entry is a unit the planner already emitted and the
/// bundle already declared, re-keyed by the body origin a static worker
/// binding names. Nothing here mints a unit, an edge, or a descriptor.
///
/// A body origin that appears twice is rejected rather than resolved by
/// last-writer, because a duplicate means two units claim one body and the
/// binding could not name either unambiguously.
pub(in crate::cranelift_backend) struct WorkerTargets {
    /// The EXECUTABLE targets: bodies with a declared `Function` to call.
    by_origin: BTreeMap<StaticOriginId, ResolvedUnitTarget>,
    /// **`D5a` checkpoint 1 -- the TEMPLATE population, over every emittable
    /// unit including the template-only ones.**
    ///
    /// Architect ruling `evt_5a0q3m9tnkh8e`: *"the constructor's raw
    /// identity/arity validation must be separate from the generated context
    /// `FuncRef` used by the call."* This map is that separation made
    /// structural -- it carries the descriptor facts and **no `FuncRef` at
    /// all**, so a template-only body can be validated against its own raw
    /// contract by code that has no way to call it.
    templates: BTreeMap<StaticOriginId, WorkerTemplate>,
}

/// **`D5a` checkpoint 1 -- one raw worker body's descriptor contract.**
///
/// ⛔ Deliberately has no `function: FuncRef` field. `construct_static_worker_
/// binding` reads only `call_site_origin`, `header`, `slots` and `offsets`, so
/// removing the callee from the record it validates against makes "validated
/// the raw contract, called the generated context" a fact about the types
/// rather than a discipline someone has to remember.
#[derive(Clone)]
pub(in crate::cranelift_backend) struct WorkerTemplate {
    pub(in crate::cranelift_backend) origin: StaticOriginId,
    pub(in crate::cranelift_backend) call_site_origin: StaticOriginId,
    pub(in crate::cranelift_backend) header: AbiFrameHeader,
    pub(in crate::cranelift_backend) slots: Vec<AbiSlot>,
    pub(in crate::cranelift_backend) offsets: Vec<u32>,
}

impl WorkerTargets {
    /// Declare every projected target **into one generated function**, and
    /// hand back that function's own `DeclaredUnitCall`s.
    ///
    /// The `FuncRef`s produced here belong to `func` alone. They are minted
    /// per function and never copied between functions -- which is why the
    /// binding stores origins and not a `FuncRef` (`D4`).
    ///
    /// This is also the operation a separately emitted caller uses: it takes
    /// any `Function`, so a caller emitted outside the main loop declares its
    /// own refs through the same route rather than borrowing another's.
    pub(in crate::cranelift_backend) fn declare_in_func<M: Module>(
        &self,
        module: &mut M,
        func: &mut Function,
    ) -> BTreeMap<StaticOriginId, DeclaredUnitCall> {
        self.by_origin
            .iter()
            .map(|(origin, target)| {
                (
                    *origin,
                    DeclaredUnitCall {
                        function: module.declare_func_in_func(target.function, func),
                        origin: target.origin,
                        call_site_origin: target.call_site_origin,
                        header: target.header,
                        slots: target.slots.clone(),
                        offsets: target.offsets.clone(),
                    },
                )
            })
            .collect()
    }

    /// The raw template contract for every emittable body, executable or not.
    pub(in crate::cranelift_backend) fn templates(
        &self,
    ) -> &BTreeMap<StaticOriginId, WorkerTemplate> {
        &self.templates
    }
}

/// Project the already-validated emittable units by exact body origin.
pub(in crate::cranelift_backend) fn resolve_worker_targets(
    plan: &StaticTransitionPlan<'_>,
    bundle: &UnitBundle,
) -> Result<WorkerTargets, CraneliftBackendError> {
    let mut templates: BTreeMap<StaticOriginId, WorkerTemplate> = BTreeMap::new();
    // The TEMPLATE population is every emittable unit -- `D5a` checkpoint 1
    // keeps the raw worker's descriptor and source binding whether or not it
    // still receives a `Function`.
    // `D5a` checkpoint 4 step 3 -- the reaching mutation for "unchanged raw
    // worker ABI". Reading the EXECUTABLE population here instead of the
    // emittable one is the "template-only means deleted" mistake: it drops the
    // superseded body's descriptor while every consumer of that descriptor
    // remains. ⛔ The two populations are otherwise identical on a program with
    // no retarget, which is why this only became measurable at checkpoint 4.
    #[cfg(test)]
    let template_population =
        if crate::cranelift_backend::lowering::d5a_route_mutation()
            == crate::cranelift_backend::lowering::D5aRouteMutation::DropSupersededWorkerTemplates
        {
            crate::cranelift_backend::lowering::record_d5a_route_application();
            plan.executable_units()?
        } else {
            plan.emittable_units()?
        };
    #[cfg(not(test))]
    let template_population = plan.emittable_units()?;
    for unit in template_population {
        let (offsets, frame_bytes) = unit.slot_offsets()?;
        if frame_bytes != unit.header().frame_bytes {
            return Err(backend_module(
                "worker template frame size disagrees with its slot run".to_string(),
            ));
        }
        let origin = unit.body_occurrence();
        if templates
            .insert(
                origin,
                WorkerTemplate {
                    origin,
                    call_site_origin: origin,
                    header: unit.header(),
                    slots: unit.slots().to_vec(),
                    offsets,
                },
            )
            .is_some()
        {
            return Err(backend_module(
                "two emittable units claim the same body origin, so no worker template could \
                 name either unambiguously"
                    .to_string(),
            ));
        }
    }
    let mut by_origin: BTreeMap<StaticOriginId, ResolvedUnitTarget> = BTreeMap::new();
    for unit in plan.executable_units()? {
        let function = bundle.function(unit.function()).ok_or_else(|| {
            backend_module(
                "a planned unit has no forward-declared function to project as a worker target"
                    .to_string(),
            )
        })?;
        let (offsets, frame_bytes) = unit.slot_offsets()?;
        if frame_bytes != unit.header().frame_bytes {
            return Err(backend_module(
                "worker target frame size disagrees with its slot run".to_string(),
            ));
        }
        let origin = unit.body_occurrence();
        let target = ResolvedUnitTarget {
            function,
            origin,
            call_site_origin: origin,
            kind: EmittableCallKind::StaticBody,
            header: unit.header(),
            slots: unit.slots().to_vec(),
            offsets,
        };
        if by_origin.insert(origin, target).is_some() {
            return Err(backend_module(
                "two emittable units claim the same body origin, so no worker binding could                  name either unambiguously"
                    .to_string(),
            ));
        }
    }
    Ok(WorkerTargets {
        by_origin,
        templates,
    })
}

/// **`RT-DECL-CLOSURE-PORT` `D5` — the checked-call closeout ledger.**
///
/// One entry per checked same-SCC recursive call that reached a real direct
/// call to a declaration-owned unit, keyed by the `call_template_id` the
/// oriented plan issued.
///
/// ⭐ **The three populations are read independently.** `planned` is the
/// oriented plan's own `recursive_calls` identities; `consumed` is what the
/// affine marker machinery actually took; `emitted` is recorded only after the
/// `Inst` exists, from the emitted call itself. Set equality across the three
/// is what "every planned checked call became exactly one correct direct call,
/// and no other checked call was emitted" means.
///
/// ⛔ **Sets, not counts.** Two populations of the same size can differ, and a
/// length comparison would pass for one that swapped a template for another.
///
/// ⛔ **This closeout restates none of D5's other laws.** Interface, segment,
/// frame-template, occurrence-fingerprint, ABI descriptor, SCC, admission and
/// input-order checks all have their own authorities and keep them; duplicating
/// one here would put a second copy in a file where the two can disagree.
///
/// ⚠ **Scope, stated rather than left to be discovered.** The ledger is opened
/// and closed by `define_unit_bodies`, which runs **only** under
/// `BodyEmissionAuthority::FunctionizedUnits`. Production selects
/// `RecursiveDescent` until `D6` retires the `TransparentDeclarationClosure`
/// residual, so today this gate is reachable only under the `cfg(test)` selector
/// witness — the same reachability every other `D5` law has, and the thing `D6`
/// changes. It is live production code on the lane it guards, not a test hook.
#[derive(Debug, Default)]
pub(in crate::cranelift_backend) struct CheckedCallLedger {
    planned: BTreeSet<u64>,
    emitted: BTreeMap<u64, CheckedCallRecord>,
}

/// One emitted checked call, bound to the exact occurrence and target the
/// planner resolved for it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct CheckedCallRecord {
    pub(in crate::cranelift_backend) reference: StaticOriginId,
    pub(in crate::cranelift_backend) target: StaticOriginId,
    /// The callee **decoded out of the emitted instruction**.
    pub(in crate::cranelift_backend) callee: cranelift_codegen::ir::FuncRef,
    /// The callee the planner-resolved target record carries. ⚠ Both are
    /// `FuncRef`s minted into the same defining function, so comparing them is
    /// lawful and is a comparison of two independently produced facts — one
    /// read from the CLIF, one from the resolved `DeclaredUnitCall`.
    pub(in crate::cranelift_backend) resolved: cranelift_codegen::ir::FuncRef,
}

impl CheckedCallLedger {
    /// ⚠ `planned` is taken **directly** from `plan.recursive_calls` — that IS
    /// the exact domain of same-SCC checked calls, so no classifier and no
    /// whitelist stands between the plan and this set.
    pub(in crate::cranelift_backend) fn open(
        plan: Option<&crate::OrientedSubcontinuationPlanV1>,
    ) -> Self {
        Self {
            planned: plan
                .map(|plan| {
                    plan.recursive_calls
                        .iter()
                        .map(|call| call.call_template_id)
                        .collect()
                })
                .unwrap_or_default(),
            emitted: BTreeMap::new(),
        }
    }

    /// Record one emitted checked call. ⛔ Called only **after** the `Inst`
    /// exists, so a template reaches this set only once its call is real.
    pub(in crate::cranelift_backend) fn record_emitted(
        &mut self,
        call_template_id: u64,
        record: CheckedCallRecord,
    ) -> Result<(), CraneliftBackendError> {
        if self.emitted.insert(call_template_id, record).is_some() {
            return Err(backend_module(
                "one checked recursive call template emitted more than one declaration-unit call"
                    .to_string(),
            ));
        }
        Ok(())
    }

    /// **Planned = consumed = emitted, before the artifact is published.**
    ///
    /// `consumed` is the affine marker machinery's own set, passed in rather
    /// than mirrored here, so the two cannot drift.
    pub(in crate::cranelift_backend) fn close(
        self,
        consumed: &BTreeSet<u64>,
    ) -> Result<(), CraneliftBackendError> {
        let emitted = self.emitted.keys().copied().collect::<BTreeSet<_>>();
        for (name, set) in [("consumed", consumed), ("emitted", &emitted)] {
            if *set != self.planned {
                let missing = self.planned.difference(set).count();
                let extra = set.difference(&self.planned).count();
                return Err(backend_module(format!(
                    "the {name} checked recursive call population does not equal the planned one:                      {missing} planned templates absent, {extra} unplanned templates present"
                )));
            }
        }
        // Each emitted call's ACTUAL callee against its exact resolved target.
        // ⛔ The callee is the one decoded from the emitted instruction; a
        // tuple that disagrees here is a call that went somewhere the planner
        // did not resolve.
        for (call_template_id, record) in &self.emitted {
            if record.callee != record.resolved {
                return Err(backend_module(format!(
                    "checked recursive call template {call_template_id} emitted a call to \
                     {:?} but the target resolved for occurrence {:?} is {:?} ({:?})",
                    record.callee, record.reference, record.resolved, record.target
                )));
            }
        }
        Ok(())
    }
}

pub(in crate::cranelift_backend) fn resolve_call_edges(
    plan: &StaticTransitionPlan<'_>,
    bundle: &UnitBundle,
) -> Result<CallEdgeTargets, CraneliftBackendError> {
    // `D5a` checkpoint 1: the edges that SURVIVE the retarget. An edge into a
    // template-only body is the seeding edge whose realization moved to a
    // generated context; resolving it would demand a `FuncId` for a unit with
    // no emitted `Function`, and `bundle.function`'s `None` is a real answer
    // rather than a prompt to fabricate one.
    let derived = plan.executable_call_edges()?;
    // `RT-CONTSPEC-ACTIVATE` `D1b`: the exact-set source-body binding, joined
    // on validated caller + callee scheduling entry. A `StaticBody` edge's
    // resolved `call_site_origin` becomes the callable SOURCE BODY; the
    // scheduling entry stays the target origin.
    let source_bindings: BTreeMap<(PredeclaredFunctionId, StaticOriginId), StaticOriginId> = plan
        .static_body_source_bindings()?
        .into_iter()
        .map(|(caller, source_body, entry)| ((caller, entry), source_body))
        .collect();
    let mut edges = Vec::with_capacity(derived.len());
    for edge in derived {
        let target = bundle.function(edge.callee()).ok_or_else(|| {
            backend_module("a call edge names a unit that was never forward-declared".to_string())
        })?;
        let unit = plan
            .emittable_units()?
            .into_iter()
            .find(|unit| unit.function() == edge.callee())
            .ok_or_else(|| backend_module("call edge callee has no abi descriptor".to_string()))?;
        if unit.entry_origin() != edge.callee_origin() {
            return Err(backend_module(
                "call edge callee origin disagrees with its abi descriptor".to_string(),
            ));
        }
        let (offsets, frame_bytes) = unit.slot_offsets()?;
        if frame_bytes != unit.header().frame_bytes {
            return Err(backend_module(
                "call edge target frame size disagrees with its slot run".to_string(),
            ));
        }
        edges.push((
            edge.caller(),
            ResolvedUnitTarget {
                function: target,
                origin: edge.callee_origin(),
                call_site_origin: match edge.kind() {
                    EmittableCallKind::StaticBody => *source_bindings
                        .get(&(edge.caller(), edge.callee_origin()))
                        .ok_or_else(|| {
                            backend_module(
                                "a static body call edge has no D1b source-body binding"
                                    .to_string(),
                            )
                        })?,
                    EmittableCallKind::Declaration => edge.call_site_origin(),
                },
                kind: edge.kind(),
                header: unit.header(),
                slots: unit.slots().to_vec(),
                offsets,
            },
        ));
    }
    #[cfg(test)]
    B2F_CALL_EDGE_RESOLUTION.with(|cell| cell.set(edges.len()));
    Ok(CallEdgeTargets { edges })
}

/// **`RT-DECL-CLOSURE-PORT` `D5`** — one mutation of the function-local
/// declared-call copy, per axis the ABI reconciliation claims to hold.
///
/// ⚠ Every variant leaves the plan's descriptor untouched. `Exact` is the
/// identity, and a control that ran only `Exact` would be asserting its own
/// setup ([[a-mutation-control-with-unwrap-or-exact-is-the-identity]]).
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D5DeclaredCallMutation {
    Exact,
    /// Phase: the word class a slot is carried in.
    Carrier,
    /// Owner: whether the callee owns or borrows the slot.
    Ownership,
    /// Owner: whose storage the slot addresses.
    StorageOwner,
    /// The slot's position within its own kind-run.
    Ordinal,
    /// The declared frame size.
    Header,
    /// Where a slot sits in the frame.
    Offsets,
    /// The call resolves some other unit's record — the wrong-target class.
    Retarget,
}

#[cfg(test)]
thread_local! {
    static D5_DECLARED_CALL_MUTATION: std::cell::Cell<D5DeclaredCallMutation> =
        const { std::cell::Cell::new(D5DeclaredCallMutation::Exact) };
}

/// Run `body` with one declared-call mutation installed, restoring `Exact` on
/// the way out **including on panic** — a control asserts inside, and a leak
/// would silently mutate every later compile on this thread.
#[cfg(test)]
pub(in crate::cranelift_backend) fn with_d5_declared_call_mutation<T>(
    mutation: D5DeclaredCallMutation,
    body: impl FnOnce() -> T,
) -> T {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            D5_DECLARED_CALL_MUTATION.with(|cell| cell.set(D5DeclaredCallMutation::Exact));
        }
    }
    D5_DECLARED_CALL_MUTATION.with(|cell| cell.set(mutation));
    let _restore = Restore;
    body()
}

#[cfg(test)]
fn d5_mutate_declared_calls(calls: &mut BTreeMap<StaticOriginId, DeclaredUnitCall>) {
    let mutation = D5_DECLARED_CALL_MUTATION.with(std::cell::Cell::get);
    if mutation == D5DeclaredCallMutation::Exact {
        return;
    }
    // ⚠ The retarget needs a DIFFERENT record to point at, so it is taken
    // before the loop below borrows the map mutably.
    let other = calls.values().next().cloned();
    for call in calls.values_mut() {
        match mutation {
            D5DeclaredCallMutation::Exact => {}
            D5DeclaredCallMutation::Carrier => {
                if let Some(slot) = call.slots.iter_mut().find(|slot| {
                    matches!(slot.kind, AbiSlotKind::Parameter | AbiSlotKind::Capture)
                }) {
                    slot.carrier = match slot.carrier {
                        AbiCarrier::ValueWord => AbiCarrier::GroundValueCarrier,
                        _ => AbiCarrier::ValueWord,
                    };
                }
            }
            D5DeclaredCallMutation::Ownership => {
                if let Some(slot) = call.slots.iter_mut().find(|slot| {
                    matches!(slot.kind, AbiSlotKind::Parameter | AbiSlotKind::Capture)
                }) {
                    slot.ownership = match slot.ownership {
                        AbiOwnership::OwnedByFrame => AbiOwnership::BorrowedForActivation,
                        _ => AbiOwnership::OwnedByFrame,
                    };
                }
            }
            D5DeclaredCallMutation::StorageOwner => {
                if let Some(slot) = call.slots.iter_mut().find(|slot| {
                    matches!(slot.kind, AbiSlotKind::Parameter | AbiSlotKind::Capture)
                }) {
                    slot.storage_owner = match slot.storage_owner {
                        AbiStorageOwner::ActivationFrame => AbiStorageOwner::PersistentStore,
                        _ => AbiStorageOwner::ActivationFrame,
                    };
                }
            }
            D5DeclaredCallMutation::Ordinal => {
                if let Some(slot) = call.slots.iter_mut().find(|slot| {
                    matches!(slot.kind, AbiSlotKind::Parameter | AbiSlotKind::Capture)
                }) {
                    slot.ordinal = slot.ordinal.wrapping_add(1);
                }
            }
            D5DeclaredCallMutation::Header => {
                call.header.frame_bytes = call.header.frame_bytes.wrapping_add(8);
            }
            D5DeclaredCallMutation::Offsets => {
                if let Some(offset) = call.offsets.first_mut() {
                    *offset = offset.wrapping_add(8);
                }
            }
            D5DeclaredCallMutation::Retarget => {
                if let Some(other) = other.clone() {
                    if other.origin != call.origin {
                        *call = other;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
thread_local! {
    /// How many call edges the most recent compile resolved.
    ///
    /// ⚠ Same limitation as [`b2f_last_unit_emission`]: it names no attempt, so
    /// read it only where one compile is known to have run to this seam.
    static B2F_CALL_EDGE_RESOLUTION: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The resolved call-edge count from the most recent compile.
#[cfg(test)]
pub(in crate::cranelift_backend) fn b2f_last_call_edge_resolution() -> usize {
    B2F_CALL_EDGE_RESOLUTION.with(std::cell::Cell::get)
}

/// The uniform internal call ABI for every target unit:
/// `(frame_ptr, services_ptr) -> i64`.
///
/// ⭐ **This is what "one fixed call-ABI scheme, not one fixed byte size"
/// means.** Every unit shares this signature; what varies per origin is the
/// *frame layout* the pointer addresses, which `B2R` declares per unit in
/// `AbiFrameHeader` + the slot run. ⛔ Reading "fixed frame" as one universal
/// byte size is the error that would reintroduce a boxed `Value` nobody asked
/// for, and `B2R` says so explicitly.
///
/// ⚠ The signature takes **no program-derived parameter**, which is the same
/// structural guarantee `AC-G0` accepts for `emit_native_int_local_graph`:
/// making a unit's *signature* vary with the program would require a visible
/// change here, so the compiler forbids that growth mode rather than a test
/// detecting it.
pub(super) fn unit_signature<M: Module>(module: &M) -> cranelift_codegen::ir::Signature {
    let mut sig = module.make_signature();
    sig.params
        .push(AbiParam::new(module.target_config().pointer_type()));
    sig.params
        .push(AbiParam::new(module.target_config().pointer_type()));
    sig.returns.push(AbiParam::new(types::I64));
    sig
}

/// **`D1` — forward-declare the whole bundle before any body is defined.**
///
/// ⭐ **The bundle is declared in one pass on purpose.** A unit's body may call
/// any other unit (that is what `D4`'s cross-owner call edges are), so a
/// declare-and-define-in-one-pass loop would be unable to emit a call to a unit
/// it has not reached yet. Declaring every signature first makes the call graph
/// order-independent, which is why the frame words `D1` as *"forward-declare the
/// whole bundle first, then define each body."*
pub(in crate::cranelift_backend) fn declare_unit_bundle<M: Module>(
    module: &mut M,
    plan: &StaticTransitionPlan<'_>,
) -> Result<UnitBundle, CraneliftBackendError> {
    let sig = unit_signature(module);
    let mut functions = BTreeMap::new();
    // `D5a` checkpoint 1: the EXECUTABLE population, not the template one. ⛔ A
    // template-only raw worker must not be declared here -- declaring it and
    // then not defining it is the undefined phantom the ruling names, and it
    // would falsify the declared/defined census below.
    for (ordinal, unit) in plan.executable_units()?.into_iter().enumerate() {
        // The symbol carries the dense ordinal purely so the linker sees
        // distinct names. ⛔ It is NOT an identity: nothing resolves a unit by
        // parsing this string, and `functions` is keyed by the planner's id.
        let name = format!("ken_unit_{ordinal}");
        let id = module
            .declare_function(&name, Linkage::Local, &sig)
            .map_err(|err| backend_module(err.to_string()))?;
        if functions.insert(unit.function(), id).is_some() {
            // ⛔ Fails closed rather than overwriting. `B2R` gives exactly one
            // descriptor per `PredeclaredFunction`, so a duplicate here means
            // the plane disagrees with that invariant, and silently keeping the
            // last one would emit a bundle whose call edges resolve to the
            // wrong body.
            return Err(backend_module(
                "two abi descriptors claim one predeclared function unit".to_string(),
            ));
        }
    }
    // `RT-CONTSPEC-ACTIVATE` `D2` -- forward-declare one target per planned
    // continuation specialization, before any body is defined. The symbol
    // carries a dense ordinal only so the linker sees distinct names; the map
    // is keyed by the planner's typed identity, never by that string.
    // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — over `O_t`, the ORDINARY residual
    // targets. A fusion-local target's selected body is lowered at its exact
    // call edge and never becomes a callable `Function`, so declaring one here
    // would mint a symbol nothing defines -- the undefined phantom the
    // executable/template split above exists to prevent, arriving by a second
    // route.
    let mut continuations = BTreeMap::new();
    for (ordinal, unit) in plan.ordinary_continuation_targets()?.into_iter().enumerate() {
        let name = format!("ken_continuation_{ordinal}");
        let id = module
            .declare_function(&name, Linkage::Local, &sig)
            .map_err(|err| backend_module(err.to_string()))?;
        if continuations.insert(unit, id).is_some() {
            return Err(backend_module(
                "two continuation descriptors claim one planned specialization".to_string(),
            ));
        }
    }
    // `RT-DECL-CLOSURE-PORT` `D5a` -- forward-declare one target per planned
    // generated producer execution context, in the same pre-definition pass and
    // for the same reason: a context is called from the enclosing
    // specialization's body, which is defined below.
    let mut contexts = BTreeMap::new();
    for (ordinal, context) in plan.continuation_contexts()?.into_iter().enumerate() {
        let name = format!("ken_continuation_context_{ordinal}");
        let id = module
            .declare_function(&name, Linkage::Local, &sig)
            .map_err(|err| backend_module(err.to_string()))?;
        if contexts.insert(context.id(), id).is_some() {
            return Err(backend_module(
                "two generated context descriptors claim one planned context".to_string(),
            ));
        }
    }
    // `RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` -- forward-declare one target per
    // installed fused region, in the same pre-definition pass and for the same
    // reason as the three above: the fused function is called from the
    // consumer's body through the redirected invocation, and that body is
    // defined below.
    //
    // Driven by `continuation_fusions()`, which is the join over the plane
    // AND its ABI arena. Iterating the plane alone would declare a target for a
    // region with no frame contract, and the definition pass would then have
    // nothing to lower it against.
    let mut fusions = BTreeMap::new();
    for (ordinal, fusion) in plan.continuation_fusions()?.into_iter().enumerate() {
        let name = format!("ken_static_continuation_fusion_{ordinal}");
        let id = module
            .declare_function(&name, Linkage::Local, &sig)
            .map_err(|err| backend_module(err.to_string()))?;
        if fusions.insert(fusion.id(), id).is_some() {
            return Err(backend_module(
                "two static continuation fusion descriptors claim one installed region".to_string(),
            ));
        }
    }
    #[cfg(test)]
    B2F_UNIT_EMISSION.with(|cell| cell.set((functions.len(), 0)));
    Ok(UnitBundle {
        functions,
        continuations,
        contexts,
        fusions,
    })
}

/// **`RT-CONTSPEC-ACTIVATE` `D2` — resolve every projected causal identity to
/// its typed declared target.**
///
/// Runs after declaration and before any body is defined, for the same reason
/// `resolve_call_edges` does: a causal edge whose target was never declared is
/// a program that cannot be emitted, and discovering that while half the
/// bodies exist leaves a partially emitted artifact.
///
/// The join is by the identity's `target()` alone. ⛔ Nothing here parses a
/// symbol name, indexes by ordinal, or aliases a `ContinuationSpecializationId`
/// to a `PredeclaredFunctionId` — a missing target rejects.
pub(in crate::cranelift_backend) fn resolve_continuation_targets(
    plan: &StaticTransitionPlan<'_>,
    bundle: &UnitBundle,
) -> Result<BTreeMap<ContinuationCallIdentity, FuncId>, CraneliftBackendError> {
    let mut resolved = BTreeMap::new();
    // **`D3` — the ORDINARY residual domain `O`, from the one plan-authoritative
    // accessor.** A fusion-local identity omits its target resolution entirely
    // (Architect `evt_48rwarx25pj2p` §3), so it must not be looked up here: its
    // target has no forward-declared `Function` to resolve to, and asking would
    // raise the never-declared refusal below for an identity that is *lawfully*
    // absent. Narrowing the INPUT is what keeps that refusal meaningful for the
    // ordinary population instead of weakening it to tolerate an absence.
    for identity in plan.ordinary_continuation_call_identities()? {
        let target = bundle.continuation(identity.target()).ok_or_else(|| {
            backend_module(
                "a projected causal identity names a continuation specialization that was never \
                 forward-declared"
                    .to_string(),
            )
        })?;
        if resolved.insert(identity, target).is_some() {
            return Err(backend_module(
                "two projected causal identities collide; the planner mints one call token per \
                 ruled recursive position, so this is a key-arity defect rather than a \
                 double-resolution"
                    .to_string(),
            ));
        }
    }
    Ok(resolved)
}

/// **`RT-DECL-CLOSURE-PORT` `D5a`** — one entry of a continuation
/// specialization body's case environment, named by **where its operand comes
/// from** rather than by the operand.
///
/// ⭐ The whole property under repair here is an **order**, so the order is
/// what this type makes observable. Cranelift `Value`s cannot be synthesized
/// without a live `FunctionBuilder`, so a control written against the assembled
/// operands could only ever re-run the pipeline and read a refusal; a control
/// written against this plan states the binding law directly.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum ContinuationCaseBinderSource {
    /// The projected `StaticWorker`, standing for one recursive field's
    /// **induction hypothesis**.
    ///
    /// `D6a`: this binding takes the
    /// [`StaticWorkerCallRoute::GeneratedContext`] route -- the planner-issued
    /// execution context, which appends this frame's continuation-input suffix
    /// -- **iff** the planner issued such a context for this
    /// `(specialization, worker body)` pair and this unit resolved it.
    /// Otherwise it lawfully takes [`StaticWorkerCallRoute::RawWorker`], like
    /// every pre-`D5a` specialization, and appends nothing.
    InductionHypothesis,
    /// The ordinary-envelope operand at this index. ⛔ An index into the
    /// envelope, never a constructor source position -- the two coincide only
    /// when no `WorkerCapture` role precedes the field.
    Ordinary(usize),
    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D6a`** -- the selected **recursive
    /// constructor argument**, at its own constructor source position.
    ///
    /// ⭐ This is a compiler-only member. It is not a new source occurrence,
    /// continuation input, ABI slot, carrier, tag or runtime descriptor: it is
    /// the *same closure* the induction hypothesis names. The unit already
    /// carries every fact needed to build it -- the closure occurrence, body,
    /// declared arity, ordered capture provenance and the worker-capture
    /// operands -- so nothing crosses the ABI to represent it.
    ///
    /// It carries [`StaticWorkerCallRoute::RawWorker`] **unconditionally**: the
    /// source scope binds the closure itself, so there is nothing to condition
    /// on. ⛔ That is *not* a claim that it differs in route from the induction
    /// hypothesis beside it. In a unit that resolved no generated context the
    /// hypothesis lawfully carries `RawWorker` too, and the two members are
    /// then separated by their positions in the run rather than by their
    /// routes. See [`StaticWorkerCallRoute`] for the asymmetric law.
    ///
    /// ⛔ Before `D6a` this position was **skipped**, and the induction
    /// hypothesis silently stood in for the argument as well. That is a wrong
    /// program, not a missing one: every later binder shifted down by one, so
    /// the case body's outer-frame references landed one slot early.
    SelectedRecursiveArgument { source_position: u32 },
    /// The continuation input at this ordinal.
    ContinuationInput(usize),
}

/// **`RT-DECL-CLOSURE-PORT` `D5a` — the ruled computational-case binding law,
/// as a plan.**
///
/// The established law, identical in the specialized and carried paths
/// (`lower_computational_match_value_composed` builds the same three segments
/// from `induction_hypotheses`, `extend_specialized(args)`, then the frame
/// environment):
///
/// ```text
/// [IH bindings, recursive-position order REVERSED]
///   ++ [ALL constructor arguments, source order]
///   ++ [frame/tail environment]
/// ```
///
/// ⭐ **`RT-CONTSRC-PRODUCER-LOCAL` `D6a` -- "ALL" is load-bearing and was the
/// defect.** The middle segment covers **every** constructor argument in source
/// order, the selected recursive one included. The pre-`D6a` construction
/// *replaced* the selected recursive argument with its own induction
/// hypothesis: it emitted the IH in segment 1 and then skipped that position in
/// segment 2, so a case body with one recursive field and one outer reference
/// got a two-member run where the source scope has three. Every binder after
/// the skipped position was off by one, which is why the measured symptom was
/// an out-of-range `Var` at the *tail* (`Var: no runtime binding for index 2`)
/// rather than anything at the position actually omitted.
///
/// ⭐ **`recursive_position` is a constructor SOURCE-FIELD coordinate, not a
/// lexical environment index.** Reading it as the latter is the exact defect
/// this function exists to prevent: it placed the worker at environment slot
/// `recursive_position`, which for a nonzero position moves the induction
/// hypothesis out of its established lexical prefix and rebinds `Var(0)` to an
/// ordinary field. The measured consequence on `px8tr_nested_post_effect` was
/// `Unsupported(Call, "callee is not a closure")` -- `Var(0)` reading a `Unit`.
///
/// The specialization eliminates **one** selected recursive callable, so its
/// projected `StaticWorker` is that position's IH-prefix binding.
///
/// The selected recursive field is **absent from the ordinary envelope** --
/// [`ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField`] is by
/// construction the population that excludes it. ⛔ `D6a`: that is a fact about
/// the *envelope*, and reading it as a fact about the *binder run* is the
/// defect. The field has no envelope operand because it needs none: it is a
/// compiler-only [`ContinuationCaseBinderSource::SelectedRecursiveArgument`],
/// built from the unit's own worker provenance. So the run's length is
/// `argument_binders + recursive_positions.len()` -- one IH per recursive
/// position, then one member per constructor argument -- which is exactly the
/// binder count the planner's own demand walk uses for this case.
///
/// ⛔ Worker-capture `Parameter` slots are **not** case binders. They construct
/// the `StaticWorker`; they never enter this run.
///
/// Every gap is a hard stop rather than a hole: a gap-filled run would silently
/// shift every later binder, which is a wrong program rather than a refused one.
pub(super) fn continuation_case_binder_run(
    argument_binders: usize,
    recursive_positions: &[usize],
    worker_recursive_position: u32,
    envelope: &[ContinuationOrdinaryEnvelopeRole],
    continuation_inputs: usize,
) -> Result<Vec<ContinuationCaseBinderSource>, CraneliftBackendError> {
    for position in recursive_positions.iter().copied() {
        if position >= argument_binders {
            return Err(backend_module(format!(
                "the selected case names a recursive position {position} outside its own \
                 {argument_binders}-binder run"
            )));
        }
    }
    let worker_position = usize::try_from(worker_recursive_position).map_err(|_| {
        backend_module(
            "the ruled recursive position exceeds this platform's index width".to_string(),
        )
    })?;
    if !recursive_positions.contains(&worker_position) {
        return Err(backend_module(format!(
            "the ruled recursive position {worker_position} is not among the selected case's \
             recursive positions, so the projected worker stands for no induction hypothesis"
        )));
    }

    // **`D6c` — THE CHECKED TOTAL, computed ONCE and reused.**
    //
    // ⛔ Chained `checked_add` with a typed refusal on overflow: no panic and no
    // wrapping. A wrapped total would understate the run's length and let the
    // sealed-run cardinality check below pass on a run it should refuse, which
    // is the one failure mode this postcondition exists to prevent.
    //
    // ⭐ The SAME value serves the allocation's capacity and the final
    // cardinality. Computing it twice would let the two drift, and the check
    // would then be comparing the run against a total the builder never used.
    let sealed_total = recursive_positions
        .len()
        .checked_add(argument_binders)
        .and_then(|partial| partial.checked_add(continuation_inputs))
        .ok_or_else(|| {
            backend_module(format!(
                "this case's binder run would hold {} induction hypotheses + {argument_binders} \
                 constructor arguments + {continuation_inputs} continuation inputs, which \
                 overflows this platform's index width",
                recursive_positions.len()
            ))
        })?;
    let mut run = Vec::with_capacity(sealed_total);

    // Segment 1 -- the IH prefix, recursive positions reversed.
    //
    // ⚠ **The reversal is not observable on any case this mechanism accepts.**
    // A specialization projects exactly one worker, so a second recursive
    // position has no IH to bind and hard-stops below; with one position,
    // reversed and forward order coincide. It is written as the law states it
    // rather than collapsed to the single-position case, so that admitting a
    // second worker later is a change to the projection and not to this order.
    // `D6c` — FABRICATED AVAILABILITY, under test only. A second recursive
    // position is claimed for this case, which the specialization projects no
    // worker for. ⛔ The claim is added to the segment's own input rather than
    // to its output: the loop below is untouched, so what refuses is the
    // production guard that owns availability, not a rewritten loop.
    #[cfg(test)]
    let fabricated: Vec<usize>;
    #[cfg(test)]
    let recursive_positions = if crate::cranelift_backend::lowering::d6c_selection_mutation()
        == crate::cranelift_backend::lowering::D6cSelectionMutation::FabricatedAvailability
    {
        crate::cranelift_backend::lowering::record_d6c_selection_application();
        fabricated = recursive_positions
            .iter()
            .copied()
            .chain(std::iter::once(worker_position.wrapping_add(1)))
            .collect();
        fabricated.as_slice()
    } else {
        recursive_positions
    };

    for position in recursive_positions.iter().rev().copied() {
        if position != worker_position {
            return Err(backend_module(format!(
                "the selected case has a recursive position {position} that the continuation \
                 specialization projects no worker for, so its induction-hypothesis prefix cannot \
                 be built"
            )));
        }
        run.push(ContinuationCaseBinderSource::InductionHypothesis);
    }

    // Segment 2 -- ALL the constructor arguments in SOURCE order.
    //
    // A nonrecursive field takes its operand from its own envelope role. A
    // recursive field takes the compiler-only `SelectedRecursiveArgument`
    // member: it is the same closure the IH prefix names, bound at a second
    // environment position and reached by its own call route, so it needs no
    // envelope operand and no ABI slot.
    //
    // ⛔ "Differing only in call route" would overstate it. The routes differ
    // only where the planner issued a generated context; where it issued none,
    // both members carry `RawWorker` and the difference is *which position of
    // the run they occupy* -- which is precisely the difference this segment
    // exists to restore.
    //
    // ⛔ `D6a`: this loop used to `continue` on a recursive position. The IH
    // then stood in for the argument as well as for the hypothesis, and every
    // later binder shifted down one slot.
    for position in 0..argument_binders {
        let source_position = u32::try_from(position).map_err(|_| {
            backend_module(
                "a continuation case binder position exceeds the planner's field width".to_string(),
            )
        })?;
        if recursive_positions.contains(&position) {
            // ⛔ The hard stop for an unprojected recursive position is
            // segment 1's, and it has already fired: a position that is not the
            // ruled `worker_position` never reaches here. So this member is
            // always the *selected* recursive argument, and `D6a` deliberately
            // does not generalize to a multi-worker population.
            //
            // `D6c` — the four run-shape mutations, under test only. Each moves
            // ONE producer input of this run and leaves the rest of the segment
            // exactly as it was, so a refusal downstream is attributable to that
            // input rather than to a rewritten builder.
            #[cfg(test)]
            match crate::cranelift_backend::lowering::d6c_selection_mutation() {
                // The pre-`D6a` defect exactly: the position is skipped and the
                // IH stands in for the argument as well.
                crate::cranelift_backend::lowering::D6cSelectionMutation::OmitSelectedArgument => {
                    crate::cranelift_backend::lowering::record_d6c_selection_application();
                    continue;
                }
                // One run naming two selected arguments.
                crate::cranelift_backend::lowering::D6cSelectionMutation::DuplicateSelectedArgument => {
                    crate::cranelift_backend::lowering::record_d6c_selection_application();
                    run.push(ContinuationCaseBinderSource::SelectedRecursiveArgument {
                        source_position,
                    });
                }
                // A source position the unit projects no worker for. ⛔ Chosen
                // by arithmetic on the ruled position rather than from the
                // envelope, so the value is not one the plan named anywhere.
                crate::cranelift_backend::lowering::D6cSelectionMutation::WrongSourcePosition => {
                    crate::cranelift_backend::lowering::record_d6c_selection_application();
                    run.push(ContinuationCaseBinderSource::SelectedRecursiveArgument {
                        source_position: source_position.wrapping_add(1),
                    });
                    continue;
                }
                _ => {}
            }
            run.push(ContinuationCaseBinderSource::SelectedRecursiveArgument { source_position });
            continue;
        }
        let index = envelope
            .iter()
            .position(|role| {
                matches!(
                    role,
                    ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField {
                        source_position: candidate,
                    } if *candidate == source_position
                )
            })
            .ok_or_else(|| {
                backend_module(format!(
                    "the ordinary envelope has no nonrecursive field at source position \
                     {source_position}, so the selected case's binder run cannot be built"
                ))
            })?;
        run.push(ContinuationCaseBinderSource::Ordinary(index));
    }

    // `D6c` — WRONG ORDER, under test only. The IH prefix and the argument
    // segment are exchanged and nothing else moves: the same members, the same
    // count, the same continuation-input tail. ⛔ Applied before segment 3 so
    // the tail stays in its ruled place and the perturbation is confined to the
    // two segments whose order is the law under test.
    #[cfg(test)]
    if crate::cranelift_backend::lowering::d6c_selection_mutation()
        == crate::cranelift_backend::lowering::D6cSelectionMutation::WrongOrder
    {
        let hypotheses = run
            .iter()
            .filter(|source| {
                matches!(source, ContinuationCaseBinderSource::InductionHypothesis)
            })
            .count();
        if hypotheses > 0 && hypotheses < run.len() {
            crate::cranelift_backend::lowering::record_d6c_selection_application();
            run.rotate_left(hypotheses);
        }
    }

    // Segment 3 -- the tail environment: this frame's continuation inputs, in
    // their ruled ordinal order.
    for ordinal in 0..continuation_inputs {
        run.push(ContinuationCaseBinderSource::ContinuationInput(ordinal));
    }

    // ⭐⭐ **`RT-CONTSRC-PRODUCER-LOCAL` `D6c` — THE CANONICAL-RUN
    // POSTCONDITION. The run is SEALED here, and nothing leaves this function
    // unvalidated.**
    //
    // This function's doc has always claimed *"every gap is a hard stop rather
    // than a hole"*, and for the gaps it names that was true. It was NOT true of
    // the run's own SHAPE: `D6c` measured a member omitted, a member duplicated,
    // and the two segments permuted, and in each case the malformed run was
    // returned and lowered. Omission is the pre-`D6a` defect exactly, and on the
    // mixed witness it compiled clean -- the case body reads only `Var(0)`, so
    // every later binder shifted with nothing positioned to notice.
    //
    // ⛔ **Validated IN PLACE against this function's own inputs. There is no
    // second builder and no second population.** Constructing an expected run
    // and comparing would be a parallel authority able to reproduce the very
    // defect it checks, and the equality would prove only that two
    // constructions agree with each other.
    //
    // ⚠ **What this deliberately does NOT require:** that `Ordinary(index)`
    // values be numerically source-ordered, or any reconstruction of the
    // ordinary envelope's order. A self-consistent envelope permutation is
    // lawful -- each member is checked against the ROLE its own index names, so
    // the envelope may be laid out however the planner chose.
    let hypotheses = recursive_positions.len();
    if run.len() != sealed_total {
        return Err(backend_module(format!(
            "the sealed binder run holds {} members, but this case seals {hypotheses} induction \
             hypotheses + {argument_binders} constructor arguments + {continuation_inputs} \
             continuation inputs = {sealed_total}. A run of the wrong length shifts every later \
             binder, which is a wrong program rather than a refused one",
            run.len()
        )));
    }

    // Segment 1's exact extent. ⛔ Both directions: a non-hypothesis inside the
    // prefix and a hypothesis outside it are different defects and both are
    // caught, the second by the segment walks below.
    for (position, source) in run.iter().enumerate().take(hypotheses) {
        // ⛔ EVERY variant enumerated, no `matches!` and no `other` arm. This is
        // a load-bearing position: a future member kind must be a compile error
        // here, forcing a decision about whether it may lead the run, rather
        // than falling into a catch-all that happens to reject it today for a
        // reason nobody chose.
        match source {
            ContinuationCaseBinderSource::InductionHypothesis => {}
            ContinuationCaseBinderSource::SelectedRecursiveArgument { .. }
            | ContinuationCaseBinderSource::Ordinary(_)
            | ContinuationCaseBinderSource::ContinuationInput(_) => {
                return Err(backend_module(format!(
                    "the sealed binder run holds {source:?} at position {position}, inside the \
                     {hypotheses}-member induction-hypothesis prefix. The IH prefix leads the run \
                     and the constructor arguments follow it; a member of another kind here is \
                     the two segments permuted"
                )));
            }
        }
    }

    // Segment 2 -- every constructor argument at its own source position.
    //
    // ⛔ The match is EXHAUSTIVE over the closed source sum with no wildcard, so
    // a future variant is a compile error here rather than a silent acceptance.
    for position in 0..argument_binders {
        let index = hypotheses + position;
        let source_position = u32::try_from(position).map_err(|_| {
            backend_module(
                "a continuation case binder position exceeds the planner's field width".to_string(),
            )
        })?;
        let recursive = recursive_positions.contains(&position);
        match &run[index] {
            ContinuationCaseBinderSource::SelectedRecursiveArgument {
                source_position: named,
            } => {
                if !recursive {
                    return Err(backend_module(format!(
                        "the sealed binder run names a selected recursive argument at run \
                         position {index} for source position {source_position}, which this case \
                         does not list as recursive"
                    )));
                }
                if *named != source_position {
                    return Err(backend_module(format!(
                        "the sealed binder run's argument segment holds a selected recursive \
                         argument for source position {named} at the slot belonging to source \
                         position {source_position}"
                    )));
                }
                if position != worker_position {
                    return Err(backend_module(format!(
                        "the sealed binder run names a selected recursive argument at source \
                         position {source_position}, but this specialization projects a worker \
                         for position {worker_position}"
                    )));
                }
            }
            ContinuationCaseBinderSource::Ordinary(role_index) => {
                if recursive {
                    return Err(backend_module(format!(
                        "the sealed binder run takes source position {source_position} from the \
                         ordinary envelope, but this case lists it as recursive -- the recursive \
                         field is a compiler-only member and has no envelope operand"
                    )));
                }
                // ⚠ The ROLE the index names, never the index's own value. This
                // is what keeps a lawful envelope permutation lawful.
                let role = envelope.get(*role_index).ok_or_else(|| {
                    backend_module(format!(
                        "the sealed binder run points at ordinary-envelope index {role_index}, \
                         which this frame's {}-role envelope does not hold",
                        envelope.len()
                    ))
                })?;
                match role {
                    ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField {
                        source_position: candidate,
                    } if *candidate == source_position => {}
                    other => {
                        return Err(backend_module(format!(
                            "the sealed binder run takes source position {source_position} from \
                             ordinary-envelope index {role_index}, which names {other:?} instead"
                        )));
                    }
                }
            }
            other @ (ContinuationCaseBinderSource::InductionHypothesis
            | ContinuationCaseBinderSource::ContinuationInput(_)) => {
                return Err(backend_module(format!(
                    "the sealed binder run holds {other:?} at run position {index}, inside the \
                     constructor-argument segment for source position {source_position}"
                )));
            }
        }
    }

    // Segment 3 -- the continuation-input tail, by exact ordinal.
    for ordinal in 0..continuation_inputs {
        let index = hypotheses + argument_binders + ordinal;
        // ⛔ EVERY variant enumerated, for the same reason as the IH prefix. The
        // ordinal mismatch is split out from the wrong-kind case so the two are
        // distinguishable in the diagnostic rather than merged into one arm.
        match &run[index] {
            ContinuationCaseBinderSource::ContinuationInput(named) => {
                if *named != ordinal {
                    return Err(backend_module(format!(
                        "the sealed binder run holds continuation input {named} at run position \
                         {index}, where this frame's continuation input {ordinal} belongs"
                    )));
                }
            }
            source @ (ContinuationCaseBinderSource::InductionHypothesis
            | ContinuationCaseBinderSource::SelectedRecursiveArgument { .. }
            | ContinuationCaseBinderSource::Ordinary(_)) => {
                return Err(backend_module(format!(
                    "the sealed binder run holds {source:?} at run position {index}, where this \
                     frame's continuation input {ordinal} belongs"
                )));
            }
        }
    }

    Ok(run)
}

/// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the projected facts one
/// continuation specialization's SELECTED CASE BODY is lowered from.**
///
/// Exactly the members [`lower_continuation_selected_case_body`] reads, and no
/// more. The frame-shaped members of the definition pass's own projection --
/// slots, offsets, header, consumer owner -- are absent because a local
/// composition has no frame: it lowers the same body with no `Function`, no
/// descriptor and no ABI of its own.
pub(super) struct ContinuationSelectedCaseBody {
    pub(super) id: ContinuationSpecializationId,
    pub(super) continuation_origin: StaticOriginId,
    pub(super) producer_alternative: u32,
    pub(super) recursive_position: u32,
    pub(super) worker_closure_origin: StaticOriginId,
    pub(super) worker_body_origin: StaticOriginId,
    pub(super) worker_declared_arity: u32,
    pub(super) worker_capture_count: usize,
}

/// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — lower a continuation
/// specialization's exact selected case body, given its operands.**
///
/// ⭐⭐ **This is a factoring, not a new mechanism, and that is the point.**
/// Architect `evt_6kn9ckdnbf0ph` rules that a fusion-local identity replaces
/// **only its direct-call realization**: the body it would have executed, the
/// two static-worker bindings, the binder run and the environment are all
/// unchanged. So the local composition and the standalone definition must lower
/// **the same body from the same plan** -- and the only way to say that and
/// have it stay true is for there to be one function, called from both seats.
/// A second copy here would be a second authority over what the selected case
/// body IS, and the two would drift with nothing able to see it.
///
/// ⛔ **What differs between the two callers is the OPERANDS, and nothing
/// else.** The definition pass loads `ordinary` and `carried_inputs` from its
/// own frame's `Parameter` and `Capture` slots at the descriptor's offsets; the
/// local composition receives the very same two runs assembled at the call
/// edge, from the planner's ordinary envelope and its continuation-input
/// projection. Both are the target specialization's own ordinary envelope in
/// its own order -- that is why the same body can consume either.
///
/// ⛔ **It returns the phase-bearing [`LoweringOperand`], and writes no result
/// slot.** The definition pass stores it to its frame's `Result` offset; the
/// local composition hands it straight to the caller's existing eliminator.
/// Neither seat's disposal belongs here, and putting one here would give the
/// other a store it must then undo.
pub(super) fn lower_continuation_selected_case_body(
    compiler: &mut Lowering<'_>,
    builder: &mut FunctionBuilder<'_>,
    facts: &ContinuationSelectedCaseBody,
    envelope: &[ContinuationOrdinaryEnvelopeRole],
    ordinary: &[LoweringOperand],
    carried_inputs: &[LoweringOperand],
    // `D5a` — `Some` exactly when the CALLER resolved a generated execution
    // context for this specialization's worker body into its own function. ⛔
    // Supplied rather than re-asked of the planner: an issued context the
    // caller did not resolve is not a context the induction hypothesis may
    // name, and that is a fact about the caller's function, not about the plan.
    retargeted_worker_body: Option<StaticOriginId>,
) -> Result<LoweringOperand, CraneliftBackendError> {
    // The ordered capture segment for the selected worker: the
    // envelope's `WorkerCapture` roles, in capture-ordinal order,
    // taking each one's operand from its own Parameter position.
    let mut worker_captures = Vec::new();
    for (position, role) in envelope.iter().enumerate() {
        if matches!(role, ContinuationOrdinaryEnvelopeRole::WorkerCapture { .. }) {
            worker_captures.push(ordinary[position].clone());
        }
    }
    if worker_captures.len() != facts.worker_capture_count {
        return Err(backend_module(
            "the ordinary envelope's worker-capture segment disagrees with the selected \
             worker's capture count"
                .to_string(),
        ));
    }

    // `D6a` -- THE INDUCTION HYPOTHESIS'S ROUTE, and only its.
    //
    // `retargeted_worker_body` is `Some` exactly when
    // `continuation_context_for` issued a generated execution context
    // for this `(specialization, worker body)` pair AND the retarget
    // below resolved it into this function. Both halves are required,
    // which is why this reads the retarget's own outcome rather than
    // re-asking the planner: an issued context this unit did not
    // resolve is not a context this binding can name.
    //
    // ⭐ `None` is the ordinary, lawful answer -- every pre-`D5a`
    // specialization, and every unit in the governed-bracket witness.
    // The hypothesis then takes the raw route, appending nothing, and
    // the two bindings below are route-identical. That is a degenerate
    // route pair, not a collapsed one.
    //
    // ⛔ Not re-derived at the call site. Both bindings below name the
    // same body origin, so no comparison available there can tell them
    // apart -- see `StaticWorkerCallRoute`.
    let induction_route = match retargeted_worker_body {
        Some(_) => StaticWorkerCallRoute::GeneratedContext,
        None => StaticWorkerCallRoute::RawWorker,
    };
    // `D6c` — CROSS-ROUTING, the hypothesis half. It takes the raw route
    // while this unit DID resolve a context; the argument below takes
    // the context route. ⛔ Only where a context was actually resolved:
    // on a route-degenerate unit both members lawfully carry
    // `RawWorker`, so there is no crossing to make and the arm declines
    // rather than counting an application it did not perform.
    #[cfg(test)]
    let induction_route = if crate::cranelift_backend::lowering::d6c_selection_mutation()
        == crate::cranelift_backend::lowering::D6cSelectionMutation::CrossRouteTargets
        && retargeted_worker_body.is_some()
    {
        crate::cranelift_backend::lowering::record_d6c_selection_application();
        StaticWorkerCallRoute::RawWorker
    } else {
        induction_route
    };

    // The EXISTING constructor, with the projected identity and arity.
    let worker = compiler.construct_static_worker_binding(
        facts.worker_closure_origin,
        facts.worker_body_origin,
        facts.worker_declared_arity,
        facts.worker_capture_count,
        worker_captures.clone(),
        induction_route,
        // `D8i` — an induction hypothesis answers for no composed
        // source continuation. Stated explicitly: this is a positive
        // claim about the hypothesis's role, not the absence of one.
        ContinuationDischarge::DirectSpecializationCall,
    )?;

    // `D6a` -- the selected recursive constructor argument.
    //
    // ⭐ The SAME closure occurrence, body origin, declared arity and
    // ordered capture operands as the induction hypothesis above, built
    // through the same constructor and validated against the same raw
    // template contract. What the two represent still differs: the
    // argument is the closure the source scope binds, while the
    // hypothesis is that closure as this specialization eliminates it.
    //
    // ⛔ The ROUTE is `RawWorker` unconditionally here, and that is not
    // the same as saying it differs from the hypothesis's. When
    // `induction_route` above resolved to `RawWorker` -- no context
    // issued -- the two bindings are route-identical, and they are
    // still two bindings for two positions of the run. The route is
    // what will separate them at the call edge in `D6b` *where a
    // context exists*; it is not what makes them two.
    //
    // ⛔ Nothing new crosses the ABI. This adds no slot, carrier, tag,
    // descriptor or source occurrence -- it is a second compiler-only
    // binding over operands this frame has already loaded.
    // `D6c` — the three binding-construction mutations, under test only.
    // Each moves ONE argument handed to the existing constructor; the
    // constructor itself, the hypothesis above and the run below are all
    // untouched, so the guard that refuses is the one that owns the
    // moved input.
    #[cfg(test)]
    let (argument_body_origin, argument_captures, argument_route) = {
        use crate::cranelift_backend::lowering::D6cSelectionMutation as Mutation;
        match crate::cranelift_backend::lowering::d6c_selection_mutation() {
            // A body this unit did not select. ⛔ The substituted value
            // is a REAL planner-issued origin -- this continuation's own
            // frame occurrence -- rather than an arithmetic neighbour.
            // A fabricated id could be refused merely for being unknown;
            // a real origin naming the wrong role is the case the guard
            // actually has to catch. The control asserts it differs from
            // the selected body.
            Mutation::WrongClosureBody => {
                crate::cranelift_backend::lowering::record_d6c_selection_application();
                (
                    facts.continuation_origin,
                    worker_captures.clone(),
                    StaticWorkerCallRoute::RawWorker,
                )
            }
            // A capture run that is not the envelope's worker-capture
            // segment: drop an operand where there is one, otherwise add
            // one the envelope holds.
            //
            // ⛔ The counter fires ONLY if the vector actually changed.
            // A unit with no captures and no ordinary operand to borrow
            // leaves this arm the IDENTITY, and counting an application
            // there would report a perturbation that never happened --
            // which is precisely how a control comes to prove the
            // opposite of what it claims.
            Mutation::WrongCaptureRun => {
                let mut perturbed = worker_captures.clone();
                if perturbed.pop().is_none() {
                    perturbed.extend(ordinary.first().cloned());
                }
                if perturbed.len() != worker_captures.len() {
                    crate::cranelift_backend::lowering::record_d6c_selection_application();
                }
                (
                    facts.worker_body_origin,
                    perturbed,
                    StaticWorkerCallRoute::RawWorker,
                )
            }
            // The argument takes the context route. Paired with the
            // hypothesis taking the raw route above, this is the
            // cross-routing the two members must never permit.
            //
            // ⛔ Only where a context was actually resolved. On a
            // route-degenerate unit both members lawfully carry
            // `RawWorker`, so there is no crossing to perform; applying
            // it there would move a route no law distinguishes and count
            // an application for a perturbation with no content.
            Mutation::CrossRouteTargets if retargeted_worker_body.is_some() => {
                crate::cranelift_backend::lowering::record_d6c_selection_application();
                (
                    facts.worker_body_origin,
                    worker_captures.clone(),
                    StaticWorkerCallRoute::GeneratedContext,
                )
            }
            _ => (
                facts.worker_body_origin,
                worker_captures.clone(),
                StaticWorkerCallRoute::RawWorker,
            ),
        }
    };
    #[cfg(not(test))]
    let (argument_body_origin, argument_captures, argument_route) = (
        facts.worker_body_origin,
        worker_captures,
        StaticWorkerCallRoute::RawWorker,
    );
    let recursive_argument = compiler.construct_static_worker_binding(
        facts.worker_closure_origin,
        argument_body_origin,
        facts.worker_declared_arity,
        facts.worker_capture_count,
        argument_captures,
        argument_route,
        // `D8i` — the SPECIALIZATION's selected recursive argument.
        // ⛔ Direct, and the contrast with `D8d`'s composed argument is
        // the point: the same source closure at the same position
        // carries an authority on the composed path and none here,
        // because only the composed consumption stands in for a causal
        // call the producer never made.
        ContinuationDischarge::DirectSpecializationCall,
    )?;

    // Exact body recovery: the selected case of the computational
    // frame this continuation belongs to, by its own alternative.
    let frame_occurrence =
        compiler.retained_body_occurrence(facts.continuation_origin)?;
    let RuntimeExpr::ComputationalMatch { cases, .. } = frame_occurrence.expr else {
        return Err(backend_module(
            "a continuation origin does not resolve to a computational frame".to_string(),
        ));
    };
    let alternative = facts.producer_alternative as usize;
    let case = cases.get(alternative).ok_or_else(|| {
        backend_module(
            "the projected producer alternative is outside the frame's case run"
                .to_string(),
        )
    })?;
    let body = compiler.case_body_occurrence(
        frame_occurrence.static_origin,
        alternative,
        &case.body,
    )?;
    // The semantic case environment, through the sole binding
    // authority, in the order `continuation_case_binder_run` states:
    // the IH prefix, then ALL the constructor arguments in source
    // order -- the selected recursive one included, as `D6a`'s
    // compiler-only member -- then this frame's continuation inputs.
    //
    // ⛔ This site chooses nothing. It maps a plan onto operands; the
    // order is the plan's, and the plan is a pure function of the
    // planner's own coordinates.
    let plan = continuation_case_binder_run(
        case.argument_binders,
        &case.recursive_positions,
        facts.recursive_position,
        envelope,
        carried_inputs.len(),
    )?;
    let mut env: Vec<LoweringEnvironmentBinding> = Vec::with_capacity(plan.len());
    for source in &plan {
        let binding = match *source {
            ContinuationCaseBinderSource::InductionHypothesis => {
                LoweringEnvironmentBinding::StaticWorker(worker.clone())
            }
            ContinuationCaseBinderSource::SelectedRecursiveArgument {
                source_position,
            } => {
                // The plan only ever names the ruled position here;
                // segment 1 hard-stops on any other. Re-checking it is
                // what keeps that a fact this site verifies rather than
                // one it inherits.
                if source_position != facts.recursive_position {
                    return Err(backend_module(format!(
                        "the binder run names a selected recursive argument at source \
                         position {source_position}, but this specialization projects a \
                         worker for position {}",
                        facts.recursive_position
                    )));
                }
                LoweringEnvironmentBinding::StaticWorker(recursive_argument.clone())
            }
            ContinuationCaseBinderSource::Ordinary(index) => {
                let operand = ordinary.get(index).ok_or_else(|| {
                    backend_module(
                        "the binder run names an ordinary-envelope index this frame loaded \
                         no operand for"
                            .to_string(),
                    )
                })?;
                LoweringEnvironmentBinding::Value(operand.clone())
            }
            ContinuationCaseBinderSource::ContinuationInput(ordinal) => {
                let operand = carried_inputs.get(ordinal).ok_or_else(|| {
                    backend_module(
                        "the binder run names a continuation input ordinal this frame \
                         loaded no operand for"
                            .to_string(),
                    )
                })?;
                LoweringEnvironmentBinding::Value(operand.clone())
            }
        };
        env.push(binding);
    }

    // `D6b` — the same instant, structured. The trace below renders the
    // ROUTE of each static-worker member; this record carries the body
    // origin beside it, which is what lets a control ask whether the
    // mixed pair is over ONE body rather than merely mixed.
    #[cfg(test)]
    crate::cranelift_backend::lowering::record_d6b_specialization_body(
        crate::cranelift_backend::lowering::D6bSpecializationBody {
            unit: facts.id,
            worker_body_origin: facts.worker_body_origin,
            retargeted: retargeted_worker_body,
            worker_call_targets: compiler
                .function_local
                .worker_calls
                .keys()
                .copied()
                .collect(),
            raw_worker_call_targets: compiler
                .function_local
                .raw_worker_calls
                .keys()
                .copied()
                .collect(),
            members: env
                .iter()
                .enumerate()
                .filter_map(|(position, binding)| match binding {
                    LoweringEnvironmentBinding::StaticWorker(worker) => {
                        Some((position, worker.route, worker.body_origin))
                    }
                    LoweringEnvironmentBinding::Value(_) => None,
                })
                .collect(),
        },
    );
    #[cfg(test)]
    d5a_trace(format!(
        "  SPEC-BODY {:?} alt={} binders={} ordinary={} envelope={:?} env=[{}]",
        facts.id,
        facts.producer_alternative,
        case.argument_binders,
        ordinary.len(),
        envelope,
        env.iter()
            // `D6a` -- the ROUTE is printed, not just the arm. Both
            // static-worker members name the same closure, body and
            // arity, so an arm-only rendering shows two identical
            // entries and a change collapsing the two routes would be
            // invisible in this log.
            //
            // ⛔ The converse does not hold, and a reader of this log
            // must not assume it: two entries rendering the SAME route
            // is the lawful route-degenerate case (no context issued),
            // not evidence that one binding was reused for both
            // members. Only a witness whose planner issues a context
            // renders a mixed pair, and only there does this log
            // discriminate the routes at all.
            .map(|binding| match binding {
                LoweringEnvironmentBinding::StaticWorker(worker) => match worker.route {
                    StaticWorkerCallRoute::RawWorker => "StaticWorker(RawWorker)",
                    StaticWorkerCallRoute::GeneratedContext =>
                        "StaticWorker(GeneratedContext)",
                },
                LoweringEnvironmentBinding::Value(LoweringOperand::Carried(_)) =>
                    "Carried",
                LoweringEnvironmentBinding::Value(LoweringOperand::Specialized(_)) =>
                    "Specialized",
            })
            .collect::<Vec<_>>()
            .join(", ")
    ));
    let lowered = compiler.lower_expr(builder, body, &env)?;

    Ok(lowered)
}

/// **`RT-CONTSPEC-ACTIVATE` `D2` — define each declared continuation target
/// from its own projected contract.**
///
/// Operands come from the **descriptor**: each `Parameter` and `Capture` slot
/// is loaded at the offset `slot_offsets` assigns it. ⛔ Function parameter 0
/// is not the payload, and the `Result` slot is never read -- it is
/// caller-initialized, and this body only writes it.
///
/// The partition is the ruled one: `Parameter` operands are the ordinary
/// envelope (nonrecursive producer fields, then selected worker captures in
/// capture-ordinal order), and `Capture` operands are the continuation inputs
/// by ordinal. The worker binding is built by the **existing** static-worker
/// constructor, and the semantic environment is the sole
/// `LoweringEnvironmentBinding` authority -- no parallel operand map and no
/// worker-body de Bruijn table.
pub(super) fn define_continuation_bodies<M: Module>(
    module: &mut M,
    compiler: &mut Lowering<'_>,
    helpers: ArtifactHelpers<'_>,
    bundle: &UnitBundle,
) -> Result<usize, CraneliftBackendError> {
    // Own every projected fact BEFORE the loop: the projection borrows the
    // plan, and the definition below needs the compiler mutably.
    struct OwnedContinuationEmission {
        id: ContinuationSpecializationId,
        slots: Vec<AbiSlot>,
        offsets: Vec<u32>,
        envelope: Vec<ContinuationOrdinaryEnvelopeRole>,
        inputs: Vec<ContinuationInputView>,
        continuation_origin: StaticOriginId,
        producer_alternative: u32,
        recursive_position: u32,
        worker_closure_origin: StaticOriginId,
        worker_body_origin: StaticOriginId,
        worker_declared_arity: u32,
        worker_capture_count: usize,
        header_parameters: u32,
        header_captures: u32,
        /// `D8o` — the owner of the source body this specialization lowers: the
        /// continuation's own consumer. ⛔ Carried from the planner view rather
        /// than derived here; it is an existing planner fact reaching the site
        /// that needs it, not a new authority.
        consumer_owner: PredeclaredFunctionId,
    }
    // `RT-WORKER-BIND` `D4` exposes its local declaration operation for a
    // separately emitted caller; a continuation function is exactly that, so
    // it declares its own worker refs rather than borrowing another's.
    let worker_targets = resolve_worker_targets(&compiler.static_transition_plan, bundle)?;
    // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the ORDINARY residual targets
    // `O_t`, and the fusion-local complement recorded as this pass's own
    // omission.
    //
    // ⛔ **The omission is recorded HERE, in the pass that would otherwise have
    // emitted the body**, and the closeout compares it with `F_t`. A statement
    // made elsewhere about what this loop does would agree with itself; this is
    // the loop's own decision, and if it ever stopped omitting, the recorded set
    // would shrink and the range equality would say so.
    let ordinary_targets = compiler.static_transition_plan.ordinary_continuation_targets()?;
    for omitted in compiler
        .static_transition_plan
        .continuation_units()?
        .iter()
        .map(|unit| unit.id())
        .filter(|id| !ordinary_targets.contains(id))
        .collect::<Vec<_>>()
    {
        compiler
            .fusion_compositions
            .as_mut()
            .ok_or_else(|| {
                backend_module(
                    "a fusion-local continuation target was omitted from the definition pass \
                     with no composition ledger open to record it; the omission would then be \
                     invisible to the range equality that is the only thing requiring it"
                        .to_string(),
                )
            })?
            .record_definition_omitted(omitted);
    }
    let emissions = compiler
        .static_transition_plan
        .continuation_units()?
        .into_iter()
        .filter(|unit| ordinary_targets.contains(&unit.id()))
        .map(|unit| {
            let (offsets, _frame_bytes) = unit.slot_offsets()?;
            Ok(OwnedContinuationEmission {
                id: unit.id(),
                slots: unit.slots().to_vec(),
                offsets,
                envelope: unit.ordinary_envelope()?,
                inputs: unit.continuation_inputs()?,
                continuation_origin: unit.continuation_origin(),
                producer_alternative: unit.producer_alternative(),
                recursive_position: unit.recursive_position(),
                worker_closure_origin: unit.worker_closure_origin(),
                worker_body_origin: unit.worker_body_origin(),
                worker_declared_arity: unit.worker_declared_arity(),
                worker_capture_count: unit.worker_capture_count(),
                header_parameters: unit.header().parameters,
                header_captures: unit.header().captures,
                consumer_owner: unit.consumer_owner(),
            })
        })
        .collect::<Result<Vec<_>, CraneliftBackendError>>()?;

    // Shape index for the `D7` definition-binding control: declared arity and
    // capture count per specialization, which is the only same-shaped
    // definition. Built BEFORE the loop because the loop consumes `emissions`.
    //
    // ⛔ Over the CALLABLE population, not over `emissions`. Measured, not
    // reasoned: this fixture plans exactly one continuation specialization, so
    // an index over specializations has a single entry and the control refuses
    // for want of a partner that was never going to be there. The two
    // same-shaped things are the worker BODIES, which is what `AC-9` means by
    // "a distinct same-shaped target".
    #[cfg(test)]
    let d7_callable_index: Vec<((usize, usize), StaticOriginId)> = compiler
        .static_transition_plan
        .emittable_units()?
        .iter()
        .map(|emittable| {
            (
                (
                    emittable
                        .slots()
                        .iter()
                        .filter(|slot| slot.kind == AbiSlotKind::Parameter)
                        .count(),
                    emittable
                        .slots()
                        .iter()
                        .filter(|slot| slot.kind == AbiSlotKind::Capture)
                        .count(),
                ),
                emittable.body_occurrence(),
            )
        })
        .collect();

    let mut defined = 0usize;
    #[cfg_attr(not(test), expect(unused_mut, reason = "only the D7 control mutates it"))]
    for mut unit in emissions {
        // Resolve the EXACT target first; the control below perturbs only what
        // the definition is handed.
        let exact_id = bundle.continuation(unit.id).ok_or_else(|| {
            backend_module(
                "a planned continuation specialization was never forward-declared".to_string(),
            )
        })?;
        let id = exact_id;

        // `RT-CONTSPEC-WITNESS` `D7`/`AC-9` — bind a DISTINCT same-shaped body
        // under this exact declared `FuncId`.
        //
        // ⛔ `id` is `exact_id` and stays that way: the declared function, the
        // specialization id, the causal token, the header, slots, offsets,
        // inputs, owner and the emitted call are all untouched, and
        // `verify_emitted_continuation_calls` remains enabled. The ONLY thing
        // that moves is which body this declared function executes -- which is
        // the residual the finished-CLIF equality gate explicitly cannot see.
        #[cfg(test)]
        if crate::cranelift_backend::lowering::CONTINUATION_EMISSION_MUTATION
            .with(std::cell::Cell::get)
            == ContinuationEmissionMutation::SubstituteContinuationBodyDefinition
        {
            let exact_shape = d7_callable_index
                .iter()
                .find(|(_, origin)| *origin == unit.worker_body_origin)
                .map(|(shape, _)| *shape)
                .ok_or_else(|| {
                    backend_module(
                        "the D7 definition-binding control could not read the exact worker \
                         body's declared arity and capture count; a control that cannot \
                         establish the shape it matches on must refuse rather than bind an \
                         arbitrary body"
                            .to_string(),
                    )
                })?;
            let substitute = d7_callable_index
                .iter()
                .find(|(shape, origin)| {
                    *shape == exact_shape && *origin != unit.worker_body_origin
                })
                .map(|(_, origin)| *origin)
                .ok_or_else(|| {
                    backend_module(
                        "the D7 definition-binding control found no distinct same-shaped \
                         callable body to bind under this declared function; that is a \
                         missing fixture precondition, not a discharge"
                            .to_string(),
                    )
                })?;
            unit.worker_body_origin = substitute;
            crate::cranelift_backend::lowering::record_d7_definition_binding_substitution();
        }

        let offsets = unit.offsets.as_slice();
        let envelope = &unit.envelope;
        let inputs = &unit.inputs;
        let slots = unit.slots.as_slice();
        if slots.len() != offsets.len() {
            return Err(backend_module(
                "a continuation slot run disagrees with its own offset walk".to_string(),
            ));
        }

        // Reject BEFORE definition on partition incompleteness: the ordinary
        // envelope must cover every Parameter slot, and the continuation
        // inputs must cover every Capture slot densely by ordinal.
        let parameter_slots: Vec<_> = slots
            .iter()
            .zip(offsets)
            .filter(|(slot, _)| slot.kind == AbiSlotKind::Parameter)
            .collect();
        let capture_slots: Vec<_> = slots
            .iter()
            .zip(offsets)
            .filter(|(slot, _)| slot.kind == AbiSlotKind::Capture)
            .collect();
        if parameter_slots.len() != envelope.len() {
            return Err(backend_module(
                "the ruled ordinary envelope does not cover the Parameter slot run".to_string(),
            ));
        }
        if capture_slots.len() != inputs.len() {
            return Err(backend_module(
                "the projected continuation inputs do not cover the Capture slot run".to_string(),
            ));
        }
        for (position, input) in inputs.iter().enumerate() {
            if input.ordinal as usize != position {
                return Err(backend_module(
                    "continuation inputs are not dense in ordinal order".to_string(),
                ));
            }
        }
        // Provenance: every worker capture role must name the worker this key
        // selected, so an envelope built against another closure rejects.
        for role in envelope.iter() {
            if let ContinuationOrdinaryEnvelopeRole::WorkerCapture { closure_origin, .. } = role {
                if *closure_origin != unit.worker_closure_origin {
                    return Err(backend_module(
                        "an ordinary-envelope worker capture names a different closure than the \
                         selected worker"
                            .to_string(),
                    ));
                }
            }
        }

        compiler.open_aggregate_events(id)?;
        let sig = unit_signature(module);
        let mut func =
            Function::with_name_signature(UserFuncName::user(0, id.as_u32()), sig);
        // Set by the retarget below; `None` means this specialization calls the
        // raw worker unit directly, which is every pre-`D5a` case.
        let mut retargeted_worker_body: Option<StaticOriginId> = None;
        let result_offset = slots
            .iter()
            .zip(offsets)
            .find(|(slot, _)| slot.kind == AbiSlotKind::Result)
            .map(|(_, offset)| *offset)
            .ok_or_else(|| {
                backend_module("continuation frame declares no result slot".to_string())
            })?;
        let trap_offset = slots
            .iter()
            .zip(offsets)
            .find(|(slot, _)| slot.kind == AbiSlotKind::Trap)
            .map(|(_, offset)| *offset)
            .ok_or_else(|| {
                backend_module("continuation frame declares no trap slot".to_string())
            })?;

        let mut function_local = helpers.declare_in_func(module, &mut func, None);
        // ONE lawful declaration per continuation `Function`, retained whole
        // and seated in BOTH existing roles. The worker constructor validates
        // through `unit_calls`; the later callee-only consumer resolves
        // through `worker_calls`. Seating only the second is what made the
        // constructor refuse -- the objects are the same, the roles are not.
        //
        // Declared here, into THIS function: no `FuncRef` crosses a function.
        let declared_workers = worker_targets.declare_in_func(module, &mut func);
        function_local.unit_calls = declared_workers.clone();
        // `D6b` -- the RAW route's table, captured before the retarget below
        // rewrites `worker_calls`. This is the only point at which the raw
        // callee for a retargeted body is still in hand.
        function_local.raw_worker_calls = declared_workers.clone();
        function_local.worker_templates = worker_targets.templates().clone();
        function_local.context_calls = declare_context_calls_in_func(
            module,
            &mut func,
            &compiler.static_transition_plan,
            bundle,
        )?;
        // `RT-DECL-CLOSURE-PORT` `D5a` -- THE RETARGET.
        //
        // If this specialization's worker body has a generated execution
        // context, the worker call resolves to that context instead of to the
        // raw unit. ⛔ Only `worker_calls` moves: `unit_calls` above keeps the
        // raw target, because the static-worker CONSTRUCTOR validates against
        // the raw body's own contract and this retarget does not change what
        // that body is.
        //
        // ⭐ Why this is what makes the whole thing work: the continuation
        // inputs live in THIS frame's Capture slots. The raw unit's ABI has
        // nowhere to put them, so calling the raw unit drops them before the
        // nested producer is reached -- which is exactly the measured defect.
        // The context's ABI has a capture run for them, so the call carries
        // them across the checked-IH worker execution.
        let mut worker_calls = declared_workers;
        // `D5a` checkpoint 4 step 3 -- the binding's three reaching mutations.
        //
        // ⛔ `Suppress` and `Transplant` perturb WHICH context the retarget is
        // handed; the exact lookup itself is untouched, so a refusal downstream
        // is attributable to the binding and not to a rewritten resolver.
        // `Transplant` declines when this unit has no foreign context to be
        // given -- it bumps the application counter when it does fire, so a
        // control can require the perturbation actually reached the seat rather
        // than reading a green as a defence.
        #[cfg(test)]
        let resolved_context = match crate::cranelift_backend::lowering::d5a_route_mutation() {
            crate::cranelift_backend::lowering::D5aRouteMutation::SuppressContextBinding => {
                crate::cranelift_backend::lowering::record_d5a_route_application();
                None
            }
            crate::cranelift_backend::lowering::D5aRouteMutation::TransplantContextBinding => {
                let foreign = compiler
                    .static_transition_plan
                    .continuation_contexts()?
                    .into_iter()
                    .find(|context| context.enclosing_specialization() != unit.id);
                match foreign {
                    Some(context) => {
                        crate::cranelift_backend::lowering::record_d5a_route_application();
                        Some(context)
                    }
                    None => compiler
                        .static_transition_plan
                        .continuation_context_for(unit.id, unit.worker_body_origin)?,
                }
            }
            _ => compiler
                .static_transition_plan
                .continuation_context_for(unit.id, unit.worker_body_origin)?,
        };
        #[cfg(not(test))]
        let resolved_context = compiler
            .static_transition_plan
            .continuation_context_for(unit.id, unit.worker_body_origin)?;
        if let Some(context) = resolved_context {
            // `D5a` checkpoint 4 step 3 -- THE TRANSPLANT STOP.
            //
            // ⭐⭐ Added because a transplant was **measured to compile**. The
            // resolved context used to be trusted wholesale and the record
            // below took its `origin` from `unit.worker_body_origin` -- the
            // asking unit's own value. `call_static_worker`'s
            // `target.origin != worker.body_origin` check therefore compared
            // that value with itself on this path and could not see a foreign
            // context at all. Handed one context in place of another, lowering
            // emitted a call that type-checked (the capture suffix made the
            // operand run agree) and transferred to a function executing a
            // DIFFERENT body.
            //
            // ⛔ Production never reaches that state: `continuation_context_for`
            // is keyed by `(enclosing, worker_body)` and is the only producer.
            // ⇒ But "unreachable by construction" was carrying the whole
            // guarantee here, with no check able to observe a violation, so the
            // ruling's transplanted-binding stop had nothing to name. These two
            // comparisons cost nothing and turn it into a fact the code checks.
            if context.enclosing_specialization() != unit.id {
                return Err(backend_module(format!(
                    "the retarget resolved generated context {:?}, whose enclosing specialization \
                     is {:?} and not the {:?} now being defined; a context executes on behalf of \
                     the one identity that owns it, so this call would transfer another \
                     specialization's captures across this one's worker execution",
                    context.id(),
                    context.enclosing_specialization(),
                    unit.id,
                )));
            }
            if context.worker_body_origin() != unit.worker_body_origin {
                return Err(backend_module(format!(
                    "the retarget resolved a generated context executing body {:?}, but this \
                     specialization selected worker body {:?}",
                    context.worker_body_origin(),
                    unit.worker_body_origin,
                )));
            }
            let target = bundle.context(context.id()).ok_or_else(|| {
                backend_module(
                    "a planned generated context was never forward-declared".to_string(),
                )
            })?;
            let (context_offsets, _frame_bytes) = context.slot_offsets()?;
            worker_calls.insert(
                unit.worker_body_origin,
                DeclaredUnitCall {
                    function: module.declare_func_in_func(target, &mut func),
                    // The context EXECUTES that body, so the origin it answers
                    // for is unchanged. ⛔ Read from the CONTEXT, not from the
                    // asking unit: taking it from `unit` is what made
                    // `call_static_worker`'s origin check self-referential
                    // here. The equality above is what lets both readings agree.
                    origin: context.worker_body_origin(),
                    call_site_origin: context.worker_body_origin(),
                    header: context.header(),
                    slots: context.slots().to_vec(),
                    offsets: context_offsets,
                },
            );
            retargeted_worker_body = Some(unit.worker_body_origin);
        }
        // ⛔ Checked AFTER the retarget, not before it. `D5a` checkpoint 4
        // step 2 removes a fully retargeted raw worker from the emitted
        // `Function` population, so demanding an *emittable-unit* target here
        // would reject exactly the case the retarget exists to serve. What this
        // specialization actually needs is a declared callee for its worker
        // body -- raw or generated -- and that is what is required.
        if !worker_calls.contains_key(&unit.worker_body_origin) {
            return Err(backend_module(
                "the selected continuation worker body has no declared callee in this function,                  neither an emittable raw unit nor a generated execution context"
                    .to_string(),
            ));
        }
        function_local.worker_calls = worker_calls;
        // `D8n` — this generated Function's own checked-frame consumption
        // transaction, spanning the specialization body exactly. ⛔ Opened before the builder and
        // closed after it, so every branch scope inside nests within it.
        let frame_scope = CheckedFrameFunctionScope::open(compiler)?;
        // ⭐⭐ `D8o` — THE BINDING THIS PASS NEVER HAD. A specialization body
        // used to run with whatever the previously defined body left in both
        // ambient fields. The owner is exactly the planner's identity for this
        // specialization; the unit is the owner of the source body it lowers,
        // which is the continuation's own consumer.
        let ambient = AmbientBodyAuthority::bind(
            compiler,
            ContinuationEmissionOwner::Specialization(unit.id),
            unit.consumer_owner,
        );
        // `D8o` — the exact body key, supplied by the pass that knows it.
        #[cfg(test)]
        crate::cranelift_backend::lowering::record_d8o_body_key(
            compiler.defining_function_id,
            crate::cranelift_backend::lowering::D8oBodyKey::ContinuationSpecialization(unit.id),
        );
        let mut func_ctx = FunctionBuilderContext::new();
        {
            let mut builder = FunctionBuilder::new(&mut func, &mut func_ctx);
            let entry = builder.create_block();
            builder.append_block_params_for_function_params(entry);
            builder.switch_to_block(entry);
            let envelope_pointer = builder.block_params(entry)[0];
            let frame = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                envelope_pointer,
                crate::activation_services::UNIT_CALL_FRAME_SLOTS,
            );
            // The same per-function activation-services preamble every
            // generated unit body binds, from the same envelope and services
            // record. Omitting it is what left this function with no boundary
            // arena; substituting the native-`Int` arena for the boundary one
            // is the defect the `S6`/`D6` ruling exists to remove, so the
            // record is read for both rather than one standing in for the
            // other.
            let host_dispatch_context = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                envelope_pointer,
                crate::activation_services::UNIT_CALL_FRAME_HOST_DISPATCH_CONTEXT,
            );
            let services = builder.block_params(entry)[1];
            let native_int_arena = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                services,
                crate::activation_services::SERVICES_NATIVE_INT_ARENA,
            );
            Lowering::require_nonzero(&mut builder, native_int_arena);
            let boundary_arena = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                services,
                crate::activation_services::SERVICES_BOUNDARY_ARENA,
            );
            Lowering::require_nonzero(&mut builder, boundary_arena);
            function_local.host_dispatch_context = Some(host_dispatch_context);
            function_local.native_int_arena = Some(native_int_arena);
            function_local.boundary_arena = Some(boundary_arena);
            function_local.services_pointer = Some(services);
            function_local.bind_unit_trap_frame(
                frame,
                i32::try_from(trap_offset).map_err(|_| {
                    backend_module("continuation trap slot offset exceeds range".to_string())
                })?,
            )?;
            compiler.function_local = function_local;

            // Descriptor-only loads. Each operand is read from the slot the
            // descriptor assigns it, and from nowhere else.
            let load_at = |builder: &mut FunctionBuilder<'_>, offset: u32| {
                let offset = i32::try_from(offset).map_err(|_| {
                    backend_module("continuation slot offset exceeds addressable range".to_string())
                })?;
                Ok::<_, CraneliftBackendError>(LoweringOperand::Carried(CarriedBoundaryWord {
                    word: builder.ins().load(types::I64, MemFlags::trusted(), frame, offset),
                }))
            };

            let mut ordinary = Vec::with_capacity(parameter_slots.len());
            for (_, offset) in &parameter_slots {
                ordinary.push(load_at(&mut builder, **offset)?);
            }
            let mut carried_inputs = Vec::with_capacity(capture_slots.len());
            for (_, offset) in &capture_slots {
                carried_inputs.push(load_at(&mut builder, **offset)?);
            }

            // `D5a` -- the operand suffix the retargeted worker call appends.
            //
            // These are THIS frame's own continuation inputs, in ordinal order,
            // which is exactly the capture run the generated context declares.
            // ⛔ Stashed rather than threaded through `construct_static_worker_
            // binding`: the worker binding is the raw body's contract and adding
            // a context's captures to it would make the raw contract vary with
            // its caller.
            if let Some(worker_body_origin) = retargeted_worker_body {
                compiler.function_local.generated_context_captures =
                    Some(GeneratedContextCaptures {
                        worker_body_origin,
                        operands: carried_inputs.clone(),
                    });
            }

            // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the selected case body,
            // through the ONE authority both realizations share.
            //
            // ⛔ The operands are this frame's own: `ordinary` from the
            // `Parameter` slots and `carried_inputs` from the `Capture` slots,
            // each at the offset the descriptor assigns it. What the shared
            // function does with them is identical to what a fusion-local
            // composition does with the run assembled at its call edge, which
            // is exactly the property that makes the two realizations the same
            // body.
            let lowered = lower_continuation_selected_case_body(
                compiler,
                &mut builder,
                &ContinuationSelectedCaseBody {
                    id: unit.id,
                    continuation_origin: unit.continuation_origin,
                    producer_alternative: unit.producer_alternative,
                    recursive_position: unit.recursive_position,
                    worker_closure_origin: unit.worker_closure_origin,
                    worker_body_origin: unit.worker_body_origin,
                    worker_declared_arity: unit.worker_declared_arity,
                    worker_capture_count: unit.worker_capture_count,
                },
                envelope,
                &ordinary,
                &carried_inputs,
                retargeted_worker_body,
            )?;

            // The Result slot is WRITTEN here and never read.
            let word = match lowered {
                LoweringOperand::Carried(carried) => carried.word,
                LoweringOperand::Specialized(value) => {
                    compiler.emit_result(&mut builder, value)?.0
                }
            };
            let result_offset = i32::try_from(result_offset).map_err(|_| {
                backend_module("continuation result slot offset exceeds range".to_string())
            })?;
            builder
                .ins()
                .store(MemFlags::trusted(), word, frame, result_offset);
            let zero = builder.ins().iconst(types::I64, 0);
            builder.ins().return_(&[zero]);
            builder.seal_all_blocks();
            builder.finalize();
        }
        ambient.release(compiler);
        frame_scope.close(compiler)?;
        // Verify, then define THIS function -- a fresh context here would
        // define an empty body and silently discard everything emitted above.
        verify_cranelift_function(&func, module.isa())?;
        compiler.commit_aggregate_events()?;
        let mut ctx = module.make_context();
        std::mem::swap(&mut ctx.func, &mut func);
        module
            .define_function(id, &mut ctx)
            .map_err(|error| backend_module(error.to_string()))?;
        defined += 1;
    }
    Ok(defined)
}

/// **`RT-DECL-CLOSURE-PORT` `D5a` — define each generated producer execution
/// context.**
///
/// The body lowered here is the **raw worker body**, unchanged. What differs
/// from that body's own ordinary unit is only the environment it runs in: the
/// context's frame carries the raw parameter run *and* the enclosing
/// specialization's continuation inputs, so a nested producer inside that body
/// can reach operands raw `fn2` provably never receives.
///
/// ⛔ **Two different questions about the raw unit, and they are decided by
/// different authorities. Do not answer one with the other.**
///
/// 1. **Descriptor, provenance and source-binding authority are RETAINED.**
///    The raw `ClosureBody` keeps its own descriptor, its own ABI, and its
///    status as the source binding for this body — the template lowered here
///    *is* that body, unchanged. ⛔ Nothing about this retarget mutates,
///    unions, or suffixes the raw descriptor.
/// 2. **Executable `Function` declaration/definition membership is NOT
///    retained by default.** It is decided from the **post-retarget final
///    graph**, by `StaticTransitionPlan::template_only_worker_bodies`: a raw
///    worker every selecting specialization has retargeted — and whose carried
///    invocation also binds a generated context — is **template-only**, and is
///    absent from the emitted-`Function` population. A raw worker with any
///    remaining final raw call stays executable.
///
/// ⚠ **This paragraph previously said the raw unit "is still emitted and is
/// not retired", keeping its body and "simply losing one caller".** That was
/// the pre-`D5a` reading and it is superseded: it answers question 2 with
/// question 1's answer, so it is true of the descriptor and false of the
/// emitted population whenever the retarget is total. The surviving half of
/// the old claim is the useful one — *"a new owner for a body does not retire
/// the old owner's unit"* remains correct about **authority**, and says
/// nothing about **membership**.
///
/// ⛔ The emission owner bound here is `Specialization(enclosing)`, and
/// `defining_unit` stays the **raw** owner. Deriving one from the other is the
/// conflation `evt_609am4v7cdt5b` ruled against, and it is precisely because
/// this function lowers someone else's body that the two must be supplied
/// independently.
pub(super) fn define_continuation_context_bodies<M: Module>(
    module: &mut M,
    compiler: &mut Lowering<'_>,
    helpers: ArtifactHelpers<'_>,
    bundle: &UnitBundle,
    call_edges: &CallEdgeTargets,
) -> Result<usize, CraneliftBackendError> {
    struct OwnedContext {
        id: ContinuationContextId,
        enclosing: ContinuationSpecializationId,
        worker_body_origin: StaticOriginId,
        raw_owner: PredeclaredFunctionId,
        /// `RT-SRCBODY-BIND-ORDER` `D2` — the RAW owner's definition arm, read
        /// from the raw owner's own descriptor rather than judged here.
        ///
        /// This context lowers someone else's body, so the binding order that
        /// body needs is decided by the unit that owns it. Taking the answer
        /// from that descriptor is what makes the claimed equivalence hold by
        /// construction; deciding it independently here would make it two
        /// judgments that happen to agree.
        raw_owner_definition: AbiUnitDefinition,
        slots: Vec<AbiSlot>,
        offsets: Vec<u32>,
        header_parameters: u32,
        header_captures: u32,
    }
    // Own every projected fact before the loop: the projection borrows the plan
    // and definition below needs the compiler mutably.
    let unit_definitions = compiler
        .static_transition_plan
        .emittable_units()?
        .into_iter()
        .map(|unit| (unit.function(), unit.definition()))
        .collect::<Vec<_>>();
    let contexts = compiler
        .static_transition_plan
        .continuation_contexts()?
        .into_iter()
        .map(|context| {
            let (offsets, _frame_bytes) = context.slot_offsets()?;
            let raw_owner = context.raw_owner();
            // `D2` — a context whose raw owner has no descriptor is a context
            // whose body has no declared ABI, which is not a binding-order
            // question to answer conservatively.
            let raw_owner_definition = unit_definitions
                .iter()
                .find(|(function, _)| *function == raw_owner)
                .map(|(_, definition)| *definition)
                .ok_or_else(|| {
                    backend_module(
                        "a generated context's raw owner has no ABI descriptor".to_string(),
                    )
                })?;
            Ok(OwnedContext {
                id: context.id(),
                enclosing: context.enclosing_specialization(),
                worker_body_origin: context.worker_body_origin(),
                raw_owner,
                raw_owner_definition,
                slots: context.slots().to_vec(),
                offsets,
                header_parameters: context.header().parameters,
                header_captures: context.header().captures,
            })
        })
        .collect::<Result<Vec<_>, CraneliftBackendError>>()?;

    let worker_targets = resolve_worker_targets(&compiler.static_transition_plan, bundle)?;
    let mut defined = 0usize;
    for context in contexts {
        let id = bundle.context(context.id).ok_or_else(|| {
            backend_module(
                "a planned generated context was never forward-declared".to_string(),
            )
        })?;
        let slots = context.slots.as_slice();
        let offsets = context.offsets.as_slice();
        if slots.len() != offsets.len() {
            return Err(backend_module(
                "a generated context slot run disagrees with its own offset walk".to_string(),
            ));
        }
        let parameter_count = slots
            .iter()
            .filter(|slot| slot.kind == AbiSlotKind::Parameter)
            .count();
        let capture_count = slots
            .iter()
            .filter(|slot| slot.kind == AbiSlotKind::Capture)
            .count();
        // The header is the declared contract and the slot run is what the body
        // will actually walk; a disagreement here means the environment this
        // body binds is not the one its caller passes operands for.
        if u32::try_from(parameter_count).ok() != Some(context.header_parameters)
            || u32::try_from(capture_count).ok() != Some(context.header_captures)
        {
            return Err(backend_module(
                "a generated context's slot run disagrees with its declared frame header"
                    .to_string(),
            ));
        }
        let result_offset = slots
            .iter()
            .zip(offsets)
            .find(|(slot, _)| slot.kind == AbiSlotKind::Result)
            .map(|(_, offset)| *offset)
            .ok_or_else(|| {
                backend_module("generated context frame declares no result slot".to_string())
            })?;
        let trap_offset = slots
            .iter()
            .zip(offsets)
            .find(|(slot, _)| slot.kind == AbiSlotKind::Trap)
            .map(|(_, offset)| *offset)
            .ok_or_else(|| {
                backend_module("generated context frame declares no trap slot".to_string())
            })?;

        let sig = unit_signature(module);
        let mut func = Function::with_name_signature(UserFuncName::user(3, id.as_u32()), sig);
        let mut function_local = helpers.declare_in_func(module, &mut func, None);
        // The raw body's OWN call edges, declared into this function. They are
        // the raw owner's edges because the body is the raw owner's body; what
        // this context changes is the environment, never which callees the
        // source names.
        let declared_calls = call_edges.declare_in_func(context.raw_owner, module, &mut func)?;
        function_local.unit_calls = declared_calls.static_bodies;
        function_local.declaration_calls = declared_calls.declarations;
        function_local.worker_calls = worker_targets.declare_in_func(module, &mut func);
        // `D6b` -- no retarget happens in a generated context body, so the two
        // tables agree here. Populated anyway rather than left empty: the raw
        // route must resolve from ITS OWN table in every function, or the
        // resolution silently depends on which function it runs in.
        function_local.raw_worker_calls = function_local.worker_calls.clone();
        function_local.worker_templates = worker_targets.templates().clone();
        function_local.context_calls = declare_context_calls_in_func(
            module,
            &mut func,
            &compiler.static_transition_plan,
            bundle,
        )?;
        // `D5a`: this context's own causal call refs, selected by the EMISSION
        // owner. ⛔ Not by `raw_owner` -- that is the filter that would hand this
        // function the raw unit's tokens and leave its own undeclared.
        let emission_owner = ContinuationEmissionOwner::Specialization(context.enclosing);
        function_local.continuation_calls = match compiler.continuation_claims.as_ref() {
            Some(ledger) => ledger.declare_owned_in_func(
                emission_owner,
                module,
                &mut func,
                &compiler.static_transition_plan,
            )?,
            None => BTreeMap::new(),
        };
        if let Some(ledger) = compiler.continuation_claims.as_mut() {
            ledger.record_declared(function_local.continuation_calls.keys().cloned())?;
        }
        // Contract 3, on this context: the projection is taken before the
        // function is defined and keyed on the emission owner this pass is about
        // to bind.
        let result_edges = compiler
            .static_transition_plan
            .continuation_result_edges_owned_by(emission_owner)?;

        // `D8n` — this generated Function's own checked-frame consumption
        // transaction, spanning the generated-context body exactly. ⛔ Opened before the builder and
        // closed after it, so every branch scope inside nests within it.
        let frame_scope = CheckedFrameFunctionScope::open(compiler)?;
        let mut func_ctx = FunctionBuilderContext::new();
        {
            let mut builder = FunctionBuilder::new(&mut func, &mut func_ctx);
            let entry = builder.create_block();
            builder.append_block_params_for_function_params(entry);
            builder.switch_to_block(entry);
            let envelope_pointer = builder.block_params(entry)[0];
            let frame = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                envelope_pointer,
                crate::activation_services::UNIT_CALL_FRAME_SLOTS,
            );
            let host_dispatch_context = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                envelope_pointer,
                crate::activation_services::UNIT_CALL_FRAME_HOST_DISPATCH_CONTEXT,
            );
            let services = builder.block_params(entry)[1];
            let native_int_arena = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                services,
                crate::activation_services::SERVICES_NATIVE_INT_ARENA,
            );
            Lowering::require_nonzero(&mut builder, native_int_arena);
            let boundary_arena = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                services,
                crate::activation_services::SERVICES_BOUNDARY_ARENA,
            );
            Lowering::require_nonzero(&mut builder, boundary_arena);
            function_local.host_dispatch_context = Some(host_dispatch_context);
            function_local.native_int_arena = Some(native_int_arena);
            function_local.boundary_arena = Some(boundary_arena);
            function_local.services_pointer = Some(services);
            function_local.bind_unit_trap_frame(
                frame,
                i32::try_from(trap_offset).map_err(|_| {
                    backend_module("generated context trap slot offset exceeds range".to_string())
                })?,
            )?;
            compiler.function_local = function_local;
            compiler.open_aggregate_events(id)?;
            // `D8o` — same binding, same unchanged domain: the emission owner is
            // the enclosing specialization and `defining_unit` stays the RAW
            // owner, which is the distinction the two fields exist to keep.
            // ⛔ After `open_aggregate_events`, so the observation this binding
            // writes is labelled with the Function it belongs to.
            let ambient = AmbientBodyAuthority::bind(compiler, emission_owner, context.raw_owner);
            // `D8o` — a generated CONTEXT body, whose owner is a Specialization
            // and whose kind is not.
            #[cfg(test)]
            crate::cranelift_backend::lowering::record_d8o_body_key(
                compiler.defining_function_id,
                crate::cranelift_backend::lowering::D8oBodyKey::GeneratedContext(context.id),
            );

            // The environment: the Parameter run then the Capture run. This is
            // the SAME conversion `define_unit_body` applies, which is why the
            // raw body's binder positions resolve identically here -- the
            // parameter prefix is byte-for-byte the run its own unit binds, and
            // the continuation inputs sit strictly after it.
            //
            // `RT-SRCBODY-BIND-ORDER` `D2`: "the same walk" is no longer enough
            // to say that, because the walk and the environment are now two
            // orders. The equivalence is preserved by taking the conversion
            // from the RAW OWNER's definition arm -- so if that body's own unit
            // reverses its parameter run, this context reverses the identical
            // prefix, and if it does not, neither does this. The capture run is
            // the enclosing specialization's ordered input projection and is
            // positional by construction, so it is never reversed.
            let mut context_parameters = Vec::new();
            let mut context_captures = Vec::new();
            #[cfg(test)]
            let mut context_parameter_ordinals = Vec::new();
            #[cfg(test)]
            let mut context_capture_ordinals = Vec::new();
            for (slot, offset) in slots.iter().zip(offsets) {
                if !matches!(slot.kind, AbiSlotKind::Parameter | AbiSlotKind::Capture) {
                    continue;
                }
                let offset = i32::try_from(*offset).map_err(|_| {
                    backend_module(
                        "generated context slot offset exceeds addressable range".to_string(),
                    )
                })?;
                let word = builder
                    .ins()
                    .load(types::I64, MemFlags::trusted(), frame, offset);
                let binding = LoweringEnvironmentBinding::Value(LoweringOperand::Carried(
                    CarriedBoundaryWord { word },
                ));
                match slot.kind {
                    AbiSlotKind::Parameter => {
                        #[cfg(test)]
                        context_parameter_ordinals.push(slot.ordinal);
                        context_parameters.push(binding)
                    }
                    _ => {
                        #[cfg(test)]
                        context_capture_ordinals.push(slot.ordinal);
                        context_captures.push(binding)
                    }
                }
            }
            let context_converts = source_body_binding_order(context.raw_owner_definition)?;
            if context_converts {
                context_parameters.reverse();
                #[cfg(test)]
                context_parameter_ordinals.reverse();
            }
            // `RT-SRCBODY-BIND-ORDER` `D3` control 4, amended (Architect,
            // producer-wide) -- the TRANSITION SENTINEL, sited at the producer
            // edge rather than inside one witness's test.
            //
            // **This gate is designed to go RED, and that red is its purpose.**
            // `D2` says a generated context binds its raw owner's parameter run
            // in the same order that owner's own unit would. `D1`'s conversion
            // reverses that run, and reversal is the IDENTITY on a run of
            // length one -- so at unary arity `D2` is inert and its cross-host
            // equivalence is not observable at all. The first generated-context
            // worker of arity two or more makes it observable, and at that
            // moment the equivalence obligation this node deferred becomes live
            // and UNMEASURED.
            //
            // **Sited here because a test can only watch the compiles it runs.**
            // The first cut asserted this bound inside the `px8tr` control, over
            // the observations of that one compile. A multi-parameter worker
            // introduced by any other program would never have entered that
            // observation vector, so the sentinel would have stayed green while
            // the obligation activated -- watching a witness while claiming a
            // population. Every generated context this crate builds passes
            // through this loop, so the gate is closed over the PRODUCER.
            //
            // **The residual, stated rather than implied.** `cfg(test)` here
            // means the `ken-runtime` lib-test build: a generated context built
            // while compiling an integration-test binary or a downstream crate
            // does not arm this gate. The fixture population that could
            // introduce a multi-parameter worker is itself `cfg(test)`
            // (`mod test_objects`), so it lies inside the reach; a worker
            // arising only from a real Ken program exercised solely by an
            // integration test lies outside it.
            //
            // Do NOT satisfy this by relaxing the bound -- a `<= 2` restates the
            // current population as the contract and destroys the gate. The
            // retiring event is the introduction of the arity-two worker, and
            // the deliverable then is the equivalence control, not a wider
            // bound. `d3_generated_context_arity_sentinel_edge_is_reached` is
            // the reaching positive control that keeps this non-vacuous.
            #[cfg(test)]
            assert!(
                context_parameter_ordinals.len() <= 1,
                "a generated-context worker declares {} parameters. D2's binding order is no \
                 longer inert, so its cross-host equivalence is now OBSERVABLE and UNMEASURED. \
                 The deliverable is the cross-host equivalence control -- that this body binds \
                 the same order in its own unit and in the context that lowers it -- not a \
                 wider bound at this gate. Recorded ordinals: {context_parameter_ordinals:?}",
                context_parameter_ordinals.len()
            );
            #[cfg(test)]
            srcbody_bind_order_record(SrcbodyBindOrderObservation {
                host: SrcbodyBindHost::GeneratedContext,
                definition: context.raw_owner_definition,
                converted: context_converts,
                body_origin: context.worker_body_origin,
                parameter_ordinals: context_parameter_ordinals,
                capture_ordinals: context_capture_ordinals,
            });
            let mut env = context_parameters;
            env.extend(context_captures);

            let body = compiler.retained_body_occurrence(context.worker_body_origin)?;
            let lowered = compiler.lower_expr(&mut builder, body, &env)?;
            // The detached-result seat, live in this context. Every operand its
            // capture projection names is reachable from `env` above.
            let lowered = compiler.eliminate_detached_producer_continuation(
                &mut builder,
                &result_edges,
                lowered,
                &env,
            )?;
            let word = match lowered {
                LoweringOperand::Carried(word) => Some(word.word),
                LoweringOperand::Specialized(Lowered::Trap(trap)) => {
                    compiler.emit_current_trap(&mut builder, &trap)?;
                    None
                }
                LoweringOperand::Specialized(value) => Some(
                    compiler
                        .transfer_unit_result_into_carrier(
                            &mut builder,
                            context.worker_body_origin,
                            &value,
                        )?
                        .word,
                ),
            };
            if let Some(word) = word {
                builder.ins().store(
                    MemFlags::trusted(),
                    word,
                    frame,
                    i32::try_from(result_offset).map_err(|_| {
                        backend_module(
                            "generated context result slot offset exceeds range".to_string(),
                        )
                    })?,
                );
            }
            let status = builder.ins().iconst(types::I64, 0);
            builder.ins().return_(&[status]);
            builder.seal_all_blocks();
            builder.finalize();
            ambient.release(compiler);
        }
        frame_scope.close(compiler)?;
        // The same emission-seam gate every other generated function passes: the
        // callee of each recorded causal emission is decoded back out of THIS
        // finished CLIF and compared with the planner-issued target.
        compiler.verify_emitted_continuation_calls(&func, bundle)?;
        // `D8j` — the composed relation's own gate, beside the direct one and
        // never inside it: the two answer different questions about different
        // callees.
        compiler.verify_recorded_composed_discharges(&func, bundle)?;
        #[cfg(test)]
        crate::cranelift_backend::lowering::record_d8j_discharged(
            compiler.function_local.composed_discharges.keys().cloned(),
        );
        // `D8k` -- the composed half of the partition, accumulated from the
        // VERIFIED relation and never from the direct instruction map.
        if let Some(ledger) = compiler.continuation_claims.as_mut() {
            ledger.record_composed(
                compiler.function_local.composed_discharges.keys().cloned(),
                emission_owner,
            )?;
        }
        if let Some(ledger) = compiler.continuation_claims.as_mut() {
            ledger.record_emitted(
                compiler.function_local.continuation_emissions.keys().cloned(),
            )?;
        }
        verify_cranelift_function(&func, module.isa())?;
        compiler.commit_aggregate_events()?;
        let mut ctx = module.make_context();
        std::mem::swap(&mut ctx.func, &mut func);
        module
            .define_function(id, &mut ctx)
            .map_err(|error| backend_module(error.to_string()))?;
        defined += 1;
    }
    Ok(defined)
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` — define one `Function` per installed
/// fused region: the producer's body and the consumer's suffix, in one frame.**
///
/// **What this actually binds, and it is PROVISIONAL: one authority, the
/// producer's, around the whole combined lowering.** The single
/// `AmbientBodyAuthority::bind` below takes
/// `Predeclared(producer_owner)`/`producer_owner` and spans the producer's body
/// *and* the consumer's suffix together. There is no consumer-phase switch in
/// this function.
///
/// **That is a known gap, not the design, and it is EXCLUDED from this cut.**
/// The consumer's case bodies are therefore lowered against the producer's
/// source-lookup authority. Armed and left this way it would be a wrong program
/// rather than a missing one, because the binders would resolve — to the wrong
/// terms.
///
/// **What makes that harmless today is the EMPTY FUSION POPULATION, and not
/// `D2F_EMITTER_ARMED`.** This function is called unconditionally on every
/// production compile; it is only the *installer* that the constant gates. So it
/// defines zero bodies because `continuation_fusions()` yields nothing to
/// iterate, which is a consequence of the gated install rather than a property
/// of this call site. An earlier wording named the constant as the reason, which
/// let an auditor confirm `false` and stop one hop short of the guarantee that
/// actually holds. **The reachable-empty path is deliberate** — guarding this
/// call as well would make the zero-fusion case take a different path from the
/// non-zero case, which is the shape the node is trying to keep exercised.
///
/// **The excluded repair, named so it is not re-derived:** the ambient authority
/// must move `Predeclared(producer) -> Predeclared(consumer) -> producer` across
/// the suffix, and the fused region needs its own checked-frame adoption,
/// because a `CheckedFrameFunctionScope` is a per-`Function` transaction while a
/// fused region's frame spans the producer's body and the consumer's suffix
/// across one. Both are later Architect-ruled wiring. Do not read the paragraph
/// above as a statement that either exists.
///
/// **The suffix is run ONCE, here.** Node `:650` measured that redirecting the
/// producer invocation alone leaves the consumer's own suffix live and executes
/// it twice. That is why the redirect, this definition, and the consumer takeover
/// are one irreducible core: any two of the three, without the third, is a wrong
/// artifact and not a partial one.
///
/// **No new elimination machinery.** The elimination is
/// `lower_computational_producer_expr` with a single-frame eliminator stack — the
/// same entry the producer dispatcher's own fallback arm uses. It handles the
/// carried and the specialized phase already, so nothing here re-derives case
/// selection, and nothing synthesizes an `ActiveContinuationFrame`.
///
/// **The claim is READ, not consumed.** The takeover at the consumer's seat is
/// the one consumption, and consuming here would make a definition spend the
/// affine right the redirect's seat still needs.
/// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — which origin keys the recursive self
/// edge and which origin records its call site.**
///
/// A two-line function because the choice is the whole content: the map key and
/// `origin` are the **callee body**, and `call_site_origin` is the **consuming
/// call**. They are different occurrences and this is the only place that says
/// so, so a control can assert the production decision instead of a copy of it.
///
/// **Why it is worth naming at all.** On the `R3` witness the claim's `seat`,
/// its `producer_body` and its redirect's callee entry ALL print `37` while the
/// consuming call is `17`. A fold of call site into body type-checks, and a
/// control whose expected values are every `37` still passes under the fold —
/// separating `17` from `37` is the only thing that catches it.
pub(super) fn fusion_self_edge_identities(
    producer_body: StaticOriginId,
    consuming_call: StaticOriginId,
) -> (StaticOriginId, StaticOriginId) {
    (producer_body, consuming_call)
}

pub(super) fn define_static_continuation_fusion_bodies<M: Module>(
    module: &mut M,
    compiler: &mut Lowering<'_>,
    helpers: ArtifactHelpers<'_>,
    bundle: &UnitBundle,
    call_edges: &CallEdgeTargets,
) -> Result<usize, CraneliftBackendError> {
    struct OwnedFusion {
        id: StaticContinuationFusionId,
        producer_owner: PredeclaredFunctionId,
        consumer_owner: PredeclaredFunctionId,
        producer_body: StaticOriginId,
        /// `RT-LEXICAL-R3-FUSION-EMITTER` `D3` -- the CALL SITE the moved
        /// suffix's claimed IH invocation is authorized at. Deliberately kept
        /// beside `producer_body` rather than folded into it: this is the
        /// occurrence that calls, that is the body called, and in this fixture
        /// they are different origins that a numeric coincidence elsewhere
        /// could hide.
        consuming_call: StaticOriginId,
        consuming_callee: StaticOriginId,
        redirect_callee: StaticOriginId,
        continuation_origin: StaticOriginId,
        /// `RT-LEXICAL-R3-FUSION-EMITTER` `D2` — the claim's consumer frame
        /// identity, re-entered locally inside the fused function.
        checked_frame_id: u64,
        slots: Vec<AbiSlot>,
        offsets: Vec<u32>,
        /// The INSTALLED fusion frame's whole header, carried rather than
        /// rebuilt from its parts. `D3`'s recursive self edge targets this
        /// definition, so it must present the frame contract the region was
        /// installed with; reassembling a header here is how it would come to
        /// differ in a field nobody thought to copy.
        header: AbiFrameHeader,
        header_parameters: u32,
        header_captures: u32,
    }
    // Own every projected fact before the loop, for the same reason the context
    // pass does: the projection borrows the plan and definition below needs the
    // compiler mutably.
    //
    // The two ORIGINS come from the claim, not from the view. The view carries
    // the plane's key; the claim carries the region the ledger actually admitted,
    // and those are the same fact only when preflight accepted. Reading the view
    // here would define a body for a region whose claim was refused.
    let fusions = {
        let claims = compiler.fusion_claims.as_ref();
        compiler
            .static_transition_plan
            .continuation_fusions()?
            .into_iter()
            .map(|fusion: StaticContinuationFusionView<'_>| {
                let (offsets, _frame_bytes) = fusion.slot_offsets()?;
                let claim = claims
                    .and_then(|ledger| ledger.claim(fusion.id()))
                    .ok_or_else(|| {
                        backend_module(
                            "an installed static continuation fusion has no outstanding region \
                             claim, so its definition would take over a suffix nothing admitted"
                                .to_string(),
                        )
                    })?;
                Ok(OwnedFusion {
                    id: fusion.id(),
                    producer_owner: claim.producer_owner(),
                    consumer_owner: claim.consumer_owner(),
                    producer_body: claim.producer_body(),
                    consuming_call: claim.consuming_call(),
                    consuming_callee: claim.consuming_callee(),
                    redirect_callee: claim.redirect().callee_origin(),
                    continuation_origin: claim.continuation_origin(),
                    // `D2` -- the consumer frame identity the claim was
                    // preflighted against, carried so the fused body re-enters
                    // THAT frame rather than resolving one of its own.
                    checked_frame_id: claim.checked_transport().frame_id(),
                    slots: fusion.slots().to_vec(),
                    offsets,
                    header: fusion.header(),
                    header_parameters: fusion.header().parameters,
                    header_captures: fusion.header().captures,
                })
            })
            .collect::<Result<Vec<_>, CraneliftBackendError>>()?
    };

    let worker_targets = resolve_worker_targets(&compiler.static_transition_plan, bundle)?;
    let mut defined = 0usize;
    for fusion in fusions {
        let id = bundle.fusion(fusion.id).ok_or_else(|| {
            backend_module(
                "an installed static continuation fusion was never forward-declared".to_string(),
            )
        })?;
        let slots = fusion.slots.as_slice();
        let offsets = fusion.offsets.as_slice();
        if slots.len() != offsets.len() {
            return Err(backend_module(
                "a fused region slot run disagrees with its own offset walk".to_string(),
            ));
        }
        let parameter_count = slots
            .iter()
            .filter(|slot| slot.kind == AbiSlotKind::Parameter)
            .count();
        let capture_count = slots
            .iter()
            .filter(|slot| slot.kind == AbiSlotKind::Capture)
            .count();
        // The header is the declared contract and the slot run is what this body
        // will actually walk; a disagreement means the environment this body
        // binds is not the one the redirected invocation passes operands for.
        if u32::try_from(parameter_count).ok() != Some(fusion.header_parameters)
            || u32::try_from(capture_count).ok() != Some(fusion.header_captures)
        {
            return Err(backend_module(
                "a fused region's slot run disagrees with its declared frame header".to_string(),
            ));
        }
        let result_offset = slots
            .iter()
            .zip(offsets)
            .find(|(slot, _)| slot.kind == AbiSlotKind::Result)
            .map(|(_, offset)| *offset)
            .ok_or_else(|| {
                backend_module("fused region frame declares no result slot".to_string())
            })?;
        let trap_offset = slots
            .iter()
            .zip(offsets)
            .find(|(slot, _)| slot.kind == AbiSlotKind::Trap)
            .map(|(_, offset)| *offset)
            .ok_or_else(|| {
                backend_module("fused region frame declares no trap slot".to_string())
            })?;

        let sig = unit_signature(module);
        let mut func = Function::with_name_signature(UserFuncName::user(4, id.as_u32()), sig);
        let mut function_local = helpers.declare_in_func(module, &mut func, None);
        // The PRODUCER's own call edges, declared into this function. This is
        // what carries the inherited producer edge: the producer's body still
        // names the callees the source named, and it is only its *host* that
        // changed. Not the consumer's edges — the consumer's redirected
        // invocation is the edge that reaches this function, not one inside it.
        let declared_calls = call_edges.declare_in_func(fusion.producer_owner, module, &mut func)?;
        function_local.unit_calls = declared_calls.static_bodies;
        // ---- `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — THE DEFINITION-LOCAL
        // ---- FUSION-RECURSIVE SELF EDGE (Architect, via `evt_6fzg11hpvfp4w`).
        //
        // The suffix moved into this function, and its claimed IH invocation is
        // a **recursive call to this same `Fusion(id)`** — not a no-op forward
        // of a current result, and not a call to the standalone producer, whose
        // body this definition now owns.
        //
        // Without this the fused body reaches
        // `call_declared_unit(producer_body)` against a table holding only the
        // producer's inherited edges and refuses with *"retained body ... has no
        // graph-derived call target in this unit"*. That refusal is correct and
        // stays: this supplies the edge the body genuinely contains rather than
        // relaxing the check.
        //
        // **Authorized from the preflighted claim, never from the failure.** The
        // ruling forbids inferring this edge from a lookup miss, from numeric
        // coincidence between origins, from body shape, or from ambient
        // "inside a fusion" state — so each fact below is read off `claim`.
        //
        // **Call-site identity is kept distinct from body identity.** The key
        // and `origin` are the callee body; `call_site_origin` is
        // `claim.consuming_call()`. In this fixture the body and the claim's
        // seat both print `37` while the consuming call is `17`; folding them
        // would compile and would make a wrong-call-site edge indistinguishable
        // from a right one.
        //
        // **The header and slots are the installed FUSION frame's**, never the
        // producer's — the callee is this fused definition, so the frame
        // contract is the one it was installed with.
        //
        // This is a **separate, definition-local obligation** from the external
        // consumer-to-fusion redirect, which remains the sole affine external
        // redirect. It consumes no region claim and records no redirect.
        // The half of the ruled verification that is genuinely derivable HERE:
        // the redirect's producer entry and the claim's producer body must be
        // the same occurrence. They reach the claim by different routes --
        // `producer_body` from `invocation_callee_entry` via the unique static
        // body triple, `redirect` from `fusion_redirect_target` -- so their
        // agreement is a real cross-check rather than a field compared with
        // itself.
        //
        // MEASURED, and stated rather than implied: `consuming_callee` does NOT
        // equal `producer_body` and must not be compared to it. On this witness
        // the consuming call is `17`, its callee occurrence `16`, and the
        // producer body `37`. The callee reaches the body through an IH
        // BINDING, and `CheckedIhBinding` names a `frame_origin` and a recursive
        // position -- not a body. An earlier draft of this guard compared the
        // two ids directly and refused every lawful region.
        //
        // ---- CORRECTION. An earlier revision of THIS comment said that
        // ---- resolution "is established at preflight". That was FALSE.
        //
        // Architect, relayed at `evt_45xd3px862ejs`: preflight's
        // `BinderAgreement` proves only marginal facts -- that the consuming
        // callee is an IH at the admitted frame and recursive position, and that
        // the result root equals the invocation callee entry. It does NOT prove
        // that this exact callee/binder resolves to that producer body.
        //
        // ---- AND THAT RELATION IS NOW CLOSED, PLANNER-SIDE. Ruled at
        // ---- `evt_2rw6vhq8xrqcm`; landed as
        // ---- `FusionClaimRefusal::BinderBodyResolution`.
        //
        // Preflight now re-resolves the consuming callee through the planner's
        // own binding authority and resolves that binder to a body, refusing
        // BEFORE the claim is issued unless it is the body being redirected. So
        // by the time a claim reaches this seat the relation is a checked
        // property of the certificate, not an assumption -- and `producer_body`
        // is that checked common result.
        //
        // ⇒ **What lowering keeps is still exactly the independent cross-check
        // below, and closing the relation upstream is not a licence to grow
        // it.** A second body authority here was ruled out in the same message:
        // re-deriving the binder relation in lowering would create a second
        // planner, and `ih_bindings` and `SemanticIr::child_origin` are
        // planner-private precisely so that cannot happen quietly.
        if fusion.redirect_callee != fusion.producer_body {
            return Err(backend_module(
                "a fused region's redirect names a producer entry other than the claim's producer \
                 body, so the definition-local recursive edge would target a body this claim \
                 never admitted"
                    .to_string(),
            ));
        }
        // The two identities, chosen by one shared function so the control that
        // separates them exercises the production decision rather than a copy of
        // it.
        let (self_edge_body, self_edge_call_site) =
            fusion_self_edge_identities(fusion.producer_body, fusion.consuming_call);
        let self_edge = DeclaredUnitCall {
            function: module.declare_func_in_func(id, &mut func),
            origin: self_edge_body,
            call_site_origin: self_edge_call_site,
            header: fusion.header,
            slots: slots.to_vec(),
            offsets: offsets.to_vec(),
        };
        // INSERTED, and its absence beforehand is required rather than assumed —
        // the producer's inherited edges must not already answer for this body,
        // or the suffix would have two answers to one lookup and the standalone
        // producer would stay reachable beside its fused definition.
        if function_local
            .unit_calls
            .insert(self_edge_body, self_edge)
            .is_some()
        {
            return Err(backend_module(
                "a fused definition already holds a call target for the body it owns, so its \
                 recursive suffix edge would be one of two answers to the same lookup"
                    .to_string(),
            ));
        }
        function_local.declaration_calls = declared_calls.declarations;
        function_local.worker_calls = worker_targets.declare_in_func(module, &mut func);
        // No retarget happens in a fused body, so the two tables agree. Populated
        // anyway rather than left empty, for the same reason as every other
        // generated function: the raw route must resolve from its own table here,
        // or resolution silently depends on which function it runs in.
        function_local.raw_worker_calls = function_local.worker_calls.clone();
        function_local.worker_templates = worker_targets.templates().clone();
        function_local.context_calls = declare_context_calls_in_func(
            module,
            &mut func,
            &compiler.static_transition_plan,
            bundle,
        )?;
        // The causal call refs declared here are the **PRODUCER's**, under
        // `Predeclared(producer_owner)`.
        //
        // The planner issues causal tokens for the producer's body under that
        // owner, and installing body ownership removed the producer's standalone
        // `Function` — so this is now the only `Function` that lowers that body,
        // and therefore the only one that can declare its refs.
        //
        // **There is deliberately no `Fusion(id)` owner in this function.** An
        // earlier cut minted `ContinuationEmissionOwner::Fusion(fusion.id)` here
        // and described it as the ambient authority. It was never bound to
        // anything: the sole `AmbientBodyAuthority::bind` below takes
        // `causal_owner`, and the planner issues no `Fusion`-owned tokens, so
        // asking for them returns the empty set and the producer's first causal
        // call refuses with *"the claimed continuation target was not declared
        // into this function"*. Whether the fused region ever becomes a causal
        // emission owner in its own right is later Architect-ruled wiring and is
        // excluded here; **it is not the case today, and this variable must not
        // be reintroduced to suggest otherwise.**
        //
        // NOT the union with the consumer's tokens. The consumer's own
        // `Function` still exists and still declares them; declaring them here
        // as well would be a second declaration of one token, which the claim
        // ledger's exact-set law refuses — correctly.
        let causal_owner = ContinuationEmissionOwner::Predeclared(fusion.producer_owner);
        function_local.continuation_calls = match compiler.continuation_claims.as_ref() {
            Some(ledger) => ledger.declare_owned_in_func(
                causal_owner,
                module,
                &mut func,
                &compiler.static_transition_plan,
            )?,
            None => BTreeMap::new(),
        };
        if let Some(ledger) = compiler.continuation_claims.as_mut() {
            ledger.record_declared(function_local.continuation_calls.keys().cloned())?;
        }
        let result_edges = compiler
            .static_transition_plan
            .continuation_result_edges_owned_by(causal_owner)?;

        let frame_scope = CheckedFrameFunctionScope::open(compiler)?;
        let mut func_ctx = FunctionBuilderContext::new();
        {
            let mut builder = FunctionBuilder::new(&mut func, &mut func_ctx);
            let entry = builder.create_block();
            builder.append_block_params_for_function_params(entry);
            builder.switch_to_block(entry);
            let envelope_pointer = builder.block_params(entry)[0];
            let frame = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                envelope_pointer,
                crate::activation_services::UNIT_CALL_FRAME_SLOTS,
            );
            let host_dispatch_context = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                envelope_pointer,
                crate::activation_services::UNIT_CALL_FRAME_HOST_DISPATCH_CONTEXT,
            );
            let services = builder.block_params(entry)[1];
            let native_int_arena = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                services,
                crate::activation_services::SERVICES_NATIVE_INT_ARENA,
            );
            Lowering::require_nonzero(&mut builder, native_int_arena);
            let boundary_arena = builder.ins().load(
                module.target_config().pointer_type(),
                MemFlags::trusted(),
                services,
                crate::activation_services::SERVICES_BOUNDARY_ARENA,
            );
            Lowering::require_nonzero(&mut builder, boundary_arena);
            function_local.host_dispatch_context = Some(host_dispatch_context);
            function_local.native_int_arena = Some(native_int_arena);
            function_local.boundary_arena = Some(boundary_arena);
            function_local.services_pointer = Some(services);
            function_local.bind_unit_trap_frame(
                frame,
                i32::try_from(trap_offset).map_err(|_| {
                    backend_module("fused region trap slot offset exceeds range".to_string())
                })?,
            )?;
            compiler.function_local = function_local;
            compiler.open_aggregate_events(id)?;

            // The two runs, walked once and kept SEPARATE. The context pass
            // concatenates them because one body binds both; here they are two
            // environments for two BODIES — the producer's and the consumer's
            // suffix — and concatenating would hand the producer the consumer's
            // captures.
            //
            // Separate ENVIRONMENTS, not separate authorities. The single
            // ambient authority below spans both; see the bind for what is
            // provisional about that.
            let mut parameters = Vec::new();
            let mut captures = Vec::new();
            for (slot, offset) in slots.iter().zip(offsets) {
                if !matches!(slot.kind, AbiSlotKind::Parameter | AbiSlotKind::Capture) {
                    continue;
                }
                let offset = i32::try_from(*offset).map_err(|_| {
                    backend_module("fused region slot offset exceeds addressable range".to_string())
                })?;
                let word = builder
                    .ins()
                    .load(types::I64, MemFlags::trusted(), frame, offset);
                let binding = LoweringEnvironmentBinding::Value(LoweringOperand::Carried(
                    CarriedBoundaryWord { word },
                ));
                // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the CURRENT FUSED
                // FRAME's ordered operand run, recorded in this same single
                // walk and in descriptor order.
                //
                // `fused_redirect_inputs` resolves `claim.inputs()` by indexing
                // `defining_abi_operands` of the function making the call. Until
                // the suffix moved in, that function was always the consumer's
                // predeclared unit, which populates the run in its own slot
                // walk. The fused definition never did, so the recursive self
                // edge above refused with *"names entry ABI position 0 outside
                // the calling function's 0 operands"* — an empty run, not a
                // wrong one.
                //
                // Written HERE rather than reassembled afterwards for the reason
                // the ordinary unit body states: one walk loads each slot once,
                // and a second pass that rebuilds the order is how the operand
                // run comes to disagree with the descriptor it was declared
                // from.
                compiler
                    .function_local
                    .defining_abi_operands
                    .push(LoweringOperand::Carried(CarriedBoundaryWord { word }));
                match slot.kind {
                    AbiSlotKind::Parameter => parameters.push(binding),
                    _ => captures.push(binding),
                }
            }

            // THE FUSION ITSELF: the producer's body is lowered THROUGH the
            // consumer's eliminator, in one pass, not to a value and then into
            // one.
            //
            // **Lowering the producer first and eliminating afterwards is
            // the defect, and it is the natural way to write this.** MEASURED:
            // that shape reaches the producer's own `"a computational recursor
            // closure names an in-flight activation, not a transferable value"`
            // refusal *inside the fused function* — the identical refusal fusion
            // exists to remove, merely relocated. The producer's body has no
            // value representation to hand anywhere; that it does not is the
            // whole reason the region is fused rather than called.
            //
            // ⇒ The eliminator goes ON THE STACK and the producer dispatcher
            // consumes it, so the producer's selected case body feeds the
            // consumer's suffix with no intermediate activation materialized.
            // This is the same entry the dispatcher's own nested arm uses to
            // fuse an inner eliminator ahead of an outer stack.
            //
            // **ONE authority spans BOTH, and that is provisional.** The bind
            // below is `Predeclared(producer_owner)`/`producer_owner` and it is
            // held across the whole combined lowering, so the consumer's case
            // bodies are lowered under the producer's source-lookup authority.
            // The ruled shape moves it
            // `Predeclared(producer) -> Predeclared(consumer) -> producer` across
            // the suffix; that switch, and the fused region's own checked-frame
            // adoption, are later Architect-ruled wiring and are **excluded from
            // this cut**. Fusing the two lowering steps is what removed the seam
            // the switch used to sit at, so it has to be reintroduced
            // deliberately rather than restored.
            let lowered = {
                let ambient =
                    AmbientBodyAuthority::bind(compiler, causal_owner, fusion.producer_owner);
                // `RT-LEXICAL-R3-FUSION-EMITTER` `D1` — arm the interior switch
                // for the extent of THIS region's body, and no longer.
                //
                // The key is the region's own `continuation_origin` and the fact
                // is its own `consumer_owner`; both come from the claim, so
                // nothing downstream infers either. Restored to whatever was
                // held on the way out rather than cleared, for the same reason
                // `AmbientBodyAuthority` restores: a nested definition pass must
                // not be handed `None` when its caller held a key.
                let enclosing_switch = compiler.fused_consumer_authority.replace((
                    fusion.continuation_origin,
                    fusion.consumer_owner,
                ));
                let lowered = fuse_producer_through_consumer_suffix(
                    compiler,
                    &mut builder,
                    fusion.producer_body,
                    fusion.continuation_origin,
                    fusion.checked_frame_id,
                    &parameters,
                    &captures,
                );
                compiler.fused_consumer_authority = enclosing_switch;
                ambient.release(compiler);
                lowered?
            };

            let lowered = compiler.eliminate_detached_producer_continuation(
                &mut builder,
                &result_edges,
                lowered,
                &captures,
            )?;
            let word = match lowered {
                LoweringOperand::Carried(word) => Some(word.word),
                LoweringOperand::Specialized(Lowered::Trap(trap)) => {
                    compiler.emit_current_trap(&mut builder, &trap)?;
                    None
                }
                LoweringOperand::Specialized(value) => Some(
                    compiler
                        .transfer_unit_result_into_carrier(
                            &mut builder,
                            fusion.continuation_origin,
                            &value,
                        )?
                        .word,
                ),
            };
            if let Some(word) = word {
                builder.ins().store(
                    MemFlags::trusted(),
                    word,
                    frame,
                    i32::try_from(result_offset).map_err(|_| {
                        backend_module("fused region result slot offset exceeds range".to_string())
                    })?,
                );
            }
            let status = builder.ins().iconst(types::I64, 0);
            builder.ins().return_(&[status]);
            builder.seal_all_blocks();
            builder.finalize();
        }
        frame_scope.close(compiler)?;
        compiler.verify_emitted_continuation_calls(&func, bundle)?;
        compiler.verify_recorded_composed_discharges(&func, bundle)?;
        #[cfg(test)]
        crate::cranelift_backend::lowering::record_d8j_discharged(
            compiler.function_local.composed_discharges.keys().cloned(),
        );
        if let Some(ledger) = compiler.continuation_claims.as_mut() {
            ledger.record_composed(
                compiler.function_local.composed_discharges.keys().cloned(),
                causal_owner,
            )?;
        }
        if let Some(ledger) = compiler.continuation_claims.as_mut() {
            ledger.record_emitted(
                compiler.function_local.continuation_emissions.keys().cloned(),
            )?;
        }
        verify_cranelift_function(&func, module.isa())?;
        compiler.commit_aggregate_events()?;
        if let Some(ledger) = compiler.fusion_claims.as_mut() {
            ledger.record_defined(fusion.id)?;
        }
        let mut ctx = module.make_context();
        std::mem::swap(&mut ctx.func, &mut func);
        module
            .define_function(id, &mut ctx)
            .map_err(|error| backend_module(error.to_string()))?;
        defined += 1;
    }
    Ok(defined)
}

/// Lower the producer's body **through** the consumer's continuation occurrence.
///
/// Split out so the fused body reads as one step, and so the "the suffix must be
/// an elimination" refusal has exactly one site. It builds a **single-frame**
/// eliminator stack: this is the consumer's own suffix and nothing else, so
/// composing a second frame here would run a continuation the claim does not own.
///
/// The eliminator's environment is the **captures** and the producer's is the
/// **parameters**. They are two runs of one frame and are never concatenated:
/// the consumer's case bodies index its own continuation inputs, and handing
/// them the producer's parameter prefix would shift every one of those indices.
fn fuse_producer_through_consumer_suffix(
    compiler: &mut Lowering<'_>,
    builder: &mut FunctionBuilder<'_>,
    producer_body: StaticOriginId,
    continuation_origin: StaticOriginId,
    checked_frame_id: u64,
    parameters: &[LoweringEnvironmentBinding],
    captures: &[LoweringEnvironmentBinding],
) -> Result<LoweringOperand, CraneliftBackendError> {
    let suffix = compiler.retained_body_occurrence(continuation_origin)?;
    let RuntimeExpr::ComputationalMatch { cases, default, .. } = suffix.expr else {
        return Err(unsupported(
            "StaticContinuationFusion",
            "the claimed consumer continuation is not a computational match, so the fused region \
             has no suffix to run",
        ));
    };
    let body = compiler.retained_body_occurrence(producer_body)?;
    compiler.lower_fused_producer_through_suffix(
        builder,
        body,
        continuation_origin,
        checked_frame_id,
        cases,
        default,
        parameters,
        captures,
    )
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` — point every fused region's
/// redirected invocation at the fused `Function`, in the caller being defined.**
///
/// **Selection is by the CLAIM's own redirect edge and by the unit being
/// defined — nothing here searches.** The edge was chosen once, by
/// `fusion_redirect_target`, from the complete key, and validated by preflight
/// against the un-narrowed population. Re-deriving it here would search an edge
/// population that this region's own installation already narrowed, and could
/// not find the very edge whose supersession narrowed it.
///
/// **The frame contract installed is the FUSED region's, and that is not
/// optional.** The record drives the caller's stack-slot layout and its operand
/// walk; keeping the producer's frame while calling the fused body would write
/// the caller's operands at the wrong offsets. The `origin` field stays the
/// original edge's callee entry: it is the transfer coordinate the source
/// invocation already named, and the operand run it names is unchanged.
///
/// **`record_redirected` is affine and fires here**, so a second unit that
/// somehow claimed the same region refuses instead of silently installing a
/// second redirect.
fn redirect_fused_producer_invocations<M: Module>(
    module: &mut M,
    func: &mut Function,
    compiler: &mut Lowering<'_>,
    bundle: &UnitBundle,
    caller: PredeclaredFunctionId,
    unit_calls: &mut BTreeMap<StaticOriginId, DeclaredUnitCall>,
) -> Result<(), CraneliftBackendError> {
    // Own every fact before the ledger is borrowed mutably below.
    let redirects = {
        let Some(ledger) = compiler.fusion_claims.as_ref() else {
            return Ok(());
        };
        if ledger.is_empty() {
            return Ok(());
        }
        let contracts = compiler
            .static_transition_plan
            .continuation_fusions()?
            .into_iter()
            .map(|fusion| {
                let (offsets, _frame_bytes) = fusion.slot_offsets()?;
                Ok((fusion.id(), fusion.header(), fusion.slots().to_vec(), offsets))
            })
            .collect::<Result<Vec<_>, CraneliftBackendError>>()?;
        let mut redirects = Vec::new();
        for fusion in ledger.planned() {
            let Some(claim) = ledger.claim(*fusion) else {
                // Already consumed at its takeover. A claim is consumed exactly
                // once, and the takeover cannot run before the body it sits in
                // is defined, so reaching this in the declaration half of a
                // definition means the region was taken over by another seat.
                continue;
            };
            if claim.consumer_owner() != caller {
                continue;
            }
            let target = bundle.fusion(*fusion).ok_or_else(|| {
                backend_module(
                    "a claimed fused region has no forward-declared target to redirect to"
                        .to_string(),
                )
            })?;
            let (_, header, slots, offsets) = contracts
                .iter()
                .find(|(id, ..)| id == fusion)
                .ok_or_else(|| {
                    backend_module(
                        "a claimed fused region has no installed frame contract".to_string(),
                    )
                })?;
            redirects.push((
                *fusion,
                claim.seat(),
                claim.redirect().callee_origin(),
                target,
                *header,
                slots.clone(),
                offsets.clone(),
            ));
        }
        redirects
    };
    for (fusion, seat, callee_origin, target, header, slots, offsets) in redirects {
        let call = DeclaredUnitCall {
            function: module.declare_func_in_func(target, func),
            origin: callee_origin,
            call_site_origin: seat,
            header,
            slots,
            offsets,
        };
        // The entry is INSERTED, and its absence beforehand is required rather
        // than assumed. Installing body ownership removed this seat's edge from
        // `executable_call_edges`, so an entry already standing here means the
        // producer is still reachable by its standalone route and the redirect
        // would be one of two answers to the same lookup.
        if unit_calls.insert(seat, call).is_some() {
            return Err(backend_module(
                "a fused region's redirected seat still holds a standalone producer call target, \
                 so the producer's body would remain reachable beside its fused definition"
                    .to_string(),
            ));
        }
        if let Some(ledger) = compiler.fusion_claims.as_mut() {
            ledger.record_redirected(fusion)?;
        }
    }
    Ok(())
}

pub(super) fn define_root_adapter<M: Module>(
    module: &mut M,
    compiler: &mut Lowering<'_>,
    helpers: ArtifactHelpers<'_>,
    bundle: &UnitBundle,
    adapter_id: FuncId,
    process_mode: bool,
    project_public_scalar_root: bool,
) -> Result<(), CraneliftBackendError> {
    compiler.open_aggregate_events(adapter_id)?;
    let root = compiler.static_transition_plan.root_emittable_unit()?;
    let root_id = bundle.function(root.function()).ok_or_else(|| {
        backend_module("the recorded root unit was never forward-declared".to_string())
    })?;
    let (offsets, frame_bytes) = root.slot_offsets()?;
    if frame_bytes != root.header().frame_bytes {
        return Err(backend_module(
            "root adapter target frame size disagrees with its slot run".to_string(),
        ));
    }
    if process_mode {
        for role in [
            AbiProcessParameter::ProcessInput,
            AbiProcessParameter::Capability,
        ] {
            compiler
                .static_transition_plan
                .process_parameter_slot(role)?
                .ok_or_else(|| {
                    backend_module("process root has no declared role-keyed ingress slot".to_string())
                })?;
        }
    }

    let sig = unit_signature(module);
    let mut func =
        Function::with_name_signature(UserFuncName::user(0, adapter_id.as_u32()), sig);
    let mut function_local = helpers.declare_in_func(
        module,
        &mut func,
        Some(TrapExitAuthority::Root {
            process_sentinel: process_mode,
            source_authorized: false,
        }),
    );
    let root_origin = root.body_occurrence();
    function_local.unit_calls.insert(
        root_origin,
        DeclaredUnitCall {
            function: module.declare_func_in_func(root_id, &mut func),
            origin: root_origin,
            // The body occurrence, NOT the scheduling entry. They coincide
            // for an ordinary root and deliberately do not when the root body
            // schedules something before itself; `call_site_origin` is matched
            // against the `body_origin` the unit actually lowers, so naming the
            // entry here would disagree exactly on that case.
            call_site_origin: root_origin,
            header: root.header(),
            slots: root.slots().to_vec(),
            offsets,
        },
    );

    let mut func_ctx = FunctionBuilderContext::new();
    {
        let mut builder = FunctionBuilder::new(&mut func, &mut func_ctx);
        let entry = builder.create_block();
        builder.append_block_params_for_function_params(entry);
        builder.switch_to_block(entry);
        let ingress = builder.block_params(entry)[0];
        let services = builder.block_params(entry)[1];
        let pointer_type = module.target_config().pointer_type();
        let native_int_arena = builder.ins().load(
            pointer_type,
            MemFlags::trusted(),
            services,
            crate::activation_services::SERVICES_NATIVE_INT_ARENA,
        );
        Lowering::require_nonzero(&mut builder, native_int_arena);
        let boundary_arena = builder.ins().load(
            pointer_type,
            MemFlags::trusted(),
            services,
            crate::activation_services::SERVICES_BOUNDARY_ARENA,
        );
        Lowering::require_nonzero(&mut builder, boundary_arena);
        function_local.services_pointer = Some(services);
        function_local.native_int_arena = Some(native_int_arena);
        function_local.boundary_arena = Some(boundary_arena);

        let mut inputs = Vec::new();
        if process_mode {
            let process_input = builder.ins().load(
                pointer_type,
                MemFlags::trusted(),
                ingress,
                crate::boundary_activation::ROOT_INGRESS_PROCESS_INPUT,
            );
            Lowering::require_nonzero(&mut builder, process_input);
            let host_dispatch_context = builder.ins().load(
                pointer_type,
                MemFlags::trusted(),
                ingress,
                crate::boundary_activation::ROOT_INGRESS_HOST_DISPATCH_CONTEXT,
            );
            Lowering::require_nonzero(&mut builder, host_dispatch_context);
            let capability = builder.ins().load(
                types::I64,
                MemFlags::trusted(),
                ingress,
                crate::boundary_activation::ROOT_INGRESS_CAPABILITY,
            );
            function_local.host_dispatch_context = Some(host_dispatch_context);
            inputs.push(LoweringOperand::Specialized(
                Lowered::BorrowedNativeValue {
                    pointer: process_input,
                },
            ));
            inputs.push(LoweringOperand::Specialized(Lowered::CapabilityToken {
                value: capability,
            }));
            #[cfg(test)]
            PROCESS_SLOT_MUTATION.with(|cell| match cell.get() {
                ProcessSlotMutation::Exact
                | ProcessSlotMutation::AttemptFixedContextOffsets
                | ProcessSlotMutation::ReintroduceLaunchIngress => {}
                ProcessSlotMutation::DeleteProcessInput => {
                    inputs.remove(0);
                }
                ProcessSlotMutation::DeleteCapability => {
                    inputs.pop();
                }
            });
        } else {
            function_local.host_dispatch_context =
                Some(builder.ins().iconst(pointer_type, 0));
        }

        compiler.function_local = function_local;
        let result = compiler.call_declared_unit(
            &mut builder,
            root_origin,
            &inputs,
            #[cfg(test)]
            Some(ingress),
        )?;
        let LoweringOperand::Carried(result) = result else {
            return Err(backend_module(
                "the internal root call did not return its result word".to_string(),
            ));
        };
        let public_result = if project_public_scalar_root {
            compiler.emit_public_carrier_scalar(&mut builder, result)?
        } else {
            result.word
        };
        builder.ins().return_(&[public_result]);
        builder.seal_all_blocks();
        builder.finalize();
    }
    verify_cranelift_function(&func, module.isa())?;
    compiler.commit_aggregate_events()?;
    #[cfg(test)]
    scale_b_record_functionized_root_adapter(&func);
    let mut ctx = module.make_context();
    std::mem::swap(&mut ctx.func, &mut func);
    module
        .define_function(adapter_id, &mut ctx)
        .map_err(|err| backend_module(err.to_string()))
}

/// **`D2` — define every declared unit against its declared activation frame.**
///
/// ⛔ **Every dynamic value crosses into a unit through the declared
/// `AbiFrameHeader` + `AbiSlot` layout, never through capture-by-construction.**
/// The frame pointer is the unit's sole parameter; each slot is addressed at the
/// offset `B2R`'s own walk assigns it.
///
/// **MEASURED:** each emitted body addresses its slots at the offsets
/// `abi::slot_offsets` assigns, and reads its result from the slot `B2R` marks
/// `AbiSlotKind::Result`.
/// **CLAIMED:** the emitted code obeys the declared frame layout.
/// **THE GAP:** ⚠ this establishes *layout* agreement only. It says nothing
/// about the **ownership modes** (`AC-12`) or about whether every transfer into
/// a slot is **representable** (`AC-11`) — those are separate obligations with
/// their own controls, and ⛔ a body that addresses the right offset while
/// violating an ownership mode satisfies everything asserted here.
pub(super) struct RootUnitResult {
    pub(super) decoder: Option<ResultDecoder>,
    pub(super) trap: Option<RuntimeTrap>,
}

/// **`RT-CONTSPEC-ACTIVATE` `D3` — the affine claim ledger over the exact
/// planned continuation-call tokens.**
///
/// Each projected causal identity is claimed **exactly once**, by the exact
/// producer unit the token itself names. This is affine on the *causal token*,
/// which is a different object from `RT-WORKER-BIND`'s worker binding -- that
/// one is deliberately NOT affine, and nothing here changes it.
///
/// ⛔ There is no `active_emission_owner`, no lowering-minted arm token, and no
/// second owner authority: the owner compared against is the token's own
/// immutable `producer_owner`, and the unit compared to it is the one
/// currently being defined.
///
/// ⚠ An affine rejection from here is **not self-explaining**. The identity is
/// four-field -- producer construct, alternative, call-site sequence, and
/// `recursive_position`. A key that lost `recursive_position` collides two
/// distinct tokens at one source position and this ledger will report a
/// double-consumption of the *right* token while the real defect is the key's
/// arity. Check the arity before believing the report.
pub(super) struct ContinuationClaimLedger {
    /// The RESOLVED target for each planned causal identity. Previously this
    /// kept only the keys and threw the `FuncId` away through `into_keys`,
    /// which is why no continuation target could ever be called: the join D1
    /// performed was discarded at the moment it became useful.
    resolved: BTreeMap<ContinuationCallIdentity, FuncId>,
    /// `None` until claimed; then the exact unit that claimed it, so owner
    /// agreement is a recorded fact rather than an inference.
    claims: BTreeMap<ContinuationCallIdentity, Option<ContinuationEmissionOwner>>,
    /// **`4b`** -- the PLANNED set, read straight off the plan's own causal call
    /// projection at open time and never derived from [`Self::resolved`].
    planned: BTreeSet<ContinuationCallIdentity>,
    /// **`4b`** -- every identity some generated function minted a `FuncRef`
    /// for, accumulated across all of them.
    declared: BTreeSet<ContinuationCallIdentity>,
    /// **`4b`** -- every identity a direct call was actually emitted for,
    /// accumulated across all generated functions after each one's CLIF has been
    /// checked.
    emitted: BTreeSet<ContinuationCallIdentity>,
    /// **`RT-CONTSRC-PRODUCER-LOCAL` `D8k`** -- every identity discharged by a
    /// VERIFIED composed source-continuation consumption, accumulated across
    /// every generated function after each one's CLIF has been checked.
    ///
    /// ⛔ Fed from `function_local.composed_discharges` and from nothing else.
    /// The direct instruction map is not a source of composed claims: its gate
    /// requires the recorded instruction to decode to `identity.target()`, and
    /// a composed instruction targets the raw worker, so an identity appearing
    /// in both would mean one of the two gates had been loosened.
    composed: BTreeSet<ContinuationCallIdentity>,
}

/// **`RT-CONTINUATION-EDGE-DISPOSITION` `D3` — the five mutations, one per
/// property the candidate/disposition layer promises.**
///
/// Each arms exactly one defect and must red for its OWN refusal. They are
/// deliberately separate variants rather than flags: a run arms at most one, so
/// a control cannot pass because some other mutation was still set.
///
/// **ONE declaration, not a `cfg(test)`/`cfg(not(test))` pair.** It was
/// authored as a pair, and the `cfg(not(test))` half carried only `None` while
/// the mutation *sites* in `core.rs` name all five variants unconditionally —
/// so the **production lib did not compile**, while the lib-**test** profile,
/// which is the only thing the seam checkpoint's `818/6/4` run built, compiled
/// perfectly. A production-only red no test profile can see.
///
/// ⇒ The variants are therefore declared once, for both profiles. What stays
/// `#[cfg(test)]` is the only thing that must: the **arming** — the
/// thread-local and its setter. In production `d3_mutation()` is a `const`
/// `None`, every site's comparison folds to `false`, and no mutation is
/// reachable by any caller.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D3Mutation {
    None,
    /// Withhold the static-worker binding the candidate authorizes.
    SuppressBindingInstallation,
    /// Settle `InlineNoCall` on bridge ENTRY rather than on a successful exit.
    MarkInlineBeforeBridgeCompletion,
    /// Drop the pending-composed half of the consumed test, so a candidate a
    /// composed call will claim is settled inline first.
    MarkInlineAfterComposedCall,
    /// Withhold the `DirectCall` settlement, leaving a candidate unsettled.
    OmitFinalDisposition,
    /// Settle one candidate twice.
    DoubleDisposition,
}

#[cfg(test)]
thread_local! {
    static D3_MUTATION: std::cell::Cell<D3Mutation> = const {
        std::cell::Cell::new(D3Mutation::None)
    };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn set_d3_mutation(mutation: D3Mutation) {
    D3_MUTATION.with(|cell| cell.set(mutation));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d3_mutation() -> D3Mutation {
    D3_MUTATION.with(std::cell::Cell::get)
}

/// **`RT-CONTINUATION-EDGE-DISPOSITION` `D3` — the seat a settlement was made
/// at.**
///
/// Two mutations (2 and 3) converge on the SAME terminal double-settlement
/// refusal, because they break the same invariant at different causal points.
/// The Architect's ruling is that a shared terminal string may corroborate both
/// rows but may not be either row's sole oracle. This enum is the
/// discriminator: it says **where** the offending settlement was made, which
/// the refusal text cannot.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D3Seat {
    /// The deferred bridge's ENTRY, before its scope has run.
    BridgeEntry,
    /// The deferred bridge's EXIT, after its scope completed.
    BridgeExit,
    /// The shared direct producer/call funnel.
    DirectFunnel,
    /// Finished-CLIF composed-discharge verification.
    ComposedPromotion,
}

/// **`RT-CONTINUATION-EDGE-DISPOSITION` `D3` — one ordered causal observation.**
///
/// Keyed by the live [`ContinuationCallIdentity`], so a row proves its
/// unmutated and armed arms reached the **same** seat for the **same** edge
/// rather than for two different ones that happen to look alike.
///
/// The two boolean halves on the bridge events are read **directly from the
/// ledger and the pending feed**, deliberately NOT through
/// `continuation_candidate_is_consumed`. That function is what mutation 3
/// mutates; routing the observation through it would make the instrument
/// inherit the defect it exists to detect, and the trace would agree with the
/// mutation instead of exposing it.
/// **`D3` — which environment binding an authorized position received.**
///
/// Mutation 1 substitutes one for the other, and the substitution is the whole
/// defect: the candidate still gets a binding, so a count sees nothing.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D3BindingKind {
    /// The capsule the candidate authorizes, callable at its recursive
    /// position.
    StaticWorker,
    /// A plain specialized value substituted in its place.
    Value,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum D3Event {
    /// The bridge entered with this candidate bypassed.
    BridgeEntry {
        identity: ContinuationCallIdentity,
        settled: bool,
        pending_composed: bool,
    },
    /// A composed claim was RECORDED during lowering, which is strictly before
    /// finished-CLIF verification promotes it.
    ComposedRecorded { identity: ContinuationCallIdentity },
    /// **The binding seat** — which KIND of environment binding a composed
    /// candidate's authorized position actually received.
    ///
    /// Keyed by the identity the target's own coordinate selects, so row 1 can
    /// bind its downstream oracle to the same edge whose binding was
    /// suppressed rather than to "some binding somewhere in the program".
    BindingInstalled {
        identity: ContinuationCallIdentity,
        kind: D3BindingKind,
    },
    /// The shared direct funnel was REACHED and RETURNED an answer.
    ///
    /// Deliberately a separate event from [`Self::Settle`], and the
    /// separation was forced by a row that could not be written without it:
    /// row 4 withholds the settlement while preserving the call, so it must
    /// assert "the funnel returned AND nothing was settled". One event
    /// standing for both facts makes that sentence unstateable — the first
    /// draft overloaded them and row 4 failed against its own instrument.
    DirectFunnelReturned { identity: ContinuationCallIdentity },
    /// The bridge scope finished, and what it and the two feeds then said.
    BridgeExit {
        identity: ContinuationCallIdentity,
        completed: bool,
        settled: bool,
        pending_composed: bool,
    },
    /// A settlement was ATTEMPTED at a named seat. Recorded before the ledger
    /// call, so a refused second settlement still leaves its seat in the trace.
    Settle {
        identity: ContinuationCallIdentity,
        disposition: CandidateDisposition,
        seat: D3Seat,
    },
}

#[cfg(test)]
impl D3Event {
    /// The live identity this observation is about.
    ///
    /// Clause 2 of the `AC-6` proof shape is that the unmutated and armed arms
    /// reach the mutation's seat **for the same derived identity**. Without an
    /// accessor a row can only compare event *kinds*, which two different
    /// edges would satisfy equally well.
    pub(in crate::cranelift_backend) fn identity(&self) -> &ContinuationCallIdentity {
        match self {
            Self::BridgeEntry { identity, .. }
            | Self::ComposedRecorded { identity }
            | Self::BindingInstalled { identity, .. }
            | Self::DirectFunnelReturned { identity }
            | Self::BridgeExit { identity, .. }
            | Self::Settle { identity, .. } => identity,
        }
    }

    /// The seat, when this observation is a settlement attempt.
    pub(in crate::cranelift_backend) fn settle_seat(&self) -> Option<D3Seat> {
        match self {
            Self::Settle { seat, .. } => Some(*seat),
            _ => None,
        }
    }
}

#[cfg(test)]
thread_local! {
    static D3_TRACE: std::cell::RefCell<Vec<D3Event>> =
        const { std::cell::RefCell::new(Vec::new()) };
    /// The candidate population the live plan projected at ledger open.
    static D3_PLAN_CANDIDATES: std::cell::RefCell<Vec<ContinuationCallIdentity>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

/// **The live plan's candidate population**, as `ContinuationCandidateLedger::
/// open` read it — typed identities, in plan order, not deduplicated by any
/// rendering.
#[cfg(test)]
pub(in crate::cranelift_backend) fn d3_plan_candidates() -> Vec<ContinuationCallIdentity> {
    D3_PLAN_CANDIDATES.with(|cell| cell.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d3_plan_candidates() {
    D3_PLAN_CANDIDATES.with(|cell| cell.borrow_mut().clear());
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d3_record(event: D3Event) {
    D3_TRACE.with(|cell| cell.borrow_mut().push(event));
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d3_trace() -> Vec<D3Event> {
    D3_TRACE.with(|cell| cell.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d3_trace() {
    D3_TRACE.with(|cell| cell.borrow_mut().clear());
}

#[cfg(not(test))]
pub(in crate::cranelift_backend) fn d3_mutation() -> D3Mutation {
    D3Mutation::None
}

/// **`RT-CONTINUATION-EDGE-DISPOSITION` `D1` — what lowering settled a binding
/// candidate to.**
///
/// The three are settled at three different seats, each of which knows
/// something the other two do not, and none of them is a discharge on its own:
/// a discharge is what [`ContinuationClaimLedger`] records, and this enum sits
/// in front of it.
///
/// `InlineNoCall` is deliberately **not** a third arm of the discharge
/// partition. A third arm would let a program that made no call satisfy a law
/// whose entire purpose is to say a call was answered. It never enters the
/// equality, and `D2` derives the call-obligation subset from the other two.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum CandidateDisposition {
    /// The verified direct producer/call seat claimed and emitted.
    DirectCall,
    /// A raw-worker call was emitted AND passed finished-CLIF verification.
    ComposedCall,
    /// The exact deferred bridge scope completed successfully with this
    /// candidate still unconsumed.
    InlineNoCall,
}

/// **`RT-CONTINUATION-EDGE-DISPOSITION` `D1` — the binding-candidate ledger, a
/// SIBLING of the claim ledger on the same artifact lifetime.**
///
/// A candidate is minted by the planner and carries the exact worker provenance
/// and selector already present in [`ContinuationCallIdentity`] — deliberately
/// no new key, because a second key would be a second authority over which edge
/// is which.
///
/// **A candidate authorizes environment installation. It does not assert a
/// call.** That separation is the whole node: the same planner edge was
/// carrying both roles, and suppressing it to remove the obligation also
/// removed the binding.
///
/// This does not widen [`ContinuationClaimLedger`], add a per-owner close, or
/// traverse failed or non-selected compilations. It opens and closes in the
/// same two wrappers, beside the aggregate-allocation and effect-seat ledgers,
/// which is the established one-artifact-one-ledger idiom.
///
/// **The ordered closeout is `D2` and it has landed:** exact
/// one-disposition/disjointness first, then the derived
/// `DirectCall ∪ ComposedCall` subset, then the unchanged exact equality and
/// claim equality over that subset. An `InlineNoCall` member is therefore not
/// a call obligation and no longer reaches the closeout as an undischarged
/// token.
pub(super) struct ContinuationCandidateLedger {
    /// Every candidate the planner minted for this artifact.
    candidates: BTreeSet<ContinuationCallIdentity>,
    /// The disposition each settled candidate took, and the seat is the only
    /// writer of each variant.
    settled: BTreeMap<ContinuationCallIdentity, CandidateDisposition>,
}

impl ContinuationCandidateLedger {
    fn open(plan: &StaticTransitionPlan<'_>) -> Result<Self, CraneliftBackendError> {
        // The SAME projection the claim ledger's `planned` is read from, so the
        // candidate population cannot drift from the population whose
        // obligations `D2` will derive out of it.
        // **`D3` — `O`, not `P`.** A fusion-local identity is settled in the
        // sibling composition ledger, never here, so admitting it as a candidate
        // would make totality-at-close demand a `CandidateDisposition` it must
        // not have. `evt_6kn9ckdnbf0ph` §2 forbids giving it one.
        let candidates = plan.ordinary_continuation_call_identities()?;
        // **`D3` — the candidate population AS THE LIVE PLAN PROJECTED IT.**
        //
        // Recorded here rather than rebuilt in a test, and the difference is
        // load-bearing: a test that re-plans its own witness proves a property
        // of the plan it just built, not of the plan the compile under
        // measurement actually used. Those can differ, and nothing would say
        // so. This is the projection `open` itself read.
        //
        // It is also a seat INDEPENDENT of every settlement seat the trace
        // observes, so an event carrying a wrong identity cannot match it by
        // construction -- the trace is not being validated against itself.
        #[cfg(test)]
        D3_PLAN_CANDIDATES.with(|cell| {
            *cell.borrow_mut() = candidates.iter().cloned().collect();
        });
        Ok(Self {
            candidates,
            settled: BTreeMap::new(),
        })
    }

    /// Settle one candidate, once.
    ///
    /// A second settlement is refused **here**, at the seat that makes it, and
    /// not deferred to closeout: the refusal names which two dispositions
    /// collided, which a set-difference at close cannot.
    pub(super) fn settle(
        &mut self,
        identity: &ContinuationCallIdentity,
        disposition: CandidateDisposition,
    ) -> Result<(), CraneliftBackendError> {
        if !self.candidates.contains(identity) {
            return Err(backend_module(format!(
                "lowering settled a disposition for an identity the planner never minted a                  binding candidate for, so the candidate population and the settling seats                  disagree about which edges exist: {disposition:?}"
            )));
        }
        if let Some(existing) = self.settled.insert(identity.clone(), disposition) {
            if existing != disposition {
                return Err(backend_module(format!(
                    "one binding candidate was settled twice, as {existing:?} and then as                      {disposition:?}; a candidate has exactly one disposition"
                )));
            }
            return Err(backend_module(format!(
                "one binding candidate was settled twice, both times as {disposition:?}; a                  candidate is settled exactly once"
            )));
        }
        Ok(())
    }

    pub(super) fn is_settled(&self, identity: &ContinuationCallIdentity) -> bool {
        self.settled.contains_key(identity)
    }

    /// **`RT-CONTINUATION-EDGE-DISPOSITION` `D2` — totality first, then the
    /// derived call-obligation subset. The ORDER is the mechanism.**
    ///
    /// Disjointness is already structural: [`Self::settle`] refuses a second
    /// settlement at the seat that makes it, so a candidate cannot hold two
    /// dispositions and reach here. What is checked here is **totality** — that
    /// every minted candidate was settled by some seat.
    ///
    /// **Totality is checked BEFORE the subset is derived, and deriving
    /// first would defeat the whole check**: an unsettled candidate is in
    /// neither `DirectCall` nor `ComposedCall`, so it would simply fall out of
    /// the subset and pass silently — which is the exact failure the existing
    /// closeout refuses and this layer exists to make impossible.
    ///
    /// The domain needs no filtering. A candidate ledger only exists inside the
    /// selected `FunctionizedUnits` arm and only reaches this call on the
    /// success path, so plan-only rows, `Err` compilations and non-selected
    /// `RecursiveDescent` plans are absent **by construction** rather than
    /// removed after the fact.
    fn close(self) -> Result<BTreeSet<ContinuationCallIdentity>, CraneliftBackendError> {
        let unsettled = self.candidates.difference(
            &self.settled.keys().cloned().collect::<BTreeSet<_>>(),
        ).count();
        if unsettled > 0 {
            return Err(backend_module(format!(
                "{unsettled} binding candidates reached the artifact closeout without a \
                 disposition; every candidate is settled exactly once by the seat that observed \
                 its event, so an unsettled candidate means a consumption path exists that no \
                 seat reports"
            )));
        }
        // The call-obligation subset. `InlineNoCall` is deliberately absent: it
        // is not a discharge and never enters the equality below.
        Ok(self
            .settled
            .into_iter()
            .filter(|(_, disposition)| {
                matches!(
                    disposition,
                    CandidateDisposition::DirectCall | CandidateDisposition::ComposedCall
                )
            })
            .map(|(identity, _)| identity)
            .collect())
    }

    #[cfg(test)]
    pub(in crate::cranelift_backend) fn dispositions(
        &self,
    ) -> &BTreeMap<ContinuationCallIdentity, CandidateDisposition> {
        &self.settled
    }

    #[cfg(test)]
    pub(in crate::cranelift_backend) fn candidate_count(&self) -> usize {
        self.candidates.len()
    }
}

impl ContinuationClaimLedger {
    pub(super) fn open(
        plan: &StaticTransitionPlan<'_>,
        bundle: &UnitBundle,
    ) -> Result<Self, CraneliftBackendError> {
        let resolved = resolve_continuation_targets(plan, bundle)?;
        let claims = resolved.keys().cloned().map(|identity| (identity, None)).collect();
        // The PLANNED set, taken from the plan's causal call projection rather
        // than from `resolved`. ⚠ Honest note: `resolve_continuation_targets`
        // walks the same projection, so planned == resolved is structural today
        // and would only separate if resolution ever dropped or added a key. It
        // is recorded because `close()` asserts the four sets equal and a set
        // that is *implied* by another is not the same evidence as one that was
        // read independently -- the load-bearing pairs are declared and emitted.
        // **`D3` — `O`.** `close()` asserts `resolved = declared = planned` over
        // this set, and all three narrow together: a fusion-local identity is
        // absent from every one of them, so each law stays literally true over
        // its own complete domain rather than being widened to accept a gap.
        let planned = plan.ordinary_continuation_call_identities()?;
        Ok(Self {
            resolved,
            claims,
            planned,
            declared: BTreeSet::new(),
            emitted: BTreeSet::new(),
            composed: BTreeSet::new(),
        })
    }

    /// **`4b`** -- record the causal tokens one generated function minted call
    /// refs for.
    pub(super) fn record_declared(
        &mut self,
        declared: impl IntoIterator<Item = ContinuationCallIdentity>,
    ) -> Result<(), CraneliftBackendError> {
        for identity in declared {
            if !self.declared.insert(identity) {
                return Err(backend_module(
                    "a causal token was declared into more than one generated function".to_string(),
                ));
            }
        }
        Ok(())
    }

    /// **`4b`** -- record the causal tokens one generated function actually
    /// emitted a verified direct call for.
    ///
    /// ⛔ Called only *after* that function's CLIF has been checked, so a token
    /// reaches this set only once its emitted callee has been decoded from the
    /// instruction stream and matched against the planner-issued target.
    pub(super) fn record_emitted(
        &mut self,
        emitted: impl IntoIterator<Item = ContinuationCallIdentity>,
    ) -> Result<(), CraneliftBackendError> {
        for identity in emitted {
            if !self.emitted.insert(identity) {
                return Err(backend_module(
                    "a causal token emitted a direct call from more than one generated function"
                        .to_string(),
                ));
            }
        }
        Ok(())
    }

    /// **`D8k`** -- record the causal tokens one generated function discharged
    /// through a VERIFIED composed source-continuation consumption, and claim
    /// them.
    ///
    /// ⛔ Called only after `verify_recorded_composed_discharges` has promoted
    /// them, so an identity reaches this set only once its recorded raw-worker
    /// call was found in the finished CLIF, its decoded callee matched the
    /// `D8b`/`D8d` target, its operand run matched that target's declared run,
    /// and its result was shown to return downstream into the unchanged
    /// continuation.
    ///
    /// ⭐⭐ **The claim is made HERE and not at the seat, and that is what makes
    /// the partition disjoint.** A composed consumption claims the same
    /// `claims` slot a direct emission would, so an identity claimed both ways
    /// is rejected as a double claim rather than silently satisfying both
    /// halves of the union.
    pub(super) fn record_composed(
        &mut self,
        discharged: impl IntoIterator<Item = ContinuationCallIdentity>,
        defining: ContinuationEmissionOwner,
    ) -> Result<(), CraneliftBackendError> {
        for identity in discharged {
            if identity.emission_owner() != defining {
                return Err(backend_module(
                    "a composed source continuation was discharged by a function that is not its \
                     emission owner"
                        .to_string(),
                ));
            }
            let consumed = self.claims.get_mut(&identity).ok_or_else(|| {
                backend_module(
                    "a composed source continuation discharged a causal token this ledger never \
                     planned"
                        .to_string(),
                )
            })?;
            if let Some(previous) = consumed {
                return Err(backend_module(format!(
                    "a causal token was claimed twice, first by {previous:?} and then by a \
                     composed source continuation; one causal obligation is discharged by one \
                     form, never by both"
                )));
            }
            *consumed = Some(defining);
            if !self.composed.insert(identity) {
                return Err(backend_module(
                    "a causal token was discharged by a composed source continuation in more \
                     than one generated function"
                        .to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Declare THIS owning Function's own `FuncRef` for every causal token it
    /// owns, keyed by the complete four-field identity.
    ///
    /// Each `FuncRef` is minted into the `Function` passed here and belongs to
    /// it alone; ⛔ none is ever passed across functions. Declaring is not
    /// claiming -- a declared target is callable, and the affine claim happens
    /// later, at the exact producer occurrence.
    pub(super) fn declare_owned_in_func<M: Module>(
        &self,
        defining: ContinuationEmissionOwner,
        module: &mut M,
        func: &mut Function,
        plan: &StaticTransitionPlan<'_>,
    ) -> Result<BTreeMap<ContinuationCallIdentity, DeclaredUnitCall>, CraneliftBackendError> {
        // The declared target is the FULL contract, not a bare `FuncRef`:
        // `DeclaredUnitCall` already carries the function, the target origin,
        // and the projected header/slots/offsets, which is exactly what
        // `call_declared_unit_target` consumes. Reusing it is what keeps this
        // on the existing unit-call ABI instead of inventing a second one.
        let units = plan.continuation_units()?;
        self.resolved
            .iter()
            .filter(|(identity, _)| identity.emission_owner() == defining)
            .map(|(identity, target)| {
                let unit = units
                    .iter()
                    .find(|unit| unit.id() == identity.target())
                    .ok_or_else(|| {
                        backend_module(
                            "a resolved causal identity names a specialization with no projected \
                             unit"
                                .to_string(),
                        )
                    })?;
                let (offsets, _frame_bytes) = unit.slot_offsets()?;
                Ok((
                    identity.clone(),
                    DeclaredUnitCall {
                        function: module.declare_func_in_func(*target, func),
                        origin: unit.continuation_origin(),
                        call_site_origin: unit.continuation_origin(),
                        header: unit.header(),
                        slots: unit.slots().to_vec(),
                        offsets,
                    },
                ))
            })
            .collect()
    }

    /// Claim ONE exact token, at its producer occurrence.
    ///
    /// ⛔ This replaces a bulk discharge that consumed every token owned by a
    /// unit before any producer occurrence was reached. That was affine
    /// bookkeeping at the wrong seat: it could not tell a call that happened
    /// from one that did not, so it would have reported a clean ledger for a
    /// program that emitted no continuation call at all.
    ///
    /// Rejects an **absent** token, a **duplicate** claim, and a **wrong
    /// owner** -- and unlike the previous shape the owner check is reachable
    /// here, because the caller supplies the unit currently being defined
    /// rather than the token's own owner being used to select it.
    ///
    /// ⚠ An affine red is not self-explaining: confirm the identity still
    /// carries all four fields including `recursive_position` before believing
    /// a double-consumption, because a collided key reports it against the
    /// right token.
    pub(super) fn claim_exact(
        &mut self,
        identity: &ContinuationCallIdentity,
        defining: ContinuationEmissionOwner,
    ) -> Result<FuncId, CraneliftBackendError> {
        if identity.emission_owner() != defining {
            return Err(backend_module(
                "a continuation call token was claimed by a context that is not its emission \
                 owner; note this compares the EMISSION owner, not the raw source-occurrence \
                 provenance owner beside it"
                    .to_string(),
            ));
        }
        let consumed = self.claims.get_mut(identity).ok_or_else(|| {
            backend_module(
                "a continuation call token was claimed that this ledger never planned".to_string(),
            )
        })?;
        if let Some(previous) = consumed {
            return Err(backend_module(format!(
                "a continuation call token was claimed twice, first by {previous:?}; before \
                 reading this as a real double-consumption, confirm the causal identity still \
                 carries all four fields including recursive_position, because a collided key \
                 reports this against the right token"
            )));
        }
        *consumed = Some(defining);
        self.resolved.get(identity).copied().ok_or_else(|| {
            backend_module("a claimed causal token has no resolved target".to_string())
        })
    }

    /// The closeout, over **two** populations since `D2`.
    ///
    /// `resolved` and `declared` are checked against the **full planned**
    /// population, because declaration is bulk over planned by `D8k`'s design.
    /// `discharged` and `claimed` are checked against **`call_obligations`**,
    /// the `DirectCall ∪ ComposedCall` subset the candidate ledger derives —
    /// so an `InlineNoCall` member is deliberately unclaimed and undischarged
    /// and is not an error, while an `InlineNoCall` member that WAS claimed
    /// still is.
    ///
    /// **Neither discharge set is bookkeeping.** An identity enters `emitted`
    /// only after its emitted callee was decoded from the finished CLIF and
    /// matched the planner-issued target, and enters `composed` only after a
    /// raw-worker call passed every clause of the finished-CLIF verification.
    ///
    /// **There is no equality against `emitted` alone.** The equality is over
    /// their **union** against `call_obligations`, and that is not a detail: a
    /// lawful `ComposedCall` obligation is answered by verified composed
    /// consumption and never becomes a direct call, so an "every obligation
    /// became one direct call" reading would exclude a legal member of the very
    /// representation this documents. What the union equality says is "every
    /// call obligation was answered exactly once, in exactly one of the two
    /// forms, and nothing that was not an obligation was answered at all."
    ///
    /// ⛔ Equality is asserted between sets, not between counts. Two sets of the
    /// same size can differ, and a length comparison here would pass for a
    /// population that swapped one token for another.
    /// `D2` — `call_obligations` is the derived `DirectCall ∪ ComposedCall`
    /// subset. **The discharge equality's FORM is unchanged**; only the set it
    /// ranges over is the derived subset rather than the full planned
    /// population, which is what lets a candidate that made no call stop being
    /// an obligation without any arm being added to the partition.
    ///
    /// **`resolved` and `declared` still range over the FULL planned set.**
    /// Declaration is bulk over planned by `D8k`'s own design, so narrowing
    /// those two would refuse every artifact with an `InlineNoCall` candidate
    /// for the opposite reason.
    pub(super) fn close(
        self,
        call_obligations: &BTreeSet<ContinuationCallIdentity>,
    ) -> Result<(), CraneliftBackendError> {
        // `D8k` -- DECLARATION may remain over the full planned set. An unused
        // declaration is a `FuncRef` nobody called, not an emitted call, so the
        // declared population stays equal to planned even where the discharge
        // took the composed form.
        for (name, set) in [
            ("resolved", self.resolved.keys().cloned().collect::<BTreeSet<_>>()),
            ("declared", self.declared.clone()),
        ] {
            if set != self.planned {
                let missing = self.planned.difference(&set).count();
                let extra = set.difference(&self.planned).count();
                return Err(backend_module(format!(
                    "the {name} continuation call population does not equal the planned one: \
                     {missing} planned tokens absent, {extra} unplanned tokens present"
                )));
            }
        }
        // **`D8k` -- THE PARTITION**, since `D2` over the derived subset:
        // `call obligations = direct-emitted ⊎ composed-consumed`, asserted as
        // a disjoint union of two sets that
        // were accumulated from two different kinds of evidence: decoded direct
        // specialization emissions, and verified composed source-continuation
        // consumptions.
        //
        // ⛔ Not weakened to a count. Two sets of the right total size can still
        // be the wrong sets, and a program that emitted one token directly and
        // consumed a different one compositionally would satisfy any arithmetic
        // statement of this law.
        //
        // ⛔ Disjointness is asserted separately from coverage, because they
        // fail for different reasons: an overlap means one obligation was
        // answered twice, in two forms; a shortfall means one was never
        // answered at all.
        let both = self
            .emitted
            .intersection(&self.composed)
            .cloned()
            .collect::<BTreeSet<_>>();
        if !both.is_empty() {
            return Err(backend_module(format!(
                "{} causal tokens were discharged BOTH by a decoded direct emission and by a \
                 verified composed consumption; the two forms partition the call-obligation \
                 population and an identity in both means one obligation was answered twice",
                both.len()
            )));
        }
        let discharged = self
            .emitted
            .union(&self.composed)
            .cloned()
            .collect::<BTreeSet<_>>();
        // `D2` — the SAME equality, over the derived call-obligation subset.
        // `InlineNoCall` candidates are not in `call_obligations`, so they are
        // simply not obligations; nothing here treats them as discharged.
        if discharged != *call_obligations {
            let missing = call_obligations.difference(&discharged).count();
            let extra = discharged.difference(call_obligations).count();
            return Err(backend_module(format!(
                "the discharged continuation call population is not the call-obligation one: \
                 {missing} call obligations were neither directly emitted nor compositionally \
                 consumed, and {extra} discharged tokens are not call obligations. Direct: {}, \
                 composed: {}",
                self.emitted.len(),
                self.composed.len()
            )));
        }
        // `D2` — the claim law, as EXACT SET EQUALITY over the same derived
        // subset, not as a count of unclaimed slots.
        //
        // A count would have to be written as "ignore every unclaimed identity
        // outside the subset", and that hides the dual error: an `InlineNoCall`
        // candidate that was **accidentally claimed**. Equality catches both
        // directions and reports them separately, because they are opposite
        // defects — a missing obligation is a call nobody answered, an extra
        // claim is an inline non-call that answered for a call it never made.
        let claimed = self
            .claims
            .iter()
            .filter(|(_, consumed)| consumed.is_some())
            .map(|(identity, _)| identity.clone())
            .collect::<BTreeSet<_>>();
        if claimed != *call_obligations {
            let missing = call_obligations.difference(&claimed).count();
            let extra = claimed.difference(call_obligations).count();
            return Err(backend_module(format!(
                "the claimed continuation call population is not the call-obligation one: \
                 {missing} call obligations were never claimed by the unit that owns them, and \
                 {extra} claims were made for identities that are not call obligations"
            )));
        }
        // Owner agreement, asserted on the RECORDED consumer, over the
        // claimed/call-obligation members.
        //
        // Honest note: selection above is already by the token's own owner, so
        // this holds by construction today and cannot fire against the current
        // code. It is kept because it is the property `D3` actually promises,
        // and it is the check that would fire if selection were ever decoupled
        // from ownership.
        for (identity, consumed) in self
            .claims
            .iter()
            .filter(|(identity, _)| call_obligations.contains(*identity))
        {
            if *consumed != Some(identity.emission_owner()) {
                return Err(backend_module(
                    "a continuation call token was claimed by a unit that does not own it"
                        .to_string(),
                ));
            }
        }
        Ok(())
    }

    /// **`D3` — every identity this ordinary ledger has TOUCHED, in any role.**
    ///
    /// The union of claimed, declared, direct-emitted and composed-discharged.
    /// It exists for exactly one consumer: the fusion-local ledger's closeout,
    /// which asserts its own consumed population is disjoint from this one.
    ///
    /// ⛔ **A union, not `planned`.** Asserting disjointness against `planned`
    /// would test the partition the planner already validated -- `O ∩ F = ∅` is
    /// checked at preflight and would be re-checked here against the same
    /// derivation. What has to be disjoint is what the two ledgers ACTUALLY
    /// recorded: a fusion-local identity that reached any ordinary role is a
    /// direct-plus-composed double realization, and it is that event, not the
    /// planner's arithmetic, that this makes visible.
    pub(super) fn touched_identities(&self) -> BTreeSet<ContinuationCallIdentity> {
        self.claims
            .iter()
            .filter(|(_, consumed)| consumed.is_some())
            .map(|(identity, _)| identity.clone())
            .chain(self.declared.iter().cloned())
            .chain(self.emitted.iter().cloned())
            .chain(self.composed.iter().cloned())
            .collect()
    }
}

/// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the SIBLING affine ledger for the
/// fusion-local realizations `F`.**
///
/// ⭐⭐ **A sibling, not a widening, and Architect `evt_6kn9ckdnbf0ph` is
/// explicit about why.** An ordinary identity's obligation is discharged by an
/// emitted call whose callee is decoded back out of the finished CLIF; a
/// fusion-local identity emits no call at all, so there is no instruction for
/// any such gate to read. Putting both in one ledger would mean either
/// weakening the direct laws to tolerate a member with no instruction, or
/// giving the composed member an instruction it does not have. Two ledgers over
/// two disjoint domains keeps each law literally true over its own complete
/// domain -- which is the same reason `O` was narrowed at the input rather than
/// the laws being relaxed.
///
/// ⛔ **This is NOT the `FusionRegionClaimLedger` and does not touch it.** The
/// region claim carries the producer/consumer relation and is spent at the
/// redirect and takeover seats; consuming a composition here does not spend it,
/// and call `17/13` stays on the region claim and out of `dom(FusionComposedEdge)`
/// entirely.
///
/// ## The affine law, and what each half refuses
///
/// `dom(planned) = dom(consumed)`, with the target range equal to `F_t`:
///
/// - an identity with **no planned edge** is refused -- a composition seat was
///   reached for something the planner never composed;
/// - a **second** consumption of one identity is refused, whether in the same
///   function or another one, which is the replay case;
/// - a consumption whose **owner, layer or target** disagrees with the planned
///   edge is refused, each named separately so the refusal says which fact
///   moved;
/// - a planned member **never consumed** is refused at close -- the case that
///   reads as success because nothing happened;
/// - a consumed identity that **also reached the ordinary ledger** in any role
///   is refused, which is the direct-plus-composed double realization.
pub(super) struct FusionCompositionLedger {
    /// `F` — the planned fusion-local realizations, keyed by exact identity.
    ///
    /// ⛔ The WHOLE composed edge is retained, not the fields a consumption
    /// happens to check. A consumption is validated against the planner's own
    /// record; re-deriving any of owner, layer or target here would make this
    /// ledger a second authority over the composition it is supposed to audit.
    planned: BTreeMap<ContinuationCallIdentity, FusionComposedEdge>,
    /// `None` until consumed; then the emission owner that consumed it.
    consumed: BTreeMap<ContinuationCallIdentity, Option<ContinuationEmissionOwner>>,
    /// The specialization targets the DECLARATION pass actually omitted, read
    /// from that pass's own output rather than re-derived.
    ///
    /// ⛔ Not derived from `planned` at close. Deriving it would compare the
    /// planner's `F_t` with itself and pass for a pass that omitted nothing --
    /// which is precisely the failure this range equality exists to catch.
    declaration_omitted: BTreeSet<ContinuationSpecializationId>,
    /// The specialization targets the DEFINITION pass actually omitted.
    ///
    /// ⛔ **Kept separate from the declaration set, and both are closed against
    /// `F_t`.** The two passes fail differently: a declaration that does not
    /// omit leaves an undefined phantom symbol, and a definition that does not
    /// omit emits a standalone `Function` for a body already lowered locally --
    /// a second realization of one edge. One merged set would be satisfied by
    /// either pass alone.
    definition_omitted: BTreeSet<ContinuationSpecializationId>,
}

impl FusionCompositionLedger {
    /// Open over the planner's composed-edge relation, once per artifact.
    ///
    /// The declaration omission is seeded from the BUNDLE -- the declaration
    /// pass's own output -- rather than from the plan. ⛔ That is the whole
    /// point: a `declare_unit_bundle` that stopped filtering would leave this
    /// set empty and the closeout would say so, where a plan-derived set would
    /// agree with the plan no matter what was declared.
    pub(super) fn open(
        plan: &StaticTransitionPlan<'_>,
        bundle: &UnitBundle,
    ) -> Result<Self, CraneliftBackendError> {
        let planned = plan.fusion_composed_edges().clone();
        let consumed = planned.keys().cloned().map(|identity| (identity, None)).collect();
        let declaration_omitted = plan
            .continuation_units()?
            .iter()
            .map(|unit| unit.id())
            .filter(|id| bundle.continuation(*id).is_none())
            .collect();
        Ok(Self {
            planned,
            consumed,
            declaration_omitted,
            definition_omitted: BTreeSet::new(),
        })
    }

    /// Record that the DEFINITION pass omitted this specialization's body.
    ///
    /// ⛔ Recorded where the omission happens, in the loop that would otherwise
    /// have emitted the `Function`, so the evidence is the pass's own decision
    /// rather than a statement about it made elsewhere.
    pub(super) fn record_definition_omitted(&mut self, target: ContinuationSpecializationId) {
        self.definition_omitted.insert(target);
    }

    /// Consume ONE fusion-local composition, at its exact call edge.
    ///
    /// The four facts are checked against the planned edge's own record, each
    /// with its own refusal, so a red names the fact that moved rather than
    /// reporting a generic mismatch.
    pub(super) fn consume(
        &mut self,
        identity: &ContinuationCallIdentity,
        defining: ContinuationEmissionOwner,
        layer: FusionCompositionLayer,
        target: ContinuationSpecializationId,
    ) -> Result<StaticContinuationFusionId, CraneliftBackendError> {
        let edge = self.planned.get(identity).ok_or_else(|| {
            backend_module(
                "a fusion-local composition was consumed for an identity the planner never \
                 composed; a composition seat reached for an unplanned identity is a lowering \
                 that decided for itself which edges are local"
                    .to_string(),
            )
        })?;
        if edge.emission_owner() != defining {
            return Err(backend_module(format!(
                "a fusion-local composition planned for emission owner {:?} was consumed while \
                 defining {defining:?}; the composition is lowered into the consumer's own \
                 function and a foreign one would place the producer's body in a frame that \
                 never held its operands",
                edge.emission_owner()
            )));
        }
        if edge.layer() != layer {
            return Err(backend_module(format!(
                "a fusion-local composition planned at layer {:?} was consumed as {layer:?}; the \
                 two ruled layers are selected by different checked bindings of the fusion key \
                 and are not substitutable",
                edge.layer()
            )));
        }
        if edge.target() != target {
            return Err(backend_module(format!(
                "a fusion-local composition planned for target {:?} was consumed against {target:?}; \
                 the composed edge names the exact specialization whose selected body is lowered \
                 locally, and another one is another body",
                edge.target()
            )));
        }
        let fusion = edge.fusion();
        let consumed = self.consumed.get_mut(identity).ok_or_else(|| {
            backend_module(
                "a fusion-local composition has a planned edge but no consumption slot; the two \
                 maps are built from one population and disagreeing means one was rebuilt"
                    .to_string(),
            )
        })?;
        if let Some(previous) = consumed {
            return Err(backend_module(format!(
                "a fusion-local composition was consumed twice, first by {previous:?}; one \
                 composed call edge is realized once, and a second consumption is a replay \
                 whether it happens in this function or another"
            )));
        }
        *consumed = Some(defining);
        Ok(fusion)
    }

    /// The closeout, over three populations.
    ///
    /// ⛔ Every comparison is between SETS. Two populations of the same size can
    /// be the wrong two, and a count would pass for a pass that consumed one
    /// composition and omitted a different target.
    pub(super) fn close(
        self,
        ordinary_touched: &BTreeSet<ContinuationCallIdentity>,
    ) -> Result<(), CraneliftBackendError> {
        let planned = self.planned.keys().cloned().collect::<BTreeSet<_>>();
        let consumed = self
            .consumed
            .iter()
            .filter(|(_, consumed)| consumed.is_some())
            .map(|(identity, _)| identity.clone())
            .collect::<BTreeSet<_>>();
        if consumed != planned {
            let missing = planned.difference(&consumed).count();
            let extra = consumed.difference(&planned).count();
            return Err(backend_module(format!(
                "the consumed fusion-local composition population is not the planned one: \
                 {missing} planned compositions were never realized, and {extra} realizations \
                 name identities that were never planned. An unrealized composition is the case \
                 that reads as success because nothing was emitted for it"
            )));
        }
        // `F_t` — the target range, against what the passes actually omitted.
        let planned_targets = self
            .planned
            .values()
            .map(FusionComposedEdge::target)
            .collect::<BTreeSet<_>>();
        for (pass, omitted) in [
            ("declaration", &self.declaration_omitted),
            ("definition", &self.definition_omitted),
        ] {
            if *omitted != planned_targets {
                let missing = planned_targets.difference(omitted).count();
                let extra = omitted.difference(&planned_targets).count();
                return Err(backend_module(format!(
                    "the {pass} pass's omitted continuation target population is not the \
                     fusion-local range F_t: {missing} fusion-local targets were still {pass}ed, \
                     and {extra} ordinary targets were omitted. A fusion-local target that keeps \
                     its standalone Function is a second realization of one edge"
                )));
            }
        }
        // THE DISJOINTNESS, against what the ordinary ledger RECORDED.
        //
        // ⛔ Not against `planned`, which the planner's own partition already
        // settles. What this catches is an identity that was composed locally
        // AND reached an ordinary role -- claimed, declared, directly emitted or
        // composed-discharged -- which is one call edge realized twice.
        let both = consumed.intersection(ordinary_touched).count();
        if both > 0 {
            return Err(backend_module(format!(
                "{both} continuation call identities were realized both as a fusion-local \
                 composition and in an ordinary role; one call edge is realized exactly once, \
                 and an identity in both ledgers has been emitted twice under one obligation"
            )));
        }
        Ok(())
    }
}

/// **`RT-DECL-CLOSURE-PORT` `D5a` checkpoint 2 — open the ONE cross-pass causal
/// ledger.**
///
/// Called from the orchestration before the first generated `Function`, so
/// every pass that can declare, claim or emit a causal call accumulates into
/// this single object.
///
/// ⛔ There is exactly one of these and it must not be re-opened. A second open
/// would discard whatever the first had accumulated and leave a partial
/// equality reading as a global one, which is the failure mode checkpoint 2
/// exists to remove — so re-opening rejects rather than replacing.
/// **`D1`** — the disposition tally from the most recent artifact close, so a
/// witness can assert WHICH disposition its candidate settled to without the
/// ledger outliving the artifact it belongs to.
#[cfg(test)]
thread_local! {
    static D1_LAST_DISPOSITIONS: std::cell::RefCell<
        std::collections::BTreeMap<CandidateDisposition, usize>,
    > = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn d1_last_dispositions()
-> std::collections::BTreeMap<CandidateDisposition, usize> {
    D1_LAST_DISPOSITIONS.with(|cell| cell.borrow().clone())
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_d1_dispositions() {
    D1_LAST_DISPOSITIONS.with(|cell| cell.borrow_mut().clear());
}

pub(super) fn open_continuation_claim_ledger(
    compiler: &mut Lowering<'_>,
    bundle: &UnitBundle,
) -> Result<(), CraneliftBackendError> {
    if compiler.continuation_claims.is_some() {
        return Err(backend_module(
            "the continuation claim ledger is already open; one artifact has exactly one ledger              and re-opening would silently discard every token recorded so far"
                .to_string(),
        ));
    }
    compiler.continuation_claims = Some(ContinuationClaimLedger::open(
        &compiler.static_transition_plan,
        bundle,
    )?);
    // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the fusion-local sibling opens on
    // the SAME boundary and for the same reason: one artifact has exactly one
    // of it, and every composition seat consumes into it. Sharing the lifetime
    // is what makes the two ledgers siblings over disjoint domains rather than
    // one ledger with a second kind of member.
    compiler.fusion_compositions = Some(FusionCompositionLedger::open(
        &compiler.static_transition_plan,
        bundle,
    )?);
    // `RT-CONTINUATION-EDGE-DISPOSITION` `D1` — the candidate ledger opens on
    // the SAME boundary and for the same reason the two below do: one artifact
    // has exactly one of it, and every pass settles into it. Sharing the
    // lifetime is what makes it a sibling in front of the claim ledger rather
    // than a widening of it.
    compiler.continuation_candidates =
        Some(ContinuationCandidateLedger::open(&compiler.static_transition_plan)?);
    // `D7` — the aggregate allocation relation opens on the same boundary and
    // for the same reason: one artifact has exactly one relation, and every
    // body's events commit into it.
    compiler.aggregate_allocations = Some(AggregateAllocationLedger::default());
    // `D7` — the host-effect seat ledger opens on the same boundary: one
    // artifact has one consumed-seat evidence, and every body claims into it.
    compiler.host_effect_seats = Some(EffectSeatLedger::default());
    Ok(())
}

/// **`D5a` checkpoint 2 — close it, once, after every generated `Function`.**
///
/// Since `D2`, two global laws rather than one: `resolved = declared =
/// planned` over the full planner population, and `discharged = claimed =
/// call_obligations` over the derived `DirectCall ∪ ComposedCall` subset.
///
/// **Not a per-pass partial:** a pass that discharges nothing is normal, and
/// only the whole-artifact sets answer whether **every call obligation** was
/// answered exactly once. Deliberately not "every planned token": an
/// `InlineNoCall` candidate is planned and is lawfully never discharged, which
/// is the whole point of the derived subset.
pub(super) fn close_continuation_claim_ledger(
    compiler: &mut Lowering<'_>,
) -> Result<(), CraneliftBackendError> {
    // The candidate ledger is taken on the same boundary as the claim ledger,
    // and `D2` closes it FIRST: totality, then the derived subset, then the
    // claim ledger's exact laws over that subset.
    let candidates = compiler.continuation_candidates.take().ok_or_else(|| {
        backend_module("the continuation candidate ledger went missing".to_string())
    })?;
    #[cfg(test)]
    D1_LAST_DISPOSITIONS.with(|cell| {
        *cell.borrow_mut() = candidates
            .dispositions()
            .values()
            .copied()
            .fold(std::collections::BTreeMap::new(), |mut acc, d| {
                *acc.entry(d).or_insert(0usize) += 1;
                acc
            });
    });
    // `D2` — the order, and it is the mechanism rather than a style choice.
    // Totality and disjointness FIRST, then the derived subset, then the
    // unchanged equality over it.
    let call_obligations = candidates.close()?;
    // `RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the fusion-local sibling closes
    // BEFORE the ordinary ledger is consumed, because its disjointness clause
    // reads what that ledger RECORDED and `close` takes it by value.
    //
    // ⛔ The order also decides which red a reader sees first, and this is the
    // useful one: a fusion-local composition that was never realized, or one
    // realized twice, explains an ordinary population that then looks short. The
    // reverse order reports the symptom and consumes the evidence.
    let ordinary_touched = compiler
        .continuation_claims
        .as_ref()
        .ok_or_else(|| backend_module("the continuation claim ledger went missing".to_string()))?
        .touched_identities();
    compiler
        .fusion_compositions
        .take()
        .ok_or_else(|| {
            backend_module("the fusion-local composition ledger went missing".to_string())
        })?
        .close(&ordinary_touched)?;
    compiler
        .continuation_claims
        .take()
        .ok_or_else(|| backend_module("the continuation claim ledger went missing".to_string()))?
        .close(&call_obligations)
}

/// **`D7` — close the aggregate allocation relation once, over the whole
/// compilation.**
///
/// ⛔ Not a per-body partial. One planner record may govern events in several
/// bodies -- a synthesized role at a seat reached under both a predeclared unit
/// and a generated specialization allocates in both -- so `image(R_f) = P` is
/// false for every individual body and imposing it would refuse lawful
/// programs. Only the whole-artifact relation answers whether every planned
/// record was allocated and every event was planned.
pub(super) fn close_aggregate_allocation_ledger(
    compiler: &mut Lowering<'_>,
) -> Result<AggregateRelationClosure, CraneliftBackendError> {
    let planned = compiler.static_transition_plan.aggregate_ownership_records();
    compiler
        .aggregate_allocations
        .take()
        .ok_or_else(|| {
            backend_module("the aggregate allocation ledger went missing".to_string())
        })?
        .close(planned)
}

/// **`D7` — close the host-effect seat authority once, over the whole
/// compilation.**
///
/// ⛔ Whole-artifact rather than per-body, because a seat inside a worker body
/// is consumed in its predeclared unit and again in each specialization that
/// contains it; no single body's claims equal the population.
///
/// ⭐ **`image(claims) ⊆ P`, exactly as the aggregate relation states it — NOT
/// an equality.** This comment previously claimed the opposite, on the reasoning
/// that a seat population derived from the source's own `Effect` occurrences
/// must be fully reached. That was measured false: an occurrence sitting in a
/// declaration body a compilation never emits takes its seats with it, and
/// requiring equality refused such a program. `P` authorizes; it does not
/// oblige, and an unreached member is lawful and reported. A half-read
/// occurrence cannot hide in that gap, because completeness is a group-local
/// equality that has already run at each visit's close.
pub(super) fn close_host_effect_seat_ledger(
    compiler: &mut Lowering<'_>,
) -> Result<EffectSeatClosure, CraneliftBackendError> {
    let planned = compiler
        .static_transition_plan
        .host_effect_seat_records()
        .to_vec();
    #[cfg(test)]
    if effect_seat_visit_mutation()
        == EffectSeatVisitMutation::DropCommittedGroupBeforeGlobalClose
    {
        if let Some(ledger) = compiler.host_effect_seats.as_mut() {
            ledger.drop_one_committed_group_for_tests();
        }
    }
    let closure = compiler
        .host_effect_seats
        .take()
        .ok_or_else(|| backend_module("the host effect seat ledger went missing".to_string()))?
        .close(&planned)?;
    #[cfg(test)]
    LAST_EFFECT_SEAT_CLOSURE.with(|cell| *cell.borrow_mut() = Some(closure.clone()));
    Ok(closure)
}

/// What the last completed seat closeout on this thread measured.
///
/// ⚠ Like `b2f_last_unit_emission`, this carries no statement about WHICH
/// compile produced it: a compile that fails before the closeout leaves the
/// previous reading standing. Read it only where one compile is known to have
/// closed.
#[cfg(test)]
thread_local! {
    static LAST_EFFECT_SEAT_CLOSURE: std::cell::RefCell<Option<EffectSeatClosure>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn last_effect_seat_closure() -> Option<EffectSeatClosure> {
    LAST_EFFECT_SEAT_CLOSURE.with(|cell| cell.borrow().clone())
}

/// How the `BufferAllocate` capacity seat was DISPATCHED, as `(specialized,
/// carried)`, since the last reset on this thread.
///
/// ⭐ **The premise a carried-capacity control cannot do without.** "The
/// compile succeeded and returned `InvalidBounds`" is the same green whether
/// the capacity took the carried route or the specialized one, so a fixture
/// that quietly stops carrying its capacity would leave the carried arm
/// untested and every assertion about it still passing. This is the only
/// instrument that separates those two worlds.
///
/// ⚠ It counts EMISSIONS, not executions: one compiled arm may run many times
/// or none. A control that wants "the carried route ran" needs the program's
/// own result, and this to know which route was compiled.
#[cfg(test)]
pub(in crate::cranelift_backend) fn capacity_phase_dispatch() -> (usize, usize) {
    super::CAPACITY_PHASE_DISPATCH.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn reset_capacity_phase_dispatch() {
    super::CAPACITY_PHASE_DISPATCH.with(|cell| cell.set((0, 0)));
}

/// **`D5a` checkpoint 4 step 1 — declare every generated context into ONE
/// generated function.**
///
/// Same per-function discipline as `WorkerTargets::declare_in_func`: the
/// `FuncRef`s belong to `func` alone and are never copied between functions.
/// ⛔ Keyed by the planner's `ContinuationContextId`, never by the body origin
/// the context executes -- that key is what would let a consumer resolve a
/// context from a body origin, which is the reconstruction the ruling forbids.
pub(in crate::cranelift_backend) fn declare_context_calls_in_func<M: Module>(
    module: &mut M,
    func: &mut Function,
    plan: &StaticTransitionPlan<'_>,
    bundle: &UnitBundle,
) -> Result<BTreeMap<ContinuationContextId, DeclaredUnitCall>, CraneliftBackendError> {
    let mut calls = BTreeMap::new();
    for context in plan.continuation_contexts()? {
        let target = bundle.context(context.id()).ok_or_else(|| {
            backend_module(
                "a planned generated context was never forward-declared".to_string(),
            )
        })?;
        let (offsets, _frame_bytes) = context.slot_offsets()?;
        calls.insert(
            context.id(),
            DeclaredUnitCall {
                function: module.declare_func_in_func(target, func),
                // The context EXECUTES this body, so the origin it answers for
                // is unchanged and the source edge it serves is untouched.
                origin: context.worker_body_origin(),
                call_site_origin: context.worker_body_origin(),
                header: context.header(),
                slots: context.slots().to_vec(),
                offsets,
            },
        );
    }
    Ok(calls)
}

pub(super) fn define_unit_bodies<M: Module>(
    module: &mut M,
    compiler: &mut Lowering<'_>,
    helpers: ArtifactHelpers<'_>,
    bundle: &UnitBundle,
    call_edges: &CallEdgeTargets,
    staged_root_value: Option<&RuntimeValue>,
) -> Result<RootUnitResult, CraneliftBackendError> {
    let root = compiler.static_transition_plan.root_emittable_unit()?.function();
    // `D4`: projected once, declared afresh into each generated function below.
    let worker_targets = resolve_worker_targets(&compiler.static_transition_plan, bundle)?;
    // `RT-DECL-CLOSURE-PORT` `D5` — opened here because this bundle pass is the
    // only place a checked same-SCC call can reach a declaration-owned unit.
    compiler.checked_call_ledger = Some(CheckedCallLedger::open(
        compiler.oriented_subcontinuation_plan.as_ref(),
    ));
    let mut root_result = None;
    // `D5a` checkpoint 1: the SAME population `declare_unit_bundle` walked.
    // Both passes read `executable_units` so declared and defined cannot
    // disagree -- reading different methods here is exactly how a phantom
    // appears.
    let emissions = compiler
        .static_transition_plan
        .executable_units()?
        .into_iter()
        .map(|unit| {
            let (offsets, frame_bytes) = unit.slot_offsets()?;
            Ok(OwnedUnitEmission {
                function: unit.function(),
                body_occurrence: unit.body_occurrence(),
                definition: unit.definition(),
                header: unit.header(),
                slots: unit.slots().to_vec(),
                offsets,
                frame_bytes,
            })
        })
        .collect::<Result<Vec<_>, CraneliftBackendError>>()?;
    for unit in emissions {
        let id = bundle.function(unit.function).ok_or_else(|| {
            backend_module("a planned unit was never forward-declared".to_string())
        })?;
        let is_root = root == unit.function;
        let outcome = define_unit_body(
            module,
            compiler,
            helpers,
            unit,
            id,
            bundle,
            call_edges,
            &worker_targets,
            is_root,
            staged_root_value,
        )?;
        if let Some(outcome) = outcome {
            if root_result.replace(outcome).is_some() {
                return Err(backend_module(
                    "more than one emitted unit claimed root result authority".to_string(),
                ));
            }
        }
    }
    // ⛔ The continuation ledger is NOT closed here. `D5a` checkpoint 2 moved
    // its lifetime out to the orchestration in `core.rs`, because two more
    // passes after this one declare, claim and emit causal calls and closing
    // here reported their tokens absent before they could exist.
    // `D5` closeout, before the artifact is published: planned = consumed =
    // emitted, and every emitted actual callee equals its exact resolved
    // target. ⛔ `consumed` is the affine machinery's OWN set, passed in rather
    // than mirrored, so the two cannot drift apart.
    compiler
        .checked_call_ledger
        .take()
        .ok_or_else(|| backend_module("the checked call ledger went missing".to_string()))?
        .close(&compiler.consumed_recursive_call_templates)?;
    root_result.ok_or_else(|| {
        backend_module("the emitted unit bundle did not define its recorded root".to_string())
    })
}

struct OwnedUnitEmission {
    function: PredeclaredFunctionId,
    /// The issued body occurrence this unit lowers, carried from the planner.
    body_occurrence: StaticOriginId,
    /// `RT-SRCBODY-BIND-ORDER` `D1` — carried because the ABI descriptor run
    /// and the source body's semantic environment are now two different orders,
    /// and only the definition arm says which units get the conversion.
    definition: AbiUnitDefinition,
    header: AbiFrameHeader,
    slots: Vec<AbiSlot>,
    offsets: Vec<u32>,
    frame_bytes: u32,
}

/// **`RT-SRCBODY-BIND-ORDER` `D1` — does this unit's body bind a SOURCE
/// parameter run?**
///
/// The ABI lays parameters out in declaration order; a source body indexes its
/// binders as de Bruijn levels from the innermost, which is the reverse. The
/// conversion is therefore owed exactly where the run being bound is a source
/// parameter run:
///
/// - `CallableDeclaration` and `ClosureBody` — yes. Both bind a written
///   parameter list, and `RT-DECL-CLOSURE-PORT` `D2` already treats the two
///   identically wherever the question is about their parameter/capture runs.
/// - `SchedulingEntry` — no. Its parameters are the closed process ingress
///   roles, resolved by `AbiProcessParameter` ordinal rather than by a binder
///   index, and reversing them would rename the two roles.
/// - `ContinuationSpecialization` — no. Its run is the planner's own ordered
///   input projection, positional by construction.
///
/// This is a question about the run's PROVENANCE, not its length: a one-
/// parameter source body is unaffected by the reversal but is still a source
/// body, and a two-role scheduling entry is affected but is still not one.
/// **`RT-SRCBODY-BIND-ORDER` `D3` — which host bound a semantic environment.**
///
/// The two seats that build one: a unit's own body, and a generated context
/// that lowers someone else's body. The equivalence `D2` owes is between them.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum SrcbodyBindHost {
    OrdinaryUnit,
    GeneratedContext,
}

/// **`RT-SRCBODY-BIND-ORDER` `D3` — one semantic environment, as PRODUCTION
/// built it.**
///
/// The row is deliberately raw: it records the ABI ordinals in the order they
/// were pushed into the environment, so a control re-derives the conversion
/// from the sequence rather than being told about it. `converted` is recorded
/// beside them precisely so a control can catch a build whose classification
/// is right and whose environment does not follow it — the failure a control
/// that reads the predicate alone is blind to.
#[cfg(test)]
#[derive(Clone, Debug)]
pub(in crate::cranelift_backend) struct SrcbodyBindOrderObservation {
    pub(in crate::cranelift_backend) host: SrcbodyBindHost,
    pub(in crate::cranelift_backend) definition: AbiUnitDefinition,
    /// What `source_body_binding_order` answered for `definition`.
    pub(in crate::cranelift_backend) converted: bool,
    /// The body this environment was built for. The join key for `D2`'s
    /// cross-host equivalence: one body, two hosts.
    pub(in crate::cranelift_backend) body_origin: StaticOriginId,
    /// Parameter ABI ordinals, in SEMANTIC ENVIRONMENT order.
    pub(in crate::cranelift_backend) parameter_ordinals: Vec<u32>,
    /// Capture ABI ordinals, in SEMANTIC ENVIRONMENT order.
    pub(in crate::cranelift_backend) capture_ordinals: Vec<u32>,
}

#[cfg(test)]
thread_local! {
    static SRCBODY_BIND_ORDER: std::cell::RefCell<Vec<SrcbodyBindOrderObservation>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(in crate::cranelift_backend) fn srcbody_bind_order_record(
    observation: SrcbodyBindOrderObservation,
) {
    SRCBODY_BIND_ORDER.with(|cell| cell.borrow_mut().push(observation));
}

/// Drains every environment built on this thread since the last take.
#[cfg(test)]
pub(in crate::cranelift_backend) fn srcbody_bind_order_take()
-> Vec<SrcbodyBindOrderObservation> {
    SRCBODY_BIND_ORDER.with(|cell| std::mem::take(&mut *cell.borrow_mut()))
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` — this became fallible for the
/// fusion arm, and `false` would have been the wrong answer rather than a safe
/// one.**
///
/// The other four arms answer from a property of their class that is already
/// settled. A static continuation fusion's binding order is not: it is decided
/// by how the emitter builds the generated definition's environment, and
/// `D2f`'s scoped source-body authorities are the successor seam. Answering
/// `false` here would pick one of the two orders for a class whose environment
/// nothing constructs yet, and the wrong pick is silent — it reverses an
/// operand run rather than failing.
pub(in crate::cranelift_backend) fn source_body_binding_order(
    definition: AbiUnitDefinition,
) -> Result<bool, CraneliftBackendError> {
    Ok(match definition {
        AbiUnitDefinition::CallableDeclaration { .. } | AbiUnitDefinition::ClosureBody { .. } => {
            true
        }
        AbiUnitDefinition::SchedulingEntry { .. }
        | AbiUnitDefinition::ContinuationSpecialization { .. } => false,
        AbiUnitDefinition::StaticContinuationFusion { .. } => {
            return Err(unsupported(
                "StaticContinuationFusion",
                "a static continuation fusion unit reached environment construction, but its \
                 source-body binding order is the emitter's to establish and no generated \
                 definition exists; RT-LEXICAL-RECURSOR-CONSUMERS D2f refuses rather than \
                 choosing an operand order for it",
            ));
        }
    })
}

fn define_unit_body<M: Module>(
    module: &mut M,
    compiler: &mut Lowering<'_>,
    helpers: ArtifactHelpers<'_>,
    unit: OwnedUnitEmission,
    id: FuncId,
    bundle: &UnitBundle,
    call_edges: &CallEdgeTargets,
    worker_targets: &WorkerTargets,
    is_root: bool,
    staged_root_value: Option<&RuntimeValue>,
) -> Result<Option<RootUnitResult>, CraneliftBackendError> {
    // ⭐ The declared size and the walked size must agree. They are the same
    // walk by construction (`abi::slot_offsets` totals for both), so this
    // rejects a corrupted descriptor rather than a divergent derivation.
    if unit.frame_bytes != unit.header.frame_bytes {
        return Err(backend_module(
            "abi frame size disagrees with its own slot run".to_string(),
        ));
    }
    let result_offset = unit
        .slots
        .iter()
        .zip(&unit.offsets)
        .find(|(slot, _)| slot.kind == AbiSlotKind::Result)
        .map(|(_, offset)| *offset)
        .ok_or_else(|| {
            // ⛔ Fails closed. `CONVENTION_SLOTS` puts a `Result` slot in every
            // unit, so its absence means the descriptor is not the one `B2R`
            // built, and returning a default word would fabricate a result.
            backend_module("unit frame declares no result slot".to_string())
        })?;
    let trap_offset = unit
        .slots
        .iter()
        .zip(&unit.offsets)
        .find(|(slot, _)| slot.kind == AbiSlotKind::Trap)
        .map(|(_, offset)| *offset)
        .ok_or_else(|| backend_module("unit frame declares no trap slot".to_string()))?;

    let sig = unit_signature(module);
    let mut func = Function::with_name_signature(UserFuncName::user(2, id.as_u32()), sig);
    // ⭐ `D4` — this unit's callees are referenced HERE, by the static identity
    // the planner assigned, before the body exists to call them.
    //
    // ⛔ **The call instructions themselves are `S6`'s**, because a unit body
    // does not descend into its own expression until `lower_expr`'s consumers
    // switch over. ⇒ What is live today is the **reference**: a `FuncRef`
    // resolved from a validated call edge through the declared bundle, with no
    // ordinal, no name parsing and no dynamic lookup anywhere on the path. ⚠ An
    // emitted `call` is not claimed and no control here asserts one.
    #[cfg(test)]
    let unit_trap_authority =
        match TRAP_FRAME_BINDING_MUTATION.with(std::cell::Cell::get) {
            TrapFrameBindingMutation::MisclassifyUnitAsRoot => Some(TrapExitAuthority::Root {
                process_sentinel: false,
                source_authorized: false,
            }),
            TrapFrameBindingMutation::Exact | TrapFrameBindingMutation::DeleteUnitLane => {
                None
            }
        };
    #[cfg(not(test))]
    let unit_trap_authority = None;
    let mut function_local =
        helpers.declare_in_func(module, &mut func, unit_trap_authority);
    let declared_calls = call_edges.declare_in_func(unit.function, module, &mut func)?;
    // `D3` — this ordinary Function declares its OWN `FuncRef` for every causal
    // token it owns, keyed by the four-field identity. Minted here, into this
    // `Function`; never passed across functions.
    function_local.continuation_calls = match compiler.continuation_claims.as_ref() {
        Some(ledger) => ledger.declare_owned_in_func(
            ContinuationEmissionOwner::Predeclared(unit.function),
            module,
            &mut func,
            &compiler.static_transition_plan,
        )?,
        None => BTreeMap::new(),
    };
    // `4b`: the DECLARED half of the four-set equality, recorded where the refs
    // are actually minted.
    if let Some(ledger) = compiler.continuation_claims.as_mut() {
        ledger.record_declared(function_local.continuation_calls.keys().cloned())?;
    }
    // `RT-DECL-CLOSURE-PORT` `D5a` contract 3 — AUTHORITY BEFORE EMISSION.
    //
    // Projected here, beside the declaration of this unit's own call refs and
    // **before the function is defined**. ⛔ It is a projection of authority the
    // planner already issued, keyed on the owner this pass is about to define;
    // lowering supplies only that owner and never searches, reverse-derives a
    // consumer, or reads an owner back off anything it emitted.
    let result_edges = compiler
        .static_transition_plan
        .continuation_result_edges_owned_by(ContinuationEmissionOwner::Predeclared(
            unit.function,
        ))?;
    // `D3`: the owner operand for the claim, supplied independently of any
    // token -- this is the ordinary producer unit currently being defined.
    #[cfg(test)]
    d5a_trace(format!(
        "UNIT-BODY entry function={:?} origin={:?}",
        unit.function, unit.body_occurrence
    ));
    compiler.open_aggregate_events(id)?;
    // `D8o` — bound for this body's lifetime and released on exit, so no later
    // Function inherits it. ⛔ The domain is unchanged: `D5a`'s rule that an
    // ordinary predeclared unit body emits as itself.
    let ambient = AmbientBodyAuthority::bind(
        compiler,
        ContinuationEmissionOwner::Predeclared(unit.function),
        unit.function,
    );
    #[cfg(test)]
    crate::cranelift_backend::lowering::record_d8o_body_key(
        compiler.defining_function_id,
        crate::cranelift_backend::lowering::D8oBodyKey::OrdinaryUnit(unit.function),
    );
    function_local.unit_calls = declared_calls.static_bodies;
    function_local.declaration_calls = declared_calls.declarations;
    // `RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` — THE REDIRECT.
    //
    // Installing body ownership removed the producer's invocation from
    // `executable_call_edges`, so the seat this unit calls the producer at has no
    // entry in the table above. That absence is not the redirect — it is the hole
    // the redirect fills, and leaving it is the `"retained body ... has no
    // graph-derived call target in this unit"` refusal.
    //
    // Applied to the function-local COPY, after the table is populated and
    // before any body is defined. Never to the plan's descriptor: the plane and
    // its ABI arena are the authority the frame contract below is checked
    // against, and mutating them would leave both sides agreeing and prove
    // nothing.
    redirect_fused_producer_invocations(module, &mut func, compiler, bundle, unit.function, &mut function_local.unit_calls)?;
    // `RT-DECL-CLOSURE-PORT` `D5` — the causal control on the ABI half.
    //
    // ⛔⛔ **Injected HERE, on the function-local COPY, and never on the plan's
    // descriptor.** That asymmetry is the entire point: `D5`'s ABI
    // reconciliation claims the declared call record still agrees with the
    // immutable descriptor `D3` validated. A mutation applied to both sides
    // would leave them agreeing and prove nothing; a mutation applied to the
    // descriptor would trip the ABI plane upstream instead. ⇒ Only a mutation
    // of the copy measures the reconciliation itself.
    #[cfg(test)]
    d5_mutate_declared_calls(&mut function_local.declaration_calls);
    // `D4`: this function's own worker refs, minted here and never copied.
    function_local.worker_calls = worker_targets.declare_in_func(module, &mut func);
    // `D6b` -- an ordinary unit body performs no retarget, so the two tables
    // agree. Populated anyway, for the same reason as the context body above.
    function_local.raw_worker_calls = function_local.worker_calls.clone();
    // `D5a` checkpoint 1: the raw template contracts, beside the call targets
    // and deliberately not derived from them.
    function_local.worker_templates = worker_targets.templates().clone();
    // `D5a` checkpoint 4 step 1: this function's own context call targets.
    function_local.context_calls = declare_context_calls_in_func(
        module,
        &mut func,
        &compiler.static_transition_plan,
        bundle,
    )?;
    // `D8n` — this generated Function's own checked-frame consumption
    // transaction, spanning the ordinary unit body exactly. ⛔ Opened before the builder and
    // closed after it, so every branch scope inside nests within it.
    let frame_scope = CheckedFrameFunctionScope::open(compiler)?;
    let mut func_ctx = FunctionBuilderContext::new();
    let root_outcome;
    {
        let mut builder = FunctionBuilder::new(&mut func, &mut func_ctx);
        let entry = builder.create_block();
        builder.append_block_params_for_function_params(entry);
        builder.switch_to_block(entry);
        let envelope = builder.block_params(entry)[0];
        let slots = builder.ins().load(
            module.target_config().pointer_type(),
            MemFlags::trusted(),
            envelope,
            crate::activation_services::UNIT_CALL_FRAME_SLOTS,
        );
        let host_dispatch_context = builder.ins().load(
            module.target_config().pointer_type(),
            MemFlags::trusted(),
            envelope,
            crate::activation_services::UNIT_CALL_FRAME_HOST_DISPATCH_CONTEXT,
        );
        let services = builder.block_params(entry)[1];
        let native_int_arena = builder.ins().load(
            module.target_config().pointer_type(),
            MemFlags::trusted(),
            services,
            crate::activation_services::SERVICES_NATIVE_INT_ARENA,
        );
        Lowering::require_nonzero(&mut builder, native_int_arena);
        let boundary_arena = builder.ins().load(
            module.target_config().pointer_type(),
            MemFlags::trusted(),
            services,
            crate::activation_services::SERVICES_BOUNDARY_ARENA,
        );
        Lowering::require_nonzero(&mut builder, boundary_arena);
        function_local.host_dispatch_context = Some(host_dispatch_context);
        function_local.native_int_arena = Some(native_int_arena);
        function_local.boundary_arena = Some(boundary_arena);
        function_local.services_pointer = Some(services);
        // The two fixed envelope loads are unconditional. Semantic frame
        // accesses below are relative only to the B2R payload base.
        #[cfg(test)]
        let bind_unit_trap_frame = TRAP_FRAME_BINDING_MUTATION.with(std::cell::Cell::get)
            != TrapFrameBindingMutation::DeleteUnitLane;
        #[cfg(not(test))]
        let bind_unit_trap_frame = true;
        if bind_unit_trap_frame {
            function_local.bind_unit_trap_frame(
                slots,
                i32::try_from(trap_offset).map_err(|_| {
                    backend_module("abi trap slot offset exceeds addressable range".to_string())
                })?,
            )?;
        }
        compiler.function_local = function_local;
        #[cfg(test)]
        if is_root
            && PROCESS_SLOT_MUTATION.with(std::cell::Cell::get)
                == ProcessSlotMutation::AttemptFixedContextOffsets
        {
            return Err(backend_module(
                "fixed host-dispatch context is not semantic process-pair storage".to_string(),
            ));
        }
        // `RT-SRCBODY-BIND-ORDER` `D1` — ONE walk, TWO orders.
        //
        // The walk below is still the single load of each Parameter/Capture
        // slot, and `defining_abi_operands` still receives them in descriptor
        // order, unchanged. What is no longer the same order is the **semantic
        // environment** the body is lowered against: `lower_expr` resolves
        // `Var(i)` as a de Bruijn index (`RuntimeExpr::Let` prepends at 0), so
        // the innermost binder is position 0 — while the ABI lays parameters
        // out in declaration order, ordinal 0 first. For a source body those
        // two are reverses of each other, and binding the descriptor run
        // directly delivered parameter 0 where the body asked for the last
        // parameter.
        //
        // The conversion applies only where the body IS a source body with
        // declared parameters — `CallableDeclaration` and `ClosureBody`. A
        // `SchedulingEntry`'s root ingress pair is a closed ABI role, not a
        // source binder run, and a `ContinuationSpecialization`'s projection is
        // the planner's own; both keep the descriptor order they had.
        let mut parameters = Vec::new();
        let mut captures = Vec::new();
        #[cfg(test)]
        let mut parameter_ordinals = Vec::new();
        #[cfg(test)]
        let mut capture_ordinals = Vec::new();
        for (slot, offset) in unit.slots.iter().zip(&unit.offsets) {
            if matches!(slot.kind, AbiSlotKind::Parameter | AbiSlotKind::Capture) {
                #[cfg(test)]
                let (base, offset) = if is_root
                    && slot.kind == AbiSlotKind::Parameter
                    && PROCESS_SLOT_MUTATION.with(std::cell::Cell::get)
                        == ProcessSlotMutation::ReintroduceLaunchIngress
                {
                    let offset = match slot.ordinal {
                        0 => crate::boundary_activation::ROOT_INGRESS_PROCESS_INPUT,
                        1 => crate::boundary_activation::ROOT_INGRESS_CAPABILITY,
                        _ => {
                            return Err(backend_module(
                                "the process-root mutation found an unknown parameter role"
                                    .to_string(),
                            ));
                        }
                    };
                    // The root adapter's companion mutation explicitly put
                    // launch ingress here. Without that producer-side change,
                    // the retained direct host context is not an admissible
                    // semantic-slot source.
                    (host_dispatch_context, offset)
                } else {
                    (
                        slots,
                        i32::try_from(*offset).map_err(|_| {
                            backend_module(
                                "abi input slot offset exceeds addressable range".to_string(),
                            )
                        })?,
                    )
                };
                #[cfg(not(test))]
                let (base, offset) = (
                    slots,
                    i32::try_from(*offset).map_err(|_| {
                        backend_module("abi input slot offset exceeds addressable range".to_string())
                    })?,
                );
                let word = builder.ins().load(
                    types::I64,
                    MemFlags::trusted(),
                    base,
                    offset,
                );
                let carried = CarriedBoundaryWord { word };
                // The process root's two ABI ordinals are closed semantic
                // roles, not generic ValueWord inputs. Recovering them here
                // prevents a borrowed process-input body from being emitted
                // twice behind a runtime carried-representation split.
                let operand = if is_root
                    && compiler.process_object
                    && slot.kind == AbiSlotKind::Parameter
                {
                    let value = compiler.emit_carrier_scalar(&mut builder, carried)?;
                    match slot.ordinal {
                        ordinal
                            if ordinal == AbiProcessParameter::ProcessInput.ordinal() =>
                        {
                            LoweringOperand::Specialized(Lowered::BorrowedNativeValue {
                                pointer: value,
                            })
                        }
                        ordinal
                            if ordinal == AbiProcessParameter::Capability.ordinal() =>
                        {
                            LoweringOperand::Specialized(Lowered::CapabilityToken {
                                value,
                            })
                        }
                        _ => {
                            return Err(backend_module(
                                "the process root has an unknown parameter role".to_string(),
                            ));
                        }
                    }
                } else {
                    LoweringOperand::Carried(carried)
                };
                // `D5a` checkpoint 4 step 1b: the SAME operand, recorded by ABI
                // position. Taken from this one walk rather than rebuilt, so
                // "index i is ABI position i" holds by construction instead of
                // by two walks agreeing.
                //
                // `RT-SRCBODY-BIND-ORDER` `D1` — that invariant is about THIS
                // vector and is unchanged: the push below is still in
                // descriptor order and is the only thing that writes it. The
                // semantic environment is now built separately, from the same
                // operands, and the two orders agree only where the definition
                // arm takes no conversion. Do not restore the old reading that
                // one push served both jobs.
                compiler
                    .function_local
                    .defining_abi_operands
                    .push(operand.clone());
                // `RT-SRCBODY-BIND-ORDER` `D3c` -- the kind at the same ABI
                // position, from the same walk, so the observatory can derive a
                // semantic position from the descriptor instead of searching
                // the environment for the operand.
                #[cfg(test)]
                compiler
                    .function_local
                    .defining_abi_slot_kinds
                    .push(slot.kind);
                match slot.kind {
                    AbiSlotKind::Parameter => {
                        #[cfg(test)]
                        parameter_ordinals.push(slot.ordinal);
                        parameters.push(LoweringEnvironmentBinding::Value(operand))
                    }
                    _ => {
                        #[cfg(test)]
                        capture_ordinals.push(slot.ordinal);
                        captures.push(LoweringEnvironmentBinding::Value(operand))
                    }
                }
            }
        }
        // `validate_slot_run` proves the Parameter run is a contiguous prefix
        // of the Capture run, so concatenating the two IS the descriptor order
        // — which is what the non-source arms below must keep byte-identically.
        let converts = source_body_binding_order(unit.definition)?;
        if converts {
            parameters.reverse();
            #[cfg(test)]
            parameter_ordinals.reverse();
        }
        let mut env = parameters;
        env.extend(captures);
        // The in-process validation API historically stages one ground
        // `RuntimeValue` as the root environment.  It is compile-time material,
        // not launch ingress and not a generated-call transfer, so it does not
        // acquire an ABI slot.  The value is lowered exactly once inside the
        // selected root unit; descendants can receive it only through their
        // ordinary declared captures.
        if is_root {
            if let Some(value) = staged_root_value {
                env.push(LoweringEnvironmentBinding::Value(
                    LoweringOperand::Specialized(compiler.lower_value(&mut builder, value)?),
                ));
            }
        }
        if is_root {
            compiler.root_terminal_authority =
                compiler.take_distinguished_root_answer_authority()?;
        }
        // **No root special case.** The planner issues every unit's body
        // occurrence at the visit that registered its scheduling entry, so the
        // carried value is already the right one for the root and for every
        // declaration alike.
        //
        // This conditional existed because `unit.origin` was an ALIAS of the
        // scheduling entry, which is wrong for any body that schedules
        // something before itself — the root was simply the one case that had
        // been noticed and patched. Substituting at one arm left every non-root
        // unit entering its entry and never reaching its body occurrence or the
        // join subtree beneath it. Reinstating a branch here would restore that
        // defect for whichever arm it did not cover.
        let body_origin = unit.body_occurrence;
        #[cfg(test)]
        srcbody_bind_order_record(SrcbodyBindOrderObservation {
            host: SrcbodyBindHost::OrdinaryUnit,
            definition: unit.definition,
            converted: converts,
            body_origin,
            parameter_ordinals,
            capture_ordinals,
        });
        let body = compiler.retained_body_occurrence(body_origin)?;
        compiler.select_terminal_result_origins(body_origin, body.expr)?;
        let lowered = compiler.lower_expr(&mut builder, body, &env)?;
        compiler.validate_join_plan_consumption(unit.function)?;
        let (result, outcome) = if is_root {
            match lowered {
                LoweringOperand::Carried(word) if !compiler.process_object => (
                    Some(word.word),
                    Some(RootUnitResult {
                        decoder: Some(ResultDecoder::Boundary),
                        trap: None,
                    }),
                ),
                LoweringOperand::Carried(word) => {
                    let tag = builder.ins().band_imm(
                        word.word,
                        crate::boundary_value::BOUNDARY_TAG_MASK as i64,
                    );
                    Lowering::require_i64(
                        &mut builder,
                        tag,
                        BoundaryTag::ImmediateExitStatus as i64,
                    );
                    let status = compiler.emit_carrier_scalar(&mut builder, word)?;
                    (
                        Some(status),
                        Some(RootUnitResult {
                            decoder: Some(ResultDecoder::ProcessStatus),
                            trap: None,
                        }),
                    )
                }
                LoweringOperand::Specialized(Lowered::Trap(trap)) => {
                    #[cfg(test)]
                    if compiler.process_object {
                        px8tr_record_trap_provenance(
                            Px8trTrapProvenanceEvent::FinalProcessObjectTrap {
                                trap: trap.clone(),
                            },
                        );
                    }
                    compiler.emit_current_trap(&mut builder, &trap)?;
                    (
                        None,
                        Some(RootUnitResult {
                            decoder: Some(ResultDecoder::TrapOnly),
                            trap: None,
                        }),
                    )
                }
                LoweringOperand::Specialized(value) => {
                    let (token, decoder) = compiler.emit_result(&mut builder, value)?;
                    (
                        Some(token),
                        Some(RootUnitResult {
                            decoder: Some(decoder),
                            trap: None,
                        }),
                    )
                }
            }
        } else {
            // `RT-DECL-CLOSURE-PORT` `D5a` — THE DETACHED-RESULT SEAT.
            //
            // The exact retained result is lowered; nothing has been
            // transferred into a carrier, allocated, published or joined. This
            // is where the landed object fixture was measured to refuse, and it
            // is the seat for every producer owner the fixed point detached as
            // an ordinary unit result.
            //
            // ⛔ Applied on the non-root path only, because this is the only
            // result surface that reaches `transfer_unit_result_into_carrier`.
            // A root that owned an undischarged projected call would be caught
            // by the whole-pass claim closure, not silently dropped.
            let lowered = compiler.eliminate_detached_producer_continuation(
                &mut builder,
                &result_edges,
                lowered,
                &env,
            )?;
            let word = match lowered {
                LoweringOperand::Carried(word) => Some(word.word),
                LoweringOperand::Specialized(Lowered::Trap(trap)) => {
                    compiler.emit_current_trap(&mut builder, &trap)?;
                    None
                }
                LoweringOperand::Specialized(value) => Some(
                    compiler
                        .transfer_unit_result_into_carrier(
                            &mut builder,
                            unit.body_occurrence,
                            &value,
                        )?
                        .word,
                ),
            };
            (word, None)
        };
        root_outcome = outcome;
        if let Some(result) = result {
            builder.ins().store(
                MemFlags::trusted(),
                result,
                slots,
                i32::try_from(result_offset).map_err(|_| {
                    backend_module("abi result slot offset exceeds addressable range".to_string())
                })?,
            );
        }
        let status = builder.ins().iconst(types::I64, 0);
        builder.ins().return_(&[status]);
        builder.seal_all_blocks();
        builder.finalize();
    }
    ambient.release(compiler);
    frame_scope.close(compiler)?;
    #[cfg(test)]
    d5a_trace(format!(
        "UNIT-BODY done function={:?} origin={:?} root={:?}",
        unit.function, unit.body_occurrence, root_outcome.as_ref().map(|_| "root")
    ));
    compiler.validate_materialized_dead_join_cfg(unit.function, &func)?;
    // `4b` -- the emission-seam equality gate, on the FINISHED function and
    // before it is defined into the module. The callee of every recorded causal
    // emission is decoded out of this CLIF and compared with the planner-issued
    // target; a disagreement rejects here rather than being emitted.
    compiler.verify_emitted_continuation_calls(&func, bundle)?;
    // `D8j` — same placement in the ordinary unit-body pass. ⛔ Both passes
    // gate, because a composed discharge can be claimed in either.
    compiler.verify_recorded_composed_discharges(&func, bundle)?;
    #[cfg(test)]
    crate::cranelift_backend::lowering::record_d8j_discharged(
        compiler.function_local.composed_discharges.keys().cloned(),
    );
    // `D8k` -- same accumulation on the ordinary pass. ⛔ The owner supplied is
    // this unit's own emission owner, so `record_composed` can hold the
    // discharge to the identity it claims rather than trusting the seat.
    if let Some(ledger) = compiler.continuation_claims.as_mut() {
        ledger.record_composed(
            compiler.function_local.composed_discharges.keys().cloned(),
            ContinuationEmissionOwner::Predeclared(unit.function),
        )?;
    }
    // `4b` closeout control: verify this function's emissions but never
    // accumulate them, so whole-pass set equality has a population to miss.
    #[cfg(test)]
    let accumulate = CONTINUATION_EMISSION_MUTATION.with(std::cell::Cell::get)
        != ContinuationEmissionMutation::SuppressEmissionAccumulation;
    #[cfg(not(test))]
    let accumulate = true;
    if let Some(ledger) = compiler.continuation_claims.as_mut() {
        if accumulate {
            ledger.record_emitted(compiler.function_local.continuation_emissions.keys().cloned())?;
        }
    }
    verify_cranelift_function(&func, module.isa())?;
    compiler.commit_aggregate_events()?;
    #[cfg(test)]
    scale_b_record_unit_body(&func);
    let mut ctx = module.make_context();
    std::mem::swap(&mut ctx.func, &mut func);
    module
        .define_function(id, &mut ctx)
        .map_err(|err| backend_module(err.to_string()))?;
    // ⛔ Counted HERE, adjacent to `define_function`, and NOT at the call site
    // in the loop above -- where it was first written and where it was
    // worthless.
    //
    // ⭐ A mutation gating the `define_unit_body` call left this test GREEN,
    // because a counter incremented once per loop iteration compares the
    // bundle's length to the length of the collection the loop walks, which are
    // equal by construction. It proved the loop ran and CLAIMED bodies were
    // defined. Only an increment on the emitting path can tell those apart.
    #[cfg(test)]
    B2F_UNIT_EMISSION.with(|cell| {
        let (declared, defined) = cell.get();
        cell.set((declared, defined + 1));
    });
    Ok(root_outcome)
}
