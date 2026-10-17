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

/// MEASURED: applying an ascribed λ and an ascribed let-tail λ checks, and
/// the kernel-admitted application heads retain their checked ascriptions.
/// CLAIMED: the inferred head is kernel-inferable even after leaving the
/// ascription producer. THE GAP: other infer-position consumers and all
/// introduction forms are not exhaustively instantiated here.
#[test]
fn ascribed_lambda_and_let_tail_apply_in_infer_position() {
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
/// postulate. CLAIMED: the ruled predicate covers proof introductions as well
/// as lambdas. THE GAP: pair/quotient/truncation introduction forms are reached
/// through the same existing helper but not individually exercised here.
#[test]
fn ascribed_refl_application_proves_without_new_trust() {
    let mut env = ElabEnv::new().expect("prelude");
    checked(
        &mut env,
        "theorem u3 (x : Nat) : Equal Nat x x = ((λu. Refl) : (u : Nat) → Equal Nat x x) Zero",
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
