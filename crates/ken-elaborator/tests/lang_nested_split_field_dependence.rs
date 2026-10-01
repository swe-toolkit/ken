//! Nested constructor-field splits preserve the motive's outer binders and
//! resolve pattern aliases before an in-matrix kernel query.
//! Spec: `spec/30-surface/34-data-match.md §3.1–3.2, §4.4`.
//! Promise class: durable invariant. Every program enters via `elaborate_file`.

use ken_elaborator::ElabEnv;
use ken_kernel::{inductive::method_type, normalize, Context, Level, Term};

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
fn diagnostic_m_align_two_recursive_field_method_telescope() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(
        "data Branch : Type where { \
           Leaf : Branch; Node : Branch → Nat → Branch → Branch \
         }",
    )
    .expect("binary constructor");
    let branch_id = env.globals["Branch"];
    let ind = env.env.inductive(branch_id).expect("checked inductive");
    let motive = Term::lam(Term::indformer(branch_id, vec![]), Term::ty(Level::Zero));
    let method = method_type(&env.env, ind, 1, &motive, &[], &[]).expect("method type");
    let mut cursor = &method;
    let mut domains = Vec::new();
    while let Term::Pi(domain, rest) = cursor {
        domains.push(format!("{domain:?}"));
        cursor = rest;
    }
    eprintln!("M-align domains: {domains:?}");
}

#[test]
fn second_nested_split_inside_zero_bucket_keeps_root_ih_tail() {
    let mut env = ElabEnv::new().expect("prelude");
    let trusted_before = env.env.trusted_base();
    env.elaborate_file(&format!(
        "{VEC}\ndata PairOut : Type where {{ Out : Nat → Nat → PairOut }}\n\
         fn deeper (n : Nat) (xs : Vec Nat (Suc n)) : PairOut = \
         match xs {{ \
           VCons m Zero VNil ↦ Out Zero Zero; \
           VCons m Zero (VCons k _ _) ↦ Out (Suc Zero) Zero; \
           VCons m (Suc a) _ ↦ Out (Suc (Suc Zero)) Zero \
         }}\n\
         const nil : PairOut = deeper Zero (VCons Nat Zero Zero (VNil Nat))\n\
         const cons : PairOut = deeper (Suc Zero) \
           (VCons Nat (Suc Zero) Zero (VCons Nat Zero Zero (VNil Nat)))\n\
         const expected_nil : PairOut = Out Zero Zero\n\
         const expected_cons : PairOut = Out (Suc Zero) Zero"
    ))
    .expect("second nested split in Zero bucket checks with root IH still owed");
    assert_normalized_equal(&env, "nil", "expected_nil");
    assert_normalized_equal(&env, "cons", "expected_cons");
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn indexed_exact_adversary_repro_checks() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\ndata PairOut : Type where {{ Out : Nat → Nat → PairOut }}\n\
         fn f (n : Nat) (xs : Vec Nat (Suc n)) : PairOut = \
         match xs {{ \
           VCons _ Zero _ ↦ Out Zero Zero; \
           VCons _ _ _ ↦ Out Zero Zero \
         }}"
    ))
    .expect("exact original F1 finding checks after shift and domain weaken");
}

#[test]
fn outer_as_alias_with_flat_inner_match_keeps_whole_constructor() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(
        "data NatBox : Type where { BoxNat : Nat → NatBox }\n\
         fn outer_flat (h : NatBox) (x : Nat) : Nat = \
         let r = match x { \
           Zero as saved ↦ match h { BoxNat z ↦ saved }; \
           (Suc k) as saved ↦ match h { BoxNat z ↦ saved } \
         } in r\n\
         const observed : Nat = outer_flat (BoxNat Zero) (Suc Zero)\n\
         const expected : Nat = Suc Zero",
    )
    .expect("flat inner match preserves the outer alias");
    assert_normalized_equal(&env, "observed", "expected");
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
fn diagnostic_outer_alias_inside_reverting_nested_split() {
    let mut env = ElabEnv::new().expect("prelude");
    let result = env.elaborate_file(&format!(
        "{VEC}\ndata Tag (n : Nat) : Vec Nat n → Type where {{ \
           MkTag : (v : Vec Nat n) → Tag n v \
         }}\n\
         data HolderD : Type where {{ \
           HoldD : (n : Nat) → (v : Vec Nat n) → Tag n v → HolderD \
         }}\n\
         fn outer_revert (h : HolderD) (x : Nat) : Nat = \
         let r = match x {{ \
           Zero as saved ↦ let q = match h {{ \
             HoldD n VNil _ ↦ saved; \
             HoldD n (VCons m _ _) _ ↦ saved \
           }} in q; \
           (Suc k) as saved ↦ let q = match h {{ \
             HoldD n VNil _ ↦ saved; \
             HoldD n (VCons m _ _) _ ↦ saved \
           }} in q \
         }} in r\n\
         const observed : Nat = outer_revert \
           (HoldD Zero (VNil Nat) (MkTag Zero (VNil Nat))) \
           (Suc Zero)\n\
         const expected : Nat = Suc Zero"
    ));
    eprintln!("M3 elaborate: {result:?}");
    if result.is_ok() {
        let normal = |name: &str| {
            let id = env.globals[name];
            let body = env.env.transparent_body(id).expect("checked").1;
            normalize(&env.env, &Context::new(), &body)
        };
        eprintln!("M3 normalized observed={:?}, expected={:?}", normal("observed"), normal("expected"));
    }
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
        "data BoxList (a : Type) : Type where { \
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
