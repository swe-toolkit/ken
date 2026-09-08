---
id: RT-PARITY-NATIVE-GOLDEN-REBASELINE
title: "Re-baseline the two base-red rt_parity_native checked_ih sentinels (drop-capture absolute StaticOriginId + admission-population-is-total set) so the native-slow CI job goes green, preferring relationship/cardinality assertions over brittle absolute ids"
status: draft
owner: runtime
size: S
gate: none
tier: T2
depends_on: [RT-NESTED-IH-NATIVE-REALIZATION]
blocks: []
github: null
origin: "Steward-filed 2026-09-08, operator (Pat) concurred (\"concur with rt_parity_native re-baseline as a small queued node\"). Surfaced during the RT-NESTED-IH-NATIVE-REALIZATION admin-merge (PR #3422, squash 6172f95a7): the native-slow CI job was base-red on two rt_parity_native checked_ih_* tests, ruled OUT-OF-SCOPE candidate-neutral base-red drift by the Architect (evt_3hk4sfm7emxk7) and admin-merged past (Steward evt_6fr6v5rg3k726, PX9-INC2B precedent). This node is the golden refresh that clears the inherited red — NOT a soundness re-open of anything."
---

> # QUEUED, not released. Small golden refresh, not a soundness change.
>
> This is a test-golden re-baseline. It queues behind the active lanes
> (`steward.md` §1: filings queue behind the lanes). It does NOT gate the
> RT-NESTED-IH chain (already landed) and touches no production code.

## The two base-red tests

`crates/ken-cli/tests/rt_parity_native.rs`, both stale since `bfce9e441`
(ABI-REVOKE-D2, which shifted the semantic-plane numbering and left the goldens
stale):

1. `checked_ih_direct_application_drop_capture_refuses_and_restores` (~:2459) —
   pins ABSOLUTE `invocation/application/callee = StaticOriginId(759/758/757)`;
   the live value is 741.
2. `checked_ih_generated_entry_admission_population_is_total` (~:2717) — pins
   whole admission-population SETs keyed by absolute `worker_body_origin`
   (963/1269/1290) and the same 757/758/759 triple.

These are source-occurrence / worker-body origins assigned at the SEMANTIC
plane, upstream of the RT-NESTED-IH work; the chain-closing candidate was proven
candidate-neutral (Architect grounded it structurally: it renumbers no origin
and reclassifies no admission row).

## Deliverable

Refresh both goldens to the current correct values so the native-slow
(`rt_parity_native`) CI job goes green.

- **Prefer RELATIONSHIP / CARDINALITY / governed-partition assertions over
  absolute ids where the test's intent allows** (Architect z3670/z3680 carry):
  these are exactly the brittle absolute-`StaticOriginId` / absolute-population
  goldens that red on every upstream numbering shift, so a bare `759 -> 741`
  bump just re-arms the same failure for the next ABI-adjacent WP. Assert
  adjacency / cardinality / the governed partition where that preserves the
  test's discriminating power; scope of the hardening is this node's call.
- Keep each test's discriminating power: the refresh must still fail on a real
  regression (do not weaken a probe to make it pass — `merge-procedure.md`).

## Acceptance

- `rt_parity_native` green in CI (the whole native-slow job), no other
  native-parity test perturbed.
- Any assertion converted from absolute-id to relationship form still reds
  under a real injected mismatch (a mutation control, not a vacuous pass).

## Validation — targeted only

`rt_parity_native` is the ~24-min D5-ceiling test that holds the shared build
lock (`agent/COORDINATION.md §12`; see the fleet memory on the build-lock
deadlock). Run it as a BACKGROUND, targeted command, never a foreground long
loop; CI is the workspace verdict.
