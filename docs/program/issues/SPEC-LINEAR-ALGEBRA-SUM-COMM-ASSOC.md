---
id: SPEC-LINEAR-ALGEBRA-SUM-COMM-ASSOC
title: "Spec 65 states vector_add and matrix_add and distributes products over them, but states no law of the sums themselves: a client cannot reorder or regroup two vector or matrix sums. Rule commutativity and associativity of vector_add and matrix_add from a checked D0 at zero trust, and state them as a section 5 with a conformance seed"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 65 sections 1.2 and 4.1 define the two sums with no law of their own. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md). Same shape as SPEC-LINEAR-ALGEBRA-MATRIX-SUM-DISTRIBUTIVITY. Steward-filed per COORDINATION section 2."
---

# Vector and matrix sums commute and associate

## Objective

Spec 65 states, as public checked laws over
`Algorithm.LinearAlgebra.NatMatrix`, that `vector_add` and `matrix_add`
are commutative and associative for every length and dimension.

## Settled inputs (spec 65 §1 and §4, main `f9ebf94d3`)

- **The sums.** `vector_add n xs ys = zip_with Nat Nat Nat n add xs ys`
  (§1.2, `NatMatrix.ken.md:39`) and `matrix_add m n a b` applies
  `vector_add n` row by row through `zip_with` (§4.1, `:467`). The
  indexed `zip_with` cannot truncate either operand.
- **The gap.** §1.3 to §4.2 export the dot, transpose, identity,
  product, distributivity and `transpose_add` laws. None states
  `vector_add` or `matrix_add` commutative or associative.
  `NatMatrix.ken.md` uses only the Nat class laws for `add`
  (`:678-706`).
- **Why they can be unconditional.** Each law is the Nat `add` law
  lifted componentwise over `Vec` induction, then lifted again row by
  row for matrices. The checked Nat laws are the class laws
  `proof comm for add` and `proof assoc for add`, already used in
  `NatMatrix.ken.md` and `Data.Numeric.Nat.Arithmetic`.
- **Seeds.** Under `conformance/stdlib/linear-algebra/`, next to
  `seed-matrix-sum-distributivity.md`.

Treat anchors as perishable. If a fixed input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §5 in spec 65 stating each ruled law by its public type, the §1.4
trust paragraph updated to include it, and a closed conformance seed
under `conformance/stdlib/linear-algebra/`, with the status lines
changed in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules the
  names and public types of the four laws, whether the matrix laws are
  proved from the vector laws, and which helpers stay private.
- **AC-1.** The §5 states each ruled law by its public type. The Spec
  leader and CV review it, with a Decision.
- **AC-2.** The seed checks each law on closed operands, including an
  empty vector, a `0 × n` and an `m × 0` matrix, and has refused blocks
  that claim a sum equal to an operand when the other operand is
  non-zero, and a regrouped sum with one component changed. The
  candidate's handoff shows a `ken check` rc on its blocks.

## Stop conditions

- A law that needs a new operation (such as a zero vector), a
  commutative-monoid class instance, or a change to `vector_add`,
  `matrix_add` or any existing law, unless the D0 rules it.
- Any new primitive, kernel or surface feature, or a new public Nat
  lemma.

Checks: 4 fired: `vector_add` and `matrix_add` are `pub` at
`NatMatrix.ken.md:39` and `:467` on `f9ebf94d3`; its 14 exported
theorems include no sum law, and the Nat `add` laws it uses are class
laws (`:678-706`). 8 fired: AC-2's changed-component refusal fails for
a sum that drops or duplicates a component, which the empty and zero
dimension cases alone cannot catch.
