//! Endpoint inclusion order and operand swaps: spec/50-stdlib/68-numerical.md §5.
//! Source: conformance/stdlib/numerical/seed-interval-inside-order.md.
//! Promise class: durable invariant. Closed cases supplement five generic proofs.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, ImportKind,
    KenMdFenceRole, Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const INTERVAL: &str = "Algorithm.Numeric.RationalInterval";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/numerical/seed-interval-inside-order.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_interval() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("§5 RationalInterval roots-loads from checked providers");
    env
}

// IsTrue is already in the compiler base. Remove only its redundant import,
// located by its parsed import span; retain every seed theorem and other import.
fn omit_redundant_is_true(source: &str) -> String {
    let mut edits = Vec::new();
    for decl in parse_decls(source).expect("seed Ken parses") {
        if let SourceDecl::ImportDecl {
            module,
            kind: ImportKind::Selective(items),
            span,
        } = decl
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
    for (span, replacement) in edits.into_iter().rev() {
        client.replace_range(span, &replacement);
    }
    client
}

fn seed_programs() -> Vec<String> {
    extract_ken_md(SEED)
        .expect("§5 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| omit_redundant_is_true(&SEED[fence.body_range]))
        .collect()
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Numeric.RationalInterval
  (Interval, interval_add, interval_mul, interval_hull,
   interval_intersect, interval_inside, interval_inside_trans,
   interval_add_comm_inside, interval_mul_comm_inside,
   interval_hull_comm_inside, interval_intersect_comm_inside)

theorem trans (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside i j) → IsTrue (interval_inside j k) →
  IsTrue (interval_inside i k) = interval_inside_trans i j k

theorem add_swap (i : Interval) (j : Interval) :
  IsTrue (interval_inside (interval_add i j) (interval_add j i)) =
  interval_add_comm_inside i j

theorem mul_swap (i : Interval) (j : Interval) :
  IsTrue (interval_inside (interval_mul i j) (interval_mul j i)) =
  interval_mul_comm_inside i j

theorem hull_swap (i : Interval) (j : Interval) :
  IsTrue (interval_inside (interval_hull i j) (interval_hull j i)) =
  interval_hull_comm_inside i j

theorem intersect_swap (i : Interval) (j : Interval) :
  IsTrue (interval_inside (interval_intersect i j) (interval_intersect j i)) =
  interval_intersect_comm_inside i j
"#;

/// SOURCE: spec/50-stdlib/68-numerical.md §5.
/// PROMISE CLASS: durable invariant.
/// MEASURED: five public names are package-owned Transparent checked proofs,
/// an arbitrary-argument client checks each exact type, and the exact trusted
/// GlobalId set is unchanged from the already-loaded provider closure.
/// CLAIMED: the five generic §5 inclusions are checked, public, and zero-trust.
/// THE GAP: imported providers' inherited trust remains; closed endpoint
/// observations and refusals are checked in the other two tests.
#[test]
fn five_generic_laws_are_owned_transparent_and_zero_trust() {
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
        .expect("five generic §5 proof bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§5 trust identities: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§5 adds no trusted GlobalId identity");
    for name in [
        "interval_inside_trans",
        "interval_add_comm_inside",
        "interval_mul_comm_inside",
        "interval_hull_comm_inside",
        "interval_intersect_comm_inside",
    ] {
        let id = env.globals[&format!("{INTERVAL}.{name}")];
        assert!(owned.contains(&id), "{name} belongs to RationalInterval");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a checked transparent body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("five exact public §5 types work on arbitrary arguments");
}

/// SOURCE: §5 seed's complete accepted Ken client.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the seed checks a strict three-interval chain and its direct
/// conclusion, all four swaps in both argument orders, four independent
/// bidirectional endpoint-value controls, and an inverted intersection.
/// CLAIMED: these concrete public law applications agree with their independent
/// closed endpoint observations. THE GAP: finite cases do not prove genericity.
#[test]
fn shipped_positive_inside_order_seed_checks() {
    let sources = seed_programs();
    let [positive, _, _] = sources.as_slice() else {
        panic!("§5 seed needs one positive client and two isolated refusals");
    };
    loaded_interval()
        .elaborate_file(positive)
        .expect("all shipped §5 law instances and same-input controls check");
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected closed §5 proposition: {other:?}"),
    }
}

fn expression_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::ECon(name, _) => format!("constructor:{name}"),
        SourceExpr::EVar(name, _) => format!("variable:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!(
                "({} {})",
                expression_shape(function),
                expression_shape(argument)
            )
        }
        other => panic!("unexpected §5 theorem application: {other:?}"),
    }
}

fn theorem<'a>(decls: &'a [SourceDecl], name: &str) -> (&'a SourceType, &'a SourceExpr) {
    let found = decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name: declared,
                theorem,
                body,
                ..
            } if declared == name => Some((theorem, body)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [one] = found.as_slice() else {
        panic!("exactly one seed theorem {name} required");
    };
    *one
}

fn expected_theorem(source: &str) -> (String, String) {
    let decls = parse_decls(source).expect("independent §5 contract parses");
    let [SourceDecl::TheoremDecl { theorem, body, .. }] = decls.as_slice() else {
        panic!("the independent contract has one theorem");
    };
    (type_shape(theorem), expression_shape(body))
}

fn assert_paired_refusal(positive: &str, negative: &str, reversed: bool) {
    let prefix = parse_decls(positive).expect("accepted §5 seed parses");
    let (observed_false, expected_refusal, positive_control) = if reversed {
        (
            "outer_not_in_inner",
            "theorem denied : IsTrue (interval_inside outer inner) = Proved",
            "inner_in_outer_by_trans",
        )
    } else {
        (
            "outer_not_in_middle",
            "theorem denied : IsTrue (interval_inside inner middle) = interval_inside_trans inner outer middle Proved Proved",
            "inner_in_middle",
        )
    };
    let (false_goal, _) = theorem(&prefix, observed_false);
    let (expected_false, _) = expected_theorem(&format!(
        "theorem observed : Equal Bool (interval_inside outer {}) False = Proved",
        if reversed { "inner" } else { "middle" }
    ));
    assert_eq!(
        type_shape(false_goal),
        expected_false,
        "refusal must have a checked False control for its denied link"
    );
    let (true_goal, true_body) = theorem(&prefix, positive_control);
    let (expected_true, _) = expected_theorem(if reversed {
        "theorem accepted : IsTrue (interval_inside inner outer) = interval_inside_trans inner middle outer Proved Proved"
    } else {
        "theorem accepted : Equal Bool (interval_inside inner middle) True = Proved"
    });
    assert_eq!(type_shape(true_goal), expected_true);
    if reversed {
        let (_, expected_body) = expected_theorem(
            "theorem accepted : IsTrue (interval_inside inner outer) = interval_inside_trans inner middle outer Proved Proved",
        );
        assert_eq!(expression_shape(true_body), expected_body);
    }
    let negatives = parse_decls(negative).expect("isolated refusal Ken parses");
    let [SourceDecl::TheoremDecl {
        theorem: actual_goal,
        body: actual_body,
        span: theorem_span,
        ..
    }] = negatives.as_slice()
    else {
        panic!("isolated refusal has exactly one attempted proof theorem");
    };
    let (expected_goal, expected_body) = expected_theorem(expected_refusal);
    assert_eq!(type_shape(actual_goal), expected_goal);
    assert_eq!(expression_shape(actual_body), expected_body);
    assert!(theorem_span.start < theorem_span.end);
    let mut env = loaded_interval();
    env.elaborate_file(positive)
        .expect("complete same-input positive prefix checks before refusal");
    match env.elaborate_file(negative) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start >= theorem_span.start
            && span.end <= theorem_span.end
            && span.start < span.end => {}
        other => panic!("§5 false same-input claim must reject in its theorem: {other:?}"),
    }
}

/// SOURCE: §5 seed's two independently appended refusal clients.
/// PROMISE CLASS: durable invariant.
/// MEASURED: ASTs pair the reversed chain with a checked False observation
/// and positive transitivity, and the wrong-premise call with a checked False
/// second link and accepted target. Fresh accepted prefixes check first;
/// each refusal then TypeMismatch-rejects inside its own appended theorem.
/// CLAIMED: neither inversion nor exchanging transitivity's links is admitted
/// on this strict chain. THE GAP: closed refusals do not prove all triples.
#[test]
fn reversed_chain_and_wrong_premise_refuse_on_their_checked_controls() {
    let sources = seed_programs();
    let [positive, reversed, wrong_premise] = sources.as_slice() else {
        panic!("§5 seed needs one accepted and two refusal Ken fences");
    };
    assert_paired_refusal(positive, reversed, true);
    assert_paired_refusal(positive, wrong_premise, false);
}
