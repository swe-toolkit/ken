---
name: a-repair-relocated-by-a-no-amend-control-is-invisible-at-the-reported-site
description: A frame that forbids amending an existing control forces the repair to land somewhere other than where the defect was reported, so a reviewer who re-checks the reported site correctly finds it unchanged and wrongly concludes the property is still unasserted.
---

A frame carried two things at once: a finding that
`lang_foreign_name_control_chars.rs:62-63` was keyed on absence-of-error where
the property is presence-of-value, and an acceptance criterion forbidding any
amendment to an assertion in that file (changing a control while rewiring the
mechanism it controls forfeits the control).

**Those two combine into a relocation.** The only legal place for the repair was
a NEW test in a NEW file. It landed there and asserted exactly the right thing.

The Adversary then reviewed the candidate, read the site it had reported, found
it unchanged, and argued the property was still uncovered. **Its reasoning was
sound and its premise was false** — it had checked the site the defect was
reported at, which is the correct place to look and no longer where the answer
lives.

**Both roles have something to do about it.**

- **Frame author:** when a control forbids amending the site, say in the frame
  **where the repair will land instead.** The relocation is a consequence of
  your own AC, so you are the only one who knows it before the work starts.
- **Reviewer:** "the reported site is unchanged" does not establish "the
  property is unasserted." Before concluding a repair is missing, grep for the
  **property** — the value, the error variant, the decoded payload — across the
  candidate's added files, not just the file you filed against.

The general form: **a no-amend control converts an in-place fix into a move,
and a move defeats site-keyed verification.** Any AC of the shape "do not touch
the existing test" has this consequence, and it is invisible unless someone
writes it down.

Related: [[withdraw-and-relocate-test-different-properties]],
[[amending-a-frame-mid-flight-must-sweep-its-guardrails-section]],
[[verify-the-report-is-real-before-explaining-it]].
