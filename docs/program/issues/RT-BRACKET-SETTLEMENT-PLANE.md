---
id: RT-BRACKET-SETTLEMENT-PLANE
title: "CHILD 2b of RT-BRACKET-CONTROL-REGION-IR. The settlement plane and its CLOSURE. Consume ONLY the bound region markers from child 2a (RT-BRACKET-REGION-TRANSPORT); partition marked settlement instances out of the RAW exact response occurrence/context inventory BEFORE the ordinary demand classifier and pass only the ordinary partition into the existing filtered/Phase-B machinery; key every region instance by its exact call/return context beside its source ports; build the parent forest; close the body-exit set at exactly returned value, returned body error and controlled trap; and rebuild the whole plane INDEPENDENTLY at plan close and require exact equality. NO OLD CLASSIFIER PREDICATE MAY KEEP A BRACKET ROW ALIVE LONG ENOUGH TO EXCLUDE IT -- that was the `bracket_release_only_suffix` carve-out and it is gone. A first-match `find_map` keyed on `effect_origin` projects one ID for a shared static origin and collapses repeated lowering contexts; parent edges and the ledger must NEVER be built on a first-match ID. THIS CHILD DOES NOT LAND ALONE -- it is a held, reviewed input assembled by child 3."
status: ready
owner: runtime
size: L
gate: none
tier: T1
depends_on: [RT-BRACKET-REGION-TRANSPORT]
blocks: [RT-BRACKET-LOWERING-AND-D0-REFUTER]
github: null
origin: "Steward recut 2026-09-22 on the Architect's second WIP audit verdict evt_3rvns2yxm898r. Retained work to be REBASED ONTO THE ACCEPTED PRODUCER rather than rebuilt: the raw-response-occurrence partition ahead of the ordinary demand classifier, and the explicit response-context key that replaced the earlier first-match-only design. Both are preserved at evidence checkpoint 81f222b7f012829cd9f8d0f3dc684410a9b2b9ee, which is EVIDENCE, NEVER A CANDIDATE."
---

> # STARTABLE. Child 2a is accepted as a held input at `370168a48`.
> #
> # **This child does NOT land on its own.** It produces a reviewed checkpoint
> # that child 3 assembles. No QA handoff as a candidate, no publication, no
> # partial landing.

## 1. What is settled. Do not re-derive.

Re-sized on route C by the Architect's sizing D0 `evt_51cm672pdy1r1`,
against child 1 at `c462fe831`.

- **Build on child 2a's held tip `370168a48`**
  (`wp/RT-BRACKET-REGION-TRANSPORT`, over `c462fe831`; runtime QA
  `evt_7rvyzxzk7s17e`, Architect `evt_3drtxbpcrzv1c`), on this child's own
  branch. Never move, reset or delete that ref. Ports arrive from 2a's
  bound Runtime-IR region markers, `RuntimeExpr::CheckedBracketFinalizer`,
  keyed by (owner, checked preorder node id). **Amendment (Architect
  `evt_3jksek4xvf2wk`):** the runtime acquire is a second bound mark,
  delivered by 2a's AC-5 on `wp/RT-BRACKET-REGION-TRANSPORT-acquire`; build
  on that tip once it is reviewed. Every other port is positional from the
  two anchors (Architect `evt_719gp6pcxtycd`, replacing AcquireK,
  dispatch and Resume in `evt_3jksek4xvf2wk` and `evt_45cep98adh4qp`).
  Each step has one successor, so there is no predicate choice and no
  search:
  - **K.** Static route: the body of the `Let` whose value is the marked
    `Effect`. Generic route: `A.args[1]`, which must be a
    `LexicalClosure` with one parameter; K is its body.
  - **Spine walk.** From K, step into the body of each
    `CheckedSubcontinuationFrame` or `CheckedJoinSite`, and into the
    scrutinee of each `ComputationalMatch`, collecting those folds. D is
    the first other node. Measured depth: 1 fold top-level, 2 nested
    inner.
  - **Conditions.** Stop with the row unless all seven hold:
    1. D is a `Match` on `Var(0)` with exactly two cases, the acquire
       result family's Err and Ok by checked constructor identity.
    2. The folds are non-empty; F is the innermost.
    3. Resume is F's case whose constructor is the checked HostIO `Ret`.
    4. AcquireErr is D's Err body, a one-argument `Ret` `Construct`.
    5. The Body entry is D's Ok body.
    6. Settlement S is the Vis two levels above the Finalizer mark
       (mark at `S.arg0.arg0`), and the mark's path from K has the spine
       path plus D's Ok case as a prefix.
    7. OutcomeMerge is `S.arg1`, a `LexicalClosure` with one parameter.
  - **D0 before any build.** Run the walk on all 13 regions of the 2a
    census (9 cases). Per region report the route, the fold count,
    conditions 1 to 7, and for each nested pair whether the child's
    Resume lies in the parent's Body subtree. A failed condition is a
    stop with the row; do not widen the walk.
  - Body and exits are source-return graph sets per instance, and the
    parent is the region whose Body is the least superset.
  - This child never discovers a port, never selects by `HostOpV1`, and
    never re-derives authority.
- **Route C discharges:** port authenticity, the static acquire to Ok
  binder to finalizer pairing per region, and lexical nesting inside one
  declaration by path prefix.
- **Route C does not discharge, so this child delivers:**
  - instances: one static region under two call/return contexts mints
    two instances;
  - cross-declaration parents: a bracket body that calls a function
    containing a bracket is not lexical nesting;
  - the three closed body exits;
  - outcome-merge and resume ports, defined structurally from the
    finalizer Vis's own continuation (`\settled. Ret ...` in
    `release_if_live`, prelude.rs ~:2819), never by search.
- **Design reference, ported by hand, not rebased.** Evidence checkpoint
  `81f222b7f` is 641 commits behind main, and 21 of its 26 paths have
  changed since. Its minting (`mark_checked_resource_brackets`) selects by
  `HostOpV1::ResourceRelease` and is NOT reusable. Its downstream is:
  `BracketResponseContextKey`, `BracketSettlementRegion`,
  `bind_bracket_response_coordinates`, the Phase-A partition
  (responses.rs:2775) and `ResponseDisposition::BracketOwned` (:735).
  The plan is built before Phase A (construction.rs:1406).
- **Route D's child 1 (`4b4c8565c`) contributes nothing** and stays held.
  Never delete, reset or move `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY-route-C`. Cut this child's work on
  its own branch.
- **`bracket_release_only_suffix` is GONE and must not return in any form.**

## 2. Deliverable

**Edge and population model (Architect `evt_4yq7wzsteew9v`, operative;
the ruling's text is the model).** No new interprocedural graph and no
reuse of `continuation_result_positions`. Body is a static per-region set
of `StaticOriginId`s from the fixed forward walk E1 to E6 with terminals T1
to T4. An instance is a static region paired with one
`BracketResponseContextKey` of a response row on one of its S origins,
never a call site. The parent is the minimal containing Body. The
independent rebuild re-scans the marks and walks `RuntimeExpr` directly.
Its six-part D0 runs on all 13 regions before any §2 code. Each stop it
names is a stop. AC-3 needs a constructed control if no region has two
keys.

**E5 and the depth-sensitive terminals (Architect `evt_6pjd3avewr8mm`,
operative; replaces E5 and amends T1 and T3 of `evt_4yq7wzsteew9v`).**
Every `ComputationalMatch` in a Body must be a forwarding bind: exactly
two cases with roles `ret` and `vis`; the Vis case has
`argument_binders == 2` and `recursive_positions == [1]`; and its body,
under `CheckedComputationalIHSlots` and `CheckedSubcontinuationFrame` only,
is exactly `Construct(vis, [Var(1), IH OrdinaryApplication of a zero-arg
Var call])`. The ruling's `forwarding_vis_arm` is the predicate. Any other
match refuses as "fold is not a forwarding bind". Successors: the
scrutinee at depth + 1, the Ret case at the same depth. Body is every node
visited, scrutinee nodes included.
- `Construct(ret, ..)` at depth > 0 is R, legal; at depth 0 it is T3, "body
  bypasses settlement".
- A Vis with Q's own Finalizer mark at depth 0 is T1; at depth > 0 it
  refuses as "settlement inside a bind scrutinee".
- A Vis with another region C's Finalizer mark is X: cross it as in E6.
  Every crossed C has C.A and C.Resume in Body(Q), and every C with C.A in
  Body(Q) is crossed; otherwise "crossing regions".
- Measured 13/13 at c18 with T1 equal to the marker S set. Still owed in
  the same non-advancing D0 before §2 code: E4 restricted to `Reachable`
  emissions, the T1 outcome shape, the instance count per region, and
  minimality (Body(10417) ⊂ Body(1529) for nested-three 24890).

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
- **Body exits close at exactly three classes (Architect
  `evt_4hqrrtzpypd83`).** Returned value and returned body error are the
  two checked `ResourceBodyResult` constructors on the one T1 edge Body ->
  S; record each T1 as carrying both, and split them at OutcomeMerge. No
  exit class is read from the `Result` match on `settled`. Controlled trap
  is a declared class with zero edges: Runtime IR has no Trap -> S edge and
  §5 forbids a new return protocol, so a `Trap` in any Body walk refuses as
  "body trap has no settlement edge", with a constructed control. Trap
  settlement is not delivered by route C: child 3 also forbids a new return
  protocol and inherits this refusal. A later runtime node delivers it when
  a measured program needs a reachable Body trap to settle. Acquisition
  failure alone bypasses settlement.
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

**Admitted outside that cluster: the role-authority bridge (Architect
`evt_6560kbj1d6e88`), and nothing else.** Project the three
already-validated spine roles onto the existing `NativeProcessSymbols` as
`itree_ret`, `itree_vis` and `coproduct_in_l`:
- `native_process_entrypoint.rs`: the three fields, and their prelude
  spellings in `legacy_prelude()`, read from the seed fixtures.
- `native_process_authority.rs` `native_process_symbols_from_record`: fill
  them from `record.spine`, after confirming `record.roles()` already
  validates all three. If it does not, stop.
- `lowering/effects.rs` `runtime_producible_constructors`: bind them, do
  not add them to `producible`, and keep the destructure exhaustive.
- `ken-elaborator/src/compiler_driver.rs` (the struct literal): fill them
  from `host_spine`.

No new carrier, role or validation change. The plane reads the three by
exact equality; no suffix matching. Controls: the producible set is equal
before and after on a package-backed and a synthetic fixture; swapping
`itree_ret` and `itree_vis` in a test-built authority makes the plane
refuse on single-resource; and the struct-literal and destructure sweep is
re-run across `crates/` at the base.

**Also admitted: two refusing `BracketOwned` arms (Architect
`evt_7sk4q84kqc3xx`), and nothing else in lowering.** At the two total
matches over `ResponseDisposition`, in `lowering/core.rs`
(`response_disposition_at_operation_root`) and `lowering/effects.rs`
(`response_disposition_at_effect`), add one arm
`Some(ResponseDisposition::BracketOwned(_))` that returns a
`LoweringCause::PlannerInvariant` refusal, with the ruling's text. No
Deferred fall-through and no wildcard; the existing three-way arm stays
byte-identical. Child 3 replaces these arms with real lowering. Owed:
`-p ken-runtime --lib` checks clean, and each lowering file's diff is
exactly the one arm.

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
