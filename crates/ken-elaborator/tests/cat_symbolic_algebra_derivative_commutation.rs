//! Partial derivatives commute in evaluation: spec/50-stdlib/66-symbolic-algebra.md §6.
//! SOURCE: conformance/stdlib/symbolic-algebra/seed-derivative-commutation.md.
//! PROMISE CLASS: durable invariant. Additional unrelated source cases in the
//! seed do not change which accepted and refused Ken clients are exercised.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::Decl;

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/symbolic-algebra/seed-derivative-commutation.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("NatPolynomial §6 roots-loads with a checked proof");
    env
}

// A theorem name identifies the program, never its fence position or the
// total number of source cases in the evolving seed document.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§6 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let has_theorem = parse_decls(source)
            .expect("seed source client parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            );
        if has_theorem {
            assert!(
                selected.replace(source).is_none(),
                "{theorem_name} must identify one seed client"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing seed client {theorem_name}"))
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, eval, deriv, deriv_commute, poly_eq, poly_eq_complete)

theorem client_deriv_commute
    (v : Nat) (w : Nat) (e : Expr) (env : Nat → Nat) :
  Equal Nat (eval env (deriv v (deriv w e)))
    (eval env (deriv w (deriv v e))) =
  deriv_commute v w e env

theorem client_poly_eq_decision
    (v : Nat) (w : Nat) (e : Expr) :
  IsTrue (poly_eq (deriv v (deriv w e)) (deriv w (deriv v e))) =
  poly_eq_complete (deriv v (deriv w e)) (deriv w (deriv v e))
    (λenv. deriv_commute v w e env)
"#;

/// MEASURED: before/after exact trusted GlobalId inventory from the same
/// checked provider closure, root-loaded package ownership/Transparency, full
/// arbitrarily quantified client, and selective import rejection of the
/// helper. CLAIMED: §6 exports the generic zero-new-trust commutation proof
/// without exporting its private variable-case scaffold. THE GAP: inherited
/// provider trust is preserved, not eliminated by this local equality.
#[test]
fn generic_commutation_is_owned_checked_and_adds_no_trust() {
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
        .expect("§6 operation and proof bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§6 trust identities: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§6 does not add trusted GlobalIds");
    let id = env.globals[&format!("{PACKAGE}.deriv_commute")];
    assert!(owned.contains(&id), "law belongs to NatPolynomial");
    assert!(
        matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
        "law has a checked body"
    );
    env.elaborate_file(GENERIC_CLIENT)
        .expect("arbitrary-argument law and existing poly_eq corollary check");
    match env.elaborate_file(&format!("import {PACKAGE} (deriv_var_deriv_eval)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{PACKAGE}.deriv_var_deriv_eval"));
        }
        other => panic!("§6 variable helper must stay private: {other:?}"),
    }
}

/// MEASURED: the full accepted Ken client checks after the package roots-loads:
/// quantified `xy_commute`, both one-valued closed derivatives, and same-tree
/// structural `Refl`. CLAIMED: the shipped theorem works for this expression
/// in all environments, but does not equate derivative expression syntax.
/// THE GAP: the closed product and env23 alone cannot establish universality;
/// the generic client and the kernel-checked public body above are essential.
#[test]
fn shipped_commutation_client_checks() {
    let positive = seed_case("xy_commute");
    loaded_package()
        .elaborate_file(positive)
        .expect("all accepted §6 seed theorems and same-tree control check");
}

fn shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", shape(function), shape(argument))
        }
        other => panic!("unexpected §6 source type form: {other:?}"),
    }
}

fn applied<'a>(ty: &'a SourceType, name: &str) -> Vec<&'a SourceType> {
    let mut args = Vec::new();
    let mut head = ty;
    while let SourceType::TApp(function, argument, _) = head {
        args.push(argument.as_ref());
        head = function;
    }
    args.reverse();
    assert!(
        matches!(head, SourceType::TCon(found, _) | SourceType::TVar(found, _) if found == name),
        "expected {name}, found {head:?}"
    );
    args
}

fn theorem<'a>(decls: &'a [SourceDecl], name: &str) -> &'a SourceType {
    let mut matches = decls.iter().filter_map(|decl| match decl {
        SourceDecl::TheoremDecl {
            name: found,
            theorem,
            ..
        } if found == name => Some(theorem),
        _ => None,
    });
    let found = matches
        .next()
        .unwrap_or_else(|| panic!("seed theorem {name} is missing"));
    assert!(matches.next().is_none(), "seed theorem {name} is ambiguous");
    found
}

fn equality(ty: &SourceType) -> [&SourceType; 3] {
    applied(ty, "Equal")
        .try_into()
        .expect("a closed equality has one carrier and two endpoints")
}

/// MEASURED: the accepted `xy_left_self` same-operand `Equal Expr` Refl and
/// both closed value controls check before attempting the negative theorem;
/// the refusal's AST uses the same left expression, swaps only its right
/// derivative order, and TypeMismatch fires at that theorem's Refl span.
/// CLAIMED: semantic commutation does not entail structural equality of the
/// two constructed derivative expression trees on xy. THE GAP: one product
/// refutes only the false structural claim, not every expression shape.
#[test]
fn cross_order_structural_equality_refuses_at_its_own_refl() {
    let positive = seed_case("xy_commute");
    let negative = seed_case("refuse_derivative_tree_commutation");
    let positives = parse_decls(positive).expect("accepted client parses");
    let negatives = parse_decls(negative).expect("refused client parses");
    let mut matching = negatives.iter().filter(|decl| {
        matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == "refuse_derivative_tree_commutation")
    });
    let Some(SourceDecl::TheoremDecl {
        params,
        theorem: refused,
        body,
        span: refusal_span,
        ..
    }) = matching.next()
    else {
        panic!("refused theorem must be present");
    };
    assert!(
        matching.next().is_none(),
        "refusal name must be unambiguous"
    );
    assert!(params.is_empty(), "structural refusal must be closed");
    assert!(
        matches!(body, SourceExpr::ECon(name, _) if name == "Refl"),
        "structural refusal must try Refl"
    );
    let [carrier, left, right] = equality(refused);
    let [same_carrier, same_left, same_right] = equality(theorem(&positives, "xy_left_self"));
    assert_eq!(
        shape(carrier),
        "name:Expr",
        "claim is structural Expr equality"
    );
    assert_eq!(shape(same_carrier), shape(carrier));
    assert_eq!(shape(left), shape(same_left), "same left derivative tree");
    assert_eq!(shape(left), shape(same_right), "control is reflexive");
    assert_ne!(
        shape(left),
        shape(right),
        "two orders yield distinct syntax"
    );
    let [left_carrier, left_value, left_one] = equality(theorem(&positives, "xy_left_value"));
    let [right_carrier, right_value, right_one] = equality(theorem(&positives, "xy_right_value"));
    assert_eq!(shape(left_carrier), "name:Nat");
    assert_eq!(shape(left_carrier), shape(right_carrier));
    assert_eq!(shape(left_one), shape(right_one), "both values are one");
    for (evaluated, derivative) in [(left_value, left), (right_value, right)] {
        let [environment, expression] = applied(evaluated, "eval")
            .try_into()
            .expect("closed value applies eval to environment and tree");
        assert_eq!(shape(environment), "name:env23");
        assert_eq!(shape(expression), shape(derivative));
    }

    let mut accepted = loaded_package();
    accepted
        .elaborate_file(positive)
        .expect("full accepted prefix checks before refusal");
    let claim_span = refused.span();
    let claim = &negative[claim_span.start..claim_span.end];
    accepted
        .elaborate_file(&format!(
            "theorem assumed_structural_claim (h : {claim}) : {claim} = h"
        ))
        .expect("the false proposition is itself a well-typed equality");
    let mut env = loaded_package();
    env.elaborate_file(positive)
        .expect("fresh refusal path checks every accepted control first");
    match env.elaborate_file(negative) {
        Err(ElabError::TypeMismatch { reason, span })
            if reason == "Refl expects an `Eq`-shaped goal"
                && span.start >= refusal_span.start
                && span.end <= refusal_span.end
                && negative.get(span.start..span.end) == Some("Refl") => {}
        other => panic!("false structural §6 equality must reject at Refl: {other:?}"),
    }
}
