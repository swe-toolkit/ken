//! LANG-INDEX-REFINEMENT-POSITIONAL: independent constructor indices whose
//! values coincide must remain in distinct positions in a dependent match.

use ken_elaborator::{ElabEnv, ElabError};

fn mat_identity(first_index: &str, second_index: &str, body: &str) -> Result<(), ElabError> {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_decl(&format!(
        "data Mat (a : Type) : Nat -> Nat -> Type where {{ \
           MNil : Mat a {first_index} {second_index}; \
           MCons : (r1 : Nat) -> (k1 : Nat) -> a -> Mat a r1 k1 \
             -> Mat a (Suc r1) k1 \
         }}"
    ))?;
    env.elaborate_decl(&format!(
        "fn mat_identity (a : Type) (r : Nat) (k : Nat) \
           (m : Mat a r k) : Mat a r k = \
         match m {{ MNil |-> MNil a; \
           MCons r1 k1 x t |-> {body} \
         }}"
    ))?;
    Ok(())
}

#[test]
fn zero_zero_preserves_equal_sibling_index() {
    // Promise class: durable invariant. MEASURED: the identity match checks
    // at two equal constructor index values. CLAIMED: refining the first
    // index never captures the equal-valued second. THE GAP: the non-overlap
    // control cannot see this capture; this case and the site-only reversal do.
    mat_identity("Zero", "Zero", "MCons a r1 k1 x t")
        .expect("equal constructor indices must not alias positions");
}

#[test]
fn successor_zero_preserves_index_nested_in_sibling() {
    // Promise class: durable invariant. MEASURED: `MNil` has indices
    // `(Suc Zero, Zero)`, so the second index's `Zero` is a proper subterm
    // of its first-index sibling. CLAIMED: refinement does not cross index
    // positions, even at a proper subterm. THE GAP: the direct equal-value
    // fixture does not exercise descent.
    mat_identity("(Suc Zero)", "Zero", "MCons a r1 k1 x t")
        .expect("a subterm in another position must not be captured");
}

#[test]
fn distinct_sibling_index_remains_admitted() {
    // Promise class: durable invariant. MEASURED: the existing non-overlap
    // form checks. CLAIMED: the repair does not prevent ordinary refinement.
    // THE GAP: this is a control, not evidence for the overlap repair.
    mat_identity("Zero", "(Suc Zero)", "MCons a r1 k1 x t")
        .expect("unrelated second index must remain accepted");
}

#[test]
fn genuinely_wrong_constructor_arm_remains_rejected() {
    // Promise class: durable invariant. MEASURED: an arm swapping independent
    // constructor field indices is rejected. CLAIMED: positional retyping
    // cannot admit a body at the wrong family indices. THE GAP: assert the
    // kernel rejection class, not arbitrary parse or name errors.
    let failure = mat_identity("Zero", "(Suc Zero)", "MCons a k1 r1 x t")
        .expect_err("wrong indexed constructor arm must fail");
    assert!(
        matches!(failure, ElabError::KernelRejected { .. }),
        "{failure:?}"
    );
}
