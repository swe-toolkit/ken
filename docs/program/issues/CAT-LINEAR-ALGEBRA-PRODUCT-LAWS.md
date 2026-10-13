---
id: CAT-LINEAR-ALGEBRA-PRODUCT-LAWS
title: "Spec 65 section 2 states left matrix identity and matrix-product associativity as proved, but Algorithm.LinearAlgebra.NatMatrix exports neither. Deliver both laws at zero trust, through the private named helper the D0 checked, and pin vector_scale's factor"
status: active
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-LINEAR-ALGEBRA-PRODUCT-LAWS]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 65 section 2 from SPEC-LINEAR-ALGEBRA-PRODUCT-LAWS (Architect D0 evt_41pxr8vn2mchx). Adversary evt_30w9qxvkmr0vm on 234ab73c1: vector_scale is pinned only at factor two. Steward-filed per COORDINATION section 2."
---

# Matrix product laws, delivered and proved

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports `matrix_identity_left` and
`matrix_product_assoc` as proved theorems with no new trust, and its
acceptance target can tell a factor-ignoring `vector_scale` from the
real one.

## Settled inputs (Architect D0 `evt_41pxr8vn2mchx`, on `e940e481c`)

- **Both laws check generically.** The statements are in the D0 and in
  spec 65 section 2. The full development is
  `/workspaces/ken/tmp/arch-pl-d0/BothNamed.ken` (SHA-256 `026fbdfd…`):
  - left identity goes through `left_head`, `zero_prefix_columns` and
    `left_tail_rows`;
  - associativity goes through `row_product_assoc`, `columns_cons`,
    `columns_add`, `columns_scale` and a new `dot_scale`.
- **One private respelling is required.** `row_product_columns` passes
  `map` a named helper:
  - `fn dot_with (n) (row) (column) = dot n row column`;
  - `map (Vec Nat n) Nat p (dot_with n row) columns`.

  With the landed λ, `left_head` and `columns_cons` fail with the
  `LANG-ELABORATED-BETA-REDEX-AT-KERNEL-INFERENCE` Internal. No public
  signature changes.
- **New import:** `mul_add_distrib_r` from `Data.Numeric.Nat.Arithmetic`.
- **The `vector_scale` gap.** The only nonempty oracle uses factor two,
  so `zip_with add xs xs` and `map (mul two) xs` both pass 3/3. The
  closed row `vector_scale three three [1,2,0] = [3,6,0]` checks on the
  landed package and refuses both.
- **Seed rows.** SPEC-LINEAR-ALGEBRA-PRODUCT-LAWS grows the seed from
  nine cases and ten positive closed assertions to twelve and thirteen.
  It adds non-square left identity, triple associativity with
  `c22 = [[1,2],[3,0]]`, and the factor-three scale row.
- **Two stale consumers** (spec-author sweep `evt_521mf004e3ahq`,
  `evt_31mkshnc41d95`). Both still count nine seed cases and become false
  when the spec lands:
  - `NatMatrix.ken.md` section 7 calls the seed "nine behavioral cases";
  - `cat_linear_algebra_nat_matrix.rs` :154-257 hard-codes the original
    ten assertions as "the nine specified seed cases".

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The two laws with their private lemmas, and the `dot_with` respelling, in
`NatMatrix.ken.md`. The laws go in the public lede after the five
section 1 laws, in section 2 order. Section 7's case count matches the
seed. The acceptance target runs all twelve seed cases, and its stated
scope says so.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass.
  `cat_linear_algebra_nat_matrix` root-loads both new names and runs
  every seed case, including the three new rows, and no comment or
  message in it still claims nine cases.
- **AC-2.** `trusted_base()` is equal before and after. There is no Axiom
  or primitive. The strict-resolution census and the Vector sentinel are
  unchanged.
- **AC-3 (mutation, QA).** Each mutant fails, and is restored:
  - `vector_scale` ignoring `factor` (`zip_with add xs xs`) fails the
    factor-three oracle;
  - `identity` with its diagonal on the anti-diagonal fails left
    identity;
  - swapping `mul_add_distrib_r`'s arguments inside `dot_scale` is
    kernel-rejected, as the D0 measured.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a public signature
  change.
- A census row, or a consumer of `NatMatrix` outside this package,
  changes.
