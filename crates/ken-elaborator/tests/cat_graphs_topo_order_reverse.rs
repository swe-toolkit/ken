//! Spec: spec/50-stdlib/62-graphs.md §21; seed: seed-topo-order-reverse.md.
//! The seed rows exercise the package, not the official conformance runner.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{extract_ken_md, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::{Decl, GlobalId, Term};

const GRAPH: &str = "Algorithm.Graphs.Dependency";
const DERIVED: &str = "Data.Collections.Derived";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/graphs/seed-topo-order-reverse.md"
));

fn load(env: &mut ElabEnv, module: &str) -> Vec<GlobalId> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    env.elaborate_module_from_roots(&[root], module)
        .unwrap_or_else(|error| panic!("{module} roots-loads: {error:?}"))
}

fn graph_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    load(&mut env, GRAPH);
    env
}

fn seed_sources() -> [&'static str; 3] {
    extract_ken_md(SEED)
        .expect("§21 seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| &SEED[fence.body_range])
        .collect::<Vec<_>>()
        .try_into()
        .expect("shared prefix and two isolated rows")
}

fn contains_const(term: &Term, target: GlobalId) -> bool {
    matches!(term, Term::Const { id, .. } if *id == target)
        || term
            .children()
            .into_iter()
            .any(|child| contains_const(child, target))
}

/// Promise class: durable invariant. MEASURED: provider-relative trusted
/// identity sets agree at both load steps. Arbitrary clients use the exact
/// package-owned Transparent declarations. Added checked APIs are compatible;
/// additional trust, ownership changes, or extra premises break this promise.
/// GAP: inherited trust and all possible clients are not certified here.
#[test]
fn open_transfer_and_laws_are_owned_transparent_and_add_no_trust() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    for provider in [
        "Core.Function.Combinators",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
        "Core.Logic.Compare",
        "Core.Classes.LawfulClasses",
        "Core.Logic.And",
        "Core.Logic.Conjunction",
        "Core.Logic.Or",
        "Core.Logic.OrdResult",
        "Core.Logic.Transport",
        "Data.Collections.List",
    ] {
        load(&mut env, provider);
    }
    let trusted = |env: &ElabEnv| env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let before_derived = trusted(&env);
    let derived_owned = load(&mut env, DERIVED);
    let after_derived = trusted(&env);
    eprintln!(
        "Derived trusted GlobalIds: {} → {}",
        before_derived.len(),
        after_derived.len()
    );
    assert_eq!(
        after_derived, before_derived,
        "Derived adds no trusted GlobalIds"
    );

    for provider in [
        "Data.Finite.Finite",
        "Data.Sums.Combinators",
        "Algorithm.FormalLanguages.Dfa",
        "Algorithm.FormalLanguages.Reachability",
    ] {
        load(&mut env, provider);
    }
    let before_graph = trusted(&env);
    let graph_owned = load(&mut env, GRAPH);
    let after_graph = trusted(&env);
    eprintln!(
        "Dependency trusted GlobalIds: {} → {}",
        before_graph.len(),
        after_graph.len()
    );
    assert_eq!(
        after_graph, before_graph,
        "Dependency adds no trusted GlobalIds"
    );

    let [prefix, _, _] = seed_sources();
    let clients = env
        .elaborate_file(prefix)
        .expect("generic and closed seed prefix checks");
    for (module, owned, export, client) in [
        (DERIVED, &derived_owned, "reverse::perm", "perm_open"),
        (
            GRAPH,
            &graph_owned,
            "topo_order_reverse",
            "topo_order_reverse_open",
        ),
        (
            GRAPH,
            &graph_owned,
            "topo_order_reverse_vertices",
            "topo_order_reverse_vertices_open",
        ),
    ] {
        let id = env.globals[&format!("{module}.{export}")];
        assert!(owned.contains(&id), "{module} owns {export}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{export} is Transparent"
        );
        let client_id = env.globals[client];
        assert!(clients.contains(&client_id));
        let Some(Decl::Transparent { body, .. }) = env.env.lookup(client_id) else {
            panic!("{client} must have a checked body");
        };
        assert!(
            contains_const(body, id),
            "{client} cites the exact {export}"
        );
    }
}

/// Promise class: durable invariant. MEASURED: the same explicit law
/// application accepts the reversed list; claiming the original list fails
/// at its own closing Refl. Compatible: more valid orders or clients.
/// Breaking: accepting the unchanged list or rejecting the true claim.
/// GAP: the Boolean fixture does not establish the universal law by itself.
#[test]
fn seed_accepts_reversed_vertices_and_refuses_the_original_list() {
    let [prefix, accepted, refused] = seed_sources();
    let mut positive = graph_env();
    positive
        .elaborate_file(prefix)
        .expect("shared seed prefix checks");
    positive
        .elaborate_file(accepted)
        .expect("reversed list checks through the public law");

    let mut negative = graph_env();
    negative
        .elaborate_file(prefix)
        .expect("fresh accepted prefix before refusal");
    let refl = refused.rfind("Refl").expect("the closing proof is Refl");
    match negative.elaborate_file(refused) {
        Err(ElabError::TypeMismatch { span, .. }) => {
            assert_eq!(
                span.start..span.end,
                refl..refl + "Refl".len(),
                "must reach the closing proof"
            );
            eprintln!("original list refuses at closing Refl: {span:?}");
        }
        other => panic!("original list must refuse at the closing Refl: {other:?}"),
    }
}

/// Promise class: durable invariant. The implementation may replace its
/// private proof steps, but selective imports must not expose them.
#[test]
fn reversal_scaffolding_is_not_selectively_importable() {
    let mut env = graph_env();
    for private in [
        "before_reverse_flip",
        "reverse_flip_step",
        "before_append_found",
        "append_found_step",
        "before_append_left",
        "append_left_step",
        "contains_reverse",
    ] {
        match env.elaborate_file(&format!("import {GRAPH} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GRAPH}.{private}"))
            }
            other => panic!("{private} must refuse selective import: {other:?}"),
        }
    }
}

/// Promise class: durable invariant. On a duplicated endpoint, raw `before`
/// cannot be flipped by reversal alone. Distinct-list extensions are
/// compatible; changing this first-occurrence behavior breaks the witness.
#[test]
fn duplicate_endpoint_shows_why_precedence_flip_needs_distinctness() {
    graph_env().elaborate_file(r#"
import Core.Classes.LawfulClasses (DecEq_instance_Bool)
import Data.Collections.Derived as DC
import Algorithm.Graphs.Dependency (before, no_duplicates)

const duplicate_vertices : List Bool = Cons Bool True (Cons Bool False (Cons Bool True (Nil Bool)))

theorem duplicate_source_before
    : Equal Bool (before Bool DecEq_instance_Bool True False duplicate_vertices) True = Proved

theorem duplicate_reversed_not_before
    : Equal Bool (before Bool DecEq_instance_Bool False True (DC.reverse Bool duplicate_vertices)) False = Proved

theorem duplicate_not_unique
    : Equal Bool (no_duplicates Bool DecEq_instance_Bool duplicate_vertices) False = Proved
"#).expect("the first-occurrence counterexample and failed uniqueness premise check");
}
