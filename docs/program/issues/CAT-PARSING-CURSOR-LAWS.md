---
id: CAT-PARSING-CURSOR-LAWS
title: "Proof-backfill for Capability/Parsing/Cursor.ken.md: construct the missing inhabitant of the package's own CursorLaws proposition for arg_cursor_ops -- the three components (CursorPeekHasRemaining, CursorAdvanceProgress, CursorEndValid) are written as Props with no term proving them for the shipped dictionary."
status: ready
owner: foundation
size: L
gate: none
tier: T1
depends_on: [CAT-PROOF-COMPLETENESS-SURVEY, CAT-NAT-ORDER-LAWS, CAT-COLLECTIONS-NTH-LAWS]
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

## WHY THIS WAS `draft` — the measured dependency, now DISCHARGED

**Released 2026-09-19 at `origin/main`
`a86ee0ca54268d7900d686ec95cb1d7cbf7c81b3`.** Both predecessors are on `main`:
`CAT-NAT-ORDER-LAWS` landed `saturates` and `suc_decreases`, and
`CAT-COLLECTIONS-NTH-LAWS` landed the `nth`/`length` pair. The section below is
the original measurement, retained because the survey does not record this edge
and it should not be re-derived. It is history, not an open condition.

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

RELEASED 2026-09-19; both predecessors are on `main`. Sizing raised from
`M`/`T2` to
`L`/`T1`: the work is inductive proof construction over a fuel-recursive
normalizer that crosses argument boundaries, plus a predicate bridge — semantic
invention, not a mechanical port.

## Settled inputs

Measured. Do not re-derive.

**1. The `draft` reason above is discharged.** Both `sub` facts this node was
waiting on are on `main` in
`catalog/packages/Data/Numeric/Nat/Order.ken.md`. **Navigate by symbol; the
line numbers are current at `a86ee0ca5` and will drift again:**

- **saturation** — `pub proof saturates for sub` (`:137`):
  `(b : Nat) → IsTrue (leq_nat a b) → Equal Nat (sub a b) Zero`
- **strict decrease** — `pub proof suc_decreases for sub` (`:155`):
  `IsTrue (leq_nat (Suc b) a) → IsTrue (leq_nat (Suc (sub a (Suc b))) (sub a b))`

Both statements are quoted byte-exact from `main`; it is only the coordinates
that moved. An earlier revision of this frame cited `:91-93` and `:109-113`,
read off the pre-squash `D1` candidate. **Those lines now hold
`zero_left for max` and `right_leq for max` — real proofs, wrong ones.** A
stale coordinate that lands on a different real symbol does not announce
itself the way a miss does.

**2. The deferred design question is answered: the real gap is the predicate,
and the wrapper is free.** `Order` carries its Boolean hypotheses as `IsTrue`
(`Order.ken.md:61`, `:71`, `:93`, `:103`, `:139`, `:158` — six sites, so this
is the package's convention and not one proof's choice); Cursor's three laws
state their conclusions as
`Equal Bool (cursor_nat_lt ...) True` (`:222`, `:235`, `:250`). An earlier
revision of this frame called that a mismatch on two axes. **It is one.**
`IsTrue (b : Bool) : Prop = Equal Bool b True`
(`Core/Classes/LawfulClasses.ken.md:54`) — the two spellings are the same
proposition, so the wrapper closes by unfolding, like `bytes_nat_length` in
input 4. **What is left is the predicate: `cursor_nat_lt` is strict `<`,
`leq_nat` is `≤`, and they are genuinely different functions.**

The bridge between them is **in scope for this node**. It mentions
`cursor_nat_lt`, a `Cursor`-local symbol, so it is not a `Derived` fact and
must not become a third predecessor.

**3. The bridge needs no new package edge.** `Cursor.ken.md:33` already imports
`sub` from `Data.Numeric.Nat.Order`, and `Order.ken.md:39` re-exports `leq_nat`
and `IsTrue`. Widen the existing import list.

> **CORRECTED 2026-09-19: WIDEN TO `(leq_nat, sub)` ONLY. DO NOT IMPORT
> `IsTrue`.** Foundation measured that importing it makes the consumer-view
> loader harness fail `AmbiguousReference`, because base already supplies
> `IsTrue`. The sentence above is right that `Order` re-exports it and wrong to
> read that as licence to name it here — a name available by two routes is
> ambiguous, not redundant. **The narrowed import preserves package check and
> fmt**, and the bridge still closes because `IsTrue` remains in scope from
> base.

**4. The `Bytes` view closes by unfolding, not by a new fact.**
`bytes_nat_length bs` is *defined* as `length UInt8 (bytes_to_list bs)`
(`Data/Collections/Derived.ken.md:928`), and `arg_length` is `bytes_nat_length`
(`Cursor.ken.md:67`).

**5. `add` facts exist; the `nth` pair HAS LANDED.**
`Data/Numeric/Nat/Arithmetic.ken.md` carries `assoc`, `comm`, `zero_l`,
`zero_r`, `suc_l`, `suc_r`. `Derived.ken.md` carried **no lemma relating `nth`
to `length`**, catalog-wide — until `CAT-COLLECTIONS-NTH-LAWS`, which landed
the pair at `a86ee0ca5`:

- `pub proof some_below_length for nth` (`Derived.ken.md:177`)
- `pub proof at_or_beyond_is_none for nth` (`:191`):
  `IsTrue (leq_nat (length a xs) n) → Equal (Option a) (nth a n xs) (None a)`

It is a discharged `depends_on` of this node, not a wall this node is expected
to hit.

**6. The `Nat` side of the positivity step is already paid — there is no
second predecessor.** `offset < len ⇒ Zero < sub len offset` is not
`saturates`, which is the zero direction. It falls out of `suc_decreases`:
instantiate `b := offset`, `a := len`, case-split on `sub len offset`, and the
`Zero` branch makes the conclusion `IsTrue (leq_nat (Suc _) Zero)`, which is
absurd — so `sub len offset` is a `Suc`, which is what `cursor_nat_lt Zero _`
reduces on. Architect, `evt_1391bs7gcxara`.

**7. Scrutinee order is opposed, and it reads like a wall.** `nth` matches `xs`
first then `n` (`Derived.ken.md:91`); `arg_remaining_from` matches `index`
first then `args` (`Cursor.ken.md:152`). Neither reduces until both scrutinees
are in WHNF, so an induction that splits only on `index` leaves `nth` stuck.
That is a missing case split, not a missing lemma.

## Deliverable

One term inhabiting `CursorLaws ArgCursor UInt8 ArgLocation arg_cursor_ops`,
exported from `Cursor.ken.md`, with whatever package-local bridge and lemmas it
needs. The proposition already exists (`:259-262`); the deliverable is the
inhabitant.

## Stop condition

**If the proof needs a general `List` fact beyond the pair
`CAT-COLLECTIONS-NTH-LAWS` lands, stop and report. Do not widen scope into
`Data/Collections/Derived.ken.md`, and do not land a general `List` fact inside
`Capability/Parsing`.** That is the same rule that produced both predecessors,
and it is the direction that fails closed.

An earlier revision of this frame made the `nth` gap itself the stop condition.
That was the wrong instrument: the gap is reached **with certainty**, in the
base case of `CursorPeekHasRemaining` with a one-argument cursor, so the stop
would have been a scheduled turn ending in the predecessor being cut anyway.
Architect ruling `evt_1391bs7gcxara`; the derivation is recorded in
`CAT-COLLECTIONS-NTH-LAWS`.

Induction on `args`/`index` discharges the outer `nth` layer with no lemma and
is the expected route for that half.

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
kernel/TCB surface; **and** the diff touches
`catalog/packages/Capability/Parsing/Cursor.ken.md` plus, under `crates/`,
**only test harnesses that mechanically reconstruct a consumer's view of a
catalog package.** Any path under `crates/**/src/**`, or any other non-test
path, is a hard stop and a report, not a scope extension.

> **AMENDED 2026-09-19 ON A HARD STOP THAT THIS `AC` PRODUCED CORRECTLY.**
> Foundation stopped at `be70f1f84` rather than edit a harness, which is the
> behaviour the clause was written to get. **The clause was wrong, not the
> stop.** `crates/ken-elaborator/tests/cat_tier_d_cursor_import.rs` holds
> `parsing_cursor_loader_visible_inventory_is_exact`, which asserts
> `published_module_surfaces(...)` equals a 17-name literal set. **A new `pub`
> export necessarily changes that set**, so the harness edit is mechanical and
> forced — refusing it would mean this package can never gain a public export
> without a separate node.
>
> **This `AC` shape has now been wrong three times, all three mine, and this
> time in the OPPOSITE direction from the first two.** On
> `CAT-COLLECTIONS-NTH-LAWS` it was twice too narrow — a path list that had
> only sampled its population — and I reshaped it into exactly the predicate
> above. **One commit later I wrote `crates/`: nothing at all into the
> successor node**, discarding the predicate I had just derived, on a node whose
> whole deliverable is a new `pub` export. Over-tight and under-tight are the
> same defect: an `AC` asserting a population I had not measured.

**`AC-3a` — the harness edit is mechanical, and it is bounded.** *Control, all
three required:* the only change to
`cat_tier_d_cursor_import.rs` is adding the new exported name to the literal
set and updating the "seventeen" prose to match; the assertion stays an
**equality**, never a subset or a `contains`; **and** deleting the added name
makes the harness go RED, restored byte-exact afterwards. **If closing this
needs any harness change beyond those, that is a fresh hard stop** — the
authorisation is for the name, not for the file.

## Design note, not a criterion

Name the `cursor_nat_lt`/`leq_nat` bridge as its own top-level declaration
whose type mentions both, rather than inlining the conversion inside each law.
Inlined, the direction of the strict-versus-non-strict step lives nowhere a
reader can check it. Not an AC: the deliverable is the inhabitant, and the
shape is the implementer's.
