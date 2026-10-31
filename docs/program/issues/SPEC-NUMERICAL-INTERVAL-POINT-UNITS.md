---
id: SPEC-NUMERICAL-INTERVAL-POINT-UNITS
title: "Spec 68 regroups, distributes and bounds interval arithmetic by endpoint-value inclusion, but states no unit: Algorithm.Numeric.RationalInterval has no law that adding the point zero, or multiplying by the point one, leaves an interval in place. Rule from a checked D0 at zero trust that each unit result is inside its operand and conversely, and state the four laws with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Numerical follows Graphs in the layer rotation. Steward-filed per COORDINATION section 2."
---

# Point zero and point one are interval units

## Objective

Spec 68 states, as public checked laws over
`Algorithm.Numeric.RationalInterval`, that for every interval `i`,
including an inverted one:

- `interval_add (interval_point rational_zero) i` is inside `i`, and
  conversely;
- `interval_mul (interval_point rational_one) i` is inside `i`, and
  conversely.

Each is an `IsTrue (interval_inside …)` law, the form §6 uses. A
structural `Equal Interval` is not claimed: `rational_add` does not
normalize spellings (§5 design notes).

## Settled inputs (main `6869c29bc`)

- **The operations.** `interval_add` and `interval_mul` act endpoint by
  endpoint, and `interval_point x` is `MkInterval x x`
  (`RationalInterval.ken.md:171`, `:179`, `:187`). `interval_inside`
  compares endpoints by `rational_leq`, so it needs no `lo ≤ hi`
  premise.
- **Not yet stated.** Spec 68 §1 to §9 and the package have no unit law
  for either operation. The package's `interval_unit` (`:1225`) is a
  private worked-example constant, not a law.
- **The Rational half exists, public:** `zero_l` and `zero_r` for
  `rational_add` (`Rational.ken.md:288`, `:337`) and `one_l` and
  `one_r` for `rational_mul` (`:427`, `:451`), each a
  `rational_equiv`, plus `respect` and `refl` for `rational_leq`
  (`:225`, `:508`). The private `equiv_leq` (`RationalInterval:1175`)
  already turns an equivalence into `IsTrue (rational_leq …)`.
- **Seeds.** Under `conformance/stdlib/numerical/`, next to
  `seed-interval-associativity-inside.md`, whose `i=[1/2,1]` can be
  reused.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §10 in spec 68 stating the ruled laws by their public types, the four
catalog theorems in `RationalInterval`, a closed conformance seed under
`conformance/stdlib/numerical/`, and the status lines, all in one
candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules:
  - the theorem names, argument order and placement;
  - whether the right-unit forms are stated or left to the §5
    commutativity laws;
  - which helpers stay private.

  It also confirms that no §1 to §9 operation or law type changes.
- **AC-1.** §10 states each ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies all four laws to the
  reused `i`.
  - Its refused block pins the wrong-unit inclusion
    `interval_inside (interval_mul (interval_point rational_zero) i) i`
    by `Proved`. The left side is `[0,0]` and `1/2 ≤ 0` is false, so the
    kernel refuses it as `TypeMismatch` with the span inside the claim.
  - Its accepted same-form control is
    `interval_inside (interval_mul (interval_point rational_one) i) i`
    by `Proved`, so the refusal is the unit, not the shape. `Refl` does
    not discriminate here: an `IsTrue` goal is not Eq-shaped, so it
    refuses both rows alike (Architect D0 evt_5y7dt3m2x9twr; Steward
    ruling evt_2v2rzk4vq7713).

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise.
- A change to an existing §1 to §9 operation or law type.

Checks: 4 fired: on `58c2e1df6`, each lower-endpoint obligation is
`rational_leq a (rational_add rational_zero a)` or its converse, which
`equiv_leq` gives from `zero_l` and its `sym`; the upper and
multiplicative cases use `zero_l` and `one_l` the same way. Each cited
signature was read at its line, and RationalInterval was grepped for a
unit law with no hit. 8 fired: the refused row and its control differ
only in the unit, and the control is true on the same `i`.
