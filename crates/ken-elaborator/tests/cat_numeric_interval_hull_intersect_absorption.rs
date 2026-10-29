//! Interval hull/intersection absorption: spec/50-stdlib/68-numerical.md §8.
//! Source: conformance/stdlib/numerical/seed-interval-hull-intersect-absorption.md.
//! Promise class: durable invariant. Four directions have open public clients;
//! closed non-nested overlapping intervals give a same-shape false refusal.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/numerical/seed-interval-hull-intersect-absorption.md"
));
const NEW_LAWS: [&str; 2] = [
    "interval_inside_intersect_hull_left",
    "interval_hull_intersect_inside_left",
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn interval_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("checked §8 interval package roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Numeric.RationalInterval
  (Interval, interval_inside, interval_hull, interval_intersect,
   interval_intersect_inside_left, interval_inside_intersect_hull_left,
   interval_inside_hull_left, interval_hull_intersect_inside_left)

theorem intersect_hull_into_left (i : Interval) (j : Interval) :
  IsTrue (interval_inside (interval_intersect i (interval_hull i j)) i) =
  interval_intersect_inside_left i (interval_hull i j)

theorem left_into_intersect_hull (i : Interval) (j : Interval) :
  IsTrue (interval_inside i (interval_intersect i (interval_hull i j))) =
  interval_inside_intersect_hull_left i j

theorem left_into_hull_intersect (i : Interval) (j : Interval) :
  IsTrue (interval_inside i (interval_hull i (interval_intersect i j))) =
  interval_inside_hull_left i (interval_intersect i j)

theorem hull_intersect_into_left (i : Interval) (j : Interval) :
  IsTrue (interval_inside (interval_hull i (interval_intersect i j)) i) =
  interval_hull_intersect_inside_left i j
"#;

/// MEASURED: the two new RationalInterval-owned Transparent identities and
/// all four public directions type-check under generic open intervals, the
/// exact before/after trusted GlobalId sets agree, and existing proof helpers
/// cannot be imported. CLAIMED: §8 provides mutual endpoint-value inclusion
/// without new trust or a wider helper surface. THE GAP: provider assumptions
/// are inherited; generic validity rests on checked proof bodies, not on the
/// separately exercised closed seed below.
#[test]
fn generic_absorption_directions_are_owned_checked_and_zero_trust() {
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
        .expect("two §8 absorption proofs root-load against checked providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§8 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§8 adds no trusted GlobalId identities");
    for name in NEW_LAWS {
        let id = env.globals[&format!("{INTERVAL}.{name}")];
        assert!(owned.contains(&id), "{name} belongs to RationalInterval");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a kernel-checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent open client consumes all four §8 public directions");
    for private in ["equiv_leq", "rational_distrib_r"] {
        match interval_env().elaborate_file(&format!("import {INTERVAL} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{INTERVAL}.{private}"));
            }
            other => panic!("{private} must remain unimportable: {other:?}"),
        }
    }
}

// Identify the programs by checked theorem identity, not by fence position.
fn seed_clients() -> (&'static str, &'static str) {
    let mut accepted = None;
    let mut refused = None;
    for fence in extract_ken_md(SEED)
        .expect("§8 Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let decls = parse_decls(source).expect("§8 source fence parses");
        let has = |name: &str| {
            decls.iter().any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name: found, .. } if found == name),
            )
        };
        if has("left_inside_intersect_hull_by_law") {
            for name in [
                "neither_inside_left_right",
                "neither_inside_right_left",
                "hull_endpoints",
                "intersect_endpoints",
                "left_intersect_hull_value",
                "left_inside_intersect_hull_value",
                "left_inside_hull_intersect_value",
                "hull_intersect_inside_left_value",
                "intersect_inside_left_by_law",
                "left_inside_hull_intersect_by_law",
                "hull_intersect_inside_left_by_law",
                "same_shape_hull_intersect_control",
                "moved_shared_argument_value",
            ] {
                assert!(has(name), "the complete accepted seed contains {name}");
            }
            assert!(accepted.replace(source).is_none(), "one accepted client");
        }
        if has("seed_neg") {
            assert!(refused.replace(source).is_none(), "one isolated refusal");
        }
    }
    (
        accepted.expect("complete §8 accepted client"),
        refused.expect("isolated §8 moved-shared-argument claim"),
    )
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected closed §8 proposition: {other:?}"),
    }
}

fn expr_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::ECon(name, _) | SourceExpr::EVar(name, _) => format!("name:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!("({} {})", expr_shape(function), expr_shape(argument))
        }
        other => panic!("unexpected §8 seed proof shape: {other:?}"),
    }
}

fn theorem_shape(source: &str, theorem_name: &str) -> (String, String) {
    let found = parse_decls(source)
        .expect("§8 proposition parses")
        .into_iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name,
                theorem,
                body,
                ..
            } if name == theorem_name => Some((type_shape(&theorem), expr_shape(&body))),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [one] = found.as_slice() else {
        panic!("exactly one theorem named {theorem_name}");
    };
    one.clone()
}

const EXPECTED_CLAIMS: &str = r#"
theorem expected_neither_lr : Equal Bool (interval_inside left right) False = Proved
theorem expected_neither_rl : Equal Bool (interval_inside right left) False = Proved
theorem expected_hull : Equal Interval (interval_hull left right) hull_value = Refl
theorem expected_intersection : Equal Interval (interval_intersect left hull_value) left = Refl

theorem expected_intersect_value : Equal Bool (interval_inside (interval_intersect left hull_value) left) True = Proved
theorem expected_intersect_hull_value : Equal Bool (interval_inside left (interval_intersect left hull_value)) True = Proved
theorem expected_hull_intersect_value : Equal Bool (interval_inside left (interval_hull left (interval_intersect left right))) True = Proved
theorem expected_hull_inside_value : Equal Bool (interval_inside (interval_hull left (interval_intersect left right)) left) True = Proved

theorem expected_intersect_law : IsTrue (interval_inside (interval_intersect left (interval_hull left right)) left) = interval_intersect_inside_left left (interval_hull left right)
theorem expected_intersect_hull_law : IsTrue (interval_inside left (interval_intersect left (interval_hull left right))) = interval_inside_intersect_hull_left left right
theorem expected_hull_intersect_law : IsTrue (interval_inside left (interval_hull left (interval_intersect left right))) = interval_inside_hull_left left (interval_intersect left right)
theorem expected_hull_inside_law : IsTrue (interval_inside (interval_hull left (interval_intersect left right)) left) = interval_hull_intersect_inside_left left right
theorem expected_same_shape : IsTrue (interval_inside (interval_hull (interval_intersect left right) left) left) = Proved
theorem expected_false_value : Equal Bool (interval_inside (interval_hull (interval_intersect left right) right) left) False = Proved
"#;
const EXPECTED_REFUSAL: &str = r#"
theorem expected_neg : IsTrue (interval_inside (interval_hull (interval_intersect left right) right) left) = Proved
"#;

// The controls use the accepted client's actual constants, not a duplicated
// interval fixture. They fix the distinct, overlapping endpoint pair and
// distinguish the moved shared argument from its valid orientation.
const FIXTURE_CONTROLS: &str = r#"
import Algorithm.Numeric.RationalInterval (interval_contains)

theorem left_has_zero_two :
  Equal Interval left (MkInterval (MkRational Zero Zero) (MkRational (Suc (Suc Zero)) Zero)) = Refl

theorem right_has_one_three :
  Equal Interval right (MkInterval (MkRational (Suc Zero) Zero) (MkRational (Suc (Suc (Suc Zero))) Zero)) = Refl

theorem pair_intersection :
  Equal Interval (interval_intersect left right) intersect_value = Refl

theorem shared_one_left : Equal Bool (interval_contains left one) True = Proved
theorem shared_one_right : Equal Bool (interval_contains right one) True = Proved
"#;

/// MEASURED: the whole accepted §8 program checks four applied public laws,
/// four independent computed True values, two False non-inclusions, endpoint
/// and overlap controls, and its same-shape accepted law application. The
/// isolated moved-argument proposition is pinned by its Ken AST and fails at
/// its own theorem with KernelRejected TypeMismatch on a fresh full prefix;
/// an independent False observation checks that exact expression. CLAIMED:
/// absorption is mutual endpoint-value inclusion for the common left interval,
/// not for the moved shared argument. THE GAP: closed clients illustrate but
/// do not establish the generic laws or official conformance execution.
#[test]
fn complete_absorption_seed_checks_and_moved_argument_refuses() {
    let (accepted, refused) = seed_clients();
    for (actual, expected) in [
        ("neither_inside_left_right", "expected_neither_lr"),
        ("neither_inside_right_left", "expected_neither_rl"),
        ("hull_endpoints", "expected_hull"),
        ("intersect_endpoints", "expected_intersection"),
        ("left_intersect_hull_value", "expected_intersect_value"),
        (
            "left_inside_intersect_hull_value",
            "expected_intersect_hull_value",
        ),
        (
            "left_inside_hull_intersect_value",
            "expected_hull_intersect_value",
        ),
        (
            "hull_intersect_inside_left_value",
            "expected_hull_inside_value",
        ),
        ("intersect_inside_left_by_law", "expected_intersect_law"),
        (
            "left_inside_intersect_hull_by_law",
            "expected_intersect_hull_law",
        ),
        (
            "left_inside_hull_intersect_by_law",
            "expected_hull_intersect_law",
        ),
        (
            "hull_intersect_inside_left_by_law",
            "expected_hull_inside_law",
        ),
        ("same_shape_hull_intersect_control", "expected_same_shape"),
        ("moved_shared_argument_value", "expected_false_value"),
    ] {
        assert_eq!(
            theorem_shape(accepted, actual),
            theorem_shape(EXPECTED_CLAIMS, expected),
            "accepted seed must retain the checked proposition and proof for {actual}"
        );
    }
    assert_eq!(
        theorem_shape(refused, "seed_neg"),
        theorem_shape(EXPECTED_REFUSAL, "expected_neg"),
        "refusal claims the false moved-shared-argument proposition with Proved"
    );
    let parsed = parse_decls(refused).expect("isolated §8 refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        span: claim_span,
        ..
    }] = parsed.as_slice()
    else {
        panic!("isolated refusal must contain exactly one theorem");
    };
    assert_eq!(name, "seed_neg");
    assert!(params.is_empty(), "false §8 claim is closed");
    let mut env = interval_env();
    env.elaborate_file(accepted)
        .expect("complete §8 seed checks first against shipped package exports");
    env.elaborate_file(FIXTURE_CONTROLS)
        .expect("actual seed intervals overlap and have exactly the stated endpoints");
    for name in [
        "left_inside_intersect_hull_by_law",
        "hull_intersect_inside_left_by_law",
        "same_shape_hull_intersect_control",
        "moved_shared_argument_value",
        "shared_one_left",
        "shared_one_right",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    let mut rejecting = interval_env();
    rejecting
        .elaborate_file(accepted)
        .expect("fresh complete accepted client checks before refusal");
    rejecting
        .elaborate_file(FIXTURE_CONTROLS)
        .expect("fresh fixture preserves overlapping, non-nested intervals");
    match rejecting.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim_span.start <= span.start
            && span.start < span.end
            && span.end <= claim_span.end => {}
        other => panic!("false moved-argument claim must reject inside seed_neg: {other:?}"),
    }
}
