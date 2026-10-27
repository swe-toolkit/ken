//! Checked interval associativity in both grouping directions by inclusion.
//! Conformance-derived seed promise class: durable invariant. Generic law
//! clients and a closed value fixture test distinct proof obligations.

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
    "/../../conformance/stdlib/numerical/seed-interval-associativity-inside.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn interval_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("checked §6 interval package roots-loads");
    env
}

fn seed_clients() -> (&'static str, &'static str) {
    let mut accepted = None;
    let mut refused = None;
    for fence in extract_ken_md(SEED)
        .expect("§6 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let decls = parse_decls(source).expect("§6 seed source parses");
        let has = |name: &str| {
            decls.iter().any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name: found, .. } if found == name),
            )
        };
        if has("add_regroup") {
            for name in [
                "add_regroup_back",
                "mul_regroup",
                "mul_regroup_back",
                "add_regroup_direct",
                "mul_regroup_direct",
                "prefix_inside_sum",
                "sum_not_inside_prefix",
                "add_regroupings_coincide",
                "mul_regroupings_coincide",
            ] {
                assert!(has(name), "complete accepted seed must include {name}");
            }
            assert!(accepted.replace(source).is_none(), "one accepted client");
        }
        if has("seed_neg") {
            assert!(refused.replace(source).is_none(), "one isolated refusal");
        }
    }
    (
        accepted.expect("seed names all four accepted §6 laws"),
        refused.expect("seed names the isolated false inclusion"),
    )
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Numeric.RationalInterval
  (Interval, interval_add, interval_mul, interval_inside,
   interval_add_assoc_inside, interval_add_assoc_rev_inside,
   interval_mul_assoc_inside, interval_mul_assoc_rev_inside)

theorem add_forward (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside (interval_add i (interval_add j k))
    (interval_add (interval_add i j) k)) =
  interval_add_assoc_inside i j k

theorem add_reverse (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside (interval_add (interval_add i j) k)
    (interval_add i (interval_add j k))) =
  interval_add_assoc_rev_inside i j k

theorem mul_forward (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside (interval_mul i (interval_mul j k))
    (interval_mul (interval_mul i j) k)) =
  interval_mul_assoc_inside i j k

theorem mul_reverse (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside (interval_mul (interval_mul i j) k)
    (interval_mul i (interval_mul j k))) =
  interval_mul_assoc_rev_inside i j k
"#;

/// SOURCE: spec/50-stdlib/68-numerical.md §6.
/// PROMISE CLASS: durable invariant.
/// MEASURED: exact trusted GlobalId identities, four owned Transparent laws,
/// and separate open clients consuming both grouping directions by signature.
/// Private equiv_leq refuses selective import. CLAIMED: checked public laws
/// cover arbitrary intervals without new local trust. THE GAP: provider trust
/// remains inherited; these types do not compute the closed seed below.
#[test]
fn four_generic_interval_assoc_laws_are_owned_private_and_zero_trust() {
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
        .expect("four §6 proofs root-load against their checked providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§6 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§6 adds no trusted GlobalId identity");
    for name in [
        "interval_add_assoc_inside",
        "interval_add_assoc_rev_inside",
        "interval_mul_assoc_inside",
        "interval_mul_assoc_rev_inside",
    ] {
        let id = env.globals[&format!("{INTERVAL}.{name}")];
        assert!(owned.contains(&id), "{name} belongs to RationalInterval");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent generic client checks both operation regroupings");
    match env.elaborate_file(&format!("import {INTERVAL} (equiv_leq)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{INTERVAL}.equiv_leq"));
        }
        other => panic!("equiv_leq must stay private: {other:?}"),
    }
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected closed inclusion proposition: {other:?}"),
    }
}

fn expr_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::ECon(name, _) => format!("constructor:{name}"),
        SourceExpr::EVar(name, _) => format!("variable:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!("({} {})", expr_shape(function), expr_shape(argument))
        }
        other => panic!("unexpected closed inclusion proof: {other:?}"),
    }
}

fn theorem_shape(source: &str, theorem_name: &str) -> (String, String) {
    let decls = parse_decls(source).expect("Ken client parses");
    let (goal, body) = decls
        .iter()
        .find_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name,
                theorem,
                body,
                ..
            } if name == theorem_name => Some((theorem, body)),
            _ => None,
        })
        .unwrap_or_else(|| panic!("expected theorem {theorem_name}"));
    (type_shape(goal), expr_shape(body))
}

const EXPECTED_ACCEPTED: &str = r#"
theorem expected_prefix : IsTrue (interval_inside (interval_add i j) (interval_add i j)) = Proved
theorem expected_false :
  Equal Bool (interval_inside (interval_add i (interval_add j k)) (interval_add i j)) False = Proved
"#;
const EXPECTED_REFUSED: &str = r#"
theorem expected_neg :
  IsTrue (interval_inside (interval_add i (interval_add j k)) (interval_add i j)) = Proved
"#;

/// SOURCE: conformance/stdlib/numerical/seed-interval-associativity-inside.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the whole accepted client checks all four laws, two independent
/// Proved rows and two closed Refl coincidence rows; the negative proposition
/// is paired with an accepted same-form prefix and an independent False value.
/// Its own theorem yields KernelRejected TypeMismatch. CLAIMED: no grouping
/// law authorizes dropping the third interval from the outer bound. THE GAP:
/// closed examples do not prove the generic laws or official conformance.
#[test]
fn accepted_regrouping_seed_refuses_only_false_prefix_inclusion() {
    let (accepted, refused) = seed_clients();
    assert_eq!(
        theorem_shape(accepted, "prefix_inside_sum"),
        theorem_shape(EXPECTED_ACCEPTED, "expected_prefix"),
        "same-form control shares the proposed container"
    );
    assert_eq!(
        theorem_shape(accepted, "sum_not_inside_prefix"),
        theorem_shape(EXPECTED_ACCEPTED, "expected_false"),
        "independent control must observe this exact false inclusion"
    );
    assert_eq!(
        theorem_shape(refused, "seed_neg"),
        theorem_shape(EXPECTED_REFUSED, "expected_neg"),
        "refusal must assert that the three-operand sum lies in the prefix"
    );
    let decls = parse_decls(refused).expect("isolated refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        span: claim_span,
        ..
    }] = decls.as_slice()
    else {
        panic!("isolated refusal must contain only seed_neg");
    };
    assert_eq!(name, "seed_neg");
    let mut env = interval_env();
    env.elaborate_file(accepted)
        .expect("complete §6 accepted client checks against checked exports");
    match env.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim_span.start <= span.start
            && span.start < span.end
            && span.end <= claim_span.end => {}
        other => panic!("false prefix inclusion must reject at its theorem: {other:?}"),
    }
}
