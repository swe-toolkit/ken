---
id: RT-LIVE-K-RESULT-SELECTION-OWNERSHIP-D0
title: "Scratch-only live-k result selection/ownership D0 (no production candidate, no QA, no Decision, no merge): the ORDINARY-RESULT-SUPPLY-ROLE D0 accepted READ as post-production (producer705 executes before materializer473 but the marker fires before finish_source_constructor, so completion/ownership/forwarding are unmeasured) and rejected WRITE as pre-production (producers507/720/940 all zero before materializer486 and Match464; ordinary_d0_control_closure restarts at each producer's own function_body, mistaking local reachability for live supply) → HARD STOP 9, RULED do-not-relabel-as-B. This successor runs ONE measurement with TWO phase-specific arms because the production question needs both and they must not be forced through a common starting point. READ ARM begins AFTER production, not at Function discovery: place the first observation AFTER finish_source_constructor completes producer705, record the exact finished SSA value and runtime word; bind that value to its immediate emitted owner (RoutedAnswer, return, call result, Result-slot) derived from completed CLIF/emission records not source adjacency; follow that same value by SSA identity and runtime word through every actual call/block argument to the locked consumer call and ordinary operand position for body452/Match451; stop at the first absent edge — completion without an owner is wrong result ownership, an owned value that reaches the seam while app473 environment 0x0e09 occupies the demand operand is wrong ordinary forwarding, and response 0x1209 remains a distinct negative control. WRITE ARM begins at exact semantic Function/control selection with producer940 ONLY (body979 Match alt1 Result::Err → Match976 alt9 ResourceError::InvalidOffset → Construct941/producer940 ResourceBodyOk); producers507 and 720 are same-family sites, never interchangeable supplies or fallbacks: independently bind the exact emitted selector/predecessor expected to enter body979/unit5, then the exact emitted Function instance and entry occurrence with every emitted instance/address mapped explicitly (source-origin equality cannot choose a machine instance); only after entry, mark the exact predecessor terminator and selected successor for body979's Result::Err then Match976's InvalidOffset; if producer940 is reached, observe after constructor completion and continue through first ownership and ordinary forwarding as for read; stop at the first absent edge and classify — no emitted selector/Function representation for the fixed source choice is outcome C at the first planner/erasure loss, an emitted selector that is unexecuted / selects another Function / enters body979 but misses a named successor is outcome B at that exact transition, a completed producer940 value advances to ownership/forwarding and may support A only if genuinely live. A hit in ken_continuation_context_1 around app486 does NOT substitute for body979 entry (producer507 already proves Function-local reachability can be false on the selected branch); per-instance results are required, aggregate zero counts are not. Expected selection comes from the locked source/reference input path; actual selection and supply come independently from finished CLIF/object selector instructions, Function entries, SSA definitions, and consumer operands; they meet only at the equality/use check. For every reached natural edge use a compile-preserving population-side mutation, opposite authority fixed; the marker map distinguishes source origin, emitted Function instance, instruction address, and runtime hit count. Preserve live apps138/175, materializers473/486, exact effect order, Trap-before-Result, identities36/37, capsule/raw zeros, the 474/473 no-call materializer, and marker-disabled CLIF byte identity to the accepted base. Do NOT synthesize ResourceBodyOk, treat environment as result, borrow HostResult, reapply k, reopen body662/body888, reify a runtime closure/tag/continuation, or reorder effects. Land READ in an ownership/forwarding classification and WRITE in exactly one of A/B/C. Restore byte-clean and report generic derivation, exact rows, controls, and hashes."
status: closed
owner: runtime
size: L
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_2k0rk8nwpn97c (thr_51y9b78x92wz3, 2026-08-31): HS9 ARCHITECT RULING — DO NOT RELABEL WRITE AS B; RUN ONE BOUNDED, PHASE-SEPARATED SELECTION/OWNERSHIP D0. Binds byte-clean pickup d9ed530dbf596b057003ff1363f3f1dbf8f0c8c2, tree 4bddffb5cdb0aa9e04ef1265883d7a41486f1054, the independently reviewed ordinary-result report/diff/map/address/evidence/marked-CLIF/disabled-CLIF/base-CLIF artifacts, and the HS9 Research advisory evt_6w61wvkzgm8aq artifact SHA-256 f842adcb2d50b2921789599c09a4b95af7b2740c70223100e7637eef1592ad59. Advisory disposition (Architect): Research's phase separation is ACCEPTED and corroborates the prior evidence ruling — read is post-production (producer705 executes before materializer473, but the existing marker is before finish_source_constructor; completion, first ownership, and forwarding remain unmeasured); write is pre-production (producers507/720/940 all remain zero before materializer486 and Match464; a carrier cannot repair a value that was never produced). The current artifact must NOT be recut merely by changing 'write A' to 'write B': original outcome B requires one represented-but-unexecuted transition and its exact first missing edge, the present evidence establishes the class but not that coordinate, and outcome C remains possible if the expected selector/Function edge was never emitted. Shared-predicate decision: HS9 shares the chain's governing predicate — a valid static/local fact repeatedly placed where the consuming dynamic control/value choice does not coexist; here ordinary_d0_control_closure restarts at each producer's own function_body, converting 'reachable if this Function is entered' into apparent live supply. The structural closure is now binding: no producer census rooted at its own Function body is execution authority; every claimed semantic supply must include, in order, a dynamically selected emitted Function/control predecessor, the exact completed producer value and first owner, and the exact emitted argument/use that reaches the demand; constructor-family membership and local dominance are insufficient (recorded as symptom-inventory entry 9 on the closed ORDINARY-RESULT node). This successor is NOT an in-place relabel of the spent ordinary-result D0; it has two phase-specific arms in one measurement. READ arm: fixed semantic authority is producer705 on the reference InvalidOffset path, existing execution of its pre-completion marker accepted, do not rediscover Function selection — (1) first observation AFTER finish_source_constructor completes producer705, record the exact finished SSA value and runtime word not a pre-construction marker; (2) bind that value to its immediate emitted owner (RoutedAnswer/return/call result/Result-slot) derived from completed CLIF/emission records not source adjacency; (3) follow that same value by SSA identity and runtime word through every actual call/block argument to the locked consumer call and ordinary operand position for body452/Match451; (4) stop at the first absent edge — completion without an owner is wrong result ownership, an owned value that reaches the seam while app473 environment 0x0e09 occupies the demand operand is wrong ordinary forwarding, response 0x1209 is a distinct negative control not a substitute. WRITE arm: fixed semantic authority is producer940 ONLY (body979 Match alt1 Result::Err → Match976 alt9 ResourceError::InvalidOffset → Construct941/producer940 ResourceBodyOk); producers507 and 720 are same-family sites, never interchangeable supplies or fallback candidates — (1) independently bind the exact emitted selector/predecessor expected to enter body979/unit5, then the exact emitted Function instance and entry occurrence, every emitted instance/address mapped explicitly (source-origin equality cannot choose a machine instance); (2) only after entry is observed, mark the exact predecessor terminator and selected successor for body979's Result::Err then Match976's InvalidOffset in order; (3) if producer940 is reached, observe after constructor completion and continue through first ownership and ordinary forwarding exactly as for read; (4) stop at the first absent edge and classify precisely — no emitted selector/Function representation for the fixed source choice is outcome C at the first planner/erasure loss, an emitted selector that is unexecuted / selects another Function / enters body979 but misses a named successor is outcome B at that exact transition, a completed producer940 value advances to ownership/forwarding and may support A only if the value is genuinely live. A hit in ken_continuation_context_1 around app486 does NOT substitute for body979 entry (producer507 already proves Function-local reachability can be false on the selected branch); per-instance results required, aggregate zero counts are not. Independent authority and controls: expected selection comes from the locked source/reference input path; actual selection and supply come independently from finished CLIF/object selector instructions, Function entries, SSA definitions, and consumer operands; they meet ONLY at the equality/use check. For every reached natural edge use a compile-preserving population-side mutation at that edge while keeping the opposite authority fixed — redirect the actual emitted selector/owner/use to a distinct real compatible target or payload without changing the consumer key or cardinality; separately drop and duplicate the edge population; each mutation must prove the subject changed and the detector did not; a mutation of source472's already-wrong environment is not credit for producer940 selection or producer-to-demand supply; if no legal same-key subject exists report CONTROL_NO_SUBJECT before refusal and claim no mutation credit; restore an independent exact positive after each negative. The marker map must distinguish source origin, emitted Function instance, instruction address, and runtime hit count; a source origin with multiple machine sites is not one marker (producer720 has nine machine sites, producer507 two); a zero-hit claim requires manifest/CLIF proof that every named marker was emitted plus an adjacent known-live positive in the same run. Preserve live applications138/175, materializers473/486, exact effect order, Trap-before-Result, identities36/37, capsule/raw zeros, the 474/473 zero-argument no-call materializer, and marker-disabled CLIF byte identity to the accepted base; do not infer a target or operand from body/table presence, arity, shape, order, proximity, or numeric id; do not synthesize ResourceBodyOk, treat environment as result, borrow HostResult, reapply k, reopen body662/body888, reify a runtime closure/tag/continuation, or reorder effects. This successor remains scratch-only and must restore byte-clean; it grants no production candidate, QA, Decision, CI, publication, merge, or product claim. After its first-missing-edge report, the Architect will select or reject a production representation. Steward owns the fresh frame/thread and Runtime release. Base origin/main d9ed530db (this doc-only recut commit only advances it; the accepted runtime-net product blobs are unchanged — recheck the seven at pickup against the accepted base). @steward owns close/reframe/release; runtime parked until this named kick. Steward-recut per COORDINATION section 2."
---

> # CLOSED 2026-08-31 — HARD STOP 10. READ TRACE VALID (not projection-authorizing);
> # WRITE OUTCOME C MEASURED INSIDE AN UNENTERED FUNCTION (not the first live loss).
> #
> # D0 report delivered (runtime-implementer evt_d002kym2bmnh, pickup 978b05dd29,
> # tree 0f846696, byte-clean, seven accepted blobs intact, no production). Architect
> # review evt_36h9bcs804saw (verified all 50 evidence members, three CLIF manifests,
> # empty disabled-vs-base diffs, byte-clean restoration, independent
> # `ken-cargo check -p ken-runtime --features px8-ds-test-support`):
> #
> # - READ trace ACCEPTED AS OBSERVATION: producer705 completes v12498/0x0305,
> #   stored to target470 Parameter position1; continuation1 receives 0x0305 at
> #   position1; u0:47 stores it as env v33 field0 (0x0e09); body452 gets response
> #   0x1209 at position0 and container 0x0e09 at demand position1; Match451 traps36.
> #   Correct completion + immediate owner, then WRONG ordinary forwarding (consumer
> #   gets the environment container, not the projected nested field0). It does NOT
> #   authorize a production projection yet.
> # - WRITE emission facts TRUE but MISPLACED: the report called "no `call fn32` in
> #   generated context0/u3:57" the first loss, but u3:57's OWN entry marker
> #   `0x51470005…04d6` is zero in the same run — context0 is UNENTERED. A missing
> #   instruction inside an unentered Function is a conditional local fact, not
> #   causal runtime authority (adding the internal body979 call would still execute
> #   zero times). This is the HS9 predicate again, not a new exception.
> # - Two further control defects: the selector scan was FuncRef-spelling scoped
> #   (`call fn32(` only; u3:57 declares BOTH `fn32` and `fn40` = colocated u0:46),
> #   and the reached-edge controls were not detector-independent (owner marker
> #   emitted in the same loop as the owner store; read-forward controls fired at
> #   `Specialized` arity, not the actual 0x0e09 use; write planner mutations have no
> #   selector subject -> CONTROL_NO_SUBJECT).
> #
> # HARD STOP 10 STANDS. No production representation selected; do NOT add a call
> # inside context0 or add the read projection on this report's authority. Symptom-
> # inventory entry 10: **the missing call inside an unentered generated context was
> # treated as the first runtime loss — keyed on local emitted Function structure
> # rather than a live incoming selector.** HS9 Research advisory (evt_6w61wvkzgm8aq)
> # already covers this predicate; no new Research pull at stop 10. Successor recut:
> # [[RT-LIVE-K-CONTEXT-ENTRY-SELECTION-D0]] (Steward owns recut + Runtime release).
> # Everything below is the ORIGINAL frame, retained as the closed record.

> # READY — SCRATCH-ONLY LIVE-K RESULT SELECTION/OWNERSHIP D0. Released to the
> # runtime ring (lane 1) on `origin/main` `d9ed530db`. Runtime is parked; this IS
> # the release.
> #
> # This is a MEASUREMENT node. It lands NO production candidate, opens NO PR,
> # routes NO QA, and needs NO Decision or merge. It reuses the accepted runtime
> # net, returns a report plus a scratch diff, source/CLIF/address maps, runtime
> # logs, and digests, and restores the branch byte-clean at the end. The
> # Architect ALONE reviews the D0 report and rules on the production design.
> # Like every prior D0/D1 in this chain, it is never `merged`.
> #
> # **This is NOT an in-place relabel of the ordinary-result D0.** Read A was
> # accepted as POST-production (producer705 runs before materializer473, but its
> # marker fires before `finish_source_constructor`, so completion / ownership /
> # forwarding are unmeasured). Write A was rejected as PRE-production
> # (producers507/720/940 all zero before materializer486 and Match464; a census
> # rooted at each producer's own `function_body` mistook local reachability for
> # live supply). **Do NOT relabel write as B** — outcome B needs one
> # represented-but-unexecuted transition and its exact first missing edge, and the
> # evidence establishes the class but not that coordinate; outcome C is still
> # open. This D0 runs ONE measurement with TWO phase-specific arms, because the
> # two witnesses fail at different phases and must NOT be forced through a common
> # starting point.

## READ arm — begin AFTER production, not at Function discovery

Fixed semantic authority is the exact source producer705 on the reference
`InvalidOffset` path. Existing execution of its pre-completion marker is
accepted; **do not rediscover Function selection.**

1. Place the first observation **after `finish_source_constructor` completes
   producer705.** Record the exact finished SSA value and runtime word — not the
   source origin or a pre-construction marker.
2. Bind that value to its immediate emitted owner: exact `RoutedAnswer`, return,
   call result, `Result`-slot, or other emitted owner. The owner is derived from
   completed CLIF/emission records, **not** inferred from source adjacency.
3. Follow that same value, by SSA identity and runtime word, through every actual
   call/block argument to the locked consumer call and ordinary operand position
   for body452/Match451.
4. Stop at the first absent edge. **Completion without an owner is wrong result
   ownership.** An owned value that reaches the seam while app473 environment
   `0x0e09` occupies the demand operand is **wrong ordinary forwarding.** Response
   `0x1209` remains a distinct negative control and is not a substitute.

## WRITE arm — begin at exact semantic Function/control selection

Fixed semantic authority is **producer940 only**: body979 Match alternative1
`Result::Err` → Match976 alternative9 `ResourceError::InvalidOffset` →
Construct941 / producer940 `ResourceBodyOk`. **Producers507 and 720 are
same-family sites, not interchangeable supplies and never fallback candidates.**

1. Independently bind the exact emitted selector/predecessor expected to enter
   body979/unit5, then the exact emitted Function instance and entry occurrence.
   Source-origin equality cannot choose a machine instance — every emitted
   instance/address is mapped explicitly.
2. Only after entry is observed, mark the exact predecessor terminator and
   selected successor for body979's `Result::Err`, then Match976's `InvalidOffset`
   successor, in order.
3. If producer940 is reached, observe after constructor completion and continue
   through first ownership and ordinary forwarding **exactly as for read.**
4. Stop at the first absent edge and classify precisely:
   - **C** — no emitted selector/Function representation for the fixed source
     choice: the first planner/erasure loss.
   - **B** — an emitted selector that is unexecuted, selects another Function, or
     enters body979 but misses a named successor: at that exact transition.
   - **A** — a completed producer940 value advances to the ownership/forwarding
     classification, and may support A **only if the value is genuinely live.**

A hit in `ken_continuation_context_1` around app486 does **not** substitute for
body979 entry: producer507 already proves Function-local reachability can be
false on the selected branch. **Per-instance results are required; aggregate zero
counts are not.**

## Independent authority and controls

Expected selection comes from the locked source/reference input path. Actual
selection and supply come **independently** from finished CLIF/object selector
instructions, Function entries, SSA definitions, and consumer operands. They meet
**only at the equality/use check** — a control that rebuilds one from the other
is detector-side and proves nothing.

- For every reached natural edge, use a **compile-preserving population-side
  mutation** at that edge while keeping the opposite authority fixed: redirect
  the actual emitted selector/owner/use to a distinct real compatible target or
  payload **without changing the consumer key or cardinality.**
- **Separately drop and duplicate** the edge population.
- Each mutation must prove the **subject changed and the detector did not.** A
  mutation of source472's already-wrong environment is **not** credit for
  producer940 selection or producer-to-demand supply.
- If no legal same-key subject exists, report **`CONTROL_NO_SUBJECT`** before
  refusal and claim no mutation credit. Restore an independent exact positive
  after each negative.
- The marker map must distinguish **source origin, emitted Function instance,
  instruction address, and runtime hit count.** A source origin with multiple
  machine sites is not one marker (producer720 has nine machine sites, producer507
  two). A zero-hit claim requires manifest/CLIF proof that every named marker was
  emitted **plus** an adjacent known-live positive in the same run.

## Preservation invariants

Preserve live applications138/175, materializers473/486, exact effect order,
Trap-before-Result, identities36/37, capsule/raw zeros, the 474/473 zero-argument
no-call materializer, and marker-disabled CLIF byte identity to the accepted
base. Do not infer a target or operand from body/table presence, arity, shape,
order, proximity, or numeric id. Do not synthesize `ResourceBodyOk`, treat
environment as result, borrow HostResult, reapply `k`, reopen body662/body888,
reify a runtime closure/tag/continuation, or reorder effects. Restore the branch
byte-clean at the end.

## Land — READ in ownership/forwarding; WRITE in exactly one of A/B/C

- **READ:** the finished producer705 value's completion, first owner, and
  forwarding to the body452/Match451 seam — classified as correctly forwarded, or
  wrong result ownership, or wrong ordinary forwarding (env `0x0e09` at the demand
  operand).
- **WRITE:** exactly one of **A** (genuinely-live producer940 value through
  ownership/forwarding), **B** (one exact represented-but-unexecuted transition),
  or **C** (no emitted selector/Function representation — first planner/erasure
  loss).

## Deliverable and the hard-stop gate

A report plus a scratch diff, source/CLIF/address maps, runtime logs, an evidence
manifest, and digests — restored byte-clean — handed to the Architect, who alone
reviews it and, after the first-missing-edge report, selects or rejects a
production representation. **Any successor result other than a genuinely-live A is
HARD STOP 10** (symptom-inventory entry 10 on this node's closeout). The HS9
Research advisory `evt_6w61wvkzgm8aq` already binds this exact seam, so no new
advisory is mandatory at HS10; the Architect decides whether a further advisory is
warranted before selecting another technique. Runtime production stays blocked
until this successor is ruled.
