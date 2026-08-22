---
id: RT-RESOURCE-RELEASE-CARRIED-OBSERVE
title: "Genuinely-live effect seats needing ResourceScalar cannot observe their need in the CarriedWord phase, so the withResource path fails object emission -- the (A)-family carried-observation CLOSURE over the ResourceScalar need (ResourceRelease/FsHandleMetadata/FsReadAt Argument(0)): observe the need in the carried phase on the lower_buffer_freeze_resource_seat EITHER_PHASE precedent, keyed (need=ResourceScalar, phase=Carried), WITHOUT widening the seat's direct Need-subset-Avail partition"
status: active
owner: runtime
size: M
gate: none
depends_on: []
blocks: [NATIVE-HANDLE-CARRIER]
github: null
origin: "Architect ruling evt_4hcny7ae7h9sb (thr_3r6wv5net6s61, 2026-08-22), the RT-DEAD-ARM-EFFECT-LOWERING D1 hard-stop, Finding 2. Surfaced by the runtime-implementer measuring past the dead arms (evt_6wtfb4p5jxhk1, scratch/reverted): behind the dead FSOp arms the cap41_* fixtures hit a genuinely LIVE refusal -- seat Argument(0) of ResourceRelease needs ResourceScalar, which it cannot observe in CarriedWord -- and the program DOES use withResource. The Architect ruled this the concrete FIRST instance of the (A) work deferred at evt_7kmh9atsrv80n, forced onto the critical path. Steward-filed per COORDINATION section 2."
---

# WHAT THIS NODE IS

The (A)-family carried-observation CLOSURE over the `ResourceScalar` need. A
genuinely-LIVE effect seat needing `ResourceScalar` -- `ResourceRelease`
`Argument(0)`, and the two structural siblings `FsHandleMetadata` `Argument(0)`
and `FsReadAt` `Argument(0)` -- cannot observe its need in the `CarriedWord`
phase, so the `withResource` path fails object emission. Unlike the sibling
[[RT-DEAD-ARM-EFFECT-LOWERING]] fixture (a dead arm the program never enters),
this arm is REACHED: the program uses `withResource`. So the fix is not a trap;
it is a real lowering route that observes the need in the carried phase.

## SCOPE WIDENED TO THE ResourceScalar FAMILY (D1 measurement + Steward ruling)

Framed as the ResourceRelease instance; widened at D1 to the ResourceScalar-need
CLOSURE. The runtime-implementer measured (`evt_2drwk6kh3d9xv`) that guard
uniformity is STRUCTURAL, not sampled: a `Lowered`'s boundary tag/class is chosen
by one `match` on its `LoweredVariant` (`boundary.rs`), with no consuming
operation in scope, so the tag is a function of the value's variant, not of the
seat that later reads it. The Architect ruled the KEY width theirs and widened it
to `(need=ResourceScalar, phase=Carried)` (`evt_3dnd21pjg193g`), rejecting the
enumerate-one-operation-at-a-time shape as an unbounded chain of near-identical
rulings for one predicate; the Architect ruled the node-scope call the Steward's.

**Steward ruling (`evt_5xq3hw23kamrd`): FAMILY CLOSURE, do not narrow.** One
structural predicate is one node (subsume-don't-proliferate); three near-identical
nodes for one guard is the proliferation `docs/PRINCIPLES.md` forbids. The
boundary is the NEED: this node covers `ResourceScalar` and terminates at the
`ExactIntU64` sibling, which is a different need with its own `carried_exact_int`
precedent -- correctly out (below).

**Still the (A) family only, not a general carried-need closure.** The
ConstructorTag/FsWriteFile (A) instance stays DEFERRED (its arm is dead in the
current fixtures, handled by [[RT-DEAD-ARM-EFFECT-LOWERING]]); do not fold it in
here.

# THE DEFECT, AS MEASURED

Measured by the runtime-implementer while temporarily extending the dead-arm
gate past the dead arms (`evt_6wtfb4p5jxhk1`, scratch, reverted; grounded at
`4ed363bef`).

- **The refusal:** `seat Argument(0) of ResourceRelease needs ResourceScalar,
  which it cannot observe in CarriedWord` -- the same `claim_host_effect_seat`
  membership shape as the dead-arm node (`cranelift_backend/lowering/
  effects.rs:227`, refusal `:277`), but on a DIFFERENT seat/need and on a LIVE
  path.
- **The arm is LIVE.** The `cap41_*` fixtures genuinely use `withResource`, so
  the `ResourceRelease` arm is reached at runtime. A trap is therefore WRONG
  here -- trapping a live arm breaks a working program (exactly the regression
  the dead-arm node's conservative oracle exists to prevent).
- **It is on the fixtures' critical path.** Behind the dead `FSOp` arms
  ([[RT-DEAD-ARM-EFFECT-LOWERING]] unblocks their emission), this is the next
  distinct blocker the `cap41_*` rows advance to. So the dead-arm node alone
  greens no row; both nodes must land for NATIVE-HANDLE-CARRIER's `D-final` to
  go all-green.

# THE RULED DESIGN FAMILY (A) -- Architect `evt_7kmh9atsrv80n` + `evt_4hcny7ae7h9sb`

A carried-observation lowering route: observe the need in the CARRIED phase
WITHOUT widening the seat's direct `Need`-subset-`Avail` partition. The gate
stays real; a new route PROVES observability, it does not relax membership (the
soundness line the D5 / RT-CARRIER-BYTESPAN work held).

- **The precedent is `lower_buffer_freeze_resource_seat`** (`effects.rs:1612`),
  an `EITHER_PHASE` carried tag-observation route that is NOT site-operand-keyed.
  It is the right shape for this seat. It is NOT the byte-span
  `SiteOperandProjection` route (that keys on `host_effect_site_operand_slots` /
  `BytesPointerLength` and is the wrong key here, as measured on the sibling).
- **No `Avail` change, seat stays strict.** This node does not touch the seat's
  `Need`-subset-`Avail` partition, `create_policy_tag`, or any dead-arm census.
- **Not a kernel / TCB edit.** This is cranelift-backend lowering policy.

**The specific mechanism for `ResourceScalar` is the Architect's to rule** (they
are the required reviewer), grounded by `D0`. The (A) description at
`evt_7kmh9atsrv80n` was written for the CreatePolicy/`ConstructorTag` instance
and additionally needed a runtime tag-namespace translation (boundary tag id ->
wire 0/1/2); whether the `ResourceScalar` need requires an analogous translation
or is already runtime-valued in both phases (like `wire_bytes_seat`) is a `D0`
measurement, not an assumption to carry in.

# `D0` -- GROUND THE ResourceScalar MECHANISM (first deliverable)

Measure and report, so the Architect rules the specific route:

1. **`ResourceScalar`'s carried representation.** Is the `ResourceScalar` need
   physically carried/observable in the `CarriedWord` phase (like the byte-span
   tag), or absent? Does the seat's `Avail` admit an `EITHER_PHASE` observation
   on the `lower_buffer_freeze_resource_seat` precedent?
2. **Runtime-valued vs statically-determined.** Is the `ResourceScalar` operand
   already runtime-valued in both phases (no tag-namespace translation needed,
   like `wire_bytes_seat`), or does it need a boundary->wire translation like
   CreatePolicy did?
3. **The bounded Spec contract question, IF the operand is genuinely
   runtime-varying.** Does Ken's `withResource` / `ResourceRelease` host-op
   contract PERMIT a runtime, non-statically-determined `ResourceScalar`? If the
   contract requires compile-time constancy, a genuinely-varying value is a
   SOURCE-LEVEL error to diagnose, not a lowering route to build. This is the
   same pre-cleared lane-1-input pattern as the CreatePolicy question
   (`evt_23ybpwnhnjy8j`); route it through the Steward as a bounded Spec input,
   NOT a lane-2 reopen. Report whether the fixtures' use is constant or varying
   first -- the constant case needs no contract question at all.

`D0` determines the node's final SIZE (currently estimated `M`) and hands the
Architect the input to rule the specific mechanism.

# `D1` -- THE ROUTE

Build the carried-observation route the Architect rules on `D0`, on the
`lower_buffer_freeze_resource_seat` `EITHER_PHASE` precedent, so the LIVE
`ResourceRelease` seat observes `ResourceScalar` in the carried phase. No `Avail`
widening; the seat's membership test stays strict.

# ACCEPTANCE

- **AC-1 (the live seats are claimable).** No `ResourceScalar` seat in the
  family -- `ResourceRelease` `Argument(0)`, `FsHandleMetadata` `Argument(0)`,
  `FsReadAt` `Argument(0)` -- refuses on the `withResource` path; the `cap41_*`
  rows advance past all three. Report the full per-row, per-seat disposition;
  the expected terminal is the `ExactIntU64` sibling (`FsReadAt` `Argument(1)`),
  a distinct need scoped OUT of this node. A further distinct blocker exposed
  behind the terminal is a measurement to report and cut, not a failure here.
- **AC-1b (positioned-resource arm folded in).** The positioned-resource arm
  (`FsReadAt` / `FsWriteAt`) resource reads (file, buffer, span origin) that were
  specialized-only now go through the same shared guarded carried observation.
  With the widened key those seats can be claimed carried, so a specialized-only
  read would leave the claim ADMITTED and the read REFUSING -- moving the refusal
  rather than closing it. State that no such moved-refusal remains.
- **AC-2 (observability, not relaxation).** The route PROVES the need is
  observable in the carried phase; it does NOT widen the seat's direct
  `Need`-subset-`Avail` partition. State the soundness argument: the gate stays
  real, membership is unchanged, and no other seat's claim is relaxed.
- **AC-3 (contract-grounded).** If `D0` finds the operand genuinely
  runtime-varying, the Spec contract answer is recorded and the route is only
  built if the contract permits it; otherwise the varying case is diagnosed as a
  source-level error, not lowered. If constant, state that no contract question
  arises.
- **AC-4 (no regression).** All currently-compiling lowering is preserved;
  workspace-green in CI. (Local: targeted `-p` only, never `--workspace`.)
- **Required reviewer:** the Architect is the required reviewer on this node's
  merge Decision (soundness-sensitive carried-observation lowering) and rules
  the specific `ResourceScalar` mechanism on `D0`. Adversary hunts the landed
  code.

# EXPLICITLY NOT IN SCOPE

- **The ConstructorTag/FsWriteFile (A) instance** -- still deferred; its arm is
  dead in the current fixtures ([[RT-DEAD-ARM-EFFECT-LOWERING]] handles it).
- **Any dead-arm census / trap work** -- that is the sibling node.
- **The `ExactIntU64` sibling** (`FsReadAt` `Argument(1)`, which cannot observe
  `ExactIntU64` in `CarriedWord`). A DIFFERENT need with its own existing carried
  precedent (`carried_exact_int`, `EITHER_PHASE`, already used by
  `BufferAllocate` 0). It is the measured terminal of this node's rows and is its
  own future cut if/when lane-1 reaches it -- NOT authorized by this node. Leave
  it refusing at the terminal.
- **Any `Avail` partition change** or a general (A)-family closure over every
  carried need. The closure here is bounded to the `ResourceScalar` NEED (keyed
  `(need=ResourceScalar, phase=Carried)`), scoped to the withResource fixtures
  (Architect section 1b family predicate: the REACHABLE, runtime-varying case,
  which is (A)). `Avail` stays byte-untouched.
- **Any kernel / TCB edit.**

# CONTENTION

`ken-runtime` cranelift backend lowering (`effects.rs`). CONTENDS with
[[RT-DEAD-ARM-EFFECT-LOWERING]] on `effects.rs`, so it is SEQUENCED AFTER it in
the runtime ring (single lane, one ring): the dead-arm node advances the
`cap41_*` rows TO this blocker; this node clears it. Not released while the
dead-arm node is in flight. NATIVE-HANDLE-CARRIER is held on BOTH.

# CAPABILITY TIER

T1-demanding on the soundness reasoning (a carried-observation route that proves
observability without relaxing membership; a possible bounded Spec contract
question), with the design FAMILY front-loaded by the Architect ruling and the
specific mechanism ruled by the Architect on `D0`. The two review gates are the
safety net: the Architect as required reviewer, and the Adversary hunting the
landed route. The runtime ring delivered comparable depth at its current tier on
RT-CAPTURE and RT-DEAD-ARM's D0/D1. Steward runs the kick-time seat check;
escalate only if the seat's live model reads mechanical-only.
