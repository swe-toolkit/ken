# WP frame — RT-GENERATED-CONTINUATION-OPERAND-PAIRING (lane-1, HostResult-repair successor)

> Successor to [[RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE]] (D1 merged at
> squash `c7541df21`). The Architect ruled px8ta's post-D1 obstruction a
> DISTINCT object with a proven causal class (object-distinctness ruling
> evt_49v063sd0gv68, thr_1nmda4kcea9wg): an arity-correct but semantically
> mispaired generated-continuation frame operand. This is NOT a continuation
> of the merged HostResult-materialization repair and NOT a reopening of the
> merged [[RT-CAPTURE-CONTEXT-FRAME-EMIT]]. Owning team: runtime. Size M.
> Capability tier: T1 (an identity-provenance investigation across a
> generated-frame authority chain whose conditional repair is a
> soundness-bearing pairing boundary; the D0 deliverable is a lineage table
> that determines the seam, not a diff). Fresh object — hard-stop count zero.

> # AMENDED 2026-08-25 (Architect hard-stop #1, evt_3nm3jvapsf7cp, thr_3nmd0xy7dgh6g)
>
> D0 is DONE and sound. The lineage table bound every hop: funcid49 places the
> transferred Console Bool at funcid51 `Parameter(0)` offset 0; funcid51 loads it
> as `v10`; the raw `ClosureBody` descriptor independently declares one parameter
> plus four captures. The FIRST wrong authority is callee environment
> reconstruction: `define_continuation_context_bodies` reverses the context's
> coalesced five-slot `Parameter` run as though all five entries were source
> parameters, so body `Var(0)` consumes `v14` (`0x0207` = `InvocationBorrowed`,
> payload 2) instead of `v10` (`ImmediateBool(false)`). The provisional repair
> uses the raw owner's own ABI header to split `1 + 4`, reverses only the declared
> source-parameter prefix per `source_body_binding_order`, and appends raw +
> context captures unchanged — mirroring `define_unit_body`. It is
> authority-derived, count-validated, general over the descriptor, and hard-codes
> no px8ta slots. Under it, px8ta advances from root sentinel `-4` to `-1` with the
> same `BufferAllocate -> ConsoleIsTerminal(false) -> ResourceRelease` trace.
>
> The new `-1` is a DISTINCT mechanism, not residual pairing: the correctly-paired
> `ImmediateBool(false)` now reaches `joins.rs::lower_carried_constructor_match`,
> which calls the node-word helper `emit_carrier_tag`/`ken_boundary_tag_local` and
> `require_i64`-refuses before any Bool case because a canonical `ImmediateBool` is
> intentionally not an arena node. That is a representation-specific eliminator gap
> and the frame's ban forbids fixing it here. The Architect ruled: this WP lands
> COMPLETE FOR ITS OWN OBJECT once the pairing repair + pairing-local controls are
> clean (honest crossing, AC-6); the Bool consumer moves to the successor
> [[RT-CARRIED-BOOL-ELIMINATOR-DISPATCH]]. D0/D1/AC-0/1/2/5/6 and the banned-repair
> list are RETAINED; AC-3/AC-4 are REPLACED with the pairing-local controls below;
> the WIP pairing test is split. No Decision — the two mechanisms are independently
> correctable, no product/design tradeoff. New-chain hard-stop count is 1. Clean
> WIP (probes retained, NOT a candidate): `15abc5eb9255d61bb9033b4e0e236f2c07997d67`
> on the WP branch, one commit over seven runtime paths, base `2efa5ee07`. Durable
> symptom inventory entry 1: `architect/rt-cont-pairing-inventory @ d33344271`.
> Runtime is HELD until this amendment + the successor cut land.

## Objective

Bind px8ta's next causal residual to ONE exact source-to-slot lineage for the
Bool binder consumed by generated `funcid51`, then repair only at the first
authority that pairs the wrong source with that frame slot. No mechanism is
authorized before D0's lineage table selects the exact seam.

The Bool eliminator in `funcid51` (emission owner
`Specialization(ContinuationSpecializationId(0))`) receives a carrier word of tag
7 (`InvocationBorrowed`, payload 2) where the selected Console `Bool(false)` —
canonically `ImmediateBool`, tag 0, payload 0 — belongs. The host provably
returned `Bool(false)`; the eliminator is handed a different but individually
valid carrier, an invocation-borrowed handle. This is identity mispairing inside
a structurally valid run, not a Bool mis-decode.

## Fixed inputs (Architect ruling evt_49v063sd0gv68, grounded at `596428b48`)

Every relevant runtime/test blob is byte-identical at current `main`
(`596428b48`) and at D1 squash `c7541df21`: `joins.rs`, `calls.rs`,
`aggregates.rs`, `units.rs`, `object_linker_packaging.rs`, and px8ta.

- `RuntimeTrap(4)` is NOT causal identity and must not name the object.
  `crates/ken-runtime/src/cranelift_backend/lowering/calls.rs::call_declared_unit_target`
  maps every nonzero root `TrapWord` to the fixed `-4` process sentinel — its own
  comment says this deliberately erases internal trap identity. The baseline
  ignored px8ta witness produces `BufferAllocate -> ConsoleIsTerminal(false) ->
  ResourceRelease -> ControlledTrap RuntimeTrap(4)`.
- The object beneath the alias, established by scratch-only, byte-restored probes
  against the landed blobs:
  1. Preserving the exact trap word through the root sentinel changed the final
     observation to `RuntimeTrap(37)` — planner trap identity 37 is
     `PatternMatchFailure: "no runtime match case selected for
     decl:px8ds-exact-edges::Bool"`.
  2. Giving each emitted carried-match default its own compile-time marker changed
     the observation to `RuntimeTrap(1062)` — marker 1062 binds the taken default
     to generated `funcid51`, emission owner
     `Specialization(ContinuationSpecializationId(0))`, NOT the merged D1
     inactive-`ResourceError` dispatcher.
  3. At only marker 1062, returning the actual scrutinee carrier word changed the
     observation to `RuntimeTrap(519)`. `519 = 0x0207`: tag 7, payload 2. In the
     landed closed `BoundaryTag` definition (`crates/ken-runtime/src/boundary_value.rs`),
     tag 7 is `InvocationBorrowed`; a canonical `Bool(false)` is `ImmediateBool`,
     tag 0, payload 0.
  4. Finalized CLIF for `funcid51` shows that exact word entering the closed Bool
     default after comparisons against the declared `Bool` constructor identities.
     CLIF dump SHA-256
     `be654ab2c7dc6f0ce16b33310092417fc985df4f4ad7ce23b992c64fc73d5530`.
- Each probe changed the final runtime observer at the production-taken site —
  these are causal mutations. The effect trace independently proves the host
  returned `Bool(false)`.
- The defect authority chain (D0 must identify exactly where it binds the Bool
  formal to the tag-7 operand, and where the selected Console Bool resides):
  `source Bool binder -> planned continuation input source -> call input ordinal
  -> declared frame slot/store -> funcid51 load/eliminator`. D0 must NOT assume in
  advance whether the fault is caller assembly order, parameter/capture
  classification, source-coordinate resolution, or callee environment
  reconstruction.

## Anchor

Base the successor branch on current `main` (`596428b48`). The Architect's
finalized `funcid51` CLIF dump (SHA-256 `be654ab2c7...`) is read-only evidence,
not a base or candidate. No prior WIP branch is load-bearing for this object.

## Deliverables

- **D0 — one exact lineage table for the `funcid51` Bool binder.** Produce a
  single table binding, with independent identity at every hop and no numeric
  coincidence standing in for pairing:
  - planner identity/owner of the Bool binder;
  - the target call instruction;
  - the descriptor slot kind and ordinal;
  - the caller input source;
  - the emitted stack store;
  - the callee load;
  - the runtime carrier tag observed at the eliminator.
  The table must locate where the chain binds the Bool formal to the tag-7 operand
  AND where the selected Console Bool actually resides. Counts and raw numeric
  coincidence are NOT pairing evidence. D0 does not assume the faulting hop in
  advance; the table selects it.
- **D0 stop.** Return the lineage table and the identified first-mispairing hop.
  Do NOT infer the repair from a count and do NOT green px8ta by any of the banned
  moves below.
- **D1 (conditional on D0) — repair at the first mispairing authority.** D0
  selected callee environment reconstruction: the repair is at
  `units.rs::define_continuation_context_bodies` (source formal/capture partition
  and semantic environment order), splitting the coalesced context run `1 + 4` via
  the raw owner's own ABI header and reversing only the declared source-parameter
  prefix (see the amendment banner). Preserve the declared frame order and the
  existing fail-closed membership/arity guards. Do NOT compensate in `joins.rs`
  and do NOT compensate in carrier decoding. Repair only the proven layer; zero
  `trusted_base()` delta.

## Acceptance criteria

- AC-0 (lineage table, not a count). D0 produces the exact `funcid51` Bool-binder
  lineage table above, with an independent identity established at every hop. An
  occurrence count, a slot cardinality, or a raw numeric coincidence does NOT
  satisfy this AC.
- AC-1 (seam identified). The table names the single first hop at which the Bool
  formal is bound to the tag-7 operand, and names where the selected Console Bool
  resides. The fault is attributed to exactly one of caller assembly order,
  parameter/capture classification, source-coordinate resolution, or callee
  environment reconstruction — chosen by the evidence, not presumed.
- AC-2 (pairing-oracle control). A same-cardinality operand-swap mutation that
  preserves all frame sizes and types must RED the source-to-slot pairing oracle.
  A control that a permutation can pass is not discriminating and does not satisfy
  this AC.
- AC-3 (pairing-local controls — REPLACES the old eliminator-branch/hostile-tag
  ACs, Architect hard-stop #1 evt_3nm3jvapsf7cp). The old AC-3/AC-4 required the
  Bool consumer to go green; that consumer is a distinct object moved to
  [[RT-CARRIED-BOOL-ELIMINATOR-DISPATCH]]. This WP's controls are pairing-local:
  1. a same-cardinality structural control consumes the reconstructed
     environment's POSITION 0 and proves it is the selected caller operand,
     followed by the four raw captures and then the context-capture suffix. The
     whole-five-word reversal mutation must change that observation and RED the
     unchanged control;
  2. paired with the actual px8ta advance: the exact tag-7-at-`Var(0)` Bool
     default disappears under the repaired production path; restoring whole-run
     reversal restores that old causal observation;
  3. a hostile word supplied as the declared source parameter must remain that
     exact hostile word at environment position 0 — pins pairing neutrality;
     whether Bool Match accepts or refuses it belongs to the successor.
- AC-4 (preservation + probe removal + test split). Preserve raw-owner header
  mismatch refusal, the existing arity/membership guards, and the D1 HostResult
  controls; remove every D0 probe before candidate handoff. The WIP test
  `generated_context_pairing_selects_both_bool_arms_and_refuses_tag_seven` must
  NOT become this WP's pairing oracle as written — it passes `selected` directly
  as the scrutinee and its constant case bodies never consume `env[0]`, so
  reversing the reconstructed environment can leave it green. SPLIT it: this WP
  keeps a control that observes `env[0]`; the successor gets the direct Bool
  eliminator false/true/hostile test without pairing scaffolding.
- AC-5 (D1 regressions held). The landed D1 one-active-`HostResult`-payload
  behavior and its hostile inactive/selected controls remain unchanged and green.
- AC-6 (px8ta advance, honest). Re-run px8ta HALF B: the claim is ONLY that this
  exact Bool-operand mispairing residual disappears — report the next observation
  if reached (e.g. `ConsoleIsTerminal` progressing), else name the first new
  causal obstruction. Do NOT promise end-to-end green.
- AC-NO-REGRESSION. Whole-suite green in CI; local targeted `-p ken-runtime` /
  `--test` only, never `--workspace`.

## Banned repairs (Architect ruling)

- treating `InvocationBorrowed` as false;
- widening Bool recognition;
- hard-coding px8ta slot numbers;
- changing the `BoundaryTag` definition;
- changing the root `-4` reporter sentinel (reporter honesty is the separate
  [[RT-UNIT-FAILURE-STATUS-PROVENANCE]] object);
- routing around the declared unit target (the target exists, is called, and
  executes — so this is also distinct from
  [[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]]).

## Reviewers

Architect (component fit: the D0 table must bind to generated-function/owner
and slot identity rather than counts; the selected seam must be justified by
the lineage evidence; any D1 must repair the first mispairing authority and
preserve declared frame order + fail-closed guards, not compensate downstream)
+ runtime-qa (the pairing-oracle control is discriminating against a
same-cardinality permutation, and the acceptance turns on the lineage table and
the swap/hostile controls, not a count). No Decision fork is open. Adversary
advisory, non-gating.

## Contention check

Touches `crates/ken-runtime/src/cranelift_backend/lowering/` (the generated-frame
authority chain: `joins.rs`, `calls.rs`, `units.rs`, `aggregates.rs`, as the
lineage table selects) and focused tests (`crates/ken-cli/tests` incl. px8ta +
runtime frame tests). Must NOT touch `joins.rs` carrier decoding as a compensation
or `crates/ken-runtime/src/boundary_value.rs` `BoundaryTag`. No overlap with lane
2 (language/elaborator) or lane 3 (foundation catalog packages). Runtime ring
exclusive.

## Capability tier

T1. Size M — one focused increment: the D0 lineage table to a selected seam, then
at most the single first-authority pairing repair. Sized to reach a seam
selection (or a genuine hard stop) within about an hour; a
seam-selected-but-D1-needs-its-own-cut outcome is a good stop, not a miss.
