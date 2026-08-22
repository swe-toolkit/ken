---
id: RT-COLD-LOWERING-PATH-ENUMERATION
title: "Bound the COMPLETE remaining downstream-refusal set for the cold cap41_*/rt_parity backend-lowering path in ONE pass -- clearing the effect-seat layer (RT-FSREADAT projection) and the join-consumption layer (RT-DEAD-ARM-JOIN-DISPOSITION) exposed TWO disjoint fail-closed refusals at once (materialized-dead join StaticOriginId(288) reconciliation; OrientedSubcontinuationPlanV1 IH-marker completeness), and the path has never been driven end-to-end so its true depth is unknown -- by bounded-EXHAUSTIVE enumeration of the plan authority's constructors run through the FULL lowering+validation pipeline, reporting the complete remaining refusal set (the input that sequences the per-gap successors) and landing the durable production coverage test whose ABSENCE is why these invariants surface serially (Architect layer-3 ruling evt_r3tt1gpv4tkn point 4; research advisory evt_5f0rzjghjhmy9). Validators untouched -- this measures and covers, it does not fix."
status: closed
owner: runtime
size: M
gate: none
depends_on: [RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL, RT-DEAD-ARM-JOIN-DISPOSITION]
blocks: []
github: null
origin: "Architect layer-3 ruling evt_r3tt1gpv4tkn (SCOPE/SEQUENCING handed to the Steward), on the research prior-art advisory evt_5f0rzjghjhmy9 pulled per the HS=3 §1a trigger. The advisory's spine: preservation (soundness-if-success) is NOT traversability (progress); no verified backend gets traversability for free (CompCert Compiler.v:446-469, CakeML compilerProofScript), so the stacked refusals are the absence of an established traversability discipline for this cold population, not a bug pile. Prior art (SmallCheck bounded exhaustive enumeration on a finite domain, DOI 10.1145/1411286.1411292; MLIR verify-after-every-pass over a declared target population) says stop discovering layers serially: enumerate the small plan space and run each plan end-to-end to surface ALL remaining refusals together. Steward-filed per COORDINATION section 2."
---

> CLOSED 2026-08-22 — co-land merged (cbac30826). AC-1/AC-3/AC-4/AC-SOUNDNESS met
> (bounded-exhaustive census, complete 5-mechanism refusal set, durable coverage
> test landed); Architect-verified evt_4ag90qfacmgwy; runtime-leader confirmed
> evt_10rk35dhtw1cq. The per-gap successors it sequenced are tracked separately.
> HS 5.

# WHAT THIS NODE IS

Bound the COMPLETE remaining downstream-refusal set for the cold
cap41_*/rt_parity backend-lowering path in ONE pass, then land the durable
production coverage test whose absence is why these invariants have surfaced one
at a time. This is the Architect's ruled "run the enumeration/coverage step to
bound the complete remaining set BEFORE committing to a landing sequence"
(evt_r3tt1gpv4tkn point 4). It converts "fix one, re-run, unknown depth" into
"enumerate the population, run it end-to-end, get the complete remaining refusal
set at once."

It is a MEASUREMENT-and-COVERAGE node, not a fix. It touches NEITHER validator.
Its report (AC-3) is what the Steward uses to cut the bounded per-gap successors.

# WHY FIRST (Architect ruling evt_r3tt1gpv4tkn; research advisory evt_5f0rzjghjhmy9)

The advisory's spine: preservation (soundness-if-success) is NOT traversability
(progress), and no verified backend gets traversability for free (CompCert,
CakeML). The stacked refusals on this path are the absence of an established
traversability discipline for a cold population, not a pile of unrelated bugs.

Clearing the join-consumption layer exposed TWO disjoint downstream refusals AT
ONCE -- a join/phi reconciliation (StaticOriginId(288)) and a distinct-subsystem
completeness gap (OrientedSubcontinuationPlanV1) -- and the implementer will not,
and should not have to, estimate remaining depth after two corrections in a day.
For a small, enumerable plan space the disciplined remedy is bounded exhaustive
enumeration run end-to-end, not serial discovery. The concrete gap the advisory
names is that there is "no production test" driving the cap41_*/rt_parity
population through this path -- that missing test IS why the invariants surface
serially.

# MECHANISM (Architect design input; validators untouched)

- Enumerate the plan authority's constructors for the cap41_*/rt_parity
  population bounded-EXHAUSTIVELY -- a PROVEN closure over the constructor set,
  not a grep and not a sample. Enumerate the PRODUCERS; show nothing bypasses.
- Run each enumerated plan end-to-end through the FULL lowering + validation
  pipeline (not a single validator in isolation), collecting every refusal from
  the finished pipeline.
- Report the COMPLETE remaining refusal set. Known already (>=2): the
  materialized-dead join StaticOriginId(288) reconciliation, and the
  OrientedSubcontinuationPlanV1 "computational IH invocation marker does not wrap
  a complete application" completeness gap. Any others the enumeration finds join
  the set.
- Land a durable coverage test that drives this population end-to-end in CI, so
  the path is no longer cold. It is expected RED until the per-gap successors
  land, and green when they do; it co-lands with the set.

# ACCEPTANCE

- **AC-1 (complete enumeration -- closure, not grep).** Bounded-exhaustive over
  the plan authority's constructors with a stated closure argument (enumerate the
  producers, show nothing bypasses). Report the constructor census and the
  closure basis. A grep/sample that happens to hit the known two does NOT
  discharge this.
- **AC-2 (end-to-end, full pipeline).** Each enumerated plan runs through the
  full lowering+validation pipeline; refusals are collected from the finished
  pipeline, never inferred from reading one validator.
- **AC-3 (complete refusal-set report -- the deliverable that sequences the
  successors).** Deliver the complete remaining refusal set: the >=2 known plus
  any others, each with its witness and terminating subsystem. If the set is
  exactly the two known, state that WITH the closure basis that makes "exactly
  two" a claim rather than an observation.
- **AC-4 (durable coverage test co-lands).** A durable test drives the
  cap41_*/rt_parity population end-to-end; expected RED until the per-gap
  successors land, green when they do. Lands with the co-landing set.
- **AC-SOUNDNESS (validators untouched).** Touches NEITHER
  validate_materialized_dead_join_cfg NOR the OrientedSubcontinuationPlanV1
  completeness check -- both are correct fail-closed boundaries (Architect
  ruling point 1; prior art unanimous). This node measures and covers; it does
  not fix or relax.
- **AC-NO-REGRESSION.** No lowering on main changes; whole-suite green in CI
  (`COORDINATION §12`). Local targeted `-p` only, never `--workspace`; runtime
  respin gate `-p ken-runtime` all-binaries + `-p ken-cli` + `-p ken-verify`.
- **Required reviewer.** Architect (traversability-discipline design fit +
  validators-untouched). The Adversary hunts the one over-accept shape: an
  INCOMPLETE enumeration reported as complete -- a census whose closure basis is
  a grep or a sample, so a third refusal that the enumeration cannot reach reads
  as "complete set = the two known."

# CO-LANDING AND ANTICIPATED SUCCESSORS

Runs on `wp/RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL`, on top of the built
(un-landed, measured-sound) carried-buffer projection + join disposition -- which
green nothing alone ONLY because of these pre-existing downstream gaps (Architect
ruling point 5; do NOT revert them, they are part of clearing the path). Per §8
the whole set co-lands as ONE green candidate once the per-gap fixes land and the
cap41_* rows go all-green.

This node's AC-3 report is what the Steward uses to cut the bounded per-gap
successors -- anticipated, each cut on its own reconciliation/completeness merits
with validators untouched, once this node bounds the set:

- **join-288 reconciliation** (Architect point 2): a join dead-classified but
  still reachable at final validation is a genuine disagreement between two
  reachability views -- determine whether 288's pre-existing "dead" disposition
  is stale/false (288 is actually reachable) or the CFG retained a live edge that
  should have been eliminated (predecessor removed, successor PHIs repaired), and
  correct THAT side. A reconciliation, never a blind drop of either side.
- **OrientedSubcontinuationPlanV1 IH-marker completeness** (Architect point 3 +
  addendum evt_1a8tf8776fd6m): the reachable-but-incomplete computational
  IH/eliminator invocation marker. Prior art (GHC join points: every occurrence
  a saturated same-arity tail call or invalid Core) pins the direction -- fix the
  incompleteness, do NOT relax the boundary. The CPS/subcontinuation-planning
  prior art (research addenda evt_3p0rwsjw51mjq) is a fix-time input for the
  builder.

# NOT IN SCOPE

- Fixing either downstream mechanism -- those are the successors this node's
  report cuts.
- Weakening or bypassing either validator.
- Any kernel / TCB edit. `ken-runtime` cranelift lowering + planning throughout;
  no operator authorization is in play.

# SEQUENCING / CONTENTION / CAPABILITY TIER

`depends_on: [RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL, RT-DEAD-ARM-JOIN-DISPOSITION]`
-- it needs the effect-seat and join-consumption layers cleared (the built,
un-landed work) to surface the downstream set. Blocks the per-gap successors and
RT-FSREADAT's full AC-1/AC-4/AC-5 green. Same branch, ring, and thread
(`thr_3j5ew8rhy35nh`) as the co-landing set; no other lane touches `ken-runtime`
cranelift lowering (`joins.rs`) or planning
(`crates/ken-runtime/src/cranelift_backend/planning/`).

Tier T1: the completeness/closure argument for the enumeration is load-bearing --
an incomplete enumeration returns a false "complete set" and mis-sequences every
downstream successor -- and the report drives all subsequent sequencing.
runtime-implementer's Opus seat is correct. Size M.
