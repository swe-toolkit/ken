---
id: RT-UNIT-FAILURE-STATUS-PROVENANCE
title: "Generated-unit failure-status provenance on the InvalidOffset witness path — preserve the root generated-unit failure's planned trap identity (calls.rs:2075-2090 TrapWord->-4) and fold the governed -3 producer through ONE signed-root-token mechanism decoded against the existing planner trap catalog at the linked reporting boundary, so the process reporter classifies by origin/kind instead of a bare scalar. Narrowed operative claim (NOT global closure): the other same-predicate producers are enumerated but out of this WP. Lands with SemanticErrorV1 still red, honestly reported as the ITree default — the InvalidOffset green belongs to the RT-ITREE-DEFAULT-SELECTION-PROVENANCE successor."
status: merged
owner: runtime
size: M
gate: none
depends_on: [RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE]
blocks: []
github: null
origin: "Steward, 2026-08-25. Recut from the Architect umbrella arm(b) ruling (evt_7jpt4hm2nm6hh), then SCOPE-RECONCILED after the AC-5 hard stop fired: Architect ruling evt_3rq4xafrf7cqf (thr_6gmh4p1m0gch4) on runtime WIP 7094c29cd. AC-5 is valid — preserving trap identity exposed a distinct ITree-default producer (now the successor [[RT-ITREE-DEFAULT-SELECTION-PROVENANCE]]). The same ruling corrected this frame twice: (1) a calls-only production boundary CANNOT satisfy 'report with origin/kind' — the frame must permit one subsuming typed envelope at the linked reporting boundary; (2) the structural census is wider than one syntactic site, so the operative closure claim must be narrowed explicitly. Steward owns the frame amendment + successor routing. Prior draft origin: Architect hard-stop #3 evt_1vhmndq7fscd1. Steward framing call per COORDINATION section 2."
---

> # MERGED 2026-08-25 at squash `2a5b3cd0e` — honest witness-path provenance landed
>
> Candidate `7b5547fe6` (tree `f14e110f`, base `027f6bf26`, 3 commits, 19 paths
> `+519/-80`) merged onto origin/main `2a5b3cd0e` through the GitHub PR publisher
> (whole-suite CI gate passed). Exact-SHA APPROVES: Architect `evt_1f9f8cmsk4ej2`
> (binds this SHA only, void on respin), runtime-qa `evt_461scgdfd5d0d`; Decision
> `dec_4g8c1bcdpc6tn` resolved. BLOB-AUDIT clean: all 19 approved crate paths landed
> byte-identical and nothing was touched outside `crates/`. The linked witness now
> reports planned identity `40` / `PatternMatchFailure`
> (`no runtime match case selected for decl:...::ITree`) instead of `unknown
> terminal sentinel`; `lowering/core.rs` + `erasure.rs` blob-identical to base, zero
> `#[ignore]` delta, no trusted-base change. SemanticErrorV1 lands honestly RED as
> the ITree default — greening the exact `InvalidOffset` is the successor
> [[RT-ITREE-DEFAULT-SELECTION-PROVENANCE]]. Umbrella [[RT-NATIVE-CARRIED-VALUE]] and
> PX8 stay blocked (three native witnesses remain).
>
> # SCOPE RECONCILE 2026-08-25 — AC-5 fired; two frame corrections (Architect)
>
> The AC-5 hard-stop arm fired as designed. Runtime WIP `7094c29cd` (tree
> `ecd97116`, parent `14deff3c`, `calls.rs` only `+10/-2`) preserved the root
> generated-unit failure's planned identity (`40`) instead of collapsing to `-4`,
> and that exposed a DISTINCT downstream producer — an ITree-elimination default
> selection — which the Architect ruled a real, separate object
> ([[RT-ITREE-DEFAULT-SELECTION-PROVENANCE]], framed alongside). `7094c29cd` is
> evidence, NOT a candidate; the `-3` fold was correctly left untouched.
>
> The same ruling corrected THIS frame twice before Runtime resumes:
> - Production surface is NOT `calls.rs`-only. A calls-only boundary cannot make
>   the LINKED reporter classify origin/kind (see "Production surface" below). The
>   frame permits ONE subsuming typed envelope at the linked reporting boundary.
> - The census is wider than one site. Other same-predicate producers remain
>   (`joins.rs::emit_current_trap`, `aggregates.rs`, two source-machine paths).
>   The operative closure claim is NARROWED to the witness path; the rest are
>   enumerated, not claimed closed.
>
> Runtime resumes from current `origin/main` `a61a19a4`, replaying ONLY the
> `7094c29cd` `calls.rs` delta as evidence into a fresh candidate SHA, to complete
> D1 only. It MAY
> land with SemanticErrorV1 still red — now honestly reported as the ITree
> default, not `unknown terminal sentinel`. Greening InvalidOffset belongs to the
> successor. `-3` and `-4` remain ONE status-provenance predicate — this is not a
> third node.

## Objective

On the InvalidOffset witness path, a generated-unit failure that carries a
planned trap identity is collapsed into a globally-interpreted scalar and then
cannot be classified by the linked process reporter. Carry the identity through
ONE signed-root-token mechanism, decoded against the EXISTING planner trap
catalog at the linked reporting boundary, so the reporter classifies by
origin/kind. Fold the governed `-3` producer under the same contract. Success is
HONEST reporting, not a green witness (see below).

## Production surface — the linked reporting boundary (Architect correction 1)

The WIP proved a number preserved in `calls.rs` does NOT satisfy AC-1's
"report with origin/kind", because the linked process discards it:

- `CompiledModule::run` decodes ONLY positive root tokens against its in-memory
  trap catalog.
- The linked process returns the token NEGATIVE; the generated C reporter
  classifies every negative other than `-1..-4` as `unknown terminal sentinel`.
- `ken_host_invocation_v1_finish` writes the scalar with `terminal_error: None`;
  `run_bound_process_effect_observation` can therefore recover only
  `TerminalErrorV1::RuntimeTrap(<magnitude>)`.
- `BoundProcessExecutableArtifact` carries NO trap catalog.

So the frame PERMITS one subsuming typed envelope at the linked reporting
boundary: decode the signed token's MAGNITUDE against the SAME planner trap
catalog, bound to the artifact/plan. NO second catalog, NO new sentinel;
unknown / zero / out-of-range identities still fail closed. This is the one place
the production surface extends beyond `calls.rs`.

## Structural census (Architect correction 2 — operative claim NARROWED)

The identity-collapse predicate has MORE producers than one syntactic site.
Enumerated (measured by the Architect):

- `calls.rs:2075-2090` — root `TrapWord` collapses to `-4` under
  `TrapExitAuthority::Root { process_sentinel: true }`, `identity_preserved:false`.
  ON the witness path. IN scope.
- `emit_carrier_dynamic_constructor`'s residual `MALFORMED_DYNAMIC_CONSTRUCTOR_STATUS
  == -3`, forwarded UNCHANGED by `call_declared_unit_target` — the governed
  in-scope `-3` member (the specific graph/code route, not the scalar spelling).
  ON the witness path. IN scope (folded under the same contract).
- `joins.rs::emit_current_trap` — also obtains an exact planned identity and
  collapses it to `-4` under `Root { process_sentinel: true }`. OUT of this WP.
- bare `MALFORMED_DYNAMIC_CONSTRUCTOR_STATUS` also emitted from `aggregates.rs`
  and two source-machine paths. OUT of this WP.

OPERATIVE CLAIM (narrowed): this WP delivers the signed-root-token envelope + the
witness-path `-4` and governed `-3` producers. It does NOT claim global closure
over the identity-collapse predicate while `joins.rs::emit_current_trap` and the
`aggregates.rs` / source-machine producers remain. Those are recorded here as
known same-predicate members for a later census/closure pass, not silently
dropped. Do NOT assert "structural closure" in the candidate.

## Fixed inputs (Architect, grounded at merged `d9bc68db0` and WIP `7094c29cd`)

- The `-4` root collapse and its `-4`->`-44` proof (log
  `a294ee06...`); production base `calls.rs` blob
  `d4e056b330f4bf2d78010be613d6511c42ab8774`.
- WIP `7094c29cd` (evidence): the root `TrapWord` now uses the existing
  `ROOT_TRAP_TOKEN` layout `(40 << 8) | 0xff = 10495`, negated only for the
  process ABI, so the magnitude preserves identity `40` exactly. The exact
  ignored `fs_read_at_malformed_offset_narrows_to_invalid_offset` row changed
  from `RuntimeTrap(4)` / `explicit entry trap` to `RuntimeTrap(10495)` /
  `unknown terminal sentinel`, `FsOpen -> BufferAllocate -> ResourceRelease x2`,
  no `FsReadAt` (Architect rerun log `4dc5028a...`). `core.rs` byte-identical.
- The linked-boundary decode chain named under "Production surface".
- The governed in-scope `-3` producer (the ONLY `-3` member in this WP's claim):
  `emit_carrier_dynamic_constructor`'s residual `MALFORMED_DYNAMIC_CONSTRUCTOR_STATUS
  == -3`, forwarded UNCHANGED by `call_declared_unit_target` to the process reporter
  (classified "malformed ExitCode::Failure payload" — a sentinel alias; the path
  never produced an ExitCode failure). The other enumerated `-3` producers
  (`aggregates.rs`, two source-machine paths) are OUT of scope.

## Deliverable

- D1 — one signed-root-token mechanism: preserve the planned trap identity from
  the root generated-unit failure (`calls.rs:2075-2090`) and decode its magnitude
  against the existing planner trap catalog at the linked reporting boundary (the
  subsuming typed envelope), so the reporter classifies by origin/kind. Fold the
  governed in-scope `-3` producer — `emit_carrier_dynamic_constructor`'s residual
  forwarded unchanged by `call_declared_unit_target` — under the same provenance
  contract. Use EXISTING typed authority / catalog only — never a new sentinel or a
  second catalog. Fail-closed
  residuals (unknown/zero/out-of-range) preserved.

## Success criterion (Architect — do NOT overpromise)

Success is HONEST reporting of the witness-path failure by origin/kind. This WP
MAY land with the SemanticErrorV1 rows still RED — now honestly reported as the
ITree default (planned identity `40`), not `unknown terminal sentinel`. Greening
the exact `InvalidOffset` observation is the job of
[[RT-ITREE-DEFAULT-SELECTION-PROVENANCE]], NOT this node. Do not un-ignore the
witness rows here.

## Acceptance criteria

- AC-1 — the witness-path failure is reported with origin/kind through the linked
  reporting-boundary envelope (magnitude decoded against the existing planner
  catalog), not as a bare `unknown terminal sentinel` scalar.
- AC-2 (the `-4` producer) — the root `TrapWord` at `calls.rs:2075-2090` no longer
  collapses to `identity_preserved:false` / `-4`; the planned identity survives to
  the reporting boundary.
- AC-3 (the governed `-3` producer) — `emit_carrier_dynamic_constructor`'s
  residual `MALFORMED_DYNAMIC_CONSTRUCTOR_STATUS == -3`, forwarded unchanged by
  `call_declared_unit_target`, carries origin/kind under the same envelope, not a
  bare scalar (folded, no separate sentinel). AC-3 binds THIS named route, not the
  scalar spelling.
- AC-4 — fail-closed backstop preserved: genuinely unclassifiable / zero /
  out-of-range identities still refuse; NO new sentinel, NO second catalog.
- AC-5 (census honesty) — the candidate does NOT claim global structural closure;
  it states the narrowed operative claim and leaves `joins.rs::emit_current_trap`
  + the `aggregates.rs`/source-machine producers explicitly enumerated as out of
  scope.
- AC-6 (mutation controls) — reintroducing the `identity_preserved:false` / `-4`
  collapse REDS AC-2; re-forwarding the bare `-3` scalar REDS AC-3; a mutation
  that admits an out-of-range identity at the boundary REDS AC-4.
- AC-7 — the production surface is `calls.rs` plus the one linked-reporting-boundary
  envelope; `core.rs` stays byte-identical; do NOT un-ignore the witness rows;
  zero `trusted_base()` delta.
- AC-NO-REGRESSION — whole-suite green in CI; local targeted `-p ken-runtime` /
  `-p ken-cli` / `-p ken-verify` only, never `--workspace`.

## Reviewers

Architect (identity preserved to the linked reporting boundary via the existing
catalog/envelope, not a new sentinel or second catalog; `-4`/`-3` one mechanism;
the operative claim is honestly narrowed; SemanticErrorV1 may land red as the
ITree default) + runtime-qa (AC-6 mutation controls red; fail-closed backstop
intact; census-honesty AC-5 stated). No Decision fork — the provenance contract
determines the answer.

## Capability tier

T1 — the review turns on the provenance / identity-preservation argument across
the root authority AND the linked reporting boundary, and on the honesty of the
narrowed closure claim. Size M.

## Sequencing

Lane-1 (runtime, priority). RESUME to complete D1 only. Base construction: cut /
rebase the branch from current `origin/main` `a61a19a4`, then replay the
`7094c29cd` `calls.rs` delta as evidence, producing a FRESH candidate SHA — do NOT
remain based on pre-reconcile `14deff3c` or `7242e5c1`. (The runtime ring already
replayed this delta at WIP `61ab731a`; re-anchor it onto `a61a19a4`, a doc-only
advance over `7242e5c1` that leaves the `calls.rs` replay unaffected.) The successor
[[RT-ITREE-DEFAULT-SELECTION-PROVENANCE]] (the exposed ITree-default producer that
greens InvalidOffset) is sequenced AFTER and must not co-run.
[[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]] (ReadSome/Wrote, `calls.rs:1631-1640`),
all ignores, and the final four-value closure fold stay held to this resulting
sequence. PX8 remains blocked.
