---
id: RT-C6-NON-CLOSURE-FUNCTION-PARAMETER
title: "A closed, checked refl cast whose function parameter is an unapplied constructor on one side, such as CFn Suc to CFn (λn. Suc n), evaluates to Unknown while the kernel reduces it, because RT-C6's push-mode tolerance in values_agree covers only a Closure pair and a CtorPending value falls to the refusing arm. Extend the push to every function-value representation without comparing them"
status: active
owner: runtime
size: S
tier: T1
gate: architect
depends_on: [RT-C6-FUNCTION-INDEX-CONVERTIBLE-ENDPOINTS]
blocks: []
github: null
origin: "Adversary finding on RT-C6 squash 501647864 (evt_hgqqhjypcvy1), fails closed. The rows are RT-C6's own row-5 shape at a parameter, so the RT-C6 objective is not met for them. Serves the L1 objective. Steward-filed per COORDINATION section 2."
---

# A canonical cast at a non-closure function parameter computes

## Objective

A closed, checked refl cast whose function parameters are convertible
evaluates to its payload whatever runtime value represents each
function, as RT-C6 already does when both are closures.

## Settled inputs (main `a533a4dc7`)

- **The arm.** `values_agree`'s push-mode tolerance is
  `(EvalVal::Closure { .. }, EvalVal::Closure { .. }) => up_to_closures`,
  followed by `_ => false` (`crates/ken-interp/src/eval.rs:1627-1628`).
  The parameter gate calls `values_agree(p, q, true)` at `eval.rs:1409`.
- **The other function values.** An unapplied constructor evaluates to
  `EvalVal::CtorPending { args: [], need: 1, .. }` (`eval.rs:274`), and
  so does a partially applied primitive op (`eval.rs:2974`).
  `EvalVal::IhClosure` (`eval.rs:260`) has no `values_agree` arm either.
- **The rows (Adversary `evt_hgqqhjypcvy1`).** With
  `data CFn (f : Nat -> Nat) : Type where { MkCFn : CFn f }` and
  `fn idN (n : Nat) : Nat = n`, the kernel check passes and kernel whnf
  gives a constructor head, while the interpreter gives Unknown, for:
  - `cast (CFn Suc) (CFn (λn. Suc n)) (Refl _) (MkCFn Suc)`;
  - the reverse direction;
  - `tcast (CFn idN) (CFn Suc) axSuc (MkCFn idN)` with
    `axiom axSuc : Eq Type (CFn idN) (CFn Suc)`.
  The `Suc` to `Suc` and `(λn. Suc n)` to `(λn. Suc n)` controls
  compute.
- **Forbidden, as in RT-C6.** Comparing function values by
  representation or identity (spec 42 §1), and reading the proof value.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first: the Architect rules which function-value pairs the push
tolerates, including `IhClosure` and a mixed `CtorPending` and
`Closure` pair, and whether a `CtorPending` pair with equal ids is
compared or tolerated. Then the change in `eval.rs` and the rows in
`rt_c6_function_parameter_cast.rs`.

## Acceptance

- **AC-0 (D0, Architect).** The ruled arms, on the landed base.
- **AC-1.** The three rows above evaluate to `MkCFn` with the same head
  as kernel whnf; the target parameter applied to `Zero` gives
  `Suc Zero`. The two controls still compute.
- **AC-2.** RT-C6's rows, its `CDep` stuck twin and its
  comparable-distinct String, Float and Float32 pins are unchanged.
- **AC-3 (mutation, QA).** Restoring `_ => false` for the new arms
  reddens the three rows to Unknown and leaves the controls computing.

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A mechanism that compares function values by representation or
  identity.
- RT-C6's carried rows H and I (closure convertibility at the value
  layer) are out of scope.
