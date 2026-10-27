//! Bound stability: spec/50-stdlib/72-optimization.md §2.4.
//! SOURCE: conformance/stdlib/optimization/seed-bounded-search-bound-stability.md.
//! PROMISE CLASS: durable invariant. Selection by theorem identity keeps
//! unrelated future seed clients from changing the accepted/refused cases.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const PACKAGE: &str = "Algorithm.Optimization.BoundedSearch";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/optimization/seed-bounded-search-bound-stability.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("BoundedSearch with bound stability roots-loads and checks");
    env
}

// The accepted block contains stable_five_to_ten; the refused block contains
// seed_neg. Neither a fence's order nor the number of source fences is an oracle.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§2.4 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("seed source client parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            );
        if found {
            assert!(
                selected.replace(source).is_none(),
                "{theorem_name} must identify one seed client"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing seed client {theorem_name}"))
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Optimization.BoundedSearch (find_below, find_below_stable)
import Data.Numeric.Nat.Order (leq_nat, leq_nat_add_left_bound)
import Data.Numeric.Nat.Arithmetic (add)

theorem client_stable (p : Nat → Bool) (bound : Nat) (larger : Nat) (n : Nat) :
  Equal Bool (leq_nat bound larger) True →
  Equal (Option Nat) (find_below p bound) (Some Nat n) →
  Equal (Option Nat) (find_below p larger) (Some Nat n) =
  find_below_stable p bound larger n

theorem client_stable_add (p : Nat → Bool) (bound : Nat)
    (extra : Nat) (n : Nat) :
  Equal (Option Nat) (find_below p bound) (Some Nat n) →
  Equal (Option Nat) (find_below p (add bound extra)) (Some Nat n) =
  find_below_stable p bound (add bound extra) n
    (leq_nat_add_left_bound bound extra)
"#;

/// MEASURED: roots-loading the package after its explicit providers creates
/// one package-owned Transparent export with the exact §2.4 client type;
/// before and after exact trusted GlobalId inventories agree (114 → 114),
/// and selective import refuses the private leq helper. CLAIMED: the law
/// is public, generic, checked, and adds zero trust. THE GAP: assumptions
/// inherited from the providers are already in `before`, not removed.
#[test]
fn generic_stability_law_is_checked_owned_and_adds_no_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Conjunction",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("§2.4 proof checks against provider closure");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§2.4 trust identities: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§2.4 adds no trusted GlobalId");
    let id = env.globals[&format!("{PACKAGE}.find_below_stable")];
    assert!(owned.contains(&id), "law belongs to BoundedSearch");
    assert!(
        matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
        "law has a checked proof body"
    );
    env.elaborate_file(GENERIC_CLIENT)
        .expect("full arbitrary-argument law and order-derived client check");
    match env.elaborate_file(&format!("import {PACKAGE} (leq_suc_not_leq_same)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{PACKAGE}.leq_suc_not_leq_same"));
        }
        other => panic!("leq helper must stay private: {other:?}"),
    }
}

/// MEASURED: the entire accepted §2.4 seed client checks after root-loading,
/// including a found Some at five, the law-derived Some at ten, the true None
/// antecedent at three, and the same-premise same-proof-form Some control.
/// CLAIMED: a found least witness persists when the finite bound grows.
/// THE GAP: finite examples do not replace the generic checked law above.
#[test]
fn bound_stability_positive_seed_checks_with_same_premise_controls() {
    let accepted = seed_case("stable_five_to_ten");
    let mut env = loaded_package();
    env.elaborate_file(accepted)
        .expect("all accepted §2.4 seed theorems check");
    for name in [
        "found_below_five",
        "stable_five_to_ten",
        "no_witness_below_three",
        "none_then_found_three_to_ten",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
}

fn shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", shape(function), shape(argument))
        }
        other => panic!("unexpected §2.4 source type: {other:?}"),
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
        .expect("a closed equality has carrier and endpoints")
}

fn implication(ty: &SourceType) -> (&SourceType, &SourceType) {
    match ty {
        SourceType::TArr(premise, conclusion, _) => (premise, conclusion),
        other => panic!("§2.4 None claim must be an implication: {other:?}"),
    }
}

/// MEASURED: parsed Ken distinguishes the false None conclusion from the
/// checked same-antecedent Some conclusion on exactly `above_two`, three and
/// ten; the complete positive client checks first. The isolated false claim
/// is well typed as an assumption but its attempted `λmissing. Proved` proof
/// kernel-rejects with TypeMismatch inside `seed_neg`.
/// CLAIMED: a missing hit below a bound is not stable under extension.
/// THE GAP: the closed counterexample rejects only this over-strong None law.
#[test]
fn none_stability_claim_refuses_at_its_own_theorem() {
    let accepted = seed_case("stable_five_to_ten");
    let refused = seed_case("seed_neg");
    let positives = parse_decls(accepted).expect("accepted seed parses");
    let negatives = parse_decls(refused).expect("refused seed parses");
    let mut matching = negatives
        .iter()
        .filter(|decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == "seed_neg"));
    let Some(SourceDecl::TheoremDecl {
        params,
        theorem: claim,
        body,
        span: claim_span,
        ..
    }) = matching.next()
    else {
        panic!("refused theorem seed_neg must be present");
    };
    assert!(matching.next().is_none(), "seed_neg must be unambiguous");
    assert!(params.is_empty(), "the counterexample is closed");
    assert!(
        matches!(body, SourceExpr::ELam(binders, proof, _) if binders == &["missing"] && matches!(proof.as_ref(), SourceExpr::ECon(name, _) if name == "Proved")),
        "the refused client must attempt the same proof form as its control"
    );
    let (premise, conclusion) = implication(claim);
    let [premise_carrier, premise_call, premise_none] = equality(premise);
    let [result_carrier, result_call, result_none] = equality(conclusion);
    let [control_carrier, control_call, control_none] =
        equality(theorem(&positives, "no_witness_below_three"));
    assert_eq!(
        shape(premise),
        shape(theorem(&positives, "no_witness_below_three"))
    );
    assert_eq!(shape(premise_carrier), "(name:Option name:Nat)");
    assert_eq!(shape(premise_carrier), shape(result_carrier));
    assert_eq!(shape(premise_none), "(name:None name:Nat)");
    assert_eq!(
        shape(result_none),
        shape(premise_none),
        "false claim is None"
    );
    assert_eq!(shape(premise_call), shape(control_call));
    assert_eq!(shape(premise_carrier), shape(control_carrier));
    assert_eq!(shape(premise_none), shape(control_none));
    let [premise_pred, premise_bound] = applied(premise_call, "find_below")
        .try_into()
        .expect("premise searches a predicate below a bound");
    let [result_pred, result_bound] = applied(result_call, "find_below")
        .try_into()
        .expect("conclusion searches the same predicate at a larger bound");
    assert_eq!(shape(premise_pred), "name:above_two");
    assert_eq!(shape(result_pred), shape(premise_pred));
    assert_eq!(shape(premise_bound), "name:three");
    assert_eq!(shape(result_bound), "name:ten");
    let (same_premise, good_conclusion) =
        implication(theorem(&positives, "none_then_found_three_to_ten"));
    assert_eq!(shape(same_premise), shape(premise), "same None premise");
    let [good_carrier, good_call, good_some] = equality(good_conclusion);
    assert_eq!(shape(good_carrier), shape(result_carrier));
    assert_eq!(shape(good_call), shape(result_call), "same extended search");
    assert_eq!(shape(good_some), "((name:Some name:Nat) name:three)");
    assert_ne!(shape(good_some), shape(result_none));
    assert_eq!(
        shape(good_conclusion),
        shape(theorem(&positives, "stable_five_to_ten")),
        "the accepted bound-stability instance yields exactly the true result"
    );

    let mut env = loaded_package();
    env.elaborate_file(accepted)
        .expect("all same-premise and same-form controls check first");
    let claim_source = &refused[claim.span().start..claim.span().end];
    env.elaborate_file(&format!(
        "theorem assumption_well_typed (h : {claim_source}) : {claim_source} = h"
    ))
    .expect("the proposition is well typed independent of proof attempt");
    let mut fresh = loaded_package();
    fresh
        .elaborate_file(accepted)
        .expect("fresh full positive prefix checks before the refusal");
    match fresh.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start >= claim_span.start
            && span.end <= claim_span.end
            && span.start < span.end => {}
        other => panic!("false None-stability theorem must kernel-reject: {other:?}"),
    }
}
