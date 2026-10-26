---
id: SPEC-NUMERICAL-INTERVAL-INSIDE-ORDER
title: "Spec 68 section 3 gives interval_inside reflexivity but no transitivity, and states no law that swapping the operands of interval_add, interval_mul, interval_hull or interval_intersect stays inside the original. Rule transitivity and operand-swap inclusion from a checked D0 at zero trust, and state them as a section 5 with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 68 section 3.2 says a transitivity law is not among its nine laws, and section 2 and section 4 add no interval commutativity. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md). Same shape as SPEC-GRAPHS-WALK-APPEND-UNIT-ASSOC. Steward-filed per COORDINATION section 2."
---

# Interval inclusion is a preorder that operand swaps respect

## Objective

Spec 68 states, as public checked laws over
`Algorithm.Numeric.RationalInterval`, that `interval_inside` is
transitive. It also states that each of `interval_add`,
`interval_mul`, `interval_hull` and `interval_intersect` with its
operands swapped lies inside the unswapped result.

## Settled inputs (spec 68 §1 to §4, main `50ca94127`)

- **Inclusion.** `interval_inside` compares endpoint values with
  `rational_leq` (§3.1). §3.2 has `interval_inside_refl` and states
  that transitivity is not among its laws.
- **No commutativity.** §2.2 and §4 state no commutativity of any
  interval operation. `Equal Interval` compares stored spellings
  (§1.1), so the laws are inclusions of endpoint values, not
  structural equalities.
- **Why they can be unconditional.** Transitivity is `trans for
  rational_leq` applied to each endpoint. The operand-swap laws follow
  from §64's `comm for rational_add` and `comm for rational_mul`
  (`Rational.ken.md:243`, `:386`), and from the min and max laws of
  §2.3, carried into `rational_leq` the way §4's private `equiv_leq`
  does.
- **Seeds.** Under `conformance/stdlib/numerical/`, next to
  `seed-interval-inclusion.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §5 in spec 68 stating each ruled law by its public type, §3.2's
sentence on the absent transitivity updated to point at it, and a
closed conformance seed under `conformance/stdlib/numerical/`, with
the status lines changed in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules the
  names, argument order and public types, which of the four operand
  swaps are stated, and which helpers stay private.
- **AC-1.** The §5 states each ruled law by its public type. The Spec
  leader and CV review it, with a Decision.
- **AC-2.** The seed checks transitivity on a closed chain of three
  distinct-endpoint intervals, and each stated swap on a closed
  operand pair with differing endpoints. It has a refused block that
  claims the outer interval of a chain is inside the inner one, with a
  same-input accepting control. The candidate's handoff shows a
  `ken check` rc on its blocks.

## Stop conditions

- A law that needs a validity or membership premise, structural
  `Equal Interval`, or a change to an existing operation or law,
  unless the D0 rules it.
- Any new primitive, kernel or surface feature, or a new public
  Rational declaration.

Checks: 4 fired: §3.2's no-transitivity sentence and the absent
commutativity in §2.2 and §4 are read on `50ca94127`; `comm for
rational_add` and `comm for rational_mul` are at `Rational.ken.md:243`
and `:386`, and `equiv_leq` is private in `RationalInterval.ken.md`.
8 fired: AC-2's reversed-chain refusal fails for a transitivity law
stated with its premises swapped, which the reflexive chain alone
cannot catch.
