---
id: RT-CLOCK-OP-NATIVE-PROMOTION
title: "Promote ClockMonotonicNow and ClockSleepUntil out of the RepresentedUnavailable tail. Carries the evidence RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE retires by name: the monotonic-under-a-backwards-wall-step property and the deadline-honoured discriminator, neither of which ken-host's op-tag test backend can express today."
status: draft
owner: runtime
size: unsized
gate: none
depends_on: [RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE]
blocks: []
github: null
tier: T1
origin: "Steward cut 2026-09-16, as condition 2 of Architect ruling evt_rsbhqs2tfamg. The ruling deferred two ken-interp clock assertions rather than relocating them, on the measured ground that ken-host's test backend is an op-tag recorder; condition 2 requires the promotion node to inherit them BY NAME, and no clock-promotion node existed to inherit them. This node exists so the deferral has a carrier."
---

> # DRAFT. Not framed, not released. Do not start.
>
> This node was cut to **carry an obligation**, not because the promotion is
> scheduled. Its frame is owed; its evidence list is not, and is below.

# Objective

Promote `ClockMonotonicNow` and `ClockSleepUntil` from `RepresentedUnavailable`
to `NativeTested`, with the artifact differential that `effect_v1.rs:250`
conditions promotion upon.

# The evidence this node INHERITS, and may not promote without

[[RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE]] inverts two `ken-interp` tests into
refusal tests and **retires two properties in doing so**. The Architect ruled
this a deferral rather than a loss (`evt_rsbhqs2tfamg`) on the explicit
condition that this node reproduce them. They are spelled out here rather than
cited, because a pointer is what a deferral evaporates through.

**The gate WP's §9 holds them verbatim at their `main` line numbers.** What must
be reproduced, restated as properties:

1. **`ClockMonotonicNow` is non-decreasing under a backwards wall-clock step.**
   The wall clock is scripted to run BACKWARDS while the monotonic source
   advances; monotonic readings must not decrease.

   **Its positive control is part of the obligation, not decoration.** The
   backwards wall step must itself be asserted *first* — a green monotonic
   result with no such control equally means the harness could not perturb the
   wall clock at all, and a vacuous pass is what this shape exists to prevent.

2. **`ClockSleepUntil` honours the deadline a caller passes.** The host must
   observe exactly the deadline supplied.

   **Its discriminator is part of the obligation.** A second, different deadline
   must produce a different observation. Without it the assertion holds for a
   host that ignores the argument entirely, which is precisely the failure it
   exists to catch.

## Why they could not simply move to `ken-host`

Measured, not assumed. `ken-host`'s test backend is an **op-tag recorder, not a
value recorder**:

    clock_sleep_until(_deadline)   pushes the tag and DISCARDS the deadline
    clock_monotonic_now()          pushes the tag and returns a CONSTANT

Both properties are about **values**, so neither is expressible there today.
Relocating them was ruled and then **withdrawn** on this measurement.

⇒ **This node owes the backend extension regardless of the deferral.** A native
differential compared against a constant-returning recorder is vacuous, so
capturing values is a precondition of the promotion itself, not a cost the
deferral imposes on it. The deferral rides work this node already had.

# Sizing note, for whoever frames this

Do not size before measuring the state of the artifact differential that
`effect_v1.rs:250` conditions promotion upon, and the size of the test-backend
extension above. Those two inputs decide whether this is a flip or a build.

# Related

- [[RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE]] — retires the assertions; its §9 is
  the verbatim record.
- [[RT-D5B-MAPPING-AVAILABILITY-FLIP]] — the same promotion shape for
  `MappingAcquireFile`. Separate op family, separate evidence; do not fold.
