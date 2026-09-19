---
id: CAT-PARSING-CURSOR-LAWS
title: "Proof-backfill for Capability/Parsing/Cursor.ken.md: construct the missing inhabitant of the package's own CursorLaws proposition for arg_cursor_ops -- the three components (CursorPeekHasRemaining, CursorAdvanceProgress, CursorEndValid) are written as Props with no term proving them for the shipped dictionary."
status: draft
owner: foundation
size: M
gate: none
tier: T2
depends_on: [CAT-PROOF-COMPLETENESS-SURVEY, CAT-NAT-ORDER-LAWS]
blocks: []
github: null
origin: "One of the seventeen proof-backfill follow-ons named by docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md (landed ec23d4ab6), under operator ruling 2026-09-13 / PRINCIPLES #16. Filed draft, NOT released: the Steward measured its dependency on CAT-NAT-ORDER-LAWS at origin/main b839fd63295e550c12524bd9f4804fdf28be7169 while selecting which of the seventeen to release first -- see the body. The survey lists the seventeen as a flat set and does not record this edge; it is filed here so the measurement is not re-derived."
---

# Proof-backfill: `Capability/Parsing/Cursor.ken.md`

The survey's recommendation, verbatim:

> `CAT-PARSING-CURSOR-LAWS`: prove the three `CursorLaws` components for the
> existing `arg_cursor_ops` and its exact location representation with no new
> trust.

The obligation is already written down as a proposition, which is what makes
this node unusually well-specified: `CursorLaws`
(`catalog/packages/Capability/Parsing/Cursor.ken.md:259-262`) is a conjunction
of `CursorPeekHasRemaining` (`:222`), `CursorAdvanceProgress` (`:235`), and
`CursorEndValid` (`:250`). No term inhabits it for `arg_cursor_ops` (`:204`).
The deliverable is the inhabitant, not the statement.

## WHY THIS IS `draft` AND NOT `ready` — the measured dependency

Measured at `origin/main` `b839fd63295e550c12524bd9f4804fdf28be7169`.

`arg_cursor_remaining` (`:166`) reduces through `arg_remaining_from` (`:152`)
to `add (sub (arg_length arg) offset) (arg_lengths_sum rest)`. So
`CursorAdvanceProgress` — advancing a peekable cursor strictly reduces the
remaining count — needs two facts about `sub` that **do not exist**:

- **saturation** — `sub len offset` reduces to `Zero` once `offset` reaches
  `len`, which is what makes `arg_cursor_normalize` (`:175`) preserve the
  remaining count while crossing an argument boundary;
- **strict decrease** — `sub len (Suc offset)` is below `sub len offset` while
  `offset < len`.

`catalog/packages/Data/Numeric/Nat/Order.ken.md` carries exactly three proofs
(`:121-125`), all closing without induction, and carries self-subtraction as a
**failing** attempt in a ` ```ken reject ` fence (`:146`).

⇒ **`CAT-NAT-ORDER-LAWS` `D1` is the prerequisite.** Releasing this node first
would force its implementer either to widen scope into `Nat` or to land `Nat`
facts inside `Capability/Parsing`. Release this once `D1` lands.

## One design question to settle at framing time, not now

`cursor_nat_lt` (`:136`) is **package-local to `Cursor`** — it is not
`Order`'s `leq_nat`. So the bridge between the two Booleans belongs in this
package, not in `CAT-NAT-ORDER-LAWS`, and this node's frame must say which
form `Order` chose to carry a `True` hypothesis (`IsTrue` vs
`Equal Bool ... True`) so the bridge matches it. `CAT-NAT-ORDER-LAWS` `D1`
records that choice in its package prose.
