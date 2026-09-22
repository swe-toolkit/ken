# RT-BRACKET-RELEASE-ORDER-PARITY — work package

- **Node:** `[[RT-BRACKET-RELEASE-ORDER-PARITY]]`
- **Owner:** runtime
- **Size:** M
- **Tier:** T1 (section 7)
- **Depends on:** nothing. The former conditional dependency on
  `fa4fc3647b09b9480940f7d5d35c50ffb7978aeb` is **DISCHARGED** — see section 8.
- **Branch:** `wp/RT-BRACKET-RELEASE-ORDER-PARITY-nested-teardown`

> ## RECUT 2026-09-21 — `D0c` + `D0c-2` RAN; ARCHITECT HAS RULED THE CAUSE.
>
> **`D1a` IS AUTHORIZED, AND ONLY AS THE TWO-ARM ENVELOPE IN §4.** Architect
> ruling `evt_4t14zmba83hjm`. **Depth is the SELECTOR, not the cause.** The
> causal boundary is Specialized or handler-owned execution versus unowned
> Deferred forward-`Ret`: a statically bounded bracket-settlement continuation
> is left on the unowned Deferred route even though it must execute before the
> enclosing bracket resumes. **Fix that predicate; do not reorder release
> events after emission.**
>
> **Measured population, final** (`D0c` `evt_29sjeg0q437k8`, `D0c-2`
> `evt_pz0w7e8ae39w`): **SIX reaching depth-2 native nests, all WRONG** — px8ta
> baseline, (C), (A), (B), composed two-deep read, and `rt_read_norights_stage`
> — across both kinds in both positions, both combinators, homogeneous and
> heterogeneous, metadata/read/write-create. **TWO reaching depth-3 composed
> nests, both CORRECT**, across read and write. **Interp is correct on all four
> composed measurements.**
>
> **Dead: mode, inner-kind, outer-kind, homogeneity, combinator, read-vs-write.**
> `§2.5`'s "Depth is controlled and is out" is refuted; `§2.5`'s structural kill
> of "inner bracket kind" SURVIVES and is corroborated — `ResourceRelease` is
> kind-agnostic, so ordering was never keyed at the release site. It is keyed in
> planner classification, exactly where `§2.5` said it would have to be.
>
> **`D0a` AFFIRMED** (`evt_17kyxq7q5v8ar`), AC-7 discharged. **`D0c` and `D0c-2`
> are CLOSED; do not re-run either.** Two arms are required: A alone fixes the
> composed family and leaves px8ta wrong; B alone fixes px8ta and leaves the
> composed family wrong.
>
> **ARM A CORRECTED 2026-09-21, hard stop 2 / symptom entry 2**
> (`evt_76nkdg0h81xnw`). The ring implemented Arm A literally and measured that
> it does NOT fix its own control: on the composed depth-2 plane the two
> `ResourceRelease` responses live in the **mixed** group, which the original
> text fenced at `>= 2`, so per-group exclusive eligibility never reached them.
> The Architect's own causal probe worked by promoting exactly that P1-free
> mixed group. **The superseded sentence "mixed-owner groups keep their current
> `>= 2`" is FALSE and has been removed.** Arm A is now a bounded
> authority-seed predicate (`§4`). Arm B is unchanged and is already green.
>
> **HARD STOP 3 RULED 2026-09-21 — `evt_6c900nagc7mn4`, adopting Research
> advisory `evt_5p5jn87bmwpcc`. THE px8ta DEPTH-3 REFUSAL IS LAWFULLY
> ELIMINATED, NOT PRESERVED.** Arm A does not make the old function-local
> lookup succeed — it cannot; the identity key is never added to
> `function_local.continuation_calls`. It replaces the ordinary route that
> formed the failing claim with an exact selected-caller → response-owner →
> typed-K-context path whose own checks stay live.
> **`RT-DEPTH3-CONTINUATION-CLAIM-UNDECLARED` is SUBSUMED**, closes atomically
> with this product, and gets no separate implementation. **This node now
> CLEARS ledger row 9.** Neither arm is to be narrowed to preserve the refusal,
> and `core.rs`/`units.rs` stay outside production scope. The entry-3
> shared-predicate answer was **YES** across all three stops — an incidental
> planner-classification outcome repeatedly read as a semantic boundary — so
> **there will be no fourth local carve-out.**
>
> **HARD STOP 4 RULED 2026-09-21 — `evt_24nwhvgacvy28`. THE CLOSEOUT IS
> CORRECT; ARM A CONFUSES BINDING AUTHORITY WITH CALL AUTHORITY.** On WIP
> `aa1cfc6a1`, `fs_read_at_malformed_window_narrows_to_invalid_bounds` refuses
> before execution with `a forward-declared response owner has no verified
> selected incoming call`, while the same fixture passes on `c28bc8305`. Its
> owner is selected from an exact ordinary checked-IH transport identity whose
> diagnostic records `disposition=None`; that establishes the planner MAY
> install the candidate's environment, **not** that this artifact emits its
> call. At close the unsettled transport candidate becomes `TransportDormant`
> and the response-owner closeout correctly refuses. **Production reached the
> same guard `RestoreSelectedKTarget` targets, but by a different cause: no
> selected call was formed at all.** This does NOT reopen the stop-3 ruling —
> the px8ta path is live and lawfully replaced. It narrows Arm A's REACH: group
> membership is necessary for the bounded authority seed and is **not** positive
> evidence that each selected caller is live. **`D0` in §4 runs BEFORE any
> implementation, and if no planning-available fact separates the sibling pair,
> the work STOPS THERE and returns to the Steward.**

## 1. Objective

On a nest of bracket scopes the inner bracket must settle before the outer.
**Native violates this on all SIX reaching depth-2 nests measured, regardless
of kind, combinator, mode, homogeneity or read/write, and satisfies it on BOTH
reaching depth-3 nests.** The separator is ruled (`evt_4t14zmba83hjm`) and is
planner classification, not depth. Repair native inside the §4 envelope.

**`D0a` is AFFIRMED** (Spec enclave `evt_17kyxq7q5v8ar`): `62-authority.md` §4.2
makes settlement normative under ADR 0021, whose delayed-body clause puts an
inner bracket's settlement inside completion of the inner EXPRESSION. Inner
settlement precedes outer-body completion, derived from locked text; no spec
edit is authorized or needed. AC-7 is discharged and is not re-asked.

**This does not turn the px8ta row green** and no acceptance criterion says it
does — see section 6.

## 2. Fixed inputs, measured

**Measured at `origin/main badc039dad6dbecc5165ec0a02054b0984e9b2e4`.**
Coordinates are perishable; re-measure at your build SHA.

### 2.1 There is ONE shape: a nest of single-resource brackets

**Ken's surface cannot acquire two resources into one scope.** `withResource`
and `withBuffer` each take exactly one acquisition and one body
(`spec/30-surface/38-ffi-io.md:411-413`). So there is no release walk over a
scope's resource list — every case is a nest.

    crates/ken-cli/tests/rt_parity_native.rs

    :184  withResource cap "source" ResourceRead  rt_read_offset_file      OUTER
    :177    withBuffer 1                          (rt_read_offset_body …)  INNER

    :336  withResource cap "source" ResourceRead  (rt_write_pair_source …) OUTER
    :327    withResource cap "sink" WriteCreate   (rt_write_pair_sink …)   MIDDLE
    :319      withBuffer 1                        (rt_write_pair_buffer …) INNER

The fixtures say so themselves: `:178` and `:320` continue
`(\outcome. rt_inner_bracket_result outcome)`, `:329` continues
`rt_file_bracket_result`.

> **An earlier cut of this node split the work into "sibling" and "nested"
> shapes. There is no sibling shape.** It came from reading "a two-resource
> bracket" in the 2026-09-03 filing as one bracket holding two resources.
> Withdrawn. **If you find a document still asserting that split, it predates
> `evt_1byx3327ppasg` and is wrong.**

### 2.2 The order is forced by locked text, not by convention

**No explicit release-order clause exists in `spec/`** — `lifo`, `LIFO`,
`reverse-acquisition`, `reverse acquisition`, `release order`, `teardown order`
all return **zero files**, against a working instrument (`resource` matches 14
spec files, `bracket` matches 7).

    ADR 0021:177-178    "The `body` is a delayed function so that acquisition
                         precedes it and settlement follows its returned value
                         or error."   (docs/adr/0021-resource-lifetime-and-
                                       ward-delegation.md)
    62-authority.md
      :325-326          binds the runtime to "ADR 0021's resource identity and
                         settlement discipline"
    38-ffi-io.md:400    the bracket "passes the handle to the bracket body"
      :405-406          "every copy ... becomes invalid when the bracket settles"
      :724-727          "Lifetime is bracket-scoped; a use after settle ...
                         yields the single `Revoked` identity"
    ADR 0021:152-153    Ward property (3): a bracket return, returned error, or
                         controlled trap leaves no live resource acquired by
                         that bracket

The inner bracket is an expression **inside the outer bracket's body**, so its
completion is an event within the outer's returned value or error.

⇒ **Releasing the outer first means the outer settled before its own body
finished. That is a violated bracket, not a violated convention.**

> **`38-ffi-io.md:400-406` does NOT carry the settle-timing step** — it fixes
> handle validity, not settlement order. Cite `ADR 0021:177-178` via
> `62-authority.md:325-326`.

### 2.3 What each engine does, RE-MEASURED at `89d2bfb57`

    depth 2, px8ta   FsHandle/FsHandle, Metadata   native [r1,r2]    WRONG
    depth 2, D0c(C)  FsHandle/FsHandle, Read       native [r1,r2]    WRONG
    depth 2, D0c(A)  Buffer/Buffer                 native [r1,r2]    WRONG
    depth 2, D0c(B)  Buffer outer / FsHandle inner native [r1,r2]    WRONG
    depth 2, composed read                         native [r1,r2]    WRONG
                     FsHandle outer / Buffer inner interp [r2,r1]    correct
    depth 2, D0c-2(E) rt_read_norights_stage       native [r1,r2]    WRONG
                     WriteCreate outer / Buffer    interp [r2,r1]    correct
    depth 3, composed write                        native [r3,r2,r1] correct
                     FsHandle/FsHandle/Buffer      interp [r3,r2,r1] correct
    depth 3, D0c-2(D) composed READ                native [r3,r2,r1] correct
                     FsHandle/FsHandle/Buffer      interp [r3,r2,r1] correct

**Native is wrong on all SIX reaching depth-2 nests and right on BOTH reaching
depth-3 nests. Interp is correct on all FOUR measured composed programs.** The
old reading here -- native right on one nest and wrong on the other, with interp
carried unre-run from 2026-09-03 -- was refuted the moment it was re-measured.

**The five-fixture accounting, stated once and binding everywhere in this
frame: THREE of the five existing composed-return fixtures are MEASURED, TWO
remain UNMEASURED.**

    MEASURED  fs_read_at_malformed_offset_narrows_to_invalid_offset      D0c
    MEASURED  fs_write_at_malformed_offset_narrows_to_invalid_offset     D0c
    MEASURED  fs_read_at_malformed_offset_without_read_right_            D0c-2(E)
                narrows_to_invalid_offset  (drives rt_read_norights_stage)
    UNMEASURED fs_read_at_malformed_window_narrows_to_invalid_bounds
    UNMEASURED fs_write_at_malformed_offset_without_write_right_
                narrows_to_invalid_offset

**`D0c-2`(D) is a FOURTH measurement, not a sixth fixture.** It is a transient
READ variant of the depth-3 write program, built for the round and removed; it
is a required depth-3 control, not a member of the five-fixture set. So: four
measurement PROGRAMS across both rounds, three measured FIXTURES of five. The
two unmeasured fixtures are unknown, not clear.

### 2.4 The row under repair

    crates/ken-cli/tests/px8ta_oriented_subcontinuation.rs
      fn public_two_three_level_brackets_finish_and_release_lifo
      body: for depth in 2..=3
      helper: assert_depth_finishes_and_releases_lifo(depth)
        opens    = effect_trace filtered to HostOpV1::FsOpen
        releases = effect_trace filtered to HostOpV1::ResourceRelease
        asserts  releases == opens.reverse()  and  len == depth

    depth 2   EXECUTES, releases in ACQUISITION order  -> this WP
    depth 3   REFUSES AT OBJECT EMISSION               -> RT-DEPTH3-...

Depth-2's behaviour is the runtime-implementer's D0 measurement under
`RT-SUBCONTINUATION-LIFO-RELEASE-ORDER` (`evt_707t1acbaxp62`), taken under
`catch_unwind` — order-independent, reproduced with depth 3 first, separate temp
dir and separate `build_native_program` per depth.

### 2.5 The confound table is HISTORICAL. The inner-kind structural kill is LIVE.

> **HISTORICAL — pre-`D0c` prediction, REFUTED 2026-09-21. It is not the
> current state and nothing below it is an instruction.** Retained only
> because the dead native-right-versus-native-wrong contrast rested on it.
>
> It asserted six properties co-varying across a pair in which the
> composed-return family is native-CORRECT and px8ta native-WRONG at depth 2:
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
> **At `89d2bfb57` BOTH families are WRONG at depth 2** (`§2.3`). The
> contrasted pair does not exist, so no axis in the table survived, and depth
> was neither controlled nor out. The header of this table also carried
> "DO NOT PICK A SITE FIRST"; that stop fired, was answered by
> `evt_4t14zmba83hjm`, and is spent.

**LIVE, and corroborated by the ruling: "inner bracket kind" is dead where you
would look for it.** `ResourceRelease`
is a single kind-agnostic op. Under
`crates/ken-runtime/src/cranelift_backend/`:

    planning/static_transition/aggregates.rs:3616
      Op::ResourceRelease => (RESOURCE_SURFACE, UNIT),

one arm covering both kinds, beside `FsReadAt` and `MappingWriteView` — and a
buffer settles *"exactly as for file resources"* (`38-ffi-io.md:344-345`).

⇒ **If ordering were keyed on resource kind, it could not be keyed where the
release is emitted.** It would have to sit in continuation placement. **Taking
"inner bracket kind" as a lead sends you to the release path, which is the one
place it provably is not.**

> **Watch the path.** That is
> `planning/static_transition/aggregates.rs`. A second `aggregates.rs` exists at
> `cranelift_backend/lowering/`, and **its `:3616` is unrelated code.** A
> basename plus a line resolves cleanly to the wrong file with no signal to the
> reader.

**px8ta is homogeneous at every level** — `:44`, `:55`, `:78`, `:169`, `:179`,
`:186` are all `withResource` with mode `ResourceMetadata` on `held-N.bin`.
Homogeneous in combinator, kind AND mode, where the composed-return nests are
heterogeneous in all three. **That made homogeneity and inner-kind separable
rather than rival** — they predicted opposite results on `D0c`(A). **`D0c`(A)
ran: the answer was WRONG, which killed BOTH.** The separation held; neither
hypothesis survived it.

**The repair site IS localized, by ruling.** It was deliberately withheld
while `D0c-2` was open, because naming one then would have been naming a
correlate. `D0c-2` closed leaving no single surface axis, and the Architect
localized the cause structurally rather than by correlation
(`evt_4t14zmba83hjm`): the two planner classification arms in `§4`. **Do not
re-derive a site from the surface table above; build A and B.**

## 3. THE DESIGN JUDGMENT, RULED

**`D1a` is AUTHORIZED and BOUNDED. Build arms A and B together (§4).**

Every gate is discharged: `D0a` AFFIRMED (`evt_17kyxq7q5v8ar`), `D0c` closed
(`evt_29sjeg0q437k8`), `D0c-2` closed (`evt_pz0w7e8ae39w`). Nothing waits on a
Spec round or a further fixture. **Do not re-run `D0c` or `D0c-2`.**

**Depth is the selector, not the cause.** Architect ruling `evt_4t14zmba83hjm`,
grounded at `89d2bfb57`: depth changes which existing planner admission class
the continuation happens to enter. The causal boundary is **Specialized or
handler-owned execution versus unowned Deferred forward-`Ret`.** In the composed
depth-2 fixture only `BufferAllocate` is a `StaticResponseContinuation`;
`FsReadAt` and both `ResourceRelease` responses are Deferred P2
`UnconsumedTransportCaller` with `handler_owner: None`, so they stay on the
forward-`Ret` route and the enclosing bracket resumes while the inner settlement
is still only a returned value. At depth 3 all three `ResourceRelease` responses
are Specialized, the Deferred population is empty, the owner calls the exact K
once and returns before its caller resumes -- lexical bracket order, LIFO.
**px8ta depth 2 is a SECOND lowering path, not another sample of the first:**
`FsOpen` is Specialized, but `bounded_deferred_response_suffix` finds the inner
`ResourceRelease` and rejects it because `ResourceRelease` is removed from
`substantive` and both admitted classes require a non-empty substantive set.

**BOTH ARMS ARE REQUIRED, and this is measured, not asserted.** The Architect's
probes: relaxing `ordinary_stage_count >= 2` to `>= 1` flipped the composed
depth-2 fixture to `[r2,r1]` while depth-3 stayed correct -- **and did NOT fix
px8ta**, which remained `[r1,r2]`. That disproves a one-line global-threshold
repair as closure over the population. **A patch containing only A fixes the
composed family and leaves px8ta wrong; only B fixes px8ta and leaves the
composed family wrong.**

**WHY that probe worked, established by the ring at `evt_19nce0t4rrtb9` and
ruled at `evt_76nkdg0h81xnw`.** The `>= 1` edit did not help because of anything
the exclusive group did. On the composed depth-2 plane there are exactly two
transport groups -- one exclusive `(predeclared=true, specialization=false)` and
one mixed `(true,true)` -- and `ordinary_stage_count == 1`. **The two governed
`ResourceRelease` responses sit in the MIXED group.** The probe promoted that
P1-free mixed group, and that is what produced `[r2,r1]`. So a repair that
promotes only the exclusive group cannot reach the releases at all, and **Arm B
cannot bridge this family either**: the bounded frontier from each specialized
response is empty, so there is no release-only suffix here to admit. Arm A is
specified in `§4` to reach the dependent group under a bounded seed predicate,
not to relax the threshold globally.

**The standing prohibitions.** Do not reorder release events after emission. No
host-dispatch reorder, trace sort, `ResourceRelease` kind split, exploratory
logging, global diagnostic recoding, aggregate relaxation, or capacity change is
authorized. Fix the classification predicate.

## 4. Deliverables

**D0 — STOP-4 LIVENESS ENUMERATION. RUN THIS BEFORE ANY IMPLEMENTATION, AND IT
MAY END THE WORK.** Architect `evt_24nwhvgacvy28`. *(This is the ruling's own
label. It is a NEW deliverable and is distinct from the discharged `D0a`, `D0c`
and `D0c-2`, none of which it reopens.)*

**Prove the boundary is available where Arm A classifies — do not assume it is.**
On exact WIP `aa1cfc6a1002c969a5a6d4a33993a2a4f2c69f13`, enumerate **every**
demand the single-exclusive Arm-A branch would newly admit, and record for the
SAME exact identity:

- the planning facts available **immediately before**
  `static_response_phase_b_split`;
- whether the shared continuation funnel is **structurally selected**; and
- the eventual candidate disposition and verified response-owner call.

**Report identities, not aggregate counts.** The population is the five §2.3
composed-return fixtures, both `D2a` controls, px8ta depths 2 and 3, and
`writeAll`.

**The discriminating sibling pair is MANDATORY, and what it requires depends on
which branch `D0` lands in.** Architect `evt_394w1nhcss7kh`.

- **Implementation branch.** The admissible planning predicate must predict
  `rt_read_offset_stage` **LIVE** and `rt_read_window_stage` **DORMANT**, with
  offset still executing and window restored to main's passing path.
- **Stop branch.** Report those same identities' **observed eventual
  dispositions and direct-call evidence**, and state that **no admissible
  planning-time fact predicts that split**. **Do not relabel those observations
  as predictions.**

A predicate that cannot separate the two has not been shown to work — but
*establishing* that it cannot is the stop branch's deliverable, not its
failure.

**Admissibility.** A candidate predicate is admissible **only if** it is a
structural planning fact **already authoritative over the emitted route**, and
it agrees **identity-for-identity** with the final direct-call evidence.
`ordinary_continuation_call_identities`, checked-IH transport membership, group
shape, and operation kind are **already disproved as sufficient** — do not
re-propose them.

**`D0` mutation controls, stated over the ROUTE BOUNDARY** so they do not depend
on a production predicate the stop branch may have just proved absent.
Suppressing the read-offset **liveness authorization** must restore the prior
Arm-A failure; forcing the dormant read-window identity through admission must
reproduce the exact missing-selected-caller refusal `a forward-declared response
owner has no verified selected incoming call`. **Both controls require positive
application counts and byte-exact restoration**, and both are owed in either
branch.

> ### THE STOP CONDITION IS PART OF THE DELIVERABLE, NOT A FAILURE OF IT
>
> **If no planning-available fact distinguishes the sibling pair without
> predicting an emission-time result, STOP AFTER `D0` AND HAND BACK.** That is
> the historical phase-ordering obstruction, and it returns to the Steward to
> size separately as a restructure that either hoists call liveness or delays
> owner classification. **Do not approximate it with a proxy, and do not add a
> fourth carve-out.** Reaching that stop is a correct and complete outcome of
> `D0` — it is the measurement doing its job, and the operator's 2026-09-21
> direction expressly admits restructuring as where this can land.

**D0a — DISCHARGED.** The Spec enclave AFFIRMED at `evt_17kyxq7q5v8ar`:
`spec/60-security/62-authority.md:325-326` §4.2 makes settlement normative under
ADR 0021, whose delayed-body clause puts an inner bracket's settlement inside
completion of the inner EXPRESSION. **Inner settlement precedes outer-body
completion**, derived from locked text; no clause need be written into `spec/`
and no spec edit is authorized. The named refutation was rejected. AC-7 is
discharged. **Do not re-ask this.**

**D0c — RAN AND IS CLOSED.** Result at `evt_29sjeg0q437k8`. Mode, inner-kind,
outer-kind and homogeneity are dead; combinator dies with kind. **Do not re-run
it.** Its remaining value is the five dead axes, not a direction.

**D0c-2 — RAN AND IS CLOSED. Read-vs-write is DEAD on both sides.** Result at
`evt_pz0w7e8ae39w`, predictions pre-registered at `evt_29wbxhm9mcvs9`.

    (E) rt_read_norights_stage REACHES teardown -- it is a valid fixture, not
        a pre-teardown refusal. Acquired [FsOpen r1, BufferAllocate r2].
          native [r1,r2] WRONG    interp [r2,r1] correct
        => ResourceWriteCreate vs read/metadata killed INSIDE depth 2.
    (D) depth-3 READ, middle bracket changed from write-create sink to
        ResourceRead source. Acquired [FsOpen r1, FsOpen r2, BufferAllocate r3].
          native [r3,r2,r1] correct   interp [r3,r2,r1] correct
        => read/write killed INSIDE depth 3.

**The live set is now exactly two: depth-equals-count versus lowering path** --
and **no fixture separates those either.** If depth selects a different lowering,
the two co-vary by construction. That is why this is no longer a measurement
question.

**THE STANDING STOP FIRED AND HAS BEEN ANSWERED.** Depth was left standing, so
the WP returned to the Steward and the structural question -- what differs in
depth-3 lowering versus depth-2 lowering -- went to the Architect, who ruled it
at `evt_4t14zmba83hjm`. The ring did not infer the structure or pick a site from
the correlation, which is the whole point of the stop, and it did not have to:
depth is the SELECTOR and the cause is planner classification. **`D1a` is
therefore no longer forbidden — it is AUTHORIZED, bounded to arms A and B
below, and site selection is settled by the ruling rather than open.**

**Note the mechanism is not "emits in acquisition order."** That would make
depth 3 release `[r1, r2, r3]`; it releases `[r3, r2, r1]`. Depth 3 is doing
something structurally different, not the same thing at greater length.

> **HISTORICAL — what `D0c-2` was authorized to split, retained for the record.
> It is closed; nothing from here to the end of this block is an instruction.**

> Depth and resource count are ONE variable, not two. Every bracket takes
> exactly one acquisition (`spec/30-surface/38-ffi-io.md:408-413`), so nest
> depth N IS N resources and no fixture can separate them.
>
> Same discipline as `D0c`: predictions registered before each run. Cheapest
> first:
>
>     (D) a depth-3 READ nest. Holds family and depth against the
>         known-correct case, varies only read/write.
>           correct -> read/write is dead; depth still stands
>           wrong   -> depth is dead; the operation axis is causal
>
>     (E) a depth-2 nest with a ResourceWriteCreate outer, varying read/write
>         inside the known-wrong case.
>           wrong   -> read/write is dead from the other side
>           correct -> the operation axis is causal, and (D) says whether
>                      depth adds anything
>
> `rt_read_norights_stage` was flagged as a possible (E) that might never
> reach teardown. It was verified and it DOES reach teardown.
>
> The block closed with a standing stop: if `D0c-2` left depth standing, stop
> and return the WP, because "why is the depth-3 lowering different" is an
> Architect ruling and not `D1a`'s to assume. **That stop fired, was honoured,
> and was answered — see the operative record above. END OF HISTORICAL BLOCK.**

**D1a — repair the planner classification, as TWO arms. Both required.**

**Arm A — `StaticTransitionPlan::static_response_phase_b_split`. A BOUNDED
AUTHORITY-SEED PREDICATE.** Architect `evt_76nkdg0h81xnw`. Stop using the
whole-plane `ordinary_stage_count >= 2` threshold to reject a single-exclusive
plane, and let one exclusive group seed authority for its P1-free mixed
dependent. The authorized classification is exactly:

```rust
let composed_plane_authority = ordinary_stage_count >= 2;
let single_exclusive_plane = ordinary_stage_count == 1;
let single_exclusive_group_authority = single_exclusive_plane
    && (exclusively_predeclared_stage
        || (!has_unitless_response && mixed_owner_stage));
let group_requires_execute_then_resume =
    composed_plane_authority || single_exclusive_group_authority;
```

**REQUIRED SEMANTIC BOUNDARY, stop 4 (`evt_24nwhvgacvy28`). The predicate above
is NECESSARY AND NOT SUFFICIENT.** For every demand **newly admitted by the
single-exclusive Arm-A branch**, response-owner admission requires **BOTH**:

1. the landed bounded plane/group law above — `ordinary_stage_count == 1`, with
   the `(true,false)` seed and only P1-free `(true,true)` dependents; **and**
2. **a positive planning-time proof that this exact `ContinuationCallIdentity`
   will reach the shared continuation funnel and emit the verified direct call
   retargeted to its response owner IN THIS ARTIFACT.**

**Absence, uncertainty, `InlineNoCall` and `TransportDormant` all mean
Deferred.** The proof is **per selected caller** — not per operation, producer
origin, fixture, count, or whole group. So a live exclusive seed may be
Specialized while a **dormant dependent stays Deferred**; the Arm-A suppression
mutation still removes seed authority from every live seed and dependent
together. Historical `ordinary_stage_count >= 2` behaviour remains
byte-for-behavior and Arm B is unaffected. **Classification must leave the
dormant demand Deferred BEFORE any owner is forward-declared.**

**Keep the existing outer conditions unchanged:** the demand must be a
checked-IH transport source; suppression restores P2; and in a P1-bearing plane
every non-exclusive group stays Deferred unless the already-existing explicit
overpromotion mutation applies on its already-existing route. Existing
`ordinary_stage_count >= 2` behavior is otherwise byte-for-behavior.

**This is NOT the forbidden global `>= 1` change, and the boundary is what makes
it lawful.** The single-exclusive case admits exactly two group shapes: the one
`(true,false)` exclusive **authority seed**, and a `(true,true)` mixed
**dependent** only when the plane is P1-free. It does **not** admit
`(false,true)` specialization-only groups, groups with no predeclared source, a
plane with zero exclusive groups, or a mixed group in a P1-bearing plane.

**`writeAll` remains the negative control:** its P1 stays main-lowered, its
mixed `ResourceRelease` group stays P2, and the existing deliberate
overpromotion still reaches the owner-escape refusal. The composed depth-2
control is the positive case -- one exclusive seed plus its P1-free mixed
dependent, both gaining execute-then-resume ownership.

**Arm B — `StaticTransitionPlan::bounded_deferred_response_suffix`.** Admit a
third structural class:

```rust
let release_only_suffix = !suffix.is_empty()
    && suffix
        .iter()
        .all(|row| row.operation() == HostOpV1::ResourceRelease);
```

Return the bounded suffix when `repeated_producer || mapping_access_chain ||
release_only_suffix`. **This is not operation-name special casing at emission:**
the preceding frontier walk has already proved exact lexical K, finite
population, no opaque frontier, no cycle, and P2 shape, and
`bounded_deferred_response_handler_owner` still requires one unique specialized
owner or refuses. The predicate recognizes the bracket-settlement tail that the
old `substantive` filter made impossible to admit.

**D2a — TWO non-ignored controls, one per arm. One aggregate release-order test
is not evidence for both.**

- **D2a-A:** a depth-2 end-to-end control for the composed single-stage arm,
  using the existing reaching `rt_read_norights_stage` shape or a
  source-equivalent fixture. Assert acquisitions and exact reverse releases.
- **D2a-B:** a distinct depth-2 px8ta control for the release-only
  bounded-suffix arm. **Do not use the px8ta depth-2-plus-depth-3 loop row as
  this arm's control.** That row now clears under `AC-6`, but it exercises both
  arms at once and a loop body reports nothing past its first failure, so it
  cannot separate them.

**D2b — THREE depth-3 causal controls on the UNCHANGED px8ta program.** Green at
depth 3 is not evidence of the replacement route; these separate it from a green
arriving for some other reason. All three run against the px8ta program as it
stands — **no new fixture, and no edit to the program under test.**

- **D2b-1 — the selected-caller edge.** Apply the existing
  `StaticResponseCallerRetargetMutation::RestoreSelectedKTarget`. Require a
  **positive application count**, **typed planner diagnostics unchanged**, and
  the exact refusal `a forward-declared response owner has no verified selected
  incoming call`. It leaves Arm A's classification and identity population
  intact and removes only the selected-caller → response-owner target, so it
  tests that edge and nothing else.
- **D2b-2 — the typed-K-context edge.** Apply the existing
  `StaticResponseOwnerBodyMutation::OmitKCall` to that same depth-3 program.
  Require a **positive application count** and the finished-owner refusal `a
  response owner emitted 0 K calls instead of exactly one`. This proves the
  selected response owner cannot merely perform the effect and omit the exact
  typed K context.
- **D2b-3 — route replacement, retained from the Architect's own probe.**
  Suppressing the single-exclusive authority seed at depth 3 restores the exact
  old `ContinuationSpecialization` undeclared-target refusal. **State
  explicitly in the candidate that this control proves ROUTE REPLACEMENT — that
  the repair removed the path which formed the failing claim. It does NOT make
  that refusal desired behaviour and is NOT a licence to preserve it.** Arm B is
  not applicable on this path (`ARCHITECT_ARM_B_APPLICATIONS=0`) and its own
  independent control is unaffected.

**SCOPE FENCE — `core.rs` and `units.rs` stay OUT of production scope.**
Architect `evt_6c900nagc7mn4`. **No fallback, no lookup widening, no new
declaration lane, and no `Fusion` owner is authorized.** The subsumption is
lawful precisely because the exact `function_local.continuation_calls` lookup,
the affine claim, the response-owner body verifier and the claim-ledger closeout
are all byte-unchanged; a candidate that edits them has substituted a different
mechanism for the ruled one and is a hard stop, not a refinement. **Arm B, both
depth-2 controls and their mutations, `writeAll` as negative control, both
composed depth-3 `[r3,r2,r1]` vectors, and every existing fail-closed closeout
remain REQUIRED.**

**DO NOT WEAKEN OR MOVE THE DETECTOR (stop 4, `evt_24nwhvgacvy28`).**
`validate_response_owner_call_coverage`, `core.rs`, `units.rs`, the exact
lookup and claim, owner-body verification, and claim/discharge equality all
remain **unchanged**. Specifically prohibited: accepting an uncalled forward
declaration; treating `TransportDormant` as owner coverage; adding a late
fallback; keying on origins or on `FsReadAt`; and carving out the read-window
fixture. **The closeout that produced stop 4 is CORRECT and is not the thing to
repair** — it refused rather than emitting something wrong, which is the
behaviour the fail-closed design exists to produce.

## 4a. Symptom inventory

SYMPTOM INVENTORY (append one line per hard-stop; never rewrite history)

1. Every measured two-resource depth-2 native nest releases in acquisition
   order across both kinds, both combinators, both modes, and homogeneous and
   heterogeneous arrangements, while the sole then-reaching depth-3 composed
   write nest released LIFO — keyed on a carried correct-versus-wrong contrast
   that treated depth as controlled although depth, resource count, read/write,
   and lowering path co-varied in its only correct observation.
2. The first Arm A repair promoted the single exclusively-predeclared group but
   left the P1-free mixed release group Deferred, so the composed depth-2
   control remained `[r1,r2]`; the successful threshold probe had promoted that
   mixed group too — keyed on treating group-local exclusive ownership as the
   whole settlement authority when the required closure is a P1-free plane
   seeded by exactly one exclusive group.
3. The complete two-arm repair makes the px8ta depth-3 row pass strict LIFO and
   removes its exact `ContinuationSpecialization` object-emission refusal,
   although AC-6 requires that refusal to remain — keyed on preserving a prior
   planner-classification failure itself instead of distinguishing a genuinely
   function-local claim declaration from a bypassed declaration guard.
4. Arm A forward-declares a response owner for the read-window fixture, but its
   exact selected caller reaches no lowering settlement and would become
   `TransportDormant`; owner coverage refuses before execution while the same
   fixture passes on current main — keyed on treating checked-IH transport and
   group membership as proof of a causal call although the candidate
   representation separates binding authority from call obligation.
5. The two-pass orchestrator builds a complete eligibility-probe plan before it
   knows whether `A` is empty, so all six empty-`A` mapping cases run two planner
   lifecycles against current main's one and regress as either a final-planning
   refusal or a bad native artifact — keyed on deciding whether discovery is
   needed only after an otherwise disposable full plan has already run, contrary
   to the one-pass boundary and without isolating final semantics from the
   preflight or its distinct mode.
6. The late-selecting Initial implementation measures the complete Mapping
   frontier inside the real planner and refutes entry 5's empty-`A` premise:
   ordinary Phase B derives nonempty `A` and completes `[Discovery, Final]` for
   all four two-operation programs, while both alternating three-operation
   programs refuse at `pair_detached_required_consumer` before Initial can close
   — keyed on inferring that current-main one-pass execution placed a program
   outside structural `A` without measuring `A`, the same substitution of
   structural eligibility for the later causal-call/lifecycle population shared
   by entries 4–6.

**Count of record:** hard stops 6; symptom entries 6. Entry 1 was classified at
`evt_177exqsxhvg5y`; entry 2 at `evt_76nkdg0h81xnw`; entry 3 at
`evt_45qpftkx3s894`; entry 4 was reported at `evt_59tt59x2mbbxg`; entry 5 was
reported at `evt_78xgqja72hktm`; entry 6 was reported at
`evt_47yqzemfp9zv3`. **Both stop-6 triggers are FIRED and the ruling is held** —
Research was requested at `evt_522yfgcveskqz`, where the entry-6 shared-predicate
question was answered: entries 4–6 all substitute unmeasured structural `A` for
later causal-call/lifecycle population. Research is not yet discharged. Both
next fire at nine after the present triggers are discharged.

## 5. Acceptance criteria

**AC-0. This WP adds no `#[ignore]` attribute anywhere.** The ignored-row count
is the number this work exists to move; a deliverable that seems to need a new
one is a hard stop to report, not a cost to absorb.

**AC-1. BOTH `D2a` controls exist, are not ignored, and pass.** Give the path
and symbol name of each. **`D2a-A` (composed single-stage arm) and `D2a-B`
(px8ta release-only-suffix arm) are separate tests** -- one aggregate
release-order test does not satisfy this.

**AC-2. Report the observed release SEQUENCE before and after, as vectors —
not pass/fail.** The assertion compares two vectors and the vectors are the
evidence:

    before   opens [r0, r1]   releases [r0, r1]    acquisition order
    after    opens [r0, r1]   releases [r1, r0]    inner-before-outer

**AC-3. MUTATION-PROVE THE TWO ARMS INDEPENDENTLY.** A control that passes on
both trees tests nothing, and a control that reddens under either mutation does
not separate the arms.

- **Arm A's mutation must model removal of the AUTHORITY SEED, not merely
  deferral of the exclusive row after its dependent has already borrowed.** It
  disables the whole `single_exclusive_group_authority` arm, for **both** the
  exclusive seed and its P1-free mixed dependent, and must restore `D2a-A` to
  `[r1,r2]`. Rename or reshape the Arm A control if its current name implies
  the narrower exclusive-only mutation. **The ordinary `>= 2` path must be left
  unchanged by the mutation.**
- **Suppressing release-only suffix admission must redden `D2a-B` with
  `[r1,r2]`.** Arm B's independent suppression and control are unchanged.

**Each mutation needs a positive application witness and must compile.** State
which branch of the repaired code each control exercises.

**AC-4. DISCHARGED, AND IT REFUTED ITS OWN PREMISE.** This criterion required
re-measuring native's claimed working case BEFORE the repair, on the reasoning
that "if the native half no longer holds there is no contrast left to explain."
It ran and the half does not hold: the composed two-deep READ is native-WRONG
(`[r1, r2]`) and interp-CORRECT (`[r2, r1]`). **The criterion did its job by
failing.** Do not re-ask it.

**What replaces it as the no-regression clause:** **BOTH reaching depth-3 nests
release `[r3, r2, r1]` at `89d2bfb57` and MUST still do so after any repair** --
the composed three-deep WRITE and the `D0c-2`(D) three-deep READ. They are the
only measured native-correct teardowns, so a repair that breaks either has
traded violations for a new one. **The px8ta depth-3 refusal is NOT among the
things preserved:** `AC-6` now requires it to be gone. Preserving it was a
criterion of the pre-ruling recut, and `evt_6c900nagc7mn4` withdrew it.

**AC-5. DISCHARGED.** Both measurement rounds registered predictions before
their runs -- `D0c` at `evt_287tm2f6ycg0x`, `D0c-2` at `evt_29sjeg0q437k8`'s
predecessor `evt_29wbxhm9mcvs9`. Both rounds are closed. **No further
discriminating fixture is authorized**, so this criterion governs nothing live
and is not re-asked.

**AC-6. `px8ta public_two_three_level_brackets_finish_and_release_lifo` PASSES
at BOTH depth 2 and depth 3, with strict LIFO at each, and is no longer
`#[ignore]`d.** Report the release vector at each depth. **There must be no
`ContinuationSpecialization` object-emission refusal at depth 3, and no refusal
of any other kind.** Architect `evt_6c900nagc7mn4`: Arm A lawfully eliminates
the route that formed the failing claim rather than satisfying the old lookup.
**A surviving depth-3 failure is a hard stop to report, not a cost to absorb** —
and it would refute the ruling's mechanism, so it returns to the Architect
rather than being worked around here.

**AC-7. DISCHARGED.** `D0a` is answered AFFIRMED, enclave event id
`evt_17kyxq7q5v8ar`, recorded on the node. Nothing further is owed here and the
question is not re-asked.

**AC-8a. USE THE EXISTING DIAGNOSTICS STRUCTURALLY.** The composed arm must no
longer leave its governed releases as unowned Deferred P2 -- and since those
releases sit in the **mixed dependent** group, the obligation is that **both the
exclusive seed AND its P1-free mixed dependent gain execute-then-resume
ownership**, not the seed alone. The px8ta inner release must acquire the unique
bounded handler owner. **Do not pin absolute origin ids.** Preserve both
reaching depth-3 vectors `[r3,r2,r1]`.

**AC-8. No-regression in CI**, per `COORDINATION §12` — green in CI, never a
local `--workspace` run. Local work is `scripts/ken-cargo -p ken-runtime` and
the named test, nothing wider.

**AC-9. CLOSE OBLIGATION, CLEARED-ROW BRANCH. This node does not reach `merged`
until the px8ta row runs green and its second blocker is closed with it.** Three
things land in the SAME candidate, or none of them do:

- `px8ta_oriented_subcontinuation.rs`'s
  `public_two_three_level_brackets_finish_and_release_lifo` carries **no
  `#[ignore]` attribute**, and the stale reason string naming
  `RT-SUBCONTINUATION-LIFO-RELEASE-ORDER` is **deleted, not edited** — once the
  row runs there is no owner to name and no successor to point at.
- `docs/program/issues/RT-DEPTH3-CONTINUATION-CLAIM-UNDECLARED.md` moves to
  `status: closed`, citing `evt_6c900nagc7mn4` and naming this node as the
  mechanism that subsumed it. **It gets no separate implementation**, and no
  part of its repair may be attempted here.
- The closeout states **in words** that the `(H)` probe recorded on that node is
  NOT claimed fixed. That node shares a refusal SIGNATURE between `(H)` and
  px8ta depth 3 and expressly never established a shared ROUTE. If `(H)` is
  ever re-measured and still refuses, that is a finding for the Steward, not a
  regression of this WP.

*(`AC-0` prohibits ADDING an `#[ignore]`; removing one is what this node exists
to do. The candidate touches `crates/` and is therefore **`full` CI, never
doc-only**.)*

> **Amended 2026-09-21 by Architect ruling `evt_6c900nagc7mn4`, which removed
> this AC's only available branch and replaced it.** The 2026-09-19 version had
> no cleared-row branch at all: `AC-6` then required the depth-3 refusal to
> SURVIVE, so the row necessarily survived too, and the only live question was
> whether it survived pointing at a live owner rather than at a `merged` node.
> **Both halves of that reasoning are void.** The refusal is lawfully
> eliminated and the row clears, so the live-owner and no-live-owner branches
> are **deleted, not retained as fallbacks**. A candidate that leaves the row
> `#[ignore]`d — pointing at anything, live or terminal — fails this criterion.
>
> **Ledger row 9 clears outright when this lands.** The row was two blockers
> under one `#[ignore]`: this node repairs the first and lawfully subsumes the
> second, so there is no successor owner to name and nothing is handed back to
> `main` still ignored.

**AC-10. `D0` IS EVIDENCE, AND IT IS REPORTED AS IDENTITIES.** The candidate —
or the hand-back, if `D0` reaches its stop condition — gives the enumeration §4
specifies: for every demand the single-exclusive Arm-A branch newly admits, the
planning facts available immediately before `static_response_phase_b_split`,
whether the shared funnel is structurally selected, and the eventual disposition
and verified response-owner call. **Identities, not aggregate counts.**

**The sibling pair is reported explicitly, in the form its branch allows.** In
the **implementation** branch: offset **predicted** live and still executing,
window **predicted** dormant and restored to main's passing path. In the
**stop** branch: the same two identities' **observed** eventual dispositions and
direct-call evidence, plus the statement that no admissible planning-time fact
predicts that split. **A stop-branch hand-back must NOT dress observations up as
predictions, and this criterion does not ask it to** — requiring predictions
from a branch that exists because no predictor was found is a contradiction, and
it was one this AC carried until `evt_394w1nhcss7kh`.

Both `D0` mutation controls carry **positive application counts** in either
branch, the forced-window control reproduces the exact refusal `a
forward-declared response owner has no verified selected incoming call`, and the
probe code is stated to have been restored byte-exactly.

**AC-10a. A PREDICATE THAT CANNOT SEPARATE THE SIBLINGS IS NOT REPORTED AS ONE
THAT CAN.** If the admissible predicate does not exist — no structural planning
fact already authoritative over the emitted route that agrees
identity-for-identity with the final direct-call evidence — **say so and stop**.
Naming the obstruction IS the deliverable in that branch. A proxy that happens
to separate these two fixtures, or any predicate that works by predicting an
emission-time result, **fails this criterion even if every test is green.**

**AC-11. NO COMPOSED-RETURN FIXTURE REGRESSES, AND EACH IS NAMED.** All five
`§2.3` composed-return fixtures pass on the final candidate —
`fs_read_at_malformed_window_narrows_to_invalid_bounds` **included**, since it
passes on `c28bc8305` and a repair that reddens it has traded a violation for a
new one. Report each fixture's result individually. **A green aggregate does not
satisfy this**, and neither does excluding, ignoring, or carving out the
read-window fixture.

## 6. What this WP is NOT

- **It DOES clear the px8ta row — that changed under a ruling, and the earlier
  text saying otherwise is removed rather than qualified.** The row's second
  depth-3 blocker, owned by `RT-DEPTH3-CONTINUATION-CLAIM-UNDECLARED`, is
  lawfully SUBSUMED by Arm A rather than separately repaired (Architect
  `evt_6c900nagc7mn4`). **This WP still does not IMPLEMENT that node**, and
  nothing here licenses touching `core.rs` or `units.rs`: the subsumption is a
  consequence of the planner-classification repair, not a second deliverable.
  The superseded sentences — that un-ignoring the row "buys a red row for no
  information" and that an AC requiring it to pass would be unsatisfiable —
  were written when the refusal was expected to survive. They are FALSE now.
- **No interp repair is authorized, and `D1b` is DELETED, not deferred.** Its
  premise -- that interp violates on the composed-return nests -- measures
  FALSE at `89d2bfb57`. **State this in the bounded form and no wider: there is
  no MEASURED interp defect. Interp is correct on all FOUR measured composed
  programs. No interp repair is authorized.** No `D1b` node exists and none is
  to be filed on the refuted premise. If a measurement in this WP shows interp
  violating anywhere, return it to the Steward.

  **THE UNMEASURED-RESIDUE CLAUSE FIRED, AND THAT IS WHAT PRODUCED STOP 4.**
  `§2.3`'s two unmeasured fixtures were run on WIP `aa1cfc6a1` and returned to
  the Steward exactly as this clause required; ruled at `evt_24nwhvgacvy28`.

  **A PASS IS NOT A RELEASE-ORDER MEASUREMENT, AND READING IT AS ONE COST A
  CORRECTION** (`evt_1p5c8y14aj3r3`, adopted `evt_3t2enjn08xft1`). At
  `89d2bfb57` the oracle `assert_narrowed_alike` (`rt_parity_native.rs:632`)
  compares bracket releases **as a SET with relative ORDER excluded** — that is
  the b2 exclusion `D2` exists to remove — so a fixture passing through it has
  had its release order observed **not at all**. State the two results with that
  distinction attached:

      fs_write_at_malformed_offset_without_write_right_  GENUINE 4th post-repair
        ordered cross-engine observation. The WIP deletes BOTH order-excluding
        helpers, so `assert_narrowed_alike` compares `effect_trace` directly and
        relative ResourceRelease order is inside the equality. Confirmed at
        `evt_7553qsjww5ve6`. It is not a concrete-vector assertion of its own.
      fs_read_at_malformed_window_narrows_to_invalid_bounds  NO release-order
        observation on EITHER tree. WIP refuses before execution; the named
        baseline passed only through the order-excluding oracle.

  ⇒ **The fixture that broke is precisely the one no release-order reading has
  ever covered.** **`§2.3`'s table is a PRE-REPAIR baseline at `89d2bfb57`,
  stays 3 MEASURED / 2 UNMEASURED, and is unchanged by either run** — a result
  measured on a repaired tree is not a reading of the tree that table describes.
  Post-repair observations live in `§4`'s `D0` record and `§5`, never in `§2.3`.
- **This is not a claim that the engines agree.** At measured depth 2 they
  plainly do not: native `[r1,r2]`, interp `[r2,r1]`. The engine contrast is
  real and is the defect. What is absent is a native-right-versus-native-wrong
  LEAD and any SECOND repair obligation on present evidence.
- **It does not author an ordering clause in `spec/`, and `D0a` settled that no
  clause is needed.** The enclave AFFIRMED the derivation from locked text at
  `evt_17kyxq7q5v8ar` and authorized no spec edit. Should anything downstream
  reopen it, that is still the enclave's to write and the Steward's to
  sequence -- but it is not an open question here.
- **It is not a measurement of the other fourteen ignored rows.** The "a
  looping test reports nothing past its first failure" finding is real, is
  recorded on the node, and is not this WP's work.

## 7. Estimated tier: T1

Semantic repair in the native backend's planner classification, on a property
whose correctness derives from locked text AFFIRMED by the enclave at
`evt_17kyxq7q5v8ar`. **The measurement load is now spent and the reasoning load
has moved to the repair.** `D0c` and `D0c-2` killed mode, inner-kind,
outer-kind, homogeneity, combinator and read-vs-write across six reaching
depth-2 nests and two reaching depth-3 nests; the Architect has ruled the cause
at `evt_4t14zmba83hjm`. **What remains T1 is holding TWO classification arms
apart** -- a bounded authority-seed predicate that reaches the P1-free mixed
dependent without becoming a global threshold relaxation,
and a third bounded-suffix class that does not become operation-name special
casing -- **and proving each independently by mutation.** The Architect's own
probe showed a one-line global threshold change fixes one family and not the
other. The diff may be small; keeping the two laws separate is not.

## 8. Contention

**The px8ta dependency is DISCHARGED — verified, not assumed.**
`RT-SUBCONTINUATION-LIFO-RELEASE-ORDER` is `merged`; its `D1` landed on `main`
as `4eb3dc4c6` and its node closure as `d959980ae`. At this WP's base
`89d2bfb57` the blob for `crates/ken-cli/tests/px8ta_oriented_subcontinuation.rs`
is `248cbd21dbb4bd5a8b9b590bfafb7f2ee70f9ed9`, **byte-identical to the blob at
`fa4fc3647b09b9480940f7d5d35c50ffb7978aeb`**. So the rewrite that branch SHA
carried is already in your base. **`D2a-B` necessarily touches that file and is
free to.** Do not treat `fa4fc3647` as a future prerequisite; it is not an
unmerged base and there is nothing to wait for.

**This WP is the ACTIVE L1 work package.** `steward/lanes.md` names it as such,
and its only release blocker is this recut landing on `main`. Any earlier text
sequencing it behind `RT-CARRIER-PRODUCER-OCCURRENCE` is superseded: that node
is not ahead of this one.

> **HISTORICAL — the queue snapshot below was measured against
> `badc039da` and is not the live contention plan. It is retained as the
> record of why the px8ta dependency was once conditional.** The queue it
> describes has since drained; re-measure at your own base if you need a
> current picture.
>
> - `fa4fc3647…` (`RT-SUBCONTINUATION-LIFO-RELEASE-ORDER` `D1`, then routed and
>   ahead of this WP) rewrote `px8ta_oriented_subcontinuation.rs` under hunk
>   `@@ -261,28 +261,69 @@`. `D0c`(C) already ran against that file and `D0c`
>   is closed, so the contention was spent for `D0c`.
> - Nothing else in that queue touched `ken-runtime/src`. `af2270b7b` was
>   `docs/` plus a playbook; `9342062315`, `b042af474`, `fabcd98ed`,
>   `89f1cc71b`, `45f66746b` were `docs/` only; `d0058baf1` was
>   `ken-elaborator`.
