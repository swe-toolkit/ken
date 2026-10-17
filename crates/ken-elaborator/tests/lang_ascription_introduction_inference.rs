//! Infer-position ascriptions retain the checked type on introduction tails.
//! Promise class: durable admission/refusal and core-inference invariant.
//! Spec 39 §5.4; Architect ruling evt_50wngwa9saa0h.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError, Term};

fn checked(env: &mut ElabEnv, source: &str) -> Term {
    let trust = env.env.trusted_base();
    let result = env
        .elaborate_decl_v1(source)
        .unwrap_or_else(|error| panic!("{source}: {error:?}"));
    assert_eq!(env.env.trusted_base(), trust, "new trust: {source}");
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(result.def_id) else {
        panic!("checked declaration must have a body: {source}")
    };
    body.clone()
}

fn refused(env: &mut ElabEnv, source: &str) -> ElabError {
    let trust = env.env.trusted_base();
    let error = env.elaborate_decl(source).expect_err("ill-typed twin");
    assert_eq!(env.env.trusted_base(), trust, "new trust: {source}");
    error
}

/// MEASURED: applying an ascribed λ checks, and the kernel-admitted
/// application head retains its checked type. CLAIMED: infer-position RAsc
/// does not strand a bare λ at kernel inference. THE GAP: the let-tail
/// recurrence and other introduction forms are separate cases below.
#[test]
fn ascribed_lambda_applies_in_infer_position() {
    let mut env = ElabEnv::new().expect("prelude");
    let root = checked(
        &mut env,
        "fn u1 (x : Nat) : Nat = ((λu. x) : (u : Nat) → Nat) Zero",
    );
    assert!(
        matches!(root,
            Term::Lam(_, body) if matches!(&*body,
                Term::App(head, _) if matches!(&**head,
                    Term::Ascript(value, _) if matches!(&**value, Term::Lam(_, _))
                )
            )
        ),
        "the checked application keeps the head's type"
    );
}

/// MEASURED: a let-tail λ under RAsc checks as an application head with the
/// whole Let ascribed. CLAIMED: introTail recurses through Let bodies.
/// THE GAP: root introductions do not by themselves exercise this branch.
#[test]
fn ascribed_let_tail_lambda_applies_in_infer_position() {
    let mut env = ElabEnv::new().expect("prelude");
    let tail = checked(
        &mut env,
        "fn u2 (x : Nat) : Nat = ((let c : Nat = x in λu. c) : (u : Nat) → Nat) Zero",
    );
    assert!(
        matches!(tail,
            Term::Lam(_, body) if matches!(&*body,
                Term::App(head, _) if matches!(&**head,
                    Term::Ascript(value, _) if matches!(&**value,
                        Term::Let { body, .. } if matches!(&**body, Term::Lam(_, _))
                    )
                )
            )
        ),
        "the let body, not only a root lambda, controls retention"
    );
}

/// MEASURED: the adversary's exact Refl application checks with no additional
/// postulate. CLAIMED: a proof-returning λ is also accepted as an applied
/// ascription. THE GAP: its introduction tail is Lam, not Refl; the distinct
/// direct Refl/Pair/TruncProj terminals have their own rows below.
#[test]
fn ascribed_refl_application_proves_without_new_trust() {
    let mut env = ElabEnv::new().expect("prelude");
    checked(
        &mut env,
        "theorem u3 (x : Nat) : Equal Nat x x = ((λu. Refl) : (u : Nat) → Equal Nat x x) Zero",
    );
}

/// MEASURED: the pair's infer-position ascription remains visible under an
/// inferred projection and carries Pair as its checked core. CLAIMED: the pair
/// introTail arm is reachable at this producer. THE GAP: the pair's existing
/// infer helper also emits an inner ascription; this pins the outer retention.
#[test]
fn pair_projection_keeps_the_ascribed_introduction_head() {
    let mut env = ElabEnv::new().expect("prelude");
    let pair = checked(
        &mut env,
        "fn pair_head (x : Nat) : Nat = ((x, True) : (v : Nat) × Bool).1",
    );
    assert!(
        matches!(pair,
            Term::Lam(_, body) if matches!(&*body,
                Term::Proj1(pair) if matches!(&**pair,
                    Term::Ascript(inner, _) if matches!(&**inner,
                        Term::Ascript(value, _) if matches!(&**value, Term::Pair(_, _))
                    )
                )
            )
        ),
        "pair projection retains its checked type at inference"
    );
}

/// MEASURED: the existing polymorphic λ ascription now emits an outer
/// Ascript around the checked binary λ core in infer position. CLAIMED: the
/// older conformance case remains true after updating its output shape.
/// THE GAP: the applied-lambda rows separately require a later kernel infer.
#[test]
fn existing_polymorphic_lambda_ascription_keeps_its_checked_type() {
    let mut env = ElabEnv::new().expect("prelude");
    let trust = env.env.trusted_base();
    let (term, _) = env
        .elaborate_expr("poly_asc", "(\\A . \\x . x) : (A : Type) -> (x : A) -> A")
        .expect("existing polymorphic ascription");
    assert!(matches!(term,
        Term::Ascript(value, _) if matches!(&*value,
            Term::Lam(_, inner) if matches!(&**inner, Term::Lam(_, _))
        )
    ));
    assert_eq!(env.env.trusted_base(), trust);
}

/// MEASURED: an ascribed Refl under an open binder stays Ascript(Refl),
/// without an inferred-let value that could wrap its RHS a second time.
/// CLAIMED: Refl keeps its checked type at RAsc. THE GAP: the downstream
/// checker may also accept bare Refl when given an expected Eq type.
#[test]
fn open_refl_ascription_keeps_the_inferred_type() {
    let mut env = ElabEnv::new().expect("prelude");
    let refl = checked(
        &mut env,
        "theorem refl_direct (x : Nat) : Equal Nat x x = (Refl : Equal Nat x x)",
    );
    assert!(matches!(refl,
        Term::Lam(_, body) if matches!(&*body,
            Term::Ascript(value, _) if matches!(&**value, Term::Refl(_))
        )
    ));
    checked(
        &mut env,
        "theorem refl_let (x : Nat) : Equal Nat x x = let p = (Refl : Equal Nat x x) in p",
    );
}

/// MEASURED: standalone infer returns Ascript(TruncProj); an unannotated let
/// also admits the same source form. CLAIMED: the truncation introduction
/// terminal retains its checked type at RAsc. THE GAP: quotient-class
/// introductions have no corresponding source constructor on this base.
#[test]
fn standalone_truncation_ascription_keeps_the_inferred_type() {
    let mut env = ElabEnv::new().expect("prelude");
    let trust = env.env.trusted_base();
    let (truncation, _) = env
        .elaborate_expr("trunc_infer", "(trunc_intro True : ‖Bool‖)")
        .expect("standalone truncation in infer position");
    assert!(matches!(truncation,
        Term::Ascript(value, _) if matches!(&*value, Term::TruncProj(_))
    ));
    assert_eq!(env.env.trusted_base(), trust);
    let direct = checked(
        &mut env,
        "theorem trunc_direct (x : Bool) : ‖Bool‖ = (trunc_intro x : ‖Bool‖)",
    );
    assert!(matches!(direct,
        Term::Lam(_, body) if matches!(&*body,
            Term::Ascript(value, _) if matches!(&**value, Term::TruncProj(_))
        )
    ));
    checked(
        &mut env,
        "theorem trunc_let (x : Bool) : ‖Bool‖ = let p = (trunc_intro x : ‖Bool‖) in p",
    );
}

/// MEASURED: the two wrong-type twins reach the kernel's TypeMismatch rather
/// than the old infer-introduction refusal. CLAIMED: preserving an ascription
/// does not allow ill-typed introduction forms to pass. THE GAP: the kernel
/// checker, not this fixture, is the universal backstop for other types.
#[test]
fn wrong_type_twins_refuse_at_the_kernel() {
    let mut env = ElabEnv::new().expect("prelude");
    for source in [
        "fn w2 (x : Nat) : Nat = ((λu. True) : (u : Nat) → Nat) Zero",
        "fn w3 (x : Nat) : Bool = ((λu. x) : (u : Nat) → Nat) Zero",
    ] {
        let error = refused(&mut env, source);
        assert!(
            matches!(
                error,
                ElabError::KernelRejected {
                    error: KernelError::TypeMismatch { .. },
                    ..
                }
            ),
            "the wrong type must reach kernel checking: {error:?}"
        );
    }
}

/// MEASURED: the wrong argument still fails before kernel admission, while
/// an unapplied ascribed introduction and the old annotated-let path check.
/// CLAIMED: the producer repair does not broaden argument checking or replace
/// the existing let-value mechanism. THE GAP: this is a finite sibling set.
#[test]
fn existing_wrong_argument_and_let_controls_keep_their_boundaries() {
    let mut env = ElabEnv::new().expect("prelude");
    let error = refused(
        &mut env,
        "fn w1 (x : Nat) : Nat = ((λu. x) : (u : Nat) → Nat) (λv. v)",
    );
    assert!(
        matches!(error, ElabError::LambdaVsNonFunction { .. }),
        "wrong argument stays rejected at the old boundary: {error:?}"
    );
    checked(
        &mut env,
        "fn unapplied (x : Nat) : (u : Nat) → Nat = ((λu. x) : (u : Nat) → Nat)",
    );
    checked(
        &mut env,
        "fn let_control (x : Nat) : Nat = let f : (u : Nat) → Nat = λu. x in f Zero",
    );
}

/// MEASURED: an ascribed variable checks as a variable core, not Ascript.
/// CLAIMED: inference only retains the ascription for introduction tails.
/// THE GAP: the changed producer shares the already-landed introduction-tail
/// predicate; this control probes its negative branch on an ordinary term.
#[test]
fn non_introduction_ascription_still_erases_to_var() {
    let mut env = ElabEnv::new().expect("prelude");
    let body = checked(&mut env, "fn plain (x : Nat) : Nat = (x : Nat)");
    assert!(
        matches!(body, Term::Lam(_, body) if matches!(&*body, Term::Var(0))),
        "the non-introduction core stays Var"
    );
}
