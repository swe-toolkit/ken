---
id: RT-DUPLICATED-RESPONSE-BLOCK
title: "Four ignored rows stop at `two host response cases claim one operation constructor` because the plan carries the same host-response block twice, not because two cases compete: across three programs the colliding constructors ALL agree on their operation and their effect-origin deltas are a single constant. Locate where the duplication enters and decide whether the planner emits it or the collision check is measuring a legitimate shape."
status: ready
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

**A constant offset across an entire set is a duplicated block, not 29
competing claims.** Twenty-nine independent collisions would not produce one
delta; they would produce a spread. The two `rt_escape` programs are DIFFERENT
Ken programs measured separately, and they agree in shape without sharing a
measurement — which is corroboration, not one number counted twice.

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

**Co-location is not co-causation, and `S6` is the sharper control because it
shares a file with the population and does not share the cause.** Any account
of the duplication that would also predict a collision in `S6` is refuted on
arrival.

# The open question

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
work off it. **This node does not pick.** The fork is a design question about
admissible Ken programs and it routes to the Architect with a measurement in
hand. What is forbidden is treating it as already decided.

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
