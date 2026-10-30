---
id: PX10-SPAWN-PLAN-WAIT
title: "A Ken program cannot start another program: there is no spawn host operation, and Capability.Process covers only the current process's argv, environment, working directory and exit. Give PX10 its first slice: a declarative spawn plan with explicit argv and environment bytes and a closed descriptor map, deny-by-default inheritance, the edge's SIGPIPE reset from spec 38 §1.10, and a wait that returns a typed exit or signal"
status: ready
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [PX9, ABI-M1, ABI-S5]
blocks: [PX10]
github: null
origin: "docs/program/10-linux-abi-completion.md §4 Track T: PX10 is operator-COMMITTED scope (processes), with a declarative spawn plan and no post-fork callback. ABI-S5 gives the child-disposition target. L1 objective: the Linux ABI program. This is PX10's first slice; pidfd signalling and the plan's later attachments are its successors. Steward-filed per COORDINATION section 2."
---

# A Ken program spawns a child and observes how it ended

## Objective

A Ken program builds a spawn plan, starts the child, and waits for it.
The plan holds a program path, argv and environment as `Bytes`, and a
descriptor map for stdin, stdout and stderr. Each entry in the map is a
fresh pipe, the null device, or one of the parent's three streams. The
wait returns `Exited code` or `Signaled signal`. The child receives only
what the plan names, and it starts with the dispositions spec 38 §1.10
requires.

## Settled inputs (main `bef52ee7c`; re-measure on the cut base)

- **The contract source.** The PX10 row of
  `10-linux-abi-completion.md` §4 is operator-committed. It calls for a
  declarative plan, with no post-`fork` Ken callback, and a plan that can
  later take namespaces, credentials, cgroups and sandbox policy without
  breaking.
- **Nothing spawns today.** `HostOpV1` has no process operation
  (`effect_v1.rs`, Console `0x01xx` to Entropy `0x0501`).
  `Capability.Process` holds `Arguments`, `Environment`, `Exit` and
  `WorkingDirectory`. They are views of the current process only.
- **The child target.** Spec 38 §1.10, landed with ABI-S5, says a child
  starts with the dispositions and mask the edge inherited, except that
  each disposition Ken established returns to `SIG_DFL`. Today that means
  an explicit SIGPIPE reset on both paths, and the mask unchanged, with no
  `SETSIGMASK` action. PX10 must reset SIGPIPE itself, not rely on a Rust
  runtime's spawn choice (`38-ffi-io.md:927-936`).
- **The error identity.** PX9's `System.Error` reaches process contexts
  (§4 PX9 row). A spawn failure, for example a missing program or a
  permission refusal, is a typed error under it.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

1. **AC-0 first.**
   - **The Architect's component design** covers:
     - the host operations and their wire;
     - the capability that authorizes a spawn;
     - how the child's descriptors are built so that nothing else is
       inherited, for example `posix_spawn` with an explicit file-action
       list and close-range;
     - how the pipe ends become Ken resources;
     - interpreter and native parity.
   - **The Spec Leader rules** the plan's public type in a new spec 38
     section, as an open record, the wait result and the error
     identities.
   - **Trust and scope.** Any trust growth is an operator stop. Anything
     beyond spawn and wait is a named successor.
2. **One spec+crates+catalog branch.** It holds the spec section, the
   host operations on both engines, the catalog surface under
   `Capability.Process`, a seed under `conformance/surface/ffi-io/`, and
   native tests. The Spec co-turn owns `spec/`, and CV votes on `spec/`
   and `conformance/`.

## Acceptance

- **AC-1.** On both engines, in parity, a Ken program spawns a child
  that writes its argv and selected environment to a stdout pipe. The
  parent reads exactly those bytes, and the wait returns `Exited 0`. A
  second child that kills itself with SIGTERM returns `Signaled 15`.
- **AC-2 (inheritance, live process).** While the child blocks, the test
  reads `/proc/<child>/fd` and `/proc/<child>/status`.
  - The child holds exactly descriptors 0, 1 and 2, although the parent
    holds a non-CLOEXEC descriptor at spawn.
  - SIGPIPE is not ignored.
  - The mask equals the parent's.
- **AC-3 (mutation, QA).** Each of two mutants reddens AC-2:
  - one that inherits the parent's descriptors;
  - one that skips the SIGPIPE reset.

## Stop conditions

- The plan needs a post-fork callback or Ken code in the child before
  `exec`.
- Any trust growth, or a kernel change. These are operator stops.

Checks: 4 fired: `HostOpV1` and `Capability.Process` were read on
`1da65f2cd`, and neither has a spawn or wait; ABI-S5's landing
(`bef52ee7c`) changed no `crates/*/src` path, and §1.10's child rule was
read on it. 8 fired: AC-2 measures
the live child, so a plan that relies on close-on-exec alone fails the
non-CLOEXEC parent descriptor. A spawn path that keeps an ignored SIGPIPE
fails the disposition row, and AC-3's mutants make both observable.
