//! Surface reproduction of proof-irrelevant Refl endpoints (`15 §2`,
//! `16 §1.2`, §8.2). Promise class: durable invariant.

use ken_elaborator::ElabEnv;

/// The constructor argument is a proof, but its two names are deliberately
/// distinct. The declaration must reach kernel check through real elaboration.
#[test]
fn positional_constructors_with_distinct_proofs_admit_refl() {
    let mut env = ElabEnv::new().expect("prelude");
    let before = env.env.trusted_base();
    env.elaborate_file(
        r#"
        data Pos : Type where { MkPos : (n : Nat) -> Equal Nat n n -> Pos }
        theorem pos_eq (n : Nat) (a : Equal Nat n n) (b : Equal Nat n n)
            : Equal Pos (MkPos n a) (MkPos n b) = Refl
        "#,
    )
    .expect("constructor proof arguments compare by typed Ω conversion");
    assert_eq!(env.env.trusted_base(), before);
}
