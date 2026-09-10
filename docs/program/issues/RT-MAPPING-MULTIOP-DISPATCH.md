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
- **D1 — build multi-effect dispatch for sequenced Mapping effects, in the shape
  D0 determined.** The response owner dispatches the pending second (and third)
  `ITree::Vis` rather than fail-closing on non-`Ret`; the three-op planner accepts
  the sequenced recursive-body. No weakening of the exact-`Ret` invariant or the
  planner agreement into unsound acceptance. Carried-seat repairs carried forward
  and exercised.

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

## Research design-input (Architect §1a)

The Architect (evt_559xpa0pqghx8) is posing a sharp research design-input question
on the static-response-owner decomposition to research (idle/ready, warm on the
RT-NESTED-IH continuation boundary). It is Ken-specific continuation reasoning, no
external prior-art analog. It does NOT block D0 — the runtime implementer runs the
empirical older-family sizing probe in parallel; research contributes to the D1
fix design once D0 settles its shape. The mechanical §1a 6th-consecutive trigger
transfers to this WP (Architect owns it here).

## Hard stop

Route to the Steward + Architect if D1 cannot dispatch the sequenced second effect
without weakening the exact-`Ret` invariant or the planner's recursive-body
agreement — if the only way to make the matrix execute is to accept an unsound
continuation, the capability is not what this frame assumes and the soundness gate
is at risk. Land nothing on that axis until the Architect rules.
