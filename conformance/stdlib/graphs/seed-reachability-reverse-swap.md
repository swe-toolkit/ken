# Reachability is preserved by reversing the graph and swapping endpoints

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §§8 and 18; `51 §2.2`.

The accepted client applies §18's public law at open endpoints on the
finite `Vertex3` chain from §8's reverse-reachability fixture. Its local
`DecEq Vertex3` instance supplies the law's equality decision. Closed
checks establish `True`/`True` for `(V0, V2)` and `False`/`False` for
`(V2, V0)`. A fresh copy of the accepted client isolates the false
same-form endpoint swap that omits `graph_reverse`.

## Accepted client

Accept this complete client before testing the isolated refusal. It defines
a checked `DecEq Vertex3` locally and uses the directed chain
`V0 → V1 → V2`.

```ken
import Core.Classes.LawfulClasses (DecEq)

import Data.Finite.Finite (Finite, MkFinite)

import Data.Collections.Derived (list_elem_head, list_elem_later)

import Algorithm.Graphs.Dependency
  (Graph, MkGraph, graph_reverse, graph_reachable,
   graph_reachable_reverse_swap)

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

theorem reverse_swap_open_endpoints (s : Vertex3) (t : Vertex3) :
  Equal Bool
    (graph_reachable Vertex3 vertex3_finite chain s
      (DecEq_instance_Vertex3.eq t))
    (graph_reachable Vertex3 vertex3_finite
      (graph_reverse Vertex3 chain) t
      (DecEq_instance_Vertex3.eq s)) =
  graph_reachable_reverse_swap
    Vertex3 vertex3_finite DecEq_instance_Vertex3 chain s t

theorem forward_v0_reaches_v2_value :
  Equal Bool
    (graph_reachable Vertex3 vertex3_finite chain V0
      (DecEq_instance_Vertex3.eq V2))
    True =
  Proved

theorem reversed_v2_reaches_v0_value :
  Equal Bool
    (graph_reachable Vertex3 vertex3_finite
      (graph_reverse Vertex3 chain) V2
      (DecEq_instance_Vertex3.eq V0))
    True =
  Proved

theorem reverse_swap_v0_v2 :
  Equal Bool
    (graph_reachable Vertex3 vertex3_finite chain V0
      (DecEq_instance_Vertex3.eq V2))
    (graph_reachable Vertex3 vertex3_finite
      (graph_reverse Vertex3 chain) V2
      (DecEq_instance_Vertex3.eq V0)) =
  graph_reachable_reverse_swap
    Vertex3 vertex3_finite DecEq_instance_Vertex3 chain V0 V2

theorem forward_v2_misses_v0_value :
  Equal Bool
    (graph_reachable Vertex3 vertex3_finite chain V2
      (DecEq_instance_Vertex3.eq V0))
    False =
  Proved

theorem reversed_v0_misses_v2_value :
  Equal Bool
    (graph_reachable Vertex3 vertex3_finite
      (graph_reverse Vertex3 chain) V0
      (DecEq_instance_Vertex3.eq V2))
    False =
  Proved

theorem reverse_swap_v2_v0 :
  Equal Bool
    (graph_reachable Vertex3 vertex3_finite chain V2
      (DecEq_instance_Vertex3.eq V0))
    (graph_reachable Vertex3 vertex3_finite
      (graph_reverse Vertex3 chain) V0
      (DecEq_instance_Vertex3.eq V2)) =
  graph_reachable_reverse_swap
    Vertex3 vertex3_finite DecEq_instance_Vertex3 chain V2 V0
```

### `stdlib/graphs/reachability-reverse-swap-open-and-closed`

- spec: §18's `graph_reachable_reverse_swap`; §§8 and 1.2; §51 §2.2.
- promise class: durable invariant.
- given: the complete client above with its finite chain and local
  `DecEq Vertex3` instance.
- measured: the open-endpoint law application, both closed law
  applications, and independent closed reachability values.
- claimed: reachability from `s` to `t` in `chain` agrees with
  reachability from `t` to `s` in the reversed chain.
- the gap: the fixed carrier and chain check the public law's use and
  discriminate its closed values; they do not prove the generic law.
- expect: the open application and both closed applications accept;
  `(V0, V2)` computes `True`/`True`, and `(V2, V0)` computes
  `False`/`False`.
- controls: the independent values check both sides of each closed pair;
  the `V0`/`V2` positive pair catches a result specialized to constant
  `False`.
- why: the chain reaches `V2` from `V0`; reversing its edges permits the
  corresponding `V2`-to-`V0` reachability, while the opposite pair is
  absent on both sides.

## Isolated refused claim

Start from a fresh complete copy of the accepted client and append only
`seed_neg` below. The accepted-with-reversal law application on `(V0, V2)`
is its positive control. The refused claim keeps the same endpoints and
reachability form but uses `chain` for both graph arguments.

### `stdlib/graphs/reject-forward-endpoint-swap-without-reversal`

- spec: §18 and §1.2.
- promise class: durable invariant.
- given: the complete accepted client, including the accepted
  with-reversal control; append only the theorem below.
- measured: the accepted reversal pair is `True`/`True`, while the
  no-reversal endpoint swap is `True`/`False`.
- claimed: swapping endpoints alone does not preserve reachability in a
  directed graph.
- the gap: a refusal alone could be caused by a broken fixture or proof
  form; the accepted same-form reversal control isolates the missing
  `graph_reverse`.
- expect: refuse at `seed_neg`; do not pin a diagnostic.
- controls: the complete accepted prefix and its `reverse_swap_v0_v2`
  theorem establish the with-reversal form; the independent forward values
  establish the two different no-reversal results.
- why: the chain has a path from `V0` to `V2` and none from `V2` to `V0`.

```ken
theorem seed_neg :
  Equal Bool
    (graph_reachable Vertex3 vertex3_finite chain V0
      (DecEq_instance_Vertex3.eq V2))
    (graph_reachable Vertex3 vertex3_finite chain V2
      (DecEq_instance_Vertex3.eq V0)) =
  Proved
```

## Evidence boundary

This file is an authored AC-2 seed on exact Spec parent
`a709a5e0a06b6a795315b3d0da62a7820f5d4fe1`. No CLI check, seed execution,
official conformance execution, CI, publication, landing, or assembled-tip
vote is claimed.
