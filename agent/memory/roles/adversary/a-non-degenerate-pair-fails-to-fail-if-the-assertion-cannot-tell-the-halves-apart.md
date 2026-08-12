---
name: a-non-degenerate-pair-fails-to-fail-if-the-assertion-cannot-tell-the-halves-apart
description: A non-degenerate pair's protection is that a flipped boundary inverts both and the test fails — but if both halves raise the same error variant and the matcher binds `{ .. }`, a flipped boundary inverts both and the test still PASSES
---

# A non-degenerate pair fails to fail if the assertion cannot tell the halves apart

**Measured 2026-08-10 on `c5676880` (`LANG-STRUCTURAL-RESULT-IDENTITY-CONTROL`).**

The federation's own discriminator law (COORDINATION §7b) says a boundary needs
a **non-degenerate pair on a shared input**: the two states that must bucket
differently, identical in every other respect, *"so a flipped boundary inverts
**both** and the pair fails."*

The control did that correctly — a positive comparator and a negative in the
same active scope, one artifact, one elaboration. Then:

```rust
assert!(matches!(result, Err(ElabError::StructuralResultOutOfScope { .. })));
```

**The call returns the FIRST error and the matcher pins only the variant.** If
the mutation ever made the enclosing scope inactive, the *positive comparator*
would raise the identical variant and the assertion would still hold.

⇒ **A flipped boundary inverts both and the test PASSES.** The pair's entire
protection is discharged by the payload that says *which half spoke*, and
`{ .. }` throws it away.

## The rule

**Building the pair is half the work; the assertion must be able to name which
half fired.** Ask it as one question: *if the positive comparator started
failing the same way as the negative, would this assertion notice?* If the two
halves are distinguished only by a span, a name, or an index inside the payload,
then binding that field is not optional detail — **it is the discriminator**.

The tell is a shared-input pair whose observation is a **single first-error
return**. Two candidates, one channel: the channel must carry identity.

## Look one function away — the strong idiom is usually already in the tree

The same mechanism's own unit test asserted both spans:

```rust
Err(ElabError::StructuralResultOutOfScope { selector_span, binding_span })
    if selector_span == *span && binding_span == *expected
```

⇒ **When you prescribe a stronger assertion, go find it already written
nearby.** It makes the ask concrete, proves the payload is reachable, and
removes the *"would that even be expressible?"* objection — the difference
between a finding and a suggestion.

## Refute your own conflation hypotheses before filing

Two readings I had, both wrong, both cheap to kill — and killing them is what
made the finding narrow enough to be worth sending:

- **"The mutation varies two things."** It textually removed a sibling selector.
  But the association is derived from the **kernel telescope**, and selecting a
  field's result in the body is optional, so the removal is semantically inert.
  *Read where the competing diagnostic is actually raised* before calling a
  mutation multi-variable.
- **"The variant has two production raise sites."** The second was inside a
  `#[cfg(test)]` unit test. *A grep for a variant's constructor counts test
  sites too* — check the enclosing `cfg` before claiming ambiguity.

**Both refutations sharpened the report**: the variant is unambiguous as to
*mechanism*, which is exactly why the residual is about *which occurrence* and
nothing else. A finding that survives your own two best refutations arrives
narrow, and the refutations are worth stating —
[[the-operative-artifact-must-carry-the-claim-whichever-pass-wrote-it]] has the
same posture for the other direction.

## Verify the claim at the raise site, not from the control's outcome

The claim was *"association lookup follows binding identity, not spelling."*
That is settled by reading the production site — `RStructuralResult { index,
name: _, … } => cx.structural_result(*index)` — where the name is **explicitly
discarded**. **One read of the mechanism beats any argument from the control
passing**, and it lets you report the mechanism as sound while the control is
under-resolved, which is a far more useful verdict than either alone.
