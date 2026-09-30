//! An inferred indexed match uses the dependent path's index premise to
//! discharge an omitted constructor, without admitting a reachable omission.
use ken_elaborator::error::ElabError;
use ken_elaborator::ElabEnv;
use ken_kernel::Decl;

const VEC: &str = r#"
data Vec (a : Type) : Nat → Type where {
  VNil : Vec a Zero;
  VCons : (n : Nat) → a → Vec a n → Vec a (Suc n)
}
"#;

#[test]
fn inferred_index_impossible_bucket_checks_without_new_trust() {
    // Promise class: durable invariant. MEASURED: a missing VNil method for
    // Vec a (Suc n) checks under an inferred match (not a checked RHS match),
    // and the admitted function is transparent with no extra trusted base.
    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before = env.env.trusted_base();
    env.elaborate_file(&format!(
        "{VEC}\nfn direct (a : Type) (n : Nat) (xs : Vec a (Suc n)) : Nat = \
         let outcome = match xs {{ VCons m _ _ ↦ m }} in Suc outcome"
    ))
    .expect("impossible VNil bucket must be synthesized");
    let id = env.globals["direct"];
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn inferred_nonnullary_impossible_bucket_checks_without_new_trust() {
    // Promise class: durable invariant. MEASURED: a missing VCons method,
    // with three fields and a recursive IH, is discharged for Vec a Zero.
    // The other constructor remains a checked transparent method.
    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before = env.env.trusted_base();
    env.elaborate_file(&format!(
        "{VEC}\nfn only_nil (a : Type) (xs : Vec a Zero) : Nat = \
         let outcome = match xs {{ VNil ↦ Zero }} in Suc outcome"
    ))
    .expect("impossible nonnullary VCons bucket must be synthesized");
    let id = env.globals["only_nil"];
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn reachable_omitted_constructor_keeps_its_exact_witness() {
    // Promise class: negative boundary. MEASURED: VNil remains reachable
    // for Vec a n, and the missing method reports its own constructor and
    // arity, rather than being filled by a new coverage rule.
    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before = env.env.trusted_base();
    let error = env
        .elaborate_file(&format!(
            "{VEC}\nfn missing (a : Type) (n : Nat) (xs : Vec a n) : Nat = \
             let outcome = match xs {{ VCons m _ _ ↦ m }} in Suc outcome"
        ))
        .expect_err("reachable VNil must not be discharged");
    assert!(
        matches!(error, ElabError::ExhaustivenessError { ref missing, .. }
        if missing.constructor == "VNil" && missing.arity == 0),
        "{error:?}"
    );
    assert_eq!(env.env.trusted_base(), trusted_before);
}
