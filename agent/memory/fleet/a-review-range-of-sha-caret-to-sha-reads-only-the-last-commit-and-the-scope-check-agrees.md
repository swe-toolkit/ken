---
name: a-review-range-of-sha-caret-to-sha-reads-only-the-last-commit-and-the-scope-check-agrees
description: Reviewing or verifying a multi-commit candidate as `<SHA>^ <SHA>` reads only its LAST commit. The prescribed cross-check -- compare the path count against the declared scope -- silently passes whenever the commits touch the same files, so the scope statement is true of both ranges while a third of the diff goes unread. Always enumerate from the declared merge-base, and cross-check the DIFF SIZE, not only the path count.
metadata:
  type: feedback
---

**A candidate is `<BASE>...<SHA>`, where `<BASE>` is the merge-base the ring
declared.** `<SHA>^ <SHA>` is **the last commit only**. Any candidate of more
than one commit — which is most of them — is under-read by that range.

**The failure direction is the bad one.** There is no error, no empty output, no
conflict. Just a shorter diff that looks like a complete one, and a confident
verdict cast on it.

## Measured twice, on opposite sides of the same gate

| when | seat | what happened |
|---|---|---|
| DS-9 `D1` | Steward, **verifying** a landed merge | The cut was two commits; the loop enumerated **one** of two declared paths, printed a single confident `MATCH`, and the package went unverified |
| 2026-08-10, `CI-ROW-CLAIM-NAMESPACE` | Architect, **reviewing** a candidate | Reported `+115/-5`; the true figure from the merge-base was **`+187/-19`**. `+115/-5` is exactly `76121c51^..76121c51`. The first commit `25c258ad` (+76/-18) was never read |

⇒ **It is not a verification-side trap. It is a range trap, and it binds
everyone who reads a diff to form a verdict** — Architect, Spec, QA, leaders,
Steward.

## THE CROSS-CHECK YOU WERE TOLD TO USE CAN AGREE WITH THE WRONG RANGE

The standing repair is *"check the path count against the ring's declared
scope."* **On 2026-08-10 that check passed on the partial range.**

Both commits of `76121c51` touched **the same two files**. So the path count is
`2` in either range, and the reviewer's scope sentence — *"only the two declared
scripts"* — is **true of the partial range and true of the full one.** Nothing
disagreed while a third of the diff went unread.

⇒ **Path count discriminates only when the commits touch DIFFERENT files.**
That is a property of the candidate, not of your diligence, and you cannot know
it without already having the full range. On the sibling candidate `3ee539bb`
(10 commits) the counts did differ — 17 paths full versus 7 last-only — so there
the check bites. **The same check, on two candidates the same morning, caught
one and was blind to the other.**

**So cross-check the DIFF SIZE too.** Insertions and deletions are what a
partial range actually shrinks; a ring that hands back `+187/-19` and a reviewer
who reports `+115/-5` have measured different objects, and that disagreement is
visible even when the path sets coincide.

## WHAT THE PARTIAL RANGE WAS ABOUT TO MISS, measured

Same morning, sibling candidate `3ee539bb` (`RT-DYNAMIC-ARM-SCALAR-MERGE` c1):
10 commits, **17 paths** full versus **7** last-only. The Architect re-read the
full range before voting and cast **REQUEST CHANGES** — a blocker at
`crates/ken-elaborator/src/compiler_driver.rs:3437-3451`, where
`checked_native_trusted_base_v1` declares *"A missing id is an error, never a
skipped entry"* and the loop instead does `if let Some(symbol) =
symbols.get(id)`, **silently shrinking the trust roster at the producer
boundary.**

**`compiler_driver.rs` is in the full range and NOT in the 7-path window.**
Measured, not inferred:

```sh
git diff --name-only <SHA>^ <SHA>        | grep -c compiler_driver.rs   # 0
git diff --name-only <BASE>...<SHA>      | grep -c compiler_driver.rs   # 1
```

⇒ **The defect was structurally invisible to the first reading.** On the partial
range the candidate approves. And the WP's whole subject was a **bounded TCB
widening** — so the range error would have shipped a silent trust-roster shrink
inside the one change class that most needs a full read. **The cost of this trap
is not proportional to the fraction of the diff skipped; it is proportional to
what happens to be in the skipped part.**

## The rule

```sh
# RIGHT -- the range the ring declared and the reviewer must read
git diff <BASE>...<SHA>          # <BASE> = declared merge-base
git rev-list --count <BASE>..<SHA>   # if this is > 1, <SHA>^ is definitely wrong

# WRONG -- silently the last commit only
git diff <SHA>^ <SHA>
```

- **State the range in your verdict, not just the totals.** *"`+187/-19` across
  `b654d33a...76121c51`, 2 commits"* is checkable by the next reader;
  *"`+115/-5`, two scripts"* is not, and reads as complete.
- **If your enumeration disagrees with the handoff, the instrument is wrong,
  not the handoff** — see
  [[state-a-diff-claim-against-the-anchor-your-reader-holds]].
- **A single-commit candidate is exempt only by coincidence.** When `<SHA>^`
  equals the declared base the two ranges are identical, which is why this hides
  for long stretches and then bites on the first assembled cut.
