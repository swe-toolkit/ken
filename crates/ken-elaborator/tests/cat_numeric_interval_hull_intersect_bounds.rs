//! Tightest hull/intersection endpoint bounds: spec/50-stdlib/68-numerical.md §7.
//! Source: conformance/stdlib/numerical/seed-interval-hull-intersect-bounds.md.
//! Promise class: durable invariant. Generic uses and nondegenerate closed
//! controls distinguish ordered premises from the false hull converse.

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
    "/../../conformance/stdlib/numerical/seed-interval-hull-intersect-bounds.md"
));
const BOUND_LAWS: [&str; 2] = ["interval_hull_least", "interval_intersect_greatest"];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn interval_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("§7 interval package roots-loads");
    env
}

// Select clients by checked theorem identity rather than Markdown fence count.
fn seed_clients() -> (&'static str, &'static str) {
    let mut accepted = None;
    let mut refused = None;
    for fence in extract_ken_md(SEED)
        .expect("§7 Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let decls = parse_decls(source).expect("§7 source fence parses");
        let has = |name: &str| {
            decls.iter().any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name: found, .. } if found == name),
            )
        };
        if has("hull_inside_outer_by_law") {
            for name in [
                "low_inside_outer",
                "high_inside_outer",
                "hull_endpoints",
                "hull_inside_outer_value",
                "outer_not_inside_hull",
                "narrow_inside_intersect_by_law",
                "narrow_inside_intersect_value",
                "intersect_not_inside_narrow",
                "inverted_inside_intersect_by_law",
                "inverted_hull_inside_outer_by_law",
                "same_shape_hull_control",
                "low_inside_low",
                "high_not_inside_low",
                "hull_not_inside_low",
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
        accepted.expect("complete §7 accepted client"),
        refused.expect("isolated §7 false hull claim"),
    )
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Numeric.RationalInterval
  (Interval, interval_inside, interval_hull, interval_intersect,
   interval_hull_least, interval_intersect_greatest)

theorem hull_bound_client (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside i k) → IsTrue (interval_inside j k) →
  IsTrue (interval_inside (interval_hull i j) k) =
  interval_hull_least i j k

theorem intersection_bound_client (k : Interval) (i : Interval) (j : Interval) :
  IsTrue (interval_inside k i) → IsTrue (interval_inside k j) →
  IsTrue (interval_inside k (interval_intersect i j)) =
  interval_intersect_greatest k i j
"#;

/// MEASURED: two checked, RationalInterval-owned Transparent laws, arbitrary
/// interval clients consuming both ordered implications, unchanged exact
/// trusted GlobalId sets and private proof helpers unimportable. CLAIMED:
/// §7's public generic bounds use existing rational proofs without a new
/// assumption or helper. THE GAP: the imported provider trust is inherited;
/// the finite seed below independently checks only closed instances.
#[test]
fn two_generic_bound_laws_are_owned_checked_private_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Logic.Conjunction",
        "Core.Logic.Transport",
        "Data.Numeric.Rational",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("both §7 proofs root-load against their checked providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§7 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§7 adds no trusted GlobalId identities");
    for name in BOUND_LAWS {
        let id = env.globals[&format!("{INTERVAL}.{name}")];
        assert!(owned.contains(&id), "{name} belongs to RationalInterval");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a kernel-checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent open clients consume both §7 public types");
    for private in ["equiv_leq", "rational_distrib_r"] {
        match interval_env().elaborate_file(&format!("import {INTERVAL} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{INTERVAL}.{private}"));
            }
            other => panic!("{private} must remain unimportable: {other:?}"),
        }
    }
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected closed §7 proposition: {other:?}"),
    }
}

fn expr_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::ECon(name, _) => format!("constructor:{name}"),
        SourceExpr::EVar(name, _) => format!("variable:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!("({} {})", expr_shape(function), expr_shape(argument))
        }
        other => panic!("unexpected §7 seed proof shape: {other:?}"),
    }
}

fn theorem_shape(source: &str, theorem_name: &str) -> (String, String) {
    let decls = parse_decls(source).expect("Ken §7 proposition parses");
    let found = decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name,
                theorem,
                body,
                ..
            } if name == theorem_name => Some((type_shape(theorem), expr_shape(body))),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [one] = found.as_slice() else {
        panic!("exactly one theorem named {theorem_name}");
    };
    one.clone()
}

const EXPECTED_ACCEPTED: &str = r#"
theorem accepted_hull : IsTrue (interval_inside (interval_hull low low) low) =
  interval_hull_least low low low Proved Proved
theorem accepted_value : Equal Bool (interval_inside low low) True = Proved
theorem independent_false : Equal Bool (interval_inside high low) False = Proved
theorem independent_hull_false : Equal Bool (interval_inside (interval_hull low high) low) False = Proved
"#;
const EXPECTED_REFUSED: &str = r#"
theorem refused_hull : IsTrue (interval_inside (interval_hull low high) low) = Proved
"#;

/// MEASURED: complete accepted seed checks both laws on strictly distinct
/// nondegenerate endpoints and on an inverted interval, direct True and False
/// computations, and Refl endpoint controls. The false hull proposition is
/// structurally paired with a same-form accepted law application and two
/// independently computed False observations; the isolated `Proved` fails at
/// its own theorem with KernelRejected TypeMismatch. CLAIMED: a hull of two
/// intervals is not necessarily inside either single operand. THE GAP:
/// closed examples illustrate, rather than establish, generic validity.
#[test]
fn complete_bounds_seed_checks_and_false_hull_converse_refuses() {
    let (accepted, refused) = seed_clients();
    for (actual, expected) in [
        ("same_shape_hull_control", "accepted_hull"),
        ("low_inside_low", "accepted_value"),
        ("high_not_inside_low", "independent_false"),
        ("hull_not_inside_low", "independent_hull_false"),
    ] {
        assert_eq!(
            theorem_shape(accepted, actual),
            theorem_shape(EXPECTED_ACCEPTED, expected),
            "accepted same-shape or independent computation {actual}"
        );
    }
    assert_eq!(
        theorem_shape(refused, "seed_neg"),
        theorem_shape(EXPECTED_REFUSED, "refused_hull"),
        "seed_neg must claim the false low-high hull inside low"
    );
    let decls = parse_decls(refused).expect("isolated §7 refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        span: claim_span,
        ..
    }] = decls.as_slice()
    else {
        panic!("isolated refusal must be exactly one theorem");
    };
    assert_eq!(name, "seed_neg");
    assert!(params.is_empty(), "false hull claim is closed");
    let mut env = interval_env();
    env.elaborate_file(accepted)
        .expect("complete accepted §7 seed checks first against shipped exports");
    for name in [
        "hull_inside_outer_by_law",
        "narrow_inside_intersect_by_law",
        "inverted_inside_intersect_by_law",
        "inverted_hull_inside_outer_by_law",
        "same_shape_hull_control",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    match env.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim_span.start <= span.start
            && span.start < span.end
            && span.end <= claim_span.end => {}
        other => panic!("false hull bound must reject inside seed_neg: {other:?}"),
    }
}
