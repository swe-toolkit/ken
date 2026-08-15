//! Controls for the `23 §3.2` open-`Int` fragment-D witness route.

use ken_elaborator::{
    attempt_d_with_int_assignment, attempt_obligation, classify,
    error::Span,
    extract::{ObligationId, ObligationTriple, ProvKind, Provenance},
    prover::{Route, Verdict},
    ElabEnv,
};
use ken_kernel::{Level, Term};
use num_bigint::BigInt;

fn triple(elab: &mut ElabEnv, id: &str, goal: Term) -> ObligationTriple {
    ObligationTriple {
        id: ObligationId(id.into()),
        hole_id: elab.env.fresh_id(),
        context: vec![],
        phi: goal.clone(),
        goal_closed: goal,
        provenance: Provenance {
            kind: ProvKind::Prove,
            span: Span::zero(),
        },
    }
}

fn pi_int_equality(elab: &ElabEnv) -> Term {
    let int_ty = Term::const_(elab.numeric_env.int_id, vec![]);
    Term::pi(
        int_ty.clone(),
        Term::Eq(
            Box::new(int_ty),
            Box::new(Term::var(0)),
            Box::new(Term::IntLit(BigInt::from(0))),
        ),
    )
}

fn pi_int_inequality(elab: &ElabEnv) -> Term {
    let int_ty = Term::const_(elab.numeric_env.int_id, vec![]);
    let bool_ty = Term::indformer(elab.numeric_env.bool_id, vec![]);
    let leq_int = Term::const_(elab.globals["leq_int"], vec![]);
    Term::pi(
        int_ty,
        Term::Eq(
            Box::new(bool_ty),
            Box::new(Term::app(
                Term::app(leq_int, Term::var(0)),
                Term::IntLit(BigInt::from(0)),
            )),
            Box::new(Term::constructor(elab.numeric_env.bool_true_id, vec![])),
        ),
    )
}

/// Promise class: routing completeness.
///
/// MEASURED: both `23 §3.2` shapes formerly routed HO and emitted one trusted
/// hole. CLAIMED: their explicit registered-Int telescope now routes D while
/// an unrelated unrecognized shape still reaches the HO default. THE GAP: a
/// search adapter remains responsible for proposing assignments.
#[test]
fn pi_bound_int_atoms_route_d_and_preserve_ho_default() {
    let mut elab = ElabEnv::new().expect("numeric environment");
    let equality = pi_int_equality(&elab);
    let inequality = pi_int_inequality(&elab);

    assert_eq!(classify(&elab.env, &equality), Route::D);
    assert_eq!(classify(&elab.env, &inequality), Route::D);

    let unrecognized = Term::lam(Term::omega(Level::zero()), Term::var(0));
    assert_eq!(classify(&elab.env, &unrecognized), Route::HO);

    for (id, goal) in [("equality", equality), ("inequality", inequality)] {
        let obligation = triple(&mut elab, id, goal);
        let before = elab.env.trusted_base().len();
        let result = attempt_obligation(&mut elab.env, &obligation);
        let Verdict::Unknown { hole_id } = result.verdict else {
            panic!("D route without a candidate must remain Unknown");
        };
        let after = elab.env.trusted_base();
        assert_eq!(after.len(), before + 1);
        assert!(after.contains(&hole_id));
    }
}

/// Promise class: kernel-checked refutation boundary.
///
/// MEASURED: candidate `1` makes `x = 0` reduce to Bottom and candidate `0`
/// does not. CLAIMED: only the kernel-accepted candidate returns Disproved;
/// the bad candidate returns Unknown with one honest hole. THE GAP: candidate
/// discovery is deliberately outside this solver-agnostic seam.
#[test]
fn assignment_is_substituted_before_kernel_checked_refutation() {
    let mut elab = ElabEnv::new().expect("numeric environment");
    let goal = pi_int_equality(&elab);
    let obligation = triple(&mut elab, "candidate", goal);

    let before_good = elab.env.trusted_base().len();
    let good = attempt_d_with_int_assignment(&mut elab.env, &obligation, &[BigInt::from(1)]);
    assert!(matches!(good, Verdict::Disproved { .. }));
    assert_eq!(elab.env.trusted_base().len(), before_good);

    let before_bad = elab.env.trusted_base().len();
    let bad = attempt_d_with_int_assignment(&mut elab.env, &obligation, &[BigInt::from(0)]);
    let Verdict::Unknown { hole_id } = bad else {
        panic!("non-refuting candidate must be Unknown");
    };
    let after_bad = elab.env.trusted_base();
    assert_eq!(after_bad.len(), before_bad + 1);
    assert!(after_bad.contains(&hole_id));
}
