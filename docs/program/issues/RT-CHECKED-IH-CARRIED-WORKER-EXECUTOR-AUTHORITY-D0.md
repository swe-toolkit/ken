---
id: RT-CHECKED-IH-CARRIED-WORKER-EXECUTOR-AUTHORITY-D0
title: "Scratch-only worker-executor-authority D0 (no production, no candidate, no QA, no Decision, no merge): answer ONE question before any further application-executor design — does the exact pending application at invocation/application 474/473 (record 608, specialization/owner 1, seat 671, worker body 662) already have a complete, unambiguous worker-execution recipe at the carried seat, distinct from CheckedIhEnvironmentTransport (whose source_call_identity names the environment MATERIALIZER funcid47, not a worker executor)? Classify the semantic operation at 474/473 from the source/planner derivation (worker execution vs partial/environment materialization vs another role; the OrdinaryApplication label and zero arg count are not proof); identify the exact existing StaticWorkerCallRoute and the exact worker_calls/raw_worker_calls entry for worker body 662 and whether the route survives after the static worker becomes a carried environment; partition the proposed operands exactly as call_static_worker_with_inputs does (explicit args, then captures, then generated-context continuation-input suffix) and name the one explicit argument (arity 1) by producer/origin/domain/word; reconcile captures 0..7 against record 608 and the WorkerCapture envelope without letting reconstruction stand in for the missing explicit argument or route; ONLY IF exact route+target+full operand partition are already authoritative, make one scratch call through the existing static-worker target-table mechanism under a move-only application authority separate from the environment transport, pair its Trap-checked Result to natural match 451, and report the selected constructor/exit/terminal error/four-effect order, preserving the environment crossing as no-call; restore byte-clean and report."
status: closed
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect review evt_7e6jprw80srj8 (thr_h4et7wgn4wkc, 2026-08-30), accepting the runtime-implementer's RT-CHECKED-IH-CARRIED-ENVIRONMENT-FORCE-CALL-D1 outcome NO (report evt_20kgrb6e5a28j, SHA-256 76554fa3cca5..., scratch-diff 992b21cd0ed4...). The D1 proved the typed role split compiles and emits the exact force call with discriminating runtime pairing, but the target the prior ruling selected — transport.source_call_identity() — is itself the captured-environment materializer (funcid47/ken_continuation_1), not a worker-body executor: it declares funcid43 but emits zero calls to it, so calling it again repeats environment materialization and match 451 correctly takes trap 36. Architect correction: CheckedIhEnvironmentTransport is authority for one force-materialized environment crossing only; prior selected-design point 5 (using its source_call_identity as an application target) is spent and withdrawn. The typed role split REMAINS correct and mandatory; what is missing is an independently grounded application-executor recipe, not another result type. No production recut authorized. Steward-recut per COORDINATION section 2. Base origin/main 774d1c90c02465187da77f13e6c3e08ab3726152, tree 062cb4dad4270c114d7a463a8d999be41e26467f; all seven reviewed product blobs (core eea98dc6, source c39f82e7, planner/lowering aggregates, calls, units, rt_parity_native) Steward-verified identical to the accepted D0 base 0be25235b. @steward owns close/reframe/release; runtime parked until this named kick. Symptom inventory: typed result roles were separated, but the environment transport's materializer identity was still used as worker-execution authority — keyed on treating authority for one semantic operation as authority for another."
---

> # CLOSED — D0 COMPLETE, OUTCOME 4 ACCEPTED (474/473 is materialization, not
> # worker execution). Architect ruling `evt_68ce7w8x6nb75` (thr_3fykd2macwy3t,
> # 2026-08-30).
> #
> # Measurement node — never `merged`; report `evt_5tm8gbxs7584g` (SHA-256
> # `1b4fd13b68a1...`, scratch-diff `59dfbc78a6b5...`) restored byte-clean at base
> # `e1ac1f27b`. The Architect reproduced all seven product blobs and every listed
> # digest and accepted the outcome. Finding: 474/473 is the zero-argument
> # `OrdinaryApplication` escape that constructs `ITree::Vis`'s carried checked-IH
> # environment; record 608 reconstructs captures 0..7 but supplies neither body
> # 662's explicit argument nor a call route. At carried `funcid50` the environment
> # has no `StaticWorkerBinding`, hence no carried `StaticWorkerCallRoute`; the
> # coincident `worker_calls[662]`/`raw_worker_calls[662]` do NOT restore authority
> # (the route type's law is carried-only). No scratch call, no manufactured
> # controls — correct.
> #
> # The semantic application is LATER: owner `PredeclaredFunctionId(5)`/`funcid45`,
> # marker/call `147/146`, template 4, `CheckedHostVisContinuation`, sole argument
> # origin 144 `Var(0)` at environment index zero — the existing sole `HostResult`
> # (produced after host dispatch, under the `FSOp::ctor_488` branch at match origin
> # 268), the arity-one value body 662 needs.
> #
> # Next: scratch-only successor
> # **`RT-CHECKED-IH-HOST-VIS-APPLICATION-EXECUTOR-AUTHORITY-D0`** on the same base
> # `e1ac1f27b`, force eligibility narrowed structurally to the exact
> # `CheckedHostVisContinuation` at 147/146 (474/473 becomes an explicit
> # negative/no-call control). Everything below this banner is retained as
> # chronology.
>
> This is a MEASUREMENT node. It lands NO production, opens NO candidate, routes NO
> QA, and needs NO Decision or merge. It returns a report plus digests (and a
> scratch diff only if a call is made), and the branch is restored byte-clean at the
> end. The Architect reviews the D0 report and chooses the next design from its
> outcome.
>
> **Why we are here.** `RT-CHECKED-IH-CARRIED-ENVIRONMENT-FORCE-CALL-D1` is CLOSED
> as a sound NO: the typed role split is correct and compiles, but the prior ruling
> selected `transport.source_call_identity()` as the application target, and that
> target is the captured-environment MATERIALIZER (`funcid47`), not a worker
> executor — calling it again just re-materializes an eight-field environment and
> match 451 takes trap 36. **The recurring predicate: authority for one semantic
> operation (environment materialization) was used as authority for another (worker
> execution).** The typed split stays mandatory; the missing piece is an
> independently grounded application-executor recipe, NOT a third result type, NOT a
> rename of the environment transport, NOT calling `funcid43` by its numeric
> identity or sibling-match shape. Measure first.

## The one question this D0 answers

Does the exact pending application at 474/473 already have a **complete,
unambiguous worker-execution recipe** at the carried seat, distinct from
`CheckedIhEnvironmentTransport`?

## Exact base and coordinates (Architect `evt_7e6jprw80srj8`)

Base `origin/main` `774d1c90c02465187da77f13e6c3e08ab3726152`, tree
`062cb4dad4270c114d7a463a8d999be41e26467f`. All seven reviewed product blobs are
identical to the accepted D0 base `0be25235b` (core `eea98dc6`, source `c39f82e7`,
planner/lowering aggregates, `calls`, `units`, `rt_parity_native`). **If main
advances before pickup, recheck those seven blobs before edits.**

The live force before consumption: kind `OrdinaryApplication`;
invocation/application `474/473`; record `608`; specialization/owner `1`; seat
`671`; worker body `662`. The withdrawn target `funcid47`/`ken_continuation_1`
allocates the tagless eight-field environment, wraps it in the identified two-field
constructor, writes it to Result+120, returns zero, and calls `funcid43` zero
times.

## Required D0 evidence (transcribed from the ruling — each is mandatory)

1. **Classify the semantic operation at 474/473** from the source/planner
   derivation: worker execution, partial/environment materialization, or another
   exact role. The `OrdinaryApplication` label and zero argument count are NOT
   themselves proof.
2. **Identify the exact existing executor authority for worker body 662:** the
   exact `StaticWorkerCallRoute` and the exact `worker_calls` or `raw_worker_calls`
   entry that would answer. A declared `funcid43`, body-origin equality, or a
   same-shape sibling is NOT target authority. Show whether the route survives after
   the static worker becomes a carried environment.
3. **Partition the proposed call operands exactly as `call_static_worker_with_inputs`
   does:** explicit arguments first, then captures, then any generated-context
   continuation-input suffix. The worker declares arity 1. Name the one explicit
   argument's semantic producer, stable origin, environment/domain coordinate, and
   runtime word. A nonrecursive field may serve ONLY if the existing binding law
   says it is that argument; matching the required count is insufficient.
4. **Reconcile captures 0..7 against record 608 and the `WorkerCapture` envelope**,
   but do NOT let that successful reconstruction stand in for the missing explicit
   argument or route.
5. **Only if the exact route, target, and full operand partition are already
   authoritative**, make ONE scratch call through the existing static-worker
   target-table mechanism, under a move-only application authority separate from the
   environment transport. Pair its Trap-checked Result to natural match 451 and
   report the selected constructor, exit, terminal error, and four-effect order.
   Preserve the earlier environment crossing as no-call.
6. **If a scratch call is made,** independently swap only the route/table and
   suppress or replace only the declared explicit argument; each must reach its own
   exact refusal, followed by restoration. If the prerequisites are absent, do NOT
   manufacture controls for an impossible call.
7. **Restore byte-clean** and return the report, scratch diff if any, all
   commands/statuses, generated CLIF, and evidence digests.

## Exhaustive outcome routing (the report lands in exactly one)

- **Existing executor authority plus complete operand recipe** — return the exact
  recipe and the successful or failing natural result for a later design ruling.
- **Exact target exists but the declared argument is absent** — name the missing
  semantic value and its last producer; do NOT call.
- **Target route is absent or ambiguous after materialization** — report the lost
  authority; do NOT select by body origin, `FuncId`, shape, count, or proximity.
- **474/473 is materialization rather than worker execution** — narrow the current
  force eligibility and identify the actual later application occurrence; do NOT
  force here.

## Closed axes remain closed (do NOT)

No tag/store/scan, environment-as-result, synthetic `ResourceBodyOk`,
`ResourceBodyResult` weakening, Trap reinterpretation/reordering, Host-Vis
reapplication, spent caller-success `ApplicationResultToRet` seat, or an alternate
callee chosen without existing route authority. Do NOT repair the withdrawn point 5
by renaming the call, appending application fields to the environment transport, or
calling `funcid43` from its numeric identity or sibling-match shape. The natural
match and Trap propagation remain correct. This D0 is a measurement of existing
authority, not a design.

## Reviewers, sequencing, contention

- **Reviewer:** the Architect reviews the D0 report, its digests, and any scratch
  diff, and chooses the next design per the outcome routing. No Runtime QA
  (scratch-only measurement, no product), no Conformance Validator, no Decision, no
  publisher CI (nothing lands). Like a prior D0/D1 measurement node, it is never
  `merged`.
- **Sequencing:** runtime ring (lane 1), single continuous turn to a complete report
  or a genuine blocker. Size M, tier T1 (deep authority/derivation reasoning across
  the static-worker target-table and generated-unit seam). Restore the branch
  byte-clean at the end regardless of outcome.
- **Contention:** reads and (only if authoritative) scratch-prototypes
  `crates/ken-runtime/src/cranelift_backend/lowering/{core,source,calls,units}.rs`,
  the static-worker target-table / `worker_calls` machinery, the generated-unit
  planner, and the `rt_parity_native.rs` evidence wrappers; produces no landed
  change, so no crate/catalog contention with the concurrent lanes. Targeted builds
  ONLY via `scripts/ken-cargo` scoped to `ken-runtime`, never `--workspace`.
