//! Kernel declaration admission (spec 18 §5): only a kernel-staged placeholder
//! or a checked, recorded assumption can receive a transparent body.

use ken_kernel::check::admit_bodies;
use ken_kernel::{
    admit_pending, declare_def, declare_inductive, declare_postulate, declare_recursive_group,
    rollback_pending, stage_placeholders, Context, CtorSpec, Decl, GlobalEnv, GlobalId,
    InductiveSpec, KernelError, Level, Term,
};

fn constant(id: GlobalId) -> Term {
    Term::const_(id, vec![])
}

fn ineligible(error: KernelError) {
    assert_eq!(
        error,
        KernelError::IllFormedDecl(
            "checked upgrade requires a staged placeholder or recorded assumption".into()
        )
    );
}

#[test]
fn p1_p2_prelude_constants_cannot_be_given_opposite_bodies() {
    let mut env = GlobalEnv::new();
    let top = env.top_id();
    let bottom = env.bottom_id();
    assert_eq!(env.trusted_base(), Vec::<GlobalId>::new());
    for (id, body) in [(bottom, constant(top)), (top, constant(bottom))] {
        let before = env.clone();
        let trusted_before = env.trusted_base();
        ineligible(admit_bodies(&mut env, &[(id, body)]).unwrap_err());
        assert_eq!(env, before, "refusal cannot mutate the kernel environment");
        assert_eq!(env.trusted_base(), trusted_before);
        assert!(matches!(env.lookup(id), Some(Decl::Opaque { .. })));
    }
    assert!(ken_kernel::check(
        &env,
        &Context::new(),
        &constant(env.tt_id()),
        &constant(bottom)
    )
    .is_err());
}

#[test]
fn postulate_named_like_prelude_is_eligible_only_at_its_own_id() {
    let mut env = GlobalEnv::new();
    let bottom = env.bottom_id();
    let top = env.top_id();
    let tt = env.tt_id();
    let named_bottom = declare_postulate(&mut env, "Bottom".into(), vec![], constant(top))
        .expect("independently checked assumption");
    assert_ne!(named_bottom, bottom);
    ineligible(admit_bodies(&mut env, &[(bottom, constant(top))]).unwrap_err());
    admit_bodies(&mut env, &[(named_bottom, constant(tt))])
        .expect("only this postulate identity is eligible");
    assert!(matches!(env.lookup(bottom), Some(Decl::Opaque { .. })));
    assert!(!env.trusted_base().contains(&named_bottom));
    assert!(env.trusted_base().is_empty());
}

#[test]
fn ineligible_later_group_member_is_refused_before_first_body_is_checked() {
    let mut env = GlobalEnv::new();
    let top = env.top_id();
    let bottom = env.bottom_id();
    let hole = declare_postulate(&mut env, "hole".into(), vec![], constant(top))
        .expect("checked assumption");
    let before = env.clone();
    let trusted_before = env.trusted_base();
    let bad_body = Term::Type(Level::zero()); // Not a proof of Top.
    ineligible(admit_bodies(&mut env, &[(hole, bad_body), (bottom, constant(top))]).unwrap_err());
    assert_eq!(env, before);
    assert_eq!(env.trusted_base(), trusted_before);
}

#[test]
fn staged_placeholder_upgrades_and_rollback_does_not_stale_authorize_reused_id() {
    let mut env = GlobalEnv::new();
    let top = constant(env.top_id());
    let pending = stage_placeholders(&mut env, vec![("staged".into(), vec![], top.clone())])
        .expect("checked signature");
    let staged = pending.ids()[0];
    assert!(env.trusted_base().contains(&staged));
    let tt = env.tt_id();
    assert_eq!(
        admit_pending(&mut env, pending, vec![constant(tt)]).unwrap(),
        vec![staged]
    );
    assert!(matches!(env.lookup(staged), Some(Decl::Transparent { .. })));
    assert!(!env.trusted_base().contains(&staged));
    let before = env.clone();
    let pending = stage_placeholders(&mut env, vec![("rolled back".into(), vec![], top)])
        .expect("checked second signature");
    let rolled_back = pending.ids()[0];
    rollback_pending(&mut env, pending).expect("valid pending rollback");
    assert_eq!(
        env, before,
        "rollback removes both declaration and provenance"
    );
    let top = env.top_id();
    let tt = env.tt_id();
    let reused = declare_postulate(&mut env, "reused".into(), vec![], constant(top))
        .expect("checked new assumption");
    assert_eq!(reused, rolled_back);
    admit_bodies(&mut env, &[(reused, constant(tt))]).expect("new assumption is eligible");
    assert!(!env.trusted_base().contains(&reused));
}

#[test]
fn cloned_assumption_keeps_its_origin_without_changing_source() {
    let mut env = GlobalEnv::new();
    let top = env.top_id();
    let hole = declare_postulate(&mut env, "hole".into(), vec![], constant(top))
        .expect("checked assumption");
    let before = env.clone();
    let mut cloned = env.clone();
    let tt = cloned.tt_id();
    admit_bodies(&mut cloned, &[(hole, constant(tt))])
        .expect("cloned recorded assumption remains eligible");
    assert_eq!(env, before);
    assert!(env.trusted_base().contains(&hole));
    assert!(!cloned.trusted_base().contains(&hole));
}

fn recursive_constant_zero() -> (GlobalEnv, GlobalId, Term, Term) {
    let mut env = GlobalEnv::new();
    let family = declare_inductive(&mut env, |id| InductiveSpec {
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
                args: vec![Term::indformer(id, vec![])],
                target_indices: vec![],
            },
        ],
    })
    .expect("checked Nat");
    let nat = Term::indformer(family, vec![]);
    let constructors = &env.inductive(family).unwrap().constructors;
    let zero = Term::constructor(constructors[0].id, vec![]);
    let one = Term::app(Term::constructor(constructors[1].id, vec![]), zero.clone());
    let rec = declare_recursive_group(
        &mut env,
        vec![(vec![], Term::pi(nat.clone(), nat.clone()))],
        |ids| {
            let step = Term::lam(
                nat.clone(),
                Term::lam(nat.clone(), Term::app(constant(ids[0]), Term::var(1))),
            );
            vec![Term::lam(
                nat.clone(),
                Term::Elim {
                    fam: family,
                    level_args: vec![],
                    params: vec![],
                    motive: Box::new(Term::Ascript(
                        Box::new(Term::lam(nat.clone(), nat.clone())),
                        Box::new(Term::pi(nat.clone(), Term::Type(Level::zero()))),
                    )),
                    methods: vec![zero.clone(), step],
                    indices: vec![],
                    scrut: Box::new(Term::var(0)),
                },
            )]
        },
    )
    .expect("SCT admits structural recursion")[0];
    assert!(env.is_recursive_transparent(rec));
    let previously_checked = declare_def(
        &mut env,
        vec![],
        Term::Eq(
            Box::new(nat.clone()),
            Box::new(Term::app(constant(rec), one.clone())),
            Box::new(zero.clone()),
        ),
        Term::Refl(Box::new(zero)),
    )
    .expect("lemma was checked against rec returning zero");
    assert!(env.transparent_body(previously_checked).is_some());
    (env, rec, nat, one)
}

#[test]
fn p3_recursion_barrier_fold_cannot_be_reupgraded_to_a_different_body() {
    let (source, rec, nat, one) = recursive_constant_zero();
    let view = source
        .with_recursion_barriers(&[rec])
        .expect("recursive barrier");
    let mut env: GlobalEnv = (*view).clone();
    assert!(matches!(env.lookup(rec), Some(Decl::Opaque { .. })));
    assert!(
        env.trusted_base().contains(&rec),
        "the derived inventory is not provenance"
    );
    let before = env.clone();
    let trusted_before = env.trusted_base();
    let different_body = Term::lam(nat, one);
    ineligible(admit_bodies(&mut env, &[(rec, different_body)]).unwrap_err());
    assert_eq!(env, before);
    assert_eq!(env.trusted_base(), trusted_before);
    assert!(matches!(env.lookup(rec), Some(Decl::Opaque { .. })));
}
