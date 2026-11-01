# Graph and walk reversal are involutions

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §10.
Frame: `docs/program/issues/SPEC-GRAPHS-REVERSE-INVOLUTION.md`.
Ruling: Architect AC-0 `evt_q8xc3yng3q7x`.

The closed client uses the three-vertex directed chain `V0 → V1 → V2`
and its two-edge walk. It applies both public laws. The `Refl` discriminator
refuses the false one-reversal graph equality and accepts the same-form
true two-reversal control. `Proved` is not the discriminator: AC-0 reports
that it refuses both graph equalities.

## Accepted client

Accept this complete client before testing the isolated refusal. It applies
the public graph and walk laws on `chain` and `chain_walk`.

```ken
import Algorithm.Graphs.Dependency
  (Graph, MkGraph, Walk, WalkHere, WalkStep, edge, graph_reverse,
   graph_reverse_involutive, walk_reverse, walk_reverse_involutive)

data Vertex3 : Type where {
  V0 : Vertex3;
  V1 : Vertex3;
  V2 : Vertex3
}

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

const chain_walk : Walk Vertex3 chain V0 V2 =
  WalkStep Vertex3 chain V0 V1 V2 Proved
    (WalkStep Vertex3 chain V0 V0 V1 Proved
      (WalkHere Vertex3 chain V0))

theorem graph_reverse_law_client :
  Equal
    (Graph Vertex3)
    (graph_reverse Vertex3 (graph_reverse Vertex3 chain))
    chain =
  graph_reverse_involutive Vertex3 chain

theorem graph_reverse_twice_refl_control :
  Equal
    (Graph Vertex3)
    (graph_reverse Vertex3 (graph_reverse Vertex3 chain))
    chain =
  Refl

theorem walk_reverse_law_client :
  Equal
    (Walk Vertex3 (MkGraph Vertex3 (edge Vertex3 chain)) V0 V2)
    (walk_reverse
      Vertex3
      (graph_reverse Vertex3 (MkGraph Vertex3 (edge Vertex3 chain)))
      V2
      V0
      (walk_reverse
        Vertex3
        (MkGraph Vertex3 (edge Vertex3 chain))
        V0
        V2
        chain_walk))
    chain_walk =
  walk_reverse_involutive
    Vertex3
    (edge Vertex3 chain)
    V0
    V2
    chain_walk

theorem walk_reverse_twice_refl_control :
  Equal
    (Walk Vertex3 (MkGraph Vertex3 (edge Vertex3 chain)) V0 V2)
    (walk_reverse
      Vertex3
      (graph_reverse Vertex3 (MkGraph Vertex3 (edge Vertex3 chain)))
      V2
      V0
      (walk_reverse
        Vertex3
        (MkGraph Vertex3 (edge Vertex3 chain))
        V0
        V2
        chain_walk))
    chain_walk =
  Refl
```

### `stdlib/graphs/graph-and-walk-reversal-involution`

- spec: `spec/50-stdlib/62-graphs.md` §10.
- promise class: durable invariant.
- given: `chain`, `chain_walk`, and the two public involution laws.
- expect: both public law applications and both closed `Refl` controls
  accept.
- measured: the graph theorem applies to `chain`; the walk theorem
  applies to its two-edge walk; each closed twice-reversed value equals
  its original.
- claimed: the public laws apply to the graph and walk clients at their
  specified types.
- gap: these closed clients do not prove either universally quantified
  law; the theorem types carry that contract.
- why: the walk has the checked edges `V0 → V1` and `V1 → V2`, and
  the accepted graph control reverses the chain twice.

## Isolated refused claim

Start from a fresh complete copy of the accepted client and append only
`seed_neg`. The claim uses the same graph carrier, endpoints, and `Refl`
form as `graph_reverse_twice_refl_control`, but reverses the graph once.
Refusal must occur at this `Refl`; do not pin a diagnostic.

### `stdlib/graphs/refuse-one-reversal-as-identity`

- spec: `spec/50-stdlib/62-graphs.md` §10 and §8.
- promise class: durable invariant.
- given: the accepted client and its two-reversal `Refl` control; append
  only the block below.
- expect: refuse equality of the reversed `chain` with `chain` at its
  own `Refl`.
- measured: `chain` has `V0 → V1` and not `V1 → V0`; the same-form
  two-reversal control accepts, while one reversal does not convert to
  `chain`.
- claimed: one reversal is not the identity on all directed graphs.
- gap: this chain refutes that false general claim; it does not prove
  the involution laws.
- why: the negative and accepted control differ only by one reversal;
  the asymmetric edge makes that difference observable.

```ken
theorem seed_neg :
  Equal
    (Graph Vertex3)
    (graph_reverse Vertex3 chain)
    chain =
  Refl
```

## Evidence boundary

Architect AC-0 checked the two public laws and their private helpers in
scratch on base `b46d344f09cf1779c4b16ee55cb5e24a0e0e6eca`, with rc 0,
no `Axiom`, and `trusted_base()` identity-equal at 114 → 114. The ruling
also records the accepted law applications and closed controls, and the
refusal of the one-reversal graph claim at its `Refl`. Those scratch
measurements are not package-test or official conformance-runner
execution of this seed. No official runner, CI, publication, landing, or
landed-trust result is claimed here.
