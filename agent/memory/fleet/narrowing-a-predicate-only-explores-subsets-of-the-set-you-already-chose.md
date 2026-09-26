---
name: narrowing-a-predicate-only-explores-subsets-of-the-set-you-already-chose
description: A count guard's mutation arm narrows the predicate to prove it discriminates, which cannot find a spelling nobody enumerated — and eighteen test-gated regions under `cfg(any(test, ...))` sit outside the counted set while the caveat's sentence covers them
metadata:
  type: feedback
---

# Narrowing a predicate only explores subsets of the set you already chose

**Measured 2026-08-17 on a guard created to fix exactly this class.** An
honest-limit caveat had claimed 22 inline test regions; the real figure was 322,
and the fix added a live count guard plus a mutation arm that **narrows** the
predicate to prove it discriminates.

⇒ **The narrowing arm is the right instrument and it worked** — it caught a
column-0-only predicate seeing 105 of 322. ⇒ ***But narrowing only explores
subsets of the set already chosen.*** **A spelling nobody enumerated is
unreachable from it**, and that is where the remainder lived:

```
#[cfg(test)]                              322   counted
#[cfg(any(test, feature = "..."))]         18   NOT counted, and the caveat covers them
#[cfg(not(test))]                          21   correctly excluded, production-only
```

⇒ ***Each refinement fixed the previous narrowing and introduced a smaller
one*** — 105, then 322, against a sentence describing 340. **The guard is blind
to the remainder by construction: adding one leaves the count unchanged and
nothing reds**, which is the failure the node existed to stop.

⇒ **Widening is the untested direction. Say so explicitly**, because a passing
narrowing mutation reads as "the predicate is pinned".

## CHECK YOUR OWN BROADER PREDICATE BEFORE PROPOSING IT

A raw regex found three more occurrences than the guard counted. ⇒ **All three
were COMMENT MENTIONS of the attribute** — so the solo-line requirement is a
**virtue** the obvious broader regex would have destroyed. **Refute your own
first hypothesis in the report**; here it converted a suspected defect into a
reason the existing predicate is well chosen.

**And name the trap in the fix you recommend.** `contains("test")` sweeps in
21 `not(test)` attributes that are production-only. **The predicate must admit
`test` and `any(test, …)` while excluding `not(test)`** — three spellings, not a
substring. **A recommendation that would break on the first attempt costs the
finding its credibility.**

Related: [[close-a-class-partition-the-declared-population]] (a column-0 pattern
missed indented members; close the enumeration against its hiding places),
[[count-a-maps-images-not-its-arms-before-claiming-per-arm-coverage]] (a
mutation set chosen by what reds is complete only on the detectable half).
