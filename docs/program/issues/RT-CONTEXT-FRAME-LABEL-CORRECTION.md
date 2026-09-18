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

# ANSWERED, 2026-09-18. BOTH READINGS BELOW ARE WRONG. DO NOT BUILD ON EITHER.

**`D0` and `D1` ran and the dichotomy this section poses is not the mechanism.**
Measured by runtime-implementer at `7cb535be5`, carried to `9dfa6978e` (the
`crates/` diff between them is empty). The two readings are kept below **only so
nobody reconstructs them as open**; they are closed.

    reading (1)   TRUE OF EXACTLY ONE ROW -- px7m:206
    reading (2)   REFUTED IN ALL FOUR

`(2)` said the branches denote one body and resolution produces two origins for
it. `PlannedOccurrence` holds `expr: &'src RuntimeExpr`, so this is answerable
by **referent**. In all four rows the two origins name **two genuinely distinct
nodes at distinct addresses** -- no aliasing anywhere. **Resolution is faithful,
and so is `agreeing_recursive_body_unit`: it reports what it was given.**

The deciding evidence was the byte rendering of the body at each origin, not the
origins themselves. Three rows: byte-identical bodies, and the check refuses
anyway. `px7m:206`: bodies differ by exactly one leaf
(`Value(String("not-found"))` versus `Value(String("unexpected-ok"))`), and the
check is correct.

## THE THIRD READING, WHICH IS THE ONE TO BUILD ON

> **The comparison tests node identity where the property it needs is body
> equality.**

## AND THE POPULATION SPLIT NAMED IN THIS NODE IS ALSO WRONG

This node framed the four rows as splitting `px7m` versus `px7l`. **They do
not.** All four have the same shape -- a two-armed match whose arms differ. The
split is **where the resolved unit sits relative to the arms' divergence**: in
three rows it is the innermost continuation body, below the divergence, so it
comes out equal; in `px7m:206` it sits above the divergence, so an inlined
literal is inside it. **The odd row is inside `px7m`.** One population by shape,
3-1, not along the axis this node named.

# THE READMISSION CONDITION: ONE LAYER ANSWERED, THE STACK STILL UNBOUNDED

The labels' `READMISSION CONDITION UNKNOWN` is answered for **exactly one more
layer** and no further. Forced past `core.rs:1230`, all four rows reach the same
next refusal, same construct, same text:

    core.rs:9558, resolve_context_capture_claim, on views.context_capture == None
    "RT-CONTSRC-PRODUCER-LOCAL D3b refuses rather than reading the direct-
     emission claim, whose index counts binders in a lexical environment this
     consumer does not hold"

> **L2 IS NOT ESTABLISHED AS THE LAST LAYER AND MUST NOT BE WRITTEN AS ONE.**
> Forcing past a refusal shows the **next** stop, never an inventory of what
> remains behind it. Two forcings revealed two layers; there is no basis for
> believing the second is terminal. **A node framed as "the last blocker for
> these rows" would be a framing that cannot be wrong.**

## AND `L2` IS DEMONSTRABLY NOT THE LAST -- TEN MORE SITES, READ NOT FORCED

**Architect, `evt_1g71815y1kyya`, read at `9dfa6978e`.** Repair only the `None`
case at `:9558` and control falls straight into these, all in the same function
and its immediate callee:

    9570   CurrentLexical claim presented to the entry-frame consumer
    9608   entry-frame claim names a predeclared frame that is not the one held
    9637   claim names a generated context of the wrong specialization
    9661   named generated context frame the planner never interned
    9688 9706 9716 9729 9735 9748    (verify_entry_frame, continued)

plus a cross-module callee with its own refusals at
`planning/static_transition/continuations.rs:4279`,
`verify_predeclared_entry_frame_membership`, reached at `core.rs:9615`.

**Two forcings revealed two layers. One read revealed ten more.**

### THE DISTINCTION THAT MAKES THAT POSSIBLE, AND IT REFINES THE LIMIT ABOVE

The implementer's limit is exactly right **and it is a limit on FORCING, not on
KNOWING.** A forcing run is **existential**: one run, one witness, one next stop,
and no number of them composes into the universal needed here. **But refusals are
not runtime events catchable only in the act -- they are static code at named
sites, so the inventory question is call-graph reachability and the instrument
for it is READING.**

    forcing            terminates when? No answer exists. Every run yields a
                       layer, which is always available. It ends by BUDGET.
    path enumeration   terminates when the PATH ends -- lowering returns Ok or
                       reaches the row's terminal. The path is finite, so the
                       observation that ends it is one the WORLD produces.

### THE BOUND IN THE OTHER DIRECTION -- "ENUMERATE THE REFUSALS" IS ALSO WRONG

A grep for `unsupported(` in `core.rs` alone returns **342** sites. That is a
**presence oracle and nothing more** -- it does not say which are reachable on
this path -- but it is enough to establish that a flat enumeration is not the
instrument either. **The read must be PATH-SCOPED, and the scoping is the design
work.**

⇒ The measurement owed is **a bounded read along one path with a stated stopping
point, not a ladder.** If the path-scoped residue comes back too large to work by
hand, **that size is the finding: the pass reports it and stops.**

## THE ROUTING FACT THIS EXPOSES

    RT-CONTSRC-PRODUCER-LOCAL     status: merged

**The refusal now blocking all four rows is owned by a node that cannot close
them.** That is the same defect shape as `RT-SITEOP-CARRIED-WITNESS` for rows
1/2/10 and `RT-CARRIED-RESIDUAL-IH-ARITY` for rows 3-6 -- **recurring one layer
down.** Rows 3-6 therefore do NOT have an owner that can close them, and any
claim that all fifteen ignored rows are live-owned is false as of this
measurement.

# D3 IS NOT ROW WORK. DO NOT PRICE IT AS PROGRESS ON THE IGNORED ROWS.

> **No repair at `core.rs:1230` makes any row pass.**

Relaxing identity to equality readmits three rows past `1230` and **changes
nothing observable about any of the four** -- they stop immediately behind it.

**Do the repair anyway: a check testing the wrong relation is a real defect and
it stands on its own merits.** But report it as a correctness fix, never as a
row closure. What rows 3-6 need first is a **bounded** answer to how deep the
stack goes, which is a measurement and not a repair.

# The superseded framing, retained for the record

Two readings were posed here, and this node existed to decide between them
rather than to assume one:

1. **The refusal is correct.** These programs genuinely have Match branches
   declaring different recursive body units, that is a legitimate Ken shape,
   and the repair is to support it — the backend's requirement that all
   branches agree is too strong.
2. **The disagreement is an artifact of resolution.** The branches denote the
   same body, and two different `StaticOriginId`s are being produced for it by
   `resolve_recursive_unit_body`'s walk. The repair is upstream, in resolution,
   and `agreeing_recursive_body_unit` is reporting faithfully.

**These were not ranked.** The label's phrasing pointed at (2) and did not
establish it. **`D1` refuted (2) outright.** The instinct to leave them unranked
was right; the error was that neither was the mechanism.

# Related

- `[[RT-CARRIED-RESIDUAL-IH-ARITY]]` — the superseded label these four rows
  carried before `b0421afd0`. Two of its clauses are the refuted ones above.
- `[[RT-SITEOP-CARRIED-WITNESS]]` — its D2 still succeeds, inherited from
  `3f4ae2d83` and **not** re-measured in the current labels. Treat as carried,
  not as established.
