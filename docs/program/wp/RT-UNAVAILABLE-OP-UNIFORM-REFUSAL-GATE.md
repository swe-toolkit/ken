# WP frame — `RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE`

**Owner:** Team Runtime · **Size:** S, pending D0 · **Risk:** medium (the gate
sits in a function ~17 call sites traverse) · **Tier:** T1 · **Gate:** none ·
**Deps:** none — this node blocks
[[RT-D5B-MAPPING-AVAILABILITY-FLIP]], not the reverse

**Origin:** adversary Finding 1 on the landed slice 4 (*"statements !=
enforcements"*), routed by the Steward to the Architect as a design question
rather than ruled, and **RULED** at `evt_21f23zmgqfxsc`. Cut as its own node on
the Steward's sequencing ruling; the Architect adopted the split at
`evt_3ws5c4xzbfxa5` (*"my heading was wrong, the paragraph is the ruling"*).

## 1. Objective

Make *"`RepresentedUnavailable` implies refused"* an **enforced** invariant
rather than a **stated** one, by gating `dispatch_host_op_v1` at the convergence
both executors traverse, and adding one control that derives the unavailable set
from `availability()`.

**This is REMEDIAL.** Four of the ten unavailable ops execute interpreted and
refuse natively **on `main` today**. It is not a hardening of a pattern against
future unflipped surfaces — that is true, and it is the smaller half.

## 2. Fixed inputs, measured at `origin/main` `d4e977a6af1083975665e587ed7e3e31f733785e`

    dispatch_host_op_v1          crates/ken-host/src/effect_v1.rs:2628
    native gate (STAYS)          crates/ken-host/src/abi_v1.rs:1551
    interpreter production call  crates/ken-interp/src/eval.rs:5593  in fs_dispatch      (:5147)
    interpreter production call  crates/ken-interp/src/eval.rs:5844  in ambient_dispatch (:5833)
    native production call       crates/ken-host/src/abi_v1.rs:2025
    availability()               crates/ken-host/src/effect_v1.rs:158-196, 35 arms,
                                 no wildcard -> 25 NativeTested / 10 RepresentedUnavailable
    HostOpAvailabilityV1         :694-697, exactly two variants, so the partition
                                 is total and no op can hide in a third class

The four reachable ops, their traces, and the six that refuse by inaccessibility
are in the node's **D1 — ANSWERED** section. Read it; do not re-derive it.

## 3. D0 — THE FIRST ACT, AND A LEGITIMATE PLACE TO STOP

**Do this before writing any gate code, and report it even if the answer is
boring.**

`dispatch_host_op_v1` has roughly seventeen call sites across four crates. Three
are production and are listed in §2. **The enclosing binding of the other
fourteen is UNRESOLVED**, and a gate inside the dispatch function changes
behaviour at every one of them.

    crates/ken-host/src/abi_v1.rs                     :2025  :4549
    crates/ken-interp/src/eval.rs                     :5593  :5844  :7168  :7200
                                                      :7234  :7264  :7316  :7641  :8428
    crates/ken-runtime/src/native_effect_v1.rs        :102
    crates/ken-runtime/src/object_linker_packaging.rs :3802  :3834
    crates/ken-verify/src/scenario.rs                 :4988

**Resolve the enclosing binding of each — do not infer it from a nearby
`#[cfg(test)]`.** An attribute decorates the *item* beneath it, which may be a
single `fn`; it is not a module boundary, and a line three thousand below it is
not covered by it. The Architect established `decode_clock_request` as production
this way: the `#[cfg(test)]` at `eval.rs:4449` decorates the single fn
`io_error_value`.

Then: **which of them dispatch an op that is `RepresentedUnavailable` today?**
Those change behaviour under the gate.

**HARD STOP, and it is a good outcome (§4b).** If the answer reaches production
paths in `ken-runtime` or `ken-verify`, or is large enough that the remedy stops
being a few lines plus a test, **stop and report to the Steward.** The cut is
then wrong and the recut is the Steward's, not yours to absorb. Reaching that
stop inside an hour is a successful turn.

## 4. RULED, and not open to re-litigation

From `evt_21f23zmgqfxsc` and `evt_3ws5c4xzbfxa5`:

1. **The invariant is `RepresentedUnavailable` implies REFUSED BY EVERY
   EXECUTOR.** "Permissive by design" was considered and rejected. The name is
   not doing double duty and `effect_v1.rs:249-251` does **not** need rewording.
2. **The gate goes in `dispatch_host_op_v1`.** The native gate at
   `abi_v1.rs:1551` **stays** — it is at the FFI boundary and returns an
   ABI-shaped code, correct for that layer. Double-gating is defense in depth.
3. **Do NOT relocate the gate to a caller.** The interpreter's two production
   callers are in *different helpers*. A gate in `fs_dispatch` — where the
   `MappingAcquireFile` finding pointed, and the obvious place — covers the op
   that started this and **misses clock and entropy entirely.** If you find
   yourself reasoning toward a caller, that is the failure this clause exists to
   catch.
4. **The refusal's shape at this layer is yours to propose, not to invent
   silently.** The layering argument says the ABI-shaped code belongs to
   `abi_v1`; what `dispatch_host_op_v1` returns is a design call inside the
   ruling, so state it in the PR body with its reasoning. If it is not obvious
   from the surrounding code, ask the Architect rather than picking.

## 5. Acceptance

**AC-GATE. A `RepresentedUnavailable` op is refused on the interpreter path.**
Demonstrated, not asserted: a test that goes **red without the gate**. A test
that passes both with and without it measures nothing.

**AC-PREDICATE. The control derives its set; it never lists it.**

    for every op in HostOpV1::ALL with availability() == RepresentedUnavailable,
        the interpreter path refuses it

Ten ops satisfy that today. **An op-specific test is a defect, not a smaller
version of this** — it reproduces exactly the enumeration failure already filed
against `AC-AVAIL`, and it would say nothing about the next unflipped surface.
**Control on the control:** adding a hypothetical eleventh unavailable op must be
covered with **zero edits** to the test. If satisfying that needs the test
amended, the set is being listed somewhere.

**AC-NATIVE-UNTOUCHED. `ClockWallNow` and the other 24 `NativeTested` ops are
unaffected.** The predicate gives this by construction — it derives the refusal
set from `availability()` rather than naming it — so the AC is discharged by the
control's *shape*, and a candidate that needs a separate allow-list to satisfy it
has the wrong shape.

**AC-D0. D0's determination is in the PR body**, with the fourteen call sites
resolved by enclosing binding and the dispatching-an-unavailable-op subset named.
A count alone does not discharge this; name them.

**AC-NO-REGRESSION. Workspace-green in CI**, never a local `--workspace` run.
Local verification is targeted only, through `scripts/ken-cargo`: `-p ken-host`,
`-p ken-interp`, and — **depending on D0** — `-p ken-runtime`, `-p ken-verify`.

> **A suite total is not evidence about a test whose subject it does not
> exercise.** `122/0` on `ken-host` was reported as a gate result for slice 4 and
> carried no information about the one test then in question. When you quote a
> total, say which of these ACs it bears on.

## 6. Contention

`crates/ken-host/src/effect_v1.rs`, shared with
[[RT-D5B-RESOURCE-TABLE-LIFECYCLE]] (node cut, routed, **implementation not
started**) and [[RT-D5B-MAPPING-AVAILABILITY-FLIP]] (draft, unsized, blocked on
this). **This node goes first** — it is remedial and small; the lifecycle
transplant is neither urgent nor started.

D0 may add `ken-interp`, `ken-runtime` or `ken-verify`. Zero cranelift expected;
**confirm rather than assume** — `git diff --name-only <base>..<cand> | grep -c
cranelift` is `0`.

## 7. What this node is NOT

- **Not a stronger availability census.** Its subject is **REACHABILITY**.
  Every `AC-AVAIL` site is a native-availability site, so the corrected
  membership predicate — a real improvement over the enumeration it replaced —
  **still passes against this defect.** The census was complete and answering a
  different question than it was read as answering. These controls sit
  *alongside* `AC-AVAIL`, never inside it, and anyone folding them together will
  drop one.
- **Not the flip.** `MappingAcquireFile` stays `RepresentedUnavailable` here.
- **Not an unblock of `conformance/surface/ffi-io/seed-mapping.md:53`.** That
  stays BLOCKED-ON-ABI-S6-D5b until the **flip**. The Architect cancelled the
  earlier F4 advice to the contrary by name.
- **Not a rewording of `effect_v1.rs:249-251`**, and **not** a removal of the
  native gate at `abi_v1.rs:1551`.
