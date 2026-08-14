---
scope: enclave
audience: (see scope README)
source: LANG-TRIVIA-KIND-MAPPING-PIN → LANG-COMMENT-POPULATION-PARITY, 2026-08-14
---

# A stated impossibility carries a qualifier, and the qualifier is the escape

A pin claimed all four `CommentKind` arms were covered, with one arm discharged
by a fixture in **another crate's test file** whose discriminating property was
its *configuration*. The reason given was that `CommentKind` is `pub(crate)`, so
**"no integration test can name it"** — true, and I checked it.

I approved, and required a `LOAD-BEARING SHAPE` comment on the far fixture
naming the dependent file, the mutation it catches, and that **nothing reds** if
someone reshapes it. It was a good guard. A later node deleted it, because the
coupling it guarded **did not need to exist**: a `#[cfg(test)]` module *inside
the crate*, beside the `From` impl, names all four arms directly. The successor's
own words: *"a cross-file coupling that was a scope decision, not an
impossibility."*

**The tell was in the sentence I verified.** *"No **integration** test can name
it"* is not *"no test can name it."* The qualifier was load-bearing and I read
past it, because the claim was true as stated and I was checking whether it was
true rather than what it was scoped to. I then spent my finding on **hardening
the consequence** instead of questioning the premise — and a guard rail is
seductive precisely because it is a real improvement to a situation that should
not exist.

**How to apply.**

- **When someone says X is impossible, read the qualifier and ask what it
  excludes.** "No integration test", "not from this crate", "not without a
  public API", "not at this layer" — each names a boundary *and* the place the
  boundary stops. Verify the claim, then verify the *scope* of the claim.
- **Before hardening a coupling, ask why it is a coupling.** A cross-file /
  cross-crate dependency guarded by a comment is strictly worse than the same
  property asserted where it lives. Subsume-don't-proliferate applies to
  *evidence*, not only to mechanisms.
- **A guard that says "nothing reds" is a diagnosis, not a fix.** If you can
  write that sentence, you have found a place where the invariant has no
  enforcer. Ask whether the enforcer can be moved to it before you write the
  warning.
- **The retirement must land AFTER the replacement.** The successor sequenced it
  correctly — the in-crate pin lands in the first commit, the stale coupling is
  retired in the third, so no intermediate state leaves the arm unpinned. The
  natural order (delete the stale comment while you are already in the file) is
  the wrong one, and the text left behind is its own hazard — see
  [[deleting-a-check-has-a-text-surface-and-it-outlives-the-check]].
- **A node that states a rule can violate it one file over.** The same commit
  answered "an integration test cannot reach this" **twice** — once by moving the
  test in-crate (correct, and stated as the reason), once by widening a
  production `fn` to `pub` for a test's benefit. Neither is necessarily wrong;
  the unrecorded *asymmetry* is. When you find a node articulating a principle,
  grep the same diff for the other branch of it.

Sibling of [[surface-the-seam-need-not-your-preferred-mechanism]]: there the
mechanism was the owner's to choose; here I chose a mechanism for a seam that
should have been closed instead.
