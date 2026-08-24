//! Positional/de-Bruijn binding order and continuation-candidate settlement/
//! fusion-local ledger end-to-end (`RT-CONTROL-INTEGRATION-TESTS-SPLIT` D1,
//! module 5 of 5, split from `control.rs`: `d9b_*`, the positional `d3_*`
//! cluster, `ccr_d3_*`, `coc_d3_*`, `sar_d3_*`, `ced_d2_*`/`ced_d1_*`/
//! `ced_d3_*` (second wave, the `m1..m5` mutation-row cluster),
//! `call_edge_executability_axis_*`, `d2b_*`, `d2k_*`,
//! `r3_the_base_uncomposed_*`, `dp_composition_time_*`, `ac_d3_self_*`,
//! `d3_the_fusion_local_composition_ledger_*`).

use super::*;
use super::control::{
    px8j_aggregate_result, px8j_capture_source_trace, px8j_recursive_sibling_result,
    px8j_scope_chain_observation_result,
};
use super::effects::{BorrowedFixtureValue, RootIngressFixture};
use super::source_frame_bridge::d8f_compile;
use crate::cranelift_backend::lowering::units::{
    srcbody_bind_order_take, SrcbodyBindHost, SrcbodyBindOrderObservation,
};
use crate::RuntimeSymbolMetadata;

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2k-0` — the five `StaticWorkerBinding`
/// walls, re-derived, with their edges and routes.**
///
/// The frame's trap is that **all five report the same sentence, and that is
/// not evidence of one root**: the refusal is emitted by a single chokepoint
/// every value-producing read funnels through, so five unrelated wrong-consumer
/// routes would produce the same string. The discriminator is the `edge`
/// argument and the causal consumer owner, never the message.
///
/// **MEASURED at this base — one root.** All five refuse at the **same call
/// site**, with the **same edge**, through **byte-identical lowering routes**.
///
/// | expression | edge | refusing caller | route |
/// |---|---|---|---|
/// | row 1, owned-scope deletion | `a Var in value position` | `core.rs:14593` | `lower_expr` child collection |
/// | row 4, `scope_segments` depth 1 | `a Var in value position` | `core.rs:14593` | same |
/// | row 4, `scope_segments` depth 2 | `a Var in value position` | `core.rs:14593` | same |
/// | row 4, `scope_segments` depth 3 | `a Var in value position` | `core.rs:14593` | same |
/// | row 5, after-hole | `a Var in value position` | `core.rs:14593` | same |
///
/// **`value_at` has exactly FOUR callers at this base**, re-counted rather than
/// carried from the frame: `core.rs:6200` (*"a source-machine Var in value
/// position"*), `core.rs:11140` (*"a continuation capture input"*),
/// `core.rs:14593` (*"a Var in value position"*), and `mod.rs:3661` (which
/// forwards its caller's edge). **No fifth caller**, so that scope signal does
/// not fire.
///
/// **What is executable here and what is not, stated rather than blurred.** The
/// edge and the refusal are asserted below, per expression. **The route is
/// not**, and cannot be: observing which caller fired requires instrumenting
/// `value_at`, and `AC-2` requires that guard to be byte-unchanged. So the
/// route column is a **measured-and-reverted probe**, and its method is
/// recorded here so it can be repeated rather than trusted:
///
/// - each `value_at` caller was temporarily tagged with its file and line, and
///   each of the five compiled under `B`-only exclusion. In every one the last
///   tag before the refusal was `core.rs:14593`; `mod.rs:3661` never fired.
/// - a `Backtrace::force_capture()` at the `StaticWorker` arm, filtered to
///   lowering frames, was **identical across all five**: the read is taken in
///   `lower_expr`'s child-argument collection, reached directly from
///   `compile_expr_into_module_with_root_projection`.
/// - both probes were reverted; the committed tree touches neither `value_at`
///   nor any caller.
///
/// **The limit of that measurement, since it is the load-bearing half.** The
/// backtrace was filtered to lowering frames and truncated at sixteen. Identical
/// filtered traces are strong evidence of one route and are **not** a proof that
/// no difference exists above the truncation. A reader who wants to raise this
/// to certainty should re-run the probe unfiltered rather than take this row.
///
/// ⇒ **One root, on this evidence.** `D2k-1b` may be sized as a single repair at
/// the owning consumer before the guard, rather than one cut per row.
///
/// **Promise class: durable invariant.** It asserts the wall these five stand
/// at and the edge that identifies it. It reds when a repair moves any of them
/// off that wall, which is the intended signal, and `D2k-1b` is what retires it.
/// **`D2k-0`'s wall predicate, hoisted so exactly one copy exists.**
///
/// It reads whatever selector exclusion is currently in force -- it sets none
/// itself -- which is what lets the redness control run the **same** predicate
/// with the wrong-consumer condition removed. A redness proof written against a
/// re-implemented twin would show only that the twin is not a constant.
fn d2k_wall_under_current_selector(
    expression: &RuntimeExpr,
    symbol: &str,
) -> Option<(String, String)> {
    let (result, _trace) = px8j_capture_source_trace(expression, false, symbol);
    match result {
        Ok(_) => None,
        Err(CraneliftBackendError::Unsupported(UnsupportedLowering { construct, reason })) => {
            // The edge is the prefix the caller supplied; the rest of the
            // sentence is the chokepoint's own and is shared by construction,
            // so it cannot discriminate anything.
            let edge = reason
                .split(" is a value-producing position")
                .next()
                .unwrap_or(&reason)
                .to_string();
            Some((construct.to_string(), edge))
        }
        Err(other) => Some(("<not-unsupported>".to_string(), format!("{other:?}"))),
    }
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D9b` — the assembled ordinary run corresponds
/// to the planner's role sequence BY EXACT ROLE POSITION, and every capture role
/// the planner can misstate is refused before emission.**
///
/// ## The independent side
///
/// The planner-issued `ContinuationOrdinaryEnvelopeRole` sequence, read from a
/// plan built separately from the emission run, and the assembler's own INPUT
/// authorities — the producer constructor's whole lowered field run and the
/// selected closure's ordered capture vector.
///
/// The expectation is then derived by the ruled law and nothing else:
///
/// ```text
/// role position i  ->  NonrecursiveField { source_position: p }  =>  fields[p]
///                      WorkerCapture     { ordinal: k }          =>  captures[k]
/// ```
///
/// ⛔ **The assembled run is never consulted to build it.** `fields` and
/// `captures` are what the assembler READ; `operands` is what it PRODUCED.
/// Deriving the expectation from `operands` would be an identity, which is the
/// defect the governing correction is about.
///
/// ## Typed and exact, never a shape tag
///
/// Both sides are typed: roles are `D9RoleKey`, operands are
/// `D9OperandIdentity` (phase + `LoweredVariant` + the SSA words and planner
/// origins the operand holds). ⛔ The string encoder this replaced collapsed
/// **every** unnamed `Lowered` arm to `"other"`, which was measured on this very
/// witness: both distinct `Int` captures encoded to `"specialized:other"`, so a
/// swap of their ordinals compared EQUAL and the discriminator could not have
/// failed for the reason it names.
///
/// ⭐ **The distinctness is asserted, not assumed.** `D9OperandIdentity` does not
/// claim global injectivity — two operands holding no SSA word at all compare
/// equal on content — so this row proves its own premise by requiring the two
/// capture identities to differ before it relies on a swap being observable.
/// Twice on this node an equal-value perturbation has been mistaken for a
/// missing guard; that is what this assertion exists to stop.
///
/// ## The refusal set
///
/// Each of the four capture-dependent perturbations moves ONE fact of the
/// planner's sequence, must APPLY on this witness, must be refused by the guard
/// that owns it, and must leave the perturbed unit with no assembled run at all
/// — the recorder is the last step before the call, so an absent record is an
/// assembly that never completed.
///
/// **Promise class: durable invariant.** The relation is keyed and derived from
/// the planner's own law, so it survives any extension that keeps the envelope
/// meaning what it says.
#[test]
fn d9b_the_assembled_ordinary_run_matches_the_planner_role_sequence_by_position() {
    use crate::cranelift_backend::lowering::{
        d9_assemblies, d9_role_key, d9_set_foreign_origin, reset_d9_assemblies,
        with_d9_envelope_mutation, D9EnvelopeMutation, D9OperandIdentity, D9RoleKey,
    };

    // The two-capture witness, armed for this row only and restored however
    // this test leaves. ⛔ Under `--test-threads=1` every row shares this
    // thread, so a switch left armed by a panic would silently re-shape the
    // fixture for whichever row ran next.
    struct ArmedWitness;
    impl Drop for ArmedWitness {
        fn drop(&mut self) {
            crate::cranelift_backend::test_objects::set_px8tr_worker_captures(false);
            crate::cranelift_backend::lowering::d9_set_foreign_origin(None);
        }
    }
    crate::cranelift_backend::test_objects::set_px8tr_worker_captures(true);
    let _armed = ArmedWitness;

    let emit = |name: &str| {
        crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(name, false)
            .map(|_| ())
            .map_err(|error| format!("{error:?}"))
    };

    // ── The independent side: the planner's own facts, per unit ────────────
    let planned = with_d5a_witness_plan(|plan| {
        plan.continuation_units()
            .expect("continuation units")
            .into_iter()
            .map(|unit| {
                (
                    unit.id(),
                    (
                        unit.ordinary_envelope()
                            .expect("the planner's own envelope")
                            .iter()
                            .map(d9_role_key)
                            .collect::<Vec<_>>(),
                        unit.ordinary_parameters() as usize,
                        unit.worker_capture_count(),
                        unit.worker_closure_origin(),
                    ),
                )
            })
            .collect::<BTreeMap<_, _>>()
    });

    // ── THE WITNESS PREMISE ────────────────────────────────────────────────
    //
    // ⛔ Stated first and asserted, because every refusal below is about a
    // capture role. A witness with no capture-bearing unit would make the whole
    // refusal set decline, and a row that then asserted "zero applications"
    // would be recording its own vacuity as a result.
    let capture_bearing = planned
        .iter()
        .filter(|(_, (_, _, captures, _))| *captures >= 2)
        .map(|(id, _)| *id)
        .collect::<Vec<_>>();
    assert!(
        !capture_bearing.is_empty(),
        "⛔ THE PREMISE FAILED: no continuation unit declares two or more worker captures, so the \
         swapped-ordinal discriminator has nothing to exchange. The armed fixture is supposed to \
         give the selected worker two captures; planned units are {planned:?}"
    );
    let perturbed_unit = capture_bearing[0];
    let capture_origin = planned[&perturbed_unit].3;
    // A REAL origin naming another closure, drawn from the plan's own worker
    // population. ⛔ Not a fabricated id, which could be refused merely for
    // being unknown rather than for naming the wrong closure.
    let foreign_origin = planned
        .values()
        .map(|entry| entry.3)
        .find(|origin| *origin != capture_origin)
        .expect(
            "the foreign-closure perturbation needs a second, different worker closure occurrence \
             in the plan; with only one it would silently become the identity",
        );
    d9_set_foreign_origin(Some(foreign_origin));

    // ── THE EXACT POSITIVE ─────────────────────────────────────────────────
    reset_d9_assemblies();
    emit("ken_d9b_exact").expect("THE EXACT POSITIVE: the two-capture witness compiles");
    let assembled = d9_assemblies();
    assert!(
        !assembled.is_empty(),
        "no continuation ordinary run was assembled, so every clause below would be vacuous"
    );

    for run in &assembled {
        let (expected_roles, declared, _, _) = planned
            .get(&run.unit)
            .unwrap_or_else(|| panic!("{:?} assembled a run the planner defines no unit for", run.unit));
        assert_eq!(
            &run.roles, expected_roles,
            "the assembler must consume the PLANNER's role sequence for {:?}, in its order",
            run.unit
        );
        assert_eq!(
            run.operands.len(),
            run.roles.len(),
            "one operand per role, keyed by position -- a length disagreement means the run and \
             the sequence it was assembled from are not the same relation: {run:?}"
        );
        assert_eq!(
            run.operands.len(),
            *declared,
            "and the run's length is the continuation's independently declared ordinary-parameter \
             count: {run:?}"
        );

        // ⭐ THE KEYED RELATION. Derived from the roles and the assembler's
        // INPUTS by the ruled law, position by position.
        let expected = run
            .roles
            .iter()
            .map(|role| match role {
                D9RoleKey::NonrecursiveField { source_position } => run
                    .fields
                    .get(*source_position as usize)
                    .unwrap_or_else(|| {
                        panic!("role names source position {source_position}, outside {run:?}")
                    })
                    .clone(),
                D9RoleKey::WorkerCapture { ordinal, .. } => run
                    .captures
                    .get(*ordinal as usize)
                    .unwrap_or_else(|| {
                        panic!("role names capture ordinal {ordinal}, outside {run:?}")
                    })
                    .clone(),
            })
            .collect::<Vec<D9OperandIdentity>>();
        assert_eq!(
            run.operands, expected,
            "the assembled run must hold, at each role position, exactly the operand that \
             position's own authority names -- a nonrecursive field from its exact lowered source \
             position, a worker capture from the selected closure's run at that exact ordinal: \
             {run:?}"
        );
    }

    // ⭐ The premise the swap depends on: the two capture operands are
    // genuinely distinguishable, so exchanging their ordinals is a different
    // run rather than an observational identity.
    let capture_run = assembled
        .iter()
        .find(|run| run.unit == perturbed_unit)
        .unwrap_or_else(|| panic!("{perturbed_unit:?} declares captures but assembled no run"));
    assert!(
        capture_run.captures.len() >= 2,
        "the perturbed unit's selected closure must carry two captures: {capture_run:?}"
    );
    assert_ne!(
        capture_run.captures[0], capture_run.captures[1],
        "⛔ THE TWO CAPTURES ARE INDISTINGUISHABLE, so a swap of their ordinals would assemble an \
         identical run and the discriminator below would pass for the wrong reason: {capture_run:?}"
    );

    // ── THE REFUSAL SET ────────────────────────────────────────────────────
    //
    // Each perturbation must APPLY, be refused by the guard that owns it, and
    // leave the perturbed unit with no assembled run.
    for (mutation, owning_guard) in [
        (D9EnvelopeMutation::SwapCaptureOrdinals, "names ordinal"),
        (
            D9EnvelopeMutation::NonrecursiveAfterCaptures,
            "after a worker capture",
        ),
        (
            D9EnvelopeMutation::ForeignCaptureClosure,
            "names closure occurrence",
        ),
        (
            D9EnvelopeMutation::DropLastCaptureRole,
            "of the selected closure",
        ),
    ] {
        reset_d9_assemblies();
        let (outcome, applications) = with_d9_envelope_mutation(mutation, || emit("ken_d9b_refuse"));
        assert_eq!(
            applications, 1,
            "⛔ {mutation:?} did not apply on this witness. A perturbation that moved nothing \
             cannot be evidence about the guard that would have refused it"
        );
        let reason = outcome.expect_err(&format!(
            "{mutation:?} moved one fact of the planner's own role sequence and the compile still \
             succeeded, so the guard that owns that fact does not fail closed"
        ));
        assert!(
            reason.contains(owning_guard),
            "{mutation:?} must be refused by the guard that OWNS the fact it moved (naming \
             {owning_guard:?}); got {reason}"
        );
        assert!(
            !d9_assemblies().iter().any(|run| run.unit == perturbed_unit),
            "{mutation:?} was refused, but {perturbed_unit:?} still recorded a completed assembly. \
             The recorder is the last step before the call, so a record here means the refusal did \
             not stand between the perturbed envelope and emission: {:?}",
            d9_assemblies()
        );
    }
}

// ── `RT-SRCBODY-BIND-ORDER` `D3` — the source-body binding-order controls ──
//
// `D1` splits one walk into two orders: the ABI descriptor run
// (`defining_abi_operands`, declaration order, unchanged) and the semantic
// environment a source body is lowered against (de Bruijn, innermost first).
// These controls measure that split from outside, on running programs, at
// exact values.

const D3_BIND_CALLEE: &str = "decl:fixture::srcbody::two_parameter";
const D3_BIND_MIRROR: &str = "decl:fixture::srcbody::two_parameter_mirror";
/// The two arguments, distinct and both single-digit so the positional
/// encoding below stays a small positive number in every reading. A negative
/// result does not round-trip as a process exit code, so the encoding is
/// deliberately arranged never to produce one.
const D3_BIND_FIRST: i64 = 7;
const D3_BIND_SECOND: i64 = 3;

/// A transparent declaration whose two-parameter source body reads BOTH of its
/// positions in one expression: `high * 10 + low`.
///
/// The result is a two-digit number whose tens digit is the parameter at
/// `high` and whose units digit is the one at `low`, so a single exit code
/// names both bindings at once and a swap of the two lands on a different
/// number rather than on a different magnitude of the same one.
///
/// Under the de Bruijn reading `lower_expr` implements, `Var(0)` is the
/// innermost binder — the LAST declared parameter — so
/// `d3_two_parameter_declaration(sym, 1, 0)` encodes `first` in the tens place.
/// Under the descriptor-order reading `D1` retires, the same expression encodes
/// `second` there.
fn d3_two_parameter_declaration(symbol: &str, high: u32, low: u32) -> RuntimeDeclaration {
    RuntimeDeclaration {
        symbol: symbol.to_string(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["first".to_string(), "second".to_string()],
                body: Box::new(RuntimeExpr::PrimitiveCall {
                    primitive: RuntimePrimitive {
                        symbol: "add_int".to_string(),
                        partiality: RuntimePartiality::Total,
                    },
                    args: vec![
                        RuntimeExpr::PrimitiveCall {
                            primitive: RuntimePrimitive {
                                symbol: "mul_int".to_string(),
                                partiality: RuntimePartiality::Total,
                            },
                            args: vec![
                                RuntimeExpr::Var(high),
                                RuntimeExpr::Value(RuntimeValue::Int((10).into())),
                            ],
                        },
                        RuntimeExpr::Var(low),
                    ],
                }),
            },
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(crate::RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    }
}

/// Runs `ExitFailure(<symbol>(7, 3))` as a whole process and returns its exit
/// code.
///
/// Both arguments are ordinary `Int` values passed through the declared ABI, so
/// the callee reads them out of its own activation frame — which is the run
/// whose order `D1` converts.
fn d3_run_two_parameter(declaration: &RuntimeDeclaration) -> i64 {
    let mut declarations = BTreeMap::new();
    declarations.insert(declaration.symbol.as_str(), declaration);
    let program = RuntimeExpr::Construct {
        constructor: crate::EXIT_FAILURE_CONSTRUCTOR.to_string(),
        args: vec![RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::DeclarationRef {
                symbol: declaration.symbol.clone(),
            }),
            args: vec![
                RuntimeExpr::Value(RuntimeValue::Int(D3_BIND_FIRST.into())),
                RuntimeExpr::Value(RuntimeValue::Int(D3_BIND_SECOND.into())),
            ],
        }],
    };
    let compiled = compile_expr_into_module(
        new_jit_module().expect("JIT module"),
        "d3_two_parameter_binding",
        Linkage::Local,
        &program,
        &NativeSeedEnvironment::empty(),
        declarations,
        None,
        true,
        None,
        Some(test_only_distinguished_root_join_plan()),
        None,
    )
    .expect("the two-parameter binding fixture lowers");
    let input = BorrowedFixtureValue {
        kind: 1,
        tag: 0,
        data: std::ptr::null(),
        len: 0,
    };
    let mut host_context = ();
    let invocation = RootIngressFixture {
        process_input: &input,
        host_context: (&mut host_context as *mut ()).cast(),
        capability: 0,
    };
    compiled
        .run(Some((&invocation as *const RootIngressFixture).cast()))
        .expect("the two-parameter binding fixture runs")
        .1
        .expect("the two-parameter binding fixture returns an exit code")
}

/// **`D3` control 1 — a two-parameter source declaration reads BOTH of its
/// positions, and each resolves to the parameter the source named.**
///
/// MEASURED: two whole-process fixtures compile and RUN. They differ in nothing
/// but which `Var` index sits in the tens place of `high * 10 + low`. Called
/// with `(first, second) = (7, 3)`, the body `Var(1) * 10 + Var(0)` exits `73`
/// and the mirror body `Var(0) * 10 + Var(1)` exits `37`.
///
/// CLAIMED: the semantic environment a `CallableDeclaration` body is lowered
/// against is `reverse(Parameter run)` — `Var(0)` is the last declared
/// parameter and `Var(1)` the first.
///
/// The pair is the control and neither half is sufficient. `73` alone is
/// equally green under an implementation that reversed nothing and was handed
/// its arguments in the other order; the mirror pins the digit order to the
/// `Var` index rather than to the call site. Both are EXACT values, and each
/// half's wrong answer is the OTHER half's right answer — a two-digit number,
/// not a trap, a zero, or a truncation, so neither can pass by failing.
///
/// Red before green: at `21fd46dc` each half returns the other's value.
///
/// Promise class: **durable invariant** — it asserts the source language's own
/// binder discipline, which no intended extension of the ABI layout may change.
/// The literals are the fixture's own arguments, not a pinned ABI fact.
#[test]
fn d3_a_two_parameter_source_declaration_binds_its_positions_in_de_bruijn_order() {
    let direct = d3_two_parameter_declaration(D3_BIND_CALLEE, 1, 0);
    let mirror = d3_two_parameter_declaration(D3_BIND_MIRROR, 0, 1);
    assert_eq!(
        d3_run_two_parameter(&direct),
        D3_BIND_FIRST * 10 + D3_BIND_SECOND,
        "`Var(1) * 10 + Var(0)` must put the FIRST declared parameter in the tens place: Var(0) \
         is the innermost binder, which is the LAST declared parameter"
    );
    assert_eq!(
        d3_run_two_parameter(&mirror),
        D3_BIND_SECOND * 10 + D3_BIND_FIRST,
        "the mirror body must reverse the digits; if both halves agree, the fixture is not \
         reading its Var indices and control 1 measures nothing"
    );
}

const D3_ROLE_CALLEE: &str = "decl:fixture::srcbody::role_discriminator";
const D3_ROLE_MIRROR: &str = "decl:fixture::srcbody::role_discriminator_mirror";
/// The two role-shaped constructors. They stand in for the process root's own
/// `ProcessInput` / `ProgramCaps` pair: same arity, same carrier, distinct
/// identity — so the only thing that can select between them is WHICH operand
/// arrived, never how it is shaped.
const D3_ROLE_INPUT: &str = "ctor:fixture::srcbody::ProcessInputLike";
const D3_ROLE_CAPS: &str = "ctor:fixture::srcbody::ProgramCapsLike";
const D3_ROLE_INPUT_PAYLOAD: i64 = 11;
const D3_ROLE_CAPS_PAYLOAD: i64 = 37;

/// A two-parameter declaration that matches ONE of its parameters against ONE
/// constructor and returns the bound field, trapping on anything else.
///
/// `scrutinee` selects which `Var` index is matched and `constructor` which
/// identity the single case names. The default is closed, so a body handed the
/// other parameter cannot silently fall through to a value.
fn d3_role_declaration(symbol: &str, scrutinee: u32, constructor: &str) -> RuntimeDeclaration {
    RuntimeDeclaration {
        symbol: symbol.to_string(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["process_input".to_string(), "program_caps".to_string()],
                body: Box::new(RuntimeExpr::Match {
                    scrutinee: Box::new(RuntimeExpr::Var(scrutinee)),
                    cases: vec![RuntimeMatchCase {
                        constructor: constructor.to_string(),
                        binders: 1,
                        // The BOUND field, not a literal: a constant body would
                        // be green even if the wrong operand had been selected
                        // by a case list that matched anything.
                        body: RuntimeExpr::Var(0),
                    }],
                    default: RuntimeTrap {
                        code: RuntimeTrapCode::PatternMatchFailure,
                        message: "d3 role discriminator default".to_string(),
                    },
                }),
            },
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(crate::RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    }
}

/// Runs `ExitFailure(<symbol>(ProcessInputLike(11), ProgramCapsLike(37)))` and
/// returns its exit code.
fn d3_run_role_discriminator(declaration: &RuntimeDeclaration) -> i64 {
    let mut declarations = BTreeMap::new();
    declarations.insert(declaration.symbol.as_str(), declaration);
    let role = |constructor: &str, payload: i64| RuntimeExpr::Construct {
        constructor: constructor.to_string(),
        args: vec![RuntimeExpr::Value(RuntimeValue::Int(payload.into()))],
    };
    let program = RuntimeExpr::Construct {
        constructor: crate::EXIT_FAILURE_CONSTRUCTOR.to_string(),
        args: vec![RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::DeclarationRef {
                symbol: declaration.symbol.clone(),
            }),
            args: vec![
                role(D3_ROLE_INPUT, D3_ROLE_INPUT_PAYLOAD),
                role(D3_ROLE_CAPS, D3_ROLE_CAPS_PAYLOAD),
            ],
        }],
    };
    let compiled = compile_expr_into_module(
        new_jit_module().expect("JIT module"),
        "d3_role_discriminator",
        Linkage::Local,
        &program,
        &NativeSeedEnvironment::empty(),
        declarations,
        None,
        true,
        None,
        Some(test_only_distinguished_root_join_plan()),
        None,
    )
    .expect("the role discriminator fixture lowers");
    let input = BorrowedFixtureValue {
        kind: 1,
        tag: 0,
        data: std::ptr::null(),
        len: 0,
    };
    let mut host_context = ();
    let invocation = RootIngressFixture {
        process_input: &input,
        host_context: (&mut host_context as *mut ()).cast(),
        capability: 0,
    };
    compiled
        .run(Some((&invocation as *const RootIngressFixture).cast()))
        .expect("the role discriminator fixture runs")
        .1
        .expect("the role discriminator fixture returns an exit code")
}

/// **`D3` control 2 — the `ProcessInput`/`ProgramCaps` discriminator: a
/// two-parameter body selects on CONSTRUCTOR IDENTITY, and each parameter
/// carries the operand its position named.**
///
/// This is `RT-ENTRY-TRAP-254`'s shape reduced to one crate. The defect there
/// was not a missing value: a well-formed operand of the right carrier arrived
/// at a `Match` that then found no case naming its constructor and took its
/// closed default. The two arguments here differ in NOTHING but identity — same
/// arity, same carrier, same payload shape — so the only thing that can decide
/// which case fires is which parameter the body bound.
///
/// MEASURED: two whole-process fixtures compile and RUN. `match Var(0) {
/// ProgramCapsLike(x) -> x }` exits `37`; `match Var(1) { ProcessInputLike(x)
/// -> x }` exits `11`.
///
/// CLAIMED: at a two-parameter source body, `Var(0)` is the SECOND declared
/// parameter and `Var(1)` the first — measured through constructor selection
/// rather than through arithmetic, so it holds for the carried-operand path
/// control 1 does not reach.
///
/// The pair is the control. Each half's default is closed, so under the
/// retired order neither half returns the other's number — both fall to the
/// match default, which is how the real defect presented. Asserting one half
/// alone would be green under a body that ignored its case list and always
/// took arm 0.
///
/// Red before green: at `21fd46dc` both halves take their default.
///
/// Promise class: **durable invariant** — a relation between which parameter a
/// source position names and which constructor arrives there. The payload
/// literals are the fixture's own arguments.
#[test]
fn d3_a_two_parameter_body_selects_the_constructor_its_position_named() {
    let caps_at_zero = d3_role_declaration(D3_ROLE_CALLEE, 0, D3_ROLE_CAPS);
    let input_at_one = d3_role_declaration(D3_ROLE_MIRROR, 1, D3_ROLE_INPUT);
    assert_eq!(
        d3_run_role_discriminator(&caps_at_zero),
        D3_ROLE_CAPS_PAYLOAD,
        "Var(0) is the innermost binder, so it must carry the SECOND declared parameter — the \
         caps-shaped operand — and its case must select"
    );
    assert_eq!(
        d3_run_role_discriminator(&input_at_one),
        D3_ROLE_INPUT_PAYLOAD,
        "Var(1) must carry the FIRST declared parameter — the input-shaped operand; if this \
         half also traps, both parameters resolve to one operand and the pair measures nothing"
    );
}


/// Compiles control 1's population and returns every semantic environment
/// production actually built, drained from the `D3` instrument.
fn d3_observed_bind_orders() -> Vec<SrcbodyBindOrderObservation> {
    let _ = srcbody_bind_order_take();
    let declaration = d3_two_parameter_declaration(D3_BIND_CALLEE, 1, 0);
    let _ = d3_run_two_parameter(&declaration);
    srcbody_bind_order_take()
}

/// **`D3` control 3 — the ROOT adapter was not reversed.**
///
/// `D1`'s conversion is owed to source parameter runs. The process root's
/// parameters are not one: they are the closed `ProcessInput`/`Capability`
/// ingress roles, resolved by `AbiProcessParameter` ordinal, so reversing them
/// would rename the two roles rather than reindex a binder.
///
/// MEASURED, from the environments PRODUCTION built while compiling control
/// 1's running population: every `SchedulingEntry` environment holds its
/// parameter ordinals in ascending descriptor order, and in the same compile at
/// least one source-body environment holds its ordinals in strictly descending
/// order.
///
/// CLAIMED: `D1` discriminates by definition arm at the emission seam itself,
/// so the root keeps descriptor order while the declaration bodies beside it do
/// not.
///
/// **This reads the recorded ORDINAL SEQUENCE, never the predicate.** An
/// earlier cut of this control asserted `source_body_binding_order` directly
/// and was measured GREEN against a build whose classification was correct and
/// whose environments ignored it — a mutation that reddened controls 1 and 2
/// left it passing. A control on the classifier answers whether the classifier
/// agrees with itself; only the sequence answers what the body was handed.
///
/// The second assertion is the discriminating half. "The root is ascending"
/// is trivially green under a build that reverses nothing anywhere — which is
/// precisely the retired behaviour. Requiring both answers OUT OF ONE COMPILE
/// is what makes the negative a decision rather than a constant.
///
/// Red before green: at `21fd46dc` no environment is descending, so the second
/// assertion fails.
///
/// Promise class: **durable invariant** — a relation between two definition
/// arms observed in one compile. No count, ordinal, or unit population is
/// pinned.
#[test]
fn d3_the_process_root_keeps_descriptor_order_while_source_bodies_do_not() {
    let observed = d3_observed_bind_orders();
    assert!(
        !observed.is_empty(),
        "the instrument recorded no environment at all, so every comparison below would pass \
         vacuously"
    );

    let ascending = |ordinals: &[u32]| ordinals.windows(2).all(|pair| pair[0] < pair[1]);
    let descending = |ordinals: &[u32]| ordinals.windows(2).all(|pair| pair[0] > pair[1]);

    let roots = observed
        .iter()
        .filter(|row| matches!(row.definition, AbiUnitDefinition::SchedulingEntry { .. }))
        .collect::<Vec<_>>();
    assert!(
        !roots.is_empty(),
        "this compile built no scheduling-entry environment, so the negative below is vacuous: \
         {observed:#?}"
    );
    for row in &roots {
        assert!(
            ascending(&row.parameter_ordinals),
            "a scheduling entry's ingress roles must reach its body in descriptor order; this \
             root was handed {:?}: {row:#?}",
            row.parameter_ordinals
        );
    }

    let reversed = observed
        .iter()
        .filter(|row| row.parameter_ordinals.len() > 1 && descending(&row.parameter_ordinals))
        .collect::<Vec<_>>();
    assert!(
        !reversed.is_empty(),
        "no environment in this compile is in reversed order, so 'the root is ascending' is \
         green under a build that converts nothing and decides nothing: {observed:#?}"
    );
    for row in &reversed {
        assert!(
            matches!(
                row.definition,
                AbiUnitDefinition::CallableDeclaration { .. }
                    | AbiUnitDefinition::ClosureBody { .. }
            ),
            "only a source body may be handed a reversed parameter run, but this one was: \
             {row:#?}"
        );
    }
}

/// **`D3` control 4 — the generated-context seat obeys the SAME binding law as
/// the unit seat.**
///
/// `D2`'s claim is an equivalence between two hosts for one body. It cannot
/// be measured as a join across the two hosts, and the reason is structural: a
/// raw worker every selecting specialization has retargeted is
/// **template-only** and is absent from the emitted-`Function` population, so
/// the body that reaches a generated context has no ordinary unit to compare
/// against — in this witness, in either setting of the deforested answer route.
/// A control that joined on body origin would find nothing and say so, which is
/// how this one was first written and why it is not written that way now.
///
/// What is comparable is the LAW. Both seats build a semantic environment from
/// the same descriptor run under the same conversion, so at either seat the
/// recorded parameter ordinals must be the descriptor's own ascending run,
/// reversed exactly when that seat converted. Checking one law at both hosts
/// asks whether the generated context follows the unit seat's rule, without
/// needing the two to meet on one body.
///
/// MEASURED, on the `D5a` generated-context witness: every recorded
/// environment — at both hosts — holds `0..n` ascending when it did not convert
/// and descending when it did; at least one environment was built at a
/// generated context; and at least one environment of length two or more is
/// recorded reversed.
///
/// **THE GAP, and it is the part of `D3` this node did not deliver.** Every
/// generated-context environment in this crate's only such populations has
/// exactly ONE parameter, and a one-element sequence satisfies the law under
/// either conversion. So over the generated-context rows this control has **no
/// discriminating power today**: it would stay green against a `D2` seat that
/// converted when it should not, or not when it should. Its power over those
/// rows is latent and arrives with the first multi-parameter worker. The
/// two-or-more reversed environment it also requires belongs to a
/// `CallableDeclaration` at the UNIT seat, so that clause discriminates `D1`,
/// not `D2`. The frame asked for a distinguishable two-parameter body carried
/// through a generated context; the populations that build one (`px8tr`,
/// `governed_nested_resource_bracket`) declare single-parameter workers, and
/// giving one a second parameter also requires changing the arity its checked
/// IH call site passes. That is a fixture change beyond this node — left
/// undone and reported, not silently narrowed.
///
/// Promise class: **durable invariant** — a law relating a recorded
/// environment to the descriptor run it was built from. No count, ordinal, or
/// population is pinned.
#[test]
fn d3_both_binding_seats_obey_one_conversion_law() {
    let _ = srcbody_bind_order_take();
    crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
        "d3_bind_order_law",
        false,
    )
    .expect("the D5a generated-context witness compiles");
    let observed = srcbody_bind_order_take();
    assert!(
        !observed.is_empty(),
        "the instrument recorded no environment, so every check below is vacuous"
    );

    for row in &observed {
        let ascending = (0..row.parameter_ordinals.len() as u32).collect::<Vec<_>>();
        let expected = if row.converted {
            ascending.iter().rev().copied().collect::<Vec<_>>()
        } else {
            ascending
        };
        assert_eq!(
            row.parameter_ordinals, expected,
            "this seat recorded converted = {} but handed its body {:?}, which is neither the \
             descriptor's ascending run nor its reverse: {row:#?}",
            row.converted, row.parameter_ordinals
        );
    }

    assert!(
        observed
            .iter()
            .any(|row| row.host == SrcbodyBindHost::GeneratedContext),
        "this witness built no generated-context environment, so the law above was checked at \
         one seat only and says nothing about D2: {observed:#?}"
    );
    assert!(
        observed
            .iter()
            .any(|row| row.parameter_ordinals.len() > 1 && row.converted),
        "no environment of length two or more was converted, so the law above is satisfied by \
         every row trivially and discriminates nothing: {observed:#?}"
    );
}

/// **`D3` control 4, amended — the REACHING POSITIVE CONTROL for the
/// producer-wide arity sentinel.**
///
/// **The sentinel itself is not in this file.** It is a `cfg(test)` gate at the
/// generated-context construction edge in `define_continuation_context_bodies`,
/// immediately before the `SrcbodyBindHost::GeneratedContext` observation is
/// recorded. Every generated context this crate's lib-test population builds
/// passes through that edge, so the bound is closed over the producer. What it
/// watches, what activates it, what retires it, and its `cfg(test)` residual
/// are all stated there, once, beside the code that enforces it.
///
/// **Why this test still exists, and it is the half a producer-side gate cannot
/// do for itself.** An assertion sited inside production code is vacuous if
/// nothing reaches it, and that vacuity is SILENT: a gate that never runs is
/// indistinguishable from a gate that always passes. This control compiles a
/// real witness and proves the edge is **reached** — that a generated context
/// is genuinely constructed and recorded — so the gate is known to be armed
/// against a live population rather than watching an empty set.
///
/// **The earlier cut, and why it was insufficient.** This test previously
/// carried the bound itself, over the observations of this one compile. That
/// made it a witness-local sentinel wearing a population-wide name: a
/// multi-parameter worker introduced by any other program would never have
/// entered its observation vector, so it would have stayed green at exactly the
/// moment the obligation activated. Moving the bound to the producer is the
/// correction; renaming this test would not have been one.
///
/// **What `D2` claims and why it is inert today.** `D2` says a generated
/// context binds its raw owner's parameter run in the same order that owner's
/// own unit would. `D1`'s conversion reverses that run, and reversal is the
/// identity on a run of length one. Every worker reaching a generated context
/// in this crate declares exactly ONE parameter, so `D2` cannot be observed
/// doing anything at all — the conversion law checked at both seats by
/// [`d3_both_binding_seats_obey_one_conversion_law`] is satisfied by those rows
/// trivially, under either decision.
///
/// MEASURED: compiling the `D5a` generated-context witness records at least one
/// `GeneratedContext` environment.
///
/// **This test deliberately does NOT re-assert the arity bound**, and the
/// reason is worth stating so nobody adds it back as belt-and-braces. The
/// producer gate fires *during* the `emit_...` call below, so a violation this
/// witness reaches panics inside the compile and the `expect` here never
/// returns. A local bound after it could not fail for the intended reason under
/// any input — it would be an assertion that reads as coverage and can never
/// run. The bound is enforced in exactly one place; this test's only claim is
/// that the place is reachable.
///
/// Red before green: against a temporary hand-added two-parameter
/// generated-context worker, the producer gate reds tests that were **not**
/// modified — which is the evidence that its reach is not witness-local. The
/// witness is not committed; see the node's handoff for the observed rows.
/// Nothing in the checked IH call-site arity, the fixture population, or the
/// worker declarations is persistently changed.
///
/// Promise class: the **transition sentinel** is the producer gate; this test
/// is its non-vacuity control and is a **durable invariant** — it asserts that
/// the edge is reachable at all, which every intended extension preserves.
#[test]
fn d3_generated_context_arity_sentinel_edge_is_reached() {
    let _ = srcbody_bind_order_take();
    crate::cranelift_backend::test_objects::emit_px8tr_nested_post_effect_object(
        "d3_bind_order_arity_sentinel",
        false,
    )
    .expect("the D5a generated-context witness compiles");
    let observed = srcbody_bind_order_take();

    let contexts = observed
        .iter()
        .filter(|row| row.host == SrcbodyBindHost::GeneratedContext)
        .collect::<Vec<_>>();
    // The producer gate is an assertion inside production code, so its failure
    // mode is not "it says the wrong thing" but "nothing ever reaches it" --
    // and a gate that never runs is indistinguishable from one that always
    // passes. This is the row that tells the two apart.
    assert!(
        !contexts.is_empty(),
        "this witness built no generated-context environment, so the producer-edge arity \
         sentinel in define_continuation_context_bodies was never reached by this compile and \
         its passing says nothing: {observed:#?}"
    );
}

// ─── RT-PRODUCER-MATCH-PORT D2 — the producer-call scrutinee unit ───────────
//
// COVERAGE OF THE THREE PRE-DELEGATION REFUSALS, STATED BECAUSE IT IS PARTIAL.
//
// The ported arm refuses three frame states before it delegates: a retained
// scrutinee index, a deferred constructor case, and any trailing composed
// eliminator. NONE of the three has a shape-reaching control in `D2`. What was
// established is weaker and is stated as what it is:
//
//   ESTABLISHED, by structural inspection of the source only -- each guard is
//   written above the delegation in the same block, so no frame carrying one of
//   these states can reach `lower_carried_match` or the port counter.
//
//   NOT ESTABLISHED -- no test drives a frame carrying any of the three. Nothing
//   below executes these paths, and no claim here rests on their having run.
//
// THE ACTUAL REASON THE D2 FIXTURE CANNOT REACH THEM, which is a property of
// where its frame is built rather than of the fixture's syntax.
//
// `OrdinaryEliminatorFrame` has five production construction sites. The `D2`
// fixture enters through exactly one of them: the direct `RuntimeExpr::Match`
// arm's deforestation branch, which hard-codes BOTH `retained_scrutinee_index`
// and `deferred_constructor_case` to `None` and passes a ONE-element eliminator
// slice. All three guard conditions are therefore constant-false on that path --
// not merely absent from this fixture's shape.
//
// The states exist elsewhere and are reached by other entry paths this fixture
// does not take: `retained_scrutinee_index: Some(0)` and
// `deferred_constructor_case: Some(&deferred)` are each set at their own sites,
// and a multi-element eliminator slice is built by the composition sites that
// push onto `composed`. Measured by enumerating every `OrdinaryEliminatorFrame {`
// construction in `crates/` and reading the two fields at each -- so this is a
// claim about the construction sites, not about the ones a grep happened to hit.
//
// The three refusals are conservative, so the failure direction is over-strict
// rather than unsound. That bounds the risk; it does not discharge the coverage,
// and it is not offered as if it did.

/// **`D2`'s port, now reached the way PRODUCTION reaches it.**
///
/// `match ((\x . Some x) 4) with Some y => y`. The scrutinee `Call` is lowered
/// as a separately owned callable unit, and this node's delegation eliminates
/// the resulting carried word with `lower_carried_match` -- the same function
/// the direct `RuntimeExpr::Match` route already used for a carried scrutinee.
/// No second transport was built, and that is the deliverable.
///
/// **`D3` deleted the selector witness along with the variant**, so this row no
/// longer arms anything: the program selects the functionized lane because the
/// classification is gone, not because a test-only mask suppressed it. That is a
/// strictly stronger route than the one `D2` was accepted on.
///
/// **What the two assertions prove TOGETHER, and neither alone.** The count is
/// taken before `lower_carried_match`, which can still refuse, so a count of 1
/// alone would not establish that an elimination was emitted. Paired with a run
/// that returns the program's declared observation, it does.
///
/// **Promise class: durable invariant.** The ported shape must produce its
/// declared observation.
#[test]
fn d3_the_ported_producer_call_scrutinee_runs_unhooked_on_the_functionized_lane() {
    let example = seed_call_port_producer_match_example();

    reset_producer_match_unit_ports();
    let report = run_example_with_seed_observation(&example, &NativeSeedEnvironment::empty())
        .expect("AC-1b: the D1 firing population must still build and run");
    assert_eq!(
        report.observation, example.observation,
        "AC-1b: the ported elimination must produce the program's declared observation"
    );
    assert!(report.verifier_passed);
    assert_eq!(
        producer_match_unit_ports(),
        1,
        "D3 is an ACTIVATION: the program must reach the port's handoff with no selector witness \
         in the tree. A zero here means the variant was removed while the port stayed dead"
    );
}

/// **`RT-CARRIED-CONTINUATION-RESUME` `D3` — the `Active` carried route is taken,
/// proven by the DISAPPEARANCE of the continuation-frame refusal plus a reached
/// detector, not by the row compiling.**
///
/// The two population members do **not** compile end to end at this base and
/// will not until the `Carried` x `Ordinary` suffix successor lands. So this
/// control cannot assert success, and it deliberately does not.
///
/// It also deliberately does **not** key on the refusal the rows now hit. That
/// message belongs to `RT-PRODUCER-MATCH-PORT`'s trailing-composed-eliminator
/// guard, and the successor node is going to change exactly that guard -- a
/// control keyed on it would either break for an unrelated reason or, worse,
/// **pass vacuously** once the string it matches no longer exists.
///
/// What it keys on instead survives the successor:
///
/// 1. the continuation-frame refusal is **absent**, whatever the row does next;
/// 2. the route was **measurably taken** -- arrivals and routes are both
///    non-zero, so the absence is not the seat having been skipped;
/// 3. under the mutation the refusal comes **back verbatim** while arrivals stay
///    non-zero, so the detector is proven reached rather than bypassed.
///
/// Clause 3 is what makes clause 1 mean anything: a negative check passes for
/// any reason, and this is its positive control.
///
/// **Promise class: durable invariant.** Every assertion holds whether the row
/// later compiles or stops at a further wall, because none of them asserts the
/// row's overall outcome. The arrivals counter is read **before** the
/// suppression branch in production, so the mutated side cannot manufacture its
/// own zero.
#[test]
fn ccr_d3_the_active_carried_route_is_taken_and_the_continuation_refusal_is_gone() {
    use crate::cranelift_backend::lowering::core::{
        ccr_d2_active_arrivals, ccr_d2_active_routes, reset_ccr_d2_counts,
        set_ccr_d2_suppress_active_route,
    };
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            set_ccr_d2_suppress_active_route(false);
        }
    }

    /// The refusal this node retired for `Active`. Held once, so the two sides
    /// of the A/B cannot drift apart.
    const CONTINUATION_REFUSAL: &str = "a carried scrutinee reached a continuation frame that \
                                        resumes a compile-time value rather than eliminating one";

    // The same shape both measured members reach the arm through: an ordinary
    // `Match` over the deferred-constructor recursive fixture.
    let witness = RuntimeExpr::Match {
        scrutinee: Box::new(px8j_deferred_recursive_field_fixture()),
        cases: ["ctor:prelude::Result::Err", "ctor:prelude::Result::Ok"]
            .into_iter()
            .map(|constructor| RuntimeMatchCase {
                constructor: constructor.to_string(),
                binders: 1,
                body: RuntimeExpr::Construct {
                    constructor: crate::EXIT_SUCCESS_CONSTRUCTOR.to_string(),
                    args: Vec::new(),
                },
            })
            .collect(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "CCR D3 witness".to_string(),
        },
    };

    let run = |label: &'static str| -> String {
        let _restore = Restore;
        reset_ccr_d2_counts();
        let (result, _trace) = px8j_capture_source_trace(&witness, false, label);
        match result.map(|_| ()) {
            Ok(()) => "Ok".to_string(),
            Err(error) => format!("{error:?}"),
        }
    };

    // ── A: the route as landed ──────────────────────────────────────────────
    let routed = run("ken_ccr_d3_routed");
    let (routed_arrivals, routed_routes) = (ccr_d2_active_arrivals(), ccr_d2_active_routes());
    assert!(
        routed_arrivals > 0,
        "the carried Active arm must be REACHED, or every claim below is about a seat this \
         program never visited: arrivals={routed_arrivals}"
    );
    assert!(
        routed_routes > 0,
        "the arm must have ROUTED to resume_active_continuation, not merely been entered: \
         arrivals={routed_arrivals} routes={routed_routes}"
    );
    assert!(
        !routed.contains(CONTINUATION_REFUSAL),
        "the continuation-frame refusal must be GONE for an Active frame. This asserts the \
         ADVANCE, not success -- the row still stops at the Carried x Ordinary suffix guard, \
         which is a different authority and deliberately not named here: {routed}"
    );

    // ── B: the mutation at this exact root ──────────────────────────────────
    set_ccr_d2_suppress_active_route(true);
    let suppressed = run("ken_ccr_d3_suppressed");
    let (suppressed_arrivals, suppressed_routes) =
        (ccr_d2_active_arrivals(), ccr_d2_active_routes());
    set_ccr_d2_suppress_active_route(false);
    assert!(
        suppressed_arrivals > 0,
        "the detector must still be REACHED under the mutation. A zero here would mean the \
         mutated side proves nothing, because the refusal could be explained by the arm never \
         being entered: arrivals={suppressed_arrivals}"
    );
    assert_eq!(
        suppressed_routes, 0,
        "the mutation must suppress the ROUTE specifically, or it is mutating something else"
    );
    assert!(
        suppressed.contains(CONTINUATION_REFUSAL),
        "the mutation must recreate the EXACT attributed refusal, verbatim. A different refusal \
         means this control is anchored to the wrong root: {suppressed}"
    );
}

/// **`RT-CARRIED-ORDINARY-COMPOSITION` `D3` — the trailing-suffix continuation
/// is taken, proven by counters and a mutation rather than by any refusal
/// string.**
///
/// ## Why this control asserts neither of the two obvious things
///
/// **It cannot assert compile success.** Both population members still stop one
/// authority further out, at the successor-owned `ComputationalMatch` refusal.
///
/// **It cannot assert the absence of the `D2` refusal either**, which is the
/// less obvious half. `D2` **deleted** that refusal from production — the only
/// surviving occurrence is a comment — so `!contains(...)` would be true for
/// free, forever, and would keep passing if the repair were ripped out and
/// replaced by anything that did not spell that exact sentence. **A string no
/// code can produce is absent for free.**
///
/// **And it must not key on the fifth wall's message**, which is independently
/// pinned in full equality and which the successor may legitimately change.
///
/// ## What it keys on instead
///
/// 1. the arm is **reached** with a nonempty suffix — a non-zero pre-guard
///    denominator;
/// 2. **every** such arrival is continued rather than refused, asserted as
///    `continuations == arrivals` rather than as `> 0`, so one continued arrival
///    cannot mask a second refused one;
/// 3. under the mutation the continuation stops, the arrival count is
///    **unchanged**, and the pre-`D2` refusal becomes producible again — which
///    is what makes clause 2 falsifiable.
///
/// ## What this control is NOT evidence for
///
/// **Its own arrival is not evidence that the population exists.** A control that
/// is a member of the population it observes proves the hook is reachable, not
/// that any program has the shape. The population evidence is `D0`'s census, and
/// the independent population there is **two** — this control is the third
/// counted member and must be excluded from any denominator quoted as
/// population.
///
/// **Promise class: durable invariant.** No assertion depends on the row's
/// overall outcome, so all of them survive the successor closing the fifth wall.
#[test]
fn coc_d3_the_trailing_suffix_is_continued_and_the_mutation_restores_the_refusal() {
    use crate::cranelift_backend::lowering::core::{
        coc_d2_suffix_arrivals, coc_d2_suffix_continuations, reset_coc_d2_counts,
        set_coc_d2_suppress_continuation,
    };
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            set_coc_d2_suppress_continuation(false);
        }
    }
    const PRE_D2_REFUSAL: &str = "the carried elimination consumes exactly one frame";

    let witness = RuntimeExpr::Match {
        scrutinee: Box::new(px8j_deferred_recursive_field_fixture()),
        cases: ["ctor:prelude::Result::Err", "ctor:prelude::Result::Ok"]
            .into_iter()
            .map(|constructor| RuntimeMatchCase {
                constructor: constructor.to_string(),
                binders: 1,
                body: RuntimeExpr::Construct {
                    constructor: crate::EXIT_SUCCESS_CONSTRUCTOR.to_string(),
                    args: Vec::new(),
                },
            })
            .collect(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "COC D3 witness".to_string(),
        },
    };

    let run = |label: &'static str| -> String {
        let _restore = Restore;
        reset_coc_d2_counts();
        let (result, _trace) = px8j_capture_source_trace(&witness, false, label);
        match result.map(|_| ()) {
            Ok(()) => "Ok".to_string(),
            Err(error) => format!("{error:?}"),
        }
    };

    // ── A: the continuation as landed ───────────────────────────────────────
    let continued = run("ken_coc_d3_continued");
    let (arrivals, continuations) = (coc_d2_suffix_arrivals(), coc_d2_suffix_continuations());
    assert!(
        arrivals > 0,
        "the arm must be REACHED with a nonempty suffix, or every claim below is about a path \
         this program never took: arrivals={arrivals}"
    );
    assert_eq!(
        continuations, arrivals,
        "EVERY suffix arrival must be continued, not merely one of them. Asserted as equality \
         rather than `> 0` so a single continued arrival cannot mask a second that refused: \
         arrivals={arrivals} continuations={continuations}"
    );
    assert!(
        !continued.contains(PRE_D2_REFUSAL),
        "the pre-D2 refusal must not be what stops this row. This is corroboration only -- the \
         load-bearing assertions are the counters, because D2 deleted this sentence from \
         production and an absent-for-free string proves nothing on its own: {continued}"
    );

    // ── B: the mutation at this exact root ──────────────────────────────────
    set_coc_d2_suppress_continuation(true);
    let suppressed = run("ken_coc_d3_suppressed");
    let (mutated_arrivals, mutated_continuations) =
        (coc_d2_suffix_arrivals(), coc_d2_suffix_continuations());
    set_coc_d2_suppress_continuation(false);
    // **AMENDED BY `RT-SPECIALIZED-ACTIVE-RESUME` `D2`, and the reason is the
    // point.** This was `assert_eq!(mutated_arrivals, arrivals)`, and that
    // equality held only because BOTH runs aborted at the same downstream wall
    // -- the constructor-shape refusal -- so both traversed the same amount of
    // program. `D2` routes the ordinary-live cell past that wall, so the
    // unmutated run now continues and reaches this arm again (measured: 2
    // against the suppressed run's 1). The equality was therefore contingent on
    // a downstream refusal rather than on anything this control owns.
    //
    // Nothing discriminating is lost. The denominator's only job is to rule out
    // "the arm was never entered" as an explanation for zero continuations, and
    // `> 0` discharges exactly that. The load-bearing clauses below --
    // `mutated_continuations == 0` and the refusal becoming producible again --
    // are untouched.
    assert!(
        mutated_arrivals > 0,
        "the mutation must still REACH the arm, or the missing continuation below could be \
         explained by the arm never being entered rather than by the mutation: \
         mutated_arrivals={mutated_arrivals}"
    );
    assert_eq!(
        mutated_continuations, 0,
        "the mutation must suppress the CONTINUATION specifically, or it is mutating something \
         else"
    );
    assert!(
        suppressed.contains(PRE_D2_REFUSAL),
        "and it must make the pre-D2 refusal producible again. This is the clause that stops the \
         A-side absence check being vacuous: {suppressed}"
    );
}

/// **`RT-CONTINUATION-EDGE-DISPOSITION` `D2` / `AC-7` — the real witness, and
/// it COMPILES.**
///
/// This is the member `D0` could not find: a selected `FunctionizedUnits`
/// artifact in which a binding is installed, a candidate settles
/// `InlineNoCall` after the deferred bridge completes, and the compile reaches
/// the existing closeout.
///
/// **The sentinel has fired, and this heading is the record of it.** Through
/// `D1` the outcome was a refusal and that was the deliverable: the candidate
/// sat in a `planned` set seeded from the full `continuation_calls()`
/// population and was neither emitted nor composed, so any closeout that
/// checked it had to refuse. `D2`'s ordered closeout -- candidate totality
/// first, then the derived `DirectCall ∪ ComposedCall` subset, then the
/// unchanged exact equality and claim equality over it -- removed the
/// obligation without adding an arm to the partition, and the same program now
/// compiles.
///
/// **The discriminator is the `Ok`, together with the binding and the
/// `InlineNoCall` settlement above it.** `D2` also corrected the production
/// diagnostic to name the call-obligation population, so the `D1`-era sentence
/// no longer exists in production and asserting its absence is free — it is
/// retained as historical evidence of the crossing, not as the oracle. The
/// `Ok` clause is what catches a derivation that MOVED the failure instead of
/// removing it, and it is how the second `close` clause keyed on `planned` was
/// found.
///
/// **Promise class: spent transition sentinel, retained as an invariant.** It
/// was named for the boundary rather than the outcome, so the crossing is
/// recorded here in operative text instead of leaving a heading that says the
/// opposite of what the assertions check.
#[test]
fn ced_d2_the_inline_candidate_settles_after_the_bridge_and_is_not_a_call_obligation() {
    use crate::cranelift_backend::lowering::{d8d_bindings, reset_d8d_bindings};
    use crate::cranelift_backend::lowering::units::{
        d1_last_dispositions, reset_d1_dispositions, CandidateDisposition,
    };

    // The refusal `D1` pinned, kept ONLY as historical evidence of the
    // crossing. `D2` corrected the production diagnostic to name the
    // call-obligation population, so this exact sentence no longer exists in
    // production and asserting its absence is now free.
    //
    // ⇒ That is why the durable discriminator below is `Ok`, not this absence.
    // Retaining a knowingly false "planned" diagnostic so an absence assertion
    // would stay non-free was the wrong trade, and it is not made here.
    const HISTORICAL_D1_REFUSAL: &str =
        "the discharged continuation call population is not the planned one";

    let witness = RuntimeExpr::Match {
        scrutinee: Box::new(px8j_deferred_recursive_field_fixture()),
        cases: ["ctor:prelude::Result::Err", "ctor:prelude::Result::Ok"]
            .into_iter()
            .map(|constructor| RuntimeMatchCase {
                constructor: constructor.to_string(),
                binders: 1,
                body: RuntimeExpr::Construct {
                    constructor: crate::EXIT_SUCCESS_CONSTRUCTOR.to_string(),
                    args: Vec::new(),
                },
            })
            .collect(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "CED D1 AC-7 witness".to_string(),
        },
    };

    reset_d1_dispositions();
    reset_d8d_bindings();
    let (result, _trace) = px8j_capture_source_trace(&witness, false, "ken_ced_d1_ac7_witness");
    let outcome = match result.map(|_| ()) {
        Ok(()) => "Ok".to_string(),
        Err(error) => format!("{error:?}"),
    };
    let dispositions = d1_last_dispositions();
    let bindings = d8d_bindings();

    // Clause 1 -- a binding IS installed. Without this the rest is about a
    // candidate that never authorized anything, which is the role this node
    // exists to separate from the call obligation.
    assert!(
        bindings > 0,
        "the witness must install a static-worker binding, or it is not exercising the binding \
         projection role at all: bindings={bindings}"
    );

    // Clause 2 -- a candidate settled InlineNoCall. This can only happen after
    // the deferred bridge returned Ok with the candidate unconsumed; the seat
    // has no other caller.
    assert!(
        dispositions.get(&CandidateDisposition::InlineNoCall).copied().unwrap_or(0) > 0,
        "a candidate must settle InlineNoCall after the bridge completes. Zero here means either \
         the bridge did not complete -- in which case this is a BRIDGE_INCOMPLETE program and not \
         a witness -- or the candidate was consumed and settled some other way: {dispositions:?}"
    );

    // Clause 3 -- it reaches the existing closeout and is NOT refused by it.
    //
    // **INVERTED BY `D2`, deliberately and under review.** This assertion read
    // `outcome.contains(...)` for the whole of `D1`, and the refusal was the
    // deliverable: the candidate was in the equality by construction and
    // nothing could take it out. `D2`'s ordered closeout derives the
    // call-obligation subset from `DirectCall ∪ ComposedCall`, `InlineNoCall`
    // is not in it, and the same witness now compiles.
    //
    // The sentinel is spent. **The absence check below is FREE**, because `D2`
    // also corrected the production diagnostic to name the call-obligation
    // population, so the `D1`-era sentence no longer exists anywhere in
    // production. It is retained as historical evidence of the crossing, not
    // as an oracle, and the discriminator is the exact `Ok` together with the
    // binding and `InlineNoCall` settlement asserted above.
    assert!(
        !outcome.contains(HISTORICAL_D1_REFUSAL),
        "the D1-era refusal must not reappear. This clause is free now that the production \
         diagnostic has been corrected, and it is kept as historical evidence rather than as \
         the discriminator: {outcome}"
    );
    assert_eq!(
        outcome, "Ok",
        "and it must compile, not merely stop failing for this one reason. A different refusal \
         here would mean the subset derivation moved the failure rather than removing it: \
         {outcome}"
    );
}

/// **`RT-CONTINUATION-EDGE-DISPOSITION` `D1` — `d8e`'s discriminator, asserted
/// by this node rather than inherited from a neighbour's green status.**
///
/// `AC-5` says `d8e` keeps **one** binding and still **refuses in value
/// position**. Both halves are load-bearing and they fail in opposite
/// directions: losing the binding is how the withdrawn planner-side exclusion
/// broke this program -- it compiled `Ok` in a shifted environment -- while
/// losing the refusal alone would mean the fail-closed guard had been
/// weakened.
///
/// **Relying on `d8e` merely staying green does not discharge this.** A
/// neighbouring row is green for its own reasons, and if `D1` ever moved this
/// behaviour the failure would land in a file this node does not own, attributed
/// to whoever touched it next. So the pair is asserted here, on this node's
/// terms.
///
/// **Promise class: durable invariant.** A relation between the installed
/// binding count and the refusal a value-position read must still produce, on
/// one program, in both bridge arms.
#[test]
fn ced_d1_d8e_keeps_its_one_binding_and_still_refuses_in_value_position() {
    const VALUE_POSITION_REFUSAL: &str = "a static worker binding has no value representation";

    // The binding half holds on BOTH arms, and it is the half the withdrawn
    // planner-side exclusion broke: it removed the installed binding and let
    // the program compile in a shifted environment.
    for (arm, computational_bridge) in [("ordinary", false), ("computational", true)] {
        let (_error, (sites, bindings, consumptions), _markers) =
            d8e_witness_compile("ced_d1_d8e", 2, computational_bridge);
        assert!(
            sites > 0,
            "the {arm} arm must still reach the composed site, or its clause is vacuous"
        );
        assert_eq!(
            (bindings, consumptions),
            (1, 0),
            "the {arm} arm must keep exactly ONE installed binding and consume nothing. A zero \
             binding count is the signature of the withdrawn planner-side exclusion"
        );
    }

    // The value-position refusal is the ORDINARY arm's alone, and saying so is
    // the point rather than a convenience. Off the source-machine path the case
    // bodies are lowered by `lower_expr`, so the callee reaches the binding in
    // value position and `D8d`'s fail-closed guard fires. The computational arm
    // lowers them through the source machine and stops earlier, for a different
    // reason -- asserting this string of both arms reds on that difference and
    // would be measuring the wrong property, not a stricter one.
    let (error, _counts, _markers) = d8e_witness_compile("ced_d1_d8e_value", 2, false);
    let reason = format!("{:?}", error.expect("the value-position read must still refuse"));
    assert!(
        reason.contains(VALUE_POSITION_REFUSAL),
        "the ordinary arm's refusal must still be D8d's own fail-closed value-position guard, \
         not an incidental downstream failure: {reason}"
    );
}

/// **`RT-CONTINUATION-EDGE-DISPOSITION` `D2` — `D5a`'s original
/// single-authority promise, isolated so it survives the composite close.**
///
/// **Why this row has to exist separately.** `D5a`'s whole-wrapper early-close
/// mutation used to move exactly one authority: the claim ledger's window. Once
/// the wrapper acquired the sibling candidate ledger, that mutation began
/// moving **both** closeouts, and candidate totality became the first refusal
/// it reaches. Re-pointing that row to the totality string — which `D2` did,
/// correctly — would have left `D5a`'s actual promise untested while its name
/// and prose still claimed it.
///
/// So the promise is re-asserted here, at the level where it is still a
/// single-authority statement: **a planned continuation call token that no unit
/// claimed is missing from the exact claim/discharge equality**, observed on a
/// ledger whose call-obligation domain is **supplied complete** rather than
/// derived from a candidate population.
///
/// **The isolation is the point and it is what keeps this test-only.** Nothing
/// here bypasses candidate totality in production, and no production special
/// case is added: the domain is handed in directly, exactly as
/// `ContinuationClaimLedger` receives it after `D2`'s derivation, so the
/// equality is exercised on its own without a second authority in front of it.
///
/// **Promise class: durable invariant.** A set relation over a population the
/// fixture does not fix — the identities come from the plan's own pairing, and
/// `ContinuationCallIdentity` has no constructor outside planning, so this row
/// cannot fabricate its population even by accident.
#[test]
fn ced_d2_an_unclaimed_planned_token_is_missing_from_the_exact_equality_in_isolation() {
    use crate::cranelift_backend::lowering::units::{declare_unit_bundle, ContinuationClaimLedger};

    let entry = d8j_root_witness_entry();
    let plan = plan_static_transition_graph_with_symbols(
        &entry,
        &BTreeMap::new(),
        &crate::NativeProcessSymbols::legacy_prelude(),
        AbiRootIngress::Value,
        true,
    )
    .expect("the witness plans");
    let identities = plan
        .composed_call_targets()
        .expect("targets")
        .iter()
        .map(|target| target.call_identity().clone())
        .collect::<Vec<_>>();
    assert!(
        !identities.is_empty(),
        "the witness must plan at least one causal identity, or this row is vacuous"
    );
    // The call-obligation domain, SUPPLIED complete. This is the isolation:
    // after `D2` the claim ledger receives this set from the candidate
    // ledger's derivation, and handing it in directly exercises the equality
    // without candidate totality standing in front of it.
    let call_obligations = identities
        .iter()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();

    let mut module = new_object_module("ced-d2-isolated-equality").expect("module");
    let bundle = declare_unit_bundle(&mut module, &plan).expect("the bundle declares");
    let mut ledger = ContinuationClaimLedger::open(&plan, &bundle).expect("the ledger opens");
    ledger
        .record_declared(identities.iter().cloned())
        .expect("declaration covers the planned set");

    // Nothing is claimed and nothing is discharged, which is exactly the state
    // the early window left behind: the specialization-owned token is planned
    // and cannot yet have been answered.
    let refusal = format!(
        "{:?}",
        ledger
            .close(&call_obligations)
            .expect_err("an unclaimed, undischarged call obligation must refuse")
    );
    // Keyed on the DISCHARGE equality's own sentence. The first draft of this
    // row reused `D5a`'s old keys -- "does not equal the planned one" plus
    // "absent" -- which belong to the declared/resolved clause, not this one.
    // It failed loudly rather than passing on a neighbouring refusal, which is
    // the behaviour a substring oracle has to have.
    assert!(
        refusal.contains("the discharged continuation call population is not the call-obligation one")
            && refusal
                .contains("neither directly emitted nor compositionally consumed"),
        "the refusal must come from the EQUALITY finding the obligation undischarged, which is \
         the single authority D5a promised and the composite-close row can no longer observe. A \
         candidate-totality message here would mean this row had drifted back onto the other \
         authority; a declared/resolved message would mean it is measuring an earlier clause: \
         {refusal}"
    );
}

// `RT-CONTINUATION-EDGE-DISPOSITION` `D3` / `AC-6` — the five mutation rows.
//
// **Five mutation-SPECIFIC CAUSAL proofs, not five distinct terminal strings**
// (Architect ruling, held checkpoint `c17b9939`). Rows 2 and 3 deliberately
// share a terminal refusal: they break the same invariant at two different
// causal points, and production says the same true thing about both. The
// ruling is that the shared string may CORROBORATE each row and may be
// NEITHER row's sole oracle — so each row pins its own ordered causal chain,
// keyed by the live `ContinuationCallIdentity`, and the seat is what tells
// them apart.
//
// **No production diagnostic was changed to manufacture different strings.**
// That was available and it is the wrong trade: it would buy a cheap
// discriminator by making production say something less true.
//
// **Every row discharges the same five clauses**, and each is asserted rather
// than argued:
//
//   1. the unmutated, production-shaped witness SUCCEEDS under merged `D2`;
//   2. the same derived identity reaches the mutation's exact seat in BOTH arms;
//   3. exactly one variant is armed;
//   4. a mutation-specific causal observation proves what moved; and
//   5. the armed run reaches its expected refusal — except row 1, where the
//      ruling asks instead for a changed pinned downstream structural oracle,
//      and this control shows why that is the harder bar rather than the softer
//      one.
//
// **The witnesses are SELECTED from the live population, not authored.** The
// Steward authorized authoring; a census over the lib suite's 425 successful
// artifact closes found 73 carrying a `DirectCall`, so the authorization is
// not exercised. `D0`'s "a `D3` control needs a witness authored for it" was
// true of the `InlineNoCall` class and does not generalize to these five.

/// Runs `body` with exactly one `D3` mutation armed, restoring `None`
/// afterwards **even if `body` panics**, so one failing row cannot leak its
/// mutation into every later test on the thread.
fn with_d3_mutation<T>(
    mutation: crate::cranelift_backend::lowering::units::D3Mutation,
    body: impl FnOnce() -> T,
) -> T {
    use crate::cranelift_backend::lowering::units::{set_d3_mutation, D3Mutation};
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            set_d3_mutation(D3Mutation::None);
        }
    }
    set_d3_mutation(mutation);
    let _restore = Restore;
    body()
}

/// One arm of a `D3` row: what the compile did, what the ledger recorded, and
/// the ordered causal trace that says WHERE.
struct D3Arm {
    outcome: String,
    dispositions: std::collections::BTreeMap<
        crate::cranelift_backend::lowering::units::CandidateDisposition,
        usize,
    >,
    trace: Vec<crate::cranelift_backend::lowering::units::D3Event>,
    /// The candidate population **the live plan projected**, captured at
    /// `ContinuationCandidateLedger::open` — typed identities, not a rendering.
    plan_candidates: Vec<ContinuationCallIdentity>,
    bindings: usize,
    consumptions: usize,
}

impl D3Arm {
    /// The seats at which a settlement was ATTEMPTED, in order.
    fn settle_seats(&self) -> Vec<crate::cranelift_backend::lowering::units::D3Seat> {
        self.trace.iter().filter_map(|e| e.settle_seat()).collect()
    }

    /// **THE candidate, selected from the live plan by type.**
    ///
    /// ⇒ Not derived from the trace, and not a rendering. Both witnesses are
    /// chosen for carrying exactly one candidate, so "the" is a claim this
    /// asserts rather than an assumption: a witness that grew a second
    /// candidate would fail here instead of silently making every clause below
    /// ambiguous about which edge it is talking about.
    ///
    /// Selecting from the plan rather than from the trace is what keeps the
    /// evidence non-circular — the trace is the thing under test.
    fn the_candidate(&self) -> ContinuationCallIdentity {
        assert_eq!(
            self.plan_candidates.len(),
            1,
            "this witness must project exactly ONE binding candidate, or no clause below can say \
             which edge it is about: {:?}",
            self.plan_candidates
        );
        self.plan_candidates[0].clone()
    }

    /// Every observation about **exactly** this identity, in order.
    fn events_for<'a>(
        &'a self,
        identity: &'a ContinuationCallIdentity,
    ) -> impl Iterator<Item = (usize, &'a crate::cranelift_backend::lowering::units::D3Event)>
    {
        self.trace
            .iter()
            .enumerate()
            .filter(move |(_, e)| e.identity() == identity)
    }

    /// The ordered positions at which **this exact identity** was settled with
    /// **this exact `(disposition, seat)`**.
    ///
    /// ⇒ The triple is the unit of evidence. A position keyed on the seat
    /// alone would be satisfied by a settlement of a different edge, or of the
    /// same edge to a different disposition, and both are distinct defects.
    fn settlements_of(
        &self,
        identity: &ContinuationCallIdentity,
        disposition: crate::cranelift_backend::lowering::units::CandidateDisposition,
        seat: crate::cranelift_backend::lowering::units::D3Seat,
    ) -> Vec<usize> {
        use crate::cranelift_backend::lowering::units::D3Event;
        self.events_for(identity)
            .filter(|(_, e)| {
                matches!(
                    e,
                    D3Event::Settle { disposition: d, seat: s, .. }
                        if *d == disposition && *s == seat
                )
            })
            .map(|(i, _)| i)
            .collect()
    }

    /// The single position of a settlement with that exact triple, requiring
    /// there be exactly one.
    fn settlement_of(
        &self,
        identity: &ContinuationCallIdentity,
        disposition: crate::cranelift_backend::lowering::units::CandidateDisposition,
        seat: crate::cranelift_backend::lowering::units::D3Seat,
        why: &str,
    ) -> usize {
        let found = self.settlements_of(identity, disposition, seat);
        assert_eq!(
            found.len(),
            1,
            "{why} -- expected exactly one {disposition:?} settlement at {seat:?} for this exact \
             identity, found {}: {:?}",
            found.len(),
            self.trace
        );
        found[0]
    }

    /// The position of the sole event of a kind, **for this exact identity**.
    fn position_of(
        &self,
        identity: &ContinuationCallIdentity,
        want: fn(&crate::cranelift_backend::lowering::units::D3Event) -> bool,
        why: &str,
    ) -> usize {
        let found = self
            .events_for(identity)
            .filter(|(_, e)| want(e))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        assert_eq!(
            found.len(),
            1,
            "{why} -- expected exactly one such event for this exact identity, found {}: {:?}",
            found.len(),
            self.trace
        );
        found[0]
    }

    /// The binding KIND this exact identity's authorized position received.
    fn binding_kind(
        &self,
        identity: &ContinuationCallIdentity,
    ) -> Option<crate::cranelift_backend::lowering::units::D3BindingKind> {
        use crate::cranelift_backend::lowering::units::D3Event;
        self.events_for(identity).find_map(|(_, e)| match e {
            D3Event::BindingInstalled { kind, .. } => Some(*kind),
            _ => None,
        })
    }

    /// Did the shared direct funnel RETURN an answer? Distinct from "a
    /// settlement was attempted" — row 4 needs exactly that separation.
    fn returned_from_funnel(&self, identity: &ContinuationCallIdentity) -> bool {
        use crate::cranelift_backend::lowering::units::D3Event;
        self.events_for(identity)
            .any(|(_, e)| matches!(e, D3Event::DirectFunnelReturned { .. }))
    }

}

/// **Witness A — the executing composed payload witness**, selected from the
/// live population for rows 1, 2 and 3.
///
/// It is `d8l2`'s payload program: the worker RETURNS its argument, so the
/// payload is passed through the composed call and is observable in the
/// answer. That matters here for a reason beyond convenience — it means this
/// witness has a **result** oracle as well as structural ones, so row 1 can
/// show which of the two actually moves.
///
/// Unmutated it compiles, runs, returns `Int(41)`, installs one binding, and
/// settles exactly one candidate `ComposedCall`.
fn d3_payload_arm(
    mutation: crate::cranelift_backend::lowering::units::D3Mutation,
) -> (D3Arm, String) {
    use crate::cranelift_backend::lowering::units::{
        d1_last_dispositions, d3_plan_candidates, d3_trace, reset_d1_dispositions,
        reset_d3_plan_candidates, reset_d3_trace,
    };
    use crate::cranelift_backend::lowering::{
        d8d_bindings, d8e_consumptions, reset_d8d_bindings,
    };

    reset_d3_trace();
    reset_d3_plan_candidates();
    reset_d1_dispositions();
    reset_d8d_bindings();
    let expr = d8l2_payload_witness(false, 41);
    let (outcome, answer) = with_d3_mutation(mutation, || {
        match compile_expr(&expr, &NativeSeedEnvironment::empty()) {
            Ok(compiled) => match compiled.run(None) {
                Ok((observation, _)) => ("Ok".to_string(), format!("{observation:?}")),
                Err(error) => ("Ok".to_string(), format!("run-err {error:?}")),
            },
            Err(error) => (format!("{error:?}"), "<did not compile>".to_string()),
        }
    });
    (
        D3Arm {
            outcome,
            dispositions: d1_last_dispositions(),
            trace: d3_trace(),
            plan_candidates: d3_plan_candidates(),
            bindings: d8d_bindings(),
            consumptions: d8e_consumptions(),
        },
        answer,
    )
}

/// **Witness B — the one-token direct emission witness**, selected from the
/// live population for rows 4 and 5.
///
/// `contspec_emission_witness` through `ac11_compiles`: a selected
/// `FunctionizedUnits` artifact with **exactly one** candidate, settling
/// `DirectCall` at the shared funnel, inside a successful closeout. One
/// candidate and one disposition is what lets rows 4 and 5 name the whole
/// population without a fixed count standing in for it.
fn d3_contspec_arm(
    mutation: crate::cranelift_backend::lowering::units::D3Mutation,
) -> D3Arm {
    use crate::cranelift_backend::lowering::units::{
        d1_last_dispositions, d3_plan_candidates, d3_trace, reset_d1_dispositions,
        reset_d3_plan_candidates, reset_d3_trace,
    };

    reset_d3_trace();
    reset_d3_plan_candidates();
    reset_d1_dispositions();
    let outcome = with_d3_mutation(mutation, || {
        match ac11_compiles(&contspec_emission_witness()) {
            Ok(()) => "Ok".to_string(),
            Err(error) => format!("{error:?}"),
        }
    });
    D3Arm {
        outcome,
        dispositions: d1_last_dispositions(),
        trace: d3_trace(),
        plan_candidates: d3_plan_candidates(),
        bindings: 0,
        consumptions: 0,
    }
}

/// **Witness C — the binding-dependent composed witness**, selected for row 1.
///
/// `d8f`'s composed program, whose baseline success is not my claim to make:
/// `d8g_the_composed_selected_argument_reaches_its_target_at_the_shared_emitter`
/// already asserts `d8f_compile(false)` compiles, and has since before this
/// node. It projects exactly one candidate, installs a `StaticWorker` at that
/// identity's authorized position, and settles it `ComposedCall`.
///
/// It replaces the executing payload witness for row 1, and the reason is the
/// Architect's block: under suppression that witness **silently succeeded** —
/// compiled, ran, returned the same answer, settled `InlineNoCall` and closed.
/// Observing a silent degradation is not clause 5. This witness's suppression
/// is REFUSED, by a guard that already exists in production.
fn d3_binding_dependent_arm(
    mutation: crate::cranelift_backend::lowering::units::D3Mutation,
) -> D3Arm {
    use crate::cranelift_backend::lowering::units::{
        d1_last_dispositions, d3_plan_candidates, d3_trace, reset_d1_dispositions,
        reset_d3_plan_candidates, reset_d3_trace,
    };
    use crate::cranelift_backend::lowering::{
        d8d_bindings, d8e_consumptions, reset_d8d_bindings,
    };

    reset_d3_trace();
    reset_d3_plan_candidates();
    reset_d1_dispositions();
    reset_d8d_bindings();
    let outcome = with_d3_mutation(mutation, || match d8f_compile(false) {
        None => "Ok".to_string(),
        Some(error) => format!("{error:?}"),
    });
    D3Arm {
        outcome,
        dispositions: d1_last_dispositions(),
        trace: d3_trace(),
        plan_candidates: d3_plan_candidates(),
        bindings: d8d_bindings(),
        consumptions: d8e_consumptions(),
    }
}

/// The exact fail-closed refusal a suppressed binding reaches, and it names
/// the substitution itself rather than a downstream consequence of it.
///
/// This is an EXISTING production guard. Nothing was added or reworded to
/// obtain a discriminator.
const D3_IH_MARKER_ON_VALUE: &str = "computational IH marker was applied to an ordinary value";

/// The terminal refusal rows 2 and 3 SHARE. Named once, so the fact that it is
/// shared is visible in the source rather than being something a reader has to
/// notice by comparing two string literals.
const D3_DOUBLE_SETTLEMENT: &str = "one binding candidate was settled twice";

/// **`D3` `AC-6` row 1 — suppressing the binding installation.**
///
/// The mutation withholds the `StaticWorker` capsule a composed candidate
/// authorizes and substitutes a plain specialized value at the same position.
/// The candidate itself is untouched, so nothing about the candidate ledger
/// changes; what changes is what its authorized position can be CALLED as.
///
/// **The armed run is REFUSED, by a guard that already exists in production
/// and whose message names the substitution itself:** a computational
/// induction-hypothesis marker applied to an ordinary value. Nothing was added
/// or reworded to obtain that discriminator.
///
/// **This row was rebuilt after an Architect block, and the reason is worth
/// keeping.** It first ran on the executing payload witness, where suppression
/// **silently succeeded** — the program compiled, ran, returned the identical
/// answer, settled `InlineNoCall` instead of `ComposedCall`, and closed. The
/// row asserted that structural difference and called it proof. It is not:
/// `AC-6` requires the armed run to reach a refusal or a closeout failure, and
/// *observing* a silent degradation is exactly the bad state mutation 1 exists
/// to make red rather than an acceptable terminal outcome. The witness moved;
/// the assertions were not inverted.
///
/// **The baseline's success is not my claim.**
/// `d8g_the_composed_selected_argument_reaches_its_target_at_the_shared_emitter`
/// has asserted `d8f_compile(false)` compiles since before this node.
///
/// **Promise class: durable invariant.** It asserts that a candidate's
/// authorized position cannot lose its capsule and still reach a successful
/// artifact — a relation between the binding kind at one identity and that
/// same identity's fate, not a literal.
#[test]
fn ced_d3_m1_suppressing_the_binding_installation_is_refused_at_the_ih_marker_guard() {
    use crate::cranelift_backend::lowering::units::{
        CandidateDisposition, D3BindingKind, D3Mutation, D3Seat,
    };

    let baseline = d3_binding_dependent_arm(D3Mutation::None);
    let armed = d3_binding_dependent_arm(D3Mutation::SuppressBindingInstallation);

    // Clause 1 — the unmutated witness SUCCEEDS, and settles its one candidate
    // at the downstream boundary. Without both halves the row is about a
    // program that was already failing, or about one whose candidate never
    // reached a disposition at all.
    assert_eq!(
        baseline.outcome, "Ok",
        "the unmutated witness must compile: {}",
        baseline.outcome
    );
    let identity = baseline.the_candidate();
    baseline.settlement_of(
        &identity,
        CandidateDisposition::ComposedCall,
        D3Seat::ComposedPromotion,
        "the baseline must settle THIS identity as ComposedCall at the promotion seat, which is \
         the downstream boundary the suppression will stop it from reaching",
    );
    assert_eq!(
        baseline.dispositions.get(&CandidateDisposition::ComposedCall).copied(),
        Some(1),
        "and the artifact must CLOSE with that disposition recorded: {:?}",
        baseline.dispositions
    );
    assert!(
        baseline.bindings > 0 && baseline.consumptions > 0,
        "and it must install and consume a static-worker binding, or there is nothing for this \
         mutation to withhold: bindings={} consumptions={}",
        baseline.bindings,
        baseline.consumptions
    );

    // Clause 2 — the SAME identity in both arms, selected from the live plan
    // by type rather than compared as a rendering.
    assert_eq!(
        armed.the_candidate(),
        identity,
        "both arms must project the same candidate from the live plan, or they are about two \
         different edges and every comparison below means nothing"
    );

    // Clause 4 — THE BINDING SEAT, keyed by that identity. The mutation is
    // observed where the choice is made, not inferred from its consequences.
    assert_eq!(
        baseline.binding_kind(&identity),
        Some(D3BindingKind::StaticWorker),
        "the baseline must install a StaticWorker at THIS identity's authorized position: {:?}",
        baseline.trace
    );
    assert_eq!(
        armed.binding_kind(&identity),
        Some(D3BindingKind::Value),
        "and the armed run must substitute a plain Value at the SAME identity's position. This \
         is the one thing that moved: {:?}",
        armed.trace
    );

    // Clause 5 — THE ARMED RUN IS REFUSED, at the guard that names the
    // substitution. This is the clause the previous witness could not
    // discharge, and it is why the witness was replaced rather than the
    // assertions reworded.
    assert!(
        armed.outcome.contains(D3_IH_MARKER_ON_VALUE),
        "the armed run must reach the fail-closed IH-marker guard. A different refusal would mean \
         the suppression is being caught somewhere else and this row is not attributing it: {}",
        armed.outcome
    );

    // Clause 5b — and it must never reach the downstream boundary the baseline
    // did. Without this the refusal above is consistent with a compile that
    // failed for an unrelated reason after settling normally.
    assert!(
        armed
            .settlements_of(
                &identity,
                CandidateDisposition::ComposedCall,
                D3Seat::ComposedPromotion
            )
            .is_empty(),
        "the armed run must NOT settle this identity at the promotion seat: {:?}",
        armed.trace
    );
    assert!(
        armed.dispositions.is_empty(),
        "and no artifact may close at all, so the candidate reaches no disposition rather than \
         reaching a different one. A non-empty tally here would mean the suppression was absorbed \
         into a successful close, which is precisely the silent degradation this row was rebuilt \
         to reject: {:?}",
        armed.dispositions
    );
}

/// **`D3` `AC-6` row 2 — settling `InlineNoCall` on bridge ENTRY.**
///
/// The defect is settling before the scope is known to have completed. Its
/// causal chain, and every step is pinned:
///
/// 1. bridge entry sees the candidate **unsettled and with no pending composed
///    record**;
/// 2. the mutation settles `InlineNoCall` **at the entry seat**, before the
///    bridge body runs;
/// 3. the body later records the composed claim;
/// 4. finished-CLIF promotion collides.
///
/// **Shares its terminal refusal with row 3 and is not keyed on it.** The
/// discriminator is step 2: a settlement at `BridgeEntry`, which row 3 never
/// makes.
///
/// **Promise class: durable invariant** — it pins an ordering relation between
/// two seats, not a message.
#[test]
fn ced_d3_m2_settling_inline_on_bridge_entry_collides_at_promotion_from_the_entry_seat() {
    use crate::cranelift_backend::lowering::units::{
        CandidateDisposition, D3Event, D3Mutation, D3Seat,
    };

    let (baseline, _) = d3_payload_arm(D3Mutation::None);
    let (armed, _) = d3_payload_arm(D3Mutation::MarkInlineBeforeBridgeCompletion);

    // Clause 1 — the unmutated arm succeeds and settles ONCE.
    assert_eq!(
        baseline.outcome, "Ok",
        "the unmutated witness must compile: {}",
        baseline.outcome
    );
    assert_eq!(
        baseline.dispositions.get(&CandidateDisposition::ComposedCall).copied(),
        Some(1),
        "and settle its one candidate exactly once, as ComposedCall: {:?}",
        baseline.dispositions
    );
    assert_eq!(
        baseline.settle_seats(),
        vec![D3Seat::ComposedPromotion],
        "the baseline makes exactly ONE settlement attempt, at the promotion seat. It performs \
         no entry settlement and no exit settlement: the exit path reads a pending composed claim \
         and correctly leaves the candidate alone: {:?}",
        baseline.settle_seats()
    );

    // Clause 2 — the same identity in both arms, typed, from the live plan.
    let identity = baseline.the_candidate();
    assert_eq!(
        armed.the_candidate(),
        identity,
        "both arms must project the same candidate from the live plan"
    );
    baseline.settlement_of(
        &identity,
        CandidateDisposition::ComposedCall,
        D3Seat::ComposedPromotion,
        "the baseline must settle THIS identity as ComposedCall at the promotion seat",
    );

    // Clause 4 — THE CAUSAL CHAIN, in order. This is row 2's own oracle and it
    // is what row 3 cannot satisfy.
    let entry = armed.position_of(
        &identity,
        |e| matches!(e, D3Event::BridgeEntry { .. }),
        "the armed run must enter the bridge with THIS identity bypassed",
    );
    match &armed.trace[entry] {
        D3Event::BridgeEntry {
            settled,
            pending_composed,
            ..
        } => assert_eq!(
            (*settled, *pending_composed),
            (false, false),
            "step 1 -- bridge entry must see the candidate UNSETTLED and with NO pending composed \
             record. Either being true here would mean the mutation is settling over something \
             that already existed, which is a different defect"
        ),
        other => panic!("expected a bridge entry: {other:?}"),
    }
    let entry_settle = armed.settlement_of(
        &identity,
        CandidateDisposition::InlineNoCall,
        D3Seat::BridgeEntry,
        "step 2 -- the mutation must settle THIS identity as InlineNoCall at the ENTRY seat. \
         This is row 2's discriminator: row 3 makes no entry settlement at all, so a control \
         that omitted this clause would be green under either mutation and would supply one \
         proof rather than two",
    );
    let recorded = armed.position_of(
        &identity,
        |e| matches!(e, D3Event::ComposedRecorded { .. }),
        "step 3 -- the body must go on to record a composed claim for the same identity",
    );
    let promotion = armed.settlement_of(
        &identity,
        CandidateDisposition::ComposedCall,
        D3Seat::ComposedPromotion,
        "step 4 -- finished-CLIF promotion must attempt ComposedCall on the same identity",
    );
    assert!(
        entry < entry_settle && entry_settle < recorded && recorded < promotion,
        "and the four steps must occur IN THAT ORDER. The ordering is the mechanism: settling \
         before the composed claim is recorded is precisely the defect, and the same four events \
         in a different order would be a different one: entry={entry} entry_settle={entry_settle} \
         recorded={recorded} promotion={promotion}"
    );

    // Clause 5 — the terminal refusal. CORROBORATION, deliberately last, and
    // deliberately shared with row 3.
    assert!(
        armed.outcome.contains(D3_DOUBLE_SETTLEMENT),
        "the collision must reach the double-settlement refusal: {}",
        armed.outcome
    );
}

/// **`D3` `AC-6` row 3 — dropping the pending-composed half of the consumed
/// test.**
///
/// The defect is at the OTHER end of the same window. Its causal chain:
///
/// 1. the composed claim is **already recorded** and pending at bridge
///    completion;
/// 2. the mutation suppresses the pending-half read, so the exit path believes
///    the candidate unconsumed;
/// 3. it therefore settles `InlineNoCall` **at the exit seat**;
/// 4. finished-CLIF promotion collides.
///
/// **And it proves the NEGATIVE that separates it from row 2: no entry
/// settlement occurred.** Without that clause the two rows would be
/// distinguishable only by their shared terminal string, which is the collapse
/// the ruling forbids.
///
/// **This is the landed timing invariant, stated as a failure:** a composed
/// claim is RECORDED during lowering and PROMOTED after verification, and the
/// pending feed is the only thing visible at both times. Reading only the
/// settled half is exactly what makes the window unsafe.
///
/// **Promise class: durable invariant.**
#[test]
fn ced_d3_m3_dropping_the_pending_half_settles_inline_at_the_exit_seat_and_collides() {
    use crate::cranelift_backend::lowering::units::{
        CandidateDisposition, D3Event, D3Mutation, D3Seat,
    };

    let (baseline, _) = d3_payload_arm(D3Mutation::None);
    let (armed, _) = d3_payload_arm(D3Mutation::MarkInlineAfterComposedCall);

    // Clause 1 — the unmutated arm succeeds, and specifically DOES NOT settle
    // inline at the exit seat: it reads the pending half and leaves the
    // candidate for promotion.
    assert_eq!(
        baseline.outcome, "Ok",
        "the unmutated witness must compile: {}",
        baseline.outcome
    );
    assert!(
        !baseline.settle_seats().contains(&D3Seat::BridgeExit),
        "and must make NO exit settlement -- the pending composed claim is what stops it. If the \
         baseline settled here, the mutation would not be suppressing anything: {:?}",
        baseline.settle_seats()
    );

    // Clause 2 — same identity in both arms, typed, from the live plan.
    let identity = baseline.the_candidate();
    assert_eq!(
        armed.the_candidate(),
        identity,
        "both arms must project the same candidate from the live plan"
    );
    baseline.settlement_of(
        &identity,
        CandidateDisposition::ComposedCall,
        D3Seat::ComposedPromotion,
        "the baseline must settle THIS identity as ComposedCall at the promotion seat",
    );

    // Clause 4 — THE CAUSAL CHAIN, and it is a different one from row 2's.
    let recorded = armed.position_of(
        &identity,
        |e| matches!(e, D3Event::ComposedRecorded { .. }),
        "step 1 -- a composed claim for THIS identity must be recorded during lowering",
    );
    let exit = armed.position_of(
        &identity,
        |e| matches!(e, D3Event::BridgeExit { .. }),
        "the armed run must complete the bridge for the same identity",
    );
    match &armed.trace[exit] {
        D3Event::BridgeExit {
            completed,
            settled,
            pending_composed,
            ..
        } => assert_eq!(
            (*completed, *settled, *pending_composed),
            (true, false, true),
            "step 2 -- at bridge completion the scope must have COMPLETED, the candidate must be \
             UNSETTLED, and a composed claim must be PENDING. That last one is the whole row: \
             the pending half is TRUE here, and the mutation makes the exit path fail to read it. \
             These three are read straight from the ledger and the feed, NOT through \
             `continuation_candidate_is_consumed` -- that is the function the mutation mutates, \
             and an instrument reading through it would agree with the mutation instead of \
             exposing it"
        ),
        other => panic!("expected a bridge exit: {other:?}"),
    }
    let exit_settle = armed.settlement_of(
        &identity,
        CandidateDisposition::InlineNoCall,
        D3Seat::BridgeExit,
        "step 3 -- the mutation must settle THIS identity as InlineNoCall at the EXIT seat",
    );
    let promotion = armed.settlement_of(
        &identity,
        CandidateDisposition::ComposedCall,
        D3Seat::ComposedPromotion,
        "step 4 -- finished-CLIF promotion must attempt ComposedCall on the same identity",
    );
    assert!(
        recorded < exit && exit < exit_settle && exit_settle < promotion,
        "and in that order: recorded={recorded} exit={exit} exit_settle={exit_settle} \
         promotion={promotion}"
    );

    // Clause 4b — THE NEGATIVE THAT SEPARATES THIS ROW FROM ROW 2.
    assert!(
        armed
            .settlements_of(&identity, CandidateDisposition::InlineNoCall, D3Seat::BridgeEntry)
            .is_empty(),
        "NO entry settlement of this identity may have occurred. This is what makes rows 2 and 3 \
         two proofs rather than one: they share a terminal refusal, and without this clause a \
         control keyed on that refusal plus 'something settled inline' would pass under either \
         mutation: {:?}",
        armed.trace
    );

    // Clause 5 — the shared terminal refusal, as corroboration.
    assert!(
        armed.outcome.contains(D3_DOUBLE_SETTLEMENT),
        "the collision must reach the double-settlement refusal: {}",
        armed.outcome
    );
}

/// **`D3` `AC-6` row 4 — omitting the direct settlement.**
///
/// The direct call and its funnel return are **preserved**; only the
/// settlement is withheld. The candidate therefore reaches the artifact
/// closeout with no disposition, and `D2`'s totality check — which runs BEFORE
/// the call-obligation subset is derived — refuses.
///
/// **This row is the one that proves `D2`'s ordering is load-bearing.** An
/// unsettled candidate is in neither `DirectCall` nor `ComposedCall`, so if the
/// subset were derived first it would simply fall out and pass silently. The
/// refusal below only exists because totality is checked first.
///
/// **Promise class: durable invariant.** It asserts that an unsettled
/// candidate cannot reach a successful close, over a witness whose whole
/// candidate population is one.
#[test]
fn ced_d3_m4_omitting_the_direct_settlement_preserves_the_call_and_fails_candidate_totality() {
    use crate::cranelift_backend::lowering::units::{CandidateDisposition, D3Mutation, D3Seat};

    let baseline = d3_contspec_arm(D3Mutation::None);
    let armed = d3_contspec_arm(D3Mutation::OmitFinalDisposition);

    // Clause 1 — the unmutated witness compiles and settles its one candidate.
    assert_eq!(
        baseline.outcome, "Ok",
        "the unmutated witness must compile: {}",
        baseline.outcome
    );
    assert_eq!(
        baseline.dispositions.get(&CandidateDisposition::DirectCall).copied(),
        Some(1),
        "and settle exactly one candidate DirectCall. Exactly one is what lets this row speak \
         about the WHOLE population without a fixed count standing in for it: {:?}",
        baseline.dispositions
    );

    // Clause 2 — the same identity reaches the same funnel in both arms,
    // typed, selected from the live plan.
    let identity = baseline.the_candidate();
    assert_eq!(
        armed.the_candidate(),
        identity,
        "both arms must project the same candidate from the live plan"
    );
    baseline.settlement_of(
        &identity,
        CandidateDisposition::DirectCall,
        D3Seat::DirectFunnel,
        "the baseline must settle THIS identity as DirectCall at the funnel",
    );

    // Clause 4 — the mutation-specific causal observation: THE CALL IS STILL
    // MADE. Without this the row would be equally consistent with a mutation
    // that suppressed the call itself, which is a different defect entirely.
    assert!(
        armed.returned_from_funnel(&identity),
        "the direct call for THIS identity must still have been made and the funnel must still \
         have RETURNED. Without this clause the row is equally consistent with a mutation that \
         suppressed the call itself, which is a different defect and would fail totality for a \
         different reason: {:?}",
        armed.trace
    );
    assert!(
        armed
            .settlements_of(&identity, CandidateDisposition::DirectCall, D3Seat::DirectFunnel)
            .is_empty(),
        "and NO settlement of it may be attempted -- that is the only thing withheld: {:?}",
        armed.trace
    );
    assert!(
        armed.settle_seats().is_empty(),
        "nor any settlement of anything else, so the whole population is unsettled: {:?}",
        armed.settle_seats()
    );
    assert!(
        baseline.returned_from_funnel(&identity)
            && baseline.settle_seats() == vec![D3Seat::DirectFunnel],
        "while the baseline both returns from the funnel and settles once, which is what makes \
         the pair above a difference of exactly one step: {:?}",
        baseline.trace
    );
    assert!(
        armed.dispositions.is_empty(),
        "so nothing is settled at all: {:?}",
        armed.dispositions
    );

    // Clause 5 — candidate TOTALITY fails at the composite close, and it must
    // be that clause rather than a later one.
    assert!(
        armed
            .outcome
            .contains("reached the artifact closeout without a disposition"),
        "the close must refuse on candidate totality. A declared/resolved or claim-equality \
         message here would mean the failure moved to a later clause and this row is measuring \
         something else: {}",
        armed.outcome
    );
    assert!(
        !armed.outcome.contains(D3_DOUBLE_SETTLEMENT),
        "and specifically not the double-settlement refusal, which is rows 2, 3 and 5's terminal: \
         {}",
        armed.outcome
    );
}

/// **`D3` `AC-6` row 5 — settling the same direct candidate twice.**
///
/// The second settlement is refused **at the seat that makes it**, not deferred
/// to closeout — which is why its message can name the collision, and why this
/// row's terminal differs from row 4's even though both mutate the same funnel.
///
/// **It is also the both-times-the-SAME-disposition arm of that refusal**,
/// which rows 2 and 3 do not reach: they collide `InlineNoCall` against
/// `ComposedCall`. So `settle`'s two refusal arms are both covered by this
/// node, by different rows, rather than one arm standing in for both.
///
/// **Promise class: durable invariant.**
#[test]
fn ced_d3_m5_settling_the_direct_candidate_twice_is_refused_at_the_second_settlement() {
    use crate::cranelift_backend::lowering::units::{
        CandidateDisposition, D3Mutation, D3Seat,
    };

    let baseline = d3_contspec_arm(D3Mutation::None);
    let armed = d3_contspec_arm(D3Mutation::DoubleDisposition);

    // Clause 1 — the unmutated witness compiles, settling exactly once.
    assert_eq!(
        baseline.outcome, "Ok",
        "the unmutated witness must compile: {}",
        baseline.outcome
    );
    assert_eq!(
        baseline.settle_seats(),
        vec![D3Seat::DirectFunnel],
        "making exactly ONE settlement attempt, at the funnel. This is the clause the armed \
         arm below moves, and pinning it as the whole sequence rather than as a count is what \
         makes 'twice' mean twice at this seat: {:?}",
        baseline.settle_seats()
    );
    assert_eq!(
        baseline.dispositions.get(&CandidateDisposition::DirectCall).copied(),
        Some(1),
        "and the candidate settles DirectCall: {:?}",
        baseline.dispositions
    );

    // Clause 2 — the same identity, twice over, in the armed arm.
    let identity = baseline.the_candidate();
    assert_eq!(
        armed.the_candidate(),
        identity,
        "both arms must project the same candidate from the live plan"
    );

    // Clause 4 — the mutation-specific causal observation: TWO attempts at the
    // SAME seat. Row 4 makes none; rows 2 and 3 make theirs at bridge seats.
    assert_eq!(
        armed
            .settlements_of(&identity, CandidateDisposition::DirectCall, D3Seat::DirectFunnel)
            .len(),
        2,
        "exactly two DirectCall settlement attempts on THIS SAME identity at the funnel. Keyed \
         on the triple rather than on the seat: two attempts on two different candidates would \
         be lawful, and is not what this row is about: {:?}",
        armed.trace
    );
    assert_eq!(
        armed.settle_seats(),
        vec![D3Seat::DirectFunnel, D3Seat::DirectFunnel],
        "and the arm attempts nothing else anywhere: {:?}",
        armed.settle_seats()
    );
    assert!(
        armed.dispositions.is_empty(),
        "and the artifact never closes, so no disposition tally survives: {:?}",
        armed.dispositions
    );

    // Clause 5 — refused immediately, and on the SAME-disposition arm.
    assert!(
        armed.outcome.contains(D3_DOUBLE_SETTLEMENT),
        "the second settlement must be refused: {}",
        armed.outcome
    );
    assert!(
        armed.outcome.contains("both times as DirectCall"),
        "and on the both-times-the-SAME-disposition arm of that refusal, which rows 2 and 3 do \
         not reach -- they collide InlineNoCall against ComposedCall. This is what keeps the two \
         arms of `settle`'s refusal separately witnessed: {}",
        armed.outcome
    );
}

/// **`D3` `AC-6` — the five rows are FIVE proofs, and this is the residue of
/// proving it.**
///
/// **Why this exists as a committed test rather than a verified claim.**
/// The five rows above were each shown to red when their own mutation is not
/// armed — a clean 5×5 diagonal. That is necessary and it is **not
/// sufficient**, because it does not rule out the one failure the ruling
/// actually forbids: rows 2 and 3 share a terminal refusal, so a control keyed
/// on that refusal plus "something settled inline" would be green under
/// **either** mutation and would supply one proof while appearing to supply
/// two.
///
/// That was verified by cross-arming each row's control with its partner's
/// mutation and watching all four fail. **A verification that lives in a
/// terminal evaporates when the terminal closes** — so the discriminating
/// observations are asserted here, over the partner mutation, as a committed
/// artifact. Delete any row's causal clause above and this reds.
///
/// **Exactly one variant is armed per arm**, as the frame requires. This is
/// four separate single-mutation runs, not a combined one.
///
/// **Promise class: durable invariant** — it asserts that four specific
/// observations are *absent* under the partner mutation, which stays true for
/// any future shape of these seats that keeps the rows independent.
#[test]
fn ced_d3_the_five_rows_are_five_proofs_and_not_one_shared_terminal() {
    use crate::cranelift_backend::lowering::units::{
        CandidateDisposition, D3BindingKind, D3Event, D3Mutation, D3Seat,
    };

    // Row 1 against all four others, on its own witness.
    //
    // QA's block, and it was the same defect one layer up: the previous commit
    // RAN this and reported it, in a message whose own subject line says a
    // verification living in a terminal evaporates when the terminal closes.
    // Naming a trap does not inoculate you against it. Here it is, committed.
    //
    // Row 1's discriminator is the pair (binding kind at the typed identity,
    // the fail-closed guard its substitution reaches). Neither half alone is
    // enough: another mutation could in principle reach the same guard for a
    // different reason, or leave the binding alone and fail there anyway.
    let m1 = d3_binding_dependent_arm(D3Mutation::SuppressBindingInstallation);
    let c_identity = m1.the_candidate();
    assert_eq!(
        m1.binding_kind(&c_identity),
        Some(D3BindingKind::Value),
        "positive control -- mutation 1 must substitute a Value at this identity, or the four \
         negatives below are about a discriminator nothing satisfies and pass for free: {:?}",
        m1.trace
    );
    assert!(
        m1.outcome.contains(D3_IH_MARKER_ON_VALUE),
        "positive control -- and must reach the IH-marker guard: {}",
        m1.outcome
    );

    for other in [
        D3Mutation::MarkInlineBeforeBridgeCompletion,
        D3Mutation::MarkInlineAfterComposedCall,
        D3Mutation::OmitFinalDisposition,
        D3Mutation::DoubleDisposition,
    ] {
        let arm = d3_binding_dependent_arm(other);
        assert_eq!(
            arm.the_candidate(),
            c_identity,
            "each cross-arm must be about the SAME edge as row 1, or the negatives below are \
             about a different candidate: {other:?}"
        );
        assert_eq!(
            arm.binding_kind(&c_identity),
            Some(D3BindingKind::StaticWorker),
            "{other:?} must leave the binding a StaticWorker. Only mutation 1 touches the \
             binding seat, and if another mutation reached Value here, row 1's binding-seat \
             half would be satisfied by it: {:?}",
            arm.trace
        );
        assert!(
            !arm.outcome.contains(D3_IH_MARKER_ON_VALUE),
            "and {other:?} must NOT reach the IH-marker guard. That guard is row 1's terminal, \
             and a second mutation arriving at it would make row 1's outcome clause satisfiable \
             by something other than the suppression it attributes it to: {}",
            arm.outcome
        );
    }

    // And WHY the last two negatives hold, pinned rather than left implicit:
    // witness C's candidate never reaches the direct funnel, so mutations 4 and
    // 5 have no seat to act on and this program is unchanged by them. If that
    // ever stops being true the pair above would start passing for a different
    // reason, so it is asserted rather than assumed.
    for inert in [D3Mutation::OmitFinalDisposition, D3Mutation::DoubleDisposition] {
        let arm = d3_binding_dependent_arm(inert);
        assert_eq!(
            arm.outcome, "Ok",
            "{inert:?} must be INERT on witness C -- its candidate never reaches the direct \
             funnel, so there is no seat for these two to move: {}",
            arm.outcome
        );
        assert_eq!(
            arm.settlements_of(
                &c_identity,
                CandidateDisposition::ComposedCall,
                D3Seat::ComposedPromotion
            )
            .len(),
            1,
            "and it must still settle once at the promotion seat, exactly as the baseline does"
        );
    }

    // The pair that shares a terminal refusal.

    // Row 2's discriminator is a settlement at the ENTRY seat. Mutation 3
    // reaches the SAME terminal refusal and must NOT make one.
    let (under_m3, _) = d3_payload_arm(D3Mutation::MarkInlineAfterComposedCall);
    assert!(
        under_m3.outcome.contains(D3_DOUBLE_SETTLEMENT),
        "precondition -- mutation 3 must reach the shared terminal, or this arm is not testing \
         the collapse at all: {}",
        under_m3.outcome
    );
    let m3_identity = under_m3.the_candidate();
    assert!(
        under_m3
            .settlements_of(
                &m3_identity,
                CandidateDisposition::InlineNoCall,
                D3Seat::BridgeEntry
            )
            .is_empty(),
        "⇒ and it must make NO entry settlement of that identity. If it did, row 2's \
         discriminator would be satisfied by mutation 3 and the two rows would be one proof \
         wearing two names: {:?}",
        under_m3.trace
    );

    // Row 3's discriminator is an EXIT settlement made while a composed claim
    // was already pending. Mutation 2 must not produce that either: it settles
    // at entry, so by the time the exit is reached the candidate is already
    // settled and the exit path correctly leaves it alone.
    let (under_m2, _) = d3_payload_arm(D3Mutation::MarkInlineBeforeBridgeCompletion);
    assert!(
        under_m2.outcome.contains(D3_DOUBLE_SETTLEMENT),
        "precondition -- mutation 2 must reach the shared terminal: {}",
        under_m2.outcome
    );
    let m2_identity = under_m2.the_candidate();
    assert!(
        under_m2
            .settlements_of(
                &m2_identity,
                CandidateDisposition::InlineNoCall,
                D3Seat::BridgeExit
            )
            .is_empty(),
        "⇒ and it must make NO exit settlement of that identity, or row 3's discriminator would \
         be satisfied by mutation 2: {:?}",
        under_m2.trace
    );
    // And the exit it does reach sees the candidate ALREADY settled — which is
    // the structural reason the two chains cannot be confused, stated as a
    // measurement rather than as the argument above.
    let exit_at = under_m2.position_of(
        &m2_identity,
        |e| matches!(e, D3Event::BridgeExit { .. }),
        "mutation 2's run must still complete the bridge for that identity",
    );
    let exit = match &under_m2.trace[exit_at] {
        D3Event::BridgeExit {
            settled,
            pending_composed,
            ..
        } => (*settled, *pending_composed),
        other => panic!("expected a bridge exit: {other:?}"),
    };
    assert_eq!(
        exit,
        (true, true),
        "under mutation 2 the exit must see the candidate ALREADY SETTLED (by the entry seat) \
         with the composed claim pending. Under mutation 3 the same read is (false, true) -- \
         unsettled with a claim pending -- and that difference is what the two rows key on"
    );

    // The pair on the direct funnel.

    // Row 4's discriminator is a returned funnel with NO settlement attempt.
    // Mutation 5 returns from the same funnel and must attempt two.
    let under_m5 = d3_contspec_arm(D3Mutation::DoubleDisposition);
    let m5_identity = under_m5.the_candidate();
    assert!(
        under_m5.returned_from_funnel(&m5_identity) && !under_m5.settle_seats().is_empty(),
        "mutation 5 must return from the funnel AND attempt a settlement, or row 4's \
         'returned but settled nothing' would also describe it: {:?}",
        under_m5.settle_seats()
    );

    // Row 5's discriminator is two attempts at one seat. Mutation 4 makes none.
    let under_m4 = d3_contspec_arm(D3Mutation::OmitFinalDisposition);
    assert_ne!(
        under_m4.settle_seats().len(),
        2,
        "mutation 4 must not make two settlement attempts, or row 5's discriminator would be \
         satisfied by it: {:?}",
        under_m4.settle_seats()
    );
    assert!(
        !under_m4.outcome.contains(D3_DOUBLE_SETTLEMENT),
        "and it must not reach the double-settlement terminal at all -- unlike rows 2, 3 and 5, \
         row 4's terminal is candidate totality, and that separation is what makes its row \
         attributable without a causal clause doing all the work: {}",
        under_m4.outcome
    );
}

/// **`RT-CALL-EDGE-EXECUTABILITY-AXIS` — the boundary sentinel.**
///
/// ⛔ **THIS DOES NOT DISCHARGE `AC-2`, and must not be reported as doing so.**
/// `AC-2` asks for a control exercising a template-only callee **whose two axes
/// differ**. That population does not exist today. This row detects the moment
/// it starts to, and nothing more.
///
/// **It ranges over call edges joined to their callee descriptors, and detects
/// the exact disagreement between the two filters on the SAME callee:**
///
/// ```text
/// template_only.contains(body_occurrence) && !template_only.contains(entry_origin)
/// ```
///
/// That conjunction is the defect's own failure direction: the callee's body is
/// superseded, so the repaired body-axis filter drops the edge, while the old
/// entry-axis probe kept it — and the edge then reached a unit with no emitted
/// `Function`.
///
/// ⚠ **Why it is not keyed on "some split-axis unit exists".** That was the
/// proxy this row replaces, and it is wrong in both directions: an unrelated
/// split-axis unit sitting beside an unrelated template-only body is **not** the
/// `AC-2` population, so the proxy would red on a combination that proves
/// nothing; and a unit whose axes differ without either origin being in the set
/// is equally irrelevant. Only the per-callee conjunction above is the
/// population, so only it is measured.
///
/// **What the current witness gives, measured rather than assumed:** the `D5a`
/// witness supersedes one worker body and every one of its call edges agrees
/// under both readings, so the repaired filter and the old one are
/// indistinguishable here. That is precisely why a passing suite is not evidence
/// about the axis, and why `AC-2` stays open.
///
/// **The population that would close it:** a fixture that both fully retargets a
/// worker body — today only this witness does — and has that body's selecting
/// unit schedule something before itself, which today only the `b2ac`
/// `computational` shapes do, and they generate no contexts at all. Concretely a
/// **nested-post-effect specialization whose superseded worker body is a
/// computational match**. That is a new planner fixture, not a variation.
///
/// PROMISE CLASS: transition sentinel, named for the boundary rather than a
/// count. It reds **deliberately** the first time a call edge exhibits the
/// conjunction — exactly when `AC-2`'s real control becomes writable. Retire it
/// then.
#[test]
fn call_edge_executability_axis_the_two_filters_cannot_yet_disagree_on_any_callee() {
    with_d5a_witness_plan(|plan| {
        let template_only = plan
            .template_only_worker_bodies()
            .expect("the superseded set");
        assert!(
            !template_only.is_empty(),
            "the witness no longer supersedes any worker body, so it is not the D5a population \
             this sentinel is about and the scan below is vacuous"
        );

        let units = plan.emittable_units().expect("emittable units");
        let edges = plan.emittable_call_edges().expect("call edges");
        // Mirror production's last-wins function-to-body join. A linear
        // first-match scan would ask a different question if duplicate
        // descriptors ever became reachable.
        let body_axis: BTreeMap<PredeclaredFunctionId, StaticOriginId> = units
            .into_iter()
            .map(|unit| (unit.function(), unit.body_occurrence()))
            .collect();

        let mut joined = 0usize;
        let mut superseded_callees = 0usize;
        let mut divergent = Vec::new();
        for edge in &edges {
            let Some(body) = body_axis.get(&edge.callee()) else {
                continue;
            };
            joined += 1;
            let entry = edge.callee_origin();
            if template_only.contains(body) {
                superseded_callees += 1;
            }
            if template_only.contains(body) && !template_only.contains(&entry) {
                divergent.push((edge.callee(), *body, entry));
            }
        }

        assert!(
            joined > 0,
            "no call edge joined to a callee descriptor, so the disagreement scan ranged over \
             nothing"
        );
        assert_eq!(
            joined,
            edges.len(),
            "the disagreement scan silently skipped {} call edge(s) with no callee descriptor",
            edges.len() - joined
        );
        // NON-VACUITY, and the sharper half. The scan does not merely run -- it
        // reaches a call edge whose callee body IS superseded, so the first
        // conjunct fires on real data and the detector sits exactly ONE clause
        // from red. Without this, an empty `divergent` would be equally
        // consistent with "the population is absent" and "the scan never saw a
        // superseded callee at all".
        assert!(
            superseded_callees > 0,
            "no call edge names a callee whose body is superseded, so the disagreement below \
             could never fire and this sentinel would pass for the wrong reason"
        );
        assert!(
            divergent.is_empty(),
            "AC-2's population now EXISTS: a call edge's callee has a superseded BODY whose ENTRY \
             is not superseded {divergent:?}. The two filters disagree in the defect's direction \
             on this callee. Write the real AC-2 control against it and retire this sentinel"
        );
    });
}


/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2b` (planner plane) — the runtime
/// envelope excludes EVERY recursive constructor position.**
///
/// > **MEASURED:** on row 3's plan, whose producer has **two** recursive
/// > positions, every interned continuation unit's closed projection is
/// > set-equal to the checked `recursive_positions`, and its ordinary envelope
/// > names **no** recursive position. **CLAIMED:** no recursive field reaches
/// > the declared-unit ABI, so a sibling `Specialized(Closure)` can no longer be
/// > cloned into the ordinary run. **THE GAP:** this is the planner plane only —
/// > row 3 does not compile (see the handback); the compiler-only binder and the
/// > caller-side reconciliation are not in this candidate.
///
/// ⛔ **The qualifying population is established FIRST, and the exclusions range
/// over it — they are not merely accompanied by a count.** A producer with a
/// single recursive position satisfies every exclusion under **both** the old
/// and the new derivation, because the two coincide there; that is exactly why
/// the defect survived every landed fixture. So `projected.len() > 1` selects
/// the population before any dependent assertion runs, which makes those
/// assertions **unreachable** on a non-sibling fixture rather than vacuously
/// true on one. An independent clause asserts the qualifying population is
/// nonempty, so "nothing qualified" cannot pass as "everything passed".
///
/// This is the same ordering discipline as the `D2a` denominator rider, applied
/// here — and an earlier revision of this very row got it wrong in the way the
/// rider describes: it asserted over every unit and checked the count
/// afterwards.
#[test]
fn d2b_the_runtime_envelope_excludes_every_recursive_position() {
    let expression =
        host_result_closure_match(px8j_recursive_sibling_result(1, 2, px8j_aggregate_result()));
    let plan = plan_static_transition_graph(&expression, &BTreeMap::new())
        .expect("row 3's fixture plans");

    let units = plan.continuation_units().expect("continuation units");
    assert!(
        !units.is_empty(),
        "the fixture interned no continuation unit, so every assertion below is vacuous"
    );

    // ⭐⭐ THE QUALIFYING POPULATION IS ESTABLISHED FIRST, AND EVERYTHING ELSE
    // RANGES OVER IT.
    //
    // ⛔ An earlier revision asserted the exclusions over EVERY unit and checked
    // `sibling_units > 0` afterwards. That ordering is the defect this control
    // exists to catch, one level up: a single-position fixture satisfies every
    // exclusion under BOTH the old and the new derivation, so the assertions
    // would all have passed and the count would have been the only thing
    // standing between a green row and a vacuous one. Qualifying first makes
    // the dependent assertions unreachable on a non-sibling fixture rather than
    // merely accompanied by a caveat.
    let qualified = units
        .iter()
        .filter(|unit| unit.recursive_positions().len() > 1)
        .collect::<Vec<_>>();
    assert!(
        !qualified.is_empty(),
        "no interned unit has more than one recursive position, so this fixture is not the \
         sibling shape and every exclusion below would hold under the single-position \
         derivation too"
    );

    for unit in qualified {
        let projected = unit.recursive_positions();
        debug_assert!(projected.len() > 1, "the qualified filter admitted a single-position unit");

        // The unit's own position is a member, which is what lets the envelope
        // derive its field count from the projection at all.
        assert!(
            projected.contains(&unit.recursive_position()),
            "a unit's own recursive position is absent from its closed projection: {projected:?}"
        );

        // THE EXCLUSION, on a producer that genuinely has siblings.
        let envelope = unit.ordinary_envelope().expect("the envelope builds");
        for role in &envelope {
            if let ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField {
                source_position,
            } = role
            {
                assert!(
                    !projected.contains(source_position),
                    "the runtime envelope names recursive source position {source_position}, so a \
                     recursive field is still an ordinary ABI parameter: projected={projected:?}"
                );
            }
        }
    }
}



/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2k-1b-i` — the CONSERVATION measurement,
/// per row.**
///
/// **The invariant is conservation, not exact-`Var` consumption** (Architect
/// `evt_5etamwj8tp2fh`). Every recognized static worker at constructor field
/// `(owner, position)` gets exactly one disposition before a runtime-value
/// boundary: consumed once at the exact-`Var` call, erased before construction
/// under positive unobservability authority, or refused before emission. **None
/// is dropped.** The predecessor cut shipped exactly the dropped state — four
/// rows compiled with the worker built and forgotten — so this test measures
/// the disposition and never the compile.
///
/// **A row that COMPILES is not a row that PASSES**, which is why `compiles`
/// is one column beside the dispositions rather than the assertion.
///
/// ## What each column is, and which are chosen versus measured
///
/// - `recognized` — the owning constructors at which a worker was recognized
///   ahead of `value_at`. **Chosen literals**, one per row.
/// - `bare_var_reads` — value-producing reads of a static worker. Zero is the
///   producer working: recognition happens before the read is taken.
/// - `installs` — static eliminations that rebound the field into the lexical
///   binding authority **without erasing its kind**. This is the consumption
///   axis, and **it is measured, not chosen**.
/// - `consuming_calls` — exact-`Var` calls that consumed a worker.
/// - `outcome` — the refusal's construct, or `compiled`.
///
/// ## Promise class: MIXED, and the split is stated because it matters
///
/// - **Durable invariant**, asserted relationally below the table: no row may
///   ever compile while a worker was recognized and nothing consumed it. That
///   is the conservation total; it survives every extension that preserves the
///   contract, and it is the assertion that would have caught `739cfde3`.
/// - **Transition sentinel** on the literal rows: depth 2 and depth 3 take the
///   separately validated required-consumer projection, install two and three
///   exact worker binders respectively, transfer their synthesized environment,
///   and return to `StaticWorkerBinding`. Depth 1 and row 5 also refuse there;
///   row 1 is excluded and remains at `BackendFailure::PlannerInvariant` for
///   missing affine checked-root authority. These are measured boundaries, not
///   closure claims, and the table is rewritten when any later route advances
///   them.
#[test]
fn d2k_1b_i_every_recognized_static_worker_reaches_a_disposition() {
    use crate::cranelift_backend::lowering::{d2k_owner_trace_take, D2kOwnerEvent};
    /// `(recognized owners, bare-Var reads, installs, consuming calls, outcome)`.
    fn disposition(
        expression: &RuntimeExpr,
        symbol: &str,
    ) -> (Vec<String>, usize, usize, usize, String) {
        let _ = d2k_owner_trace_take();
        let (result, _trace) = px8j_capture_source_trace(expression, false, symbol);
        let events = d2k_owner_trace_take();
        let mut recognized: Vec<String> = events
            .iter()
            .filter_map(|event| match event {
                D2kOwnerEvent::StaticWorkerField { constructor, .. } => Some(constructor.clone()),
                _ => None,
            })
            .collect();
        recognized.dedup();
        let count = |predicate: fn(&&D2kOwnerEvent) -> bool| events.iter().filter(predicate).count();
        let outcome = match &result {
            Ok(_) => "compiled".to_string(),
            Err(CraneliftBackendError::Unsupported(UnsupportedLowering { construct, .. })) => {
                format!("refused:{construct}")
            }
            Err(other) => format!("refused-other:{other}"),
        };
        (
            recognized,
            count(|event| matches!(event, D2kOwnerEvent::StaticWorkerRead { .. })),
            count(|event| matches!(event, D2kOwnerEvent::StaticWorkerBinderInstalled { .. })),
            count(|event| matches!(event, D2kOwnerEvent::StaticWorkerCallConsumed { .. })),
            outcome,
        )
    }

    let rows = [
        (
            "row1-owned-scope",
            disposition(
                &host_result_closure_match(px8j_layered_recursive_result(1, 1)),
                "ken_d2k1bi_row1",
            ),
        ),
        (
            "row4-depth-1",
            disposition(
                &host_result_closure_match(px8j_scope_chain_observation_result(1, 0)),
                "ken_d2k1bi_row4_d1",
            ),
        ),
        (
            "row4-depth-2",
            disposition(
                &host_result_closure_match(px8j_scope_chain_observation_result(2, 0)),
                "ken_d2k1bi_row4_d2",
            ),
        ),
        (
            "row4-depth-3",
            disposition(
                &host_result_closure_match(px8j_scope_chain_observation_result(3, 0)),
                "ken_d2k1bi_row4_d3",
            ),
        ),
        (
            "row5-after-hole",
            disposition(
                &host_result_closure_match(px8j_equal_payload_hole_placement(
                    Px8jSelectedScopePlacement::AfterReturnHole,
                )),
                "ken_d2k1bi_row5",
            ),
        ),
    ];

    // THE DURABLE HALF. Asserted over the population first and as a relation,
    // because it is the one clause that must hold for every row under every
    // future route: a recognized worker never survives into a compiled object
    // without something having consumed it.
    for (label, (recognized, _, installs, consuming_calls, outcome)) in &rows {
        assert!(
            !(outcome == "compiled" && !recognized.is_empty() && *consuming_calls == 0),
            "{label} compiled with {} recognized static worker field(s), {installs} rebound and \
             {consuming_calls} consumed. A row that compiles is not a row that passes: a worker \
             built and then neither consumed nor authoritatively erased is the forbidden fourth \
             state, and it is what the producer-alone cut shipped.",
            recognized.len()
        );
    }

    // THE MEASURED HALF, each component against a LITERAL and never against the
    // population -- the discipline `D2k-1a` was rebuilt under, because a
    // sameness check across the five is green under a uniform move.
    let tree1 = vec!["ctor:fixture::PX8JTree1::Node".to_string()];
    let scope = vec!["ctor:fixture::PX8JScopeTree::Node".to_string()];
    let hole = vec!["ctor:fixture::PX8JHoleOutput::Node".to_string()];
    let refused_worker = "refused:StaticWorkerBinding".to_string();
    let refused_root_authority = "refused-other:Cranelift backend failure: native static \
        transition planner invariant failed; please report this compiler bug: terminal answer \
        has no affine checked-root authority"
        .to_string();
    assert_eq!(
        rows,
        [
            // Row 1 refuses EARLIER than the conservation close, at a wall that
            // is not this increment's -- so its recognized field never reaches
            // a disposition here. That is the A/B's informative side: without a
            // row that moves for a different reason, "they all refuse" is
            // equally consistent with the arming having done nothing.
            (
                "row1-owned-scope",
                (tree1, 0, 0, 0, refused_root_authority)
            ),
            (
                "row4-depth-1",
                (scope.clone(), 0, 0, 0, refused_worker.clone())
            ),
            (
                "row4-depth-2",
                (scope.clone(), 0, 2, 0, refused_worker.clone())
            ),
            (
                "row4-depth-3",
                (scope, 0, 3, 0, refused_worker.clone())
            ),
            ("row5-after-hole", (hole, 0, 0, 0, refused_worker)),
        ],
        "RT-REQUIRED-OCCURRENCE-PROJECTION D4: every row remains attributed separately. \
         Depth 2 and depth 3 take the one depth-2+ consumer, rebind at each traversed level, \
         and return to the downstream StaticWorkerBinding refusal after the environment transfer. \
         Depth 1 and row 5 remain at StaticWorkerBinding, while excluded row 1 reaches the \
         PlannerInvariant for missing affine checked-root authority. \
         None of these refusals is a closure claim."
    );
}

/// **`D2k-1c-0` — the conservation ledger pairs each consumption to ONE MINTED
/// TRANSPORT, exercised directly.**
///
/// **RE-DERIVED from `D2k-1b-i`'s pairing control, and it went red because the
/// RULED SEMANTICS CHANGED — not because the repair is incomplete.** That
/// control asserted a pairing keyed by `field_origin`, and every row of it was
/// true of what it measured. `D2k-1c-0` then measured that **one planner field
/// origin is RECOGNIZED more than once in a single compile** — `row1`'s
/// `Construct` occurrence is entered twice — which makes an origin key one
/// transport too coarse: at `rebinds = 2`, consuming transport
/// #1 twice while transport #2 was dropped balanced the per-origin counters at
/// `2 == 2` and closed green. Architect `evt_2npnrzesz3t65` replaced the
/// counters with `minted`/`consumed` relations over an opaque transport
/// identity. The rows below are the same *questions* asked of the new
/// representation, plus the two the old one could not express at all.
///
/// **This control exists because the five rows cannot reach it.** They all
/// refuse at the ledger's first closeout branch — recognized, never rebound —
/// so their sentinel witnesses the drop case and nothing else. **A witness that
/// stops at the first refusal cannot prove the claim the later branches make**,
/// so those branches get their own.
///
/// The field origins come from the planner's own positional child table, so the
/// provenance under test is the same `child_static_origin(owner, position)` the
/// producer and the binder use. The transport identities come from the ledger's
/// own issuer, because they cannot come from anywhere else — the type's field is
/// private to its module, so a test cannot forge one, which is the property
/// being relied on rather than described.
///
/// **Promise class: durable invariant.** Every row is a property of
/// conservation itself, not of the current route or the current five: each
/// transport is discharged by exactly one consumption of that same transport,
/// and no aggregate, count or sibling may substitute for it.
#[test]
fn d2k_1c_0_conservation_pairs_each_consumption_to_one_minted_transport() {
    use crate::cranelift_backend::lowering::{FuncId, StaticWorkerFieldLedger};

    let owner_expr = RuntimeExpr::Construct {
        constructor: "ctor:fixture::Pair::Mk".to_string(),
        args: vec![RuntimeExpr::Var(0), RuntimeExpr::Var(0)],
    };
    let (plan, owner) = planned_root_occurrence(&owner_expr);
    let field_a = plan
        .child_static_origin(owner, 0)
        .expect("the constructor plans its first argument as child 0");
    let field_b = plan
        .child_static_origin(owner, 1)
        .expect("the constructor plans its second argument as child 1");
    assert_ne!(
        field_a, field_b,
        "the two fields must be distinct occurrences or every row below is vacuous"
    );
    // The consumer occurrence recorded against a discharge. Any planned origin
    // serves; what matters is that the relation stores WHICH call paid, so a
    // refusal can name both consumers rather than reporting a total.
    let consumer = owner;
    // `from_u32` is a TEST-only way to name a body identity, exactly as the
    // existing constructor controls use it. Production always passes
    // `defining_function_id`, which is `None` outside the emission pass.
    let body_one = Some(FuncId::from_u32(0));
    let body_two = Some(FuncId::from_u32(1));
    let ctor = "ctor:fixture::Pair::Mk";

    // 1. Constructed and never rebound is the DROP -- the state the
    //    producer-alone cut shipped, and the one the five reach today.
    let mut dropped = StaticWorkerFieldLedger::default();
    dropped
        .recognize(owner, 0, field_a, ctor, body_one)
        .expect("the issuer mints");
    assert!(
        dropped.close().is_err(),
        "a constructed field that no static elimination rebinds must refuse"
    );

    // 2. THE ARCHITECT'S PAIRED DISCRIMINATOR, RED HALF. Two constructions
    //    of ONE planner origin -- the shape `row1` measures -- with only one
    //    rebind and one consumption. The second constructed worker has no
    //    transport and no consumption, and it must not be covered by the
    //    first's. This is the row a `field_origin`-keyed recognition map could
    //    not express at all: `or_insert` made the two constructions one record,
    //    so this compile closed GREEN with a worker forgotten.
    let mut half_paid = StaticWorkerFieldLedger::default();
    let built_once = half_paid
        .recognize(owner, 0, field_a, ctor, body_one)
        .expect("mints");
    let built_again = half_paid
        .recognize(owner, 0, field_a, ctor, body_one)
        .expect("mints");
    assert_ne!(
        built_once, built_again,
        "two constructions of one planner origin must be two recognitions, or this row and the \
         one below are the same test"
    );
    let transport = half_paid.rebind(built_once, body_one).expect("recognized");
    half_paid
        .note_consuming_call(Some(transport), consumer, body_one)
        .expect("the transported worker is consumed");
    assert!(
        half_paid.close().is_err(),
        "one construction rebound and consumed cannot discharge a SECOND construction of the \
         same planner origin; the second is constructed-then-forgotten, which is the forbidden \
         fourth state one link before the transport identity can see it"
    );

    // 3. THE PAIRED DISCRIMINATOR, GREEN HALF, and it is the positive
    //    control for row 2: the same two constructions, each transitioned and
    //    each consumed, is conserved. Without this, row 2 passes for a ledger
    //    that refuses everything.
    let mut both_paid = StaticWorkerFieldLedger::default();
    let one = both_paid
        .recognize(owner, 0, field_a, ctor, body_one)
        .expect("mints");
    let two = both_paid
        .recognize(owner, 0, field_a, ctor, body_one)
        .expect("mints");
    let transport_one = both_paid.rebind(one, body_one).expect("recognized");
    let transport_two = both_paid.rebind(two, body_one).expect("recognized");
    assert_ne!(
        transport_one, transport_two,
        "two transitions must mint distinct transports, or every row below collapses into a count"
    );
    both_paid
        .note_consuming_call(Some(transport_one), consumer, body_one)
        .expect("lawful");
    both_paid
        .note_consuming_call(Some(transport_two), consumer, body_one)
        .expect("lawful");
    assert!(
        both_paid.close().is_ok(),
        "two constructions of one occurrence, each transitioned once and each consumed once, is \
         conserved"
    );

    // 4. ONE CONSTRUCTION CANNOT TRANSITION TWICE. A constructed field enters
    //    binding authority once; a second rebind of it would mint a transport
    //    with nothing behind it.
    let mut twice = StaticWorkerFieldLedger::default();
    let built = twice
        .recognize(owner, 0, field_a, ctor, body_one)
        .expect("mints");
    let _first = twice.rebind(built, body_one).expect("recognized");
    assert!(
        twice.rebind(built, body_one).is_err(),
        "one constructed field cannot enter binding authority as two transports"
    );

    // 5. Consuming one transport TWICE refuses at the call, and cannot cover an
    //    outstanding sibling. Under the per-origin balance this compile closed
    //    green at 2 == 2 with one worker dropped.
    let mut doubled = StaticWorkerFieldLedger::default();
    let r1 = doubled
        .recognize(owner, 0, field_a, ctor, body_one)
        .expect("mints");
    let r2 = doubled
        .recognize(owner, 0, field_a, ctor, body_one)
        .expect("mints");
    let t1 = doubled.rebind(r1, body_one).expect("recognized");
    let _t2 = doubled.rebind(r2, body_one).expect("recognized");
    doubled
        .note_consuming_call(Some(t1), consumer, body_one)
        .expect("the first consumption of a minted transport is lawful");
    assert!(
        doubled
            .note_consuming_call(Some(t1), consumer, body_one)
            .is_err(),
        "a second consumption of one transport must refuse AT THE CALL; it cannot be absorbed \
         into a total and it cannot discharge the sibling transport"
    );
    assert!(
        doubled.close().is_err(),
        "the sibling transport is still outstanding, so the close must refuse even though two \
         consuming calls were attempted against two transports"
    );

    // 6. A DIRECT binding carries no transport and discharges nothing. This is
    //    how an unrelated pre-existing worker call could have satisfied a tally.
    let mut unrelated = StaticWorkerFieldLedger::default();
    let r = unrelated
        .recognize(owner, 0, field_a, ctor, body_one)
        .expect("mints");
    let _t = unrelated.rebind(r, body_one).expect("recognized");
    unrelated
        .note_consuming_call(None, consumer, body_one)
        .expect("a direct binding's consumption is lawful and pays no debt");
    assert!(
        unrelated.close().is_err(),
        "a pre-existing static worker call must not discharge a transported field's obligation"
    );

    // 7. CROSS-SCOPE, on BOTH links. A field constructed in one generated body
    //    may not be transitioned from another, and a transport minted in one
    //    may not be consumed from another -- repeated lowering in different
    //    bodies is repeated construction, not a licence to reuse an identity.
    let mut crossed = StaticWorkerFieldLedger::default();
    let r = crossed
        .recognize(owner, 0, field_a, ctor, body_one)
        .expect("mints");
    assert!(
        crossed.rebind(r, body_two).is_err(),
        "a field constructed in one generated body must not be transitioned from another"
    );
    let t = crossed.rebind(r, body_one).expect("same body is lawful");
    assert!(
        crossed
            .note_consuming_call(Some(t), consumer, body_two)
            .is_err(),
        "a transport minted in one generated body must not be dischargeable from another"
    );
    assert!(
        crossed.close().is_err(),
        "and the refused cross-scope consumption leaves the transport outstanding"
    );

    // 8. Both unknown directions fail closed, so a second builder of the worker
    //    arm cannot enter or leave the ledger unaccounted. Each needs an
    //    identity this ledger never issued, and the only way to hold one is to
    //    mint it in another ledger -- which is exactly the compile-to-compile
    //    confusion being refused. Neither type can be forged: both have private
    //    fields in a module that exposes only its issuers.
    let (foreign_recognition, foreign_transport) = {
        let mut other = StaticWorkerFieldLedger::default();
        let r = other
            .recognize(owner, 1, field_b, ctor, body_one)
            .expect("mints");
        let t = other.rebind(r, body_one).expect("recognized there");
        (r, t)
    };
    let mut stranger = StaticWorkerFieldLedger::default();
    assert!(
        stranger.rebind(foreign_recognition, body_one).is_err(),
        "rebinding a field this compilation never constructed must refuse"
    );
    assert!(
        stranger
            .note_consuming_call(Some(foreign_transport), consumer, body_one)
            .is_err(),
        "consuming a transport this compilation never minted must refuse"
    );
}

/// **`D2k-1c-1` — THE JOIN OF THE THREE-LINK CHAIN IS A LAW, and each half of
/// it has a row that fails without it.**
///
/// `close()` asserted `dom(transitioned) = dom(recognized)` and
/// `dom(consumed) = dom(minted)`, **each keyed on its own map's keys**, and
/// supplied the link between them as prose: *"`minted` in bijection with
/// `transitioned` because one transition mints exactly one transport."* That is
/// a claim about `rebind`'s body, not something the close could fail on.
/// Adversary `evt_733esjz2t4bn8`, confirmed.
///
/// **Why these rows cannot be reached through the ordinary methods, and why
/// that is the point rather than a weakness of the control.** `rebind` inserts
/// into `minted` and `transitioned` back to back with no branch between, so
/// today its adjacency is what makes the join hold — and the finding is exactly
/// that the close cannot fail when the adjacency does not hold. A row built by
/// calling `rebind` would therefore prove the adjacency, not the law. **These
/// rows build the admitted states directly**, which the test module can do
/// because the ledger's fields are private to `lowering` and this module
/// descends from it, while **the identities still cannot be forged** — every
/// recognition and transport below comes from the ledger's own issuer, whose
/// field is private to a module this one does not descend from.
///
/// **Disclosure, so no reader takes these for reachable defects: the ledger
/// states rows 1-3 construct have NO current writer.** Nothing in production
/// produces them — `rebind` inserts into `minted` and `transitioned` back to
/// back, and `note_consuming_call` refuses an identity it never minted — so
/// these are not live states the compiler can enter today. The laws exist for
/// the same reason `close()`'s own `⊆` re-check does: to catch a **future
/// second writer** of these fields that does not go through today's call sites.
/// Without this sentence a reader arriving at directly-built impossible ledgers
/// can reasonably read them as defects already present.
///
/// **Row 2 is why the finding's own one-line repair is not the fix.**
/// `range(transitioned) ⊆ dom(minted)` is satisfied by two recognitions naming
/// ONE minted transport, whose single consumption then discharges both — the
/// constructed-then-forgotten state with the containment green. The law is the
/// agreeing bijection, and injectivity is a consequence of it.
///
/// **Promise class: durable invariant.** Every row is a property of the chain
/// itself — a transition and a transport must name each other — and none of them
/// mentions the current route, the current five rows, or any count.
#[test]
fn d2k_1c_1_a_transition_and_its_transport_must_name_each_other() {
    use crate::cranelift_backend::lowering::{
        FuncId, MintedStaticWorkerTransport, StaticWorkerFieldLedger,
    };

    let owner_expr = RuntimeExpr::Construct {
        constructor: "ctor:fixture::Pair::Mk".to_string(),
        args: vec![RuntimeExpr::Var(0), RuntimeExpr::Var(0)],
    };
    let (plan, owner) = planned_root_occurrence(&owner_expr);
    let field_a = plan
        .child_static_origin(owner, 0)
        .expect("the constructor plans its first argument as child 0");
    let consumer = owner;
    let body = Some(FuncId::from_u32(0));
    let ctor = "ctor:fixture::Pair::Mk";

    // 0. THE POSITIVE CONTROL, first, so no row below can pass for a ledger
    //    that refuses everything. One construction, one transition, one
    //    consumption -- built through the ordinary methods -- closes green, and
    //    the two new laws are silent on it.
    let mut lawful = StaticWorkerFieldLedger::default();
    let recognition = lawful
        .recognize(owner, 0, field_a, ctor, body)
        .expect("the issuer mints");
    let transport = lawful.rebind(recognition, body).expect("recognized");
    lawful
        .note_consuming_call(Some(transport), consumer, body)
        .expect("the transported worker is consumed");
    assert!(
        lawful.close().is_ok(),
        "the ordinary chain must still close, or every refusal below is vacuous"
    );

    // 1. THE ADMITTED STATE THE FINDING NAMES: `transitioned[r] = T` with
    //    `T` absent from `minted`. Before the join law this closed GREEN --
    //    loop one saw `r` transitioned, loop two saw `r` recognized, and the
    //    loops over `minted` and `consumed` never quantified over `T` at all.
    //    The transport is minted by the issuer and deliberately never recorded,
    //    which is the one way to hold a real identity that `minted` does not
    //    know.
    let mut stranded = StaticWorkerFieldLedger::default();
    let recognition = stranded
        .recognize(owner, 0, field_a, ctor, body)
        .expect("the issuer mints");
    let unrecorded = stranded
        .issuer
        .mint()
        .expect("a fresh issuer is not exhausted");
    stranded.transitioned.insert(recognition, unrecorded);
    assert!(
        stranded.close().is_err(),
        "a transition naming a transport that was never minted is a constructed field whose \
         obligation no other law quantifies over; the close must fail on it rather than describe \
         it in prose"
    );

    // 2. THE CONTAINMENT IS NOT ENOUGH. Two constructions transitioning to ONE
    //    minted transport satisfies `range(transitioned) subset dom(minted)`,
    //    and that transport's single lawful consumption then discharges both.
    //    This row is green under the one-line repair the finding proposed and
    //    red under the agreeing bijection, so it is what distinguishes them.
    let mut shared = StaticWorkerFieldLedger::default();
    let built_once = shared
        .recognize(owner, 0, field_a, ctor, body)
        .expect("mints");
    let built_again = shared
        .recognize(owner, 0, field_a, ctor, body)
        .expect("mints");
    assert_ne!(
        built_once, built_again,
        "two constructions of one planner origin must be two recognitions, or this row cannot \
         express the state it is about"
    );
    let shared_transport = shared.rebind(built_once, body).expect("recognized");
    shared.transitioned.insert(built_again, shared_transport);
    shared
        .note_consuming_call(Some(shared_transport), consumer, body)
        .expect("one lawful consumption of the one minted transport");
    assert!(
        shared.close().is_err(),
        "one transport cannot be the transition of two constructed fields; its single consumption \
         would discharge both, which is the forbidden state with every containment satisfied"
    );

    // 3. THE JOIN, BACK. A transport standing behind a recognition that
    //    transitions elsewhere is a construction that entered binding authority
    //    twice. `rebind` refuses the second one at the call; this asserts the
    //    close can fail on the state, which is what the call-side refusal being
    //    the ONLY guard is the objection to.
    let mut doubled = StaticWorkerFieldLedger::default();
    let recognition = doubled
        .recognize(owner, 0, field_a, ctor, body)
        .expect("mints");
    let kept = doubled.rebind(recognition, body).expect("recognized");
    let extra = {
        // Minted for the same recognition, recorded in `minted` alone -- the
        // state `rebind`'s second-transition refusal exists to prevent.
        let spare = doubled.issuer.mint().expect("the issuer is not exhausted");
        doubled.minted.insert(
            spare,
            MintedStaticWorkerTransport {
                recognition,
                field_origin: field_a,
                owner,
                position: 0,
                constructor: ctor.to_string(),
                scope: body,
            },
        );
        spare
    };
    doubled
        .note_consuming_call(Some(kept), consumer, body)
        .expect("lawful");
    doubled
        .note_consuming_call(Some(extra), consumer, body)
        .expect("lawful in isolation, which is why the close must be the one to refuse");
    assert!(
        doubled.close().is_err(),
        "a second transport behind one recognition is an obligation with no construction behind \
         it, and consuming both does not make it one"
    );
}

/// **`RT-LEXICAL-RECURSOR-CONSUMERS` `D2k` `AC-1b` — the unmarked seeds are
/// ABSENCE COMPARATORS and earn no fusion credit.**
///
/// `AC-1` was rebound because its five witnesses cannot carry the selected
/// repair's required input: they are seed-lane compiles with no checked package,
/// so no oriented plan can be decoded for them, and the completed A/B measured
/// that even a supplied empty plan is admitted and still resolves zero keys.
/// This control keeps them in the suite as what they actually are.
///
/// **The two halves are asserted TOGETHER, on purpose.** Each row states
///
/// - **the refusal it OBSERVES**, and
/// - **the fusion plane it LACKS** — no oriented plan present, no resolved key,
///   an empty resolved plane.
///
/// Separating them is how a zero becomes evidence for something it does not
/// support. A seed that resolves nothing is indistinguishable from a compile
/// that never ran, so the plane is only meaningful **beside the refusal that
/// proves the compile reached a wall**. That pairing is this control's whole
/// content; neither half discharges any fusion property, and the assertion
/// message says so rather than leaving a reader to infer it.
///
/// **The wall half is REUSED from [`d2k_wall_under_current_selector`], not
/// re-derived**, so this row cannot drift from `D2k-0`'s. What is asserted is
/// the refusal's **construct**, per row against its own literal — deliberately
/// not its reason, which names constructor origins that renumber under
/// unrelated planner changes, and not its edge, which `D2k-1b` retired by
/// recognizing the binding ahead of the value read.
///
/// **Row 1 sits at a different wall and that is kept, not smoothed.** It reaches
/// the planner invariant for missing affine checked-root authority before this
/// increment's conservation close. A uniform expectation across the five would
/// go green under a uniform move, which is exactly the case worth catching.
///
/// **This test claims NOTHING about fusion working.** The positive that gives
/// a resolved plane its meaning is the checked twin, on a different fixture
/// family, and it is deliberately not cited here as though it covered these
/// rows — that citation is the defect `AC-1`'s rebind exists to prevent.
///
/// **Promise class: durable invariant.** It asserts a relation — refusal
/// observed together with plane absent — over the whole seed population. It reds
/// if a seed starts resolving a plane, which would mean a forbidden route made
/// these fixtures green, and it reds if a seed stops refusing, which is
/// `D2k-1b`'s repair and is the event that retires it.
#[test]
fn d2k_1b_unmarked_seeds_refuse_and_resolve_no_fusion_plane() {
    use crate::cranelift_backend::lowering::core::{
        d2f_gate_arrivals_take, d2f_production_fusion_planes_take,
    };
    /// `(refusing construct, any oriented plan present, resolved keys, resolved
    /// planes)` for one seed row, under the same wrong-consumer exclusion every
    /// other `D2k` control runs.
    fn seed_row(
        expression: &RuntimeExpr,
        symbol: &str,
    ) -> (Option<String>, bool, usize, Vec<usize>) {
        let _ = d2f_gate_arrivals_take();
        let _ = d2f_production_fusion_planes_take();
        let refusal = d2k_wall_under_current_selector(expression, symbol);
        let planes = d2f_production_fusion_planes_take();
        // Drained ONCE. Taking this twice would leave the second read empty
        // and make the key count zero for the wrong reason.
        let arrivals = d2f_gate_arrivals_take();
        (
            refusal.map(|(construct, _edge)| construct),
            arrivals.iter().any(|arrival| arrival.oriented_present),
            arrivals.iter().map(|arrival| arrival.keys.len()).sum(),
            planes,
        )
    }

    let row1 = host_result_closure_match(px8j_layered_recursive_result(1, 1));
    let row4_d1 = host_result_closure_match(px8j_scope_chain_observation_result(1, 0));
    let row4_d2 = host_result_closure_match(px8j_scope_chain_observation_result(2, 0));
    let row4_d3 = host_result_closure_match(px8j_scope_chain_observation_result(3, 0));
    let row5 = host_result_closure_match(px8j_equal_payload_hole_placement(
        Px8jSelectedScopePlacement::AfterReturnHole,
    ));
    // The absence, written once: no oriented plan reached the builder, no key
    // resolved, and the one plane the builder built was empty.
    let absent = (false, 0usize, vec![0usize]);
    let conservation = Some("StaticWorkerBinding".to_string());

    assert_eq!(
        [
            ("row1-owned-scope", seed_row(&row1, "ken_d2k1b_row1")),
            ("row4-depth-1", seed_row(&row4_d1, "ken_d2k1b_row4_d1")),
            ("row4-depth-2", seed_row(&row4_d2, "ken_d2k1b_row4_d2")),
            ("row4-depth-3", seed_row(&row4_d3, "ken_d2k1b_row4_d3")),
            ("row5-after-hole", seed_row(&row5, "ken_d2k1b_row5")),
        ]
        .map(|(label, (construct, present, keys, planes))| {
            (label, construct, (present, keys, planes))
        }),
        [
            (
                "row1-owned-scope",
                Some("<not-unsupported>".to_string()),
                absent.clone(),
            ),
            ("row4-depth-1", conservation.clone(), absent.clone()),
            ("row4-depth-2", conservation.clone(), absent.clone()),
            ("row4-depth-3", conservation.clone(), absent.clone()),
            ("row5-after-hole", conservation.clone(), absent.clone()),
        ],
        "AC-1b: each unmarked seed must REACH a refusal and, on that same compile, carry NO \
         oriented plan, resolve NO fusion key, and leave the built plane empty. The absence is \
         recorded BESIDE the refusal that proves the compile ran, because a zero plane on its \
         own is indistinguishable from a compile that never happened. Nothing here discharges \
         any fusion property: these rows are absence comparators and the checked twin on a \
         different fixture family is the only positive. A row that starts resolving a plane \
         means a forbidden route made a seed green by supplying a plan, adding markers or \
         weakening the key; a row that stops refusing is D2k-1b's repair and retires this \
         control."
    );
}

/// **`RT-LEXICAL-R3-FUSION-EMITTER` — control 1 of the atomic `DP`+`D1`+`D2`+`D3`
/// object. The BASE uncomposed population law, and the regression net for
/// `89ee005b`.**
///
/// Architect `evt_2f0nnwtzqy65m`, which **withdrew** the earlier authorized
/// population. The base checked plan must keep the singleton sequences it has
/// today: the outer slot and its call name **only** the outer frame, and the
/// inner slot names **only** the inner frame. The producer's membership is
/// transported at the **composition splice that creates it**, never promised in
/// advance by a base call template that describes a layer no uncomposed segment
/// contains.
///
/// **This test exists because the opposite was built and measured.** `89ee005b`
/// populated the outer slot with the rooted `ParentFrame` closure — statically
/// true, and it made `ReHomed` refuse with
/// *"does not carry its exact checked frame sequence: expected={0, 1}
/// instantiated={0}"*. `ReHomed` reaches `instantiate_checked_invocation_segment`
/// **unarmed**, with a one-layer segment, so a base template that promises the
/// inner frame is promising a layer that is not there. That commit is preserved
/// as negative evidence and is not a candidate.
///
/// ⇒ **`ParentFrame` proves static nesting only, never dynamic segment
/// membership.** The two are different relations and this control is what keeps
/// them apart, because the static one is *true* and therefore tempting.
///
/// **Both halves are asserted together and that is deliberate.** The population
/// law alone would pass against a plan nobody consumes; the refusal alone would
/// pass for any reason, including a compile that failed earlier. Together they
/// say: the base sequences are singletons **and** the three roots still reach
/// exactly the ordinary refusal that fact produces.
///
/// **Nothing here is keyed on a fixture constant.** The roles are derived from
/// the plan's own control witnesses — root is `DistinguishedRoot`, producer is
/// its `ParentFrame` child — so renumbering the fixture cannot make this pass by
/// coincidence, and the test states the *law* rather than the fixture's current
/// integers.
///
/// **Promise class: durable invariant.** The base uncomposed population is a
/// singleton per slot for as long as membership is composition-time; the
/// literals are `1` (a singleton's length) and the refusal sentence each root
/// already produces. It does **not** go red when the atomic object arms — the
/// composed path adds membership at the splice and leaves these base templates
/// alone. A red here means base population crept back, which is the thing
/// `89ee005b` demonstrated is unlawful.
#[test]
fn r3_the_base_uncomposed_slot_population_stays_singleton_and_rehomed_expects_no_inner_frame() {
    use crate::cranelift_backend::planning::{d2j_checked_fixture_under, D2jCause, D2J_DECLARATION};

    /// One cause through the production entry, unarmed and uncomposed.
    fn compile_cause(cause: D2jCause, symbol: &str) -> Option<CraneliftBackendError> {
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let mut declarations = std::collections::BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        crate::cranelift_backend::lowering::core::compile_expr_into_object_module(
            crate::cranelift_backend::artifact::new_object_module_for_lowering_tests("ken-r3-base")
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
    for (cause, symbol) in [
        (D2jCause::Exact, "ken_r3_base_exact"),
        (D2jCause::ReHomed, "ken_r3_base_rehomed"),
        (D2jCause::ProducerArity, "ken_r3_base_arity"),
    ] {
        let (_, _, oriented) = d2j_checked_fixture_under(cause);

        // Roles by control witness, never by fixture constant.
        let root = oriented
            .frames
            .iter()
            .find(|frame| frame.control_witness == crate::OrientedControlWitnessV1::DistinguishedRoot)
            .expect("the checked twin has a distinguished root frame");
        let producer = oriented
            .frames
            .iter()
            .find(|frame| {
                frame.control_witness
                    == crate::OrientedControlWitnessV1::ParentFrame(root.frame_id)
            })
            .expect("the checked twin has a producer frame parented to the root");

        let outer_slot = oriented
            .computational_ih_slots
            .iter()
            .find(|slot| slot.frame_template_id == root.frame_id)
            .expect("the root frame has its own binder slot");
        let inner_slot = oriented
            .computational_ih_slots
            .iter()
            .find(|slot| slot.frame_template_id == producer.frame_id)
            .expect("the producer frame has its own binder slot");
        let call = match oriented.computational_ih_calls.as_slice() {
            [only] => only.clone(),
            other => panic!("the twin carries exactly one checked IH call: {}", other.len()),
        };

        rows.push((
            cause,
            outer_slot.frame_templates.clone(),
            inner_slot.frame_templates.clone(),
            call.callee_frame_templates.clone(),
            compile_cause(cause, symbol),
        ));
    }

    let observed: Vec<(D2jCause, Vec<u64>, Vec<u64>, Vec<u64>, bool)> = rows
        .iter()
        .map(|(cause, outer, inner, callee, error)| {
            let ordinary = matches!(
                error,
                Some(CraneliftBackendError::Unsupported(UnsupportedLowering { construct, reason }))
                    if *construct == "ComputationalMatch"
                        && reason.contains("in-flight activation")
            );
            (*cause, outer.clone(), inner.clone(), callee.clone(), ordinary)
        })
        .collect();

    let (root_id, producer_id) = {
        let (_, _, oriented) = d2j_checked_fixture_under(D2jCause::Exact);
        let root = oriented
            .frames
            .iter()
            .find(|frame| frame.control_witness == crate::OrientedControlWitnessV1::DistinguishedRoot)
            .expect("root frame")
            .frame_id;
        let producer = oriented
            .frames
            .iter()
            .find(|frame| {
                frame.control_witness == crate::OrientedControlWitnessV1::ParentFrame(root)
            })
            .expect("producer frame")
            .frame_id;
        (root, producer)
    };

    let expected: Vec<(D2jCause, Vec<u64>, Vec<u64>, Vec<u64>, bool)> = [
        D2jCause::Exact,
        D2jCause::ReHomed,
        D2jCause::ProducerArity,
    ]
    .into_iter()
    .map(|cause| {
        (
            cause,
            vec![root_id],
            vec![producer_id],
            vec![root_id],
            true,
        )
    })
    .collect();

    assert_eq!(
        observed, expected,
        "the BASE uncomposed plan must keep singleton sequences -- outer slot and its call naming \
         only the root frame, inner slot naming only the producer frame -- and all three roots must \
         still reach the ordinary in-flight-activation refusal. A callee sequence containing the \
         producer frame here promises a layer no uncomposed segment carries: it is what made \
         ReHomed refuse with \"does not carry its exact checked frame sequence\" at 89ee005b, which \
         is preserved as negative evidence. Membership is transported at the composition splice \
         that creates it; ParentFrame proves static nesting only, never dynamic segment membership. \
         Rows are (cause, outer slot, inner slot, call callee, reached the ordinary refusal). \
         errors={:?}",
        rows.iter().map(|(cause, _, _, _, error)| (cause, error)).collect::<Vec<_>>()
    );
}

/// **`RT-LEXICAL-R3-FUSION-EMITTER` `DP` — the composition-time membership
/// population law: what the checked source may claim, and what a claim is worth
/// to the binding fingerprint.**
///
/// `composed_frame_templates` is new planner population, so the question that
/// decides whether it is safe is not "does the lawful case work" but **"can it
/// be made to say nothing, and does anything notice?"** Every row here is a way
/// the sequence could have been vacuous, and every one is a refusal rather than
/// a tolerated shape.
///
/// | row | claim | verdict |
/// |---|---|---|
/// | lawful | a frame of this segment, not already named | accepted |
/// | stale | a frame the plan does not contain | refused |
/// | repeat | the same frame twice in one sequence | refused |
/// | overlap | a frame the ordinary sequence already names | refused |
///
/// **Why `overlap` is a refusal and not a harmless duplicate.** The ordinary and
/// composed sequences are concatenated to build a composed segment's expectation.
/// A frame in both would be expected twice, and the instantiator would meet it as
/// *"one invocation instantiates a checked frame template more than once"* — a
/// Runtime error blaming the segment for a defect that is in the plan. Refusing
/// at `validate` puts the diagnosis where the mistake is.
///
/// **The fingerprint row is the one that is easy to omit and expensive to lose.**
/// The two sequences are encoded back to back, so `([outer], [inner])` and
/// `([outer, inner], [])` are the same frame ids in the same order — and until
/// the composed sequence carries a length prefix they hash **identically**. The
/// binding fingerprint is what makes a template's identity depend on what it
/// claims, so two templates whose composition-time claims differ must not agree
/// on it. Asserting `!=` here is asserting that the separator is load-bearing.
///
/// **What this does NOT cover, stated rather than implied.** `validate` also
/// refuses a composed frame from another checked segment. This fixture has one
/// segment site, so that row has no witness here and is **not** exercised — the
/// closure exists in the validator and this test is not evidence for it.
///
/// **Promise class: durable invariant.** Acceptance and refusal of authored
/// claims, plus the inequality of two fingerprints that describe different
/// claims. It carries no literal frame ids of its own: every id is read off the
/// fixture's own plan, and the one literal is the count `2`, which is what makes
/// a repeat a repeat.
#[test]
fn dp_composition_time_membership_is_validated_and_changes_the_binding_fingerprint() {
    // The lawful claim: `outer` keeps the ordinary sequence and `inner` — a real
    // frame of the same segment that the ordinary sequence does not name — is
    // claimed as a composition-time member.
    let base = oriented_test_ih_plan();
    let ordinary = base
        .computational_ih_calls
        .first()
        .expect("the IH fixture carries a call template")
        .clone();
    let outer = *ordinary
        .callee_frame_templates
        .first()
        .expect("the ordinary sequence is non-empty");
    let inner = base
        .frames
        .iter()
        .map(|frame| frame.frame_id)
        .find(|frame_id| !ordinary.callee_frame_templates.contains(frame_id))
        .expect("the fixture has a frame the ordinary sequence does not name");
    let stale = base
        .frames
        .iter()
        .map(|frame| frame.frame_id)
        .max()
        .expect("the fixture has frames")
        + 1;

    /// Re-seal the first call template's composition-time claim and revalidate.
    ///
    /// The fingerprint is recomputed, never left stale: a plan that fails only
    /// because its fingerprint no longer matches would refuse for a reason that
    /// has nothing to do with the claim under test, and every row below would
    /// pass for the wrong reason.
    fn under(
        base: &crate::OrientedSubcontinuationPlanV1,
        composed: Vec<u64>,
    ) -> (crate::OrientedSubcontinuationPlanV1, Result<(), &'static str>) {
        let mut plan = base.clone();
        let call = plan
            .computational_ih_calls
            .first_mut()
            .expect("the IH fixture carries a call template");
        call.composed_frame_templates = composed;
        call.occurrence_binding_fingerprint =
            crate::compiler_private_computational_ih_call_binding_fingerprint(call);
        let verdict = plan.validate();
        (plan, verdict)
    }

    let (lawful, lawful_verdict) = under(&base, vec![inner]);
    let (_, stale_verdict) = under(&base, vec![stale]);
    let (_, repeat_verdict) = under(&base, vec![inner, inner]);
    let (_, overlap_verdict) = under(&base, vec![outer]);

    // The separator row. Same ids, same order, different claim: one says
    // "`inner` joins only when composed", the other says "`inner` is always a
    // member". Their binding fingerprints must disagree.
    let composed_claim = lawful
        .computational_ih_calls
        .first()
        .expect("the lawful plan kept its call template")
        .occurrence_binding_fingerprint;
    let folded_claim = {
        let mut call = ordinary.clone();
        call.callee_frame_templates = vec![outer, inner];
        call.composed_frame_templates = Vec::new();
        crate::compiler_private_computational_ih_call_binding_fingerprint(&call)
    };

    // The claim survives transport. A population the checked source authors and
    // the decoder drops is a population Runtime never sees.
    let decoded = crate::OrientedSubcontinuationPlanV1::decode(&lawful.canonical_bytes())
        .expect("the lawful plan round-trips");
    let decoded_claim = decoded
        .computational_ih_calls
        .first()
        .expect("the decoded plan kept its call template")
        .composed_frame_templates
        .clone();

    assert_eq!(
        (
            lawful_verdict,
            stale_verdict,
            repeat_verdict,
            overlap_verdict,
            composed_claim == folded_claim,
            decoded_claim.as_slice(),
        ),
        (
            Ok(()),
            Err("computational IH call names a stale composed frame"),
            Err("computational IH call repeats a composed frame"),
            Err("computational IH call composes a frame it already names"),
            false,
            [inner].as_slice(),
        ),
        "DP: a composition-time claim naming a real, not-already-named frame of this segment is \
         accepted; a stale frame, a repeat, and a frame the ordinary sequence already names are \
         each refused by their own sentence; the claim changes the binding fingerprint, so \
         'inner joins when composed' and 'inner is always a member' are not the same template; \
         and the claim survives encode/decode. A row that stops refusing means the population \
         can be made vacuous; a fingerprint that starts agreeing means the two sequences were \
         encoded without a separator and a composition-time claim became invisible to the \
         identity that is supposed to bind it."
    );
}


/// **`RT-LEXICAL-R3-FUSION-EMITTER` `AC-D3-SELF` — the recursive self edge's
/// call site is not its callee body, and on this witness that is the ONLY way
/// to see it.**
///
/// The Steward measured the inputs and wrote them into the frame: on the `R3`
/// twin the claim's **`seat`, its `producer_body`, and its redirect's callee
/// entry all print `37`**, while the consuming call is **`17`**. The coincidence
/// is three-way.
///
/// ⇒ **A control whose expected values are every `37` passes under the fold it
/// is supposed to catch.** Folding call site into body type-checks — the two are
/// the same type — and three of the four identities agree, so any assertion
/// drawn from those three is satisfied by the wrong code. Separating `17` from
/// `37` is the whole content of this control.
///
/// **It exercises the production decision, not a copy.** The emission seam and
/// this test both call `fusion_self_edge_identities`, which exists for exactly
/// that reason: an assertion that re-spelled the choice locally would agree with
/// itself no matter what the seam did.
///
/// **The three-way coincidence is asserted too**, not as a property worth
/// preserving but as the *precondition of this test's own value*. If a future
/// fixture separates those three, the fold stops being invisible and a reader
/// should know this control was built for a harder case than the one they have.
///
/// **Promise class: durable invariant.** The self edge keys on the callee body
/// and records the consuming call as its call site, and those are different
/// occurrences. The literals are the claim's own measured origins, read from the
/// claim rather than written in.
#[test]
fn ac_d3_self_the_recursive_edges_call_site_is_separated_from_its_callee_body() {
    use crate::cranelift_backend::planning::{d2j_checked_fixture_under, D2jCause, D2J_DECLARATION};

    let (entry, declaration, oriented) = d2j_checked_fixture_under(D2jCause::Exact);
    let mut declarations = std::collections::BTreeMap::new();
    declarations.insert(D2J_DECLARATION, &declaration);
    let mut planner =
        crate::cranelift_backend::planning::plan_static_transition_graph(&entry, &declarations)
            .expect("the checked twin plans");
    let plane = crate::cranelift_backend::planning::build_static_continuation_fusion_plan(
        &planner,
        &entry,
        &declarations,
        Some(&oriented),
    )
    .expect("the checked twin resolves a plane");
    planner
        .install_static_continuation_fusions(plane)
        .expect("the resolved plane installs");
    let ledger =
        crate::cranelift_backend::planning::FusionRegionClaimLedger::preflight(&planner)
            .expect("the installed region preflights a claim");
    let fusion = *ledger
        .planned()
        .iter()
        .next()
        .expect("the twin installs exactly one region");
    let claim = ledger.claim(fusion).expect("the region's claim is outstanding");

    // The production decision, called rather than restated.
    let (edge_body, edge_call_site) =
        crate::cranelift_backend::lowering::units::fusion_self_edge_identities(
            claim.producer_body(),
            claim.consuming_call(),
        );

    assert_eq!(
        (
            // The edge keys on the CALLEE BODY and records the CONSUMING CALL.
            edge_body == claim.producer_body(),
            edge_call_site == claim.consuming_call(),
            // ...and those are different occurrences. This is the row the fold
            // fails.
            edge_body != edge_call_site,
            // The precondition that makes the row above the only witness: three
            // of the four identities agree, so nothing drawn from them can see
            // the fold.
            claim.seat() == claim.producer_body(),
            claim.redirect().callee_origin() == claim.producer_body(),
        ),
        (true, true, true, true, true),
        "AC-D3-SELF: the definition-local recursive edge must key on the claim's callee body and \
         record the claim's consuming call as its call site, and those must be different \
         occurrences. On this witness the seat, the producer body and the redirect's callee entry \
         all agree, so an assertion drawn from any of them is satisfied by a build that folded \
         call site into body; the body-versus-call-site inequality is the only row that is not. \
         body={edge_body:?} call_site={edge_call_site:?} seat={:?} redirect_callee={:?}",
        claim.seat(),
        claim.redirect().callee_origin()
    );
}

/// **`RT-LEXICAL-R3-FUSION-EMITTER` `D3` — the fusion-local composition
/// ledger's affine refusals, each reached by its own perturbation.**
///
/// **MEASURED:** on the `Exact` and `ReHomed` planner witnesses, with the
/// fusion plane installed, the ledger plans exactly the two composed edges;
/// consuming an identity it never planned, one under an owner that is not the
/// edge's, one against a target the identity does not name, and one twice, are
/// each refused with that fact's own message; a closeout with a planned member
/// unconsumed is refused; and the whole-population closeout succeeds only when
/// every planned member has been consumed and both omission sets equal `F_t`.
///
/// **CLAIMED:** `dom(planned) = dom(consumed)` is affine and total, and no half
/// of it passes for the other.
///
/// **THE GAP, and it is two named clauses rather than a hedge.** Two of the
/// closeout's clauses are **not reachable by any perturbation of this ledger**,
/// and neither is claimed as measured:
///
/// - the per-fusion `{Outer, Inner}` layer law is IMPLIED by the population
///   equality above it plus the planner's own preflight, which already refuses
///   a body-owning fusion that does not carry exactly one of each. It is kept
///   because that implication routes through a law this ledger does not own,
///   and a redundant refusal naming a silent-wrong-answer path earns its line.
/// - the cross-ledger disjointness is implied by `claim_exact` refusing an
///   identity outside `O` — an `F` member cannot enter the ordinary ledger to
///   be found in both.
///
/// **Promise class: durable invariant.** Every assertion is over sets and over
/// refusal identity, never over a count of edges: this witness plans two, and a
/// row that said "two" would red the moment a second fusion was installed
/// without saying anything about affinity.
#[test]
fn d3_the_fusion_local_composition_ledger_is_affine_and_total() {
    use crate::cranelift_backend::lowering::units::{declare_unit_bundle, FusionCompositionLedger};
    use crate::cranelift_backend::planning::{
        d2j_checked_fixture_under, d2j_installed_plan_under, D2jCause, D2J_DECLARATION,
    };
    use std::collections::{BTreeMap, BTreeSet};

    for cause in [D2jCause::Exact, D2jCause::ReHomed] {
        let (entry, declaration, oriented) = d2j_checked_fixture_under(cause);
        let mut declarations = BTreeMap::new();
        declarations.insert(D2J_DECLARATION, &declaration);
        // The SAME installed witness the planner-side partition control uses.
        let plan = d2j_installed_plan_under(cause, &entry, &declarations, &oriented)
            .expect("the witness installs");

        let mut module = crate::cranelift_backend::artifact::new_object_module_for_lowering_tests(
            "ken-d3-composition-ledger",
        )
        .expect("object module");
        let bundle = declare_unit_bundle(&mut module, &plan).expect("bundle declares");

        // The planned population and the fusion-local targets, read from the
        // planner rather than from the ledger the assertions are about.
        let planned = plan.fusion_composed_edges().clone();
        let fused_targets = planned
            .values()
            .map(|edge| edge.target())
            .chain(
                plan.fusion_outer_realizations()
                    .values()
                    .map(|realization| realization.target()),
            )
            .collect::<BTreeSet<_>>();
        assert!(
            !planned.is_empty(),
            "{cause:?}: the witness must plan at least one composed edge, or every row below \
             passes vacuously over an empty population"
        );

        let open = || FusionCompositionLedger::open(&plan, &bundle).expect("ledger opens");

        // ---- The declaration pass actually omitted `F_t`, read from the
        // bundle it produced rather than from the plan.
        for target in &fused_targets {
            assert!(
                bundle.continuation(*target).is_none(),
                "{cause:?}: a fusion-local target keeps a declared Function, so its selected body \
                 would be realized twice"
            );
        }

        // ---- Refusal 1: an identity this ledger never planned.
        //
        // The unplanned identity is a REAL planner-issued ordinary identity,
        // not a fabricated one. A fabricated key could be refused merely for
        // being unrecognizable; an ordinary identity is the case the guard
        // actually has to catch, because it is exactly what a fork that
        // misrouted `O` into this ledger would present.
        let ordinary = plan
            .ordinary_continuation_call_identities()
            .expect("ordinary identities");
        if let Some(stranger) = ordinary.iter().next() {
            let mut ledger = open();
            let error = ledger
                .consume(stranger, stranger.emission_owner(), stranger.target())
                .expect_err("an unplanned identity must be refused");
            assert!(
                format!("{error:?}").contains("the planner never composed"),
                "{cause:?}: {error:?}"
            );
        }

        // ---- Refusals 2 and 3: a wrong owner and a wrong target, each named.
        for (identity, edge) in &planned {
            let wrong_owner = planned
                .values()
                .map(|other| other.emission_owner())
                .find(|owner| *owner != edge.emission_owner());
            if let Some(wrong_owner) = wrong_owner {
                let mut ledger = open();
                let error = ledger
                    .consume(identity, wrong_owner, identity.target())
                    .expect_err("a foreign emission owner must be refused");
                assert!(
                    format!("{error:?}").contains("was consumed while defining"),
                    "{cause:?}: {error:?}"
                );
            }
            let wrong_target = fused_targets.iter().find(|target| **target != edge.target());
            if let Some(wrong_target) = wrong_target {
                let mut ledger = open();
                let error = ledger
                    .consume(identity, edge.emission_owner(), *wrong_target)
                    .expect_err("a disagreeing target must be refused");
                assert!(
                    format!("{error:?}").contains("was consumed against"),
                    "{cause:?}: {error:?}"
                );
            }

            // ---- Refusal 4: the replay.
            let mut ledger = open();
            ledger
                .consume(identity, edge.emission_owner(), identity.target())
                .expect("the exact consumption is accepted");
            let error = ledger
                .consume(identity, edge.emission_owner(), identity.target())
                .expect_err("a second consumption of one identity must be refused");
            assert!(
                format!("{error:?}").contains("consumed twice"),
                "{cause:?}: {error:?}"
            );
        }

        // ---- Refusal 5: a planned member left unconsumed. THE case that reads
        // as success because nothing was emitted for it.
        let mut ledger = open();
        for target in &fused_targets {
            ledger.record_definition_omitted(*target);
        }
        for (identity, realization) in plan.fusion_outer_realizations() {
            ledger
                .record_outer_realized(identity, realization.fusion(), realization.target())
                .expect("the R realization is accepted");
        }
        let error = ledger
            .close(&BTreeSet::new())
            .expect_err("an unrealized composition must be refused at close");
        assert!(
            format!("{error:?}").contains("never realized"),
            "{cause:?}: {error:?}"
        );

        // ---- Refusal 6: the definition pass did not omit `F_t`.
        let mut ledger = open();
        for (identity, edge) in &planned {
            ledger
                .consume(identity, edge.emission_owner(), identity.target())
                .expect("consumes");
        }
        for (identity, realization) in plan.fusion_outer_realizations() {
            ledger
                .record_outer_realized(identity, realization.fusion(), realization.target())
                .expect("the R realization is accepted");
        }
        let error = ledger
            .close(&BTreeSet::new())
            .expect_err("a definition pass that omitted nothing must be refused");
        assert!(
            format!("{error:?}").contains("definition pass's omitted"),
            "{cause:?}: {error:?}"
        );

        // ---- The positive: every planned member consumed, both omission sets
        // equal to `F_t`, and nothing shared with the ordinary ledger.
        let mut ledger = open();
        for (identity, edge) in &planned {
            ledger
                .consume(identity, edge.emission_owner(), identity.target())
                .expect("consumes");
        }
        for (identity, realization) in plan.fusion_outer_realizations() {
            ledger
                .record_outer_realized(identity, realization.fusion(), realization.target())
                .expect("the R realization is accepted");
        }
        for target in &fused_targets {
            ledger.record_definition_omitted(*target);
        }
        ledger
            .close(&ordinary)
            .expect("the total, affine, disjoint closeout succeeds");

        // ---- `R` is refused when its owned body never realized it, and when a
        // second body claims the same identity. Neither is a consumption.
        let mut ledger = open();
        for (identity, edge) in &planned {
            ledger
                .consume(identity, edge.emission_owner(), identity.target())
                .expect("consumes");
        }
        for target in &fused_targets {
            ledger.record_definition_omitted(*target);
        }
        let error = ledger
            .close(&ordinary)
            .expect_err("a planned R with no emitted owned body must be refused");
        assert!(
            format!("{error:?}").contains("never realized by an emitted owned body"),
            "{cause:?}: {error:?}"
        );
        for (identity, realization) in plan.fusion_outer_realizations() {
            let mut ledger = open();
            ledger
                .record_outer_realized(identity, realization.fusion(), realization.target())
                .expect("the first realization is accepted");
            let error = ledger
                .record_outer_realized(identity, realization.fusion(), realization.target())
                .expect_err("a second realization of one outer identity must be refused");
            assert!(
                format!("{error:?}").contains("realized twice"),
                "{cause:?}: {error:?}"
            );
        }
    }
}
