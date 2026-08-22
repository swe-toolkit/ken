---
id: RT-EXACTINT-CARRIED-OBSERVE
title: "A genuinely-live effect seat needing ExactIntU64 (FsReadAt Argument(1)) cannot observe its need in the CarriedWord phase, so the withResource path still fails object emission behind the now-closed ResourceScalar family -- the ExactIntU64-need carried-observation closure on the existing carried_exact_int EITHER_PHASE precedent"
status: active
owner: runtime
size: S
gate: none
depends_on: []
blocks: [NATIVE-HANDLE-CARRIER]
github: null
origin: "Measured terminal of [[RT-RESOURCE-RELEASE-CARRIED-OBSERVE]] AC-1 (runtime-implementer evt_e68jv2mssxnd, runtime-qa evt_6e1kf4tdghchs, Architect evt_24nyqqhs5fy1f, 2026-08-22): with the ResourceScalar family closed, the cap41_* rows advance to `seat Argument(1) of FsReadAt needs ExactIntU64, which it cannot observe in CarriedWord`. The Architect scoped this OUT of the ResourceScalar node as a distinct need with its own precedent; the Steward cut it as the next lane-1 successor (evt_5xq3hw23kamrd). Steward-filed per COORDINATION section 2."
---

# D0/D1 RESOLVED — 2026-08-22 (Architect `evt_2kspreq08s3a`; in review)

D0 corrected the framing below in two ways, and the Architect ruled the
mechanism; D1 is built and the merge gate is running.

- **It is an Avail-move, NOT a new route.** `carried_exact_int` is an `Avail`
  classification (`EITHER_PHASE`), not an `EffectSeatClaimRoute` -- the framing
  below (and RT-RESOURCE-RELEASE's route-not-Avail precedent) suggested a route.
  The Architect ruled a DELIBERATE Avail-move of the positioned exact-`Int`
  seats onto the existing `carried_exact_int`, decoded carried via the already-
  in-production `narrow_carried_int_u64`. This does NOT contradict the
  route-not-Avail ruling: that was need-specific (the ResourceScalar carried arm
  would read any word's bits as a scalar, so a fail-closed guard had to dominate
  the read). Here the decoder is ITSELF fail-closed-with-validity -- it branches
  on the boundary tag, `require_i64`s the viewed path, and an out-of-range value
  returns `valid=0` into the operation's existing narrow-failure lane
  (InvalidBounds/InvalidOffset), a lawful outcome, not a trap or misread. The
  accept path re-runs the fail-closed consumer (the decoder), so no route guard
  is needed. Subsume-don't-proliferate: `carried_exact_int` already IS the
  `EITHER_PHASE` mechanism for this need (`BufferAllocate` `0` uses it).
- **Scope: the six-seat positioned-arm unit (Steward node-scope call).** The
  trap-preserved live census is `FsReadAt` `Arg(1)/3/4` (D0's initial "one seat"
  was truncated by the walk aborting at the first refusal). `FsWriteAt` `1/3/4`
  share the same emitter arm and the same one-decoder reader
  (`narrow_positioned_int_seat`), are inert-but-correct on this witness
  (`EITHER_PHASE` still admits the specialized phase, the reader is total over
  both), and move as one unit -- splitting would leave `FsWriteAt` on `exact_int`
  within the same arm, the proliferation shape. `FsChangeMode` `Arg(1)` is
  `dead_arm=true` (trapped, not wired). `BufferFreeze` `1/2` are the other
  emitter arm, deferred pure wiring. The Architect confirmed the mechanism is
  uniform across the positioned arm, so the six-seat authorization is the
  Steward's and is GIVEN.
- **The `Arg(2)` reply-path removal is spun out.** The D0 side-classification
  below is now its own node -- [[RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL]] (cut
  `ready`, held on this node) -- because closing `ExactIntU64` moved it onto the
  `cap41_*` critical path. It is NOT folded here; see that node.

The rest of this frame is the original D0-first framing, retained for its
measurements. The MERGED banner will record the final landing.

# WHAT THIS NODE IS

The next blocker on the `cap41_*` critical path after
[[RT-RESOURCE-RELEASE-CARRIED-OBSERVE]] closed the `ResourceScalar` family. A
genuinely-LIVE effect seat -- `FsReadAt` `Argument(1)` needs `ExactIntU64` --
cannot observe its need in the `CarriedWord` phase, so the `withResource` path
still fails object emission. Same `claim_host_effect_seat` membership shape as
the two predecessors, on a DIFFERENT need (`ExactIntU64`, a scalar integer) and
on a LIVE path (the `cap41_*` fixtures reach it).

**This is a carried-observation closure over the `ExactIntU64` need**, framed the
same way the ResourceScalar node was ruled: one structural predicate, one node,
not a per-operation chain. D0 enumerates the `ExactIntU64` seats and the closure
covers the need, not a single operation. The boundary is the need: it does NOT
touch the `ResourceScalar` family (merged) or any other need.

# WHY THIS IS SMALLER THAN THE ResourceScalar NODE

The route already exists. `carried_exact_int` is an `EITHER_PHASE` carried
observation ALREADY IN PRODUCTION -- `BufferAllocate` `0` reads through it
(runtime-implementer evt_2drwk6kh3d9xv, Architect evt_24nyqqhs5fy1f). So unlike
the ResourceScalar node -- which had to generalize
`lower_buffer_freeze_resource_seat` into `lower_resource_token_seat` and add a new
`EffectSeatClaimRoute::CarriedResourceObservation` -- this node most likely WIRES
the `ExactIntU64` seats to the existing `carried_exact_int` route rather than
inventing one. D0 confirms whether that is exactly true or whether a seat needs
more; the size is S on that basis and D0 may confirm or revise it.

# `D0` -- GROUND THE ExactIntU64 SEATS (first deliverable)

Measure and report, so the Architect rules the specific wiring:

1. **Enumerate the `ExactIntU64` seats.** Which effect seats need `ExactIntU64`
   and refuse in `CarriedWord` on the live `cap41_*` path? `FsReadAt`
   `Argument(1)` is the measured terminal; report any structural siblings (the
   same-kind argument positions) so the closure covers the need, not one seat --
   the same subsume-don't-proliferate call the ResourceScalar node settled.
2. **Confirm the existing route applies.** Can each enumerated seat read through
   the existing `carried_exact_int` (`EITHER_PHASE`) route, exactly as
   `BufferAllocate` `0` does, with no `Avail` change and no new route -- or does a
   seat need more? Report the delta from a pure wiring change.
3. **Guard uniformity / soundness.** State whether the same fail-closed-consumer
   argument the ResourceScalar node established carries here (the observation
   proves the need is present in the carried phase; it does not relax the seat's
   `Need`-subset-`Avail` membership). `ExactIntU64` is a scalar integer, not a
   borrowed-opaque handle, so the borrowed-opaque nuance does not apply; state
   the discriminator this need actually rests on.

`D0` determines the node's final SIZE (estimated `S`) and hands the Architect the
input to rule the specific wiring.

## `D0` side-classification: the FsReadAt Arg(2) reply-path gate removal

(Architect ruling `evt_2qdpkfvtqrxzy`; a distinct ResourceScalar-family item.)

A distinct ResourceScalar-family item, NOT ExactIntU64 work -- carried here only
because it shares the `effects.rs` edit for contention; keep it distinct in the
accounting. The Adversary flagged (M8, `evt_5wx3bax63yak`) that FsReadAt's Arg(2)
buffer REPLY/span-provenance path (`effects.rs:3226`) still refuses a carried
buffer specialized-only, after RT-RESOURCE-RELEASE made the REQUEST path
carried-capable. The Architect ruled the fix is REMOVAL of a vestigial gate, NOT
a reroute: `3226`'s destructured `span_origin` is UNUSED (the constructor projects
the span from `site_operand_argument(builder, static_origin, 2, &seats)` at
`3233`), so routing it through `lower_resource_token_seat` would add a guarded read
whose scalar result is discarded -- a dead read. Do NOT reroute it.

CLASSIFY in D0, then route (do not fix it inside this node, do not absorb it
silently):

1. Confirm `site_operand_argument(.., 2, ..)` projects the buffer argument
   correctly when Arg(2) arrives CARRIED (that operand-list projection, not the
   destructured payload, is the live path that binds the span).
2. Confirm Arg(2) is already validated as a resource token on the REQUEST path
   (`2477` via `lower_resource_token_seat`), so the reply-path gate is a redundant
   re-validation whose removal drops only a spurious refusal, not a real check.
3. Direction: confirm removal enables no scalar misread (the value was already
   discarded).

If (1)-(3) hold and it is a clean removal, report it and I cut it as its own tiny
ResourceScalar-family successor (RT-RESOURCE-RELEASE's leftover reader). If any of
(1)-(3) needs real design work, name it with that argument as its own successor.
Either way it is a SEPARATE node from this one -- report the classification, do
not land the removal here.

# `D1` -- THE WIRING

Wire the enumerated `ExactIntU64` seats to observe the need in the carried phase
on the existing `carried_exact_int` `EITHER_PHASE` route the Architect rules on
`D0`. No `Avail` widening; the seat's membership test stays strict. Repoint any
transition sentinel strictly DOWNSTREAM with the direction argument recorded, the
same non-vacuity discipline the two predecessors held.

# ACCEPTANCE

- **AC-1 (the live seats are claimable).** No enumerated `ExactIntU64` seat
  refuses on the `withResource` path; the `cap41_*` rows advance past this
  blocker. Report the full per-row, per-seat disposition. A further distinct
  blocker exposed behind this one is a measurement to report and cut (or, if the
  rows go green, hand back to NHC's `D-final`), not a failure of this node.
- **AC-2 (observability, not relaxation).** The route PROVES the need is
  observable in the carried phase; it does NOT widen the seat's direct
  `Need`-subset-`Avail` partition. State the soundness argument for the
  `ExactIntU64` scalar need; if it is a pure wiring to the existing production
  route, say so and cite the `BufferAllocate` `0` precedent.
- **AC-3 (no regression).** All currently-compiling lowering is preserved;
  workspace-green in CI. (Local: targeted `-p` only, never `--workspace`; the
  respin gate is `-p ken-runtime` all-binaries + `-p ken-cli` + `-p ken-verify`,
  the coverage the two predecessors ran, since the last CI-red was a local/CI
  gate-scope gap.)
- **Required reviewer:** the Architect is the required reviewer on this node's
  merge Decision (carried-observation lowering, soundness-adjacent) and rules the
  specific wiring on `D0`. Adversary hunts the landed code.

# EXPLICITLY NOT IN SCOPE

- **The `ResourceScalar` family** -- closed by
  [[RT-RESOURCE-RELEASE-CARRIED-OBSERVE]] (merged). This node touches only
  the `ExactIntU64` need.
- **Any `Avail` partition change** or a general carried-need closure over every
  need. Bounded to the `ExactIntU64` need on the live `cap41_*` path.
- **Any dead-arm census / trap work** -- that is [[RT-DEAD-ARM-EFFECT-LOWERING]].
- **Any kernel / TCB edit.**

# CONTENTION

`ken-runtime` cranelift backend lowering (`effects.rs`). Both predecessors are
merged and the M8 Adversary hunt on the landed
[[RT-RESOURCE-RELEASE-CARRIED-OBSERVE]] route reported SOUNDNESS CLEAN
(`evt_5wx3bax63yak`) -- no soundness fold coming. Its one completeness observation
(the FsReadAt Arg(2) buffer reply-path gate) is DEFERRED (implementer
`evt_1rz7rnphp9ndw`, Architect `evt_2qdpkfvtqrxzy`): a mechanism/removal item, not
a fold, off the critical path. It rides into this node's `D0` as the
side-classification above and, if a clean removal, becomes its own tiny
ResourceScalar-family successor -- sharing this node's `effects.rs` edit for
contention but distinct in the accounting. RELEASED to the runtime ring (anchor
`evt_47kvrp1esty58`). Single ring, single lane, one `effects.rs`.
NATIVE-HANDLE-CARRIER is held on this node.

# CAPABILITY TIER

T2-leaning if D0 confirms a pure wiring to the existing `carried_exact_int`
route (mechanical application of an in-production precedent to enumerated seats),
with the Architect confirming the soundness argument carries unchanged. If D0
finds a seat needs more than wiring, it escalates toward the ResourceScalar
node's T1 profile -- the two review gates (Architect required reviewer, Adversary
hunt) are the safety net either way. Size S pending D0.
