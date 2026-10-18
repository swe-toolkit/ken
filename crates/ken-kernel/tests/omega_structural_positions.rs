//! Checked Cast-proof irrelevance with neutral source and target types.
//! Conformance: `conversion/cast-omega-proof-position` in seed-conversion.md.

use ken_kernel::check::infer;
use ken_kernel::env::Context;
use ken_kernel::term::{Level, Term};
use ken_kernel::{convert, GlobalEnv};

fn cast(a: Term, b: Term, proof: Term, value: Term) -> Term {
    Term::Cast(Box::new(a), Box::new(b), Box::new(proof), Box::new(value))
}

/// Durable invariant (`16 §3.1`, §8.2): Cast's proof position is Ω by
/// formation, while its relevant value remains structurally significant.
/// MEASURED: two checked open Casts differing only in proofs convert; replacing
/// the value with another checked variable does not. CLAIMED: neutral Cast
/// congruence ignores proof contents, not its Type-valued operand. GAP: changed
/// A/B require separate raw structural controls: they do not share an inferred
/// result type or a single checked source value at genuinely distinct types.
#[test]
fn checked_neutral_cast_skips_only_omega_proof() {
    let env = GlobalEnv::new();
    let before_trust = env.trusted_base();
    let mut ctx = Context::new();
    ctx.push(Term::Type(Level::zero())); // A : Type 0
    ctx.push(Term::Type(Level::zero())); // B : Type 0
    ctx.push(Term::var(1)); // x : A
    ctx.push(Term::var(2)); // y : A
    ctx.push(Term::Eq(
        Box::new(Term::Type(Level::zero())),
        Box::new(Term::var(3)),
        Box::new(Term::var(2)),
    )); // p : Eq (Type 0) A B
    ctx.push(Term::Eq(
        Box::new(Term::Type(Level::zero())),
        Box::new(Term::var(4)),
        Box::new(Term::var(3)),
    )); // q : Eq (Type 0) A B

    let a = Term::var(5);
    let b = Term::var(4);
    let x = Term::var(3);
    let y = Term::var(2);
    let first = cast(a.clone(), b.clone(), Term::var(1), x.clone());
    let other_proof = cast(a.clone(), b.clone(), Term::var(0), x);
    let other_value = cast(a, b.clone(), Term::var(0), y);
    assert_ne!(first, other_proof);
    assert_eq!(infer(&env, &ctx, &first).unwrap(), b);
    assert_eq!(infer(&env, &ctx, &other_proof).unwrap(), b);
    assert_eq!(infer(&env, &ctx, &other_value).unwrap(), b);
    assert!(convert(&env, &ctx, &b, &first, &other_proof));
    assert!(!convert(&env, &ctx, &b, &first, &other_value));
    assert_eq!(env.trusted_base(), before_trust);
}
