---
id: RT-CONTEXT-FRAME-SLOT-HOLDS-ONE-PER-FUNCTION
title: "FunctionLocalRefs holds ONE constructed_context_frame per function while the value it stores carries the coordinate key that identifies it, so a function constructing frames for two worker bodies keeps only the last and the admission query for the other falls through SILENTLY to a route that refuses on a missing context_capture -- surfacing as a BoundaryCarrier arity message that is a fallback symptom, not the mechanism. The sibling field generated_context_captures has the identical single-slot shape and a diagnostic that asserts a universal its own container cannot support."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-17. Successor cut on the diagnosis RT-CARRIED-RESIDUAL-IH-ARITY landed (762d24347, Decision dec_5dv5ppn0a4msj), which explicitly scoped the repair OUT of that S/T1 node and named the sizing a Steward call. That node established the arity refusal is CORRECT at all four sites and must not be relaxed; this node is the actual repair for its four rows. Steward-filed per COORDINATION section 2."
---

# The mechanism, measured at `origin/main` `89dc3b0e5`

**One slot, one writer, two readers.** Verified by enumeration, not by the
writeup that reported it:

    crates/ken-runtime/src/cranelift_backend/lowering/

    mod.rs:1249    constructed_context_frame: Option<ConstructedContextFrame>
                   a single Option on FunctionLocalRefs

    core.rs:10866  THE ONLY WRITE   self.function_local
                                      .constructed_context_frame = Some(...)

    core.rs:13577  READ -- the admission query
    calls.rs:986   READ -- the operand gather

Every other occurrence tree-wide is a `constructed_context_frame: None`
test-fixture initializer (six of them).

**The admission query falls through silently** (`core.rs:13577`):

    if let Some(frame) = self.function_local.constructed_context_frame.as_ref() {
        if frame.worker_body_origin == body_origin
            && frame.worker_captures.len() == captures
            && frame.context_captures.len() == claims.len()
        { return Ok(true); }
    }
    if claims.len() != captures { return Ok(false); }
    Ok(claims.iter().all(|c| c.availability.context_capture.is_some()))

When the slot holds a frame for a **sibling** body, the inner `if` does not
fire, control reaches the generic path, and that path refuses because claim 0
has no `context_capture`.

> **TWO DIFFERENT FACTS PRODUCE ONE OUTCOME.** *No frame was ever stashed* and
> *a frame was stashed, for a different body* are indistinguishable here. The
> second is the live case and it is the one the message does not describe.

**Measured in the failing rows** (`RT-CARRIED-RESIDUAL-IH-ARITY`, landed): the
live frame matches on **both cardinalities** — 3 worker captures, 3 context
captures — while keyed to a sibling body. So every quantity the query checks
agrees, and the identity does not.

# THE KEY IS THE TRIPLE, NOT THE BODY ORIGIN. THE STRUCT SAYS SO BY NAME.

**This is the one thing that must not be got wrong, and the obvious reading of
the landed label gets it wrong.**

`RT-CARRIED-RESIDUAL-IH-ARITY`'s `#[ignore]` label says the rows readmit *"when
that slot is keyed by `worker_body_origin` rather than holding one frame per
function."* **Keyed by `worker_body_origin` alone is insufficient**, and
`ConstructedContextFrame`'s own doc comment forbids it in terms
(`mod.rs:1062-1066`):

> *"Keyed by the **complete planner-issued coordinate triple**, never by body
> origin alone. One function can reach two retargets over one body, and a frame
> consumed at the wrong one would be an arity-correct call carrying another
> occurrence's values — the exact silent shape `D6a` and
> `generated_context_captures` both guard against by retaining their key."*

    THE KEY   (continuation_origin, recursive_position, worker_body_origin)

**The gather consumer already matches on all three** (`calls.rs:986` filters
`continuation_origin`, `recursive_position`, then `worker_body_origin`), which
is independent confirmation that the triple is the real identity and the
one-field match at `core.rs:13577` is the narrow one.

⇒ **A fix keyed on body origin alone replaces a loud refusal with an
arity-correct call carrying another occurrence's values.** That is strictly
worse than the current failure: the current one stops, that one does not.

# The sibling field has the same shape, and a diagnostic that overclaims

`generated_context_captures: Option<GeneratedContextCaptures>`
(`mod.rs:1243`) — **single Option slot, one write (`units.rs:4028`), one read
(`calls.rs:346`), and its struct carries `worker_body_origin` as its first
field.** Same shape: the key lives in the value, the container holds one.

**Its consumer's error text asserts a universal its container cannot support:**

> *"a static worker binding for body origin {:?} is routed to a generated
> context, but this frame stashed no continuation-input suffix **for any
> body**"*

**A one-slot container cannot distinguish "no suffix for any body" from "a
suffix for one other body".** And the comment 15 lines above
(`calls.rs:331-336`) states that `D6a` **binds two workers over one body
origin** — so the multi-body case is known to the file that carries this
message.

**This is a finding, NOT a demonstrated failure.** No row is known to fail on
the sibling. It is scoped as a measurement in the frame, not as an assumed
deliverable — see `AC-5`.

# What must NOT change

**Per-function scoping is deliberate and load-bearing** (`mod.rs:1245-1248`):
the stored operands are `ir::Value`s **of this Cranelift `Function`**. The
repair is a keyed container **within** a function; entries must never be shared
across functions. **Making it a global map is the wrong fix and would be
unsound**, not merely untidy.

**The arity refusal stays.** `RT-CARRIED-RESIDUAL-IH-ARITY` established
`reject_carried_residual_arguments` is correct at all four of its call sites.
**Nothing here relaxes it** — the arity text is the fallback route's report,
and the repair stops control from reaching that route, rather than changing
what it says when it does.

# Why widening admission is already backstopped

**Stated in the code before this node existed** (`core.rs:13570-13576`):

> *"Admission here is a PERMISSION, not the authority. The consumer re-matches
> the frame on the complete planner-issued coordinate key and re-checks both
> cardinalities against the context's own declared frame header, so a frame
> admitted here and wrong there refuses at the call. The two cannot disagree
> silently."*

⇒ **The two-layer structure is the reason this repair is a keying change and
not a soundness argument.** The frame must preserve it: if the repair collapses
admission and authority into one check, it removes the backstop that makes
widening safe.

# Related

- [[RT-CARRIED-RESIDUAL-IH-ARITY]] — the diagnosis, landed. **Provenance and
  the four rows this node readmits.** Its label's `worker_body_origin` wording
  is corrected here, not inherited.
- [[RT-CAPTURE-CONTEXT-FRAME-EMIT]] — `merged`, `L`. Introduced
  `constructed_context_frame` at its `D2`. **The single-slot cardinality is
  from that node; the key discipline in the struct doc is also from it.**
- [[RT-IGNORED-FAILING-ROWS-INVENTORY]] — the ledger. These four rows are its
  `S1` cluster.
- [[RT-HOST-RESPONSE-ROUTE-KEY-COLLISION]] — **different subsystem, and worth
  reading beside this one.** Its hypothesis is a map keyed too narrowly while
  the value carries the discriminator; this is a container with no key at all.
  **Same addressing defect at two cardinalities. Do not fold them** — separate
  code, separate rows, and node 2's cause is not yet measured.
