//! Bracket roles are extracted only from normalized checked constructor and
//! binder structure, not from runtime host-operation resemblance.

use ken_elaborator::compiler_driver::{prepare_native_program_sources, CompilerSource};

fn regions(source: &str) -> Vec<ken_elaborator::compiler_driver::NormalizedBracketRegion> {
    prepare_native_program_sources("rt-bracket-region", vec![CompilerSource::new("region.ken", source)])
        .unwrap_or_else(|err| panic!("checked bracket preparation: {err:?}"))
        .bracket_regions()
        .to_vec()
}

/// Promise class: durable non-forgery invariant. The one canonical bracket
/// is selected, while console effects in the same checked declaration do not
/// create additional regions.
#[test]
fn canonical_bracket_with_console_lookalike_has_one_region() {
    let rows = regions(include_str!("../../ken-cli/tests/fixtures/rt_bracket_body_once_lookalike.ken"));
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].finalizer_paths.len(), 1);
    assert_ne!(rows[0].acquire_response_binder_path, rows[0].ok_resource_binder_path);
}

/// Promise class: durable distinction between ordinary body work and bracket
/// finalization. The body performs two public `release` calls on the same
/// resource; only the finalizer's private constructor is a region port.
#[test]
fn two_public_early_releases_do_not_become_finalizer_ports() {
    let rows = regions(include_str!("fixtures/rt_bracket_region_early_release.ken"));
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].finalizer_paths.len(), 1);
}

/// Promise class: transition sentinel for normalizer commuting conversions.
/// A match in the bracket body has two dynamic Ret leaves, but the linear
/// continuation stays one static finalizer inside one Ok-binder region.
/// If normalization later duplicates that continuation, revisit this row's
/// expected static count and retain grouping by the Ok resource binder.
#[test]
fn multiple_ret_leaves_group_by_one_ok_resource_binder() {
    let source = include_str!("fixtures/rt_bracket_region_multileaf.ken")
        .replace("__ROOT_BODY__", "(\\resource. leaf_body terminal resource)");
    let rows = regions(&source);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].finalizer_paths.len(), 1);
}

/// Promise class: durable nested-owner invariant with a transition sentinel
/// for the present linear normalizer. Two brackets in one `main` each keep
/// one static finalizer, attributed to the exact Ok binder the operand reads.
/// A wrong declaration-only or outermost-Ok key merges two groups into one.
#[test]
fn nested_brackets_with_multiple_inner_ret_leaves_have_two_binder_groups() {
    let source = include_str!("fixtures/rt_bracket_region_multileaf.ken")
        .replace("__ROOT_BODY__", "(\\resource. level_one_body cap terminal resource)");
    let rows = regions(&source);
    assert_eq!(rows.len(), 2, "one region per bracket, not per Ret leaf: {rows:?}");
    assert_ne!(rows[0].ok_resource_binder_path, rows[1].ok_resource_binder_path);
    assert_ne!(rows[0].acquire_vis_path, rows[1].acquire_vis_path);
    assert!(rows.iter().all(|row| row.finalizer_paths.len() == 1));
    assert_eq!(rows[0].owner, rows[1].owner);
    // P1: owner-only grouping would produce one group on this SAME population.
    assert_eq!(rows.iter().map(|r| &r.owner).collect::<std::collections::BTreeSet<_>>().len(), 1);
    // P2: the outer acquire is an ancestor of the inner, so an outermost-Ok
    // selector would give both finalizers one owner instead of two.
    assert!(rows.iter().any(|outer| rows.iter().any(|inner|
        outer.ok_resource_binder_path != inner.ok_resource_binder_path
            && inner.ok_resource_binder_path.starts_with(&outer.ok_resource_binder_path))));
}

/// Promise class: durable owner disambiguation. Three canonical producers
/// inside one checked declaration retain three acquire→Ok→finalizer triples;
/// a depth-two sample alone would not exercise the third nested owner.
#[test]
fn three_nested_producers_keep_three_binder_regions() {
    let source = include_str!("fixtures/rt_bracket_region_multileaf.ken")
        .replace("__ROOT_BODY__", "(\\resource. level_two_body cap terminal resource)");
    let rows = regions(&source);
    assert_eq!(rows.len(), 3);
    assert_eq!(rows.iter().map(|row| &row.ok_resource_binder_path)
        .collect::<std::collections::BTreeSet<_>>().len(), 3);
    assert_eq!(rows.iter().map(|row| &row.acquire_vis_path)
        .collect::<std::collections::BTreeSet<_>>().len(), 3);
}
