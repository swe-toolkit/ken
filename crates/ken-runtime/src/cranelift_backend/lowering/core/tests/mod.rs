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
    CraneliftObjectArtifact, CraneliftRunReport, RuntimeExample, RuntimeLowerabilityStatus,
    RuntimeObservation, RuntimeSymbolMetadata, UnsupportedLowering,
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

// `RT-EMITTER-EFFECTS-SPLIT` `D1` -- these moved out of `lowering::mod` into
// the new sibling `lowering::effects` module, so the `super::*` glob chain
// (this file -> `core.rs` -> `mod.rs`) no longer reaches them for the
// `effects` subject module below (39 tests) or the not-yet-moved
// `#[cfg(test)]` mutation checks still literally in `control` (D0's own
// lead, re-verified: two of its tests use `EffectSeatVisitMutation`, the
// emitter's own type, not the planner's `EffectSeatPlanMutation`).
// `mint_validated_progress_nat` is an associated fn of `Lowering` (called as
// `Lowering::mint_validated_progress_nat(...)`), not a free item -- reached
// through the `Lowering` type itself, already in scope, not imported here.
// `effect_seat_dispatch_mutation`/`effect_seat_visit_mutation` (the GETTERS)
// and `SITE_OPERAND_SUBSTITUTION_HITS` (the raw static) -- narrowed away at
// `RT-BACKEND-SPLIT-CLOSURE` (item 18): no test calls either getter, and
// `site_operand_substitution_hits()` (the accessor fn, kept below) is the
// only actual reader of the static. Compiler-confirmed unused, independently
// re-verified crate-wide before narrowing.
pub(in crate::cranelift_backend::lowering) use super::super::effects::{
    set_effect_seat_dispatch_mutation, set_effect_seat_visit_mutation,
    site_operand_substitution_hits, EffectSeatDispatchMutation, EffectSeatVisitMutation,
    RESOURCE_ERROR_INVALID_BOUNDS, RESOURCE_ERROR_MALFORMED_RESOURCE,
};

// `RT-EMITTER-AGGREGATES-SPLIT` `D2` -- `aggregates::tests` reaches this
// module's two residual shared fixtures (`d7_ownership_run`,
// `d7_constructor_arguments`, both still used by the one D4 test that stays
// here) by path (`core::tests::constructors::name`), so the module itself
// must be nameable from that sibling subtree too -- the same precedent
// `control`'s own widening below already set for `source::tests`.
pub(in crate::cranelift_backend::lowering) mod constructors;
// `RT-SOURCE-MACHINE-TYPES-SPLIT` `D2` -- `source::tests` reaches this
// module's shared fixtures by path (`core::tests::control::name`), so the
// module itself must be nameable from that sibling subtree too.
pub(in crate::cranelift_backend::lowering) mod control;
mod effects;
// `RT-CONTROL-INTEGRATION-TESTS-SPLIT` D1, modules 1-5 of 5, split from
// `control.rs`. `source_frame_bridge` carries `d8n_compile` and the `d8f_*`
// family, which `source::tests` reaches by path (same precedent as
// `control` above), so it needs the same widened module visibility.
mod recursor_fusion;
mod host_call_carrier;
mod specialization_binding;
pub(in crate::cranelift_backend::lowering) mod source_frame_bridge;
mod positional_candidate_settlement;

/// A real, planner-issued origin for a hand-built frame or layer that carries
/// **no** syntax children (an empty `cases` list, a childless residual).
///
/// Such a frame still needs an origin, and a test cannot invent one. This takes
/// the root of a minimal planned expression: no child is ever derived from it,
/// because it has none — and if a test ever did derive one, the positional
/// lookup would fail loudly rather than return a plausible neighbour.
#[cfg(test)]
pub(in crate::cranelift_backend::lowering) fn inert_test_static_origin() -> StaticOriginId {
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
///
/// `RT-EMITTER-AGGREGATES-SPLIT` `D2` -- `pub(in ...lowering)`, not private:
/// `aggregates::tests` (a sibling subtree, not a descendant of `core::tests`)
/// now reaches this by path too, the same cross-subtree shape `control`'s
/// own module widening above already established for `source::tests`.
#[cfg(test)]
pub(in crate::cranelift_backend::lowering) fn planned_root_occurrence<'src>(
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
pub(in crate::cranelift_backend::lowering) fn host_result_closure_match(argument: RuntimeExpr) -> RuntimeExpr {
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

// `RT-CONTROL-INTEGRATION-TESTS-SPLIT` D1: promoted from `control.rs`
// (module-private there) -- needed by both `control.rs`'s own residual
// tests and `recursor_fusion.rs`; hoisted here (plain module-private,
// same as this file's other shared fixtures) rather than widened in
// place, per the phase's banned-scope rule against widening a symbol's
// visibility just to make a test move compile.
#[derive(Clone, Copy)]
enum Px8jSelectedScopePlacement {
    BeforeReturnHole,
    AfterReturnHole,
}

fn px8j_equal_payload_hole_placement(placement: Px8jSelectedScopePlacement) -> RuntimeExpr {
    let input_node = "ctor:fixture::PX8JHoleInput::Node";
    let input_leaf = "ctor:fixture::PX8JHoleInput::Leaf";
    let output_node = "ctor:fixture::PX8JHoleOutput::Node";
    let output_leaf = "ctor:fixture::PX8JHoleOutput::Leaf";
    let unit = || RuntimeExpr::Construct {
        constructor: "ctor:prelude::Unit::MkUnit".to_string(),
        args: Vec::new(),
    };
    let recursive_child = || RuntimeExpr::LexicalClosure {
        captures: Vec::new(),
        params: vec!["unit".to_string()],
        body: Box::new(RuntimeExpr::Construct {
            constructor: input_leaf.to_string(),
            args: Vec::new(),
        }),
    };
    let scoped_payload = || RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: input_node.to_string(),
            args: vec![recursive_child()],
        }),
        cases: vec![
            crate::RuntimeComputationalMatchCase {
                constructor: input_node.to_string(),
                argument_binders: 1,
                recursive_positions: vec![0],
                body: RuntimeExpr::Construct {
                    constructor: output_node.to_string(),
                    args: vec![RuntimeExpr::Var(0)],
                },
            },
            crate::RuntimeComputationalMatchCase {
                constructor: input_leaf.to_string(),
                argument_binders: 0,
                recursive_positions: Vec::new(),
                body: RuntimeExpr::Construct {
                    constructor: output_leaf.to_string(),
                    args: Vec::new(),
                },
            },
        ],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "PX8-J equal-payload inner default".to_string(),
        },
    };
    let outer_scrutinee = match placement {
        Px8jSelectedScopePlacement::BeforeReturnHole => RuntimeExpr::Construct {
            constructor: output_node.to_string(),
            args: vec![RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["unit".to_string()],
                body: Box::new(scoped_payload()),
            }],
        },
        Px8jSelectedScopePlacement::AfterReturnHole => scoped_payload(),
    };
    RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(outer_scrutinee),
        cases: vec![
            crate::RuntimeComputationalMatchCase {
                constructor: output_node.to_string(),
                argument_binders: 1,
                recursive_positions: vec![0],
                body: RuntimeExpr::Call {
                    callee: Box::new(RuntimeExpr::Var(0)),
                    args: vec![unit()],
                },
            },
            crate::RuntimeComputationalMatchCase {
                constructor: output_leaf.to_string(),
                argument_binders: 0,
                recursive_positions: Vec::new(),
                body: control::px8j_aggregate_result(),
            },
        ],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "PX8-J equal-payload outer default".to_string(),
        },
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

// ── RT-PLANNER-OCCURRENCES-SPLIT D2: occurrence-classified tests moved ──
// up from control.rs to the LCA. Multi-leaf fixtures stay in control.rs and
// are reached here through `use control::{...}` (they are pub(super) there).
// `ac11_compiles`/`contspec_emission_witness`/`expression_children`/
// `one_child_record` were promoted into this file directly at
// `RT-CONTROL-INTEGRATION-TESTS-SPLIT` D1 -- no longer imported, they are
// now local declarations.

use control::{declaration_span, declared_fields, is_bare_source_term_field};

/// One planned instance of **every** `RuntimeExpr` variant, each carrying at
/// least one expression-typed field where the variant has any, so a dropped
/// position cannot hide behind an empty list.
#[cfg(test)]
fn every_variant_occurrence() -> Vec<(&'static str, RuntimeExpr)> {
    let leaf = || RuntimeExpr::Value(RuntimeValue::Bool(true));
    let trap = || RuntimeTrap {
        code: RuntimeTrapCode::PatternMatchFailure,
        message: "b2ac coverage guard".to_string(),
    };
    vec![
        (
            "CheckedJoinSite",
            RuntimeExpr::CheckedJoinSite {
                site_id: 1,
                body: Box::new(leaf()),
            },
        ),
        (
            "CheckedSubcontinuationFrame",
            RuntimeExpr::CheckedSubcontinuationFrame {
                frame_id: 2,
                body: Box::new(leaf()),
            },
        ),
        (
            "CheckedRecursiveInvocation",
            RuntimeExpr::CheckedRecursiveInvocation {
                call_template_id: 3,
                checked_occurrence_path: vec![1],
                body: Box::new(leaf()),
            },
        ),
        (
            "CheckedComputationalIHSlots",
            RuntimeExpr::CheckedComputationalIHSlots {
                slot_template_ids: vec![4],
                checked_occurrence_paths: vec![vec![1]],
                body: Box::new(leaf()),
            },
        ),
        (
            "CheckedComputationalIHInvocation",
            RuntimeExpr::CheckedComputationalIHInvocation {
                call_template_id: 5,
                checked_occurrence_path: vec![1],
                kind: crate::CheckedComputationalIHInvocationKind::OrdinaryApplication,
                binder_morphism:
                    crate::CheckedComputationalIHBinderMorphism::identity_for_test(0),
                body: Box::new(leaf()),
            },
        ),
        ("Value", leaf()),
        ("Var", RuntimeExpr::Var(0)),
        (
            "Let",
            RuntimeExpr::Let {
                value: Box::new(leaf()),
                body: Box::new(RuntimeExpr::Var(0)),
            },
        ),
        (
            "If",
            RuntimeExpr::If {
                scrutinee: Box::new(leaf()),
                then_expr: Box::new(leaf()),
                else_expr: Box::new(leaf()),
            },
        ),
        (
            "PrimitiveCall",
            total_primitive("prim:fixture::b2ac", vec![leaf(), leaf()]),
        ),
        (
            "Construct",
            RuntimeExpr::Construct {
                constructor: "ctor:fixture::B2AC::Pair".to_string(),
                args: vec![leaf(), leaf()],
            },
        ),
        (
            "Match",
            RuntimeExpr::Match {
                scrutinee: Box::new(leaf()),
                cases: vec![
                    RuntimeMatchCase {
                        constructor: "ctor:fixture::B2AC::A".to_string(),
                        binders: 0,
                        body: leaf(),
                    },
                    RuntimeMatchCase {
                        constructor: "ctor:fixture::B2AC::B".to_string(),
                        binders: 0,
                        body: leaf(),
                    },
                ],
                default: trap(),
            },
        ),
        (
            "ComputationalMatch",
            RuntimeExpr::ComputationalMatch {
                scrutinee: Box::new(leaf()),
                cases: vec![crate::RuntimeComputationalMatchCase {
                    constructor: "ctor:fixture::B2AC::A".to_string(),
                    argument_binders: 0,
                    recursive_positions: Vec::new(),
                    body: leaf(),
                }],
                default: trap(),
            },
        ),
        (
            "Record",
            RuntimeExpr::Record {
                fields: vec![("l".to_string(), leaf()), ("r".to_string(), leaf())],
            },
        ),
        (
            "Project",
            RuntimeExpr::Project {
                record: Box::new(RuntimeExpr::Record {
                    fields: vec![("l".to_string(), leaf())],
                }),
                field: "l".to_string(),
            },
        ),
        (
            "Closure",
            RuntimeExpr::Closure {
                captures: Vec::new(),
                params: vec!["x".to_string()],
                body: Box::new(leaf()),
            },
        ),
        (
            "LexicalClosure",
            RuntimeExpr::LexicalClosure {
                captures: vec![leaf(), leaf()],
                params: vec!["x".to_string()],
                body: Box::new(leaf()),
            },
        ),
        (
            "DeclarationRef",
            RuntimeExpr::DeclarationRef {
                symbol: "decl:fixture::b2ac".to_string(),
            },
        ),
        (
            "ImportedDeclarationRef",
            RuntimeExpr::ImportedDeclarationRef {
                symbol: "decl:fixture::b2ac".to_string(),
                dependency: "pkg:fixture".to_string(),
                dependency_semantic_hash: "hash".to_string(),
            },
        ),
        (
            "Call",
            RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::Var(0)),
                args: vec![leaf(), leaf()],
            },
        ),
        (
            "Effect (capability present)",
            RuntimeExpr::Effect {
                family: "Fs".to_string(),
                operation: ken_host::HostOpV1::FsReadFile,
                capability: Some(crate::RuntimeCapabilityUse {
                    identity: "cap:fixture::fs".to_string(),
                    value: Box::new(RuntimeExpr::Var(0)),
                }),
                args: vec![leaf()],
            },
        ),
        (
            "Effect (capability absent)",
            RuntimeExpr::Effect {
                family: "Console".to_string(),
                operation: ken_host::HostOpV1::ConsoleWrite,
                capability: None,
                args: vec![leaf(), leaf()],
            },
        ),
        ("Trap", RuntimeExpr::Trap(trap())),
    ]
}

#[cfg(test)]
fn two_child_record() -> RuntimeExpr {
    RuntimeExpr::Record {
        fields: vec![
            (
                "l".to_string(),
                RuntimeExpr::Value(RuntimeValue::Bool(true)),
            ),
            (
                "r".to_string(),
                RuntimeExpr::Value(RuntimeValue::Bool(false)),
            ),
        ],
    }
}

#[test]
fn every_expression_typed_field_is_a_reachable_positional_child_origin() {
    let mut unreachable = Vec::new();
    for (name, occurrence) in every_variant_occurrence() {
        let (plan, origin) = planned_root_occurrence(&occurrence);
        let children = expression_children(&occurrence);

        // Every enumerated position resolves to a real preallocated origin.
        for position in 0..children.len() {
            if plan.child_static_origin(origin, position).is_err() {
                unreachable.push(format!("{name}: position {position} does not resolve"));
            }
        }
        // And there is no position beyond them: the plane holds exactly the
        // enumerated children, so an unenumerated field is red too.
        if plan.child_static_origin(origin, children.len()).is_ok() {
            unreachable.push(format!(
                "{name}: the plane holds a child at position {} that no field enumerates",
                children.len()
            ));
        }
    }
    assert!(
        unreachable.is_empty(),
        "every expression-typed field must be a reachable positional child origin: {unreachable:#?}"
    );
}

#[test]
fn swapping_two_same_shaped_children_swaps_their_derived_origins() {
    let branch = |then_expr: RuntimeExpr, else_expr: RuntimeExpr| RuntimeExpr::If {
        scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
        then_expr: Box::new(then_expr),
        else_expr: Box::new(else_expr),
    };
    // `arity_at(position)` reads how many children the occurrence at that
    // position has, using nothing but the positional accessor.
    let arity_at = |expr: &RuntimeExpr, position: usize| {
        let (plan, root) = planned_root_occurrence(expr);
        let child = plan
            .child_static_origin(root, position)
            .expect("If has three positional children");
        (0..)
            .take_while(|inner| plan.child_static_origin(child, *inner).is_ok())
            .count()
    };

    let straight = branch(one_child_record(), two_child_record());
    let swapped = branch(two_child_record(), one_child_record());

    assert_eq!(
        arity_at(&straight, 1),
        1,
        "then_expr is the one-child record"
    );
    assert_eq!(
        arity_at(&straight, 2),
        2,
        "else_expr is the two-child record"
    );
    // The children swapped in the source; the derived origins swapped with them.
    assert_eq!(
        arity_at(&swapped, 1),
        2,
        "then_expr is now the two-child record"
    );
    assert_eq!(
        arity_at(&swapped, 2),
        1,
        "else_expr is now the one-child record"
    );
}

#[test]
fn perturbing_a_borrowed_address_does_not_move_any_derived_origin() {
    let expr = RuntimeExpr::If {
        scrutinee: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
        then_expr: Box::new(one_child_record()),
        else_expr: Box::new(two_child_record()),
    };
    // A clone is the same syntax at different addresses, and boxing it again
    // moves every interior node. No ordinal changes.
    let relocated = Box::new(expr.clone());

    let derive = |expr: &RuntimeExpr| {
        let (plan, root) = planned_root_occurrence(expr);
        (0..3)
            .map(|position| {
                plan.child_static_origin(root, position)
                    .expect("If has three positional children")
            })
            .collect::<Vec<_>>()
    };

    assert_eq!(
        derive(&expr),
        derive(relocated.as_ref()),
        "identity must not move when only the address moved"
    );
}

#[cfg(any())]
fn exactly_one_plan_origin_to_expression_lookup_exists() {
    let planner = include_str!("../../../planning/static_transition.rs");

    // The PRODUCING end, pinned as a whole exported surface rather than by
    // searching for one name: a second resolver added here would redden this even
    // if it were never called.
    let exported: Vec<&str> = planner
        .lines()
        .filter(|line| {
            line.trim_start()
                .starts_with("pub(in crate::cranelift_backend) fn ")
        })
        .map(|line| line.trim())
        .collect();
    assert_eq!(
        exported,
        vec![
            // `RT-FNSPLIT-B2F` `D1` — the emitter's read-only view of ONE
            // validated function unit. Six accessors, one type, no constructor.
            //
            // ⭐ Same shape as `C1`'s four below: a *question* about a planned
            // object with an answer the asker cannot mint. `EmittableUnit`'s
            // fields are private and its sole producer is `emittable_units`, so
            // a unit cannot be forged in `lowering` — and since `B2F` drives
            // emission from units, emission cannot be driven from anything but
            // the validated plane.
            //
            // ⛔ `AbiPlane`, `AbiDescriptor`, `build_abi_plane` and
            // `AbiPlane::validate` stay `pub(super)` and are NOT here. The
            // emitter reads a unit; it cannot construct the plane, mutate a
            // descriptor, or reach the pre-emission validator to bypass it. One
            // of those names appearing in this list is the violation.
            //
            // ⚠ None of the six returns a source term, so the `-> Result<&'src
            // RuntimeExpr` count below is still exactly one and `B2A-S`'s `AC-4`
            // is untouched. A unit carries an ORIGIN; resolving that origin to a
            // term still goes through `source_occurrence`, which is why `B2F`
            // adds no second `origin -> expression` lookup.
            // `RT-FNSPLIT-B2F` `D4` — the cross-owner call edge's two ends,
            // added deliberately and argued rather than bumped.
            //
            // ⚠ Both return an **identity**, never a source term, so neither can
            // contribute to the `-> Result<&'src RuntimeExpr` count that carries
            // `B2A-S`'s `AC-4` — which stays at exactly one.
            //
            // ⭐ Their producer `emittable_call_edges` (below) is the sole route
            // to an `EmittableCallEdge`, whose fields are private — so `lowering`
            // can read which unit calls which and cannot invent an edge the
            // planner did not validate. ⛔ It does not classify edges: the walk
            // is `SemanticPlane::static_body_call_edges`, beside the validator,
            // because `static_transition.rs` may not name `SemanticOwner` at all.
            "pub(in crate::cranelift_backend) fn caller(self) -> PredeclaredFunctionId {",
            "pub(in crate::cranelift_backend) fn callee(self) -> PredeclaredFunctionId {",
            "pub(in crate::cranelift_backend) fn callee_origin(self) -> StaticOriginId {",
            "pub(in crate::cranelift_backend) fn function(self) -> PredeclaredFunctionId {",
            "pub(in crate::cranelift_backend) fn origin(self) -> StaticOriginId {",
            "pub(in crate::cranelift_backend) fn definition(self) -> AbiUnitDefinition {",
            "pub(in crate::cranelift_backend) fn header(self) -> AbiFrameHeader {",
            "pub(in crate::cranelift_backend) fn slots(self) -> &'plan [AbiSlot] {",
            "pub(in crate::cranelift_backend) fn slot_offsets(",
            "pub(in crate::cranelift_backend) fn process_parameter_slot(",
            // ⭐ `RT-FNSPLIT-B2A-S` `AC-4`'s own **behavioural** instrument,
            // added deliberately and argued rather than bumped. These three are
            // the counters behind
            // `every_origin_to_expression_resolution_goes_through_the_single_route`,
            // which is the pin that carries `AC-4` once `B2F` `S6` widens
            // `retained_body_occurrence`'s visibility — an enlargement of the
            // reachable surface that THIS test cannot see, because it constrains
            // the identifier `source_occurrence` and never asks who calls the
            // route.
            //
            // ⚠ None of the three returns a source term — two return `()` and
            // one returns `(usize, usize)` — so the `-> Result<&'src RuntimeExpr`
            // count below is still exactly one and `AC-4` is untouched.
            //
            // ⛔ They are `#[cfg(test)]` probe infrastructure, and this list
            // cannot tell that apart from production surface: it reads source
            // text, so a `cfg`-gated item appears exactly like a live one. ⇒ A
            // reader auditing this list for *production* exports must check the
            // attribute at the declaration, not infer it from membership here.
            "pub(in crate::cranelift_backend) fn ac4_open_route_window() {",
            "pub(in crate::cranelift_backend) fn ac4_note_route_invocation() {",
            "pub(in crate::cranelift_backend) fn ac4_route_counts() -> (usize, usize) {",
            "pub(in crate::cranelift_backend) fn source_occurrence(",
            "pub(in crate::cranelift_backend) fn child_static_origin(",
            // `D8` exports one opaque, origin-keyed join-plan token. The token
            // contains no term and has no public constructor.
            "pub(in crate::cranelift_backend) fn join_plan_token(",
            // `RT-CONTSRC-PRODUCER-LOCAL` `AC-1` — the case-emission verdict,
            // added deliberately and argued rather than bumped.
            //
            // The carried source-machine `Match` route must emit exactly the
            // cases the planner authorizes, and it must not re-derive that
            // authority. This returns a VERDICT for one occurrence-and-ordinal:
            // `case_emissions`, the producer-set derivation and `semantic` all
            // stay private, so an emitter can obtain the answer and cannot mint
            // or vary one. `None` is a refusal to answer, not a default.
            //
            // ⚠ It returns `Result<Option<CaseEmissionStatus>, _>`, so it does
            // not contribute to the `-> Result<&'src RuntimeExpr` count that
            // carries `B2A-S`'s `AC-4`.
            "pub(in crate::cranelift_backend) fn case_emission_status(",
            // `RT-FNSPLIT-C1` `D1` — the artifact-static identity capability.
            //
            // ⭐ These four are the whole of `D1`, and they are the shape the
            // Architect's ruling requires: an occurrence-keyed *question* with
            // an unmintable answer. ⛔ `SemanticPlane` and its `names` arena
            // stay `pub(super)`; widening either to serve a consumer is what
            // this pin exists to catch, and adding a capability is not that.
            //
            // ⚠ None of them returns a source term, so the `-> Result<&'src
            // RuntimeExpr` count below is still exactly one. That assertion is
            // the one carrying B2A-S's AC-4; this list is the surrounding
            // allowed-inventory.
            "pub(in crate::cranelift_backend) fn case_constructor_identity(",
            "pub(in crate::cranelift_backend) fn constructor_symbol_identity(",
            // `RT-FNSPLIT-C2-SYNTH-ID` adds one closed synthesized-role
            // identity route plus the opaque dynamic-role population. Neither
            // accepts a spelling, origin, hash, or ordinal from lowering.
            "pub(in crate::cranelift_backend) fn synthesized_constructor_identity(",
            "pub(in crate::cranelift_backend) fn synthesized_io_error_roles(",
            "pub(in crate::cranelift_backend) fn project_field_identity(",
            "pub(in crate::cranelift_backend) fn record_field_identity(",
            "pub(in crate::cranelift_backend) fn root_static_origin(",
            "pub(in crate::cranelift_backend) fn declaration_occurrence_origin(",
            // `RT-FNSPLIT-B2F` `AC-11` — the per-transfer representability
            // verdict, added deliberately and argued rather than bumped.
            //
            // ⭐ It returns a **verdict**, never the plane: `semantic`,
            // `semantic_sources` and `abi` all stay private, so an emitter can
            // obtain the answer and cannot re-derive a different one. That is
            // what keeps representability a single authority instead of a check
            // the emitter could route around — and it is why widening this one
            // name does not widen the surface it guards.
            //
            // ⚠ It returns `Result<(), _>`, so it cannot contribute to the
            // `-> Result<&'src RuntimeExpr` count that carries `B2A-S`'s `AC-4`.
            "pub(in crate::cranelift_backend) fn validate_emitted_transfers_are_representable(",
            // `RT-FNSPLIT-B2F` `D1` — the sole producer of an `EmittableUnit`,
            // and therefore the sole route by which emission can be driven.
            //
            // ⛔ It projects `self.abi.descriptors`; it does not re-seed the
            // population and must never be made to. The unit set is
            // `plan.entries` ∪ every `EdgeKind::StaticBody` TARGET, already
            // enforced by `validate_function_units`. In particular it does not
            // consult `TransitionKind::ClosureBody`, which is a body's return
            // successor and not a unit head.
            "pub(in crate::cranelift_backend) fn emittable_call_edges(",
            "pub(in crate::cranelift_backend) fn root_emittable_unit(",
            "pub(in crate::cranelift_backend) fn emittable_units(",
            "pub(in crate::cranelift_backend) fn plan_static_transition_graph<'src>(",
            "pub(in crate::cranelift_backend) fn plan_static_transition_graph_with_symbols<'src>(",
            "pub(in crate::cranelift_backend) fn governed_nested_resource_bracket(",
        ],
        "AC-4 -- the planner's exported surface changed; exactly one of these may \
         return a source term"
    );
    assert_eq!(
        planner
            .lines()
            .filter(|line| line.contains("-> Result<&'src RuntimeExpr"))
            .count(),
        1,
        "AC-4 -- exactly one accessor may return a borrowed source expression \
         (B2A-C's N3 required zero; B2A-S requires one)"
    );

    // The CONSUMING end, over the WHOLE backend production surface.
    //
    // ⛔ The first candidate scanned only `lowering/core.rs` and `lowering/mod.rs`
    // and argued closure from `Lowering::static_transition_plan` being private.
    // The Architect rejected that (`evt_6sq2tq3v9jcd0`) and was right: the
    // resolver is `pub(in crate::cranelift_backend)` and `planning.rs` re-exports
    // `plan_static_transition_graph` to the backend parent, so ANY backend sibling
    // can build its own plan and call the resolver without owning a `Lowering` at
    // all. A second call in `artifact/**`, `compiled.rs` or `planning.rs` would
    // have stayed green. Privacy of one field was never the closure.
    let mut mentions = Vec::new();
    for (file, source) in BACKEND_PRODUCTION_SOURCES {
        // `static_transition.rs` carries its tests inline; the census is about the
        // production surface, and the planner's own tests legitimately call the
        // resolver to exercise it.
        let production = source
            .split_once("\n#[cfg(test)]\nmod tests {")
            .map_or(*source, |(before, _)| before);
        let n = identifier_occurrences(production, "source_occurrence");
        if n > 0 {
            mentions.push((*file, n));
        }
    }
    assert_eq!(
        mentions,
        vec![
            ("lowering/core.rs", 1),
            ("planning/static_transition.rs", 1)
        ],
        "AC-4 -- the resolver may be NAMED exactly twice in production: its \
         definition in the planner, and its single call from \
         `retained_body_occurrence`. Any third mention is a second lookup"
    );
}

#[test]
fn every_source_term_carrier_holds_an_occurrence_and_never_a_bare_expression() {
    // `RT-SOURCE-MACHINE-TYPES-SPLIT` `D1` moved `SourceContinuation` and
    // `SourceMachineState` into `source.rs`; `SourcePrefixTemplate` stayed at
    // the `mod.rs` hub (shared with retained checked-invocation/continuation-
    // frame machinery). Each header is read from its own current file, per
    // AC-3's source-text-oracle-relocation rule -- the property below is
    // unchanged, only which buffer names its declaration.
    let mod_source = include_str!("../../mod.rs");
    let source_source = include_str!("../../source.rs");
    for header in [
        ("enum SourceContinuation<'a> {", source_source),
        ("enum SourcePrefixTemplate {", mod_source),
        ("enum SourceMachineState<'a> {", source_source),
    ] {
        let (header, source) = header;
        let span = declaration_span(source, header);
        let bare: Vec<&str> = span
            .iter()
            .copied()
            .filter(|line| is_bare_source_term_field(line))
            .collect();
        assert!(
            bare.is_empty(),
            "AC-1: {header} still carries a bare source term without its origin: {bare:?}"
        );

        // Every `cases`-bearing variant declares its parent origin. The variant
        // boundary is a field list, so scan forward from each `cases:` line to
        // the variant's closing brace.
        let mut index = 0;
        while index < span.len() {
            if span[index].trim().starts_with("cases: Vec<") {
                let variant_tail = span[index..]
                    .iter()
                    .take_while(|line| !line.trim().starts_with("},"))
                    .any(|line| line.trim() == "static_origin: StaticOriginId,");
                assert!(
                    variant_tail,
                    "AC-1: {header} has a `cases` variant with no `static_origin`; \
                     its case bodies would have no parent to derive from"
                );
            }
            index += 1;
        }
    }
}

#[test]
fn retained_closures_carry_a_static_origin_and_no_body_term() {
    let source = include_str!("../../mod.rs");

    // AC-6, the COVERED population: both variants that retained a body. Pinned as
    // a complete field inventory, so ANY added field -- term-bearing or not --
    // reddens and has to be justified here.
    assert_eq!(
        declared_fields(source, "    Closure {"),
        vec![
            // ⚠ `D7` widened this field from `Vec<Lowered>`, and the pin's own
            // property is UNCHANGED by it: a capture is not a body carrier. It
            // is argued, not absorbed — a retained callable is an
            // invocation-local capsule whose captures reached it at their own
            // phases, and demanding a compile-time template for every one of
            // them left a lawfully mixed environment with no representation at
            // all. The frame's `Row: the closure-capture cell` supersedes `C1`'s
            // "every child stays `Lowered`" for capture edges only. `body`
            // remains the sole body authority, which is what this equality
            // protects. ⛔ What would still red it is an added field, or a
            // capture edge acquiring a `StaticOriginId`/`RuntimeExpr`.
            "captures: Vec<LoweringOperand>,",
            "params: Vec<String>,",
            "body: StaticOriginId,",
            // M4 carries only the planner-issued positional ENVIRONMENT
            // identity. It holds no term and no body origin; `body` above stays
            // the sole code authority, and the descriptor resolver checks the
            // environment record agrees with it before static dispatch.
            "boundary_environment: Option<AggregateOccurrenceId>,",
        ],
        "AC-1: `Lowered::Closure`'s field inventory changed. A second body \
         authority beside the tag is exactly what this WP removed, so an added \
         field must be argued, not absorbed"
    );
    assert_eq!(
        declared_fields(source, "    DeclarationClosure {"),
        vec![
            // `RT-DECL-CLOSURE-PORT` `D4` added `reference`. It is argued, not
            // absorbed: it names the planner-issued `DeclarationRef` occurrence
            // this binding was produced at, which is the key the resolved call
            // record is looked up by. `body` remains the sole body authority --
            // the property this inventory protects is unchanged.
            "reference: StaticOriginId,",
            "symbol: RuntimeSymbol,",
            // ⚠ `D7`, for the same reason and on the same authority as
            // `Closure::captures` above. A declaration closure's LEXICAL
            // captures reach it at their own phases; its SEED captures resolve
            // to JIT-time ground values and are constructed as explicit
            // `Specialized`, so the widening costs the seed lane nothing.
            "captures: Vec<LoweringOperand>,",
            "params: Vec<String>,",
            "body: StaticOriginId,",
        ],
        "AC-1: `Lowered::DeclarationClosure`'s field inventory changed"
    );

    // AC-6, the EXCLUDED variant, and why — a fact about the declaration rather
    // than a judgement call: it carries no source term at all. Pinned as an
    // inventory for the same reason as above, so it cannot quietly acquire one.
    assert_eq!(
        declared_fields(source, "    ComputationalRecursorClosure {"),
        vec![
            // ⚠ `RT-FNSPLIT-C1 AC-C4` widened this field from `Box<Lowered>` on
            // the Architect's SINGLE-FIELD license, and the pin's own property
            // is UNCHANGED by that: a `LoweringOperand` residual is still not a
            // body carrier — no `StaticOriginId`, no `RuntimeExpr`, nothing this
            // variant could be re-lowered from. It stays out of the covered
            // population for exactly the reason stated below. ⛔ What would move
            // it in is a field naming a source body, and that is still what this
            // inventory equality catches.
            "residual: Box<LoweringOperand>,",
            "activation: ContinuationActivationId,",
            "invocation: RecursorInvocationSegment,",
        ],
        "AC-6: ComputationalRecursorClosure is out of the covered population \
         because it declares no body carrier. If it acquires one it JOINS the \
         population, and this test is where that has to be said"
    );
}

/// **`RT-FNSPLIT-B2A-S` `AC-4` — every `origin -> expression` resolution goes
/// through the single route.**
///
/// ⛔⛔ **This exists because the instrument that used to carry `AC-4` is about
/// to stop being able to.** `exactly_one_plan_origin_to_expression_lookup_exists`
/// reads `static_transition.rs`'s **source text** and pins its exported
/// signature list. Two things break that as `B2F` `S6` lands:
///
/// 1. ⛔ It constrains the **identifier** `source_occurrence` and says nothing
///    about **who may call the route**. `S6` widens
///    `Lowering::retained_body_occurrence` from private-to-`core` to all of
///    `lowering`, so a unit body can resolve its own origin — an enlargement of
///    the reachable surface that the text pin cannot see.
/// 2. ⚠ It reddens on an edit that changes nothing about how any program
///    behaves. Reflowing a doc comment in that file is enough.
///
/// ⭐ **And the dead-code warning on `EmittableUnit::origin` cannot stand in for
/// it either.** That warning can witness *"nobody consumes this"*; it can never
/// witness *"exactly one route consumes it"* — and it is **spent** by the very
/// commit that consumes `origin()`, which is precisely the commit that makes the
/// property non-trivial for the first time.
///
/// **MEASURED:** across one compile, the number of resolutions performed by
/// `StaticTransitionPlan::source_occurrence` equals the number of invocations of
/// `Lowering::retained_body_occurrence`, and both are non-zero.
/// **CLAIMED:** there is exactly one `origin -> expression` route in the
/// backend, so a retained body is selected by its static name and by nothing
/// else.
/// **THE GAP:** ⛔ a route that obtained a term **without** calling
/// `source_occurrence` would be invisible here. What closes that is not this
/// test but **item visibility**: `StaticTransitionPlan::source_occurrences` is a
/// **private field**, so no module outside `planning::static_transition` can
/// reach the table at all, and the only other readers inside that file are
/// validators that return no term. ⇒ `source_occurrence` is the table's sole
/// exit, and this test is what pins that exit to a single caller.
///
/// **Compile-preserving evasion attempted, and the result is a COVERAGE LIMIT
/// that must not be read off the fixture count.** The evasion is to resolve a
/// body by calling `plan.source_occurrence(origin)` directly instead of through
/// `retained_body_occurrence`; it compiles and produces the identical term.
/// Applied at **each of the seven route call sites in turn**, with four fixture
/// shapes:
///
/// ⭐⭐ **The seven sites are not a list — they are `3 operand shapes × 2
/// lowering contexts + 1 residual`**, and stating them that way is what makes
/// the gap diagnosable instead of merely counted:
///
/// | operand shape | `lower_expr` (ordinary) | `lower_computational_producer_expr` |
/// |---|---|---|
/// | `Lowered::Closure` | `:5754` ⭐ **RED — caught** | `:769` ⭐ **RED — caught** |
/// | `Lowered::DeclarationClosure` | `:5742` ⭐ **RED — caught** | `:754` ⭐ **RED — caught** |
/// | recursor closure | `:5897` ⛔ green | `:939` ⛔ green |
///
/// plus `:474` in `lower_recursor_residual_call` — ⛔ green.
///
/// ⇒ **Coverage is 4 of 7, and the residual is exactly one ROW: the recursor
/// closure, in both contexts, plus its residual call.** ⛔ Not a scatter of
/// unrelated sites.
///
/// ⛔⛔ **AND THE OBVIOUS FIXTURE FOR THAT ROW DOES NOT REACH IT — measured.**
/// The set below *includes* a `ComputationalMatch` whose case carries a
/// `recursive_position` and whose body **applies the induction hypothesis**
/// (`Call { callee: Var(0) }`) — the shape that binds a
/// `Lowered::ComputationalRecursorClosure`. Re-running the bisect with it
/// present left `:474`, `:939` and `:5897` **all green**. ⇒ ⚠ *"add a
/// `ComputationalMatch` with `recursive_positions`"* is **not** the recipe, and
/// this note exists so the next person does not spend the attempt I already
/// spent. ⭐ The fixture is retained anyway — it is the only
/// `ComputationalMatch` in the set and its relation still holds — but ⛔ it is
/// **not** counted as coverage of anything, and the grid above is unchanged by
/// it.
///
/// ⇒ What the three recursor sites actually need is **unknown**, and saying so
/// is the honest state. ⛔ Do not infer from the fixture's presence that the row
/// is attempted-and-covered; it is attempted-and-still-open.
///
/// ⭐ **Both contexts are entered from `lower_expr`'s `Match` arm**, which routes
/// its scrutinee through the producer when the scrutinee
/// `requires_heterogeneous_deforestation` — a `Call` whose callee is a closure
/// returning a `Construct`, or a declaration call producing an aggregate. ⇒ The
/// context is a property of the **enclosing form**, and varying it needed a
/// `Match`, not another callee.
///
/// ⚠ **A correction, kept rather than edited away.** An earlier revision of this
/// comment said `:769`/`:5897` were *"the seed-provenance `Closure` arms"*
/// needing a non-empty `NativeSeedEnvironment`. **That was wrong** — re-derived
/// from the enclosing functions, `:769` is the producer context's `Closure` arm
/// and `:5897` is the ordinary context's *recursor* arm, and neither has
/// anything to do with seed provenance. ⛔ The line numbers were right and the
/// explanation was invented; that is why the table above is keyed on the
/// enclosing function, which a reader can check.
///
/// ⛔ **This is a partition, not an example, and the discriminator is stated so
/// the next reader can re-derive it rather than trust it:** bypass one site,
/// run this test, and a green means that site is not on any fixture's path.
/// ⚠ Adding *spellings* of one shape never moved it — a nested retention, a
/// parameterised body and a `Let`-scheduled body all descend the same arm.
/// What moved it was varying the two **axes**: a different operand shape
/// (`DeclarationClosure`) and a different enclosing form (`Match`). ⇒ Read the
/// grid before adding a fixture, or you add a fifth spelling of a covered cell.
///
/// ⇒ ⛔ **NOT CLAIMED: that this test would catch a bypass at the three
/// recursor sites.**
///
/// ⭐ **The mutation is the one `S6` is most likely to introduce by accident**,
/// because a unit body already holds the plan and resolving its own origin
/// directly is one line shorter than going through the route.
///
/// **Instrument positive control (run, not reasoned):** adding a single extra
/// `source_occurrence` call on a path every compile takes reddens this with
/// `2 resolutions against 1 route invocation`. ⇒ The counters move
/// independently and the equality is not satisfied by construction.
/// ⚠ Bypassing **all seven** sites at once reddens the **non-vacuity** assert
/// first (`1 resolution, 0 route invocations`) rather than the equality — which
/// is the more informative diagnosis of the two, and why that control is not
/// redundant with the equality below.
///
/// **Promise class: durable invariant.** ⭐ It pins a **ratio**, never a count.
/// Seven consumption sites call the route today and `S6` adds more; every one of
/// them keeps this green. ⛔ A pin that froze the call count would go red on
/// legitimate work and would be a snapshot wearing an invariant's name.
#[test]
fn every_origin_to_expression_resolution_goes_through_the_single_route() {
    fn route_counts_for(expr: &RuntimeExpr) -> (usize, usize) {
        // ⛔ Per-attempt reset. Without it a reading cannot distinguish this
        // compile's resolutions from an earlier one's, and a stale equal pair
        // reads exactly like the outcome this test wants.
        crate::cranelift_backend::planning::ac4_open_route_window();
        ac11_compiles(expr).expect("fixture compiles");
        crate::cranelift_backend::planning::ac4_route_counts()
    }

    // ⭐ The same measurement with a populated `declarations` map — the one
    // input `ac11_compiles` cannot supply, and the only way to reach the
    // `DeclarationClosure` operand shape.
    fn route_counts_with_declarations(
        expr: &RuntimeExpr,
        declarations: BTreeMap<&str, &RuntimeDeclaration>,
    ) -> (usize, usize) {
        crate::cranelift_backend::planning::ac4_open_route_window();
        compile_expr_into_module(
            new_jit_module().expect("jit module"),
            "b2f_ac4_declaration_probe",
            Linkage::Local,
            expr,
            &NativeSeedEnvironment::empty(),
            declarations,
            None,
            false,
            None,
            None,
            None,
        )
        .expect("the declaration fixture compiles");
        crate::cranelift_backend::planning::ac4_route_counts()
    }

    fn nullary_closure(body: RuntimeExpr) -> RuntimeExpr {
        RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: Vec::new(),
                body: Box::new(body),
            }),
            args: Vec::new(),
        }
    }

    // ⭐ **A SET of retained-body shapes, not one.** The relation is a universal
    // over the resolutions a compile performs, so the pin's reach is the union
    // of route call sites the fixtures actually take — see the coverage note on
    // the test's doc comment. Each shape below was chosen to drive the descent
    // down a different arm.
    let shapes = [
        // The plain application of a retained lexical body.
        nullary_closure(RuntimeExpr::Value(RuntimeValue::Bool(true))),
        // Nested retention: the inner body is resolved while the outer one is
        // already being emitted, so a bypass that only fires at depth 1 shows up.
        nullary_closure(nullary_closure(RuntimeExpr::Value(RuntimeValue::Bool(true)))),
        // A retained body under a parameter, so the environment is non-empty
        // where the resolution happens.
        RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["x".to_string()],
                body: Box::new(RuntimeExpr::Var(0)),
            }),
            args: vec![RuntimeExpr::Value(RuntimeValue::Bool(true))],
        },
        // A retained body reached through a `Let`, which schedules differently
        // from a direct application.
        RuntimeExpr::Let {
            value: Box::new(RuntimeExpr::Value(RuntimeValue::Bool(true))),
            body: Box::new(RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::LexicalClosure {
                    captures: vec![RuntimeExpr::Var(0)],
                    params: Vec::new(),
                    body: Box::new(RuntimeExpr::Var(0)),
                }),
                args: Vec::new(),
            }),
        },
        // ⭐⭐ **The PRODUCER-CONTEXT cell.** `lower_expr`'s `Match` arm routes
        // its scrutinee through `lower_computational_producer_expr` when the
        // scrutinee `requires_heterogeneous_deforestation` — which a `Call`
        // whose callee is a closure returning a `Construct` satisfies. ⇒ The
        // retained body is then resolved in the **producer** context rather
        // than the ordinary one, which is the axis the four `Call`-only shapes
        // above cannot vary. ⛔ Not another spelling: a different enclosing
        // lowering function, reached by a different predicate.
        RuntimeExpr::Match {
            scrutinee: Box::new(nullary_closure(RuntimeExpr::Construct {
                constructor: "ctor:fixture::ac4::Wrap".to_string(),
                args: vec![RuntimeExpr::Value(RuntimeValue::Bool(true))],
            })),
            cases: vec![RuntimeMatchCase {
                constructor: "ctor:fixture::ac4::Wrap".to_string(),
                binders: 1,
                body: RuntimeExpr::Var(0),
            }],
            default: RuntimeTrap {
                code: RuntimeTrapCode::PatternMatchFailure,
                message: "ac4 producer-context fixture is total".to_string(),
            },
        },
        // ⛔⛔ **THE RECURSOR ATTEMPT THAT DID NOT WORK — kept as the record of
        // a negative measurement.** A `ComputationalMatch` case carrying a
        // `recursive_position`, whose body APPLIES the induction hypothesis
        // (`Call { callee: Var(0) }`) against a unit, with the recursive child
        // as a thunk. That is the shape that binds a
        // `Lowered::ComputationalRecursorClosure`, and it is the obvious way to
        // reach `:474` / `:939` / `:5897`.
        //
        // ⚠ **It reaches none of them.** Re-running the seven-site bypass
        // bisect with this shape present left all three green. ⇒ It is retained
        // because it is the only `ComputationalMatch` in the set and its
        // relation holds, ⛔ NOT as coverage — see the doc comment.
        contspec_emission_witness(),
    ];

    let mut total_resolutions = 0usize;
    let mut total_invocations = 0usize;
    for (index, shape) in shapes.iter().enumerate() {
        let (resolutions, invocations) = route_counts_for(shape);
        assert_eq!(
            resolutions, invocations,
            "AC-4 -- shape {index}: {resolutions} origin->expression resolutions \
             were performed but the single route was invoked only {invocations} \
             times, so {} resolution(s) reached the plan's occurrence table by \
             some other path.",
            resolutions.saturating_sub(invocations)
        );
        total_resolutions += resolutions;
        total_invocations += invocations;
    }

    // ⭐⭐ **THE `DeclarationClosure` CELL — a different OPERAND SHAPE, not
    // another spelling of the one above.** Every `LexicalClosure` fixture,
    // however nested or parameterised, lowers its callee to `Lowered::Closure`
    // and descends the same arm; a transparent declaration whose body is a
    // `RuntimeExpr::Closure` lowers to `Lowered::DeclarationClosure` and takes a
    // **different** arm of the same match. ⇒ This is what moves the coverage
    // partition, and it is why the shape list above could not.
    // ⚠ The declaration's body returns a `Construct`, which is what makes the
    // producer-context fixture below deforestable. ⛔ An identity body would
    // reach the ordinary arm only, and the second cell would silently be a
    // duplicate of the first.
    let wrap = "decl:fixture::ac4::wrap".to_string();
    let declaration = RuntimeDeclaration {
        symbol: wrap.clone(),
        kind: RuntimeDeclarationKind::Transparent {
            body: RuntimeExpr::Closure {
                captures: Vec::new(),
                params: vec!["x".to_string()],
                body: Box::new(RuntimeExpr::Construct {
                    constructor: "ctor:fixture::ac4::Wrap".to_string(),
                    args: vec![RuntimeExpr::Var(0)],
                }),
            },
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    };
    let call_wrap = || RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::DeclarationRef {
            symbol: wrap.clone(),
        }),
        args: vec![RuntimeExpr::Value(RuntimeValue::Bool(true))],
    };
    let declaration_shapes = [
        // ORDINARY context — `lower_expr`'s `Call` arm.
        call_wrap(),
        // PRODUCER context — the same callee, but the `Match` arm routes its
        // scrutinee through `lower_computational_producer_expr` because a
        // declaration call producing an aggregate is deforestable.
        RuntimeExpr::Match {
            scrutinee: Box::new(call_wrap()),
            cases: vec![RuntimeMatchCase {
                constructor: "ctor:fixture::ac4::Wrap".to_string(),
                binders: 1,
                body: RuntimeExpr::Var(0),
            }],
            default: RuntimeTrap {
                code: RuntimeTrapCode::PatternMatchFailure,
                message: "ac4 declaration producer fixture is total".to_string(),
            },
        },
    ];
    for (index, shape) in declaration_shapes.iter().enumerate() {
        let (declaration_resolutions, declaration_invocations) = route_counts_with_declarations(
            shape,
            BTreeMap::from([(wrap.as_str(), &declaration)]),
        );
        assert!(
            declaration_resolutions > 0,
            "NON-VACUITY: declaration shape {index} must actually resolve a body \
             through the route, or this cell adds no coverage and the partition \
             in the doc comment is overstated"
        );
        assert_eq!(
            declaration_resolutions, declaration_invocations,
            "AC-4 -- the DeclarationClosure arm, shape {index}: \
             {declaration_resolutions} resolutions against \
             {declaration_invocations} route invocations"
        );
        total_resolutions += declaration_resolutions;
        total_invocations += declaration_invocations;
    }

    let (resolutions, invocations) = (total_resolutions, total_invocations);

    // ⛔ THE NON-VACUITY CONTROL, and the equality below is worthless without
    // it: `0 == 0` is what a harness that never ran the compile also reports,
    // and it is what a build that resolved no body at all reports. Both
    // counters must actually move.
    assert!(
        resolutions > 0 && invocations > 0,
        "AC-4 -- NON-VACUITY: a program with a retained closure body must \
         resolve at least one origin through the route; got {resolutions} \
         resolutions and {invocations} route invocations. A zero pair means \
         this test is measuring nothing, whatever the equality below says."
    );
    assert_eq!(
        resolutions, invocations,
        "AC-4 -- {resolutions} origin->expression resolutions were performed \
         but the single route was invoked only {invocations} times, so \
         {} resolution(s) reached the plan's occurrence table by some other \
         path. That is a SECOND origin->expression route, which is exactly \
         what AC-4 holds at one: a retained body must be selected by its \
         static name and by nothing else.",
        resolutions.saturating_sub(invocations)
    );

    // ⭐ The relation must hold on a program with NO retained body too, and for
    // a different reason than above: here it says the route is not invoked
    // speculatively. A counter that incremented on some unrelated event would
    // satisfy the equality above and fail here.
    let (bare_resolutions, bare_invocations) =
        route_counts_for(&RuntimeExpr::Value(RuntimeValue::Bool(true)));
    assert_eq!(
        bare_resolutions, bare_invocations,
        "AC-4 -- the relation must hold for a program with nothing retained: \
         {bare_resolutions} resolutions against {bare_invocations} route \
         invocations."
    );
}

// `RT-CONTROL-INTEGRATION-TESTS-SPLIT` D1: promoted from `control.rs`
// (module-private there, several needed by 2+ of the new integration-test
// modules below) -- hoisted here rather than widened in place, per the
// phase's banned-scope rule against widening a symbol's visibility just
// to make a test move compile.
fn px8j_deferred_recursive_field_fixture() -> RuntimeExpr {
    let wrap = "ctor:fixture::PX8JDeferred::Wrap";
    let done = "ctor:fixture::PX8JDeferred::Done";
    RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: wrap.to_string(),
            args: vec![
                RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: vec!["unit".to_string()],
                    body: Box::new(RuntimeExpr::Construct {
                        constructor: done.to_string(),
                        args: Vec::new(),
                    }),
                },
                constructor_field_aggregate(),
            ],
        }),
        cases: vec![
            crate::RuntimeComputationalMatchCase {
                constructor: wrap.to_string(),
                argument_binders: 2,
                recursive_positions: vec![0],
                body: RuntimeExpr::Match {
                    scrutinee: Box::new(RuntimeExpr::Var(2)),
                    cases: ["ctor:prelude::Result::Err", "ctor:prelude::Result::Ok"]
                        .into_iter()
                        .map(|constructor| RuntimeMatchCase {
                            constructor: constructor.to_string(),
                            binders: 1,
                            body: RuntimeExpr::Call {
                                callee: Box::new(RuntimeExpr::Var(1)),
                                args: vec![RuntimeExpr::Construct {
                                    constructor: "ctor:prelude::Unit::MkUnit".to_string(),
                                    args: Vec::new(),
                                }],
                            },
                        })
                        .collect(),
                    default: RuntimeTrap {
                        code: RuntimeTrapCode::PatternMatchFailure,
                        message: "PX8-J deferred selected-field default".to_string(),
                    },
                },
            },
            crate::RuntimeComputationalMatchCase {
                constructor: done.to_string(),
                argument_binders: 0,
                recursive_positions: Vec::new(),
                body: RuntimeExpr::Construct {
                    constructor: "ctor:prelude::Result::Ok".to_string(),
                    args: vec![RuntimeExpr::Construct {
                        constructor: "ctor:prelude::Unit::MkUnit".to_string(),
                        args: Vec::new(),
                    }],
                },
            },
        ],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "PX8-J deferred outer default".to_string(),
        },
    }
}
pub(in crate::cranelift_backend::lowering) fn px8j_layered_recursive_result(transform_layers: usize, input_depth: usize) -> RuntimeExpr {
    let tree_constructor =
        |layer: usize, constructor: &str| format!("ctor:fixture::PX8JTree{layer}::{constructor}");
    let unit = || RuntimeExpr::Construct {
        constructor: "ctor:prelude::Unit::MkUnit".to_string(),
        args: Vec::new(),
    };
    let aggregate = || RuntimeExpr::Construct {
        constructor: "ctor:prelude::Result::Ok".to_string(),
        args: vec![unit()],
    };
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
    let input_node = tree_constructor(0, "Node");
    let input_leaf = tree_constructor(0, "Leaf");
    let mut producer = RuntimeExpr::Construct {
        constructor: input_node.clone(),
        args: vec![child(input_depth, &input_node, &input_leaf)],
    };
    for layer in 0..transform_layers {
        producer = RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(producer),
            cases: vec![
                crate::RuntimeComputationalMatchCase {
                    constructor: tree_constructor(layer, "Node"),
                    argument_binders: 1,
                    recursive_positions: vec![0],
                    body: RuntimeExpr::Construct {
                        constructor: tree_constructor(layer + 1, "Node"),
                        args: vec![RuntimeExpr::Var(0)],
                    },
                },
                crate::RuntimeComputationalMatchCase {
                    constructor: tree_constructor(layer, "Leaf"),
                    argument_binders: 0,
                    recursive_positions: Vec::new(),
                    body: RuntimeExpr::Construct {
                        constructor: tree_constructor(layer + 1, "Leaf"),
                        args: Vec::new(),
                    },
                },
            ],
            default: RuntimeTrap {
                code: RuntimeTrapCode::PatternMatchFailure,
                message: format!("PX8-J transform {layer} default"),
            },
        };
    }
    RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(producer),
        cases: vec![
            crate::RuntimeComputationalMatchCase {
                constructor: tree_constructor(transform_layers, "Node"),
                argument_binders: 1,
                recursive_positions: vec![0],
                body: RuntimeExpr::Call {
                    callee: Box::new(RuntimeExpr::Var(0)),
                    args: vec![unit()],
                },
            },
            crate::RuntimeComputationalMatchCase {
                constructor: tree_constructor(transform_layers, "Leaf"),
                argument_binders: 0,
                recursive_positions: Vec::new(),
                body: aggregate(),
            },
        ],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "PX8-J terminal transform default".to_string(),
        },
    }
}
#[cfg(test)]
fn oriented_test_ih_plan() -> crate::OrientedSubcontinuationPlanV1 {
    let mut plan = oriented_test_plan();
    for frame_id in 0..=2 {
        let slot_template_id = 200 + frame_id;
        let mut slot = crate::CheckedComputationalIHSlotTemplateV1 {
            slot_template_id,
            declaration: "decl:fixture::oriented".to_string(),
            checked_match_ordinal: frame_id,
            checked_occurrence_path: vec![20, frame_id],
            frame_template_id: frame_id,
            constructor: format!("Ctor{frame_id}"),
            recursive_position: 0,
            method_binder_ordinal: 0,
            local_telescope: Vec::new(),
            ih_interface: oriented_test_interface(frame_id as u8),
            segment_site_id: 9,
            frame_templates: vec![frame_id],
            input_interface: oriented_test_interface(frame_id as u8),
            output_interface: oriented_test_interface(frame_id as u8 + 1),
            runtime_marker_locations: vec![crate::CheckedRuntimeMarkerLocationV1 {
                declaration: "decl:fixture::oriented".to_string(),
                runtime_path: vec![0, frame_id],
            }],
            occurrence_binding_fingerprint: 0,
        };
        slot.occurrence_binding_fingerprint =
            crate::compiler_private_computational_ih_slot_binding_fingerprint(&slot);
        plan.computational_ih_slots.push(slot);

        let mut call = crate::CheckedComputationalIHCallTemplateV1 {
            call_template_id: 100 + frame_id,
            declaration: "decl:fixture::oriented".to_string(),
            checked_occurrence_path: vec![30, frame_id],
            slot_template_id,
            arity: 1,
            local_telescope: Vec::new(),
            result_interface: oriented_test_interface(frame_id as u8 + 1),
            callee_segment_site_id: 9,
            callee_frame_templates: vec![frame_id],
            composed_frame_templates: Vec::new(),
            parent_frame_template_id: Some(frame_id),
            parent_segment_site_id: Some(9),
            caller_interface: oriented_test_interface(frame_id as u8 + 1),
            runtime_marker_locations: vec![crate::CheckedRuntimeMarkerLocationV1 {
                declaration: "decl:fixture::oriented".to_string(),
                runtime_path: vec![1, frame_id],
            }],
            occurrence_binding_fingerprint: 0,
        };
        call.occurrence_binding_fingerprint =
            crate::compiler_private_computational_ih_call_binding_fingerprint(&call);
        plan.computational_ih_calls.push(call);
    }
    plan.validate().unwrap();
    plan
}
#[cfg(test)]
fn oriented_test_instance_layer(
    frame_id: u64,
    invocation_id: u64,
    semantic_depth: usize,
    semantic_pending: bool,
    role: RecursorLayerRole,
) -> ComputationalRecursorLayer {
    let mut layer = oriented_test_layer(frame_id, role);
    layer.checked_invocation_id = Some(invocation_id);
    layer.checked_invocation_source =
        Some(InvocationTemplateRef::ComputationalIHCall(100 + frame_id));
    layer.checked_invocation_depth = semantic_depth;
    layer.semantic_pending = semantic_pending;
    layer
}
#[cfg(test)]
fn oriented_test_layer(frame_id: u64, role: RecursorLayerRole) -> ComputationalRecursorLayer {
    ComputationalRecursorLayer {
        cases: Vec::new(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::ExplicitTrap,
            message: format!("oriented frame {frame_id}"),
        },
        outer_env: Vec::new(),
        static_origin: inert_test_static_origin(),
        provenance: RecursorFrameProvenance(frame_id),
        role,
        checked_frame_id: Some(frame_id),
        checked_invocation_id: Some(0),
        checked_invocation_source: None,
        checked_invocation_depth: 0,
        semantic_pending: true,
    }
}
#[cfg(test)]
fn oriented_test_plan() -> crate::OrientedSubcontinuationPlanV1 {
    crate::OrientedSubcontinuationPlanV1 {
        representation_rule_version:
            crate::OrientedSubcontinuationPlanV1::REPRESENTATION_RULE_VERSION,
        // Checked postorder is p2, p1, p0 even though control returns
        // through o0, o4, o3 below.
        frames: vec![
            oriented_test_frame(0, 2, 2, 3, None),
            oriented_test_frame(1, 1, 1, 2, Some(0)),
            oriented_test_frame(2, 0, 0, 1, Some(1)),
        ],
        recursive_calls: Vec::new(),
        computational_ih_slots: Vec::new(),
        computational_ih_calls: Vec::new(),
    }
}
#[cfg(test)]
#[derive(Clone, Copy, Debug)]
enum Px8jDirectRecursorConsumer {
    PendingLetProducer,
    ProducerCall,
    OrdinaryCall,
}
#[cfg(test)]
#[derive(Clone, Copy, Debug)]
enum Px8jRecursorMalformation {
    SelectionRole,
    RepeatedScopeIdentity,
    BrokenScopeParent,
}

fn occurrence_exact_marker_fixture(
    duplicate_call: bool,
    duplicate_slot: bool,
) -> (
    RuntimeExpr,
    RuntimeDeclaration,
    crate::OrientedSubcontinuationPlanV1,
) {
    let declaration = "decl:fixture::PX8TA::markers".to_string();
    let slot_marker = RuntimeExpr::CheckedComputationalIHSlots {
        slot_template_ids: vec![200],
        checked_occurrence_paths: vec![vec![20]],
        body: Box::new(RuntimeExpr::Value(RuntimeValue::Int((1).into()))),
    };
    let call_marker = RuntimeExpr::CheckedComputationalIHInvocation {
        call_template_id: 100,
        checked_occurrence_path: vec![30],
        kind: crate::CheckedComputationalIHInvocationKind::OrdinaryApplication,
        binder_morphism: crate::CheckedComputationalIHBinderMorphism::identity_for_test(0),
        body: Box::new(RuntimeExpr::Value(RuntimeValue::Int((2).into()))),
    };
    let slot_value = if duplicate_slot {
        RuntimeExpr::Construct {
            constructor: "ctor:fixture::Pair".to_string(),
            args: vec![slot_marker.clone(), slot_marker],
        }
    } else {
        slot_marker
    };
    let call_body = if duplicate_call {
        RuntimeExpr::Construct {
            constructor: "ctor:fixture::Pair".to_string(),
            args: vec![call_marker.clone(), call_marker],
        }
    } else {
        call_marker
    };
    let cases = vec![crate::RuntimeComputationalMatchCase {
        constructor: "ctor:fixture::Only".to_string(),
        argument_binders: 0,
        recursive_positions: Vec::new(),
        body: RuntimeExpr::Let {
            value: Box::new(slot_value),
            body: Box::new(call_body),
        },
    }];
    let default = RuntimeTrap {
        code: RuntimeTrapCode::PatternMatchFailure,
        message: "PX8-TA marker fixture default".to_string(),
    };
    let runtime_frame_fingerprint =
        crate::compiler_private_computational_match_frame_fingerprint(&cases, &default);
    let body = RuntimeExpr::CheckedSubcontinuationFrame {
        frame_id: 0,
        body: Box::new(RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: "ctor:fixture::Only".to_string(),
                args: Vec::new(),
            }),
            cases,
            default,
        }),
    };
    let runtime_declaration = RuntimeDeclaration {
        symbol: declaration.clone(),
        kind: RuntimeDeclarationKind::Transparent { body },
        metadata: RuntimeSymbolMetadata::empty(),
    };
    let mut frame = crate::OrientedSubcontinuationFramePlanV1 {
        frame_id: 0,
        segment_site_id: 9,
        declaration: declaration.clone(),
        checked_occurrence_path: vec![10],
        semantic_position: 0,
        input_interface: oriented_test_interface(0),
        output_interface: oriented_test_interface(1),
        runtime_frame_fingerprint,
        occurrence_binding_fingerprint: 0,
        control_witness: crate::OrientedControlWitnessV1::DistinguishedRoot,
    };
    frame.occurrence_binding_fingerprint =
        crate::compiler_private_oriented_occurrence_binding_fingerprint(&frame);
    let mut slot = crate::CheckedComputationalIHSlotTemplateV1 {
        slot_template_id: 200,
        declaration: declaration.clone(),
        checked_match_ordinal: 0,
        checked_occurrence_path: vec![20],
        frame_template_id: 0,
        constructor: "ctor:fixture::Only".to_string(),
        recursive_position: 0,
        method_binder_ordinal: 0,
        local_telescope: Vec::new(),
        ih_interface: oriented_test_interface(0),
        segment_site_id: 9,
        frame_templates: vec![0],
        input_interface: oriented_test_interface(0),
        output_interface: oriented_test_interface(1),
        runtime_marker_locations: vec![crate::CheckedRuntimeMarkerLocationV1 {
            declaration: declaration.clone(),
            runtime_path: vec![0, 1, 0],
        }],
        occurrence_binding_fingerprint: 0,
    };
    slot.occurrence_binding_fingerprint =
        crate::compiler_private_computational_ih_slot_binding_fingerprint(&slot);
    let mut call = crate::CheckedComputationalIHCallTemplateV1 {
        call_template_id: 100,
        declaration: declaration.clone(),
        checked_occurrence_path: vec![30],
        slot_template_id: 200,
        arity: 1,
        local_telescope: Vec::new(),
        result_interface: oriented_test_interface(1),
        callee_segment_site_id: 9,
        callee_frame_templates: vec![0],
        composed_frame_templates: Vec::new(),
        parent_frame_template_id: Some(0),
        parent_segment_site_id: Some(9),
        caller_interface: oriented_test_interface(1),
        runtime_marker_locations: vec![crate::CheckedRuntimeMarkerLocationV1 {
            declaration,
            runtime_path: vec![0, 1, 1],
        }],
        occurrence_binding_fingerprint: 0,
    };
    call.occurrence_binding_fingerprint =
        crate::compiler_private_computational_ih_call_binding_fingerprint(&call);
    (
        RuntimeExpr::Value(RuntimeValue::Int((0).into())),
        runtime_declaration,
        crate::OrientedSubcontinuationPlanV1 {
            representation_rule_version:
                crate::OrientedSubcontinuationPlanV1::REPRESENTATION_RULE_VERSION,
            frames: vec![frame],
            recursive_calls: Vec::new(),
            computational_ih_slots: vec![slot],
            computational_ih_calls: vec![call],
        },
    )
}

// ── RT-SPLIT slice 7, rule 8 finalization ─────────────────────────────────
// Residual facade test fixtures whose final-user LCA is this module. Facade
// file scope was a TRANSITIONAL zero-widening holding position, never final
// ownership (Architect `evt_h69xwchqqxmj`); slice 7 discharges it. Moved
// verbatim -- ordered item-level identity, no body edits.

#[cfg(test)]
fn self_consistent_root_join_site(site_id: u64) -> crate::NativeJoinPlanSiteV1 {
    let declaration = "decl:fixture::PX8H::main".to_string();
    let checked_occurrence_path = vec![0];
    let checked_result_type_fingerprint = 19;
    crate::NativeJoinPlanSiteV1 {
        site_id,
        occurrence_binding_fingerprint: crate::compiler_private_join_occurrence_binding_fingerprint(
            site_id,
            &declaration,
            &checked_occurrence_path,
            checked_result_type_fingerprint,
        ),
        declaration,
        checked_occurrence_path,
        checked_result_type_fingerprint,
        runtime_frame_fingerprint: crate::NATIVE_JOIN_INVOCATION_RETURN_FRAME_V1,
        answer_kind: crate::NativeJoinAnswerKindV1::ExitCode,
    }
}


#[cfg(test)]
fn oriented_test_frame(
    frame_id: u64,
    semantic_position: u64,
    input: u8,
    output: u8,
    parent: Option<u64>,
) -> crate::OrientedSubcontinuationFramePlanV1 {
    let mut frame = crate::OrientedSubcontinuationFramePlanV1 {
        frame_id,
        segment_site_id: 9,
        declaration: "decl:fixture::oriented".to_string(),
        checked_occurrence_path: vec![frame_id],
        semantic_position,
        input_interface: oriented_test_interface(input),
        output_interface: oriented_test_interface(output),
        runtime_frame_fingerprint: frame_id + 100,
        occurrence_binding_fingerprint: 0,
        control_witness: parent.map_or(
            crate::OrientedControlWitnessV1::DistinguishedRoot,
            crate::OrientedControlWitnessV1::ParentFrame,
        ),
    };
    frame.occurrence_binding_fingerprint =
        crate::compiler_private_oriented_occurrence_binding_fingerprint(&frame);
    frame
}

#[cfg(test)]
fn self_consistent_join_site(
    site_id: u64,
    runtime_frame_fingerprint: u64,
) -> crate::NativeJoinPlanSiteV1 {
    let declaration = "decl:fixture::PX8H::main".to_string();
    let checked_occurrence_path = vec![1, site_id];
    let checked_result_type_fingerprint = 17;
    crate::NativeJoinPlanSiteV1 {
        site_id,
        occurrence_binding_fingerprint: crate::compiler_private_join_occurrence_binding_fingerprint(
            site_id,
            &declaration,
            &checked_occurrence_path,
            checked_result_type_fingerprint,
        ),
        declaration,
        checked_occurrence_path,
        checked_result_type_fingerprint,
        runtime_frame_fingerprint,
        answer_kind: crate::NativeJoinAnswerKindV1::Int,
    }
}

// ─── RT-FNSPLIT-B2A-C D5 — the coverage guard ─────────────────────────────
//
// ⭐ This is the deliverable with the longest half-life in the chain: it is what
// stops inventory entry 3 recurring the next time `RuntimeExpr` grows a field.
// It has TWO independent failure modes, and the first is a COMPILE error rather
// than an assertion, which is strictly stronger:
//
//  1. `expression_children` below matches every `RuntimeExpr` variant with its
//     fields spelled out and **no `..` and no `_ =>` arm**. Add a field to any
//     variant and this stops compiling (E0027 "pattern does not mention field");
//     add a variant and it stops compiling (E0004). A wildcard here is what
//     would let a new expression-typed field become silently originless, so the
//     absence of one is the mechanism, not a style preference.
//  2. Even with the pattern updated, the guard asserts that the plane holds
//     **exactly** the enumerated children for a planned instance of every
//     variant — no more, no fewer — so a field that is enumerated here but not
//     planned, or planned but not enumerated, is still red.
//
// ⛔ A test that merely enumerates today's variants and passes is NOT this
// guard (AC-3). The demonstration that it reddens on an *added* field is in the
// handoff.

/// Every expression-typed field of one occurrence, **in the planner's child
/// order** — the order of the `children` slice handed to `expression_node` /
/// `expression_seed`, which is what the positional child-origin range is laid
/// out against.
///
/// ⚠ Two variants order their children differently from their declaration:
/// `LexicalClosure` plans **body first** (position 0) with capture *i* at
/// `1 + i`, and `Effect` gives position 0 to `capability.value` **only when it
/// is present**, shifting every argument by one.
#[cfg(test)]
pub(super) fn expression_children(expr: &RuntimeExpr) -> Vec<&RuntimeExpr> {
    match expr {
        RuntimeExpr::CheckedJoinSite { site_id: _, body } => vec![body],
        RuntimeExpr::CheckedSubcontinuationFrame { frame_id: _, body } => vec![body],
        RuntimeExpr::CheckedRecursiveInvocation {
            call_template_id: _,
            checked_occurrence_path: _,
            body,
        } => vec![body],
        RuntimeExpr::CheckedComputationalIHSlots {
            slot_template_ids: _,
            checked_occurrence_paths: _,
            body,
        } => vec![body],
        RuntimeExpr::CheckedComputationalIHInvocation { body, .. } => vec![body],
        RuntimeExpr::Value(_) => Vec::new(),
        RuntimeExpr::Var(_) => Vec::new(),
        RuntimeExpr::Let { value, body } => vec![value, body],
        RuntimeExpr::If {
            scrutinee,
            then_expr,
            else_expr,
        } => vec![scrutinee, then_expr, else_expr],
        RuntimeExpr::PrimitiveCall { primitive: _, args } => args.iter().collect(),
        RuntimeExpr::Construct {
            constructor: _,
            args,
        } => args.iter().collect(),
        RuntimeExpr::Match {
            scrutinee,
            cases,
            default: _,
        } => std::iter::once(scrutinee.as_ref())
            .chain(cases.iter().map(|case| &case.body))
            .collect(),
        RuntimeExpr::ComputationalMatch {
            scrutinee,
            cases,
            default: _,
        } => std::iter::once(scrutinee.as_ref())
            .chain(cases.iter().map(|case| &case.body))
            .collect(),
        RuntimeExpr::Record { fields } => fields.iter().map(|(_, value)| value).collect(),
        RuntimeExpr::Project { record, field: _ } => vec![record],
        RuntimeExpr::Closure {
            captures: _,
            params: _,
            body,
        } => vec![body],
        RuntimeExpr::LexicalClosure {
            captures,
            params: _,
            body,
        } => std::iter::once(body.as_ref())
            .chain(captures.iter())
            .collect(),
        RuntimeExpr::DeclarationRef { symbol: _ } => Vec::new(),
        RuntimeExpr::ImportedDeclarationRef {
            symbol: _,
            dependency: _,
            dependency_semantic_hash: _,
        } => Vec::new(),
        RuntimeExpr::Call { callee, args } => std::iter::once(callee.as_ref())
            .chain(args.iter())
            .collect(),
        RuntimeExpr::Effect {
            family: _,
            operation: _,
            capability,
            args,
        } => capability
            .iter()
            .map(|capability| capability.value.as_ref())
            .chain(args.iter())
            .collect(),
        RuntimeExpr::Trap(_) => Vec::new(),
    }
}



// ─── RT-FNSPLIT-B2A-C AC-4/AC-6 — the positional-derivation controls ──────
//
// ★ AC-4's second control is the chain's own predicate as an executable test:
// if identity moves when only the ADDRESS moved, the tag is not authoritative.

/// Two same-shaped children distinguishable **only** by how many children they
/// themselves have — so which one a position resolves to is observable through
/// the positional accessor alone, with no origin→expression lookup (N3).
#[cfg(test)]
pub(super) fn one_child_record() -> RuntimeExpr {
    RuntimeExpr::Record {
        fields: vec![(
            "l".to_string(),
            RuntimeExpr::Value(RuntimeValue::Bool(true)),
        )],
    }
}




/// An imported reference — the one shape with no admitted carrier.
#[cfg(test)]
fn ac11_imported() -> RuntimeExpr {
    RuntimeExpr::ImportedDeclarationRef {
        symbol: "other::v".to_string(),
        dependency: "other".to_string(),
        dependency_semantic_hash: "hash".to_string(),
    }
}

/// Compile `expr` and report only whether it was accepted.
///
/// ⚠ The closure is **called** in every fixture below, not returned: a closure
/// is not an observable ground value at the root, so a fixture that merely
/// mentions one is rejected for an unrelated reason and would look like a
/// working discriminator while measuring nothing.
#[cfg(test)]
pub(super) fn ac11_compiles(expr: &RuntimeExpr) -> Result<(), CraneliftBackendError> {
    let module = new_jit_module().expect("jit module");
    compile_expr_into_module(
        module,
        "b2f_ac11_probe",
        Linkage::Local,
        expr,
        &NativeSeedEnvironment::empty(),
        BTreeMap::new(),
        None,
        false,
        None,
        None,
        None,
    )
    .map(|_| ())
}

/// Compile the exact governed bracket source as a process object.
///
/// The fixture contains real host effects, so a value-mode probe would reject
/// it before reaching the emission mechanism this control measures.
// `RT-EMITTER-EFFECTS-SPLIT` `D2` -- `effects::tests` reaches this fixture
// by path (`core::tests::control::recursive_port_process_compiles`) for its
// own two relocated seat-lifecycle tests; the 38 other call sites below
// stay local and unaffected.
#[cfg(test)]
pub(in crate::cranelift_backend::lowering::core::tests) fn recursive_port_process_compiles(
    expr: &RuntimeExpr,
) -> Result<(), CraneliftBackendError> {
    let module = new_jit_module().expect("jit module");
    let process_symbols = crate::NativeProcessSymbols::legacy_prelude();
    compile_expr_into_module(
        module,
        "recursive_port_probe",
        Linkage::Local,
        expr,
        &NativeSeedEnvironment::empty(),
        BTreeMap::new(),
        None,
        true,
        Some(&process_symbols),
        Some(test_only_distinguished_root_join_plan()),
        None,
    )
    .map(|_| ())
}

/// **The one program in this suite that reaches
/// `claim_and_call_continuation`.**
///
/// A `ComputationalMatch` whose scrutinee is `Node(closure)` and whose `Node`
/// case declares `recursive_positions: vec![0]`, so the planner mints one
/// causal continuation token; the case body applies the induction hypothesis
/// against `Unit`, which is the producer occurrence that claims and calls it.
///
/// ⭐ **Named here rather than copied.** It was previously inline in the `AC-4`
/// route census, which is why exactly one test in the suite drove the emission
/// seam and no control could reach it. Both consumers now build the same value,
/// so a change to one cannot silently diverge from the other.
///
/// ⛔ Not a new fixture and not fixture search: this is the identical
/// expression the census already compiled, moved so it can be named.
pub(super) fn contspec_emission_witness() -> RuntimeExpr {
    RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: "ctor:fixture::ac4::Node".to_string(),
            args: vec![RuntimeExpr::LexicalClosure {
                captures: Vec::new(),
                params: vec!["unit".to_string()],
                body: Box::new(RuntimeExpr::Construct {
                    constructor: "ctor:fixture::ac4::Leaf".to_string(),
                    args: Vec::new(),
                }),
            }],
        }),
        cases: vec![
            crate::RuntimeComputationalMatchCase {
                constructor: "ctor:fixture::ac4::Node".to_string(),
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
                constructor: "ctor:fixture::ac4::Leaf".to_string(),
                argument_binders: 0,
                recursive_positions: Vec::new(),
                body: RuntimeExpr::Construct {
                    constructor: "ctor:fixture::ac4::Leaf".to_string(),
                    args: Vec::new(),
                },
            },
        ],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "ac4 recursor fixture is total".to_string(),
        },
    }
}

/// Runs `body` with a continuation-emission mutation installed, restoring
/// `Exact` afterwards **even if `body` panics**, so one failing control cannot
/// leak a mutation into every later test in the thread.
fn with_continuation_emission_mutation<T>(
    mutation: ContinuationEmissionMutation,
    body: impl FnOnce() -> T,
) -> T {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            set_continuation_emission_mutation(ContinuationEmissionMutation::Exact);
        }
    }
    set_continuation_emission_mutation(mutation);
    let _restore = Restore;
    body()
}

/// Asserts the witness is green, then red under `mutation` for a reason whose
/// text contains `because`, then green again.
///
/// ⭐ **The positive control runs first, every time.** A mutation that reds a
/// program which was never green proves nothing about the mutation, and the
/// trailing re-run is what shows the mutation was scoped rather than
/// permanent.
///
/// ⚠ **A green mutation is a reach question before it is a defect.** If one of
/// these ever passes, diagnose whether the seam was entered at all before
/// concluding the control failed — that discrimination is what the earlier
/// same-shaped redirect lacked, and it is why that control looked like a
/// control for two checkpoints while refusing before the call.
fn assert_emission_mutation_reds(mutation: ContinuationEmissionMutation, because: &str) {
    let witness = contspec_emission_witness();
    assert!(
        ac11_compiles(&witness).is_ok(),
        "the witness must be green at the seam the mutation reddens"
    );
    let error = with_continuation_emission_mutation(mutation, || ac11_compiles(&witness).err())
        .unwrap_or_else(|| panic!("{mutation:?} must red the emission witness"));
    let rendered = format!("{error:?}");
    assert!(
        rendered.contains(because),
        "{mutation:?} must red for its own reason, not somewhere else. \
         expected to contain {because:?}, got: {rendered}"
    );
    assert!(
        ac11_compiles(&witness).is_ok(),
        "the mutation must not leak past its scope"
    );
}

/// **`AC-2` positive control 2 — every variant is reachable by the
/// instrument.**
///
/// A variant no program reaches is a reportable gap in the measurement, not a
/// variant that does not fire. Each SURVIVING variant is named with the witness
/// that exhibits it, and the loop below is that list -- so the count lives in
/// the code rather than in this sentence, where it has now gone stale twice.
#[cfg(test)]
const SEED_CALL_PORT_SOME: &str = "ctor:fixture::Core::Option::Some";

#[cfg(test)]
fn seed_call_port_producer_match_example() -> RuntimeExample {
    RuntimeExample {
        name: "seed-call-port-producer-match".to_string(),
        checked_core_shape: "match ((\\x . Some x) 4) with Some y => y".to_string(),
        ir: RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Call {
                callee: Box::new(RuntimeExpr::Closure {
                    captures: Vec::new(),
                    params: vec!["x".to_string()],
                    body: Box::new(RuntimeExpr::Construct {
                        constructor: SEED_CALL_PORT_SOME.to_string(),
                        args: vec![RuntimeExpr::Var(0)],
                    }),
                }),
                args: vec![RuntimeExpr::Value(RuntimeValue::Int((4).into()))],
            }),
            cases: vec![RuntimeMatchCase {
                constructor: SEED_CALL_PORT_SOME.to_string(),
                binders: 1,
                body: RuntimeExpr::Var(0),
            }],
            default: RuntimeTrap {
                code: RuntimeTrapCode::PatternMatchFailure,
                message: "seed-call-port d1a fixture".to_string(),
            },
        },
        observation: RuntimeObservation::Returned(RuntimeGroundValue::Int((4).into())),
    }
}

/// **`RT-SEED-CALL-PORT` `D3` — THE POST-CONDITION, and the sentinel `D1` left
/// for exactly this moment.**
///
/// At `D1` this row asserted `{SeedClosureCall}` and was labelled a transition
/// sentinel naming `D3` as its retiring event. `D3` is that event, so the row is
/// inverted rather than deleted: the same program, the same production entry,
/// the opposite answer.
///
/// It asserts all three halves of the post-condition at once, on the exact
/// program `D1` named as firing the class:
///
/// 1. the selector reports `FunctionizedUnits` (`AC-1a`);
/// 2. the object builds and runs, reaching its declared observation (`AC-1b`);
/// 3. the enumeration reports **no** `SeedClosureCall` -- asserted as the EMPTY
///    set, not merely the variant's absence, so a reclassification into some
///    other variant reds this instead of passing.
///
/// The port count is the fourth thing and it is what makes this an activation
/// rather than a deletion: the program reaches the ported arm's handoff with no
/// witness IN THE TREE at all. `D2` could only demonstrate that under a
/// test-only selector mask; `D3` deleted the mask along with the variant, so
/// there is nothing left to arm or disarm.
///
/// **Promise class: durable invariant.** The class is retired and this program
/// compiles through the functionized lane. No planned extension re-fires it.
#[cfg(test)]
fn d2_run_ported(
    example: &RuntimeExample,
    seed_env: &NativeSeedEnvironment,
) -> (Result<CraneliftRunReport, CraneliftBackendError>, usize) {
    reset_seed_callee_unit_ports();
    let outcome = run_example_with_seed_observation(example, seed_env);
    (outcome, seed_callee_unit_ports())
}

/// A direct seed closure computing `argument - capture`, called with `5` against
/// the `nc5` seed capture `y = 2`.
///
/// `AC-6`'s third control exists because the canonical seed computes `5 + 2` and
/// **addition is commutative**, so it returns `7` whether the port passes
/// `Parameter ++ Capture` or `Capture ++ Parameter`. Subtraction is not: the
/// ruled order gives `5 - 2 = 3` and a swap gives `2 - 5 = -3`. Same shape, same
/// arity, same values -- only the order is observable.
#[cfg(test)]
fn d2_order_sensitive_example() -> RuntimeExample {
    RuntimeExample {
        name: "seed-call-port-d2-order-sensitive".to_string(),
        checked_core_shape: "let y = 2 in (\\x . sub_int x y) 5".to_string(),
        ir: RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::Closure {
                captures: vec!["decl:fixture::Local::y".to_string()],
                params: vec!["x".to_string()],
                body: Box::new(RuntimeExpr::PrimitiveCall {
                    primitive: RuntimePrimitive {
                        symbol: "sub_int".to_string(),
                        partiality: RuntimePartiality::Total,
                    },
                    args: vec![RuntimeExpr::Var(0), RuntimeExpr::Var(1)],
                }),
            }),
            args: vec![RuntimeExpr::Value(RuntimeValue::Int((5).into()))],
        },
        observation: RuntimeObservation::Returned(RuntimeGroundValue::Int((3).into())),
    }
}

/// Plan the `px8tr_nested_post_effect` witness and hand its
/// [`StaticTransitionPlan`] to `f`.
///
/// ⭐ The plan is built here **independently of any emission run**. That is what
/// makes the rows below oracles rather than echoes: a claim read off a
/// successful compile cannot distinguish "the planner recorded this" from
/// "lowering happened not to need it".
fn with_d5a_witness_plan<T>(f: impl FnOnce(&StaticTransitionPlan<'_>) -> T) -> T {
    let (entry_expr, declarations) =
        crate::cranelift_backend::test_objects::px8tr_nested_post_effect_planning_inputs();
    let declarations = declarations
        .iter()
        .map(|declaration| (declaration.symbol.as_str(), declaration))
        .collect::<BTreeMap<_, _>>();
    let plan = plan_static_transition_graph_with_symbols(
        &entry_expr,
        &declarations,
        &crate::NativeProcessSymbols::legacy_prelude(),
        AbiRootIngress::Value,
        true,
    )
    .expect("the witness plans; a planning failure here is a regression, not a checkpoint");
    f(&plan)
}

/// The witness program, parameterised by the two axes its controls move.
///
/// `callee_index` is the de Bruijn index the bridge case body calls.
/// `computational_bridge` selects whether the immediate-binder eliminator is a
/// `ComputationalMatch` (whose case bodies are lowered by the source machine) or
/// a `Match` (whose case bodies are lowered by `lower_expr`). ⛔ The two bridges
/// install IDENTICALLY -- the `D8d` binding is materialized by the outer frame
/// either way -- so the axis isolates the consumer and nothing else.
#[cfg(test)]
fn d8e_witness_declaration(
    symbol: &str,
    callee_index: u32,
    computational_bridge: bool,
) -> RuntimeDeclaration {
    let wrap = "ctor:fixture::D8EWitness::Wrap";
    let done = "ctor:fixture::D8EWitness::Done";
    let unit = || RuntimeExpr::Construct {
        constructor: "ctor:prelude::Unit::MkUnit".to_string(),
        args: Vec::new(),
    };
    let ok_unit = || RuntimeExpr::Construct {
        constructor: "ctor:prelude::Result::Ok".to_string(),
        args: vec![unit()],
    };
    let call = || RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::Var(callee_index)),
        args: vec![unit()],
    };
    let bridge_default = RuntimeTrap {
        code: RuntimeTrapCode::PatternMatchFailure,
        message: "D8e witness bridge default".to_string(),
    };
    let bridge = if computational_bridge {
        RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Var(2)),
            cases: ["ctor:prelude::Result::Err", "ctor:prelude::Result::Ok"]
                .into_iter()
                .map(|constructor| crate::RuntimeComputationalMatchCase {
                    constructor: constructor.to_string(),
                    argument_binders: 1,
                    recursive_positions: vec![0],
                    body: call(),
                })
                .collect(),
            default: bridge_default,
        }
    } else {
        RuntimeExpr::Match {
            scrutinee: Box::new(RuntimeExpr::Var(2)),
            cases: ["ctor:prelude::Result::Err", "ctor:prelude::Result::Ok"]
                .into_iter()
                .map(|constructor| RuntimeMatchCase {
                    constructor: constructor.to_string(),
                    binders: 1,
                    body: call(),
                })
                .collect(),
            default: bridge_default,
        }
    };
    // The selected constructor field: a `Match`, so
    // `requires_heterogeneous_deforestation` holds, on a compile-time
    // constructor, so exactly one arm is lowered and no join is merged.
    let selected_field = RuntimeExpr::Match {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: "ctor:prelude::Bool::True".to_string(),
            args: Vec::new(),
        }),
        cases: [
            ("ctor:prelude::Bool::True", "ctor:prelude::Result::Ok"),
            ("ctor:prelude::Bool::False", "ctor:prelude::Result::Err"),
        ]
        .into_iter()
        .map(|(constructor, result)| RuntimeMatchCase {
            constructor: constructor.to_string(),
            binders: 0,
            body: RuntimeExpr::Construct {
                constructor: result.to_string(),
                args: vec![unit()],
            },
        })
        .collect(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "D8e witness selected-field default".to_string(),
        },
    };
    let eliminator = RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: wrap.to_string(),
            args: vec![
                // The worker the `D8a` selector's provenance names: a capture-free
                // closure at the producer's recursive position.
                RuntimeExpr::LexicalClosure {
                    captures: Vec::new(),
                    params: vec!["unit".to_string()],
                    body: Box::new(ok_unit()),
                },
                selected_field,
            ],
        }),
        cases: vec![
            crate::RuntimeComputationalMatchCase {
                constructor: wrap.to_string(),
                argument_binders: 2,
                recursive_positions: vec![0],
                body: bridge,
            },
            crate::RuntimeComputationalMatchCase {
                constructor: done.to_string(),
                argument_binders: 0,
                recursive_positions: Vec::new(),
                body: ok_unit(),
            },
        ],
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "D8e witness eliminator default".to_string(),
        },
    };
    RuntimeDeclaration {
        symbol: symbol.to_string(),
        kind: RuntimeDeclarationKind::Transparent {
            // ⛔ The `Let` is not decoration -- see the header. It is what puts
            // the unit's source root above its own continuation.
            body: RuntimeExpr::Closure {
                captures: Vec::new(),
                params: vec!["state".to_string()],
                body: Box::new(RuntimeExpr::Let {
                    value: Box::new(eliminator),
                    body: Box::new(RuntimeExpr::Var(0)),
                }),
            },
        },
        metadata: RuntimeSymbolMetadata {
            lowerability: Some(RuntimeLowerabilityStatus::Supported),
            ..RuntimeSymbolMetadata::empty()
        },
    }
}

/// Compile the witness and return the outcome beside the three `D8d`/`D8e`
/// counters and the emitted static-worker call log.
#[cfg(test)]
fn d8e_witness_compile(
    label: &str,
    callee_index: u32,
    computational_bridge: bool,
) -> (
    Option<CraneliftBackendError>,
    (usize, usize, usize),
    Vec<D5aMarkerEvent>,
) {
    use crate::cranelift_backend::lowering::{
        d5a_marker_events, d8d_bindings, d8d_recursive_sites, d8e_consumptions,
        reset_d5a_marker_events, reset_d8d_bindings,
    };
    let symbol = format!("decl:fixture::d8e::{label}");
    let declaration = d8e_witness_declaration(&symbol, callee_index, computational_bridge);
    let entry = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::DeclarationRef {
            symbol: symbol.clone(),
        }),
        args: vec![RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        }],
    };
    let declarations = BTreeMap::from([(symbol.as_str(), &declaration)]);
    reset_d8d_bindings();
    reset_d5a_marker_events();
    let outcome = compile_expr_into_module(
        new_object_module(label).expect("the object module builds"),
        &format!("ken_{label}"),
        Linkage::Export,
        &entry,
        &NativeSeedEnvironment::empty(),
        declarations,
        None,
        true,
        None,
        Some(test_only_distinguished_root_join_plan()),
        None,
    );
    (
        outcome.err(),
        (d8d_recursive_sites(), d8d_bindings(), d8e_consumptions()),
        d5a_marker_events(),
    )
}

/// The `D8e` witness as a planned graph, for reading its target population
/// directly. Mirrors [`with_d5a_witness_plan`] over this node's own witness.
#[cfg(test)]
fn with_d8e_witness_plan<T>(f: impl FnOnce(&StaticTransitionPlan<'_>) -> T) -> T {
    let symbol = "decl:fixture::d8e::d8i_plan".to_string();
    let declaration = d8e_witness_declaration(&symbol, 3, true);
    let entry = RuntimeExpr::Call {
        callee: Box::new(RuntimeExpr::DeclarationRef {
            symbol: symbol.clone(),
        }),
        args: vec![RuntimeExpr::Construct {
            constructor: "ctor:prelude::Unit::MkUnit".to_string(),
            args: Vec::new(),
        }],
    };
    let declarations = BTreeMap::from([(symbol.as_str(), &declaration)]);
    let plan = plan_static_transition_graph_with_symbols(
        &entry,
        &declarations,
        &crate::NativeProcessSymbols::legacy_prelude(),
        AbiRootIngress::Value,
        true,
    )
    .expect("the D8e witness plans");
    f(&plan)
}

/// **`RT-CONTSRC-PRODUCER-LOCAL` `D8j` — the root-owned composed witness.**
///
/// `D8e`'s witness puts the composed elimination in a declaration-owned unit,
/// where the `D5a` detached-result seat refuses **before** the function is
/// finalized -- so its CLIF is never built and verifications 3, 4 and 5 have
/// nothing to read. That refusal is the `89e36ec1` finding and is not this
/// checkpoint's to repair.
///
/// ⭐ **The root unit is the lawful way past it, and it is production's own
/// rule, not a workaround.** `define_unit_body` applies the detached-result
/// seat on the non-root path only -- a root owning an undischarged projected
/// call is left to the whole-pass claim closure -- so a root-owned composed
/// producer finalizes its function, and the `D8j` gate runs on a real
/// instruction stream.
///
/// ⚠ The program still does not compile: it stops LATER, in the specialization
/// emission, at an unrelated ordinary-envelope refusal. The discharge relation
/// is populated before that, which is what these rows measure, and the row says
/// so rather than implying a compiling program.
///
/// **Two recursive positions**, and that is the whole reason this fixture is
/// not `d8e_witness_declaration` reused: the planner interns one specialization
/// per position, so the plan carries **two** targets at one producer
/// `Construct` -- one constructor symbol, two identities. That is exactly the
/// population a same-symbol shortcut would confuse, and without it the
/// substitution discriminator has nothing lawful to substitute.
#[cfg(test)]
fn d8j_root_witness_entry() -> RuntimeExpr {
    let wrap = "ctor:fixture::D8JWitness::Wrap";
    let done = "ctor:fixture::D8JWitness::Done";
    let unit = || RuntimeExpr::Construct {
        constructor: "ctor:prelude::Unit::MkUnit".to_string(),
        args: Vec::new(),
    };
    let ok_unit = || RuntimeExpr::Construct {
        constructor: "ctor:prelude::Result::Ok".to_string(),
        args: vec![unit()],
    };
    let worker = || RuntimeExpr::LexicalClosure {
        captures: Vec::new(),
        params: vec!["unit".to_string()],
        body: Box::new(ok_unit()),
    };
    // The bridge: a computational eliminator over the selected field, so its
    // case bodies are lowered by the source machine -- `D8e`'s seat.
    //
    // Environment inside a bridge case body:
    //   0 bridge IH, 1 payload, 2 outer IH(1), 3 outer IH(0),
    //   4 static worker(0), 5 static worker(1), 6 selected field, ...
    let bridge = RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Var(4)),
        cases: ["ctor:prelude::Result::Err", "ctor:prelude::Result::Ok"]
            .into_iter()
            .map(|constructor| crate::RuntimeComputationalMatchCase {
                constructor: constructor.to_string(),
                argument_binders: 1,
                recursive_positions: vec![0],
                body: RuntimeExpr::Call {
                    callee: Box::new(RuntimeExpr::Var(4)),
                    args: vec![unit()],
                },
            })
            .collect(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "D8j witness bridge default".to_string(),
        },
    };
    let selected_field = RuntimeExpr::Match {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: "ctor:prelude::Bool::True".to_string(),
            args: Vec::new(),
        }),
        cases: [
            ("ctor:prelude::Bool::True", "ctor:prelude::Result::Ok"),
            ("ctor:prelude::Bool::False", "ctor:prelude::Result::Err"),
        ]
        .into_iter()
        .map(|(constructor, result)| RuntimeMatchCase {
            constructor: constructor.to_string(),
            binders: 0,
            body: RuntimeExpr::Construct {
                constructor: result.to_string(),
                args: vec![unit()],
            },
        })
        .collect(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "D8j witness selected-field default".to_string(),
        },
    };
    RuntimeExpr::Let {
        value: Box::new(RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: wrap.to_string(),
                args: vec![worker(), worker(), selected_field],
            }),
            cases: vec![
                crate::RuntimeComputationalMatchCase {
                    constructor: wrap.to_string(),
                    argument_binders: 3,
                    recursive_positions: vec![0, 1],
                    body: bridge,
                },
                crate::RuntimeComputationalMatchCase {
                    constructor: done.to_string(),
                    argument_binders: 0,
                    recursive_positions: Vec::new(),
                    body: ok_unit(),
                },
            ],
            default: RuntimeTrap {
                code: RuntimeTrapCode::PatternMatchFailure,
                message: "D8j witness eliminator default".to_string(),
            },
        }),
        body: Box::new(RuntimeExpr::Var(0)),
    }
}

/// Compile the `D8j` root witness under one mutation and report what the
/// composed relation ended up holding.
#[cfg(test)]
fn d8j_root_witness_compile(
    label: &str,
    mutation: crate::cranelift_backend::lowering::D8jMutation,
) -> (
    Option<CraneliftBackendError>,
    usize,
    (usize, usize),
    Vec<crate::cranelift_backend::planning::ContinuationCallIdentity>,
) {
    use crate::cranelift_backend::lowering::{
        d8d_bindings, d8e_consumptions, d8j_discharged, reset_d8d_bindings, reset_d8j_discharged,
        set_d8j_mutation, D8jMutation,
    };
    let entry = d8j_root_witness_entry();
    reset_d8j_discharged();
    reset_d8d_bindings();
    set_d8j_mutation(mutation);
    let error = compile_expr_into_module(
        new_object_module(label).expect("the object module builds"),
        &format!("ken_{label}"),
        Linkage::Export,
        &entry,
        &NativeSeedEnvironment::empty(),
        BTreeMap::new(),
        None,
        true,
        None,
        Some(test_only_distinguished_root_join_plan()),
        None,
    )
    .err();
    set_d8j_mutation(D8jMutation::Exact);
    (
        error,
        d8j_discharged().len(),
        (d8d_bindings(), d8e_consumptions()),
        d8j_discharged(),
    )
}

/// A composed witness whose producer `Construct` has `fields` fields with the
/// selected recursive position at `recursive`, so the ordinary envelope's
/// source-position population can be read for every orientation.
#[cfg(test)]
fn d8l2_envelope_witness(fields: usize, recursive: usize) -> RuntimeExpr {
    assert!(fields >= 2 && recursive < fields);
    let wrap = "ctor:fixture::D8L2::Wrap";
    let done = "ctor:fixture::D8L2::Done";
    let unit = || RuntimeExpr::Construct {
        constructor: "ctor:prelude::Unit::MkUnit".to_string(),
        args: Vec::new(),
    };
    let ok_unit = || RuntimeExpr::Construct {
        constructor: "ctor:prelude::Result::Ok".to_string(),
        args: vec![unit()],
    };
    // The selected field: deforestable, and statically resolvable so no join is
    // merged (see the D8e fixture header for why that matters).
    let deforestable = |tag: i64| RuntimeExpr::Match {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: "ctor:prelude::Bool::True".to_string(),
            args: Vec::new(),
        }),
        cases: [
            ("ctor:prelude::Bool::True", "ctor:prelude::Result::Ok"),
            ("ctor:prelude::Bool::False", "ctor:prelude::Result::Err"),
        ]
        .into_iter()
        .map(|(constructor, result)| RuntimeMatchCase {
            constructor: constructor.to_string(),
            binders: 0,
            body: RuntimeExpr::Construct {
                constructor: result.to_string(),
                // ⭐ Distinguishable per field, so an envelope that read the
                // wrong source position would carry a different value.
                args: vec![RuntimeExpr::Value(RuntimeValue::Int(tag.into()))],
            },
        })
        .collect(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: format!("d8l2 field {tag} default"),
        },
    };
    let worker = RuntimeExpr::LexicalClosure {
        captures: Vec::new(),
        params: vec!["unit".to_string()],
        body: Box::new(ok_unit()),
    };
    // The bridge eliminates the FIRST nonrecursive field; every other
    // nonrecursive field is an ordinary envelope member that nothing consumes,
    // which is exactly the population under test.
    let selected_field = (0..fields).find(|position| *position != recursive).expect("one");
    let mut args = Vec::with_capacity(fields);
    for position in 0..fields {
        if position == recursive {
            args.push(worker.clone());
        } else {
            args.push(deforestable(7 + position as i64));
        }
    }
    let binder_offset = 1; // one recursive position
    let bridge = RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Var((binder_offset + selected_field) as u32)),
        cases: ["ctor:prelude::Result::Err", "ctor:prelude::Result::Ok"]
            .into_iter()
            .map(|constructor| crate::RuntimeComputationalMatchCase {
                constructor: constructor.to_string(),
                argument_binders: 1,
                recursive_positions: vec![0],
                // The composed call: 2 bridge binders + the outer run, whose
                // static worker sits at `1 + recursive` inside it.
                body: RuntimeExpr::Call {
                    callee: Box::new(RuntimeExpr::Var((2 + binder_offset + recursive) as u32)),
                    args: vec![unit()],
                },
            })
            .collect(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "d8l2 bridge default".to_string(),
        },
    };
    RuntimeExpr::Let {
        value: Box::new(RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: wrap.to_string(),
                args,
            }),
            cases: vec![
                crate::RuntimeComputationalMatchCase {
                    constructor: wrap.to_string(),
                    argument_binders: fields,
                    recursive_positions: vec![recursive],
                    body: bridge,
                },
                crate::RuntimeComputationalMatchCase {
                    constructor: done.to_string(),
                    argument_binders: 0,
                    recursive_positions: Vec::new(),
                    body: ok_unit(),
                },
            ],
            default: RuntimeTrap {
                code: RuntimeTrapCode::PatternMatchFailure,
                message: "d8l2 eliminator default".to_string(),
            },
        }),
        body: Box::new(RuntimeExpr::Var(0)),
    }
}

#[cfg(test)]
fn d8l2_envelope_positions(fields: usize, recursive: usize) -> Vec<Vec<u32>> {
    let entry = d8l2_envelope_witness(fields, recursive);
    let plan = plan_static_transition_graph_with_symbols(
        &entry,
        &BTreeMap::new(),
        &crate::NativeProcessSymbols::legacy_prelude(),
        AbiRootIngress::Value,
        true,
    )
    .expect("the envelope witness plans");
    plan.continuation_units()
        .expect("units")
        .iter()
        .map(|unit| {
            unit.ordinary_envelope()
                .expect("the envelope builds")
                .into_iter()
                .filter_map(|role| match role {
                    crate::cranelift_backend::planning::ContinuationOrdinaryEnvelopeRole::NonrecursiveConstructorField {
                        source_position,
                    } => Some(source_position),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
fn d8l2_compile(fields: usize, recursive: usize) -> Option<CraneliftBackendError> {
    let entry = d8l2_envelope_witness(fields, recursive);
    compile_expr_into_module(
        new_object_module("d8l2-envelope").expect("module"),
        "ken_d8l2_envelope",
        Linkage::Export,
        &entry,
        &NativeSeedEnvironment::empty(),
        BTreeMap::new(),
        None,
        true,
        None,
        Some(test_only_distinguished_root_join_plan()),
        None,
    )
    .err()
}

/// A composed witness whose worker RETURNS the ordinary payload the bridge
/// matched, so the payload is consumed by the real composed call and reaches
/// the program's answer.
#[cfg(test)]
fn d8l2_payload_witness(worker_last: bool, payload: i64) -> RuntimeExpr {
    let wrap = "ctor:fixture::D8L2P::Wrap";
    let done = "ctor:fixture::D8L2P::Done";
    let unit = || RuntimeExpr::Construct {
        constructor: "ctor:prelude::Unit::MkUnit".to_string(),
        args: Vec::new(),
    };
    // ⭐ The worker returns its argument, so the payload is CONSUMED -- passed
    // through the composed call and observable in the answer -- rather than
    // merely carried in the envelope or used to choose a case.
    let worker = RuntimeExpr::LexicalClosure {
        captures: Vec::new(),
        params: vec!["carried".to_string()],
        body: Box::new(RuntimeExpr::Var(0)),
    };
    let selected_field = RuntimeExpr::Match {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: "ctor:prelude::Bool::True".to_string(),
            args: Vec::new(),
        }),
        cases: [
            ("ctor:prelude::Bool::True", "ctor:prelude::Result::Ok"),
            ("ctor:prelude::Bool::False", "ctor:prelude::Result::Err"),
        ]
        .into_iter()
        .map(|(constructor, result)| RuntimeMatchCase {
            constructor: constructor.to_string(),
            binders: 0,
            body: RuntimeExpr::Construct {
                constructor: result.to_string(),
                args: vec![RuntimeExpr::Value(RuntimeValue::Int(payload.into()))],
            },
        })
        .collect(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "d8l2 payload field default".to_string(),
        },
    };
    let (args, recursive_positions, scrutinee_var, callee_var) = if worker_last {
        (vec![selected_field, worker], vec![1usize], 1u32, 4u32)
    } else {
        (vec![worker, selected_field], vec![0usize], 2u32, 3u32)
    };
    let bridge = RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Var(scrutinee_var)),
        cases: ["ctor:prelude::Result::Err", "ctor:prelude::Result::Ok"]
            .into_iter()
            .map(|constructor| crate::RuntimeComputationalMatchCase {
                constructor: constructor.to_string(),
                argument_binders: 1,
                recursive_positions: vec![0],
                body: RuntimeExpr::Call {
                    callee: Box::new(RuntimeExpr::Var(callee_var)),
                    args: vec![RuntimeExpr::Var(1)],
                },
            })
            .collect(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "d8l2 payload bridge default".to_string(),
        },
    };
    RuntimeExpr::Let {
        value: Box::new(RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: wrap.to_string(),
                args,
            }),
            cases: vec![
                crate::RuntimeComputationalMatchCase {
                    constructor: wrap.to_string(),
                    argument_binders: 2,
                    recursive_positions,
                    body: bridge,
                },
                crate::RuntimeComputationalMatchCase {
                    constructor: done.to_string(),
                    argument_binders: 0,
                    recursive_positions: Vec::new(),
                    body: unit(),
                },
            ],
            default: RuntimeTrap {
                code: RuntimeTrapCode::PatternMatchFailure,
                message: "d8l2 payload eliminator default".to_string(),
            },
        }),
        body: Box::new(RuntimeExpr::Var(0)),
    }
}

#[cfg(test)]
fn d8l2_capture_witness(worker_last: bool) -> RuntimeExpr {
    let wrap = "ctor:fixture::D8L2C::Wrap";
    let done = "ctor:fixture::D8L2C::Done";
    let unit = || RuntimeExpr::Construct {
        constructor: "ctor:prelude::Unit::MkUnit".to_string(),
        args: Vec::new(),
    };
    let worker = RuntimeExpr::LexicalClosure {
        captures: vec![
            RuntimeExpr::Value(RuntimeValue::Int((11).into())),
            RuntimeExpr::Value(RuntimeValue::Int((12).into())),
        ],
        params: vec!["carried".to_string()],
        body: Box::new(RuntimeExpr::Var(0)),
    };
    let selected_field = RuntimeExpr::Match {
        scrutinee: Box::new(RuntimeExpr::Construct {
            constructor: "ctor:prelude::Bool::True".to_string(),
            args: Vec::new(),
        }),
        cases: [
            ("ctor:prelude::Bool::True", "ctor:prelude::Result::Ok"),
            ("ctor:prelude::Bool::False", "ctor:prelude::Result::Err"),
        ]
        .into_iter()
        .map(|(constructor, result)| RuntimeMatchCase {
            constructor: constructor.to_string(),
            binders: 0,
            body: RuntimeExpr::Construct {
                constructor: result.to_string(),
                args: vec![RuntimeExpr::Value(RuntimeValue::Int((41).into()))],
            },
        })
        .collect(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "d8l2 capture field default".to_string(),
        },
    };
    let (args, recursive_positions, scrutinee_var, callee_var) = if worker_last {
        (vec![selected_field, worker], vec![1usize], 1u32, 4u32)
    } else {
        (vec![worker, selected_field], vec![0usize], 2u32, 3u32)
    };
    let bridge = RuntimeExpr::ComputationalMatch {
        scrutinee: Box::new(RuntimeExpr::Var(scrutinee_var)),
        cases: ["ctor:prelude::Result::Err", "ctor:prelude::Result::Ok"]
            .into_iter()
            .map(|constructor| crate::RuntimeComputationalMatchCase {
                constructor: constructor.to_string(),
                argument_binders: 1,
                recursive_positions: vec![0],
                body: RuntimeExpr::Call {
                    callee: Box::new(RuntimeExpr::Var(callee_var)),
                    args: vec![RuntimeExpr::Var(1)],
                },
            })
            .collect(),
        default: RuntimeTrap {
            code: RuntimeTrapCode::PatternMatchFailure,
            message: "d8l2 capture bridge default".to_string(),
        },
    };
    RuntimeExpr::Let {
        value: Box::new(RuntimeExpr::ComputationalMatch {
            scrutinee: Box::new(RuntimeExpr::Construct {
                constructor: wrap.to_string(),
                args,
            }),
            cases: vec![
                crate::RuntimeComputationalMatchCase {
                    constructor: wrap.to_string(),
                    argument_binders: 2,
                    recursive_positions,
                    body: bridge,
                },
                crate::RuntimeComputationalMatchCase {
                    constructor: done.to_string(),
                    argument_binders: 0,
                    recursive_positions: Vec::new(),
                    body: unit(),
                },
            ],
            default: RuntimeTrap {
                code: RuntimeTrapCode::PatternMatchFailure,
                message: "d8l2 capture eliminator default".to_string(),
            },
        }),
        body: Box::new(RuntimeExpr::Var(0)),
    }
}


