//! Endpoint-wise rational interval inclusion: spec/50-stdlib/68-numerical.md §3.
//! Source: conformance/stdlib/numerical/seed-interval-inclusion.md.
//! Promise class: durable invariant. The closed seed supplements, rather than
//! replaces, checked universal laws on arbitrary interval arguments.

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
    "/../../conformance/stdlib/numerical/seed-interval-inclusion.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_interval() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("RationalInterval §3 roots-loads from checked providers");
    env
}

// The compiler base already binds IsTrue. Remove only its duplicate unaliased
// selective import from the parsed seed; keep every other item and spacing-
// independent source declaration unchanged.
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
        .expect("shipped interval-inclusion seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| omit_redundant_is_true(&SEED[fence.body_range]))
        .collect()
}

// The §3 types are restated by an importing client at arbitrary arguments.
// A changed premise, missing member witness, or swapped containment direction
// cannot pass simply by retaining a theorem's exported name.
const GENERIC_CLIENT: &str = r#"
import Data.Numeric.Rational (Rational)
import Algorithm.Numeric.RationalInterval
  (Interval, interval_contains, interval_add, interval_mul, interval_hull,
   interval_intersect, interval_inside, interval_inside_sound,
   interval_inside_refl, interval_add_inside, interval_mul_inside,
   interval_inside_hull_left, interval_inside_hull_right,
   interval_intersect_inside_left, interval_intersect_inside_right,
   interval_inside_complete)

theorem generic_sound (i : Interval) (k : Interval) (x : Rational) :
  IsTrue (interval_inside i k) → IsTrue (interval_contains i x) →
  IsTrue (interval_contains k x) = interval_inside_sound i k x

theorem generic_refl (i : Interval) : IsTrue (interval_inside i i) =
  interval_inside_refl i

theorem generic_add (i : Interval) (i2 : Interval) (j : Interval) (j2 : Interval) :
  IsTrue (interval_inside i i2) → IsTrue (interval_inside j j2) →
  IsTrue (interval_inside (interval_add i j) (interval_add i2 j2)) =
  interval_add_inside i i2 j j2

theorem generic_mul (i : Interval) (i2 : Interval) (j : Interval) (j2 : Interval) :
  IsTrue (interval_inside i i2) → IsTrue (interval_inside j j2) →
  IsTrue (interval_inside (interval_mul i j) (interval_mul i2 j2)) =
  interval_mul_inside i i2 j j2

theorem generic_hull_left (i : Interval) (j : Interval) :
  IsTrue (interval_inside i (interval_hull i j)) =
  interval_inside_hull_left i j

theorem generic_hull_right (i : Interval) (j : Interval) :
  IsTrue (interval_inside j (interval_hull i j)) =
  interval_inside_hull_right i j

theorem generic_intersect_left (i : Interval) (j : Interval) :
  IsTrue (interval_inside (interval_intersect i j) i) =
  interval_intersect_inside_left i j

theorem generic_intersect_right (i : Interval) (j : Interval) :
  IsTrue (interval_inside (interval_intersect i j) j) =
  interval_intersect_inside_right i j

theorem generic_witnessed_converse (i : Interval) (k : Interval) (q : Rational) :
  IsTrue (interval_contains i q) →
  ((x : Rational) → IsTrue (interval_contains i x) →
    IsTrue (interval_contains k x)) →
  IsTrue (interval_inside i k) = interval_inside_complete i k q
"#;

/// SOURCE: spec/50-stdlib/68-numerical.md §3.1–3.2.
/// PROMISE CLASS: durable invariant.
/// MEASURED: loading the published module from the explicit providers
/// preserves the exact trusted GlobalId set; ten newly exported names have
/// package-owned transparent bodies and nine generic theorem types are
/// usable by an importing client.
/// CLAIMED: the endpoint test and all nine laws are public checked terms at
/// zero additional trust, with the soundness/converse premises intact.
/// THE GAP: the imported providers' assumptions remain inherited; the
/// generic clients do not themselves test the closed endpoint truth table.
#[test]
fn generic_inclusion_exports_are_package_owned_checked_and_zero_trust() {
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
        .expect("all interval inclusion proof bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§3 trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§3 adds no trusted GlobalId identity");
    for name in [
        "interval_inside",
        "interval_inside_sound",
        "interval_inside_refl",
        "interval_add_inside",
        "interval_mul_inside",
        "interval_inside_hull_left",
        "interval_inside_hull_right",
        "interval_intersect_inside_left",
        "interval_intersect_inside_right",
        "interval_inside_complete",
    ] {
        let id = env.globals[&format!("{INTERVAL}.{name}")];
        assert!(owned.contains(&id), "{name} must belong to the package");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must contain a kernel-checked transparent body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("all nine §3 types are available at arbitrary client arguments");
}

/// SOURCE: conformance/stdlib/numerical/seed-interval-inclusion.md positives.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the shipped positive Ken source client elaborates after a
/// roots-loaded import, including soundness, both isotonicity rows, self and
/// both opposite-bound Boolean controls before any negative is attempted.
/// CLAIMED: the exact closed interval results and law applications match the
/// independently authored seed on non-negative rational endpoints.
/// THE GAP: a finite fixture cannot replace the universal checked proofs.
#[test]
fn shipped_positive_inclusion_seed_checks_from_public_package() {
    let sources = seed_programs();
    let positive = sources.first().expect("seed has a positive Ken client");
    loaded_interval()
        .elaborate_file(positive)
        .expect("all shipped positive interval inclusion cases check");
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unsupported closed inclusion proposition: {other:?}"),
    }
}

fn expr_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::ECon(name, _) => format!("constructor:{name}"),
        SourceExpr::EVar(name, _) => format!("variable:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!("({} {})", expr_shape(function), expr_shape(argument))
        }
        other => panic!("unsupported closed interval endpoint: {other:?}"),
    }
}

fn type_application<'a>(ty: &'a SourceType, name: &str) -> Option<Vec<&'a SourceType>> {
    let mut args = Vec::new();
    let mut head = ty;
    while let SourceType::TApp(function, argument, _) = head {
        args.push(argument.as_ref());
        head = function;
    }
    args.reverse();
    match head {
        SourceType::TVar(found, _) | SourceType::TCon(found, _) if found == name => Some(args),
        _ => None,
    }
}

fn expr_application<'a>(expr: &'a SourceExpr, name: &str) -> Option<Vec<&'a SourceExpr>> {
    let mut args = Vec::new();
    let mut head = expr;
    while let SourceExpr::EApp(function, argument, _) = head {
        args.push(argument.as_ref());
        head = function;
    }
    args.reverse();
    match head {
        SourceExpr::EVar(found, _) | SourceExpr::ECon(found, _) if found == name => Some(args),
        _ => None,
    }
}

// A checked `Equal Bool (interval_inside inner outer) flag` control supplies
// both the proposition and argument roles, independently of theorem names.
fn closed_inside(ty: &SourceType) -> Option<(&SourceType, &SourceType, &SourceType, bool)> {
    let equality = type_application(ty, "Equal")?;
    let [carrier, observed, value] = equality.as_slice() else {
        return None;
    };
    if !type_application(carrier, "Bool").is_some_and(|args| args.is_empty()) {
        return None;
    }
    let truth = if type_application(value, "True").is_some_and(|args| args.is_empty()) {
        true
    } else if type_application(value, "False").is_some_and(|args| args.is_empty()) {
        false
    } else {
        return None;
    };
    let inside = type_application(observed, "interval_inside")?;
    let [inner, outer] = inside.as_slice() else {
        return None;
    };
    Some((*observed, *inner, *outer, truth))
}

fn interval_bounds(decls: &[SourceDecl], term: &SourceType) -> Option<(String, String)> {
    let name = match term {
        SourceType::TVar(name, _) | SourceType::TCon(name, _) => name,
        _ => return None,
    };
    let definitions = decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::ViewDecl {
                name: declared,
                params,
                body,
                ..
            } if declared == name && params.is_empty() => Some(body),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [definition] = definitions.as_slice() else {
        return None;
    };
    let args = expr_application(definition, "MkInterval")?;
    let [low, high] = args.as_slice() else {
        return None;
    };
    Some((expr_shape(low), expr_shape(high)))
}

fn assert_refusal_matches_its_bound(positive: &str, refused: &str, lower: bool) {
    let positive_decls = parse_decls(positive).expect("positive controls parse");
    let controls = positive_decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl { theorem, .. } => closed_inside(theorem),
            _ => None,
        })
        .collect::<Vec<_>>();
    let for_bound = |lower_bound| {
        controls
            .iter()
            .filter(|(_, inner, outer, truth)| {
                if *truth {
                    return false;
                }
                let (Some((wide_low, wide_high)), Some((narrow_low, narrow_high))) = (
                    interval_bounds(&positive_decls, inner),
                    interval_bounds(&positive_decls, outer),
                ) else {
                    return false;
                };
                let one_bound_differs = if lower_bound {
                    wide_low != narrow_low && wide_high == narrow_high
                } else {
                    wide_low == narrow_low && wide_high != narrow_high
                };
                let reverse_checked =
                    controls
                        .iter()
                        .any(|(_, reverse_inner, reverse_outer, yes)| {
                            *yes && type_shape(reverse_inner) == type_shape(outer)
                                && type_shape(reverse_outer) == type_shape(inner)
                        });
                one_bound_differs && reverse_checked
            })
            .collect::<Vec<_>>()
    };
    let lower_cases = for_bound(true);
    let upper_cases = for_bound(false);
    let ([lower_case], [upper_case]) = (lower_cases.as_slice(), upper_cases.as_slice()) else {
        panic!("one checked false-and-reverse control is required for each bound");
    };
    assert_eq!(
        type_shape(lower_case.1),
        type_shape(upper_case.1),
        "both refusal controls must share the same wide interval"
    );
    let control = if lower { lower_case } else { upper_case };
    let false_inside = control.0;
    let term = &positive[false_inside.span().start..false_inside.span().end];
    let expected = format!("theorem expected_false_bound : IsTrue ({term}) = Proved");
    let expected_decls = parse_decls(&expected).expect("derived false comparison parses");
    let [SourceDecl::TheoremDecl {
        theorem: expected_goal,
        ..
    }] = expected_decls.as_slice()
    else {
        panic!("derived refusal must have one theorem");
    };
    let negative_decls = parse_decls(refused).expect("negative fence parses");
    let [SourceDecl::TheoremDecl {
        theorem: negative_goal,
        body: SourceExpr::ECon(proof, _),
        ..
    }] = negative_decls.as_slice()
    else {
        panic!("negative fence must be one attempted proof theorem");
    };
    assert_eq!(proof, "Proved", "refusal must attempt a checked proof");
    assert_eq!(
        type_shape(negative_goal),
        type_shape(expected_goal),
        "negative must assert this bound's checked false inclusion proposition"
    );
}

/// SOURCE: conformance/stdlib/numerical/seed-interval-inclusion.md refusals.
/// PROMISE CLASS: durable invariant.
/// MEASURED: all positive controls check first; each refused theorem's parsed
/// proposition equals the corresponding checked false bound observation,
/// with a checked reverse-true control and shared wide interval. Only a
/// TypeMismatch within that theorem is accepted for each independent bound.
/// CLAIMED: inclusion rejects a wide interval whose lower or upper endpoint
/// exceeds the containing target, one violated comparison at a time.
/// THE GAP: these two fixed endpoint fixtures do not prove generic laws;
/// unlike a generic TypeMismatch, an unrelated false claim cannot pass.
#[test]
fn lower_and_upper_refusals_match_their_controlled_false_inclusions() {
    let sources = seed_programs();
    let (positive, negatives) = sources.split_first().expect("positive seed Ken fence");
    let [lower, upper] = negatives else {
        panic!("seed must include one independent refusal for each bound");
    };
    for (refused, is_lower) in [(lower, true), (upper, false)] {
        let mut env = loaded_interval();
        env.elaborate_file(positive)
            .expect("all positive seed controls check before a refusal");
        assert_refusal_matches_its_bound(positive, refused, is_lower);
        let negative_decls = parse_decls(refused).expect("negative theorem parses");
        let [SourceDecl::TheoremDecl { span: theorem, .. }] = negative_decls.as_slice() else {
            panic!("negative fence must contain one theorem");
        };
        match env.elaborate_file(refused) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) if span.start >= theorem.start
                && span.end <= theorem.end
                && span.start < span.end => {}
            other => panic!("false endpoint-inclusion theorem must kernel-reject: {other:?}"),
        }
    }
}
