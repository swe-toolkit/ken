---
id: LANG-POW10-CASCADE-LITERAL-CLAUSE
title: "The pow10 generator's own doc comment says every branch is a concrete literal, which its own recursion refutes in the same way the elab.rs copy did -- but its conclusion rests on a DIFFERENT and TRUE property (no saturating/min/clamp anywhere in the generated cascade), so this is a wording repair on a sound argument, not a second false justification"
status: merged
owner: language
size: XS
gate: none
depends_on: []
blocks: []
github: null
origin: "Architect scope hand-off on the LANG-REFINED-FALLBACK-COLDNESS-CLAIM merge verdict evt_3w84rh9jdg981 / evt_5jmye3pdj3ra7, explicitly named as a scope call for the Steward rather than an Architect ruling. Filed by the Steward rather than carried, because a carry with no home is what evaporates. Re-verified against main 5edd3de3: decimal_char.rs:60-62 read directly."
---

> # RELEASED 2026-08-14 AS A PASSENGER ON [[LANG-STACK-ARC-EVIDENCE-USABILITY]].
>
> **This is the "ride it with the next candidate" the Sizing section calls
> for.** Both nodes touch `crates/ken-elaborator` and land as one candidate.
> **It is not a deliverable of that node** — it has its own `D1` and its own
> `AC-1`-`AC-4`, and it is reviewed on its own terms.
>
> **If this one grows past `XS`, that is the finding** and it comes off the
> candidate rather than delaying it. See Sizing.

## What this is

**A one-clause wording repair, and it is deliberately sized `XS` so it is not
mistaken for the node it resembles.**

`crates/ken-elaborator/src/decimal_char.rs:60-62`:

> *"Because **every branch here is a concrete literal** (never
> `saturating_*`/`.min(_)`/`clamp`), the align path this feeds is
> exact-or-stuck, never wrong (Architect's hard-gated condition 1)."*

**The same generator, twelve lines below, refutes the literal half** — at each
of the 31 levels the `True` arm is a bare literal and the `False` arm is the
next nested `match`, and the innermost `False` arm is `{unbounded_name} k`, an
application.

## WHY THIS IS NOT A SECOND `LANG-REFINED-FALLBACK-COLDNESS-CLAIM`

**Read this before sizing the work, because the resemblance is the trap.**

`LANG-REFINED-FALLBACK-COLDNESS-CLAIM` repaired a clause whose **conclusion
depended directly on the property that was false.** The coldness argument was:
*the arms are all bare literals, therefore the cheap unrefined attempt always
resolves them, therefore the refined fallback is never entered.* Break the
premise and the safety argument for a `-3120` saving goes with it.

**Here the conclusion rests on a different property, and that property is
TRUE.** The load-bearing content is the **parenthetical**, not the clause it
qualifies: *no `saturating_*`, no `.min(_)`, no `clamp` anywhere in the
generated cascade.* That is what makes the align path **exact-or-stuck rather
than wrong**, and it holds regardless of whether an arm is a literal or a nested
`match`. **The argument is sound; only its stated reason is over-broad.**

⇒ **Separate the conclusion from the warrant.** The warrant here survives
untouched. This node fixes the sentence so a reader does not inherit the
over-broad form a third time — it does **not** reopen the hard gate.

**The provenance claim is NOT established.** It is plausible that the `elab.rs`
clause was inherited from this one, and the shapes match. **Nobody has traced
it**, and this node does not assert it. Do not write "this was the source" into
the repair.

## Deliverable

**`D1` — restate the clause so it says what actually holds.** The generated
cascade contains no lossy or clamping operation — no `saturating_*`, no
`.min(_)`, no `clamp` — at any level, whether an arm is a `10^k` literal, the
next nested `eq_int` match, or the final `unbounded_name` application.
Therefore the align path is exact-or-stuck, never wrong.

**Do not weaken it to "mostly literals",** and do not delete the parenthetical:
the parenthetical is the part that was always true and always load-bearing.

## Acceptance criteria

**`AC-1` — the repaired clause is checked against `pow10_cascade_body`'s
recursion**, in the same file, and the review says so. Both defects in this
family arose from restating a justification without re-reading the code twelve
lines away.

**`AC-2` — the hard-gated condition is NOT reopened.** *"Architect's hard-gated
condition 1"* stands; `exact-or-stuck, never wrong` stands. This node changes a
sentence, not a guarantee. If the repair appears to require weakening either,
**stop and report** — that would mean the warrant is not what this node says it
is, and that is an Architect question.

**`AC-3` — comment text only.** No change to `pow10_cascade_body`,
`pow10_literal`, `MAX_SHIFT`, or any generated source. Diff should be one doc
comment.

**`AC-4` — no-regression, in CI.** `COORDINATION §12` — the venue is CI, never a
local `--workspace` run. Build targeted, `-p ken-elaborator`.

## Sizing

**`XS`.** One doc comment in one file. **If it grows past that, the growth is
the finding** — it would mean the exact-or-stuck argument depends on something
other than the absence of clamping operations, which is a report to the Steward
and a question for the Architect, not extra work to absorb.

**Ride it with the next candidate that touches `ken-elaborator`** rather than
spending a ring turn on it alone. It does not need its own release.

## Not this node

- **Not tuning `MAX_SHIFT`** and not touching the generated cascade.
- **Not a re-derivation of the align path's exactness.** The hard gate is
  settled; see `AC-2`.
- **Not a provenance investigation** into whether the `elab.rs` clause was
  copied from here. Interesting, unestablished, and worth nobody's turn.
