---
id: ABI-S5
title: "Spec 38 says nothing about signal disposition, although the native host already ignores SIGPIPE and the CLI worker installs SIGSEGV/SIGBUS handlers. PX10 cannot state what a spawned child inherits until the executable edge's disposition is a contract. Rule from a measured inventory what the edge establishes and what a child must be reset to, state it as spec 38 §1.10 with a seed, and witness the posture on the running process"
status: ready
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

## Settled inputs (main `95814a580`)

- **No contract.** `spec/30-surface/38-ffi-io.md` has no signal or
  terminal section (§1.1 to §1.9, then §2). `spec/` mentions a fatal
  signal only as outside the obligation (`71-assumption-boundary.md:323`).
- **What the code establishes.**
  - `establish_process_posture_v1` (`ken-host/src/abi_v1.rs:99`) calls
    the C companion `abi_v1/sigpipe.c`, which installs `SIG_IGN` for
    SIGPIPE, before admitting root execution (`:108`).
  - The CLI's elaboration worker (`ken-cli/src/worker_process.rs:175`,
    `:214`) installs SIGSEGV and SIGBUS recording handlers that chain to
    the previous action, and resets to `SIG_DFL` before re-raising
    (`:168`, `:252`).
- **The observable today.** `closed_stdout_is_an_io_failure_not_sigpipe_termination`
  (`ken-cli/tests/console_exec.rs:45`) runs a program whose stdout is a
  closed pipe and expects `IOError.BrokenPipe`, not death by SIGPIPE.
- **Why PX10 needs it.** `exec` keeps ignored dispositions and resets
  handled ones to default. A child spawned from the edge as it stands
  would inherit `SIG_IGN` for SIGPIPE unless the spawn plan resets it.
- **Terminal basics.** No host operation queries whether a stream is a
  terminal (`git grep -n isatty crates/` is empty).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

1. **AC-0 first (Architect, on the kickoff base).** A measured inventory
   of every signal disposition and signal mask the `ken` executable
   changes on the path that runs a Ken program, including whether that
   path runs inside the worker child. For each entry, give its site, its
   reason, and whether a spawned child must reset it. The Spec Leader
   then rules the §1.10 text. That ruling covers the inherited-disposition
   obligation PX10 must meet. It also rules whether terminal basics needs
   a contract now. If terminal basics needs a new host operation, it is a
   successor the ruling names, not part of this branch.
2. **One spec+crates branch.** It contains §1.10 and a conformance seed
   under `conformance/surface/ffi-io/`. It also contains a native test
   that reads the dispositions of the process running a Ken program, for
   example `SigIgn` and `SigCgt` in `/proc/<pid>/status` while the
   program blocks on input, and asserts the ruled set exactly. Any
   disposition AC-0 finds unjustified is removed in the same branch. The
   Spec co-turn owns `spec/`, and CV votes on `spec/` and `conformance/`.

## Acceptance

- **AC-1.** The native test passes on the candidate. Its asserted
  ignored and caught sets equal §1.10's sets as the seed lists them, and
  both sets come from the measured process, not from a constant in the
  test.
- **AC-2 (mutation, QA).** Two mutants each redden AC-1:
  - one that also ignores a signal §1.10 leaves at default, for example
    SIGINT;
  - one that drops the SIGPIPE ignore. That mutant also reddens
    `closed_stdout_is_an_io_failure_not_sigpipe_termination`.
- **AC-3.** §1.10 names, for every entry in the AC-0 inventory, the
  disposition a child must start with, so PX10's spawn plan has a stated
  target.

## Stop conditions

- AC-0 finds a disposition whose removal changes an existing test's
  observable outcome beyond the ruled set.
- A new host operation, ABI or wire change, or any kernel or trust
  change, which is an operator stop.

Checks: 2 fired: the dispositions were found at their two sites,
`sigpipe.c` through `abi_v1.rs:108` and `worker_process.rs:214`; the
fan-in of every path that sets one is AC-0's inventory, because a grep
for `sigaction` cannot see a library or runtime default. 8 fired: AC-1
reads the live process, so a test that asserts a constant would pass the
SIGINT mutant, and AC-2 requires that mutant to redden.
