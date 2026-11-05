---
id: RT-NATIVE-BRACKET-RETURN-CLAUSE
title: "Native bracket settlement is placed on the response-row plane, InPlace when a response owner exists and Deferred otherwise, while the body's control transfer runs on the runtime K. So a body outcome can reach the bracket's resume K with no release and no outcome_merge (PX10 B-closure stop 12, live origin 420, null 979). Give the bracket's frame one return clause that every body exit returns into, and refuse at planning any edge that carries a body outcome past the bracket's delimiter"
status: active
owner: runtime
size: L
tier: T1
gate: architect
depends_on: []
blocks: [PX10-NATIVE-SPAWN-PARITY]
github: null
origin: "Architect recut evt_zbd2e5wv721d (PX10 B-closure stop 12, §1b entry 16 located, sharing entries 10-13's predicate), on Research advisory evt_2v4hdh6wpzk5s. L1 objective: the Linux ABI program. Steward-filed per COORDINATION section 2."
---

# A bracket settles through its frame's return clause

## Objective

Every exit of a native bracket body, Ok or Err, returns into the
bracket frame's own continuation. That continuation performs the
release, applies `outcome_merge`, then calls the carried resume K.
There is one settlement path and no mode split on the response-row
plane. Planning refuses any transport edge that delivers a body-outcome
word (`ResourceBodyOk` or `ResourceBodyErr`) to a K past the bracket's
delimiter.

## Settled inputs (main `1e49f870c`; measured on B-closure r2 `8b5744bd2`)

- **The mode split.** `BracketSettlementControlView`
  (`planning/static_transition/brackets.rs:160`) carries a
  `BracketSettlementMode` of `InPlace` or `Deferred` (`:166`). The mode
  is `InPlace` exactly when a returned-exit response owner exists
  (`:221` to `:225`).
- **The placement.** `settle_bracket_at_source`
  (`lowering/bracket_emission.rs:18`) settles through
  `perform_response_at_source` (`lowering/core.rs:6240`) at the owner
  row's source. The relay and successor resumes of §1b entries 10 to 13
  are also placed on response rows.
- **The failure.** On live, region 0 is `InPlace` at response row 3's
  source (D0BRK id=0). No runtime body exit passes through that row. The
  raw `ResourceBodyOk MkUnit` reaches region 0's resume, a Result match
  at origin 420 in funcid53, and traps 44. Null delivers
  `ResourceBodyErr MkUnit` to origin 979. Region 0's Buffer release runs
  only at host finalize: D0FIN begin shows trace_len 16, and seq 16
  falls between the markers. No region-0 release site executed natively.
- **Unmeasured.** The path that carries the body outcome to 420. Whether
  live seq 14 and 15 are native releases: D0HOSTEV prints a consumed
  settlement the same as a native release op.
- **Evidence.** The scratch `/workspaces/ken/tmp/px10-t44-d0` is held
  as it is, with every overlay and log. It is AC-0's base.
- **Kept.** W's capacity result, ANSWER-READER's causal-edge outcome
  (`5dfe23688`), and the r2 commits outside settlement placement. The
  parked `5cd38e8b5` and r2 `8b5744bd2` are not moved.
- **Current pins.** The landed bracket chain (`8a4190f94`) is pinned by
  `px8ta_oriented_subcontinuation`, `px7f_resource_native`,
  `px8f_buffer_native`, `rt_ignored_response_occurrence_native`,
  `rt_escape_second_resource_native`, `rt_parity_native`,
  `rt_span_prov_native`, `rt_branched_scrutinee_unit_body_port` and
  `abi_s6_mapping_file_backed_native`. Each stays green.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate on main:

- The answer-type check on transport edges.
- The bracket frame's return clause, replacing the InPlace/Deferred
  split, `settle_bracket_at_source`'s response-row route and the
  response-row placement of relay and successor resumes.

B-closure then rebases onto it.

## Acceptance

- **AC-0 (D0).** The Architect specifies the D0 on the held scratch.
  The runtime implementer runs it and reports. The Architect rules from
  the report and does not build or test. The D0 adds the answer-type
  check alone over r2, and the check refuses the live fixture at
  planning. The refusal names the edge that carries region 0's body
  outcome to 420, which both locates the unmeasured path and proves the
  check bites. The Architect rules:
  - the return clause's form;
  - whether the check lands as its own increment or with the clause;
  - the increment cut, sized to one implementer turn each.
- **AC-1, refusal pin.** A planning test builds a bracket whose body
  outcome is transported past its delimiter, and it is refused with the
  check's typed cause. Removing the check makes it plan.
- **AC-2, one path.** Every listed bracket pin is green. A pin of the
  planner's settlement plan shows no response-row mode for any region.
  Restoring the mode split reddens it.
- **AC-3, measured with B-closure.** On a scratch merge of the
  candidate and r2, live and null reach the interpreter's exit. Their
  traces match the interpreter's with no finalize-time release: D0FIN
  begin and end print the same trace_len. Equal traces alone are not
  evidence, because the failing build already matches 17 of 17.
- **AC-4.** Full-workspace CI green on the exact candidate.

## Stop conditions

- A kernel or trust change.
- A bracket pin that the return clause cannot keep without a second
  settlement path.
- argv's trap 53 after ReadEof surviving the clause is not this WP. It
  is a new PX10 inventory entry.
- The `CapacityExhausted{Invocation, Words, 256, 269}` question stays
  deferred until AC-3 holds.

Checks: 5 fired: the mode is keyed on whether a response owner exists,
read at `brackets.rs:221`, a response-row property. The transfer that
skips settlement runs on the K. The clause has no mode to key. 8 fired:
AC-3 requires zero finalize-time releases, because the failing build
already produces equal traces. AC-1's refusal is a cause the unchecked
planner cannot raise. 3 fired: the bracket pins were found by grepping
`crates/*/tests` for the settlement mode and bracket fixtures on
`1e49f870c`.
