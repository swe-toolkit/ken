//! Structural Eq-at-Type and compound cast witnesses (`16 §2.2`, §3.2).

use ken_kernel::env::Context;
use ken_kernel::obs::{bottom_term, top_term};
use ken_kernel::term::{GlobalId, Level, LevelVar, Term};
use ken_kernel::{
    check, convert_type, declare_inductive, declare_postulate, infer, whnf, CtorSpec, GlobalEnv,
    InductiveSpec, KernelError,
};

fn eq(ty: Term, a: Term, b: Term) -> Term {
    Term::Eq(Box::new(ty), Box::new(a), Box::new(b))
}
fn type_eq(a: Term, b: Term) -> Term {
    eq(Term::Type(Level::zero()), a, b)
}
fn cast(a: Term, b: Term, e: Term, value: Term) -> Term {
    Term::Cast(Box::new(a), Box::new(b), Box::new(e), Box::new(value))
}

// The shape oracle for the Π arm is written from AC-0 rather than invoking
// the production helper; an unsound orientation cannot pass by self-agreement.
fn expected_sym_type0(a: &Term, b: &Term, proof: Term) -> Term {
    let sort = Term::Type(Level::zero());
    let proof_domain = eq(sort.clone(), ken_kernel::subst::weaken(a, 1), Term::var(0));
    let motive = Term::Ascript(
        Box::new(Term::lam(
            sort.clone(),
            Term::lam(
                proof_domain.clone(),
                eq(sort.clone(), Term::var(1), ken_kernel::subst::weaken(a, 2)),
            ),
        )),
        Box::new(Term::pi(
            sort.clone(),
            Term::pi(proof_domain, Term::Omega(Level::zero().suc())),
        )),
    );
    Term::J(
        Box::new(motive),
        Box::new(Term::Refl(Box::new(a.clone()))),
        Box::new(Term::Ascript(
            Box::new(proof),
            Box::new(eq(sort, a.clone(), b.clone())),
        )),
    )
}

struct Fixture {
    env: GlobalEnv,
    nat: Term,
    zero: Term,
    suc: GlobalId,
    vec: GlobalId,
    nil: GlobalId,
    cons: GlobalId,
}
impl Fixture {
    fn new() -> Self {
        let mut env = GlobalEnv::new();
        let nat_id = declare_inductive(&mut env, |nat| InductiveSpec {
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
        .unwrap();
        let nat = Term::indformer(nat_id, vec![]);
        let zero = Term::constructor(env.inductive(nat_id).unwrap().constructors[0].id, vec![]);
        let suc = env.inductive(nat_id).unwrap().constructors[1].id;
        let vec = declare_inductive(&mut env, |v| InductiveSpec {
            level_params: vec![LevelVar(0)],
            params: vec![Term::Type(Level::Var(LevelVar(0)))],
            indices: vec![nat.clone()],
            level: Level::Var(LevelVar(0)),
            constructors: vec![
                CtorSpec {
                    args: vec![],
                    target_indices: vec![zero.clone()],
                },
                CtorSpec {
                    args: vec![
                        nat.clone(),
                        Term::var(1),
                        Term::app(
                            Term::app(
                                Term::indformer(v, vec![Level::Var(LevelVar(0))]),
                                Term::var(2),
                            ),
                            Term::var(1),
                        ),
                    ],
                    target_indices: vec![Term::app(Term::constructor(suc, vec![]), Term::var(2))],
                },
            ],
        })
        .unwrap();
        let nil = env.inductive(vec).unwrap().constructors[0].id;
        let cons = env.inductive(vec).unwrap().constructors[1].id;
        Self {
            env,
            nat,
            zero,
            suc,
            vec,
            nil,
            cons,
        }
    }
    fn vec(&self, param: Term, index: Term) -> Term {
        Term::app(
            Term::app(Term::indformer(self.vec, vec![Level::zero()]), param),
            index,
        )
    }
    fn suc(&self, n: Term) -> Term {
        Term::app(Term::constructor(self.suc, vec![]), n)
    }
    fn opaque(&mut self, label: &str, ty: Term) -> Term {
        Term::const_(
            declare_postulate(&mut self.env, label.into(), vec![], ty).unwrap(),
            vec![],
        )
    }
}

fn assert_typed_cast(
    f: &Fixture,
    ctx: &Context,
    source: Term,
    target: Term,
    value: Term,
    expected_head: fn(&Term) -> bool,
    inferable: bool,
) {
    let eq_type = type_eq(source.clone(), target.clone());
    assert!(infer(&f.env, ctx, &eq_type).is_ok(), "formed equality");
    let mut proof_ctx = ctx.clone();
    proof_ctx.push(eq_type);
    let redex = cast(
        ken_kernel::subst::weaken(&source, 1),
        ken_kernel::subst::weaken(&target, 1),
        Term::var(0),
        ken_kernel::subst::weaken(&value, 1),
    );
    let before = infer(&f.env, &proof_ctx, &redex).expect("typed cast redex");
    let trusted = f.env.trusted_base();
    let reduct = whnf(&f.env, &proof_ctx, &redex);
    assert_ne!(reduct, redex, "cast must fire with neutral evidence");
    assert!(expected_head(&reduct), "wrong cast reduct: {reduct:?}");
    assert_eq!(
        check(&f.env, &proof_ctx, &reduct, &before),
        Ok(()),
        "cast reduct must check against the redex's inferred type: {reduct:?}"
    );
    if inferable {
        let after = infer(&f.env, &proof_ctx, &reduct).expect("inferable reduct");
        assert!(convert_type(&f.env, &proof_ctx, &before, &after));
    }
    assert_eq!(f.env.trusted_base(), trusted);
}

/// Durable invariant: a neutral type-equality proof supplies the domain
/// symmetry and the codomain equality used by a dependent Π cast.
#[test]
fn pi_structural_equality_and_typed_cast() {
    let mut f = Fixture::new();
    let source_domain = f.opaque("source_domain", Term::Type(Level::zero()));
    let target_domain = f.opaque("target_domain", Term::Type(Level::zero()));
    let g = f.opaque("g", Term::pi(source_domain.clone(), f.nat.clone()));
    let h = f.opaque("h", Term::pi(target_domain.clone(), f.nat.clone()));
    let source_index = Term::app(ken_kernel::subst::weaken(&g, 1), Term::var(0));
    let target_index = Term::app(ken_kernel::subst::weaken(&h, 1), Term::var(0));
    let a = Term::pi(source_domain, f.vec(f.nat.clone(), source_index));
    let b = Term::pi(target_domain, f.vec(f.nat.clone(), target_index));
    let value = f.opaque("function", a.clone());
    let ctx = Context::new();
    let reduct = whnf(&f.env, &ctx, &type_eq(a.clone(), b.clone()));
    let Term::Pi(source_dom, source_cod) = &a else {
        unreachable!()
    };
    let Term::Pi(target_dom, target_cod) = &b else {
        unreachable!()
    };
    let Term::Sigma(dom_eq, cod_family) = &reduct else {
        panic!("Π/Π type equality must decompose");
    };
    assert!(convert_type(
        &f.env,
        &ctx,
        dom_eq,
        &type_eq((**source_dom).clone(), (**target_dom).clone())
    ));
    let Term::Pi(cod_dom, cod_eq) = &**cod_family else {
        panic!("codomain must be a family")
    };
    assert_eq!(**cod_dom, ken_kernel::subst::weaken(target_dom, 1));
    let Term::Eq(cod_sort, cod_source, cod_target) = &**cod_eq else {
        panic!("codomain must be a type equality")
    };
    assert_eq!(**cod_sort, Term::Type(Level::zero()));
    assert_eq!(**cod_target, ken_kernel::subst::shift(target_cod, 1, 1));
    let mut cod_ctx = Context::new();
    cod_ctx.push(type_eq((**source_dom).clone(), (**target_dom).clone()));
    cod_ctx.push(ken_kernel::subst::weaken(target_dom, 1));
    let source_at_x = ken_kernel::subst::weaken(source_dom, 2);
    let target_at_x = ken_kernel::subst::weaken(target_dom, 2);
    let back_x = cast(
        target_at_x.clone(),
        source_at_x.clone(),
        expected_sym_type0(&source_at_x, &target_at_x, Term::var(1)),
        Term::var(0),
    );
    let expected_source =
        ken_kernel::subst::subst0(&ken_kernel::subst::shift(source_cod, 2, 1), &back_x);
    assert!(
        convert_type(&f.env, &cod_ctx, cod_source, &expected_source),
        "Π codomain must be indexed by the typed back-cast"
    );
    assert!(infer(&f.env, &ctx, &reduct).is_ok(), "formed reduct");
    assert_typed_cast(
        &f,
        &ctx,
        a.clone(),
        b.clone(),
        value,
        |t| matches!(t, Term::Lam(..)),
        false,
    );
    for ty in [a, b] {
        assert_eq!(
            check(
                &f.env,
                &ctx,
                &Term::Refl(Box::new(ty.clone())),
                &type_eq(ty.clone(), ty)
            ),
            Ok(())
        );
    }
}

/// Durable invariant: Π symmetry is indexed by the inferred domain level,
/// not hardwired to Type 0. Distinct Type-1 domains force neutral evidence
/// to travel through both the Eq-Type arm and the cast's back-transport.
#[test]
fn pi_type_one_structural_equality_and_typed_symmetry() {
    let mut f = Fixture::new();
    let high = Level::zero().suc();
    let source_dom = f.opaque("type_one_source", Term::Type(high.clone()));
    let target_dom = f.opaque("type_one_target", Term::Type(high.clone()));
    let source = Term::pi(source_dom.clone(), Term::Type(Level::zero()));
    let target = Term::pi(target_dom.clone(), Term::Type(Level::zero()));
    let value = f.opaque("type_one_function", source.clone());
    let ctx = Context::new();
    let equality = eq(Term::Type(high.clone()), source.clone(), target.clone());
    infer(&f.env, &ctx, &equality).expect("Type-1 Π equality is formed");
    let structure = whnf(&f.env, &ctx, &equality);
    let Term::Sigma(domain_eq, family) = &structure else {
        panic!("Type-1 Π equality must structurally decompose")
    };
    assert!(convert_type(
        &f.env,
        &ctx,
        domain_eq,
        &eq(Term::Type(high.clone()), source_dom, target_dom),
    ));
    let Term::Pi(_, cod_eq) = &**family else {
        panic!("Type-1 codomain equality must be a family")
    };
    assert!(matches!(&**cod_eq, Term::Eq(sort, ..) if **sort == Term::Type(high)));
    infer(&f.env, &ctx, &structure).expect("decomposed Type-1 equality is typed");

    let mut proof_ctx = ctx;
    proof_ctx.push(equality);
    let redex = cast(
        source,
        target,
        Term::var(0),
        ken_kernel::subst::weaken(&value, 1),
    );
    let before = infer(&f.env, &proof_ctx, &redex).expect("typed neutral-evidence cast");
    let trusted = f.env.trusted_base();
    let reduct = whnf(&f.env, &proof_ctx, &redex);
    assert_ne!(redex, reduct, "Type-1 Π cast must fire");
    assert!(matches!(reduct, Term::Lam(..)));
    assert_eq!(
        check(&f.env, &proof_ctx, &reduct, &before),
        Ok(()),
        "Type-1 back-cast must use symmetry at the inferred level"
    );
    assert_eq!(f.env.trusted_base(), trusted);
}

/// Durable invariant: Σ's codomain witness is indexed by the *source* fst.
#[test]
fn sigma_structural_equality_and_typed_cast() {
    let mut f = Fixture::new();
    let a = Term::sigma(f.nat.clone(), f.vec(f.nat.clone(), Term::var(0)));
    let b = Term::sigma(f.nat.clone(), f.vec(f.nat.clone(), f.suc(Term::var(0))));
    let value = f.opaque("pair", a.clone());
    let ctx = Context::new();
    let reduct = whnf(&f.env, &ctx, &type_eq(a.clone(), b.clone()));
    let Term::Sigma(source_dom, source_cod) = &a else {
        unreachable!()
    };
    let Term::Sigma(target_dom, target_cod) = &b else {
        unreachable!()
    };
    let Term::Sigma(_, cod_family) = &reduct else {
        panic!("Σ/Σ type equality must decompose")
    };
    let Term::Pi(cod_dom, cod_eq) = &**cod_family else {
        panic!("codomain must be a family")
    };
    assert_eq!(**cod_dom, ken_kernel::subst::weaken(source_dom, 1));
    let Term::Eq(cod_sort, cod_source, cod_target) = &**cod_eq else {
        panic!("Σ codomain must compare types")
    };
    assert_eq!(**cod_sort, Term::Type(Level::zero()));
    let mut cod_ctx = Context::new();
    cod_ctx.push(type_eq((**source_dom).clone(), (**target_dom).clone()));
    cod_ctx.push(ken_kernel::subst::weaken(source_dom, 1));
    let forward_x = cast(
        ken_kernel::subst::weaken(source_dom, 2),
        ken_kernel::subst::weaken(target_dom, 2),
        Term::var(1),
        Term::var(0),
    );
    let expected_source = ken_kernel::subst::shift(source_cod, 1, 1);
    let expected_target =
        ken_kernel::subst::subst0(&ken_kernel::subst::shift(target_cod, 2, 1), &forward_x);
    assert!(convert_type(&f.env, &cod_ctx, cod_source, &expected_source));
    assert!(convert_type(&f.env, &cod_ctx, cod_target, &expected_target));
    assert!(infer(&f.env, &ctx, &reduct).is_ok());
    assert_typed_cast(
        &f,
        &ctx,
        a.clone(),
        b,
        value,
        |t| matches!(t, Term::Pair(..)),
        false,
    );
    assert_eq!(
        check(
            &f.env,
            &ctx,
            &Term::Refl(Box::new(a.clone())),
            &type_eq(a.clone(), a)
        ),
        Ok(())
    );
}

/// Durable invariant: quotient relations are compared pointwise after the
/// underlying value type has been transported with e.1.
#[test]
fn quotient_structural_equality_and_typed_cast() {
    let mut f = Fixture::new();
    let n = f.opaque("n", f.nat.clone());
    let m = f.opaque("m", f.nat.clone());
    let relation = |end: Term| {
        Term::Ascript(
            Box::new(Term::lam(
                f.nat.clone(),
                Term::lam(
                    f.nat.clone(),
                    eq(
                        f.nat.clone(),
                        Term::var(1),
                        ken_kernel::subst::weaken(&end, 2),
                    ),
                ),
            )),
            Box::new(Term::pi(
                f.nat.clone(),
                Term::pi(f.nat.clone(), Term::Omega(Level::zero())),
            )),
        )
    };
    let a = Term::Quot(Box::new(f.nat.clone()), Box::new(relation(n)));
    let b = Term::Quot(Box::new(f.nat.clone()), Box::new(relation(m)));
    let ctx = Context::new();
    let reduct = whnf(&f.env, &ctx, &type_eq(a.clone(), b.clone()));
    let Term::Sigma(dom_eq, rel_family) = &reduct else {
        panic!("Quot/Quot must decompose")
    };
    let Term::Quot(source_dom, source_rel) = &a else {
        unreachable!()
    };
    let Term::Quot(target_dom, target_rel) = &b else {
        unreachable!()
    };
    assert!(convert_type(
        &f.env,
        &ctx,
        dom_eq,
        &type_eq(f.nat.clone(), f.nat.clone())
    ));
    let Term::Pi(x_type, y_family) = &**rel_family else {
        panic!("missing relation x binder")
    };
    assert_eq!(**x_type, ken_kernel::subst::weaken(&f.nat, 1));
    let Term::Pi(y_type, rel_eq) = &**y_family else {
        panic!("missing relation y binder")
    };
    assert_eq!(**y_type, ken_kernel::subst::weaken(&f.nat, 2));
    let Term::Eq(rel_sort, rel_source, rel_target) = &**rel_eq else {
        panic!("Quot relation family must compare propositions")
    };
    assert_eq!(**rel_sort, Term::Omega(Level::zero()));
    let mut rel_ctx = Context::new();
    rel_ctx.push(type_eq((**source_dom).clone(), (**target_dom).clone()));
    rel_ctx.push(ken_kernel::subst::weaken(source_dom, 1));
    rel_ctx.push(ken_kernel::subst::weaken(source_dom, 2));
    let expected_source = Term::app(
        Term::app(ken_kernel::subst::weaken(source_rel, 3), Term::var(1)),
        Term::var(0),
    );
    let forward = |v: Term| {
        cast(
            ken_kernel::subst::weaken(source_dom, 3),
            ken_kernel::subst::weaken(target_dom, 3),
            Term::var(2),
            v,
        )
    };
    let expected_target = Term::app(
        Term::app(
            ken_kernel::subst::weaken(target_rel, 3),
            forward(Term::var(1)),
        ),
        forward(Term::var(0)),
    );
    assert!(convert_type(&f.env, &rel_ctx, rel_source, &expected_source));
    assert!(convert_type(&f.env, &rel_ctx, rel_target, &expected_target));
    assert!(infer(&f.env, &ctx, &reduct).is_ok());
    let value = Term::QuotClass(Box::new(f.zero.clone()));
    assert_typed_cast(
        &f,
        &ctx,
        a.clone(),
        b,
        value,
        |t| matches!(t, Term::QuotClass(..)),
        false,
    );
    assert_eq!(
        check(
            &f.env,
            &ctx,
            &Term::Refl(Box::new(a.clone())),
            &type_eq(a.clone(), a)
        ),
        Ok(())
    );
}

/// Durable invariant: the quotient-class payload moves from A to B along
/// e.1. Distinct opaque domains make a reversed proof fail typing.
#[test]
fn quotient_cast_orients_distinct_underlying_domains() {
    let mut f = Fixture::new();
    let a = f.opaque("quot_source", Term::Type(Level::zero()));
    let b = f.opaque("quot_target", Term::Type(Level::zero()));
    let x = f.opaque("quot_member", a.clone());
    let relation = |dom: Term| {
        Term::Ascript(
            Box::new(Term::lam(
                dom.clone(),
                Term::lam(
                    dom.clone(),
                    eq(
                        ken_kernel::subst::weaken(&dom, 2),
                        Term::var(1),
                        Term::var(0),
                    ),
                ),
            )),
            Box::new(Term::pi(
                dom.clone(),
                Term::pi(dom, Term::Omega(Level::zero())),
            )),
        )
    };
    let source = Term::Quot(Box::new(a.clone()), Box::new(relation(a)));
    let target = Term::Quot(Box::new(b.clone()), Box::new(relation(b)));
    assert_typed_cast(
        &f,
        &Context::new(),
        source,
        target,
        Term::QuotClass(Box::new(x)),
        |t| matches!(t, Term::QuotClass(..)),
        false,
    );
}

/// Durable invariant: the index equality projected from e supplies the J
/// witness of Vec's recursive field transport. It is not Refl(Vec Nat n).
#[test]
fn inductive_index_rewrite_has_typed_subcast() {
    let mut f = Fixture::new();
    let n = f.opaque("n", f.nat.clone());
    let m = f.opaque("m", f.nat.clone());
    let x = f.opaque("x", f.nat.clone());
    let xs = f.opaque("xs", f.vec(f.nat.clone(), n.clone()));
    let source = f.vec(f.nat.clone(), f.suc(n.clone()));
    let target = f.vec(f.nat.clone(), f.suc(m.clone()));
    let cons = Term::constructor(f.cons, vec![Level::zero()]);
    let value = Term::app(
        Term::app(Term::app(Term::app(cons, f.nat.clone()), n.clone()), x),
        xs,
    );
    let ctx = Context::new();
    let reduct = whnf(&f.env, &ctx, &type_eq(source.clone(), target.clone()));
    assert!(matches!(reduct, Term::Sigma(_, _)));
    assert!(infer(&f.env, &ctx, &reduct).is_ok());
    let mut proof_ctx = Context::new();
    proof_ctx.push(type_eq(source.clone(), target.clone()));
    let param = Term::proj1(Term::var(0));
    let index = Term::proj2(Term::var(0));
    assert!(convert_type(
        &f.env,
        &proof_ctx,
        &infer(&f.env, &proof_ctx, &param).unwrap(),
        &type_eq(f.nat.clone(), f.nat.clone())
    ));
    assert!(convert_type(
        &f.env,
        &proof_ctx,
        &infer(&f.env, &proof_ctx, &index).unwrap(),
        &eq(f.nat.clone(), f.suc(n.clone()), f.suc(m.clone()))
    ));
    assert_typed_cast(
        &f,
        &ctx,
        source.clone(),
        target,
        value,
        |t| matches!(t, Term::App(..)),
        true,
    );
    assert_eq!(
        check(
            &f.env,
            &ctx,
            &Term::Refl(Box::new(source.clone())),
            &type_eq(source.clone(), source)
        ),
        Ok(())
    );
}

#[test]
fn inductive_parameter_change_and_empty_telescope() {
    let mut f = Fixture::new();
    let bool_id = declare_inductive(&mut f.env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![],
            target_indices: vec![],
        }],
    })
    .unwrap();
    let bool_ty = Term::indformer(bool_id, vec![]);
    let source = f.vec(f.nat.clone(), f.zero.clone());
    let target = f.vec(bool_ty, f.zero.clone());
    let nil = Term::app(Term::constructor(f.nil, vec![Level::zero()]), f.nat.clone());
    assert_typed_cast(
        &f,
        &Context::new(),
        source,
        target,
        nil,
        |t| matches!(t, Term::App(..)),
        true,
    );
    let empty = Term::indformer(bool_id, vec![]);
    assert_eq!(
        whnf(
            &f.env,
            &Context::new(),
            &type_eq(empty.clone(), empty.clone())
        ),
        top_term(&f.env)
    );
    assert_eq!(
        check(
            &f.env,
            &Context::new(),
            &Term::Refl(Box::new(empty.clone())),
            &type_eq(empty.clone(), empty)
        ),
        Ok(())
    );
}

/// Durable invariant: a dependent Π motive reduces a non-refl J through
/// the Π cast into a lambda that checks at J's inferred result type.
#[test]
fn j_dependent_motive_fires() {
    let mut f = Fixture::new();
    let n = f.opaque("n", f.nat.clone());
    let m = f.opaque("m", f.nat.clone());
    let base_type = Term::pi(f.vec(f.nat.clone(), n.clone()), f.nat.clone());
    let base = f.opaque("base", base_type);
    let proof_ty = eq(f.nat.clone(), n.clone(), Term::var(0));
    let motive = Term::Ascript(
        Box::new(Term::lam(
            f.nat.clone(),
            Term::lam(
                proof_ty.clone(),
                Term::pi(f.vec(f.nat.clone(), Term::var(1)), f.nat.clone()),
            ),
        )),
        Box::new(Term::pi(
            f.nat.clone(),
            Term::pi(proof_ty, Term::Type(Level::zero())),
        )),
    );
    let mut ctx = Context::new();
    let evidence_ty = eq(f.nat.clone(), n, m);
    ctx.push(evidence_ty.clone());
    let redex = Term::J(
        Box::new(motive),
        Box::new(base),
        Box::new(Term::Ascript(Box::new(Term::var(0)), Box::new(evidence_ty))),
    );
    let before = infer(&f.env, &ctx, &redex).expect("typed dependent J");
    let Term::J(motive, base, evidence) = &redex else {
        unreachable!()
    };
    let first = ken_kernel::obs::j_reduce(&f.env, &ctx, motive, base, evidence)
        .expect("J must fire on non-refl evidence");
    assert!(matches!(first, Term::Cast(..)), "J must hand off to cast");
    let first_type = infer(&f.env, &ctx, &first).expect("typed J-cast");
    assert!(convert_type(&f.env, &ctx, &before, &first_type));
    let reduct = whnf(&f.env, &ctx, &redex);
    assert!(
        matches!(reduct, Term::Lam(..)),
        "the Π cast must finish at λ"
    );
    assert_eq!(check(&f.env, &ctx, &reduct, &before), Ok(()));
}

/// Durable invariant: unsupported same-former reductions stay neutral,
/// including a level mismatch, a neutral head, and Trunc/Trunc.
#[test]
fn unsupported_and_neutral_type_equalities_do_not_fabricate_components() {
    let f = Fixture::new();
    let mut ctx = Context::new();
    ctx.push(Term::Type(Level::zero()));
    let pi = Term::pi(f.nat.clone(), f.nat.clone());
    let neutral = type_eq(pi.clone(), Term::var(0));
    assert_eq!(whnf(&f.env, &ctx, &neutral), neutral);
    let trunc = Term::Trunc(Box::new(f.nat.clone()));
    let trunc_pair = type_eq(trunc.clone(), trunc);
    assert_eq!(whnf(&f.env, &ctx, &trunc_pair), trunc_pair);
    let large = Term::pi(f.nat.clone(), Term::Type(Level::zero()));
    let mismatch = eq(Term::Type(Level::zero().suc()), pi, large);
    assert_eq!(whnf(&f.env, &ctx, &mismatch), mismatch);
}

/// Durable invariant: when both independent D indices change, each
/// dependent constructor field transports over the projected index proofs;
/// a field depending on both needs two chained J transports.
#[test]
fn two_index_changes_transport_each_dependent_field() {
    let mut f = Fixture::new();
    let nat = f.nat.clone();
    let vec_id = f.vec;
    let suc = f.suc;
    let item = nat.clone();
    let vec = move |i: Term| {
        Term::app(
            Term::app(Term::indformer(vec_id, vec![Level::zero()]), item.clone()),
            i,
        )
    };
    let twin = declare_inductive(&mut f.env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![nat.clone(), nat.clone()],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![
                nat.clone(),
                nat.clone(),
                vec(Term::var(1)),
                vec(Term::var(1)),
                Term::sigma(vec(Term::var(3)), vec(Term::var(3))),
            ],
            target_indices: vec![
                Term::app(Term::constructor(suc, vec![]), Term::var(4)),
                Term::app(Term::constructor(suc, vec![]), Term::var(3)),
            ],
        }],
    })
    .expect("two-index family");
    let ctor = f.env.inductive(twin).unwrap().constructors[0].id;
    let n = f.opaque("n", nat.clone());
    let m = f.opaque("m", nat.clone());
    let n2 = f.opaque("n2", nat.clone());
    let m2 = f.opaque("m2", nat.clone());
    let xs = f.opaque("xs", vec(n.clone()));
    let ys = f.opaque("ys", vec(m.clone()));
    let z = f.opaque("z", Term::sigma(vec(n.clone()), vec(m.clone())));
    let family = |i: Term, j: Term| Term::app(Term::app(Term::indformer(twin, vec![]), i), j);
    let source = family(f.suc(n.clone()), f.suc(m.clone()));
    let target = family(f.suc(n2.clone()), f.suc(m2.clone()));
    let value = [n, m, xs, ys, z]
        .into_iter()
        .fold(Term::constructor(ctor, vec![]), Term::app);
    assert_typed_cast(
        &f,
        &Context::new(),
        source,
        target,
        value,
        |t| matches!(t, Term::App(..)),
        true,
    );
}

/// Durable invariant: a dependent D sub-cast retains open Γ variables
/// when J binds its motive. A second shift by the constructor-arg count
/// would send the type parameter out of scope and leave the cast stuck.
#[test]
fn open_parameter_index_rewrite_has_scoped_witness() {
    let f = Fixture::new();
    let mut ctx = Context::new();
    ctx.push(Term::Type(Level::zero())); // a : Type 0
    ctx.push(f.nat.clone()); // n : Nat
    ctx.push(f.nat.clone()); // m : Nat
    ctx.push(Term::var(2)); // x : a
    ctx.push(f.vec(Term::var(3), Term::var(2))); // xs : Vec a n
    let before_proof = type_eq(
        f.vec(Term::var(4), f.suc(Term::var(3))),
        f.vec(Term::var(4), f.suc(Term::var(2))),
    );
    ctx.push(before_proof);
    let param = Term::var(5);
    let n = Term::var(4);
    let m = Term::var(3);
    let value = [param.clone(), n.clone(), Term::var(2), Term::var(1)]
        .into_iter()
        .fold(Term::constructor(f.cons, vec![Level::zero()]), Term::app);
    let source = f.vec(param.clone(), f.suc(n));
    let target = f.vec(param, f.suc(m));
    let redex = cast(source, target, Term::var(0), value);
    let before = infer(&f.env, &ctx, &redex).expect("open redex typed");
    let reduct = whnf(&f.env, &ctx, &redex);
    assert_ne!(reduct, redex, "open indexed cast must fire");
    assert_eq!(check(&f.env, &ctx, &reduct, &before), Ok(()));
    let after = infer(&f.env, &ctx, &reduct).expect("inferable constructor");
    assert!(convert_type(&f.env, &ctx, &before, &after));
}

/// Durable invariant: D/D only decomposes on fully applied families with
/// equivalent level arguments. These raw controls make each guard reachable.
#[test]
fn inductive_arity_and_level_mismatches_stay_neutral() {
    let f = Fixture::new();
    let ctx = Context::new();
    let partial = Term::app(Term::indformer(f.vec, vec![Level::zero()]), f.nat.clone());
    let partial_eq = type_eq(partial.clone(), partial);
    assert_eq!(whnf(&f.env, &ctx, &partial_eq), partial_eq);
    let at_zero = f.vec(f.nat.clone(), f.zero.clone());
    let at_one = Term::app(
        Term::app(
            Term::indformer(f.vec, vec![Level::zero().suc()]),
            f.nat.clone(),
        ),
        f.zero.clone(),
    );
    let levels = eq(Term::Type(Level::zero().suc()), at_zero, at_one);
    assert_eq!(whnf(&f.env, &ctx, &levels), levels);
}

/// Durable invariant: an Ω-level mismatch between quotient relations
/// cannot be converted into a spurious structural component equality.
#[test]
fn quotient_relation_level_mismatch_stays_neutral() {
    let f = Fixture::new();
    let ctx = Context::new();
    let relation = |omega: Level, body: Term| {
        Term::Ascript(
            Box::new(Term::lam(f.nat.clone(), Term::lam(f.nat.clone(), body))),
            Box::new(Term::pi(
                f.nat.clone(),
                Term::pi(f.nat.clone(), Term::Omega(omega)),
            )),
        )
    };
    let low = relation(
        Level::zero(),
        eq(f.nat.clone(), f.zero.clone(), f.zero.clone()),
    );
    let high = relation(Level::zero().suc(), type_eq(f.nat.clone(), f.nat.clone()));
    let a = Term::Quot(Box::new(f.nat.clone()), Box::new(low));
    let b = Term::Quot(Box::new(f.nat.clone()), Box::new(high));
    let pair = type_eq(a, b);
    assert_eq!(whnf(&f.env, &ctx, &pair), pair);
}

/// Durable invariant: the domain pair of different rigid Π heads is
/// impossible, not a spurious reflexive proof of the compound equality.
#[test]
fn differing_pi_domains_cannot_admit_refl() {
    let mut f = Fixture::new();
    let bool_id = declare_inductive(&mut f.env, |_| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: Level::zero(),
        constructors: vec![CtorSpec {
            args: vec![],
            target_indices: vec![],
        }],
    })
    .unwrap();
    let bool_ty = Term::indformer(bool_id, vec![]);
    let left = Term::pi(f.nat.clone(), f.nat.clone());
    let right = Term::pi(bool_ty, f.nat.clone());
    let ctx = Context::new();
    let reduct = whnf(&f.env, &ctx, &type_eq(left.clone(), right.clone()));
    let Term::Sigma(domain, _) = reduct else {
        panic!("Π decomposition must be Σ")
    };
    assert_eq!(whnf(&f.env, &ctx, &domain), bottom_term(&f.env));
    assert!(matches!(
        check(
            &f.env,
            &ctx,
            &Term::Refl(Box::new(left)),
            &type_eq(Term::pi(f.nat.clone(), f.nat.clone()), right)
        ),
        Err(KernelError::TypeMismatch { .. })
    ));
}
