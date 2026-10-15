//! Ambient hypotheses and index/embedded convoys share one motive telescope.
//! Promise class: durable invariant. Spec 34 §3.2.1, §3.5 and 39 §2.1.

use ken_elaborator::{ElabEnv, ElabError};

// A measured 1,982,464-byte prelude peak has more than fourfold headroom.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn on_elaboration_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked");
}

const DUAL_FIN: &str = "data DualFin : Nat -> Type where { DualFZ : (n : Nat) -> DualFin (Suc n); DualFS : (n : Nat) -> DualFin n -> DualFin (Suc n) }";
const DUAL_ENV: &str = "data DualEnv (a : Type) : Nat -> Type where { DualNil : DualEnv a Zero; DualCons : (n : Nat) -> a -> DualEnv a n -> DualEnv a (Suc n) }";
const FIN_TO_NAT: &str = "fn dual_fin_to_nat (n : Nat) (i : DualFin n) : Nat = match i { DualFZ m ↦ Zero; DualFS m j ↦ Suc (dual_fin_to_nat m j) }";
const LOOKUP: &str = "fn dual_lookup (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : a = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ x }; DualFS m j ↦ match xs { DualCons k x rest ↦ dual_lookup a m rest j } }";
const VEC: &str = "data Vec (a : Type) : Nat -> Type where { VNil : Vec a Zero; VCons : (n : Nat) -> a -> Vec a n -> Vec a (Suc n) }";

fn env(dual: bool) -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    let fixtures: &[&str] = if dual {
        &[DUAL_FIN, DUAL_ENV, FIN_TO_NAT, LOOKUP]
    } else {
        &[VEC]
    };
    for fixture in fixtures {
        env.elaborate_decl(fixture)
            .unwrap_or_else(|error| panic!("fixture {fixture}: {error:?}"));
    }
    env
}

fn check_true(dual: bool, source: &str) {
    let mut env = env(dual);
    let trust = env.env.trusted_base();
    env.elaborate_decl(source)
        .unwrap_or_else(|error| panic!("true proof must check: {source}: {error:?}"));
    assert_eq!(env.env.trusted_base(), trust, "no new trusted identity");
}

fn check_typed_refusal(source: &str) {
    let mut env = env(true);
    let trust = env.env.trusted_base();
    let error = env
        .elaborate_decl(source)
        .expect_err("false or deferred proof must fail");
    assert!(
        matches!(
            error,
            ElabError::TypeMismatch { .. } | ElabError::KernelRejected { .. }
        ),
        "refusal must be typed, never an Internal: {error:?}"
    );
    assert_eq!(env.env.trusted_base(), trust, "no new trusted identity");
}

fn e_like(name: &str, before: bool, after: bool, outer: bool) -> String {
    let xs_before = if before { " (xs : DualEnv a n)" } else { "" };
    let xs_after = if after { " (xs : DualEnv a n)" } else { "" };
    let proof = if outer {
        "match c { True ↦ match i { DualFZ m ↦ Refl; DualFS m j ↦ Refl }; False ↦ Refl }"
    } else {
        "match i { DualFZ m ↦ Refl; DualFS m j ↦ Refl }"
    };
    format!("theorem {name} (a : Type) (g : Nat -> a) (c : Bool) (n : Nat){xs_before} (i : DualFin n){xs_after} : Equal a (g (dual_fin_to_nat n i)) (g (dual_fin_to_nat n i)) = {proof}")
}

fn h_like(name: &str, use_h: bool, disagree: bool) -> String {
    let lookup = "dual_lookup a (Suc m) xs (DualFZ m)";
    let right = if use_h { "y" } else { lookup };
    let hyp_left = if disagree { "y" } else { lookup };
    let leaf = if use_h { "h" } else { "Refl" };
    format!("theorem {name} (a : Type) (m : Nat) (xs : DualEnv a (Suc m)) (y : a) (h : Equal a ({hyp_left}) y) : Equal a ({lookup}) ({right}) = match xs {{ DualCons k x rest ↦ {leaf} }}")
}

const G1: &str = "fn s1g (a : Type) (pe : (n : Nat) → DualEnv a n → Prop) (f : (m : Nat) → (x : a) → (e : DualEnv a m) → pe (Suc m) (DualCons a m x e) → Nat) (n : Nat) (xs : DualEnv a n) (h : pe n xs) (i : DualFin n) : Nat = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ f k x rest h }; DualFS m j ↦ Zero }";
const G2: &str = "theorem g2 (a : Type) (pe : (n : Nat) → DualEnv a n → Prop) (n : Nat) (xs : DualEnv a n) (h : pe n xs) (i : DualFin n) : pe n xs = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ h }; DualFS m j ↦ match xs { DualCons k x rest ↦ h } }";
const VEC_G2: &str = "theorem vec_g2 (a : Type) (b : Type) (pe : (n : Nat) → Vec b n → Prop) (n : Nat) (xs : Vec a n) (ys : Vec b n) (h : pe n ys) : pe n ys = match xs { VNil ↦ h; VCons k x rest ↦ match ys { VCons l y tail ↦ h } }";
const G2_FALSE: &str = "theorem g2_false (a : Type) (pe : (n : Nat) → DualEnv a n → Prop) (n : Nat) (xs : DualEnv a n) (ys : DualEnv a n) (h : pe n ys) (i : DualFin n) : pe n xs = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ h }; DualFS m j ↦ match xs { DualCons k x rest ↦ h } }";
const H1: &str = "theorem h1 (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) (y : a) (h : Equal a (dual_lookup a n xs i) y) : Equal a (dual_lookup a n xs i) y = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ h }; DualFS m j ↦ match xs { DualCons k x rest ↦ h } }";
const K2: &str = "theorem k2 (a : Type) (pp : a → Prop) (hp : (x : a) → pp x) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : pp (dual_lookup a n xs i) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ hp x }; DualFS m j ↦ match xs { DualCons k x rest ↦ hp (dual_lookup a m rest j) } }";
const H1_FALSE: &str = "theorem h1_false (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) (y : a) (other : a) (h : Equal a (dual_lookup a n xs i) y) : Equal a (dual_lookup a n xs i) other = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ h }; DualFS m j ↦ match xs { DualCons k x rest ↦ h } }";
const K2_FALSE: &str = "theorem k2_false (a : Type) (pp : a → Prop) (hp : (x : a) → pp x) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : pp (dual_lookup a n xs i) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ hp x }; DualFS m j ↦ match xs { DualCons k x rest ↦ hp x } }";
const H9_FALSE: &str = "theorem h9_false_refl (a : Type) (m : Nat) (xs : DualEnv a (Suc m)) (y : a) (h : Equal a (dual_lookup a (Suc m) xs (DualFZ m)) y) : Equal a (dual_lookup a (Suc m) xs (DualFZ m)) y = match xs { DualCons k x rest ↦ Refl }";
const H6: &str = "theorem h6 (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) (y : a) (h : Equal a (dual_lookup a n xs i) y) : Equal a (dual_lookup a n xs i) y = match i { DualFZ m ↦ h; DualFS m j ↦ h }";
const H7: &str = "theorem h7 (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) (y : a) (h : Equal a (dual_lookup a n xs i) y) : Equal a (dual_lookup a n xs i) (dual_lookup a n xs i) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ Refl }; DualFS m j ↦ match xs { DualCons k x rest ↦ Refl } }";
const H8: &str = "theorem h8 (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) (y : a) (h : Equal a (dual_lookup a n xs i) y) : Equal a (dual_lookup a n xs i) y = h";
const A1: &str = "theorem lookup_and (a : Type) (pp : a → Prop) (qq : a → Prop) (hp : (x : a) → pp x) (hq : (x : a) → qq x) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : And (pp (dual_lookup a n xs i)) (qq (dual_lookup a n xs i)) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ and_intro (pp x) (qq x) (hp x) (hq x) }; DualFS m j ↦ match xs { DualCons k x rest ↦ lookup_and a pp qq hp hq m rest j } }";
const A1F: &str = "theorem lookup_and_false (a : Type) (pp : a → Prop) (qq : a → Prop) (hp : (x : a) → pp x) (hq : (x : a) → qq x) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : And (pp (dual_lookup a n xs i)) (qq (dual_lookup a n xs i)) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ and_intro (pp x) (qq x) (hp x) (hp x) }; DualFS m j ↦ match xs { DualCons k x rest ↦ lookup_and_false a pp qq hp hq m rest j } }";

#[test]
fn ac1_e9_ambient_binder_before_index_checks() {
    on_elaboration_stack(|| check_true(true, &e_like("e9", true, false, true)));
}

#[test]
fn ac1_e11_ambient_binder_after_index_checks() {
    on_elaboration_stack(|| check_true(true, &e_like("e11", false, true, true)));
}

#[test]
fn ac1_g2_nested_hypothesis_checks() {
    on_elaboration_stack(|| check_true(true, G2));
}

#[test]
fn ac1_context_and_embedded_companions_coexist() {
    on_elaboration_stack(|| {
        check_true(true, &h_like("h9", false, false));
        check_true(true, &h_like("h5", true, false));
        check_true(true, G1);
        check_true(false, VEC_G2);
    });
}

#[test]
fn ac2_existing_context_controls_stay_accepted() {
    on_elaboration_stack(|| {
        check_true(true, &e_like("e8", false, false, true));
        check_true(true, &e_like("e10", true, false, false));
        check_true(true, H6);
        check_true(true, H7);
        check_true(true, H8);
    });
}

// MEASURED: both true recursive coordinates check in the same nested
// index/field match as the convoyed h and recursive call. CLAIMED: the
// goal and each moved field share the goal's single transport. THE GAP:
// a false endpoint could still be accepted, guarded by the paired test.
#[test]
fn ac1_ambient_hypothesis_and_recursive_coordinate_check() {
    on_elaboration_stack(|| {
        check_true(true, H1);
        check_true(true, K2);
    });
}

#[test]
fn ac3_disagreeing_hypotheses_and_false_recursive_coordinates_refuse_typed() {
    on_elaboration_stack(|| {
        check_typed_refusal(&h_like("h9_false", true, true));
        check_typed_refusal(G2_FALSE);
        check_typed_refusal(H1_FALSE);
        check_typed_refusal(K2_FALSE);
        check_typed_refusal(H9_FALSE);
    });
}

#[test]
fn ac3b_and_methods_under_sigma_keep_their_scope() {
    on_elaboration_stack(|| {
        check_true(true, A1);
        check_typed_refusal(A1F);
    });
}
