//! Reachability antisymmetry under a dependency order: spec 62 §12.
//! Promise class: durable invariant. The generic law and the three closed
//! conformance rows are checked against the real Dependency module.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr, KenMdFenceRole,
};
use ken_kernel::Decl;

const GRAPH: &str = "Algorithm.Graphs.Dependency";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/graphs/seed-reachability-antisymmetric.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn graph_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], GRAPH)
        .expect("§12 Dependency roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Core.Classes.LawfulClasses (DecEq)
import Data.Finite.Finite (Finite)
import Algorithm.Graphs.Dependency
  (Graph, TopoOrder, graph_reachable, graph_reachable_antisymmetric)
theorem client_antisym
      (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q)
      (order : TopoOrder q d g) (s : q) (t : q)
    : Equal Bool (graph_reachable q fq g s (d.eq t)) True
      → Equal Bool (graph_reachable q fq g t (d.eq s)) True
      → Equal q s t =
  graph_reachable_antisymmetric q fq d g order s t
"#;

/// MEASURED: the full provider-first trusted GlobalId sets agree, the law
/// belongs to Dependency as an importable Transparent declaration, its
/// private walk helper cannot be imported, and an open client type-checks.
/// CLAIMED: §12 exports the checked generic law without new trust or helper
/// leakage. THE GAP: a generic proof alone does not validate a closed graph.
#[test]
fn generic_law_is_public_checked_and_adds_no_trusted_identity() {
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
        .expect("§12 graph package checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(
        before, after,
        "no trusted GlobalId identities are added or removed"
    );

    let id = env.globals[&format!("{GRAPH}.graph_reachable_antisymmetric")];
    assert!(owned.contains(&id), "the public law is owned by Dependency");
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    env.elaborate_file(&format!(
        "import {GRAPH} (graph_reachable_antisymmetric as client_law)"
    ))
    .expect("law selectively imports");
    match env.elaborate_file(&format!("import {GRAPH} (walks_antisymmetric)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{GRAPH}.walks_antisymmetric"));
        }
        other => panic!("private walk helper cannot be imported: {other:?}"),
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("public law checks over open q, fq, d, g, order, s, and t");
}

struct SeedCases<'a> {
    accepted: &'a str,
    law_refusal: &'a str,
    order_only_refusal: &'a str,
}

// Select whole Ken programs by parsed declaration identity, not fence order
// or the prose surrounding a fence.
fn seed_cases() -> SeedCases<'static> {
    let mut accepted = None;
    let mut law_refusal = None;
    let mut order_only_refusal = None;
    for fence in extract_ken_md(SEED)
        .expect("§12 seed extracts")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let decls = parse_decls(source).expect("§12 Ken source parses");
        let has_accepted = decls.iter().any(|decl| {
            matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == "v1_does_not_reach_v0")
        });
        let has_law_refusal = decls.iter().any(|decl| {
            matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == "v1_does_not_reach_v0_cycle2")
        });
        let has_cycle_order = decls.iter().any(
            |decl| matches!(decl, SourceDecl::ViewDecl { name, .. } if name == "cycle2_order"),
        );
        if has_accepted {
            assert!(accepted.replace(source).is_none(), "one accepted case");
        } else if has_law_refusal {
            assert!(has_cycle_order, "law refusal includes the cycle order");
            assert!(law_refusal.replace(source).is_none(), "one law refusal");
        } else if has_cycle_order {
            assert!(
                order_only_refusal.replace(source).is_none(),
                "one order-only refusal"
            );
        }
    }
    SeedCases {
        accepted: accepted.expect("missing accepted chain client"),
        law_refusal: law_refusal.expect("missing two-cycle law refusal"),
        order_only_refusal: order_only_refusal.expect("missing two-cycle order-only refusal"),
    }
}

fn refers_to_law(body: &Expr) -> bool {
    match body {
        Expr::EVar(name, _) => name == "graph_reachable_antisymmetric",
        Expr::EApp(function, argument, _) => refers_to_law(function) || refers_to_law(argument),
        Expr::ELam(_, inner, _) => refers_to_law(inner),
        Expr::EAsc(inner, _, _) => refers_to_law(inner),
        _ => false,
    }
}

/// MEASURED: the real chain client proves its forward leg, then uses the
/// public law to turn an assumed reverse leg into Bottom; the same graph
/// independently checks that the reverse reachability decision is False.
/// CLAIMED: the accepted witness exercises antisymmetry's force rather than
/// just its diagonal case. THE GAP: this finite graph does not prove the law.
#[test]
fn one_way_chain_checks_forward_leg_and_refutes_putative_back_leg() {
    let cases = seed_cases();
    let decls = parse_decls(cases.accepted).expect("accepted chain parses");
    let uses_law = decls.iter().any(|decl| {
        matches!(decl, SourceDecl::TheoremDecl { name, body, .. }
            if name == "v1_does_not_reach_v0" && refers_to_law(body))
    });
    assert!(
        uses_law,
        "the accepted back-leg proof applies the public law"
    );
    let mut env = graph_env();
    env.elaborate_file(cases.accepted)
        .expect("ordered one-way chain, real forward leg, and law-derived contradiction check");
    env.elaborate_file(
        "theorem reverse_chain_value : Equal Bool \
         (graph_reachable Vertex3 vertex3_finite chain V1 (vertex3_eq V0)) False = Proved",
    )
    .expect("the reverse decision is independently False on the ordered chain");
}

/// MEASURED: after a fresh complete accepted client, the same law-derived
/// claim on a two-cycle refuses as KernelRejected; an independent fresh
/// client refuses the same cycle's order without any law use. CLAIMED: the
/// back edge makes the order unconstructible rather than the law malformed.
/// THE GAP: these are package elaborations, not official conformance runs.
#[test]
fn two_cycle_refuses_both_law_row_and_order_only_row() {
    let cases = seed_cases();
    let order_only_decls = parse_decls(cases.order_only_refusal).expect("order-only row parses");
    assert!(
        order_only_decls
            .iter()
            .all(|decl| !matches!(decl, SourceDecl::TheoremDecl { .. })),
        "order-only refusal contains no law theorem"
    );
    let law_decls = parse_decls(cases.law_refusal).expect("two-cycle law row parses");
    let forward_leg_start = law_decls
        .iter()
        .find_map(|decl| match decl {
            SourceDecl::TheoremDecl { name, span, .. } if name == "v0_reaches_v1_cycle2" => {
                Some(span.start)
            }
            _ => None,
        })
        .expect("law row includes its forward leg after cycle2_order");
    let mut matching_order_only = graph_env();
    matching_order_only
        .elaborate_file(cases.accepted)
        .expect("fresh valid chain before law-row order-only prefix");
    match matching_order_only.elaborate_file(&cases.law_refusal[..forward_leg_start]) {
        Err(ElabError::KernelRejected { .. }) => {}
        other => panic!("the law row's own cycle order must refuse before any law use: {other:?}"),
    }
    for (case_name, source) in [
        ("law row", cases.law_refusal),
        ("order-only row", cases.order_only_refusal),
    ] {
        let mut env = graph_env();
        env.elaborate_file(cases.accepted)
            .expect("fresh complete chain client checks before the two-cycle");
        match env.elaborate_file(source) {
            Err(ElabError::KernelRejected { .. }) => {}
            other => panic!("{case_name} must refuse as KernelRejected: {other:?}"),
        }
    }
}
