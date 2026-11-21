---
id: RT-TEST-PX10-AC2-MASK-CONTROL-QUIESCENT
title: "ac2_checked_interpreter_live_child_fd_and_signal_boundary reads the spawning thread's signal mask from /proc after the child publishes readiness, while that thread can still be inside the spawn's all-signals-blocked window, so its SIGUSR1-only control intermittently reads a full block and reddens CI on unrelated candidates. Sample the control at a point where the spawning thread is not spawning, keeping the control able to fail"
status: ready
owner: runtime
size: S
tier: T2
gate: architect
depends_on: []
blocks: []
github: null
origin: "Measured CI defect, second recurrence: I1b-a run 43634381146 and LANG-DELETE I4 run 44904939387 (job 136848455580) both reddened at the spawning-thread mask control with every signal blocked except SIGHUP, SIGKILL and SIGSTOP. Each cost a merge-queue retry. Steward-filed per COORDINATION section 2."
---

# The spawning-thread mask control reads a settled mask

## Objective

The AC-2 PX10 row's parent-mask control reads the spawning thread's mask
at a point where that thread is not inside a spawn. It still asserts
SIGUSR1 alone, and it can no longer read the transient full block.

## Settled inputs (main `df433d9ab`; re-check anchors at release)

- **The row.** `ac2_checked_interpreter_live_child_fd_and_signal_boundary`
  (`crates/ken-cli/tests/px10_spawn_plan_wait.rs:818`). The spawning thread
  installs `SignalBoundary` (`:716`), which sets the mask to SIGUSR1 alone
  (`:736`), then runs the Ken program.
- **The sample.** The test thread reads `/proc/self/task/{tid}/status`
  (`:871`) once the child's PID file appears, and asserts `SigBlk` equals
  the SIGUSR1 bit (`:946`).
- **The failure.** Both red runs read `SigBlk` = 2^64 - 0x40101, every
  signal except SIGHUP, SIGKILL and SIGSTOP. No Ken crate sets a signal
  mask outside tests, so the block comes from the spawn primitive's own
  parent-side window. The child can exec and write its PID file before the
  parent leaves that window.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

`crates/ken-cli/tests/px10_spawn_plan_wait.rs` only. The child-side
assertions (the child's empty mask, ignored set, descriptors) and the
production spawn path do not change.

## Acceptance

- **AC-1.** The control mask is sampled from the spawning thread at a
  point where no spawn is in progress, for example after
  `SignalBoundary::install` and before the Ken program runs. It asserts
  SIGUSR1 alone. The child-side rows are unchanged and pass.
- **AC-2 (QA mutation).** Dropping the `sigaddset(SIGUSR1)` from
  `SignalBoundary::install` reddens the control row. Restore it
  byte-identically.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The spawning thread's mask is still fully blocked after the spawn call
  has returned. That is a product leak in the spawn path, not a sampling
  race. Report it to the Architect.
- The repair needs a production-code change.

Checks: 4 fired. I read the row and `SignalBoundary` on `df433d9ab`, and
the I4 failure log; no non-test Ken source sets a signal mask. 8 fired.
AC-2's mutant leaves the child rows green and reddens only the control,
so the control still pins the parent's mask after the move.
