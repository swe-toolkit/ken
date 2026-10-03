---
id: KERNEL-QUOT-EQ-INTERIM-CAST-LEVEL
title: "The kernel proves Eq Nat 0 1, because Quot-Form takes any relation and Eq at quotient classes reduces to it, and it admits a cross-level cast, because Cast never compares the two levels. Make Eq at quotient classes neutral until Quot-Form carries an equivalence proof, and refuse a cast whose types sit at different levels"
status: active
owner: kernel
size: S
tier: T1
gate: architect
depends_on: [KERNEL-INT-DIV-MOD-NATIVE]
blocks: [KERNEL-QUOT-FORM-EQUIVALENCE]
github: null
origin: "Operator 2026-10-03 (evt_6d6fzbqt2sa8r): soundness plan approved, interim close plus P2 right after KERNEL-INT-DIV-MOD-NATIVE. Architect ruling evt_4hjm341apdy5t (thr_v8ehjnn76p69) on Research evt_7njjw249ab1pd and the Architect's quotient finding evt_2dz39cdyfdmh7. Steward-filed per COORDINATION section 2."
---

# Close the quotient inconsistency and the cross-level cast

## Objective

No closed term proves `Eq Nat 0 1` through a quotient, and `cast A B e a`
is refused when `A` and `B` sit at different levels. This is the interim
close; `KERNEL-QUOT-FORM-EQUIVALENCE` (P0) restores quotient equality
reduction once Quot-Form carries an equivalence proof.

## Settled inputs (Architect `evt_4hjm341apdy5t`)

- **The inconsistency.** Quot-Form (spec 16 §5) does not require `R` to be
  an equivalence. `Eq (A/R) [a] [b] ⇝ R a b` (16 §2.2, `obs.rs:129` and
  `eq_at_quot` `:350`), so `refl [a]` proves `R a a`; with `R := λx y. Eq
  Nat 0 1` the kernel accepts a closed proof of `Eq Nat 0 1` (probe on
  `948864d3e`).
- **Interim fix.** `eq_at_quot` stays neutral at every pair of classes: Eq
  at a quotient class does not reduce. Sound but incomplete; the Architect
  measured that only tests are affected and the catalog has 0 quotients.
- **P2.** Spec 16 §3.1 already requires `A : Type l` and `B : Type l` at one
  `l`. `check.rs:332-342` synthesizes `_l_b` and never compares it, and
  builds `eq_ty` at `Type l_a`, so Research's cross-level `cast S1 S0 (refl
  S1) p` is admitted. Add the level comparison, failing closed.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. Kernel: Eq at quotient classes is neutral; Cast refuses unequal levels.
2. Spec 16: §2.2's quotient rule and §5's Equality bullet state the interim
   (neutral until Quot-Form carries an equivalence proof); §8.4 and §9's C8
   row follow. No change to §3.1. §5.1's Type target gains the interim
   paragraph ruled in `evt_59v6y2svdkx63`. The schema is kept and checked
   where the transport collapses by regularity (`M [x] ≡ M [y]`, including
   every constant motive). A Type-target elimination whose respect needs a
   class equality is refused until P0: a dependent motive, or a motive into
   a quotient. The Ω-target rule is unchanged. §9's C15 row splits:
   constant-motive accept and refuse are unchanged; the dependent-motive
   accept is deferred to P0 with interim verdict refused; `r_bad` stays
   refused. §9's numbered item 4 ("relation-as-equality") states neutral
   class Eq until P0 (spec-author `evt_1tkny60qttp4r`).
3. Conformance: the `quotient-eq` rows (`seed-kernel.md`,
   `seed-observational.md`, `seed-conversion.md`
   `quotient-eq-through-conv`) state the interim verdict; a row pins the
   `0 = 1` probe refused and one pins the cross-level cast refused.
   `quotient-respect-schema-dependent-motive` restates `given` without the
   reduction-derived `h'`, pins refused for both `r_ok` and `r_bad`, and
   marks the accept half as restored by P0. The constant-motive
   `quotient-respect-schema-{rejects,accepts}-…` rows are unchanged.
   `seed-evaluation.md`'s `can-eq-by-type-computes` and
   `agree-observational-corpus` drop the quotient-class reduction claim
   (kernel-implementer `evt_47tab4rs3fm50`). They mark the quotient-class
   `Eq` case deferred to P0 and pin no X1 verdict (kernel QA
   `evt_79844c7p7041x`). X1's `eq_reduce` returns `Unknown` there, not a
   neutral value; it never computed the old reduct either. Do not claim
   cross-engine agreement, and do not substitute `unknown` (spec 42 §4
   reserves it for holes).
4. Test consumers of the reduction are migrated (check 3: grep
   `crates/*/tests`, `conformance/`, `examples/`, `catalog/` for quotient
   equality uses, not only `eq_at_quot`).

## Acceptance

- **AC-1.** The `0 = 1` probe is refused, and `cast A B e t` at equal levels
  checks unchanged. Research's constructor-field route (`evt_7daqm6ydmbnsr`
  S2: `R : Bool → Nat → Ω_0`, `data Bx = bx (Bool/R)`, `refl (bx [true]) :
  Bottom`; and the non-reflexive `R x y := Eq Bool x false` twin) is refused.
  `eq_at_inductive` passes raw constructor arguments into the conjunct, so
  a direct `refl [a]` probe alone does not reach the arm. `trusted_base()`
  has no delta.
- **AC-2 (falsifiers; each must redden).** M1: restore the `QuotClass` pair
  arm in `eq_at_quot`; the `0 = 1` pin reddens. M2: delete the level
  comparison; the cross-level pin reddens and the equal-level pin stays
  green.

## Stop conditions

- A non-test path depends on quotient equality reducing: stop to the
  Architect with the site. §5.1 is ruled (`evt_59v6y2svdkx63`):
  `infer_quot_elim` (`check.rs:909-941`) forms no `h'`, so no kernel code
  changes for it. The spec prose is the only consumer, handled in
  deliverable 2.
- A catalog or `examples/` program stops checking.
