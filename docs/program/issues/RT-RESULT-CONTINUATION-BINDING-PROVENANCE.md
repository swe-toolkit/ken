---
id: RT-RESULT-CONTINUATION-BINDING-PROVENANCE
title: "RT-ITREE D2/D3A/D3B — repair the checked ITree Ret fresh-result binding, ROUTE-SPECIFICALLY. CURRENT AUTHORITY IS HS13 (evt_59t7b49m41z8m); route-specificity is inherited from HS12 (evt_7a6pp8n24r1ms), whose MECHANISM LOCUS is refuted. The two arms fail DIFFERENTLY and a uniform diagnosis is withdrawn. DirectInvocationReturn: the carried CFG arm of call_checked_ih_transport_from_case_environment (core.rs:7699-7714) settles InlineNoCall and returns the transported CheckedIhCapturedEnvironment word with NO Direct declared-call application on that arm, so D3A ADDS the ruled declared call and its local return is fresh R2. TailProducerToBackedge: the governed application ALREADY EXISTS and a real declared-call producer ALREADY emits fresh R2 (calls.rs:2022, Result-slot load :2120, RoutedAnswer::checked(returned) source.rs:4369-4374) — nothing is unapplied; the bodyless recursor arm (source.rs:4478-4515) DISCARDS that result and substitutes the initial carried seed, so the missing work is exact produced-result TRANSFER. HS13 (evt_59t7b49m41z8m) REFUTED HS12's MECHANISM LOCUS while its semantic property stands: the Tail route does not exist at the producer (cursor N vs a freshly minted selected continuation at N+2) and composed lowering reduces RoutedAnswer to a bare LoweringOperand in between, so no value/route pair exists there to couple and ownership cannot carry an already-erased value. THIS NODE IS THEREFORE D0-ONLY: enumerate every natural-path seam where the Rust type shrinks to LoweringOperand plus the complete caller closure, and determine whether that closure admits ONE compile-time affine return typestate (Produced owns operand and ContinuationCallIdentity; Routed forms only by consuming Produced when the exact Tail route arrives with agreeing producer identity; the active jump consumes Routed once; ordinary/direct is a distinct exhaustive variant). A compiler-control return value, never a runtime carrier. D0 lands no production and routes no QA; an unboundable closure is HS14. D0 ATTEMPT 1 IS NOT ACCEPTED (Architect evt_7tsep4b5sdqew): its instrument and restoration were sound but it rooted the ledger at the WRONG one of the exactly TWO RoutedAnswer::checked callers (core.rs:7622-7626 via ClaimedContinuationResult) instead of Tail's own producer at source.rs:4369-4374, so its NO describes a different producer; both D0 ACs now name the start and require type/identity correlation rather than a shared route/role tag. D3B binds the DELIVERED fresh R2 to the ordinary Ret-case capture on both arms (301/460/459/452 are READ-side evidence coordinates only; the write analogue is derived independently). D2 localization ACCEPTED as evidence (ac1ebdacb; no merge, no QA) and is DIRECT-SCOPED. The merge is ATOMIC (D3A + graph-authorized result flow + product) — no application-only checkpoint. The landed fresh-result route contributes retained topology/identity facts only; NO predecessor supplies Tail value authority and none is authorized."
status: active
owner: runtime
size: M
gate: none
depends_on: [RT-ITREE-DEFAULT-SELECTION-PROVENANCE, RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR, RT-CHECKED-IH-K-AVAILABILITY-LOCATOR, RT-CHECKED-IH-GENERATED-ENTRY-ACCESS, RT-CHECKED-IH-SELF-RESUMPTION-RESULT-PROVENANCE, RT-CHECKED-IH-FRESH-RESULT-ROUTE]
blocks: []
github: null
origin: "Architect hard-stop-2 ruling evt_5w03f4zbg02ry, 2026-08-26, splitting RT-ITREE-DEFAULT-SELECTION-PROVENANCE; then hard-stop-3 ruling evt_1hren6zm8mgxv, 2026-08-26 (option (c), D2/D3 phase separation, Research advisory evt_4cbecpkg2e0gs accepted). D1's route slice landed independently (21d62130); this node localizes the ResourceBodyResult continuation-binding boundary observed on top of it, then repairs it. Steward-owned recut per the ruling; the final-product ACs (AC-5 / AC-D1-PRODUCT / final InvalidOffset witnesses) live here. Hard-stop-4 ruling evt_6mnawfvm8fc4j, 2026-08-26: the single D3 application leap is split into coupled D3A (application, evidence only) + D3B (result-flow localization then single-edge repair); the atomic merge adds per-step result-flow pairing and dual suppression. Inventory fold 529f21c43e1c0c5257d2f7898481aaa3dc3a0429 (entries 1-4). Frame fixed-input correction evt_10rgb8n31c5sj, 2026-08-26: origins 301/460/459/452 are READ-side evidence coordinates only; D3B derives the write analogue independently from its own planner facts and forbids reusing the read coordinates as write authority (Steward-owned, not a Decision). Hard-stop-5 ruling evt_494k61s04fnv9, 2026-08-26: D3B localization is VALID and lowering has reached the end of its authority — the missing component is an UPSTREAM planner-owned checked-IH result-successor relation, framed as the independently-landable predecessor RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR (which this node now depends_on); D3A stays frozen/non-landable until it lands, then the atomic D3A+D3B consumer builds and D3B consumes ONLY that successor projection. Inventory fold 244b2468afd4f0cd06837fd3079f291d7d330af5 (entry 5). Architect INCORPORATION ruling evt_2prk31prke9cc, 2026-08-26 (accepting Research advisory evt_261gm8y54xttt), grounded on origin/main@a09878026: the predecessor and this consumer conflated two semantic edges by pairing the D3A application result R1 forward to the later capture. Reconciled to the continuation-inheritance reading — D3A applies the inherited continuation capability K at each exact recursive arrival to yield the FRESH result R2, and D3B binds that R2 through ordinary Ret-case/capture semantics; the transitive R1 -> capture requirement is deleted from the operative Objective/Deliverables/ACs/pairing/suppressions/reviewers/Sequencing. Suppression and at-most-once controls are now THREE independent axes (inheritance, application, fresh-result binding). The advisory is incorporated and is NOT hard stop 6 (count remains five). Runtime stays HELD until the predecessor recut lands and the D3A+D3B work is explicitly re-released. Architect hard-stop-8 ruling evt_54efxydhb3n6w, 2026-08-27 (thr_2g0w05my2d5ym), verified on exact base 00e66312b4ef617eb658a2e75db9f99ff2c56492 / tree e286949f8fe5053e4719e54d0cc66adbe073dcdf: on four of five governed coordinates the exact governed K application exposes NO LOCAL LoweringOperand result at CheckedComputationalIHInvocationReturn, because the active self-resumption arm returns Lowered::RecursiveBackedge, a protocol marker and not a value (lowering/source.rs:1969). That is an absence of a LOCAL result and NOT a proof that no fresh dynamic result is eventually produced — the owning carried merge may still produce it, which is precisely D0's question. The landed architecture's two sibling proofs (K authority plus governed application coordinate; fresh-result destination) lack a THIRD — which emitted control edge PRODUCES the fresh dynamic result. That producer relation is a component boundary, not a D3B field, and is framed as the behaviorally-inert predecessor RT-CHECKED-IH-SELF-RESUMPTION-RESULT-PROVENANCE, which this node now depends_on. No mechanism is authorized: spec/40-runtime/42-evaluation.md section 6.2 makes the tail-resumptive loop realization normative, so a mandatory D0 measures the existing carried-loop exit first and a D0 answering NO stops and returns coordinates to the Architect rather than selecting a recursive call, frame morphism, or explicit continuation. The atomic D3A+D3B contract, the three independent suppression axes, and the deletion of the transitive R1 -> capture requirement are all UNCHANGED; the producer proof does not become a fourth axis. D3A+D3B stays frozen with no candidate and no QA until the predecessor lands and the Steward issues a SECOND explicit release. The next hard stop is 9 and mechanically triggers the mandatory Research advisory before any Architect ruling, including during the predecessor's D0. TERMINAL CLAUSE — HS9 THROUGH HS13, AND HS13 IS THE CURRENT AUTHORITY; everything above this sentence is chronology, not live contract. Hard-stop-9 ruling evt_7wbxwxa74cdnr, 2026-08-28: fresh R2 for the loop rows is the governed K application result AS DELIVERED INTO THE RET CASE'S INPUT BINDER, not the Ret body output and not the merge parameter; the prior D0 YES was an instrumentation error (co-emission is not pairing) and the corrected predecessor RT-CHECKED-IH-FRESH-RESULT-ROUTE landed 7d36d24f0. Hard-stop-12 ruling evt_7a6pp8n24r1ms, 2026-08-28 (clean stop evt_4av4pckhtjd3f, Research advisory evt_4v1wg8hb4zxtm returning a confident negative): stops 1-11 each found an endpoint MISSING; stop 12 found BOTH endpoints PRESENT and the directed edge between them ABSENT. A real declared-call producer and the exact active header both exist; the bodyless Tail transition discards the producer and supplies the initial seed as the only active-jump value. DISPOSITION: NO NEW PREDECESSOR — RT-CHECKED-IH-TAIL-RESULT-PRODUCER-ROUTE is closed/subsumed and removed from depends_on, because changing the active predecessor operand changes the Ret input, capture and body result, so a node landing it first would be either behaviourally inert (and not the repair) or would break the products it landed ahead of. The landed TailResumedRetInput variant is REPLACED by TailProducerToBackedge and its Tail VALUE authority is WITHDRAWN, retaining topology/identity facts only; this atomic candidate supplies the replacement relation and the one typed producer-to-backedge bridge itself. D3A+D3B stays FROZEN and requires a NEW explicit Steward release against the recut frame blob — the HS12 ruling releases nothing, landing releases nothing, and all prior releases are SPENT. Next mechanical Research trigger is HS15. Hard-stop-13 ruling evt_59t7b49m41z8m, 2026-08-28 (clean stop evt_36303wpnhwx18: no commit, no candidate, no QA, baseline restored) — THIS IS THE CURRENT AUTHORITY AND IT SUPERSEDES HS12's MECHANISM LOCUS WHILE HS12's SEMANTIC PROPERTY STANDS. The ring built the HS12-ruled design exactly and the erasure happens underneath it: the declared-call result is produced in selected continuation cursor N, the governed Tail route becomes available only in a freshly minted selected continuation at N+2, and in between lower_computational_match_value_composed and the adjacent source/active return seams reduce RoutedAnswer to a bare LoweringOperand — so no value/route pair exists at the producer to couple, and ownership cannot carry an already-erased value. Cursor N/N+2 is diagnostic evidence only, never an identity key and never a proximity match. THIS NODE IS THEREFORE D0-ONLY and D3A/D3B are NOT AUTHORIZED: enumerate every natural-path seam where the Rust type shrinks to LoweringOperand plus the complete caller closure, and determine whether that closure admits ONE compile-time affine return typestate. D0 lands no production and routes no QA; the Architect reviews the D0 report and Runtime QA stays unrouted; an unboundable closure is HS14, a clean stop. No Decision and no Research advisory follow from HS13; HS15 remains the next mechanical Research trigger. Symptom inventory entry 13 is a LIFETIME/ORDERING class — the endpoint-addition predicate stays terminal at entry 12."
---

> # HARD STOP 14 TAKEN — THE CORRECTED D0 RETURNED **NO**, AND IT IS ACCEPTED
> # (Architect `evt_bm4trnrjpymy`, 2026-08-29). D3A/D3B REMAIN FROZEN.
>
> **The HS13 D0 is COMPLETE and its answer is NO.** Corrected D0 measured on
> exact `47b55bd5a1ab1533063ceffaaaeae7fc4e5161c1`, tree `6903217f9`, frame blob
> `676f94b686f5c45e452ae59de463afb3001bf287`. The Architect accepted the
> measurement and report ONLY: **no production candidate, no QA route, no
> D3A/D3B authority, and no merge gate exists.** The attempt-1 wrong-producer
> defect is closed — the trace roots only at Tail's own
> `call_checked_ih_transport_from_case_environment` return, `source.rs:4369-4374`.
>
> **The locus of the NO, which is the part that matters for the disposition.**
> Bare extraction begins at `source.rs:1250-1264`; the NO becomes forced at
> `ConstructArgument`, `source.rs:1647` and `:1691-1710`, where the exact
> produced operand becomes **constructor-field material** and only the distinct
> `RoutedAnswer::direct(constructed)` continues through `ResumeOuter`. Across all
> 52 roots (30 read, 22 write) there are **zero exceptions** and no
> `HS13_D0_CONSTRUCT_REPLACE` event. **The original SSA is not destroyed — it
> survives as a constructor field. What is lost is its top-level
> compiler-control state.** Keeping it pending as an affine `Produced` value
> until the later Tail route requires it to coexist with the ordinary
> constructed result across the general composed return protocol and the
> compiler-closed caller surface. All five affine conditions are NO.
>
> ### WHAT THIS ESTABLISHES, AND WHY IT IS NOT "STOP 15 OF THE SAME KIND"
>
> **Stops 1-11 each found an endpoint MISSING. Stop 12 found both endpoints
> PRESENT and the directed edge ABSENT. Stop 14 finds that the edge HS12
> permitted is not constructible** — and names the obstruction as a protocol
> this node is explicitly FORBIDDEN to change. Every earlier stop was answerable
> by supplying something. This one is not.
>
> **The HS12-permitted design is refuted by MEASUREMENT, not by argument.** That
> is what the D0-only release was for: the ring did not spend a build on an
> unconstructible design. A D0 that returns NO is the release working.
>
> **DISPOSITION IS ROUTED TO THE ARCHITECT AND IS NOT DECIDED HERE.** The
> obstruction is a component-design object — the general composed return
> protocol at `ConstructArgument` — and this node cannot change it from inside.
> **No D3 release follows from HS14**, the frame is dispositive on that, and
> every prior release remains SPENT. HS15 is still the next mechanical Research
> trigger; HS14 does not trigger one.
>
> **The runtime ring is NOT idle on this.** Lane 1 moved to
> [[RT-RESULT-CLOSURE-LIFETIME-CONTAINMENT-CONTROL]], released 2026-08-29 on a
> measured-clear `aggregates.rs` contention check. **If D3 is re-released over
> `aggregates.rs`, that node yields to this one.**

> # DISCHARGED, NOT SUPERSEDED — HARD STOP 13, `evt_59t7b49m41z8m` (2026-08-28)
>
> **HS13 ran to completion and produced HS14; it was not overturned.** Read the
> distinction literally, because the two labels differ on the one clause that
> still binds: **HS13's D0-ONLY restriction is CARRIED FORWARD, not lifted.**
> D3A/D3B remain unauthorized, this node still lands no production, and Runtime
> QA stays unrouted. A "superseded" banner here would read as releasing exactly
> the restriction that is still in force.
>
> **THIS IS THE OPERATIVE RULING AND THE CURRENT AUTHORITY. It supersedes the
> HS12 banner below and every banner under it, all of which are history.** Cite
> by event id, never by number. Clean stop `evt_36303wpnhwx18`: no commit, no
> candidate, no QA, baseline restored, branch free.
>
> ## THIS NODE IS D0-ONLY. D3A AND D3B ARE NOT AUTHORIZED.
>
> The only authorized phase is the **D0 return-boundary closure** in Phase
> structure, and its acceptance criteria are the two `AC-HS13-D0-*` entries that
> open Acceptance criteria. **D0 lands no production and routes no QA** — the
> Architect reviews the D0 report and Runtime QA stays unrouted. An unboundable
> closure is **HS14**: a clean stop to the Architect, never a mechanism selected
> under pressure. **Every passage below that describes D3A/D3B as the deliverable
> states the TARGET, not the authorized work.**
>
> ## HS12's MECHANISM LOCUS IS REFUTED; ITS SEMANTIC PROPERTY STANDS.
>
> The required directed edge is still producer result -> active jump -> Ret
> input; the seed is still the negative control; `RecursiveBackedge` remains
> marker-only; Direct is preserved; no predecessor is authorized. **But "couple
> the operand with the existing Tail route AT THE EXACT PRODUCER" is IMPOSSIBLE
> AS WRITTEN, because the route does not exist there:** the declared-call result
> is produced in selected continuation cursor N, the governed Tail route becomes
> available only in a freshly minted selected continuation at N+2, and in between
> `lower_computational_match_value_composed` and the adjacent source/active
> return seams reduce `RoutedAnswer` to a plain `LoweringOperand`. No value/route
> pair exists at the producer locus, so every later bind observes `Unavailable`.
>
> **The ring built the HS12-ruled design exactly — a real
> `ContinuationCallIdentity` producer, a move-shaped transfer, nothing on the
> rejection list — and the erasure happens underneath it.** So "carry it by
> ownership" may not be repairable by a better carry, which is why the authorized
> turn measures the boundary instead of selecting a mechanism.
>
> **Cursor N/N+2 is DIAGNOSTIC EVIDENCE ONLY. It is never an identity key and it
> authorizes no proximity match.** Permitting `Unavailable` merely preserves the
> exact `ResourceBodyResult` default — it is not partial progress, and the
> ring's refusal to permit it was correct.
>
> **Appending another carrier field to `SourceSelectedContinuation` cannot repair
> a value already erased by a Rust return type.** That is why this is a return
> BOUNDARY question and not a carrier question.
>
> **Symptom inventory entry 13 is a LIFETIME/ORDERING class and the
> endpoint-addition predicate stays TERMINAL at entry 12.** Do not answer HS13 by
> adding an endpoint.
>
> No Decision and no Research advisory follow from HS13; **HS15 remains the next
> mechanical Research trigger.** Runtime is HELD until the Steward issues a NEW
> EXPLICIT release against the recut frame blob: **landing a frame releases
> nothing**, and all prior releases are SPENT.

> # D0 ATTEMPT 1 IS NOT ACCEPTED — IT STARTED AT THE WRONG CHECKED PRODUCER
> # (Architect `evt_7tsep4b5sdqew`, 2026-08-28). A CORRECTED D0 IS AUTHORIZED.
>
> Attempt 1 (`evt_385bg72pmnghg`; exact object
> `1acfeccf2fb2b52a7803f742c3fb7fe6e2fd8fc4`, tree
> `82c8352460d72c7eb360b0af639d6d54613a9603`) reported NO / HS14. **The
> instrument and the byte restoration are SOUND and are not re-litigated:** the
> required nominal caller token caught two omitted same-shaped callers at
> `core.rs:5326` and `:5349` with E0061, then compiled at 30 callers — a hand or
> grep roster could have looked complete with both absent. What is NOT
> established is that the closure it measured lies on the path D0 asked about.
>
> **`RoutedAnswer::checked` has EXACTLY TWO callers.** The D0 phase locks the
> start to Tail's own producer: `call_checked_ih_transport_from_case_environment`
> at `source.rs:4369-4374`, where `transport.source_call_identity()` is still
> locally available. Attempt 1 instrumented the OTHER one — the core
> `ClaimedContinuationResult` branch, whose `RoutedAnswer::checked` is minted by
> `call_checked_ih_environment_transport` at `core.rs:7622-7626`. The 28/15
> identity-bearing claims and the clone-while-pending counts are therefore facts
> about a different checked producer.
>
> **A shared `route`/`role` tag cannot close that gap**, because both callers
> mint checked route; aggregate product reach of both sites is not the exact
> directed edge. The outcome-changing question stays open in BOTH directions: the
> exact source path may force a source-machine typestate redesign at S4/S5, or it
> may join the general S1/S2/S3 protocol later. **HS14 is licensed only if the
> EXACT SOURCE-PRODUCER closure cannot stay bounded without redesigning general
> composed lowering. Another general producer being broad does not establish
> that predicate.**
>
> **THE AC ASYMMETRY IS THE STEWARD'S DEFECT AND IS FIXED IN THE ACs, NOT ARGUED
> AWAY.** The phase text names the start producer; `AC-HS13-D0-CLOSURE` said only
> "on the natural path" and named no start. **A ring gates on the AC.** When
> exactly two callers mint the same wrapper and the criterion says "the natural
> path", the criterion does not distinguish them — and the reusable form is that
> a start point stated only in phase prose is not a criterion. Both D0 ACs now
> carry the start producer and the correlation standard explicitly.
>
> **Carry forward the Architect's own build-closure finding** (not the blocking
> issue, and independently closed at review): the instrument ledger counted
> `core/tests/constructors.rs:6591`, which the recorded `ken-runtime --lib` checks
> never compile. `test -p ken-runtime --lib --no-run` is green at log SHA-256
> `d00f81893923325878c295202086a4850acd19207d2dafc1c748bff4fda42099`; removing
> only that caller token gives E0061 at log SHA-256
> `00cd42f5b3a12300197da397305944a64f62ae90e78c8dce7897014df33cc40e`; the
> instrument diff is restored at
> `5ee33605e5f9ea566a989c33a14c6cb951bb0cd6c3c2525a578fd44b82f4aea7`.
>
> **The corrected turn's scope is UNCHANGED and still D0-ONLY:** no production,
> no commit, no candidate, Runtime QA stays UNROUTED, D3A/D3B stay FROZEN. A NO
> is still HS14 and still a SUCCESS. A YES still authorizes nothing without an
> Architect review and a SEPARATE later release. **HS15 remains the next
> mechanical Research trigger; HS14 does not trigger it.** Runtime is HELD until
> the Steward releases against the amended frame blob.

> # SUPERSEDED — HARD STOP 12, `evt_7a6pp8n24r1ms` (2026-08-28)
>
> **HISTORY, NOT THE OPERATIVE RULING.** HS13 (`evt_59t7b49m41z8m`) above
> REFUTES this banner's MECHANISM LOCUS. Its SEMANTIC PROPERTY survives — the
> required directed edge, the seed negative, marker-only `RecursiveBackedge`,
> no new predecessor — and is restated in the HS13 banner, which is what to
> cite. **The permitted-design recipe in this banner is refuted and must not be
> built.** It supersedes the HS11 banner below and every banner under it, all of
> which are history. Cite by event id, never by number.
> Bound object `bb33dfb71e302a68377ffde8038f7dc8bd2c82ac`, tree
> `3a63194c34fd0bb8c485f142e61a84769751a742`. Hard stop `evt_4av4pckhtjd3f`;
> Architect hold/call `evt_64n0w33af4n6k`; **Research advisory
> `evt_4v1wg8hb4zxtm`, incorporated.** Exact blobs, grounded independently by the
> Architect: `calls.rs` `fa010fed973dfa8cb638c3a2a546594b93443efb`, `source.rs`
> `88fcc401b0e078f78298a0998d09364b22e64a27`, `core.rs`
> `68f9394ce4d75f68bcfbaaeff7b294040a4fd50b`, predecessor frame
> `c25da2f1539676b8c97e6aef3c27b6c3198ade47`, parent D3 frame
> `d8f0920d5df0cf4c904a51aa39d5776384443167`. No candidate; QA unrouted.
>
> ## THE TWELFTH STOP ADDED BOTH TRUE ENDPOINTS AND PROVED THE EDGE BETWEEN THEM
> ## ABSENT. STOP ADDING ENDPOINTS.
>
> **D0 IS ACCEPTED. A real fresh-result producer EXISTS** — the
> continuation-specialization call at `calls.rs:2022`, its Result-slot load at
> `:2120`, and `RoutedAnswer::checked(returned)` at `source.rs:4369-4374`. The
> source machine structurally pairs operand and route in
> `SourceMachineState::Value`, so **this is not a neighbouring-value guess.**
>
> **The value is then DISCARDED.** The later bodyless recursor arm constructs a
> NEW `RoutedAnswer::direct(Carried(word))` from the residual seed at
> `source.rs:4478-4515`; `core.rs:12218-12233` emits that seed as the active
> jump's first argument, whose first header block parameter becomes the Ret input.
> `RecursiveBackedge` remains a protocol marker, not a value. D0's read rows
> 301/511 and write rows 525/314 dynamically distinguish producer, jump seed, and
> header value.
>
> **THE PRIOR-ART NEGATIVE IS CONFIDENT AND IT IS THE REASON THIS IS THE LAST
> ENDPOINT STOP.** No sound tail-resumptive, trampoline, SSA-join, or ITree
> iteration technique reinterprets an old seed as a fresh result. Every known
> family carries the next value explicitly as the predecessor's
> tail-call/jump/yield argument. **Ken already has the downstream carrier** — the
> active jump's first argument and the header's first block parameter — **and
> lacks only the directed producer-to-predecessor value transition.**
>
> ## NO NEW PREDECESSOR. THE MISSING EDGE IS THE TAIL REPAIR ITSELF.
>
> **`RT-CHECKED-IH-TAIL-RESULT-PRODUCER-ROUTE` IS SUBSUMED INTO THIS NODE AND
> REMOVED FROM `depends_on`.** Changing the active predecessor operand changes the
> Ret input, the capture, and the body result — so it belongs INSIDE the already
> atomic D3A+D3B increment, never in a proof node that lands first. Its D0
> evidence and hard-stop history are preserved as tracker history at
> [[RT-CHECKED-IH-TAIL-RESULT-PRODUCER-ROUTE]]. **Do not add a sibling
> authority.**
>
> ## THE PERMITTED TAIL DESIGN — REFUTED AT HS13. HISTORY, NOT A RECIPE.
>
> **DO NOT BUILD WHAT THIS SECTION DESCRIBES.** HS13 (`evt_59t7b49m41z8m`)
> refuted this MECHANISM LOCUS — the ring implemented it exactly and it cannot
> work, because the Tail route does not exist at the producer and composed
> lowering erases `RoutedAnswer` to a bare `LoweringOperand` before it does.
> **The semantic property below still stands; only the recipe for reaching it is
> withdrawn.** The authorized work is the D0 return-boundary closure in Phase
> structure. Retained so the refuted design is auditable.
>
> At the exact governed producer, **couple the exact `RoutedAnswer::checked(
> returned)` operand with the existing Tail route in the source-machine
> state/control path.** Carry that pair forward BY OWNERSHIP to the exact bodyless
> Tail predecessor, consume it ONCE when emitting the existing active jump, and
> **emit `returned` as the jump's first argument.** Reuse the existing header
> parameter.
>
> **Add no runtime word, header parameter, ABI field, heap/stack receipt, side
> table, lookup, search, capture write, or fallback.**
>
> **The transfer must be STRUCTURAL, not an optional compiler receipt.** Encode it
> in the typed source-machine transition/continuation path so the value and the
> route CANNOT SEPARATE, and so one-shot consumption follows from MOVING the
> state. Do not store a `Cranelift Value` in function-local optional state, do not
> recover it later by liveness/number/proximity, and do not promote the
> control-only `answer_route`. **The planner route authorizes identities; it does
> not manufacture the runtime value.**
>
> **Replace the Tail variant; do not extend it.** `DirectInvocationReturn` remains
> unchanged. The withdrawn `TailResumedRetInput` source claim is REMOVED, not
> retained beside the repair. Its replacement identifies the actual declared-call
> result producer and the exact active predecessor/header/Ret/binder/capture/body
> route. The static relation contains identities; the emitted typed transition
> carries the actual value. **There is ONE Tail authority, not two.**
>
> **The seed stays semantically real and negatively controlled.** It remains the
> initial `CheckedIhCapturedEnvironment` residual used to install the checked
> invocation — but it is NEVER the governed Tail result and never the active jump
> argument after the checked computation returns.
>
> **No Decision object is required**; the disposition follows from the exact
> emitted dataflow plus the bounded prior-art negative. **The HS12 Research
> trigger is DISCHARGED — if this chain reaches HS15, the next mechanical trigger
> fires there.**
>
> **D3 REMAINS FROZEN and QA REMAINS UNROUTED.** The Steward owns this recut, the
> inventory append, and **a NEW explicit release after the recut lands. Landing a
> frame alone still releases nothing.**

> # SUPERSEDED — HARD STOP 11, `evt_79trx05xee0dj` (2026-08-28)
>
> **Superseded by the HS12 banner above.** History; cite by event id, never by
> number.
> Bound object `24c14f4dacbcdf6789952f7a9d3f75155b310e64`, tree
> `36abd930d5acd2a8ab84141f422bd7bc795a5074`, frame blob
> `492235777030f4f12083f0de883efe008e1aa0af`, spec-42 blob
> `69b9d6d267ba20235f42972865c2b20504531d62`. Exact relevant blobs: `source.rs`
> `88fcc401b0e078f78298a0998d09364b22e64a27`, `core.rs`
> `68f9394ce4d75f68bcfbaaeff7b294040a4fd50b`, `lowering/mod.rs`
> `2ee945bc07c2facbbe016b505f8a8ab449862c44`, route planner `aggregates.rs`
> `9eb2c118e227c3a7db2849e03046db02d93a48eb`. No candidate; QA unrouted.
>
> **HS11 IS A VALID CLEAN STOP, AND THE SHARP CONCLUSION IS NOT THE ONE THE STOP
> REACHED FOR. The landed Tail route CERTIFIES THE CARRIED SEED AS THOUGH IT WERE
> FRESH `R2`. Its endpoint pairing is real; it is a pairing of the WRONG VALUE.**
>
> **The code makes the causal error exact — read it, do not re-argue it.**
>
> 1. Tail application installs the invocation but returns the initial carried
>    residual UNCHANGED at `source.rs:4512-4515`:
>    `RoutedAnswer::direct(LoweringOperand::Carried(word))`. **That word is the
>    seed, not a completed result.**
> 2. The supposed Tail SOURCE-RESULT observation is recorded later at
>    `source.rs:2148-2150` from that SAME `word.word`, immediately before
>    `lower_carried_computational_match`. **Nothing at that site produced a fresh
>    result; the observer renamed its input.**
> 3. Active self-resumption records and jumps that same `scrutinee.word` into the
>    header at `core.rs:12218-12233`.
> 4. The checked-answer fallback binds the SAME scrutinee directly into the Ret
>    case environment at `core.rs:12615-12621` and records it as the Ret input at
>    `:12636-12640`.
>
> **⇒ The HS9 causal control proved exact SSA identity from seed to sink.**
> `CoEmissionOnly` correctly showed this was not mere co-emission — **but exact
> identity does not turn a seed into a result.** The route type states the
> boundary itself at `aggregates.rs:373-375`: it contains no result value, no
> emission-local value number, and no runtime carrier. The LLDB `0x0e09`
> observation on the final unit frame is the DYNAMIC confirmation of that STATIC
> read.
>
> **`0x1109` and `0x1209` are separately live and are NOT authorized candidates.**
> Shape and liveness do not establish which word is `R2`; selecting either is the
> forbidden result search. Do not consume them by observation.
>
> **The compiler-only `answer_route` preservation experiment CANNOT repair this.**
> It propagates control metadata through `resume_active_continuation` and reaches
> frame 301, but it carries NO value provenance and leaves the same seed in the
> Ret input and the final capture. **Promoting it would create an unframed receipt
> while preserving the defect.**
>
> **DISPOSITION.**
>
> 1. **Atomic D3A+D3B is FROZEN AGAIN.** The fresh release `evt_98vzwa6e9qv1` is
>    SPENT by this clean stop. No code candidate and no QA from HS11.
> 2. **`TailResumedRetInput` is WITHDRAWN as fresh-result VALUE authority.** Its
>    destination topology, active-header identity, and Ret binder identity remain
>    useful FACTS; its current SOURCE-VALUE claim does not. `DirectInvocationReturn`
>    remains valid for its one-row population. **The HS10 statement that no
>    corrected predecessor is needed is FALSIFIED for Tail** by the exact producer
>    read above.
> 3. **Do NOT add a sibling authority.** The existing fresh-result-route
>    predecessor is recut by **REPLACEMENT**: preserve the Direct variant; a Tail
>    variant may exist ONLY if its source endpoint is the actual result-producing
>    operation AFTER the checked computation returns. **The initial
>    `CheckedIhCapturedEnvironment` word must be an explicit NEGATIVE CONTROL,
>    never the source.**
> 4. **The corrective predecessor starts with D0 ONLY** — see
>    [[RT-CHECKED-IH-TAIL-RESULT-PRODUCER-ROUTE]].
> 5. **HS12 is the stop if that D0 cannot name an existing compiler-visible
>    producer/value edge** without a persistent receipt, runtime lane, direct
>    capture write, second lookup, result search, clone/stack, ABI carrier, or
>    target synthesis. **Before any Architect ruling on HS12 the Architect will
>    mechanically hold and call Research with the exact new fork. NO RESEARCH CALL
>    IS DUE AT HS11** — do not manufacture one, and do not carry this sentence
>    forward as though it applied to the next stop.
> 6. **D3 stays frozen until the corrected predecessor lands through fresh
>    exact-object gates AND the Steward issues ANOTHER explicit release.** Add no
>    fallback. Landing alone releases nothing; this ruling alone releases nothing.
>
> **The shared predicate is UNCHANGED from HS9: a static or local endpoint treated
> as a complete directed dynamic value edge.** Eleven stops, one predicate.
>
> No Decision object is required; this follows deductively from the exact source
> path and the runtime value observation.

> # SUPERSEDED — HARD STOP 10, `evt_1ckwtvwe23e3e` (2026-08-28)
>
> **HISTORY. Superseded by the HS11 banner above**, which falsifies its claim that
> no corrected predecessor is needed for Tail. Its route-variant partition (Direct
> vs Tail) and its correction of the uniform-D3A-recipe defect both STAND; what
> does not stand is treating the Tail route's certified source endpoint as a fresh
> result value. Cite by event id, never by number.
> Bound object `da95daadf`, tree `94d9177eee14`, frame blob `531039677cb9`,
> spec-42 blob `69b9d6d267ba`.
>
> **HS10 WAS A VALID CLEAN STOP — ON THIS FRAME'S DEFECT, NOT ON A MISSING
> COMPONENT.** The stop was correct and the D0 measurements are accepted: four
> real Tail rows plus one body-refined Direct row; every real Tail callee has
> `recursive_unit_body=None`; `source.rs:4512-4515` hands the carried residual
> word onward; declared-target inventories are multi-valued; replaying the old
> direct-call D3A experiment leaves both `ResourceBodyResult` products unchanged.
>
> **BUT THE INFERENCE FROM THEM WAS WRONG. `recursive_unit_body=None` means TAIL
> VARIANT, not NO K APPLICATION.** The landed route constructor makes that
> partition explicitly: at `aggregates.rs:5508-5578` a body-refined invocation
> transport yields `DirectInvocationReturn`, and ABSENCE of that direct transport
> yields `TailResumedRetInput`. The Tail validator (`:5765-5805`) then requires
> the exact zero-argument checked invocation/call/callee, selected recursive case,
> active governed frame, `ActiveSelfResumption`, `CheckedSelectedRecursor`, and
> forward Ret-input edge. **That IS the authorized application protocol, and it
> deliberately has NO declared target to select.**
>
> **LOWERING ALREADY EXECUTES IT.** The governed call reaches the exact
> `ComputationalRecursorClosure`; `mint_checked_computational_ih_instance`
> consumes its checked marker; `install_recursor_invocation` installs the
> already-checked semantic frames (`source.rs:4424-4469`, `:4912-4984`);
> `ApplyRecursorSelection` raises the route from the exact checked selecting layer
> (`source.rs:1528-1567`, `mod.rs:10015+`). **That is the Tail `K` application.
> It is not a declared-unit call.**
>
> **The carried word at `source.rs:4512-4515` is the SEED/INPUT to that installed
> continuation, never the completed application result.** Treating the local step
> as the whole application repeats HS8's distinction exactly: no local result is
> not no eventual dynamic result. Normative `42 §6.2` agrees — `apply k resp`
> produces the next tree, the tail loop re-evaluates, `Ret r -> r`. **The
> unchanged final products show D3's fresh-result BINDING is still absent; they do
> NOT show the source APPLICATION is absent.**
>
> ## THE DEFECT IS THE OLD D3A RECIPE, AND IT IS THIS FRAME'S
>
> It applied ONE **Direct** mechanism to BOTH route variants: resolve
> `transport.source_call_identity()` and emit a declared target call. **That
> recipe is valid only for `DirectInvocationReturn`.** The multi-valued target
> inventories correctly prove that INVENTING a Tail target is forbidden; they do
> NOT create a new predecessor need.
>
> ⇒ **ADD NO PREDECESSOR**, and no source identity, target authority, inventory
> search, persistent receipt, second lookup, runtime carrier, clone, stack, ABI
> lane, direct capture write, or merge fallback. **The landed fused route is
> sufficient authority.** D3A is amended below to be ROUTE-VARIANT-SPECIFIC.
>
> **HS COUNT AND THE RESEARCH TRIGGER.** This is hard stop 10 chronologically;
> the HS9 research trigger is already discharged, and **the next mechanical
> research trigger remains HS12.** The shared predicate is unchanged from HS9: a
> local/static endpoint mistaken for a complete directed dynamic edge. **No
> Decision object is required** — deductive from the landed route partition,
> lowering control flow, and `42 §6.2`.
>
> **D3A+D3B IS FROZEN AGAIN and needs a FRESH EXPLICIT STEWARD RELEASE against
> the AMENDED frame blob.** Neither this ruling nor any landing authorizes a code
> turn. No QA and no candidate from the HS10 D0.

> # SUPERSEDED — HARD STOP 9, `evt_7wbxwxa74cdnr` (2026-08-28)
>
> **HISTORY. Superseded by the HS10 banner above.** Its determination of what
> fresh `R2` IS still holds and is carried forward; what it did not settle is the
> Direct/Tail variant split, which is why the D3A recipe below was uniform and
> wrong. Cite it by event id, never by number.
>
> **THE DETERMINATION.** For the loop rows, fresh `R2` is the result of the
> governed `K` application **as delivered into the Ret case's INPUT BINDER**. It
> is NOT the result of evaluating that Ret body, and therefore NOT the carried
> elimination's merge parameter. **This is determined, not a free design fork:**
> normative `spec/40-runtime/42-evaluation.md §6.2` is `Ret r -> r`, so the
> result is `r`; and the emitted order independently agrees — header input, Ret /
> checked-fallback input, case environment and capture, body evaluation, merge.
> The merge parameter is causally downstream of the capture and cannot flow
> backward to it under SSA dominance.
>
> **THE PRIOR D0 `YES` WAS AN INSTRUMENTATION ERROR; the answer is NO.**
> Co-emission of header, Ret body, merge predecessor, and merge parameter did not
> pair one dynamic result across them. **No recursive call, stack, clone, reverse
> edge, direct capture write, or new runtime lane follows from this NO.** The
> existing forward tail loop already has the right mechanism family — the proof
> object named its OUTPUT instead of its INPUT.
>
> **THE SHARED PREDICATE BEHIND ALL NINE STOPS.** Static endpoint facts have
> repeatedly been treated as a directed dynamic value-flow edge. HS6, HS7 and HS8
> each added one endpoint — availability, access, then a "producer" — and HS9
> shows the last endpoint was still on the WRONG SIDE OF THE CONSUMER.
> **Therefore do NOT add a fourth local field or another endpoint predecessor.**
> The structural closure is ONE typed, directed fresh-`R2` route whose source,
> intermediate tail edge, and sink COMPOSE.
>
> **CONTAINMENT. D3A+D3B REMAINS FROZEN.** The Architect's approval of
> `c8ddfb896`, landed as `830aa0952`, **no longer establishes predecessor
> sufficiency for D3.** Main is not behaviorally regressed — both fields are
> compile-time validation-only, the destination is discarded, the false loop arm
> drives no emitted value or block — but `CarriedLoopExitResult` is **latent
> false authority and D3 MUST NOT CONSUME IT.**
>
> **THE CORRECTED PREDECESSOR IS [[RT-CHECKED-IH-FRESH-RESULT-ROUTE]]** (this
> node now `depends_on` it), which REPLACES `CheckedIhFreshResultProducer` with a
> typed fresh-result ROUTE relation — `DirectInvocationReturn` preserved,
> `TailResumedRetInput` new. **Replace, do not extend.** It lands behaviorally
> inert.
>
> **WHAT D3 DOES AFTER IT LANDS:** apply `K` to produce `R2` and bind that `R2`
> ALONG THE CERTIFIED FORWARD ROUTE. It does NOT resume an abandoned compiler
> continuation from a merge, does NOT trace `R1`, and does NOT add a new runtime
> mechanism. The three dynamic suppression axes are unchanged and **the static
> route certificate is not a fourth axis.**
>
> **NO DECISION OBJECT IS REQUIRED** — the ruling is deductive from the spec and
> emitted causality.
>
> **THE STOP RULE:** if Runtime cannot derive the exact forward
> checked-answer-to-Ret-binder route without prohibited authority, **stop cleanly
> as HS10.** Do not fall back to the merge; do not select a new mechanism.
>
> **The Steward issues the second explicit release of this atomic D3A+D3B
> consumer only AFTER the corrected predecessor passes fresh Architect and
> Runtime QA gates and LANDS.** Landing authorizes nothing by itself.
>
> **THE PREDECESSOR HAS NOW LANDED** — `RT-CHECKED-IH-FRESH-RESULT-ROUTE` at
> `7d36d24f04678d3c9a2636fb06fd8c7aaf5dfb89`, eight paths blob-verified. The
> release itself is still owed and is NOT granted by this note.
>
> **STEWARD FRAME CORRECTION, 2026-08-28 — applied before that release, not
> after.** The Objective and the D3B phase text described the predecessor as
> supplying **TWO separable proofs** — a `K`-inheritance proof and a separate
> fresh-`R2`-destination proof. **That is the exact architecture HS9 falsified.**
> What landed is ONE fused `CheckedIhFreshResultRoute` whose source, intermediate
> tail edge, and sink COMPOSE; composition is the whole point of the replacement
> and is what the two-proof object lacked. Both passages now name the single
> route relation.
>
> **Why this mattered enough to fix before releasing.** Left uncorrected, the
> operative contract would have sent the ring looking for a projection that no
> longer exists, and the nearest thing answering to "the fresh-`R2`-destination
> proof" is `CheckedIhFreshResultDestination` — which names a DESTINATION, never
> a producer. Consuming it alone is precisely the latent false authority this
> node is forbidden to touch. **A frame that survives the ruling which
> invalidated it does not read as stale; it reads as authoritative**, which is
> what makes this class of defect expensive. The defect was the Steward's.
>
> # HARD STOP 9 TAKEN — `evt_5p5mknw26g4qq` (2026-08-28), the measurement behind the ruling
>
> **This block records the hard stop itself. The ruling above is what governs.**
>
> **HS9 fired on the atomic D3A+D3B consumer and was taken cleanly.** The
> implementer stopped on `830aa0952` with no commit, candidate, fallback, marker
> relabeling, capture write, or ABI/control repair; branch
> `wp/RT-RESULT-CONTINUATION-BINDING-PROVENANCE-d3a-d3b-r2` clean at that exact
> base, tree `41193dd086e780d5311668f30703c41f8f1c4815`.
>
> **The measured gap:** the Direct write coordinate `741/740/739` has the exact
> body-refined inherited transport and can enter the existing declared-call lane.
> All four loop producers (read `305/304/303`, `515/514/513`; write `529/528/527`
> and independently-derived `318/317/316`) have `recursive_unit_body=None` and no
> direct inherited transport at that identical lookup, so they apply only through
> active carried self-resumption — locally `RecursiveBackedge`. The only typed
> fresh producer is the eventual merge named by `CarriedLoopExitResult`, and by
> the time that merge parameter is available the governed source-call
> continuation has already unwound through the protocol marker. **The missing
> piece is a typed morphism from that eventual merge result back to the exact
> ordinary Ret/capture continuation abandoned at governed self-resumption.**
> Arrival magnitudes are diagnostics, never pins.
>
> **THE MANDATORY RESEARCH ADVISORY IS ROUTED AND IN FLIGHT** (`runtime-leader`
> `evt_1g79zjszzvbx7`; Research picked it up `evt_1jk0rrqf5xtmd`), exactly as the
> HS8 banner required — the advisory precedes any Architect ruling, and that was
> a frame-level obligation, not a judgment call. **No candidate, no QA, and no
> Architect ruling may be routed until it returns.** Research is advisory only
> and does not select a repair.
>
> **Every apparent repair the implementer enumerated selects a forbidden or new
> mechanism** — marker/residual/earlier `R1` as `R2`; recovering a source
> transport or call identity for loop rows; relowering or cloning the Ret
> destination after merge; retaining the source continuation until merge (an
> explicit suspended-continuation stack); carrying through an
> ABI/frame/environment/runtime lane; a direct capture write or invented
> environment-index/frame-slot morphism. **That enumeration is evidence for the
> Architect, not an authorization to pick the least-bad item on it.**
>
> # SUPERSEDED — HARD STOP 8, `evt_54efxydhb3n6w` (2026-08-27)
>
> **HISTORY, NOT THE OPERATIVE RULING.** The operative ruling is **HS13
> (`evt_59t7b49m41z8m`), in the banner at the TOP of this file**; HS9, HS10,
> HS11 and HS12 are all newer than this one. Read this banner only for lineage,
> and cite nothing from it as current authority.
>
> **Disambiguation, because this file contains an earlier in-line mention of
> "HS8" inside the HS7 banner.** That older mention is NOT this ruling. The
> hard-stop-8 ruling recorded here is **`evt_54efxydhb3n6w`, cited by event id
> throughout this banner**. Cite the event id, never the number.
>
> **D3A+D3B is FROZEN AGAIN and needs a THIRD sibling proof first.** Runtime
> hard-stopped an eighth time on this mechanism chain: on four of five governed
> coordinates the exact governed `K` application exposes **NO LOCAL
> `LoweringOperand` result** at `CheckedComputationalIHInvocationReturn`.
> The active self-resumption arm jumps to the already-active loop header,
> switches the builder to an unreachable block, and returns
> `Lowered::RecursiveBackedge` — which `lowering/source.rs:1969` defines in its
> own comment as a **protocol marker, not a value**. The implementer stopped
> clean: no product edit, no commit, no fallback, no relabeling. The Architect
> independently reproduced it on exact base
> `00e66312b4ef617eb658a2e75db9f99ff2c56492` / tree
> `e286949f8fe5053e4719e54d0cc66adbe073dcdf`.
>
> **That is an absence of a LOCAL result, NOT a proof that no fresh dynamic
> result is eventually produced.** The same carried elimination owns a one-word
> merge block, and the active re-entry is tail control — a later `Ret` /
> checked-answer exit may reach that owning merge even though the abandoned
> local compiler continuation sees only the marker. Whether it does is exactly
> what the predecessor's mandatory D0 measures. Do not read this banner as
> settling it.
>
> **The missing piece is a COMPONENT BOUNDARY, not one more D3B field.** The
> landed architecture has two sibling proofs — exact inherited `K` authority and
> its governed application coordinate, and the exact fresh-result DESTINATION. It
> lacks the third: **which emitted control edge PRODUCES the fresh dynamic
> result.** `CheckedIhFreshResultDestination` names a destination, never a
> producer. Building the producer inside this atomic consumer would make lowering
> invent the very edge whose authority is absent, repeating this chain's
> decomposition failure.
>
> **The replacement predecessor is
> [[RT-CHECKED-IH-SELF-RESUMPTION-RESULT-PROVENANCE]]** (this node now
> `depends_on` it), owner runtime, tier T1, size M, and it lands BEHAVIORALLY
> INERT — it neither applies `K` nor binds `R2`.
>
> **NO MECHANISM IS AUTHORIZED YET.** The ruling deliberately declines to open
> the recursive-call / frame-morphism / explicit-continuation fork:
> `spec/40-runtime/42-evaluation.md §6.2` specifies the ITree driver as
> tail-resumptive and realizable as a loop without a suspended-resumption stack,
> so the first candidate producer to MEASURE is the existing carried-loop exit. A
> mandatory D0 in the predecessor measures it; a D0 answering NO is a SUCCESS
> that stops and returns coordinates to the Architect, and picking a mechanism on
> a NO is an unauthorized design decision.
>
> **What is UNCHANGED and NOT reopened:** the atomic D3A+D3B contract (no
> application-only checkpoint); the THREE independent suppression axes
> (inheritance, application, fresh-result binding) — **the producer proof does NOT
> become a fourth axis**; the deletion of the transitive `R1 -> capture`
> requirement; the landed generated-entry predecessor and the result-successor
> destination proof, both still valid.
>
> **Re-release is required and is the Steward's.** After the predecessor passes
> its own Architect/QA gates and LANDS, the Steward issues a SECOND EXPLICIT
> RELEASE of this same atomic D3A+D3B consumer, now consuming all THREE sibling
> proofs. Neither the predecessor's frame landing nor its merge authorizes this
> consumer. No D3 candidate and no QA until then.
>
> **Next stop is 9 and it MECHANICALLY triggers the mandatory Research advisory
> BEFORE any Architect ruling — even if it occurs during the predecessor's D0.**
> That is a frame-level obligation, not a judgment call for the Steward or the
> ring.

> # HARD STOP 7 — the DECOMPOSITION was the defect (Architect evt_1z1p9t4tdyd2v)
>
> Runtime hard-stopped a third time, on the landed locator base `b76943684`
> (runtime-implementer `evt_dqa3989tfmm2`; leader routing `evt_7fhdmyaqk4371`).
> The locator makes `K` immediately indexable ONLY AFTER
> `checked_ih_continuation_inheritance_for_invocation` selects the inheritance
> view, and that accessor requires the earlier transport's full
> `ContinuationCallIdentity` — which the generated descendant re-entry boundary
> does not carry. A compile-preserving path-local propagation experiment
> validated `K` and then showed the final application is never reached; it was
> fully reverted, leaving the consumer branch clean with no candidate.
>
> **The Architect ruled that the repeated last-gap decomposition is ITSELF the
> design defect.** Three consecutive planner-owned predecessors — HS5's
> result-successor relation, HS6's K-availability locator, and the proposed
> source-call-identity half — are all components of ONE accessor, each scoped to
> the last observed gap, each landing and revealing the next. **Do NOT frame a
> fourth narrow predecessor.** Frame ONE complete generated-descendant-entry
> access predecessor, specified from the CONSUMER'S ACTUAL ENTRY CONDITIONS: a
> closed `CheckedIhGeneratedEntryAccess` relation (name provisional) joining the
> emitted generated-context population to the continuation-inheritance
> population, re-derived and validated for totality and exact bijection,
> installed in `define_continuation_context_bodies` as COMPILE-TIME
> function-definition authority, and consumed at the governed descendant
> invocation through the EXISTING full accessor. The carrier is NEVER a runtime
> parameter — not ABI, frame header, environment binding, carried value,
> `RoutedAnswer`, `SourceControl` member, or runtime tag. The ruling's six
> numbered requirements, its required controls, and its forbidden list are
> authoritative; read `evt_1z1p9t4tdyd2v` in full before framing.
>
> Per the hard-stop-chain rule the Steward triggered one bounded Research
> advisory before freezing the replacement frame (`evt_6ysq39xjpbjk4`): Q1 the
> generated-context join's functional/non-functional census — whether
> `(ContinuationContextId, final invocation, callee)` maps to more than one
> `ContinuationCallIdentity`, which decides whether the predecessor attaches one
> validated row or must refine the existing `intern_generated_contexts` key and
> statically retarget callers; Q2 prior-art failure modes for compile-time
> context splitting. The advisory may sharpen the row/key controls; it may not
> reopen the authority boundary or propose a runtime carrier.
>
> **The replacement predecessor is [[RT-CHECKED-IH-GENERATED-ENTRY-ACCESS]]**
> (this node now `depends_on` it), owner runtime, tier T1, size L.
>
> **HS8 SUPERSEDED WHAT THIS BANNER USED TO SAY — do not act on an older
> revision of it.** It recorded Q1 = NON-FUNCTIONAL and named ruling item 5's
> SPLIT arm as the required positive path: refine the `intern_generated_contexts`
> key by source-call identity and statically retarget callers. **That contract is
> FALSIFIED** (runtime-implementer `evt_799ckhz4kgtcd`; Architect
> `evt_14hwxs5q2j087`, independently reproduced at exact `1403722a2`). The Q1
> census was correct in the destination-inheritance frame, but the retarget must
> execute in the CALLER frame, and `ContinuationContextId(0)`'s incoming set
> there is `{call 4}` — not `{A, B}`. The two frames share no discriminator.
>
> **The predecessor is RECUT onto the confluence/quotient branch** (Architect
> mechanism ruling `evt_3myrd8sp2tp8n`, on Research graph advisory
> `evt_2ffs47ax5gbnn`). The sole colliding pair W0/W1 AGREES on the
> consumer-relevant projection by full typed `Eq` — same destination owner/body,
> binding, `CheckedIhImmediateKBindingLocator`, and
> `CheckedIhFreshResultDestination` — so a planner-validated confluence
> certificate proves common typed consumer authority at one seat, and lowering
> receives a SANITIZED projection carrying no source identity at all. **Static
> splitting and region cloning are WITHDRAWN.** `intern_generated_contexts` and
> its callers are UNCHANGED.
>
> **Nobody has measured whether static cloning is available** — the advisory's
> item 3 had a false precondition and correctly declined to classify it. Do not
> record it as ruled out.
>
> The advisory's reach boundary is carried verbatim in that frame and must not be
> rounded into a global claim.
>
> **D3A stays frozen and non-landable. The atomic D3A+D3B contract and the three
> independent suppression axes are UNCHANGED.** After the complete predecessor
> lands through ordinary gates, the Steward must EXPLICITLY re-release this same
> atomic consumer; landing alone authorizes nothing.

> # HARD STOP 6 — held on a second predecessor extension (Architect evt_bqyqcvn0ng1d)
>
> Runtime hard-stopped building the D3A+D3B consumer: the landed predecessor
> [[RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR]] proves WHICH `K` survives and WHICH
> invocation uses it, but exposes no locator into the `env` that
> `RuntimeExpr::Var(index)` reads — semantic identity without immediate
> availability. Two consumer guesses (source recursive slot; final recursor
> residual) each returned runtime failure -1; the consumer branch reverted clean
> at `c1945c6fb`. HS6 is this chain's designated sustained-Research trigger
> (Research advisory `evt_1t84ypm156mqh`). The Architect HS6 component ruling
> `evt_bqyqcvn0ng1d` (advisory incorporated) rules the closure is a planner-owned
> predecessor EXTENSION — the immediate K-availability locator — framed as the
> independently-landable [[RT-CHECKED-IH-K-AVAILABILITY-LOCATOR]] (this node now
> `depends_on` it). D3A stays frozen/non-landable and the consumer branch stays
> held clean until the locator lands, then the ATOMIC D3A+D3B consumer is
> explicitly RE-RELEASED against the extended base and obtains `K` through the
> existing accessor. The three suppression axes and the atomic contract are
> UNCHANGED; HS6 is predecessor incompleteness, not a new consumer axis.
>
> # HARD STOP 3 DISCHARGED 2026-08-26 — option (c), D2/D3 phase separation (Architect evt_1hren6zm8mgxv)
>
> The Architect accepted Research advisory `evt_4cbecpkg2e0gs`: there is no
> principled unmodified same-path positive for an application ABSENT from the
> only governed branch. The prior demand for a fresh pre-repair application
> positive is WITHDRAWN as impossible. Phase separation is the structural
> closure. This discharges hard stop 3; the shared predicate across all three
> stops is recorded (a downstream semantic result claimed before the
> graph-authorized predecessor operation that produces it). Durable inventory
> fold: `7e5d54b9839451d8d31d76070934af84516e7cf8` over current main.
>
> **D2 disposition — localization ACCEPTED as EVIDENCE ONLY, not a merge
> candidate.** At evidence object `ac1ebdacb8fefa79e264656c029c84fb6a69a69d`,
> `call_checked_ih_transport_from_case_environment` classifies the selected
> binding at `core.rs:7699`: the `StaticWorker` arm continues through
> capture/envelope assembly to `call_declared_unit_target` (`:7840-7846`); the
> `Value(Carried(word))` arm settles `InlineNoCall` and returns the word
> (`:7701-7713`) — NO application instruction exists on that CFG arm. The
> complete test population takes zero calls to either arm; the admitted
> read/write programs reach ONLY the carried arm with exact planner descriptors
> read `608`/`662`/`939`/spec 1 and write `720`/`1238`/`1257`/spec 3. Accepted
> classification: the expected `ResourceBodyResult` was never minted because the
> source continuation was not applied **(HISTORY — this global claim is
> DIRECT-SCOPED by HS12; Tail's application and producer both exist and the
> missing fact is delivery)**; slot 1 faithfully carries a planner-typed
> `CheckedIhCapturedEnvironment` (not a result) through the correct
> parameter-plus-seven-capture mapping. **`ac1ebdacb` stays FROZEN evidence; QA
> is NOT requested on it. No fresh pre-repair positive object is required.** The
> committed one-case Direct positive
> `d2_checked_ret_result_reaches_the_exact_continuation_capture` remains VOID —
> it must not be credited or merged under its present name/prose.
>
> **Frame correction (this recut) — three changes the ruling required.**
> (1) AC-D2-4 is REPLACED with exact natural reachability (below): D2 measures
> only that the unchanged admitted programs reach the carried branch with their
> exact identities, emit no application there, the typed result is absent, and
> each reaches its exact fail-closed default; an entry-marker/refusal mutation
> may prove reachability but must NOT apply the worker or inject a result.
> (2) The same-path application positive and the application-removal mutation
> MOVE to D3. (3) The old instruction that D3 waits for a corrected pre-repair
> positive is DELETED — D3 waits for THIS recut to land and a fresh Runtime
> release. **Runtime is HELD until this recut lands and is re-released.**
>
> Process note (Architect): the next Research trigger on this chain is hard
> stop 6 — hard stops 4 and 5 do not re-consult Research.
>
> ## Lineage (compact)
> D1 [[RT-ITREE-DEFAULT-SELECTION-PROVENANCE]] route-transport slice landed at
> origin/main `21d62130` (PR #2948), advancing the admitted programs
> monotonically to this later fail-closed boundary. This node is the D2/D3 half
> of the RT-ITREE hard-stop-2 split (Architect evt_5w03f4zbg02ry).

> # HARD STOP 4 DISCHARGED 2026-08-26 — D3 split into coupled D3A + D3B (Architect evt_6mnawfvm8fc4j)
>
> HS4 accepted. Exact WIP `7199330550f9eae611b417c30b289722cd8057b1` (tree
> `9f838714182f6a2b837b5819fe6b194adc1e569a`, base `6f00843de`) stays EVIDENCE
> ONLY — no QA, no merge, no further capture/result rewrite. The application
> repair is REAL but NOT SUFFICIENT. The Architect independently reproduced the
> read product (`1 passed`, `ResourceBodyResult` frontier) and traced the first
> governed application (spec 2, source record 608, SSA `v1629`) whose source
> continuation is `CheckedComputationalIHInvocationReturn -> ConstructArgument
> (origin 476) -> TerminalResumeOuter -> Computational (origin 301, Ret binders=1
> recursive=[], Vis binders=2 recursive=[1])`. The call result first becomes a
> NEW ITree node and CROSSES an active recursive computation; it is not an
> immediate consumer of closure 460's capture 0. Closure 460 (under
> `460 -> 465 child 1 -> 301 child 1`, inside origin 301's Ret case) has capture 0
> = source occurrence 459, plain `Var(0)`, which `build_checked_ih_bindings`
> classifies `None` — correctly the ordinary Ret payload, NOT an IH binder. Body
> 452 reads `Var(1)` and requires that capture to be the `ResourceBodyResult`; in
> the WIP CFG capture 0 instead receives later SSA `v1650`/`v1673`, neither the
> application result `v1629` nor the constructed `v1637`.
>
> **Read-side coordinates only (frame fixed-input correction evt_10rgb8n31c5sj).**
> The disposable probe established origins 301 / 460 / 459 / 452 for the READ
> program only. They are read-side EVIDENCE coordinates, not write authority. The
> WRITE program's active frame, eventual Ret payload, ordinary capture
> occurrence, and body read MUST be derived INDEPENDENTLY from its OWN existing
> graph/planner facts before any repair — never reused from, or inferred by
> similarity to, the read coordinates. If the write analogue cannot be derived
> from existing planner relations, hard-stop (HS5).
>
> **The frame's causal sentence — "returning the declared call result makes it
> flow into closure 460 capture 0" — is FALSE and is WITHDRAWN.** The first next
> predecessor operation is `TerminalResumeOuter -> resume_active_continuation` on
> the exact active computational frame, NOT closure-capture insertion. A direct
> capture-0 replacement would skip the intervening ITree semantics and is
> PROHIBITED. Treating capture `Var(0)` as an IH from its numeric index would
> contradict the planner derivation.
>
> **D2 narrow-form correction.** D2 remains valid ONLY in its narrow form: the
> carried arm itself emitted no application and returned
> `CheckedIhCapturedEnvironment`. Its stronger causal gloss — that this omission
> ALONE explains the final closure capture/default — is WITHDRAWN. The WIP proves
> application FEASIBILITY through the ruled transport/projection/envelope/
> single-call, but NOT result delivery, and must NOT land alone: an
> applied-but-unconsumed call is not semantics-preserving, and the two reported
> executions of one continuation target are not yet paired to distinct governed
> arrivals.
>
> **Recut (this edit).** The single D3 application leap is replaced by two
> COUPLED phases: D3A (exact carried application, per-arrival paired, evidence
> only — no final-capture/`InvalidOffset` claim) and D3B (localize then repair
> the FIRST graph-authorized edge where the applied result fails to reach the
> eventual `ITree::Ret` payload / closure capture). The final merge is ATOMIC:
> application + graph-authorized result flow + product controls; no
> application-only checkpoint. Runtime stays HELD until this recut lands and is
> re-released. Next Research trigger remains hard stop 6. Inventory fold updated
> to `529f21c43e1c0c5257d2f7898481aaa3dc3a0429` (entries 1-4).

> # HARD STOP 5 DISCHARGED 2026-08-26 — the missing edge is UPSTREAM in the planner (Architect evt_494k61s04fnv9)
>
> HS5 accepted. Evidence WIP `4e516e54712a47cf14c47b7abf2840f943071af9` (tree
> `9f7ac95f038bfb69bd6a881ec14133957e569078`, corrected base `14040ecae`) stays
> EVIDENCE ONLY — no QA, no merge, no further lowering-side repair. Frozen HS4
> `7199330550` unchanged. Hard-stop count is 5; Research is not triggered until
> hard stop 6.
>
> **D3B localization is VALID and independently closed the fixed-input
> correction.** Read paired through frame 301 / closure 460 / capture 459 / body
> 452; write INDEPENDENTLY paired through frame 314 / closure 473 / capture 472 /
> body 465 — not a reused coordinate. Each path reaches its own exact
> active-self-resumption and then an ordinary ZERO-argument checked invocation for
> which no planner-issued successor exists
> (`checked_ih_environment_transport_for_invocation` returns `None` at both
> `(Spec(2), cont 301, pos 1)` and `(Spec(5), cont 314, pos 1)`;
> `recursive_unit_body=None`). The predeclared boundary-closure records (241/352)
> establish code/capture layouts but do NOT prove a dynamic-environment crossing
> and do NOT turn the zero-argument invocation into the later one-parameter
> closure call.
>
> **HISTORY ONLY — the predecessor-ownership model below is SUPERSEDED by HS12
> (`evt_7a6pp8n24r1ms`). It is retained to record what HS5 concluded and why, and
> it is NOT a live instruction.** The landed successor contributes retained
> topology/identity facts only; no predecessor owns the Tail relation, this
> candidate supplies it, and no further predecessor or predecessor-node hard stop
> is authorized. **The live contract is HS13 (`evt_59t7b49m41z8m`) — this node
> is D0-ONLY and D3A/D3B are NOT authorized.** The HS12 UPDATE in Deliverables
> (D3B) survives only as SEMANTIC PROPERTY and history: its mechanism locus is
> refuted and it is not a recipe to build.
>
> **Lowering has reached the end of its authority.** The next component is an
> UPSTREAM planner-owned checked-IH result-successor relation — a genuine
> predecessor, not a lowering fallback and not another continuation-identity lane.
> It is framed as the independently-landable, behaviorally-inert predecessor
> [[RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR]] (this node now `depends_on` it). D3A
> stays FROZEN/non-landable while that predecessor is built; after it lands, the
> D3 branch rebases and builds the ATOMIC D3A+D3B consumer, whose D3B consumes
> ONLY that exact successor projection through the existing shared D3A call lane
> and ordinary active-continuation semantics. All existing D3A/D3B controls remain
> mandatory. The five symptoms share the recorded predicate: a downstream semantic
> result claimed before the graph-authorized predecessor operation that produces
> or pairs it — the structural closure now belongs in the planner relation, not a
> sixth lowering exception. Inventory fold `244b2468afd4f0cd06837fd3079f291d7d330af5`
> (entry 5). Runtime stays HELD until the planner predecessor is framed, landed,
> and the successor D3A+D3B work is explicitly re-released.

> # CONTINUATION-INHERITANCE RECONCILE 2026-08-26 (Architect incorporation evt_2prk31prke9cc)
>
> Architect incorporation `evt_2prk31prke9cc` (accepting Research advisory
> `evt_261gm8y54xttt`), grounded on `origin/main@a09878026`. The advisory is
> INCORPORATED; it is NOT hard stop 6 and the count remains FIVE (no new
> implementer hard stop produced it). This supersedes the earlier standing note
> that hard stops 4 and 5 do not consult Research.
>
> **The prior operative reading conflated two semantic edges.** The Objective,
> D3B Deliverables, `AC-D3B-RESULTFLOW`, its pairing, the suppressions, the
> reviewer text, and the Sequencing paired the D3A application result forward
> through the source-control chain and the active recursive computation into the
> later Ret capture. A forward source path proves CONTROL and AVAILABILITY, not
> dynamic VALUE identity. This recut deletes that transitive requirement from the
> operative text and reconciles to the continuation-inheritance reading:
>
> - **`K`** — the exact captured continuation environment / call capability
>   authorized by one existing `CheckedIhEnvironmentTransport`; **(HISTORY: this
>   Direct-shaped definition is superseded by the LIVE route-specific `K` in
>   Phase structure — HS12. Do not read it as the live definition.)**
> - **`R1`** — the earlier D3A result that constructs / resumes the next ITree
>   computation;
> - **`R2`** — the FRESH result produced when `K` is applied at the recursively
>   exposed zero-argument checked invocation.
>
> **D3A** applies the inherited `K` at each exact recursive arrival and yields
> fresh `R2`. **D3B** binds THAT `R2` through ordinary Ret-case /
> `CheckedCaseBinderLayout` / lexical-capture semantics to the exact closure
> capture and the `InvalidOffset` product — it does NOT trace `R1` forward, does
> NOT assert `R1` reaches the capture, and does NOT re-derive the relation in
> lowering. Both D3A's application site and D3B's fresh-result destination consume
> ONLY the predecessor [[RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR]]'s
> continuation-inheritance projection (the `K`-inheritance proof and the
> fresh-`R2`-destination proof, exposed separately). Suppression and at-most-once
> controls are now THREE independent axes — inheritance, application, and
> fresh-result binding — with no scalar totals. Runtime stays HELD until the
> predecessor recut lands and this D3A+D3B work is explicitly re-released; QA and
> the Architect review the exact atomic candidate.

## Symptom inventory

Append one line per hard stop; never rewrite history.

1. Forcing the localized outer ITree carried match through an origin-only
   checked-return bypass progressed into value-deduplicated
   `ResourceBodyResult` defaults instead of `InvalidOffset` — keyed on an
   artificial predecessor-route bypass, not a planner-authorized route.
2. After D1's route repair the admitted programs naturally terminate at the
   ordinary `ResourceBodyResult` match, but the expected result is absent from
   the entire eight-entry receiving environment — keyed on a producer-to-binding
   chain that never places the source continuation's result into environment
   slot 1.
3. The pre-repair localization phase's required AC-D2-4 positive must apply the
   exact source continuation at the carried seam, but the complete runtime-test
   census reaches neither arm and the admitted read program reaches only the
   carried early-return arm — keyed on requiring the repaired operation as its
   own pre-repair control.
4. Exact WIP `7199330550f9eae611b417c30b289722cd8057b1` makes the governed
   carried call execute and return a new value, but source control then runs
   `CheckedComputationalIHInvocationReturn -> ConstructArgument(476) ->
   TerminalResumeOuter -> Computational(301)`; the later Ret-case closure 460
   still captures the prior transported environment at capture 0, and final
   `Var(1)` reaches the same default — keyed on claiming a call result before
   the intervening recursive ITree computation binds it to the later capture.
   (Origins 301/460/459/452 here are READ-side evidence coordinates only, from
   the disposable read-program probe; the write analogue is derived
   independently — frame fixed-input correction evt_10rgb8n31c5sj.)
5. Exact corrected-base WIP `4e516e54712a47cf14c47b7abf2840f943071af9`
   independently pairs the D3A result through construction, `ResumeOuter`, the
   active-frame header, the recursive child, and self-resumption for both
   programs. Read then reaches the exact zero-argument checked invocation at
   frame 301/position 1; write independently reaches frame 314/position 1. Both
   have no planner-issued checked-IH result-successor relation, while the
   separately existing boundary-closure records do not supply that missing
   successor or turn the zero-argument invocation into the later one-parameter
   closure call — keyed on asking lowering to consume a result-flow edge the
   planner never issued.
6. A result-successor identity existed without an immediate `K` environment
   locator.
7. The locator required a source-call identity that is absent at generated entry.
8. The local Tail return was `RecursiveBackedge`, a protocol marker, not a value.
9. `CarriedLoopExitResult` named the Ret-body OUTPUT rather than the Ret INPUT.
10. One Direct declared-call recipe was incorrectly imposed on Tail.
11. The fused Tail route paired the INITIAL SEED to the Ret input and named that
    pair fresh-result flow — keyed on `source.rs:4512-4515` returning the carried
    residual unchanged while `source.rs:2148-2150` records that same `word.word`
    as a source result, so the observer renamed its input and the certified
    pairing is exact SSA identity from seed to sink rather than a value edge.
12. A real declared-call result producer and the exact active header both exist,
    but the bodyless Tail transition discards the producer and supplies the
    initial seed as the only active-jump value — keyed on treating two endpoints
    as though tail resumption made their missing directed value edge implicit.
13. Both endpoints and the HS12-ruled transfer design are present, but the
    producer and the later route live in DIFFERENT selected-continuation
    lifetimes, and an intervening Rust return boundary erases the result-bearing
    type before the consumer route exists — keyed on
    `lower_computational_match_value_composed` and the adjacent source/active
    return seams reducing `RoutedAnswer` to a bare `LoweringOperand` between the
    producer at cursor N and the freshly minted selected continuation at N+2, so
    there is no pair at the producer to couple and no value left to carry by
    ownership.

> **ENTRY 13 IS NOT A THIRTEENTH ENDPOINT GAP. THE ENDPOINT-ADDITION PREDICATE
> STAYS TERMINAL AT ENTRY 12.** Entries 1-12 are one class — a static or local
> endpoint read as a complete directed dynamic value edge. Entry 13 is a
> LIFETIME/ORDERING class: nothing is missing at either endpoint, and there is
> nothing left to add an endpoint to. **Do not stretch the 1-12 predicate to
> cover it, and do not answer it by adding another endpoint** — that is the move
> the first twelve stops exhausted.

> **ENTRIES 5 AND 11 ARE THE STALE-ENTRY LESSON, NOT JUST TWO STOPS.** The
> inventory was left stale at entry 5 while stops 6-10 accumulated, so the frame
> read as though the chain had stopped growing. **An inventory that lags is worse
> than an absent one: it dates itself and therefore reads as current.** Append at
> the stop, not at the ruling.
>
> **THE TWELVE ENTRIES SHARE ONE PREDICATE** (HS9, restated at HS11, and ruled
> terminal at HS12 `evt_7a6pp8n24r1ms`): a static or local endpoint treated as a
> complete directed dynamic value edge. Each stop added one more endpoint —
> availability, access, a producer, a destination, a route — and none of them
> added a VALUE. Reading the list as twelve separate gaps is what keeps producing
> the next one.
>
> **ENTRY 12 IS WHERE THE SERIES CLOSES, AND THAT IS A DIFFERENT KIND OF ENTRY.**
> Stops 1-11 each found an endpoint MISSING. Stop 12 found **both true endpoints
> PRESENT** and proved the directed edge between them absent — and the bounded
> prior-art negative (`evt_4v1wg8hb4zxtm`) established that no known technique
> manufactures that edge from the endpoints. **⇒ There is nothing left to add an
> endpoint to. The structural closure is an explicit value-bearing source-machine
> transition, and it is now IN this node rather than in front of it.**

## Objective

Deliver, as ONE atomic increment, the route-specific repair UPSTREAM of ordinary
`ResourceBodyResult` selection:

- **`DirectInvocationReturn`** — add its ruled declared call; its local return IS
  fresh `R2`.
- **Tail (`TailProducerToBackedge`)** — its governed application ALREADY EXISTS
  and is preserved. **The missing work is exact produced-result TRANSFER.**
- **D3B** — bind the delivered operand to the exact ordinary Ret closure capture
  and body read, then repair the FIRST graph-authorized edge where it fails to
  arrive, so both admitted programs green the exact `InvalidOffset`.

> ## HS13 RESTATED IN THE OBJECTIVE — **D0-ONLY**. D3A/D3B NOT AUTHORIZED.
>
> **The operative HS13 banner is at the TOP of this file** (`evt_59t7b49m41z8m`);
> this is a restatement in place, not a second authority.
>
> **Everything below describing D3A/D3B as the deliverable states the TARGET, not
> the authorized work.** The only authorized phase is the D0 return-boundary
> closure in Phase structure. **D0 lands no production and routes no QA.**
>
> **HS12's MECHANISM LOCUS IS REFUTED; its SEMANTIC PROPERTY STANDS.** The
> required directed edge is still producer result -> active jump -> Ret input,
> the seed is still the negative control, and `RecursiveBackedge` remains
> marker-only. **But "couple the operand with the existing Tail route AT THE
> EXACT PRODUCER" is IMPOSSIBLE AS WRITTEN, because the route does not exist
> there:** the declared-call result is produced in selected continuation cursor
> N, the governed Tail route becomes available only in a freshly minted selected
> continuation at N+2, and in between
> `lower_computational_match_value_composed` and the adjacent source/active
> return seams reduce `RoutedAnswer` to a plain `LoweringOperand`. No value/route
> pair exists at the producer locus, so every later bind observes `Unavailable`.
>
> **Cursor N/N+2 is DIAGNOSTIC EVIDENCE ONLY. It is never an identity key and it
> authorizes no proximity match.** Permitting `Unavailable` merely preserves the
> exact `ResourceBodyResult` default — it is not partial progress and stays
> discarded.
>
> **Appending another carrier field to `SourceSelectedContinuation` cannot repair
> a value already erased by a Rust return type.** That is why this is a return
> BOUNDARY question and not a carrier question.

> **Do NOT restate this as "the carried arm fails to apply `K` and returns the
> transported word unapplied."** That was the pre-HS12 diagnosis and it is
> WITHDRAWN for Tail. Tail's application is not missing — its produced result is
> DISCARDED at `source.rs:4478-4515`. Stating the withdrawn uniform-application
> defect here and correcting it further down is exactly how this frame kept
> re-authorizing superseded readings.

**THE APPLICATION IS ROUTE-VARIANT-SPECIFIC (HS10, `evt_1ckwtvwe23e3e`). There is
no single uniform D3A mechanism, and assuming one is what produced HS10.**
For `DirectInvocationReturn` the application is the existing body-refined
transport plus ONE declared call, whose local return IS fresh `R2`.

**FOR TAIL, THE VARIANT IS `TailProducerToBackedge` — a REPLACEMENT, and
`TailResumedRetInput` IS ABSENT (HS12, `evt_7a6pp8n24r1ms`).** The old name
survives only in the historical banners above. The conceptual name here is the
frame's; the exact spelling of the landed arm is the increment's to choose.

**State the Tail split precisely, because the imprecise version is what HS12
corrected.** The existing governed recursor application REMAINS — consume the
checked marker once, install the invocation segment once, run the existing active
self-resumption. What Tail does NOT use is Direct's
`continuation_calls[...]` lookup and Direct's extra declared call. **That is NOT
the same as "no declared call produces the value."** D0 established the actual
fresh-result producer AS a declared call: the continuation-specialization call at
`calls.rs:2022`, its Result-slot load at `:2120`, yielding
`RoutedAnswer::checked(returned)` at `source.rs:4369-4374`.

The landed `CheckedIhFreshResultRoute` (`7d36d24f0`) contributes **retained
topology and identity facts — destination topology, active-header identity, Ret
binder identity. Its Tail VALUE authority is WITHDRAWN.** This atomic candidate
REPLACES the Tail variant and supplies the one missing producer-to-backedge value
edge. **The MECHANISM for that edge is UNDETERMINED and is D0's question (HS13)
— it is NOT "a new typed source-machine transition", which HS13 refuted: the
value is erased by a Rust return type before the Tail route exists.** **No new
predecessor.** Bind
that FRESH `R2` — NOT the earlier D3A result `R1` traced forward — through
ordinary Ret-case / closure-capture semantics to the exact capture the Ret-case
closure consumes. Then repair the
FIRST graph-authorized edge where the fresh `R2` fails to reach that ordinary
capture, so the admitted read-offset and write-offset full programs green the
exact `InvalidOffset` observation. Runtime must NOT repair the default, write
closure capture 0 directly, search the environment at runtime, trace `R1`
transitively into the capture, or re-derive the inheritance relation in lowering.

## Phase structure (option (c) + HS4 split, Architect evt_1hren6zm8mgxv / evt_6mnawfvm8fc4j)

> ### LIVE VOCABULARY — `K` IS ROUTE-SPECIFIC (HS12, `evt_7a6pp8n24r1ms`)
>
> The operative Objective and ACs use `K` for BOTH variants, so it is defined
> here per route. **The single Direct-shaped definition in the 2026-08-26
> continuation-inheritance blockquote below is HISTORY** — accurate for the model
> HS5 held, and NOT the live definition.
>
> - **Direct `K`** — the captured continuation environment / call capability
>   authorized by its exact `CheckedIhEnvironmentTransport`.
> - **Tail `K`** — the EXISTING governed checked-recursor application capability,
>   identified by the exact invocation / call / callee and the active governed
>   frame, with `direct_transport=None` and NO Direct declared target. **There is
>   no transport authorizing it, and requiring one is the HS10 demand.**
> - **`R2`** is fresh on both arms. On Direct it is the added declared call's
>   local return. **On Tail it is the produced operand of the
>   continuation-specialization call, carried by the bridge — NEVER recovered
>   from `K` itself, and never the initial carried seed.**
>
> The candidate's replacement route relation selects the variant; the bridge
> carries Tail's produced `R2` separately from `K`.

- **D2 — localization. ACCEPTED as evidence (`ac1ebdacb`); NOT a merge
  candidate; NO QA. DIRECT-SCOPED (HS12).** Narrow form only: **at the carried
  CFG arm, no DIRECT declared-call application or result was emitted and the arm
  returned `CheckedIhCapturedEnvironment`** (the stronger causal gloss is
  WITHDRAWN — HS4). **It makes NO claim about Tail: HS12 proved Tail's governed
  application and its declared-call producer both EXIST, and that the missing
  fact is DELIVERY, not application.** Its ACs are the census (AC-D2-1/2/3,
  satisfied by the accepted evidence) plus the reworded natural-reachability
  AC-D2-4.
- **D3A — route-specific application and, for Tail, produced-result transfer.
  EVIDENCE ONLY; must NOT land alone.** **The mechanism is selected by THIS
  CANDIDATE'S SINGLE ROUTE RELATION, after it replaces the Tail variant — NOT by
  the landed route variant.** The landed `DirectInvocationReturn` is RETAINED and
  still selects Direct; the landed Tail variant `TailResumedRetInput` is REPLACED
  and contributes only topology/identity facts, **never selection or value
  authority.** The two arms are NOT interchangeable (HS10):
  - **`DirectInvocationReturn`** — retain the existing body-refined
    `CheckedIhEnvironmentTransport` plus ONE declared call (the shape proven
    feasible by WIP `719933055`). **Its local return IS fresh `R2`.**
  - **`TailProducerToBackedge`** (the REPLACEMENT variant; `TailResumedRetInput`
    is ABSENT — HS12 `evt_7a6pp8n24r1ms`) — the governed recursor application is
    the EXISTING exact zero-argument governed recursor call. Consume the checked
    marker ONCE, install the invocation segment ONCE, run the existing active
    self-resumption. **`recursive_unit_body=None` and `direct_transport=None` are
    REQUIRED DISCRIMINATORS, not obstacles.**
    **What Tail does NOT use is Direct's `continuation_calls[...]` lookup and
    Direct's extra declared call — that is NOT "no declared call produces the
    value".** The actual fresh-result producer IS a declared call: the
    continuation-specialization call at `calls.rs:2022` with its Result-slot load
    at `:2120`, yielding `RoutedAnswer::checked(returned)` at
    `source.rs:4369-4374`.
    **The initial `CheckedIhCapturedEnvironment` residual is NEVER `R2`.** Fresh
    `R2` is that produced operand, and it reaches the active header ONLY via the
    D3A→D3B Tail value bridge below. **There is no pre-existing certified
    active-header / Ret-input VALUE edge to flow along — HS12 proved that edge
    absent, and the bridge is what supplies it.**

  Every governed arrival is paired to its exact identity for its own variant.
  Makes NO claim about the final capture or `InvalidOffset`, and asserts NO
  identity between `R1` and `R2`.
- **D0 — RETURN-BOUNDARY CLOSURE. THIS IS THE ONLY AUTHORIZED PHASE (HS13,
  `evt_59t7b49m41z8m`). It lands NO production and routes NO QA.**

  On both unchanged admitted products, **enumerate every natural-path seam** from
  `RoutedAnswer::checked(returned)` (`source.rs:4369-4374`) to the later Tail
  route **where the Rust type shrinks to `LoweringOperand`** — including
  `lower_computational_match_value_composed` and the adjacent source/active
  return seams — **plus the COMPLETE CALLER CLOSURE of those seams.**

  Then determine whether that closure admits ONE **compile-time affine return
  typestate**:
  - **`Produced`** owns the exact operand and the exact `ContinuationCallIdentity`
    BEFORE the Tail route exists;
  - **`Routed`** can be formed ONLY by consuming `Produced`, when the exact later
    Tail route arrives AND its producer identity AGREES;
  - the active jump consumes `Routed` **ONCE**;
  - an ordinary/direct result is a **DISTINCT EXHAUSTIVE VARIANT, not an empty
    Tail state**;
  - **NO function in the measured return closure may silently extract a bare
    operand while `Produced` or `Routed` is pending.**

  **This is a Rust COMPILER-CONTROL RETURN VALUE, not a runtime carrier.** D0
  must REJECT: `Option`, `RefCell`, long-lived control/frame storage, cloning the
  operand, cursor matching, a persistent receipt, a side table, a lookup/search,
  post-emission rewrite, a runtime word, a header/ABI field, a capture write, a
  fallback, and `answer_route` promotion.

  **FAIL-CLOSED:** a Tail route receiving an ordinary result FAILS CLOSED. **No
  `Unavailable` default is accepted as Tail evidence** — it preserves the exact
  `ResourceBodyResult` default and proves nothing.

  **HS14 STOP RULE.** If the complete signature/caller closure CANNOT be bounded
  without redesigning general composed lowering outside this exact return path,
  **that is HARD STOP 14: stop cleanly and route the SCOPE BOUNDARY back to the
  Architect BEFORE selecting any mechanism.** Do not select the least-bad
  mechanism, and do not widen the closure to make an answer reachable.

- **THE TAIL VALUE TRANSFER — TARGET ONLY, MECHANISM UNDETERMINED, NOT
  AUTHORIZED UNTIL D0 ANSWERS.** Its atomicity spans result PRODUCTION and
  Ret-input DELIVERY, so it was never a D3A-only obligation. Do not read its
  placement as licence for D3B to consume the old certified edge — **that edge
  does not exist.** D3B consumes the delivered operand or it consumes nothing.
- **D3B — fresh-result binding then single-edge repair.** On both
  unchanged admitted programs, take D3A's FRESH `R2` as delivered by the bridge
  and bind it, via the Ret case's
  `CheckedCaseBinderLayout` to the exact ordinary closure-capture occurrence and
  body read (read: closure 460 / capture 459 / body 452; write: its independently
  derived analogue closure 473 / capture 472 / body 465). Identify the FIRST
  graph-authorized edge where the fresh `R2` fails to reach that ordinary capture.
  ONLY that edge may be repaired. D3B does NOT trace `R1` forward through the
  source-control chain, does NOT re-derive the inheritance relation in lowering,
  and consumes ONLY the operand the D3A→D3B bridge delivers. **The landed
  `CheckedIhFreshResultRoute` contributes RETAINED TOPOLOGY AND IDENTITY FACTS —
  destination topology, active-header identity, Ret binder identity. Its Tail
  VALUE authority is WITHDRAWN (HS12) and may not be consumed as one.**
  **It must NOT consume `CarriedLoopExitResult`** — that arm is latent false
  authority (HS9), and it names the Ret body's OUTPUT where this consumer needs
  its INPUT.
- **The MERGE is ATOMIC — and this bullet states the TARGET, not authorized
  work (HS13, `evt_59t7b49m41z8m`).** D3A application of inherited `K` + D3B
  fresh-`R2` destination binding + the D3A→D3B bridge + the product controls,
  landed together — no application-only checkpoint. **Runtime does NOT build
  this after a frame erratum lands.** The sequence is: (1) the D0
  return-boundary closure answer is completed against `AC-HS13-D0-CLOSURE` and
  `AC-HS13-D0-AFFINE`, (2) the **Architect reviews that D0 answer**, and (3) the
  Steward issues a SUBSEQUENT fresh explicit D3 release. **The immediate
  authorized release is D0-ONLY, and a frame landing authorizes nothing.** A D0
  answering NO is HS14 and no D3 release follows at all. No predecessor recut is
  outstanding; HS12 subsumed the last one.

## Evidence objects (Architect probe-verified; evidence ONLY, not candidates)

- Localization object `ac1ebdacb8fefa79e264656c029c84fb6a69a69d` — ACCEPTED as
  D2 localization evidence per the ruling. Structural split at
  `lowering/core.rs:7699-7714`: `StaticWorker` -> capture/envelope ->
  `call_declared_unit_target` (`:7840-7846`); `Value(Carried(word))` ->
  `InlineNoCall` -> returns the word (`:7701-7713`), no application. Complete
  suite: zero calls to either arm; admitted read/write reach only the carried
  arm with read `608`/`662`/`939`/spec 1, write `720`/`1238`/`1257`/spec 3.
  Durable inventory anchor `7e5d54b9839451d8d31d76070934af84516e7cf8` over
  current main. STAYS FROZEN — do not edit, do not promote, no QA.
- HS4 **DIRECT-CALL-RECIPE** feasibility object
  `7199330550f9eae611b417c30b289722cd8057b1`
  (tree `9f838714182f6a2b837b5819fe6b194adc1e569a`, base `6f00843de`) — EVIDENCE
  ONLY, do NOT merge/QA. One production path, `core.rs +124/-41`, no fallback:
  revalidates the two-endpoint transport, validates `source_record` and capture
  counts, projects ordinals via `checked_ih_capture_origin`, one envelope walk,
  one `continuation_calls[transport.source_call_identity()]` lookup, one
  `call_declared_unit_target`. LLDB proves `ken_continuation_1` executes at the
  new carried site (returned words `0x0f09`/`0x1109`), but final closure body 452
  capture 0 / `Var(1)` still reads `0x1009` (the untagged transported
  environment) — application executed, result NOT delivered. **Proves the DIRECT
  CALL RECIPE only. Its body is exclusively the Direct mechanism — the
  two-endpoint `CheckedIhEnvironmentTransport`, `source_record` validation,
  ordinal projection through `checked_ih_capture_origin`, the
  `continuation_calls[...]` lookup and `call_declared_unit_target`. It proves NO
  Tail application, producer, or bridge claim, and none of those constructs
  exists on Tail (`direct_transport=None`).** D3B (result flow) is unbuilt.
  `erasure.rs` blob `8532ced2...` unchanged; `RoutedAnswer::checked(` remains 3
  callers. STAYS FROZEN.
- HS5 D3B-localization object `4e516e54712a47cf14c47b7abf2840f943071af9` (tree
  `9f7ac95f038bfb69bd6a881ec14133957e569078`, corrected base `14040ecae`, frame
  blob `5e043db9`) — EVIDENCE ONLY, do NOT merge/QA. The mechanically-rebased
  **DIRECT-CALL** feasibility tip; its D3B localization independently derived
  BOTH read (frame
  301 / closure 460 / capture 459 / body 452) and write (frame 314 / closure 473
  / capture 472 / body 465) paths from their own planner facts and proved each
  reaches an ordinary zero-argument checked invocation with NO planner-issued
  successor (`..._for_invocation` = `None` at `(Spec(2),301,1)` and
  `(Spec(5),314,1)`). This is the object that established lowering's authority
  boundary and motivated the upstream predecessor
  [[RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR]]. STAYS FROZEN.
- Production-only parent `cc7dc7c021be67bb94f3d68de5aef8e93ffc3255` (base/current
  main `de304429c`): read naturally terminates at planned identity `36` /
  `decl:rt_parity_fs_read_at_offset_single::ResourceBodyResult`; write at `37` /
  `decl:rt_parity_fs_write_at_offset_single::ResourceBodyResult`. No
  force-origin or route bypass.
- Instrumentation object `e701eaeb972505097371761807f5dd8fa18a1522` (tree
  `d2ee1aaa8`): evidence ONLY, must NOT be promoted — its observation-trap
  pre-interning shifts diagnostic identities to 78/79, so 36/37 are correctly
  bound from the production-only parent, not the instrumented object.
- Terminal facts: the read terminal ordinary `Match` is origin `451`, owner
  `main`, path `[0,1,1,0,0,2,1,0,0,1,1,0,0]`; write is origin `464` at the same
  path. Scrutinees are origins `450`/`463` at path + final child `0`, both
  syntactic `Var(1)`. This is the ordinary `RuntimeExpr::Match` path in
  `lowering/core.rs` (`producer_route=false`); `lower_expr(Var(1))` reads env
  slot 1, obtains a `Carried` word, and calls `lower_carried_match(..., None)`.
- The complete local environment has eight entries, all `Carried`; the closed
  `env.iter()` × two-case scan returns `EnvironmentHasNoReceivingIdentity` on
  both programs — the expected result is not present in ANY of the eight slots.
  Fail-closed behavior and pre-dispatch effects are preserved: read `FsOpen ->
  BufferAllocate -> ResourceRelease(FsHandle) -> ResourceRelease(Buffer)`, no
  `FsReadAt`; write `FsOpen(source) -> FsOpen(sink) -> ResourceRelease(source)
  -> ResourceRelease(sink)`, no `FsWriteAt`, empty sink already created; both
  exit via a controlled `PatternMatchFailure`. `erasure.rs` blob `8532ced2...`
  unchanged across base/parent/object.

## Deliverables

- **D2 (localization — ACCEPTED, no new object required).** The census in
  AC-D2-1/2/3 is satisfied by the accepted evidence `ac1ebdacb`. The only
  outstanding localization AC is the reworded natural-reachability AC-D2-4,
  provable on the unchanged programs plus an entry-marker/refusal mutation at
  the carried branch that does NOT apply the worker or inject a result. NO
  repair site, NO QA on the evidence object.
- **D3A (route-specific application plus the Tail produced-result transfer —
  evidence only; Architect evt_1hren6zm8mgxv, amended HS10/HS11/HS12):**

  > **WIP `719933055` PROVED THE DIRECT CALL RECIPE ONLY.** It is not evidence
  > for the whole route-specific shape. Tail's application, its producer, and the
  > value bridge are grounded separately by HS11 (`evt_79trx05xee0dj`) and HS12
  > (`evt_7a6pp8n24r1ms`). Do not cite `719933055` for any Tail claim.

  > ### THE NEXT THREE BULLETS ARE `DirectInvocationReturn`-ONLY.
  >
  > **Tail's discriminator is `direct_transport=None`, so requiring a Direct
  > transport of it is the HS10 demand this frame exists to have removed.** Tail
  > preparation is the EXISTING governed recursor application plus the new typed
  > value bridge — nothing from these three bullets is required of it.

  - **(Direct only.)** Keep the exact `CheckedIhEnvironmentTransport` as the
    sole two-endpoint authority. In the `Carried(word)` branch, validate its
    planner record as the exact `CheckedIhCapturedEnvironment` for the
    transport's source owner and seat, and validate the runtime field count
    against the planner-declared capture count. The word is a capture vector —
    NOT code identity, NOT a semantic answer.
  - **(Direct only.)** Project capture ordinal `i` from that word with the
    existing positional carrier projection, governed by the transport's exact
    source record and `checked_ih_capture_origin`. NEVER inspect a runtime
    tag, family, spelling, body word, or field-count coincidence to choose the
    path.
  - **(Direct only.)** Assemble the existing
    `ContinuationOrdinaryEnvelopeRole` ONCE: nonrecursive fields still come
    from their ruled case-environment coordinates; `WorkerCapture` fields come
    from the exact projected carried-environment ordinals; continuation inputs
    still come from the existing transport morphism. Do NOT synthesize a
    `StaticWorkerBinding` or redirect into the neighboring `StaticWorker`
    branch.
  - **THE APPLICATION STEP, SPLIT BY ROUTE VARIANT. Amended at HS10
    (`evt_1ckwtvwe23e3e`): the single recipe below previously applied to BOTH
    variants, and that uniformity IS the defect HS10 found.** Applying the Direct
    recipe to a Tail row demands a declared target that does not exist, and the
    multi-valued inventories then make selecting one an invention.
    - **`DirectInvocationReturn` ONLY** — resolve only
      `function_local.continuation_calls[transport.source_call_identity()]`, emit
      ONE declared call through the existing call authority, record it under the
      exact transport, and pair every governed arrival to that exact
      transport/call identity. Factor the `StaticWorker` and carried-capture
      sources into one downstream envelope/call path rather than duplicating the
      continuation body or creating a second call lane.
    - **`TailProducerToBackedge` ONLY** (`TailResumedRetInput` is ABSENT) — do
      NOT resolve a source call identity, do NOT consult `continuation_calls`,
      and do NOT emit Direct's extra declared call. **That is a prohibition on
      DIRECT'S lookup and DIRECT'S extra call, NOT a claim that no declared call
      produces the value — the fresh-result producer IS the declared
      continuation-specialization call at `calls.rs:2022` (HS12).** The
      application is the EXISTING governed recursor call path already in
      lowering: the governed call reaches the exact `ComputationalRecursorClosure`
      (`source.rs:4424-4469`), `mint_checked_computational_ih_instance` consumes
      its checked marker ONCE, `install_recursor_invocation` installs the
      already-checked semantic frames ONCE (`:4912-4984`), and
      `ApplyRecursorSelection` raises the route from the exact checked selecting
      layer (`source.rs:1528-1567`, `mod.rs:10015+`). Pair every governed arrival
      to its exact invocation/call/callee triple and active governed frame.
    - **THE TAIL VALUE TRANSFER — TARGET ONLY. ITS MECHANISM IS WITHDRAWN AND
      UNDETERMINED (HS13, `evt_59t7b49m41z8m`); D0 ABOVE IS WHAT IS AUTHORIZED.**
      The application protocol above is real and it stays; what it does NOT do is
      carry a value. **The semantic target is unchanged:** the exact
      `RoutedAnswer::checked(returned)` operand (`calls.rs:2022` call, `:2120`
      Result-slot load, `source.rs:4369-4374` return) must reach the existing
      active jump (`core.rs:12218-12233`) as its first argument, reusing the
      existing header block parameter, with the Ret input following unchanged —
      and the bodyless Tail predecessor at `source.rs:4478-4515` must stop
      substituting the seed.

      > **THE HS12 RECIPE FOR REACHING IT IS REFUTED — DO NOT BUILD IT.** It said:
      > couple the operand with the existing Tail route in the source-machine
      > state/control path AT THE EXACT PRODUCER, reusing
      > `SourceMachineState::Value`, then carry the pair forward BY OWNERSHIP and
      > consume it once. **The ring built exactly that and it cannot work:** the
      > Tail route does not exist at the producer (cursor N vs a freshly minted
      > selected continuation at N+2), and the composed lowering seams reduce
      > `RoutedAnswer` to a bare `LoweringOperand` in between. **Ownership cannot
      > carry a value that a Rust return type has already erased, and appending a
      > carrier field to `SourceSelectedContinuation` does not change that.**
      > **Whether ANY mechanism exists is D0's question. Do not select one.**

      Whatever D0 finds, these remain prohibited: storing a `Cranelift Value` in
      function-local optional state, recovering it later by
      liveness/number/proximity, matching on cursor position, or promoting the
      control-only `answer_route` — the planner route authorizes identities, it
      does not manufacture the runtime value.
      **Add no runtime word, header parameter, ABI field, heap/stack receipt,
      side table, lookup, search, capture write, or fallback.**
      **REPLACE, do not extend.** The withdrawn `TailResumedRetInput` source
      claim is removed, not retained beside the repair; its replacement names the
      actual declared-call result producer and the exact active
      predecessor/header/Ret/binder/capture/body route. **One Tail authority, not
      two.** `DirectInvocationReturn` is unchanged and its blob is proved
      identical across the increment.
    - **Selection between the two is by THIS CANDIDATE'S SINGLE ROUTE RELATION,
      after it replaces the Tail variant** — the landed `DirectInvocationReturn`
      is retained and still selects Direct, while the landed Tail variant
      contributes only topology/identity facts and **never selection or value
      authority (HS12).** The landed partition at `aggregates.rs:5508-5578` is the
      shape to follow, not the authority to consume. Never select by probing a
      runtime tag, family, spelling, body word, or field-count coincidence. A row
      whose variant cannot be determined is a HARD STOP, not a default to either
      arm.

    This phase makes NO claim about the final capture or `InvalidOffset` and must
    NOT land alone.
  - **`DirectInvocationReturn` ONLY.** If the source record cannot be validated,
    its captures cannot be projected from existing planner facts, or the exact
    continuation target was not already declared into this destination function
    — HARD-STOP. **These three conditions are Direct-only and may NEVER be
    demanded of Tail: Tail is required to have `direct_transport=None` and no
    Direct declared target, so a source record, a capture projection and a
    declared target do not exist there to validate.** Requiring them of Tail
    re-creates the HS10 demand for a Direct transport.
  - **`TailProducerToBackedge` fail-closed condition — TARGET-ONLY, and
    CONDITIONAL ON WHATEVER MECHANISM D0 ESTABLISHES.** The exact produced
    operand and an agreeing later route must REACH the exact active jump and be
    consumed there EXACTLY ONCE; failing either is a HARD-STOP. **Do not state
    this as an already-formed typed producer/route pair, and do not select its
    carrier here.** HS13 (`evt_59t7b49m41z8m`) is precisely the finding that no
    such pair exists at the producer — the route does not yet exist there and the
    result-bearing type is erased in between — so a condition phrased over a
    formed pair is unsatisfiable as written and presumes the mechanism D0 is
    authorized to determine. Nothing about transport, projection, or target
    enters this condition.
  - Under EITHER arm: do NOT add a second identity catalog, ABI lane, raw cast,
    environment search, or family-specific fallback.
- **D3B (result-flow localization then single-edge repair — the coupled atomic
  half; Architect evt_6mnawfvm8fc4j):**
  - On both unchanged admitted programs, take D3A's FRESH `R2` (the result of
    applying inherited `K` at the exact recursively-exposed invocation) and,
    for Tail via the D3A→D3B value bridge (Direct via its own declared-call
    return), and using the landed `CheckedIhFreshResultRoute` ONLY for its
    RETAINED topology/identity facts and NEVER as Tail value authority (HS12),
    carry it to the
    eventual ordinary `ITree::Ret` payload the Ret-case closure consumes.
    **Bind ONLY that later `R2`. NEVER the initial carried environment word, and
    never `R1`** (HS10). Do NOT
    trace the earlier D3A result `R1` transitively through the source-control
    chain into the capture — that is the withdrawn reading. The READ program's
    destination coordinates are origin 301 active frame / closure 460 / capture
    occurrence 459 / body 452 — READ-side EVIDENCE coordinates only, from the
    disposable Architect probe. The WRITE program's active frame, eventual Ret
    payload, ordinary capture occurrence, and body read MUST be derived
    INDEPENDENTLY from its OWN existing graph/planner facts before any repair —
    never reused from, or inferred by similarity to, the read coordinates.
  - Bind each program's FRESH `R2` through the Ret case's `CheckedCaseBinderLayout`
    to THAT program's own exact ordinary closure-capture occurrence and body read
    (read: capture 0 = occurrence 459, `Var(0)`, body-452 `Var(1)`; write: its
    independently derived analogue). Identify the FIRST edge where the fresh `R2`
    fails to reach that ordinary capture (the transported environment persists in
    its place). ONLY that edge may be repaired.
  - If the write analogue cannot be derived from existing planner relations,
    HARD-STOP **INSIDE THIS ATOMIC CANDIDATE** — do NOT infer it by source
    similarity to the read program, and do NOT infer a predecessor from it. **The
    former "hard stop 5 under the D3B rule below" routing is WITHDRAWN with that
    rule (HS12): no predecessor node owns this, and none is authorized.**
  - Authority comes from EXISTING graph/planner facts: source-continuation frame
    identity, active-frame lineage, constructed occurrence/result edge, Ret-case
    binder role, and the exact boundary-closure capture descriptor. Numeric
    origins are report coordinates only. `CheckedIhBinding(None)` at the Ret
    capture is a NEGATIVE CONTROL — it forbids reclassifying the capture as an
    IH.
  - **HS12 UPDATE (`evt_7a6pp8n24r1ms`) — this REPLACES the former HS5 update,
    which is WITHDRAWN.** The landed successor
    [[RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR]] contributes RETAINED TOPOLOGY AND
    IDENTITY FACTS ONLY. **It does NOT own the missing Tail relation, and NO
    predecessor supplies Tail value authority.** This atomic candidate SUPPLIES
    the replacement Tail relation and the value bridge itself, and D3B CONSUMES
    the replacement relation plus the bridge-delivered operand — through the
    ordinary active-continuation semantics, never re-deriving the relation in
    lowering. **NO new predecessor and NO further hard stop on a predecessor
    node is authorized**; the former "builds ONLY after that predecessor lands"
    and "HARD STOP 6 on the predecessor node" instructions are void, since the
    edge HS12 identified is the semantic repair itself and cannot be landed
    ahead of the products it changes. Direct is unaffected: it consumes its own
    declared-call result as before.
  - Prohibitions PRESERVED under both arms: do NOT create a lowering-side
    reverse search, a second binder catalog, a second identity catalog, an ABI
    lane, a raw cast, an environment search, or a family-specific fallback; do
    NOT write capture 0 directly, project a convenient result field, inject the
    result after `ResumeOuter`, or change D1.

## Acceptance criteria

> **THE ONLY CRITERIA IN SCOPE FOR THE AUTHORIZED TURN ARE THE TWO
> `AC-HS13-D0-*` ENTRIES IMMEDIATELY BELOW.** Everything after them governs the
> eventual D3 increment and is NOT satisfiable at D0. **The Architect reviews
> the D0 report; Runtime QA stays UNROUTED, because D0 lands no production.**

- **`AC-HS13-D0-CLOSURE` (HS13, `evt_59t7b49m41z8m`) — the seam and caller
  closure is proved CLOSED, structurally, and NOT by a grep roster.** On both
  unchanged admitted products, deliver a ledger naming every exact
  `RoutedAnswer` -> `LoweringOperand` shrink seam on the natural path, that
  seam's input and output types, EVERY caller of it, and each caller's
  disposition. **A name list is not a closure proof.**

  > **THE NATURAL PATH STARTS AT TAIL'S OWN PRODUCER, AND THE START IS PART OF
  > THIS CRITERION** (Architect `evt_7tsep4b5sdqew`). Root the ledger at
  > `call_checked_ih_transport_from_case_environment` — the bare
  > `LoweringOperand` it returns, which source wraps as
  > `RoutedAnswer::checked(returned)` at `source.rs:4369-4374` with
  > `transport.source_call_identity()` still locally available. **NOT at the
  > other `RoutedAnswer::checked` caller**, `call_checked_ih_environment_transport`
  > at `core.rs:7622-7626`, reached through `ClaimedContinuationResult`. **There
  > are exactly two, and a ledger rooted at the wrong one measures a different
  > producer** however closed it is. This is stated here because attempt 1 read
  > "the natural path" and took the core branch.

 The instrument must be
  compiler-backed, so that an OMITTED caller is a build ERROR rather than a
  silent absence — the observation has to differ when the claim is false. One
  acceptable instrument: a disposable private result wrapper that is neither
  `Copy` nor `Clone` and exposes no bare-operand conversion, so every surviving
  extraction site must be named to compile. **An equivalent instrument is
  acceptable only if the report NAMES the false appearance it rules out.** The
  completed measured closure must build green under the TARGETED affected builds
  only (`scripts/ken-cargo -p <crate>` / `--test <name>`; never `--workspace`),
  **and that build set must COMPILE EVERY CALLER THE LEDGER NAMES.** A
  `-p <crate> --lib` build does not compile a `cfg(test)` caller, so a ledger row
  in a test target needs `--lib --no-run` or `--test <name>` coverage or it is a
  counted-but-unchecked row — the compiler-backed property is exactly what a row
  outside the build set does not have.
  Record the diff and log hashes, then byte-restore: the wrapper is measurement
  scaffolding and lands nothing.
- **`AC-HS13-D0-AFFINE` (HS13, `evt_59t7b49m41z8m`) — the affine feasibility
  answer is two-sided, and its NO is as much a delivered result as its YES.**
  Report **YES** only if ALL of the following hold across the closure
  `AC-HS13-D0-CLOSURE` measured: every pending `Produced`/`Routed` path remains
  owned; no path can extract a bare `LoweringOperand`; ordinary/direct is an
  explicit exhaustive variant rather than a fallthrough; the exact producer
  identity is CHECKED when the later route arrives, not assumed from position or
  proximity; and one-shot consumption at the active jump is expressible without
  any mechanism on the rejection list in Phase structure. Report **NO** with the
  FIRST exact seam and caller that forces a general composed-lowering redesign.
  **A NO is HS14 — a clean stop routed to the Architect, and a successful D0
  outcome rather than a failed turn.**

  > **THE REPORTED SEAM MUST LIE ON THE TRACED VALUE'S OWN PATH, PROVED BY TYPE
  > OR IDENTITY CORRELATION — never by a shared `route`/`role` tag, and never by
  > aggregate product reach** (Architect `evt_7tsep4b5sdqew`). Both
  > `RoutedAnswer::checked` callers mint checked route, so a tag match is
  > consistent with the traced value never reaching the seam at all. Use a
  > compiler-backed non-`Copy`/non-`Clone` `Produced` value, or a genuinely
  > equivalent identity-bearing type, so a bare extraction cannot hide; then walk
  > THAT value to the later Tail route or to the first unavoidable general
  > boundary. If it enters the general S1/S2/S3 protocol, prove the entry by
  > correlation and report it; if it stays bounded to source-machine returns, say
  > so and name the boundary. **A NO grounded in a DIFFERENT producer's breadth
  > is not HS14.** That is precisely the attempt-1 defect, and it is
  > outcome-changing: the exact source path may instead force a source-machine
  > typestate redesign at S4/S5, which is a different answer. Reporting YES on a closure that was not
  measured closed under `AC-HS13-D0-CLOSURE` is the failure this pair exists to
  prevent.
- **`AC-TAIL-TRANSFER-CONTROLS` (HS12, `evt_7a6pp8n24r1ms`) — the controls must
  discriminate the NEW EDGE, not the endpoints it connects.**

  > **NOT YET IN SCOPE (HS13).** This AC governs the eventual transfer
  > implementation. **D0 is a measurement and lands no production, so this AC is
  > not satisfiable at D0 and must not be attempted there.** It is retained
  > unchanged because the semantic property survived HS13 — only the mechanism
  > locus was refuted. **Re-read it against whatever mechanism D0 finds before
  > treating any arm below as still applicable**; arms naming a producer-locus
  > pairing or an ownership carry describe a recipe that no longer exists.

  Twelve stops
  produced controls that passed on endpoint facts; this one is about a directed
  value transition, so every arm below must be exhibited. **Each arm must
  preserve compilation far enough to OBSERVE the claimed failure** — an arm that
  fails to build proves nothing, and this chain has twice shipped a control that
  could not fail.

  **This is a PER-COORDINATE MUTATION FAMILY, and it has NO fixed count. Do not
  restate it as a numbered list of arms** — the axes below vary the producer, the
  two neighbours, the route coordinates, the consumption directions, the
  marker/header, and the Direct substitution INDEPENDENTLY, so any stated total
  is both wrong and narrowing: it converts an open family into a checklist
  somebody can finish. **The Steward wrote a "nine arms" gloss into the tracker
  and the Architect struck it (`evt_2mkjv4g5ss9pp`).** The axes, non-exhaustively:
  - delete the producer-to-transition capture;
  - substitute the SEED for `returned`;
  - substitute either neighbouring live word;
  - mismatch the producer call, the governed invocation/application/callee, the
    active header, the Ret binder, the ordinary capture, or the body read;
  - drop consumption, and separately duplicate it;
  - payload the `RecursiveBackedge` marker;
  - add a second header argument;
  - route Tail through Direct.
- **`AC-SEED-NEGATIVE` (HS12).** The seed stays semantically real — it remains
  the initial `CheckedIhCapturedEnvironment` residual used to install the checked
  invocation — but it is NEVER the governed Tail result and never the active jump
  argument after the checked computation returns. **Substituting the seed for
  `returned` while leaving the producer INTACT must RED the producer-to-body-read
  claim AND both admitted full-program products.** A tree in which the seed could
  still be selected as the transferred value has not met this criterion, whatever
  it asserts.
- **`AC-TAIL-PRODUCTS-EXACT` (HS12).** The admitted read/write products reach
  exact `InvalidOffset` — **not merely a changed default.** A product that moved
  off the `ResourceBodyResult` default without landing on the exact expected
  value is a fresh symptom, not a pass.
- **`AC-AFFECTED-CLOSURE` (HS12, and it is broader here than the Rust-side
  reading).** Cover every target that loads any module whose CLOSURE this
  increment changes, diff-touched or not — **including every `ken run` consumer,
  not only compile-time ones.** A target that shells out to `ken run` over
  catalog-consuming sources is a closure consumer no Rust-side analysis reaches.
  This is not a relaxation of the targeted-build rule: what changes is which
  targets count as affected, never how many crates build at once. The criterion
  has now cost three lanes a red merge, and the `ken run` half cost lane 3 one
  more after the Rust-side half was already written into the frames.
- AC-D2-1 (environment census — accepted) — the eight-binding environment is
  fully censused, each slot bound to its exact producer / insertion op / source
  origin / binder-capture role / carried identity; slot 1 is traced
  producer-to-`Var(1)`-read through every join/continuation. Satisfied by
  evidence `ac1ebdacb`.
- AC-D2-2 (producer census + single classification — accepted, **DIRECT-SCOPED
  under HS12**) — every planner-authorized producer of the two receiving
  `ResourceBodyResult` identities is censused; classified as EXACTLY ONE arm.
  **The classification is scoped to what D2 actually measured: at
  `call_checked_ih_transport_from_case_environment`'s exact carried CFG arm, NO
  DIRECT declared-call application or result is emitted and the arm returns the
  seed.** It makes NO claim that Tail's governed application or its declared-call
  producer is absent, and NO claim that fresh `R2` is globally never minted —
  HS12 (`evt_7a6pp8n24r1ms`) proved both exist on Tail and that the missing fact
  is DELIVERY. The prior "never-minted (the source continuation was not applied)"
  wording is WITHDRAWN as a global claim. Satisfied by evidence `ac1ebdacb`.
- AC-D2-3 (typed carrier — accepted) — the live carrier is bound to a typed
  producer identity (`CheckedIhCapturedEnvironment`, not a result) and the
  source value it represents is stated; no spelling / ABI / family / trap /
  field-count / `Var(1)`-index authority is introduced. Satisfied by evidence
  `ac1ebdacb`.
- AC-D2-4 (natural reachability ONLY — REPLACES the withdrawn pre-repair
  positive, Architect evt_1hren6zm8mgxv) — on the UNCHANGED admitted read/write
  programs: each reaches the exact graph-authorized carried branch with its
  exact transport / source-record / worker-body / result identities (read
  `608`/`662`/`939`/spec 1, write `720`/`1238`/`1257`/spec 3); NO continuation
  application is emitted on that branch; the typed result is ABSENT from the
  closed eight-entry receiving environment; each program reaches its exact
  downstream fail-closed default. An entry-marker/refusal mutation at THIS exact
  carried branch may prove both programs reach it (then byte-restore), but it
  MUST NOT apply the worker or inject a result. A neighboring `StaticWorker`
  test may remain as instrument/regression health only and MUST NOT be credited
  as same-path evidence. This AC is discharged by the accepted evidence plus the
  reachability-marker mutation; it does NOT require a post-repair positive.
- AC-D3A-APPLICATION (carried application executes — D3A, evidence only;
  **ROUTE-VARIANT-SPECIFIC, amended at HS10**) — on the UNCHANGED admitted
  read/write programs the D3A candidate makes each governed carried arrival apply
  its exact source continuation **through the mechanism THIS CANDIDATE'S ROUTE
  RELATION selects after replacing the Tail variant** — landed Direct retained,
  landed Tail topology/identity only, never value authority (HS12) — and expose
  the fresh result:
  - **`DirectInvocationReturn`** — through the ruled
    transport/projection/envelope/single-declared-call shape; the call result is
    fresh `R2`.
  - **`TailProducerToBackedge`** (`TailResumedRetInput` is ABSENT) — through the
    existing zero-argument governed recursor call: checked marker consumed once,
    invocation segment installed once, existing active self-resumption run.
    **Fresh `R2` IS a declared-call return** — the continuation-specialization
    call at `calls.rs:2022`, its Result-slot load at `:2120`, returned as
    `RoutedAnswer::checked(returned)` at `source.rs:4369-4374`. **What Tail
    lacks is Direct's `continuation_calls[...]` lookup and Direct's extra call,
    NOT declared-call production; the prior wording said otherwise and HS12
    corrected it.** `R2` reaches the active header ONLY through the D3A→D3B
    value bridge — **there is no pre-existing certified active-header /
    Ret-input VALUE edge to carry it, and it is NEVER the initial
    `CheckedIhCapturedEnvironment` residual word.**

  **A candidate that satisfies this AC by making Tail rows take the Direct path
  FAILS it**, however green the suite: that is the HS10 defect restated, not the
  criterion met. This AC makes NO claim about the final capture or
  `InvalidOffset` and does NOT land alone.
- AC-D3A-PAIRING (one application per arrival, **per variant**) — pair EVERY
  governed carried-branch arrival with EXACTLY ONE application event, keyed by
  the identity its own variant carries: for Direct, the same transport identity,
  source record, worker body, source result and destination owner; for Tail, the
  same exact invocation/call/callee triple and active governed frame. Unpaired
  scalar totals are INSUFFICIENT — the programs may legitimately reach the seam
  more than once. **A pairing that silently covers only the Direct row satisfies
  nothing about the four Tail rows; report the pairing population per variant so
  a one-row proof cannot read as a five-row one.**
- AC-D3B-RESULTFLOW (fresh-result-delivery positive — D3B; TWO separately paired
  paths) — the atomic (D3A+D3B) candidate applies inherited `K` at each exact
  recursively-exposed invocation, yields the FRESH result `R2`, and binds THAT
  `R2` — for Tail through the D3A→D3B value bridge this candidate supplies, with
  the landed `CheckedIhFreshResultRoute` contributing RETAINED topology/identity
  facts and NOT Tail value authority (HS12), and the
  Ret-case `CheckedCaseBinderLayout` — to THAT program's own exact closure-capture
  occurrence and body read, then proceeds through exact `ResourceBodyOk` /
  `ResourceBodyErr` selection to the independently specified `InvalidOffset`
  observation and effect prefix. The candidate does NOT trace the earlier D3A
  result `R1` transitively through the source-control chain into the capture, and
  asserts NO identity between `R1` and `R2`. The READ path is paired through its
  exact read-side destination coordinates (origin 301 / closure 460 / capture
  occurrence 459 / body 452 `Var(1)`); the WRITE path is paired through its OWN
  newly derived exact analogue. Reusing the read numeric coordinates as write
  authority is FORBIDDEN; if the write analogue cannot be derived from existing
  planner relations, hard-stop.
- AC-D3B-RESULTFLOW-PAIRING (two separate pairings, never one transitive) — pair,
  SEPARATELY: (i) the `K`-inheritance — the inherited continuation capability to
  the exact recursively-exposed zero-argument invocation; and (ii) the fresh `R2`
  — the result of applying `K` there to the exact eventual Ret payload and closure
  capture. Do NOT collapse the two into one transitive pairing from `R1` to the
  capture. No scalar-total substitution; a bare target-call / consumption count
  does not suffice.
- AC-D3-TRIPLE-SUPPRESS (three independent causal mutations; **the application
  axis is ROUTE-VARIANT-SPECIFIC, amended at HS10**) — after the full repair,
  INDEPENDENTLY: (i) suppress ONLY the `K`-inheritance (the inherited
  continuation capability at the recursively-exposed invocation), keeping
  application and binding paths live; (ii) suppress ONLY the D3A production
  application of `K`, keeping entry/descriptor/detector and the inheritance live;
  (iii) suppress ONLY the newly repaired D3B fresh-`R2` binding edge, keeping
  inheritance and application live. EACH must return BOTH programs to the
  localized sole default, and restore byte-identically to recover both exact
  products. No scalar-total substitution.

  **Axis (ii) is TWO DIFFERENT MUTATIONS, one per variant, and neither covers the
  other:**
  - **Direct suppression deletes ONLY its declared call.**
  - **Tail suppression deletes ONLY the exact marker-consumption /
    invocation-install application, while keeping the inheritance AND THIS
    CANDIDATE'S REPLACEMENT TAIL ROUTE/TOPOLOGY IDENTITIES PRESENT — and it MUST
    FAIL CLOSED.** A Tail suppression that also removes the route, or that lets
    the row fall back to any other arm, is measuring the wrong thing.
    **"Present" here means the replacement route's topology and identity facts,
    NOT the predecessor's withdrawn value-certified edge (HS12) — that edge does
    not exist and cannot be kept present.**

  **Per the standing lesson from `RT-FRESH-RESULT-ROUTE-PAIRING-LEG-CONTROLS`:
  count these controls PER CONJUNCT, not per axis.** One mutation that reddens
  the suite does not establish that the axis is pinned for a variant it never
  exercised. Name, for each variant, the arm that reddens when only that
  variant's application is deleted.
- AC-D3-ATMOSTONCE — prove at-most-once INDEPENDENTLY for the `K`-inheritance, the
  application of `K`, AND the fresh-`R2` binding, each either STRUCTURALLY or via
  the opposite duplicate mutation. A removal mutation proves at-least-once only;
  no scalar total substitutes for any of the three. **All three dynamic axes are
  retained (HS10) and the static route certificate is still NOT a fourth.**
  **On the Tail variant, at-most-once is specifically about the marker
  consumption and the invocation install: each must happen EXACTLY ONCE per
  governed arrival.** Prove it for Tail rows on their own population — a Direct
  row's proof says nothing here, because the two variants apply through different
  mechanisms.
- AC-D3-CHECKED-TRACE — retain a SEPARATE exact checked-route trace through
  `CheckedSelectedRecursor`, checked `CarriedEliminationEntered`, and
  `CarriedFallbackEmitted`, but do NOT substitute that trace for the
  carried-application or result-flow pairing.
- AC-D3-INDEPENDENT-ORACLE — keep expected `InvalidOffset` and the effect
  prefixes INDEPENDENT of the new lowering logic. No result derived from the
  repair mechanism may serve as its own oracle.
- AC-5 / AC-D1-PRODUCT (final product, GATED on D3 — relocated from RT-ITREE) —
  on BOTH admitted programs SUCCESS is the exact `InvalidOffset` SemanticErrorV1
  observation with the preserved effect prefixes (read `FsOpen -> BufferAllocate
  -> ResourceRelease(FsHandle) -> ResourceRelease(Buffer)`, no `FsReadAt`; write
  `FsOpen(source) -> FsOpen(sink) -> ResourceRelease(source) ->
  ResourceRelease(sink)`, no `FsWriteAt`) — not merely the absence of the trap.
  The transitional route/frontier witness left by the D1 slice is replaced by
  the durable nonignored `InvalidOffset` witnesses here.
- AC-D3-SCOPE — both fail-closed defaults, `erasure.rs` (blob `8532ced2...`),
  D1's private route lane, ordinary-case precedence, the checked-answer caller
  population, and the respective read/write effect prefixes are all preserved.
  PROHIBITED (HS4-augmented): writing closure capture 0 directly; projecting a
  convenient result field; injecting the call result after `ResumeOuter`; keying
  on runtime words / tags / family / spelling / field counts; using `Var(0)` or a
  capture ordinal as authority; reclassifying the `CheckedIhBinding(None)` Ret
  capture as an IH; a lowering-side reverse search or second binder catalog;
  scanning environment tags in production to "find" a matching value;
  family-specific routing; raw casts; reminting the checked answer as a
  `ResourceBodyResult`; synthesizing a worker; duplicating the continuation body;
  adding a second transport / identity catalog / ABI lane; bypassing the default;
  altering the already-correct parameter/capture mapping; changing D1.
- AC-NO-REGRESSION — whole-suite green in CI; local targeted `-p ken-runtime` /
  `-p ken-cli` / `-p ken-verify` only, never `--workspace`.

## Reviewers

> **FOR THE AUTHORIZED TURN (D0, HS13 `evt_59t7b49m41z8m`): the ARCHITECT
> reviews the D0 report against `AC-HS13-D0-CLOSURE` and `AC-HS13-D0-AFFINE`,
> and RUNTIME QA IS NOT ROUTED — D0 lands no production, so there is no
> candidate to gate.** A D0 NO is HS14 and routes to the Architect as a clean
> stop. **Everything below is the review contract for the eventual D3A+D3B
> candidate and is NOT in scope now.**

Architect (D2 reworded reachability introduces no application/result and no
spelling/ABI/family/trap/field-count/index authority; D3A applies the inherited
`K` at the exact recursively-exposed invocation supplied by the predecessor's
`K`-inheritance proof, **through the mechanism THIS CANDIDATE'S ROUTE RELATION
selects after replacing the Tail variant, and not a uniform one (HS10); landed
Direct retained, landed Tail topology/identity only (HS12)** — for
`DirectInvocationReturn` the
body-refined `CheckedIhEnvironmentTransport` plus ONE declared call, projected
capture ordinals, single envelope/call path, no `StaticWorkerBinding` synthesis,
no second identity catalog/ABI lane; for `TailProducerToBackedge` the EXISTING
zero-argument governed recursor call with checked marker consumed once and
invocation segment installed once, **no `continuation_calls` lookup and no extra
Direct declared call, no target selection, and the initial carried environment
word never treated as `R2`** — with fresh `R2` produced by the declared
continuation-specialization call and delivered by the D3A→D3B value bridge, and
does NOT land alone. **Reject any candidate that
routes Tail rows through the Direct recipe, and check the pairing population PER
VARIANT so a one-row Direct proof cannot pass as a five-row one.** D3B binds that
FRESH `R2` (never `R1` traced transitively, never the initial environment word)
through the D3A→D3B value bridge for Tail — the landed
`CheckedIhFreshResultRoute` supplying RETAINED topology/identity facts only, never
Tail value authority (HS12) — and the Ret-case binder to the exact ordinary
closure capture, repairing the FIRST graph-authorized edge where `R2` fails to
reach it, with authority from EXISTING planner relations only, no lowering-side
reverse search, no second binder catalog, no direct capture-0 write, the
`CheckedIhBinding(None)` Ret capture preserved as a negative control, and no
re-derivation of the inheritance relation in lowering; both fail-closed defaults
and `erasure.rs` intact) + runtime-qa (AC-D2-4 proves natural reachability WITHOUT
applying/injecting; D3A pairing is one-application-per-arrival; AC-D3B-RESULTFLOW
binds the FRESH `R2` with no `R1`->capture trace and no scalar-total substitution;
AC-D3B-RESULTFLOW-PAIRING keeps the `K`-inheritance and fresh-`R2` pairings
SEPARATE; the THREE independent causal suppressions — inheritance, application,
and the repaired fresh-`R2` binding edge — each redden to the localized default
and byte-restore; at-most-once holds INDEPENDENTLY for inheritance, application,
and fresh-result binding; the checked-route trace is retained but NOT substituted
for the pairings; the final `InvalidOffset` product holds on both programs with
the exact effect prefixes, independent of the lowering logic). QA is requested on
the atomic D3A+D3B candidate only — NOT on the frozen `ac1ebdacb` or `719933055`
evidence.

## Capability tier

T1 — a graph/claim continuation-binding repair reviewed on the provenance
argument, not a differential diff. **The review union is ROUTE-SPECIFIC, and
stating it globally is what made the earlier text read Direct-shaped:**

- **`DirectInvocationReturn`** — which body-refined
  `CheckedIhEnvironmentTransport`, which projected capture ordinals, and the ONE
  declared call.
- **`TailProducerToBackedge`** — **at D0 (HS13) the argument is a RETURN-BOUNDARY
  CLOSURE argument:** whether the enumerated seam set and its complete caller
  closure admit ONE compile-time affine return typestate, and whether that
  closure can be BOUNDED without redesigning general composed lowering. **It is
  no longer an ownership-carry argument — HS13 refuted that locus.** The property
  it must eventually establish is unchanged: that the operand delivered is the
  declared continuation-specialization call's result and not a neighbouring live
  word, and that it is consumed exactly once at the active jump. **Transport and
  capture projection are NOT part of this argument** — Tail's discriminator is
  `direct_transport=None`.
- **Shared by both** — one application per arrival, and the exact result-flow
  edge from the application result to the eventual Ret payload / closure
  capture.

The pre-repair localization is an accepted object. **Feasibility is accepted
ROUTE-SPECIFICALLY, never globally:** `719933055` proves the DIRECT call recipe
only; Tail's application, producer and bridge are grounded by HS11
(`evt_79trx05xee0dj`) and HS12 (`evt_7a6pp8n24r1ms`) instead. An unqualified
"D3A application feasibility is accepted" credits Tail with a proof that was
never performed on it. Size M.

## Sequencing

Lane-1 (runtime, priority). D2 localization is ACCEPTED (evidence `ac1ebdacb`,
no merge, no QA); **D3A's DIRECT call recipe is proven feasible (evidence
`719933055`, no merge, no QA) — that WIP proves NOTHING about Tail, whose
application, producer and value bridge are grounded by HS11
`evt_79trx05xee0dj` and HS12 `evt_7a6pp8n24r1ms`**; D3B localization is ACCEPTED
(evidence `4e516e54`, HS5). The
**ALL PREDECESSORS HAVE LANDED AND NO FURTHER ONE IS AUTHORIZED — restated and
HARDENED at HS12 (`evt_7a6pp8n24r1ms`), previously HS10
(`evt_1ckwtvwe23e3e`).** Every node in `depends_on` is `merged`, including
[[RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR]] and the corrected
[[RT-CHECKED-IH-FRESH-RESULT-ROUTE]] (`7d36d24f0`). **The landed fused route
contributes RETAINED TOPOLOGY AND IDENTITY FACTS; its Tail VALUE authority is
WITHDRAWN (HS12). This atomic candidate replaces the Tail variant and supplies
the one missing producer-to-backedge value edge itself. Adding a predecessor is
explicitly forbidden.** What
blocked HS10 was this frame's uniform D3A recipe, since amended to be
route-variant-specific.

**HS12 SUBSUMED THE TWELFTH STOP'S WORK INTO THIS NODE RATHER THAN IN FRONT OF
IT.** [[RT-CHECKED-IH-TAIL-RESULT-PRODUCER-ROUTE]] is REMOVED from `depends_on`
and retained as tracker history for its D0 evidence and hard-stop record.
**Reason, and it generalizes: changing the active predecessor operand changes the
Ret input, the capture, and the body result, so it cannot land as a separate
inert proof — a node that lands first would either be behaviourally inert (and
therefore not the repair) or would break the products it landed ahead of.** The
Tail value transfer is now a D3A bullet, inside the atomic increment.

**HS13 (`evt_59t7b49m41z8m`) — THE NEXT AUTHORIZED TURN IS D0 ONLY, NOT THE
ATOMIC CONSUMER.** The third release is SPENT. The ring attempted the accepted
Direct recipe plus the HS12 Tail transfer and hard-stopped cleanly: no commit, no
candidate, no QA, baseline restored. **HS12's mechanism locus is refuted, its
semantic property stands.** No new predecessor is authorized — the return
boundary is inside the same semantic repair. No Decision and no Research advisory
follow from HS13; **HS15 remains the next mechanical Research trigger.**

Runtime is HELD until the Steward issues a **NEW EXPLICIT RELEASE against the
RECUT frame blob** — neither the HS12 ruling, nor the HS13 ruling, nor this recut
landing, nor any predecessor landing authorizes a code turn. **Landing a frame
alone releases nothing**, and the previous releases are SPENT.

**The next turn delivers the D0 return-boundary closure in Phase structure. It
lands NO production and routes NO QA, and it may end at HS14** — an unboundable
signature/caller closure is a clean stop routed to the Architect, never a
mechanism selected under pressure.

ONLY AFTER D0 ANSWERS AND THE STEWARD RELEASES AGAIN does the Runtime ring
rebase the D3 branch and build the ATOMIC D3A+D3B
consumer candidate (D3A application of inherited `K` **per route variant**
yielding fresh `R2` + D3B fresh-`R2` binding to the ordinary Ret capture +
product) — no application-only checkpoint. **D3A and D3B consume THE
CANDIDATE'S SINGLE RELATION AFTER REPLACEMENT: the landed Direct arm is
retained and consumed as-is; Tail consumes the REPLACEMENT Tail relation and
the delivered bridge operand; the landed Tail variant contributes
topology/identity facts ONLY, never value authority (HS12).** Never re-deriving
the relation in lowering, never tracing `R1` into the
capture, and never binding the initial carried environment word. After this node greens `InvalidOffset`,
[[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]] (ReadSome/Wrote) and the final
four-value closure fold follow; the D1 follow-up
[[RT-CHECKED-SUCCESSOR-EMIT-REACHABILITY]] is sequenced after this node on the
single Runtime ring (ring contention, no logical dependency). PX8 stays blocked
until the whole native carried-value program lands. Single Runtime lane object
at a time. **Operative inventory state: entries 1-13. The ENDPOINT-ADDITION
predicate is TERMINAL at entry 12 under HS12 (`evt_7a6pp8n24r1ms`) — stop 12
ends the endpoint series. Entry 13 (HS13, `evt_59t7b49m41z8m`) is the LIVE
symptom and is a DIFFERENT class: lifetime/ordering, with the result-bearing
type erased at an intervening Rust return boundary before the consumer route
exists.**
Earlier fold hashes (`529f21c43e1c0c5257d2f7898481aaa3dc3a0429`, entries 1-4;
`244b2468afd4f0cd06837fd3079f291d7d330af5`, entry 5) are HISTORY and are not the
current inventory.
