//! Corner-wise inclusion order: spec/50-stdlib/67-geometry.md §5.
//! Seed: conformance/stdlib/geometry/seed-box-inside-order.md.
//! Promise class: durable invariant. Generic laws complement closed inverted,
//! intersected and hull boxes with same-operand reverse-order refusals.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr, ImportKind, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const GEOMETRY: &str = "Algorithm.Geometry.NatBox";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/geometry/seed-box-inside-order.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_geometry() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("NatBox §5 proofs and provider closure root-load");
    env
}

// The compiler base binds IsTrue already. Omit only its redundant unaliased
// import item in the executable client; do not change the source seed or any
// other imported name, fixture, assertion, or refused proposition.
fn without_redundant_is_true(source: &str) -> String {
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
        .expect("§5 seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| without_redundant_is_true(&SEED[fence.body_range]))
        .collect()
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Geometry.NatBox
  (Box, inside, intersect, hull, inside_trans, intersect_greatest, hull_least)

theorem client_inside_trans (a : Box) (b : Box) (c : Box)
    (ab : IsTrue (inside a b)) (bc : IsTrue (inside b c)) :
  IsTrue (inside a c) = inside_trans a b c ab bc

theorem client_intersect_greatest (a : Box) (b : Box) (c : Box)
    (ab : IsTrue (inside a b)) (ac : IsTrue (inside a c)) :
  IsTrue (inside a (intersect b c)) = intersect_greatest a b c ab ac

theorem client_hull_least (b : Box) (c : Box) (d : Box)
    (bd : IsTrue (inside b d)) (cd : IsTrue (inside c d)) :
  IsTrue (inside (hull b c) d) = hull_least b c d bd cd
"#;

/// SOURCE: spec/50-stdlib/67-geometry.md §§5.1–5.2.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the three new laws are transparent and NatBox-owned; a public
/// client uses their exact generic premises and results; the four per-axis
/// helpers reject selective import. Explicit providers and package have
/// identical sets of trusted GlobalId identities.
/// CLAIMED: §5's unconditional corner-order proofs add no trusted surface.
/// THE GAP: inherited provider trust remains; this does not enumerate all
/// other public package exports or infer behavior from a fixed seed alone.
#[test]
fn generic_order_laws_are_owned_checked_private_and_add_no_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} root-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("NatBox §5 checked bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§5 trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§5 adds no trusted GlobalId identity");
    for law in ["inside_trans", "intersect_greatest", "hull_least"] {
        let id = env.globals[&format!("{GEOMETRY}.{law}")];
        assert!(owned.contains(&id), "{law} must be NatBox-owned");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("generic public client consumes all three exact §5 types");
    for private in [
        "axis_within",
        "axis_within_trans",
        "axis_within_meet",
        "axis_within_join",
    ] {
        match env.elaborate_file(&format!("import {GEOMETRY} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GEOMETRY}.{private}"));
            }
            other => panic!("{private} must remain private: {other:?}"),
        }
    }
}

/// SOURCE: conformance/stdlib/geometry/seed-box-inside-order.md accepted.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the complete accepted seed client elaborates through package
/// imports, checking an inverted transitive chain, overlapping intersection,
/// inverted-operand hull and same-operand reverse-order controls.
/// CLAIMED: the three order laws have their specified closed behavior.
/// THE GAP: these fixed boxes supplement, not replace, the generic proofs.
#[test]
fn shipped_order_seed_accepted_client_checks() {
    let programs = seed_programs();
    assert_eq!(programs.len(), 4, "one accepted and three refusal fences");
    loaded_geometry()
        .elaborate_file(&programs[0])
        .expect("complete accepted §5 seed checks through package imports");
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unsupported closed §5 proposition: {other:?}"),
    }
}

fn parsed_goal(source: &str) -> String {
    let declarations = parse_decls(source).expect("independent proposition parses");
    let [SourceDecl::TheoremDecl { theorem, .. }] = declarations.as_slice() else {
        panic!("expected one theorem proposition");
    };
    type_shape(theorem)
}

/// SOURCE: conformance/stdlib/geometry/seed-box-inside-order.md refusals.
/// PROMISE CLASS: durable invariant.
/// MEASURED: every accepted seed declaration and each same-operand positive
/// plus observed-false reverse control check first. Three isolated theorems
/// state independently specified, well-typed reverse inclusions; Proved then
/// receives KernelRejected(TypeMismatch) at its own theorem on fresh clients.
/// CLAIMED: neither the hull nor a whole operand fits in its narrower peer,
/// and a transitive inside chain cannot be reversed, even with inverted boxes.
/// THE GAP: finite reverse cases do not exhaust possible corner comparisons;
/// QA owns independent AC-3 production mutations of the three laws/helpers.
#[test]
fn shipped_order_seed_reverse_inclusions_refuse_on_controlled_pairs() {
    let programs = seed_programs();
    assert_eq!(programs.len(), 4, "one accepted and three refusal fences");
    let positive = &programs[0];
    let accepted_decls = parse_decls(positive).expect("accepted §5 Ken parses");
    let cases = [
        (
            "reject_hull_inside_left_operand",
            "IsTrue (inside (hull small mid) small)",
            "IsTrue (inside (hull small mid) mid)",
            "Equal Bool (inside mid small) False",
        ),
        (
            "reject_left_inside_intersection",
            "IsTrue (inside mid (intersect mid small))",
            "IsTrue (inside (intersect mid small) small)",
            "Equal Bool (inside mid small) False",
        ),
        (
            "reject_reverse_inverted_chain",
            "IsTrue (inside big inverted)",
            "IsTrue (inside inverted big)",
            "Equal Bool (inside big inverted) False",
        ),
    ];
    for ((name, goal, accepted, reversed_false), negative) in
        cases.into_iter().zip(programs.iter().skip(1))
    {
        for control in [accepted, reversed_false] {
            let wanted = parsed_goal(&format!("theorem control : {control} = Proved"));
            assert!(
                accepted_decls.iter().any(|decl| matches!(decl,
                    SourceDecl::TheoremDecl { theorem, .. } if type_shape(theorem) == wanted)),
                "{name}: same-operand accepted/false control must be in seed"
            );
        }
        let independent = parsed_goal(&format!("theorem expected : {goal} = Proved"));
        let negative_decls = parse_decls(negative).expect("isolated refusal parses");
        let [SourceDecl::TheoremDecl {
            name: actual_name,
            params,
            theorem,
            body: Expr::ECon(proof, _),
            span,
            ..
        }] = negative_decls.as_slice()
        else {
            panic!("each refusal must have exactly one closed attempted theorem");
        };
        assert_eq!(actual_name, name);
        assert!(params.is_empty(), "{name} must assert a closed proposition");
        assert_eq!(proof, "Proved", "{name} attempts a checked proof");
        assert_eq!(type_shape(theorem), independent, "{name}: exact false goal");

        let proposition_span = theorem.span();
        let proposition = &negative[proposition_span.start..proposition_span.end];
        let hypothetical = format!("theorem typed_refusal (h : {proposition}) : {proposition} = h");
        let mut type_control = loaded_geometry();
        type_control
            .elaborate_file(positive)
            .expect("full positive prefix checks before well-typed control");
        type_control
            .elaborate_file(&hypothetical)
            .expect("the false proposition is well-typed when witnessed");

        let mut env = loaded_geometry();
        env.elaborate_file(positive)
            .expect("full positive prefix checks before each fresh refusal");
        match env.elaborate_file(negative) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span: failed,
            }) if failed.start >= span.start
                && failed.end <= span.end
                && failed.start < failed.end => {}
            other => panic!("{name} must kernel-reject at its own theorem: {other:?}"),
        }
    }
}
