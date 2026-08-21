//! Recursor fusion-identity plane and continuation-key routing end-to-end
//! lowering tests (`RT-CONTROL-INTEGRATION-TESTS-SPLIT` D1, module 1 of 5,
//! split from `control.rs`: `d2f_*`, `r3_fused_*`/`r3_4b_*`,
//! `d0_r3_fusion_gate_*`, `contkey_*`, `required_consumer_*`,
//! `planned_closure_preexistence_*`, `missing_call_input_*`).

use super::*;
use super::control::{
    px8j_capture_source_trace, px8j_equal_payload_hole_placement,
    px8j_layered_recursive_result, px8j_scope_chain_observation_result,
    Px8jSelectedScopePlacement,
};

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` — a PRODUCTION compile builds the
/// fusion identity plane, and before this it never did.**
///
/// `D2h` landed the plane as production state under `allow(dead_code)`. Every
/// call to `build_static_continuation_fusion_plan` was inside `#[cfg(test)]`, so
/// the emitter's own input did not exist on any real compile — a fact invisible
/// to `D2h`'s controls, which call the builder directly and therefore cannot
/// distinguish "the plane is correct" from "the plane is never built".
///
/// ⛔ **The subject is REACHABILITY, not the plane's contents.** `D2h`'s
/// controls own correctness and are untouched. This asserts only that the
/// production path arrives at the builder, which is the precondition every
/// emitter AC rests on and the one nobody had measured.
///
/// **The observation is a vector, not a count, and that is load-bearing.** An
/// empty vector means production never reached the builder; a size alone cannot
/// tell that apart from reaching it and resolving nothing. Both are legal
/// today — this witness plans no admitted fusion — so the assertion is on
/// arrival, and the resolved sizes are recorded rather than pinned.
///
/// **There is no count equality here, and its absence is the point.** The
/// first cut of this control asserted `planes.len() == reached.get()` where
/// `reached` was built from `planes.len()` -- a tautology that read as a
/// two-population relation. An equality between counters is a measurement only
/// when its two sides come from **different reads**; this control has one read,
/// so the `expect` is the entire content. A second counter added to restore the
/// equality's shape would be cosmetic. The equality becomes real once a fusion
/// resolves, because builder arrival and resolved-fusion population are then
/// independently meaningful quantities -- that is the moment to reinstate it.
///
/// **Promise class: durable invariant.** The relation asserted is "a production
/// compile reaches the builder", which survives every intended extension; no
/// count, size, or identity is frozen.
#[test]
fn d2f_a_production_compile_builds_the_fusion_identity_plane() {
    let before = host_result_closure_match(px8j_equal_payload_hole_placement(
        Px8jSelectedScopePlacement::BeforeReturnHole,
    ));
    let _ = crate::cranelift_backend::lowering::core::d2f_production_fusion_planes_take();
    let (result, _trace) =
        px8j_capture_source_trace(&before, false, "ken_d2f_fusion_plane_wiring");
    assert!(matches!(
        result,
        Err(CraneliftBackendError::Unsupported(UnsupportedLowering {
            construct: "ComputationalMatch",
            reason,
        })) if reason == "a computational recursor closure names an in-flight activation, not a transferable value"
    ), "the plane must be observed before the fixture's measured D8 refusal");
    let planes = crate::cranelift_backend::lowering::core::d2f_production_fusion_planes_take();
    // THE WHOLE CLAIM, and it is one line on purpose. Delete the production
    // wiring and this is empty -- not merely a different size.
    //
    // There is deliberately NO equality here. `planes` is the only read this
    // control performs, so any count compared against `planes.len()` would be
    // derived from that same read and the comparison would restate the value
    // rather than check it. Arrival IS the measurement until a fusion resolves.
    std::num::NonZeroUsize::new(planes.len()).expect(
        "no production compile reached build_static_continuation_fusion_plan, so the fusion \
         identity plane is still test-only state and every emitter acceptance criterion rests \
         on an input that is never constructed",
    );
}

/// The governed `R3` terminal-stop selector population.
///
/// **MEASURED:** armed `Exact` and `ReHomed` reach the fused-selector controls
/// below, while `ProducerArity` refuses earlier at its widened producer
/// construct. **CLAIMED:** every terminal-stop control in this file ranges over
/// this same population. **THE GAP:** this const keeps the nine controls in
/// lockstep; it does not prove that a future cause reaches the terminal stop,
/// so the target-authority control separately executes `ProducerArity` and
/// pins its current earlier refusal.
const R3_TERMINAL_STOP_POPULATION: [(crate::cranelift_backend::planning::D2jCause, &'static str);
    2] = [
    (crate::cranelift_backend::planning::D2jCause::Exact, "exact"),
    (
        crate::cranelift_backend::planning::D2jCause::ReHomed,
        "rehomed",
    ),
];

/// `RT-LEXICAL-RECURSOR-CONSUMERS` `D2f` **Deliverable 0 — the applied-root
/// production-path gate.** Architect `evt_6907h4rv5kq1a` and `evt_4trsqtkxtghjx`.
///
/// **Every row here is rebaselined on its own cause-selected root.** No
/// coordinate, key, descriptor or refusal literal is transported from the
/// withdrawn bare-root revision, and there is no origin translation.
///
/// | row | phase reached | resolved | definitions |
/// |---|---|---|---|
/// | old `px8j` seed | builder, `oriented_present = false` | nothing | 0 |
/// | `Exact`, applied root | builder, then the twin's OWN ordinary refusal | one key, one descriptor | 0 |
/// | `ReHomed`, bare root | builder, then its OWN ordinary refusal | one key, one descriptor | 0 |
/// | `Frame` / `SelectedSlot` / `Invocation` | **outer transport validation, `validator_admitted = false`, refusing at its own named authority — no arrival** | nothing | 0 |
/// | `ExactSuffix` / `CallIdentity` | builder | nothing | 0 |
///
/// **The five refusal causes are TWO phases, not one.** The census records that
/// the outer validator did not admit three before the builder, while two arrive
/// and resolve zero. Both end with no key, ID, descriptor or definition — so a
/// control asserting only that shared consequence would read as satisfied while
/// being blind to which phase actually fired. `validator_admitted` asserts the
/// outer-validator phase per tier-3 row; arrival asserts the builder phase for
/// the remaining pair.
///
/// **What arrival LENGTH actually counts, and why it is NOT what carries the
/// tier-3 rows.** `D2k-1d`, from Adversary `evt_1gk177jznv5rz`. An earlier
/// revision of this doc called `d2f_gate_note_arrival` an *"unconditional push
/// — executed once per builder call with no predicate in front of it"* and
/// concluded that `0` means the builder was never reached. **The push has no
/// predicate in front of it and it does have an early return in front of it:**
/// it and `d2f_note_production_fusion_plane` both sit below
/// `build_static_continuation_fusion_plan(...)?`, whose four error exits
/// short-circuit them. So the length counts builder calls **that returned
/// `Ok`**, and a `0` means "never reached" OR "reached and errored" — the third
/// case the old dichotomy omitted, and one that genuinely occurs for these same
/// three causes wherever the builder is called directly.
///
/// **So each tier-3 row asserts the census PHASE and its refusal SENTENCE, and
/// its count is supplementary.** The three read `0` today because the OUTER
/// `validate_oriented_subcontinuation_transport` at the compile entry runs
/// before the builder and returns through `?` — while the builder's own first
/// statement is a second call of that same validator. That ordering is the real
/// ground of the two-tier split. A reordering would leave every tier-3 row
/// reading `0` unchanged, but `validator_admitted` would change to `true` if the
/// outer validator admitted and the builder's copy refused. The refusal
/// sentence separately names the authority that refused. `D2k-1e` deliberately
/// does not touch the duplicated validator; whether the outer call should exist
/// is a separate question from whether this control measures which copy fired.
///
/// **The arrived-empty pair still rests on its count, and soundly.** For
/// `ExactSuffix` and `CallIdentity` a length of `1` is a positive fact — the
/// builder was reached AND returned `Ok` — and no error path can be mistaken
/// for it. It is only the zero that is ambiguous.
///
/// **Where "sole production compile path" comes from, and what it rests on.**
/// `compile_expr_into_module_with_root_projection` is the only scope holding
/// the static transition plan and the oriented plan authoritatively at once,
/// and the four public compile entries — `compile_expr_into_module`,
/// `compile_expr_into_object_module`, `compile_program_expr_into_module` and
/// `compile_program_expr_into_object_module` — all delegate into it. That is a
/// **structural** claim about the delegation graph: it is established by those
/// four being its only callers, not by a measurement over program shapes.
/// Stated so a reader knows which kind of claim it is.
///
/// ⇒ **"Production compile" in this row's NAME means that delegation graph, and
/// NOT a compile of real Ken source.** The fixtures here are `D2jCause`
/// synthetic programs, as at every control on this seat. Measured 2026-08-13
/// and recorded in full on
/// [`d2f_the_two_binder_projections_share_one_source_field_transport`]: **eight
/// SAMPLED real-source programs** censused zero `ComputationalMatch`, including
/// `px7p`'s own green integration program. The name should not be read as
/// claiming a real-source witness, because these fixtures are not one.
///
/// ⛔ **Do NOT read that as "the front end emits none."** It does emit them —
/// measured 2026-08-13 via the nested-result selector path, which the eight
/// programs did not exercise. What holds here is the narrower fact that **these
/// rows' own fixtures are synthetic**, which is a statement about this file and
/// not about the front end.
///
/// **Named rather than cited by line**, because a line number goes stale
/// silently and then points at unrelated real content — a reader who follows it
/// gets a confident wrong answer instead of a missing one.
///
/// **`ReHomed` is a POSITIVE on a bare root**, not a negative comparator. That
/// cause removes the outer `LexicalClosure`, so the re-homed program has zero
/// ABI inputs and applying two `Unit`s would build an ill-typed program whose
/// failure would be evidence about this harness. It gets its own root from the
/// one constructor's explicit branch, and its own planner comparator.
///
/// **Promise class: durable invariant.** Arrival, presence, phase, and key
/// agreement between two independently reached derivations. The literals are
/// `1` (the cardinality the identity plane is defined to produce for one
/// candidate) and `0` (the definition population before an emitter exists).
/// `D2k-1d` adds three refusal-sentence literals, one per tier-3 cause. They
/// are the same class as the `in-flight activation` literal already asserted on
/// the positives: each names **which gate refused**, so it moves only when that
/// gate's authority moves, and a cause that broke something upstream instead
/// would fail here rather than pass as coverage.
/// **`R3` — the ARMED `D2f` compile's current terminal stop, pinned so the
/// source comment beside the installer cannot silently stop being true.**
///
/// Architect `evt_6kn9ckdnbf0ph` §5 requires the corrected stop comment to
/// carry *"a control or assertion that becomes red when the statement ceases to
/// be true"*, because the prose it replaces claimed a step-5/step-6 stop that
/// had already been overtaken with nothing going red.
///
/// **MEASURED**, armed on an isolated `Exact` and `ReHomed` compile: **the
/// compile COMPLETES. There is no terminal stop left to pin.** Measured beside
/// it and asserted below, so completion is not the only thing this row states:
/// exactly one fusion-local composition is realized per compile, at the
/// **`Inner`** layer, and the outer dispatch / fused invocation pair is `(1, 1)`.
///
/// **CLAIMED:** the armed `D2f` emitter chain lowers both roots end to end into
/// an object module, with those populations.
///
/// **THE GAP:** completion is not execution. Nothing here runs the emitted code
/// or checks that it computes the right answer — this row says the compile
/// produces an artifact, not that the artifact is correct. It also says nothing
/// about roots other than these two.
///
/// **THIS ROW WAS A TRANSITION SENTINEL FOR FIVE INCREMENTS, AND IT HAS NOW
/// RETIRED ITSELF BY REACHING ITS OWN BOUNDARY.** Each red was the stop moving
/// forward and each restatement is recorded rather than rewritten, so the
/// movement is auditable:
///
/// 1. `ContinuationSpecialization: the claimed continuation target was not
///    declared into this function` — the fail-closed intermediate of the
///    half-landed replacement, when `O = P \ F` had closed the direct path for
///    `F` and the local path was not yet open.
/// 2. the root-result escape in `emit_result` -> `ground_value` ->
///    `into_specialized_at`, on the fusion key's own producer construct. Per
///    `evt_6kn9ckdnbf0ph` that order was the *pre-mechanism* one and never a
///    refutation — the worker is a compiler-local intermediate between two
///    lowering steps, so its absence from any ABI operand run is the design.
/// 3. `StaticContinuationFusion: a fusion-composition splice issued a capability
///    that no dynamic invocation segment consumed` — the splice closeout's
///    *outstanding* arm, which had never been evaluated on this witness because
///    every prior armed compile took its **descent-failed** arm instead. The
///    splice capability was later retired outright.
/// 4. `StaticWorkerBinding: ... requires an ordinary specialized constructor
///    field` — the composed eliminator had no disposition for a recursive
///    position whose field transports a worker. Cleared by the two-member
///    binder wiring (`evt_5yhm9c78dm27s`).
/// 5. `StaticWorkerBinding: ... was rebound into the binding authority as
///    transport ... and never consumed at an exact-Var call` — the ledger close,
///    likewise never evaluated before, because stop 4 refused ahead of the
///    rebind. Cleared by `evt_37715knv356yp`: one source-field transport,
///    projected to both authorized binder members, consumed through either.
///
/// **Promise class: durable invariant**, changed from *transition sentinel* now
/// that the boundary it was named for has been reached. Completing an armed
/// compile is a property the mechanism must keep, not a waypoint — an intended
/// extension that preserves the contract keeps this green, and a regression in
/// the emitter chain reds it. ⛔ Do not re-weaken it to "stops at X" if a future
/// change reintroduces a stop: that would pin the regression as the expectation.
///
/// **The population assertions must NOT be relaxed into inequalities.**
/// `>= 1 realized` stays green through both the repair and its absence, and
/// `>= 1` on the outer pair stays green through a compile that reaches the
/// descent and drops the invocation.
///
/// Production stays unarmed. `D2F_EMITTER_ARMED` is `false`; the arm here is
/// the `cfg(test)` RAII `D2fEmitterTestArm`, which disarms on drop so a
/// panicking assertion cannot leak an armed gate into the next test on this
/// thread.
#[test]
fn d2f_armed_compile_completes_and_its_populations_are_pinned() {
    use crate::cranelift_backend::lowering::core::D2fEmitterTestArm;
    use crate::cranelift_backend::planning::{d2j_checked_fixture_under, D2jCause};

    fn compile_armed(cause: D2jCause, symbol: &str) -> Option<CraneliftBackendError> {
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let mut declarations = std::collections::BTreeMap::new();
        declarations.insert(
            crate::cranelift_backend::planning::D2J_DECLARATION,
            &declaration,
        );
        // Armed for exactly this compile and disarmed on drop.
        let _arm = D2fEmitterTestArm::arm();
        crate::cranelift_backend::lowering::core::compile_expr_into_object_module(
            crate::cranelift_backend::artifact::new_object_module_for_lowering_tests(
                "ken-d2f-armed-stop",
            )
            .expect("object module"),
            symbol,
            cranelift_module::Linkage::Export,
            &entry,
            &crate::NativeSeedEnvironment::empty(),
            declarations,
            None,
            false,
            None,
            None,
            Some(oriented),
        )
        .err()
    }

    let mut rows = Vec::new();
    let mut realized = Vec::new();
    let mut realized_outer = Vec::new();
    for (cause, symbol) in [
        (D2jCause::Exact, "ken_d2f_armed_stop_exact"),
        (D2jCause::ReHomed, "ken_d2f_armed_stop_rehomed"),
    ] {
        crate::cranelift_backend::lowering::reset_r3_local_compositions();
        crate::cranelift_backend::lowering::reset_r3_outer_dispatches();
        crate::cranelift_backend::lowering::reset_r3_fused_invocations();
        let error = compile_armed(cause, symbol);
        // THE STOP IS GONE. This is no longer "which refusal did we reach" --
        // it is whether the armed compile completes at all.
        let reached = error.is_none();
        rows.push((cause, reached));
        realized_outer.push((
            cause,
            crate::cranelift_backend::lowering::r3_outer_dispatches().len(),
            crate::cranelift_backend::lowering::r3_fused_invocations().len(),
        ));
        // The LAYERS, not a count. A count of one is satisfied by the wrong
        // one of the two planned edges, and which layer is realized is exactly
        // the fact the open question turns on.
        realized.push((
            cause,
            crate::cranelift_backend::lowering::r3_local_compositions()
                .into_iter()
                .map(|(_, layer)| layer)
                .collect::<Vec<_>>(),
        ));
    }

    assert_eq!(
        rows,
        vec![(D2jCause::Exact, true), (D2jCause::ReHomed, true)],
        "THE ARMED COMPILE MUST COMPLETE on both roots. There is no terminal stop left to \
         pin: the shared-transport ruling cleared the last one, and this row is now a DURABLE \
         INVARIANT rather than the transition sentinel it was for five increments. \
         ⛔ If this is red, the armed emitter chain has REGRESSED. Do not restate it to \
         whatever refusal now fires -- that would pin the regression as the expectation, which \
         is the opposite of what every earlier restatement of this row did. The five superseded \
         stops are listed on this function's doc comment; a sixth belongs there only if a \
         RULING deliberately reintroduces one"
    );
    assert_eq!(
        realized_outer,
        vec![
            (D2jCause::Exact, 1usize, 1usize),
            (D2jCause::ReHomed, 1usize, 1usize),
        ],
        "R is REACHED again, and the descent and the fused invocation are pinned as a PAIR -- \
         one outer dispatch and one fused invocation per compile, never a count on either alone. \
         THE RESTORATION THIS ROW ASKED FOR, and it arrived by the predicted route: the pair was \
         (1, 1) at `ddb04292`, went to (0, 0) when the eliminator-role axis moved the I path's \
         stop into the fused body's own definition -- which runs BEFORE `define_unit_bodies`, so \
         the compile ended ahead of the consumer unit's body where both live -- and returns to \
         (1, 1) now that the two-member binder wiring carries the I path past that stop. The \
         (0, 0) reading was a statement about ORDER and never about R, and it is kept here \
         because a future reader meeting a (0, 0) red needs to know it has been lawful once. \
         ⛔ Never relax either element to an inequality: `>= 1` on the pair stays green through \
         a compile that reaches the descent and drops the invocation, which is the one \
         asymmetry this row exists to catch"
    );
    assert_eq!(
        realized,
        vec![
            (D2jCause::Exact, vec![FusionCompositionLayer::Inner]),
            (D2jCause::ReHomed, vec![FusionCompositionLayer::Inner]),
        ],
        "exactly one of the two planned composed edges is realized per compile, and it is the \
         Inner one. The Outer edge's producer construct reaches no claim seat anywhere in the \
         armed compile, so its composition is not merely refused -- it is never attempted. If \
         this row is red because the Outer edge became reachable, that is the mechanism \
         advancing and the measurement must be restated, not widened to an inequality"
    );

    // The arm is scoped, so an unarmed compile must still reach the ordinary
    // baseline. Without this the row above would pass just as well if arming
    // had silently stopped working.
    let (entry, declaration, oriented) = d2j_checked_fixture_under(D2jCause::Exact);
    let mut declarations = std::collections::BTreeMap::new();
    declarations.insert(
        crate::cranelift_backend::planning::D2J_DECLARATION,
        &declaration,
    );
    let unarmed = crate::cranelift_backend::lowering::core::compile_expr_into_object_module(
        crate::cranelift_backend::artifact::new_object_module_for_lowering_tests(
            "ken-d2f-unarmed-stop",
        )
        .expect("object module"),
        "ken_d2f_unarmed_stop",
        cranelift_module::Linkage::Export,
        &entry,
        &crate::NativeSeedEnvironment::empty(),
        declarations,
        None,
        false,
        None,
        None,
        Some(oriented),
    )
    .err();
    assert!(
        matches!(
            &unarmed,
            Some(CraneliftBackendError::Unsupported(UnsupportedLowering { construct, reason }))
                if *construct == "ComputationalMatch" && reason.contains("in-flight activation")
        ),
        "the POSITIVE control for the arm: unarmed, the same root must still reach the ordinary \
         in-flight-activation baseline, so the armed row above is attributable to arming and not \
         to a compile that refuses everywhere: {unarmed:?}"
    );
}

/// `RT-LEXICAL-R3-FUSION-EMITTER` `D3` -- the fused invocation accepts only
/// the claim's exact ordered argument-origin projection, and a refusal cannot
/// spend the affine region claim.
///
/// **MEASURED:** on both armed roots, the exact projection emits one fused call
/// and consumes one claim. Replacing its sole origin with the consuming callee
/// reaches lowering's visited-origin comparison and refuses with neither event;
/// dropping or appending one origin refuses in preflight, also with neither
/// event.
///
/// **CLAIMED:** the planner closes projection arity before issuing a claim, and
/// lowering independently closes the same ordered projection against the
/// arguments it actually visited before the claim is consumed.
///
/// **THE GAP:** both governed roots have one explicit argument, so their only
/// permutation is the identity. A non-identity reorder is unrepresentable on
/// this population, and the production projection has no independent ordering
/// choice: its index is the source-child traversal index. A synthetic
/// multi-argument witness would measure a population production does not have.
/// Also, shortening a unary run produces the same empty `Vec` as an absent
/// projection; because the claim has no presence bit, that row proves an empty
/// parameter run refuses. It does not distinguish wrong length from absence.
/// Reorder and length-versus-presence become owed when a multi-argument fused
/// invocation exists in the governed population; no such witness is
/// manufactured here.
#[test]
fn r3_fused_parameter_projection_refuses_before_claim_consumption() {
    use crate::cranelift_backend::lowering::core::D2fEmitterTestArm;
    use crate::cranelift_backend::planning::{
        d2j_checked_fixture_under, r3_fusion_claim_consumptions,
        reset_r3_fusion_claim_consumptions, with_fusion_claim_parameter_mutation, D2jCause,
        FusionClaimParameterMutation,
    };

    fn compile(
        cause: D2jCause,
        mutation: FusionClaimParameterMutation,
        symbol: &str,
    ) -> (Option<CraneliftBackendError>, usize, usize) {
        reset_r3_fusion_claim_consumptions();
        crate::cranelift_backend::lowering::reset_r3_fused_invocations();
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let mut declarations = std::collections::BTreeMap::new();
        declarations.insert(
            crate::cranelift_backend::planning::D2J_DECLARATION,
            &declaration,
        );
        let error = with_fusion_claim_parameter_mutation(mutation, || {
            let _arm = D2fEmitterTestArm::arm();
            crate::cranelift_backend::lowering::core::compile_expr_into_object_module(
                crate::cranelift_backend::artifact::new_object_module_for_lowering_tests(
                    "ken-r3-parameter-projection",
                )
                .expect("object module"),
                symbol,
                cranelift_module::Linkage::Export,
                &entry,
                &crate::NativeSeedEnvironment::empty(),
                declarations,
                None,
                false,
                None,
                None,
                Some(oriented),
            )
            .err()
        });
        (
            error,
            r3_fusion_claim_consumptions().len(),
            crate::cranelift_backend::lowering::r3_fused_invocations().len(),
        )
    }

    fn classify(error: &Option<CraneliftBackendError>) -> &'static str {
        match error {
            None => "completed",
            Some(CraneliftBackendError::Unsupported(UnsupportedLowering {
                construct,
                reason,
            })) if *construct == "StaticContinuationFusion"
                && reason.contains("visited argument origins") =>
            {
                "visited-origin refusal"
            }
            Some(CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(reason)))
                if reason
                    == "a static continuation fusion claim's ordered input projection is \
                       unavailable or disagrees with the capture run its own ABI frame declares" =>
            {
                "preflight arity refusal"
            }
            Some(_) => "other refusal",
        }
    }

    let mut rows = Vec::new();
    for (cause, prefix) in R3_TERMINAL_STOP_POPULATION {
        for (mutation, suffix) in [
            (FusionClaimParameterMutation::Exact, "exact"),
            (FusionClaimParameterMutation::MoveFirstToCallee, "moved"),
            (FusionClaimParameterMutation::DropLast, "short"),
            (FusionClaimParameterMutation::AppendCallee, "long"),
        ] {
            let symbol = format!("ken_r3_projection_{prefix}_{suffix}");
            let (error, consumptions, invocations) = compile(cause, mutation, &symbol);
            rows.push((cause, mutation, classify(&error), consumptions, invocations));
        }
    }

    assert_eq!(
        rows,
        vec![
            (
                D2jCause::Exact,
                FusionClaimParameterMutation::Exact,
                "completed",
                1,
                1,
            ),
            (
                D2jCause::Exact,
                FusionClaimParameterMutation::MoveFirstToCallee,
                "visited-origin refusal",
                0,
                0,
            ),
            (
                D2jCause::Exact,
                FusionClaimParameterMutation::DropLast,
                "preflight arity refusal",
                0,
                0,
            ),
            (
                D2jCause::Exact,
                FusionClaimParameterMutation::AppendCallee,
                "preflight arity refusal",
                0,
                0,
            ),
            (
                D2jCause::ReHomed,
                FusionClaimParameterMutation::Exact,
                "completed",
                1,
                1,
            ),
            (
                D2jCause::ReHomed,
                FusionClaimParameterMutation::MoveFirstToCallee,
                "visited-origin refusal",
                0,
                0,
            ),
            (
                D2jCause::ReHomed,
                FusionClaimParameterMutation::DropLast,
                "preflight arity refusal",
                0,
                0,
            ),
            (
                D2jCause::ReHomed,
                FusionClaimParameterMutation::AppendCallee,
                "preflight arity refusal",
                0,
                0,
            ),
        ],
        "the exact projection completes and consumes once; a same-length moved origin reaches \
         lowering's own ordered comparison, while short and long projections stop in preflight; \
         every refusal occurs before claim consumption and before a fused invocation is recorded"
    );
}

/// `RT-LEXICAL-R3-FUSION-EMITTER` `D3` -- the checked worker presented at the
/// exact consuming call must name the claim's producer body before the affine
/// claim can be spent.
///
/// **MEASURED:** after the real selector chooses the claim on both armed roots,
/// replacing the worker binding's body with that same real consuming-call
/// occurrence reaches the worker-body closure and refuses with zero claim
/// consumptions and zero fused invocations. The exact binding completes at one
/// and one.
///
/// **CLAIMED:** a fused invocation cannot enter a worker body other than the
/// producer body and redirect callee the region claim closed over, and refusal
/// leaves the affine claim outstanding.
///
/// **THE GAP:** this pins the worker-body relation after exact call selection.
/// It does not pin the consuming-call selector, producer captures, the
/// post-field route, or a failure after the declared call starts building.
///
/// **Promise class: durable invariant.** The assertion is relational over the
/// real selector outcome, the named refusal, and both affine events; it freezes
/// no origin literal or population count.
#[test]
fn r3_fused_worker_body_refuses_before_claim_consumption() {
    use crate::cranelift_backend::lowering::core::{
        with_d2f_worker_body_mutation, D2fEmitterTestArm, D2fWorkerBodyMutation,
    };
    use crate::cranelift_backend::planning::{
        d2j_checked_fixture_under, r3_fusion_claim_consumptions,
        reset_r3_fusion_claim_consumptions, D2jCause, D2J_DECLARATION,
    };

    fn compile(
        cause: D2jCause,
        mutation: D2fWorkerBodyMutation,
        symbol: &str,
    ) -> (Option<CraneliftBackendError>, usize, usize) {
        reset_r3_fusion_claim_consumptions();
        crate::cranelift_backend::lowering::reset_r3_fused_invocations();
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let mut declarations = std::collections::BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        let error = with_d2f_worker_body_mutation(mutation, || {
            let _arm = D2fEmitterTestArm::arm();
            crate::cranelift_backend::lowering::core::compile_expr_into_object_module(
                crate::cranelift_backend::artifact::new_object_module_for_lowering_tests(
                    "ken-r3-worker-body",
                )
                .expect("object module"),
                symbol,
                cranelift_module::Linkage::Export,
                &entry,
                &crate::NativeSeedEnvironment::empty(),
                declarations,
                None,
                false,
                None,
                None,
                Some(oriented),
            )
            .err()
        });
        (
            error,
            r3_fusion_claim_consumptions().len(),
            crate::cranelift_backend::lowering::r3_fused_invocations().len(),
        )
    }

    fn classify(error: &Option<CraneliftBackendError>) -> &'static str {
        match error {
            None => "completed",
            Some(CraneliftBackendError::Unsupported(UnsupportedLowering { construct, reason }))
                if *construct == "StaticContinuationFusion"
                    && reason.contains("checked worker body")
                    && reason.contains("is not the claim's producer body") =>
            {
                "worker-body refusal"
            }
            Some(_) => "other refusal",
        }
    }

    let mut rows = Vec::new();
    for (cause, prefix) in R3_TERMINAL_STOP_POPULATION {
        for (mutation, suffix) in [
            (D2fWorkerBodyMutation::Exact, "exact"),
            (D2fWorkerBodyMutation::UseConsumingCallOrigin, "wrong-body"),
        ] {
            let symbol = format!("ken_r3_worker_body_{prefix}_{suffix}");
            let (error, consumptions, invocations) = compile(cause, mutation, &symbol);
            rows.push((cause, mutation, classify(&error), consumptions, invocations));
        }
    }

    assert_eq!(
        rows,
        vec![
            (
                D2jCause::Exact,
                D2fWorkerBodyMutation::Exact,
                "completed",
                1,
                1,
            ),
            (
                D2jCause::Exact,
                D2fWorkerBodyMutation::UseConsumingCallOrigin,
                "worker-body refusal",
                0,
                0,
            ),
            (
                D2jCause::ReHomed,
                D2fWorkerBodyMutation::Exact,
                "completed",
                1,
                1,
            ),
            (
                D2jCause::ReHomed,
                D2fWorkerBodyMutation::UseConsumingCallOrigin,
                "worker-body refusal",
                0,
                0,
            ),
        ],
        "the exact worker body completes and consumes once; replacing that binding's body with \
         the selected consuming-call occurrence reaches the named closure and refuses before \
         either affine event"
    );
}

/// `RT-LEXICAL-R3-FUSION-EMITTER` `D3` -- the selected Call occurrence must be
/// the claim's exact consuming Call before its ordered ordinary parameter run
/// can enter the fused target.
///
/// **MEASURED:** after the real selector chooses the claim on both armed roots,
/// substituting a real occurrence from that call's same-compile projected
/// argument run reaches the consuming-call closure and refuses with zero claim
/// consumptions and zero fused invocations. Exact selection completes at one
/// and one.
///
/// **CLAIMED:** the field supplies the local worker binding, the claim's exact
/// consuming Call supplies the ordered ordinary parameter run, and the checked
/// worker body closes independently against `claim.producer_body()`.
///
/// **THE GAP:** this pins the consuming-call relation after exact selection. It
/// does not pin producer captures, premature claim consumption, or the
/// post-field direct-call route.
///
/// **Promise class: durable invariant.** The alternative is selected from the
/// exact descent's real same-compile projected run while excluding the claim's
/// consuming call, seat, producer body, and redirect callee. No origin literal,
/// sentinel, hand-built claim, or coincident call-site/callee identity can
/// discharge it.
#[test]
fn r3_fused_wrong_consuming_call_refuses_before_claim_consumption() {
    use crate::cranelift_backend::lowering::core::{
        with_d2f_consuming_call_mutation, D2fConsumingCallMutation, D2fEmitterTestArm,
    };
    use crate::cranelift_backend::planning::{
        d2j_checked_fixture_under, r3_fusion_claim_consumptions,
        reset_r3_fusion_claim_consumptions, D2jCause, D2J_DECLARATION,
    };

    fn compile(
        cause: D2jCause,
        mutation: D2fConsumingCallMutation,
        symbol: &str,
    ) -> (Option<CraneliftBackendError>, usize, usize) {
        reset_r3_fusion_claim_consumptions();
        crate::cranelift_backend::lowering::reset_r3_fused_invocations();
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let mut declarations = std::collections::BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        let error = with_d2f_consuming_call_mutation(mutation, || {
            let _arm = D2fEmitterTestArm::arm();
            crate::cranelift_backend::lowering::core::compile_expr_into_object_module(
                crate::cranelift_backend::artifact::new_object_module_for_lowering_tests(
                    "ken-r3-consuming-call",
                )
                .expect("object module"),
                symbol,
                cranelift_module::Linkage::Export,
                &entry,
                &crate::NativeSeedEnvironment::empty(),
                declarations,
                None,
                false,
                None,
                None,
                Some(oriented),
            )
            .err()
        });
        (
            error,
            r3_fusion_claim_consumptions().len(),
            crate::cranelift_backend::lowering::r3_fused_invocations().len(),
        )
    }

    fn classify(error: &Option<CraneliftBackendError>) -> &'static str {
        match error {
            None => "completed",
            Some(CraneliftBackendError::Unsupported(UnsupportedLowering { construct, reason }))
                if *construct == "StaticContinuationFusion"
                    && reason.contains("selected fused consuming Call occurrence")
                    && reason.contains("not the claim's exact consuming Call") =>
            {
                "consuming-call refusal"
            }
            Some(_) => "other refusal",
        }
    }

    let mut rows = Vec::new();
    for (cause, prefix) in R3_TERMINAL_STOP_POPULATION {
        for (mutation, suffix) in [
            (D2fConsumingCallMutation::Exact, "exact"),
            (
                D2fConsumingCallMutation::UseProjectedArgumentOccurrence,
                "wrong-call",
            ),
        ] {
            let symbol = format!("ken_r3_consuming_call_{prefix}_{suffix}");
            let (error, consumptions, invocations) = compile(cause, mutation, &symbol);
            rows.push((cause, mutation, classify(&error), consumptions, invocations));
        }
    }

    assert_eq!(
        rows,
        vec![
            (
                D2jCause::Exact,
                D2fConsumingCallMutation::Exact,
                "completed",
                1,
                1,
            ),
            (
                D2jCause::Exact,
                D2fConsumingCallMutation::UseProjectedArgumentOccurrence,
                "consuming-call refusal",
                0,
                0,
            ),
            (
                D2jCause::ReHomed,
                D2fConsumingCallMutation::Exact,
                "completed",
                1,
                1,
            ),
            (
                D2jCause::ReHomed,
                D2fConsumingCallMutation::UseProjectedArgumentOccurrence,
                "consuming-call refusal",
                0,
                0,
            ),
        ],
        "exact selection completes and consumes once; a different real occurrence from the \
         call's projected run reaches the named closure and refuses before either affine event"
    );
}

/// `RT-LEXICAL-R3-FUSION-EMITTER` `D3` -- fusion admission accepts an empty
/// producer-capture run only, independently of the checked continuation-capture
/// suffix later exposed as `claim.inputs()`.
///
/// **MEASURED:** the real selected producer descriptor has zero captures on
/// both governed roots. Exact retains its two checked continuation captures and
/// completes at one claim consumption and one fused invocation; ReHomed retains
/// its zero-capture comparator and also completes at one and one. Changing only
/// the post-selection producer-capture count to non-empty reaches the named
/// admission refusal on both roots at zero and zero.
///
/// **CLAIMED:** a non-empty producer-capture population is a new ABI
/// disposition and refuses before fusion ABI installation, emission, affine
/// claim consumption, or fused invocation. It is never folded into ordinary
/// parameters or into the distinct `claim.inputs()` suffix.
///
/// **THE GAP:** this pins the producer descriptor's capture-count boundary and
/// its separation from the real zero/two consumer suffixes. It does not define
/// an ABI for producer captures, invent a capture input, or pin failures after
/// the declared fused call begins building.
///
/// **Promise class: durable invariant.** The mutation is downstream of exact
/// producer-descriptor selection and changes no source, claim, relation, or ABI
/// input.
#[test]
fn r3_fused_nonempty_producer_captures_refuse_before_emission() {
    use crate::cranelift_backend::lowering::core::D2fEmitterTestArm;
    use crate::cranelift_backend::planning::{
        d2j_checked_fixture_under, d2j_installed_plan_under,
        r3_fusion_claim_consumptions, reset_r3_fusion_claim_consumptions,
        with_fusion_producer_capture_mutation, D2jCause, FusionProducerCaptureMutation,
        D2J_DECLARATION,
    };

    fn continuation_capture_count(cause: D2jCause) -> u32 {
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let mut declarations = std::collections::BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        let plan = d2j_installed_plan_under(cause, &entry, &declarations, &oriented)
            .expect("the governed root installs its exact fusion plan");
        let views = plan
            .continuation_fusions()
            .expect("the installed fusion plane rejoins its ABI");
        assert_eq!(views.len(), 1, "one governed root installs one fusion");
        views[0].header().captures
    }

    fn compile(
        cause: D2jCause,
        mutation: FusionProducerCaptureMutation,
        symbol: &str,
    ) -> (Option<CraneliftBackendError>, usize, usize) {
        reset_r3_fusion_claim_consumptions();
        crate::cranelift_backend::lowering::reset_r3_fused_invocations();
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let mut declarations = std::collections::BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        let error = with_fusion_producer_capture_mutation(mutation, || {
            let _arm = D2fEmitterTestArm::arm();
            crate::cranelift_backend::lowering::core::compile_expr_into_object_module(
                crate::cranelift_backend::artifact::new_object_module_for_lowering_tests(
                    "ken-r3-producer-captures",
                )
                .expect("object module"),
                symbol,
                cranelift_module::Linkage::Export,
                &entry,
                &crate::NativeSeedEnvironment::empty(),
                declarations,
                None,
                false,
                None,
                None,
                Some(oriented),
            )
            .err()
        });
        (
            error,
            r3_fusion_claim_consumptions().len(),
            crate::cranelift_backend::lowering::r3_fused_invocations().len(),
        )
    }

    fn classify(error: &Option<CraneliftBackendError>) -> &'static str {
        match error {
            None => "completed",
            Some(CraneliftBackendError::Backend(BackendFailure::PlannerInvariant(reason)))
                if reason.contains("producer capture run is non-empty")
                    && reason.contains("continuation-input capture suffix") =>
            {
                "producer-capture refusal"
            }
            Some(_) => "other refusal",
        }
    }

    assert_eq!(
        (
            continuation_capture_count(D2jCause::Exact),
            continuation_capture_count(D2jCause::ReHomed),
        ),
        (2, 0),
        "Exact retains its two checked continuation captures while ReHomed is the real \
         zero-capture comparator; neither count is the producer-capture population"
    );

    let mut rows = Vec::new();
    for (cause, prefix) in R3_TERMINAL_STOP_POPULATION {
        for (mutation, suffix) in [
            (FusionProducerCaptureMutation::Exact, "exact"),
            (
                FusionProducerCaptureMutation::ForceNonEmptyAfterSelection,
                "nonempty-producer-captures",
            ),
        ] {
            let symbol = format!("ken_r3_producer_captures_{prefix}_{suffix}");
            let (error, consumptions, invocations) = compile(cause, mutation, &symbol);
            rows.push((cause, mutation, classify(&error), consumptions, invocations));
        }
    }

    assert_eq!(
        rows,
        vec![
            (
                D2jCause::Exact,
                FusionProducerCaptureMutation::Exact,
                "completed",
                1,
                1,
            ),
            (
                D2jCause::Exact,
                FusionProducerCaptureMutation::ForceNonEmptyAfterSelection,
                "producer-capture refusal",
                0,
                0,
            ),
            (
                D2jCause::ReHomed,
                FusionProducerCaptureMutation::Exact,
                "completed",
                1,
                1,
            ),
            (
                D2jCause::ReHomed,
                FusionProducerCaptureMutation::ForceNonEmptyAfterSelection,
                "producer-capture refusal",
                0,
                0,
            ),
        ],
        "both exact roots retain their real consumer-capture behaviour and complete once; a \
         non-empty producer-capture population refuses before either affine event"
    );
}

/// `RT-LEXICAL-R3-FUSION-EMITTER` `D3` -- a selected affine fusion claim is
/// settled only after its declared fused call has been accepted.
///
/// **MEASURED:** on both governed roots, withholding the last input only after
/// the real claim, target, ordinary parameter run and capture suffix have been
/// checked reaches the descriptor-driven call builder exactly once. That
/// builder refuses the real target's missing declared input with zero claim
/// consumptions and zero fused invocations. The unchanged operand runs are
/// accepted and then consume and invoke exactly once.
///
/// **CLAIMED:** a claim that is live at the fallible call-build boundary stays
/// outstanding when that build refuses; the affine settlement occurs only
/// after successful call emission.
///
/// **THE GAP:** the application counter proves this is a post-selection
/// call-build refusal, not an earlier selector or preflight failure. The
/// mutation changes only the real operand run handed to the real target; it
/// creates no source relation, claim, target, settlement or call instruction.
/// It does not prove the later post-field route remains absent.
///
/// **Promise class: durable invariant.** The exact error category, mutation
/// reach count and both affine events jointly pin the ordering without freezing
/// an origin literal or hand-building a ledger entry.
#[test]
fn r3_fused_late_call_build_refusal_keeps_claim_outstanding() {
    use crate::cranelift_backend::lowering::core::{
        with_d2f_call_build_mutation, D2fCallBuildMutation, D2fEmitterTestArm,
    };
    use crate::cranelift_backend::planning::{
        d2j_checked_fixture_under, r3_fusion_claim_consumptions,
        reset_r3_fusion_claim_consumptions, D2jCause, D2J_DECLARATION,
    };

    fn compile(
        cause: D2jCause,
        mutation: D2fCallBuildMutation,
        symbol: &str,
    ) -> (Option<CraneliftBackendError>, usize, usize, usize) {
        reset_r3_fusion_claim_consumptions();
        crate::cranelift_backend::lowering::reset_r3_fused_invocations();
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let mut declarations = std::collections::BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        let (error, applications) = with_d2f_call_build_mutation(mutation, || {
            let _arm = D2fEmitterTestArm::arm();
            crate::cranelift_backend::lowering::core::compile_expr_into_object_module(
                crate::cranelift_backend::artifact::new_object_module_for_lowering_tests(
                    "ken-r3-late-call-build",
                )
                .expect("object module"),
                symbol,
                cranelift_module::Linkage::Export,
                &entry,
                &crate::NativeSeedEnvironment::empty(),
                declarations,
                None,
                false,
                None,
                None,
                Some(oriented),
            )
            .err()
        });
        (
            error,
            applications,
            r3_fusion_claim_consumptions().len(),
            crate::cranelift_backend::lowering::r3_fused_invocations().len(),
        )
    }

    fn classify(error: &Option<CraneliftBackendError>) -> &'static str {
        match error {
            None => "completed",
            Some(CraneliftBackendError::Backend(BackendFailure::Module(reason)))
                if reason == "callee frame is missing a declared input" =>
            {
                "late call-build refusal"
            }
            Some(_) => "other refusal",
        }
    }

    let mut rows = Vec::new();
    for (cause, prefix) in R3_TERMINAL_STOP_POPULATION {
        for (mutation, suffix) in [
            (D2fCallBuildMutation::Exact, "exact"),
            (
                D2fCallBuildMutation::WithholdLastDeclaredInput,
                "missing-last-input",
            ),
        ] {
            let symbol = format!("ken_r3_late_call_build_{prefix}_{suffix}");
            let (error, applications, consumptions, invocations) =
                compile(cause, mutation, &symbol);
            rows.push((
                cause,
                mutation,
                classify(&error),
                applications,
                consumptions,
                invocations,
            ));
        }
    }

    assert_eq!(
        rows,
        vec![
            (
                D2jCause::Exact,
                D2fCallBuildMutation::Exact,
                "completed",
                0,
                1,
                1,
            ),
            (
                D2jCause::Exact,
                D2fCallBuildMutation::WithholdLastDeclaredInput,
                "late call-build refusal",
                1,
                0,
                0,
            ),
            (
                D2jCause::ReHomed,
                D2fCallBuildMutation::Exact,
                "completed",
                0,
                1,
                1,
            ),
            (
                D2jCause::ReHomed,
                D2fCallBuildMutation::WithholdLastDeclaredInput,
                "late call-build refusal",
                1,
                0,
                0,
            ),
        ],
        "both exact claims consume once only after accepted emission; both altered compiles reach \
         the real call builder exactly once and its late refusal leaves each claim outstanding"
    );
}

/// `RT-LEXICAL-R3-FUSION-EMITTER` `D3` -- the fusion-owned outer realization
/// lowers its selected body locally and never reintroduces a direct fused or R
/// specialization call at the post-field seam.
///
/// **MEASURED:** on both governed roots, the real local selected-body descent
/// reaches the claim's exact consuming call with its checked worker and ordered
/// operand run. Deferring only that already-validated call back to the outer
/// post-field seam reaches the named direct-call exclusion exactly once and
/// refuses with zero claim consumptions and zero fused invocations. The exact
/// route completes at one and one.
///
/// **CLAIMED:** the R post-field fork is non-calling. It may lower the selected
/// body locally, but only the exact consuming call reached inside that body may
/// emit and consume the fused invocation.
///
/// **THE GAP:** the mutation preserves the real claim, worker binding, visited
/// argument run, capture suffix and declared target by moving the completed
/// call preparation rather than rebuilding it. It does not prove the ordinary
/// O direct path, which never enters the outer-realization dispatcher and is
/// intentionally byte-identical.
///
/// **Promise class: durable invariant.** The assertion relates the named route
/// exclusion to the mutation's post-closure application count and both affine
/// events, without freezing an origin, target, arity or operand value.
#[test]
fn r3_fused_post_field_direct_call_reintroduction_refuses_before_emission() {
    use crate::cranelift_backend::lowering::core::{
        with_d2f_post_field_direct_call_mutation, D2fEmitterTestArm,
        D2fPostFieldDirectCallMutation,
    };
    use crate::cranelift_backend::planning::{
        d2j_checked_fixture_under, r3_fusion_claim_consumptions,
        reset_r3_fusion_claim_consumptions, D2jCause, D2J_DECLARATION,
    };

    fn compile(
        cause: D2jCause,
        mutation: D2fPostFieldDirectCallMutation,
        symbol: &str,
    ) -> (Option<CraneliftBackendError>, usize, usize, usize) {
        reset_r3_fusion_claim_consumptions();
        crate::cranelift_backend::lowering::reset_r3_fused_invocations();
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let mut declarations = std::collections::BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        let (error, applications) = with_d2f_post_field_direct_call_mutation(mutation, || {
            let _arm = D2fEmitterTestArm::arm();
            crate::cranelift_backend::lowering::core::compile_expr_into_object_module(
                crate::cranelift_backend::artifact::new_object_module_for_lowering_tests(
                    "ken-r3-post-field-direct-call",
                )
                .expect("object module"),
                symbol,
                cranelift_module::Linkage::Export,
                &entry,
                &crate::NativeSeedEnvironment::empty(),
                declarations,
                None,
                false,
                None,
                None,
                Some(oriented),
            )
            .err()
        });
        (
            error,
            applications,
            r3_fusion_claim_consumptions().len(),
            crate::cranelift_backend::lowering::r3_fused_invocations().len(),
        )
    }

    fn classify(error: &Option<CraneliftBackendError>) -> &'static str {
        match error {
            None => "completed",
            Some(CraneliftBackendError::Unsupported(UnsupportedLowering {
                construct,
                reason,
            })) if *construct == "StaticContinuationFusion"
                && reason.contains("reintroduced directly")
                && reason.contains("post-field seam") =>
            {
                "post-field direct-call refusal"
            }
            Some(_) => "other refusal",
        }
    }

    let mut rows = Vec::new();
    for (cause, prefix) in R3_TERMINAL_STOP_POPULATION {
        for (mutation, suffix) in [
            (D2fPostFieldDirectCallMutation::Exact, "exact"),
            (
                D2fPostFieldDirectCallMutation::ReintroduceDirectFusionCall,
                "direct-call",
            ),
        ] {
            let symbol = format!("ken_r3_post_field_direct_call_{prefix}_{suffix}");
            let (error, applications, consumptions, invocations) =
                compile(cause, mutation, &symbol);
            rows.push((
                cause,
                mutation,
                classify(&error),
                applications,
                consumptions,
                invocations,
            ));
        }
    }

    assert_eq!(
        rows,
        vec![
            (
                D2jCause::Exact,
                D2fPostFieldDirectCallMutation::Exact,
                "completed",
                0,
                1,
                1,
            ),
            (
                D2jCause::Exact,
                D2fPostFieldDirectCallMutation::ReintroduceDirectFusionCall,
                "post-field direct-call refusal",
                1,
                0,
                0,
            ),
            (
                D2jCause::ReHomed,
                D2fPostFieldDirectCallMutation::Exact,
                "completed",
                0,
                1,
                1,
            ),
            (
                D2jCause::ReHomed,
                D2fPostFieldDirectCallMutation::ReintroduceDirectFusionCall,
                "post-field direct-call refusal",
                1,
                0,
                0,
            ),
        ],
        "both exact roots consume and invoke once through the consuming Call inside the selected \
         body; deferring that same prepared call to the forbidden post-field seam reaches the \
         exclusion once and refuses before either affine event"
    );
}

/// `RT-LEXICAL-R3-FUSION-EMITTER` `D3` -- the outer realization may descend
/// only while its exact selected region claim remains outstanding after
/// identity and closure.
///
/// **MEASURED:** both governed roots select and close against their real claim.
/// Moving that same move-only claim out of the outstanding map immediately
/// afterward reaches the production outstanding detector once and refuses with
/// zero successful consumptions and zero fused invocations. Exact compilation
/// completes with one of each.
///
/// **CLAIMED:** opaque identity selection and closure do not license replay.
/// The claim must remain outstanding at the boundary where the R selector
/// enters its locally lowered body.
///
/// **THE GAP:** the mutation represents one lawful corrupted ledger state -- an
/// escaped selected claim. It does not separately instantiate duplicate storage
/// or a second attempted consumption, whose affine behavior belongs to the
/// ledger's own controls. It proves this real R selector refuses non-outstanding
/// state before emission, settlement or invocation.
///
/// **Promise class: durable invariant.** The assertion relates selected-claim
/// state to the selector boundary and affine events without freezing a fusion,
/// owner, body, call, projection, capture count or target coordinate.
#[test]
fn r3_fused_outer_selector_refuses_an_escaped_selected_claim() {
    use crate::cranelift_backend::lowering::core::{
        with_d2f_outer_claim_state_mutation, D2fEmitterTestArm,
        D2fOuterClaimStateMutation,
    };
    use crate::cranelift_backend::planning::{
        d2j_checked_fixture_under, r3_fusion_claim_consumptions,
        reset_r3_fusion_claim_consumptions, D2jCause, D2J_DECLARATION,
    };

    fn compile(
        cause: D2jCause,
        mutation: D2fOuterClaimStateMutation,
        symbol: &str,
    ) -> (Option<CraneliftBackendError>, usize, usize, usize) {
        reset_r3_fusion_claim_consumptions();
        crate::cranelift_backend::lowering::reset_r3_fused_invocations();
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let mut declarations = std::collections::BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        let (error, applications) = with_d2f_outer_claim_state_mutation(mutation, || {
            let _arm = D2fEmitterTestArm::arm();
            crate::cranelift_backend::lowering::core::compile_expr_into_object_module(
                crate::cranelift_backend::artifact::new_object_module_for_lowering_tests(
                    "ken-r3-outer-claim-state",
                )
                .expect("object module"),
                symbol,
                cranelift_module::Linkage::Export,
                &entry,
                &crate::NativeSeedEnvironment::empty(),
                declarations,
                None,
                false,
                None,
                None,
                Some(oriented),
            )
            .err()
        });
        (
            error,
            applications,
            r3_fusion_claim_consumptions().len(),
            crate::cranelift_backend::lowering::r3_fused_invocations().len(),
        )
    }

    fn classify(error: &Option<CraneliftBackendError>) -> &'static str {
        match error {
            None => "completed",
            Some(CraneliftBackendError::Unsupported(UnsupportedLowering {
                construct,
                reason,
            })) if *construct == "StaticContinuationFusion"
                && reason.contains("selected region claim")
                && reason.contains("no longer outstanding after closure") =>
            {
                "non-outstanding selected-claim refusal"
            }
            Some(_) => "other refusal",
        }
    }

    let mut rows = Vec::new();
    for (cause, prefix) in R3_TERMINAL_STOP_POPULATION {
        for (mutation, suffix) in [
            (D2fOuterClaimStateMutation::Exact, "exact"),
            (
                D2fOuterClaimStateMutation::EscapeAfterClosure,
                "escaped",
            ),
        ] {
            let symbol = format!("ken_r3_outer_claim_state_{prefix}_{suffix}");
            let (error, applications, consumptions, invocations) =
                compile(cause, mutation, &symbol);
            rows.push((
                cause,
                mutation,
                classify(&error),
                applications,
                consumptions,
                invocations,
            ));
        }
    }

    assert_eq!(
        rows,
        vec![
            (
                D2jCause::Exact,
                D2fOuterClaimStateMutation::Exact,
                "completed",
                0,
                1,
                1,
            ),
            (
                D2jCause::Exact,
                D2fOuterClaimStateMutation::EscapeAfterClosure,
                "non-outstanding selected-claim refusal",
                1,
                0,
                0,
            ),
            (
                D2jCause::ReHomed,
                D2fOuterClaimStateMutation::Exact,
                "completed",
                0,
                1,
                1,
            ),
            (
                D2jCause::ReHomed,
                D2fOuterClaimStateMutation::EscapeAfterClosure,
                "non-outstanding selected-claim refusal",
                1,
                0,
                0,
            ),
        ],
        "both exact roots consume and invoke once; escaping the same selected claim after closure \
         reaches the independent outstanding detector once and refuses before either affine event"
    );
}

/// `RT-LEXICAL-R3-FUSION-EMITTER` `D3` -- the fused capture suffix must be the
/// claim's exact ordered projection through the real entry ABI.
///
/// **MEASURED:** Exact reaches one two-member `claim.inputs()` projection and
/// one two-member entry-ABI result. Dropping its last projected member,
/// duplicating its first, swapping its two, or replacing its first with the
/// second real entry-ABI source reaches the production integrity detector once
/// and refuses with zero claim consumption and zero fused invocation. ReHomed
/// reaches the same seam with a measured `(0, 0)` projection and completes
/// with one consumption and one invocation.
///
/// **CLAIMED:** after opaque selection and identity closure, the call's capture
/// suffix is exactly `claim.inputs()` in source order -- no missing, repeated,
/// transposed or differently sourced member may reach call emission.
///
/// **THE GAP:** the source-derived row reuses another member of Exact's real
/// entry-ABI projection; it creates no capture authority, claim, ABI operand or
/// source relation. No governed claim contains a `ProducerLocal` coordinate, so
/// this control does not manufacture one to widen the population. ReHomed is a
/// zero-capture comparator and therefore cannot itself discriminate a suffix
/// mutation; its measured empty projection plus successful affine pair is the
/// control that keeps that limitation visible.
///
/// **Promise class: durable invariant.** The assertion relates the claim's
/// ordered source authorities to the suffix actually presented at the fused
/// call, without freezing owner ids, ABI positions, carriers or operand values.
#[test]
fn r3_fused_capture_projection_refuses_before_emission() {
    use crate::cranelift_backend::lowering::core::{
        with_d2f_capture_projection_mutation, D2fCaptureProjectionMutation,
        D2fEmitterTestArm,
    };
    use crate::cranelift_backend::planning::{
        d2j_checked_fixture_under, r3_fusion_claim_consumptions,
        reset_r3_fusion_claim_consumptions, D2jCause, D2J_DECLARATION,
    };

    fn compile(
        cause: D2jCause,
        mutation: D2fCaptureProjectionMutation,
        symbol: &str,
    ) -> (Option<CraneliftBackendError>, usize, Vec<(usize, usize)>, usize, usize) {
        reset_r3_fusion_claim_consumptions();
        crate::cranelift_backend::lowering::reset_r3_fused_invocations();
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let mut declarations = std::collections::BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        let (error, applications, populations) =
            with_d2f_capture_projection_mutation(mutation, || {
                let _arm = D2fEmitterTestArm::arm();
                crate::cranelift_backend::lowering::core::compile_expr_into_object_module(
                    crate::cranelift_backend::artifact::new_object_module_for_lowering_tests(
                        "ken-r3-capture-projection",
                    )
                    .expect("object module"),
                    symbol,
                    cranelift_module::Linkage::Export,
                    &entry,
                    &crate::NativeSeedEnvironment::empty(),
                    declarations,
                    None,
                    false,
                    None,
                    None,
                    Some(oriented),
                )
                .err()
            });
        (
            error,
            applications,
            populations,
            r3_fusion_claim_consumptions().len(),
            crate::cranelift_backend::lowering::r3_fused_invocations().len(),
        )
    }

    fn classify(error: &Option<CraneliftBackendError>) -> &'static str {
        match error {
            None => "completed",
            Some(CraneliftBackendError::Unsupported(UnsupportedLowering {
                construct,
                reason,
            })) if *construct == "StaticContinuationFusion"
                && reason.contains("capture suffix")
                && reason.contains("exact ordered input projection") =>
            {
                "capture-integrity refusal"
            }
            Some(_) => "other refusal",
        }
    }

    let mut rows = Vec::new();
    for (cause, prefix) in R3_TERMINAL_STOP_POPULATION {
        // Exact owns the non-empty two-capture projection and therefore the
        // four lawful mutations. ReHomed is the zero-capture comparator, so
        // only its exact arm has a subject.
        let mutations = if cause == D2jCause::Exact {
            vec![
                (D2fCaptureProjectionMutation::Exact, "exact"),
                (D2fCaptureProjectionMutation::DropLast, "dropped"),
                (D2fCaptureProjectionMutation::DuplicateFirst, "duplicated"),
                (D2fCaptureProjectionMutation::SwapFirstTwo, "swapped"),
                (
                    D2fCaptureProjectionMutation::UseSecondSourceForFirst,
                    "source-derived",
                ),
            ]
        } else {
            vec![(D2fCaptureProjectionMutation::Exact, "exact")]
        };
        for (mutation, suffix) in mutations {
            let symbol = format!("ken_r3_capture_projection_{prefix}_{suffix}");
            let (error, applications, populations, consumptions, invocations) =
                compile(cause, mutation, &symbol);
            rows.push((
                cause,
                mutation,
                classify(&error),
                applications,
                populations,
                consumptions,
                invocations,
            ));
        }
    }

    assert_eq!(
        rows,
        vec![
            (
                D2jCause::Exact,
                D2fCaptureProjectionMutation::Exact,
                "completed",
                0,
                vec![(2, 2)],
                1,
                1,
            ),
            (
                D2jCause::Exact,
                D2fCaptureProjectionMutation::DropLast,
                "capture-integrity refusal",
                1,
                vec![(2, 2)],
                0,
                0,
            ),
            (
                D2jCause::Exact,
                D2fCaptureProjectionMutation::DuplicateFirst,
                "capture-integrity refusal",
                1,
                vec![(2, 2)],
                0,
                0,
            ),
            (
                D2jCause::Exact,
                D2fCaptureProjectionMutation::SwapFirstTwo,
                "capture-integrity refusal",
                1,
                vec![(2, 2)],
                0,
                0,
            ),
            (
                D2jCause::Exact,
                D2fCaptureProjectionMutation::UseSecondSourceForFirst,
                "capture-integrity refusal",
                1,
                vec![(2, 2)],
                0,
                0,
            ),
            (
                D2jCause::ReHomed,
                D2fCaptureProjectionMutation::Exact,
                "completed",
                0,
                vec![(0, 0)],
                1,
                1,
            ),
        ],
        "Exact's real two-capture suffix is accepted only in claim order; every alteration \
         reaches the independent integrity detector before either affine event, while ReHomed \
         proves the same seam is a real zero-capture comparator"
    );
}

/// `RT-LEXICAL-R3-FUSION-EMITTER` `D3` -- both terminal-stop fused selectors
/// must reach the production target-authority validator before call emission.
///
/// **MEASURED:** the existing armed Exact and ReHomed full-pipeline compiles
/// each reach `fusion_target_carries_claim_authority`, then complete with one
/// claim consumption and one fused invocation. Armed `ProducerArity` refuses
/// earlier at its widened producer construct, with no validator reach,
/// consumption, or invocation.
///
/// **CLAIMED:** the validator is wired into the complete currently governed
/// terminal-stop population. That population is defined by reaching this
/// validator and the later terminal stop: `Exact` and `ReHomed`. `ProducerArity`
/// refuses earlier, as recorded beside `D2F_EMITTER_ARMED` in `core.rs`, and is
/// not a member. Bypassing the validator call must make this control red even
/// though both compiles otherwise still succeed.
///
/// **THE GAP:** the sole writer currently derives the map key and both checked
/// fields from the same claim. This control proves validator reachability, not
/// wrong-target refusal; the source-site future-divergence comment names the
/// type and writer changes that would make that separate relation expressible.
///
/// **Promise class: durable invariant.** Every selector in the reachable
/// terminal-stop population must retain the target-authority validation
/// independently of how many selectors or claims an intended extension adds.
#[test]
fn r3_fused_target_authority_validator_is_wired_to_both_real_selectors() {
    use crate::cranelift_backend::lowering::core::{
        observe_d2f_target_authority_validation, D2fEmitterTestArm,
    };
    use crate::cranelift_backend::planning::{
        d2j_checked_fixture_under, r3_fusion_claim_consumptions,
        reset_r3_fusion_claim_consumptions, D2jCause, D2J_DECLARATION,
    };

    fn compile(
        cause: D2jCause,
        symbol: &str,
    ) -> (Option<CraneliftBackendError>, usize, usize, usize) {
        reset_r3_fusion_claim_consumptions();
        crate::cranelift_backend::lowering::reset_r3_fused_invocations();
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let mut declarations = std::collections::BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        let (error, validations) = observe_d2f_target_authority_validation(|| {
            let _arm = D2fEmitterTestArm::arm();
            crate::cranelift_backend::lowering::core::compile_expr_into_object_module(
                crate::cranelift_backend::artifact::new_object_module_for_lowering_tests(
                    "ken-r3-target-authority-validation",
                )
                .expect("object module"),
                symbol,
                cranelift_module::Linkage::Export,
                &entry,
                &crate::NativeSeedEnvironment::empty(),
                declarations,
                None,
                false,
                None,
                None,
                Some(oriented),
            )
            .err()
        });
        (
            error,
            validations,
            r3_fusion_claim_consumptions().len(),
            crate::cranelift_backend::lowering::r3_fused_invocations().len(),
        )
    }

    for (cause, prefix) in R3_TERMINAL_STOP_POPULATION {
        let symbol = format!("ken_r3_target_authority_{prefix}");
        let (error, validations, consumptions, invocations) = compile(cause, &symbol);
        assert!(error.is_none(), "{cause:?}: {error:?}");
        assert!(
            validations > 0,
            "{cause:?}: the real fused selector bypassed target-authority validation"
        );
        assert_eq!(
            (consumptions, invocations),
            (1, 1),
            "{cause:?}: validator reach is meaningful only on an accepted affine call"
        );
    }

    let (error, validations, consumptions, invocations) = compile(
        D2jCause::ProducerArity,
        "ken_r3_target_authority_producer_arity",
    );
    assert!(
        matches!(
            &error,
            Some(CraneliftBackendError::Unsupported(UnsupportedLowering {
                construct,
                reason,
            })) if *construct == "ComputationalMatch" && reason.contains(
                "case ctor:fixture::D2gOut::Node expects 1 constructor arguments but value has 2"
            )
        ),
        "ProducerArity must retain its own earlier widened-producer refusal: {error:?}"
    );
    assert_eq!(
        (validations, consumptions, invocations),
        (0, 0, 0),
        "ProducerArity is outside the terminal-stop population only while its earlier refusal \
         prevents validator reach and both affine events"
    );
}

/// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — ONE RECOGNIZED SOURCE FIELD, ONE
/// TRANSPORT, TWO AUTHORIZED BINDER PROJECTIONS.** Architect
/// `evt_37715knv356yp`, control 1.
///
/// The ledger's own laws — a second consumption of one transport refuses at the
/// call, an unconsumed transport refuses at close, a direct binding pays no
/// debt, and neither link crosses a scope — are all pinned by
/// `d2k_1c_0_conservation_pairs_each_consumption_to_one_minted_transport`, which
/// exercises `StaticWorkerFieldLedger` directly. **This control does not restate
/// them.** It pins the one thing that ledger control cannot see: that the two
/// members the composed eliminator assembles are projections of the **same**
/// obligation rather than two obligations that happen to name one worker body.
///
/// **Why it has to be measured here and cannot be inferred from the green
/// compile.** A compile that completes proves *some* consumption discharged the
/// transport. It does not distinguish the ruled shape from two others that also
/// complete: one member carrying no transport while the other is consumed, or
/// two transports minted with both consumed. Only reading the members' transport
/// identities out of one assembled run separates them.
///
/// **The identity cannot be forged and cannot be recovered any other way.**
/// `StaticWorkerTransportId` has a private field in a module exposing only its
/// issuers, so the equality asserted below is between two values the ledger
/// minted, not between two values this test built.
///
/// **MEASURED, by restoring the superseded sub-rule in production:** setting the
/// hypothesis back to `transport: None` reds this control and
/// `d2f_armed_compile_completes_and_its_populations_are_pinned` together, with
/// the ledger's own *"rebound ... and never consumed"* refusal as the evidence.
/// Reverted; both green.
///
/// **THE GAP, and it is about this control's own rows.** That mutation reds at
/// the **completion guard**, not at the two transport assertions — the compile
/// stops before they are reached. So on this witness the rows that actually
/// discriminate are completion, the run's length, and the slot order; the
/// `is_some` and the equality are **stated but not independently falsifiable**:
///
/// - a differing pair would need a second `rebind` behind one recognition, which
///   the ledger refuses outright — the shape is unrepresentable rather than
///   merely untested;
/// - a `None` hypothesis that still completes needs a case body reaching its
///   producer through the *selected argument* instead of the hypothesis, which
///   is control 2 of the ruling and needs a checked fixture this witness does
///   not provide.
///
/// **The obvious candidate for control 2 was tried and does NOT serve, and this
/// is a measurement rather than a guess.** `D2jCause::CallIdentity` is described
/// as *"the consuming `Call` calls the ordinary child instead of the
/// hypothesis"*, which is control 2's shape exactly. Compiled armed, it refuses
/// at `ComputationalMatch: "a computational recursor closure names an in-flight
/// activation, not a transferable value"` and records **no** binder run at all —
/// so it is declined by a predecessor guard and never reaches this seat, rather
/// than reaching it and disagreeing. It is a planner-defect cause, not a lawful
/// alternative program. ⇒ Control 2 needs a genuinely different checked body,
/// authored rather than selected.
///
/// **AUTHORING IT WAS ATTEMPTED, AND IT IS A DEEPER CHANGE THAN IT LOOKS.** A
/// `SelectedArgumentCall` cause was built as `CallIdentity`'s positive twin —
/// the identical source rewrite (callee `Var(1)`), but with its oriented plan
/// derived from its OWN body, which is exactly the treatment that makes
/// `ProducerArity` a witness rather than a refusal. It compiled, and it still
/// refuses at the same `"names an in-flight activation"` guard with **no**
/// binder run recorded — declined upstream just as `CallIdentity` is. The
/// attempt was reverted rather than left in the tree as a variant that reads
/// like a working witness.
///
/// ⇒ **Plan derivation was not the obstacle, so the remaining one is the CHECKED
/// STRUCTURE** — and reading the real checker settles what that structure is.
///
/// **THE ANSWER, from `ken_elaborator::erasure`, and it makes control 2
/// EXPRESSIBLE rather than blocked.** `computational_ih_application_spine`
/// returns `Some` **only when the application head's de Bruijn index maps to a
/// computational IH slot** (`branch_remap.computational_ih_slot(index)`). A head
/// naming the *constructor-field* binder has no slot, so the recognizer answers
/// `None`, no `consume_computational_ih_call` is recorded, and **no
/// `CheckedComputationalIHInvocation` marker is emitted around that call at
/// all**.
///
/// ⇒ **A lawful selected-argument-consuming body simply HAS NO IH-invocation
/// marker there**, and both attempts above were self-contradictory by
/// construction: each kept the marker — which asserts an IH invocation — while
/// pointing the callee at a binder that is not the hypothesis. That is exactly
/// the disagreement `CallIdentity` exists to be, which is why deriving the plan
/// from its own body changed nothing. The *source* still carried a marker the
/// checker would never have put there.
///
/// **This is also precisely why the fixture must come from the real checker**
/// (Steward scope ruling): hand-building it would have let the marker simply be
/// deleted here, which **asserts** that the checker omits it instead of
/// **testing** that it does. The route is the `ken-cli` integration harness that
/// `px7l_checked_host_recursive_bind` and `px8l_recursive_decl_native` already
/// establish — real Ken source through the ordinary front end — not another
/// `D2jCause`. Its entry point is
/// `ken_cli::build_native_program(source, SourceFormat::Ken, name, &dir)` on an
/// inline source string, so the witness is authored as Ken and the checked form
/// is whatever the front end makes of it.
///
/// **The shape that source has to reach** is a computational match over a
/// datatype whose recursive field transports a static worker, with the case body
/// APPLYING that field rather than the hypothesis. Whether Ken's surface admits
/// such a program at all is the open question, and if it does not, that is the
/// result to report — the fixture must not be weakened to make this control
/// runnable, because a control armed against a shape the checker never emits is
/// worth less than an honest gap.
///
/// **REFUTED, 2026-08-13, and the refutation leads because that is where a
/// withdrawn claim otherwise survives.** The paragraph below once read *"the
/// ordinary front end does not emit a `ComputationalMatch` at all"*. That is
/// **false**. Architect `evt_2gzjt1zqy402z`'s bounded producer probe measured
/// `NESTED_LIFT_NAT_THREE_SOURCE` — real source, ordinary front end — producing
/// a **retained generated lifted-family `ComputationalMatch` in erased Runtime
/// IR**, `recursive_positions = [2, 3]`, under both `Executable` and `Library`
/// selection. Surface Ken **does** have an erased-IR producer, via the
/// nested-result selector path (`recursive result for xs/ys`), which none of the
/// eight programs below exercised.
///
/// **The only statement the table supports, and the only one to quote:** these
/// eight *sampled* programs censused zero, and every then-current positive
/// control at this seat was synthetic. Both halves are still true. Neither
/// licenses a claim about the front end's whole output — the sample had no
/// nested inductive with an explicit selector in it, so it could not have seen
/// the producer that exists.
///
/// **THE SAMPLE, retained because it is still the evidence for what it covers.**
/// Measured through `ken_cli::build_native_program` on real Ken source,
/// censusing the checker's own erased output (`runtime_program.declarations`,
/// whose only body-bearing kind is `Transparent`, so the census is not partial):
///
/// | real-source program | `ComputationalMatch` | outcome |
/// |---|---|---|
/// | `data Branch = Tip \| Fork (Bool -> Branch)`, `Fork k` case applies `k` | 0 | BUILT |
/// | first-order `Link rest` case, `Suc (depth rest)` | 0 | BUILT |
/// | effectful `Link rest \|-> walk rest` (self-call present) | 0 | BUILT |
/// | effectful `Link rest` case, field only, no self-call | 0 | BUILT |
/// | `px7p`'s own PROGRAM (a GREEN integration test today) | 0 | BUILT |
/// | `px8l`/`px7m`/`px7l` PROGRAMs | — | refuse at object emission on the
/// |   |   | pre-existing `RT-CLOSURE-BOUNDARY-LANE` / `RT-CARRIER-BYTESPAN-OBSERVE`
/// |   |   | debt, which is why those rows are `#[ignore]`d |
///
/// **The mechanism reading agrees with the measurement, and it is one
/// predicate.** `checked_match_uses_computational_recursive_hypothesis`
/// (`ken_elaborator::checked_core`) answers `true` only when a branch body
/// **references the IH binder range** — `runtime_body_references_outer_binder_
/// range(body, 0, recursive_count, 0)`. A surface self-call erases to a
/// `RecursiveDeclarationCall`, **not** to a reference to that range, which is
/// why even the self-calling positive above censused 0. So:
///
/// ⇒ **The property that DEFINES control 2 — a body that uses the selected
/// recursive argument and never the hypothesis — is the very property that makes
/// this predicate answer `false`.** A selected-argument-only body is classified
/// as an ordinary `Match` and never reaches the composed eliminator seat at all.
/// Control 2 is not merely hard to author from source; **as stated it is
/// self-defeating at the classifier**, one gate above anything this file tests.
///
/// **CORRECTION to the paragraph above, Architect `evt_7mg1x1vqe7qph`: the
/// predicate is EXISTENTIAL OVER BRANCHES, not universal.** The loop
/// `return Ok(true)`s on the **first** branch that references its IH binder
/// range, so an *entirely* selected-argument-only match is ordinary, while a
/// selected-argument-only **branch** is lawful inside a `ComputationalMatch`
/// that some *other* branch has earned. The sentence above says "a body" where
/// the code says "any branch"; read it as scoped to the whole-match case.
///
/// **The prescribed multi-branch construction was then built and measured, and
/// it is blocked by a DIFFERENT fact than the one this row first reported.** A
/// three-constructor `Tree` with one self-calling branch and one field-only
/// branch — the ruled shape exactly — still censuses **0** computational
/// matches. The decisive detail is that nothing was folded away: the erased
/// program retains **two** declarations, `main` and `size`, and `size` carries
/// **3 ordinary matches, 0 computational, 0 recursive positions**. The
/// self-calling branch is present and is still not IH-referencing.
///
/// ⇒ **The multi-branch route needs one IH-referencing branch to earn the
/// classification, and neither mechanism below supplies one.** Two independent
/// mechanisms, each measured:
///
/// 1. **Surface recursion is named general recursion checked by SCT, not
///    eliminator compilation.** A self-call erases to a
///    `RecursiveDeclarationCall` — the same self-reference `px8l`'s own row
///    asserts is retained — and never to a reference into the IH binder range.
/// 2. **The prelude's hand-built `ITree` eliminator never reaches erasure.** A
///    `bind`-using program erases to **exactly one declaration, `main`**, with
///    zero matches of any kind: `bind` is specialized away at the checked-host
///    boundary, so the one eliminator in the system that *does* carry IH binders
///    is not in the erased IR to be matched on.
///
/// **This is reported, not routed around.** The existential refinement is
/// correct and it is not what was blocking. Naming which claim moved matters
/// more than the negative: the first report's *reason* was too broad.
///
/// ⛔ **AND THE SENTENCE THAT ONCE CLOSED THIS PARAGRAPH IS ALSO REFUTED.** It
/// read *"the blocker is that the surface has no producer of an IH-binder
/// reference at all"*. There is one. Architect `evt_2gzjt1zqy402z`'s probe
/// measured branch `ctor_577` of the generated lifted family, `argument_count =
/// 4`, `recursive_positions = [2, 3]`, whose peeled body's variable occurrences
/// are `[1, 0]` — **both inside the recursive range**, produced by the two
/// `recursive result for xs/ys` selectors.
///
/// ⇒ **The two mechanisms above remain exactly true of what they name** —
/// SCT-checked self-recursion, and the specialized-away `ITree` eliminator — and
/// they were never the whole population. The nested-result selector path is a
/// third mechanism neither of them covers, and it is the producer. **A list of
/// two measured mechanisms was written as though it were a closed enumeration;
/// that is the error to learn from here, and it is the second time on this row
/// that a sample was reported in the voice of a population.**
///
/// **THE GAP, stated as its own sentence.** MEASURED: eight real-source programs,
/// including two of this repo's own integration programs, produce zero
/// `ComputationalMatch` nodes. CLAIMED, and NOT proven: that *no* Ken program
/// can. The first gate above the predicate —
/// `validate_supported_match_motive`, which sets
/// `computational_recursive_hypotheses` — **can** be satisfied from surface
/// source, so the closure argument is about the second gate only, and it rests
/// on eight witnesses plus one predicate read, not on an enumeration of the
/// surface.
///
/// **What this says about the node, and it is larger than control 2.** Every
/// control that reaches this seat — control 1 included, green above — does so
/// through a `D2jCause` synthetic fixture. That much is measured rather than
/// suspected, and it is the `D0` history the scope ruling cited, observed
/// directly instead of inherited.
///
/// ⛔ **The REASON once given here was wrong and is withdrawn.** This paragraph
/// read *"the front end does not produce the shape"*. It does produce it — see
/// the refutation at the head of
/// [`d2f_the_two_binder_projections_share_one_source_field_transport`]. So the
/// synthetic-fixture fact stands, its explanation does not, and **why** these
/// rows are synthetic is now open rather than answered: a real-source producer
/// exists and no row here uses it. Read this as a debt with a known payer, not
/// as a closed boundary.
///
/// **What the same probe DID establish**, on both roots, read directly out of
/// the recorded run: `[(0, Some(TransportId(0))), (1, Some(TransportId(0)))]` —
/// the two members carry one transport, which is the ruled coordinate observed
/// rather than inferred from the compile completing.
///
/// They are written anyway, because an invariant that holds by construction
/// today is exactly the one a later change silently breaks — but nobody should
/// read them as exercised.
///
/// **Promise class: durable invariant.** It is the sharing relation itself, not
/// a count and not the current stop: any extension that keeps one source-field
/// obligation behind the paired projections keeps this green, and collapsing to
/// a per-member transport — the sub-rule `evt_37715knv356yp` superseded — reds
/// it immediately.
///
/// Production stays unarmed; the arm is the `cfg(test)` RAII `D2fEmitterTestArm`.
#[test]
fn d2f_the_two_binder_projections_share_one_source_field_transport() {
    use crate::cranelift_backend::lowering::core::D2fEmitterTestArm;
    use crate::cranelift_backend::planning::{d2j_checked_fixture_under, D2jCause};

    for (cause, symbol) in [
        (D2jCause::Exact, "ken_d2f_shared_transport_exact"),
        (D2jCause::ReHomed, "ken_d2f_shared_transport_rehomed"),
    ] {
        crate::cranelift_backend::lowering::reset_r3_run_worker_members();
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let mut declarations = std::collections::BTreeMap::new();
        declarations.insert(
            crate::cranelift_backend::planning::D2J_DECLARATION,
            &declaration,
        );
        let error = {
            let _arm = D2fEmitterTestArm::arm();
            crate::cranelift_backend::lowering::core::compile_expr_into_object_module(
                crate::cranelift_backend::artifact::new_object_module_for_lowering_tests(
                    "ken-d2f-shared-transport",
                )
                .expect("object module"),
                symbol,
                cranelift_module::Linkage::Export,
                &entry,
                &crate::NativeSeedEnvironment::empty(),
                declarations,
                None,
                false,
                None,
                None,
                Some(oriented),
            )
            .err()
        };
        assert!(
            error.is_none(),
            "{cause:?}: the armed compile must complete, or the run recorded below is the run of \
             a compile that stopped somewhere and the sharing claim is about nothing: {error:?}"
        );

        // The runs that actually carry a worker member. A composed eliminator
        // over an ordinary case assembles none, and those rows are silent here
        // rather than being asserted about.
        let carrying: Vec<_> = crate::cranelift_backend::lowering::r3_run_worker_members()
            .into_iter()
            .filter(|run| !run.is_empty())
            .collect();
        assert_eq!(
            carrying.len(),
            1,
            "{cause:?}: exactly one assembled case run carries static-worker members on this \
             witness. More than one and the equality below would be averaging over runs; none \
             and it would be vacuous: {carrying:?}"
        );
        let run = &carrying[0];
        assert_eq!(
            run.len(),
            2,
            "{cause:?}: the ruled run holds exactly TWO worker members -- the induction \
             hypothesis and the selected recursive argument. One member is the `D6a` skip shape, \
             which shifts every later binder; three is a second projection nobody authorized: \
             {run:?}"
        );
        assert_eq!(
            (run[0].0, run[1].0),
            (0, 1),
            "{cause:?}: the hypothesis leads the run and the constructor argument follows it, \
             per CheckedCaseBinderLayout: {run:?}"
        );
        assert!(
            run[0].1.is_some(),
            "{cause:?}: the HYPOTHESIS must carry a transport. `None` here is exactly the \
             superseded sub-rule of evt_5yhm9c78dm27s, under which the body's own recursive call \
             discharged nothing and the close refused: {run:?}"
        );
        assert_eq!(
            run[0].1, run[1].1,
            "{cause:?}: BOTH projections must carry the SAME transport. One recognized source \
             field is one obligation; two distinct transports would be two obligations behind one \
             recognition, which the close refuses, and a consumption through either member must \
             discharge the one debt: {run:?}"
        );
    }
}

#[test]
fn d2f_0_the_applied_root_production_path_gate() {
    use crate::cranelift_backend::lowering::core::d2f_gate_arrivals_take;
    use crate::cranelift_backend::planning::{d2j_checked_fixture_under, D2jCause};

    /// One cause, compiled through the production entry on its own root.
    fn compile_cause(
        cause: D2jCause,
        symbol: &str,
    ) -> (
        Vec<crate::cranelift_backend::lowering::core::D2fGateArrival>,
        Option<CraneliftBackendError>,
    ) {
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let _ = crate::cranelift_backend::lowering::core::d2f_gate_arrivals_take();
        let mut declarations = std::collections::BTreeMap::new();
        declarations.insert(
            crate::cranelift_backend::planning::D2J_DECLARATION,
            &declaration,
        );
        let result = crate::cranelift_backend::lowering::core::compile_expr_into_object_module(
            crate::cranelift_backend::artifact::new_object_module_for_lowering_tests("ken-d2f-gate")
                .expect("object module"),
            symbol,
            cranelift_module::Linkage::Export,
            &entry,
            &crate::NativeSeedEnvironment::empty(),
            declarations,
            None,
            false,
            None,
            None,
            Some(oriented),
        );
        (
            crate::cranelift_backend::lowering::core::d2f_gate_arrivals_take(),
            result.err(),
        )
    }

    /// Compile one cause inside the existing entry census. The census and the
    /// D2f arrival recorder are separate thread locals: `compile_cause` drains
    /// only the latter, while this scope restores only the former.
    fn compile_cause_with_census(
        cause: D2jCause,
        symbol: &str,
    ) -> (
        Vec<crate::cranelift_backend::lowering::core::D2fGateArrival>,
        Option<CraneliftBackendError>,
        Vec<crate::cranelift_backend::lowering::core::MatchRecursorCensusRow>,
    ) {
        let ((arrivals, error), rows) =
            crate::cranelift_backend::lowering::core::with_match_recursor_census(|| {
                compile_cause(cause, symbol)
            });
        (arrivals, error, rows)
    }

    /// The planner comparator for one cause, on that cause's OWN root. This is
    /// a DIFFERENT derivation from the compile above; neither reads the other,
    /// which is the only reason their agreement says anything.
    fn planner_plane(
        cause: D2jCause,
    ) -> crate::cranelift_backend::planning::StaticContinuationFusionPlan {
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let mut declarations = std::collections::BTreeMap::new();
        declarations.insert(
            crate::cranelift_backend::planning::D2J_DECLARATION,
            &declaration,
        );
        let plan = crate::cranelift_backend::planning::plan_static_transition_graph(
            &entry,
            &declarations,
        )
        .expect("the cause-selected root plans");
        crate::cranelift_backend::planning::build_static_continuation_fusion_plan(
            &plan,
            &entry,
            &declarations,
            Some(&oriented),
        )
        .expect("the cause-selected root resolves a plane")
    }

    let ordinary_refusal = |error: &Option<CraneliftBackendError>, side: &str| {
        assert!(
            matches!(
                error,
                Some(CraneliftBackendError::Unsupported(UnsupportedLowering { construct, reason }))
                    if *construct == "ComputationalMatch"
                        && reason.contains("in-flight activation")
            ),
            "{side}: before emission this root must reach the twin's OWN ordinary refusal -- \
             that refusal is the thing fusion is supposed to eliminate, so it is the baseline \
             the later 0 -> 1 movement is measured against: {error:?}"
        );
    };

    // ---- the three planning positives, each on its own root.
    //
    // `D2k` `AC-1a` extends this gate to the THIRD positive. `ProducerArity` is
    // a positive for this builder/planner population: it reaches the builder and
    // resolves its own key and descriptor. It is not a successful-lowering or
    // terminal-stop positive. Armed lowering refuses earlier at its widened
    // producer construct; the authoritative population and refusal account is
    // beside `D2F_EMITTER_ARMED` in `core.rs`. The executable assertions below
    // measure only planning/build arrival and the unarmed baseline.
    let (exact_arrivals, exact_error) = compile_cause(D2jCause::Exact, "ken_d2f_gate_exact");
    let (rehomed_arrivals, rehomed_error) = compile_cause(D2jCause::ReHomed, "ken_d2f_gate_rehomed");
    let (arity_arrivals, arity_error) =
        compile_cause(D2jCause::ProducerArity, "ken_d2f_gate_arity");
    let exact = match exact_arrivals.as_slice() {
        [only] => only.clone(),
        other => panic!("the applied exact root must reach the builder once: {}", other.len()),
    };
    let rehomed = match rehomed_arrivals.as_slice() {
        [only] => only.clone(),
        other => panic!("the bare re-homed root must reach the builder once: {}", other.len()),
    };
    let arity = match arity_arrivals.as_slice() {
        [only] => only.clone(),
        other => panic!(
            "the widened-arity root must reach the builder once: {}",
            other.len()
        ),
    };

    // ---- the old seed witness, which carries no marker at all.
    let seed_expr = host_result_closure_match(px8j_equal_payload_hole_placement(
        Px8jSelectedScopePlacement::BeforeReturnHole,
    ));
    let _ = d2f_gate_arrivals_take();
    let (seed_result, _trace) = px8j_capture_source_trace(&seed_expr, false, "ken_d2f_gate_seed");
    assert!(matches!(
        seed_result,
        Err(CraneliftBackendError::Unsupported(UnsupportedLowering {
            construct: "ComputationalMatch",
            reason,
        })) if reason == "a computational recursor closure names an in-flight activation, not a transferable value"
    ), "the seed must reach the builder before its measured D8 refusal");
    let seed = match d2f_gate_arrivals_take().as_slice() {
        [only] => only.clone(),
        other => panic!("the seed witness must reach the builder once: {}", other.len()),
    };

    // ---- AC-6a, per row, BESIDE the non-empty positive.
    //
    // Every number in the refusal rows is a zero and no zero proves anything by
    // itself, so the positives' populations are operands of the same assertion.
    // `D2k-1e`: the count is supplementary; the census phase and refusal
    // SENTENCE carry these rows together. `compile_cause` already returns the
    // error; the previous revision dropped it with `.0` and kept a bare zero,
    // which is precisely the shape that cannot tell "the validator refused"
    // from "the compile failed earlier for an unrelated reason" -- the
    // standard `d0_r3_...` states and
    // `d2j_the_source_side_causes_refuse_before_any_id_exists`
    // applies. Each row now names the authority that refused it and reads the
    // census bit written from the OUTER validator, so the same error from the
    // builder's own validator cannot silently satisfy the phase claim.
    //
    // The expected sentences are the ones the transport validator itself emits;
    // the two occurrence-mismatch reasons continue past this prefix with the
    // declaration they were raised in, so `contains` is the right relation.
    let no_arrival: Vec<(
        D2jCause,
        usize,
        bool,
        bool,
        Option<CraneliftBackendError>,
    )> = [
        (
            D2jCause::Frame,
            "checked plan frame marker is missing or transplanted",
        ),
        (
            D2jCause::SelectedSlot,
            "checked computational-IH slot Runtime occurrences differ",
        ),
        (
            D2jCause::Invocation,
            "checked computational-IH call Runtime occurrences differ",
        ),
    ]
    .into_iter()
    .map(|(cause, expected)| {
        let (arrivals, error, census) =
            compile_cause_with_census(cause, "ken_d2f_gate_neg");
        let validator_admitted = match census.as_slice() {
            [row] => row.validator_admitted,
            rows => panic!(
                "{cause:?}: one production compile must record exactly one census row, got {}: \
                 {rows:?}",
                rows.len()
            ),
        };
        let named_its_own_authority = matches!(
            &error,
            Some(CraneliftBackendError::Unsupported(UnsupportedLowering { construct, reason }))
                if *construct == "OrientedSubcontinuationPlanV1" && reason.contains(expected)
        );
        (
            cause,
            arrivals.len(),
            validator_admitted,
            named_its_own_authority,
            error,
        )
    })
    .collect();
    let arrived_empty: Vec<_> =
        [D2jCause::ExactSuffix, D2jCause::CallIdentity]
            .into_iter()
            .map(|cause| {
                let arrivals = compile_cause(cause, "ken_d2f_gate_empty").0;
                let one = match arrivals.as_slice() {
                    [only] => only.clone(),
                    other => panic!("{cause:?} must reach the builder once: {}", other.len()),
                };
                (
                    cause,
                    arrivals.len(),
                    one.keys.len(),
                    one.descriptors.len(),
                    one.walked_admitted_continuation_discoveries,
                    one.oriented_frames,
                    one.oriented_recursive_calls,
                    one.oriented_computational_ih_slots,
                    one.oriented_computational_ih_calls,
                )
            })
            .collect();

    assert_eq!(
        (
            // the positives -- the denominator for everything below them
            (exact.oriented_present, exact.keys.len(), exact.descriptors.len(), exact.fusion_definitions),
            (rehomed.oriented_present, rehomed.keys.len(), rehomed.descriptors.len(), rehomed.fusion_definitions),
            (
                arity.oriented_present,
                arity.keys.len(),
                arity.descriptors.len(),
                arity.fusion_definitions
            ),
            // the unmarked seed
            (seed.oriented_present, seed.keys.len(), seed.descriptors.len(), seed.fusion_definitions),
            // AC-6a phase A: refused at its OWN named authority, never arrived
            no_arrival
                .iter()
                .map(|(_, arrivals, admitted, named, _)| (*arrivals, *admitted, *named))
                .collect::<Vec<_>>(),
            // AC-6a phase B: arrived once, resolved nothing
            arrived_empty
                .iter()
                .map(|(_, a, k, d, walked, frames, recursive, slots, calls)| {
                    (*a, *k, *d, *walked, *frames, *recursive, *slots, *calls)
                })
                .collect::<Vec<_>>(),
        ),
        (
            (true, 1, 1, 0),
            (true, 1, 1, 0),
            (true, 1, 1, 0),
            (false, 0, 0, 0),
            vec![(0, false, true), (0, false, true), (0, false, true)],
            vec![
                (1, 0, 0, 4, 2, 0, 2, 1),
                (1, 0, 0, 4, 2, 0, 2, 1),
            ],
        ),
        "all THREE positives must resolve exactly one key and one descriptor at definition \
         count zero, while the unmarked seed reaches the same builder and resolves nothing, \
         three marker causes refuse at their own named authority without ever reaching the \
         builder's Ok return, and two source-shape causes reach it and resolve nothing. The \
         two refusal tiers are kept apart deliberately, and each tier-3 row is carried by the \
         outer validator's admission bit plus its refusal SENTENCE rather than by its zero: an \
         arrivals count counts builder calls that returned Ok, so a zero alone cannot tell \
         'refused upstream' from 'reached the builder and errored'. The census bit identifies \
         which phase fired, while the named authority identifies why. Only the \
         arrived-and-empty pair is evidence about the builder, and every tier is an operand of \
         the same assertion as the positives so that \
         no zero stands alone -- rows {no_arrival:?} and {arrived_empty:?}"
    );

    // ---- the pre-emission seat, per positive, on its own root.
    ordinary_refusal(&exact_error, "exact");
    ordinary_refusal(&rehomed_error, "re-homed");
    ordinary_refusal(&arity_error, "widened-arity");

    // ---- production must agree with the INDEPENDENT planner derivation, per cause.
    let exact_planner = planner_plane(D2jCause::Exact);
    let rehomed_planner = planner_plane(D2jCause::ReHomed);
    assert_eq!(
        (exact.keys.as_slice(), exact.descriptors.as_slice()),
        (exact_planner.observed_keys(), exact_planner.observed_descriptors()),
        "the applied exact root must resolve the SAME complete key and descriptor through the \
         production compile as the planner controls derive from it"
    );
    assert_eq!(
        (rehomed.keys.as_slice(), rehomed.descriptors.as_slice()),
        (rehomed_planner.observed_keys(), rehomed_planner.observed_descriptors()),
        "and the bare re-homed root likewise, against ITS own planner derivation"
    );

    let arity_planner = planner_plane(D2jCause::ProducerArity);
    assert_eq!(
        (arity.keys.as_slice(), arity.descriptors.as_slice()),
        (
            arity_planner.observed_keys(),
            arity_planner.observed_descriptors()
        ),
        "and the widened-arity root likewise, against ITS own planner derivation"
    );

    // ---- non-aliasing, by whole keys and never by id (AC-6c), now PAIRWISE
    // over the three positives (`D2k` `AC-1a`).
    //
    // Stated over ALL PAIRS of the positive population rather than as a fixed
    // list of `assert_ne!`s: individual inequalities are what let a later fourth
    // positive be added while one pair silently coincides. Distinctness is a
    // property of the population, so it is derived from the population -- add a
    // positive to the array and it is covered with no second edit.
    //
    // Not a set, deliberately: `StaticContinuationFusionKey` is `Eq` and not
    // `Ord`, and giving a planner identity a total order to let a test build a
    // `BTreeSet` would widen a production type for a test's convenience.
    let positives = [
        ("exact", &exact.keys),
        ("re-homed", &rehomed.keys),
        ("widened-arity", &arity.keys),
    ];
    let coincident: Vec<(&str, &str)> = (0..positives.len())
        .flat_map(|left| (left + 1..positives.len()).map(move |right| (left, right)))
        .filter(|(left, right)| positives[*left].1 == positives[*right].1)
        .map(|(left, right)| (positives[left].0, positives[right].0))
        .collect();
    assert_eq!(
        coincident,
        Vec::<(&str, &str)>::new(),
        "the three positive roots describe three different programs, so their complete keys \
         must be pairwise distinct -- established by the keys themselves, never by an id \
         inequality, since the planes are independent interners that all lawfully issue local \
         id 0. exact={:?} rehomed={:?} arity={:?}",
        exact.keys,
        rehomed.keys,
        arity.keys
    );
}

/// Gate 4b's observer is a pure read of the production planner inputs.
///
/// MEASURED: the same checked fixture, entry symbol, object-module name and
/// test-only emitter arm produce byte-identical object artifacts with arrival
/// recording enabled and disabled. The enabled leg records exactly one arrival
/// and its five input populations; the disabled leg records none.
///
/// CLAIMED: recording the input populations does not alter compilation output.
///
/// THE GAP: `d2f_0_the_applied_root_production_path_gate` pins the five input
/// populations at `(4, 2, 0, 2, 1)` for the perturbed `ExactSuffix` and
/// `CallIdentity` causes, while
/// `r3_4b_input_observation_is_artifact_identical_when_disabled` pins the same
/// tuple for unperturbed `Exact`. The latter resolves one key and descriptor;
/// the former two resolve none. The `walked` input-population count is sound,
/// but it does not move when a downstream relation declines, so it cannot
/// establish whether a decline occurred or attribute one.
///
/// Promise class: durable invariant.
#[test]
fn r3_4b_input_observation_is_artifact_identical_when_disabled() {
    use crate::cranelift_backend::lowering::core::{
        d2f_gate_arrivals_take, set_d2f_gate_observation_enabled, D2fEmitterTestArm,
        D2fGateArrival,
    };
    use crate::cranelift_backend::planning::{
        d2j_checked_fixture_under, D2jCause, D2J_DECLARATION,
    };

    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            set_d2f_gate_observation_enabled(true);
            let _ = d2f_gate_arrivals_take();
        }
    }

    fn emit(record_arrival: bool) -> (Vec<u8>, Vec<D2fGateArrival>) {
        let (entry, declaration, oriented) = d2j_checked_fixture_under(D2jCause::Exact);
        let mut declarations = std::collections::BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        let _ = d2f_gate_arrivals_take();
        set_d2f_gate_observation_enabled(record_arrival);
        let compiled = {
            let _arm = D2fEmitterTestArm::arm();
            crate::cranelift_backend::lowering::core::compile_expr_into_object_module(
                crate::cranelift_backend::artifact::new_object_module_for_lowering_tests(
                    "ken-r3-4b-input-identity",
                )
                .expect("object module"),
                "ken_r3_4b_input_identity",
                cranelift_module::Linkage::Export,
                &entry,
                &crate::NativeSeedEnvironment::empty(),
                declarations,
                None,
                false,
                None,
                None,
                Some(oriented),
            )
            .expect("the test-only armed checked fixture compiles")
        };
        let arrivals = d2f_gate_arrivals_take();
        let bytes = compiled
            .module
            .finish()
            .emit()
            .expect("the checked fixture emits an object artifact");
        (bytes, arrivals)
    }

    let _restore = Restore;
    let (enabled_bytes, enabled_arrivals) = emit(true);
    let (disabled_bytes, disabled_arrivals) = emit(false);
    let enabled = match enabled_arrivals.as_slice() {
        [only] => only,
        other => panic!(
            "the enabled observation must record exactly one builder arrival: {}",
            other.len()
        ),
    };

    assert_eq!(
        (
            enabled.walked_admitted_continuation_discoveries,
            enabled.oriented_frames,
            enabled.oriented_recursive_calls,
            enabled.oriented_computational_ih_slots,
            enabled.oriented_computational_ih_calls,
        ),
        (4, 2, 0, 2, 1),
        "the enabled route must reach the observer and report the walked ledger plus each named \
         oriented-plan vector separately"
    );
    assert!(
        disabled_arrivals.is_empty(),
        "the disabled control must suppress the observation rather than reproducing the enabled leg"
    );
    assert!(
        !enabled_bytes.is_empty(),
        "the artifact-identity relation must compare an emitted object, not two empty buffers"
    );
    assert_eq!(
        enabled_bytes, disabled_bytes,
        "enabling the gate-4b input observation must leave the emitted object artifact byte-identical"
    );
}

/// **`RT-LEXICAL-R3-FUSION-EMITTER` `D0` — the gate, before any emitter body.**
///
/// Three rows, and the reason they are one assertion is that **two of them are
/// zeros**. A zero proves nothing on its own: an absence comparator that
/// resolves nothing and a refusal that resolves nothing look identical to a
/// compile that never ran. The non-zero positive is the denominator both are
/// read against, so it is an operand here rather than a separate test.
///
/// | row | subject | resolved plane |
/// |---|---|---|
/// | absence | the seed `px8j` before-hole compile, no oriented plan | **0** |
/// | positive | the checked twin through the production entry | **1** |
/// | refusal | that twin with one marker stripped | never planned |
///
/// **Why the seed cannot be the positive, since this is the defect the node's
/// frame says has already cost this work once.** `px8j` is a seed-lane compile
/// with no checked frame, no selected-IH slot and no checked-IH-invocation
/// marker; production oriented plans are decoded from a checked package's
/// metadata, and a seed-lane compile has none to decode. The `oriented` gate
/// then returns an empty plane **before candidate enumeration runs**. So the
/// seed is retained here as the **absence comparator** and is never the
/// fusion-positive. Handing it a plan is a forbidden route, not an untried one.
///
/// **The positive is the landed `D2j` checked twin** — designated by the frame
/// as the `R3`-shaped one; that designation is the frame's and is cited, not
/// re-derived here. What this test verifies about it is narrower and its own:
/// that it reaches the builder through `compile_expr_into_object_module` with
/// `Some(oriented)` — **never a direct builder or emitter call** — and resolves
/// exactly one key, one id and one descriptor.
///
/// **Both planes are read by the same call**, `build_static_continuation_fusion_plan`.
/// That is what makes `0` and `1` **one currency** instead of an arrival field
/// compared against a plane length, and it is also the non-constancy proof: the
/// same instrument answers both ways in this test, so neither number is a shape
/// it returns for anything handed to it.
///
/// **CORRECTION, and it is the load-bearing kind.** An earlier version of this
/// block said the two reads differ *"only in the argument that is genuinely
/// different between the two lanes — `Some(oriented)` against `None`."*
/// **That attribution is false.** The seed row and the positive row differ in
/// **four** things at once: a different planner, a different expression,
/// different declarations, **and** the oriented argument. So those two rows
/// establish a shared **return-type currency** and nothing more — they cannot
/// attribute the `0`/`1` difference to `oriented`, because three other
/// variables moved with it. The currency claim survives; the causal one never
/// held.
///
/// **The one-variable cell below is what attribution needs**, and it is the
/// `D2h` soundness discriminator (Steward `evt_2n6n5hnxyh0cg`). It re-reads the
/// **same planner, same expression, same declarations** as the positive, with
/// the oriented plan **withheld** — `None`. One variable moves.
///
/// The fork it was given had two branches: plane **0** means the attribution
/// holds, plane **1** means a candidate forms without the plan and is a stop.
///
/// **MEASURED: neither. It REFUSES** —
/// `OrientedSubcontinuationPlanV1` / *"checked subcontinuation markers have no
/// checked plan metadata"*. The markers are still on the expression and the
/// plan they require is gone, so the builder rejects the combination rather
/// than quietly resolving nothing.
///
/// **What that refusal is, stated on its own axis.** It is raised by the
/// `OrientedSubcontinuationPlanV1` validator **before** transport, before any
/// candidate is formed, and therefore before any key, ID or descriptor exists.
/// So it is a **fail-closed plan-dependence refusal** — the marked expression
/// will not proceed without the plan its markers require.
///
/// **It is NOT plane 0, and it is NOT a cardinality result.** Nothing here
/// counts candidates, because the path stops upstream of counting. ⇒ **This row
/// attributes nothing about the `0`/`1` cardinality**, and must not be read as
/// the attribution the one-variable cell was added to supply.
///
/// On the **plan-dependence axis alone**, it is a stronger answer than plane 0
/// would have been: plane 0 would say a candidate failed to form, this says the
/// pipeline refuses before it could be asked for one. That is the only axis on
/// which "stronger" is a claim this row can carry.
///
/// **It is asserted as the refusal it is, not re-mapped onto the fork's zero
/// branch.** Folding an unanticipated outcome into the nearest anticipated one
/// is how a measurement stops being one, and this row would have read as a
/// clean plane-0 confirmation for every future reader.
///
/// `px8j` is retained beside it as the **distinct seed absence comparator**,
/// with no plan injected, because it answers a different question: what an
/// unmarked seed-lane compile does, not what withholding a plan does to a
/// marked one.
///
/// **The production compile and the planner derivation are independent, and
/// that is the only reason their agreement says anything.** Neither reads the
/// other: one is the arrival recorded inside a real compile, the other is
/// `build_static_continuation_fusion_plan` run separately over the same root.
/// Agreement on the **whole key and descriptor** is asserted, not agreement on
/// an id — two independent interners both lawfully issue local id `0`.
///
/// **The refusal row is the validator's exact sentence, not a count.** Stripping
/// the frame marker must produce the `OrientedSubcontinuationPlanV1` refusal
/// naming a missing or transplanted frame marker, and must do so **before the
/// builder is reached** — asserted as zero arrivals beside the positive's one.
/// A count alone could not tell "the validator refused" from "the compile
/// failed earlier for an unrelated reason".
///
/// **`fusion_definitions` is `0` in every row and that is deliberate.** No
/// emitter body exists at this deliverable, so a non-zero definition count here
/// would mean something was armed ahead of its gate.
///
/// **Promise class: durable invariant.** Arrival, presence, plane cardinality
/// and key agreement between two independently reached derivations. The
/// literals are `1` (what the identity plane is defined to produce for one
/// candidate) and `0` (the definition population before an emitter exists).
#[test]
fn d0_r3_fusion_gate_resolves_zero_for_the_seed_and_one_for_the_checked_twin() {
    use crate::cranelift_backend::lowering::core::d2f_gate_arrivals_take;
    use crate::cranelift_backend::planning::{d2j_checked_fixture_under, D2jCause, D2J_DECLARATION};

    /// One cause through the PRODUCTION entry, with its own oriented plan.
    fn compile_cause(
        cause: D2jCause,
        symbol: &str,
    ) -> (
        Vec<crate::cranelift_backend::lowering::core::D2fGateArrival>,
        Option<CraneliftBackendError>,
    ) {
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let _ = d2f_gate_arrivals_take();
        let mut declarations = std::collections::BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        let result = crate::cranelift_backend::lowering::core::compile_expr_into_object_module(
            crate::cranelift_backend::artifact::new_object_module_for_lowering_tests("ken-r3-d0")
                .expect("object module"),
            symbol,
            cranelift_module::Linkage::Export,
            &entry,
            &crate::NativeSeedEnvironment::empty(),
            declarations,
            None,
            false,
            None,
            None,
            Some(oriented),
        );
        (d2f_gate_arrivals_take(), result.err())
    }

    // ---- row 2 first: the positive is the denominator for the two zeros.
    let (positive_arrivals, positive_error) = compile_cause(D2jCause::Exact, "ken_r3_d0_positive");
    let positive = match positive_arrivals.as_slice() {
        [only] => only.clone(),
        other => panic!("the checked twin must reach the builder exactly once: {}", other.len()),
    };

    // The independent derivation. Same root, different route: this one never
    // reads the arrival above.
    let (entry, declaration, oriented) = d2j_checked_fixture_under(D2jCause::Exact);
    let mut declarations = std::collections::BTreeMap::new();
    declarations.insert(D2J_DECLARATION, &declaration);
    let planner = crate::cranelift_backend::planning::plan_static_transition_graph(
        &entry,
        &declarations,
    )
    .expect("the checked twin plans");
    let planner_plane = crate::cranelift_backend::planning::build_static_continuation_fusion_plan(
        &planner,
        &entry,
        &declarations,
        Some(&oriented),
    )
    .expect("the checked twin resolves a plane");

    // ── THE ONE-VARIABLE CELL — the `D2h` soundness discriminator ──
    //
    // Same planner, same expression, same declarations as the positive above.
    // The ONLY thing that moves is the oriented plan, withheld. This is what
    // the seed row cannot do: that row changes four things at once, so it
    // establishes a shared return-type currency and attributes nothing.
    //
    // Plane 0 here means the candidate genuinely depends on the plan. Plane 1
    // would mean a candidate forms without it -- fusion independent of
    // `oriented` -- and that is a stop before D2/D3, not a number to record.
    //
    // MEASURED, and it is a THIRD outcome the fork did not name: withholding
    // the plan resolves neither plane 0 nor plane 1. It REFUSES. The markers
    // are still on the expression and the plan they require is gone, so the
    // builder rejects the combination rather than quietly resolving nothing.
    //
    // The refusal is raised BEFORE transport, before a candidate is formed, and
    // so before any key, ID or descriptor exists. It is a fail-closed
    // plan-dependence refusal and NOT a cardinality result: nothing here counts
    // candidates, because the path stops upstream of counting. On the
    // plan-dependence axis alone it is stronger than plane 0 would have been --
    // that is the only axis on which "stronger" is a claim this row carries.
    //
    // It is recorded as the refusal it is, NOT re-mapped onto the fork's
    // zero branch. Re-mapping an unanticipated outcome onto the nearest
    // anticipated one is how a measurement stops being one.
    let withheld = crate::cranelift_backend::planning::build_static_continuation_fusion_plan(
        &planner,
        &entry,
        &declarations,
        None,
    );
    let withheld_refuses_for_missing_metadata = matches!(
        &withheld,
        Err(CraneliftBackendError::Unsupported(UnsupportedLowering { construct, reason }))
            if *construct == "OrientedSubcontinuationPlanV1"
                && reason.contains("no checked plan metadata")
    );

    // ---- row 1: the seed, which carries no marker at all.
    let seed_expr = host_result_closure_match(px8j_equal_payload_hole_placement(
        Px8jSelectedScopePlacement::BeforeReturnHole,
    ));
    let _ = d2f_gate_arrivals_take();
    let (seed_result, _seed_trace) =
        px8j_capture_source_trace(&seed_expr, false, "ken_r3_d0_seed");
    assert!(matches!(
        seed_result,
        Err(CraneliftBackendError::Unsupported(UnsupportedLowering {
            construct: "ComputationalMatch",
            reason,
        })) if reason == "a computational recursor closure names an in-flight activation, not a transferable value"
    ), "the seed must reach the builder before its measured D8 refusal");
    let seed = match d2f_gate_arrivals_take().as_slice() {
        [only] => only.clone(),
        other => panic!("the seed witness must reach the builder once: {}", other.len()),
    };
    // The seed's plane through the SAME instrument the positive is measured
    // with, so "resolved plane 0" and "resolved plane 1" are one currency
    // rather than an arrival field compared against a plane length. `None` is
    // the seed lane's real argument -- it has no checked package metadata to
    // decode a plan from, which is why it is the absence comparator.
    let seed_declarations = std::collections::BTreeMap::new();
    let seed_planner = crate::cranelift_backend::planning::plan_static_transition_graph(
        &seed_expr,
        &seed_declarations,
    )
    .expect("the seed witness plans");
    let seed_plane = crate::cranelift_backend::planning::build_static_continuation_fusion_plan(
        &seed_planner,
        &seed_expr,
        &seed_declarations,
        None,
    )
    .expect("the seed witness resolves a plane");

    // ---- row 3: one marker stripped, and the validator's own sentence.
    let (stripped_arrivals, stripped_error) = compile_cause(D2jCause::Frame, "ken_r3_d0_stripped");
    let stripped_refusal = matches!(
        &stripped_error,
        Some(CraneliftBackendError::Unsupported(UnsupportedLowering { construct, reason }))
            if *construct == "OrientedSubcontinuationPlanV1"
                && reason.contains("frame marker is missing or transplanted")
    );

    assert_eq!(
        (
            // row 2 -- the POSITIVE, and it must be non-zero or nothing below
            // it means anything
            (
                positive.oriented_present,
                positive.keys.len(),
                positive.descriptors.len(),
                planner_plane.len(),
                positive.fusion_definitions,
            ),
            // row 1 -- the absence comparator at resolved plane zero, with its
            // plane read by the same instrument as the positive's
            (
                seed.oriented_present,
                seed.keys.len(),
                seed.descriptors.len(),
                seed_plane.len(),
                seed.fusion_definitions,
            ),
            // row 3 -- refused by the validator, before the builder
            (stripped_refusal, stripped_arrivals.len()),
            // the ONE-VARIABLE cell: same planner, expression and declarations
            // as the positive, oriented plan withheld. Neither plane -- a
            // refusal, which is a stronger answer than plane 0.
            withheld_refuses_for_missing_metadata,
        ),
        (
            (true, 1, 1, 1, 0),
            (false, 0, 0, 0, 0),
            (true, 0),
            true,
        ),
        "D0: the checked twin must reach the builder through the production entry with \
         Some(oriented) and resolve exactly one key, one id and one descriptor at definition \
         count zero; the unmarked seed must reach the SAME builder and resolve nothing; and \
         one stripped marker must produce the plan validator's own refusal without reaching \
         the builder at all. positive={positive:?} seed={seed:?} stripped={stripped_error:?}"
    );

    // ---- the two derivations agree on the WHOLE key and descriptor.
    //
    // Never on an id: the production plane and the planner plane are separate
    // interners and both lawfully issue local id 0, so an id equality would
    // hold between two unrelated fusions.
    assert_eq!(
        (positive.keys.as_slice(), positive.descriptors.as_slice()),
        (planner_plane.observed_keys(), planner_plane.observed_descriptors()),
        "the production compile must resolve the same complete key and descriptor the \
         planner derives independently from the same root"
    );

    // ---- the pre-emission seat, stated rather than left implicit.
    //
    // The positive still refuses, and refusing is correct here: no emitter body
    // exists yet, so this is the ordinary refusal fusion is meant to remove.
    // It is the baseline the later 0 -> 1 movement is measured against, and
    // recording it now is what makes that movement attributable.
    assert!(
        matches!(
            &positive_error,
            Some(CraneliftBackendError::Unsupported(UnsupportedLowering { construct, reason }))
                if *construct == "ComputationalMatch"
                    && reason.contains("in-flight activation")
        ),
        "before any emitter body the checked twin must still reach its own ordinary \
         refusal: {positive_error:?}"
    );
}

/// `RT-CONTKEY-CONSUMING-OCCURRENCE` AC-1/AC-3/AC-4: the planner-carried
/// relation agrees with an ordinal-selected body read from the outer
/// eliminator, has a non-empty population on both governed rows, and leaves
/// the continuation's own owner relation exact.
#[test]
fn contkey_rows_four_and_five_carry_the_exact_outer_consuming_occurrence() {
    fn observe(label: &str, expression: &RuntimeExpr) -> usize {
        let declarations = BTreeMap::new();
        let plan = plan_static_transition_graph(expression, &declarations)
            .expect("the governed continuation row plans");
        let units = plan.continuation_units().expect("continuation units");
        assert_eq!(
            units.len(),
            1,
            "{label}: the discovery carry is not specialization identity and must not split the \
             one interned unit",
        );
        let mut count = 0;
        for unit in &units {
            let Some(carried) = unit.consuming_occurrence() else {
                continue;
            };
            let direct = plan
                .rederive_continuation_consuming_occurrence(unit)
                .expect("the outer selected case body re-derives");
            eprintln!(
                "{label}: carried={carried:?} direct={direct:?} consumer_owner={:?}",
                unit.consumer_owner(),
            );
            assert_eq!(
                direct,
                Some(carried),
                "{label}: the carried occurrence must equal the directly selected outer body",
            );
            assert!(
                plan.continuation_consumer_owner_is_exact(unit)
                    .expect("consumer-owner authority is readable"),
                "{label}: consumer_owner must still name the continuation occurrence's owner",
            );
            count += 1;
        }
        assert_eq!(
            count, 1,
            "{label}: exactly one specialization edge must carry the outer consuming relation",
        );
        count
    }

    let row4 = host_result_closure_match(px8j_scope_chain_observation_result(1, 0));
    let row5 = host_result_closure_match(px8j_equal_payload_hole_placement(
        Px8jSelectedScopePlacement::AfterReturnHole,
    ));
    assert_eq!(
        observe("row4-depth-1", &row4) + observe("row5-after-hole", &row5),
        2
    );
}

/// `RT-CONTKEY-CONSUMER-DESCENT-CARRY` AC-1 through AC-3 and D3/D5.
///
/// MEASURED: for the row-4 fixtures at depths 1, 2 and 3, the production
/// planner records the required consumer on the discovery at producer use and
/// records the target-derived consumer installed on its child push. The same
/// compiles report the existing key's consuming occurrence and interned-unit
/// population. Each separately sized compile observes one outermost required-
/// consumer level; intermediate boundaries are not observed in that compile
/// and are inferred from the sibling compiles.
///
/// CLAIMED: at depth 1, `required` coincides with that level's consuming
/// occurrence; from depth 2 onward, `required(N)` is the existing occurrence
/// established at level `N-1`. The discovery fact is traversal state and must
/// not split specialization identity.
///
/// THE GAP: this observes the carried planner identity before any lowering
/// consumer exists. It does not claim that a later Closure/static-worker route
/// consumes the fact or that the governed source compiles. Child-push records
/// are retained as diagnostics, but equality with the same singleton unit is
/// constructional and is not an independent control.
///
/// Promise class: durable invariant. The depth-2/depth-3 equalities compare
/// independently produced planner records. The depth-1 equality is
/// same-compile and both resolvers share one source seed, so it pins resolver
/// agreement rather than correctness of that seed. None uses fixture origin
/// literals. The cross-compile comparisons instead rely on the generator's
/// wrapper-invariance:
/// adding a wrapper preserves the inner level's origins from the sibling
/// compile. Renumbering inside the generator can therefore false-red this
/// control without a carry defect.
#[test]
fn contkey_row_four_discovery_carries_the_outer_boundary_then_previous_consumers() {
    use crate::cranelift_backend::planning::{
        take_continuation_required_consumer_observations, ContinuationConsumingOccurrence,
    };

    #[derive(Debug)]
    struct Observed {
        unit_consumer: ContinuationConsumingOccurrence,
        required: ContinuationConsumingOccurrence,
        units: usize,
    }

    fn observe(depth: usize) -> Observed {
        let _ = take_continuation_required_consumer_observations();
        let expression = host_result_closure_match(px8j_scope_chain_observation_result(depth, 0));
        let declarations = BTreeMap::new();
        let plan = plan_static_transition_graph(&expression, &declarations)
            .expect("the governed row-4 continuation plans");
        let units = plan.continuation_units().expect("continuation units");
        let [unit] = units.as_slice() else {
            panic!(
                "row4-depth-{depth}: expected one interned specialization, got {}",
                units.len(),
            );
        };
        let unit_consumer = unit
            .consuming_occurrence()
            .expect("the existing depth-specific key relation remains populated");

        let mut required_at_use = BTreeSet::new();
        let mut advanced = BTreeSet::new();
        let mut raw = BTreeSet::new();
        for observation in take_continuation_required_consumer_observations() {
            let observed_required = observation
                .required()
                .expect("a carry observation must name the required consumer");
            raw.insert((
                observation.is_child_push(),
                format!("{:?}", observation.continuation_origin()),
                format!("{:?}", observation.result_root()),
                format!("{:?}", observed_required.body_origin()),
                format!("{:?}", observed_required.eliminator_origin()),
                format!("{:?}", observation.derived_at_consumer()),
            ));
            if !observation.is_child_push() {
                let derived = observation.derived_at_consumer().expect(
                    "a consumer-level observation must carry the independent derivation",
                );
                assert_eq!(
                    derived,
                    observed_required,
                    "row4-depth-{depth}: the lagged required occurrence must re-derive from \
                     the enclosing specialization at the consumer level",
                );
            }
            if observation.is_child_push() {
                advanced.insert(observed_required);
            } else {
                required_at_use.insert(observed_required);
            }
        }
        eprintln!(
            "row4-depth-{depth}: unit_consumer={unit_consumer:?} raw_required_carry={raw:?}"
        );
        let required = required_at_use.into_iter().collect::<Vec<_>>();
        let [required] = required.as_slice() else {
            panic!("row4-depth-{depth}: producer-use carry is not one exact identity");
        };
        let advanced = advanced.into_iter().collect::<Vec<_>>();
        let [_advanced] = advanced.as_slice() else {
            panic!("row4-depth-{depth}: child-push carry is not one exact identity");
        };
        Observed {
            unit_consumer,
            required: *required,
            units: units.len(),
        }
    }

    let depth_1 = observe(1);
    let depth_2 = observe(2);
    let depth_3 = observe(3);

    assert_eq!(
        depth_1.required, depth_1.unit_consumer,
        "row4-depth-1 must use its own outermost consumer before the lag begins",
    );
    assert_eq!(
        depth_2.required, depth_1.unit_consumer,
        "row4-depth-2 must carry the exact consumer established at depth 1",
    );
    assert_eq!(
        depth_3.required, depth_2.unit_consumer,
        "row4-depth-3 must carry the exact consumer established at depth 2",
    );
    assert_eq!(
        [depth_1.units, depth_2.units, depth_3.units],
        [1, 1, 1],
        "the discovery-only carry must leave the interned-unit population unchanged",
    );
}

fn required_consumer_projection_census(
) -> Vec<(
    &'static str,
    crate::cranelift_backend::planning::RequiredConsumerProjectionDisposition,
)> {
    use crate::cranelift_backend::planning::{
        take_continuation_required_consumer_observations,
        RequiredConsumerProjectionDisposition,
    };

    fn observe(expression: RuntimeExpr) -> RequiredConsumerProjectionDisposition {
        let _ = take_continuation_required_consumer_observations();
        plan_static_transition_graph(&expression, &BTreeMap::new())
            .expect("the governed expression must reach static-transition planning");
        let dispositions = take_continuation_required_consumer_observations()
            .into_iter()
            .filter_map(|observation| observation.projection_disposition())
            .collect::<BTreeSet<_>>();
        let dispositions = dispositions.into_iter().collect::<Vec<_>>();
        let [disposition] = dispositions.as_slice() else {
            panic!(
                "one governed row must report one projection disposition, got {dispositions:?}"
            );
        };
        *disposition
    }

    vec![
        (
            "row1-owned-scope",
            observe(host_result_closure_match(px8j_layered_recursive_result(1, 1))),
        ),
        (
            "row4-depth-1",
            observe(host_result_closure_match(px8j_scope_chain_observation_result(1, 0))),
        ),
        (
            "row4-depth-2",
            observe(host_result_closure_match(px8j_scope_chain_observation_result(2, 0))),
        ),
        (
            "row4-depth-3",
            observe(host_result_closure_match(px8j_scope_chain_observation_result(3, 0))),
        ),
        (
            "row5-after-hole",
            observe(host_result_closure_match(px8j_equal_payload_hole_placement(
                Px8jSelectedScopePlacement::AfterReturnHole,
            ))),
        ),
    ]
}

/// `RT-REQUIRED-CONSUMER-REACH-CENSUS` D2: the existing required-consumer
/// observation reports each governed row's projection disposition without
/// inferring it from the later refusal.
///
/// MEASURED: each row's real planner call reports whether it minted a
/// projection, skipped minting because `required == source`, or had no required
/// consumer to place in the pending projection list. CLAIMED: the projection
/// surface reaches exactly the rows classified `Minted`. THE GAP: this is a
/// planning-surface census only; it does not claim that any residual is closed.
///
/// Promise class: transition sentinel. Rewrite this table when an authorized
/// route changes which governed call enters or mints from the projection list.
#[test]
fn required_consumer_projection_censuses_each_governed_row() {
    use crate::cranelift_backend::planning::RequiredConsumerProjectionDisposition;

    let census = required_consumer_projection_census();
    assert_eq!(
        census,
        vec![
            (
                "row1-owned-scope",
                RequiredConsumerProjectionDisposition::SkippedRequiredEqualsSource,
            ),
            (
                "row4-depth-1",
                RequiredConsumerProjectionDisposition::SkippedRequiredEqualsSource,
            ),
            (
                "row4-depth-2",
                RequiredConsumerProjectionDisposition::Minted,
            ),
            (
                "row4-depth-3",
                RequiredConsumerProjectionDisposition::Minted,
            ),
            (
                "row5-after-hole",
                RequiredConsumerProjectionDisposition::SkippedRequiredEqualsSource,
            ),
        ],
        "each governed row must retain its measured projection-surface disposition",
    );
}

/// `RT-REQUIRED-OCCURRENCE-PROJECTION` AC-1: each coordinate of the new
/// consumer-level relation is independently validated before lowering can
/// receive the opaque projection.
#[test]
fn required_consumer_projection_refuses_each_wrong_coordinate() {
    use crate::cranelift_backend::planning::{
        with_required_consumer_projection_mutation, RequiredConsumerProjectionMutation,
    };

    let expression = host_result_closure_match(px8j_scope_chain_observation_result(2, 0));
    let declarations = BTreeMap::new();
    for (mutation, expected, other) in [
        (
            RequiredConsumerProjectionMutation::BodyOrigin,
            "a required-consumer projection has a mismatched body_origin",
            "mismatched eliminator_origin",
        ),
        (
            RequiredConsumerProjectionMutation::EliminatorOrigin,
            "a required-consumer projection has a mismatched eliminator_origin",
            "mismatched body_origin",
        ),
    ] {
        let (error, applications) = with_required_consumer_projection_mutation(mutation, || {
            match plan_static_transition_graph(&expression, &declarations) {
                Ok(_) => panic!("a mutated required-consumer coordinate must refuse"),
                Err(error) => error,
            }
        });
        let rendered = format!("{error:?}");
        assert_eq!(
            applications, 1,
            "the mutation must alter one real depth-2 projection rather than a sentinel",
        );
        assert!(
            rendered.contains(expected),
            "the mutated relation field must name its own refusal: {rendered}",
        );
        assert!(
            !rendered.contains(other),
            "the refusal must not attribute the other relation field: {rendered}",
        );
    }
}

/// `RT-REQUIRED-OCCURRENCE-PROJECTION` D3/D4: the production funnel consumes
/// the opaque projection. The route installs both binders; suppressing only
/// that branch removes both installations. The synthesized-environment repair
/// makes both legs converge again at the later `StaticWorkerBinding` boundary.
#[test]
fn required_consumer_projection_reaches_the_depth_two_funnel() {
    use crate::cranelift_backend::lowering::core::with_required_consumer_route_suppressed;
    use crate::cranelift_backend::lowering::{d2k_owner_trace_take, D2kOwnerEvent};

    fn compile() -> (String, usize) {
        let _ = d2k_owner_trace_take();
        let expression = host_result_closure_match(px8j_scope_chain_observation_result(2, 0));
        let (result, _trace) =
            px8j_capture_source_trace(&expression, false, "ken_required_consumer_depth2");
        let installs = d2k_owner_trace_take()
            .iter()
            .filter(|event| {
                matches!(event, D2kOwnerEvent::StaticWorkerBinderInstalled { .. })
            })
            .count();
        let outcome = match result {
            Err(CraneliftBackendError::Unsupported(UnsupportedLowering {
                construct, ..
            })) => construct.to_string(),
            Ok(_) => "compiled".to_string(),
            Err(other) => format!("other:{other}"),
        };
        (outcome, installs)
    }

    assert_eq!(compile(), ("StaticWorkerBinding".to_string(), 2));
    let (suppressed, applications) = with_required_consumer_route_suppressed(compile);
    assert_eq!(applications, 1, "the mutation must suppress one real funnel route");
    assert_eq!(
        suppressed,
        ("StaticWorkerBinding".to_string(), 0),
        "without the projected consumer depth 2 must retain the same later boundary but lose both binder installations",
    );
}

/// `RT-REQUIRED-CONSUMER-REACH-CENSUS` D5: the route-suppression differential
/// records closure presence and transfer reach as separate facts.
///
/// MEASURED: at the real `transfer_into_carrier` entry, both enabled depth-2+
/// rows present origin 5's constructor with a closure at argument 0. Suppressing
/// the one real required-consumer route returns each row to its old
/// `StaticWorkerBinding` refusal and removes that exact crossing entirely.
///
/// CLAIMED: the required-consumer route manufactures the closure-bearing
/// crossing at `StaticOriginId(5)` / `Constructor.arg[0].Closure` for both
/// rows. THE GAP: branch 1 versus branch 3′ remains unseparated because
/// suppression removes the only observation point along with the crossing.
/// This does not establish the later repair, its owner, or any subsumption with
/// the separate durable-closure population.
///
/// `RT-CROSSING-CALLEE-IDENTITY` D1/D2 MEASURED: the depth-2 origin-5 crossing
/// set is the singleton `{source LexicalClosure argument -> callee body
/// StaticOriginId(49)}`; depth 3's set is the singleton `{the same caller arm ->
/// callee body StaticOriginId(59)}`. Both callees are the source program's exact
/// direct HostResult `Match` bodies, not either row's required-consumer
/// projection.
///
/// The invoking-site tag is ambient for `carry_call_input`'s whole dynamic
/// guard extent, not lexical proof that the helper directly invoked a transfer.
/// A reverted re-entrancy probe measured `enclosing_transfers = 0` in all four
/// compiles, closing that depth axis: the helper is the immediate invoker here.
/// The caller/callee refinement closes the remaining breadth axis across its six
/// direct callers.
///
/// CLAIMED: branch 3' is closed because each callee is a source-program-authored
/// direct HostResult `Match` body. THE GAP: branch 1 versus the durable-lane
/// branch remains open on closure pre-existence. This does not choose a repair
/// or infer its owner. The caller/callee identities are transition sentinels
/// for this measured route and must be rewritten if an authorized lowering
/// change moves it.
///
/// D3 MEASURED: the depth-2 plan's source-occurrence table contains the typed
/// identity at ordinal 5, and it renders as `StaticOriginId(5)`. CLAIMED: that
/// assertion runs before the four-row table, so a stale key is loud even for
/// the two rows whose expected origin-5 crossing is absent. THE GAP: existence
/// identifies the probe key, not the meaning of the occurrence it names.
///
/// D4 MEASURED: `(origin -> crossing)` is retained as a relation. The table
/// below pins the whole crossing vector per row; it does not panic under a false
/// general law when another fixture gives one origin multiple crossings.
///
/// `RT-CLOSURE-BOUNDARY-LANE` D1/D2: depth 2 and depth 3 separately reach
/// `transfer_into_carrier` from `carry_call_input`, as the exact callee bodies
/// and caller tags below assert. Each caller is handing a source-authored
/// closure-bearing argument to a generated unit inside the live runtime, not
/// publishing a durable or serialized artifact. Both therefore route through
/// `41-values.md:76-83`'s live-domain clause. The repair attempt stops at B2F's
/// closed carrier language: generated-unit parameters are one `ValueWord`, but
/// no invocation-owned tag/class row represents `Closure`; the only closure row
/// is the explicitly retired persistent lane. B2F directly calls a statically
/// selected closure body, but cannot carry this first-class closure-bearing
/// argument without a new representation plus owner/domain/liveness authority.
/// The exact refusal therefore remains conservative until that successor exists.
///
/// Promise class: transition sentinel. The exact origin and path are the
/// measured residual; the invoking-site tag is the measured route. Rewrite this
/// table when an authorized lowering repair moves either row; do not preserve
/// it as a permanent shape requirement.
///
/// `RT-SYNTHESIZED-ENV-RECORD-OCCURRENCE` D0-D2: the planner-issued empty
/// environment Record replaces the closure child on both enabled rows. The
/// origin-5 transfer therefore retains the same caller and callee identities but
/// has no closure path, passes aggregate reconciliation and reaches the opaque
/// unit-result transfer. Both rows then return to the downstream
/// `StaticWorkerBinding` refusal.
///
/// MEASURED (2026-08-15, exact
/// `257a9ddcc78c1a4fcebccac7048dc8a049efa301`): the existing non-ignored
/// source-compilation paths exercised by
/// `scripts/ken-cargo test -p ken-cli --tests -- --nocapture --test-threads=1`
/// exclude every test marked `#[ignore]` by the corpus's membership rule,
/// regardless of its stated reason. At exact
/// `c88a5e423bb61669ab8a1f3421bdcb610ba992f9`, that was 33 exclusions. Of
/// those, the six closure-at-boundary tests were measured individually: each
/// produced three completed returns with `unit_boundary_environment_fields={}`
/// before its expected `Closure` refusal — an outcome that refusal makes forced
/// rather than observed. Six assumed behaviours became six measured ones; the
/// shape-bearing population remains unmeasured. The selected non-ignored paths
/// returned `unit_boundary_environment_fields={}` on all 81 completed returns
/// across 15 processes; the plans contained 7 through 301 source occurrences.
/// This is a scoped corpus measurement, not a universal property of Ken
/// programs.
///
/// CLAIMED: these hand-authored rows pin the lowering's internal IR contract:
/// a planner-issued unit-boundary environment Record is transferable and does
/// not change the later static-worker disposition. These two row4 controls are
/// the only current exercise of this complete mechanism and establish no
/// source-language capability.
/// GAP: this internal transition does not repair the later static-worker wall.
#[test]
fn required_consumer_route_manufactures_the_depth_two_plus_closure_crossing() {
    use crate::cranelift_backend::lowering::core::with_required_consumer_route_suppressed;
    use crate::cranelift_backend::lowering::{
        d2k_owner_trace_take, BoundaryTransferInvokingSite, D2kOwnerEvent,
        GeneratedUnitCallInputCallee, GeneratedUnitCallInputCaller,
    };

    #[derive(Debug, Eq, PartialEq)]
    struct Crossing {
        origin: StaticOriginId,
        root_kind: &'static str,
        root_to_closure_path: Option<String>,
        invoking_site: BoundaryTransferInvokingSite,
    }

    #[derive(Debug, Eq, PartialEq)]
    struct Observed {
        label: &'static str,
        outcome: String,
        suppressions: usize,
        origin_5_crossings: Vec<Crossing>,
        closure_child_present: bool,
        origin_5_transfer_into_carrier_reached: bool,
        unit_result_transfer_reached: Option<bool>,
    }


    fn compile(depth: usize) -> (String, Vec<D2kOwnerEvent>, bool) {
        crate::cranelift_backend::lowering::reset_d5a_trace();
        let _ = d2k_owner_trace_take();
        let expression =
            host_result_closure_match(px8j_scope_chain_observation_result(depth, 0));
        let (result, _trace) = px8j_capture_source_trace(
            &expression,
            false,
            &format!("ken_required_consumer_d5_depth{depth}"),
        );
        let outcome = match result {
            Err(CraneliftBackendError::Unsupported(UnsupportedLowering {
                construct, ..
            })) => construct.to_string(),
            Err(CraneliftBackendError::Backend(
                BackendFailure::PlannerInvariant(reason),
            ))
                if reason == "aggregate producer has no planned ownership record" =>
            {
                "AggregateProducerOwnership".to_string()
            }
            Ok(_) => "compiled".to_string(),
            Err(other) => format!("other:{other}"),
        };
        let unit_result_transfer_reached =
            crate::cranelift_backend::lowering::take_d5a_trace()
                .iter()
                .any(|event| event.contains("UNIT-RESULT transfer"));
        (
            outcome,
            d2k_owner_trace_take(),
            unit_result_transfer_reached,
        )
    }

    fn observe(
        label: &'static str,
        depth: usize,
        suppressed: bool,
        target_origin: StaticOriginId,
    ) -> Observed {
        let ((outcome, events, unit_result_transfer_reached), suppressions) =
            if suppressed {
                with_required_consumer_route_suppressed(|| compile(depth))
            } else {
                (compile(depth), 0)
            };
        let origin_5_crossings = events
            .into_iter()
            .filter_map(|event| match event {
                D2kOwnerEvent::BoundaryTransferEntered {
                    origin,
                    root_kind,
                    closure_path,
                    invoking_site,
                } if origin == target_origin => Some(Crossing {
                    origin,
                    root_kind,
                    root_to_closure_path: closure_path,
                    invoking_site,
                }),
                _ => None,
            })
            .collect::<Vec<_>>();
        let closure_child_present = origin_5_crossings
            .iter()
            .any(|crossing| crossing.root_to_closure_path.is_some());
        let origin_5_transfer_into_carrier_reached = !origin_5_crossings.is_empty();
        Observed {
            label,
            outcome,
            suppressions,
            origin_5_crossings,
            closure_child_present,
            origin_5_transfer_into_carrier_reached,
            unit_result_transfer_reached: (!suppressed)
                .then_some(unit_result_transfer_reached),
        }
    }

    fn expected_source_callee(depth: usize, ordinal: usize) -> StaticOriginId {
        let expression = host_result_closure_match(px8j_scope_chain_observation_result(depth, 0));
        let plan = plan_static_transition_graph(&expression, &BTreeMap::new())
            .expect("the callee-identity fixture must plan");
        let callee = plan
            .source_occurrence_origin_at_ordinal_for_test(ordinal)
            .expect("the measured callee body must remain in the planned source population");
        let RuntimeExpr::Match { default, .. } = plan
            .source_occurrence(callee)
            .expect("the measured callee must be a source occurrence")
        else {
            panic!("the measured callee must remain the direct HostResult Match body")
        };
        assert_eq!(
            default.message, "direct HostResult default",
            "the measured callee must remain the source program's direct HostResult body",
        );
        callee
    }

    let origin_existence_expression =
        host_result_closure_match(px8j_scope_chain_observation_result(2, 0));
    let origin_existence_plan =
        plan_static_transition_graph(&origin_existence_expression, &BTreeMap::new())
            .expect("the row-4 depth-2 existence fixture must plan");
    let origin_5 = origin_existence_plan
        .source_occurrence_origin_at_ordinal_for_test(5)
        .expect(
            "the origin-5 probe key must name a planned source occurrence before any row may \
             assert that its crossing is absent, including rows expecting false",
        );
    assert_eq!(
        format!("{origin_5:?}"),
        "StaticOriginId(5)",
        "the row-independent plan lookup must identify the exact hardcoded probe key",
    );
    let depth_2_callee = expected_source_callee(2, 49);
    let depth_3_callee = expected_source_callee(3, 59);
    assert_eq!(format!("{depth_2_callee:?}"), "StaticOriginId(49)");
    assert_eq!(format!("{depth_3_callee:?}"), "StaticOriginId(59)");
    assert_eq!(
        [
            observe("row4-depth-2/enabled", 2, false, origin_5),
            observe("row4-depth-2/suppressed", 2, true, origin_5),
            observe("row4-depth-3/enabled", 3, false, origin_5),
            observe("row4-depth-3/suppressed", 3, true, origin_5),
        ],
        [
            Observed {
                label: "row4-depth-2/enabled",
                outcome: "StaticWorkerBinding".to_string(),
                suppressions: 0,
                origin_5_crossings: vec![Crossing {
                    origin: origin_5,
                    root_kind: "Constructor",
                    root_to_closure_path: None,
                    invoking_site: BoundaryTransferInvokingSite::GeneratedUnitCallInput {
                        caller: GeneratedUnitCallInputCaller::SourceLexicalClosureArgument,
                        callee: GeneratedUnitCallInputCallee::Body(depth_2_callee),
                    },
                }],
                closure_child_present: false,
                origin_5_transfer_into_carrier_reached: true,
                unit_result_transfer_reached: Some(true),
            },
            Observed {
                label: "row4-depth-2/suppressed",
                outcome: "StaticWorkerBinding".to_string(),
                suppressions: 1,
                origin_5_crossings: Vec::new(),
                closure_child_present: false,
                origin_5_transfer_into_carrier_reached: false,
                unit_result_transfer_reached: None,
            },
            Observed {
                label: "row4-depth-3/enabled",
                outcome: "StaticWorkerBinding".to_string(),
                suppressions: 0,
                origin_5_crossings: vec![Crossing {
                    origin: origin_5,
                    root_kind: "Constructor",
                    root_to_closure_path: None,
                    invoking_site: BoundaryTransferInvokingSite::GeneratedUnitCallInput {
                        caller: GeneratedUnitCallInputCaller::SourceLexicalClosureArgument,
                        callee: GeneratedUnitCallInputCallee::Body(depth_3_callee),
                    },
                }],
                closure_child_present: false,
                origin_5_transfer_into_carrier_reached: true,
                unit_result_transfer_reached: Some(true),
            },
            Observed {
                label: "row4-depth-3/suppressed",
                outcome: "StaticWorkerBinding".to_string(),
                suppressions: 1,
                origin_5_crossings: Vec::new(),
                closure_child_present: false,
                origin_5_transfer_into_carrier_reached: false,
                unit_result_transfer_reached: None,
            },
        ],
        "D5 must preserve the exact enabled/suppressed origin-5 crossing table",
    );
}

/// `RT-PLANNED-CLOSURE-PREEXISTENCE` D1/D2: ask the planner whether the
/// origin-5 result already has a closure-typed field, without executing the
/// projected route or observing a crossing.
///
/// MEASURED: for row 4 at depths 2 and 3, the planner's closed
/// result-producer analysis selects source `Construct` origin 12, whose field
/// zero is a `LexicalClosure`. Changing that already-real source field to an
/// integer changes the planner-side classification to `Other`.
///
/// CLAIMED: the closure shape pre-exists in both governed source plans, so both
/// rows select the durable-lane branch and become rows of
/// `RT-CLOSURE-BOUNDARY-LANE`. That node's sizing population is therefore its
/// original explicit escape row plus these two recursor rows, not the original
/// row alone; its numeric size remains for that node to frame.
///
/// THE GAP: this classifies source construction and routes the rows. It does
/// not select the durable-lane repair, infer ownership beyond that routing, or
/// promise that origin 12 remains stable after an authorized planner rewrite.
///
/// Promise class: transition sentinel. Rewrite the exact origins when an
/// authorized planning change moves the producer; do not preserve the ordinal
/// as a permanent ABI.
#[test]
fn planned_closure_preexistence_routes_recursors_to_the_durable_lane() {
    use crate::cranelift_backend::planning::PlannedResultFieldKindForTest;

    fn classify(expression: &RuntimeExpr) -> (bool, Vec<(String, PlannedResultFieldKindForTest)>) {
        let plan = plan_static_transition_graph(expression, &BTreeMap::new())
            .expect("the governed recursor expression must plan");
        let origin = plan
            .source_occurrence_origin_at_ordinal_for_test(5)
            .expect("the planner-side probe must retain source origin 5");
        let (closed, fields) = plan
            .planned_result_field_kinds_for_test(origin, 0)
            .expect("the planner must classify origin 5's closed result producers");
        (
            closed,
            fields
                .into_iter()
                .map(|(producer, kind)| (format!("{producer:?}"), kind))
                .collect(),
        )
    }

    for (label, depth) in [("row4-depth-2", 2), ("row4-depth-3", 3)] {
        let expression = host_result_closure_match(px8j_scope_chain_observation_result(depth, 0));
        assert_eq!(
            classify(&expression),
            (
                true,
                vec![(
                    "StaticOriginId(12)".to_string(),
                    PlannedResultFieldKindForTest::LexicalClosure,
                )],
            ),
            "{label}: field zero must be closure-typed by source construction",
        );
    }

    let mut changed = host_result_closure_match(px8j_scope_chain_observation_result(2, 0));
    let RuntimeExpr::Call { args, .. } = &mut changed else {
        panic!("the real HostResult wrapper must remain a Call")
    };
    let RuntimeExpr::ComputationalMatch { cases, .. } = &mut args[0] else {
        panic!("the real depth-2 result must remain a ComputationalMatch")
    };
    let RuntimeExpr::Let { body, .. } = &mut cases[0].body else {
        panic!("the real selected Node case must retain its Let")
    };
    let RuntimeExpr::Construct { args, .. } = body.as_mut() else {
        panic!("the real selected Node case must return a Construct")
    };
    args[0] = RuntimeExpr::Value(RuntimeValue::Int(0.into()));
    let (closed, changed_fields) = classify(&changed);
    assert!(
        closed,
        "the source-field mutation must keep the producer set closed"
    );
    assert_eq!(
        changed_fields
            .iter()
            .map(|(_, kind)| *kind)
            .collect::<Vec<_>>(),
        vec![PlannedResultFieldKindForTest::Other],
        "the planner-side classifier must distinguish a non-closure source field",
    );
}

/// `RT-PLANNED-CLOSURE-PREEXISTENCE` D3: a test-only diagnostic lookup cannot
/// alter the compile outcome it exists to report.
///
/// Population predicate: an exact `#[cfg(test)]` binding in `core.rs` whose
/// initializer contains `?`, so that error propagation can change test-profile
/// control flow. The base `ad47054a5` census is 14 bindings:
/// `_aggregate_relation`, `_effect_seats`, `(returned, _call)`, `vector`, two
/// outer `target` bindings, two nested `substitute` bindings, `claimed_owner`,
/// `discharge`, `callee`, two `callee_body_origin` bindings, and `_`. Eleven
/// are mutation/control machinery whose refusal is their intended behavior;
/// the three call-input callee tags are diagnostic-only and are the repaired
/// population. The five production `child_static_origin(...)?` calls are not
/// members because they are not test-only bindings.
///
/// MEASURED: forcing the diagnostic's real planner lookup to a missing child
/// preserves the exact `StaticWorkerBinding` compile outcome while changing the
/// call-input tag from `Body` to `MissingBodyChildByMutation` and recording a
/// non-zero mutation hit.
/// CLAIMED: missing diagnostic metadata degrades only the tag. THE GAP: this
/// says nothing about a missing production child or a production compile.
#[test]
fn missing_call_input_callee_child_degrades_the_tag_not_the_compile() {
    use crate::cranelift_backend::lowering::{
        d2k_owner_trace_take, BoundaryTransferInvokingSite, CallInputCalleeDiagnosticMutationGuard,
        D2kOwnerEvent, GeneratedUnitCallInputCallee, GeneratedUnitCallInputCaller,
    };

    fn run(mutate: bool) -> (String, u32, Vec<GeneratedUnitCallInputCallee>) {
        let _ = d2k_owner_trace_take();
        let guard = mutate.then(CallInputCalleeDiagnosticMutationGuard::install);
        let expression = host_result_closure_match(px8j_scope_chain_observation_result(2, 0));
        let (result, _trace) =
            px8j_capture_source_trace(&expression, false, "ken_planned_closure_d3");
        let hits = guard
            .as_ref()
            .map_or(0, CallInputCalleeDiagnosticMutationGuard::hits);
        drop(guard);
        let callees = d2k_owner_trace_take()
            .into_iter()
            .filter_map(|event| match event {
                D2kOwnerEvent::BoundaryTransferEntered {
                    origin,
                    invoking_site:
                        BoundaryTransferInvokingSite::GeneratedUnitCallInput {
                            caller: GeneratedUnitCallInputCaller::SourceLexicalClosureArgument,
                            callee,
                        },
                    ..
                } if format!("{origin:?}") == "StaticOriginId(5)" => Some(callee),
                _ => None,
            })
            .collect();
        let outcome = match result {
            Err(CraneliftBackendError::Unsupported(UnsupportedLowering { construct, .. })) => {
                construct.to_string()
            }
            Ok(_) => "compiled".to_string(),
            Err(other) => format!("other:{other}"),
        };
        (outcome, hits, callees)
    }

    let baseline = run(false);
    let missing = run(true);
    assert_eq!(baseline.0, "StaticWorkerBinding");
    assert_eq!(
        missing.0, baseline.0,
        "the diagnostic mutation must not change compilation"
    );
    assert_eq!(baseline.1, 0, "the baseline installs no mutation");
    assert!(
        missing.1 > 0,
        "the missing-child mutation must reach a real lookup"
    );
    let [GeneratedUnitCallInputCallee::Body(body)] = baseline.2.as_slice() else {
        panic!(
            "the baseline must retain one exact body-level tag: {:?}",
            baseline.2
        )
    };
    assert_eq!(format!("{body:?}"), "StaticOriginId(49)");
    let [GeneratedUnitCallInputCallee::MissingBodyChildByMutation { entry }] =
        missing.2.as_slice()
    else {
        panic!(
            "the missing planner child must degrade the exact call-input tag: {:?}",
            missing.2
        )
    };
    assert_ne!(
        entry, body,
        "the degraded tag must retain the closure entry rather than mislabel it as its body",
    );
}

/// AC-2: replacing the forward-selected outer case body with the
/// continuation's own occurrence is rejected by the independent direct-body
/// derivation before the plan can escape.
#[test]
fn contkey_wrong_own_occurrence_seed_is_rejected() {
    use crate::cranelift_backend::planning::with_continuation_consuming_occurrence_seed_mutated;

    let expression =
        host_result_closure_match(px8j_scope_chain_observation_result(1, 0));
    let declarations = BTreeMap::new();
    let error = with_continuation_consuming_occurrence_seed_mutated(|| {
        match plan_static_transition_graph(&expression, &declarations) {
            Ok(_) => panic!("the continuation's own occurrence must not pass as its consumer"),
            Err(error) => error,
        }
    });
    let rendered = format!("{error:?}");
    eprintln!("contkey body_origin mutation refusal: {rendered}");
    assert!(
        rendered.contains(
            "a continuation specialization's consuming occurrence has a mismatched body_origin: it is not the exact outer selected case body derived from its eliminator"
        ),
        "the body_origin mutation must produce its field-specific consuming-occurrence refusal: {rendered}",
    );
    assert!(
        !rendered.contains("mismatched eliminator_origin"),
        "the body_origin refusal must not report the other relation field: {rendered}",
    );
}

/// `RT-CONTKEY-ELIMINATOR-ORIGIN-UNFIRED` AC-1/AC-3: replacing only the
/// selected relation's outer eliminator with the real inner match is rejected
/// by the position-zero guard, with a field-specific control message.
#[test]
fn contkey_wrong_inner_match_eliminator_seed_is_rejected() {
    use crate::cranelift_backend::planning::with_continuation_consuming_eliminator_seed_mutated;

    let expression =
        host_result_closure_match(px8j_scope_chain_observation_result(1, 0));
    let declarations = BTreeMap::new();
    let error = with_continuation_consuming_eliminator_seed_mutated(|| {
        match plan_static_transition_graph(&expression, &declarations) {
            Ok(_) => panic!("the inner match must not pass as the outer consuming eliminator"),
            Err(error) => error,
        }
    });
    let rendered = format!("{error:?}");
    eprintln!("contkey eliminator_origin mutation refusal: {rendered}");
    assert!(
        rendered.contains(
            "a continuation specialization's consuming occurrence has a mismatched eliminator_origin: it does not select the continuation as its position-zero child"
        ),
        "the eliminator_origin mutation must produce its field-specific consuming-occurrence refusal: {rendered}",
    );
    assert!(
        !rendered.contains("mismatched body_origin"),
        "the eliminator_origin refusal must not report the other relation field: {rendered}",
    );
}
