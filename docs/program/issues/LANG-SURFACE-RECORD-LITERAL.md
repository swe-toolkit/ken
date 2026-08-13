---
id: LANG-SURFACE-RECORD-LITERAL
title: "`33 §2` names record literals `{ x = 1, y = 2 }`, punning `{ x, y }` and functional update `{ p | y = 3 }` as having their expected definitional behaviour, and none of the three parses -- expression-position `{` has no arm in `parse_atom_expr_base` at all, so the brace fork the sibling frame warned about does not exist here: refinement braces live in `parse_type`, which is a separate parser"
status: merged
owner: language
size: M
gate: none
depends_on: [LANG-SURFACE-RECORD-DECL]
blocks: []
github: null
origin: Steward measurement 2026-08-11, taken as the stay-one-release-ahead successor to LANG-SURFACE-RECORD-DECL, whose excluded scope names these three forms as "a sibling's". Carries one inherited obligation from LANG-LEX-HEX-FLOAT, filed by the adversary at evt_99agje0m3rx1's sibling evt_70mrxth1wmx9c and dispositioned by the Steward at evt_5d5epyqg2tjap.
---

## The gap

`spec/30-surface/33-declarations.md:71` lists four things as having their
expected definitional behaviour once a record elaborates to right-nested Σ with
definitional η:

> field access `p.x`, record literals `{ x = 1, y = 2 }`, punning `{ x, y }`,
> and **functional update** `{ p | y = 3 }`

**`LANG-SURFACE-RECORD-DECL` delivers the first.** The other three are this
node.

## The measurement that sizes this, and it is the good news

Measured at `origin/main` = `a6438b76`:

**Expression-position `{` is entirely free.** `parse_atom_expr_base`
(`crates/ken-elaborator/src/parser.rs:2246`) dispatches on `KwIf`, the numeric
literal tokens, `Str`, and `Ident`, and **has no `Token::LBrace` arm at all.**

**The refinement brace is in a different parser.** `parse_type`
(`parser.rs:1650`) handles `{ x : A | φ }` at `:1654-1656`, and `parse_type`
and `parse_expr` (`:1811`) are separate entry points.

⇒ **The brace fork that `LANG-SURFACE-RECORD-DECL`'s excluded-scope section
warned about does not exist for these three forms.** That section said the
literals "open a brace fork this node does not need", citing
`31-lexical.md:194`'s pairing of "record/refinement braces". That pairing is
real in the lexer's vocabulary and **is not a parser conflict**, because the
two never contend for the same position.

**Do not take this as licence to skip the neighbour enumeration.** It changes
what the enumeration will find, not whether it is required — see `AC-5`.

## The three forms disambiguate at one token of lookahead

All three open `{` followed by an identifier. The fork is decided by the token
**after** that identifier:

| next token | form | example |
|---|---|---|
| `=` | record literal | `{ x = 1, y = 2 }` |
| `,` or `}` | punning | `{ x, y }` |
| `\|` | functional update | `{ p \| y = 3 }` |

This is `LL(2)` and needs no backtracking. **State in the handback whether the
implementation actually used two tokens of lookahead or backtracked** — if it
backtracked, something about the above is wrong and I want to know which row.

## Why the elaboration target is already complete

A record is a right-nested Σ and `Term::Pair` already exists. `LANG-SURFACE-PAIR`
landed tuple introduction, so `(1, 2)` already checks against a two-field
record type. **These three forms are surface spellings of a term the elaborator
already builds** — the work is name-directed field ordering, not a new value
form.

**Functional update is the one with real content.** `{ p | y = 3 }` must
project every field of `p` except the updated ones and rebuild the Σ, and
definitional η (`13 §2`) is what makes the rebuilt value equal to `p` when the
update set is empty. That is the property worth asserting.

## Not this node

- **Record patterns in `match`** are `34 §3`'s, and `34-data-match.md:272`
  already specifies them as projecting the negative Σ components. Not here.
- **Named-argument application and record-field constructor labels** are
  explicitly deferred by `34-data-match.md:170-173` as "a later surface
  refinement" (`SURF-gadt-field-sugar`).
- **No new kernel term and no `trusted_base()` change.**
