---
id: RT-NATIVE-BRACKET-RETURN-CLAUSE
title: "A native bracket's return clause H, the bind's Ret case that releases and merges, drops off the eliminator stack for claims inside the bracket body's scrutinee, in the very function that opened H. So the body's raw answer finishes the region with nothing below to settle it, and reaches the resume (PX10 B-closure stop 12, live origin 420, null 979). Refuse at lowering any such claim lowered without H, and keep H on the stack at the step that loses it"
status: active
owner: runtime
size: L
tier: T1
gate: architect
depends_on: []
blocks: [PX10-NATIVE-SPAWN-PARITY]
github: null
origin: "Architect recut evt_zbd2e5wv721d (PX10 B-closure stop 12, §1b entry 16 located, sharing entries 10-13's predicate), on Research advisory evt_2v4hdh6wpzk5s; restated on the AC-0 ruling evt_59x9w96jsgxzk (Steward scope evt_10fgng94khbww). L1 objective: the Linux ABI program. Steward-filed per COORDINATION section 2."
---

# A bracket's return clause stays on the stack

## Objective

Every claim and composed delivery inside a bracket region's
return-clause scrutinee is lowered with that region's clause H on the
eliminator stack. H is the bind's Ret case, the parent of S, which
runs the release and then the merge. Lowering refuses any such claim
that is not. S, the merge and resume keep their single path through
`settle_bracket_at_source`.

## Settled inputs (main `94b2d9273`; measured on B-closure r2 `8b5744bd2`)

- **The edge (D2).** Live region 0's response owner is row 3. Its Ret
  hands the raw `ResourceBodyOk MkUnit` to its caller, which continues
  into the resume, a Result match at origin 420, and traps 44. Null
  delivers `ResourceBodyErr MkUnit` to 979. No region-0 release site
  executes natively; the Buffer is released only at host finalize.
- **The cause (D4, D6).** Region 0's clause H is origin 427 on live
  (986 on null, 440 on the lawful px8f). In live and null, three claims
  whose head lies in H's scrutinee are lowered without H, in the very
  context that pushes H (live 1228, 1250, 1374 under `[606]`). px8f keeps
  H (1544 under `[619, 440]`). Which producer step reaches 606 with 427
  gone is unmeasured.
- **No planner plane separates them.** Record mentions (RC), lexical
  reach from the exit row's K (RX) and context placement (D6) all give
  the same answer for live and px8f. The property lives on the lowering
  stack.
- **The seats.** A claim meets a stack at `claim_and_call_continuation`
  (`lowering/core.rs:9307`) and a composed delivery at
  `lower_computational_match_value_composed_once` (`:4555`). The single
  settlement path is `settle_bracket_at_source`
  (`lowering/bracket_emission.rs:18`), through the ledger.
- **Evidence.** The scratch `/workspaces/ken/tmp/px10-t44-d0` is held
  as it is, with every overlay and log. The D0 artifacts are under
  `local/architect-shared/rc-ac0-d{1,3,4,6}`.
- **Kept.** W's capacity result, ANSWER-READER's causal-edge outcome
  (`5dfe23688`), and the r2 commits. The parked `5cd38e8b5` and r2
  `8b5744bd2` are not moved.
- **Current pins.** The landed bracket chain (`8a4190f94`) is pinned by
  `px8ta_oriented_subcontinuation`, `px7f_resource_native`,
  `px8f_buffer_native`, `rt_ignored_response_occurrence_native`,
  `rt_escape_second_resource_native`, `rt_parity_native`,
  `rt_span_prov_native`, `rt_branched_scrutinee_unit_body_port` and
  `abi_s6_mapping_file_backed_native`. `px8f_buffer_native` ran past
  900 s and 1650 s under D0 overlays; budget for it.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate on main, built in this order:

1. **The check, `BracketClauseOnStack`.** The planner's
   `BracketSettlementControlView` gives each region its `clause_origin`
   H and `clause_scrutinee` (the origins in H's child-0 subtree). It
   also gives each continuation context and unit body its
   `clauses_inside`. At both lowering seats, take the first
   Computational frame's origin as the head. For each region in the
   body's `clauses_inside`, if the head is in that region's scrutinee
   and H is not on the stack, refuse with `PlannerInvariant`
   `"BracketReturnClause"`, naming the region and clause.
2. **The fix.** Keep H on the eliminator stack at the producer step
   that loses it, as named by the refusal's backtrace.

Not built: a second settlement path, an owner-Ret clause, runtime tag
dispatch, or a change to the InPlace/Deferred split. B-closure then
rebases onto the candidate.

## Acceptance

- **AC-0, met.** The D0 chain located the edge at row 3's Ret (D2), and
  its cause at claims inside region 0's return-clause scrutinee that are
  lowered without the clause frame (D4, D6). No planner plane separates
  live from the lawful control (RC, RX, D6), so the check lands at
  lowering.
- **AC-1.** `BracketClauseOnStack` refuses at lowering, naming the
  region and clause. Its pin is a `px8-ds-test-support` mutation that
  drops H at the step the fix names. Deleting the check lets the
  mutation compile, and the pin reds.
- **AC-2.** The nine listed bracket pins stay green, with the check on.
- **AC-3, measured with B-closure.** On a scratch merge of the
  candidate and r2, live and null reach the interpreter's exit. D0FIN
  begin and end print the same trace_len, so there is no finalize-time
  release, and row 3's Ret no longer carries a `ResourceBodyResult`.
  Equal traces alone are not evidence, because the failing build already
  matches 17 of 17.
- **AC-4.** Full-workspace CI green on the exact candidate.

## Stop conditions

- A kernel or trust change.
- Step 1, the check alone on the scratch merge with r2, refuses any of
  the nine pins. That is the unmeasured population: stop to the
  Architect before any fix.
- The fix needs a new frame kind, a second S lowering or a change to
  the planner's routing records.
- argv's trap 53 after ReadEof surviving the fix is not this WP. It is
  PX10 entry 17, reserved.
- The `CapacityExhausted{Invocation, Words, 256, 269}` question stays
  deferred until AC-3 holds.

Checks: 5 fired: the property "the clause is on the stack" lives on the
lowering plane, and three planner planes measured cannot carry it, so
the check reads the stack at the two seats. 8 fired: the check refuses
live 1228 and null 1726, passes px8f 1544 with 440 on the stack, and
exempts px8f 1419, whose context does not contain 440. AC-3 requires
zero finalize-time releases, because the failing build already produces
equal traces. 6 fired: the measured violating population is three
claims each in live and null; the nine pins are unmeasured, so Step 1
measures them before the fix.
