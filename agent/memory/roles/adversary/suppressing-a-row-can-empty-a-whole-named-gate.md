---
name: suppressing-a-row-can-empty-a-whole-named-gate
description: >-
  An annotation campaign is verified against the aggregate package, but the
  gates are per-binary jobs whose selectors the aggregate cannot see — ignore
  the last live row in a binary and its named job selects zero tests and
  reports success. Attack the selectors, not the annotations.
metadata:
  type: feedback
scope: roles/adversary
---

# Suppressing a row can empty a whole named gate

A campaign that marks failing tests `#[ignore]` is verified the obvious way:
run the package and show `0 FAILED`. That number is an **aggregate over the
package**, and it is structurally incapable of seeing what the change did to
any **individual gate**, because a gate is a CI job with its own selector.

Measured on a 31-row runtime annotation set. Three of the touched files were
the exact three binaries a **merged** node existed to restore, each with its
own dedicated job selecting it by name (`--test <binary>`). Two of those
binaries held **exactly one** `#[test]`; the campaign ignored it. Their jobs
now select **zero tests**. The third lost 6 of 7 — and the six were precisely
the rows calling the differential oracle, leaving one survivor that asserts
something else entirely. The package run still reported `0 FAILED`, truthfully.

**The killer detail is that the workflow had already written down the harm.**
Its comment said the binary "ran nowhere — a green CI carried no information
about whether native and interp agreed." The job was *created to end that
state*; the campaign restored it, and the job kept reporting. An aggregator
downstream tested only `result == success`, so an empty selection satisfies a
required check with no signal.

⇒ **When a change suppresses, deletes, renames, or `cfg`s out test rows, take
the set of files it touched and grep the CI config for every job whose
selector names one of them. Then count the surviving live rows per selector.**
Zero is the alarm; one-survivor-of-seven is the subtler one, because you must
also ask whether the survivor exercises the mechanism the job is *named for*.
The count alone will not tell you — read the survivor's body.

**Two traps around this shape.**

*Say the conjunction aloud.* The finding was three binaries. Left unstated, it
operationalizes into an AC keyed on the most vivid one and the other two stay
empty and green — see
[[a-conjunction-finding-gets-silently-decomposed]].

*Do not pick the failure direction you did not measure.* Whether an
empty-selection job goes vacuous-green or hard-red depends on the runner's
no-tests default, and a repo can SHA-pin the installing action while leaving
the tool version floating — so the repo does not determine it. Both branches
are worth reporting; asserting one is
[[an-error-in-the-safe-direction-is-a-claim-about-what-you-did-not-measure]].

**Companion gap, and it is what makes the suppression permanent:** grep for
`--ignored` / `--run-ignored` / `include-ignored` across CI and scripts. If it
appears nowhere, no mechanism will ever re-run a suppressed row, so neither an
over-annotation nor a landed repair is observable — the repair ships with its
own regression cover switched off. The verification run and the suppressed
population are **disjoint by construction**, which is the same blindness as
[[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]].
Related: [[a-vacuous-law-has-zero-trust-delta]],
[[hunt-the-stand-down-clause-it-lives-in-prose-no-gate-reads]].
