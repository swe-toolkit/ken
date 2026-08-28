---
id: RT-FRESH-RESULT-ROUTE-PAIRING-LEG-CONTROLS
title: "The fresh-result route pairing proof pins ONE of its five substantive conjuncts — CheckedIhFreshResultRouteObservationMutation has exactly two arms and suppresses only source_result_value, so the sink-half identity (header_input_value == ret_input_value) and three sibling conjuncts have no negative control at all and can be deleted with the whole suite green. Latent today; the directed-edge claim that distinguishes this node from the co-emission object it replaced is asserted in the positive alone."
status: draft
owner: runtime
size: S
gate: none
tier: T2
depends_on: [RT-CHECKED-IH-FRESH-RESULT-ROUTE]
blocks: []
github: null
origin: "Adversary M8 hunt evt_39yvk4d78cfr (thr_3q2mw0qb0xcq8), 2026-08-28, on squash 7d36d24f04678d3c9a2636fb06fd8c7aaf5dfb89 (range bd4ddf213..208309bb1, eight paths, squash-diff byte-identical to range-diff). The hunt's verdict on that candidate was CLEAN core with mostly strong controls; this is its ONE grounded finding, classed leak/gap and explicitly LATENT, advisory and non-blocking. Routed to the Steward for disposition rather than reopening the merged node. Every coordinate below was re-measured by the Steward against the landed tree before filing. Steward framing per COORDINATION section 2."
---

> # NOT AN AMENDMENT TO THE MERGED NODE, AND NOT WORK FOR THE D3 CONSUMER
>
> `RT-CHECKED-IH-FRESH-RESULT-ROUTE` is `merged` at `7d36d24f0` and stays merged.
> Nothing here reopens it, and nothing here is a product defect: the finding is a
> **control gap**, not a miscompile.
>
> It is also **not** folded into `RT-RESULT-CONTINUATION-BINDING-PROVENANCE`.
> That node is frozen, carries nine hard stops and three suppression axes, and
> bundling a control obligation into it repeats precisely the defect
> `AC-DERIVE` was recut to remove. **A criterion added to a frozen node is one
> nobody reads.**
>
> **CONTENTION: this node edits the same two files D3A+D3B will edit**
> (`rt_parity_native.rs`, `lowering/mod.rs`). It is QUEUED BEHIND D3 and must not
> be released concurrently with it.

## The finding, re-measured by the Steward at the landed tree

The change's headline claim — the one that distinguishes the route object from
the `CheckedIhFreshResultProducer` it replaced — is that the certified tail route
is **a directed value-flow edge rather than four co-emitted endpoints**. HS9's
whole diagnosis was that co-emission is not pairing.

The dynamic proof of that claim is
`checked_ih_fresh_result_route_observation_is_forward_and_paired`
(`crates/ken-cli/tests/rt_parity_native.rs:1136`). Its `paired` predicate
(`:1149`) is a conjunction. Five of its conjuncts are substantive (the other two
are `.is_some()` seat checks):

| # | line | conjunct | negative control |
|---|---|---|---|
| 1 | `:1152` | `source_result_value == active_edge_value` (source -> active edge) | **YES** |
| 2 | `:1153` | `active_answer_route == Some("CheckedSelectedRecursor")` | none |
| 3 | `:1155` | `header_input_value == ret_input_value` (header -> Ret-input SINK) | none |
| 4 | `:1156` | `actual_ret_case_body_origin == Some(expected_ret_case_body_origin)` | none |
| 5 | `:1158` | the `matches!` forward-order chain `selected < source < active < ret` | none |

The only negative control is `CheckedIhFreshResultRouteObservationMutation`
(`crates/ken-runtime/src/cranelift_backend/lowering/mod.rs:9618`), whose arms are
exactly `{Exact, CoEmissionOnly}`. **Its own doc-comment states the scope:**
*"preserve all four co-emitted seats while deleting only the value-identity
pairing between the governed result and active edge"* — conjunct 1. The recorder
(`mod.rs:9731-9733`) suppresses only `source_result_value` under
`coemission_only`.

**Why that leaves four conjuncts unpinned, and it is a deduction, not a guess.**
Delete conjunct 3 (replace `:1155` with `&& true`) and the whole suite stays
green:

- `Exact` asserts `all(paired)`. Dropping a conjunct only **weakens** `paired`,
  and a weaker predicate never flips a pass to a fail.
- `CoEmissionOnly` asserts `all(!paired)`. Under that arm `source_result_value`
  is `None`, so conjunct 1 (`:1152`) is already false and `paired` is false
  **regardless of conjunct 3** — so `!paired` still holds.

The same argument runs unchanged for conjuncts 2, 4 and 5. **One nulling arm
keyed on one seat cannot discriminate any conjunct that does not read that
seat.**

The negative assertion (`:1179-1188`) does check that the leg-2 seats are
`.is_some()` — but `.is_some()` is a co-emission check. It never establishes that
their **identity** can break, which is the entire property at issue.

## Why this is LATENT and not a live defect

The positive genuinely reads production on both sides. `header_input_value` is
`block_params(header)[0]` (`lowering/core.rs:12218`) and `ret_input_value` is
`scrutinee.word` (`core.rs:12636`) — independent SSA reads that would be unequal
if production misrouted. **A real sink bug fails `Exact` today.** Nothing in the
tree is miscompiling and nothing is reverted.

What is missing is the **symmetric meta-control the design implicitly claims**:
proof that the sink discriminator has power. The merged node's retro credits
conjunct 3 to "body-merge substitution", but that is `RouteBodyMergeOutput` — a
**static** certificate-field rejection in the confluence test. It never exercises
the **dynamic** emission identity. So a later refactor that made
`ret_input_value` derive tautologically from `header_input_value` would pass
every control in the tree.

> **THE REUSABLE FORM, and it is why this is worth a node rather than a note.**
> A conjunctive predicate's controls must be counted **per conjunct**, never per
> predicate. A single negative arm makes the whole `paired` predicate *look*
> two-sided, and the summary sentence "the pairing proof has a negative control"
> is TRUE while four fifths of it is unpinned. **The tell is available by
> inspection and costs nothing: for each conjunct, name the arm that reddens when
> only that conjunct is deleted.** An arm that cannot name one pins nothing.
>
> This is the same family as the two gate rejects on
> `RT-RETAINED-UNIT-RESULT-CLOSURE-REPRESENTATION` — a control that cannot fail
> is not weaker evidence, it is none — applied one level down, to a conjunct
> rather than to a whole arm.

## Deliverables

- **D1 — a discriminating negative arm for every substantive conjunct.** Extend
  `CheckedIhFreshResultRouteObservationMutation` with arms that suppress each
  conjunct's identity **while preserving co-emission of its seats**, asserted
  exactly as `CoEmissionOnly` is. The sink arm (conjunct 3, e.g.
  `RetInputCoEmissionOnly`) is the one the hunt named; conjuncts 2, 4 and 5 are
  in scope on the same argument and are **not** a follow-up.

## Acceptance criteria

- **AC-PER-CONJUNCT (predicate form, and it is the operative one).** For **every
  substantive conjunct of `paired`**, there exists a negative arm that reddens
  the suite when that conjunct alone is deleted. The five-row table above is
  **non-exhaustive illustration, not the roster** — if `paired` gains a conjunct,
  it gains this obligation. Do not satisfy this by enumerating five arms and
  stopping; satisfy it by making the property hold.

  > Framed as a predicate deliberately. An enumerated roster is exactly what let
  > the sibling node's `AC-NO-SYNTHESIS` miss the traversal-root axis, and what
  > let this gap exist in the first place: `CoEmissionOnly` is a correct arm for
  > the conjunct it names and says nothing about the four it does not.

- **AC-DISCRIMINATES.** Each new arm must be proved to discriminate **by
  mutation**, two-sided: with the arm active the suite must RED at the named
  assertion, and restoring must return the exact prior green. An arm asserted
  only in the positive direction repeats the defect being fixed.

- **AC-NOT-MANUFACTURED.** Each arm must break the **identity** while leaving
  both seats co-emitted and `.is_some()`. An arm that nulls a seat outright is
  testing co-emission, not pairing, and does not discharge its conjunct — that
  distinction is the whole content of HS9.

- **AC-POSITIVE-INTACT.** `Exact`'s `all(paired)` and the existing
  `CoEmissionOnly` behaviour must be byte-unchanged in effect. This node adds
  discrimination; it does not weaken, relax, or re-scope the landed positive.

- **AC-AFFECTED-CLOSURE.** Cover every target that loads any module whose closure
  this increment changes, diff-touched or not. `lowering/mod.rs` is a widely
  loaded module and a diff-touched target set is structurally blind to consumers
  broken by a closure change — that blindness has now cost this lane one red CI
  merge and lane 3 one QA reject. Not a relaxation of the targeted-build hard
  rule: what changes is which targets count as affected, not how many crates
  build at once.

## What is explicitly OUT of scope

- Any change to production emission. This is observer-side and test-side only.
- Any change to the route relation, its variants, or the `matches_governed_arrival`
  gate. Those were attacked in the same hunt and **held**.
- Anything the D3 consumer owns. If this work appears to require applying `K`,
  binding `R2`, or touching the merge, **stop — that is a hard stop to the
  Steward**, not a scope negotiation.

## Verified clean in the same hunt, recorded so nobody re-litigates it

The hunt attacked and could not break: the producer-to-route authority swap
(`matches_governed_arrival` still compares the exact `(invocation, call, callee)`
tuple; the one production consumer at `source.rs:4099` rejects a non-matching
route fail-closed); the 14-arm `Route*` confluence/mutation suite, which is
NON-VACUOUS despite its neighbor-conditional arms, because each child asserts
`red.is_err()` AND an exact stderr arm message AND `is_exact()`, so a silent
no-op would leave the exact build succeeding and fail the parent; direction
reversal, guarded unconditionally in production with only `Forward`
constructible outside test builds; and the sink binder census, pinned to
`ConstructorChild` field 0 with `frame_origin == active_frame_origin` and
requiring exactly one non-recursive single-binder case.

**That the applied-count discipline the Adversary flagged missing on an earlier
node is PRESENT here is a positive result and should not be re-derived.**
