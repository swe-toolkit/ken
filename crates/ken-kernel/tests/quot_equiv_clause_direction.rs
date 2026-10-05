//! Quot-Form `IsEquiv` clause direction (`16 §5`). The relation is iff over
//! an open predicate, so `R x y` and `R y x` are distinct propositions and a
//! witness for one clause cannot stand in for another.

use ken_kernel::env::Context;
use ken_kernel::term::{Level, Term};
use ken_kernel::{infer, GlobalEnv, KernelError};

fn v(i: usize) -> Term {
    Term::var(i)
}

/// `Σ (P u → P v) (P v → P u)` with `P`, `u`, `v` at the given indices.
fn iff(p: usize, u: usize, w: usize) -> Term {
    Term::sigma(
        Term::pi(Term::app(v(p), v(u)), Term::app(v(p + 1), v(w + 1))),
        Term::pi(
            Term::app(v(p + 1), v(w + 1)),
            Term::app(v(p + 2), v(u + 2)),
        ),
    )
}

/// Context `[A : Type 0, P : A → Ω 0]`.
fn open_ctx() -> Context {
    let mut ctx = Context::new();
    ctx.push(Term::Type(Level::zero()));
    ctx.push(Term::pi(v(0), Term::Omega(Level::zero())));
    ctx
}

/// `R := λ x y. P x ↔ P y`, at `[A, P]`.
fn relation() -> Term {
    Term::Ascript(
        Box::new(Term::lam(v(1), Term::lam(v(2), iff(2, 1, 0)))),
        Box::new(Term::pi(v(1), Term::pi(v(2), Term::Omega(Level::zero())))),
    )
}

/// `λ x. (λ p. p, λ p. p) : Π x. R x x`.
fn reflexive() -> Term {
    let id = Term::lam(Term::app(v(1), v(0)), v(0));
    Term::lam(v(1), Term::pair(id.clone(), id))
}

/// `λ x y h. (h.2, h.1) : Π x y. R x y → R y x`.
fn symmetric() -> Term {
    Term::lam(
        v(1),
        Term::lam(
            v(2),
            Term::lam(
                iff(2, 1, 0),
                Term::pair(Term::proj2(v(0)), Term::proj1(v(0))),
            ),
        ),
    )
}

/// `λ x y h. h : Π x y. R x y → R x y`, the unswapped clause.
fn symmetric_unswapped() -> Term {
    Term::lam(v(1), Term::lam(v(2), Term::lam(iff(2, 1, 0), v(0))))
}

fn transitive_with(body: Term) -> Term {
    Term::lam(
        v(1),
        Term::lam(
            v(2),
            Term::lam(v(3), Term::lam(iff(3, 2, 1), Term::lam(iff(4, 2, 1), body))),
        ),
    )
}

/// `λ x y z h1 h2. (λ p. h2.1 (h1.1 p), λ q. h1.2 (h2.2 q))
///   : Π x y z. R x y → R y z → R x z`.
fn transitive() -> Term {
    transitive_with(Term::pair(
        Term::lam(
            Term::app(v(5), v(4)),
            Term::app(Term::proj1(v(1)), Term::app(Term::proj1(v(2)), v(0))),
        ),
        Term::lam(
            Term::app(v(5), v(2)),
            Term::app(Term::proj2(v(2)), Term::app(Term::proj2(v(1)), v(0))),
        ),
    ))
}

/// `λ x y z h1 h2. h1 : Π x y z. R x y → R y z → R x y`.
fn transitive_first_premise() -> Term {
    transitive_with(v(1))
}

fn quot_with(sym: Term, trans: Term) -> Term {
    Term::Quot(
        Box::new(v(1)),
        Box::new(relation()),
        Box::new(Term::pair(reflexive(), Term::pair(sym, trans))),
    )
}

#[test]
fn iff_equivalence_with_real_clause_proofs_forms_a_quotient() {
    let env = GlobalEnv::new();
    assert_eq!(
        infer(&env, &open_ctx(), &quot_with(symmetric(), transitive())),
        Ok(Term::Type(Level::zero()))
    );
}

#[test]
fn unswapped_symmetry_witness_is_refused() {
    let env = GlobalEnv::new();
    assert!(matches!(
        infer(&env, &open_ctx(), &quot_with(symmetric_unswapped(), transitive())),
        Err(KernelError::TypeMismatch { .. })
    ));
}

#[test]
fn first_premise_transitivity_witness_is_refused() {
    let env = GlobalEnv::new();
    assert!(matches!(
        infer(
            &env,
            &open_ctx(),
            &quot_with(symmetric(), transitive_first_premise())
        ),
        Err(KernelError::TypeMismatch { .. })
    ));
}
