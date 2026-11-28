//! Numerical intervals §25: scaling both bounds and the query by a
//! positive point preserves the Boolean membership decision.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole, Span,
};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

#[path = "support/checked_is_true.rs"]
mod checked_is_true;

const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";
const RATIONAL: &str = "Data.Numeric.Rational";
const LAW: &str = "interval_mul_point_exact";
const OPEN: &str = "scaled_member_unscales";
const TWIN: &str = "zero_scaled_member_unscales";
const NECESSITY: &str = "premise_free_scaling_is_false";

fn read_seed() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../conformance/stdlib/numerical/seed-interval-mul-point-exact.md");
    std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

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
        "Core.Logic.Or",
    ] {
        env.elaborate_module_from_roots(roots, provider)
            .unwrap_or_else(|error| panic!("provider {provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    env.elaborate_module_from_roots(roots, RATIONAL)
        .expect("Rational provider checks");
    let after_rational = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(roots, INTERVAL)
        .expect("RationalInterval roots-loads with checked bodies");
    let after_interval = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "point-scale trusted GlobalIds: {} → {} → {}",
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

fn assert_law_citation(
    env: &ElabEnv,
    declared: &[GlobalId],
    name: &str,
    law: GlobalId,
    expected: bool,
) {
    let id = env.globals[name];
    assert!(declared.contains(&id), "the client declares {name}");
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) else {
        panic!("{name} must have a checked body");
    };
    assert_eq!(cites(body, law), expected, "{name}: checked law citation");
}

struct Row<'a> {
    fence: &'a str,
    declaration: Span,
    body: Span,
}

impl Row<'_> {
    fn prefix(&self) -> &str {
        &self.fence[..self.declaration.start]
    }

    fn source(&self) -> &str {
        &self.fence[self.declaration.start..self.declaration.end]
    }

    fn by(&self, proof: &str) -> String {
        let start = self.body.start - self.declaration.start;
        let end = self.body.end - self.declaration.start;
        let mut source = self.source().to_owned();
        source.replace_range(start..end, proof);
        source
    }
}

// Select authored declarations by identity, not headings or fence order.
// Standalone alternatives may reuse the open theorem's name; only its
// accepted fence also contains the checked refutation of premise-free scaling.
fn seed_row<'a>(seed: &'a str, theorem: &str, required_sibling: Option<&str>) -> Row<'a> {
    let mut row = None;
    for fence in extract_ken_md(seed).expect("§25 seed extracts").fences {
        if fence.role != KenMdFenceRole::Source {
            continue;
        }
        let source = &seed[fence.body_range];
        let decls = parse_decls(source).expect("§25 Ken source parses");
        if let Some(sibling) = required_sibling {
            if !decls
                .iter()
                .any(|d| matches!(d, SourceDecl::TheoremDecl { name, .. } if name == sibling))
            {
                continue;
            }
        }
        for decl in decls {
            if let SourceDecl::TheoremDecl {
                name, span, body, ..
            } = decl
            {
                if name == theorem {
                    assert!(row.is_none(), "duplicate eligible seed theorem {theorem}");
                    row = Some(Row {
                        fence: source,
                        declaration: span,
                        body: body.span().clone(),
                    });
                }
            }
        }
    }
    row.unwrap_or_else(|| panic!("missing authored seed theorem {theorem}"))
}

const GENERIC_CLIENT: &str = r#"
import Core.Classes.LawfulClasses (IsTrue, leq_nat)
import Data.Numeric.Rational (Rational, numerator, rational_mul)
import Algorithm.Numeric.RationalInterval
  (Interval, interval_contains, interval_mul, interval_point, interval_mul_point_exact)

theorem mul_point_exact_open (i : Interval) (t : Rational) (x : Rational)
    : IsTrue (leq_nat (Suc Zero) (numerator t))
      → Equal Bool
        (interval_contains (interval_mul i (interval_point t)) (rational_mul x t))
        (interval_contains i x) =
  interval_mul_point_exact i t x
"#;

/// SOURCE: numerical intervals §25 and Architect AC-0.
/// PROMISE CLASS: durable invariant.
/// MEASURED: exact trust identities, owned Transparent law and checked citation
/// from an open client using the positivity premise.
/// CLAIMED: the generic law is usable without adding trusted declarations.
/// THE GAP: this does not erase trust inherited from the providers.
#[test]
fn exact_trust_and_owned_open_point_scale() {
    let (mut env, law) = loaded_interval();
    let declared = env
        .elaborate_file(&checked_is_true::selected_source(GENERIC_CLIENT))
        .expect("generic positive-scale client checks");
    assert_law_citation(&env, &declared, "mul_point_exact_open", law, true);
}

/// SOURCE: §25 seed, accepted and necessity arms.
/// PROMISE CLASS: durable invariant; [1, 2], 2 and 5 are fixtures.
/// MEASURED: the open converse cites the law; the closed membership controls
/// and the checked refutation of premise-free scaling do not.
/// CLAIMED: positive scaling gives an open converse, and zero invalidates the
/// premise-free claim independently of any diagnostic.
/// THE GAP: the counterexample uses one interval; this is package checking,
/// not an official conformance-runner invocation.
#[test]
fn authored_positive_converse_and_independent_zero_counterexample_check() {
    let seed = read_seed();
    let accepted = seed_row(&seed, OPEN, Some(NECESSITY));
    let (mut env, law) = loaded_interval();
    let declared = env
        .elaborate_file(&checked_is_true::selected_source(accepted.fence))
        .expect("complete accepted §25 seed checks");
    assert_law_citation(&env, &declared, OPEN, law, true);
    for name in [
        "zero_scaled_contains_five_times_zero",
        "base_excludes_five",
        NECESSITY,
    ] {
        assert_law_citation(&env, &declared, name, law, false);
    }
}

fn fresh_prefix(accepted: &Row<'_>) -> ElabEnv {
    let (mut env, _) = loaded_interval();
    env.elaborate_file(&checked_is_true::selected_source(accepted.prefix()))
        .expect("fresh accepted seed prefix checks without the open witness");
    env
}

fn assert_own_type_mismatch(env: &mut ElabEnv, source: &str, name: &str, trial: &str) {
    let source = checked_is_true::selected_source(source);
    let decls = parse_decls(&source).expect("isolated claim parses");
    let [SourceDecl::TheoremDecl {
        name: actual,
        span: claim,
        ..
    }] = decls.as_slice()
    else {
        panic!("expected one appended theorem");
    };
    assert_eq!(actual, name);
    match env.elaborate_file(&source) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim.start <= span.start && span.start < span.end && span.end <= claim.end => {
            eprintln!("{name} ({trial}): KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("{name} ({trial}) must kernel-refuse in its own theorem: {other:?}"),
    }
}

/// SOURCE: §25 seed, open computation/enclosure controls and refused twin.
/// PROMISE CLASS: durable invariant.
/// MEASURED: in fresh checked prefixes, all three non-law closings of the
/// open row and the authored zero-scale twin refuse in their declarations.
/// CLAIMED: computation and the forward enclosure term do not close this
/// converse; the positive law cannot be used with a zero numerator.
/// THE GAP: narrowing is to these four proofs, not exact diagnostic text or
/// token location. The zero-scale falsity is checked by the necessity arm.
#[test]
fn non_law_closings_and_zero_scale_twin_refuse() {
    let seed = read_seed();
    let accepted = seed_row(&seed, OPEN, Some(NECESSITY));
    for (label, closing, enclosure) in [
        ("identity", "λmember. member", false),
        ("Proved", "λmember. Proved", false),
        (
            "forward enclosure",
            "λmember. interval_mul_encloses base (interval_point two) x two member Proved",
            true,
        ),
    ] {
        let mut env = fresh_prefix(&accepted);
        if enclosure {
            env.elaborate_file("import Algorithm.Numeric.RationalInterval (interval_mul_encloses)")
                .expect("the forward enclosure control imports its law");
        }
        assert_own_type_mismatch(&mut env, &accepted.by(closing), OPEN, label);
    }
    let twin = seed_row(&seed, TWIN, None);
    assert_own_type_mismatch(
        &mut fresh_prefix(&accepted),
        twin.source(),
        TWIN,
        "zero premise",
    );
}

/// SOURCE: Architect AC-0, private scale proof chain.
/// PROMISE CLASS: durable invariant.
/// MEASURED: selective import of each helper fails for its qualified identity.
/// CLAIMED: the scale proof stages are private to the interval package.
/// THE GAP: the separate exact export roster detects other public additions.
#[test]
fn point_scale_proof_helpers_are_private() {
    let (mut env, _) = loaded_interval();
    for helper in [
        "rational_mul_leq_scale",
        "rational_mul_leq_cancel_r",
        "leq_nat_cong_both",
        "mul_scale_shuffle",
    ] {
        let source = format!("import {INTERVAL} ({helper})");
        match env.elaborate_file(&source) {
            Err(ElabError::UnboundName { name, .. }) if name == format!("{INTERVAL}.{helper}") => {}
            other => panic!("private helper {helper} must refuse its import: {other:?}"),
        }
    }
}
