---
id: RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE
title: "Enforce the RepresentedUnavailable invariant uniformly across BOTH executors by gating dispatch_host_op_v1 at the convergence, with a control that is a PREDICATE over availability() rather than an assertion about any named op. The invariant is STATED at effect_v1.rs:193 and native-enforced at abi_v1.rs:1551, but the interpreter consults availability() nowhere -- so of the ten unavailable ops, THREE reachable via the op_* intern table (ClockMonotonicNow, ClockSleepUntil, EntropyRandomBytes) PLUS ONE reachable via the separately interned PrivateMappingAcquireFile global (MappingAcquireFile) execute interpreted and refuse natively on main TODAY, and the availability flip moves only that last one. State the split, never the sum of four: the obvious re-derivation reads the intern table alone and returns three, dropping the very op the flip is about. REMEDIAL, not preventive. Architect ruling evt_21f23zmgqfxsc: the interpreter MUST refuse; RepresentedUnavailable is a language-surface claim, not a native-backend one. The gate must sit at the convergence and NOT in a caller -- the interpreter's two production callers are in different helpers (fs_dispatch, ambient_dispatch), so the natural-looking fs_dispatch placement misses clock and entropy entirely. Its SUBJECT IS REACHABILITY, a different question from AC-AVAIL's availability census -- do not fold the two together."
status: ready
owner: runtime
size: S/M
gate: none
depends_on: []
blocks: [RT-D5B-MAPPING-AVAILABILITY-FLIP]
github: null
tier: T1
origin: "Adversary Finding 1 on the landed slice 4 (statements != enforcements), routed by the Steward to the Architect as a design question rather than ruled; Architect RULED evt_21f23zmgqfxsc. Steward cut 2026-09-16 as its OWN node rather than as a rider on the flip slice -- a sequencing/packaging call (steward.md §3), not a departure from the ruling's design content, which is adopted verbatim. Fixed inputs measured at origin/main d4e977a6af1083975665e587ed7e3e31f733785e."
---

> # READY, and RE-FRAMED 2026-09-16 after D0. Frame:
> `docs/program/wp/RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE.md`.
>
> The design is RULED and is not open. **D1 is CLOSED** (three via the intern
> table + one via `PrivateMappingAcquireFile`), which makes this node REMEDIAL —
> it closes a divergence live on `main` today, not a hazard that future
> unflipped surfaces might create.
>
> **D0 is ANSWERED and must not be re-run.** It completed, hit hard-stop
> condition 2, and the stop was **withdrawn**: the executing turn reported
> rather than absorbed (`evt_4w12zca5j1g03`), the Architect closed the design
> fork it raised with a third option — **split the entry**, not relocate the
> gate and not delete the arms (`evt_1y4rvywv1y6fr`) — and the Steward's funnel
> measurement showed the remedy moves 21 call sites while touching **zero**
> setup lines and **zero** assertions (`evt_5sxcg2m9qah9j`). Size S to **S/M**;
> no recut, no blocking node. **D0's results are now fixed inputs in the frame's
> §3.** Per the sequencing ruling below, this lands BEFORE the availability flip.

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

   **The convergence is load-bearing, and the natural-looking alternative
   silently misses three of the four reachable ops.** The interpreter's two
   production callers do not sit in one helper: `eval.rs:5593` is inside
   `fs_dispatch` (`:5147`) and `eval.rs:5844` is inside `ambient_dispatch`
   (`:5833`), reached from two different arms of the evaluator. A gate in
   `fs_dispatch` — which is where the `MappingAcquireFile` finding pointed, and
   the obvious place to put it — covers the op that started this and **misses
   clock and entropy entirely** (Architect, `evt_3ws5c4xzbfxa5` point 4). The
   ruling names the convergence; the two-helper split is *why*, not merely that
   one location is tidier. **Do not relocate this gate to a caller.**
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

# D0 — ANSWERED 2026-09-16. Do not re-run it.

The blast radius is measured and closed. Full results are **fixed inputs in the
frame's §3**; the headline, and the two things worth carrying at node level:

**The call-site population is CLOSED at 96** — 15 outside `effect_v1.rs`, 81
inside its test module, no integration-test tail, swept across all 505 `.rs`
files under `crates/`. **None of the 15 outside dispatches a
`RepresentedUnavailable` op today**, so no production path changes behaviour,
and `ken-verify` has zero production dispatch sites. The earlier count of
"roughly seventeen, fourteen unresolved" was the pre-measurement estimate.

**One production site the earlier list under-resolved:**
`ken-runtime/src/native_effect_v1.rs:102` is production — it compiles with no
`cfg` — and has **zero production callers**; `#![allow(dead_code)]` at `:7` is
what stops rustc reporting it. It is a latent surface governed automatically by
the gate's predicate the day it gains a caller. Falsifier: *any production
reference to the module outside its own file flips this.*

**The method, recorded because a text match gets it wrong.** Resolve each site
by **`cfg` satisfiability under `test=false`**, never by grepping for
`cfg(test)`. `scenario.rs:4988` sits under `#[cfg(all(test, target_os =
"linux"))]`, which a `cfg(test)` text match does not fire on — reading it as
production is a false trip of the hard stop. This is the same hazard the node
warned about one level up: an attribute decorates the item beneath it.

**Why D0 stopped, and why the stop was withdrawn.** D0 hit hard-stop condition
2 — eight tests, ~900 setup lines, 66 assertions dispatching unavailable ops,
**six of the eight asserting the executing arm**, which a naive gate deletes.
That was the correct call and the turn was a success. It raised a design fork,
the Architect closed it with a third option (**split the entry**: gate stays at
the convergence, the body below becomes a `pub(crate)` inner entry, all ten arms
stay live), and the Steward's funnel measurement then showed the remedy
re-points **21 call sites** and moves **zero** setup lines and **zero**
assertions. Condition 2 does not bind under the ruled design.

**The constraint the Architect's D1 measurement adds** (`evt_3ws5c4xzbfxa5`
point 5) stands unchanged: a gate at the convergence must leave `ClockWallNow`
and the other 24 `NativeTested` ops **untouched**. The predicate control gives
that by construction.

# D1 — ANSWERED: FOUR of the ten are reachable today. This node is REMEDIAL.

**Closed 2026-09-16** by runtime-implementer (`evt_228edjqb8z29m`, the partition
and its key) and the Architect (`evt_3ws5c4xzbfxa5`, the traces). Measured at
`origin/main` `d4e977a6af1083975665e587ed7e3e31f733785e`.

    ClockMonotonicNow    reachable, EXECUTES interpreted, refused natively
    ClockSleepUntil      reachable, EXECUTES interpreted, refused natively
    EntropyRandomBytes   reachable, EXECUTES interpreted, refused natively
    MappingAcquireFile   reachable, EXECUTES interpreted, refused natively
    FsSeek FsSetLength FsSync FsGetInheritance FsSetInheritance FsDuplicate
                         no Ken-level surface in prelude.rs under any of the
                         three spellings -- refuse by INACCESSIBILITY, not by
                         a guard. Re-check on any op that later gains a surface.

Trace coordinates, so no reader re-derives them:

    Ken surface       prelude.rs:592  data ClockOp = WallNow | MonotonicNow
                                                   | SleepUntil Deadline
                      prelude.rs:594  data EntropyOp = RandomBytes Int
                      handlers at prelude.rs:1653, :1659, :1678
    decode            eval.rs:4576    decode_clock_request (production -- the
                                      #[cfg(test)] at :4449 decorates the single
                                      fn io_error_value, NOT a module)
    call site         eval.rs:6247    inside the EvalVal::Ctor arm
    helper            eval.rs:5833    ambient_dispatch
    host entry        eval.rs:5844    dispatch_host_op_v1, its SECOND statement
    implemented arms  effect_v1.rs:2895 / :2898 / :2902 -> backend.clock_*,
                                      backend.entropy_random_bytes
    native refusal    effect_v1.rs:558-569, the ten-op arm whose body is
                      return Err(TerminalErrorV1::OperationUnavailable(operation))

**The sharpest form, and the reason no further trace is owed.** `ClockWallNow`
is `NativeTested` (`:164`); `MonotonicNow` (`:165`) and `SleepUntil` (`:166`) are
not — three consecutive lines. All three are constructors of **one** Ken data
declaration, decoded by **one** function, on **consecutive branches of one
if-else chain** (`decode_clock_request` `:4581`/`:4586`/`:4592`). `WallNow`
proves the whole path is exercised end to end; the other two differ from it only
by which arm they take. The surface is not hypothetically live.

**Stated with its limit:** the backend is `InterpreterHostBackend { handler }`,
so what the handler ultimately does is the handler's business. The divergence
does not depend on that — the interpreter **executes and returns a value** where
native returns `OperationUnavailable`. Execute-versus-refuse is the property,
and it is settled.

⇒ **"Bounded until the flip" bounds nothing.** Four of ten are reachable and the
flip moves exactly one of them. **This node is REMEDIAL, not preventive:** it
closes a divergence live on `main` today that the flip leaves in place for three
ops. Frame it that way — "hardening a pattern for future unflipped surfaces" is
true and it is the smaller half.

**How D1 was nearly answered wrongly, kept because the gate's own control has
the same failure mode.** The naive census — grepping the ten Rust variant names
against `prelude.rs` — returns `MappingAcquireFile` 5, all nine others 0, which
reads as a clean "only one op has a Ken surface, the divergence is bounded." It
is an artifact of spelling: one concept carries three names with no mechanical
transformation between them, and the direction disagrees (`MappingAcquireFile`
*gains* a `Private` prefix as `PrivateMappingAcquireFile`; `ClockMonotonicNow`
*loses* its `Clock` as `MonotonicNow`). Even the five hits are substring matches
inside the Ken identifier, not the Rust variant. The authoritative key is the
prelude's own `op_* => "<KenName>"` intern table. **A census keyed on a spelling
its subject does not use returns a confident wrong number, and this node's
control must derive its set from `availability()` for exactly that reason.**

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

**Size S/M, tier T1 — sized on D0's measurement, not on an estimate.** The
production diff is the gate plus the entry split; the test diff re-points 21
call sites at the inner entry and changes nothing else. The tier is T1 because
the review turns on an argument, not on byte-faithfulness: whether the control's
predicate genuinely derives its set, whether the entry split keeps enforcement
**at** the convergence rather than relocating it outward, and whether the two
in-crate sites `pub(crate)` does not reach were dispositioned by name.

# Contention

`crates/ken-host/src/effect_v1.rs`, shared with
[[RT-D5B-RESOURCE-TABLE-LIFECYCLE]] and the future flip slice. Sequence against
whichever of those is in flight; check each node's `status:` at `origin/main`,
not for a branch ref. **D0 resolved the incidental contention question: the
expected diff is `ken-host` only** (`effect_v1.rs`, `abi_v1.rs`), because no
call site outside that crate dispatches an unavailable op and the out-of-crate
callers stay on the gated entry unchanged.
