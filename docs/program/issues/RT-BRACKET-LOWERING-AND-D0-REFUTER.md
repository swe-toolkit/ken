---
id: RT-BRACKET-LOWERING-AND-D0-REFUTER
title: "CHILD 3 of RT-BRACKET-CONTROL-REGION-IR (after 1, 2a and 2b), AND THE ONLY ONE THAT ROUTES A CANDIDATE. The lowering consumer, the D0 EQUIVALENCE REFUTER, and closeout. Lowering receives ONLY a validated BracketSettlementControlView and consumes settlement, outcome merge and resume in order; an affine compiler ledger closes over exact region and edge identities. D0 IS NOT A CENSUS: it compares interpreter and linked-native normalized observation tuples over five mandatory rows and requires three compile-preserving mutations, and it CAN REFUTE THE REPRESENTATION where population equality provably cannot. ASSEMBLE CHILDREN 1, 2a AND 2b HERE AND ROUTE ONLY THE COMBINED TREE AS ONE ATOMIC CANDIDATE -- children 1, 2a and 2b are held, reviewed inputs and never independently landed products. Closeout un-ignores px8ta_oriented_subcontinuation::public_two_three_level_brackets_finish_and_release_lifo on BOTH its blockers and removes an order-excluding helper ONLY after both engines agree."
status: active
owner: runtime
size: L
gate: none
tier: T1
depends_on: [RT-BRACKET-SETTLEMENT-PLANE, RT-BRACKET-PRODUCER-ROUTE-NESTED-BODY-ORDER]
blocks: []
github: null
origin: "Steward recut 2026-09-22 on the Architect's second WIP audit verdict evt_3rvns2yxm898r. THIS IS THE LARGEST OF THE THREE CHILDREN AND THE STEWARD MIS-SIZED THE PARENT TWICE BEFORE; the split point if it proves too big is named in section 6 rather than discovered, and the Steward recuts at the FIRST audit here rather than the second. Pre-registration commit c22f4861d already satisfies the sequencing half of AC-14 and must be preserved; evidence checkpoint 81f222b7f012829cd9f8d0f3dc684410a9b2b9ee is EVIDENCE, NEVER A CANDIDATE."
---

> **Recut 2026-10-23 (Architect D0 `evt_3yked97dme4gw`).** This child
> lands together with link 3 (RT-BRACKET-SETTLEMENT-PLANE) as one
> candidate, fresh on `main`. It is the lawful replacement for link 3's
> emission refusal (COORDINATION §8a). Links 1 (`77352e68e`) and 2
> (`b6adca031`) are on `main`. Do not use the held assembly `0b4975281` as
> a base. Its lowering checkpoints R5 to R8 (`0b4975281..8d3731a3d`) are
> references only, and so are the held refs.
> RT-BRACKET-PRODUCER-ROUTE-NESTED-BODY-ORDER is rebuilt last
> (`evt_1gxk3pcr6rgw2`).
>
> **D0 first (Architect).** After the link 3 coverage census, rule the
> lowering unit on link 3's port: what ports from R5 to R8, and the px8ta
> depth-3 row's disposition until NESTED-BODY-ORDER lands. The bar is zero
> main-green regressions across the ken-cli bracket population.

## 1. Deliverable

- **Lowering consumer.** Receives ONLY a validated
  `BracketSettlementControlView`. Body completion consumes its settlement edge,
  lowers the existing release through EXISTING host-effect machinery, consumes
  outcome merge, then consumes resume. Inline versus an existing private control
  unit is secondary.
- **Counting ledger** over exact region and edge identities (Architect
  `evt_32mpqmy5v0y6s`, R6a). A marked S may be lowered once per control
  path, so each region counts settlement, outcome merge and resume, in
  that order. Every lowering takes its mode's route, no unplanned edge is
  consumed, and each region closes with settlement at least 1 and all
  three counts equal. There is no exemption for zero lowerings. **This
  proves REPRESENTATION CLOSURE, not semantic equivalence.**
- **`D0`, the equivalence refuter.** Pre-registered BEFORE it is run -- the
  pre-registration already exists at `c22f4861d` and is preserved.
- **Closeout.** Un-ignore the px8ta row; remove an order-excluding helper ONLY
  after both engines agree.
- **Assembly.** Children 1 and 2 are combined here and the combined tree is the
  ONLY thing routed as a candidate.

## 2. `D0` -- the observation

```text
(exit status or exact ground result,
 terminal error,
 ordered host-effect trace,
 acquire-relative resource identities,
 host-result/outcome class)
```

Compare interpreter and linked-native tuples on the SAME checked input.
Normalize resource identities ONLY by acquisition ordinal. Do not compare sets,
and do not compare release vectors alone.

**Five mandatory rows, not a subset:**

1. px8ta depths 2 and 3 -- strict LIFO and no emission refusal;
2. all four two-operation Mapping programs -- parity and no terminal `-1`;
3. both alternating Mapping programs -- parity remains green;
4. O0/read-offset and W0/read-window -- O0 live, W0 preserves main;
5. one acquisition-failure case and one body-error/release-error discriminator,
   which observe the OUTCOME MERGE and not merely order.

**Further measured members** (RT-NESTED-RELAY D0, `evt_3y63h26apw108`,
folded in at `evt_5q52byr2tg4x2`). They join the D0 comparison as depth-2
rows, beside the five above:

- out-of-bounds `BRANCHED_SCRUTINEE` on main `c87de89d3`: rc 72 in both
  engines; native releases FsHandle 1 before Buffer 2, interp Buffer 2 first
  (log SHA-256 `ce265ee4`);
- `rt_parity_native` row 01 builds 0 and 2, rows 14 and 17, and
  `abi_s6_mapping_surface_native` row 34 builds 0 and 1, which flip the same
  way (42-unit inventory SHA-256 `3acfab1a`);
- the order-excluding assertions these rows pass through today:
  `rt_parity_native.rs:597-677` and `abi_s6_mapping_surface_native.rs:575-597`.

**Three compile-preserving mutations, each with an unmutated positive sibling
and a POSITIVE executed-count assertion:**

- reverse one REAL child/parent settlement edge without changing its endpoints
  -- ordered observation must RED;
- bypass one REAL outcome-merge edge while preserving settlement emission --
  the result/outcome comparison must RED INDEPENDENTLY OF ORDER;
- assign one settlement both `BracketOwned` and `Specialized` -- plan closure
  must reach the exact dual-owner refusal.

Every equivalence pin states:

```text
MEASURED: exact normalized interpreter/native observation tuple.
CLAIMED:  explicit bracket control preserves result propagation and effect order.
THE GAP:  closed only for the exercised path plus region/edge closure, never by
          call coverage.
```

## 3. Acceptance criteria

- **AC-1.** All five `D0` rows pass with the pre-registered tuple and a recorded
  MEASURED / CLAIMED / THE GAP pin each.
- **AC-2.** Identities normalized by ACQUISITION ORDINAL only. A candidate that
  compares sets, or release vectors alone, FAILS this AC even when green.
- **AC-3.** All three mutations behave as section 2 requires. A mutation that
  stays green is a STOP, not a finding to explain.
- **AC-4.** The ledger is affine and the text recording it says outright that it
  proves representation closure and NOT semantic equivalence.
- **AC-5.** The px8ta row is un-ignored and green on BOTH blockers: strict LIFO
  at depth 2 AND no emission refusal at depth 3.
- **AC-6.** `N` from `64fd9e6abf63072b3222124b6df4c546b7a1d243` is used ONLY as a
  non-bracket regression control, never as bracket-ownership input.
- **AC-7.** The order-excluding helper is removed only AFTER both engines agree,
  never before.
- **AC-8.** One atomic candidate carrying children 1 (route C,
  `c462fe831`), 2a, 2b and 3. No partial landing (Architect sizing D0
  `evt_51cm672pdy1r1`).

## 4. Stop condition

**STOP, and the deliverable is the stop and its evidence, if** any mandatory
`D0` observation differs between the engines, either behavioral mutation stays
green, a marked region is ambiguous, or the existing typed result path cannot
carry the sequence.

**STOP if depth 3 still refuses on `ContinuationSpecialization` after
`BracketOwned` lowering.** Do not widen `function_local.continuation_calls`
(Architect `evt_51cm672pdy1r1`). That `BracketOwned` lowering clears both
px8ta blockers is a prediction; this stop is how it is tested.

On the last one, **do not reopen `RT-PLANNER-KRET-GRAFTED-SPINE`** and do not
invent a return protocol inside it. **No census, narrowed matrix, carve-out,
pending obligation or fallback.** It comes to the Steward and then the operator.

## 5. Scope

`lowering/core.rs`, the control view and ledger, the differential harness, and
the px8ta closeout. **No new carrier, return protocol, KRET lane, host
operation, dispatcher reorder, trace sort or fallback.**

A `Trap` in a bracket Body stays refused by 2b's "body trap has no
settlement edge" (Architect `evt_4hqrrtzpypd83`). Trap settlement needs a
return protocol, so it is out of scope here and goes to a later node.

Lowering must claim every marked S by its plane key (Architect
`evt_1yb966jffc2zr`). A marked S with no response owner, such as
multi-leaf's, otherwise reaches the origin-only `None` Construct arm
(lowering/core.rs ~:16680) and is lowered ordinarily. Lowering also claims S for
every owner in `PendingVisSettlements.bracket_exits`, whatever its ordinary
class (Protocol, RetOnly, `Excluded{Relay}`, `Excluded{Underived}`), and
replaces 2b's emission refusal for them. Its D0 counts that no S is
emitted by any ordinary arm (Architect `evt_5yqxd0jyc9bjj`).
File-backed-in-FS's bracket emission is masked at 2b by the earlier
`FSOp` unavailable-lane refusal (Architect `evt_274rqjw2c74k2`). Child
3's D0 exercises that owner's claim once the lane exists, or names a
substitute source that reaches the claim.
Relay admission lost through a marked S is S's ownership, so lowering
claims or re-settles it (Architect `evt_392v8fmvem7z0`): relay 352 on
nested-two's owner0, forwarding Q1 S743, and on file-backed-in-FS's owner0,
forwarding Q1 S645. Child 3's D0 counts both.

**Assembly and emission (Architect `evt_4fj9r29qyx40k`,
`evt_6vx1v2n5y3czj`, `evt_1r37j7z3dgpbm`).** Child 1 is route C only:
one `--no-ff` merge of 2b's `09d607a10` onto the pre-registration
checkpoint carries route C,
2a and 2b at their reviewed SHAs. Route D `4b4c8565c` is never merged,
cherry-picked or rebased. The marked S's Phase A demand is retained and
resolved in Phase B into a disjoint `bracket_rows` table outside the
plane, keyed by `BracketSettlementId`; no Phase B classifier sees it.
D0-E measured all 13 regions Continuation-keyed (Architect
`evt_1r37j7z3dgpbm`).

**Settlement authority (recut, Architect `evt_7r2aae0wx4hm6`, R8
`evt_4y4mx4cda73sc`; Research `evt_13kknengs4121`).** An S whose region
or a plane ancestor has an owner-returned exit (11 regions) settles in
place at every lowered copy. **A marked S's host effect lowers under its
row's planned base-owner authority at every copy; K resumes lexically.**
The producer trampoline emits S under foreign owners (S649 under
Specialization(1), K Specialization(0)), and which owner lowers a copy
keys nothing about the effect.
- `drive_response_at_source` binds `row.base_owner()` authority around
  the host effect only, releasing it before any error propagates;
  `settle_bracket_at_source` supplies it and the handler-owned driver
  passes none.
- Source (f) of the seat owners is `row.base_owner()`, with R6c's
  fail-closed check for empty `bracket_rows`. R7's drive-site owner
  check is removed.
- The transfer to a planned response-owner site is withdrawn: its K
  frame cannot be built at a foreign producer copy (D0
  `evt_3xbvjtkeg15c5`).
- Before the build, a sweep classifies every owner reader reached inside
  `lower_process_host_effect` and the two aggregate gate consumers. Any
  answer that depends on the physical function is a stop.

The other regions (multi-leaf, anonymous mapping) are Deferred on the
existing residual route, claimed in the ledger. **Retained:** route C
`c462fe831`, 2a `c18c8db68`, 2b `09d607a10`, `bracket_rows` keyed by
`BracketSettlementId` (R2), the counting ledger over static lowerings
(R6a), the claimed Deferred producer route (R6b, `evt_32mpqmy5v0y6s`),
and the per-lowering instrument over px8ta depths 2 and 3, the nine
sources and the five `rt_native_bracket_body_once` tests. Release
exactly once at runtime is the px8ta and body-once observation. The
refuter D0 runs only when px8ta reaches its observations at both depths
and every ledger closes.

## 6. If this child is itself too big, THE SPLIT POINT IS NAMED IN ADVANCE

Split at the lowering/observation seam: the lowering consumer plus the affine
ledger as one unit, and the `D0` refuter plus closeout as the next. The ledger
proves representation closure and is checkable without the equivalence tuples,
so that seam leaves neither half an inert precursor.

**The Steward recuts at the FIRST audit here, not the second.** The parent node
was carried through two audits before the cut was ruled the defect, and that
delay is the Steward's error, not the ring's.

## Hard-stop inventory (§1b)

§1a (emission seam): 4 (Architect `evt_q5j50vngtzhv`). **HELD at the §4
stop** (Architect `evt_4yf9jdc9veepy`): px8ta depth 3 differs between
the engines in both settlement modes, so the defect is in the existing
producer route. It is framed as the held input
`RT-BRACKET-PRODUCER-ROUTE-NESTED-BODY-ORDER`, assembled here. WIP
`8d3731a3d` is evidence only, and R8 is retained. A wall while building
that fix is the 5th here if the fix lands in child 3. The next Research
trigger is the 6th.
§1a (assembly): 0.

1. Marked S reached on the computational producer Construct arm
   (core.rs:3912/6965), which the in-place gate did not cover. Keyed on
   the Construct lowering arm (fan-in).
2. In-place S's host-effect seat had no synthesized-aggregate owner.
   Keyed on effect-seat owners derived from the ordinary response
   population, then from source containment.
3. On the producer arm, S is lowered under a specialization that neither
   contains S nor is its K specialization. Keyed on the settling owner,
   reconstructed from containment plus the row's base owner.
4. Depth-3 native: the outer settlement is reached on the producer route
   before the middle body runs, with the body result unwritten (tag 0).
   Keyed on producer-route sequencing of nested bracket bodies,
   independent of settlement mode.

**Shared predicate (§1b).** In-place settlement was keyed on where the S
Construct happens to be lowered, and the plan has no relation naming that
place; each entry asked that fact of a relation that does not carry it.
