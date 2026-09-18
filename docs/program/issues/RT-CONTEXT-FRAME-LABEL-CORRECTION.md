---
id: RT-CONTEXT-FRAME-LABEL-CORRECTION
title: "Four ignored rows carry a label naming this ID and the ID has no node: their labels already establish that the context-frame admission gate CANNOT readmit them at either setting, and that the surviving refusal is `plain Match branches declare different recursive body units`. Decide whether that refusal is correct for these programs -- the branches genuinely declare different units -- or whether the units disagree only because body origins are resolved from closure structure, and establish the readmission condition the labels record as UNKNOWN."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-18, fifth repair node from the RT-IGNORED-FAILING-ROWS-INVENTORY ledger on operator directive 2026-09-15 'The other tests should be fixed.' The ID was minted by the Steward at a label closure and no node was ever created for it: measured at origin/main 2f046723e, four #[ignore] labels name RT-CONTEXT-FRAME-LABEL-CORRECTION as their owner and `ls docs/program/issues/` returns zero files matching it. Four of the fourteen failing-ignored rows have been pointing at an owner that does not exist. Steward-filed per COORDINATION section 2."
---

# The four rows, and the defect in the tracker

Row coordinates are the `#[ignore]` **attribute** line.

    crates/ken-cli/tests/px7m_hostresult_computational_match.rs:163
    crates/ken-cli/tests/px7m_hostresult_computational_match.rs:206
    crates/ken-cli/tests/px7l_checked_host_recursive_bind.rs:163
    crates/ken-cli/tests/px7l_checked_host_recursive_bind.rs:241

All four carry `RT-CONTEXT-FRAME-LABEL-CORRECTION` as their owning ID. **No
node by that ID exists.** A row whose recorded signature names a node that will
never close it reads as owned and is not. **Stale prose is cosmetic; a stale
identifier is a routing defect**, and this one has been live since the ID was
minted.

# What the labels already established. Do not redo any of it.

The labels are unusually complete and were measured at `b0421afd0`. They are
this node's most valuable fixed input, and **two clauses are already recorded
REFUTED** — do not re-open either:

- **REFUTED: "a single `Option` slot overwritten per construction, so the slot
  holds only the last."** `constructed_context_frame` is written **exactly
  once** per compile on all four rows. No frame is lost to a second write.
- **REFUTED: "it readmits when the slot is keyed by `worker_body_origin`."**
  Keying it readmits **nothing**.

**And the admission axis is measured RED AT BOTH ENDS:**

    admit fewer (today's setting)   Ok(None); control falls to the
                                    zero-argument route, which reports the
                                    BoundaryCarrier arity
    admit more  (frame arm forced   still fails, at
    to return Ok(true), strictly    agreeing_recursive_body_unit:
    more permissive than any key)   "plain Match branches declare different
                                    recursive body units: X versus Y"

⇒ **No setting of this gate passes these rows.** The BoundaryCarrier arity text
is a **fallback symptom, not the mechanism**, and the arity refusal itself is
correct and must not be relaxed.

# The code says the gate was never the authority

Measured at `origin/main` `2f046723e`. The labels' coordinate for the admission
query has drifted by one to three lines; the mechanism is unchanged.

`crates/ken-runtime/src/cranelift_backend/lowering/core.rs:13572-13584` carries,
in the tree, the reason relaxing it cannot work:

> Admission here is a PERMISSION, not the authority. The consumer re-matches
> the frame on the complete planner-issued coordinate key and re-checks both
> cardinalities against the context's own declared frame header, so a frame
> admitted here and wrong there refuses at the call. The two cannot disagree
> silently.

**That comment and the labels' both-ends-red measurement are the same finding
reached two ways**, and together they retire the admission gate as a repair
site. This node does not touch it.

# The surviving refusal

`agreeing_recursive_body_unit`, `core.rs:1230-1252`: it takes the units
declared by the Match branches, and refuses on the first disagreement.

    px7m:163   369 versus 332
    px7m:206   378 versus 341
    px7l:163   343 versus 322
    px7l:241   362 versus 347

The label states, and this node must decide: **"the resolved body origins come
from the closure structure rather than from the frame."** The resolver is
`resolve_recursive_unit_body` at `core.rs:13597`, which walks `Match` cases and
collects one declared unit per case.

# The open question, and the one the labels flagged UNKNOWN

Two readings survive, and this node exists to decide between them rather than
to assume one:

1. **The refusal is correct.** These programs genuinely have Match branches
   declaring different recursive body units, that is a legitimate Ken shape,
   and the repair is to support it — the backend's requirement that all
   branches agree is too strong.
2. **The disagreement is an artifact of resolution.** The branches denote the
   same body, and two different `StaticOriginId`s are being produced for it by
   `resolve_recursive_unit_body`'s walk. The repair is upstream, in resolution,
   and `agreeing_recursive_body_unit` is reporting faithfully.

**These are not ranked here.** The label's phrasing points at (2) and the label
does not establish it.

**And the labels record their own limit, which this node must respect:**

> READMISSION CONDITION UNKNOWN: the stack behind this gate was measured only
> in the forced configuration and only to its first stop, so whether
> `core.rs:1230` is the last layer or the next in a queue is NOT established --
> do not read either into this label.

⇒ **Establishing that is the node's first deliverable.** A repair proposed
before it is known whether `1230` is the last layer is a repair priced against
an unknown remaining stack.

# Related

- `[[RT-CARRIED-RESIDUAL-IH-ARITY]]` — the superseded label these four rows
  carried before `b0421afd0`. Two of its clauses are the refuted ones above.
- `[[RT-SITEOP-CARRIED-WITNESS]]` — its D2 still succeeds, inherited from
  `3f4ae2d83` and **not** re-measured in the current labels. Treat as carried,
  not as established.
