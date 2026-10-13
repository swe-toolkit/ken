//! A nested match on two vectors checks a variable proposition motive.
//! Promise class: durable invariant. Spec 34 §3.5; 39 §2.1.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::KernelError;

// Adjacent dependent-match suites measured a 1,982,464-byte prelude peak and
// a 3 MiB minimum. The fixed 8 MiB worker leaves >4x the measured peak.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn on_elaboration_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked");
}

// The same indexed constructor telescope as catalog Data.Vector.Vector. No
// postulate or proof fixture supplies the missing equality between indices.
const VEC: &str = "data Vec (a : Type) : Nat -> Type where { VNil : Vec a Zero; VCons : (n : Nat) -> a -> Vec a n -> Vec a (Suc n) }";

const DIRECT: &str = r#"
theorem vec_pair_ind
  (a : Type) (b : Type)
  (motive : (n : Nat) → Vec a n → Vec b n → Prop)
  (on_nil : (ys : Vec b Zero) → motive Zero (VNil a) ys)
  (on_cons : (t : Nat) → (x : a) → (xs : Vec a t) → (y : b) → (ys : Vec b t) →
      motive t xs ys → motive (Suc t) (VCons a t x xs) (VCons b t y ys))
  (n : Nat) (xs : Vec a n) (ys : Vec b n)
  : motive n xs ys =
  match xs {
    VNil ↦ on_nil ys;
    VCons t x tail_xs ↦
      match ys {
        VCons _ y tail_ys ↦
          on_cons t x tail_xs y tail_ys
            (vec_pair_ind a b motive on_nil on_cons t tail_xs tail_ys)
      }
  }
"#;

const BINDER: &str = r#"
theorem vec_pair_ind
  (a : Type) (b : Type)
  (motive : (n : Nat) → Vec a n → Vec b n → Prop)
  (on_nil : (ys : Vec b Zero) → motive Zero (VNil a) ys)
  (on_cons : (t : Nat) → (x : a) → (xs : Vec a t) → (y : b) → (ys : Vec b t) →
      motive t xs ys → motive (Suc t) (VCons a t x xs) (VCons b t y ys))
  (n : Nat) (xs : Vec a n)
  : (ys : Vec b n) → motive n xs ys =
  match xs {
    VNil ↦ λys. on_nil ys;
    VCons t x tail_xs ↦
      λys. match ys {
        VCons _ y tail_ys ↦
          on_cons t x tail_xs y tail_ys
            (vec_pair_ind a b motive on_nil on_cons t tail_xs tail_ys)
      }
  }
"#;

fn env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl(VEC).expect("indexed Vec fixture");
    env
}

#[test]
fn variable_proposition_motive_direct_pair_induction() {
    // MEASURED: checked two-vector induction, no new trust. CLAIMED: a
    // neutral proposition goal can convoy the second vector's index.
    // THE GAP: one checked motive does not cover every proposition syntax.
    on_elaboration_stack(|| {
        let mut env = env();
        let trust = env.env.trusted_base();
        env.elaborate_decl(DIRECT)
            .expect("direct two-vector induction must check");
        assert_eq!(env.env.trusted_base(), trust, "no new trust");
    });
}

#[test]
fn variable_proposition_motive_with_result_binder() {
    // MEASURED: the same induction checks with ys under a result Π.
    // CLAIMED: the second vector's refinement survives this binder.
    // THE GAP: this pins a single named dependent Π arrangement.
    on_elaboration_stack(|| {
        let mut env = env();
        let trust = env.env.trusted_base();
        env.elaborate_decl(BINDER)
            .expect("result-binder two-vector induction must check");
        assert_eq!(env.env.trusted_base(), trust, "no new trust");
    });
}

#[test]
fn wrong_cons_step_index_is_rejected_after_reaching_the_cast() {
    // MEASURED: the same direct theorem with an on_cons result at predecessor
    // index fails at kernel type checking; the true version checks. CLAIMED:
    // widened motive recovery never accepts a wrong specialized step.
    // THE GAP: this variant shifts one index; other bad methods are untested.
    on_elaboration_stack(|| {
        let mut env = env();
        let trust = env.env.trusted_base();
        env.elaborate_decl(DIRECT).expect("matching true control");
        let wrong = DIRECT.replace(
            "motive t xs ys → motive (Suc t) (VCons a t x xs) (VCons b t y ys)",
            "motive t xs ys → motive t xs ys",
        );
        assert_ne!(wrong, DIRECT, "the false step must differ");
        let error = env
            .elaborate_decl(&wrong.replace("vec_pair_ind", "vec_pair_ind_wrong"))
            .expect_err("a predecessor-index step must not prove a successor goal");
        assert!(
            matches!(
                error,
                ElabError::KernelRejected {
                    error: KernelError::TypeMismatch { .. },
                    ..
                }
            ),
            "the bad method must reach kernel type checking: {error:?}"
        );
        assert_eq!(env.env.trusted_base(), trust, "no new trust");
    });
}
