---
id: LANG-REFINED-RECURSION-LET-VALUE-CONTAINS-CALL
title: "Dependent-result structural recursion on a refined parameter is still refused with K0014 when an un-ascribed let binds a value that returns the recursive call through an inner let or match, rather than the call itself. The let-scope authority keys on the value's head being the call. Extend it to a value that returns the call"
status: active
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-REFINED-RECURSION-LET-BOUND-CALL]
blocks: []
github: null
origin: "Adversary M8 finding evt_5rxph9sdnkr14 (F1) on LANG-REFINED-RECURSION-LET-BOUND-CALL (6a00e978c): a sibling of the closed shapes. It fails closed, and the K0014 it reports is false. Steward-filed per COORDINATION section 2."
---

# A let whose value returns the recursive call

## Objective

With the `lang_refined_recursion_let_bound_call.rs` harness (`AnyNat`,
`R = { y : Nat | Equal Nat y n }`), these are accepted:

- `let r = (let s = g m in s) in Suc r`;
- the same step with result `Nat ensures Equal Nat result n`;
- `let r = (match m { Zero ↦ g m ; Suc k ↦ g m }) in Suc r`.

## Fixed inputs (main `6a00e978c`, Adversary `evt_5rxph9sdnkr14`)

- **The rows.** All three are refused with K0014 on `6a00e978c`. Each
  recurses only on `m < n`.
- **The mechanism, measured by the Adversary with instrumentation that
  was then reverted.** All three refuse in the `RefinedWorkerPlan::new`
  pre-check.
  - The Let arm computes authority as `mode.with_call(val, members)`
    (`refined_recursion.rs` ~:270).
  - `member_call_refined_args` (~:184) grants arguments only when the
    head of `peel_app(val)` is a member `Const`.
  - A `Let` or `Elim` value that returns the call still carries the
    call's projected argument into the outer let's inferred type, and
    that type is walked with empty authority.
- **Accepted controls on `6a00e978c`:**
  - `let r = g m in let s = r in Suc s`, 3 goals;
  - `Suc (match m { … ↦ g m })`, 4 goals;
  - `let r = (let s = g m in Suc s) in r`, 3 goals;
  - `let r = Suc (let s = g m in s) in r`, 3 goals.
- **Ruling in force:** the Architect's AC-0 for the parent WP,
  `evt_3vsjks22y4pnd`. Authority belongs to a call whose value is in
  flight, and it is inherited inward and never reset.

Treat anchors as perishable. If a fixed input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

When a let's value returns a member call through an inner `Let` body or
through every arm of an `Elim`, the let's type and scope inherit that
call's refined arguments. The rows join
`lang_refined_recursion_let_bound_call.rs`.

## Acceptance

- **AC-0 (route, Architect).** Rule the "returns the call" relation:
  which tail positions count (an inner let body, `match` arms, `if`
  arms), and what an `Elim` whose arms return different calls inherits.
- **AC-1.** The three rows are accepted with obligation counts pinned.
  The four controls and every row already in the test file are
  unchanged.
- **AC-2.** The kept refusals in the test file stay K0014. Add one more
  refusal: a let whose value returns `g (g m)` through an inner let.
- **AC-3 (mutation, QA).** Restoring the head-only check reddens the
  three rows back to K0014.

## Stop conditions

- Any kernel or `trusted_base()` change.
- A changed SCT verdict outside the rows named here.

Checks: 4 fired: `with_call` (~:166), `member_call_refined_args`
(~:184) and the Let arm's authority (~:270) are on `6a00e978c` at the
lines cited. 7 fired: AC-2's nested-call row tests that the tail walk
does not admit a non-decreasing call. 8 fired: an admit-all-projections
rule would also turn the three rows green, so AC-2 rejects it.
