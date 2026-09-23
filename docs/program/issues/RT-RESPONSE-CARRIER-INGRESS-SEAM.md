---
id: RT-RESPONSE-CARRIER-INGRESS-SEAM
title: "a Specialized host response reaches a represented constructor's boundary transfer before its exact response owner is entered, so the admissibility walk correctly refuses it as compiler control (StaticResponseDeferred) -- bind the exact Vis/caller/owner identity to the value-construction path so the constructor crosses only after the owner yields a real, validated response value, or prove from a completed lowering that the selected call is unconsumed and demote that identity consistently; rows rt_escape r2_cross_buffer_freeze and, if D0 confirms the same path, rt_span sp_a_foreign_span_freeze"
status: closed
owner: runtime
size: L
gate: none
tier: T1
depends_on: [RT-PROCESS-EXIT-STATUS]
blocks: []
github: null
origin: "Architect ruling evt_6np1yzwgg8m2r on RT-PROCESS-EXIT-STATUS checkpoint d47f1f990, which closed that node on D1 outcome (iii) and asked the Steward for a scoped follow-on for the owner/constructor seam. Operator L1 directive 2026-09-17: clearing the ignored rows is the top priority. Steward-filed per COORDINATION section 2."
---

# A Specialized response crosses a constructor before its owner runs

> # CLOSED 2026-09-23: `D1` outcome (1), a measured STOP for both rows.
>
> Runtime `evt_2bk7cn1ycgf0h`, Architect `evt_2hffnnh3w0qr9`, on clean
> `fbf4ed78c`. **The settled-input path below is SUPERSEDED.** The consumer
> is not a constructor field. Each row's WHOLE pending `ITree::Vis` (row 13:
> Vis 746, the `Ret` arm value of match 571; row 14: Vis 528, match 355) is
> carried through a `ComputationalMatch` arm join, `core.rs:14561` ->
> `joins.rs::carried_join_arm` -> `transfer_represented_boundary_value`, into
> the correct refusal. Row 14 shares row 13's path. Horn B is refuted: the
> owner executes the effect, and substituting its result at the join would
> move `ResourceRelease` earlier. Horn A has no witness. No code changed.
>
> **Scope, Architect `evt_2817gkyjtdaa7`:** both Vis are the prelude
> `release_if_live` finalizer, the final-settlement mismatch that
> `RT-BRACKET-CONTROL-REGION-IR` exists to close. After the operator's lane
> ruling on the bracket tree, the rows join it as extra D0/closeout controls:
> an exact marker -> settlement port -> region instance -> `BracketOwned` join
> per row, no early `ResourceRelease`, no placeholder transfer, and both
> differentials green before un-ignoring. If either row lacks an authenticated
> final-settlement port, that is the umbrella's own STOP, not a licence for a
> new carrier. The `RT-PROCESS-EXIT-STATUS` annotation checkpoint
> `d47f1f990` stays staged for that candidate.

## Settled inputs -- Architect `evt_6np1yzwgg8m2r`, at `d47f1f990`

- Row 13: `crates/ken-cli/tests/rt_escape_second_resource_native.rs`,
  `r2_cross_buffer_freeze_fails_closed_with_invalid_bounds`. Phase B marks
  `BufferFreeze` and all three `ResourceRelease` `Vis` **Specialized**, with an
  empty Deferred population.
- The first refusal is constructor origin 746. Its `ResourceRelease` operation
  root 745 lowers to `StaticResponseDeferred` outside a response owner
  (`core.rs:15018-15034`). Because a sibling is carried, `core.rs:13055-13092`
  calls `transfer_represented_boundary_value` on the specialized constructor
  child. `aggregates.rs:1165-1183` then descends through
  `represented_boundary_admissibility` to `boundary.rs:1056-1059`, which
  **correctly** refuses compiler control.
- This is an **ingress/phase mismatch at the producer-to-carrier seam**. The
  boundary guard is right. A planner's selected call is prospective: the
  existing measurement cannot establish lowering-time consumption, because
  preflight aborts first.
- Row 14: `crates/ken-cli/tests/rt_span_prov_native.rs`,
  `sp_a_foreign_span_freeze_rejects_own_span_succeeds_on_both_engines`, is
  ignored on the same refusal sentence. Whether it takes the same path is
  **unmeasured**.

## Deliverables

- **D0.** Take row 14's first refusal off the `sp-a-freeze` helper thread and
  report its identities and path in the same form as row 13's. Same path means
  this node owns row 14 too. A different path means row 14 is out of scope;
  say so.
- **D1.** Bind the exact Release `Vis`/caller/owner identity to the
  value-construction path, by one of the Architect's two horns:
  - **(A)** show, from a **completed** lowering, that the selected call is
    truly unconsumed, then demote that identity consistently throughout
    planning and lowering; or
  - **(B)** keep it Specialized and make the constructor cross only **after**
    the exact owner produces a real, validated response value, by owner-local
    construction or an explicit checked transport seam.

  Before writing code, return a short sketch naming the horn and the
  mechanism to the runtime leader and the Architect. **No particular mechanism
  has been proved feasible, and a planned K transport alone is not evidence.**
- **D2.** Un-ignore the rows this node owns, and land `RT-PROCESS-EXIT-STATUS`'s
  corrected annotation (checkpoint `d47f1f990`) with this candidate.

## Acceptance criteria

- **AC-1.** Both `StaticResponseDeferred` refusal arms are unchanged. An
  independent control shows an **actual placeholder** crossing the same
  represented-constructor path is still refused.
- **AC-2.** Every row un-ignored passes differentially:
  `assert_native_matches_interpreter` plus both per-engine `InvalidBounds`
  assertions for row 13. Native-only green does not count.
- **AC-3.** The handback names the exact identities bound (origins, owner,
  caller) and shows a completed lowering in which the constructor crosses only
  after its owner.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

- **Not authorized:** a new carrier representation, a source-graph rewrite,
  treating the placeholder as a `BoundaryWord`, dropping the child, turning the
  Releases Deferred on speculation, or reworking the fixture. If a horn needs
  one of these, stop and hand it to the Architect.
- **Contention with the held bracket tree:** `wp/RT-BRACKET-PRODUCER-
  AUTHENTICITY` and the child-2 checkpoint edit `planning/static_transition/
  responses.rs` heavily. If the repair must change response-owner machinery
  there, stop and return to the Steward before editing. Never move either
  held ref.
