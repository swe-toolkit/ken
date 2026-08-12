---
id: RT-NESTED-IH-NATIVE-REALIZATION
title: "Native realization of the nested-IH recursive computation beyond scalar admission -- emitted definition, ABI/owner wiring, and execution that survives the Cranelift verifier and agrees with the interpreter at Nat 3"
status: draft
owner: runtime
size: L
gate: none
depends_on: [RT-DYNAMIC-ARM-SCALAR-MERGE]
blocks: [KERNEL-NESTED-IND]
github: null
origin: Steward-filed 2026-08-12 (COORDINATION §2) on runtime-leader's statement of c2's AC-K12 relationship, evt_77pege8j5cv14, requested at evt_6pmftb5fpxrkm. Discharges the second Steward condition on the c1/c2 cut (evt_6z7wf6dw94cym), which required c2 to state that relationship before assignment.
---

## Why this node exists

`KERNEL-NESTED-IND` is `active` on **one** open criterion, `AC-K12`. Its four
other late deliverables are merged. `AC-K12` requires a nested-inductive
recursive computation to **form a valid native artifact, pass Cranelift
verification, execute natively, and agree with the interpreter at Nat 3**, with
its carried control no longer ignored.

`RT-DYNAMIC-ARM-SCALAR-MERGE` slice `c2` was the candidate for that. It is not
sufficient, and runtime-leader stated the gap when asked (`evt_77pege8j5cv14`):

> **`c2` partially advances `AC-K12`; it does not discharge it.** It clears the
> real `D5` scalar-merge refusal by proving the arriving operand is
> `StructuralNat` while retaining the unrelated-`Data`, merge-shape, and
> catch-all boundaries. **The missing capability is native realization of the
> full nested-IH continuation/recursive computation beyond scalar admission:**
> its emitted definition, ABI/owner wiring and runtime execution must survive
> the verifier and reach the same result as the interpreter.

⇒ **`c2` clears a refusal; this node supplies an execution.** Those are
different capabilities and only the second satisfies `AC-K12`.

## Why it is a node rather than a third slice of `RT-DYNAMIC-ARM-SCALAR-MERGE`

The same reason `c1` and `c2` were cut apart in the first place: `c1` is a
fail-closed contract, `c2` is semantic admission, and **combining an unbounded
question with a bounded one is what made slice `a` too wide.** Native
realization is the unbounded one. Folding it into `c2` would reproduce exactly
the defect the `c1`/`c2` cut repaired.

The constraint is grounded, per `steward.md §4c`: `AC-K12` is a written
criterion on a live node, the gap is a **measured capability gap** stated by the
owning team, and the node is on the graph because three Kernel seats and, behind
them, `DS-9` are waiting on it. It is not a tidier-graph preference.

## The edge this node also repairs

**`KERNEL-NESTED-IND` declared no dependency on Runtime at all.** All five of
its `depends_on` entries are `merged` or `closed`, so `gen-progress.sh` showed it
**active with no blockers** while it was in fact blocked on
`RT-DYNAMIC-ARM-SCALAR-MERGE` — a relationship that existed only in prose inside
the node body.

**It failed in the direction that hides an idle team.** A reader of the tracker,
including the operator, saw a Kernel node active and unblocked and would conclude
the ring was progressing. Both edges are now declared, in `depends_on`, which is
the side `gen-progress.sh` reads.

This is a **different defect class** from the one-way `blocks:` edge fixed in PR
#1951 and is invisible to the sweep that found it: there the edge existed on one
side, here on neither. A prose-mention sweep across every live node found no
other instance — `DS-9`/`RT-SCALE-B`-style mentions are documented deliberate
non-edges, not omissions.

## What makes this `draft`, and exactly what flips it to `ready`

**Not framing debt, and not a lane request.** Runtime's lane is on the
`RecursiveDescent` retirement chain (`RT-LEXICAL-ROW2-MISSING-MINT` →
`RT-RECURSOR-TRANSPORT` → `RT-DESCENT-RETIRE`), then `c1`, then `c2`. That
ordering is the operator's standing priority and is not in question here.

**This node cannot be written shovel-ready before `c2` lands**, because `c2`
defines its input: which operand shapes arrive at native lowering already
admitted, and which refusals `c2` deliberately retained. Framing the ACs against
a guessed admission surface would produce controls that measure the wrong
boundary.

> **Flip condition, stated so `draft` is checkable rather than a holding
> pattern:** the Steward frames this node and flips it `ready` **when `c2`
> merges**, using `c2`'s landed admission surface as the frame's fixed input.
> There is no other gate on it — not the retirement chain, not a decision, not
> the operator.

## What the frame must carry when written

Recorded now while the reasoning is fresh, so the framing turn does not
re-derive it:

- **Four stages, and they are separate observations.** Emitted definition,
  verifier acceptance, native execution, interpreter agreement at Nat 3. A
  control that collapses any two of them cannot say which failed.
- **Interpreter agreement is a differential, not a self-check.** The oracle is
  `ken-interp`'s result for the same computation; "the native run produced 3" is
  not the criterion.
- **The carried control must no longer be ignored** — that is part of `AC-K12`'s
  own wording and is the thing most likely to be satisfied vacuously.
- **`AC-K12` is not claimed or advanced by `c1` or `c2`.** Whatever this node
  discharges, the criterion belongs to `KERNEL-NESTED-IND` and closing it is
  Kernel's, on Runtime's delivered capability.
