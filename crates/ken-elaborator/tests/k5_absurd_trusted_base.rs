//! `Absurd` contributes both its proof and motive to the trusted-base walk.
//! All dependency witnesses are admitted by the checked kernel API; no test
//! needs to install an unchecked transparent declaration.

use ken_elaborator::trusted_base_delta;
use ken_kernel::{
    declare_def, declare_inductive, declare_postulate, CtorSpec, GlobalEnv, InductiveSpec, Level,
    Term,
};

fn bottom(env: &GlobalEnv) -> Term {
    Term::const_(env.bottom_id(), vec![])
}

/// The only reference to the postulate sits in `Absurd`'s proof position.
#[test]
fn absurd_proof_position_counted_in_trusted_base_delta() {
    let mut env = GlobalEnv::new();
    let bottom_ty = bottom(&env);
    let p = declare_postulate(&mut env, "proof of Bottom".into(), vec![], bottom_ty)
        .expect("checked Bottom postulate");
    let ty = Term::Type(Level::zero());
    let trusted_before = env.trusted_base();
    let def_id = declare_def(
        &mut env,
        vec![],
        ty.clone(),
        Term::Absurd(Box::new(ty), Box::new(Term::const_(p, vec![]))),
    )
    .expect("checked absurd elimination");
    assert_eq!(env.trusted_base(), trusted_before);
    assert!(
        trusted_base_delta(&env, def_id).contains(&p),
        "a postulate referenced only in the proof must be counted"
    );
}

/// The only reference to the opaque type `c` sits in `Absurd`'s motive.
#[test]
fn absurd_motive_position_counted_in_trusted_base_delta() {
    let mut env = GlobalEnv::new();
    let c = declare_postulate(
        &mut env,
        "opaque type".into(),
        vec![],
        Term::Type(Level::zero()),
    )
    .expect("checked type postulate");
    let bottom_ty = bottom(&env);
    let c_ty = Term::const_(c, vec![]);
    let trusted_before = env.trusted_base();
    let def_id = declare_def(
        &mut env,
        vec![],
        Term::pi(bottom_ty.clone(), c_ty.clone()),
        Term::lam(
            bottom_ty,
            Term::Absurd(Box::new(c_ty), Box::new(Term::var(0))),
        ),
    )
    .expect("checked motive-only absurd elimination");
    assert_eq!(env.trusted_base(), trusted_before);
    assert!(
        trusted_base_delta(&env, def_id).contains(&c),
        "a postulate referenced only in the motive must be counted"
    );
}

/// The closed Nat motive does not mention an unrelated postulate in scope.
#[test]
fn absurd_with_no_postulate_reference_has_empty_delta() {
    let mut env = GlobalEnv::new();
    let bottom_ty = bottom(&env);
    let p = declare_postulate(&mut env, "unrelated".into(), vec![], bottom_ty)
        .expect("checked unrelated postulate");
    let nat = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![],
            target_indices: vec![],
        }],
    })
    .expect("checked Nat family");
    let bottom_ty = bottom(&env);
    let nat_ty = Term::indformer(nat, vec![]);
    let trusted_before = env.trusted_base();
    let def_id = declare_def(
        &mut env,
        vec![],
        Term::pi(bottom_ty.clone(), nat_ty.clone()),
        Term::lam(
            bottom_ty,
            Term::Absurd(Box::new(nat_ty), Box::new(Term::var(0))),
        ),
    )
    .expect("checked no-reference absurd elimination");
    assert_eq!(env.trusted_base(), trusted_before);
    assert!(
        !trusted_base_delta(&env, def_id).contains(&p),
        "an unrelated postulate in scope must not appear in the delta"
    );
}
