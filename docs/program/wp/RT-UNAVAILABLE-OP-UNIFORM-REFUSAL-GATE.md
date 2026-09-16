# WP frame — `RT-UNAVAILABLE-OP-UNIFORM-REFUSAL-GATE`

**Owner:** Team Runtime · **Size:** S, pending D0 · **Risk:** medium (the gate
sits in a function **96** call sites traverse — 15 outside `effect_v1.rs`, 81 in
its test module) · **Tier:** T1 · **Gate:** none ·
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

## 2. Fixed inputs

All measured at `origin/main` `d4e977a6af1083975665e587ed7e3e31f733785e`.

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

**The two existing refusal sites, both producing `OperationUnavailable`:**

    effect_v1.rs:558-568   host_effect_wire_layout_v1 (fn begins :399)
                           ENUMERATES the ten by name -- see §4 point 5, the
                           enumeration is a deliberate build break, LEAVE IT
    abi_v1.rs:808-814      require_native_operation_v1
                           DERIVES from availability()

One stale artifact, recorded so nobody re-finds it: the comment at
`effect_v1.rs:555` says *"the **twenty-two** matched above are exactly the
`NativeTested` set."* Re-measured over the enclosing function — 35 variants
named, 10 in the refusal arm — **the number is 25, stale by three promotions.**
Only the prose is stale; the mechanism is correct. Fixing the comment is not in
this WP's scope and is not an AC.

## 3. D0 — THE FIRST ACT, AND A LEGITIMATE PLACE TO STOP

**Do this before writing any gate code, and report it even if the answer is
boring.**

`dispatch_host_op_v1` has **96** call sites. They split into two populations
with different D0 obligations, and **the larger one is the one D0's original
scope could not see.**

### 3a. Fifteen outside `effect_v1.rs`. Twelve have an UNRESOLVED binding.

    crates/ken-host/src/abi_v1.rs                     :2025  :4549
    crates/ken-interp/src/eval.rs                     :5593  :5844  :7168  :7200
                                                      :7234  :7264  :7316  :7641  :8428
    crates/ken-runtime/src/native_effect_v1.rs        :102
    crates/ken-runtime/src/object_linker_packaging.rs :3802  :3834
    crates/ken-verify/src/scenario.rs                 :4988

Fifteen listed; §2 already resolves **three** of them as production; **twelve**
remain unresolved. A gate inside the dispatch function changes behaviour at every
one of them.

**Resolve the enclosing binding of each — do not infer it from a nearby
`#[cfg(test)]`.** An attribute decorates the *item* beneath it, which may be a
single `fn`; it is not a module boundary, and a line three thousand below it is
not covered by it. The Architect established `decode_clock_request` as production
this way: the `#[cfg(test)]` at `eval.rs:4449` decorates the single fn
`io_error_value`.

Then: **which of them dispatch an op that is `RepresentedUnavailable` today?**
Those change behaviour under the gate.

### 3b. Eighty-one inside `effect_v1.rs`, resolved test-only by MEASUREMENT

**The obvious reasoning for that resolution is the reasoning §3a forbids**,
which is why the measurement is written out rather than the conclusion.

All 81 lie inside one `#[cfg(test)] mod tests`, and that is established by
**module extent**, not by attribute proximity:

    #[cfg(test)]  :4109   decorates   mod tests  :4110
    mod tests is the LAST column-0 item in a 9465-line file -> it runs to EOF
    all 81 call sites lie in :4649-:9429 -> inside it
    :2628 is the DEFINITION, not a call site

**`effect_v1.rs` is an INSTANCE of §3a's hazard, not an exception to it.** It
carries **four** `#[cfg(test)]` attributes and **three decorate a single `fn`**:

    :1179 -> :1180   fn is_native_mapped
    :1387 -> :1388   fn insert_fs_handle_without_provenance_for_test
    :2184 -> :2185   fn force_generation_for_test
    :4109 -> :4110   mod tests                    <- the only one on a module

Read the file as having "one `#[cfg(test)]`, on a `mod`" and you will be
fractionally less careful in the one file where that care is the deliverable.

**These tests dispatch unavailable ops today** — the Architect counts 107
references to the ten unavailable variants inside that module, including a table
at `:4827-4835` pairing `ClockMonotonicNow` and `ClockSleepUntil` with their
canonical requests. After the gate, calls like those stop reaching the arms at
`:2895`/`:2898`/`:2902`. **This is the single largest behavioural consequence of
the change.**

**D0 must NAME that population before any code is written**, and say for each
whether it is **asserting the dispatch arm's behaviour** (must be re-expressed
against the gate) or **using the op as a vehicle** (must be re-pointed at an
available op). D0 does not have to fix them. It has to mean that you meet them
as a named list rather than as a wall of red you triage under pressure.

### 3c. HARD STOP — and it is a good outcome (§4b)

Stop and report to the Steward if **either** holds:

1. resolving §3a reaches production paths in `ken-runtime` or `ken-verify`; or
2. **§3b's re-expression population is large enough that the remedy stops being
   a few lines plus a test.**

**Condition 2 is not a footnote on condition 1.** The test population can blow
this WP's `S` sizing without touching a single production path in either crate —
so "the production paths are clean" is *not* a finding that the cut is sound.
Sizing is the Steward's under §4b; **it is not yours to absorb silently.** The
cut is then wrong and the recut is the Steward's. Reaching that stop inside an
hour is a successful turn.

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
4. **The refusal's shape is RULED — it is not a design fork and needs no
   proposal.** (Architect, `evt_17w6mab5k1y4a`; this clause previously asked for
   a proposal.) The function's existing error type already carries the right
   variant with the right payload:

       effect_v1.rs:2628  -> Result<HostDispatchReplyV1, TerminalErrorV1>
       effect_v1.rs:4068     TerminalErrorV1::OperationUnavailable(HostOpV1)

   At the top of the function, before the capability match at `:2638`:

       if operation.availability() == HostOpAvailabilityV1::RepresentedUnavailable {
           return Err(TerminalErrorV1::OperationUnavailable(operation));
       }

   **The layering objection does not apply, because this is the value the other
   two refusal sites already produce** — `effect_v1.rs:558-568` and
   `abi_v1.rs:808-814`. The ABI-shaped *code* is `abi_v1`'s business and stays
   at `:1551`; `TerminalErrorV1` is what `ken-host` already speaks here.

   **Returning anything else would undercut the objective.** The invariant is
   *one* refusal for an unavailable op regardless of executor; two paths refusing
   with different values hold it in the weak sense and fail it where anyone would
   test it. **Same variant, same payload, both paths.** Cite this in the PR body;
   do not re-derive it.

5. **Do NOT convert `effect_v1.rs:558-568` to a predicate.** (Architect ruling.)
   That arm enumerates the ten by name and **the enumeration is the
   enforcement** — the comment at `:545-551` records that it replaced a `_`
   wildcard precisely so a new operation becomes `error[E0004]` instead of a
   runtime refusal. Deriving it from `availability()` would delete a build break.
   It looks like the WP's own value applied one screen further down. It is not.

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

> **This does not make enumeration wrong, and §4 point 5 is the case that
> proves it.** The governing rule (Architect):
>
> **Enumerate when the COMPILER's exhaustiveness check is the enforcement.
> Derive when a TEST is the enforcement.**
>
> A named match arm makes adding a variant a build break, which is the only way
> to get one. A test over a named list cannot fail for an op absent from the
> list — which is `AC-AVAIL`'s defect exactly. Both constructs pick what actually
> enforces; they differ because the enforcing mechanisms differ. **AC-PREDICATE
> governs the new test. It does not govern a match arm.**

**AC-NATIVE-UNTOUCHED. `ClockWallNow` and the other 24 `NativeTested` ops are
unaffected.** The predicate gives this by construction — it derives the refusal
set from `availability()` rather than naming it — so the AC is discharged by the
control's *shape*, and a candidate that needs a separate allow-list to satisfy it
has the wrong shape.

**AC-D0. D0's determination is in the PR body.** Discharged on **named items**,
and this AC states **no total** — deliberately. An earlier draft pinned
"fourteen", a number derived from a round estimate rather than from the list, so
a *correct* determination naming twelve would have failed it. **A count alone
does not discharge this; name them.** Required:

- each unresolved §3a call site resolved by **enclosing binding**, and the
  dispatching-an-unavailable-op subset named;
- the §3b `ken-host` test population named, split into *asserting the arm* and
  *using the op as a vehicle*.

Either one absent is the finding, whatever the totals say.

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
