---
id: RT-CROSSING-CALL-SITE-ATTRIBUTION
title: "The suppression differential cannot separate branch 1 from branch 3' -- the separator is the CALL SITE, so record which invocation of transfer_into_carrier the origin-5 crossing comes from"
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-REQUIRED-CONSUMER-REACH-CENSUS]
blocks: []
github: null
origin: Architect, 2026-08-15, resolving dec_35e0tfng528d on RT-REQUIRED-CONSUMER-REACH-CENSUS D5 (evt_38p42gjq12br). He named this increment in the resolution -- "adding the invoking site is the cheap next increment, and it is the one I would frame." Steward-filed (agents cannot create tracked work per COORDINATION section 2).
---

> # THE PREDECESSOR MEASURED SOMETHING REAL AND IT IS NOT WHAT THE FORK NEEDED
>
> [[RT-REQUIRED-CONSUMER-REACH-CENSUS]] `D5` established, cleanly and for the
> first time, that **the required-consumer route manufactures the
> closure-bearing crossing** at `StaticOriginId(5)` /
> `Constructor.arg[0].Closure`, and that suppressing the route returns both rows
> to `StaticWorkerBinding`.
>
> **It did not select a repair branch, and its own `CLAIMED` line was amended to
> stop saying it had.** `closure_path` is computed **only at the crossing**, so
> on the suppressed rows `closure_child_present: false` is an artifact of there
> being no observation point — not evidence the graph lacks a closure. The
> Architect's branch 1 and branch 3 **both predict that row exactly**.
>
> **Do not re-run the suppression differential expecting a better answer.**
> Without the projection these rows never build the subgraph at all, so *"does
> the closure pre-exist"* may be **ill-posed rather than merely unmeasured**.
> A second differential on the same axis buys nothing.

## The live pair, and the one thing that separates it

| | claim | where the repair goes |
|---|---|---|
| **(1)** | the realization produces a value that **should not be closure-shaped** | the value production |
| **(3')** | the value is **legitimately** closure-shaped and should never have been **routed** to a carrier | keep the realization local |

**Both survive the whole `D5` table.** What separates them is not another
differential but the **call site**:

> On the **enabled** path, why is `transfer_into_carrier` invoked on origin 5's
> constructor at all — is that invocation on the **required-consumer
> realization's return path**, or in an **ordinary constructor-carrying path
> that would have run regardless**?

If it is the realization's return path, the projection routed a value it was
built to keep local, and the repair is (3'). If it is a path that would have run
anyway, the projection did not route anything — it produced a value of the wrong
shape into a pre-existing carrier path, and the repair is (1).

## Deliverables

**`D1` — record the invoking site on the crossing event.** The
`BoundaryTransferEntered` event already carries `origin` and `root_kind`. Add
the **invoking call site** of `transfer_into_carrier` — enough to name which
producer entered the walk, not a full backtrace. Same `#[cfg(test)]` discipline
as the predecessor: nothing added to a production build surface.

**`D2` — attribute origin 5's crossing for row 4 depths 2 and 3.** Report the
invoking site per row, and state **which of (1) or (3') it selects**, with the
reasoning written out. **A report that names the site without selecting a branch
does not discharge this** — the site is only worth recording because it decides
the fork.

**`D3` — the two latent misreports the Architect flagged, both non-blocking on
`D5` and both live the moment this node extends the same diagnostic.**

1. **`transfer_into_carrier_reached` is filtered to `StaticOriginId(5)`**, so it
   means *origin-5 crossing reached*, never *any crossing reached*. **The field
   name is wider than the measurement.** Rename it, or narrow what it records.
2. **`first_boundary_closure_path` skips non-`Specialized` constructor fields**
   (`else { continue }`) at sites where **the production walk errors on them**.
   So a future *"closure absent"* can mean *"behind a field this walker declines
   to enter."* The diagnostic is a near-copy of a production traversal and
   diverges from it here.

**Neither affected `D5`'s table.** They matter here because this node adds a
third field to the same event and reads the same walker, and a diagnostic whose
field names overstate their scope is how a later reader draws the wrong
conclusion from a correct table.

## The live stop

**If the invoking site is neither cleanly the realization's return path nor
cleanly a would-have-run-anyway path** — for instance if one row is each way, or
if the site is shared and the two cases are not distinguishable from it — **say
so and stop.** Do not force a branch. The predecessor's `D5` was amended
precisely because a measurement was labelled with a branch it did not establish,
and repeating that here costs the repair cut, not just the sentence.

## Acceptance criteria

**`AC-1`.** `D1`'s field is `#[cfg(test)]`-gated and adds nothing to a
production build surface, verified the way the predecessor verified it (targeted
`ken-runtime` check), not asserted.

**`AC-2`.** `D2` names the invoking site **per row** for row 4 depths 2 and 3,
and selects (1) or (3') with the reasoning stated. **Naming the site without a
selection fails this.**

**`AC-3`.** The report does not claim the closure's **pre-existence** either
way. That question may be ill-posed on this route and it is not what this node
measures.

**`AC-4`.** `D3`'s two misreports are both addressed in the tree — a field whose
name matches its measurement, and a walker divergence either removed or
documented at the site.

**`AC-5`.** Any new control declares its **promise class** and says to rewrite
it on an authorized route change, matching the predecessor's discipline. That
discipline is what kept the `D2a` collision from recurring.

**`AC-6`.** No repair to either branch lands here. **This node selects the
branch; it does not cut against it.** The repair is the successor's, and its
owner follows from the selection.

**`AC-7`.** No-regression, in CI (`COORDINATION §12`).

## Why this earns a slot

**It is the named next increment of a ruled fork, not new scope.** The Architect
resolved `dec_35e0tfng528d` with the separator identified and the increment
sized: *"the event already carries `origin` and `root_kind`; adding the invoking
site is the cheap next increment, and it is the one I would frame."*

**And the alternative is guessing.** The chain's next repair is either in value
production or in routing, and those are different files, different owners, and —
per the predecessor's own warning — the `D2k-1c` cost if cut wrong. One field on
an existing event decides it.

**This is lane 1.** It sits on the `RecursiveDescent` retirement path:
`PROJECTION` (merged) → `CENSUS` (merged) → **this** → repair → `TRANSPORT`
→ `DESCENT-RETIRE`.
