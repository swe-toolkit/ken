---
scope: fleet
audience: (see scope README) — anyone counting occurrences with a regex,
  anyone who "hardens" a working instrument by adding an exclusion to it, and
  anyone whose refined measurement disagrees with their first rough one
source: 2026-09-19 — the Steward measured the net `#[ignore]` change on the
  operator's top-priority lane. The first, naive grep was correct. Adding a
  guard to exclude diff headers made it report zero removals, and the false
  zero agreed with the roster's published story, so nothing about it looked
  wrong.
---

# A guard added to a pattern can consume the token the pattern searches for

Counting removed `#[ignore]` attributes in a diff. The naive pattern was right:

```sh
grep -c '^-.*#\[ignore'        # correct
```

Then it got "hardened". A diff has `--- a/path` header lines starting with
`-`, so exclude them by requiring the next character not be a dash:

```sh
grep -c '^-[^-].*#\[ignore'    # reports ZERO
```

**`[^-]` matched the `#`.** The line is `-#[ignore = "..."]`: `^-` takes the
dash, `[^-]` takes the `#`, and `.*#\[ignore` then needs a **second**
`#[ignore` later on the same line. There is only one. Every removal became
invisible.

> **A character class written to exclude noise will eat the signal whenever
> the two are adjacent.** The guard cannot tell that the character it is
> consuming is the first character of the thing being counted — it sees one
> position, not a role.

## THE REFINEMENT WAS LESS CORRECT THAN WHAT IT REFINED

This is the part worth carrying. **The rough instrument was right and the
careful one was wrong**, and every signal pointed the other way: the second
version was longer, handled a known edge case, and was written *because* the
first looked sloppy. Reviewing it reads as reviewing a hardened pattern.

⇒ **When a refined measurement disagrees with the rough one it replaced, the
refinement is the suspect.** Not the first pass. The disagreement is free
information and it is thrown away by assuming the newer number supersedes the
older, which is the default reading.

## THE FALSE ZERO POINTED WHERE THE STORY ALREADY POINTED

`lanes.md` said one row had cleared and the lane was converging. The broken
grep said zero removals — no contradiction, nothing to explain, consistent
with the roster. **A false zero that agrees with the artifact you are checking
against does not present as a measurement at all; it presents as
confirmation.** Had it disagreed it would have been re-run in seconds.

The two errors compose: an instrument defect that is hard to see in the
pattern, producing a value that is hard to doubt in context. Either alone gets
caught.

See [[the-false-zero-that-agrees-with-you-is-the-one-that-is-never-caught]]
for the general form; this file is the mechanism that manufactured one.

## THE REMEDY BELOW WAS ITSELF INCOMPLETE — MEASURED THE SAME DAY

**Added hours after this file landed, by its author.** The staged-pipe form in
step 1 fixes the character-class bug and is still correct for that. It does
**not** fix a second, independent defect in the same measurement, and the
corrected pipeline reported a number that was still wrong.

**`#\[ignore` matches the ATTRIBUTE and every PROSE MENTION of it.** The reason
strings on these rows are multi-paragraph essays, and one of them is a comment
reading `SO THIS ROW IS TWO BLOCKERS UNDER ONE #[ignore] AND EITHER FIX ALONE
LEAVES` (`px8ta_oriented_subcontinuation.rs:307`). Measured on `crates/`:

```
raw   `#\[ignore`                 28  at BOTH ends of the window
anchored `^[[:space:]]*#\[ignore` 23 -> 22
```

⇒ **The raw count was inflated by roughly six mentions and, worse, was FLAT at
both ends** — so it reported *zero clearances* over a window in which a row was
genuinely cleared. **The same false-zero-that-agrees-with-the-story as the
original bug, reached by a completely different mechanism.** Two defects, one
symptom, and fixing the first is what made the second look settled.

**An attribute is a line-initial token; a mention is anywhere.** Anchor it:

```sh
grep -c '^[[:space:]]*#\[ignore'
```

**And a count is not an identity.** Anchoring still cannot tell a *cleared* row
from a *re-attributed* one — 10 of 11 removals that window were the same tests
re-appearing with rewritten reasons. Pair removals to additions **by the
enclosing test name**, and confirm the survivors at the tree (`git show
<ref>:<path>`), because a nearest-following-`fn` heuristic mis-attributes at
hunk boundaries — mine did, on exactly one row, and produced a duplicate name
that was the tell.

⇒ **The general rule: `grep` keys on TEXT and cannot distinguish a definition
from a mention of one.** Where the subject's own documentation quotes its
syntax — and a `#[ignore = "<essay>"]` corpus is precisely that — the naive key
counts the documentation. Anchor to whatever makes it a definition, and
reconcile per-file counts across the two ends rather than trusting a total.

## How to apply

1. **Stage the guard and the match separately, and anchor the key.** One filter
   per concern, piped — never interleaved in a single pattern, where they
   compete for the same characters — and never an unanchored key for a
   definition:

   ```sh
   grep '^-' | grep -v '^---' | grep -c '^-[[:space:]]*#\[ignore'
   ```

   Each stage is independently checkable, none can consume another's token, and
   the anchor keeps prose mentions out of the count.

   **Verified against a known answer before being written here.** On the window
   above, the anchored form returns `-11 / +10` — net `-1`, reproducing the
   `23 -> 22` that `git show <ref>:<path>` confirms at the tree — while the
   unanchored form returns `-11 / +11`, net zero. Positive control: on a
   three-line fixture holding an unindented attribute, an indented one, and a
   prose mention, it returns **2**. A remedy shipped without that control is
   the same mistake one level up, which is how this file came to need
   amending.

2. **Run the guarded and unguarded forms and compare.** They should differ by
   exactly the noise you meant to exclude. Any other delta is the guard
   misfiring. This costs one extra command.

3. **Print the matched lines once before trusting a count.** `grep -n` on a
   sample beats `grep -c` on faith; a zero has no lines to inspect, which is
   precisely why a zero needs the unguarded form run beside it.

4. **State counts as a split, never a sum.** `-10 +11` shows ten removals and
   eleven additions; `net +1` hides that ten rows were touched. Here the split
   was what revealed the removals were re-attributions rather than
   clearances — the net alone would have said the lane went backwards, which
   is a different and wrong story.

Adjacent-instrument siblings:
[[an-oracle-that-greps-a-name-fires-on-prose-that-denies-it]],
[[grep-the-producer-not-the-cited-proxy]],
[[a-probe-truncated-before-the-grep-is-not-a-measurement]].
