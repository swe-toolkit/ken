//! Argmax cost-order invariance: spec/50-stdlib/72-optimization.md §3.5.
//! Source: conformance/stdlib/optimization/seed-argmax-cost-order-invariance.md.
//! Promise class: durable invariant. A non-head earlier tied maximum survives
//! strict-order-preserving cost transformation; flattening selects the head.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const PACKAGE: &str = "Algorithm.Optimization.Argmin";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/optimization/seed-argmax-cost-order-invariance.md"
));
const PRIVATE_HELPERS: [&str; 3] = ["select_from_cost_order", "choose_cost_order", "costlier"];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_argmin() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("Argmin/Argmax §3.5 checked package root-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Optimization.Argmin
  (argmin, argmin_cost_order_invariant, argmax, argmax_cost_order_invariant)
import Core.Function.Combinators (comp)
import Data.Collections.NonEmpty (NonEmpty)
import Data.Numeric.Nat.Order (lt_nat)

theorem client_argmax_cost_order
    (a : Type) (cost : a → Nat) (f : Nat → Nat)
    (same_order : (i : Nat) → (j : Nat) → Equal Bool (lt_nat (f i) (f j)) (lt_nat i j))
    (xs : NonEmpty a) :
  Equal a (argmax a (comp a Nat Nat f cost) xs) (argmax a cost xs) =
  argmax_cost_order_invariant a cost f same_order xs

theorem client_argmin_cost_order
    (a : Type) (cost : a → Nat) (f : Nat → Nat)
    (same_order : (i : Nat) → (j : Nat) → Equal Bool (lt_nat (f i) (f j)) (lt_nat i j))
    (xs : NonEmpty a) :
  Equal a (argmin a (comp a Nat Nat f cost) xs) (argmin a cost xs) =
  argmin_cost_order_invariant a cost f same_order xs
"#;

/// MEASURED: the exact trusted GlobalId sets before/after Argmin root-load
/// agree; §3.5's public law and three shared helpers are package-owned and
/// Transparent, all three helpers refuse selective import, and a generic
/// client consumes both argmax's new and argmin's existing public signatures.
/// CLAIMED: the shared private comparison helper adds no trusted identity,
/// leaves the argmin public boundary intact, and exposes the §3.5 law.
/// THE GAP: inherited provider assumptions persist; no closed seed alone
/// establishes a universally quantified law or a full module export census.
#[test]
fn argmax_cost_order_law_is_generic_checked_private_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Function.Combinators",
        "Core.Logic.Or",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Collections.List",
        "Data.Collections.Derived",
        "Data.Collections.NonEmpty",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} root-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("§3.5 law and generalized private helpers kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§3.5 Argmax trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§3.5 adds no trusted GlobalId identity");
    let law = env.globals[&format!("{PACKAGE}.argmax_cost_order_invariant")];
    assert!(owned.contains(&law), "§3.5 law belongs to Argmin");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent generic argmax and argmin clients consume public types");
    for helper in PRIVATE_HELPERS {
        let id = env.globals[&format!("{PACKAGE}.{helper}")];
        assert!(owned.contains(&id), "Argmin owns {helper}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{helper} is kernel-checked rather than an Axiom"
        );
        match loaded_argmin().elaborate_file(&format!("import {PACKAGE} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{PACKAGE}.{helper}"));
            }
            other => panic!("{helper} must stay unimportable: {other:?}"),
        }
    }
}

// Identify the full accepted seed and isolated refusal by theorem identity,
// not by the position of a fenced program in the Markdown document.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§3.5 seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("§3.5 source fence parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            );
        if found {
            assert!(
                selected.replace(source).is_none(),
                "{theorem_name} must identify one §3.5 source program"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing §3.5 theorem {theorem_name}"))
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected closed §3.5 seed proposition: {other:?}"),
    }
}

fn expr_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::ECon(name, _) | SourceExpr::EVar(name, _) => format!("name:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!("({} {})", expr_shape(function), expr_shape(argument))
        }
        other => panic!("unexpected closed §3.5 seed proof: {other:?}"),
    }
}

fn theorem_shape(source: &str, name: &str) -> (String, String) {
    let found = parse_decls(source)
        .expect("§3.5 seed obligation parses")
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
        panic!("exactly one theorem {name} required");
    };
    one.clone()
}

const EXPECTED_ACCEPTED_CONTROLS: &str = r#"
theorem cost_zero : Equal Nat (label_cost Zero) five = Proved
theorem cost_one : Equal Nat (label_cost one) seven = Proved
theorem cost_two : Equal Nat (label_cost two) two = Proved
theorem cost_three : Equal Nat (label_cost three) seven = Proved

theorem shifted_law :
  Equal Nat (argmax Nat (comp Nat Nat Nat shifted label_cost) labelled)
    (argmax Nat label_cost labelled) =
  argmax_cost_order_invariant Nat label_cost shifted add_three_same_order labelled

theorem shifted_tie :
  Equal Nat (argmax Nat (comp Nat Nat Nat shifted label_cost) labelled) one = Proved

theorem original_tie : Equal Nat (argmax Nat label_cost labelled) one = Proved

theorem flattened_head :
  Equal Nat (argmax Nat (comp Nat Nat Nat flattened label_cost) labelled) Zero = Proved
"#;
const EXPECTED_REFUSAL: &str = r#"
theorem false_flattened :
  Equal Nat (argmax Nat (comp Nat Nat Nat flattened label_cost) labelled)
    (argmax Nat label_cost labelled) = Proved
"#;

/// MEASURED: all §3.5 accepted seed declarations kernel-check, including
/// the open order-preservation proof, exact four costs 5/7/2/7, the public
/// law application, independently checked non-head earliest-tie results 1/1,
/// and flat-head 0. The isolated 0=1 false claim is AST-pinned, well-typed
/// under a hypothetical witness, then kernel-rejected at its own `Proved`
/// after a fresh full accepted prefix. CLAIMED: preserving strict comparisons
/// retains the earlier greatest-cost tie, while flattening cannot satisfy the
/// premise. THE GAP: this one finite input does not prove the generic law or
/// classify all transforms that fail its premise.
#[test]
fn shifted_argmax_tie_checks_and_flattened_false_claim_refuses() {
    let accepted = seed_case("shifted_selects_cost_choice");
    let refused = seed_case("seed_neg");
    for (actual, expected) in [
        ("zero_cost_is_five", "cost_zero"),
        ("one_cost_is_seven", "cost_one"),
        ("two_cost_is_two", "cost_two"),
        ("three_cost_is_seven", "cost_three"),
        ("shifted_selects_cost_choice", "shifted_law"),
        ("shifted_selects_earlier_tie", "shifted_tie"),
        ("cost_selects_earlier_tie", "original_tie"),
        ("flattened_selects_head", "flattened_head"),
    ] {
        assert_eq!(
            theorem_shape(accepted, actual),
            theorem_shape(EXPECTED_ACCEPTED_CONTROLS, expected),
            "accepted control {actual} must keep its §3.5 obligation"
        );
    }
    assert_eq!(
        theorem_shape(refused, "seed_neg"),
        theorem_shape(EXPECTED_REFUSAL, "false_flattened"),
        "refusal must claim false flattened-cost equality with Proved"
    );
    let decls = parse_decls(refused).expect("§3.5 isolated refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        theorem,
        body: SourceExpr::ECon(proof, _),
        span: claim_span,
        ..
    }] = decls.as_slice()
    else {
        panic!("isolated refusal is exactly one theorem with Proved");
    };
    assert_eq!(name, "seed_neg");
    assert!(params.is_empty(), "false conclusion is closed");
    assert_eq!(proof, "Proved");
    let mut accepted_env = loaded_argmin();
    accepted_env
        .elaborate_file(accepted)
        .expect("complete §3.5 accepted seed kernel-checks");
    for name in [
        "zero_cost_is_five",
        "one_cost_is_seven",
        "two_cost_is_two",
        "three_cost_is_seven",
        "add_three_same_order",
        "shifted_selects_cost_choice",
        "shifted_selects_earlier_tie",
        "cost_selects_earlier_tie",
        "flattened_selects_head",
    ] {
        let id = accepted_env.globals[name];
        assert!(matches!(
            accepted_env.env.lookup(id),
            Some(Decl::Transparent { .. })
        ));
    }
    let claim = &refused[theorem.span().start..theorem.span().end];
    accepted_env
        .elaborate_file(&format!(
            "theorem claimed_equality_is_well_typed (h : {claim}) : {claim} = h"
        ))
        .expect("false proposition itself typechecks with an explicit witness");
    let mut refusing = loaded_argmin();
    refusing
        .elaborate_file(accepted)
        .expect("fresh full accepted prefix checks before false conclusion");
    match refusing.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start >= claim_span.start
            && span.start < span.end
            && span.end <= claim_span.end => {}
        other => panic!("flattened-cost false result must reject at its Proved: {other:?}"),
    }
}
