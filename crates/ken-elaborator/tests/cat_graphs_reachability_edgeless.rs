//! Edgeless graph reachability: spec/50-stdlib/62-graphs.md §23.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const GRAPH: &str = "Algorithm.Graphs.Dependency";

fn loaded_graph() -> (ElabEnv, GlobalId) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    let roots = &[root];
    let mut env = ElabEnv::new().expect("compiler base");
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
        env.elaborate_module_from_roots(roots, provider)
            .unwrap_or_else(|error| panic!("provider {provider}: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(roots, GRAPH)
        .expect("Dependency root-loads after the providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "Dependency trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert!(!before.is_empty(), "the provider baseline is non-vacuous");
    assert_eq!(
        after, before,
        "Dependency adds no trusted GlobalId identities"
    );

    let law = env.globals[&format!("{GRAPH}.graph_reachable_edgeless")];
    assert!(
        owned.contains(&law),
        "Dependency owns graph_reachable_edgeless"
    );
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    (env, law)
}

fn cites(body: &Term, law: GlobalId) -> bool {
    matches!(body, Term::Const { id, .. } if *id == law)
        || body.children().into_iter().any(|child| cites(child, law))
}

const PREFIX: &str = r#"import Data.Finite.Finite (Finite, bool_finite)

import Algorithm.Graphs.Dependency
  (Graph, MkGraph, graph_edgeless, graph_reachable, graph_reachable_edgeless)

theorem gen_edgeless (q : Type) (fq : Finite q) (s : q) (target : q → Bool)
  : Equal Bool (graph_reachable q fq (graph_edgeless q) s target) (target s) =
  graph_reachable_edgeless q fq s target

fn is_true (v : Bool) : Bool = v

const complete_bool : Graph Bool = MkGraph Bool (λu. λv. True)

theorem complete_reaches_true
  : Equal Bool (graph_reachable Bool bool_finite complete_bool False is_true) True =
  Proved

theorem edgeless_misses_true
  : Equal Bool (graph_reachable Bool bool_finite (graph_edgeless Bool) False is_true) False =
  Proved
"#;

const FROM_FALSE: &str = r#"theorem edgeless_from_false
  : Equal Bool (graph_reachable Bool bool_finite (graph_edgeless Bool) False is_true) False =
  graph_reachable_edgeless Bool bool_finite False is_true
"#;

const WRONG_FROM_FALSE: &str = r#"theorem edgeless_from_false
  : Equal Bool (graph_reachable Bool bool_finite (graph_edgeless Bool) False is_true) True =
  graph_reachable_edgeless Bool bool_finite False is_true
"#;

const FROM_TRUE: &str = r#"const edgeless : Graph Bool = MkGraph Bool (λu. λv. False)

theorem named_edgeless_from_true
  : Equal Bool (graph_reachable Bool bool_finite edgeless True is_true) True =
  graph_reachable_edgeless Bool bool_finite True is_true
"#;

const CONVERSION_CLIENTS: &str = r#"import Core.Classes.LawfulClasses (DecEq)
import Algorithm.Graphs.Dependency (edge, graph_from_successors)

theorem open_edgeless_edge (q : Type) (s : q) (t : q)
  : Equal Bool (edge q (graph_edgeless q) s t) False = Proved

theorem edgeless_literal_target
  : Equal Bool (graph_reachable Bool bool_finite (graph_edgeless Bool) False is_true) False =
  graph_reachable_edgeless Bool bool_finite False (λv. v)

fn no_successors (q : Type) (v : q) : List q = Nil q

theorem edgeless_from_empty_successors
    (q : Type) (fq : Finite q) (d : DecEq q) (s : q) (target : q → Bool)
  : Equal Bool (graph_reachable q fq (graph_from_successors q d (no_successors q)) s target)
      (target s) =
  graph_reachable_edgeless q fq s target
"#;

/// SOURCE: graphs §23; the accepted seed and the open conversion controls.
/// PROMISE CLASS: durable invariant.
/// MEASURED: accepted theorems are checked by identity, law applications cite
/// the owned Transparent proof, provider trust is unchanged, and a selective
/// import of the checked helper refuses.
/// CLAIMED: edgeless reachability is the start's target test, not always False;
/// the constructor form works by conversion, without new trust or helper API.
/// THE GAP: finite examples distinguish decisions; the generic checked body
/// proves the law beyond these examples. Provider assumptions remain inherited.
#[test]
fn edgeless_clients_use_the_owned_law_without_new_trust() {
    let (mut env, law) = loaded_graph();
    let accepted = format!("{PREFIX}\n{FROM_FALSE}\n{FROM_TRUE}\n{CONVERSION_CLIENTS}");
    let declared = env
        .elaborate_file(&accepted)
        .expect("accepted clients check");
    for (name, by_law) in [
        ("gen_edgeless", true),
        ("complete_reaches_true", false),
        ("edgeless_misses_true", false),
        ("edgeless_from_false", true),
        ("named_edgeless_from_true", true),
        ("open_edgeless_edge", false),
        ("edgeless_literal_target", true),
        ("edgeless_from_empty_successors", true),
    ] {
        let id = env.globals[name];
        assert!(declared.contains(&id), "the client itself declares {name}");
        let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) else {
            panic!("{name} must have a checked Transparent body");
        };
        assert_eq!(cites(body, law), by_law, "{name}: checked law citation");
    }

    match env.elaborate_file(&format!("import {GRAPH} (edgeless_walk_start)")) {
        Err(ElabError::UnboundName { name, .. })
            if name == format!("{GRAPH}.edgeless_walk_start") => {}
        other => panic!("private edgeless helper must refuse selective import: {other:?}"),
    }
}

/// SOURCE: graphs §23; the seed's same-form False/True pair.
/// PROMISE CLASS: durable invariant with exact values scoped to this fixture.
/// MEASURED: the full shared prefix checks; changing only the by-law result
/// from False to True refuses in the appended declaration.
/// CLAIMED: the edgeless graph cannot reuse the complete graph's True result.
/// THE GAP: the refusal span identifies the declaration, not an internal
/// proof step. The separate True-start client guards against always-False.
#[test]
fn edgeless_reachability_refuses_the_complete_graph_result() {
    let (mut env, _) = loaded_graph();
    env.elaborate_file(PREFIX)
        .expect("fresh accepted prefix checks");
    let decls = parse_decls(WRONG_FROM_FALSE).expect("refused client parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = decls.as_slice()
    else {
        panic!("the refused client must be exactly one theorem");
    };
    assert_eq!(name, "edgeless_from_false");
    match env.elaborate_file(WRONG_FROM_FALSE) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim.start <= span.start && span.start < span.end && span.end <= claim.end => {
            eprintln!("{name}: KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("edgeless True claim must kernel-refuse at its own theorem: {other:?}"),
    }
}
