//! Four interval distributivity inclusions: spec/50-stdlib/68-numerical.md §4.
//! Source: conformance/stdlib/numerical/seed-interval-distributivity.md.
//! Promise class: durable invariant. Closed clients supplement checked generic laws.

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
    "/../../conformance/stdlib/numerical/seed-interval-distributivity.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_interval() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], INTERVAL)
        .expect("§4 RationalInterval roots-loads from checked providers");
    env
}

// IsTrue is already bound to its canonical GlobalId in the compiler base.
// Drop only the redundant unaliased selective import, using its parsed span;
// the shipped seed and every declaration after the import remain unchanged.
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
        .expect("§4 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| omit_redundant_is_true(&SEED[fence.body_range]))
        .collect()
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Numeric.RationalInterval
  (Interval, interval_add, interval_mul, interval_inside,
   interval_distrib_l_inside, interval_factor_l_inside,
   interval_distrib_r_inside, interval_factor_r_inside)

theorem generic_distrib_l (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside
    (interval_mul i (interval_add j k))
    (interval_add (interval_mul i j) (interval_mul i k))) =
  interval_distrib_l_inside i j k

theorem generic_factor_l (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside
    (interval_add (interval_mul i j) (interval_mul i k))
    (interval_mul i (interval_add j k))) =
  interval_factor_l_inside i j k

theorem generic_distrib_r (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside
    (interval_mul (interval_add j k) i)
    (interval_add (interval_mul j i) (interval_mul k i))) =
  interval_distrib_r_inside i j k

theorem generic_factor_r (i : Interval) (j : Interval) (k : Interval) :
  IsTrue (interval_inside
    (interval_add (interval_mul j i) (interval_mul k i))
    (interval_mul (interval_add j k) i)) =
  interval_factor_r_inside i j k
"#;

/// SOURCE: spec/50-stdlib/68-numerical.md §4 and §64's Rational laws.
/// PROMISE CLASS: durable invariant.
/// MEASURED: all four exports belong to RationalInterval and have checked
/// transparent proof bodies, their four arbitrary-input public types check
/// independently, two proof stages refuse selective import, and the exact
/// provider trusted GlobalId set survives loading the interval package.
/// CLAIMED: four separate public §4 inclusions check at zero new trust.
/// THE GAP: this measures a local delta; provider assumptions remain inherited,
/// and the generic types do not alone check closed endpoint observations.
#[test]
fn generic_four_inclusions_are_owned_transparent_private_and_zero_trust() {
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
        .expect("the four generic §4 proof bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§4 trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§4 adds no trusted GlobalId identity");
    for name in [
        "interval_distrib_l_inside",
        "interval_factor_l_inside",
        "interval_distrib_r_inside",
        "interval_factor_r_inside",
    ] {
        let id = env.globals[&format!("{INTERVAL}.{name}")];
        assert!(
            owned.contains(&id),
            "{name} belongs to the interval package"
        );
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a kernel-checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("client independently consumes all four exact §4 types");
    for private in ["equiv_leq", "rational_distrib_r"] {
        match env.elaborate_file(&format!("import {INTERVAL} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{INTERVAL}.{private}"));
            }
            other => panic!("{private} must stay package-private: {other:?}"),
        }
    }
}

/// SOURCE: conformance/stdlib/numerical/seed-interval-distributivity.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the shipped distinct-endpoint positive client roots-loads and
/// checks both left directions, both right directions, and the observed
/// dropped-product False control on the same operands before any refusal.
/// CLAIMED: §4 holds for the closed exact endpoint values of the seed.
/// THE GAP: a fixed fixture cannot replace four generic checked proof types.
#[test]
fn shipped_positive_distributivity_seed_checks() {
    let sources = seed_programs();
    let [positive, _negative] = sources.as_slice() else {
        panic!("§4 seed needs one positive client and one isolated refusal");
    };
    loaded_interval()
        .elaborate_file(positive)
        .expect("four shipped law instances and same-input controls check");
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unsupported closed §4 proposition: {other:?}"),
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

fn theorem_by_name<'a>(decls: &'a [SourceDecl], expected: &str) -> &'a SourceType {
    let matching = decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl { name, theorem, .. } if name == expected => Some(theorem),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [theorem] = matching.as_slice() else {
        panic!("the seed needs exactly one {expected}");
    };
    theorem
}

fn expr_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::ECon(name, _) => format!("constructor:{name}"),
        SourceExpr::EVar(name, _) => format!("variable:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!("({} {})", expr_shape(function), expr_shape(argument))
        }
        other => panic!("unsupported closed §4 expression: {other:?}"),
    }
}

fn view_by_name<'a>(decls: &'a [SourceDecl], expected: &str) -> &'a SourceExpr {
    let matching = decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::ViewDecl {
                name, params, body, ..
            } if name == expected && params.is_empty() => Some(body),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [body] = matching.as_slice() else {
        panic!("the seed needs exactly one closed {expected} interval");
    };
    body
}

fn assert_dropped_product_refusal_is_paired(positive: &str, negative: &str) {
    let decls = parse_decls(positive).expect("positive seed parses");
    let instances = decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl { name, body, .. }
                if name == "left_distributivity_law_instance" =>
            {
                Some(body)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    let [instance] = instances.as_slice() else {
        panic!("one left distributivity law instance must be exercised");
    };
    let args = expr_application(instance, "interval_distrib_l_inside")
        .expect("left law instance applies the checked public export");
    let [i, j, k] = args.as_slice() else {
        panic!("the law instance has precisely three interval operands");
    };
    let snippet = |expr: &SourceExpr| &positive[expr.span().start..expr.span().end];
    for (stage, expression) in [
        (
            "left_distributed",
            format!(
                "interval_mul {} (interval_add {} {})",
                snippet(i),
                snippet(j),
                snippet(k)
            ),
        ),
        (
            "left_product",
            format!("interval_mul {} {}", snippet(i), snippet(j)),
        ),
        (
            "left_factored",
            format!(
                "interval_add (interval_mul {} {}) (interval_mul {} {})",
                snippet(i),
                snippet(j),
                snippet(i),
                snippet(k)
            ),
        ),
    ] {
        let derived = format!("const derived_stage : Interval = {expression}");
        let statements = parse_decls(&derived).expect("derived interval stage parses");
        let [SourceDecl::ViewDecl { body, .. }] = statements.as_slice() else {
            panic!("derived interval stage has one definition");
        };
        assert_eq!(
            expr_shape(view_by_name(&decls, stage)),
            expr_shape(body),
            "{stage} must have the law instance's exact endpoint operation role"
        );
    }
    let false_control = theorem_by_name(&decls, "dropped_product_control");
    let expected_false = "theorem derived_drop_false : Equal Bool (interval_inside left_distributed left_product) False = Proved";
    let expected_true = "theorem derived_drop_true : Equal Bool (interval_inside left_distributed left_product) True = Proved";
    let parse_expected = |source: &str| {
        let declarations = parse_decls(source).expect("derived assertion parses");
        let [SourceDecl::TheoremDecl { theorem, .. }] = declarations.as_slice() else {
            panic!("derived assertion is exactly one theorem");
        };
        type_shape(theorem)
    };
    assert_eq!(
        type_shape(false_control),
        parse_expected(&expected_false),
        "checked False control must be the law instance's dropped product"
    );
    let negative_decls = parse_decls(negative).expect("refused seed Ken parses");
    let [SourceDecl::TheoremDecl {
        theorem,
        body: SourceExpr::ECon(proof, _),
        span: theorem_span,
        ..
    }] = negative_decls.as_slice()
    else {
        panic!("isolated refusal must have exactly one attempted proof theorem");
    };
    assert_eq!(proof, "Proved", "refusal must attempt a checked proof");
    assert_eq!(
        type_shape(theorem),
        parse_expected(&expected_true),
        "the refused proposition must change only the control's False to True"
    );
    assert!(theorem_span.start < theorem_span.end);
}

/// SOURCE: the §4 seed's isolated dropped-product refusal.
/// PROMISE CLASS: durable invariant.
/// MEASURED: positive law instances and the same-input False observation check
/// first; parsed ASTs confirm the control observes the left law instance's
/// operands with one product removed, and the refusal changes only False to
/// True. The attempted proof kernel-rejects within its own theorem span.
/// CLAIMED: the full distributed interval cannot fit inside the retained
/// product without the missing term for this nondegenerate seed fixture.
/// THE GAP: one closed refusal cannot prove generic §4 inclusions; an
/// unrelated false proposition is rejected by the AST comparison, not counted.
#[test]
fn dropped_product_refusal_is_about_the_same_checked_operands() {
    let sources = seed_programs();
    let [positive, negative] = sources.as_slice() else {
        panic!("§4 seed needs one positive client and one isolated refusal");
    };
    let mut env = loaded_interval();
    env.elaborate_file(positive)
        .expect("positive same-input control and four laws check first");
    assert_dropped_product_refusal_is_paired(positive, negative);
    let declarations = parse_decls(negative).expect("refusal parses");
    let [SourceDecl::TheoremDecl { span: theorem, .. }] = declarations.as_slice() else {
        panic!("one attempted theorem is required");
    };
    match env.elaborate_file(negative) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start >= theorem.start && span.end <= theorem.end && span.start < span.end => {}
        other => panic!("dropped-product claim must kernel-reject: {other:?}"),
    }
}
