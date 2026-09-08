# WP frame — RT-NESTED-IH-NATIVE-REALIZATION D3-D5 (thread PER-SPECIALIZATION keying into EVERY source-origin-keyed static plane; GATE-0 measured C LIVE -> dual emission; §1b deeper predicate closed at entry 4 = join-ownership)

> The completing D3-D5 slice of RT-NESTED-IH-NATIVE-REALIZATION (node
> `docs/program/issues/RT-NESTED-IH-NATIVE-REALIZATION.md`). Runtime lane,
> operator kernel-chain priority (`evt_7nkzsy27p7npw`). Owner: runtime. Size: M.
> Tier: T1. Gate: none. Architect (`evt_1nkx3f30hqp9y` z4027, `evt_2a1gzw40mprz0`
> z4029, `evt_53snpb8396g8a` HS#3 + amendment `evt_2cbtpf894nfzt`,
> `evt_557xhr47qtzrg` HS#4) is the REQUIRED reviewer on the candidate. Builds on
> RT-REALIZED-BACKEDGE-SOURCE-CONTINUATION (merged `897f1ea6a`) and
> RT-CHECKED-IH-REALIZATION-AUTHORITY (merged `e68ecd79`). On land, D3-D5 complete
> and `KERNEL-NESTED-IND` unblocks.

> ## §1b STRUCTURAL CLOSURE — DEEPER PREDICATE, RESOLVED AT ENTRY 4 (Architect `evt_557xhr47qtzrg`, correcting the entry-3 closure `evt_53snpb8396g8a` / `evt_2cbtpf894nfzt`)
>
> FOUR hard stops, ONE predicate — deeper than the entry-3 "consumer class"
> framing, which the Architect corrected at HS#4: **a static plane keyed by
> SOURCE ORIGIN ALONE, while in-place realization + dual emission make one source
> origin (body 61) correspond to MULTIPLE per-specialization realizations.** The
> backend ALREADY keys contexts by `(enclosing_specialization, worker_body_origin)`
> pervasively (`continuations.rs:6408`, `responses.rs:70`: "one context per
> (enclosing_specialization, worker_body_origin)"); the join-ownership plane and
> its siblings are the planes that never became specialization-aware. Enumerating
> them one at a time is the RT-NATIVE-FNSPLIT signal — STOP enumerating. The
> complete closure threads the specialization identity the backend already
> computes and carries into EVERY source-origin-keyed plane, so each emission of
> body 61 owns and disposes its own per-specialization realization. GATE-0
> measured C LIVE (below), so the retained consumer gets a worker (dual emission),
> not dissolution. Fold into the SAME candidate (the prototype at `b17a1b6ea` /
> WIP `ea9cfde3` are prototype-only). Node stays draft until seeded.

## The four source-origin-keyed planes (the corrected §1b inventory)

Body 61 is realized IN-PLACE as a tail-recursive backedge inside enclosing
function F (z4027 owner partition + z4029 edge reconciliation), AND — because
GATE-0 measured it LIVE — is ALSO emitted as a standalone out-of-line worker for
the separately-retained first-class callable C = `StaticOriginId(71)`. Two
emissions of one source body, distinct identities, distinct edge kinds. Each of
the four stops is a static plane that keyed on source origin 61 ALONE and so
could not tell the two per-specialization realizations apart:

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
4. **HS#4 — join-ownership (RULED `evt_557xhr47qtzrg`, THIS recut).** After (a)
   issues the worker, body 61's spine — incl. source join `StaticOriginId(44)` —
   is emitted TWICE. But `function_owner` -> `required_join_origins`
   (`joins_traps.rs:675`) key by source origin ALONE and are SINGLE-OWNER
   (`validate_function_units` `semantic_ir.rs:2395` forbids double-owner). So
   exactly ONE emission owns+disposes its copy of 44; the other carries 44's code
   with no scoped disposition -> `finalize_join_disposition` UNDER-coverage at
   `joins.rs:2145`: `function left planned source join StaticOriginId(44) neither
   emitted nor statically unselected`. (`joins.rs:2140` is the OVER-coverage face
   — HS#1's "classified outside its owning function".)

## Mechanism (the closure, grounded @ `496d8637b`)

DISPOSITION = WITHIN-LANE STRUCTURAL CLOSURE (not a 5th point-fix; not — on the
grounding — the operator hard-stop, with a measured escape). Bring the
join-ownership plane into the per-`(owning-emission / specialization, origin)`
keying the backend uses everywhere else. Each emission of body 61 owns and
disposes its OWN copy of the spine's joins: 44 under F's specialization (z4027,
unchanged) AND 44 under the worker's specialization (new, worker-scoped).
`function_owner` / `required_join_origins` become resolved PER OWNING EMISSION;
single-owner becomes single-owner-PER-EMISSION — precisely what the
context/continuation machinery already assumes. A source origin appearing in two
specializations' partitions is EXPECTED and sound under `(specialization, origin)`
keying; it is a "double-owner" defect ONLY under the source-origin-alone keying
that is the actual bug.

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
   `validate_function_units`, so each emission owns+disposes its own spine joins.
   44 under F stays F's (z4027); 44 under the worker's specialization is new and
   worker-scoped. Additive; no global double-owner.

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
- The landed RT-CHECKED-IH D4 positional-ABI vector — AC-ABI-PIN.

## Acceptance criteria (Architect-pinned)

- **AC-WORKER-ISSUED (built, retained):** body 61's worker template EXISTS for the
  retained callable 71, derived from 71's compiler-owned marking; the D7 gate is
  satisfied, not bypassed. Identity BY origin 61 across both realizations, NOT
  deduped; the worker carries its OWN `StaticBody` call boundary.
- **AC-JOIN-PER-SPECIALIZATION (HS#4):** `function_owner` / `required_join_origins`
  / `finalize_join_disposition` resolve PER owning emission; each emission of body
  61 owns+disposes its own copy of the spine joins (incl. 44). The `joins.rs:2145`
  under-coverage refusal clears WITHOUT a join bypass, owner special-case, or
  `NativeJoinPlanV1`. `validate_function_units` passes under
  single-owner-per-emission (no global double-owner).
- **AC-DISCRIMINATOR-GATE (SEPARATE — derivability):** 61's body identity and 71's
  capture SCHEMA are compiler-owned/static; if any part is runtime-DETERMINED, the
  disposition FLIPS to must-refuse and the B2F durable-carrier is the OPERATOR FORK.
- **AC-PLAN-LEVEL:** worker + join obligations issued at the PLAN level
  (reify-and-seal on origin 71's plan), not per-function — assert neither refusal
  recurs at a second use of 71.
- **AC-RECURSOR-UNTOUCHED (critical; per-emission):** z4027/z4029 are F's-emission
  facts and are PRESERVED. Under per-specialization keying, join 50 and join 44
  UNDER F still resolve to F, partition + ABI stops stay closed, recursor edges
  stay `RealizedRecursorTransfer`. The worker's join-ownership is ADDITIVE (its
  specialization owns its own copies); neither realization's edge kind or ownership
  leaks into the other.
- **AC-COMPLETE-PARITY (the goal):** native execution COMPLETES and native result
  == interpreter == `Nat 3`.
- **AC-ABI-PIN:** retain the RT-CHECKED-IH D4 positional-ABI `#[cfg(test)]` pin —
  recursor oriented frame / slots / parents / calls == the landed D4 vector.
- **AC-DUAL-CENSUS (the closure's falsifiable guardrail):** census EVERY static
  plane keyed by source origin ALONE — `EdgeKind::StaticBody` consumers (z4029),
  emittable-unit / worker consumers (`resolve_worker_targets` by-origin +
  `worker_templates`, `declare_retained_body_targets_in_func`, the D7 gate), AND
  the join-ownership plane (`function_owner`, `required_join_origins`,
  `finalize_join_disposition`, `validate_function_units`) — and confirm each is
  per-specialization-correct for the dual emission. IF a plane the census finds
  CANNOT be made per-specialization by threading the existing
  `(specialization, origin)` identity, THAT plane is the Condition-2 escape — STOP
  and route it. A reason a consumer must still see the old form must be SHOWN.

## §1b closure verdict

The four planes are all instances of the ONE deeper predicate (source-origin-alone
keying vs per-specialization dual emission). Applied UNIFORMLY — thread the
existing specialization identity into every source-origin-keyed plane the census
finds — this is the complete structural closure, not a plane-by-plane chain. So no
HS#5 on this family is expected. A genuinely-new mechanism returns to the
Architect; §1a next fires at HS#6.

## Forbidden boundary (all carries closed; unchanged from z4022/z4027/z4029/z4032)

- Thread specialization identity into the join-ownership plane; extend the
  existing worker-template issuance. No join bypass, no owner special-case, no
  selector or catch-all widening.
- No second join inventory / no `NativeJoinPlanV1` (stays withdrawn); no
  `.residual`; no unchecked / caller-authored plan; no terminal-`All` /
  `KERNEL-NESTED-IND` provenance (templates/ownership carry only 71's own
  oriented-plan / checked-IH provenance).
- No new `Lowered` / `LoweringOperand` variant or plan construct — if the join
  plane or the issuance genuinely needs one, that is the Condition-2 hard-stop to
  Steward/operator.
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
`declare_retained_body_targets_in_func`), AND the join-ownership plane
(`function_owner` / `required_join_origins` / `finalize_join_disposition` /
`validate_function_units`) — soundness-bearing producers of the kernel-checkable
required-join set, the ABI boundary signatures, and the worker-template
population. Runtime is the sole lane on this surface. Architect REQUIRED reviewer
(`evt_557xhr47qtzrg` + the z4027/z4029/z4032 chain) + Runtime QA. TCB
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
