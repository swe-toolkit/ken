//! Direct Eq-head conversion is ordinary conversion, judged by the final
//! kernel check. A mismatch must never be admitted or leave a declaration.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::KernelError;

fn kernel_mismatch_without_residue(env: &mut ElabEnv, source: &str, name: &str) {
    let before_id = env.env.next_global_id();
    let before_trust = env.env.trusted_base();
    let error = env
        .elaborate_decl_v1(source)
        .expect_err("mismatched Eq endpoints must refuse");
    assert!(
        matches!(
            error,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "the final kernel check must reject mismatched Eq endpoints: {error:?}"
    );
    assert!(!env.globals.contains_key(name));
    assert_eq!(env.env.next_global_id(), before_id);
    assert_eq!(env.env.trusted_base(), before_trust);
}

// Promise class: durable invariant. MEASURED: unequal Eq endpoints refuse at
// the final kernel check, while the same endpoints admit with no obligation.
// CLAIMED: direct-Eq-head ordinary conversion retains main's rejection class.
// THE GAP: removal of the Eq deferral changes the negative to an elaborator
// TypeMismatch, which the separate paired mutation must observe.
#[test]
fn eq_return_mismatch_reaches_kernel_and_matching_return_admits() {
    let mut env = ElabEnv::new().expect("checked prelude");
    kernel_mismatch_without_residue(
        &mut env,
        "theorem bogus (n : Int) (m : Int) (p : Eq Int n n) : Eq Int n m = p",
        "bogus",
    );
    let good = env
        .elaborate_decl_v1("theorem good (n : Int) (p : Eq Int n n) : Eq Int n n = p")
        .expect("matching Eq endpoints");
    assert!(good.obligations.is_empty());
}

// Promise class: durable invariant. MEASURED: a mismatched Eq argument is
// kernel-refused, while the matching call admits with no obligation. CLAIMED:
// deferral does not coerce an unequal proof at a function-call boundary.
// THE GAP: the rejection class depends on the Eq arm, not merely on refusal;
// the paired mutation must distinguish kernel from elaborator rejection.
#[test]
fn eq_argument_mismatch_reaches_kernel_and_matching_call_admits() {
    let mut env = ElabEnv::new().expect("checked prelude");
    env.elaborate_decl_v1("fn useab (a : Int) (b : Int) (p : Eq Int a b) (z : Int) : Int = z")
        .expect("ordinary function with an Eq argument");
    kernel_mismatch_without_residue(
        &mut env,
        "fn mismatch (n : Int) (m : Int) (p : Eq Int n n) : Int = useab n m p n",
        "mismatch",
    );
    env.elaborate_decl_v1("fn usenn (n : Int) (p : Eq Int n n) (z : Int) : Int = z")
        .expect("matching Eq argument consumer");
    let good = env
        .elaborate_decl_v1("fn good_call (n : Int) (p : Eq Int n n) : Int = usenn n p n")
        .expect("matching Eq argument");
    assert!(good.obligations.is_empty());
}
