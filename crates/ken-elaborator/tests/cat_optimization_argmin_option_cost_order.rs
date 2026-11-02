//! Option selectors under strict-order-preserving cost transforms: spec 72 §§1.7, 3.7.
//! Source: conformance/stdlib/optimization/seed-argmin-option-cost-order-invariance.md.
//! Promise class: durable invariant. The generic checked laws and the two
//! same-form accepted/refused seed pairs have independent obligations.

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
    "/../../conformance/stdlib/optimization/seed-argmin-option-cost-order-invariance.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn package_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("Argmin and provider closure root-load");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Optimization.Argmin
  (argmin_option, argmax_option,
   argmin_option_cost_order_invariant, argmax_option_cost_order_invariant)
import Core.Function.Combinators (comp)
import Data.Numeric.Nat.Order (lt_nat)

theorem min_option_generic
    (a : Type) (cost : a → Nat) (f : Nat → Nat)
    (same_order : (i : Nat) → (j : Nat) → Equal Bool (lt_nat (f i) (f j)) (lt_nat i j))
    (xs : List a)
    : Equal (Option a) (argmin_option a (comp a Nat Nat f cost) xs) (argmin_option a cost xs) =
  argmin_option_cost_order_invariant a cost f same_order xs

theorem max_option_generic
    (a : Type) (cost : a → Nat) (f : Nat → Nat)
    (same_order : (i : Nat) → (j : Nat) → Equal Bool (lt_nat (f i) (f j)) (lt_nat i j))
    (xs : List a)
    : Equal (Option a) (argmax_option a (comp a Nat Nat f cost) xs) (argmax_option a cost xs) =
  argmax_option_cost_order_invariant a cost f same_order xs
"#;

/// SOURCE: spec 72 §§1.7, 3.7. PROMISE CLASS: durable invariant.
/// MEASURED: nine providers are preloaded; before/after trusted GlobalId
/// identity sets agree; both Argmin-owned declarations have Transparent bodies;
/// a selective-import client consumes their full open types on arbitrary
/// candidates, costs, transforms, proofs of order preservation, and lists.
/// CLAIMED: both generic Option laws are public checked zero-new-trust proofs.
/// THE GAP: inherited provider assumptions remain in both trusted sets; the
/// closed seed below alone cannot establish either universal proposition.
#[test]
fn both_option_cost_order_laws_are_generic_public_checked_and_zero_trust() {
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
        .expect("§§1.7/3.7 proofs root-load");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§§1.7/3.7 trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "Option laws add no trusted GlobalId");
    for law in [
        "argmin_option_cost_order_invariant",
        "argmax_option_cost_order_invariant",
    ] {
        let id = env.globals[&format!("{PACKAGE}.{law}")];
        assert!(owned.contains(&id), "Argmin owns {law}");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("both public open Option laws are consumable without extra premises");
}

// Fence selection is by a unique parsed theorem identity, not fence position
// or a substring in prose. No refusal counts unless the whole client checks.
fn seed_case(name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("Option cost-order seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("seed client parses")
            .into_iter()
            .any(|decl| matches!(decl, SourceDecl::TheoremDecl { name: actual, .. } if actual == name));
        if found {
            assert!(
                selected.replace(source).is_none(),
                "duplicate seed theorem {name}"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("seed theorem {name} missing"))
}

fn shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", shape(function), shape(argument))
        }
        other => panic!("unexpected closed Option goal: {other:?}"),
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
        matches!(head, SourceType::TCon(actual, _) | SourceType::TVar(actual, _) if actual == name),
        "expected {name}, found {head:?}"
    );
    args
}

fn theorem<'a>(decls: &'a [SourceDecl], name: &str) -> (&'a SourceType, &'a SourceExpr) {
    let matches = decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name: actual,
                theorem,
                body,
                ..
            } if actual == name => Some((theorem, body)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [one] = matches.as_slice() else {
        panic!("exactly one seed theorem {name} required");
    };
    *one
}

fn expr_applied<'a>(expr: &'a SourceExpr, name: &str) -> Vec<&'a SourceExpr> {
    let mut args = Vec::new();
    let mut head = expr;
    while let SourceExpr::EApp(function, argument, _) = head {
        args.push(argument.as_ref());
        head = function;
    }
    args.reverse();
    assert!(
        matches!(head, SourceExpr::EVar(actual, _) if actual == name),
        "expected checked law {name}, found {head:?}"
    );
    args
}

fn expr_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::EVar(name, _) | SourceExpr::ECon(name, _) => format!("name:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!("({} {})", expr_shape(function), expr_shape(argument))
        }
        other => panic!("unexpected seed law application: {other:?}"),
    }
}

fn assert_law_instance(decls: &[SourceDecl], name: &str, selector: &str, empty: bool) {
    let (_, body) = theorem(decls, name);
    let law = format!("arg{selector}_option_cost_order_invariant");
    let [candidate, cost, transform, same_order, list] = expr_applied(body, &law)
        .try_into()
        .expect("five generic law arguments");
    assert_eq!(expr_shape(candidate), "name:Nat");
    assert_eq!(expr_shape(cost), "name:label_cost");
    assert_eq!(expr_shape(transform), "name:shifted");
    assert_eq!(expr_shape(same_order), "name:add_three_same_order");
    assert_eq!(
        expr_shape(list),
        if empty {
            "(name:Nil name:Nat)"
        } else {
            "name:labelled"
        }
    );
}

fn assert_selected_output(
    decls: &[SourceDecl],
    name: &str,
    selector: &str,
    transformed: bool,
    label: &str,
) {
    let (goal, body) = theorem(decls, name);
    assert!(
        matches!(body, SourceExpr::ECon(proof, _) if proof == "Proved"),
        "{name} must directly check its claimed output"
    );
    let [carrier, selected, result] = applied(goal, "Equal")
        .try_into()
        .expect("output claim is an equality");
    assert_eq!(shape(carrier), "(name:Option name:Nat)");
    let [candidate, cost, list] = applied(selected, selector)
        .try_into()
        .expect("output claim uses its selector");
    assert_eq!(shape(candidate), "name:Nat");
    assert_eq!(shape(list), "name:labelled");
    if transformed {
        let [a, b, c, transform, original] = applied(cost, "comp")
            .try_into()
            .expect("flattened output uses the transformed cost");
        for ty in [a, b, c] {
            assert_eq!(shape(ty), "name:Nat");
        }
        assert_eq!(shape(transform), "name:flattened");
        assert_eq!(shape(original), "name:label_cost");
    } else {
        assert_eq!(shape(cost), "name:label_cost");
    }
    let [result_a, result_label] = applied(result, "Some")
        .try_into()
        .expect("nonempty selection returns Some");
    assert_eq!(shape(result_a), "name:Nat");
    assert_eq!(shape(result_label), format!("name:{label}"));
}

fn assert_control_and_refusal(control: &SourceType, refusal: &SourceType, selector: &str) {
    let [control_carrier, control_left, control_right] = applied(control, "Equal")
        .try_into()
        .expect("control is structural equality");
    let [refusal_carrier, refusal_left, refusal_right] = applied(refusal, "Equal")
        .try_into()
        .expect("refusal is structural equality");
    assert_eq!(shape(control_carrier), "(name:Option name:Nat)");
    assert_eq!(shape(refusal_carrier), shape(control_carrier));
    assert_eq!(shape(control_right), shape(refusal_right));

    let [control_a, control_cost, control_list] = applied(control_left, selector)
        .try_into()
        .expect("control selects on a list");
    let [refusal_a, refusal_cost, refusal_list] = applied(refusal_left, selector)
        .try_into()
        .expect("refusal selects on a list");
    assert_eq!(shape(control_a), "name:Nat");
    assert_eq!(shape(refusal_a), shape(control_a));
    assert_eq!(shape(control_list), "name:labelled");
    assert_eq!(shape(refusal_list), shape(control_list));
    let [control_a, control_b, control_c, control_transform, control_original] =
        applied(control_cost, "comp")
            .try_into()
            .expect("control composes a cost transform");
    let [refusal_a, refusal_b, refusal_c, refusal_transform, refusal_original] =
        applied(refusal_cost, "comp")
            .try_into()
            .expect("refusal composes a cost transform");
    for (control, refusal, expected) in [
        (control_a, refusal_a, "name:Nat"),
        (control_b, refusal_b, "name:Nat"),
        (control_c, refusal_c, "name:Nat"),
        (control_original, refusal_original, "name:label_cost"),
    ] {
        assert_eq!(shape(control), expected);
        assert_eq!(shape(refusal), expected);
    }
    assert_eq!(shape(control_transform), "name:shifted");
    assert_eq!(shape(refusal_transform), "name:flattened");
    assert_eq!(
        shape(refusal_left),
        shape(control_left).replace("name:shifted", "name:flattened"),
        "the only changed left expression is the cost transform"
    );
    let [right_a, right_cost, right_list] = applied(control_right, selector)
        .try_into()
        .expect("right selects original costs on the same list");
    assert_eq!(shape(right_a), "name:Nat");
    assert_eq!(shape(right_cost), "name:label_cost");
    assert_eq!(shape(right_list), "name:labelled");
}

/// SOURCE: the complete CV §1.7/§3.7 seed. PROMISE CLASS: durable invariant.
/// MEASURED: the full accepted seed, including both Nil cases, two tie values,
/// both flattened outputs and both Proved controls, checks before each fresh
/// refusal. Each isolated false theorem has a Proved body and KernelRejected
/// TypeMismatch within its own span. Parsed ASTs pin matching selectors,
/// carrier, list, original costs and only shifted-versus-flattened transforms.
/// CLAIMED: flattening distinct costs changes either Option selector's result.
/// THE GAP: the closed counterexamples do not replace the generic checked laws.
#[test]
fn actual_seed_accepts_and_both_flattened_claims_refuse_at_own_proved_form() {
    let accepted = seed_case("min_law_on_tie");
    let positives = parse_decls(accepted).expect("full accepted seed parses");
    for (name, selector, empty) in [
        ("min_law_on_tie", "min", false),
        ("min_law_on_nil", "min", true),
        ("max_law_on_tie", "max", false),
        ("max_law_on_nil", "max", true),
    ] {
        assert_law_instance(&positives, name, selector, empty);
    }
    let mut positive = package_env();
    positive
        .elaborate_file(accepted)
        .expect("entire accepted min/max list, Nil, outputs and controls check");
    for name in [
        "min_law_on_tie",
        "min_law_on_nil",
        "max_law_on_tie",
        "max_law_on_nil",
        "min_selects_earlier_tie",
        "max_selects_earlier_tie",
        "min_flattened_head",
        "max_flattened_head",
        "min_control",
        "max_control",
    ] {
        let id = positive.globals[name];
        assert!(matches!(
            positive.env.lookup(id),
            Some(Decl::Transparent { .. })
        ));
    }

    for (name, selector, transformed, label) in [
        ("min_selects_earlier_tie", "argmin_option", false, "one"),
        ("max_selects_earlier_tie", "argmax_option", false, "two"),
        ("min_flattened_head", "argmin_option", true, "Zero"),
        ("max_flattened_head", "argmax_option", true, "Zero"),
    ] {
        assert_selected_output(&positives, name, selector, transformed, label);
    }

    for (control_name, refusal_name, selector) in [
        ("min_control", "reject_min_flattened", "argmin_option"),
        ("max_control", "reject_max_flattened", "argmax_option"),
    ] {
        let (control, control_body) = theorem(&positives, control_name);
        assert!(
            matches!(control_body, SourceExpr::ECon(name, _) if name == "Proved"),
            "accepted {control_name} must have the same Proved proof form"
        );
        let refused = seed_case(refusal_name);
        let negatives = parse_decls(refused).expect("isolated refusal parses");
        let [SourceDecl::TheoremDecl {
            name,
            params,
            theorem,
            body,
            span: claim_span,
            ..
        }] = negatives.as_slice()
        else {
            panic!("{refusal_name} is exactly one closed theorem");
        };
        assert_eq!(name, refusal_name);
        assert!(params.is_empty());
        assert!(
            matches!(body, SourceExpr::ECon(name, _) if name == "Proved"),
            "{refusal_name} must use the checked control's Proved form"
        );

        let mut negative = package_env();
        negative
            .elaborate_file(accepted)
            .expect("fresh complete accepted client precedes each refusal");
        match negative.elaborate_file(refused) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) if span.start >= claim_span.start
                && span.start < span.end
                && span.end <= claim_span.end => {}
            other => panic!("{refusal_name} must reject at own claim: {other:?}"),
        }
        assert_control_and_refusal(control, theorem, selector);
    }
}
