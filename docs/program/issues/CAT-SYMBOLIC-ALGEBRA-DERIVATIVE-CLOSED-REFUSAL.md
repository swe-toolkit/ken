---
id: CAT-SYMBOLIC-ALGEBRA-DERIVATIVE-CLOSED-REFUSAL
title: "The derivative seed's refusal row claims d/dx(x·x) = x at every open environment with Proved, which the kernel refuses whether the claim is true or false, so the row cannot fail under a broken product rule. Replace it with a closed env_two refusal that a dropped product-rule summand would accept"
status: active
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [CAT-SYMBOLIC-ALGEBRA-DERIVATIVE]
blocks: []
github: null
origin: "Adversary M8 finding evt_6pdhd08fvnsqy (F1) on CAT-SYMBOLIC-ALGEBRA-DERIVATIVE (3da033384): a negative row that cannot fail. Check 8. Steward-filed per COORDINATION section 2."
---

# A derivative refusal that a broken product rule would accept

## Objective

The derivative seed's refused block fails only because the claim is
false. A product rule that drops a summand makes the same block accept.

## Fixed inputs (main `3da033384`, Adversary `evt_6pdhd08fvnsqy`)

- **The row.** The seed
  `conformance/stdlib/symbolic-algebra/seed-nat-polynomial-derivative.md`
  (~:171, theorem ~:174) refuses `(env : Nat → Nat) → Equal Nat (eval env
  (deriv Zero square)) (eval env x)` witnessed by `Proved`. The pin is
  `crates/ken-elaborator/tests/cat_symbolic_algebra_derivative.rs`
  ~:198-202 and ~:277.
- **Measured by the Adversary in a disposable test:**
  - Over an open `env`, `eval` does not reduce, so `Proved` is refused
    with TypeMismatch for the shipped claim.
  - It is refused just the same for a true claim (`deriv Zero square`
    against `doubled`).
  - It is also refused for the claim under either client-local derivative
    that drops one product-rule summand, although those claims are true
    at every env (witnessed by `poly_eq_sound`).
  - The closed claim `Equal Nat (eval env_two (deriv Zero square))
    (eval env_two x) = Proved` is refused by the shipped `deriv` and
    accepted under either summand-dropping derivative.
- **Already guarded:** the five-valued closed oracles, the
  `square_derivative_is_four_at_two` pin, and `deriv_taylor` go red
  under a broken product rule. This WP fixes only the refusal row and
  its text.

Treat anchors as perishable. If a fixed input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The seed's refused block becomes the closed `env_two` claim. The seed's
"why" text and the pin's MEASURED/CLAIMED describe what the row
discriminates. The package source is unchanged. The seed edit goes
through the CV for fidelity, as with `579f38801`.

## Acceptance

- **AC-1.** The new refused block is kernel-refused at its own
  equality. The positive client and every other pin are unchanged and
  green.
- **AC-2.** The pin also checks the closed claim under a client-local
  derivative that drops one product-rule summand, and that claim is
  accepted. This is the configuration the refusal excludes.
- **AC-3 (mutation, QA).** Each one-summand rule (left and right) turns
  the refused block into an accepted one.

## Stop conditions

- Any change to `NatPolynomial.ken.md` or `trusted_base()`.
- A change to spec 66 §5's public laws.

Checks: 8 fired: the open-env refusal is the configuration that gives
the same observation with a true or a false claim, so AC-2 pins the
discriminating counter-configuration. 7 fired: both summand drops are
run in AC-3, not one.
