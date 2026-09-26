---
name: a-test-pinning-a-missing-port-refusal-inverts-its-signal-when-the-port-lands
description: A witness that pins the refusal its own node exists to remove goes red when the next increment succeeds, with a message asserting a regression. When the port lands, repointing such an oracle to completion is a correction (not a loosening) if completion still inverts under the guarded regression and a mechanism check survives; a negative-contrast discriminator ("a different input still refuses") cannot be carried by completion and its loss must be surfaced.
metadata:
  type: feedback
---

# A test pinning a missing-port refusal inverts its signal when the port lands

An **advancing refusal** is a node whose acceptance is that a refusal moves or
disappears: the program lowers further than before and stops somewhere later,
or completes. Witness tests for such nodes are predicates on the **failure**.
Both ends of that life cycle have been measured: the pin being written (it
inverts on success), and the pin being repointed when the port lands (correct
versus loosened). The QA-side rule is in the `qa-test-design` playbook's
transition-sentinel class, which requires naming the event that retires a
sentinel; this lesson is what happens when that is skipped, and how to audit
the repoint.

## 1. A pin on the current refusal inverts when its own node succeeds

**Measured 2026-08-17 on `5bac56000`** (`RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT`
D1). The node's frame stated its acceptance in bold — *"the refusal advances --
it no longer originates at route 1. The acceptance is the advance, not a green
test"* — and warned that the advance *"will read as this node failing when what
happened is that it succeeded."* Its witness test then pinned the refusal:
`assert_eq!(route1.len(), 1, "the two-arm plain Match must take route 1 exactly
once")` plus `result.expect_err(...)`. Both hold only while the refusal is
unfixed, so when the next increment lands the pin goes red inside that
increment's diff with failure text asserting a regression. The node authored
the warning against the misreading its own artifact manufactures.

⇒ **When a node's success condition is "the current behaviour stops
happening", read its tests as a predicate on the FAILURE and ask who turns them
off.** Such frames almost always assign no owner for retiring the pin, because
it was written to discharge this increment's AC and nobody re-read it against
the next one. Sibling of
[[a-carried-obligation-gated-on-a-merge-event-fires-on-an-accepted-partial]].

**The instrument makes the inversion unreadable.** The recorder pushed a row
only at the route it watched, so the `0` that arrives with the advance is
consistent with *entered, took another route* (the AC's success) and *never
entered* (an earlier `Err`, or any change at the single caller). Nothing
recorded separates them. The contrast sat 14,700 lines up in the same file: the
sibling census records a row for every entry and partitions with a
`reached_selector` flag, and says why — *"a census of firings is a numerator,
not a population"*; *"a silently empty census and a genuinely empty population
are the two readings this must never conflate."* The new observer copied that
helper's scope machinery verbatim (identical `Restore` shape) and dropped both
properties. ⇒ **Diff a new observer against the nearest existing one in the
same file — a re-derivation keeps the mechanics and loses the design**
([[a-pin-cannot-disagree-with-its-own-source]]). What such
recorders do and do not witness:
[[a-recorder-witnesses-the-line-it-sits-on-not-the-mechanism]].

**Measure the denominator; do not argue it.** The first hypothesis was that the
firings-only row made `len()==1` weak today. One sentinel row pushed at function
entry answered it: `1 -> 2`, so the function is entered exactly once and that
entry takes the route. The hypothesis was false, and the surviving finding was
strictly about the next increment's reading. ⇒ A missing denominator is a
two-line mutation away from being a number: push a sentinel row at the head of
the function and the existing `assert_eq!` prints the population. Revert and
re-confirm byte-identity after
([[disable-the-mutation-mechanism-to-find-vacuous-controls]]).

## 2. When the port lands: corrected, or loosened to pass?

**Measured 2026-08-24 on `cac27f3b`** (M6 RT-CHECKED-IH respin #2, after a CV
reject of respin #1 for four fixtures failing stale terminal/recorder oracles).
Four fixtures each compiled a byte-identical witness program (all four witness
sources sha1 `60a1be46`) that used to advance through several subsystems and
stop at a checked-IH nullary-force refusal (*"static worker expects 1 arguments
but call provides 0"*), pinned as each fixture's advancing-refusal sentinel. M6
closed that refusal, so the program completes. The respin was delta-anchored:
core and census byte-identical to the already-cleared `171c432f`, only the four
oracle bodies changed. Verdict clean with one observation; reported
evt_7gx28410e56hn (Steward M6 gate thread thr_5cnzr5sj3rdn0).

The discriminators, per staled oracle:

1. **A sentinel whose own comment said "re-measure when this boundary lands, do
   not widen/delete" is correctly discharged by repointing to the new
   terminal.** When the new terminal is completion, `result.expect(...)` is the
   re-measurement, and it is a **stronger** non-vacuity anchor than "stops at a
   later blocker": completion necessarily traverses every reader and seat on the
   program's path, so it cannot hold on an unrelated upstream failure.
2. **Correction, not loosening, iff the completion anchor still INVERTS under
   the regression the sentinel guarded** (revert the mechanism — the `Avail`
   move, the reader, the route — and the refusal returns, so `result.expect`
   panics) **AND** either a positive mechanism recorder is retained (a deferral
   ledger, an exact `match_arms_walked == 2`, a branch-walk row) or completion
   provably exercises the mechanism. A **position-frozen** assertion
   (`route1[0]`, `route1.len() == 1`) that breaks only because completion
   legitimately invokes a resolver more times is correctly replaced by a
   **property filter** (`!route1 && match_branch_entered`) plus a uniqueness
   assert (`intended.len() == 1`): selecting the row by meaning rather than
   incidental count is more robust, not weaker.
3. **Completion cannot replace a NEGATIVE CONTRAST discriminator.** A clause
   like *"a different input STILL refuses"* — here a carried seat of a different
   `Need` still refused, pinning that a route is keyed on the need rather than
   admitting any carried seat — is a contrast, and completion is not one. When a
   **sibling** node closes the contrasting input, the discriminator is gone, not
   relocated, and completion detects nothing about the keying property. That is
   a real fixture-level regression-detection loss even when the property still
   holds structurally elsewhere (here the ledger's second admissibility
   re-derives `(CarriedWord, ResourceScalar)` byte-unchanged). It is not a
   loosening-to-pass if a prior sibling forced it and it is named with a
   restoration home — but **surface it**: a completion-only fixture that used to
   carry a keying contrast is where a future "admit anything" regression slips
   through.

**Loosening proper** — an exact match relaxed to `.contains`, a specific
terminal downgraded to "any error", an assertion deleted with no anchor and no
inversion — is the finding. See also
CHECKS.md check 8
(a success anchor is not automatically a mechanism anchor) and
[[green-vs-green-does-not-confirm-a-fix]] (a bare "it compiles further" row is
vacuous under the mutation it must catch, which is why the negative contrast
existed).
