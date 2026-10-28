//! Exact natural-coordinate geometry, spec/50-stdlib/67-geometry.md §§1–3.
//! Positive and negative clients come from
//! conformance/stdlib/geometry/seed-nat-box.md.
//! Promise class: durable invariant. The fixed endpoints, corners and
//! distance are specified by that seed, not derived from the functions tested.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr, ImportKind, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const GEOMETRY: &str = "Algorithm.Geometry.NatBox";
const CONTAINMENT_SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/geometry/seed-box-containment.md"
));

// The compiler base already binds canonical IsTrue. Remove only a parsed
// duplicate import; keep all other imported names and ignore source spacing.
fn without_redundant_is_true_import(source: &str) -> String {
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

fn containment_sources() -> Vec<String> {
    extract_ken_md(CONTAINMENT_SEED)
        .expect("shipped box-containment seed Ken fences parse")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| without_redundant_is_true_import(&CONTAINMENT_SEED[fence.body_range]))
        .collect()
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn geometry_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("NatBox package roots-loads from published providers");
    env
}

/// SOURCE: spec/50-stdlib/67-geometry.md §§1.3–1.4.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after provider closure, both checked Nat.Order bound proofs
/// and the six original geometry laws plus the two commutativity laws are
/// transparent; six independent original law clients elaborate, with identical
/// trusted GlobalId sets
/// before and after loading Nat.Order and then NatBox.
/// CLAIMED: both tightness laws are reusable as public proofs and the
/// original six generic geometry laws and two commutativity laws add no new trust.
/// THE GAP: checking loaded theorem types and transparent proof bodies is
/// universal proof evidence, while closed output behavior is pinned below;
/// inherited provider assumptions remain in both exact identity sets.
#[test]
fn generic_public_geometry_laws_and_order_bounds_add_no_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-load: {error:?}"));
    }
    let providers = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    env.elaborate_module_from_roots(&[root()], "Data.Numeric.Nat.Order")
        .expect("public bound provider roots-loads");
    assert_eq!(
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        providers,
        "Nat.Order adds no trust"
    );
    for attached in ["min::greatest", "max::least"] {
        let id = env.globals[&format!("Data.Numeric.Nat.Order.{attached}")];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("NatBox proofs kernel-check with public Nat bounds");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "NatBox adds no trusted GlobalId");
    for law in [
        "intersect_sound",
        "intersect_complete",
        "hull_contains_left",
        "hull_contains_right",
        "intersect_comm",
        "hull_comm",
        "intersect_assoc",
        "hull_assoc",
        "manhattan_symm",
        "manhattan_self",
    ] {
        let id = env.globals[&format!("{GEOMETRY}.{law}")];
        assert!(owned.contains(&id), "{law} must be package-owned");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} must have a checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("all six independently restated §1.3 law types elaborate");
}

// §1.3 theorem types are restated at generic boxes/points, not only seeded
// closed values; a weakened declaration cannot pass just by keeping its name.
const GENERIC_CLIENT: &str = r##"
import Algorithm.Geometry.NatBox
  (Point, Box, contains, intersect, hull, manhattan,
   intersect_sound, intersect_complete, hull_contains_left,
   hull_contains_right, manhattan_symm, manhattan_self)

theorem generic_intersect_sound (b : Box) (c : Box) (p : Point) :
  IsTrue (contains (intersect b c) p) →
  And (IsTrue (contains b p)) (IsTrue (contains c p)) =
  intersect_sound b c p
theorem generic_intersect_complete (b : Box) (c : Box) (p : Point) :
  IsTrue (contains b p) → IsTrue (contains c p) →
  IsTrue (contains (intersect b c) p) =
  intersect_complete b c p
theorem generic_hull_contains_left (b : Box) (c : Box) (p : Point) :
  IsTrue (contains b p) → IsTrue (contains (hull b c) p) =
  hull_contains_left b c p
theorem generic_hull_contains_right (b : Box) (c : Box) (p : Point) :
  IsTrue (contains c p) → IsTrue (contains (hull b c) p) =
  hull_contains_right b c p
theorem generic_manhattan_symm (p : Point) (q : Point) :
  Equal Nat (manhattan p q) (manhattan q p) = manhattan_symm p q
theorem generic_manhattan_self (p : Point) :
  Equal Nat (manhattan p p) Zero = manhattan_self p
"##;

// The 22 seed theorems and their fixtures are verbatim. The seed's redundant
// IsTrue import is omitted in this in-memory client: the compiler base already
// binds the same canonical IsTrue identity, and reimporting it ambiguates that
// name. Constructed expectations never call intersect, hull or manhattan.
const SEED_POSITIVE: &str = r##"
import Algorithm.Geometry.NatBox
  (Point, MkPoint, Box, MkBox, axis_contains, contains, intersect,
   hull, manhattan, intersect_sound, intersect_complete,
   hull_contains_left, hull_contains_right, manhattan_symm,
   manhattan_self)

const n0 : Nat = Zero
const n1 : Nat = Suc n0
const n2 : Nat = Suc n1
const n3 : Nat = Suc n2
const n4 : Nat = Suc n3
const n5 : Nat = Suc n4

const b02 : Box = MkBox (MkPoint n0 n0) (MkPoint n2 n2)
const b13 : Box = MkBox (MkPoint n1 n1) (MkPoint n3 n3)
const b01 : Box = MkBox (MkPoint n0 n0) (MkPoint n1 n1)
const b23 : Box = MkBox (MkPoint n2 n2) (MkPoint n3 n3)
const only_first : Point = MkPoint n0 n0
const in_both : Point = MkPoint n1 n2

theorem axis_lower_in : Equal Bool (axis_contains n1 n2 n1) True = Proved
theorem axis_upper_in : Equal Bool (axis_contains n1 n2 n2) True = Proved
theorem axis_below_out : Equal Bool (axis_contains n1 n2 n0) False = Proved
theorem axis_above_out : Equal Bool (axis_contains n1 n2 n3) False = Proved
theorem axis_inverted_out : Equal Bool (axis_contains n2 n1 n2) False = Proved

theorem contains_in : Equal Bool (contains b02 in_both) True = Proved
theorem contains_out : Equal Bool (contains b02 (MkPoint n3 n1)) False = Proved

theorem intersect_corners :
  Eq Box (intersect b02 b13)
    (MkBox (MkPoint n1 n1) (MkPoint n2 n2)) = Refl

theorem intersect_disjoint_corners :
  Eq Box (intersect b01 b23)
    (MkBox (MkPoint n2 n2) (MkPoint n1 n1)) = Refl

theorem empty_meet_excludes_lower_input_corner :
  Equal Bool (contains (intersect b01 b23) (MkPoint n1 n1)) False = Proved

theorem empty_meet_excludes_upper_input_corner :
  Equal Bool (contains (intersect b01 b23) (MkPoint n2 n2)) False = Proved

theorem only_first_inside : Equal Bool (contains b02 only_first) True = Proved
theorem only_first_outside_second :
  Equal Bool (contains b13 only_first) False = Proved
theorem only_first_outside_intersection :
  Equal Bool (contains (intersect b02 b13) only_first) False = Proved

theorem hull_corners :
  Eq Box (hull b01 b23)
    (MkBox (MkPoint n0 n0) (MkPoint n3 n3)) = Refl

theorem distance_value :
  Equal Nat (manhattan (MkPoint n1 n4) (MkPoint n3 n1)) n5 = Proved

theorem seed_law_intersect_complete :
  IsTrue (contains (intersect b02 b13) (MkPoint n2 n1)) =
  intersect_complete b02 b13 (MkPoint n2 n1) Proved Proved

theorem seed_law_intersect_sound :
  And (IsTrue (contains b02 in_both))
      (IsTrue (contains b13 in_both)) =
  intersect_sound b02 b13 in_both Proved

theorem seed_law_hull_left :
  IsTrue (contains (hull b01 b23) (MkPoint n0 n1)) =
  hull_contains_left b01 b23 (MkPoint n0 n1) Proved

theorem seed_law_hull_right :
  IsTrue (contains (hull b01 b23) (MkPoint n3 n2)) =
  hull_contains_right b01 b23 (MkPoint n3 n2) Proved

theorem seed_law_manhattan_symm :
  Equal Nat (manhattan (MkPoint n1 n4) (MkPoint n3 n1))
            (manhattan (MkPoint n3 n1) (MkPoint n1 n4)) =
  manhattan_symm (MkPoint n1 n4) (MkPoint n3 n1)

theorem seed_law_manhattan_self :
  Equal Nat (manhattan (MkPoint n3 n4) (MkPoint n3 n4)) Zero =
  manhattan_self (MkPoint n3 n4)
"##;

/// SOURCE: conformance/stdlib/geometry/seed-nat-box.md positive client.
/// PROMISE CLASS: durable invariant.
/// MEASURED: all 22 fixed-value and closed-law declarations elaborate
/// after importing NatBox with its constructor exports, against separately
/// constructed Bool/Nat/corner expectations.
/// CLAIMED: the five operations compute the specified fixed seed values.
/// THE GAP: finite seeds do not establish the six generic laws, checked above.
#[test]
fn twenty_two_seed_assertions_check_from_client_import() {
    let mut env = geometry_env();
    env.elaborate_file(SEED_POSITIVE)
        .expect("22 positive seed assertions must check from client import");
    for name in [
        "axis_lower_in",
        "axis_upper_in",
        "axis_below_out",
        "axis_above_out",
        "axis_inverted_out",
        "contains_in",
        "contains_out",
        "intersect_corners",
        "intersect_disjoint_corners",
        "empty_meet_excludes_lower_input_corner",
        "empty_meet_excludes_upper_input_corner",
        "only_first_inside",
        "only_first_outside_second",
        "only_first_outside_intersection",
        "hull_corners",
        "distance_value",
        "seed_law_intersect_complete",
        "seed_law_intersect_sound",
        "seed_law_hull_left",
        "seed_law_hull_right",
        "seed_law_manhattan_symm",
        "seed_law_manhattan_self",
    ] {
        let id = env.globals[name];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must be a checked seed assertion"
        );
    }
}

// Append exactly one false proposition to the same previously checked
// positive client; never count an unrelated parse or import failure as a
// semantic refusal. These five variants are seed-nat-box.md's negatives.
const REJECT_ONE_BOX_ONLY_POINT: &str = r##"
-- Reject: point in only b02 cannot be in their intersection.
theorem reject_one_box_only_point :
  IsTrue (contains (intersect b02 b13) only_first) = Proved
"##;

const REJECT_FABRICATED_OTHER_PREMISE: &str = r##"
-- Reject: Proved cannot inhabit the missing b13 membership premise.
theorem reject_fabricated_other_premise :
  IsTrue (contains (intersect b02 b13) only_first) =
  intersect_complete b02 b13 only_first Proved Proved
"##;

const REJECT_POINT_IN_EMPTY_MEET: &str = r##"
-- Reject: an empty intersection contains no input corner.
theorem reject_point_in_empty_meet :
  IsTrue (contains (intersect b01 b23) (MkPoint n1 n1)) = Proved
"##;

const REJECT_WRONG_HULL_CORNERS: &str = r##"
-- Reject: wrong lower corner, unchanged upper corner.
theorem reject_wrong_hull_corners :
  Eq Box (hull b01 b23)
    (MkBox (MkPoint n1 n1) (MkPoint n3 n3)) = Refl
"##;

const REJECT_WRONG_DISTANCE: &str = r##"
-- Reject: one less than the five required by two nonzero axes.
theorem reject_wrong_distance :
  Equal Nat (manhattan (MkPoint n1 n4) (MkPoint n3 n1)) n4 = Proved
"##;

/// SOURCE: conformance/stdlib/geometry/seed-nat-box.md five refusals.
/// PROMISE CLASS: durable invariant.
/// MEASURED: with all 22 positive assertions already checked, each false
/// seed theorem is refused at its own proof term by the specified typed
/// mismatch (the hull's false Refl at the elaboration boundary).
/// CLAIMED: one-box membership, fabricated premises, empty membership,
/// wrong hull corners and a wrong exact distance cannot be proved.
/// THE GAP: these five witnesses cover their fixed cases; generic truths
/// belong to the six separately checked package laws.
#[test]
fn five_false_seed_propositions_are_refused_after_positive_control() {
    for (bad, label, refl) in [
        (REJECT_ONE_BOX_ONLY_POINT, "one-box intersection", false),
        (
            REJECT_FABRICATED_OTHER_PREMISE,
            "fabricated other premise",
            false,
        ),
        (
            REJECT_POINT_IN_EMPTY_MEET,
            "empty intersection member",
            false,
        ),
        (REJECT_WRONG_HULL_CORNERS, "incorrect hull corner", true),
        (REJECT_WRONG_DISTANCE, "incorrect Manhattan distance", false),
    ] {
        let mut env = geometry_env();
        env.elaborate_file(SEED_POSITIVE)
            .expect("same positive client must check before each refusal");
        match env.elaborate_file(bad) {
            Err(ElabError::TypeMismatch { span, .. }) if refl => assert_eq!(
                bad.get(span.start..span.end),
                Some("Refl"),
                "{label}: refusal must target the false proof"
            ),
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) if !refl => {
                let declaration_start = bad.find("theorem reject_").expect("negative theorem");
                assert!(
                    span.start >= declaration_start
                        && span.end <= bad.len()
                        && span.start < span.end,
                    "{label}: kernel rejection must target this false theorem, got {span:?}"
                );
            }
            other => panic!("{label}: expected semantic proof rejection, got {other:?}"),
        }
    }
}

/// SOURCE: spec/50-stdlib/67-geometry.md §3.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after explicit providers, loading NatBox leaves the exact
/// trusted GlobalId set unchanged. The operation and seven laws are owned,
/// transparent, and used by generic clients; `axis_within` stays private.
/// CLAIMED: the §3 corner and witnessed-inclusion theory adds checked
/// public contracts without enlarging the trusted base or helper surface.
/// THE GAP: finite seed values and a false claim are measured separately.
#[test]
fn section_three_generic_containment_laws_add_no_trust() {
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
        .expect("NatBox §3 checked bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§3 trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§3 adds no trusted GlobalId");
    for public in [
        "inside",
        "inside_sound",
        "inside_refl",
        "intersect_inside_left",
        "intersect_inside_right",
        "inside_hull_left",
        "inside_hull_right",
        "inside_complete",
    ] {
        let id = env.globals[&format!("{GEOMETRY}.{public}")];
        assert!(owned.contains(&id), "{public} must belong to NatBox");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{public} must have a checked body"
        );
    }
    env.elaborate_file(CONTAINMENT_GENERIC_CLIENT)
        .expect("seven generic §3 law clients consume their exact public propositions");
    match env.elaborate_file(&format!("import {GEOMETRY} (axis_within)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{GEOMETRY}.axis_within"));
        }
        other => panic!("per-axis helper must remain private: {other:?}"),
    }
}

const CONTAINMENT_GENERIC_CLIENT: &str = r#"
import Algorithm.Geometry.NatBox
  (Point, Box, contains, intersect, hull, inside, inside_sound,
   inside_refl, intersect_inside_left, intersect_inside_right,
   inside_hull_left, inside_hull_right, inside_complete)

theorem client_inside_sound (b : Box) (c : Box) (p : Point)
    (i : IsTrue (inside b c)) (h : IsTrue (contains b p)) :
  IsTrue (contains c p) = inside_sound b c p i h

theorem client_inside_refl (b : Box) :
  IsTrue (inside b b) = inside_refl b

theorem client_intersect_inside_left (b : Box) (c : Box) :
  IsTrue (inside (intersect b c) b) = intersect_inside_left b c

theorem client_intersect_inside_right (b : Box) (c : Box) :
  IsTrue (inside (intersect b c) c) = intersect_inside_right b c

theorem client_inside_hull_left (b : Box) (c : Box) :
  IsTrue (inside b (hull b c)) = inside_hull_left b c

theorem client_inside_hull_right (b : Box) (c : Box) :
  IsTrue (inside c (hull b c)) = inside_hull_right b c

theorem client_inside_complete (b : Box) (c : Box) (q : Point)
    (w : IsTrue (contains b q))
    (all : (p : Point) → IsTrue (contains b p) → IsTrue (contains c p)) :
  IsTrue (inside b c) = inside_complete b c q w all
"#;

/// SOURCE: conformance/stdlib/geometry/seed-box-containment.md positives.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the actual positive seed fences elaborate from the published
/// package, including nested, overlap, hull, inverted and empty cases and
/// applications of all seven generic checked laws.
/// CLAIMED: the public interface computes the finite corner observations
/// and clients may use the proofs on the specified example boxes.
/// THE GAP: finite oracles do not replace generic proofs checked above.
#[test]
fn shipped_box_containment_seed_positive_cases_check() {
    let mut sources = containment_sources();
    sources
        .pop()
        .expect("separate inward-containment refusal fence");
    assert!(!sources.is_empty(), "seed needs positive source fences");
    let mut env = geometry_env();
    for positive in sources {
        env.elaborate_file(&positive)
            .expect("every shipped positive containment declaration checks");
    }
}

// A closed false `IsTrue` claim need not distinguish this specific boundary
// from an unrelated false claim. Compare proposition trees ignoring spans and
// formatting, deriving the negative from a checked False observation whose
// factor order is reversed in the fixture's checked positive controls.
fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unsupported closed containment proposition: {other:?}"),
    }
}

fn application<'a>(ty: &'a SourceType, name: &str) -> Option<Vec<&'a SourceType>> {
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

fn inside_pair(ty: &SourceType) -> Option<(&SourceType, &SourceType)> {
    let args = application(ty, "inside")?;
    let [first, second] = args.as_slice() else {
        return None;
    };
    Some((first, second))
}

fn inside_claim(ty: &SourceType) -> Option<(&SourceType, &SourceType)> {
    let args = application(ty, "IsTrue")?;
    let [inside] = args.as_slice() else {
        return None;
    };
    inside_pair(inside)
}

fn assert_strictly_smaller_refusal_goal(positive: &str, shared: &str, refused: &str) {
    let positive_decls = parse_decls(positive).expect("positive containment fixture parses");
    let false_inside = positive_decls
        .iter()
        .filter_map(|decl| {
            let SourceDecl::TheoremDecl { theorem, .. } = decl else {
                return None;
            };
            let equality = application(theorem, "Equal")?;
            let [carrier, observed, value] = equality.as_slice() else {
                return None;
            };
            if !application(carrier, "Bool").is_some_and(|args| args.is_empty())
                || !application(value, "False").is_some_and(|args| args.is_empty())
            {
                return None;
            }
            let (inner, small) = inside_pair(observed)?;
            Some((*observed, inner, small))
        })
        .collect::<Vec<_>>();
    let controls = parse_decls(shared).expect("same-box negative controls parse");
    let positive_pairs = controls
        .iter()
        .filter_map(|decl| {
            let SourceDecl::TheoremDecl { theorem, .. } = decl else {
                return None;
            };
            inside_claim(theorem)
        })
        .collect::<Vec<_>>();
    let holds = |a: &SourceType, b: &SourceType| {
        positive_pairs.iter().any(|(left, right)| {
            type_shape(left) == type_shape(a) && type_shape(right) == type_shape(b)
        })
    };
    let matching = false_inside
        .iter()
        .filter(|(_, inner, small)| {
            holds(inner, inner)
                && holds(small, inner)
                && positive_pairs.iter().any(|(left, outer)| {
                    type_shape(left) == type_shape(inner)
                        && type_shape(outer) != type_shape(inner)
                        && type_shape(outer) != type_shape(small)
                        && holds(small, outer)
                })
        })
        .collect::<Vec<_>>();
    let [(observed, _, _)] = matching.as_slice() else {
        panic!("one false inward relation must share boxes with four checked controls");
    };
    let negative_decls = parse_decls(refused).expect("refused theorem parses");
    let [SourceDecl::TheoremDecl {
        theorem: negative_goal,
        body: Expr::ECon(proof, _),
        ..
    }] = negative_decls.as_slice()
    else {
        panic!("separate refused block must be one final theorem with a proof");
    };
    assert_eq!(proof, "Proved", "negative claim must attempt its proof");
    let term = &positive[observed.span().start..observed.span().end];
    let expected = format!("theorem expected_refusal : IsTrue ({term}) = Proved");
    let expected_decls = parse_decls(&expected).expect("derived inward claim parses");
    let [SourceDecl::TheoremDecl {
        theorem: expected_goal,
        ..
    }] = expected_decls.as_slice()
    else {
        panic!("derived inward claim is one theorem");
    };
    assert_eq!(
        type_shape(negative_goal),
        type_shape(expected_goal),
        "refusal must assert the false inside relation on the controlled boxes"
    );
}

/// SOURCE: conformance/stdlib/geometry/seed-box-containment.md refusal.
/// PROMISE CLASS: durable invariant.
/// MEASURED: all shipped positives and four same-pair controls check first;
/// the final theorem states the known false `inside` proposition derived
/// from checked value and reverse-order controls. Its own `Proved` receives
/// `KernelRejected(TypeMismatch)`, not an earlier parse or fixture error.
/// CLAIMED: the inner box does not fit within the smaller point-box, though
/// the smaller one fits within the inner and both lie in the outer.
/// THE GAP: this fixed pair does not prove every false corner comparison.
#[test]
fn shipped_strictly_smaller_box_refuses_on_its_controlled_pair() {
    let mut sources = containment_sources();
    let negative = sources
        .pop()
        .expect("separate inward-containment refusal fence");
    let first_positive = sources
        .first()
        .expect("shipped containment positive fixture");
    let declarations = parse_decls(&negative).expect("negative fence parses");
    let Some(SourceDecl::TheoremDecl { span, .. }) = declarations.last() else {
        panic!("negative fence ends in false inside theorem");
    };
    let (shared, refused) = negative.split_at(span.start);
    let mut env = geometry_env();
    for positive in &sources {
        env.elaborate_file(positive)
            .expect("all shipped positives check first");
    }
    env.elaborate_file(shared)
        .expect("four same-pair controls check first");
    assert_strictly_smaller_refusal_goal(first_positive, shared, refused);
    match env.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start < span.end && span.end <= refused.len() => {}
        other => panic!("false inward containment must fail at its own theorem: {other:?}"),
    }
}
