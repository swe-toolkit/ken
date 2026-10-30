//! Hull/intersection regrouping: spec/50-stdlib/68-numerical.md §9.
//! Promise class: durable invariant. Generic clients use all four public laws;
//! a three-interval closed fixture includes inverted inputs and a false
//! dropped-operand claim with an independent False-value control.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";
const NEW_LAWS: [&str; 4] = [
    "interval_hull_assoc_inside",
    "interval_hull_assoc_rev_inside",
    "interval_intersect_assoc_inside",
    "interval_intersect_assoc_rev_inside",
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn interval_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("§9 interval package root-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Numeric.RationalInterval
  (Interval, interval_inside, interval_hull, interval_intersect,
   interval_hull_assoc_inside, interval_hull_assoc_rev_inside,
   interval_intersect_assoc_inside, interval_intersect_assoc_rev_inside)

theorem hull_forward (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside
    (interval_hull i (interval_hull j k))
    (interval_hull (interval_hull i j) k)) =
  interval_hull_assoc_inside i j k

theorem hull_reverse (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside
    (interval_hull (interval_hull i j) k)
    (interval_hull i (interval_hull j k))) =
  interval_hull_assoc_rev_inside i j k

theorem intersect_forward (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside
    (interval_intersect i (interval_intersect j k))
    (interval_intersect (interval_intersect i j) k)) =
  interval_intersect_assoc_inside i j k

theorem intersect_reverse (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside
    (interval_intersect (interval_intersect i j) k)
    (interval_intersect i (interval_intersect j k))) =
  interval_intersect_assoc_rev_inside i j k
"#;

/// MEASURED: four RationalInterval-owned public Transparent declarations
/// check against independent open exact-type clients; imported providers'
/// trusted GlobalId set is identical before and after loading the package,
/// and existing private helpers cannot be imported. CLAIMED: §9's four
/// generic, premise-free directions add no new local trust or helper surface.
/// THE GAP: imported trust remains inherited; the closed value examples below
/// illustrate but cannot prove the generic contracts.
#[test]
fn four_generic_regroupings_are_public_checked_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Conjunction",
        "Core.Classes.LawfulClasses",
        "Core.Logic.Transport",
        "Data.Numeric.Rational",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("§9 package root-loads against checked providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§9 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§9 adds no trusted GlobalId identities");
    for name in NEW_LAWS {
        let id = env.globals[&format!("{INTERVAL}.{name}")];
        assert!(owned.contains(&id), "{name} belongs to RationalInterval");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a kernel-checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("all four open, premise-free public types check");
    for private in ["equiv_leq", "rational_distrib_r"] {
        match interval_env().elaborate_file(&format!("import {INTERVAL} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{INTERVAL}.{private}"));
            }
            other => panic!("{private} must stay private: {other:?}"),
        }
    }
}

// Independent package-owned fixture. The three different intervals are
// i=[1/2,1], j=[1/3,2], k=[3/2,3]. This is not the CV-owned §9 seed.
const CLOSED_CLIENT: &str = r#"
import Data.Numeric.Rational (Rational, MkRational)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_inside, interval_hull, interval_intersect,
   interval_hull_assoc_inside, interval_hull_assoc_rev_inside,
   interval_intersect_assoc_inside, interval_intersect_assoc_rev_inside)

const third : Rational = MkRational (Suc Zero) (Suc (Suc Zero))
const half : Rational = MkRational (Suc Zero) (Suc Zero)
const one : Rational = MkRational (Suc Zero) Zero
const three_halves : Rational = MkRational (Suc (Suc (Suc Zero))) (Suc Zero)
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const three : Rational = MkRational (Suc (Suc (Suc Zero))) Zero

const i : Interval = MkInterval half one
const j : Interval = MkInterval third two
const k : Interval = MkInterval three_halves three
const inverted : Interval = MkInterval three half

theorem first_interval : Equal Interval i (MkInterval half one) = Refl
theorem second_interval : Equal Interval j (MkInterval third two) = Refl
theorem third_interval : Equal Interval k (MkInterval three_halves three) = Refl
theorem inverted_interval : Equal Interval inverted (MkInterval three half) = Refl

theorem hull_forward_client :
  IsTrue (interval_inside
    (interval_hull i (interval_hull j k))
    (interval_hull (interval_hull i j) k)) = interval_hull_assoc_inside i j k

theorem hull_reverse_client :
  IsTrue (interval_inside
    (interval_hull (interval_hull i j) k)
    (interval_hull i (interval_hull j k))) = interval_hull_assoc_rev_inside i j k

theorem intersect_forward_client :
  IsTrue (interval_inside
    (interval_intersect i (interval_intersect j k))
    (interval_intersect (interval_intersect i j) k)) = interval_intersect_assoc_inside i j k

theorem intersect_reverse_client :
  IsTrue (interval_inside
    (interval_intersect (interval_intersect i j) k)
    (interval_intersect i (interval_intersect j k))) = interval_intersect_assoc_rev_inside i j k

theorem hull_left_value :
  Equal Interval (interval_hull i (interval_hull j k)) (MkInterval third three) = Refl

theorem hull_right_value :
  Equal Interval (interval_hull (interval_hull i j) k) (MkInterval third three) = Refl

theorem hull_prefix_value :
  Equal Interval (interval_hull i j) (MkInterval third two) = Refl

theorem hull_forward_value :
  Equal Bool (interval_inside (interval_hull i (interval_hull j k))
    (interval_hull (interval_hull i j) k)) True = Proved

theorem hull_reverse_value :
  Equal Bool (interval_inside (interval_hull (interval_hull i j) k)
    (interval_hull i (interval_hull j k))) True = Proved

theorem intersect_forward_value :
  Equal Bool (interval_inside (interval_intersect i (interval_intersect j k))
    (interval_intersect (interval_intersect i j) k)) True = Proved

theorem intersect_reverse_value :
  Equal Bool (interval_inside (interval_intersect (interval_intersect i j) k)
    (interval_intersect i (interval_intersect j k))) True = Proved

theorem same_container_control :
  IsTrue (interval_inside (interval_hull i j) (interval_hull i j)) = Proved

theorem dropped_operand_value :
  Equal Bool
    (interval_inside (interval_hull i (interval_hull j k)) (interval_hull i j))
    False = Proved

theorem inverted_first_hull :
  IsTrue (interval_inside (interval_hull inverted (interval_hull j k))
    (interval_hull (interval_hull inverted j) k)) =
  interval_hull_assoc_inside inverted j k

theorem inverted_middle_hull_reverse :
  IsTrue (interval_inside (interval_hull (interval_hull i inverted) k)
    (interval_hull i (interval_hull inverted k))) =
  interval_hull_assoc_rev_inside i inverted k

theorem inverted_last_intersect :
  IsTrue (interval_inside (interval_intersect i (interval_intersect j inverted))
    (interval_intersect (interval_intersect i j) inverted)) =
  interval_intersect_assoc_inside i j inverted

theorem inverted_first_intersect_reverse :
  IsTrue (interval_inside (interval_intersect (interval_intersect inverted j) k)
    (interval_intersect inverted (interval_intersect j k))) =
  interval_intersect_assoc_rev_inside inverted j k

theorem inverted_first_hull_value :
  Equal Bool (interval_inside (interval_hull inverted (interval_hull j k))
    (interval_hull (interval_hull inverted j) k)) True = Proved

theorem inverted_middle_hull_reverse_value :
  Equal Bool (interval_inside (interval_hull (interval_hull i inverted) k)
    (interval_hull i (interval_hull inverted k))) True = Proved

theorem inverted_last_intersect_value :
  Equal Bool (interval_inside (interval_intersect i (interval_intersect j inverted))
    (interval_intersect (interval_intersect i j) inverted)) True = Proved

theorem inverted_first_intersect_reverse_value :
  Equal Bool (interval_inside (interval_intersect (interval_intersect inverted j) k)
    (interval_intersect inverted (interval_intersect j k))) True = Proved
"#;

const FALSE_DROPPED_OPERAND: &str = r#"
theorem seed_neg :
  IsTrue (interval_inside (interval_hull i (interval_hull j k)) (interval_hull i j)) = Proved
"#;

/// MEASURED: the checked three-interval fixture fixes all endpoints, applies
/// each law, checks independent closed True values, and puts an inverted input
/// in all three argument positions. A same-container positive and an exact
/// False-value control precede the fresh-prefix dropped-operand theorem; only
/// its own `Proved` is rejected with a kernel type mismatch. CLAIMED: neither
/// regrouping direction licenses dropping `k` from the containing hull, even
/// though both complete regroupings include each other. THE GAP: this one
/// closed witness does not replace the four generic checked laws or execution
/// of the independently owned conformance seed.
#[test]
fn closed_hull_intersect_regroupings_refuse_dropped_operand() {
    let mut env = interval_env();
    env.elaborate_file(CLOSED_CLIENT)
        .expect("four complete laws, values, three inverted positions and False control check");
    let parsed = parse_decls(FALSE_DROPPED_OPERAND).expect("isolated §9 refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        span: claim_span,
        ..
    }] = parsed.as_slice()
    else {
        panic!("one isolated refusal theorem");
    };
    assert_eq!(name, "seed_neg");
    let mut rejecting = interval_env();
    rejecting
        .elaborate_file(CLOSED_CLIENT)
        .expect("fresh full accepted fixture checks before refusal");
    match rejecting.elaborate_file(FALSE_DROPPED_OPERAND) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim_span.start <= span.start
            && span.start < span.end
            && span.end <= claim_span.end => {}
        other => panic!("false dropped-k claim must reject inside seed_neg: {other:?}"),
    }
}
