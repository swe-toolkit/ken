---
id: RT-MATERIALIZED-DEAD-JOIN-RECONCILE
title: "Reconcile the materialized-but-dead source-join disagreement as a CLASS over StaticOriginId(288) AND (301) (8 entries in RT_PARITY_SOURCE) -- a join dead-classified yet still reachable at final validation is a genuine disagreement between two reachability views, so determine which view is stale (the pre-existing dead disposition is false and the origin is actually reachable, OR the CFG retained a live edge that should have been eliminated -- predecessor removed, successor PHIs repaired) and correct THAT side, never a blind drop of either. validate_materialized_dead_join_cfg stays byte-untouched -- it is the correct fail-closed boundary that CAUGHT this; the fix is on the producing side. Bounded by RT-COLD-LOWERING-PATH-ENUMERATION report 1 (Architect ruling evt_r3tt1gpv4tkn point 2 + sharpened evt_4ag90qfacmgwy: this is a class over both origins, not an instance)."
status: closed
owner: runtime
size: M
gate: none
depends_on: [RT-COLD-LOWERING-PATH-ENUMERATION]
blocks: []
github: null
origin: "Bounded successor cut by the Steward from RT-COLD-LOWERING-PATH-ENUMERATION's AC-3 report (runtime-implementer evt_1m6eg23vnbj4n; Architect verified evt_4ag90qfacmgwy). The materialized-but-dead join refusal is a CLASS, not an instance: origins 288 AND 301, 8 entries across RT_PARITY_SOURCE. Architect ruling point 2 (evt_r3tt1gpv4tkn) fixes the RECONCILIATION mechanism, never a drop of either side; the sharpened review requires the AC to exercise both origins as a non-degenerate pair. Steward-filed per COORDINATION section 2."
---

> CLOSED 2026-08-22 — co-land merged (cbac30826). The 288+301 reachability-vs-
> consumption reconciliation (the consumed-guard fix, 8/8 cleared) WAS the landed
> deliverable, Architect-approved evt_7rsy01s7k1d7x; runtime-leader confirmed
> nothing pending on this node's AC (evt_10rk35dhtw1cq). validate_materialized_
> dead_join_cfg stayed byte-untouched (the fail-closed boundary that caught it).
> HS 5.

# WHAT THIS NODE IS

Reconcile the materialized-but-dead source-join disagreement surfaced by the
cold-lowering enumeration -- as a CLASS over `StaticOriginId(288)` AND
`StaticOriginId(301)` (8 entries in `RT_PARITY_SOURCE`), not as a single-origin
patch. `validate_materialized_dead_join_cfg` refuses a source join that is
materialized (consumed AND dispositioned) yet whose blocks are still reachable
at final validation. That is a genuine disagreement between two reachability
views, and the fix reconciles it on the PRODUCING side. The validator stays
byte-untouched -- it is the correct fail-closed boundary that caught this
(Architect ruling point 1; prior art unanimous).

# THE MEASURED FACT (RT-COLD-LOWERING-PATH-ENUMERATION report 1; Architect evt_4ag90qfacmgwy)

The enumeration ran all 11 admissible `RT_PARITY_SOURCE` entries end-to-end and
collected every refusal. The materialized-dead join refusal appears at TWO
origins, 288 and 301, across 8 entries -- a class the enumeration turned from a
latent single-origin miss into a visible pair. A successor scoped to 288 alone
would green one origin and leave 301.

# MECHANISM (Architect design ruling evt_r3tt1gpv4tkn point 2; validator untouched)

For each origin, determine WHICH reachability view is stale -- never assume,
never drop either side:

- Either the origin's pre-existing "dead" disposition is stale/false and the
  origin is actually reachable in the finished CFG (then the disposition is the
  wrong side -- it should not have been dispositioned dead);
- or the CFG retained a live edge into the origin's blocks that should have been
  eliminated (predecessor removed, successor PHIs repaired), and the CFG is the
  wrong side.

Correct the side that is wrong. This is a reconciliation of the producing
passes, not a weakening of the validator and not a blind drop of a
disposition or an edge.

# ACCEPTANCE

- **AC-1 (both origins, non-degenerate pair).** The fix exercises BOTH 288 and
  301; the 8 affected `RT_PARITY_SOURCE` entries no longer stop at
  `validate_materialized_dead_join_cfg`. Report the per-origin reconciliation
  (which view was stale, what was corrected) for each. A fix that greens 288 and
  leaves 301 does NOT discharge this.
- **AC-2 (reconciliation, never a drop).** For each origin the correction is on
  exactly the stale side, argued from the finished CFG -- never a blanket drop of
  the dead disposition (that would statically-unselect a reachable region, a
  miscompile) and never a blanket drop of the retained edge without repairing the
  successor PHIs.
- **AC-SOUNDNESS (validator byte-untouched).** `validate_materialized_dead_join_cfg`
  is unchanged -- it stays the fail-closed backstop. A wrongly-reconciled origin
  must STILL trip it. Ship one durable test in that shape (a reconciliation that
  leaves a live origin dispositioned dead trips the validator).
- **AC-NO-REGRESSION.** No lowering currently on `main` changes disposition;
  whole-suite green in CI (`COORDINATION §12`). Local targeted `-p` only, never
  `--workspace`; runtime respin gate `-p ken-runtime` all-binaries + `-p ken-cli`
  + `-p ken-verify`.
- **Required reviewer.** Architect (reconciliation-direction soundness +
  validator-untouched). The Adversary hunts the two over-accept shapes: a blind
  drop of either the disposition or the edge, and a reconciliation of 288 that
  degrades 301.

# CO-LANDING

Part of the RT-FSREADAT co-landing set (§8) on
`wp/RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL` (thread `thr_3j5ew8rhy35nh`). The
whole set lands as ONE green candidate once the `cap41_*`/terminal rows go
all-green. Do NOT revert the built projection + join disposition (Architect
ruling point 5).

# NOT IN SCOPE

- Weakening or bypassing `validate_materialized_dead_join_cfg` -- it stays the
  fail-closed boundary.
- The IH-marker completeness gap ([[RT-IH-MARKER-PRODUCER-COMPLETE]]), the
  BoundaryCarrier layer-1 witness (folded into
  [[RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL]]), the closure-crossing standing
  limitation (tracked in the merged RT-CLOSURE-BOUNDARY-LANE lane, excluded from
  this arc), and the `rt_write_pair_source` elaboration TypeMismatch (not a
  lowering refusal; separate disposition) -- each is its own line in the ledger.
- Any kernel / TCB edit. `ken-runtime` cranelift lowering (`joins.rs`); no
  operator authorization is in play.

# SEQUENCING / CONTENTION / CAPABILITY TIER

`depends_on: [RT-COLD-LOWERING-PATH-ENUMERATION]` -- bounded by its report 1.
Same branch, ring, and thread as the co-landing set; `ken-runtime` cranelift
lowering (`joins.rs`), no other lane touches it.

Tier T1: a soundness-adjacent reconciliation on the source-join layer with a
mandatory negative control; the review turns on the per-origin stale-view
argument, not a diff. Size M (two origins, 8 entries).
