---
id: RT-HOST-RESPONSE-DUPLICATE-PRELUDE-BLOCK
title: "SUPERSEDED BY RT-DUPLICATED-RESPONSE-BLOCK -- a duplicate filing of the same defect by the same author ten minutes apart, not a neighbouring node. Same four rows, same producer, same census (29 / all agree / one delta). The surviving id is the one that was already in flight. This node's unique material -- the refuted-premise interrogation, the RT-CLOSURE-BOUNDARY-LANE undetermined state, the readmission-condition debt, and the deliverable shape -- was folded into the survivor, not discarded. Kept as a tombstone so existing citations resolve."
status: closed
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward framing debt, cut 2026-09-18. RT-HOST-RESPONSE-ROUTE-KEY-COLLISION was DIAGNOSIS-ONLY and succeeded (merged ef11485dd; all four AC-1 dispositions read STAYS IGNORED). Its landed labels state a readmission condition that names work no node covers. Recorded as debt in that node's MERGED banner: 'THE REPAIR HAS NO NODE AND THAT IS STEWARD FRAMING DEBT. The 4 rows do not move until it is cut.' Third repair node sized from the RT-IGNORED-FAILING-ROWS-INVENTORY ledger (488acf24e) under the operator's L1 priority (2026-09-17): L1 clears the ignored tests, and nothing in the lane outranks it until the 15 selected rows are cleared. Every load-bearing fact below re-measured by the Steward against origin/main e75f1fe27 before filing."
---

> # TOMBSTONE — SUPERSEDED BY `[[RT-DUPLICATED-RESPONSE-BLOCK]]`
>
> **DO NOT WORK THIS NODE.**
>
> **Steward, 2026-09-18. This is a duplicate I filed of my own node, ten
> minutes later.**
>
>     RT-DUPLICATED-RESPONSE-BLOCK              filed 16:07:25Z   IN FLIGHT
>     RT-HOST-RESPONSE-DUPLICATE-PRELUDE-BLOCK  filed 16:17:02Z   this file
>
> Same four rows (`px7n:149`, `px7n:170`, `rt_escape:653`, `rt_escape:713`),
> same producer (`host_response_routes` in `static_transition/responses.rs`),
> same census numbers (29 colliding constructors, all agreeing on `operation`,
> one delta per program: 365, 317, 317), same merged predecessor. **It is the
> same defect, and both were given a WP frame.**
>
> **The survivor is the earlier id because it is the one in flight** — the
> ring's branch, posts and running `D3` census all cite it, and an id is a
> citation key. Renaming mid-turn for the better name would break every
> reference for a cosmetic gain.
>
> **Nothing here was lost.** This was the richer of the two documents, and the
> four things it carried that the survivor did not were folded into the survivor
> under their own headings — the refuted-premise interrogation, the
> `RT-CLOSURE-BOUNDARY-LANE` undetermined state, the readmission-condition debt
> the four labels name, and the deliverable shape. **Read them there.** The text
> below is left intact as the record of what was filed, not as live direction.
>
> **How this happened, since the mechanism is reusable:** the two filings came
> from opposite ends of the same chain. This one was cut from the *readmission
> condition* the four `#[ignore]` labels state; the survivor was cut from the
> *refusal message* the four rows produce. **The same defect names itself
> differently at its cause and at its symptom**, so neither filing's search
> terms could find the other — a grep keyed on either name misses the node
> keyed on the other. What would have caught it is neither name: it is the row
> set. **Before filing, search for the ROWS, not for the mechanism.**

# Why this node exists at all

`RT-HOST-RESPONSE-ROUTE-KEY-COLLISION` merged at `ef11485dd` having answered its
question. It was diagnosis-only, it landed **zero `src/` files**, and its four
dispositions all read STAYS IGNORED. **That is a delivered result, not a
shortfall.** What it produced was a corrected, measured readmission condition,
written into the live `#[ignore]` labels:

> *"Readmits when the duplicate prelude block is resolved AND the
> frame-marker single-consumption holds."*   (`px7n`, both rows)
>
> *"Readmits when the duplicate prelude block is resolved; re-measure for a
> second blocker at that point."*   (`rt_escape`, both rows)

**No node covers "the duplicate prelude block is resolved."** Four of the 15
selected rows are therefore parked behind unframed work. This node is that
work.

# The four rows, at `origin/main` `e75f1fe27`

Keyed by file and line, because the names do not disambiguate them:

    crates/ken-cli/tests/px7n_nested_computational_eliminator.rs
      :149  nested_ok_payload_reaches_both_real_executors
      :170  nested_err_payload_reaches_both_real_executors

    crates/ken-cli/tests/rt_escape_second_resource_native.rs
      :653  escaped_resource_used_by_fanning_host_op_matches_interpreter
      :713  nat_fanout_escaped_resource_matches_interpreter

**`rt_escape_second_resource_native.rs` has FOUR `#[ignore]` attributes, not
two.** The labels' phrase *"the two rows in this file"* means the two rows
carrying THIS label. The other two are `:684` (`RT-SITEOP-CARRIED-WITNESS D2`)
and `:776` (`RT-PROCESS-EXIT-STATUS`), and they are **not** in this node's
scope. Read the phrase as scoped to the label, or you will mis-enumerate the
file.

# The mechanism, measured

`crates/ken-runtime/src/cranelift_backend/planning/static_transition/responses.rs:1246-1287`:

```rust
fn host_response_routes(
    plan: &StaticTransitionPlan<'_>,
) -> Result<BTreeMap<RuntimeSymbol, HostResponseRoute>, CraneliftBackendError> {
    let mut routes = BTreeMap::new();
    for occurrence in plan.source_occurrences.iter().flatten() {
        let RuntimeExpr::Match { cases, .. } = occurrence.expr else {
            continue;
        };
        for (alternative, case) in cases.iter().enumerate() {
            ...
            if routes.insert(case.constructor.clone(), route).is_some() {
                return Err(planner_error(
                    "two host response cases claim one operation constructor",
                ));
            }
```

The map is keyed on `case.constructor` and the loop ranges over **every**
`Match` occurrence in the plan. So the invariant encodes

> **one host-operation constructor ⇒ one response-handling site in the whole
> program**

and `selected_host_response_route` (`:1289`) then resolves a `Vis` site's
operation subtree against that single entry.

**What produces the second entry is a duplication, and that is measured, not
assumed.** From the diagnosis node's census, run to completion by the probe
rather than counted by grep:

| program | colliding constructors | agree on `operation`? | distinct effect-origin deltas |
|---|---|---|---|
| `px7n-nested-computational-eliminator` | 29 | all 29 | one value, `365` |
| `rt_escape_escape_file_then_readat` | 29 | all 29 | one value, `317` |
| `rt_escape_nat_fanout_escaped` | 29 | all 29 | one value, `317` |

⇒ **A single distinct delta across the whole set is a block shifted wholesale
in origin-id space — one block materialized twice, not 29 competing claims.**
Twenty-nine independent program sites would not share one offset. The diagnosis
states the conclusion outright: *"the occurrence is the prelude's,
duplicated."*

**These counts are COLLISIONS OBSERVED IN THESE PLANS.** They are not a claim
about how many host-operation constructors exist, and they do not transfer
between programs — the two `rt_escape` rows build different Ken programs and
each was measured on its own.

# THE CONCLUSION THAT OUTLIVED ITS PREMISE — interrogate this before building

The diagnosis's own amendment carries:

> *"The conclusion stands; the reason does not."*

The refuted reason was *"a program that performs `RandomBytes` at two sites
with two response handlers is valid Ken"* — measured false, because **these
programs perform `RandomBytes` zero times**; `EntropyOp` is a prelude
declaration and neither test file mentions entropy. The surviving conclusion is
*"the invariant is too strong; pairing N `Vis` sites to N handlers is planner
work."*

**Ask what that conclusion LICENSES.** It licenses the expensive arm — planner
work to make routing occurrence-aware — and the only case ever offered for it
was the one measured false. **No replacement case is on the record.** If every
actual second entry is a materialization artefact, then the invariant is not
too strong; it is correctly refusing a plan that should never have contained
the block twice, and the repair is upstream of the planner entirely.

**This node does not pick.** The fork is a design question about admissible Ken
programs, and it routes to the Architect with a measurement in hand (see the
frame's `AC-2`). What the node forbids is inheriting *"the invariant is too
strong"* as settled and sizing planner work off it.

**Two repairs already ruled out, do not re-propose them:**

- **Widening the key to `(constructor, operation)` changes nothing.** Both
  entries already carry `EntropyRandomBytes`. Measured.
- **Deferring the refusal to the point of use is a measured NON-FIX for half
  the rows.** See below.

# Deferring to point-of-use splits the four rows in two

This experiment was already run, and its result is the reason the four rows
carry two different readmission conditions:

| rows | deferring the refusal to point of use |
|---|---|
| `px7n` `:149`, `:170` | **clears the collision.** The run then reaches the labelled mechanism: `OrientedSubcontinuationPlanV1: checked Runtime frame marker was consumed more than once`. |
| `rt_escape` `:653`, `:713` | **does NOT clear the row.** The constructor each row SELECTS (`FSOp::ctor_543` in its own program) is itself one of the counted collisions. |

⇒ **A repair that only defers cannot readmit more than two of the four**, and
the two it does reach then stop at a second blocker.

# The second blockers, and their node status

**Both are unframed, and this node does not absorb either.**

- **`RT-FRAME-MARKER-ONCE`** — `status: draft`. It is what the `px7n` pair hits
  next. **Zero rows in the tree cite it today** (the diagnosis relabelled the
  two that used to). A `draft` node with no citing rows is not a released
  repair, so **do not size this node as though the `px7n` pair readmits at its
  end.**
- **`RT-CLOSURE-BOUNDARY-LANE`** — `status: merged`, one citing row remaining
  elsewhere. Whether it is *also* a real blocker under the `rt_escape` pair is
  **UNDETERMINED**: nothing has seen past the collision on those rows, so the
  durable-lane claim is neither confirmed nor refuted. **Undetermined is the
  honest state — do not record it as ruled out.**

# What this node is NOT

- **Not a rewrite of the static-transition planner.** If the Architect rules
  the occurrence-pairing arm, that is a successor node with its own frame, cut
  by the Steward.
- **Not the other eleven selected rows.**
- **Not `RT-FRAME-MARKER-ONCE` and not `RT-CLOSURE-BOUNDARY-LANE`.** This node
  measures whether they are reached; it repairs neither.
- **Not a promise of four readmissions.** The deliverable is four rows
  re-dispositioned by file and line, each either readmitted or carrying a
  **measured** next blocker. The ledger's central finding — 12 of 27 stated
  reasons already false — is why a stated blocker is never accepted in place of
  a measured one.
