# WP frame — RT-CARRIED-BOOL-ELIMINATOR-DISPATCH (lane-1, pairing-WP successor)

> Successor to [[RT-GENERATED-CONTINUATION-OPERAND-PAIRING]], cut from the
> Architect hard-stop #1 ruling (evt_3nm3jvapsf7cp, thr_3nmd0xy7dgh6g). The
> pairing repair correctly delivers the selected `ImmediateBool(false)` to the
> carried Match consumer, which then refuses it: a DISTINCT
> representation-specific eliminator gap, not a pairing residual. Owning team:
> runtime. Size M. Capability tier: T1 (a representation-discrimination repair in
> the carried-eliminator authority, with a census obligation over which immediate
> inductives share the consumer; the D0 deliverable is an authority + census, not
> a diff). No Decision — the two mechanisms (pairing in `units.rs`, Bool
> elimination in `joins.rs`) are independently correctable. Held behind the
> pairing WP; `draft` until it merges.

## Objective

Route the canonical carried Bool through a finite scalar dispatcher that maps the
exact Match case identities to `BoundaryTag::ImmediateBool` payload 0/1, using the
existing tag-checked scalar authority — leaving the node-backed constructor path
unchanged and failing closed on every malformed/hostile case. The pairing repair
now delivers the correct word; this WP makes the eliminator consume it.

## Fixed inputs (Architect ruling evt_3nm3jvapsf7cp, grounded at WIP `15abc5eb9255`)

- Consumer:
  `crates/ken-runtime/src/cranelift_backend/lowering/joins.rs::lower_carried_constructor_match`.
  It unconditionally calls `emit_carrier_tag` and `emit_carrier_field_count`;
  `emit_carrier_tag` calls the node-word helper `ken_boundary_tag_local`, whose
  `define_node_word` requires the carrier to resolve to an arena node. Canonical
  `ImmediateBool` is intentionally not a node, so `require_i64` refuses before any
  Bool case comparison. This is a representation-specific eliminator gap: the
  consumer treats every non-HostResult value as a node-backed constructor.
- Canonical carried Bool is `BoundaryTag::ImmediateBool`
  (`crates/ken-runtime/src/boundary_value.rs`), payload exactly 0 or 1.
- Existing authority the D1 path must REUSE: `emit_carrier_class` already
  recognizes immediate Bool; `emit_carrier_scalar` plus `merge_scalar_operand`
  already provide a tag-checked scalar observer.
- Node-only `emit_carrier_tag` must remain node-only — do NOT invent a node tag
  for an immediate and do NOT change `BoundaryTag`.
- The exact Match case identities must map canonical False/True to payload 0/1;
  hostile tag 7, malformed Bool payload outside `{0,1}`, wrong arity, and
  missing/duplicate/wrong-family cases must fail closed before selecting an arm.

## Anchor

Base the successor branch on the `main` that carries the merged pairing WP (do NOT
base on the pairing WIP `15abc5eb9255`, which is probes-plus-provisional and not a
candidate). The correctly-paired Bool word only reaches this consumer after the
pairing repair lands, which is why this WP is sequenced after it.

## Deliverables

- **D0 — case-family identity authority + immediate-representation census.** Bind
  the authority establishing the Match case-family identity (which cases are
  canonical False/True), and census which immediate inductive representations can
  reach this same consumer — structural Nat in particular, and any other immediate
  inductive — so the D1 fix cannot silently capture them. Return the bound
  authority and the census. Counts alone do not discharge this.
- **D0 stop.** Return the authority + census and the chosen shape of the scalar
  dispatcher before building it. Do NOT broaden Bool recognition to discharge the
  census.
- **D1 (conditional on D0) — the scalar Bool dispatcher.** Route the exact Bool
  family through a finite scalar dispatcher built on the existing tag/scalar
  authority (`emit_carrier_scalar` / `merge_scalar_operand`), leaving the
  node-backed constructor path unchanged. Every other immediate inductive the
  census found (especially structural Nat) gets an EXPLICIT measured disposition —
  a fail-closed refusal or a named separate object — never a silent fall-through
  into the Bool fix. Repair only the proven layer; zero `trusted_base()` delta.

## Acceptance criteria

- AC-1 — canonical false selects ONLY False.
- AC-2 — canonical true selects ONLY True.
- AC-3 — payload 2 under `ImmediateBool` refuses (malformed Bool payload outside
  `{0,1}`).
- AC-4 — tag 7 with payload 0/1 refuses (wrong family in a Bool slot; the
  hostile-tag refusal that moved here from the pairing WP's old AC-4).
- AC-5 — a reversed false/true mapping mutation REDS both paired rows (AC-1 and
  AC-2). A mapping a permutation can pass is not discriminating.
- AC-6 — bypassing the exact tag guard REDS the hostile row (AC-3/AC-4).
- AC-7 — existing node-constructor eliminators and the landed D1 HostResult
  eliminators remain green (the scalar path does not disturb the node path).
- AC-8 — structural Nat (and any other immediate inductive the D0 census found)
  carries the explicit measured disposition D1 chose, proven by a control, not
  left implicit.
- AC-9 (px8ta advance, honest). Re-run px8ta HALF B on the post-pairing base: the
  claim is ONLY that the canonical-Bool eliminator refusal disappears — report the
  next observation if reached, else name the first new causal obstruction. Do NOT
  promise end-to-end green.
- AC-NO-REGRESSION — whole-suite green in CI; local targeted `-p ken-runtime` /
  `--test` only, never `--workspace`.

## Banned repairs (Architect ruling)

- broadening "nonzero means true";
- keying the Bool family on spelling alone;
- turning another immediate type (structural Nat, etc.) into Bool, or letting one
  fall through the Bool fix silently;
- inventing a node tag for an immediate, or changing `BoundaryTag`;
- disturbing the node-backed constructor path or the landed D1 HostResult
  eliminators.

## Reviewers

Architect (component fit: the scalar dispatcher reuses the existing tag/scalar
authority rather than a new one, the node path is untouched, and the immediate
census disposition is explicit and measured) + runtime-qa (the mapping and
hostile controls are discriminating against a permutation and a guard bypass, and
the Nat disposition is proven by a control). No Decision fork open. Adversary
advisory, non-gating.

## Contention check

Touches `crates/ken-runtime/src/cranelift_backend/lowering/joins.rs`
(`lower_carried_constructor_match` and the scalar dispatch it gains) and focused
runtime/eliminator tests (incl. the direct Bool false/true/hostile test split out
of the pairing WP, and px8ta). Must NOT touch `crates/ken-runtime/src/boundary_value.rs`
`BoundaryTag` or the node-only `emit_carrier_tag` helper. No overlap with lane 2
(language/elaborator) or lane 3 (foundation catalog packages). Runtime ring
exclusive, sequenced after the pairing WP.

## Capability tier

T1. Size M — one focused increment: the D0 case-family authority + immediate
census to a chosen dispatcher shape, then the single scalar-path repair. Sized to
reach the census + chosen shape (or a genuine hard stop) within about an hour; a
census-done-D1-needs-its-own-cut outcome is a good stop, not a miss.
