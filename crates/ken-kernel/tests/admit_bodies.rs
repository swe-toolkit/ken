//! Checked opaque-body admission: certificates cannot retire their own trust.
//! The negative paths leave both the declaration and the trusted base intact.

use ken_kernel::{
    check::admit_bodies, declare_def, declare_postulate, Context, Decl, GlobalEnv, KernelError,
    Level, Term,
};

fn constant(id: ken_kernel::GlobalId) -> Term {
    Term::const_(id, vec![])
}

#[test]
fn direct_self_certificate_is_refused_by_sct_without_retiring_hole() {
    let mut env = GlobalEnv::new();
    let goal = constant(env.bottom_id());
    let hole = declare_postulate(&mut env, "hole".into(), vec![], goal).unwrap();
    let before = env.clone();
    let error = admit_bodies(&mut env, &[(hole, constant(hole))]).unwrap_err();
    assert!(
        matches!(error, KernelError::NotTerminating(ref why) if why.starts_with("SCT:")),
        "direct self-certificate must reach SCT, got {error:?}"
    );
    assert_eq!(env, before);
    assert!(matches!(env.lookup(hole), Some(Decl::Opaque { .. })));
    assert!(env.trusted_base().contains(&hole));
}

#[test]
fn indirect_certificate_escape_is_refused_without_retiring_hole() {
    let mut env = GlobalEnv::new();
    let goal = constant(env.bottom_id());
    let hole = declare_postulate(&mut env, "hole".into(), vec![], goal.clone()).unwrap();
    let bridge = declare_def(&mut env, vec![], goal, constant(hole)).unwrap();
    assert!(env.transparent_body(bridge).is_some());
    let before = env.clone();
    let error = admit_bodies(&mut env, &[(hole, constant(bridge))]).unwrap_err();
    assert!(
        matches!(error, KernelError::NotTerminating(ref why) if why.contains("escapes the admission group")),
        "indirect cycle must reach the escape check, got {error:?}"
    );
    assert_eq!(env, before);
    assert!(env.trusted_base().contains(&hole));
}

#[test]
fn group_escape_to_another_member_is_refused_atomically() {
    let mut env = GlobalEnv::new();
    let goal = constant(env.top_id());
    let first = declare_postulate(&mut env, "first".into(), vec![], goal.clone()).unwrap();
    let second = declare_postulate(&mut env, "second".into(), vec![], goal.clone()).unwrap();
    let bridge = declare_def(&mut env, vec![], goal, constant(second)).unwrap();
    let before = env.clone();
    let cert = constant(env.tt_id());
    let error = admit_bodies(
        &mut env,
        &[(first, constant(bridge)), (second, cert)],
    )
    .unwrap_err();
    assert!(
        matches!(error, KernelError::NotTerminating(ref why) if why.contains("escapes the admission group")),
        "escape through a non-member to a different group member must refuse: {error:?}"
    );
    assert_eq!(env, before);
    assert!(env.trusted_base().contains(&first));
    assert!(env.trusted_base().contains(&second));
}

#[test]
fn honest_checked_certificate_retires_only_its_hole() {
    let mut env = GlobalEnv::new();
    let goal = constant(env.top_id());
    let hole = declare_postulate(&mut env, "hole".into(), vec![], goal).unwrap();
    let other = declare_postulate(&mut env, "other".into(), vec![], Term::Omega(Level::zero()))
        .unwrap();
    let before = env.trusted_base();
    assert!(before.contains(&hole));
    let cert = constant(env.tt_id());
    admit_bodies(&mut env, &[(hole, cert.clone())]).unwrap();
    assert!(matches!(env.lookup(hole), Some(Decl::Transparent { .. })));
    assert_eq!(ken_kernel::whnf(&env, &Context::new(), &constant(hole)), cert);
    let mut expected = before;
    expected.retain(|id| *id != hole);
    assert_eq!(env.trusted_base(), expected);
    assert!(env.trusted_base().contains(&other));
}

#[test]
fn non_opaque_member_and_wrongly_typed_certificate_cannot_upgrade() {
    let mut env = GlobalEnv::new();
    let goal = constant(env.top_id());
    let hole = declare_postulate(&mut env, "hole".into(), vec![], goal.clone()).unwrap();
    let before = env.clone();
    let error = admit_bodies(&mut env, &[(hole, Term::ty(Level::zero()))]).unwrap_err();
    assert!(matches!(error, KernelError::TypeMismatch { .. }));
    assert_eq!(env, before);

    let cert = constant(env.tt_id());
    let transparent = declare_def(&mut env, vec![], goal, cert.clone()).unwrap();
    let before = env.clone();
    let error = admit_bodies(&mut env, &[(transparent, cert)]).unwrap_err();
    assert!(matches!(error, KernelError::IllFormedDecl(ref why) if why.contains("present opaque")));
    assert_eq!(env, before);
    assert!(env.trusted_base().contains(&hole));
}
