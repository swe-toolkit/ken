---
id: RT-MAPPING-MULTIOP-DISPATCH
title: "Build GENERAL runtime static-response-owner dispatch for repeated same-producer / following N>=2 effects through the simple linear-continuation plane, so a native program that sequences a second (and third) effect executes and matches the interpreter with exact-Ret preserved. D0 proved this is a GENERAL plane gap — an older FS-metadata-repeat family traps identically at the same P2 wall (units.rs:3430), not a Mapping-specific defect — so it is fixed ONCE at the response owner; ABI-S6 D5a-surface Mapping multi-op and FS-metadata-repeat are two CONSUMERS of the one fix. Two coordinated sub-parts: P1 the interpreter-loop repeated-dispatch, P2 the planner heterogeneous recursive-body agreement. Carries the Mapping carried-Int (HS#2) and carried-byte-span (HS#3) seat repairs forward atomically and gives them their first executing witnesses."
status: merged
owner: runtime
size: L
gate: runtime
tier: T1
depends_on: []
blocks: [ABI-S6]
github: null
origin: "Steward-filed 2026-09-10 on the Architect ruling evt_559xpa0pqghx8 (D5a-surface HS#4, thr_67d5fmztsmdzj), a PREREQUISITE / SCOPE SPLIT (not a further already-lawful seat repair). runtime-implementer's directed diagnosis (evt_2mhd52b54w2y8) FALSIFIED the Architect's HS#3 carried-Int-representation premise and reached the response-owner continuation boundary the Architect's conditional named — the (b) branch verbatim (a genuine missing continuation capability, not a representation mismatch). The Architect ruled HS#2/#3/#4 ONE predicate (single-op-only D5a-core promotion) and split this out as the structural closure. Soundness/TCB-adjacent (response-owner decomposition + planner invariants) -> Architect REQUIRED review. Bounded/grounded; does not need the operator's return. RESCOPED 2026-09-10 on the Architect ruling evt_7r3ejswrwsxaz (D0=NO adopted): D0 proved the gap is GENERAL (an older FS-metadata-repeat family traps identically), so the node is reframed from Mapping-specific to the general repeated-N>=2 same-producer dispatch capability, sized L, with sub-parts P1/P2 and added criterion S7; the id is kept as the stable handle (reframe, not rename). Design UNCHANGED (S1-S6 stand); the DESIGN needs no operator return to authorize (forced critical path)."
---

> # COMPLETE 2026-09-10 — node MERGED at origin/main 1c48b6c5c
> # ("RT-MAPPING-MULTIOP-DISPATCH respin: fix stack-overflow regression"). The
> # CI-red respin candidate 0fd1eead9 landed; blob-verified 17/17 changed paths
> # against origin/main. Gates on the exact SHA: Runtime QA, Architect delta +
> # M4 APPROVE (evt_2m1jv4q0yxv6n / evt_3k6q8qa406f07), Adversary NO-DEFECT
> # (evt_5qgbdffsq56k0, carrying the prior full-lens APPROVE evt_fsd2bf8e4hrv on
> # the 16 byte-identical files); no Spec/CV gate applied. Decision dec_2d203c2h22qp8
> # resolved APPROVE (resolved_by runtime-leader). The general repeated-N>=2
> # same-producer dispatch capability (P1 interpreter-loop repeated-dispatch + P2
> # planner heterogeneous recursive-body agreement) is built at the response
> # owner; the exact-Ret invariant and planner recursive-body agreement (S1-S7)
> # are preserved, not relaxed. The respin's sole delta vs the S1-S7-approved
> # 187436161 was crates/ken-elaborator/src/modules.rs (+17/-14, RMatch
> # iterator/collect -> explicit capacity-preserving loop) = the AUTHORIZED
> # per-level frame reduction on the structural rewrite descent (RUST_MIN_STACK
> # unset, no budget bump). ABI-S6 D5a-surface Mapping multi-op and the older
> # FS-metadata-repeat family are the two consumers now unblocked. OWED (filed as
> # a separate queued runtime node, NOT blocking this merge — the Adversary's two
> # non-blocking missing-negative-test notes + the Architect S4 positive control):
> # [[RT-MAPPING-DISPATCH-CONTROL-COVERAGE]].
> #
> # A fail-closed runtime continuation-capability gap, now known GENERAL. The
> # static response owner cannot execute a SECOND same-producer effect sequenced
> # after the first: it calls K, requires the returned carrier tag == `ITree::Ret`,
> # but K returns the pending second `ITree::Vis`, and it fail-closes to a runtime
> # trap. D0 proved this is NOT Mapping-specific — an older FS-metadata-repeat
> # family traps identically at the same P2 wall — so it is fixed ONCE at the
> # response owner (the interpreter loop over the `Vis` chain), with D5a-surface
> # Mapping multi-op and FS-metadata-repeat as two consumers. This builds the
> # missing multi-effect dispatch; it does NOT relax the exact-Ret invariant or the
> # planner's recursive-body agreement. Those invariants ARE the soundness gate and
> # the Architect's S1-S7 review criterion.

## The measured defect (Architect evt_559xpa0pqghx8; runtime-implementer evt_2mhd52b54w2y8)

The trap is at `crates/ken-runtime/src/cranelift_backend/lowering/units.rs:3430`:
the static response owner calls K, then requires the returned carrier tag to equal
its exact `ITree::Ret` identity. For a SEQUENCED Mapping program the returned word
is not `Ret` — it is the pending SECOND `ITree::Vis`. The second Mapping effect is
never dispatched (`DeferredResponseSubCase::UnconsumedTransportCaller`,
`handler_owner: None`; raw effect trace = Allocate, first effect, terminal
Release — no second dispatch). The owner demands `Ret` and fail-closes to
`UnclassifiedRuntimeTrap{-1}`.

The complete pre-repair matrix (direct linear continuations, not helper-body
nesting; runtime-implementer, log `/tmp/abi-s6-surface-d1-matrix-direct-pre.log`
SHA-256 `a05641cc…`):

- `read->read`: compiles, native `UnclassifiedRuntimeTrap{-1}`.
- `read->write`: `MappingWriteView Argument(2) BytesPointerLength/CarriedWord`
  refusal (the HS#3 byte-span seat) — now lowers after the authorized byte-seat
  repair, then takes the SAME runtime trap.
- `write->read`, `write->write`: compile, native `UnclassifiedRuntimeTrap{-1}`.
- `read->write->read`, `write->read->write`: planner refusal — "one governed
  invocation's typed continuation units disagree on their declared recursive
  body" (the three-op planner recursive-body agreement).

The carried seats/readers are EXONERATED: substituting known-valid Ints AND
bypassing both Mapping resource guards still traps, so it is NOT
`narrow_carried_int_u64` and NOT the resource observer; `read->read` failing
identically proves it is not write-specific and not carried-representation at all.
This is a genuine continuation-capability gap in the static-response-owner P2
decomposition. D0 RE-MEASURES the exact loci at pickup (line numbers drift).

## D0 RESOLVED: NO — general plane gap (evt_17wdtegrxyb1j; adopted evt_7r3ejswrwsxaz)

D0 (the older-family sizing probe) is ANSWERED and the answer is **NO**: the same
simple linear-continuation plane lacks multi-effect dispatch UNIVERSALLY, not just
for Mapping. An older FS family — a `withResource ... ResourceMetadata` body doing
a direct linear `bind (resourceMetadata file) (\first. bind (resourceMetadata
file) (\second. Ret ...))`, and its x3 sibling — builds natively and traps
IDENTICALLY:

- interpreter: exit 0, `NormalReturn`; trace `[FsOpen, FsHandleMetadata,
  FsHandleMetadata, ResourceRelease]` (x3 adds a third `FsHandleMetadata`), all
  metadata outcomes `FileMetadataV1 { size: 13, kind: File }`;
- native: `Err(UnclassifiedRuntimeTrap { terminal_value: -1 })` — the SAME
  host-independent trap at `units.rs:3430`. For x2 the first `FsHandleMetadata` is
  Specialized, the following is `UnconsumedTransportCaller` (`handler_owner=None`);
  for x3 the first is Specialized and the next two are Deferred P2, both
  `handler_owner=None`. The first owner's K leads to the next pending `Vis`, not
  `Ret`.

Evidence: `/tmp/rt-mapping-d0-fs-metadata-compact.log` SHA-256 `796ccbc7…`;
staticlib materialization SHA-256 `229b9a5e…`. Current mechanism loci at the
carried tip `d24529b6`: `responses.rs:2369`
(`requires_execute_then_resume = ordinary_stage_count >= 2` — the promotion exists
but repeated same-producer stages fall OUTSIDE it), `responses.rs:2418` (those
demands become `UnconsumedTransportCaller`), `units.rs:3343-3430` (owner requires
exact `Ret`), `aggregates.rs:6299` (the heterogeneous Mapping three-op recursive-
body disagreement — P2). The FS-homogeneous x3 reaches native build and traps on
the P1 gap, BYPASSING the planner refusal, which proves P2 is a SEPARATE
sub-problem (the disagreement is specific to heterogeneous op bodies in one
governed invocation), not merely P1 surfacing earlier. So the fix is an L/T1
GENERAL response-owner/planner build, and it is fixed ONCE at the response owner.

## Soundness — completeness at the continuation layer, invariants preserved

The response owner's exact-`Ret` requirement and the three-op planner's
recursive-body agreement are fail-closed invariants: an unsequenced program that
returns a non-`Ret` word correctly traps rather than executing a bogus
continuation. The fix must make the owner DISPATCH the pending second `ITree::Vis`
(the genuine missing capability), NOT weaken the exact-`Ret` check into accepting
any word, and NOT relax the planner's recursive-body agreement into admitting
disagreeing units. TCB-adjacent (response-owner decomposition + planner
invariants) — this is the Architect's required-review criterion.

## Endorsed design target (Architect ruling evt_6yyx5dsyy1h0p)

The Architect ENDORSED research's advisory (evt_1dq89yd8pej68) as the design
direction, with one native-backend adaptation, and CONFIRMED it unchanged on the
D0=NO rescope (evt_7r3ejswrwsxaz). This is the target the build realizes. D0
resolved NO, so the branch is BUILD (construct the loop on this plane), and it is
GENERAL — producer-agnostic, since the interpreter loop is an eliminator over the
`ITree` `Vis` chain and does not care which producer minted each `Vis`. It covers
FS-metadata-repeat, Mapping same-op repeat, and the two-op Mapping owner trap
alike.

THE SHAPE. Ken's IR literally IS an interaction tree (`ITree::Ret` /
`ITree::Vis`), so the sound static response owner is the free-monad/ITree
interpreter over the pending-`Vis` chain: dispatch each effect, thread its
continuation's result to the next, and demand `Ret` ONLY at the recursion base;
a `Vis` is the RECURSIVE case (dispatch-and-continue), NEVER a terminal.
`units.rs:3430` is that loop UNROLLED EXACTLY ONCE — it handles the first `Vis`,
fails to recurse on K's result, then demands `Ret`, so the legitimately-returned
second `Vis` reads as a failed terminal. The fix makes the owner the loop it
already half-is. Both guards STRENGTHEN, not weaken: (i) exact-`Ret` is demanded
only at the true base — the current fail-close conflates "a `Vis` where I
expected `Ret`" with "non-terminal failure"; the loop distinguishes them
structurally; (ii) the 3-op "typed continuation units disagree on their declared
recursive body" is per-op decomposition declaring a body per op — one shared
interpreter body makes agreement hold by construction. `handler_owner=None` is a
MIS-DECOMPOSITION (the first effect's result was never threaded back to the same
owner to produce the next `Vis`), cured by the same owner claiming EVERY effect
via the recursion — NOT by constructing a new per-effect owner. The 3-op planner
refusal is the SAME missing multi-dispatch surfacing earlier, cured by the one
shared body — not a distinct invariant. Q1 = (a) owner-recurses-over-the-chain;
reserve nested-owner (b) for genuinely nested handler scopes (none here — (b)
multiplies bodies and re-creates the #3 refusal).

NATIVE-BACKEND ADAPTATION (Architect's, load-bearing). Ken's native path is a
STATIC cranelift lowering, not a dynamic interpreter. So the native realization
is a STATIC UNROLL of a STATICALLY-BOUNDED `Vis` chain (N known at plan time for
a straight-line multi-op `withMapping` body), threading each continuation,
terminating at `Ret` — the single-effect execute-then-resume (inc3/D1)
generalized from one step to N. The interpreter loop is the SEMANTICS; the static
unroll is its native realization, and it MUST equal the interp path
(native==interp). This is why statically-bounded chains are the hard boundary
(S4): effectful-recursion / unbounded / multi-shot effect trees are OUT OF SCOPE
and must be REFUSED fail-closed, never silently unrolled.

## The one capability, fixed ONCE at the response owner (Architect §1b closure)

Predicate: **the static response owner was only ever exercised on programs whose
governed invocation does at most ONE effect per producer, so repeated-N>=2
same-producer dispatch was left unbuilt — for EVERY producer that reaches this
plane, not just Mapping.** D0 proved this general: the FS-metadata-repeat family
traps identically. So the structural closure is to build the general
repeated-dispatch capability ONCE at the response owner; the earlier Mapping
hard-stops and the FS finding are two CONSUMERS of it, not independent bugs:

- HS#2 — carried Int seats (`MappingReadView`/`MappingWriteView` window Ints):
  moved SPECIALIZED_ONLY -> EITHER_PHASE `carried_exact_int`. Built.
- HS#3 — carried byte-span seat (`MappingWriteView` arg 2): moved to
  `carried_bytes` + `wire_bytes_seat` fail-closed observer. Built.
- HS#4 — response-owner multi-effect dispatch (this node), now GENERAL. The layer
  that makes the carried seats actually EXECUTE, and that D5a-surface Mapping
  multi-op AND FS-metadata-repeat both consume.

The structural closure is to stop patching one producer's surface layer-by-layer
and build "repeated same-producer / following N>=2 effect dispatch through the
static response owner" as one general capability.

## Carried-seat atomicity (the Steward/runtime-leader cut)

The HS#2 Int-seat and HS#3 byte-span-seat repairs are correct, but a carried seat
is only EXERCISED when a value crosses a continuation to a SECOND op — which does
not execute until multi-op dispatch works. So they have NO executing
(native==interp) witness today: a mutation reds their lowering, but nothing proves
their runtime behavior. They are semantically atomic with the response-owner fix
and must land WITH it — they are NOT a mergeable partial on their own.

**Cut: this node CARRIES `wp/ABI-S6-mapping-surface` FORWARD from the clean WIP
checkpoint `8feb193ccf1c4cdf6b52241ed5db56f27f4c114b`** (runtime-implementer's
non-candidate checkpoint: single-op surface + both carried-seat repairs, contract
controls green). The response-owner fix builds on top; the carried seats get their
executing witnesses here and land atomically in one candidate. runtime-leader
confirms the base at pickup (or proposes the fold-in alternative).

## Deliverables (D0 resolved; two coordinated sub-parts P1 then P2)

- **D0 — RESOLVED: NO, general (see "D0 RESOLVED").** The sizing question is
  answered — the plane lacks multi-effect dispatch universally; the fix is an
  L/T1 general response-owner/planner build, not a Mapping re-home. No further
  sizing probe owed. Build P1 then P2 to the settled design target.
- **P1 — general repeated-dispatch (the interpreter loop).** Generalize the
  existing execute-then-resume promotion so a FOLLOWING same-producer stage gets a
  handler owner and is dispatched: `responses.rs:2369`
  (`requires_execute_then_resume = ordinary_stage_count >= 2` — the promotion
  exists but repeated same-producer stages fall outside it) + `responses.rs:2418`
  (those demands become `UnconsumedTransportCaller`) + the owner at
  `units.rs:3343-3430` recursing to `Ret`-at-base. The SAME owner claims every
  effect via the recursion (no new per-effect owner). This cures the
  FS-metadata-repeat, the Mapping same-op repeat, AND the two-op Mapping owner
  trap. Statically-bounded chains only (S4). No weakening of the exact-`Ret`
  invariant. Carried-seat repairs carried forward and exercised through P1's
  continuation threading (S6).
- **P2 — planner recursive-body agreement (heterogeneous multi-op), DISTINCT from
  P1.** For HETEROGENEOUS multi-op governed invocations (the Mapping three-op
  forms) reconcile the planner recursive-body agreement at `aggregates.rs:6299`
  via the one-shared-body (S3). This is verified INDEPENDENTLY of P1: the
  FS-homogeneous x3 reaches native build and traps on the P1 gap, BYPASSING the
  planner refusal — proving the disagreement is specific to heterogeneous op
  bodies in one invocation, not merely P1 surfacing earlier (this refines
  research's Q3, which slightly over-unified the two). No relaxation of the
  agreement into admitting genuinely disagreeing units.

## Acceptance criteria (each with its control)

- **AC-MULTIOP-MATRIX-EXECUTES (the capability — both consumers).** The full
  sequenced matrix executes and matches the interpreter across BOTH consumers of
  the one fix:
  - Mapping (D5a-surface): `read->read`, `read->write` (write->read to the CORRECT
    bytes), `write->read`, `write->write`, and the three-op `read->write->read` /
    `write->read->write`.
  - FS-metadata-repeat (the D0 consumer): the x2 and x3 `resourceMetadata` repeat
    programs execute native==interp (`NormalReturn`, exact trace, identical
    `FileMetadataV1` outcomes).
  Control: each program that today reds `UnclassifiedRuntimeTrap{-1}` (or the
  planner refusal) goes green with native==interp parity on ordered non-release
  effects and the release set; the single-op programs stay green (no regression).
- **AC-P2-HETEROGENEOUS (P2, verified independently of P1).** A heterogeneous
  Mapping three-op governed invocation reaches native build POST-FIX (the
  `aggregates.rs:6299` recursive-body disagreement is reconciled), then executes
  via P1. Control: before P2 the heterogeneous three-op form reds the planner
  refusal while the FS-homogeneous x3 does not (the two are distinct); after P2
  the heterogeneous form builds and executes, and a mutation re-introducing the
  per-op body disagreement reds it — without admitting genuinely disagreeing
  units.
- **AC-CARRIED-SEATS-WITNESSED (atomicity).** The carried Int-window seats (all
  three) and the carried byte-span seat now have an EXECUTING native==interp
  witness — a sequenced program that carries each across the continuation and
  observes the correct result. Control: a mutation returning any carried seat to
  SPECIALIZED_ONLY (or the byte seat to the non-carried form) reds exactly its
  carried-sequential program at runtime, not merely at lowering.
- **AC-INVARIANTS-PRESERVED (the soundness gate — hard).** The exact-`Ret`
  response-owner check and the planner's recursive-body agreement still fail-close
  on genuinely malformed continuations. Control: an unsequenced program returning
  a non-`Ret` word still traps; a mutation that makes the owner accept any word
  (rather than dispatching the pending `Vis`), or the planner admit disagreeing
  units, reddens this control. Building dispatch must NOT widen the invariant into
  accepting unsound continuations — the Architect's required-review criterion.
- **AC-NO-TCB-WIDENING.** No new op / wire / right / effect / continuation
  primitive; the frozen anonymous three-op Mapping wire is unchanged; no new
  trusted surface. Control: `trusted_base()` differential delta zero; the diff is
  confined to the response-owner dispatch + P2 decomposition + planner
  recursive-body agreement in `units.rs` and the cranelift lowering, and the
  carried-seat repairs carried forward.
- **AC-AFFECTED-CLOSURE.** Re-run every runtime/cli target whose response-owner /
  planner / mapping-lowering path this touches (targeted via `scripts/ken-cargo`,
  never `--workspace`; CI is the workspace verdict). `rt_parity_native` and the
  mapping-surface suites green.

### The S1-S7 soundness gate (Architect required-review criteria)

The Architect's gate refines AC-INVARIANTS-PRESERVED and is the exact criterion
the candidate is reviewed against on return to the Architect. S1-S6 are the
original gate; S7 was added on the general-scope rescope (evt_7r3ejswrwsxaz)
because the fix is now on a SHARED response-owner path. Each is an acceptance
obligation with a control:

- **S1 — exact-`Ret` stays load-bearing.** The recursion's node dispatch is a
  SEALED, fail-closed match: `Vis` => recurse ONLY if the effect is dispatchable;
  `Ret` => complete ONLY at the base; any malformed/undispatchable node
  fail-closes (honest refusal, no silent catch-all, COORDINATION §7). Widening
  "`Ret` here" to "`Ret`-at-base OR dispatchable-`Vis`-recurse" must admit NO
  malformed/undispatchable node as terminal. Control: a malformed/undispatchable
  following node still fail-closes.
- **S2 — per-effect capability admission fires on effects 2..N exactly as on
  effect 1.** The loop changes response-owner THREADING only, never per-effect
  checks: resource/right/protection admission, opacity absolute (no address
  crosses to Ken/token), bounds-check-NOT-clamp (out-of-range rejects), and
  revocation/liveness re-checked at EACH access all fire on the 2nd..Nth Mapping
  effect identically. Controls: ReadOnly-refuses-`mapWrite`, forged-handle,
  wrong-kind, out-of-range, revoked-mid-sequence — each on effect 2, not just
  effect 1.
- **S3 — one shared body preserves per-node effect TYPING.** "One recursive body"
  means one uniform dispatch structure, NOT type erasure; each `Vis` retains its
  own effect type and continuation type. Verify no per-op typing is lost in the
  collapse.
- **S4 — HARD BOUNDARY: statically-bounded chains ONLY.** Effectful-recursion /
  unbounded / multi-shot effect trees are OUT OF SCOPE and must be REFUSED
  (fail-closed), never silently unrolled or looped. Control: if such a sequence
  is constructible at the D5a surface, it is shown refused; if not constructible
  here, the boundary is stated explicitly so a future increment does not lean on
  this loop for the unbounded case (which carries its own totality obligation).
- **S5 — native==interp trace parity on the COMPLETE multi-op matrix.** The
  interp path is the reference the loop makes native match: variant / exit /
  ordered-trace / release-set / dispatch-skip all identical, plus write->read
  executing to the correct bytes.
- **S6 — carried-seat co-validation (the atomicity payoff).** The loop's
  threading of e1's result into e2's dispatch is EXACTLY what exercises the HS#2
  Int seats and the HS#3 byte-span seat across a continuation — so the carried
  seats carried forward on `wp/ABI-S6-mapping-surface` get their executing
  witnesses through this mechanism. Control: a mutation breaking the continuation
  threading must red a carried-sequential witness program (not only the
  response-owner control).
- **S7 — ADDITIVE and INERT on the existing corpus (added on the general-scope
  rescope).** Because the generalization is on a SHARED response-owner path, every
  program that works today must be UNCHANGED: single-effect programs, and any
  heterogeneous N>=2 sequence the existing `ordinary_stage_count >= 2` promotion
  already handles. native==interp parity preserved on the existing effect corpus;
  the widened gating fires ONLY on the previously-excluded repeated-stage case;
  census the non-diff dimensions (ID/numbering, stack depth) and do NOT
  re-baseline a working path as if it were the new capability. Control: the
  existing effect corpus stays green with identical traces/numbering; a new
  capability is zero-cost when unused, and any change to a currently-working
  sequence reds this control (it is a regression, never a rebaseline).

`gate: runtime`, but SOUNDNESS/TCB-ADJACENT (response-owner decomposition +
planner invariants), so the **Architect owns the soundness gate** (invariants
preserved — the S1-S7 gate is THE criterion) and Runtime QA owns the build. On the
candidate: Architect required review AGAINST S1-S7 + Runtime QA + Adversary on the
exact SHA, then Steward M1-M4 -> lieutenant. CV re-review of the D5a-surface
conformance seed applies when D5a-surface's own multi-op acceptance resumes on
this node's landing (this node is runtime continuation machinery, not the
conformance surface). `blocks: ABI-S6` — specifically D5a-surface's remaining
multi-op acceptance (full matrix, write->read to correct bytes native==interp),
which the Steward re-releases on this node's landing; the carried seats get their
executing witnesses then.

## Research design-input (Architect §1a — DISCHARGED)

Research delivered the §1a design-input advisory (evt_1dq89yd8pej68): the sound
shape is the free-monad/ITree interpreter loop (owner recurses over the `Vis`
chain, `Ret` at base, one shared body), strengthening both guards; the structural
answer is D0-branch-independent. The Architect ENDORSED it (evt_6yyx5dsyy1h0p)
with the native static-unroll adaptation and the S1-S6 gate — both folded into
"Endorsed design target" and the acceptance above. The research pull is
DISCHARGED; the mechanical §1a 6th-consecutive trigger remains on this WP and the
Architect owns it here. The D0=NO rescope did NOT re-fire §1a
(evt_7r3ejswrwsxaz): the FS probe CONFIRMED the design in hand (same wall, same
loci), so it is a scope refinement, not a new unaided round.

## Hard stop

Route to the Steward + Architect if P1 cannot dispatch the sequenced following
effect, or P2 cannot reconcile the heterogeneous recursive-body, without weakening
the exact-`Ret` invariant or admitting genuinely disagreeing units — if the only
way to make the matrix execute is to accept an unsound continuation, the capability
is not what this frame assumes and the soundness gate is at risk. Land nothing on
that axis until the Architect rules.
