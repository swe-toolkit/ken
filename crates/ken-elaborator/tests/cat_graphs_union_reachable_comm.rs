//! Union reachability commutes: spec/50-stdlib/62-graphs.md §16.
//! An open law client and the closed three-vertex chain distinguish a union
//! from one of its operands; both interior proof helpers remain private.

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
        .expect("checked graph union and its reachability law root-load");
    env
}

// This is the §15 union-test Vertex3 prefix verbatim except that its selective
// Dependency import also includes graph_union_reachable_comm.
const PREFIX: &str = r###"import Data.Finite.Finite (Finite, MkFinite)

import Data.Collections.Derived (list_elem_head, list_elem_later)

import Algorithm.Graphs.Dependency
  (Graph, MkGraph, edge, graph_union, graph_reachable, graph_union_reachable_comm)

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

const R_GEN: &str = r###"theorem gen
      (q : Type) (fq : Finite q) (g : Graph q) (h : Graph q) (s : q) (target : q → Bool)
    : Equal Bool
        (graph_reachable q fq (graph_union q g h) s target)
        (graph_reachable q fq (graph_union q h g) s target) =
  graph_union_reachable_comm q fq g h s target"###;

const R_GH: &str = r###"theorem gh_reaches_v2 : Equal Bool (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_only h_only) V0 is_v2) True = Proved"###;
const R_HG: &str = r###"theorem hg_reaches_v2 : Equal Bool (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 h_only g_only) V0 is_v2) True = Proved"###;
const R_LAW_GH: &str = r###"theorem law_gh : Equal Bool (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_only h_only) V0 is_v2) (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 h_only g_only) V0 is_v2) = graph_union_reachable_comm Vertex3 vertex3_finite g_only h_only V0 is_v2"###;
const R_LAW_HG: &str = r###"theorem law_hg : Equal Bool (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 h_only g_only) V0 is_v2) (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_only h_only) V0 is_v2) = graph_union_reachable_comm Vertex3 vertex3_finite h_only g_only V0 is_v2"###;
const R_SAME: &str = r###"theorem orders_agree : Equal Bool (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_only h_only) V0 is_v2) (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 h_only g_only) V0 is_v2) = Proved"###;
const X_G_ONLY: &str = r###"theorem union_is_g_only : Equal Bool (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_only h_only) V0 is_v2) (graph_reachable Vertex3 vertex3_finite g_only V0 is_v2) = Proved"###;

// Inspect the parsed Ken propositions in the authored client fixtures, not
// the text or layout of a catalog, spec, or repository source file.
fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected closed reachability type: {other:?}"),
    }
}

fn equality_shape(source: &str) -> (String, String, String) {
    let decls = parse_decls(source).expect("closed reachability claim parses");
    let [SourceDecl::TheoremDecl {
        theorem,
        body: Expr::ECon(proof, _),
        ..
    }] = decls.as_slice()
    else {
        panic!("one closed theorem with a constructor proof");
    };
    assert_eq!(proof, "Proved", "positive and negative use the same proof");
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
        panic!("Equal has carrier and two endpoints");
    };
    (type_shape(carrier), type_shape(left), type_shape(right))
}

/// SOURCE: spec 62 §16; Architect's pre-registered open law client.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after explicit providers load, Dependency adds no trusted
/// GlobalId; it owns graph_union_reachable_comm as a Transparent declaration,
/// and a selective open import applies its exact q/fq/g/h/s/target contract.
/// CLAIMED: union reachability commutes at open finite graphs with no new trust.
/// THE GAP: the providers retain their own trust, and the separate closed
/// graph below tests discriminator values rather than proving the open law.
#[test]
fn union_reachable_comm_is_owned_transparent_open_and_zero_local_trust() {
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
            .unwrap_or_else(|error| panic!("provider {provider}: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], GRAPH)
        .expect("Dependency roots-loads after its providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "union reachable comm trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "the new law and helpers add no trust");
    let id = env.globals[&format!("{GRAPH}.graph_union_reachable_comm")];
    assert!(owned.contains(&id), "Dependency owns the public law");
    assert!(
        matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
        "the public theorem carries a kernel-checked body"
    );
    env.elaborate_file(&format!(
        "import Data.Finite.Finite (Finite)\n\
         import Algorithm.Graphs.Dependency (Graph, graph_union, graph_reachable, graph_union_reachable_comm)\n\
         {R_GEN}"
    ))
    .expect("R-GEN selectively imports and applies the open law");
}

/// SOURCE: spec 62 §16's three-vertex closed chain.
/// PROMISE CLASS: normative compatibility vector for the specified edges.
/// MEASURED: both union operand orders reach V2 from V0, both public-law
/// applications and a same-form Proved control accept; a fresh accepted
/// prefix then rejects replacing the swapped union with g_only as
/// KernelRejected/TypeMismatch at its own theorem enclosing its Proved.
/// CLAIMED: union order does not change reachability, but the union is not
/// equivalent to an input that lacks the second link of this chain.
/// THE GAP: the finite example is not a proof for arbitrary graphs; the
/// generic checked law above supplies the universally quantified statement.
#[test]
fn union_reachable_comm_closed_both_orders_and_false_single_input() {
    let positive = equality_shape(R_SAME);
    let negative = equality_shape(X_G_ONLY);
    assert_eq!(positive.0, negative.0, "same Bool carrier");
    assert_eq!(positive.1, negative.1, "same union/input and start vertex");
    assert_ne!(positive.2, negative.2, "one swapped union vs g_only");

    let mut accepted = loaded_graph();
    accepted
        .elaborate_file(PREFIX)
        .expect("§15 three-vertex graph prefix roots-loads");
    for (row, source) in [
        ("R-GEN", R_GEN),
        ("R-GH", R_GH),
        ("R-HG", R_HG),
        ("R-LAW-GH", R_LAW_GH),
        ("R-LAW-HG", R_LAW_HG),
        ("R-SAME", R_SAME),
    ] {
        accepted
            .elaborate_file(source)
            .unwrap_or_else(|error| panic!("{row} must accept: {error:?}"));
    }

    // Isolate the negative: neither the two law-at-value assertions nor the
    // two independent True-value theorems precede it in this environment.
    let mut refused = loaded_graph();
    refused
        .elaborate_file(PREFIX)
        .expect("fresh §15 three-vertex prefix");
    refused
        .elaborate_file(R_SAME)
        .expect("same-form positive control accepts before refusal");
    let proof = X_G_ONLY.rfind("Proved").expect("own proof term");
    match refused.elaborate_file(X_G_ONLY) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start <= proof
            && span.end >= proof + "Proved".len()
            && span.end <= X_G_ONLY.len() => {}
        other => panic!("X-G-ONLY must refuse at its own Proved: {other:?}"),
    }
}

/// SOURCE: spec 62 §16 private edge inclusion helper.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the selective import is refused as UnboundName with its fully
/// qualified Dependency name and import span.
/// CLAIMED: clients cannot import the private edge inclusion helper.
/// THE GAP: this checks the named helper, not all private declarations.
#[test]
fn union_reachable_comm_helpers_remain_private() {
    let mut env = loaded_graph();
    let source = format!("import {GRAPH} (graph_union_edge_swap)");
    let name = format!("{GRAPH}.graph_union_edge_swap");
    match env.elaborate_file(&source) {
        Err(ElabError::UnboundName {
            name: refused,
            span,
        }) if refused == name && span.start <= "import ".len() && span.end >= source.len() - 1 => {}
        other => panic!("private {name} must refuse its import: {other:?}"),
    }
}
