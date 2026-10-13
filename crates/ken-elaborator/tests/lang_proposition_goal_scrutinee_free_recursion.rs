//! Scrutinee-free proposition goals keep the ordinary nested-match plan.
//! Promise class: durable invariant. Spec 34 §3.5 and 39 §2.1.

use ken_elaborator::{ElabEnv, ElabError};

// The adjacent indexed-match suites state 8 MiB against a measured
// 1,982,464-byte prelude peak; this provisions more than four times that peak.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn on_elaboration_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked");
}

const VEC: &str = "data Vec (a : Type) : Nat -> Type where { VNil : Vec a Zero; VCons : (n : Nat) -> a -> Vec a n -> Vec a (Suc n) }";
const DUAL_FIN: &str = "data DualFin : Nat -> Type where { DualFZ : (n : Nat) -> DualFin (Suc n); DualFS : (n : Nat) -> DualFin n -> DualFin (Suc n) }";
const DUAL_ENV: &str = "data DualEnv (a : Type) : Nat -> Type where { DualNil : DualEnv a Zero; DualCons : (n : Nat) -> a -> DualEnv a n -> DualEnv a (Suc n) }";
const LOOKUP: &str = "fn dual_lookup (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : a = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ x }; DualFS m j ↦ match xs { DualCons k x rest ↦ dual_lookup a m rest j } }";

fn env(dual: bool) -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    let fixtures: &[&str] = if dual {
        &[DUAL_FIN, DUAL_ENV, LOOKUP]
    } else {
        &[VEC]
    };
    for fixture in fixtures {
        env.elaborate_decl(fixture)
            .unwrap_or_else(|error| panic!("fixture {fixture}: {error:?}"));
    }
    env
}

fn checks(dual: bool, source: &str) {
    let mut env = env(dual);
    let trust = env.env.trusted_base();
    env.elaborate_decl(source)
        .unwrap_or_else(|error| panic!("must check without a convoy: {source}: {error:?}"));
    assert_eq!(env.env.trusted_base(), trust, "no new trusted identity");
}

const DUAL_TOP: &str = "theorem dual_top (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Top = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ Proved }; DualFS m j ↦ match xs { DualCons k x rest ↦ dual_top a m rest j } }";
const VEC_TOP: &str = "theorem vec_top (a : Type) (b : Type) (n : Nat) (xs : Vec a n) (ys : Vec b n) : Top = match xs { VNil ↦ Proved; VCons k x r ↦ match ys { VCons _ y r2 ↦ vec_top a b k r r2 } }";
const MOTIVE_I: &str = "theorem motive_i (a : Type) (pp : (n : Nat) → DualFin n → Prop) (hz : (m : Nat) → pp (Suc m) (DualFZ m)) (hs : (m : Nat) → (j : DualFin m) → pp m j → pp (Suc m) (DualFS m j)) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : pp n i = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ hz m }; DualFS m j ↦ match xs { DualCons k x rest ↦ hs m j (motive_i a pp hz hs m rest j) } }";
const CLOSED_EQUAL: &str = "theorem closed_equal (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Equal Nat Zero Zero = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ Proved }; DualFS m j ↦ match xs { DualCons k x rest ↦ closed_equal a m rest j } }";
const DUAL_PROP: &str = "theorem dual_prop (a : Type) (pp : Prop) (h : pp) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : pp = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ h }; DualFS m j ↦ match xs { DualCons k x rest ↦ dual_prop a pp h m rest j } }";
const VEC_PROP: &str = "theorem vec_prop (a : Type) (b : Type) (pp : Prop) (h : pp) (n : Nat) (xs : Vec a n) (ys : Vec b n) : pp = match xs { VNil ↦ h; VCons k x r ↦ match ys { VCons _ y r2 ↦ vec_prop a b pp h k r r2 } }";
const BOTTOM_TWIN: &str = "theorem bottom_twin (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Bottom = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ Proved }; DualFS m j ↦ match xs { DualCons k x rest ↦ bottom_twin a m rest j } }";
const NOT_BOTTOM: &str = "theorem not_bottom (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Not Bottom = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ λh. h }; DualFS m j ↦ match xs { DualCons k x rest ↦ λh. not_bottom a m rest j h } }";
const NONRECURSIVE: &str = "theorem nonrecursive (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Top = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ Proved }; DualFS m j ↦ match xs { DualCons k x rest ↦ Proved } }";

#[test]
fn g1_top_dual() {
    on_elaboration_stack(|| checks(true, DUAL_TOP));
}

#[test]
fn minimal_top_vec() {
    on_elaboration_stack(|| checks(false, VEC_TOP));
}

#[test]
fn g2_motive_over_index_only() {
    on_elaboration_stack(|| checks(true, MOTIVE_I));
}

#[test]
fn g3_closed_equal_reduces_to_top() {
    on_elaboration_stack(|| checks(true, CLOSED_EQUAL));
}

#[test]
fn g5_constant_proposition_dual() {
    on_elaboration_stack(|| checks(true, DUAL_PROP));
}

#[test]
fn constant_proposition_vec() {
    on_elaboration_stack(|| checks(false, VEC_PROP));
}

#[test]
fn population_recursive_index_proposition_without_scrutinee() {
    on_elaboration_stack(|| {
        let mut env = env(true);
        env.elaborate_decl(
            "fn same_len (n : Nat) (m : Nat) : Prop = match n { Zero ↦ Top; Suc k ↦ same_len k m }",
        )
        .expect("recursive proposition fixture");
        let trust = env.env.trusted_base();
        let source = "theorem index_prop (a : Type) (h : (k : Nat) → same_len k k) (step : (k : Nat) → same_len k k → same_len (Suc k) (Suc k)) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : same_len n n = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ h (Suc m) }; DualFS m j ↦ match xs { DualCons k x rest ↦ step m (index_prop a h step m rest j) } }";
        env.elaborate_decl(source)
            .expect("index-only proposition checks");
        assert_eq!(env.env.trusted_base(), trust, "no new trusted identity");
    });
}

#[test]
fn scrutinee_dependent_gains_still_check() {
    on_elaboration_stack(|| {
        let lookup_motive = "theorem lookup_motive (a : Type) (pp : a → Prop) (hp : (x : a) → pp x) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : pp (dual_lookup a n xs i) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ hp x }; DualFS m j ↦ match xs { DualCons k x rest ↦ lookup_motive a pp hp m rest j } }";
        checks(true, lookup_motive);
        let mut env = env(true);
        env.elaborate_decl(
            "fn same_len (n : Nat) (m : Nat) : Prop = match n { Zero ↦ Top; Suc k ↦ same_len k m }",
        )
        .expect("recursive proposition fixture");
        let trust = env.env.trusted_base();
        let same_len_lookup = "theorem same_len_lookup (n : Nat) (xs : DualEnv Nat n) (i : DualFin n) (h : (v : Nat) → same_len v v) (step : (v : Nat) → same_len v v → same_len (Suc v) (Suc v)) : same_len (dual_lookup Nat n xs i) (dual_lookup Nat n xs i) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ h x }; DualFS m j ↦ match xs { DualCons k x rest ↦ same_len_lookup m rest j h step } }";
        env.elaborate_decl(same_len_lookup)
            .expect("recursive proposition over lookup checks");
        assert_eq!(env.env.trusted_base(), trust, "no new trusted identity");
    });
}

#[test]
fn g4_bottom_false_twin_gets_typed_refusal() {
    on_elaboration_stack(|| {
        let mut env = env(true);
        let trust = env.env.trusted_base();
        let error = env
            .elaborate_decl(BOTTOM_TWIN)
            .expect_err("Bottom is uninhabited");
        assert!(
            matches!(
                error,
                ElabError::KernelRejected { .. } | ElabError::TypeMismatch { .. }
            ),
            "false twin must have a typed error rather than Internal: {error:?}"
        );
        assert_eq!(env.env.trusted_base(), trust, "no new trusted identity");
    });
}

#[test]
fn not_bottom_and_nonrecursive_step_controls() {
    on_elaboration_stack(|| {
        checks(true, NOT_BOTTOM);
        checks(true, NONRECURSIVE);
    });
}
