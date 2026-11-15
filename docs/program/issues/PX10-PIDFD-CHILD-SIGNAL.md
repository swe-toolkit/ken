---
id: PX10-PIDFD-CHILD-SIGNAL
title: "A PX10 child is held by its raw PID: wait_child uses waitpid and terminal settlement sends kill(pid, SIGKILL), so after an auto-reap (spec 38 §1.11.3's SIGCHLD residual) the kill can reach an unrelated process that reused the PID, and Ken has no way to signal a live child. Hold the child by a pidfd and add pidfd signalling, interpreter and native"
status: ready
owner: runtime
size: L
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "L1 objective: the Linux ABI program, docs/program/10-linux-abi-completion.md §4 PX10 (\"pidfd identity and signalling where available\"), in Steward order (operator 2026-10-30). Spec 38 §1.11 names pidfd signalling as successor work. Successor to PX10-NATIVE-SPAWN-PARITY B2 (0a5cf58eb). Steward-filed per COORDINATION section 2."
---

# Hold the child by a pidfd, and let Ken signal it

## Objective

A PX10 child is identified by a pidfd from spawn to settlement. Wait
and the terminal-error kill go through that pidfd, so neither can reach
another process. A Ken program can send a signal to a live child it
holds, with the same result on the interpreter and native engines.

## Settled inputs (main `273981a28`)

- **The identity is a raw PID.** `ChildProcessV1` holds `pid: pid_t`
  (`crates/ken-host/src/process_v1.rs:186`), which comes from
  `posix_spawn` (`:155`). `wait` calls `waitpid(self.pid, ..)` (`:211`).
  `settle(terminal_error)` sends `kill(self.pid, SIGKILL)` (`:237`) and
  accepts `ESRCH`. No product root uses a pidfd today: `pidfd` occurs
  only in spec 38 `:1233` and its seed `:490`, both as deferred work.
- **Why the hazard is real.** Spec 38 §1.11.3 (`:1215`) already admits
  a launcher that ignores SIGCHLD, so the OS may auto-reap the child.
  The PID can then be reused before terminal settlement, and the
  `kill` at `:237` signals whatever process holds it now.
- **The host primitives exist.** The locked `libc 0.2.186` exports
  `SYS_pidfd_open` (434), `SYS_pidfd_send_signal` (424) and `P_PIDFD`
  for `waitid`. The build host runs Linux 6.18. Taking the pidfd right
  after `posix_spawn` returns, before any wait, is race-free, because
  an unwaited child's PID cannot be reused. The auto-reap residual is
  the exception, and AC-0 rules it.
- **The operation family.** PX10 has `ProcessPlanOpen` to
  `ProcessWait`, `0601` to `0605` (`crates/ken-host/effect_abi_v1.catalog:105-109`),
  all `NativeTested` (`effect_v1.rs:227`). The public surface is
  `Capability.Process.Child` (`catalog/packages/Capability/Process/Child.ken.md`).
  Its private operations are prelude-registered, for example
  `PrivateProcessWait` (`crates/ken-elaborator/src/prelude.rs:2524`, `:2643`).
- **Contention.** `LANG-DELETE-LEGACY-RESOLUTION` I1b-b is building on
  `prelude.rs`, `modules.rs` and the catalog homes. A new private
  operation and its home touch the same files.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

`ken-host` process identity, wait and settlement; one new PX10 host
operation for signalling, carried through the effect ABI catalog,
erasure, the interpreter and native lowering; its prelude registration
and `Capability.Process.Child` home; spec 38 §1.11 and its seed. The
existing five operations' types and their public signatures do not
move. There is no kernel change and no TCB growth.

## Deliverable

1. **AC-0 first (Architect D0, on the cut base).** Rule these points:
   - the signalling operation's public type and authority, meaning
     the grant it needs, how revocation behaves, and the signal
     vocabulary;
   - its result on a waited or settled child;
   - the behaviour where `pidfd_open` is unavailable, so that "where
     available" is a stated contract rather than a silent fallback;
   - the auto-reap case, where the PID is already gone at spawn
     return;
   - the cut.
   The expected cut is in two increments:
   - **Increment 1** is pidfd identity, with no surface change. It is
     `ken-host` only and lands on its own.
   - **Increment 2** is the operation, with the spec enclave's §1.11
     text and seed. AC-0 names who owns its prelude and home half,
     and places it after I1b-b lands, or on paths disjoint from it.
2. **One runtime branch per increment**, with native and interpreter
   tests, following the D0.

## Acceptance

- **AC-1 (identity, Increment 1).** The child's pidfd refers to the
  spawned process. A test reads the pidfd's `Pid:` line in
  `/proc/self/fdinfo` and checks that it equals the PID `posix_spawn`
  returned. Wait and the terminal-error kill go through the pidfd.
  Differential, in the `ken-host` unit tests: start a live decoy
  process, and set the child's recorded PID to the decoy's while its
  pidfd still refers to the spawned child. Terminal settlement must
  observe the child `Signaled 9` and leave the decoy alive. A mutant
  that restores `kill(pid, ..)` or `waitpid(pid, ..)` kills or waits
  on the decoy, and so reddens this row. The existing PX10 rows stay
  green on both engines, including `rt_parity_native`'s request and
  outcome pairs.
- **AC-2 (signal, Increment 2).** A Ken program spawns a child that
  blocks, signals it with SIGTERM, and waits. It observes
  `ChildExit.Signaled 15` on the interpreter and natively, with equal
  request and outcome pairs. A signal sent after the child has been
  waited returns the error AC-0 rules, and it never reaches the
  process. Negative: a call without the ruled authority refuses.
- **AC-3 (no regression).** Full-workspace CI is green on each exact
  candidate.

## Stop conditions

- The signalling operation needs TCB growth, or a change to an
  existing PX10 operation's type. TCB growth goes to the operator.
- The pidfd cannot be obtained race-free for some spawn path that
  AC-0 did not rule.

Checks: 4 fired. The plan needs `pidfd_open`, `pidfd_send_signal` and
`waitid(P_PIDFD)` in Rust, and the locked libc's x86_64 gnu module
declares each one (`not_x32.rs:407`, `:417`; `linux_like/mod.rs:1268`).
3 fired. A grep for `pidfd` over `crates/`, `catalog/`, `spec/` and
`conformance/` found only the two deferral sentences. The
`process_v1.rs` callers of `pid()` and `wait` are the ring's sweep.
8 fired. A build that opened a pidfd but still killed and waited by
PID would pass the fdinfo row, since the two agree in every normal
run. AC-1's decoy differential separates them: only a pidfd-routed
kill and wait leave the decoy alive and observe the child.
