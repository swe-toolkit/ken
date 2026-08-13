---
id: LANG-TRIVIA-KIND-MAPPING-PIN
title: "`LANG-COMMENT-CLASSIFIER-SHARED` made scanner divergence unrepresentable and moved the surface one hop to `From<CommentKind> for TriviaKind`, which is now the sole place a classification becomes a behaviour -- the completeness axis is closed by the compiler but the per-arm mapping is asserted nowhere, and the one fixture that covers the block form is a configuration where the doc rule and the positional heuristic return the same answer, so a Block/DocBlock transposition compiles and reds nothing"
status: ready
owner: language
size: XS
gate: none
depends_on: [LANG-COMMENT-CLASSIFIER-SHARED]
blocks: []
github: null
origin: Adversary pass on 457c51ee at evt_2b00q5w5jzyd9, which identified the residual and explicitly declined to ship a witness without one read it had not run. The Steward ran that read at 424ab5da; it came back the way that makes the residual real, and produced the discriminating witness the finding lacked.
---

## What this is

`LANG-COMMENT-CLASSIFIER-SHARED` (`457c51ee`) closed the two-scanner divergence
structurally: both callers take `next` from one `classify_comment` return
(`lexer.rs:334`, `lossless.rs:302`) and neither re-scans, so there is no second
end to disagree with the first.

**The divergence surface did not vanish; it moved one hop.**
`From<CommentKind> for TriviaKind` (`lossless.rs:45-54`) is now the sole place a
classification becomes a behaviour.

- **The completeness axis is closed by construction.** The `From` impl is total
  with no `_` arm, so a new `CommentKind` variant is a compile error. This is
  the half that catches the realistic change, and it needs nothing.
- **The per-arm mapping is asserted nowhere.** No test in
  `crates/ken-elaborator/tests` names any `TriviaKind` variant. Swapping
  `Block` and `DocBlock` compiles.

## Why the existing fixture cannot catch it, measured rather than argued

`ac3_doc_comment_attaches_to_following_declaration_discriminatingly` covers the
block form and asserts the comment attaches `Leading` to the **following**
declaration. The Adversary flagged — correctly, and without guessing — that
whether this discriminates depends on the positional heuristic, which it had
not read.

**Read at `424ab5da`.** `attach_comments` (`lossless.rs:366-445`), for a comment
on its own line between two top-level sibling declarations:

- `same_line_after` is **false** — there is a `\n` before the comment.
- `common_home` is **None**. `smallest_common_enclosing` (`:760`) keeps only
  spans with `span.start <= left.start && right.end <= span.end`, and
  `node_spans` holds per-decl spans from `collect_decl_spans`. **No collected
  span encloses two top-level siblings**, and `root` is not in the vector — it
  is only the `unwrap_or_else` fallback.
- ⇒ falls through to `else if next.is_some()` → `(Leading, next_home)`.

The doc arm (`:406-417`) returns `(Leading, next_home)` for the same input.
**Identical placement, identical home.** ⇒ `ac3` is a fixture where the doc rule
and the positional heuristic agree, so it cannot tell which produced the
outcome, and a transposition leaves it green.

## The discriminating witness

**A doc block comment on the same line after a declaration, with a following
declaration.** There the two rules split:

| | placement | home |
|---|---|---|
| doc rule (`:406`) | `Leading` | the FOLLOWING declaration |
| positional (`:428`) | `Trailing` | the PRECEDING declaration |

`same_line_after` is what forks them, and the doc arm returns before ever
reaching that test.

This pins the mapping through the observable the formatter consumes rather than
through the enum name, so it needs no `TriviaKind` in the test surface.
(`TriviaKind` is `pub`, so a direct variant assertion is also available; prefer
the behavioural one — it fails for the reason that matters.)

## There are TWO transposable pairs, not one, and the fixture doubles for free

Adversary widening at `evt_7nzmq49v193z6`, from a surface-wide grep. The
doc-attachment rule keys on `TriviaKind::is_doc_comment` (`lossless.rs:56-65`),
which matches **both** `DocLineComment` and `DocBlockComment`. ⇒ the `From` impl
has two pairs whose transposition flips attachment:

- **`Block ↔ DocBlock`** — the `{--` witness above.
- **`Line ↔ DocLine`** — the identical shape with `---` and no closer, and
  **nothing in the suite reddens on it.**

Measured over the whole test surface at `7ed80cdf`: there are exactly five
`CommentPlacement` assertions, in two files. `Trailing` is asserted for exactly
one comment form — `--`, at `kenfmt_b1_lossless.rs:59`. **No doc form is ever
asserted at any placement other than `Leading`**, and that one `Leading` is
`ac3`'s agreeing configuration. Of thirty `{--` fixtures, none is the
discriminating shape; the only same-line-after fixture anywhere is
`lang_surface_block_comments.rs:49`, a **line** comment inside the lex-agrees
corpus rather than a placement assertion.

⇒ `Line ↔ DocLine` is **half-pinned**: a transposition would make a plain `--`
attach as documentation and `("-- trailing", Trailing)` would redden, so the
plain direction is caught and **the doc direction is not.**

**Both rows cost one loop iteration.** `ac3` already iterates
`[("---", ""), ("{--", " --}")]`; this node applies that same pair to the
discriminating configuration instead of the agreeing one. Two rows pin all four
doc-relevant arms.

> **Provenance, so a later reader weights it correctly.** The `Block ↔ DocBlock`
> half rests on a Steward read of `attach_comments` at `424ab5da`. The
> `Line ↔ DocLine` half is **derived from that same structure and was not
> independently executed** — the Adversary was explicit about this. `AC-2`
> requires the mutation to red for **each** pair, which is what converts the
> derivation into a run.

## Severity, honestly

**Low, and it does not grow.** The compiler closes the half that catches a new
comment form. What remains is a transposition, which is a rarer edit — but it is
exactly the *"disagree about kind, which the formatter turns into a changed
attachment"* harm the predecessor's finding named, arriving at the one seam that
node created. It is one test.

**Filed rather than folded.** It was twice described as a rider on Language's
next node in this crate; `LANG-PRELUDE-COLLECTIONS` touches `prelude.rs`,
`l3a_acceptance.rs` and `error.rs`, and adding `lossless.rs` to it would be a
fourth file for an unrelated concern. A rider with no suitable host is an
obligation recorded in prose and owned by nobody — the failure mode this arc has
twice been about.

## Deliverables and acceptance

- **D1** — one test asserting same-line doc-comment attachment for **both** doc
  forms: placement `Leading` and home the **following** declaration. Iterate
  `[("---", ""), ("{--", " --}")]`, the pair `ac3` already uses, applied to the
  same-line-after configuration.
- **AC-1** — the test passes at current `main`, for both rows.
- **AC-2 — each transposition reds it, separately.** Run the mutation twice:
  swap `Block`/`DocBlock` in `From<CommentKind> for TriviaKind` and show the
  `{--` row failing; restore; swap `Line`/`DocLine` and show the `---` row
  failing; restore before landing. **Report both.** A single combined run does
  not distinguish "both pinned" from "one pinned twice", and the `Line`/`DocLine`
  half of this frame is derived rather than executed — this AC is what runs it.
  **If a row does not red, its fixture is not the discriminating configuration**
  and the witness was mis-transcribed; say so rather than adjusting the mutation.
- **AC-3 — `ac3` and `kenfmt_b1_lossless.rs:58-60` still pass unchanged.** No
  assertion amended in either. Adding is fine.
- **AC-4 — no new red in CI.** Targeted locally: `-p ken-elaborator`.

## Not this node

- Changing the `From` impl, the classifier, or any comment semantics. This
  pins existing behaviour; it does not alter it.
- The positional heuristic itself, or any other placement it produces.
- Formatter output.
