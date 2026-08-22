---
id: RT-DEAD-ARM-EFFECT-LOWERING
title: "A whole-program-dead but type-total request-handler arm is lowered at full strength, so its ConstructorTag effect seat (claim_host_effect_seat) fails the ENTIRE object emission on a path no execution reaches -- the cut is to lower a provably-unreachable total-handler arm's refusing effect seat to a runtime TRAP (fail-closed), gated on a conservative whole-program construction-site census, keeping the seat's Need-subset-Avail partition strict and unchanged"
status: merged
owner: runtime
size: M
gate: none
depends_on: []
blocks: [NATIVE-HANDLE-CARRIER]
github: null
origin: "Architect ruling evt_7kmh9atsrv80n (thr_4q62g2fmmrxm9, 2026-08-22), on the NATIVE-HANDLE-CARRIER D-final refuse arm. Surfaced by NHC D-final (all five cap41_* rows RED, evt_1srzc4frpjhxn) and pinned by the runtime-implementer's two grounding measurements (evt_8j0tjp15ypw3): the refusing FsWriteFile seat sits in a dead arm of a total FSOp request handler that this program never constructs. The Architect ruled NEITHER the carried-observation route (A) NOR preserve-the-specialization (C) is this fixture's blocker; the immediate fix is dead-arm effect lowering. Steward-filed per COORDINATION section 2."
---

# MERGED — 2026-08-22 (D1 + revised-D1 landed; both refusal sites gated)

Landed at `55c7f51de` (respin; superseded the CI-red `9b9fbf3c1`/PR #2741),
merged to `main` `569ba3d0d` on Decision `dec_4p9n9a0b0rfqq` (Architect
required-reviewer APPROVE `evt_1qnc66xke540m`, differential re-APPROVE on the
respin `evt_7k51qr0dbxx3z`; runtime-qa gate passed). The corrected two-conjunct
deadness predicate (`(1)` never program-constructed AND `(2)` not
runtime-producible via the sealed `NativeProcessSymbols` destructure) closes
the D1 hard-stop hole; both refusal sites consult one shared predicate; the
trap is single-sourced; the ledger keeps `claims` a truthful attestation
(`unreachable` disjoint). AC-1 (narrowed) met per row: no dead-arm refusal
fails object emission; all five governed rows advance to the same live
ResourceRelease/ResourceScalar blocker — the successor
[[RT-RESOURCE-RELEASE-CARRIED-OBSERVE]]. The AC-4 negative control is
non-vacuous (dispatched through a called closure). Adversary hunts the landed
code as usual.

**One CI-red round on the way (benign):** two ken-cli transition-sentinel
anchors (`rt_capture_projection_grow.rs`,
`rt_branched_scrutinee_unit_body_port.rs`) pinned the pre-fix ConstructorTag
terminal state; the dead-arm advance moved them to the ResourceRelease/
ResourceScalar blocker, and the Architect censused the tree (exactly two,
non-vacuity preserved) before the implementer repointed them per the sentinels'
own instructions. The gate gap (local `-p ken-runtime --lib` + `rt_parity_native`
missed the ken-cli sibling pins) is the retro finding; the respin gate is
`-p ken-runtime` all-binaries + `ken-cli` + `ken-verify`.

**Carry (Steward-owned, non-blocking):** make conjunct-`(2)` completeness
STRUCTURAL — route the runtime minting sites through `NativeProcessSymbols`, or
a test asserting every module-level constructor const is a
`NativeProcessSymbols` field. Fail-closed-sound today (a missed field traps its
arm and halts, never miscompiles); cut as
[[RT-NATIVE-VOCAB-STRUCTURAL-COMPLETENESS]], queued behind lane-1 indefinitely,
Architect required reviewer.

Everything below is the node as framed and is retained as the record.

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
  the strict seat (today's behaviour). **(i) as first stated is REFUTED in the
  LIVE direction and CORRECTED — see the "D1 HARD-STOP" section below: a
  whole-program SYNTACTIC construction census is NOT the conservative oracle
  this required, because it is blind to host-synthesized values.)**
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

# D1 HARD-STOP -- CORRECTED DEADNESS CRITERION (Architect `evt_4hcny7ae7h9sb`)

The first D1 cut proved the mechanism's deadness oracle UNSOUND in the LIVE
direction, demonstrated with a witness (runtime-implementer `evt_6wtfb4p5jxhk1`,
WIP `b61923254` -- a NON-merge candidate that reds one existing control). A
whole-program SYNTACTIC construction-site census errs toward DEAD: it sees only
values built by program syntax and is blind to values produced OUTSIDE it.
Witness: an effect RESPONSE `Result::Ok` is host-synthesized, never
`Construct`-ed, so the census "proves dead" the SUCCESS continuation of every
effect. That is the wrong conservatism direction; property (i) demanded
PROVEN-dead and a syntactic census is not a proof. The trap keeps it fail-CLOSED
per (ii) (halt, not miscompile), but it would break working programs -- exactly
the regression (i) exists to prevent. The implementer reported the red control
rather than editing it, which is correct: a control that reds against a new
predicate is the predicate being refuted, not the control going stale.

**CORRECTED CRITERION (Architect rules the direction; the ring builds it).** An
arm on scrutinee `s:T` with constructor `c` is dead iff a value carrying `c` can
never become `s` at runtime. Sound sufficient condition = BOTH:
- **(1)** `c` is never program-constructed -- no `Construct c`, and no
  `RuntimeValue::Constructor c` literal nested through args / record fields /
  closure captures (the second class the ring already widened to, correctly);
  AND
- **(2)** `c` is not producible for a `T`-value by ANY runtime/host origin.
  `host_effect_recipe_tree(operation)` / `SynthesizedFixedConstructorRole`
  (`aggregates.rs:556`) is the authority for the effect-RESPONSE origin and MUST
  be unioned in as LIVE. An `FSOp` REQUEST is program-constructed and never
  host-synthesized (stays in (1), so the FSOp target stays dead cleanly); a
  RESPONSE like `Result::Ok` is host-synthesized (so (2) marks it LIVE). The
  request/response axis is the axis that decides whether a syntactic census is
  sound.

**REVISED-D1 FIRST DELIVERABLE (grounding, not a fork).** Enumerate the COMPLETE
set of non-syntactic `RuntimeValue::Constructor` origins and make the predicate
exhaustive-by-construction over a SEALED origin-kind set with NO catch-all
(COORDINATION section 7): a future origin kind not taught to the predicate must
be a COMPILE error, not a silent unsound trap. Recipe-tree responses are one
origin; ground whether there are others (primitive/builtin results returning
constructors -- `Bool`/`Ordering`/`Result` from comparisons; program-entry
inputs from the host). If unsure whether an origin can yield `c` ⇒ treat LIVE,
never trap. Completeness of (2) is not optional.

**Do NOT take the shortcut "trap any seat that can't be claimed, skip the
deadness analysis."** That turns a loud compile error into a SILENT runtime trap
on genuinely-live effects, masks the real (A) need, and lets a compile-only test
go green falsely. The deadness analysis is load-bearing precisely so the loud
refusal SURVIVES on live arms -- that is the negative control (AC-4).

**Refusal SITES = 2, not 1 (scope note; trigger (b) unchanged).** Behind the
seat refusal (`effects.rs:277`), seven further dead arms refuse at a DIFFERENT
site -- the represented-unavailable-lane check atop `lower_process_host_effect`
(FsAppendFile, FsMetadata, FsReadDirectory, FsCreateDirectory, FsRemoveFile,
FsRemoveDirectory, FsRename). Compute the deadness predicate ONCE per arm and
consult it at each of the ≤N refusal sites (DRY); a modest scope bump, NOT a
switch to trigger (a). The leader's (b) ruling stands (fewer arms touched =
fewer places the predicate can be wrong).

# `D0` -- CENSUS EXISTENCE (first deliverable; determines SIZE, not design)

Does the lowering pipeline already carry a sound constructed-constructor census
(for other dead-code / specialization purposes), or must this node build a
conservative one? Report which, name the mechanism if it exists, and state the
resulting node size. This is the Architect's one in-node scoping input -- a
measurement, not a fork.

# `D1` -- THE FIX (per the corrected criterion above)

Build the deadness predicate as conjunct **(1) ∪ (2)** (see "D1 HARD-STOP"):
program-construction census UNION recipe-tree-synthesized-LIVE, exhaustive-by-
construction over the sealed origin-kind set (the revised-D1 first deliverable).
On a PROVEN-dead arm, emit a trap for the refusing seat; on any arm not proven
dead, leave today's strict lowering untouched. Compute the predicate once per
arm and consult it at each of the ≤N refusal sites (the seat at `effects.rs:277`
and the represented-unavailable-lane check atop `lower_process_host_effect`).
Keep the failing control failing until the predicate is right; do not edit it.

# ACCEPTANCE

- **AC-1 (dead-arm emission unblocked; rows ADVANCE).** NARROWED by the D1
  hard-stop ruling (Architect `evt_4hcny7ae7h9sb`, Finding 2): this node does
  NOT green the `cap41_*` rows. Behind the dead arms the fixtures hit a
  genuinely LIVE refusal (`seat Argument(0) of ResourceRelease needs
  ResourceScalar, ... CarriedWord`; `withResource` IS used), which is
  (A)-family and OUT of this node's scope -- cut as its own node
  [[RT-RESOURCE-RELEASE-CARRIED-OBSERVE]]. This node's honest deliverable:
  **no dead-arm refusal fails object emission, and each of the four `cap41_*`
  rows and the `AC-5` row
  (`fs_read_at_malformed_offset_narrows_to_invalid_offset`, `--ignored`) in
  `crates/ken-cli/tests/rt_parity_native.rs` ADVANCES to its next distinct
  blocker, measured and named.** Report the full per-row disposition; the next
  blocker (expected: the ResourceRelease/ResourceScalar live refusal) is a
  measurement to report, not a failure of this node.
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
  names a terminal state (`a carried recursive hypothesis is an eliminated
  value ... but the call provides 1`) that `c7f462857`
  (RT-CAPTURE-CONTEXT-FRAME-EMIT D1+D2) already cleared -- that string appears
  zero times in the D-final run. CORRECTED in this node's landing (`55c7f51de`):
  the row now names the measured post-fix terminal state.

# SYMPTOM INVENTORY (Architect section 1b)

**Entry 1** (`evt_4hcny7ae7h9sb`, measured by the runtime-implementer at
`b61923254`): *deadness oracle (whole-program syntactic construction census)
unsound in the LIVE direction: a host-synthesized constructor
(`Result::Ok` effect response) reads as never-constructed and is wrongly proven
dead* -- **keyed on:** the census sees only program-syntax origins, not
runtime/host value production.

Disposition: closed by the corrected two-conjunct predicate. Conjunct (2)
unions in the runtime's own constructor vocabulary
(`NativeProcessSymbols`, destructured exhaustively so a new origin is a compile
error). The witness control
(`an_incomplete_duplicate_discarded_or_misobserved_visit_rejects`) passes again
**without being edited**, which is what makes the correction a repair rather
than an accommodation.

**Entry 2** (measured, same node): *a dead-arm control is vacuous unless the arm
is genuinely LOWERED; with a scrutinee lowering can resolve statically, the
unselected arm is folded and its effect is never visited* -- **keyed on:**
instrumenting `lower_process_host_effect` showed the refusing operation absent
from the lowered set entirely, so both rows of the pair compiled for the same
trivial reason. Closed by routing the request through a called closure, which
forces the runtime tag dispatch.

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

`ken-runtime` cranelift backend lowering (`effects.rs`, and per `D0` a small
census helper over `source_occurrences`). NATIVE-HANDLE-CARRIER is held on this
node and touches only the `rt_parity_native.rs` fixture rows (which this node
ADVANCES, not greens). [[RT-RESOURCE-RELEASE-CARRIED-OBSERVE]] -- the (A)-family
successor for the live ResourceRelease/ResourceScalar refusal -- also lives in
`effects.rs`, so it CONTENDS with this node and is SEQUENCED AFTER it in the
runtime ring (single lane, one ring). It is not released while this node is in
flight.

# CAPABILITY TIER

T1-demanding on the soundness reasoning (conservative over-approximation oracle,
fail-closed trap direction, a negative control that genuinely discriminates),
but the DESIGN is fully front-loaded by the Architect ruling. Executable on the
current runtime seat given the ruled design PLUS the two review gates that are
the safety net: the Architect as required reviewer and QA mutation-proof of the
mandatory negative control (AC-4). The runtime ring delivered comparable depth
at its current tier on RT-CAPTURE. Steward runs the kick-time seat check; escalate
only if the seat's live model reads mechanical-only.
