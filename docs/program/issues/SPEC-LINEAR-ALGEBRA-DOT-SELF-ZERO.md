---
id: SPEC-LINEAR-ALGEBRA-DOT-SELF-ZERO
title: "Spec 65 gives `dot` its symmetry, distributivity, scaling and zero-annihilation laws (§§2 to 11), but no checked law concludes anything about a vector from a value of `dot`: nothing says a Nat vector whose dot product with itself is zero is the zero vector. Rule from a checked D0 at zero trust the public definiteness law, with any Nat zero-inversion step it needs and a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-GEOMETRY-BOX-INSIDE-INTERSECT-HULL]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Linear algebra follows Geometry in the layer rotation. Steward-filed per COORDINATION section 2."
---

# A vector orthogonal to itself is zero

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports a public checked law. For
every `n : Nat` and `v : Vec Nat n`:

```ken
Equal Nat (dot n v v) Zero → Equal (Vec Nat n) v (vector_zero n)
```

Spec 65 states it by its public type.

## Settled inputs (main `eb65971bf`)

- **The operations.** `dot` (`NatMatrix.ken.md:52`) matches the left
  vector and yields `add (mul x y) (dot k tail_xs tail_ys)` per entry.
  `vector_zero` (`:50`) is `repeat Nat n Zero`.
- **The Nat shape.** `add` (`Arithmetic.ken.md:18`) and `mul` (`:24`)
  both recurse on their second argument. So `add a (Suc b)` and
  `mul (Suc k) (Suc k)` reduce to a `Suc` head, which suggests the
  zero inversion is a case split rather than a named lemma. The D0
  decides whether that holds.
- **The neighbouring laws.** `dot_zero_left` (`:913`) and
  `dot_zero_right` (`:940`) are the converse direction. `dot_comm`
  (`:89`), `dot_distrib` (`:123`) and `dot_scale` (`:170`) are
  equations about `dot`. No public law takes an equation about `dot` as
  a premise.
- **The Nat gap.** `Data/Numeric/Nat/Arithmetic.ken.md` exports only
  `mul_add_distrib_r` (`:153`) and `mul_add_distrib_l` (`:178`) as
  theorems. Nat exports no law that `add a b = Zero` or `mul a a = Zero`
  forces a summand or factor to `Zero`. Spec 67 §6.1 is the precedent
  for a stdlib node adding a Nat law, but this node adds one only if
  the D0 shows it is needed.
- **The spec.** Spec 65 runs §1 to §13 (`65-linear-algebra.md`, §13 at
  `:1155`, file end `:1221`).
- **Seeds and tests.** Seeds go under `conformance/stdlib/linear-algebra/`.
  `seed-zero-annihilation.md` is the nearest neighbour. The package test
  pattern is
  `crates/ken-elaborator/tests/cat_linear_algebra_matrix_product_zero.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The ruled law in NatMatrix, any private or public Nat step it needs, a
spec 65 section stating it by its public type, a closed conformance seed,
a named package test, and spec 65's status lines, all in one candidate.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the source, and the named-binary commands with a bounded
  timeout. The Foundation implementer runs it on scratch and reports
  rc, the `Axiom` grep, and `trusted_base()` as an identity set before
  and after. The Architect rules from that report and does not build
  or test. It rules:
  - the law's name, placement and section number;
  - whether any Nat step is public, and which package owns it;
  - that no existing §1 to §13 operation or law type changes.
- **AC-1.** The section states the ruled law by its public type. The
  Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law to a closed
  vector whose self-dot is `Zero`. Its refused block claims the
  conclusion for a closed vector with one nonzero entry. The seed's
  accepted same-form control states that vector's self-dot as its
  nonzero value, so the refusal is the failed premise, not the form.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise
  beyond the self-dot equation.
- A change to an existing §1 to §13 operation or law type.

Checks: 4 fired: `dot`, `vector_zero`, `add` and `mul` were read at
their lines, and the reduction shape that the zero inversion needs was
written against `add` and `mul` recursing on their second argument. 3
fired: NatMatrix's 49 public names and Nat Arithmetic's two public
theorems were listed; none takes a `dot` or zero-sum premise. 8 fired:
the refused row and its control use the same vector, and differ only in
the claim.
