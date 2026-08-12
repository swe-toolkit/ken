---
id: RT-LEXICAL-ROW2-MISSING-MINT
title: "Row 2 of the lexical-recursor population fails post-compile with a missing Mint rather than at a lowering boundary, so it is not repairable by RT-LEXICAL-RECURSOR-CONSUMERS' D2"
status: active
owner: runtime
size: S
gate: none
depends_on: [RT-LEXICAL-RECURSOR-CONSUMERS]
blocks: [RT-RECURSOR-TRANSPORT]
github: null
origin: Runtime D0/D1 checkpoint evt_3wzr30y2jjh41 at exact 0de4f130a9864623afcffa0b4751f65ddd87e818, QA-approved at evt_1f9hknbynb5nk, measured at 9adeb30f. Steward ruling evt_rcjr99tjga9 removed this class (R4) from RT-LEXICAL-RECURSOR-CONSUMERS. Steward-filed per COORDINATION §2.
---

> # FRAMED AND `ready`. NOT RELEASED.
>
> Frame: `docs/program/wp/RT-LEXICAL-ROW2-MISSING-MINT.md`, shovel-ready.
> `RT-LEXICAL-RECURSOR-CONSUMERS` holds Runtime's lane.
>
> **This node was `draft` pending one measurement, and that measurement is in.**
> The three `B`-only compiles deferred after the first abort are all `R1`
> (Architect `evt_nae7n2yxg0mk`, accepted with `RT-LEXICAL-RECURSOR-CONSUMERS`
> `D2a`), which closed the `R1` cell at five compiles across rows 1 and 4 and
> **reassigned nothing to this node and added no sibling.** The population here
> is one row, and now on evidence rather than by default.
>
> `ready` means framed, not start-it-now. See the frame's *Contention*: this
> node's files are the active recursor arc's files, and it is not on the
> `RecursiveDescent` critical path.

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
residual enum is emptied — **both producer paths must install and consume the
recursive IH.** Contrast the spent exact-set oracle in `#7`, whose subject
*disappears* with the deletion. A control whose subject survives is a control
that must still pass.

> **THIS PREMISE IS NOW UNDER MEASUREMENT — do not cite it as settled.**
> Steward ruling `evt_26cb49zckgq4f`, 2026-08-12, on the `D1` handback.
>
> `D1` measured that under `B`-only exclusion the `SourceMachine` seat is
> entered with `LoweringOperand::Carried` and returns before the mint, so the
> functionized lane requires **zero** `SourceMachine` installations for this
> occurrence. **If that generalizes to every recursive occurrence, then row 2's
> subject does NOT survive the retirement and this paragraph is false** — row 2
> would be precisely the spent oracle it contrasts itself against.
>
> **It is not established that it generalizes**, and the fixture cannot decide
> it: `px8j_all_three_producer_paths_reach_real_consumers` constructs only the
> recursive occurrence (`recursive_computational_result_depth(2, ..)`), so it
> cannot distinguish *this occurrence routes elsewhere* from *the path is dead*.
> The deciding measurement is in the frame's `D1` closure block, and the
> generalization is the **Architect's** call on its result — not the row's.
>
> Until it returns, row 2 **stays** on `D3`'s bar. Removing it now would trade
> an observed prospective regression for an assumption, which is the same trade
> `evt_2jnf3x8f06psz` refused.

**The B-only result is precursor evidence, not the acceptance proof.** `D3`'s
final obligation is the unchanged row green on the **real, no-hook retirement
tree**.

⇒ **The hook-artifact uncertainty belongs to this node's `D0`**, which already
requires an activation denominator and a cause (i) versus (ii) determination.
**If `D0` proves the hook alone caused the missing `Mint`, this node may close
without a production repair.** Until that proof exists, deleting row 2 from
`D3`'s bar would trade an **observed prospective regression** for an assumption.

**`D3`'s six-row / no-hook wording is RETAINED and is not amended** (same
ruling). The fix was the missing edge, not the criterion.
