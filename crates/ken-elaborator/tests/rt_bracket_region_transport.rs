//! The checked acquire and finalizer reach Runtime IR by exact source-node
//! identity, not by host operations or enclosing-Vis resemblance.

use ken_elaborator::compiler_driver::{
    prepare_native_program_sources, CompilerSource, NativeProgramPreparationV1,
};
use ken_runtime::{
    CheckedBracketMarkRole, CheckedBracketRegionMarker, RuntimeDeclarationKind, RuntimeExpr,
};

fn prepare(source: &str) -> NativeProgramPreparationV1 {
    prepare_native_program_sources(
        "rt-bracket-region-transport",
        vec![CompilerSource::new("region.ken", source)],
    )
    .unwrap_or_else(|error| panic!("checked bracket preparation: {error:?}"))
}

fn visit<'a>(expr: &'a RuntimeExpr, markers: &mut Vec<&'a CheckedBracketRegionMarker>) {
    match expr {
        RuntimeExpr::CheckedBracketFinalizer { region, body } => {
            assert!(
                matches!(body.as_ref(), RuntimeExpr::Construct { .. }),
                "a bracket marker must directly wrap the finalizer Construct"
            );
            markers.push(region);
            visit(body, markers);
        }
        RuntimeExpr::CheckedJoinSite { body, .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { body, .. }
        | RuntimeExpr::CheckedRecursiveInvocation { body, .. }
        | RuntimeExpr::CheckedComputationalIHSlots { body, .. }
        | RuntimeExpr::CheckedComputationalIHInvocation { body, .. }
        | RuntimeExpr::Project { record: body, .. }
        | RuntimeExpr::Closure { body, .. } => visit(body, markers),
        RuntimeExpr::LexicalClosure { captures, body, .. } => {
            for capture in captures {
                visit(capture, markers);
            }
            visit(body, markers);
        }
        RuntimeExpr::Let { value, body } => {
            visit(value, markers);
            visit(body, markers);
        }
        RuntimeExpr::If {
            scrutinee,
            then_expr,
            else_expr,
        } => {
            visit(scrutinee, markers);
            visit(then_expr, markers);
            visit(else_expr, markers);
        }
        RuntimeExpr::Effect { capability, args, bracket_acquire, .. } => {
            if let Some(mark) = bracket_acquire {
                assert_eq!(mark.role, CheckedBracketMarkRole::Acquire);
                markers.push(mark);
            }
            if let Some(capability) = capability {
                visit(&capability.value, markers);
            }
            for arg in args {
                visit(arg, markers);
            }
        }
        RuntimeExpr::PrimitiveCall { args, .. }
        | RuntimeExpr::Construct { args, .. } => {
            for arg in args {
                visit(arg, markers);
            }
        }
        RuntimeExpr::Call { callee, args } => {
            visit(callee, markers);
            for arg in args {
                visit(arg, markers);
            }
        }
        RuntimeExpr::Match {
            scrutinee, cases, ..
        } => {
            visit(scrutinee, markers);
            for case in cases {
                visit(&case.body, markers);
            }
        }
        RuntimeExpr::ComputationalMatch {
            scrutinee, cases, ..
        } => {
            visit(scrutinee, markers);
            for case in cases {
                visit(&case.body, markers);
            }
        }
        RuntimeExpr::Record { fields } => {
            for (_, field) in fields {
                visit(field, markers);
            }
        }
        RuntimeExpr::Value(_)
        | RuntimeExpr::Var(_)
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. }
        | RuntimeExpr::Trap(_) => {}
    }
}

fn verify_exact_binding(source: &str, expected_regions: usize) {
    let prepared = prepare(source);
    let regions = prepared.bracket_regions();
    assert_eq!(regions.len(), expected_regions);
    let mut markers = Vec::new();
    for declaration in &prepared.runtime_program().declarations {
        if let RuntimeDeclarationKind::Transparent { body } = &declaration.kind {
            visit(body, &mut markers);
        }
    }
    assert_eq!(
        markers.len(),
        regions.iter().map(|region| 1 + region.finalizer_paths.len()).sum::<usize>(),
        "each region must have one acquire and every checked finalizer marked"
    );
    for region in regions {
        let key = |marker: &&CheckedBracketRegionMarker| {
            marker.owner == region.owner.to_string()
                && marker.ok_resource_binder_path == region.ok_resource_binder_path
        };
        let acquires: Vec<_> = markers.iter().copied().filter(|marker| key(marker)).filter(|marker| {
            marker.role == CheckedBracketMarkRole::Acquire
        }).collect();
        let finalizers: Vec<_> = markers.iter().copied().filter(|marker| key(marker)).filter(|marker| {
            marker.role == CheckedBracketMarkRole::Finalizer
        }).collect();
        assert_eq!(acquires.len(), 1, "one acquire per Ok binder: {region:?}");
        assert_eq!(finalizers.len(), region.finalizer_paths.len());
        assert_eq!(acquires[0].constructor_node_id, region.acquire_constructor_node_id);
        for (index, marker) in finalizers.iter().enumerate() {
            assert_eq!(marker.constructor_node_id, region.finalizer_constructor_node_ids[index]);
            assert_eq!(marker.finalizer_path, region.finalizer_paths[index]);
        }
        for marker in acquires.iter().chain(finalizers.iter()) {
            assert_eq!(marker.acquire_vis_path, region.acquire_vis_path);
            assert_eq!(marker.acquire_response_binder_path, region.acquire_response_binder_path);
            assert_eq!(marker.ok_resource_binder_path, region.ok_resource_binder_path);
        }
    }
}

/// Promise class: durable exact region-to-private-Constructor binding over the
/// route-C resource, lookalike, early-release and nested source witnesses.
/// Counts are lexical bracket binders, not dynamic Ret-leaf counts.
#[test]
fn source_brackets_bind_by_checked_node_not_public_release_host_op() {
    verify_exact_binding(
        include_str!("../../ken-cli/tests/fixtures/rt_bracket_body_once_resource.ken"),
        1,
    );
    verify_exact_binding(
        include_str!("../../ken-cli/tests/fixtures/rt_bracket_body_once_lookalike.ken"),
        1,
    );
    verify_exact_binding(
        include_str!("fixtures/rt_bracket_region_early_release.ken"),
        1,
    );
    let multileaf = include_str!("fixtures/rt_bracket_region_multileaf.ken");
    for (body, count) in [
        ("(\\resource. leaf_body terminal resource)", 1),
        ("(\\resource. level_one_body cap terminal resource)", 2),
        ("(\\resource. level_two_body cap terminal resource)", 3),
    ] {
        verify_exact_binding(&multileaf.replace("__ROOT_BODY__", body), count);
    }
}

/// Promise class: durable preorder binding on a continuation shared by two
/// dynamic Ret leaves. A +1 mutation to the collector's constructor-head id
/// must refuse this exact source at erasure, not silently choose a neighbour.
#[test]
fn multileaf_finalizer_binds_at_its_exact_checked_preorder_node() {
    let source = include_str!("fixtures/rt_bracket_region_multileaf.ken")
        .replace("__ROOT_BODY__", "(\\resource. leaf_body terminal resource)");
    verify_exact_binding(&source, 1);
}

/// Promise class: durable equivalence across FS handle, buffer, anonymous
/// mapping and mapping nested inside an FS acquisition. The marker still names
/// the binder on its own checked resource kind rather than the outer owner.
#[test]
fn buffer_and_mapping_acquisitions_carry_separate_checked_marker_regions() {
    verify_exact_binding(
        include_str!("../../ken-cli/tests/fixtures/rt_bracket_body_once_buffer.ken"),
        1,
    );
    verify_exact_binding(include_str!("fixtures/rt_bracket_transport_mapping.ken"), 1);
    verify_exact_binding(
        include_str!("fixtures/rt_bracket_transport_file_backed.ken"),
        2,
    );
}
