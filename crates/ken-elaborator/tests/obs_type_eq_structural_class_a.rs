//! Source-facing Class A fixtures for structural Eq Type and typed casts.
//! The five internal retries are carried coordinates from the earlier ba2
//! trace: f7 FZero twice, f7 FSuc once, f4 inner-ys FZero twice. The two
//! complete source checks, not these prior coordinates, are the live oracle.

use ken_elaborator::ElabEnv;

const F7: &str = include_str!("fixtures/obs_type_eq_structural/f7.ken.md");
const F4: &str = include_str!("fixtures/obs_type_eq_structural/f4.ken.md");

/// Durable invariant: matching Fin and a refined Vec sibling checks its
/// dependent proof in both FZero and FSuc arms without new trust.
#[test]
fn class_a_f7_checks_both_fin_arms() {
    let mut env = ElabEnv::new().expect("prelude");
    let trusted = env.env.trusted_base();
    env.elaborate_ken_md_file(F7)
        .expect("f7 must check in both Fin branches");
    assert_eq!(env.env.trusted_base(), trusted);
}

/// Durable invariant: matching a second refined Vec sibling within the
/// FZero/FSuc split checks its proof at the original indexed goal.
#[test]
fn class_a_f4_checks_both_nested_arms() {
    let mut env = ElabEnv::new().expect("prelude");
    let trusted = env.env.trusted_base();
    env.elaborate_ken_md_file(F4)
        .expect("f4 must check after both sibling refinements");
    assert_eq!(env.env.trusted_base(), trusted);
}
