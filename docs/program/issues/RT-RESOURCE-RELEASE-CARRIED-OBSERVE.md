---
id: RT-RESOURCE-RELEASE-CARRIED-OBSERVE
title: "Genuinely-live effect seats needing ResourceScalar cannot observe their need in the CarriedWord phase, so the withResource path fails object emission -- the (A)-family carried-observation CLOSURE over the ResourceScalar need (ResourceRelease/FsHandleMetadata/FsReadAt Argument(0)): observe the need in the carried phase on the lower_buffer_freeze_resource_seat EITHER_PHASE precedent, keyed (need=ResourceScalar, phase=Carried), WITHOUT widening the seat's direct Need-subset-Avail partition"
status: merged
owner: runtime
size: M
gate: none
depends_on: []
blocks: [NATIVE-HANDLE-CARRIER]
github: null
origin: "Architect ruling evt_4hcny7ae7h9sb (thr_3r6wv5net6s61, 2026-08-22), the RT-DEAD-ARM-EFFECT-LOWERING D1 hard-stop, Finding 2. Surfaced by the runtime-implementer measuring past the dead arms (evt_6wtfb4p5jxhk1, scratch/reverted): behind the dead FSOp arms the cap41_* fixtures hit a genuinely LIVE refusal -- seat Argument(0) of ResourceRelease needs ResourceScalar, which it cannot observe in CarriedWord -- and the program DOES use withResource. The Architect ruled this the concrete FIRST instance of the (A) work deferred at evt_7kmh9atsrv80n, forced onto the critical path. Steward-filed per COORDINATION section 2."
---

# MERGED — 2026-08-22

Landed at `ef32b6ced` (candidate `ef99c319a`, base `69df7e775`), Decision
`dec_3m2p4tmgnpa9t` resolved APPROVE. Both deliverables delivered: D0 grounded
the `ResourceScalar` carried representation (runtime-valued resource-token
handle, observed via the borrowed-opaque guards — no boundary->wire translation,
no constant-vs-varying Spec contract question arose, so AC-3's no-contract branch
holds), D1 built the route.

- **runtime-qa APPROVE** (`evt_6e1kf4tdghchs`), independently reproduced at
  `ef99c319a`; two-layer admission independence mutation-proven (loosening the
  route key reddened via the ledger's second, independently re-derived
  admissibility check, a distinct error string).
- **Architect required-reviewer APPROVE** (`evt_24nyqqhs5fy1f`), full-diff read,
  all nine soundness properties verified by direct read: fail-closed consumer
  (both guards dominate the scalar read), exact complement of `Direct`, no `Avail`
  change, independent second authority, non-degenerate discriminator (one witness,
  two assertions moving in opposite directions), the borrowed-opaque nuance
  grounded and not overclaimed, single authority (`lower_resource_token_seat`),
  positioned-arm completeness, sentinels repointed strictly downstream. Approval
  binds exact `ef99c319a`; a `crates/ken-runtime` fold requires differential
  re-review.
- **AC-1 met, all three ResourceScalar seats** (ResourceRelease/FsHandleMetadata/
  FsReadAt Arg(0)) advance past the refusal; the rows TERMINATE at the
  `ExactIntU64` sibling (FsReadAt Arg(1)), scoped OUT — cut as the successor
  [[RT-EXACTINT-CARRIED-OBSERVE]]. That successor, not remaining work of this
  node, is what NHC's `D-final` now waits on.
- Gates 1142/0 (`-p ken-runtime` all binaries + `-p ken-cli` + `-p ken-verify`).
- **M8 Adversary hunt: SOUNDNESS CLEAN** (`evt_5wx3bax63yak`, landed blob
  `8c4009ea` == approved `ef99c319a` blob, byte-identical; approval transfers by
  blob since the squash makes `ef99c319a` a non-ancestor). Verified the new route
  is fail-closed by construction: every carried resource-scalar read re-runs
  `lower_resource_token_seat`, whose `require_i64` emits `return -1` (runtime
  failure) on a tag/class mismatch BEFORE the scalar load. Gate-weaker-than-
  consumer, sound because the accept path re-runs the fail-closed authority.

## POST-MERGE COMPLETENESS OBSERVATION (Adversary, safe-direction, non-blocking)

The Adversary flagged one completeness gap for the runtime ring to triage
(NOT a soundness defect, NOT a reopen): FsReadAt's `Argument(2)` BUFFER seat has
two readers. AC-1b converted the REQUEST path (`effects.rs:2477`) to
`lower_resource_token_seat` (handles carried), but the REPLY/span-provenance path
(`effects.rs:3226`) still reads `seats.specialized(SEAT_2)?` and refuses a carried
buffer ("FsReadAt buffer operand is not a resource"). So a carried FsReadAt buffer
is admitted at request, then re-refused at reply -- the same "moving the refusal
rather than closing it" shape AC-1b addressed, on a fourth reader not
cross-referenced. Direction is SAFE (a pattern-match guard -> clean refusal, never
a scalar misread -> no miscompile), and it is OFF the current `cap41_*` critical
path (the rows hit the `ExactIntU64` terminal at Arg(1) first). The Adversary also
notes `3226`'s destructured `span_origin` is unused (the constructor projects via
`site_operand_argument` at `3233`), so that specialized-only match is a vestigial
gate whose only post-D1 effect is rejecting carried buffers.

**Disposition: DEFERRED by design, and the fix is REMOVAL not reroute**
(runtime-implementer `evt_1rz7rnphp9ndw` deferred it; Architect
`evt_2qdpkfvtqrxzy` corrected the mechanism, both confirmed at `aa0178eed`). The
`3226` `span_origin` binding is UNUSED -- the constructor projects the span from
`site_operand_argument(.., 2, ..)` at `3233`, not the destructured payload -- so
the gate is vestigial and rerouting it through `lower_resource_token_seat` would
add a guarded read whose scalar result is discarded (a dead read). The correct
fix is to REMOVE the vestigial gate. That removal carries its own small
soundness/completeness check, so it is a distinct ResourceScalar-family successor,
not a silent fold. Routed into [[RT-EXACTINT-CARRIED-OBSERVE]]'s `D0` as a
side-classification (CLASSIFY, then the Steward cuts the tiny removal successor if
clean, or names a design successor if not) -- kept distinct in the accounting from
that node's ExactIntU64 need. Safe-direction and off the critical path; does not
gate NHC.

The below is the node as framed, retained for its measurements.

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
