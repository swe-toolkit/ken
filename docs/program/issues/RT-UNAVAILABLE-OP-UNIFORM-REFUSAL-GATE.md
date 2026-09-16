---
id: RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE
title: "Enforce the RepresentedUnavailable invariant uniformly across BOTH executors by gating dispatch_host_op_v1, with a control that is a PREDICATE over availability() rather than an assertion about any named op. The invariant is STATED at effect_v1.rs:193 and native-enforced at abi_v1.rs:1551, but the interpreter path consults availability() nowhere -- so a RepresentedUnavailable op currently executes interpreted and refuses natively. Architect ruling evt_21f23zmgqfxsc: the interpreter MUST refuse; RepresentedUnavailable is a language-surface claim, not a native-backend one. Its SUBJECT IS REACHABILITY, which is a different question from AC-AVAIL's availability census -- do not fold the two together."
status: draft
owner: runtime
size: S
gate: none
depends_on: []
blocks: [RT-D5B-MAPPING-AVAILABILITY-FLIP]
github: null
tier: T1
origin: "Adversary Finding 1 on the landed slice 4 (statements != enforcements), routed by the Steward to the Architect as a design question rather than ruled; Architect RULED evt_21f23zmgqfxsc. Steward cut 2026-09-16 as its OWN node rather than as a rider on the flip slice -- a sequencing/packaging call (steward.md §3), not a departure from the ruling's design content, which is adopted verbatim. Fixed inputs measured at origin/main d4e977a6af1083975665e587ed7e3e31f733785e."
---

> # DRAFT. Not framed, not released. Do not start.
>
> The design is RULED and is not open. What is unmeasured is the blast radius
> (see D0) and the reachability of the other nine ops. The Steward frames and
> releases this when the runtime lane reaches it — which, per the sequencing
> ruling below, is BEFORE the availability flip.

# Objective

Make *"`RepresentedUnavailable` implies refused"* an **enforced** invariant
rather than a **stated** one, by gating `dispatch_host_op_v1`
(`crates/ken-host/src/effect_v1.rs:2628`) — the single function both executors
traverse — and adding a control that derives the unavailable set from
`availability()` instead of naming ops.

# Why this is not a rider on the availability flip

The Architect's ruling placed this with the flip slice. The **design** is
theirs and is adopted unchanged; the **packaging** is the Steward's, and it
splits, for three reasons. The third is the deciding one.

1. **The subjects are different sizes.** The gate's subject is the ten
   `RepresentedUnavailable` ops and a reusable pattern. The flip's subject is
   one op, `MappingAcquireFile`. Bundled, the flip candidate's review turns on
   two unrelated arguments — *"is this op ready to be natively available?"* and
   *"is the uniform-refusal invariant correctly enforced for every unavailable
   op?"* — and a reviewer who approves the first has not thereby approved the
   second.

2. **The Architect's own argument is an independence argument.** The section
   heading reads *"why this belongs to the flip slice"*, but its body says the
   opposite about the content: *"the structural defect outlives that, and it is
   the more important half... every future unflipped surface reintroduces this
   divergence unless the gate exists."* A defect that outlives the flip and is
   not about the op is not scoped by the flip.

3. **The gate is ready and the flip is not.** The gate needs nothing that does
   not already exist: the ruling is made, the location is named, the control's
   shape is specified. The flip needs `MappingAcquireFile` to actually be
   native-ready — `effect_v1.rs:250` conditions promotion on *"its artifact
   differential"*, which is unbuilt. Bundling makes a ready deliverable wait on
   an unready one, which `steward/merge-policy.md` rules against directly.

**The ordering claim, stated so it can be refuted:** gate-before-flip is what
makes the Architect's point 3 (*"site 2 is the single flip point"*) true as an
**auditable** property rather than only as an end state. If the two land
together, a reviewer cannot observe that flipping site 2 opens both executors,
because the thing that makes the interpreter closed arrives in the same diff.
If the flip lands first, the property is never available for this op at all.
Its falsifier: show a reading of site 2 that establishes the single-flip-point
property without a separately-landed gate.

# What is RULED and not open

From `evt_21f23zmgqfxsc`, adopted verbatim:

1. **The invariant is `RepresentedUnavailable` implies REFUSED BY EVERY
   EXECUTOR.** The name is not doing double duty; `effect_v1.rs:249-251` does
   not need rewording. "Permissive by design" is rejected.
2. **The gate goes in `dispatch_host_op_v1`** — one gate, both paths. The
   native gate at `abi_v1.rs:1551` **stays**: it sits at the FFI boundary and
   returns an ABI-shaped code, correct for that layer. Double-gating is
   defense in depth and the two return shapes belong to their own layers.
3. **The control must be a PREDICATE**, never a list:

       for every op in HostOpV1::ALL with availability() == RepresentedUnavailable,
           the interpreter path refuses it

   Ten ops satisfy that today. **The control must derive that set, never
   enumerate it** — an op-specific test would reproduce the enumeration defect
   already filed against AC-AVAIL, and would say nothing about the next
   unflipped surface.

# The subject is REACHABILITY, not availability. Keep them apart.

Architect, `evt_435455zcfpw8t`, and this is the single most load-bearing
sentence in the node:

> **The five AC-AVAIL sites were complete, correct, and would not have caught
> this — and the predicate fix does not change that.**

Two distinct failures were in play and only one was fixed:

    the LIST could not report being short     -> fixed by AC-AVAIL's predicate
    the LIST's SUBJECT was not REACHABILITY   -> untouched by that fix

An availability census answers *"is this op marked and gated as unavailable?"*
Reachability asks *"can a Ken program get here?"* Every AC-AVAIL site is a
**native**-availability site, so a change that makes an op reachable from Ken
source through the interpreter touches none of them — and a sixth or tenth site
found by the corrected membership rule would also be native and also stay
green.

⇒ **This node's gate and control are not a stronger census.** They are a control
on a different subject, which is why they exist *alongside* AC-AVAIL rather than
inside it. Anyone concluding *"the census was fixed, so we are covered"* is
wrong. Frame the ACs so the next reader cannot fold the two together and drop
one.

# How the divergence arose — it was a deleted guard, and that is the reusable part

Before slice 4, unavailability for this op was enforced **in the prelude**, in
Ken source (`crates/ken-elaborator/src/prelude.rs`, deleted at `d4e977a6a`):

    proc private_mapping_file_unavailable (a : Auth) (e : Type) (r : Type) ...
        (Err ResourceError (ResourceBracketResult e r) (ResourceHostIO Unsupported))

That is a language-level refusal producing a Ken-level `ResourceError`, and it
bound **both** executors because it sat above both. The divergence did not exist
until that stub was deleted and appeared the moment it was.

**Deleting it was correct and spec-mandated** (`spec/.../38-ffi-io.md:769-784`
requires the offset-less `FileBacked`), so the deletion is not the defect. The
defect is that the stub was the sole enforcement of a **second** thing nobody
named, and nothing took over that duty.

# D0 — the blast radius of a gate in `dispatch_host_op_v1`

**This is the sizing question and it must be answered before the frame is
written.** The Architect measured the **production** callers exactly — the
interpreter at `eval.rs:5593` and `:5844`, native at `abi_v1.rs:2025`. But
`dispatch_host_op_v1` has roughly seventeen call sites across four crates:

    crates/ken-host/src/abi_v1.rs              :2025  :4549
    crates/ken-interp/src/eval.rs              :5593  :5844  :7168  :7200
                                               :7234  :7264  :7316  :7641  :8428
    crates/ken-runtime/src/native_effect_v1.rs :102
    crates/ken-runtime/src/object_linker_packaging.rs :3802 :3834
    crates/ken-verify/src/scenario.rs          :4988

A gate inside the dispatch function changes behaviour at **every** one of them,
not only at the three production sites. **The enclosing binding of the other
fourteen is UNRESOLVED** — a nearby `#[cfg(test)]` attribute is evidence about
the item it decorates, not about a line several thousand below it, and reading
it as a module boundary is the error this node should not make in its own
framing.

**D0:** resolve the enclosing binding of each of the fourteen, then determine
which of them dispatch an op that is `RepresentedUnavailable` today. Any that
do will change behaviour under the gate. That set is the node's real size; if
it is large or crosses into `ken-verify` production paths, the cut is wrong and
comes back to the Steward rather than being absorbed.

# D1 — are the other nine ops currently reachable?

`MappingAcquireFile` is one of ten. The others are `ClockMonotonicNow`,
`ClockSleepUntil`, `FsSeek`, `FsSetLength`, `FsSync`, `FsGetInheritance`,
`FsSetInheritance`, `FsDuplicate`, `EntropyRandomBytes`
(`effect_v1.rs:165-194`).

**Whether any of the nine is reachable from Ken source through the interpreter
today is UNMEASURED, and this node must not assume either answer.** No
`..._unavailable` prelude stub of the deleted form survives at `d4e977a6a`, but
absence of that *spelling* is not absence of enforcement — the concept may be
spelled another way, or the op may simply have no Ken-level surface to be
invoked from, which refuses by inaccessibility rather than by a guard.

This bears directly on urgency and on how the runtime-implementer's question is
answered: *"bounded until the flip"* bounds the divergence for **one** op. If
any of the nine is reachable, there is a live divergence the flip does not
touch. Report a count and the method; a zero from a grep that could not have
fired is not a measurement.

# The decisive consequence, recorded so it is not re-derived

`conformance/surface/ffi-io/seed-mapping.md:53` is BLOCKED-ON-ABI-S6-D5b, so
someone is about to write that fixture. Without the gate, it **passes on the
interpreter while native refuses**, and the conformance suite reports
availability the system does not have. A green that does not mean what it says
is worse than a red — worse here than the underlying divergence, because it
converts the divergence into positive evidence of correctness.

**Architect correction, carried by name** (`evt_21f23zmgqfxsc`): the earlier F4
advice to @spec-leader that the seed "wants re-examining for unblocking once
this lands" is **cancelled**. Slice 4 landed the surface; it did not make the
operation available. `seed-mapping.md:53` stays BLOCKED-ON-ABI-S6-D5b until the
FLIP slice, not this one and not slice 4.

# Not this node

- The availability flip itself — [[RT-D5B-MAPPING-AVAILABILITY-FLIP]]. This node
  blocks it; it does not contain it.
- The `ResourceTableV1` lifecycle cluster —
  [[RT-D5B-RESOURCE-TABLE-LIFECYCLE]].
- The `consumed_mapping_is_actually_unmapped` test-soundness defect —
  [[RT-MAPPING-UNMAP-PROBE-SOUNDNESS]].
- Any rewording of `effect_v1.rs:249-251`. The ruling says explicitly that the
  name is not doing double duty.
- Removing the native gate at `abi_v1.rs:1551`. It stays, by ruling.

# Sizing / tier

**Size S pending D0, tier T1.** The diff is plausibly a few lines plus one
test. The tier is T1 because the review turns on an argument, not on
byte-faithfulness: whether the control's predicate genuinely derives its set,
whether the gate's placement preserves the layering the ruling describes, and
whether the fourteen unresolved call sites were resolved rather than assumed.
**If D0 returns a large blast radius, the size is wrong and the recut is the
Steward's.**

# Contention

`crates/ken-host/src/effect_v1.rs`, shared with
[[RT-D5B-RESOURCE-TABLE-LIFECYCLE]] and the future flip slice. Sequence against
whichever of those is in flight; check each node's `status:` at `origin/main`,
not for a branch ref. Possible incidental contention in `ken-interp`,
`ken-runtime` and `ken-verify` depending on D0's answer — which is another
reason D0 precedes framing.
