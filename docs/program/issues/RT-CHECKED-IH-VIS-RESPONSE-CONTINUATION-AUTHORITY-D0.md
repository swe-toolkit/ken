---
id: RT-CHECKED-IH-VIS-RESPONSE-CONTINUATION-AUTHORITY-D0
title: "Scratch-only Vis response-continuation-authority D0 (no production candidate, no QA, no Decision, no merge): the RUNTIME-REACHABILITY D0 proved the affine checked-IH capsule is emitted but NEVER reached at runtime — read executes application138/context0 then Match451/trap36, write executes application175/context1 then Match464/trap37, and applications146/159 plus every capsule instruction hit ZERO, so the first missing edge PRECEDES capsule context selection. The Architect withdrew the future Host-Vis endpoint relation as execution authority: checked_ih_host_vis_endpoint selects a CheckedHostVisContinuation by a unique DOWNSTREAM same-constructor syntactic occurrence, not the source Vis e k relation saying which k receives the current response, and runtime falsifies it. Ken's driver is Vis e k -> apply k (H e) exactly once and in order (spec/40-runtime/42-evaluation.md sections 6.2, 6.4), so continuation authority belongs at the LIVE REQUEST BOUNDARY, never at an operation the continuation may later produce. The affine compiler-only capsule family stays SELECTED (callable recipe, ordered captures, Function-local target, one call, Trap-before-Result); only its downstream-constructor issuance KEY is withdrawn. For BOTH read and write witnesses, using NO hardcoded origins or constructor spellings: (1) from the dynamically live Host-Vis application structurally derive the enclosing ITree::Vis, its operation child, continuation child/binder k, response slot/type, and current HostResult operand — this is Vis e k, never same-constructor ancestry in a dispatcher match; (2) resolve that exact k through issued planner authority to worker closure occurrence/body/arity/ordered captures/StaticWorkerCallRoute/checked-IH record-transport/downstream consumer, never inferring from body-or-table presence, environment shape, runtime word, order, or numeric proximity; (3) produce the complete ordered response-to-successor chain from application138/175, state whether body662/body888 is immediate k, else enumerate each apply k resp and the first missing relation before 146/159, and prove whether 146/159 are selected successors, alternatives, or unrelated; (4) locate an independent definition/use pair for delivery (fixed expectation from the branch-owning Vis continuation binder or independent planner relation, actual use from the real call/branch terminator — rebuilding continuation_result_edges_owned_by from continuation_calls()/continuation_call_binding_for is NOT independent merely for being another function); (5) mutation-prove at natural seams — redirect/drop and duplicate the actual response-to-continuation successor edge/population preserving arity, and independently mutate the actual edge-producing use against unchanged binder/plan expectation or the converse (a comparison-only clone mutation is detector-side; a coherent single-authority change may remain valid; do not pin numeric coordinates); (6) preserve the accepted runtime net, effect order, traps 36/37, and the 474/473 zero-argument no-call materializer. Does NOT authorize substituting application138/175's HostResult: a current response is the capsule's argument only if the exact source Vis binding proves its k is body662/body888. Land in exactly one of four outcomes. Restore byte-clean and report generic derivation, exact rows, controls, and hashes."
status: ready
owner: runtime
size: L
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_7q7w05ag61zpw (thr_4rmv6f973sc62, 2026-08-31), following the HS6 Research prior-art advisory, ACCEPTING the runtime-implementer's RT-CHECKED-IH-CAPSULE-RUNTIME-REACHABILITY-D0 outcome 1 as a scratch measurement (report evt_5ke73vgnrn, report/diff/map/manifest SHA-256 3e4c3007d4695a8b729fdc997de454a059d3e2c77b14e134d155a5f9f77568e7 / cd5b765efa2cbb6ff8360bd3ec5a42e0bd9a52438031490a2e8e3fc04817da18 / 86968cf71ad4b5c615a4b11ef2b840eb7f4887e0fe8545dcb4927820b2337340 / 303d425c98b4abeaa6bbf951dd59611a4dc7f1bd071e06239049183e3db7f794). The Architect reproduced all four artifacts, both dynamic traces, the first-unexecuted-edge attribution, and both control-debt mutations at pickup 0102330ad4e13ef7b636a29411911b7d31e10407, tree bf6a014f966f883b9893a19fcf4e689be726bdbd, byte-clean. Finding: read executes application138/context0 then Match451/trap36; write application175/context1 then Match464/trap37; applications146/159 and every capsule instruction hit ZERO — the first missing edge precedes capsule context selection. Accepted controls: same-arity redirect-arrival CFG mutation (real edge, one-arrival closeout refuses); removal of control_result_ordinary_index (capsule CLIF byte-identical, redundant). NOT accepted as delivery evidence: the result-producer control mutates a cloned expected ContinuationCallIdentity (aggregates.rs:5495-5503); the detached edge is unchanged and core.rs:10430-10436 proves only comparison notices a corrupted expectation, so full control_identity stays a candidate coordinate not independently proven delivery authority. DESIGN RULING: the future Host-Vis endpoint relation is withdrawn as execution authority (checked_ih_host_vis_endpoint keys on a unique downstream same-constructor occurrence, not source Vis e k; runtime falsifies it). Ken's driver is Vis e k -> apply k (H e) once, in order (spec/40-runtime/42-evaluation.md sections 6.2, 6.4); authority belongs at the live request boundary. The affine compiler-only capsule family stays selected; only its downstream-constructor issuance key is withdrawn. SIXTH consecutive hard stop on the body-662 executor/staging question (force D1 NO evt_7e6jprw80srj8; carried-worker outcome 4 evt_5tm8gbxs7584g; Host-Vis outcome 3 evt_6rvrzpg80c02j; capsule feasibility outcome 2 evt_1zpgs2h0kd74q; staged-call static outcome 1 evt_6qkn80ag2tq89 / evt_4maaydhem9esy; runtime-reachability outcome 1 evt_5ke73vgnrn / this ruling). Shared predicate: a valid static fact repeatedly placed where the dynamic value or source-control choice that must consume it does not coexist. Base origin/main ea5cfdbc6 (advanced from 0102330ad by a doc-only L2 frame amendment only; the seven product blobs are unchanged — recheck at pickup); all seven reviewed product blobs (core eea98dc6, source c39f82e7, planner aggregates e7bc3628, lowering aggregates eaf1019b, calls fa010fed, units ccc6ddb2, parity test 6b2f14a7) Steward-verified identical to accepted base 0be25235b. @steward owns close/reframe/release; runtime parked until this named kick. Steward-recut per COORDINATION section 2."
---

> # READY — SCRATCH-ONLY VIS-RESPONSE-CONTINUATION-AUTHORITY D0. Released to
> # the runtime ring (lane 1) on `origin/main` `ea5cfdbc6`. Runtime is parked;
> # this IS the release.
> #
> # This is a MEASUREMENT node. It lands NO production candidate, opens NO PR,
> # routes NO QA, and needs NO Decision or merge. It reuses the accepted runtime
> # net, returns a report plus a scratch diff, source/CLIF/address maps, runtime
> # logs, and digests, and restores the branch byte-clean at the end. The Architect
> # ALONE reviews the D0 report and rules on the production design. Like every prior
> # D0/D1 in this chain, it is never `merged`.
> #
> # **The symptom is DIAGNOSED and the endpoint-authority axis is CLOSED.** The
> # RUNTIME-REACHABILITY D0 proved the capsule is emitted but never reached: both
> # witnesses execute an earlier ordinary application (read 138 / write 175) to the
> # natural match/trap; capsule applications 146/159 hit ZERO. The Architect
> # withdrew the future Host-Vis endpoint relation as execution authority — it keys
> # on a downstream same-constructor occurrence, not the source `Vis e k`. **Ken's
> # driver is `Vis e k -> apply k (H e)`, once and in order; authority lives at the
> # LIVE REQUEST BOUNDARY.** This D0 derives that boundary structurally for both
> # witnesses and decides whether the capsule endpoint is the immediate `k`, a
> # successor on a unique ordered chain, off-chain, or lacking independent delivery
> # provenance. The affine capsule family stays selected; only its
> # downstream-constructor issuance KEY is withdrawn. **Do NOT substitute
> # app138/175's HostResult.**

## What this D0 does

For BOTH the read AND write parity witnesses, using NO hardcoded origins or
constructor spellings in the derivation: structurally derive the live
`ITree::Vis` enclosing the dynamic Host-Vis application, resolve its exact
continuation binder `k` through issued planner authority, produce the complete
ordered response-to-successor chain from application 138/175, locate an
independent definition/use pair for delivery, and mutation-prove at the natural
seams. Land in exactly one of four outcomes. Restore byte-clean.

## Exact base and coordinates (Architect `evt_7q7w05ag61zpw`)

Base `origin/main` `ea5cfdbc6` (advanced from the D0 pickup `0102330ad` by a
doc-only L2 frame amendment only; the seven product blobs are unchanged). All
seven reviewed product blobs are identical to accepted base `0be25235b` (core
`eea98dc6`, source `c39f82e7`, planner aggregates `e7bc3628`, lowering aggregates
`eaf1019b`, calls `fa010fed`, units `ccc6ddb2`, parity test `6b2f14a7`). **If
main advances before pickup, recheck those seven blobs before edits.** Accepted
runtime facts: read executes `application138 / context0` then `Match451 / trap36`;
write executes `application175 / context1` then `Match464 / trap37`;
`application146/159` and every capsule instruction hit ZERO.

## The design ruling (Architect — build to this, do not redesign)

The future Host-Vis endpoint relation is **withdrawn** as execution authority.
`checked_ih_host_vis_endpoint` selects a `CheckedHostVisContinuation` because it
sits under a destination-frame match case with the same constructor — a unique
downstream syntactic occurrence, NOT the source `Vis e k` relation naming which
`k` receives the current response. Ken's driver is `Vis e k -> apply k (H e)`
exactly once and in order (`spec/40-runtime/42-evaluation.md` sections 6.2, 6.4),
so continuation authority belongs at the **live request boundary** binding the
current operation to its continuation, never at an operation the continuation may
later produce. The affine compiler-only capsule family stays SELECTED — callable
recipe, ordered captures, Function-local target, one call, Trap-before-Result all
remain valid; only its downstream-constructor issuance key is withdrawn.

**This does NOT authorize substituting `application138/175`'s HostResult.** A
current response is the capsule's argument ONLY if the exact source `Vis` binding
proves its `k` is `body662/body888`. If `k` differs and later exposes `146/159`,
every intervening continuation/effect executes in source order. If `146/159` are
alternatives, forcing them changes the program.

## Required D0 work (Architect ruling — each mandatory, both witnesses)

1. From the dynamically live Host-Vis application, structurally derive the
   enclosing `ITree::Vis`, its operation child, continuation child/binder `k`,
   response slot/type, and current HostResult operand. This is `Vis e k` — never
   same-constructor ancestry in a dispatcher match.
2. Resolve that exact `k` through issued planner authority to worker closure
   occurrence, body, arity, ordered captures, `StaticWorkerCallRoute`,
   checked-IH record/transport if any, and downstream consumer. NEVER infer from
   body/table presence, environment shape, runtime word, order, or numeric
   proximity.
3. Produce the complete ordered response-to-successor chain from
   `application138/175`. State whether `body662/body888` is immediate `k`;
   otherwise enumerate each `apply k resp` and the first missing relation before
   `146/159`. Prove whether `146/159` are selected successors, alternatives, or
   unrelated occurrences.
4. Locate an independent definition/use pair for delivery. Fixed expectation comes
   from the branch-owning `Vis` continuation binder or an independent planner
   relation; actual use comes from the real call/branch terminator.
   `continuation_result_edges_owned_by` rebuilding from `continuation_calls()` via
   `continuation_call_binding_for` is NOT independent merely because it is
   another function.
5. Mutation-prove at natural seams: redirect/drop and duplicate the ACTUAL
   response-to-continuation successor edge/population while preserving arity; and
   independently mutate the actual edge-producing use against the unchanged
   binder/plan expectation, or the converse. Mutating a comparison-only clone is
   detector-side. A coherent single-authority change may remain valid; do NOT pin
   numeric coordinates.
6. Preserve the accepted runtime net, effect order, traps 36/37, and the
   `474/473` zero-argument no-call materializer. Restore byte-clean and report
   the generic derivation, exact rows, controls, and hashes.

## Exhaustive outcome routing (land in exactly one)

- **(A) Live `Vis` binds `body662/body888` immediately and exposes an
  independent use edge** — report it; the next production design may select the
  capsule context at `application138/175` and call only after that application's
  own HostResult exists.
- **(B) Live `Vis` binds another `k`, but a unique ordered chain reaches
  `146/159`** — report every intermediate response and the first missing `apply`;
  the next design starts there.
- **(C) `146/159` is not on the selected chain, or the relation is
  absent/ambiguous** — the current capsule pairing is wrong for runtime control
  and gains no authority.
- **(D) A unique successor exists but binder and use lack independent
  provenance** — stop and name the representation seam separating issuance from
  use.

## Symptom inventory carried by this chain (Architect, verbatim)

1. Environment materialization had no worker-execution authority — keyed on carried
   environment material.
2. The zero-argument `474/473` materializer could not own an arity-one application —
   keyed on materialization site.
3. The later Host-Vis site had a HostResult but only outer-body route authority —
   keyed on downstream site presence.
4. Shared generated-context identity erased the nested application path — keyed on
   `(specialization, worker body)`.
5. Per-path capsule code was emitted but runtime selected an earlier ordinary path —
   keyed on future endpoint identity.
6. Delivery refusal corrupted an expected clone, not the actual producer — keyed on
   detector operand.

Shared predicate: a valid static fact was repeatedly placed where the dynamic
value or source-control choice needed to consume it did not coexist. Structural
closure roots continuation authority at live `Vis e k`; later endpoint identity
is a consequence to prove, never execution-control ownership.

## Closed axes remain closed (do NOT)

Do NOT substitute `application138/175`'s sibling HostResult; do NOT call the
capsule early; do NOT key continuation authority on a downstream
same-constructor occurrence; do NOT infer `k` from body/table presence,
environment shape, runtime word, order, or numeric proximity; do NOT prove
delivery with a comparison-only clone mutation; do NOT pin numeric coordinates in
the derivation or controls. No runtime callable/continuation reification, global
optional capsule, environment-as-result, synthetic `ResourceBodyOk`, match
weakening, Trap change, or caller-success revival. Preserve `ResourceBodyResult`,
natural match semantics, typed traps 36/37, effect order, and the accepted
runtime net. This D0 is a structural measurement of the live `Vis e k` boundary,
not a production design.

## Reviewers, sequencing, contention

- **Reviewer:** the Architect ALONE reviews the D0 report, digests,
  source/CLIF/address maps, runtime logs, and scratch diff, and rules on the
  production design per the outcome routing. No Runtime QA (scratch-only, no
  production candidate), no Conformance Validator, no Decision, no publisher CI.
  Never `merged`.
- **Sequencing:** runtime ring (lane 1), single continuous turn to a complete
  report or a genuine blocker. Size L, tier T1 — a structural derivation of the
  live `Vis e k` boundary across two witnesses, resolving `k` through planner
  authority, the full ordered response-to-successor chain, an independent
  definition/use delivery pair, and the natural-seam mutation controls, over the
  accepted runtime net. Hold one continuous turn, do NOT `compact_self` mid-turn
  (a self-compact silently drops the assignment), and report a genuine blocker
  instead if headroom runs out. Restore byte-clean at the end regardless of
  outcome.
- **Contention:** reads and scratch-prototypes
  `crates/ken-runtime/src/cranelift_backend/lowering/{core,source,calls,units}.rs`,
  the generated-unit planner and its continuation/`Vis` derivation, the
  static-worker `raw_worker_calls` target-table machinery, and the
  `rt_parity_native.rs` evidence wrappers; produces no landed change, so no
  crate/catalog contention with the concurrent lanes. Targeted builds ONLY via
  `scripts/ken-cargo` scoped to `ken-runtime`, never `--workspace`.
