---
id: LANG-FOREIGN-NAME-CONTROL-CHARS
title: "Escape decoding made `foreign` symbol and library names able to carry an embedded NUL, where the source text `\\0` previously reached the compiler as two harmless characters -- a NUL in a name that will cross a C-ABI boundary is the classic truncation vector, the declared and effective names silently differ, and there is no consumer today only because the loader path has not landed yet"
status: ready
owner: language
size: XS
gate: none
depends_on: []
blocks: []
github: null
origin: Architect finding at evt_4fqzrhc1x0xvk while approving LANG-SURFACE-LITERAL-ESCAPES, recorded in dec_35jvztfp40v9f, explicitly non-blocking and explicitly needing an owner. Routed by the Steward 2026-08-13, who measured the parse sites and corrected the placement.
---

## What this is

**A few lines of validation at a boundary, before the consumer that would be
hurt by its absence exists.**

`LANG-SURFACE-LITERAL-ESCAPES` made `Token::Str` escape-decoded. `Token::Str`
also carries `foreign` declarations' symbol and library names
(`parser.rs:1597-1598` and `:1606-1608`, spec `38 §2.1`). ⇒ **A `foreign` name
can now contain an embedded NUL or any control character.** Previously the
source text `\0` reached the compiler as backslash-zero — two harmless
characters.

## Why it is worth doing now and not later

**This is not exploitable today, and the frame says so plainly.** The Architect
searched `crates/` for `dlopen`, `dlsym` and `CString::new`; the only hits are C
shims inside `px8f` test harnesses, **not a production consumption path.**

The claim is narrower and it is about cost asymmetry: **rejecting this at the
parse site costs a few lines now; discovering it after a linker or loader path
lands costs a security erratum on a shipped surface.** Ken's Linux-ABI story
says that consumer is coming.

**And it is invisible from the consumer's side.** By the time a loader sees it,
the name is an ordinary `String`. That is how a boundary check goes missing.

## The placement correction — read this before writing the check

The finding says *reject at decode*. **Taken literally that is wrong.** The
decode site is `lexer.rs:229`, which produces `Token::Str` for **every** string
literal in the language. Rejecting control characters there would forbid `"\0"`
in ordinary string data — **a language change nobody has authorized**, and one
the escape work just deliberately made expressible.

⇒ **The check belongs at the two `foreign` parse sites**, where the string is
known to be a name that will cross a C-ABI boundary.

## Not this node

- Any change to the lexer, to escape decoding, or to what string literals may
  contain.
- Validating any other string: module roots, `library` paths used as data,
  diagnostics.
- Building or anticipating the loader. This is the producer-side check.
- Deciding the eventual C-ABI name policy. **This rejects control characters;
  it does not define what a well-formed symbol name is.**
