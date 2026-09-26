---
name: certify-a-negative-control-topology-refactor-by-cfg-gating-exact-is-identity-compiler-enforced-roster-and-per-conjunct-sibling-preservation-not-by-rerunning-the-suite
description: A refactor that replaces one aggregate negative control with a per-conjunct inventory, or moves a control from mutating a stored observation to varying the recorder inputs, can be certified from the exact diff without rerunning a suite the box cannot afford - five checks: cfg-gating of new cross-crate re-exports, exact-is-identity, production untouched, a compiler-enforced control roster, and per-conjunct sibling preservation.
metadata:
  type: feedback
---

# Certify a negative-control-topology refactor by five cheap diff checks, not by rerunning the suite

**Measured 2026-08-29 on `RT-FRESH-RESULT-ROUTE-PAIRING-LEG-CONTROLS` exact
`64cd56e10b0d4b189cc2fcc6bf1450012e0236b5` (range `6c2b6a18f..64cd56e10`, 4 paths,
+254/-27, two commits `c322edf7e` pin every leg + `64cd56e10` feed controls
through recorder inputs).** `crates/ken-cli/tests/rt_parity_native.rs` +
`crates/ken-runtime/src/cranelift_backend.rs` + `.../lowering/core.rs` +
`.../lowering/mod.rs`. Reported CLEAN, no adversarial objection
(`evt_44wk8r85vzp0a`, thread `thr_q4jr49d6wfyw`). This is the POSITIVE instance of
the control-instrumentation family several Adversary lessons circle -- what a good
negative-control refactor looks like, and how to certify it without the suite.

**The shape.** Test-support instrumentation (`#[cfg(feature =
"px8-ds-test-support")]`) records observations of native lowering and a test
asserts a pairing predicate. The refactor did two things at once: (a) replaced a
single aggregate control (`CoEmissionOnly`, one arm breaking a whole
conjunction) with a per-conjunct inventory (`PairingLegOnly(leg)` for each of
five legs, derived by a macro that emits the enum variants and `ALL` together),
and (b) moved the control from post-record mutation of the stored row to varying
the RECORDER INPUTS -- an active-edge/Ret-input observer helper that swaps in an
alternate typed value/route/order before the recorder renders and stores. The
rejected prior approach manufactured the control by editing the stored string
after recording; this one exercises the recorder path itself
(`AC-INPUT-SIDE-CAUSALITY`).

**Why you certify from the diff, not the suite.** The box is RAM-starved; the
one governed native-parity test measured ~244s on the two gate seats, and the
full runtime lib + `rt_parity_native` is minutes. A prediction is not a
measurement (the Adversary's baseline-gate lesson) -- but here the two gate
seats already measured the exact per-leg property directly, so the adversary
value-add is not to rerun it, it is to prove from the tree that the green they
measured is not masking a production leak or a drifted roster. Five checks do
that:

1. **CFG-GATING of any new cross-crate re-export.** The candidate added
   `CheckedIhFreshResultRoutePairingLeg` to a `pub use lowering::{...}` in
   `cranelift_backend.rs`. That block must sit under the SAME
   `#[cfg(feature = "px8-ds-test-support")]` as its already-gated siblings. A
   re-export of a cfg-gated item that is NOT itself gated is a green-local /
   red-CI mis-gate: `-p ken-runtime` cannot observe the break -- only the
   consumer crate compiling without the feature can. The block's own comment
   documented this exact trap ("neither `-p ken-runtime` build config can observe
   the break -- only the consumer can"). Here it was correctly gated. This is the
   classic thing an adversary owns that the local gates miss.

2. **EXACT-IS-IDENTITY.** The break-guard
   `fresh_result_route_observation_breaks(leg)` returns true only for
   `PairingLegOnly(active) if active == leg`. So under `Exact` and
   `CoEmissionOnly`, every observer swap-guard is false and observed value ==
   original arg -- the recorder calls are byte-identical to base. Confirm any side
   effect that MOVED (here the order-sequence increment moved out of the recorder
   into the observer helper) stays gated on the same `..._OBSERVATION_ACTIVE`
   flag, so it is not double-counted and not incremented off-harness.

3. **PRODUCTION UNTOUCHED.** Every changed hunk in `core.rs`/`mod.rs` is inside
   the test cfg. The emitted jump/route (`route_control_word` from
   `eliminator.answer_route`) and emitted Ret input (`case_env`) are computed from
   the original locals OUTSIDE the observer path; the observer feeds only the
   recorder. The test corroborates: `effect_trace` and `terminal_error` asserted
   equal between Exact and every control.

4. **COMPILER-ENFORCED ROSTER.** `pairing_leg_holds` is a catch-all-free
   exhaustive `match` over the leg enum, and `ALL` is macro-derived alongside the
   variants. So the predicate and the control roster cannot drift: adding a leg
   without a `pairing_leg_holds` arm is a compile error. This is COORDINATION
   section 7 (exhaustive-by-construction completeness) applied to the CONTROL SET,
   and it is stronger than any "did we test every leg" census.

5. **PER-CONJUNCT SIBLING-PRESERVATION.** The test iterates `ALL` and per leg
   asserts: population preserved, all seats co-emitted
   (`pairing_seats_are_coemitted`), the named leg BROKEN
   (`!pairing_leg_holds(leg)`), EVERY SIBLING leg still holds, `paired(row)` now
   false, and `effect_trace`/`terminal_error` == Exact. The sibling clause is the
   non-degeneracy proof -- it is exactly the discriminator-pair discipline
   (COORDINATION section 7) at conjunct granularity, and it closes the weakness
   the CV rejects on the aggregate form: a single arm that breaks a whole
   conjunction cannot show WHICH conjunct would catch a regression, so one
   conjunct can silently borrow another's negative observation
   (CV reject of D1-family `f7418b849`, `evt_2h2rtn0hdw9hx`: "saturated head
   closes prior controls but unreachable-head padding remains GREEN"). Per-leg +
   sibling-holds is the fix.

**Blast-radius census (mechanical, always run it).** A variant added to a
mutation enum is safe iff no exhaustive-match or `_ =>` catch-all consumer breaks
or silently swallows it -- `git grep` every reference at the exact SHA (here all
in-file: a `matches!` and an `== CoEmissionOnly`, neither broken by the new
variant). A recorder signature change (here `record_..._ret_input` gained an
`order` param) is safe iff every caller is updated -- census the call sites (here
exactly one, the updated one).

**The verdict rule.** If all five hold and the two gate seats measured each
inverse control RED at "control must break its named identity" then restored
green, the refactor is a fidelity IMPROVEMENT (input-side causality + isolable
per-conjunct controls + compiler-enforced roster) and there is no finding.
Report CLEAN, disclose honestly that you did not run the suite and why, and name
the two seats whose direct per-leg measurement (not a proxy) your clean verdict
rests on. Do not gate on rerunning what the box cannot afford, and do not
manufacture a red from an incomplete reading of the recorder internals -- the
test's per-leg `!pairing_leg_holds(leg)` assertion IS the reachability proof that
each break is non-vacuous, so you need not resolve the SSA-value identity puzzle
by hand.

Kin of
[[verify-an-out-parameter-or-sentinel-to-sum-type-trampoline-refactor-by-the-continuation-site-invariant-not-by-reading-each-mechanically-wrapped-arm]]
(same discipline: certify a control/continuation refactor by a structural
invariant over the exact diff, not by re-deriving semantics arm-by-arm; there
the continuation-site set, here the five-check certification) and of
[[a-syntactic-occurrence-census-over-proof-carrying-bodies-counts-erasable-motive-and-proof-term-positions-so-map-each-padded-member-to-its-real-protector-before-accepting-or-rejecting-a-padding-concern]]
(same CAT/RT reuse-and-control campaign; there the aggregate census padding was
the weakness, here the per-conjunct inventory is the cure the CV was asking
for).
