---
id: LANG-REFINED-RECURSION-LET-BOUND-CALL
title: "Dependent-result structural recursion on a refined parameter is still refused with K0014 when the recursive call is bound by an un-ascribed let, though base b22a41649 accepted it: the refined-worker plan admits the induced projection of the coerced recursive argument only inside the call's own result ascription, not after it has flowed into the let's inferred type. Admit it there too"
status: merged
owner: language
size: S
tier: T1
gate: architect
depends_on: [KERNEL-SCT-GROUP-TRANSPARENT-COMPANIONS]
blocks: []
github: null
origin: "Adversary M8 finding evt_4fyd0ntdxvmbj on KERNEL-SCT-GROUP-TRANSPARENT-COMPANIONS (a0311a8a7): an accept-to-refuse sibling of the four rows that a0311a8a7 restored. It fails closed. Steward-filed per COORDINATION section 2."
---

# Let-bound recursive calls in dependent-result recursion

## Objective

With `def AnyNat = { x : Nat | Top }`, each of these is accepted with
the 3 obligations it had on `b22a41649`:

- `fn g (n : AnyNat) : { y : Nat | Equal Nat y n } = match n { Zero ↦
  Zero; Suc m ↦ let r = g m in Suc r }`;
- the same with `: Nat ensures Equal Nat result n`;
- the same with the arm `let r = g m in r`.

## Settled inputs (main `a0311a8a7`, Adversary `evt_4fyd0ntdxvmbj`)

- **The rows.** All three are OK with 3 obligations on `b22a41649`, and
  K0014 ("idempotent self-loop has no strictly-decreasing parameter") on
  both `a0311a8a7^` and `a0311a8a7`.
- **The mechanism, measured.** The only refusal is the `Proj1` arm in
  `refined_recursion.rs` (~:213) on `((@3, g2) : g648)`. That is the
  projection of the coerced recursive argument, now sitting in the let's
  inferred type `{ y : Nat | Equal Nat y (m, proof).1 }`. The `Let` arm
  (~:229) walks `ty` under the enclosing mode, `Induced(vec![])` from the
  pre-check (~:383 to :388), so the plan fails. The fallback (~:500)
  then admits the original bodies, which SCT refuses.
- **Controls, already green on `a0311a8a7`.** The ascribed `let r : Nat =
  g m in Suc r` gives 3 obligations, a plain `Nat` result gives 1, and
  `Suc (g m)` (d2s) is accepted.
- **Kept refusals.** `g (g m)`, `g (Suc (g m))` and a swap that keeps `n`
  stay K0014.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The worker plan admits the induced projection of a coerced recursive
argument wherever that redex has flowed into a binder's type. The three
rows join `lang_refined_parameter_dependent_result.rs`.

## Acceptance

- **AC-0 (route, Architect).** Rule the admission criterion. The rule
  must say whether a `match` motive or a Pi or Lam domain carrying the
  inferred type is in the same class, and the WP covers whichever ones
  the ruling places in it.
- **AC-1.** The three rows give 3 obligations each, the four
  `a0311a8a7` rows and the three controls are unchanged, and
  `trusted_base()` is unchanged apart from the open holes.
- **AC-2.** The three kept refusals stay K0014, and so does a source
  literal-pair projection in a let type.
- **AC-3 (mutation, QA).** Restoring the enclosing mode at the `Let` arm
  reddens the three rows back to K0014.

## Amendment (Steward, 2026-10-23, `evt_6m9k4hzdj295`)

On the Architect's AC-0 ruling `evt_3vsjks22y4pnd`, the frame now names
these rows. Where it conflicts with the sections above, this amendment
wins.

- **Rule.** A saturated member call's refined arguments are inherited
  inward by the Let arm and by the Ascript arm's Induced branch. The
  change is in `refined_recursion.rs` only.
- **AC-1, added.**
  - 3 obligations: `let r = g m in let s = r in Suc s` and `let r = g m
    in let q = Suc r in q`.
  - 2 obligations: `let r = g (m, Axiom) in Suc r`, the same class as
    main's accepted ascribed form.
- **AC-2, added.** These stay K0014:
  - the let-bound forms of the kept refusals;
  - `let r = g m in let q = ((m, Axiom) : AnyNat).1 in Suc r` and its
    `(Suc m, Axiom)` twin.
- **AC-3.** Two seams: the Let arm's mode, and the Ascript arm's reset.
- **Ascription strip.** The outer strip on the call term is kept only if
  a named row needs it.

## Stop conditions

- Any kernel or `trusted_base()` change.
- A changed SCT verdict outside the rows named here.

Checks: 4 fired: the `Proj1` arm, the `Let` arm, the
`Induced(vec![])` pre-check and the `admit_pending` fallback are on
`a0311a8a7` at the lines cited. 7 fired: AC-2 keeps the nested and
swapped recursion that an admit-every-projection rule would wrongly
accept.
