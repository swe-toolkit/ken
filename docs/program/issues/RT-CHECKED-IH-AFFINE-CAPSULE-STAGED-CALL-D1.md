---
id: RT-CHECKED-IH-AFFINE-CAPSULE-STAGED-CALL-D1
title: "Scratch-only staged-call D1 (no production candidate, no QA, no Decision, no merge) for the selected affine checked-IH application capsule: implement, AS SCRATCH, the compile-time per-application-path generated-context specialization the FEASIBILITY D0 proved is required, then emit the nested body-662 call. The stop the D0 hit is structural — contexts intern/look up ONLY by (enclosing specialization, worker body), so the seven body-941 Host-Vis applications share Context0/func50 (every first parameter v10; origin144 and sibling origin153 indistinguishable; sixteen physical record-608 arrivals), and a move-only value cannot make seven caller identities or sixteen emission seams affine. Build two typed compiler-only plan objects — (1) CheckedIhApplicationCapsulePlan binding the exact source environment/worker relation, nested Host-Vis marker/application/callee/argument, destination context, and issued downstream continuation identity; (2) a closed generated-context KEY with ordinary-shared and checked-IH-capsule-path arms (the capsule arm holds an opaque capsule identity, NOT an Option/global-slot/body-key-supplement/runtime-selector) — construct them from already-issued transports, occurrence ancestry, and continuation identities (NOT constructor-name substring scans, NOT a search for match451, NOT hardcoded origin/body numbers). Prove application146 selects its capsule-specific body-941 context while every sibling selects only its own authorized context; route every exact carried-environment arrival to ONE compiler-local application block/join carrying the local environment word and HostResult as block parameters; project fields 0..7 ONCE at the join (not in sixteen predecessors); resolve the local raw target; only then mint a non-Clone/non-Copy CheckedIhApplicationAuthority and emit one body-worker call per capsule member (explicit HostResult, eight captures, no suffix, status+Trap before Result, non-trap Result becomes CheckedIhApplicationResult into the issued downstream continuation/natural match). Exercise BOTH the bound read AND write parity witnesses (never special-case read coordinates); preserve outer body-941 execution and 474/473 as an environment-only zero-call control. Land exactly one of four outcomes; outcomes 2-4 stop before a worker call. Restore byte-clean and report."
status: closed
owner: runtime
size: L
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_4eg2hgk35j4qf (thr_5cc5rgdvqv0fs, 2026-08-31), ACCEPTING the runtime-implementer's RT-CHECKED-IH-AFFINE-APPLICATION-CAPSULE-FEASIBILITY-D0 outcome 2 (report evt_1zpgs2h0kd74q, report/diff SHA-256 b5978f4abeac10d24def62b66464102c7640935eb8ff5c542e17673670a2fd0a / f21245de914d62bd099f8c4d99784fea754a1e31bec4482a889e98e07480a0ac): the capsule family is FEASIBLE — the planner derives one closed read-witness relation (spec1 / record608 / seat671 / body662 / RawWorker -> operation674 / Host-Vis 147/146 / callee145 / argument144 -> destination spec2 / body941 / Context0 -> issued downstream control spec3 / body452 / match451), operation ancestry discriminating (frame 12: 25 Host-Vis markers, exactly one under FSOp::ctor_488; ctor_490 selects application126) — but the present stage cannot consume it lawfully. Contexts intern/look up ONLY by (enclosing specialization, worker body): seven body-941 applications share Context0/func50 (every first parameter v10; origin144 vs sibling origin153 indistinguishable; sixteen physical record-608 arrivals). No call, no post-call controls — correct. The chain (evt_7e6jprw80srj8 force-D1 NO, evt_5tm8gbxs7584g carried-worker outcome 4, evt_6rvrzpg80c02j Host-Vis outcome 3, evt_1zpgs2h0kd74q capsule-feasibility outcome 2) shares one predicate: body 662's callable authority and its future HostResult never coexist in one compiler-local object; the staging fix mints per-path contexts so they can. Base origin/main c698779b682a56bbdc4a5ab8fd6f85f8a9be45d7, tree 05d5940b52401ab1b3dee6aada7dfaf199db59a6; all seven reviewed product blobs (core eea98dc6, source c39f82e7, planner aggregates e7bc3628, lowering aggregates eaf1019b, calls fa010fed, units ccc6ddb2, parity test 6b2f14a7) Steward-verified identical to accepted base 0be25235b. @steward owns close/reframe/release; runtime parked until this named kick. Steward-recut per COORDINATION section 2."
---

> # CLOSED — D1 COMPLETE, OUTCOME 1 ACCEPTED NARROWLY as static staged-call
> # feasibility; HELD as production authority. Architect ruling `evt_4maaydhem9esy`
> # (thr_2mxxq9zx6ezrn, 2026-08-31).
> #
> # Measurement node — never `merged`; report `evt_6qkn80ag2tq89` (report / diff /
> # manifest SHA-256 `da5925a517a6...` / `ff0a28008ea9c...` / `543887f46514...`)
> # restored byte-clean at pickup `ad18411c7`, tree `00fcad9b8`. The Architect
> # reproduced all three artifacts and all seven product blobs and ACCEPTED the
> # static representation as FEASIBLE across BOTH read and write: generic derivation
> # (no hardcoded fixture coordinates, no `match451` search), compile-time context
> # specialization, target locality, one emitted raw call per capsule Function (read
> # CLIF one `call fn35` -> `u0:43`; write one `call fn35` -> `u0:45`), and a
> # statically well-typed result edge. Outer routes stay `GeneratedContext`; `474/473`
> # unchanged. This establishes compile-time context specialization, target locality,
> # operand assembly, one emitted instruction per capsule Function, and a well-typed
> # result edge.
> #
> # HELD as production authority — three gaps the D1 did NOT close:
> #  (1) Neither runtime execution REACHES the capsule path. The `RT_AFFINE_D1
> #      NESTED_CALL` lines are printed while CLIF is generated — they are not dynamic
> #      call observations. Both binaries execute an earlier ordinary sibling, then
> #      produce the unchanged four-effect prefix and take natural `ResourceBodyResult`
> #      trap 36/37 BEFORE the capsule call runs. "One response instance reaches the
> #      join once" is unproved.
> #  (2) The predecessor mutations do NOT mutate predecessor population:
> #      `omit-predecessor` passes one block argument, `duplicate-predecessor` three —
> #      both fail generic Cranelift block-arity validation. They prove the fresh
> #      block has two parameters, NOT that every carried-environment arrival reaches
> #      one join with none omitted or duplicated.
> #  (3) `control_result_ordinary_index` is derived, stored, and exposed on the plan,
> #      but no production path reads it — the result follows the current
> #      detached-edge control because `control_identity` matches. It is redundant
> #      (remove) or authority (consume with an independent mutation).
> #
> # The symptom moved from executor identity to RUNTIME STAGING/REACHABILITY: the
> # correct future capsule application IS emitted, but an earlier ordinary context
> # traps before the future application can run.
> #
> # Next: scratch-only successor
> # **`RT-CHECKED-IH-CAPSULE-RUNTIME-REACHABILITY-D0`** on base `ad18411c7` — reuse
> # the exact D1 scratch mechanism (diff `ff0a28008ea9c...`) as the scaffold, add NO
> # production candidate, and install one discriminating runtime net over the
> # six-stage read/write chain to find the first unexecuted edge. Everything below
> # this banner is retained as chronology.
>
> # READY — SCRATCH-ONLY STAGED-CALL D1. Released to the runtime ring (lane 1) on
> # `origin/main` `c698779b6`. Runtime is parked; this IS the release.
> #
> # This is a MEASUREMENT node. It lands NO production candidate, opens NO PR, routes
> # NO QA, and needs NO Decision or merge. It builds a production-SHAPED scratch
> # prototype, returns a report plus a scratch diff and digests, and restores the
> # branch byte-clean at the end. The Architect reviews the D1 report and rules on
> # the production design. Like every prior D0/D1 in this chain, it is never
> # `merged`.
> #
> # **The representation is SELECTED and the stop is DIAGNOSED — this D1 is the
> # staged-call feasibility, not another seat search.** The FEASIBILITY D0 proved
> # the capsule family feasible and the planner relation derivable, and pinned the
> # exact structural stop: contexts intern only by `(enclosing specialization, worker
> # body)`, so the seven body-941 Host-Vis applications share one context/func50
> # where `origin144` and sibling `origin153` are indistinguishable (`v10`), and
> # func50 has sixteen record-608 arrivals. A move-only value cannot make seven
> # caller identities or sixteen emission seams affine. **This D1 implements the
> # compile-time per-path context specialization that separates them, then emits the
> # nested call — as SCRATCH.**

## What this D1 does

Implement, as a scratch prototype restored byte-clean, the compile-time
per-application-path generated-context specialization the D0 proved is required,
and — only once the staging closes — emit one nested body-662 worker call per
capsule member, paired Trap-before-Result to the issued downstream continuation.
Exercise BOTH the bound read AND the write parity witness. Land in exactly one of
four outcomes; outcomes 2-4 stop before any worker call.

## Exact base and coordinates (Architect `evt_4eg2hgk35j4qf`)

Base `origin/main` `c698779b682a56bbdc4a5ab8fd6f85f8a9be45d7`, tree
`05d5940b52401ab1b3dee6aada7dfaf199db59a6`. All seven reviewed product blobs are
identical to accepted base `0be25235b` (core `eea98dc6`, source `c39f82e7`,
planner aggregates `e7bc3628`, lowering aggregates `eaf1019b`, calls `fa010fed`,
units `ccc6ddb2`, parity test `6b2f14a7`). **If main advances before pickup,
recheck those seven blobs before edits.** The derived read-witness relation
(accepted at the D0): `spec1 / record608 / seat671 / body662 / RawWorker ->
operation674 / Host-Vis 147/146 / callee145 / argument144 -> destination spec2 /
body941 / Context0 -> issued downstream control spec3 / body452 / match451`.

## The selected staging design (Architect ruling — implement this, do not redesign)

Production needs two typed, compiler-only plan objects:

1. **`CheckedIhApplicationCapsulePlan`** — binds the exact source
   environment/worker relation, the nested Host-Vis marker/application/callee/
   argument, the destination context, and the issued downstream continuation
   identity.
2. **A closed generated-context KEY** with an ordinary-shared arm and a
   checked-IH-capsule-path arm. The capsule arm contains an OPAQUE capsule
   identity — NOT an `Option`, a global slot, a body-key supplement, or a runtime
   selector.

Construct these typed relations from **already-issued transports, occurrence
ancestry, and continuation identities**. Do NOT select control by
constructor-name substring scans, a search for `match451`, or hardcoded
origin/body numbers. `match451` is a witness reached through the selected
continuation, NOT target authority. The scratch string rows and diagnostic scans
from the D0 are evidence only.

**Outer route unchanged.** The outer body-941 route stays `GeneratedContext`;
`application146` is NOT retargeted to body 662. The exact pending Host-Vis plan
supplies the capsule-specific `ContinuationContextId`, resolved in the caller's
Function-local context table. Body-only `worker_calls[941]` may remain the
ordinary shared route but cannot answer a capsule path. If the call API needs a
composite recipe, it carries the existing outer route PLUS the planner-issued
context ID; do NOT derive a context from `static_origin` or weaken the
carried-only route law.

**Per-path Function, single join.** The per-path Function preserves `origin144`
as `Parameter(0)` by construction. Every exact record-608 arrival must branch to
ONE compiler-local application block carrying the local environment word and the
HostResult as block parameters. Project fields 0..7 ONCE at the join, not in
sixteen predecessors.

**Authority and call.** Only after the context key, caller target, predecessor
population, and join close may lowering mint a non-`Clone`, non-`Copy`
`CheckedIhApplicationAuthority`. It resolves the local raw target and emits one
body-worker instruction per capsule member: explicit HostResult, eight captures,
no suffix. Status and Trap precede Result. Only the non-trap value becomes
`CheckedIhApplicationResult` and enters the issued downstream continuation /
natural match. `CheckedIhCapturedEnvironment` stays distinct; `474/473` stays
no-call materialization. Affinity is an emission property (one capsule member,
context Function, join, call instruction, result edge); CFG/source closure
separately proves one response instance reaches the join once and cannot enter a
sibling Function.

## Required D1 work (Architect ruling — each mandatory)

1. Implement a production-shaped typed capsule population and the closed
   ordinary/capsule context key.
2. Enumerate applicable members, derive relations for BOTH the bound read AND
   write parity witnesses, and NEVER special-case read coordinates.
3. Prove `application146` selects its capsule-specific body-941 context while every
   sibling selects only its own authorized context.
4. Route every exact carried-environment arrival to ONE join, with NO dropped or
   duplicated predecessor.
5. Project each capture ONCE at the join, resolve the local raw target, and form
   one move-only authority.
6. Only then emit one nested raw-worker call per member and pair Trap-before-Result
   to the issued downstream control.
7. Preserve outer body-941 execution and `474/473` as an environment-only zero-call
   control.
8. Run the exact ignored read and write witnesses and accept ONLY their natural
   typed outcomes.

## Post-call mutation controls (only after a real call exists)

Independently mutate, each reached, refusing for its named reason, then restoring
an unchanged positive: capsule suppression/duplication;
application-to-context sibling substitution; predecessor omission/duplication; one
capture origin/order; route/table choice; HostResult suppression/replacement; and
Result delivery. Do NOT manufacture controls for an impossible call.

## Exhaustive outcome routing (land in exactly one)

- **(1) Staged call feasible** — staged contexts, one join, and a natural
  call/result pair are feasible. Return the exact staged design, the join, and the
  natural (successful or failing) typed result for both witnesses, for the
  production-design ruling.
- **(2) Application-specific context selection violates the carried outer-route
  law** — name the exact conflict; stop before a worker call.
- **(3) Predecessor/control closure cannot preserve the downstream continuation** —
  name where the closure fails; stop before a worker call.
- **(4) Typed capsule population absent/ambiguous for either witness** — name the
  missing/ambiguous relation; stop before a worker call.

## Closed axes remain closed (do NOT)

No runtime callable/continuation reification, global optional capsule, inferred
callee, sibling HostResult substitution, environment-as-result, synthetic
`ResourceBodyOk`, match weakening, Trap change, or caller-success revival. Do not
retarget `application146` to body 662; do not derive the context from
`static_origin`; do not weaken the carried-only route law; do not select control
by constructor-name substring, a `match451` search, or hardcoded numbers. Preserve
`ResourceBodyResult`, natural match semantics, typed trap 36, and all prior
C/I/E/S and source/member/projection/direction/delivery controls.

## Reviewers, sequencing, contention

- **Reviewer:** the Architect ALONE reviews the D1 report, digests, and scratch
  diff, and rules on the production design per the outcome routing. No Runtime QA
  (scratch-only, no production candidate), no Conformance Validator, no Decision, no
  publisher CI. Never `merged`.
- **Sequencing:** runtime ring (lane 1), single continuous turn to a complete report
  or a genuine blocker. Size L, tier T1 — this is a production-SHAPED scratch build
  (typed capsule population + closed context key + per-path Function + join +
  authority + call, across two witnesses and the full mutation net), materially
  heavier than the prior measurement D0s. The seat restored to ~42% ctx after the
  D0 auto-compaction, so it starts with headroom; the build is read-and-write heavy
  and will climb — hold one continuous turn, do NOT `compact_self` mid-turn (a
  self-compact silently drops the assignment), and report a genuine blocker instead
  if headroom runs out. Restore byte-clean at the end regardless of outcome.
- **Contention:** reads and scratch-prototypes
  `crates/ken-runtime/src/cranelift_backend/lowering/{core,source,calls,units}.rs`,
  the generated-unit planner and its generated-context interning/keying, the
  static-worker `raw_worker_calls` target-table machinery, and the
  `rt_parity_native.rs` evidence wrappers; produces no landed change, so no
  crate/catalog contention with the concurrent lanes. Targeted builds ONLY via
  `scripts/ken-cargo` scoped to `ken-runtime`, never `--workspace`.
