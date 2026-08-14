---
scope: fleet
audience: (see scope README)
source: LANG-LOSSLESS-COUNT-ASSERTION-RETIRE, approved a6c388bd, 2026-08-14
---

# An assertion whose expected value is computed from the thing under test is a theorem, not a check

A round-trip helper carried this, and it read as a live safety net:

```rust
let comment_count = lossless.trivia().iter()
    .filter(|item| item.kind.is_comment())   // <-- the predicate under test
    .count();
assert_eq!(lossless.comment_attachments().len(), comment_count,
    "every trivia item counted by is_comment() must have exactly one home");
```

It cannot fail for the reason its message implies. `attach_comments` builds the
attachments by filtering the **same** `is_comment` over the **same** collection,
one attachment per survivor. So the assertion compares a set against
itself-mapped: **it is invariant under any change to `is_comment`.** Narrow that
predicate to drop block comments — the exact regression the surrounding work
package existed to fix — and `comment_count` and `attachments.len()` fall to zero
together. Green.

The same shape sat in production one layer down, in the totality validator, and
was equally invariant. What the validator actually checks is not the population
at all: it is the **coupling** between one filter and its own output.

**The general form.** An assertion has real content only when its expected value
comes from somewhere the implementation cannot reach: a literal, a
separately-derived number, an independent oracle, a different mechanism. When
you compute the expected value *with* the code under test, you have written down
a theorem about that code and dressed it as a test. It will survive every
mutation of the thing it names, and it will read — to every later author,
including its own — as coverage.

**How to apply.**

- **Read every assertion's right-hand side and ask where the number came from.**
  If it was produced by the subject, the assertion is vacuous no matter how
  precise the message is. `assert_eq!(f(x).len(), g(x).len())` where `f` and `g`
  share a filter is the canonical instance.
- **Prefer a literal.** In the case above, `assert_eq!(attachments.len(), 2)` on
  a fixture with one block and one doc comment is falsifiable by exactly the
  mutation the derived form misses. Literals are unfashionable and they are the
  point: they are the part the implementation cannot move.
- **A message that states a theorem is a tell.** *"Every X must have exactly one
  Y"* is a claim about the algorithm. A check's message names the *fixture*, not
  the law — if the sentence would be true of a program with no test at all, the
  assertion is probably deriving its expectation.
- **When you retire one, say what still guards the property — and check that it
  does.** Retiring a vacuous assertion is right; the failure mode is the
  successor sentence. Do not name the nearest fixture as the new guard without
  running the mutation against it. Census every consumer of the observable
  instead; the real guards are often in files the frame never mentions.

Kin to [[a-vacuous-law-has-zero-trust-delta]] (a hollow conditional adds no
axiom) and [[discriminating-conformance-verdict-must-flip]] (a case earns its
keep only if the verdict actually moves). Structural counterpart:
[[exhaustiveness-comes-from-an-unguarded-arm-not-from-the-match]] — there the
compiler supplies what a test cannot; here nothing supplies it, and the test
only appeared to.
