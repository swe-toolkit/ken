---
id: RT-LEXICAL-ROW2-MISSING-MINT
title: "Row 2 of the lexical-recursor population fails post-compile with a missing Mint rather than at a lowering boundary, so it is not repairable by RT-LEXICAL-RECURSOR-CONSUMERS' D2"
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-LEXICAL-RECURSOR-CONSUMERS]
blocks: []
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

## Sequencing

Behind `RT-LEXICAL-RECURSOR-CONSUMERS`. Not on the RecursiveDescent critical
path: `#6d` closes on `R1`-`R3`, and `#7` `RT-DESCENT-RETIRE` gates on
`RT-RECURSOR-TRANSPORT` and `RT-FNUNIT-RESULT-TOKEN`, neither of which is this.
