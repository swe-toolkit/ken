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
> Two things gate the frame, and neither is scheduling slack:
> [[RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE]] must land first (see below), and
> the artifact differential that `effect_v1.rs:250` conditions promotion upon
> must exist. **Do not size this node before the differential's state is
> measured** — it is the input that decides whether this is a small flip or a
> build.

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

# Sizing / tier

**Unsized, tier T1.** Deliberately unsized: `effect_v1.rs:250` conditions
promotion on an artifact differential, and whether that differential exists,
partially exists, or is unbuilt is the difference between a several-line flip
and a build. Measure it before sizing. T1 regardless — a promotion out of the
unavailable tail is a claim that the operation works, and the review is of that
claim.

# Contention

`crates/ken-host/src/effect_v1.rs`, shared with the gate node and
[[RT-D5B-RESOURCE-TABLE-LIFECYCLE]]. Cranelift involvement is expected here and
was the reason this cluster was held out of slice 4 — confirm at framing rather
than assuming either way.
