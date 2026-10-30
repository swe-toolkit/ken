---
id: ABI-S5
title: "Spec 38 says nothing about signal disposition, although both run paths ignore SIGPIPE and the CLI worker catches SIGSEGV/SIGBUS. PX10 cannot state what a spawned child inherits until the executable edge's disposition is a contract. Rule from a measured inventory what the edge establishes and what a child must be reset to, state it as spec 38 §1.10 with a seed, and witness the posture on the running process"
status: merged
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [PX9]
blocks: [PX10]
github: null
origin: "docs/program/10-linux-abi-completion.md §4 Track S: ABI-S5 gates PX10, the Track T committed exit, because signal disposition is part of what a spawned child inherits. L1 objective: the Linux ABI program; ABI-S6 closes with ABI-S6-D5B-COW-DISCHARGE. Steward-filed per COORDINATION section 2."
---

# The executable edge's signal disposition is a contract

## Objective

Spec 38 §1.10 states which signal dispositions a Ken executable
establishes at entry, what a Ken program observes in place of each
ignored signal, and which dispositions a child process must start with.
A test reads the running Ken process's dispositions and finds exactly
the stated set.

## Settled inputs (main `db6d0fee4`, AC-0 `evt_6arm6g95v9rgg`)

- **No contract.** `spec/30-surface/38-ffi-io.md` has no signal or
  terminal section (§1.1 to §1.9, then §2). `spec/` mentions a fatal
  signal only as outside the obligation (`71-assumption-boundary.md:323`).
- **The measured inventory (AC-0 §2).** Two paths run a Ken program.
  - **`ken run`.** The program runs in the worker child that the CLI
    re-executes (`worker_process.rs:132`). In the parent and the worker,
    SIGPIPE is ignored by the Rust std runtime before `main`, and SIGSEGV
    and SIGBUS are caught by std's stack-overflow guard. The worker
    chains its fault recorder over that guard (`worker_process.rs:214`).
    glibc reserves signals 32 and 33. No mask is set.
  - **The native artifact.** It is C-started, so std's setup never runs.
    `sigpipe.c`, called from `establish_process_posture_v1`
    (`abi_v1.rs:108`), ignores SIGPIPE. Nothing is caught, and no mask is
    set.
  - Ken resets nothing it did not set. A launcher's own ignored signals
    and mask pass through both paths unchanged.
  - Every entry is justified, and nothing is removed.
- **The observables today.** In `console_exec.rs:45`,
  `closed_stdout_is_an_io_failure_not_sigpipe_termination` (`ken run`)
  expects `IOError.BrokenPipe`, not death by SIGPIPE. In `px4b:470`,
  `linked_console_broken_pipe_reaches_ken_instead_of_signal_termination`
  pins the same for the artifact.
- **Why PX10 needs it.** `exec` keeps ignored dispositions and resets
  caught ones to default. A child spawned from the edge would inherit
  `SIG_IGN` for SIGPIPE unless the spawn plan resets it.
- **Terminal basics already ships without a contract.**
  `HostOpV1::ConsoleIsTerminal` (`effect_v1.rs:20`) backs
  `proc is_terminal (stream : Stream) : IO Bool visits [Console]`
  (`prelude.rs:1917`). It runs natively (`abi_v1.rs:552`, `px4b:435`) and
  in the interpreter (`eval.rs:4315`). `spec/`, `conformance/` and
  `catalog/` never mention it.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

1. **The Spec Leader's ruling of §1.10**, from AC-0 §3 and §4. It
   states:
   - the edge's ignored and caught sets on each path;
   - the child rule: a spawned child starts with what the edge
     inherited, with each disposition the edge set returned to `SIG_DFL`.
     The mask is unchanged;
   - the contract of the shipped `is_terminal`, if the ruling keeps it
     in §1.10.
2. **One spec+crates branch.** It contains §1.10 and a conformance seed
   under `conformance/surface/ffi-io/`. It also contains a native test on
   each path, built as AC-0 §5 describes:
   - a clean `pre_exec` baseline;
   - a wait on the program's `read(0)`;
   - exact assertions on signals 1 to 31, for the `ken run` worker and
     for the artifact.

   If LANG-PRELUDE-FLOOR-CONSOLE-AND-CHAR lands first, the probe's
   `Stdin` is `ConsoleStream.Stdin`. The Spec co-turn owns `spec/`, and
   CV votes on `spec/` and `conformance/`.

## Acceptance

- **AC-1.** The native test passes on the candidate. Its asserted
  ignored and caught sets equal §1.10's sets as the seed lists them, and
  both sets come from the measured process, not from a constant in the
  test.
- **AC-2 (mutation, QA).** There are four mutants, one of each kind per
  path, as AC-0 §4(b) defines them. Each reddens its own path's AC-1
  row.
  - An extra `SIG_IGN` for SIGINT: in `main` for `ken run`, and in
    `sigpipe.c` for the artifact.
  - M-RT resets SIGPIPE in `main`. It also reddens
    `closed_stdout_is_an_io_failure_not_sigpipe_termination`.
  - M-C skips `sigpipe.c`. It also reddens
    `linked_console_broken_pipe_reaches_ken_instead_of_signal_termination`.
- **AC-3.** §1.10 names, for every entry in the AC-0 inventory, the
  disposition a child must start with, so PX10's spawn plan has a stated
  target.

## Stop conditions

- A new host operation, ABI or wire change, or any kernel or trust
  change, which is an operator stop.

Checks: 2 fired: AC-0 found the dispositions at four Ken sites and
three library defaults, by reading the live process on both paths. 8
fired: under a launcher that ignores SIGINT, a test without the clean
baseline passes the SIGINT mutant. M-C leaves `closed_stdout` green, so
each SIGPIPE mutant is paired with its own path's pin.
