# Reachability is monotone under edge inclusion

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §7.
Ruling: Architect AC-0 `evt_2bvgxef2k0jd8`.

The closed client applies the generic graph-monotonicity law from `chain` to
`looped` on a three-vertex carrier. `chain` has the two edges V0 → V1 → V2;
`looped` adds only V2 → V0. From V0, both graphs reach target V2. Independent
closed values check both results, and the law transports the affirmative
result across pointwise edge inclusion. From V2, `looped` reaches V0 but
`chain` does not. These finite cases discriminate direction but do not prove
the generic law.

## Accepted client

Accept the complete client before testing the isolated refusal.

```ken
import Data.Finite.Finite (Finite, MkFinite)

import Data.Collections.Derived (list_elem_head, list_elem_later)

import Algorithm.Graphs.Dependency
  (Graph, MkGraph, edge, graph_reachable, graph_reachable_graph_monotone)

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

const looped : Graph Vertex3 =
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
              V0 ↦ True;
              V1 ↦ False;
              V2 ↦ False
            }
        })

fn is_v0 (v : Vertex3) : Bool =
  match v {
    V0 ↦ True;
    V1 ↦ False;
    V2 ↦ False
  }

fn is_v2 (v : Vertex3) : Bool =
  match v {
    V0 ↦ False;
    V1 ↦ False;
    V2 ↦ True
  }

theorem chain_inside_looped
    : (u : Vertex3)
      → (v : Vertex3)
      → Equal Bool (edge Vertex3 chain u v) True
      → Equal Bool (edge Vertex3 looped u v) True =
  λu.
    λv.
      match u {
        V0 ↦
          match v {
            V0 ↦ λe. absurd e;
            V1 ↦ λe. Proved;
            V2 ↦ λe. absurd e
          };
        V1 ↦
          match v {
            V0 ↦ λe. absurd e;
            V1 ↦ λe. absurd e;
            V2 ↦ λe. Proved
          };
        V2 ↦
          match v {
            V0 ↦ λe. absurd e;
            V1 ↦ λe. absurd e;
            V2 ↦ λe. absurd e
          }
      }

theorem added_edge_absent : Equal Bool (edge Vertex3 chain V2 V0) False = Proved

theorem added_edge_present : Equal Bool (edge Vertex3 looped V2 V0) True = Proved

theorem start_misses_target : Equal Bool (is_v2 V0) False = Proved

theorem chain_v0_reaches_v2
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 is_v2) True =
  Proved

theorem looped_v0_reaches_v2_value
    : Equal Bool (graph_reachable Vertex3 vertex3_finite looped V0 is_v2) True =
  Proved

theorem looped_v0_reaches_v2_by_law
    : Equal Bool (graph_reachable Vertex3 vertex3_finite looped V0 is_v2) True =
  graph_reachable_graph_monotone
    Vertex3
    vertex3_finite
    chain
    looped
    V0
    is_v2
    chain_inside_looped
    chain_v0_reaches_v2

theorem looped_v2_reaches_v0
    : Equal Bool (graph_reachable Vertex3 vertex3_finite looped V2 is_v0) True =
  Proved

theorem chain_v2_misses_v0
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 is_v0) False =
  Proved
```

### `stdlib/graphs/reachability-graph-monotone-under-edge-inclusion`

- spec: §7, `graph_reachable_graph_monotone`.
- promise class: durable invariant.
- given: `chain`, `looped`, their pointwise edge-inclusion proof, and
  affirmative reachability in `chain` from V0 to target V2.
- expect: the public law proves reachability in `looped`; independent
  closed `Proved` controls check both reachability values.
- measured: the two-edge witness reaches V2 in `chain`; the same target
  is reachable in `looped`; the law application agrees with its direct
  closed-value control. The added edge is absent from `chain` and present
  in `looped`, while V0 is not already a target.
- claimed: affirmative reachability is preserved when every edge of one
  graph is also an edge of another.
- gap: these finite cases discriminate the law's direction but do not
  prove it for arbitrary graphs, carriers, or targets.
- why: `looped` adds exactly V2 → V0, while the positive path from V0
  to V2 uses the two edges already present in `chain`.

## Isolated refused claim

On a fresh copy, append only `seed_neg` to the complete accepted client.
At V2, `looped` reaches V0, while `chain` does not. The accepted
`looped_v2_reaches_v0` has the same result shape and `Proved` form; the
accepted `chain_v2_misses_v0` independently checks the chain result as
False. Require refusal at the appended `Proved`, not at an import or an
earlier fixture.

### `stdlib/graphs/reject-reachability-graph-monotonicity-converse`

- spec: §7, which does not state the converse of edge-inclusion
  monotonicity.
- promise class: durable invariant.
- given: the same V2 start and V0 target; the accepted client checks that
  `looped` reaches V0 and `chain` does not.
- expect: refuse the claim that V2 reaches V0 in `chain`.
- measured: the same-form claim for `looped` accepts, and the `chain`
  result independently evaluates to False.
- claimed: reachability in a graph with fewer edges does not follow from
  reachability in a graph with more edges.
- gap: this counterexample rejects the stated false converse; it does not
  characterize every pair of graphs.
- why: the only additional edge, V2 → V0, is needed to reach the target
  from V2.

```ken
theorem seed_neg : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 is_v0) True =
  Proved
```

## Evidence boundary

Architect's AC-0 D0 on `944cabd38344c9118f50692b510ac57da7ec8bf8`
checked the generic proof and independent client with `ken check` and
`ken fmt --check` rc 0, no `Axiom`, and equal scratch `trusted_base()`
identity sets (114 → 114). Its closed client accepted the complete
positive block and rejected the isolated converse at its appended
`Proved`. The seven closed Bool rows refused `Refl`. These are isolated
D0 measurements, not an exact-candidate check or official conformance-runner
execution. The catalog export and package-owned test are not in this
spec-and-seed candidate; Foundation owns their later turn. No CI,
publication, landing, or landed-trust result is claimed.
