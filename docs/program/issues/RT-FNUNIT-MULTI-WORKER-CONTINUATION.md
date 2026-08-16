---
id: RT-FNUNIT-MULTI-WORKER-CONTINUATION
title: "RECORDED OBLIGATION: the functionized lane owes multi-worker continuation specialization -- a match case with two recursive positions (a binary-tree fold) fails because the specialization projects exactly one worker, and the code says D6a deliberately does not generalize"
status: draft
owner: runtime
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Architect D1 verdict evt_5cxzxp4b6q31v, 2026-08-16, construct 3 of RT-DESCENT-LANE-COMPLETENESS: MISSING PORT. Filed as a durable obligation record because AC-9 requires a named owner that survives RT-DESCENT-RETIRE's D6 -- prose in a deleted test's place does not discharge it. Steward-filed per COORDINATION section 2."
---

**This node is a RECORD, not a dispatch.** It exists so the obligation outlives
the tests that currently carry it. It is queued behind the operator's one-lane
priority and no ring is released on it.

## The obligation

**The functionized-units lane owes multi-worker continuation specialization.**

`backend_module` raises an **internal error**, not a typed refusal, when the
selected case has a recursive position the continuation specialization projects
no worker for — *"its induction-hypothesis prefix cannot be built."* The code
states the limit itself: **`D6a` deliberately does not generalize to a
multi-worker population.**

**The trigger is not exotic.** A match case with `recursive_positions` of length
two — **a binary-tree fold** — yields exactly this. The specialization projects
**one** worker, so any position other than the single ruled `worker_position`
fails.

## Why it is a MISSING PORT and not correct semantics

Architect `evt_5cxzxp4b6q31v`, using the discriminator that separated all four
constructs:

> **Does the refusal make a claim about the PROGRAM'S DENOTATION, or about the
> COMPILER'S OWN BOOKKEEPING?**

**No denotational claim is made anywhere here.** It names a compiler structure
that is absent. That is the whole difference from constructs 1 and 2, which say
what a value *is* and why it cannot exist across a boundary.

## THE OPEN QUESTION THAT COULD MAKE THIS BLOCKING

**`0/12` does NOT bound this construct.** That source-reachability measurement
was taken over the twelve `LexicalCallArgumentRecursor` renderings, and its
argument is about the **lexical-call-argument shape**. **This construct's
mechanism is a different shape** — a limit on `ComputationalMatch` cases with
more than one `recursive_positions` entry.

The row-3 **fixture** is unreachable. **The single-worker limit is a general
property of the continuation specialization**, and whether a two-recursive-position
match is source-reachable by some other program **was never asked.**

⇒ **[[RT-DESCENT-LANE-COMPLETENESS]] `D5` asks it.** If the answer is yes, this
obligation stops being a recorded gap and **blocks the retirement.**

Related: [[RT-FNUNIT-CHECKED-ROOT-AUTHORITY-ROUTING]] is the sibling obligation
from the same ruling.

## CURRENTLY MASKED BY AN ELABORATOR GAP. This question goes live when that closes.

**Architect `evt_7msgce14888x4`, on `D5`'s probe result
(runtime-implementer `evt_6tveatdhcz72y`).**

**This construct's source-reachability is UNDETERMINED, not zero.** The probe
that would have decided it never reached the guard: an ordinary binary-tree
traversal fails first, in the **elaborator**, at
`crates/ken-elaborator/src/compiler_driver.rs:2013-2017`.

**It was intercepted, not turned away.** A positive control declaring
`data D5Tree = D5Leaf | D5Node D5Tree Nat D5Tree` — two recursive positions —
**built and selected `FunctionizedUnits`**, so the shape is admitted. What is
missing is a traversal that reaches lowering.

⇒ **Do NOT record this construct as fixture-only-by-nature.** *"The guard was
not reached"* and *"the guard is unreachable"* are different facts, and only
the first is established.

**The load-bearing copy of this dependency is on
[[LANG-CHECKED-IH-BODY-VIEW-CAUSE]], not here** — whoever closes the elaborator
gap reads their own node and has no reason to open this one. That node carries
the activation warning; this is its mirror.

**What is already built, so the next attempt is a re-run:** the positive
control establishes admission. Only the traversal is missing.
