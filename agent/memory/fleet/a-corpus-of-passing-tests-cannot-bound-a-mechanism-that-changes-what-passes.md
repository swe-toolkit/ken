---
name: a-corpus-of-passing-tests-cannot-bound-a-mechanism-that-changes-what-passes
description: A census over the non-ignored tests cannot bound a mechanism when failing at that mechanism's condition is what got tests ignored — the empty result is guaranteed by the membership rule, not observed. Ask for the membership rule at every claim (writing it down once does not stop the inference), carry an exclusion's reason into every conclusion built on the same observations, and name which disposition flips if the claim is wrong.
metadata:
  type: feedback
---

# A corpus of passing tests cannot bound a mechanism that changes what passes

**The rule.** When a census bounds a mechanism ("the source-reachable population
is measured empty"), ask what the corpus's **membership rule** is. If failing at
this condition is what removed a test from the corpus, a census of the survivors
cannot see the condition. The empty set is then guaranteed by the selection
criterion, not observed. The same inference recurs even after the rule is
written down, so the question is re-asked at every claim, not once per campaign.

Sibling of [[corpus-property-gate-only-as-strong-as-the-corpus]], where the
corpus merely happened to lack the shape. Here the corpus was selected to lack
it.

## Instance 1: a refusal defeated on "measured empty"

**Measured 2026-08-15 on `de551a4dd..4eec77390`.** A substitution replaces a
`Closure`'s environment with a `Record` before the carrier transfer, defeating
the closure refusal for the substituted case. It merged on two legs: the
direction is fail-safe, and the source-reachable population is measured empty
over the non-ignored `ken-cli --tests` corpus (81 completed returns, all `{}`).

Six ignored tests carry, verbatim, *"a runtime-local closure has no durable
lane across the boundary."* 161 `#[test]`, 33 `#[ignore]`, 6 naming this exact
condition. The measured population is "programs that currently compile," and
the mechanism exists to change what compiles.

- **Distinguish "the probe ran" from "the population was reachable."** Plans
  carried 7 to 301 source occurrences, offered as proof the probe ran. It is;
  it proves the instrument fired. It does not prove the corpus could contain the
  shape. Anti-vacuity evidence usually establishes the first and gets read as
  the second.
- **Claim the narrow version.** The finding was not that the six excluded tests
  would show a non-empty set; it was that the corpus is structurally incapable
  of answering the question. "This cannot answer" is settled by reading the
  selection rule; "the answer is wrong" invites a counter-measurement.
- **Separate the legs and say which one carries.** "Fail-safe direction" stands
  alone; "nothing reaches it" is what makes defeating a refusal tolerable. A
  merge argument with an independent safe leg reads as robust until you point
  out the other leg is doing the work.
- **Prefer the cheap documentation repair to the expensive re-measurement.**
  "Measured empty over the corpus that excludes this population" is defensible;
  "the source-reachable population is measured empty" is the sentence a later
  reader cites when deciding the seam was cleared.

## Instance 2: the reason a measurement was excluded bounds what including it proves

**Measured 2026-08-16 on `2b4ad0faa..c88a5e423`**, a `+3/-0` comment-only cut
closing instance 1's finding.

A reviewer excluded the six tests' returns from the tally because "those returns
precede the attempted crossing, and the crossing that would populate the field
is exactly what gets refused." The ruling is right: folding them in would raise
the tally while weakening it. But the node's conclusion one paragraph away read
"all six agreed, so the emptiness claim survives a corpus that can actually
contain the shape." If those returns structurally cannot populate the field, the
agreement is forced, not observed, and the six cannot bear the shape either;
that is why they are ignored.

- **When a control-validity ruling says an observation cannot vary, carry it
  through to every claim built on the same observations.** Reviewers apply such
  a ruling where it was raised (the tally) and leave the neighbouring
  conclusion's wording intact.
- **Say what the work did buy.** Six assumed behaviours became six measured
  ones. A finding that only removes a conclusion invites deleting the work;
  naming the residual value protects it while the overstatement is corrected.
- **Re-selecting by stated reason is the original error at smaller scale.** The
  repair named "the six closure-at-boundary tests." Censused: 33 ignored, 6 with
  that reason, and the largest remaining block (20 of 27) was ignored for a
  reason naming the same subsystem being measured. An excluded test contributes
  nothing regardless of why it is excluded, so a repair naming only the subset
  that matches the diagnosis reproduces the diagnosis's blind spot. Ask for the
  membership rule and keep the credit: "excludes all 33, of which the six were
  measured individually."
- **Check which half of an edited sentence is actually perishable.** The merge
  flagged that a paraphrase dropped a base-SHA pin as the perishable part.
  Dropping it made the text more durable; the reason outlives the SHA. The
  perishable clause was the one added: a present-tense claim that certain tests
  are `#[ignore]`d, which becomes false exactly when the node fixing them
  succeeds. When someone flags a dropped clause as perishable, check the added
  ones against the same standard.

## Instance 3: a written-down membership rule did not stop the inference

**Measured 2026-08-17 on the campaign's largest deletion.** A behaviour moved
from compiling to refused, claimed as a representability gap over zero
source-reachable programs. Against its own evidence: the refusal is expected at
no real-source layer, its rows are Rust-built fixtures throughout, and the
source-level corpus excludes six tests by `#[ignore]` whose stated reason is the
same shape as the refused row.

A prior node had already been filed to make that membership rule visible. The
rule was written down and the same inference was drawn anyway. A documented
membership rule is a reading aid, not a guard: nothing reds when a census is
taken over the surviving set.

- **Check whether a node already exists to settle the premise.** One had been
  filed for exactly this refusal and had not reported, so the load-bearing
  premise of the largest change was the open question of the node commissioned
  to answer it. That framing lands harder than the corpus argument alone,
  because it needs no methodological agreement.
- **Name the disposition the claim flips.** "Zero source-reachable" is what
  converts compiling-to-refused from a capability loss into a representability
  gap. A scope objection to a measurement reads as pedantry until you say which
  disposition flips if it is wrong.
- **A census whose predicate does not match its question.** The same merge
  self-reported a compile-time census (no deleted symbols referenced) answering
  a behavioural question (do these controls still pass), when the controls
  assert on an emitter outcome and red without naming a symbol. Say when the
  author caught it themselves; that is what makes the pattern a shared method
  rather than a running complaint.
