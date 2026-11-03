//! Pointwise congruence of polynomial substitution: spec 66 §13.
//! This package-owned test checks the generic export and closed clients;
//! it does not execute the separate conformance seed.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("NatPolynomial roots-loads with its pointwise substitution law");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, poly_eq, substitute, poly_eq_substitute_pointwise)

theorem generic_client
    (s : Nat → Expr) (t : Nat → Expr) (e : Expr)
    (h : (v : Nat) → IsTrue (poly_eq (s v) (t v)))
  : IsTrue (poly_eq (substitute s e) (substitute t e)) =
  poly_eq_substitute_pointwise s t e h
"#;

/// Promise class: durable invariant (spec 66 §13).
///
/// MEASURED: complete trusted-base GlobalId sets agree before and after
/// roots-loading NatPolynomial on its providers; the new public law is owned
/// and Transparent; a selective-import client applies its generic type at
/// arbitrary substitutions and expression under only the pointwise premise.
/// CLAIMED: the law is checked and adds no trusted identity to the package.
/// THE GAP: provider trust remains inherited, and the generic client cannot
/// by itself distinguish a false closed pointwise premise.
#[test]
fn generic_pointwise_law_is_owned_checked_and_trust_free() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Collections.Derived",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("pointwise law kernel-checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§13 trusted GlobalIds: before={before:?} after={after:?}");
    assert_eq!(after, before, "§13 must add no trusted GlobalId identity");
    let name = format!("{PACKAGE}.poly_eq_substitute_pointwise");
    let id = env.globals[&name];
    assert!(
        owned.contains(&id),
        "pointwise law belongs to NatPolynomial"
    );
    assert!(
        matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
        "pointwise law has a checked body"
    );
    env.elaborate_file(GENERIC_CLIENT)
        .expect("generic selective-import client accepts the public contract");
}

const CLOSED_CLIENTS: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EVar, EAdd, EMul, poly_eq, poly_eq_refl, substitute,
   poly_eq_substitute_pointwise)

const one : Nat = Suc Zero
const e0 : Expr = EMul (EVar Zero) (EVar Zero)

fn sum_left (v : Nat) : Expr =
  match v {
    Zero ↦ EAdd (EVar Zero) (EVar one);
    Suc w ↦ EVar (Suc w)
  }

fn sum_right (v : Nat) : Expr =
  match v {
    Zero ↦ EAdd (EVar one) (EVar Zero);
    Suc w ↦ EVar (Suc w)
  }

theorem images_agree (v : Nat) : IsTrue (poly_eq (sum_left v) (sum_right v)) =
  match v {
    Zero ↦ Proved;
    Suc w ↦ poly_eq_refl (EVar (Suc w))
  }

theorem client_through_law :
  IsTrue (poly_eq (substitute sum_left e0) (substitute sum_right e0)) =
  poly_eq_substitute_pointwise sum_left sum_right e0 images_agree

theorem control_direct :
  IsTrue (poly_eq (substitute sum_left e0) (substitute sum_right e0)) = Proved
"#;

const REFUSE_DIRECT: &str = r#"
fn drop_left (v : Nat) : Expr =
  match v {
    Zero ↦ EVar one;
    Suc w ↦ EVar (Suc w)
  }

theorem refuse_direct :
  IsTrue (poly_eq (substitute sum_left e0) (substitute drop_left e0)) = Proved
"#;

const REFUSE_IMAGES: &str = r#"
fn drop_left (v : Nat) : Expr =
  match v {
    Zero ↦ EVar one;
    Suc w ↦ EVar (Suc w)
  }

theorem refuse_images (v : Nat) : IsTrue (poly_eq (sum_left v) (drop_left v)) =
  match v {
    Zero ↦ Proved;
    Suc w ↦ poly_eq_refl (EVar (Suc w))
  }
"#;

const FALSE_CONTROL: &str = r#"
fn drop_left (v : Nat) : Expr =
  match v {
    Zero ↦ EVar one;
    Suc w ↦ EVar (Suc w)
  }

theorem images_differ :
  Equal Bool (poly_eq (sum_left Zero) (drop_left Zero)) False = Proved

theorem results_differ :
  Equal Bool (poly_eq (substitute sum_left e0) (substitute drop_left e0)) False = Proved
"#;

fn refuses_at_own_theorem(name: &str, negative: &str) {
    let combined = format!("{CLOSED_CLIENTS}\n{negative}");
    let mut env = loaded_package();
    match env.elaborate_file(&combined) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) => {
            let theorem_start = combined
                .find(&format!("theorem {name}"))
                .expect("isolated refusal names its theorem");
            assert!(
                span.start >= theorem_start && span.start < span.end && span.end <= combined.len(),
                "{name} must refuse at its own proof: {span:?}"
            );
        }
        other => panic!("{name} must refuse with kernel TypeMismatch: {other:?}"),
    }
}

/// Promise class: durable invariant (spec 66 §13, package-owned witnesses).
///
/// MEASURED: the law application and a direct Proved same-form control accept
/// under two pointwise equal images; the changed-image fixture proves both
/// image and result decisions False. Separate inputs then refuse the direct
/// wrong-image claim and its false image premise at their own proof sites.
/// CLAIMED: the law does not make a false substitution result provable.
/// THE GAP: these closed rows are not official conformance-runner execution.
#[test]
fn closed_pointwise_clients_accept_and_false_images_refuse() {
    let mut accepted = loaded_package();
    accepted
        .elaborate_file(CLOSED_CLIENTS)
        .expect("the law application and same-form direct control accept");
    let mut false_control = loaded_package();
    false_control
        .elaborate_file(&format!("{CLOSED_CLIENTS}\n{FALSE_CONTROL}"))
        .expect("changed image and result are both decidably False");
    refuses_at_own_theorem("refuse_direct", REFUSE_DIRECT);
    refuses_at_own_theorem("refuse_images", REFUSE_IMAGES);
}
