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
- **The ruled mechanism: a lifeline pipe, with the child authenticated by
  the pipes it holds (Architect `evt_3m6y438gmkd8v`, superseding the
  `getppid` check).** A child whose parent dies before `role()` is
  reparented, so a `getppid` comparison sends it down the re-exec path and
  it never reaches the lifeline.
  1. The marker has 7 fields: `<fault fd>:<parent pid>:<fault dev>:<fault
     ino>:<lifeline fd>:<lifeline dev>:<lifeline ino>`. `run_parent`
     creates the lifeline pipe, sets FD_CLOEXEC on its write end before
     spawn, leaves the read end inheritable, and holds the writer through
     every exit, including the early `return 1` paths.
  2. `role()` returns `Child { fault, lifeline }` when both pipe identities
     match. Otherwise it returns `ProtocolFault` when the marker pid is
     `getppid()`, and `Parent` when it is not. A malformed marker gives
     `Parent`.
  3. `enter_child(fault, lifeline)` sets FD_CLOEXEC on both descriptors,
     installs the recorders as today, and then spawns the watcher before it
     returns. The watcher retries `read(lifeline)` on EINTR and, on EOF or
     any other result, calls `libc::kill(libc::getpid(), libc::SIGKILL)`.
     `main.rs` passes both descriptors.
- **Boundary.** The child is gone within watcher-scheduling latency of the
  parent's death, in the startup window as in the steady state. It is not
  "zero instructions after parent death". A synchronous `poll` before
  returning was rejected, because no row can show it does anything.
- **Rejected:** PR_SET_PDEATHSIG (Linux only, fires on the forking
  thread's death) and signal forwarding (misses SIGKILL and the
  early-return path, and is redundant since the caller observes the
  parent's status).
- **Accepted residual, recorded and not fixed.** SIGSTOP or SIGCONT to
  the CLI pid alone stops only the parent. It is not a cancellation path.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The lifeline in `crates/ken-cli/src/worker_process.rs` and the `main.rs`
arm, one new test file in `crates/ken-cli/tests`, and the AC-2 Row E
fixtures in `tests/lang_elab_native_stack_depth.rs` moved to the 7-field
spelling where they build a well-formed marker. No elaborator, kernel or
runtime change.

## Acceptance

Rows P0-P3 are red on `f06db5ab4` and green after the repair.

- **P0 (parent died before `role()`).** The harness creates both pipes
  with raw `libc::pipe`, sets FD_CLOEXEC on the lifeline write end and
  closes it before spawn. It writes the 7-field marker with the correct fds
  and identities and the pid of a spawned-and-reaped process, then spawns
  `ken run` on the delayed-effect fixture. The child dies by SIGKILL, the
  effect file is never written, and no worker output appears. No hold hook
  is added to production code.
- **P1 (KILL).** `ken run` on R2's delayed-effect program. Once the
  child exists, SIGKILL the parent pid. Within 2 s the child pid is gone,
  and after the effect's deadline its output was never written.
- **P2 (TERM).** The same with SIGTERM. The parent's wait status is
  signal TERM, and the child is gone within 2 s.
- **P3 (check).** R1: `ken check` on a long elaboration, parent KILLed.
  The child is gone within 2 s.
- **Controls.** A normal `ken run` keeps its exit code and output.
  AC-2 rows A-E (C0023 overflow, C0022 protocol fault) are unchanged. A
  foreign `1:1:0:0` is now malformed and still gives `Parent` and a fresh
  parent (C0023).
- **E′.** A well-formed 7-field marker naming pipes the process does not
  hold, with a pid that is not its parent's, gives `Parent`.
- **Parent-claimed wrong identity.** The 7-field form with the real
  parent's pid and a wrong pipe identity gives `ProtocolFault`, C0022.
- **Mutations (QA).** Removing the watcher thread's spawn turns P1-P3 red
  while A-E stay green. Restoring the `getppid` comparison ahead of the
  identity check turns P0 red: the child re-executes as a parent and the
  effect is written. Each mutation is restored byte-identically.

## Stop conditions

- Any elaborator, kernel, runtime or `trusted_base()` change.
- A target where the lifeline cannot be authenticated like the fault fd.

## Hard-stop inventory (§1b)

§1a: 0. The pre-`role()` interleaving (`evt_4wes0t9y97fj4`) was a ruling
question answered before building (`evt_3m6y438gmkd8v`).
