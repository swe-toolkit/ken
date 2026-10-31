---
id: SPEC-LINEAR-ALGEBRA-VECTOR-ZERO-UNIT
title: "Spec 65 makes vector_add commutative and associative but has no zero vector, so the sums have no unit and scaling by zero has no stated result (§5.2: neither a zero vector nor a commutative-monoid instance is introduced). Rule from a checked D0 at zero trust a public zero vector, show it is the two-sided unit of vector_add, the result of scaling by Zero and a zero of dot, and state the laws with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Linear algebra is next in the layer rotation after Geometry. Steward-filed per COORDINATION section 2."
---

# The zero vector is the unit of vector sums

## Objective

`Algorithm.LinearAlgebra.NatMatrix` publishes a zero vector of every
length `n`, and spec 65 states, as public checked laws, that for every
`n` and `u : Vec Nat n`:

- adding the zero vector on the left gives `u`;
- adding it on the right gives `u`;
- `vector_scale n Zero u` is the zero vector;
- `dot n` of the zero vector and `u` is `Zero`.

Each is a structural `Equal` for all `n`, including zero.

## Settled inputs (main `0fdc10a5d`)

- **The gap.** Spec 65 §5.2 says "Neither a zero vector nor a
  commutative-monoid instance is introduced" (`65-linear-algebra.md:507`).
  `vector_add_comm` and `vector_add_assoc` exist; no unit does. No
  catalog, spec, conformance or test file names `vector_zero`.
- **The operations.** `vector_add` is `zip_with … add`
  (`NatMatrix.ken.md:39`), and `vector_scale` is `map … (mul factor)`
  (`:42`). Spec 65 §5.2 records that `zip_with` matches on its second
  operand, so the two unit laws need different proof routes.
- **Private material exists.** `repeat` (`:1385`) builds the zero rows
  of `identity`. `dot_zero_left` (`:1520`) already proves
  `dot n (repeat Nat n Zero) xs = Zero`, privately.
- **The Nat half.** `zero_r for add` is `Refl` (`Arithmetic.ken.md:30`).
  `zero_l for add` (`:32`) and `zero_l for mul` (`:70`) are public.
  NatMatrix imports neither yet (`NatMatrix.ken.md:37`).
- **Counts.** The package has eight operations and twenty-six laws,
  which is thirty-four public names. Seeds live under
  `conformance/stdlib/linear-algebra/`, next to
  `seed-vector-scale-action.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate:

- a §10 in spec 65 stating the operation and the laws by their public
  types;
- the catalog operation and theorems in `NatMatrix`;
- a closed conformance seed;
- the status lines and inventory counts.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules:
  - the operation's name and body, including whether it promotes
    `repeat Nat n Zero`;
  - the theorem names, argument order and placement;
  - which helpers stay private.

  It also confirms that no §1 to §9 operation or law type changes.
- **AC-1.** §10 states the operation and each ruled law by its public
  type. The Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client uses `n = 2` and `u = [1,2]`.
  - It applies each law and checks each side by `Refl` against its
    computed value: `[1,2]`, `[1,2]`, `[0,0]` and `0`.
  - Its refused block pins `vector_scale 2 Zero u = u`, the scale-one
    law at the wrong factor. Its left side computes to `[0,0]`, against
    `[1,2]`.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise.
- A change to an existing §1 to §9 operation or law type.

Checks: 4 fired: the four obligations were written against
`vector_add` and `vector_scale` at `NatMatrix.ken.md:39` and `:42` on
`0fdc10a5d`. The Nat laws they reduce to were read at their
definitions in `Arithmetic.ken.md:30`, `:32` and `:70`, and the dot
law already exists privately at `:1520`. 8 fired: the refused row
differs from the computed left side in both components, so a kernel
that accepted it would show the scaling law was not checked.
