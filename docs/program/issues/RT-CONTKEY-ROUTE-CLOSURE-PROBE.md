---
id: RT-CONTKEY-ROUTE-CLOSURE-PROBE
title: "the one question the carry merge deliberately did not answer: with required_consuming_occurrence now available at depths 2 and 3, does the D2k-1c route repair close row 4, or does it still refuse at the next boundary -- a measured-and-reverted probe that authors no candidate, because both ways forward from a residual cross a banned surface and the choice between them is the Architect's"
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-CONTKEY-CONSUMER-DESCENT-CARRY, RT-CONSUMER-CARRY-CONTROL-DEBT]
blocks: [RT-LEXICAL-RECURSOR-CONSUMERS]
github: null
origin: "Architect evt_56dvtaft7ep38, verbatim in substance: 'That supplying the relation CLOSES the route is NOT established. The original stop reported a further refusal at the next Closure/static-worker boundary and a second recognition retained in the standalone definition. (c) supplies a missing input; whether the route then closes is a separate measurement.' He then instructed: 'Frame the successor as carry the consumer to the level that needs it and let the route question be its own increment.' RT-CONTKEY-CONSUMER-DESCENT-CARRY was that successor and merged at b0f9c2ff with no closure AC. This node is the increment the instruction reserved. Steward-filed per COORDINATION §2; not released to Runtime until RT-CONSUMER-CARRY-CONTROL-DEBT lands, because the two share static_transition.rs."
---

## What this is

**A measured-and-reverted probe. It authors no candidate and lands no repair.**
The deliverable is the measurement, exactly as `D2k-0` was and exactly as
`D2k-1c`'s stop was.

[[RT-CONTKEY-CONSUMER-DESCENT-CARRY]] merged at `b0f9c2ff` and supplied the
missing input: at depth `N` a discovery now holds `required(N)`, the consuming
occurrence established at level `N-1`. **It supplied that input and nothing
else.** `required_consuming_occurrence` is **production-written and
test-only-read**, no production consumer was wired, and no row closed.

**This node asks the one question that was deliberately left unasked: if a
production consumer reads the carry, does the route close?**

## Why it is a probe and not a repair, and this is the whole design of the node

`D2k-1c` stopped because **both ways forward from its residual cross a line it
banned** — one mutates the planner-owned
`BodyEmissionDisposition::ContinuationTemplate` population, the other needs an
exact projection through the excluded continuation-source surface. The Steward
disposition was that the slice is not widened and **Runtime was right not to
author a candidate**.

**Nothing about that has changed except the availability of the carry.** If the
route still refuses, the fork is the same fork and it is still the Architect's.
⇒ **Authoring a repair here would be choosing between two banned surfaces on
the ring's own authority, which is precisely what the last stop refused to do.**

**If the route DOES close, that is a large result and it still does not license
a repair in this node.** Report it; the repair is then a clean cut with a
measured warrant rather than a hypothesis, and it will be framed as one.

## What `D2k-1c` measured, and which line of it is now stale

From the stop record in `docs/program/wp/RT-LEXICAL-RECURSOR-CONSUMERS-D2k.md`.
Applying the landed consuming occurrence **inside the standalone
specialization** closed **row 4 depth 1 and row 5's first transport**. Then:

| observation | status now |
|---|---|
| row 4 depths 2/3 construct a second worker-bearing constructor **whose next outer consumer has no carried relation at all** | **this is the half `b0f9c2ff` supplied.** It is the reason this node exists |
| moved to the real call funnel, the route refuses at the next boundary — `Closure` at depth 1, a further worker-bearing-constructor boundary at depths 2/3 | **unmeasured since the carry landed** |
| row 5 separately keeps a standalone continuation definition with a **second unconsumed recognition** | **unmeasured since the carry landed**, and it is a separate finding from row 4's |

⇒ **Exactly one of the three inputs to that stop has moved.** Do not assume the
other two moved with it, and do not assume they did not.

## Fixed inputs, measured at `main` `c86eeb46`

Re-derive at your base. Every site is
`crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs`.

**`:8148` — the carrier.** Note its doc comment says *"established one level
outside the discovery"*, which [[RT-CONSUMER-CARRY-CONTROL-DEBT]] `C3` records
as wrong at depth 1. **That node corrects it; read the corrected text at your
base, not this sentence.**

```rust
required_consuming_occurrence: Option<ContinuationRequiredConsumingOccurrence>,
```

**`:11446` — where the value is advanced**, from the already-interned target:

```rust
let required_consuming_occurrence = target_unit
    .key
    .consuming_occurrence
    .map(ContinuationRequiredConsumingOccurrence::Exact);
```

and carried to the child at the push, `:11466`-`:11472`.

**The two reads, and BOTH are `#[cfg(test)]`.** This is the seam the node
measures across:

| site | what it reads | profile |
|---|---|---|
| `:11450` | the advanced value, into `CONTINUATION_REQUIRED_CONSUMER_OBSERVATIONS` | `#[cfg(test)]` |
| `:10976` `required_consuming_occurrence_for_alternative`, called at `:11343` | the carried value, resolving `Source` seeds | **the function is `#[cfg(test)]` and so is its call site** |

**What production uses instead, and it is the point.** The specialization key
built immediately after `:11360` takes `consuming_occurrence:
consuming_occurrence_from_seed(plan, &discovery, alternative)?` — the
**same-level, source-keyed** value. **The level-appropriate value is present on
the discovery and no production path reads it.**

⇒ **The probe's production edit is exactly this substitution, at whichever
site the route repair actually consults**, and `D1` is where you establish
which site that is rather than assuming it is this one.

## Deliverables

**`D0` — name the consumer site before changing it.** Re-apply the `D2k-1c`
route repair far enough to reach the refusing boundary on row 4, and record
**which production read of a consuming occurrence the route actually depends
on.** It may be the key construction above; it may be downstream on the call
funnel, which is where `D2k-1c` said it *"reaches the carried body/eliminator
on the live stack."* **Name the site with `file:line`, read from `git show
<sha>:<path>`, and state the SHA you read.**

> Two wrong coordinate sets were published on this lane before, and both
> resolved to real unrelated code. Nothing errors when this goes wrong.

**`D1` — substitute the carried value at that site and compile row 4 at depths
2 and 3.** Report, per depth: the refusal (or its absence), and if present its
exact construct, `edge`, message, and **which unit raises it** — the last of
those is what distinguished a real root from a forwarded one at `D2k-0`.

**`D2` — the same for row 5's standalone recognition.** Is the second
unconsumed recognition still retained? **This is a separate question with a
separate answer** and a node that reports only row 4 has delivered half.

**`D3` — depth 1 and row 5's first transport, which `D2k-1c` already closed.**
Confirm they still close under the substitution. **A repair that closes the
deep levels by breaking the shallow one is not progress**, and nothing
currently asserts the composition.

**`D4` — if anything still refuses, say which of the two banned surfaces the
residual demands**, and stop there. `ContinuationTemplate` population, or the
continuation-source projection, or **neither, if the carry opened a third
route**. Name it; do not take it.

**`D5` — revert every probe byte and verify byte-identity**, in the `D2k-0`
manner. The tree this node leaves behind is `main` plus, at most, a control
that is assertable **without** the reverted repair.

## Acceptance criteria

**`AC-1` — the probe demonstrably consumed the CARRIED value, not the
same-level one.** Show the read site and the raw pairs actually observed at
each depth. At row 4 these are `(16,5)` at depth 2 and `(26,21)` at depth 3;
**a probe reporting `(26,21)` and `(36,31)` read the same-level value and
measured the old question.** This is the exact off-by-one that produced a wrong
route selection on this lane once already.

**`AC-2` — raw values, not verdicts.** Every reported outcome carries the
observed identities. *"The refusal is gone"* without the pairs is not an
answer; reporting the raw pairs is the only reason the earlier probe's wrong
branch was recoverable.

**`AC-3` — no candidate, no retained repair, no widening.** If the route
closes, that is reported and the repair is framed separately. **Closing the
route inside this node is a stop condition, not a bonus.**

**`AC-4` — neither banned surface is crossed**, in the probe or in anything
left behind: no `BodyEmissionDisposition::ContinuationTemplate` population
change, no continuation-source surface projection.

**`AC-5` — depth 1 is unchanged in the reverted tree.** The landed
`row4-depth-1` and `row5-after-hole` controls hold on the same derivation.

**`AC-6` — no-regression, in CI.** `COORDINATION §12`; the venue is CI, never a
local `--workspace` run. Build and test targeted, `-p ken-runtime`.

## Stop conditions -- return to the Steward, do not decide

1. **The route closes.** Report it and stop. It is the good outcome and it is
   still not this node's repair.
2. **The residual demands a banned surface.** Name which, stop, and it routes
   to the Architect as a mechanism question — **with the need surfaced and the
   vehicle left open.** A bundled mechanism anchors the owner and its rejection
   then reads as *"the need cannot be met"*, when the owner can usually meet it
   more cheaply from inside their own lane.
3. **The route repair cannot be re-applied far enough to reach a boundary.**
   That is a finding about the repair's own base, not a licence to reconstruct
   it from scratch.
4. **Row 4 and row 5 disagree** — one closes and the other does not. Report
   both; do not average them into a single verdict.

## Sizing

**`S`.** One substitution at one already-identified read, two fixtures, a
revert, and a written report. **`D0` is the part that can grow**: if locating
the route's real consumer read turns into an investigation, that is the hard
stop and it is a good outcome — report it and the node is re-cut, exactly as
this node's predecessor was cut from `D2k`'s stop.

## Not this node

- **Not a repair of anything it measures.** Stated three times above because
  this lane has produced a repair-during-a-probe once and the cost was a turn.
- **Not [[RT-CONSUMER-CARRY-CONTROL-DEBT]].** That node corrects the carry's
  controls and its stated law; this node consumes the carry. **They share
  `static_transition.rs` and must not run concurrently** — control debt runs
  first, and it is in this node's `depends_on` for that reason.
- **Not [[RT-CONTKEY-REFUSAL-PROFILE-SPLIT]]**, which owns the unnamed-cause
  refusals in the same file. Same sequencing constraint.
- **Not row 1.** It is a different class, its two target edges distinguish it
  from rows 4 and 5, and it was correctly left unprobed. **Do not let a
  three-row conclusion in.**
