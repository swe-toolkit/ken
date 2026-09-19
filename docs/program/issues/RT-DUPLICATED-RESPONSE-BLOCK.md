---
id: RT-DUPLICATED-RESPONSE-BLOCK
title: "Four ignored rows stop at `two host response cases claim one operation constructor` because the plan carries the same host-response block twice, not because two cases compete: across three programs the colliding constructors ALL agree on their operation and their effect-origin deltas are a single constant. Locate where the duplication enters and decide whether the planner emits it or the collision check is measuring a legitimate shape."
status: active
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-18, third repair node from the RT-IGNORED-FAILING-ROWS-INVENTORY ledger on operator directive 2026-09-15 'The other tests should be fixed.' RT-HOST-RESPONSE-ROUTE-KEY-COLLISION (landed ef11485dd, annotation-only) ruled BOTH of its own section-3 branches out and closed with AC-4 unreached, stating in its own words: 'the right repair addresses the duplication, not the key -- so relaxing the key is the wrong unit.' It named the duplication and did not own it. Measured by the Steward at origin/main 60df2cfd2: zero occurrences of that duplication as a subject in any docs/program/issues or docs/program/wp file. This node is that owner. Steward-filed per COORDINATION section 2."
---

> # THIS NODE ABSORBED A DUPLICATE OF ITSELF. THE MATERIAL BELOW CAME IN WITH IT.
>
> **Steward, 2026-09-18: I filed this work twice, frame and all.**
> `RT-HOST-RESPONSE-DUPLICATE-PRELUDE-BLOCK` was filed at 16:17:02Z — ten
> minutes after this node at 16:07:25Z — against the same four rows, the same
> producer, the same census numbers (29 / all agree / one delta: 365, 317, 317),
> and the same merged predecessor. It is **the same defect, not a neighbour.**
>
> **This id survives because it is the one in flight** and the one the ring's
> posts, branch and D3 census cite. An id is a citation key; renaming mid-turn
> to the tidier name would break every reference for a cosmetic gain. The
> duplicate is `superseded` and points here.
>
> **The later filing was the RICHER document**, which is why this is a fold and
> not a delete. Four things it carried that this node did not are folded in
> below under their own headings: the refuted-premise interrogation, the
> `RT-CLOSURE-BOUNDARY-LANE` undetermined state, the readmission-condition debt
> the four labels name, and the deliverable shape. **The premise interrogation
> is the one that changes what an implementer may size** — read it before `D2`.
>
> ### THE FIRST FOLD MOVED MATERIAL AND LEFT THE AUTHORITY BEHIND
>
> **The fold above was incomplete when it landed, and the missing piece was the
> one the fold was for.** All four items moved into this NODE. What did not
> move was in the closed node's **FRAME**: a design-fork section and an `AC`
> reserving the *"is the invariant too strong"* question to the Architect,
> with *"an edit that lands without that citation is out of scope by
> construction."*
>
> Meanwhile this node's frame had `D1'` + `AC-4` — **the implementer justifying
> the relaxation and proceeding.** So node and frame contradicted each other on
> the one question the fold was about, and the scope moved with it: Arm B's
> repair, which the closed frame scoped out to a successor, became executable
> inside this node. **Restored in the frame's §3a, `Dr` and `AC-6`.**
> Architect, `evt_261zdt47mgr9p`; independently reached by the implementer.
>
> **The general shape, and it is the more expensive half of the duplicate
> story:** *a fold moves material and does not move authority.* **Material is
> what a document SAYS and you can diff it. A gate is what a document FORBIDS,
> and it appears in neither document's diff** — the closed frame still contains
> its `AC` intact, and the surviving frame never lacked anything it once had.
> **The loss is between the two files, so no diff of either one shows it.**
>
> ⇒ **Folding is not "is the unique material preserved."** It is **"for every
> obligation the closed artifact imposed, which artifact imposes it now"** —
> enumerated over the closed FRAME's deliverables and ACs **by position, not by
> recall.** The search-key rule applies to the folder: this fold was checked
> against what the node said, and the binding text was in the frame.
>
> **And an assignment does not travel either.** The Architect's standing note
> read *"the `RT-HOST-RESPONSE-DUPLICATE-PRELUDE-BLOCK` fork is mine when
> released."* That node is now void and this node's frame did not name them, so
> **the assignment would have evaporated silently.** They re-attached it by
> hand. Enumerate assignments alongside obligations.

# The refusal, and what the predecessor already settled

Four of the fourteen failing-ignored rows on `main` stop at one byte-identical
planner message:

    two host response cases claim one operation constructor

    crates/ken-runtime/src/cranelift_backend/planning/
      static_transition/responses.rs:1281

The producer is `host_response_routes`, which inserts into a
`BTreeMap<RuntimeSymbol, HostResponseRoute>` keyed on `case.constructor` alone
and errors on any second insert at the same key.

**`RT-HOST-RESPONSE-ROUTE-KEY-COLLISION` already decided this is not a key
problem, and the decision is landed evidence, not an opinion.** It built the
key-widening repair — deferring the refusal from construction to use — ran it,
and reverted it before commit. That measurement is its section 9.5 and it is
this node's most important fixed input: the deferral **splits the population
2/2 and closes neither half.**

# What this node is NOT allowed to redo

The predecessor's section 3 had two branches and **both are refuted**. Do not
re-open either:

- **DIFFERENT (fix the key)** — refuted. The colliding constructors do not
  disagree about their operation; across all three measured programs every
  colliding constructor agrees on its `operation`. There is no key too narrow
  to distinguish cases that never differ.
- **SAME (the tests are wrong)** — refuted. The four rows assert
  native/interpreter agreement on valid Ken programs and do not mention the
  colliding construct at all.

**AC-4 of that node was recorded "not reached: no key change is landed, so
there is no widened key to justify."** That is the seam this node enters
through, and it is why the unit of repair moved.

# The evidence that names duplication rather than competition

Per-plan censuses, each run to completion and counted by the probe rather than
by grep:

| program | colliding constructors | agree on operation | effect-origin deltas |
|---|---|---|---|
| `px7n-nested-computational-eliminator` | 29 | 29 of 29 | one distinct value, 365 |
| `rt_escape_escape_file_then_readat` | 29 | 29 of 29 | one distinct value, 317 |
| `rt_escape_nat_fanout_escaped` | 29 | 29 of 29 | one distinct value, 317 |

**Re-measured at `9dfa6978e` by this node's `D3`, with the same instrument:
every cell above is unchanged.** The only crate that moved between the framing
base `60df2cfd2` and `9dfa6978e` is `ken-elaborator/src/parser.rs`;
`ken-runtime` and `ken-cli` are byte-identical across that range, so the parser
was the single route by which a number could have moved, and it did not. The
counts are **per invocation** of `host_response_routes`, which runs **twice**
per compile on all three programs — a line count over the probe's output
therefore reads 58 where the census reads 29, and the two numbers are correct
about different quantities.

**A constant offset across an entire set is a duplicated block, not 29
competing claims.** Twenty-nine independent collisions would not produce one
delta; they would produce a spread.

**CORRECTED at `9dfa6978e`: the cross-program agreement rests on TWO programs,
not three.** The two `rt_escape` programs were censused separately and no
census value transfers between them — but every proc of
`rt_escape_escape_file_then_readat` also appears in
`rt_escape_nat_fanout_escaped`, with `after_file_escape`, `handle_outer` and
`main` byte-identical and `read_body` differing in four lines. Two agreeing
censuses across those two rows are **one shape seen twice**, not two
independent replications. The independent pair is `px7n` and one `rt_escape`
program.

**And the transfer behind that correction is UNTESTED.** The proc-level overlap
was measured because the two programs produce the same `ComputationalMatch`
scrutinee refusal — a different refusal, reached by a different probe, from the
collision census this paragraph is about. Whether the **collision** replicates
independently across the two plans has been measured by nobody. Restoring
"three" requires that measurement; the overlap above does not license it, and
neither does its absence license reading the two rows as independent.

# Why deferral does not close the rows, and the split it produces

- **`px7n:149`, `px7n:170` — the collision clears and a SECOND refusal is
  underneath.** They then fail with `OrientedSubcontinuationPlanV1: checked
  Runtime frame marker was consumed more than once`, which is
  `[[RT-FRAME-MARKER-ONCE]]` (`draft`). Their original labels were never stale;
  they were SHADOWED by a newer refusal stacked in front.
- **`rt_escape:653`, `rt_escape:713` — unchanged under deferral.** The
  constructor each row SELECTS is `FSOp::ctor_543` qualified by its own
  program, and that selected constructor is itself one of that plan's 29
  counted collisions. So the collision is not merely adjacent to these rows'
  refusal; it is the thing selecting.

⇒ **Resolving the duplication is necessary for all four and sufficient for
none.** This node owns the duplication. It does not own
`[[RT-FRAME-MARKER-ONCE]]`, and it must not absorb it.

# The discriminating control that already exists

`S6` (`escaped_buffer_used_by_fanning_host_op_matches_interpreter`) lives in
the same file as two of these rows, **gets PAST response-route construction**,
and fails elsewhere — on source-specific inheritances disagreeing on their
typed consumer projection. `S8` likewise emits no collision at all.

**Measured against the instrumented build at `9dfa6978e`, which sharpens the
control rather than merely confirming it:** `S6` reports
`overwrites=0 colliding_constructors=0 distinct_deltas=[] routes_final=29`. It
builds a route map of **the same size as the colliding programs' — 29 — with
zero collisions in it.** So the control is not "a program that happens to be
smaller"; it is a program that reaches the same map cardinality by a plan that
presents each constructor once. It then refuses at `source-specific
inheritances at one generated entry disagree on their typed consumer
projection, including the fresh-result route`, exactly as recorded.

**Co-location is not co-causation, and `S6` is the sharper control because it
shares a file with the population and does not share the cause.** Any account
of the duplication that would also predict a collision in `S6` is refuted on
arrival.

# The open question — CLOSED. READ THE RULING BELOW BEFORE THIS SECTION.

> **This heading and the two readings under it are the RECORD OF WHAT WAS
> ASKED. Both were REJECTED by the Architect, `evt_4eghtvj2fhpz0`,
> 2026-09-18** — the ruling is in the next section and in section 8.9a of this
> node's work package. **Neither reading is the answer, and reading (2) below,
> left standing alone, authorises deleting `responses.rs:1279`, which the
> ruling states is the only guard against a silent wrong-continuation route.**
>
> Steward, 2026-09-18: the ruling was recorded downstream of here and this
> section was not marked, so **a reader who stops at the section titled "the
> open question" gets the pre-ruling answer from the node that owns the
> ruling.** Related: `[[RT-HOST-RESPONSE-OCCURRENCE-KEY]]` inherited exactly
> reading (2)'s sentence as a live claim (Architect, `evt_5m7k7k4vrg5ay`).

Two readings survive the evidence, and this node exists to decide between them
rather than to assume one:

1. **The planner emits the block twice.** The same host-response block is
   planned or walked twice, at a fixed origin offset, so `source_occurrences`
   presents one program's response cases to `host_response_routes` more than
   once. The collision check is then reporting a real planner defect
   faithfully, and the repair is upstream of the check.
2. **The duplication is legitimate and the check is the wrong instrument.**
   The shape is intended — the same constructor genuinely appears under two
   occurrences that both belong in the plan — and a construction-time
   uniqueness assertion over `case.constructor` is simply not a property the
   plan was ever required to have.

**These are not ranked here, and the frame does not prefer one.** Reading (1)
is where the phrase "duplicated block" points, and that phrasing came from a
census of deltas rather than from reading the producer — so it is a
description of the symptom, not yet a diagnosis of the cause. Establishing
which reading holds is deliverable D0, and it is the whole of the node's first
turn.

# THE CONCLUSION THAT OUTLIVED ITS PREMISE — interrogate before sizing `D2`

**Folded in from the duplicate filing. This is the half that bears on what you
are allowed to size.**

The predecessor diagnosis's own amendment carries:

> *"The conclusion stands; the reason does not."*

The refuted reason was *"a program that performs `RandomBytes` at two sites
with two response handlers is valid Ken"* — **measured false, because these
programs perform `RandomBytes` zero times.** `EntropyOp` is a prelude
declaration and neither test file mentions entropy. The surviving conclusion is
*"the invariant is too strong; pairing N `Vis` sites to N handlers is planner
work."*

**Ask what that conclusion LICENSES.** It licenses the expensive arm — planner
work to make routing occurrence-aware — and **the only case ever offered for it
was the one measured false. No replacement case is on the record.** If every
actual second entry is a materialization artefact, then the invariant is not
too strong; it is correctly refusing a plan that should never have contained
the block twice, and the repair is upstream of the planner entirely.

⇒ **This maps onto the fork above:** inheriting *"the invariant is too strong"*
as settled is choosing reading (2) without measuring it, and sizing planner
work off it. **This node did not pick.** The fork was a design question about
admissible Ken programs and it routed to the Architect with a measurement in
hand.

**RULED, 2026-09-18, by the Architect. The sentence above is spent and is kept
only as the record of why the question was asked.** The ruling is **neither
arm**, and its full text is section 8.9a of this node's work package:

- **The uniqueness invariant is CORRECT.** It is decided on the **consumer**,
  which neither arm mentioned: `selected_host_response_route` looks up by
  constructor alone, and its uniqueness check is *within one Vis subtree*, so
  it guarantees a Vis selects at most one route and says nothing about whether
  it selected the **right** one. The two colliding copies agree on `operation`
  and differ on all three origins — the fields naming which producer and which
  continuation the response reaches — so they are **not interchangeable**, and
  `responses.rs:1279` is the guard against a **silent wrong-continuation
  route**.
- **Neither "de-duplicate" nor "relax the key" survives.** Per-arm inlining of
  a shared callee is legitimate, and the invariant is a claim about the source
  dispatcher while `plan.source_occurrences` is post-inlining — the subject is
  wrong, not the claim. Re-keying the **producer alone** leaves the consumer
  selecting by constructor, so the collision disappears and the mis-route does
  not.
- **The repair is a third thing: producer and consumer move together, or
  neither moves.** `[[RT-HOST-RESPONSE-OCCURRENCE-KEY]]` is **one part** of it
  and is not sufficient alone.

**What is forbidden is no longer "treating it as already decided" — it is
treating it as still open, or treating the occurrence key as the whole
repair.**

**Two repairs already ruled out — do not re-propose them:**

- **Widening the key to `(constructor, operation)` changes nothing.** Both
  entries already carry `EntropyRandomBytes`. Measured.
- **Deferring the refusal to point of use is a measured NON-FIX for half the
  rows.** See the split above.

# THE SECOND BLOCKERS' STATUS, AND ONE OF THEM IS UNDETERMINED NOT CLEARED

- **`[[RT-FRAME-MARKER-ONCE]]` — `status: draft`, and ZERO rows in the tree
  cite it today** (the diagnosis relabelled the two that used to). A `draft`
  node with no citing rows is not a released repair, so **do not size this node
  as though the `px7n` pair readmits at its end.**
- **`[[RT-CLOSURE-BOUNDARY-LANE]]` — `status: merged`**, one citing row
  remaining elsewhere. Whether it is *also* a real blocker under the
  `rt_escape` pair is **UNDETERMINED**: nothing has ever seen past the collision
  on those rows, so the durable-lane claim is neither confirmed nor refuted.
  **Undetermined is the honest state — do not record it as ruled out.**

# WHAT THE FOUR LABELS PROMISE, WHICH IS WHY THE ROWS CANNOT MOVE

All four rows' `#[ignore]` strings state a readmission condition —
*"readmits when the duplicate prelude block is resolved"* — **which named work
no node covered until this one.** That is the debt this node discharges, and it
is the reason the rows are parked rather than merely failing.

# THE DELIVERABLE SHAPE — NOT A PROMISE OF FOUR READMISSIONS

The deliverable is **four rows re-dispositioned by file and line, each either
readmitted or carrying a MEASURED next blocker.** The ledger's central finding —
12 of 27 stated reasons already false — is why a stated blocker is never
accepted in place of a measured one.

This node is **not** a rewrite of the static-transition planner (if the
Architect rules the occurrence-pairing arm, that is a successor node with its
own frame, cut by the Steward), **not** the other eleven selected rows, and
**not** `RT-FRAME-MARKER-ONCE` or `RT-CLOSURE-BOUNDARY-LANE` — it measures
whether they are reached and repairs neither.

# 2026-09-19: `ready` -> `active`. THE STATUS WAS CORRECT; THE RELEASE IS NEW.

**Flagged by the runtime-leader (`evt_1zj4ezc15qrvb`) as a possibly-stale
`ready`, on the grounds that `fb214f7c5` is titled "AC-11 discharged, AC-8
fenced". Adjudicated by the Steward and recorded here so the shape is not
re-surfaced: the status was NOT stale.**

    fb214f7c5   DOES touch this node's wp frame (5 files, two lanes). Its
                subject names the LANG node, which is why it reads as
                another lane's commit. The citation was correct.
    d9d8d692f   ancestor of main. One file, +323/-0, section 8.11a-i.
                Purely additive. No production change, no repair, no row
                readmitted.

**AC-11 discharged PLUS AC-8 fenced is consistent with an open node**, because
`AC-8`'s fence is a statement of remaining work in its own words: *"NEITHER
repair is justified: not the key change, not the relaxation. This is the node's
state, not a stall."* A fence that names what would decide it is not a closure.

> **The general form, which is the part worth carrying.** A discharged AC and a
> fenced AC look alike in a commit subject and mean opposite things about
> whether a node is finished. **Read which ACs the commit moved and what the
> fence says, never the count of ACs named in the subject line.** The subject
> line is the one surface that cannot express the difference.

**Released 2026-09-19 to the runtime ring as the `AC-8` discriminator
increment** (kick `evt_20yrkxxjt5t0w`). Scope is one predicate at one
coordinate: **`repeated_producer`'s reachability on the four ignored rows**,
`responses.rs:3326-3328`, the sole unmeasured production `USE` surviving
8.11h's classification. Both outcomes are pre-committed in `AC-8`'s own text.

**Not released with it:** `[[RT-HOST-RESPONSE-OCCURRENCE-KEY]]` stays `draft`.
Relaxation is refuted by 8.11h-2's `ANY USE` return and must not be re-proposed
as a shortcut. The sort-key residual (`:2069`, `:2286-2291`, `:2900`, `CARRY`
only if `StaticResponseContinuationId::from_position` is label-only) is
recorded unresolved and is not folded into this increment.
