---
id: ELAB-OMEGA-PLAN-ELIM-RULE-TYPES
title: "The Omega erasure planner visits an eliminator's scrutinee with no expected type, so it calls the kernel's infer on it, and the kernel's infer of an Elim is its full admission rule. In a normal form the scrutinee is often itself a stuck Elim, so every level re-admits everything beneath it: the cost is quadratic in elimination nesting, and on PX11's exchange it is 388 s of a 563 s plan. Pass down the parameter, index and scrutinee types the kernel's own elimination rule fixes, with an unchanged plan"
status: merged
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: [PX11-NATIVE-STREAM-SOCKET-PARITY]
github: null
origin: "Architect ruling evt_7910w302ys8tc on PX11 Inc2's A-PROBE (runtime-implementer evt_3e3vpwgvrmkts): arm A-mixed, planner first. The repair is untrusted elaborator code, with no kernel change. The Language ring owns crates/ken-elaborator. L1 Linux ABI program, Steward order."
---

# Omega planner passes down the elimination rule's types

## Objective

PX11's exchange program builds natively within its 450 s bound, because
the Omega erasure planner stops re-admitting nested eliminators, and every
plan it produces is unchanged.

## Settled inputs (main `a7bcf9d18`; Architect `evt_7910w302ys8tc`)

- **The measurement.** A-PROBE on exchange main, with n = 183,983 and plan
  time 562.820 s. The planner's exclusive time is 44.8%, and the kernel
  Context clones (K1) are 39.7%, all nested under planner calls.
  `Elim.scrutinee` alone is 1,849 calls and 387.737 s inclusive.
- **The mechanism.** `visit`'s `Term::Elim` arm in `omega_erasure.rs`
  passes `None` for the scrutinee, so it calls `infer_node`. The kernel's
  `infer_elim` (`check.rs` ~`:652`-`:744`) is the full admission rule.
- **The enablers.** `subst_levels`, `subst_tel` and `subst_outer` are pub
  in `ken_kernel::subst`, and `InductiveDecl`'s `params`, `indices` and
  `level_params` are pub. The step-5 scrutinee type is the kernel's own
  construction. There is no kernel change.
- **The closure.** The ruling's exact replacement covers the param, index
  and scrutinee visits, plus the import line. The motive and the method
  visits are unchanged. Build it as written.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The Elim arm, the test-only recorder, and the rows below. No change to
`ken-kernel`, to the motive or method visits, to any other `visit` site, or
to PX11's fixture, bound or profile.

## Acceptance

- **AC-1.** The Elim arm as ruled. The existing `omega_erasure.rs` unit
  tests stay green unchanged.
- **AC-2.** A `#[cfg(test)]` thread-local recorder in `omega_erasure.rs`
  sums the node count of each term `visit` passes to `infer_node`. The row
  is the ruling's 16-deep Bool elimination chain. The recorded sum is at
  most 2x the body's node count. Report the sum on the candidate and under
  m1, which restores `None` at the scrutinee and must exceed the bound.
- **AC-3 (measurement, not committed).** Rerun the A-PROBE counters on the
  candidate, rebased into the PX11 stack as the probe was, for p2,
  drain_full and exchange main. Compare the four plan sets against base,
  element for element, on all three. Report the ruling's pre-registered
  arm, R-a to R-e.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

The ruling's arms go to the Architect:
- R-d, any plan set differs from base;
- R-e, `Elim.scrutinee` still holds at least 1/4 of plan time;
- an admitted body whose arity differs from the slices.

R-b and R-c return to the Steward for a successor frame, and R-c goes to
the operator first.

Checks: 2 fired. Elim params, indices and scrutinee close here. Every
other P site measures 11.3 s together. 8 fired. AC-2 sums the work input,
not a visit count, and m1 must cross its bound. 9 fired. The plan
equivalence the closure argues for is measured element by element on three
programs.
