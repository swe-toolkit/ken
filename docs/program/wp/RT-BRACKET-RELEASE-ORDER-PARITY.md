# RT-BRACKET-RELEASE-ORDER-PARITY — work package

- **Node:** `[[RT-BRACKET-RELEASE-ORDER-PARITY]]`
- **Owner:** runtime
- **Size:** M
- **Tier:** T1 (section 7)
- **Depends on:** `fa4fc3647b09b9480940f7d5d35c50ffb7978aeb` **if and only if
  you touch `px8ta_oriented_subcontinuation.rs`** — see section 8.
- **Branch:** `wp/RT-BRACKET-RELEASE-ORDER-PARITY-nested-teardown`

> ## THE REPAIR DIRECTION IS KNOWN. THE MECHANISM IS NOT, AND SIX THINGS CO-VARY.
>
> Inner-before-outer is forced by bracket semantics, so there is no open
> question about *what* correct looks like. **What is open is why native gets it
> right on one nest and wrong on another**, and the most attractive answer
> — "the inner bracket's kind" — **is provably not keyed where the release is
> emitted** (section 2.5). Kill the confounds before you pick a site.

## 1. Objective

On a nest of bracket scopes the inner bracket must settle before the outer.
Native violates this on `withResource`-in-`withResource` and satisfies it on
`withResource`-wrapping-`withBuffer`. **Establish which property separates
those two cases, then repair native.**

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

### 2.3 The two nests, and what each engine does

    composed-return   OUTER withResource file, INNER withBuffer
                      interp  file then buffer = outer-then-inner   VIOLATES
                      native  buffer then file = inner-then-outer   CORRECT

    px8ta row         withResource nested in withResource
                      native  outer-then-inner                      VIOLATES

**Native is right on one nest and wrong on the other.** Interp's behaviour is
carried from the 2026-09-03 finding and **has not been re-run** — re-measure it
before relying on it.

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

### 2.5 SIX CONFOUNDS. ONE IS ALREADY DEAD. DO NOT PICK A SITE FIRST.

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

**"Inner bracket kind" is dead where you would look for it.** `ResourceRelease`
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
heterogeneous in all three. **That makes homogeneity and inner-kind separable
rather than rival** — they predict opposite results on `D0c`(A).

**The repair site is NOT localized, deliberately.** Naming one before `D0c`
would be naming a correlate. That is `D1a`'s first act, informed by `D0c`.

## 3. THE DESIGN JUDGMENT, FRONT-LOADED

**Build `D0c` and `D1a` now. Do not wait on the Spec round.**

`D0a` confirms a derivation from locked text; it is not an open design
question, and the direction inverts only if Spec reads settlement as unordered
relative to body completion — which would be refuting what a bracket is.
**Parking a repair on an absent ruling whose expected value is "yes" is how a
lane stalls.**

**If `D0a` comes back refuted, stop and return the WP to the Steward.** Do not
re-aim the repair yourself.

**`D0c` before `D1a`, and this is the ordering that matters.** Six properties
co-vary; three cheap fixtures collapse them. A repair chosen before that is a
repair chosen on a correlate, and it will look right on the fixture it was
chosen from.

## 4. Deliverables

**D0a — confirm the settlement derivation with the Spec enclave**, including:
**does `62-authority.md:325-326` incorporating ADR 0021 by reference discharge
the ordering requirement, or must the clause be written into `spec/`?** Runs
concurrently. **Named refutation: Spec reads settlement as not ordered relative
to body completion.**

**D0c — kill the confounds, cheapest first.** These buy a direction for the
repair; they are not measurements of the ignored-row population.

    (C) one-token edit to a fixture you already run: px8ta's inner bracket
        mode ResourceMetadata -> ResourceRead. Homogeneous combinator,
        HETEROGENEOUS mode.
          still wrong -> mode is not it; the split is combinator or kind
          now correct -> it was never "bracket kind"; it is nest homogeneity
    (A) withBuffer inside withBuffer.
          inner-kind predicts CORRECT, homogeneity predicts WRONG.
          One bit; separates the two leading hypotheses outright.
    (B) withResource inside withBuffer — inverts the pair while holding
        heterogeneity. inner-kind predicts WRONG, outer-kind predicts CORRECT.
        withBuffer's body is an ordinary HostIO computation, so this is
        expressible today.

**D1a — repair native's nested teardown** so the inner bracket settles before
the outer, at the site `D0c` points to. Report the site and the argument for
it, not only the diff.

**D2a — a nested release-order control that is NOT `#[ignore]`d.** It must
execute, observe the effect trace, and assert `releases == opens.reverse()`.
Depth 2 is sufficient; **do not extend it to depth 3** (section 6).

## 5. Acceptance criteria

**AC-0. This WP adds no `#[ignore]` attribute anywhere.** The ignored-row count
is the number this work exists to move; a deliverable that seems to need a new
one is a hard stop to report, not a cost to absorb.

**AC-1. `D2a` exists, is not ignored, and passes.** Give its path and symbol
name.

**AC-2. Report the observed release SEQUENCE before and after, as vectors —
not pass/fail.** The assertion compares two vectors and the vectors are the
evidence:

    before   opens [r0, r1]   releases [r0, r1]    acquisition order
    after    opens [r0, r1]   releases [r1, r0]    inner-before-outer

**AC-3. Control: the fix must be capable of failing.** Show `D2a` goes red
against the pre-fix binary, and state which branch of the repaired code it
exercises. A control that passes on both trees tests nothing.

**AC-4. THE COMPOSED-RETURN NEST IS NATIVE'S OWN WORKING CASE AND MUST STAY
CORRECT.** Report its release order before and after. **A repair for px8ta that
breaks it has traded one violation for another** — and unlike most
no-regression clauses this one has teeth, because it is a case where native is
already right and the repair is aimed squarely at it.

**AC-5. Each `D0c` fixture records its PREDICTION BEFORE its run**, and which
hypotheses the result kills. **A discriminating fixture whose prediction is
written down afterwards discriminates nothing** — it explains.

**AC-6. `px8ta public_two_three_level_brackets_finish_and_release_lifo` remains
`#[ignore]`d, and its depth-3 behaviour is unchanged.** Run it under
`catch_unwind` and report what depth 3 does. **Expected: the same
`ContinuationSpecialization` object-emission refusal.** A *different* depth-3
failure is a finding — report it, do not absorb it.

**AC-7. `D0a` has an answer recorded, affirmed or refuted**, with the enclave
event id. An unanswered `D0a` does not close this WP.

**AC-8. No-regression in CI**, per `COORDINATION §12` — green in CI, never a
local `--workspace` run. Local work is `scripts/ken-cargo -p ken-runtime` and
the named test, nothing wider.

**AC-9. CLOSE OBLIGATION. This node does not reach `merged` until
`px8ta_oriented_subcontinuation.rs:326` carries a LIVE owner at the START of its
`#[ignore]` string.** *(Control: the row satisfies one of two branches at the
candidate's base. **(b) LIVE OWNER** — the first node token in that reason
string resolves to a node whose status is `ready`, `active` or `draft`; `merged`
and `closed` fail, and present-and-terminal fails exactly as absent does.
**(c) NO LIVE OWNER** — the string opens by stating in words that no live node
owns the next step and names this node as having established that, which leaves
a reader a record to go to rather than a dead end. **Branch (a) CLEARED is
foreclosed here** by `AC-6` and `AC-0`. A string that names a terminal node
without taking branch (c)'s words fails. The candidate carrying this edit
touches `crates/` and is therefore **`full` CI, never doc-only**.)*

> **Added by Steward amendment 2026-09-19. Unlike most close obligations, this
> one has only ONE branch available to it.** The usual form is *clears or names
> a live owner*, and `AC-6` above forecloses the first half: the row **remains
> `#[ignore]`d** by this node's own design, and `AC-0` bars adding attributes.
> So the row survives this node, and the only question is whether it survives
> pointing at something live.
>
> Today it does not. It opens
> `#[ignore = "RT-SUBCONTINUATION-LIFO-RELEASE-ORDER …` — `merged` — and names
> no other token at all. **This node is its live owner and this frame is the
> only record of that**, via the title's naming of
> `public_two_three_level_brackets_finish_and_release_lifo` as *"one of the
> fifteen originally selected ignored rows (population now 14)"*. A reader who
> starts at the row sees a merged node and stops. Measured at
> `evt_73fa9v0ca03ve`, where this row was one of 8 of 14 in that state.
>
> **If this node's repair lands and the row still refuses at depth 3** — which
> `AC-6` expects, as the same `ContinuationSpecialization` object-emission
> refusal — **then the successor owner is whatever node takes that refusal, and
> naming it is part of closing here.** If no live node owns it, the attribute
> says so in those words and names this node as having established it. Handing
> the row back to `main` still pointing at a merged node reproduces exactly the
> state `RT-SITEOP-ORPHANED-ROWS-REFUSAL-CENSUS` was cut to clean up.
>
> **Why an AC when `M7a` arm 2 already reaches this row at close.** `M7a` is a
> **gate**, run by the merge seats at close; an AC is a **design input**, read
> by the implementer before the work and by QA during review. **Reachability by
> the gate is not visibility to the author** (runtime-implementer,
> `evt_719q9chqy9dwz`). This is the same reason
> `RT-SITEOP-ORPHANED-ROWS-REFUSAL-CENSUS` carries `AC-11` despite arm 2.
>
> **Not licensed by this AC:** the census's `AC-6` proposal will name this node
> as the expected applier for this row. That is a **routing** statement backed
> by measurement. The **mechanism content** of the successor string is this
> node's finding to make — the census did not measure this row.

## 6. What this WP is NOT

- **It does not clear the px8ta row and must not try.** That row carries a
  second blocker at depth 3 owned by
  `RT-DEPTH3-CONTINUATION-CLAIM-UNDECLARED`. **Un-ignoring it here buys a red
  row for no information**, and an AC requiring it to pass would be
  unsatisfiable — the WP could only meet it by repairing a defect it does not
  own.
- **It is not the interp repair (`D1b`).** Interp violates the same rule on the
  composed-return nests. That is a real correctness defect with a known
  direction, and it is a separate WP so that the two engines' repairs are
  reviewed apart. **Do not fix both here.**
- **It does not author an ordering clause in `spec/`.** If `D0a` concludes one
  is needed, that is the enclave's to write and the Steward's to sequence.
- **It is not a measurement of the other fourteen ignored rows.** The "a
  looping test reports nothing past its first failure" finding is real, is
  recorded on the node, and is not this WP's work.

## 7. Estimated tier: T1

Semantic repair in the native backend's teardown path, on a property with no
explicit spec clause, where correctness comes from a derivation over locked text
and **six properties co-vary across the only two data points.** The diff may be
small; the reasoning that picks it is not. **Not a T2 mechanical change** — a
cheap seat would match native to interp or interp to native, or would take the
inner-kind correlate at face value and edit the release path, which section 2.5
rules out from source.

## 8. Contention

**Measured against the queue at `badc039da`.**

- `fa4fc3647b09b9480940f7d5d35c50ffb7978aeb` (`RT-SUBCONTINUATION-LIFO-RELEASE-ORDER`
  `D1`, routed, ahead of this WP) **rewrites
  `crates/ken-cli/tests/px8ta_oriented_subcontinuation.rs`** under hunk
  `@@ -261,28 +261,69 @@`. **`D0c`(C) edits that file, so cut your branch after
  it lands** — or stage `D0c`(C) on a copy and fold it after.
- Nothing else in the queue touches `ken-runtime/src`. `af2270b7b` is `docs/`
  plus a playbook; `9342062315`, `b042af474`, `fabcd98ed`, `89f1cc71b`,
  `45f66746b` are `docs/` only; `d0058baf1` is `ken-elaborator`.
- **`RT-CARRIER-PRODUCER-OCCURRENCE` is the lane's kick ahead of this WP.** It
  works in `constructors.rs`; this WP does not. They do not collide, but they
  are sequenced, not concurrent.
