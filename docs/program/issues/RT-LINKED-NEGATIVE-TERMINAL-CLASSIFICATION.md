---
id: RT-LINKED-NEGATIVE-TERMINAL-CLASSIFICATION
title: "Linked reporting-boundary negative-terminal classification erratum — the linked negative-terminal wire domain is HETEROGENEOUS but RT-UNIT-FAILURE's new consumer treats the whole domain as planner-trap tokens, so the total decode_signed_root_trap refuses the fixed process-boundary sentinels (-1..-4) that emit_process_exit_status legitimately emits, hard-erroring valid programs (runtime-computed ExitFailure(300) -> -3). Partition the consumer into a disjoint union (fixed sentinels -> typed process-boundary Ok observation; 0xff-tagged signed root token -> planner provenance; else fail-closed), owned by ONE sealed sentinel authority the Rust classifier AND generated C both consume so the two reporters cannot drift. Consumer-classification closure, NOT producer-provenance closure."
status: draft
owner: runtime
size: M
gate: none
depends_on: [RT-UNIT-FAILURE-STATUS-PROVENANCE]
blocks: []
github: null
origin: "Steward, 2026-08-25, from the Architect ruling evt_530y3skvjk8y6 (thr_6gmh4p1m0gch4) on adversary M8 finding evt_4am5ebv5p4xnn against merged [[RT-UNIT-FAILURE-STATUS-PROVENANCE]] (2a5b3cd0e). The landed tree is byte-identical to the reviewed tree (contrary independent validation of the exact-SHA approval, not a landing mismatch). The Architect ruled the defect node-worthy, not deferrable, and directed it to PREEMPT [[RT-ITREE-DEFAULT-SELECTION-PROVENANCE]]. Steward owns the frame; the fix boundary, sealing requirements, and pins below are the Architect's ruling transcribed. Steward framing call per COORDINATION section 2."
---

> # DEFERRED 2026-08-26 — regression characterization RETRACTED (Architect hard stop evt_sj7wr86hw1w4)
>
> Hard stop 1 on the erratum mechanism. The Architect RETRACTED the
> `evt_530y3skvjk8y6` LIVE-regression classification: the required production premise
> is false on the current admitted surface. `ExitCode` is kernel-checked
> `Success | Failure UInt8` (`prelude.rs:2524`); a checked `Failure` payload cannot
> naturally be `300` (`Failure (add_int 299 1)` is correctly rejected `Int` vs
> `UInt8`). `int_to_uint8_raw` is an internal unchecked cast whose contract
> requires a prior range check; driving it with `300` violates that contract
> and the real bound
> path reaches `-1`, not `-3`. The only observed `-3` route used a naked runtime
> object plus a manually assembled `BoundProcessExecutableArtifact`, below the
> admission boundary — a low-level emitter/classifier measurement, not a checked
> bound-process program. So a "valid full program returning runtime-computed
> `ExitFailure(300)`" does NOT exist on the present production surface: there is no
> valid-program regression, and RT-UNIT-FAILURE's merge stands.
>
> RECLASSIFIED: urgent regression -> DRAFT/DEFERRED boundary investigation. The
> consumer-partition WIP `406e86742ba559b76b629c4502ac8db35affc526` is coherent as a
> mechanism but is NOT an authorized candidate; it is PARKED as evidence only and
> must NOT be folded into [[RT-ITREE-DEFAULT-SELECTION-PROVENANCE]]. Do NOT
> amend AC-1 to the naked-object/manual-artifact domain — that greens the
> test by moving the subject below the boundary this node claims. The typed
> partition is not landed
> merely because the generated C defensive reporter has branches.
>
> FIRST DELIVERABLE (before any mechanism work) — a closed production-reachability
> census:
> 1. Enumerate the negative terminal classes reachable through
>    `build_bound_process_starter_executable_artifact` from admitted checked
>    programs, without internal raw-cast precondition violations, naked
>    emitters, or manual artifact construction.
> 2. For each reachable fixed sentinel, name the exact typed producer authority and
>    give a natural full-program witness.
> 3. If NO fixed sentinel is reachable, record that the high-level consumer's refusal
>    is the correct invariant-violation boundary — do NOT land the typed partition.
> 4. If a fixed sentinel IS reachable, only then decide the high-level contract and
>    revive the typed partition against that natural witness.
>
> The raw-cast `-1` observation and both coherent-fixture `-1` observations are
> UNRESOLVED routes in that census, not proof of valid-program reachability; the
> naked-object `-3` is a low-level measurement only. The latent `joins.rs` provenance
> loss and vacuous artifact comparison remain separately latent. RT-ITREE is
> UNBLOCKED and re-released immediately from checkpoint `9faf52e5` — no repair
> lands first
> (Architect sequencing correction). `blocks` cleared.
>
> Everything BELOW is the SUPERSEDED urgent-regression framing, retained for the
> mechanism design (revived only under census step 4 above).
>
> # ERRATUM 2026-08-25 (SUPERSEDED) — LIVE regression on merged RT-UNIT-FAILURE; PREEMPTS RT-ITREE
>
> [[RT-UNIT-FAILURE-STATUS-PROVENANCE]] (merged `2a5b3cd0e`) narrowed its PRODUCER
> claim to two routes while making the linked CONSUMER global:
> `object_linker_packaging.rs` routes EVERY negative process terminal through
> `decode_signed_root_trap`, which refuses `-2`/`-3`/`-4`. But
> `emit_process_exit_status` (`calls.rs:2301-2352`), unchanged by that WP and outside
> its census, legitimately bare-emits the fixed process-boundary sentinels for valid
> programs. A well-typed program returning runtime-computed `ExitFailure(300)`
> (payload outside `0..=255`) reaches the established `-3` process-boundary result;
> `run_bound_process_effect_observation` then returns
> `Err(UnclassifiedRuntimeTrap { terminal_value: -3 })` and DISCARDS the otherwise
> complete observation (stdout, stderr, fs delta, effect trace, terminal). The
> generated C reporter still classifies that same value and returns normally, so the
> two reporters now DIVERGE on one live value. Fail-closed prevents accept-wrong, but
> losing a valid program's whole observation is a live correctness regression.
>
> The Architect's exact-SHA review verified the narrowed producers and missed the
> global consumer blast radius. This node repairs the CONSUMER; the landed
> signed-root-token producer mechanism stays intact. It preempts RT-ITREE (which
> re-anchors and replays its D0 evidence afterward), because current main regresses a
> valid program and RT-ITREE's full-program observations must be read through the
> repaired boundary. The ring is sequential.

## Objective

At the linked reporting boundary, classify the heterogeneous negative-terminal wire
domain into a disjoint union instead of treating all of it as planner-trap tokens.
The four fixed process-boundary sentinels are a DISTINCT typed domain — not planner
traps, not a second catalog, not a new wire code — owned by ONE sealed authority
that the linked Rust classifier and the generated C reporter both consume, so the
two reporters cannot drift again. The honest claim is consumer-classification
closure, NOT producer-provenance closure.

## Fixed inputs (Architect ruling; ground on current `origin/main` `e7dca2de7`)

- Base = current `origin/main` `e7dca2de7` (RT-UNIT-FAILURE landed; the
  signed-root-token producer mechanism and the artifact-bound planner catalog are in
  place and stay intact).
- The regressing consumer: `object_linker_packaging.rs` (~:327) routes every
  negative process terminal through `decode_signed_root_trap` (~:256), refusing
  `-2`/`-3`/`-4`.
- The legitimate sentinel producer OUT of conversion scope:
  `emit_process_exit_status` (`calls.rs:2301-2352`) — process-result validation
  outcomes authorized by `process_exit_status` / the root exit decoder, reached
  via the root `emit_result` `ProcessStatus` path (`calls.rs:2256`). Its `-2`/`-3`
  are NOT planner/graph trap occurrences.
- The existing C meanings of the four sentinels (the typed classes must match):
  `-1` malformed borrowed input, `-2` malformed `ExitCode`, `-3` malformed
  `ExitCode::Failure` payload, `-4` explicit entry trap.
- Repro (the required witness): a bound-process full program whose `main` returns
  runtime-computed `ExitFailure(300)`.

## The disjoint-union classification (Architect ruling — the fix boundary)

Every negative terminal at the linked reporting boundary maps to exactly one of:

1. A fixed process-boundary sentinel `-1`/`-2`/`-3`/`-4` -> return
   `Ok(EffectObservation)` with a typed process-boundary terminal error matching the
   existing C meaning above. This is NOT "no error" pass-through — the observation
   remains a controlled trap and preserves its exact process-boundary class, with all
   observation fields retained.
2. A well-formed signed root token whose magnitude carries the `0xff` tag -> decode
   through the exact artifact-bound planner catalog into `RuntimeTrapProvenanceV1`,
   UNCHANGED from the landed mechanism.
3. Every other negative -> fail closed as `UnclassifiedRuntimeTrap`.

Do NOT convert `emit_process_exit_status`'s `-2`/`-3` into planner traps — that would
lie about their authority and turn one heterogeneous-domain bug into a catalog
overreach. Express the four sentinel classes as ONE sealed typed process-boundary
enum carried by `TerminalErrorV1` (or an equally subsuming typed variant), NOT four
ad-hoc reporter branches and NOT `RuntimeTrapProvenanceV1` with fabricated
identities. ONE Rust authority owns the numeric sentinel values and meanings;
`emit_process_exit_status`, the linked Rust classifier, and generated C must consume
that authority or mechanically derived constants so the two reporters cannot drift.
This is a distinct process-sentinel domain, not a second planner trap catalog.

## Deliverable

- D1 — the sealed consumer classifier. (a) One total parser over `i64`: the
  four exact sentinels, the signed-token class, and a uniform fail-closed
  residual. (b) One exhaustive match over the sealed parsed type to build
  `TerminalErrorV1`; NO `_` arm over the completeness-critical typed class. (c)
  The single sentinel authority owning the four numeric values/meanings, consumed
  by `emit_process_exit_status`, the linked Rust classifier, and generated C (or
  mechanically derived constants). (d) The effect-wire encoder/decoder updated
  exhaustively for the new typed process-boundary error, round-tripping each
  class. Keep the fail-closed unknown residual and the planner-catalog mechanism
  intact.

## Acceptance criteria

- AC-1 (regression fixed) — a real bound-process full-program witness for
  runtime-computed `ExitFailure(300)` returns `Ok(EffectObservation)`, RETAINS all
  observation fields (stdout, stderr, fs delta, effect trace, terminal), carries the
  exact malformed-`ExitCode::Failure`-payload class, and AGREES with the generated C
  reporter's classification.
- AC-2 (sentinel pins) — `-1`, `-2`, `-3`, `-4` each classify to their exact typed
  process-boundary class (matching the C meanings), returning `Ok` controlled-trap
  observations, not `Err`.
- AC-3 (signed-token preserved) — one in-range `0xff`-tagged signed planner token
  still decodes to `RuntimeTrapProvenanceV1` through the artifact-bound catalog
  (the landed provenance witness stays green).
- AC-4 (fail-closed residual) — at least one non-tagged unknown negative still
  refuses as `UnclassifiedRuntimeTrap`.
- AC-5 (mutation controls, each compile-preserving + byte-restored) — mutating `-3`
  back into the planner decoder REDDENS the `ExitFailure(300)` full-program witness;
  admitting the unknown negative as any sentinel REDDENS the residual control (AC-4);
  treating the signed token as a fixed sentinel REDDENS the existing provenance
  witness (AC-3).
- AC-6 (structural seal) — the parser is total over `i64` and the `TerminalErrorV1`
  match is exhaustive with no `_` over the typed process-boundary class; the effect
  wire round-trips each class. Reintroducing a `_`/catch-all over the typed class, or
  a second reporter reading a raw sentinel constant not sourced from the single
  authority, is a review reject.
- AC-7 (claim scope / no drift) — the candidate claims
  CONSUMER-classification closure only, NOT global producer-provenance closure.
  It does NOT convert `emit_process_exit_status` into planner traps, does NOT
  fold the latent `joins.rs::emit_current_trap` bare-`-4` producer rewrite (that
  stays in the recorded global producer census/closure follow-up), and does NOT
  fold the vacuous artifact-binding comparison (inert cleanup, different
  predicate). The consumer partition MUST still classify `-4` honestly if it
  ever arrives.
- AC-8 (no successor bleed) — NO erasure, ITree-selection, witness-ignore, or
  SemanticError repair here; the SemanticError parity rows stay ignored for
  [[RT-ITREE-DEFAULT-SELECTION-PROVENANCE]]. `erasure.rs` byte-identical; zero
  `trusted_base()` change.
- AC-NO-REGRESSION — whole-suite green in CI; local targeted `-p ken-runtime` /
  `-p ken-host` / `-p ken-cli` / `-p ken-verify` only, never `--workspace`.

## Reviewers

Architect (the negative-terminal domain is partitioned into the exact disjoint
union; the four sentinels are a distinct sealed typed domain owned by one
authority the Rust classifier and C both consume; no planner-catalog overreach;
consumer closure honestly scoped; latent producer + inert cleanup NOT folded) +
runtime-qa (AC-5 mutation controls red as specified; the `ExitFailure(300)`
witness returns Ok with all fields and agrees with C; the structural seal AC-6
holds; fail-closed residual intact). No Decision fork — the Architect ruling
determines the boundary.

## Capability tier

T1 — a boundary-classification design over a heterogeneous wire domain, a sealed
typed-domain authority spanning two reporters, and a review that turns on the
partition/seal argument (not a differential diff). Size M.

## Sequencing

Lane-1 (runtime, priority). PREEMPTS [[RT-ITREE-DEFAULT-SELECTION-PROVENANCE]]
(Architect ruling): the runtime ring holds RT-ITREE at a clean D0 checkpoint
(D0 instrumentation preserved/committed, no D1), fixes this erratum from current
`origin/main` `e7dca2de7`, and RT-ITREE re-anchors on then-current main and replays
ONLY its D0 evidence/instrumentation into a fresh candidate afterward. This ordering
holds even if the consumer fix avoids `calls.rs` — the ring is sequential, current
main regresses a valid program, and RT-ITREE's full-program observations must be
interpreted through the repaired reporting boundary. After this lands,
RT-ITREE resumes, then [[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]] and the final
ReadEof/un-ignore/CI-rearm fold per the Steward sequence. The umbrella
[[RT-NATIVE-CARRIED-VALUE]] and PX8 stay blocked.
