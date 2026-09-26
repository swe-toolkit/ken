---
name: read-the-deletions-first-on-a-node-that-removes-vacuous-assertions
description: A debt node removing vacuous equalities is where a live assertion leaves alongside them — here the excision was surgical, keeping the population constraint that shared a binding with the vacuous equality, and the residual was that its replacement assertion is a third strength class the doc lists as one of two
metadata:
  type: feedback
---

# Read the deletions first on a node that removes vacuous assertions

**Measured 2026-08-14 on `b6b86916`, a debt node closing seven carries, two of
them mine.**

A node whose stated job is *"remove assertions that cannot fail"* is precisely
where a **non-vacuous** assertion leaves in the same hunk. With `-24` in a test
file, the deletions are the first thing to read.

⇒ **Here they were surgical, and the distinction is worth naming because the
debt item did not ask for it.** The removed equality and a retained destructure
shared one binding:

```rust
let [_advanced] = advanced.as_slice() else {
    panic!("child-push carry is not one exact identity");     // KEPT
};
// assert_eq!(depth_N.advanced, depth_N.unit_consumer)        // REMOVED
```

**The equality compared a copy with its source; the destructure constrains the
population.** Same value, same binding, different claims — and only one was
vacuous.

⇒ ***When a repair removes a vacuous assertion, check what else lived on that
binding.*** A `let [x] = set.as_slice()` beside a tautological `assert_eq!`
reads as scaffolding for the assertion and is an independent claim about
cardinality.

## COUNT THE STRENGTH CLASSES, NOT THE ASSERTIONS

The node also added a replacement assertion at the boundary the debt item said
was un-asserted. Checking whether it repeated the vacuity that just left gave
**three** classes where the doc names two:

| comparison | provenance | class |
|---|---|---|
| depth-2/3 vs sibling | **different compiles** | independent records |
| **depth-1 vs its own unit** | two resolvers over **one seed** | same-compile, shared seed |
| the deleted pair | literally the unit's key | copy vs source |

⇒ ***A middle class exists between "independent" and "tautological", and it is
the one that gets grouped with whichever neighbour the author was thinking
about.*** Here the doc listed the new assertion beside the cross-compile pair
under a property true of all three (*"no fixture literals"*), which is not the
distinction that matters.

⇒ **The precise statement for a shared-seed comparison: it reds on a
disagreement between the two consumers and cannot red on a defect in what they
share.** That is worth one clause wherever such an equality is listed as a
control. (The shared-seed provenance was first asserted from reading one site;
it was measured at both on `afdabc502` and holds.)

## "AN UNEXERCISED BRANCH" CAN BE A CASE SPLIT, NOT A COVERAGE GAP

A carried item said a primary `.map(Source)` branch is unexercised and sits one
level off from its fallback. **Both arms construct the same variant from the
same shape**, so the branch cannot produce a different *kind* of value. The
first restatement of the item was the smaller question *"can the two inputs
ever disagree?"*, filed as an unrun read.

**Measured one node later (`afdabc502`), both framings were wrong.** The
branches are **not alternatives for one value**: the primary fires only when a
parent exists, the fallback only when one does not. Both are exercised, by
different depths, and they can never disagree because they never co-occur.
The seed walk starts every root as `(origin, None, None)`, so at the outermost
match the `or_else` fires; deeper levels take the parent's.

⇒ ***"A branch is unexercised" assumes the branches compete for one input.***
When a `map(..).or_else(..)` is really a **presence test on an inherited
value**, the two arms belong to different cases of the traversal, and the
"offset" that looks like a smell is the mechanism itself.

⇒ **Before filing an unexercised-branch item, ask what selects the branch.** If
the selector is depth, position, or presence of an inherited value, the arms are
a case split and the coverage question is *"is each case reached?"*, not *"do
they agree?"* When the arms are variant-identical and do compete, the smaller
question (can the inputs disagree?) is the right restatement, and it still needs
its own evidence: **name that read as unrun if you do not take it.**
