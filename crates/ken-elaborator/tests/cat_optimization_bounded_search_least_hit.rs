//! The least in-range hit is the result: spec/50-stdlib/72-optimization.md §2.6.
//! Source: conformance/stdlib/optimization/seed-bounded-search-least-hit.md.
//! Promise class: durable invariant. A generic checked law and paired finite
//! clients distinguish the true earliest hit from a later-hit false claim.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::{Decl, KernelError};

const PACKAGE: &str = "Algorithm.Optimization.BoundedSearch";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/optimization/seed-bounded-search-least-hit.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("BoundedSearch least-hit law roots-loads and kernel-checks");
    env
}

// Select programs by parsed theorem identity, not by fence position or count.
// Each selected source is a Ken client consumed by the elaborator.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("least-hit seed Ken fences extract")
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
import Algorithm.Optimization.BoundedSearch (find_below, find_below_least_hit)
import Data.Numeric.Nat.Order (lt_nat)

theorem client_least_hit (p : Nat → Bool) (bound : Nat) (n : Nat) :
  Equal Bool (lt_nat n bound) True →
  Equal Bool (p n) True →
  ((m : Nat) → Equal Bool (lt_nat m n) True → Equal Bool (p m) False) →
  Equal (Option Nat) (find_below p bound) (Some Nat n) =
  find_below_least_hit p bound n
"#;

/// SOURCE: spec/50-stdlib/72-optimization.md §2.6.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after explicit providers load, the exact trusted GlobalId set
/// stays unchanged; the new public law is package-owned and Transparent; an
/// arbitrary-argument selective-import client checks its full premise order;
/// the outcome helper refuses selective import with UnboundName.
/// CLAIMED: the least-hit law is a checked public general theorem adding no trust.
/// THE GAP: provider assumptions already exist in the before set; the finite
/// seed below distinguishes false results but does not replace this proof.
#[test]
fn generic_least_hit_law_is_owned_checked_and_trust_free() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in ["Core.Logic.Transport", "Data.Numeric.Nat.Order"] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("least-hit law kernel-checks against its provider closure");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after, before, "no trusted GlobalId is added or removed");
    let id = env.globals[&format!("{PACKAGE}.find_below_least_hit")];
    assert!(owned.contains(&id), "public law is owned by BoundedSearch");
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    env.elaborate_file(GENERIC_CLIENT)
        .expect("arbitrary predicate, bound and witness obey the full public law");
    match env.elaborate_file(&format!("import {PACKAGE} (hit_outcome_some)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{PACKAGE}.hit_outcome_some"));
        }
        other => panic!("outcome helper must stay private: {other:?}"),
    }
}

/// SOURCE: conformance/stdlib/optimization/seed-bounded-search-least-hit.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the complete seed client roots-loads and checks the explicit
/// prefix refutation, an application of the public law yielding Some three,
/// a direct same-input result control, and a same-form Some four control for
/// the distinct predicate whose first hit is four.
/// CLAIMED: an in-range hit with no earlier hit is the reported least hit.
/// THE GAP: closed predicates and finite bounds do not prove universality;
/// the generic law above supplies that checked statement.
#[test]
fn least_hit_seed_accepted_client_and_controls_check() {
    let accepted = seed_case("least_hit_by_law");
    let mut env = loaded_package();
    env.elaborate_file(accepted)
        .expect("entire accepted least-hit seed client checks");
    for name in [
        "above_two_refuted_below_three",
        "least_hit_by_law",
        "least_hit_by_proved",
        "above_three_hits_at_four",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
}

/// SOURCE: conformance/stdlib/optimization/seed-bounded-search-least-hit.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: a fresh full accepted client checks before the false later-hit
/// theorem; Proved then kernel-refuses with TypeMismatch at that theorem's
/// own span. The accepted client includes both exact-result and same-form
/// controls, so neither the import nor the Some-four proof form is the cause.
/// CLAIMED: a later hit cannot be reported instead of an earlier in-range hit.
/// THE GAP: this controls one predicate and bound, not every possible search.
#[test]
fn false_later_hit_is_refused_at_its_own_theorem() {
    let accepted = seed_case("least_hit_by_law");
    let refused = seed_case("reject_later_hit_by_proved");
    let mut matched = parse_decls(refused)
        .expect("refused seed client parses")
        .into_iter()
        .filter(|decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == "reject_later_hit_by_proved"));
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
        other => panic!("false later hit must kernel-refuse at its own theorem: {other:?}"),
    }
}
