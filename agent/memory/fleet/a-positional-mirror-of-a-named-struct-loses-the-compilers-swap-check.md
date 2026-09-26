---
name: a-positional-mirror-of-a-named-struct-loses-the-compilers-swap-check
description: Ken-level constructors mirroring named Rust structs are positional, so two same-typed adjacent fields can be swapped silently; the two such constructors are also outside the differential's image, so no arity test, elaboration, or comparison could see it before the checker that reads them is written
metadata:
  type: feedback
---

# A positional mirror of a named struct loses the compiler's swap check

**Measured 2026-08-16 on `c0375ae47..a459e84a7`**, a Ken-level mirror of a Rust
reference module.

A review already recorded that several declared types are *"well-formed and
correctly-sized, not shown right."* ⇒ **Name the specific way they can be
wrong**, which turns a general caveat into a guard someone can build:

| shape | swap detectable? |
|---|---|
| two same-typed adjacent fields, constructed by the differential | **yes** — compared positionally |
| two same-typed adjacent fields, **outside** the differential's image | **no** |
| fields of different types | yes — will not type-check |

⇒ ***Exactly the intersection matters***: same-typed **and** never constructed.
Here that was two constructors out of nine, and both are read for the first time
by the increment that has not been written yet.

⇒ ***A positional constructor mirroring a NAMED struct silently drops a check
the source language was providing.*** Rust field names make a `gamma`/`delta`
swap a compile error; the positional mirror makes it a silent one, and **the
mirror's fidelity review is about arity and shape, which a swap preserves.**

⇒ **File it against the increment that will first read the fields**, with the
cheap guard: construct one value of each with **distinguishable** contents and
decode it back, pinning field order before anything depends on it.

## MEASURE WHETHER A GAP YOU FOUND ACTUALLY DISCRIMINATES

The case population covered two nesting orders and not the third. The Adversary
added the missing one: **it passes.** Then it mutated the arm the new case
exercises — **an existing case caught the mutation and the new one did not**,
because under the other nesting the two expressions coincide.

⇒ ***The proposed row was strictly weaker than what was already there.*** **A gap
visible in an enumeration is not a gap in discriminating power**, and the second
question is the only one that matters.

⇒ **Report the negative result anyway.** The absence is visible to anyone
auditing the list; the redundancy is not. **Saying "I checked and it is
covered" prevents the next reader from spending a pass on it** — which is worth
as much as a finding and costs one paragraph.

**Apply to your own proposed row the standard you would demand of a
reviewer's proposed fixture.** Here the row was already written and would have
been filed as a gap.
