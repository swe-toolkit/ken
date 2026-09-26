---
name: a-fail-closed-over-accept-gate-can-have-correct-production-code-but-an-untested-safety-net-negative-arm
description: >-
  RT-MAPPING-MULTIOP-DISPATCH (1874361, +1943/-54 runtime candidate), an
  over-accept gate: verdict NO DEFECT / APPROVE + two non-blocking missing-test
  observations. The change lets one specialized owner dispatch a statically
  bounded response suffix ONLY when it is a repeated-producer chain or a Mapping
  read/write chain; every heterogeneous/cross-class sequence must fall through
  to the established owner via an `else => Ok(Vec::new())` arm. Method: for a
  fail-closed over-accept gate, the acceptance-producing arms being correct is
  necessary but NOT sufficient -- the over-accept SAFETY NET is the negative arm
  (the else that returns empty/unowned), and it needs its OWN negative test.
  Here the classification and the else-empty fallback read correct, but NO test
  in the changeset constructs a genuinely cross-class sequence and asserts the
  owner is None / route byte-identical to base; a future regression flipping that
  arm to return the suffix (a real over-accept) would pass every control,
  because the positive controls only ever FIRE the mechanism on in-class inputs
  and cannot exercise the else. Second method: a positive native-vs-interp
  PARITY control without an owner-identity or mutation discriminator cannot prove
  the NEW mechanism fired rather than an incidentally-correct pre-existing path
  (the Mapping chain had a SuppressLocalContinuationDrive mutation that flips
  owned->trap; the repeated-producer class had only parity, so it cannot
  distinguish mechanism-fired from same-trace-by-another-route). Sibling of the
  control-must-reach-and-discriminate family.
metadata:
  type: feedback
---

# A fail-closed over-accept gate can have correct production code but an untested safety-net negative arm

**Measured 2026-09-10 on `1874361` (RT-MAPPING-MULTIOP-DISPATCH, +1943/-54,
16 crates files), a pre-merge over-accept gate. Verdict NO DEFECT / APPROVE;
two non-blocking missing-negative-test observations (routed evt_fsd2bf8e4hrv).**

The change lets a single specialized response owner dispatch a statically
bounded P2 suffix of Deferred responses, but ONLY when the suffix is one of two
evidenced classes: a repeated-producer call chain, or a Mapping read/write
access chain. `bounded_deferred_response_suffix` classifies with conjunctive
`.all(...)` and, for anything else,
`else { Ok(Vec::new()) }` (responses.rs ~2879-2882) -- unowned, so the sequence
falls through to its established owner/route unchanged (the "S7" fail-closed
safety net). The whole design is pervasively fail-closed (None/empty on opacity,
cycles, unknown Vis, multi-owner hard-error), and I verified each acceptance arm
is sound by reading.

## The over-accept guard is the ELSE arm, and it needs its own negative test

For an over-accept gate the natural focus is the arms that ACCEPT (here: own the
suffix). But the thing that PREVENTS over-acceptance is the negative arm -- the
`else => empty/unowned` fallback. That branch is where a future regression would
over-accept: flip `Ok(Vec::new())` to `Ok(suffix)` and a heterogeneous /
cross-class sequence gets wrongly owned. So the guard's regression-detectability
lives entirely in whether a test drives a genuinely CROSS-CLASS input through it.

Here there was none. Every positive control fed an IN-CLASS input (a pure Mapping
chain, or a pure repeated-producer chain), which by construction takes the
accept arm, never the else. The one test whose name said "heterogeneous"
(`abi_s6...read_write_read`) was alternating direction WITHIN the Mapping class
-- still `mapping_access_chain`, still the accept arm. No test built an FS
producer interleaved with a Mapping access, or a Mapping op followed by an
unrelated AmbientOp, and asserted `bounded_deferred_response_handler_owner`
returns None / the route is byte-identical to base. So **the production else-arm
was correct, yet the over-accept guard was entirely untested: a regression that
turned the safety net into an over-accept would pass the whole suite.** That is a
missing-negative-test finding, NOT a present over-acceptance -- report
it as such, non-blocking, and do not inflate it into a defect the code does not
have.

## A parity control without a discriminator cannot prove the NEW path fired

The Mapping-chain class had a real mutation discriminator: a test flips
`HandlerOwnedDeferredResponseMutation::SuppressLocalContinuationDrive` and asserts
the native build then TRAPS (`UnclassifiedRuntimeTrap{terminal_value:-1}`)
instead of succeeding -- proving the new owner-drive is load-bearing (owned ->
trap when suppressed). The repeated-producer class had only a native-vs-interp
`effect_trace` PARITY assertion. Parity proves the OUTPUT is correct; it does not
prove the NEW bounded-suffix mechanism produced it rather than some pre-existing
lowering path that happens to emit the same trace. A positive control that only
FIRES the mechanism, with no owner-identity assertion and no
mechanism-off mutation, cannot discriminate "new path fired" from
"same trace by another route." This is the runtime twin of the
CHECKS.md check 7
/ reach-and-discriminate family: the control must be able to FAIL when the
specific claimed mechanism is absent.

## Method

1. For a fail-closed over-accept (or admission) gate, find the negative arm --
   the `else`/`None`/`empty` that denies the new capability -- and check it has a
   test that drives an input INTO that arm and asserts the denial. The
   acceptance arms being correct does not cover it; the positive controls
   structurally cannot reach the else.
2. Treat a correct-but-untested guard branch as a regression-detectability gap
   (missing negative test), severity gap-tier, non-blocking -- distinct from a
   present over-acceptance. Say plainly the code is sound today and name the
   exact cross-class input a control should use.
3. For any "the new mechanism generalizes" claim, require a discriminator (a
   mechanism-off mutation like SuppressLocalContinuationDrive, or an
   owner-identity assertion), not just behavioral parity. Parity across a
   correct output cannot tell the new path from an incidentally-correct old one.
   Sibling of
   [[downstream-code-coupled-to-a-kernel-admission-invariant-needs-a-negative-control-on-the-changed-branch]]
   (verify the discriminating axis the change itself introduces, not an
   orthogonal one).
