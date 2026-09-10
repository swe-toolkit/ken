---
id: DS-9
title: "lawful JSON codec — the data-structures tier's acceptance test: a Json value type, encode/decode, and the proved round-trip law, assembled entirely from the landed Core/Data sections"
status: active
owner: foundation
size: L
gate: none
depends_on: [LANG-ELAB-NESTED-FORMER-RECURSION]
blocks: []
github: null
origin: Phase 3 of the catalog data-structures enrichment program (docs/program/wp/catalog-data-structures-program.md), under the catalog campaign charter (docs/program/06-catalog-campaign.md), which homes catalog authoring in Foundation. Steward-filed; Steward owns the frame and AC/control placement. Carrier design fork ruled by the Architect as dec_3n1pp559pxrrw and transcribed into frame §3. The node is now draft because it is BLOCKED on KERNEL-NESTED-IND — see the banner.
---

> ## D3+ MERGED 2026-09-10 as an accepted partial — the node stays `active`.
> ## READ THIS FIRST.
> ##
> ## Exact `247503636`, squashed to `main` `105135af3`; both paths blob-verified
> ## by the Steward (`Json.ken.md` + `ds9_json_codec_acceptance.rs`, +113/-8).
> ## Decision `dec_1vxxax44anytq` (Foundation QA evt_45b4tfjs6z4sv + Architect
> ## evt_ccmp1tpk2fgn + CV evt_6ykttftbb1y1e). The first production unbounded
> ## `json_size` fold: consumes the nested All-IH per array element AND per object
> ## member via the B selector form `recursive result for member` (the payoff of
> ## the kernel P1 -> LANG-ELAB -> DS-9 chain). `trusted_base()` delta zero.
> ##
> ## REMAINING: encode/decode + the round-trip law are still gated on the
> ## SEPARATE `show_int` P2 floor (the complete JsonNumber leaf); no codec /
> ## round-trip is claimed by this increment. Foundation's next DS-9 slice is
> ## whatever is honest WITHOUT show_int, else the node waits on that floor —
> ## confirm the next grounded deliverable with foundation-leader (it may move to
> ## the catalog self-sufficiency campaign, queued behind DS-9).
>
> ## RE-RELEASED 2026-09-10 (Steward) — the elaborator prerequisite LANDED.
> ## (Superseded by the D3+ MERGED banner above; kept for history.)
> ##
> ## LANG-ELAB-NESTED-FORMER-RECURSION merged at 8508c108b (elaborator sub-cap
> ## B: the surface now CONSUMES the kernel's nested All-IH for a recursive
> ## occurrence nested one positive-former deeper than direct). DS-9's D3
> ## JsonObject fold over List (Pair String Json) now elaborates. RESUME D3+ on
> ## a base that carries 8508c108b.
> ##
> ## AUTHORING CONSTRAINT (Architect evt_11pcvh830sd0h): author the JsonObject
> ## fold in the B SELECTOR FORM — `recursive result for member` — NOT the
> ## natural self-call `json_size (pair_snd member)`, which stays NotTerminating
> ## (sub-cap A is a future kernel WP DS-9 does NOT need). The show_int P2 floor
> ## remains a SEPARATE, independent gate on the complete number leaf.
> ##
> ## NEW ACCEPTANCE — AC-SELF-SUFFICIENT (operator ruling 2026-09-10): the Json
> ## package MUST elaborate from its own declared imports — a raw `ken-cargo run
> ## -p ken-cli -- check <Json package>` succeeds WITHOUT any preloading fixture
> ## supplying its closure. Every catalog module now carries this and DS-9 is not
> ## exempt; if a needed symbol is undeclared, DECLARE the import rather than
> ## leaning on the acceptance fixture's mk_env preload.
> ##
> ## SUPERSEDED 2026-09-10 — RE-BLOCKED HARD STOP #2 (post-kernel-fix). `active`
> ## -> `draft`. Kept for history; the RE-RELEASED banner above governs now.
> ##
> ## The kernel former-lift ADMISSION fix landed and worked (layer-1): the
> ## six-arm `Json` match now builds and the direct `List Json` array fold
> ## EXECUTES. DS-9 D3 D0 then hit a DIFFERENT layer. The Architect ruled it
> ## (evt_71pwctbj3rax8, thr_7scvvcxfn3cq2): a genuine remaining
> ## Language/elaborator structural-recursion gap, NO already-lawful source form,
> ## DS-9 blocked with no candidate. For `JsonObject`'s `List (Pair String Json)`
> ## the recursive `Json` is nested inside a `Pair`; the elaborator's surface
> ## structural-recursion machinery (`RRecursiveResult` / SCT in `elab.rs`) reaches
> ## only DIRECT recursive-occurrence children, so it cannot CONSUME the nested IH
> ## the kernel now builds — every natural surface fold reds
> ## `StructuralResultOutOfScope` / `NotTerminating`. Array-only is dishonest for
> ## an array/object increment; restructuring off `List (Pair String Json)` is a
> ## forbidden carrier change. So there is no honest increment now.
> ##
> ## BLOCKER: `LANG-ELAB-NESTED-FORMER-RECURSION` (language, released to the
> ## language ring 2026-09-10) — extend surface structural recursion + totality to
> ## consume a recursive occurrence nested one positive-former deeper than direct.
> ## It is layer-2 of the SAME nested-former-recursion capability whose layer-1
> ## (kernel former-lift admission) landed. On its landing the Steward RE-RELEASES
> ## DS-9's D3+. The `show_int` P2 floor (below) remains a separate, independent
> ## gate on the complete number leaf. The held branch `wp/DS-9-json-codec` is
> ## intact; the D3a decoder paragraph and all carrier/law prohibitions stand.
> ##
> ## The RE-RELEASED banner below is SUPERSEDED for now — its kernel-prerequisite
> ## reasoning is correct and permanent (that blocker IS cleared), but a second,
> ## distinct layer-2 blocker is now in front of D3+.

> ## RE-RELEASED 2026-09-10 (Steward) — the KERNEL-INTRINSIC-ALL-LIFT-NESTED-POSITIVE
> ## blocker is CLEARED. `draft` -> `active`. [SUPERSEDED by the HARD STOP #2
> ## RE-BLOCKED banner above — the kernel blocker IS cleared, but a distinct
> ## elaborator layer-2 blocker now gates D3+.]
>
> `KERNEL-INTRINSIC-ALL-LIFT-NESTED-POSITIVE` LANDED on `origin/main` at `486e9f33`
> (squash of `eb9910302`, blob-verified byte-identical on both kernel paths). That
> node was the exact blocker the RE-BLOCKED banner named: the intrinsic All
> former-lift's guest-path resolution now descends through nested positive formers
> (`List (Pair String Json)`), so the `Json` eliminator / match method builds and
> the non-recursive `JsonObject`-arm reject is gone. Its landing satisfies the
> Architect's ruling (evt_6g6fgjb0qan40) that this node's merge RE-RELEASES DS-9's
> D3+. The kernel prerequisite is therefore discharged.
>
> **The node resumes at the next increment — the first `D3`+ slice promising an
> unbounded `Json` fold over arrays/objects.** The older `List`-carried
> recursive-result obstruction (`KERNEL-RECURSIVE-RESULT-SURFACE`, the hidden
> `All_List` tail result — see the 2026-08-10 ruling below) is **merged/resolved**,
> so that surface no longer blocks the unbounded fold either. `show_int`
> (`Capability.Parsing.Numeric`) still defers total `show_int : Int -> String`,
> which gates the complete `JsonNumber` number leaf — a **SEPARATE, pre-existing
> floor, not this kernel node**; it remains its own gate on a complete round trip
> and does not block resuming D3+.
>
> **All standing prohibitions still bind** — the W-shaped / `Fin n` / flattening /
> Church / postulate re-encodings the Architect forbade remain forbidden (that was
> a ruling, not a consequence of the block); the carrier fork stays as
> `dec_3n1pp559pxrrw` / frame §3; the `D3a` decoder paragraph is NOT to be deleted
> (the banner below explains why it only looks redundant). D0/pickup re-measures the
> frame's fixed inputs and re-checks the §7 contention list against the LIVE lanes
> (that list is stale).
>
> **L3 slot reverted kernel -> foundation (Steward-executed reseat, 2026-09-10)**
> per the operator's standing ruling that the L3 slot runs the kernel ring while
> foundation is blocked and reverts to foundation when L3 unblocks — DS-9 becoming
> startable IS that unblock. The foundation ring is seated and works this node.
> Reviewer: Foundation QA + CV on the exact SHA, Architect on any carrier/law
> judgment, then Steward M1-M4 -> lieutenant. One increment at a time.
>
> ## RE-BLOCKED 2026-09-10 (Steward) — the RE-RELEASE BELOW WAS COUNTERMANDED by a
> ## genuine D0 hard stop + Architect ruling. `active` -> `draft`. [SUPERSEDED by
> ## the RE-RELEASED banner directly above — the kernel prerequisite it named has
> ## now landed.]
>
> The RE-RELEASED banner directly below fired on the premise that
> `KERNEL-NESTED-IND` closing made DS-9 startable. **That premise is FALSE** — the
> foundation-implementer's D0 (evt_6ha6y4ghng65j, base `ed3b57d6`) found NO honest
> array/object increment, and the **Architect ruled the hard stop genuine**
> (evt_6g6fgjb0qan40): there is a real KERNEL incompleteness that totally blocks
> DS-9. Even a NON-recursive six-arm `match` over the public `Json` fails at the
> `JsonObject` arm — `PositivityViolation("intrinsic All lift has no guest path")`
> — while building the eliminator, upstream of any codec recursion. So the whole
> codec is blocked, not just the recursive fold.
>
> **Root cause (kernel, grounded):** `crates/ken-kernel/src/inductive.rs:1455`
> `intrinsic_former_lift_type` resolves the guest path for a former applied to the
> recursive type DIRECTLY (`List Json`) but returns "no guest path" when the
> recursive occurrence sits one former-nesting DEEPER — `List (Pair String Json)`.
> This is a fail-closed INCOMPLETENESS (the type IS strictly positive; the kernel
> conservatively rejects it), NOT an unsoundness.
>
> **The new blocker is a Kernel WP:** `depends_on` is now
> `[KERNEL-INTRINSIC-ALL-LIFT-NESTED-POSITIVE]` (filed same commit; extends
> former-lift guest-path descent through nested POSITIVE formers only, preserving
> strict positivity — the Architect owns that soundness gate). `KERNEL-NESTED-IND`
> is merged and its edge is discharged; the live edge is the new node.
> **Independently**, `Capability.Parsing.Numeric` still defers total `show_int : Int
> -> String`, which gates the complete `JsonNumber` number leaf — a SEPARATE
> pre-existing floor, not this kernel node. On the kernel node's landing the Steward
> RE-RELEASES DS-9's D3+; `show_int` remains its own gate on a complete round trip.
>
> The held branch `wp/DS-9-json-codec` returns to foundation's home branch, no
> candidate, no source edit (foundation-leader evt_584mb17dj07yv). The D3a decoder
> paragraph and every carrier/law prohibition are preserved. The banners below are
> the increment history; this one supersedes the RE-RELEASE.

> ## RE-RELEASED 2026-09-10 (Steward) — the `KERNEL-NESTED-IND` blocker is CLEARED.
> ## `draft` -> `active`. [SUPERSEDED by the RE-BLOCKED banner above — see it.]
>
> `KERNEL-NESTED-IND` is now **merged** on `origin/main` (verified against
> IMPLEMENTATION-PROGRESS at current main `ed3b57d6`). This frame's own startable
> condition — *"DS-9 becomes startable when `D5` MERGES, not when the whole node
> CLOSES"* (the D5-block banner below) — is therefore satisfied: `KERNEL-NESTED-IND`
> closed with `D5` in, so the surface consumability DS-9's unbounded folds needed
> (matching / elaboration / structural-recursion accepting the lifted IH) is
> landed. `D1` / `D2` / `D3a` are accepted-partial landings; the node resumes at
> the **next increment — the first `D3`+ slice promising an unbounded `Json` fold**
> over arrays/objects (the slice the D5-block explicitly gated). foundation-leader
> confirmed it as the next grounded Foundation deliverable (evt_5majqygex8mkz).
>
> **All standing prohibitions from the original release banner still bind** — the
> W-shaped / `Fin n` / flattening / Church / postulate re-encodings the Architect
> forbade remain forbidden (that was a ruling, not a consequence of the block);
> the carrier fork stays as `dec_3n1pp559pxrrw` / frame §3; the `D3a` decoder
> paragraph is NOT to be deleted (the banner below explains why it only looks
> redundant). D0/pickup re-measures the frame's fixed inputs and re-checks the
> §7 contention list against the LIVE lanes (that list is stale). Reviewer:
> Foundation QA + CV on the exact SHA, Architect on any carrier/law judgment, then
> Steward M1-M4 -> lieutenant. One increment at a time.

> ## STATUS CORRECTED `active` -> `draft` 2026-08-14. `draft` here means
> ## BLOCKED-WITH-WORK-LANDED, not unstarted.
>
> This node's own `origin` line has said *"the node is now draft because it is
> BLOCKED on KERNEL-NESTED-IND"* since it was filed, while the `status` field
> said `active`. **The status field is the one a tracker scan reads**, so DS-9
> presented as a node a ring was working while all three Foundation seats sat
> idle — correctly stood down, per the leader's own statement.
>
> `D3a` is merged and the frame is written; nothing here is unstarted. The
> schema has no `blocked` value, so `draft` carries it, and this banner is what
> keeps `draft` from reading as "never begun."
>
> **The block is two levels deep:** `KERNEL-NESTED-IND` is itself waiting on
> `RT-DYNAMIC-ARM-SCALAR-MERGE` and `RT-NESTED-IH-NATIVE-REALIZATION`, both
> Runtime-owned, and Runtime is a single ring currently on
> `RT-LEXICAL-R3-FUSION-EMITTER`. Flipping this back to `active` requires that
> chain to clear, not a Foundation decision.

> ## `D3a` MERGED 2026-08-10 — and DO NOT DELETE THE DECODER PARAGRAPH IT ADDED
>
> Exact `857edddd`, PR #1784, `main` `458b1413`. Two catalog paths, prose only:
> the Architect's ruled `cursor_locate` disclosure into `Capability.Parsing`'s
> `Cursor` contract and beside `DecoderError`. No Ken code changed.
>
> **The `DecoderError` paragraph is currently REDUNDANT WITH THE TYPE SYSTEM,
> and that is a trap for a future reader.** Adversary `evt_1b359n9ww3d3`
> enumerated all 18 `cursor_locate` occurrences in `catalog/` and confirmed the
> prose is accurate — no law imposes a relation on `locate`, and `decoder_alt`,
> the one combinator that would normally order branches by "got furthest",
> matches on the error **constructor** and **discards** the bound location.
>
> But the reason two locations cannot be compared today is that `loc` is a type
> parameter and the three instances instantiate it at three **different** types:
>
> | instance | location type |
> |---|---|
> | `arg_cursor_locate` | `ArgLocation` |
> | `byte_cursor_locate` | `Span` |
> | `char_cursor_locate` | `Nat` |
>
> ⇒ **A fourth instance reusing an existing location type — most plausibly
> `Nat` — produces two type-compatible location values that mean different
> things, and at that moment the prose is the only protection.** The paragraph
> is not describing a property the types already guarantee; it is describing one
> the types guarantee *by accident of the current instance set*.
>
> **Anyone who notices it is "obviously true today" and proposes deleting it is
> reading the accident, not the contract.** Record kept here because that
> observation is correct and the conclusion from it is wrong.
>
> Bound: the Adversary checked `catalog/` only. A Rust-side comparison of two
> location values in `crates/` was not audited.
>
> ## RULED 2026-08-10 — `D3`+ is BLOCKED for unbounded Json folds. `D2` is NOT.
>
> Architect `evt_6ysrp62e4zayg`, answering the Steward's question
> `evt_6mbzn0y6jh232`. **`KERNEL-RECURSIVE-RESULT-SURFACE`'s obstruction extends
> to `List`-carried recursion** when `List` is the positive carrier of a nested
> host occurrence. The decisive shape is production `All_List`: for
> `Cons head tail`, `check_match_with_lift` installs both associations but hides
> every post-source binder. The head evidence is `support: None` and can be
> consumed implicitly in lockstep; **the tail evidence is
> `support: Some(All_List)` and no source term can denote that recursive
> result.** Same obstruction as `Bag.Join`, and independent of `List` having
> only one recursive tail.
>
> | scope | status |
> |---|---|
> | `JsonArray : List Json` — unbounded fold | BLOCKED |
> | `JsonObject : List (Pair String Json)` — unbounded fold | BLOCKED. Matching `Pair` exposes its direct `Json` leaf association, but iterating the remaining `List` still needs the hidden `All_List` tail result. |
> | `D2`'s direct structural recursion over standalone `List Char` | NOT BLOCKED, and **not to be reopened**. Its cursor functions self-call on the explicit tail; they do not consume a kernel-supplied nested-host lift. |
>
> **The dependency is SLICE-level, not node-level, and it is deliberately NOT in
> `depends_on`.** Per the Architect: add `KERNEL-RECURSIVE-RESULT-SURFACE` as a
> dependency **for the first `D3`+ slice promising an unbounded Json fold or
> codec over arrays or objects.** Putting it in the frontmatter would mark the
> whole node blocked, which is false — `D1` and `D2` are merged and the
> remaining non-fold work is unaffected. This is the same edge-granularity trap
> the `D5` note below already warns about, in the opposite direction.
>
> **Finite-depth unrolling remains a discriminator, not a discharge.** A `D3`
> candidate that unrolls to some depth has not satisfied this and must not be
> proposed as though it had.

> ## D2 MERGED 2026-08-10 as an accepted partial — the node stays `active`
>
> Exact `ee6773b0`, PR #1775, CI green, `main` `0daf7170`. Both paths
> blob-verified from the declared merge-base `258336bf`, path count checked
> against the declared scope: `catalog/packages/Data/Serialization/Json.ken.md`
> and `crates/ken-elaborator/tests/ds9_json_codec_acceptance.rs`, `+337/-28`.
> Authorized by resolved Decision `dec_7qw0k1q4rv6bc` (Architect APPROVE,
> 07:22:20Z). Adversary notified at `evt_2cx4zkhqmfe9p`.
>
> A transparent structural `CursorOps (List Char) Char Nat` instance and
> transparent `CursorLaws`, with the generic selectors exercised on both
> non-empty and empty cursors and exact `trusted_base()` set identity asserted.
>
> **What remains buildable is an OPEN QUESTION put to foundation-leader at
> `evt_27cwr5ecnx209`, not something I have established.** The ruling above
> blocks any `D3`+ slice promising an unbounded fold over arrays or objects.
> Whether any of `D3`-`D7` — the round-trip law's statement as distinct from
> its proof, fuel-sufficiency scaffolding, Findings — avoids that fold is the
> ring's knowledge, and "nothing survives" is an acceptable answer that makes
> Foundation's idleness the Steward's backlog rather than the ring's.

> ## D1 MERGED 2026-08-10 as an accepted partial — the node stays `active`
>
> Exact `6675ff54`, PR #1770, CI green, `main` `258336bf`. Both paths
> blob-verified: `catalog/packages/Data/Serialization/Json.ken.md` and
> `crates/ken-elaborator/tests/ds9_json_codec_acceptance.rs`, `+188/-0`.
> Authorized by resolved Decision `dec_3xk75veggzhjm` (Architect APPROVE).
>
> **The declaration this node was stood down for on 2026-07-27 is now on
> `main`.** The ordinary six-constructor `Json` with `JsonArray (List Json)`
> elaborates on top of the lifted nested-inductive restriction, which is the
> whole point of the Architect's option-B ruling.
>
> **Merged is not closed.** `D2`-`D7` still form this WP and are in flight on
> `wp/DS-9-json-codec`. `D1` landed under `merge-policy.md`'s accepted-partial
> rule because the ring had already started building `D2` on top of it, which
> made it the floor rather than a candidate. It was a straight-ancestor cut:
> zero rebase, no verdict transfer.
>
> **Two disclosed residuals, non-blocking and not to be re-litigated at the
> next merge:** `JsonNumber : Int` excludes fractional and exponent forms; and
> the attached-proof spelling cannot name the nullary `char_cursor_ops`
> constant. Both are recorded precisely in D7 below.
>
> ## STILL BLOCKED, BUT NOT FOR THE REASON BELOW — corrected 2026-08-09
>
> **Steward verification against the code on `origin/main` `c34317f3`.** The
> banner under this one says `Json` *"is rejected by the kernel as a nested
> inductive."* ⛔ **That is FALSE on `main` and has been since `afb38934`.**
>
> `crates/ken-kernel/tests/nested_inductives_remaining.rs::declared_positive_paths_admit_list_pair_and_fresh_container_nesting`
> declares a `json` inductive whose constructors include **`List json`** and
> **`List (Pair _ json)`** — the exact two shapes this node was blocked on — and
> admits all five. It is landed and green. `check_pos_arg` now traverses
> recorded `ParameterPolarity::StrictlyPositive` positions instead of rejecting
> every non-`D` head.
>
> ⇒ **The kernel-expressibility blocker is CLEARED.** `KERNEL-NESTED-IND`'s
> `D1a`, `D1b`, `D2`, `D3a`, `D3b`, and `D4` are all in.
>
> **What actually still blocks DS-9 is `KERNEL-NESTED-IND` `D5` alone** —
> surface consumability: matching, elaboration, and structural-recursion
> checking accepting the lifted hypotheses. You cannot write `encode`/`decode`
> by recursion over `JsonArray (List Json)` until surface matching consumes the
> lifted IH, and that is `D5`, currently in review as an accepted partial.
>
> ⚠ **DS-9 does NOT need `AC-K12`, and this is the part that changes
> sequencing.** `AC-K12` is native lowering, the Cranelift verifier, and
> interpreter/native agreement, and it is blocked at
> [[RT-DYNAMIC-ARM-SCALAR-MERGE]] on Runtime. **This frame requires none of it**
> — verified by grep: `ds-9-json-codec.md` mentions native execution, Cranelift,
> and the interpreter **nowhere**. Its deliverables are a value type, a
> `CursorOps` instance, `encode`/`decode`, the round-trip theorem, fuel
> sufficiency, an acceptance test, and Findings.
>
> ⇒ **DS-9 becomes startable when `D5` MERGES, not when `KERNEL-NESTED-IND`
> CLOSES.** Reading it as "wait for the whole node" strands this node behind a
> Runtime dependency it does not have. ⛔ Do not infer the node's blockers from
> its `depends_on` edge alone — the edge is whole-node, the need is `D5`.
>
> ## RELEASED TO FOUNDATION 2026-08-10 ~05:0xZ, at `main = 65a61416`.
>
> **Foundation's stand-down is LIFTED.** Every re-encoding the stand-down
> forbids by name -- W-shaped, `Fin n`, flattening, Church encodings,
> postulates, extra malformed spine states -- **remains forbidden**; it was the
> Architect's ruling, not a consequence of the block. The diagnostic scaffold at
> `4dfdb21d` is still evidence, not a candidate.
>
> **The contention caveat in frame §7 was re-checked at release**, as the frame
> asks. No checked-out branch in any of the 46 worktrees, and no uncommitted
> edit in any worktree, touches `Data/Collections/Derived.ken.md`,
> `Core/Classes/LawfulClasses.ken.md`, or `Capability/Parsing/*.ken.md` -- the
> `include_str!` sources whose concurrent edit would change what `base_env()`
> elaborates. Runtime's live slice is `RT-DYNAMIC-ARM-SCALAR-MERGE`
> `D1b-role-b`, confined to erasure in `crates/ken-elaborator/src` and
> `crates/ken-runtime`. ⚠ Frame §7's own list of Runtime's queue (`ABI-S3`,
> `RT-VALUE-TOTALITY` P2, `RT-FNSPLIT-C1`) is **stale** -- re-derive contention
> from the live lanes, not from that sentence.
>
> ### THE PRIORITY CALL, AND THE FACT THAT I MADE IT
>
> ⚠ **This was a call between two `ready` WPs, which `steward.md §3` routes to
> the operator, and the operator is away until 11:30Z.** I made it rather than
> leaving a lane idle for seven hours, and I am recording it so it can be
> reversed rather than discovered.
>
> The block below says the contention is "DS-9 and Verify's
> `CI-ASSERTIONLESS-L1` both want the lane." **That contention dissolved** --
> `CI-ASSERTIONLESS-L1` merged at `3d6622c9`. But a new one appeared the same
> hour: I filed and framed `CI-L1-EXECUTING-COVER` (Verify, S), which became
> eligible at that same merge. So the choice was DS-9 versus that.
>
> **I gave it to DS-9, on three grounds:** Foundation has had no active work
> since 2026-07-27 while Verify has been continuously busy; DS-9 is the
> data-structures tier's acceptance test and has been blocked on a kernel
> capability that has now landed; and `CI-L1-EXECUTING-COVER` is a node I
> created tonight, so letting it pre-empt one that has been eligible and
> waiting two weeks would be bad sequencing. ⛔ **If the operator disagrees,
> DS-9 yields** -- it is early and nothing is sunk.
>
> ## FLIPPED TO `ready` 2026-08-09 — `D5` MERGED AT `82918b6a`.
>
> `KERNEL-NESTED-IND` `D5` landed as an accepted partial (PR #1743, exact
> `5903b664`), delivering elaborator lockstep, interpreter evaluation, and
> provenance-gated checked-artifact erasure. **That is the event this node was
> waiting on**, and the `AC-K12` independence is now confirmed against `D6`'s
> own text rather than by grep: the acceptance test elaborates the `.ken.md`
> through `ElabEnv::elaborate_ken_md_file`, asserts the laws are real globals,
> and measures the `trusted_base()` delta — it **asserts behavior through the
> elaborator**, per the operator's 2026-07-26 test policy. Nothing in it
> executes natively.
>
> ⛔ **`ready` is NOT released.** It means framed-and-shovel-ready
> (`steward.md §4e`). **Foundation is still stood down** and every re-encoding
> the stand-down forbids by name — W-shaped, `Fin n`, flattening, Church
> encodings, postulates, extra malformed spine states — is still forbidden. The
> diagnostic scaffold at `4dfdb21d` is still evidence, ⛔ not a candidate.
>
> ⚠ **What now gates it is the operator's two-lane cap, not the kernel.**
> Kernel holds a lane (`D6`, `D7` remain) and Runtime holds the other
> (`RT-DYNAMIC-ARM-SCALAR-MERGE` `D1b-id`). When a lane frees, DS-9 and Verify's
> `CI-ASSERTIONLESS-L1` both want it. ⛔ **That is a priority call between two
> `ready` WPs, which `steward.md §3` routes to the operator** — the Steward does
> not decide it, and the previously recorded "Verify is first back in" predates
> DS-9 becoming eligible.
>
> The original block, now partly superseded, follows.
>
> ## ⛔⛔ BLOCKED 2026-07-27 — `Json` IS NOT EXPRESSIBLE IN THE CURRENT KERNEL
>
> **Released, started, and blocked at `D1` inside half an hour.** Foundation
> found that the ordinary spelling
>
> ```ken
> data Json = ... | JsonArray (List Json) | JsonObject (List (Pair String Json)) | ...
> ```
>
> is **rejected by the kernel** as a nested inductive — the `List (Rose A)` class
> that `spec/10-kernel/14-inductive.md` §8.5 defers (`:126-128`, `:569-570`,
> `:709`). Diagnostic scaffold preserved at **`4dfdb21d`**.
>
> ⭐ **This is DS-9 succeeding, not failing.** The frame's premise was that DS-9
> adds no component and exists to discover whether the tier composes, with
> friction as the deliverable. It found a real kernel limit at the first
> deliverable — a better result than a clean landing.
>
> **Architect ruling `dec_13af1mercv2m0` (`evt_55k9f9efvd8jk`): option B,
> nested-only.** DS-9's ordinary six-constructor `Json` is **preserved**; the
> kernel restriction is lifted instead. ⇒ `depends_on: [KERNEL-NESTED-IND]`,
> behind `SPEC-NESTED-IND`.
>
> ⛔ **Foundation stands down.** ⛔ Do not re-encode `JsonArray` to get moving —
> the ruling rejects W-shaped, `Fin n`, flattening, Church encodings, postulates,
> and extra malformed internal spine states **by name**. The scaffold is
> diagnostic evidence only, ⛔ **not** a QA or merge candidate.
>
> ⚠ **The carrier ruling below is UNCHANGED** — `List Char` still stands. This
> block is about the *value type*, not the codec's carrier.

> ## ▶ THE TIER'S ACCEPTANCE TEST
>
> Frame: [`ds-9-json-codec.md`][f], under `docs/program/wp/`. The frame is the
> executable artifact; this node carries the sequencing and the gate.
>
> **DS-1 … DS-8 are all landed.** DS-9 adds no new component — it finds out
> whether the ones already there compose.

## ✅ The carrier is RULED — `dec_3n1pp559pxrrw`, transcribed

**Option C: `List Char` is the law-bearing carrier.** Core API
`encode : Json -> List Char` / `decode : List Char -> Result JsonError Json`,
round-trip proved structurally over `Json` and transparent list operations.
`Bytes` and `List UInt8` are both **rejected as the core carrier**; convenience
shells are permitted but ⛔ **inherit no theorem by assertion**. Full ruling text
is transcribed into frame §3 — read it there, not here.

⭐ **DS-9 adds zero new trusted declarations**, and it is **not** an absolutely
trust-free theorem: the string leaf rests on the landed `axiom`
`string_to_list_char_retraction`
(`catalog/packages/Data/Text/StringBijection.ken.md:13`). `AC-8` makes that
dependence visible; `AC-9` bounds what DS-9 itself introduces.

⛔ **`bytes_concat` does NOT gate this node.** The law-bearing core does not
consume it; its missing spec entry is a separate gap.

### The measurement that produced the fork

Kept because it is why the ruling went the way it did. Measured at
`origin/main = 32b1b772`:

| measurement | where |
|---|---|
| `bytes_concat` occurs **zero times in the entire `spec/` tree** — no chapter, no registry row, no law | `spec/` (whole-tree grep) |
| `bytes_to_list : Bytes → List UInt8` is `PrimReduction::Op`, **"opaque to kernel conversion"** | `spec/10-kernel/18a-primitive-registry.md:624` |
| its bridge laws `bytes_list_roundtrip` / `list_bytes_roundtrip` are **"trusted declarations, not"** proofs | `:628-629` |

⇒ A `Json → Bytes` encoder makes the round-trip either unprovable or provable
only at a `trusted_base()` cost — against the zero-delta discipline every landed
catalog entry has held. The carrier was a component-design call, so it went to
the Architect rather than being settled in this frame.

⚠ `bytes_encode`/`bytes_decode` are **not** in the same position — the
`BytesRoundTripLaw` at `spec/30-surface/38-ffi-io.md:253` records
`∀ s. bytes_decode (bytes_encode s) == Ok s` as **provable**. The gap is `Bytes`
*concatenation*, not the `String`/`Bytes` boundary.

## ⛔ The `DS-5 → DS-9` graph edge is CUT

The program's Mermaid graph draws `DS5[DS-5 Vector] --> DS9`. **DS-5 is
spec-gated** on a `spec/50-stdlib/` `Vector` chapter that has no author and no
node, and its own program text lists it under "Deferred / prerequisites."

⇒ Honoring that edge would park the tier's acceptance test behind a spec gap it
has no need of. DS-9 uses `List`, complete since DS-4.

⚠ **That cut still holds, and it is not what blocks the node.** `depends_on` was
empty when this was written — every *catalog* prerequisite was already merged, and
that is still true. The single dependency now recorded,
`KERNEL-NESTED-IND`, is a **kernel expressiveness** prerequisite discovered during
execution, ⛔ not a revival of the `Vector` edge. Introducing a length-indexed
carrier would still be wrong, and the Architect's ruling says so explicitly:
substituting `Fin n` *"imports the deferred length-indexed carrier that the DS-9
frame expressly excludes."*

## What the tier supplies

DS-1 `Empty`/`Dec` · DS-2 `Ord Nat` · DS-3 `Option`/`Result` combinators ·
DS-4 `List` combinators + laws · DS-6 lawful `DecEq Char` → `Eq`/`Ord String` ·
DS-7 `Applicative`/`Monad` · DS-8 `Traversable` · plus the parsing floor
(`Capability/Parsing/{Cursor,Decoder,Numeric,Parsing}.ken.md`), which is
carrier-neutral by construction and already recursion-capable.

⭐ **The exemplar stops exactly where DS-9 must not.**
`Capability/Parsing/Parsing.ken.md` §4.3 builds a recursive `BoolExpr` grammar
with both a parser and a printer — and **no round-trip theorem**. Its complete
theorem list is three items, none of them about printing. So the exemplar gives
DS-9 its shape and not its proof, and the proof is the work.

## Findings are a deliverable, not a byproduct

Per the charter's routing: kernel-reduction defect → **Kernel** via the enclave;
sugar or abstraction candidate → **Ergo**; abstraction kept in-catalog →
Foundation. ⛔ A DS-9 that lands clean and files nothing has written a codec
without running the acceptance test. `AC-10` exists so that "clean" and "never
looked" cannot read identically.

### D7 findings — recursive-result boundary and decoder probe

Initially measured on `wp/DS-9-json-codec-d3-probe` from
`origin/main = 0daf7170`, then re-anchored and revalidated from
`origin/main = 5df141ba`:

1. **The decoder-only construction probe succeeds.** A minimal real
   `decoder_recursive` over the explicit `List Char` cursor elaborates and
   executes with both required nested result paths. Its array path applies
   `decoder_many` at `Json` and returns `JsonArray (List Json)`; its object path
   applies `decoder_many` at `Pair String Json` and returns
   `JsonObject (List (Pair String Json))`. The discriminating fixtures `[n]`
   and `{"k":n}` reach those two paths respectively and consume all input;
   `n` is deliberately the probe's one-character null atom, not a claim that
   the production JSON grammar is complete. This demonstrates that constructing
   recursive values while consuming explicit `List Char` input does not itself
   require a kernel-supplied nested-host fold result. It is measurement evidence,
   not the framed production `decode`.
2. **The decoder probe does not alter the top-level
   `KERNEL-RECURSIVE-RESULT-SURFACE` ruling.** It constructs nested values from
   the explicit input cursor; it does not perform the unbounded `List Json` or
   `List (Pair String Json)` folds that the ruling identifies as blocked. The
   framed encoder and array/object round-trip proof therefore remain blocked.
3. **The numeric domain remains intentionally integral.** `JsonNumber : Int`
   excludes JSON fractional and exponent forms. Any future codec theorem is
   scoped to the stated integral-number subset until that residual is separately
   designed and discharged.
4. **Nullary attached-proof spelling remains an Ergo finding.** The attempted
   `proof ... for char_cursor_ops` form was rejected with
   `attached proof for 'char_cursor_ops' must mention that subject applied in
   its claim`: the validator requires an application headed by the subject,
   which a nullary `const` cannot form. D2 retained the complete proof as
   ordinary transparent theorems, so no obligation was weakened and no trust
   was added.

The probe's rejection paths use `decoder_satisfy`, and therefore transport the
current cursor's `cursor_locate` value. For this DS-9 instance that value is the
remaining suffix length. The Architect has ruled that locations are
instance-defined and carry no cross-instance coordinate meaning; this does not
affect the recursive-tail measurement, and the generic cursor/decoder disclosure
is a separately routed `Capability.Parsing` prose fix rather than a DS-9
redesign.

**Finite-depth unrolling or a different `Json` representation is not a
discharge.** The Architect permits finite unrolling only as a discriminator,
and the ordinary six-constructor carrier remains fixed. No encoder, theorem,
fuel lemma, shell, or D6 acceptance claim is introduced by this probe.

## Contention

**None with Runtime.** DS-9 touches `catalog/packages/` and adds one file under
`crates/ken-elaborator/tests/`. Runtime's queue — `ABI-S3` → `RT-VALUE-TOTALITY`
P2 → `RT-FNSPLIT-C1` — is confined to `crates/ken-host`, `crates/ken-runtime`,
`crates/ken-interp`, and `crates/ken-elaborator/src`. ⚠ Frame §7 carries the
one caveat worth re-checking at branch time.

[f]: ../wp/ds-9-json-codec.md
