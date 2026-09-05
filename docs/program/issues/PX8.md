---
id: PX8
title: "partial/positioned IO — the completion program's root; closure condition"
status: closed
owner: runtime
size: L
gate: none
depends_on: [PX8-F-CAP-41, PX8-WROTE-ABS, PX8-ERRID-SCOPE, RT-NATIVE-CARRIED-VALUE, RT-WRITEALL-ERROR-ROUTE-NATIVE, RT-WRITEALL-IO-IDENTITY-COMPLETE, PX8-NOPROGRESS-ABS]
blocks: [PX9]
github: null
origin: docs/program/09-posix-linux-abi-campaign.md (charter, PX-C phase); closure condition added 2026-07-22 (operator-approved)
---

 # CLOSED — CLOSURE-PROPERTY RE-VERIFIED = YES, 2026-09-05 (Architect verdict
> # evt_6afgrapqf4f1r, grounded on current main; Architect state checkpointed
> # 62b78b67b). The positioned/partial-IO closure property HOLDS on both engines.
> # PX8 CLOSES -> ABI-R3 (already merged) + PX9 unblock (PX9 still gated on its
> # other dep ABI-REVOKE, draft). This executes the standing operator plan
> # (PX8-F-CAP-41 close-out, 2026-08-22: "PX8 stays OPEN until the native
> # carried-value program lands") — the program has now landed and re-verified.
> #
> # Population re-derived STRUCTURALLY from the closed effect_v1.rs sums (not the
> # WP list), byte-stable vs the z3270 derivation, no reified arm added. Every
> # z3270 gap now closed on both engines, absolute + co-indexed:
> # px8f_write_partition.rs has zero #[ignore] rows and drives the real derived
> # writeAll on native (build_native_program) and interp (PosixHost) through one
> # LD_PRELOAD interposer with exact-constructor oracles — obs1 full, obs2 short
> # co-indexed, obs3 write-zero->NoProgress (not Wrote(0)), obs4 mid-stream
> # Interrupted, BrokenPipe/Unsupported as genuine mid-stream after an exact
> # Wrote(2) prefix; interp NoProgress absolute via B1. The G1/G2/G3 gaps in the
> # historical "WHY NO" section below are ALL now discharged (A1 2168f3ae3,
> # RT-WRITEALL-IO-IDENTITY-COMPLETE 4b1a0a590, B1 d03957492).
> #
> # RT-COMPMATCH-TREE-SCRUTINEE DROPPED from depends_on (Architect A3 flag): its
> # #[ignore] is a native-codegen capability gap (tree-producing match scrutinee
> # refuses at object emission — pre-existing base debt, orthogonal to IO), while
> # the clause-(b) foreign-span value it would witness is already discharged
> # absolutely on both engines (dispatch_host_op_v1 effect_v1.rs:1919/4479, interp
> # e2e :479). A capability gap masquerading as a PX8 dependency, not a residual.
> #
> # HISTORICAL — the prior RE-VERIFIED = NO banner (Architect evt_56ssrfbr4tt37,
> # 2026-09-05 @febce9a10) is superseded by the YES verdict above; the "WHY NO"
> # gap analysis below is retained as the record of what the residual program had
> # to land, all now closed.
>
> WHAT HOLDS (the SUCCESS/progress half, both engines, ABSOLUTE — real value the
> FOLD delivered): ReadSome{span,count}, ReadEof (§1.7.1 effective 0), Wrote incl.
> capped-short, InvalidBounds/InvalidOffset/BufferLimit/AllocationFailed; co-index
> clause (b) structural (TransferCountV1 carries effective_request; span.length ==
> count; closed sums map 1:1 to LOCKED §1.7.2).
>
> WHY NO (the ERROR/termination half + writeAll route, "both engines absolute"
> fails). The residual native carried-value program is only PARTIALLY landed.
> Grounded gaps and their now-filed owners:
> - G1 writeAll §1.7.3 obs 1/3/4 (all-full / write-zero->NoProgress / mid-stream
>   Io error after exact prefix) unwitnessed on native (px8f_write_partition.rs:355
>   IGNORED, ken-verify). OWNER = [[RT-WRITEALL-ERROR-ROUTE-NATIVE]] (A1, filed
>   2026-09-05, active, GATE-0-first, option-(b) fork-ready). The critical
>   sub-item; the error/termination continuation is DISTINCT from the success
>   carry the FOLD closed (the old label named merged RT-CLOSURE-BOUNDARY-RESIDUAL).
> - G2 host-Io error identities (Unsupported/BrokenPipe/Interrupted) on native.
>   RESOLVED into the writeAll route; NO B2. The separability measurement
>   returned NO (runtime-implementer evt_1aqmj64zhk9b1, evidence 85d35017): a
>   single positioned writeAt with an injected BrokenPipe/Unsupported refuses at
>   StaticResponseDeferred BEFORE host dispatch, so there is no synchronous
>   single-op Io identity off the writeAll boundary and no independent B2 node.
>   Mid-stream Interrupted is witnessed by A1. The two remaining in-path
>   identities (Unsupported/ENOSYS + BrokenPipe/EPIPE), forced as GENUINE
>   mid-stream replies after an exact prior Wrote prefix, are owned by
>   [[RT-WRITEALL-IO-IDENTITY-COMPLETE]] (filed 2026-09-05, active, S; a small
>   serialized follow-up, same file, behind A1; Architect-reviewed candidate).
> - G3 NoProgress absolute oracle missing on the INTERPRETER. OWNER =
>   [[PX8-NOPROGRESS-ABS]] (B1, filed 2026-09-05, ready, independent, S).
> - G4 clause-(b) foreign-span-freeze rejection unwitnessed on native. OWNER =
>   [[RT-COMPMATCH-TREE-SCRUTINEE]] (A3, existing draft). Weakest gap (shared
>   dispatcher; interp + malformed-span controls cover the mechanism).
> - G5 (secondary tail, GATE-0 per node, priority after A1/A3, NOT yet wired as
>   PX8 deps pending per-node confirmation each is a distinct property VALUE vs
>   coverage): [[RT-PROCESS-EXIT-STATUS]] (cross-buffer freeze->InvalidBounds),
>   [[RT-CARRIED-RESOURCE-SCALAR]] + [[RT-SITEOP-CARRIED-WITNESS]] D2 residual
>   (px7f escape/right-denial/second-release; buffer_allocate->InvalidBounds is
>   COVERAGE-ONLY, InvalidBounds already native-witnessed = not a PX8 gate),
>   [[RT-BORROWED-INPUT-CARRIER-DURABILITY]] (px8l recursive-decl execution trap).
>
> EXIT GATE (Architect-owned, binds above every node): PX8 closes when the
> closure PROPERTY re-verifies — every positioned/partial-IO reified value (a)
> absolute vs LOCKED §38, (b) co-indexed, on BOTH engines — NOT when these nodes
> go green. The Architect re-derives the population structurally from the closed
> effect_v1.rs sums AGAIN at the end, so a reified arm added meanwhile is caught.
> Do not let node-green substitute for that re-verification.
>
> EXIT-GATE POPULATION IS BOUNDED (Architect evt_41vprsanc96e8, grounded
> @f0e96061d): the property ranges over the positioned/partial-IO §1.7 reified
> values ONLY — progress (ReadSome/ReadEof/Wrote/NoProgress), window
> (InvalidBounds/InvalidOffset), buffer (BufferLimit/AllocationFailed), and the
> IN-PATH host-Io identities (Interrupted + Unsupported + BrokenPipe) — NOT the
> whole ResourceErrorV1 enum. The resource-lifecycle/authority identities
> (Closed/RightNotHeld/ReleaseFailed/MalformedResource/ResourceKindMismatch) are
> ADR-0021/ADR-0022's closure, on the authority track (RT-CARRIED-RESOURCE-SCALAR
> et al.), OUTSIDE PX8. Per-row: rt_escape:755 + rt_parity:850 InvalidBounds are
> COVERAGE-ONLY (already positioned-witnessed at rt_parity:4763/:884); px7f
> Closed/RightNotHeld and px8l closure-representation traps are off the
> reified-value axis. So the PX8 residual is A1 + RT-WRITEALL-IO-IDENTITY-COMPLETE
> + A3 (RT-COMPMATCH-TREE-SCRUTINEE) + B1 (PX8-NOPROGRESS-ABS); no B2, no G5.
>
> ## ⚠ STATUS CORRECTED `active` → `draft` — 2026-07-25 (Steward, tracker honesty)
>
> **Nothing is building this, and its `depends_on` is unmet** (`PX8-F-CAP-41`
> draft, `PX8-ERRID-SCOPE` draft; `PX8-WROTE-ABS` merged 2026-07-28). The tracker's own
> legend defines `draft` = *not framed / **deps unmet***, `ready` = *deps met,
> unassigned*, `active` = ***a team is building***. So `active` was a false claim
> that a seat held this node, and it polluted the releasable-frontier read that the
> next sequencing pass depends on.
>
> ⛔ **Blocked-ness is DERIVED, never spelled in `status`.** `gen-progress.sh`
> computes the frontier as *`ready` **and** every `depends_on` merged*, and lists
> blockers separately. ⇒ Do not invent a "blocked" status; fix `depends_on` and let
> the generator say so.
>
> ⇒ **Flip to `ready` only when every `depends_on` entry is `merged`.**
>
> ### ⛔ EDGE REPAIRED 2026-07-28 — `PX8-ERRID-SCOPE` was a ONE-SIDED blocker
>
> `PX8-ERRID-SCOPE` has carried `blocks: [PX8]` since it was split out on
> 2026-07-27, and **PX8's `depends_on` never named it back.** `gen-progress.sh`
> reads `depends_on` **only** — it never consults `blocks` — so the rule directly
> above would have cleared PX8 for release with an unstarted **L** prerequisite
> outstanding, the moment `PX8-F-CAP-41` merged.
>
> ⚠ **Latent, not live:** PX8 is `draft`, so the generator was not yet emitting a
> false frontier row. The failure mode is a *reader* — a Steward pass applying
> the flip rule to this node's own `depends_on` list, which is the authoritative
> one and was incomplete. ⇒ `PX8-ERRID-SCOPE` is now recorded on both sides.

**This issue exists because PX8 had none.** `docs/program/10-linux-abi-completion.md`
makes **15 of its 18 items** unblock on `PX8 --> ABI-R3` and `PX8 --> PX9`, and
**nothing in the repository defined when that happens.** The sub-issues existed;
the parent did not. So "is PX8 done?" was a judgement call with no artifact
behind it — which is the same *restated-instead-of-derived* defect that
**ABI-R3 exists to fix**, applied to the program's own root.

> **Why `blocks:` is empty.** `ABI-R3` and `PX9` are the two items this gates,
> but **neither has an issue file yet** (they are unframed items in
> `10-linux-abi-completion.md`, not tracked issues), so there is no id to point
> at. Minting stubs purely to satisfy a schema field would be inventing scope.
> **The gate is bound in prose below and in that document's §5 graph.** When
> `ABI-R3` and `PX9` are framed, each takes `depends_on: [PX8]` and this note
> comes out.

## ⛔ Closure condition — a PROPERTY, not a checklist

Per `10-…:239-241` (*"Frames must state the mechanism, not name one"*), the
gate is the property. The sub-issue list below is the **currently known
sufficient set**, not the definition — if these all merge and the property does
not hold, **PX8 is not done.**

> **PX8 is closed when: every value the positioned/partial IO path reifies into
> checked Ken code is (a) correct against the LOCKED text of
> `spec/30-surface/38-ffi-io.md`, asserted ABSOLUTELY rather than differentially,
> and (b) indexed to the same request/span/buffer as every other value in the
> same reply — on BOTH the interpreter and the native backend.**

Two clauses, both load-bearing, both learned the hard way:

- **(a) absolutely, not differentially.** `RT-PARITY` merged green with six
  differentials over a **shared wrong basis**. A differential is a *relative*
  oracle: it establishes that two implementations agree, never that either is
  right. Where the spec is the authority, the oracle asserts against the
  **normative text**. This is exactly how `BUDGET-EFF`'s defect survived.
- **(b) co-indexed.** `SPAN-SEAL`'s whole subject. `BUDGET-EFF` is the same
  shape arriving through **reification** rather than through a producer: the
  span is correct and the count's `remaining` is indexed to the *raw* request
  while the span is indexed to the *effective* one — two halves of one
  `ReadSome` describing different requests.

## Known sufficient set

| sub-issue | status | note |
|---|---|---|
| `SPAN-SEAL` | ✅ merged | co-indexing for carrier producers |
| `RT-PARITY` | ✅ closed | interp/native parity — **necessary, not sufficient**, see (a) |
| `BUDGET-EFF` | ✅ merged | `remaining` bounded by the effective request |
| `SEAL-2` | ✅ merged | derived carrier-producer enumeration (PR #912 @ 4ac9141e, CI green) |
| `RT-ESCAPE` | ✅ merged | native-lowering completeness (PR #911 @ 238a5c5d, CI green) |
| `RT-SPLIT` | ✅ merged | ruled NOT a PX8 semantic dependency (see below) — does not gate |

> **The known-sufficient set went fully green as of `origin/main = 4ac9141e`
> (2026-07-23) — and PX8 STILL DID NOT CLOSE.** The Architect's grounded
> closure-property verdict (`evt_163mfgjs7fkh8`) derived the live reified-value
> population from the closed Rust sums (`ReadEof`, `ReadSome(BufferSpan,
> TransferCount)`, `Wrote(TransferCount)`, `SemanticErrorV1` — `crates/ken-host/
> src/effect_v1.rs:2089-2101`) rather than from the WP list, and found the
> property **fails on both clauses**. This is exactly the outcome this file was
> created to catch: the named mechanisms all merged, and the property still had
> holes. **Three closure-insufficiency items now gate PX8** (do not close until
> all three discharge AND the property is re-verified):
>
> | gap | clause | item | shovel-readiness |
> |---|---|---|---|
> | A1 — closed-endpoint `start==capacity` host-rejects instead of deriving `ReadEof` | (a) behavior | [[PX8-F-CAP-41]] | bounded fix + RED conformance oracle; **operator queuing gate** |
> | A2 — interp capped-short `Wrote` no absolute oracle; PR-C error identities unreached | (a) evidence | [[PX8-WROTE-ABS]] | **A2a ✅ MERGED** PR #1142 (`d5a938c4`); A2b = [[PX8-ERRID-SCOPE]], blocked behind [[PX8-ERRID-ALLOC]] → [[RT-NATIVE-FNSPLIT]] |
> | (b) — `BufferSpan` has no originating-buffer identity; `freeze` accepts a same-shape foreign span | (b) provenance | [[PX8-SPAN-PROV]] | ✅ **MERGED @ `cbf6a298` (PR #914), 2026-07-23 — clause-(b) DISCHARGED**; deferred native cells → [[RT-NATIVE-FNSPLIT]] |
>
> **UPDATE 2026-07-23:** clause-(b) [[PX8-SPAN-PROV]] MERGED (@ `cbf6a298`,
> PR #914) — **DISCHARGED**. **Two clause-(a) items remain:** [[PX8-F-CAP-41]]
> (A1 — operator queuing gate RELEASED; running as Foundation Track 2 alongside
> [[RT-NATIVE-FNSPLIT]]) and [[PX8-WROTE-ABS]] (A2 — needs a normative scoping
> call on the PR-C error-row set). PX8 does NOT close until both discharge AND
> the closure property is re-verified (absolute, co-indexed, both engines).
>
> PX8 stays `active`; **ABI-R3 and PX9 stay held.** This IS the artifact-backed
> gate working — a list-green close would have shipped three real defects.

> ## ⭐⭐⭐ RE-SEQUENCED 2026-07-28 — every remaining blocker is behind `RT-NATIVE-FNSPLIT`
>
> | blocker | state |
> |---|---|
> | [[PX8-WROTE-ABS]] (A2a) | ✅ **MERGED** PR #1142, `main = 45647b51`, `eval.rs` blob `57041e57` |
> | [[PX8-F-CAP-41]] Ph2 → [[NATIVE-HANDLE-CARRIER]] | ⛔ blocked on [[RT-NATIVE-FNSPLIT]] |
> | [[PX8-ERRID-ALLOC]] → [[PX8-ERRID-SCOPE]] (A2b) | ⛔ blocked on [[RT-NATIVE-FNSPLIT]] — **new edge** |
>
> ⇒ ⭐ **`RT-NATIVE-FNSPLIT` is now the ABI program's critical path on its own.**
> `PX8` gates 15 of 19 nodes and all three of its blockers either landed or wait
> on that one node. ⛔ No cycle — `RT-NATIVE-FNSPLIT` has `depends_on: []`.
>
> **The new edge, measured:** `PX8-ERRID-ALLOC` is built and twice-approved, but
> PR #1141 died on `Cranelift backend failure: Code for function is too large`.
> Both the original and wire-corrected candidates fail identically against an
> **unchanged fixture blob** ⇒ allocation growth crossed the wall, not the
> mapping correction. The only mapping-preserving reduction (`e65c81b5`) was
> built, causally controlled, and still fails. ⛔ Further reduction would have to
> come out of the ruled wire-identity mapping — banned.
>
> ⭐ **The ceiling is NOT general:** `PX8-WROTE-ABS` passed the *same*
> `rt_parity_native` job, because it adds no native lowering alternatives.
>
> ⚠ `PX8` still does not close until A2b and `PX8-F-CAP-41` discharge **and** the
> closure property is re-verified (absolute, co-indexed, both engines).

> ## ⚖️ CLAUSE-(a) SCOPE RULED 2026-07-27 — Architect `evt_5h884g6xhtts3`
>
> **Question routed** (Steward `evt_2bh3tqwcvyb7n`, corrected by
> `evt_7aeprfb7qrwfh`): does clause (a)'s **universal** absolute-evidence claim
> bind a value population with **no production route on one engine**? Raised
> because the interpreter capped-short `Wrote` value looked unreachable.
>
> ### ⛔ ANSWER: it BINDS. No exclusion, no seam.
>
> - ⛔ **No named durable exclusion.** It would let an **admitted closed-sum
>   arm** escape the very absolute oracle this property exists to require. *"If a
>   future backend produces a short write, the same reifier runs; correctness
>   cannot begin only when that environment becomes convenient to reproduce."*
> - ⛔ **No production seam** — and none is needed.
> - ✅ **The discharge is component-boundary evidence:** a test-local
>   `HostEffectBackendV1` → the real `dispatch_host_op_v1` → the real minted
>   `TransferCountV1` → the existing reifier → the `§38`-derived literal.
>   Production unchanged. See [[PX8-WROTE-ABS]].
>
> **Three durable boundaries decided it:** this node defines closure over every
> value the path **reifies**, derived from the **closed Rust sums** and not from
> whichever end-to-end fixtures are convenient; LOCKED `§38.1.7.2` explicitly
> admits `write returns 0 < n <= effective request -> Wrote n, including a short
> write`, with `remaining = effective - n`; and the shared dispatcher accepts
> exactly that range and mints the private count itself.
>
> ### ⭐⭐⭐ The generalizable rule — it governs the other three instances too
>
> **Evidence must distinguish every normatively different behavior over the
> component's ADMITTED SEMANTIC DOMAIN.** Mutations that are *extensionally
> equal over that domain* need no discriminator. ⛔ Conversely, a genuinely
> uninhabited value — excluded by the **closed type / validator / contract
> itself** — does not bind merely because one can write suggestive prose about
> it.
>
> ⇒ ⛔ **Derive the domain from the authoritative constructor and admission
> boundary FIRST, then ask reachability. Never infer semantic absence from the
> lack of a convenient top-level producer.** ⭐ Carry this to
> [[CONF-FMT8-LEVELTOK]], [[SEC1-IFC-R3]], and [[CONF-SEC4-REFL-PAIR]] — the
> other three instances of *"an operand I cannot construct is byte-identical, to
> any reader, to one not yet built."* **Three of the four sit under universally
> quantified properties, and this ruling is the test for all of them.**

### ⚠ RT-SPLIT is bundled into PX8 by the docs, and probably should not be

`RT-SPLIT` is a **pure maintainability decomposition** of
`cranelift_backend.rs` (22,081 lines) and its own text says it *"feeds no
G-gate."* `10-…:272-275` lists it among PX8's in-flight set, but on its own
terms it carries **no semantic obligation** in the closure property above.

**Ruling (Steward, 2026-07-22): `RT-SPLIT` does NOT gate PX8.** It is real work
and stays queued for the Runtime ring, but `ABI-R3` and `PX9` **do not wait for
it**. Holding 15 items behind a file-splitting refactor would be a pure
sequencing loss. If someone believes it *is* a semantic dependency, the burden
is to name which clause of the closure property it discharges.

## Why this matters downstream

`ABI-R3` (the derived operation inventory) and `PX9` (cross-domain
`System.Error`) are the two gates. **Nothing else in the completion program
moves until they open**, and between them they transitively gate every item
except `ABI-R1` and `ABI-S3`. `PX9` alone gates six items including the whole
of Track T.

## Notes

- **Do not close this issue on the sub-issue list going green.** Re-check the
  property. The list is evidence, not the definition — that distinction is the
  entire reason this file exists.
- The completion program's coverage record is `10-linux-abi-completion.md` §9.
  Update it when this closes.

> ### ONE INHERITED CAVEAT FROM `PX8-ERRID-SCOPE`, ALREADY TRIAGED
>
> A doc comment in that WP's evidence states **"writes wire resource-error code
> 1"** as a fact. It is **prose, not a pin**, safe only because the wire
> numbering is **append-only**. **If any PX8 work makes that numbering stop
> being append-only, that sentence goes wrong silently** — no test covers it.
> Full triage on [[PX8-ERRID-SCOPE]]; do not re-file.
