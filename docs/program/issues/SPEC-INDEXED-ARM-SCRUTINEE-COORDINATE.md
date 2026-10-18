---
id: SPEC-INDEXED-ARM-SCRUTINEE-COORDINATE
title: "Inside an indexed match arm, the user names of a variable scrutinee and its bare-variable solved index still denote the pre-match coordinate, so a body that reuses them, such as dual_lookup a (Suc q) rest j, is refused with DualFin (Suc q) vs DualFin k. The Spec front desk reads them as denoting the constructor coordinate. State that rule in spec 34 section 3.3"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: [LANG-SOLVED-INDEX-SUBSTITUTION]
github: null
origin: "Architect G2 ruling on LANG-SOLVED-INDEX-SUBSTITUTION (evt_44mfvbvp38fjy, item d), made blocking by evt_2hy029razcfgq: after elimination, secrestpp depends on it. Spec leader front-desk reading evt_7qmn27nc7th3m: the constructor-coordinate reading, scoped to indexed-match branch substitution. Steward-filed per COORDINATION section 2."
---

# An indexed arm's names denote the constructor coordinate

## Objective

Spec 34 states what the user names of a variable scrutinee and its
bare-variable solved index denote inside an indexed match arm, so the
language ring can build to it.

## Settled inputs

- **The reading (Spec leader `evt_7qmn27nc7th3m`).** The constructor
  coordinate, by substitution as in Lean or Agda case splitting: in the
  `DualFZ q` arm of `match j` with `j : DualFin k`, `j` denotes
  `DualFZ q` and `k` denotes `Suc q`. It is scoped to indexed-match branch
  substitution. Plain function parameters outside a match do not change.
- **Measured today (Architect `evt_44mfvbvp38fjy`, on `13f7e885b`).**
  The old-coordinate bodies u11_mixed, u7_pred_old, u6_let_old and u10_old
  are refused on main. The constructor-coordinate bodies u11_new,
  u11_pred_new and u6_let_new are accepted.
- **Current text.** 34 §3.3: an indexed branch binds no scrutinee
  equation, and only §3.2's index refinement specializes the branch. 34
  §3.2.1 pins the sibling convoy, peeled-field injectivity and goal
  refinement. `OQ-indexed-scrutinee-evidence` (90) is open and adjacent:
  it asks about a packed equation, not about what names denote.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

A normative paragraph in `spec/30-surface/34-data-match.md` §3.3 stating
the substitution rule and its scope, with a conformance seed of one
accepted and one refused row.

## Acceptance

- **AC-0 (D0, Architect).** Confirm that the rule needs no kernel change
  beyond KERNEL-OMEGA-STRUCTURAL-POSITIONS. If it needs another, stop to
  the operator.
- **AC-1.** The paragraph, reviewed by the Spec leader and CV with a
  Decision. The seed's accepted row is u11_new's shape. Its refused row
  is a non-constructor coordinate that stays refused under the rule.
- **AC-2.** `OQ-indexed-scrutinee-evidence` is updated to say this rule
  does not settle it, or is closed if the D0 rules that it does.

## Stop conditions

- A rule that changes plain function parameters outside an enclosing
  match.
- Any kernel or `trusted_base()` requirement: an operator question.

LANG-SOLVED-INDEX-SUBSTITUTION builds the rule (its AC-1b).
