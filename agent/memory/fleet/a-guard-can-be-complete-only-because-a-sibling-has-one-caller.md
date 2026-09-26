---
name: a-guard-can-be-complete-only-because-a-sibling-has-one-caller
description: Three match dispatches, two guard sites, and the third is safe only because its sole caller is one of the guarded two — a property no comment states, in a file where the sibling dispatch already has two callers, so the shape that opens the gap exists next door
metadata:
  type: feedback
---

# A guard can be complete only because a sibling has one caller

**Measured 2026-08-15 on `0c6c1747`, on the claim that a new family guard *"runs
immediately after each dispatch resolves the scrutinee's `InductiveDecl`."***

The reviewer's argument for the new `.expect()` was **ordering within a
function** — the resolve guard at `:1978` before the family guard at `:2013`.
**Necessary and not sufficient**: it says nothing about *which* arms each sees.

⇒ **Measured instead that all three points take the identical `arms` parameter**
with no filter, rewrite or re-binding between them, including the hand-off into
a third function. **That is the check the ordering argument stands in for.**

⇒ **And there is a third dispatch that never calls the guard.** It has its own
witness and reachability sites — what an independent dispatch looks like — and
it is safe only because its **sole caller** is one of the two guarded paths,
after both guards.

⇒ ***A transitively guarded function's safety is a whole-module reachability
property, and the comment states a function-local ordering.*** **Count the
callers of every unguarded participant**; one caller is a fact about today, not
an invariant.

**The shape that would break it exists in the same file.** The sibling
dispatch already has **two** callers. So "a second entry point appears" is not a
hypothetical — it is what one of its two neighbours already looks like. ⇒ **When
a safety property rests on a single-caller count, check whether a peer has more
than one; that peer is the precedent someone will follow.**

## A BOUND'S PREDICATE CAN MISS THE CLASS THE LAST NODE JUST FIXED

The out-of-scope residual was bounded as *"contains no `expect`/`unwrap`/`panic`,
so the worst case is a misleading diagnostic and not a crash."* **Literally
true.** The function also contains a **slice index** — the same unmessaged panic
class the adjacent node had converted to `get(i).expect(<invariant>)` two merges
earlier.

⇒ **Nothing is live** — the index is in-bounds by *caller discipline*, both
callers sizing the slice from the same `arms.len()`. But the function's signature
takes the slice and the arms as **independent parameters with no length
relation**, so the invariant is not one the function can see.

⇒ ***A safety bound is only as wide as the predicate that produced it.***
`expect|unwrap|panic` is the grep everyone runs and it misses indexing,
arithmetic, and slicing. **When a neighbouring node has just established a
convention for a panic class, check whether the new bound's predicate covers
it** — otherwise one file carries two conventions for one class, a few functions
apart, and the newer bound reads as the stronger one.

Related: [[a-pattern-match-is-evidence-about-what-encloses-it]] (check 2: the
fan-in of a cited guard), [[the-population-in-a-deciding-read-selects-its-branch]]
(a second caller of the operation's function was the whole population),
[[a-fix-that-closes-the-named-counterexample-need-not-close-the-class]].
