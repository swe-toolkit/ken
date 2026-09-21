# WP frame — `LANG-FOREIGN-NAME-FORMAT-CHARS`

    owner   language       tier   T2        size   M
    depends none. The spec authority is landed on `main`.
    node    docs/program/issues/LANG-FOREIGN-NAME-FORMAT-CHARS.md
    review  Architect REQUIRED. Code merge → full CI + M8 Adversary.

## 1. Objective

Make a raw in-scope Unicode `Cf` codepoint a hard lexical error anywhere in Ken
source. Today the lexer accepts one in a comment, in the token stream, and
inside a string, character, or byte-literal body, so `31 §1f` is normative and
unenforced.

## 2. Fixed inputs

Base: `dcbc24c33231f0529e5e975dc21597fbb7d6a5ab`. `lexer.rs` is blob-identical
at this base and at `e8ab799b5`, where these inputs were first measured.

`spec/30-surface/31-lexical.md §1f` (`:514`, landed `2d72bd7e7`) is the design
authority and settles the policy: the predicate is the closed Unicode general
category `Cf`, not an enumerated roster; the error is hard, not advisory; the
guard sits on the decoded source codepoint before context dispatch and upstream
of escape-decoding; U+FEFF as a byte-order mark at source offset 0 is the sole
exception. Operator, 2026-09-06: "concur. reframe as whole-source lexical
policy."

Measured at the base:

- The lexer is `crates/ken-elaborator/src/lexer.rs`. A `git grep` over `crates/`
  for `202E`, `U+FEFF`, `0xFEFF`, `feff`, `is_format`, `GeneralCategory` and
  `Trojan` returns **zero** hits. No guard exists in any form.
- **Zero raw in-scope `Cf` codepoints occur anywhere in tracked source.** So the
  guard lands green with no corpus migration owed — and equally, **no existing
  file witnesses it.** Every control below is an authored fixture, and a suite
  that only re-checks the corpus proves nothing.
- **`advance` is not the whole-source chokepoint.** `cur` (`:318`) and `advance`
  (`:322`) decode one `char` at `self.pos`, but `skip_ws_comments` (`:341`)
  delegates to the free function `classify_comment` (`:236`) and its end
  scanners, which walk `src` by byte offset with `starts_with` and assign
  `self.pos = next`. **A comment body is therefore never decoded through
  `advance`**, and a guard placed only there leaves `§1f`'s first-named context
  open. `lossless.rs::append_trivia` rescans with the same classifier.
- `unicode-normalization` is a direct workspace dependency and `unicode-ident`
  is in `Cargo.lock` transitively. **Neither exposes a general-category
  predicate.**

Treat anchors as perishable. If a fixed input is false on the landed base, stop
and report the mismatch; do not build around it.

## 3. Scope

Expected to change: `crates/ken-elaborator/src/lexer.rs` and its tests.

Boundaries that must not move:

- **The escape repertoire (`31 §3`) is untouched.** The rule restricts the raw
  spelling, never the data.
- **No identifier-class change.** Blessed Unicode letters (`§1e`,
  `SPEC-IDENT-BLESSED`) stay unimplemented; `§1f` is written to cover them later
  on the same guard, which is a property of the placement, not work here.
- **No `kenfmt` change.** `§1f` names a formatter auto-escape as the inverse of
  this rule; that is a separate deliverable and folding it in is a scope
  expansion.
- **No `conformance/` change.** CV owns a separately authored conformance
  witness for `§1f`. Keep this candidate's diff inside `crates/`.

## 4. Deliverable

One guard that dominates every codepoint read of the source, rejecting the first
raw in-scope `Cf` with a lexical error that carries its byte offset, plus the
fixtures that exercise it. Establishing which point actually dominates every
read — given the comment-scanner bypass measured above — is the first step of
the work, not a separate node.

Realize the category predicate without adding a third-party dependency. A
hand-typed roster of the codepoints `§1f` names as instances is a **wrong
implementation**, and AC-2 is written to catch it.

## 5. Acceptance

**AC-1 — whole-source coverage, including the two contexts a per-context patch
forgets.** A raw U+202E rejects in each of: a line comment body, a nestable
block comment body, the token stream, an ordinary string literal body, and a
**triple-quoted raw string body**. All five report the same error kind at the
offending codepoint's offset. The raw-string case is load-bearing: `§1f` states
that a raw string performs no escape processing and so cannot carry an in-scope
`Cf` at all, and it is the context a guard bolted onto the ordinary-string
decoder misses.

**AC-2 — the predicate is the category, not `§1f`'s examples.** A `Cf` codepoint
that `§1f` never names — U+00AD SOFT HYPHEN, and one of U+0600 or U+061C —
rejects on the same guard, in a comment and in a string body. An implementation
built from the codepoints `§1f` lists passes AC-1 and **fails here**, which is
the point of the criterion. Name, in the candidate, where the category data
comes from and which Unicode version it is pinned to.

**AC-3 — the two boundary pairs, each on one codepoint.** Both halves of each
pair must be exercised:

- **Escape round-trip.** `"\u{202E}"` accepts and the decoded `String` contains
  exactly U+202E, unchanged from what a raw occurrence would have produced,
  while the same literal spelled with a raw U+202E rejects. A guard placed after
  escape-decoding rather than upstream of it fails the accepting half.
- **BOM.** U+FEFF at source offset 0 is consumed as a byte-order mark and the
  file lexes to the same token stream as the identical file without it, while
  U+FEFF at any later offset rejects.

## 6. Stop condition

Hand back, rather than working around, on any of:

- Realizing the `Cf` category predicate genuinely requires a new third-party
  dependency. That is a fork for the Steward to route, not a call to make inside
  the WP.
- No single point dominates every codepoint read, so `§1f`'s one-guard placement
  is unreachable without restructuring the lexer's comment path. Report the
  mechanism; do not substitute a per-context family of checks, which `§1f`
  rejects by name because a later context would not inherit it.
- Any fixed input above measuring false at the landed base.
