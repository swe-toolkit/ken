---
id: SPEC-CORE-LET-TYPING
title: "Spec 18 has no typing rule for the core Let former the kernel already checks in both modes. State the existing infer and check rules, and dispatch cases, so the contract matches landed acceptance; then return the ζ conformance case"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-02 ('approve 1 and 2 as recommended'): option (a) of spec-author evt_z5scwvtpbzck, raised by the conformance validator's block of d0ae7c2a8 (dec_3qwsee14arhgz) on KERNEL-REFL-ENDPOINT-TYPED-CONVERSION's ζ case. Steward-filed per COORDINATION section 2."
---

# Spec 18 types core Let as the kernel does

## Objective

`spec/10-kernel/18-judgments.md` states the typing of core `Let` in infer
and check mode, as the kernel implements it. No accepted program and no
reduction changes.

## Settled inputs (spec-author `evt_z5scwvtpbzck`, read at `e893ecb7a`)

- **The gap.** Core `Let` is a binder and core former in 11 §1, and 17
  §1/§3.2 gives its non-recursive ζ, which 17 §5's termination argument
  counts. Spec 18 §2-3 gives it no rule and no dispatch case. 39 §6 lowers
  surface `RLet` to `Let(A, rhs', body')`.
- **The kernel, which this node specifies and does not change.**
  - `infer(Let{ty, val, body})` (`check.rs:307-311`): `classify(ty)`, which
    admits a Type- or Ω-classified type; then `check(val, ty)`; then
    `infer(subst0(body, val))` in the original context.
  - `check(Let, expected)` (`check.rs:473-482`): the same two guards, then
    `check(subst0(body, val), expected)` directly. This is not the generic
    infer-then-convert fallback (`:524-537`), so check mode accepts a let
    whose substituted body is an introduction form that cannot infer alone
    (pinned at `check.rs:2234-2250`).
  - Errors: a malformed `ty`, a wrong RHS, or a body that fails its
    substituted infer or check. A valid let checked against a wrong outer
    type gives `TypeMismatch` (`:2213-2223`). In infer mode, a substituted
    non-inferable introduction form gets the existing "cannot infer an
    introduction form" error (`:360-369`).
  - Raw well-formedness treats `Let` as one binder (`check.rs:52-56`).
  - WHNF and normalization substitute ζ (`conv.rs:385-388`, `:572-575`).
- **The rule must describe the substituted body.** A rule that types the
  body as `infer Γ, x:A ⊢ body` would reject check-mode cases the kernel
  admits. That is a behaviour change, which this node does not make.
- **Consumers that stay accepted:** `let5_checking_mode_let.rs:14-48` and
  `conformance/surface/elaboration/seed-multi-binding-let.md:260-285`,
  `:320-341`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **Spec 18.**
   - §2 gains the Let rule: classify `A`, check the RHS at `A`, and type
     the capture-avoiding substitution `body[val/0]` in `Γ`.
   - §3.1 `infer` and §3.2 `check` gain their Let dispatch cases. Check
     mode checks the substituted body against the expected type, which
     keeps checking-mode introduction.
   - The algorithm and API coverage statements in §3-§4 stay truthful.
2. **The ζ conformance case.** It was carried from
   `KERNEL-LEQ-INT-LITERAL-REDUCTION` and removed from
   `KERNEL-REFL-ENDPOINT-TYPED-CONVERSION`. It returns here as a
   conformance seed, grounded in the new rule, with its spelling corrected.

## Acceptance

- **AC-1.** The rule and both dispatch cases are stated in spec vocabulary.
  Each is cited to the kernel arm it describes.
- **AC-2.** The ζ seed:
  - an accepted let in infer mode, with the inferred type of the
    substituted body;
  - an accepted check-mode let whose substituted body is an introduction
    form that cannot infer alone;
  - a let checked against a wrong outer type, rejected with
    `TypeMismatch`;
  - a let whose RHS does not check at its annotation, rejected.
- **AC-3.** The conformance validator votes on the exact tip, and the
  Architect approves.

## Stop conditions

- Any kernel, elaborator or `trusted_base()` change.
- A seeded case that the current kernel does not decide as AC-2 states.
  Stop and report it; do not change the rule to fit.

## Closeout

Merged `b540ebb0f` (PR #4451), exact `e27d5a5f8`: conformance validator
`evt_77gnvcxsy71c5`, Architect `evt_3bpahwyabbz5y`, Decision
`dec_6adymtz3csp5a`.

- Spec 18 §2 states the core `Let` rule: classify `A`, check the RHS at `A`,
  and type `body[val/0]` in `Γ`. §3.1 and §3.2 give the infer and check
  dispatch cases, each cited to its kernel arm.
- The ζ conformance seed is in `conformance/kernel/conversion/seed-conversion.md`.
  That discharges the ζ carry in the `KERNEL-REFL-ENDPOINT-TYPED-CONVERSION`
  closeout.
