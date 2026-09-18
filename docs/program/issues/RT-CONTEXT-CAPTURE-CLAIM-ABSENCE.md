---
id: RT-CONTEXT-CAPTURE-CLAIM-ABSENCE
title: "The A-versus-B discriminator RT-CONTEXT-FRAME-REFUSAL-DEPTH-CENSUS measured itself unable to answer: at the depth-3 stop `resolve_context_capture_claim` (core.rs:9551) the availability claim is ABSENT rather than rejected, and whether the planner SHOULD have issued a ContinuationEnvironmentClaim for this coordinate (outcome A, repairable) or no claim can exist here (outcome B, terminal -- the disposition is an exemption row plus a rewritten label, NOT a repair) turns on the producer's conditions, which no node has read. Four ignored rows converge completely on this one stop, so one disposition serves all four."
status: ready
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
> census walked past it to a third stop and found the stack **finite at depth
> 3**.
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

    A  finite, REPAIRABLE  iff the planner SHOULD have issued a claim for this
                           coordinate and did not
    B  finite, TERMINAL    iff no claim can exist here, and the refusal is
                           correct and permanent

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

# THE CENSUS NAMED ONE PRODUCER ARM. THERE ARE SIX.

The census writes: *"A producer site exists (`continuations.rs:4625`,
`context_capture: Some(claim)`), so this is not an unimplemented field -- it is
a coordinate for which no claim was produced."* **That conclusion is sound and
its evidence base is one arm.** Measured by the Steward at `e75f1fe27`, all
occurrences in `planning/`, not a sample:

    continuations.rs:4625    context_capture: Some(claim)
    continuations.rs:10491   context_capture: Some(declared)
    continuations.rs:10541   context_capture: Some(ContinuationEnvironmentDraft::EntryFrame { .. })
    continuations.rs:10280   context_capture: None
    continuations.rs:10403   context_capture: None
    continuations.rs:10423   context_capture: None

⇒ **Three arms issue a claim and three decline.** Do not inherit *"the
producer"* as singular. **The three `None` arms are where the fork is decided**:
an arm that declines for a stated reason is evidence for `B`; an arm reached by
fallthrough with no reason is evidence for `A`. Which arm each of the four rows'
coordinate actually takes is `D0` and nobody has measured it.

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
