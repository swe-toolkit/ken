//! Numerical intervals §24: shifting a point and both bounds by one value
//! preserves the Boolean membership decision in both directions.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

#[path = "support/checked_is_true.rs"]
mod checked_is_true;

const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/numerical/seed-interval-add-point-exact.md"
));
const ACCEPTED_WITNESS: &str = "shifted_member_unshifts";
const REFUSED: [&str; 2] = ["shifted_contains_one", "shifted_contains_unshifted"];
const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";
const RATIONAL: &str = "Data.Numeric.Rational";
const LAW: &str = "interval_add_point_exact";

fn loaded_interval() -> (ElabEnv, GlobalId) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    let roots = &[root];
    let mut env = ElabEnv::new().expect("fresh compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Core.Logic.Conjunction",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(roots, provider)
            .unwrap_or_else(|error| panic!("provider {provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert!(
        !before.is_empty(),
        "the inherited provider baseline is nonempty"
    );
    env.elaborate_module_from_roots(roots, RATIONAL)
        .expect("Rational provider checks");
    let after_rational = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(roots, INTERVAL)
        .expect("RationalInterval roots-loads with checked bodies");
    let after_interval = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "point-shift trusted GlobalIds: {} → {} → {}",
        before.len(),
        after_rational.len(),
        after_interval.len()
    );
    assert_eq!(after_rational, before, "Rational adds no trust identities");
    assert_eq!(
        after_interval, after_rational,
        "RationalInterval adds no trust identities"
    );
    let law = env.globals[&format!("{INTERVAL}.{LAW}")];
    assert!(owned.contains(&law), "RationalInterval owns the law");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    (env, law)
}

fn cites(body: &Term, law: GlobalId) -> bool {
    matches!(body, Term::Const { id, .. } if *id == law)
        || body.children().into_iter().any(|child| cites(child, law))
}

struct SeedCases {
    accepted: &'static str,
    refused: [(&'static str, &'static str); REFUSED.len()],
}

// Anchor cases on parsed theorem names, not Markdown order or headings.
fn seed_cases() -> SeedCases {
    let mut cases = BTreeMap::new();
    for fence in extract_ken_md(SEED)
        .expect("§24 seed extracts")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        for decl in parse_decls(source).expect("§24 Ken source parses") {
            if let SourceDecl::TheoremDecl { name, .. } = decl {
                if name == ACCEPTED_WITNESS || REFUSED.contains(&name.as_str()) {
                    assert!(
                        cases.insert(name.clone(), source).is_none(),
                        "duplicate seed case {name}"
                    );
                }
            }
        }
    }
    let source = |name| {
        *cases
            .get(name)
            .unwrap_or_else(|| panic!("missing seed case {name}"))
    };
    SeedCases {
        accepted: source(ACCEPTED_WITNESS),
        refused: REFUSED.map(|name| (name, source(name))),
    }
}

fn check_seed_prefix(env: &mut ElabEnv, accepted: &str) -> Vec<GlobalId> {
    // Key IsTrue on the checked catalog identity, but never synthesize an
    // import: the seed is responsible for all of its dependencies.
    env.elaborate_file(&checked_is_true::selected_source(accepted))
        .expect("complete accepted §24 seed client checks")
}

const GENERIC_CLIENT: &str = r#"
import Data.Numeric.Rational (Rational, rational_add)
import Algorithm.Numeric.RationalInterval
  (Interval, interval_contains, interval_add, interval_point, interval_add_point_exact)

theorem add_point_exact_open (i : Interval) (t : Rational) (x : Rational)
    : Equal Bool
        (interval_contains (interval_add i (interval_point t)) (rational_add x t))
        (interval_contains i x) =
  interval_add_point_exact i t x
"#;

const COMPUTATION_ONLY: &str = r#"theorem shifted_excludes_one_by_computation
    : Equal Bool (interval_contains shifted (rational_add rational_zero one)) False =
  Proved
"#;

const ENCLOSURE_ONLY: &str = r#"theorem unshift_by_enclosure (x : Rational)
    : CatalogIsTrue (interval_contains shifted (rational_add x one))
      → CatalogIsTrue (interval_contains base x) =
  λmember. interval_add_encloses base (interval_point one) x one member (interval_point_contains one)
"#;

fn assert_law_citation(
    env: &ElabEnv,
    declared: &[GlobalId],
    name: &str,
    law: GlobalId,
    by_law: bool,
) {
    let id = env.globals[name];
    assert!(declared.contains(&id), "the client declares {name}");
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) else {
        panic!("{name} must have a checked body");
    };
    assert_eq!(cites(body, law), by_law, "{name}: checked law citation");
}

/// SOURCE: numerical intervals §24 and the seed's open converse witness.
/// PROMISE CLASS: durable invariant.
/// MEASURED: exact provider trust sets agree through both packages. The
/// owned Transparent proof is cited by checked generic and converse clients.
/// CLAIMED: clients can use the generic law to unshift members without new
/// trust. The closed exclusion also checks without citing the law.
/// THE GAP: the closed row does not distinguish the converse; inherited
/// provider trust remains. This package test is not a conformance-runner run.
#[test]
fn exact_trust_and_owned_open_point_shift() {
    let (mut env, law) = loaded_interval();
    let declared = check_seed_prefix(&mut env, seed_cases().accepted);
    for name in [ACCEPTED_WITNESS, "shifted_excludes_one"] {
        assert_law_citation(&env, &declared, name, law, true);
    }
    let computed = env
        .elaborate_file(COMPUTATION_ONLY)
        .expect("closed exclusion computes");
    assert_law_citation(
        &env,
        &computed,
        "shifted_excludes_one_by_computation",
        law,
        false,
    );

    let (mut generic_env, generic_law) = loaded_interval();
    let generic = generic_env
        .elaborate_file(GENERIC_CLIENT)
        .expect("generic client checks independently");
    assert_law_citation(
        &generic_env,
        &generic,
        "add_point_exact_open",
        generic_law,
        true,
    );
}

fn assert_own_type_mismatch(env: &mut ElabEnv, source: &str, name: &str) {
    let decls = parse_decls(source).expect("isolated claim parses");
    let [SourceDecl::TheoremDecl {
        name: actual,
        span: claim,
        ..
    }] = decls.as_slice()
    else {
        panic!("expected one appended theorem");
    };
    assert_eq!(actual, name);
    match env.elaborate_file(source) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim.start <= span.start && span.start < span.end && span.end <= claim.end => {
            eprintln!("{name}: KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("{name} must kernel-refuse at its own theorem: {other:?}"),
    }
}

/// SOURCE: numerical intervals §24, the two dropped-shift rows, and the
/// enclosure-only control from Architect AC-0.
/// PROMISE CLASS: durable invariant; [1, 2] and shift 1 are fixture values.
/// MEASURED: fresh accepted prefixes; both dropped-shift clients and the
/// open enclosure-only proof refuse with TypeMismatch in their own theorem.
/// CLAIMED: the law must be used at x + t, and enclosure alone does not type
/// check as this open converse.
/// THE GAP: the enclosure probe rules out one proof term, not all clients;
/// the diagnostics are declaration-level, not a pin on internal proof steps.
#[test]
fn dropped_shifts_and_enclosure_only_refuse_in_the_own_theorem() {
    let SeedCases { accepted, refused } = seed_cases();
    for (name, source) in refused {
        let (mut env, _) = loaded_interval();
        check_seed_prefix(&mut env, accepted);
        assert_own_type_mismatch(&mut env, &checked_is_true::selected_source(source), name);
    }

    let (mut env, _) = loaded_interval();
    check_seed_prefix(&mut env, accepted);
    env.elaborate_file("import Algorithm.Numeric.RationalInterval (interval_add_encloses, interval_point_contains)")
        .expect("enclosure-only control imports its own laws");
    assert_own_type_mismatch(&mut env, ENCLOSURE_ONLY, "unshift_by_enclosure");
}

/// SOURCE: numerical intervals §24, private AC-0 cancellation chain.
/// PROMISE CLASS: durable invariant.
/// MEASURED: each selective import from the checked package refuses with
/// UnboundName for the requested helper.
/// CLAIMED: the five proof helpers are not client API.
/// THE GAP: this checks the five named import attempts; the exact export
/// roster separately detects any other public addition.
#[test]
fn point_shift_proof_helpers_are_private() {
    let (mut env, _) = loaded_interval();
    for helper in [
        "rational_add_leq_shift",
        "rational_add_leq_cancel_r",
        "rational_add_sub_cancel",
        "rational_leq_add_left",
        "rational_zero_leq",
    ] {
        let source = format!("import {INTERVAL} ({helper})");
        match env.elaborate_file(&source) {
            Err(ElabError::UnboundName { name, span })
                if name == format!("{INTERVAL}.{helper}")
                    && span.start <= "import ".len()
                    && span.end >= source.len() - 1 => {}
            other => panic!("private helper {helper} must refuse its import: {other:?}"),
        }
    }
}
