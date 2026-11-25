---
id: RT-CHECKED-IH-FRESH-RESULT-CAPTURE-POPULATION
title: "Checked-IH continuation inheritance holds a fresh result's ordinary Ret/capture destination as one capturing closure, and refuses with N0003 'more than one ordinary fresh-result capture destination' when two sibling closures capture and read the same Ret constructor child. PX11's exchange main has two such transports into one frame. Record every capturing closure as a population over the one binder, validate each member, and refuse only distinct cases, distinct children or distinct predecessor frames"
status: ready
owner: runtime
size: S
tier: T1
gate: architect
depends_on: []
blocks: [PX11-NATIVE-STREAM-SOCKET-PARITY]
github: null
origin: "Architect ruling evt_66ga05tmvxvdw on PX11 Inc2's probe P-FR; Inc2 inventory entry 10. It closes the lanes.md carry from Adversary evt_69qysjcqjrkhm (an outer-match-bound value used in two sibling nested arms, each followed by a continuation). L1 Linux ABI program, Steward order."
---

# One fresh result, many capturing closures

## Objective

A fresh result whose Ret constructor child is captured and read by several
sibling closures plans natively, and agrees with the interpreter. PX11's
exchange gets past continuation inheritance.

## Settled inputs (main `f139b7817`; Architect `evt_66ga05tmvxvdw`)

- **The witness.** P-FR: both refused transports are at one active frame
  in main, in one non-recursive Ret case body, over one constructor child.
  Two sibling closures each capture that child and read it.
- **The record.** `CheckedIhFreshResultDestination` (`static_transition/
  aggregates.rs:667`) is singular. `fresh_result_destination` (`:6927`)
  refuses a second member at `:7049`. Its validation is in
  `validate_fresh_result_disjointness` (`:7176`) and
  `build_checked_ih_continuation_inheritances` (`:7365`). The one
  production consumer of the closure identity is
  `checked_ih_post_call_consumer_frames` (`:15018`), reached from
  `responses.rs:7039`.
- **The code.** The ruling gives Edits 1 to 6: the split record, the
  collected population, disjointness and validation over every member, the
  consumer as a frame set, and the test-support changes. Build them as
  written. The other multiplicity guards it names stay unchanged.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

`aggregates.rs` and the test file and fixtures the rows need. No change to
the sibling multiplicity guards the ruling keeps, to `lowering/source.rs`,
or to the exchange fixture or profile.

## Acceptance

The ruling's rows, built as written:
- **AC-1.** Fixtures P2 (two sibling arms read the result) and P1 (one arm
  does) in `rt_parity_native.rs` give `fresh_result_capture_count` 2 and 1,
  with the same frame, Ret body and child, and different closure origins. No
  numeric origins are pinned.
- **AC-2 (reported once).** P2 refuses at `f139b7817` with the `:7049`
  text. If it does not, rework the fixture, not the fix.
- **AC-3.** P2 native agrees with the interpreter (N-pass). N-later holds
  for the Architect, and N-same is a defect to fix.
- **AC-4.** m1 (`captures.truncate(1)`) reddens the P2 count. m2
  (SubstituteEarlierResult on a non-first member) refuses. m3 (the frame
  set as a Vec) is pinned red where reached, or stated as a GAP.
- **AC-5.** The 16 named existing rows pass in one `rt_parity_native`
  invocation with `--exact`, and `assert_local_shape` asserts a count of 1.
- **AC-6 (measurement, not committed).** The exchange on the Inc2 replay
  `6c7c5a2ac` with the candidate applied, reported by the arms X-art,
  X-limb, X-pred, X-case, X-child, X-same, X-valid and X-other.
- **AC-7.** Full-workspace CI is green on the exact candidate.

## Stop conditions

N-later, X-pred, X-case, X-child, X-same and X-other go to the Architect.
X-limb is the node's pass arm, and its later refusal is inventory entry 11.
Do not relax the distinct-case, distinct-child or distinct-frame refusals.

Checks: 4 fired. I read the record, the collector and its `:7049` refusal,
both validation functions, the consumer and its `responses.rs:7039` caller
on `f139b7817`. 8 fired. The unfixed code refuses P2, so the 2-against-1
pair cannot come from it, and m2 perturbs a non-first member. 2 fired. The
consumer fed by the same record is in scope, and the distinct-value
refusals stay.
