---
id: LANG-LET-INTRODUCTION-FORM-ASCRIPTION
title: "An annotated let whose value is an introduction form is refused when its body applies it: the kernel's Let rule checks the body with the value substituted, so let f : T = λu. e in f a becomes the unannotated redex (λu. e) a, and the kernel cannot infer it. Keep the annotation on the substituted value"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-LET-ANNOTATION-MOVED-FIELD-COORDINATE]
blocks: []
github: null
origin: "Routed out of LANG-LET-ANNOTATION-MOVED-FIELD-COORDINATE by the Architect's D0 evt_qd02djxjvf70 (r2pi's third mechanism). Steward-filed per COORDINATION section 2."
---

# An annotated let value keeps its annotation

## Objective

An annotated `let` binding a lambda (or another introduction form) whose
body applies it elaborates, in a match arm or out of one.

## Settled inputs (Architect `evt_qd02djxjvf70`, on `a04ec5865`)

- **Repro, with no match.**
  `fn t (x : Nat) : Nat = let f : (u : Nat) → Nat = λu. x in f Zero` is
  refused with "cannot infer an introduction form". It also reproduces in
  a DualFZ arm with no moved field, and as r2pi from the
  LANG-LET-ANNOTATION-MOVED-FIELD-COORDINATE fixture.
- **Mechanism.** The kernel's `Let` rule (check.rs:398 infer, :580 check)
  checks `subst0(body, val)`. Substituting an unannotated lambda for `f`
  turns `f a` into the redex `(λu. e) a`, which the kernel cannot infer.
- **Candidate repair (not yet ruled).** The elaborator emits the let
  value as `Ascript(val, ty)` when it is an introduction form. The kernel
  infers through `Ascript`, so the change is elaborator-only.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first: the Architect rules the repair on the landed base. The ruling
names the introduction forms it covers, where the value is wrapped, and
whether the stored term or `trusted_base()` changes for lets that check
today. Then implement it.

## Acceptance

- **AC-1.** The no-match repro and r2pi are Ok, with `trusted_base()`
  unchanged.
- **AC-2.** An annotated let used without application, an unannotated
  let, and a let of a non-introduction value check as before. The let
  suites `let5_checking_mode_let` and
  `lang_let_annotation_moved_field_coordinate` stay green.
- **AC-3 (mutation, QA).** Removing the ascription returns the no-match
  repro to "cannot infer an introduction form".

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
- The repair changes the stored term of a let that checks today, beyond
  the ascription the D0 rules.
