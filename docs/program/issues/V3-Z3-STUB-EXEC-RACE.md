---
id: V3-Z3-STUB-EXEC-RACE
title: "Make the z3 process-adapter stub tests deterministic: parsed_model_is_candidate_not_verdict intermittently gets Unknown instead of Disproved in CI even after the 5 s stub timeout, so the cause is not the timeout; measure the stub's actual process error, then remove that race in the test harness"
status: ready
owner: verify
size: S
gate: architect
tier: T2
depends_on: []
blocks: []
github: null
origin: "Operator 2026-09-29: \"It's worth bringing verify into one of the lanes (L2) to address the flaky test.\" Successor to V3-Z3-STUB-TIMEOUT-FLAKE (b2d5ffbf7), whose fix did not hold. Steward-filed per COORDINATION section 2."
---

# Remove the z3 stub race

## Objective

`crates/ken-elaborator/tests/v3_z3_process_adapter.rs` gives the same
verdicts on every run, so a CI run fails only on a real adapter defect.

## Fixed inputs (read at `4902a2869`)

- **The failure.** `parsed_model_is_candidate_not_verdict` panics at `:130`
  (`assert!(matches!(verdict, Verdict::Disproved { .. }))`) in the
  "optional z3 process adapter" job, "Test feature-on adapter" step. The
  failing test finished about 0.2 s after its siblings, far under
  `STARTUP_SAFE_STUB_TIMEOUT` (5 s, `:18`).
- **Recurrence after the timeout fix `b2d5ffbf7`.** It failed on the decoder
  candidate `bc92bba84` (09-28 05:35Z, run 36379487204) and on
  `LANG-SESSION-SCOPE` 2a (09-28 15:52Z, run 36446667141). The 2a commits
  then passed 20 of 20 local runs each.
- **The adapter folds every process failure into `Unknown`**
  (`crates/ken-elaborator/src/prover.rs`, `attempt_d_with_z3_process`
  at `:505`: spawn failure, timeout, `unknown`, malformed output). So the
  test's panic hides which one happened.
- **The harness.** `stub` (`:71`) writes a shell script and sets it
  executable; the adapter then executes it. The file's five tests run on
  parallel threads, and three of them write and exec stubs.
- **Best current hypothesis, unmeasured.** A sibling thread's fork inherits
  the stub's still-open write descriptor, so the exec fails with `ETXTBSY`
  ("Text file busy"), a known Linux race for write-then-exec under
  parallel process spawning.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Scope

- Expected: the test file, and any test-support helper it alone uses.
- Must not move: the adapter's fail-closed behaviour and the verdict
  authority in `prover.rs`. Every process failure stays `Unknown`, and the
  kernel refutation check stays the only route to `Disproved`. A production
  change of any kind is out of scope.

## Deliverable

First, make the failure name its cause. For example, in the test, spawn the
stub directly and report the `io::Error`, or run the file under parallel
load until it reproduces. Then remove the measured race in the harness. For
`ETXTBSY`, write the stub under a temporary name, close it, and rename it
into place, or give each stub its own directory. Retrying the verdict
assertion does not count as a fix.

## Acceptance

- **AC-1 (reproduce, then fix).** Show a reproduction on the base, either
  a stress loop of the test file under parallel threads or a forced
  `ETXTBSY` with a held write descriptor, and show the named cause. The
  same loop passes on the candidate.
- **AC-2 (controls).**
  - The fail-closed tests still discriminate. A stub that prints a
    non-refuting model still gives `Unknown`, and
    `every_process_and_protocol_failure_is_unknown` stays green and
    unedited in meaning.
  - Reverting the harness fix brings back the reproduction.

## Stop conditions

- The measured cause is not in the test harness, for example an adapter
  defect in `prover.rs`. Stop to the Architect with the error text.
- No reproduction after a bounded stress run. Stop with the loop and its
  counts. Do not land a speculative fix.
