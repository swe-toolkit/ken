# Union of two directed graphs

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §15.
Ruling: Architect's SPEC-GRAPHS-UNION AC-0, released at `evt_7gczgbcdawtt1`.

Status: authored. The accepted client and refusals below are expected
conformance cases, not an official conformance-runner result. The closed
fixture exercises the six public laws on one three-vertex carrier; it does
not replace their generic checked proofs.

The carrier is `Vertex3`. `g_only` has only `V0 → V1`, and `h_only` has
only `V1 → V2`. Their union is `union_graph`. The edge cases below check
the Bool equation at one present and one absent edge. The inclusion laws
check one edge from each input. The edge-case function returns a
Type-sorted `Or`; its client below eliminates that data into an `Omega`
goal. The reachability controls show that the union reaches `V2` from
`V0`, while neither input alone does.

## Accepted client

Accept the complete client before either isolated refusal. The two
refusal arms each start from a fresh copy of this complete accepted
client, followed by that arm's same-form accepted control and then only
its refused claim.

```ken
import Core.Classes.LawfulClasses (bool_or)

import Core.Logic.Or (Inl, Inr)

import Data.Finite.Finite (Finite, MkFinite)

import Data.Collections.Derived (list_elem_head, list_elem_later)

import Algorithm.Graphs.Dependency
  (Graph,
    MkGraph,
    edge,
    graph_reachable,
    graph_union,
    graph_union_edge,
    graph_union_edge_left,
    graph_union_edge_right,
    graph_union_edge_cases,
    graph_union_reachable_left,
    graph_union_reachable_right)

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

const union_graph : Graph Vertex3 = graph_union Vertex3 g_only h_only

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

theorem union_edge_v1_v2_value : Equal Bool (edge Vertex3 union_graph V1 V2) True = Proved

theorem union_edge_v2_v0_value : Equal Bool (edge Vertex3 union_graph V2 V0) False = Proved

theorem union_edge_equation_v1_v2
    : Equal Bool
        (edge Vertex3 union_graph V1 V2)
        (bool_or (edge Vertex3 g_only V1 V2) (edge Vertex3 h_only V1 V2)) =
  graph_union_edge Vertex3 g_only h_only V1 V2

theorem union_edge_equation_v1_v2_value
    : Equal Bool
        (edge Vertex3 union_graph V1 V2)
        (bool_or (edge Vertex3 g_only V1 V2) (edge Vertex3 h_only V1 V2)) =
  Proved

theorem union_edge_equation_v2_v0
    : Equal Bool
        (edge Vertex3 union_graph V2 V0)
        (bool_or (edge Vertex3 g_only V2 V0) (edge Vertex3 h_only V2 V0)) =
  graph_union_edge Vertex3 g_only h_only V2 V0

theorem union_edge_equation_v2_v0_value
    : Equal Bool
        (edge Vertex3 union_graph V2 V0)
        (bool_or (edge Vertex3 g_only V2 V0) (edge Vertex3 h_only V2 V0)) =
  Proved

theorem g_only_v0_v1_value : Equal Bool (edge Vertex3 g_only V0 V1) True = Proved

theorem union_edge_left_v0_v1 : Equal Bool (edge Vertex3 union_graph V0 V1) True =
  graph_union_edge_left Vertex3 g_only h_only V0 V1 g_only_v0_v1_value

theorem union_edge_left_v0_v1_value : Equal Bool (edge Vertex3 union_graph V0 V1) True = Proved

theorem h_only_v1_v2_value : Equal Bool (edge Vertex3 h_only V1 V2) True = Proved

theorem union_edge_right_v1_v2 : Equal Bool (edge Vertex3 union_graph V1 V2) True =
  graph_union_edge_right Vertex3 g_only h_only V1 V2 h_only_v1_v2_value

theorem union_edge_right_v1_v2_value : Equal Bool (edge Vertex3 union_graph V1 V2) True = Proved

theorem union_edge_cases_v1_v2 : Equal Bool (edge Vertex3 union_graph V1 V2) True =
  match graph_union_edge_cases Vertex3 g_only h_only V1 V2 union_edge_v1_v2_value {
    Inl from_g ↦ absurd from_g;
    Inr from_h ↦ from_h
  }

theorem g_only_v0_reaches_v1_value
    : Equal Bool (graph_reachable Vertex3 vertex3_finite g_only V0 is_v1) True =
  Proved

theorem union_reachable_left_v0_v1
    : Equal Bool (graph_reachable Vertex3 vertex3_finite union_graph V0 is_v1) True =
  graph_union_reachable_left
    Vertex3
    vertex3_finite
    g_only
    h_only
    V0
    is_v1
    g_only_v0_reaches_v1_value

theorem union_reachable_left_v0_v1_value
    : Equal Bool (graph_reachable Vertex3 vertex3_finite union_graph V0 is_v1) True =
  Proved

theorem h_only_v1_reaches_v2_value
    : Equal Bool (graph_reachable Vertex3 vertex3_finite h_only V1 is_v2) True =
  Proved

theorem union_reachable_right_v1_v2
    : Equal Bool (graph_reachable Vertex3 vertex3_finite union_graph V1 is_v2) True =
  graph_union_reachable_right
    Vertex3
    vertex3_finite
    g_only
    h_only
    V1
    is_v2
    h_only_v1_reaches_v2_value

theorem union_reachable_right_v1_v2_value
    : Equal Bool (graph_reachable Vertex3 vertex3_finite union_graph V1 is_v2) True =
  Proved

theorem union_v0_reaches_v2_value
    : Equal Bool (graph_reachable Vertex3 vertex3_finite union_graph V0 is_v2) True =
  Proved

theorem g_only_v0_misses_v2_value
    : Equal Bool (graph_reachable Vertex3 vertex3_finite g_only V0 is_v2) False =
  Proved

theorem h_only_v0_misses_v2_value
    : Equal Bool (graph_reachable Vertex3 vertex3_finite h_only V0 is_v2) False =
  Proved
```

### `stdlib/graphs/union-edge-equation`

- spec: §15.1, `graph_union_edge`.
- promise class: durable invariant.
- given: `g_only`, `h_only`, and endpoints V1,V2 and V2,V0.
- expect: the public equation holds at both a present and absent edge.
- measured: each law application agrees with its independent `Proved`
  value; V1→V2 is present and V2→V0 is absent.
- claimed: union adjacency is pointwise `bool_or` of the input edges.
- gap: two closed endpoints do not establish the generic equation.
- why: the endpoints distinguish a disjoint input edge from an absent edge.

### `stdlib/graphs/union-edge-left`

- spec: §15.1, `graph_union_edge_left`.
- promise class: durable invariant.
- given: the true V0→V1 edge in `g_only`.
- expect: the law proves that V0→V1 is an edge of the union.
- measured: the application agrees with an independent closed `Proved`.
- claimed: each left-input edge is included in the union.
- gap: one edge does not establish inclusion for arbitrary inputs.
- why: this edge occurs only in the left input.

### `stdlib/graphs/union-edge-right`

- spec: §15.1, `graph_union_edge_right`.
- promise class: durable invariant.
- given: the true V1→V2 edge in `h_only`.
- expect: the law proves that V1→V2 is an edge of the union.
- measured: the application agrees with an independent closed `Proved`.
- claimed: each right-input edge is included in the union.
- gap: one edge does not establish inclusion for arbitrary inputs.
- why: this edge occurs only in the right input.

### `stdlib/graphs/union-edge-cases`

- spec: §15.1, `graph_union_edge_cases`.
- promise class: durable invariant.
- given: the true union edge V1→V2.
- expect: the public function returns an `Or` witness, which the
  client eliminates into an `Omega` goal to prove that edge.
- measured: the left branch is impossible; the right branch supplies
  the checked edge proof. An independent closed `Proved` checks the
  same conclusion.
- claimed: a true union edge has a true edge in at least one input.
- gap: this closed case does not establish the generic function.
- why: only `h_only` contains V1→V2, so the witness must select it.

### `stdlib/graphs/union-reachable-left-and-right`

- spec: §15.1, `graph_union_reachable_left` and
  `graph_union_reachable_right`.
- promise class: durable invariant.
- given: V0 reaches V1 in `g_only`; V1 reaches V2 in `h_only`.
- expect: each public law transports that reachability to the union.
- measured: both applications agree with independent closed `Proved`
  values. The union reaches V2 from V0; each input alone does not.
- claimed: reachability from either input graph is preserved in the
  union, and a path may use edges from both inputs.
- gap: these closed paths do not establish generic reachability
  monotonicity or its composition for every finite carrier.
- why: V0→V1→V2 uses one edge from each input; neither input contains
  the two-edge path.

## Isolated refused claims

For each arm, start from a fresh copy of the complete accepted client.
Append the arm's accepted same-form control, check that prefix accepts,
then append only its twin. The two refusals are independent; neither
prefix includes the other twin.

### `stdlib/graphs/reject-union-edge-present-as-absent`

- spec: §15.3, the V1→V2 discriminator.
- promise class: durable invariant.
- given: the accepted client and the true same-form claim for V1→V2.
- expect: `KernelRejected(TypeMismatch)` at this twin's own theorem
  span; do not pin the diagnostic code or reason string.
- measured: the same-form true claim accepts before the twin is added.
- claimed: the union includes V1→V2 from `h_only`.
- gap: this one false claim does not characterize every graph union.
- why: the edge is present in `h_only`, so the union's result is True.

Accepted same-form control, then append only the refused claim:

```ken
theorem t1_same_form_control : Equal Bool (edge Vertex3 union_graph V1 V2) True = Proved

theorem refuse_union_v1_v2_as_absent : Equal Bool (edge Vertex3 union_graph V1 V2) False =
  Proved
```

### `stdlib/graphs/reject-union-edge-absent-as-present`

- spec: §15.3, the V2→V0 discriminator.
- promise class: durable invariant.
- given: the accepted client and the false same-form claim for V2→V0.
- expect: `KernelRejected(TypeMismatch)` at this twin's own theorem
  span; do not pin the diagnostic code or reason string.
- measured: the same-form false claim accepts before the twin is added.
- claimed: an edge absent from both inputs is absent from the union.
- gap: this one false claim does not characterize every graph union.
- why: neither input contains V2→V0, so the union's result is False.

Accepted same-form control, then append only the refused claim:

```ken
theorem t2_same_form_control : Equal Bool (edge Vertex3 union_graph V2 V0) False = Proved

theorem refuse_union_v2_v0_as_present : Equal Bool (edge Vertex3 union_graph V2 V0) True =
  Proved
```

The twins pin complementary errors: the first is accepted by an
operation that drops the right input; the second is accepted by an
operation that returns True for every edge. AC-0 also measured a
Bool-and operation mutant. Its complete 4-by-9 refusal table belongs
to the spec's §15.3 discriminator; this seed does not claim that these
two twins alone distinguish every mutant from every other mutant.

## Evidence boundary

Architect's AC-0 measured the six declarations and private helper in
scratch, with no `Axiom` and equal trusted identity sets (114 → 114).
Its closed AC-2 client checked the union-only reachability, both same-
form controls, both refusals, and all 36 cells of the pre-registered
operation-mutant table. Those are scratch measurements.

CV checked this seed against a scratch overlay on base
`c565e0cf38094e43b3c7f2d2a27d15383be741d2`, applying the Architect's
product-cut patch (SHA256
`6eedb113dfb09c703f497939a6b072509eda0377d32f6ae31138da9c624133a8`).
The extracted accepted client passed `ken check`; each fresh-prefix twin
was `KernelRejected(TypeMismatch)` at its own theorem span. Logs:
`/workspaces/ken/tmp/cv-graphs-union-seed-accepted.log`,
`/workspaces/ken/tmp/cv-graphs-union-seed-twin1.log`, and
`/workspaces/ken/tmp/cv-graphs-union-seed-twin2.log`. A temporary
`.ken.md` copy passed `ken fmt --check`. These are local scratch-overlay
checks, not checks on the assembled candidate or an official
conformance-runner result. No CI, publication, landing, or landed-trust
measurement is claimed.
