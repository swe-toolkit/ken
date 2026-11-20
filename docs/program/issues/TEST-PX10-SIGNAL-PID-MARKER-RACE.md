---
id: TEST-PX10-SIGNAL-PID-MARKER-RACE
title: "revoked_child_signal_dispatch_reifies_without_delivery reads its child's PID marker while the child's shell has created the file but not yet written it, so the parse of an empty file panics and a CI shard is red with no product change. Make the marker read wait for the whole PID, so the row is deterministic"
status: active
owner: runtime
size: S
tier: T2
gate: runtime-qa
depends_on: []
blocks: []
github: null
origin: "Steward attribution evt_3mvhjd13q7a97 on SPEC-FFI-IO-PROCESS-NATIVE-STATUS PR #5410, run 44742686943 shard 2/7. Runtime-owned PX10 test; the Runtime ring is idle until PX11 lands, and no Verify seat is running. Steward-filed per COORDINATION section 2."
---

# The signal-dispatch row reads a complete PID

## Objective

`ken-interp`'s
`eval::px5b_effect_observation_tests::revoked_child_signal_dispatch_reifies_without_delivery`
passes or fails on the product, never on when its child's shell flushes
a marker file.

## Settled inputs (main `93b3d40b9`)

- **The race.** The child runs `printf '%s' "$$" > <marker>; exec sleep
  30` (`crates/ken-interp/src/eval.rs:8893`). The redirect creates the
  file before `printf` writes into it. The reader polls every 5 ms and
  parses the first content it reads, `.expect("child reported numeric
  PID")` (`:8913`-`:8919`).
- **The observation.** Run 44742686943 job 136310270959 panicked at
  `:8916` with `ParseIntError { kind: Empty }`. The candidate in that run
  touched only spec 38 and one conformance seed.
- **The population.** This is the only `$$` shell marker under `crates/`
  at `93b3d40b9`.

Treat anchors as perishable. If a fixed input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

This test's marker write or read. No change to the product, to what the
row asserts (revocation reifies without delivering the signal, and the
child's identity is unchanged), or to other tests.

## Deliverable

One Runtime-ring candidate. The repair hypothesis: the child writes to a
temporary name and renames it onto the marker, which is atomic on one
filesystem. Alternatively, the reader treats an empty or partial read as
not yet ready, until the deadline.

## Acceptance

- **AC-1 (mutation).** Insert a disposable `sleep 0.2` between the
  creation of the file the child writes and its `printf`, at the same
  point in the base and repaired scripts. The base test then fails with
  the empty-parse panic, and the repaired test passes. Remove the delay
  byte-identically.
- **AC-2.** The row passes 50 consecutive targeted runs locally
  (`scripts/ken-cargo test -p ken-interp --lib -- --exact <row>`), and
  full-workspace CI is green on the exact candidate.

## Stop conditions

- The repair needs a change outside this test, or to what it asserts.

Checks: 8 fired. AC-1's delay mutant forces the failing schedule, so a
repair that only narrows the window cannot pass it. 6 fired. I grepped
`$$` across `crates/` on `origin/main`, and this is the only site.
