---
id: LANG-ELAB-WORKER-LIFELINE-SPAWN-CODED
title: "The lifeline watcher LANG-ELAB-WORKER-PARENT-DEATH added starts with a bare std::thread::spawn, which reads RUST_MIN_STACK and panics on failure, so the child exits 101 uncoded where base gave C0022 or rc 0. Give the watcher a fixed stack, make enter_child fallible, and refuse with C0022"
status: merged
owner: verify
size: S
tier: T2
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary M8 finding on f266df257 (evt_9j4bc9avzzc6); Architect ruling evt_7p88cxwfmwnzf: real, its own S erratum, does not preempt LANG-TYPE-POSITION-SORT-UP-TO-WHNF, recommended for the Verify ring. Steward-filed per COORDINATION section 2."
---

# The lifeline watcher fails coded

## Objective

A failure to start the worker child's lifeline watcher is refused with
C0022, like a worker spawn failure, and never as an uncoded panic.

## Settled inputs (Architect `evt_7p88cxwfmwnzf`, on `f266df257`)

- **The defect.** `enter_child` (worker_process.rs ~:180) starts the
  watcher at ~:192 with a bare `std::thread::spawn`, which is
  `Builder::new().spawn(f).expect(..)` and reads `RUST_MIN_STACK`. A
  failure panics, the child exits 101, and `run_parent` returns 101. The
  worker spawn at main.rs ~:74 uses an explicit `stack_size` and maps
  `Err` to `CliError::ElaborationWorkerSpawn` (C0022, cli_errors.rs
  ~:59). The watcher is the only uncoded front-end spawn.
- **The ruled repair.** The ruling's code is the text.
  - `enter_child(fault, lifeline) -> std::io::Result<()>` spawns the
    watcher through `std::thread::Builder::new().name("ken-lifeline")
    .stack_size(64 * 1024).spawn(..)?`, with the read loop and SIGKILL
    unchanged.
  - The `Role::Child` arm in main.rs emits
    `CliError::ElaborationWorkerSpawn(error)` and exits 1 on `Err`.
  - The child never proceeds without its watcher. C0022 is reused: no
    new code and no spec 70 change.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The change in `crates/ken-cli/src/worker_process.rs` and
`crates/ken-cli/src/main.rs`, and one test file in `crates/ken-cli/tests`.

## Acceptance

- **AC-1 (durable row).** `ken check` on `const one : Nat = Suc Zero`
  with `RUST_MIN_STACK=100000000000` set gives rc 0 with base-identical
  stdout. The bare `std::thread::spawn` gives rc 101 with "failed to
  spawn thread".
- **AC-2 (measured once, not pinned).** A watcher spawn failure exits
  C0022 with rc 1 inside R2's RLIMIT_AS window, and the bare spawn gives
  101 there. The window is build-dependent, so it is candidate evidence
  only and must not become a test row.
- **AC-3.** `worker_spawn_failure_is_coded` and the parent-death rows
  P1-P3 stay green.

## Stop conditions

- A new error code, or any change to the parent path.
- Any elaborator, kernel, runtime or `trusted_base()` change.
