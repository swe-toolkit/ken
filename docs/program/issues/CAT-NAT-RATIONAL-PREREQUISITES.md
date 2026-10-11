---
id: CAT-NAT-RATIONAL-PREREQUISITES
title: "Data.Numeric.Rational's proofs need natural multiplication laws that Data.Numeric.Nat.Arithmetic keeps private, plus a positive-factor cancellation and two order transports that do not exist. Deliver section 4 of spec/50-stdlib/64-rational.md: six existing laws made public under their identities, three new checked laws, at zero trust"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-NUMERIC-RATIONAL-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Section 4 of the rational contract (SPEC-NUMERIC-RATIONAL-CONTRACT, exact 1a944dd98, Architect evt_5k6fz5fqxnqgm) names these as Nat provider obligations, never re-proved in the Rational package. First of two CAT nodes; Data.Numeric.Rational follows. Steward-filed per COORDINATION section 2."
---

# Natural laws the rationals need

## Objective

The Nat providers export every law section 4 of
`spec/50-stdlib/64-rational.md` requires, so the Rational package proves
its laws from them rather than re-proving or assuming them.

## Settled inputs (on `f2c439e5c`, spec at `1a944dd98`)

- **Existing and private** in `Data.Numeric.Nat.Arithmetic`: `zero_l`,
  `one_l`, `one_r`, `comm` and `assoc` for `mul`, and
  `mul_add_distrib_l` (`(a + b) c = a c + b c`). Section 4 makes these
  six public under their existing identities: no redefinition of `mul`,
  no competing proof, no second Nat class dictionary. `mul::suc_l` stays
  private and may support the new proofs.
- **New:** `mul::cancel_r` (equality cancellation by `Suc k`) in
  Arithmetic, checked in the spec's scratch development. In
  `Data.Numeric.Nat.Order`, `leq_nat_mul_right_mono` and
  `leq_nat_mul_right_cancel` over the canonical `leq_nat`. **Neither order
  law has been checked yet.**
- **Consumers:** 39 files across `catalog/`, `crates/*/tests`, `examples/`
  and `conformance/` name `Data.Numeric.Nat.Arithmetic`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Section 4's surface in the two Nat packages, each law a checked theorem,
with acceptance rows for the three new laws.

## Acceptance

- **AC-1.** `ken fmt --check` and `ken check` pass. An acceptance target
  root-loads the nine section 4 names from a separate package and uses
  each once.
- **AC-2.** `trusted_base()` is equal before and after. No Axiom,
  postulate or primitive.
- **AC-3 (consumers).** The 39 importers stay green. Sweep them by
  mechanism for a local declaration that a newly public name would now
  collide with or shadow, and report the count.
- **AC-4 (mutation, QA).** Dropping the `Suc` from `cancel_r`'s factor,
  or flipping either order law's direction, makes a proof kernel-reject.

## Stop conditions

- Either order law needs an Axiom, a kernel or surface change, or a
  public-surface change outside the two Nat packages.
- Making a law public changes an existing proof's identity or breaks an
  importer's public surface: stop with the list.

## Closeout

Merged `8f27e6893` from exact `f17fbe4f8` (PR #4679). Foundation QA
`evt_6d4c3bdtdd54d`, Architect `evt_4qzrvzgqexe78`, Decision
`dec_6f9a776sqzznq`.

- `Data.Numeric.Nat.Arithmetic` publishes six existing multiplication
  laws in place under their identities and adds `mul::cancel_r`. Public
  laws lead their private helpers, top-down.
- `Data.Numeric.Nat.Order` adds `leq_nat_mul_right_mono` and
  `leq_nat_mul_right_cancel`.
- The acceptance target root-loads the nine names. `trusted_base()` is
  114 before and after, and the 42 importers of the arithmetic package
  show no collision. §1a: 0.

`CAT-NUMERIC-RATIONAL-VALUES` consumes these laws.
