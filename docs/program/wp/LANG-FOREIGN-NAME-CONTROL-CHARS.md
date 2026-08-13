# LANG-FOREIGN-NAME-CONTROL-CHARS — reject control characters in `foreign` symbol and library names

**Owner: language. Size: XS. Gate: none.**

**Base: re-derive `origin/main` at cut time**, after `LANG-SURFACE-LITERAL-ESCAPES`
(`29e7e6bb`) lands. Fixed inputs measured at `52d0e252` plus that candidate.

## Fixed inputs

| fact | site |
|---|---|
| the `foreign` declaration parse | `crates/ken-elaborator/src/parser.rs:1590` — `parse_foreign_decl`, spec `38 §2.1` |
| the symbol name, taken straight from `Token::Str` with no validation | same, `:1597-1598` |
| the library name, likewise | same, `:1606-1608` |
| **the decode site, which is SHARED with every string literal** | `crates/ken-elaborator/src/lexer.rs:229` |
| the absence of a production consumer today | no `dlopen` / `dlsym` / `CString::new` in `crates/` outside `px8f` test shims |

## D1 — reject at the two parse sites, not at the decode site

Add validation to `parse_foreign_decl` for **both** strings: reject a name
containing any Unicode control character — **at minimum U+0000.**

**Do not put this in the lexer.** `lexer.rs:229` produces `Token::Str` for every
string literal in the language, and rejecting control characters there would
forbid `"\0"` in ordinary string data — a language change nobody authorized, and
one the escape work just deliberately made expressible.

## D2 — the diagnostic says why, and points at the character

The error must name **which** name (symbol or library), **which** character, and
**where**. A reader hitting this has almost certainly written `\0` believing it
was two characters; the message should make the decoding visible rather than
just refusing.

## Acceptance criteria

- **AC-1 — both names are checked.** Symbol and library. **A check on one is
  the same defect with a smaller surface**, and the library name is the one
  that reaches a loader path first.
- **AC-2 — a positive control: an ordinary string literal containing `\0` still
  elaborates.** This is what proves the check did not land in the lexer. **The
  negative test alone passes for a rejection that is far too wide** — including
  the wrong one this frame exists to prevent.
- **AC-3 — the rejecting test uses an ESCAPE, not a raw control byte.** `\0` in
  source is the case that changed; a literal NUL byte in a fixture file tests a
  path that was already reachable and does not exercise the regression.
- **AC-4 — the boundary of the check is stated in the code**, at the check: what
  it rejects, and that it is **not** a definition of a well-formed C symbol.
  **A reader who mistakes this for name validation will not add the real one.**
- **AC-5 — the span points at the offending literal**, not at the `foreign`
  keyword.

## Pre-stated licensing — read BEFORE reporting

| what this lands | what it licenses |
|---|---|
| control characters rejected in `foreign` names | **Nothing about the C-ABI surface.** It closes one producer-side hole. It does not validate symbol names, does not make the boundary safe, and **does not discharge whatever validation the eventual loader consumer owes.** Say so in the handback. |

> **This is not a vulnerability fix and must not be reported as one.** There is
> no consumer today. It is a cheap check placed before the consumer exists,
> which is the only time it is cheap.

## Banned scope

- Any change to `lexer.rs`, to escape decoding, or to what a string literal may
  contain.
- Validating any other string in the language.
- Defining a well-formed symbol-name policy, or normalizing names.
- Anticipating the loader: no `CString`, no linkage work, no ABI surface.

## Hard stops — return to the Steward

- **The check cannot be made at the parse sites** without touching the lexer.
- **An existing test or catalog source uses a control character in a `foreign`
  name**, which would mean this is a behaviour change rather than a hole.

## Sequencing and contention

Language, one lane, after `LANG-SURFACE-LITERAL-ESCAPES` merges — it edits the
file that work touches. **Order against `LANG-SURFACE-BLOCK-COMMENTS` is the
leader's**; this is XS and does not displace it.

**Verify is concurrently in `ken-elaborator`** on `V3-VERDICT-CENSUS`, reading
`prover.rs` and writing nothing under `crates/`. No contention.

Local runs targeted only — `scripts/ken-cargo -p ken-elaborator --test <name>`.
**Never `--workspace`.**
