---
id: RT-BRACKET-CONTROL-REGION-IR
title: "Make bracket-settlement ownership and order EXPLICIT IN THE CONTROL IR. OPERATOR CHOSE THIS REPRESENTATION 2026-09-22 ('concur with rec. should be in the IR') after RT-DELAYED-OWNER-CLASSIFICATION-TWO-PASS was parked at hard stop 7; Architect ruled the shape at evt_4y0w2788qfxk1. THE SEMANTIC UNIT IS A BRACKET CONTROL REGION -- not a response, operation, call seat, group or fixture. One region owns one acquisition-success lifetime from delayed-body entry through settlement-result merge, and NESTED REGIONS' CONTROL EDGES ESTABLISH INNER-BEFORE-OUTER; response-owner liveness does not and has now twice proved to be only an indirect correlate of settlement order. A compiler-private checked wrapper marks the bracket at erasure, a private settlement plane resolves markers into regions on StaticTransitionPlan before response Phase B assigns any owner, a closed ResponseDisposition makes BracketOwned a PROJECTION of that plane rather than a predicate, and lowering consumes only a validated control view. D0 IS AN EQUIVALENCE REFUTER ON THE SMALLEST END-TO-END VERTICAL SLICE AND IT RUNS BEFORE THE EMITTER IS GENERALIZED: it compares interpreter and linked-native observation tuples and CAN REFUTE THE REPRESENTATION, which population equality provably cannot. This node begins at ZERO hard stops; the parked chain keeps 7/7."
status: ready
owner: runtime
size: L
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-22, on current main 03e9b0afdf4c055479c0b70130b48d264bf58b93. Operator closed the representation fork in conversation ('concur with rec. should be in the IR') after the Steward put it to them at evt_47n69ztgsp6b8; the Architect's terminal ruling evt_4zk1ckv4czk4e required that any future attempt be a separately framed architecture decision on current main with the representation chosen by the operator. Shape ruled at evt_4y0w2788qfxk1, which confirmed the Steward's bounded-scope reading and adopted the Steward's request that the equivalence refuter come first. THIS IS NOT AN AMENDMENT to RT-DELAYED-OWNER-CLASSIFICATION-TWO-PASS and does not inherit its scope fence; that node stays parked at draft and its evidence stays evidence."
---

> # AUTHORIZED AND STARTABLE. Build it.
> #
> # **Start from landed current `main`.** This is a NEW node, not an amendment
> # to the parked one, and it does not inherit that node's scope fence.
> #
> # **`D0` runs FIRST and can stop this node.** It is not a census. It is an
> # end-to-end equivalence observation on the smallest vertical slice, and it is
> # the only instrument here that can refute the representation.
> #
> # **Do not answer a `D0` failure with a larger census.** That is the exact
> # move that cost seven hard stops on the parked node.

## 1. What is settled. Do not re-derive any of it.

- **The parked node's commit law is FALSE and is not to be revisited.**
  `P == A intersect L` plus strict selected-caller coverage committed clean
  artifacts on all four two-operation Mapping programs and every one trapped at
  native terminal `-1`. Exact call-seat presence plus graph coverage proves the
  selected edge exists and the compiler graph is closed, and proves NOTHING
  about observational equivalence with the Deferred route.
- **`P = A intersect L`, selected-caller coverage and call liveness authorize
  nothing in this node.** They are not inputs to bracket ownership.
- **`N` remains valid evidence** from `64fd9e6abf63072b3222124b6df4c546b7a1d243`
  and serves here as a NON-BRACKET REGRESSION CONTROL. It is not
  bracket-ownership input.
- **The counterexample** `9d6a9547f09b7893cbbe1c8c2bcefd6eaefec14c` is why the
  first four Mapping rows are MANDATORY in `D0`'s matrix rather than optional.
- **The row this exists to clear** is
  `px8ta_oriented_subcontinuation.rs::public_two_three_level_brackets_finish_and_release_lifo`,
  which carries TWO blockers under one `#[ignore]`: depth 2 executes and
  releases in ACQUISITION order rather than strict LIFO, and depth 3 refuses at
  object emission on `ContinuationSpecialization`. Either fix alone leaves it
  red.

## 2. `D0` -- THE EQUIVALENCE REFUTER. IT RUNS FIRST AND CAN STOP THIS NODE.

Build the SMALLEST END-TO-END VERTICAL SLICE that carries one bracket through
marker, plane, ownership and lowering. Do not generalize the emitter first.
Pre-register the observation tuple BEFORE running it:

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

**Mandatory first matrix. All five, not a subset:**

1. px8ta depths 2 and 3 -- strict LIFO and no emission refusal;
2. all four two-operation Mapping programs -- parity and no terminal `-1`;
3. both alternating Mapping programs -- parity remains green;
4. O0/read-offset and W0/read-window -- O0 remains live, W0 preserves main;
5. one acquisition-failure case and one body-error/release-error discriminator
   -- these observe the OUTCOME MERGE, not merely order.

**Every equivalence pin states, in these words:**

```text
MEASURED: exact normalized interpreter/native observation tuple.
CLAIMED:  explicit bracket control preserves result propagation and effect order.
THE GAP:  closed only for the exercised path plus region/edge closure, never by
          call coverage.
```

## 3. `D1` -- producer representation

One compiler-private checked Runtime-IR wrapper around the existing bracket
expression. These names state semantic shape, NOT required spelling:

```rust
CheckedResourceBracket {
    control: CheckedResourceBracketControl {
        template: BracketTemplateId,
        acquire: BracketAcquirePort,
        acquired_body: BracketBodyPort,
        settlement: BracketSettlementPort,
        outcome_merge: BracketOutcomeMergePort,
        resume: BracketResumePort,
    },
    body: Box<RuntimeExpr>,
}
```

Each port is a DISTINCT NEWTYPE over an exact checked occurrence coordinate,
never an interchangeable integer or string. The wrapper is METADATA: not a
runtime value, not public Ken syntax, not an effect, not an ABI field.
Reference evaluation unwraps `body` unchanged.

Checked erasure/prelude production mints it from canonical checked declaration
identities for `withResource`, `withBuffer` and `withMapping`. **Cranelift must
not discover a region** from names, `HostOpV1`, expression resemblance, or a
search for `ResourceRelease`. The marker identifies the bracket; planning
validates that its settlement port is the exact release consuming the resource
produced at its acquire port. Public early `release` stays ordinary body work
because it owns no settlement port.

**A template is not an execution instance.** Distinct exact call/return contexts
mint distinct opaque `BracketSettlementId`s, with no conversion to origin,
continuation, response-owner, function-owner or operation identities.

## 4. `D2` -- planner control plane

Resolve markers into a private `BracketSettlementPlan` on
`StaticTransitionPlan`, one `BracketSettlementRegion` per instance:

```text
id; parent; acquire; acquire-failure resume; body; closed body exits;
settlement; outcome merge; resume
```

The body-exit set is EXACTLY returned value, returned body error, and controlled
trap. All three enter the same settlement node. Acquisition failure alone
bypasses settlement. Settlement completion enters the EXISTING outcome merge
before resume, preserving success, body error, release error, combined
body-plus-release error, and trap-primary cleanup behavior.

```text
AcquireOk -> Body -> BodyExit -> Settlement -> OutcomeMerge -> Resume
AcquireErr -----------------------------------------------> Resume
```

`parent` is the unique nearest enclosing region in the exact source-return
control graph -- never origin proximity, never numeric depth. For child `C` of
parent `P`, `C.Resume` lies inside `P.Body`, so the graph contains
`C.Settlement -> C.Resume -> P.BodyExit -> P.Settlement`. Crossing regions,
cycles, two parents, or a child resume outside the parent body all REFUSE.

Build this plane AFTER exact call/return contexts exist and BEFORE response
Phase B assigns an owner. At plan close, rebuild it INDEPENDENTLY from checked
markers plus the final source-return graph and require exact equality.

## 5. `D3` -- ownership reconciliation

The region owns its settlement, through a closed disposition:

```rust
enum ResponseDisposition {
    BracketOwned(BracketSettlementId),
    Specialized,
    Deferred,
}
```

`BracketOwned` is PROJECTED from the control plane, never derived by another
predicate in `static_response_phase_b_split`. Exclude bracket-owned responses
from `StaticResponseOwnerSpecialization`, from bounded Deferred-handler
selection, and from `A/N/P/L` entirely. Refuse dual ownership, an unowned marked
settlement, and one region owning two settlement effects.

## 6. `D4` -- lowering consumer

Lowering receives ONLY a validated `BracketSettlementControlView`. Body
completion consumes its settlement edge, lowers the existing release through
existing host-effect machinery, consumes outcome merge, then consumes resume.
Inline versus an existing private control unit is secondary.

Close a compiler ledger over exact region and edge identities: every marked
region emitted once, every mandatory edge consumed once, no unplanned edge
consumed, every settlement reached through its owner. **These equalities prove
REPRESENTATION CLOSURE, not semantic equivalence.** Only `D0` does the latter.

## 7. `D5` -- closeout

Un-ignore
`px8ta_oriented_subcontinuation.rs::public_two_three_level_brackets_finish_and_release_lifo`
and remove an order-excluding helper ONLY after both engines agree.

## 8. Acceptance criteria

- **AC-1.** `D0`'s five-case matrix passes on the vertical slice BEFORE the
  emitter is generalized, with the pre-registered tuple and the MEASURED /
  CLAIMED / THE GAP pin recorded for each case.
- **AC-2.** Resource identities are normalized by ACQUISITION ORDINAL only. A
  candidate comparing sets, or comparing release vectors alone, fails this AC
  even if it is green.
- **AC-3.** All three `D0` mutations behave as section 9 requires, each with an
  unmutated positive sibling and an executed-count assertion that is POSITIVE.
- **AC-4.** The marker is metadata only: no runtime value, no public Ken syntax,
  no effect, no ABI field. Reference evaluation unwraps `body` unchanged, shown
  by a control that would fail if it did not.
- **AC-5.** Cranelift discovers no region from names, `HostOpV1`, expression
  resemblance, or a `ResourceRelease` search. Demonstrated by a program whose
  shape resembles a bracket but carries no marker, and which gets no region.
- **AC-6.** The plane is built after exact call/return contexts and before
  response Phase B, and the independent rebuild at plan close is EXACTLY equal.
  The rebuild is a second derivation, not a re-read of the first.
- **AC-7.** `BracketOwned` is a projection. A control shows that no predicate in
  `static_response_phase_b_split` can produce it.
- **AC-8.** Bracket-owned responses appear in NONE of
  `StaticResponseOwnerSpecialization`, bounded Deferred-handler selection, or
  `A/N/P/L`.
- **AC-9.** Dual ownership, an unowned marked settlement, and one region owning
  two settlement effects each reach an exact named refusal.
- **AC-10.** The compiler ledger closes over exact region and edge identities,
  and the frame text recording it says outright that it proves representation
  closure and NOT semantic equivalence.
- **AC-11.** `N` is used only as a non-bracket regression control and is nowhere
  an input to bracket ownership.
- **AC-12.** Unrelated response specialization is byte-for-behavior unchanged.
- **AC-13.** The px8ta row is un-ignored and green on BOTH blockers -- strict
  LIFO at depth 2 AND no emission refusal at depth 3.
- **AC-14.** `D0` was pre-registered before it was run, evidenced by the
  registration landing ahead of the measurement rather than beside it.

## 9. Required controls and mutations

**Three compile-preserving mutations, all mandatory:**

1. Reverse one REAL child/parent settlement edge without changing its endpoints.
   Ordered observation must RED and the application count must be POSITIVE.
2. Bypass one REAL outcome-merge edge while preserving settlement emission. The
   result/outcome comparison must RED INDEPENDENTLY OF ORDER.
3. Assign one settlement both `BracketOwned` and `Specialized`. Plan closure
   must reach the exact dual-owner refusal.

Each negative carries an unmutated positive sibling and an executed-count
assertion. A mutation that stays green is a STOP, not a finding to explain.

## 10. Stop condition

**STOP IMMEDIATELY, and the deliverable is then the stop and its evidence, if:**

- a marked region is ambiguous;
- any mandatory `D0` observation differs between the engines;
- either behavioral mutation stays green;
- the existing typed result path cannot carry the sequence.

On the last one, **do not reopen `RT-PLANNER-KRET-GRAFTED-SPINE`** and do not
invent a return protocol inside it.

**Do not answer a stop with a larger census, a narrowed matrix, a carve-out, a
pending obligation, or a fallback.** On any stop branch, restructuring comes to
the Steward and then the operator. It is not the ring's to route and not the
implementer's to improvise.

## 11. Scope

Checked erasure/prelude production for the marker; the settlement plane on
`StaticTransitionPlan`; `ResponseDisposition` and its reconciliation in
`static_response_phase_b_split`; the lowering consumer; and the controls above.

**NO new carrier, return protocol, KRET lane, host operation, dispatcher
reorder, trace sort, or fallback is authorized.**

## 12. What must not happen

- No standalone marker, inert plane, or partial owner as a deliverable. **ONE
  VERTICAL PRODUCT LANDS.**
- No reopening of the parked node and no inheritance of its scope fence.
- No use of call presence, coverage or liveness as evidence of semantic
  authority. That is the defect this node exists to remove.
