---
id: RT-FORCED-RETURNS-BOUND-RESTORE
title: "One clause: the six ignored tests' eighteen empty returns are FORCED by the refusal, not observed -- the measurement the Architect barred from this comment came back one repair later, and it now sits one sentence above the 81-return tally a reader will add it to"
status: merged
owner: runtime
size: XS
gate: none
depends_on: [RT-IGNORED-CORPUS-MEMBERSHIP-RULE]
blocks: []
github: null
origin: "Steward, 2026-08-16, on Adversary hunt evt_6kf03g1fv60qw against merged range 63644c71d..f2f20703c (PR #2389). The defect's proximate cause is this Steward's own AC-2 on the predecessor node, which is recorded below rather than elided. Steward-filed per COORDINATION section 2."
---

> # THE STEWARD'S `AC-2` CAUSED THIS. Stated first so the repair is not read as the ring's error.
>
> `RT-IGNORED-CORPUS-MEMBERSHIP-RULE` `AC-2` said *"the six keep their
> credit — the comment must still record that the six were measured
> individually."* **The ring discharged it exactly as written.**
>
> **But two merges earlier the Architect had ruled that same measurement OUT of
> this same comment** (`evt_29828sqq22195`), and the `AC` did not carry the
> ruling forward. **The candidate is correct against its frame; the frame was
> wrong.**

## The defect: a barred measurement returned, minus the bound that makes it honest

**Landed on `main` at `4f1592615`**, in
`crates/ken-runtime/src/cranelift_backend/lowering/core/tests/control.rs`:

> *"Of those, the six closure-at-boundary tests were measured individually:
> **each produced three completed returns with
> `unit_boundary_environment_fields={}`** before its expected `Closure`
> refusal."*

**Two independent reasons that sentence cannot stand as written:**

1. **The Architect excluded exactly this content from exactly this comment.**
   `evt_29828sqq22195`: those returns *precede* the attempted crossing, and the
   crossing that would populate the field is what gets refused, so folding them
   in *"raises the tally while weakening what it means."* **The comment before
   PR #2389 did not mention them. PR #2389 added them.**
2. **The bound established one merge later is missing.** Adversary
   `evt_6d81evnk2nyfn`, accepted and recorded in
   [[RT-BOUNDARY-IGNORED-CORPUS-MEASURE]]: *"all six agreed"* is **forced, not
   observed**, because those returns structurally cannot populate the field.

> ### POSITION MAKES IT WORSE THAN CONTENT ALONE
>
> The sentence sits **one sentence above** *"returned
> `unit_boundary_environment_fields={}` on all 81 completed returns."* ⇒ a
> reader naturally tallies **18 + 81 = 99 confirmations.** The eighteen are
> **one structurally-determined outcome repeated eighteen times**, and they are
> drawn from precisely the tests that cannot exhibit the shape.
>
> **`"before its expected `Closure` refusal"` states the SEQUENCE, not the
> IMPLICATION.** It tells a reader the refusal came after; it does not tell them
> the refusal is **why** the set was empty. **That gap is the whole content of
> the finding.**

## `D0` — one clause, and it is already written

**The correction currently lives in a node file readers have no reason to
open.** Move its substance to where the claim lives:

> *"…before its expected `Closure` refusal — **an outcome that refusal makes
> forced rather than observed.** Six assumed behaviours became six measured
> ones; **the shape-bearing population remains unmeasured.**"*

**Alternatively, delete the sentence**, which is what the Architect's ruling
strictly implies. `D0` may take either route; **the bound-and-keep version is
preferred** because the individual measurement did convert six assumptions into
six measurements, and that is worth recording once it cannot be miscounted.

## Acceptance criteria

**`AC-1`. A reader cannot add the eighteen to the eighty-one.** After the
change, the text must make the eighteen returns' emptiness **forced** on its
face — not merely sequenced before a refusal. **Check by reading the paragraph
top to bottom and asking what number a reader walks away with.**

**`AC-2`. No new measurement, and no re-measurement.** This is a clause. Do not
re-run the corpus, do not add returns, do not touch any `#[ignore]`.

**`AC-3`. The membership rule from `RT-IGNORED-CORPUS-MEMBERSHIP-RULE` stays
exactly as landed.** That repair was correct and is not reopened — the count 33
stays bounded to exact `c88a5e423` rather than stated as the rule.

**`AC-4`. Do not restore the `"fails at base 21fd46dc"` pin.** Carried forward
from the predecessor; the perishable clause is the present-tense one.

**`AC-5`.** No-regression, in CI (`COORDINATION §12`). Local validation
targeted only — `-p ken-runtime`, never `--workspace`.

## Banned scope

- **Re-running or extending the corpus measurement.**
- **Reopening the membership rule** — see `AC-3`.
- **Editing any `#[ignore]` attribute or its reason string.**

## Sequencing

**Immediate, and it should not wait behind larger runtime work.** It is a
one-clause correction to a comment on `main` that currently overstates its own
evidence, and every day it stands is a day a reader can take the tally at face
value.

## Provenance

Adversary `evt_6kf03g1fv60qw`, hunting the merged range `63644c71d..f2f20703c`.
**The generalizable point, which is why this node exists rather than a silent
fixup:** *both nodes edited this one comment, and the second did not inherit the
first's ruling* — which is how a deliberately-excluded measurement returns one
repair later, with nobody having decided to reinstate it.
