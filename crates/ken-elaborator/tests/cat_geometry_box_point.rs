//! Public point-box laws: spec/50-stdlib/67-geometry.md §14.
//! Source: conformance/stdlib/geometry/seed-box-point.md.
//! Promise class: durable invariant. Generic checked proofs and closed
//! same-form controls/refusals have distinct obligations.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, ImportKind,
    KenMdFenceRole, Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const GEOMETRY: &str = "Algorithm.Geometry.NatBox";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/geometry/seed-box-point.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_geometry() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("NatBox and its provider closure root-load");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Geometry.NatBox
  (Point, MkBox, contains, area, contains_point_box, area_point_box)

theorem generic_contains_point (p : Point)
    : IsTrue (contains (MkBox p p) p) = contains_point_box p

theorem generic_area_point (p : Point)
    : Equal Nat (area (MkBox p p)) (Suc Zero) = area_point_box p
"#;

/// SOURCE: spec 67 §14. PROMISE CLASS: durable invariant.
/// MEASURED: the exact trusted GlobalId identity set is unchanged after the
/// five-provider closure and NatBox load; both new names belong to NatBox,
/// have Transparent checked bodies and accept an open-point selective-import
/// client. Both inherited span helpers refuse selective import by UnboundName.
/// CLAIMED: both universal public laws check without new trust or public span
/// helpers. THE GAP: the providers may carry inherited trust and the closed
/// seed below alone cannot establish either generic theorem.
#[test]
fn generic_point_box_laws_are_public_checked_and_zero_new_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Logic.Conjunction",
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
        .expect("both §14 proof bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§14 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "point-box laws add no trusted GlobalId");
    for law in ["contains_point_box", "area_point_box"] {
        let id = env.globals[&format!("{GEOMETRY}.{law}")];
        assert!(owned.contains(&id), "{law} must be NatBox-owned");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} must have a checked Transparent body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("both exact public §14 types check for an open point");
    for helper in ["axis_span_point", "axis_span"] {
        let id = env.globals[&format!("{GEOMETRY}.{helper}")];
        assert!(owned.contains(&id), "{helper} remains owned by NatBox");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        match env.elaborate_file(&format!("import {GEOMETRY} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GEOMETRY}.{helper}"));
            }
            other => panic!("{helper} must remain private: {other:?}"),
        }
    }
}

// IsTrue is already bound by the compiler floor. Strip only the exact parsed,
// unaliased redundant import; all other seed declarations and imports remain.
fn without_redundant_is_true(source: &str) -> String {
    let mut client = source.to_owned();
    for declaration in parse_decls(source)
        .expect("Ken seed parses")
        .into_iter()
        .rev()
    {
        if let SourceDecl::ImportDecl {
            module,
            kind: ImportKind::Selective(items),
            span,
        } = declaration
        {
            if module == "Core.Classes.LawfulClasses" {
                let mut kept = Vec::new();
                let mut removed = false;
                for item in items {
                    if item.name == "IsTrue" && item.rename.is_none() {
                        removed = true;
                    } else {
                        kept.push(match item.rename {
                            Some(alias) => format!("{} as {alias}", item.name),
                            None => item.name,
                        });
                    }
                }
                if removed {
                    let replacement = if kept.is_empty() {
                        String::new()
                    } else {
                        format!("import {module} ({})", kept.join(", "))
                    };
                    client.replace_range(span.start..span.end, &replacement);
                }
            }
        }
    }
    client
}

// Select a unique checked declaration, never fence ordinal or prose spelling.
fn seed_case(name: &str) -> String {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§14 seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("§14 seed program parses")
            .into_iter()
            .any(|decl| matches!(decl, SourceDecl::TheoremDecl { name: actual, .. } if actual == name));
        if found {
            assert!(
                selected.replace(source).is_none(),
                "duplicate seed theorem {name}"
            );
        }
    }
    without_redundant_is_true(selected.unwrap_or_else(|| panic!("missing §14 seed theorem {name}")))
}

fn shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", shape(function), shape(argument))
        }
        other => panic!("unexpected closed point-box goal: {other:?}"),
    }
}

fn theorem<'a>(decls: &'a [SourceDecl], name: &str) -> (&'a SourceType, &'a SourceExpr) {
    let matches = decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name: actual,
                theorem,
                body,
                ..
            } if actual == name => Some((theorem, body)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [one] = matches.as_slice() else {
        panic!("exactly one seed theorem {name} is required");
    };
    *one
}

fn law_body(decls: &[SourceDecl], name: &str, law: &str, point: &str) {
    let (_, body) = theorem(decls, name);
    let SourceExpr::EApp(function, argument, _) = body else {
        panic!("{name} must apply the public law {law}");
    };
    assert!(matches!(function.as_ref(), SourceExpr::EVar(actual, _) if actual == law));
    assert!(matches!(argument.as_ref(), SourceExpr::EVar(actual, _) if actual == point));
}

fn proved_body<'a>(decls: &'a [SourceDecl], name: &str) -> &'a SourceType {
    let (goal, body) = theorem(decls, name);
    assert!(
        matches!(body, SourceExpr::ECon(actual, _) if actual == "Proved"),
        "{name} must check with the same Proved proof form"
    );
    goal
}

const FIXTURE_VALUES: &str = r#"
theorem origin_is_zero : Equal Point origin (MkPoint n0 n0) = Refl
theorem off_is_three_five : Equal Point off (MkPoint n3 n5) = Refl
theorem other_is_three_four : Equal Point other (MkPoint n3 n4) = Refl
"#;

/// SOURCE: conformance/stdlib/geometry/seed-box-point.md, spec 67 §14.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the entire positive seed (both public-law applications at the
/// origin and off, the reflexive membership at off, and both direct Proved
/// controls) checks before each fresh isolated false claim. Independent Refl
/// fixtures bind the off and outside coordinates. Each isolated refusal has
/// the same Proved form and differs from its accepted control only in its
/// value/point; the kernel returns TypeMismatch within its own declaration.
/// CLAIMED: inclusive point-box area is one and the same box does not contain
/// a distinct outside point. THE GAP: finite witnesses do not establish the
/// checked open laws above; a refusal absent a full positive prefix is invalid.
#[test]
fn seed_controls_check_and_both_false_point_box_claims_refuse() {
    let accepted = seed_case("origin_area_law");
    let positive_decls = parse_decls(&accepted).expect("accepted client parses");
    for (name, law, point) in [
        ("origin_area_law", "area_point_box", "origin"),
        ("off_area_law", "area_point_box", "off"),
        ("off_contains_law", "contains_point_box", "off"),
    ] {
        law_body(&positive_decls, name, law, point);
    }
    let area_control = proved_body(&positive_decls, "off_area_value");
    let contains_control = proved_body(&positive_decls, "off_contains_value");
    assert_eq!(
        shape(area_control),
        "(((name:Equal name:Nat) (name:area ((name:MkBox name:off) name:off))) (name:Suc name:Zero))"
    );
    assert_eq!(
        shape(contains_control),
        "(name:IsTrue ((name:contains ((name:MkBox name:off) name:off)) name:off))"
    );
    let mut positive = loaded_geometry();
    positive
        .elaborate_file(&accepted)
        .expect("entire accepted §14 seed with both controls checks");
    positive
        .elaborate_file(FIXTURE_VALUES)
        .expect("origin/off/other must denote the required coordinates");
    for name in [
        "origin_coordinates",
        "off_coordinates",
        "other_coordinates",
        "origin_area_law",
        "off_area_law",
        "off_contains_law",
        "off_area_value",
        "off_contains_value",
        "origin_is_zero",
        "off_is_three_five",
        "other_is_three_four",
    ] {
        let id = positive.globals[name];
        assert!(matches!(
            positive.env.lookup(id),
            Some(Decl::Transparent { .. })
        ));
    }
    for (name, control, expected) in [
        (
            "seed_area_neg",
            area_control,
            "(((name:Equal name:Nat) (name:area ((name:MkBox name:off) name:off))) name:Zero)",
        ),
        (
            "seed_contains_neg",
            contains_control,
            "(name:IsTrue ((name:contains ((name:MkBox name:off) name:off)) name:other))",
        ),
    ] {
        let refused = seed_case(name);
        let negative_decls = parse_decls(&refused).expect("isolated refusal parses");
        let [SourceDecl::TheoremDecl {
            name: actual,
            params,
            theorem: goal,
            body: SourceExpr::ECon(proof, _),
            span: claim_span,
            ..
        }] = negative_decls.as_slice()
        else {
            panic!("{name} must be one closed theorem with a Proved body");
        };
        assert_eq!(actual, name);
        assert!(params.is_empty());
        assert_eq!(proof, "Proved", "{name} must use the control's proof form");
        assert_eq!(
            shape(goal),
            expected,
            "{name} must pin the specified false claim"
        );
        let only_changed = if name == "seed_area_neg" {
            shape(control).replace("(name:Suc name:Zero)", "name:Zero")
        } else {
            let control_shape = shape(control);
            let (prefix, suffix) = control_shape
                .rsplit_once(" name:off))")
                .expect("control ends at the tested point");
            format!("{prefix} name:other)){suffix}")
        };
        assert_eq!(shape(goal), only_changed, "only the value/point may change");
        let mut negative = loaded_geometry();
        negative
            .elaborate_file(&accepted)
            .expect("fresh complete accepted client precedes the false theorem");
        negative
            .elaborate_file(FIXTURE_VALUES)
            .expect("fresh fixture identities precede the false theorem");
        match negative.elaborate_file(&refused) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) if span.start >= claim_span.start
                && span.start < span.end
                && span.end <= claim_span.end => {}
            other => panic!("{name} must kernel-reject at its own Proved claim: {other:?}"),
        }
    }
}
