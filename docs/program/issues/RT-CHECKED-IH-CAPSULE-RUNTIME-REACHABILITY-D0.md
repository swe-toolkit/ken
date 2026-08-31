---
id: RT-CHECKED-IH-CAPSULE-RUNTIME-REACHABILITY-D0
title: "Scratch-only capsule runtime-reachability D0 (no production candidate, no QA, no Decision, no merge): the STAGED-CALL D1 proved the affine checked-IH application capsule is STATICALLY feasible across both witnesses (correct future capsule application is emitted, one raw call per capsule Function), but neither bound runtime execution REACHES it — both binaries execute an earlier ordinary sibling and take natural ResourceBodyResult trap 36/37 before the capsule call runs, so the D1 NESTED_CALL prints were CLIF-generation events, not dynamic observations. Reuse the exact D1 scratch mechanism (diff ff0a28008ea9c...) as the scaffold; add NO production candidate. For BOTH the bound read AND write parity witnesses, map source occurrence -> CLIF instruction -> machine address and install ONE discriminating runtime net (real dynamic observations, not compile-time prints) over: (1) the earliest ordinary sibling outer-context call that actually executes; (2) the capsule-specific outer Host-Vis call (read application146 / write application159); (3) capsule Function entry; (4) its application block and nested raw-worker instruction; (5) Trap-before-Result completion and capsule delivery/outer return; (6) the natural ResourceBodyResult match and typed trap. Record exact runtime hit counts, runtime HostResult/environment words, call/return order, and the FIRST unexecuted edge. Counts alone do not pair sites: identify each site from its source application and decoded callee. Discharge the two control debts the D1 could not: mutate an actual CFG predecessor edge/population (not application-block argument count), and independently mutate the PRODUCER of the downstream result coordinate; resolve control_result_ordinary_index (if redundant, remove it from the proposed production design and name the typed identity that fully determines delivery; if necessary, demonstrate its live consumer and refusal). Land in exactly one of four outcomes; for the selected outcome identify the exact planner/control relation that should own the next stage. Do NOT call the future capsule early, borrow a sibling HostResult, reinterpret the trap as suspension, or forward environment material as a result. Restore byte-clean and report."
status: ready
owner: runtime
size: L
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_4maaydhem9esy (thr_2mxxq9zx6ezrn, 2026-08-31), ACCEPTING the runtime-implementer's RT-CHECKED-IH-AFFINE-CAPSULE-STAGED-CALL-D1 outcome 1 NARROWLY as static staged-call feasibility (report evt_6qkn80ag2tq89, report/diff/manifest SHA-256 da5925a517a6ba185597eccda425dabb98761237490a0a86e430efed14498fb5 / ff0a28008ea9c6b4877abf6c9f86f492dc7eb7f9fbbb36f59c111bce17076123 / 543887f465145fc39eee0eb7e22d5f2980a220bbd150f47cdf3eb9034d572127). The Architect reproduced all three artifacts and all seven product blobs at pickup ad18411c712a0316a8b9d0248963c694bd53b253, tree 00fcad9b87591c982d19ceb2ac54b95de87d6b46, byte-clean, and accepted the generic static representation as feasible across both read and write (read: spec1/record608/body662 -> application146/HostResult144 -> capsule ContextId1 over outer body941 -> issued control spec3; write: unique enclosing-transport chain spec2/record719/body888 -> spec3/body1238 -> application159/HostResult157 in spec5/body1259 -> capsule ContextId2 -> issued control spec6; GeneratedContextKey a closed ordinary/capsule sum with opaque capsule IDs; read CLIF one call fn35->u0:43, write one call fn35->u0:45; 474/473 unchanged). HELD as production authority on three gaps: (1) neither runtime execution reaches the capsule path — the NESTED_CALL lines are CLIF-generation prints, both binaries execute an earlier ordinary sibling and trap 36/37 before the capsule call; (2) the predecessor mutations fail generic Cranelift block-arity validation (1 vs 3 args), proving block arity not predecessor population; (3) control_result_ordinary_index is derived/exposed but no production path reads it. The symptom moved from executor identity to runtime staging/reachability. The chain (evt_7e6jprw80srj8 force-D1 NO, evt_5tm8gbxs7584g carried-worker outcome 4, evt_6rvrzpg80c02j Host-Vis outcome 3, evt_1zpgs2h0kd74q capsule-feasibility outcome 2, evt_4eg2hgk35j4qf staged-call selection, evt_4maaydhem9esy staged-call outcome 1 accepted narrowly) shares one predicate: body 662's callable authority and its future HostResult never coexist in one compiler-local object; the staging fix mints per-path contexts so they can — this D0 measures whether the runtime ever REACHES the staged call. Base origin/main ad18411c712a0316a8b9d0248963c694bd53b253, tree 00fcad9b87591c982d19ceb2ac54b95de87d6b46; all seven reviewed product blobs (core eea98dc6, source c39f82e7, planner aggregates e7bc3628, lowering aggregates eaf1019b, calls fa010fed, units ccc6ddb2, parity test 6b2f14a7) Steward-verified identical to accepted base 0be25235b. @steward owns close/reframe/release; runtime parked until this named kick. Steward-recut per COORDINATION section 2."
---

> # READY — SCRATCH-ONLY RUNTIME-REACHABILITY D0. Released to the runtime ring
> # (lane 1) on `origin/main` `ad18411c7`. Runtime is parked; this IS the release.
> #
> # This is a MEASUREMENT node. It lands NO production candidate, opens NO PR, routes
> # NO QA, and needs NO Decision or merge. It reuses the D1 scratch prototype as a
> # scaffold, installs a runtime observation net, returns a report plus a scratch diff
> # and digests, and restores the branch byte-clean at the end. The Architect ALONE
> # reviews the D0 report and rules on the production design. Like every prior D0/D1
> # in this chain, it is never `merged`.
> #
> # **The representation is SELECTED and STATICALLY FEASIBLE — the symptom is now
> # RUNTIME STAGING/REACHABILITY, not executor identity.** The STAGED-CALL D1 emitted
> # the correct future capsule application (one raw call per capsule Function), but
> # its `NESTED_CALL` lines were CLIF-generation prints; both bound binaries execute
> # an earlier ordinary sibling and take natural `ResourceBodyResult` trap 36/37
> # BEFORE the capsule call runs. **This D0 installs a discriminating RUNTIME net over
> # the six-stage read/write chain and identifies the FIRST unexecuted edge**
> # — where, dynamically, the earlier ordinary context traps before the future
> # application can run.

## What this D0 does

Reuse the exact D1 scratch mechanism (diff `ff0a28008ea9c...`) as the scratch
scaffold — add NO production candidate. For BOTH the bound read AND write parity
witnesses, map `source occurrence -> CLIF instruction -> machine address` and
install ONE discriminating runtime net (real dynamic observations, not
compile-time prints) over the six-stage chain below. Record exact runtime hit
counts, runtime HostResult/environment words, call/return order, and the FIRST
unexecuted edge. Land in exactly one of four outcomes and name the exact
planner/control relation that should own the next stage. Restore byte-clean.

## Exact base and coordinates (Architect `evt_4maaydhem9esy`)

Base `origin/main` `ad18411c712a0316a8b9d0248963c694bd53b253`, tree
`00fcad9b87591c982d19ceb2ac54b95de87d6b46`. All seven reviewed product blobs are
identical to accepted base `0be25235b` (core `eea98dc6`, source `c39f82e7`,
planner aggregates `e7bc3628`, lowering aggregates `eaf1019b`, calls `fa010fed`,
units `ccc6ddb2`, parity test `6b2f14a7`). **If main advances before pickup,
recheck those seven blobs before edits.** The accepted static relations:

- **read:** `spec1 / record608 / body662 -> application146 / HostResult144 ->
  capsule ContextId1` over outer `body941 -> issued control spec3`; read CLIF one
  `call fn35 -> u0:43`.
- **write:** unique enclosing-transport chain `spec2 / record719 / body888 ->
  spec3 / body1238 -> application159 / HostResult157` in `spec5 / body1259 ->
  capsule ContextId2 -> issued control spec6`; write CLIF one `call fn35 -> u0:45`.

## The runtime net (Architect ruling — install this, do not redesign)

For each witness, map `source occurrence -> CLIF instruction -> machine address`
and install ONE discriminating runtime net over these six stages:

1. The earliest ordinary sibling outer-context call that actually executes.
2. The capsule-specific outer Host-Vis call (read `application146` / write
   `application159`).
3. Capsule Function entry.
4. Its application block and nested raw-worker instruction.
5. Trap-before-Result completion and capsule delivery / outer return.
6. The natural `ResourceBodyResult` match and typed trap.

**Real dynamic observations, not compile-time prints.** The D1's failure was
exactly that its `NESTED_CALL` lines fired while CLIF was generated. Every hit in
this net must be a runtime event on the executing binary. Record exact runtime
hit counts, the runtime HostResult / environment words observed, call/return
order across the six stages, and the FIRST edge that is never executed.

**Counts alone do not pair sites.** Identify each observed site from its source
application and decoded callee, never from a hit count coinciding with an
expected number. A shared count across two sites is not identity.

## Required D0 work (Architect ruling — each mandatory)

1. Reuse the exact D1 scratch mechanism as the scaffold; add no production
   candidate.
2. For BOTH read AND write, map `source occurrence -> CLIF instruction -> machine
   address` for all six stages; NEVER special-case read coordinates.
3. Install the discriminating runtime net; record exact hit counts, runtime
   HostResult/environment words, call/return order, and the first unexecuted edge,
   each site identified by source application and decoded callee.
4. **Control debt 1 — predecessor population.** Mutate an ACTUAL CFG predecessor
   edge / population (add, drop, or redirect a real carried-environment arrival
   edge), NOT an application-block argument count. The mutation must red for the
   exact predecessor/CFG property — that every arrival reaches one join with none
   omitted or duplicated — then restore.
5. **Control debt 2 — downstream result coordinate.** Independently mutate the
   PRODUCER of the downstream result coordinate and observe the exact refusal,
   then restore.
6. **Resolve `control_result_ordinary_index`.** If it is redundant, remove it from
   the proposed production design and state which typed identity fully determines
   delivery. If it is necessary, demonstrate its live consumer and its refusal
   under an independent mutation.
7. Preserve outer body-941 execution and `474/473` as an environment-only
   zero-call control; make no production edit.
8. Restore byte-clean and return the report, scratch diff, all commands/statuses,
   the source->CLIF->address maps, generated CLIF, runtime observation logs, and
   evidence digests.

## Exhaustive outcome routing (land in exactly one)

- **(1) An earlier ordinary shared context reaches the natural match/trap before
  the capsule outer application** — record where, dynamically, the earlier context
  traps; identify the planner/control relation that should stage the capsule
  application after it.
- **(2) The capsule outer application executes but selects another context
  target** — name the selected target and the relation that should have chosen the
  capsule context.
- **(3) The capsule Function enters but its application block / raw call is
  unreachable** — name the unreachable edge and the relation that should make it
  reachable.
- **(4) The raw call executes but Trap/Result delivery or downstream control loses
  it** — name where delivery is lost and the relation that should own it.

For the selected outcome, identify the exact planner/control relation that should
own the next stage.

## Closed axes remain closed (do NOT)

Do NOT call the future capsule early, borrow a sibling HostResult, reinterpret the
trap as suspension, or forward environment material as a result. No runtime
callable/continuation reification, global optional capsule, inferred callee,
environment-as-result, synthetic `ResourceBodyOk`, match weakening, Trap change,
or caller-success revival. Do not retarget `application146`/`application159` to
body 662; do not derive the context from `static_origin`; do not weaken the
carried-only route law; do not select control by constructor-name substring, a
`match451` search, or hardcoded numbers. Preserve `ResourceBodyResult`, natural
match semantics, typed trap 36/37, and all prior C/I/E/S and
source/member/projection/direction/delivery controls. This D0 is a runtime
measurement, not a production design.

## Reviewers, sequencing, contention

- **Reviewer:** the Architect ALONE reviews the D0 report, digests, source->CLIF->
  address maps, runtime logs, and scratch diff, and rules on the production design
  per the outcome routing. No Runtime QA (scratch-only, no production candidate),
  no Conformance Validator, no Decision, no publisher CI. Never `merged`.
- **Sequencing:** runtime ring (lane 1), single continuous turn to a complete
  report or a genuine blocker. Size L, tier T1 — this reuses the production-shaped
  D1 scratch scaffold and layers a discriminating runtime net (source->CLIF->
  machine-address mapping, dynamic observation over six stages across two
  witnesses, plus two real CFG/producer mutation controls and the
  `control_result_ordinary_index` resolution). Hold one continuous turn, do NOT
  `compact_self` mid-turn (a self-compact silently drops the assignment), and
  report a genuine blocker instead if headroom runs out. Restore byte-clean at the
  end regardless of outcome.
- **Contention:** reads and scratch-prototypes
  `crates/ken-runtime/src/cranelift_backend/lowering/{core,source,calls,units}.rs`,
  the generated-unit planner and its generated-context interning/keying, the
  static-worker `raw_worker_calls` target-table machinery, and the
  `rt_parity_native.rs` evidence wrappers; produces no landed change, so no
  crate/catalog contention with the concurrent lanes. Targeted builds ONLY via
  `scripts/ken-cargo` scoped to `ken-runtime`, never `--workspace`.
