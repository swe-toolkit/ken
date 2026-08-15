---
id: CONF-STALE-RED-DISPOSITIONS
title: "Eleven conformance sites assert RED-UNTIL-BUILT or BLOCKED-ON against producers that have already landed -- the mirror of the unproducible-fixture defect, and it makes finished work read as outstanding"
status: active
owner: spec-enclave
size: M
gate: none
depends_on: [CONF-BLOCKER-MARKER-RECONCILE]
blocks: []
github: null
origin: "Produced by CONF-BLOCKER-MARKER-RECONCILE's D4 stop. spec-leader verified at exact base e2c2e6e78 that D4's premise was false -- the CAT-3, bytes-CP0, and CAT-4 producers are landed -- and the conformance-validator independently added the coupled AC-2 finding that the four buffer-I/O markers resolve to merged RT-NATIVE-FNSPLIT, whose closure records no residual build work. Steward ruling evt_bgat447r9s6w removed D4 and filed this. Steward-filed per COORDINATION §2."
---

> # THIS IS THE MIRROR OF `CONF-FMT8-LEVELTOK`, AND IT IS THE COMMONER DIRECTION.
>
> That node found rows waiting on something that **will never exist**. This one
> finds rows waiting on something that **already does**. Both read identically
> to anyone scanning the corpus — red, with a reason — and both are wrong in a
> way no test catches, because **a conformance corpus has no clock.** A
> disposition is written once, against the tree of that day, and nothing
> re-examines it when the tree moves underneath.
>
> **This direction is the more damaging one.** An unproducible row overstates
> the work remaining. A stale red **understates the work completed** — it hides
> a landed capability behind a marker that says "not yet", and the next person
> to plan against the corpus plans to build it again.

## The population, measured at `e2c2e6e78`

**Eleven sites, two shapes, one defect.**

| where | count | asserts | but |
|---|---|---|---|
| `stdlib/collections/seed-cat3-collection-laws.md` | 5 | `RED-UNTIL-BUILT` — `length`/`min`, `map`/`length`, `filter`/`mem` unlanded; the `view`/`lens` record | producers **present and ancestral** (`72c2315ca`); `prelude.rs:495` carries `fn filter` |
| `stdlib/collections/seed-cat4-maps-sets-relations.md` | 1 | `RED-UNTIL-BUILT` — every CAT-4 op is net-new | cited `0f941e96a` is **not** ancestral, **but the base contains the named Map operations and proofs anyway** |
| `surface/bytes-io/seed-bytes-io.md` | 1 | `RED-UNTIL-BUILT (CP0)` | CP0 producer **present and ancestral** (`6088e0b8a`) |
| `behavioral/buffer-io/seed-buffer-io.md` | 4 | `BLOCKED-ON-NATIVE-REACHABILITY ([[RT-NATIVE-FNSPLIT]])` | that node is `merged`, **its closure records no residual build work, and its contract requires these matrices to flip** |

**The CAT-4 row is the instructive one and must not be adjudicated by its
citation.** Its cited SHA is genuinely not ancestral, so a check that stops at
provenance concludes "correctly pending". **The operations and proofs are in the
base regardless.** ⇒ **Adjudicate against the tree, not against the row's cited
commit.** A stale citation and a stale disposition are independent failures and
this row has one without the other.

## What this node is NOT

**Not a marker or link exercise.** [[CONF-BLOCKER-MARKER-RECONCILE]] owns
ownership metadata and is explicitly forbidden from flipping dispositions;
this node owns the dispositions and inherits none of that node's `AC-3`.

**Not a licence to turn eleven reds green.** A landed producer is necessary and
not sufficient: the row's fixture must actually pass. **"The producer exists"
justifies re-adjudicating the row, never the verdict.**

## Deliverables

**`D1` — adjudicate each of the eleven against the tree at your base.** For
each: name the producer, cite it at `file:line` or by symbol, and state whether
the fixture the row describes can now be built and run. **Report per site, with
the fact it turns on** — not a table of verdicts.

**`D2` — for each site whose fixture is now buildable, run it and record the
real result.** Green, red, or red-for-a-different-reason are three distinct
outcomes and **the third is the most valuable one**: it means the capability
landed and the conformance expectation was wrong about it.

**`D3` — correct the disposition to what `D2` measured.** A row whose fixture
passes becomes green with its evidence. A row that still fails states the
current reason, **not the superseded one**. A row blocked on something genuinely
absent keeps `BLOCKED-ON-` with a live blocker named.

**`D4` — the buffer-I/O four, against `RT-NATIVE-FNSPLIT`'s flip contract.**
Read that node's closure and its stated contract, then report whether each
matrix meets the flip condition. **If it does, flip it.** If the contract is
unmet despite the node being `merged`, **that is a finding about the node's
closure** and it routes to the Steward — do not edit the node.

**`D5` — say what produced the staleness, in one paragraph.** Not a retro.
Whether these went stale at a known moment (a merge that should have flipped
them) or drifted unnoticed determines whether anything cheap would catch the
next one, and that is a Steward input.

## Acceptance criteria

**`AC-1`.** All eleven are adjudicated. **Control:** the count adjudicated
equals the count of the four populations at your base. **If your base's counts
differ from 5/1/1/4, report the new numbers rather than the ones here** — the
tree has moved under this corpus before, which is the whole subject of the node.

**`AC-2`.** Every disposition change cites the evidence that justifies it —
a run, with its result. **A change justified by "the producer landed" alone
fails this**, and that is the single most likely way to get this node wrong.

**`AC-3`.** No row is deleted and no row id changes.

**`AC-4`.** Rows that remain red state a **current** reason. **Control:** no
retained reason names a producer that exists at your base.

**`AC-5`.** `crates/` byte-identical to the candidate base. **Control:** blob
identity. Every gap here is measured; none is repaired.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Do not edit `crates/`.** If a fixture fails because production is wrong,
  that is a finding routed to the Steward, not a repair taken here.
- **Do not create tracker nodes.** `COORDINATION §2`.
- **Do not touch the formatting seed.** [[CONF-BLOCKER-MARKER-RECONCILE]] is in
  flight on it and the two must not collide.
- **Do not widen to seeds outside the four populations above.** If you suspect
  more, say so and stop — that is the Steward's re-cut.

## Contention

**Real and managed by path.** `CONF-BLOCKER-MARKER-RECONCILE` edits
`conformance/surface/formatting/seed-canonical-format.md` **only**; this node
edits the collections, bytes-io, and buffer-io seeds and **must not touch the
formatting seed**. The `depends_on` records that ordering. If the enclave runs
these concurrently, that path split is the whole safety argument — respect it.

## Why this earns a slot

**The FMT8 census was framed on the premise that a red row reads as pending
when it is really permanent.** This is the same reader problem with the sign
reversed, and it turned up **within one node** of that one — which suggests the
corpus's dispositions are not being maintained against the tree in either
direction. Eleven sites is enough to be worth measuring and small enough to
finish.
