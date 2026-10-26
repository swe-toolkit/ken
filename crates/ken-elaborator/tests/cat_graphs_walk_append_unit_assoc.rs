//! Checked left unit and associativity of graph walks: spec/50-stdlib/62-graphs.md §4.
//! Promise class: durable invariant. The generic checked laws establish open
//! walks; the closed seed discriminates equal routes from different routes.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::Type as SourceType;
use ken_elaborator::{extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::Decl;

const GRAPH: &str = "Algorithm.Graphs.Dependency";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/graphs/seed-walk-append-unit-assoc.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn graph_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GRAPH)
        .expect("checked graph exports roots-load");
    env
}

fn seed_sources() -> [&'static str; 2] {
    let sources: Vec<_> = extract_ken_md(SEED)
        .expect("§4 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| &SEED[fence.body_range])
        .collect();
    sources
        .try_into()
        .unwrap_or_else(|_| panic!("§4 seed has one accepted and one refused client"))
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Graphs.Dependency
  (Graph, Walk, WalkHere, walk_append, walk_append_here_left, walk_append_assoc)

theorem client_walk_append_here_left
    (q : Type) (g : Graph q) (s : q) (u : q) (walk : Walk q g s u) :
  Equal (Walk q g s u)
    (walk_append q g s s u (WalkHere q g s) walk) walk =
  walk_append_here_left q g s u walk

theorem client_walk_append_assoc
    (q : Type) (g : Graph q) (s : q) (t : q) (u : q) (v : q)
    (first : Walk q g s t) (second : Walk q g t u)
    (third : Walk q g u v) :
  Equal (Walk q g s v)
    (walk_append q g s u v (walk_append q g s t u first second) third)
    (walk_append q g s t v first (walk_append q g t u v second third)) =
  walk_append_assoc q g s t u v first second third
"#;

/// SOURCE: spec/50-stdlib/62-graphs.md §4.
/// PROMISE CLASS: durable invariant.
/// MEASURED: provider imports precede the package, exact trusted GlobalIds do
/// not change on package load, both new owners are Transparent, and an
/// independent client consumes the specified generic proposition types.
/// CLAIMED: both public laws are kernel-checked for open walks with no new
/// assumption. THE GAP: the inherited providers retain their prior trust;
/// concrete edge-order discrimination requires the separate seed client.
#[test]
fn generic_walk_laws_are_owned_transparent_and_zero_trust_delta() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Data.Finite.Finite",
        "Core.Logic.Transport",
        "Core.Classes.LawfulClasses",
        "Data.Collections.Derived",
        "Core.Logic.Or",
        "Data.Collections.List",
        "Data.Numeric.Nat.Order",
        "Data.Sums.Combinators",
        "Algorithm.FormalLanguages.Dfa",
        "Algorithm.FormalLanguages.Reachability",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], GRAPH)
        .expect("§4 graph laws kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§4 trusted_base() count: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§4 adds no trusted GlobalId identities");

    for name in ["walk_append_here_left", "walk_append_assoc"] {
        let id = env.globals[&format!("{GRAPH}.{name}")];
        assert!(owned.contains(&id), "{name} is graph-package-owned");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent client consumes both generic §4 types");
}

// Compare Ken proposition ASTs, never Markdown positions or source spelling.
// The expected goals are specified independently of the seed's own claims.
fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("con:{name}"),
        SourceType::TVar(name, _) => format!("var:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("expected an applied closed walk proposition: {other:?}"),
    }
}

fn theorem_goal_shape(source: &str, theorem_name: &str) -> String {
    let decls = parse_decls(source).expect("Ken client parses");
    let goal = decls
        .iter()
        .find_map(|decl| match decl {
            SourceDecl::TheoremDecl { name, theorem, .. } if name == theorem_name => Some(theorem),
            _ => None,
        })
        .unwrap_or_else(|| panic!("expected checked theorem {theorem_name}"));
    type_shape(goal)
}

const EXPECTED_SAME_ROUTE: &str = r#"
theorem expected_same_route :
  Equal (Walk Vertex4 diamond D0 D3)
    (walk_append Vertex4 diamond D0 D1 D3 walk_01 walk_13)
    (WalkStep Vertex4 diamond D0 D1 D3 Proved walk_01) = Refl
"#;

const EXPECTED_OTHER_ROUTE: &str = r#"
theorem expected_other_route :
  Equal (Walk Vertex4 diamond D0 D3)
    (walk_append Vertex4 diamond D0 D1 D3 walk_01 walk_13)
    (walk_append Vertex4 diamond D0 D2 D3 walk_02 walk_23) = Refl
"#;

/// SOURCE: conformance/stdlib/graphs/seed-walk-append-unit-assoc.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the entire accepted diamond client checks first, including the
/// left unit, associativity with WalkHere in all three positions, an
/// independent closed Refl, and an explicit same-input D1 route. The
/// separately parsed refusal has that same D1 input and the specified D2
/// output; its Refl alone receives the exact Eq-goal TypeMismatch.
/// CLAIMED: the laws preserve checked edge order rather than equating any
/// walks with the same endpoints. THE GAP: this one diamond cannot replace
/// the generic proof or exclude every possible wrong route.
#[test]
fn seed_accepts_same_route_and_refuses_other_route_at_its_claim() {
    let [accepted, refused] = seed_sources();
    assert_eq!(
        theorem_goal_shape(accepted, "joined_via_d1_has_same_route"),
        theorem_goal_shape(EXPECTED_SAME_ROUTE, "expected_same_route"),
        "the accepted control observes this very joined D1 route"
    );
    assert_eq!(
        theorem_goal_shape(refused, "other_route_is_same_walk"),
        theorem_goal_shape(EXPECTED_OTHER_ROUTE, "expected_other_route"),
        "the refused claim changes only the route to D2, not its input"
    );
    let declarations = parse_decls(refused).expect("isolated refusal parses");
    let [SourceDecl::TheoremDecl { name, body, .. }] = declarations.as_slice() else {
        panic!("refused fence contains only the other-route theorem");
    };
    assert_eq!(name, "other_route_is_same_walk");
    let body_span = body.span();
    let mut env = graph_env();
    env.elaborate_file(accepted)
        .expect("complete §4 accepted client checks against shipped exports");
    match env.elaborate_file(refused) {
        Err(ElabError::TypeMismatch { span, reason })
            if reason == "Refl expects an `Eq`-shaped goal"
                && body_span.start <= span.start
                && span.start < span.end
                && span.end <= body_span.end => {}
        other => panic!("other-route proposition must reject at Refl: {other:?}"),
    }
}
