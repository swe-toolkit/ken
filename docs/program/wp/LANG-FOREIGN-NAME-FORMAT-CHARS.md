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
authority, and it has **five** operative properties, not four:

1. the predicate is the closed category `Cf`, not an enumerated roster;
2. the error is hard, not advisory;
3. the guard sits on the decoded source codepoint before context dispatch and
   upstream of escape decoding;
4. U+FEFF as a byte-order mark at offset 0 is the sole exception;
5. **the mandated formatter (`§1c`) auto-escapes on save as the exact inverse**
   -- "a raw in-scope `Cf` codepoint in a literal's source spelling is rewritten
   to its escape, value unchanged -- so the hard error is in practice a
   save-time auto-fix, not a barrier."

**Property 5 was missing from this frame until 2026-09-21 and its absence made
AC-1 demand the opposite of the spec.** It is not incidental prose: Architect
component ruling `evt_7yywkwkgrfttj` required the formatter inverse, the
spec-author delivery `evt_261teccj0bw55` recorded it as delivered, and the exact
spec approval `evt_7nsaxdg1y5n18` approved both the escape value identity and
the formatter auto-escape inverse. Read `§1f` whole; a four-property summary of
it is this frame's own recorded defect.

Operator, 2026-09-06: "concur. reframe as whole-source lexical policy."

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
withdrawn twice over: closing the formatter bypass is required, **and so is
building the formatter's auto-escape inverse** (`§1f` property 5).

Boundaries that must not move:

- **The escape repertoire (`31 §3`) is untouched.** The rule restricts the raw
  spelling, never the data.
- **No identifier-class change.** Blessed Unicode letters (`§1e`,
  `SPEC-IDENT-BLESSED`) stay unimplemented; `§1f` covers them later on the same
  guard, which is a property of the placement, not work here.
- **No output change for input that was already lexically clean**, in either
  `kenfmt` route or in `parse_lossless`. Input carrying a raw `Cf` inside a
  literal spelling is not clean, and the formatter is required to change it.
- **The single whole-source guard does not move.** The repair pass is the
  formatter's alone and runs *before* validation; every semantic, lossless and
  recovery path still reaches `ValidatedSource` with raw `Cf` rejected before
  context dispatch. Solving this by admitting raw `Cf` through the ordinary
  lexer, or by adding per-context semantic checks, is a wrong implementation.
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

**The formatter's auto-escape inverse (`§1f` property 5).** Before semantic
validation, and in the formatter alone, run a **sealed lexer-internal
repair observation** that returns only original-source edit spans for raw `Cf`
occurring inside escape-processing literal spellings. It must not expose an
unchecked `Lexer`, a token stream, an AST, or a second general parsing API, and
it must reuse the lexer module's existing context machinery rather than
hand-maintaining a second literal/comment grammar. Apply the resulting plan over
original coordinates **in descending order**, then feed the rewritten source
through the ordinary `ValidatedSource` pipeline, so the strict guard rechecks
the result rather than being bypassed by it.

The split the repair must honour:

- ordinary **string** and **character** spellings are rewritten using their
  existing visible Unicode escape, decoded value unchanged;
- a **byte-string** spelling may use only its existing byte escape, and only
  where the value is representable there;
- **comments, the token stream, triple-quoted raw strings, and every
  non-literal occurrence remain hard errors** -- no value-preserving escape
  exists in those positions, and `§1f` says a raw string cannot carry an
  in-scope `Cf` at all;
- any later diagnostic keeps correct original and document coordinates across
  the rewrite map.

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

**AC-1 — the guard rejects everywhere it must, and the BOM boundary holds.**
Through the semantic entry paths (`Lexer::lex`, `parse_lossless`), a raw U+202E
rejects with the same error kind at the offending codepoint's offset in each of:
a line comment body, a nestable block comment body, the token stream, an
ordinary string literal body, and a **triple-quoted raw string body**. U+FEFF at
offset 0 is consumed as a byte-order mark and U+FEFF at any later offset is the
typed rejection; the BOM case is proved through `parse_lossless` and
`layout::format_ken` together -- the token stream agrees with the BOM-free
source, and token plus trivia pieces form an exact `0..source.len()` partition
that reconstructs the original bytes with one `Bom` item at `0..3` that does not
attach to a declaration. Two mutations must redden this: making `suffix_from`
reset its absolute origin, and bypassing root validation for the recovery loop.

**AC-2 — the predicate is the category, not `§1f`'s examples.** A `Cf` codepoint
that `§1f` never names — U+00AD SOFT HYPHEN, and one of U+0600 or U+061C —
rejects on the same guard, in a comment and in a string body. An implementation
built from the codepoints `§1f` lists passes AC-1 and **fails here**, which is
the point of the criterion. Name, in the candidate, where the category data
comes from and which Unicode version it is pinned to.

**AC-3 — the escape round-trip, both directions, and the formatter's split.**
Both halves of the round-trip must be exercised, and the split must be shown to
be a split rather than a blanket.

- **Decode identity.** `"\u{202E}"` accepts and the decoded `String` contains
  exactly U+202E, unchanged from what a raw occurrence would have produced. A
  guard placed after escape decoding rather than upstream of it fails this half.
- **Formatter inverse.** `layout::format_ken` and `format_ken_md` **succeed** on
  a raw U+202E inside an ordinary string, rewriting it to visible `\u{202E}`,
  and reparsing the output proves the decoded value is exactly U+202E. The
  paired raw character-literal case follows its existing escape repertoire with
  the same value identity.
- **The split.** In the same formatter routes, a raw `Cf` in a comment body, in
  the token stream, and in a triple-quoted raw string remains a typed hard error
  at its exact source or document span. Under `ken ignore` / `ken reject`
  recovery the split is identical: literal payloads auto-fix, comment,
  raw-string and non-literal occurrences reject.

Two mutations must redden AC-3 on opposite sides: **removing the
formatter-only repair pass** must redden the success half, and **bypassing
post-rewrite `ValidatedSource`** must redden the hard-error half. A candidate
where one mutation reddens both has not built a split.

## 6. Stop condition

Hand back, rather than working around, on any of:

- Realizing the `Cf` category predicate genuinely requires a new third-party
  dependency. That is a fork for the Steward to route, not a call to make inside
  the WP.
- A construction path that cannot be closed: some route to a `Lexer` over
  unvalidated text survives that you cannot remove within this scope. Report the
  route; do not compensate with per-context checks.
- The sealed repair observation being unbuildable without exposing an unchecked
  `Lexer`, a token stream, an AST, or a second general parsing API. Report the
  seam; do not ship the exposure and do not hand-maintain a second literal
  grammar to avoid it.
- Any fixed input above measuring false at the landed base.
