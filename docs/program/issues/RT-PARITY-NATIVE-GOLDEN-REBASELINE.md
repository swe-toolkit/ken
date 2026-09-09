---
id: RT-PARITY-NATIVE-GOLDEN-REBASELINE
title: "Re-baseline every stale rt_parity_native checked_ih_* golden (census at base — four measured at 60c482cb8: two duplicated direct-origin 741/759, the admission-population set, and the confluence W0/W1 coordinate) so the native-slow CI job goes green, converting brittle absolute ids to relationship/cardinality assertions"
status: merged
owner: runtime
size: S
gate: none
tier: T2
depends_on: [RT-NESTED-IH-NATIVE-REALIZATION]
blocks: []
github: null
origin: "Steward-filed 2026-09-08, operator (Pat) concurred (\"concur with rt_parity_native re-baseline as a small queued node\"). Surfaced during the RT-NESTED-IH-NATIVE-REALIZATION admin-merge (PR #3422, squash 6172f95a7): the native-slow CI job was base-red on two rt_parity_native checked_ih_* tests, ruled OUT-OF-SCOPE candidate-neutral base-red drift by the Architect (evt_3hk4sfm7emxk7) and admin-merged past (Steward evt_6fr6v5rg3k726, PX9-INC2B precedent). This node is the golden refresh that clears the inherited red — NOT a soundness re-open of anything."
---

> # RELEASED 2026-09-09 (operator priority) — restores the CI signal.
>
> Operator (Pat), 2026-09-09, ruled "Release now, priority" after the base-red
> `rt_parity_native` cluster left CI's aggregate permanently red since ~2026-09-07
> (every PR merged via per-SHA admin-merge-past). This node is the golden refresh
> that clears the inherited red and removes the merge-past bypass. Steward flipped
> `draft` -> `active` and released to the runtime ring; it is the runtime lane's
> top-priority-next WP (does NOT interrupt the in-flight ABI-S6 D3 T1 build —
> taken at D3's next seam, or interleaved if the ring can). Still a test-golden
> re-baseline: no production code, does NOT gate the RT-NESTED-IH chain (landed).

## The stale checked_ih_* goldens — a CENSUS, not a fixed list (SCOPE WIDENED)

The frame's original "two tests" was an UNDER-COUNT of the stale population, not
a scope boundary. The binding AC was always "the whole `rt_parity_native` job
goes green," so the true deliverable is every stale `checked_ih_*` golden the
job carries — measured at the candidate base, not enumerated. The
runtime-implementer's targeted one-test census at base `60c482cb8`
(evt_57hex5x044kvf) found FOUR, all in `crates/ken-cli/tests/rt_parity_native.rs`
and all one defect class — stale absolute source-occurrence / worker-body
coordinates from the `bfce9e441` (ABI-REVOKE-D2) semantic-plane renumbering:

1. `checked_ih_direct_application_drop_capture_refuses_and_restores` (~:2459) —
   pins ABSOLUTE `invocation/application/callee = StaticOriginId(759/758/757)`;
   live value is 741.
2. `checked_ih_generated_entry_admission_population_is_total` (~:2717) — pins
   whole admission-population SETs keyed by absolute `worker_body_origin`
   (963/1269/1290 shifted) and the same 757/758/759 triple.
3. `checked_ih_generated_entry_confluence_reaches_exact_capsules` (~:1227) —
   fails to find the pinned W0/W1 `(binding=755, invocation=759)` coordinate.
4. `checked_ih_direct_application_pairs_one_declared_call_result` (~:2361) —
   the SAME 741-vs-759 direct-origin triple as (1), so that coordinate is
   DUPLICATED across two tests: one numbering shift re-arms both.

Scope ruling (Steward, evt_4j5bmxnemtbkm): WIDEN this WP in place to cover the
whole stale set; the confluence golden is NOT separately owned. Re-census at the
candidate base and fix all — plus any further stale golden the job surfaces
there. These are semantic-plane origins upstream of RT-NESTED-IH; that
chain-closing candidate was proven candidate-neutral (Architect: it renumbers no
origin and reclassifies no admission row), so this is a golden refresh, not a
soundness re-open.

## Deliverable

Refresh EVERY stale `checked_ih_*` golden that `rt_parity_native` requires for a
green job — census-based, not the enumerated list. Census the full stale set at
the candidate base first (the four above, plus any other the job surfaces) and
fix all of them so the native-slow (`rt_parity_native`) CI job goes green.

- **Prefer RELATIONSHIP / CARDINALITY / governed-partition assertions over
  absolute ids where the test's intent allows** (Architect z3670/z3680 carry):
  these are exactly the brittle absolute-`StaticOriginId` / absolute-population
  goldens that red on every upstream numbering shift, so a bare `759 -> 741`
  bump just re-arms the same failure for the next ABI-adjacent WP (ABI-S6 D3
  among them). The duplicated 741/759 direct-origin coordinate across :2459 and
  :2361 is the tell: prefer a SINGLE relationship expression of the invariant
  over two absolute restatements. Assert adjacency / cardinality / the governed
  partition where that preserves the test's discriminating power; scope of the
  hardening is this node's call.
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
