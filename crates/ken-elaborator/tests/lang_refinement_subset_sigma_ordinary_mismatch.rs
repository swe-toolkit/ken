//! An introduction carrier mismatch is judged by the checked pair;
//! an ordinary mismatch after forgetting is judged at final admission.

use ken_elaborator::{ElabEnv, ElabError, Span};
use ken_kernel::{KernelError, Term};

// Promise class: durable invariant. MEASURED: a Nat cannot enter a Bool
// subset, and the kernel reports its actual expected/found carrier heads.
// CLAIMED: introducing a Nat into a Bool subset reaches a site-local kernel
// refusal, while ordinary misses retain final-declaration localization.
// THE GAP: this row cannot distinguish the carrier, goal and pair checks:
// removing either the carrier or pair check leaves this payload unchanged.
#[test]
fn wrong_subset_carrier_is_kernel_refused_with_nat_bool_payload() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let source = "fn f (n : Nat) : { x : Bool | Equal Bool x True } = n";
    let before_id = env.env.next_global_id();
    let before_trust = env.env.trusted_base();
    let error = env
        .elaborate_decl_v1(source)
        .expect_err("Nat cannot be introduced into a Bool subset");
    let expected_bool = env.globals["Bool"];
    let found_nat = env.globals["Nat"];
    assert!(
        matches!(&error, ElabError::KernelRejected {
            error: KernelError::TypeMismatch { expected, found }, span
        } if *span == Span::new(source.len() - 1, source.len())
            && matches!(expected.as_ref(), Term::IndFormer { id, .. } if *id == expected_bool)
            && matches!(found.as_ref(), Term::IndFormer { id, .. } if *id == found_nat)),
        "kernel must report Bool expected and Nat found at the bad carrier: {error:?}"
    );
    assert!(!env.globals.contains_key("f"));
    assert_eq!(env.env.next_global_id(), before_id);
    assert_eq!(env.env.trusted_base(), before_trust);
}

// Promise class: durable invariant. MEASURED: forgetting a Nat subset to a
// Bool result is refused by the kernel at the whole declaration. CLAIMED:
// forgetting does not introduce a new subset pair or site-localize the
// mismatch. THE GAP: checking the projected value inside coercion narrows
// the span to the body rather than the final declaration.
#[test]
fn ordinary_forget_mismatch_is_localized_to_whole_declaration() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let source = "fn g (p : { x : Nat | Equal Nat x Zero }) : Bool = p";
    let before_id = env.env.next_global_id();
    let before_trust = env.env.trusted_base();
    let error = env
        .elaborate_decl_v1(source)
        .expect_err("a Nat subset cannot be forgotten to Bool");
    let expected_bool = env.globals["Bool"];
    let found_nat = env.globals["Nat"];
    assert!(
        matches!(&error, ElabError::KernelRejected {
            error: KernelError::TypeMismatch { expected, found }, span
        } if *span == Span::new(0, source.len())
            && matches!(expected.as_ref(), Term::IndFormer { id, .. } if *id == expected_bool)
            && matches!(found.as_ref(), Term::IndFormer { id, .. } if *id == found_nat)),
        "ordinary forgetting must have whole-declaration kernel localization: {error:?}"
    );
    assert!(!env.globals.contains_key("g"));
    assert_eq!(env.env.next_global_id(), before_id);
    assert_eq!(env.env.trusted_base(), before_trust);
}
