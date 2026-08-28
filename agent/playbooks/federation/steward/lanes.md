# Live lane roster (Steward)

**The single source of truth for the current lane configuration — how many
lanes, which ring each is, and each lane's objective.** This is TIME-VARYING
operator direction. The playbook (`steward.md` §0) holds the stable lane
discipline and points here for the roster; the roster does NOT live in the
playbook.

**Read this at session start and after every compaction**, in the same startup
sequence as `COORDINATION.md`, `MODELS.md`, and your memory scopes. A lane is a
state, not an event — you must know the roster before you act, so it is a
resident startup read, not a fetch-when-needed pointer.

**Update this file only on an operator ruling, and cite the ruling.** No
measurement of yours adds, retires, or re-scopes a lane (`steward.md` §3). When
you measure something that bears on the roster, surface it to the operator; do
not act on it against the roster.

> **Why this file exists.** The roster used to be baked into `steward.md` §0. On
> 2026-08-21 the operator moved from one lane to a three-lane trial; the playbook
> was not where that changed, and the Steward ran the retired one-lane premise
> for a day (it did not launch the authorized foundation lane, and held live
> lane-2 successors as "retired"). A roster in the playbook is time-varying state
> wearing a permanent hat. Operator, 2026-08-22: playbook = stable discipline,
> this file = mutable roster.

## Current roster — three-lane trial (operator 2026-08-22; REAFFIRMED 2026-08-25)

Three concurrent lanes. The trial's own purpose is to measure whether three
lanes overburden the Architect (see lane 3).

> **Operator, 2026-08-25 (reaffirmation, correcting a Steward single-lane
> relapse):** "there are three lanes authorized right now. language (lane 2) was
> unblocking rt on priority, and when that was done should have unblocked
> foundation (lane 3) with module/import." The lanes are runtime / language /
> foundation. Language's job, after its runtime-unblocking work, is
> `LANG-MODULE-IMPORT-SYSTEM` (module/import) — which UNBLOCKS lane-3 foundation.
> Foundation is NOT idle-by-design: it has ready CAT WPs to author now, and its
> reuse-remediation node waits on module/import. Do not collapse to one lane.

The objectives below were re-measured **2026-08-27 against `origin/main`
`ef91b8225`** (earlier same-day refresh at `61c2fefa0`; previous 2026-08-25).
**The roster STRUCTURE — three lanes, runtime / language / foundation — is
operator-owned and UNCHANGED; only the node citations were re-measured.**
Re-measure each node (`git fetch`; read status) before acting; a node id decays,
and at the first 2026-08-27 measurement **seven of the cited nodes had advanced
past the state this table claimed** — five to `merged`. Treat every id below as
a pointer to check, not a fact.

| lane | ring | objective |
|---|---|---|
| 1 | runtime | The native carried-value program `RT-NATIVE-CARRIED-VALUE` (`active`, M-series defunctionalization). M6/M4/M3 merged; `RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE` and `RT-UNIT-FAILURE-STATUS-PROVENANCE` MERGED. CURRENT: `RT-CHECKED-IH-GENERATED-ENTRY-ACCESS` (`active`) — **HS11 per-arrival recut landed `fec63506a`** (frame blob `1ae3e449a8f2`, Architect frame-review APPROVE `evt_2wvn3szecym9f`) and **RE-RELEASED `evt_1mgb3zbskwbg3`**. SEVEN hard stops taken; next mandatory §1a/§1b advisory trigger is **stop 12, NOT 10** (Architect `evt_2s144kdddyckn`). **LANDED `00e66312b` (2026-08-27), all 15 paths blob-verified by the Steward, zero mismatches.** The chain closed at SEVEN hard stops, never reaching the stop-12 advisory trigger. `RT-RESULT-CONTINUATION-BINDING-PROVENANCE` (`active`) was released `evt_6pecj1epnd9pe` and is **FROZEN AGAIN at HARD STOP 8** (`evt_54efxydhb3n6w`): on four of five governed coordinates the exact governed `K` application exposes NO LOCAL `LoweringOperand` result, because the active self-resumption arm returns `RecursiveBackedge`, a PROTOCOL MARKER (`lowering/source.rs:1969`). **That is a LOCAL absence, NOT proof that no fresh result is eventually produced** — the owning carried merge may still produce it, and that is what D0 measures. Do not restate this as "yields no `R2`"; the Architect corrected exactly that overclaim (`evt_33ajd0hmezn2c`). `RT-CHECKED-IH-SELF-RESUMPTION-RESULT-PROVENANCE` (the THIRD sibling proof — WHICH emitted control edge PRODUCES the fresh result; the landed destination type names a destination, not a producer) is **MERGED**: **D0 came back YES**, first candidate `3eaa39f7` was rejected on Direct-variant authority (an ordinary gate outcome, NOT a hard stop), respin **`c8ddfb896` LANDED `830aa0952` (2026-08-28)**, all three paths blob-verified by the Steward, gates Architect `evt_3w11j1atvvj55` + Runtime QA `evt_3y9y0y36rq5xe`, Decision `dec_6ca3h5eg831yj`. CURRENT: **`RT-RESULT-CONTINUATION-BINDING-PROVENANCE`** (`active`) — the atomic D3A+D3B consumer, **RE-RELEASED by the Steward's SECOND EXPLICIT RELEASE `evt_4nd2g8q6963se` (2026-08-28)**, now consuming all THREE sibling proofs, tier T1. Contract unchanged: ATOMIC merge (no application-only checkpoint), THREE suppression axes with the producer proof NOT a fourth, transitive `R1 -> capture` still deleted, read-side coordinates 301/460/459/452 never write authority. **Next stop is 9 and MECHANICALLY triggers the Research advisory before any Architect ruling.** **HARD STOP 9 FIRED, WAS ADVISED, AND IS RULED — the whole cycle closed 2026-08-28.** Stop `evt_5p5mknw26g4qq` taken cleanly at `830aa0952`; mandatory Research advisory `evt_58t039yrevmsk`; **Architect ruling `evt_7wbxwxa74cdnr`, now OPERATIVE (supersedes HS8).** The ruling: for the loop rows fresh `R2` is the governed `K` application result AS DELIVERED INTO THE RET CASE'S INPUT BINDER — NOT the Ret body's output, so NOT the merge parameter. Determined by `§6.2` (`Ret r -> r`) plus emitted causality; the merge is downstream of the capture and cannot flow backward under SSA dominance. **The prior D0 `YES` was an INSTRUMENTATION ERROR — co-emission is not pairing — and the correct answer is NO.** The nine stops share ONE predicate: static endpoint facts treated as a directed dynamic value-flow edge, so **do NOT add a fourth field or another endpoint predecessor**. `CarriedLoopExitResult` is LATENT FALSE AUTHORITY; `830aa0952` no longer establishes predecessor sufficiency for D3, though main is NOT behaviorally regressed (compile-time-only, destination discarded, false arm drives nothing). **Corrected predecessor minted and `ready`: `RT-CHECKED-IH-FRESH-RESULT-ROUTE`** (REPLACES the producer enum; `DirectInvocationReturn` preserved, `TailResumedRetInput` new; inert; tier T1; no Decision required — the ruling is deductive). Stop rule: inability to derive the forward route without prohibited authority is **HS10, stop cleanly**, never a fallback to the merge. **`RT-RESULT-CONTINUATION-BINDING-PROVENANCE` is therefore BLOCKED, not being worked.** So lane 1 moved to its unblocked successor: **`RT-RETAINED-UNIT-CALL-TARGET-DERIVATION` RELEASED and `active`** (Steward, 2026-08-28, framed at `f4045946d`, tier T1) — dep `RT-CARRIED-IH-DISPATCH-SITEOP` merged, NOT gated on the D3 chain. Its inherited owner/key numbers (`ContinuationSpecializationId(2)`, `{800,1008,1321,1374}`, `1236`) are D0's to reproduce or correct, NOT verified by the Steward. **AC-DERIVE RECUT AND LANDED `35b9d3fa1` (2026-08-28), three paths blob-verified, and the ring is UNBLOCKED (`evt_7ymwjr55nx5rv`).** Architect `evt_2jdfsv6w8nh19` ruled the post-call closure-boundary stop a DISTINCT component object and ruled this chain does NOT share HS7's predicate ("each stop supplies another missing input to the same generated-entry accessor at the same final consumer"), so the completed derivation lands as accepted partial work. The recut criterion: the trigger row must no longer REACH the `calls.rs:1638` miss and must advance to completion or to the named closure-representation refusal — **the original "runs end-to-end, `#[ignore]` removed" is SUPERSEDED** (it bundled two component objects; Steward defect, reasoning preserved in the node). **The `#[ignore]` STAYS, re-pointed** to the successor. **The Architect classification is NOT a gate vote**: candidate `6282f149545c973f801c1cf8e715212d9ba77d99` still owes fresh Architect then Runtime QA on the exact SHA. **MERGED as accepted partial at `e03b4d500df422ed2fd7a14569279f1a48be64cd` (2026-08-28), three paths blob-verified by the Steward.** Candidate `a5d21de81`, gates Architect `evt_6h4c1b3r1pfxb` + Runtime QA `evt_19jczj3cfhxpx`, Decision `dec_4tzkskdqy30m4`, routed `evt_jpd8qqxm1vd1`. `ken-ci` auto-close did NOT fire (`github: null`) so the Steward flipped the status by hand — watch for that shape, it leaves a landed node reading `active`. The trigger row's `#[ignore]` REMAINS at `px8f_buffer_native.rs:250`, re-pointed to the successor; that is the recut AC being met, not a defect. **CURRENT lane-1 work: `RT-RETAINED-UNIT-RESULT-CLOSURE-REPRESENTATION`, flipped `ready` and RELEASED by the Steward 2026-08-28 (tier T1)** — the landing discharged its `depends_on`, but the landing alone never authorizes a start; the release does. **A frame-defect correction was carried into it BEFORE release:** its `AC-REFUSE-MALFORMED` had the same enumerated-roster shape that let the predecessor's `AC-NO-SYNTHESIS` miss the traversal-root axis (Architect proved it by seeding from all graph owners — build compiled, control stayed green), so both ACs are now PREDICATE form with the roster demoted to non-exhaustive illustration. **D3A+D3B REMAINS FROZEN**; this landing does not unfreeze it or alter closure transport. **Node status flipped `ready` -> `active` on release (watchdog 2c — a dispatched node left at `ready` is invisible to the per-node sweep).** **The Adversary's M8 hunt on `e03b4d500` returned CLEAN core with non-vacuous controls and ONE grounded LATENT finding (`evt_4qegxp4q31ytc`), now FILED as `RT-RETAINED-TARGET-IDEMPOTENT-REDERIVATION` (`draft`, S/T2, QUEUED, not released):** `units.rs:683` collides on the body key without comparing target values, so a call-graph diamond resolving to the SAME target is refused as ambiguous. **Latent, not a regression** — nothing green completes that staged path while the successor's refusal stands. **The Adversary recommended folding its fixture into the successor and the Steward DECLINED that placement** — the successor is a distinct component object (Architect `evt_2jdfsv6w8nh19`) and bundling them is the exact defect `AC-DERIVE` was recut to remove. Fix and controls are constructible today (claim-level, mirroring the existing `DuplicateTargetClaim` control on the identical-target case, which has NO coverage); release when the runtime seat frees and re-check `units.rs` contention. **D0 ON THE SUCCESSOR IS DONE AND ITS HARD STOP IS RULED (2026-08-28).** A narrow existing-M4 reach/wiring WIP (`aa1b3c793`, parent `e03b4d500`) CLEARS the closure refusal, and the row then compiles, runs, opens both files, and reaches `PatternMatchFailure: no runtime match case selected for ResourceBodyResult`. The Steward routed the classification at `evt_24pf5b7qea3zx` with a SHARPENED question — every prior stop was a COMPILE-TIME emitter refusal, this is the first that compiles AND runs, so the runtime match is the first consumer to actually READ the represented value, and a gate ceasing to fire proves only that it stopped looking. **Architect ruling `evt_4t60zeht79x36`: the trap is a DISTINCT downstream consumer ALREADY OWNED by `RT-RESULT-CONTINUATION-BINDING-PROVENANCE` — do NOT mint a successor** (the Steward's conditional half said it would frame one; corrected). **And M4 did NOT represent the wrong value — measured, not argued:** what reaches the match is class `Constructor`, tag `0`, **seven** fields = the `CheckedIhCapturedEnvironment` shape, NOT this node's **nine**-field object; it is the frozen D3 defect already recorded at `RT-RESULT-CONTINUATION-BINDING-PROVENANCE.md:301-304`. **HS7 re-derived at stop THREE (the Steward re-asked rather than inheriting the stop-two answer): still NO shared predicate** — three different consumers/phases/values/authorities, and the new runtime measurement STRENGTHENS the distinction (object 3 is not object 2 missing a field). **RECUT APPLIED**, all five required items: corrected trigger coordinate (cite the fn name + blob `e0046cd4e`, NOT the stale `:201` — it is line 250), `AC-ADVANCE` (this node had NO advancement AC at all — a Steward defect from carrying only half the predecessor's lesson; compile success alone is insufficient, the end state must be NAMED), `AC-IGNORE-REPOINTED` (ignore STAYS, re-points to the existing D3 node), `AC-POSITIONAL` (the nine captures must be proved consumed in exact positional order; **the seven-field downstream frontier may NEVER be credited as proof the M4 representation is correct — advancement is not correctness**), and a symptom inventory. Runtime may now finish controls and cut a fresh exact-SHA candidate for Architect then Runtime QA, landing as accepted partial. **D3A+D3B REMAINS FROZEN** — re-pointing the ignore names the frontier's owner, it does not unfreeze or release it. **TWO GATE REJECTS FOLLOWED, AND THEY ARE ONE DEFECT — carry this, not the two instances.** Reject 1 (`evt_x9s5s9fks5tg`, `ad7f5044`) found the result proof FAIL-OPEN at its final authorization consumer: a missing target relation returned `Ok(false)` and owner-wide containment authorized the same environment anyway. Reject 2 (`evt_63cdjgpfzzp1e`, `7b5fc2374`) found the committed `SuppressResultAuthorizationArm` INERT — it injected an unconditional `Err` inside `boundary_continuation_result_authorization`, so it proved only that a detector-side refusal prevents D3; the Architect applied the real caller-side mutation (remove only the caller match, leave the result-derived record and both fallbacks byte-present) and **the build compiled and the trigger still reached the same D3 frontier**. Both are the same underlying thing: **the exact result proof was not the EXCLUSIVE authorizer.** The intervening repair fixed the SYMPTOM (fail-closed on disagreement) and not the EXCLUSIVITY, which is why reject 2 read as a fresh finding and was not. **Fail-closed and exclusive are DIFFERENT PROPERTIES** — the first says this arm refuses when its inputs disagree, the second says no OTHER arm may authorize what this one governs; a frame asking only for the first admits a candidate that has neither. **THE FRAMING DEBT WAS THE STEWARD'S**: "prove the exact result arm is causally necessary" originated in the reject-1 respin list, so it lived ONLY in gate prose, and the frame carried no exclusivity criterion at all while the ring failed it twice. **`AC-EXCLUSIVE` IS NOW FOLDED IN AND DURABLE** (`ed828dfb3`, additive `+43/-0`, routed `evt_3nbsgadf8x3hm`): the control must be CALLER-SIDE, and a control that manufactures an error inside the arm it claims to suppress is not evidence of causal necessity. **RESPIN 3 `e0ec51c5247b05792b7c17a9323c359ec7df8ff1` APPROVED by the Architect `evt_b3xny9yz85m4`** (selection separated from authorization; the exclusivity check observes the selected proof and returns no environment ahead of both weaker arms, which stay present for their own populations). Runtime QA routed `evt_3j8jwwhcvdhre` on the exact SHA. The fold does NOT weaken or re-open that approved mechanism. **THEN CI WENT RED ON THE APPROVED OBJECT AND THE MERGE STOPPED — `dec_2v7k1dnp3ykwn` is SPENT, PR #3033 closed as an orphan.** Runtime QA approved `evt_4s7kmfmeqbt51` (independent mutation: replacing only the exclusivity return with `false` reddened the suppression control — real two-sided proof), Decision resolved, routed `evt_54ddty36xwsyr`, and the publisher stopped on `test shard 3/4`. Failure: `contspec_ordinary_prefix_uses_the_ordered_worker_envelope` panicked at `continuations.rs:8977` with `PlannerInvariant("a continuation declares fewer ordinary parameters than its selected worker has captures, so the ruled envelope has no nonrecursive prefix")`. **Candidate-caused, verified: main GREEN on the exact base `4d3d4d848` and on `5c5275657`.** **`continuations.rs` IS NOT ONE OF THE FIVE CHANGED PATHS** — an increment that changes a CLOSURE rather than a FILE breaks consumers no diff-touched target set can see. **MY FIRST MECHANISM FOR IT WAS WRONG AND D0 REFUTED IT (`evt_34gsff3cf1wq7`); the correction is the more useful half.** I said the nine-capture environment moved a capture count read by a planner invariant elsewhere. Measured: base and candidate select the SAME worker with the SAME two captures `[0,1]`, production `ordinary` is 2 on both, and there is NO nine-capture population and NO worker-selection change. The real cause is that the candidate's new result-proof path routes the consumer test's ALREADY-EXISTING `ConstructorFieldCountPrefix` mutation through `ordinary_envelope`, which enforces the invariant — a shape the full planner previously tolerated is now refused. **This makes `AC-AFFECTED-CLOSURE` MORE necessary, not less:** I had assumed the blind spot needed a changed production value to hide behind, and it did not — every production count was byte-identical on both sides and the untouched consumer still broke, because what changed was which code path an existing test state flows through. **That is the case both a diff-touched target set AND a reviewer reasoning about changed values will miss.** **Neither gate was negligent; both ran the targets my frame named, and the criterion was wrong.** **THE DEFECT IS A CROSS-LANE REPEAT AND IT IS MINE:** I diagnosed this exact shape in lane 3 on 2026-08-27 (CV found a candidate passing 170/170 diff-touched targets while reddening a consumer-closure oracle in an untouched file), wrote the affected-target-closure criterion into the FOUNDATION frames, and never carried it to RUNTIME. **A criterion repaired in one lane is not thereby repaired in the fleet.** Folded in as `AC-AFFECTED-CLOSURE`: cover every target that loads any module whose closure the increment changes, diff-touched or not — explicitly NOT a relaxation of the targeted-build hard rule, since what changes is which targets count as affected, not how many crates build at once. **Also learned, and it saved ~20 minutes: failed-job logs ARE exposed before the run completes** — the plain `gh api .../logs` call fails only on terminal escape sequences, which reads as "not yet available" but is not; pass `--allow-escape-sequences`. **REPAIR LANDED — `RT-RETAINED-UNIT-RESULT-CLOSURE-REPRESENTATION` IS MERGED at `bd4ddf2138362bd1ac7066c39161602fdc9dddc2` (2026-08-28), all SIX paths blob-verified by the Steward, not taken on the lieutenant's report.** Candidate `6ce003a2656c3fce28ef3d9a6f875868c7487d88`, gates Architect `evt_3pxw7bgzg0qsr` + Runtime QA `evt_33ewq579c60ba`, Decision `dec_10ph8afwxqxn8`, routed `evt_2z0dsfpy5stcn`, CI run 33162090121 green. **The repair was TEST-ONLY and the Steward verified that independently rather than crediting it:** the delta `e0ec51c52..6ce003a26` touches only `continuations.rs`, all five previously-approved paths are BLOB-IDENTICAL across it (aggregate mechanism blob `a5b455b6d52b3da39ee3ba1f24d57b3a6059a958` intact), and the hunks at 8938/8969 sit inside the top-level `#[cfg(test)]` module opening at 7671 — so the twice-approved production mechanism landed bit-for-bit. **That is also what settles the semantic question**: the new refusal is not introduced by the repair, and D0 showed the refused shape is unreachable in production and manufactured only by the test's own `ConstructorFieldCountPrefix` mutation, so the recut is the correct locus and not a way of quieting a true red. The replacement control is STRONGER than the one it replaced — baseline plan-level assertions preserved via the existing builder, PLUS a required exact `PlannerInvariant` text instead of a generic `is_err`, two-sided on both producer and consumer. **`AC-AFFECTED-CLOSURE` was discharged on its FIRST USE against the exact consumer that had reddened** (full Runtime lib `993 passed; 0 failed`, complete `px8f_buffer_native`, separate advancement run holding the named D3 frontier). **`ken-ci` auto-close did NOT fire AGAIN (`github: null`) — the Steward flipped `active` -> `merged` by hand. That is now TWICE on consecutive nodes; treat the auto-close as unreliable and check it every landing.** The trigger row's `#[ignore]` REMAINS, re-pointed to `RT-RESULT-CONTINUATION-BINDING-PROVENANCE` — recut AC met, not a defect. **NEXT NODE RELEASED: `RT-CHECKED-IH-FRESH-RESULT-ROUTE` flipped `ready` -> `active` and released by the Steward 2026-08-28** (T1, size M, `gate: none`, NO Decision required — the HS9 ruling is deductive); all five `depends_on` re-measured `merged` at release, implementer seat verified `gpt-5.6-sol • high` = correctly T1-provisioned. REPLACE, do not extend: the `CheckedIhFreshResultProducer` enum and its `CarriedLoopExitResult` arm are not retained in parallel, and inability to derive the forward route without prohibited authority is **HS10 — stop cleanly, never fall back to the merge**. `RT-RETAINED-TARGET-IDEMPOTENT-REDERIVATION` (`draft`, S/T2) remains QUEUED behind it. **The Adversary's M8 hunt on `bd4ddf213` (`evt_2kdx72vs884zp`) returned CLEAN core, strongly fail-closed, non-vacuous controls, and ONE grounded LATENT finding, now FILED as `RT-RESULT-CLOSURE-LIFETIME-CONTAINMENT-CONTROL` (`draft`, S/**T1**, QUEUED):** the escape/use-after-scope check at `aggregates.rs:6913` has ZERO negative coverage — neutering it to `Ok(())` leaves the whole suite green, because all nine population mutations abort at population-validate before any boundary crossing runs. **Not folded into the frozen D3 node**, which already carries three suppression axes and nine hard stops; a criterion added to a frozen node is one nobody reads. **T1 despite S**: the entire deliverable is the judgment the parent node failed TWICE — whether a control is causally honest — so `AC-NOT-MANUFACTURED` requires D1 go GREEN against a `:6913`-neutered tree, and a manufactured-only control is a HARD STOP, not a landing. **Steward correction carried into the frame:** the hunt cited the proofs call at "~3957"; measured it is `:3920`, and `~3957` is where `meet` is DERIVED from the escape analysis (`ActivationOwned` iff a child owner is `InvocationArena`) — that is the natural producer D0 must move, not the call site. **D3A+D3B REMAINS FROZEN** and needs its own separate explicit release. **`RT-CHECKED-IH-FRESH-RESULT-ROUTE` IS MERGED at `7d36d24f04678d3c9a2636fb06fd8c7aaf5dfb89` (2026-08-28), all EIGHT paths blob-verified by the Steward** — candidate `208309bb1`, Decision `dec_6zp6prvd0vwnt`, CI run `33175063437` green on all 15 checks. It landed behaviorally inert as framed. **`ken-ci` auto-close did NOT fire for the THIRD consecutive node** (`status: active`, `github: null` after a clean landing); the Steward flipped it by hand. Stop treating the auto-close as a fallback — check `status:`/`github:` at every landing. **The D3 FRAME DEFECT WAS FOUND AND CORRECTED BEFORE THE RELEASE, NOT AFTER**: the D3 Objective and D3B phase text still described the predecessor as TWO separable proofs (a `K`-inheritance proof plus a fresh-`R2`-destination proof), which is the exact architecture HS9 falsified — what landed is ONE fused `CheckedIhFreshResultRoute` whose source, tail edge, and sink COMPOSE. Uncorrected it would have pointed the ring at `CheckedIhFreshResultDestination`, which names a destination and never a producer, i.e. the latent false authority D3 is forbidden to consume. **The general form: a frame passage that survives the ruling invalidating it does not read as stale, it reads as authoritative.** The Steward's defect; corrected in the same route as the status flip. D3A+D3B is now UNBLOCKED on dependencies but still needs the SECOND EXPLICIT RELEASE, which is owed and not yet given. **THAT RELEASE IS NOW GIVEN: `evt_5errsa25a9vjh` (2026-08-28), the SECOND EXPLICIT RELEASE the frozen banner required.** It was issued only after the frame correction LANDED at `da95daadf04402809a278e66a546b70f6e99d738` — squashed and RETITLED ("docs: correct fresh-result route follow-up frames"), verified by blob identity on all five paths, not by subject. Releasing while the correction lived only in a convo post would have handed the ring the defective text; a correction is corrected when the TREE says so. Seat re-checked at kick time: `gpt-5.6-sol • high`, correctly T1. **CURRENT lane-1 work is therefore `RT-RESULT-CONTINUATION-BINDING-PROVENANCE` (D3A+D3B), ACTIVE.** **The Adversary's M8 hunt on `7d36d24f0` (`evt_39yvk4d78cfr`) returned CLEAN core with mostly strong controls and ONE grounded LATENT finding, now FILED as `RT-FRESH-RESULT-ROUTE-PAIRING-LEG-CONTROLS` (`draft`, S/T2, QUEUED behind D3 on file contention):** the `paired` predicate at `rt_parity_native.rs:1149` has FIVE substantive conjuncts and only ONE negative control — `CheckedIhFreshResultRouteObservationMutation` (`mod.rs:9618`) has arms exactly `{Exact, CoEmissionOnly}` and its recorder suppresses only `source_result_value`, so the SINK-half identity `header_input_value == ret_input_value` plus three siblings can each be deleted with the whole suite green. Deduction, not a run: dropping a conjunct only weakens `paired`, so `Exact`'s `all(paired)` cannot flip; and under `CoEmissionOnly` conjunct 1 is already false, so `all(!paired)` still holds regardless. LATENT — the positive genuinely reads production on both sides (`core.rs:12218` vs `:12636`, independent SSA reads), so a real sink bug still fails today; what is missing is proof the sink discriminator HAS power. **The reusable form: count a conjunctive predicate's controls PER CONJUNCT, never per predicate** — one arm makes the whole predicate look two-sided while four fifths is unpinned, and the summary sentence "the pairing proof has a negative control" stays true throughout. Framed with `AC-PER-CONJUNCT` in PREDICATE form (the five-row table is illustration, NOT the roster) because an enumerated roster is what produced the gap. **Steward verified every cited coordinate against the landed tree before filing**, including the enum doc-comment that scopes itself to one leg. |
| 2 | language | `LANG-INDEX-REFINEMENT-OMEGA-ARM` **MERGED** — both deliverables landed (D1 `e13df606a`, D2 `ef91b8225`, blob-verified); no D3. `LANG-MOD-CANONICAL-PAIR-PACKAGE` **MERGED `40e7f1199`** (blob-verified; its surviving `wp/` branch is the pre-squash remnant, NOT an unlanded candidate). **FO IS RECUT INTO A THREE-NODE REPAIR SEQUENCE (2026-08-27).** The landed D1 statement `fok_embedding_adequacy_statement` is REFUTED by an accepted capture-exploiting certificate (`evt_2yh515wg0mczy`); Architect `evt_6hx31xvw9tqs2` REJECTED the whole checker/derivation/adequacy interface as a semantic soundness gate, not repairable by finishing the proof. Sequence: `CORE-FO-CHECK-TREE-SORT-VALIDATION` (`ready`, predecessor) → `V3-FO-SORTED-EIGENPARAMETER-DERIVATION` (`draft`, NEW, the ATOMIC lockstep increment — never split it) → `V3-FO-EMBEDDING-ADEQUACY` D2a/D2b. `V3-FO-CHECKER-SOUNDNESS` and `V3-FO-SUBST-DEPTH-CONTROL` stay `merged` with superseded-banners. Route FO is fail-safe meanwhile (`prover.rs:562-604` withholds `Unknown`, never `Proved`) — the rejection invalidates the proposed THEOREM GATE, not the production verdict boundary. Only remaining candidate active: `LANG-MOD-CATALOG-COMPLETENESS` — its operative contract is **RECUT #3 (`4ffa8562c`, AC-CENSUS), NOT the "authorized partial / remainder held on Nat Decision" banner**, which is a lower, historical banner in the same file. The Nat hold is DISCHARGED: `dec_1kqwn6hdvn7d2` resolved and BOTH halves merged (`LANG-MOD-NAT-PROVIDER-INTERFACE`, `LANG-MOD-NAT-FLOOR-REALIZATION` at `d5c41ec1`). RELEASED `evt_7zr9t5k9d0ry8`, scope CORRECTED `evt_65h1skh3ryeae`: a 1106-line census artifact ALREADY LANDED at `027f6bf26` (2026-08-27 state `40e7f1199`, path `crates/ken-elaborator/tests/lang_mod_catalog_evidence_frontier.rs`), so the live deliverable is a DELTA MEASUREMENT against AC-C1..C6, not a fresh census. QUEUED behind FO node 1 under the one-WP rule (implementer's call, endorsed). The `wp/` branch is a landed remnant — retire, do not publish. HAZARD for any Omega-elimination work: the omega arm retains a bounded TWO-INDEX limitation as UNSUPPORTED (Architect `evt_7wbrfyvwv5517`) — single-index only; a multi-index need HARD-STOPS to Steward + Architect. NEXT: the z3 integration campaign (operator 2026-08-26). `LANG-MODULE-IMPORT-SYSTEM` COMPLETE. **OPERATOR RULING 2026-08-28 — LANE 2's NEXT OCCUPANT IS THE VERIFY RING, NOT LANGUAGE:** *"let lane 2 finish its current wp, then bring up verify on lane 2 to rework CI tests to make them run faster."* So when `V3-FO-SORTED-EIGENPARAMETER-DERIVATION` lands, lane 2's ring changes from language to **verify**, and its objective becomes `CI-NATIVE-PARITY-DURATION` (`draft`, S/T2, filed 2026-08-28). Flip it `ready` and release it to verify then. **This is a ring change within lane 2, not a fourth lane** — it does not disturb the three-lane structure, and the z3 campaign queues behind it. |
| 3 | foundation | Catalog-reuse modernization. Expressibility trial COMPLETE (3-lane feasibility PROVEN, operator 2026-08-26). Pilot chain DONE: `CAT-ORDER-PUB-EXPORT`, `CAT-GCD-REFACTOR`, `CAT-REUSE-CENSUS` all MERGED. CURRENT: `CAT-NAT-REUSE-CONSUMERS` (`active`) — six per-package increments. D1 `6ba6f6bef`, D2 `428ea1188`, D3 `9de02daff`, D4 `100dd6afa`, D5 `aa0e5cc44` MERGED. **D6 (`Derived.ken.md`) RELEASED `evt_6yetvf5fvv6nm` and HELD on the operator's Arm A/Arm B trust-surface decision** — the risk increment, LAST by design; D6 closes the batch. **Next node after it is FRAMED: `CAT-DERIVED-PUB-EXPORT` (`draft`), the group-4 provider prerequisite.** **A consuming TEST FIXTURE's root set is part of an increment's path set here** — established by D1 (cc6a/cc7/cc8) and D2 (cc2/cc3/cc4/ds9/d0), ruled for D4 at `evt_1b31assx1ktg8`/`evt_6snwh0xy60jh8`/`evt_2r8cavz7b1bms`. Carry that authorization INTO the D5 release so it does not hard-stop for it again. |

**Lane 1 — runtime (priority).** The native carried-value program
`RT-NATIVE-CARRIED-VALUE` (Architect frame `evt_9kat78d438cb`): a finite
compile-time-known defunctionalization carried at runtime as discriminant only.
M-series seats. M6 (Track-1 D0 `RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION`), M4
(`RT-CLOSURE-BOUNDARY-RESIDUAL`), and M3 (`RT-CARRIED-IH-DISPATCH-SITEOP`) merged.
M3's crossing exposed two successors; the first was recut 2026-08-25 after three
consecutive Architect hard stops on a shared predicate (a downstream semantic
classification used as upstream producer/provenance authority): the ExitCode WP
`RT-EXITCODE-FAILURE-PAYLOAD-TRANSPORT` is `closed`/falsified (Architect
evt_1vhmndq7fscd1) and REPLACED by `RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE`
— an owner-bound probe of the causal dynamic-constructor dispatch
residual — which is now **MERGED**. The `-3` reporter alias, split out as
`RT-UNIT-FAILURE-STATUS-PROVENANCE`, is **MERGED** too.
`RT-RETAINED-UNIT-CALL-TARGET-DERIVATION` (`draft`) stays distinct. The NHC chain
+ `RT-BACKEND-MODULE-SPLIT` are drained/merged.
Architect is required reviewer on the M-series — the Architect-heavy lane.

**CURRENT lane-1 work (measured 2026-08-27):**
`RT-CHECKED-IH-GENERATED-ENTRY-ACCESS` (`active`), the complete planner-owned
predecessor that replaced the repeated last-gap decomposition. It has taken
**seven** Architect hard stops (HS5-HS11); the last five share one root — a
property proved in one frame carried into another without re-derivation — and the
frame defects were the Steward's. HS10 replaced the partial governed-only
projection map with ONE TOTAL `Governed`/`NonGoverned` admission map (landed
`61c2fefa0`). **HS11 then falsified the once-only premise itself**: one installed
certificate/key carries member set `{A,B}` and every arrival at that static key
consults it, so repeated governed arrivals are LAWFUL. `AC-ADMIT-VISIT-ONCE` was
retired for `AC-ADMIT-PER-ARRIVAL` (three bags incremented independently per
installation and per call key, pointwise `raw = admitted = validated > 0`, no
literal multiplicity pin) plus `AC-ADMIT-ARRIVAL-MUTATIONS`. Keep the three
cardinalities apart: certificate (one key per STATIC coordinate), arrival
multiplicity (zero-or-more), per-arrival action ("once" = per arrival, never per
compile/installation/key/certificate). HS11 recut landed `fec63506a` (blob
`1ae3e449a8f2`, Architect APPROVE `evt_2wvn3szecym9f`) and was re-released
`evt_1mgb3zbskwbg3`. It blocks
`RT-RESULT-CONTINUATION-BINDING-PROVENANCE` (`active`), whose **D3A+D3B stays
FROZEN and needs its own separate explicit release** — neither the frame landing
nor the predecessor landing authorizes the consumer. Next mandatory §1a/§1b
research-advisory trigger on this node is **stop 12, NOT 10** (Architect
`evt_2s144kdddyckn`, verbatim — HS9 consumed the ninth-stop advisory).

**Lane 2 — language. CURRENT (measured 2026-08-27 at `ef91b8225`):**
`LANG-INDEX-REFINEMENT-OMEGA-ARM` is COMPLETE — both deliverables landed (D1
`e13df606a`, D2 `ef91b8225`), and it has no D3. The live node is
**`V3-FO-EMBEDDING-ADEQUACY` D2**, re-released `evt_52vwvmn0ee859` after that
predecessor gate was fully discharged; its own held evidence commit
`3f687a460` is transition evidence and NOT a candidate, and the two Architect
rulings `evt_1wnk1ek4s8sgj` + `evt_pw69nxgxn99j` are CUMULATIVE — neither
supersedes the other. `LANG-MOD-CATALOG-COMPLETENESS` and
`LANG-MOD-CANONICAL-PAIR-PACKAGE` also `active`. The z3 integration campaign is
NEXT, once these drain. The prelude recut below is DONE and is history, not
current work.

**THE FO RECUT (2026-08-27), which supersedes the FO paragraph above.** The
Architect REJECTED the current FO checker/derivation/adequacy interface as a
semantic soundness gate (`evt_6hx31xvw9tqs2`, base `ef91b8225`) and ruled it
**not repairable by finishing the current proof**. Cause: both Rust and Ken give
`ForallRight` an arbitrary eigenterm, the guard checks only non-occurrence in the
conclusion, and the shared untyped de Bruijn substitution installs a fresh
`Bound(k)` across world AND object binders. **Freshness is not eigenparameter
provenance.** `fok_checker_soundness` is a STRUCTURAL REFLECTION theorem for the
relation it is given, and that relation carries the same permissive rule — so a
Rust-side guard alone does not close the class.

**Steward disposition: three nodes, not four**, confirmed against the frames
rather than asserted:

1. `CORE-FO-CHECK-TREE-SORT-VALIDATION` — `ready`, PROMOTED from optional
   hardening to PREDECESSOR. Its old "Why this is hardening and NOT a soundness
   fix" section is FALSIFIED and removed (it was keyed on formula reachability;
   the refutation is on the certificate axis). **The tag-vs-pass fork is now
   RULED: validation pass, no sort tag on the target datatypes** — a carried tag
   moves the datatype `fok_checker_soundness` is stated over and would collapse
   the sequence into one atomic frame. Discovering the pass is insufficient is a
   HARD STOP to the Steward, not something to absorb.
2. `V3-FO-SORTED-EIGENPARAMETER-DERIVATION` — NEW, `draft`, **ONE ATOMIC
   INCREMENT and it cannot be split** (envelope item 5 lockstep: Rust
   checker/search + Ken checker + `FokDerivation` constructors + reflection
   proofs together). It SUPERSEDES the relation `V3-FO-CHECKER-SOUNDNESS` proved
   and SUBSUMES `V3-FO-SUBST-DEPTH-CONTROL`'s control obligation as its `AC-4`.
   Both of those stay `merged` — their deliverables did land — and carry banners.
3. `V3-FO-EMBEDDING-ADEQUACY` — D2 recut into `D2a` (re-measure whether the
   landed statement text survives the corrected relation; may hard-stop) + `D2b`
   (prove it). **Do not pre-decide `D2a`**: `fok_classically_valid` is
   `fok_derives (⊢ q)`, so correcting `FokDerivation` changes what the statement
   MEANS without necessarily changing what it SAYS.

**Say this whenever quoting "REJECT": production is unaffected.** The ruling
invalidates the proposed theorem gate, not the production verdict boundary.

> **Carry this into any lane-2 work that eliminates index-dependent Omega
> evidence.** The omega arm fixed decisions 1-3 but explicitly did NOT close the
> multi-index case: a bounded TWO-INDEX goal-restoration limitation is retained
> as unsupported and unrepaired (Architect `evt_7wbrfyvwv5517`). The supported
> transition is the single-index branch-goal witness. Both Type and Omega
> multi-restoration cases still reject, and the Type behaviour is unchanged for
> all inputs — the gap predates the omega arm, which merely exposed it. A ring
> that needs multi-index support HARD-STOPS to Steward + Architect; it does not
> repair the elaborator from a downstream node.

Operator 2026-08-26 (direction, unchanged): "first launch the internal-provision
prelude recut and finish that effort, then return to the z3 integration
campaign." That recut — `LANG-MOD-PAIR-FLOOR-PROVIDER`, a Steward-owned spec WP
(Architect shaping `evt_7d0ecgkd8ate3`), frame landed `c1945c6fb`, spec ring
anchor `evt_6yc0k921tf3j` — **is MERGED as `8f3b6fd2`**, so the operator's "then"
clause is the live half. It generalizes
prelude membership to ONE internal-provision arm (kernel or compiler origin),
admits Pair as the first compiler-provided member reusing the four existing
compiler-installed Pair `GlobalId`s, and supersedes the exact-nine boundary of
`LANG-MOD-PAIR-STRICT-BOUNDARY`. Its build successor
`LANG-MOD-CANONICAL-PAIR-PACKAGE` (`depends_on` repointed) realizes the split
inventories after the spec lands. After the recut lands, lane 2 returns to the
z3 integration campaign (verify/FO-checker resume). The module/import history
below is DONE.

**Module/import campaign `LANG-MODULE-IMPORT-SYSTEM` (history — essentially
COMPLETE).** This was the lane's prior objective and it UNBLOCKED lane-3
foundation. Framing is
COMPLETE (Architect 4-WP decomposition `evt_hpnhqy1ex286`; spec-surface merged
`def16ecf4`). Member-WP state (re-measure before acting): WP-1
`LANG-MOD-LOADER-ENTRY` merged; WP-3 `LANG-MOD-PUB-ELIGIBILITY` merged; WP-4A
`LANG-MOD-CATALOG-REALIZATION` merged; `LANG-MOD-CATALOG-COMPLETENESS`
(Component B) `active` (authorized partial; remainder held on the Nat Decision
`dec_1kqwn6hdvn7d2`). WP-2 `LANG-MOD-STRICT-RESOLUTION` (the strict root-loaded
resolution soundness core) is **`merged`** — its D0 census (`c64c62190`) + D1
enforcement (`5a74301f4`) shipped inside the Component A/B realization; a
2026-08-25 Steward re-release off its stale `ready` node was withdrawn on the
implementer's hard stop (no non-duplicative delta). **So module/import is
essentially complete; the remaining member work is the Nat prerequisite +
Component B's remainder + `CAT-GCD-REFACTOR`.** The Nat Decision
`dec_1kqwn6hdvn7d2` is RESOLVED (2026-08-25): the operator ruled the prelude
membership rule (`30-taxonomy §4`) itself the defect and superseded the
provider-registry mechanism — Nat's home is PRELUDE-FLOOR MEMBERSHIP (amend the
general rule to a bootstrapping arm; admit the existing kernel {Nat,Zero,Suc}
into the strict floor, reuse identity). Reframed into a spec WP
(`LANG-MOD-NAT-PROVIDER-INTERFACE`) + a build WP
(`LANG-MOD-NAT-FLOOR-REALIZATION`). **BOTH ARE NOW `merged`** (the build half at
squash `d5c41ec1`, blob-audited), measured 2026-08-27 — this row previously said
`ready, release FIRST` and that was two days stale. The chain is UNBLOCKED and
there is nothing here to release. Do NOT re-release WP-2 off its node — re-measure
the tree.

The earlier lane-2 objectives are DONE and are history, not current work:
`V3-FO-CHECKER-SOUNDNESS` is `closed` (FO checker-soundness theorem complete,
both fragments); `CI-Z3-BASE-IMAGE` + the FO/Z3 chain landed;
`KERNEL-CONV-TRUNC-CONGRUENCE` merged. The residual FO frontier
`V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY` (rotation fork) is filed and separate; it is
not the module/import priority. Verify/kernel are reviewers here, not a separate
active lane.

**Lane 3 — foundation: catalog-reuse modernization campaign.** The expressibility
trial (five CAT algos — `CAT-SORT`/`CAT-GCD`/`CAT-DEQUE`/`CAT-BSEARCH`/`CAT-VEC`,
charter `docs/program/wp/foundation-expressibility-trial.md`) is COMPLETE — all
merged. Its purpose (measure whether three lanes overload the Architect) is
DISCHARGED: the operator ruled 2026-08-26 that feasibility is PROVEN and directed
the lane to continue.

The lane's new objective is the **catalog-reuse modernization** campaign
(operator 2026-08-26; charter `docs/program/wp/catalog-reuse-modernization.md`):
now that the prelude is expanded and module imports work, rework catalog packages
along three axes — (a) remove defs redundant with the prelude, (b) import canonical
tools from sibling modules instead of reimplementing, (c) restructure files
top-down. Census-first, conservative/risk-tagged depth, lane-3 priority.

Current state (measured 2026-08-27 against `origin/main` `61c2fefa0`;
re-measure before acting):
- PILOT CHAIN: **DONE.** `CAT-ORDER-PUB-EXPORT`, `CAT-GCD-REFACTOR` and
  `CAT-REUSE-CENSUS` are all **MERGED**. The per-package recipe is proved and the
  catalog-wide inventory that sizes the rework is landed. (The CAT
  reuse-remediation was never blocked on the `LANG-MODULE-IMPORT-SYSTEM` umbrella
  in the end — the import + pub-export capability it needed landed, and the Order
  half went through the pilot prerequisite.)
- CURRENT BATCH: `CAT-NAT-REUSE-CONSUMERS` (`active`) — the census's first scoped
  rework batch, six independently-releasable per-package increments. D1
  `6ba6f6bef` (`Arguments.ken.md`), D2 `428ea1188` (`Diagnostics/Core.ken.md`),
  D3 `9de02daff` (`Parsing.ken.md`), D4 `100dd6afa` (`Formatting/Doc.ken.md`)
  and D5 `aa0e5cc44` (`Parsing/Cursor.ken.md`, nine paths blob-verified) are all
  MERGED. **D6 (`Derived.ken.md`) RELEASED `evt_6yetvf5fvv6nm`** and in build —
  the risk increment, LAST by design: its `AC-PROP` can hard-stop to
  spec/Architect, and that is a payoff, not a setback. D6 closes the batch.
- **NEXT RELEASE AFTER D6 IS A PROVIDER PREREQUISITE, NOT ANOTHER CONSUMER
  BATCH** (Steward determination 2026-08-27, measured against census §4.2/§4.3).
  The census proposed seven low-risk groups; `CAT-NAT-REUSE-CONSUMERS` drained
  groups 2 and 3. **Every one of the five remaining groups is blocked on a
  provider that is not yet public, a module that fails standalone elaboration,
  or both.** Groups 1 and 7 sit behind modules in the §4.3 standalone-failure
  set (`LawfulFunctors`, `BytesKeys`, `Cursor`); group 5 sits behind the
  `Nat.Order` atomic owner-migration wall. **Group 4 (derived-list reuse) is the
  only one whose provider — `Data.Collections.Derived` — is NOT in that failure
  set**, so the next node is a `CAT-ORDER-PUB-EXPORT`-shaped pub-export
  prerequisite over exactly `list_append`, `length`, `reverse`, `concat_map`.
  **NOW FRAMED as `CAT-DERIVED-PUB-EXPORT` (`draft`, S/T2, 2026-08-28)**,
  measured at `35b9d3fa1`: `Derived.ken.md` has **zero** `pub` declarations and
  the in-scope names are bare `fn`. The landed spelling to copy is
  `pub fn <name>` (`Nat/Order.ken.md:49,59,69,79`; `Nat/Arithmetic.ken.md:18,24`)
  — copy the landed precedent, not spec prose. It stays `draft` behind
  `CAT-NAT-REUSE-CONSUMERS` (encoded in `depends_on`, since a prose-only gate
  gates nothing): D6 edits this exact file, so the dependency is CONTENTION and
  fixed-input staleness, not semantics. Flip `ready` and release once D6 resolves,
  with D0 re-measuring at the post-D6 SHA.
  > **SCOPE CORRECTED — this row previously said "exactly `list_append`,
  > `length`, `reverse`, `concat_map`", and that was consumer-derived.** Those
  > four are what census group 4 CONSUMES. Census §4.2's
  > `Data.Collections.Derived` EXPORT set is nine names, and the node takes SIX —
  > §4.2 minus the three §4.3 ungroups as higher-risk attached-law ownership
  > (`Perm`, `insert`, `sort`). The four-name scope silently dropped
  > `eq_from_ord` and `count`. **The general form: when a provider node's scope
  > comes from what one consumer needs rather than from what the provider is
  > recorded as exporting, it under-covers, and the leftover names come back as a
  > second WP re-contending the same file.**
  > **The `CAT-ORDER-PUB-EXPORT` split hazard is MEASURED ABSENT here**, which is
  > why this is S/T2 and not a migration: census row `D` reads `[ok] standalone
  > exit 0; no provider ownership error`, `ambient=-`, and §4.3 states the
  > orphan/foreign-attached predicate is measured only for the Nat-order
  > component. A standalone or ownership failure appearing at D0 is a HARD STOP,
  > not something to patch through — that is how Order's split was earned.
  > Note `reverse` carries `proof involutive`; an attached proof on an exported
  > fn is NOT a gap (`Arithmetic` pub-exports `add`/`mul` with attached proofs).
- **A CONSUMING TEST FIXTURE'S ROOT SET IS PART OF AN INCREMENT'S PATH SET here,
  and the frames do not say so.** D1 landed four paths (`Arguments.ken.md` +
  cc6a/cc7/cc8) and D2 landed seven (incl. cc2/cc3/cc4/ds9/d0), so this is
  established and twice-reviewed. D4 nonetheless hard-stopped at `AC-STOP`
  because its frame under-specified the path set — a Steward defect, ruled at
  `evt_1b31assx1ktg8`. **Carry the authorization INTO the D5 release.** State it
  as a PREDICATE, never a per-file module list: *each authorized fixture may
  roots-load whatever its root set needs to reach the same semantic assertions it
  reached before, and no more.* An enumerated list was outrun twice in one
  increment — first by a chained `UnboundName` the first measurement could not
  see (`evt_6snwh0xy60jh8`), then by an inventory roster naming the deleted defs
  (`evt_2r8cavz7b1bms`, authorized WITH a required canonical-provider pin, on the
  D3 precedent). Weakening an assertion that tests preserved behaviour remains a
  hard stop; removing an inventory entry naming a definition the increment
  deletes does not.
- **AFFECTED-TARGET CLOSURE, NOT THE DIFF-TOUCHED SET — a Steward frame defect,
  corrected 2026-08-27 (`evt_4apxf4vgbmb1e`).** D6 respin `dd595213` passed 28
  diff-touched Rust targets 170/170 and CV still found a candidate-caused red:
  `lang_mod_catalog_completeness.rs:97`, a pre-existing consumer-closure oracle
  in a file the increment never touches. **A diff-touched target set is blind to
  exactly the consumers an increment breaks by changing a CLOSURE rather than a
  file.** Every remaining increment must re-run the COMPLETE AFFECTED-TARGET
  CLOSURE: every target that loads any module whose closure the increment
  changes, diff-touched or not. QA applied the criterion my frames gave it; the
  criterion was wrong. Write the closure rule into every remaining frame.
- **NAMED IMPORT DOES NOT BOUND THE CLOSURE.** D6 imports
  `Data.Numeric.Nat.Order (min, sub)` — arithmetic only — but root-loading is
  MODULE-granular, so `Nat.Order`'s package-local `data OrdResult = Lt | Eq | Gt`
  (`Order.ken.md:47`; its own §1 says "`OrdResult` remains package-local") lands
  in `env.globals` anyway. It is genuinely DISTINCT from
  `Core.Logic.OrdResult.OrdResult`, never an alias — asserting those IDs equal is
  FALSE. Any consumer importing from a package with package-local datatypes
  inherits them. **Groups 4 and 5 will hit this.** Filed as a campaign
  observation, NOT a blocker.
- **PROXY vs PROPERTY — the reusable form of the D6 ruling.** The failing
  assertion `!globals.contains_key("Data.Numeric.Nat.Order.OrdResult")` was never
  the property; it was a PROXY that tracked "Derived's compare uses the canonical
  OrdResult" only while Order sat outside the closure. Once Order is legitimately
  in the closure the proxy measures NOTHING, and keeping it blocks a correct
  increment without preserving anything. **Replacing a dead proxy with a direct
  pin of the property is a STRENGTHENING, and is authorizable; bare deletion is
  not.** Authorized WITH conditions on the D3 precedent (`evt_2r8cavz7b1bms`):
  pin the real types/bodies to the canonical ID, assert the Order-local ID PRESENT
  and DISTINCT, keep every `Data.Collections.Derived.*` absence and Core.Compare
  ownership check intact, and **prove the new control discriminates BY MUTATION**
  (compare-uses-Order-local must red; identity-collapse must red). The mutation
  condition is load-bearing: QA blocked the prior respin for a control that could
  not fail, and a replacement oracle shipped on its own say-so repeats that defect
  one level up.
- MEASURED CARRY-FORWARD for every remaining increment (Adversary
  `evt_5sw5w9w4jj35z`, M8 on D2): importing direct from a canonical owner makes
  the consumer transitively inherit that provider's OWN un-migrated ambient
  surface. `AC-AMBIENT-DELTA` asks for a measurement and a report, **not a
  shrink** — census growth there is inherited debt, not a defect in the
  increment. Note also that a package sitting in the RESIDUAL bucket (e.g.
  `Parsing` at `UnresolvedCon(SourceId)`) has that inheritance MASKED rather than
  absent, so report the exact post-edit residual rather than an expected vector.

Reviewers: foundation-qa + conformance-validator (catalog implementation
standard); a genuine design/spec gap (eligibility, attached-proof ownership)
HARD-STOPS to spec/Architect — a gap finding is the payoff. The three-lane
Architect-burden question is now ANSWERED (feasible), so this lane runs as normal
directed foundation work, not a probe.

## Not a lane

**Doc track** runs concurrently but is contention-free (`library/`, `agent/`,
not `crates/`) — it is the standing exception, not a lane (`CLAUDE.md`).

## Roster history

- 2026-08-28 (at `bb33dfb71`): **OPERATOR RULING — lane 2's ring changes to
  verify once its current WP lands. Structure UNCHANGED: still three lanes.**
  Verbatim: *"let lane 2 finish its current wp, then bring up verify on lane 2 to
  rework CI tests to make them run faster."* Lane 2 stays lane 2; what changes is
  which ring occupies it and what its objective is. Filed
  `CI-NATIVE-PARITY-DURATION` (`draft`, owner verify, S/T2) as that objective,
  held `draft` until `V3-FO-SORTED-EIGENPARAMETER-DERIVATION` lands.
  > **THE MEASUREMENT THAT PRODUCED THE RULING, and the part worth carrying: two
  > of the three floors are invisible from the job listing.** Operator asked for
  > CI under 20m, ideally under 10m; run `33192361977` measured 28m55s.
  > **Floor 1** is one test: `checked_ih_generated_entry_confluence_and_route_-
  > mutations_reject` loops 39 subprocess mutations serially and costs 1299.983s
  > as ONE nextest scheduling unit, so wall-clock cannot go below ~1300s while it
  > exists. **Floor 2 is the one that defeats the obvious fix**: total CPU across
  > the binary is 4171.65s, and on a 4-vCPU runner that floors at 17.4m no matter
  > how finely it is split — so splitting alone lands at ~18m, under the ceiling
  > but nowhere near the target, and **`--partition` is what reaches 10m while
  > being INERT until the split lands** (it cannot subdivide a single 1300s
  > test). **Sequencing matters more than either step: D2 measured before D1
  > reads as "partitioning does not help", which is false and would retire the
  > actual remedy.** Floor 3 is shard imbalance (18m vs 11m) from partitioning by
  > test rather than by duration.
  > **ALSO ANSWERED, because the operator asked and the answer was not obvious:
  > ignored tests ARE executed and their failures are NOT blocking.**
  > `ci.yml:157` runs `--run-ignored=only` under `set +e`; the sweep ran 33 rows
  > at 40-75s each, most FAILING, for a 10m non-blocking job. The operator's
  > proposed short-circuit is therefore live — **but a bare one would make the
  > sweep read green forever, and noticing when an ignored row STARTS passing is
  > the sweep's entire purpose.** So it is built as a REGISTERED short-circuit
  > against the existing `.github/ignored-test-exemptions.toml` `readmission`
  > field, enforced by mutation. **The general form, and this is the third time
  > it has come up: making an instrument cheaper by removing what it measures is
  > not an optimization.**
  > **`cancel-in-progress` SURFACED TO THE OPERATOR, NOT ACTED ON.** `ci.yml:17-19`
  > groups on `github.ref`, so every push to `main` kills `main`'s previous
  > in-flight run, and **a cancelled run neither failed nor passed**. At a
  > 28-minute CI, any landing cadence faster than that leaves `main` permanently
  > unverified — which is what happened between `31258f403` and `bb33dfb71`,
  > several cancellations caused by my own doc routes. Shortening CI narrows the
  > window and does not close it. Whether `main` keeps `cancel-in-progress` is the
  > operator's call.
- 2026-08-28 (at `50369d47b`): **no roster change. All three lanes moved in one
  stretch, and two of the three moves were mine to make.**
  - **Lane 2 ROUTED.** `CORE-FO-CHECK-TREE-SORT-VALIDATION` node 1 candidate
    `57d209fcacab914c8616199bd01bbeb698480266` cleared all three gates on the
    exact SHA (Architect `evt_thwayapdvwds`, Language QA `evt_282w3vzhacddy`,
    Adversary CLEAN `evt_3txpdz4gtw81r`). Decision `dec_41xx97vpph08m` resolved
    and routed `evt_64qqd97hf8kbq`. I verified the shape against the DECLARED
    range rather than the handback: tree, merge-base, three commits, four paths,
    `+615/-76` all exact, changed-path intersection with `origin/main` empty, M3
    clean. **M8 is pre-discharged on this SHA alone** — the hunt was requested
    pre-merge; a respin is uncovered.
    > **THEN CI WENT RED ON THE APPROVED OBJECT. `dec_41xx97vpph08m` IS SPENT;
    > PR #3041 is an orphan.** Run `33183516806`, `test shard 3/4`:
    > `forall_world_right_constructor_and_checker_reject_a_nonfresh_eigenparameter`
    > panicked at
    > `crates/ken-elaborator/tests/v3_fo_checker_soundness_d1a_rule_correspondence.rs:40`
    > — `d1a_world_checker_accepts` failed to kernel-check (expected
    > `(((g225 Dg3) ((g691 g771) g777)) cg4)`, found `g1`).
    > **Candidate-caused, attributed and not assumed:** main is GREEN at
    > `50369d47b`, and that test file is UNTOUCHED by the candidate while
    > `include_str!`ing at `:24` the exact `FoKripke.ken` the candidate edited.
    > **An increment that changes a CLOSURE rather than a FILE breaks consumers
    > no diff-touched target set can see.**
    > **THIRD LANE, SAME DEFECT, MINE AGAIN.** `AC-AFFECTED-CLOSURE` was
    > diagnosed in FOUNDATION 2026-08-27, carried to RUNTIME 2026-08-28 after it
    > cost that lane a red merge, and **never carried to LANGUAGE** — when this
    > candidate reddened it sat in three runtime frames and zero language frames.
    > Now folded into `CORE-FO-CHECK-TREE-SORT-VALIDATION` durably, not left in
    > gate prose (a criterion living only in gate prose is the `AC-EXCLUSIVE`
    > failure). **A criterion repaired in one lane is not thereby repaired in the
    > fleet, and repairing the lane where it bit is the easy half that feels
    > complete.** Neither gate was negligent: both ran the targets my frame named,
    > and the frame named the wrong set.
  - **Lane 1 took HARD STOP 10 on D3A+D3B**, cleanly, at `da95daadf` with no
    commit, candidate, or QA, branch freed (`evt_4wkf1n81x7zq`). Routed to the
    Architect by `runtime-leader` `evt_3yy045bpks910`, picked up
    `evt_3zqmkf8bgngfs`. The measured gap: every real tail callee is
    `ComputationalRecursorClosure { recursive_unit_body: None }` and the exact
    lowering arm returns the carried residual word unchanged
    (`lowering/source.rs:4512-4515`) — `CheckedIhCapturedEnvironment`, not fresh
    `R2`. The static route certificate derives; tail execution has no authorized
    K-application source. Every prohibited alternative was enumerated and NOT
    taken.
    > **I HAD THE ADVISORY OBLIGATION WRONG IN MY OWN RESUME CHECKPOINT, and the
    > ring was right without me.** The checkpoint asserted *"HS10 mechanically
    > triggers the mandatory Research advisory before any Architect ruling."* It
    > does not. `§1a`/`§1b` is the **Architect's** counting procedure
    > (`architect.md`), and the Architect's own ruling `evt_2s144kdddyckn` puts
    > the next mandatory trigger at **stop 12, not 10**; HS9 consumed the
    > ninth-stop advisory. The D3 frame's HS10 stop rule carries no advisory
    > clause either. **I had carried the HS8 banner's "the next stop is 9 and it
    > MECHANICALLY triggers the advisory" forward one stop.** Had I acted on it I
    > would have inserted a gate the law does not require, in front of a ruling
    > the lane was already waiting on. **The general form: a trigger stated as
    > "the NEXT one" is a fact about a specific count, and it expires the moment
    > that count is consumed — re-read it at the next stop instead of
    > incrementing it.**
  - **HS10 IS RULED — `evt_1ckwtvwe23e3e`, and it does NOT mint a predecessor.**
    The stop was VALID but its inference was wrong: `recursive_unit_body=None`
    means **Tail variant**, not **no K application**. The landed route
    constructor partitions explicitly (`aggregates.rs:5508-5578`), the Tail
    validator (`:5765-5805`) IS the authorized application protocol, and lowering
    already executes it (`source.rs:4424-4469`, `:4912-4984`, `:1528-1567`). The
    carried word at `source.rs:4512-4515` is the SEED to that installed
    continuation, never the completed result. **The defect was THIS FRAME's
    uniform D3A recipe applying one Direct mechanism to both route variants.**
    Add no predecessor. **Steward owns the amendment + a fresh explicit release
    against the amended frame blob; runtime resumes only then.** Amendment
    applied this route: Objective, phase structure, D3A deliverable, reviewers,
    `AC-D3A-APPLICATION`, `AC-D3A-PAIRING`, `AC-D3-TRIPLE-SUPPRESS` axis (ii),
    `AC-D3-ATMOSTONCE`, and Sequencing all made ROUTE-VARIANT-SPECIFIC.
    **Next mechanical research trigger remains HS12** — this confirms the
    correction below.
    > **THREE MORE STALE PASSAGES FOUND WHILE AMENDING, and the count is the
    > point.** `fresh-`R2`-destination projection` — the two-proof language HS9
    > falsified — still sat in the D3B deliverable, `AC-D3B-RESULTFLOW`, and the
    > Reviewers line. **I corrected exactly two instances of this on 2026-08-28
    > and believed the correction complete.** A `grep` for the phrase, which
    > costs one command, would have found all five then. **Correcting the
    > instances you were looking at is not correcting the defect; enumerate the
    > phrase before declaring the class closed.**
  - **Lane 1 successor NOT released, and the near-miss is worth recording.**
    `RT-RESULT-CLOSURE-LIFETIME-CONTAINMENT-CONTROL` (S/**T1**) was flipped
    `draft` -> `active` while D3 sat blocked at HS10 with the seat idle — chosen
    over its two S/T2 siblings on a `§4h` capability match, contention
    re-measured clear on `aggregates.rs`. **The HS10 ruling then unblocked D3,
    which is lane 1's main line and takes the seat, so the flip was REVERTED to
    `draft` before any release was posted.** Nothing reached the ring.
    > **What made the reversal free was holding the release post until the frame
    > correction LANDED.** Had I posted on the strength of my own working-tree
    > edit, I would have had to retract a release. **The rule that a correction
    > is corrected only when the tree says so also bounds the blast radius of
    > being wrong about priority.**
    > That node needed THREE stale-passage corrections of its own before it was
    > releasable — a banner forbidding the ring to pick it up, a contention
    > section calling a merged node "in flight", and a sequencing line calling it
    > `draft`. Those corrections were kept; only the status flip was reverted.
    > It also claimed the two surfaces were *"expected disjoint"*. **They were
    > not** — the route node's landing touched `aggregates.rs`. The merge removed
    > the conflict; the disjointness never existed. An unmeasured expectation had
    > been sitting in the frame as though it were a finding.
    >
    > **THE STANDING RULE THIS EARNS, across three consecutive nodes now:
    > re-reading a queued frame's OWN banners against today's tree is an
    > unconditional step of releasing it, not a thing to remember.** The passages
    > that go void while a node waits are exactly the ones written to describe
    > what it was waiting on.
  - **Lane 3 was STRANDED and is moving again.** `foundation-qa` posted a PENDING
    verdict on D6 Arm-A `4bebdec09` at 14:08 and then ENDED ITS TURN — pane idle,
    branch switched back to home, the closure/substitution/mutation work it had
    itself named left unowned, while `foundation-leader`'s status still read "QA
    active". WIP audit fired `evt_4va4qwkakd8c7` and the pane roused directly;
    seat confirmed `Working`. **The thread is what resolved this — an idle pane
    cannot separate "handed off" from "stalled", but a leader and a QA describing
    the same seat differently can.**

- 2026-08-27 (fifth refresh, at `00e66312b`): **no roster change.** Lane 1's
  `RT-CHECKED-IH-GENERATED-ENTRY-ACCESS` LANDED (15/15 paths blob-verified by
  me, not taken on the lieutenant's report), which discharged the last of the
  four `depends_on` on `RT-RESULT-CONTINUATION-BINDING-PROVENANCE` and let me
  make the owed explicit D3A+D3B re-release. Lane 3 D5 landed and D6 went out.
  Recorded the post-D6 provider-prerequisite determination above.
  > **LESSON — I found the SAME defect in my own repaired instrument, one level
  > down.** Yesterday I rewrote watchdog step 2 because an AGGREGATE landing
  > count read 5 while runtime landed ZERO, hiding a dead lane for 16 hours.
  > The replacement was per-lane paths — and its lane-2 path
  > `':/crates/ken-elaborator/'` **catches lane-3 CAT work**, because the
  > foundation increments repair elaborator test fixtures. It read 5 for lane 2
  > when the true lane-2 count was 2. **A narrower instrument is not thereby a
  > correct one**: I fixed the granularity and never asked whether the new
  > buckets were disjoint. Repaired by requiring every hit be ATTRIBUTED to a
  > lane by subject before counting — not by widening, which is what caused it.
  > The general form: when you repair an over-broad measurement by subdividing
  > it, the subdivision inherits the original's blind spot unless you prove the
  > new partition is actually disjoint over the population you are counting.
- 2026-08-27 (fourth refresh, at `ad36b0fcd`): **no roster change — citation
  re-measurement, plus one release it produced.** The Nat chain this file called
  `ready, release FIRST` was in fact merged on BOTH halves, two days stale. That
  correction discharged the stale Nat hold on `LANG-MOD-CATALOG-COMPLETENESS`.
  Released `evt_7zr9t5k9d0ry8`, then CORRECTED at `evt_65h1skh3ryeae` — see the
  second lesson below, which is the more important of the two.
  > **LESSON 1 — the row cited the WRONG BANNER, which is not the stale-row shape
  > two entries down.** This node carries four stacked banners, and the one my row
  > quoted ("authorized partial; remainder held on the Nat Decision") is a
  > HISTORICAL banner sitting BELOW the operative RECUT #3 in the same file. **A
  > node with stacked recut banners has no single "the frame" to read** — reading
  > top-down and stopping at the first authoritative-sounding block gets you a
  > superseded contract that still reads perfectly. Find the banner that names
  > itself OPERATIVE and says what it supersedes, then check nothing below it is
  > newer.
  > **LESSON 2 — I released it as unstarted, and it was not.** I checked for prior
  > work with `git log --grep=<node-id>`, got two hits (neither the census), and
  > concluded the ring had produced nothing. In fact `027f6bf26` landed a
  > 1106-line evidence-frontier artifact on 2026-08-25, advanced since at
  > `40e7f1199`. **Its commit subject — "LANG-MOD Component B evidence frontier
  > partial" — does not contain the node id**, so the grep could not see it. This
  > is the `ZERO = NAME` lesson firing on my own instrument: a zero-hit census is
  > evidence about a NAME, never about a mechanism. **When checking whether a
  > deliverable exists, grep the DELIVERABLE'S PATH in the tree, not the node id
  > in commit subjects** — `git log -- <path>` would have shown it instantly, and
  > the artifact was sitting in `crates/ken-elaborator/tests/` the whole time.
  > The surviving `wp/LANG-MOD-CATALOG-COMPLETENESS` branch is a landed pre-squash
  > remnant (blob `541ff8e7d` identical to what `027f6bf26` landed) and is now
  > STALE against main; retire it, never publish it.
- 2026-08-27 (third refresh, at `bd68352bb`): **no roster change.** Corrected two
  nodes still reading `active` that had in fact MERGED —
  `LANG-MOD-CANONICAL-PAIR-PACKAGE` (`40e7f1199`) and
  `LANG-INDEX-REFINEMENT-OMEGA-ARM` (both deliverables in). **The Steward
  released the first of those off its stale status and had to withdraw it**
  (`evt_19mgss08dkyy4` → `evt_6m6mzkdqgzxc8`); the language-leader's pickup
  preflight caught it before an implementer was kicked.
  > **THE LESSON, and it indicts the refresh two entries below.** That refresh
  > re-measured the rows I was actively working and **carried the rest forward
  > unverified** — which is worse than not refreshing, because it launders stale
  > rows as freshly checked. **A citation refresh that only re-measures the nodes
  > you are already thinking about is not a refresh.** The cheap complete sweep:
  > enumerate every `status: active` node from the tree, then `git log --grep`
  > each id against `origin/main` for a landing commit, and blob-verify any hit.
  > That is one command and it found both stale rows.
  > **And it must be blob identity, never ancestry** — an unlanded-looking `wp/`
  > branch is the expected appearance of landed work, because the publisher
  > squashes.
- 2026-08-27 (second refresh, at `ef91b8225`): **no roster change — citation
  re-measurement only.** Three landings in one stretch moved all three lanes:
  lane 1's HS11 recut `fec63506a`, lane 2's omega-arm D2 `ef91b8225` (completing
  that node), and lane 3's D3 `9de02daff`. Both held re-releases were discharged
  (`evt_1mgb3zbskwbg3` runtime, `evt_52vwvmn0ee859` FO D2). Added two carry-
  forwards that are hazards rather than status: the omega arm's **retained
  two-index limitation** (a landed predecessor that does NOT cover the
  multi-index case), and lane 3's **fixture-path-set authorization** (which the
  frames omit and which cost D4 a hard stop). Structure untouched.
- 2026-08-27: **no roster change — citation re-measurement only.** A watchdog
  step-5 sweep against `origin/main` `61c2fefa0` found SEVEN cited nodes had
  advanced past what this table claimed, five of them to `merged`: lane 1's named
  active successor and its `-3` alias, and lane 3's entire pilot chain
  (`CAT-ORDER-PUB-EXPORT`, `CAT-GCD-REFACTOR`, `CAT-REUSE-CENSUS`). Lane 2's
  "CURRENT" still named a merged spec WP. None of the three lanes' actual current
  work appeared in the table at all. Structure untouched — three lanes,
  runtime / language / foundation, operator 2026-08-22 and 2026-08-25. **This is
  the decay the file's own header warns about, and it is worth re-running that
  sweep at any tick where a lane looks unexpectedly quiet**: a stale objective row
  is what produced the documented single-lane relapse, and it fails silently
  because every individual row still reads plausibly.
- 2026-08-25: operator REAFFIRMED the three-lane trial after a Steward single-lane
  relapse (I had collapsed to runtime-only post-compaction and left WP-2 + three
  ready CAT WPs unreleased). Lane objectives refreshed to current node reality
  (lane 1 = native carried-value M-series; lane 2 = module/import; lane 3 = CAT
  trial). Structure unchanged: three lanes, runtime / language / foundation.
- 2026-08-21 → 2026-08-22: operator moved from one lane (runtime, 2026-08-17) to
  the three-lane trial above.
- 2026-08-17: one lane (runtime, RecursiveDescent-retirement residuals); lanes 2
  and 3 idle. (Superseded.)
