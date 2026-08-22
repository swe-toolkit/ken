---
id: RT-DEAD-ARM-EFFECT-LOWERING
title: "A whole-program-dead but type-total request-handler arm is lowered at full strength, so its ConstructorTag effect seat (claim_host_effect_seat) fails the ENTIRE object emission on a path no execution reaches -- the cut is to lower a provably-unreachable total-handler arm's refusing effect seat to a runtime TRAP (fail-closed), gated on a conservative whole-program construction-site census, keeping the seat's Need-subset-Avail partition strict and unchanged"
status: ready
owner: runtime
size: M
gate: none
depends_on: []
blocks: [NATIVE-HANDLE-CARRIER]
github: null
origin: "Architect ruling evt_7kmh9atsrv80n (thr_4q62g2fmmrxm9, 2026-08-22), on the NATIVE-HANDLE-CARRIER D-final refuse arm. Surfaced by NHC D-final (all five cap41_* rows RED, evt_1srzc4frpjhxn) and pinned by the runtime-implementer's two grounding measurements (evt_8j0tjp15ypw3): the refusing FsWriteFile seat sits in a dead arm of a total FSOp request handler that this program never constructs. The Architect ruled NEITHER the carried-observation route (A) NOR preserve-the-specialization (C) is this fixture's blocker; the immediate fix is dead-arm effect lowering. Steward-filed per COORDINATION section 2."
---

# WHAT THIS NODE IS

A completeness gap in host-effect-seat lowering, exposed (not caused) by the
RT-CAPTURE chain moving the `cap41_*` rows past their old stop. A
whole-program-dead but type-total request-handler arm is lowered at full
strength, and its `ConstructorTag` effect seat fails the ENTIRE object emission
on a path no execution of the program reaches.

**This is not the carried-observation route (A) and not
preserve-the-specialization (C).** The Architect ruled both out for this
population (`evt_7kmh9atsrv80n`); (A) is a real but DEFERRED successor for a
genuinely live, runtime-varying policy, explicitly carried and not cut here.

# THE DEFECT, AS MEASURED

Measured by the runtime-implementer at `75b573c1d` (`evt_1srzc4frpjhxn`,
`evt_8j0tjp15ypw3`); the `ken-runtime` lowering sites are unchanged through
current `main` (later tips advanced only via `ken-elaborator` and doc-only).

- **The refusal**, byte-identical on all five governed rows:
  `unsupported runtime-IR lowering: Effect: seat Argument(1) of FsWriteFile
  needs ConstructorTag, which it cannot observe in CarriedWord`, wrapped as
  `Packaging(ObjectLinkerPackagingError { stage: ObjectEmission, field:
  "checked_process_object", ... })`.
- **Static call site:** `crates/ken-runtime/src/cranelift_backend/lowering/
  effects.rs:277` -- the `if !admits` refusal in `claim_host_effect_seat` (fn at
  `:227`), the `Need`-subset-`Avail` membership test; message format at `:280`.
  The seat row is `planning/static_transition/effects.rs:420`
  (`(Op::FsWriteFile, 1) | (Op::FsOpen, 1) => Some(tag)`) with `tag =
  (SelectClosedTag, Need::ConstructorTag, Avail::SPECIALIZED_ONLY)` at `:362`.
- **The arm is dead.** The refusing `FsWriteFile` is not applied at a call
  site; it is an arm of the `FSOp` request-handler `match`, and `Argument(1)`
  (the `CreatePolicy`) is a FIELD of the matched request constructor (`Var(2)`,
  a match binder). A whole-program census over every emittable unit body found
  `FSOp::WriteFile` constructed **0** times (all ten named `FSOp` ops: 0
  constructions; only three anonymous `ctor_49x` ops are ever constructed). The
  handler is total over `FSOp`, so the arm is lowered unconditionally and
  refuses on a value no execution constructs.
- **The failure is entry-sensitive and attributable:** two rows in the same
  file PASS compiling the identical source with only the entry substituted,
  because they do not route through the `FSOp` coproduct handler. The five red
  rows fail on an arm their own entries never enter.

# THE RULED DESIGN -- Architect `evt_7kmh9atsrv80n` (definitive)

When a total-handler arm is PROVEN unreachable by a whole-program
construction-site census, lower its refusing effect seat to a runtime TRAP
instead of failing object emission. **Keep the seat strict; gate the ARM's
lowering on reachability, not the seat's `Avail`.**

This STRENGTHENS what compiles, so it is soundness-sensitive and carries the
same direction discipline as the kernel SCT ruling. Three properties are
inviolable:

- **(i) Conservative oracle, fail-closed to strict.** The reachability census
  is a sound OVER-approximation: an arm is treated dead only if PROVEN
  never-constructed program-wide. Anything not proven dead stays LIVE and keeps
  the strict seat (today's behaviour).
- **(ii) The substitute is a TRAP, and this is load-bearing.** A trap
  (abort / `unreachable`) is the only substitute that is fail-closed under an
  incomplete census: if a request value ever reaches this arm from outside the
  census's view (e.g. constructed across the checked-continuation boundary), the
  program HALTS -- it never yields a wrong result and never relaxes a capability
  gate. Trap-RESULT-soundness does not depend on census completeness; only
  trap-LIVENESS does. Do NOT elide the arm (breaks match totality / control
  flow) and do NOT realize it as a silent success.
- **(iii) Narrowest trigger + mandatory negative control.** Change behaviour
  ONLY on arms that today cause an unsatisfiable emission refusal; preserve all
  currently-compiling lowering. The negative control is the discriminator that
  proves the fix did not over-accept and it is not optional.

**REJECTED: option 2** (admit a carried tag only where the arm is unreachable).
It makes a static capability gate (`Need` subset `Avail`) depend on a global
reachability property, coupling the seat abstraction to whole-program analysis
-- the same "a dynamic/contextual property names a static capability" smell the
`RT-NATIVE-FNSPLIT` closure retired. Keep the seat strict.

# `D0` -- CENSUS EXISTENCE (first deliverable; determines SIZE, not design)

Does the lowering pipeline already carry a sound constructed-constructor census
(for other dead-code / specialization purposes), or must this node build a
conservative one? Report which, name the mechanism if it exists, and state the
resulting node size. This is the Architect's one in-node scoping input -- a
measurement, not a fork.

# `D1` -- THE FIX

Gate the refusing arm's effect-seat lowering on the (existing or newly-built,
per `D0`) conservative construction-site census: on a PROVEN-dead arm, emit a
trap for the refusing seat; on any arm not proven dead, leave today's strict
lowering untouched.

# ACCEPTANCE

- **AC-1 (rows green).** The four `cap41_*` rows and the `AC-5` row
  (`fs_read_at_malformed_offset_narrows_to_invalid_offset`, `--ignored`) in
  `crates/ken-cli/tests/rt_parity_native.rs` compile past the `ObjectEmission`
  refusal. NOTE: greening the NATIVE lowering may expose a distinct downstream
  interpreter-parity result -- the implementer's D-final observed the native
  build fails FIRST, so the interpreter half of `differential` was never
  exercised. Report the full per-row disposition; a newly-exposed distinct
  blocker is a measurement to report, not a failure of this node.
- **AC-2 (conservative oracle).** The census is a sound over-approximation; an
  arm not proven dead keeps the strict seat. State the soundness argument.
- **AC-3 (trap, not elision, not silent success).** The dead arm's refusing
  seat lowers to an abort/`unreachable`; match totality and control flow are
  preserved. A test observes the trap form, not a removed arm.
- **AC-4 (MANDATORY negative control).** A program that DOES construct
  `FSOp::WriteFile` must STILL lower the seat at full strength -- and thus still
  refuse today, pending (A). The trap must never fire for a reachable arm. This
  control must genuinely discriminate (QA mutation-proof), not be a vacuous
  stand-in; a fixture that constructs the constructor must not be trapped into a
  false pass.
- **AC-5 (no regression).** All currently-compiling lowering is preserved;
  workspace-green in CI. (Local: targeted `-p` only, never `--workspace`.)
- **Required reviewer:** the Architect is the required reviewer on this node's
  merge Decision (soundness-sensitive lowering change). Adversary hunts the
  landed code.

# HOUSEKEEPING FOLDED IN (Architect + implementer flagged)

- The `AC-5` row's `#[ignore]` reason at `rt_parity_native.rs:745` is STALE: it
  names a terminal state (`a carried recursive hypothesis is an eliminated value
  ... but the call provides 1`) that `c7f462857` (RT-CAPTURE-CONTEXT-FRAME-EMIT
  D1+D2) already cleared -- that string appears zero times in the D-final run.
  Correct it when this node touches that row.

# EXPLICITLY NOT IN SCOPE

- **Route (A)** -- the carried-observation route + runtime tag-namespace
  translation (boundary tag id -> wire `CreatePolicy` 0/1/2), on the
  `lower_buffer_freeze_resource_seat` (`effects.rs:1612`, `EITHER_PHASE`)
  precedent. It is the fix for a genuinely LIVE, runtime-varying policy, and it
  additionally carries a bounded Spec contract question (does Ken's
  `FsWriteFile`/`FsOpen` host-op contract permit a runtime, non-statically-
  determined `CreatePolicy`/`ResourceOpenMode`?). None of that gates this node.
  Carry, do not cut.
- **Any change to the seat's `Avail` partition** or to `create_policy_tag`
  (`lowering/mod.rs:8102`, which already folds compile-time-literal fields by
  constructor name). Keep the seat strict.
- **Any kernel / TCB edit.** This is cranelift-backend lowering policy.

## The family predicate — scopes (A) right (Architect section 1b)

Not a licence to build a general closure now.

The `ConstructorTag` seat demand is well-founded exactly when the arm is
REACHABLE and the constructor field is RUNTIME-VARYING. It is spurious (a) on
unreachable arms -- this fixture, handled by THIS node; (b) on
compile-time-literal fields -- already handled by `create_policy_tag`. The
runtime-varying live case is (A). The fixtures hit ONLY case (a).

# CONTENTION

`ken-runtime` cranelift backend lowering (`effects.rs`, and per `D0` possibly a
census helper). No other lane-1 node is open on these files;
NATIVE-HANDLE-CARRIER is held on this node and touches only the
`rt_parity_native.rs` fixture rows (which this node greens). No `crates/`
contention.

# CAPABILITY TIER

T1-demanding on the soundness reasoning (conservative over-approximation oracle,
fail-closed trap direction, a negative control that genuinely discriminates),
but the DESIGN is fully front-loaded by the Architect ruling. Executable on the
current runtime seat given the ruled design PLUS the two review gates that are
the safety net: the Architect as required reviewer and QA mutation-proof of the
mandatory negative control (AC-4). The runtime ring delivered comparable depth
at its current tier on RT-CAPTURE. Steward runs the kick-time seat check; escalate
only if the seat's live model reads mechanical-only.
