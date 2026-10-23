//! Formal derivatives: spec/50-stdlib/66-symbolic-algebra.md §5.
//! The shipped seed's equal-function and square-product observations are
//! durable invariants, not a substitute for the checked generic proof bodies.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/symbolic-algebra/seed-nat-polynomial-derivative.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("NatPolynomial §5 roots-loads with checked laws");
    env
}

fn seed_cases() -> (&'static str, &'static str) {
    let mut fences = extract_ken_md(SEED)
        .expect("shipped derivative seed extracts")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| &SEED[fence.body_range])
        .collect::<Vec<_>>();
    let refusal = fences.pop().expect("separate derivative refusal");
    let [positive] = fences.as_slice() else {
        panic!("one complete positive client must precede the refusal");
    };
    (positive, refusal)
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, eval, poly_eq, deriv, deriv_respects_eval, poly_eq_deriv)

theorem client_deriv_respects_eval
    (v : Nat) (e1 : Expr) (e2 : Expr)
    (h : (g : Nat → Nat) → Equal Nat (eval g e1) (eval g e2))
    (env : Nat → Nat) :
  Equal Nat (eval env (deriv v e1)) (eval env (deriv v e2)) =
  deriv_respects_eval v e1 e2 h env

theorem client_poly_eq_deriv
    (v : Nat) (e1 : Expr) (e2 : Expr)
    (h : IsTrue (poly_eq e1 e2)) :
  IsTrue (poly_eq (deriv v e1) (deriv v e2)) =
  poly_eq_deriv v e1 e2 h
"#;

/// MEASURED: exact trusted GlobalId set equality before and after loading §5
/// on the five existing providers, public transparent bodies and a fresh
/// generic client, with the Taylor scaffold unimportable from another module.
/// CLAIMED: §5 exports both universal laws and `deriv` without new trust.
/// THE GAP: the existing providers retain their inherited trust; the seed
/// supplies closed product-rule controls, not a replacement for these laws.
#[test]
fn generic_derivative_exports_check_without_trust_or_public_scaffolding() {
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
        .expect("§5 operations and proof bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§5 trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(before.len(), 114, "five-provider trust inventory");
    assert_eq!(after, before, "§5 adds no trusted GlobalId");
    for public in ["deriv", "deriv_respects_eval", "poly_eq_deriv"] {
        let id = env.globals[&format!("{PACKAGE}.{public}")];
        assert!(owned.contains(&id), "{public} belongs to the owner");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{public} has a checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent client consumes both universal interfaces");
    for private in ["var_deriv", "deriv_taylor", "taylor_rem", "deriv_extract"] {
        match env.elaborate_file(&format!("import {PACKAGE} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{PACKAGE}.{private}"));
            }
            other => panic!("{private} must stay private: {other:?}"),
        }
    }
}

/// MEASURED: the full shipped positive client kernel-checks after roots-load,
/// including two independent derivative value-five oracles, the product
/// control at four, the x control at two and both public-law applications.
/// CLAIMED: §5's product rule and forward semantic preservation hold for the
/// closed seed as well as the checked universal exports above.
/// THE GAP: finite values alone cannot establish the generic theorems.
#[test]
fn shipped_derivative_seed_positive_client_checks() {
    let (positive, _) = seed_cases();
    let mut env = loaded_package();
    env.elaborate_file(positive)
        .expect("shipped derivative positive client checks completely");
    for name in [
        "originals_agree_at_every_environment",
        "factored_evaluates_to_six",
        "distributed_evaluates_to_six",
        "factored_derivative_evaluates_to_five",
        "distributed_derivative_evaluates_to_five",
        "derivative_evaluations_agree",
        "derivative_poly_eq_remains_true",
        "square_derivative_matches_doubled",
        "square_derivative_is_four_at_two",
        "doubled_is_four_at_two",
        "x_is_two_at_two",
        "square_derivative_equals_double_at_two",
    ] {
        assert!(
            matches!(
                env.env.lookup(env.globals[name]),
                Some(Decl::Transparent { .. })
            ),
            "{name} is a checked positive observation"
        );
    }
}

fn theorem<'a>(declarations: &'a [SourceDecl], name: &str) -> &'a SourceType {
    declarations
        .iter()
        .find_map(|declaration| match declaration {
            SourceDecl::TheoremDecl {
                name: found,
                theorem,
                ..
            } if found == name => Some(theorem),
            _ => None,
        })
        .unwrap_or_else(|| panic!("seed must contain theorem {name}"))
}

fn shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => name.clone(),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", shape(function), shape(argument))
        }
        other => panic!("unexpected equality proposition: {other:?}"),
    }
}

fn equal_nat(ty: &SourceType) -> (String, String) {
    let SourceType::TApp(lhs_and_carrier, rhs, _) = ty else {
        panic!("seed claim must be Equal Nat with two operands");
    };
    let SourceType::TApp(equal_and_carrier, lhs, _) = lhs_and_carrier.as_ref() else {
        panic!("seed claim must be Equal Nat with two operands");
    };
    let SourceType::TApp(equal, carrier, _) = equal_and_carrier.as_ref() else {
        panic!("seed claim must be Equal Nat with two operands");
    };
    assert_eq!(shape(equal), "Equal");
    assert_eq!(shape(carrier), "Nat");
    (shape(lhs), shape(rhs))
}

/// MEASURED: the accepted prefix fixes different values four and two on the
/// very two operands in the false universal theorem. The refusal has exactly
/// the intended Nat-to-Nat environment binder and its claim type separately
/// checks when supplied as a hypothesis. A fresh prefix then checks before
/// that same claim with `Proved` kernel-refuses; setup errors do not count.
/// CLAIMED: the derivative of x·x is not universally the value of x.
/// THE GAP: this single counterexample does not characterize other products;
/// the generic proof and the separate value-five cases provide broader checks.
#[test]
fn shipped_square_derivative_as_one_x_refuses_at_its_own_equality() {
    let (positive, refusal) = seed_cases();
    let accepted = parse_decls(positive).expect("positive client parses");
    let (derivative_at_two, four) =
        equal_nat(theorem(&accepted, "square_derivative_is_four_at_two"));
    let (x_at_two, two) = equal_nat(theorem(&accepted, "x_is_two_at_two"));
    let (double_at_two, double_four) = equal_nat(theorem(&accepted, "doubled_is_four_at_two"));
    let (same_derivative, same_double) =
        equal_nat(theorem(&accepted, "square_derivative_equals_double_at_two"));
    assert_eq!(four, double_four, "both values are four");
    assert_ne!(four, two, "the closed counterexample is nondegenerate");
    assert_eq!(derivative_at_two, same_derivative);
    assert_eq!(double_at_two, same_double);
    assert_ne!(derivative_at_two, x_at_two);

    let refused = parse_decls(refusal).expect("refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        theorem: false_goal,
        body: SourceExpr::ECon(proof, _),
        ..
    }] = refused.as_slice()
    else {
        panic!("refusal is precisely one universally quantified theorem with a proof");
    };
    assert_eq!(name, "refuse_square_derivative_as_one_x");
    let [binder] = params.as_slice() else {
        panic!("the refused claim has exactly one environment binder");
    };
    assert_eq!(
        binder.names,
        ["env"],
        "the claim binds the queried environment"
    );
    let SourceType::TArr(domain, codomain, _) = &binder.ty else {
        panic!("the refusal environment must be a total Nat-to-Nat function");
    };
    assert_eq!(shape(domain), "Nat", "domain of the refused environment");
    assert_eq!(
        shape(codomain),
        "Nat",
        "codomain of the refused environment"
    );
    assert_eq!(proof, "Proved");
    let (refused_derivative, refused_x) = equal_nat(false_goal);
    assert_eq!(
        refused_derivative.replace("env", "env_two"),
        derivative_at_two,
        "same square derivative as the accepted four oracle"
    );
    assert_eq!(
        refused_x.replace("env", "env_two"),
        x_at_two,
        "same x as the accepted two oracle"
    );

    let mut env = loaded_package();
    env.elaborate_file(positive)
        .expect("all positive controls check before refusal");
    // A hypothetical witness discharges the exact parsed claim without adding
    // an axiom. This checks its type before the refusal asks `Proved` to inhabit
    // that claim: a kernel error while forming the proposition cannot count.
    let claim_span = false_goal.span();
    let claimed_equality = &refusal[claim_span.start..claim_span.end];
    let well_typed_claim = format!(
        "theorem refusal_claim_typechecks (env : Nat → Nat) (h : {claimed_equality}) : {claimed_equality} = h"
    );
    env.elaborate_file(&well_typed_claim)
        .expect("the refused proposition itself is well-typed under the same binder");
    match env.elaborate_file(refusal) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start < span.end && span.end <= refusal.len() => {}
        other => panic!("false square-derivative equality must kernel-refuse: {other:?}"),
    }
}
