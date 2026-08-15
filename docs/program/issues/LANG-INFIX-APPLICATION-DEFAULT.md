---
id: LANG-INFIX-APPLICATION-DEFAULT
title: "with symbolic operator names defined, `a <+> b` must parse as application of `<+>` at the normative default `infixl 9` -- one precedence level in the existing cascade, and it needs no fixity table"
status: draft
owner: language
size: S
gate: none
depends_on: [LANG-SYMBOLIC-OPERATOR-NAMES]
blocks: [LANG-FIXITY-DECL-SURFACE]
github: null
origin: "Architect scope ruling evt_1s7mqjg4tyxx1 (2026-08-15), part (ii) of his three-way decomposition of user-defined operators. Filed `draft` ONLY because its predecessor has not landed -- the scope is settled and the frame below is complete. Steward-filed and cut per COORDINATION section 2."
---

> # `draft` FOR ONE REASON: ITS PREDECESSOR HAS NOT LANDED.
>
> **The scope question is answered and this frame is shovel-ready.** Flip it
> `ready` the moment [[LANG-SYMBOLIC-OPERATOR-NAMES]] goes `active`; nothing
> else is owed on it. It is `draft` rather than `ready` because a `ready` node
> whose dependency is `ready` fails the schema gate, and because a team pulling
> it today would find its premise false — `<+>` cannot yet be defined.

## Treat every anchor here as perishable

If a fixed input is false against the landed code, **say so and escalate — do
not quietly build around it.** Measured at `e6d2716cf`.

## What this is

`spec/30-surface/32-grammar.md:199` carries the production:

> `| expr binop expr -- operators (declared fixity)`

and `32-grammar.md:373` supplies the default normatively:

> user operators take declared fixity (`infixl`/`infixr`/`infix N`, **default
> `infixl 9`**)

⇒ **A user operator with no fixity declaration is not unparseable — it is
`infixl 9`.** So infix application is deliverable with no fixity table at all,
and that is what makes it separable.

## Why the "freezing" objection does not apply — recorded so it is not re-raised

The Steward withheld a fixture pinning `infixl 9` on the reasoning that it
would freeze a default the pending ruling might change. **It does not.**
`:373` states the default normatively, and the Architect's ruling
(`evt_1s7mqjg4tyxx1`) confirmed that *declared fixity exists* rather than
revisiting what the default is.

⇒ **Pinning `infixl 9` pins the spec.** Pin it.

## Deliverables

**`D1` — parse `a <+> b` as application of `<+>` to `a` and `b`.** At one
precedence level in the existing recursive-descent cascade
(`parser.rs:1994` → `:2012` → `:2032`), at the default binding power.

**`D2` — state where in the cascade you put it, and why.** `infixl 9` is
tighter than the arithmetic operators the cascade already implements under the
VAL2 #11 pin. **Report the resulting relative binding against `+` and `*` with
a fixture that shows it**, not a claim about it. If placing it correctly
requires restructuring the cascade rather than adding a level, **stop and
report** — that is a bigger change than this node and it is a Steward re-cut.

**`D3` — left associativity, pinned.** `a <+> b <+> c` parses as
`(a <+> b) <+> c`. **Control:** assert on the parsed or elaborated shape, not
on the evaluated value — a symmetric operator evaluates identically either way
and would pass a value assertion under the wrong associativity.

## Acceptance criteria

**`AC-1`.** `a <+> b` elaborates to the same term as the prefix application of
`<+>`. **Control:** assert the two elaborate equal, which is the statement that
infix is *notation* and nothing more.

**`AC-2` — associativity and precedence are asserted structurally.** `D3`'s
shape assertion plus `D2`'s relative-binding fixture. **A value-only assertion
fails this criterion**, for the reason `D3` gives.

**`AC-3` — every currently-parsing program parses identically.** **Control:**
the existing arithmetic, comparison, and application fixtures named
individually and green. **This node inserts a level into a working cascade,
which is exactly where a silent reassociation would hide.**

**`AC-4` — direction stated.** This **adds** accepted programs; nothing
accepted becomes rejected. **If anything does, stop.**

**`AC-5`.** No-regression, in CI (`COORDINATION §12`). Targeted locally:
`-p ken-parser`, `-p ken-elaborator`.

## AT THIS NODE'S MERGE DECISION: PING THE ARCHITECT. Do not let this drop.

**He has committed to ruling [[LANG-FIXITY-DECL-SURFACE]]'s import-scoping
question at this node's merge Decision** (`evt_33rw7w8xkdya2`), and
deliberately not before — the right base for that ruling is one where the
parser's actual table-consultation shape is visible, rather than asserting it
from the export type alone.

**Steward: this is an obligation on the merge, not on the ring.** Raise it when
the Decision is opened. Ruling it there means (iii) is not waiting on a
round-trip when the ring reaches it; missing it costs a full exchange at the
worst moment.

## Banned scope

- **No fixity declaration parsing and no fixity table.** Every operator is
  `infixl 9` here. That is [[LANG-FIXITY-DECL-SURFACE]].
- **Do not change the arithmetic ordering.** It is landed and normative under
  the same sentence that settles this feature's existence.
- **Do not widen `BinOp`** (`ast.rs:558`). A user operator is an ordinary
  function applied infix, not a new built-in.
