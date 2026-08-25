# WP frame — RT-CARRIED-IH-DISPATCH-SITEOP (M3)

> Track-1 consumer M3 of [[RT-NATIVE-CARRIED-VALUE]]. Gated by the landed
> Track-1 D0 ([[RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION]], M6, merged) and
> unblocked further by the landed M4 ([[RT-CLOSURE-BOUNDARY-RESIDUAL]], merged
> `f02922221`). Owning team: runtime. Size M. Capability tier: T1 (a
> soundness-bearing carrier-representation upgrade; design front-loaded by the
> Architect, but the fail-closed marshalling invariant is load-bearing).

## Objective

A boundary-carried data value in the `CarriedWord` phase cannot present the
`ConstructorTag` an effect seat demands to marshal a sum-typed argument into a
host call, so object emission refuses. Upgrade the boundary-carried value
representation so a `CarriedWord` phase can PRESENT a finite, compile-time-known
constructor discriminant to the effect seat, which then dispatches on it —
making `avail.admits(ConstructorTag)` satisfiable for a genuinely discriminated
carried phase, and letting the checked carried-value rows execute correctly
through the effect seat.

## Design judgment (front-loaded; Architect ruling evt_3r7fhkcd3e)

M3 is a DISTINCT BUILD, not a collapse-to-re-point. The Architect ruled this at
the landed M4 SHA `f02922221`, grounded at the object DB:

- M3's object is DISTINCT from M4's. M4 (`BoundaryClosureEnvironment`) transports
  closure code identity + captures across the closure crossing; it does not give
  a DATA value a constructor discriminant. M3's gap is that a boundary-carried
  data value (`CarriedWord` phase) lacks the constructor discriminant an effect
  seat needs to marshal a sum-typed host argument. M4's machinery does not green
  the M3 rows — confirmed: the rows still refuse at the effect seat post-M4.
- SAME defunctionalization FAMILY, DIFFERENT application. The representation
  decision is M4's: a finite compile-time-known set, with the runtime carrying
  only the discriminant/data while code/tag identity stays compile-time
  (Ahmed-Blume). M3 applies it to DATA-constructor discrimination for host
  marshalling, where M4 applied it to closure-code identity. They share the
  DECISION, not a seam or a defect.
- Arity/family caution (from [[RT-NATIVE-CARRIED-VALUE]]) holds: M3 is the
  BoundaryCarrier/`CarriedWord` family; M6 was the `Call` family; they fail in
  OPPOSITE arity directions. Do not collapse them because the numbers rhyme.

REUSE, do not REIMPLEMENT (load-bearing — do not stand up a parallel
defunctionalization). Reuse M4's landed machinery and principle where they
overlap:

- the "runtime carries only the discriminant/data; identity stays compile-time"
  principle M4 established;
- the env-Record positional schema (`SynthesizedAggregateRole`,
  aggregates.rs:115; and the declared-children positional transport used by
  `BoundaryClosureEnvironment`, aggregates.rs:143) where a carried constructor's
  fields need positional transport;
- the finite-static-dispatch pattern (M4's fail-closed static apply) IF the
  `reject_carried_residual_arguments` dispatch facet is scoped in (see Scope) —
  that facet is literally the code-id + env-Record + finite-static-apply M4
  already built, so it re-points onto landed machinery rather than new
  invention.

## Fixed inputs (measured @ `f02922221`, verified)

- Production seam (the LIVE trigger of M3's rows):
  `crates/ken-runtime/src/cranelift_backend/lowering/effects.rs`, the effect-seat
  claim routine. `record.avail.admits(observed)` routes the seat (`:496`,
  `:505`, and `:797` for the `Direct` route); when a HostOp argument seat's need
  is `ConstructorTag` and the crossing value's phase is `CarriedWord`, `admits`
  is false and the routine errors at `:548`:
  `"seat {slot} of {operation} needs {need}, which it cannot observe in
  {observed}"`. A second effect-seat site carries the same structure at `:2045` /
  `:2125`.
- The refusal M3 removes is the LIVE-arm branch. It is reached only AFTER the
  dead-arm guard `effect_arm_is_provably_dead` (`:362`, called at `:538`) returns
  false — i.e. the arm is NOT provably dead, so `UnreachableArm` (`:299`) is NOT
  substituted at `:543`. M3 must not disturb the provably-dead-arm path (that is
  [[RT-DEAD-ARM-EFFECT-LOWERING]]'s).
- The rows (both already own `RT-CARRIED-IH-DISPATCH-SITEOP` and carry the exact
  effect-seat string, post-M4):
  - `crates/ken-cli/tests/px8f_buffer_native.rs:200`
    (`linked_checked_write_all_observes_short_progress_and_matches_interpreter`),
    seat `Argument(1)` of `FsOpen`.
  - `crates/ken-cli/tests/px8ta_oriented_subcontinuation.rs:372`
    (`px8ds_real_same_depth_path_runs_exact_edges`, HALF B), seat `Argument(0)`
    of `ConsoleIsTerminal`. This is the M4 re-point: M4's bind-continuation arm
    retired its prior closure refusal, and the row now reaches THIS distinct
    object-emission successor.
- Reuse anchors present at `f02922221`: `SynthesizedAggregateRole`
  (aggregates.rs:115), `BoundaryClosureEnvironment` (aggregates.rs:143).

## Scope

Frame M3 around the MEASURED seam (`effects.rs` effect-seat claim routine,
`:548`), NOT the filing-origin seam. The node's original title/objective named
`reject_carried_residual_arguments` (core.rs:2935); the issue doc itself records
that the effect-seat layer supersedes that characterization. Writing an AC
against core.rs:2935 would name a location a DIFFERENT node owns:
`reject_carried_residual_arguments`'s arity refusal ("an eliminated value, not a
callable, but the call provides 1") is [[RT-SITEOP-CARRIED-WITNESS]] D2's
(px8ta:255) — an AC-exact-wrong-location trap. M3's deliverables and ACs target
the effect-seat claim routine.

The `reject_carried_residual_arguments` dispatch facet is DEFERRED, not folded.
Whether M3's carrier upgrade also unlocks that dispatch — letting
[[RT-SITEOP-CARRIED-WITNESS]] D2's row re-point/collapse onto M3's landing — is a
downstream measurement and a Steward scoping call, not prejudged here. Both
facets are the same "`CarriedWord` lacks a discriminated identity" family, so a
unified carrier upgrade is the natural closure IF folded. The initial M3 scope is
the effect-seat marshalling seam (px8f + HALF B); measure the fold after the
carrier upgrade lands.

## Deliverables

1. The carrier-representation upgrade: a finite, compile-time-known constructor
   discriminant carried alongside the boundary-carried word, reusing M4's
   discriminant-only-at-runtime principle and the env-Record positional schema
   for any carried constructor's fields.
2. The effect-seat dispatch: the effect-seat claim routine admits a discriminated
   carried value at a `ConstructorTag` seat and dispatches on the presented
   discriminant, so a genuinely discriminated `CarriedWord` satisfies
   `avail.admits(ConstructorTag)` and marshals into the host call.
3. The two rows execute correctly end-to-end and are un-ignored ONLY on genuine
   green (px8f `FsOpen` Arg1; px8ta HALF B `ConsoleIsTerminal` Arg0).

## Acceptance criteria

- AC-EXEC (end-to-end, not cold-enum): acceptance is px8f
  (`linked_checked_write_all_...`) and px8ta HALF B
  (`px8ds_real_same_depth_path_runs_exact_edges`) EXECUTING correctly through the
  effect seat and matching the interpreter, then un-ignored per COORDINATION §8a.
  A lowering-enum `Completes` disposition is NOT acceptance: the cold row
  `rt_cold_lowering_path_enumeration.rs:543` is a cold-enum disposition, and
  cold-enum `Completes` is not parity-correct (Architect z203 guard
  evt_1vcwzkd3g0s1r). Do not accept on the cold-enum row.
- AC-FAILCLOSED (exactness, load-bearing): an effect seat admits a discriminated
  carried value ONLY when the `ConstructorTag` is genuinely present/transported.
  A still-opaque or wrong-tag carried value MUST still refuse at `effects.rs:548`.
  Control: a test that reds if an undiscriminated `CarriedWord` is admitted at a
  `ConstructorTag` seat (do not weaken `avail.admits` to a blanket accept).
- AC-DEADARM (no-regression on the neighbour): the provably-dead-arm path
  (`effect_arm_is_provably_dead` `:362` → `UnreachableArm` `:543`) is undisturbed.
  M3 removes only the LIVE-arm refusal. Control: the dead-arm substitution still
  fires where the arm is provably dead.
- AC-REUSE: no parallel defunctionalization machinery — the discriminant carrier
  reuses M4's principle and the env-Record positional schema. Reviewer-checkable
  at the diff (differential against a reimplementation).

## Contention check

Touches `crates/ken-runtime/src/cranelift_backend/lowering/effects.rs` (the
effect-seat claim routine) and the boundary-carried value representation on the
shared carried-value path, plus the `crates/ken-cli` px8f/px8ta rows. No other
lane touches this surface (runtime is the sole priority lane; language/foundation
are on their own tracks). The dead-arm neighbour ([[RT-DEAD-ARM-EFFECT-LOWERING]])
shares `effects.rs` but a different branch; AC-DEADARM guards the boundary. No
spec/, kernel/, or conformance/ touch anticipated.

## Sequencing

Releasable now: M6 (D0) and M4 are both merged. M3 is the direct Track-1
carried-value consumer and greens more rows (px8f + HALF B) than the execution
successor. [[RT-BORROWED-INPUT-CARRIER-DURABILITY]] (the px8l execution trap at
object_linker_packaging.rs:2221) is queued behind M3 as its own execution-parity
node (Architect stop-#1 ruling evt_2dnst700ynbeh); the Architect reviews it at
its WP release. The Architect reviews THIS WP at release for soundness and the
AC-FAILCLOSED / AC-DEADARM guards above.
