# WP frame — RT-NESTED-IH-NATIVE-REALIZATION D3-D5 (reconcile body 61 PER CONSUMER CLASS; GATE-0 C-liveness decides worker-obligation vs scope-dissolution; §1b closure resolved at entry 3)

> The completing D3-D5 slice of RT-NESTED-IH-NATIVE-REALIZATION (node
> `docs/program/issues/RT-NESTED-IH-NATIVE-REALIZATION.md`). Runtime lane,
> operator kernel-chain priority (`evt_7nkzsy27p7npw`). Owner: runtime. Size: M.
> Tier: T1. Gate: none. Architect (`evt_1nkx3f30hqp9y` z4027, `evt_2a1gzw40mprz0`
> z4029, `evt_53snpb8396g8a` HS#3 + amendment `evt_2cbtpf894nfzt`) is the REQUIRED
> reviewer on the candidate. Builds on RT-REALIZED-BACKEDGE-SOURCE-CONTINUATION
> (merged `897f1ea6a`) and RT-CHECKED-IH-REALIZATION-AUTHORITY (merged `e68ecd79`).
> On land, D3-D5 complete and `KERNEL-NESTED-IND` unblocks.

> ## §1b STRUCTURAL CLOSURE — RESOLVED AT ENTRY 3 (Architect `evt_53snpb8396g8a` + amendment `evt_2cbtpf894nfzt`)
>
> Three hard stops, ONE family: **in-place checked-IH realization has a
> NON-UNIFORM effect on the static-graph consumers of body 61.** The consumer
> classes are OPPOSITE and EXHAUSTIVE — a consumer either keys on 61 as a CALL
> BOUNDARY (partition wall, ABI boundary-signature — must STOP seeing a call after
> realization) or needs 61 as an EMITTABLE WORKER (worker_templates for the
> retained call — must STILL get a template). A single edge-kind reconciliation
> (z4029) satisfies the first class and, by construction, breaks the second —
> which is why HS#3 surfaced. The complete closure reconciles body 61's identity
> PER CONSUMER CLASS. The z4029 edge reconciliation is RETAINED (correct for the
> boundary class); this recut ADDS the retained-class resolution — which is one of
> TWO outcomes decided by a MEASUREMENT (below), not asserted. Fold into the SAME
> candidate (the prototype at `b17a1b6ea` is prototype-only). Node stays draft
> until seeded.

## Mechanism (Architect HS#3 ruling `evt_53snpb8396g8a`, amended `evt_2cbtpf894nfzt`, grounded @ `496d8637b`)

Body B = `StaticOriginId(61)` is realized IN-PLACE as a tail-recursive backedge
inside its enclosing function F (z4027 owner partition + z4029 edge
reconciliation — its recursor `StaticBody` edge became a non-boundary backedge,
`RecursiveBackedge` already emitted in F). But the SAME body B is ALSO the target
of a separately-RETAINED first-class callable C = `StaticOriginId(71)` — an
invocation-local compiler control capsule (`mod.rs:3539`) whose captures mix
JIT-seed (`ArtifactStatic`, minted before execution) and activation-carried
(invocation-time SSA) phases.

The collision, grounded: `resolve_worker_targets` (`units.rs:995`) populates
`worker_templates` from `plan.emittable_units()`, keyed by
`unit.body_occurrence()`. z4029 removed body 61 from the emittable-unit
population (its recursor `StaticBody` edge became a backedge), so its worker
template VANISHED. When C is lowered it hits the D7 pre-emission
capture-contract gate (`calls.rs:577-680`), which requires a planner-issued
worker template keyed by B's origin — now absent. Refusal (verbatim, at
`calls.rs:623-630`): `Unsupported { construct: "RetainedCallableCaptureContract",
reason: "a mixed-phase retained callable at StaticOriginId(71) has no
planner-issued worker template for body origin StaticOriginId(61) in this
function" }`.

THE DISPOSITION IS DECIDED BY A MEASUREMENT, not read off the shape (the
amendment's correction — the capsule existing and the machinery existing do NOT
establish liveness; that was a shape-read). GATE-0 measures whether body 61 is
actually INVOKED through C's (origin 71) first-class retained value at a
REACHABLE site:

- **C LIVE (a genuine escape) -> (a) DUAL EMISSION.** The missing worker template
  IS a NEW COMPILER-DERIVED PRODUCER OBLIGATION: the planner issues body 61's
  worker template for the retained callable 71 as a PLAN-LEVEL obligation, keyed
  on 71's compiler-owned oriented-plan + the static body 61 + the static capture
  schema. Body 61's SOURCE compiles TWICE — the specialized inline backedge in F
  (recursor path, z4029) AND a standalone worker (the escaped retained use, 71) —
  two emissions, DISTINCT identities and DISTINCT edge kinds, one source body.
  This is the standard inlined-AND-escaping pattern (LLVM/GCC emit the out-of-line
  copy of an address-taken function even when it is inlined at every direct call
  site).
- **C DEAD / vestigial -> (c) SPURIOUS RETENTION.** The worker-template omission
  is a CORRECT REFUSAL. The fix is UPSTREAM: extend the realization's
  reconciliation SCOPE to COMPLETELY DISSOLVE C and every capture-contract
  obligation it carries — NOT manufacture a worker for a callable nobody calls.
  Incomplete dissolution recurs as HS#4 at the next C consumer. NOT a producer
  obligation.

Why measure first: issuing a worker for a DEAD C is wasted emission AND papers
over an incomplete realization scope — it would HIDE a real (c) defect behind a
green build. The measurement is the single fact that decides producer-obligation
vs correct-refusal. (b) forbid-realization stays REFUTED as pessimizing (it would
sacrifice the in-place recursor realization z4027/z4029 correctly achieved to
serve a cold first-class reference); prior art does BOTH, not forbid.

SOUNDNESS is a SEPARATE gate from liveness (AC-DISCRIMINATOR-GATE, below):
derivability (is 61's body / 71's schema compiler-owned?) vs liveness (is C
invoked?). Both must pass for (a). 61 is a `StaticOriginId` (compiler-owned static
body); 71's capture SCHEMA is the static `AbiCaptureProvenance` classification the
D7 gate itself uses (`Carried` = activation-frame SSA vs `ArtifactStatic` =
pre-exec seed — a STATIC per-capture classification; only the VALUES are runtime).
So the template derives PURELY from 71's compiler-owned marking, never
caller-authored; template from the SCHEMA, capture VALUES through the environment.

## Deliverable

GATE-0 FIRST, then the branch it selects:

0. **GATE-0 (AC-C-LIVENESS) — the mandatory first measurement.** Measure whether
   body 61 is invoked through origin 71's first-class retained value at a
   reachable site (a reachability/invocation measurement of C, NOT a shape-read of
   the capsule or the machinery). Report the measurement WITH the candidate; it
   decides the disposition and the Architect reviews against it.

Then exactly one branch:

- **(a) C LIVE:** issue body 61's worker template for the retained callable 71 as
  a PLAN-LEVEL producer obligation — keep body 61 in the worker-template
  population BECAUSE it is a retained-callable target of 71 (even though its
  recursor edge was reconciled), extending the existing `resolve_worker_targets` /
  `declare_retained_body_targets_in_func` issuance. Reify-and-seal on origin 71's
  plan (kernel-checkable) so every use-context has the template BY CONSTRUCTION —
  NOT per-function on demand (recurs as HS#4/#5). Identity is BY origin 61 across
  BOTH realizations; do NOT dedup them (different ABIs, different call patterns).
  The standalone worker gets its OWN real `StaticBody` call boundary — it must NOT
  inherit the recursor backedge's non-boundary edge kind (z4029's kind is for the
  in-place path ONLY).
- **(c) C DEAD:** extend the realization's reconciliation scope to COMPLETELY
  dissolve C and every capture-contract obligation it carries.

The ordinary-`Match` selector and the `source.rs` catch-all stay untouched.

HARD-STOP ESCAPES (report + STOP, do not route around):
- **Condition-2 / inc3 axis-B:** if branch (a) genuinely requires a NEW
  `Lowered`/`LoweringOperand` variant or a new plan construct, that crosses a
  forbidden carry — route to Steward/operator (funding line); do NOT mint the
  variant. A larger BUILD through existing machinery is within-lane; a new
  construct is not.
- **AC-DISCRIMINATOR-GATE:** if any part of 61's body / 71's capture SCHEMA is
  runtime-DETERMINED (not merely runtime-valued), the durable first-class-callable
  carrier (the B2F analogue) is the OPERATOR FORK — route it.

## Fixed inputs (Architect-measured @ `496d8637b`; re-measure at your D0 cut)

- THE REFUSAL: `calls.rs:623-630`, the D7 capture-contract gate
  (`calls.rs:577-680`).
- THE ISSUANCE MACHINERY (Condition 2 exists here): `resolve_worker_targets`
  (`units.rs:995`) templating `worker_templates` from `plan.emittable_units()`
  keyed by `unit.body_occurrence()`; `declare_retained_body_targets_in_func`
  (`units.rs:2900`).
- THE RETAINED CALLABLE: `mod.rs:3539` (invocation-local control capsule, origin
  71); its `AbiCaptureProvenance` classification (`Carried` vs `ArtifactStatic`).
- GATE-0 MEASUREMENT TARGET: the reachable invocation sites of origin 71's
  first-class retained value (does control reach an invocation of body 61 through
  C).
- THE z4029 EDGE RECONCILIATION (retained, must still hold): realized-recursor
  edge minted at `construction.rs:771` (`register_static_body`), reconciled to a
  non-boundary backedge; partition sites `semantic_ir.rs:1296` / `:1350`; ABI
  boundary-signature validator `abi.rs:2735` / `:2854` / `:2771` / `:2861`.
- `validate_function_units` (`semantic_ir.rs:2395`) — AC-RECURSOR-UNTOUCHED
  control. The landed RT-CHECKED-IH D4 positional-ABI vector — AC-ABI-PIN.

## Acceptance criteria (Architect-pinned)

- **AC-C-LIVENESS (GATE-0, ahead of building anything):** a reachability/invocation
  MEASUREMENT of whether body 61 is invoked through origin 71's first-class value
  decides (a) vs (c). Capsule/machinery existence is NOT liveness. The candidate
  reports the measurement; it is the discriminator.
- **AC-WORKER-ISSUED (branch a):** body 61's worker template EXISTS for the
  retained callable 71, derived from 71's compiler-owned marking; the D7 gate is
  satisfied, not bypassed. Identity BY origin 61 across both realizations, NOT
  deduped (distinct ABIs); the worker carries its OWN `StaticBody` call boundary.
- **AC-DISCRIMINATOR-GATE (SEPARATE from liveness — derivability):** measure that
  61's body identity and 71's capture SCHEMA are compiler-owned/static; if any
  part is runtime-DETERMINED (body identity chosen at runtime, a capture SHAPE a
  caller authors, or provenance only from a forbidden carry), the disposition
  FLIPS to must-refuse and the B2F durable-carrier is the OPERATOR FORK.
- **AC-PLAN-LEVEL (branch a):** issued at the PLAN level (reify-and-seal on origin
  71's plan), not per-function — assert the refusal does not recur at a second use
  of 71.
- **AC-RECURSOR-UNTOUCHED (critical, sharpened):** the two realizations carry
  DISTINCT edge kinds. z4027/z4029 still hold: `validate_function_units` passes,
  join 50 owned by F, the recursor edge stays a non-boundary backedge, partition +
  ABI stops stay closed. The retained-worker reference carries its own `StaticBody`
  boundary; neither realization's edge kind leaks into the other. (Branch (c):
  dissolving C must not re-wall the recursor path either.)
- **AC-COMPLETE-PARITY (the goal):** native execution COMPLETES and native result
  == interpreter == `Nat 3`.
- **AC-ABI-PIN:** retain the RT-CHECKED-IH D4 positional-ABI `#[cfg(test)]` pin —
  recursor oriented frame / slots / parents / calls == the landed D4 vector.
- **AC-DUAL-CENSUS (AC-STATICBODY-CENSUS extended to BOTH directions):** census
  every `EdgeKind::StaticBody` consumer (z4029, boundary class) AND every
  emittable-unit / worker consumer (`resolve_worker_targets` by-origin + the
  `worker_templates` map, `declare_retained_body_targets_in_func`, the D7 gate —
  worker class), confirming body 61's DUAL role is correctly resolved and neither
  edge kind leaks into the other. A reason a consumer must still see the old form
  must be SHOWN, not assumed.

## §1b symptom inventory (RESOLVED at entry 3 — per-consumer-class closure)

Family predicate: "in-place checked-IH realization has a NON-UNIFORM effect on the
static-graph consumers of body 61." Two consumer classes, OPPOSITE and EXHAUSTIVE.

1. **HS#1 — partition (BOUNDARY class; RULED z4027, `evt_1nkx3f30hqp9y`).** Join 50
   walled to inner unit G though inlined into F.
2. **HS#2 — ABI boundary signatures (BOUNDARY class; RULED z4029,
   `evt_2a1gzw40mprz0`).** `boundary_signatures` reads the `StaticBody` edge as a
   callee definition. Both closed by the single edge reconciliation.
3. **HS#3 — retained-callable worker template (WORKER class; RULED
   `evt_53snpb8396g8a`, amended `evt_2cbtpf894nfzt`).** `resolve_worker_targets`
   lost body 61 from the emittable population when z4029 reconciled its edge; the
   D7 gate refuses retained callable 71. Closed PER MEASUREMENT: C live -> plan-level
   worker obligation; C dead -> complete dissolution.

CLOSURE VERDICT: the boundary class (HS#1/#2) and the worker class (HS#3) are
exhaustive over how a static-graph consumer keys on a body, so reconciling body
61's identity PER CONSUMER CLASS (worker if the retained consumer is live, else
dissolution) is the complete closure — no HS#4 on this family is expected. A
genuinely-new mechanism returns to the Architect; §1a next fires at HS#6.

## Forbidden boundary (all carries closed; unchanged from z4022/z4027/z4029)

- Plan-level producer obligation INSIDE the existing worker-template issuance,
  keyed on 71's compiler-owned oriented-plan. No selector or catch-all widening.
- No second join inventory / no `NativeJoinPlanV1` (stays withdrawn); no
  `.residual`; no unchecked / caller-authored plan; no terminal-`All` /
  `KERNEL-NESTED-IND` provenance (template carries only 71's own oriented-plan /
  checked-IH provenance).
- No new `Lowered` / `LoweringOperand` variant or carrier — if the obligation
  genuinely needs one, that is the Condition-2 hard-stop to Steward/operator.
- Static/dynamic: template from the SCHEMA, capture VALUES through the environment.

## Out of scope (Architect flag — do NOT fold in)

`define_continuation_context_bodies` (`units.rs:4183`), emission-owner
`Specialization` but lowering the raw owner's body, does NOT call
`validate_join_plan_consumption`. That is the OTHER realization shape (a
separately-emitted context fn), unreconciled — a DISTINCT latent gap, NOT the
`iterative_composition` witness here. Flag for a separate node if/when a witness
surfaces; do not fold it into this WP.

## Contention check

Touches the planner static graph (the realized-recursor `EdgeKind` from z4029)
and the plan-level worker-template issuance (`resolve_worker_targets` /
`declare_retained_body_targets_in_func`) — soundness-bearing producers of the
kernel-checkable required-join set, the ABI boundary signatures, and the
worker-template population. Runtime is the sole lane on this surface. Architect
REQUIRED reviewer (`evt_53snpb8396g8a` / `evt_2cbtpf894nfzt`) + Runtime QA. TCB
classification assessed at M4; the Architect review is required regardless.

## Sequencing

Releasable now (predecessors merged, closure ruled at entry 3, ring holding for
this amended frame). GATE-0 (C-liveness) is the first deliverable and is reported
with the candidate. On the candidate: my M1-M4 (Architect REQUIRED reviewer +
Runtime QA). Accepted-partial discipline holds: if native completes but parity
fails, or advances to a NEW named refusal, report it verbatim + site and STOP (§1a
next fires at HS#6). If AC-DISCRIMINATOR-GATE measures any part of 61's body / 71's
schema runtime-determined, STOP and route the operator B2F fork. If branch (a)
needs a new `Lowered`/operand variant, STOP and route the Condition-2 funding line.
On land, D3-D5 complete and `KERNEL-NESTED-IND` unblocks; the operator-prioritized
runtime kernel chain advances toward `DS-9`.
