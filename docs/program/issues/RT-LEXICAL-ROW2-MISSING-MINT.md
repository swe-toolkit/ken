---
id: RT-LEXICAL-ROW2-MISSING-MINT
title: "Row 2 of the lexical-recursor population fails post-compile with a missing Mint rather than at a lowering boundary, so it is not repairable by RT-LEXICAL-RECURSOR-CONSUMERS' D2"
status: merged
owner: runtime
size: S
gate: none
depends_on: [RT-LEXICAL-RECURSOR-CONSUMERS]
blocks: [RT-RECURSOR-TRANSPORT]
github: null
origin: Runtime D0/D1 checkpoint evt_3wzr30y2jjh41 at exact 0de4f130a9864623afcffa0b4751f65ddd87e818, QA-approved at evt_1f9hknbynb5nk, measured at 9adeb30f. Steward ruling evt_rcjr99tjga9 removed this class (R4) from RT-LEXICAL-RECURSOR-CONSUMERS. Steward-filed per COORDINATION §2.
---

> # MERGED 2026-08-12 — EVIDENCE-COMPLETE, NO PRODUCTION REPAIR
>
> Frame: `docs/program/wp/RT-LEXICAL-ROW2-MISSING-MINT.md`.
>
> **The row is not a semantic regression.** `D1` closed with **no production
> repair** (Steward `evt_26cb49zckgq4f`) and the successor measurement then
> fired the present branch: on the functionized lane row 2's recursive IH **is**
> minted, installed and consumed, by the carried/`Composed` route. The old
> assertion was **over-specified** — it pinned *which producer path mints* where
> the invariant is that the occurrence gets an IH installed and consumed.
> Architect `evt_1rzcz31qm9y9q` ruled the diagnosis correct and **kept row 2 on
> `D3`'s bar** on corrected grounds.
>
> | deliverable | disposition |
> |---|---|
> | `D0` attribution sentinel | merged, PR #1950 (corrected in place, not deleted — no repair landed) |
> | `D1` bounded repair | **closed with no repair**; the functionized lane requires zero `SourceMachine` installations for this occurrence |
> | `D2` discriminating controls | merged, PR #1955; bullet 4 discharged, bullets 1-3 **superseded** — each presupposes a repaired root |
> | successor IH measurement | merged, PR #1957, CI green at `6a804eb7` |
> | `AC-2` | struck as defective, replaced with the measured lane-conditional sets, PRs #1953 / #1958 |
> | `AC-4` negative control | **routed to `D3`**, proof 4 of four — owed there, not here |
>
> **What this node did NOT establish**, and no ruling depends on it: whether the
> `SourceMachine` path is reachable on a functionized lane by **any** occurrence.
> The fixture builds only the recursive occurrence, so it cannot distinguish
> *this occurrence routes elsewhere* from *the path is dead*.

## Why this is its own node rather than a `#6d` deliverable

Runtime's `D1` partitioned rows 1-5 into four classes across three
authorities. Three of them — `R1` source-machine non-constructor guard,
`R2` closure boundary-transfer arm, `R3` computational-recursor
boundary-transfer arm — are lowering refusals at a boundary.

**`R4` (row 2) is not.** It is a **post-compile missing `Mint`**, so it has no
boundary at all.

`RT-LEXICAL-RECURSOR-CONSUMERS`'s `D2` is defined in its frame as *"repair only
the proven root boundary or boundaries."* A failure with no boundary is **not
expressible in that deliverable**, so folding it in would mean widening the
deliverable's definition to fit the finding rather than routing the finding.
That is the split, and it is a scope judgment, not a difficulty judgment.

⇒ The constraint is grounded in `#6d`'s own written `D2` scope, not in a
preference for a tidier graph (`steward.md §4c`).

## What is established, and what is not

**Established** (Runtime `D0`/`D1`, QA-approved): row 2 fails post-compile with
a missing `Mint`; it is not a lowering refusal; it is one of four classes in a
population that closes at 16 `B` compiles across 10 tests.

**Also established, and it is what released this node from `draft`:** the three
compiles left unmeasured after the first abort have since been measured, and
**all three are `R1`**. The `R1` cell closes at five compiles across rows 1 and
4; none was reassigned to row 2 and none added a sibling here.

⇒ The one-row population is now a measurement rather than an artifact of a probe
that stopped at the first cause. It remains a **floor**: `D0` re-closes it at the
candidate's own base, and if it comes back wider, that is a re-size signal to
post and stop.

## Sequencing — ON THE CRITICAL PATH. Corrected 2026-08-11.

**Release after [[RT-LEXICAL-RECURSOR-CONSUMERS]] and BEFORE
[[RT-RECURSOR-TRANSPORT]] `D3`.** Architect ruling `evt_2jnf3x8f06psz`, on a
Steward escalation.

> **STRUCK — *"Not on the RecursiveDescent critical path: `#6d` closes on
> `R1`-`R3`, and `#7` `RT-DESCENT-RETIRE` gates on `RT-RECURSOR-TRANSPORT` and
> `RT-FNUNIT-RESULT-TOKEN`, neither of which is this."***
>
> **The reasoning was about the wrong node.** It checked `#7`'s dependencies —
> correctly — and never asked whether **`#6b`'s own `D3` bar** includes row 2.
> It does: `D3` must prove **all six rows green without any exclusion hook**,
> stated in four places, and row 2 is one of the six. `#6d` cannot repair it.
> ⇒ **This node sat off the graph while being required by it.**
>
> **The carve-out changed the repair OWNER, not the retirement ACCEPTANCE
> SURFACE.** `#6d` was correctly forbidden to absorb `R4` — its `D2` is
> boundary-only and a missing `Mint` has no boundary — but that says nothing
> about whether a post-retirement semantic regression is acceptable. **Those are
> two different questions and the carve-out only answered the first.**

**Why row 2's assertion survives the retirement**, which is what makes this a
real blocker rather than a bookkeeping edge: its subject is still live after the
residual enum is emptied — **the row 2 program's recursive occurrence still gets
an IH minted, installed and consumed.** Contrast the spent exact-set oracle in
`#7`, whose subject *disappears* with the deletion. A control whose subject
survives is a control that must still pass.

> #### THE SURVIVING SUBJECT IS THE IH LIFECYCLE, NOT THE PATH LABELS
>
> Architect ruling `evt_1rzcz31qm9y9q`, 2026-08-12, closing the measurement
> opened by Steward ruling `evt_26cb49zckgq4f`. **Row 2 stays on `D3`'s bar and
> the conclusion of `evt_2jnf3x8f06psz` stands — on corrected grounds.**
>
> **The leading sentence above previously read *"both producer paths must
> install and consume the recursive IH."* That is withdrawn.** Requiring both
> historical producer-path *labels* to survive was an over-specification: the
> semantic obligation is that the occurrence's IH is minted, installed and
> consumed on **the lane actually selected**, not that every lane carry the same
> labels.
>
> **The measurement that settled it** (`6a804eb7`, PR #1957, CI green): on the
> functionized lane row 2's recursive IH **is** separately minted, installed and
> consumed, by the carried/`Composed` route. **Row 2 is not a semantic
> regression.** Exact installed-and-consumed multisets — descent
> `{Composed, SourceMachine, SourceMachine}`, functionized `{Composed}`.
>
> **Why that keeps row 2 on the bar rather than removing it.** `#7`'s subject
> disappears with the retired mechanism; row 2's semantic subject **must still
> execute** after the mechanism and the selector hooks are gone. The contrast
> the paragraph draws is correct; only its statement of what survives was wrong.
>
> **Still not established, and no ruling depends on it:** whether the
> `SourceMachine` path is reachable on a functionized lane by **any** occurrence,
> or is generally descent-specific. The fixture builds only the recursive
> occurrence (`recursive_computational_result_depth(2, ..)`), so it cannot
> distinguish *this occurrence routes elsewhere* from *the path is dead*. That
> unmeasured reachability is **neither evidence for removing row 2 nor a
> prerequisite for `D3`'s acceptance.**

**The B-only result is precursor evidence, not the acceptance proof.** `D3`'s
final obligation is on the **real, no-hook retirement tree**, against the
corrected lane-conditional meaning — see the four proofs `evt_1rzcz31qm9y9q`
requires of `D3`, recorded in [[RT-RECURSOR-TRANSPORT]]'s frame. The row's
*meaning* is what must be unchanged there, not its current text: `D3` may
rewrite or replace the original row 2 test during its control sweep so the
no-hook final tree is the witness.

⇒ **The hook-artifact uncertainty belongs to this node's `D0`**, which already
requires an activation denominator and a cause (i) versus (ii) determination.
**If `D0` proves the hook alone caused the missing `Mint`, this node may close
without a production repair.** Until that proof exists, deleting row 2 from
`D3`'s bar would trade an **observed prospective regression** for an assumption.

**`D3`'s six-row / no-hook wording is RETAINED and is not amended** (same
ruling). The fix was the missing edge, not the criterion.
