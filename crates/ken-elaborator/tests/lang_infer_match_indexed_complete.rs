//! Inferred matches on indexed families share the dependent method path,
//! whether or not a root constructor was omitted.
//! Spec: `spec/30-surface/34-data-match.md §3.1–3.2, §4.1–4.3`.
//! Promise class: durable invariant. All fixtures exercise the checked
//! source-to-eliminator path through `ElabEnv::elaborate_file`.

use ken_elaborator::{ElabEnv, ElabError};

const VEC: &str = r#"
data Vec (a : Type) : Nat → Type where {
  VNil : Vec a Zero;
  VCons : (n : Nat) → a → Vec a n → Vec a (Suc n)
}
"#;

const FIN: &str = r#"
data Fin : Nat → Type where {
  FZ : (n : Nat) → Fin (Suc n);
  FS : (n : Nat) → Fin n → Fin (Suc n)
}
"#;

#[test]
fn complete_vec_at_variable_index_checks() {
    // MEASURED: both root methods are supplied and the indexed eliminator
    // kernel-checks. CLAIMED: completeness must not change its motive path.
    // THE GAP: a surface acceptance alone need not discriminate constant
    // motives; reverting the indexed dispatch makes this exact row red.
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\nfn complete (a : Type) (n : Nat) (xs : Vec a n) : Nat = \
         let r = match xs {{ VNil ↦ Zero; VCons m _ _ ↦ m }} in r"
    ))
    .expect("complete indexed Vec at n must check");
}

#[test]
fn complete_vec_at_successor_index_checks() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\nfn complete_suc (a : Type) (n : Nat) (xs : Vec a (Suc n)) : Nat = \
         let r = match xs {{ VNil ↦ Zero; VCons m _ _ ↦ m }} in r"
    ))
    .expect("adding a VNil arm to a well-typed omitted-index match must check");
}

#[test]
fn complete_fin_with_both_arms_checks() {
    // Fin has no family parameters: the old constant-motive path erroneously
    // passed its one index as a parameter to the kernel eliminator.
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{FIN}\nfn complete_fin (n : Nat) (i : Fin (Suc n)) : Nat = \
         let r = match i {{ FZ m ↦ Zero; FS m _ ↦ Suc m }} in r"
    ))
    .expect("complete indexed Fin with no parameters must check");
}

#[test]
fn nested_root_split_closes_the_remaining_method_telescope() {
    // A nested tail split can yield an eliminator rather than a lambda for
    // the outer constructor's pending IH. The method is applied to that IH.
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\nfn nested (a : Type) (n : Nat) (xs : Vec a (Suc n)) : Nat = \
         let r = match xs {{ VCons m x VNil ↦ Zero; VCons m x _ ↦ m }} in r"
    ))
    .expect("nested split under omitted root VNil must close its method");
}

#[test]
fn nested_vnil_and_vcons_specialize_the_carried_root_ih() {
    // Both nested constructor methods carry the outer VCons IH. Its type
    // specializes to the nested constructor through the nested motive.
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\nfn nested_both (a : Type) (n : Nat) (xs : Vec a (Suc n)) : Nat = \
         let r = match xs {{ \
           VCons m _ VNil ↦ Zero; \
           VCons m _ (VCons k _ _) ↦ Suc k \
         }} in r"
    ))
    .expect("both specialized nested methods kernel-check");
}

#[test]
fn nested_split_threads_two_root_ih_columns() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(
        "data PairIx : Nat → Type where { \
           PZero : PairIx Zero; \
           PStep : (m : Nat) → PairIx m → PairIx m → PairIx (Suc m) \
         }\nfn nested_pair (n : Nat) (xs : PairIx (Suc n)) : Nat = \
         let r = match xs { \
           PStep m PZero _ ↦ Zero; PStep m _ _ ↦ m \
         } in r",
    )
    .expect("both dependent root IHs survive the nested split");
}

#[test]
fn constant_nested_motive_keeps_the_nonindexed_control() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(
        "data NatBox : Type where { BoxNat : Nat → NatBox }\n\
         fn boxed (b : NatBox) : Nat = let r = match b { \
           BoxNat Zero ↦ Zero; BoxNat (Suc n) ↦ n \
         } in r",
    )
    .expect("independent nested motive remains the constant method path");
}

#[test]
fn nested_variable_index_without_a_dependent_tail_checks() {
    // The near-neighbour of a concrete index: a constructor-bound variable
    // is eligible even when this nested motive has no dependent convoy.
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\ndata HolderVar (a : Type) : Type where {{ \
           HoldVar : (n : Nat) → Vec a n → HolderVar a \
         }}\nfn variable_nested (a : Type) (h : HolderVar a) : Nat = \
         let r = match h {{ \
           HoldVar n VNil ↦ Zero; HoldVar n (VCons m _ _) ↦ m \
         }} in r"
    ))
    .expect("distinct variable index without a dependent tail checks");
}

#[test]
fn nested_concrete_index_advises_an_annotation() {
    let mut env = ElabEnv::new().expect("prelude");
    let error = env
        .elaborate_file(&format!(
            "{VEC}\ndata Holder (a : Type) : Type where {{ Hold : Vec a Zero → Holder a }}\n\
         fn no_equational_generalization (a : Type) (h : Holder a) : Nat = \
         let r = match h {{ Hold VNil ↦ Zero; Hold (VCons m _ _) ↦ m }} in r"
        ))
        .expect_err("concrete nested index needs an explicit separate match");
    assert!(
        matches!(&error, ElabError::TypeMismatch { reason, .. }
        if reason.contains("nested indexed split") && reason.contains("annotate")),
        "{error:?}"
    );
}

#[test]
fn unlowerable_pattern_field_result_has_surface_diagnostic() {
    // MEASURED: an inferred result type `Vec a m` uses the VCons-local `m`.
    // CLAIMED: a leaf-local type cannot escape into the indexed motive.
    // THE GAP: the checked twin below pins a lawful field-dependent branch.
    let mut env = ElabEnv::new().expect("prelude");
    let error = env
        .elaborate_file(&format!(
            "{VEC}\nfn field_result (a : Type) (n : Nat) (xs : Vec a (Suc n)) : Vec a n = \
             let r = match xs {{ VCons m x tl ↦ tl }} in r"
        ))
        .expect_err("inferred motive cannot contain the constructor-local m");
    assert!(
        matches!(error, ElabError::InferredMatchResultEscapesPattern {
            ref escaping_binder, ..
        } if escaping_binder.as_deref() == Some("m")),
        "{error:?}"
    );
    let rendered = error.to_string();
    assert!(
        rendered.contains("m") && rendered.contains("annotate"),
        "{rendered}"
    );
}

#[test]
fn checked_field_result_remains_supported() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\nfn field_result_checked (a : Type) (n : Nat) \
         (xs : Vec a (Suc n)) : Vec a n = \
         match xs {{ VCons m x tl ↦ tl }}"
    ))
    .expect("checked field-dependent match uses its expected motive");
}

#[test]
fn lowerable_inferred_match_remains_accepted() {
    // Controls the new refusal: the same inferred path with a closed Nat
    // result must lower. A blanket refusal would make this test red.
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\nfn lowerable (a : Type) (n : Nat) (xs : Vec a (Suc n)) : Nat = \
         let r = match xs {{ VCons m _ _ ↦ Suc m }} in r"
    ))
    .expect("inferred Nat result lowers outside every constructor field");
}

#[test]
fn reachable_missing_root_still_reports_unmatched_constructor() {
    let mut env = ElabEnv::new().expect("prelude");
    let error = env
        .elaborate_file(&format!(
            "{VEC}\nfn missing_root (a : Type) (n : Nat) (xs : Vec a n) : Nat = \
             let r = match xs {{ VCons m _ _ ↦ m }} in r"
        ))
        .expect_err("VNil is possible at an unconstrained index");
    assert!(
        matches!(error, ElabError::ExhaustivenessError { ref missing, .. }
            if missing.constructor == "VNil"),
        "{error:?}"
    );
}

#[test]
fn nested_column_omission_stays_refused() {
    // The root VNil is impossible at Suc n, but the nested tail may be VCons.
    let mut env = ElabEnv::new().expect("prelude");
    let error = env
        .elaborate_file(&format!(
            "{VEC}\nfn missing_nested (a : Type) (n : Nat) (xs : Vec a (Suc n)) : Nat = \
             let r = match xs {{ VCons m x VNil ↦ Zero }} in r"
        ))
        .expect_err("an indexed root cannot license nested-column omission");
    assert!(
        matches!(error, ElabError::ExhaustivenessError { ref missing, .. }
            if missing.constructor == "VCons"),
        "{error:?}"
    );
}
