---
id: LANG-EFFECT-ROW-LET-BOUND-SCRUTINEE-LATENT
title: "After LANG-EFFECT-ROW-MATCH-BINDER-LATENT, a match binder still releases nothing when its scrutinee is an unannotated let-bound constructor application: `let b = MkBox action in match b { MkBox f ↦ let ignored = f MkUnit in host_program (print_line \"b\") }` checks and prints only `b`, while the direct, ascribed and annotated forms print `ab`. bound_value_latent has no arm for RApp, so the binding records an empty latent. Give every let-bound value the latent it releases, or refuse it, and pin the let-bound scrutinee"
status: merged
owner: verify
size: S
tier: T1
gate: architect
depends_on: [LANG-EFFECT-ROW-MATCH-BINDER-LATENT]
blocks: [LANG-EFFECT-ROW-CELL-SCRUTINEE-CARRIED-ROW, LANG-PROC-NON-TREE-RESULT-TREE]
github: null
origin: "Adversary finding evt_75vsh9fp0n6m5 on cdc1aed63 (medium, wrong code). Steward-filed per COORDINATION section 2, sequenced ahead of LANG-PROC-NON-TREE-RESULT-TREE on the Verify ring."
---

# A let-bound scrutinee carries its latent

## Objective

A checked program never silently drops a match-bound callback's
action. With the P1 header of `lang_effect_row_match_binder_latent.rs`:

```ken
let b = MkBox action in
match b { MkBox f ↦ let ignored = f MkUnit in
  host_program APartial (print_line "b") }
```

prints `ab`, as the direct scrutinee `match MkBox action`, the
ascription `(MkBox action : Box)` and the annotation `let b : Box`
already do. Otherwise it is refused at check time.

## Settled inputs (main `cdc1aed63`)

- **The path.** In `crates/ken-elaborator/src/elab.rs`, a let binder's
  latent comes from `pending_let_latent` (`:1378`), then
  `let_binder_latent` (`:17913`), then `bound_value_latent`
  (`:17929`). `enter_match_latent` (`:1322`) reads that latent for a
  variable scrutinee.
- **The gap.** `bound_value_latent` has arms for RVar, RPatternAlias,
  RCon, RCheckedGlobal, RLam and RAsc. Every other form, including
  RApp (a constructor application), falls to the empty arm at
  `:17955`. A direct scrutinee goes through `infer_expr_row_type`
  (`:17975`) instead, and an ascription adds `written_type_row`
  (`:17773`), which is why those variants are correct.
- **The measurement.** The Adversary ran the program above on the
  candidate binary, which printed `b` and exited 0. The three variants
  printed `ab`. No row in the landed test pins a let-bound scrutinee;
  R8 pins a let copy of a match binder.
- **Stated gap kept.** The landed test's GAP for an application that
  returns a closure stays as it is unless AC-0 rules otherwise.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

`bound_value_latent`, or its caller, gives each let-bound value form
the latent it releases, or refuses the binding at check time. One
candidate, `/crates` only.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the patch, and the named-binary commands with a bounded
  timeout. The Verify implementer runs it on scratch and reports. The
  Architect rules from that report, and does not build or test. It
  rules:
  - every value form that reaches the empty arm, enumerated from the
    `RExpr` variants, not only RApp; for each one, whether it carries a
    latent, refuses, or is shown pure;
  - for RApp, how the latent is separated from the application's
    evaluation row, so that nothing is paid twice;
  - whether the fix reuses `infer_expr_row_type` for the bound value.
- **AC-1.** The repro prints `ab` and exits 0. Each of the three
  variants still prints `ab`. A same-form control, `MkBox` over a pure
  callback, checks and prints only `b`, so the change is the latent,
  not the form.
- **AC-2.** Every form AC-0 enumerated has a row: carried, refused or
  pure, as ruled. The 13 landed MATCH-BINDER rows, the strict census
  (98, 64, 12, 52) and frame budget row M stay green.

## Stop conditions

- A kernel, trust or surface change.
- A form that can carry a latent only by changing a spec 36 §1 rule.

Checks: 2 fired: the empty arm is the fan-in for every unlisted value
form, so AC-0 enumerates them from the `RExpr` variants, not from the
one repro. 7 fired: MATCH-BINDER's criterion, run on the let-bound
sibling, fails; AC-2 runs the fix's criterion on every sibling form.
8 fired: the pure-callback control has the same binding and match form
and prints only `b`, so a fix that sequenced everything would fail it.
