---
id: RT-D5B-POSTCALL-REFUSAL-MECHANISM
title: "What is the mechanism of the CheckedIhDetachedCallerCut refusal at lowering/core.rs:7720 -- the only distinct Packaging reason in abi_s6_mapping_file_backed_native and the cause of 8 of its 11 base reds, where a two-step SelfDefining post-call consumer receipt meets one computational eliminator frame. THREE mechanisms have been proposed and measured away in one session, all sharing the premise that the defect is reachable from the call site's inputs; the measurement says those inputs are internally consistent and the disagreement is upstream of them. The mechanism is UNKNOWN and this node exists to find it, not to carry a candidate"
status: draft
owner: runtime
size: M
gate: architect
depends_on: []
blocks: []
github: null
tier: T1
origin: "Cut by the Steward 2026-09-14, superseding two unlanded drafts withdrawn before commit (RT-POSTCALL-RECEIPT-ALIGNMENT-AUTHORITY, RT-SELFDEFINING-RECEIPT-FRAME-ARITY), each named for a mechanism refuted within minutes of being written. This node is deliberately named for the QUESTION. Chain: implementer evt_4y7sypns2j0v, Steward hold evt_5t0n56p3jvpck, implementer withdrawal evt_1281f1q5v7j9p, Architect evt_5x5cfwgp4bk24 + evt_41r81fdj0741f + evt_pzd11rqycqe, implementer measurements evt_49vn1p2c57e7b + evt_5ycnn29hy38qs + evt_3gyahpfpnja76. COORDINATION §1a fired by the Steward on the third refuted mechanism."
---

> # READ FIRST: THREE REFUTED MECHANISMS. THIS NODE CARRIES NO CANDIDATE.
>
> Each was proposed by a competent seat, was locally reasonable, and was killed
> by measurement — two of them by the proposer's own stated criterion. **Naming
> a fourth before reading this list is the failure mode this node was recut
> twice to avoid.**
>
> **1. TRIM PLACEMENT — VOID.** *"Where does trim authority live for `:7720`"*
> has no content: `:7720` is reached only under `SelfDefining`, where
> `executable_exits()` and `selected_case_exits()` return the **same value**.
> There is nothing to trim. (Architect fork `evt_5x5cfwgp4bk24`, voided by
> measurement `evt_49vn1p2c57e7b`.)
>
> **2. ACCESSOR CHOICE — DEAD EVERYWHERE, NOT JUST HERE.** Swapping the accessor
> at `:7720` changes nothing. Grounded by a **closed producer enumeration**, not
> by sampling: a `CheckedIhPostCallConsumer`'s selection is `None` or
> `SelfDefining` and `CallerCompleted` is **unconstructible** on it
> (`evt_5ycnn29hy38qs`).
>
> **3. MISSING ROUTE — REFUTED NUMERICALLY.** `:7720` not routing a
> required-consumer incoming edge is real (see the surviving census below) but
> **does not explain the mismatch**: `incoming_consumer_edge_index` is **0** at
> all 11 reaches, so `executable_exits()[0..]` is the whole 2-chain and
> `&eliminators[0..]` is the whole single frame. **Both halves of the
> correspondence are no-ops at index 0.** Routing yields the identical 2-against-1.
> (Architect account `evt_pzd11rqycqe` §5, refuted on its own stated criterion by
> `evt_3gyahpfpnja76`.)
>
> **The shared premise of all three: that the defect is reachable from this call
> site's inputs.** The measurement says those inputs are internally consistent at
> index 0, and the disagreement is **upstream of all of them**.

## The observation

`lowering/core.rs:7719-7720` at `0d94d58b6` — re-derive at your base (`AC-1`):

    let residual = self
        .checked_ih_post_call_residual(consumer.selected_case_exits(), eliminators)?;

Measured across the whole suite, no variation:

    11 x   site=7720  index=0  executable_exits=2  selected_case_exits=2
                              suffix=2            eliminators=1

The receipt describes a two-step exit chain; the site holds one computational
eliminator frame; the bounds guard at `core.rs:9256` refuses. **This is the only
distinct `Packaging` reason in the run and the cause of 8 of 11 base reds.**

## What is NOT in doubt

- **Completeness, not soundness.** The compiler **rejects a valid program**; it
  does not admit an invalid one. Rests on the `:9256` bounds guard and the
  `#[track_caller]` counts, which survived every correction.
- **The guard works and must not be weakened.** When a probe equalized the
  counts, the refusal **moved** to the per-frame content arm at `:9268` and the
  wrong defining call still failed to compile.
- **No length-derived trim, at any of the five call sites, ever.** Architect,
  `evt_41r81fdj0741f`: `caller_exit_index` is an alignment witness; a difference
  of two lengths is not. Structural and variant-independent — **and now vacuous
  here**, since there is nothing to trim.
- **`:7720` is the only site in the crate that stops after the first half of the
  two-step protocol.** Census at `evt_pzd11rqycqe` §4; the protocol is written
  out at `planning/static_transition/aggregates.rs:7826-7829` and
  `responses.rs:598-606`. **This survives the refutation of mechanism 3** — it is
  a real asymmetry, it is simply not the cause.

## Deliverables

**`D1` — FIND THE MECHANISM. Upstream of the call site, not at it.** Which
producer emits a two-step exit chain for a body the lowering site reaches with
one computational frame, and **where does the disagreement actually live?**
**Both operands are internally consistent at `:7720`** — start where they are
produced, not where they meet.

> **`D1` ADMITS THREE OUTCOMES AND MUST NOT BE READ AS A TWO-WAY CHOICE**
> (Architect, `evt_53mb86qj64fj1`):
>
>   1. the receipt is wrong, **or**
>   2. the eliminator frame set is wrong, **or**
>   3. **BOTH ARE CORRECT AND THE CUT BETWEEN THEM IS MISPLACED.**
>
> **An earlier wording of this deliverable asked "which of the two is
> inconsistent with the program?", which presupposes outcome 3 away.** That is
> the shared premise of the three refuted mechanisms, reappearing in the sentence
> meant to escape it — and the evidence points at 3: the frames equal
> `receipt[1..]` elementwise, and the index producer names the consumer's **own**
> occurrence, so both operands may be correct under one convention applied at the
> wrong place. **Do not re-narrow it.**

**State the DIRECTION and report counts with the population traced** (`AC-2`).
**"I could not determine it" is an acceptable answer** and must be reported as a
finding with its argument, not as an absence.

**`D2` — repair the co-class control. Independent of `D1`; does not wait on it.**
`wrong_defining_call_breaks_the_required_consumer_edge`
(`abi_s6_mapping_file_backed_native.rs:463`) asserts one of **three co-class
reason strings**, all emitted as `CheckedIhDetachedCallerCut`:

    :9256   "is longer than the exact local eliminator prefix"
    :9263   "does not match a computational local prefix"
    :9268   "does not match the exact local eliminator prefix"

All three mean *the edge broke*, so the control **cannot distinguish "the guard
fell" from "the guard moved"** — and on 2026-09-14 it reported the second as the
first, producing a false refutation that reached an Architect ruling before
anyone opened the panic line. Assert the refusal **class**; if an exact arm is
load-bearing, assert it separately and say why.

**This is the durable product of the session and no correction touched it.**

**`D3` — route the required-consumer incoming edge at `:7720`, as a CORRECTNESS
deliverable, explicitly NOT as the cause.** Fork on
`required_consumer_incoming_edge()` — `Ok(None)` for ordinary transports
reproduces today's behaviour, so the route is total by construction — following
`aggregates.rs:7826-7829`. **Do not let this land carrying an implication that it
fixes the refusal: measured, it is a no-op at index 0.**

**`D3` MUST follow `D1`. This is a hard sequence, not a preference** (Architect,
`evt_53mb86qj64fj1`). `D1` now decides what correct routing *means*: the open
fork is whether the cut is `[index..]` or `[index+1..]`, and the two differ by
exactly the consumer's own occurrence. **Building `D3` before `D1` settles lands
the wrong cut with a passing test** — the `[index..]` cut is a no-op at index 0,
so a green `D3` would prove nothing and look like a fix.

## THE OPEN FORK `D1` MUST SETTLE, AND THE TWO READINGS THAT NARROW IT

`incoming_consumer_edge_index` names the position of the consumer's **own**
defining occurrence in the receipt chain, uniqueness enforced by its producer
(`responses.rs:2366-2396`; no match and multiple matches are both planner
errors). With `index=0`:

    [index..]    2 steps vs 1 frame   refuses (today, and after routing)
    [index+1..]  1 step  vs 1 frame   matches exactly

    (a) the cut under-cuts by the consumer's own step
    (b) [index..] is right and the receipt should not carry that step at all

**Same symptom, opposite repairs.** Two readings that narrow this, both the
Architect's at `evt_53mb86qj64fj1`, neither selecting an arm:

- **The convention is UNIFORM across both variants — there is no
  `SelfDefining`-only asymmetry.** `occurrence_subtree_contains`
  (`occurrences.rs:290`) is **reflexive** (root is tested against the needle
  before any descent), so both index producers name the consumer's own
  occurrence. ⇒ either `[index..]` under-cuts for **both** variants, or the
  receipt should exclude that step for **both**. **This dissolves the stated
  objection to (a); it does not select (a).**
- **The two site families differ in where their eliminator list COMES FROM.** At
  `core.rs:9402-9406` and `:9421-9425` the list is **constructed from** the exit
  chain, so correspondence holds by construction whatever the convention. At
  `:7720` it is the **ambient** local frame list, where correspondence is an
  assumption. `:4348`'s provenance is untraced.

**The discriminator, recorded so it is not re-derived:** does
`selected_case_exits` contain the consumer's own occurrence for **both**
variants, **and** is the ambient frame list at `:7720` expected to contain a
frame for that occurrence at all? **Counts and the producer set, per `AC-3`** —
never a sample.

**`required_consumer_executable_suffix` and `:7720` must not be decided
separately.** They read the same cut, and `aggregates.rs:7826` consumes it too.

## Acceptance criteria

**`AC-1` — re-derive every line number and count at your own base.**

**`AC-2` — every instrument reports counts and the population it searched, never
a bare verdict.** Non-negotiable on this arc: a bare red produced a false
refutation today, and a zero-hit grep on a wrong path produced a false absence.

**`AC-3` — an enumeration must be closed at its PRODUCERS, not sampled.** The
`SelfDefining` fact generalized only when every construction site was enumerated
and found closed at two. **A larger sample of instances would never have licensed
it** (`evt_5ycnn29hy38qs`). Any "by construction" claim in this node's closure
must name its producer set.

**`AC-4` — `D2`'s repaired control must be SHOWN to discriminate** *guard fell*
from *guard moved*. Passing is not sufficient.

**`AC-5` — the guard is untouched.** No change to the conditions at `:9256`,
`:9263`, `:9268`.

**`AC-6` — nothing is synthesized to make two counts agree.** No fabricated
receipt, step, eliminator frame, or index. Manufacturing either operand is
indistinguishable from fixing the defect and tests a shape no program presents.

**`AC-7` — a proposed mechanism must be stated with its falsification criterion
before it is measured.** Both refutations that landed today worked because the
proposer named in advance what would refute them. Adopt it as the standard here.

## COORDINATION §1a HAS FIRED ON THIS CHAIN

**Chain: *"what is the mechanism of the `:7720` refusal."* Three stops, all
refuted by measurement, 2026-09-14.** The Steward's tracker is the count of
record and this is stop three. ⇒ **the Architect holds on proposing a fourth
mechanism, and research is called for a prior-art advisory.**

**The research seat is quota-dead** (fleet outage 2026-09-14, ~16 of 23 seats,
recovery near 2026-09-19). **The advisory is therefore QUEUED UNCONSUMED**, and a
participant line reading *"idle, awaiting a named research request"* is last-set
status, not liveness.

**This costs the lane nothing, and the reason is a distinction worth stating
plainly: `§1a`'s hold binds the Architect's RULING, not the ring's
MEASUREMENT.** `D1` is a measurement over producers and `D2` is a control repair;
**neither needs a ruling to proceed, and both are the whole of the ring's next
work.** So there is no five-day block to trade against, and the
rule-unaided-provisionally deviation taken on the HS24/entry-18 arc **does not
apply here** — its premise was a lane blocked on a ruling, and this lane is not.

**What the hold actually prevents** is a fourth mechanism entering the tree as
design authority on the same unexamined premise as the first three. **That is
worth holding for, because it is exactly what the last three hours produced.**

**This is not a judgment on any seat.** Two of the three were refuted on their
proposer's own stated criterion, which is the system working. `§1a` exists for
exactly this shape: a design chain that has absorbed three competent attempts
sharing an unexamined premise. The implementer declined to propose a fourth and
named the pattern in their own record; the Architect named the shared premise in
theirs.

**Distinct from the `RT-CONSTRUCTOR-AUTHORITY-DISCHARGE` `D2` chain** (*"how does
`D2` obtain discriminating evidence"*), which stands separately at **two**.

## A re-measure obligation this node creates elsewhere

**The 11 base reds were never a stable population.** Every measurement over
`abi_s6_mapping_file_backed_native` on 2026-09-14 inherited this refusal as a
mask. [[RT-DISCHARGE-LEDGER-COLLISION-SOURCE-REACHABILITY]] records a venue
finding *"the mapping fixture cannot reach the code at all — zero population"* —
**a compile dying early here produces exactly that reading.** Re-take it after
this node lands, **before** that node's `D1` is answered from it.

## Sizing

**`M`, T1.** `D2` is small and independent. `D1` is a reasoning deliverable over
producers; if it becomes an unbounded search, stop and report — that is `D1`'s
stated acceptable outcome, not a failure.

## Not this node

- **Not any of the three refuted mechanisms**, re-proposed under a new name.
- **Not a repair of any refusal arm.** All three work.
- **Not `RT-CONSTRUCTOR-AUTHORITY-DISCHARGE` or its discharge ledger.** The nine
  *"finished generated-Result proof graph is not closed"* refusals behind this
  one are that mechanism — adjacent, and not evidence about the collision shape.
