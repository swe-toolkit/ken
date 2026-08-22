---
id: RT-COLD-LOWERING-CHECKED-FAMILY-ENUMERATION
title: "Bound the COMPLETE remaining downstream-refusal set for the SECOND cold-lowering witness family -- the checked-program source the four cap41_* terminal pins share -- in ONE exhaustive pass, with the same discipline RT-COLD-LOWERING-PATH-ENUMERATION applied to RT_PARITY_SOURCE. That first enumeration found FIVE mechanisms over rt_parity but the OrientedSubcontinuationPlanV1 IH-marker refusal has ZERO entries there; it provably lives in THIS family, which is on RT-FSREADAT's own AC-4/AC-5 critical path and was never enumerated. Sequencing successors off the first report alone would OMIT a real, already-identified refusal and rediscover it serially -- the exact failure the enumeration exists to end -- so the complete successor set requires enumerating this family end-to-end too (collect ALL, do not stop at the IH-marker; a second population may hide a 6th mechanism exactly as the first hid three). Validators untouched -- this measures and covers, it does not fix. (Architect binding finding evt_4ag90qfacmgwy on AC-3 report evt_1m6eg23vnbj4n; ruling point 4 evt_r3tt1gpv4tkn.)"
status: active
owner: runtime
size: S
gate: none
depends_on: [RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL, RT-DEAD-ARM-JOIN-DISPOSITION]
blocks: []
github: null
origin: "Architect review of RT-COLD-LOWERING-PATH-ENUMERATION AC-3 (evt_4ag90qfacmgwy): the node's ACs are MET and SOUND over RT_PARITY_SOURCE (AC-1 closure over the entry signature, not the name-suffix habit; 11 admissible entries; complete five-mechanism set collected in one pass), but the refusal set cut from that report ALONE is INCOMPLETE for RT-FSREADAT closure -- the IH-marker (OrientedSubcontinuationPlanV1) has ZERO entries in RT_PARITY_SOURCE and provably lives in the SECOND witness family, the checked-program source the four terminal pins share, which is on RT-FSREADAT's AC-4/AC-5 critical path and was never enumerated. The implementer correctly did NOT extend unilaterally; the Architect handed the scope call (extend the first node's AC vs. a sibling enumeration node) to the Steward. Steward decision: SIBLING node -- the first node is verified done for its population, and a second family needs its own closure argument and coverage test. Steward-filed per COORDINATION section 2."
---

# WHAT THIS NODE IS

Bound the COMPLETE remaining downstream-refusal set for the SECOND cold-lowering
witness family -- the checked-program source the four `cap41_*` terminal pins
share -- in ONE exhaustive pass, then land the durable coverage test that drives
this family end-to-end. It is the sibling of
[[RT-COLD-LOWERING-PATH-ENUMERATION]] over a distinct population: that node
enumerated `RT_PARITY_SOURCE` (the `rt_parity` family, 11 entries, five
mechanisms); this node enumerates the checked-program family the terminal pins
share.

It is a MEASUREMENT-and-COVERAGE node, not a fix. It touches NEITHER validator.
Its report (AC-3) plus the first node's report together are the COMPLETE input
the Steward uses to cut the bounded per-gap successors -- neither report alone is
complete.

# WHY THIS FAMILY, AND WHY BEFORE THE SUCCESSOR SEQUENCE (Architect binding finding evt_4ag90qfacmgwy)

The first enumeration converted unknown depth into a known set for `rt_parity`:
FIVE mechanisms in one pass, not the two anticipated. But the IH-marker
(`OrientedSubcontinuationPlanV1` "computational IH invocation marker does not
wrap a complete application") has ZERO entries in `RT_PARITY_SOURCE`. It provably
lives in the SECOND witness family -- the checked-program source that the four
terminal pins (`cap41_*`) share -- which is on RT-FSREADAT's OWN critical path
(its AC-4/AC-5) and was never enumerated.

Cutting the successor sequence off the first report alone would OMIT the
IH-marker refusal and then rediscover it serially -- the precise failure this
enumeration arc exists to end. So the Architect's ruling point 4 ("bound the
complete remaining set BEFORE committing a landing sequence",
evt_r3tt1gpv4tkn) applies to BOTH populations. A second population can hide a
6th mechanism exactly as the first hid three; the enumeration is what turns a
latent miss into a visible one before it drives sequencing.

# MECHANISM (same discipline as the sibling node; validators untouched)

- Define the family by the ENTRY SIGNATURE at the differential substitution
  site -- the shared checked-program source of the four `cap41_*` terminal pins
  -- NOT by a name-suffix habit. (The first node's recorded near-miss: keying on
  the `_stage` suffix silently dropped `rt_write_pair_source`; freeze a
  predicate, not a roster.) State the closure basis.
- Run each enumerated entry end-to-end through the FULL lowering + validation
  pipeline; collect EVERY refusal from the finished pipeline. Do NOT stop at the
  IH-marker -- run all entries to completion and report how far each got, exactly
  as the sibling runner did for `rt_parity`.
- Report the COMPLETE remaining refusal set for this family: the IH-marker known
  to live here, plus any others the enumeration finds. Each with its witness and
  terminating subsystem.
- Land a durable coverage test that drives this family end-to-end in CI, so the
  path is no longer cold. Expected RED until the per-gap successors land, green
  when they do; co-lands with the set (§8).

# ACCEPTANCE

- **AC-1 (complete enumeration -- closure, not grep).** Bounded-exhaustive over
  the checked-program family's entries with a stated closure argument (enumerate
  the producers/entries, show nothing bypasses). Report the census and the
  closure basis. A grep/sample that happens to hit the known IH-marker does NOT
  discharge this.
- **AC-2 (end-to-end, full pipeline).** Each enumerated entry runs through the
  full lowering+validation pipeline; refusals are collected from the finished
  pipeline, never inferred from reading one validator. Collect ALL -- do not stop
  at the first refusal or at the IH-marker.
- **AC-3 (complete refusal-set report -- the deliverable that, WITH the first
  node's report, sequences the successors).** Deliver this family's complete
  remaining refusal set: the IH-marker plus any others, each with its witness and
  terminating subsystem. If the set is exactly the IH-marker, state that WITH the
  closure basis that makes "exactly one" a claim rather than an observation.
- **AC-4 (durable coverage test co-lands).** A durable test drives the
  checked-program family end-to-end; expected RED until the per-gap successors
  land, green when they do. Lands with the co-landing set.
- **AC-SOUNDNESS (validators untouched).** Touches NEITHER
  `validate_materialized_dead_join_cfg` NOR the `OrientedSubcontinuationPlanV1`
  completeness check -- both are correct fail-closed boundaries (Architect ruling
  point 1 + IH-marker addendum evt_1a8tf8776fd6m; prior art unanimous: GHC join
  points, Lean IR checker, typed-CPS). This node measures and covers; it does not
  fix or relax.
- **AC-NO-REGRESSION.** No lowering on `main` changes; whole-suite green in CI
  (`COORDINATION §12`). Local targeted `-p` only, never `--workspace`; runtime
  respin gate `-p ken-runtime` all-binaries + `-p ken-cli` + `-p ken-verify`.
- **Required reviewer.** Architect (traversability-discipline design fit +
  validators-untouched). The Adversary hunts the one over-accept shape: an
  INCOMPLETE enumeration reported as complete -- a census whose closure basis is
  a grep or a sample, so a refusal the enumeration cannot reach reads as
  "complete set = the IH-marker alone."

# THE COMPLETE SUCCESSOR LEDGER (both reports; successors cut only AFTER this node reports)

The first node's report (`RT_PARITY_SOURCE`, five mechanisms) plus this node's
report (checked-program family) together bound the whole set. The successor
sequence is committed only when BOTH have reported. Recorded dispositions, from
the Architect's sharpened directions (evt_4ag90qfacmgwy); each is still ruled on
its own merits at fix time, validators untouched:

1. **Materialized-but-dead join reconciliation -- a CLASS, not an instance**
   (origins 288 AND 301, 8 entries). The successor fixes the RECONCILIATION
   mechanism -- determine which reachability view is stale (the dead disposition
   or the CFG's retained live edge), never assumed, never a blind drop of either
   side -- and its AC must exercise BOTH origins as a non-degenerate pair.
2. **BoundaryCarrier carried-recursive-hypothesis** (`rt_allocate_stage`, 1
   entry) is the LAYER-1 effect-seat family still live -- a new witness of the
   SAME mechanism the predecessors cleared, not a new class. Fold into / extend
   the layer-1 fix; diagnose same-family at fix time.
3. **Closure-cannot-cross-the-boundary** (`rt_write_writable_stage`, 1 entry)
   matches `RT-CLOSURE-BOUNDARY-LANE`'s standing ignore reason -- route to that
   EXISTING node, NOT a new cold-lowering successor. A tracked standing
   limitation, correctly excluded from this arc.
4. **Elaboration KernelRejected TypeMismatch** (`rt_write_pair_source`, 1 entry)
   is NOT a lowering refusal -- it never reaches the backend, so it is out of the
   cold-lowering arc either way. Disposition must be made explicit, not dropped:
   diagnose whether the TypeMismatch is a CORRECT rejection of a genuinely
   ill-typed fixture (then it is not a defect -- record why, and the lowering
   population is effectively 10) or a real elaboration gap (then it routes to the
   language/elaboration owner as a separate concern).
5. **IH-marker completeness** (`OrientedSubcontinuationPlanV1`) -- this family's
   refusal. The sound fix is attributed to the PRODUCER/transform, never the
   validator (Architect IH-marker addendum evt_1a8tf8776fd6m, confirmed by
   research evt_3p0rwsjw51mjq): diagnose which producer step created the
   plan/marker disagreement, then resolve to ONE of the two lawful,
   non-interchangeable representations -- (i) it IS complete => fix the producer
   so plan and marker agree on a full `Call` of the checked arity; or (ii) it is
   GENUINELY partial => the distinct closure/PAP form with its own later apply.
   NEVER pad the call, infer missing arguments at emission, or reinterpret a
   full-call node as partial. Same shape as mechanism 1: keep the checker,
   correct the producer.
6. **Any 6th** this enumeration surfaces joins the ledger with its witness.

# CO-LANDING

Runs on `wp/RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL` (thread `thr_3j5ew8rhy35nh`),
on top of the built (un-landed, measured-sound) carried-buffer projection + join
disposition -- do NOT revert them (Architect ruling point 5). Per §8 the whole
set co-lands as ONE green candidate once the per-gap fixes land and the
`cap41_*`/terminal rows go all-green; RT-FSREADAT's full AC-1/AC-4/AC-5 wait on
that. The single merge Decision covers every co-land member.

# NOT IN SCOPE

- Fixing any downstream mechanism -- those are the successors the two reports cut.
- Weakening or bypassing either validator.
- Re-enumerating `RT_PARITY_SOURCE` -- [[RT-COLD-LOWERING-PATH-ENUMERATION]]
  already did, verified by the Architect. This node is the DISJOINT second family.
- Any kernel / TCB edit. `ken-runtime` cranelift lowering + planning throughout;
  no operator authorization is in play.

# SEQUENCING / CONTENTION / CAPABILITY TIER

`depends_on: [RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL, RT-DEAD-ARM-JOIN-DISPOSITION]`
-- it needs the effect-seat and join-consumption layers cleared (the built,
un-landed work) to surface this family's downstream set, exactly as the sibling
enumeration did. Same branch, ring, and thread as the co-landing set; no other
lane touches `ken-runtime` cranelift lowering (`joins.rs`) or planning
(`crates/ken-runtime/src/cranelift_backend/planning/`).

Tier T1: the completeness/closure argument for the enumeration is load-bearing --
an incomplete enumeration returns a false "complete set" and mis-sequences every
downstream successor. Small (S): it reuses the sibling node's runner discipline
over a second, smaller population. runtime-implementer's Opus seat is correct.
