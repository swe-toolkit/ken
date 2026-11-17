//! Reversing a finite graph swaps directed reachability endpoints: spec 62 §18.
//! Promise class: durable invariant. The closed opposite endpoint pairs
//! distinguish a graph reversal from simply exchanging source and target.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const GRAPH: &str = "Algorithm.Graphs.Dependency";
const LAW: &str = "graph_reachable_reverse_swap";
const HELPER: &str = "graph_reachable_reverse_swap_true";

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
        .expect("checked Dependency roots-loads");
    env
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

const OPEN: &str = r#"import Core.Classes.LawfulClasses (DecEq)
import Data.Finite.Finite (Finite)
import Algorithm.Graphs.Dependency (Graph, graph_reverse, graph_reachable, graph_reachable_reverse_swap)

theorem generic_reverse_swap
      (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) (s : q) (t : q)
    : Equal Bool
        (graph_reachable q fq g s (d.eq t))
        (graph_reachable q fq (graph_reverse q g) t (d.eq s)) =
  graph_reachable_reverse_swap q fq d g s t
"#;

// The entire checked Architect client is held in one source literal. Its
// independent True/False conclusions make the false strengthening visible.
const ACCEPTED: &str = r#"import Core.Classes.LawfulClasses (DecEq)

import Data.Finite.Finite (Finite, MkFinite)

import Data.Collections.Derived (list_elem_head, list_elem_later)

import Algorithm.Graphs.Dependency
  (Graph, MkGraph, graph_reverse, graph_reachable, graph_reachable_reverse_swap)

data Vertex3 : Type where {
  V0 : Vertex3;
  V1 : Vertex3;
  V2 : Vertex3
}

fn vertex3_eq (x : Vertex3) (y : Vertex3) : Bool =
  match x {
    V0 ↦
      match y {
        V0 ↦ True;
        V1 ↦ False;
        V2 ↦ False
      };
    V1 ↦
      match y {
        V0 ↦ False;
        V1 ↦ True;
        V2 ↦ False
      };
    V2 ↦
      match y {
        V0 ↦ False;
        V1 ↦ False;
        V2 ↦ True
      }
  }

theorem vertex3_eq_sound
      (x : Vertex3) (y : Vertex3)
    : Equal Bool (vertex3_eq x y) True → Equal Vertex3 x y =
  match x {
    V0 ↦
      match y {
        V0 ↦ λsame. Proved;
        V1 ↦ λsame. absurd same;
        V2 ↦ λsame. absurd same
      };
    V1 ↦
      match y {
        V0 ↦ λsame. absurd same;
        V1 ↦ λsame. Proved;
        V2 ↦ λsame. absurd same
      };
    V2 ↦
      match y {
        V0 ↦ λsame. absurd same;
        V1 ↦ λsame. absurd same;
        V2 ↦ λsame. Proved
      }
  }

theorem vertex3_eq_complete
      (x : Vertex3) (y : Vertex3)
    : Equal Vertex3 x y → Equal Bool (vertex3_eq x y) True =
  match x {
    V0 ↦
      match y {
        V0 ↦ λsame. Proved;
        V1 ↦ λsame. absurd same;
        V2 ↦ λsame. absurd same
      };
    V1 ↦
      match y {
        V0 ↦ λsame. absurd same;
        V1 ↦ λsame. Proved;
        V2 ↦ λsame. absurd same
      };
    V2 ↦
      match y {
        V0 ↦ λsame. absurd same;
        V1 ↦ λsame. absurd same;
        V2 ↦ λsame. Proved
      }
  }

instance DecEq Vertex3 {
  eq = vertex3_eq;
  sound = vertex3_eq_sound;
  complete = vertex3_eq_complete
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

const chain : Graph Vertex3 =
  MkGraph
    Vertex3
    (λu.
      λv.
        match u {
          V0 ↦
            match v {
              V0 ↦ False;
              V1 ↦ True;
              V2 ↦ False
            };
          V1 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ True
            };
          V2 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False
            }
        })

theorem swap_law_client
      (s : Vertex3) (t : Vertex3)
    : Equal Bool
        (graph_reachable Vertex3 vertex3_finite chain s (vertex3_eq t))
        (graph_reachable
          Vertex3
          vertex3_finite
          (graph_reverse Vertex3 chain)
          t
          (vertex3_eq s)) =
  graph_reachable_reverse_swap Vertex3 vertex3_finite DecEq_instance_Vertex3 chain s t

theorem v0_reaches_v2
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 (vertex3_eq V2)) True =
  Proved

theorem reversed_v2_reaches_v0
    : Equal Bool
        (graph_reachable
          Vertex3
          vertex3_finite
          (graph_reverse Vertex3 chain)
          V2
          (vertex3_eq V0))
        True =
  Proved

theorem swap_law_v0_v2
    : Equal Bool
        (graph_reachable Vertex3 vertex3_finite chain V0 (vertex3_eq V2))
        (graph_reachable
          Vertex3
          vertex3_finite
          (graph_reverse Vertex3 chain)
          V2
          (vertex3_eq V0)) =
  graph_reachable_reverse_swap Vertex3 vertex3_finite DecEq_instance_Vertex3 chain V0 V2

theorem v2_misses_v0
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 (vertex3_eq V0)) False =
  Proved

theorem reversed_v0_misses_v2
    : Equal Bool
        (graph_reachable
          Vertex3
          vertex3_finite
          (graph_reverse Vertex3 chain)
          V0
          (vertex3_eq V2))
        False =
  Proved

theorem swap_law_v2_v0
    : Equal Bool
        (graph_reachable Vertex3 vertex3_finite chain V2 (vertex3_eq V0))
        (graph_reachable
          Vertex3
          vertex3_finite
          (graph_reverse Vertex3 chain)
          V0
          (vertex3_eq V2)) =
  graph_reachable_reverse_swap Vertex3 vertex3_finite DecEq_instance_Vertex3 chain V2 V0

theorem swap_with_reverse_agrees
    : Equal Bool
        (graph_reachable Vertex3 vertex3_finite chain V0 (vertex3_eq V2))
        (graph_reachable
          Vertex3
          vertex3_finite
          (graph_reverse Vertex3 chain)
          V2
          (vertex3_eq V0)) =
  Proved
"#;

const WITHOUT_REVERSE: &str = r#"theorem swap_without_reverse_claim
    : Equal Bool
        (graph_reachable Vertex3 vertex3_finite chain V0 (vertex3_eq V2))
        (graph_reachable Vertex3 vertex3_finite chain V2 (vertex3_eq V0)) =
  Proved
"#;

/// SOURCE: spec/50-stdlib/62-graphs.md §18; Architect AC-0.
/// PROMISE CLASS: durable invariant. MEASURED: the ten providers load first;
/// Dependency adds no trusted GlobalId to that exact set, and the public law
/// is owner-local and Transparent. A generic six-binder application imports
/// the public API and contains its checked owner Const. CLAIMED: a checked
/// decision-level reverse-swap law is reusable without new package-local
/// trust. THE GAP: this does not remove assumptions inherited from providers.
#[test]
fn reverse_swap_law_is_generic_owned_transparent_and_zero_new_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], GRAPH)
        .expect("Dependency roots-loads with checked reverse-swap law");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after, before, "no new trusted GlobalId identities");
    let id = env.globals[&format!("{GRAPH}.{LAW}")];
    assert!(owned.contains(&id), "Dependency owns the new public law");
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    env.elaborate_file(OPEN)
        .expect("generic exact public law type checks under selective imports");
    let (_, body) = env
        .env
        .transparent_body(env.globals["generic_reverse_swap"])
        .expect("generic application has a checked body");
    assert!(
        body_contains_const(&body, id),
        "generic application must use exact checked Dependency owner Const"
    );
}

/// SOURCE: spec 62 §18. PROMISE CLASS: durable invariant.
/// MEASURED: selective import of the owner-local one-direction helper rejects
/// only as UnboundName with its exact qualified name. CLAIMED: public Boolean
/// equality is the reuse surface, not a second exported one-way theorem.
/// THE GAP: this controls one declared helper, not future private scaffolding.
#[test]
fn one_direction_step_remains_private() {
    let mut env = loaded_graph();
    let name = format!("{GRAPH}.{HELPER}");
    let id = env.globals[&name];
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    match env.elaborate_file(&format!("import {GRAPH} ({HELPER})")) {
        Err(ElabError::UnboundName { name: rejected, .. }) if rejected == name => (),
        other => panic!("private one-direction helper import must refuse: {other:?}"),
    }
}

/// SOURCE: spec 62 §18, Architect's closed Vertex3 discriminator.
/// PROMISE CLASS: durable invariant with fixed closed control values.
/// MEASURED: the entire accepted client checks its open s/t law, true and
/// false reachability for opposite endpoint pairs, corresponding reversed
/// reachabilities, both closed applications and same-form Proved control.
/// After a fresh complete accepted prefix, the sole added no-reversal claim
/// rejects as KernelRejected TypeMismatch within its own parsed theorem and
/// encloses its own Proved. CLAIMED: graph reversal, not endpoint exchange
/// alone, gives the decision equality. THE GAP: this fixed carrier does not
/// substitute for the separately checked generic theorem.
#[test]
fn accepted_chain_and_false_no_reversal_claim_are_distinguished() {
    let mut positive = loaded_graph();
    positive
        .elaborate_file(ACCEPTED)
        .expect("whole directed-chain client checks");
    for name in [
        "swap_law_client",
        "v0_reaches_v2",
        "reversed_v2_reaches_v0",
        "swap_law_v0_v2",
        "v2_misses_v0",
        "reversed_v0_misses_v2",
        "swap_law_v2_v0",
        "swap_with_reverse_agrees",
    ] {
        assert!(
            matches!(
                positive.env.lookup(positive.globals[name]),
                Some(Decl::Transparent { .. })
            ),
            "{name} must have a checked body"
        );
    }
    let parsed = parse_decls(WITHOUT_REVERSE).expect("standalone negative theorem parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = parsed.as_slice()
    else {
        panic!("the appended negative is exactly one theorem");
    };
    assert_eq!(name, "swap_without_reverse_claim");
    let mut fresh = loaded_graph();
    fresh
        .elaborate_file(ACCEPTED)
        .expect("fresh entire accepted prefix checks before only negative");
    let span = match fresh
        .elaborate_file(WITHOUT_REVERSE)
        .expect_err("no-reversal claim must refuse")
    {
        ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        } => span,
        other => panic!("negative must refuse at only own claim with exact kind: {other:?}"),
    };
    assert!(
        claim.start <= span.start && span.start < span.end && span.end <= claim.end,
        "refusal must stay inside sole appended claim: {span:?} vs {claim:?}"
    );
    let terminal = WITHOUT_REVERSE
        .rfind("Proved")
        .expect("negative terminal exists");
    assert!(
        span.start <= terminal && terminal + "Proved".len() <= span.end,
        "refusal must enclose its own Proved token"
    );
    assert!(
        WITHOUT_REVERSE.get(span.start..span.end).is_some(),
        "error span must follow UTF-8 byte boundaries"
    );
}
