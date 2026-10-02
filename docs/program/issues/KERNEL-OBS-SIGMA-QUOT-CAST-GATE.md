---
id: KERNEL-OBS-SIGMA-QUOT-CAST-GATE
title: "The Σ and quotient casts fire when their Eq Type stays neutral and project e.1 and e.2 from a neutral e, so their reducts do not check (NotASigma, TypeMismatch). Gate them as cast_at_pi is gated, so they stay neutral, as spec 16 §3.2 requires"
status: ready
owner: kernel
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary M8 finding 2 evt_1rnwn0jc09jw9 on 884f493fe (KERNEL-OBS-TYPE-EQ-STRUCTURAL): pre-existing, fails closed. The Π sibling was gated by that WP. Operator approved 2026-10-02 ('approve 1 and 2 as recommended'): next on the kernel ring after KERNEL-OBS-NESTED-CAST-LINEAR, ahead of KERNEL-J-NONREFL-ENDPOINT-SHARING. Steward-filed per COORDINATION section 2."
---

# Σ and quotient casts stay neutral on a neutral equality

## Objective

Spec 16 §3.2: a Σ or quotient cast is neutral when its sub-equalities are
neutral. Every compound-former cast that fires gives a reduct that checks.

## Settled inputs (Adversary `evt_1rnwn0jc09jw9`, read at `884f493fe`)

- TYPE-EQ-STRUCTURAL gated `cast_at_pi` on its `Eq Type` arm
  (`obs.rs:799-809`), so a Π whose arm returns `None` stays stuck.
- `cast_reduce` still calls `cast_at_sigma` unconditionally (`obs.rs:779`,
  `:843`), and `cast_at_quot` whenever the arm returns `None` (`:785`,
  `:1156`). Each reduct projects `e.1`/`e.2` from a neutral `e`.
- With `e : Eq Type A B`, four redexes infer while their reducts fail
  `check`: subset Σ (`NotASigma`), a quotient with relation levels `Ω0` vs
  `Ω1` (`NotASigma`), a Σ with an Ω-sorted first component (`TypeMismatch`),
  and a Σ with different domain levels (`NotASigma`). They fire at the parent
  too.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

`cast_at_sigma` and `cast_at_quot` fire only when the `Eq Type` arm yields the
decomposition they project, by the same gate as `cast_at_pi`. Otherwise the
cast is stuck.

## Acceptance

- **AC-1.** The four repros are stuck, not reduced.
- **AC-2 (controls).**
  - A Σ and a quotient cast whose equality decomposes still reduce, and their
    reducts check against the redex's type.
  - The Π gate rows are unchanged.
  - Removing either gate reddens its AC-1 rows.
- **AC-3.** `trusted_base()` is unchanged, the catalog census has no verdict
  change, and the cast and TYPE-EQ rows stay green.

## Stop conditions

- A cast that fires today on a decomposing equality would be stuck: stop to
  the Architect.
- A spec change.
