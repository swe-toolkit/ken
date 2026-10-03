//! Level-polymorphic declaration closure and open-universe Eq fences.
//! `12 §4`, `16 §2.2`; level and body admission are separate gates.

use ken_kernel::env::{Context, PrimReduction};
use ken_kernel::obs::{bottom_term, top_term};
use ken_kernel::{
    check, declare_def, declare_inductive, declare_postulate, declare_primitive, whnf, CtorSpec,
    GlobalEnv, InductiveSpec, KernelError, Level, LevelVar, Term,
};

const U: LevelVar = LevelVar(0);
const V: LevelVar = LevelVar(1);

fn var(level: LevelVar) -> Level {
    Level::Var(level)
}

fn eq(ty: Term, left: Term, right: Term) -> Term {
    Term::Eq(Box::new(ty), Box::new(left), Box::new(right))
}

fn undeclared(level: LevelVar) -> KernelError {
    KernelError::IllFormedDecl(format!("undeclared level variable {level:?}"))
}

fn reflexive_open_type_eq() -> Term {
    eq(Term::ty(var(U).suc()), Term::ty(var(U)), Term::ty(var(U)))
}

/// Research S1 soundness counterexample: the unabstracted `u` in BOTH the
/// signature and body must be refused at f, before a g {0} reference exists.
/// A genuinely abstracted `u` remains a valid level-polymorphic definition.
#[test]
fn unabstracted_f_refused_before_g_can_form() {
    let mut env = GlobalEnv::new();
    let before = env.next_global_id();
    let sig = Term::ty(var(U).suc());
    let body = Term::ty(var(U));
    assert_eq!(
        declare_def(&mut env, vec![], sig.clone(), body.clone()),
        Err(undeclared(U)),
    );
    assert_eq!(env.next_global_id(), before);
    assert!(
        env.lookup(before).is_none(),
        "g cannot refer to an admitted f"
    );

    let f = declare_def(&mut env, vec![U], sig, body).expect("bound u is valid");
    let use_at_zero = Term::const_(f, vec![Level::zero()]);
    let ctx = Context::new();
    assert_eq!(
        check(&env, &ctx, &use_at_zero, &Term::ty(Level::zero().suc())),
        Ok(()),
    );
    assert_eq!(whnf(&env, &ctx, &use_at_zero), Term::ty(Level::zero()));
}

/// M1: this signature has the escaped `u` and its body (`tt`) is closed.
/// Skipping only the stage_placeholders closure call must admit it.
#[test]
fn signature_only_escape_is_refused_at_staging() {
    let mut env = GlobalEnv::new();
    let before = env.next_global_id();
    let tt = Term::const_(env.tt_id(), vec![]);
    assert_eq!(
        declare_def(&mut env, vec![], reflexive_open_type_eq(), tt),
        Err(undeclared(U)),
    );
    assert_eq!(env.next_global_id(), before);
}

/// M1b: the signature is closed (`Top`) but the typed body ascription carries
/// an escaped level. Skipping only admit_bodies' closure call must admit it.
#[test]
fn body_only_escape_is_refused_before_transparent_upgrade() {
    let mut env = GlobalEnv::new();
    let before = env.next_global_id();
    let ty = top_term(&env);
    let body = Term::Ascript(
        Box::new(Term::const_(env.tt_id(), vec![])),
        Box::new(reflexive_open_type_eq()),
    );
    assert_eq!(declare_def(&mut env, vec![], ty, body), Err(undeclared(U)),);
    assert_eq!(env.next_global_id(), before);
    assert!(env.lookup(before).is_none());
}

#[test]
fn direct_checked_body_upgrade_rejects_escape_without_mutating_opaque_member() {
    let mut env = GlobalEnv::new();
    let ty = top_term(&env);
    let hole =
        declare_postulate(&mut env, "hole".into(), vec![], ty).expect("closed opaque placeholder");
    let before = env.lookup(hole).expect("hole exists").clone();
    let body = Term::Ascript(
        Box::new(Term::const_(env.tt_id(), vec![])),
        Box::new(reflexive_open_type_eq()),
    );
    assert_eq!(
        ken_kernel::check::admit_bodies(&mut env, &[(hole, body)]),
        Err(undeclared(U)),
    );
    assert_eq!(env.lookup(hole), Some(&before));
}

#[test]
fn explicit_global_level_arguments_are_checked_in_declaration_types() {
    let mut env = GlobalEnv::new();
    let a = declare_postulate(&mut env, "A".into(), vec![U], Term::ty(var(U)))
        .expect("A binds its own level");
    assert_eq!(
        declare_postulate(
            &mut env,
            "bad const".into(),
            vec![],
            Term::const_(a, vec![var(V)])
        ),
        Err(undeclared(V)),
    );
    let d = declare_inductive(&mut env, |_| InductiveSpec {
        level_params: vec![U],
        params: vec![],
        indices: vec![],
        level: var(U),
        constructors: vec![],
    })
    .expect("family binds its own level");
    assert_eq!(
        declare_postulate(
            &mut env,
            "bad former".into(),
            vec![],
            Term::indformer(d, vec![var(V)])
        ),
        Err(undeclared(V)),
    );
}

#[test]
fn duplicate_params_and_opaque_or_primitive_free_levels_are_refused() {
    let mut env = GlobalEnv::new();
    let before = env.next_global_id();
    assert_eq!(
        declare_postulate(
            &mut env,
            "duplicate".into(),
            vec![U, U],
            Term::Omega(var(U))
        ),
        Err(KernelError::IllFormedDecl(
            "duplicate level parameter".into()
        )),
    );
    assert_eq!(
        declare_postulate(&mut env, "free".into(), vec![U], Term::Omega(var(V))),
        Err(undeclared(V)),
    );
    assert_eq!(
        declare_primitive(
            &mut env,
            vec![],
            Term::ty(var(U)),
            PrimReduction::OpaqueType
        ),
        Err(undeclared(U)),
    );
    assert_eq!(env.next_global_id(), before);
    assert!(env.lookup(before).is_none());
    let valid = declare_postulate(
        &mut env,
        "bound".into(),
        vec![U, V],
        Term::Omega(var(U).max(var(V))),
    );
    assert!(
        valid.is_ok(),
        "two distinct declared variables remain valid: {valid:?}"
    );
}

#[test]
fn inductive_family_level_and_constructor_fields_must_be_closed() {
    let mut env = GlobalEnv::new();
    let before = env.next_global_id();
    assert_eq!(
        declare_inductive(&mut env, |_| InductiveSpec {
            level_params: vec![],
            params: vec![],
            indices: vec![],
            level: var(U),
            constructors: vec![],
        }),
        Err(undeclared(U)),
    );
    assert_eq!(
        env.next_global_id(),
        before,
        "failed admission releases family id"
    );
    assert_eq!(
        declare_inductive(&mut env, |_| InductiveSpec {
            level_params: vec![U],
            params: vec![],
            indices: vec![],
            level: var(U),
            constructors: vec![CtorSpec {
                args: vec![Term::ty(var(V))],
                target_indices: vec![],
            }],
        }),
        Err(undeclared(V)),
    );
    assert_eq!(env.next_global_id(), before);
}

/// M2: raw open levels are not proof of inequality; closed unequal levels
/// still reduce to Bottom, while syntactically equal open levels reduce Top.
#[test]
fn unequal_open_universe_levels_remain_neutral_but_closed_ones_decide() {
    let env = GlobalEnv::new();
    let ctx = Context::new();
    for universe in [false, true] {
        let former = |level| {
            if universe {
                Term::Omega(level)
            } else {
                Term::Type(level)
            }
        };
        for (left, right) in [
            (var(U), var(V)),
            (Level::zero(), var(U)),
            (var(U).suc().max(var(V)), var(U).suc()),
        ] {
            let equality = eq(
                Term::ty(var(U).suc().max(var(V).suc())),
                former(left),
                former(right),
            );
            assert_eq!(whnf(&env, &ctx, &equality), equality);
        }
        let closed = eq(
            Term::ty(Level::zero().suc()),
            former(Level::zero()),
            former(Level::zero().suc()),
        );
        assert_eq!(whnf(&env, &ctx, &closed), bottom_term(&env));
        let identical_open = eq(Term::ty(var(U).suc()), former(var(U)), former(var(U)));
        assert_eq!(whnf(&env, &ctx, &identical_open), top_term(&env));
    }
}
