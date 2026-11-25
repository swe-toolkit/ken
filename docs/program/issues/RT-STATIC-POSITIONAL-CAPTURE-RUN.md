---
id: RT-STATIC-POSITIONAL-CAPTURE-RUN
title: "Static aggregate planning describes a captured environment's positional children by slicing one fixed 64-entry static run, and refuses any record with more captures. PX11's exchange has 80 such records with a maximum of 79, and eight above 64, so the build refuses with N0002 in static planning. Grow one process-wide positional run by doubling, issue each arity its exact prefix, and refuse only an arity the u32 ordinal cannot name"
status: ready
owner: runtime
size: S
tier: T1
gate: architect
depends_on: []
blocks: [PX11-NATIVE-STREAM-SOCKET-PARITY]
github: null
origin: "Architect ruling evt_1552vcx50h6qt (arm C-cap) on PX11 Inc2's probe P-CAP (runtime-implementer evt_1xm9gkdg2tbrc). The ruling scopes it to its own node, independent of RT-ORIENTED-PARENT-EDGE-COMPOSITION. L1 Linux ABI program, Steward order."
---

# Positional capture runs of any arity

## Objective

Static aggregate planning serves a captured environment of any arity
the `u32` ordinal space can name. Every arity gets a positional child
model exactly its length, so PX11's exchange gets past static planning.

## Settled inputs (main `f19404552`; Architect `evt_1552vcx50h6qt`)

- **The witness.** The wrapped residual record at `aggregates.rs`
  ~`:6312` has 79 entries from 79 distinct source coordinates: one child,
  39 worker captures and 39 context captures. The exchange has 80 such
  calls, 41 of them above 32. `drain_full` is `[5, 5, 11]`, and p2 is
  `{8: 1}`.
- **The bound.** `CHECKED_IH_CAPTURE_OPERAND_LIMIT` (`static_transition/
  aggregates.rs:5201`), the static run `CHECKED_IH_CAPTURE_OPERANDS`
  (`:5211`), and `positional_capture_declared_children` (`:5224`). There
  are three callers (~`:6087`, ~`:6173`, ~`:6312`), and the lowering
  consumers in `lowering/aggregates.rs` (~`:4118`, ~`:4301`) compare
  lengths only.
- **The code.** The ruling gives the exact replacement function and its
  test, and the signature does not change. Build it as written.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The function, the deleted constant and static, and the replaced test,
in that one file. No change to the callers, the lowering consumers, any
fixture, or the exchange's profile.

## Acceptance

- **AC-1.** The ruling's test,
  `the_positional_run_serves_every_ordinal_arity_and_never_truncates`.
  m1 (the 64 refusal restored) reddens the 65, 79 and 1000 arities. m2 (no
  growth, truncating to the run) reddens the length assertion. The `u32`
  refusal row is CLAIMED and is not mutated.
- **AC-2.** The `ken-runtime` static_transition aggregates tests pass
  unchanged, apart from the replaced test.
- **AC-3 (measurement, not committed).** The exchange on Inc2, rebased
  onto this candidate and RT-ORIENTED-PARENT-EDGE-COMPOSITION. Report it
  by the ruling's arms K-a, K-next, K-sib and K-other, and by the 450 s W
  arms (`evt_4ah3748sfskdk`).
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

K-next, K-sib and K-other go to the Architect. K-next is inventory entry
9, so the §1b question comes before any ruling. Do not raise the
constant instead of growing the run.

Checks: 2 fired. One shared function serves all three callers, and the
consumers compare lengths only. 8 fired. m2 is the truncation the old
bound guarded against, and the length assertion catches it. 4 fired. I
read the constant, the static and the function on `f19404552`.
