# Reachability is invariant under union grouping

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §17, child
`0b97318f87b282302bdef4cbb3e151c699c23d88`.
Ruling: Architect AC-0 D0, `evt_10x9vpre83d6t`.

Status: authored behavioral oracle. The Foundation D0-CLEAN report
`evt_7rfx18cbnj57e` records scratch results on landed base
`b99258223ce8a1b9e5c77013c81551aaa8ab6415` using the D0 clients
reproduced below. This is not an official conformance run or a check of
the Spec child or an assembled candidate. This seed has not been run by
the official runner. No CI, publication, landing, or landed-trust
measurement is claimed.

The carrier is `Vertex4`. `g_only`, `h_only`, and `k_only` contain only
V0 → V1, V1 → V2, and V2 → V3 respectively. With start V0 and target
`is_v3`, both three-way union groupings reach V3, but the union of the first
two graphs does not. The only walk to V3 uses all three edges.

The promise class is **durable invariant**: the cases observe reachability
values and a checked public theorem, not a roster, milestone count, or
repository spelling. The finite fixture does not replace the generic law.

## Accepted client

Accept the complete client. The generic row applies the public theorem at
open arguments. The closed rows establish reachability for both groupings,
check that the first two graphs miss V3, apply the theorem at the closed
arguments, and prove an independent same-form equality control.

The D0 accepted client is identified by SHA-256
`d189c34134bf427a0b191e7e5df76bef358bb639f3982e96e2e3ecb0255f2bad`.

```ken
import Data.Finite.Finite (Finite, MkFinite)

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
```

### `stdlib/graphs/A-GEN`

- spec: §17, `graph_union_reachable_assoc`.
- promise class: durable invariant.
- given: arbitrary `q`, `fq`, `g`, `h`, `k`, `s`, and `target`.
- expect: the public theorem proves equality of the two reachability
  observations in its stated orientation.
- measured: a direct generic application of the imported public law.
- claimed: the theorem's public type and binder order are usable by a
  client at open arguments.
- the gap: this client application does not replace the catalog's
  generic checked proof.

### `stdlib/graphs/A-LEFT` and `stdlib/graphs/A-RIGHT`

- spec: §17 and §15's `graph_union` semantics.
- promise class: durable invariant.
- given: `vertex4_finite`, the three one-edge graphs, start V0, and
  predicate `is_v3`, with the two groupings in the law.
- expect: both reachability observations are `True`.
- measured: independent closed `Proved` rows for each grouping.
- claimed: both groupings reach the target through the three-edge
  chain.
- the gap: one finite chain does not establish the universally
  quantified theorem or all graph-union behavior.

### `stdlib/graphs/A-GH-MISS`

- spec: §17's Vertex4 discriminator and §1.2 reachability semantics.
- promise class: durable invariant.
- given: `vertex4_finite`, `g_only`, `h_only`, start V0, and `is_v3`.
- expect: reachability over `graph_union g_only h_only` is `False`,
  established by `Proved`.
- measured: the two-graph result in the accepted client.
- claimed: the first two graphs alone do not reach V3.
- the gap: this closed control does not establish that the three-edge
  union reaches V3; the two positive grouping rows establish that.

### `stdlib/graphs/A-LAW-CLOSED`

- spec: §17, `graph_union_reachable_assoc`.
- promise class: durable invariant.
- given: the same start, target, carrier, and three graphs, with each
  grouping.
- expect: `law_closed` applies the public theorem to the closed inputs.
- measured: a direct closed law application.
- claimed: the theorem's closed orientation and binder order are
  usable by clients.
- the gap: the closed application does not prove the generic law body.

### `stdlib/graphs/A-SAME`

- spec: §17, equality of the two reachability observations.
- promise class: durable invariant.
- given: the same start, target, carrier, and three graphs, with each
  grouping.
- expect: `groupings_agree` proves the same-form equality with
  `Proved`, without applying the public theorem.
- measured: an accepted equality control independent of the law.
- claimed: the closed groupings have equal reachability results.
- the gap: this control does not establish the generic law.

## Isolated refused claim

Use a fresh input containing the same complete `Vertex4` fixture and all
accepted rows above. Then append only `drop_k_claim`. The D0 refused source
is identified by SHA-256
`f9d74c340f9a087677624bc3e9a2ff309da042569a61bb086600c2af3374142f`.

```ken
import Data.Finite.Finite (Finite, MkFinite)

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

theorem drop_k_claim
    : Equal Bool
        (graph_reachable
          Vertex4
          vertex4_finite
          (graph_union Vertex4 (graph_union Vertex4 g_only h_only) k_only)
          V0
          is_v3)
        (graph_reachable Vertex4 vertex4_finite (graph_union Vertex4 g_only h_only) V0 is_v3) =
  Proved
```

### `stdlib/graphs/X-DROP-K`

- spec: §17 and §1.2's reachability semantics.
- promise class: durable invariant.
- given: a fresh input containing the accepted prefix and only the
  appended `drop_k_claim`.
- expect: `KernelRejected` at `drop_k_claim`'s own `Proved`.
- control: `left_grouping_reaches_v3` and `g_h_misses_v3` prove
  `True` and `False` at the two sides of the claim; the same-form
  `groupings_agree` control is accepted independently.
- measured: the three-edge reachability is `True`, the two-edge
  reachability is `False`, and the isolated `drop_k_claim` is
  `KernelRejected` at its own `Proved`.
- claimed: omitting `k_only` changes the reachability observation, so
  the claimed equality is false.
- the gap: this finite fixture discriminates the drop-k claim but does
  not establish the generic associativity law.
- why: V3 is reachable in the three-edge union but not in the union
  of `g_only` and `h_only`. The left side matches
  `left_grouping_reaches_v3`; the claimed right side matches
  `g_h_misses_v3`. Only the asserted equality between them is false.

## Coverage boundary

These rows check reachability under the two groupings of a three-way
union. They do not assert structural equality of `Graph` values or a
public edge-level regrouping law.

## Evidence boundary

Foundation's D0-CLEAN report `evt_7rfx18cbnj57e` records scratch checks
on base `b99258223`, including the two source blocks' acceptance/refusal.
Those measurements do not check this seed or an official conformance
run. The scratch clients identify the authored inputs; the spec child,
seed child, assembled candidate, CI, publication, landing, and landed
trust each have separate review or execution gates.

Checks: 4 fired: the D0 accepted client uses the stated public theorem at
open and closed arguments; the fixture and false claim use the exact
reached public operation. 8 fired: both grouped reachability rows prove
True, the two-graph control proves False, and the rejected same-form
claim differs only by omitting `k_only`.
