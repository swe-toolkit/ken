---
id: RT-NATIVE-SETTLEMENT-PLACEMENT-KEY
title: "RNSPK's settlement placement key is a second authority for where a checked-IH K call's work happens, beside the emission owner, transport sources and generated-entry installation the existing planes already read. Since its first product commit (03a82e1c5) the committed checked_ih controls have been red, on a different invariant at each commit, as each repair synchronized one more consumer. Decide each checked-IH K call's emission placement once, record it, and have every plane read the record, so the controls are green again with the key present"
status: active
owner: runtime
size: L
tier: T1
gate: architect
depends_on: []
blocks: [RT-NATIVE-FORWARDED-BOUNDARY-TRAP, PX10-NATIVE-SPAWN-PARITY]
github: null
origin: "Architect ruling (i) evt_5b1gt2a3cz0f0 on PX10 B1a's F-NEXT call (evt_a0asf7f4xt0w), with Research evt_1kywnb81qgrqm and addendum evt_3xevbv79j2pad; Steward scope read evt_h4eybf0c1ydg recut it into its own node, landing before B1a resumes. L1 objective: the Linux ABI program. Recut 2026-11-09 20:45Z (Steward evt_1g0fjah621vx3) on the Architect's WP-mis-sized verdict after the bisection D0 (evt_3jatzr9kk7e11): the node keeps its id and becomes the one placement authority; the fail-closed trap moves to RT-NATIVE-FORWARDED-BOUNDARY-TRAP. Steward-filed per COORDINATION section 2."
---

# One placement authority

## Objective

Each checked-IH K call's emission placement, its owner and its seat, is
decided once and recorded on the plan. Every plane that names that
placement reads the record and none recomputes it. With the settlement
key present, the committed controls are green again, as they were at the
B1a anchor `a966f0ee5`.

## Settled inputs

- **The bisection** (D0 `evt_32z8fxd3myrn1`,
  `/workspaces/ken/tmp/rnspk-bisection-d0-result.txt`). The three rows
  `checked_ih_generated_entry_admission_population_is_total`,
  `_confluence_reaches_exact_capsules` and
  `checked_ih_fresh_result_coemission_only_is_not_paired` pass 3/3 at
  `a966f0ee5` and fail at its first product commit, R = `03a82e1c5`, and
  at every measured commit after it. Each commit fails them on a
  different invariant (the symptom inventory below).
- **The predicate** (Architect `evt_3jatzr9kk7e11`). All five failures
  are two planes disagreeing about where one checked-IH call's work
  happens. Plane A/B's settlement key is a second authority for that
  placement. The six F walls of 16:45 to 17:29 are the same predicate on
  F, so no fix keyed to F can repair the controls. Step 0 confirms the
  predicate on the plane that carries it before anything is designed.
- **The raw roots exist before response phase A** (Step 0 of the 19:15Z
  correction, `evt_3dx0b3gtgcexr`, accepted by the Architect):
  `plan.continuation_specializations[id].key.worker.body_origin` after
  `construction.rs:1407`-`1417`, and
  `plan.abi.descriptors[f].body_occurrence`. Response phase A is
  `construction.rs:1429`; relay routes build at `:1594`.
- **Base.** `e35fc4695`, whose product equals `91e7e83a0`: the
  settlement planes A, B and C, `forwarding_boundary_trap`,
  `is_boundary_forwarding_arm` and the AC-1c fixtures. `e0636a365` and
  `1eeac62b4` are not carried; their hunks are population-wide.
  `wp/RT-NATIVE-SETTLEMENT-PLACEMENT-KEY-implementation` stays at
  `1eeac62b4` as a record. Held `250e3e6e9` and B1a's WIP2 are never
  moved.
- **Retained, not product here:** the AC-1 overlay logs, the S0b
  analysis, and the trap, which `RT-NATIVE-FORWARDED-BOUNDARY-TRAP`
  re-bases on this node.

Treat anchors as perishable. If a settled input is false on the base,
stop and report the mismatch.

## Deliverable

One candidate, stacked under B1a. It records the placement once and
switches every reader to it: plane A/B's settlement key, the effect
claims (`effects.rs`), bracket `settlement_at`, both transport-source
stages, the detached-result seat and generated-entry installation. The
record's shape and its point of construction are the Architect's, after
Step 0.

## Acceptance

- **Step 0 (before any design).** At R, in a fresh clean detached
  worktree, on `fs-read-at-offset-single`, print the two values the
  InPlace S refusal compares (the `effects.rs` placement lookup against
  the validated view) for the failing K-call identity.
  - If both name a placement (owner or seat) for the same call, and they
    differ, the predicate is confirmed.
  - Otherwise stop and report to the Architect and the Steward.
- **AC-1, controls.** On the committed product, with no F work and no
  overlay, these are green:
  - checked_ih 9/9, including the three bisection rows;
  - px8ta 3/3, both G controls and the smoke 9/9;
  - the two-print parity row (`rt_selected_pending_call_admission.rs:303`)
    and span freeze 2/2;
  - entry 18's row, B-closure's census pin and six-case matrix;
  - AC-1c's committed F production-refusal row.
- **AC-2, mutation.** One consumer recomputes the placement locally
  instead of reading the record, and that consumer's named invariant
  returns on `fs-read-at-offset-single`. A mutation that does not redden
  is reported as non-discriminating, not replaced.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Symptom inventory

Each is keyed on "placement re-decided in plane X":

1. `03a82e1c5`: an InPlace S effect claim disagrees with its validated
   view (`fs-read-at-offset-single`, `fs-write-at-offset-single`).
2. `e9fbbca83`: a marked bracket settlement is lowered outside its K
   call's emission placement.
3. `250e3e6e9`: the pre-schema transport source set disagrees with the
   post-ownership set.
4. `e35fc4695`: the detached-result seat gets a unit result that is not
   a specialized constructor.
5. `1eeac62b4`: the governed generated entry is installed and has zero
   arrivals.

## Inventory (node §1a)

1. Plane B's owner check keyed on `k_specialization`, the response's
   specialization, not the placement it settles (`evt_yz6rxwdke9f6`).
2. After planes A and B, F2 argv and v_null refuse at the
   response-owner call-coverage gate (`units.rs:9779`): owner
   `StaticResponseOwnerId(2)`'s selected caller has no recorded verified
   incoming call. Keyed on `ContinuationCallIdentity`, the selected
   caller (`evt_3aax4002wm3yt`).
3. The checked-IH environment transport moves a K call's emission to its
   destination owner, but three consumers read the planned emitter or a
   plan-wide fact instead. Keyed on the planned emitter, not the code
   identity that emits the call (`evt_662q5ektq3zcx`, Architect
   `evt_1dnbnhbgs176n`).
4. Detached seat pass-through keyed on owner comparison instead of on
   the Function's emission evidence (`evt_dd551d27htbj`, Architect
   `evt_7gs4bp33yneey`).
5. Self relocation withdrawn at the post-ownership builder only, while
   the pre-schema stratum is a parallel derivation. Keyed on
   transport-source membership, used as if it were relocation
   (`evt_3tfjfceyg80y9`, Architect `evt_6rnvdpnq9b9vk`).
6. Escaping K calls excluded per caller, used as if escape were
   per-caller rather than transitive (`evt_brqpp3zm90sp`, Architect
   `evt_p77sfegqy2k2`).

## Stop conditions

- Step 0 does not confirm the predicate.
- A kernel or trust change.
- A change to the Process gate, `HostOpV1::availability` or AC-2g.
  Those belong to B2.
- PX10 §1a reaching 21 (it is 18). That triggers the Architect's re-ask
  to Research.

Checks: 4 fired: AC-1's target state was measured green at `a966f0ee5`
(`rnspk-d0-bisect-a966f0ee5-three.log`, 3/3), so it is a reached state,
not an assumed one. 6 fired: the symptom inventory is the five bisection
observations, one log each, not inferred from commit messages. 8 fired:
AC-1 runs without F work or the overlay, because every F-only gate in
this chain was green while the controls were red.
