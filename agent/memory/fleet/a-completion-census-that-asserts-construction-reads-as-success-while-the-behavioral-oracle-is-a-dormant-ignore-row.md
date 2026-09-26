---
name: a-completion-census-that-asserts-construction-reads-as-success-while-the-behavioral-oracle-is-a-dormant-ignore-row
description: >-
  A green running check can cover a strictly weaker property than the one it
  reads as while the behavioral oracle for the real claim sits in an #[ignore]
  row. Two forms: a completion census whose Completes terminal means "an
  artifact was built" (construction, not execution), and a two-level mechanism
  whose authority issuer is unit-pinned while its consumer has only a running
  mechanism-DISABLED negative control. Read what the green row asserts, find
  where the behavioral oracle lives and whether it runs, and check sibling rows
  for any oracle at all.
metadata:
  type: feedback
---

# A green row can assert construction while the behavioral oracle is a dormant ignore row

Deferred-execution work (a representation-only increment, a mechanism whose
end-to-end witnesses are blocked at a downstream successor) leaves the suite
green on something **weaker** than the property a reader infers. The real
behavioral oracle exists, if at all, as an `#[ignore]`'d row: it **discloses**
the gap but no running gate **enforces** it.

## Form 1: a completion census over construction

A census moves rows from a pinned advancing-refusal to `Disposition::Completes`,
where

```
Completes  <=>  build_native_program(...) returns Ok(_)
```

That asserts an artifact was produced. The artifact is discarded and never
executed. So `Completes` is a **construction** claim, strictly weaker than the
**execution** claim ("the artifact computes the right value") that the census
framing ("when the blocker is retired, move this row to `Completes`") reads as.

## Form 2: a two-level mechanism pins the issuer and leaves the consumer dark

A mechanism **issues** authority at one layer (a planner decides which items get
it) and **consumes** it at another (lowering reconstructs and calls the item).
The issuer can be well pinned by running unit tests, positive and negative,
that refute over-broad issuance without native codegen. The consumer can still
have **zero** running coverage: its functions are called only from production
lowering, and every end-to-end witness that would drive them is `#[ignore]`'d
at a downstream successor blocker. A regression in consumption that still lets
lowering reach the same successor refusal is invisible to CI.

A **running negative control is not positive coverage.** A control that
disables the mechanism and asserts the old refusal returns proves the gate is
load-bearing, but it exercises the disabled-refusal path, not the enabled
"admit and produce the correct result" path.

## The discriminator

1. **Read what the green row asserts.** "Lowering returned Ok" or "an artifact
   was built" is construction; it does not execute. A census over
   `build_native_program` is a construction census.
2. **Find where the behavioral oracle lives and whether it runs.** A completing
   row whose parity test is ignored is "builds but runs wrong", not "works".
   For a two-level mechanism, confirm the issuer is pinned, then separately ask
   whether the **consumer** has any running oracle.
3. **Check sibling rows for any behavioral oracle at all.** The "green =
   success" framing hides the known-broken (ignore-disclosed) and the entirely
   unmonitored (no oracle) equally.
4. **Before scoring a removed assertion as a detection loss, check whether the
   test that carried it was already ignored.** If it was, nothing running was
   lost.
5. **Remedy direction:** a lowering-level unit control on the consumer now, or
   make the first un-ignored end-to-end row the running positive proof when the
   successor blocker lands.

This is usually honest deferral within an increment's scope, so it is a
non-blocking coverage-honesty observation, not a defect. Surface it anyway,
because a later reader treats the green rows as parity-correct.

## Instances

- **2026-08-24, M6 Track-1 D0, RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION**,
  landed squash `79d64a967` (byte-identical to routed candidate `427b56d6`;
  parent `5032d5022`). Per-file blob identity against the landed tree reduced a
  30-file delta to 5 un-hunted files (25 identical to cleared `171c432f` /
  `cac27f3b`); always reduce a cross-base squash that way first. The mechanism
  (a per-`(owner, producer origin)` `InvocationReturn` branch gated by
  `checked_ih_environment_transport_at`, not the plan-wide
  `checked_ih_environment_transports_owned_by`) was sound and pinned. The
  census `rt_cold_lowering_path_enumeration.rs` moved 8 stages to `Completes`:
  4 (fs read/write narrowing) have paired parity tests in
  `rt_parity_native.rs` still ignored with the reason "native construction
  completes, but execution traps on a malformed `ExitCode::Failure` payload";
  the other 4 (the cap41 family) have no behavioral test at all, only `proc`
  source-template fixtures. CLEAN, one observation. evt_4m6w2wnwrq27y
  (thread thr_5f4cz4txebpe0).
- **2026-08-25, RT-CLOSURE-BOUNDARY-RESIDUAL M4**, landed squash `f02922221`
  (byte-identical to gated candidate `a6f6d5d52`; parent `4486e109a`).
  `Lowered::Closure` gained a planner-issued `boundary_environment`. The
  issuer was pinned by running planner tests
  (`bind_continuation_authorization_is_reaching_and_not_generic`,
  `bind_target_exactness_rejects_multi_target_and_unpaired`,
  `bind_continuation_production_rejects_multiple_static_targets`). The
  consumer (`represented_boundary_admissibility`'s `Some` arm,
  `call_boundary_closure_environment`, `emit_boundary_closure_environment`)
  had no `#[test]` caller, and every witness (px8f/px8l/px8ta-exact) was
  ignored at a successor blocker. The new running
  `px8ds_retired_flat_order_does_not_gain_m4_representation` is a
  disable-and-refuse inversion control (the cfg switch
  `px8ds_retired_flat_order_enabled()` is a compile-time `false` in
  production, so it can only disable). A suspected dropped negative-contrast
  assertion was refuted because the pre-split test was itself ignored. CLEAN,
  one observation. evt_4xqsp8x5vfmcb (thread thr_e3mbsmfbmmp4).

Related:
[[a-test-pinning-a-missing-port-refusal-inverts-its-signal-when-the-port-lands]]
(same enumeration family),
[[a-not-run-row-census-reads-the-ignore-label-not-the-row]] (a skip
discloses but no running gate enforces),
[[a-repro-is-evidence-not-a-completion-oracle]] (an Ok terminal is not a
behavioral oracle),
[[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]].
