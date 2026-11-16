//! Union grouping preserves graph reachability: spec/50-stdlib/62-graphs.md §17.
//! Promise class: durable invariant. An open law checks the universal type;
//! the three-edge chain independently distinguishes dropping the third graph.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

#[path = "support/checked_is_true.rs"]
mod checked_is_true;

const GRAPH: &str = "Algorithm.Graphs.Dependency";
const ALIAS_WITNESS: &str = "theorem selected_true : IsTrue True = Proved\n";

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
            .unwrap_or_else(|error| panic!("provider {provider}: {error:?}"));
    }
}

fn loaded_graph() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    env.elaborate_module_from_roots(&[root()], GRAPH)
        .expect("Dependency roots-loads");
    env
}

fn selected_accepted() -> String {
    // Keep the registered client whole, and prove that its test-side IsTrue
    // selection is an explicit import alias rather than an ambient name.
    checked_is_true::selected_source_with_import(&format!("{ACCEPTED}{ALIAS_WITNESS}"))
}

fn body_contains_const(body: &Term, owner: GlobalId) -> bool {
    let mut terms = vec![body];
    while let Some(term) = terms.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return true;
        }
        terms.extend(term.children());
    }
    false
}

const ACCEPTED: &str = r#"import Data.Finite.Finite (Finite, MkFinite)

import Data.Collections.Derived (list_elem_head, list_elem_later)

import Algorithm.Graphs.Dependency
  (Graph, MkGraph, edge, graph_union, graph_reachable, graph_union_reachable_assoc)

data Vertex4 : Type where {
  V0 : Vertex4;
  V1 : Vertex4;
  V2 : Vertex4;
  V3 : Vertex4
}

const vertex4_finite : Finite Vertex4 =
  MkFinite
    Vertex4
    (Cons Vertex4 V0 (Cons Vertex4 V1 (Cons Vertex4 V2 (Cons Vertex4 V3 (Nil Vertex4)))))
    (λv.
      match v {
        V0 ↦
          list_elem_head
            Vertex4
            V0
            (Cons Vertex4 V1 (Cons Vertex4 V2 (Cons Vertex4 V3 (Nil Vertex4))));
        V1 ↦
          list_elem_later
            Vertex4
            V1
            V0
            (Cons Vertex4 V1 (Cons Vertex4 V2 (Cons Vertex4 V3 (Nil Vertex4))))
            (list_elem_head Vertex4 V1 (Cons Vertex4 V2 (Cons Vertex4 V3 (Nil Vertex4))));
        V2 ↦
          list_elem_later
            Vertex4
            V2
            V0
            (Cons Vertex4 V1 (Cons Vertex4 V2 (Cons Vertex4 V3 (Nil Vertex4))))
            (list_elem_later
              Vertex4
              V2
              V1
              (Cons Vertex4 V2 (Cons Vertex4 V3 (Nil Vertex4)))
              (list_elem_head Vertex4 V2 (Cons Vertex4 V3 (Nil Vertex4))));
        V3 ↦
          list_elem_later
            Vertex4
            V3
            V0
            (Cons Vertex4 V1 (Cons Vertex4 V2 (Cons Vertex4 V3 (Nil Vertex4))))
            (list_elem_later
              Vertex4
              V3
              V1
              (Cons Vertex4 V2 (Cons Vertex4 V3 (Nil Vertex4)))
              (list_elem_later
                Vertex4
                V3
                V2
                (Cons Vertex4 V3 (Nil Vertex4))
                (list_elem_head Vertex4 V3 (Nil Vertex4))))
      })

const g_only : Graph Vertex4 =
  MkGraph
    Vertex4
    (λu.
      λv.
        match u {
          V0 ↦
            match v {
              V0 ↦ False;
              V1 ↦ True;
              V2 ↦ False;
              V3 ↦ False
            };
          V1 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False;
              V3 ↦ False
            };
          V2 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False;
              V3 ↦ False
            };
          V3 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False;
              V3 ↦ False
            }
        })

const h_only : Graph Vertex4 =
  MkGraph
    Vertex4
    (λu.
      λv.
        match u {
          V0 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False;
              V3 ↦ False
            };
          V1 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ True;
              V3 ↦ False
            };
          V2 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False;
              V3 ↦ False
            };
          V3 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False;
              V3 ↦ False
            }
        })

const k_only : Graph Vertex4 =
  MkGraph
    Vertex4
    (λu.
      λv.
        match u {
          V0 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False;
              V3 ↦ False
            };
          V1 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False;
              V3 ↦ False
            };
          V2 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False;
              V3 ↦ True
            };
          V3 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False;
              V3 ↦ False
            }
        })

fn is_v3 (v : Vertex4) : Bool =
  match v {
    V0 ↦ False;
    V1 ↦ False;
    V2 ↦ False;
    V3 ↦ True
  }

theorem gen
      (q : Type)
      (fq : Finite q)
      (g : Graph q)
      (h : Graph q)
      (k : Graph q)
      (s : q)
      (target : q → Bool)
    : Equal Bool
        (graph_reachable q fq (graph_union q (graph_union q g h) k) s target)
        (graph_reachable q fq (graph_union q g (graph_union q h k)) s target) =
  graph_union_reachable_assoc q fq g h k s target

theorem left_grouping_reaches_v3
    : Equal Bool
        (graph_reachable
          Vertex4
          vertex4_finite
          (graph_union Vertex4 (graph_union Vertex4 g_only h_only) k_only)
          V0
          is_v3)
        True =
  Proved

theorem right_grouping_reaches_v3
    : Equal Bool
        (graph_reachable
          Vertex4
          vertex4_finite
          (graph_union Vertex4 g_only (graph_union Vertex4 h_only k_only))
          V0
          is_v3)
        True =
  Proved

theorem g_h_misses_v3
    : Equal Bool
        (graph_reachable Vertex4 vertex4_finite (graph_union Vertex4 g_only h_only) V0 is_v3)
        False =
  Proved

theorem law_closed
    : Equal Bool
        (graph_reachable
          Vertex4
          vertex4_finite
          (graph_union Vertex4 (graph_union Vertex4 g_only h_only) k_only)
          V0
          is_v3)
        (graph_reachable
          Vertex4
          vertex4_finite
          (graph_union Vertex4 g_only (graph_union Vertex4 h_only k_only))
          V0
          is_v3) =
  graph_union_reachable_assoc Vertex4 vertex4_finite g_only h_only k_only V0 is_v3

theorem groupings_agree
    : Equal Bool
        (graph_reachable
          Vertex4
          vertex4_finite
          (graph_union Vertex4 (graph_union Vertex4 g_only h_only) k_only)
          V0
          is_v3)
        (graph_reachable
          Vertex4
          vertex4_finite
          (graph_union Vertex4 g_only (graph_union Vertex4 h_only k_only))
          V0
          is_v3) =
  Proved
"#;

const DROP_K: &str = r#"theorem drop_k_claim
    : Equal Bool
        (graph_reachable
          Vertex4
          vertex4_finite
          (graph_union Vertex4 (graph_union Vertex4 g_only h_only) k_only)
          V0
          is_v3)
        (graph_reachable Vertex4 vertex4_finite (graph_union Vertex4 g_only h_only) V0 is_v3) =
  Proved
"#;

/// SOURCE: spec 62 §17. PROMISE CLASS: durable invariant.
/// MEASURED: after the same ten providers used by the union-commutation test,
/// exact trusted GlobalId sets agree across the Dependency load; the public
/// law has a checked owner-local Transparent body. The open accepted client
/// applies its full seven-binder type and contains its authenticated owner
/// Const. CLAIMED: arbitrary finite graphs can reuse the proved law without
/// new local trust. THE GAP: inherited provider trust is not erased, and the
/// separate closed discriminator does not establish a generic theorem.
#[test]
fn union_grouping_law_is_open_owned_transparent_and_zero_new_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], GRAPH)
        .expect("Dependency kernel-checks after providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after, before, "no new trusted GlobalId identities");
    let id = env.globals[&format!("{GRAPH}.graph_union_reachable_assoc")];
    assert!(
        owned.contains(&id),
        "Dependency owns the checked grouping law"
    );
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    env.elaborate_file(&selected_accepted())
        .expect("full accepted client and explicit aliased IsTrue witness check");
    let (_, body) = env
        .env
        .transparent_body(env.globals["gen"])
        .expect("open generic client body");
    assert!(
        body_contains_const(&body, id),
        "generic law uses owner Const"
    );
}

/// SOURCE: spec 62 §17. PROMISE CLASS: durable invariant.
/// MEASURED: both private edge regrouping helpers refuse selective imports
/// as UnboundName at their exact qualified names. CLAIMED: the public law is
/// the reuse surface, while edge-level derivation stays package-private.
/// THE GAP: this controls these two declared helpers, not any future helper.
#[test]
fn both_union_regroup_helpers_remain_private() {
    for helper in [
        "graph_union_edge_regroup_right",
        "graph_union_edge_regroup_left",
    ] {
        let source = format!("import {GRAPH} ({helper})");
        let mut env = loaded_graph();
        match env.elaborate_file(&source) {
            Err(ElabError::UnboundName { name, .. }) if name == format!("{GRAPH}.{helper}") => (),
            other => panic!("{helper} must refuse selective import: {other:?}"),
        }
    }
}

/// SOURCE: spec 62 §17. PROMISE CLASS: durable invariant.
/// MEASURED: the complete registered Vertex4 prefix checks its open law,
/// independent True three-way and False two-way controls, closed law and
/// same-form agreement. After that whole accepted prefix is freshly checked
/// for the sole negative, dropping k refuses only as kernel TypeMismatch
/// within the parsed drop_k_claim theorem, enclosing its Proved.
/// CLAIMED: the third graph's unique edge is needed for V0 to reach V3.
/// THE GAP: this one finite walk does not replace the generic checked proof.
#[test]
fn full_three_graph_client_accepts_and_drop_k_refuses_at_own_claim() {
    let accepted = selected_accepted();
    let mut positive = loaded_graph();
    positive
        .elaborate_file(&accepted)
        .expect("full Vertex4 client checks");
    for name in [
        "gen",
        "left_grouping_reaches_v3",
        "right_grouping_reaches_v3",
        "g_h_misses_v3",
        "law_closed",
        "groupings_agree",
        "selected_true",
    ] {
        assert!(
            matches!(
                positive.env.lookup(positive.globals[name]),
                Some(Decl::Transparent { .. })
            ),
            "{name} must check"
        );
    }
    let checked_negative = checked_is_true::selected_source(DROP_K);
    let parsed = parse_decls(&checked_negative).expect("negative parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = parsed.as_slice()
    else {
        panic!("only the drop_k_claim theorem follows the accepted prefix");
    };
    assert_eq!(name, "drop_k_claim");
    let mut fresh = loaded_graph();
    fresh
        .elaborate_file(&accepted)
        .expect("fresh whole accepted prefix checks before refusal");
    let span = match fresh
        .elaborate_file(&checked_negative)
        .expect_err("drop-k claim must refuse")
    {
        ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        } => span,
        other => panic!("drop-k must reject at own theorem with exact kind: {other:?}"),
    };
    assert!(
        claim.start <= span.start && span.start < span.end && span.end <= claim.end,
        "refusal must be inside drop_k_claim: {span:?} vs {claim:?}"
    );
    let terminal = checked_negative.rfind("Proved").expect("own proof term");
    assert!(
        span.start <= terminal && terminal + "Proved".len() <= span.end,
        "refusal must enclose its own Proved: {span:?}"
    );
    assert!(
        std::str::from_utf8(&checked_negative.as_bytes()[span.start..span.end]).is_ok(),
        "refusal span must be on UTF-8 byte boundaries"
    );
}
