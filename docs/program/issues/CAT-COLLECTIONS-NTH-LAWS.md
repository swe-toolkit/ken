---
id: CAT-COLLECTIONS-NTH-LAWS
title: "Land the nth/length pair in Data/Collections/Derived.ken.md: nth returning Some bounds the index below the length, and an index at or beyond the length returns None -- the general List facts CAT-PARSING-CURSOR-LAWS reaches in the base case of its first law and cannot discharge in-package."
status: ready
owner: foundation
size: M
gate: none
tier: T1
depends_on: []
blocks: [CAT-PARSING-CURSOR-LAWS]
github: null
origin: "Cut by Architect ruling evt_1391bs7gcxara (2026-09-19) after the Steward framed CAT-PARSING-CURSOR-LAWS with the nth gap as a stop condition. The Architect ruled a stop condition is the wrong instrument: the gap is reached with certainty in the base case of CursorPeekHasRemaining, so the stop would be a scheduled turn that ends in this node being cut anyway. Not one of the seventeen proof-backfill follow-ons in docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md -- it is a newly measured predecessor to one of them."
---

# Proof-backfill predecessor: the `nth`/`length` pair

`Data/Collections/Derived.ken.md` defines `nth` (`:91`) and `length` (`:169`)
and relates them nowhere. `CAT-PARSING-CURSOR-LAWS` needs both directions and
has no cursor structure left to induct on at the point it needs them.

# Frame

Releasable now. Nothing blocks it: the facts it needs already exist.

## Why this is a node and not a stop condition

Architect ruling `evt_1391bs7gcxara`, derived at the object rather than
assumed. `arg_cursor_peek` is two `nth` calls at two different layers — `nth
Bytes index args` and `nth UInt8 offset (bytes_to_list arg)`. Induction on
`index`/`args` discharges the outer layer with no lemma, and lands in
`index = Zero`, `args = Cons arg rest`. With `rest = Nil` — a one-argument
cursor, the common case — the goal reduces to `offset < arg_length arg`, whose
only hypothesis is `nth UInt8 offset (bytes_to_list arg) = Some value`. Since
`arg_length` is `bytes_nat_length` is definitionally
`length UInt8 (bytes_to_list arg)`, the residual obligation is exactly this
node's first direction, over a bare `List UInt8`.

The escape was checked and is closed: `arg_cursor_normalize` does test
`cursor_nat_lt offset (arg_length arg)`, so a *normalized* cursor carries the
invariant by construction — but the three laws at `Cursor.ken.md:222-258` all
open `(cur : c) →` over the bare carrier with no normalization or reachability
precondition, so the normalizer's check is not available as a hypothesis.

## Settled inputs

Measured. Do not re-derive.

**1. No new package edge, so there is no cycle to check.** `Derived.ken.md:66`
already imports `Core.Classes.LawfulClasses (bool_and, bool_leq)`; widen that
list to add `IsTrue` and `leq_nat`. Widening an import list on an edge that
already exists cannot introduce a cycle — the edge is the thing a cycle would
be made of, and it is already there. (Corroborating but not the argument:
`LawfulClasses` imports only `Core.Logic.*` and `Data.Text.StringBijection`,
not `Data.Collections.Derived`.)

**2. Nothing in the catalog already states this.** `nth` appears in ken-fenced
code in exactly four files — `Parsing/Cursor`, `Parsing/Parsing`,
`Process/Arguments`, `Collections/Derived` — and zero declarations relate it to
`length`. Swept catalog-wide per declaration with whitespace collapsed, so a
wrapped statement cannot hide.

**3. Scrutinee order.** `nth a n xs` matches `xs` first, then `n` (`:91`).
`length a xs` matches `xs` (`:169`). `leq_nat m n` matches `m` first, then `n`
(`LawfulClasses.ken.md:489`). Neither `nth` nor `leq_nat` reduces until both
scrutinees are in WHNF; an induction that splits only one leaves the other
stuck. That reads like a wall and is a missing case split.

**4. `IsTrue` is an alias.** `IsTrue (b : Bool) : Prop = Equal Bool b True`
(`LawfulClasses.ken.md:54`) — non-recursive and single-clause, so it unfolds on
any argument without needing its scrutinee in WHNF. The two spellings are one
proposition. Note that `LawfulClasses`' own `leq_nat` proofs use the
`Equal Bool ... True` spelling (`:499`), so the sibling convention in that file
differs from what `AC-2` requires here — see `AC-2` for why the consumer wins.

## Deliverable

Two exported declarations in `Data/Collections/Derived.ken.md`, for a general
element type `a`:

- `nth a n xs = Some a v` implies `Suc n ≤ length a xs`;
- `length a xs ≤ n` implies `nth a n xs = None a`.

Names and binder order are the implementer's. The **statement form is not** —
see `AC-2`.

## Stop condition

**If either direction needs a `Nat` fact that `Data/Numeric/Nat/Order.ken.md`
and `Data/Numeric/Nat/Arithmetic.ken.md` do not already carry, stop and report.
Do not land a `Nat` fact inside `Data/Collections`.** That is the same rule
that produced this node, one layer out, and it is the direction that fails
closed.

## Acceptance criteria

**`AC-1` — both directions are inhabited and neither is degenerate.**
*Control, both halves required:* the package elaborates with both terms
present; **and** replacing either direction's term with `Refl` makes the
package go RED, restored byte-exact afterwards. A positive check alone passes
for a statement that is accidentally trivial.

**`AC-2` — the hypothesis is spelled `IsTrue` over `leq_nat`.** Carry it as
`IsTrue (leq_nat (Suc n) (length a xs))`, not `Equal Bool (...) True`.

*Control:* each added statement's hypothesis is grepped and matches
`IsTrue (leq_nat`, with `Equal Bool` absent from both hypothesis positions.
**This half is textual on purpose.** The two spellings are definitionally the
same proposition, so no elaboration, no mutation, and no test can separate
them — `AC-1`'s control is invariant under this choice and cannot discharge it.

*Why it is required at all, given that nothing breaks either way:* the consumer
is `CAT-PARSING-CURSOR-LAWS`, reaching these through `Order`'s `suc_decreases`,
which is in `IsTrue` form. The argument is legibility at the use site, not
provability and not term size — there is no conversion to avoid. It needs a
criterion because the nearest model of a `leq_nat` fact,
`pub proof refl for leq_nat ... : Equal Bool (leq_nat x x) True`
(`LawfulClasses.ken.md:499`), uses the **other** spelling, so an implementer
copying the neighbour diverges from the consumer and nothing reds.

*One report, not a deliverable:* if Ken's conversion checker turns out to treat
`IsTrue` as **opaque** at these use sites rather than unfolding it, say so
rather than quietly relying on the `IsTrue` form working. Nobody has verified
it; the definition makes it free in principle. If it is opaque, the spelling
stops being legibility and becomes load-bearing, this `AC` changes character,
and its control above becomes the least interesting thing about it. The node
goes green either way, which is what makes it cheap to miss.

**`AC-3` — `nth` and `length` are unchanged.** This node relates the two
functions the package already ships; it does not redefine either to make the
relation provable. *Control:* `nth` (`:91`) and `length` (`:169`) are
byte-identical to their pre-candidate text, extracted and compared
programmatically, not by eye.

**`AC-4` — no new trust, and the diff goes exactly one place.** *Control:* the
added lines contain no `Axiom`, postulate, primitive, `Omega` carrier, or
kernel/TCB surface; **and** the diff touches exactly
`catalog/packages/Data/Collections/Derived.ken.md` and, under `crates/`,
nothing at all. Any other path — test or not — means this frame did not
anticipate something: that is a hard stop and a report, not a scope extension.
