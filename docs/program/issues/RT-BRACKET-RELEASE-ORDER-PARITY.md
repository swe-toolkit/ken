---
id: RT-BRACKET-RELEASE-ORDER-PARITY
title: "Bracket teardown order. There is ONE shape, not two: Ken's surface cannot express two resources in one scope, so every measured case is a NEST of single-resource brackets, and inner-before-outer is FORCED by bracket semantics -- AFFIRMED from locked text by the Spec enclave at evt_17kyxq7q5v8ar. MEASURED at 89d2bfb57 across two closed rounds: native violates the rule on all SIX reaching depth-2 nests and satisfies it on BOTH reaching depth-3 nests, so mode, inner-kind, outer-kind, homogeneity, combinator AND read-vs-write are all dead as leads; interp is correct on all FOUR measured composed programs, so there is no MEASURED interp defect and no interp repair is authorized -- but THREE of the five existing composed-return fixtures are measured and TWO remain UNMEASURED, and there plainly IS an engine contrast at measured depth 2 where native is wrong and interp right. ARCHITECT RULED the cause at evt_4t14zmba83hjm: depth is the SELECTOR, not the cause -- the causal boundary is Specialized or handler-owned execution versus unowned Deferred forward-Ret, where a statically bounded bracket-settlement continuation stays on the unowned route although it must execute before the enclosing bracket resumes. D1a is authorized ONLY as two required arms: a BOUNDED AUTHORITY-SEED predicate in static_response_phase_b_split, and a release-only third class in bounded_deferred_response_suffix. Either arm alone leaves half the population wrong. ARM A CORRECTED at evt_76nkdg0h81xnw (hard stop 2 / symptom entry 2) after the ring measured that per-group exclusive eligibility does NOT fix its own control: the governed ResourceRelease responses sit in the MIXED group that the original text fenced at >= 2, and the causal probe worked by promoting exactly that P1-free mixed group, so one exclusive group now seeds authority for its P1-free mixed dependent on a single-exclusive plane only."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-03; RECUT 2026-09-18 twice. Original filing: scope-call disposition of the Q2 finding the Architect routed to the Steward (Architect evt_66q0njbd8qjf1, runtime thread thr_13yeftxjnxz2z). While building R3 for RT-COMPOSED-RETURN-FORWARD-RET-EDGE (b2), the runtime-implementer found the parity oracle still fails all five composed-return fixtures on RESOURCE RELEASE ORDER: interp releases resource1 then resource2, native releases 2 then 1. A decisive STRUCTURAL check (runtime-implementer evt_5merj95jgakap; Architect CONCUR evt_66q0njbd8qjf1) EXONERATES R3: the captured-environment carrier is built as worker.captures in POSITION order (emit_checked_ih_captured_environment aggregates.rs:3848-3865, field ordinal N = capture N) and R3 projects emit_carrier_field(carrier, ordinal) at that same ordinal (the landed Direct route's convention, core.rs:7893), so R3 threads the file/buffer handles in PLANNER order and is NOT a capture-ordinal permutation. The divergence is downstream in bracket teardown, outcome-independent, pre-existing (these five fixtures previously base-trapped before reaching teardown, so it was invisible), and orthogonal to the composed-return object. FIRST RECUT (Architect evt_71r7rzjzepckc) split the node into SIBLING and NESTED shapes. SECOND RECUT WITHDRAWS THAT SPLIT: the Steward opened the fixtures and there is no sibling shape -- rt_parity_native.rs:184 is withResource wrapping withBuffer and :336 is withResource/withResource/withBuffer three deep, each combinator takes exactly one acquisition and one body (38-ffi-io.md:411-413), and the repo's own helper names say it (rt_inner_bracket_result, rt_file_bracket_result). 'A two-resource bracket' was loose prose in the 2026-09-03 filing meaning a bracket NEST holding two resources; the Architect read it as one bracket holding two and the Steward wrote the first recut on that reading. Architect withdrew the split at evt_1byx3327ppasg. Coordinates re-measure at the build SHA; b2 branch WIP was 430350cff at the finding."
---

> # BOTH MEASUREMENT ROUNDS CLOSED; ARCHITECT HAS RULED THE CAUSE.
> #
> # **`D1a` is AUTHORIZED and ONLY as the two-arm envelope in Deliverables.**
> # Architect ruling `evt_4t14zmba83hjm`, grounded at `89d2bfb57`.
> #
> # **Final measured population** (`D0c` `evt_29sjeg0q437k8`, `D0c-2`
> # `evt_pz0w7e8ae39w`): SIX reaching depth-2 native nests, ALL WRONG. TWO
> # reaching depth-3 composed nests, BOTH CORRECT. Interp correct on all FOUR
> # MEASURED composed programs -- **no MEASURED interp defect**, no interp
> # repair authorized, and `D1b` stays deleted. **THREE of the five existing
> # composed-return fixtures are measured; TWO remain UNMEASURED** and would be
> # a finding if they disagree. There IS an engine contrast at measured depth
> # 2 -- that contrast is the defect; what is absent is a second repair
> # obligation.
> #
> # **Dead leads:** mode, inner-kind, outer-kind, homogeneity, combinator,
> # read-vs-write. The superseded claim -- that interp violates on the
> # composed-return nests where native is CORRECT, and that native-right versus
> # native-wrong was the lead -- is false.
> #
> # **THE CAUSE IS PLANNER CLASSIFICATION, NOT DEPTH.** Depth changes which
> # existing admission class the continuation enters; the causal boundary is
> # Specialized/handler-owned execution versus unowned Deferred forward-`Ret`.
> # px8ta depth 2 is a SECOND lowering path, not another sample of the first.
> # **Both arms are required and this is measured:** the Architect's probe
> # relaxing the global threshold fixed the composed family and left px8ta
> # wrong.
> #
> # `D0a` AFFIRMED `evt_17kyxq7q5v8ar`, AC-7 discharged. `D0c` and `D0c-2` are
> # CLOSED -- do not re-run either. Historical prediction text below is retained
> # under explicit historical labels only.
> #
> # **ARM A CORRECTED 2026-09-21 -- HARD STOP 2 / SYMPTOM ENTRY 2**
> # (`evt_76nkdg0h81xnw`). The ring implemented Arm A literally and measured
> # that it does not fix its own control: the governed `ResourceRelease`
> # responses sit in the **mixed** group that the original text fenced at
> # `>= 2`, so per-group exclusive eligibility never reached them, and Arm B
> # cannot bridge that family because its bounded frontier there is empty. The
> # Architect's own probe worked by promoting that P1-free mixed group. **The
> # sentence "the mixed-owner law is preserved exactly" is FALSE and is
> # removed.** Arm A is now a bounded authority-seed predicate. Arm B is
> # unchanged and already green. No Research trigger until stop or entry 3.

## There is ONE shape. The surface cannot express the other one.

**Ken has no construct that acquires two resources into one scope.**
`withResource` and `withBuffer` each take exactly one acquisition and one body
(`spec/30-surface/38-ffi-io.md:411-413`:
`(body : BufferHandle -> HostIO a (ResourceBodyResult e r))`). So there is no
release *walk* over a scope's resource list, and nothing for a "sibling order"
to be a property of.

**Every measured case is a NEST of single-resource brackets.** Measured at
`origin/main badc039dad6dbecc5165ec0a02054b0984e9b2e4`:

    crates/ken-cli/tests/rt_parity_native.rs

    :184  withResource cap "source" ResourceRead  rt_read_offset_file      OUTER
    :177    withBuffer 1                          (rt_read_offset_body …)  INNER

    :336  withResource cap "source" ResourceRead  (rt_write_pair_source …) OUTER
    :327    withResource cap "sink" WriteCreate   (rt_write_pair_sink …)   MIDDLE
    :319      withBuffer 1                        (rt_write_pair_buffer …) INNER

    The fixtures name it themselves: :178 and :320 continue
    `(\outcome. rt_inner_bracket_result outcome)`; :329 continues
    `rt_file_bracket_result`. The file bracket's body IS the buffer bracket.

> ### HOW THE SPLIT GOT IN, RECORDED BECAUSE THE MECHANISM IS REUSABLE
>
> The 2026-09-03 filing said **"a two-resource bracket (file + buffer)"**,
> meaning a bracket *nest* holding two resources. The Architect read it as one
> bracket holding two and built a sibling/nested split on it
> (`evt_71r7rzjzepckc`); the Steward wrote the first recut on that reading.
> **Neither opened the fixture.** Withdrawn at `evt_1byx3327ppasg`.
>
> ⇒ **A premise is what you reason FROM, so it is not what gets checked.** The
> Steward verified the Architect's spec citations closely enough to refute one
> of them, in the same pass that took his structural claim as given. **A
> structure described in prose is the thing nobody opens.**

## The order is FORCED. It is not a convention and not a scope call.

**No explicit release-order clause exists anywhere in `spec/`.** Re-measured
independently: `lifo`, `LIFO`, `reverse-acquisition`, `reverse acquisition`,
`release order`, `teardown order` all return **zero files**, against a working
instrument — `resource` matches 14 spec files, `bracket` matches 7.

**It does not need one:**

1. Settlement **follows the body's returned value or error** —
   `ADR 0021:177-178`, in
   `docs/adr/0021-resource-lifetime-and-ward-delegation.md`, verbatim: *"The
   `body` is a delayed function so that acquisition precedes it and settlement
   follows its returned value or error."*
2. That ADR is binding on the runtime through
   `spec/60-security/62-authority.md:325-326`, which requires the settlement
   observations to hold *"under ADR 0021's resource identity and settlement
   discipline."*
3. The handle is valid for the body's duration and dies at settlement —
   `38-ffi-io.md:400`, `:405-406`, `:724-727`.
4. Ward's exported property (3), `ADR 0021:152-153`: *"a bracket return,
   returned error, or controlled trap leaves no live resource acquired by that
   bracket."*

**The inner bracket is an expression inside the outer bracket's body**, so its
completion is an event within the outer's returned value or error.

⇒ **Inner settles strictly before outer, by the definition of what a bracket
is.** Releasing the outer first means the outer settled before its own body had
finished — **a violated bracket, not a violated convention.**

> **`38-ffi-io.md:400-406` does NOT carry the settle-timing step.** It fixes
> handle validity, not settlement order. The timing is `ADR 0021:177-178` via
> `62-authority.md:325-326`. The first recut cited `:400-406` for it; caught in
> self-review before routing, and it matters because this reading was offered to
> the Spec enclave as refutable — sent there on `:400-406`, Spec finds no
> ordering rule and the reading looks unsupported.
>
> **That question went to Spec and is ANSWERED** (`evt_17kyxq7q5v8ar`):
> `62-authority.md:325-326` incorporating ADR 0021 by reference DOES discharge
> it. No ordering clause need be written into `spec/`, no spec edit is
> authorized, and `AC-7` is discharged. Do not re-ask.

## NATIVE violates it at EVERY measured depth 2. Interp violates it nowhere MEASURED.

Measured at `89d2bfb57` by `D0c` (`evt_29sjeg0q437k8`):

    depth 2, px8ta   FsHandle/FsHandle, Metadata   native [r1,r2]   WRONG
    depth 2, D0c(C)  FsHandle/FsHandle, Read       native [r1,r2]   WRONG
    depth 2, D0c(A)  Buffer/Buffer                 native [r1,r2]   WRONG
    depth 2, D0c(B)  Buffer outer / FsHandle inner native [r1,r2]   WRONG
    depth 2, composed read                         native [r1,r2]   WRONG
                     FsHandle outer / Buffer inner interp [r2,r1]   correct
    depth 2, D0c-2(E) rt_read_norights_stage       native [r1,r2]   WRONG
                     WriteCreate outer / Buffer    interp [r2,r1]   correct
    depth 3, composed write                        native [r3,r2,r1] correct
                     FsHandle/FsHandle/Buffer      interp [r3,r2,r1] correct
    depth 3, D0c-2(D) composed READ                native [r3,r2,r1] correct
                     FsHandle/FsHandle/Buffer      interp [r3,r2,r1] correct

**There is no native-right-versus-native-wrong lead.** Native is wrong on all
SIX reaching depth-2 nests regardless of kind, combinator, mode, homogeneity or
read/write, and correct on BOTH reaching depth-3 nests. Interp is correct on all
FOUR measured composed programs.

**There IS an engine contrast, and the table above shows it:** at measured depth
2 native releases `[r1,r2]` and interp `[r2,r1]`. That contrast is the defect
this node repairs. What died is the *lead* -- the idea that some surface
property separates a native-correct family from a native-wrong one.

**The mechanism is NOT "emits in acquisition order."** That would make depth 3
release `[r1,r2,r3]`; it releases `[r3,r2,r1]`. Depth 3 is doing something
structurally different, not the same thing at greater length.

**`D0b` (a Steward scope call on "sibling" order) is GONE** -- there is no
unspecified shape.

**The five-fixture accounting, binding everywhere on this node: THREE of the
five existing composed-return fixtures are MEASURED, TWO remain UNMEASURED.**

    MEASURED  fs_read_at_malformed_offset_narrows_to_invalid_offset      D0c
    MEASURED  fs_write_at_malformed_offset_narrows_to_invalid_offset     D0c
    MEASURED  fs_read_at_malformed_offset_without_read_right_            D0c-2(E)
                narrows_to_invalid_offset  (drives rt_read_norights_stage)
    UNMEASURED fs_read_at_malformed_window_narrows_to_invalid_bounds
    UNMEASURED fs_write_at_malformed_offset_without_write_right_
                narrows_to_invalid_offset

**`D0c-2`(D) is a FOURTH measurement, not a sixth fixture** -- a transient READ
variant of the depth-3 write program, built for the round and removed. Four
measurement PROGRAMS; three measured FIXTURES of five.

**The interp DEFECT CLAIM is retired on measurement, and only within that
bound:** no MEASURED interp defect, interp correct on all four measured
programs, no interp repair authorized. The two unmeasured fixtures are unknown
rather than clear, and would be a finding if they disagree.

## The correlate table is HISTORICAL. The inner-kind structural kill is LIVE.

> **HISTORICAL — pre-`D0c` prediction, REFUTED 2026-09-21. It is not the
> current state and nothing in it is an instruction.** Retained only because
> the dead native-right-versus-native-wrong contrast rested on it.
>
> It read six properties as co-varying perfectly across a pair in which the
> composed-return family is native-CORRECT and px8ta native-WRONG, so that
> "inner bracket kind" was one reading of two data points rather than their
> content (Architect, `evt_1byx3327ppasg`):
>
>                         composed-return      px8ta
>                         native CORRECT       native WRONG
>     inner combinator    withBuffer           withResource
>     inner kind          Buffer               FsHandle
>     inner error type    ResourceError        FileError
>     inner acquisition   capacity : Int       (name, mode)
>     outer mode          Read / WriteCreate   ResourceMetadata
>     NEST HOMOGENEITY    heterogeneous        HOMOGENEOUS
>
> **At `89d2bfb57` BOTH families are WRONG at depth 2.** The contrasted pair
> does not exist, so no axis in the table survived, and depth was neither
> controlled nor out. The instruction this section carried -- write the lead
> as a hypothesis to kill, and do not localize a site first -- fired, was
> honoured, and is spent: the Architect localized the cause structurally at
> `evt_4t14zmba83hjm`.

### LIVE: one confound is dead structurally, and it is the one first proposed

**Resource kind cannot be carried by the release emission.** `ResourceRelease`
is a single kind-agnostic op. Under
`crates/ken-runtime/src/cranelift_backend/`:

    planning/static_transition/aggregates.rs:3616
      Op::ResourceRelease => (RESOURCE_SURFACE, UNIT),

one arm covering both kinds, in a table beside `FsReadAt` and
`MappingWriteView` — and the spec is explicit that a buffer settles *"exactly
as for file resources"* (`38-ffi-io.md:344-345`).

⇒ **If ordering were keyed on resource kind it could not be keyed where the
release is emitted.** It would have to sit in continuation placement. **So a
frame that offers "inner bracket kind" as a lead sends the repair to the
release path, which is the one place it provably is not.**

> **Note the path.** That is `planning/static_transition/aggregates.rs:3616`.
> There is a second `aggregates.rs` in this tree at
> `cranelift_backend/lowering/`, and **its line 3616 is unrelated code**
> (`reconcile_declared_children`). A basename plus a line resolves cleanly to
> the wrong file and the reader gets no signal.

### px8ta is homogeneous at every level — verified, not assumed

`px8ta_oriented_subcontinuation.rs` `:44`, `:55`, `:78`, `:169`, `:179`, `:186`:
every level is `withResource` with mode `ResourceMetadata` on `held-N.bin`.
**Homogeneous in combinator, kind AND mode**, where the composed-return nests
are heterogeneous in all three. That makes homogeneity and inner-kind
**separable rather than rival stories** — they predict opposite results on a
cheap fixture.

## What is established (do not re-derive)

- The divergence is NOT in the composed-return repair (R3) — handles threaded
  in planner order, no permutation. Structural check decisive; Architect
  concurs.
- It is pre-existing, exposed only once the five composed-return fixtures ran
  past the base `ResourceBodyResult` trap into teardown.
- `public_two_three_level_brackets_finish_and_release_lifo`'s helper
  `assert_depth_finishes_and_releases_lifo` asserts `releases ==
  opens.reverse()` over `depth` nested scopes, and the row is `#[ignore]`d.

> ## THIS NODE IS A BLOCKER ON A COUNTED ROW
>
> `public_two_three_level_brackets_finish_and_release_lifo` is one of the
> **fifteen originally selected** ignored rows
> (`RT-IGNORED-FAILING-ROWS-INVENTORY`), the operator's top-priority
> population. **The cohort was selected as fifteen; the population now stands
> at fourteen** — row 15 closed at `9c3a5f588` — and this is ledger **row 9**.
> **`D1a` is what moves it.**
>
> **It does not clear the row.** That row carries a SECOND blocker at depth 3 —
> an object-emission refusal owned by
> `RT-DEPTH3-CONTINUATION-CLAIM-UNDECLARED` — invisible until now because depth
> 2 panics first and a loop body reports nothing past its first failure. **Both
> are owed; neither subsumes the other.**

## Deliverables

D0a. **DISCHARGED.** Spec enclave AFFIRMED at `evt_17kyxq7q5v8ar`:
     `62-authority.md` §4.2 makes settlement normative under ADR 0021, whose
     delayed-body clause puts an inner bracket's settlement inside completion
     of the inner EXPRESSION. Inner settlement precedes outer-body completion,
     derived from locked text. No spec edit authorized or needed. Do not
     re-ask.

D0c. **RAN AND IS CLOSED** (`evt_29sjeg0q437k8`). Mode, inner-kind, outer-kind
     and homogeneity are each dead with a WRONG case on both sides; combinator
     dies with kind because the two never vary independently. **Do not re-run
     it.** Its value is the four dead axes, not a direction.

D0c-2. **RAN AND IS CLOSED** (`evt_pz0w7e8ae39w`; predictions pre-registered
     at `evt_29wbxhm9mcvs9`). **Read-vs-write is DEAD on both sides.** (E)
     `rt_read_norights_stage` reaches teardown and is a valid fixture: native
     `[r1,r2]` wrong, interp `[r2,r1]` correct, killing write-create vs
     read/metadata inside depth 2. (D) a depth-3 READ: native and interp both
     `[r3,r2,r1]` correct, killing read/write inside depth 3. **Live set is now
     depth-equals-count versus lowering path, and no fixture separates those
     either** -- if depth selects the lowering they co-vary by construction.
     **The standing stop FIRED and has since been ANSWERED.** Depth was left
     standing, the structural question went to the Architect, and the ruling
     landed at `evt_4t14zmba83hjm`: depth is the SELECTOR, the cause is planner
     classification. `D1a` is now AUTHORIZED, bounded to the two arms below.
     Interp is correct on all FOUR composed measurements across both rounds;
     no interp-repair evidence emerged, which further confirms `D1b`'s deletion.

     Historical, the split it was authorized to make:
     Depth-equals-count, read-vs-write and lowering path are confounded in the
     single composed three-deep write. **Depth and resource count are ONE
     variable** -- every bracket takes exactly one acquisition
     (`spec/30-surface/38-ffi-io.md:408-413`), so nest depth N IS N resources
     and no fixture separates them. Predictions registered BEFORE each
     run. Cheapest first:

       (D) a depth-3 READ nest. Holds family and depth against the
           known-correct case, varies only read/write.
             correct -> read/write dead; depth still stands
             wrong   -> depth dead; the operation axis is causal
       (E) a depth-2 nest with a `ResourceWriteCreate` outer, varying
           read/write inside the known-wrong case.
             wrong   -> read/write dead from the other side
             correct -> the operation axis is causal
           `rt_read_norights_stage` (`rt_parity_native.rs:220-222`) already
           nests one at depth 2 and MAY serve, but it reads as a
           negative/refusal stage and may never reach teardown. Verify that
           before relying on it, and report it if it does not.

D1a. **Repair the planner classification, as TWO REQUIRED ARMS** (Architect
     `evt_4t14zmba83hjm`). Either arm alone leaves half the population wrong.

     **A. A BOUNDED AUTHORITY-SEED PREDICATE** (corrected
     `evt_76nkdg0h81xnw`). `StaticTransitionPlan::static_response_phase_b_split`
     -- stop using the whole-plane `ordinary_stage_count >= 2` threshold to
     reject a single-exclusive plane, and let one exclusive group SEED
     authority for its P1-free mixed DEPENDENT:

         composed_plane_authority       = ordinary_stage_count >= 2
         single_exclusive_plane         = ordinary_stage_count == 1
         single_exclusive_group_authority =
             single_exclusive_plane
             && (exclusively_predeclared_stage
                 || (!has_unitless_response && mixed_owner_stage))
         group_requires_execute_then_resume =
             composed_plane_authority || single_exclusive_group_authority

     Outer conditions unchanged: checked-IH transport source; suppression
     restores P2; in a P1-BEARING plane every non-exclusive group stays
     Deferred unless the already-existing explicit overpromotion mutation
     applies on its already-existing route. `>= 2` behavior is otherwise
     byte-for-behavior. **This is NOT the forbidden global `>= 1` change:** the
     single-exclusive case admits ONLY the one `(true,false)` exclusive seed
     and a `(true,true)` mixed dependent on a P1-free plane. It admits no
     `(false,true)` specialization-only group, no group without a predeclared
     source, no plane with zero exclusive groups, and no mixed group in a
     P1-bearing plane. **`writeAll` stays the negative control** -- P1
     main-lowered, mixed `ResourceRelease` group P2, deliberate overpromotion
     still reaching the owner-escape refusal.

     **B.** `StaticTransitionPlan::bounded_deferred_response_suffix` -- admit a
     third structural class `release_only_suffix` (non-empty, every row
     `HostOpV1::ResourceRelease`), returned when `repeated_producer ||
     mapping_access_chain || release_only_suffix`. Not emission-time
     operation-name special casing: the frontier walk has already proved exact
     lexical K, finite population, no opaque frontier, no cycle and P2 shape,
     and the handler-owner check still requires one unique specialized owner.

     **Do not reorder release events after emission.** No host-dispatch
     reorder, trace sort, `ResourceRelease` kind split, exploratory logging,
     global diagnostic recoding, aggregate relaxation, or capacity change.

D2a. **TWO non-ignored controls, one per arm.** `D2a-A` for the composed
     single-stage arm (the reaching `rt_read_norights_stage` shape or a
     source-equivalent); `D2a-B`, distinct, for the px8ta release-only-suffix
     arm -- **not the still-ignored depth-2-plus-depth-3 loop row.** Mutation
     must redden each independently with `[r1,r2]`, each with a positive
     application witness that compiles. **One aggregate release-order test is
     not evidence for both arms.**

D2.  Re-enable release-order parity on the five composed-return fixtures
     (remove the b2 exclusion marker) once both engines agree. **THREE of the
     five are measured; TWO remain UNMEASURED** -- see the accounting above for
     which. Measured does NOT mean agreeing: at depth 2 the engines disagree,
     and that disagreement is what `D1a` repairs. Re-enable only once they
     agree, and treat a disagreement from either unmeasured fixture as a
     finding to return to the Steward.

## Acceptance

- **Every acceptance criterion names its NEST.** "Release order" without saying
  which nest is the ambiguity this node was recut twice to remove.
- Report the observed release SEQUENCE as vectors, before and after. **The
  assertion compares two vectors; the vectors are the evidence** — a pass/fail
  is not.
- **BOTH reaching depth-3 nests release `[r3, r2, r1]` at `89d2bfb57` and MUST
  still do so after any repair** -- the composed three-deep WRITE and the
  `D0c-2`(D) three-deep READ. They are the only measured native-correct
  teardowns, so a repair that breaks either has traded violations for a new
  one. **AC-6's exact depth-3 object-emission refusal on the ignored px8ta row
  is preserved unchanged.** (This replaces the old clause naming the
  composed-return nest as native's working case; that clause was refuted by its
  own re-measurement -- the composed TWO-deep read is native-wrong.)
- **DISCHARGED.** Both rounds registered predictions before their runs -- `D0c`
  at `evt_287tm2f6ycg0x`, `D0c-2` at `evt_29wbxhm9mcvs9` -- and both are closed.
  **No further discriminating fixture is authorized**, so this governs nothing
  live.
- **TWO independent controls, one per `D1a` arm, and each mutation-proved.**
  **Arm A's mutation must model removal of the AUTHORITY SEED** -- disabling the
  whole `single_exclusive_group_authority` arm for BOTH the exclusive seed and
  its P1-free mixed dependent -- not merely deferring the exclusive row after
  the dependent has borrowed. It must restore `D2a-A` to `[r1,r2]` and leave the
  ordinary `>= 2` path unchanged; rename or reshape the control if its name
  implies the narrower exclusive-only mutation. Suppressing release-only suffix
  admission must redden `D2a-B` with `[r1,r2]`. Each mutation needs a positive
  application witness and must compile. **One aggregate release-order test is
  not evidence for both arms.**
- **Structural diagnostics, not pinned ids.** The composed arm must no longer
  leave its governed releases as unowned Deferred P2 -- and because those
  releases sit in the MIXED dependent group, **both the exclusive seed and its
  P1-free mixed dependent must gain execute-then-resume ownership**, not the
  seed alone. The px8ta inner release must acquire the unique bounded handler
  owner.
- The px8ta row is **not** un-ignored by this node alone — its depth-3 blocker
  is `RT-DEPTH3-CONTINUATION-CLAIM-UNDECLARED`'s.
- **The native repair** is grounded in the settlement derivation affirmed at
  `evt_17kyxq7q5v8ar`, **not in matching whichever engine was easier to
  change.** There is exactly ONE repair, and it is native-side: interp measures
  correct on all FOUR measured composed programs, so no interp repair is
  authorized and none is to be filed.
