---
id: CAT-LINEAR-ALGEBRA-NAT-MATRIX
title: "Spec 65 section 1 specifies dimension-safe Nat vectors and matrices, but no catalog package declares them and the Vec carrier they use is private. Publish Vec and deliver Algorithm.LinearAlgebra.NatMatrix: seven operations and five laws, every law proved at zero trust"
status: ready
owner: foundation
size: M
tier: T2
gate: architect
depends_on: [SPEC-LINEAR-ALGEBRA-DIMENSION-SAFE-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 65 section 1 from SPEC-LINEAR-ALGEBRA-DIMENSION-SAFE-CONTRACT (Architect D0 evt_20e1sjrny6hjb, evt_1ze7b0na77885; review evt_2bc3shy87e8vf). Steward-filed per COORDINATION section 2."
---

# Dimension-safe Nat matrices, delivered and proved

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports the seven operations and
five proved laws of spec 65 section 1, over the existing catalog `Vec`,
with no new trust.

## Settled inputs (spec 65 section 1 and its D0)

- **The contract.** Spec 65 sections 1.2 and 1.3 give the seven
  signatures (`vector_add`, `vector_scale`, `dot`, `matrix_vector`,
  `transpose`, `matrix_product`, `identity`) and the five theorems
  (`dot_comm`, `dot_distrib`, `transpose_invol`, `vector_identity`,
  `matrix_identity_right`), verbatim. Left identity and associativity are
  gated (section 1.4) and are not owed.
- **The carrier is private today.** In
  `catalog/packages/Data/Vector/Vector.ken.md`, `Vec`, `map`, `zip_with`
  and `vec_zero_vnil` are not `pub`; only `Fin`, `FZero` and `FSuc` are
  exported. This WP publishes `Vec`, `VNil`, `VCons`, `map`, `zip_with`
  and `vec_zero_vnil`, as the D0 rules, and changes no body.
- **The development exists.** The spec-author's scratch development
  checked the seven operations, the generic transpose, vector identity,
  right identity and dot commutativity proofs rc=0 with no Axiom. The
  Architect checked generic `dot_distrib`. Both are in
  `thr_w31jh51s4kt1`. The dot and transpose empty cases transport through
  `vec_zero_vnil`.
- **Nat arithmetic.** `Data.Numeric.Nat.Arithmetic` exports `add`, `mul`,
  their `assoc`, `comm` and unit laws, and both distributivity laws.
  There is no exported four-term add shuffle: Statistics has a private
  `shuffle_add`, and Rational has a private `mul_shuffle`.
- **The seed** is `conformance/stdlib/linear-algebra/seed-nat-matrix.md`.
  It has nine cases and ten closed positive assertions, including the
  direct `dot` and `identity two` oracles, and one wrong-inner-dimension
  row that must not check.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The Vec publication, then the new package with the seven operations and
five laws in section 1's order, reading top-down, and an acceptance
target that runs the seed.

## Acceptance

- **AC-1.** `ken fmt --check` and `ken check` pass. A new
  `cat_linear_algebra_nat_matrix` target root-loads all twelve names and
  runs every seed case with its listed result. The mismatch row is
  refused, and the same product at matching dimensions checks.
- **AC-2.** `trusted_base()` is equal before and after. No Axiom,
  postulate, primitive or `Float`. `cat_vec_acceptance`,
  `cat_vector_closeout` and every existing importer of
  `Data.Vector.Vector` are unchanged.
- **AC-3 (mutation, QA).** Each mutant makes a proof or a seed oracle
  kernel-reject, and is restored:
  - `dot` as constant `Zero`;
  - `transpose` that reverses row order;
  - `identity` with its diagonal on the anti-diagonal.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, a primitive or
  `Float`.
- A proof needs a shared four-term add shuffle. A private local helper is
  fine. Exporting one from `Nat.Arithmetic`, which would also serve
  Statistics and Rational, is a public-surface change: stop for the
  Architect to rule it, with the `leq_nat_cong_two` export question.
- Publishing `Vec` changes another package's resolution or a
  strict-resolution census row.
