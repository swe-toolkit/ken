---
id: RT-BRACKET-SETTLEMENT-PLANE
title: "CHILD 2 of 3 of RT-BRACKET-CONTROL-REGION-IR. The settlement plane and its CLOSURE. Consume ONLY authenticated ports from child 1; partition marked settlement instances out of the RAW exact response occurrence/context inventory BEFORE the ordinary demand classifier and pass only the ordinary partition into the existing filtered/Phase-B machinery; key every region instance by its exact call/return context beside its source ports; build the parent forest; close the body-exit set at exactly returned value, returned body error and controlled trap; and rebuild the whole plane INDEPENDENTLY at plan close and require exact equality. NO OLD CLASSIFIER PREDICATE MAY KEEP A BRACKET ROW ALIVE LONG ENOUGH TO EXCLUDE IT -- that was the `bracket_release_only_suffix` carve-out and it is gone. A first-match `find_map` keyed on `effect_origin` projects one ID for a shared static origin and collapses repeated lowering contexts; parent edges and the ledger must NEVER be built on a first-match ID. THIS CHILD DOES NOT LAND ALONE -- it is a held, reviewed input assembled by child 3."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: [RT-BRACKET-PRODUCER-AUTHENTICITY]
blocks: [RT-BRACKET-LOWERING-AND-D0-REFUTER]
github: null
origin: "Steward recut 2026-09-22 on the Architect's second WIP audit verdict evt_3rvns2yxm898r. Retained work to be REBASED ONTO THE ACCEPTED PRODUCER rather than rebuilt: the raw-response-occurrence partition ahead of the ordinary demand classifier, and the explicit response-context key that replaced the earlier first-match-only design. Both are preserved at evidence checkpoint 81f222b7f012829cd9f8d0f3dc684410a9b2b9ee, which is EVIDENCE, NEVER A CANDIDATE."
---

> # AUTHORIZED AND STARTABLE. Build it.
> #
> # **This child does NOT land on its own.** It produces a reviewed checkpoint
> # that child 3 assembles. No QA handoff as a candidate, no publication, no
> # partial landing.

## 1. What is settled. Do not re-derive.

- **Retained, rebase rather than rebuild:** the raw-inventory partition ahead of
  the ordinary demand classifier, and the explicit response-context key.
- **`bracket_release_only_suffix` is GONE and must not return in any form.** It
  was an operation-based carve-out inside the old response-owner route.
- Ports arrive ALREADY AUTHENTICATED from child 1. This child never discovers a
  port and never re-derives authority.
- **CHILD 1 IS ACCEPTED. Build on `4b4c8565c543c148039ea06ac798c7c91eda2ce8`**
  -- QA approved at that tip in `evt_4z54qy2pvmw0f`, EIGHT commits from
  `a4d12698a` (measured). It delivers injected post-elaboration marker
  identities NOT resolvable from source, exact canonical call-occurrence
  capture, and producer-marked acquire / body / settlement / outcome-merge /
  resume roles. `HostOpV1` may VALIDATE a marked port and may NEVER discover
  one; that rule is INHERITED, not re-litigated here.
- **DO NOT DELETE, RESET OR FORCE-MOVE `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.**
  It is the ONLY ref holding child 1's accepted tip, it was never pushed, and
  child 3 assembles from it. Releasing a branch means dropping the holder, not
  removing the name. Cut child 2's work on its own branch.

## 2. Deliverable

A private `BracketSettlementPlan` on `StaticTransitionPlan`, one
`BracketSettlementRegion` per INSTANCE:

```text
id; parent; acquire; acquire-failure resume; body; closed body exits;
settlement; outcome merge; resume
```

The typed edge relation:

```text
AcquireOk -> Body -> BodyExit -> Settlement -> OutcomeMerge -> Resume
AcquireErr -----------------------------------------------> Resume
```

- **Partition FIRST.** Derive the raw exact response occurrence/context
  inventory, partition marked settlement instances out of it BEFORE
  `static_response_context_demands_filtered`, bind the control plane from the
  marked partition, and pass ONLY the ordinary partition onward.
- **Instance identity.** Each region carries its exact call/return context key
  beside its source ports, and EVERY disposition and control-view query uses
  that key.
- **`parent`** is the unique nearest enclosing region in the exact source-return
  control graph -- never origin proximity, never numeric depth. For child `C` of
  parent `P`, `C.Resume` lies inside `P.Body`, so the graph contains
  `C.Settlement -> C.Resume -> P.BodyExit -> P.Settlement`.
- **Body exits close at exactly three:** returned value, returned body error,
  controlled trap. All three enter the same settlement node. Acquisition failure
  alone bypasses settlement.
- **Independent rebuild at plan close** from checked markers plus the final
  source-return graph, required EXACTLY equal. It is a second derivation, not a
  re-read of the first.
- **`ResponseDisposition`** closed over `BracketOwned(BracketSettlementId)`,
  `Specialized`, `Deferred`, with `BracketOwned` PROJECTED from the plane.

## 3. Acceptance criteria

**Every AC below states what a control OBSERVES, so RUN it before handing it to
QA. An observation you have predicted but not run is written as OWED, not as
fact.** Child 1 cost two frame corrections to exactly that.

- **AC-1.** Bracket-owned rows appear in NONE of
  `StaticResponseOwnerSpecialization`, bounded Deferred-handler selection, or
  `A/N/P/L`. Shown by measurement of those populations, not by inspection.
- **AC-2.** No predicate in `static_response_phase_b_split` can produce
  `BracketOwned`. It is a projection.
- **AC-3.** Two distinct call/return contexts over one shared static origin mint
  DISTINCT instances. This is the first-match defect and it needs its own
  control.
- **AC-4.** The independent rebuild at plan close is exactly equal, and is
  demonstrably a second derivation.
- **AC-5.** Each of these reaches an exact named refusal: dual ownership; an
  unowned marked settlement; one region owning two settlement effects; crossing
  regions; a cycle; two parents; a child resume outside the parent body.
- **AC-6.** The plane is built after exact call/return contexts exist and BEFORE
  response Phase B assigns any owner.
- **AC-7.** Unrelated response specialization is byte-for-behavior unchanged.

## 4. Stop condition

**STOP if** a marked region is ambiguous, if the independent rebuild is not
exactly equal, or if excluding bracket rows from the ordinary machinery requires
ANY predicate in the old classifier. That last one is the defect returning under
a new name.

## 5. Scope

`StaticTransitionPlan` and the `static_transition` cluster, including
`responses.rs` for the reconciliation. **No new carrier, return protocol, KRET
lane, host operation, dispatcher reorder, trace sort or fallback.**

**A CROSS-CRATE exhaustiveness sweep is owed ONLY IF THE TYPE IS VISIBLE
OUTSIDE `ken-runtime`. CHECK VISIBILITY FIRST -- do not inherit the answer.**
Measured at `dda3ff6d7`: `ResponseDisposition` and `StaticTransitionPlan` are
`pub(in crate::cranelift_backend)` with no public re-export and are referenced
in `ken-runtime` ONLY, so `-p ken-runtime` IS the complete type-check gate for
this variant. Positive control, same grep and tree: child 1's `RuntimeExpr` is
`pub` and reaches `ken-cli`, `ken-elaborator`, `ken-interp` and `ken-runtime`,
which is why ITS sweep found four sites in two crates.

WHAT STILL APPLIES HERE: the WITHIN-crate sweep by GREP before building (20
`ResponseDisposition` references), and RE-CHECKING the visibility conclusion at
the moment the variant lands. If `BracketSettlementId` or any new type reaches
a public surface, the cross-crate sweep is owed again ON THAT TYPE and today's
answer does not carry. Never `--workspace`.
