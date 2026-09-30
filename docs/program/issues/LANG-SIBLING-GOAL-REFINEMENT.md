---
id: LANG-SIBLING-GOAL-REFINEMENT
title: "A goal that mentions an indexed sibling binder refined by a match on a different scrutinee fails (BadEliminator, 'could not classify the branch goal'); record each failing leaf's proof, its inferred type and its installing match before any fix"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-INFER-MATCH-INDEX-COVERAGE]
blocks: []
github: null
origin: "Architect AC-0 disposition evt_6e58trrr1bnfx on LANG-REFINED-SIBLING-MATCH-TAIL (component 3, mechanism not yet known, so it keeps an AC-0). Inherits the operator's 2026-09-30 Language scheduling ('concur with recs'). Steward-filed per COORDINATION section 2."
---

# Goal refinement over a refined sibling

## Objective

A goal that mentions a sibling binder, whose index was fixed by matching a
different scrutinee, is refined consistently. So f7, f4, f5 and f6 check.

## Settled inputs (Architect `evt_6e58trrr1bnfx`)

- **The witnesses.** f7 is the minimal `BadEliminator("refl a does not
  match Eq A x y")`. f4 and f5 raise `Internal("index refinement: could not
  classify the branch goal")`. f6 is a `BadEliminator`. All are in
  `LANG-REFINED-SIBLING-MATCH-TAIL.md` and `evt_3tsanywqzghge`.
- **The compared indices are distinct binders:** the inner VCons field
  index `_` and the outer Fin-branch `m`, related only by generated
  premises.

## Deliverable

After an Architect-ruled AC-0, one repair to goal refinement.

## Acceptance

- **AC-0 (no fix).** For each failing J base and each classification
  attempt, including f6, record:
  - the resolved `leaf.proof`: sentinel region, slot, and the match that
    owns the region;
  - its kernel-inferred type, against
    `Eq(leaf.index_ty, leaf.target, leaf.scrutinee)`;
  - which match installed the leaf that the goal rewrite consumed.

  It is one mechanism if and only if every failure is a leaf whose proof's
  endpoints are not its recorded endpoints (a wrong region, orientation or
  slot). Stop to the Architect with the rows.
- **AC-1.** f7, f4, f5 and f6 check. `lookup_zip_with` checks unchanged,
  together with `LANG-INFER-MATCH-INDEX-COVERAGE`.
- **AC-2.** The controls e2, e3, e6, f1-f3 and f8 are unchanged, plus a
  mutation named in the ruling.

## Stop conditions

- Any kernel conversion change, trust change, or change to `zip_with`.
- g1 fails at parse (`expected a type, found Lambda`). That is the
  surface-grammar gap (`evt_7aem5zqk3dqm8`) and out of scope.
