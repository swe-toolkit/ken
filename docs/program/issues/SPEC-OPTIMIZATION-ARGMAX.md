---
id: SPEC-OPTIMIZATION-ARGMAX
title: "Spec 72 defers maximization, so a client cannot select the greatest-cost candidate from a NonEmpty list and rely on checked membership, optimality and an earliest-tie rule. Rule argmax with checked laws at zero trust from a D0, and state it as a section 3 with a conformance seed"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: [CAT-OPTIMIZATION-ARGMAX]
github: null
origin: "Operator 2026-09-13: every catalog package fully proved. Spec 72 §1.3 lists maximization (`argmax`) first among deferred work. Optimization is the next layer in the in-place rotation after symbolic algebra. Same shape as SPEC-OPTIMIZATION-ARGMIN-OPTION. Steward-filed per COORDINATION section 2."
---

# Greatest-cost candidate selection

## Objective

Spec 72 states `argmax` over `NonEmpty a` with a `Nat` cost, and its
checked laws. A client gets a member of the input whose cost is at
least every member's cost. Among tied candidates it gets the earliest.

## Settled inputs (spec 72 §§1 to 2, on `main`)

- **Sibling (§1.1, §1.2).** `argmin (a : Type) (cost : a → Nat) (xs :
  NonEmpty a) : a`, with seven public laws: `argmin_member`,
  `argmin_least`, `argmin_no_cheaper`, `argmin_head_on_tie`,
  `argmin_singleton`, `argmin_keeps_head`, `argmin_takes_tail`.
  Its tie rule keeps the head unless the tail is strictly better
  under `lt_nat`.
- **Vocabulary.** `NonEmpty` with private `NonEmptyCons`.
  Membership is the Ω-valued `list_elem` over `nonempty_to_list`.
  The order is `Data.Numeric.Nat.Order` (`leq_nat`, `lt_nat` and its
  two bridges).
- **The gap.** §1.3: "maximization (`argmax`) ... remain deferred".
- **Package.** `catalog/packages/Algorithm/Optimization/Argmin.ken.md`.
  The seeds are under `conformance/stdlib/optimization/`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A new spec 72 §3 that states `argmax` and each ruled law by its public
type, with the §1.3 deferral text corrected, plus a closed conformance
seed under `conformance/stdlib/optimization/`. The status lines change
in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  rc 0, with no `Axiom` and `trusted_base()` measured before and after.
  It rules:
  - `argmax`'s signature and home module;
  - its tie rule, which keeps the earliest of the greatest-cost
    candidates;
  - each public law by name and type, mirroring §1.2 where it applies;
  - whether `argmin` and `argmax` share a private fold, with
    `argmin`'s public surface and its laws unchanged.
- **AC-1.** §3 states each ruled law by its public type, and the §1.3
  deferral text is corrected. The Spec leader and CV review it, with a
  Decision.
- **AC-2.** The seed's closed positive client selects from a list with
  a tied maximum and gets the earlier tied candidate. Its refused block
  claims the later tied candidate. A same-shape positive control states
  the earlier one.

## Stop conditions

- A law needs decidable equality on `a`, a new primitive, or a kernel
  or surface feature.
- Any `trusted_base()` change.
- Any change to `argmin`'s public signature or its seven laws.

Checks: 4 fired: the sibling's seven law names, `NonEmpty`'s private
constructor and `list_elem`'s Ω-valued membership are read from spec 72
§1 on main. 8 fired: AC-2's refused block (the later tied candidate)
is rejected by an earliest-tie rule but accepted by a non-strict
comparison that keeps the last tie.
