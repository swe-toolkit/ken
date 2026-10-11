//! A dependent match checks a stated Pair equality with the same `Refl`
//! meaning as a standalone equality, including nested match/let arms.
//! Promise class: durable invariant. Spec 10-kernel/16 §2.1 and §8.4;
//! spec 30-surface/34 §§1 and 4.4. The emitted proofs are kernel-checked.

use ken_elaborator::{ElabEnv, ElabError};

// This declaration workload shares the elaborator's deep prelude path. The
// documented unoptimized bare-prelude peak is 1,982,464 bytes and a nested
// dependent-match fixture measured a 3 MiB minimum viable thread stack;
// 8 MiB gives >4x that peak and >5 MiB margin over that nested minimum.
// The local Builder::stack_size provision is not a depth/stack assertion.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn on_elaboration_stack<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked")
}

const DFA_PREFIX: &str = "
    data Dfa q a = MkDfa (q -> a -> q) q (q -> Bool)
    fn step (q : Type) (a : Type) (d : Dfa q a) (s : q) (x : a) : q =
      match d { MkDfa transition initial accepting ↦ transition s x }
    fn run (q : Type) (a : Type) (d : Dfa q a) (s : q) (w : List a) : q =
      match w { Nil ↦ s; Cons x rest ↦ run q a d (step q a d s x) rest }
    fn product (q : Type) (r : Type) (a : Type)
        (combine : Bool -> Bool -> Bool) (d : Dfa q a) (e : Dfa r a)
        : Dfa (Pair q r) a =
      MkDfa (Pair q r) a
        (λp. λx. mk_pair q r (step q a d (pair_fst q r p) x)
                            (step r a e (pair_snd q r p) x))
        (mk_pair q r (start q a d) (start r a e))
        (λp. combine (final q a d (pair_fst q r p))
                     (final r a e (pair_snd q r p)))
    fn start (q : Type) (a : Type) (d : Dfa q a) : q =
      match d { MkDfa transition initial accepting ↦ initial }
    fn final (q : Type) (a : Type) (d : Dfa q a) (s : q) : Bool =
      match d { MkDfa transition initial accepting ↦ accepting s }
";

fn run_product_source(nil_body: &str) -> String {
    format!(
        "{DFA_PREFIX}
      theorem run_product
        (q : Type) (r : Type) (a : Type)
        (combine : Bool -> Bool -> Bool) (d : Dfa q a) (e : Dfa r a)
        (s1 : q) (s2 : r) (w : List a)
        : Equal (Pair q r)
            (run (Pair q r) a (product q r a combine d e)
                 (mk_pair q r s1 s2) w)
            (mk_pair q r (run q a d s1 w) (run r a e s2 w)) =
      match w {{
        Nil ↦ {nil_body};
        Cons x rest ↦ run_product q r a combine d e
          (step q a d s1 x) (step r a e s2 x) rest
      }}"
    )
}

fn check_trust_free(source: &str) {
    let source = source.to_owned();
    on_elaboration_stack(move || {
        let mut env = ElabEnv::new().expect("prelude");
        let trust_before = env.env.trusted_base();
        env.elaborate_file(&source)
            .unwrap_or_else(|error| panic!("kernel-checking fixture: {error:?}"));
        assert_eq!(env.env.trusted_base(), trust_before);
    });
}

#[test]
fn product_run_nil_arm_checks_refl_at_the_stated_pair_equality() {
    // MEASURED: the full recursive product-run theorem checks a Nil Refl.
    // CLAIMED: a match arm keeps the stated Pair equality's proof shape.
    // THE GAP: this fixture takes the real dependent match/branch simplifier
    // and its generated proof through kernel checking, not a helper oracle.
    check_trust_free(&run_product_source("Refl"));
}

#[test]
fn nested_nil_arm_checks_the_same_stated_pair_equality() {
    // MEASURED: a second, neutral Bool match surrounds both Refl leaves.
    // CLAIMED: nested arms inherit the equality-preserving simplification.
    // THE GAP: checking only a direct Refl would not reach this match route.
    check_trust_free(&run_product_source(
        "match combine True True { True ↦ Refl; False ↦ Refl }",
    ));
}

#[test]
fn let_arm_checks_the_stated_pair_equality() {
    // MEASURED: a let in the Cons arm passes a convertible Pair goal to Refl.
    // CLAIMED: branch simplification is not limited to a direct Refl body.
    // THE GAP: this takes the ordinary List match with a let-owned leaf.
    check_trust_free(
        "theorem pair_arm (a : Nat) (b : Nat) (xs : List Nat)
           : Equal (Pair Nat Nat)
             (mk_pair Nat Nat a b) (mk_pair Nat Nat a b) =
         match xs { Nil ↦ Refl; Cons x rest ↦ let y = x in Refl }",
    );
}

#[test]
fn false_pair_goal_rejects_nonconvertible_sides() {
    // MEASURED: swapping symbolic Pair components refuses at Refl's
    // not-convertible guard, rather than at the Eq-shape or Internal guard.
    // CLAIMED: retaining equality origin cannot make false proof acceptable.
    // THE GAP: the opposite observable is an exact diagnostic reason, not
    // merely an error; this fixture shares the accepted Pair carrier form.
    on_elaboration_stack(|| {
        let mut env = ElabEnv::new().expect("prelude");
        let error = env
            .elaborate_file(
                "theorem pair_arm_false (a : Nat) (b : Nat) (xs : List Nat)
               : Equal (Pair Nat Nat)
                 (mk_pair Nat Nat a b) (mk_pair Nat Nat b a) =
             match xs { Nil ↦ Refl; Cons x rest ↦ Refl }",
            )
            .expect_err("distinct symbolic components cannot check by Refl");
        assert!(
            matches!(error, ElabError::TypeMismatch { ref reason, .. }
            if reason == "Refl: the two sides of the goal are not convertible"),
            "must reject for non-convertibility, got {error:?}"
        );
    });
}

#[test]
fn non_pair_equal_nat_list_match_still_checks() {
    // MEASURED: standalone and both arms of a symbolic Nat equality check.
    // CLAIMED: the Pair goal repair preserves the non-Pair path.
    // THE GAP: this is one Nat/neutral fixture, not a census of Nat cases.
    check_trust_free(
        "fn preserve (s : Nat) (xs : List Nat) : Nat =
           match xs { Nil ↦ s; Cons x rest ↦ s }
         theorem preserve_nil_standalone (s : Nat)
           : Equal Nat (preserve s (Nil Nat)) s = Refl
         theorem preserve_match (s : Nat) (xs : List Nat)
           : Equal Nat (preserve s xs) s =
           match xs { Nil ↦ Refl; Cons x rest ↦ Refl }",
    );
}
