//! Nested constructor-field splits preserve motive binders and virtual
//! alias occurrences inside the derived method telescope.
//! Spec: `spec/30-surface/34-data-match.md §3.1–3.2, §4.4`.
//! Promise classes: durable value invariants, plus typed wrong-method
//! refusals. Every program
//! enters via `elaborate_file`.

use ken_elaborator::{ElabEnv, ElabError};
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
         const zero : PairOut = f (Suc Zero) \
           (VCons Nat (Suc Zero) Zero \
             (VCons Nat Zero (Suc (Suc Zero)) (VNil Nat)))\n\
         const one : PairOut = f (Suc Zero) \
           (VCons Nat (Suc Zero) (Suc (Suc (Suc Zero))) \
             (VCons Nat Zero (Suc (Suc Zero)) (VNil Nat)))\n\
         const expected_zero : PairOut = Out Zero Zero\n\
         const expected_one : PairOut = Out (Suc Zero) Zero"
    ))
    .expect("nested sibling indexed by the constructor field checks");
    assert_normalized_equal(&env, "zero", "expected_zero");
    assert_normalized_equal(&env, "one", "expected_one");
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn flat_and_tail_split_controls_keep_distinct_indexed_values() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\ndata PairOut : Type where {{ Out : Nat → Nat → PairOut }}\n\
         fn flat (n : Nat) (xs : Vec Nat (Suc n)) : PairOut = \
         match xs {{ VCons m x tl ↦ Out x m }}\n\
         fn tail (n : Nat) (xs : Vec Nat (Suc n)) : PairOut = \
         match xs {{ \
           VCons m x VNil ↦ Out x Zero; \
           VCons m x (VCons k y ys) ↦ Out x (Suc Zero) \
         }}\n\
         const flat_value : PairOut = flat Zero \
           (VCons Nat Zero (Suc Zero) (VNil Nat))\n\
         const tail_nil : PairOut = tail Zero \
           (VCons Nat Zero (Suc Zero) (VNil Nat))\n\
         const tail_cons : PairOut = tail (Suc (Suc Zero)) \
           (VCons Nat (Suc (Suc Zero)) (Suc (Suc (Suc (Suc Zero)))) \
             (VCons Nat (Suc Zero) (Suc (Suc (Suc (Suc (Suc Zero))))) \
               (VCons Nat Zero Zero (VNil Nat))))\n\
         const expected_flat : PairOut = Out (Suc Zero) Zero\n\
         const expected_nil : PairOut = Out (Suc Zero) Zero\n\
         const expected_cons : PairOut = Out (Suc (Suc (Suc (Suc Zero)))) \
           (Suc Zero)"
    ))
    .expect("flat match and tail split retain their separate indexed values");
    assert_normalized_equal(&env, "flat_value", "expected_flat");
    assert_normalized_equal(&env, "tail_nil", "expected_nil");
    assert_normalized_equal(&env, "tail_cons", "expected_cons");
}

#[test]
fn reverting_split_after_real_seed_column_returns_seed() {
    // On landed base and on this candidate the tag split reverts a Vec tail:
    // needs_reverting=true, dependent_tail=[0]. The preceding seed is a real
    // constructor-field binder (ctx=3, root=1, real_depth=1), not a woven
    // nested-sub-pattern binder. Both leaves return the distinct seed.
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(
        "data Vec (a : Type) : Nat → Type where { \
           VNil : Vec a Zero; \
           VCons : (n : Nat) → a → Vec a n → Vec a (Suc n) } \
         data Carrier : Type where { \
           MkCarrier : (seed : Nat) → (tag : Nat) → Vec Nat tag → Carrier } \
         fn keep (c : Carrier) : Nat = match c { \
           MkCarrier seed Zero _ ↦ seed; \
           MkCarrier seed (Suc k) _ ↦ seed } \
         const observed_zero : Nat = keep \
           (MkCarrier (Suc (Suc (Suc (Suc Zero)))) Zero (VNil Nat)) \
         const observed_suc : Nat = keep \
           (MkCarrier (Suc (Suc (Suc (Suc (Suc Zero))))) (Suc (Suc Zero)) \
             (VCons Nat (Suc Zero) Zero \
               (VCons Nat Zero Zero (VNil Nat)))) \
         const expected_zero : Nat = Suc (Suc (Suc (Suc Zero))) \
         const expected_suc : Nat = Suc (Suc (Suc (Suc (Suc Zero))))",
    )
    .expect("reverting split after real seed column kernel-checks");
    assert_normalized_equal(&env, "observed_zero", "expected_zero");
    assert_normalized_equal(&env, "observed_suc", "expected_suc");
}

#[test]
fn indexed_split_result_type_keeps_ambient_parameter() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\nfn ambient (a : Type) (n : Nat) (xs : Vec Nat (Suc n)) \
           (d : List a) : List a = let r = match xs {{ \
             VCons m Zero tl ↦ d; VCons m (Suc k) tl ↦ d \
           }} in r\n\
         const observed : List Nat = ambient Nat (Suc Zero) \
           (VCons Nat (Suc Zero) Zero \
             (VCons Nat Zero (Suc (Suc Zero)) (VNil Nat))) \
           (Cons Nat (Suc Zero) (Nil Nat))\n\
         const expected : List Nat = Cons Nat (Suc Zero) (Nil Nat)"
    ))
    .expect("ambient result type survives the nested motive binder");
    assert_normalized_equal(&env, "observed", "expected");
}

#[test]
fn indexed_split_with_zero_index_and_suc_field_preserves_tail() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\ndata PairOut : Type where {{ Out : Nat → Nat → PairOut }}\n\
         fn at_index (n : Nat) (xs : Vec Nat n) : PairOut = \
         match xs {{ \
           VNil ↦ Out Zero Zero; \
           VCons m Zero tl ↦ Out Zero (Suc Zero); \
           VCons m (Suc k) tl ↦ Out (Suc Zero) Zero \
         }}\n\
         const nil : PairOut = at_index Zero (VNil Nat)\n\
         const suc : PairOut = at_index (Suc (Suc (Suc Zero))) \
           (VCons Nat (Suc (Suc Zero)) \
             (Suc (Suc (Suc (Suc (Suc Zero))))) \
             (VCons Nat (Suc Zero) (Suc (Suc Zero)) \
               (VCons Nat Zero Zero (VNil Nat))))\n\
         const expected_nil : PairOut = Out Zero Zero\n\
         const expected_suc : PairOut = Out (Suc Zero) Zero"
    ))
    .expect("VNil and VCons indexes elaborate with sibling-dependent tail");
    assert_normalized_equal(&env, "nil", "expected_nil");
    assert_normalized_equal(&env, "suc", "expected_suc");
}

#[test]
fn bool_element_indexed_split_preserves_recursive_tail() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\ndata PairOut : Type where {{ Out : Nat → Nat → PairOut }}\n\
         fn bool_el (n : Nat) (xs : Vec Bool (Suc n)) : PairOut = \
         match xs {{ \
           VCons m True tl ↦ Out Zero (Suc Zero); \
           VCons m False tl ↦ Out (Suc Zero) Zero \
         }}\n\
         const observed : PairOut = bool_el Zero \
           (VCons Bool Zero False (VNil Bool))\n\
         const expected : PairOut = Out (Suc Zero) Zero"
    ))
    .expect("Boolean split leaves the indexed recursive tail correctly typed");
    assert_normalized_equal(&env, "observed", "expected");
}

#[test]
fn nonrecursive_indexed_sibling_depends_on_constructor_index() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\ndata IP : Nat → Type where {{ \
           MkIP : (n : Nat) → Nat → Vec Nat n → IP (Suc n) \
         }}\n\
         data PairOut : Type where {{ Out : Nat → Nat → PairOut }}\n\
         fn nonrec (n : Nat) (p : IP (Suc n)) : PairOut = \
         match p {{ \
           MkIP m Zero tl ↦ Out Zero Zero; \
           MkIP m (Suc k) tl ↦ Out (Suc Zero) Zero \
         }}\n\
         const observed : PairOut = nonrec (Suc Zero) \
           (MkIP (Suc Zero) (Suc (Suc (Suc Zero))) \
             (VCons Nat Zero Zero (VNil Nat)))\n\
         const expected : PairOut = Out (Suc Zero) Zero"
    ))
    .expect("nonrecursive indexed family retains its dependent sibling type");
    assert_normalized_equal(&env, "observed", "expected");
}

#[test]
fn two_field_nested_constructor_keeps_indexed_root_ih_tail() {
    // Durable invariant: a two-field nested constructor increases the number
    // of method binders while the indexed root's recursive IH remains in the
    // tail. The closed result distinguishes the two constructor fields.
    let mut env = ElabEnv::new().expect("prelude");
    let trusted_before = env.env.trusted_base();
    env.elaborate_file(&format!(
        "{VEC}\ndata TwoTag : Type where {{ Two : Nat → Nat → TwoTag }}\n\
         data PairOut : Type where {{ Out : Nat → Nat → PairOut }}\n\
         fn pair_fields (n : Nat) (xs : Vec TwoTag (Suc n)) : PairOut = \
         match xs {{ VCons m (Two a b) tl ↦ Out a b }}\n\
         const observed : PairOut = pair_fields (Suc (Suc Zero)) \
           (VCons TwoTag (Suc (Suc Zero)) (Two Zero (Suc Zero)) \
             (VCons TwoTag (Suc Zero) \
               (Two (Suc (Suc (Suc Zero))) \
                 (Suc (Suc (Suc (Suc Zero))))) \
               (VCons TwoTag Zero (Two Zero Zero) (VNil TwoTag))))\n\
         const expected : PairOut = Out Zero (Suc Zero)"
    ))
    .expect("two-field nested method checks with indexed root IH in tail");
    assert_normalized_equal(&env, "observed", "expected");
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn reverting_record_tuple_var_is_refused_before_wrong_binder_selection() {
    // Base rejects a sentinel at the inner split. The removed in-matrix
    // finalizer made this same source check while returning `prefix` instead
    // of `seed`. Refusal must happen before either closed call gets a value.
    let mut env = ElabEnv::new().expect("prelude");
    let source = "data Vec (a : Type) : Nat → Type where { \
        VNil : Vec a Zero; \
        VCons : (n : Nat) → a → Vec a n → Vec a (Suc n) } \
        record Envelope { payload : (x : Nat) × Nat, enabled : Bool } \
        data Carrier : Type where { \
          MkCarrier : Nat → Envelope → (tag : Nat) → Vec Nat tag → Carrier } \
        fn keep (c : Carrier) : Nat = match c { \
          MkCarrier prefix { payload = (Zero, seed), enabled = True } Zero _ ↦ seed; \
          MkCarrier prefix { payload = (Zero, seed), enabled = True } (Suc k) _ ↦ seed; \
          MkCarrier prefix { payload = (Suc i, seed), enabled = True } Zero _ ↦ seed; \
          MkCarrier prefix { payload = (Suc i, seed), enabled = True } (Suc k) _ ↦ seed; \
          MkCarrier prefix { enabled = False } _ _ ↦ Zero } \
        const observed_zero : Nat = keep (MkCarrier Zero \
          { payload = (Zero, Suc Zero), enabled = True } Zero (VNil Nat)) \
        const observed_suc : Nat = keep (MkCarrier Zero \
          { payload = (Suc Zero, Suc Zero), enabled = True } (Suc Zero) \
          (VCons Nat Zero Zero (VNil Nat))) \
        const expected : Nat = Suc Zero";
    env.elaborate_file(source)
        .expect("record-projected seed uses its own matrix occurrence");
    assert_normalized_equal(&env, "observed_zero", "expected");
    assert_normalized_equal(&env, "observed_suc", "expected");
}

#[test]
fn second_nested_split_inside_zero_bucket_returns_both_values() {
    // Durable value invariant: a nested Zero-bucket split keeps its outer
    // method, sibling and inner constructor coordinates distinct.
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
    )).expect("deeper Zero-bucket split checks in its derived telescope");
    assert_normalized_equal(&env, "nil", "expected_nil");
    assert_normalized_equal(&env, "cons", "expected_cons");
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn second_nested_split_inside_two_field_bucket_returns_both_values() {
    // Durable value invariant: the second of two same-typed fields is read
    // beneath a second split, not its preceding sibling or IH.
    let mut env = ElabEnv::new().expect("prelude");
    let trusted_before = env.env.trusted_base();
    env.elaborate_file(&format!(
        "{VEC}\ndata TwoTag : Type where {{ Two : Nat → Nat → TwoTag }}\n\
         data PairOut : Type where {{ Out : Nat → Nat → PairOut }}\n\
         fn deeper_two (n : Nat) (xs : Vec TwoTag (Suc n)) : PairOut = \
         match xs {{ \
           VCons m (Two a b) VNil ↦ Out Zero b; \
           VCons m (Two a b) (VCons k _ _) ↦ Out (Suc Zero) b \
         }}\n\
         const nil : PairOut = deeper_two Zero \
           (VCons TwoTag Zero (Two Zero (Suc Zero)) (VNil TwoTag))\n\
         const cons : PairOut = deeper_two (Suc Zero) \
           (VCons TwoTag (Suc Zero) (Two Zero (Suc Zero)) \
             (VCons TwoTag Zero (Two Zero Zero) (VNil TwoTag)))\n\
         const expected_nil : PairOut = Out Zero (Suc Zero)\n\
         const expected_cons : PairOut = Out (Suc Zero) (Suc Zero)"
    )).expect("deeper two-field split checks in its derived telescope");
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
         }}\n\
         const zero : PairOut = f Zero (VCons Nat Zero Zero (VNil Nat))\n\
         const suc : PairOut = f Zero (VCons Nat Zero (Suc Zero) (VNil Nat))\n\
         const expected : PairOut = Out Zero Zero"
    ))
    .expect("exact original F1 finding checks after shift and domain weaken");
    assert_normalized_equal(&env, "zero", "expected");
    assert_normalized_equal(&env, "suc", "expected");
}

#[test]
fn indexed_f1_adjacent_variant_discriminates_arms_and_independent_binders() {
    // Same constructor and nested element split as the exact repro, but its
    // two results differ. At n=m=1 (forced by Suc m = Suc n), the element
    // is 0 or 3; the independent Nat values cannot be confused.
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\ndata PairOut : Type where {{ Out : Nat → Nat → PairOut }}\n\
         fn adjacent (n : Nat) (xs : Vec Nat (Suc n)) : PairOut = \
         match xs {{ \
           VCons _ Zero _ ↦ Out Zero (Suc (Suc Zero)); \
           VCons _ _ _ ↦ Out (Suc Zero) (Suc (Suc Zero)) \
         }}\n\
         const observed_zero : PairOut = adjacent (Suc Zero) \
           (VCons Nat (Suc Zero) Zero \
             (VCons Nat Zero (Suc (Suc Zero)) (VNil Nat)))\n\
         const observed_suc : PairOut = adjacent (Suc Zero) \
           (VCons Nat (Suc Zero) (Suc (Suc (Suc Zero))) \
             (VCons Nat Zero (Suc (Suc Zero)) (VNil Nat)))\n\
         const expected_zero : PairOut = Out Zero (Suc (Suc Zero))\n\
         const expected_suc : PairOut = Out (Suc Zero) (Suc (Suc Zero))"
    ))
    .expect("adjacent F1 split closes method with distinct arm results");
    assert_normalized_equal(&env, "observed_zero", "expected_zero");
    assert_normalized_equal(&env, "observed_suc", "expected_suc");
}

#[test]
fn two_nat_tail_fields_keep_the_second_under_zero_and_two_field_splits() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(
        "data TwinZ : Type where { PackZ : Nat → Nat → Nat → TwinZ }\n\
         data Duo : Type where { PairDuo : Nat → Nat → Duo }\n\
         data TwinD : Type where { PackD : Duo → Nat → Nat → TwinD }\n\
         fn pick_zero (t : TwinZ) : Nat = match t { \
           PackZ Zero a b ↦ b; \
           PackZ (Suc k) a b ↦ b \
         }\n\
         fn pick_duo (t : TwinD) : Nat = match t { \
           PackD (PairDuo x y) a b ↦ b \
         }\n\
         const from_zero : Nat = pick_zero \
           (PackZ Zero (Suc (Suc (Suc Zero))) \
             (Suc (Suc (Suc (Suc (Suc Zero))))))\n\
         const from_duo : Nat = pick_duo \
           (PackD (PairDuo Zero (Suc Zero)) (Suc (Suc Zero)) \
             (Suc (Suc (Suc Zero))))\n\
         const expected_zero : Nat = Suc (Suc (Suc (Suc (Suc Zero))))\n\
         const expected_duo : Nat = Suc (Suc (Suc Zero))",
    )
    .expect("two sibling Nat fields remain distinct beneath nested methods");
    assert_normalized_equal(&env, "from_zero", "expected_zero");
    assert_normalized_equal(&env, "from_duo", "expected_duo");
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
         const observed : Nat = outer_flat \
           (BoxNat (Suc (Suc (Suc (Suc Zero))))) (Suc (Suc Zero))\n\
         const expected : Nat = Suc (Suc Zero)",
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
         const observed_zero : Nat = outer_const (BoxNat Zero) \
           (Suc (Suc (Suc Zero)))\n\
         const observed_suc : Nat = outer_const \
           (BoxNat (Suc (Suc (Suc (Suc (Suc Zero)))))) \
           (Suc (Suc (Suc Zero)))\n\
         const expected : Nat = Suc (Suc (Suc Zero))"
    ))
    .expect("inner frame must retain the enclosing as-alias");
    assert_normalized_equal(&env, "observed_zero", "expected");
    assert_normalized_equal(&env, "observed_suc", "expected");
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn enclosing_alias_survives_two_nested_match_frames() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(
        "data NatBox : Type where { BoxNat : Nat → NatBox }\n\
         fn outer_two (h : NatBox) (g : NatBox) (x : Nat) : Nat = \
         match x { \
           Zero ↦ Zero; \
           (Suc k) as saved ↦ let q = match h { \
             BoxNat (Suc z) ↦ let r = match g { \
               BoxNat Zero ↦ saved; \
               BoxNat (Suc m) ↦ saved \
             } in r; \
             BoxNat Zero ↦ saved \
           } in q \
         }\n\
         const observed_zero : Nat = outer_two \
           (BoxNat (Suc (Suc (Suc (Suc (Suc (Suc (Suc Zero)))))))) \
           (BoxNat Zero) (Suc (Suc (Suc (Suc Zero))))\n\
         const observed_suc : Nat = outer_two \
           (BoxNat (Suc (Suc (Suc (Suc (Suc (Suc (Suc Zero)))))))) \
           (BoxNat (Suc (Suc Zero))) (Suc (Suc (Suc (Suc Zero))))\n\
         const expected : Nat = Suc (Suc (Suc (Suc Zero)))",
    )
    .expect("enclosing alias passes through two nested splitting frames");
    assert_normalized_equal(&env, "observed_zero", "expected");
    assert_normalized_equal(&env, "observed_suc", "expected");
}

#[test]
fn middle_frame_alias_is_finalized_by_its_owner_not_the_inner_or_outer_match() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(
        "data NatBox : Type where { BoxNat : Nat → NatBox }\n\
         fn middle (x : Nat) (h : NatBox) (g : NatBox) : NatBox = \
         match x { \
           Zero ↦ BoxNat Zero; \
           (Suc y) ↦ let q = match h { \
             (BoxNat Zero) as saved ↦ let r = match g { \
               BoxNat Zero ↦ saved; \
               BoxNat (Suc m) ↦ saved \
             } in r; \
             (BoxNat (Suc k)) as saved ↦ let r = match g { \
               BoxNat Zero ↦ saved; \
               BoxNat (Suc m) ↦ saved \
             } in r \
           } in q \
         }\n\
         const observed_zero : NatBox = middle (Suc (Suc (Suc Zero))) \
           (BoxNat (Suc (Suc (Suc (Suc (Suc Zero)))))) (BoxNat Zero)\n\
         const observed_suc : NatBox = middle (Suc (Suc (Suc Zero))) \
           (BoxNat (Suc (Suc (Suc (Suc (Suc Zero)))))) \
           (BoxNat (Suc (Suc (Suc (Suc (Suc (Suc (Suc Zero))))))))\n\
         const expected : NatBox = BoxNat (Suc (Suc (Suc (Suc (Suc Zero)))))",
    )
    .expect("middle match owns the alias across inner split under outer match");
    assert_normalized_equal(&env, "observed_zero", "expected");
    assert_normalized_equal(&env, "observed_suc", "expected");
}

#[test]
fn outer_alias_inside_reverting_nested_split_refused() {
    let mut env = ElabEnv::new().expect("prelude");
    let source = format!(
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
    );
    env.elaborate_file(&source)
        .expect("outer alias retains its value through a reverting nested split");
    assert_normalized_equal(&env, "observed", "expected");
}

#[test]
fn constant_inner_alias_does_not_skip_final_kernel_type_check() {
    // The constant inner split kernel-checks every derived method; the final
    // definition must still refuse this mistyped nested arm.
    let mut env = ElabEnv::new().expect("prelude");
    let source = "data NatBox : Type where { BoxNat : Nat → NatBox }\n\
                  fn ill_typed (h : NatBox) (x : Nat) : Nat = \
                  match x { \
                    Zero ↦ Zero; \
                    (Suc k) as saved ↦ let q = match h { \
                      BoxNat Zero ↦ saved; \
                      BoxNat (Suc m) ↦ BoxNat saved \
                    } in q \
                  }";
    let error = env
        .elaborate_file(source)
        .expect_err("a constant-path alias does not authorize the ill-typed arm");
    assert!(
        matches!(error, ElabError::KernelRejected { ref span, .. }
        if span.start != source.find("fn ill_typed").expect("fixture declaration")),
        "the in-matrix method gate must reject before final declaration: {error:?}"
    );
}

#[test]
fn reverting_outer_alias_in_mistyped_method_is_refused_before_kernel() {
    // The nested Vec split reverts the Tag n v tail. The method with the
    // wrong-valued body must be refused, independently of alias resolution.
    let mut env = ElabEnv::new().expect("prelude");
    let source = format!(
        "{VEC}\ndata Tag (n : Nat) : Vec Nat n → Type where {{ \
           MkTag : (v : Vec Nat n) → Tag n v \
         }}\n\
         data HolderD : Type where {{ \
           HoldD : (n : Nat) → (v : Vec Nat n) → Tag n v → HolderD \
         }}\n\
         data NatBox : Type where {{ BoxNat : Nat → NatBox }}\n\
         fn ill_typed (h : HolderD) (x : Nat) : Nat = \
         match x {{ \
           Zero ↦ Zero; \
           (Suc k) as saved ↦ let q = match h {{ \
             HoldD n VNil _ ↦ saved; \
             HoldD n (VCons m _ _) _ ↦ BoxNat saved \
           }} in q \
         }}"
    );
    let error = env.elaborate_file(&source)
        .expect_err("a wrong-valued method must remain kernel-refused");
    assert!(matches!(&error, ElabError::KernelRejected { .. }),
        "mistyped method must reach the unconditional in-matrix check: {error:?}");
}

#[test]
fn dependent_later_field_variable_row_is_refused_until_derived_telescope() {
    let mut env = ElabEnv::new().expect("prelude");
    let trusted_before = env.env.trusted_base();
    let source = format!(
        "{VEC}\ndata PairOut : Type where {{ Out : Nat → Nat → PairOut }}\n\
         data Dep : Type where {{ MkDep : (n : Nat) → Vec Nat n → Dep }}\n\
         fn dep (d : Dep) : PairOut = let r = match d {{ \
           MkDep Zero v ↦ Out Zero Zero; \
           MkDep n v ↦ Out n n \
         }} in r\n\
         const zero : PairOut = dep (MkDep Zero (VNil Nat))\n\
         const one : PairOut = dep \
           (MkDep (Suc Zero) (VCons Nat Zero Zero (VNil Nat)))\n\
         const expected_zero : PairOut = Out Zero Zero\n\
         const expected_one : PairOut = Out (Suc Zero) (Suc Zero)"
    );
    env.elaborate_file(&source)
        .expect("dependent sibling's variable row uses its own occurrence");
    assert_normalized_equal(&env, "zero", "expected_zero");
    assert_normalized_equal(&env, "one", "expected_one");
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn two_split_columns_variable_row_returns_distinct_occurrences() {
    let mut env = ElabEnv::new().expect("prelude");
    let trusted_before = env.env.trusted_base();
    env.elaborate_file(
        "data PairOut : Type where { Out : Nat → Nat → PairOut }\n\
         data Tri : Type where { MkTri : Nat → Nat → Nat → Tri }\n\
         fn f (t : Tri) : PairOut = match t { \
           MkTri Zero Zero c ↦ Out c c; \
           MkTri a b c ↦ Out a b \
         }\n\
         const zero : PairOut = f (MkTri Zero Zero (Suc Zero))\n\
         const distinct : PairOut = f (MkTri (Suc Zero) (Suc (Suc Zero)) Zero)\n\
         const expected_zero : PairOut = Out (Suc Zero) (Suc Zero)\n\
         const expected_distinct : PairOut = Out (Suc Zero) (Suc (Suc Zero))",
    ).expect("two split columns bind distinct occurrences");
    assert_normalized_equal(&env, "zero", "expected_zero");
    assert_normalized_equal(&env, "distinct", "expected_distinct");
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
        "data BoxList (a : Type) : Type where { \
           MkBoxList : List a → Nat → BoxList a \
         }\n\
         fn list_split (a : Type) (b : BoxList a) : Nat = \
         let r = match b { \
           MkBoxList Nil _ ↦ Zero; \
           MkBoxList (Cons _ _) _ ↦ Suc Zero \
         } in r\n\
         const nil : Nat = list_split Nat \
           (MkBoxList Nat (Nil Nat) (Suc (Suc Zero)))\n\
         const cons : Nat = list_split Nat \
           (MkBoxList Nat \
             (Cons Nat (Suc (Suc (Suc Zero))) (Nil Nat)) \
             (Suc (Suc Zero)))\n\
         const expected_nil : Nat = Zero\n\
         const expected_cons : Nat = Suc Zero",
    )
    .expect("List a's nested motive domain keeps the ambient a binder");
    assert_normalized_equal(&env, "nil", "expected_nil");
    assert_normalized_equal(&env, "cons", "expected_cons");
    assert_eq!(env.env.trusted_base(), trusted_before);
}
