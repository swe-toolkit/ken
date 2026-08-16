---
id: RT-OVERCONSTRUCTED-OUTER-RECOGNITION
title: "Stop minting static-worker recognitions for nested constructor fields whose transports no emitted call ever names, and establish the non-traversal as a property of the emitter rather than of the measured rows"
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-SECOND-RECOGNITION-ERASURE]
blocks: []
github: null
origin: "Steward, 2026-08-16, on RT-SECOND-RECOGNITION-ERASURE D1d selecting (A) over-construction at evt_2c9cqdpyh28p1, measured at 790c16ea6. Carries the two qualifications the Architect recorded at evt_3sfw746tk6td2 and the corrected AC-3b from RT-MINT-SITE-STATIC-DISCRIMINATOR. Every coordinate re-verified by symbol against origin/main fcfd0c784 before filing. Steward-filed per COORDINATION section 2."
---

## What `D1d` settled, and what it did NOT

**`D1d` selected `(A) over-construction`** with both columns and a
discriminating positive control (`evt_64878jgfndq66`, `evt_2c9cqdpyh28p1`).

| column | result | what it excludes |
|---|---|---|
| execution | depth 2 and depth 3 each strictly evaluate the governed lexical-call argument and exit `0` | **(B) under-consumption** — the compile is correct |
| emission | sole emitted static-worker call at origin 15 names `T1` (depth 2) / `T2` (depth 3); **no call instruction names `T0`**, nor `T1` at depth 3 | **(C) under-recorded consumption** — the read reaches the newest transport directly |

**Positive control:** a field-sensitive depth-2 probe preserved constructor
shape, changed the seed worker's returned constructor from `Leaf` to `Wrong`,
and moved process status `0` to `1`. **The oracle detects a wrong field value**,
so the correct-execution column is a measurement and not an absence.

> ### THIS IS NOT A MISCOMPILE, AND THE FRAME MUST NOT BE READ AS ONE
>
> **(B) was the branch that would have outranked this node, and it is
> excluded.** The generated code is correct today. What is wrong is that the
> mint creates obligations for outer recognitions that no emitted call will ever
> discharge, so `close` refuses — **correctly** — on a compile that is otherwise
> fine. **The leak is bookkeeping. Size the repair accordingly and do not
> import urgency this measurement does not support.**

## The two routes, and this node does not pick between them

**`RT-MINT-SITE-STATIC-DISCRIMINATOR` `D0` returned `NO`:** the static plan
exports no total mint-to-reader relation, so no principled mint-time predicate
can be specified, let alone discharged.

**(R1) mint-time.** `D0`'s `NO` **sizes** this route rather than closing it:
the successor is **extend what the static plan exports, and then specify the
predicate** — not *"specify the predicate `D0` could not."* Two builds, and the
first is the one nobody has scoped.

**(R2) rebind-site.** **The rebind site is a second vantage that already
observes the supersession**, so a repair keyed there needs **no total mint-time
relation at all** — precisely the thing `D0` ruled cannot be had. Its soundness
condition is that the outer transports are **not traversed**, which is exactly
what `D1d`'s emission column measured.

> ### (R2) IS NOT `transfer`. `D1c` DOES NOT REACH IT.
>
> `transfer` claimed **succession** — T0's obligation carried forward and
> discharged by T1's consumption. **`D1c` refuted that**, and it is dead.
>
> **(R2) claims something different: the obligation is VOID because the field is
> never read.** Different claim, different soundness condition. **Rejecting
> (R2) by pattern-match against the refuted lean would kill a live option on a
> ground that does not reach it.** Recorded by the Architect at
> `evt_3sfw746tk6td2` for exactly this moment.

## `D0` — THE DECIDING READ. Is the non-traversal a property of the EMITTER?

**This is the whole node, and it is the campaign's signature trap arriving one
level up.**

`D1d` measured non-traversal on **two rows** — row4 at depth 2 and depth 3.
**`RT-MINT-SITE-STATIC-DISCRIMINATOR` died on exactly this distinction:** a
runtime observation over occurrences is **the wrong KIND of fact for a static
site**, and measuring every row in the tree today would produce a claim about
one SHA rather than a property of the mechanism.

> **(R2) is sound only if "no emitted call names an outer transport" is a
> property of the CALL EMITTER — something the emitter's own structure
> guarantees for every occurrence it will ever handle.** If it is instead a
> coincidence of the measured rows, **(R2) collapses into the wall `D0` already
> hit**, and this node has learned that at the cost of one read rather than one
> repair.

**Report the answer as one of three, and do not blend them:**

1. **EMITTER PROPERTY** — the emitter structurally names only the transport on
   the binding it is lowering, so no occurrence can name an outer one. **(R2) is
   live; go to `D1`.**
2. **NOT ESTABLISHED** — the emitter permits naming an outer transport and the
   measured rows merely never do. **(R2) is dead; report and stop.** Do not
   fall back to (R1) inside this node.
3. **CONDITIONAL** — it holds under a stated restriction the site can test.
   **State the restriction as a predicate, and say whether the site can
   evaluate it.** A restriction the site cannot name is not a restriction.

**Read the emitter's construction of the call instruction**, the same site
`D1d` instrumented — by symbol, not by line, since this file moves under every
neighbouring merge.

## Deliverables

**`D0` — the emitter read above.** Answer 1, 2, or 3 with the warrant.
**Deliver `D0` on its own and hand it back before building anything**, whatever
it returns. `D1`/`D2` are released by the Steward on the answer.

**`D1` — if and only if `D0` returns EMITTER PROPERTY: stop minting the
obligation for outer recognitions.** The repair is at whichever site the `D0`
warrant supports. **The obligation must not be created**, rather than created
and then forgiven — a second writer of `consumed` is barred by `AC-1`.

**`D2` — the control.** A mutation demonstrating the repair discriminates in
**both** directions: one that would erase a recognition whose transport IS
named by an emitted call (must red), and one that leaves an outer recognition
minted (must red). **A repair whose control only shows the leak gone has not
shown it stopped at the right place.**

## Acceptance criteria

**`AC-1`.** **The ledger's law is unchanged.** No relaxation of `close`, no
second writer of `consumed`, no widening of the agreeing bijection. **A repair
that makes the refusal go away by changing what `close` demands is the defect
this campaign exists to prevent.**

**`AC-2`.** **No erasure of a recognition whose TRANSPORT is traversed or
rebound by a later read**, even when that recognition's own field is never read
directly. **This is `RT-MINT-SITE-STATIC-DISCRIMINATOR`'s corrected `AC-3b`,
carried forward** — "the field is never read" and "the transport is never
consumed" are **independent** properties, and `D1b` is what separated them.

**`AC-3`.** **`D0`'s answer is a property of the mechanism or it is reported as
not established.** A predicate discovered by looking at what the measured rows
happen to share **inherits the exact defect that closed the mint-site node.**
**Answer 2 is a success for this node**, not a failure to be worked around.

**`AC-4`.** **`D2`'s control is discriminating, demonstrated by mutation, not
argued.** A count of zero leaks is worth exactly what the demonstration that
the instrument would have seen one is worth.

**`AC-5`.** **Row4-depth-1 and row5-after-hole are behaviourally unchanged**,
and the `D2k` controls still pass.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **`transfer` in any form.** Refuted by `D1c`; it is not a fallback if `D0`
  returns answer 2.
- **Weakening the refusal message to make the leak acceptable.** The message's
  own repair is `RT-SECOND-RECOGNITION-ERASURE` `D2` and is text only.
- **Changing a producer so the ledger balances.** A condition that dissolves
  because you changed what feeds it has not been understood.
- **Erasing on the authority of a runtime observation over occurrences.** That
  is the closed node's finding, and it binds here.
- **Extending the static plan's exports** — that is (R1), and (R1) is not
  released by this node.

## Sequencing

**Lane 1 (operator priority).** `D0` is releasable now: `D1a`-`D1d` are
delivered and this node needs nothing further from them.

> **Do NOT read the `depends_on` edge as "wait for
> `RT-SECOND-RECOGNITION-ERASURE` to merge."** Its `D1a`-`D1d` are
> measurement-only and produce no candidate; only its `D2` (refusal message
> text) will ever land. **This node's `D0` does not depend on that text**, and
> gating on it would be waiting on a landing unrelated to the question.

**`RT-SECOND-RECOGNITION-ERASURE` `D2` may run in parallel** — different site,
text only, and `AC-1` bars it from touching the law.
