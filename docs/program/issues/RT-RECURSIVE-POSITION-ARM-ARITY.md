---
id: RT-RECURSIVE-POSITION-ARM-ARITY
title: "One recursive position index is pushed unchanged into every plain-Match arm body -- resolve_recursive_unit_body recurses per arm with the eliminator's single position, and args.get(position) refuses on any arm whose constructor does not carry that position, so the branched-scrutinee port descends correctly and then fails inside the arm"
status: ready
owner: runtime
size: S
gate: none
depends_on: []
blocks: [NATIVE-HANDLE-CARRIER, PX8-F-CAP-41]
github: null
origin: "NATIVE-HANDLE-CARRIER D0'' attribution, runtime-leader evt_2xw4mxcd5z9zp, measured at exact 86049d660. Predecessor D0' evt_2kdscqgge6x2p; the Steward's rejection of D0's same-refusal reading and the D0'' cut are evt_506j9kvpby4sz. Steward-filed per COORDINATION section 2."
---

> # THE BLOCKER IS POSITIVELY ATTRIBUTED. This is not a suspected gap.
>
> **Measured `evt_2xw4mxcd5z9zp` at exact `86049d660`:** all four governed
> `cap41_*` programs record **`entered=1`, `route1=0`, `match_arms_walked=1`**.
> The plain-`Match` port **reaches this population and walks its arms in every
> case.** The failure is strictly downstream of it.
>
> The refusal is `core.rs:15924`, the `args.get(position)` guard, reached while
> recursive resolution is at a `RuntimeExpr::Construct` whose requested
> recursive position is absent. **It is neither the `D2` plain-`Match` agreement
> refusal nor a route-1 return.** Each targeted row is 0 passed / 1 failed,
> exit 101.

# WHY THIS NODE EXISTS RATHER THAN A REOPENED PORT NODE

[[RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT]] did what it claimed. Its subject was
*whether a plain-`Match` scrutinee gets descended into at all*, and the answer is
now yes, measured, on this exact population. **Reopening it would relitigate a
discharged claim.** What this node owns is a different mechanism one level down:
what index the descent carries with it.

# THE MECHANISM, READ FROM THE CODE — and it is a READING, not a measurement

`resolve_recursive_unit_body` (`lowering/core.rs:15892`) takes a single
`position`. On the plain-`Match` branch it recurses **per arm** at `:15911`:

```rust
let Some(unit) = self.resolve_recursive_unit_body(body.static_origin, position)? else {
```

**The same `position` goes into every arm.** It originates at the sole call site
`:16190`, `recursive_position_unit_body(eliminator.static_origin, position)`,
where it indexes the **eliminator's** recursive-position list. On the
`Construct` path it is then used directly as an index into **that arm's
constructor arguments** at `:15922`.

⇒ **Hypothesis:** the arms of a plain `Match` are distinct constructors with
distinct arities, so an index meaningful for one arm is out of range for
another, and `:15924` refuses on the first arm that does not carry it.

> ### THIS IS THE PART THE FIRST DELIVERABLE MUST NOT INHERIT
>
> **Everything in this section is a reading of the encoding, and a reading of
> the encoding is not a probe of it.** The mechanism is plausible, it is
> consistent with every measured number, and it is exactly the kind of story
> that survives review because it explains the evidence. **It has not been
> measured.**
>
> The competing reading is that the position is correct and the constructor
> genuinely lacks it — a real representation gap rather than an index-plumbing
> defect. **Those have different owners and different fixes**, and no number
> collected so far separates them.

# `D0` — separate the two readings. One run, and it is the whole deliverable.

At the arm where `:15924` first refuses, report **per arm** of the governed
plain `Match`:

1. the **constructor identity** of the arm body's `Construct`,
2. its **argument count**,
3. the `position` value carried into it,
4. and the **eliminator's** recursive-position list that `position` indexes.

| reading | means | owner |
|---|---|---|
| arm arities **differ**, and `position` is valid for at least one arm and out of range for another | the descent carries a per-eliminator index into a per-arm structure — an **index-plumbing defect in the port's recursion** | runtime, small cut, this node |
| **every** arm lacks the position, including arms whose arity would allow it | the recursive position is genuinely absent from this constructor family — a **representation gap** | route to the Architect before any cut; not this node's shape |
| the position is valid at every arm and the refusal comes from a different `Construct` | the failing site is not the governed arm at all | attribute again; the population is not what we think |

**Report the readings. Do not repair.** A red result is the deliverable. This is
the third measurement-first cut in this chain and the previous two both changed
the disposition — the first `D0'` reading would have hard-stopped the lane on a
refusal string, and `D0''` overturned it.

# ACCEPTANCE

- **`AC-1`** — the four values above are reported for **every arm** of the
  governed `Match`, not only the refusing one. **Control:** an arm count that
  matches `match_arms_walked` summed over the run. ⛔ Reporting only the failing
  arm cannot distinguish row 1 from row 2 of the table, which is the entire
  question.
- **`AC-2`** — the disposition row is **selected by the numbers**, and the
  losing rows are stated with the value that excluded them.

# BANNED SCOPE

- **No repair, no arm-index change, no `:15924` guard relaxation.** Widening the
  guard to accept a missing position would convert a refusal into a silent wrong
  answer, and nothing in the evidence says the refusal is wrong.
- **No `NATIVE-HANDLE-CARRIER` deliverable.** `D3`/`D4`/`D5` stay gated.
- **No reopening of [[RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT]].**
- **The `+59` `cap41_*` restoration stays uncommitted and is not a merge
  candidate** (Architect `evt_3tfef2baj5pd`). It is the measurement artifact, on
  `wp/NATIVE-HANDLE-CARRIER-D0` at `86049d660`, diff-check clean. **Lifting the
  carrier's honest-partial ban did not authorize adding failing tests to
  `main`.**

# CONTENTION

`lowering/core.rs` is the campaign's decomposition target
([[RT-BACKEND-MODULE-SPLIT]], 20,360 lines). This node is measurement-only and
touches no production line, so it does not contend. **A later repair would** —
re-derive the site's location at pickup rather than trusting `:15911`/`:15924`.
