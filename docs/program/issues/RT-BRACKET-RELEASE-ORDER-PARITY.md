---
id: RT-BRACKET-RELEASE-ORDER-PARITY
title: "Bracket teardown order. There is ONE shape, not two: Ken's surface cannot express two resources in one scope, so every measured case is a NEST of single-resource brackets, and inner-before-outer is FORCED by bracket semantics (settlement follows the body's returned value or error; the inner bracket's completion is an event in the outer's body). Both engines violate it, on different nests: interp releases outer-then-inner on the composed-return fixtures, where native is CORRECT; native releases outer-then-inner on px8ta row `public_two_three_level_brackets_finish_and_release_lifo`, one of the fifteen selected ignored rows. Native being right on one nest and wrong on another is the lead, and six properties co-vary across that pair."
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

> # OPERATIVE (Steward, 2026-09-03; RECUT TWICE, 2026-09-18)
> #
> # SCOPE RULING (unchanged, 2026-09-03): filed as a DISTINCT runtime node, NOT
> # folded into RT-COMPOSED-RETURN-FORWARD-RET-EDGE (b2). b2 landed its
> # composed-return repair on OUTCOME parity; release-order parity is EXCLUDED
> # for those five fixtures and tracked HERE. R3 is exonerated by the structural
> # check (handles threaded in planner order, no permutation — see `origin`).
> #
> # SEQUENCING: b2 is `closed` (main `5668d363d`, 2026-09-05), so this node's
> # release precondition has been met since that date and the node is `ready`.
> # Architect is the required design reviewer; runtime QA gate; Steward M1-M3a
> # -> lieutenant. Coordinates re-measure at the release SHA.

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
> **Open question for Spec, stated and not answered here:** does
> `62-authority.md:325-326` incorporating ADR 0021 by reference discharge this,
> or must the ordering clause be written into `spec/`?

## Both engines violate it, on DIFFERENT nests

    composed-return   OUTER withResource file, INNER withBuffer
                      interp  file then buffer = outer-then-inner   VIOLATES
                      native  buffer then file = inner-then-outer   CORRECT

    px8ta row         withResource nested in withResource
                      native  outer-then-inner                      VIOLATES

**Native satisfies the rule on one nest and breaks it on another.** That, not a
shape distinction, is the lead.

**`D0b` (a Steward scope call on "sibling" order) is GONE.** There is no
unspecified shape, so **interp on the composed-return fixtures is a correctness
defect with a direction, not a disposition.**

## The lead is a CORRELATE. Write it as a hypothesis to kill.

**Six properties co-vary perfectly across the pair**, so "inner bracket kind"
is one reading of two data points, not their content (Architect,
`evt_1byx3327ppasg`):

                        composed-return      px8ta
                        native CORRECT       native WRONG
    inner combinator    withBuffer           withResource
    inner kind          Buffer               FsHandle
    inner error type    ResourceError        FileError
    inner acquisition   capacity : Int       (name, mode)
    outer mode          Read / WriteCreate   ResourceMetadata
    NEST HOMOGENEITY    heterogeneous        HOMOGENEOUS

**Depth is controlled and is out.** Both families cover 2-deep and 3-deep, and
at depth 2 one is right and the other wrong.

### One confound is already dead, and it is the one first proposed

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
> **fifteen selected** ignored rows (`RT-IGNORED-FAILING-ROWS-INVENTORY`), the
> operator's top-priority population. **`D1a` is what moves it.**
>
> **It does not clear the row.** That row carries a SECOND blocker at depth 3 —
> an object-emission refusal owned by
> `RT-DEPTH3-CONTINUATION-CLAIM-UNDECLARED` — invisible until now because depth
> 2 panics first and a loop body reports nothing past its first failure. **Both
> are owed; neither subsumes the other.**

## Deliverables

D0a. **Confirm the settlement derivation with the Spec enclave**, including the
     ADR-by-reference question. A confirmation with a named refutation, not a
     fresh investigation. **Named refutation: Spec reads settlement as not
     ordered relative to body completion.**

D0c. **Kill the confounds with discriminating fixtures, cheapest first.** These
     buy a DIRECTION for the repair; they are not measurements of the
     population.

       (C) one-token edit to a fixture already run: change px8ta's inner
           bracket mode from `ResourceMetadata` to `ResourceRead`.
           homogeneous combinator, HETEROGENEOUS mode.
             still wrong -> mode is not it; combinator or kind
             now correct -> never "bracket kind"; it is nest homogeneity
       (A) `withBuffer` inside `withBuffer`.
             inner-kind predicts CORRECT, homogeneity predicts WRONG.
             One bit, separates the two leading hypotheses outright.
       (B) `withResource` inside `withBuffer` — inverts the pair while holding
           heterogeneity. Inner-kind predicts WRONG, outer-kind predicts
           CORRECT. `withBuffer`'s body is an ordinary `HostIO` computation, so
           this is expressible today.

D1a. **Repair native's nested teardown** so the inner bracket settles before
     the outer. Report the site and the argument for it, not only the diff.

D1b. **Repair interp on the composed-return nests** — it releases
     outer-then-inner there, which the derivation forbids. This has a direction
     now and is no longer gated on a Steward decision.

D2.  Re-enable release-order parity on the five composed-return fixtures
     (remove the b2 exclusion marker) once both engines agree.

## Acceptance

- **Every acceptance criterion names its NEST.** "Release order" without saying
  which nest is the ambiguity this node was recut twice to remove.
- Report the observed release SEQUENCE as vectors, before and after. **The
  assertion compares two vectors; the vectors are the evidence** — a pass/fail
  is not.
- **The composed-return nest is native's own working case and must stay
  correct.** A repair for px8ta that breaks it has traded one violation for
  another, and this is the control with teeth: it is a case where native is
  already right.
- `D0c` reports each fixture's prediction BEFORE its run, and which hypotheses
  each result kills. A discriminating fixture whose prediction is recorded
  afterwards discriminates nothing.
- The px8ta row is **not** un-ignored by this node alone — its depth-3 blocker
  is `RT-DEPTH3-CONTINUATION-CLAIM-UNDECLARED`'s.
- Both repairs are grounded in the settlement derivation, **not in matching
  whichever engine was easier to change.**
