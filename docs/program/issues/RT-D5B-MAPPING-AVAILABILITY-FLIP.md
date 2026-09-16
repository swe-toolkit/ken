---
id: RT-D5B-MAPPING-AVAILABILITY-FLIP
title: "Promote MappingAcquireFile out of the RepresentedUnavailable tail: flip site 2 (effect_v1.rs:193) to NativeTested and carry the sites the AC-AVAIL membership rule identifies. Cluster A of the ABI-S6-d5b drain. Gated on RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE, which is what makes site 2 the SINGLE flip point, and on the artifact differential that effect_v1.rs:250 conditions promotion upon. Also holds the Architect's F3 SAFETY-precondition carry."
status: draft
owner: runtime
size: unsized
gate: none
depends_on: [RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE, RT-D5B-HOST-FILE-ACQUISITION-SURFACE]
blocks: []
github: null
tier: T1
origin: "Steward cut 2026-09-16. Cluster A of the backup-branch census (3 hunks + 3 shared with cluster B, plus 23 test hunks the runtime-implementer measured as belonging to this flip). Held out of slice 4 on Architect ruling evt_3wtg8w8krmmt: 'Membership is a plan, not evidence.' Unblocked for CUTTING -- not for release -- by Architect ruling evt_21f23zmgqfxsc on the permissive-by-design question."
---

> # DRAFT. Not framed, not released. Do not start.
>
> **Gate 1 is CLEARED.** [[RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE]] landed at
> `dcb848eaa68111a32f6bb135afbbba1d8e862ad1`, so site 2 is now the single flip
> point as a checkable property rather than an end state.
>
> **Gate 2 is MEASURED, and the answer is "unbuilt".** See "The differential
> state" below: `MappingAcquireFile` has **no real-artifact differential**, so
> this is a build and not a several-line flip. The node stays unsized only
> because the build's shape is the framer's call; the input it was waiting on is
> no longer missing.

# Objective

Promote `MappingAcquireFile` from `RepresentedUnavailable` to `NativeTested`,
making the file-acquisition surface landed by
[[RT-D5B-HOST-FILE-ACQUISITION-SURFACE]] actually available.

# Why the refusal gate lands first

Architect, `evt_21f23zmgqfxsc` point 3: the gate *"makes SITE 2 the single flip
point, which is the property I asked for in F1."* That property is purchased by
the gate, not by this node.

Without the gate the interpreter consults `availability()` **nowhere**, so it is
already open. Flipping site 2 then opens native while the interpreter was never
closed — one auditable line for one executor plus an unwritten second act, which
is the shape F1 asked to avoid. With the gate landed first, this node's diff can
be read as *"one line, both executors"* against a base where that is verifiably
true.

# What this node carries

- **The flip itself.** Site 2, `effect_v1.rs:193`, plus whatever the AC-AVAIL
  **membership rule** identifies at framing time —
  [[RT-D5B-HOST-FILE-ACQUISITION-SURFACE]] §4a. Use the rule, not its list of
  five instances; the list was correct at `e11341c7b9d1` and is a snapshot.
- **The 23 test hunks** in the backup's `effect_v1.rs` cluster. The
  runtime-implementer measured every one as belonging to this flip. If a
  re-census finds hunks that belong elsewhere, that is a finding to report, not
  to absorb.
- **Architect F3: the SAFETY-precondition split, and the irreducible TOCTOU.**
  Carried forward from slice 4 unactioned, by ruling. It belongs here because it
  bears on what is safe to make available, not on what is safe to compile.

# Not this node

- **The uniform-refusal gate and its predicate control.** That is
  [[RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE]], it is this node's `depends_on`,
  and its subject is REACHABILITY across all ten unavailable ops. Do not fold it
  in here on the grounds that it is small: the Architect's warning is explicit
  that the next reader will try to fold it into the availability census and drop
  one of the two.
- The `ResourceTableV1` lifecycle cluster — [[RT-D5B-RESOURCE-TABLE-LIFECYCLE]].
- `conformance/surface/ffi-io/seed-mapping.md:53`. It is BLOCKED-ON-ABI-S6-D5b
  and **this** is the node that unblocks it — the Architect cancelled by name
  the earlier advice that slice 4 would. Unblocking it is the spec-leader's act
  on this node's landing, not a deliverable here.

# The differential state — MEASURED (Steward, 2026-09-16, at `394a5545f`)

**What the promotion protocol actually requires**, read off the mechanism rather
than the prose. A promoted op needs all three, in `crates/ken-verify`:

    1. a real-artifact differential run   CanonicalDifferentialRun, native vs interp
    2. a NativeTestedEvidence for it      NativeTestedEvidence::from_<op>_run(&run)
    3. the transition asserted            confirm_native_tested_transition(op, evidence)
                                          == Ok(HostOpAvailabilityV1::NativeTested)

plus, in every existing instance, a **negative control** — the evidence is
perturbed (`wrong_native`) and the confirmation must then fail.

**`MappingAcquireFile` has none of the three.** No `from_mapping_acquire_file_run`
constructor exists, and it appears in no `confirm_native_tested_transition`
assertion.

⇒ **This node is a BUILD, not a flip.** The one-line availability change is the
last step, not the work. The work is constructing a real-artifact differential
for file-backed mapping acquisition and its negative control.

**Instrument reach, stated so the absence is a measurement and not a silence.**
`NativeTestedEvidence` occurs in exactly two files tree-wide
(`ken-verify/src/scenario.rs`, `ken-verify/src/catalog.rs`) and
`CanonicalDifferentialRun` in the same two. `catalog.rs`'s only
`confirm_native_tested_transition` calls are a **predicate** unit test on a
hard-coded `ConsoleFlush` with synthesized booleans — not a differential for any
op. So `scenario.rs` is the whole real-artifact harness, and a grep that covers
it covers everything.

## The finding this turned up, which is NOT this node's scope

    NATIVE_TESTED_TARGETS_V1                            25 ops
    with a real-artifact differential in scenario.rs    15
    WITHOUT one                                         10

The ten: `FsOpen`, `FsHandleMetadata`, `FsReadAt`, `FsWriteAt`,
`ResourceRelease`, `BufferAllocate`, `BufferFreeze`, `MappingAllocate`,
`MappingReadView`, `MappingWriteView` — the whole resource / positioned-IO /
mapping family.

**This is a question, not a defect claim.** `effect_v1.rs:272` says an operation
stays `RepresentedUnavailable` *until its artifact differential gates promote it
explicitly*, and ten ops are promoted without one visible. Either the protocol
was applied through a route this instrument cannot see, or those ten were
promoted before it existed and nobody swept backward. **Which of those it is
changes what this node owes**: if the protocol is the real bar, this node builds
a differential; if ten promotions already bypassed it, the bar is aspirational
and that is a much larger conversation than a mapping flip. **Routed to the
Architect; do not resolve it inside this node.**

# Sizing / tier

**Unsized, tier T1 — and the reason is no longer "unmeasured".** The differential
state is measured above and says *build*. What remains open is the build's
shape, which is the framer's call and depends on the Architect's answer to the
ten-op question. T1 regardless: a promotion out of the unavailable tail is a
claim that the operation works, and the review is of that claim.

# Contention

`crates/ken-host/src/effect_v1.rs`, shared with the gate node and
[[RT-D5B-RESOURCE-TABLE-LIFECYCLE]]. Cranelift involvement is expected here and
was the reason this cluster was held out of slice 4 — confirm at framing rather
than assuming either way.
