# WP frame — `LANG-FOREIGN-NAME-FORMAT-CHARS`

    owner   language       tier   T2        size   M
    depends none. The spec authority is landed on `main`.
    node    docs/program/issues/LANG-FOREIGN-NAME-FORMAT-CHARS.md
    review  Architect REQUIRED. Code merge → full CI + M8 Adversary.

## 1. Objective

Make a raw in-scope Unicode `Cf` codepoint a hard lexical error anywhere in Ken
source, on one guard that every entry path inherits structurally. Today the
lexer accepts one in a comment, in the token stream, and inside a string,
character, or byte-literal body, so `31 §1f` is normative and unenforced.

## 2. Fixed inputs

Base: `3de9a5030a91ab7133817157617c2705d4e1cfbd`. `lexer.rs` is blob-identical
(`104efc154`) here and at `994e5f95a`, `dcbc24c33` and `e8ab799b5`, where these
inputs were first measured.

`spec/30-surface/31-lexical.md §1f` (`:514`, landed `2d72bd7e7`) is the design
authority: the predicate is the closed category `Cf`, not an enumerated roster;
the error is hard; the guard sits on the decoded source codepoint before context
dispatch and upstream of escape decoding; U+FEFF as a byte-order mark at offset
0 is the sole exception. Operator, 2026-09-06: "concur. reframe as whole-source
lexical policy."

Measured at the base:

- A `git grep` over `crates/` for `202E`, `U+FEFF`, `0xFEFF`, `feff`,
  `is_format`, `GeneralCategory` and `Trojan` returns **zero** hits. No guard
  exists in any form.
- **Zero raw in-scope `Cf` codepoints occur in any `.ken` or `.ken.md` file.**
  The guard lands green with no corpus migration owed — and equally, **no
  existing Ken source witnesses it.** Every control below is an authored
  fixture; a suite that only re-checks the corpus proves nothing.
- **Two tracked NON-Ken files contain U+00AD and are out of scope.**
  `crates/ken-runtime/src/cranelift_backend/lowering/core/tests/constructors.rs`
  (one, in a Rust comment near `:2438`) and
  `docs/program/ds-campaign-judgment-log.md` (five, in prose). Neither is read
  by the Ken lexer. **Do not edit either file** — `constructors.rs` is
  concurrently owned by L1's active `RT-GRAFTED-SPINE-CONTROL-GRAPH`. A sweep
  finding them is this input reproducing, not a defect.
- **Two entry paths, one raw constructor.** `Lexer::lex` (`lexer.rs:1353`) and
  `Lexer::new` (`:310`) are both `pub`; `Lexer::new` has **exactly one caller in
  all of `crates/`** — `format.rs:95`, inside `canonicalize_lexed_tokens`. That
  is the formatter bypass: `literate.rs:235` routes an `Ignore`/`Reject` fence
  body there, and the loop swallows `ElabError::ParseError` at `format.rs:103`
  while the next arm, `Err(error) => return Err(error)`, propagates the rest.
- **`is_comment` is a complement, not a roster.** `lossless.rs:99` reads
  `!matches!(self, TriviaKind::Whitespace)`, so a newly added `TriviaKind`
  variant joins the comment class silently and reaches the attachment filters at
  `:411` and `:836`.
- `unicode-normalization` is a direct workspace dependency and `unicode-ident`
  is transitive in `Cargo.lock`. **Neither exposes a general-category
  predicate.**

Treat anchors as perishable. If a fixed input is false on the landed base, stop
and report the mismatch; do not build around it.

## 3. Scope

Expected to change: `crates/ken-elaborator/src/lexer.rs`, `format.rs`,
`literate.rs`, `lossless.rs`, and their tests.

The former "no `kenfmt` change" boundary was a defect I authored. It is
withdrawn: closing the formatter bypass is in scope and is required.

Boundaries that must not move:

- **The escape repertoire (`31 §3`) is untouched.** The rule restricts the raw
  spelling, never the data.
- **No identifier-class change.** Blessed Unicode letters (`§1e`,
  `SPEC-IDENT-BLESSED`) stay unimplemented; `§1f` covers them later on the same
  guard, which is a property of the placement, not work here.
- **No output change for input that was already lexically clean**, in either
  `kenfmt` route or in `parse_lossless`.
- **No `conformance/` change.** CV owns a separately authored conformance
  witness for `§1f`. Keep this candidate's diff inside `crates/`.
- No UCD table, escape semantics, public Ken package surface, primitive,
  postulate, or trust movement.

## 4. Deliverable

Architect ruling `evt_5wf4xp43ay9j2` fixes the mechanism. It is not an
implementer choice, and a per-context family of checks is ruled out by name.

1. An opaque, crate-private `ValidatedSource<'s>` in the lexer module. Its sole
   root constructor takes the **complete decoded source**, runs the closed `Cf`
   preflight exactly once, and returns `Result<ValidatedSource<'s>, ElabError>`
   carrying a **distinct typed** `RawFormatCharacter { character, span }`. No
   caller can manufacture the capability from an arbitrary suffix.
2. It retains the complete `&str` and an **absolute byte start**. A crate-private
   `suffix_from(cursor)` derives a view after a UTF-8 boundary check, preserving
   `absolute_start = cursor`. It never re-validates and never resets an origin.
3. `Lexer::new` takes `ValidatedSource`, and construction over unvalidated text
   is made impossible, not merely discouraged. `Lexer::lex(src)` validates once
   then roots a view; `canonicalize_lexed_tokens(src)` validates the complete
   fragment once **before** its recovery loop, then derives suffix views as
   `cursor` advances.
4. BOM consumption moves to construction from the validated view: skip U+FEFF
   only when `absolute_start == 0` and the view begins with it. Token spans stay
   relative to each view, so the recovery loop may keep adding `cursor`.
5. **No checks in the comment, token, string, character or byte-string arms**,
   and no pending error in a suffix lexer. Validation completes before any lexer
   exists.

`format_ken_md` rebases the typed error by `fence.body_range.start` on both the
source/example route and the ignore/reject recovery route, exactly as the
existing `ParseError` branch does. No fallback may convert it into recovery or
return an unrebased body-local span.

The accepted `0..3` BOM bytes are accounted for in the lossless partition as a
dedicated `TriviaKind::Bom`, recognized by `append_trivia` only at span `0..3`
at source start. **Not `Whitespace`** — U+FEFF is deliberately no longer Unicode
whitespace, and naming it whitespace would make the public lossless
representation lie. Repair `is_comment` so only real comment variants attach;
its current complement form would admit `Bom` silently.

Realize the category predicate without adding a third-party dependency. A
hand-typed roster of the codepoints `§1f` names as instances is a **wrong
implementation**, and AC-2 is written to catch it.

## 5. Acceptance

**AC-1 — whole-source coverage, through the real public routes.** A raw U+202E
rejects with the same error kind at the offending codepoint's offset in each of:
a line comment body, a nestable block comment body, the token stream, an
ordinary string literal body, and a **triple-quoted raw string body**. The raw
string is load-bearing: it performs no escape processing, so it cannot carry an
in-scope `Cf` at all, and it is the context a guard bolted onto the
ordinary-string decoder misses. Separately, through `format_ken_md` itself
and not a direct canonicalizer call: raw `Cf` in a comment body and in a string
body, inside both a `ken ignore` and a `ken reject` fence, returns
`RawFormatCharacter` for the first offender **at its exact document byte span**.
A deliberately non-parseable ignore/reject fragment carrying visible `\u{202E}`
escape data still reaches the recovery lexer and remains legal data.

**AC-2 — the predicate is the category, not `§1f`'s examples.** A `Cf` codepoint
that `§1f` never names — U+00AD SOFT HYPHEN, and one of U+0600 or U+061C —
rejects on the same guard, in a comment and in a string body. An implementation
built from the codepoints `§1f` lists passes AC-1 and **fails here**, which is
the point of the criterion. Name, in the candidate, where the category data
comes from and which Unicode version it is pinned to.

**AC-3 — the two boundary pairs, each on one codepoint.** Both halves of each
pair must be exercised.

- **Escape round-trip.** `"\u{202E}"` accepts and the decoded `String` contains
  exactly U+202E, unchanged from what a raw occurrence would have produced,
  while the same literal spelled with a raw U+202E rejects. A guard placed after
  escape decoding rather than upstream of it fails the accepting half.
- **BOM, proved through `parse_lossless` and `layout::format_ken`.** U+FEFF at
  offset 0 is consumed; the token stream agrees with the BOM-free source; token
  plus trivia pieces still form an exact `0..source.len()` partition that
  reconstructs the original bytes, with one `Bom` item at `0..3`; and U+FEFF at
  any later offset is the typed rejection. Two mutations must redden this
  control: making `suffix_from` reset its absolute origin, and bypassing root
  validation for the recovery loop.

## 6. Stop condition

Hand back, rather than working around, on any of:

- Realizing the `Cf` category predicate genuinely requires a new third-party
  dependency. That is a fork for the Steward to route, not a call to make inside
  the WP.
- A construction path that cannot be closed: some route to a `Lexer` over
  unvalidated text survives that you cannot remove within this scope. Report the
  route; do not compensate with per-context checks.
- Any fixed input above measuring false at the landed base.
