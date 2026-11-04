# Reachability is antisymmetric under a dependency order

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §12.
Frame: `docs/program/issues/SPEC-GRAPHS-REACHABILITY-ANTISYMMETRIC.md`.

The cases exercise a finite chain client of the public
`graph_reachable_antisymmetric` law. The accepted law application uses
reflexive reachability at `True`; the chain also supplies a positive
non-reflexive reachability control. The law refusal attempts to supply a
false reverse-reachability premise while using the same valid chain order.
The separate order-only refusal attempts to build an order on a two-cycle,
without applying the law. Refusals pin only `KernelRejected`, never
diagnostic text. These finite cases do not establish the generic law.

The Architect's AC-0 ruling (`evt_5q2nv0x3cp63w`, item 6) requires
`Proved` for order obligations, `λe. absurd e` for impossible edges, and
an order-only refusal beside the law refusal. It also records that `Refl`
is refused on the accepted control, so this seed does not use `Refl` as
an obligation or a discriminator. Foundation's D0 report and the
Architect's ruling are not CV execution of this seed. This seed has not
yet been run against an assembled candidate or the official conformance
runner.

## Shared client: one-way chain and checked order

The graph has the single edge `True → False`. Its order lists `True`
before `False`; the order's edge obligations use `Proved` for the actual
edge and `λe. absurd e` for absent edges.

```ken
import Core.Classes.LawfulClasses (DecEq_instance_Bool, bool_eq)

import Data.Finite.Finite (bool_finite)

import Algorithm.Graphs.Dependency
  (Graph, MkGraph, TopoOrder, MkTopoOrder, graph_reachable, graph_reachable_antisymmetric)

const chain : Graph Bool =
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

const chain_order : TopoOrder Bool DecEq_instance_Bool chain =
  MkTopoOrder
    Bool
    DecEq_instance_Bool
    chain
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

theorem true_reaches_true
    : Equal Bool (graph_reachable Bool bool_finite chain True (bool_eq True)) True =
  Proved

theorem true_reaches_false
    : Equal Bool (graph_reachable Bool bool_finite chain True (bool_eq False)) True =
  Proved

theorem accepted_chain_antisymmetry : Equal Bool True True =
  graph_reachable_antisymmetric
    Bool
    bool_finite
    DecEq_instance_Bool
    chain
    chain_order
    True
    True
    true_reaches_true
    true_reaches_true
```

### stdlib/graphs/reachability-antisymmetric-chain-positive

- spec: §12, `graph_reachable_antisymmetric`.
- promise class: durable invariant.
- given: the one-way chain, its checked `chain_order`, and the
  reflexive reachability proof at `True` for both law premises.
- expect: the law proves `Equal Bool True True`.
- control: `true_reaches_false` separately establishes the real forward
  edge's non-reflexive reachability on this chain.
- claimed: a concrete client can apply the public law with a checked
  order and both required reachability premises.
- the gap: this diagonal instance does not establish the generic law or
  its behavior on arbitrary carriers and graphs.

### stdlib/graphs/reachability-antisymmetric-forward-control

- spec: §1.2, reachability semantics.
- promise class: durable invariant.
- given: the same carrier, finite certificate, graph, and checked order.
- expect: `True` reaches `False` along the graph's actual edge.
- claimed: the chain fixture exercises non-reflexive reachability.
- the gap: this closed edge does not establish generic completeness.

## Isolated law refusal on the valid chain

Append only this theorem to the shared client. Its proposed order is the
accepted `chain_order`; the second reachability premise is the accepted
forward control. The first premise falsely claims that `False` reaches
`True`. Refusal is asserted only as `KernelRejected`; no diagnostic text
or code is part of the oracle.

```ken
theorem refuse_false_reverse_premise : Equal Bool False True =
  graph_reachable_antisymmetric
    Bool
    bool_finite
    DecEq_instance_Bool
    chain
    chain_order
    False
    True
    Proved
    true_reaches_false
```

### stdlib/graphs/reachability-antisymmetric-refuses-false-premise

- spec: §12 and §1.2, reachability semantics.
- promise class: durable invariant.
- given: the shared client and its valid chain order.
- expect: the isolated law application is refused with error class
  `KernelRejected`.
- control: `true_reaches_false` proves the forward leg on the same
  chain; the order is independently accepted by the positive client.
- claimed: the law cannot be applied with a fabricated reverse
  reachability proof, even when its order premise is valid.
- the gap: this one false premise does not establish every invalid law
  application is refused.

## Isolated order-only refusal on a two-cycle

Append only this graph and attempted certificate to the shared client.
This block constructs no law application. The cycle has both opposite
edges, so the proposed `[True, False]` order fails its forward-edge
obligation for `False → True`. Refusal pins only `KernelRejected`.

```ken
const two_cycle : Graph Bool =
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
          False ↦
            match v {
              True ↦ True;
              False ↦ False
            }
        })

const refuse_order_on_two_cycle : TopoOrder Bool DecEq_instance_Bool two_cycle =
  MkTopoOrder
    Bool
    DecEq_instance_Bool
    two_cycle
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
              True ↦ λe. Proved;
              False ↦ λe. absurd e
            }
        })
```

### stdlib/graphs/reachability-antisymmetric-order-only-refusal

- spec: §1.3, dependency-order certificate; §12,
  `graph_reachable_antisymmetric`.
- promise class: durable invariant.
- given: the two-cycle and a proposed order with the same list as the
  accepted chain order.
- expect: the isolated order construction is refused with error class
  `KernelRejected`.
- control: the shared client constructs an order with the same list on
  the one-way chain, where the only actual edge points forward.
- claimed: the order failure is independently observable without
  invoking the antisymmetry law.
- the gap: this fixed two-cycle and list do not establish the generic
  order theorem.

## Evidence boundary

The Architect's AC-0 ruling reports the generic law's D0 proof rows,
including accepted and refused cases and unchanged trusted identities;
Foundation reports the scratch D0 harness and regression results. Those
are attributed reports, not CV runs. CV has authored these seed cases but
has not run them against an assembled candidate, run package tests, or
executed the official conformance runner. The finite examples do not
replace the generic checked law, establish a landed trust result, or
imply CI, publication, or landing.
