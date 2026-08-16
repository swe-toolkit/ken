---
id: RT-FNUNIT-CHECKED-ROOT-AUTHORITY-ROUTING
title: "RECORDED OBLIGATION: the functionized lane owes routing the affine checked-root authority to whichever generated unit emits the terminal answer -- descent mints it unconditionally, functionized mints it only in the root unit, so a terminal answer emitted while defining a non-root unit finds None"
status: draft
owner: runtime
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Architect D1 verdict evt_5cxzxp4b6q31v, 2026-08-16, construct 4 of RT-DESCENT-LANE-COMPLETENESS: MISSING PORT. Filed as a durable obligation record because AC-9 requires a named owner that survives RT-DESCENT-RETIRE's D6 -- prose in a deleted test's place does not discharge it. Steward-filed per COORDINATION section 2."
---

**This node is a RECORD, not a dispatch.** It exists so the obligation outlives
the test that currently carries it. It is queued behind the operator's one-lane
priority and no ring is released on it.

## The obligation

**The functionized-units lane owes routing the affine checked-root authority to
whichever generated unit emits the terminal answer.**

The authority is a **compiler-internal affine token**. The retiring lane mints
it **unconditionally**. The functionized lane mints it **only in the root
unit**, under an `is_root` guard. So when the terminal answer is emitted while
defining a **non-root** unit, the mint finds `None` and the planner invariant
fires: *"terminal answer has no affine checked-root authority."*

## Why it is a MISSING PORT and not correct semantics

Architect `evt_5cxzxp4b6q31v`, on the same discriminator that separated all four
constructs — **denotation versus the compiler's own bookkeeping.**

**This is an unrouted token, not a statement about the program.** Nothing here
claims a value cannot exist; it claims a token was not carried to where it was
needed. That places it with construct 3 and against constructs 1 and 2.

## Bound

**Source-reachability of the one measured program is zero** — it is one of the
nine, all fixture-only. **Unlike [[RT-FNUNIT-MULTI-WORKER-CONTINUATION]], no
open reachability question is recorded against this construct.** If one is
found, it belongs here.

Related: [[RT-FNUNIT-MULTI-WORKER-CONTINUATION]] is the sibling obligation from
the same ruling, and it is the one carrying an open reachability question.
