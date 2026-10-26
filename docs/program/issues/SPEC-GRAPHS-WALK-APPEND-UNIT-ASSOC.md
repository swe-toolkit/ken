---
id: SPEC-GRAPHS-WALK-APPEND-UNIT-ASSOC
title: "Spec 62 section 3 joins walks with walk_append but states no law of it: its right unit holds only by computation, and a client cannot remove a leading WalkHere or regroup three joined walks. Rule the left-unit and associativity laws of walk_append from a checked D0 at zero trust, and state them as a section 4 with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 62 section 3 says neither a left-unit nor an associativity law is part of its contract. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md). Same shape as SPEC-LINEAR-ALGEBRA-SUM-COMM-ASSOC. Steward-filed per COORDINATION section 2."
---

# Walk append has a left unit and associates

## Objective

Spec 62 states, as public checked laws over
`Algorithm.Graphs.Dependency`, that `WalkHere` is a left unit of
`walk_append` and that `walk_append` is associative, for every graph
and every pair or triple of walks.

## Settled inputs (spec 62 §3, main `8cbcf9a60`)

- **The operation.** `walk_append q g s t u first second` recurses on
  `second`: `WalkHere` returns `first`, and `WalkStep prev end e
  prefix` re-applies `WalkStep` to the append of `first` and `prefix`
  (`Dependency.ken.md:91-104`).
- **The gap.** §3 says right `WalkHere` identity holds by computation,
  left identity does not hold by computation on an open second walk,
  and that neither a left-unit nor an associativity law is public.
  The package exports no law about `walk_append`.
- **Why they can be unconditional.** Both laws follow by induction on
  the last walk, with congruence through `WalkStep`. No decidable
  equality on `q` and no order premise is needed.
- **Seeds.** Under `conformance/stdlib/graphs/`, next to
  `seed-walk-composition-order.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §4 in spec 62 stating each ruled law by its public type, §3's
sentence on the absent laws updated to point at it, and a closed
conformance seed under `conformance/stdlib/graphs/`, with the status
lines changed in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules the
  names, argument order and public types of the two laws, the equality
  they are stated in over `Walk q g s u`, and which helpers stay
  private.
- **AC-1.** The §4 states each ruled law by its public type. The Spec
  leader and CV review it, with a Decision.
- **AC-2.** The seed checks both laws on closed walks over a small
  graph, including a `WalkHere` operand in each position of the
  associativity law. On a graph with two routes between the same
  endpoints, it has a refused block that claims a joined walk equal to
  the walk through the other intermediate vertex, and a same-input
  accepting control. The candidate's handoff shows a `ken check` rc on its
  blocks.

## Stop conditions

- A law that needs a new operation, a premise beyond §1 to §3's
  evidence, or a change to `walk_append`, `Walk` or any existing law,
  unless the D0 rules it.
- Any new primitive, kernel or surface feature, or function
  extensionality.

Checks: 4 fired: `walk_append` is `pub` at `Dependency.ken.md:91` on
`8cbcf9a60`, recursing on the second walk, and spec 62 §3 states that
no left-unit or associativity law is public. 8 fired: AC-2's
other-route refusal fails for a law stated up to endpoints only, which
a seed of `WalkHere` operands alone cannot catch.
