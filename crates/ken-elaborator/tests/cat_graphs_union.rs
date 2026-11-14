//! Checked graph union: spec/50-stdlib/62-graphs.md §15.
//! The open client checks the published proof types; the closed graph
//! distinguishes a union from either input and from invented/omitted edges.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError, Expr, Type as SourceType};
use ken_kernel::{Decl, KernelError};

const GRAPH: &str = "Algorithm.Graphs.Dependency";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_graph() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[root()], GRAPH)
        .expect("Graph Dependency and its checked proofs root-load");
    env
}

const GENERIC_CLIENT: &str = r###"import Data.Finite.Finite (Finite)
import Core.Logic.Or (Or, Inl, Inr)
import Core.Classes.LawfulClasses (bool_or, or_left, or_right)
import Algorithm.Graphs.Dependency
  (Graph, edge, graph_union, graph_union_edge, graph_union_edge_left,
   graph_union_edge_right, graph_union_edge_cases,
   graph_union_reachable_left, graph_union_reachable_right, graph_reachable)

fn generic_union (q : Type) (g : Graph q) (h : Graph q) : Graph q = graph_union q g h

theorem generic_edge (q : Type) (g : Graph q) (h : Graph q) (s : q) (t : q) :
  Equal Bool (edge q (graph_union q g h) s t)
    (bool_or (edge q g s t) (edge q h s t)) = graph_union_edge q g h s t

theorem generic_left (q : Type) (g : Graph q) (h : Graph q) (s : q) (t : q) :
  Equal Bool (edge q g s t) True → Equal Bool (edge q (graph_union q g h) s t) True =
  graph_union_edge_left q g h s t

theorem generic_right (q : Type) (g : Graph q) (h : Graph q) (s : q) (t : q) :
  Equal Bool (edge q h s t) True → Equal Bool (edge q (graph_union q g h) s t) True =
  graph_union_edge_right q g h s t

fn generic_cases (q : Type) (g : Graph q) (h : Graph q) (s : q) (t : q) :
  Equal Bool (edge q (graph_union q g h) s t) True
    → Or (Equal Bool (edge q g s t) True) (Equal Bool (edge q h s t) True) =
  graph_union_edge_cases q g h s t

theorem generic_cases_into_omega (q : Type) (g : Graph q) (h : Graph q) (s : q) (t : q)
    (present : Equal Bool (edge q (graph_union q g h) s t) True) :
  Equal Bool (bool_or (edge q g s t) (edge q h s t)) True =
  match graph_union_edge_cases q g h s t present {
    Inl from_g ↦ or_left (edge q g s t) (edge q h s t) from_g;
    Inr from_h ↦ or_right (edge q g s t) (edge q h s t) from_h
  }

theorem generic_reachable_left
    (q : Type) (fq : Finite q) (g : Graph q) (h : Graph q) (s : q) (target : q → Bool) :
  Equal Bool (graph_reachable q fq g s target) True
    → Equal Bool (graph_reachable q fq (graph_union q g h) s target) True =
  graph_union_reachable_left q fq g h s target

theorem generic_reachable_right
    (q : Type) (fq : Finite q) (g : Graph q) (h : Graph q) (s : q) (target : q → Bool) :
  Equal Bool (graph_reachable q fq h s target) True
    → Equal Bool (graph_reachable q fq (graph_union q g h) s target) True =
  graph_union_reachable_right q fq g h s target
"###;

/// SOURCE: spec 62 §15; PROMISE CLASS: durable invariant.
/// MEASURED: after every provider is loaded, Graph Dependency adds no
/// trusted GlobalId; each of the seven names is owned and Transparent;
/// an independent open client imports all seven, consumes the five public
/// theorem proofs and the Type-sorted Or result, and eliminates Or into Omega.
/// A selective import of the private Bool match refuses at its qualified name.
/// CLAIMED: graph union and its generic evidence are public, checked and local-
/// trust-free without leaking the internal Bool split. THE GAP: generic
/// instantiation does not establish the closed graph's edge values below.
#[test]
fn generic_union_is_public_checked_and_zero_local_trust() {
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
            .unwrap_or_else(|error| panic!("{provider}: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], GRAPH)
        .expect("Graph Dependency root-loads after its providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after, before, "union adds no trusted identities");
    for name in [
        "graph_union",
        "graph_union_edge",
        "graph_union_edge_left",
        "graph_union_edge_right",
        "graph_union_edge_cases",
        "graph_union_reachable_left",
        "graph_union_reachable_right",
    ] {
        let id = env.globals[&format!("{GRAPH}.{name}")];
        assert!(owned.contains(&id), "{name} is owned by Dependency");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("all seven new names and six law shapes check at open arguments");
    match env.elaborate_file(&format!("import {GRAPH} (bool_or_true_cases)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{GRAPH}.bool_or_true_cases"))
        }
        other => panic!("private helper must refuse selective import: {other:?}"),
    }
}

const PREFIX: &str = r###"import Data.Finite.Finite (Finite, MkFinite)

import Data.Collections.Derived (list_elem_head, list_elem_later)

import Algorithm.Graphs.Dependency
  (Graph, MkGraph, edge, graph_union, graph_reachable)

data Vertex3 : Type where {
  V0 : Vertex3;
  V1 : Vertex3;
  V2 : Vertex3
}

const vertex3_finite : Finite Vertex3 =
  MkFinite
    Vertex3
    (Cons Vertex3 V0 (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3))))
    (λv.
      match v {
        V0 ↦ list_elem_head Vertex3 V0 (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)));
        V1 ↦
          list_elem_later
            Vertex3
            V1
            V0
            (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)))
            (list_elem_head Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)));
        V2 ↦
          list_elem_later
            Vertex3
            V2
            V0
            (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)))
            (list_elem_later
              Vertex3
              V2
              V1
              (Cons Vertex3 V2 (Nil Vertex3))
              (list_elem_head Vertex3 V2 (Nil Vertex3)))
      })


import Core.Classes.LawfulClasses (bool_or, bool_not)

const g_only : Graph Vertex3 =
  MkGraph Vertex3
    (λu. λv.
      match u {
        V0 ↦ match v { V0 ↦ False; V1 ↦ True; V2 ↦ False };
        V1 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ False };
        V2 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ False }
      })

const h_only : Graph Vertex3 =
  MkGraph Vertex3
    (λu. λv.
      match u {
        V0 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ False };
        V1 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ True };
        V2 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ False }
      })

fn is_v1 (v : Vertex3) : Bool =
  match v { V0 ↦ False; V1 ↦ True; V2 ↦ False }

fn is_v2 (v : Vertex3) : Bool =
  match v { V0 ↦ False; V1 ↦ False; V2 ↦ True }

const U : Graph Vertex3 = graph_union Vertex3 g_only h_only
"###;

const UNION_REACHES_V2: &str = r###"theorem control_union_reaches_V2 : Equal Bool (graph_reachable Vertex3 vertex3_finite U V0 is_v2) True = Proved"###;
const G_MISSES_V2: &str = r###"theorem control_g_misses_V2 : Equal Bool (graph_reachable Vertex3 vertex3_finite g_only V0 is_v2) False = Proved"###;
const H_MISSES_V2: &str = r###"theorem control_h_misses_V2 : Equal Bool (graph_reachable Vertex3 vertex3_finite h_only V0 is_v2) False = Proved"###;
const T1_POSITIVE: &str =
    r###"theorem control_T1_positive : Equal Bool (edge Vertex3 U V1 V2) True = Proved"###;
const T2_POSITIVE: &str =
    r###"theorem control_T2_positive : Equal Bool (edge Vertex3 U V2 V0) False = Proved"###;
const T1_TWIN: &str = r###"theorem cell_T1 : Equal Bool (edge Vertex3 U V1 V2) False = Proved"###;
const T2_TWIN: &str = r###"theorem cell_T2 : Equal Bool (edge Vertex3 U V2 V0) True = Proved"###;

// Compare proposition structure from authored Ken fixtures, not source text
// in the repository. Formatting or a declaration-span shift is immaterial.
fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected closed edge proposition: {other:?}"),
    }
}

fn equality_shape(source: &str) -> (String, String, String) {
    let decls = parse_decls(source).expect("closed edge theorem parses");
    let [SourceDecl::TheoremDecl {
        theorem,
        body: Expr::ECon(proof, _),
        ..
    }] = decls.as_slice()
    else {
        panic!("one closed edge theorem with a constructor proof");
    };
    assert_eq!(
        proof, "Proved",
        "the matched controls use the same proof form"
    );
    let mut args = Vec::new();
    let mut head = theorem;
    while let SourceType::TApp(function, argument, _) = head {
        args.push(argument.as_ref());
        head = function;
    }
    args.reverse();
    assert!(
        matches!(head, SourceType::TVar(name, _) | SourceType::TCon(name, _) if name == "Equal")
    );
    let [carrier, left, right] = args.as_slice() else {
        panic!("three Equal arguments");
    };
    (type_shape(carrier), type_shape(left), type_shape(right))
}

fn check_twin(tag: &str, positive: &str, twin: &str) {
    let good = equality_shape(positive);
    let wrong = equality_shape(twin);
    assert_eq!(wrong.0, good.0, "{tag}: same Bool carrier");
    assert_eq!(wrong.1, good.1, "{tag}: same edge, graph and vertices");
    assert_ne!(wrong.2, good.2, "{tag}: distinct false edge value");
    let mut env = loaded_graph();
    env.elaborate_file(PREFIX)
        .expect("fresh shared Vertex3 and graph prefix");
    env.elaborate_file(positive)
        .expect("same-form true edge control accepts first");
    let start = twin.rfind("Proved").expect("twin proof");
    match env.elaborate_file(twin) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start <= start
            && span.end >= start + "Proved".len()
            && span.end <= twin.len()
            && span.start < span.end => {}
        other => panic!("{tag} must refuse at its own theorem and proof: {other:?}"),
    }
}

/// SOURCE: spec 62 §15's closed three-vertex discriminator.
/// PROMISE CLASS: normative compatibility vector for the specified edges.
/// MEASURED: the union reaches V2 from V0 while neither input does; on a
/// fresh accepted prefix for each twin, the same-form true edge checks first,
/// and the false claim refuses as KernelRejected/TypeMismatch in its own span.
/// CLAIMED: union preserves both input edges and invents none.
/// THE GAP: these closed cases do not prove all open graphs; the checked
/// generic client above supplies the separate public-law evidence.
#[test]
fn closed_union_reachability_and_two_same_form_edge_refusals() {
    let mut env = loaded_graph();
    env.elaborate_file(PREFIX)
        .expect("closed Vertex3 carrier checks");
    for (label, statement) in [
        ("union reaches V2", UNION_REACHES_V2),
        ("g alone misses V2", G_MISSES_V2),
        ("h alone misses V2", H_MISSES_V2),
    ] {
        env.elaborate_file(statement)
            .unwrap_or_else(|error| panic!("{label}: {error:?}"));
    }
    check_twin("no edge V1 to V2", T1_POSITIVE, T1_TWIN);
    check_twin("invented edge V2 to V0", T2_POSITIVE, T2_TWIN);
}
