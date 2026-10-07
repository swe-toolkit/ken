//! EnvMark refuses removal of a suffix referenced by an upgraded prefix.
//! Spec 18 §5: checked declarations and trusted-base accounting remain valid.
//! Promise class: durable rollback safety invariant. Both public rollback APIs
//! must refuse before mutation; suffix-only upgrades remain removable.

use ken_kernel::check::admit_bodies;
use ken_kernel::{
    check, declare_def, declare_postulate, env_mark, rollback_pending, rollback_to_mark,
    stage_placeholders, Context, Decl, GlobalEnv, GlobalId, KernelError, Term,
};

fn constant(id: GlobalId) -> Term {
    Term::const_(id, vec![])
}

fn postulate_top(env: &mut GlobalEnv) -> GlobalId {
    let top = constant(env.top_id());
    declare_postulate(env, "p".into(), vec![], top).unwrap()
}

fn suffix_proof(env: &mut GlobalEnv) -> GlobalId {
    declare_def(env, vec![], constant(env.top_id()), constant(env.tt_id())).unwrap()
}

fn upgrade_prefix_to_suffix(env: &mut GlobalEnv, p: GlobalId) -> GlobalId {
    let d = suffix_proof(env);
    admit_bodies(env, &[(p, constant(d))]).unwrap();
    assert!(matches!(env.lookup(p), Some(Decl::Transparent { body, .. })
        if body == &constant(d)));
    assert!(!env.trusted_base().contains(&p));
    d
}

fn assert_refusal_left_valid_prefix(env: &GlobalEnv, before: &GlobalEnv, p: GlobalId, d: GlobalId) {
    assert_eq!(env, before, "refusal must not mutate the environment");
    assert_eq!(env.next_global_id(), before.next_global_id());
    assert_eq!(env.trusted_base(), before.trusted_base());
    assert!(!env.trusted_base().contains(&p));
    assert!(env.lookup(d).is_some(), "the referenced suffix must remain");
    let Some(Decl::Transparent { ty, body, .. }) = env.lookup(p) else {
        panic!("the upgraded prefix must remain transparent");
    };
    assert_eq!(body, &constant(d));
    check(env, &Context::new(), body, ty).expect("prefix body must retain its type");
}

fn prefix_error() -> KernelError {
    KernelError::IllFormedDecl("environment mark is not a prefix of the current environment".into())
}

/// MEASURED: a public EnvMark rollback refuses after a prefix upgrade, leaving
/// its declaration and referenced suffix exactly as they were at the call.
/// CLAIMED: no successful rollback can dangle an upgraded prefix body.
/// THE GAP: rollback_pending has a distinct staged-tail preflight; tested below.
#[test]
fn rollback_to_mark_refuses_upgraded_prefix_referencing_suffix() {
    let mut env = GlobalEnv::new();
    let p = postulate_top(&mut env);
    let mark = env_mark(&env);
    let d = upgrade_prefix_to_suffix(&mut env, p);
    let before = env.clone();

    assert_eq!(rollback_to_mark(&mut env, mark), Err(prefix_error()));
    assert_refusal_left_valid_prefix(&env, &before, p, d);
}

/// MEASURED: staged-tail preflight permits the attempted rollback to reach
/// the EnvMark prefix guard; refusal keeps both the placeholder and proof.
/// CLAIMED: pending rollback never leaves an upgraded prefix body dangling.
/// THE GAP: alternate kernel-owned index registrations are out of this WP.
#[test]
fn rollback_pending_refuses_upgraded_prefix_referencing_suffix() {
    let mut env = GlobalEnv::new();
    let p = postulate_top(&mut env);
    let top = constant(env.top_id());
    let pending = stage_placeholders(&mut env, vec![("staged".into(), vec![], top)]).unwrap();
    let staged = pending.ids()[0];
    let d = upgrade_prefix_to_suffix(&mut env, p);
    let before = env.clone();

    assert_eq!(rollback_pending(&mut env, pending), Err(prefix_error()));
    assert_refusal_left_valid_prefix(&env, &before, p, d);
    assert!(matches!(env.lookup(staged), Some(Decl::Opaque { .. })));
}

/// A guard on the whole environment instead of the marked prefix would reject
/// this valid rollback; both the upgraded postulate and its proof are suffix.
#[test]
fn suffix_only_upgrade_remains_removable() {
    let mut env = GlobalEnv::new();
    let before = env.clone();
    let mark = env_mark(&env);
    let p = postulate_top(&mut env);
    let d = upgrade_prefix_to_suffix(&mut env, p);
    let removed = rollback_to_mark(&mut env, mark).unwrap();
    assert_eq!(removed.iter().map(Decl::id).collect::<Vec<_>>(), vec![d, p]);
    assert_eq!(env, before);
    assert_eq!(env.trusted_base(), before.trusted_base());
}
