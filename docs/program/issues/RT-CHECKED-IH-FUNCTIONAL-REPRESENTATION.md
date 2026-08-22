---
id: RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION
title: "Give Ken a first-class representation for an ESCAPING functional induction hypothesis, so the nullary_force of a checked computational IH whose realized value is stored into a constructor (escapes its frame) can be lowered honestly. The decisive escape measurement (runtime-implementer evt_79jd1nxamqd95) proved the realized IH value's immediate parent is a Construct on both checked-family programs -- it is stored straight into a constructor field, so the non-escaping use-site-specialization remedy cannot apply (no in-frame application site, no response argument at the use). Ken has no first-class functional-IH value: StaticWorkerBinding is compiler metadata with no runtime word/tag/layout/env-pointer/callable identity (lowering/mod.rs:3578-3603), LoweringOperand is exactly {Specialized(Lowered), Carried(CarriedBoundaryWord)} with no closure/worker arm, and an ordinary carried word cannot hold `lambda response. rec (k response)`. This node introduces the genuine new representation. It carries a design D0 the Architect rules before build: materialized closure value vs defunctionalized carried tag (code identity + environment + apply dispatcher). Successor to the closed RT-CHECKED-IH-RECURSIVE-OPERAND-SEAT."
status: active
owner: runtime
size: L
gate: none
depends_on: [RT-IH-MARKER-PRODUCER-COMPLETE, RT-NATIVE-TRACK0-REARM]
blocks: [RT-CARRIED-IH-DISPATCH-SITEOP, RT-CLOSURE-BOUNDARY-RESIDUAL]
github: null
origin: "Cut by the Steward (scoping ruling evt_5pmk273zg5paa) on the Architect's pre-committed conditional ruling (evt_2f4bbmt7qfde1) after the decisive ESCAPE measurement (runtime-implementer evt_79jd1nxamqd95) returned ESCAPING. Supersedes RT-CHECKED-IH-RECURSIVE-OPERAND-SEAT (closed; both its Closure-A operand-seat mechanism and its NULLARY_FORCE re-reading were refuted -- the realized IH value escapes into a constructor, a capability gap, not a seam fix). HS=5 (Steward of record); ESCAPING is a scoping decision, not a hard-stop increment. Steward-filed per COORDINATION section 2."
---

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
> every native checked-IO full program carries the checked continuation closure,
> so this representation gates ALL FOUR native witnesses (ReadEof/ReadSome/Wrote
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
  StaticWorker and calls it with zero args; the seam passes (0==0), the refusal is
  the worker gate at `calls.rs:220`. Confirmed independently by the Architect
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
    over the SEALED checked-IH code-identity shape set (the checked-IH RuntimeExpr
    family) with NO `_`/fallback arm, so a new shape is a ken-runtime build error
    (the ABI-R3 next_in_inventory discipline applied where it CAN live — over a
    sealed type; COORDINATION section 7 exhaustive-by-construction).
  - **INSTANCE (lowering-time, fail-closed):** the template id is resolved through
    the EXISTING plan lookup (`computational_ih_call` / `computational_ih_slot`,
    `lowering/mod.rs` ~10576-10592), so an id the plan does not hold is refused
    fail-closed — the SAME point `erasure.rs`'s double bijection is asserted, so
    the roster and the bijection are one fact with no parallel list to drift.
  - **VALUE:** the escaped IH crosses as the admitted environment `Record` only,
    with NO runtime code-identity tag.
  Every apply and slot site MUST resolve its id through the plan lookup — a single
  site that reads `call_template_id` and uses it directly reopens the off-roster
  hole. This delivers the property the earlier fold-in protected (no off-roster
  identity silently admitted) with no drift surface. The Adversary hunts
  over-accept shapes: a boundary crossing dressed as the representation, a
  relabeled/weakened arity gate, a `_`/fallback arm on the kind match, or an apply
  site that bypasses the plan lookup.
- **AC-REENUM (terminal; was shared with [[RT-IH-MARKER-PRODUCER-COMPLETE]]).**
  Rerun report-2's checked-family runner end-to-end. Both checked-family programs
  green ⇒ the family is bounded; re-point both runner tables from the
  advancing-refusal pins (set by the co-land) to green. Any FURTHER refusal ⇒ STOP
  and report to Architect + Steward.
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
to fake a no-wildcard match over them — an enum mirroring the plan's current ids —
is the hand-maintained parallel roster the Architect had warned against (and it
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
