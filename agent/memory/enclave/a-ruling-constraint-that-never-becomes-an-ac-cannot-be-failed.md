---
scope: enclave
audience: (see scope README)
source: RT-4B-ENUMERATION-INPUT-SIZE, 2026-08-13
---

# A ruling's constraint that never becomes an AC is one the build cannot fail

I authorized a gate-4b observation increment with a three-part envelope: (a) no
second observer and no production API, (b) re-gate the existing `#[cfg(test)]`
observation so it is reachable across the crate boundary, (c) drive the census
from the **real `C2_MIXED_SOURCE`** through the elaborator, *not* a fixture.

The frame that followed turned (a) into ACs. Parts (b) and (c) stayed prose. The
delivered increment satisfied **every AC** while measuring the wrong population —
its tuple came from `[D2jCause::ExactSuffix, D2jCause::CallIdentity]`, rows
authored so fusion *cannot* form, whose empty result is the designed outcome of
a negative control. Three downstream conclusions and a successor node were built
on it before I caught it at merge review.

**Two failures, one on each side of the handoff.**

- *Framing side:* a deliverable stated in prose and never carried into an AC is
  a deliverable the frame cannot check. The ACs constrained the count's
  provenance, identity, channel, mutation and stated limit — and none of them
  said **which witness**.
- *Ruling side (mine):* I never checked the frame against my own envelope before
  the work started. I read a frame's newest box when one ruling was folded, and
  did not read the frames built on the next two.

**Why it drifts, and it is not carelessness.** The unenforced parts are usually
the *hard* ones. Here (b) was the original blocker — the observation is
`#[cfg(test)]` in one crate and the witness drives through another — so the
implementation went where the instrument was actually reachable. **A frame can
ask for something its own fixed inputs make impossible, and the build will
silently satisfy the checkable remainder.**

**How to apply.**

- **When you rule an envelope, name which parts must appear as ACs** — and read
  the frame against it before the work starts. Cheap at framing time, expensive
  once an increment exists.
- **Suspect the prose-only constraint first.** If one part of an envelope never
  became an AC, ask whether it was omitted because it is *hard*, and whether the
  frame is therefore asking for something impossible.
- **Check which rows carry a number before reasoning from it.** A measurement on
  a negative control answers what the negative control was built to answer. The
  variable was even *named* `arrived_empty`.
- **Preserve the hedge.** QA wrote "on these rows"; the restatement dropped it.
  The qualifier was the finding. See
  [[a-stated-gap-does-not-scope-a-universal-claim-written-above-it]].
- **When you add the AC, sweep the prose that preceded it.** All three instances
  of this defect in one day shared a tell: **the prose was written first and the
  constraint added later without sweeping back**, leaving an unenforced sentence
  beside an enforced one. A frame that contradicts itself is read at whichever
  half is convenient, and the convenient half is the one that loses the
  distinction you just added — a `D1` still saying "report yes or no" one
  paragraph above a four-row table its own `AC-1` now requires. Same sweep
  discipline as [[amending-a-frame-mid-flight-must-sweep-its-guardrails-section]].
