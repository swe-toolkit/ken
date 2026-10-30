---
id: SPEC-LINEAR-ALGEBRA-VECTOR-SCALE-ACTION
title: "Spec 65 publishes vector_scale as one of its seven exact operations, but states no law about scaling itself: only dot_scale and matrix_vector_homogeneous mention it. Rule from a checked D0 at zero trust that natural scaling distributes over vector_add, distributes over a sum of factors, composes through mul and is the identity at one, and state the four laws with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Linear algebra is next in the layer rotation after Geometry. Steward-filed per COORDINATION section 2."
---

# Natural scaling acts on vectors

## Objective

Spec 65 states, as public checked laws over
`Algorithm.LinearAlgebra.NatMatrix`, that for every `n`, natural `x`
and `y`, and `u` and `v` of type `Vec Nat n`:

- `vector_scale n x (vector_add n u v)` equals
  `vector_add n (vector_scale n x u) (vector_scale n x v)`;
- `vector_scale n (add x y) u` equals
  `vector_add n (vector_scale n x u) (vector_scale n y u)`;
- `vector_scale n (mul x y) u` equals
  `vector_scale n x (vector_scale n y u)`;
- `vector_scale n (Suc Zero) u` equals `u`.

Each is a structural `Equal (Vec Nat n)` equality.

## Settled inputs (spec 65 §1 to §8, main `40fa3de6c`)

- **The operation.** `vector_scale n factor xs` is
  `map Nat Nat n (mul factor) xs` (`NatMatrix.ken.md:42`), so each law
  reduces to one Nat law per component.
- **Not yet stated.** Spec 65 names `vector_scale` only in §1.2's
  signature and prose, §3's `dot_scale`, and §6's
  `matrix_vector_homogeneous` and its private stage
  (`65-linear-algebra.md:54`, `:62`, `:282`, `:562`, `:584`). No section states a law
  whose subject is scaling.
- **The Nat half exists.** `Data.Numeric.Nat.Arithmetic` exports
  `mul_add_distrib_r` (`mul a (add b c)`, `:153`), `mul_add_distrib_l`
  (`mul (add a b) c`, `:178`), `assoc for mul` (`:201`) and
  `one_l for mul` (`:127`). These are the four component laws in order.
- **Seeds.** Under `conformance/stdlib/linear-algebra/`, next to
  `seed-matrix-vector-linearity.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §9 in spec 65 stating the ruled laws by their public types, the four
catalog theorems in `NatMatrix`, a closed conformance seed under
`conformance/stdlib/linear-algebra/`, and the status lines, all in one
candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules:
  - the theorem names, argument order and placement;
  - which helpers stay private.

  It also confirms that no §1 to §8 operation or law type changes.
- **AC-1.** §9 states each ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client uses `x = 2`, `y = 3`,
  `u = [1,2]` and `v = [3,0]`.
  - It applies each law and checks each side independently by `Refl`
    against its computed vector: `[8,4]`, `[5,10]`, `[6,12]` and
    `[1,2]`.
  - Its refused block pins the dropped-scale regrouping
    `vector_scale 2 (add x y) u = vector_add 2 (vector_scale 2 x u) u`.
    Its right side is `[3,6]`, against the computed `[5,10]`.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise.
- A change to an existing §1 to §8 operation or law type.

Checks: 4 fired: the per-component reduction was written against the
`vector_scale` body at `NatMatrix.ken.md:42` on `1da65f2cd`, unchanged on
`40fa3de6c`, and the four Nat laws were read at their definitions in
`Arithmetic.ken.md`. Spec 65 was grepped for `vector_scale`: its hits are
in §1.2, §3 and §6 only, and none is a scaling law. 8 fired: the seed's
refused row has the second law's shape with one inner `vector_scale`
dropped. For the seed's values, its right side differs from the computed
left side in both components, so a kernel that accepted it would show the
law was not checked.
