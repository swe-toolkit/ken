//! An ordinary carrier mismatch at a subset introduction is judged by the
//! kernel in the current context, not by an elaborator-only reason string.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{KernelError, Term};

// Promise class: durable invariant. MEASURED: a Nat cannot enter a Bool
// subset, and the kernel reports its actual expected/found carrier heads.
// CLAIMED: the terminal ordinary-conversion arm keeps kernel authority even
// on the recursive carrier leg of a subset introduction. THE GAP: changing
// that arm to an elaborator TypeMismatch removes this exact kernel payload.
#[test]
fn wrong_subset_carrier_is_kernel_refused_with_nat_bool_payload() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let before_id = env.env.next_global_id();
    let before_trust = env.env.trusted_base();
    let error = env
        .elaborate_decl_v1("fn f (n : Nat) : { x : Bool | Equal Bool x True } = n")
        .expect_err("Nat cannot be introduced into a Bool subset");
    let expected_bool = env.globals["Bool"];
    let found_nat = env.globals["Nat"];
    assert!(
        matches!(&error, ElabError::KernelRejected {
            error: KernelError::TypeMismatch { expected, found }, ..
        } if matches!(expected.as_ref(), Term::IndFormer { id, .. } if *id == expected_bool)
            && matches!(found.as_ref(), Term::IndFormer { id, .. } if *id == found_nat)),
        "kernel must report Bool expected and Nat found: {error:?}"
    );
    assert!(!env.globals.contains_key("f"));
    assert_eq!(env.env.next_global_id(), before_id);
    assert_eq!(env.env.trusted_base(), before_trust);
}
