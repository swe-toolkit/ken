---
id: CAT-COMPARE-LAWS
title: "give list_compare the equality soundness/completeness and first-difference lexicographic laws that pair_compare already has for equality, and bridge list_eq to them -- the next of the seventeen proof-backfill follow-ons, selected by Foundation while CAT-PARSING-CURSOR-LAWS is parked"
status: ready
owner: foundation
size: L
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "One of the seventeen proof-backfill follow-ons named by docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md, under operator ruling 2026-09-13 / PRINCIPLES #16. Selected by foundation-leader (evt_3hckxfacz0p9t) while CAT-PARSING-CURSOR-LAWS is parked at 6065b4993 behind the non-local same-GlobalId resolver precursor. Framed and released by the Steward 2026-09-20; all current-code facts measured at origin/main 051039fa02aa8ce2eccf0c788eb50e77a4c57acf."
---

# Compare laws

**`list_compare` carries no proof at all, while `pair_compare` already carries
both directions of its equality law.** Survey row for
`Core/Logic/Compare.ken.md`: *"`pair_compare::eq` and `pair_compare::eq_cases`
prove only pair equality branches. `list_compare` has no general lexicographic,
equality, or order-coherence proof."* This is an absent proof, not a named
deferral.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Settled inputs — measured at `051039fa0`. Do not re-derive.

**1. The shape you are mirroring already exists in the same file.**
`catalog/packages/Core/Logic/Compare.ken.md` carries two attached proofs on
`pair_compare`:

- `:37` `pub proof eq for pair_compare` — soundness. Both component
  comparisons `ord_eq` gives `pair_compare ... = ord_eq`.
- `:295` `pub proof eq_cases for pair_compare` — completeness. The converse,
  returning an `And` of the two component equalities, discharged by a
  `match ... eqn :` on the head comparison with the impossible branches
  closed through `absurd`.

Both are ordinary checked Ken over `Equal OrdResult`. **`list_compare`
(`:387`) and `list_eq` (`:369`) have no attached proof of any kind.**

**2. The module's import surface is small and contains no `IsTrue`.** The
whole file imports exactly `Core.Logic.Or (Or, Inl, Inr)`,
`Core.Logic.OrdResult (OrdResult, Lt, Eq, Gt, ord_eq, ord_lt, ord_gt)` and
`Core.Logic.Transport (sym)`. Three of the four modules in the census's
`expected_clean` list are `Core.Logic.Or`, `Core.Logic.OrdResult` and
`Core.Logic.Transport`, so this module's own ambient residual is genuinely its
own and is the proof vocabulary itself.

**3. This node ADDS NO CATALOG MODULE, so the census failure it can cause is a
different one from the two that just bit L2 and L3.** `Core.Logic.Compare` is
already a row in `catalog_ambient_passthrough_migration_census`
(`crates/ken-elaborator/tests/lang_mod_strict_resolution_d0.rs`), with the
vector

    Core.Logic.Compare = [And, Equal, Proved, and_intro]

`discovered` does not change, so there is no mirroring update to make. What
can change is **this row's vector**, and that is not bookkeeping — see the stop
condition.

> **DO NOT AIM FOR `clean`, and disregard any instruction elsewhere that says
> to.** That test is a behavioral census by its own doc comment: a module
> inherits the ambient residual of everything it roots-loads. `expected_clean`
> has exactly four members and 47 rows sit in the census, including
> `Core.Classes.LawfulClasses` and `Data.Numeric.Nat.Order`. Reaching `clean`
> is not this node's business and is not an acceptance criterion here.

## Deliverable

**Four attached proofs on `Core/Logic/Compare.ken.md`, in the style of the two
that are already there.** One file, no new module, no new trusted assumption.

1. **`eq` for `list_compare`** — soundness. Equal-length lists whose elements
   are pointwise `ord_eq` under `cmp` give `list_compare a cmp xs ys = ord_eq`.
2. **`eq_cases` for `list_compare`** — completeness. The converse: from
   `list_compare a cmp xs ys = ord_eq`, recover equal length and the pointwise
   element equalities.
3. **First-difference lexicographic coherence.** A common prefix of pointwise
   `ord_eq` elements does not affect the result, so a first difference decides
   it: with `cmp x y = ord_lt`, `list_compare` on `prefix ++ (x :: xs)` against
   `prefix ++ (y :: ys)` is `ord_lt` **for arbitrary tails `xs`, `ys`**. State
   `ord_gt` too, or state one and derive the other.
4. **The `list_eq` bridge.** Under an explicitly stated compatibility
   hypothesis relating `eqf` and `cmp` — `cmp u v = ord_eq` exactly when
   `eqf u v = True` — `list_eq a eqf xs ys = True` and
   `list_compare a cmp xs ys = ord_eq` agree.

Investigation needed to build these is their first step, not a separate node.
Item 3's statement is the one to settle before writing terms; if the
`prefix ++` formulation is awkward against the landed `List`, state coherence
by induction on a shared prefix instead, and say which form you took.

## Acceptance criteria

**AC-1 — the lexicographic law has a REACHING WITNESS at a non-degenerate
position, and the witness discriminates against comparing the wrong thing.**
Instantiate item 3 at a concrete pair of lists with a common prefix of length
at least one, the first difference strictly inside both lists, and **tails that
disagree in the opposite direction** — so an implementation or a law that
resolved on the tails rather than the first difference returns the other
`OrdResult`. Show the instantiated law's type and show the same expression
computing to that result. **A witness at `Nil`, at index 0, or with equal
tails is vacuous for this AC and must not be offered as such**: every
candidate law agrees there.

**AC-2 — item 2 is the converse and is not discharged by restating item 1.**
Name, for `eq_cases`, the branch of the `match ... eqn :` that is closed by
`absurd` rather than by computation, exactly as `pair_compare_eq_cases_lt_at`
and `pair_compare_eq_cases_gt_at` do at `:273` and `:284`. A completeness
proof with no impossible branch to close is the tell that the statement
collapsed into the soundness direction.

**AC-3 — `trusted_base()` delta is ZERO and no law is vacuous.** No new
axiom, postulate, or trusted entry, and no law whose hypothesis is
unsatisfiable. For each of the four, state either the reaching witness or the
existing call site that consumes it. A law nothing can instantiate passes a
trust-delta check by adding nothing, which is why the trust check alone does
not discharge this.

## Stop condition

**Hand back rather than work around if a proof needs an ambient name outside
`[And, Bottom, Equal, Prop, Proved, and_fst, and_intro, and_snd]`.** Everything
in `Core.Logic.Compare`'s current vector is inside that set, and so is
`Core.Classes.LawfulClasses`'s whole vector — and `LawfulClasses` imports
`Core.Logic.Compare (list_compare, list_eq, pair_compare, pair_compare_lt_cases,
pair_compare_result_of)`. So a name inside the set costs nothing downstream,
while **a name outside it propagates into `LawfulClasses`, into
`Data.Numeric.Nat.Order`, and into the `Core.Classes.Membership` row L2 is
adding right now.** That is a cross-lane change and it is mine to price, not
yours to absorb. Report the name; do not migrate a provider and do not edit
another row.

Also hand back if the item 4 compatibility hypothesis cannot be stated in
current Ken. **Do not postulate it** — a bridge whose hypothesis is assumed
proves nothing about the two functions.

## Not this node

- **`Capability/Parsing` and anything Cursor touches.** `CAT-PARSING-CURSOR-LAWS`
  is parked at `6065b4993` and its surface stays untouched.
- **Migrating `Core.Classes.LawfulClasses` or any provider off its ambient
  names.** See the stop condition.
- **Any new comparison function, instance, or carrier.** The four laws are
  over the functions that are already there.
- **`CAT-NAT-ORDER-LAWS`** — the min/max/sub/compare algebra is its own survey
  row and its own node.
