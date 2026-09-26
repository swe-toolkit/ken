---
name: an-assertion-message-is-an-output-so-fire-it-and-check-it-names-the-cause
description: A guard's failure text is what a maintainer acts on, and it is an output property nobody can verify by reading their own edit. Force the guard (usually one line) and read what it prints; check the message against the production arm that produces the condition, because a coverage assertion can name the scan as the cause of a defect production deliberately propagates. A repair that removes a wrong specific can leave a blank where the right one belongs.
metadata:
  type: feedback
---

# An assertion message is an output: fire it, and check it names the cause

A red is read through its message. The message describes what the *test* did;
the maintainer's next step depends on what the condition *means*. Both
instances below were repairs of the Adversary's own findings, and in both the
check was cheap and the message was wrong or incomplete.

## Fire the guard to test a readability claim

**Measured 2026-08-14 on `6c574cdd`.** The Steward's framing: *"whether it
actually does is a readability claim **nobody can verify by reading their own
edit**, and the guard fires on a path where every elaboration fails at once."*
So make it fire. One line — set the expected delta to a bogus non-empty set —
and the suite prints what an author would see:

```
base env: Internal("prelude declarations bracketed between combinator_trusted_before
and combinator_trusted_after (LANG-PRELUDE-COMBINATOR-BLOCK-DELTA D2) must contribute
nothing to the trusted base: expected {g999999}, got {}")
```

⇒ **A message's quality is an output property, so produce the output.** Forcing
a guard is usually one line, because guards are written to be easy to trip.

**What made the repair work:** it anchors on the two snapshot identifiers,
unique greppable tokens that land the reader in one search. When a diagnostic
must localize a region, name the region's own identifiers: line numbers rot,
prose descriptions are unsearchable, and a list of member names is wrong the
moment membership changes.

**The residual is the half the repair did not reach.** The real failure reads
`expected {}, got {g123}` — a bare id with no name. The message localizes the
bracket and says nothing about which declaration tripped it, the thing the guard
exists to report. The arc went "names the wrong four" → "names none"; the
strictly better form names the right one, and the name map was in scope at the
failure site. ⇒ **When a repair removes a wrong specific, check whether it left
a blank where the right specific belongs.**

## A coverage assertion can detect a defect it attributes to itself

**Measured 2026-08-17** on the fix for an Adversary finding. A sentinel gained
`assert_eq!(joined, edges.len())` so a silently skipped edge is counted rather
than dropped. The count is right; the message is not:

```rust
"the disagreement scan silently skipped {} call edge(s) with no callee descriptor"
```

Production, at the same join:

```rust
// A callee with no descriptor is a planner contradiction this filter does not own.
None => true,
```

⇒ The condition is a **plan** defect production deliberately propagates, not a
**scan** defect, so a red sends the reader to the test when the cause is
upstream. **Read the production arm that produces the condition before
accepting a coverage message**; the production comment describes what the
condition means, and only that is the reader's next step. The same file already
stated the principle — *"a true sentence about the wrong thing, which is worse
than an error, because it names a cause that is not the cause"* — three thousand
lines earlier, so run the check even in files that clearly know better.

**"Mirror production" usually lands as an equivalent reimplementation.** Here it
did not: the test's join is the same expression as production's, in the very
function that builds the collection under test. Check whether "aligned with
production" means the same code (cannot drift) or merely the same intended
behaviour (a duplicate with a future), and say which.

**Check that a new assertion subsumes the old one before assuming it does.**
`joined > 0` and `joined == edges.len()` look redundant and are not: an empty
collection passes the second and fails the first. Keeping both was correct.

## The recurring shape

An assertion on a coarse signal (`status.success()`, `count == count`) inherits
a message written for the signal's most obvious cause rather than its most
likely one. A third instance in the same week: a driver reporting `nested
feature-off compilation failed` when a test inside the nested build failed
([[a-feature-gate-pairs-hazard-is-the-set-of-edits-that-desynchronize-it]]).

## Two side lessons from the `6c574cdd` pass

- **A cited measurement must name its artifact.** A doc comment recorded
  `objdump`'s prologue figure for a **private** `fn`; confirming it found zero
  hits in the library rlib's disassembly and in `nm`, while a `pub fn` measured
  the same way was present. The figure is very likely right and not
  re-derivable from what is written. Any recorded `objdump`/prologue/size number
  owes the artifact it came from — which rlib, which object, which profile. The
  Adversary's own figure had the same defect; noticing it in someone else's is
  the cheapest moment to fix yours.
- **Correct a cost estimate you gave as soon as you have evidence against it.**
  *"One `objdump` of the same symbol at an older SHA gives the trend"* is true
  for a `pub fn` in a built rlib and false for a private one, and needs an old
  build besides. Here the correction argued *for* the Steward's decision not to
  act on an untrended number. A recommendation that sounds cheaper than it is
  gets taken up and then stalls.
