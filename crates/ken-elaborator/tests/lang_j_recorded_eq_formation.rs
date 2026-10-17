//! Surface J retains the checked Eq formation when observational whnf
//! reduces an equality at a function or pair carrier (16 §2, 34 §3.4).
//! Promise class: durable invariant; all admitted proofs are kernel-checked.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{KernelError, Term};

fn admit_theorem(source: &str, name: &str) {
    let mut env = ElabEnv::new().expect("prelude");
    let before = env.env.trusted_base();
    let result = env
        .elaborate_decl_v1(source)
        .unwrap_or_else(|error| panic!("{name} must check: {error:?}"));
    assert_eq!(result.name, name);
    assert!(result.obligations.is_empty(), "{name}: unexpected hole");
    assert!(env.env.transparent_body(result.def_id).is_some(), "{name}: no checked body");
    assert_eq!(env.env.trusted_base(), before, "{name}: new trust");
    let (_, body) = env.env.transparent_body(result.def_id).unwrap();
    let mut pending = vec![&body];
    let mut j_count = 0;
    while let Some(node) = pending.pop() {
        if let Term::J(_, _, evidence) = node {
            j_count += 1;
            assert!(
                matches!(&**evidence, Term::Ascript(_, recorded)
                    if ken_kernel::recorded_eq_endpoints(&env.env, recorded).is_some()),
                "{name}: J evidence must carry a checked Eq recording: {evidence:?}"
            );
        }
        pending.extend(node.children());
    }
    assert!(j_count > 0, "{name}: no J in checked body");
}

#[test]
fn j_function_carrier_uses_recorded_equality() {
    // MEASURED: an abstract pointwise equality checks at its original Pi
    // carrier; CLAIMED: J reads the Eq formation, not its Pi reduct.
    // THE GAP: success alone would not show actual J emission, hence the
    // checked-body inspection above and QA's no-helper-read mutation.
    admit_theorem(
        "theorem jf (f g : Nat -> Nat) (h : Equal (Nat -> Nat) f g) \
            : Equal (Nat -> Nat) g f = J (\\b' _. Equal (Nat -> Nat) b' f) (\\x. Refl) h",
        "jf",
    );
}

#[test]
fn architect_jf_with_bare_refl_is_still_blocked() {
    // Diagnostic red baseline, not a candidate: after the recorded-J repair,
    // a bare Refl at a Pi equality is refused at the distinct Refl-intro gate.
    admit_theorem(
        "theorem jf_exact (f g : Nat -> Nat) (h : Equal (Nat -> Nat) f g) \
            : Equal (Nat -> Nat) g f = J (\\b' _. Equal (Nat -> Nat) b' f) Refl h",
        "jf_exact",
    );
}

#[test]
fn j_function_carrier_accepts_an_ascribed_witness() {
    admit_theorem(
        "theorem jfa (f g : Nat -> Nat) (h : Equal (Nat -> Nat) f g) \
            : Equal (Nat -> Nat) g f = \
            J (\\b' _. Equal (Nat -> Nat) b' f) (\\x. Refl) (h : Equal (Nat -> Nat) f g)",
        "jfa",
    );
}

#[test]
fn j_sigma_carrier_uses_recorded_equality() {
    admit_theorem(
        "theorem js (p q : Pair Nat Nat) (h : Equal (Pair Nat Nat) p q) \
            : Equal (Pair Nat Nat) q p = \
            J (\\q' _. Equal (Pair Nat Nat) q' p) Refl h",
        "js",
    );
}

#[test]
fn j_nat_carrier_control_stays_checked() {
    admit_theorem(
        "theorem jn (a b : Nat) (h : Equal Nat a b) \
            : Equal Nat b a = J (\\b' _. Equal Nat b' a) Refl h",
        "jn",
    );
}

#[test]
fn non_equality_witness_is_still_refused_without_trust() {
    let mut env = ElabEnv::new().expect("prelude");
    let before = env.env.trusted_base();
    let error = env
        .elaborate_decl_v1(
            "theorem jnot (n : Nat) : Equal Nat n n = \
                J (\\b' _. Equal Nat b' n) Refl n",
        )
        .expect_err("a Nat is not an equality witness");
    assert!(
        matches!(error, ElabError::TypeMismatch { ref reason, .. }
            if reason == "J's third argument must have an `Eq` type"),
        "wrong refusal: {error:?}"
    );
    assert_eq!(env.env.trusted_base(), before);
}

#[test]
fn mistyped_function_carrier_motive_is_rejected_by_kernel_without_trust() {
    let mut env = ElabEnv::new().expect("prelude");
    let before = env.env.trusted_base();
    let error = env
        .elaborate_decl_v1(
            "theorem jbad (n : Nat) (f g : Nat -> Nat) (h : Equal (Nat -> Nat) f g) \
                : Equal (Nat -> Nat) g f = \
                J (\\b' _. Equal Nat n n) Refl h",
        )
        .expect_err("wrong J result type must not be admitted");
    assert!(
        matches!(error, ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. }, ..
        }),
        "must reach kernel result-type checking: {error:?}"
    );
    assert_eq!(env.env.trusted_base(), before);
}
