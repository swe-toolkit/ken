//! Stored interval hull and intersection idempotence: spec 68 §19.
//! Promise class: durable invariant. Equal rational values need not have the
//! same stored spelling; idempotence preserves the operand's own spelling.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

#[path = "support/checked_is_true.rs"]
mod checked_is_true;

const RATIONAL: &str = "Data.Numeric.Rational";
const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";
const HULL: &str = "interval_hull_idempotent";
const INTERSECT: &str = "interval_intersect_idempotent";
const HELPER: &str = "pick_self";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn checked_accepted() -> String {
    // Explicitly alias the catalog's IsTrue identity so it cannot collide
    // with the prelude spelling in a roots-loaded environment.
    checked_is_true::selected_source(ACCEPTED)
}

fn loaded_interval() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], RATIONAL)
        .expect("Rational root-loads with two attached self laws");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("RationalInterval root-loads with two stored laws");
    env
}

fn body_contains_const(body: &Term, owner: GlobalId) -> bool {
    let mut terms = vec![body];
    while let Some(term) = terms.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return true;
        }
        terms.extend(term.children());
    }
    false
}

// Architect's complete accepted client: two open Rational self laws, two
// open interval laws, six closed applications, both value inclusions, and
// one independent stored-spelling computation checked by Refl.
const ACCEPTED: &str = r#"import Core.Classes.LawfulClasses (IsTrue)

import Data.Numeric.Rational (Rational, MkRational, rational_min, rational_max)

import Algorithm.Numeric.RationalInterval
  (Interval,
    MkInterval,
    interval_hull,
    interval_intersect,
    interval_inside,
    interval_hull_idempotent,
    interval_intersect_idempotent)

const half : Rational = MkRational (Suc Zero) (Suc Zero)

const two_quarters : Rational = MkRational (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))

const two : Rational = MkRational (Suc (Suc Zero)) Zero

const half_two : Interval = MkInterval half two

const quarters_two : Interval = MkInterval two_quarters two

const inverted : Interval = MkInterval two half

theorem min_self_client (x : Rational) : Equal Rational (rational_min x x) x =
  (proof idempotent for rational_min) x

theorem max_self_client (x : Rational) : Equal Rational (rational_max x x) x =
  (proof idempotent for rational_max) x

theorem hull_law_client (i : Interval) : Equal Interval (interval_hull i i) i =
  interval_hull_idempotent i

theorem intersect_law_client (i : Interval) : Equal Interval (interval_intersect i i) i =
  interval_intersect_idempotent i

theorem hull_half_two : Equal Interval (interval_hull half_two half_two) half_two =
  interval_hull_idempotent half_two

theorem intersect_half_two : Equal Interval (interval_intersect half_two half_two) half_two =
  interval_intersect_idempotent half_two

theorem hull_quarters_two
    : Equal Interval (interval_hull quarters_two quarters_two) quarters_two =
  interval_hull_idempotent quarters_two

theorem intersect_quarters_two
    : Equal Interval (interval_intersect quarters_two quarters_two) quarters_two =
  interval_intersect_idempotent quarters_two

theorem hull_inverted : Equal Interval (interval_hull inverted inverted) inverted =
  interval_hull_idempotent inverted

theorem intersect_inverted : Equal Interval (interval_intersect inverted inverted) inverted =
  interval_intersect_idempotent inverted

theorem same_value_inside_forward : IsTrue (interval_inside half_two quarters_two) = Proved

theorem same_value_inside_back : IsTrue (interval_inside quarters_two half_two) = Proved

theorem hull_keeps_first_spelling
    : Equal Interval (interval_hull half_two quarters_two) half_two =
  Refl
"#;

const FALSE_EQUAL_VALUE: &str = r#"theorem hull_equal_value_claim
    : Equal Interval (interval_hull half_two quarters_two) quarters_two =
  interval_hull_idempotent quarters_two
"#;

/// SOURCE: spec/50-stdlib/68-numerical.md §19 and Architect AC-0.
/// PROMISE CLASS: durable invariant. MEASURED: before loading Rational and
/// after loading RationalInterval, exact trusted GlobalId sets are equal.
/// Both public interval laws have Transparent bodies owned by RationalInterval.
/// CLAIMED: these checked laws add no new trust and belong to their package.
/// THE GAP: inherited provider assumptions remain, and ownership alone does
/// not constrain the propositions (the open clients do that below).
#[test]
fn provider_first_laws_are_owned_checked_and_zero_new_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("provider {provider}: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    env.elaborate_module_from_roots(&[root()], RATIONAL)
        .expect("Rational with self laws roots-loads");
    let owned = env
        .elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("RationalInterval with idempotence laws roots-loads");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(
        after, before,
        "the two packages add no trusted GlobalId identity"
    );
    for name in [HULL, INTERSECT] {
        let owner = env.globals[&format!("{INTERVAL}.{name}")];
        assert!(owned.contains(&owner), "{name} belongs to RationalInterval");
        assert!(
            matches!(env.env.lookup(owner), Some(Decl::Transparent { .. })),
            "{name} has a checked body"
        );
    }
    env.elaborate_file(&checked_accepted())
        .expect("whole accepted client checks exact open and closed law types");
    for (local, law) in [
        ("hull_law_client", HULL),
        ("intersect_law_client", INTERSECT),
    ] {
        let owner = env.globals[&format!("{INTERVAL}.{law}")];
        let (_, body) = env
            .env
            .transparent_body(env.globals[local])
            .expect("open generic application has a checked body");
        assert!(
            body_contains_const(&body, owner),
            "{local} must use the exact checked law owner Const"
        );
    }
}

/// SOURCE: spec 68 §19. PROMISE CLASS: durable invariant.
/// MEASURED: the Rational-owned Transparent helper cannot be imported
/// selectively and rejects only with its qualified UnboundName. CLAIMED:
/// the selector's proof step remains private while the two attached Rational
/// self laws are the reusable checked interface. THE GAP: no assertion here
/// enumerates other Rational-private helpers.
#[test]
fn rational_pick_self_helper_is_private() {
    let mut env = loaded_interval();
    let name = format!("{RATIONAL}.{HELPER}");
    let helper = env.globals[&name];
    assert!(matches!(
        env.env.lookup(helper),
        Some(Decl::Transparent { .. })
    ));
    match env.elaborate_file(&format!("import {RATIONAL} ({HELPER})")) {
        Err(ElabError::UnboundName { name: rejected, .. }) if rejected == name => (),
        other => panic!("Rational pick_self must refuse selective import: {other:?}"),
    }
}

/// SOURCE: spec 68 §19, Architect's equal-value/different-spelling cases.
/// PROMISE CLASS: durable invariant with fixed normative fixture values.
/// MEASURED: the whole accepted client checks both generic Rational proofs,
/// both generic interval laws, six closed applications, two opposite
/// value-order inclusions by Proved, and the hull's first stored spelling
/// by Refl. The single false claim is appended alone after a fresh complete
/// accepted prefix. It refuses as KernelRejected TypeMismatch within its
/// own parsed theorem span, enclosing the law application in its body.
/// CLAIMED: structural interval equality preserves spelling even when
/// distinct Rational spellings have equal values. THE GAP: a closed fixture
/// does not replace the checked open law proof or official conformance.
#[test]
fn complete_client_checks_and_false_equal_value_spelling_refuses() {
    let mut positive = loaded_interval();
    positive
        .elaborate_file(&checked_accepted())
        .expect("whole accepted client checks");
    for name in [
        "min_self_client",
        "max_self_client",
        "hull_law_client",
        "intersect_law_client",
        "hull_half_two",
        "intersect_half_two",
        "hull_quarters_two",
        "intersect_quarters_two",
        "hull_inverted",
        "intersect_inverted",
        "same_value_inside_forward",
        "same_value_inside_back",
        "hull_keeps_first_spelling",
    ] {
        assert!(
            matches!(
                positive.env.lookup(positive.globals[name]),
                Some(Decl::Transparent { .. })
            ),
            "{name} must have a checked body"
        );
    }
    let parsed = parse_decls(FALSE_EQUAL_VALUE).expect("isolated refusal parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = parsed.as_slice()
    else {
        panic!("only the equal-value claim follows the accepted prefix");
    };
    assert_eq!(name, "hull_equal_value_claim");
    let mut fresh = loaded_interval();
    fresh
        .elaborate_file(&checked_accepted())
        .expect("fresh whole accepted prefix checks before the sole refusal");
    let span = match fresh
        .elaborate_file(FALSE_EQUAL_VALUE)
        .expect_err("equal-value spelling claim must refuse")
    {
        ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        } => span,
        other => panic!("only the equal-value claim may reject as TypeMismatch: {other:?}"),
    };
    assert!(
        claim.start <= span.start && span.start < span.end && span.end <= claim.end,
        "refusal must be inside hull_equal_value_claim: {span:?} vs {claim:?}"
    );
    let body_token = FALSE_EQUAL_VALUE
        .rfind("interval_hull_idempotent")
        .expect("negative theorem body uses the public law");
    assert!(
        span.start <= body_token && body_token + "interval_hull_idempotent".len() <= span.end,
        "refusal must enclose its own public law application"
    );
    assert!(
        FALSE_EQUAL_VALUE.get(span.start..span.end).is_some(),
        "error span must be on UTF-8 boundaries"
    );
}
