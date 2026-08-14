---
id: LANG-MATCH-DIAGNOSTIC-PROSE
title: "The match checker's two error variants now SAY things that are false -- the exhaustiveness message calls an applied pattern a constructor, the reachability doc cites 34 §5 (Refinement types) for an obligation in §4.2, and a test file's header still advertises a gap the same file's own regression test proves closed"
status: merged
owner: language
size: S
gate: none
depends_on: [LANG-EXHAUSTIVENESS-WITNESS-PAYLOAD]
blocks: []
github: null
origin: "Two items are the Architect's, recorded in his LANG-EXHAUSTIVENESS-WITNESS-PAYLOAD verdict (dec_15pkbkyeae43t, resolved 2026-08-14T15:16:35Z): the SHOULD-FIX on the Display noun, and the CARRY that the witness is one piece of constructive evidence rather than the complete gap. The remaining three were measured by the Steward at origin/main dfd00ba8 while framing this node. Filed because Language's entire other backlog is two gate: operator nodes, so this is the ring's only ungated work."
---

## What this is

**A prose-and-citation defect, and every item is a statement in the tree that is
FALSE rather than merely thin.** That is the bar this node was filtered against
-- *would a fresh agent read the inaccuracy as ground truth and act on a false
premise?* Items that only read awkwardly were left out.

**It is deliberately small.** The payload work is done and merged; this is the
prose that did not move with it.

## The five items, measured

**Item 1 and item 2 are the Architect's own, non-blocking on that merge and
explicitly conditioned: they should land "before someone cites this diagnostic
as precedent for the wording."**

**`I1` -- the exhaustiveness message calls an applied pattern a constructor.**
`crates/ken-elaborator/src/error.rs` renders
`"non-exhaustive match at {}-{}: missing constructor '{}'"`, and the payload it
interpolates is now a `MissingPatternWitness` whose `Display` produces an
applied pattern. A user with a partial `Vector` match reads
**`missing constructor 'ConsVector _ _ _'`**. `ConsVector _ _ _` is not a
constructor. **The distinction between the name and the applied pattern is the
entire reason the payload was changed, so the one place a user actually reads is
now the one place that still conflates them.** `34 §4.1` supplies the wording:
the error *"names the unmatched pattern"*. It never says "missing constructor".

**`I2` -- the `ExhaustivenessError` doc comment is false twice over.** It reads
*"`missing` names the first uncovered constructor (`34 §4`)"*. The field is no
longer a constructor, and **"first" overclaims**: `§4.1`'s witness is *"the
constructive evidence of the gap"* -- **one** most-general uncovered pattern. A
match can be non-exhaustive in several columns at once, and neither the payload
nor the spec promises the witness is the only gap. **A consumer that reads it as
complete would be wrong.** The Architect named the type's doc comment as the
right home for that qualification.

**`I3` -- `ReachabilityError`'s doc comment cites the wrong section.** It reads
*"A redundant match arm (`34 §5`)"*. **`34 §5` is "Refinement types".** The
obligation is **`34 §4.2` Reachability** -- *"an arm whose patterns are entirely
subsumed by the union of the earlier arms matches no value and is a
redundant-arm warning/error."* A reader following the citation lands in an
unrelated section.

**`I4` -- `ReachabilityError`'s message may describe an implementation that no
longer exists, and this one is a MEASUREMENT, not a finding.** It renders
*"redundant match arm at {}-{}: constructor already covered"*. `§4.2` defines
redundancy as **subsumption by the union of the earlier arms**, which need not
be a single constructor -- and the checker was explicitly changed to compile a
column-by-column pattern matrix that splits on nested sub-patterns, precisely
because tracking coverage by top-level constructor was wrong. **Determine at the
three production emission sites whether "constructor already covered" is still
an accurate description of every case that reaches them.** If it is, say so and
change nothing. **Do not rewrite it on this frame's suspicion.**

**`I5` -- a test file advertises a gap its own later test proves closed.**
`crates/ken-elaborator/tests/val1_string_literals.rs` carries a header block
listing *"Workaround for two surface gaps: GAP-nested-patterns: nested
constructor patterns trigger ReachabilityError"*. **The same file, further
down, holds `is_even_nested_pattern_elaborates_and_reduces`, whose own comment
says the checker "used to" track coverage by top-level constructor and
elaborates a `Suc (Suc m)` nested pattern directly.** `l2_acceptance.rs` carries
the matching regression block. **The gap is closed and the header still sells the
workaround** -- which is how a future author concludes nested patterns must be
avoided and writes an accumulator type they do not need.

## Deliverables

**`D0` -- re-derive all five at your base before changing anything.** This node
was framed at `dfd00ba8` against a payload that merged in PR #2215. **Report
what you actually find**, including any item that is already correct. An item
this frame got wrong is a finding, not an obstacle.

**`D1` -- `I1` and `I2`: the message noun and the doc comment.** The noun comes
from `§4.1`'s own wording. The doc comment must stop saying "constructor",
stop saying "first", and say what the witness **is** -- one most-general
uncovered pattern, constructive evidence of a gap, not a claim that it is the
only one.

**`D2` -- `I3`: the citation.** `34 §4.2`, not `34 §5`. **Check the
neighbouring variants' citations while you are in that file** -- a wrong
cross-reference is rarely alone -- and report what you checked, not just what
you changed.

**`D3` -- `I4`: measure, then decide.** Either the message is accurate and
stays, with the measurement recorded, or it is not and it is corrected to
`§4.2`'s subsumption language.

**`D4` -- `I5`: the stale header.** Remove the closed gap from the workaround
list and point at the regression test that closed it. **Do not delete the test
or restructure it to use nested patterns** -- it is a passing test of something
else, and rewriting it is a separate question.

## Acceptance criteria

**`AC-1` -- no user-facing string calls a pattern a constructor.** Grep the
rendered forms, not the source spellings, and report the enumeration.

**`AC-2` -- every changed citation resolves to a section that exists and
contains the obligation.** Quote the section heading you landed on. **`34 §5` is
"Refinement types"; that is how this class of defect is caught, and it is
checkable.**

**`AC-3` -- `I4` is answered by evidence from the three emission sites**, and
the answer is recorded whichever way it goes. **"Reworded for clarity" does not
discharge it.**

**`AC-4` -- the second half of `I5` is verified, not assumed.** The header lists
**two** gaps. This frame only establishes that the nested-patterns one is
closed. **Check the mutual-recursion claim separately and leave it alone if it
still holds** -- removing a live caveat is worse than leaving a stale one.

**`AC-5` -- no behavior change.** No payload, no emission site, no checker
logic. If a fix appears to require one, **stop and report** -- that is a
different node.

**`AC-6` -- no-regression, in CI.** `COORDINATION §12` -- the venue is CI, never
a local `--workspace` run. Build targeted, `-p ken-elaborator`.

## Sizing

**`S`, and it should be well under the hour.** Five prose items in two files
plus one measurement. **`D3` is the only one that can surprise you**; if it
turns into a checker question, hand that back rather than absorbing it.

## Not this node

- **Not a general `ElabError` diagnostic-quality pass.** Two variants, and only
  because their payloads or their citations went stale under a landed change.
- **Not a change to the exhaustiveness or reachability checker.** See `AC-5`.
- **Not the redundant-arm feature itself.** `§4.2`'s two subtleties -- that
  guarded arms do not cover, and that a literal column never closes -- are
  behavior questions. **Guards are not implemented in the surface at all**, so
  the first is vacuous today; neither is in scope here.
- **Not an amendment to `34 §4.1` or `§4.2`.** The spec is the fixed input; the
  tree is what moves.
