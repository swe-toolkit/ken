---
name: the-one-site-that-reuses-a-binding-instead-of-deriving-is-where-both-defects-live
description: Six sites gained a new callee-identity argument; five derived it and the sixth reused a local, and that local is the one an existing mutation deliberately substitutes — so the odd-one-out carried both a mutation contamination and a level mismatch, and the frame's own two-axes sentence became false of its own seam
metadata:
  type: feedback
---

# The one site that reuses a binding instead of deriving is where both defects live

**Measured 2026-08-15 on `9d8326c8c...9ba3950af`.** A new `callee:
StaticOriginId` argument was threaded to six call sites. Five compute
`child_static_origin(X, 0)`. The sixth passes a local named `origin`.

⇒ ***Scan a fan-out of N new arguments for the one that is shaped differently
from the other N-1, and read that one first.*** Here the odd site carried
**both** defects, and neither was visible at any of the five.

## THE REUSED BINDING WAS THE MUTATED ONE

```rust
#[cfg(test)]
let origin = self.call_input_transfer_origin_under_mutation(origin)?;  // shadows
… carry_call_input(builder, origin, input, …, /* callee */ origin)
```

and the mutation substitutes rather than flags:

```rust
let root = self.static_transition_plan.root_static_origin()?;
Ok(root)
```

⇒ **Armed, the recorded callee is the program root.** The mutation moves the
transfer coordinate *and* the callee identity together.

***The frame's own comment three lines up now states the opposite***: *"Same
call, same arguments, same moment, **two axes**."* The new field is a **third
consumer of that one variable**, so the sentence a reader uses to decide the
mutation is attributable is false of its own seam.

⇒ ***When a new field is threaded through a function, check every `#[cfg(test)]`
mutation already living in it.*** A mutation's blast radius is every consumer of
the variable it substitutes, and that set grows silently each time someone adds
an argument. **The mutation was not changed and still became less
attributable.**

## SEVERITY IS LATENT, AND SAY SO

The variant is expected in **zero** controls; only one of the six is ever
asserted. So nothing reads the corrupted value today. ⇒ **File it as a trap for
the next reader rather than a live wrong answer** — and name the successor most
likely to spring it, which is what makes a latent finding actionable instead of
a smell.

## THE SECOND SYMPTOM IS THE SAME ROOT, NOT A SECOND FINDING

Five sites record the callee's **body** (`child_static_origin(.., 0)`); the
sixth records its **scheduling entry**. Pre-mutation that is a real identity of
the real callee — **defensible, and one level up**, so a control comparing the
field across callers reads a disagreement where there is none.

⇒ **Group symptoms by cause before filing.** Two findings that share a root
invite two independently-designed repairs; one finding with two symptoms gets
one. ⇒ And **state the repair as a direction with the open question named** —
whether a child ordinal exists at that plan node was not the finder's to decide,
so the fix proposed was *"take it before the mutation"*, which holds either way.
