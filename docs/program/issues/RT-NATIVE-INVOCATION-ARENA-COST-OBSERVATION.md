---
id: RT-NATIVE-INVOCATION-ARENA-COST-OBSERVATION
title: "A native run reports a capacity fault only as the count at the first allocation past a limit, so a promoted operation's arena cost can be found only by bisecting builds against raised profiles, and no row can detect a change in that cost. Report each activation's end-of-run invocation-region counts in the native effect observation, which are its high-water marks because the arena only grows, and pin ResourceStreamWrite's per-operation cost by an operation-count differential"
status: merged
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-NATIVE-RESOURCE-STREAM-WRITE-PROMOTION]
blocks: []
github: null
origin: "Architect evt_1k5678p2q6ztk (successor) on RT-NATIVE-RESOURCE-STREAM-WRITE-PROMOTION Increment 2's post-write capacity stop, resumed on research advisory evt_3cw1xsbpeg2jp. The Inc2 parity rows run under a non-binding 4x ceiling with a stated GAP (no capacity measured); this node closes that gap for every promoted operation. Steward-filed per COORDINATION section 2."
---

# Native runs report their invocation-arena high-water marks

## Objective

A native effect run reports, for each activation, the invocation region's
node, word, data-byte and native-Int-limb counts at activation end. A
promoted operation then gets a cost row that pins its per-operation
delta, not the fixture's level. ResourceStreamWrite is the seed.

## Settled inputs (main `7412b0deb`; re-anchored at AC-0 on `e5177f7c1`)

- **The arena only grows.** The emitted allocator's only stores to the
  counts are `count + 1` and the new word total
  (`crates/ken-runtime/src/boundary_value_clif.rs:1809`-`:1813`, the sole
  stores to `ARENA_NODE_COUNT` and `ARENA_WORD_COUNT`). The count at
  activation end is therefore the activation's high-water mark.
- **What a fault reports today.** The region check
  (`crates/ken-runtime-support/src/activation_abi.rs:340`-`:355`) refuses
  when the current count plus the allocation's need exceeds the limit,
  and reports the count at that allocation, not a peak.
- **The limits.** `BoundaryRegionLimitsV1`'s four fields
  (`ken-runtime-support/src/boundary_resource_profile.rs:133`-`:141`).
- **Why no row detects cost today.** The Inc2 write rows run under a
  4x ceiling and state "THE GAP: this row measures no capacity; a change
  in the write's arena cost is not detected here" (Architect
  `evt_1k5678p2q6ztk`, rule 2).
- **The model.** A cost row of the Go `AllocsPerRun` kind: sources
  differ only in operation count, and the row pins the delta against a
  stated expected cost, with a third point for constancy (research
  `evt_3cw1xsbpeg2jp`).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Amendment: AC-1 affine model (2026-11-18)

The stop "the deltas at N = 1, 2 and 3 are not constant" fired (F round,
`evt_3fmjm3zvm3f7d`). Architect `evt_3cqsj6ncrym8t`: the growth is the
continuation's captured environment, not the write. Host-computation
lowering inlines each non-recursive proc (`erasure.rs:4393`-`:4475`), and
every host-value closure captures its whole runtime context
(`:3240`-`:3243`), so each inlined level adds its binders to the next
environment. Steward disposition: Option A, conditional on probe G landing
in arm M1. AC-1 becomes the affine model the measurement supports:

- write sources at N = 1, 2, 3 and 4 (the F family);
- the first difference D1 pinned at (6, 103, 0, 0) invocation and
  (2, 3, 0, 0) persistent;
- the second difference pinned constant at (0, 6, 0, 0) invocation and
  zero persistent, with CLAIMED naming it the per-level captured binder
  width (5 parameters plus the λ binder);
- the payload variant on D1, and the 8x control.

AC-2 is unchanged: one extra invocation node per write moves every first
difference. The GAP line states that a change to capture policy, such as
free-variable capture, also reddens the row and re-rules the literals,
and is not a write regression. If G lands in M2 or M3, the stop returns
to the Architect and this amendment does not apply.

Withdrawn (2026-11-18): probe G landed in M2 (`evt_3apy9bhbacc6c`), so the
capture-width derivation is falsified and this amendment does not apply.
The Architect holds the ruling for a research advisory at §1a 3
(`evt_5j2zq0t2jkhes`). AC-1 stands as originally written until the next
ruling; the measured D1 and second difference are unchanged.

## Amendment: AC-1 pins D1 only (2026-11-18)

Architect census attribution `evt_68ym86q2hxtss`, arm C1: the 6 words per
level are 3 dead captured fields on each of the 2 closure records per
write. The fields are the write's HostResult and its two pattern-bound
reply values, and no later code reads them (classification (ii)).
Steward disposition: AC-1 and its first stop condition are replaced as
follows.

- **AC-1.** Pin D1 only, F1 to F2: (6, 103, 0, 0) invocation and
  (2, 3, 0, 0) persistent. The payload variant equals D1, and the 8x
  control is equal.
- **CLAIMED.** "D1 is the first chain level's per-write cost on the
  current lowering. Its invocation words include the capture of dead
  binders at that depth and R's inline copy of its environment. It is
  measured from the region_end header counts."
- **GAP.** "The per-write invocation word cost grows by 6 words per
  earlier chain level, because every later capture record retains each
  earlier write's HostResult and its two pattern-bound reply values
  (`LANG-NATIVE-CLOSURE-DEAD-BINDER-CAPTURE`). D2 and D3 are not pinned.
  Fixing capture policy or R's layout lowers D1, and that re-rules the
  literal. Such a change is not a write regression."
- **AC-2 is unchanged.** One extra zero-field invocation node per write
  moves D1 by exactly one node.
- **Candidate hygiene.** `census_probe_write` and its call are not in the
  candidate. The old ac1 test's withdrawn assertions are replaced by the
  D1 pin. The W and G fixtures need not stay.

Symptom inventory (§1b):

1. Recorded in this WP's thread (`evt_7p7d7cjqzs23c`, the persistent-reply
   lane).
2. The per-write invocation word cost grows by 6 words per chain level,
   because native host-computation lowering inlines non-recursive procs
   and every lambda captures its whole runtime context, not its free
   variables. Keyed on the static inline depth of the continuation site,
   not on the write's request or reply.
3. The source-derived localization (inlined proc parameters widening a
   whole-context continuation capture by 6 per level) was falsified: an
   added pass-through parameter on every inlined level moved no count.
   Keyed on the elaborator's source-level binder count, not on what the
   lowered plan allocates. The Architect rules the three entries share one
   predicate: the cost model is read from source, not from the lowered
   allocation plan (`evt_5j2zq0t2jkhes`).

## Scope

The end-of-activation count report in the native effect observation, and
the ResourceStreamWrite cost rows. No change to the limits, the region
check, the trace's existing events, or any availability label.

## Deliverable

One runtime candidate.

## Acceptance

- **AC-0 (D0, Architect, before code).** On the landed Inc2 base, rule
  where the counts are read (the region header at activation end), where
  they are carried (the observation, a side record, or a test-only
  accessor), and whether that is an ABI or observation-format change.
  Rule the expected per-write cost, or how it is measured once and
  stated.
- **AC-1.** Native write sources at N = 1, 2 and 3 writes with the same
  payload report end counts whose differences are equal and match the
  stated per-write cost. One payload-length variant shows whether the
  word or data-byte cost depends on the payload, as ruled.
- **AC-2.** A disposable mutation that adds one allocation per write
  turns AC-1 red at the delta, while the fixture level would still pass
  under the 4x ceiling. Restore it byte-identically.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The report needs a change to the region check, the limits, or an
  existing trace event's meaning.
- D1, its payload variant or the 8x control differs from the pinned
  vectors on the candidate's base.

Checks: 2 fired. I read the allocator stores at
`boundary_value_clif.rs:1809`-`:1813` on main: the increment and the new
word total. That they are the sole stores is the Architect's census
(`evt_1k5678p2q6ztk`, check 2), which AC-0 re-confirms on its base. 8 fired. AC-2's
per-write leak passes any ceiling row but changes the delta, so AC-1 is
red exactly when cost moves per operation.
