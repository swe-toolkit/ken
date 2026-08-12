---
name: a-validator-whose-expected-value-is-its-own-builder-re-run
description: >-
  A gate that computes its expected value by re-running the builder that
  produced the value it checks cannot see a defect in that derivation — it is
  self-oracled, and the word "independently" in the comment beside it is the
  tell.
metadata:
  type: feedback
scope: roles/adversary
---

# A validator whose expected value is its own builder re-run

Three planner gates landed in the shape

```rust
plan.case_emissions = build_case_emission_plan(&plan)?;
validate_case_emission_plan(&plan, &plan.case_emissions)?;   // checks
                                    // records != build_case_emission_plan(plan)
```

so the comparison is `build_X(plan)` against `build_X(plan)`. The builder is
the answer key for its own output.

**Establish it by closure, not by reading one site.** Two facts make it a
tautology and both are greppable: the population has **exactly one writer**
(so the stored value is always the builder's), and the builder **reads only
plan state that is final before the assignment** (so a later re-run cannot
differ). Check the second one specifically against whatever runs *between* the
two call sites — here an ABI installer ran in between, and its only look at
`plan.abi` turned out to be inside a `#[cfg(test)]` block. Without that check
the second call site could have been genuine and the finding overstated.

**Measure it with a defect in the derivation, not by deleting the gate.**
Removing a correct gate leaves a green suite, so it measures nothing. Inject a
compile-valid defect into the builder instead and watch two numbers: 114 of 611
tests reddened, and the gate's own message appeared **zero** times. That
separates "the defect is caught" from "this gate catches it" — the coverage was
entirely downstream behavioural tests.

**The tell is a comment claiming independence.** *"The two populations are
independently re-derived and checked"* is what makes it costly: a reader
budgeting trust sees two authorities agreeing. Re-derived, yes; independent,
no. Report the word, not just the code — and price the repeat cost, since each
builder now runs three times per plan.

**Be exact about the blast radius.** In the same commit a sibling gate joined
two *separately derived* populations by origin — genuinely load-bearing. Naming
what is unaffected is what keeps the finding from being discounted whole.

Fourth instance of this class, after a join validator that derived `required`
from the very sets it then validated. When you find one, grep the file for
every other `validate_*` and ask what supplies its expectation.
Related: [[a-ruling-that-widens-a-shared-map-names-only-the-consumer-it-was-about]],
[[differential-oracle-is-blind-to-a-shared-premise]],
[[audit-a-detector-against-the-one-case-whose-answer-you-already-know]].
