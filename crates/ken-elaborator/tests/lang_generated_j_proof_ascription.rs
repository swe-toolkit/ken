//! Generated J proof arguments must remain inferable after a generic index
//! premise specializes to Refl beneath a surviving cast.
use ken_elaborator::ElabEnv;
use ken_kernel::Decl;

fn check_generic_law(literate_source: &str, theorem: &str) {
    let source = literate_source
        .split_once("```ken\n")
        .expect("fixture contains a Ken fence")
        .1
        .split_once("\n```")
        .expect("fixture closes its Ken fence")
        .0;
    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before = env.env.trusted_base();
    env.elaborate_file(source)
        .unwrap_or_else(|error| panic!("{theorem} must check: {error:?}"));
    let id = *env.globals.get(theorem).expect("checked theorem identity");
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn generic_zip_with_constructor_law_checks_without_new_trust() {
    // Promise class: durable invariant. MEASURED: the generic open-index E1
    // law elaborates to a checked transparent theorem with zero trust growth.
    // CLAIMED: generated J evidence in its proof stays inferable at admission.
    // THE GAP: this fixture pins this law, not every possible generated J.
    check_generic_law(
        include_str!("fixtures/lang_generated_j/e1.ken.md"),
        "zip_with_vcons",
    );
}

#[test]
fn generic_two_tail_constructor_law_checks_without_new_trust() {
    // Promise class: durable invariant. MEASURED: the generic open-index E4
    // law elaborates to a checked transparent theorem with zero trust growth.
    // CLAIMED: generated J evidence in its proof stays inferable at admission.
    // THE GAP: no claim is made about sibling-goal or matrix coverage.
    check_generic_law(
        include_str!("fixtures/lang_generated_j/e4.ken.md"),
        "zw2_vcons",
    );
}

#[test]
fn sibling_tail_generic_law_checks_after_j_proof_substitution() {
    // Promise class: durable invariant. MEASURED: E5 admits a generic theorem
    // whose inner tail passes through a generated Cast/J beneath neutral g.
    // CLAIMED: substitution of the equality premise by Refl remains checkable.
    // THE GAP: this tests a real surviving cast; it does not imply every J
    // builder's proof shape is covered. A mutation of only the congruence
    // builder's proof ascription must restore the introduction-form failure.
    check_generic_law(
        include_str!("fixtures/lang_generated_j/e5.ken.md"),
        "zw3_vcons",
    );
}
