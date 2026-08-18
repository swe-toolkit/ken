---
id: PX8-F-CAP-41
title: "PX8 clause-(a) behavior blocker — closed buffer endpoint (start==capacity) must derive zero-effective ReadEof, not host-reject"
status: draft
owner: foundation
size: M
gate: none
depends_on: [NATIVE-HANDLE-CARRIER, RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT, RT-RECURSIVE-POSITION-ARM-ARITY]
blocks: [PX8]
github: 41
origin: charter backlog (#41); RE-GROUNDED as a live PX8 clause-(a) blocker by architect verdict evt_163mfgjs7fkh8 (2026-07-23); RE-SCOPED spec-first by architect ruling evt_xnkrzjy1c8br (2026-07-23)
---

# THE FOUR `cap41_*` ROWS ARE THIS NODE'S, AND THEY COME BACK HERE — 2026-08-17

**Measured, not inferred.** All four `cap41_*` rows in
`crates/ken-cli/tests/rt_parity_native.rs` were added by a single commit,
`4c9c59d3e` *"WIP PX8-F-CAP-41: seal capacity-carrying buffer handle"*, carried
into [[NATIVE-HANDLE-CARRIER]]'s candidate through its rebase. They are **this
node's acceptance fixture**, not the carrier's deliverable.

They are being **deleted** from that candidate (Steward `evt_7xt2j81m7tevz`),
because a new test switched off in the same commit that ships the work it exists
to measure is a partial claiming its own evidence is unavailable. **They return
here, un-ignored and green, as the missing port's acceptance evidence.**

⇒ **Do not re-add them to any candidate before the port lands**, and do not
`#[ignore]` them with an annotation naming the carrier — the carrier is not what
retires them.

## THE DELETION LEFT A LIVE COVERAGE GAP, AND IT IS CHEAPER TO CLOSE THAN IT LOOKS

**Adversary finding on the landed squash `f9dd79f52`, `evt_3hg4qay5686x0`.
Verified independently before filing.** The carrier implements spec `38 §1.7.1`'s
five-step admission ladder for derived `readAt` at `prelude.rs:2371`
(`private_read_at_admit_window`, with `buffer_min_int` at `:2366`). **Two of
those steps are new behaviour on the checked surface and nothing in the tree
executes them:**

| step | behaviour now in checked code | witnessed by |
|---|---|---|
| 3, tail cap | `effective = min(length, capacity - start)` — a window running past the buffer tail is **silently shortened** and reported as ordinary short progress | nothing |
| 4, closed endpoint | `effective = 0` returns `ReadEof` **from checked code, without emitting a read or visiting the host** | nothing |

**The spec names the witnesses by literal value:** at capacity 8, `(start=8,
length=4)` returns `ReadEof` while `start=9` returns `InvalidBounds`. Those are
exactly the four deleted rows' programs.

> ### THE PROGRAMS ARE STILL IN THE TREE. ONLY THE RUST ROWS WERE DELETED.
>
> `rt_parity_native.rs:349-387` still carries `rt_cap41_endpoint_buffer`,
> `rt_cap41_out_of_range_buffer`, and the offset-precedence pair, with `_file`
> wrappers at `:389-423` and `_stage` wrappers at `:426-459` — inside the
> `RT_PARITY_SOURCE` literal. **Every `cap41` occurrence in `crates/` is inside
> that literal; there are zero Rust-level references**, and
> `assert_cap41_derived_without_read` is gone from the repo.
>
> ⇒ **Closing this gap is writing test functions, not restoring programs.**

**No live fixture reaches either branch, and that is measured rather than
assumed.** The full cross product of `MkBufferWindow` against its `withBuffer`
capacity across `crates/` gives the live pairs `(0,1)@1`, `(0,2)@8`, `(0,6)@6`,
`(0,6)@8`, `(0,8)@8`, `(2,4)@8`, `(-1,1)@1`. **Every one fits its buffer exactly
or sits strictly inside it**, so no live program is tail-capped and none reaches
the closed endpoint. The only windows with `start >= capacity` are the four dead
ones. The capacity-2 handle at `rt_escape_second_resource_native.rs:421` looks
like a live endpoint case and is not — that buffer is never read.

> ### THE WALL THAT BLOCKS THE NEIGHBOURING ROWS DOES NOT BLOCK THIS
>
> The `cap41_*` rows were expensive because `differential()` compiles a native
> artifact, and this fixture's siblings refuse at object emission under
> [[RT-CARRIER-BYTESPAN-OBSERVE]] / [[RT-SITEOP-CARRIED-WITNESS]]. **Steps 3 and
> 4 never visit the host**, so that ceiling does not bind an interpreter-only row
> over the same programs.
>
> **This is a statement about what is NOT blocking, not a prescribed remedy.**
> The shape of the fix is the ring's call when this node is cut.

**One inventory sharpening, because it was recorded as a pair and the two halves
differ.** The Architect's resolution named `rt_body_ok` and
`rt_cap41_expect_eof` together as remaining live. Both still **compile**; only
`rt_body_ok` is **row-reachable**, through `rt_allocate_stage`.
`rt_cap41_expect_eof` (`:335`) is referenced only by the four dead procs.
⇒ **Compiling and running are different properties**, and an inventory that runs
them together will report a dead declaration as live.

## The edge hazard, with its trigger corrected

`depends_on` still names [[NATIVE-HANDLE-CARRIER]], and an earlier banner below
says that makes this node read startable *"the moment the partial merges."*
**That names the wrong event, twice.** `gen-progress.sh` requires **both** that
this node's own `status` be `ready` — it is `draft` — **and** that every
`depends_on` entry be `merged` **or `closed`**. A partial landing on a node that
stays `active` changes nothing.

⇒ **The actual trigger is the carrier's status flip, and `closed` fires it
exactly like `merged` does.** That matters because closing the carrier and moving
its residual to a successor is the disposition currently favoured for it — so the
tidying step is itself the trap.

**Fence DISCHARGED 2026-08-17: the edge is wired.** This node's `depends_on` now
names [[RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT]] alongside
[[NATIVE-HANDLE-CARRIER]], so the carrier's status flip no longer clears Phase 2
on its own and the port has a named owner.

**How the owner was determined, in three measurements, none of them a reading:**

| measurement | result |
|---|---|
| route (`evt_4tqpqn2gpcsx6`) | all five refusing programs take the non-`Construct` scrutinee route — rules out the `RT-WORKER-BIND` / `StaticWorker` lineage as the **first** blocker only |
| scrutinee variant (`evt_2fzzxf778smjj`) | uniformly plain `RuntimeExpr::Match` — kills the `Call` arm, so the closed `RecursiveDescent` residual lineage is not the home |
| plain-`Match` origin (`evt_5zknkg76cn3w5`) | `Var` scrutinee, two non-recursive `Result` cases, `Construct` bodies in both arms — a **local lowerer gap**, not a missed `ComputationalMatch`, so the owner is runtime and not upstream |

**Phase 2 still does not become startable when the port lands.** This node is
`draft` and `gen-progress.sh` requires its own `status: ready` as well as every
`depends_on` entry cleared. Two conditions, and only one of them is about the
port.

> # UNPAIRED 2026-08-17 — Phase 2 does NOT close on [[NATIVE-HANDLE-CARRIER]]'s merge
>
> **Architect ruling `evt_13ax2j6e0jfq2`; Steward disposition, this unpairing is
> the Steward's.** The banner below mandated the opposite and is superseded on
> its own stated ground.
>
> ### Why the pairing dissolved
>
> The pairing rested on the honest-partial ban, and **that ban is discharged.**
> Its warrant was that the candidate *regresses* an already-GREEN native row
> (`AC-5`). Measured 2026-08-17 with `--ignored` at both ends
> (`evt_6h59tq0zpe7dn`): the row refuses **identically** at base `7b8dad7df` and
> at tip `3d23f1182` — same `stage`, `field`, and `reason`, byte-identical.
> ⇒ The candidate regresses nothing, so the ban is stale and the partial is
> admissible.
>
> ⛔⛔ **`AC-5` IS STILL OUTSTANDING. Do not read the lifted ban as a met
> criterion** — the ban being stale and the criterion being satisfied are
> different facts, one word apart in every artifact that describes this.
>
> ### ⇒ What Phase 2 may NOT claim
>
> **The four CAP-41 rows refuse** — *not* "still refuse": measured 2026-08-17,
> they **do not exist** at the carrier candidate's declared base, so *still* is
> false and the word is load-bearing. They refuse at `lowering/core.rs:2929`
> `reject_carried_residual_arguments`. The Architect ruled the gap a **MISSING
> PORT** whose owner is **outside this node** — the determinant is whether the
> body has a declared recursive-position unit, not any property of the carried
> value.
>
> ⇒ **The carrier partial may land; Phase 2 closure may NOT ride on it.**
> Closing Phase 2 on that merge would be **a claim about the past** while the
> rows it names refuse. The pairing's premise — "the fixture cannot land without
> the fix" — turned out to be true in the other direction too: **the fix can land
> without the fixture going green.**
>
> ### The separate-framing prohibition is LIFTED, and Phase 2 needs a scope call
>
> ⛔ The bar below on framing Phase 2 separately **no longer binds** — it existed
> only to enforce the one-merge pairing. Phase 2 now needs one of:
> - **hold the closure claim** until the missing port lands, keeping
>   `depends_on` pointed at the port's owner rather than at the carrier; or
> - **redefine Phase 2 to exclude the refusing rows**, which is a real scope cut
>   and is the Steward's to make.
>
> ⚠ **Neither is chosen yet, and `depends_on` still names
> [[NATIVE-HANDLE-CARRIER]].** That edge is what `gen-progress.sh` reads, so it
> is what will misreport first: when the carrier partial merges, this node's
> only dependency reads `merged` and Phase 2 enters the frontier as startable
> while the rows it exists to green still refuse. ⇒ **Re-point or extend
> `depends_on` at the port owner as part of the scope call, before that merge
> lands.**

> ## SUPERSEDED 2026-08-17 — was: Phase 2 IS FRAMED, and NOT a separate WP (07-27)
>
> **Read as history. Its operative bars are lifted by the banner above.**
>
> **Phase 2 and [[NATIVE-HANDLE-CARRIER]] are ONE deliverable and close in ONE
> merge.** The frame is `docs/program/wp/NATIVE-HANDLE-CARRIER.md` (owner
> **Runtime**, size **M**), measured at `origin/main = 5404108a`.
>
> ⛔ **Do not frame or release Phase 2 separately.** The carrier fix is
> meaningless without the fixture it unblocks, and the fixture cannot land
> without the fix — the Architect ruled an honest partial inadmissible here
> because the candidate *regresses* an already-GREEN native row. ⇒ Flip **both**
> nodes on the same merge.
>
> ⭐ **The Phase 2 WIP is the frame's input**, and it needs no separate fold:
> `f0eb65ce` is the **parent** of the carrier's `c07e63c2`, so `c07e63c2` alone
> already carries the handle/admission impl. The text below says to "fold with
> `f0eb65ce`" — that is a measured premise error; there is nothing to fold.
>
> ⚠ Both refs are recorded as `preserved/*`. A handoff-gate "preserved at …"
> line is a claim, ⛔ not a resolved ref — resolve every name you write down.

> ## ⚠ STATUS CORRECTED `active` → `draft` — 2026-07-25 (Steward, tracker honesty)
>
> **Nothing is building this, and its `depends_on` is unmet** (NATIVE-HANDLE-CARRIER (draft)). The tracker's own
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

## ⚠ RE-SCOPED spec-first 2026-07-23 (two-phase, like [[PX8-SPAN-PROV]])

**The "bounded S-sized prelude fix" premise is FALSE on `origin/main@cbf6a298`.**
On intake, foundation-implementer (evt_7gy73496fwn1p) found — and the Architect
ruled (evt_xnkrzjy1c8br) — that **capacity is not observable in checked Ken**:
`BufferWindow` carries only caller-forgeable `(start, length)`, `Resource Buffer`
is opaque, and `readAt` receives no acquisition capacity. So the checked path
**cannot distinguish `start == capacity` from `start < capacity` or `start >
capacity`**; a caller-supplied capacity would be forgeable (unsound bypass) and a
host primitive would violate the locked no-host-visit postcondition. The RED seed
records this exact missing observation (`seed-buffer-io.md:602-607`).

**Sound mechanism (Architect) — the same unforgeable-acquisition-bound idiom just
merged as `PrivateBufferSpan` in [[PX8-SPAN-PROV]]:** bind capacity to the
**acquisition**, not the request. A constructor-private checked handle
`data BufferHandle = PrivateBufferHandle (Resource Buffer) Int` where
`withBuffer capacity` is the **sole constructor** (stores the just-allocated
capacity); private constructor/projections out of the public name map;
`withBuffer` passes the handle to the body; `readAt`/`writeAt`/`spanBytes` accept
it; release + private host ops project the underlying exact `Resource Buffer`;
`BufferWindow` stays the public raw request descriptor. Host resource token + ABI
unchanged; unforgeable by checked user code. Derived `readAt` admission then:
validate host-width/nonneg `fileOffset`; validate nonneg `start`/`length` +
`start <= capacity`; `effective = min(length, capacity - start)`; derived
`ReadEof`/no-primitive iff `effective == 0`; else `PrivateFsReadAt` on the
effective range (start 8/len 4 cap 8 ⇒ EOF/no-host; start 9 ⇒ `InvalidBounds`;
invalid offset ⇒ error). Keep host `checked_buffer_range` as defense in depth.

**Because this changes the checked buffer-handle surface + all buffer consumers,
it is two-phase:**
- **✅ Phase 1 (spec-first, SPEC ENCLAVE) — MERGED @ `origin/main = 8ebe370a`
  (PR #915, 2026-07-23).** Folded the capacity-carrying `BufferHandle` into §38 +
  the `Resource Buffer` signatures + prelude API; locked the ordered admission
  algorithm + the four absolute `seed-buffer-io.md` rows (RED-until-Phase-2). CV
  independent fold + Architect soundness APPROVE, exact `60cecd3b`. Enclave retros
  requested (thr_220eqm77azw9v).
- **⛔ Phase 2 (impl, FOUNDATION) — BLOCKED 2026-07-23 on [[NATIVE-HANDLE-CARRIER]].**
  Kicked off `8ebe370a` (root `evt_4ea3p6r302xq3`); foundation-implementer completed
  the checked handle + admission + fixtures and hit a **native-lowering hard-stop**
  (evt_563ss8821n7f): the sealed `BufferHandle` does not lower on the native path
  (`Driver(MissingClosureMetadata …)`, pre-Cranelift, pre-erasure — *distinct* from
  RT-NATIVE-FNSPLIT's VReg wall). Architect means/representation ruling
  `evt_2zkjr68y1sdgf`: representation stands; fix the compiler layer; do **not** land
  as an honest partial (it *regresses* an already-GREEN pre-existing 2-bracket native
  row). Interp half is confirmed GREEN on all four rows at `f0eb65ce`. **WIP
  preserved durably on origin** as
  `preserved/px8-f-cap-41-p2-buffer-handle-f0eb65ce`. Foundation
  re-sequenced onto [[NATIVE-HANDLE-CARRIER]]; on that fix, **fold with `f0eb65ce` and
  run the full two-engine oracle** — Phase 2 then lands complete. The (superseded)
  re-sized
  derivation + consumers in `crates/` (elaborator prelude: `PrivateBufferHandle`
  repr + `withBuffer` mint + migrate `readAt`/`writeAt`/`spanBytes`/`freeze`/
  `writeAll` to consume the handle + the derived-`readAt` capacity admission;
  ken-host keeps `checked_buffer_range` as defense in depth, private ops project
  the raw `Resource Buffer`). Oracle = flip the four RED seed rows GREEN (both
  engines). **Sized M.** Foundation cuts a fresh branch off `8ebe370a`.

**No longer merely deferred backlog — the Architect's PX8 closure-property
verdict (`evt_163mfgjs7fkh8`) identifies this as a *live clause-(a) behavior
blocker*.** PX8 cannot close while it is red.

## The defect (Architect-grounded, exact anchors)

LOCKED §38 says `0 <= start <= capacity`; a **positive raw request at
`start == capacity`** has zero effective length and the **derived wrapper must
return without invoking the positive primitive** — a zero-effective `ReadEof`
path with no host visit (`spec/30-surface/38-ffi-io.md:404-408`). Current
`readAt` bypasses the primitive **only when the caller's raw length is zero**
(`crates/ken-elaborator/src/prelude.rs:1977-1986`). A **positive** raw length at
the closed endpoint therefore reaches host dispatch, where `checked_buffer_range`
rejects every `effective == 0` as `InvalidBounds`
(`crates/ken-host/src/effect_v1.rs:1736-1750`).

⇒ The reified checked value is `Err InvalidBounds` where the locked contract
requires the derived zero-effective `ReadEof`. The conformance record states this
explicitly and remains **RED** (`conformance/behavioral/buffer-io/
seed-buffer-io.md:598-614`) — that RED row is the acceptance oracle.

## Scope note / queuing gate

The fix site is bounded (`prelude.rs:1977-1986`: broaden the primitive-bypass
condition from raw-length-zero to effective-length-zero at the closed endpoint,
routing to the derived `ReadEof`), with a LOCKED spec anchor and a RED
conformance row as the absolute oracle ⇒ **conformance/CV in the review lane.**

⚠ **History:** the operator's queuing gate was RELEASED (operator "the plan sgtm"
2026-07-23) and this was kicked to the Foundation ring as Track 2 of two impl
tracks — but **on intake it re-scoped spec-first** (see the banner above): §38
locked the *behavior contract* (derived `ReadEof`/no-host at the closed endpoint)
but the tree has **no capacity representation** to drive that admission, so
Phase 1 must fold the capacity-carrying `BufferHandle` into the spec + API before
any impl. Current status: **Phase 1 MERGED @ `8ebe370a` (PR #915); Phase 2
(Foundation impl) now ACTIVE off that lock.**

Track 1 [[RT-NATIVE-FNSPLIT]] (Runtime) continues unaffected. Sibling clause-(a)
evidence gap: [[PX8-WROTE-ABS]] (A2 — still needs the operator's normative scope
call); clause-(b) gap: [[PX8-SPAN-PROV]] (✅ merged, the idiom this reuses); root:
[[PX8]].
