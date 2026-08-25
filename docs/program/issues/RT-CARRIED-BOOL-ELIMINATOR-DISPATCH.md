---
id: RT-CARRIED-BOOL-ELIMINATOR-DISPATCH
title: "Lane-1 successor to the pairing WP — route the canonical carried Bool (BoundaryTag::ImmediateBool, payload 0/1) through a finite scalar dispatcher in joins.rs::lower_carried_constructor_match, which today calls node-only emit_carrier_tag and require_i64-refuses every immediate scalar before any Bool case. D0 binds the case-family identity authority and censuses which immediate inductive representations reach the same consumer; D1 adds the scalar Bool path using existing tag/scalar authority, leaving node-backed constructors unchanged and giving structural Nat an explicit measured disposition"
status: draft
owner: runtime
size: M
gate: none
depends_on: [RT-GENERATED-CONTINUATION-OPERAND-PAIRING]
blocks: []
github: null
origin: "Steward, 2026-08-25, from the Architect hard-stop #1 ruling evt_3nm3jvapsf7cp (thr_3nmd0xy7dgh6g) on RT-GENERATED-CONTINUATION-OPERAND-PAIRING. The pairing repair correctly delivers the selected ImmediateBool(false) to the next consumer, exposing a DISTINCT representation-specific eliminator gap: joins.rs::lower_carried_constructor_match treats every non-HostResult value as a node-backed constructor (emit_carrier_tag -> ken_boundary_tag_local -> require_i64), while canonical Bool is an immediate scalar sum, so it refuses before any Bool case comparison. Ruled a separate object with no Decision (two independently correctable mechanisms). Steward owns the final ID/frame."
---

> # Successor to the generated-continuation pairing repair
>
> Held behind [[RT-GENERATED-CONTINUATION-OPERAND-PAIRING]]: the correctly-paired
> `ImmediateBool(false)` only reaches this consumer after the pairing repair
> lands, so `depends_on` that WP. `status: draft` — release to the runtime ring
> once the pairing WP merges (Steward owns lane order). Full frame:
> `docs/program/wp/RT-CARRIED-BOOL-ELIMINATOR-DISPATCH.md`.
>
> This is NOT a pairing residual — the source binder now receives exactly the
> correct word. It is a representation-specific eliminator gap in the carried
> Match consumer. Distinct from the pairing object, from the merged
> [[RT-CAPTURE-CONTEXT-FRAME-EMIT]], and from the reporter-honesty
> [[RT-UNIT-FAILURE-STATUS-PROVENANCE]] object.

## Objective (Architect ruling evt_3nm3jvapsf7cp)

Route the canonical carried Bool through a finite scalar dispatcher that maps the
exact Match case identities to `ImmediateBool` payload 0/1, using the existing
tag-checked scalar observer — leaving the node-backed constructor path unchanged
and failing closed on every malformed/hostile case. Do not broaden "nonzero means
true," do not key on spelling alone, and do not turn another immediate type into
Bool.

## Fixed inputs (Architect ruling, grounded at WIP `15abc5eb9255`)

- The consumer is `crates/ken-runtime/src/cranelift_backend/lowering/joins.rs::lower_carried_constructor_match`,
  which unconditionally calls `emit_carrier_tag` and `emit_carrier_field_count`.
  `emit_carrier_tag` calls the node-word helper `ken_boundary_tag_local`, whose
  `define_node_word` requires the carrier to resolve to an arena node; canonical
  `ImmediateBool` is intentionally not a node, so `require_i64` refuses before any
  Bool case comparison.
- Canonical carried Bool is `BoundaryTag::ImmediateBool` with payload exactly 0 or
  1 (`crates/ken-runtime/src/boundary_value.rs`).
- `emit_carrier_class` already recognizes immediate Bool, and `emit_carrier_scalar`
  plus `merge_scalar_operand` already provide a tag-checked scalar observer — the
  existing authority the D1 path must reuse.
- Node-only `emit_carrier_tag` must REMAIN node-only: do not invent a node tag for
  an immediate and do not change `BoundaryTag`.
- The exact Match case identities must map canonical False/True to payload 0/1;
  hostile tag 7, malformed Bool payload outside `{0,1}`, wrong arity, and
  missing/duplicate/wrong-family cases must fail closed before selecting an arm.

## Deliverables

- **D0 — case-family identity authority + immediate-representation census.** Bind
  the authority that establishes the Match case family identity (which cases map
  to canonical False/True), and census which immediate inductive representations
  can reach this same consumer — in particular structural Nat and any other
  immediate inductive — so the fix does not silently capture them. Return the
  authority and the census; counts alone do not discharge it.
- **D1 (conditional on D0) — the scalar Bool dispatcher.** Route the exact Bool
  family through a finite scalar dispatcher using the existing tag/scalar
  authority (`emit_carrier_scalar` / `merge_scalar_operand`), leaving the
  node-backed constructor path unchanged. Other immediate inductives (especially
  structural Nat) receive an EXPLICIT measured disposition — a fail-closed refusal
  or a named separate object — never a silent fall-through into the Bool fix.
  Repair only the proven layer; zero `trusted_base()` delta.

## Acceptance criteria (Architect-required controls)

- AC-1 — canonical false selects only False.
- AC-2 — canonical true selects only True.
- AC-3 — payload 2 under `ImmediateBool` refuses (malformed Bool payload).
- AC-4 — tag 7 with payload 0/1 refuses (wrong family in a Bool slot — the
  hostile-tag refusal moved here from the pairing WP).
- AC-5 — a reversed false/true mapping mutation REDS both paired rows (AC-1/AC-2).
- AC-6 — bypassing the exact tag guard REDS the hostile row (AC-3/AC-4).
- AC-7 — existing node-constructor eliminators and the landed D1 HostResult
  eliminators remain green (the scalar path does not disturb the node path).
- AC-8 — structural Nat (and any other immediate inductive the D0 census finds)
  has the explicit measured disposition D1 chose — proven by a control, not left
  implicit.
- AC-NO-REGRESSION — whole-suite green in CI; local targeted `-p ken-runtime` /
  `--test` only, never `--workspace`.

## Sequencing

Next lane-1 object AFTER [[RT-GENERATED-CONTINUATION-OPERAND-PAIRING]] merges.
`draft` until then; Steward releases it on the pairing WP's landing.
[[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]] and
[[RT-UNIT-FAILURE-STATUS-PROVENANCE]] remain distinct and sequenced separately.
Steward owns lane order.
