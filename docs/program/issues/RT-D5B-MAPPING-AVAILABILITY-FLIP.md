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
> ## 2026-09-17: THIS NODE NOW RECEIVES A NAMED ROW POPULATION FROM THE D5b PORT
>
> **Three OWED method rows are being handed to this node by name**, per
> [[ABI-S6-HS18-D5B-SUBSTRATE-PORT]]'s `AC-1e`. They are owed work the port
> cannot close, because they sit in files the six-site ruling excludes from
> hand-separation, and they are **the refused grant's own carriers** rather than
> bystanders — measured references to the operation in each source body:
>
>     lowering/effects.rs::lower_process_host_effect        8    census site 5
>     st/effects.rs::host_effect_seat_contract              5    the sixth site
>     st/effects.rs::host_effect_constructor_dispatch       1    the sixth site
>
> **This does not raise this node's priority, start it, or change its gates** —
> same as the retracted claim recorded below, and stated here so the two are not
> confused. It records that the port's `REMAINING = 0` is satisfiable **only
> because these rows have a recipient**, and this node is it.
>
> **The count is a PREDICATE, not a fixed three.** `AC-1e` derives the bucket
> from the file-exclusion rule, so a fourth carrier would arrive here on its own.
> **A row excluded by file that does NOT reference the refused operation is not
> this node's** — it is owed work with no owner and goes back to the Steward.
>
> ## 2026-09-17: A "THIS NODE BLOCKS A `ready` NODE" CLAIM WAS MADE AND WITHDRAWN
>
> **RETRACTED THE SAME HOUR. This node's priority is UNCHANGED, and nothing below
> raises it.** Recorded because the claim was broadcast fleet-wide
> (`evt_30p9m0j8bj1f2`) before it was withdrawn, and a retraction that is not
> written where the claim lives does not travel.
>
> **What was claimed:** that the grant this node holds is the only thing making
> [[RT-D5B-POSTCALL-REFUSAL-MECHANISM]] workable on `main`, so this node sat on a
> `ready` node's critical path. **What was actually measured** (Steward, at
> `origin/main` `b0eb29e71`):
>
>     abi_s6_mapping_file_backed_native.rs   13 tests, ONE program (const SOURCE),
>                                            15 of 15 build/run sites pass it,
>                                            SOURCE :67 acquires a FileBacked mapping
>     lowering/effects.rs :2812              if !CRANELIFT_HOST_EFFECT_CONSUMERS_V1
>                                              .contains(&operation) { refuse }
>
> That census is complete and correct, and it answers **"can the ACCEPTANCE TEST
> reach the refusal"** — not **"can anything on `main` reach it."** The Architect
> measured under the edge instead of inheriting it (`evt_1grgpvv8cjq66`): the
> emitter `bind_checked_ih_detached_caller_cut` lives in `lowering/source.rs`, a
> **Region 2** file the substrate port lands, and its call sites are gated on
> **tail shape** rather than on any effect — so an op-invoking program is routed
> **away** from that emitter, not toward it.
>
> ⇒ **The `blocks` edge is removed and the grant is NOT established as that
> node's blocker.** Three readings are live there and the substrate port's build
> discriminates them; see that node's banner.
>
> **THE GRANT'S FOOTPRINT IS SIX SITES, NOT FIVE.** The census of five was short.
> The sixth is `CRANELIFT_HOST_EFFECT_CONSUMERS_V1` membership plus the matching
> removal from the named-unavailable-lanes arm, both in
> `planning/static_transition/effects.rs` — **the site that actually operates the
> gate**, and it carries site 3's signature of a pure deletion with no line to
> grep for. Found by the runtime-implementer (`evt_316yppb8r4zxa`), confirmed by
> the Architect (`evt_1jfpng6yvy89v`). There is also a seventh touch in
> `st/aggregates.rs`. **Treat six as a floor, and close it with the
> `AC-PREDICATE` — zero diff lines naming the operation at pathspec `crates/` —
> not with a site list.**
>
> ### PRICING RULE — it OUTLIVES the retracted edge. Do not delete both.
>
> The Architect holds `gate: architect` on [[RT-D5B-POSTCALL-REFUSAL-MECHANISM]]
> **and** is the seat that refused this grant twice and defined its scope both
> times. They disclosed it unprompted (`evt_prasphavwv64`), and then measured
> against their own position — the finding that withdrew the edge above is
> **theirs**, and it removed the pressure that would have pushed this grant
> through.
>
> **The rule stands whether or not any node blocks on this one.** It is about how
> this node gets priced, not about who is waiting.
>
> **Price this node without deference to that refusal.** The correct input from
> that seat is the refusal's **ground**, not its authority: *"membership is a
> plan, not evidence"* (`evt_3wtg8w8krmmt`). If this node supplies evidence of
> native availability, the refusal is discharged on its own terms.
>
> **And do NOT let this through on the argument that a blocked node needs it.**
> That is precisely the argument the original refusal rejected. A downstream
> block raises this node's **priority**; it supplies none of its **evidence**.
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
>
> **Whoever frames this: take the ORDERING from "The ordering" section, not from
> the shape of the work.** The availability flip and the differential must land
> in **one** change. Built in the intuitive order — differential first, flip
> after — the evidence test fails before it reads any evidence, and the failure
> looks like a broken differential.

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

# A REFUSED INSTANCE OF THIS FLIP EXISTS ON A PRESERVED REF. It is a HAZARD.

**`preserve/ABI-S6-HS18-verifier-checkpoint-not-a-candidate`
(`5d977ac7968dff3763d330690a9b4df530925d79`) already contains this flip, fully
coordinated, and it is REFUSED.** Recorded here so the next reader finds it as a
hazard rather than as a head start.

**Do not harvest it.** The flip is refused on the merits — `MappingAcquireFile`
has no real-artifact differential, and `catalog.rs:315` would refuse to confirm
one if it did. It is exactly as unauthorized on that ref as it would be
anywhere else; being already written does not advance this node by a line.

**What it is good for is the opposite purpose: it is the known-bad tree.** Any
gate written to stop this flip should be run against `5d977ac79` and required
to **FAIL**. That control has already retired one proposed phrasing that
reasoning alone endorsed (see below), and it costs one command.

**The five coordinated sites, measured at `6f49f8521` against `5d977ac79`:**

    site                                       main          5d977ac79      shape
    1  effect_abi_v1.catalog:97                unavailable   native         change
    2  effect_v1.rs availability() arm         Unavailable   NativeTested   ADD
    3  effect_v1.rs ten-op refusal arm         10 ops        9 -- MAF GONE  DELETE
    4  effect_v1.rs NATIVE_TESTED_TARGETS_V1   [_; 25]       [_; 26]        ADD
    5  lowering/effects.rs                     0 occurrences 5+, incl. a
                                                             lowering arm   ADD

**THIS TABLE OF FIVE IS A RECORD TAKEN AT `6f49f8521`, AND IT IS SHORT BY ONE.**
Left unaltered because it records what was measured then. **The sixth site was
found 2026-09-17** — `CRANELIFT_HOST_EFFECT_CONSUMERS_V1` membership plus the
matching removal from the named-unavailable-lanes arm, both in
`planning/static_transition/effects.rs`, with a seventh touch in
`st/aggregates.rs`. See the banner. **The lesson the miss carries: sites 3 and
the sixth are pure DELETIONS with no added line, so any census built by grepping
for what a diff ADDS is structurally blind to them. Use the `AC-PREDICATE`, not
a site list.**

Five is a **floor, not a census** — nobody has proven no sixth exists, which is
the argument for a predicate over an enumeration.

**Four additions and one deletion, and the asymmetry is the lesson.** Sites 1,
2, 4 and 5 each produce a new line a reviewer or a grep can query. **Site 3
produces silence** — a roster shrinking by one leaves nothing to match. It is
also the site that holes [[RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE]] by
subtraction. **An omission has no complainant.**

**A relation test does NOT catch this, and the measurement is the point:**

    tree        availability()                      refusal arm   relation
    main        10 RepresentedUnavailable / 25 NT   10 members    10 == 10  HOLDS
    5d977ac79    9 RepresentedUnavailable / 26 NT    9 members     9 ==  9  HOLDS

The flip edits both sides, so every internal relation stays true. **A
consistency invariant cannot detect a coordinated unauthorized change** —
coherence and warrant are different properties. The checkpoint's flip is
internally coherent; what is wrong with it is that it is unauthorized and
unproven (Architect, `evt_7jcex53r71gn6`).

⇒ **When this node is eventually framed and the flip is authorized, it still
gets written fresh.** The gate protecting other candidates from it lives in
`docs/program/wp/ABI-S6-HS18-MAIN-BASED-CLOSURE.md` §4, keyed on the operation
at pathspec `crates/`, and it retires with that candidate rather than landing
as a test.

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

> ## CORRECTED — the verdict survives, its premise does not
>
> Architect `evt_11kc7kd57v10b`.
>
> **"This node is a BUILD, not a flip" is still true, and every number below is
> still right.** What was wrong is *why*, and the error is not cosmetic — it
> licensed a sequencing that cannot pass. See "What the protocol actually is"
> immediately below, and take the ordering from there, not from this section's
> original reading.

**What `MappingAcquireFile` has**, measured: **none of the three** artifacts
listed below. No `from_mapping_acquire_file_run` constructor exists, and the op
appears in no `confirm_native_tested_transition` assertion.

    1. a real-artifact differential run   CanonicalDifferentialRun, native vs interp
    2. a NativeTestedEvidence for it      NativeTestedEvidence::from_<op>_run(&run)
    3. the transition asserted            confirm_native_tested_transition(op, evidence)
                                          == Ok(HostOpAvailabilityV1::NativeTested)

plus, in every existing instance, a **negative control** — the evidence is
perturbed and the confirmation must then fail.

⇒ **This node is a BUILD, not a flip.** The one-line availability change is not
the work. The work is constructing a real-artifact differential for file-backed
mapping acquisition and its negative control.

## What the protocol actually is — it CONFIRMS, it does not PROMOTE

The original reading of this section called the three artifacts above a
*promotion gate*. **They are not one, and the mechanism says so directly.**
`crates/ken-verify/src/catalog.rs:315-322`, verified at `d7596ac68`:

```rust
pub fn confirm_native_tested_transition(
    operation: HostOpV1,
    evidence: NativeTestedEvidence,
) -> Result<HostOpAvailabilityV1, StatusTransitionError> {
    if !ken_host::NATIVE_TESTED_TARGETS_V1.contains(&operation)
        || operation.availability() != HostOpAvailabilityV1::NativeTested
    {
        return Err(StatusTransitionError::OutsideNativeTestedSet(operation));
    }
```

**It requires the op to be `NativeTested` ALREADY, before it will look at any
evidence.** A `RepresentedUnavailable` op does not *fail* the differential — it
never reaches it, exiting at `OutsideNativeTestedSet`. The function's own doc
comment at `:313-314` says it outright: *"Deferred operations cannot be confirmed
through this gate."*

⇒ **The promoting act is the `ken-host` edit** — `availability()` plus the
roster. The differential is a **post-hoc confirmation** that a declaration
already made is backed by a real artifact.

### The sentence this node quoted documents a DIFFERENT CONSTANT

`effect_v1.rs:270-272` is a doc comment, and its subject is the item immediately
below it at `:274`:

    :270  /// PX5's intended promotion set. Membership is a plan, not evidence: every
    :271  /// operation remains `RepresentedUnavailable` until its artifact differential
    :272  /// gates promote it explicitly.
    :274  pub const PX5_PLANNED_NATIVE_TARGETS: [HostOpV1; 5] = [ ... ];

    :282  pub const NATIVE_TESTED_TARGETS_V1: [HostOpV1; 25] = [ ...   <- NO doc comment

**`NATIVE_TESTED_TARGETS_V1` carries no doc comment at all.** The rule read here
as governing the 25-op roster is a caveat on the 5-op PX5 *plan*, and in place it
says something narrower and sensible: do not mistake membership in the plan for
evidence. It is not a standing invariant over the roster — which is why nothing
enforces it over the roster, and why the ten below can sit there silently.

## THE ORDERING — what the broken premise licensed

**Do NOT sequence this as "differential first, then flip."** That order cannot
pass: before the flip, `confirm_native_tested_transition` returns
`OutsideNativeTestedSet`, so the evidence test fails for a reason that has
nothing to do with the evidence. The availability edit and the differential must
land in the **same** change:

    1. flip `availability()` and add the op to NATIVE_TESTED_TARGETS_V1
    2. add the scenario and the CanonicalDifferentialRun
    3. assert confirm_native_tested_transition(op, ev) == Ok(NativeTested)
    4. the negative control that perturbs the evidence and requires failure

**Step 4 is load-bearing and is the reason step 3 is not circular.**
`catalog.rs:379-395` already gives the control shape: three
`evidence(false,true,true)`-style rows, one per field, each asserting its own
`StatusTransitionError` variant. **A control that flips all three fields at once
cannot tell you which field the confirmation actually reads.**

**Instrument reach, stated so the absence is a measurement and not a silence.**
`NativeTestedEvidence` occurs in exactly two files tree-wide
(`ken-verify/src/scenario.rs`, `ken-verify/src/catalog.rs`) and
`CanonicalDifferentialRun` in the same two. `catalog.rs`'s only
`confirm_native_tested_transition` calls are a **predicate** unit test on a
hard-coded `ConsoleFlush` with synthesized booleans — not a differential for any
op. So `scenario.rs` is the whole real-artifact harness, and a grep that covers
it covers everything.

## The finding this turned up — ANSWERED, and NOT this node's scope

    NATIVE_TESTED_TARGETS_V1                            25 ops
    with a real-artifact differential in scenario.rs    15
    WITHOUT one                                         10

The ten: `FsOpen`, `FsHandleMetadata`, `FsReadAt`, `FsWriteAt`,
`ResourceRelease`, `BufferAllocate`, `BufferFreeze`, `MappingAllocate`,
`MappingReadView`, `MappingWriteView` — the whole resource / positioned-IO /
mapping family.

**The answer is neither of the two readings this section originally offered.**
The protocol was not bypassed and was not applied invisibly: it is **not a
promotion bar at all** (see above), so there was never a backward sweep owed.
Nothing enumerates `NATIVE_TESTED_TARGETS_V1` and requires a differential per
member. That is the whole reason ten can sit there silently.

### The census survived a probe of its own blind spot

Worth recording as method, because the number was right for a reason the
instrument could not have guaranteed. The Steward's instrument was keyed on
`from_<op>_run` **spellings**, and there is a generic route it could not have
seen — `scenario.rs:4661`:

```rust
for operation in PX5_PLANNED_NATIVE_TARGETS {
    let evidence = NativeTestedEvidence::from_run(operation, &run);
```

`from_run(operation, &run)` is fully op-parameterized, so a table-driven harness
confirms ops whose names appear nowhere near a constructor — **exactly the shape
a name-keyed grep reports as absent.** The Architect re-derived by a different
key, enumerating the `confirm_native_tested_transition` call sites and unioning
the ops each covers:

    singletons  ClockWallNow :3477   ConsoleRead :3577   FsReadDirectory :3633
                FsCreateDirectory :3740   FsRemoveFile :3842  FsRemoveDirectory :3945
                FsAppendFile :4214   FsMetadata :4344   FsRename :4494
                FsChangeMode :4727                                        = 10
    loop :4665  PX5_PLANNED_NATIVE_TARGETS                                =  5
                                                                    union = 15

Complement over the 25 is the ten, exactly. **A correct count from an instrument
with a reachable hole is luck until someone probes the hole rather than
re-reading the count.** The generic route exists; it happens to iterate the five
already counted.

**One refinement, Steward at `d7596ac68`:** there are **twelve**
`confirm_native_tested_transition` call sites in `scenario.rs`, not eleven. The
twelfth is `:5215`, and it does not change the union — it is a **negative**
control asserting `Err(MissingExactArtifactEvidence)` on a `ConsoleWrite` already
inside PX5's five. The discriminator for anyone re-running this is **positive
confirmations only**; counting all call sites gives 12 and the wrong split.

### If the ten are ever to be closed, the closure is a PREDICATE

Not a ruling — the Architect left this to be framed or escalated, and it is not
this node's scope. Recorded so the shape is not re-derived: the closure is a test
that **iterates the roster and requires each member to be confirmed**, never a
list of fifteen. **A list cannot report being short.**

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

`crates/ken-verify/src/scenario.rs` is now also in scope, since the differential
lives there.

# Housekeeping to fold in, since this node touches `scenario.rs`

Not a defect and not worth its own respin. `scenario.rs` carries **nine**
comments reading `// Ignored pending RT-CARRIER-BYTESPAN-OBSERVE` — `:4620
:4672 :4732 :4783 :4817 :4876 :5012 :5074 :5127` — and **`#[ignore]` appears zero
times anywhere in `ken-verify`** (both verified at `d7596ac68`). The quarantine
those comments name was closed by `7ca5cfc05`, and
`docs/program/evidence/ci-shard-duration-balance-33230600665.json:22533` records
`real_artifact_five_op_observation_matches_interp_on_twin_roots` running in CI
with its own shard row.

**The annotations survived the fix they described.** It matters because a reader
auditing evidence coverage — exactly what produced the ten-op finding above —
would discount nine running tests as quarantined. Delete them when you are next
in the file.
