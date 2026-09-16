---
id: RT-UNAVAILABLE-ROSTER-RELATION-TEST
title: "A durable test that the effect_v1.rs ten-op refusal arm enumerates exactly the RepresentedUnavailable set of availability(). Closes the coherent-but-partial hazard: one availability site moves and the others do not. It does NOT close the unauthorized-but-coherent hazard, which is a candidate gate's job -- the two are different properties and neither instrument substitutes for the other."
status: draft
owner: runtime
size: S
gate: none
depends_on: [RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE]
blocks: []
github: null
tier: T2
origin: "Steward cut 2026-09-16 at the Architect's direction (evt_7jcex53r71gn6): 'The relation test is still worth having -- file it separately.' It was first proposed by runtime-implementer as the durable phrasing of AC-UNAVAILABLE-ROSTER-INTACT, then WITHDRAWN by its author after the Architect measured it against the known-bad tree and it passed there. The withdrawal was correct for that use and does not touch this one: the phrasing fails at detecting an unauthorized coordinated flip, and succeeds at detecting a partial one. Same sentence, two jobs, one of which it does well."
---

> # DRAFT. Not framed, not released. Do not start.
>
> Cut so a real hazard has a carrier, not because it is scheduled. It is small,
> mechanical, and T2 — but read the section below before sizing it, because the
> thing that makes it worth writing is also the thing that makes it easy to
> write in a form that does nothing.

# Objective

A durable test asserting that `effect_v1.rs`'s enumerating refusal arm lists
**exactly** the `RepresentedUnavailable` set of `availability()`, derived from
the classifier rather than from a count.

# The hazard it closes: coherent-but-PARTIAL

An availability change spans several coordinated sites — at the last census,
five. **A partial edit moves one and leaves the others**, producing a tree
where the wire layer refuses an operation the classifier calls available, or
the reverse. That is a genuine structural defect, it has no compiler
enforcement, and nothing on `main` watches for it today.

**The test needs no editing by a legitimate flip**, because a legitimate flip
moves both sides. That is what makes it durable rather than a sentinel.

# The hazard it does NOT close, and the measurement that proves it

**Unauthorized-but-coherent.** This phrasing was measured against
`preserve/ABI-S6-HS18-verifier-checkpoint-not-a-candidate`
(`5d977ac7968dff3763d330690a9b4df530925d79`), which carries a refused
`MappingAcquireFile` promotion:

    tree        availability()                      refusal arm   relation
    main        10 RepresentedUnavailable / 25 NT   10 members    10 == 10  HOLDS
    5d977ac79    9 RepresentedUnavailable / 26 NT    9 members     9 ==  9  HOLDS

**It passes on the known-bad tree.** The flip edits both sides, so the relation
stays true across it.

⇒ **A consistency invariant cannot detect a coordinated unauthorized change.**
Coherence and warrant are different properties, and a relation test measures
only the first. Authorization is a property of a **transition**, not of a
state, so only a diff against a named base can express it — which is inherently
one-shot and therefore a candidate gate, not a test.

    hazard                      instrument                       lifetime
    unauthorized-but-coherent   diff predicate vs a named base   candidate gate, retires
    coherent-but-partial        arm == RepresentedUnavailable    durable test, permanent

**Do not let this node absorb the other job.** The next reader will see one
sentence that looks like it covers both and delete the gate as redundant. It
is not redundant; it is the only one of the two that works on the case that
actually happened.

# Deliverables (indicative — this node is not framed)

- A test deriving the expected roster from `availability()`'s
  `RepresentedUnavailable` classification and asserting the refusal arm's
  membership equals it, **by name, both directions**.
- **Derived, never counted.** A count goes stale the moment an eleventh
  unavailable op is added; the derivation does not. This is the inverse of the
  candidate gate's phrasing, and the inversion is deliberate — derive when you
  want invariance under legitimate growth, pin when the change you fear **is**
  the growth.
- A control that the test FAILS on a synthesized partial edit (one side moved,
  the other not). Without it the test holds for an implementation that reads
  one roster twice.

# Related

- [[RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE]] — owns the arm; its enumeration is
  the `error[E0004]` build-break mechanism, which is why the arm enumerates and
  the test over it derives.
- [[RT-D5B-MAPPING-AVAILABILITY-FLIP]] — the legitimate flip this test must
  survive without editing.
- [[ABI-S6-HS18-MAIN-BASED-CLOSURE]] — carries the candidate gate for the other
  hazard, at frame §4.
