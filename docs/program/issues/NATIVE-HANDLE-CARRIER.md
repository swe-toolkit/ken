---
id: NATIVE-HANDLE-CARRIER
title: "Native build-pipeline completeness — a constructor-private resource-carrying handle fails checked-core body-view lowering (MissingClosureMetadata) when it crosses the higher-order withBuffer normalization boundary"
status: active
owner: runtime
size: M
gate: none
depends_on: [RT-NATIVE-FNSPLIT, RT-JOIN-DISPOSITION, RT-DECL-CLOSURE-PORT, RT-BACKEND-PRIMITIVE-LOWERING-SPLIT, RT-SITEOP-CARRIED-WITNESS]
blocks: [PX8-F-CAP-41]
github: null
origin: discovered under [[PX8-F-CAP-41]] Phase 2 impl (foundation-implementer hard-stop evt_563ss8821n7f); Architect means/representation ruling evt_2zkjr68y1sdgf (thr_570t9qzcthjv9, 2026-07-23). Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # HARD-STOPPED 2026-08-17. THE ARCHITECT RULED: THE GAP IS NOT THIS NODE'S.
>
> **RULED at `evt_559gymspqap8w`** (answering `evt_1vdg5skdf1ndh`). **The hard
> stop was correct and so was refusing to add `int_to_uint64_raw`.**
>
> **The fix does not belong here.** The component that must change is
> **synthesized error-value construction and site-operand provenance**, not the
> handle carrier — so **this node narrows to what its name says** and the gap
> goes to a successor.
>
> ### THE SUCCESSOR ALREADY EXISTED: [[RT-SITEOP-CARRIED-WITNESS]], now `ready`.
>
> **It was filed 2026-08-07 against this same conflict** and sat `draft` on one
> bar: an open Architect fork on the mechanism, which is the fork just ruled. It
> is a **fold, not a new node** — checked before writing one, per §4e. Its
> `depends_on` ([[RT-CARRIER-BYTESPAN-OBSERVE]]) is `merged`, and it now carries
> the ruling verbatim in `§3b` of its frame.
>
> **`depends_on` gained it on 2026-08-17.** The hard stop is the evidence: this
> node cannot reach `AC-5` until that one lands.
>
> ### THE RULED MECHANISM, AND THE TRAP IT NAMES ONE LAYER IN
>
> **`CarriedWord` is the correct representation, §38 stays closed, AND the
> seat-observation side is already correct** — the gap was on neither side of
> the fork as I posed it. The four `Fs*` `Argument(0)` seats are
> `SPECIALIZED_ONLY` **deliberately and not because the observer fails them**
> (`planning/static_transition.rs:5403-5408`); each is the `SiteOperand(0)` of
> its own synthesized `FileError`, so **the binding constraint is a second,
> different consumer of the same operand.**
>
> ⇒ **Widening the seat's `Avail` would be the same trap the ring just stepped
> around, one layer in**: a green, well-tested change that relocates the refusal
> into `FileError` synthesis rather than removing it.
>
> **The available direction is §2g's ruled emitted-helper route** — project the
> carried word to runtime `(pointer, len)` through an emitted helper. That is
> sanctioned by construction, not the banned `Carried -> Lowered` inverse.
>
> **`status: active` and NOBODY IS WORKING IT.** There is no `blocked` status;
> `ready` would put it back on the dispatch frontier when it is waiting on a
> successor, so `active` plus this banner is the accurate pair. **Do not
> dispatch this node until [[RT-SITEOP-CARRIED-WITNESS]] lands.**
>
> **No candidate exists and none was sent to QA.** The ring ran the diagnostic
> first deliverable at base `bad9a9bb0` and reset the branch (`evt_4eynen6drs79x`).
>
> **What the diagnostic ESTABLISHED, and it is real progress:** `c07e63c2`
> rebases cleanly (the `D1` tree equals `git merge-tree --write-tree bad9a9bb
> c07e63c2`, range-diff preserving both the main-side RAII/compiler-driver
> context and the carrier/BigInt changes), and **the saved CAP-41 fixture now
> passes checked-core body view, the computational-IH census, and erasure.** ⇒
> **axes (a)-(c) are no longer where the failure lives.**
>
> **The first native refusal is elsewhere, and it is a class this frame
> anticipated:**
>
> ```
> Effect: seat Argument(0) of FsReadFile needs BytesPointerLength,
> which it cannot observe in CarriedWord
> ```
>
> **Verified in the tree, not taken on report** —
> `px4b_native_production.rs:429/510/614/719`,
> `px7f_resource_native.rs:296/327`, `px7l_checked_host_recursive_bind.rs:137`,
> and decisively **`rt_parity_native.rs:477/506/533/578`, the file holding the
> control row this node must return to GREEN.** So the pre-existing two-bracket
> row reaches the identical wall.
>
> ⇒ **`int_to_uint64_raw` CANNOT restore `AC-5`, and adding it would be an
> unauthorized partial** — a green, well-tested change that fixes nothing.
>
> **THE ARCHITECT'S PRIMITIVE ENUMERATION HELD. This is the residual it
> explicitly reserved a stop for:** *"the stop condition is retained only for a
> non-primitive constructor/effect gap, not another primitive."* Every primitive
> on the route was already handled; the gap is an effect seat.
>
> **The `S` sizing on the native slice was derived from that primitive
> enumeration, which is no longer the binding constraint. Do not plan against
> it.**
>
> > **Why the diagnostic-first clause was worth keeping — recorded because it
> > paid off within minutes of the kickoff.** On release I discharged the
> > axis-(d) **contention** route-back (its concurrent RT track,
> > [[RT-NATIVE-FNSPLIT]], is `merged`) but kept the Architect's
> > **diagnostic-first** clause in the same bullet, on the ground that the two
> > had independent bases and only one had expired. **The whole release chain
> > pointed at the `int_to_uint64_raw` arm and its fresh durable home. Going
> > there first would have produced a passing candidate that does not fix the
> > defect, with nothing to red.**

> ## `ready` AS OF 2026-08-17. EVERY DEPENDENCY IS MERGED — there is no bar left.
>
> **All four `depends_on` entries are `merged`.** `RT-NATIVE-FNSPLIT`,
> `RT-JOIN-DISPOSITION` and `RT-DECL-CLOSURE-PORT` were already; the fourth,
> [[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]], landed at squash `7b05136bd` (PR
> #2545) as the **complete** `D0`-`D4` move rather than a partial — which is what
> its `AC-6` was written to require before a flip could release this node.
>
> **The frame is written. Do not re-frame it, and do not read the absence of
> `D0`-style headings as absence of a frame** — this node predates that
> convention and carries a diagnostic first deliverable, the six-axis Architect
> acceptance matrix at `evt_2zkjr68y1sdgf`, its controls, and a fence, under
> prose headings.
>
> ### TWO COORDINATES IN THIS FRAME WENT STALE WHEN THE SPLIT LANDED. Corrected here.
>
> **(i) The durable home now has a path.** Below, this node's remaining
> `ken-runtime` work is described as the `int_to_uint64_raw` arm "in the durable
> home" — an abstraction that was correct only while the move was pending. It is
> now `crates/ken-runtime/src/cranelift_backend/lowering/core/primitive.rs`, with
> `lower_primitive_call` at **`:43`** as `pub(super) fn`.
> **[[RT-BACKEND-MODULE-SPLIT]] still cites `core.rs:17977` refusing at `:18208`
> for this same arm; those coordinates now resolve to nothing.** Do not search
> `core.rs` for the dispatcher.
>
> **(ii) The axis-(d) fence names a contention that no longer exists.** It says
> that if the fix requires `ken-runtime` constructor/value lowering, that "would
> contend the concurrent RT track — STOP and route to the Steward." **The
> concurrent RT track was `RT-NATIVE-FNSPLIT`, which is `merged`**, and the ring
> that would do this work is the same runtime ring — it cannot contend with
> itself. Read literally today, the fence sends the ring to ask permission for
> work its own sequencing already assigned it.
>
> ⇒ **The contention route-back is DISCHARGED. The other half of that bullet is a
> DIFFERENT CLAUSE and still binds.** *"Do not pre-assign the fix to
> `ken-runtime`/Cranelift on current evidence"* is the Architect's
> **diagnostic-first** instruction: de-erase the `CheckedCoreBodyViewError` lane
> and isolation-flip fixture `f0eb65ce` **before** choosing a fix site. **Only
> the contention basis expired; retiring both because they share a bullet is the
> error.**
>
> ### The superseded reason, retained because the correction is the point
>
> **It read: *"its `depends_on` names `RT-BACKEND-MODULE-SPLIT`, which is still
> `draft`."*** That edge was re-pointed on 2026-08-10 to the one #8 child that
> re-homes the code this node edits — see the NARROWED banner below, which
> records it correctly and in full.
>
> ⇒ **The status stayed right while its stated cause went stale**, and the
> stale cause sat in the *leading* banner while the correction sat sixty lines
> down. **A reader arriving top-down got the true status from a false
> premise**, and a grep for the current dependency would not have surfaced the
> line that contradicts it. **Fix the reason where the reader meets it, not
> only where the change happened.**
>
> **The original rule still holds:** `ready` means shovel-ready — a written
> frame **and** dependencies merged. A node advertising startable work it does
> not have makes the backlog read deeper than it is, and that depth is what a
> Steward reads to decide whether a team is idle for want of work or for want
> of a lane.
>
> **The correction is not a downgrade of the work.** A node advertising
> startable work it does not have makes the backlog read deeper than it
> is, and that depth is exactly what a Steward reads to decide whether a
> team is idle for want of work or for want of a lane.

> # SEQUENCED 2026-08-08 — RESUMES AFTER [[RT-BACKEND-MODULE-SPLIT]], campaign node #8
>
> **Operator instruction, 2026-08-08: slot this node after
> `RT-BACKEND-MODULE-SPLIT`.** Recorded as a `depends_on` edge, which is the
> only thing `gen-progress.sh` reads — a prose note would not have held it back.
>
> **Read the hold banner below as HISTORY, not as the current blocker.** Its
> gate — `RT-DECL-CLOSURE-PORT`'s `AC-1` row — **merged**, and all three of the
> original dependencies are now merged. This node was genuinely resumable
> between that merge and this instruction. It is now held by a **new and
> deliberate** edge, not by the old one.
>
> **Consequence, stated because it is a real cost and the decision is the
> operator's:** this node blocks [[PX8-F-CAP-41]] Phase 2, which is on the
> critical path to **`PX8` clause-(a) closure**. That chain now sits behind the
> entire RecursiveDescent retirement *and* the module split — five nodes, not
> zero. The elaborator half is already done and preserved at `c07e63c2`, so
> nothing rots; the cost is latency on `PX8`, not rework.
>
> **What resuming will cost, unchanged by the wait:** a rebase over
> `RT-DECL-CLOSURE-PORT`'s `core.rs` rewrite — and now over the module split's
> file moves too, which is the one item this sequencing makes *cheaper* rather
> than dearer. Doing the split first means this node rebases onto the new module
> layout once, instead of landing against the old layout and being moved by #8.

> # NARROWED 2026-08-10 — THE EDGE NOW NAMES ONE #8 CHILD, NOT THE PHASE
>
> **`depends_on` was `RT-BACKEND-MODULE-SPLIT`; it is now
> [[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]]**, cut item 2 of 18.
>
> **This honours the 2026-08-08 instruction, it does not override it.** That
> instruction's own rationale is the paragraph directly above: rebase onto the
> new module layout **once**. The primitive slice is the module move that
> re-homes exactly the code this node edits, so depending on it delivers the
> stated benefit in full — while the other sixteen slices, which touch nothing
> this node touches, stop holding it.
>
> **Architect ruling `evt_54zvaqbrm752x` §5** established the release point on a
> bounded ownership proof: this node's remaining `ken-runtime` work is the
> `int_to_uint64_raw` arm inside `lower_primitive_call`, and that dispatcher
> plus its twelve exclusively-primitive helpers move as one family. The
> Architect assigned this bookkeeping to the Operator and Steward.
>
> ⇒ **What it buys:** this node heads 19 transitive dependents — the whole
> remaining Linux ABI completion program. They now wait on **two** #8 children
> instead of all eighteen.
>
> ⛔ **The arm still lands here, not there.** The primitive slice is a
> byte-for-behaviour move and is explicitly forbidden to add
> `int_to_uint64_raw`. This node adds it once, in the durable home.

> # HELD 2026-07-29 — 11/12 GREEN, STOPPED ON [[RT-DECL-CLOSURE-PORT]]'s `AC-1` ROW
>
> **Superseded as the blocking reason — see the sequencing banner above.**
>
> **Steward disposition `evt_5mtkdft1nxmwp`.** This node was released, picked up,
> rebased, and ran its first outward validation pass. It stopped on one row —
> and that row is **not this node's to fix.**
>
> | | |
> |---|---|
> | preserved candidate | `85dcee259dc65f9e3c1d625c0ee0ed8342577492` |
> | tree | `b7cf904162bbacca83d70a0fe4bc2a86d9c36aa0` |
> | superseded WIP | `8bc7556af024886a6db01679f35a2bb063166876` (tree `9bbce2f6`) |
> | result | `rt_parity_native` **11 passed / 1 failed** |
>
> ⛔ **DO NOT reset, delete, or repoint `85dcee25`.** It carries a completed,
> uncontested `D1` rebase — `git range-diff` 3/3 `=`, no conflict, no side
> choice — plus the re-derived identity arm. ⚠ **A recorded SHA is not a copy**;
> the hazard is a hard reset from a handoff gate, not storage.
>
> ### ✅ THE COPY NOW EXISTS — pushed 2026-07-30
>
> ⚠ **The warning above was correct and the copy it called for had never been
> made.** Measured 2026-07-30: `85dcee25` lived on exactly one local ref
> (`refs/heads/wp/NATIVE-HANDLE-CARRIER`) with **zero refs at `origin`** — the
> handoff-gate hard reset it warns about would have destroyed it. Now durable:
>
> | SHA | date | durable ref at `origin` |
> |---|---|---|
> | `85dcee25` | 07-29 16:08 | ⭐ `preserved/native-handle-carrier-hs21-85dcee25` |
> | `8bc7556a` | 07-29 13:54 | `preserved/native-handle-carrier-hs21-8bc7556a` |
> | `c07e63c2` | 07-23 15:13 | `preserved/native-handle-carrier-c07e63c2` |
>
> ⛔⛔ **ALL THREE ARE ON DIVERGENT LINEAGES — none is an ancestor of another**,
> so each needs its own ref and ⛔ **preserving the newest does NOT subsume the
> rest.** Each was cut as an independent "preserve the hard-stop tree" commit off
> a different base. ⇒ Read the table as three separate artifacts, not a history.
>
> ### The one red row belongs to a different node
>
> `fs_write_at_malformed_offset_narrows_to_invalid_offset` fails **before
> observation**, at `checked_process_object`, with `Compilation error: Code for
> function is too large`. It is **candidate-caused, not inherited** — the exact
> row passes `1/1` on detached `origin/main = af056a78`.
>
> ⭐ **That row is [[RT-DECL-CLOSURE-PORT]]'s `AC-1` verbatim** — the sole AC that
> decides that node — and the Architect already measured the wall there:
> `authority=RecursiveDescent`, `residual=TransparentDeclarationClosure`, the
> oversized function being the `RecursiveDescent` **root itself**
> (`evt_3t7t27e3rv8cx`). ⇒ ⛔ **Not a new node** — filing one would duplicate a
> framed, ready node that already owns this exact row.
>
> ### What is green, and stays green
>
> ✅ four CAP-41 two-engine rows · ✅
> `uint64_checked_wrapper_admits_max_and_rejects_both_neighbors` · ✅ `AC-5`
> `fs_read_at_malformed_offset_narrows_to_invalid_offset` · ✅
> `int_to_uint64_raw_preserves_the_exact_big_native_int` 1/1 · ✅
> `px8f_buffer_io_surface` 8/8.
>
> ### ▶ What this node still owes on resume
>
> 1. A **second rebase** over `RT-DECL-CLOSURE-PORT`'s `core.rs` rewrite. `D1`'s
>    machinery is proven on this exact branch, so this is bounded.
> 2. `AC-2`'s **Big-identity-versus-i64-cast** mutation — pre-empted by the stop.
> 3. The two `AC-4` **positive-red controls**: `(c)` pre-erasure presence and
>    `(f)` public-name census presence.
> 4. The **full 6/6** `rt_span_prov_native` module plus the named CAP-41 /
>    `AC-5` / private-public controls. ⛔ **No honest partial is authorized**
>    (standing Architect ruling).
>
> ⚠ **`AC-1`'s stale-base control still binds the second rebase.** ⛔ Resolve
> hunk-by-hunk; do **not** resolve toward making a row pass — that hazard is now
> sharper, because the row that is supposed to go green is known by name.
>
> ### ⚠ The sequencing premise that put this node first was measured false
>
> It was *"NHC is 5/6 green and cheap to finish."* This node's `AC-1` is
> unreachable on any tree until the ceiling falls. ⭐ **Same class as the
> premise recorded in [[RT-DECL-CLOSURE-PORT]]'s own banner** — a Steward
> sequencing inference, unmeasured, holding a ring. The reorder costs one extra
> rebase and **shortens Foundation's wait from two nodes to one.**
>
> ⛔⛔ **`AC-4` CHANGED WHILE YOU WERE HELD** (PR #1236). Its axes `(c)`
> (erasure) and `(f)` (absent from the public name map) are the matrix's only
> **absence** claims and carried no control. Each now owes a **positive
> control** showing the check **reds when the thing IS there**. Read the frame.
>
> ## ⭐ §5a RESEARCH-CONSULT TRIGGER — THE COUNT OF RECORD IS BACK HERE
>
> Returned from [[RT-JOIN-DISPOSITION]] on its merge, because **the count
> follows the active work**. ⛔ Do not read it from that node — it is closed and
> its block now claims nothing.
>
> | | |
> |---|---|
> | **COUNT OF RECORD** | **21** |
> | ENTRIES | 12 |
> | NEXT PREDICATE CHECK | **15th entry** — the 12th is CONSUMED (`independent/mixed`, no recut) |
> | NEXT RESEARCH PULL | **#24** — #21 fired and is spent |
>
> ⭐ **Provenance of the count above.** It left this node on 2026-07-29 when
> this node was bound behind [[RT-JOIN-DISPOSITION]], and returned on that
> node's merge. Research pull **#21** fired and is spent (`evt_165w63xtakbpb` →
> advisory `evt_6nrz0cgqm1hkd`, landed durably at
> `docs/program/rt-join-disposition-research-advisory-21.md`). The 12th-entry
> predicate check was **answered** by the Architect — `independent/mixed`, ⛔ no
> recut, no count freeze or reset — and its five-subfamily partition table is on
> [[RT-JOIN-DISPOSITION]].
>
> **Hard stop #20 (2026-07-29):** Foundation's [[PX8-ERRID-ALLOC]] rebase still
> failed the native size gate at `checked_process_object`. Architect ruling
> `evt_3t7t27e3rv8cx` — the object routes to the monolithic `RecursiveDescent`
> root, so `FunctionizedUnits` never applied. Produced
> [[RT-DECL-CLOSURE-PORT]].
>
> **Hard stop #21 (2026-07-29):** this node's own fixture tripped a fail-closed
> invariant introduced by `RT-FNSPLIT-RECUR-PORT` (`6a451b45`) —
> `emitted source join StaticOriginId(1000) was later dispositioned as
> statically unselected` (`lowering/mod.rs:1712`). `rt_span_prov_native`,
> 5 passed / 1 failed on
> `sp_a_foreign_span_freeze_rejects_own_span_succeeds_on_both_engines`.
> ⭐ `main` is GREEN on that row — CI's shard filter excludes only
> `rt_parity_native`, `px8f_buffer_native`, `px8f_write_partition` — so the
> candidate is the first program shape to violate the invariant, not an
> inheritor of a red row.
>
> ⇒ **The #21 research pull FIRED and is dispatched** (`evt_165w63xtakbpb`):
> does a backend emit joins before or after static branch elimination, and is
> "emit then retract" sound or a smell? Mechanism direction is the Architect's
> (`evt_7fnxkjz9z6ghw`). ⛔ Runtime holds; no candidate, mutation, or suite run.


> ## ⛔ `draft` → `ready` 2026-07-28 — the banner promised what the status withheld
>
> This node said **"✅ FRAMED — shovel-ready"** while its frontmatter said
> `status: draft`. ⛔ **`gen-progress.sh` computes the frontier as `status:
> ready` AND every `depends_on` merged/closed** — so at `draft` this node would
> **not** have entered the frontier when `RT-NATIVE-FNSPLIT` merged, no matter
> what the banner claimed. ⇒ A Steward pass would have had to stand between the
> umbrella's merge and this kickoff, which is exactly what `§2a-bis` exists to
> remove.
>
> ⭐ **`ready` is correct despite the unmerged dependency.** `RT-SCALE-B` is the
> in-repo precedent: `ready` with an `active` dep. **Blocking is expressed by
> `depends_on`, not by `draft`** — the frontier ANDs the two. `draft` is a claim
> about *framing*, and this node's framing is done.

> ## ✅ FRAMED 2026-07-27 — shovel-ready; blocked ONLY on `RT-NATIVE-FNSPLIT`
>
> **Frame:** `docs/program/wp/NATIVE-HANDLE-CARRIER.md`, measured at
> `origin/main = 5404108a`. Owner **Runtime**, size **M**.
>
> ⭐ **This WP closes [[PX8-F-CAP-41]] Phase 2 in the same merge** — one
> deliverable, two nodes. Flip both together.
>
> ### ⛔ Premise correction the frame carries: there is ONE input ref
>
> The text below says to "fold `c07e63c2` with `f0eb65ce`". **Measured: there is
> nothing to fold** — `f0eb65ce` is `c07e63c2`'s parent. Take `c07e63c2` alone
> (`origin/preserved/native-handle-carrier-c07e63c2`); it already carries the
> handle/admission impl *and* the elaborator slice.
>
> ### ⚠ And the rebase is real work, not a preliminary
>
> `c07e63c2` is based at `8ebe370a`; **`origin/main` is 215 commits ahead**, and
> `prelude.rs`, `erasure.rs`, and `compiler_driver.rs` — all three production
> files of the elaborator slice — were **also edited on `main`** (+224 lines
> there against the branch's +188). A side-preference conflict resolution
> silently reverts landed work. That is `AC-1`.
>
> ⭐ **Status stays `draft` because `depends_on` is unmet**, not because it is
> unframed. Flip to `ready` when [[RT-NATIVE-FNSPLIT]] merges, then kick Runtime
> with a full handoff gate.

## ⚙ RE-HOMED to Runtime 2026-07-23 (elaborator slice DONE; continuation is native)

The **elaborator half is complete** — Foundation de-erased the driver error and fixed
the true root cause: `MissingClosureMetadata` was **masking**
`CheckedCoreBodyViewError::UnsupportedTermShape` / `int_lit_outside_native_i64` —
checked-core `BigInt` literals were narrowed to `i64`, and the CAP-41 fixture reaches
u64-max via the checked `intToUInt64` bound. Foundation widened checked-body literals
to `BigInt` (lossless map to `RuntimeIntV1`), preserving the underlying error through
the driver. Body-view + erasure GREEN. **Preserved on origin as
`preserved/native-handle-carrier-c07e63c2` @ `c07e63c2`** — ⚠ this used to say
`wp/NATIVE-HANDLE-CARRIER`, which has **never existed at `origin`**; the SHA was
safe but the fetch instruction was not (corrected 2026-07-30). (Parent carrier
fixture `f0eb65ce` = `preserved/px8-f-cap-41-p2-buffer-handle-f0eb65ce`; the
two-commit branch is one
`ken-elaborator` production slice + test call-site migrations; **no `ken-runtime`
touched**). Sized **S** by the implementer.

The fixture now advances through body-view/census/erasure and fails only at **object
emission**: `int_to_uint64_raw is not in the supported native set`
(`crates/ken-runtime/src/cranelift_backend/lowering/core.rs`). **The remaining work is
`ken-runtime`** — add the primitive, carry the CAP-41 fixture to full native GREEN
(lifting any *further* stacked native gaps), then **fold with `c07e63c2`** and run the
Architect's six-axis matrix + controls = the full two-engine oracle. That merge closes
**this WP and [[PX8-F-CAP-41]] Phase 2** together.

**⛔ Serialized against [[RT-NATIVE-FNSPLIT]] — FAST-FOLLOW (Steward ruling
`evt_1v37rgez26kmf`, runtime-leader read `evt_7dedryvh3fd48`).** RT-NATIVE-FNSPLIT
lands first (it owns the indivisible `lowering/core.rs` continuation-partitioning
change); combining it with CAP-41's primitive-support + two-engine oracle would make
one high-risk unreviewable `core.rs` assembly. **No concurrent `core.rs` edits.**
`depends_on: [RT-NATIVE-FNSPLIT]`; owner flipped foundation→runtime; Foundation stood
down (its elaborator slice `c07e63c2` is the preserved input). Steward kicks the
fast-follow (full handoff gate) **when RT-NATIVE-FNSPLIT merges**.

**Re-homed closure = M** (runtime-leader): take `c07e63c2`, add the `int_to_uint64_raw`
native lowering (identity-precedent arm on the signed-magnitude `RuntimeIntV1` carrier —
`lower_primitive_call` already treats `uint8_to_int`/`int_to_uint8_raw` as identity on
`Lowered::Int`), **run the exact native end-to-end until GREEN**, then the six-axis
matrix + controls + attestation/digest rider for touched native code.

**⚠ Diagnostic-staircase contingency (runtime-leader):** `int_to_uint64_raw` is NOT
asserted the final gap — the CAP fixture has revealed a new wall at each layer
(`MissingClosureMetadata` → `int_lit_outside_native_i64` → `int_to_uint64_raw`). The
acceptance is "full two-engine oracle GREEN," and any further native lowering gap the
exact fixture hits is surfaced/triaged, never worked around.

## Architect means confirmation (`evt_7xrcjp0apb4f1`) — shovel-ready for the fold

**`int_to_uint64_raw` is the sound closure of the exposed axis-(d) gap**, with a
**load-bearing constraint: it is NOT a machine `i64 -> u64` conversion.** Ken's
fixed-width carriers share the exact `Int` runtime representation; the interpreter
implements this raw narrowing as **value identity**. The native arm must:
- require exactly one `Lowered::Int` argument;
- return that same `Lowered::Int` **unchanged** — including the native-Int **tag
  sidecar** and payload/arena slot;
- preserve `18446744073709551615` as the existing **Big signed-magnitude** value;
- leave range admission to the derived checked `intToUInt64` wrapper (which proves
  `0 <= n <= u64::MAX` before calling the raw cast).

Extend the existing `uint8_to_int | int_to_uint8_raw` native arm (the representation-
level identity pattern). **Do NOT** use a Cranelift integer cast or an `i64` fast-path
that loses the Big arm — that would truncate/wrap/retag and is the failure mode.

**No further primitive gap expected on this route (Architect enumerated it):** the
checked closure's primitives are `leq_int, and_bool, int_to_uint64_raw, sub_int,
eq_int, add_int`; native already handles all but `int_to_uint64_raw`. `Some`/`None`,
handle construction/projection, and result branching are constructor/control lowering,
**not** primitives. ⇒ native code slice = **S**; the stop condition is retained only
for a **non-primitive constructor/effect** gap, not another primitive (do not
pre-inflate scope).

**Required focused discriminators (before the full oracle):**
1. `intToUInt64 u64::MAX` reaches `Some` natively, preserving the exact Big value/tag.
2. `intToUInt64 (u64::MAX + 1)` and `intToUInt64 (-1)` reach `None` — proving the
   checked **wrapper**, not the raw identity, owns admission.
3. The raw native arm and the runtime-IR/interpreter evaluator agree on representation
   identity; no wrap/truncation mutation survives.
4. Existing UInt8 conversion behavior is unchanged.

**Scope discipline:** this WP must **not** claim complete fixed-width-conversion
support if it adds only UInt64. A family generalization (the full representation-
sharing `IntN <-> Int` identity set) is **optional**, not required for CAP-41 GREEN,
and if taken must remain exact identity + be tested as a family (Small **and** Big
carriers), never an unreviewed wildcard over primitive names. The means confirm does
not waive the normal exact-SHA QA/CV/Architect gate on `c07e63c2` + the Runtime fold.

Discovered while implementing [[PX8-F-CAP-41]] Phase 2 (the sealed capacity-carrying
`BufferHandle`). The locked representation `data BufferHandle = PrivateBufferHandle
(Resource Buffer) Int` **does not lower on the native path** — the failure is raised
**before erasure and before Cranelift**, so it is a *distinct* native-completeness
gap from [[RT-NATIVE-FNSPLIT]] (which addresses the later single-Cranelift-function
`VReg::MAX` size wall). A ≤2-bracket program excludes the size wall.

## The defect (Architect-grounded, `evt_2zkjr68y1sdgf`)

On the exact preserved fixture
`preserved/px8-f-cap-41-p2-buffer-handle-f0eb65ce` (`f0eb65ce`, at origin),
native compilation fails inside the front half of the pipeline:
`compile_native_program_sources` builds the normalized checked-core package, then
`checked_computational_ih_templates` asks `checked_core_declaration_body_view` for
`main`; that body-view error is **collapsed by the driver to**
`Driver(MissingClosureMetadata { section: "checked computational IH authoritative
runtime body", symbol: [..., "main"] })`.

**The differentiating shape is not resource nesting.** The landed control
`BufferSpan = PrivateBufferSpan (Resource Buffer) Int Nat` already nests the same
`Resource Buffer` in a constructor-private data value and has native-reaching
coverage. The new handle differs by **crossing the higher-order `withBuffer`
body/normalization boundary**. So the fix is in the **compiler layer**, not the
checked representation — the representation is normatively locked by §38 and stands
(a token-only handle cannot supply the raw resource every public consumer needs and
would reopen the authority/ABI boundary Phase 1 deliberately closed).

**Decisive regression evidence:** a *pre-existing two-bracket* native read row
(`fs_read_at_malformed_offset_narrows_to_invalid_offset`), changed **only** by the
required `Resource Buffer -> BufferHandle` API migration, was GREEN before and now
fails before execution with the identical `MissingClosureMetadata`. This candidate
therefore *regresses an already-reachable native buffer program* — which is why
PX8-F-CAP-41 Phase 2 cannot land as a SPAN-PROV-style honest partial.

## First deliverable is diagnostic (do not pick a fix site from the wrapper)

The driver **erases** the underlying `CheckedCoreBodyViewError`, so the observed
`MissingClosureMetadata` does **not** identify the exact missing body-view lane. The
WP must first:
1. **Preserve/report the underlying `CheckedCoreBodyViewError` lane** (de-erase the
   driver collapse) so the exact failing body-view is visible.
2. **Isolation-flip the exact saved fixture** (`f0eb65ce`) against that de-erased
   lane. Only then choose the fix site.

## Acceptance matrix (Architect `evt_2zkjr68y1sdgf` — close every native axis)

- **(a)** normalized checked declaration body view for the higher-order handle path,
  with the underlying error lane visible;
- **(b)** computational-IH census / metadata consistency;
- **(c)** erasure of handle construction, match, and projections;
- **(d)** runtime constructor / value lowering **if the carrier survives
  deforestation** (contingency — see contention flag);
- **(e)** unchanged raw `Resource Buffer` host request and wire ABI;
- **(f)** constructor and both projections remain **absent from the public name map**.

**Controls (all must hold):** the migrated pre-existing two-bracket read row returns
GREEN again; all four CAP-41 rows are **absolute GREEN on both engines** with no
forbidden read/event/host; the landed `BufferSpan` product remains GREEN;
malformed / stale / closed authority behavior is unchanged.

## Fence & contention

- **Primary surface = `ken-elaborator`** (compiler-driver / checked-core / erasure)
  — disjoint from [[RT-NATIVE-FNSPLIT]]'s `ken-runtime`/Cranelift, so the two native
  WPs run **contention-free** as filed.
- **Axis (d) contingency — HALF OF THIS EXPIRED 2026-08-17. Read both clauses
  separately.** As filed it read: *if* the fix requires `ken-runtime`
  constructor/value lowering, that would contend the concurrent RT track — **STOP
  and route to the Steward** to sequence it; and, do not pre-assign the fix to
  `ken-runtime`/Cranelift on current evidence (Architect).
  - **The contention route-back is DISCHARGED.** The concurrent RT track was
    [[RT-NATIVE-FNSPLIT]], now `merged`, and the ring that would do this work is
    the same runtime ring — **it cannot contend with itself.** Do not stop for
    contention, and do not route back for permission on that basis.
  - **The Architect's diagnostic-first clause STILL BINDS.** Do not pre-assign
    the fix site. De-erase the underlying `CheckedCoreBodyViewError` lane and
    isolation-flip `f0eb65ce` first; **only then** choose where the fix goes.
  - **The two clauses had independent bases and only one lapsed.** Retiring both
    because they shared a bullet is the error this note exists to prevent.
  - The `ken-runtime` landing site now exists and is named: the arm goes in
    `lowering/core/primitive.rs` (dispatcher at `:43`), not `core.rs`.
- **Local builds targeted only** (`scripts/ken-cargo -p <crate>`); never
  `--workspace` (COORDINATION §12). Full `-p ken-interp` if the reifier/value shape
  changes (attested `eval.rs` ⇒ OID-bump rider).

## What "done" unblocks

Once the carrier lowers, **fold the fix with the preserved
`preserved/px8-f-cap-41-p2-buffer-handle-f0eb65ce`** (`f0eb65ce`) and run the
full two-engine oracle;
[[PX8-F-CAP-41]] Phase 2 then lands **complete** (interp **and** native GREEN) — no
honest-partial, no operator scope exception. Sibling native-completeness WP:
[[RT-NATIVE-FNSPLIT]] (independent). Root gate: [[PX8]] (this is on the critical
path to PX8 clause-(a) closure via PX8-F-CAP-41).

## Sequencing (Steward)

> **THIS PARAGRAPH IS HISTORY. The frontmatter is authoritative: `status:
> draft`, `owner: runtime`, `size: M`.** It reads `active` and `Size TBD` and
> `Foundation-owned`, all three superseded — the node was re-homed to Runtime
> on 2026-07-23 (see the RE-HOMED section above) and sized after the fix
> surface was scoped. Retained for the filing rationale only.

**`active` 2026-07-23** — filed as the named prerequisite that unblocks
[[PX8-F-CAP-41]] Phase 2, Foundation-owned, replacing PX8-F-CAP-41 Phase 2 as the
Foundation track (the preserved `f0eb65ce` carries the handle/admission impl forward
for the eventual fold). Independent of [[RT-NATIVE-FNSPLIT]] (Runtime, Track 1),
which continues. Size **TBD** until the diagnostic step scopes the fix surface.
