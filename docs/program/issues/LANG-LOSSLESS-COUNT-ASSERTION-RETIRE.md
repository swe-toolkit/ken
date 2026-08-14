---
id: LANG-LOSSLESS-COUNT-ASSERTION-RETIRE
title: "`assert_round_trip`'s comment-count assertion cannot fire -- production reconciles the same two sets and refuses first, so `kenfmt_b1_lossless.rs:27` states a theorem while its message reads as a live check, and the `pub fn is_comment` it is the sole external caller of exists only to feed it"
status: ready
owner: language
size: XS
gate: none
depends_on: []
blocks: []
github: null
origin: "Adversary hunt evt_6n7y2mzn83grn on the landed squash be8535b9, plus the Architect's non-blocking finding at evt_73bmxjfmkv7bq carried as a residual on LANG-COMMENT-POPULATION-PARITY. Triaged by the Steward 2026-08-14 as ONE node because the two findings are the same fact from opposite ends. Every load-bearing fact below re-verified by the Steward against main c4725c7c before filing."
---

## What this is

**Two findings that are one repair.** They arrived from different reviewers,
one turn apart, and neither reviewer could see the other's half.

- **The Architect** (`evt_73bmxjfmkv7bq`): `D3` of the parity node made
  `TriviaKind::is_comment` `pub`, and a census found **zero production
  consumers outside the module** — the only external caller is the integration
  test that needed it. The export's justification is not written down.
- **The Adversary** (`evt_6n7y2mzn83grn`): that same assertion **cannot fire.**

⇒ **The public-API widening exists to support an assertion that is a
restatement.** Fix either one alone and the other still reads as a live
concern; fix the assertion and the export follows for free.

## The measurement

`is_comment()` has exactly three call sites in the tree — verified at
`main c4725c7c`:

```
crates/ken-elaborator/src/lossless.rs:408      attach_comments's filter
crates/ken-elaborator/src/lossless.rs:798      validate_attachment_totality's filter
crates/ken-elaborator/tests/kenfmt_b1_lossless.rs:25   the widened helper
```

**The first two are the two sides `validate_attachment_totality` reconciles**,
and `lossless.rs:235` calls it as `validate_attachment_totality(&trivia,
&attachments)?` — so `parse_lossless` returns `Err` on any mismatch. The helper
at `:25` builds a **third** copy of the same filter over the same collection
and compares it against the output of that reconciliation.

**Measured by mutation, not argued.** The Adversary narrowed `attach_comments`
to the old filter — the production-side version of the very defect the parity
node fixed — and ran `D4`:

```
panicked at crates/ken-elaborator/tests/kenfmt_b1_lossless.rs:10:33:
block-and-doc: source must parse:
  Internal("comment attachment is not total: 2 comments, 0 unique homes, 0 attachments")
```

**Line 10 — the `parse_lossless` expect. Line 27 is never reached.**

⇒ **`:27`'s falsifiability was carried entirely by the disagreement the parity
node removed.** While the test-side predicate differed from production's, the
assertion measured a real relation and `D4`'s fixture failed it at `2 != 0`.
Now both operands are the same two sets production has already reconciled.

**This is not a defect in that repair, and the node must not be written as
one.** The parity node's value is measured and survives: the false red on
catalog authors is gone, and `D4`'s fixture drives a block and a doc-line
comment through the full byte round-trip and AST reparse — coverage the corpus
cannot provide. What is left is one assertion whose message,
*"every trivia item counted by `is_comment()` must have exactly one home"*,
reads as an independent check while stating a theorem.

## The design call, front-loaded — RETIRE, do not document

**Taken by the Steward. It is a test-topology and public-surface call, the same
class as the three calls the parity node front-loaded; the Architect reviews it
on the merge Decision like any other diff.**

The Adversary stated the counter-argument fairly rather than burying it:
**keeping `:27` is defence-in-depth against `validate_attachment_totality`
being deleted.** That is legitimate, and it is still not what should land.

1. **A check that cannot fail is the failure this arc exists to fight.** The
   parity node's `AC-3` enumerates six transpositions precisely because a
   control nobody has run against a mutation is not known to be a control.
   Leaving `:27` in place, documented as redundant, keeps a line that reads
   live to every future author who greps for what guards attachment totality.
2. **If defence-in-depth is wanted, `:27` is the wrong instrument for it.** The
   thing to guard is *that `parse_lossless` calls
   `validate_attachment_totality` at all* — a pin on that call, not a third
   copy of its filter in an integration test that only runs after it passes.
3. **`D4`'s fixture is what guards the population now**, and it is a real
   discriminating control: restoring the old filter reds it at
   `left: 2, right: 0`.

**No replacement control is owed, and do not build one.** There is no measured
gap here — production enforces the relation and returns `Err`. Adding a pin on
the `validate_attachment_totality` call site because it *could* be deleted is a
safety intuition, not a grounded constraint, and this node does not authorize
it.

## Deliverables

**D1 — drop the count comparison from `assert_round_trip`.** Delete
`kenfmt_b1_lossless.rs:22-31` (the `comment_count` binding and the `assert_eq!`
that consumes it). **Change nothing else in the helper.** Lines 9-20 — the byte
round-trip and the AST-reparse equality — are what the helper is for and stay
exactly as they are.

**D2 — revert the export.** `TriviaKind::is_comment` returns to `fn` from
`pub fn` in `src/lossless.rs`. **Verified writable before filing:** `TriviaKind`
is imported nowhere outside `src/lossless.rs` at `main c4725c7c` — the only
occurrences elsewhere are in comments and doc comments, never a `use` or a
value position. Nothing else changes.

**D3 — retire the two residuals that this node discharges.** Both sit on nodes
already merged, and both currently read as open obligations:

1. `docs/program/issues/LANG-COMMENT-POPULATION-PARITY.md` — the
   `Residual: D3's export has no recorded justification` section. Record that
   it is discharged **by retirement rather than by documentation**, and by
   which deliverable.
2. The same file's merge record references the export as carried. Leave the
   merge record's history intact; only the residual's live status changes.

**Editing the residual text is `D3`'s, not `D2`'s** — kept separate so the diff
attributes.

## Acceptance criteria

**AC-1 — the retirement is exercised, not merely asserted.** After `D1`,
re-run `D4`'s fixture (`block_and_doc_comments_are_counted_for_attachment`) and
confirm it still passes. **Then re-run the Adversary's mutation**: narrow
`attach_comments`'s filter at `src/lossless.rs:408` to
`kind == TriviaKind::LineComment`, confirm `D4` **still reds** at
`kenfmt_b1_lossless.rs:10` with the `comment attachment is not total` message,
and restore. **Report the failing text.** This is the whole argument for the
retirement being safe: the coverage that mattered is production's, and it is
still there.

**AC-2 — the export census returns zero, after the change.**
`git grep -n 'is_comment' -- crates/` must show exactly the two
`src/lossless.rs` call sites and nothing else. Report the output verbatim. A
third row means `D1` was incomplete.

**AC-3 — `-p ken-elaborator` green**, and `src/lossless.rs` is byte-identical
to `c4725c7c` **apart from `D2`'s single `pub`**. Report the diff of that file.

**AC-4 — no surviving text describes the retired assertion as a live check.**
Grep `crates/ken-elaborator/` for `must have exactly one home` and for
`is_comment` in comment text, and report one row each with what the text now
says. The parity node's own `AC-6a` existed for this exact shape — a warning
that outlives its cause reads as a live constraint — and this node must not
create a fresh instance of it while closing one.

**AC-5 — no-regression, in CI.** Green in CI on the candidate. Do **not** run a
local `--workspace` build; the venue is CI (`COORDINATION §12`).

## Contention

**None.** Language has no other node `ready` or `active`. The two Language
residuals still parked on `LANG-PRELUDE-ELABORATION-DEPTH` — `env.globals`
injectivity, and `trusted_base_labels`' two-namespace flattening — are in
`tests/lang_prelude_collections.rs`, which this node does not touch. **Do not
fold them in**; they travel together into whichever candidate next enters that
file, and this is not it.

Runtime is active on `RT-DYNAMIC-ARM-SCALAR-MERGE`, whose `crates/ken-elaborator`
scope is `compiler_driver.rs`, `prelude.rs` and `erasure.rs`. Intersection with
this node is empty.

## Not this node

- **No change to `attach_comments`, `validate_attachment_totality`, the `From`
  impl, the placement heuristic, or any fixture.** `D4`'s fixture stays exactly
  as it landed.
- **No new control**, pin, or oracle — see the design call above.
- **No widening or narrowing of any other export.** `is_doc_comment` was
  private before the parity node and stays private.
- No catalog content change.

## A nit the Adversary reported against itself, and it is not a deliverable

The `D2` module doc says a transposition *"fails exactly the row naming that
pair."* A transposition makes **two** rows wrong and only the earlier one runs,
so the sentence is a clause loose. The Adversary predicted this would make three
of the six pairs report identically, **ran it, and was refuted** — `assert_eq!`
prints both values and all six `left`/`right` pairs are distinct, so the message
identifies the pair uniquely. **Nothing diagnostic is lost and no change is
authorized here.** Recorded so the next reader of that sentence does not re-open
it.
