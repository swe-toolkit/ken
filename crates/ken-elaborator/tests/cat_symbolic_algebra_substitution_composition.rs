//! Composition and constructor identity: spec/50-stdlib/66-symbolic-algebra.md §4.
//! Source: conformance/stdlib/symbolic-algebra/
//! seed-nat-polynomial-substitution-composition.md.
//! Promise class: durable invariant. The fixture's two structural outputs
//! separate applying `inner0` then `outer0` from the opposite order.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole, Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/symbolic-algebra/seed-nat-polynomial-substitution-composition.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("NatPolynomial §4 roots-loads with checked proofs");
    env
}

fn seed_parts() -> (Vec<&'static str>, &'static str, &'static str, &'static str) {
    let mut sources = extract_ken_md(SEED)
        .expect("shipped composition seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| &SEED[fence.body_range])
        .collect::<Vec<_>>();
    let refusal = sources.pop().expect("final separate refusal fence");
    let containing = |name: &str| {
        sources
            .iter()
            .copied()
            .find(|source| {
                parse_decls(source)
                    .expect("shipped positive seed parses")
                    .iter()
                    .any(|declaration| {
                        matches!(declaration, SourceDecl::TheoremDecl { name: found, .. } if found == name)
                    })
            })
            .unwrap_or_else(|| panic!("shipped seed needs the {name} client"))
    };
    let forward = containing("forward_nested_has_expected_tree");
    let base = containing("refusal_fixture_forward_law");
    assert_ne!(forward, base, "independent seed clients");
    (sources, forward, base, refusal)
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EVar, substitute, compose_subst, substitute_compose, substitute_var)

theorem client_substitute_compose
    (s : Nat → Expr) (t : Nat → Expr) (e : Expr) :
  Equal Expr (substitute s (substitute t e))
    (substitute (compose_subst s t) e) =
  substitute_compose s t e

theorem client_substitute_var (e : Expr) :
  Equal Expr (substitute EVar e) e =
  substitute_var e
"#;

/// SOURCE: spec/50-stdlib/66-symbolic-algebra.md §4.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the exact trusted GlobalId set stays identical before/after
/// roots-loading §4 on the five published providers; three public package-owned
/// transparent bodies satisfy independently restated generic theorem types;
/// the two congruence helpers cannot be selectively imported.
/// CLAIMED: the §4 builder and two structural proofs are exported at zero new
/// trust with no public proof scaffolding.
/// THE GAP: existing provider trust remains inherited. The separately loaded
/// seed supplies closed order observations, not a generic proof replacement.
#[test]
fn generic_composition_exports_check_without_trust_or_private_helpers() {
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
        .expect("§4 operations and universal proofs kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§4 trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§4 adds no trusted GlobalId");
    for public in ["compose_subst", "substitute_compose", "substitute_var"] {
        let id = env.globals[&format!("{PACKAGE}.{public}")];
        assert!(owned.contains(&id), "{public} belongs to NatPolynomial");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{public} has a checked transparent body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("client independently consumes both generic structural propositions");
    for private in ["expr_add_cong", "expr_mul_cong"] {
        match env.elaborate_file(&format!("import {PACKAGE} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{PACKAGE}.{private}"));
            }
            other => panic!("{private} must remain private: {other:?}"),
        }
    }
}

/// SOURCE: the §4 composition seed's closed fixture and reversed-order base.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the shipped accepted clients run in fresh roots-loaded
/// environments. All six positive theorems in each fence are kernel-checked,
/// including both exact forward/reversed trees and both public law instances.
/// CLAIMED: composition follows the specified order and constructor `EVar`
/// acts as the identity on the closed fixture.
/// THE GAP: these closed cases alone cannot establish universal laws; the
/// preceding client checks the generic statements against the public proofs.
#[test]
fn shipped_composition_seed_positive_clients_check() {
    let (positives, forward, base, _) = seed_parts();
    // Each positive fence checks in its own fresh client environment: a future
    // additional positive seed case runs too, without freezing the fence count.
    for source in positives {
        let mut env = loaded_package();
        env.elaborate_file(source)
            .expect("each shipped composition positive client checks");
    }
    let mut env = loaded_package();
    env.elaborate_file(forward)
        .expect("shipped forward and identity client checks");
    for name in [
        "forward_nested_has_expected_tree",
        "forward_composed_map_has_expected_tree",
        "public_forward_composition_instance",
        "reversed_builder_has_distinct_tree",
        "constructor_identity_reduces",
        "constructor_identity_public_law",
    ] {
        assert!(
            matches!(
                env.env.lookup(env.globals[name]),
                Some(Decl::Transparent { .. })
            ),
            "{name} is a checked seed control"
        );
    }
    let mut refusal_base = loaded_package();
    refusal_base
        .elaborate_file(base)
        .expect("shipped refusal base alone checks all six controls");
    for name in [
        "refusal_fixture_forward_output",
        "refusal_fixture_forward_composed_output",
        "refusal_fixture_forward_law",
        "refusal_fixture_reverse_nested_output",
        "refusal_fixture_reverse_composed_output",
        "refusal_fixture_reverse_law",
    ] {
        assert!(
            matches!(
                refusal_base.env.lookup(refusal_base.globals[name]),
                Some(Decl::Transparent { .. })
            ),
            "{name} is a checked refusal-base control"
        );
    }
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unsupported §4 seed proposition form: {other:?}"),
    }
}

fn application<'a>(ty: &'a SourceType, name: &str) -> Option<Vec<&'a SourceType>> {
    let mut args = Vec::new();
    let mut head = ty;
    while let SourceType::TApp(function, argument, _) = head {
        args.push(argument.as_ref());
        head = function;
    }
    args.reverse();
    match head {
        SourceType::TVar(found, _) | SourceType::TCon(found, _) if found == name => Some(args),
        _ => None,
    }
}

fn theorem_type<'a>(declarations: &'a [SourceDecl], name: &str) -> &'a SourceType {
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
        .unwrap_or_else(|| panic!("shipped seed must include theorem {name}"))
}

fn equal_expr_endpoints(theorem: &SourceType) -> (String, String) {
    let parts = application(theorem, "Equal").expect("proposition must be Equal");
    let [carrier, lhs, rhs] = parts.as_slice() else {
        panic!("Equal has a carrier and exactly two operands");
    };
    assert_eq!(type_shape(carrier), "variable:Expr");
    (type_shape(lhs), type_shape(rhs))
}

// The intended negative is fixed by independently accepted, *non-degenerate*
// forward and reverse seed controls. Any unrelated false equality must fail
// this proposition pin even if its proof would also be kernel-rejected.
fn assert_reversed_order_proposition(base: &str, refused: &str) {
    let base_declarations = parse_decls(base).expect("shipped refusal base parses");
    let (_, forward_map) = equal_expr_endpoints(theorem_type(
        &base_declarations,
        "refusal_fixture_forward_composed_output",
    ));
    let (_, reverse_map) = equal_expr_endpoints(theorem_type(
        &base_declarations,
        "refusal_fixture_reverse_composed_output",
    ));
    assert_ne!(forward_map, reverse_map, "closed expected outputs differ");
    let (forward_nested, forward_composed) = equal_expr_endpoints(theorem_type(
        &base_declarations,
        "refusal_fixture_forward_law",
    ));
    let (_, reverse_composed) = equal_expr_endpoints(theorem_type(
        &base_declarations,
        "refusal_fixture_reverse_law",
    ));
    assert_ne!(
        forward_composed, reverse_composed,
        "composition orders have distinct checked arguments"
    );
    let (forward_observed, _) = equal_expr_endpoints(theorem_type(
        &base_declarations,
        "refusal_fixture_forward_composed_output",
    ));
    let (reverse_observed, _) = equal_expr_endpoints(theorem_type(
        &base_declarations,
        "refusal_fixture_reverse_composed_output",
    ));
    assert_eq!(forward_composed, forward_observed);
    assert_eq!(reverse_composed, reverse_observed);

    let refused_declarations = parse_decls(refused).expect("shipped refusal parses");
    let [SourceDecl::TheoremDecl { name, theorem, .. }] = refused_declarations.as_slice() else {
        panic!("refusal block must contain exactly its one theorem");
    };
    assert_eq!(name, "refuse_reversed_composition_claim");
    let (refused_left, refused_right) = equal_expr_endpoints(theorem);
    assert_eq!(refused_left, forward_nested, "same forward nested fixture");
    assert_eq!(refused_right, reverse_composed, "same reverse-map fixture");
}

/// SOURCE: §4 seed `reject-reversed-substitution-composition`.
/// PROMISE CLASS: durable invariant.
/// MEASURED: accepted forward/reverse control theorems fix the exact same
/// operands of the one refused theorem; distinct outputs are independently
/// checked in a closed counterexample; the refused theorem alone then reaches
/// a kernel `TypeMismatch` at its own proof, not a setup failure.
/// CLAIMED: applying the generic forward composition proof to the reverse
/// builder cannot establish the forward result on this noncommuting fixture.
/// THE GAP: this is a closed order discriminator; the generic composition
/// theorem itself is checked independently by the package's proof body.
#[test]
fn shipped_reversed_composition_claim_refuses_on_checked_closed_fixture() {
    let (_, _, base, refused) = seed_parts();
    assert_reversed_order_proposition(base, refused);

    let mut env = loaded_package();
    env.elaborate_file(base)
        .expect("all six reversed-order base controls check before the refusal");
    match env.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start < span.end && span.end <= refused.len() => {}
        other => panic!("reversed-composition theorem must kernel-refuse: {other:?}"),
    }

    // The two accepted output definitions are *observably different*, not
    // merely distinct identifiers for convertible expressions.
    let mut distinct = loaded_package();
    distinct
        .elaborate_file(base)
        .expect("the same base checks in a fresh non-degenerate witness");
    let false_equality =
        "theorem distinct_order_control : Equal Expr bad_forward0 bad_reverse0 = Refl";
    match distinct.elaborate_file(false_equality) {
        Err(ElabError::TypeMismatch { span, .. })
            if false_equality.get(span.start..span.end) == Some("Refl") => {}
        other => panic!("forward/reverse trees must be non-convertible: {other:?}"),
    }
}
