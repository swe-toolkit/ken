---
name: a-number-sound-as-a-measurement-is-not-sound-as-a-criterion
description: "A number can be correct, useful and honestly derived as a MEASUREMENT and be unfalsifiable or unreproducible the moment it is promoted to a CRITERION - and the promotion is invisible, because the number does not change. Three instances in ninety minutes across two seats. Record the derivation and compute the total at the point of use; publish the selector beside any count; phrase criteria over a predicate or a boundary, never a total."
metadata:
  type: feedback
---

# A number sound as a measurement is not sound as a criterion

**Measured 2026-09-16 on `RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE`.** Two seats,
three instances, ninety minutes — and in every case the number was *correct*.

    "66 assertions across the eight tests"
        sound as sizing: it is why the cut was too big for S.
        proposed as an AC -> UNFALSIFIABLE. A re-expression that deletes one
        assertion and adds another elsewhere holds 66 exactly, and that
        substitution is the thing the AC existed to catch.

    "four ops are reachable from Ken"
        sound as a tally.
        re-derived from the `op_*` intern table -> 3. The four arrive by TWO
        mechanisms and one is interned separately, so the obvious derivation
        silently drops the very op the dependent WP is about.

    AC-D0 pinning NO total, discharging on named items
        the same rule, applied BEFORE anyone made the mistake -- and then
        both seats reintroduced a total one artifact over.

**The third row is the one to notice.** The rule was already written down, in
the same frame, and it did not travel to the next artifact on its own.

## The second half: an UNSTATED SELECTOR makes a correct number unreproducible

Two seats counted the same population and got 96 and 102. Neither was wrong:

    grep 'dispatch_host_op_v1('   -> 81 + 15 = 96 CALL SITES
    grep 'dispatch_host_op_v1'    -> 102 RAW HITS
      the difference: 1 definition (a generic parameter sits between the name
      and the paren, so the paren-selector cannot see it), 3 `use` imports,
      2 occurrences inside `//` comments

The 96 is the useful number and the 102 is the **checkable** one, because its
accounting closes: every raw hit is assigned to a bucket. A reader who
re-derives 96 with a different selector gets a different number and cannot tell
whether they or you are wrong.

⇒ **A number published without its selector is a conclusion, not a
measurement.** The same shape as publishing a total without its derivation, one
level down.

## What to do

- **Record the derivation; compute the total at the point of use.** State the
  split, never the sum: *"3 via the intern table + 1 interned separately"*
  cannot be re-derived wrongly, while *"4 reachable"* invites exactly the
  derivation that returns 3.
- **Publish the selector beside any count**, and prefer the count whose
  accounting closes over the count that is merely relevant.
- **Phrase every acceptance criterion over a predicate or a boundary, never a
  total.** *"Only the dispatch entry name changes, at the 21 named call sites;
  no assertion line and no setup line changes"* discharges on what happened.
  *"The assertion count stays 66"* discharges on a tally that the defect
  preserves.
- **When you promote a number from one artifact into another, re-ask whether it
  can still fail for the reason you want it to.** That question is the only
  thing that distinguishes the two uses, and nothing about the number itself
  will prompt it.

Sibling of [[narrowing-a-counts-scope-never-turns-a-tally-into-a-pairing]] and
[[an-enumeration-needs-a-proven-closure-not-a-better-grep]]. The
reachability instance is the same partition described in
[[repairing-a-census-completeness-does-not-re-aim-its-subject]].
