//! An annotated let retains the type of an introduction-valued RHS when the
//! kernel substitutes it into an inferable body. Promise class: durable
//! admission/refusal invariant. Spec 39 §5.4, Architect D0 evt_4s2vxf2saxvy0.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, Term};

// The sibling DualEnv fixture family measured a 1,982,464-byte prelude peak;
// 8 MiB gives more than fourfold headroom for its nested-match cases.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn on_elaboration_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked");
}

fn accepts(env: &mut ElabEnv, source: &str) -> Term {
    let trust = env.env.trusted_base();
    let result = env
        .elaborate_decl_v1(source)
        .unwrap_or_else(|error| panic!("{source}: {error:?}"));
    assert_eq!(env.env.trusted_base(), trust, "accepted let added trust");
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(result.def_id) else {
        panic!("accepted declaration has no checked body: {source}");
    };
    body.clone()
}

fn refuses_typed(env: &mut ElabEnv, source: &str) {
    let trust = env.env.trusted_base();
    let error = env
        .elaborate_decl(source)
        .expect_err("wrong annotation or application must refuse");
    assert!(
        matches!(
            error,
            ElabError::TypeMismatch { .. }
                | ElabError::KernelRejected { .. }
                | ElabError::LambdaVsNonFunction { .. }
        ),
        "must refuse by a type error: {error:?}"
    );
    assert_eq!(env.env.trusted_base(), trust, "refusal added trust");
}

/// MEASURED: both applying the checked lambda and using it without application
/// kernel-check; the stored introduction value alone gains its checked type.
/// CLAIMED: the substituted introduction remains inferable and an ordinary
/// non-introduction let remains unchanged. THE GAP: the dependent nested-match
/// coordinate is exercised separately below.
#[test]
fn root_introduction_value_is_ascribed_only_when_needed() {
    on_elaboration_stack(|| {
        let mut env = ElabEnv::new().expect("prelude");
        accepts(
            &mut env,
            "fn intro_nomatch (x : Nat) : Nat = let f : (u : Nat) → Nat = λu. x in f Zero",
        );
        let body = accepts(
            &mut env,
            "fn intro_unapplied (x : Nat) : (u : Nat) → Nat = let f : (u : Nat) → Nat = λu. x in f",
        );
        assert!(
            matches!(body, Term::Lam(_, inner) if matches!(&*inner, Term::Let { val, .. } if matches!(&**val, Term::Ascript(_, _))))
        );
        let ordinary = accepts(
            &mut env,
            "fn intro_nonintro (x : Nat) : Nat = let v : Nat = x in v",
        );
        assert!(
            matches!(ordinary, Term::Lam(_, inner) if matches!(&*inner, Term::Let { val, .. } if matches!(&**val, Term::Var(_))))
        );
        accepts(
            &mut env,
            "fn intro_unannotated (x : Nat) : Nat = let v = x in v",
        );
        accepts(
            &mut env,
            "theorem intro_refl (x : Nat) : Equal Nat x x = let p : Equal Nat x x = Refl in p",
        );
        let trust = env.env.trusted_base();
        let error = env
            .elaborate_decl("fn intro_unannotated_lambda (x : Nat) : Nat = let f = λu. x in f Zero")
            .expect_err("an unannotated lambda still needs a checking type");
        assert!(
            matches!(&error, ElabError::TypeMismatch { reason, .. } if reason.contains("cannot infer type of lambda")),
            "wrong unannotated-lambda refusal: {error:?}"
        );
        assert_eq!(env.env.trusted_base(), trust, "refusal added trust");
        refuses_typed(
            &mut env,
            "fn intro_wrong_result (x : Nat) : Nat = let f : (u : Nat) → Nat = λu. x in f",
        );
        refuses_typed(
            &mut env,
            "fn intro_wrong_arg (x : Nat) : Nat = let f : (u : Nat) → Nat = λu. x in f (λu. u)",
        );
    });
}

/// MEASURED: a nested-let tail lambda and an inferred let in application-head
/// position both check. CLAIMED: the annotation survives either Let emission
/// site, including a tail introduction. THE GAP: these are one-level tails;
/// deeper nested chains follow the same recursive branch of the predicate.
#[test]
fn let_chain_tail_and_infer_site_keep_the_introduction_type() {
    on_elaboration_stack(|| {
        let mut env = ElabEnv::new().expect("prelude");
        accepts(
            &mut env,
            "fn intro_chain (x : Nat) : Nat = let f : (u : Nat) → Nat = (let c : Nat = x in λu. c) in f Zero",
        );
        let body = accepts(
            &mut env,
            "fn intro_infer_site (x : Nat) : Nat = (let f : (u : Nat) → Nat = λu. x in f) Zero",
        );
        assert!(
            matches!(body, Term::Lam(_, inner) if matches!(&*inner, Term::App(head, _) if matches!(&**head, Term::Let { val, .. } if matches!(&**val, Term::Ascript(_, _)))))
        );
    });
}

/// MEASURED: no moved-field repair is required for the DualFZ arm, while r2pi
/// exercises a nested moved-field let used in function position. CLAIMED:
/// ascription repairs the introduction-form failure without new trust. THE
/// GAP: the existing LAM acceptance suite independently checks R1/R2 and
/// their false twins.
#[test]
fn nested_branch_introduction_and_r2pi_check_without_new_trust() {
    on_elaboration_stack(|| {
        let mut env = ElabEnv::new().expect("prelude");
        for fixture in [
            "data DualFin : Nat -> Type where { DualFZ : (n : Nat) -> DualFin (Suc n); DualFS : (n : Nat) -> DualFin n -> DualFin (Suc n) }",
            "data DualEnv (a : Type) : Nat -> Type where { DualNil : DualEnv a Zero; DualCons : (n : Nat) -> a -> DualEnv a n -> DualEnv a (Suc n) }",
            "fn dual_lookup (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : a = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ x }; DualFS m j ↦ match xs { DualCons k x rest ↦ dual_lookup a m rest j } }",
        ] {
            let trust = env.env.trusted_base();
            env.elaborate_decl(fixture)
                .unwrap_or_else(|error| panic!("fixture {fixture}: {error:?}"));
            assert_eq!(env.env.trusted_base(), trust, "fixture added trust");
        }
        accepts(&mut env, "theorem intro_fzpi (a : Type) (pp : a → Prop) (hp : (x : a) → pp x) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : pp (dual_lookup a n xs i) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ let f : (u : Nat) → pp x = λu. hp x in f Zero }; DualFS m j ↦ match xs { DualCons k x rest ↦ hp (dual_lookup a m rest j) } }");
        accepts(&mut env, "theorem intro_r2pi (a : Type) (pp : a → Prop) (hp : (x : a) → pp x) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : pp (dual_lookup a n xs i) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ hp x }; DualFS m j ↦ match xs { DualCons k x rest ↦ let f : (u : Nat) → pp (dual_lookup a m rest j) = λu. hp (dual_lookup a m rest j) in f Zero } }");
    });
}
