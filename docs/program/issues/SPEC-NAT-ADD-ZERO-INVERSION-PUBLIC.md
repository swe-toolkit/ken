---
id: SPEC-NAT-ADD-ZERO-INVERSION-PUBLIC
title: "Two catalog packages privately re-derive the same Nat facts, that a sum equal to Zero has Zero summands: NatBox's add_zero_left and add_zero_right, and NatMatrix's add_eq_zero_left and add_eq_zero_right. Nat Arithmetic exports neither. Export the pair from Data.Numeric.Nat.Arithmetic as checked laws, state them in spec 64 §4, and replace both private copies with imports"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-LINEAR-ALGEBRA-DOT-SELF-ZERO]
blocks: []
github: null
origin: "Architect factoring signal evt_61cvds1xbp8v7 on SPEC-LINEAR-ALGEBRA-DOT-SELF-ZERO AC-0; Steward accepted it as L3's next node (evt_2g6mf277rxn1c). Operator 2026-09-13: every catalog package fully proved. Steward-filed per COORDINATION section 2."
---

# Zero sums have zero summands, once

## Objective

`Data.Numeric.Nat.Arithmetic` exports two checked laws. For every `a`,
`b : Nat`:

```ken
Equal Nat (add a b) Zero → Equal Nat a Zero
Equal Nat (add a b) Zero → Equal Nat b Zero
```

NatBox and NatMatrix import them and keep no private copy. Spec 64 §4
states them by their public types.

## Settled inputs (main after SPEC-LINEAR-ALGEBRA-DOT-SELF-ZERO lands)

- **The private copies.**
  - NatBox: `add_zero_left` (`NatBox.ken.md:1367`) and `add_zero_right`
    (`:1373`) on main `4090b2b45`. Their only use is
    `axis_distance_zero` at `:1045` and `:1046`.
  - NatMatrix: `add_eq_zero_left` (`:1767`) and `add_eq_zero_right`
    (`:1773`), after `add_shuffle` (`:1746`). These lines are on the
    DOT-SELF-ZERO implementation child `e01b94f99`.
  - Both pairs split on the second summand. A `Suc` head refutes the
    Zero equation by `absurd`, because `add` recurses on its second
    argument (`Arithmetic.ken.md:18`).
- **The squaring step.** NatMatrix's private `mul_self_eq_zero` (`:1779`)
  has one consumer, so it stays private (Architect `evt_61cvds1xbp8v7`).
- **The Nat surface.** Nat Arithmetic is an imported package, not
  prelude. Today it exports `add`, `mul`, the `add` and `mul` proofs and
  `mul_add_distrib_r` (`:153`) and `mul_add_distrib_l` (`:178`). Spec 64
  §4, "Nat prerequisite surface" (`64-rational.md:276`), states that
  surface. `nat_arithmetic_laws_acceptance.rs:30` pins the registered
  laws by name.
- **The precedent.** Spec 67 §6.1 added Nat Order laws from a Geometry
  node.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The two laws exported from Nat Arithmetic, and both private pairs
deleted in favour of imports, in one candidate. It also carries spec 64
§4's statement, the acceptance pin and a conformance seed.

## Acceptance

- **AC-0 (D0).** The Architect specifies the development, and the
  Foundation implementer runs it on scratch. The report gives the rc,
  the Axiom grep, and `trusted_base()` as an identity set before and
  after, for Nat Arithmetic, NatBox and NatMatrix. The Architect rules:
  - the law names;
  - their place in Arithmetic;
  - that NatBox's and NatMatrix's public types do not change;
  - the consumer sweep for each deleted private name and for each
    package's public-name inventory.
- **AC-1.** Spec 64 §4 states both laws by their public types. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies each law to a closed sum
  equal to Zero. Its refused block claims the left law's conclusion for
  `add (Suc Zero) Zero`. The accepted same-form control states that sum
  as `Suc Zero`, so the refusal comes from the failed premise, not the
  form.
- **AC-3.** Deleting either imported law from Arithmetic fails NatBox's
  and NatMatrix's checks.

## Stop conditions

- A law that needs an Axiom, or a kernel or surface change.
- A change to a public type in NatBox or NatMatrix.
- A prelude change. Nat Arithmetic is not prelude, and must not become
  prelude.

Checks: 3 fired: grepped catalog/, crates/*/tests, conformance/ and
examples/ for add_zero_left, add_zero_right and add_eq_zero. The private
pairs have the uses named above. The other add_zero hits are
vector_add_zero_* and matrix_add_zero_*, which are unrelated public
names. 4 fired: both private pairs were read at their lines, and both
split on the second summand. 8 fired: the refused claim and its control
use the same sum and differ only in the claimed value.
