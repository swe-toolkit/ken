# WP frame — RT-NESTED-IH-NATIVE-REALIZATION D3-D5 (thread PER-EMISSION identity into EVERY plane keyed by source-origin / one global emission model; GATE-0 measured C LIVE -> dual emission; §1b deeper predicate, entries 1-6; HS#6 = BoundaryClosureEnvironment issuance, durability-gated A/A'/B trichotomy — re-key / durably materialize / B2F operator fork)

> The completing D3-D5 slice of RT-NESTED-IH-NATIVE-REALIZATION (node
> `docs/program/issues/RT-NESTED-IH-NATIVE-REALIZATION.md`). Runtime lane,
> operator kernel-chain priority (`evt_7nkzsy27p7npw`). Owner: runtime. Size: M.
> Tier: T1. Gate: none. Architect (`evt_1nkx3f30hqp9y` z4027, `evt_2a1gzw40mprz0`
> z4029, `evt_53snpb8396g8a` HS#3 + amendment `evt_2cbtpf894nfzt`,
> `evt_557xhr47qtzrg` HS#4, `evt_4a4tj2h1eatsa` HS#5, `evt_312vam8jegfbf` +
> `evt_7f76p7y4k3jma` HS#6) is the REQUIRED reviewer on the candidate. Builds on
> RT-REALIZED-BACKEDGE-SOURCE-CONTINUATION (merged `897f1ea6a`) and
> RT-CHECKED-IH-REALIZATION-AUTHORITY (merged `e68ecd79`). On land, D3-D5 complete
> and `KERNEL-NESTED-IND` unblocks.

> ## §1b STRUCTURAL CLOSURE — DEEPER PREDICATE, ENTRIES 1-6 (Architect `evt_312vam8jegfbf` + AMENDMENT `evt_7f76p7y4k3jma` HS#6 adds entry 6 = BoundaryClosureEnvironment issuance, durability-gated A/A'/B; `evt_4a4tj2h1eatsa` entry 5; `evt_557xhr47qtzrg` entry 4)
>
> SIX hard stops, ONE predicate — deeper than the entry-3 "consumer class"
> framing, which the Architect corrected at HS#4: **a static plane keyed by
> SOURCE ORIGIN ALONE (or by a SINGLE GLOBAL emission model), while in-place
> realization + dual emission make one source origin (body 61) correspond to
> MULTIPLE per-emission realizations that can need DIFFERENT ownership AND
> DIFFERENT representation.** The backend ALREADY keys contexts by
> `(enclosing_specialization, worker_body_origin)` pervasively
> (`continuations.rs:6408`, `responses.rs:70`); the join-ownership and
> join-representation planes and their siblings are the planes that never became
> emission-aware. Enumerating them one at a time is the RT-NATIVE-FNSPLIT signal —
> the closure is to thread the compiler-owned emission identity the backend already
> carries into EVERY such plane, so each emission of body 61 owns, disposes, AND
> represents its own per-emission realization. HS#5 arrived at BUILD (not at the
> census) because the z4033/z4034 AC-DUAL-CENSUS enumerated planes 1-4 but MISSED
> the representation plane — the closure still covers it; the CENSUS was
> under-enumerated (Architect owns the miss), which is why AC-DUAL-CENSUS is now
> "prove per-emission or name the Cond-2 escape, re-derived from origin/main."
> GATE-0 measured C LIVE (below), so the retained consumer gets a worker (dual
> emission), not dissolution. Entry 6 (HS#6) FORKS on TWO ORDERED measurements —
> FIRST is the `BoundaryClosureEnvironment` record DURABLE/frame-independent or
> LIVE-DOMAIN; THEN are body 50's escaping captures static-by-value or
> dynamic/by-reference-to-frame: (A) durable + static-by-value -> re-key
> per-emission; (A') live-domain-or-by-ref but static -> durably MATERIALIZE
> (within-lane iff existing machinery, else Cond-2); (B) any dynamic-escaping
> capture -> the reserved B2F operator fork. HS#5's `Some` is NOT durability
> evidence; the durability must be measured. Node stays draft until seeded.

## The six planes — keyed by source-origin or one global emission model (the corrected §1b inventory)

Body 61 is realized IN-PLACE as a tail-recursive backedge inside enclosing
function F (z4027 owner partition + z4029 edge reconciliation), AND — because
GATE-0 measured it LIVE — is ALSO emitted as a standalone out-of-line worker for
the separately-retained first-class callable C = `StaticOriginId(71)`. Two
emissions of one source body, distinct identities, distinct edge kinds. Each of
the six stops is a static plane that keyed on source origin 61 ALONE (or on a
single global emission model) and so could not tell the two per-emission
realizations apart (entry 6 forks A/A'/B on an ordered durability-then-capture
measurement — see the deliverable):

1. **HS#1 — partition wall (RULED z4027, `evt_1nkx3f30hqp9y`).** Join 50 walled to
   inner unit G though inlined into F. Source `StaticBody` boundary trusted.
2. **HS#2 — ABI boundary signatures (RULED z4029, `evt_2a1gzw40mprz0`).**
   `boundary_signatures` (`abi.rs:2854`) reads the `StaticBody` edge as a callee
   definition. Both closed by the single edge reconciliation (edge -> non-boundary
   backedge).
3. **HS#3 — worker-template omission (RULED `evt_53snpb8396g8a` + amendment
   `evt_2cbtpf894nfzt`).** `resolve_worker_targets` (`units.rs:995`) keys
   `worker_templates` by `unit.body_occurrence()`; z4029 removed body 61 from the
   emittable population, the template vanished, the D7 gate (`calls.rs:577-680`,
   refusal `calls.rs:623-630`) refuses retained callable 71. Closed PER GATE-0
   MEASUREMENT: C LIVE -> plan-level worker obligation (dual emission).
4. **HS#4 — join-ownership (RULED `evt_557xhr47qtzrg`; population corrected by
   measurement `evt_34t60sqcw27mv` / disposition `evt_52xmt58zxxyqv`, THIS recut).**
   Join ownership by source origin ALONE (`function_owner` ->
   `required_join_origins`, `joins_traps.rs:675`, SINGLE-OWNER via
   `validate_function_units` `semantic_ir.rs:2395`) MIS-CHARGES source join
   `StaticOriginId(44)` to F, which never emits it. MEASURED: 44 is emitted EXACTLY
   ONCE, by worker 6 (`PredeclaredFunctionId(6)`, body 50, consumes `{50, 44}`); F
   (`PFI(3)`, body 82) consumes `{30, 33, 36, 50, 72, 73}` and never emits 44; the
   genuinely dual-emitted join is `50`, not 44. Source-origin-alone leaves F with a
   PHANTOM requirement for 44 it never emits or disposes ->
   `finalize_join_disposition` UNDER-coverage at `joins.rs:2145`: `function left
   planned source join StaticOriginId(44) neither emitted nor statically
   unselected`. (`joins.rs:2140` is the OVER-coverage face — HS#1's "classified
   outside its owning function".)

5. **HS#5 — join-representation (RULED `evt_4a4tj2h1eatsa`; the plane the
   z4033/z4034 census MISSED — Architect owns the miss).** After option (A) clears
   HS#4, a carried `Match` refuses at `joins.rs:2499` (`:2497` on origin/main):
   `UnsupportedLowering { construct: "a carried Match arm", reason: "dynamic arms
   must produce scalar Int or Bool values" }` — the SAME carried Match admits its
   `StructuralNat` arm and refuses its `Closure` arm. `build_join_result_plan`
   (`joins_traps.rs:544`) computes ONE joins vector indexed by SOURCE ORIGIN with a
   SINGLE GLOBAL `functionized_units`, unioning across owner-descriptors
   (`:477-483`, either -> `CarrierWord`). SOUNDNESS: a `CarrierWord` join CANNOT
   mint the recursor backedge (`merge_scalar_operand`
   `ScalarMergeKind::RecursiveBackedge => Err`), so F's recursor emission REQUIRES
   `NativeScalarPair` while the worker's closure emission REQUIRES `CarrierWord`;
   for a source join in BOTH, the union-to-`CarrierWord` would BREAK F's backedge.
   The representation type is PLANNER-OWNED and CLOSED by design
   (`joins_traps.rs:44-46`: two-way; lowering may not add a third or select from a
   predecessor). WIP `ec6d401dc`.

6. **HS#6 — BoundaryClosureEnvironment issuance (RULED `evt_312vam8jegfbf`,
   AMENDED `evt_7f76p7y4k3jma`; §1a research-discharged this cycle, next trigger
   HS#9).** After HS#5's Fork 1 clears, worker 5 (`PFI(5)`, unit result origin 61)
   RETURNS the already-issued worker 6 / body 50 closure across its return boundary
   (upward-funarg ESCAPE); the raw `Lowered::Closure` reaches
   `boundary_transfer_admissibility` (`boundary.rs:1032`) and is refused at
   `boundary.rs:1047`: `a closure cannot cross the boundary: it is runtime-local
   and live-domain only, and it has no durable lane` (`boundary_environment=None`).
   It is the GENERIC closure refusal, NOT the `ComputationalRecursorClosure`
   in-flight-activation arm — body 50 is a representable closure whose DURABLE
   issuance did not fire. The per-emission machinery EXISTS:
   `BoundaryClosureEnvironment` (`aggregates.rs:149`, keyed by `owner:
   ContinuationEmissionOwner`, holding `capture_origins: Vec<StaticOriginId>` + a
   structural `AggregateOccurrenceId`) and its per-emission
   `CheckedIhEnvironmentTransport` (`aggregates.rs:~190`, lifetime-tracked
   `PlannedReferentLifetime`; "the continuation call result SUBSTITUTES for the raw
   closure"). BUT the amendment corrects a shape-read: HS#5's
   `boundary_environment=Some` proves only a WITHIN-FRAME (live-domain) environment
   existed, NOT that it is durable; an ESCAPE needs a DURABLE (frame-independent)
   environment. Neither `Some` nor `None` is the durability tell — it must be
   MEASURED. WIP `aca946d3b` (parent `ec6d401dc`).

## Mechanism (the closure, grounded @ `496d8637b`)

DISPOSITION = WITHIN-LANE STRUCTURAL CLOSURE (not a 5th point-fix; not — on the
grounding — the operator hard-stop, with a measured escape). Bring the
join-ownership plane into the per-`(owning-emission / specialization, origin)`
keying the backend uses everywhere else. Each emission owns and disposes exactly
the spine joins it ACTUALLY emits (measured per emission, `evt_34t60sqcw27mv`):
worker 6 (`PredeclaredFunctionId(6)`, body 50) emits+consumes `{50, 44}`; F
(`PFI(3)`, body 82) emits+consumes `{30, 33, 36, 50, 72, 73}` and never emits 44.
Join 44 is emitted EXACTLY ONCE (worker 6); the genuinely dual-emitted join is
`50` (both F and worker 6 emit it). `function_owner` / `required_join_origins`
become resolved PER OWNING EMISSION; single-owner becomes
single-owner-PER-EMISSION — precisely what the context/continuation machinery
already assumes. A source origin appearing in two emissions' partitions (join 50)
is EXPECTED and sound under `(specialization, origin)` keying; charging 44 to F is
a PHANTOM obligation ONLY under the source-origin-alone keying that is the actual
bug.

Why within-lane: this threads the SAME specialization identity the backend
already computes and carries (through continuations / responses / aggregates)
into the one plane that ignores it. It extends an existing discipline; it does
not invent one.

The branch-(a) issuance layer (HS#3), already built and confirmed correct by the
Architect: recursor relations stay `RealizedRecursorTransfer`; the worker
projects its own `EmittableCallKind::StaticBody`; both D7 omissions clear
(body 61 AND the next-layer 61/body-50 spine); both checked markers realize;
identity distinct by origin; NO new plan/operand variant (Condition-2 not tripped
for issuance). This recut adds the join-ownership plane as the completion of (a).

## Deliverable

Branch (a) — GATE-0 measured C LIVE (`evt_4mdmhsv4k28b1`: reachable call spine
73 -> 72 -> 71, body edge 71 -> 61, both join origins consumed; measured, not a
shape-read). Complete the dual emission across BOTH layers:

1. **Worker issuance (HS#3, built).** Keep body 61 in the worker-template
   population BECAUSE it is a retained-callable target of 71, extending
   `resolve_worker_targets` / `declare_retained_body_targets_in_func`.
   Reify-and-seal on origin 71's plan (kernel-checkable), NOT per-function.
   Identity BY origin 61 across both realizations; do NOT dedup; the worker gets
   its OWN `StaticBody` call boundary (never the recursor backedge's non-boundary
   kind).
2. **Join-ownership per-specialization (HS#4, this recut).** Thread the existing
   `(specialization / owning-emission, origin)` keying into `function_owner`,
   `required_join_origins`, `finalize_join_disposition`, and
   `validate_function_units`, so each function is required to own exactly its
   MEASURED emitted joins (option A, `evt_52xmt58zxxyqv`): required(F) =
   `{30,33,36,50,72,73}` (44 REMOVED — F never emits it); required(worker 6) =
   `{50,44}`; required(worker `PFI(5)`/body 61) = none. Move 44's required-ownership
   to worker 6, its measured emitter — this resolves BOTH the F under-coverage and
   the worker's missing disposition from one change. Do NOT make F emit a second
   copy of 44 (option B refused: it fabricates a disposition lowering never
   performs, and re-electing the realization to inline body 50 into F IS the
   Condition-2 construct — do not mint it). Additive; the only dual-charged join is
   50 (both emitters).

3. **Join-representation per-emission (HS#5, this recut).** MEASURE FIRST (mirror
   of the HS#4 population run): is the closure-carrying source join
   WORKER-EMISSION-ONLY (like 44) or SHARED with F's recursor-backedge emission
   (like 50)?
   - FORK 1 (worker-only): make that join's representation `CarrierWord` for the
     worker emission via EXISTING machinery — summarize the worker emission with
     its actual functionized-ness so its closure arm -> `CarrierRequired` ->
     `CarrierWord` (the union at `joins_traps.rs:477-483` already promotes it). No
     conflict with F. Within-lane.
   - FORK 2 (shared with F): genuine per-`(owning-emission, source-origin)`
     representation — F's copy stays `NativeScalarPair` (its `RecursiveBackedge`),
     the worker's copy is `CarrierWord` (its closure). The per-source-origin joins
     vector + union CANNOT express this; key the representation per-`(emission,
     origin)`, minting the `JoinPlanToken` per owning emission
     (`joins_traps.rs:79-85`; it already carries origin + representation),
     threading the SAME compiler-owned emission identity (z4033/z4034). NO union
     hack, NO third representation, NO lowering-side selection.
   CONDITION-2 GATE (Architect flags this the LIKELIEST plane to trip it — the
   representation type is CLOSED `:44-46`): Fork 1 is plainly within-lane. Fork 2 is
   MEASURED — if per-`(emission,origin)` representation forces lowering to SELECT
   from an emitted predecessor (violating `:44-46`), or needs a NEW plan-map
   construct beyond re-keying the existing per-origin vector, that IS the
   Condition-2 hard-stop -> STOP and route to Steward/operator; do NOT mint it.

4. **BoundaryClosureEnvironment issuance per-emission (HS#6, this recut) — GATED
   by TWO ORDERED MEASUREMENTS, forking A / A' / B (amendment
   `evt_7f76p7y4k3jma`).** Rule and re-key NOTHING until measured; do NOT read the
   fork off "returned across a boundary", and do NOT read durability off HS#5's
   `Some`.
   - MEASUREMENT 1 (FIRST, gates everything): is `AggregateOccurrenceId(22)`'s
     represented environment DURABLE / frame-independent (a compiler-owned
     aggregate/const record that survives `PFI(5)`'s frame) or LIVE-DOMAIN
     (frame-based, valid only while `PFI(5)`'s frame is alive)?
   - MEASUREMENT 2 (THEN): are body 50's escaping captures STATIC/compiler-owned
     BY-VALUE snapshots, or DYNAMIC runtime SSA / BY-REFERENCE into `PFI(5)`'s
     activation frame? (A by-reference-to-frame capture cannot cross; it must be
     captured by-value into a durable record or it is (B).)
   Select the branch and keep the turn active only through the selected permitted
   branch:
   - (A) RE-KEY per-emission — ONLY IF durable AND all escaping captures static
     BY-VALUE. `None` is a missing per-emission issuance of an EXISTING durable
     capability; fire `BoundaryClosureEnvironment` / `CheckedIhEnvironmentTransport`
     keyed per-emission from the `ContinuationEmissionOwner` identity in hand
     (z4033/z4034), so the raw closure is SUBSTITUTED before boundary admissibility.
     No borrow of F / the source-owner. §1b entry 6, within-lane.
   - (A') MATERIALIZE durably — IF live-domain (or any by-reference-to-frame
     capture) BUT all captures static/by-value-snapshottable: the escape needs a
     DURABLE MATERIALIZATION (frame-independent record), NOT mere re-keying of a
     live-domain environment. WITHIN-LANE ONLY IF the materialization uses EXISTING
     machinery with NO new `Lowered`/`LoweringOperand` variant or plan construct;
     ELSE it trips Condition-2 -> HARD-STOP to Steward/operator.
   - (B) B2F OPERATOR FORK — IF ANY escaping capture is DYNAMIC runtime SSA needing
     runtime-managed heap lifetime past the frame. Condition-2 HARD-STOP to
     Steward/operator; `None` is a correct refusal no re-keying or materialization
     satisfies. Do NOT mint the lane; do NOT fake durability by issuing a
     frame-local environment per-emission (that returns a closure pointing at
     `PFI(5)`'s torn-down frame = USE-AFTER-FREE). MIXED: a SINGLE
     dynamic-runtime-escaping capture forces (B) for the whole environment.
   Report BOTH measurements with the disposition.

The ordinary-`Match` selector and the `source.rs` catch-all stay untouched.

HARD-STOP ESCAPES (report + STOP, do not route around):
- **Condition-2 (join-ownership plane) — a DISTINCT measurement from the issuance
  one.** The issuance layer cleared Condition-2 (extended existing machinery, no
  new variant). The join plane is measured SEPARATELY: if making it
  specialization-aware genuinely requires a NEW plan construct or a new
  `Lowered`/`LoweringOperand` variant — rather than threading the specialization
  identity already in hand — that trips Condition-2; STOP and route to
  Steward/operator (the funding line). Do NOT mint the construct. The
  `(specialization, body_origin)` keying already present in continuations /
  responses is strong evidence this is an extension, not a construct — measure it.
- **AC-DISCRIMINATOR-GATE:** if any part of 61's body / 71's capture SCHEMA is
  runtime-DETERMINED (not merely runtime-valued), the durable first-class-callable
  carrier (the B2F analogue) is the OPERATOR FORK — route it. (Measured
  compiler-owned at GATE-0: `StaticOriginId(61)` body; closed `Lexical` schema,
  arity 1, nine positional captures; only VALUES runtime. Re-confirm at the cut.)

## Fixed inputs (Architect-measured @ `496d8637b`; re-measure at your D0 cut)

- THE HS#4 REFUSAL: `finalize_join_disposition` UNDER-coverage `joins.rs:2145`
  (OVER-coverage face `joins.rs:2140`). WIP checkpoint
  `ea9cfde3841d5ad65834592922a2c9fb99a48968`.
- THE JOIN-OWNERSHIP PLANE (make per-specialization): `function_owner` ->
  `required_join_origins` (`joins_traps.rs:675`); `finalize_join_disposition`
  (`joins.rs:2140`/`:2145`); `validate_function_units` (`semantic_ir.rs:2395`,
  single-owner -> single-owner-per-emission). Source join `StaticOriginId(44)`.
- THE EXISTING PER-SPECIALIZATION KEYING TO THREAD IN:
  `continuation_context_for(enclosing_specialization, worker_body_origin)`; "one
  context per (enclosing_specialization, worker_body_origin)"
  (`continuations.rs:6408`, `responses.rs:70`).
- THE WORKER ISSUANCE MACHINERY (HS#3): `resolve_worker_targets` (`units.rs:995`)
  templating `worker_templates` from `plan.emittable_units()` keyed by
  `unit.body_occurrence()`; `declare_retained_body_targets_in_func`
  (`units.rs:2900`). THE D7 GATE: `calls.rs:577-680`, refusal `calls.rs:623-630`.
- THE RETAINED CALLABLE: `mod.rs:3539` (invocation-local control capsule, origin
  71); its `AbiCaptureProvenance` classification (`Carried` vs `ArtifactStatic`).
- THE z4029 EDGE RECONCILIATION (retained, must still hold): realized-recursor
  edge minted at `construction.rs:771` (`register_static_body`), reconciled to a
  non-boundary backedge; partition `semantic_ir.rs:1296`/`:1350`; ABI validator
  `abi.rs:2735`/`:2854`/`:2771`/`:2861`.
- THE HS#6 BOUNDARY-CLOSURE MACHINERY: `boundary_transfer_admissibility`
  (`boundary.rs:1032`, refusal `:1047`); `BoundaryClosureEnvironment`
  (`aggregates.rs:149`; `owner: ContinuationEmissionOwner`; `capture_origins:
  Vec<StaticOriginId>`; `AggregateOccurrenceId`); `CheckedIhEnvironmentTransport`
  (`aggregates.rs:~190`; `PlannedReferentLifetime`). Returned closure body 50 under
  `PFI(5)`, `boundary_environment=None`; measure `AggregateOccurrenceId(22)`
  durability (frame-independent vs live-domain) + body 50 capture schema
  (static-by-value vs dynamic / by-reference-to-frame).
- The landed RT-CHECKED-IH D4 positional-ABI vector — AC-ABI-PIN.

## Acceptance criteria (Architect-pinned)

- **AC-WORKER-ISSUED (built, retained):** body 61's worker template EXISTS for the
  retained callable 71, derived from 71's compiler-owned marking; the D7 gate is
  satisfied, not bypassed. Identity BY origin 61 across both realizations, NOT
  deduped; the worker carries its OWN `StaticBody` call boundary.
- **AC-JOIN-PER-SPECIALIZATION (HS#4):** `function_owner` / `required_join_origins`
  / `finalize_join_disposition` resolve PER owning emission; each function is
  required to own exactly its MEASURED emitted joins (worker 6 owns `{50,44}`; F
  owns `{30,33,36,50,72,73}` — 44 excluded; the dual-emitted 50 charged to both).
  The `joins.rs:2145` under-coverage refusal clears WITHOUT a join bypass, owner
  special-case, `NativeJoinPlanV1`, or an F second-copy of 44.
  `validate_function_units` passes under single-owner-per-emission (no global
  double-owner).
- **AC-DISCRIMINATOR-GATE (SEPARATE — derivability):** 61's body identity and 71's
  capture SCHEMA are compiler-owned/static; if any part is runtime-DETERMINED, the
  disposition FLIPS to must-refuse and the B2F durable-carrier is the OPERATOR FORK.
  HS#6 is the concrete instance for the RETURNED closure: its capture-set lifetime
  (durable/static-by-value vs live-domain/by-ref vs dynamic-escaping) selects
  A / A' / B.
- **AC-NO-DANGLING-ESCAPE (HS#6 named failure mode):** the returned closure (body
  50) crossing `PFI(5)`'s boundary must NEVER be issued a re-keyed LIVE-DOMAIN
  environment — that returns a closure pointing at `PFI(5)`'s torn-down frame
  (upward-funarg use-after-free). Re-key (A) is permitted ONLY on a
  measured-DURABLE environment; a live-domain environment routes to (A') durable
  materialization (existing machinery, else Cond-2) or (B) the operator fork, never
  a re-key.
- **AC-PLAN-LEVEL:** worker + join obligations issued at the PLAN level
  (reify-and-seal on origin 71's plan), not per-function — assert neither refusal
  recurs at a second use of 71.
- **AC-RECURSOR-UNTOUCHED (critical; per-emission):** z4027/z4029 are F's-emission
  facts and are PRESERVED. F owns exactly what it EMITS — `{30,33,36,50,72,73}`
  (join 44 EXCLUDED; F never emits it); the dual-emitted join 50 resolves to F for
  F's own copy. Partition + ABI stops stay closed, recursor edges stay
  `RealizedRecursorTransfer`. The worker's join-ownership is ADDITIVE (worker 6's
  specialization owns `{50,44}`); neither realization's edge kind or ownership
  leaks into the other.
- **AC-COMPLETE-PARITY (the goal):** native execution COMPLETES and native result
  == interpreter == `Nat 3`.
- **AC-ABI-PIN:** retain the RT-CHECKED-IH D4 positional-ABI `#[cfg(test)]` pin —
  recursor oriented frame / slots / parents / calls == the landed D4 vector.
- **AC-DUAL-CENSUS (the closure's falsifiable guardrail — PROVE per-emission, do
  NOT grep):** RE-DERIVE from origin/main (do NOT trust this enumeration — the
  z4033/z4034 census MISSED the representation plane and HS#5 arrived at build) the
  set of EVERY static plane whose value can depend on the emission model, and for
  EACH prove it is either (i) made per-emission by threading the compiler-owned
  emission identity, or (ii) named as the Condition-2 escape. The known planes:
  `EdgeKind::StaticBody` consumers (z4029); emittable-unit / worker consumers
  (`resolve_worker_targets` by-origin + `worker_templates`,
  `declare_retained_body_targets_in_func`, the D7 gate); the join-OWNERSHIP plane
  (`function_owner`, `required_join_origins`, `finalize_join_disposition`,
  `validate_function_units`); AND the join-REPRESENTATION plane
  (`build_join_result_plan`, `summarize_result_phase`,
  `result_phase_environment_for_owner`, the per-origin joins vector, the
  `functionized_units` flag, the union at `joins_traps.rs:477-483`); AND the
  BoundaryClosureEnvironment / CheckedIhEnvironmentTransport ISSUANCE /
  MATERIALIZATION plane (`aggregates.rs:149` + `~:190`;
  `unit_boundary_environment_record` / `_occurrence` / `_fields`; the
  `boundary_environment: Option<AggregateOccurrenceId>` issuance sites — measured
  per-emission AND for durability, HS#6). Charge EVERY
  source join to the emission(s) that ACTUALLY emit it (measured owner run):
  source-origin-alone charging (44 -> F) is the defect being fixed; the
  dual-emitted join (50) is charged to BOTH emitters WITH its per-emission
  representation (F `NativeScalarPair` for the backedge; worker `CarrierWord` for
  the closure). FALSIFIABLE GUARDRAIL: after the change, for every function
  required == emitted AND every join's representation matches its emission's need;
  a plane that UNIONS incompatibly (representation vs the `RecursiveBackedge`
  constraint) is a DEFECT, not a pass. IF a plane CANNOT be made per-emission by
  threading the existing identity — or forces a lowering-side selection / a new
  plan construct — THAT plane is the Condition-2 escape: STOP and route it.

## §1b closure verdict

The SIX planes are all instances of the ONE deeper predicate (a static plane keyed
by source origin, or by a single global emission model, vs per-emission dual
emission). Inventory: 1 partition wall (z4027) / 2 ABI boundary_signature (z4029) /
3 worker-template emittable-unit (z4032) / 4 join-ownership (z4033/z4034) / 5
join-REPRESENTATION `build_join_result_plan` (HS#5) / 6 BoundaryClosureEnvironment
+ CheckedIhEnvironmentTransport issuance (HS#6 `evt_7f76p7y4k3jma`). Applied
UNIFORMLY — thread the compiler-owned emission identity into every such plane the
census finds. §1a and §1b CONVERGE at entry 6: IF the returned closure's
environment is durable + its captures static-by-value (A), or live-domain/by-ref
but materializable with EXISTING machinery (A'), entry 6 SHARES the entry-4
predicate at the issuance/materialization plane and the in-lane chain continues; IF
A' needs a new construct, or ANY capture is dynamic-runtime-escaping (B), the
in-lane chain ENDS here at Condition-2 / the B2F operator fork. §1a DISCHARGED for
HS#6 (amendment on the same advisory, not a new trigger); next research trigger
HS#9.

## Forbidden boundary (all carries closed; unchanged from z4022/z4027/z4029/z4032)

- Thread specialization identity into the join-ownership plane; extend the
  existing worker-template issuance. No join bypass, no owner special-case, no
  selector or catch-all widening.
- No second join inventory / no `NativeJoinPlanV1` (stays withdrawn); no
  `.residual`; no unchecked / caller-authored plan; no terminal-`All` /
  `KERNEL-NESTED-IND` provenance (templates/ownership carry only 71's own
  oriented-plan / checked-IH provenance).
- No new `Lowered` / `LoweringOperand` variant or plan construct — if the join
  plane, the issuance, the representation, or the BoundaryClosureEnvironment
  materialization genuinely needs one, that is the Condition-2 hard-stop to
  Steward/operator.
- Representation plane: NO union hack, NO third `JoinResultRepresentation`, NO
  lowering-side selection of representation from an emitted predecessor
  (`joins_traps.rs:44-46` — the type is closed). Per-`(emission,origin)`
  representation is minted in the PLANNER (`build_join_result_plan`) via the
  existing `JoinPlanToken`, never in lowering.
- HS#6 escape: NEVER re-key a LIVE-DOMAIN environment for the returned closure
  (dangling upward-funarg use-after-free); (A) re-key is permitted only on a
  measured-durable environment. A live-domain-but-static environment routes to
  (A') durable materialization (existing machinery, else Cond-2); any
  dynamic-runtime-escaping capture routes to (B) the operator fork. Do NOT fake
  durability with a frame-local per-emission environment.
- Static/dynamic: template from the SCHEMA, capture VALUES through the environment.

## Out of scope (Architect flag — do NOT fold in)

`define_continuation_context_bodies` (`units.rs:4183`), emission-owner
`Specialization` but lowering the raw owner's body, does NOT call
`validate_join_plan_consumption`. That is the OTHER realization shape (a
separately-emitted context fn), unreconciled — a DISTINCT latent gap, NOT the
`iterative_composition` witness here. Flag for a separate node if/when a witness
surfaces; do not fold it into this WP.

## Contention check

Touches the planner static graph (the realized-recursor `EdgeKind` from z4029),
the plan-level worker-template issuance (`resolve_worker_targets` /
`declare_retained_body_targets_in_func`), the join-ownership plane
(`function_owner` / `required_join_origins` / `finalize_join_disposition` /
`validate_function_units`), AND the join-representation plane
(`build_join_result_plan` / the per-origin joins vector / `JoinPlanToken`), AND the
BoundaryClosureEnvironment / CheckedIhEnvironmentTransport issuance plane
(`aggregates.rs:149` / `~:190`) — soundness-bearing producers of the
kernel-checkable required-join set, the ABI boundary signatures, the
worker-template population, the join result representation, and the durable
boundary-closure environment. Runtime is the sole lane on this surface. Architect
REQUIRED reviewer (`evt_7f76p7y4k3jma` HS#6 + `evt_4a4tj2h1eatsa` HS#5 +
`evt_557xhr47qtzrg` + the z4027/z4029/z4032 chain) + Runtime QA. TCB
classification assessed at M4; the Architect review is required regardless.

## Sequencing

Releasable now (predecessors merged, HS#4 ruled, ring building from the ruling per
`evt_5y8mfsztaw3pe`). The candidate reports the GATE-0 C-liveness measurement AND
the Condition-2 join-plane measurement. On the candidate: my M1-M4 (Architect
REQUIRED reviewer + Runtime QA). Accepted-partial discipline holds: if native
completes but parity fails, or advances to a NEW named refusal, report it verbatim
+ site and STOP (§1a next fires at HS#6). If AC-DISCRIMINATOR-GATE measures any
part of 61's body / 71's schema runtime-determined, STOP and route the operator
B2F fork. If making the join-ownership plane (or the issuance) specialization-aware
genuinely needs a new `Lowered`/operand variant or plan construct, STOP and route
the Condition-2 funding line. On land, D3-D5 complete and `KERNEL-NESTED-IND`
unblocks; the operator-prioritized runtime kernel chain advances toward `DS-9`.
