---
id: LANG-GADT-SEQUENCE-TRACKER-GAP
title: "`34 §8` names four `SURF-gadt-*` build WPs and all four have frames in `docs/program/wp/` -- none has a tracker node, so `gen-progress.sh` shows the whole dependent-constructor area as absent, while the code has in fact moved past every one of the four frames' stated baselines"
status: ready
owner: language
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Steward sweep 2026-08-14 at ca803dfc, taken while looking for Language's next WP under COORDINATION §4e (stay one release ahead). Found by running the deferral-language grep that LANG-DECEQ-CHAR-LAWFUL-INSTANCES recommends -- `spec/30-surface/34-data-match.md:900` names the four WPs -- then checking each for a tracker row."
---

## What this is

**A tracker gap, not a build gap.** `spec/30-surface/34-data-match.md:900`
states: *"The dependent-constructor feature should be built in separate WPs"*
and names four:

- `SURF-gadt-parser-ast`
- `SURF-gadt-elaboration`
- `SURF-gadt-coverage-diagnostics`
- `SURF-gadt-field-sugar`

**All four have written frames** in `docs/program/wp/`. **None has a node in
`docs/program/issues/`.** The tracker is generated from `issues/`, so the
operator's view of progress shows this entire feature area as neither started
nor planned, in both directions at once: the landed work is invisible, and so is
the remaining work.

## What was measured, at `ca803dfc`

**The code has moved past every frame's stated baseline.** Each frame carries a
"current implementation state is perishable" clause, and in each case the state
it describes is now false.

| frame says | measured at `ca803dfc` |
|---|---|
| `parser-ast`: *"`parser.rs` accepts only `data D p... = C type_atom* \| ...`"* | `parse_explicit_data_decl` exists (`parser.rs:1360`); `Decl::ExplicitDataDecl` and `ExplicitDataCtor` exist (`ast.rs:294`, `:57`); `tests/explicit_data_parser.rs` has **12** tests |
| `coverage-diagnostics`: *"As of PR #318 ... indexed coverage, omitted impossible arms, and indexed dependent-match expansion remain out of scope"* | `tests/explicit_data_elaboration.rs` has **15** tests including `indexed_impossible_constructor_may_be_omitted_from_non_empty_vector_match`, which asserts absurd-method synthesis, the passed scrutinee index, and index-before-scrutinee motive abstraction |

**Diagnostics that the frames place in the unbuilt slice are also present:**
`error.rs:183`/`:515` emit `non-exhaustive match ... missing constructor '{}'`
and `error.rs:185`/`:522` emit `redundant match arm ... constructor already
covered`; `elab.rs:8355` carries AC4 reachability bookkeeping.

## The residual is NOT established, and this node does not guess it

**The Steward measured the above by grep and by reading the landed tests. That
is enough to establish the tracker gap and not enough to establish the
frontier.** Two specific things were checked and came back inconclusive:

1. **AC5's negative half.** The landed test pins the *accept* side (omitted
   impossible `EmptyVector` arm accepts for a `Suc n` vector). `34 §8` AC5 pins
   a **pair** — the omission accepts *while* applying that function to the
   empty vector rejects. The reject half was not located.
2. **Whether the exhaustiveness diagnostic satisfies `§4.1`'s wording.** The
   spec requires naming the **unmatched pattern witness**; the implementation
   names the **missing constructor**. Those may or may not be the same
   obligation, and for an indexed family they plausibly differ.

**A Steward warning about the method, because it already fired once here.** The
first census run for this node grepped `crates/ken-elaborator/src/` for
`unmatched` -- the **spec's** vocabulary -- got zero hits, and concluded the
diagnostics were unbuilt. They are built, under the **implementation's**
vocabulary (`missing constructor`, `redundant`). **A census keyed on the
spec's words cannot see a mechanism the code spells differently**, and it fails
in the direction that manufactures work. Key the audit on the spec's
*obligations* and find the code that discharges them, not on the spec's nouns.

## Deliverables

**`D1` — the four tracker nodes.** One `docs/program/issues/` node per named
WP, each pointing at its existing frame in `docs/program/wp/`. Do **not**
rewrite the frames; they are good and this node is not a reframing.

**`D2` — the status of each, measured rather than assumed.** For each of the
four, report which of its frame's ACs are discharged by landed code, citing the
test or `file:line` that discharges each. Where an AC is discharged, the node's
status reflects it; where it is not, the node stays `draft` with the residual
named in one sentence.

**`D3` — the two open questions above, answered.** AC5's reject half: present
or absent, with the citation either way. And whether `missing constructor`
discharges `§4.1`'s "names the unmatched pattern witness" for an **indexed**
family, or whether the witness obligation is strictly stronger.

**`D4` — if `D3` finds a real gap, do not fix it here.** Name it in the
relevant slice's node and stop. This node's product is an accurate tracker.

## Acceptance criteria

**`AC-1` — every one of the four is on the tracker after this lands**, and
`scripts/gen-progress.sh` shows the dependent-constructor area. The failure this
node exists to close is *invisibility*, so the control is that the area appears
at all.

**`AC-2` — every status claim is backed by a citation, not by a grep count.**
A slice marked as landed names the tests that land it. A slice marked `draft`
names what is missing. **A bare "N tests exist" is not a discharge**; the
question is which AC each one discharges.

**`AC-3` — the audit is keyed on obligations, not on spec nouns.** State, for
each AC checked, the search you ran to look for its discharge. This is the
control on the failure mode recorded above.

**`AC-4` — no production change.** `crates/` is untouched. This is a tracker
and audit node; if it produces a code deliverable, that is a different node.

**`AC-5` — no-regression, in CI.** `COORDINATION §12` -- the venue is CI, never
a local `--workspace` run. With `AC-4` holding, this should be trivially green;
if it is not, something in `D1` touched code and that is the finding.

## Sizing

**`S`.** It is four small files plus a measured read of two test files against
two frames. The one-hour target applies, and **the audit is the work** -- if
`D2` is turning into a day, the residual is bigger than this node and the right
outcome is to report that and stop.

## Not this node

- **No reframing of the four WP frames.** They are detailed and their design
  content is sound; only their perishable state clauses are stale.
- **No implementation of any residual `D3` finds.** That is the relevant
  slice's node.
- **Not `SURF-gadt-field-sugar`'s scope question.** The spec explicitly defers
  named-argument/record-field constructor ergonomics; nothing here reopens it.
- Not the `34 §2.5` deferral, which is already filed as
  [[LANG-DECEQ-CHAR-LAWFUL-INSTANCES]] and is operator-gated.

## Why this is Language's next WP

Language has **no `ready` node** once `LANG-PRELUDE-COMBINATOR-BLOCK-DELTA`
merges, and its only two drafts -- [[LANG-DECEQ-CHAR-LAWFUL-INSTANCES]] and
[[LANG-FOREIGN-NAME-FORMAT-CHARS]] -- are both `gate: operator`. This node is
`gate: none` and depends on nothing, so it is releasable now. It also produces
the input needed to choose the *following* Language WP, which is currently
unknowable: the frontier of the dependent-constructor area cannot be named until
`D2` is done.
