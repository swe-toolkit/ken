//! Greatest-cost selection: spec/50-stdlib/72-optimization.md §3.
//! Source: conformance/stdlib/optimization/seed-argmax.md.
//! Promise class: durable invariant. The seed distinguishes the earliest
//! maximum from a later equal-cost value; generic clients use all seven laws.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole, Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const ARGMIN: &str = "Algorithm.Optimization.Argmin";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/optimization/seed-argmax.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_argmin() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], ARGMIN)
        .expect("Argmin/Argmax and their provider closure root-load");
    env
}

fn seed_programs() -> Vec<&'static str> {
    extract_ken_md(SEED)
        .expect("§3 seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| &SEED[fence.body_range])
        .collect()
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Optimization.Argmin
  (argmax, argmax_member, argmax_greatest, argmax_no_costlier,
   argmax_head_on_tie, argmax_singleton, argmax_keeps_head, argmax_takes_tail)
import Core.Classes.LawfulClasses (leq_nat)
import Data.Collections.Derived (list_elem)
import Data.Collections.NonEmpty
  (NonEmpty, nonempty_cons, nonempty_singleton, nonempty_to_list)
import Data.Numeric.Nat.Order (lt_nat)

theorem client_member (a : Type) (cost : a → Nat) (xs : NonEmpty a) :
  list_elem a (argmax a cost xs) (nonempty_to_list a xs) =
  argmax_member a cost xs

theorem client_greatest (a : Type) (cost : a → Nat) (xs : NonEmpty a)
    (y : a) (member : list_elem a y (nonempty_to_list a xs)) :
  IsTrue (leq_nat (cost y) (cost (argmax a cost xs))) =
  argmax_greatest a cost xs y member

theorem client_no_costlier (a : Type) (cost : a → Nat) (xs : NonEmpty a)
    (y : a) (member : list_elem a y (nonempty_to_list a xs)) :
  Equal Bool (lt_nat (cost (argmax a cost xs)) (cost y)) False =
  argmax_no_costlier a cost xs y member

theorem client_head_on_tie (a : Type) (cost : a → Nat) (x : a) (rest : List a)
    (bound : (y : a) → list_elem a y rest → IsTrue (leq_nat (cost y) (cost x))) :
  Equal a (argmax a cost (nonempty_cons a x rest)) x =
  argmax_head_on_tie a cost x rest bound

theorem client_singleton (a : Type) (cost : a → Nat) (x : a) :
  Equal a (argmax a cost (nonempty_singleton a x)) x =
  argmax_singleton a cost x

theorem client_keeps_head (a : Type) (cost : a → Nat)
    (x : a) (y : a) (ys : List a)
    (h : Equal Bool
      (lt_nat (cost x) (cost (argmax a cost (nonempty_cons a y ys)))) False) :
  Equal a (argmax a cost (nonempty_cons a x (Cons a y ys))) x =
  argmax_keeps_head a cost x y ys h

theorem client_takes_tail (a : Type) (cost : a → Nat)
    (x : a) (y : a) (ys : List a)
    (h : Equal Bool
      (lt_nat (cost x) (cost (argmax a cost (nonempty_cons a y ys)))) True) :
  Equal a (argmax a cost (nonempty_cons a x (Cons a y ys)))
    (argmax a cost (nonempty_cons a y ys)) =
  argmax_takes_tail a cost x y ys h
"#;

/// SOURCE: spec/50-stdlib/72-optimization.md §3.2.
/// PROMISE CLASS: durable invariant.
/// MEASURED: all eight named §3 globals are transparent and Argmin-owned;
/// a generic client invokes all seven specified law types; selected private
/// proof helpers refuse selective import. Exact trusted GlobalId sets agree.
/// CLAIMED: the seven specified public laws are checked with no new trust.
/// THE GAP: this test does not enumerate the entire module export inventory;
/// the existing providers may carry inherited assumptions, and the seed
/// below observes selection on only one finite input.
#[test]
fn generic_argmax_exports_check_without_new_trust_or_public_helpers() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
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
        .elaborate_module_from_roots(&[root()], ARGMIN)
        .expect("Argmin and all checked Argmax laws root-load");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§3 trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "Argmax adds no trusted GlobalId");
    for name in [
        "argmax",
        "argmax_member",
        "argmax_greatest",
        "argmax_no_costlier",
        "argmax_head_on_tie",
        "argmax_singleton",
        "argmax_keeps_head",
        "argmax_takes_tail",
    ] {
        let id = env.globals[&format!("{ARGMIN}.{name}")];
        assert!(owned.contains(&id), "{name} must be Argmin-owned");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("generic client consumes exactly seven §3 laws");
    for private in [
        "argmax_from",
        "costlier",
        "select_from",
        "select_from_member",
        "argmax_from_greatest",
        "choose_ge_no",
        "choose_ge_yes",
    ] {
        match env.elaborate_file(&format!("import {ARGMIN} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{ARGMIN}.{private}"));
            }
            other => panic!("{private} must stay private: {other:?}"),
        }
    }
}

/// SOURCE: conformance/stdlib/optimization/seed-argmax.md positive.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the entire source-shaped positive client checks through the
/// roots-loaded package, including its earlier-tied-maximum theorem.
/// CLAIMED: on costs 1,5,1,5 the earliest maximum (three) is returned.
/// THE GAP: one closed input supplements, but cannot replace, generic laws.
#[test]
fn shipped_argmax_seed_positive_client_checks() {
    let programs = seed_programs();
    let [positive, _refusal] = programs.as_slice() else {
        panic!("§3 seed has one positive client and one isolated refusal");
    };
    let mut env = loaded_argmin();
    env.elaborate_file(positive)
        .expect("all shipped §3 positive controls check");
    let id = env.globals["argmax_earliest_tie"];
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unsupported closed Argmax proposition: {other:?}"),
    }
}

/// SOURCE: conformance/stdlib/optimization/seed-argmax.md refusal.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the full positive client checks first; the isolated refusal is
/// the parsed later-tied `four` claim, its proposition checks with a witness,
/// and its `Proved` body reaches KernelRejected(TypeMismatch) at that theorem.
/// CLAIMED: `argmax` retains the earlier `three` rather than later `four`.
/// THE GAP: a single tie cannot characterize all inputs; QA owns the
/// independent later-tie selector mutation for the general discrimination.
#[test]
fn shipped_argmax_later_tie_claim_refuses_at_its_own_equality() {
    let programs = seed_programs();
    let [positive, refused] = programs.as_slice() else {
        panic!("§3 seed has one positive client and one isolated refusal");
    };
    let expected = parse_decls(
        "theorem expected_later_tie : Equal Nat \
         (argmax Nat seed_cost tie_max) four = Proved",
    )
    .expect("independent later-tie proposition parses");
    let [SourceDecl::TheoremDecl {
        theorem: expected_goal,
        ..
    }] = expected.as_slice()
    else {
        panic!("independent expected proposition has one theorem");
    };
    let negative = parse_decls(refused).expect("refusal Ken fence parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        theorem,
        body: ken_elaborator::Expr::ECon(proof, _),
        span,
        ..
    }] = negative.as_slice()
    else {
        panic!("refusal contains exactly one attempted theorem");
    };
    assert_eq!(name, "seed_neg");
    assert!(params.is_empty(), "the refused proposition is closed");
    assert_eq!(proof, "Proved", "refusal must attempt a checked proof");
    assert_eq!(
        type_shape(theorem),
        type_shape(expected_goal),
        "refusal must claim the later maximum on the exact positive input"
    );
    let claim_span = theorem.span();
    let claimed_equality = &refused[claim_span.start..claim_span.end];
    let well_typed_claim = format!(
        "theorem refusal_claim_typechecks (h : {claimed_equality}) : {claimed_equality} = h"
    );
    let mut checked_type = loaded_argmin();
    checked_type
        .elaborate_file(positive)
        .expect("full positive client checks before the type control");
    checked_type
        .elaborate_file(&well_typed_claim)
        .expect("false proposition itself is well-typed under a witness");

    let mut env = loaded_argmin();
    env.elaborate_file(positive)
        .expect("full earliest-tie positive client checks before refusal");
    match env.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span: failure,
        }) if failure.start >= span.start
            && failure.end <= span.end
            && failure.start < failure.end => {}
        other => panic!("false later-tie theorem must kernel-reject: {other:?}"),
    }
}
