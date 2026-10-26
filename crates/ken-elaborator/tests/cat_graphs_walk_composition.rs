//! Checked walk composition and forward order: spec/50-stdlib/62-graphs.md §3.
//! Promise class: durable invariant. The shipped §3 seed supplies closed
//! clients; independent generic clients consume both package exports.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const GRAPH: &str = "Algorithm.Graphs.Dependency";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/graphs/seed-walk-composition-order.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn graph_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GRAPH)
        .expect("checked graph exports and providers roots-load");
    env
}

fn seed_sources() -> Vec<&'static str> {
    extract_ken_md(SEED)
        .expect("§3 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| &SEED[fence.body_range])
        .collect()
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Graphs.Dependency
  (Graph, edge, Walk, WalkHere, TopoOrder, before, ordered_vertices,
   walk_append, ordered_walk_forward)
import Core.Classes.LawfulClasses (DecEq)

fn client_append
  (q : Type) (g : Graph q) (s : q) (t : q) (u : q)
  (first : Walk q g s t) (second : Walk q g t u) : Walk q g s u =
  walk_append q g s t u first second

theorem client_right_walk_here
  (q : Type) (g : Graph q) (s : q) (t : q) (first : Walk q g s t) :
  Equal (Walk q g s t)
    (walk_append q g s t t first (WalkHere q g t)) first = Refl

theorem client_ordered_walk_forward
  (q : Type) (d : DecEq q) (g : Graph q)
  (order : TopoOrder q d g)
  (s : q) (next : q) (t : q)
  (first : Equal Bool (edge q g s next) True)
  (rest : Walk q g next t) :
  Equal Bool
    (before q d s t (ordered_vertices q d g order)) True =
  ordered_walk_forward q d g order s next t first rest
"#;

/// SOURCE: spec/50-stdlib/62-graphs.md §3.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the two package-owned transparent exports root-load, their exact
/// generic signatures check in a separate client, and an open walk retains
/// right WalkHere by reduction. The existing walk_before helper refuses
/// selective import; the package adds no trusted GlobalId to its provider set.
/// CLAIMED: §3's generic construction and proof are checked with no new trust.
/// THE GAP: inherited provider trust persists; the closed chain behavior needs
/// the independent seed clients below.
#[test]
fn generic_walk_exports_are_checked_private_and_trust_free() {
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
        .expect("§3 graph operation and proof kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§3 trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§3 adds no trusted GlobalId identities");

    for name in [
        "walk_append",
        "walk_append_here_left",
        "walk_append_assoc",
        "ordered_walk_forward",
    ] {
        let id = env.globals[&format!("{GRAPH}.{name}")];
        assert!(owned.contains(&id), "{name} belongs to graph package");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent client consumes both generic §3 types");
    match env.elaborate_file(&format!("import {GRAPH} (walk_before)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{GRAPH}.walk_before"));
        }
        other => panic!("walk_before must remain private: {other:?}"),
    }
}

/// SOURCE: conformance/stdlib/graphs/seed-walk-composition-order.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the shared three-vertex fixture checks before the two shipped
/// positive theorems: the explicit two-edge constructor result and the
/// ordered-walk direction for the same chain.
/// CLAIMED: both concrete §3 observations hold for checked paths.
/// THE GAP: a three-vertex instance does not prove the generic laws; their
/// independently consumed types are covered by the preceding test.
#[test]
fn shipped_walk_seed_two_positive_fences_check() {
    let sources = seed_sources();
    let [fixture, composition, forward, _refused] = sources.as_slice() else {
        panic!("§3 seed must retain a fixture, two positives and one refusal");
    };
    let mut env = graph_env();
    for (source, label) in [
        (*fixture, "three-vertex chain fixture"),
        (*composition, "two-edge composition"),
        (*forward, "forward-order same-walk control"),
    ] {
        env.elaborate_file(source)
            .unwrap_or_else(|error| panic!("{label} checks: {error:?}"));
    }
}

fn applied_type<'a>(ty: &'a SourceType, name: &str) -> Option<Vec<&'a SourceType>> {
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

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unsupported closed order proposition: {other:?}"),
    }
}

fn expr_shape(expr: &Expr) -> String {
    match expr {
        Expr::ECon(name, _) => format!("constructor:{name}"),
        Expr::EVar(name, _) => format!("variable:{name}"),
        Expr::EApp(function, argument, _) => {
            format!("({} {})", expr_shape(function), expr_shape(argument))
        }
        other => panic!("unsupported same-walk proof term: {other:?}"),
    }
}

fn proposition_shape(proposition: &str) -> String {
    let source = format!("theorem expected_order_relation : {proposition} = Refl");
    let decls = parse_decls(&source).expect("derived order relation parses");
    let [SourceDecl::TheoremDecl { theorem, .. }] = decls.as_slice() else {
        panic!("derived order relation is one theorem");
    };
    type_shape(theorem)
}

// A wrong closed order proposition normalizes to Bottom just as an unrelated
// false proposition does. Derive the reversed claim from the checked control's
// own graph/order/start/end, rather than treating any refusal as sufficient.
fn assert_paired_walk_goals(forward: &str, refused: &str) -> String {
    let positive_decls = parse_decls(forward).expect("forward-order control parses");
    let [SourceDecl::TheoremDecl {
        theorem: positive_goal,
        body: positive_term,
        ..
    }] = positive_decls.as_slice()
    else {
        panic!("forward control is exactly one theorem");
    };
    let equality = applied_type(positive_goal, "Equal").expect("control states Equal");
    let [carrier, lhs, truth] = equality.as_slice() else {
        panic!("control has one carrier and two endpoints");
    };
    assert_eq!(type_shape(carrier), "variable:Bool");
    assert_eq!(type_shape(truth), "variable:True");
    let before = applied_type(lhs, "before").expect("control observes public order");
    let [q, dec, start, end, vertices] = before.as_slice() else {
        panic!("control's before has its carrier, dictionary, endpoints and list");
    };
    assert_ne!(
        type_shape(start),
        type_shape(end),
        "the pair is nondegenerate"
    );
    let projection = applied_type(vertices, "ordered_vertices")
        .expect("control uses the checked order projection");
    assert_eq!(
        projection.len(),
        4,
        "one shared order, not a second fixture"
    );
    assert_eq!(type_shape(q), type_shape(projection[0]));
    assert_eq!(type_shape(dec), type_shape(projection[1]));
    let snippet = |ty: &SourceType| &forward[ty.span().start..ty.span().end];
    let relation = |left: &SourceType, right: &SourceType, result: &str| {
        format!(
            "Equal Bool (before ({}) ({}) ({}) ({}) ({})) {result}",
            snippet(q),
            snippet(dec),
            snippet(left),
            snippet(right),
            snippet(vertices)
        )
    };
    assert_eq!(
        type_shape(positive_goal),
        proposition_shape(&relation(start, end, "True")),
        "accepting control states the forward proposition"
    );
    let negative_decls = parse_decls(refused).expect("refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        theorem: negative_goal,
        body: negative_term,
        ..
    }] = negative_decls.as_slice()
    else {
        panic!("refusal is exactly one attempted theorem");
    };
    assert_eq!(name, "refuse_reversed_order_on_same_chain_walk");
    assert_eq!(
        expr_shape(negative_term),
        expr_shape(positive_term),
        "refusal must apply the same law to the same walk and order"
    );
    assert_eq!(
        type_shape(negative_goal),
        proposition_shape(&relation(end, start, "True")),
        "refusal must reverse only the control's two endpoints"
    );
    format!(
        "theorem reversed_is_false : {} = Proved",
        relation(end, start, "False")
    )
}

/// SOURCE: conformance/stdlib/graphs/seed-walk-composition-order.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: both shipped positives check before the isolated reversed
/// theorem; its AST keeps the same law term and exactly swaps the paired
/// order endpoints. The reversed observation checks as False and the
/// attempted True type refuses as TypeMismatch.
/// CLAIMED: the checked forward law cannot prove the reverse order on this
/// very graph, order, edge and remaining walk.
/// THE GAP: this fixed chain does not by itself prove the generic theorem.
#[test]
fn shipped_reversed_order_refuses_after_same_walk_control() {
    let sources = seed_sources();
    let [fixture, composition, forward, refused] = sources.as_slice() else {
        panic!("§3 seed must retain a fixture, two positives and one refusal");
    };
    let reverse_false = assert_paired_walk_goals(forward, refused);
    let mut env = graph_env();
    for source in [fixture, composition, forward] {
        env.elaborate_file(source)
            .expect("fixture and both positives must check first");
    }
    env.elaborate_file(&reverse_false)
        .expect("same order has the independently derived reverse direction False");
    match env.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start < span.end && span.end <= refused.len() => {}
        other => panic!("same-walk reversed proposition must refuse: {other:?}"),
    }
}
