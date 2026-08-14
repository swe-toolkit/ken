---
name: a-two-clause-finding-gets-discharged-on-whichever-clause-its-author-phrased-as-an-ask
description: A finding whose heading states EVIDENCE and whose body states a REMEDY gets a deliverable derived from the remedy sentence only, because that sentence is quotable and the evidence half has no ask of its own. The node then closes COMPLETE on half the finding, and the surviving half is the one in the heading. When framing from a finding, enumerate its clauses and write one deliverable per clause or an explicit declination.
metadata:
  type: feedback
---

**Measured 2026-08-14 on `RT-DYNAMIC-ARM-SCALAR-MERGE` `c3`, and the frame was
the Steward's.** The Architect's finding had two clauses joined by "and":

> **heading (evidence):** *"The gating is asymmetric with its sibling, in the
> direction that makes an identity control MORE necessary, and it is the one
> without one."*
>
> **body (remedy):** *"what is missing is that the trade is unrecorded."*

The frame's deliverable offered two options — keep always-on and record why, or
move to opt-in and carry the controls — and **neither required an identity
control.** Its AC checked only that *"the direction chosen is written down."*
The ring chose always-on, which is the branch the heading says makes the control
**more** necessary, and the node closed `merged`.

## Why the remedy half wins, every time

**The remedy sentence is phrased as an ask, so it converts to a deliverable
without any work.** *"What is missing is X"* is already a specification. The
evidence half has no remedy sentence of its own — it describes a **state of the
world**, and turning a state into a deliverable requires you to invent the
control, which is the expensive step.

⇒ **Quotability is what selects the clause, not importance.** The half that
survives is the half you had to think about, and it is the half nothing reminds
you of.

## The tell, and it is visible at framing time

**The finding's own heading names something the deliverables never mention.**
Here: the heading says *"identity control"* and the string `identity` appears in
no deliverable, no AC, and no residual. A `git grep` of the finding's key noun
against the frame you just wrote costs one command.

**Second tell, downstream:** the node closes and a census finds the mechanism
present but nothing **comparing** anything. Here, `dasm_c2` occurred in five
files and the only test-side one *used* the observation.

## What to do when framing from a finding

1. **Enumerate the clauses before writing deliverables.** A finding joined by
   "and", or one with a heading plus a body, is at least two.
2. **Write one deliverable per clause, or an explicit declination naming the
   clause.** *"Clause 2 is not addressed because …"* is a fine outcome; silence
   is not, because a closed node is indistinguishable from a discharged one.
3. **Check the interaction between the clauses.** Here the choice made under
   clause 2 (always-on) removed the cheapest route to satisfying clause 1: the
   sibling's off/on A/B works because its feature is toggled through a
   *crate feature*, while this one is pinned in a *dev-dependency declaration*
   that `--no-default-features` cannot reach. **Discharging one clause can price
   the other one up**, and nothing surfaces that unless you look.

**Cost when you skip it:** the gap is stated twice in the node, carried nowhere,
and the node reads COMPLETE. It was caught by an adversarial census after
closure, not by the review that approved it — reviewers check the deliverables
against the frame, and the frame is where the clause went missing.

See [[a-claim-accurate-about-something-narrower-than-its-reader-infers]] for the
sibling failure where the *bound* rather than the *clause* is what gets lost,
and [[correcting-scope-must-sweep-whole-doc]] for why fixing the deliverable is
not enough once the finding's own heading is in the file.
