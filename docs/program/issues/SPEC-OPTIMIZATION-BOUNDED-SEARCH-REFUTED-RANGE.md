---
id: SPEC-OPTIMIZATION-BOUNDED-SEARCH-REFUTED-RANGE
title: "Spec 72 §2 states that find_below returns None only when no m below the bound satisfies p, but not the converse: a client that has refuted every m below the bound cannot conclude that the search returns None without casing on the result itself. Rule from a checked D0 at zero trust and state the converse as a public law with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Optimization follows Symbolic algebra in the layer rotation. Steward-filed per COORDINATION section 2."
---

# A refuted range returns None

## Objective

Spec 72 states, as a public checked law over
`Algorithm.Optimization.BoundedSearch`, that for every `p` and
`bound`: if every `m` with `lt_nat m bound = True` has `p m = False`,
then `find_below p bound` equals `None Nat`. This is the converse of
`find_below_complete`.

## Settled inputs (main `166d74d86`)

- **The forward laws exist, public.** `find_below_sound` turns `Some n`
  into `n < bound` and `p n = True`, and `find_below_complete` turns
  `None` into the refuted range (`BoundedSearch.ken.md:42`, `:66`).
- **A proof route** (Architect AC-0 evt_6bgw9h9v84f0p). Casing the
  result directly is refused: `match ... eqn:` takes only nullary enums.
  A private helper `refuted_outcome_none` binds the result as `outcome`
  with its equation. `None` is immediate. `Some n` meets
  `find_below_sound`'s `p n = True` against the premise's `p n = False`
  at the same `n`, which is absurd. The public law
  `find_below_refuted p bound` instantiates it with `Refl`.
- **Not yet stated.** Spec 72 §2.1 to §2.4 and the package have no
  converse. §2.2 (`72-optimization.md:503`) says the three laws
  determine the result, but no law concludes `None`.
- **Seeds.** Under `conformance/stdlib/optimization/`, next to
  `seed-bounded-search-bound-stability.md`, whose `above_two`, `three`
  and `ten` can be reused.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §2.5 in spec 72 stating the ruled law by its public type, the catalog
theorem in `BoundedSearch`, a closed conformance seed under
`conformance/stdlib/optimization/`, and the status lines, all in one
candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules the theorem name, argument order and premise form, and
  which helpers stay private. It also confirms that no §1 to §3
  operation or law type changes.
- **AC-1.** §2.5 states the ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law to `above_two`
  at bound `three`, with the premise discharged by
  `above_two_refuted_below_three` (its arms by `Proved`).
  - Its refused block pins `Equal (Option Nat) (find_below above_two
    ten) (None Nat)` by `Proved`, on a fresh accepted prefix. The search
    computes `Some three`, so it is refused with KernelRejected
    TypeMismatch.
  - Its accepted same-form control is the same claim at bound `three`
    by `Proved`, so the refusal is the bound, not the form. `Refl` does
    not discriminate: an `Equal` goal is not Eq-shaped, so `Refl`
    refuses the true control too (AC-0).

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or any premise
  beyond the refuted range.
- A change to an existing §1 to §3 operation or law type.

Checks: 4 fired: the converse is written from the public
`find_below_sound`, read at its line, and a case on the result; the
package was grepped for a law concluding `None`, and there was no hit.
8 fired: the refused row and its control share `above_two` and the
claim form and differ only in the bound. Under `Proved` the control is
accepted and only the false claim is refused (AC-0, d0c to d0f).
