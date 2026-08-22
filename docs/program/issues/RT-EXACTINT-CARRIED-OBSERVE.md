---
id: RT-EXACTINT-CARRIED-OBSERVE
title: "A genuinely-live effect seat needing ExactIntU64 (FsReadAt Argument(1)) cannot observe its need in the CarriedWord phase, so the withResource path still fails object emission behind the now-closed ResourceScalar family -- the ExactIntU64-need carried-observation closure on the existing carried_exact_int EITHER_PHASE precedent"
status: ready
owner: runtime
size: S
gate: none
depends_on: []
blocks: [NATIVE-HANDLE-CARRIER]
github: null
origin: "Measured terminal of [[RT-RESOURCE-RELEASE-CARRIED-OBSERVE]] AC-1 (runtime-implementer evt_e68jv2mssxnd, runtime-qa evt_6e1kf4tdghchs, Architect evt_24nyqqhs5fy1f, 2026-08-22): with the ResourceScalar family closed, the cap41_* rows advance to `seat Argument(1) of FsReadAt needs ExactIntU64, which it cannot observe in CarriedWord`. The Architect scoped this OUT of the ResourceScalar node as a distinct need with its own precedent; the Steward cut it as the next lane-1 successor (evt_5xq3hw23kamrd). Steward-filed per COORDINATION section 2."
---

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
merged. The M8 Adversary hunt on the landed [[RT-RESOURCE-RELEASE-CARRIED-OBSERVE]]
route reported SOUNDNESS CLEAN (`evt_5wx3bax63yak`), so no soundness fold is
coming -- BUT it raised one non-blocking completeness observation (the FsReadAt
Arg(2) buffer reply-path reader at `effects.rs:3226` still refuses a carried
buffer specialized-only). If the runtime ring rules that a real gap to close now,
it is a small fold on `effects.rs` that would contend this node (single ring,
single lane, one `effects.rs`). SEQUENCED AFTER the ring dispositions that
observation as deferred (off the current critical path; the rows hit this node's
`ExactIntU64` terminal first). Do not kick this node until that disposition is in.
NATIVE-HANDLE-CARRIER is held on this node.

# CAPABILITY TIER

T2-leaning if D0 confirms a pure wiring to the existing `carried_exact_int`
route (mechanical application of an in-production precedent to enumerated seats),
with the Architect confirming the soundness argument carries unchanged. If D0
finds a seat needs more than wiring, it escalates toward the ResourceScalar
node's T1 profile -- the two review gates (Architect required reviewer, Adversary
hunt) are the safety net either way. Size S pending D0.
