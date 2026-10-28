//! Cost-order-invariant argmin: spec/50-stdlib/72-optimization.md §1.5.
//! SOURCE: conformance/stdlib/optimization/seed-argmin-cost-order-invariance.md.
//! PROMISE CLASS: durable invariant. An open generic client and checked proof
//! establish the law; the seed distinguishes the preserved non-head tie from
//! the flattened-cost head on the same input, with an isolated false claim.

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
    "/../../conformance/stdlib/optimization/seed-argmin-cost-order-invariance.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_argmin() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("Argmin and its provider closure root-load");
    env
}

// Select the complete Ken client by the identity of its contained theorem.
// An unrelated valid fence, added to the seed, must not change this choice.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§1.5 seed Ken fences extract")
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
                "{theorem_name} must identify exactly one seed client"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing seed client {theorem_name}"))
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Optimization.Argmin (argmin, argmin_cost_order_invariant)
import Core.Function.Combinators (comp)
import Data.Collections.NonEmpty (NonEmpty)
import Data.Numeric.Nat.Order (lt_nat)

theorem client_cost_order_invariant
    (a : Type) (cost : a → Nat) (f : Nat → Nat)
    (same_order : (i : Nat) → (j : Nat) → Equal Bool (lt_nat (f i) (f j)) (lt_nat i j))
    (xs : NonEmpty a) :
  Equal a (argmin a (comp a Nat Nat f cost) xs) (argmin a cost xs) =
  argmin_cost_order_invariant a cost f same_order xs
"#;

/// MEASURED: roots-loading the already checked providers and Argmin retains
/// the exact trusted GlobalId inventory; the new qualified law is owned and
/// Transparent, an arbitrary-type client consumes its exact §1.5 signature,
/// and each private helper refuses selective import in a fresh environment.
/// CLAIMED: the public law has a checked generic proof with zero added trust.
/// THE GAP: inherited provider assumptions remain in both trusted sets;
/// closed seed cases below supplement but do not prove the universal theorem.
#[test]
fn cost_order_law_is_owned_generic_checked_and_private_helpers_stay_private() {
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
        .expect("the checked §1.5 proof root-loads");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§1.5 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§1.5 adds no trusted GlobalId");
    let id = env.globals[&format!("{PACKAGE}.argmin_cost_order_invariant")];
    assert!(owned.contains(&id), "law belongs to Argmin");
    assert!(
        matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
        "law has a kernel-checked transparent body"
    );
    env.elaborate_file(GENERIC_CLIENT)
        .expect("§1.5 law is consumable for arbitrary candidate types and costs");
    for private in ["select_from_cost_order", "choose_cost_order"] {
        match loaded_argmin().elaborate_file(&format!("import {PACKAGE} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{PACKAGE}.{private}"));
            }
            other => panic!("{private} must remain private: {other:?}"),
        }
    }
}

// Parse the exact false seed proposition rather than relying on a refusal
// alone: a true `Zero` conclusion would otherwise check and lose the intended
// opposite-observable pair. Ignore source positions, layout and theorem name.
fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected closed §1.5 proposition: {other:?}"),
    }
}

/// MEASURED: the entire accepted Ken seed client checks, including four
/// fixture costs, the open order `Refl`, the public law application, both
/// independently reduced earliest-tie results, and flattened selection Zero.
/// The isolated false claim has the AST-pinned flattened-vs-original type
/// and kernel-rejects at its own proof, after the positive prefix succeeds.
/// CLAIMED: preserving every strict comparison keeps the earliest minimum;
/// a constant transform instead picks the head and cannot omit the premise.
/// THE GAP: finite cases alone cannot establish the generic law above, and a
/// constant transform is only one counterexample to premise-free invariance.
#[test]
fn shifted_cost_seed_checks_and_flattened_cost_false_claim_refuses() {
    let accepted = seed_case("shifted_selects_cost_choice");
    let refused = seed_case("seed_neg");
    let negative_decls = parse_decls(refused).expect("isolated refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        theorem,
        body: SourceExpr::ECon(proof, _),
        span,
        ..
    }] = negative_decls.as_slice()
    else {
        panic!("isolated refusal has one theorem with a direct proof attempt");
    };
    assert_eq!(name, "seed_neg");
    assert!(params.is_empty(), "the false claim must be closed");
    assert_eq!(proof, "Proved");
    let expected = parse_decls(
        "theorem expected_false : Equal Nat \
         (argmin Nat (comp Nat Nat Nat flattened label_cost) labelled) \
         (argmin Nat label_cost labelled) = Proved",
    )
    .expect("independently stated false proposition parses");
    let [SourceDecl::TheoremDecl {
        theorem: expected_goal,
        ..
    }] = expected.as_slice()
    else {
        panic!("expected false proposition has one theorem");
    };
    assert_eq!(
        type_shape(theorem),
        type_shape(expected_goal),
        "seed_neg must assert flattened selection equals the original minimum"
    );
    let mut env = loaded_argmin();
    env.elaborate_file(accepted)
        .expect("complete accepted §1.5 client and opposite results check");
    for name in [
        "zero_cost_is_five",
        "one_cost_is_two",
        "two_cost_is_seven",
        "three_cost_is_two",
        "add_three_same_order",
        "shifted_selects_cost_choice",
        "shifted_selects_earlier_tie",
        "cost_selects_earlier_tie",
        "flattened_selects_head",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    match env.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span: failure,
        }) if failure.start >= span.start
            && failure.end <= span.end
            && failure.start < failure.end => {}
        other => panic!("false flattened-cost result must kernel-reject: {other:?}"),
    }
}
