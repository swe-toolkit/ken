# LANG-COMMENT-CLASSIFIER-SHARED

**Owner:** language. **Size:** S. **Gate:** none.
**Predecessor:** `LANG-SURFACE-BLOCK-COMMENTS` (PR #2132).

Make the two scanners' agreement about comments **unrepresentable** rather than
documented, by giving them one shared classifier.

Architect finding at `evt_6703rhjmt19gm`, resolving `dec_7tr1mx06rnap2`,
non-blocking on its own candidate and explicitly routed for an owner:
*"both sites answer the same question -- given `src` and a position, which
comment kind starts here and where does it end? One shared function returning
`(kind, end)`, called by the lexer to advance and by `append_trivia` to bound
its slice, makes divergence unrepresentable instead of tested-against."*

## Fixed inputs, measured at `fa26625b` (the predecessor candidate)

| what | where |
|---|---|
| the lexer's dispatch, THREE arms | `src/lexer.rs:174-188` — `{--`, then `{-`, then `--` |
| the lexer's nested-block end-scanner | `src/lexer.rs:202-230` — `&mut self`, advances `self.pos`, depth counter, errors at EOF |
| the lexer's doc-block end-scanner | `src/lexer.rs:236-255` — first literal `--}`, deliberately non-nesting |
| the lossless dispatch, FOUR arms | `src/lossless.rs:285-304` — `{--`, `{-`, `---`, `--` |
| the lossless nested-block end-scanner | `src/lossless.rs:354` — `(src, start, end) -> Result<usize>`, bounded to `end` |
| the lossless doc-block end-scanner | `src/lossless.rs:387` — same signature |
| the lossless whitespace arm, which is NOT a comment arm | `src/lossless.rs:305-322` — also carries the non-trivia-bytes totality error |
| the existing control corpus | `tests/lang_surface_block_comments.rs` |
| the invariance net, and its stated limit | `lex_agrees_with_lossless` compares `is_ok()` booleans only |

## The finding is real, and its stated population is one arm too wide

**Read this before you write the fix, because inheriting the wider description
produces a wider change than the hazard justifies.**

The finding describes *"the same four-way dispatch (`{--`, `{-`, `---`,
`--`)"* duplicated across the two sites. Measured, **the lexer's dispatch has
three arms, not four**, and the missing arm is deliberate: `lexer.rs:179-182`
folds `---` into the `--` arm, with the reason stated at the site — a doc line
comment is a line comment whose text starts with a dash, and both scan
identically to end-of-line or EOF. `lossless.rs:282-284` records the same
reasoning from the other side.

⇒ **The `---`-before-`--` ordering is not duplicated at all.** There is exactly
one site that distinguishes those two kinds, so reordering them in the lexer is
not representable. **Do not "restore" a `---` arm to the lexer to make the two
dispatches look alike** — that would add a distinction the semantic lexer has
no use for, and it would delete a documented decision.

**What IS duplicated, and what the node is for:**

1. **The `{--`-before-`{-` ordering, at both sites.** This one is
   classification-sensitive in both directions: reorder it in either scanner
   and doc blocks silently become plain blocks **in that scanner only**. The
   two then disagree about **kind** rather than acceptance, so the formatter
   reflows a doc comment as ordinary and changes its attachment. That is
   meaning-affecting, and nothing fails to compile.
2. **Both end-scanners, twice each.** Four functions answering two questions.
   The pairs differ only in cursor mechanics, not in the grammar they encode.

## The design call, front-loaded

**Take the lossless signature, not the lexer's.** `(src, start, end) ->
Result<...>` is the more general of the two: the lexer can call it and assign
`self.pos` from the returned end, passing `src.len()` as its bound. The reverse
does not work — `&mut self` cannot be called from a free function over a slice.

**The bound is safe for the lexer, and here is why it is not merely convenient.**
The lossless scanners refuse to run past `end` because their region is exactly
the gap between two lexer tokens; the lexer's equivalent bound is EOF. Those
coincide because `materialize_partition` establishes that an inter-token region
contains whole trivia — a comment cannot straddle a token boundary, or the
lexer would not have produced that boundary. So one function with an explicit
`end` serves both, and the lexer passing `src.len()` is not a special case.

**Classify comments only. Leave whitespace to each caller.** The lexer skips
whitespace in a separate loop before dispatching; the lossless side folds it
into the same `if/else` chain and hangs its non-trivia-bytes totality error off
that arm. Pulling whitespace into the shared function would either drag that
error into the lexer, where it is not true, or drop it, where it is
load-bearing. **Return `None` when the cursor is not at a comment** and let each
caller keep the arm it already has.

**The lexer will start receiving a kind it does not use.** That is correct and
should be left alone — it discards trivia by design. Do not add a second
comment-only entry point to spare it the discriminant.

## Deliverables

- **D1** — one classifier over `(src, pos, end)` returning the comment kind and
  its end offset, or `None` at a non-comment position, with the unterminated
  errors it already raises preserved verbatim on both forms.
- **D2** — `Lexer::skip_ws_comments` rewired to it; the three lexer scanner
  bodies removed, not left as dead code beside their replacement.
- **D3** — `append_trivia` rewired to it; its whitespace arm and totality error
  unchanged.
- **D4** — a direct end-equality assertion, which is the residual the Architect
  named separately and which survives even if D1 changes shape.

## Acceptance criteria

- **AC-1 — the ordering exists once.** After the change, `{--` is tested before
  `{-` in exactly one location in `crates/ken-elaborator`. State the count and
  the site. This is the criterion the whole node exists for; if it is not
  literally one, the extraction did not happen.
- **AC-2 — the existing corpus passes unchanged.** Every test in
  `tests/lang_surface_block_comments.rs` passes with **no edit to any assertion**.
  Changing a control while rewiring the mechanism it controls forfeits the
  control. Adding tests is fine; amending one is a stop.
- **AC-3 — the direct end-equality assertion, on the nesting corpus.** For each
  nesting fixture, assert the lossless layer's comment end **equals** the
  lexer's for the same input. The existing net compares `is_ok()` booleans, so
  it sees a divergence only when it flips acceptance; round-trip cannot supply
  this either, because `reconstruct() == src` is byte-preserving under either
  attribution. **This must compare two offsets, not two booleans and not two
  reconstructions.**
- **AC-4 — a divergence mutation reds it.** Reorder `{-` ahead of `{--` in the
  shared classifier and show a **kind**-sensitive test failing. Restore before
  landing. If nothing reds, the classifier is not on both paths and D2 or D3 is
  incomplete.
- **AC-5 — `---` keeps its single site and its recorded reason.** The lexer
  gains no `---` arm and `lexer.rs:179-182`'s reasoning survives the edit, moved
  if the code moved. A diff that deletes it is a stop.
- **AC-6 — no new red in CI.** Targeted locally: `-p ken-elaborator`. Never
  `--workspace` on the box.
- **AC-7 — an unrelated one-line rider, carried because it is in your crate's
  test surface and does not deserve a node.**
  `tests/lang_foreign_name_control_chars.rs:62-63` is the positive control for
  `LANG-FOREIGN-NAME-CONTROL-CHARS`, and it is keyed on **absence of error**
  where the property is **presence of value**:

  ```rust
  env.elaborate_decl("const has_nul : String = \"a\\0b\"")
      .expect("an ordinary string literal containing \\0 must still elaborate");
  ```

  It discriminates correctly against the mutation it was written for — a check
  wrongly moved into `lexer.rs`. **It does not reach one step wider:** if a
  later change made the lexer silently **drop or normalize** control characters
  in string data rather than reject them, this stays green while the capability
  it exists to protect is gone. Adversary finding at `evt_cxbze6z3yns8`, which
  correctly declined to prescribe a repair without one read first.

  **That read is done, and the repair exists.** `const` dispatches to
  `parse_view_decl(start, false, DefKeyword::Const)` (`parser.rs:181`), so a
  `const` declaration is a `Decl::ViewDecl { keyword: DefKeyword::Const, body,
  .. }` whose `body` is `Expr::EStr(String, Span)` (`ast.rs:615`). The decoded
  value is reachable exactly the way `Decl::ForeignDecl { symbol, .. }` exposes
  it.

  ⇒ **Assert the value, in the idiom already one file over.** Go through
  `parse_decls` and `assert_eq!` the `EStr` payload against `"a\0b"`, matching
  `d0_foreign_names_decode_escapes_uniformly`'s
  `assert_eq!(symbol, "sym'bol", …)`. **No new accessor is needed, and adding
  one would be the wrong answer.**

  This rider is severable: if it turns out to cost more than a few lines, say
  so and drop it rather than growing the node. It is not the node's purpose.

## Not this node

- Changing what any comment form **means**, where it attaches, or whether doc
  blocks nest. The predecessor settled all three and they are spec-grounded at
  `31 §5`.
- Any formatter behaviour. This makes an existing invariant structural; it does
  not change output.
- Touching `parse_foreign_decl` or the `Cc` check — different boundary, and
  `LANG-FOREIGN-NAME-FORMAT-CHARS` is where the adjacent class is parked.
- Unifying anything else the two layers both do. The finding is about comment
  classification; a general lexer/lossless convergence project is not
  authorized and would not be S-sized.

## Contention

`src/lexer.rs` and `src/lossless.rs`, both `crates/ken-elaborator`, both
Language-owned. No other ring holds either file: Runtime is in
`crates/ken-runtime`, Verify's lane is `src/prover.rs`. `lexer.rs` is a
frequently-touched file, so cut from `main` **after** #2132 lands rather than
from the candidate.
