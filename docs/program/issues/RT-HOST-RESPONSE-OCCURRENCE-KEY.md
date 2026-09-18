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
origin: "Steward, 2026-09-18, fourth repair node from the RT-IGNORED-FAILING-ROWS-INVENTORY ledger on operator directive 2026-09-15 'The other tests should be fixed.' The occurrence-keying insight is the Steward's own section 3a amendment to RT-HOST-RESPONSE-ROUTE-KEY-COLLISION, written 2026-09-17: 'What the key actually omits is the OCCURRENCE.' It was routed to that node's AC-4 and AC-4 closed 'not reached', so the assignment has sat undischarged since. RT-DUPLICATED-RESPONSE-BLOCK then established the reading that licenses it -- the two copies are legitimate instantiations, not a duplicate awaiting resolution -- which makes the construction-time uniqueness assertion the wrong instrument rather than a faithful report of a planner defect. Architect ruled the repair LIVE 2026-09-18 and set its acceptance bar. Steward-filed per COORDINATION section 2."
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

**That invariant is too strong, and saying so is now licensed evidence rather
than a proposal.** `[[RT-DUPLICATED-RESPONSE-BLOCK]]` measured that the two
colliding copies track two semantically different match arms — legitimate
instantiations, not one block awaiting de-duplication. A construction-time
uniqueness assertion over `case.constructor` is therefore not a property the
plan was ever required to have.

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
`RT-DUPLICATED-RESPONSE-BLOCK`'s `D2`/`D3` land on `main`, re-read this node's
fixed inputs against that `main`, correct any coordinate that moved, and flip to
`ready`. **Flipping on the node's status alone is not sufficient** -- that is the
condition that already holds and is why this says `draft`.

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

Both halves are required and the second is the one a tuple-widening satisfies
by accident:

    MUST STILL REFUSE   two response-handling sites within ONE occurrence
                        claiming one operation constructor
    MUST NOT REFUSE     N legitimate instantiations each contributing one
                        response handler

**A repair that cannot still refuse the first has relaxed the invariant, not
re-keyed it.**

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
