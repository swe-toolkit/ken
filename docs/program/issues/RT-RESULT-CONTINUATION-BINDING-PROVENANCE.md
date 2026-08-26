---
id: RT-RESULT-CONTINUATION-BINDING-PROVENANCE
title: "RT-ITREE D2/D3A/D3B — the checked ITree Ret carried arm (call_checked_ih_transport_from_case_environment, core.rs:7699-7714) settles InlineNoCall and returns the transported CheckedIhCapturedEnvironment word WITHOUT applying the source continuation. HS4 (evt_6mnawfvm8fc4j) proved applying it is real but insufficient: the call result becomes a new ITree node and crosses an active recursive computation (TerminalResumeOuter -> Computational 301) before the Ret-case closure 460 capture 0 / final Var(1) it must reach (301/460/459/452 are READ-side evidence coordinates only; the write analogue is derived independently), so an applied-but-unconsumed call is not semantics-preserving. D2 localization ACCEPTED as evidence (ac1ebdacb; no merge, no QA). D3A (evidence only): the exact carried application, per-arrival paired. D3B: localize then repair the FIRST graph-authorized edge where the applied result fails to reach the eventual Ret payload/closure capture. The merge is ATOMIC (D3A + graph-authorized result flow + product) — no application-only checkpoint."
status: active
owner: runtime
size: M
gate: none
depends_on: [RT-ITREE-DEFAULT-SELECTION-PROVENANCE, RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR, RT-CHECKED-IH-K-AVAILABILITY-LOCATOR]
blocks: []
github: null
origin: "Architect hard-stop-2 ruling evt_5w03f4zbg02ry, 2026-08-26, splitting RT-ITREE-DEFAULT-SELECTION-PROVENANCE; then hard-stop-3 ruling evt_1hren6zm8mgxv, 2026-08-26 (option (c), D2/D3 phase separation, Research advisory evt_4cbecpkg2e0gs accepted). D1's route slice landed independently (21d62130); this node localizes the ResourceBodyResult continuation-binding boundary observed on top of it, then repairs it. Steward-owned recut per the ruling; the final-product ACs (AC-5 / AC-D1-PRODUCT / final InvalidOffset witnesses) live here. Hard-stop-4 ruling evt_6mnawfvm8fc4j, 2026-08-26: the single D3 application leap is split into coupled D3A (application, evidence only) + D3B (result-flow localization then single-edge repair); the atomic merge adds per-step result-flow pairing and dual suppression. Inventory fold 529f21c43e1c0c5257d2f7898481aaa3dc3a0429 (entries 1-4). Frame fixed-input correction evt_10rgb8n31c5sj, 2026-08-26: origins 301/460/459/452 are READ-side evidence coordinates only; D3B derives the write analogue independently from its own planner facts and forbids reusing the read coordinates as write authority (Steward-owned, not a Decision). Hard-stop-5 ruling evt_494k61s04fnv9, 2026-08-26: D3B localization is VALID and lowering has reached the end of its authority — the missing component is an UPSTREAM planner-owned checked-IH result-successor relation, framed as the independently-landable predecessor RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR (which this node now depends_on); D3A stays frozen/non-landable until it lands, then the atomic D3A+D3B consumer builds and D3B consumes ONLY that successor projection. Inventory fold 244b2468afd4f0cd06837fd3079f291d7d330af5 (entry 5). Architect INCORPORATION ruling evt_2prk31prke9cc, 2026-08-26 (accepting Research advisory evt_261gm8y54xttt), grounded on origin/main@a09878026: the predecessor and this consumer conflated two semantic edges by pairing the D3A application result R1 forward to the later capture. Reconciled to the continuation-inheritance reading — D3A applies the inherited continuation capability K at each exact recursive arrival to yield the FRESH result R2, and D3B binds that R2 through ordinary Ret-case/capture semantics; the transitive R1 -> capture requirement is deleted from the operative Objective/Deliverables/ACs/pairing/suppressions/reviewers/Sequencing. Suppression and at-most-once controls are now THREE independent axes (inheritance, application, fresh-result binding). The advisory is incorporated and is NOT hard stop 6 (count remains five). Runtime stays HELD until the predecessor recut lands and the D3A+D3B work is explicitly re-released."
---

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
> source continuation was not applied; slot 1 faithfully carries a planner-typed
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
>   authorized by one existing `CheckedIhEnvironmentTransport`;
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

## Objective

Localize the first unresolved authority, which is UPSTREAM of ordinary
`ResourceBodyResult` selection: the checked ITree `Ret` carried arm that should
apply the inherited continuation capability `K` at the exact recursively-exposed
checked invocation and yield the FRESH result `R2`, instead of the carried arm
returning the transported captured-environment word unapplied. Consuming the
predecessor's continuation-inheritance projection (the `K`-inheritance proof and
the fresh-`R2`-destination proof), bind that FRESH `R2` — NOT the earlier D3A
result `R1` traced forward — through ordinary Ret-case / closure-capture
semantics to the exact capture the Ret-case closure consumes. Then repair the
FIRST graph-authorized edge where the fresh `R2` fails to reach that ordinary
capture, so the admitted read-offset and write-offset full programs green the
exact `InvalidOffset` observation. Runtime must NOT repair the default, write
closure capture 0 directly, search the environment at runtime, trace `R1`
transitively into the capture, or re-derive the inheritance relation in lowering.

## Phase structure (option (c) + HS4 split, Architect evt_1hren6zm8mgxv / evt_6mnawfvm8fc4j)

- **D2 — localization. ACCEPTED as evidence (`ac1ebdacb`); NOT a merge
  candidate; NO QA.** Narrow form only: the carried arm emitted no application
  and returned `CheckedIhCapturedEnvironment` (the stronger causal gloss is
  WITHDRAWN — HS4). Its ACs are the census (AC-D2-1/2/3, satisfied by the
  accepted evidence) plus the reworded natural-reachability AC-D2-4.
- **D3A — apply the inherited continuation capability. EVIDENCE ONLY; must NOT
  land alone.** The ruled `CheckedIhEnvironmentTransport` single-application shape
  (proven feasible by WIP `719933055`) applies the inherited `K` at each exact
  recursively-exposed arrival — the site supplied by the predecessor's
  `K`-inheritance proof — and yields the FRESH result `R2`. Every governed arrival
  is paired to its exact transport/call identity. Makes NO claim about the final
  capture or `InvalidOffset`, and asserts NO identity between `R1` and `R2`.
- **D3B — fresh-result destination binding then single-edge repair.** On both
  unchanged admitted programs, take D3A's FRESH `R2` and bind it, through the
  predecessor's fresh-`R2`-destination projection, via the Ret case's
  `CheckedCaseBinderLayout` to the exact ordinary closure-capture occurrence and
  body read (read: closure 460 / capture 459 / body 452; write: its independently
  derived analogue closure 473 / capture 472 / body 465). Identify the FIRST
  graph-authorized edge where the fresh `R2` fails to reach that ordinary capture.
  ONLY that edge may be repaired. D3B does NOT trace `R1` forward through the
  source-control chain, does NOT re-derive the inheritance relation in lowering,
  and consumes ONLY the predecessor's continuation-inheritance projection.
- **The MERGE is ATOMIC:** D3A application of inherited `K` + D3B fresh-`R2`
  destination binding + the product controls, landed together — no
  application-only checkpoint. Runtime builds this after the predecessor recut
  lands and a fresh release.

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
- HS4 application-feasibility object `7199330550f9eae611b417c30b289722cd8057b1`
  (tree `9f838714182f6a2b837b5819fe6b194adc1e569a`, base `6f00843de`) — EVIDENCE
  ONLY, do NOT merge/QA. One production path, `core.rs +124/-41`, no fallback:
  revalidates the two-endpoint transport, validates `source_record` and capture
  counts, projects ordinals via `checked_ih_capture_origin`, one envelope walk,
  one `continuation_calls[transport.source_call_identity()]` lookup, one
  `call_declared_unit_target`. LLDB proves `ken_continuation_1` executes at the
  new carried site (returned words `0x0f09`/`0x1109`), but final closure body 452
  capture 0 / `Var(1)` still reads `0x1009` (the untagged transported
  environment) — application executed, result NOT delivered. Proves D3A
  feasibility; D3B (result flow) is unbuilt. `erasure.rs` blob `8532ced2...`
  unchanged; `RoutedAnswer::checked(` remains 3 callers. STAYS FROZEN.
- HS5 D3B-localization object `4e516e54712a47cf14c47b7abf2840f943071af9` (tree
  `9f7ac95f038bfb69bd6a881ec14133957e569078`, corrected base `14040ecae`, frame
  blob `5e043db9`) — EVIDENCE ONLY, do NOT merge/QA. The mechanically-rebased D3A
  feasibility tip; its D3B localization independently derived BOTH read (frame
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
- **D3A (exact carried application — evidence only; the ruled component shape,
  proven feasible by `719933055`; Architect evt_1hren6zm8mgxv):**
  - Keep the exact `CheckedIhEnvironmentTransport` as the sole two-endpoint
    authority. In the `Carried(word)` branch, validate its planner record as the
    exact `CheckedIhCapturedEnvironment` for the transport's source owner and
    seat, and validate the runtime field count against the planner-declared
    capture count. The word is a capture vector — NOT code identity, NOT a
    semantic answer.
  - Project capture ordinal `i` from that word with the existing positional
    carrier projection, governed by the transport's exact source record and
    `checked_ih_capture_origin`. NEVER inspect a runtime tag, family, spelling,
    body word, or field-count coincidence to choose the path.
  - Assemble the existing `ContinuationOrdinaryEnvelopeRole` ONCE: nonrecursive
    fields still come from their ruled case-environment coordinates;
    `WorkerCapture` fields come from the exact projected carried-environment
    ordinals; continuation inputs still come from the existing transport
    morphism. Do NOT synthesize a `StaticWorkerBinding` or redirect into the
    neighboring `StaticWorker` branch.
  - Resolve only
    `function_local.continuation_calls[transport.source_call_identity()]`,
    emit ONE declared call through the existing call authority, record it under
    the exact transport, and pair every governed arrival to that exact
    transport/call identity. Factor the `StaticWorker` and carried-capture
    sources into one downstream envelope/call path rather than duplicating the
    continuation body or creating a second call lane. This phase makes NO claim
    about the final capture or `InvalidOffset` and must NOT land alone.
  - If the source record cannot be validated, its captures cannot be projected
    from existing planner facts, or the exact continuation target was not
    already declared into this destination function — HARD-STOP. Do NOT add a
    second identity catalog, ABI lane, raw cast, environment search, or
    family-specific fallback.
- **D3B (result-flow localization then single-edge repair — the coupled atomic
  half; Architect evt_6mnawfvm8fc4j):**
  - On both unchanged admitted programs, take D3A's FRESH `R2` (the result of
    applying inherited `K` at the exact recursively-exposed invocation) and,
    through the predecessor's fresh-`R2`-destination projection, carry it to the
    eventual ordinary `ITree::Ret` payload the Ret-case closure consumes. Do NOT
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
    HARD-STOP under the D3B rule below (hard stop 5) — do NOT infer it by source
    similarity to the read program.
  - Authority comes from EXISTING graph/planner facts: source-continuation frame
    identity, active-frame lineage, constructed occurrence/result edge, Ret-case
    binder role, and the exact boundary-closure capture descriptor. Numeric
    origins are report coordinates only. `CheckedIhBinding(None)` at the Ret
    capture is a NEGATIVE CONTROL — it forbids reclassifying the capture as an
    IH.
  - HS5 UPDATE: the missing planner relation is now OWNED UPSTREAM by
    [[RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR]]. The atomic D3A+D3B consumer builds
    ONLY after that predecessor lands; D3B then CONSUMES that exact successor
    projection through the existing shared D3A call lane and ordinary
    active-continuation semantics — it does NOT re-derive the relation in
    lowering. Do NOT create a lowering-side reverse search, a second binder
    catalog, a second identity catalog, an ABI lane, a raw cast, an environment
    search, or a family-specific fallback; do NOT write capture 0 directly,
    project a convenient result field, inject the result after `ResumeOuter`, or
    change D1. If the predecessor cannot derive the complete relation from its
    forward planner facts, that is HARD STOP 6 on the predecessor node (Research
    triggered) — not a lowering exception here.

## Acceptance criteria

- AC-D2-1 (environment census — accepted) — the eight-binding environment is
  fully censused, each slot bound to its exact producer / insertion op / source
  origin / binder-capture role / carried identity; slot 1 is traced
  producer-to-`Var(1)`-read through every join/continuation. Satisfied by
  evidence `ac1ebdacb`.
- AC-D2-2 (producer census + single classification — accepted) — every
  planner-authorized producer of the two receiving `ResourceBodyResult`
  identities is censused; classified as EXACTLY ONE arm: never-minted (the
  source continuation was not applied). Satisfied by evidence `ac1ebdacb`.
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
- AC-D3A-APPLICATION (carried application executes — D3A, evidence only) — on
  the UNCHANGED admitted read/write programs the D3A candidate makes each
  governed carried arrival apply its exact source continuation through the ruled
  transport/projection/envelope/single-call shape and return the call result.
  This AC makes NO claim about the final capture or `InvalidOffset` and does NOT
  land alone.
- AC-D3A-PAIRING (one application per arrival) — pair EVERY governed
  carried-branch arrival with EXACTLY ONE application event carrying the same
  transport identity, source record, worker body, source result, and
  destination owner. Unpaired scalar totals are INSUFFICIENT — the programs may
  legitimately reach the seam more than once.
- AC-D3B-RESULTFLOW (fresh-result-delivery positive — D3B; TWO separately paired
  paths) — the atomic (D3A+D3B) candidate applies inherited `K` at each exact
  recursively-exposed invocation, yields the FRESH result `R2`, and binds THAT
  `R2` — through the predecessor's fresh-`R2`-destination projection and the
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
- AC-D3-TRIPLE-SUPPRESS (three independent causal mutations) — after the full
  repair, INDEPENDENTLY: (i) suppress ONLY the `K`-inheritance (the inherited
  continuation capability at the recursively-exposed invocation), keeping
  application and binding paths live; (ii) suppress ONLY the D3A production
  application of `K`, keeping entry/descriptor/detector and the inheritance live;
  (iii) suppress ONLY the newly repaired D3B fresh-`R2` binding edge, keeping
  inheritance and application live. EACH must return BOTH programs to the
  localized sole default, and restore byte-identically to recover both exact
  products. No scalar-total substitution.
- AC-D3-ATMOSTONCE — prove at-most-once INDEPENDENTLY for the `K`-inheritance, the
  application of `K`, AND the fresh-`R2` binding, each either STRUCTURALLY or via
  the opposite duplicate mutation. A removal mutation proves at-least-once only;
  no scalar total substitutes for any of the three.
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

Architect (D2 reworded reachability introduces no application/result and no
spelling/ABI/family/trap/field-count/index authority; D3A applies the inherited
`K` at the exact recursively-exposed invocation supplied by the predecessor's
`K`-inheritance proof — the authorized `CheckedIhEnvironmentTransport`
single-application shape, sole two-endpoint authority, projected capture ordinals,
single envelope/call path, no `StaticWorkerBinding` synthesis, no second identity
catalog/ABI lane — yields fresh `R2`, and does NOT land alone; D3B binds that
FRESH `R2` (never `R1` traced transitively) through the predecessor's
fresh-`R2`-destination projection and the Ret-case binder to the exact ordinary
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
argument (which transport, which projected capture ordinals, one application per
arrival, and the FIRST graph-authorized result-flow edge from the application
result to the eventual Ret payload / closure capture), not a differential diff;
the pre-repair localization and D3A application feasibility are already accepted
objects. Size M.

## Sequencing

Lane-1 (runtime, priority). D2 localization is ACCEPTED (evidence `ac1ebdacb`,
no merge, no QA); D3A application feasibility is proven (evidence `719933055`,
no merge, no QA); D3B localization is ACCEPTED (evidence `4e516e54`, HS5). The
UPSTREAM planner-only predecessor [[RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR]] lands
FIRST (independently landable, behaviorally inert); Runtime is HELD on this node
until that predecessor lands and the successor D3A+D3B work is explicitly
re-released. Then the Runtime ring rebases the D3 branch and builds the ATOMIC
D3A+D3B consumer candidate (D3A application of inherited `K` yielding fresh `R2` +
D3B fresh-`R2` destination binding to the ordinary Ret capture + product) — no
application-only checkpoint; D3A and D3B consume ONLY the predecessor's
continuation-inheritance projection, never re-deriving it in lowering and never
tracing `R1` into the capture. After this node greens `InvalidOffset`,
[[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]] (ReadSome/Wrote) and the final
four-value closure fold follow; the D1 follow-up
[[RT-CHECKED-SUCCESSOR-EMIT-REACHABILITY]] is sequenced after this node on the
single Runtime ring (ring contention, no logical dependency). PX8 stays blocked
until the whole native carried-value program lands. Single Runtime lane object
at a time. Inventory fold `529f21c43e1c0c5257d2f7898481aaa3dc3a0429` (entries
1-4, appended by this recut).
