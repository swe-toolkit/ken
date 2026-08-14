---
id: LANG-COMMENT-POPULATION-PARITY
title: "The B1 round-trip helper counts a comment population that production stopped using -- `assert_round_trip` filters `TriviaKind::LineComment` while `attach_comments` filters `is_comment()`, so the whole-`catalog/` walk is green only because no catalog source contains a block or doc comment, and the first author who writes one gets a red in a different crate accusing the attachment mechanism of losing a home"
status: active
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

**No catalog content change.** Whether `catalog/` should contain a block
comment — so that the corpus exercises a landed surface feature rather than
being green by absence — is a catalog authoring call, not Language's, and the
fixture control in the frame catches this defect deterministically without it.
Recorded here so the absence is a decision rather than an oversight.
