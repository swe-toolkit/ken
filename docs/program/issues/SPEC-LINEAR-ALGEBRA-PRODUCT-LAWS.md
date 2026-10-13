---
id: SPEC-LINEAR-ALGEBRA-PRODUCT-LAWS
title: "Spec 65 gates left matrix identity and product associativity on LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL, but the Architect measured left identity checking on the landed elaborator through a named helper. Add section 2: left identity as a proved law, and associativity proved or gated on its measured prerequisite"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: [CAT-LINEAR-ALGEBRA-NAT-MATRIX]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 65 section 1.4 names the two gated laws. Architect D0 evt_3yey05qyvbds0 in LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL: the left-identity development checks with a named helper. Steward-filed per COORDINATION section 2."
---

# Matrix product laws for spec 65

## Objective

Spec 65 section 2 states left matrix identity as a proved law that the
catalog can deliver now. It states associativity either as proved or as
gated on a prerequisite that has been measured, not assumed.

## Settled inputs (on `234ab73c1`)

- **Section 1 is delivered.** `Algorithm.LinearAlgebra.NatMatrix`
  (`234ab73c1`) exports the seven operations and five laws of spec 65
  section 1, including `matrix_identity_right`, at zero trust.
- **The gate text is stale for left identity.** Spec 65 section 1.4
  gates `matrix_identity_left` and `matrix_product_assoc` on
  `LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL`. The Architect's D0 in that
  WP (`evt_3yey05qyvbds0`) found a different cause. The failing proof
  writes `map`'s argument as a λ. Spelled with a named helper instead
  (`fn dot_with (n) (row) (column) = dot n row column`), the whole
  left-identity development checks rc=0 on `cc2d62abe`. The λ spelling
  is `LANG-ELABORATED-BETA-REDEX-AT-KERNEL-INFERENCE`, an elaborator
  defect, not a missing law.
- **Associativity is unmeasured.** Section 1.4 says its proposition
  type-forms and its proof was never run.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect, measured).** On the landed base, check a left-identity
   development against the catalog package (rc=0, no Axiom). Attempt the
   generic associativity proof. If it fails, name the failing elaborator
   capability and the WP that owns it, from the measurement.
2. **Section 2, normative.** State `matrix_identity_left` as a proved
   theorem with its type. State `matrix_product_assoc` as proved if the D0
   checks it, otherwise as gated on the measured prerequisite. Section 1.4
   is restated to match. A law's statement does not depend on how a
   helper is spelled.
3. **Seed rows.** Add a left-identity case to
   `conformance/stdlib/linear-algebra/seed-nat-matrix.md`. Add an
   associativity case only if the law is proved.

## Acceptance

- **AC-1.** Every law that section 2 states as proved checks generically
  in the D0 development, with no Axiom and no trust change.
- **AC-2.** Any law still gated names a prerequisite whose failing
  development is recorded in the WP thread.
- **AC-3.** Conformance-validator vote on the exact SHA of the spec paths
  plus the seed, then Architect review.

## Stop conditions

- Left identity does not check on the landed base with a named helper.
- A law needs a kernel, trust, syntax or `Float` change.
