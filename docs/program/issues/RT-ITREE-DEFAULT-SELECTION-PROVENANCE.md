---
id: RT-ITREE-DEFAULT-SELECTION-PROVENANCE
title: "AC-5 successor exposed by identity-preserving trap provenance — an admitted checked program's runtime lowering SELECTS the generated ITree-elimination fail-closed default (planned identity 40, PatternMatchFailure 'no runtime match case selected for ...::ITree') where the synthesized Err InvalidOffset should continue through the checked ITree/result path. The erasure default VALUE (erasure.rs:2740-2744) is CORRECT and fail-closed; the first causally wrong authority is the lowering branch that selected it, and identity 40 cannot locate the occurrence (Planner::intern_trap value-dedups). First deliverable is occurrence localization by graph-derived StaticOriginId, not repair."
status: draft
owner: runtime
size: M
gate: none
depends_on: [RT-UNIT-FAILURE-STATUS-PROVENANCE]
blocks: []
github: null
origin: "Steward, 2026-08-25, from the Architect AC-5 ruling (evt_3rq4xafrf7cqf, thr_6gmh4p1m0gch4) on runtime WIP 7094c29cd. Preserving the root generated-unit failure's planned trap identity (RT-UNIT-FAILURE-STATUS-PROVENANCE) exposed a DISTINCT downstream producer: the checked ITree-elimination default is selected for an admitted program. The Architect ruled AC-5 valid and directed this be framed as its own object (explicitly NOT the prohibited -3-vs-4 'third node'; it is a different producer requiring its own object read). Steward framing call per COORDINATION section 2."
---

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

- D0 — on the exact read-offset and write-offset full programs, record the
  runtime-taken default occurrence by graph-derived `StaticOriginId` / source path
  and owning declaration — NOT by trap value or family spelling. At that same
  emitted branch, record match kind, checked frame/invocation identities,
  `SourceComputationalAnswerRoute`, actual carried class/tag/field-count, and the
  planner-issued case identities. Count the COMPLETE candidate population before
  selecting a repair site. Prove a positive neighboring ITree case takes its
  ordinary/checked-return case and NOT the default; prove a natural route/identity
  mutation makes the exact full-program occurrence take the default while the
  instrument still identifies THAT occurrence. If D0 exposes yet another distinct
  authority, HARD-STOP and return the object read (do not repair).
- D1 — only after the occurrence and first differing authority are known, repair
  the graph/claim route so the synthesized `Err InvalidOffset` continues through
  the checked ITree/result path. The erasure fail-closed default remains intact.

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
