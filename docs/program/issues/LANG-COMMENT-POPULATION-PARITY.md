---
id: LANG-COMMENT-POPULATION-PARITY
title: "The B1 round-trip helper counts a comment population that production stopped using -- `assert_round_trip` filters `TriviaKind::LineComment` while `attach_comments` filters `is_comment()`, so the whole-`catalog/` walk is green only because no catalog source contains a block or doc comment, and the first author who writes one gets a red in a different crate accusing the attachment mechanism of losing a home"
status: merged
owner: language
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: Adversary hunt on the landed squash `b233ba68` (`LANG-TRIVIA-KIND-MAPPING-PIN`), evt_3c8m2serregjw, findings 1-3 with a run repro. Triaged by the Steward 2026-08-14 as one node rather than three, because the obvious repair to the live defect silently deletes the only pin the other two findings depend on. The expressibility fact in the frame's design section is the Steward's own measurement at `2ca91a3a`.
---

## The defect

`crates/ken-elaborator/tests/kenfmt_b1_lossless.rs`, in `assert_round_trip`,
counts the comments it expects to find homes for by filtering trivia on
`item.kind == TriviaKind::LineComment`. Production attaches on
`kind.is_comment()` (`crates/ken-elaborator/src/lossless.rs`, in
`attach_comments`), which is every non-whitespace kind.

The two sides have disagreed since `LANG-SURFACE-BLOCK-COMMENTS` widened the
enum. Nothing reddened, because the corpus the helper walks contains no block
or doc comment at all.

**The production predicate's own doc comment states the hazard the test
embodies.** `is_comment()` carries, verbatim: *"widening this, and not just the
doc kinds, is what keeps `attach_comments`/`validate_attachment_totality` from
silently losing block comments the way an unwidened `LineComment`-only filter
would."* The test-side filter is that unwidened filter.

**Reproduced by the Adversary**, appending one `{- -}` comment to a real
catalog source and reverting:

```
panicked at crates/ken-elaborator/tests/kenfmt_b1_lossless.rs:27:5:
  …/ProofErasureBoundaryChecker.ken: every comment must have exactly one home
  left: 8
 right: 7
```

**The diagnosis the failure gives is false and points away from the cause.**
The message names attachment totality; the attachment mechanism is correct.
The first person to write a block comment in `catalog/` is sent to the wrong
crate.

## Why this is one node and not three

The Adversary reported three findings and declined to propose a repair for any
of them, on the ground that two of them interact. They do:

**The obvious repair to the defect above deletes the only variant-level pin in
the tree.** That `TriviaKind::LineComment` filter is the sole place in any
crate that names a comment variant by name. It is therefore the only thing that
reddens when `CommentKind::Line` and `CommentKind::Block` are transposed in the
`From` impl — measured, by running that mutation. Widening it to the production
predicate is correct, and it makes the `Line`/`Block` transposition free.

Whoever fixes the live defect would be removing a pin, with nothing telling
them so. That is the reason to frame the two together and to order them.

## What is already known about the pin's reach

`LANG-TRIVIA-KIND-MAPPING-PIN` landed a header claiming *"All four
`CommentKind` arms are pinned in that configuration, one row per arm."*

**Measured false for one pair, by running the mutation.** Transposing
`DocLine`/`DocBlock` in the `From` impl leaves 169 tests green across eleven
targets, including the pin file itself at 2/2. The two `d1` rows both assert
`Leading` with the same home because both flow through the same
`is_doc_comment()` branch, so their conjunction establishes class membership,
not the arm.

⇒ Four arms admit six transpositions. The node's mutation evidence covered the
two cross-class ones. Of the two within-class ones, `DocLine`/`DocBlock` is
free and `Line`/`Block` is caught only incidentally.

## Not this node

**No change to `attach_comments`, to the `From` impl, or to any placement
rule.** Both are correct; the Adversary checked and so did the Steward. This
node changes what the tests count and what the pin file claims.

## Merged 2026-08-14

**Candidate `0580a7449c9d2011681fd7e7a89e13636076848b`, landed as squash
`be8535b9`** (PR #2158, CI green; Decision `dec_72wd6gwaqmg4n`, Architect,
read `resolved` from the object). Merge-base `bc62216a`, derived independently
and matching the declared value; three commits, four paths, `+80/-31`; **4/4
blobs verified identical after landing.** Both SHAs are recorded because a
squash rewrites the candidate, so it is never an ancestor of `main` — ask
content, not ancestry.

**`AC-6a` verified on the landed tree.** The `LOAD-BEARING SHAPE` block is gone
from `kenfmt_b1_lossless.rs`, and neither phrase the AC forbids survives
anywhere in it: no prohibition on changing the fixture's configuration, no
claim that the pin file cannot re-derive an arm. That AC existed because a
warning outlives its cause silently, and the surviving clause was aimed at
exactly the author `D4` invites into that file.

### The sequencing is the part worth reusing

`cdee99a6` lands the in-crate four-arm pin **first**, `a9c209bc` widens the
filter and corrects the `D5` claim, and `0580a744` only then retires the
superseded text. **There is no intermediate commit where the `Line` arm is
unpinned** — the replacement is fully in place before the thing it replaces is
removed. The natural order is to delete the stale comment while already editing
that file, and that order is the one that leaves a window.

### What the Architect retracted, and why it generalises

They had required the `LOAD-BEARING SHAPE` note two reviews earlier, and praised
it for saying *"nothing reds."* Their own reading of the miss: **the right
question was not *"how do we protect this fragile coupling?"* but *"why does the
coupling exist?"*** Nobody, reviewer included, asked why the pin had to be an
*integration* test — and once asked, an in-crate `#[cfg(test)]` module beside the
`From` impl names all four arms and the coupling dissolves. `D5`'s new header
carries the distinction forward: the old claim was **unachievable**, not merely
unproven, so a future author reading *"this file doesn't pin the arms"* does not
go rediscover the wall.

**`D4` is a discriminating pair, not a coverage bump** — the fixture carries one
block and one doc-line comment and **no** `Line`, so restoring the old filter
reds it at `left: 2 right: 0`. A fixture containing a `LineComment` would have
passed under both filters. The population claim beside it was checked
independently: grepping `catalog/` for `{-` and leading `---` returns zero files.

## Residual (DISCHARGED): `D3`'s export had no recorded justification

> ### DISCHARGED BY `LANG-LOSSLESS-COUNT-ASSERTION-RETIRE` D1/D2, BY
> ### RETIREMENT, NOT DOCUMENTATION. See [[LANG-LOSSLESS-COUNT-ASSERTION-RETIRE]].
>
> **The repair below — "one clause on the method" — was superseded, not
> applied.** An Adversary hunt on this node's own squash (`evt_6n7y2mzn83grn`)
> measured the other end of the same fact: **the assertion that `pub` existed
> to serve could not fire.** `attach_comments` and `validate_attachment_totality`
> are the two sides `parse_lossless` reconciles at `src/lossless.rs:235` with a
> `?`, so the helper's third copy of the filter compared two sets production had
> already agreed on. Narrowing `attach_comments` reddened at
> `kenfmt_b1_lossless.rs:10`, never reaching the count assertion.
>
> ⇒ **The export and the assertion were one item.** Documenting the export
> would have written a justification for a line that stated a theorem.
> `LANG-LOSSLESS-COUNT-ASSERTION-RETIRE` D1 deleted the count comparison and D2
> reverted `is_comment` to private, closing both. No replacement control was
> added.
>
> **CORRECTED 2026-08-14 — the sentence that stood here named the wrong guard,
> and it was the Steward's.** It read: *"`D4`'s fixture
> (`block_and_doc_comments_are_counted_for_attachment`) is what guards the
> population now."* **False, and false in the direction that gets a control
> deleted.** Architect measurement at `evt_tt31sfem2vnw`.
>
> **Two different mutations, and only one of them is caught.**
>
> | mutation | caught? | by what |
> |---|---|---|
> | narrow **`attach_comments`'s filter** at `:408` | yes | production — the two sides now disagree, and `D4` reds at `kenfmt_b1_lossless.rs:10` |
> | narrow **`is_comment` itself** | **no** | nothing here. Both `:408` and `:798` move together, so `validate_attachment_totality` compares a set against itself-mapped and is **invariant** under it. `D4`'s fixture then yields zero comments, zero attachments, a byte-exact round-trip and an unchanged AST — **green** |
>
> **The real guards against the second are FOUR sites, in two files no frame in
> this arc mentioned.** Adversary measurement at `evt_1ezcaqqqd62ge`, which ran
> the mutation across every `ken-elaborator` binary reaching a block or doc
> fixture:
>
> | site | test | what its failure says |
> |---|---|---|
> | `tests/lang_surface_block_comments.rs:121` | `ac3` (both `---` and `{-- --}`) | an attachment record exists |
> | `tests/lang_trivia_kind_mapping_pin.rs:59` | `d1` | an attachment record exists |
> | `tests/lang_trivia_kind_mapping_pin.rs:100` | `d2`, ordinary `{- note -}` | an attachment record exists |
> | **`tests/lang_surface_block_comments.rs:253`** | **`ac7_formatter_round_trips_a_block_comment`** | **the comment's TEXT survives `format_ken`** |
>
> ⇒ **`D4` guards the coupling between `attach_comments`'s filter and its own
> output. Those four guard the population.** Do not treat them as
> interchangeable: an author tidying either of those two files would have read
> the old sentence, believed `D4` covered them, and deleted the only controls
> that do.
>
> **Separate MECHANISM guards from CONSEQUENCE guards — the correction above
> was itself short by one, and the missing one is the only consequence guard.**
> The first three assert that an internal record exists. `:253` asserts
> `out1.contains("{- leading -}")` — that the user's comment is still in the
> formatter's output. Under this mutation `ken fmt` **silently deletes a block
> comment**, which is the harm this whole arc exists to prevent, and exactly one
> assertion in the tree catches it. The three mechanism guards read as a
> complete list *because they are all the same shape*, and the shape is what
> makes them greppable; a `contains` on formatter output is invisible to the
> search that found them.
>
> **And the three mechanism guards are SETUP LINES, not assertions.** All three
> are `.find(...).unwrap_or_else(|| panic!("… must have an attachment"))`,
> running before the real assertions of a test that is about placement and home
> spans. **Attachment existence is not what any of those tests checks — it is
> the lookup each does first**, so an author rewriting the lookup for an
> unrelated reason removes the guard without touching an assertion. That is the
> same incidental-guard shape this arc has twice now retired. `:253` is the only
> one of the four whose failure message names the property it protects.
>
> **Bound on the sweep, stated rather than implied:** other crates were not run.
> `ken-cli`'s formatter and documentation-gate suites run `ken fmt` over
> `catalog/`, and no catalog source contains a block or doc comment, so nothing
> is expected there — but it is unmeasured.
>
> **The generalizable form, from the same measurement:** an assertion whose
> expected value is computed from the thing it is testing is a theorem, not a
> check — and here that defect occurred at **two layers at once**, in the
> retired test and in the production validator underneath it.

Architect finding, non-blocking, at `evt_73bmxjfmkv7bq`, on the approved
candidate. **Not a defect in what landed** and not a reason to reopen the node.

**`D2` states a rule and `D3` takes the other branch, one file apart.** `D2`'s
own comment says *"in-crate because `CommentKind` is `pub(crate)` and no
integration test can name it"* — when an integration test cannot reach
something, move the test, not the boundary. `D3` meets the identical obstacle
for `is_comment` and moves the boundary, `fn` to `pub fn`.

**Censused at the approved SHA: zero production consumers outside the module.**

```
src/lossless.rs:96              pub fn is_comment(self) -> bool {
src/lossless.rs:408                 .filter(|item| item.kind.is_comment())
src/lossless.rs:798                 .filter(|item| item.kind.is_comment())
tests/kenfmt_b1_lossless.rs:25      .filter(|item| item.kind.is_comment())
```

The only external caller is the integration test that needed it, and the
method's doc comment still describes **internal** callers
(`attach_comments`/`validate_attachment_totality`). So a reader asking *"why is
this `pub`?"* finds an answer about `attach_comments`, and the export's only
visible cause is a test's reach.

**The asymmetry is defensible and the point is that nobody wrote it down.**
Exporting `CommentKind` would publish a lexer internal; exporting one predicate
on an already-public enum, in a module whose own docstring names *"formatter and
source-tooling clients"* as its audience, is a different act. **The repair is
one clause on the method stating the caller-facing contract** rather than the
internal one.

**Not filed as a node**, and explicitly **not** the alternative repair — moving
`assert_round_trip`'s totality check in-crate and reverting the `pub` is a
larger change for a smaller gain, and the Architect said as much. It rides the
next Language candidate that enters `src/lossless.rs`.

## Not this node, continued

**No catalog content change.** Whether `catalog/` should contain a block
comment — so that the corpus exercises a landed surface feature rather than
being green by absence — is a catalog authoring call, not Language's, and the
fixture control in the frame catches this defect deterministically without it.
Recorded here so the absence is a decision rather than an oversight.
