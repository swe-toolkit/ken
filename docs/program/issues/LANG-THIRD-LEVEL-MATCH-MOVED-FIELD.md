---
id: LANG-THIRD-LEVEL-MATCH-MOVED-FIELD
title: "A true proof with a third-level match inside a nested recursive step's DualFS/DualCons arm is refused: matching j fails with a cast-proof mismatch, and matching rest under an Equal goal fails with NotAFunction on an elim head. Neither is a regression, and the let-annotation repair does not reach them. Measure both mechanisms and repair them"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-RECURSIVE-STEP-COORDINATE-COHERENCE]
blocks: []
github: null
origin: "Adversary hunt of 45c3bd6db (evt_7qdsskjwg0mwh), secondary finding. Architect scoping ruling evt_2qa859pwa5sfq: a separate L node with a D0. Steward-filed per COORDINATION section 2."
---

# A third-level match in the recursive step checks

## Objective

A true proof that matches again on `j` or `rest` inside the DualFS/DualCons
arm of a nested recursive step elaborates.

## Settled inputs (Architect `evt_2qa859pwa5sfq`, on `17bf5738e`)

- **Rows.** These use the same probe heads as
  `LANG-LET-ANNOTATION-MOVED-FIELD-COORDINATE`, inside the DualFS/DualCons
  arm:
  - sec (`match j { DualFZ p ↦ Refl; DualFS p j2 ↦ Refl }`, Equal goal)
    and secpp (pp goal, `hp (dual_lookup a m rest j)` leaves). On the
    first attempt both fail inside the inner match with a cast-proof
    mismatch, `Eq Nat @7 @3` against `Eq Nat (Suc @7) (Suc @11)`.
  - secrest (`match rest { DualNil ↦ Refl; DualCons q y r2 ↦ Refl }`,
    Equal goal). On the first attempt it fails with NotAFunction on an
    elim head.
  - secrestpp is Ok on main.
- **Not a regression.** The Adversary measured both rows as refused on
  `59d4f9701` too. The let-annotation repair, (a) and (b) alone or
  together, does not change them.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first: the Architect traces each row's first-attempt failure to its
producer and rules the repair. The D0 decides whether the two mechanisms
are one change, or splits them. Then the repair.

## Acceptance

- **AC-1.** sec, secpp and secrest are Ok with `trusted_base()`
  unchanged. secrestpp stays Ok.
- **AC-2.** A false twin of each row stays refused and typed. The suites
  named in `LANG-LET-ANNOTATION-MOVED-FIELD-COORDINATE` AC-2 stay green.
- **AC-3 (mutation, QA).** Reverting each repaired producer reddens its
  row.

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
- The D0 cannot place a row's failure in one producer: report the
  measured blocker.
