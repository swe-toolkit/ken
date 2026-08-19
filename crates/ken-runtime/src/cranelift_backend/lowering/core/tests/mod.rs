//! Subject-partitioned tests for the lowering SCC (RT-SPLIT §10.1).
//!
//! Slice 4 populates `control`, `effects` and `constructors` -- the subjects
//! whose tests reach `lowering::core`-private items. `values.rs` is populated
//! in slice 5 from the Architect's ruled row list (`evt_3xvn8g7n5rv7m`).

// `super` here is `core`; re-exported so the leaf subject modules inherit
// the same namespace via their own `use super::*`.
pub(in crate::cranelift_backend) use super::*;

// Ruled test module: imports are permitted here (AC-8 class 2). The subject
// modules reach these through this module rather than inheriting them from a
// production glob -- `lowering/mod.rs` must not import through the facade
// (§10.3), so the test subtree names its own fixture dependencies explicitly.
//
// RT-SPLIT slice 7, rule 8: this list was 31 names. The residual facade
// fixtures whose final-user LCA is this subtree have MOVED here (see the
// 37-item ledger), so they are now local declarations rather than imports.
// What remains is exactly the fixtures whose final-user LCA genuinely IS the
// facade, and which therefore have no lawful lower home:
//
//   emit_process_entrypoint_object_with_cranelift  -- also consumed by
//       `object_linker_packaging.rs:659`, ABOVE the facade, and its exact
//       declaration text is pinned by a cross-crate oracle in
//       `ken-cli/tests/px4b_native_production.rs:752`
//   total_primitive  -- users span `artifact/api/tests.rs` and this subtree
//
// `run_example_with_seed_observation` is a facade re-export of an
// `artifact::api` entrypoint, not one of the 37 residual fixtures.
pub(in crate::cranelift_backend) use super::super::super::{
    emit_process_entrypoint_object_with_cranelift, run_example_with_seed_observation,
};

// Crate-root items the subject tests assert against.
pub(in crate::cranelift_backend) use crate::{
    CraneliftObjectArtifact, RuntimeExample, RuntimeLowerabilityStatus, RuntimeObservation,
    RuntimeSymbolMetadata, UnsupportedLowering,
};

// Ruled test module: a `use` is permitted here (AC-8 class 2).
pub(in crate::cranelift_backend) use crate::cranelift_backend::test_support::{
    test_only_distinguished_root_join_plan, total_primitive,
};

// RT-SPLIT slice 7 (§10.5a′) — the three artifact privates these subject tests
// reach across the ownership boundary now arrive through owner-adjacent
// adapters instead of the facade, because the originals moved down into
// `artifact` and a sibling subtree cannot see them. Aliasing back to the
// original names keeps every leaf-test call token unchanged, so this is an
// IMPORT-ONLY edit: no subject test body changes and no production item is
// widened. §10.2 places these tests by the behavior they DISCRIMINATE, which
// is lowering — a setup callee living in `artifact` does not reassign them.
pub(in crate::cranelift_backend) use crate::cranelift_backend::artifact::{
    compile_expr_for_lowering_tests as compile_expr,
    new_jit_module_for_lowering_tests as new_jit_module,
    new_object_module_for_lowering_tests as new_object_module,
};

mod constructors;
mod control;
mod effects;

/// A real, planner-issued origin for a hand-built frame or layer that carries
/// **no** syntax children (an empty `cases` list, a childless residual).
///
/// Such a frame still needs an origin, and a test cannot invent one. This takes
/// the root of a minimal planned expression: no child is ever derived from it,
/// because it has none — and if a test ever did derive one, the positional
/// lookup would fail loudly rather than return a plausible neighbour.
#[cfg(test)]
fn inert_test_static_origin() -> StaticOriginId {
    planned_root_occurrence(&RuntimeExpr::Var(0)).1
}

/// The inert plan's source term.
///
/// A `static` rather than a temporary because the plan now **borrows** the term
/// it planned (B2A-S D2): a plan built from `&RuntimeExpr::Var(0)` in an
/// expression position would borrow a value dropped at the end of that statement.
/// Giving the term `'static` lets the returned plan satisfy any caller's lifetime.
#[cfg(test)]
static INERT_TEST_EXPR: RuntimeExpr = RuntimeExpr::Var(0);

/// The plan companion of `inert_test_static_origin`, for tests that build a
/// `Lowering` to exercise a validator rather than to lower an expression.
#[cfg(test)]
fn inert_test_plan() -> StaticTransitionPlan<'static> {
    planned_root_occurrence(&INERT_TEST_EXPR).0
}

/// Plans one expression on its own and returns the closed plan together with
/// its **root occurrence's** origin.
///
/// Unit tests that build a `Lowering` by hand need a real plan, because the
/// lowering derives every child origin out of it (`RT-FNSPLIT-B2A-C` D2). Note
/// that a test *cannot* fabricate an origin even if it wanted to:
/// `StaticOriginId`'s ordinal stays planner-private, so the only origins in
/// existence anywhere are the ones the planner issued.
#[cfg(test)]
fn planned_root_occurrence<'src>(
    expr: &'src RuntimeExpr,
) -> (StaticTransitionPlan<'src>, StaticOriginId) {
    let plan =
        plan_static_transition_graph(expr, &BTreeMap::new()).expect("test fixture is plannable");
    let root = plan
        .root_static_origin()
        .expect("a planned graph has a root source occurrence");
    (plan, root)
}

// Shared by >1 subject module: §10.2 places a helper at the lowest
// tests/mod.rs ancestor shared by its actual users.
fn console_write_effect() -> RuntimeExpr {
    RuntimeExpr::Effect {
        family: "Console".to_string(),
        operation: ken_host::HostOpV1::ConsoleWrite,
        capability: None,
        args: vec![
            RuntimeExpr::Construct {
                constructor: "ctor:prelude::Stream::Stdout".to_string(),
                args: Vec::new(),
            },
            RuntimeExpr::Value(RuntimeValue::Bytes(b"probe".to_vec())),
        ],
    }
}
// RT-SPLIT slice 7, rule 8 correction: this module's OWN code calls
// `recursive_computational_result_depth`, so its final-user LCA is HERE, not
// `control.rs`. My first ledger pass omitted `tests/mod.rs` from the user
// scan -- a parent cannot see a child's privates, so placing it in `control`
// broke this caller. One row, same window defect as the other two.
#[cfg(test)]
fn recursive_computational_result_depth(depth: usize, leaf_body: RuntimeExpr) -> RuntimeExpr {
    let node = "ctor:fixture::RecursiveTree::Node";
    let leaf = "ctor:fixture::RecursiveTree::Leaf";
    fn child(depth: usize, node: &str, leaf: &str) -> RuntimeExpr {
        RuntimeExpr::LexicalClosure {
            captures: Vec::new(),
            params: vec!["unit".to_string()],
            body: Box::new(if depth == 0 {
                RuntimeExpr::Construct {
                    constructor: leaf.to_string(),
                    args: Vec::new(),
                }
            } else {
                RuntimeExpr::Construct {
                    constructor: node.to_string(),
                    args: vec![child(depth - 1, node, leaf)],
                }
            }),
        }
    }
    let recursive_child = child(depth, node, leaf);
    RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: node.to_string(),
            args: vec![recursive_child],
        }),
        cases: vec![
            crate::RuntimeComputationalMatchCase {
                constructor: node.to_string(),
                argument_binders: 1,
                recursive_positions: vec![0],
                body: RuntimeExpr::Call {
                    callee: Box::new(RuntimeExpr::Var(0)),
                    args: vec![RuntimeExpr::Construct {
                        constructor: "ctor:prelude::Unit::MkUnit".to_string(),
                        args: Vec::new(),
                    }],
                },
            },
            crate::RuntimeComputationalMatchCase {
                constructor: leaf.to_string(),
                argument_binders: 0,
                recursive_positions: Vec::new(),
                body: leaf_body,
            },
        ],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "recursive tree default".to_string(),
        },
    }
}

fn recursive_computational_result(leaf_body: RuntimeExpr) -> RuntimeExpr {
    recursive_computational_result_depth(0, leaf_body)
}

// ── RT-SPLIT slice 7, rule 8 finalization ─────────────────────────────────
// Residual facade test fixtures whose final-user LCA is this module. Facade
// file scope was a TRANSITIONAL zero-widening holding position, never final
// ownership (Architect `evt_h69xwchqqxmj`); slice 7 discharges it. Moved
// verbatim -- ordered item-level identity, no body edits.

#[cfg(test)]
fn host_result_computational_fixture(
    ok_binders: usize,
    include_ok: bool,
    mismatched_result_kind: bool,
) -> RuntimeExpr {
    let result_ok = "ctor:prelude::Result::Ok".to_string();
    let result_err = "ctor:prelude::Result::Err".to_string();
    let scalar_tree = "ctor:fixture::Tree::Scalar".to_string();
    let exit_tree = "ctor:fixture::Tree::Exit".to_string();
    let mut producer_cases = vec![RuntimeMatchCase {
        constructor: result_err,
        binders: 1,
        body: RuntimeExpr::Construct {
            constructor: if mismatched_result_kind {
                exit_tree.clone()
            } else {
                scalar_tree.clone()
            },
            args: if mismatched_result_kind {
                Vec::new()
            } else {
                vec![RuntimeExpr::Value(RuntimeValue::Int((9).into()))]
            },
        },
    }];
    if include_ok {
        producer_cases.push(RuntimeMatchCase {
            constructor: result_ok,
            binders: ok_binders,
            body: RuntimeExpr::Construct {
                constructor: scalar_tree.clone(),
                args: vec![RuntimeExpr::Value(RuntimeValue::Int((7).into()))],
            },
        });
    }
    RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Effect {
                family: "Console".to_string(),
                operation: ken_host::HostOpV1::ConsoleWrite,
                capability: None,
                args: vec![
                    RuntimeExpr::Construct {
                        constructor: "ctor:prelude::Stream::Stdout".to_string(),
                        args: Vec::new(),
                    },
                    RuntimeExpr::Value(RuntimeValue::Bytes(b"probe".to_vec())),
                ],
            }),
            cases: producer_cases,
            default: RuntimeTrap {
                code: RuntimeTrapCode::PatternMatchFailure,
                message: "dynamic Result producer default".to_string(),
            },
        }),
        cases: vec![
            crate::RuntimeComputationalMatchCase {
                constructor: scalar_tree,
                argument_binders: 1,
                recursive_positions: Vec::new(),
                body: RuntimeExpr::Var(0),
            },
            crate::RuntimeComputationalMatchCase {
                constructor: exit_tree,
                argument_binders: 0,
                recursive_positions: Vec::new(),
                body: RuntimeExpr::Construct {
                    constructor: crate::EXIT_SUCCESS_CONSTRUCTOR.to_string(),
                    args: Vec::new(),
                },
            },
        ],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "computational tree default".to_string(),
        },
    }
}

#[cfg(test)]
fn constructor_field_aggregate() -> RuntimeExpr {
    RuntimeExpr::Match {
        scrutinee: Box::new(RuntimeExpr::Effect {
            family: "Console".to_string(),
            operation: ken_host::HostOpV1::ConsoleIsTerminal,
            capability: None,
            args: vec![RuntimeExpr::Construct {
                constructor: "ctor:prelude::Stream::Stdout".to_string(),
                args: Vec::new(),
            }],
        }),
        cases: [
            ("ctor:prelude::Bool::True", "ctor:prelude::Result::Ok", 7),
            ("ctor:prelude::Bool::False", "ctor:prelude::Result::Err", 9),
        ]
        .into_iter()
        .map(|(constructor, result, payload)| RuntimeMatchCase {
            constructor: constructor.to_string(),
            binders: 0,
            body: RuntimeExpr::Construct {
                constructor: result.to_string(),
                args: vec![RuntimeExpr::Value(RuntimeValue::Int((payload).into()))],
            },
        })
        .collect(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "px7p aggregate producer default".to_string(),
        },
    }
}

#[cfg(test)]
fn host_result_closure_match(argument: RuntimeExpr) -> RuntimeExpr {
    let exit_success = || RuntimeExpr::Construct {
        constructor: crate::EXIT_SUCCESS_CONSTRUCTOR.to_string(),
        args: Vec::new(),
    };
    RuntimeExpr::Call {
        callee: Box::new(ordinary_match_closure(
            vec![
                RuntimeMatchCase {
                    constructor: "ctor:prelude::Result::Err".to_string(),
                    binders: 1,
                    body: exit_success(),
                },
                RuntimeMatchCase {
                    constructor: "ctor:prelude::Result::Ok".to_string(),
                    binders: 1,
                    body: exit_success(),
                },
            ],
            RuntimeTrap {
                code: RuntimeTrapCode::PatternMatchFailure,
                message: "direct HostResult default".to_string(),
            },
        )),
        args: vec![argument],
    }
}

#[cfg(test)]
pub(super) fn big(sign: crate::Sign, limbs: &[u64]) -> RuntimeExpr {
    RuntimeExpr::Value(RuntimeValue::Int(crate::RuntimeIntV1::Big {
        sign,
        limbs: limbs.to_vec(),
    }))
}

#[cfg(test)]
fn ordinary_match_closure(cases: Vec<RuntimeMatchCase>, default: RuntimeTrap) -> RuntimeExpr {
    RuntimeExpr::LexicalClosure {
        captures: Vec::new(),
        params: vec!["value".to_string()],
        body: Box::new(RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Var(0)),
            cases,
            default,
        }),
    }
}

// ── RT-PLANNER-UNITS-ABI-SPLIT D2: Units/ABI-owned D5 declaration-call ──
// fixtures and the pre-emission ABI-domain-refusal control, moved up to the
// lowering/core/tests LCA. control.rs's retained D5 tests reach these through
// their existing `use super::*`.
#[cfg(test)]
fn oriented_test_interface(name: u8) -> crate::CheckedAnswerInterfaceV1 {
    let mut bytes = crate::CHECKED_ANSWER_INTERFACE_V1_HEADER.to_vec();
    bytes.push(name);
    crate::CheckedAnswerInterfaceV1::new(bytes).unwrap()
}

#[cfg(test)]
const D5_DECLARATION: &str = "decl:fixture::d5::loop";
const D5_FRAME_CARRIER: &str = "decl:fixture::d5::frames";
const D5_CALL_TEMPLATE: u64 = 900;
const D5_FRAME: u64 = 90;

fn d5_cases() -> Vec<crate::RuntimeComputationalMatchCase> {
    vec![crate::RuntimeComputationalMatchCase {
        constructor: "ctor:fixture::D5::Only".to_string(),
        argument_binders: 1,
        recursive_positions: Vec::new(),
        body: RuntimeExpr::Var(0),
    }]
}

#[cfg(test)]
fn d5_default() -> RuntimeTrap {
    RuntimeTrap {
        code: RuntimeTrapCode::PatternMatchFailure,
        message: "no runtime match case selected for ind:fixture::D5".to_string(),
    }
}

/// The declaration that carries the plan's one checked frame marker.
///
/// ⚠ It is **never referenced**, and that is deliberate. The transport
/// validator requires one Runtime frame marker per planned frame
/// (`planning.rs`: `markers.len() != plan.frames.len()`), but a
/// `ComputationalMatch` in the declaration under test would drag the
/// computational-recursor lane into a fixture about declaration calls. ⛔ Its
/// body must produce **no** residual of its own, or control 1 would be
/// measuring this declaration instead of the one it names.
#[cfg(test)]
fn d5_frame_carrier() -> RuntimeDeclaration {
    RuntimeDeclaration {
        symbol: D5_FRAME_CARRIER.to_string(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::CheckedSubcontinuationFrame {
                frame_id: D5_FRAME,
                body: Box::new(RuntimeExpr::ComputationalMatch {
                    scrutinee: Box::new(RuntimeExpr::Construct {
                        constructor: "ctor:fixture::D5::Only".to_string(),
                        args: vec![RuntimeExpr::Value(RuntimeValue::Int((0).into()))],
                    }),
                    cases: d5_cases(),
                    default: d5_default(),
                }),
            },
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    }
}

/// The declaration under test: one capture, one parameter, and one checked
/// same-SCC self-call in its body.
///
/// ⚠ The marker's structural path is `[3]` — `LexicalClosure` reaches its body
/// on edge `3` (`planning.rs::collect_checked_oriented_markers`), and captures
/// on edges `10 + i`. Deriving it any other way would make the fixture agree
/// with a mis-stated plan.
#[cfg(test)]
fn d5_declaration() -> RuntimeDeclaration {
    RuntimeDeclaration {
        symbol: D5_DECLARATION.to_string(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::LexicalClosure {
                captures: vec![RuntimeExpr::Value(RuntimeValue::Int((7).into()))],
                params: vec!["n".to_string()],
                body: Box::new(RuntimeExpr::CheckedRecursiveInvocation {
                    call_template_id: D5_CALL_TEMPLATE,
                    checked_occurrence_path: vec![5],
                    body: Box::new(RuntimeExpr::Call {
                        callee: Box::new(RuntimeExpr::DeclarationRef {
                            symbol: D5_DECLARATION.to_string(),
                        }),
                        args: vec![RuntimeExpr::Var(0)],
                    }),
                }),
            },
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    }
}

#[cfg(test)]
fn d5_frame() -> crate::OrientedSubcontinuationFramePlanV1 {
    let mut frame = crate::OrientedSubcontinuationFramePlanV1 {
        frame_id: D5_FRAME,
        segment_site_id: 9,
        declaration: D5_DECLARATION.to_string(),
        checked_occurrence_path: vec![D5_FRAME],
        semantic_position: 0,
        input_interface: oriented_test_interface(1),
        output_interface: oriented_test_interface(2),
        runtime_frame_fingerprint: crate::compiler_private_computational_match_frame_fingerprint(
            &d5_cases(),
            &d5_default(),
        ),
        occurrence_binding_fingerprint: 0,
        control_witness: crate::OrientedControlWitnessV1::DistinguishedRoot,
    };
    frame.occurrence_binding_fingerprint =
        crate::compiler_private_oriented_occurrence_binding_fingerprint(&frame);
    frame
}

#[cfg(test)]
fn d5_call_template() -> crate::CheckedRecursiveInvocationTemplateV1 {
    crate::CheckedRecursiveInvocationTemplateV1 {
        call_template_id: D5_CALL_TEMPLATE,
        declaration: D5_DECLARATION.to_string(),
        checked_occurrence_path: vec![5],
        callee: D5_DECLARATION.to_string(),
        level_instantiation: Vec::new(),
        recursion_group: "scc:fixture::d5".to_string(),
        scc_index: 0,
        admission: 1,
        arity: 1,
        local_telescope: vec![oriented_test_interface(1)],
        result_interface: oriented_test_interface(2),
        callee_segment_site_id: 9,
        callee_frame_templates: vec![D5_FRAME],
        caller_interface: oriented_test_interface(2),
        runtime_marker_locations: vec![crate::CheckedRuntimeMarkerLocationV1 {
            declaration: D5_DECLARATION.to_string(),
            runtime_path: vec![3],
        }],
        occurrence_binding_fingerprint: 0,
    }
}

/// The plan, **re-fingerprinted after `edit`**.
///
/// ⛔⛔ Re-fingerprinting is the whole reason a checked-plan mutation is a
/// control at all. `OrientedSubcontinuationPlanV1::validate` checks
/// `occurrence_binding_fingerprint` over EVERY field of the template, and it
/// runs on the compile path. ⇒ A mutation that leaves the stale fingerprint in
/// place is refused by the plan's own consistency law, upstream of `D5`, and a
/// control built on one would be measuring that law instead
/// ([[a-mutation-on-the-discriminator-input-measures-the-consistency-law-not-the-decision]]).
#[cfg(test)]
fn d5_plan_with(
    edit: impl FnOnce(&mut crate::CheckedRecursiveInvocationTemplateV1),
) -> crate::OrientedSubcontinuationPlanV1 {
    let mut call = d5_call_template();
    edit(&mut call);
    call.occurrence_binding_fingerprint =
        crate::compiler_private_recursive_call_binding_fingerprint(&call);
    crate::OrientedSubcontinuationPlanV1 {
        representation_rule_version:
            crate::OrientedSubcontinuationPlanV1::REPRESENTATION_RULE_VERSION,
        frames: vec![d5_frame()],
        recursive_calls: vec![call],
        computational_ih_slots: Vec::new(),
        computational_ih_calls: Vec::new(),
    }
}

#[cfg(test)]
fn d5_plan() -> crate::OrientedSubcontinuationPlanV1 {
    d5_plan_with(|_| {})
}

/// The entry expression: one unchecked call into the declaration-owned unit.
#[cfg(test)]
fn d5_entry() -> RuntimeExpr {
    RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::DeclarationRef {
            symbol: D5_DECLARATION.to_string(),
        }),
        args: vec![RuntimeExpr::Value(RuntimeValue::Int((5).into()))],
    }
}

/// Compile the fixture and return the outcome together with **the declaration
/// calls actually emitted**, read back from the emitted instructions.
#[cfg(test)]
fn d5_compile(
    plan: crate::OrientedSubcontinuationPlanV1,
    extra: Option<&RuntimeDeclaration>,
) -> (
    Result<(), String>,
    Vec<(StaticOriginId, StaticOriginId, cranelift_codegen::ir::FuncRef)>,
) {
    let entry = d5_entry();
    let declaration = d5_declaration();
    let carrier = d5_frame_carrier();
    let mut declarations = BTreeMap::from([
        (D5_DECLARATION, &declaration),
        (D5_FRAME_CARRIER, &carrier),
    ]);
    if let Some(extra) = extra {
        declarations.insert(extra.symbol.as_str(), extra);
    }
    reset_d5_emitted_declaration_calls();
    let outcome = compile_expr_into_module(
        new_jit_module().expect("JIT module"),
        "d5_declaration_unit_call",
        Linkage::Local,
        &entry,
        &NativeSeedEnvironment::empty(),
        declarations,
        None,
        false,
        None,
        None,
        Some(plan),
    )
    .map(|_| ())
    .map_err(|error| format!("{error:?}"));
    (outcome, d5_emitted_declaration_calls())
}

#[test]
fn d5_c4_abi_domain_mutations_each_refuse_before_any_call_is_emitted() {
    for mutation in [
        units::D5DeclaredCallMutation::Carrier,
        units::D5DeclaredCallMutation::Ownership,
        units::D5DeclaredCallMutation::StorageOwner,
        units::D5DeclaredCallMutation::Ordinal,
        units::D5DeclaredCallMutation::Header,
        units::D5DeclaredCallMutation::Offsets,
    ] {
        units::with_d5_declared_call_mutation(mutation, || {
            let (outcome, emitted) = d5_compile(d5_plan(), None);
            let refusal = outcome.expect_err(&format!(
                "D5 control 4: the {mutation:?} mutation must be refused. A \
                 compile that accepts it means the ABI reconciliation is \
                 not reading that field, and a green D5 would be green for \
                 the wrong reason"
            ));
            assert!(
                refusal.contains("disagree")
                    || refusal.contains("parameter-then-capture input run"),
                "D5 control 4: the {mutation:?} mutation must get D5's OWN \
                 refusal, not some later one it happens to also trip. \
                 Otherwise the control names a plane that never ran: {refusal}"
            );
            assert!(
                emitted.is_empty(),
                "D5 control 4: the {mutation:?} mutation must refuse BEFORE \
                 emission. A recorded call means a mis-declared frame was \
                 already written: {emitted:?}"
            );
        });
    }
    // The positive control on the harness: unmutated, the same fixture compiles
    // and emits BOTH declaration-unit calls — the entry's unchecked one and the
    // body's checked self-call. ⛔ Without this row every refusal above is
    // equally consistent with the fixture never reaching the seam at all.
    let (outcome, emitted) = d5_compile(d5_plan(), None);
    outcome.expect("D5 control 4: the unmutated fixture compiles");
    assert_eq!(
        emitted.len(),
        2,
        "D5 control 4: after D2a both the entry's unchecked call and the \
         body's checked self-call are emitted: {emitted:?}"
    );
}

// ── Control 4, the wrong-target class ─────────────────────────────────────

#[test]
fn d5_c4_a_retargeted_declaration_call_is_refused_before_emission() {
    let (baseline, baseline_emitted) = d5_compile(d5_plan(), None);
    baseline.expect("the unmutated fixture compiles");
    units::with_d5_declared_call_mutation(units::D5DeclaredCallMutation::Retarget, || {
        let (outcome, emitted) = d5_compile(d5_plan(), None);
        // ⚠ **This row measures the FIXTURE, and says so.** The retarget
        // swaps a caller's declaration-call record for another record in
        // the same caller's map. Each caller here holds exactly one, so the
        // swap is the identity and the compile is byte-for-byte the
        // baseline. That is a reachability fact
        // ([[mutation-proof-injection-point-is-a-reachability-tell]]), not
        // evidence about the wrong-target class.
        //
        // ⛔ It is kept, and kept honest, rather than deleted or dressed up
        // as a passing control: two declaration-owned callables reachable
        // from ONE caller is what makes the class expressible, and building
        // that fixture belongs with the mutual same-SCC work that D5 still
        // owes. Asserting equality with the baseline is what stops this
        // reading as a discharged control.
        assert!(
            outcome.is_ok() && emitted.len() == baseline_emitted.len(),
            "the retarget is inert on single-record callers, so it must \
             reproduce the baseline exactly: {outcome:?} {emitted:?}"
        );
    });
}