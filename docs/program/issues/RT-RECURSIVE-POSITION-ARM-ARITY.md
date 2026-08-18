---
id: RT-RECURSIVE-POSITION-ARM-ARITY
title: "One recursive position index is pushed unchanged into every plain-Match arm body -- resolve_recursive_unit_body recurses per arm with the eliminator's single position, and args.get(position) refuses on any arm whose constructor does not carry that position, so the branched-scrutinee port descends correctly and then fails inside the arm"
status: merged
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

> # `D0` IS DISCHARGED. The index-plumbing row is selected BY MEASUREMENT.
>
> **Measured `evt_1ejy6n0qjvg7e` / `evt_55xajyh0w6cma` at `64bdb4cd8`.** All four
> governed programs: eliminator recursive-position list **`[1]`**, carried
> position **`1`**, arms **`Ret`** (1 argument) and **`Vis`** (2 arguments).
> Position 1 is **out of range for every `Ret`** and **valid for every `Vis`**.
>
> **Arm-count control holds:** 2 reported arms and `match_arms_walked=2` per
> row; 8 reported = 8 walked over the run. `AC-1` demanded every arm, and every
> arm was reported.
>
> **`match_arms_walked` reads `1` above and `2` here, and both are correct.**
> `D0''` ran production control flow, where `?` aborts at the **first** refusing
> arm, so only one arm is ever walked. `D0` used a **test-only** probe that
> continued past the observed arm error specifically so it could measure the
> **second** arm — which is what `AC-1` required and what excluded the
> representation-gap row. **The counter did not change; the number of arms
> reached did.** Do not read the pair as an instrument disagreeing with itself.
>
> **The losing rows, with the value that excluded them (`AC-2`):** the
> representation-gap row is excluded because `Vis` **does** carry position 1 —
> it is not absent from the family. The re-attribution row is excluded because
> the refusing `Construct` is the governed arm body, at the same eliminator-
> derived index.

# THE MECHANISM — hypothesized here, then MEASURED. Both halves stand.

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

> ### THE DISCIPLINE THAT MADE THIS MEASURABLE — kept deliberately
>
> This section was published as **an explicit reading, flagged as not a probe**,
> and `D0` was framed to measure it rather than confirm it. It came back true.
> **A hypothesis that survives because it was measured is worth more than the
> same sentence asserted**, and the two preceding cuts in this chain both
> overturned a reading that explained the evidence just as well.

## WHERE THE INDEX COMES FROM — this half is a READING, and it is not measured

`position` originates at `core.rs:16164` from **`case.recursive_positions`** —
the recursive positions of the **eliminator case being lowered** — and is used
at `:16173` to index **that case's own children**. The same value is then
handed to `recursive_position_unit_body` at `:16190`, which walks the
eliminator's **scrutinee** and indexes **the scrutinee producer's** constructor
arguments by it.

⇒ Those two argument arrays coincide only when the scrutinee is a direct
`Construct` **of the constructor being matched**. When the scrutinee is a plain
`Match` whose arms produce **different** constructors, the index belongs to one
of them and is applied to all. `Vis` owns recursive position 1; `Ret` is a base
case that owns no recursive position at all.

**This paragraph is a reading of the encoding.** It is consistent with every
`D0` number and it is the natural explanation of them. **`D1` does not need it
to be true** — the deliverable below is stated against the contract, not
against this account.

# `D0` — DISCHARGED at `64bdb4cd8`. Separate the two readings.

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

# `D1` — the repair. Narrow, and the soundness-bearing option is FENCED OUT.

**What is authorized:** on the **per-arm descent path only**, an arm that does
not carry `position` yields **no declared unit** — `Ok(None)` — instead of the
`Err` at `:15924`.

**Why this is the contract's own answer and not a relaxation.** `Ok(None)` is
**already a supported value at the sole call site**: `:16190` passes this
result straight into `make_computational_recursor` as an `Option`, and the
function's own doc at `:15868-15874` names `None` as a normal outcome —
*"Structural-data recursive positions return `None`; they resume the eliminator
directly and take no arguments."* The `Match` branch **already collapses the
whole descent to `Ok(None)` when any arm yields none** (`:15911-15913`). So the
disposition for a non-carrying arm is defined by the surrounding code; the hard
`Err` is the outlier.

> ### THE ARM-SKIPPING OPTION IS NOT AUTHORIZED. It is the silent-wrong-answer case.
>
> **Do not skip a non-carrying arm and let the remaining arms' unit stand as
> the declared unit.** That converts a refusal into an answer, which is exactly
> what this node's banned scope was written to prevent, and nothing measured so
> far says the surviving arms' unit is correct for the whole `Match`.
>
> If the ring concludes skipping is the right semantics, **that is a hand-back
> with the argument, not a decision inside this cut.**

**`D1` is a candidate-producing deliverable.** Targeted runs only.

## `D1` MUST REPORT WHAT THE REPAIR DOES TO THE ROW, not just that it compiles

The `D0` probe **continued past the arm error and hit a later `BoundaryCarrier`
refusal** (`evt_1ejy6n0qjvg7e`). That was a test-only continuation, so it is an
**observation, not a measurement** — but it is direct evidence that **this guard
may not be the last blocker on these rows.**

⇒ **Report the post-repair disposition of all four `cap41_*` rows explicitly.**
"The guard no longer fires" is not the deliverable; **where each row stands
after it** is. Both outcomes are good and neither is a failure of `D1`:

| post-repair outcome | what it means |
|---|---|
| rows pass | the index-plumbing defect was the last blocker; `NATIVE-HANDLE-CARRIER` `D3`/`D4`/`D5` ungate |
| rows fail at a **new** site | the repair is correct and a further blocker is now attributed — report its site and identity, and **do not chase it in this cut** |
| rows fail at **`:15924` still** | the repair did not reach the path; re-attribute before changing anything else |

# ACCEPTANCE

- **`AC-1` — MET.** The four values above are reported for **every arm** of the
  governed `Match`, not only the refusing one. **Control:** an arm count that
  matches `match_arms_walked` summed over the run. **Met at 8 reported = 8
  walked.** Reporting only the failing arm could not have distinguished row 1
  from row 2 of the table, which was the entire question.
- **`AC-2` — MET.** The disposition row is **selected by the numbers**, and the
  losing rows are stated with the value that excluded them. Both losing rows are
  excluded above by the `Vis` arity.
- **`AC-3`** — the `Err` at `:15924` is **still reached and still returned** when
  the scrutinee is a **direct `Construct`** whose args do not carry `position`.
  **Control:** a case that exercises the non-`Match` path and observes the
  refusal. A repair that makes this path return `Ok(None)` too has deleted a
  genuine inconsistency check and fails this criterion.
- **`AC-4`** — the post-repair disposition of **all four** `cap41_*` rows is
  reported, each with its site and refusal identity if it still fails. A report
  that names only the rows that changed does not discharge this.

> # `AC-4` ROW 2 IS REALIZED. Measured by CI on `5c933c6ca`, PR #2607.
>
> **The repair works and the refusal ADVANCED.** `main` was green (2 passed / 0
> failed on `rt_branched_scrutinee_unit_body_port`); at the candidate the suite
> is 1/1, and the failing assertion is the **terminal-refusal text**, at
> `:113`, not the `expect_err` at `:111`. **The build still refuses** — it now
> refuses at the `ObjectEmission` `BoundaryCarrier` site instead of the
> constructor-arity guard.
>
> ⇒ **This is the arity node's own `AC-4` table, row 2, verbatim: "rows fail at
> a NEW site — the repair is correct and a further blocker is now attributed."**
> Do not chase that further blocker inside this cut; it belongs to
> [[RT-BRANCH-LOCAL-DECLARED-CALLABLE]].
>
> **The handback's "preserved downstream result: 1 passed / 1 failed" was an
> ABSOLUTE reading offered as a preservation claim.** The baseline was 2/0.
> **A preservation claim is differential and needs the before-number**; QA and
> the Architect both accepted the absolute count as if it could show
> preservation. That is what let the inverted pin reach CI.

# BANNED SCOPE

- **The `:15924` guard change is authorized ONLY on the per-arm descent path,
  and ONLY to `Ok(None)`** (see `D1`). The direct-`Construct` path keeps its
  `Err` — that is `AC-3`, with its own control. **No arm-skipping** (fenced in
  `D1`). **No other production change.**
  - *Superseded, and kept so the narrowing is visible:* this bullet read **"No
    repair, no arm-index change, no `:15924` guard relaxation"** while `D0` was
    the only deliverable. Its warrant — *"nothing in the evidence says the
    refusal is wrong"* — **is what `D0` changed.** The evidence now says the
    refusal fires on a base-case constructor that structurally owns no recursive
    position. The ban is lifted **exactly that far** and no further.
- **No `NATIVE-HANDLE-CARRIER` deliverable.** `D3`/`D4`/`D5` stay gated.
- **[[RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT]]'s witness pin MAY be advanced
  inside `D1`, and nothing else about that node may be.** Its
  `rt_branched_scrutinee_unit_body_port.rs` witness closes with an assertion
  that the refusal text is *"recursive position is outside its source
  constructor"*. **`D1`'s authorized change necessarily falsifies it** — the arm
  yields `Ok(None)`, the descent advances, and the program refuses at a later
  site. **Advance that trailing assertion, or stop it naming the terminal
  site.** Leave the four assertions above it — `entered==1`, `!route1`,
  `match_branch_entered`, `match_arms_walked==1` — untouched; they are that
  node's actual subject and they all still hold.
  - *Superseded, and kept so the narrowing is visible:* this bullet read **"No
    reopening of [[RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT]]"**. **Its warrant was
    to stop the port node's subject — whether the descent happens at all — being
    relitigated.** Updating a trailing assertion about which refusal comes
    *after* the descent does not touch that subject. **The ban stands for
    everything else about that node.**
  - **This collision was a defect in this frame, not in the ring's work.** The
    banned scope and the authorization were written against each other, and CI
    is what surfaced it.
- **The `+59` `cap41_*` restoration stays uncommitted and is not a merge
  candidate** (Architect `evt_3tfef2baj5pd`). It is the measurement artifact, on
  `wp/NATIVE-HANDLE-CARRIER-D0` at `86049d660`, diff-check clean. **Lifting the
  carrier's honest-partial ban did not authorize adding failing tests to
  `main`.**

# CONTENTION

`lowering/core.rs` is the campaign's decomposition target
([[RT-BACKEND-MODULE-SPLIT]], 20,360 lines). **`D0` was measurement-only and did
not contend. `D1` DOES** — it edits a production line in that file.

**Re-derive the site's location at pickup.** Every line number in this node
(`:15911`, `:15922`, `:15924`, `:16164`, `:16190`) was read at `64bdb4cd8` in a
20k-line file that is under active decomposition pressure. **Locate the function
by name — `resolve_recursive_unit_body` — and the guard by its `args.get`
shape, never by the offsets written here.**
