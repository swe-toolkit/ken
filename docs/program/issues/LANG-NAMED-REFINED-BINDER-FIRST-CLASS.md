---
id: LANG-NAMED-REFINED-BINDER-FIRST-CLASS
title: "A function whose binder is a named refinement, used first-class, skips its obligation: fn take5 (x : Five) : Int = use5 x; fn apply (g : Int → Int) : Int = g six; const observed : Int = apply take5 checks with zero obligations, because the same-root reuse exemption trusts the binder as already introduced. Its literal twin emits the obligation inside take5. Introduce or refuse"
status: closed
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-NAMED-REFINEMENT-TYPE-ARGUMENT]
blocks: []
github: null
origin: "F-H: Architect evt_7zj4g45k4t6dn, found while ruling LANG-NAMED-REFINEMENT-TYPE-ARGUMENT's D0 on 6b938b70b and filed separately from it. Fails open: a program the refinement forbids is accepted. No kernel impact: predicates are erased and the kernel term stays well-typed. Steward-filed per COORDINATION section 2."
---

# A named-refinement binder used first-class keeps its obligation

## Objective

A function whose parameter is annotated with a named refinement either
carries that predicate to every caller, including a first-class use where it
is passed as a function value, or emits the obligation inside its body as
the literal spelling does.

## Settled inputs (Architect `evt_7zj4g45k4t6dn`, on `6b938b70b`)

- **The row.** With `const six : Int = 6; def Five = { x : Int | Equal Int x
  5 }; fn use5 (x : Five) : Int = x`, the program `fn take5 (x : Five) : Int
  = use5 x; fn apply (g : Int → Int) : Int = g six; const observed : Int =
  apply take5` is accepted with 0 obligations.
- **Literal twin.** With `x : { y : Int | Equal Int y 5 }`, `take5` emits 1
  obligation inside its body, so the literal form fails closed.
- **Mechanism.** The same-root reuse exemption treats a value typed by the
  binder's named refinement as already introduced. That holds at a direct
  call, where the caller is charged. It fails when `take5` is passed where
  `Int → Int` is expected, because conversion through the carrier charges
  no one.
- **Shared residual.** The parameter positions that
  `LANG-NAMED-REFINEMENT-TYPE-ARGUMENT` admits, P13 `(k : Five → Int)` and
  Derived's `TrueBool → Bool`, share this residual with every signature
  binder.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **D0 (measure only).** Every route by which a function with a
   named-refinement binder reaches a position typed by its carrier: a
   first-class argument, a let, a record or data field, and a class method.
   Census those uses across `catalog/`, `library/`, `crates/*/tests`,
   `examples/` and `conformance/`. The Architect rules whether the
   obligation moves into the body or the first-class use is refused.
2. **The ruled closure.**

## Acceptance

- **AC-1.** The row emits its obligation, or is refused with a message
  naming the binder, per the ruling.
- **AC-2 (controls).** A direct call `take5 six` keeps exactly 1 obligation
  at the caller. The literal twin is unchanged. The corpus census shows no
  newly refused or undischargeable program beyond what the ruling names.
- **AC-3 (mutation, QA).** Removing the closure returns the row to zero
  obligations.

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A catalog, library or example program that relies on the first-class use.

## Closeout

Closed before release, superseded by `LANG-REFINEMENT-SUBSET-SIGMA`
(operator 2026-10-07; Architect `evt_30frdrmj45ehg`). Subset Σ closes the
class structurally, and this WP's rows are acceptance rows there.
