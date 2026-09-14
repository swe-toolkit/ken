---
id: RT-CHECKED-IH-RESULT-OBLIGATION-REKEY
title: "HS15 recut: re-key the checked-IH result obligation from call-occurrence identity to the value's per-arm constructor identity, discharged inside the existing selected case block from case_constructor_identity(static_origin, index), so certification follows the consumer per arm -- closing HS15 with no exact-provenance recovery, no producer split, and no new planner mechanism"
status: ready
owner: runtime
size: M
gate: none
depends_on: []
blocks: []
github: null
tier: T1
origin: "Architect HS15 ruling evt_5sddep840sxcj (2026-09-14), the §1b structural-closure recut of the ABI-S6 HS18 arc -- explicitly a recut, NOT amendment 9. Gated on and unblocked by the runtime reachability measurement evt_6h85t50nkg73 (reachable without reshaping lowering; the refuting branch is absent). Prior-art advisory evt_13cg63dwa86yg + evt_5t2xz56yv9bjm; inventory disposition ABI-S6 entry 28 (landed 0b2e5446). Fixed inputs measured on the HS18-arc WIP b601e2ec7; re-measure the cited coordinates at D0. Steward-filed (agents cannot create tracked work, COORDINATION §2); the design is the Architect's ruling and the Architect is the required reviewer on the mechanism."
---

> # RELEASED 2026-09-14 — HS15 §1b structural-closure recut, FENCED. `ready`.
>
> The disposition of HS15 is ruled (Architect evt_5sddep840sxcj) and the one
> pre-build gate is cleared: the runtime reachability measurement
> (evt_6h85t50nkg73) proved the per-arm discharge site reachable inside each
> existing `selected` block WITHOUT reshaping lowering, so the ruling's refuting
> branch (discharge structurally pinned upstream of block selection) is absent.
> Released to the runtime ring. Architect is the required reviewer on the
> mechanism; runtime-qa reviews; Adversary reviews (crates/). Built on the
> HS18-arc WIP `b601e2ec7` — re-measure every cited coordinate at D0.

## What this is

The structural closure of HS15. The checked-IH **result** obligation is today
keyed on the **call occurrence** — the `ContinuationCallIdentity` receipt minted
for the `RoutedAnswer` upstream of head/case selection. HS15 proved (full-token
measurement, entry 28) that this receipt is a **union**: both constructor arms
(`Ret` -> 4442/38, `Vis` -> 3380/38) descend from ONE identical identity
(`producer_alternative` 1/1, `producer_construct_origin` 1617/1617,
`call_site_sequence` 0/0). Applying `RequiredConsumerIncomingEdge`
unconditionally to that seat is what put a **compile-time-refusable mismatch
into a runtime trap** (Trap 43).

**This node re-keys the obligation off the union.** The obligation stops being
keyed on the call occurrence and is keyed on the **value's constructor
identity**, discharged **inside the case arm**, where that identity is a planner
fact from `case_constructor_identity(eliminator.static_origin, index)`. The
runtime tag **selects** the block; it never **supplies** the identity. That is
the same footing this lowering already gives case **arity** three lines above
(the bound is the declared `argument_binders`, "NOT anything read off the
carried word") — so the distinction is **checkable rather than asserted**.

## The §1b predicate, and why this is value-keyed not seat-keyed

Inventory entries 24-27 established one predicate: every attribution in the arc
was **seat-keyed where the fact is value-keyed**. The closure does not recover
provenance and does not split the producer; it moves the obligation onto the
value's own **static** constructor identity. A constructor identity is a planner
query over `(static_origin, index)` — static, per-arm, planner-owned — so this
is still value-keyed and still static. The §1b predicate survives unchanged;
only the closure's **shape** moved, from splitting the producer to re-keying the
obligation.

## Fixed inputs, measured on the HS18-arc WIP `b601e2ec7`

The reachability measurement grounded the flow; these are its coordinates.

- **The per-arm identity source**, `lowering/core.rs:15534`: the case loop
  obtains `case_constructor_identity(eliminator.static_origin, index)` **before**
  emitting selection. Static, per-arm, planner-owned.
- **The selected block**, `core.rs:15567`: control switches into the case's
  existing `selected` block; only **after** that switch are the arm fields
  projected, the arm environment built, and the case body lowered.
- **The discharge site**, `core.rs:15758`: the body is lowered with the existing
  `remaining_eliminators`; this is where the source-context transfer and the
  residual are already emitted. Consumer execution and the per-arm certificate
  can share this block — nothing moves upstream, no receipt/edge carrier is
  threaded into the block.
- **The upstream edge selection** stays pure: `apply_required_consumer_incoming_edge`
  (`core.rs:9226`) has no `FunctionBuilder`, validates the defining
  call/completed prefix, returns a suffix slice, and emits no runtime operation
  or certificate; the `StaticResponseReturn` arm (`core.rs:4337-4381`) computes
  that suffix, preserves the carried word, and schedules work via
  `continue_composed_value` rather than lowering at the receipt seat.
- **The arity precedent to mirror**, `core.rs:15538`: the case-arity bound is the
  declared `argument_binders`, checked against the carried word's `field_count`
  at runtime "which is where a disagreement belongs." The result obligation is
  ruled onto the same footing.
- **The fail-closed default**, `core.rs:9375`: the unmatched-arm refusal stays as
  the default for an arm matching nothing.

## Deliverables

**`D1` — re-key the discharge to per-arm constructor identity.** Move the
checked-IH result obligation from the call-occurrence receipt to
`case_constructor_identity(eliminator.static_origin, index)`, discharged inside
the `selected` block at the `core.rs:15758` body-lowering site. **Report the
exact site and that the identity is the planner query's value, not read off the
carried word or inferred from the runtime tag.**

**`D2` — stop applying `RequiredConsumerIncomingEdge` unconditionally at the
receipt seat.** The edge/suffix selection stays upstream and pure (no
`FunctionBuilder`, emits nothing); the certificate is emitted downstream, per
arm, in the selected block. **Report that the receipt seat no longer carries the
result obligation and that Trap 43's unconditional application is gone.**

**`D3` — the execution control, and it is the load-bearing deliverable.** A
native observation that the exact per-arm discharge **executes for the
runtime-TAKEN arm**, paired with a **skip-discharge mutation that reddens**.
Pin it:

- **MEASURED** = the exact selected-arm discharge executes for the arm the
  runtime actually took.
- **CLAIMED** = certification follows the consumer, per arm.
- **GAP** = emitted-but-untaken case code and silently-skipped certification
  must be distinguished.

**A compile-time case count is not evidence of execution, and "Trap 43
disappeared" is not evidence of execution** — an untaken arm's emitted code and
a silently-skipped discharge both leave those unchanged. The mutation must red
because the discharge **did not run on the taken arm**, not because a case
vanished from the emitted set.

**`D4` — retentions, reported present and unchanged.** Each of these is retained
and must be shown still to hold, not merely left alone:

- Q2 attribution stays **local and non-vacuous** — closed, do not reopen.
- Certificate-follows-consumer.
- Amendment-8 origin; the entry-18 verifier repair.
- The `core.rs:9375` unmatched-arm refusal as the **fail-closed default**.

**`D5` — the advancing outcome.** Report what the seat does for the taken arm
now that the obligation is per-arm. Trap 43 resolving for the taken arm is the
expected shape. **If it advances to a refusal nobody predicted, stop and report
— that is a finding, not a failure.**

## THE CUT — stated so this node cannot drift into a refused family

- **Exact nested-call provenance at the creation site is CLOSED as an
  impossibility** (advisory Q1 + the entry-28 measurement) and **is not to be
  reopened by anyone.** This node does not recover it.
- **The runtime tag SELECTS the block; it never SUPPLIES the identity.** Using
  the selected arm to infer "therefore it came from the nested call" stays
  **refused** — that is provenance laundering, and it is dead. Reading the
  planner's own per-arm identity is not.
- **No producer-side split.** The measurement proved the receipt is minted
  upstream of the split with no branch-determining static key, so a producer
  split is unavailable here and is not the closure.
- **No new planner mechanism.** Not in scope: any new planner relation, row,
  query class, carrier, phase, or schema; any change to what the tag means.
- **No conditional summary and no caller-visible arm.** No condition crosses the
  call boundary; nothing is instantiated by a caller. Each arm discharges its
  own obligation unconditionally in its own block. If a proposal needs the
  caller to know which arm ran, it has left this ruling and re-entered a fenced
  family.
- **Do not reshape lowering to make the site reachable.** Reachability is
  already confirmed as-is (evt_6h85t50nkg73); reshaping to create a site would
  be the fourth softening.

## Why this is a node and why it is FENCED

The constraint is an Architect ruling, cited: `evt_5sddep840sxcj`. It is the
§1b structural-closure recut of the ABI-S6 HS18 hard-stop arc, and the Architect
handed the Steward this scope explicitly as a recut, not amendment 9. It is
**compiler-internal**: a re-key of an existing obligation onto an **existing**
planner query (`case_constructor_identity`), discharged at an **existing**
reachable site. It grows no TCB, adds no capability, changes no ABI or schema,
relocates no emission ownership, and adds no planner authority — so it is
assessed FENCED and carries no operator escalation.

## Sizing / tier

**Size M, tier T1.** The diff is bounded, but the review turns on an argument,
not a byte-count: that the re-keyed obligation is value-keyed and static (not the
refused runtime-tag authority), and that the execution control actually
witnesses per-arm discharge on the taken arm rather than a compile-time case
census. Architect required on the mechanism.

## Contention

Runtime ring, `crates/ken-runtime/src/cranelift_backend` + `lowering/core.rs`,
built on the HS18-arc WIP `b601e2ec7`. Touches `crates/`, so the candidate is a
CODE merge -> full CI, M8/M8a Adversary. No cross-lane contention: L2 language is
on `crates/ken-elaborator`, L3 foundation on `catalog/`. Re-measure the cited
`core.rs` coordinates at D0. Runtime is held at `5d977ac79` and `b601e2ec7` is
not itself a candidate; this recut is the work that produces the candidate which
lifts that hold.

## Not this node

- Recovering exact nested-call provenance — closed, permanently.
- Any producer-side specialization/split, multi-exit CPS, or conditional summary.
- Any change to the meaning of the runtime tag or to the unmatched-arm refusal.
- The certificate-demandability question — that is the *other* branch of the
  ruling and fires only if this reachable disposition had failed. It did not.
