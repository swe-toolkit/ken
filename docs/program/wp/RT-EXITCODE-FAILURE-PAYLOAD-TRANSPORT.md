# WP frame — RT-EXITCODE-FAILURE-PAYLOAD-TRANSPORT (WITHDRAWN — falsified)

> # WITHDRAWN / FALSIFIED 2026-08-25 — do not implement this frame
>
> The Architect falsified this WP as a product object after the hard-stop #3
> research advisory (ruling evt_1vhmndq7fscd1, thr_305pn5gzx37h). The exact-Int
> carrier already admits every valid exit code and the two named process-exit
> consumers are not missing transport; the causal defect is a dynamic-constructor
> dispatch residual (`emit_carrier_dynamic_constructor`'s direct `return_(-3)` at
> `StaticOriginId(34)`), not an ExitCode payload gap. Do NOT resume D1 and do NOT
> ship the production refactor in `34ab178ac` (kept read-only as the probe
> checkpoint only). Replacement:
> `docs/program/wp/RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE.md`. The `-3`
> reporter alias is tracked separately as [[RT-UNIT-FAILURE-STATUS-PROVENANCE]].
> Node: [[RT-EXITCODE-FAILURE-PAYLOAD-TRANSPORT]] is `closed`. Everything below is
> retained for provenance and is superseded.

> M-series successor of [[RT-NATIVE-CARRIED-VALUE]], sequenced FIRST of M3's two
> exposed objects (Architect evt_4kkspzs62gtn6; the retained-call successor
> [[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]] is second, no fold, no technical
> dependency). Owning team: runtime. Size M. Capability tier: T1 (a
> soundness-bearing execution-parity closure that must reconcile TWO producer
> phases via their phase-appropriate landed observers without adding
> representation; review turns on the census argument, not a one-line diff).
>
> AMENDED 2026-08-25 (Architect hard-stop #1 ruling, evt_3kprh7knmxa3w,
> thr_305pn5gzx37h). The original frame over-bound reuse to one helper name
> (`narrow_carried_int_u64` at both phases). That single-observer sentence is
> WITHDRAWN: the semantic property is "observe exact `Int` in the phase actually
> held," not "route both phases through the carried observer." The two-surface
> census stands; the corrected component shape (two phase-appropriate observers, a
> factored carried decoder outcome below its effect-seat policy wrapper, and one
> shared exit mapper) is in Fixed inputs and Deliverables below.

## Objective

Close the process-exit consumers over the exact-`Int` carrier forms that already
exist, so a checked program crossing M3's effect seat maps its
`ExitCode::Failure` payload to a real exit code instead of forcing the native
sentinel `-3`. The representation and the observer already exist; the gap is
that the consumers admit only `ImmediateInt`. This is execution-parity work in
the process-exit/ExitCode native-trap family (value `-3`), distinct from
borrowed-input durability (`-1`, [[RT-BORROWED-INPUT-CARRIER-DURABILITY]]) and
the closed entry trap (`-2`, [[RT-ENTRY-TRAP-254]]).

## Fixed inputs (Architect re-measure at landed origin/main; amended by hard-stop #1)

- Two phase-appropriate observers already landed, and the correct split is
  already established at `effects.rs:1564-1586`:
  `LoweringOperand::Specialized(Lowered::Int)` narrows via `narrow_native_int_u64`;
  `LoweringOperand::Carried` narrows via `narrow_carried_int_u64`
  (`effects.rs:1589-1700`). Each phase must use the observer for the type it
  actually holds. Do NOT convert one phase's operand to the other's carrier form
  merely to reuse a helper name — that adds allocation/representation traffic to
  satisfy prose, not semantics, and violates subsume-don't-proliferate and the
  phase boundary's one-way discipline. Add no third `Int` representation and no
  duplicate persistent decoder.
- POLICY-EMBEDDING caveat (why the carried observer cannot be reused verbatim at
  the process-exit consumer): `narrow_carried_int_u64` embeds effect-seat POLICY,
  not just decoding — after `int_view` it calls `require_i64(status,
  BOUNDARY_OK)`, so a tag/class/owner/shape failure returns from the generated
  unit before the process-exit consumer can map it. Reusing that wrapper verbatim
  here would preserve the wrong policy and can surface sentinel `-1` (whose native
  report names a DIFFERENT object) instead of the honest `-3`.
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
- DECISIVE census note (STANDS): the Architect's scratch probe touching ONLY
  surface A's immediate-only arm did NOT green the witness. A one-site patch is
  insufficient; the WP must reconcile BOTH producer surfaces. (Only the separate
  claim "`narrow_carried_int_u64` is sufficient at each phase" is withdrawn —
  surface B uses `narrow_native_int_u64`, per the split above.)

## Deliverables

- D0 (buildability probe FIRST, the M6/M4/M3 pattern). Census both `-3` producer
  surfaces and confirm the PHASE-APPROPRIATE observer is reachable at each: the
  carried phase (`core.rs:11523`) reaches `narrow_carried_int_u64`'s decoder; the
  specialized phase (`calls.rs:2301`, surface B) reaches `narrow_native_int_u64`.
  Confirm no third representation and no `Lowered -> CarriedBoundaryWord`
  conversion is needed at surface B. Output: the exact set of sites the D1 change
  must touch, and the phase each trigger reaches.
- D1 (component shape per the Architect hard-stop #1 ruling; names are
  implementer-owned, the boundaries are the ruling):
  1. Factor the carried decoder BELOW its policy wrapper into one internal
     emitted outcome preserving three facts — decoded `value`, `fits_u64`, and
     the carrier decoder `status`. On a non-OK status it must NOT load the view's
     uninitialized fields; converge through an error block with inert value /
     false validity plus the original status.
  2. Keep `narrow_carried_int_u64` as the strict effect-seat wrapper: it calls
     that internal outcome, RETAINS `require_i64(status, BOUNDARY_OK)`, and
     returns the existing `(value, valid)` contract unchanged. Existing
     effect-seat wrong-tag/class/owner behaviour must not move.
  3. Add a process-exit CARRIED wrapper over the same internal outcome: it
     converts any non-OK carrier status to `valid = false`, then feeds the shared
     exit mapper — so malformed tag/class/owner/shape becomes `-3`, never a
     plausible code and never the unrelated `-1`.
  4. Keep surface B on `narrow_native_int_u64`: its structural field/arity/class
     failures and any unavailable specialized narrowing feed the SAME invalid
     input to the shared exit mapper. No carrier conversion belongs there.
  5. Factor the canonical failure mapping into ONE emission method used by both
     surfaces: invalid -> `-3`; valid zero -> `1`; valid `1..=255` -> value;
     everything else -> `-3`. This is the shared policy; the two observers remain
     phase-specific mechanisms.
  Also clean the trigger witness's stale "Ignored pending M3" narrative comment to
  name this WP as owner (the ignore attribute itself stays — this WP does not
  promise px8ta green; see below).

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
- AC-3 (no new representation, phase-appropriate observers). The change uses each
  phase's landed observer for the type it holds (carried decoder outcome for the
  carried phase; `narrow_native_int_u64` for surface B); it introduces no third
  `Int` carrier form, no duplicate persistent decoder, and no `Lowered ->
  CarriedBoundaryWord` conversion at surface B. Zero `trusted_base()` delta.
- AC-5 (the policy split is real — the crux of hard-stop #1). A carried
  wrong-tag, wrong-class, and wrong-owner/shape input reaching the PROCESS-EXIT
  carried wrapper yields `-3` (never `-1`, never a plausible code); the SAME
  malformed carried inputs routed through an existing effect seat still take the
  strict `narrow_carried_int_u64` wrapper's hard-refusal path (unchanged). This
  pair pins that the process wrapper and the effect-seat wrapper apply different
  policy over the shared internal outcome.
- AC-6 (error path does not consume the uninitialized view). A non-OK `int_view`
  status cannot cause a load from the view output. Prove it by mutating the status
  branch or poisoning the output and showing the error path does not read it.
- AC-7 (surface B stays native, no carrier traffic). Surface B's witness reaches
  `narrow_native_int_u64` directly; the change adds no carrier allocation or
  boundary transfer on the specialized phase.
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

Architect (component fit: each phase uses its phase-appropriate observer with no
added representation; the carried decoder outcome is factored below its
effect-seat policy wrapper; the process-exit carried wrapper yields `-3` not
`-1`; one shared exit mapper serves both surfaces — the policy split (AC-5) and
the two-surface census are the crux he flagged) + runtime-qa (the independent
per-phase mutation controls, the policy-split and uninitialized-load controls,
and the real-consumer acceptance pair). Adversary advisory, non-gating.

## Contention check

Touches `crates/ken-runtime` (`core.rs`, `cranelift_backend/lowering/calls.rs`,
`effects.rs`) and `crates/ken-cli/tests`. No overlap with lane 2
(language/elaborator) or lane 3 (foundation catalog packages). Runtime ring
exclusive; the runtime ring is idle-closed after M3.

## Capability tier

T1. Size M — one focused increment: a D0 census then a two-phase consumer
closure with independent controls. Sized to reach a releasable increment or a
genuine hard stop (px8ta advancing to a distinct object) within about an hour.
