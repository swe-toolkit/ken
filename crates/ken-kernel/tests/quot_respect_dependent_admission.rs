//! C15: an executed Type-target quotient-eliminator admission test.
//! An opaque motive makes `M [x]` and `M [y]` genuinely different neutral
//! types. Correctly oriented transport is admitted; the reverse is refused.

use ken_kernel::env::Context;
use ken_kernel::obs::{top_term, tt_term};
use ken_kernel::subst::{apply_args, weaken};
use ken_kernel::{
    convert_type, declare_inductive, declare_postulate, infer, CtorSpec, GlobalEnv, InductiveSpec,
    KernelError, Level, Term,
};

fn as_const(id: ken_kernel::GlobalId) -> Term {
    Term::const_(id, vec![])
}

fn bool_type(env: &mut GlobalEnv) -> (Term, Term) {
    let id = declare_inductive(env, |_| InductiveSpec {
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
                args: vec![],
                target_indices: vec![],
            },
        ],
    })
    .unwrap();
    let true_ = Term::constructor(env.inductive(id).unwrap().constructors[0].id, vec![]);
    (Term::indformer(id, vec![]), true_)
}

fn total_equivalence(env: &GlobalEnv, a: &Term) -> Term {
    let top = top_term(env);
    let tt = tt_term(env);
    Term::pair(
        Term::lam(a.clone(), tt.clone()),
        Term::pair(
            Term::lam(
                a.clone(),
                Term::lam(a.clone(), Term::lam(top.clone(), tt.clone())),
            ),
            Term::lam(
                a.clone(),
                Term::lam(
                    a.clone(),
                    Term::lam(a.clone(), Term::lam(top.clone(), Term::lam(top, tt))),
                ),
            ),
        ),
    )
}

/// Symmetry of `Eq Type_l a b`, by equality elimination, not a fabricated
/// `Refl` between non-convertible motive applications.
fn symmetric_type_eq(level: &Level, a: &Term, b: &Term, evidence: Term) -> Term {
    let u = Term::Type(level.clone());
    let domain = Term::Eq(
        Box::new(u.clone()),
        Box::new(weaken(a, 1)),
        Box::new(Term::var(0)),
    );
    let motive = Term::Ascript(
        Box::new(Term::lam(
            u.clone(),
            Term::lam(
                domain.clone(),
                Term::Eq(
                    Box::new(u.clone()),
                    Box::new(Term::var(1)),
                    Box::new(weaken(a, 2)),
                ),
            ),
        )),
        Box::new(Term::pi(
            u.clone(),
            Term::pi(domain, Term::Omega(level.clone().suc())),
        )),
    );
    Term::J(
        Box::new(motive),
        Box::new(Term::Refl(Box::new(a.clone()))),
        Box::new(Term::Ascript(
            Box::new(evidence),
            Box::new(Term::Eq(
                Box::new(u),
                Box::new(a.clone()),
                Box::new(b.clone()),
            )),
        )),
    )
}

#[test]
fn type_target_respect_checks_correct_direction_at_a_genuinely_dependent_motive() {
    let mut env = GlobalEnv::new();
    let (carrier, true_) = bool_type(&mut env);
    let relation = Term::Ascript(
        Box::new(Term::lam(
            carrier.clone(),
            Term::lam(carrier.clone(), top_term(&env)),
        )),
        Box::new(Term::pi(
            carrier.clone(),
            Term::pi(carrier.clone(), Term::Omega(Level::zero())),
        )),
    );
    let quot = Term::Quot(
        Box::new(carrier.clone()),
        Box::new(relation.clone()),
        Box::new(total_equivalence(&env, &carrier)),
    );
    assert_eq!(
        infer(&env, &Context::new(), &quot),
        Ok(Term::Type(Level::zero()))
    );

    // Only fixture-level opaque assumptions: M and f are abstract so the
    // motive endpoints do NOT collapse by conversion; r_ok and r_bad are
    // independently typed proof inputs to the actual kernel admission gate.
    let initial_trust = env.trusted_base();
    let motive_id = declare_postulate(
        &mut env,
        "C15 abstract motive".into(),
        vec![],
        Term::pi(quot.clone(), Term::Type(Level::zero())),
    )
    .unwrap();
    let motive = as_const(motive_id);
    let method_id = declare_postulate(
        &mut env,
        "C15 abstract method".into(),
        vec![],
        Term::pi(
            carrier.clone(),
            Term::app(weaken(&motive, 1), Term::QuotClass(Box::new(Term::var(0)))),
        ),
    )
    .unwrap();
    let method = as_const(method_id);

    // Under x, y, h: x=@2, y=@1, h=@0. Rxy and Eq Quot [x] [y]
    // are convertible *because* quotient equality now reduces to Rxy.
    let related = apply_args(weaken(&relation, 2), &[Term::var(1), Term::var(0)]);
    let mut open = Context::new();
    open.push(carrier.clone());
    open.push(carrier.clone());
    open.push(related.clone());
    let x_class = Term::QuotClass(Box::new(Term::var(2)));
    let y_class = Term::QuotClass(Box::new(Term::var(1)));
    let q3 = weaken(&quot, 3);
    let class_eq = Term::Eq(
        Box::new(q3.clone()),
        Box::new(x_class.clone()),
        Box::new(y_class.clone()),
    );
    let h_prime = Term::Ascript(Box::new(Term::var(0)), Box::new(class_eq));
    let class_proof_type = infer(&env, &open, &h_prime).expect("derive class equality from h");
    assert!(convert_type(&env, &open, &class_proof_type, &related));
    let mx = Term::app(weaken(&motive, 3), x_class.clone());
    let my = Term::app(weaken(&motive, 3), y_class);
    assert!(!convert_type(&env, &open, &mx, &my));
    let fx = Term::app(weaken(&method, 3), Term::var(2));
    let fy = Term::app(weaken(&method, 3), Term::var(1));

    // Congruence by J over h'. In Γ,x,y,h,z,p, its goal is
    // Eq Type_0 (M[x]) (M[z]), and its base is Refl(M[x]).
    let proof_domain = Term::Eq(
        Box::new(weaken(&q3, 1)),
        Box::new(weaken(&x_class, 1)),
        Box::new(Term::var(0)),
    );
    let cong_motive = Term::Ascript(
        Box::new(Term::lam(
            q3.clone(),
            Term::lam(
                proof_domain.clone(),
                Term::Eq(
                    Box::new(Term::Type(Level::zero())),
                    Box::new(weaken(&mx, 2)),
                    Box::new(Term::app(weaken(&motive, 5), Term::var(1))),
                ),
            ),
        )),
        Box::new(Term::pi(
            q3,
            Term::pi(proof_domain, Term::Omega(Level::zero().suc())),
        )),
    );
    let cong = Term::J(
        Box::new(cong_motive),
        Box::new(Term::Refl(Box::new(mx.clone()))),
        Box::new(h_prime),
    );
    let eq_types = |a: Term, b: Term| {
        Term::Eq(
            Box::new(Term::Type(Level::zero())),
            Box::new(a),
            Box::new(b),
        )
    };
    let cong_ty = infer(&env, &open, &cong).expect("typed motive congruence");
    assert!(convert_type(
        &env,
        &open,
        &cong_ty,
        &eq_types(mx.clone(), my.clone())
    ));
    let sym = symmetric_type_eq(&Level::zero(), &mx, &my, cong.clone());
    let sym_ty = infer(&env, &open, &sym).expect("typed symmetric congruence");
    assert!(convert_type(
        &env,
        &open,
        &sym_ty,
        &eq_types(my.clone(), mx.clone())
    ));

    let ill_oriented = Term::Cast(
        Box::new(mx.clone()),
        Box::new(my.clone()),
        Box::new(cong.clone()),
        Box::new(fy.clone()),
    );
    assert!(matches!(
        infer(&env, &open, &ill_oriented),
        Err(KernelError::TypeMismatch { .. })
    ));

    let correct = Term::Eq(
        Box::new(mx.clone()),
        Box::new(fx.clone()),
        Box::new(Term::Cast(
            Box::new(my.clone()),
            Box::new(mx.clone()),
            Box::new(sym),
            Box::new(fy.clone()),
        )),
    );
    let reversed = Term::Eq(
        Box::new(my.clone()),
        Box::new(fy),
        Box::new(Term::Cast(
            Box::new(mx.clone()),
            Box::new(my),
            Box::new(cong),
            Box::new(fx),
        )),
    );
    let wrap = |body: Term| {
        Term::pi(
            carrier.clone(),
            Term::pi(weaken(&carrier, 1), Term::pi(related.clone(), body)),
        )
    };
    let ok_ty = wrap(correct);
    let bad_ty = wrap(reversed);
    let good_id = declare_postulate(&mut env, "C15 correct respect".into(), vec![], ok_ty)
        .expect("correct-direction schema itself must be well formed");
    let bad_id = declare_postulate(&mut env, "C15 reverse respect".into(), vec![], bad_ty)
        .expect("reverse-direction comparison is a well-formed, distinct type");
    let expected_trust = [motive_id, method_id, good_id, bad_id];
    let added = env
        .trusted_base()
        .into_iter()
        .filter(|id| !initial_trust.contains(id))
        .collect::<Vec<_>>();
    assert_eq!(added, expected_trust);
    let scrut = Term::Ascript(Box::new(Term::QuotClass(Box::new(true_))), Box::new(quot));
    let elim = |respect| Term::QuotElim {
        motive: Box::new(motive.clone()),
        method: Box::new(method.clone()),
        respect: Box::new(respect),
        scrut: Box::new(scrut.clone()),
    };
    assert!(infer(&env, &Context::new(), &elim(as_const(good_id))).is_ok());
    assert!(matches!(
        infer(&env, &Context::new(), &elim(as_const(bad_id))),
        Err(KernelError::TypeMismatch { .. })
    ));
}
