# Reversing a topological order reverses its vertex list

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §21.

## Shared accepted prefix

The only edge of `forward` is from `True` to `False`; its order lists
`[True, False]`. The prefix checks the open transfer and vertex law, the
open permutation law, and the flipped edge and its precedence on the
transferred order. Check each row below independently with this prefix.

```ken
import Core.Classes.LawfulClasses (DecEq, DecEq_instance_Bool)

import Data.Collections.Derived as DC

import Core.Logic.Transport (trans)

import Algorithm.Graphs.Dependency
  (Graph,
    MkGraph,
    edge,
    graph_reverse,
    before,
    TopoOrder,
    MkTopoOrder,
    ordered_vertices,
    topo_order_reverse,
    topo_order_reverse_vertices)

fn topo_order_reverse_open
      (q : Type) (d : DecEq q) (g : Graph q) (order : TopoOrder q d g)
    : TopoOrder q d (graph_reverse q g) =
  topo_order_reverse q d g order

theorem topo_order_reverse_vertices_open
      (q : Type) (d : DecEq q) (g : Graph q) (order : TopoOrder q d g)
    : Equal
        (List q)
        (ordered_vertices q d (graph_reverse q g) (topo_order_reverse q d g order))
        (DC.reverse q (ordered_vertices q d g order)) =
  topo_order_reverse_vertices q d g order

theorem perm_open
      (a : Type) (eqf : a → a → Bool) (xs : List a)
    : DC.Perm a eqf xs (DC.reverse a xs) =
  DC.reverse::perm a eqf xs

const forward : Graph Bool =
  MkGraph
    Bool
    (λu.
      λv.
        match u {
          True ↦
            match v {
              True ↦ False;
              False ↦ True
            };
          False ↦ False
        })

const forward_order : TopoOrder Bool DecEq_instance_Bool forward =
  MkTopoOrder
    Bool
    DecEq_instance_Bool
    forward
    (Cons Bool True (Cons Bool False (Nil Bool)))
    Proved
    (λx.
      match x {
        True ↦ Proved;
        False ↦ Proved
      })
    (λx.
      λy.
        match x {
          True ↦
            match y {
              True ↦ λe. absurd e;
              False ↦ λe. Proved
            };
          False ↦
            match y {
              True ↦ λe. absurd e;
              False ↦ λe. absurd e
            }
        })

const reversed_order : TopoOrder Bool DecEq_instance_Bool (graph_reverse Bool forward) =
  topo_order_reverse Bool DecEq_instance_Bool forward forward_order

theorem reversed_edge : Equal Bool (edge Bool (graph_reverse Bool forward) False True) True =
  Proved

theorem reversed_before
    : Equal Bool
        (before
          Bool
          DecEq_instance_Bool
          False
          True
          (ordered_vertices
            Bool
            DecEq_instance_Bool
            (graph_reverse Bool forward)
            reversed_order))
        True =
  Proved
```

## stdlib/graphs/topo-order-reverse-vertices

- spec: `spec/50-stdlib/62-graphs.md` §21.
- promise class: durable invariant.
- given: append this claim to the shared prefix.
- expect: accepts the vertex list `[False, True]` through the public law.
- why: the transfer reverses the list; retaining the same set of vertices
  alone does not suffice.

```ken
theorem reversed_list
    : Equal
        (List Bool)
        (ordered_vertices Bool DecEq_instance_Bool (graph_reverse Bool forward) reversed_order)
        (Cons Bool False (Cons Bool True (Nil Bool))) =
  trans
    (List Bool)
    (ordered_vertices Bool DecEq_instance_Bool (graph_reverse Bool forward) reversed_order)
    (DC.reverse Bool (ordered_vertices Bool DecEq_instance_Bool forward forward_order))
    (Cons Bool False (Cons Bool True (Nil Bool)))
    (topo_order_reverse_vertices Bool DecEq_instance_Bool forward forward_order)
    Refl
```

## stdlib/graphs/topo-order-reverse-unreversed-refuses

- spec: `spec/50-stdlib/62-graphs.md` §21.
- promise class: durable invariant.
- given: append this claim to a fresh copy of the prefix. Relative to the
  accepted claim, change only the claimed list (in its type and proof).
- expect: refuses with a type mismatch at the closing `Refl`, not at the
  public law application or earlier in the client.
- why: the reversed list is not convertible to `[True, False]`.

```ken
theorem reversed_list
    : Equal
        (List Bool)
        (ordered_vertices Bool DecEq_instance_Bool (graph_reverse Bool forward) reversed_order)
        (Cons Bool True (Cons Bool False (Nil Bool))) =
  trans
    (List Bool)
    (ordered_vertices Bool DecEq_instance_Bool (graph_reverse Bool forward) reversed_order)
    (DC.reverse Bool (ordered_vertices Bool DecEq_instance_Bool forward forward_order))
    (Cons Bool True (Cons Bool False (Nil Bool)))
    (topo_order_reverse_vertices Bool DecEq_instance_Bool forward forward_order)
    Refl
```

The package test checks these authored clients. It does not execute the
seed through the official conformance runner.
