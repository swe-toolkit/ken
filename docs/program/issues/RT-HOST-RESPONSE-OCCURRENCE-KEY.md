---
id: RT-HOST-RESPONSE-OCCURRENCE-KEY
title: "Re-key the static-transition host-response route map on the OCCURRENCE, not on the operation constructor alone, so that N legitimate instantiations each contributing one response handler stop colliding -- while two response-handling sites within ONE occurrence claiming one constructor still refuse. This is the sole route to clearing all four of the failing-ignored rows that stop at `two host response cases claim one operation constructor`, and it is an undischarged assignment from RT-HOST-RESPONSE-ROUTE-KEY-COLLISION section 3a, not a new discovery."
status: draft
owner: runtime
size: M
gate: none
tier: T1
depends_on: [RT-DUPLICATED-RESPONSE-BLOCK]
blocks: []
github: null
origin: "Steward, 2026-09-18, fourth repair node from the RT-IGNORED-FAILING-ROWS-INVENTORY ledger on operator directive 2026-09-15 'The other tests should be fixed.' The occurrence-keying insight is the Steward's own section 3a amendment to RT-HOST-RESPONSE-ROUTE-KEY-COLLISION, written 2026-09-17: 'What the key actually omits is the OCCURRENCE.' It was routed to that node's AC-4 and AC-4 closed 'not reached', so the assignment has sat undischarged since. RT-DUPLICATED-RESPONSE-BLOCK then located WHERE the second presentation comes from -- per-arm inlining of a shared callee -- which makes the construction-time uniqueness assertion wrong in its SUBJECT, stated over post-inlining occurrences rather than over the source dispatcher, and NOT unnecessary. Corrected 2026-09-18 on Architect evt_5m7k7k4vrg5ay: this line previously said that node 'established the reading that licenses it', which overstates in the same direction the body did -- that node measured the origin and explicitly did NOT measure whether producing the second presentation is correct (its 8.2). The Architect ruled the repair LIVE 2026-09-18 and rejected both design arms (evt_4eghtvj2fhpz0); separately, the Architect set this node's acceptance bar and has since REPLACED it with the three-clause bar below. The liveness ruling and the bar are two acts and this line used to fuse them. Steward-filed per COORDINATION section 2."
---

# The refusal, and why the key is now the right unit when it was not before

Four of the fourteen failing-ignored rows on `main` stop at one byte-identical
planner message. Measured by the runtime-implementer 2026-09-18 at `60df2cfd2`,
each row run alone with `--exact`, current text read rather than carried:

    px7n:149   two host response cases claim one operation constructor
    px7n:170   two host response cases claim one operation constructor
    rt_escape:653   same
    rt_escape:713   same

    crates/ken-runtime/src/cranelift_backend/planning/
      static_transition/responses.rs:1281

Row coordinates are the `#[ignore]` **attribute** line throughout this node and
its frame.

`host_response_routes` builds a `BTreeMap<RuntimeSymbol, HostResponseRoute>`
keyed on `case.constructor` alone, over **every** `Match` in the plan, and
errors on any second insert at that key. The invariant it encodes is:

> one host-operation constructor ⇒ one response-handling site in the whole
> program

**That invariant is stated over the wrong UNIT, which is a narrower claim than
"too strong".** `[[RT-DUPLICATED-RESPONSE-BLOCK]]` located WHERE the second
presentation comes from — per-arm inlining of a shared callee — so
`plan.source_occurrences` is **post-inlining** while the invariant is a claim
about the **source dispatcher**. Re-keying on the occurrence fixes the unit.
**It does not retire the assertion, and this node may not be read as saying it
does.**

> ### THIS PARAGRAPH SAID "MEASURED", AND IT LICENSED DELETING `:1279`
>
> **Architect, `evt_5m7k7k4vrg5ay`, 2026-09-18. Two defects, and they failed in
> the same direction: toward a larger permission than the evidence grants.**
>
> **The verb.** It read *"`[[RT-DUPLICATED-RESPONSE-BLOCK]]` **measured** that
> the two colliding copies track two semantically different match arms."* The
> cited node's own 8.2 strikes exactly that step:
>
> > *"I measured WHERE the second presentation comes from. **I did not measure
> > whether producing it is correct** ... an earlier revision went one step
> > further and said 'and the plan is CORRECT'. It should not have. That is an
> > inference, not a measurement."*
>
> ⇒ **This node attributed as MEASURED the one step its source went back and
> marked as not-measured.** A word promising an instrument nobody ran.
>
> **The licence.** *"A construction-time uniqueness assertion over
> `case.constructor` is therefore not a property the plan was ever required to
> have"* **authorises deleting `responses.rs:1279`.** Under `evt_4eghtvj2fhpz0`
> it must not be: the consumer at `:1298` looks up by constructor alone, the two
> colliding copies agree on `operation` and differ on all three origins, so
> **`:1279` is the guard between here and a silent wrong-continuation route.**
>
> **Both design arms were REJECTED.** *"Per-arm inlining is legitimate"*
> survives inside Arm A's rejection, as the reason the assertion's SUBJECT is
> wrong. **It does not survive as a reason the assertion should not exist.** The
> old paragraph took Arm A's surviving premise and drew Arm B's rejected
> conclusion — a bar read as a licence, one level up from where that already
> happened once in this chain.

# WHY THIS IS `draft` AND WHAT FLIPS IT

**`draft` here is a BLOCKED marker, not an unframed one.** The frame is
complete and fixed at a named SHA; nothing about it is provisional.

It is `draft` because `depends_on` names `RT-DUPLICATED-RESPONSE-BLOCK`, whose
**node** is on `main` but whose **work** is not done. The schema check states
the consequence exactly: *"a team pulling this node will find its premise
false."* This node's D0 asks whether the host-response use site can name its
occurrence -- a question posed against a code path `RT-DUPLICATED-RESPONSE-BLOCK`
is about to change. **Measuring it before that lands measures the wrong tree.**

**The flip is the Steward's and its trigger is precise:** when
`RT-DUPLICATED-RESPONSE-BLOCK`'s `D2`/`D3` land on `main` **AND the gate
corrections in this change land**, re-read this node's fixed inputs against
that `main`, correct any coordinate that moved, **run the citation re-read
below**, and flip to `ready`. **Flipping on the node's status alone is not
sufficient** -- that is the condition that already holds and is why this says
`draft`.

> ### THE TRIGGER USED TO FIRE ONE LANDING TOO EARLY, AND THE GAP WAS NOT EMPTY
>
> **Architect, `evt_kk4kh7d1znhs`, 2026-09-18.** The precondition was `D2`/`D3`
> alone. `D2`/`D3` landed in `RT-DUPLICATED-RESPONSE-BLOCK`'s `e080a6966`
> (squash `5899268451d42e7c1337929a996921d6483b6541`), so the trigger fired
> **before** the corrections on this page existed on `main`.
>
> **In that window `main` held, simultaneously:** this node eligible to flip;
> the OLD trigger, scoped to *"coordinates that moved"*; and the predecessor's
> §1 Objective offering retirement of the check as *"as good a result"*, with
> its *"# The open question"* section presenting both readings as live.
> **A coordinate-keyed re-read comes back clean on all three** — which is the
> finding this page already carries, arriving in the one window where it bites.
>
> ⇒ **A correction that lands after the event it governs does not govern it.**
> The re-read is supposed to check this node's `:39-44` as a fixed input, and
> until this change lands that input is still wrong on `main`. Flipping first
> certifies text that is about to be corrected — the weakest moment to run it.
>
> **The conjunction has an equivalent single form: flip when THIS change
> lands.** It is a descendant of `e080a6966`, so it implies `D2`/`D3` and needs
> no second clause.
>
> **The licence was NOT introduced by `e080a6966`** — runtime-implementer,
> `evt_fy4afrj7wt73`, measured it live on `main` at `831e521e5`, before that
> candidate. **The exposure started earlier than the flip window; the remedy is
> unchanged.**

> ### A RE-READ KEYED ON MOVED COORDINATES COMES BACK CLEAN ON WHAT ACTUALLY BROKE
>
> **Architect, `evt_5m7k7k4vrg5ay`.** This trigger used to say only *"correct
> any coordinate that moved."* The defect corrected above is **not a coordinate
> that moved.** It is a claim whose **WARRANT** moved — and it moved by the
> cited node retracting a verb and by the Architect rejecting the arm. **A
> coordinate-keyed re-read returns nothing on it, and the node flips carrying
> the licence.**
>
> ⇒ **The question the re-read must ask instead:**
>
>     for every claim in this node that CITES the predecessor,
>     does the predecessor still say that -- and with WHICH VERB
>
> **There are exactly two, both corrected 2026-09-18, both to be re-asked at the
> flip:** the paragraph above, and the `origin:` line. Neither is reachable by
> asking whether a line number moved.

# THIS IS AN UNDISCHARGED ASSIGNMENT. IT IS NOT A DISCOVERY.

`RT-HOST-RESPONSE-ROUTE-KEY-COLLISION` section 3a, Steward, 2026-09-17:

> **What the key actually omits is the OCCURRENCE.** `host_response_routes`
> maps `case.constructor` over *every* `Match` in the plan ... **The invariant
> is too strong. The repair is NOT a tuple widening.** Pairing N `Vis` sites to
> N handlers is planner work. **`AC-4` still governs it.**

That AC-4 closed **"not reached: no key change is landed, so there is no
widened key to justify."** The assignment has been live and unowned since.
Attribute it there; do not restate it here as though this node found it.

# THE ONE-PARAGRAPH WARNING ABOUT SECTION 3a

**Section 3a carries a closure AND a live assignment, and the closure attaches
to the PEER question only.** Its closing paragraph — *"CLOSED BY 9.2 AND 9.3 ...
Do not re-measure this from this paragraph"* — closes whether the two colliding
entries are peers or whether one is a prelude occurrence. It does **not** close
the occurrence-keying assignment three paragraphs above it. **The ban's reason
was narrower than the text it sits in.** Read section 3a to the end before
concluding that any of it forbids this node's work.

# WHAT THIS NODE IS NOT ALLOWED TO REBUILD

**Deferral is not re-keying, and nobody rebuilds deferral.** Section 9.5 of the
predecessor built and measured the deferral repair — moving the refusal from
construction to point of use — and reverted it before commit. It leaves the key
**global** and the map **lossy**: the last writer wins at insert, and 58 route
overwrites were measured silently discarding the wrong instantiation's
`effect` / `producer_call` / `response` origins before any use-site check runs.
It splits the four rows 2/2 and closes neither half.

**Occurrence-keying is a different object.** It changes what the map can
*hold*, not when the map is *checked*. A repair that re-introduces a
use-site-only refusal over a global key has rebuilt 9.5 and is refused on
arrival.

# THE ACCEPTANCE BAR, SET BY THE ARCHITECT

> ### THIS BAR WAS TWO CLAUSES AND IS NOW THREE. THE THIRD REPLACES, NOT EXTENDS.
>
> **Architect, `evt_4eghtvj2fhpz0`, 2026-09-18.** The two-clause bar below was
> published here as *"THE ACCEPTANCE BAR"* and **was insufficient on its own**
> — both its clauses are producer-side, and **every producer-side control
> passes on a repair that mis-routes.**
>
> **Say "replaces", not "gains".** A reader arriving at a bar that looks
> complete will not go looking for a third clause, which is the whole failure
> mode: the two clauses below are individually correct and jointly **not a
> bar.** Architect, `evt_7ct1bwa50pne7`; Steward sweep.
>
> **And this node is ONE PART of the repair, not the whole of it.** The ruling
> is that producer and consumer move together. Re-keying alone does not clear
> the rows.

All three are required. Clause 2 is the one a tuple-widening satisfies by
accident; **clause 3 is the one nothing else in this chain checks.**

    MUST STILL REFUSE   two response-handling sites within ONE occurrence
                        claiming one operation constructor
    MUST NOT REFUSE     N legitimate instantiations each contributing one
                        response handler
    MUST NOT MIS-ROUTE  the consumer selects by the key the producer inserted
                        under, and a Vis site whose route copy is ABSENT
                        REFUSES rather than falling back to the last-written
                        route

**A repair that cannot still refuse the first has relaxed the invariant, not
re-keyed it.** **A repair that satisfies the first two and fails the third has
kept the guard green while routing every `Vis` site to the wrong copy** — which
is a silent wrong-continuation route, and is what `responses.rs:1279` is
currently the only barrier against.

# THE POPULATION IS ALL FOUR ROWS

Corrected 2026-09-18 by the runtime-implementer and the Architect
independently, against an earlier Steward per-row split that had the two `px7n`
rows blocked on `[[RT-FRAME-MARKER-ONCE]]`:

    all four   blocked at :1279 today. This node is the route to clearing it.
    px7n only  BEHIND :1279, measured, conditional on a clearance that does
               not exist today: [[RT-FRAME-MARKER-ONCE]]. Not a current
               blocker and this node does not own it.
    rt_escape  BEHIND :1279, measured under suppression: a ComputationalMatch
               tree-producing-scrutinee refusal. Whether
               [[RT-CLOSURE-BOUNDARY-LANE]] sits below THAT is UNDETERMINED
               and stays undetermined.

The earlier split would have recorded two of the fourteen rows as having moved
to a different owner when they had not moved at all.

# A CORROBORATION CLAIM IN THE PREDECESSOR THAT DOES NOT SURVIVE

`[[RT-DUPLICATED-RESPONSE-BLOCK]]` states, of its three-program census:

> The two `rt_escape` programs are DIFFERENT Ken programs measured separately,
> and they agree in shape without sharing a measurement — which is
> corroboration, not one number counted twice.

**Measured 2026-09-18: `:713` is `:653` plus two procs, and three of their four
shared procs are byte-identical** (`after_file_escape`, `handle_outer`, `main`;
`read_body` differs by four lines). The two programs are therefore not
independent, and the identical `317` delta may be one observation reported
twice. The **per-plan** finding — 29 collisions, 29 of 29 agreeing on
operation, a single constant offset within each plan — is unaffected, because
it is computed within one plan. What weakens is the **cross-program**
replication count: two independent programs, not three.

**THE BASIS IS AN UNTESTED TRANSFER, AND MARKING IT IS THE POINT.** The
independence measurement was taken on the site of the `ComputationalMatch`
scrutinee refusal, by a different probe. The sentence it corrects is about the
**collision census** — a different refusal. The two `rt_escape` plans do differ
as plans (907 versus 1224 nodes, 39 versus 69 match occurrences), and **nobody
has tested whether the COLLISION replicates independently across them or arises
from the procs they share byte-for-byte.**

**Carry the correction anyway.** It weakens the Steward's own claim rather than
strengthening it, which is the safe direction on this evidence — the collision
comes from two dispatcher instantiations and the shared procs are exactly where
those plausibly live. **But a later reader who notices the transfer must not
read that as licence to restore "three".** Restoring it needs a measurement of
the collision's independence, which nobody has taken.

That sentence is the Steward's and the correction belongs in the predecessor's
measured-outcome section, not here. It is recorded here because this node's
fixed inputs would otherwise inherit it.

# WITNESS: A MINIMAL REPRO, AND IT MEASURES "OCCURRENCE" DIRECTLY

**Steward, 2026-09-19, from the runtime-implementer's probes in
`thr_3s7btdee77g0n`** (`evt_6kwtw2dv0df8r`, `evt_4vvdf6mj9kx7t`), recorded here
because the finding was made under a closed campaign and the implementer
correctly declined to write it into a node they do not own.

**This node's thesis is that the route map should key on the OCCURRENCE rather
than the operation constructor. Until now the evidence for that was the four
ignored rows, each a large program needing a 29-constructor census to read.
There is a two-line repro.**

    withBuffer 1 (pure) ; withBuffer 1 (pure)          REFUSES
    withBuffer 1 (pure) ; withBuffer 2 (pure)          REFUSES  (distinct
                                                       capacities)
    withBuffer 1 (pure) ; withResource Read (pure)     REFUSES  (distinct
                                                       OPERATIONS)

    all three:  "two host response cases claim one operation constructor"

Three builds, one edit apart. Varying the operation is what identifies the
colliding constructor as the **release**, not the acquisition: every bracket
emits a `ResourceRelease`, so no choice of bracket kind or capacity avoids it.

**Controls — nesting is unaffected:**

    cr-write-writable   three nested brackets, three releases   BUILDS
    fixG, fixJ          three nested                            BUILD

So the refusal is not "more than one release". It is **two brackets in
sequence at the same level**.

## THE MEASUREMENT THAT BEARS DIRECTLY ON THE KEY

The Architect challenged the above as possibly static-only (`evt_6ppf0bzjzpssq`):
a response case is a code position, so **one** bracket site invoked twice
should mint **one** case and produce the interleaved footprint with no
collision. That was tested:

    proc pb_tw_one (_marker : Int) = withBuffer 1 pb_tw_leaf      ONE site
    pb_tw_stage = bind (pb_tw_one 0) (\first. bind (pb_tw_one 1) ...)

    "two host response cases claim one operation constructor"

**One static bracket site. Two invocations. Two response cases.**

⇒ **Cases are minted per INVOCATION, not per source site.** That is this node's
key question answered by measurement rather than by argument: the unit the
route map collides on is the occurrence, and legitimate distinct occurrences of
one constructor are exactly what it cannot currently distinguish. Fold this
into the acceptance bar's reading of "occurrence" rather than treating it as
corroboration.

## TWO CONSEQUENCES, STATED WITH THEIR FENCES

- **Factoring is not a workaround.** The severity fork the Architect posed —
  "narrow gap with a factoring workaround" versus "sequential resource use is
  not expressible in native" — settles on the **stronger** arm, because the
  proc form is the factoring and it refuses.
- **Not a new defect class, and not even a new mechanism.** Called "a new
  refusal class" when first found; one `git grep` showed four existing nodes on
  the same message. **The proc-form result was also already predicted by a
  ruling in the tree** — `RT-DUPLICATED-RESPONSE-BLOCK` §8.9a, lines 268-271,
  read from the file: *"Per-arm inlining of a shared callee is legitimate, and
  the invariant is a claim about the source dispatcher while
  `plan.source_occurrences` is post-inlining — the subject is wrong, not the
  claim."* A shared callee invoked twice is inlined per arm, so two occurrences
  is what that ruling says to expect. The Architect independently withdrew the
  static-only challenge on that citation — **and the build had already run and
  refused before the withdrawal was posted.** The measurement did not come from
  the ruling and the ruling was not re-read from the measurement, so the two
  agree independently rather than one being derived from the other.

  ⇒ **The witness is new; the refusal and the mechanism are not.** What the
  measurement adds is a two-line program where the existing ruling can be
  checked directly, instead of a 29-constructor census inside four large
  programs. Read it as confirmation with a cheap reproducer, not as a
  discovery — and do not let a second node be filed on it.

## THE SHAPE THE WITNESS DOES NOT COVER -- UNMEASURED, ATTACHED DELIBERATELY

**Architect, 2026-09-19, recorded and explicitly NOT run.** All four measured
shapes share a property easy to miss because it is the normal case: **a
statically countable number of bracket executions** — two, whether from two
call sites or two invocations of one.

    FIFTH SHAPE   a bracket inside a RECURSIVE proc, whose executions cannot
                  be statically enumerated at all.

This is attached here rather than left to be rediscovered because it bears on
the repair, not just on the witness. **A route map re-keyed on the occurrence
still has to enumerate occurrences**, and a recursive proc supplies no static
bound on how many there are. Whether occurrence-keying is sufficient, or only
sufficient for the statically-countable case, is therefore an open question
about this node's own thesis — not a further witness.

**It is unmeasured and must not be cited as though it were.** It was deferred
on purpose: it is the fifth "five-minute build" in a night whose ignore count
moved by zero, and the node it would inform is now released and being worked.
Take it up inside the repair's design, where it is load-bearing, rather than
as another probe.

# Related

- `[[RT-DUPLICATED-RESPONSE-BLOCK]]` — establishes the reading that licenses
  this repair. Must land first.
- `[[RT-HOST-RESPONSE-ROUTE-KEY-COLLISION]]` — section 3a is the assignment;
  section 9.5 is the measurement that bounds it.
- `[[RT-FRAME-MARKER-ONCE]]` — `px7n`'s next stop once this lands. Not owned
  here.
- `[[RT-COMPMATCH-TREE-SCRUTINEE]]` — this node's acceptance run produces, in
  production, the observation that node's `D0` currently reaches only under
  suppression. An upside, not a dependency in either direction.
