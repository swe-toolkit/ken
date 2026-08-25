# WP frame — RT-EXITCODE-FAILURE-PAYLOAD-TRANSPORT (M3 successor, first)

> M-series successor of [[RT-NATIVE-CARRIED-VALUE]], sequenced FIRST of M3's two
> exposed objects (Architect evt_4kkspzs62gtn6; the retained-call successor
> [[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]] is second, no fold, no technical
> dependency). Owning team: runtime. Size M. Capability tier: T1 (a
> soundness-bearing execution-parity closure that must reconcile TWO producer
> phases and reuse the landed observer without adding representation; review
> turns on the census argument, not a one-line diff).

## Objective

Close the process-exit consumers over the exact-`Int` carrier forms that already
exist, so a checked program crossing M3's effect seat maps its
`ExitCode::Failure` payload to a real exit code instead of forcing the native
sentinel `-3`. The representation and the observer already exist; the gap is
that the consumers admit only `ImmediateInt`. This is execution-parity work in
the process-exit/ExitCode native-trap family (value `-3`), distinct from
borrowed-input durability (`-1`, [[RT-BORROWED-INPUT-CARRIER-DURABILITY]]) and
the closed entry trap (`-2`, [[RT-ENTRY-TRAP-254]]).

## Fixed inputs (Architect re-measure at landed origin/main 5fff430db)

- Observer already landed: `effects.rs:1589-1700`, `narrow_carried_int_u64` —
  representation-blind, reads both immediate and persistent exact `Int`. REUSE
  it; do not add a third `Int` representation or a duplicate persistent decoder.
- Producer surface A (carried phase): `core.rs:11523-11577`,
  `transfer_carried_failure_exit_status` — admits only
  `BoundaryTag::ImmediateInt`; every persistent exact `Int` goes to `-3`.
- Producer surface B (specialized phase): `calls.rs:2301-2370`,
  `emit_process_exit_status` — sibling; produces `-3` on an un-narrowable
  dynamic `Int`.
- `object_linker_packaging.rs:2223` REPORTS the sentinel only; NOT the defect
  site.
- Canonical exit mapping to apply after narrowing, in BOTH phases: `0 -> 1`,
  `1..=255 -> value`, out-of-range / malformed / wrong-class payload -> `-3`
  (honest reject).
- Trigger witness (still exactly as measured): `px8ds_real_same_depth_path_runs_
  exact_edges` (`crates/ken-cli/tests/px8ta_oriented_subcontinuation.rs:372`,
  HALF B) — executes to `RuntimeTrap(3)` / exit 1, stderr `malformed
  ExitCode::Failure payload`, trace `BufferAllocate`, `ResourceRelease`, ZERO
  `ConsoleIsTerminal`.
- DECISIVE census note: the Architect's scratch probe replacing ONLY surface A's
  immediate-only arm with `narrow_carried_int_u64` did NOT green the witness. A
  one-site patch is insufficient; the WP must reconcile BOTH producer surfaces.

## Deliverables

- D0 (buildability probe FIRST, the M6/M4/M3 pattern). Census both `-3` producer
  surfaces and confirm `narrow_carried_int_u64` is reachable and sufficient at
  each of the carried (`core.rs:11523`) and specialized (`calls.rs:2301`) phases
  to narrow a persistent/dynamic exact `Int`. Confirm no third representation is
  needed. Output: the exact set of sites the D1 flip must touch, and the phase
  each trigger reaches.
- D1. Close both consumers over the observer and apply the canonical mapping in
  both phases. Retain the fail-closed `-3` for genuinely malformed/wide/
  wrong-class payloads. Clean the trigger witness's stale "Ignored pending M3"
  narrative comment to name this WP as the owner (the ignore attribute itself
  stays — this WP does not promise px8ta green; see below).

## Acceptance criteria

- AC-1 (non-degenerate pair through the REAL consumer, each phase). In each
  reachable phase (carried and specialized), a valid dynamic `Failure 91` and a
  `Failure 92` map to exit status 91 / 92 respectively through the actual
  process-exit consumer; a malformed / out-of-range / wrong-class payload cannot
  be read as a valid exit code (maps to `-3` / honest reject); `Success` remains
  `0`. Not a unit test of the observer in isolation — assert at the consumer.
- AC-2 (independent per-phase mutation — the census is real, not forced).
  Surface A and surface B each have a witness that fails if THAT phase silently
  reverts to immediate-only. Mutating one phase's decoder must not be masked by
  the other: neither witness may pass with its own phase left immediate-only.
  (Guards against the one-site-patch failure the Architect measured.)
- AC-3 (no new representation, no duplicate decoder). The change reuses
  `narrow_carried_int_u64`; it introduces no third `Int` carrier form and no
  second persistent decoder. Zero `trusted_base()` delta.
- AC-4 (scope-honesty on px8ta). This WP does NOT un-ignore px8ta by fiat. Object
  completion = the process-exit boundary faithfully transports or honestly
  rejects. If, with both phases closed, `px8ds` runs genuinely end-to-end green,
  un-ignore it and pin the exit status; if it advances to a distinct nonzero
  outcome with a still-missing causal effect, STOP and hand back to the Steward
  to re-point a successor — do NOT widen this cut to chase it.
- AC-NO-REGRESSION. Whole-suite green in CI; the ExitCode `Success`=0 path and
  the entry-trap (`-2`) / borrowed-input (`-1`) behaviors stay green; local
  targeted `-p ken-runtime` / `--test` only.

## Reviewers

Architect (component fit: the consumer closure must reuse the landed observer,
add no representation, and the canonical mapping must be sound across both
phases; the census-both-surfaces requirement is the crux he flagged) +
runtime-qa (the independent per-phase mutation controls and the real-consumer
acceptance pair). Adversary advisory, non-gating.

## Contention check

Touches `crates/ken-runtime` (`core.rs`, `cranelift_backend/lowering/calls.rs`,
`effects.rs`) and `crates/ken-cli/tests`. No overlap with lane 2
(language/elaborator) or lane 3 (foundation catalog packages). Runtime ring
exclusive; the runtime ring is idle-closed after M3.

## Capability tier

T1. Size M — one focused increment: a D0 census then a two-phase consumer
closure with independent controls. Sized to reach a releasable increment or a
genuine hard stop (px8ta advancing to a distinct object) within about an hour.
