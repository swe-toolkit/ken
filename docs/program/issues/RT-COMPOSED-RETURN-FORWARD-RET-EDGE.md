---
id: RT-COMPOSED-RETURN-FORWARD-RET-EDGE
title: "Close the composed-return Tail wall by shape (a): a governed Tail result takes ONE certified forward SSA edge from the existing producer to the existing shared Ret block, bypassing the source-machine answer collapse, constructor transfer, active carried backedge, and checked fallback. The single relaxed constraint is the lossy middle; everything else (spec, kernel, ABI, runtime state) is unchanged. Flips the base-red Tail `ResourceBodyResult` `PatternMatchFailure` rows to exact `InvalidOffset` on the two `SourceFormat::Ken` witnesses."
status: active
owner: runtime
size: L
gate: none
tier: T1
depends_on: []
blocks: [PX8]
github: null
origin: "Operator ruling 2026-08-30 (this session, verbatim: \"use shape (a)\") FUNDED the shape-(a) relaxation after the constraint-differential report (docs/program/rt-composed-return-constraint-differential-report.md, merged 5b20fe84f) found BOTH native walls INCIDENTAL, not spec-mandated. The build design is the fresh operator+Architect decision the report fed; Architect ruling evt_70n2y6s9wanf9 (base origin/main 7d807a78e, tree 37da2a975) mints this fresh T1 node and closes the held authority-only build RT-COMPOSED-RETURN-PRODUCER-ORDER-BUILD (refuted/superseded without implementation). Steward-filed and released per COORDINATION section 2."
---

> # ACTIVE — lane 1, the funded shape-(a) build. D1 LANDED; D2 RELEASED. `active`.
>
> The operator funded shape (a); the Architect designed it (`evt_70n2y6s9wanf9`)
> and minted this fresh node in place of the refuted authority-only build. This is
> a T1 soundness-bearing native-lowering build; the runtime ring's standing
> hard-stop protocol applies. A design fork HARD-STOPS to the Architect.
>
> **D1 (sink seam, byte-inert) MERGED** — squash `f193b074e` (2026-08-30),
> candidate `dd0b824272`, Decision `dec_6gctzq9sann54`, gates Runtime QA
> `evt_5synxy6g2q9yy` + Architect `evt_5svnf4fdqdes5`/`evt_70h7f8nyv3c8h`, all four
> blobs Steward+lieutenant+Adversary verified, Adversary M8 NO OBJECTION
> (`evt_5865012wcz8qa`: the seam is a deliberately-inert validation gate, byte-inert
> differential proves identical executable bytes). Accepted PARTIAL — node stays
> `active`. **D2 (authority/plan, byte-inert) RELEASED** off `f193b074e` — the exact
> post-selection confluence/member join plus the replacement Tail plan, with
> wrong-member/projection/source/sink controls, no live-edge claim. **D3 (atomic
> activation) STAYS HELD** and needs its own separate explicit release after D2
> lands; neither D1 nor D2 landing authorizes it.

## The single relaxed constraint, and what stays closed

The ONE relaxed constraint: a governed Tail result BYPASSES the source-machine
answer collapse, the constructor transfer, the active carried backedge, and the
checked fallback — it takes one certified forward SSA edge from the existing
producer to the existing shared `Ret` block. This is the "lossy middle" the
constraint-differential report identified as incidental (comparators keep the
operation/continuation association live and move the response FORWARD; Ken alone
collapses the answer and reconstructs a seed word backward).

Everything else stays closed. **Produced-transfer, D3, Direct-only salvage,
recovery, storage, runtime tags, general carriers, active-header ABI widening, a
second `Ret` lowering, and HS15 remain closed.** No spec, kernel, trust, or wire
change. The forward-edge block capability is COMPILER STATE, not a runtime lane —
it never enters a Ken value, carrier, frame, ABI, or memory.

## Exact base and coordinates (Architect `evt_70n2y6s9wanf9`)

Base: `origin/main` `7d807a78e014f768311b53380c77fa1daf4c6e06`, tree
`37da2a97565b0e968ed1927ea44906be3c886597` (the Architect's cited tree; a
disjoint Steward doc publish moved the commit label off `8455964281`, the tree is
identical). Coordinates decay — re-measure before acting.

- `crates/ken-runtime/src/cranelift_backend/lowering/source.rs` blob
  `88fcc401b0e078f78298a0998d09364b22e64a27`: governed validation `3976-4117`;
  transport selection `4309`; call/result `4369-4371`; current collapse entry at
  `4373`. The lossy middle is `1260-1264`, `1647`, and `1691-1710`.
- `crates/ken-runtime/src/cranelift_backend/lowering/core.rs` blob
  `79ec94b749836a6e1747d6b6da0b572f919105cd`: active loop/header `12225-12282`;
  shared `Ret` block creation `12364-12368`; ordinary `Ret` jump `12433-12445`;
  stale checked-fallback jump `12634-12743`.
- `crates/ken-runtime/src/cranelift_backend/planning/static_transition/aggregates.rs`
  blob `9eb2c118e227c3a7db2849e03046db02d93a48eb`: fresh route `5461-5582`; Tail
  validation `5751-5855`; member/projection row `5902-5977`; confluence
  `6123-6257`; access publication `6323-6543`; validation `6624-6684`.

## Required component shape (Architect `evt_70n2y6s9wanf9`)

1. **Replace the planner's false Tail emission description.** `TailResumedRetInput`
   currently claims `ActiveSelfResumption` plus `CheckedAnswerFallbackDirect` — the
   path that loses the result. REPLACE it, do not add a sibling, with a Tail
   producer-to-Ret plan naming the governed source/binding, the selected case, the
   active frame, the exact `Ret` body, the exact
   `ConstructorChild { frame_origin, field_position: 0 }`, forward direction, and
   producer-result-direct delivery. `DirectInvocationReturn` stays.
2. **Extend the existing compiler-only `active_carried_computational_eliminations`
   stack entry.** When `core.rs:12364` creates the shared `Ret` block, install
   `{active_frame_origin, ret_case_body_origin, binder field 0, return_body block}`
   in that exact entry. Missing, duplicate, or wrong-frame/body/binder lookup
   refuses. This block handle never enters a Ken value, carrier, frame, ABI, or
   memory.
3. **Reuse the retained confluence authority.** Governed validation retains its
   projection. After selecting the transport and BEFORE emitting at `:4369`, an
   exact lookup requires the access coordinate, projection equality, and
   membership of that transport's own `source_call_identity`; then the Tail plan
   must match the unique emission sink. This yields ONE move-only compiler proof,
   not a copied member map or positional choice.
4. **Under that proof, emit the declared call once.** Direct preserves the old
   return. Tail requires the returned carried word and immediately emits
   `jump(return_body, [returned.word])`, switches to an unreachable builder block,
   and returns a sealed `RecursiveBackedge` disposition so no remaining source
   continuations emit on that predecessor. No second `Ret` body.
5. **The later fallback may serve other honest populations, but a governed Tail
   producer-to-Ret plan neither claims nor reaches it.**

## Deliverables — one-hour turns (Architect `evt_70n2y6s9wanf9`)

- **D1 — sink seam, byte-inert.** Extend the active-stack record, install the
  unique strict `Ret` sink, add population/uniqueness controls. No consumer or
  product claim.
- **D2 — authority/plan, byte-inert.** Exact post-selection confluence/member
  join plus the replacement Tail plan, and wrong-member/projection/source/sink
  controls. No live-edge claim.
- **D3 — atomic activation.** Consume the proof at `source.rs:4369-4373`, emit the
  returned-word jump, retire the old Tail header/fallback observer claims, add the
  two durable products, and run affected closure. D3 code, causal controls, and
  products land together. D1/D2 may land alone ONLY with byte-inert evidence and
  honest prose.

## Acceptance criteria (Architect `evt_70n2y6s9wanf9`)

- **AC-PLAN-EXACT** — one governed source and one `Ret` sink. Removal,
  duplication, reversal, cross-variant, and wrong-body/binder/frame mutations RED
  before emission. The old Tail header/fallback fields are ABSENT (replaced, not
  siblinged).
- **AC-ONE-AUTHORITY** — the edge proof forms ONLY after exact transport selection
  and confluence-member validation. Wrong member/transport, projection
  disagreement, missing class, or a proof bypass RED. No second catalog.
- **AC-FORWARD-SSA** — the exact call result is the SOLE governed predecessor
  argument to the shared `Ret` block. Ordinary `Ret` is unchanged. A governed Tail
  does not enter `RoutedAnswer::checked`, `ConstructArgument`, active
  self-resumption, or the checked fallback.
- **AC-CAUSAL-PAIR** — with call emission retained, route its result through the
  old collapse: both products revert to base `ResourceBodyResult`
  `PatternMatchFailure`. Keep the edge but substitute an independent non-result
  word: both likewise fail. Suppress or duplicate the call/edge and the exact
  emission/effect census REDs. This separates producer existence, result identity,
  and binding — a control that cannot distinguish the three is manufactured and is
  a HARD STOP.
- **AC-PRODUCTS-EXACT** — pin the merge-base negatives. The candidate
  `SourceFormat::Ken` witnesses `fs-read-at-offset-single` / `rt_read_offset_stage`
  and `fs-write-at-offset-single` / `rt_write_writable_stage` agree with the
  interpreter on exact `InvalidOffset`; each target effect and tail resumption
  occur exactly once. If write exposes another blocker, HARD-STOP rather than
  weaken or substitute. Both rows become durable, non-ignored tests.
- **AC-NEGATIVE-SCOPE** — Direct remains 3/3 through `DirectInvocationReturn`;
  ordinary `Ret`, non-governed calls, malformed topology, and non-checked routes
  preserve behavior. No spec/kernel/trust/wire change.
- **AC-AFFECTED-CLOSURE** — local commands use `scripts/ken-cargo` ONLY: targeted
  `-p ken-runtime`, exact `-p ken-cli --test rt_parity_native` product/mutation
  rows, and the existing `ken run` end-to-end targets. CI supplies all `ken-cli`
  integration targets plus the workspace gate. Record base and candidate; closure
  is NOT a hand-picked source grep.
- **AC-PROHIBITIONS** — zero new runtime words, active-header params, frame slots,
  captures, stores, tags, recovery, general carriers, a second `Ret` body, or
  revival of Produced-transfer / D3 / Direct-only / HS15.

## Reviewers

Architect — the implemented mechanism matches this ruling: the plan is replaced
(not siblinged) and names the exact governed source/case/frame/Ret-body/child; the
edge proof forms only after transport selection and confluence-member validation
and is one move-only compiler proof; the governed Tail emits a single
`jump(return_body, [returned.word])` and never touches the collapse/fallback; the
block handle is compiler state only; prohibition-clean. Runtime QA — the controls
red/green as specified, `AC-CAUSAL-PAIR` genuinely separates producer existence
from result identity from binding (a control that stays green under any of the
three substitutions is manufactured and is a HARD STOP), and the two witnesses
produce exact `InvalidOffset` as durable non-ignored tests. A design fork
HARD-STOPS to the Architect.

## Capability tier

T1 — a soundness-bearing native-lowering construction reviewed on the
provenance/forward-edge argument and its causal controls, not a differential diff.
Size L.

## Sequencing

Lane 1 (runtime), the funded shape-(a) objective (operator 2026-08-30). No
`depends_on` — the base is current main and the coordinates are measurable now.
`blocks: [PX8]` — this closes the Tail composed-return route the
`RT-NATIVE-CARRIED-VALUE` ignored full-program rows exercise. The refuted
authority-only build `RT-COMPOSED-RETURN-PRODUCER-ORDER-BUILD` is `closed`
(superseded, built nothing). All other closed axes stay closed unless a candidate
names and relaxes a further constraint — which is a fresh Architect decision, not
this frame.
