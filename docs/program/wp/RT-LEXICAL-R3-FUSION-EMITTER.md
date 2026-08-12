# RT-LEXICAL-R3-FUSION-EMITTER — the one expression that needs fusion

Owner: runtime. Size: M. Node: [[RT-LEXICAL-R3-FUSION-EMITTER]].

**Fixed inputs measured at `origin/main` = `5a794bff`.** Line numbers are
anchors to re-find at your own base, not values to trust. Re-derive your
merge-base from `origin/main`; **do not take a SHA from this frame.**

**Seat tier: T2 build ring.** Architect votes at merge. **No Spec vote** if your
diff stays in `crates/`.

## 1. What this node owns, exactly

**One expression:** row 5's **before-hole** member of the eight-expression
lexical-recursor consumer population — `selected_scope` before the hole.

It is carved out of [[RT-LEXICAL-RECURSOR-CONSUMERS]] (`#6d`) **together with
its repair and its discriminating-control obligations.** That coupling is the
point of the carve-out, not a detail of it: Architect ruling `evt_7knsqyqg72103`
states that moving the machinery while leaving the expression in `#6d` would
give `#6d` an acceptance surface it cannot discharge. **If you find yourself
repairing an expression that is not this one, stop — it belongs to the parent.**

Seven siblings do **not** need fusion and are not yours: rows 1 and 4 are at the
`StaticWorkerBinding` wall after `D2a`; row 3 is at its retained
singular-specialization wall after `D2b`; row 5's **after**-hole expression is
at the `StaticWorkerBinding` wall; row 2 belongs to
[[RT-LEXICAL-ROW2-MISSING-MINT]].

## 2. Why fusion is genuinely required here, and is not an optimization

Under constraints already settled and **not reopened by this node**:

- the producer owner **lacks the downstream call arguments**, so eager forcing
  changes CBV;
- the recursor closure is a **live activation/cursor**, so representing or
  transferring it weakens `#6d`'s `AC-3` guard 2 (a closure is never made
  boundary-transferable);
- the producer and its exact consuming suffix live in **different units**.

⇒ The ruled lawful repair is **one planner-identified producer-plus-suffix
emission region**. Suppressing that fusion must restore the measured refusal —
that is `AC-3` below, and it is the control that distinguishes a fusion that
does work from one that is merely present.

## 3. THE TRAP — read this before you plan, it has already cost this work once

> ### `px8j`'s before-hole compile CANNOT carry an oriented plan
>
> **It is not your positive fixture.**
>
> `px8j` is a **seed-lane** compile deliberately preserved as the **unmarked
> negative**: no checked frame, no selected-IH slot, no checked-IH-invocation
> marker. `test_objects.rs:70` passes a literal `None` for
> `oriented_subcontinuation_plan`, and production oriented plans are decoded
> from a checked package's metadata (`planning.rs:144`) — a seed-lane compile
> has no metadata to decode. The `oriented` gate
> (`planning/static_transition.rs:8901`, `:9058`) then returns an empty plan
> **before any candidate enumeration runs.**
>
> Measured on the exact witness: `planes=[0] oriented_present=[false]`. One
> production compile reached the builder and resolved **zero**.
>
> **An earlier version of the parent frame pinned acceptance to this witness.
> That was my defect, not the ring's**, and it made `AC-1`/`AC-2` unsatisfiable
> — an emitter built against it would discharge its no-activation criterion
> **vacuously**, because a proof over nothing emitted passes for free.
>
> ⇒ **The positive fixture is the landed `D2g`/`D2j` checked `R3`-shaped twin**,
> with its own independently authored `OrientedSubcontinuationPlanV1`, consumed
> through the **one hoisted `#[cfg(test)]` constructor** and entered through
> `compile_expr_into_object_module` with `Some(oriented)` — **never** by calling
> the builder or emitter directly. **`px8j` is retained as the absence /
> ordinary-refusal comparator** and must never again be described as the
> fusion-positive.

**Forbidden routes, already ruled out** (Architect `evt_6vf66hmwv52y6`) and
listed so they are not re-derived as fresh ideas: no `Some(plan)` handed to
`px8j_capture_source_trace`; no synthesized default plan; no marker inference
from the Runtime shape; no weakening of the required checked-transport key
member; and **no making fusion independent of `oriented`**, which would reopen
`D2h`'s soundness-bearing key re-derivation.

## 4. The interior seam, located — do not re-derive it

The per-phase authority switch **has no function-level boundary left to sit
at**, and that is a consequence of a forced design choice, not an oversight.
Collapsing producer and consumer lowering into the single fused dispatcher call
was forced — lowering the producer to a value first relocates its own
"in-flight activation" refusal into the fused function — and the collapse
deleted the boundary the switch occupied.

⇒ Per the Runtime implementer's located answer (`thr_2wp6pehk4ybgk`), which
this frame adopts so the turn starts from it:

- **The switch site is inside the eliminator's case-body lowering**, keyed on
  the frame whose `static_origin` is a claimed fusion's `continuation_origin`.
  That is the only place the consumer's case body is identifiable.
- **The fused body's checked-frame adoption belongs at that same interior
  point**, not at the fused function's entry.

Architect ruling `evt_4vqey13cxxjqs` (durable at `51e6a266`) governs the
semantics: the fused function opens its **own fresh per-`Function` scope** and
re-enters the **consumer** frame identity; authority runs
producer → consumer → producer **per phase, and is NEVER `Fusion`**. `Fusion`
remains region/definition identity only.

**Binding, carried from the parent:** never assert `frame_origin` equality
(`25 != 10`). The carrier gate covers **one** function.

## 5. Deliverables

**`D0` — the gate, committed BEFORE any emitter definition.** Three rows: the
old negative at resolved plane `0`; the checked positive at resolved plane `1`
with **exactly one** key/ID/descriptor; and a one-marker-stripped exact
validator refusal. **No emitter AC may be credited until the positive row is
non-zero.** This deliverable exists because every prior control that reached a
fusion candidate used a synthetic fixture and called the builder directly —
**none of them compiled the acceptance fixture**, so the mechanism was untested
against it on *both* sides of the gate.

**`D1` — the interior authority switch**, at the seam located in section 4.
**Implemented and committed, then HELD.** Not routed to QA, not merged alone.

**`D2` — the fusion-specific checked-frame adoption**, at that same interior
point. Built on top of the held `D1`, and **held with it.**

**`D3` — arm the emitter.** Flip `D2F_EMITTER_ARMED`. **This is the final
implementation step**, taken only after `D0`'s positive row is non-zero and
`D1` and `D2` are implemented. **Arming is authorized by this frame and by
nothing you inherit from `#6d`.**

**`D4` — the before-hole expression is repaired and green** under `B`-only
exclusion at the pre-retirement base. **If arming plus `D2` necessarily makes
`D4` green, it belongs in the same candidate** — do not cut another inert
increment to carry it.

> ### `D1`+`D2`+`D3` MERGE AS ONE ATOMIC CANDIDATE — 2026-08-12
>
> **Architect ruling `evt_4m0q1m4zn4k79`, on exact `33a77bd4`. `D1` may not
> merge unexercised, and a pre-arm decline is not its control.**
>
> The `D1` mechanism as built is structurally coherent — the claim supplies
> `(continuation_origin, consumer_owner)`, the selected composed case-body seam
> matches `eliminator.static_origin`, `AmbientBodyAuthority` installs the
> consumer's `Predeclared` owner and unit, both `Ok` and `Err` restore the
> producer facts, `Fusion` is never authority, and there is no signature,
> planner or ABI expansion. **But with the installer unarmed the production
> population is empty, the field stays `None`, and none of that behaviour
> fires. Green compilation of that state proves only that inert scaffolding
> compiles.**
>
> **Binding ordering.** Implement `D1`, hold it. Build `D2` on top, hold it.
> Make `D3` arming the last implementation step. Then route **one** review
> candidate spanning `D1`+`D2`+`D3` (plus `D4` if arming makes it green), whose
> controls exercise the real checked `D0` positive through
> `compile_expr_into_object_module`.
>
> **Forbidden:** routing `33a77bd4` alone to QA or merging it; any standalone
> inert `D1` or `D2` merge; a direct-builder fixture; a test-only setter for
> `fused_consumer_authority`. A pre-arm observation that the real seam is
> reached and declines is lawful **as diagnostic evidence only** and earns no
> `D1` or AC credit.
>
> **This changes merge granularity, not semantic order.** `D1` and `D2` are
> still implemented before the arm, no `main` state ever carries the arm
> without both, and the control becomes constructible only inside the armed
> range.
>
> **What was superseded is this frame's own `D3` clause** — *"`D1`/`D2` are
> landed with their controls"*. It was **infeasible as written**, because
> `D1`'s real control needs `D3`'s non-empty population. Do not reconstruct it
> from a memory of this frame.
>
> **Section 9's sizing target still applies per increment.** The atomic unit is
> the **review candidate**, not the turn: `D1`, `D2` and `D3` remain separate
> commits with separate handbacks to the leader. This adds no review hop and no
> new party.

## 6. Acceptance criteria

**AC-1 — the positive is a real full-pipeline compile.** The checked `D2j`
`R3`-shaped twin compiles through `compile_expr_into_object_module` with
`Some(oriented)` and resolves **plane 1** with exactly one key/ID/descriptor.
*Control:* the committed row, with its own freshly derived coordinates. **Do
not cite origin-23 or any other old `px8j` coordinate** — those were struck.

**AC-2 — suppressing the fusion restores the measured refusal.** *Control:* a
committed mutation that disables the fusion for this expression and reproduces
the exact refusal, with evidence the detector was reached. **This is the AC the
node exists for.** A fusion that is present but doing nothing passes every
criterion except this one.

**AC-3 — the five parent guards are intact**, unchanged from `#6d`: 
`RecursiveBackedge` stays protocol-only; a closure is never made
boundary-transferable; an actual non-constructor computational scrutinee still
refuses; source-join closeout still rejects an un-emitted/unselected join; a
missing recursive-IH authority still refuses. *Control:* a committed negative
witness per guard, **each with a positive control proving its path is reached.**

**AC-4 — `px8j` still refuses ordinarily**, as the absence comparator, and its
refusal is asserted as such rather than as an incidental pass.

**AC-5 — no banned mechanism.** No fallback to `RecursiveDescent`, no
`BoundaryUse`, no `PlannedEffectSeat` widening, no lowering-minted token, no
invocation-local activation/resume/return-hole state in ABI data. *Control:*
name the ABI payload at each new crossing and show ordinary typed fields.

**AC-6 — zero new `#[ignore]`**, and no tracker `status:` change in the
candidate. *Control:* `git diff`.

**AC-7 — CI green** on the merge. Not a local `--workspace` run
(`COORDINATION §12`).

**AC-8 — the live authority control, on the atomic candidate.** Architect
`evt_4m0q1m4zn4k79`; this is the AC that the atomic ordering exists to make
constructible, and **it must run from the actual armed production compile**,
not from a fixture.

- **One installed fusion definition and one region-keyed switch** are asserted
  from that compile.
- **The live authority fields are observed in three positions:** producer
  `Predeclared` **before** the consumer phase, consumer `Predeclared` **during**
  the exact selected case body, producer **restored after** it. **No `Fusion`
  authority at any of the three.**
- **The error control forces an error *after* the switch, through that same
  production path**, and observes producer restoration **before the error
  propagates.** An error injected anywhere else does not discharge this.
- **`D0`'s plane and refusal rows are retained**, along with this frame's guard
  controls. `AC-8` is added to them, not substituted for them.

## 7. Excluded scope

- **Retirement of the residual class and any lane deletion.** That is
  [[RT-RECURSOR-TRANSPORT]] and [[RT-DESCENT-RETIRE]].
- **The other seven expressions**, including row 5's after-hole member.
- **`D2h`'s key re-derivation.** Soundness-bearing and not reopened.
- **Unwinding any of the eleven landed `D2f` partials.** They are the
  substrate; they are inert and correctly labelled.
- **A direct-builder fixture, and a test-only setter for
  `fused_consumer_authority`** (Architect `evt_4m0q1m4zn4k79`). The first is
  the exact defect `D0` exists to correct; the second manufactures the state
  whose real arrival is the thing under test. **If `AC-8` seems to need
  either, the candidate is not yet armed** — that is a sequencing answer, not a
  fixture problem.
- **A standalone inert `D1` or `D2` merge.** See the atomic-candidate ruling in
  section 5.

## 8. Stop conditions — return to me, do not decide

- **`D0`'s positive row will not go non-zero** through the lawful route. That
  is this frame's central premise failing, and it is the same wall that stopped
  the work before — **do not repair it by supplying the plan.**
- **The interior seam turns out not to be where section 4 says**, or the switch
  cannot be expressed there without a signature change rippling beyond the
  eliminator.
- **A guard in `AC-3` cannot be preserved** under the fused lowering.
- **The repair needs a new planner/ABI population beyond what is landed.**
  This stop is written against the *cumulative* state, not against a single
  increment — that is how the parent's identical stop failed to fire across
  eleven partials, each of which was individually small enough not to trigger
  it. **Ask it at every cut, about the running total.**

## 9. Contention and sizing

`crates/ken-runtime/src/cranelift_backend/lowering/core*`, `.../units.rs`, and
the eliminator's case-body lowering path.

**Runtime runs one node at a time.** [[RT-LEXICAL-ROW2-MISSING-MINT]] held this
file set and **merged 2026-08-12** at `main` `741f66c3` — that sequencing bar is
cleared and this node is the released successor. Re-derive the intersection at
candidate time anyway: a merge-base goes stale without your branch moving, and
row 2's arc landed nine PRs into `core/tests/control.rs`.

`scripts/ken-cargo test -p ken-runtime --lib` plus your focused suite.
**Never `--workspace`**; that is CI's gate.

**Sizing note.** The parent measured this emitter increment plus its review
cycle at **closer to one working day**. That estimate predates the interior-seam
discovery, so treat it as a floor. If you reach a hard stop inside an hour, that
is a good outcome — say so and hand back.
