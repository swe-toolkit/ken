---
id: KERNEL-QUOT-FORM-EQUIVALENCE
title: "Quot-Form accepts any relation at any Omega level, which is what made quotient equality unsound and lets its reduct sit above the Eq's level. Make Quot-Form take an IsEquiv proof with the relation at the carrier's level, and restore Eq at quotient classes reducing to the relation"
status: ready
owner: kernel
size: M
tier: T1
gate: architect
depends_on: [KERNEL-QUOT-EQ-INTERIM-CAST-LEVEL]
blocks: []
github: null
origin: "Operator 2026-10-03 (evt_6d6fzbqt2sa8r): P0 approved. Architect ruling evt_4hjm341apdy5t (thr_v8ehjnn76p69), closing the quotient inconsistency evt_2dz39cdyfdmh7 and U1. Steward-filed per COORDINATION section 2."
---

# Quot-Form takes an equivalence proof

## Objective

`A / R / e` is formed only with `e : IsEquiv A R` and `R : A → A → Ω_l` at
the carrier's level `l`, and `Eq (A/R) [a] [b]` reduces to `R a b` again.

## Settled inputs (Architect `evt_4hjm341apdy5t`)

- The rule, in kernel vocabulary, is in the ruling (the OTT shape, Altenkirch,
  McBride and Swierstra 2007; relation level as TT^obs §4.1). `IsEquiv A R`
  is the conjunction of reflexivity, symmetry and transitivity at `Ω_l`.
- `Term::Quot` gains the third field `e`. Conversion compares `A` and `R`
  and skips `e` (Ω proof irrelevance). `check_quotient_rel` requires
  `whnf(cod2) = Ω_l` with `l ≡ synth_type(A)`.
- Consumers (grep of `Term::Quot(` and the `"quot"` tag, perishable):
  checked_core encode/decode `:2501`, `:4096`, `:4514`, `:4604`; elab.rs
  `:213`, `:2065`, `:2157`, `:2681`, `:2828`, `:7873`, `:7992`, `:15795`;
  fo_kripke.rs `:1247`; ken-interp eval.rs; 8 test files. The catalog has 0
  quotients.
- The interim (`KERNEL-QUOT-EQ-INTERIM-CAST-LEVEL`) made quotient equality
  neutral; this WP restores the reduction, which is sound once `e` exists.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. Spec 16 §5 Quot-Form and §2.2's quotient rule; §8.4's upward quotient arm
   marked closed; conformance rows follow.
2. Kernel, checked core, elaborator and interpreter carry `e`; the surface
   quotient form requires it.
3. `eq_at_quot` reduces at class pairs again.

## Acceptance

- **AC-1.** The interim's `0 = 1` probe stays refused, now at formation
  (the relation `λx y. Eq Nat 0 1` has no `IsEquiv` proof); a quotient by a
  real equivalence reduces `Eq (A/R) [a] [b]` to `R a b`. A relation at
  `Ω_{l+1}` over `A : Type l` is refused. `trusted_base()` delta is the
  formation side condition only.
- **AC-2 (falsifiers).** M1: skip the `IsEquiv` check; a pin forming the
  `0 = 1` quotient reddens. M2: accept `Ω(_)` without the level equation;
  the level pin reddens.

## Stop conditions

- A consumer outside the listed sites needs semantic change, not a third
  child: stop to the Architect.
