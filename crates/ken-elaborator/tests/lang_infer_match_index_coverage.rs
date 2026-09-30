//! An inferred indexed match uses the dependent path's index premise to
//! discharge an omitted constructor, without admitting a reachable omission.
//!
//! Spec: `spec/30-surface/34-data-match.md §4.1–4.3`.
//! Conformance: `surface/data-match/indexed-impossible-pair` (TR5b omission)
//! in `conformance/surface/data-match/seed-data-match.md`. Its separate TR5a
//! impossible-application case is not claimed by these tests.
use ken_elaborator::error::ElabError;
use ken_elaborator::ElabEnv;
use ken_kernel::{normalize, Context, Decl, Term};

const VEC: &str = r#"
data Vec (a : Type) : Nat → Type where {
  VNil : Vec a Zero;
  VCons : (n : Nat) → a → Vec a n → Vec a (Suc n)
}
"#;

fn nat(env: &ElabEnv, successors: usize) -> Term {
    let zero = Term::Constructor {
        id: env.globals["Zero"],
        level_args: vec![],
    };
    let suc = Term::Constructor {
        id: env.globals["Suc"],
        level_args: vec![],
    };
    (0..successors).fold(zero, |value, _| Term::app(suc.clone(), value))
}

fn observed_closed_nat(env: &mut ElabEnv, source: &str) -> Term {
    let id = env
        .elaborate_decl(source)
        .expect("closed observation checks");
    let (_, body) = env
        .env
        .transparent_body(id)
        .expect("checked observation body");
    normalize(&env.env, &Context::new(), &body)
}

#[test]
fn inferred_index_impossible_bucket_checks_without_new_trust() {
    // Promise class: durable invariant. MEASURED: a missing VNil method for
    // Vec a (Suc n) checks under an inferred match (not a checked RHS match).
    // CLAIMED: the VCons method returns its index field m and the complete
    // elim computes it. THE GAP: one value could conceal a constant wrong
    // method. At n=0 the index m=0 and element x=1, yielding 1; at n=1 the
    // index m=1 and element x=0, yielding 2. Either a constant-one method or
    // an element-returning method disagrees at one of these two checked inputs.
    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before = env.env.trusted_base();
    env.elaborate_file(&format!(
        "{VEC}\nfn direct (a : Type) (n : Nat) (xs : Vec a (Suc n)) : Nat = \
         let outcome = match xs {{ VCons m _ _ ↦ m }} in Suc outcome"
    ))
    .expect("impossible VNil bucket must be synthesized");
    let id = env.globals["direct"];
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    let observed_zero = observed_closed_nat(
        &mut env,
        "const direct_at_zero : Nat = direct Nat Zero \
         (VCons Nat Zero (Suc Zero) (VNil Nat))",
    );
    assert_eq!(observed_zero, nat(&env, 1));
    let observed_one = observed_closed_nat(
        &mut env,
        "const direct_at_one : Nat = direct Nat (Suc Zero) \
         (VCons Nat (Suc Zero) Zero (VCons Nat Zero Zero (VNil Nat)))",
    );
    assert_eq!(observed_one, nat(&env, 2));
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn inferred_nonnullary_impossible_bucket_checks_without_new_trust() {
    // Promise class: durable invariant. MEASURED: a missing VCons method,
    // with three fields and a recursive IH, is discharged for Vec a Zero.
    // CLAIMED: the present VNil method returns Zero and computes to Suc Zero
    // through the wrapper. THE GAP: admission alone cannot tell Zero from a
    // different well-typed result; the closed VNil observation does.
    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before = env.env.trusted_base();
    env.elaborate_file(&format!(
        "{VEC}\nfn only_nil (a : Type) (xs : Vec a Zero) : Nat = \
         let outcome = match xs {{ VNil ↦ Zero }} in Suc outcome"
    ))
    .expect("impossible nonnullary VCons bucket must be synthesized");
    let id = env.globals["only_nil"];
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    let observed = observed_closed_nat(
        &mut env,
        "const nil_observed : Nat = only_nil Nat (VNil Nat)",
    );
    assert_eq!(observed, nat(&env, 1));
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn reachable_omitted_constructor_keeps_its_exact_witness() {
    // Promise class: durable invariant (negative boundary).
    // MEASURED: VNil remains reachable for Vec a n, and the missing method
    // reports its own constructor and
    // arity, rather than being filled by a new coverage rule. CLAIMED: §4.3's
    // type-possible constructor is required. THE GAP: the positive controls
    // above establish an actual omitted-impossible method at a fixed index.
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
