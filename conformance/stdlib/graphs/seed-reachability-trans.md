# Reachability is transitive

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §11, in Spec child
`2c226ec2d7aa5cee2515fc2eb01bf1d68e79e34d`.
Frame: `docs/program/issues/SPEC-GRAPHS-REACHABILITY-TRANS.md`.

Status: authored. The closed rows exercise one finite instance of the
public §11 law; they do not replace its generic checked proof. No
official conformance-runner result is claimed.

The graph is the one-way chain `V0 → V1 → V2`. The first leg asks
whether `V0` reaches the vertex selected by `vertex3_eq V1`; this is
the supplied `DecEq` instance's `d.eq V1` predicate. The second leg
asks whether `V1` reaches `is_v2`. The transitivity law composes those
facts. On this same graph, `V2` does not reach `is_v0`.

## Accepted shared client

The following whole client accepts. Its first and second legs are
separately stated, and `v0_reaches_v2_trans` applies the public law to
them. The same-form control independently proves the forward result.

```ken
import Core.Classes.LawfulClasses (DecEq)

import Data.Finite.Finite (Finite, MkFinite)

import Data.Collections.Derived (list_elem_head, list_elem_later)

import Algorithm.Graphs.Dependency (Graph, MkGraph, graph_reachable, graph_reachable_trans)

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

fn is_v2 (v : Vertex3) : Bool =
  match v {
    V0 ↦ False;
    V1 ↦ False;
    V2 ↦ True
  }

fn is_v0 (v : Vertex3) : Bool =
  match v {
    V0 ↦ True;
    V1 ↦ False;
    V2 ↦ False
  }

theorem v0_reaches_v1
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 (vertex3_eq V1)) True =
  Proved

theorem v1_reaches_v2
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V1 is_v2) True =
  Proved

theorem v0_reaches_v2_trans
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 is_v2) True =
  graph_reachable_trans
    Vertex3
    vertex3_finite
    DecEq_instance_Vertex3
    chain
    V0
    V1
    is_v2
    v0_reaches_v1
    v1_reaches_v2

theorem v0_reaches_v2_same_form_control
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V0 is_v2) True =
  Proved

theorem v2_misses_v0
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 is_v0) False =
  Proved
```

### stdlib/graphs/reachability-trans-first-leg

- spec: §11, `graph_reachable_trans`.
- promise class: durable invariant.
- given: the finite chain and the `DecEq` predicate for `V1`.
- expect: `V0` reaches `V1`.
- why: this is the first law premise, with the instance's `d.eq V1`.

### stdlib/graphs/reachability-trans-second-leg

- spec: §11, `graph_reachable_trans`.
- promise class: durable invariant.
- given: the finite chain and `is_v2`.
- expect: `V1` reaches `V2`.
- why: this is the second law premise on the same chain.

### stdlib/graphs/reachability-trans-composes-chain

- spec: §11, `graph_reachable_trans`.
- promise class: durable invariant.
- given: the accepted first and second legs above.
- expect: `graph_reachable_trans` proves that `V0` reaches `is_v2`.
- why: this is a closed instance of the public generic law, not a
  substitute for its generic checked proof.

### stdlib/graphs/reachability-trans-same-form-control

- spec: §11, `graph_reachable_trans`.
- promise class: durable invariant.
- given: the same chain and `Proved` proof form as the false reverse
  claim below.
- expect: `V0` reaches `is_v2`.
- why: the control establishes that the proposition shape and proof
  form accept for this true forward claim.

### stdlib/graphs/reachability-trans-reverse-value

- spec: §1.2, reachability semantics.
- promise class: durable invariant.
- given: the chain and `is_v0` target.
- expect: `V2` does not reach `is_v0`.
- why: the directed chain has no route from `V2` back to `V0`.

## Isolated refused claim

On a fresh copy of the accepted client above, append only the theorem
below. It has the same `Equal Bool` and `Proved` form as the accepted
control; only the start and target vertices change. The false reverse
claim must be refused at its own theorem body, not at an import or
fixture. The expected rejection is a type mismatch; no diagnostic
code is pinned.

### stdlib/graphs/reject-reverse-reachability-on-chain

- spec: §11, and §1.2's reachability semantics.
- promise class: durable invariant.
- given: the accepted client, with the same one-way chain.
- expect: `KernelRejected TypeMismatch` at the claim that `V2`
  reaches the target `is_v0`; no diagnostic code is pinned.
- control: `V0` reaches `is_v2` with the same `Proved` form.
- why: transitivity composes forward reachability; it does not reverse
  the chain.

Append only this theorem to a fresh copy of the accepted client:

```ken
theorem refuse_v2_reaches_v0
    : Equal Bool (graph_reachable Vertex3 vertex3_finite chain V2 is_v0) True =
  Proved
```

## Evidence boundary

The Architect's AC-0 D0 report (`evt_bvjs9ftn3aw`) measured the generic
proof on scratch base `84b15bb0984e7ba7218da33adf50118b07a7a271`,
with no added `Axiom` and equal trusted identity sets (114 → 114).
It also measured the accepted forward control and refusal of the false
reverse claim. The report says `Refl` refused both `Equal Bool` forms,
so `Proved`, not `Refl`, is the discriminator. These are Architect-
reported scratch results, not CV test runs or official conformance-
runner execution. The finite seed cases do not establish the generic
law or a landed trust result.
