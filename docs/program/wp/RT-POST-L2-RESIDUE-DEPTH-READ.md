# RT-POST-L2-RESIDUE-DEPTH-READ — work package

**Owner:** runtime
**Size:** M
**Tier:** T1 (see §7)
**Gate:** none
**Depends on:** nothing. Startable immediately.
**Node:** `docs/program/issues/RT-POST-L2-RESIDUE-DEPTH-READ.md`

> **READ THE NODE FIRST AND DO NOT SKIP ITS OPENING BLOCK.** Two instruments
> that look right are unavailable to this WP, and both were proposed in this
> node's own history by people who had just argued against the failure each one
> instances. This frame assumes you have read why.

## 1. Objective

Establish, by a **static path-scoped read**, how much refusal residue stands
between the four `RT-CONTEXT-FRAME-LABEL-CORRECTION` rows and a passing
lowering, **under the one hypothesized repair named in §3**, and report it with
its coverage stated.

**This is a measurement. It changes no production code.**

## 2. Fixed inputs, measured

All coordinates read at `9dfa6978ebf6c86e5d4ebafe8f2ecf6a565a1158`.
`git diff 9dfa6978e 3da9b9637 -- crates/` is **empty** across the six doc-only
publishes that followed, so every coordinate below carries to the current tip.

### The four rows, and the operands published for them

    px7m_hostresult_computational_match.rs:163   owner(5) binding 395 env 391
    px7m_hostresult_computational_match.rs:206   owner(5) binding 409 env 405
    px7l_checked_host_recursive_bind.rs:163      owner(3) binding 358 env 356
    px7l_checked_host_recursive_bind.rs:241      owner(3) binding 377 env 375

> **THESE ARE COORDINATE FIELDS, NOT `defining_owner`.** The triple is
> `ProducerLocalBinding::binding_owner` / `binding_origin` /
> `ProducerLocalLocator::environment_origin` (`continuations.rs:386-410`), all
> three inside `ContinuationSourceCoordinate::ProducerLocal` (`:450-453`) --
> which is the **only** operand `core.rs:9557-9567` interpolates. Do not read
> `owner(5)` as `defining_owner`; see §3c.

### The three known stops, in order

    L0   unforced       BoundaryCarrier arity report
    L1   core.rs:1230   agreeing_recursive_body_unit -- branches declare
                        different recursive body units
    L2   core.rs:9558   resolve_context_capture_claim, the `return Err(` on
                        `views.context_capture == None`.
                        RT-CONTSRC-PRODUCER-LOCAL D3b.  Owner: MERGED.

> **`:9558`, not `:9578`.** The two were reported by different seats and the
> file settles it: `:9558` is the `return Err(unsupported(`; `:9578` is the
> `frame,` field inside the `EntryFrame` arm and is not a refusal at all. The
> finding is unaffected -- same function, same construct, same text.

### `verify_entry_frame`'s six arms, fully enumerated, no catch-all

    core.rs:9595   fn verify_entry_frame(frame: ContinuationFrameIdentity,
                                         defining_owner: ContinuationEmissionOwner, ..)

    9604   (Predeclared,      Predeclared)
    9629   (GeneratedContext, Specialization)
    9747   (Predeclared,      Specialization)     refuses
    9759   (GeneratedContext, Predeclared)        refuses
    9785   (Predeclared,      Fusion)             refuses
    9797   (GeneratedContext, Fusion)             refuses

    continuations.rs:92-106   ContinuationEmissionOwner has 3 variants;
                              Predeclared(PredeclaredFunctionId) is the only
                              one carrying that payload type
    continuations.rs:4279     verify_predeclared_entry_frame_membership,
                              reached from core.rs:9615

### The ten further sites behind L2, read not forced

    9570   CurrentLexical claim presented to the entry-frame consumer
    9608   entry-frame claim names a predeclared frame that is not the one held
    9637   claim names a generated context of the wrong specialization
    9661   named generated context frame the planner never interned
    9688   9706   9716   9729   9735   9748    (verify_entry_frame, continued)

**Two forcings revealed two layers. One read revealed ten more.** That ratio is
the argument for the instrument and it belongs in the measured outcome.

### The bound in the other direction

`grep -c 'unsupported(' core.rs` = **342**. A presence oracle only. It does not
say which are reachable, and it is why a flat enumeration is not the instrument.

## 3. THE DESIGN JUDGMENT, FRONT-LOADED

**Two decisions are already made. Do not re-open them; they cost an exchange
each and both were wrong on the first try.**

### 3a. The instrument is a STATIC PATH-SCOPED READ. Never a trace.

The rows stop at L2 today, so a dynamic trace of any of the four contains
**zero** of the ten sites. Obtaining one requires forcing past `:9558`, which is
the method this instrument replaces. **The word "trace" must not appear in any
deliverable of this WP.**

### 3b. The count has no subject without a named repair. `H` is chosen at `D2`.

`frame` comes from the claim; the absent claim *is* the L2 refusal. So `frame`
has no value today and acquires one only from a repair. **Every number this WP
reports must be attributable to a named `H`.**

**`H` is NOT named in this frame, deliberately.** A first draft named
*"supply a predeclared-frame claim -> arm 9604"* and that selection was
unsound: it eliminated `9759` from a surviving PAIR that came from an unmeasured
pruning. Name `H` at `D2`, from `D1`'s surviving pair, not before.

### 3c. The pruning is real; its INPUT is unmeasured. That is `D0`.

`defining_owner` alone prunes six arms to two:

    Predeclared(_)      -> 9604, 9759
    Specialization(_)   -> 9629, 9747
    Fusion(_)           -> 9785, 9797

**Nobody has measured `defining_owner`.** The frame's first draft asserted
`Predeclared(_)` on the strength of the implementer's published
`owner PredeclaredFunctionId(5)`. That value is
`ProducerLocalBinding::binding_owner` -- a field of the *coordinate* -- and the
refusal at `core.rs:9557-9567` interpolates `{coordinate:?}` **and nothing
else**. `defining_owner` is a separate parameter (`core.rs:9555`) that is not
printed there. **The two facts share a newtype and are not the same fact**; the
node's own section carries the full derivation and the ten other roles that
share that type.

> **If `defining_owner` is `Specialization(_)`, the survivors are `{9629,
> 9747}` and `9604` is UNREACHABLE for these rows** -- the first draft's `H`
> would have been measuring a branch the rows cannot enter. That is the size of
> the error `D0` prevents.

**Re-derive the pruning from `D0`'s measurement. Do not re-confirm the pair
above.**

## 4. Deliverables

**D0 — measure `defining_owner`. ONE OPERAND, ZERO FORCING.** Print
`defining_owner` alongside `{coordinate:?}` at `core.rs:9558` and record its
variant for each of the four rows. **The rows already stop at `:9558` unforced**,
so this is not a forcing step and not a rung — it is one more operand at a
refusal the rows reach on the run that is already being made. Revert the probe
(`grep -c RTPROBE` = 0) before the candidate is cut.

**Do not infer it from the refusal's prose.** *"A generated context capture..."*
is a fixed string in the format literal, identical on every execution.

**D1 — the surviving arm pair, derived.** From `D0`'s measured variant, state
which two of `verify_entry_frame`'s six arms are reachable for these rows and
which four are not. Record whether the four rows agree on the variant; if they
do not, that split is a finding and `D2` onward is per-group.

**D2 — name `H`.** Choose the hypothesized repair from `D1`'s surviving pair,
and **state why the other member of the pair was not chosen.** If one member
refuses outright its residue is one site and measuring it answers nothing —
that is a valid reason, but it must be stated against the *measured* pair, not
assumed.

**D3 — the path-scoped read under `H`.** Starting at the arm `H` lands on,
enumerate every refusal site reachable on the path a row takes under `H`,
following into callees including `continuations.rs:4279`. For each site record:
coordinate,
the condition it refuses on, and **whether the four rows' known operands decide
it, leave it open, or cannot reach it.** Three-way, not two: *undecided from the
operands you have* is a distinct and expected outcome and must not be collapsed
into either of the others.

**D4 — the terminus.** State where the path ends and which of the two endings it
is: lowering returns `Ok`, or it reaches a refusal that `H` does not address.
**If it is the latter, name the refusal and its owning node, and say whether
that owner can close these rows.**

**D5 — coverage, stated as a limit.** What the read covered and what it did not.
Any branch not followed is recorded as **unread**, never as empty. A site whose
reachability the operands do not decide is listed, not dropped.

**D6 — the measured-outcome section** in this frame, including the
forcing-versus-reading ratio from §2 and the escape-hatch disposition from §5.

## 5. Acceptance criteria

**AC-0 — no production change.** `git diff --stat <base> HEAD -- crates/` shows
**zero** changes under `crates/`, or only env-gated probes that are reverted
before the candidate is cut (`grep -c RTPROBE` = 0). *Control:* the diff itself.

**AC-1 — `H` is named in every count.** No residue number appears in any
deliverable without `H` attached to it. *Control:* a reader can ask "residue
under what?" of any number in the report and find the answer adjacent to it.

**AC-2 — the three-way classification is actually three-way.** D3 contains at
least the *category* "undecided from the rows' operands", and if it is empty
that emptiness is asserted deliberately rather than by omission. *Control:* an
empty category stated as empty is distinguishable from a category that was never
considered; a report with only two categories fails this.

**AC-3 — coverage is a limit, not a boundary.** D5 distinguishes unread from
empty. *Control:* every branch named in D3 appears in D5 as covered or unread.

**AC-4 — the escape hatch is exercised or explicitly not.** D6 states either the
residue count under `H`, **or** that the residue was too large to work by hand
and the pass stopped. **Both are acceptable completions.** *Control:* a report
that is neither -- a partial count with no statement about why it stopped -- is
the failure this AC exists to catch.

**AC-5 — the pruning is DERIVED from D0, never re-confirmed.** D1 names the
surviving pair by citing D0's measured `defining_owner` variant, and the four
unreachable arms are named as unreachable *because of that measurement*.
*Control:* the pair `{9604, 9759}` appearing in D1 without D0 having measured
`Predeclared(_)` is this AC failing — that pair is exactly what the unsound
first draft asserted, so its presence is indistinguishable from inheritance
unless D0 is cited beside it.

**AC-6 — no row-closure claim anywhere.** *Control:* search the deliverables for
any sentence asserting a row is closed, closer, or unblocked. There should be
none; this WP measures.

> **AC-4 IS THE ONE MOST LIKELY TO BE SKIPPED, BECAUSE STOPPING FEELS LIKE
> FAILING.** It is not. A residue too large to enumerate by hand is a real
> finding about the depth of this stack and it is worth more than a number
> produced by pushing through. **Report it and stop.**

## 6. What this WP is NOT

- **Not a repair.** `H` is *hypothesized*, not implemented. Do not build it.
- **Not a forcing run.** No `KEN_RTPROBE_*` gate is needed and none should be
  added. If you reach for one, re-read the node's opening block.
- **Not an enumeration of `core.rs`'s 342 refusal sites.**
- **Not `RT-CONTEXT-FRAME-LABEL-CORRECTION`'s `D3`.** That repair stands on its
  own merits and is tracked there.

## 7. Estimated tier: T1

The mechanical part -- walking a call graph and listing sites -- is T2. **The
part that decides whether this WP is worth anything is not:** classifying each
site three ways against operands that only partially decide it, recognising when
the residue has outgrown the instrument, and resisting a count that would read
as progress. A cheaper seat produces a site list; the judgment calls are where
the value is.

## 8. Contention

None expected. The WP writes no `crates/` file. Its only repo writes are this
frame's §9 and the node's status. `RT-CONTEXT-FRAME-LABEL-CORRECTION` is the
adjacent node and this WP does not edit it.

## 9. Measured outcome

To be written by the implementing ring.
