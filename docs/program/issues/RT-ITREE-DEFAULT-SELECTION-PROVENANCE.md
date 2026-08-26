---
id: RT-ITREE-DEFAULT-SELECTION-PROVENANCE
title: "AC-5 successor exposed by identity-preserving trap provenance — an admitted checked program's runtime lowering SELECTS the generated ITree-elimination fail-closed default (planned identity 40, PatternMatchFailure 'no runtime match case selected for ...::ITree') where the synthesized Err InvalidOffset should continue through the checked ITree/result path. The erasure default VALUE (erasure.rs:2740-2744) is CORRECT and fail-closed; the first causally wrong authority is the lowering branch that selected it, and identity 40 cannot locate the occurrence (Planner::intern_trap value-dedups). First deliverable is occurrence localization by graph-derived StaticOriginId, not repair."
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-UNIT-FAILURE-STATUS-PROVENANCE]
blocks: []
github: null
origin: "Steward, 2026-08-25, from the Architect AC-5 ruling (evt_3rq4xafrf7cqf, thr_6gmh4p1m0gch4) on runtime WIP 7094c29cd. Preserving the root generated-unit failure's planned trap identity (RT-UNIT-FAILURE-STATUS-PROVENANCE) exposed a DISTINCT downstream producer: the checked ITree-elimination default is selected for an admitted program. The Architect ruled AC-5 valid and directed this be framed as its own object (explicitly NOT the prohibited -3-vs-4 'third node'; it is a different producer requiring its own object read). Steward framing call per COORDINATION section 2."
---

> # D0b AMENDMENT 2026-08-26 — D0 hard stop 1 (Architect object ruling evt_ts40fq959fvx)
>
> D0 evidence `49a7aff4999f3f6f64a9b5df790bc27da87183b7` (tree `f96ce855`, base
> `0253dd0f6`, four-path `+119/-2`) is ACCEPTED as PARTIAL: it localizes the
> terminal-producing outer carried `ComputationalMatch` to graph occurrence `301`
> (read) / `314` (write), owned by `main` at source path `[0,1,1,0,0,2,1,0,0]`,
> checked frame 1, invocation 0; forcing only 301/314 changes the terminal, forcing
> neighbors does not; NO repair site selected. It does NOT establish that the
> observed `ResourceBodyResult` default is a second natural product defect — that
> default appears only AFTER `KEN_RT_ITREE_D0_FORCE_ORIGIN` makes an origin-keyed
> bypass of the predecessor-route authority (`eliminator.answer_route ==
> CheckedSelectedRecursor` weakened to "checked OR this origin named"). A failure
> downstream of an artificial bypass is probe aftermath until the bypass's semantic
> authority is established.
>
> Therefore: NO D1 and NO ResourceBodyResult repair node. Runtime Leader HOLDS at
> `49a7aff49`; resumes D0b ONLY after this amendment; returns the next object read.
> Hard-stop count 1 (no Research trigger). Identities 36/37 and populations read
> `451/661`, write `464/675/887` are retained as probe-aftermath evidence ONLY
> (value-deduplicated; none localizes an occurrence; no family-specific repair
> authorized). Symptom entry 1 folded below (was Architect one-file commit
> `bdbd82c1`). D0b requirements are the Deliverables/AC-D0b-* below.
>
> # RE-RELEASED 2026-08-26 — erratum preemption RETRACTED (Architect hard stop evt_sj7wr86hw1w4)
>
> The RT-UNIT-FAILURE "LIVE regression" that preempted this node is RETRACTED. The
> Architect ruled (hard stop 1 on the erratum mechanism) that the `ExitFailure(300)
> -> -3` witness lives BELOW the admission boundary: `ExitCode` is kernel-checked
> `Success | Failure UInt8` (`prelude.rs:2524`), a checked `Failure` payload cannot
> naturally be `300`, and the only `-3` route used a naked runtime object plus a
> manually assembled artifact — not a checked bound-process program. No natural
> admitted producer reaches a fixed negative sentinel, so there is no valid-program
> regression, and RT-UNIT-FAILURE's merge stands.
> [[RT-LINKED-NEGATIVE-TERMINAL-CLASSIFICATION]] is reclassified to a DEFERRED
> boundary investigation (production-reachability census first); its WIP `406e86742`
> is parked as evidence only and must NOT be folded here.
>
> NO regression repair lands first. This node resumes NOW from current `origin/main`
> `0253dd0f6` (a doc-only advance over the `e7dca2de7` it was cut from — identical
> crate base), replaying ONLY its committed D0 evidence/instrumentation (checkpoint
> `9faf52e5`) into a fresh candidate. Read its full-program observations through the
> CURRENT reporting boundary — the earlier "read through the repaired boundary"
> conditioning is VOID (no repair). The D0 prohibition is unchanged: do NOT select a
> repair site before the graph authority is known. `depends_on` drops the erratum.
>
> # RELEASED 2026-08-25 (SUPERSEDED by RE-RELEASED above) — its base `2a5b3cd0e` is stale; the current re-release base is `0253dd0f6`
>
> [[RT-UNIT-FAILURE-STATUS-PROVENANCE]] merged at `2a5b3cd0e` (honest witness-path
> provenance: the exact row now reports planned identity `40` /
> `PatternMatchFailure`, not `unknown terminal sentinel`). This node's `depends_on`
> is satisfied; flipped to
> `ready`. Runtime resumes here NEXT in the lane-1 sequence;
> [[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]] (ReadSome/Wrote) and the final
> ReadEof/un-ignore/CI-rearm fold follow after this lands — none co-run, all touch
> `calls.rs`.
>
> BASE re-grounded to current `origin/main` `2a5b3cd0e`: build D0's occurrence
> localization against the LANDED honest-red witness there, not the superseded WIP.
> WIP `7094c29cd` in Fixed inputs remains only the exposing evidence; the trigger
> rows (the read-offset + write-offset positioned full programs) now reproduce the
> identity-`40` / `PatternMatchFailure` state directly on `2a5b3cd0e`. Reviewers
> Architect + runtime-qa unchanged; D0 may HARD-STOP to an Architect object read if
> it exposes a further distinct authority (do NOT repair from current evidence).
>
> # FRAMED 2026-08-25 — distinct AC-5 successor (Architect ruling evt_3rq4xafrf7cqf)
>
> Exposed by the AC-5 hard stop on WIP `7094c29cd`: once the root generated-unit
> failure preserves its planned identity `40` instead of collapsing to `-4`, the
> row reports `RuntimeTrap(10495)` whose recoverable identity `40` maps (same-build
> catalog read) to `RuntimeTrap { code: PatternMatchFailure, message: "no runtime
> match case selected for decl:rt_parity_fs_read_at_offset_single::ITree" }`. This
> is a DIFFERENT producer from the `-4`/`-3` identity-collapse predicate — a real
> separate object, not a third status-provenance node.
>
> Sequenced AFTER [[RT-UNIT-FAILURE-STATUS-PROVENANCE]] (honest reporting lands
> first) and must not co-run with it. This node greens the SemanticErrorV1 witness
> that RT-UNIT-FAILURE-STATUS-PROVENANCE deliberately leaves red.

## Symptom inventory

Append one line per hard stop; never rewrite history.

1. Forcing the localized outer ITree carried match through an origin-only
   checked-return bypass progresses into value-deduplicated `ResourceBodyResult`
   defaults instead of `InvalidOffset` — keyed on an artificial predecessor-route
   bypass, not a planner-authorized route.

## Layer and first authority (Architect ruling — read before framing a repair)

- The failure is an ITree-elimination default SELECTION for an admitted program.
  The synthesized `Err InvalidOffset` should continue through the checked
  ITree/result path; instead the runtime lowering takes the fail-closed default.
- NOT the defect: `ken-elaborator/src/erasure.rs:2740-2744` is the first producer
  of the typed fail-closed default VALUE (`lower_body_term_with_plans` Match arm,
  `RuntimeTrap(PatternMatchFailure, "no runtime match case selected for {family}")`
  from `view.family_symbol`). Every checked `Match`/`ComputationalMatch` needs
  such a default; deleting, weakening, or family-special-casing it would convert a
  loud runtime mismatch into SILENT acceptance. Do not touch it.
- The first causally wrong authority is the runtime lowering branch that SELECTED
  that default for this admitted program.
- Identity `40` CANNOT locate the occurrence: `Planner::intern_trap`
  (`planning/static_transition/joins_traps.rs:567-588`) interns by `RuntimeTrap`
  value equality, so all eliminations with the same code + family message share
  one identity. `family = ITree` and `identity = 40` name the KIND, not the
  occurrence (the corpus already records two eliminations of one family agreeing
  on every trap-catalog field).

## Prohibited authorities (Architect — no repair from current evidence)

Do NOT: change the erasure default; key on the `ITree` spelling; infer an
occurrence from identity `40`; treat the absence of `FsReadAt` as the bug
(dispatch skip is the EXPECTED narrowing behavior — the failure is later, when the
synthesized `Err InvalidOffset` should continue through the checked path). No
error-text, first-failure, numeric-origin, body-shape, or family-name authority.

## Fixed inputs (Architect AC-5 ruling, WIP `7094c29cd` as evidence)

- WIP `7094c29cd` (tree `ecd97116`, parent `14deff3c`, `calls.rs` `+10/-2`) —
  evidence only, not a candidate. The exact ignored
  `fs_read_at_malformed_offset_narrows_to_invalid_offset` row reproduces
  `RuntimeTrap(10495)` (`= (40 << 8) | 0xff`), stderr `unknown terminal sentinel`,
  effect prefix `FsOpen -> BufferAllocate -> ResourceRelease(FsHandle) ->
  ResourceRelease(Buffer)`, no `FsReadAt`, status 101 (Architect rerun log
  `4dc5028a...`; catalog mapping log `92cdb1a0...`).
- Trigger rows: the read-offset AND write-offset positioned full programs
  (`rt_parity_native::fs_read_at_malformed_offset_narrows_to_invalid_offset`,
  `fs_write_at_malformed_offset_narrows_to_invalid_offset`).
- `erasure.rs:2740-2744` and `joins_traps.rs:567-588` per the layer section above.

## Deliverables (occurrence localization FIRST; repair only after)

- D0 (DONE, PARTIAL — Architect object ruling evt_ts40fq959fvx) — evidence
  `49a7aff49` localizes the runtime-taken outer carried `ComputationalMatch` to
  read `301` / write `314` (owner `main`, source path `[0,1,1,0,0,2,1,0,0]`,
  checked frame 1, invocation 0) with occurrence-specific influence isolation
  (forcing only 301/314 changes the terminal; neighbors do not). No repair site
  selected. The `ResourceBodyResult` default observed under the origin-force probe
  is NOT yet a second natural defect (probe aftermath — see D0b).
- D0b (NEXT — predecessor-route and recursor-layer disposition; kept in THIS node,
  not a family-specific successor) — for read `301` and write `314`, bind ALL of
  the following from graph/claim provenance, NONE inferred from occurrence, family,
  message, or numeric origin:
  1. The exact `ComputationalRecursorLayer` that supplied the frame: `role`
     (`SelectsOccurrence` vs `ExitsScope`), `semantic_pending`, checked frame +
     invocation identity/source/depth, producer origin, and — for an exit layer —
     scope origin plus parent scope.
  2. The exact predecessor `RoutedAnswer`: its `route`, independent
     `EliminatorRole`, and the producer that minted those fields. Record the frame
     field and the result of `RoutedAnswer::raise` SEPARATELY.
  3. The actual runtime carrier at the taken outer default: `BoundaryClass`, exact
     typed `ConstructorIdentity` OR a fail-closed "not in the artifact's closed
     constructor authority", and field count. A compile-time `Value`/SSA handle is
     NOT this measurement. A test-only observation may transport the planner/graph-
     derived origin and typed identity but must add no production API, second
     identity catalog, error-text authority, or family-name dispatch.
  4. The receiving ITree case identities and the predecessor's planner-authored
     answer interface. Classify the mismatch BEFORE choosing a repair: (a) ordinary
     ITree constructor with wrong identity transport; (b) checked answer whose exact
     producer route was dropped; or (c) an answer already past one computational
     frame presented to the wrong selection/unwind layer.
  5. One positive control using the same producer/consumer mechanism where the
     planner-authorized predecessor takes its ordinary/checked-return route
     successfully. The natural mutation must act at the REAL route/role producer;
     `KEN_RT_ITREE_D0_FORCE_ORIGIN` is an isolation probe ONLY and cannot discharge
     the route-authority mutation.
  Return another object read after D0b. ONLY if it proves the origin bypass was
  semantically authorized AND a natural full-program path still reaches a localized
  `ResourceBodyResult` default does that occurrence become the next object.
- D1 — only after D0b identifies the occurrence AND the first differing graph/claim
  authority, repair the graph/claim route so the synthesized `Err InvalidOffset`
  continues through the checked ITree/result path. The erasure fail-closed default
  remains intact. NO D1 until D0b binds that authority.

## Acceptance criteria

- AC-1 — the runtime-taken default occurrence is identified by graph-derived
  `StaticOriginId` / owning declaration, and the complete candidate population is
  counted, BEFORE any repair site is selected.
- AC-2 — a positive neighboring ITree case is proven to take its
  ordinary/checked-return case, not the default (the instrument discriminates).
- AC-3 — a natural route/identity mutation makes the exact full-program occurrence
  take the default while the instrument still identifies that occurrence (the
  localization is occurrence-specific, not kind/family-specific).
- AC-4 — the repair is at the graph/claim route; the erasure fail-closed default
  (`erasure.rs:2740-2744`) is byte-unchanged; no error-text / first-failure /
  numeric-origin / body-shape / family-name authority is introduced.
- AC-5 — SUCCESS is the original exact `InvalidOffset` SemanticErrorV1 observation
  with the same no-`FsReadAt` prefix on BOTH the read-offset and write-offset full
  programs — not merely the absence of the trap.
- AC-D0b-1 — the supplying `ComputationalRecursorLayer` for read `301` and write
  `314` is bound from provenance (role `SelectsOccurrence`/`ExitsScope`,
  `semantic_pending`, checked frame + invocation identity/source/depth, producer
  origin; exit-layer scope + parent scope), NOT inferred from
  occurrence/family/message/numeric origin.
- AC-D0b-2 — the predecessor `RoutedAnswer` is bound (route, independent
  `EliminatorRole`, minting producer), with the frame field and the
  `RoutedAnswer::raise` result recorded SEPARATELY.
- AC-D0b-3 — the actual runtime carrier at the taken outer default is measured as
  `BoundaryClass` + typed `ConstructorIdentity` (or fail-closed "not in the closed
  constructor authority") + field count — NOT a compile-time `Value`/SSA handle;
  the test-only observation adds no production API, second catalog, error-text, or
  family-name dispatch.
- AC-D0b-4 — the receiving ITree case identities and the predecessor's
  planner-authored answer interface are recorded, and the mismatch is classified as
  exactly one of (a)/(b)/(c) BEFORE any repair site is named.
- AC-D0b-5 — a positive control at the real route/role producer shows the
  planner-authorized predecessor taking its ordinary/checked-return route;
  `KEN_RT_ITREE_D0_FORCE_ORIGIN` does NOT stand in for this route-authority
  mutation.
- AC-D0b-SCOPE — `erasure.rs:2740-2744` byte-identical; no third
  `RoutedAnswer::checked` caller; no reclassification of checked frames; no keying
  on `ITree`/`ResourceBodyResult`; trap identities 36/37/40/43 are NOT occurrence
  authority; the forced-route chain is NOT continued by bypassing
  `ResourceBodyResult` cases one at a time; `KEN_RT_ITREE_D0_FORCE_ORIGIN` remains
  an isolation probe.
- AC-NO-REGRESSION — whole-suite green in CI; local targeted `-p ken-runtime` /
  `-p ken-cli` / `-p ken-verify` only, never `--workspace`.

## Reviewers

Architect (the occurrence is localized by graph-derived identity, not by trap
value / family spelling / numeric origin; the erasure default is untouched; the
repair is at the selection authority and greens the exact InvalidOffset
observation) + runtime-qa (the discrimination controls AC-2/AC-3 red as specified;
fail-closed default intact). An object read may HARD-STOP into an Architect ruling
if D0 exposes a further distinct authority.

## Capability tier

T1 — occurrence localization against a value-deduplicated identity, and a
graph/claim-route repair reviewed on the provenance argument (which occurrence,
which selection authority), not a differential diff. Size M.

## Sequencing

Lane-1 (runtime, priority). Sequenced AFTER
[[RT-UNIT-FAILURE-STATUS-PROVENANCE]] lands (honest reporting first) and must not
co-run with it. This node greens the SemanticErrorV1 witness. After it lands,
[[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]] (ReadSome/Wrote) and the final
four-value closure fold follow per the Steward sequence. PX8 stays blocked until
the whole native carried-value program lands. Release queued behind
RT-UNIT-FAILURE-STATUS-PROVENANCE; the Architect reviews the WP at release.
