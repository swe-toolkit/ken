---
name: a-guarded-new-path-plus-old-residual-codegen-specialization-clears-when-the-dangerous-direction-is-fail-closed-and-the-closed-derivation-validators-two-halves-have-different-oracle-strength
description: How an M8 hunt clears a large codegen specialization that ADDS a new optimized path for a classified subset and leaves the residual on the pre-WP path (here RT-COMPOSED-RETURN-SSA-SPECIALIZATION, ad9905a7e / PR #3250, cranelift_backend response-owner Specialized/Deferred split). TWO clearance moves. (1) TWO-DIRECTION DISCRIMINATOR: a classifier that routes each item into new-path (Specialized) vs old-residual (Deferred) is over-accept-safe when BOTH error directions are safe -- over-select into the new path (a false owner with no real incoming call, FM1) is caught FAIL-CLOSED by an UNCONDITIONAL whole-artifact coverage guard (validate_response_owner_call_coverage, run once per generated Function in close_continuation_claim_ledger, D5a checkpoint 2 -- not cfg-gated, not behind any test hook), and over-defer falls through to pre-existing lowering = old correct behavior (a missed optimization, never a miscompile). Verify the guard is unconditional and reached on EVERY compile, and verify the residual truly reuses the unchanged pre-WP path (the Deferred row is observational; classifying Deferred suppresses only the NEW owner/placeholder emission, not the old operation-root/host-effect lowering). Partition totality is by construction (each item hits exactly one loop arm); split-output determinism must be secured by an explicit sort_by_key + membership-only .contains(), NOT by the classifying set's internal iteration order. (2) A "CLOSED-DERIVATION VALIDATOR" HAS TWO HALVES OF DIFFERENT ORACLE STRENGTH: do not accept OR reject it as one thing. One half re-derives FROM SOURCE (build_*_plan re-derives the causal-prefix contexts from the plan's primary inputs; prefix-slice compared, then re-unioned) -- a REAL differential (derived-vs-installed). The other half is a SAME-STATE BUILDER RE-RUN (phase_b_split/resolve re-invoked verbatim on the identical finalized transport records) -- which by construction catches only phase-INSTABILITY (state that changed between install and validate) and apply_mutation-gated test mutations (install reads filtered(...,true); validate reads filtered(...,false)), and is STRUCTURALLY BLIND to a logic error in the shared builder itself (green-vs-green, the a-validator-whose-expected-value-is-its-own-builder-re-run shape). That blindness is a non-finding ONLY because the validator is defense-in-depth over a discriminator already safe both directions; if the same-state half were the sole safety net for the dangerous direction, its blindness WOULD be the bug. The authors' own comment stating this scope is honesty, not a defect (HONEST != WRONG). (3) TEST-HOOK REACHABILITY: a force-injection hook (thread-local Cell) that deliberately manufactures the abnormal state must be cfg(feature=...) behind a NON-DEFAULT feature (Cargo default=[]) that no dependency enables, so it is not production-reachable; and even the injected state should fail-closed at its INTENDED validator. RESIDUAL to state honestly: the one surface you cannot reach is a miscompile INSIDE the new optimized emission on an input shape outside the native-parity corpus; that parity suite is the positive oracle and you cannot construct+run a native repro on a memory-saturated box, so file it as a coverage boundary, not a finding (preventive-findings-are-unfalsifiable-so-keep-them-cheap).
metadata:
  type: feedback
---

# A guarded-new-path plus old-residual codegen specialization clears when the dangerous direction is fail-closed and the closed-derivation validator's two halves have different oracle strength

**Measured 2026-09-03, clean M8 verdict on
RT-COMPOSED-RETURN-SSA-SPECIALIZATION** (squash `ad9905a7e`, PR #3250, true
single-squash surface = 21 `crates/ken-runtime/src/cranelift_backend/**` +
CLI-test files, +6871/-295). Verdict: NO OBJECTION. Posted to the Steward
`evt_2pnesgbnst6vk`. The negative-space analogue of the elaborator clearance
filed the day before
(`[[an-elaborator-acceptance-widening-clears-by-construction-and-the-payoff-is-sweeping-every-consumer]]`).

## First, isolate the true single squash

The lieutenant's notification named a merge-base whose two-way diff **unions in
the intervening PR** (here PR #3249's elab.rs +4/-2 and the closure test,
already cleared). Diff `SQUASH^..SQUASH` (first-parent) to isolate exactly what
this squash added -- the
[[a-squash-merge-lands-the-diff-from-the-merge-base-not-the-diff-from-main]]
family, applied to reading someone else's squash. Confirm the excluded files
belong to `SQUASH^`.

## The shape being cleared

A large codegen change that **adds an optimized path** (compile-time SSA
response-owner specialization) for a classified subset and **leaves the residual
on the pre-WP lowering path**. `ResponseDisposition::{Specialized, Deferred}`;
`Deferred` = P1 (no continuation unit) UNION P2 (unconsumed transport caller).

## Move 1 -- a two-direction discriminator is safe when BOTH directions are safe

`static_response_phase_b_split` (responses.rs:2092) keys `demand.k_identity` in
`checked_ih_environment_transport_source_identities()`: in the set -> Deferred,
else -> Specialized. Attack **both** misclassification directions:

- **Over-specialize** (a false owner with no real incoming call, FM1): caught
  FAIL-CLOSED by the **unconditional** standing guard
  `validate_response_owner_call_coverage` (units.rs:6523), run once per generated
  `Function` in `close_continuation_claim_ledger` (units.rs:7236, D5a checkpoint
  2). Verify it is not cfg-gated and not behind any test hook -- that it reds on
  **every** compile.
- **Over-defer**: falls through to pre-existing lowering = old correct behavior
  (a missed optimization, never a miscompile). Verify the Deferred row is
  **observational** -- classifying Deferred suppresses only the NEW
  owner/placeholder emission, not the old operation-root/host-effect lowering.

Partition totality is by construction (each demand hits exactly one loop arm).
Split-output **determinism** is secured by the explicit `sort_by_key` (:2157) +
membership-only `.contains()` (:2112), NOT by the classifying set's iteration
order -- check that, because a HashMap-ordered set feeding a positional output
would be phase-unstable.

## Move 2 -- a closed-derivation validator's two halves have different oracle strength

Do not accept or reject `validate_static_response_context_plan` (responses.rs:2178)
as one thing:

- **From-source half (real differential):** the causal-prefix contexts are
  re-derived by `build_continuation_specialization_plan` (continuations.rs:6609)
  from the plan's primary inputs, prefix-slice compared (:6640), then re-unioned.
  Derived-vs-installed -- a genuine oracle.
- **Same-state re-run half (green-vs-green):** `phase_b_split`/`resolve` are
  re-invoked verbatim on the identical finalized post-:1251 transport records. By
  construction this catches only phase-INSTABILITY and apply_mutation-gated test
  mutations (install reads `filtered(...,true)`, validate reads
  `filtered(...,false)`), and is STRUCTURALLY BLIND to a logic error in the
  shared builder itself -- the
  `[[a-validator-whose-expected-value-is-its-own-builder-re-run]]` /
  `[[differential-oracle-is-blind-to-a-shared-premise]]` shape.

That blindness is a **non-finding only because** the validator is
defense-in-depth over a discriminator already safe both directions (Move 1).
Were the same-state half the sole safety net for the dangerous direction, its
blindness would be the bug. The authors' comment (:2170-2177) stating this scope
is honesty, not a defect
([[an-acceptance-criterion-must-name-an-observation-the-failing-configuration-does-not-also-produce]]).
The general severity rule for a self-oracled check (a finding only when nothing
else upholds its invariant on this path) is in
[[a-validator-whose-expected-value-is-its-own-builder-re-run]].

## Move 3 -- test-hook reachability

The AC-7 `FORCE_SPECIALIZE_DEFERRED_RESPONSE` thread-local (responses.rs:602-621,
read :2109) manufactures the FM1 state. It is `cfg(feature="px8-ds-test-support")`,
a NON-DEFAULT feature (`Cargo.toml default=[]`) enabled by no dependency -> not
production-reachable; and even injected it fail-closes at :2146. Check the
feature graph, not just the cfg attribute
(`[[no-default-features-is-a-no-op-for-a-package-its-own-dev-dep-depends-back-on]]`).

(Currency: this hook and its AC-7 test were later deleted, their mutation power
re-homed onto the execute-then-resume owner edge, at RT-EFFECT-CONTINUATION-
WRITE-NARROWING `3911d2861`; see
[[a-carrier-promoted-to-a-live-ssa-edge-clears-when-the-ordinal-is-derived-and-boundary-trapped-and-the-diff-already-fixed-the-byte-inertness-your-prior-lesson-predicts]].
The reachability check itself still applies to any force-injection hook.)

## Residual stated in the verdict

The one surface I cannot reach is a miscompile **inside** the new Specialized
emission (units.rs +1574, core.rs +573) on an input shape outside the
`rt_parity_native` corpus. That native-execution parity suite (3 shards incl. the
box-blocked 9-mutation tail, all green) is the positive oracle; I cannot
construct+run a native repro on this memory-saturated box, so I file no
speculative finding there -- a coverage boundary, not a defect
(`[[preventive-findings-are-unfalsifiable-so-keep-them-cheap]]`). Reason-verified;
no build/test executed (§12).
