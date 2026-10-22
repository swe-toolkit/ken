//! Checked companion admission: type conversion, trust, recursion and rollback.

use ken_kernel::{
    admit_pending, admit_pending_with_companions, check, declare_def, declare_inductive,
    declare_postulate, rollback_pending, stage_placeholders, CtorSpec, GlobalEnv, GlobalId,
    InductiveSpec, KernelError, Level, Term,
};

fn c(id: GlobalId) -> Term {
    Term::const_(id, vec![])
}
fn ty0() -> Term {
    Term::Type(Level::zero())
}
fn endo() -> Term {
    Term::pi(ty0(), ty0())
}
fn stage(env: &mut GlobalEnv, name: &str, tys: Vec<Term>) -> ken_kernel::PendingAdmission {
    stage_placeholders(
        env,
        tys.into_iter().map(|t| (name.into(), vec![], t)).collect(),
    )
    .unwrap()
}

/// c : Type0 -> Type0 := \A. w A (companion, staged first)
/// w : Type0 -> Type0 := \A. A ; v : Pi A. A -> c A (members)
/// p : Pi A. A -> w A (postulate declared after staging, mentions a member)
/// v := \A a. p A a   needs w A == c A, i.e. the companion transparent.
fn setup(
    env: &mut GlobalEnv,
) -> (
    ken_kernel::PendingAdmission,
    ken_kernel::PendingAdmission,
    GlobalId,
    Term,
) {
    let comps = stage(env, "companion", vec![endo()]);
    let comp = comps.ids()[0];
    let v_ty = Term::pi(
        ty0(),
        Term::pi(Term::var(0), Term::app(c(comp), Term::var(1))),
    );
    let members = stage(env, "member", vec![endo(), v_ty]);
    let w = members.ids()[0];
    let p_ty = Term::pi(ty0(), Term::pi(Term::var(0), Term::app(c(w), Term::var(1))));
    let p = declare_postulate(env, "p".into(), vec![], p_ty).unwrap();
    let v_body = Term::lam(
        ty0(),
        Term::lam(
            Term::var(0),
            Term::app(Term::app(c(p), Term::var(1)), Term::var(0)),
        ),
    );
    (comps, members, p, v_body)
}

#[test]
fn positive_member_typed_against_companion_type() {
    let mut env = GlobalEnv::new();
    let (comps, members, _p, v_body) = setup(&mut env);
    let comp = comps.ids()[0];
    let w = members.ids()[0];
    let tb_before = env.trusted_base();
    let ids = admit_pending_with_companions(
        &mut env,
        members,
        vec![Term::lam(ty0(), Term::var(0)), v_body],
        comps,
        vec![Term::lam(ty0(), Term::app(c(w), Term::var(0)))],
    )
    .unwrap_or_else(|r| panic!("positive refused: {:?}", r.error));
    assert_eq!(ids.len(), 3);
    let tb_after = env.trusted_base();
    assert_eq!(
        tb_before,
        [comp, w, ids[2], _p].into_iter().collect::<Vec<_>>()
    );
    assert_eq!(
        tb_after,
        vec![_p],
        "only the existing postulate remains trusted"
    );
    for id in ids {
        assert!(matches!(
            env.lookup(id),
            Some(ken_kernel::Decl::Transparent { .. })
        ));
    }
}

#[test]
fn mutation_without_companions_refuses() {
    let mut env = GlobalEnv::new();
    let (comps, members, _p, v_body) = setup(&mut env);
    let error = admit_pending(
        &mut env,
        members,
        vec![Term::lam(ty0(), Term::var(0)), v_body],
    )
    .unwrap_err()
    .0;
    assert!(
        matches!(error, KernelError::TypeMismatch { .. }),
        "{error:?}"
    );
    rollback_pending(&mut env, comps).expect("companion remains rollbackable");
}

#[test]
fn recursive_companion_refused_and_companions_returned() {
    let mut env = GlobalEnv::new();
    let (comps, members, _p, v_body) = setup(&mut env);
    let comp = comps.ids()[0];
    let refusal = admit_pending_with_companions(
        &mut env,
        members,
        vec![Term::lam(ty0(), Term::var(0)), v_body],
        comps,
        vec![Term::lam(ty0(), Term::app(c(comp), Term::var(0)))],
    )
    .unwrap_err();
    assert!(
        matches!(refusal.error, KernelError::NotTerminating(ref m) if m.contains("companion definition is recursive")),
        "{:?}",
        refusal.error
    );
    assert_eq!(refusal.removed.len(), 3, "two members and later postulate");
    assert!(matches!(
        env.lookup(comp),
        Some(ken_kernel::Decl::Opaque { .. })
    ));
    rollback_pending(&mut env, refusal.companions).expect("companion handle intact");
}

/// Promise class: durable invariant (spec 17 §4).
/// MEASURED: a self-referencing companion is refused even when the unrelated
/// member never calls it. CLAIMED: every transparent companion must terminate,
/// independently of how members use it. THE GAP: the indirect path is separate.
#[test]
fn recursive_companion_refused_without_member_conversion() {
    let mut env = GlobalEnv::new();
    let companions = stage(&mut env, "companion", vec![endo()]);
    let companion = companions.ids()[0];
    let members = stage(&mut env, "member", vec![endo()]);
    let error = admit_pending_with_companions(
        &mut env,
        members,
        vec![Term::lam(ty0(), Term::var(0))],
        companions,
        vec![Term::lam(ty0(), Term::app(c(companion), Term::var(0)))],
    )
    .unwrap_err()
    .error;
    assert!(
        matches!(error, KernelError::NotTerminating(ref reason) if reason == "companion definition is recursive"),
        "{error:?}"
    );
}

/// Promise class: durable invariant (spec 17 §4; spec 18 §5).
/// MEASURED: a well-typed Nat-recursive companion passes ordinary SCT because
/// every call descends on the constructor field, yet companion admission
/// refuses it while an unrelated member remains opaque. CLAIMED: companions
/// must be non-recursive, not merely SCT-certified. THE GAP: the separate
/// indirect probe checks recursion through an external transparent definition.
#[test]
fn structurally_decreasing_companion_is_still_refused() {
    let mut env = GlobalEnv::new();
    let nat = declare_inductive(&mut env, |nat| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![
            CtorSpec {
                args: vec![],
                target_indices: vec![],
            },
            CtorSpec {
                args: vec![Term::indformer(nat, vec![])],
                target_indices: vec![],
            },
        ],
    })
    .expect("Nat");
    let zero = env.inductive(nat).unwrap().constructors[0].id;
    let nat_ty = Term::indformer(nat, vec![]);
    let fn_ty = Term::pi(nat_ty.clone(), nat_ty.clone());
    let companions = stage(&mut env, "companion", vec![fn_ty.clone()]);
    let companion = companions.ids()[0];
    let members = stage(&mut env, "member", vec![fn_ty]);
    let member = members.ids()[0];

    // Under the successor method: Var(1) is the constructor field and
    // Var(0) is the unused induction hypothesis. SCT records strict descent.
    let body = Term::lam(
        nat_ty.clone(),
        Term::Elim {
            fam: nat,
            level_args: vec![],
            params: vec![],
            motive: Box::new(Term::Ascript(
                Box::new(Term::lam(nat_ty.clone(), nat_ty.clone())),
                Box::new(Term::pi(nat_ty.clone(), Term::Type(Level::zero()))),
            )),
            methods: vec![
                Term::constructor(zero, vec![]),
                Term::lam(
                    nat_ty.clone(),
                    Term::lam(nat_ty.clone(), Term::app(c(companion), Term::var(1))),
                ),
            ],
            indices: vec![],
            scrut: Box::new(Term::var(0)),
        },
    );
    check(
        &env,
        &ken_kernel::env::Context::new(),
        &body,
        &Term::pi(nat_ty.clone(), nat_ty.clone()),
    )
    .expect("companion body type-checks while staged opaque");
    ken_kernel::sct::sct_check(&env, &[(companion, body.clone())])
        .expect("ordinary SCT accepts this structurally decreasing recursion");

    let refusal = admit_pending_with_companions(
        &mut env,
        members,
        vec![Term::lam(nat_ty.clone(), Term::var(0))],
        companions,
        vec![body],
    )
    .expect_err("a recursive companion must be refused even when SCT accepts it");
    assert!(
        matches!(refusal.error, KernelError::NotTerminating(ref reason) if reason == "companion definition is recursive"),
        "{:?}",
        refusal.error
    );
    assert!(
        env.lookup(member).is_none(),
        "failed member group rolled back"
    );
    assert!(matches!(
        env.lookup(companion),
        Some(ken_kernel::Decl::Opaque { .. })
    ));
    rollback_pending(&mut env, refusal.companions).expect("companion remains rollbackable");
}

#[test]
fn companion_reaching_itself_through_external_definition_refused() {
    // indirect form
    let mut env = GlobalEnv::new();
    let comps = stage(&mut env, "companion", vec![endo()]);
    let comp = comps.ids()[0];
    let bridge = declare_def(
        &mut env,
        vec![],
        endo(),
        Term::lam(ty0(), Term::app(c(comp), Term::var(0))),
    )
    .unwrap();
    let members = stage(&mut env, "member", vec![endo()]);
    let refusal = admit_pending_with_companions(
        &mut env,
        members,
        vec![Term::lam(ty0(), Term::var(0))],
        comps,
        vec![Term::lam(ty0(), Term::app(c(bridge), Term::var(0)))],
    )
    .unwrap_err();
    assert!(
        matches!(refusal.error, KernelError::NotTerminating(ref m) if m.contains("companion definition is recursive")),
        "{:?}",
        refusal.error
    );
}

#[test]
fn recursion_routed_through_companion_refused() {
    let mut env = GlobalEnv::new();
    let comps = stage(&mut env, "companion", vec![endo()]);
    let comp = comps.ids()[0];
    let members = stage(&mut env, "member", vec![endo()]);
    let w = members.ids()[0];
    // w := \A. c A ; c := \A. w A  -- non-decreasing loop through the companion
    let refusal = admit_pending_with_companions(
        &mut env,
        members,
        vec![Term::lam(ty0(), Term::app(c(comp), Term::var(0)))],
        comps,
        vec![Term::lam(ty0(), Term::app(c(w), Term::var(0)))],
    )
    .unwrap_err();
    assert!(
        matches!(refusal.error, KernelError::NotTerminating(ref m) if m.contains("escapes the admission group")),
        "{:?}",
        refusal.error
    );
    let before_ids = env.declarations().len();
    rollback_pending(&mut env, refusal.companions).expect("companion handle intact");
    assert!(env.declarations().len() < before_ids);
}

/// Promise class: durable invariant (spec 17 §4; spec 18 §5).
/// MEASURED: a malformed member fails checking after the companion was
/// installed on a private view; the public env retains only its original
/// opaque companion and can still admit it through the returned handle.
/// CLAIMED: failures never publish a partially checked transparent group.
/// THE GAP: this tests typing failure; the recursion refusal is separate.
#[test]
fn member_check_failure_is_atomic_and_companion_can_fallback() {
    let mut env = GlobalEnv::new();
    let companions = stage(&mut env, "companion", vec![endo()]);
    let companion = companions.ids()[0];
    let members = stage(&mut env, "member", vec![endo()]);
    let member = members.ids()[0];
    let refusal = admit_pending_with_companions(
        &mut env,
        members,
        vec![Term::lam(ty0(), Term::var(1))],
        companions,
        vec![Term::lam(ty0(), Term::var(0))],
    )
    .unwrap_err();
    assert!(
        matches!(refusal.error, KernelError::VarOutOfScope { .. }),
        "{:?}",
        refusal.error
    );
    assert!(env.lookup(member).is_none());
    assert!(matches!(
        env.lookup(companion),
        Some(ken_kernel::Decl::Opaque { .. })
    ));
    assert_eq!(env.trusted_base(), vec![companion]);
    let ids = admit_pending(
        &mut env,
        refusal.companions,
        vec![Term::lam(ty0(), Term::var(0))],
    )
    .expect("recovered companion can admit without a member");
    assert_eq!(ids, vec![companion]);
    assert!(env.trusted_base().is_empty());
}

#[test]
fn companion_without_body_refused() {
    let mut env = GlobalEnv::new();
    let comps = stage(&mut env, "companion", vec![endo()]);
    let members = stage(&mut env, "member", vec![endo()]);
    let refusal = admit_pending_with_companions(
        &mut env,
        members,
        vec![Term::lam(ty0(), Term::var(0))],
        comps,
        vec![],
    )
    .unwrap_err();
    assert!(
        matches!(refusal.error, KernelError::IllFormedDecl(ref m) if m.contains("body count")),
        "{:?}",
        refusal.error
    );
}

#[test]
fn companions_staged_after_members_refused() {
    let mut env = GlobalEnv::new();
    let members = stage(&mut env, "member", vec![endo()]);
    let comps = stage(&mut env, "companion", vec![endo()]);
    let w = members.ids()[0];
    let refusal = admit_pending_with_companions(
        &mut env,
        members,
        vec![Term::lam(ty0(), Term::var(0))],
        comps,
        vec![Term::lam(ty0(), Term::app(c(w), Term::var(0)))],
    )
    .unwrap_err();
    assert!(
        matches!(refusal.error, KernelError::IllFormedDecl(ref m) if m.contains("staged before")),
        "{:?}",
        refusal.error
    );
}
