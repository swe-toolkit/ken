---
id: RT-MAPPING-MULTIOP-DISPATCH
title: "Build runtime static-response-owner multi-effect dispatch for SEQUENCED Mapping effects, so a native program that performs a second (and third) Mapping effect after the first — through the simple linear-continuation plane — executes and matches the interpreter. The D5a-core native promotion was only ever exercised on SINGLE-op mapping programs, so everything multi-op sequencing requires was left unbuilt; this closes it as one capability rather than patching D5a-surface layer by layer. Carries the already-built carried-Int (HS#2) and carried-byte-span (HS#3) seat repairs forward and gives them their first executing witnesses."
status: active
owner: runtime
size: unsized
gate: runtime
tier: T1
depends_on: []
blocks: [ABI-S6]
github: null
origin: "Steward-filed 2026-09-10 on the Architect ruling evt_559xpa0pqghx8 (D5a-surface HS#4, thr_67d5fmztsmdzj), a PREREQUISITE / SCOPE SPLIT (not a further already-lawful seat repair). runtime-implementer's directed diagnosis (evt_2mhd52b54w2y8) FALSIFIED the Architect's HS#3 carried-Int-representation premise and reached the response-owner continuation boundary the Architect's conditional named — the (b) branch verbatim (a genuine missing continuation capability, not a representation mismatch). The Architect ruled HS#2/#3/#4 ONE predicate (single-op-only D5a-core promotion) and split this out as the structural closure. Soundness/TCB-adjacent (response-owner decomposition + planner invariants) -> Architect REQUIRED review. Bounded/grounded; does not need the operator's return."
---

> # A fail-closed runtime continuation-capability gap. D5a-core's native Mapping
> # promotion was exercised ONLY on single-op programs, so the static-response
> # owner cannot execute a SECOND Mapping effect sequenced after the first: it
> # calls K, requires the returned carrier tag == `ITree::Ret`, but K returns the
> # pending second `ITree::Vis`, and it fail-closes to a runtime trap. This builds
> # the missing multi-effect dispatch; it does NOT relax the exact-Ret invariant
> # or the planner's recursive-body agreement. Those invariants ARE the soundness
> # gate and the Architect's review criterion.

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
direction, with one native-backend adaptation. This is the target D1 builds to;
D0 (the older-family sizing probe) decides WIRE vs BUILD but NOT this shape.

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

## The one capability, HS#2/#3/#4 are ONE thing (Architect §1b)

Predicate: **the D5a-core native promotion was only ever exercised on SINGLE-op
mapping programs, so everything multi-op sequencing requires was left unbuilt at
every layer it touches.** The three hard-stops were this predicate surfacing at
successively deeper layers, not independent bugs:

- HS#2 — carried Int seats (`MappingReadView`/`MappingWriteView` window Ints):
  moved SPECIALIZED_ONLY -> EITHER_PHASE `carried_exact_int`. Built.
- HS#3 — carried byte-span seat (`MappingWriteView` arg 2): moved to
  `carried_bytes` + `wire_bytes_seat` fail-closed observer. Built.
- HS#4 — response-owner multi-effect dispatch (this node). The layer that makes
  the carried seats actually EXECUTE.

The structural closure is to stop patching D5a-surface layer-by-layer and build
"sequenced multi-op mapping execution" as one capability.

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

## Deliverables (D0-first)

- **D0 — THE deciding sizing question (Architect's, verbatim; answer before D1
  commits a shape).** Does a native program that sequences two OLDER-family
  effects — two Buffer ops, or Buffer->FS, or two FS ops — through THIS SAME
  simple linear-continuation plane EXECUTE (native==interp) today?
  - **YES** => the multi-effect response-owner dispatch capability EXISTS and
    Mapping simply is not wired into it: a bounded "re-home Mapping into the
    existing dispatch" fix.
  - **NO** => this plane lacks multi-effect dispatch universally: a larger
    response-owner / planner build.
  Do not assume older families exercise this exact plane (withFile-style scoping
  may route them through a different transport path with a handler owner
  assigned) — this is an EMPIRICAL probe, run it. Report the answer, the fix shape
  it implies, and a refined size. Hard-stop to the Steward + Architect if the
  answer is NO and the build is materially larger than a re-home (the frame
  re-sizes then).
- **D1 — build the endorsed interpreter-loop dispatch (see "Endorsed design
  target"), in the WIRE-or-BUILD form D0 determines.** Make the static response
  owner the (statically-unrolled) `Vis`-chain interpreter loop: dispatch the
  pending second (and third) `ITree::Vis` rather than fail-closing on non-`Ret`;
  the SAME owner claims every effect via the recursion (no new per-effect owner);
  one shared interpreter body so the three-op planner recursive-body agreement
  holds by construction. No weakening of the exact-`Ret` invariant or the planner
  agreement into unsound acceptance. Statically-bounded chains only (S4).
  Carried-seat repairs carried forward and exercised (S6).

## Acceptance criteria (each with its control)

- **AC-MULTIOP-MATRIX-EXECUTES (the capability).** The full sequenced matrix
  executes and matches the interpreter: `read->read`, `read->write` (write->read
  to the CORRECT bytes), `write->read`, `write->write`, and the three-op
  `read->write->read` / `write->read->write`. Control: each program that today
  reds `UnclassifiedRuntimeTrap{-1}` (or the planner refusal) goes green with
  native==interp parity on ordered non-release effects and the release set; the
  single-op programs stay green (no regression).
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

### The S1-S6 soundness gate (Architect required-review criteria, evt_6yyx5dsyy1h0p)

The Architect's six-point gate refines AC-INVARIANTS-PRESERVED and is the exact
criterion the candidate is reviewed against on return to the Architect. Each is
an acceptance obligation with a control:

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

## Gate, reviewer, sequencing

`gate: runtime`, but SOUNDNESS/TCB-ADJACENT (response-owner decomposition +
planner invariants), so the **Architect owns the soundness gate** (invariants
preserved — AC-INVARIANTS-PRESERVED is THE criterion) and Runtime QA owns the
build. On the candidate: Architect required review + Runtime QA + Adversary on the
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
Architect owns it here.

## Hard stop

Route to the Steward + Architect if D1 cannot dispatch the sequenced second effect
without weakening the exact-`Ret` invariant or the planner's recursive-body
agreement — if the only way to make the matrix execute is to accept an unsound
continuation, the capability is not what this frame assumes and the soundness gate
is at risk. Land nothing on that axis until the Architect rules.
