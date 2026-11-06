//! Pointwise agreement below a finite search bound: spec/50-stdlib/72-optimization.md §2.7.
//! Source: conformance/stdlib/optimization/seed-bounded-search-agree.md.
//! Promise class: durable invariant. A generic checked law and paired finite
//! clients distinguish agreement on the search domain from a differing hit.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::{Decl, KernelError};

const PACKAGE: &str = "Algorithm.Optimization.BoundedSearch";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/optimization/seed-bounded-search-agree.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("BoundedSearch agreement law roots-loads and kernel-checks");
    env
}

// Select a Ken client by its parsed theorem identity, not by fence position.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("agreement seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("seed client parses")
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
import Algorithm.Optimization.BoundedSearch (find_below, find_below_ext)
import Data.Numeric.Nat.Order (lt_nat)

theorem client_ext (p : Nat → Bool) (q : Nat → Bool) (bound : Nat) :
  ((m : Nat) → Equal Bool (lt_nat m bound) True → Equal Bool (p m) (q m)) →
  Equal (Option Nat) (find_below p bound) (find_below q bound) =
  find_below_ext p q bound
"#;

/// SOURCE: spec/50-stdlib/72-optimization.md §2.7.
/// PROMISE CLASS: durable invariant.
/// MEASURED: explicit provider closure is loaded first; the complete trusted
/// GlobalId identity set remains unchanged after loading this package; the new
/// public law is owned and Transparent; an open selective-import client checks
/// its entire public premise and conclusion; the step helper refuses import.
/// CLAIMED: agreement strictly below the bound suffices for the same checked
/// search result without adding trust or exposing the private proof helper.
/// THE GAP: the provider assumptions are already in the before set; the finite
/// seed below cannot substitute for the universally quantified checked law.
#[test]
fn generic_agreement_law_is_owned_public_checked_and_trust_free() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in ["Core.Logic.Transport", "Data.Numeric.Nat.Order"] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("bounded-search agreement proof kernel-checks against its provider closure");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after, before, "no trusted GlobalId is added or removed");
    let id = env.globals[&format!("{PACKAGE}.find_below_ext")];
    assert!(owned.contains(&id), "public law is owned by BoundedSearch");
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    env.elaborate_file(GENERIC_CLIENT)
        .expect("open predicates and bound obey the exact public agreement type");
    match env.elaborate_file(&format!("import {PACKAGE} (find_next_ext)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{PACKAGE}.find_next_ext"));
        }
        other => panic!("step helper must stay private: {other:?}"),
    }
}

/// SOURCE: conformance/stdlib/optimization/seed-bounded-search-agree.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the complete accepted client roots-loads and checks its explicit
/// proof of agreement below three, its application of the public law, and the
/// same-form direct Proved control on the identical predicates and bound.
/// CLAIMED: predicates agreeing below the bound give the same search result.
/// THE GAP: this fixed bound does not prove the generic law above.
#[test]
fn agreement_seed_accepted_client_and_same_form_control_check() {
    let accepted = seed_case("agree_below_three_by_law");
    let mut env = loaded_package();
    env.elaborate_file(accepted)
        .expect("entire accepted agreement seed client checks");
    for name in [
        "above_two_agrees_with_never_below_three",
        "agree_below_three_by_law",
        "agree_below_three_by_proved",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
}

/// SOURCE: conformance/stdlib/optimization/seed-bounded-search-agree.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: a fresh accepted client checks first; the same-form bound-ten
/// claim then kernel-refuses with TypeMismatch within its own theorem span.
/// CLAIMED: agreement must cover every candidate below the stated bound;
/// outside-range agreement at three cannot justify an equality at bound ten.
/// THE GAP: the finite refusal does not alone establish the general law.
#[test]
fn disagreement_within_larger_bound_refuses_at_its_own_theorem() {
    let accepted = seed_case("agree_below_three_by_law");
    let refused = seed_case("reject_agree_below_ten_by_proved");
    let mut matched = parse_decls(refused)
        .expect("refused seed client parses")
        .into_iter()
        .filter(|decl| {
            matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == "reject_agree_below_ten_by_proved")
        });
    let claim = matched.next().expect("refused theorem is present");
    assert!(
        matched.next().is_none(),
        "refused theorem identity is unique"
    );
    let claim_span = claim.span();

    let mut env = loaded_package();
    env.elaborate_file(accepted)
        .expect("complete accepted client checks before isolated false claim");
    match env.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start >= claim_span.start
            && span.end <= claim_span.end
            && span.start < span.end => {}
        other => panic!("false bound-ten agreement must kernel-refuse at its theorem: {other:?}"),
    }
}
