---
id: RT-NATIVE-TRACK0-REARM
title: "Track 0 of the native carried-value program — un-ignore the stale first-order native rows, re-measure, and re-arm the vacuous native CI jobs (decision-4 de-vacuuming), using the workspace ignored-sweep as the oracle"
status: ready
owner: runtime
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-08-22, filing Track 0 of [[RT-NATIVE-CARRIED-VALUE]] from the Architect's program frame (evt_9kat78d438cb). Shovel-ready: no new mechanism, the (need,phase) route protocol it relies on is already merged. Steward-filed per COORDINATION section 2."
---

> # SHOVEL-READY — Track 0 of [[RT-NATIVE-CARRIED-VALUE]]. No new mechanism.
>
> Frame: `docs/program/wp/RT-NATIVE-TRACK0-REARM.md`, inputs pinned @
> origin/main 6425709fb. Owner Runtime, size S.

> # CORRECTED 2026-08-22 — AC-2 STRUCK; Track 0 is pure hygiene
>
> Sweep-first measurement (runtime-implementer evt_4nmr897nemv8p) FALSIFIED the
> frame's Track-0-green premise before any row was committed — the oracle
> working as designed. Closing M1/M2 did NOT green
> `px8f_buffer_native.rs:203` / `px8f_write_partition.rs:354`; it MOVED them to
> the closure-boundary seam (`boundary.rs:1044`, M4's seam), because the
> checked-write full-programs carry the checked continuation closure. The
> Architect ruled the closure-boundary refusal is the SAME Track-1
> defunctionalization decision, not new and not a separate member.
>
> - AC-2 is STRUCK (falsified). Replacement: re-label the two rows to their
>   measured closure-boundary blocker; rows stay ignored pending Track 1.
> - Track 0 is PURE HYGIENE — re-label stale→measured, confirm M1/M2 gone
>   (verified), establish the sweep baseline. It lights NO PX8 witness green by
>   itself; ALL native full-program witnesses (ReadEof/ReadSome/Wrote AND
>   SemanticErrorV1) are Track-1-gated.
> - Deliverables 3/4 (CI re-arm, de-vacuum) correctly do NOT proceed for these
>   binaries while rows stay ignored — reported, not forced. native-write-
>   partition/native-buffer stay vacuously green; that de-vacuuming re-homes to
>   Track 1 (the rows un-ignore only when Track 1 greens them).
> - MERGE_READY increment: 0faa80980ab36828cccfe5d67dc1233882286c17, three test
>   files only, six stale labels re-pointed to their measured blockers (four
>   rt_parity rows → RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION; buffer/write-
>   partition → RT-CLOSURE-BOUNDARY-LANE); two already-accurate rows correctly
>   left un-churned. No ci.yml/registry/lowering change.
> - POINTER REFINEMENT owed before merge (Architect evt_399x8k6mxacwa):
>   RT-CLOSURE-BOUNDARY-LANE is MERGED, so the buffer/write-partition labels
>   point at closed work — the same stale-label defect in a new direction. The
>   LIVE owner of this residual closure-boundary population is
>   [[RT-CLOSURE-BOUNDARY-RESIDUAL]] (M4, under the Track-1 D0). Re-point those
>   two labels at RT-CLOSURE-BOUNDARY-RESIDUAL (naming RT-CLOSURE-BOUNDARY-LANE
>   as the resolved origin seam at boundary.rs:1044) before merge, then the label
>   names a seam AND a live owner. Closes on the re-pointed scope.

## Objective

The first-order carried-observation mechanism (the (need,phase)-keyed
EffectSeatClaimRoute protocol, effects.rs:495-505) is MERGED, but its native
full-program rows are still `#[ignore]`d and two native CI jobs are vacuously
green over a zero-test selection. Un-ignore the stale rows, re-measure native,
re-arm the jobs, and de-vacuum them per binary — lighting PX8's first-order
native witnesses (ReadEof, ReadSome, Wrote).

## Deliverables

1. Run the workspace `--run-ignored=only` sweep (ci.yml:142-166,
   `scripts/ci-ignored-sweep.py`) FIRST as the oracle for which ignored rows are
   now stale-passing — do not assert green from row prose.
2. Un-ignore the first-order stale rows: `px8f_buffer_native.rs:203`,
   `px8f_write_partition.rs:354`, and the ResourceRelease half of
   `rt_parity_native.rs:694`. Buffer + write-partition full-program rows are
   expected GREEN. The `rt_parity_native` rows advance to their Track-1 wall —
   re-label their `#[ignore]` to name the Track-1 seam (or leave for Track 1).
3. Re-arm the CI jobs: un-ignoring `px8f_write_partition:354` re-arms
   native-write-partition (ci.yml:250); `px8f_buffer_native:203` re-arms
   native-buffer (ci.yml:299) — automatically per those jobs' own comments.
4. Decision-4 de-vacuuming, per binary: once a binary has NO remaining ignored
   row, DROP `--no-tests=pass` from its job so a future re-ignore cannot
   silently re-vacuum it. Coupled to that binary's last row un-ignoring — NOT
   applied globally up front (that would red the board today). Remove
   un-ignored rows from the ignored-sweep `expected` set as they land.

## Acceptance criteria

- AC-1: the ignored-sweep runs clean and its `expected` set no longer lists the
  rows this WP un-ignores.
- AC-2: `px8f_buffer_native` and `px8f_write_partition` native full-program rows
  run and are GREEN, un-ignored, on the native backend.
- AC-3: native-write-partition and native-buffer CI jobs select a real test
  (no longer vacuous); `--no-tests=pass` dropped from any binary with zero
  remaining ignored rows. Control: confirm the job reddens if its selected test
  is made to fail, then restore.
- AC-SCOPE: no new lowering mechanism introduced; the (need,phase) route
  protocol is unchanged.

## Capability tier

T2 — mechanical measurement + CI bookkeeping over an already-merged mechanism;
the design judgment is front-loaded in the Architect's frame. Review is
differential (which rows, which jobs), not an argument.
