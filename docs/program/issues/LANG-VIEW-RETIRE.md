---
id: LANG-VIEW-RETIRE
title: "Operator ruling SURF-1 retired the single definition keyword `view` and split it into `const`/`fn`/`proc`, but the landed elaborator still accepts it -- and `view` is not an alias: it takes an EARLY RETURN out of the bidirectional purity check that `33 §1` calls a hard error, so every definition still spelled `view` has never been checked for the effect discipline the spec requires"
status: merged
owner: language
size: M
gate: none
depends_on: [LANG-RECORD-STACK-OVERFLOW]
blocks: []
github: null
origin: Operator instruction 2026-08-13, raised from a `view` fixture in the record-literal candidate. Spec basis is SURF-1 as recorded at spec/30-surface/36-effects.md:410 (`1.6.4 view retired`), 33-declarations.md:60, and 31-lexical.md:591; 31-lexical.md:656 records the implementation gap in the spec's own words.
---

## The ruling, and the gap the spec already admits

`view` is **retired** by operator ruling SURF-1 and split three ways by the
`36 §1.6.1` rule:

| former `view` | becomes |
|---|---|
| pure, 0 explicit value params (incl. top-level `let`) | `const` |
| pure, ≥1 explicit value param | `fn` |
| concrete effect at any arity | `proc` |
| effect-polymorphic (declares a row variable) | `proc` |
| operator (symbolic name, `33 §6`) | `fn`/`proc`, or `const` if nullary-pure |

The spec records the implementation gap itself, at `31-lexical.md:656`:

> The landed V0 lexer still spells `view`/`let` until the D4 migration; the
> surface here is the target.

⇒ **This node is that migration.** It is not a discovery task — the target
mapping is ratified and written down.

## `view` IS NOT AN ALIAS FOR `fn`, AND THIS IS THE WHOLE RISK

Measured at `origin/main` = `cbe58725`:

- **`crates/ken-elaborator/src/elab.rs:4687`** —
  `if keyword == DefKeyword::View { return Ok(()); }` — an **early return out
  of the effect-row verification** that `const`/`fn`/`proc` all undergo.
- **`crates/ken-elaborator/src/elab.rs:4772`** — `DefKeyword::View => {}`, a
  no-op arm in the purity-mismatch diagnostics.

`33 §1` says a keyword/effect mismatch is a **hard error**. ⇒ **Every
definition still spelled `view` has been exempt from that hard error for its
whole life.**

**So migrating a `view` to `fn` is not a rename — it subjects a definition to a
check it has never faced.** Some will fail. **Those failures are the point of
this node, not an obstacle to it:** each one is a latent purity violation the
legacy keyword has been hiding, and the count is a number nobody has today.

## The second trap: `view` lets you NAME a definition `const`, `fn`, or `proc`

`parser.rs:83` `expect_legacy_view_name` accepts `Token::KwConst`,
`Token::KwFn` and `Token::KwProc` as the **name** of a `view` definition, where
the non-legacy path (`expect_ident`) does not. So `view fn (x : A) : B = …`
defines something *called* `fn` and is legal today.

⇒ Any such definition becomes **unspellable** the moment `view` goes. That is a
rename with call-site consequences, not a keyword swap, and it needs its own
census before anything is edited.

## What is one function, and therefore is NOT the work

`parse_view_decl` (`parser.rs:457`) is the **shared** parser for all four
keywords — `parser.rs:195-198` dispatch `view`/`const`/`fn`/`proc` into it.
Retiring `view` removes a dispatch arm and a `DefKeyword` variant; it does not
remove a parse path. The legacy names `parse_view_decl` and `ViewDecl` are a
separate cosmetic question and are **not** in this node.

## The measured surface

Keyword-position `view <ident>` occurrences at `cbe58725`:

| area | occurrences | files | owner |
|---|---|---|---|
| `crates/` | 116 | 28 | **language — this node** |
| `spec/` | 100 | 24 | spec enclave |
| `docs/` | 168 | 64 | doc ring |
| `conformance/` | 71 | 17 | conformance-validator |
| `library/` | 2 | 2 | librarian |

**`conformance/` and `spec/` occurrences are PROSE in seed and specification
documents, not executable fixtures** — checked; they are descriptions like
*"given a `view classify (x) : Tag`"*. So they do **not** break when the
elaborator stops accepting the keyword, and they are not this node's to edit.

## Not this node

- **`spec/`, `docs/`, `conformance/`, `library/` prose.** Owned by the enclave,
  the doc ring, CV and the librarian respectively. They are successors, and
  they are not on this node's critical path because none of them executes.
- **Top-level `let`.** `36 §1.6.4` removes it in the same clause, and it is a
  sibling migration with its own population. Keep the two separable.
- **Renaming `parse_view_decl` / `ViewDecl`.** Cosmetic, and it would bury the
  semantic diff.
