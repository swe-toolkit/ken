//! Nested constructor-field splits preserve the motive's outer binders and
//! resolve pattern aliases before an in-matrix kernel query.
//! Spec: `spec/30-surface/34-data-match.md §3.1–3.2, §4.4`.
//! Promise class: durable invariant. Every program enters via `elaborate_file`.

use ken_elaborator::ElabEnv;
use ken_kernel::{normalize, Context};

const VEC: &str = r#"
data Vec (a : Type) : Nat → Type where {
  VNil : Vec a Zero;
  VCons : (n : Nat) → a → Vec a n → Vec a (Suc n)
}
"#;

fn assert_normalized_equal(env: &ElabEnv, observed: &str, expected: &str) {
    let normal = |name: &str| {
        let id = *env.globals.get(name).expect("named checked value");
        let body = env
            .env
            .transparent_body(id)
            .expect("checked transparent value")
            .1;
        normalize(&env.env, &Context::new(), &body)
    };
    assert_eq!(normal(observed), normal(expected));
}

#[test]
fn indexed_nested_sibling_type_keeps_outer_index_binder() {
    // MEASURED: a nested split on the VCons element leaves its Vec Nat m
    // sibling well-typed and both closed calls normalize to different values.
    // CLAIMED: the motive lifts outer variables past its extra binder.
    // THE GAP: the tested source retains the sibling after the split, so
    // removing the shift reproduces the original KernelRejected TypeMismatch.
    let mut env = ElabEnv::new().expect("prelude");
    let trusted_before = env.env.trusted_base();
    env.elaborate_file(&format!(
        "{VEC}\ndata PairOut : Type where {{ Out : Nat → Nat → PairOut }}\n\
         fn f (n : Nat) (xs : Vec Nat (Suc n)) : PairOut = \
         let r = match xs {{ \
           VCons _ Zero _ ↦ Out Zero Zero; \
           VCons _ _ _ ↦ Out (Suc Zero) Zero \
         }} in r\n\
         const zero : PairOut = f Zero (VCons Nat Zero Zero (VNil Nat))\n\
         const one : PairOut = f Zero (VCons Nat Zero (Suc Zero) (VNil Nat))\n\
         const expected_zero : PairOut = Out Zero Zero\n\
         const expected_one : PairOut = Out (Suc Zero) Zero"
    ))
    .expect("nested sibling indexed by the constructor field checks");
    assert_normalized_equal(&env, "zero", "expected_zero");
    assert_normalized_equal(&env, "one", "expected_one");
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn enclosing_as_alias_survives_constant_inner_match_frame() {
    // MEASURED: an outer match alias is used from both arms of a nested
    // constant-motive match, and the closed call reduces to its actual value.
    // CLAIMED: an inner match must not consume its enclosing alias's sentinel.
    // THE GAP: the inner match has its own replacement frame and nested
    // split, so treating an unknown sentinel as an error would break it.
    let mut env = ElabEnv::new().expect("prelude");
    let trusted_before = env.env.trusted_base();
    env.elaborate_file(&format!(
        "data NatBox : Type where {{ BoxNat : Nat → NatBox }}\n\
         fn outer_const (h : NatBox) (x : Nat) : Nat = \
         let r = match x {{ \
           Zero as saved ↦ let q = match h {{ \
             BoxNat Zero ↦ saved; \
             BoxNat (Suc m) ↦ saved \
           }} in q; \
           (Suc k) as saved ↦ let q = match h {{ \
             BoxNat Zero ↦ saved; \
             BoxNat (Suc m) ↦ saved \
           }} in q \
         }} in r\n\
         const observed : Nat = outer_const (BoxNat Zero) (Suc Zero)\n\
         const expected : Nat = Suc Zero"
    ))
    .expect("inner frame must retain the enclosing as-alias");
    assert_normalized_equal(&env, "observed", "expected");
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn nonindexed_nested_list_split_domain_uses_ambient_parameter() {
    // MEASURED: with a closed Nat result, the nested List a column checks
    // and its Nil/Cons closed calls normalize distinctly. CLAIMED: the
    // non-indexed motive's domain must be weakened under the split binder.
    // THE GAP: codomain shift cannot affect this closed result; shift-only
    // must remain red and the additional domain weakening must make it green.
    let mut env = ElabEnv::new().expect("prelude");
    let trusted_before = env.env.trusted_base();
    env.elaborate_file(
        "data List (a : Type) : Type where { \
           Nil : List a; Cons : a → List a → List a \
         }\n\
         data BoxList (a : Type) : Type where { \
           MkBoxList : List a → Nat → BoxList a \
         }\n\
         fn list_split (a : Type) (b : BoxList a) : Nat = \
         let r = match b { \
           MkBoxList Nil _ ↦ Zero; \
           MkBoxList (Cons _ _) _ ↦ Suc Zero \
         } in r\n\
         const nil : Nat = list_split Nat (MkBoxList Nat (Nil Nat) Zero)\n\
         const cons : Nat = list_split Nat \
           (MkBoxList Nat (Cons Nat Zero (Nil Nat)) Zero)\n\
         const expected_nil : Nat = Zero\n\
         const expected_cons : Nat = Suc Zero",
    )
    .expect("List a's nested motive domain keeps the ambient a binder");
    assert_normalized_equal(&env, "nil", "expected_nil");
    assert_normalized_equal(&env, "cons", "expected_cons");
    assert_eq!(env.env.trusted_base(), trusted_before);
}
