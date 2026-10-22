---
id: RT-BRACKET-REGION-TRANSPORT
title: "CHILD 2a of RT-BRACKET-CONTROL-REGION-IR, by route C. Child 1's bracket regions are computed on the normalized plane and then dropped (compiler_driver binds them to _), and after erasure the confined finalizer lowers to the same HostOpV1::ResourceRelease as a public release, so nothing downstream can tell them apart. Carry each region across erasure as one compiler-private Runtime-IR bracket marker, bound exactly to its region, so the settlement plane reads ports from markers and never from HostOpV1. THIS CHILD DOES NOT LAND ALONE: it is a held, reviewed input assembled by child 3."
status: merged
owner: runtime
size: M
gate: none
tier: T1
depends_on: [RT-BRACKET-PRODUCER-AUTHENTICITY]
blocks: [RT-BRACKET-SETTLEMENT-PLANE]
github: null
origin: "Operator 2026-10-18 (\"concur with rec on px8ta decision\": route C child 1, children 2 and 3 sized after it). Architect sizing D0 evt_51cm672pdy1r1 on child 1 at c462fe831 (Architect approval as a held input evt_2hz1a3x9tzp8k, Decision dec_1ngs256pczqdk) found the region-transport seam that neither child-2 nor child-3 frame named, and cut it as its own child. Steward-filed per COORDINATION section 2."
---

> **Recut 2026-10-21 (`evt_1gxk3pcr6rgw2`; operator 2026-10-22, "keep
> separate").** This is link 2 of the bottom-up chain and lands alone, as
> a fresh candidate on `main` after link 1 (`77352e68e`). The accepted
> held input `370168a48`, built on `c462fe831`, is a reference only: never
> moved, rebased or landed. Where a settled input below names
> `c462fe831` as the base, re-measure it on `main`.
>
> **Base and unit (Architect `evt_1m5yxewmk3xhw`).** Link 2 is AC-0 to
> AC-5 as one landing unit: the net delta `c462fe831..c18c8db68` applied
> fresh on `77352e68e` (ported tree `813dfe6ba`), with two expectation
> corrections: the arity test code E0084, and the px7f owner ordinals +1.
> Evidence: `/workspaces/ken/local/arch-rt2-d0/`.

# Bracket regions survive erasure as bound markers

## Objective

Every route-C bracket region reaches Runtime IR as exactly one
compiler-private marker on its finalizer occurrence, bound one-to-one to
the region the collector found, so child 2b can read authenticated ports
from Runtime IR.

## Settled inputs (Architect `evt_51cm672pdy1r1`, on `c462fe831`)

- **Build on `c462fe831`** (`wp/RT-BRACKET-PRODUCER-AUTHENTICITY-route-C`),
  on this child's own branch. Never move, reset or delete that ref, or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
- **The drop.** `complete_native_program_preparation` destructures the
  regions as `bracket_regions: _` (compiler_driver.rs ~:3070 on
  `77352e68e`). The only reader is the test accessor `bracket_regions()`.
- **The erasure collision.** `RuntimeExpr::Effect` carries `HostOpV1`
  only (ken-runtime-support ir.rs ~:754 on `77352e68e`), and both checked
  release IDs lower to `HostOpV1::ResourceRelease`. The static arm of
  `lower_checked_host_computation` sees none of the finalizers (measured
  `evt_59wt0vsvvcp1p`). Each finalizer reaches erasure as a private
  constructor `RuntimeExpr::Construct` from `lower_body_term_with_plans`,
  and the release is dispatched dynamically through `Vis`. The marker's
  subject is that `Construct` (Architect `evt_48knx3bsxhaxr`).
- **The binding key (amendment 1, Architect `evt_48knx3bsxhaxr`).** It is
  (owner `StableSymbol`, preorder node id). The id is the Ω-erasure
  plan's `Term::children` preorder, the same numbering that the decoder
  binds to canonical bytes (checked_core.rs ~:3145-3195, :5177), from one
  traversal shared with the collector. Erasure's edge tags stay out of the
  binding. At the bound node, the decoded `ConstructorReference` must be
  the confined `PrivateBracketRelease` by checked constructor id. The id
  is transient: not serialized, not in the canonical bytes or their hash,
  and not a runtime value. Constraints 1 to 7 of that ruling are
  operative.
- **The precedent.** The oriented markers (`CheckedRecursiveInvocation`,
  `CheckedComputationalIH*`) carry a `checked_occurrence_path` template
  computed on `normalized_bodies`, and
  `bind_oriented_runtime_marker_locations` (erasure.rs ~:1942) binds each
  template exactly, refusing an unbound one.
- **Authorized form.** The marker is umbrella-D1 compiler IR: a
  `#[doc(hidden)]` compiler-private Runtime-IR wrapper, not a runtime
  value, syntax, effect, host op or ABI field.
- **Visibility.** `RuntimeExpr` is `pub` and reaches ken-cli,
  ken-elaborator, ken-interp, ken-runtime and ken-runtime-support. The
  precedent `CheckedSubcontinuationFrame` is matched in four of them, so
  the cross-crate exhaustiveness sweep is owed, by grep.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

One marker variant in ir.rs, its minting and exact binding in erasure, the
transient preorder id on the decoded view in `checked_core.rs`, the
`compiler_driver` handoff of `bracket_regions` into erasure, and the
collector arity hardening below.

## Acceptance

- **AC-0 (D0, run first).** On the c462fe831 test population (single plus
  the console lookalike, two public early releases, multi-leaf, nested-2,
  triple) plus Buffer, anonymous Mapping and FileBacked-in-FS:
  - collector regions equal bound Runtime markers, one to one;
  - every marked finalizer binds to exactly one region;
  - the public `release` effects stay unmarked.
  - paired: shifting the collector's id by +1 refuses by name on the
    multi-leaf row, and binding by ordinal swaps nested-2's inner and
    outer finalizers.
- **AC-1 (paired).** Minting by `HostOpV1::ResourceRelease` instead
  over-marks the early-release fixture: 3 markers where 1 is expected.
  This is a prediction; run it.
- **AC-2.** A region with no marker, a marker with no region, and a
  region bound twice each reach an exact named refusal.
- **AC-3 (arity hardening).** A hand-built collector row with
  `PrivateBracketRelease` under-applied (2 args) refuses with E0084 (E0082 is now
  `elab.refinement_split_demand` on `main`).
  Paired: at `c462fe831` the same row returns `Ok` with 0 regions.
- **AC-4.** The cross-crate `RuntimeExpr` match sweep is complete by grep,
  and runtime behaviour on the population is unchanged.

- **AC-5 (amendment 2, Architect `evt_3jksek4xvf2wk`; the acquire mark).**
  The settlement plane needs a runtime acquire, and 2a's accepted tip
  `370168a48` locates only the finalizer.
  - Build on a new branch `wp/RT-BRACKET-REGION-TRANSPORT-acquire` from
    `370168a48`; that ref and its approval stay as they are, and the
    approval does not carry to the new tip.
  - Erasure binds one Acquire mark per region by the 2a mechanism,
    `(owner, preorder node id)`, validated by checked constructor id in the
    collector's `acquire()` set. Its subject is the acquire's selected
    operation `Construct`: `injection[2]` under `args[3]` of the Vis at
    `acquire_vis_path`.
  - The form is the existing variant with a closed
    `CheckedBracketMarkRole { Acquire, Finalizer }` on
    `CheckedBracketRegionMarker`, not a second `RuntimeExpr` variant. The
    region key is `(owner, ok_resource_binder_path)`.
  - The bijection extends to exactly one Acquire mark and at least one
    Finalizer mark per region, all with an equal key. An unbound acquire,
    a second Acquire mark and an Acquire mark with no region each reach a
    named refusal.
  - Paired: deleting the Acquire mark reaches that refusal, never a
    fallback to an enclosing Vis. The consumer sweep for the new role is
    complete by grep.

## Stop conditions

- A marker that needs a host op, an ABI field, a runtime value or a
  kernel change: stop to the operator.
- A region that cannot be bound exactly from the two walks: stop with the
  row. Do not fall back to selecting by `HostOpV1`.
- One declaration body lowered twice, so that one region meets two
  `Construct`s with the same id: stop with the row. Do not deduplicate.
