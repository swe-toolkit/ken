//! Syntactically absent differentiation: spec/50-stdlib/66-symbolic-algebra.md §14.
//! Source: conformance/stdlib/symbolic-algebra/seed-derivative-absent-variable.md.
//! Promise class: durable invariant. A generic checked law and closed
//! same-form accepted/refused clients distinguish absence from independence.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, KenMdFenceRole,
};
use ken_kernel::{Decl, KernelError};

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/symbolic-algebra/seed-derivative-absent-variable.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn package_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("§14 NatPolynomial roots-loads with its checked proof");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EAdd, EMul, eval, deriv, poly_eq, poly_eq_complete, occurs, deriv_absent)

theorem scaled_absent
      (v : Nat) (k : Nat) (x : Expr) (env : Nat → Nat)
    : Equal Bool (occurs v x) False → Equal Nat (eval env (deriv v (EMul (EConst k) x))) Zero =
  λabsent. deriv_absent v (EMul (EConst k) x) env absent

theorem shifted_absent_canonical
      (v : Nat) (c : Nat) (x : Expr)
    : Equal Bool (occurs v x) False → IsTrue (poly_eq (deriv v (EAdd (EConst c) x)) (EConst Zero)) =
  λabsent.
    poly_eq_complete
      (deriv v (EAdd (EConst c) x))
      (EConst Zero)
      (λenv. deriv_absent v (EAdd (EConst c) x) env absent)
"#;

/// SOURCE: spec 66 §14. PROMISE CLASS: durable invariant.
/// MEASURED: the five named providers load first; exact before/after trusted
/// GlobalId sets agree. Both new public declarations are owner-local and
/// Transparent; selective imports consume the open law and derive its
/// canonical-equality consequence. Both Boolean helpers refuse import.
/// CLAIMED: syntactic absence gives a reusable evaluated-zero proof without
/// new trust or exporting the internal Boolean eliminations.
/// THE GAP: inherited provider trust remains. Closed examples cannot establish
/// the open theorem, and the generic checked body does not execute a seed.
#[test]
fn generic_absent_derivative_and_occurs_are_checked_public_and_zero_trust() {
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
        .expect("§14 proof and earlier laws kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§14 NatPolynomial trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§14 adds no trusted GlobalId identity");

    for name in ["occurs", "deriv_absent"] {
        let id = env.globals[&format!("{PACKAGE}.{name}")];
        assert!(owned.contains(&id), "NatPolynomial owns {name}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} is checked and transparent rather than assumed"
        );
        env.elaborate_file(&format!("import {PACKAGE} ({name} as public_{name})"))
            .expect("public declaration selectively imports");
    }
    for name in ["bool_or_false_left", "bool_or_false_right"] {
        let id = env.globals[&format!("{PACKAGE}.{name}")];
        assert!(owned.contains(&id), "NatPolynomial owns {name}");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        match env.elaborate_file(&format!("import {PACKAGE} ({name})")) {
            Err(ElabError::UnboundName { .. }) => {}
            other => panic!("Boolean eliminator {name} must remain private: {other:?}"),
        }
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("open §14 law and one-step poly_eq consequence check");
}

// Select real Ken source fences by parsed theorem identity, not fence order
// or textual section headings. Only the isolated claim is appended to a fresh
// full accepted prefix when exercising either refusal.
fn seed_case(name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§14 Ken seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("§14 seed source parses")
            .into_iter()
            .any(|decl| matches!(decl, SourceDecl::TheoremDecl { name: actual, .. } if actual == name));
        if found {
            assert!(
                selected.replace(source).is_none(),
                "{name} must identify one seed client"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("§14 seed client {name} is absent"))
}

fn succ_depth(index: &SourceExpr) -> usize {
    match index {
        SourceExpr::ECon(name, _) if name == "Zero" => 0,
        SourceExpr::EApp(function, predecessor, _) if matches!(function.as_ref(), SourceExpr::ECon(name, _) if name == "Suc") => {
            1 + succ_depth(predecessor)
        }
        other => panic!("seed index must be a closed Nat constructor: {other:?}"),
    }
}

fn application_index(body: &SourceExpr) -> usize {
    let mut arguments = Vec::new();
    let mut head = body;
    while let SourceExpr::EApp(function, argument, _) = head {
        arguments.push(argument.as_ref());
        head = function;
    }
    arguments.reverse();
    assert!(matches!(head, SourceExpr::EVar(name, _) if name == "deriv_absent"));
    let [index, expression, environment, premise] = arguments
        .as_slice()
        .try_into()
        .expect("§14 application must take exactly v, e, env, absent");
    assert!(matches!(expression, SourceExpr::EVar(name, _) if name == "e1"));
    assert!(matches!(environment, SourceExpr::EVar(name, _) if name == "env23"));
    assert!(matches!(premise, SourceExpr::ECon(name, _) if name == "Proved"));
    succ_depth(index)
}

const CLOSED_OBSERVATIONS: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial (occurs)

theorem absent_index_decides_false : Equal Bool (occurs (Suc (Suc Zero)) e1) False = Proved
theorem occurring_zero_decides_true : Equal Bool (occurs Zero e1) True = Proved
theorem occurring_one_decides_true : Equal Bool (occurs (Suc Zero) e1) True = Proved

theorem unused_derivative_is_still_zero
    : Equal Nat (eval env23 (deriv (Suc Zero) e1)) Zero = Proved

theorem used_derivative_is_one
    : Equal Nat (eval env23 (deriv Zero e1)) (Suc Zero) = Proved
"#;

/// SOURCE: CV §14 seed. PROMISE CLASS: durable invariant.
/// MEASURED: the complete actual accepted seed and independent closed
/// occurrence/derivative observations check. Each isolated same-form `Proved`
/// refusal, after a fresh accepted prefix, is KernelRejected/TypeMismatch
/// at its own declaration. The parsed clients share e1/env23 and differ in
/// their chosen index, including an occurring but semantically unused index.
/// CLAIMED: occurrence is syntactic, and a false absence premise cannot use
/// the public theorem even when the derivative happens to evaluate to zero.
/// THE GAP: these closed cases do not prove the generic §14 quantifiers or
/// make syntactic absence necessary for a zero-valued derivative.
#[test]
fn actual_seed_discriminates_absence_from_occurrence_and_independence() {
    let accepted = seed_case("absent_variable_zero");
    let positives = parse_decls(accepted).expect("actual §14 accepted Ken parses");
    let accepted_law = positives
        .iter()
        .find_map(|decl| match decl {
            SourceDecl::TheoremDecl { name, body, .. } if name == "absent_variable_zero" => {
                Some(body)
            }
            _ => None,
        })
        .expect("accepted §14 control present");
    assert_eq!(application_index(accepted_law), 2);

    let mut positive = package_env();
    positive
        .elaborate_file(accepted)
        .expect("complete actual §14 seed checks before refusals");
    let id = positive.globals["absent_variable_zero"];
    assert!(matches!(
        positive.env.lookup(id),
        Some(Decl::Transparent { .. })
    ));
    positive
        .elaborate_file(CLOSED_OBSERVATIONS)
        .expect("closed occurrence and derivative facts compute independently");

    for (name, expected_index) in [("refuse_occurring_zero", 0), ("refuse_occurring_unused", 1)] {
        let refused = seed_case(name);
        let decls = parse_decls(refused).expect("isolated §14 refusal parses");
        let [SourceDecl::TheoremDecl {
            name: actual,
            params,
            body,
            span: claim_span,
            ..
        }] = decls.as_slice()
        else {
            panic!("{name} must be exactly one closed theorem");
        };
        assert_eq!(actual, name);
        assert!(params.is_empty(), "the refused claim is closed");

        let mut negative = package_env();
        negative
            .elaborate_file(accepted)
            .expect("fresh complete seed prefix checks before the refusal");
        match negative.elaborate_file(refused) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) if span.start >= claim_span.start
                && span.start < span.end
                && span.end <= claim_span.end => {}
            other => panic!("{name} must reject at its own false premise: {other:?}"),
        }
        // Check the Ken client's theorem application after observing its
        // refusal: a true-neighbour mutation first reddens the refusal pin.
        assert_eq!(application_index(body), expected_index);
    }
}
