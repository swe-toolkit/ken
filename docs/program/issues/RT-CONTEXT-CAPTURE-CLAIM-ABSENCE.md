---
id: RT-CONTEXT-CAPTURE-CLAIM-ABSENCE
title: "The A-versus-B discriminator RT-CONTEXT-FRAME-REFUSAL-DEPTH-CENSUS measured itself unable to answer: at the depth-3 stop `resolve_context_capture_claim` (core.rs:9551) the availability claim is ABSENT rather than rejected, and whether the planner SHOULD have issued a ContinuationEnvironmentClaim for this coordinate (outcome A, repairable) or no claim can exist here (outcome B, terminal -- the disposition is an exemption row plus a rewritten label, NOT a repair) turns on the producer's conditions, which no node has read. Four ignored rows converge completely on this one stop, so one disposition serves all four."
status: merged
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-18, on re-reading the merged RT-CONTEXT-FRAME-REFUSAL-DEPTH-CENSUS deliverable (docs/program/evidence/rt-context-frame-refusal-depth-census.md, landed 9682927b7). That census closes its verdict with 'the discriminator is a planner-side question -- is RT-CONTSRC-PRODUCER-LOCAL D3b's missing claim a gap or a correct absence? -- and answering it means reading the producer's conditions, which is a different node's work.' This is that node. Under the operator's standing L1 direction (2026-09-15, 'The other tests should be fixed'). Steward-filed per COORDINATION section 2."
---

> # THE QUESTION THIS NODE OWNS WAS FIRST RECORDED WRONG, BY ME
>
> **Before this node existed, the Steward recorded the open question for these
> four rows as *"READMISSION CONDITION UNKNOWN -- whether `core.rs:1230`
> (`agreeing_recursive_body_unit`) is the last layer or the next in a queue."***
> That question is **ANSWERED**. `core.rs:1230` is **not** the last layer; the
> census walked past it to a third stop, and all four rows **converge there
> completely**.
>
> **The error was a sourcing error, and it is the reason this banner is here.**
> The "unknown" wording lives in the four rows' live `#[ignore]` labels, written
> by `RT-CONTEXT-FRAME-LABEL-CORRECTION` at `b0421afd0`, and in this census
> node's own `origin:` text, which describes **its predecessor's** limit. Both
> are true about when they were written. **The census then answered it and
> nothing propagated back into either surface.** A reader who consults the label
> -- the artifact physically attached to the failing row, and so the first thing
> anyone reads -- re-opens a settled question.
>
> ⇒ **Rewriting those four labels is a DELIVERABLE of this node, not tidying.**
> The evidence that the staleness is live rather than theoretical is that it
> cost the Steward a full cut of the wrong node before the deliverable was read.

# What the census settled, and what it handed on

`docs/program/evidence/rt-context-frame-refusal-depth-census.md`, measured at
`origin/main` `0298c51eb08df955c502a4fa2bfd17e315745e22`. **Depth 3 on every
row; all four converge, same stops in the same order.**

    row                                          L1        L2        L3
    px7l delayed_capturing_generic_bind          arity  ->  branch ->  claim
    px7l runtime_selected_non_unit_response      arity  ->  branch ->  claim
    px7m dynamic_ok_payload                      arity  ->  branch ->  claim
    px7m dynamic_err_payload                     arity  ->  branch ->  claim

**Every coordinate below re-verified by the Steward at `e75f1fe27`** by symbol,
not by position:

    L1  recursive_position_captures_all_planner_recoverable   core.rs:13532
    L2  agreeing_recursive_body_unit                          core.rs:1230
        its own two-direction unit test                       core.rs:1271
    L3  resolve_context_capture_claim                         core.rs:9551
        the absence arm                                       core.rs:9557-9566

**`L1` and `L2` are both CORRECT and were forced only as measurement.** `L2`
carries a unit test asserting it refuses (`[41,41]` agrees, `[41,42]` refuses on
the exact message); forcing it reddens that test, which is direct evidence the
guard is deliberate and covered.

> **`L1` AND `L3` READ THE SAME FIELD.** `core.rs:13594` is
> `claims.iter().all(|claim| claim.availability.context_capture.is_some())` --
> the same presence test `L3` unwraps at `core.rs:9557`, all-quantified at the
> admission gate instead of per-claim at the consumer. `L1`'s own doc comment
> (`core.rs:13527-13531`) says so: *"`context_capture` is the field
> `resolve_context_capture_claim` consumes; a `None` there is precisely the
> refusal this gate must respect rather than route around."* **The census forced
> `L1` to `Ok(true)` -- the "route around" that comment names -- and reported
> meeting `L3` as a discovery.** ⇒ **"Depth 3" over-counts: two distinct
> predicates plus a convergence result, not three independent layers.** The
> Architect's `evt_4hzm4praxvdrq`, re-measured before folding; frame §1a.A.

## `L3` is a different KIND of stop, and that is the whole node

    L1  guard REJECTS data that is PRESENT   (a frame exists; its key mismatches)
    L2  guard REJECTS data that is PRESENT   (two units exist; they differ)
    L3  the data is ABSENT                   (`views.context_capture` is None)

`L3` is `let Some(claim) = views.context_capture else { return Err(...) }`.
**Nothing is being rejected.** The census stopped here on principle rather than
on budget: forcing `L1` and `L2` removed a rejection of data the compiler
already held, whereas **forcing `L3` would mean fabricating a
`ContinuationEnvironmentClaim` the planner never issued**, and every stop found
after that would be a fact about the fabrication.

# THE FORK, FRONT-LOADED AND NOT RULED HERE

    A  REPAIRABLE, depth   iff the planner SHOULD have issued a claim for this
       beyond L3 UNKNOWN       coordinate and did not
    B  finite, TERMINAL     iff no claim can exist here, and the refusal is
                            correct and permanent

**`A` is NOT "finite, repairable"**, though the census's verdict says so. Its
own limits section says depth 3 is *"not proof that nothing lies behind `L3`"*
and that a repair *"may expose an `L4` this census never saw"* -- and under `A`
that is exactly what happens: a real claim is issued, `L3` passes for real, and
what sits behind it is unmeasured. **A repair RE-OPENS the census.** See the
frame's §1a.B; the correction is the Architect's at `evt_4hzm4praxvdrq`.

**Both fit every measurement in the census.** It says so itself, and that is a
stated limit rather than an incomplete walk.

> **OUTCOME `B` IS A REAL RESULT AND THE NODE CLOSES GREEN ON IT.** If no claim
> can exist at this coordinate, these four rows are **correctly refused and are
> not a defect**, and the disposition is an **exemption row plus a rewritten
> label** -- not a repair. Report it as the finding it is. A node whose
> deliverable is *"these rows do not clear here"* has delivered.
>
> **Do not arrive at `A` because `A` is the outcome that produces code.** Three
> nodes in this series each closed one layer, each found the refusal at its own
> layer was correct, and each located the cause upstream. A fourth blind repair
> is a fourth cycle to learn the same shape.

# THE CENSUS NAMED ONE PRODUCER ARM. THE PRODUCTION SOURCE IS ONE HOP UP.

The census writes: *"A producer site exists (`continuations.rs:4625`,
`context_capture: Some(claim)`), so this is not an unimplemented field -- it is
a coordinate for which no claim was produced."* **That conclusion is sound and
its evidence base is one arm** — which is the defect worth naming. Do not
inherit *"the producer"* as singular.

> **THE STEWARD'S FIRST REPLACEMENT POPULATION WAS ALSO WRONG, AND THAT ERROR
> IS THE REUSABLE PART.** This section first listed **six** literal arms --
> three `Some`, three `None` -- from a grep over `planning/`. **Five of the six
> are `#[cfg(test)]` fixtures** (`mod tests` spans `continuations.rs:8324-11079`)
> and **there is not one production `context_capture: None` literal in
> `planning/`.** Raised by the Architect at `evt_4hzm4praxvdrq`, re-measured and
> confirmed by the Steward before folding. ⇒ **A PATH IS NOT A PRODUCTION
> FILTER.** `planning/` reads as production and carries an inline test module.
> Full correction in the frame at §1a.C.

**The production picture at `e75f1fe27`:** `:4625` is the only production
literal `Some`; `:4560` forwards a variable; `:830` is a faithful pass-through
(`map(..).transpose()?`, no `Some`-to-`None` collapse); and there are **zero**
assignment-form writes anywhere in `crates/ken-runtime/src/`, so this is the
complete writer set.

⇒ **Production `None` has exactly ONE source, one hop above the arm:**
`predeclared_entry_frame_slot` returning `Ok(None)`, whose own comment reads
*"`None` when the frame declares no member: fails closed. Nothing invents a
position, and no fallback reads the direct-emission index as a frame slot."*

**So `D0` re-aims:** not *"which arm does the coordinate take?"* but **"does the
predeclared entry frame declare a member for this coordinate?"** Narrower, in a
different function than this node first named, and directly measurable.

# OUTCOME `A` IS NOT A LOCAL REPAIR

Every read of `context_capture` in `crates/ken-runtime/src/`:

    core.rs:13594   .all(.. .is_some())    L1, ADMISSION
    core.rs:9557    let Some(claim) ..     L3, REFUSAL
    calls.rs:982    .any(.. .is_none())    gather_cannot_serve, ROUTE SELECTION

**`calls.rs:982` does not refuse -- it SELECTS.** Issuing a claim where there
was none flips `gather_cannot_serve` for that claim and changes which route
`calls.rs` takes: **silently, and outside the four rows this node fences.**

⇒ **If `A` is ever taken, `calls.rs:982` needs a named disposition in the same
node.** This is the measured reason behind *"do not arrive at `A` because `A`
produces code"* -- it is not a caution about motive, it is a fact about blast
radius. The Architect's `evt_4hzm4praxvdrq`; frame §1a.E.

# What must not happen

- **Do not force `L3`.** Fabricating a claim the planner never issued measures
  the fabrication. The census declined this deliberately; do not treat its
  restraint as an unfinished step.
- **Do not repair at `L1` or `L2`.** Both are correct, both were forced only as
  measurement, and `L2` has a live two-direction unit test at `core.rs:1271`
  that a relaxation reddens.
- **Do not size this as a repair before `D0` reports.** The `A`/`B` split
  decides whether there is any repair at all.
- **Do not resolve a row by symbol name alone.** `nested_ok_payload_...` and
  `nested_err_payload_...` have same-named twins one file over; these four are
  keyed by FILE and LINE below and confirmed by function name.

# The rows, at `e75f1fe27`

    crates/ken-cli/tests/px7l_checked_host_recursive_bind.rs:163
      delayed_capturing_generic_bind_agrees_across_real_executors
    crates/ken-cli/tests/px7l_checked_host_recursive_bind.rs:241
      runtime_selected_non_unit_response_is_consumed_across_real_executors
    crates/ken-cli/tests/px7m_hostresult_computational_match.rs:163
      dynamic_ok_payload_selects_a_multistep_tree_across_real_executors
    crates/ken-cli/tests/px7m_hostresult_computational_match.rs:206
      dynamic_err_payload_selects_a_multistep_tree_across_real_executors

**One disposition serves all four.** The census established they converge
completely, which is what makes this one node rather than four.

# D3 exemption — one record covering four rows

**B, at clean `1f33e45d87070d4c3420d86aaba6cf85b89a30aa`.** This single
exemption covers both `px7l_checked_host_recursive_bind.rs` rows and both
`px7m_hostresult_computational_match.rs` rows named above. Each remains ignored:
the unforced baseline stops first at the correct `BoundaryCarrier` arity refusal
at `L1`; the prior diagnostic forced `L1` and `L2` only to reveal the common
`L3` claim-absence refusal. It did not force `L3` or measure what lies beyond.

The producer was called on each target `ProducerLocal` coordinate and returned
`Ok(None)`: binding/environment origins 358/356 and 377/375 in `px7l`, 395/391
in `px7m` dynamic-ok, and both 409/405 and 410/410 in dynamic-err. Each
predeclared frame has two actual `EntryAbi` members, witnessed by `Ok(Some(0))`
and `Ok(Some(1))` controls. `continuation_owner_entry_sources` constructs its
entry run solely from the descriptor's Parameter+Capture ABI slots;
whole-coordinate matching in `predeclared_entry_frame_slot` cannot declare a
mid-body
`ProducerLocal` as one of those members. An absent *predeclared-entry* capture
claim is therefore correct under the current ABI, not an omitted planner
claim. A same-coordinate `Ok(Some)` followed by `L3` `None` would have refuted
that reading, but none occurred. Generated-context projections that take no
predeclared arm describe a different consumer and do not supply entry members.

`L1` is `recursive_position_captures_all_planner_recoverable`; `L2` is
`agreeing_recursive_body_unit`; `L3` is `resolve_context_capture_claim`.
`L3` is the terminal claim-absence refusal for this predeclared-entry route.
There is **no live owner for follow-on representation work** at these rows;
the four `#[ignore]` labels name this node's B finding rather than a repair.
`calls.rs::gather_cannot_serve` retains its route selection unchanged: a missing
claim remains missing; a matching, validated constructed frame may take its
existing route, while other cases continue to refuse. This ruling does not
exclude a separately scoped, authenticated creation-site constructed frame or
generated-context representation, and authorizes neither an invented entry
slot nor a lowering bypass. The `L2` two-direction unit test and all guards
remain in force. No production source change belongs to this exemption.

# Related

- [[RT-CONTEXT-FRAME-REFUSAL-DEPTH-CENSUS]] -- `merged`. Its deliverable is
  `docs/program/evidence/rt-context-frame-refusal-depth-census.md` and it is
  the input to this node. **Read the evidence file, not the node's title or
  `origin:`** -- those describe the predecessor's limit and are the exact
  surfaces that misled the Steward into cutting the wrong question.
- [[RT-CONTSRC-PRODUCER-LOCAL]] -- `D3b` is named in both refusal messages at
  `core.rs:9551`. Its conditions are what `D0` reads.
- [[RT-CARRIED-RESIDUAL-IH-ARITY]] -- `closed`, premise refuted. Established
  the `L1` arity refusal is correct at all four sites and is a fallback
  symptom.
- [[RT-CONTEXT-FRAME-SLOT-HOLDS-ONE-PER-FUNCTION]] -- `closed`, refuted on its
  own premise. Keying the slot by `worker_body_origin` readmits nothing.
- [[RT-CONTEXT-FRAME-ADMISSION-EVIDENCE-KEY]] -- `draft`, queued. A different
  question at the same subsystem: whether the permission seat's delegation to
  `calls.rs` is recorded where its enforcer reads it. **Do not fold.**
