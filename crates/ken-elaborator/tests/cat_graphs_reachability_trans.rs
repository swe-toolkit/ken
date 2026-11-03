//! Graph reachability transitivity: spec/50-stdlib/62-graphs.md §11.
//! Promise class: durable invariant. The open client checks the public law
//! independently of the closed directed-chain seed.

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
    "/../../conformance/stdlib/graphs/seed-reachability-trans.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn graph_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[root()], GRAPH)
        .expect("§11 Graphs Dependency roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Data.Finite.Finite (Finite)
import Core.Classes.LawfulClasses (DecEq)
import Algorithm.Graphs.Dependency
  (Graph, WalkHere, graph_reachable, graph_reachable_complete,
   graph_reachable_trans)

theorem client_trans
    (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q)
    (s : q) (t : q) (target : q → Bool)
  : Equal Bool (graph_reachable q fq g s (d.eq t)) True
    → Equal Bool (graph_reachable q fq g t target) True
    → Equal Bool (graph_reachable q fq g s target) True =
  graph_reachable_trans q fq d g s t target

theorem client_refl
    (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) (s : q)
  : Equal Bool (graph_reachable q fq g s (d.eq s)) True =
  graph_reachable_complete q fq g s s (d.eq s)
    (WalkHere q g s) (d.complete s s Refl)

theorem client_refl_then_trans
    (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q)
    (s : q) (target : q → Bool)
  : Equal Bool (graph_reachable q fq g s target) True
    → Equal Bool (graph_reachable q fq g s target) True =
  graph_reachable_trans q fq d g s s target (client_refl q fq d g s)
"#;

/// MEASURED: all ten providers root-load before Dependency; complete trusted
/// GlobalId identity sets agree; the new law is Dependency-owned, Transparent
/// and importable by an open client. Reflexivity and its composition check
/// from public evidence without adding an export, while five private graph
/// helpers refuse import. CLAIMED: §11 publishes one checked transitivity law
/// without local trust or a helper leak. THE GAP: inherited provider trust and
/// independent closed graph behavior need separate measurements.
#[test]
fn generic_transitivity_is_owned_public_checked_and_zero_trust_delta() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
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
        .expect("§11 Graphs package checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§11 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(before, after, "§11 adds no trusted GlobalId identities");
    let public_laws = [
        "walk_append_here_left",
        "walk_append_assoc",
        "graph_reachable_complete",
        "graph_find_walk_some",
        "graph_reachable_along_walk",
        "graph_reachable_target_monotone",
        "graph_reachable_graph_monotone",
        "graph_reachable_trans",
        "graph_reverse_edge",
        "graph_reachable_reverse",
        "walk_reverse_append",
        "graph_reverse_involutive",
        "walk_reverse_involutive",
        "ordered_complete",
        "ordered_unique",
        "ordered_forward",
        "before_irreflexive",
        "before_transitive",
        "cycle_refutes_order",
        "ordered_walk_forward",
        "contains_sound",
        "contains_complete",
    ];
    assert_eq!(public_laws.len(), 22, "§11 exports the twenty-second law");
    env.elaborate_file(&format!("import {GRAPH} ({})", public_laws.join(", ")))
        .expect("all twenty-two named laws remain publicly importable");
    for law in public_laws {
        let id = env.globals[&format!("{GRAPH}.{law}")];
        assert!(owned.contains(&id), "{law} belongs to Dependency");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("generic transitivity, derived reflexivity, and composition check");
    for private in [
        "require_some",
        "walk_transport_end",
        "named",
        "graph_reachable_refl",
        "edge_reach_trans",
    ] {
        match env.elaborate_file(&format!("import {GRAPH} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GRAPH}.{private}"));
            }
            other => panic!("{private} remains private: {other:?}"),
        }
    }
}

// Select complete Ken clients by their named law uses, not by Markdown order.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§11 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        if parse_decls(source)
            .expect("§11 seed source parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            )
        {
            assert!(
                selected.replace(source).is_none(),
                "one seed case {theorem_name}"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing §11 seed case {theorem_name}"))
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("expected a closed §11 proposition: {other:?}"),
    }
}

fn proof_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::ECon(name, _) => format!("constructor:{name}"),
        SourceExpr::EVar(name, _) => format!("variable:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!("({} {})", proof_shape(function), proof_shape(argument))
        }
        other => panic!("expected a closed §11 proof application: {other:?}"),
    }
}

fn expected_row(goal: &str, proof: &str) -> (String, String) {
    let parsed = parse_decls(&format!("theorem expected : {goal} = {proof}"))
        .expect("independent closed §11 row parses");
    let [SourceDecl::TheoremDecl { theorem, body, .. }] = parsed.as_slice() else {
        panic!("expected one theorem");
    };
    (type_shape(theorem), proof_shape(body))
}

/// MEASURED: the complete CV seed first leg, second leg, public-law client,
/// same-form Proved control, and reversed False value all check against the
/// directed chain. The named claims retain independent closed goal and proof
/// shapes. After a fresh complete accepted prefix the reversed True claim is
/// well typed as an assumption but KernelRejected TypeMismatch at its own
/// Proved. CLAIMED: composition travels forward, not backward, on the seed
/// graph. THE GAP: these finite rows are package-test elaboration, not an
/// official conformance-runner execution or a substitute for the open proof.
#[test]
fn complete_seed_composes_forward_and_reverse_claim_refuses() {
    let accepted = seed_case("v0_reaches_v2_trans");
    let refused = seed_case("refuse_v2_reaches_v0");
    let parsed = parse_decls(accepted).expect("accepted §11 seed parses");
    for (name, goal, proof) in [
        (
            "v0_reaches_v1",
            "Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 (vertex3_eq V1)) True",
            "Proved",
        ),
        (
            "v1_reaches_v2",
            "Equal Bool (graph_reachable Vertex3 vertex3_finite chain V1 is_v2) True",
            "Proved",
        ),
        (
            "v0_reaches_v2_trans",
            "Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 is_v2) True",
            "graph_reachable_trans Vertex3 vertex3_finite DecEq_instance_Vertex3 chain V0 V1 is_v2 v0_reaches_v1 v1_reaches_v2",
        ),
        (
            "v0_reaches_v2_same_form_control",
            "Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 is_v2) True",
            "Proved",
        ),
        (
            "v2_misses_v0",
            "Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 is_v0) False",
            "Proved",
        ),
    ] {
        let matches = parsed.iter().filter(|decl| match decl {
            SourceDecl::TheoremDecl { name: actual, params, theorem, body, .. }
                if actual == name && params.is_empty() =>
                    (type_shape(theorem), proof_shape(body)) == expected_row(goal, proof),
            _ => false,
        }).count();
        assert_eq!(matches, 1, "seed law/control {name} retains its proposition and proof");
    }
    let mut positive = graph_env();
    positive
        .elaborate_file(accepted)
        .expect("all five §11 seed rows, including the public law, check");
    let mut edge_table = format!("import {GRAPH} (edge)\n");
    for (i, from, to, expected) in [
        (0, "V0", "V0", "False"),
        (1, "V0", "V1", "True"),
        (2, "V0", "V2", "False"),
        (3, "V1", "V0", "False"),
        (4, "V1", "V1", "False"),
        (5, "V1", "V2", "True"),
        (6, "V2", "V0", "False"),
        (7, "V2", "V1", "False"),
        (8, "V2", "V2", "False"),
    ] {
        edge_table.push_str(&format!(
            "theorem chain_edge_{i} : Equal Bool (edge Vertex3 chain {from} {to}) {expected} = Proved\n"
        ));
    }
    positive
        .elaborate_file(&edge_table)
        .expect("the complete nine-edge table remains the one-way V0→V1→V2 chain");
    let rejected = parse_decls(refused).expect("isolated §11 refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        theorem,
        body: SourceExpr::ECon(proof, _),
        span: claim_span,
        ..
    }] = rejected.as_slice()
    else {
        panic!("isolated §11 refusal is exactly one closed Proved claim");
    };
    assert_eq!(name, "refuse_v2_reaches_v0");
    assert!(params.is_empty());
    assert_eq!(proof, "Proved");
    let false_goal = "Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 is_v0) True";
    assert_eq!(type_shape(theorem), expected_row(false_goal, "Proved").0);
    let mut typing = graph_env();
    typing
        .elaborate_file(accepted)
        .expect("the complete accepted fixture checks before refusal typing");
    typing
        .elaborate_file(&format!(
            "theorem assumed_reverse (p : {false_goal}) : {false_goal} = p"
        ))
        .expect("the false claim is well typed as an assumption, not a malformed goal");
    let mut rejecting = graph_env();
    rejecting
        .elaborate_file(accepted)
        .expect("fresh complete seed prefix checks before refusal");
    match rejecting.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim_span.start <= span.start
            && span.start < span.end
            && span.end <= claim_span.end => {}
        other => panic!("false reverse Proved must reject at its own theorem: {other:?}"),
    }
}
