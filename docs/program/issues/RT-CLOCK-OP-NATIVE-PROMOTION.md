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

## A SECOND CARRIER EXISTS, WRITTEN, AND THIS NODE INHERITS IT BY NAME

**Steward, 2026-09-17.** The section above states the two properties and their
controls, which is what a deferral needs to survive. It names **no carrier** —
and a property with no carrier is reconstructed from prose by whoever frames
this. **Two written bodies exist. They are recorded here so they do not have to
be.**

`ABI-S6-HS18-D5B-SUBSTRATE-PORT`'s source tree carries **post-promotion** forms
of both assertions, in the same file and module as `main`'s pre-promotion ones:

    crates/ken-interp/src/eval.rs, mod px5b_effect_observation_tests

    PROPERTY 1  source b601e2ec7898:8955  ac2_monotonic_readings_survive_a_wall_clock_step_backwards   ~84 lines
                main             :8972    ac2_monotonic_is_refused_while_the_wall_clock_still_reads_and_steps_back

    PROPERTY 2  source b601e2ec7898:9039  ac3_the_deadline_a_caller_passes_is_the_deadline_honoured    ~50 lines
                main             :9071    ac3_sleep_until_is_refused_on_a_path_that_still_serves_an_available_op

    CONTROL     source b601e2ec7898:9089  ac3b_the_deadline_decoder_refuses_a_second_argument
                main             :9135    ac3b_the_deadline_decoder_refuses_a_second_argument   <- IDENTICAL

**The third row is byte-identical in both trees.** It is the alignment control:
it establishes that these are the same module at the same ordinal positions, so
the first two rows are a genuine pre/post pair rather than two similarly-named
tests. `ac3b` is also **property 2's discriminator**, already written and
already on `main` — the second-deadline case the section above requires.

**Why they are not in the D5B candidate.** The port initially took the source
bodies, which asserts reification against a tree that refuses. The Steward ruled
them `ADJUDICATE` rather than `OWED` (`evt_3a9n197g3y627`) and the implementer
restored `main`'s refusal bodies — `f877914b4` carries
`ac2_monotonic_is_refused…` at `:8974` and `ac3_sleep_until_is_refused…` at
`:9073`, `ken-interp` 82/0. **`main`'s coverage was never dropped; the
post-promotion forms were simply not taken.**

⇒ **When this node lands the promotion, the two source bodies are the
starting point** — they are the assertions in their post-promotion form, and
they are what `main`'s two refusal tests must be inverted back into. They still
need the value-capturing backend below; a written body does not make an op-tag
recorder able to express it.

**Do not read the coordinates as current.** They are positions in
`b601e2ec7898`, a fixed tree. Re-find by symbol name, not by line.

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

# The differential state — MEASURED (Steward, 2026-09-16, at `394a5545f`)

**Neither op has a real-artifact differential. This is a build.**

> **CORRECTED by Architect `evt_11kc7kd57v10b`: the three artifacts below are a
> CONFIRMATION, not a promotion gate.** `confirm_native_tested_transition`
> requires the op to be `NativeTested` *already* before it reads any evidence
> (`catalog.rs:315-322`; its doc comment at `:313-314` says so outright).
> **Consequence for this node: the availability flip and the differential must
> land in ONE change, flip first within it.** Built differential-first, the
> evidence test exits at `OutsideNativeTestedSet` — a failure that has nothing
> to do with the evidence and does not look like an ordering problem. The full
> correction, the four-step ordering, and the negative-control shape are in
> [[RT-D5B-MAPPING-AVAILABILITY-FLIP]]; they apply here unchanged.

A promoted op is confirmed by three things in `crates/ken-verify`: a
`CanonicalDifferentialRun` (native vs interp), a
`NativeTestedEvidence::from_<op>_run(&run)`, and
`confirm_native_tested_transition(op, evidence) == Ok(NativeTested)` — plus, in
every existing instance, a negative control that perturbs the evidence and
requires the confirmation to fail.

**`ClockMonotonicNow` and `ClockSleepUntil` have none of the three.** The only
clock op with a differential is `ClockWallNow`
(`from_clock_wall_now_run`, asserted in `scenario.rs`), which is a useful
template and is **not** evidence about either of these.

## `ClockMonotonicNow` needs a COMPARATOR, not just a scenario

Architect `evt_11kc7kd57v10b`, recorded as a sizing input and not a new
requirement. `from_clock_wall_now_run` exists as its own constructor for a
reason: `ClockWallNow` is **nondeterministic**, so it uses
`compare_clock_wall_now()` rather than the generic `compare_exact()`.

**`ClockMonotonicNow` has the same property.** A monotonic reading is not
byte-equal across lanes either, so `from_run`'s `compare_exact()` is the wrong
comparator for it, and the generic table-driven route is not available. This op
needs its own constructor and its own comparator, on the `ClockWallNow` model.

⇒ **Whoever sizes this: that is a third build, not a third scenario row.**
`ClockSleepUntil` may be exact-comparable; `ClockMonotonicNow` is not.

⇒ **Two builds converge here and they are the same build.** The deferred
assertions above need a value-capturing test backend; the differential needs a
real-artifact run that compares values. **Neither is satisfiable against an
op-tag recorder**, which is why §"Why they could not simply move" and this
section have the same remedy. Frame them together or the second one re-derives
the first.

**Instrument reach:** `NativeTestedEvidence` and `CanonicalDifferentialRun`
occur in exactly two files tree-wide (`ken-verify/src/scenario.rs`,
`ken-verify/src/catalog.rs`), and `catalog.rs` only unit-tests the predicate on a
hard-coded `ConsoleFlush` with synthesized booleans. The absence above is a
measurement, not a silence.

**See also** [[RT-D5B-MAPPING-AVAILABILITY-FLIP]], which carries the ten-op
finding this measurement raised — **ten** of the 25 `NativeTested` ops have no
real-artifact differential either — now **answered**: the confirmation protocol
was never a promotion bar, so no backward sweep was owed and nothing about those
ten changes what this node owes.

# Sizing note, for whoever frames this

Unsized, and no longer for want of a measurement. **Three things decide the
size, and all three are now known inputs rather than open questions:** the
value-capturing test-backend extension, the `ClockMonotonicNow` comparator, and
the one-change ordering. Whether the differential exists is settled — it does
not.

# Related

- [[RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE]] — retires the assertions; its §9 is
  the verbatim record.
- [[RT-D5B-MAPPING-AVAILABILITY-FLIP]] — the same promotion shape for
  `MappingAcquireFile`. Separate op family, separate evidence; do not fold.
