---
id: CAT-LINEAR-ALGEBRA-SUM-COMM-ASSOC
title: "Algorithm.LinearAlgebra.NatMatrix exports spec 65 section 5: vector_add_comm, vector_add_assoc, matrix_add_comm and matrix_add_assoc, proved at zero trust from the Architect's D0, with the section 5 seed run by a package test"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-LINEAR-ALGEBRA-SUM-COMM-ASSOC]
blocks: []
github: null
origin: "Catalog half of SPEC-LINEAR-ALGEBRA-SUM-COMM-ASSOC (D0 evt_4t7088x37v0yv). Same shape as CAT-LINEAR-ALGEBRA-MATRIX-SUM. Steward-filed per COORDINATION section 2."
---

# NatMatrix exports the sum laws

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports `vector_add_comm`,
`vector_add_assoc`, `matrix_add_comm` and `matrix_add_assoc` with
exactly spec 65 §5's public types, each a checked proof.

## Settled inputs (spec 65 §5 and its seed, from `f34dc41b0`)

- **The surface.** §5 states the four laws for every length and
  dimension. Associativity is stated left-grouped to right-grouped.
- **The D0.** The Architect's checked D0 on `4e7cf78a6`
  (`evt_4t7088x37v0yv`) gives the 361 inserted lines verbatim (sha256
  `2dc786b0…`, saved at
  `local/architect-shared/la-sum-comm-assoc-d0/inserted-canonical.ken`),
  placed after `pub theorem transpose_add` and before `fn prepend`.
  The four private helpers are `vector_add_cons`, `vector_add_assoc_cons`,
  `matrix_add_cons` and `matrix_add_assoc_cons`. `ken check` and
  `ken fmt --check` rc 0, `trusted_base()` 114 to 114, no new import.
- **The proof technique.** A right-nested `vector_add (Suc k) X
  (vector_add (Suc k) Y Z)` does not convert to its `VCons` form, so
  the step lemmas rewrite through one-level `*_cons` equations with
  explicit `trans` and `cong`. Do not replace them with a direct `Refl`.
- **The one consumer.** `cat_linear_algebra_nat_matrix.rs` pins the
  public inventory at 22 names. It becomes 26, and its
  `visible == expected` assertion keeps the helpers private.
- **Module prose.** The counts in `NatMatrix.ken.md` (lede fourteen
  laws, §2 four laws, §7 twenty-two names and fourteen theorems) become
  eighteen, six, twenty-six and eighteen, and §4 gains one paragraph on
  the technique.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The four exports and four private helpers in `NatMatrix.ken.md` with
the prose counts corrected, the migrated pin, the spec 65 §5 status
lines updated to say the exports are delivered (with §5.2's trust
sentence corrected to "after loading its four providers and again
after loading NatMatrix", `evt_6db5xwe26jj19`), and a new
`crates/ken-elaborator/tests/cat_linear_algebra_sum_comm_assoc.rs` that
root-loads each new public name and runs the seed. Seed execution-status
statements change in the same candidate to say the package test checks
them, with no conformance-runner, CI or landing claim.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  the four names as package-owned and transparent, keeps the four
  helpers private, and runs the seed's positive cases, including the
  empty vector, `0 × n` and `m × 0`. Its refused blocks are pinned to
  the seed's propositions. `cat_linear_algebra_nat_matrix` and
  `cat_linear_algebra_matrix_sum` still pass. The Spec leader reviews
  the spec/ status lines, and CV the seed's.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom
  or primitive. The census row for `Algorithm.LinearAlgebra.NatMatrix`
  is unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails, and is restored:
  - `vector_add_assoc_cons` with the `sym` around the Nat assoc law
    dropped fails check;
  - `matrix_add_comm` stated as `Equal … (matrix_add m n a b) a` fails
    check or the seed's refused operand-equality block;
  - `vector_add_cons` restated with `add y x` in the head fails check.

## Stop conditions

- A law needs an Axiom, a new operation, a class instance, a kernel or
  surface change, or a public type other than §5's.
- A change to `vector_add`, `matrix_add` or their existing laws, a new
  public Nat lemma, a new import, or a census row change.

Checks: 4 fired: the four types, helper set, placement and one consumer
are read from `evt_4t7088x37v0yv` on `4e7cf78a6`; the 22-name pin and
the fourteen and twenty-two prose counts are on `0812a8d3d`. 11 fired:
the right-nested sum does not convert (measured by the D0), so the
frame keeps the one-level rewrites. 8 fired: each AC-3 mutant breaks
an obligation the D0 measured as direction- or component-sensitive.
