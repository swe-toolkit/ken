---
id: CAT-PARSING-CURSOR-LAWS
title: "Proof-backfill for Capability/Parsing/Cursor.ken.md: construct the missing inhabitant of the package's own CursorLaws proposition for arg_cursor_ops -- the three components (CursorPeekHasRemaining, CursorAdvanceProgress, CursorEndValid) are written as Props with no term proving them for the shipped dictionary."
status: draft
owner: foundation
size: L
gate: none
tier: T1
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

# Frame

Released when `CAT-NAT-ORDER-LAWS` `D1` lands. Sizing raised from `M`/`T2` to
`L`/`T1`: the work is inductive proof construction over a fuel-recursive
normalizer that crosses argument boundaries, plus a predicate bridge — semantic
invention, not a mechanical port.

## Settled inputs

Measured. Do not re-derive.

**1. The `draft` reason above is discharged.** `D1` candidate
`2e7d1855d97c99344d74007028d0ab1e9e523638` lands both `sub` facts this node was
waiting on, in `catalog/packages/Data/Numeric/Nat/Order.ken.md`:

- **saturation** — `saturates` (`:91-93`):
  `(b : Nat) → IsTrue (leq_nat a b) → Equal Nat (sub a b) Zero`
- **strict decrease** — `suc_decreases` (`:109-113`):
  `IsTrue (leq_nat (Suc b) a) → IsTrue (leq_nat (Suc (sub a (Suc b))) (sub a b))`

**2. The deferred design question is answered, and the answer is a mismatch on
two axes, not one.** `Order` carries its Boolean hypotheses as `IsTrue`
(`Order.ken.md:188`). Cursor's three laws state their conclusions as
`Equal Bool (cursor_nat_lt ...) True` (`:222`, `:235`, `:250`). The bridge
crosses **both** the predicate — `cursor_nat_lt` is strict `<`, `leq_nat` is
`≤` — **and** the wrapper, `IsTrue` against `Equal Bool _ True`. The body above
anticipated the predicate axis only.

**3. The bridge needs no new package edge.** `Cursor.ken.md:33` already imports
`sub` from `Data.Numeric.Nat.Order`, and `Order.ken.md:39` re-exports `leq_nat`
and `IsTrue`. Widen the existing import list.

**4. The `Bytes` view closes by unfolding, not by a new fact.**
`bytes_nat_length bs` is *defined* as `length UInt8 (bytes_to_list bs)`
(`Data/Collections/Derived.ken.md:900`), and `arg_length` is `bytes_nat_length`
(`Cursor.ken.md:67`).

**5. `add` facts exist; `nth` facts do not.**
`Data/Numeric/Nat/Arithmetic.ken.md` carries `assoc`, `comm`, `zero_l`,
`zero_r`, `suc_l`, `suc_r`. `Derived.ken.md` carries **no lemma relating `nth`
to `length`** — `nth` occurs there only in its own definition (`:91`), in
`char_at` (`:882`), in prose, and in the API list. Measured, not assumed.

## Deliverable

One term inhabiting `CursorLaws ArgCursor UInt8 ArgLocation arg_cursor_ops`,
exported from `Cursor.ken.md`, with whatever package-local bridge and lemmas it
needs. The proposition already exists (`:259-262`); the deliverable is the
inhabitant.

## Stop condition

**If the proof needs a general `List`/`nth` fact that belongs in
`Data/Collections/Derived.ken.md`, stop and report. Do not widen scope into
`Derived`, and do not land a general `List` fact inside `Capability/Parsing`.**
That is the same dependency shape this node already measured for `sub`, and it
was resolved with a predecessor rather than a scope extension. `nth` is where it
can recur: `arg_cursor_peek` is two `nth` calls, and both
`CursorPeekHasRemaining` and `CursorEndValid` turn on them.

Proving those two laws by direct induction on the `args`/`index`/`offset`
structure — the recursion `arg_remaining_from` and `arg_cursor_peek` both
follow — is in scope and is the expected route.

## Acceptance criteria

**`AC-1` — the inhabitant exists and is not degenerate.** *Control, both halves
required:* the package elaborates with the new term present; **and** replacing
the `CursorAdvanceProgress` component of the inhabitant with `Refl` makes the
package go RED, restored byte-exact afterwards. A positive check alone passes
for a proposition that is accidentally trivial.

**`AC-2` — the shipped dictionary and the shipped propositions are unchanged.**
This node proves what the package already ships; it does not adjust the package
until it becomes provable. *Control:* these declarations are byte-identical to
their pre-candidate text — `cursor_nat_lt` (`:136`), `arg_lengths_sum` (`:146`),
`arg_remaining_from` (`:152`), `arg_cursor_remaining` (`:166`),
`arg_cursor_peek` (`:169`), `arg_cursor_normalize` (`:175`),
`arg_cursor_advance` (`:194`), `arg_cursor_locate` (`:201`), `arg_cursor_ops`
(`:204`), and the four `Prop`s at `:222`, `:235`, `:250`, `:259`. Extract and
compare them programmatically, not by eye.

**`AC-3` — no new trust, and the diff goes exactly one place.** *Control:* the
added lines contain no `Axiom`, postulate, primitive, `Omega` carrier, or
kernel/TCB surface; **and** the diff touches exactly
`catalog/packages/Capability/Parsing/Cursor.ken.md` and, under `crates/`,
nothing at all. Any other path — test or not — means this frame did not
anticipate something: that is a hard stop and a report, not a scope extension.

## Design note, not a criterion

Name the two-axis bridge as its own top-level declaration whose type mentions
both `cursor_nat_lt` and `leq_nat`, rather than inlining the conversion inside
each law. Inlined, the direction of the `IsTrue` ↔ `Equal Bool _ True`
conversion lives nowhere a reader can check it. Not an AC: the deliverable is
the inhabitant, and the shape is the implementer's.
