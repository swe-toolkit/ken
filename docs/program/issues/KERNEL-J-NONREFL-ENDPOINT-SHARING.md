---
id: KERNEL-J-NONREFL-ENDPOINT-SHARING
title: "A non-refl J reduction re-reduces the level below about eight times, through j_endpoints, the type-equality witness and the Cast arm's components, so a source-derived chain of J over step_i helpers costs work exponential in its depth. Make whnf of that chain linear in depth by reducing each endpoint once, with the J reduct's shape unchanged"
status: active
owner: kernel
size: M
tier: T1
gate: architect
depends_on: [KERNEL-OBS-NESTED-CAST-LINEAR]
blocks: []
github: null
origin: "Architect evt_1k4yc761p6rty on the KERNEL-OBS-NESTED-CAST-LINEAR site D0 (evt_3maz6hzwmq32r), disposition (a): the S residual is a different mechanism from the cast-regularity repeat. Steward-filed per COORDINATION section 2."
---

# One reduction per J endpoint

## Objective

Kernel `check` of the source-derived nested-J fixture S costs reducer work
linear in its depth k, with the verdict it gives today.

## Settled inputs (D0 `evt_3maz6hzwmq32r`, on `10daa9242` plus N0–N2)

- **The fixture.** S is `KERNEL-OBS-NESTED-CAST-LINEAR`'s source-derived
  `step_i` / `J (λx _. x) t e` chain. It checks `Refl Lk : Eq Type0 Lk
  (type_id Type0 Lk)`, and its k=4 verdict pin lands with that WP.
- **The growth.** Total reducer entries are 8,531, 69,808, 560,397 and
  4,485,482 at k = 2 to 5, about ×8 per level. `j_nonrefl` calls are 54,
  438, 3,510 and 28,086, at the same ratio.
- **No single site repeats.** Per `j_nonrefl` call, each site's cost is
  constant at every k: A (the Cast arm's components) about 6.6, D
  (`j_endpoints`) about 4.5, E (the type-equality witness) about 148.
  Cast regularity and `cast_at_inductive` are 0 with N0–N2 in place.
- **The mechanism** (Architect `evt_1k4yc761p6rty`). One non-refl J
  reduction re-reduces the level below about 8 times, through D, E and A.
  - `j_nonrefl` is at `obs.rs:1207` (spec 15 §4.3).
  - `type_eq_by_j` and `type_eq_by_j_with_base` each `infer(source)`.
  - `canonical_type_eq_base` decides whether `Eq Type X X` is neutral or
    Top.
  - The Cast arm then reduces `p_a_refl` and `p_b_e`, which `j_nonrefl`
    built unreduced.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Whnf of S costs work linear in k. Each endpoint of a non-refl J reduct is
reduced once, and the reduct is built from the reduced forms with its shape
unchanged. The Architect rules the repair after AC-0.

## Acceptance

- **AC-0 (parent attribution; measure only).** For each `j_nonrefl` call
  at level i−1, name the site of the level-i reduction that encloses it.
  The ×8 must come out as a sum over D, E and A, at k = 2 to 4. The
  Architect rules the repair from that table, before any edit.
- **AC-1.**
  - A committed pin bounds S's entries by `c·k` at k = 4, 8 and 16.
  - Its verdict equals today's.
  - The k=4 pin from `KERNEL-OBS-NESTED-CAST-LINEAR` becomes a bounded pin.
- **AC-2 (controls).**
  - Reverting the repair reddens the S pin.
  - The T1 and T2 pins and their N1/N2 reverts keep their results.
  - The J-refl rule and a J left neutral at an open index keep their
    results.
  - The 57-package census shows no verdict change, and `trusted_base()` is
    equal.

## Stop conditions

- **A repair that changes the J reduct's shape.** Two examples: building
  the witness without typing it, or taking the universe level from the
  motive instead of `infer(source)`. Either is a change to the J reduction
  rule's implementation and an operator question before any build.
- Any fuel, cache keyed on term identity, or depth cutoff.
- Any change to what the kernel accepts: stop to the Architect.
