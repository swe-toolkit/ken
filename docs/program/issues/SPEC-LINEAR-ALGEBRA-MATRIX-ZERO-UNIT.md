---
id: SPEC-LINEAR-ALGEBRA-MATRIX-ZERO-UNIT
title: "Spec 65 §4 gives matrix_add and §5 its commutativity and associativity, but the package exports no zero matrix and no unit law for matrix_add, while §10 gives vector_add its zero and both unit laws. A client that needs the additive identity of indexed matrices must build zero rows from the private repeat and prove the unit by its own induction. Rule from a checked D0 at zero trust a public matrix_zero and its two unit laws, with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Linear algebra follows Geometry in the layer rotation. Steward-filed per COORDINATION section 2."
---

# The zero matrix is the unit of matrix addition

## Objective

`Algorithm.LinearAlgebra.NatMatrix` exports a public
`matrix_zero (m : Nat) (n : Nat) : Vec (Vec Nat n) m`, and spec 65
states, as public checked laws, for every `m`, `n` and
`a : Vec (Vec Nat n) m`:

```ken
Equal (Vec (Vec Nat n) m) (matrix_add m n (matrix_zero m n) a) a
Equal (Vec (Vec Nat n) m) (matrix_add m n a (matrix_zero m n)) a
```

## Settled inputs (main `fcc816c65`)

- **The neighbours exist.** In `NatMatrix.ken.md`:
  - `matrix_add` (`:471`) is `zip_with … (vector_add n) a b`;
  - `matrix_add_comm` (`:654`) and `matrix_add_assoc` (`:692`) are §5;
  - `vector_zero` (`:47`) is `repeat Nat n Zero`, over the private
    `repeat` (`:1516`);
  - `vector_add_zero_left` and `vector_add_zero_right` (`:860`, `:876`)
    are the row-level unit laws a row-by-row proof reuses.
- **The precedent.** Spec 65 §10 (`65-linear-algebra.md:884`)
  introduced the public `vector_zero` together with its unit laws in
  one candidate, and exported both the left and the right form.
- **Not yet stated.** The package, spec 65 and the linear-algebra seeds
  were grepped for `matrix_zero`, "zero matrix" and a matrix_add unit
  law, with no hit. The package states thirty-three laws (`:4`,
  `:2927`). Spec 65 runs to §11 (`:980`, file end `:1061`).
- **Seeds.** Under `conformance/stdlib/linear-algebra/`, next to
  `seed-vector-zero-unit.md`, whose values can be reused.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The public `matrix_zero` operation and the ruled laws in `NatMatrix`,
a spec 65 section stating them by their public types, a closed
conformance seed under `conformance/stdlib/linear-algebra/`, the
public-inventory pin and the package's law count advanced, and the
status lines, all in one candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set, or that measurement assigned to the candidate's package test. It
  rules:
  - the definition of `matrix_zero` and its placement;
  - the law names, argument order and placement;
  - the section number;
  - whether the right unit is exported, as §10 did, or left as a
    composition with `matrix_add_comm`;
  - whether the left annihilation `matrix_vector m n (matrix_zero m n)
    v = vector_zero m` travels in this candidate or as its own node.

  It confirms that no §1 to §11 operation or law type changes.
- **AC-1.** The section states the operation and each ruled law by its
  public type. The Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies each law to a nonzero
  2 × 2 matrix and to a zero-row matrix.
  - Its refused block pins the sum of the zero matrix and the nonzero
    matrix as the zero matrix. The sum is the nonzero matrix, so the
    claim is false.
  - Its accepted same-form control is the same sum as the nonzero
    matrix, so the refusal is the value, not the form.
  - The D0 rules the proof form that separates the two rows.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise.
- A change to an existing §1 to §11 operation or law type.

Checks: 4 fired: the laws are written from public `matrix_add`,
`vector_zero` and the §10 row unit laws, with the private `repeat` that
`vector_zero` already uses, each read at its line; the package, spec 65
and seeds were grepped for a zero matrix or a matrix_add unit law, with
no hit. 8 fired: the refused row and its control share the sum and the
claim form and differ only in the claimed value, the zero matrix
against the nonzero matrix.
