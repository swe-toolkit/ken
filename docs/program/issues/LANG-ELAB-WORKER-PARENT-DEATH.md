---
id: LANG-ELAB-WORKER-PARENT-DEATH
title: "Since LANG-ELAB-NATIVE-STACK-DEPTH AC-2, a signal sent to the ken CLI pid alone kills only the parent: the re-exec child is reparented, keeps running with the caller's stdio, and ken run performs host effects after its supervisor killed it. Tie the child's life to the parent with an authenticated lifeline pipe whose EOF kills the child"
status: ready
owner: verify
size: S
tier: T2
gate: architect
depends_on: [LANG-ELAB-NATIVE-STACK-DEPTH]
blocks: []
github: null
origin: "Adversary M8 finding on f06db5ab4 (evt_2k35d730jf503); Architect ruling evt_1bv82pwc0dca6: real, a regression AC-2 introduced, one S node for the verify ring, ranked ahead of diagnostics and ergonomics work. Steward-filed per COORDINATION section 2."
---

# The worker child dies with its parent

## Objective

Killing or terminating the `ken` process a caller started stops all of
its work and effects, as it did before AC-2.

## Settled inputs (Architect `evt_1bv82pwc0dca6`, on `f06db5ab4`)

- **The defect.** `run_parent` (worker_process.rs ~:73-130) spawns the
  child with inherited stdio and blocks in `child.wait()`. Nothing ties
  the child's lifetime to the parent: no PR_SET_PDEATHSIG, no lifeline,
  no parent-pid check after `role()`'s startup check (~:54). A signal to
  the CLI pid alone (TERM, KILL, HUP, a `timeout(1)` expiry) orphans the
  child. The parent's `child.wait()` error path (`return 1`, ~:111-116)
  does the same. Ctrl-C at a terminal is unaffected.
- **Repros (Adversary).** R1: `ken check` on a long elaboration, parent
  KILLed, child keeps running. R2: `ken run` writes its effect 15 s after
  the parent's SIGKILL. R3: `ken repl` after TERM leaves the child alive.
- **The ruled mechanism: a lifeline pipe.**
  1. In the parent, before spawn, create a second pipe with FD_CLOEXEC
     on its write end, so only the parent holds it. Pass the read end to
     the child, authenticated like the fault fd by extending the marker
     with its `fd:dev:ino`. The parent keeps the write end open until it
     returns.
  2. In the child, in `enter_child` before the worker spawns, set
     FD_CLOEXEC on the read end. Spawn one thread that blocks in
     `read(lifeline)`, and on EOF or error calls
     `libc::kill(libc::getpid(), libc::SIGKILL)`.
- **Rejected:** PR_SET_PDEATHSIG (Linux only, fires on the forking
  thread's death) and signal forwarding (misses SIGKILL and the
  early-return path, and is redundant since the caller observes the
  parent's status).
- **Accepted residual, recorded and not fixed.** SIGSTOP or SIGCONT to
  the CLI pid alone stops only the parent. It is not a cancellation path.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The lifeline in `crates/ken-cli/src/worker_process.rs`, plus `main.rs`
only if the marker plumbing needs it, and one test file in
`crates/ken-cli/tests`. No elaborator, kernel or runtime change.

## Acceptance

Each row is red on `f06db5ab4` and green after the repair.

- **P1 (KILL).** `ken run` on R2's delayed-effect program. Once the
  child exists, SIGKILL the parent pid. Within 2 s the child pid is gone,
  and after the effect's deadline its output was never written.
- **P2 (TERM).** The same with SIGTERM. The parent's wait status is
  signal TERM, and the child is gone within 2 s.
- **P3 (check).** R1: `ken check` on a long elaboration, parent KILLed.
  The child is gone within 2 s.
- **Controls.** A normal `ken run` keeps its exit code and output.
  AC-2 rows A-E (C0023 overflow, C0022 protocol fault) are unchanged.
- **Mutation (QA).** Removing the watcher thread's spawn turns P1-P3 red
  while A-E stay green.

## Stop conditions

- Any elaborator, kernel, runtime or `trusted_base()` change.
- A target where the lifeline cannot be authenticated like the fault fd.
