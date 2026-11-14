//! Spec 68 §13: inverted intervals have no member, and ordered endpoints belong.
//! Promise class: durable invariant. Closed seed cases supplement generic laws.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr, KenMdFenceRole,
};
use ken_kernel::Decl;

#[path = "support/checked_is_true.rs"]
mod checked_is_true;

const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/numerical/seed-interval-inverted-empty.md"
));
const LAWS: [&str; 3] = [
    "interval_inverted_empty",
    "interval_ordered_contains_lo",
    "interval_ordered_contains_hi",
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn interval_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("§13 RationalInterval roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Data.Numeric.Rational (Rational, rational_leq)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_contains, interval_inverted_empty,
   interval_ordered_contains_lo, interval_ordered_contains_hi)
theorem g_empty (lo : Rational) (hi : Rational) (x : Rational)
    : Equal Bool (rational_leq lo hi) False
      → IsTrue (interval_contains (MkInterval lo hi) x)
      → Bottom =
  interval_inverted_empty lo hi x
theorem g_lo (lo : Rational) (hi : Rational)
    : IsTrue (rational_leq lo hi) → IsTrue (interval_contains (MkInterval lo hi) lo) =
  interval_ordered_contains_lo lo hi
theorem g_hi (lo : Rational) (hi : Rational)
    : IsTrue (rational_leq lo hi) → IsTrue (interval_contains (MkInterval lo hi) hi) =
  interval_ordered_contains_hi lo hi
"#;

/// MEASURED: the complete before/after trusted GlobalId sets agree after
/// provider-first package loading; three public owned Transparent laws import
/// and check at independently written open generic signatures.
/// CLAIMED: the §13 public laws are checked without new trust.
/// THE GAP: inherited provider trust persists, and generic checks alone do
/// not exercise the closed inverted and ordered boundary cases.
#[test]
fn three_generic_public_laws_are_owned_checked_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
        "Data.Numeric.Rational",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("§13 package proof bodies check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(before, after, "no trusted GlobalId identity changes");
    for law in LAWS {
        let id = env.globals[&format!("{INTERVAL}.{law}")];
        assert!(owned.contains(&id), "{law} belongs to RationalInterval");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        env.elaborate_file(&format!("import {INTERVAL} ({law} as probe_{law})"))
            .unwrap_or_else(|error| panic!("{law} selectively imports: {error:?}"));
    }
    env.elaborate_file(&checked_is_true::selected_source_with_import(GENERIC_CLIENT))
        .expect("all three independently stated generic §13 signatures check");
}

struct SeedCases<'a> {
    accepted: &'a str,
    refused: &'a str,
}

// Use parsed Ken declaration identity rather than Markdown headings or fence order.
fn seed_cases() -> SeedCases<'static> {
    let mut accepted = None;
    let mut refused = None;
    for fence in extract_ken_md(SEED)
        .expect("§13 seed extracts")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let decls = parse_decls(source).expect("§13 Ken source parses");
        if decls.iter().any(|decl| {
            matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == "inverted_zero_membership_eliminates")
        }) {
            assert!(accepted.replace(source).is_none(), "one accepted client");
        } else if decls.iter().any(|decl| {
            matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == "reject_inverted_zero_membership")
        }) {
            assert!(refused.replace(source).is_none(), "one isolated refusal");
        }
    }
    SeedCases {
        accepted: accepted.expect("missing accepted §13 client"),
        refused: refused.expect("missing isolated inverted refusal"),
    }
}

fn references(body: &Expr, name: &str) -> bool {
    match body {
        Expr::EVar(found, _) => found == name,
        Expr::EApp(function, argument, _) => {
            references(function, name) || references(argument, name)
        }
        Expr::ELam(_, inner, _) | Expr::EAsc(inner, _, _) => references(inner, name),
        _ => false,
    }
}

/// MEASURED: the whole accepted seed client type-checks, including the law
/// applied to an inverted interval and both endpoint laws on an ordered one.
/// A same-form ordered membership control checks independently with Proved.
/// CLAIMED: the closed boundary exercises each public law and the order axis.
/// THE GAP: closed rationals cannot establish the open generic law by themselves.
#[test]
fn accepted_seed_uses_all_three_laws_and_ordered_control() {
    let accepted = seed_cases().accepted;
    let decls = parse_decls(accepted).expect("accepted seed client parses");
    for (theorem, law) in [
        (
            "inverted_zero_membership_eliminates",
            "interval_inverted_empty",
        ),
        (
            "ordered_low_endpoint_is_a_member",
            "interval_ordered_contains_lo",
        ),
        (
            "ordered_high_endpoint_is_a_member",
            "interval_ordered_contains_hi",
        ),
    ] {
        assert!(
            decls.iter().any(|decl| matches!(decl,
                SourceDecl::TheoremDecl { name, body, .. }
                    if name == theorem && references(body, law))),
            "{theorem} applies {law}"
        );
    }
    assert!(decls.iter().any(|decl| matches!(decl,
        SourceDecl::TheoremDecl { name, .. }
            if name == "ordered_zero_membership_control")));
    let mut env = interval_env();
    env.elaborate_file(&checked_is_true::selected_source_with_import(accepted))
        .expect("complete accepted law client and same-form ordered control check");
    env.elaborate_file(
        "theorem inverted_decision_is_false : Equal Bool \
         (interval_contains (MkInterval rational_one rational_zero) rational_zero) False = Proved",
    )
    .expect("the inverted member decision is independently False");
}

/// MEASURED: after the complete accepted seed has checked in a fresh real
/// package environment, the isolated direct-Proved inverted member claim
/// refuses with KernelRejected. CLAIMED: reversing the endpoint order makes
/// this otherwise accepted membership claim false. THE GAP: this package
/// test is not an official conformance-runner execution.
#[test]
fn isolated_inverted_membership_refuses_after_accepted_client() {
    let cases = seed_cases();
    let mut env = interval_env();
    env.elaborate_file(&checked_is_true::selected_source_with_import(cases.accepted))
        .expect("accepted complete client checks before isolated refusal");
    match env.elaborate_file(&checked_is_true::selected_source(cases.refused)) {
        Err(ElabError::KernelRejected { .. }) => {}
        other => panic!("direct inverted membership must refuse by kernel: {other:?}"),
    }
}
