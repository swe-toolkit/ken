---
id: LANG-ASCRIPTION-INTRODUCTION-INFERENCE
title: "An ascribed introduction form in infer position is refused by the kernel: infer_ascription erases the ascription after checking, so ((λu. x) : (u : Nat) → Nat) Zero reaches the kernel as the redex (λu. x) Zero, which it cannot infer. Keep the ascription on an introduction-form core, the criterion of the let fix, and amend spec 39 section 5.4"
status: ready
owner: language
size: S
tier: T2
gate: architect
depends_on: [LANG-LET-INTRODUCTION-FORM-ASCRIPTION]
blocks: []
github: null
origin: "Adversary evt_4svh2xfvvnr9f (sibling of LANG-LET-INTRODUCTION-FORM-ASCRIPTION on bdedaf437); Architect ruling evt_50wngwa9saa0h: real, repair at the producer, one S node with its own spec line. Steward-filed per COORDINATION section 2."
---

# An ascribed introduction form keeps its ascription

## Objective

An ascribed lambda, pair, `Refl`, quotient class or truncation in infer
position, such as an application head, elaborates to a term the kernel
infers.

## Settled inputs (Architect `evt_50wngwa9saa0h`, on `15661b590`)

- **Repro.** `fn u1 (x : Nat) : Nat = ((λu. x) : (u : Nat) → Nat) Zero`
  and its let-tail twin u2 `((let c : Nat = x in λu. c) : (u : Nat) →
  Nat) Zero` are KernelRejected with "cannot infer an introduction form
  ... (use ascription)". Wrong-type twins w2 `((λu. True) : (u : Nat) →
  Nat) Zero` and w3 (`: Bool` result) refuse with the same message
  rather than a type error.
- **Mechanism.** `infer_ascription` (elab.rs ~:11897) returns
  `(e_core, ty_core)`. `App(Lam, a)` reaches the kernel, which only
  checks introduction forms (check.rs:470-478).
- **Ruled code.** At the end of `infer_ascription`, return
  `Ok(ascribed_inference(e_core, ty_core))`, where the outlined
  `#[inline(never)] fn ascribed_inference` wraps the core with the landed
  `ascribe_introduction_let_value` (:1506). The one-line inline form
  fails `elab_recursive_frame_budget::ascription_infer_cycle_fits_stated_stack`;
  the outlined form passes.
- **Population.** 0 of 1003 infer-position ascriptions in the catalog
  and examples are introduction forms; 19 ascription suites are green
  under the prototype.
- **Spec.** Spec 39 §5.4's RAsc arm (39-elaboration.md:712) says
  "ascription erased after check". It gains the Architect's text: if
  `introTail(e')`, return `(Ascript(e', A), A)`; otherwise erase.
  `introTail` is λ / pair / refl / quotient class / truncation, through a
  `let` body. Kernel 11 §1 is unchanged.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The ruled code and the spec 39 §5.4 RAsc amendment in one candidate. The
Spec leader approves the spec line at the exact SHA; with spec/ in the
diff, the CV resolves the Decision.

## Acceptance

- **AC-1.** u1, u2 and the Adversary's exact u3 (`theorem u3 (x : Nat) :
  Equal Nat x x = ((λu. Refl) : (u : Nat) → Equal Nat x x) Zero`) are
  Ok, with `trusted_base()` unchanged. A `ken run` or native parity row
  on u1 agrees with the interpreter.
- **AC-2.** w2 and w3 refuse with a kernel `TypeMismatch`, not the
  intro-form text. The wrong-argument twin, the let-bound control, the
  unapplied ascription and a non-introduction ascription `(x : Nat)` are
  unchanged; that last core stays `Var`, not `Ascript`.
- **AC-3 (mutation, QA).** Removing the wrap returns u1 to
  KernelRejected with the intro-form message.
- **AC-4.** `elab_recursive_frame_budget` stays green.

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
- A change to an existing core-shape assertion outside the
  introduction-form population.
