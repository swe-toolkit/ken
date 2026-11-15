# Reachability is invariant under union operand order

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §16, child
`5f6532146fc3e76233ef2658f5728796788dbfa8`.
Ruling: Architect's AC-0, `evt_771vg4j5zt8a4`.

Status: authored behavioral oracle. The accepted and refused source
blocks below are not an official conformance-runner result. Architect's
AC-0 was a detached scratch development on landed base `a931dcb40`;
it did not execute this seed or deliver the catalog package.

The carrier is `Vertex3`. `g_only` contains only V0 → V1, and
`h_only` contains only V1 → V2. Their union reaches V2 from V0 in
either operand order, but `g_only` alone does not reach V2. The
accepted same-form `R-SAME` control checks equality of the two union
reachability results independently of the public law application.

The promise class is **durable invariant**: the case observes
reachability values and the result of a checked public theorem, not a
roster, milestone count, or repository spelling. The finite instances
do not replace the generic checked law.

## Accepted client

Accept the complete client. Its first row applies the public theorem at
generic arguments. The closed rows establish both reachability values,
apply the theorem in both orientations, and check an independent
same-form equality control.

The D0 source for this client is
`accepted-client.ken`, SHA-256
`080a68fb14b7e23441777eec88e3ffa6bff2661b6792cd863f29a22fd539201e`.

```ken
import Data.Finite.Finite (Finite, MkFinite)

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
              V2 ↦ False
            };
          V2 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False
            }
        })

const h_only : Graph Vertex3 =
  MkGraph
    Vertex3
    (λu.
      λv.
        match u {
          V0 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
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

fn is_v1 (v : Vertex3) : Bool =
  match v {
    V0 ↦ False;
    V1 ↦ True;
    V2 ↦ False
  }

fn is_v2 (v : Vertex3) : Bool =
  match v {
    V0 ↦ False;
    V1 ↦ False;
    V2 ↦ True
  }

const U : Graph Vertex3 = graph_union Vertex3 g_only h_only

theorem gen
      (q : Type) (fq : Finite q) (g : Graph q) (h : Graph q) (s : q) (target : q → Bool)
    : Equal Bool
        (graph_reachable q fq (graph_union q g h) s target)
        (graph_reachable q fq (graph_union q h g) s target) =
  graph_union_reachable_comm q fq g h s target

theorem gh_reaches_v2
    : Equal Bool
        (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_only h_only) V0 is_v2)
        True =
  Proved

theorem hg_reaches_v2
    : Equal Bool
        (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 h_only g_only) V0 is_v2)
        True =
  Proved

theorem law_gh
    : Equal Bool
        (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_only h_only) V0 is_v2)
        (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 h_only g_only) V0 is_v2) =
  graph_union_reachable_comm Vertex3 vertex3_finite g_only h_only V0 is_v2

theorem law_hg
    : Equal Bool
        (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 h_only g_only) V0 is_v2)
        (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_only h_only) V0 is_v2) =
  graph_union_reachable_comm Vertex3 vertex3_finite h_only g_only V0 is_v2

theorem orders_agree
    : Equal Bool
        (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_only h_only) V0 is_v2)
        (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 h_only g_only) V0 is_v2) =
  Proved
```

### `stdlib/graphs/R-GEN`

- spec: §16, `graph_union_reachable_comm`.
- promise class: durable invariant.
- given: arbitrary `q`, `fq`, `g`, `h`, `s`, and `target`.
- expect: the public theorem proves equality of the two reachability
  observations in its stated orientation.
- measured: a direct generic application of the imported public law.
- claimed: this client can use the public contract at open arguments.
- the gap: the client application is not a replacement for the
  catalog's generic checked proof.

### `stdlib/graphs/R-GH` and `stdlib/graphs/R-HG`

- spec: §16 and §15's `graph_union` semantics.
- promise class: durable invariant.
- given: `vertex3_finite`, the two operand graphs, start V0, and the
  predicate `is_v2`, with operand orders `g_only, h_only` and
  `h_only, g_only` respectively.
- expect: both reachability observations are `True`.
- measured: independent closed `Proved` rows for both union orders.
- claimed: exchanging the operands preserves this reachable target,
  and both input edges are needed to reach it.
- the gap: one finite path does not establish the universally
  quantified theorem or all graph-union behavior.

### `stdlib/graphs/R-LAW-GH` and `stdlib/graphs/R-LAW-HG`

- spec: §16, `graph_union_reachable_comm`.
- promise class: durable invariant.
- given: the same start, target, carrier and finite certificate, with
  each operand order.
- expect: the public theorem proves the equality for each order.
- measured: direct applications of the theorem in both orientations.
- claimed: the theorem's argument order and orientation are usable by
  clients.
- the gap: the closed applications do not prove the generic law body.

### `stdlib/graphs/R-SAME`

- spec: §16, operand-order invariance.
- promise class: durable invariant.
- given: the two union reachability values for V0 and `is_v2`.
- expect: the values are equal, established independently with
  `Proved`.
- measured: an accepted same-form equality without applying the law.
- claimed: the two union orders produce the same closed observation.
- the gap: this control does not establish the generic law.

## Isolated refused claim

Use a fresh input containing the same `Vertex3` fixture, `g_only`,
`h_only`, `is_v2`, and the `R-SAME` control. Then append only
`X-G-ONLY`. The D0 refusal source is
`refused-client.ken`, SHA-256
`872ee1ad8e1f5d0097e9c51513480dd33e5ca692e342a7d5e750163e95163154`.

```ken
import Data.Finite.Finite (Finite, MkFinite)

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


theorem orders_agree : Equal Bool (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_only h_only) V0 is_v2) (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 h_only g_only) V0 is_v2) = Proved

theorem union_is_g_only : Equal Bool (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_only h_only) V0 is_v2) (graph_reachable Vertex3 vertex3_finite g_only V0 is_v2) = Proved
```

### `stdlib/graphs/X-G-ONLY`

- spec: §16's `Vertex3` discriminator and §1.2's reachability
  semantics.
- promise class: durable invariant.
- given: the fresh accepted prefix ending with `R-SAME`.
- expect: `KernelRejected` at `X-G-ONLY`'s own `Proved`; do not pin
  a diagnostic code or message.
- control: `R-SAME` proves the same `Equal Bool` and `Proved` form on
  the same fixture.
- why: `union g_only h_only` reaches V2 from V0 by using both edges;
  `g_only` alone does not. The claimed equality is false, so the
  refusal is attributable to the value difference, not an import,
  fixture, or proof-form failure.

## Coverage boundary

These rows check operand-order invariance of reachability. They do not
assert public edge-level commutation, nor a decomposition of union
reachability into reachability in either input graph.

## Evidence boundary

Architect's AC-0 ruling `evt_771vg4j5zt8a4` reports that Foundation's
D0 checked the generic law and client, the accepted rows, and the
isolated X-G-ONLY refusal on scratch based on landed
`a931dcb4095bb6bda1d15a8b235e75d54187a9bd`. The D0 source hashes
above identify the row inputs used for this seed. These are scratch
measurements, not checks of this seed, the Spec child, an assembled
candidate, or the official conformance runner. No CI, publication,
landing, or landed-trust measurement is claimed.
