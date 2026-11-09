---
id: SPEC-LINEAR-ALGEBRA-DOT-RIGHT-LINEARITY
title: "Spec 65 exports the dot product's linearity in its first argument (dot_distrib, dot_scale) but keeps the second-argument laws private, so a client must rebuild right-linearity from dot_comm, although the package already proves it. Rule from a checked D0 at zero trust that the two right-position laws become public, so dot is bilinear in the public API, and state them with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Linear algebra follows Geometry in the L3 layer rotation (docs/program/06-catalog-campaign.md). Steward-filed per COORDINATION section 2."
---

# The dot product is linear in both arguments

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports the dot product's
second-argument laws as public checked theorems: `dot n xs (vector_add n
ys zs)` equals `add (dot n xs ys) (dot n xs zs)`, and `dot n xs
(vector_scale n factor ys)` equals `mul factor (dot n xs ys)`. With
`dot_distrib` and `dot_scale`, the public API then states that dot is
bilinear. Spec 65 states both by their public types.

## Settled inputs (main `ebd6b4f26`)

- **The public first-argument laws.** In `NatMatrix.ken.md`: `dot_comm`
  (`:91`), `dot_distrib` (`:125`) and `dot_scale` (`:172`).
- **The private second-argument laws.** `dot_distrib_right` (`:2794`) and
  `dot_scale_right` (`:2817`) already have exactly the types above. Each
  is a composition of `dot_comm` with `dot_distrib` or `dot_scale`.
  Inside the package, they are used at `:1211`, `:1247`, `:1339`,
  `:1346` and `:2936`.
- **The privacy pins.** Four package tests assert that these laws are
  private (Steward amendment `evt_6rtw73k3h6r0n`):
  - `cat_linear_algebra_nat_matrix.rs:277`, an exact `visible ==
    expected` set over the package's whole public API;
  - `cat_linear_algebra_matrix_sum.rs:138`;
  - `cat_linear_algebra_matrix_vector_linearity.rs:124`;
  - `cat_linear_algebra_transpose_adjoint.rs:112`-`113`.

  The prose that calls them private is at spec 65 `:413`, `:431`,
  `:602`-`608` and `:677`, at `NatMatrix.ken.md:3197`, and at
  `spec/SPEC-PROGRESS.md:127`. A grep of every source root finds no
  other consumer.
- **Seeds and tests.** Seeds go under
  `conformance/stdlib/linear-algebra/`, next to `seed-dot-self-zero.md`.
  Tests are named `cat_linear_algebra_<slug>.rs`. Spec 65 ends at §14
  (`:1224`).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate carries all of it:

- the two laws made public, with their names and placement as AC-0
  rules them;
- the four privacy pins migrated, as AC-3 states;
- the stale "private" prose corrected wherever it appears;
- a §15 in spec 65 stating both laws by their public types;
- a closed conformance seed;
- the status lines.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules:
  - whether the names stay or are renamed, and where the laws sit;
  - how each privacy pin migrates;
  - that no §1 to §14 operation or law type changes.
- **AC-1.** §15 states both laws by their public types, and the §4.3,
  §6 and §7 prose no longer calls them private. The Spec Leader and CV
  review it, with a Decision.
- **AC-2.** The seed's accepted client applies both laws to closed
  vectors, with each side checked independently by `Proved`. Its refused
  block pins one false claim in the form of an accepted control, such as
  the distribution law with one summand's vector swapped for a different
  closed vector. A kernel that accepts it would show the law was not
  checked.
- **AC-3.** Every package test that checked before still checks. The
  migrated public pins (`nat_matrix:277` and
  `matrix_vector_linearity:124`) and the new
  `cat_linear_algebra_dot_right_linearity` client fail if either law is
  made private again. `matrix_sum:138` and `transpose_adjoint:112`-`113`
  drop the names (Architect `evt_7zfpwrrkpp18`).

## Stop conditions

- A law needs an Axiom, or a kernel or surface change.
- A change to an existing §1 to §14 operation or law type.

Checks: 3 fired: `dot_distrib_right` and `dot_scale_right` were grepped
over `catalog/`, `crates/`, `conformance/`, `examples/`, `spec/`,
`r_layer_tests` and `docs/program/issues`; the consumers are the package,
three tests, spec 65, SPEC-PROGRESS and three merged frames. The name
grep missed the whole-API set pin at `nat_matrix:277`; the amendment
adds it. 4 fired:
both private signatures were read at `:2794` and `:2817` on `ebd6b4f26`,
and they match the objective's types exactly.
