# Pointwise-equal edges give equal reachability

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §24.
Spec child: `243f00f8600bea58d0bc9ec8a9964279d40b5b89`.
Architect AC-0: `evt_553g793zejmhh`.
Base: `9720876d5bd3d298273d40c3322aa1295ba5dc23`.

The law says that pointwise-equal edge relations give equal reachability
for the same finite carrier, start and target. It does not equate the
`Graph` values themselves. The accepted client is open in the graph
parameter `b`, start `s`, and target predicate `target`.

The frame's closed-`b`, start-`True` form is deliberately not the
accepted law-dependent row: Architect AC-0 measured it as convertible
by `Refl`, even with `target` open. In the accepted row, `b`, `s`, and
`target` remain open, so the public law application is load-bearing.

## Accepted block and supporting controls

The five supporting controls in the block do not cite the new law:
`meet_edges_agree`, the two closed reachability values, the one-way edge
inclusion, and `equal_reach_refuted`. The last control turns a hypothetical
equality into `Equal Bool False True`; it is what makes the false twin's
claim independently refuted rather than relying on its refusal.

```ken
import Data.Finite.Finite (bool_finite)
import Core.Classes.LawfulClasses (bool_and, bool_not)
import Core.Logic.Transport (cong, sym, trans)
import Algorithm.Graphs.Dependency
  (Graph, MkGraph, edge, graph_reachable, graph_edgeless, graph_reachable_edge_equiv)

fn left_meet (b : Bool) : Graph Bool = MkGraph Bool (λu. λv. bool_and b (bool_and u v))

fn right_meet (b : Bool) : Graph Bool = MkGraph Bool (λu. λv. bool_and (bool_and v u) b)

theorem meet_edges_agree (b : Bool) (u : Bool) (v : Bool)
    : Equal Bool (edge Bool (left_meet b) u v) (edge Bool (right_meet b) u v) =
  match b {
    True ↦
      match u {
        True ↦ match v { True ↦ Proved; False ↦ Proved };
        False ↦ match v { True ↦ Proved; False ↦ Proved }
      };
    False ↦
      match u {
        True ↦ match v { True ↦ Proved; False ↦ Proved };
        False ↦ match v { True ↦ Proved; False ↦ Proved }
      }
  }

fn is_true (v : Bool) : Bool = v

const single_step : Graph Bool = MkGraph Bool (λu. λv. bool_and (bool_not u) v)

theorem single_step_reaches_true
    : Equal Bool (graph_reachable Bool bool_finite single_step False is_true) True =
  Proved

theorem edgeless_misses_true
    : Equal Bool (graph_reachable Bool bool_finite (graph_edgeless Bool) False is_true) False =
  Proved

theorem edgeless_included_in_single_step (u : Bool) (v : Bool)
    : Equal Bool (edge Bool (graph_edgeless Bool) u v) True
      → Equal Bool (edge Bool single_step u v) True =
  λabsurd.
    cong
      Bool
      Bool
      False
      True
      (λx. match x { False ↦ edge Bool single_step u v; True ↦ True })
      absurd

theorem equal_reach_refuted
    : Equal
        Bool
        (graph_reachable Bool bool_finite (graph_edgeless Bool) False is_true)
        (graph_reachable Bool bool_finite single_step False is_true)
      → Equal Bool False True =
  λsame.
    trans
      Bool
      False
      (graph_reachable Bool bool_finite (graph_edgeless Bool) False is_true)
      True
      (sym
        Bool
        (graph_reachable Bool bool_finite (graph_edgeless Bool) False is_true)
        False
        edgeless_misses_true)
      (trans
        Bool
        (graph_reachable Bool bool_finite (graph_edgeless Bool) False is_true)
        (graph_reachable Bool bool_finite single_step False is_true)
        True
        same
        single_step_reaches_true)

theorem meet_reach_agree (b : Bool) (s : Bool) (target : Bool → Bool)
    : Equal
        Bool
        (graph_reachable Bool bool_finite (left_meet b) s target)
        (graph_reachable Bool bool_finite (right_meet b) s target) =
  graph_reachable_edge_equiv Bool bool_finite (left_meet b) (right_meet b) s target (meet_edges_agree b)
```

### `stdlib/graphs/reachability-edge-equiv-controls`

- spec: §24 and §1.2.
- promise class: durable invariant.
- given: the five supporting controls in the accepted block, with no
  application of `graph_reachable_edge_equiv` in their bodies.
- expect: `meet_edges_agree` accepts by matching on `b`, `u`, and `v`;
  `single_step_reaches_true` yields `True`; `edgeless_misses_true`
  yields `False`; the inclusion accepts; and `equal_reach_refuted`
  derives `Equal Bool False True` from the hypothetical reachability
  equality.
- measured: Architect AC-0 checked the accepted block in scratch.
- claimed: for the closed source and target control, the edgeless graph
  misses `True` while the graph with the single edge reaches it.
- gap: these controls do not prove the generic edge-equivalence law.
- why: the two closed reachability results differ, while the inclusion
  and contradiction control make the false twin's counterexample
  independent of its refusal.

### `stdlib/graphs/reachability-edge-equiv-open`

- spec: §24, `graph_reachable_edge_equiv`.
- promise class: durable invariant.
- given: the accepted block with `b`, `s`, and `target` open.
- expect: the block accepts; `meet_reach_agree` applies the public law
  using `meet_edges_agree b`.
- measured: Architect AC-0 checked this open client in scratch.
- claimed: for these two `Bool` graphs, pointwise edge equality implies
  equal reachability at every open start and target.
- gap: this instance does not check the generic law's proof body or
  other carriers.
- controls: the five supporting controls above do not cite the law.
- why: the edge expressions remain dependent on open `b`, and the
  conclusion remains open in `s` and `target`; computation alone does
  not close the row.

## The open row does not close by computation

For each trial, start from a fresh copy of the accepted block and change
only the body of `meet_reach_agree`. Keep `b`, `s`, and `target` open.

```ken
theorem meet_reach_agree (b : Bool) (s : Bool) (target : Bool → Bool)
    : Equal
        Bool
        (graph_reachable Bool bool_finite (left_meet b) s target)
        (graph_reachable Bool bool_finite (right_meet b) s target) =
  Proved
```

### `stdlib/graphs/reachability-edge-equiv-proved-refusal`

- spec: §24.
- promise class: durable invariant.
- given: a fresh accepted block with the open row closed by `Proved`.
- expect: `KernelRejected(TypeMismatch)` at the theorem.
- measured: Architect AC-0 observed this refusal in scratch.
- claimed: computation alone does not establish the open reachability
  equality.
- gap: this does not characterize other possible proofs of the law.
- why: the graph edges remain dependent on open `b`.

```ken
theorem meet_reach_agree (b : Bool) (s : Bool) (target : Bool → Bool)
    : Equal
        Bool
        (graph_reachable Bool bool_finite (left_meet b) s target)
        (graph_reachable Bool bool_finite (right_meet b) s target) =
  Refl
```

### `stdlib/graphs/reachability-edge-equiv-refl-refusal`

- spec: §24.
- promise class: durable invariant.
- given: a fresh accepted block with the open row closed by `Refl`.
- expect: elaborator `TypeMismatch` at the `Refl` token because the
  reachability endpoints are not convertible.
- measured: Architect AC-0 observed this refusal and its token span in
  scratch.
- claimed: the open conclusion is not reflexive by conversion.
- gap: this does not characterize other possible proofs of the law.
- why: `Refl` requires convertible endpoints; the two open graph
  expressions do not reduce to the same Boolean.

## Refuse equality from only one-way inclusion

Start from a fresh copy of the accepted prefix without
`meet_reach_agree`, then append only the twin below. The prefix includes
the five controls above.

```ken
theorem refused_inclusion_reach_equal (s : Bool) (target : Bool → Bool)
    : Equal
        Bool
        (graph_reachable Bool bool_finite (graph_edgeless Bool) s target)
        (graph_reachable Bool bool_finite single_step s target) =
  graph_reachable_edge_equiv
    Bool
    bool_finite
    (graph_edgeless Bool)
    single_step
    s
    target
    edgeless_included_in_single_step
```

### `stdlib/graphs/reachability-edge-equiv-one-way-inclusion-refusal`

- spec: §24; one-way edge inclusion establishes monotonicity, not
  equality.
- promise class: durable invariant.
- given: the fresh accepted prefix followed by the one-way-inclusion
  twin, open in `s` and `target`.
- expect: `KernelRejected(TypeMismatch)` at declaration granularity.
- measured: Architect AC-0 observed this refusal at the twin
  declaration in scratch.
- claimed: the public equality law cannot be applied with only one-way
  inclusion.
- gap: the refusal alone would not establish that the claimed equality
  is false.
- controls: `equal_reach_refuted` derives `Equal Bool False True` from
  the claimed equality, using the two closed reachability controls.
- why: the law requires pointwise edge equality; the supplied witness
  proves only edgeless-edge inclusion in `single_step`.

```ken
theorem refused_inclusion_reach_equal (s : Bool) (target : Bool → Bool)
    : Equal
        Bool
        (graph_reachable Bool bool_finite (graph_edgeless Bool) s target)
        (graph_reachable Bool bool_finite single_step s target) =
  Proved
```

### `stdlib/graphs/reachability-edge-equiv-one-way-proved-refusal`

- spec: §24, especially the equality result.
- promise class: durable invariant.
- given: a fresh accepted prefix without `meet_reach_agree`, followed by
  the same false equality claim, closed directly by `Proved`.
- expect: `KernelRejected(TypeMismatch)`.
- measured: Architect AC-0 observed this refusal in scratch.
- claimed: the false equality is not derivable by computation.
- gap: this one source and target do not characterize all false
  reachability equalities.
- controls: `single_step_reaches_true`, `edgeless_misses_true`, and
  `equal_reach_refuted` establish the counterexample independently.
- why: for `s = False` and `target = is_true`, the reachability results
  are `False` and `True`.

## Evidence boundary

Architect AC-0 (`evt_553g793zejmhh`) reports that the law is public
Transparent and checked in scratch on the exact landed base
`9720876d5bd3d298273d40c3322aa1295ba5dc23`. After loading the eleven
providers and then `Dependency`, `trusted_base()` had equal exact
`GlobalId` sets (114 → 114); no new `Axiom` was added. AC-0 also checked
the generic client, accepted open row, both compute-only refusals, and
the one-way-inclusion twin. Its mutation of the second inclusion made
`Dependency` fail to load with `KernelRejected(TypeMismatch)`; the
source was restored byte-identically.

These are scratch observations, not execution of this authored Markdown
seed. On the exact assembled candidate
`e8938dbcf8a791683f654f366a8f84849f6f523c`, Foundation QA reports that
the `cat_graphs_reachability_edge_equiv` AC-T target passed 4/4 and that
this Markdown seed was parsed and exercised as package-test input
(`evt_74trr18cnrtd4`). This is package-test evidence, not official
conformance-runner execution. The Markdown seed has not been executed by
the official conformance runner. CV ran no tests. No CI, publication,
landing, or measured landed-trust result is claimed.

Clean-room provenance: no `local/refs/` or external reference
implementations were consulted. This seed is grounded in the settled
§24 child, Architect AC-0, and first principles.
