---
id: SPEC-LINEAR-ALGEBRA-IDENTITY-SYMMETRIC
title: "Spec 65 states transpose laws for products, sums and matrix-vector application, but no law says the identity matrix is its own transpose, although NatMatrix already proves it as the private helper transpose_identity. Rule from a checked D0 at zero trust whether that helper becomes a public law with its type unchanged, and state it with a conformance seed"
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

# The identity matrix is symmetric

## Objective

Spec 65 states, as a public checked law over
`Algorithm.LinearAlgebra.NatMatrix`, that `transpose n n (identity n)`
equals `identity n` as a structural `Vec (Vec Nat n) n` equality, for
every `n`.

## Settled inputs (spec 65 §1 to §7, main `c0db23fe2`)

- **The operations.** `transpose` (`NatMatrix.ken.md:59`) and `identity`
  (`:71`) are public. `identity (Suc k)` is a unit row followed by
  `identity k` with a zero prepended to each row.
- **The proof exists, privately.** `theorem transpose_identity`
  (`:1485`) has exactly the target type:
  `Equal (Vec (Vec Nat n) n) (transpose n n (identity n)) (identity n)`.
  It is a helper for the identity laws at `:1561` and `:1624` and is not
  exported.
- **No public law.** §1 to §7 state `transpose_invol`, `transpose_product`,
  `transpose_add` and `transpose_adjoint`, and none about `identity` under
  `transpose`.
- **Seeds.** Under `conformance/stdlib/linear-algebra/`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §8 in spec 65 stating the ruled law by its public type, a closed
conformance seed under `conformance/stdlib/linear-algebra/`, and the status
lines, all in one candidate. The catalog half travels in the candidate when
AC-0 rules it. Otherwise it is a separate Foundation WP built from the D0.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base, with
  rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity set. It
  rules:
  - the public name, and whether the existing helper is made public in
    place or a public law wraps it;
  - which helpers stay private;
  - the proof form of each seed row (`Refl` or `Proved`), as the §7 seed
    distinguishes them.

  It also confirms that no §1 to §7 operation or law type changes, and that
  the two consumers at `:1561` and `:1624` still check.
- **AC-1.** §8 states the law by its public type. The Spec Leader and CV
  review it, with a Decision.
- **AC-2.** The seed's accepted client:
  - applies the law at `n = 3`;
  - checks the closed transpose of `identity 3` independently against the
    literal 3 × 3 identity.

  Its refused block pins one false claim: a non-symmetric 2 × 2 matrix,
  for example rows `[1,2]` and `[0,1]`, claimed equal to its transpose.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise.
- A change to an existing §1 to §7 operation or law type.

Checks: 4 fired: `transpose_identity` was read at `NatMatrix.ken.md:1485`
on `c0db23fe2`, with its type and its two consumers, and the public
exports were listed; none is about `identity` under `transpose`. 8 fired:
the refused row is a matrix that differs from its transpose, so a law
misstated as "every square matrix is symmetric" would accept it.
