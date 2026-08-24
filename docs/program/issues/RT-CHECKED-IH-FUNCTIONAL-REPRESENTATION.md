---
id: RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION
title: "Give Ken a first-class representation for an ESCAPING functional induction hypothesis, so the nullary_force of a checked computational IH whose realized value is stored into a constructor (escapes its frame) can be lowered honestly. The decisive escape measurement (runtime-implementer evt_79jd1nxamqd95) proved the realized IH value's immediate parent is a Construct on both checked-family programs -- it is stored straight into a constructor field, so the non-escaping use-site-specialization remedy cannot apply (no in-frame application site, no response argument at the use). Ken has no first-class functional-IH value: StaticWorkerBinding is compiler metadata with no runtime word/tag/layout/env-pointer/callable identity (lowering/mod.rs:3578-3603), LoweringOperand is exactly {Specialized(Lowered), Carried(CarriedBoundaryWord)} with no closure/worker arm, and an ordinary carried word cannot hold `lambda response. rec (k response)`. This node introduces the genuine new representation. It carries a design D0 the Architect rules before build: materialized closure value vs defunctionalized carried tag (code identity + environment + apply dispatcher). Successor to the closed RT-CHECKED-IH-RECURSIVE-OPERAND-SEAT."
status: ready
owner: runtime
size: L
gate: none
depends_on: [RT-IH-MARKER-PRODUCER-COMPLETE, RT-NATIVE-TRACK0-REARM, RT-CHECKED-IH-CAPTURED-ENV-SCHEMA]
blocks: [RT-CARRIED-IH-DISPATCH-SITEOP, RT-CLOSURE-BOUNDARY-RESIDUAL]
github: null
origin: "Cut by the Steward (scoping ruling evt_5pmk273zg5paa) on the Architect's pre-committed conditional ruling (evt_2f4bbmt7qfde1) after the decisive ESCAPE measurement (runtime-implementer evt_79jd1nxamqd95) returned ESCAPING. Supersedes RT-CHECKED-IH-RECURSIVE-OPERAND-SEAT (closed; both its Closure-A operand-seat mechanism and its NULLARY_FORCE re-reading were refuted -- the realized IH value escapes into a constructor, a capability gap, not a seam fix). HS=5 (Steward of record); ESCAPING is a scoping decision, not a hard-stop increment. Steward-filed per COORDINATION section 2."
---

> # RELEASED 2026-08-24 — all predecessors merged + reseat done; kicked on the pi
> ring.
>
> Every gate is cleared: [[RT-CHECKED-IH-CAPTURED-ENV-SCHEMA]] merged (7426bb1f2),
> [[RT-NATIVE-TRACK0-REARM]] merged, [[RT-IH-MARKER-PRODUCER-COMPLETE]] closed; and
> the operator-named runtime→pi reseat is DONE (Steward-executed: implementer
> gpt-5.6-sol high T1, leader+qa gpt-5.6-terra medium — moot.toml b7375783c). The
> reason this node was held (predecessor + reseat seam) is gone, so it is flipped
> draft -> ready and kicked on the fresh pi runtime ring. The captured-env fork
> that reverted it is resolved: the predecessor admits the non-empty StaticWorker
> capture env. Tier-3 = Record build + defunctionalize at core.rs:11674, dispatcher
> AC-REPR (two-tier, PR #2802), plus the production EMITTER constructing
> SynthesizedArgument::WorkerCaptureOperand at the force seam (the tier-1 reconcile
> machinery is DORMANT until this emitter exists). Capability tier T1 (genuine
> lowering implementation) — implementer correctly seated at sol/high. The D0
> (DEFUNCTIONALIZE) and the two-tier dispatch design below are RULED and durable;
> the earlier "not startable / skeleton" prose further down is superseded. M3/M4
> stay gated on M6.

> # TIER-3 BUILD — SYMPTOM INVENTORY + RESOLUTIONS (accumulating)
>
> Ruled resolutions the Architect seeds at each M6 tier-3 hard-stop. Read
> alongside AC-REPR; CV + Adversary review each respin against these.
>
> RESOLUTION — owner-key axis (Architect evt_6pft1vh77ww1z, 2026-08-24). The frame
> was silent on WHICH emission owner keys the CheckedIhCapturedEnvironment record
> while framing it as (a) captured free variables and (b) force-seam-constructed.
> RULED: for the CheckedIhCapturedEnvironment role the authorizing emission owner
> is the FORCE-emission owner (the owner that forces the worker whose closure_origin
> is the seat), NOT the containment owner. The owner in the key names which context
> EMITS the aggregate (the force seam), not the values' definition origin. The
> force-seam emitter the implementer built is CORRECT and stays; the fix is entirely
> producer-side — repoint the tier-2 CheckedIhCapturedEnvironment owner enumeration
> to the force-emission owners (walk real force edges), clone the seat's single
> canonical run byte-identical across owner keys so the tier-1 two-authorities
> cross-check (checked_ih_capture_origin) stays intact, and authorize a force owner
> ONLY for the seat it actually forces (reject a seat-only lookup fallback — that
> weakens the shared check). Prefer keying by the force owner and dropping the
> unconsumed containment-keyed records for this role, reconciled with the tier-1
> records.len()==1 fixture (its seat must be force-reachable); if a consumer needs
> the containment key, keep both additively and state which. This is a bounded
> planner-enumeration repoint — NOT a new representation route, NOT a node scope
> change, no trusted_base delta (no operator TCB gate). Returns to Architect + CV +
> Adversary. Auditability on the respin: (a) the force-owner record's run is
> byte-identical to the seat's canonical run; (b) checked_ih_capture_origin reds if
> pointed at the wrong seat's run; (c) a discriminating pair — a FORCED checked-IH
> seat gets a force-owner record while a contained-but-not-forced seat does not.
>
> Symptom #1 (Architect): the captured-env authority population keyed emission
> owners by CONTAINMENT (a proxy inherited from the inline-emitted siblings), which
> under-covers the FORCE-emission owner the frame's force-seam construction actually
> uses — the wrong axis (containment) for a force-emitted role.
>
> SHARED-CAUSE PREDICATE (§1b, Architect evt_36k7yzf27t67x, 2026-08-24 — named
> at hard-stop #2 rather than waiting for a 3rd). A synthesized-aggregate's
> Specialization-owner enumeration must be EMISSION-ACCURATE: it must produce the
> same owner set the consumer keys by (defining_emission_owner = the lowered units).
> The continuation_contexts subtree-containment proxy
> (synthesized_seat_emission_owners, static_transition.rs:589) is a valid authority
> ONLY for aggregates emitted inline
> at their own seat; for any role emitted otherwise it under-covers, and the consumer
> fail-closes at a legitimately-emitted seat. #1 (checked-IH, force-seam) and #2
> (host-result-error, emitted non-inline from a specialization body) are two
> instances of this ONE predicate.
>
> Symptom #2 (Architect): host-result-error (Constructor role) Specialization-owner
> enumeration uses the continuation_contexts subtree-containment proxy, which
> under-covers the lowered-unit emission owner the consumer keys by (seat 264 under
> Specialization(2) is genuinely lowered and emits FileOperationRead inline, but
> the proxy does not yield Specialization(2)) — same predicate as #1.
>
> SCOPE RECUT (Steward, on the Architect's §1b closure ruling). This SUPERSEDES
> the per-role "repoint the checked-IH owner axis" framing of RESOLUTION #1: the
> tier-3
> reconciliation is a STRUCTURAL CLOSURE over the shared predicate, not a second
> per-role patch. Reconcile the synthesized-aggregate Specialization-owner
> enumeration axis with the consumer's defining_emission_owner axis for ALL affected
> roles — host-result-error (Constructor) fixed via the emission-accurate
> (lowered-unit) authority checked_ih_force_emissions already uses;
> UnitBoundaryEnvironment AUDITED against the inline-at-own-seat predicate in the
> SAME pass (state with
> evidence whether it stays on the subtree proxy or takes the emission-accurate
> enumeration) to prevent the predictable hard-stop #3. Retain everything already
> proved (checked-IH force-owner records, schema 12/12, controls). Two hard
> constraints the Architect flags as the unsound-if-careless places: (i) do NOT
> clone content
> across owners for Constructor — its children/meet/allocation may differ per
> specialization (lifetime lanes); the emission-accurate enumeration fixes WHICH
> owners get records, per-owner content is still computed per owner; (ii) authorize
> tightly — a record only for owners that genuinely emit the seat (walk real
> lowered-unit emissions), never a blanket over all specializations; preserve
> validate_aggregate_producers_are_unique. STILL IN M6 (no successor node): same
> class, mechanically determined, and the closure is meant to END the chain. Bounded
> planner-enumeration reconciliation, no new representation route, no trusted_base
> delta (codegen) — no operator TCB gate. Returns to Architect + CV + Adversary.
> Auditability: a discriminating pair proving a Specialization that lowers a
> non-inline host-effect seat now GETS its record (was the refusal), and a control
> that the emission-accurate owner set equals the consumer's
> defining_emission_owner set for the fixture.
>
> CLOSED TRANSFORMATION SCHEMA (Steward, folding Architect mechanism ruling
> evt_1hvz3zt0ewage + Research advisory evt_724e1rh6t5wbg, 2026-08-24). This is
> the THIRD-hard-stop response and it SUPERSEDES this inventory's accumulating
> one-wall-at-a-time mode: at a third distinct refusal the response is to CLOSE
> the representation contract as a schema and census it up front, not to keep
> discovering walls one at a time. The three advancing refusals are three
> independent invariants of ONE defunctionalization/closure-conversion
> correctness statement — #1 owner-of-code and #2 owner-of-aggregate-enumeration
> are the
> dynamic-provenance face; #3 binder-map is the lexical-scoping face — same
> capability, distinct invariants, jointly establishing that the new representation
> denotes the original closure. Defunctionalize was NOT the wrong D0; no successor
> node; no re-decomposition.
>
> THE DEEPER §1b CAUSE shared by all three walls (Architect): the M6
> representation cross-checks independently-authored coordinates but never stated
> the FRAME each
> lives in nor the morphism joining them, so each axis surfaces as a separate
> wall the first time its cross-frame comparison is reached. Closing the schema
> = stating,
> PER AXIS, the two independent authorities AND the common frame/morphism (or
> stable identity) that joins them, and validating AFTER the map.
>
> THE SCHEMA (census all five up front; the build proves the whole schema, not
> the next occurrence). For each axis: name the two independent authorities, name
> the
> common frame + morphism (or stable identity) that joins them, compare only in
> that one frame, and reject any bare cross-frame comparison.
> 1. code/plan identity;
> 2. ordered environment fields + owner/lifetime;
> 3. source-binder -> target-binder map under every inserted context;
> 4. apply-argument arity/order;
> 5. result/interface identity + every dispatcher/apply site.
>
> TWO-AUTHORITIES DISCIPLINE this establishes (Architect, general form — durable
> beyond M6): a two-authorities cross-check MUST compare in a COMMON FRAME.
> Cross-frame comparison gives FALSE NEGATIVES (spurious rejection — this bug,
> #3); deriving one authority FROM the other gives FALSE POSITIVES (vacuity — the
> trap
> ruled against at tier-1, dec_2304df97xzbp5). Correct discipline = two
> authorities + explicit frame morphism + validate-after-map. Keep the
> independent oracle; do
> NOT collapse it.
>
> Symptom #3 / §1b INVENTORY ENTRY #3 (Architect, verified at 064133e45):
> method_binder_ordinal (plan-authored IH-subsequence ordinal,
> compiler_driver.rs:1637 = binder_ordinal - argument_count) compared by bare !=
> (lowering/mod.rs:10613)
> against binder_index (consumer runtime de Bruijn index, ALREADY mapped via
> BranchBinderRemap, erasure.rs:2302/2836/3258) — a coordinate-join across two
> frames without the morphism. `0 != 4` does NOT establish the call names the
> wrong binder; it can be the same binder after enclosing runtime binders
> (lawful
> weakening). Corroboration the frame is under-specified (not the values wrong):
> fixtures already disagree on the ordinal's frame — 0 (plan), 4
> (source_frame_bridge.rs:881/1873/5569), 1 (specialization_binding.rs:469).
> Shares the deeper predicate with #1/#2.
>
> REPAIR CLASS / MECHANISM (Architect ruling — the build input). Keep BOTH
> authorities; reconcile their FRAMES before comparing. Apply the SAME context
> morphism the consumer already used to bring the plan's ordinal into the runtime
> frame, then validate equality in that one frame. All pieces EXIST: the
> argument_count offset (compiler_driver.rs:1637, inverted to recover the
> source-frame position) + the enclosing-binder history the consumer already
> threads + BranchBinderRemap::runtime_index (erasure.rs:4200). Compose them into
> the
> explicit source->target morphism at lowering/mod.rs:10613. Do NOT relabel the
> ordinal, do NOT special-case 0->4, do NOT delete the oracle (GHC Unique / Lean
> VarId.alphaEqv precedent).
>
> DECISIVE CHECK — build STEP 1, FAIL-CLOSED (the lawful-weakening vs
> misbinding fork; held fail-closed, not assumed): for the failing occurrence,
> compose the
> morphism and compute where plan slot ordinal 0 lands in the runtime frame. Lands
> on 4 -> LAWFUL WEAKENING — the seam was joining two frames without the map;
> apply the morphism, validate after, it greens correctly. Does NOT land on 4 ->
> GENUINE
> MISBINDING — STOP and report, do NOT force-green. (Architect's prior, from
> uniform planner-authored 0 vs varying consumer indices, leans
> lawful-weakening, but the check is fail-closed.)
>
> DISPOSITION: STILL IN M6, no successor, no new D0, no trusted_base delta
> (codegen) — no operator TCB gate. Research §1a discharged (standard
> scoping-discipline
> repair class; typed closure conversion / defunctionalization; GHC Unique; Lean
> VarId + IndexRenaming; no further research round). Everything already proved
> (owner records, schema 12/12, both new controls) stands. Returns to Architect + CV
> + Adversary.

> # D0 RULED 2026-08-22 — DEFUNCTIONALIZE; re-homed as native-program Track-1
>
> The Architect ruled the D0 this node carried (materialized closure value vs
> defunctionalized carried tag) in the native-program frame (evt_9kat78d438cb):
> DEFUNCTIONALIZE — code id + env Record + finite static apply dispatcher. This
> is not a new invention; [[RT-CLOSURE-CROSSING-ELIMINATE]] (merged PR #2327)
> proved it for the source-authored closure population, and
> `spec/40-runtime/41-values.md:76-118` sanctions live-domain closure exchange,
> constraining only the durable lane.
>
> The one open discriminator the build must confirm: are ALL apply sites of a
> stored/escaped checked IH statically enumerable from the checked plan? Yes ->
> the defunctionalized carried tag is sound. A genuinely non-enumerable apply
> site is a NEW fork (materialization) that returns to the enclave + Architect,
> never a silent widen. Architect's lean, corroborated by the merged
> source-authored case: enumerability holds.
>
> This is the M6 seat (calls.rs:222, escaping functional IH as
> StaticWorkerBinding — the representation itself) and the Track-1 D0. It now
> follows Track 0 ([[RT-NATIVE-TRACK0-REARM]]) and gates the consumers M3
> ([[RT-CARRIED-IH-DISPATCH-SITEOP]]) and M4 ([[RT-CLOSURE-BOUNDARY-RESIDUAL]]).
> No longer deferred — it is the runtime lane-1 Track-1 crux. Needs a build frame
> before release. HS 5.
>
> REACH BROADER THAN FIRST FRAMED (Architect evt_4sp2xftkmc1mz, 2026-08-22): this
> D0 is the WHOLE remaining PX8-closure critical path. Track-0 measurement showed
> every native checked-IO full program carries the checked continuation
> closure, so this representation gates ALL FOUR native witnesses
> (ReadEof/ReadSome/Wrote
> AND SemanticErrorV1), not just the positioned one — discharging M3+M4+M6 at
> their three seats across every native checked-IO program.
>
> TRACK-1 ENTRY CRITERION (bounds the D0, from the Architect): confirm the
> checked-write closure is the checked continuation — statically enumerable apply
> sites, matching CROSSING-ELIMINATE's defunctionalization precondition — NOT a
> genuinely non-enumerable source closure. Lean (merged-state logic): it is the
> checked continuation, defunctionalizable. A genuinely non-enumerable apply site
> is the Architect's return-fork condition (back to Architect + enclave), never a
> silent widen.

# WHAT THIS NODE IS

> DEFERRED (draft). This is a genuine new-representation deliverable, not part of
> the RT-FSREADAT co-land set. The co-land (join half `6a45ae1a7` + producer fix
> `64019430c` + projection/disposition/join-reconcile + the advancing-refusal
> re-point) lands WITHOUT this node; the two checked-family programs stay a
> documented advancing refusal until this representation lands. Its sequencing
> into a lane is a priority call surfaced to the operator -- nothing in the
> co-land, the NHC close, or the RT-BACKEND-MODULE-SPLIT pivot waits on it.
>
> It is NOT startable as framed: it is gated on the Architect's design D0 below
> (which representation). The frame is a skeleton; the deliverable and its ACs
> are fixed once D0 is ruled.

Ken can compile a functional induction hypothesis only when the compiler can
prove the IH does not escape -- then every use is a direct in-frame application
and no value need exist. The two checked-family programs violate that: the
realized IH value is stored into a constructor (`ITree.Vis`), so it escapes, and
Ken has no object that can carry `lambda response. rec (k response)` across that
boundary. This node builds that object.

# THE CAPABILITY GAP (measured, grounded)

- Decisive measurement (runtime-implementer evt_79jd1nxamqd95, on `64019430c`):
  the force marker's `parent_chain` immediate parent is `Expression(Construct)` on
  BOTH checked-family programs -- the realized value is stored straight into a
  constructor argument position. Two marker populations exist; the failing one is
  the single `template=0, arity=0` nullary_force per program (`core.rs:11699`),
  distinct from the 12 fine `template=4, arity=1` markers (`source.rs:655`).
- `core.rs:11699` lowers the marker body as an ordinary value-producing
  expression, so `Call(Var(0), args=[])` resolves `Var(0)` to the arity-1
  StaticWorker and calls it with zero args; the seam passes (0==0), the refusal
  is the worker gate at `calls.rs:220`. Confirmed independently by the
  Architect
  (evt_5wvex36s7nm6d).
- The non-escaping remedy (use-site specialization with the actual response) is
  inapplicable HERE: at this use there is no response argument and no in-frame
  application site -- the result is immediately stored into a constructor.
- No existing Ken object represents the value: `StaticWorkerBinding` is compiler
  metadata (`lowering/mod.rs:3578-3603`, "the binding never becomes a value");
  `LoweringOperand` = `{ Specialized(Lowered), Carried(CarriedBoundaryWord) }` with
  no closure/worker arm and `worker.captures` itself `Vec<LoweringOperand>`; a
  carried word cannot hold the closure. Naming it thunk/capture/self-register does
  not make the live worker a `LoweringOperand`.

# DESIGN D0 (Architect rules before build) -- which representation

Prior art (Architect + research advisory evt_5jp0npxf78erv) admits two viable
shapes; neither is cheaper by naming:

- **(a) Materialized closure value.** A first-class closure object packaging code
  + environment (typed closure conversion, Minamide/Morrisett/Harper POPL 1996;
  Lean IR `pap`/`ap`/`fap`). A new value representation and its
  construction/application sites.
- **(b) Defunctionalized carried tag.** A first-order tag carrying code identity +
  environment plus an apply dispatcher (Reynolds; Danvy/Nielsen). Fits Ken's
  carried-word machinery more closely, at the cost of an apply dispatcher.

The choice, its soundness argument, and the boundary discipline are the
Architect's design ruling (the `any -> Architect` edge). This node does not
prejudge it.

# DELIVERABLE (fixed once D0 is ruled)

The chosen representation, its construction at the nullary_force seam
(`core.rs:11699`), and its application at the escaped use sites, such that the
two checked-family programs' checked-IH invocations lower and run correctly.
Likely a language spillover on the marker/planner side (the elaborator's
nullary_force emission, `compiler_driver.rs:1718-1743`) -- assigned by this
node's owner at framing per COORDINATION section 9a.

# ACCEPTANCE (terminal -- inherited from the closed predecessor)

- **AC-REPR.** The realized functional IH is represented by the D0-chosen object;
  the arity-1 worker is never called with zero args; the escaped value is
  constructed and applied honestly across the constructor boundary; no closure
  boundary is crossed illegitimately and no arity gate is weakened or relabeled.
  Off-roster-identity enforcement is TWO-TIER (Architect ruling
  evt_3k0bv5px3m097, refining the earlier single-match fold-in which was
  unbuildable — Rust exhaustiveness is over TYPES, not over the u64 template ids):
  - **KIND (compile-time, type-structural):** the lowering dispatch is a match
    over the SEALED checked-IH code-identity shape set (the checked-IH
    RuntimeExpr family) with NO `_`/fallback arm, so a new shape is a
    ken-runtime build error
    (the ABI-R3 next_in_inventory discipline applied where it CAN live — over a
    sealed type; COORDINATION section 7 exhaustive-by-construction).
  - **INSTANCE (lowering-time, fail-closed):** the template id is resolved
    through the EXISTING plan lookup (`computational_ih_call` /
    `computational_ih_slot`,
    `lowering/mod.rs` ~10576-10592), so an id the plan does not hold is refused
    fail-closed — the SAME point `erasure.rs`'s double bijection is asserted,
    so the roster and the bijection are one fact with no parallel list to
    drift.
  - **VALUE:** the escaped IH crosses as the admitted environment `Record` only,
    with NO runtime code-identity tag.
  Every apply and slot site MUST resolve its id through the plan lookup — a
  single site that reads `call_template_id` and uses it directly reopens the
  off-roster
  hole. This delivers the property the earlier fold-in protected (no off-roster
  identity silently admitted) with no drift surface. The Adversary hunts
  over-accept shapes: a boundary crossing dressed as the representation, a
  relabeled/weakened arity gate, a `_`/fallback arm on the kind match, or an apply
  site that bypasses the plan lookup.
- **AC-REENUM (terminal; was shared with [[RT-IH-MARKER-PRODUCER-COMPLETE]]).**
  Rerun report-2's checked-family runner end-to-end. Both checked-family programs
  green ⇒ the family is bounded; re-point both runner tables from the
  advancing-refusal pins (set by the co-land) to green. Any FURTHER refusal ⇒ STOP
  and report to Architect + Steward. As of the third-hard-stop ruling, the census
  is the CLOSED TRANSFORMATION SCHEMA (five axes, TIER-3 block above): the build
  proves the whole schema up front, not the next occurrence, so a further refusal
  on an un-censused axis is a schema-closure miss, not a new capability gap.
- **AC-NO-REGRESSION.** Whole-suite green in CI (COORDINATION section 12). Local
  targeted `-p` only, never `--workspace`.
- **Required reviewer.** Architect -- design D0 ruling AND soundness review of the
  built representation. The Adversary hunts over-accept shapes (a boundary
  crossing dressed as the new representation; a relabeled arity; a `_`/fallback
  arm on the kind match; an apply site bypassing the plan lookup).

# DISPATCHER-DESIGN ITERATIONS (in-thread rulings made durable)

Iteration 1 (Architect ruling evt_3k0bv5px3m097, 2026-08-22). Deliverable-1
enumerability was discharged (runtime-implementer evt_7k1nnw3g8rejn; enumerability
HOLDS, no return-fork; Architect verified independently via the double bijection).
The implementer then flagged, rather than silently resolving, that AC-REPR's
original single-match fold-in was UNBUILDABLE as an instance-level mechanism: Rust
exhaustiveness is over types, the template ids are u64 plan data, and the only way
to fake a no-wildcard match over them — an enum mirroring the plan's current
ids — is the hand-maintained parallel roster the Architect had warned against
(and it
still needs a u64->enum fallback, so it collapses to a runtime check that can
drift). Keyed on: mechanism specified over the wrong DOMAIN (runtime ids vs sealed
types). Refined to the KIND-exhaustive + INSTANCE-fail-closed + VALUE-env-only
two-tier form now in AC-REPR. A strengthening, not a defect — the design is ruled
and stands; HS unchanged at 5 (iteration 1, nowhere near the research trigger).

# BUILT-REPRESENTATION SOUNDNESS REVIEW (Architect verifies on handback)

Not new gates — the criteria the Architect's soundness review checks; build toward
them:
- a. No `_`/fallback arm on the kind match — a genuinely sealed type, so a new
  checked-IH shape is a compile error.
- b. EVERY apply and slot site resolves the id THROUGH the plan lookup — no site
  reads `call_template_id` and uses it directly. A single bypass reopens the
  off-roster hole; universal lookup is what makes the fail-closed resolution total
  over the population.
- c. The crossing value carries no code-identity tag — env `Record` only.
- d. Arity seated honestly through the existing `supplied == call.arity` gate: the
  arity-1 worker is never called with zero args, the template arity is untouched
  (the two-arity Closure-A unsound middle stays barred).

# SEQUENCING / CONTENTION / CAPABILITY TIER

Deferred; not in a lane until the operator sequences it. Depends on the co-land
base (producer fix `64019430c` + join half `6a45ae1a7` on `main`). `ken-runtime`
lowering plus a likely `compiler_driver` (language) spillover -- contention with
those crates is possible, so it is not co-runnable with other runtime/language
work on the same files; the Steward sequences it to avoid that.

Tier T1: novel representation design and a soundness-bearing lowering change,
reviewed on the argument, not a differential diff. Size L.
