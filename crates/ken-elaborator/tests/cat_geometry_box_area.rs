//! Natural area of closed boxes: spec/50-stdlib/67-geometry.md §4.
//! The shipped seed is conformance/stdlib/geometry/seed-box-area.md.
//! Promise class: durable invariant. Checked generic laws complement the
//! independent closed point, proper-box, inverted-axis, and nesting cases.

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
    "/../../conformance/stdlib/geometry/seed-box-area.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_geometry() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("NatBox area and its provider closure roots-load");
    env
}

// The compiler base already binds canonical IsTrue. Remove only its parsed,
// unaliased duplicate import; retain every other import and declaration.
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
        .expect("§4 seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| without_redundant_is_true(&SEED[fence.body_range]))
        .collect()
}

// Restate each §4 public law at arbitrary boxes or coordinates. A weakened
// public type cannot pass merely because the fixed seed boxes are convenient.
const GENERIC_CLIENT: &str = r#"
import Core.Classes.LawfulClasses (leq_nat)
import Algorithm.Geometry.NatBox
  (Box, MkBox, MkPoint, inside, area, area_inside,
   area_inverted_x, area_inverted_y)

fn client_area (b : Box) : Nat = area b

theorem client_area_inside (b : Box) (c : Box)
    (h : IsTrue (inside b c)) :
  IsTrue (leq_nat (area b) (area c)) = area_inside b c h

theorem client_area_inverted_x
    (lx : Nat) (ly : Nat) (hx : Nat) (hy : Nat)
    (h : IsTrue (leq_nat (Suc hx) lx)) :
  Equal Nat (area (MkBox (MkPoint lx ly) (MkPoint hx hy))) Zero =
  area_inverted_x lx ly hx hy h

theorem client_area_inverted_y
    (lx : Nat) (ly : Nat) (hx : Nat) (hy : Nat)
    (h : IsTrue (leq_nat (Suc hy) ly)) :
  Equal Nat (area (MkBox (MkPoint lx ly) (MkPoint hx hy))) Zero =
  area_inverted_y lx ly hx hy h
"#;

/// SOURCE: spec/50-stdlib/67-geometry.md §§4.1–4.3.
/// PROMISE CLASS: durable invariant.
/// MEASURED: all four §4 globals are transparent and belong to a roots-loaded
/// NatBox; their exact types check in a generic client, all thirteen new
/// helpers refuse selective import, and the trusted GlobalId identity set
/// stays equal above the explicitly loaded provider closure.
/// CLAIMED: the four §4 exports have the specified checked generic types
/// at zero new trust, while the thirteen named new helpers stay private.
/// THE GAP: inherited provider assumptions remain; exact public-inventory
/// closure is source-reviewed, and fixed values are checked in the seed.
#[test]
fn generic_area_exports_are_checked_and_private_helpers_add_no_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("§4 area bodies and generic proofs kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§4 trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§4 adds no trusted GlobalId identity");

    for public in ["area", "area_inside", "area_inverted_x", "area_inverted_y"] {
        let id = env.globals[&format!("{GEOMETRY}.{public}")];
        assert!(owned.contains(&id), "{public} must belong to NatBox");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{public} must have a checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("generic public client consumes each §4 type");

    for private in [
        "axis_span",
        "sub_left_mono",
        "sub_below",
        "sub_right_antitone",
        "axis_span_mono",
        "mul_right_mono_any",
        "mul_left_mono_any",
        "mul_both_mono",
        "axis_span_within",
        "area_intersect_left",
        "area_intersect_right",
        "axis_span_inverted",
        "axis_span_point",
    ] {
        match env.elaborate_file(&format!("import {GEOMETRY} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GEOMETRY}.{private}"));
            }
            other => panic!("{private} must remain private: {other:?}"),
        }
    }
}

/// SOURCE: conformance/stdlib/geometry/seed-box-area.md positive client.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the exact shipped positive Ken fence checks after importing the
/// four public exports; its checked observations distinguish inclusive point
/// and proper boxes, each separately inverted axis and a nested pair.
/// CLAIMED: §4's closed area results and positive law applications hold.
/// THE GAP: these fixed boxes supplement, not replace, the generic proofs.
#[test]
fn shipped_area_seed_positive_client_checks() {
    let programs = seed_programs();
    let [positive, _refusal] = programs.as_slice() else {
        panic!("§4 seed must have one positive and one isolated refusal fence");
    };
    loaded_geometry()
        .elaborate_file(positive)
        .expect("all shipped §4 positive declarations check");
}

// Compare parsed proposition trees independent of whitespace or spans. The
// closed false-value control in the positive fence must also check first.
fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unsupported closed area proposition: {other:?}"),
    }
}

/// SOURCE: conformance/stdlib/geometry/seed-box-area.md refused claim.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after the same positive seed checked the independently observed
/// false strict comparison, the only appended theorem asserts precisely that
/// strict comparison as IsTrue; its own Proved receives kernel TypeMismatch.
/// CLAIMED: this nested inner box cannot have strictly greater area than its
/// containing outer box, not just any unrelated false closed proposition.
/// THE GAP: this witness is closed; generic monotonicity is checked above.
#[test]
fn shipped_strictly_larger_inner_area_refuses_on_the_pinned_pair() {
    let programs = seed_programs();
    let [positive, refused] = programs.as_slice() else {
        panic!("§4 seed must have one positive and one isolated refusal fence");
    };
    let expected_false = parse_decls(
        "theorem expected_false : Equal Bool \
         (leq_nat (Suc (area outer_box)) (area inner_box)) False = Proved",
    )
    .expect("same-pair false-value control parses");
    let expected_refusal = parse_decls(
        "theorem expected_refusal : IsTrue \
         (leq_nat (Suc (area outer_box)) (area inner_box)) = Proved",
    )
    .expect("same-pair false strict-area proposition parses");
    let [SourceDecl::TheoremDecl {
        theorem: expected_false_goal,
        ..
    }] = expected_false.as_slice()
    else {
        panic!("independent false-value proposition is one theorem");
    };
    let [SourceDecl::TheoremDecl {
        theorem: expected_refusal_goal,
        ..
    }] = expected_refusal.as_slice()
    else {
        panic!("independent refused proposition is one theorem");
    };
    let positive_decls = parse_decls(positive).expect("positive area Ken parses");
    let matching_false_controls = positive_decls
        .iter()
        .filter(|decl| {
            matches!(decl, SourceDecl::TheoremDecl { theorem, .. }
                if type_shape(theorem) == type_shape(expected_false_goal))
        })
        .count();
    assert_eq!(
        matching_false_controls, 1,
        "positive seed needs one independent false value on this pair"
    );

    let negative_decls = parse_decls(refused).expect("isolated refusal Ken parses");
    let [SourceDecl::TheoremDecl {
        name,
        theorem,
        body: Expr::ECon(proof, _),
        span,
        ..
    }] = negative_decls.as_slice()
    else {
        panic!("refusal fence must contain just one attempted theorem proof");
    };
    assert_eq!(name, "reject_inner_area_strictly_larger");
    assert_eq!(proof, "Proved", "refusal must attempt its proof");
    assert_eq!(
        type_shape(theorem),
        type_shape(expected_refusal_goal),
        "refusal must assert the strictly-larger-inner-area proposition"
    );

    let mut env = loaded_geometry();
    env.elaborate_file(positive)
        .expect("same-pair area values, nesting and false strict comparison check first");
    match env.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span: failure,
        }) if failure.start >= span.start
            && failure.end <= span.end
            && failure.start < failure.end => {}
        other => panic!("false strict-area theorem must kernel-reject: {other:?}"),
    }
}
