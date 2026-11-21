//! Graph union reachability absorption and idempotence: §19 of the graphs
//! contract. Promise class: durable invariant. The finite input is a
//! discriminator, while the exported transparent laws cover open graphs.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
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
            .unwrap_or_else(|error| panic!("provider {provider}: {error:?}"));
    }
}

fn loaded_graph() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    env.elaborate_module_from_roots(&[root()], GRAPH)
        .expect("Dependency root-loads");
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

// The three-vertex prefix is the same in the accepted and refused clients.
// It establishes both independent values before either use of the public law.
const PREFIX: &str = r#"import Data.Finite.Finite (Finite, MkFinite)

import Data.Collections.Derived (list_elem_head, list_elem_later)

import Algorithm.Graphs.Dependency
  (Graph, MkGraph, edge, graph_union, graph_reachable,
   graph_union_reachable_absorb, graph_union_reachable_idem)

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

const g_base : Graph Vertex3 =
  MkGraph Vertex3
    (λu. λv.
      match u {
        V0 ↦ match v { V0 ↦ False; V1 ↦ True; V2 ↦ False };
        V1 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ False };
        V2 ↦ match v { V0 ↦ True; V1 ↦ False; V2 ↦ False }
      })

const h_sub : Graph Vertex3 =
  MkGraph Vertex3
    (λu. λv.
      match u {
        V0 ↦ match v { V0 ↦ False; V1 ↦ True; V2 ↦ False };
        V1 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ False };
        V2 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ False }
      })

const h_adds : Graph Vertex3 =
  MkGraph Vertex3
    (λu. λv.
      match u {
        V0 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ False };
        V1 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ True };
        V2 ↦ match v { V0 ↦ False; V1 ↦ False; V2 ↦ False }
      })

fn is_v2 (v : Vertex3) : Bool =
  match v { V0 ↦ False; V1 ↦ False; V2 ↦ True }
"#;

const BASE_MISSES: &str = r#"theorem base_misses_v2
  : Equal Bool (graph_reachable Vertex3 vertex3_finite g_base V0 is_v2) False = Proved"#;
const ADDS_REACHES: &str = r#"theorem adds_reaches_v2
  : Equal Bool
      (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_base h_adds) V0 is_v2)
      True = Proved"#;
const GEN_ABSORB: &str = r#"theorem gen_absorb
      (q : Type) (fq : Finite q) (g : Graph q) (h : Graph q) (s : q)
      (target : q → Bool)
      (included : (u : q) → (v : q)
        → Equal Bool (edge q h u v) True
        → Equal Bool (edge q g u v) True)
    : Equal Bool
        (graph_reachable q fq (graph_union q g h) s target)
        (graph_reachable q fq g s target) =
  graph_union_reachable_absorb q fq g h s target included"#;
const GEN_IDEM: &str = r#"theorem gen_idem
      (q : Type) (fq : Finite q) (g : Graph q) (s : q) (target : q → Bool)
    : Equal Bool
        (graph_reachable q fq (graph_union q g g) s target)
        (graph_reachable q fq g s target) =
  graph_union_reachable_idem q fq g s target"#;
const SUB_SAME: &str = r#"theorem sub_same
  : Equal Bool
      (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_base h_sub) V0 is_v2)
      (graph_reachable Vertex3 vertex3_finite g_base V0 is_v2) = Proved"#;
const LAW_IDEM: &str = r#"theorem law_idem
  : Equal Bool
      (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_base g_base) V0 is_v2)
      (graph_reachable Vertex3 vertex3_finite g_base V0 is_v2) =
  graph_union_reachable_idem Vertex3 vertex3_finite g_base V0 is_v2"#;
const LAW_ABSORB_HEAD: &str = r#"theorem law_absorb
  : Equal Bool
      (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_base h_sub) V0 is_v2)
      (graph_reachable Vertex3 vertex3_finite g_base V0 is_v2) =
  graph_union_reachable_absorb
    Vertex3 vertex3_finite g_base h_sub V0 is_v2
    "#;
const INCLUSION: &str = r#"(λu. λv.
      match u {
        V0 ↦ match v {
          V0 ↦ λe. absurd e; V1 ↦ λe. Proved; V2 ↦ λe. absurd e
        };
        V1 ↦ match v {
          V0 ↦ λe. absurd e; V1 ↦ λe. absurd e; V2 ↦ λe. absurd e
        };
        V2 ↦ match v {
          V0 ↦ λe. Proved; V1 ↦ λe. absurd e; V2 ↦ λe. absurd e
        }
      })"#;

fn law_absorb(h: &str) -> String {
    format!("{}{INCLUSION}", LAW_ABSORB_HEAD.replace("h_sub", h))
}

fn inclusion_claim(h: &str) -> String {
    format!(
        "theorem included_edges : (u : Vertex3) → (v : Vertex3) \
         → Equal Bool (edge Vertex3 {h} u v) True \
         → Equal Bool (edge Vertex3 g_base u v) True = {INCLUSION}"
    )
}

/// SOURCE: graphs §19, graph_union_reachable_absorb and _idem.
/// PROMISE CLASS: durable invariant.
/// MEASURED: provider-first exact trusted GlobalId sets agree, both laws are
/// owned Transparent declarations, and open clients actually refer to those
/// checked identities.
/// CLAIMED: public generic laws prove absorption/idempotence with no local
/// trust addition, rather than relying on a closed example alone.
/// THE GAP: provider trust remains inherited; these observations do not claim
/// that the primitive providers are assumption-free or that CI has run.
#[test]
fn union_absorb_and_idem_open_public_owned_and_trust_neutral() {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], GRAPH)
        .expect("Dependency root-loads after the providers");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert!(!before.is_empty(), "the provider baseline is non-vacuous");
    eprintln!(
        "absorb trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "Dependency adds no assumption identities");
    for (name, source) in [
        ("graph_union_reachable_absorb", GEN_ABSORB),
        ("graph_union_reachable_idem", GEN_IDEM),
    ] {
        let owner = env.globals[&format!("{GRAPH}.{name}")];
        assert!(owned.contains(&owner), "Dependency owns public {name}");
        assert!(matches!(
            env.env.lookup(owner),
            Some(Decl::Transparent { .. })
        ));
        let client = format!("import Data.Finite.Finite (Finite)\nimport {GRAPH} (Graph, edge, graph_union, graph_reachable, {name})\n{source}");
        let elaborated = env
            .elaborate_file(&client)
            .unwrap_or_else(|error| panic!("open {name} client rejected: {error:?}"));
        let client_owner = env.globals[if name.ends_with("idem") {
            "gen_idem"
        } else {
            "gen_absorb"
        }];
        assert!(elaborated.contains(&client_owner));
        let Some(Decl::Transparent { body, .. }) = env.env.lookup(client_owner) else {
            panic!("open {name} client did not check as transparent");
        };
        assert!(
            contains_const(body, owner),
            "open client cites exact public law identity"
        );
    }
}

/// SOURCE: graphs §19, the accepted and refused Vertex3 seed clients.
/// PROMISE CLASS: durable invariant with exact values scoped to this fixture.
/// MEASURED: two independent reachability values and all open/closed law
/// applications check; the same-value closed Proved control checks, but the
/// otherwise identical law application with h_adds refuses in its own theorem.
/// CLAIMED: subgraph inclusion lets the union retain g's reachability, while
/// adding a genuinely new edge cannot reuse that inclusion proof.
/// THE GAP: one finite graph does not prove the generic law (the Transparent
/// proof above does). The separate premise test isolates why the whole-law
/// refusal occurs; its span alone names only the enclosing declaration.
#[test]
fn union_absorb_closed_values_idem_and_added_edge_refusal() {
    let mut accepted = loaded_graph();
    accepted
        .elaborate_file(PREFIX)
        .expect("checked Vertex3 prefix");
    for (label, row) in [
        ("base misses", BASE_MISSES),
        ("added edge reaches", ADDS_REACHES),
        ("generic absorb", GEN_ABSORB),
        ("generic idem", GEN_IDEM),
        ("closed independent equality", SUB_SAME),
        ("closed idem", LAW_IDEM),
    ] {
        accepted
            .elaborate_file(row)
            .unwrap_or_else(|error| panic!("{label}: {error:?}"));
    }
    accepted
        .elaborate_file(&law_absorb("h_sub"))
        .expect("closed absorption with true inclusion");

    let mut refused = loaded_graph();
    refused
        .elaborate_file(PREFIX)
        .expect("fresh checked Vertex3 prefix");
    refused.elaborate_file(BASE_MISSES).expect("base is False");
    refused
        .elaborate_file(ADDS_REACHES)
        .expect("union with added edge is True");
    // The accepted law was checked above on the same fixture; the refused
    // environment contains no other law claim before the appended false one.
    let wrong = law_absorb("h_adds");
    // Both mentions of h changed: the conclusion and the fourth law argument.
    // The inclusion proof and every other argument stay byte-identical.
    assert_eq!(LAW_ABSORB_HEAD.matches("h_sub").count(), 2);
    let proof = wrong
        .find("graph_union_reachable_absorb")
        .expect("law call");
    match refused.elaborate_file(&wrong) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start <= proof
            && span.end >= proof + "graph_union_reachable_absorb".len()
            && span.end <= wrong.len() => {}
        other => panic!("added edge must refuse at the appended law claim: {other:?}"),
    }
}

/// SOURCE: graphs §19 edge-inclusion premise, isolated from the equality.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the same inclusion proof term accepts from h_sub into g_base
/// and refuses from h_adds into g_base at its own declaration.
/// CLAIMED: the added V1 → V2 edge invalidates the premise even when no
/// reachability equality is involved.
/// THE GAP: a three-vertex witness isolates this premise; the generic law
/// still rests on its own checked body, not on this finite client.
#[test]
fn union_inclusion_premise_distinguishes_added_edge() {
    let mut positive = loaded_graph();
    positive.elaborate_file(PREFIX).expect("inclusion prefix");
    positive
        .elaborate_file(&inclusion_claim("h_sub"))
        .expect("subgraph inclusion is checked independently");

    let mut negative = loaded_graph();
    negative
        .elaborate_file(PREFIX)
        .expect("fresh inclusion prefix");
    let false_inclusion = inclusion_claim("h_adds");
    match negative.elaborate_file(&false_inclusion) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start == 0 && span.end <= false_inclusion.len() => {}
        other => panic!("h_adds inclusion must fail at its own premise: {other:?}"),
    }
}

/// SOURCE: graphs §19 private proof scaffolding.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the checked module is loaded, yet a selective client import of
/// its private helper refuses as UnboundName on that import.
/// CLAIMED: clients cannot turn a private edge decomposition into a new API.
/// THE GAP: this pin addresses the named helper, not all future helpers.
#[test]
fn union_edge_absorb_helper_is_private() {
    let mut env = loaded_graph();
    let source = format!("import {GRAPH} (graph_union_edge_absorb)");
    match env.elaborate_file(&source) {
        Err(ElabError::UnboundName { name, span })
            if name == format!("{GRAPH}.graph_union_edge_absorb")
                && span.start <= "import ".len()
                && span.end >= source.len() - 1 => {}
        other => panic!("private edge helper import must refuse: {other:?}"),
    }
}
