---
id: RT-NATIVE-RESOURCE-STREAM-WRITE-PROMOTION
title: "PX10 B2 promoted ResourceStreamRead and five Process operations to NativeTested but held ResourceStreamWrite at RepresentedUnavailable until Strict source could reach it, so a native Ken program cannot write to a child's stdin. Once LANG-PROCESS-CHILD-PIPE-WRITE-HOME homes write_pipe, produce the native differential parity witness for a child stdin write and then promote ResourceStreamWrite"
status: active
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [LANG-PROCESS-CHILD-PIPE-WRITE-HOME, RT-NATIVE-PIPE-EARLY-CLOSE-SETTLEMENT]
blocks: []
github: null
origin: "Architect evt_6ykbqn2mrxr4t on the PX10 B2 stop evt_70swd255qh2zx: B2 holds the ResourceStreamWrite promotion until a checked source home exists. effect_v1.rs names this follow-on. L1 Linux ABI program (docs/program/10-linux-abi-completion.md), Steward order. Steward-filed per COORDINATION section 2."
---

# Native child stdin writes

## Objective

A Strict Ken program that spawns a child, writes bytes to its stdin with
`write_pipe`, closes the write end and waits, produces the same terminal
result and the same ordered canonical requests, outcomes and bindings on
the native backend as on the interpreter. Then `ResourceStreamWrite` is
labelled `NativeTested`.

## Settled inputs (main after `LANG-PROCESS-CHILD-PIPE-WRITE-HOME` lands)

- **The label.** `HostOpV1::availability`
  (`crates/ken-host/src/effect_v1.rs:223` at `760b8eccd`) gives
  `ResourceStreamWrite` `RepresentedUnavailable`. Its doc comment
  (`:177`-`:185`) says this follow-on promotes it, and that the
  differential comes first and the label follows. The ABI catalog row is
  `effect_abi_v1.catalog:104`, `unavailable`.
- **The source home.** `LANG-PROCESS-CHILD-PIPE-WRITE-HOME` (routed
  `b88d12577`) exports `write_pipe`, `close_pipe_write` and
  `close_pipe_read` from `Capability.Process.Child`. `write_pipe` lowers
  to `PrivateResourceStreamWrite` (`prelude/process.rs:275`).
- **The precedent.** B2 (`0a5cf58eb`) promoted `ResourceStreamRead` with
  native differential rows in `crates/ken-cli/tests/rt_parity_native.rs`
  and `rt_native_settlement_placement_key.rs`, and updated the
  `px10_plan_census_tests` unavailable-operation census.
- **The obligations.** `conformance/surface/ffi-io/seed-spawn-plan-wait.md`
  :238-270 states the write, direction and early-close obligations.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Increments (Architect evt_4ry3fw55kaxhz)

- **Increment 1, now on `943d14a06`.** Native lowering for `take_stdin`:
  erasure gives the take-stdin constructor, matched by checked identity,
  an explicit literal operand 0 at `Argument(1)`, at every runtime
  `Effect` producer. One native row takes stdin and waits with no
  explicit close. No label change.
- **Increment 2, after `RT-NATIVE-PIPE-EARLY-CLOSE-SETTLEMENT` lands.**
  AC-1, AC-2 and AC-3 below, and the label flip. It also corrects the
  `px10_plan_census_tests.rs:113`-`:117` comment.

## Scope

The native parity rows for a child stdin write, the availability label and
the catalog row, and the census and test updates that the label forces.
No spec, catalog-package, authority or trust change. Any lowering repair
the differential exposes is in scope only if the Architect rules it part
of this promotion.

## Deliverable

One runtime candidate, as for B2's `ResourceStreamRead` promotion.

## Acceptance

- **AC-1 (differential before label).** A native row spawns a child that
  echoes its stdin (for example `cat`), writes a fixed non-empty byte
  string, closes the write end and reads the child's stdout. Both engines
  return the same bytes and the same ordered canonical requests, outcomes
  and bindings. The row runs and passes on the candidate's parent with
  the label still `RepresentedUnavailable`, through whatever route the
  ruling admits, or it records why it cannot. The label flips only in the
  same candidate as a passing row.
- **AC-2 (early close).** A second row closes the write end before any
  write and waits. Both engines agree on the child's end-of-input result.
- **AC-3 (label pin).** Flipping `ResourceStreamWrite` back to
  `RepresentedUnavailable` reddens a named row: the native route refuses
  the program.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The native write needs a lowering, planner or ABI change the Architect
  has not ruled in scope.
- The two engines disagree on any canonical request or outcome. Report the
  differential; do not relabel.

Checks: 3 fired. `ResourceStreamWrite` is the only Resource stream
operation still `RepresentedUnavailable`, and no native test issues it
(`px10_plan_census_tests.rs:113` says the census cannot). 8 fired: AC-3's
revert mutant leaves the program issuing an unavailable operation, so the
native route must refuse it, and that row fails if the label does not
gate. 9 fired: the frame waits on the PIPE-HOME landing because the rows
need its imports; it is released when that landing is on main.
