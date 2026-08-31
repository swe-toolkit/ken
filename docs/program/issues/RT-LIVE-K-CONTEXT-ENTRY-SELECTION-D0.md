---
id: RT-LIVE-K-CONTEXT-ENTRY-SELECTION-D0
title: "Scratch-only live-k context-entry selection D0 (no production candidate, QA, Decision, CI, publication, or merge): the SELECTION-OWNERSHIP D0 accepted the READ trace as observation (producer705 completes 0x0305, correct immediate owner, then WRONG ordinary forwarding — body452 demand gets env container 0x0e09 not projected field0) but did NOT authorize a production projection, and measured the WRITE 'first loss' (no call fn32 inside generated context0/u3:57) INSIDE AN UNENTERED FUNCTION — u3:57's own entry marker is zero, so a missing internal instruction is a conditional local fact, not live authority; this is the HS9 predicate again -> HARD STOP 10. This recut keeps the same scratch-only D0 and exact accepted runtime net and moves the WRITE measurement ONE EDGE EARLIER: prove whether context0/u3:57 is ENTERED by a live selector/call from the app486-emitting Function, enumerated by target Function identity (module FuncId46/u2:46), NOT by a fn32 spelling; if no selector/call to context0 exists classify C at app486->context0; if a selector exists but is unhit or wrong-target classify B there; do NOT inspect context0's internal body979 edge as causal until context0 entry is proven live. It also makes both READ controls detector-independent: READ OWNERSHIP decodes the finished CLIF after emission keyed by completed producer SSA plus exact owner slot and requires owner-store cardinality 1, mutating ONLY the store population with the decoder and one fixed completion observation unchanged (drop observes 0, duplicate 2, restore 1); READ FORWARDING places any credited mutation at the FINAL carried call-frame use after materializer473 with the body452 consumer key and position1 fixed, and if no legal compatible redirect exists retains CONTROL_NO_SUBJECT with no response-0x1209 credit. Every absence scan enumerates by Function identity, never fn32 or a source-origin spelling. Preserve per-instance markers, adjacent live positives, effect order, Trap-before-Result, identities36/37, capsule/raw zeros, the 474/473 no-call materializer, and disabled-CLIF byte identity. Restore byte-clean; hand the first-live-edge report to the Architect, who alone selects or rejects a production representation. Any result other than a genuinely-live A at a PROVEN-LIVE coordinate is HARD STOP 11."
status: ready
owner: runtime
size: L
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect HS10 review evt_36h9bcs804saw (thr_51y9b78x92wz3, 2026-08-31): CHANGES REQUIRED — the natural read trace is valid, but the write 'first loss' is inside an unentered Function. Binds exact pickup 978b05dd29ab1a40cdc5b89e4410bd3928cff111, tree 0f84669667686499db8567e1c1e3f069701191d7, report/diff/map/runtime-map/selector-scan/marker-manifest hashes 3709c8f8…ccbb0 / fcf16007…dece / f4bcc8b3…6bbd / 8e4ef8f1…77ed / e9fe4d84…0a21 / c9ff6555…baed, and 50-member evidence manifest c84dcc40…8bd (all 50 members, three 28-member CLIF manifests, both empty disabled-vs-base diffs, seven base blobs, byte-clean restoration, and an independent `scripts/ken-cargo check -p ken-runtime --features px8-ds-test-support` all verified). ACCEPTED OBSERVATIONS: the natural read trace is a sound observation — producer705 completes SSA v12498 (0x0305), stored immediately into target470 call-frame Parameter position1; continuation1 receives 0x0305 at position1; completed u0:47 stores that value as field0 of materialized environment v33 (0x0e09); body452 receives response 0x1209 at position0 and container 0x0e09 at demand position1; Match451 consumes 0x0e09 and traps36 — correct completion and immediate owner, then wrong ordinary forwarding (semantic result nested, consumer gets its environment container); it does NOT authorize a production projection yet. The local write emission facts are also true — planner carries PredeclaredFunctionId(8)/body1238 -> PredeclaredFunctionId(5)/body979 at call-site979; body979 emitted as FuncId46/u2:46; generated context0/u3:57 contains no call to that target and body979/Result::Err/InvalidOffset/producer940 markers are all zero beside live app486/body465/Match464 positives. BLOCKING FINDING 1 (not the first dynamic loss): u3:57's own context0/body1238 entry marker 0x51470005000004d6 is zero in the same run; no live predecessor into u3:57 is established. A missing instruction inside an unentered Function is a conditional local fact, not causal runtime authority — adding the internal body979 call would still execute zero times if context0 remains unselected. This is exactly the HS9 predicate: a valid local fact placed where its consuming dynamic control choice does not coexist; the prior accepted live-k measurement already found app486 materializes the environment for body1238 and emits no body1238 call. The artifact may retain the narrower statement 'the inactive context representation also lacks its body979 call' but may NOT label that coordinate the first runtime loss or select a production repair there. BLOCKING FINDING 2 (selector scan is FuncRef-spelling scoped): finished u3:57 declares FuncId46 more than once — both `fn32 = colocated u0:46 sig32` and `fn40 = colocated u0:46 sig40`; neither is called, so the likely conclusion survives, but the oracle counts only `call fn32(` and the report said the target is declared once. Absence must be closed by callee Function identity: enumerate every FuncRef resolving to module FuncId46/u2:46 and every call using any such ref; a local ref name is not target authority. BLOCKING FINDING 3 (reached-edge controls not independent): the read owner mutation changes the detector with the subject — the owner marker is emitted inside the same `for copy in 0..immediate_owner_copies` loop as the owner store; drop yields three completion records and zero owner records, duplicate three completion and six owner records, neither reaching a distinct owner-cardinality refusal (the run merely reaches the unchanged baseline trap); CONTROL_APPLIED is the mutation's own report, not an independent detector. The read-forward drop/duplicate controls fire while the mutated position is still Specialized (removed_phase=Specialized) and refuse on descriptor arity before the measured final carried use — they prove the earlier vector's cardinality checks, not the actual 0x0e09 use at body452 demand position1 (the honest redirect no-subject result receives no credit, as reported). For write, planner-edge drop and redirect retain the identical trap precisely because no selector consumes them, and duplicate refuses at the upstream graph-uniqueness law — useful inertness/plan-population probes, not mutation proofs of an emitted selector; there is no emitted selector subject, so report CONTROL_NO_SUBJECT at that absent edge rather than crediting the planner mutations as selector controls. REQUIRED RECUT (keep the same scratch-only D0 and exact pickup; do not open production, QA, Decision, CI, publication, or merge): (1) For WRITE, begin one edge earlier — bind the exact live app486-emitting Function/instruction to the expected generated context0/u3:57 entry; enumerate finished FuncRefs/calls by target Function identity; if no selector/call to context0 exists, stop there and classify C at app486 -> context0; if a selector exists but is unhit or wrong-target, classify B there; do NOT inspect context0's internal body979 edge as causal until context0 entry is proven live. (2) For READ ownership, keep the mutation detector independent — decode the finished CLIF after emission, keyed by completed producer SSA plus exact owner slot, and require owner-store cardinality 1; mutate only the store population; the decoder and one fixed completion observation must remain unchanged; drop must observe 0, duplicate 2, then restore 1. (3) For READ forwarding, place any credited mutation at the final carried call-frame use after materializer473, not at the earlier Specialized operand vector; keep the body452 consumer key and position1 fixed; if no legal compatible redirect exists, retain CONTROL_NO_SUBJECT and claim no redirect credit; do not use response 0x1209. (4) For every absence scan, enumerate by Function identity, not fn32 or any source-origin spelling; preserve the existing per-instance markers, adjacent live positives, effect order, Trap-before-Result, identities36/37, capsule/raw zeros, 474/473 no-call, and disabled-CLIF byte identity. DISPOSITION: changes required; no production representation selected; do not add a call inside context0 or add the read projection on the prior report's authority; hard stop 10 stands; symptom-inventory entry 10 recorded on the closed SELECTION-OWNERSHIP node: 'the missing call inside an unentered generated context was treated as the first runtime loss — keyed on local emitted Function structure rather than a live incoming selector'; the existing HS9 Research advisory already covers this predicate, no new Research pull required at stop 10. Steward owns the recut and Runtime release; runtime parked until this named kick. Scratch-only doc recut per COORDINATION section 2; the doc commit advances current origin/main and the accepted runtime-net product blobs are unchanged (recheck the seven at pickup against the accepted base 978b05dd29)."
---

> # READY — SCRATCH-ONLY LIVE-K CONTEXT-ENTRY SELECTION D0. Released to the runtime
> # ring (lane 1). Runtime is parked; this IS the release.
> #
> # MEASUREMENT node. It lands NO production candidate, opens NO PR, routes NO QA,
> # needs NO Decision or merge. It reuses the accepted runtime net at exact pickup
> # `978b05dd29` (recheck the seven accepted blobs), returns a report plus a scratch
> # diff, source/CLIF/address maps, runtime logs, and digests, and restores the
> # branch byte-clean. The Architect ALONE reviews the report and rules on the
> # production design. Like every prior D0 in this chain, it is never `merged`.
> #
> # **Why this recut exists (Architect HS10 review evt_36h9bcs804saw).** The prior
> # D0 called "no `call fn32` inside generated context0/u3:57" the first WRITE loss
> # — but u3:57's OWN entry marker is zero: context0 is UNENTERED. A missing
> # instruction inside an unentered Function is a conditional local fact, not live
> # authority (adding the internal call would still execute zero times). That is the
> # HS9 predicate — a valid local fact placed where its consuming dynamic control
> # choice does not coexist. So the WRITE arm moves ONE EDGE EARLIER, and both READ
> # controls become detector-independent. HARD STOP 10 stands; no production
> # representation is selected.

## WRITE arm — begin ONE EDGE EARLIER: is context0 entered by a live selector?

Do **not** inspect context0's internal body979 edge as causal until context0
entry is proven live.

1. Bind the exact **live app486-emitting Function/instruction** to the expected
   generated **context0/u3:57 entry**. app486 materializes the environment for
   body1238; the question is whether a live selector/call then **enters** the
   generated context.
2. Enumerate finished **FuncRefs and calls by target Function identity** — module
   `FuncId46/u2:46` (and the context0 entry Function) — **never** by a local ref
   spelling. `u3:57` declares `FuncId46` more than once (`fn32 = colocated u0:46
   sig32` AND `fn40 = colocated u0:46 sig40`); a `call fn32(` scan is spelling-
   scoped and misses `fn40` and any other ref resolving to the same target.
3. Classify at the first proven-live absent edge:
   - **C** — no selector/call to context0 exists (the target FuncRef may survive
     while the call instruction does not): the first planner/erasure loss, at
     `app486 -> context0`.
   - **B** — a selector exists but is unhit, selects another Function, or enters
     context0 but misses a named successor: at that exact transition.
   - **A** — only if context0 entry is proven live AND a genuinely-live producer940
     value advances through ownership/forwarding.

## READ ownership arm — DETECTOR INDEPENDENT of the mutated subject

The prior owner control emitted its marker inside the same
`for copy in 0..immediate_owner_copies` loop as the owner store, so mutating the
store also mutated the detector (`CONTROL_APPLIED` was the mutation's own report).

- **Decode the finished CLIF after emission**, keyed by the completed producer SSA
  plus the exact owner slot, and require **owner-store cardinality 1**.
- **Mutate ONLY the store population.** The decoder and one fixed completion
  observation must remain unchanged across every mutation.
- Drop must observe **0**, duplicate **2**, then restore **1**. That is the
  independent proof the subject changed and the detector did not.

## READ forwarding arm — mutate at the FINAL carried use, not the Specialized vector

The prior forward controls fired while the mutated position was still
`Specialized` (`removed_phase=Specialized`) and refused on descriptor arity —
proving the earlier vector's cardinality checks, not the actual `0x0e09` use.

- Place any credited mutation at the **final carried call-frame use after
  materializer473**, with the **body452 consumer key and position1 fixed.**
- If no legal compatible redirect exists at that use, retain **`CONTROL_NO_SUBJECT`**
  and claim no redirect credit. **Do not use response `0x1209`** as a subject.

## Absence scans and preservation

- **Every absence scan enumerates by Function identity**, never `fn32` or a
  source-origin spelling. A zero-hit claim requires manifest/CLIF proof that every
  named marker was emitted plus an adjacent known-live positive in the same run.
- Preserve the existing per-instance markers, adjacent live positives (app486 /
  body465 / Match464), exact effect order, Trap-before-Result, identities36/37,
  capsule/raw zeros, the 474/473 zero-argument no-call materializer, and
  marker-disabled CLIF byte identity to the accepted base. Do not synthesize
  `ResourceBodyOk`, treat environment as result, borrow HostResult, reapply `k`,
  reopen body662/body888, reify a runtime closure/tag/continuation, or reorder
  effects. Restore byte-clean.

## Deliverable and the hard-stop gate

A report plus a scratch diff, source/CLIF/address maps, runtime logs, an evidence
manifest, and digests — restored byte-clean — handed to the Architect, who alone
reviews it and, after the first-live-edge report, selects or rejects a production
representation. **Any result other than a genuinely-live A at a PROVEN-LIVE
coordinate is HARD STOP 11** (symptom-inventory entry 11 on this node's closeout).
The HS9 Research advisory `evt_6w61wvkzgm8aq` already binds this seam's predicate,
so no new advisory is mandatory at HS11; the Architect decides whether a further
advisory is warranted before selecting another technique. Runtime production stays
blocked until this successor is ruled. Do not add a call inside context0 or add
the read projection on the prior report's authority.
