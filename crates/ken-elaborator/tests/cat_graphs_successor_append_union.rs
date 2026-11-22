//! Appending successor lists agrees edgewise with graph union: graphs §20.
//! The public open law and both finite discriminator orders are checked here;
//! neither a finite seed nor a trusted-count comparison substitutes for the
//! provider-first exact GlobalId trust gate.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const GRAPH: &str = "Algorithm.Graphs.Dependency";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load_providers(env: &mut ElabEnv) {
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
            .unwrap_or_else(|error| panic!("provider {provider} roots-loads: {error:?}"));
    }
}

fn loaded_graph() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    load_providers(&mut env);
    env.elaborate_module_from_roots(&[root()], GRAPH)
        .expect("checked Dependency root-loads");
    env
}

fn contains_const(body: &Term, owner: GlobalId) -> bool {
    let mut pending = vec![body];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

const OPEN_CLIENT: &str = r#"import Core.Classes.LawfulClasses (DecEq)
import Data.Collections.Derived (list_append)
import Algorithm.Graphs.Dependency
  (edge, graph_union, graph_from_successors,
   successors_append, graph_from_successors_append)

theorem open_append
      (q : Type) (d : DecEq q) (f : q → List q) (g : q → List q) (s : q) (t : q)
    : Equal Bool
        (edge q (graph_from_successors q d (successors_append q f g)) s t)
        (edge
          q
          (graph_union q (graph_from_successors q d f) (graph_from_successors q d g))
          s
          t) =
  graph_from_successors_append q d f g s t

fn open_appended_list (q : Type) (f : q → List q) (g : q → List q) (v : q)
    : List q = successors_append q f g v
"#;

const PREFIX: &str = r#"import Core.Classes.LawfulClasses (DecEq_instance_Bool)
import Algorithm.Graphs.Dependency
  (edge, graph_union, graph_from_successors,
   successors_append, graph_from_successors_append)

fn none_out (v : Bool) : List Bool = Nil Bool
fn true_to_false (v : Bool) : List Bool =
  match v {
    True ↦ Cons Bool False (Nil Bool);
    False ↦ Nil Bool
  }

theorem appended_edge_true : Equal Bool
    (edge Bool
      (graph_from_successors Bool DecEq_instance_Bool
        (successors_append Bool none_out true_to_false))
      True False)
    True = Proved

theorem left_edge_false : Equal Bool
    (edge Bool (graph_from_successors Bool DecEq_instance_Bool none_out) True False)
    False = Proved

theorem right_edge_true : Equal Bool
    (edge Bool (graph_from_successors Bool DecEq_instance_Bool true_to_false) True False)
    True = Proved
"#;

const SEED_UNION: &str = r#"theorem seed_union
    : Equal Bool
        (edge Bool
          (graph_from_successors Bool DecEq_instance_Bool
            (successors_append Bool none_out true_to_false))
          True False)
        (edge Bool
          (graph_union Bool
            (graph_from_successors Bool DecEq_instance_Bool none_out)
            (graph_from_successors Bool DecEq_instance_Bool true_to_false))
          True False) =
  graph_from_successors_append Bool DecEq_instance_Bool none_out true_to_false True False
"#;

const SEED_UNION_SWAPPED: &str = r#"theorem seed_union_swapped
    : Equal Bool
        (edge Bool
          (graph_from_successors Bool DecEq_instance_Bool
            (successors_append Bool true_to_false none_out))
          True False)
        (edge Bool
          (graph_union Bool
            (graph_from_successors Bool DecEq_instance_Bool true_to_false)
            (graph_from_successors Bool DecEq_instance_Bool none_out))
          True False) =
  graph_from_successors_append Bool DecEq_instance_Bool true_to_false none_out True False
"#;

const SEED_LEFT_ONLY: &str = r#"theorem seed_left_only
    : Equal Bool
        (edge Bool
          (graph_from_successors Bool DecEq_instance_Bool
            (successors_append Bool none_out true_to_false))
          True False)
        (edge Bool
          (graph_from_successors Bool DecEq_instance_Bool none_out)
          True False) =
  graph_from_successors_append Bool DecEq_instance_Bool none_out true_to_false True False
"#;

const SEED_RIGHT_ONLY: &str = r#"theorem seed_right_only
    : Equal Bool
        (edge Bool
          (graph_from_successors Bool DecEq_instance_Bool
            (successors_append Bool true_to_false none_out))
          True False)
        (edge Bool
          (graph_from_successors Bool DecEq_instance_Bool none_out)
          True False) =
  graph_from_successors_append Bool DecEq_instance_Bool true_to_false none_out True False
"#;

/// SOURCE: Architect AC-0 §2, §6; forthcoming spec/50-stdlib/62-graphs.md §20.
/// PROMISE CLASS: durable invariant.
/// MEASURED: all ten providers are preloaded; exact trusted GlobalId sets
/// before/after Dependency agree; its two new names are package-owned and
/// Transparent, and selective open clients cite those exact owner identities.
/// Both private helpers refuse selective import from the checked module.
/// CLAIMED: successor-list append and its edge law are public, checked and
/// add no local trust or public proof helper.
/// THE GAP: inherited provider assumptions are not removed; the closed rows
/// below separately exercise the Boolean graph's non-degenerate edges.
#[test]
fn open_law_and_function_are_owned_public_trust_neutral_and_helpers_private() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    load_providers(&mut env);
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], GRAPH)
        .expect("Dependency root-loads after all providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "successor append trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert!(!before.is_empty(), "provider baseline must be non-vacuous");
    assert_eq!(after, before, "Dependency adds no trusted GlobalId");
    for name in ["successors_append", "graph_from_successors_append"] {
        let id = env.globals[&format!("{GRAPH}.{name}")];
        assert!(owned.contains(&id), "Dependency owns {name}");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(OPEN_CLIENT)
        .expect("open operation and its generic law check by selective import");
    for (client, law) in [
        ("open_append", "graph_from_successors_append"),
        ("open_appended_list", "successors_append"),
    ] {
        let (_, body) = env
            .env
            .transparent_body(env.globals[client])
            .expect("checked client");
        assert!(
            contains_const(&body, env.globals[&format!("{GRAPH}.{law}")]),
            "{client} cites the exact checked {law} owner"
        );
    }
    for helper in ["contains_append", "bool_or_regroup"] {
        let id = env.globals[&format!("{GRAPH}.{helper}")];
        assert!(owned.contains(&id), "Dependency owns private {helper}");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        match env.elaborate_file(&format!("import {GRAPH} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GRAPH}.{helper}"));
            }
            other => panic!("{helper} must remain private: {other:?}"),
        }
    }
}

/// SOURCE: Architect AC-0 §6; forthcoming graphs §20 and conformance seed.
/// PROMISE CLASS: durable invariant (closed Bool values are fixture controls).
/// MEASURED: the appended edge is True, the empty-list operand edge False,
/// the listing operand edge True, and both swapped-order applications of the
/// checked public law elaborate from a shared value-checked prefix.
/// CLAIMED: the edge law composes both successor lists in either order.
/// THE GAP: the finite True/False values do not replace the generic checked
/// public law; the open client above checks its generic type separately.
#[test]
fn appended_successor_edges_and_both_operand_orders_check() {
    let mut env = loaded_graph();
    env.elaborate_file(PREFIX)
        .expect("three independently observed edge values check");
    env.elaborate_file(SEED_UNION)
        .expect("public law relates appended lists to their graph union");
    env.elaborate_file(SEED_UNION_SWAPPED)
        .expect("public law also relates operands in reversed order");
}

/// SOURCE: Architect AC-0 §6; forthcoming graphs §20 and conformance seed.
/// PROMISE CLASS: durable invariant.
/// MEASURED: each refusal begins at a fresh checked prefix with exact edge
/// values and its same-form full-union application accepted first. Replacing
/// the union by the empty-list operand alone rejects as KernelRejected
/// TypeMismatch inside the entire added declaration, for both positions.
/// CLAIMED: neither operand alone captures the added graph's edge at the
/// distinguishing source/target, even when its position is exchanged.
/// THE GAP: declaration-wide spans do not isolate a subexpression; the
/// independent opposite-value controls attribute these paired refusals.
#[test]
fn each_operand_alone_refuses_after_fresh_opposite_edge_controls() {
    for (accepted, refused, name) in [
        (SEED_UNION, SEED_LEFT_ONLY, "seed_left_only"),
        (SEED_UNION_SWAPPED, SEED_RIGHT_ONLY, "seed_right_only"),
    ] {
        let parsed = parse_decls(refused).expect("one rejected claim parses");
        let [SourceDecl::TheoremDecl {
            name: parsed_name,
            span: claim,
            ..
        }] = parsed.as_slice()
        else {
            panic!("each refusal contains one theorem");
        };
        assert_eq!(parsed_name, name);
        let mut env = loaded_graph();
        env.elaborate_file(PREFIX)
            .expect("fresh accepted three-value prefix");
        env.elaborate_file(accepted)
            .expect("full-union sibling checks before its operand-alone refusal");
        match env
            .elaborate_file(refused)
            .expect_err("operand-alone claim must reject")
        {
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            } if span.start >= claim.start && span.end <= claim.end && span.start < span.end => {}
            other => panic!("{name} must reject in its own declaration: {other:?}"),
        }
    }
}
