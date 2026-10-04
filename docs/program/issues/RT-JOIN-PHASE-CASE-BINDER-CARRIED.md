---
id: RT-JOIN-PHASE-CASE-BINDER-CARRIED
title: "The join planner gives a case binder the phase that lowering will see: a source join whose scrutinee is a carried case binder (the nat_fanout_escaped row's join 1244 under ReadProgress::ReadSome) is not planned NativeScalarPair, so it no longer refuses in object emission with 'planned native scalar lanes but lowering produced a carried boundary word'"
status: active
owner: runtime
size: S
gate: architect
tier: T1
depends_on: [RT-NATIVE-TREE-MATCH-RUNTIME-SCRUTINEE]
blocks: []
github: null
origin: "Architect ruling 2026-09-28 (evt_2pf70v5ty9mf0) on the RT-NATIVE-TREE-MATCH D2 observation (evt_79a1xw3p73q6): the nat row's next refusal is a separate blocker outside that WP. Size is provisional and is re-set at AC-0. Steward-filed per COORDINATION section 2."
---

# Join phase of a carried case binder

## Objective

`nat_fanout_escaped_resource_matches_interpreter` gets past join 1244 on a
base where TREE-MATCH has landed.

## Settled inputs (`evt_79a1xw3p73q6`, Architect `evt_2pf70v5ty9mf0`)

- **The refusal**, from
  `crates/ken-runtime/src/cranelift_backend/lowering/joins.rs:390`: "a carried
  `Match` arm source join StaticOriginId(1244) (...) planned native scalar
  lanes but lowering produced a carried boundary word".
- **The join.** 1244 is a single-case `BufferSpan` projection with body
  `Var(2)`, planned `NativeScalarPair`. At lowering, its `Var(0)` is
  `Carried(v5188)`.
- **The binder.** The nearest binder is the `ReadProgress::ReadSome` case
  (Match 1249, body 1246, two binders). It is not a binder of either composed
  Option frame, and 1244 lies outside every composed subtree.
- **Two planes disagree.** The owner seed from
  `result_phase_environment_for_owner` (`joins_traps.rs:546`) gives `Var(0)`
  `CarrierRequired`. The planner's effective lexical phase while it summarizes
  1244 is `SpecializedOnly`. The `Var` arm defaults a missing entry to
  `ResultPhaseSummary::SPECIALIZED` (`joins_traps.rs:476`).
- **A second witness, not corroboration** (Architect `evt_1mz68b0assf2d`).
  Join 70, the `bytes_at` argument-0 Match in the TREE-MATCH Option fixture's
  `main`, was planned `SpecializedOnly` (so `NativeScalarPair`), and its arm 62
  lowered `Carried`. That is the same observation class as 1244. No common
  cause is measured, and arm 62 is not shown to be a case binder. AC-0 measures
  it separately.
- **Unmeasured:** where the planner assigns the phase of 1249's case binders,
  and whether that assignment reads the scrutinee's planned phase or takes the
  default.

## Deliverable

One repair, ruled by the Architect, after which the planner assigns the case
binder the phase that lowering produces. The row then reaches its next
refusal or runs natively.

## Acceptance

- **AC-0 (probe, then ruling; no product change).** Name the site that assigns
  the phase of 1249's case binders, and say whether that plane carries the
  phase lowering sees (Check 5). The Architect rules the repair and sets the
  size.
- **AC-1.** The nat row gets past join 1244. Record its next first refusal
  verbatim. If it runs natively, its differential must match the interpreter.
  It stays ignored unless it goes green.
- **AC-2 (control).** Reverting the repair restores the verbatim 1244
  refusal. The targeted runtime and ken-cli suites stay green.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A repair that changes phase planning for joins outside case-binder scrutinees
  is a stop to the Architect with the census of affected joins.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
