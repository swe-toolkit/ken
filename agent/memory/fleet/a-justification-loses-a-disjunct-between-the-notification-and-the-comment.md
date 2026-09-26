---
name: a-justification-loses-a-disjunct-between-the-notification-and-the-comment
description: The notification said the cascade's arms were "a bare literal or a two-way eq_int match"; the landed comment said "a bare literal", and reading the generator showed every False arm is a nested match — the load-bearing sentence for a stack saving was false in the one place a future reader will find it
metadata:
  type: feedback
---

# A justification loses a disjunct between the notification and the comment

**Measured 2026-08-14 on `51b3a75c`, a stack-footprint reduction whose entire
saving rests on one arm being cold.**

The Steward's notification: *"every arm is **a bare literal or a two-way
`eq_int` match** the unrefined attempt always resolves."*

The landed doc comment: *"every one of that cascade's arms is **a bare
literal** that the cheap unrefined attempt always resolves."*

**Reading the generator settles it in one look.** The cascade is emitted by a
recursive `format!`:

```rust
"match (eq_int k {k}) {{ True |-> {lit} ; False |-> {rest} }}"
```

⇒ **Every `True` arm is a bare literal; every `False` arm is the next nested
`match`, and the innermost is an application.** Half the arms, and none of the
recursive ones, are literals. **The notification was right and the comment
dropped the disjunct that covers 31 of the 62 arms.**

⇒ ***When a prose justification exists in two places, diff them.*** The looser
one is usually the durable artifact — a message is written once and read once, a
comment is read by everyone who touches the function afterwards. **The version
that survives is the one that should be checked hardest, and it is the one
written last, in a hurry, by someone summarising.**

## THE FALSE REASON MISLEADS IN BOTH DIRECTIONS

The conclusion (*this arm is never entered here*) may well hold. **The stated
reason does not**, and a reason is not merely decoration on a stack
optimisation:

- a future author reading *"safe because the arms are bare literals"* would
  conclude a cascade with non-literal arms is where this **inverts** —
- while the current cascade **already has 31 non-literal arms** and is fine.

⇒ **A justification that is false in the safe direction still costs**, because
the next person applies it as a rule. Check whether a wrong reason would
*exclude* cases the mechanism actually covers, not only whether it *admits*
cases it does not.

## READ THE GENERATOR, NOT THE GENERATED SHAPE

The claim was about *arms of a cascade*. There is no cascade to read — there is a
**thirty-line function that emits one**, and its `format!` string answers the
question exactly, with no sampling and no counting.

⇒ ***When a claim is about a repetitive structure, find whether it is generated;
if it is, the generator is the complete population in one screen.*** Same rule
that answered the previous node's substitution challenge: **a generated
population's properties are the generator's properties**, and reading it beats
any number of instances.

## AN OPTIMISATION MEASURED ON ONE PROGRAM AND CLAIMED FOR A POPULATION

The saving is real for the program that motivated it. **The inversion is silent**
— a program that does take the extracted arm pays a new frame at every level, and
the candidate's own mirror experiment measured that frame class at `+6472`,
enough to exceed the `~96 KiB` of margin the change bought over 31 levels.

⇒ **State the inversion's size using the candidate's own numbers, and label the
assumption.** *"If the fallback's frame is comparable to the 6472 the mirror
extraction measured"* is a derivation with one `objdump` between it and a
measurement — **naming that read is what turns a worry into a task.**

**And name the failure mode's venue**: a stack inversion surfaces as a SIGSEGV
on the guard page, not a red test. **An optimisation whose regression mode is an
abort has no detector by default**, which is the sentence that belongs beside the
coldness claim rather than a reason to decline the change.
