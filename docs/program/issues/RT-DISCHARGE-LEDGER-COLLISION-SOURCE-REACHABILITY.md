---
id: RT-DISCHARGE-LEDGER-COLLISION-SOURCE-REACHABILITY
title: "Is there a Ken source program in which two callers demand different constructor identities of one response-owner body -- the shape RT-CONSTRUCTOR-AUTHORITY-DISCHARGE's discharge ledger exists to catch, which no permitted instrument has been able to construct. The stake is measured, not speculative: in exactly the collision case the units.rs:4407-4422 identity-mismatch diagnostic is structurally unreachable, so if the shape IS reachable the ledger is the SOLE net"
status: draft
owner: runtime
size: M
gate: architect
depends_on: []
blocks: []
github: null
tier: T1
origin: "Cut by the Steward on the Architect's ruling evt_56zx7a2z055yg, refined at evt_5gws0pnssfqch, closing RT-CONSTRUCTOR-AUTHORITY-DISCHARGE D2's acceptance. The gate split into COVERAGE (discharged -- arm1=4 per run, the new arm-1 path is exercised not vacuous) and DISCRIMINATION (not discharged, and unobtainable by any permitted means). Filed as its own node rather than dissolved into a closing one, on the Architect's explicit instruction. The positive control that fails closed is the runtime implementer's b1575f536; its measured account is evt_7rh94vjedhaga. The units.rs:4394 finding is the Architect's, re-verified against the object DB by the Steward at 686ffa8ac rather than relayed."
---

> # READ FIRST: WHAT THIS NODE IS NOT ALLOWED TO BE QUOTED AS.
>
> **It is not evidence that the double-discharge property is established.** The
> Architect's own guard on their acceptance, verbatim: *"the net is in place, its
> coverage is measured, and whether it ever has to fire is unmeasured."*
>
> **It is not an unreachability claim.** One fixture was measured. `WRITE_ALL`
> not exhibiting the shape is **not** evidence the language cannot express it,
> and the implementer reporting the fixture and stopping there was the correct
> call rather than excess caution. **Nobody may upgrade that measurement into a
> reachability verdict — that is this node's `D1`, and it is open.**

## The question

**Is there a Ken source program in which two callers demand different
constructor identities of one response-owner body?**

That is the shape `RT-CONSTRUCTOR-AUTHORITY-DISCHARGE`'s discharge ledger was
built to catch: one function-local SSA word claimed by two call-result
obligations under different identities, reaching arm 1 and arm 2 of the guard.

## Why it matters, and this is the half that makes it a node

**Measured by the Architect at `686ffa8ac`, re-verified by the Steward against
the object DB.** In `lowering/units.rs`, the one other guard that *looks* like it
would catch an identity mismatch is structurally unreachable in exactly the
collision case:

    4380   if proven {
    4381       finished.push(FinishedUnitResultContract { unit, target, identity })
    4394   let missing = required.iter().filter(|(target, identity)| {
    4399       !finished.iter().any(|c| c.unit == target_unit
    4401                              && c.target == *target
    4402                              && c.identity == *identity) })
    4407   if !missing.is_empty() {
    4418       if actual.identity != *demanded { return Err(...) }

The collision's precondition is that **both** demands were proven. Both
therefore push into `finished` at `:4381` under their own identities; both
`(target, identity)` pairs in `required` find a match at `:4399`; `missing` at
`:4394` is empty; and the `:4407-4422` block never runs. **The `:4418`
diagnostic fires only when a demand was NOT proven — it is blind to the case
where both were.**

⇒ **If the shape is reachable from Ken source, the discharge ledger is the SOLE
net.** That is why an open reachability question here is worth a frame rather
than a sentence in a commit message.

## Why no instrument has answered it

**Three venue findings in a row, one shared shape** (2026-09-14):

1. The mapping fixture **cannot reach** the code at all — zero population.
2. `WRITE_ALL` **reaches** the code but **cannot exhibit** the shape: both
   bodies carrying an `independent_contract` are demanded under exactly one
   identity, and that identity **is** their own contract, so arm 2 never runs.
3. A mutation **cannot manufacture** the shape without synthesizing an identity,
   which `RT-CONSTRUCTOR-AUTHORITY-DISCHARGE` `:333-335` forbids in terms — a
   prohibition the Architect **extended to controls as a ruling** at
   `evt_5gws0pnssfqch`, rather than treating it as already applying there.

**The governing frame line is `:332`** — *"if either is unavailable in principle
at some site, THAT IS THE FINDING."* An in-principle unavailability is a result
to report, not an obstacle to work around. **Do not close this node by citing
`:334`'s preference clause**; that clause governs the METHOD (why we do not
manufacture) and says nothing about SUFFICIENCY.

## Deliverables

**`D1` — is the shape reachable from Ken source? Answer with the ARGUMENT, not a
fixture hunt.** Two callers demanding different constructor identities of one
response-owner body. This is a question about what the language and the planner
admit, not a search problem; an unbounded fixture search is the failure mode to
avoid. **State the DIRECTION of the answer**: reachable (the ledger can fire, and
is the sole net) versus not reachable (the ledger is defence in depth against a
shape no program presents).

**`D2` — only if `D1` is YES: a source fixture exhibiting it.** It turns the
existing fail-closed control at `b1575f536` into a live one. **Do not begin `D2`
before `D1` is answered** — funding a fixture build for a shape nobody has shown
constructible risks paying for something impossible and having the failure read
as an implementer miss rather than a language fact.

**`D3` — only if `D1` is NO: what ENFORCES it?** "No current program does this"
and "no program can" are different claims with different consequences. Name the
invariant and where it lives. An unenforced accident can regress; a real
invariant means the ledger is defence in depth and should be recorded as such.

## Acceptance criteria

**`AC-1` — the guard structure is re-derived at your own base.** The
`:4380-4422` reading above was measured at `686ffa8ac`. **Read it at your base;
do not inherit these line numbers** — that exact error is what
`RT-FORWARDING-PROOF-SATISFIER-DISJOINTNESS` was split out of.

**`AC-2` — every instrument reports COUNTS AND THE POPULATION IT SEARCHED, never
a bare verdict.** This is a requirement, not advice. On 2026-09-14 four separate
findings on this arc were caught **only** because a failure message carried
counts: a control searching the wrong relation returned a true zero about the
wrong question, which as a pass/fail would have read as a clean *"not
constructible."* An instrument that returns a verdict cannot distinguish "not
applicable here" from "measured false."

**`AC-3` — no identity or demand is synthesized**, in a fixture or in a control.
`RT-CONSTRUCTOR-AUTHORITY-DISCHARGE` `:333-338`, extended to controls by the
Architect at `evt_5gws0pnssfqch`. A synthesized identity is indistinguishable
from a real one at the type level, so a control built on one tests the guard
against a shape no Ken program can present and reports it as evidence about Ken
programs.

**`AC-4` — a negative `D1` is reported as a finding with its argument, not as an
absence.** "I could not construct one" is not an answer to `D1`. If the honest
outcome is that it is unknown, say unknown and say what would settle it.

**`AC-5` — no-regression, in CI.** `COORDINATION §12` — the venue is CI, never a
local `--workspace` run.

**`AC-6` — THIS NODE'S CLOSURE MUST DROP THE `#[ignore]` IF THE SHAPE BECOMES
PRODUCIBLE BY ANY ROUTE, NOT ONLY BY THE FIXTURE THIS NODE AUTHORS.** Architect,
`evt_1a1m9yf2b0zqn`.

`discharge_ledger_refuses_one_word_discharging_two_obligations` in
`px8f_buffer_native` is `#[ignore]`d at `0d94d58b6`, and its reify trigger names
**the fixture** — *"a Ken source program in which two callers demand different
constructor identities of one response-owner body."* That covers the route where
**someone deliberately authors the shape**, which is `D2` above and has an owner.

**It does not cover the second route: `WRITE_ALL` or its siblings DRIFTING into
the shape as the fixtures grow.** Nothing re-measures
`demanded_per_independent_body` after 2026-09-14. On that route the mutation
would begin firing correctly while the control stayed ignored — **an inert
control beside a live population**, which is the defect shape this whole arc
spent the day rejecting, arriving by calendar instead of by construction.

⇒ **Closing this node means checking both routes and dropping the attribute if
either has fired.** The trigger is the shape existing, not this node having built
it.

**A live pin on `WRITE_ALL`'s counts is deliberately NOT the mechanism** (same
ruling). It would read as *"the double-discharge control passes"* — the exact
over-quote the Architect forbade — and it would redden on a legitimate fixture
change at precisely the moment the real control should be run instead.

## The §1a count, which is live

**The chain *"how does `D2` obtain discriminating evidence"* stands at TWO
stops:** the venue refutation, and the fail-closed positive control. **A third
report that discriminating evidence cannot be obtained fires `COORDINATION §1a`**
— the Architect holds and research is called for a prior-art advisory, rather
than anyone ruling unaided. The Architect named this trigger in advance at
`evt_56zx7a2z055yg`. **The Steward's tracker is the count of record.**

## Sizing

**`M`, and the one-hour target applies to `D1` alone.** `D1` is a reasoning
deliverable; if it turns into an unbounded search, that is the signal to stop and
report, not to keep searching. `D2` and `D3` are mutually exclusive and both
depend on it.

## Not this node

- **Not `RT-CONSTRUCTOR-AUTHORITY-DISCHARGE`'s `D2`.** That closed on the built
  work: the ledger covering both arms, a measured execution witness, and a
  two-sided control that fails closed with an account of why.
- **Not the satisfier-disjointness measurement.** That is
  [[RT-FORWARDING-PROOF-SATISFIER-DISJOINTNESS]], a different mechanism
  (`prove_forwarded_value`'s three satisfier arms) and still `draft` with no
  result.
- **Not a repair of the ledger**, and not a change to the `:4407-4422`
  diagnostic. Whether that diagnostic *should* cover the collision case is a
  separate question; this node establishes whether the case exists.
