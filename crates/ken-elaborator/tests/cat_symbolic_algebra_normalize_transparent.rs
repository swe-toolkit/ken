//! Normalization transparency: spec/50-stdlib/66-symbolic-algebra.md §9.
//! Source: conformance/stdlib/symbolic-algebra/
//! seed-nat-polynomial-normalize-transparent.md.
//! Promise class: durable invariant. A checked generic client is paired with
//! the full closed seed and its isolated moved-derivative-index refusal.
//! Unrelated expression/substitution extensions preserving §9 stay green;
//! changing normalization, index preservation or canonical equality must not.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, Expr as SourceExpr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::Decl;

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/symbolic-algebra/seed-nat-polynomial-normalize-transparent.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn package_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("NatPolynomial §9 package roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, normalize, poly_eq, deriv, substitute,
   normalize_poly_eq, deriv_normalize, substitute_normalize)

theorem generic_normalize_poly_eq (e : Expr)
    : IsTrue (poly_eq (normalize e) e) =
  normalize_poly_eq e

theorem generic_deriv_normalize (v : Nat) (e : Expr)
    : IsTrue (poly_eq (deriv v (normalize e)) (deriv v e)) =
  deriv_normalize v e

theorem generic_substitute_normalize (s : Nat → Expr) (e : Expr)
    : IsTrue (poly_eq (substitute s (normalize e)) (substitute s e)) =
  substitute_normalize s e
"#;

/// MEASURED: the exact trusted GlobalId set before and after roots-loading
/// NatPolynomial with its five imported providers, package ownership and
/// checked Transparent bodies for all three §9 public names, and an open
/// client consuming their exact premise-free types. CLAIMED: the §9 laws are
/// reusable with no new trusted assumptions. THE GAP: inherited provider
/// assumptions remain; the finite seed below is not a generic proof.
#[test]
fn three_generic_normalization_laws_are_checked_public_and_zero_trust() {
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
        .expect("all three §9 law bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§9 NatPolynomial trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§9 adds no trusted GlobalId identity");
    for name in [
        "normalize_poly_eq",
        "deriv_normalize",
        "substitute_normalize",
    ] {
        let id = env.globals[&format!("{PACKAGE}.{name}")];
        assert!(owned.contains(&id), "NatPolynomial owns {name}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a checked, non-axiom body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("open client consumes every §9 public theorem type");
}

// Identify source programs by checked declaration, not Markdown fence order.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§9 Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("§9 source fence parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            );
        if found {
            assert!(
                selected.replace(source).is_none(),
                "{theorem_name} must identify exactly one source program"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing §9 source theorem {theorem_name}"))
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected closed §9 proposition: {other:?}"),
    }
}

fn expr_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::ECon(name, _) | SourceExpr::EVar(name, _) => format!("name:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!("({} {})", expr_shape(function), expr_shape(argument))
        }
        other => panic!("unexpected closed §9 proof: {other:?}"),
    }
}

fn theorem_shape(source: &str, name: &str) -> (String, String) {
    let found = parse_decls(source)
        .expect("§9 theorem parses")
        .into_iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name: actual,
                theorem,
                body,
                ..
            } if actual == name => Some((type_shape(&theorem), expr_shape(&body))),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [one] = found.as_slice() else {
        panic!("exactly one theorem named {name} required");
    };
    one.clone()
}

const EXPECTED_CONTROLS: &str = r#"
theorem expected_same_index :
  IsTrue (poly_eq (deriv Zero (normalize e)) (deriv Zero e)) = Proved

theorem expected_moved_is_false :
  Equal Bool (poly_eq (deriv Zero (normalize e)) (deriv one e)) False = Proved

theorem expected_left_zero :
  Equal Nat (eval zero_env (deriv Zero (normalize e))) Zero = Proved

theorem expected_right_one :
  Equal Nat (eval zero_env (deriv one e)) one = Proved
"#;
const EXPECTED_REFUSAL: &str = r#"
theorem expected_refusal :
  IsTrue (poly_eq (deriv Zero (normalize e)) (deriv one e)) = Proved
"#;

/// MEASURED: the entire accepted CV source checks its independently built
/// normal form, three public-law instances, direct poly_eq oracles, a
/// non-identity variable swap and value eight, plus zero-versus-one derivative
/// observations and the same-index positive against the moved-index False.
/// A fresh package load accepts that complete prefix, then rejects only the
/// closed false theorem parsed from its own span. The expected false shape is
/// checked after the behavioral rejection so a true-neighbor seed mutation
/// reaches the rejection pin first. CLAIMED: the differentiation index cannot
/// be moved while preserving normalization transparency. THE GAP: this one
/// finite fixture does not establish the generic laws or official conformance.
#[test]
fn full_seed_accepts_and_moved_derivative_index_refuses() {
    let accepted = seed_case("normalize_poly_eq_instance");
    let refusal = seed_case("refuse_moved_derivative_index_after_normalization");
    let mut positive = package_env();
    positive
        .elaborate_file(accepted)
        .expect("complete §9 accepted client and independent oracles check");
    for name in [
        "explicit_normal_form",
        "normalize_poly_eq_instance",
        "normalize_poly_eq_oracle",
        "deriv_zero_normalize_instance",
        "deriv_zero_poly_eq_oracle",
        "deriv_one_normalize_instance",
        "deriv_one_poly_eq_oracle",
        "substitute_normalize_instance",
        "substitute_normalize_poly_eq_oracle",
        "swapped_expression_shape",
        "swapped_normal_form_shape",
        "swapped_expression_value",
        "swapped_normal_form_value",
        "deriv_zero_normalized_value_at_zero",
        "deriv_zero_original_value_at_zero",
        "deriv_one_normalized_value_at_zero",
        "deriv_one_original_value_at_zero",
        "moved_variable_poly_eq_is_false",
    ] {
        let id = positive.globals[name];
        assert!(
            matches!(positive.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} is a checked independent seed control"
        );
    }
    let decls = parse_decls(refusal).expect("isolated moved-index refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        span: claim_span,
        ..
    }] = decls.as_slice()
    else {
        panic!("refusal is exactly one source theorem");
    };
    assert_eq!(name, "refuse_moved_derivative_index_after_normalization");
    assert!(params.is_empty(), "moved-variable claim is closed");
    let claim = refusal
        .get(claim_span.start..claim_span.end)
        .expect("the sole theorem span is within the refused source");
    assert!(matches!(
        parse_decls(claim)
            .expect("isolated moved-index theorem span parses")
            .as_slice(),
        [SourceDecl::TheoremDecl { .. }]
    ));
    let mut refusing = package_env();
    refusing
        .elaborate_file(accepted)
        .expect("fresh complete accepted client succeeds before false claim");
    refusing
        .elaborate_file(claim)
        .expect_err("the isolated false moved-index proof must be refused");

    // These shape controls cannot mask a true-neighbor behavioral mutation:
    // each runs after the claim's own rejection assertion.
    for (actual, expected) in [
        ("deriv_zero_poly_eq_oracle", "expected_same_index"),
        ("moved_variable_poly_eq_is_false", "expected_moved_is_false"),
        ("deriv_zero_normalized_value_at_zero", "expected_left_zero"),
        ("deriv_one_original_value_at_zero", "expected_right_one"),
    ] {
        assert_eq!(
            theorem_shape(accepted, actual),
            theorem_shape(EXPECTED_CONTROLS, expected),
            "accepted §9 control {actual} keeps its checked proposition and proof"
        );
    }
    assert_eq!(
        theorem_shape(refusal, "refuse_moved_derivative_index_after_normalization"),
        theorem_shape(EXPECTED_REFUSAL, "expected_refusal"),
        "false claim must move only the derivative index and try Proved"
    );
}
