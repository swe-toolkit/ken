---
id: SPEC-SYMBOLIC-ALGEBRA-POLY-EQ-CONGRUENCE
title: "Spec 66 §2 decides semantic equality of polynomial expressions with poly_eq, but a client that knows two subterms are poly_eq cannot conclude that the larger expressions built from them are: Algorithm.SymbolicAlgebra.NatPolynomial keeps its expression congruence proofs private. Rule from a checked D0 at zero trust that poly_eq is preserved by EAdd and EMul, and state the two laws with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Symbolic algebra is next in the layer rotation after Time. Steward-filed per COORDINATION section 2."
---

# Semantic equality survives addition and multiplication

## Objective

Spec 66 states, as public checked laws over
`Algorithm.SymbolicAlgebra.NatPolynomial`, that for every `a`, `a'`,
`b` and `b'` with `IsTrue (poly_eq a a')` and `IsTrue (poly_eq b b')`:

- `IsTrue (poly_eq (EAdd a b) (EAdd a' b'))`;
- `IsTrue (poly_eq (EMul a b) (EMul a' b'))`.

## Settled inputs (spec 66 §1 to §9, main `e913d5f1f`)

- **The interpretation.** `eval` maps `EAdd` to `add` and `EMul` to
  `mul` of the operands' values (`NatPolynomial.ken.md:103`).
- **The equality half exists.** `poly_eq_sound` (`:718`) turns
  `IsTrue (poly_eq e1 e2)` into equal `eval` at every environment, and
  `poly_eq_complete` (`:741`) turns equal `eval` at every environment
  back into `IsTrue (poly_eq e1 e2)`. Each law is sound, then `cong`
  over `add` or `mul`, then complete; no new Nat law is needed.
- **Not yet stated.** The package's expression congruence proofs are
  private (prose at `:52` and `:2788`). Spec 66 and the package export
  no `poly_eq` congruence, symmetry or transitivity law.
- **Seeds.** Under `conformance/stdlib/symbolic-algebra/`, next to
  `seed-nat-polynomial-canonical-equality.md`. Its `x0_sum` and
  `two_times_x0` are `poly_eq`, and `x1` can be reused.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §10 in spec 66 stating the ruled laws by their public types, the two
catalog theorems in `NatPolynomial`, a closed conformance seed under
`conformance/stdlib/symbolic-algebra/`, and the status lines, all in
one candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules:
  - the names, argument order, premise form and placement;
  - whether `poly_eq` symmetry and transitivity belong in this §10 or
    stay out;
  - which helpers stay private.

  It also confirms that no §1 to §9 operation or law type changes.
- **AC-1.** §10 states each ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies both laws with
  `a = x0_sum`, `a' = two_times_x0` and `b = b' = x1`.
  - Its refused block pins
    `IsTrue (poly_eq (EMul x0_sum x1) (EAdd two_times_x0 x1))`.
  - That swaps the outer operator, so the two sides denote `2·x0·x1`
    and `2·x0 + x1`, which differ at `x0 = x1 = 1`.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise
  beyond the two `poly_eq` facts.
- A change to an existing §1 to §9 operation or law type.

Checks: 4 fired: `eval`, `poly_eq_sound` and `poly_eq_complete` were
read at `:103`, `:718` and `:741` on `e913d5f1f`. The public theorem
list was grepped for a `poly_eq` congruence, symmetry or transitivity
law; the only congruence hits are private-proof prose. 8 fired: the
refused row keeps both `poly_eq` premises true and changes only the
outer constructor, so it fails because the denotations differ (2 versus
3 at the all-ones environment), not because a premise fails.
