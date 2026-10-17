---
id: RT-C6-FUNCTION-INDEX-CONVERTIBLE-ENDPOINTS
title: "A closed, checked refl cast between data-family endpoints whose function index differs only by convertibility, such as CFn (λn.n) to CFn (λn. idN n), evaluates to Unknown, while the kernel reduces it. The Spec leader reads spec 42 C6 and §16.3.2's indexed-family case as requiring it to compute. Make the evaluator compute it without comparing closures by representation"
status: draft
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-C5-COMPOUND-HIGHER-ORDER-INDICES]
blocks: []
github: null
origin: "Architect RT-C5-COMPOUND D0 ruling evt_1p1wrct6dcx9 (rows 5 and 6, routed to the Spec enclave); Spec leader reading evt_3ww04vvp4k6v0: canonicity requires these casts to compute by C6. Steward-filed per COORDINATION section 2."
---

# A canonical cast at a convertible function index computes

## Objective

A closed, checked refl cast whose endpoints share an inductive head, and
whose function-valued indices are convertible but not identical terms,
evaluates to its payload in the interpreter. The kernel already reduces it.

## Settled inputs

- **The rows (Architect `evt_1p1wrct6dcx9`, measured at `1a84b2378` with the
  C5 identical-endpoint arm).** `fn idN (n : Nat) : Nat = n`.
  - Row 5: `CFn (λn.n)` to `CFn (λn. idN n)` by refl. The kernel check
    passes, the interpreter returns Unknown, and kernel whnf returns the
    value.
  - Row 6: `CPairFn (λn.n, 1)` to `CPairFn (λn. idN n, 1)` by refl. Same
    outcomes.
- **The contract (Spec leader `evt_3ww04vvp4k6v0`).** This is C6, not C5.
  §16.3.2's indexed-family case consumes the index equality and rewrites the
  constructor's index-determining argument. With `idN` transparent, δ/β
  makes the index equality canonical, so canonicity requires the cast to
  compute. This is not an evaluator η step, and no spec change is owed. If
  `idN` is opaque, or the function equality is not canonically
  decomposable, the §16.3.2 guard must be checked first.
- **Forbidden.** Comparing closures by representation or identity
  (spec 42 §1), and reading the proof value under Ω proof irrelevance.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The Architect's D0 names the mechanism: where the evaluator obtains the
canonical index equality, and what it may read. The ring builds that rule
in `crates/ken-interp`.

## Acceptance

- **AC-0 (D0, Architect).** The mechanism, with its soundness boundary and
  an opaque-`idN` row.
- **AC-1.** Rows 5 and 6 evaluate to their value. A non-convertible twin
  stays Unknown.
- **AC-2 (mutation, QA).** Removing the rule returns rows 5 and 6 to
  Unknown, while the C5 rows stay values.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A mechanism that compares closures by representation or identity.
