//! Exact rational interval hull and intersection: spec/50-stdlib/68-numerical.md §2.
//! Source: conformance/stdlib/numerical/seed-interval-hull-intersection.md.
//! Promise class: durable invariant; closed values supplement generic checked laws.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, ImportKind, KenMdFenceRole,
};
use ken_kernel::{Decl, KernelError};

const RATIONAL: &str = "Data.Numeric.Rational";
const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/numerical/seed-interval-hull-intersection.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

// IsTrue already belongs to the isolated compiler base. Remove only a
// redundant, unaliased IsTrue selective import from parsed Ken declarations;
// no seed line spelling or incidental whitespace is an oracle.
fn omit_redundant_is_true(source: &str) -> String {
    let mut edits = Vec::new();
    for declaration in parse_decls(source).expect("seed Ken parses") {
        if let SourceDecl::ImportDecl {
            module,
            kind: ImportKind::Selective(items),
            span,
        } = declaration
        {
            if module != "Core.Classes.LawfulClasses" {
                continue;
            }
            let mut keep = Vec::new();
            let mut removed = false;
            for item in items {
                if item.name == "IsTrue" && item.rename.is_none() {
                    removed = true;
                } else {
                    keep.push(match item.rename {
                        Some(alias) => format!("{} as {alias}", item.name),
                        None => item.name,
                    });
                }
            }
            if removed {
                let replacement = if keep.is_empty() {
                    String::new()
                } else {
                    format!("import {module} ({})", keep.join(", "))
                };
                edits.push((span.start..span.end, replacement));
            }
        }
    }
    let mut client = source.to_owned();
    for (range, replacement) in edits.into_iter().rev() {
        client.replace_range(range, &replacement);
    }
    client
}

fn seed_programs() -> Vec<String> {
    extract_ken_md(SEED)
        .expect("conformance Ken fences are well-formed")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| omit_redundant_is_true(&SEED[fence.body_range]))
        .collect()
}

fn loaded_interval() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("Interval §2 loads from its checked providers");
    env
}

// Public types from §2.2 and §2.3 are restated on arbitrary client inputs.
// A weakened or swapped contract cannot pass by keeping the same name.
const GENERIC_CLIENT: &str = r#"
import Data.Numeric.Rational (Rational, rational_leq, rational_min, rational_max)
import Algorithm.Numeric.RationalInterval
  (Interval, interval_contains, interval_hull, interval_intersect,
   interval_hull_contains_left, interval_hull_contains_right,
   interval_intersect_sound, interval_intersect_complete)

theorem min_leq_left (x : Rational) (y : Rational) :
  IsTrue (rational_leq (rational_min x y) x) = (proof leq_left for rational_min) x y
theorem min_leq_right (x : Rational) (y : Rational) :
  IsTrue (rational_leq (rational_min x y) y) = (proof leq_right for rational_min) x y
theorem min_greatest (x : Rational) (y : Rational) (k : Rational) :
  IsTrue (rational_leq k x) → IsTrue (rational_leq k y) →
  IsTrue (rational_leq k (rational_min x y)) =
  (proof greatest for rational_min) x y k
theorem max_left_leq (x : Rational) (y : Rational) :
  IsTrue (rational_leq x (rational_max x y)) = (proof left_leq for rational_max) x y
theorem max_right_leq (x : Rational) (y : Rational) :
  IsTrue (rational_leq y (rational_max x y)) = (proof right_leq for rational_max) x y
theorem max_least (x : Rational) (y : Rational) (k : Rational) :
  IsTrue (rational_leq x k) → IsTrue (rational_leq y k) →
  IsTrue (rational_leq (rational_max x y) k) =
  (proof least for rational_max) x y k

theorem hull_left (i : Interval) (j : Interval) (x : Rational) :
  IsTrue (interval_contains i x) → IsTrue (interval_contains (interval_hull i j) x) =
  interval_hull_contains_left i j x
theorem hull_right (i : Interval) (j : Interval) (x : Rational) :
  IsTrue (interval_contains j x) → IsTrue (interval_contains (interval_hull i j) x) =
  interval_hull_contains_right i j x
theorem intersect_sound (i : Interval) (j : Interval) (x : Rational) :
  IsTrue (interval_contains (interval_intersect i j) x) →
  And (IsTrue (interval_contains i x)) (IsTrue (interval_contains j x)) =
  interval_intersect_sound i j x
theorem intersect_complete (i : Interval) (j : Interval) (x : Rational) :
  IsTrue (interval_contains i x) → IsTrue (interval_contains j x) →
  IsTrue (interval_contains (interval_intersect i j) x) =
  interval_intersect_complete i j x
"#;

/// SOURCE: spec/50-stdlib/68-numerical.md §§2.2–2.3.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after explicit providers, both package loads preserve the exact
/// trusted GlobalId identity set. The two operations, six attached proofs,
/// and four interval laws belong to their owners with transparent bodies.
/// An independently worded client checks all ten generic public types.
/// CLAIMED: the §2 laws are checked, public and add no trust.
/// THE GAP: imported assumptions remain inherited; closed result behavior
/// needs independent value oracles, which the seed and controls test below.
#[test]
fn public_generic_min_max_and_containment_laws_add_no_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-load: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let rational_owned = env
        .elaborate_module_from_roots(&[root()], RATIONAL)
        .expect("Rational §2.3 loads from explicit providers");
    assert_eq!(
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        before,
        "Rational §2.3 must add no trusted identity"
    );
    for name in [
        "rational_min",
        "rational_max",
        "rational_min::leq_left",
        "rational_min::leq_right",
        "rational_min::greatest",
        "rational_max::left_leq",
        "rational_max::right_leq",
        "rational_max::least",
    ] {
        let id = env.globals[&format!("{RATIONAL}.{name}")];
        assert!(rational_owned.contains(&id), "{name} belongs to Rational");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must have a checked transparent body"
        );
    }
    let interval_owned = env
        .elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("Interval §2.2 checks from Rational §2.3");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "Interval §2.2 must add no trusted identity");
    for name in [
        "interval_hull",
        "interval_intersect",
        "interval_hull_contains_left",
        "interval_hull_contains_right",
        "interval_intersect_sound",
        "interval_intersect_complete",
    ] {
        let id = env.globals[&format!("{INTERVAL}.{name}")];
        assert!(interval_owned.contains(&id), "{name} belongs to Interval");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must have a checked transparent body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("ten independently restated arbitrary-argument types check");
    for private in [
        "rational_pick",
        "pick_leq_first",
        "pick_greatest",
        "pick_least",
    ] {
        match env.elaborate_file(&format!("import {RATIONAL} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{RATIONAL}.{private}"));
            }
            other => panic!("{private} must remain private: {other:?}"),
        }
    }
}

/// SOURCE: conformance/stdlib/numerical/seed-interval-hull-intersection.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the real §2 source-fence client elaborates against the shipped
/// modules, checking closed endpoints, inclusive/exclusive membership,
/// overlapping and disjoint intersections, and all four law instances.
/// Independent closed controls also check min/max operand selection under
/// equivalent but structurally distinct rational spellings.
/// CLAIMED: §2 endpoint and member results match independently stated
/// exact values; the operations and laws are available to outside clients.
/// THE GAP: closed instances cannot establish arbitrary-input containment;
/// the generic checked proof bodies and client types are tested above.
#[test]
fn closed_seed_checks_hull_and_intersection_cases() {
    let programs = seed_programs();
    let positive = programs.first().expect("seed has a positive Ken client");
    loaded_interval()
        .elaborate_file(positive)
        .expect("§2 source-shaped positive client checks");
    loaded_interval()
        .elaborate_file(CLOSED_CONTROLS)
        .expect("independent closed endpoints and tied-spelling selection check");
}

fn expect_false_theorem_rejected(env: &mut ElabEnv, negative: &str, label: &str) {
    match env.elaborate_file(negative) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) => {
            let in_theorem = parse_decls(negative)
                .expect("negative is valid Ken")
                .into_iter()
                .any(|decl| {
                    matches!(decl, SourceDecl::TheoremDecl { span: theorem, .. }
                        if span.start >= theorem.start && span.end <= theorem.end && span.start < span.end)
                });
            assert!(
                in_theorem,
                "{label}: K0004 must target its false theorem: {span:?}"
            );
        }
        other => panic!("{label}: expected kernel TypeMismatch, got {other:?}"),
    }
}

// Independent closed controls prevent a future seed edit from silently
// dropping a negative obligation; no seed fixture identifier is used here.
const CLOSED_CONTROLS: &str = r#"
import Data.Numeric.Rational (Rational, MkRational, rational_min, rational_max)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_contains, interval_hull, interval_intersect)
const witness_zero : Rational = MkRational Zero Zero
const witness_half : Rational = MkRational (Suc Zero) (Suc Zero)
const witness_one : Rational = MkRational (Suc Zero) Zero
const witness_two : Rational = MkRational (Suc (Suc Zero)) Zero
const witness_three : Rational = MkRational (Suc (Suc (Suc Zero))) Zero
const witness_four : Rational = MkRational (Suc (Suc (Suc (Suc Zero)))) Zero
const witness_two_quarters : Rational =
  MkRational (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))
theorem min_tie_uses_left_spelling :
  Equal Rational (rational_min witness_half witness_two_quarters) witness_half = Refl
theorem max_tie_uses_right_spelling :
  Equal Rational (rational_max witness_half witness_two_quarters) witness_two_quarters = Refl
theorem min_swapped_tie_uses_left_spelling :
  Equal Rational (rational_min witness_two_quarters witness_half) witness_two_quarters = Refl
theorem max_swapped_tie_uses_right_spelling :
  Equal Rational (rational_max witness_two_quarters witness_half) witness_half = Refl
theorem min_reversed_inputs :
  Equal Rational (rational_min witness_two witness_half) witness_half = Refl
theorem max_reversed_inputs :
  Equal Rational (rational_max witness_two witness_half) witness_two = Refl
const witness_low : Interval = MkInterval witness_zero witness_half
const witness_high : Interval = MkInterval witness_two witness_three
const witness_left : Interval = MkInterval witness_half witness_two
const witness_right : Interval = MkInterval witness_one witness_three
const witness_hull : Interval = interval_hull witness_low witness_high
const witness_empty : Interval = interval_intersect witness_low witness_high
const witness_overlap : Interval = interval_intersect witness_left witness_right
theorem hull_is_zero_three :
  Equal Interval witness_hull (MkInterval witness_zero witness_three) = Refl
theorem hull_three_inside :
  Equal Bool (interval_contains witness_hull witness_three) True = Proved
theorem hull_four_outside :
  Equal Bool (interval_contains witness_hull witness_four) False = Proved
theorem disjoint_is_inverted :
  Equal Interval witness_empty (MkInterval witness_two witness_half) = Refl
theorem disjoint_half_outside :
  Equal Bool (interval_contains witness_empty witness_half) False = Proved
theorem overlap_is_one_two :
  Equal Interval witness_overlap (MkInterval witness_one witness_two) = Refl
theorem overlap_one_inside :
  Equal Bool (interval_contains witness_overlap witness_one) True = Proved
theorem overlap_half_outside :
  Equal Bool (interval_contains witness_overlap witness_half) False = Proved
"#;

const FALSE_OUTSIDE_HULL: &str = r#"
theorem false_outside_hull : IsTrue (interval_contains witness_hull witness_four) = Proved
"#;
const FALSE_DISJOINT_MEMBER: &str = r#"
theorem false_disjoint_member : IsTrue (interval_contains witness_empty witness_half) = Proved
"#;
const FALSE_OVERLAP_MEMBER: &str = r#"
theorem false_overlap_member : IsTrue (interval_contains witness_overlap witness_half) = Proved
"#;

/// SOURCE: §2 seed's three independent negative Ken fences.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the full positive seed compiles before each of its separate
/// false theorem submissions. Three independent fixed client refusals also
/// run after checked positive controls with source-independent fixtures.
/// Every failure is KernelRejected(TypeMismatch) within its own theorem.
/// CLAIMED: an outside-hull point, a disjoint-intersection endpoint and a
/// left-only overlap member cannot be proved to inhabit their results.
/// THE GAP: these fixed false points do not characterize arbitrary inputs;
/// the generic laws are separately checked. QA owns AC-3's operation mutant.
#[test]
fn false_hull_and_intersection_claims_reject_at_their_own_theorems() {
    let programs = seed_programs();
    let (positive, negatives) = programs.split_first().expect("seed has a positive client");
    for negative in negatives {
        let mut env = loaded_interval();
        env.elaborate_file(positive)
            .expect("positive seed client checks before its negative");
        expect_false_theorem_rejected(&mut env, negative, "seed refusal");
    }
    for (negative, label) in [
        (FALSE_OUTSIDE_HULL, "outside hull"),
        (FALSE_DISJOINT_MEMBER, "disjoint result"),
        (FALSE_OVERLAP_MEMBER, "left-only overlap member"),
    ] {
        let mut env = loaded_interval();
        env.elaborate_file(CLOSED_CONTROLS)
            .expect("independent closed controls check before refusal");
        expect_false_theorem_rejected(&mut env, negative, label);
    }
}
