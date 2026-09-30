//! Observational reducts must remain typable at the redex's type (`16 §2.2`, §4.1).
use ken_kernel::env::Context;
use ken_kernel::term::{GlobalId, Level, LevelVar, Term};
use ken_kernel::{convert_type, declare_inductive, declare_postulate, infer, whnf, CtorSpec, GlobalEnv, InductiveSpec};

struct Fixture {
    env: GlobalEnv,
    nat: Term,
    vec_id: GlobalId,
}

impl Fixture {
    fn new() -> Self {
        let mut env = GlobalEnv::new();
        let nat_id = declare_inductive(&mut env, |nat| InductiveSpec {
            level_params: vec![], params: vec![], indices: vec![], level: Level::zero(),
            constructors: vec![CtorSpec { args: vec![], target_indices: vec![] },
                CtorSpec { args: vec![Term::indformer(nat, vec![])], target_indices: vec![] }],
        }).unwrap();
        let nat = Term::indformer(nat_id, vec![]);
        let zero_id = env.inductive(nat_id).unwrap().constructors[0].id;
        let suc_id = env.inductive(nat_id).unwrap().constructors[1].id;
        let vec_id = declare_inductive(&mut env, |v| InductiveSpec {
            level_params: vec![LevelVar(0)],
            params: vec![Term::Type(Level::Var(LevelVar(0)))],
            indices: vec![nat.clone()], level: Level::Var(LevelVar(0)),
            constructors: vec![
                CtorSpec { args: vec![], target_indices: vec![Term::constructor(zero_id, vec![])] },
                CtorSpec { args: vec![nat.clone(), Term::var(1),
                    Term::app(Term::app(Term::indformer(v, vec![Level::Var(LevelVar(0))]), Term::var(2)), Term::var(1))],
                    target_indices: vec![Term::app(Term::constructor(suc_id, vec![]), Term::var(2))] },
            ],
        }).unwrap();
        Self {env, nat, vec_id}
    }
    fn vec(&self, index: Term) -> Term {
        Term::app(Term::app(Term::indformer(self.vec_id, vec![Level::zero()]), self.nat.clone()), index)
    }
    fn opaque(&mut self, label: &str, ty: Term) -> Term {
        Term::const_(declare_postulate(&mut self.env, label.into(), vec![], ty).unwrap(), vec![])
    }
    fn family(&self, shape: &str) -> Term {
        match shape {
            "vec" => self.vec(Term::var(0)),
            "pi" => Term::pi(self.nat.clone(), self.vec(Term::var(1))),
            "sigma" => Term::sigma(self.nat.clone(), self.vec(Term::var(1))),
            _ => unreachable!(),
        }
    }
}
fn eq(ty: Term, a: Term, b: Term) -> Term {
    Term::Eq(Box::new(ty), Box::new(a), Box::new(b))
}
fn assert_typed_reduction(env: &GlobalEnv, redex: &Term, shape: &str) {
    let ctx = Context::new();
    let before = infer(env, &ctx, redex).expect("well-typed redex");
    // J is measured at the reducer seam: subsequent WHNF of a Pi cast
    // can produce a checking-only lambda, which cannot be inferred alone.
    let after_term = if let Term::J(motive, base, evidence) = redex {
        ken_kernel::obs::j_reduce(env, &ctx, motive, base, evidence).expect("J must reduce")
    } else {
        whnf(env, &ctx, redex)
    };
    assert_ne!(&after_term, redex, "site must reduce rather than stay neutral");
    let after = infer(env, &ctx, &after_term).unwrap_or_else(|err| panic!("{shape}: reduct lost type: {err:?}; reduct: {after_term:?}"));
    assert!(convert_type(env, &ctx, &before, &after), "reduct inferred {after:?}, redex {before:?}");
}

/// Durable invariant: the first equality conjunct witnesses the dependent
/// Sigma codomain cast; each row compares the inferred before/after types.
#[test]
fn sigma_dependent_reduct_has_typed_witness() {
    for shape in ["vec", "pi", "sigma"] {
        let mut f = Fixture::new();
        let n = f.opaque("n", f.nat.clone());
        let m = f.opaque("m", f.nat.clone());
        let family = f.family(shape);
        let sigma = Term::sigma(f.nat.clone(), family.clone());
        let x = f.opaque("x", ken_kernel::subst::subst0(&family, &n));
        let y = f.opaque("y", ken_kernel::subst::subst0(&family, &m));
        let p = Term::Ascript(Box::new(Term::pair(n, x)), Box::new(sigma.clone()));
        let q = Term::Ascript(Box::new(Term::pair(m, y)), Box::new(sigma.clone()));
        assert_typed_reduction(&f.env, &eq(sigma, p, q), shape);
    }
}

/// Durable invariant: a later constructor field transports along the
/// accumulated earlier argument equality instead of a fabricated Refl.
#[test]
fn inductive_dependent_telescope_has_typed_witness() {
    for shape in ["vec", "pi", "sigma"] {
        let mut f = Fixture::new();
        let n = f.opaque("n", f.nat.clone());
        let m = f.opaque("m", f.nat.clone());
        let family = f.family(shape);
        let x = f.opaque("x", ken_kernel::subst::subst0(&family, &n));
        let y = f.opaque("y", ken_kernel::subst::subst0(&family, &m));
        let d = declare_inductive(&mut f.env, |_| InductiveSpec {
            level_params: vec![], params: vec![], indices: vec![], level: Level::zero(),
            constructors: vec![CtorSpec { args: vec![f.nat.clone(), family], target_indices: vec![] }],
        }).unwrap();
        let ctor = f.env.inductive(d).unwrap().constructors[0].id;
        let mk = |a: Term, b: Term| Term::app(Term::app(Term::constructor(ctor, vec![]), a), b);
        assert_typed_reduction(&f.env, &eq(Term::indformer(d, vec![]), mk(n, x), mk(m, y)), shape);
    }
}

/// Durable invariant: two earlier conjuncts contribute to the dependent
/// third field's transport. The middle field is dependent itself, so the
/// accumulated proof must respect its preceding cast, not just tuple size.
#[test]
fn inductive_three_field_prefix_uses_both_earlier_conjuncts() {
    for shape in ["vec", "pi", "sigma"] {
        let mut f = Fixture::new();
        let n = f.opaque("n", f.nat.clone());
        let m = f.opaque("m", f.nat.clone());
        let a_mid = f.opaque("a-mid", f.vec(n.clone()));
        let b_mid = f.opaque("b-mid", f.vec(m.clone()));
        let family = f.family(shape);
        let a_end = f.opaque("a-end", ken_kernel::subst::subst0(&family, &n));
        let b_end = f.opaque("b-end", ken_kernel::subst::subst0(&family, &m));
        let third = ken_kernel::subst::weaken(&family, 1);
        let middle = f.vec(Term::var(0));
        let d = declare_inductive(&mut f.env, |_| InductiveSpec {
            level_params: vec![], params: vec![], indices: vec![], level: Level::zero(),
            constructors: vec![CtorSpec { args: vec![f.nat.clone(), middle, third], target_indices: vec![] }],
        }).unwrap();
        let ctor = f.env.inductive(d).unwrap().constructors[0].id;
        let mk = |a: Term, b: Term, c: Term| {
            Term::app(Term::app(Term::app(Term::constructor(ctor, vec![]), a), b), c)
        };
        assert_typed_reduction(&f.env, &eq(Term::indformer(d, vec![]),
            mk(n, a_mid, a_end), mk(m, b_mid, b_end)), shape);
    }
}

/// Durable invariant: non-refl J constructs pair-eq from its own e, with
/// both an opaque and a Pi/Sigma-valued dependent motive.
#[test]
fn j_nonrefl_dependent_motive_has_typed_pair_equality() {
    for shape in ["opaque", "pi", "sigma"] {
        let mut f = Fixture::new();
        let n = f.opaque("n", f.nat.clone());
        let m = f.opaque("m", f.nat.clone());
        let motive_ty = Term::pi(f.nat.clone(), Term::pi(
            eq(f.nat.clone(), n.clone(), Term::var(0)), Term::Type(Level::zero())
        ));
        let motive = if shape == "opaque" {
            f.opaque("P", motive_ty)
        } else {
            let result = ken_kernel::subst::weaken(&f.family(shape), 1);
            Term::Ascript(Box::new(Term::lam(f.nat.clone(), Term::lam(
                eq(f.nat.clone(), n.clone(), Term::var(0)), result
            ))), Box::new(motive_ty))
        };
        let base_ty = Term::app(Term::app(motive.clone(), n.clone()), Term::Refl(Box::new(n.clone())));
        let base = f.opaque("base", base_ty);
        let evidence = f.opaque("e", eq(f.nat.clone(), n, m));
        assert_typed_reduction(&f.env, &Term::J(Box::new(motive), Box::new(base), Box::new(evidence)), shape);
    }
}
