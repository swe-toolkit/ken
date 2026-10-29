//! Graph monotonicity: spec/50-stdlib/62-graphs.md §7.
//! SOURCE: conformance/stdlib/graphs/seed-reachability-graph-monotone.md.
//! PROMISE CLASS: durable invariant. Generic public law, checked finite
//! inclusion, exact edge tables, and a same-form false converse are separate.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const GRAPH: &str = "Algorithm.Graphs.Dependency";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/graphs/seed-reachability-graph-monotone.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_graph() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[root()], GRAPH)
        .expect("§7 graph package roots-loads");
    env
}

// Select whole clients by checked theorem identity, not Markdown fence order.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§7 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        if parse_decls(source)
            .expect("§7 seed source parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            )
        {
            assert!(
                selected.replace(source).is_none(),
                "duplicate {theorem_name} client"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing §7 seed theorem {theorem_name}"))
}

const GENERIC_CLIENT: &str = r#"
import Data.Finite.Finite (Finite)
import Algorithm.Graphs.Dependency
  (Graph, edge, graph_reachable, graph_reachable_graph_monotone)

theorem graph_inclusion_client
    (q : Type) (fq : Finite q) (g : Graph q) (h : Graph q)
    (s : q) (target : q → Bool)
    (included : (u : q) → (v : q)
      → Equal Bool (edge q g u v) True
      → Equal Bool (edge q h u v) True)
  : Equal Bool (graph_reachable q fq g s target) True
    → Equal Bool (graph_reachable q fq h s target) True =
  graph_reachable_graph_monotone q fq g h s target included

theorem same_graph_client
    (q : Type) (fq : Finite q) (g : Graph q)
    (s : q) (target : q → Bool)
  : Equal Bool (graph_reachable q fq g s target) True
    → Equal Bool (graph_reachable q fq g s target) True =
  graph_reachable_graph_monotone q fq g g s target (λu. λv. λe. e)
"#;

/// MEASURED: exact provider/package trusted GlobalId identity sets agree;
/// the package owns the Transparent public law, an arbitrary-carrier client
/// applies its precise type and same-graph boundary, and private walk/search
/// helpers refuse import. CLAIMED: §7 publishes one checked, trust-free
/// graph-monotonicity law, not a new public walk operation. THE GAP: provider
/// trust is inherited and the finite seed cannot prove the universal law.
#[test]
fn generic_graph_monotonicity_is_owned_checked_private_and_trust_free() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    for provider in [
        "Core.Logic.Conjunction",
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
        .expect("§7 graph monotonicity checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§7 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "§7 adds no trusted GlobalId identities");
    let id = env.globals[&format!("{GRAPH}.graph_reachable_graph_monotone")];
    assert!(owned.contains(&id), "new law belongs to Dependency");
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    env.elaborate_file(GENERIC_CLIENT)
        .expect("generic law and same-graph public clients check");
    for helper in ["walk_lift", "require_some", "walk_reachable"] {
        match env.elaborate_file(&format!("import {GRAPH} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GRAPH}.{helper}"));
            }
            other => panic!("{helper} must remain private: {other:?}"),
        }
    }
}

// The fixed finite fixture is specified by its edge tables, not just the
// positive reachability observations: a direct V0→V2 edge would be a shortcut,
// and a second h-only edge would no longer be a one-edge extension.
fn edge_table_controls() -> String {
    let mut controls = String::new();
    for (index, from, to, in_chain, in_looped) in [
        (0, "V0", "V0", "False", "False"),
        (1, "V0", "V1", "True", "True"),
        (2, "V0", "V2", "False", "False"),
        (3, "V1", "V0", "False", "False"),
        (4, "V1", "V1", "False", "False"),
        (5, "V1", "V2", "True", "True"),
        (6, "V2", "V0", "False", "True"),
        (7, "V2", "V1", "False", "False"),
        (8, "V2", "V2", "False", "False"),
    ] {
        controls.push_str(&format!(
            "theorem chain_edge_{index} : Equal Bool (edge Vertex3 chain {from} {to}) {in_chain} = Proved\n"
        ));
        controls.push_str(&format!(
            "theorem looped_edge_{index} : Equal Bool (edge Vertex3 looped {from} {to}) {in_looped} = Proved\n"
        ));
    }
    controls
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected §7 proposition type: {other:?}"),
    }
}

fn expr_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::ECon(name, _) => format!("constructor:{name}"),
        SourceExpr::EVar(name, _) => format!("variable:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!("({} {})", expr_shape(function), expr_shape(argument))
        }
        other => panic!("unexpected §7 law application: {other:?}"),
    }
}

fn expected_theorem(proposition: &str, proof: &str) -> (String, String) {
    let parsed = parse_decls(&format!("theorem expected : {proposition} = {proof}"))
        .expect("independent §7 proposition parses");
    let [SourceDecl::TheoremDecl { theorem, body, .. }] = parsed.as_slice() else {
        panic!("expected one theorem");
    };
    (type_shape(theorem), expr_shape(body))
}

/// MEASURED: the complete accepted seed checks its inclusion proof and seven
/// closed Bool Proved values. Independently checked edge tables pin the real
/// V0→V1→V2 path, absence of a V0→V2 shortcut, and exactly one new h-only
/// edge V2→V0. The same result in h is both computed and proved by the public
/// law on g. The isolated same-form converse is AST-pinned, well typed and
/// refused at its own Proved after a fresh complete accepted prefix.
/// CLAIMED: edge inclusion preserves positive reachability, not conversely.
/// THE GAP: finite tables/refusal do not prove the generic theorem; the
/// arbitrary-carrier client and checked body supply that separate evidence.
#[test]
fn complete_two_edge_seed_checks_and_false_converse_refuses() {
    let accepted = seed_case("chain_v0_reaches_v2");
    let refused = seed_case("seed_neg");
    let mut positive = loaded_graph();
    positive
        .elaborate_file(accepted)
        .expect("complete accepted §7 seed checks");
    positive
        .elaborate_file(&edge_table_controls())
        .expect("the actual seed graphs have exactly the specified edge tables");
    let parsed = parse_decls(accepted).expect("complete accepted §7 program parses");
    for (name, proposition, proof) in [
        ("added_edge_absent", "Equal Bool (edge Vertex3 chain V2 V0) False", "Proved"),
        ("added_edge_present", "Equal Bool (edge Vertex3 looped V2 V0) True", "Proved"),
        ("start_misses_target", "Equal Bool (is_v2 V0) False", "Proved"),
        ("chain_v0_reaches_v2", "Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 is_v2) True", "Proved"),
        ("looped_v0_reaches_v2_value", "Equal Bool (graph_reachable Vertex3 vertex3_finite looped V0 is_v2) True", "Proved"),
        ("looped_v0_reaches_v2_by_law", "Equal Bool (graph_reachable Vertex3 vertex3_finite looped V0 is_v2) True", "graph_reachable_graph_monotone Vertex3 vertex3_finite chain looped V0 is_v2 chain_inside_looped chain_v0_reaches_v2"),
        ("looped_v2_reaches_v0", "Equal Bool (graph_reachable Vertex3 vertex3_finite looped V2 is_v0) True", "Proved"),
        ("chain_v2_misses_v0", "Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 is_v0) False", "Proved"),
    ] {
        let matches = parsed.iter().filter(|decl| match decl {
            SourceDecl::TheoremDecl { name: actual, params, theorem, body, .. }
                if actual == name && params.is_empty() =>
                    (type_shape(theorem), expr_shape(body)) == expected_theorem(proposition, proof),
            _ => false,
        }).count();
        assert_eq!(matches, 1, "accepted §7 control {name} retains its checked claim");
        let id = positive.globals[name];
        assert!(matches!(positive.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    let rejected = parse_decls(refused).expect("isolated §7 converse parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        theorem,
        body: SourceExpr::ECon(proof, _),
        span: claim_span,
        ..
    }] = rejected.as_slice()
    else {
        panic!("isolated converse is one closed Proved theorem");
    };
    assert_eq!(name, "seed_neg");
    assert!(params.is_empty(), "converse claim is closed");
    assert_eq!(proof, "Proved");
    let false_goal = "Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 is_v0) True";
    assert_eq!(
        type_shape(theorem),
        expected_theorem(false_goal, "Proved").0,
        "the refused claim must target reachability in the smaller graph at V2"
    );
    let mut typing = loaded_graph();
    typing
        .elaborate_file(accepted)
        .expect("accepted prefix checks before false proposition typing");
    typing
        .elaborate_file(&format!(
            "theorem typed_refusal (p : {false_goal}) : {false_goal} = p"
        ))
        .expect("false converse is well typed when assumed");
    let mut rejecting = loaded_graph();
    rejecting
        .elaborate_file(accepted)
        .expect("fresh complete accepted prefix checks");
    match rejecting.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start >= claim_span.start
            && span.end <= claim_span.end
            && span.start < span.end => {}
        other => panic!("false converse must reject at its own Proved: {other:?}"),
    }
}
