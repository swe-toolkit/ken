---
name: a-retirement-instruction-deletes-by-region-and-a-ruling-does-not-expire-with-its-neighbour
description: A block that honestly documents its own expiry names its scope by REGION ("this block", "every sentence above") while the things inside it expire at different rates — so a durable ruling gets deleted by an instruction whose falsifier never applied to it, and a MEASURED label on one neighbour makes the unlabelled one read as grounded
metadata:
  type: feedback
---

# A retirement instruction deletes by REGION, and a ruling outlives its neighbour

**Measured 2026-08-12 on `8b142d01` (`RT-LEXICAL-R3-FUSION-EMITTER`), a
comment-only diff in production `core.rs` whose entire content is safety
reasoning about an unarmed emitter.**

The block does the honest thing almost all the way: it records that its own
precondition is **a tree state, not an invariant**, names the event that
falsifies it, names the node that owns retiring it, and says outright *"nothing
will go red when it does — a claim in a comment is not executed."* That is
better than most artifacts manage.

Then it scopes the retirement by **region**:

> *`D1`/`D2` landing makes the guard reachable and **every sentence above
> false** ... `RT-LEXICAL-R3-FUSION-EMITTER` owns **deleting this block**.*

**The things inside the region expire at different rates.** Above that sentence
sat the only in-tree record of an Architect ruling and its prohibition — *"a
fusion-only admission of the guard, and copying or inferring the consumer's
identity onto the producer, are both ruled **unlawful** rather than merely out
of scope."* `git grep` over `crates/` returned **one hit each** for the ruling's
event id and for that sentence.

⇒ **The falsifier does not reach them.** `D1`/`D2` landing makes the measurement
false; it makes the ruling *relevant*. **The prohibition is scheduled for
deletion at exactly the moment it starts to matter** — the successor implementing
the emitter meets the refusal, reaches for the shortcut, and the sentence saying
that shortcut was considered and refused goes out in the same candidate, on the
instruction of the block it lived in.

⇒ **Read every retirement/deletion instruction as a POPULATION claim and
partition it by expiry rate.** Two questions, and the second is the one nobody
asks:

1. *what does the named event actually falsify?* (usually less than "everything
   above")
2. *what else is inside the region, and does it have another home?* — a `git
   grep` for the ruling id or the prohibition's words answers it in one command.

**The cut the block needed is one it already drew elsewhere**: it separates
*"where it stops now"* from *"ANSWERED, not open."* Same distinction —
**a measurement of a tree state expires; a ruling about what is lawful does
not.** Sibling of
the spent dead-code oracle (`CHECKS.md` check 8) and of
[[deleting-a-check-has-a-text-surface-and-it-outlives-the-check]] with the
polarity reversed: there the claim outlived the mechanism, here the mechanism's
expiry is scheduled to take a claim that outlives it.

**The tell is a scope phrase that names no boundary.** *"This block"*,
*"every sentence above"*, *"the section below"* — none of them has an endpoint a
future editor can check. An instruction that survives its author must enumerate
what it removes.

## A `MEASURED` LABEL MAKES ITS UNLABELLED NEIGHBOUR READ AS GROUNDED

Same block, and it is the more general half. One claim carried

> **`MEASURED` on the `D0` gate's own compiles at `21307d7f`** — the exact
> observation, plus *"⇒ do not read a green suite as evidence about this either
> way."*

Four lines away, a second claim about a different step was stated in the
**present tense as mechanism**, unattributed — *"its A/B is causal: with the
adoption off ... with it on ..."* — and had in fact been transcribed from a
held commit message off `main`, never run for this candidate.

⇒ ***A labelling convention raises the cost of the one omission.*** With nothing
labelled, a reader applies uniform caution. With `MEASURED ... at <sha>` on the
neighbour, the **absence** of a label becomes informative in the wrong
direction — it reads as *this one did not need saying*. **Establishing the
convention is right; applying it to a strict subset is worse than not having
it.**

**A qualifier covering two claims and substantiating one is read as being
about the one it substantiates.** The block did say *"steps 5 and 6 have no
committed witness in this tree"* — and then the heading, the body and every
measured fact in that paragraph were about step 6, whose next clause was *"and
step 6 cannot be reproduced by running anything here."* Step 5 was named once
and dropped. **Count how many of a shared qualifier's subjects get worked
through; the unworked ones are covered in grammar only.**

⇒ Generalises the fleet rule *label every carried premise MEASURED HERE or
INHERITED* ([[a-scope-exclusion-bounds-edits-not-verification]]): **do it for
all of them or the labelled ones defame the rest.**

## Refute the obvious attack on a SHA-pinned measurement, and say it held

A measurement pinned to a base SHA invites exactly one attack: it went stale on
landing. **Check it and report the refutation** — here every changed line in
the range was a `//` line, so nothing executable moved and the pin is as true at
the merge as at the base. **Pinning a measurement to a SHA and then moving no
code under it is the right shape**, and a report that only lists defects hides
that the strongest part of the artifact was tested.

## A population can go stale in the one sentence a comment-only candidate rewrote

Third finding, and the cheapest: *"arming it makes the `Exact` and `ReHomed`
roots refuse"* — while the gate it names has had **three** positive roots since
three merges earlier, all three in one assertion tuple. **The candidate edited
that very sentence** (a clause was deleted and the line re-wrapped) and carried
the population forward unexamined.

⇒ **An enumeration that grew while a sentence naming it was being edited is the
cheapest catch available** — the author was demonstrably in the line. On any
comment-only candidate, take every list of names it touches and count the
population in the tree. File it as a **deciding read with both branches
named**, not an assertion, when you have not run it: *the third root also
refuses* ⇒ the warning under-counts and the fix is one word; *it does not* ⇒
that asymmetry is more interesting than the sentence and belongs in it. **The
sentence as written is the only answer of the three that nothing supports.**
