---
id: RT-REQUIRED-OCCURRENCE-PROJECTION
title: "Project the required consuming occurrence into lowering as a validated value derived in planning -- a second, differently-named relation, never the key's source-level certificate and never a bare carrier"
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-CONSUMING-OCCURRENCE-ROUTE-WIRE]
blocks: []
github: null
origin: "Architect ruling evt_1y5bfgkg6v5b6 (2026-08-15), answering the Steward's mechanism question evt_5pmepsy308e8r on the RT-CONSUMING-OCCURRENCE-ROUTE-WIRE hard stop (runtime-implementer evt_3kk9xbfpfwcqn, measured at 46a8ba199). The ruling states every option in mechanism terms and pre-authorizes the D1 fork so the ring cannot stall a second time. Steward-filed per COORDINATION section 2."
---

> # THE SURFACE IS RULED. YOU ARE NOT DECIDING WHETHER TO OPEN IT.
>
> **The Architect ruled the projection is the right surface and may be opened**
> — but **not as a widening of the key, and not as a bare carrier.** Four
> constraints below are his, in mechanism terms, so **no part of this frame is
> guessing at his answer.** Do not re-litigate any of them.
>
> **`AC-5` of the predecessor is NOT relaxed.** The ban was correct for that
> node; the projection belongs to **this successor with its own ACs**, not to a
> widened frame there.

## Why the existing key slot is refused, and it is stronger than "the validator objects"

The predecessor's measurement routed the lagged value through
`ContinuationSpecializationKey.consuming_occurrence` and was rejected before
lowering. **The reason is definitional, not a guard being strict.** From
`rederive_consuming_occurrence` (`static_transition.rs:11044+`), read rather
than inferred from the refusal sentence:

- `validate_continuation_consuming_occurrences` **recomputes** the occurrence
  and demands `rederive(...) == Some(claimed)`. The stored value must equal an
  independent derivation — it is a **checked-redundant certificate**, not
  storage that happens to be guarded.
- The function's own doc establishes the position-zero relation is **injective
  by construction**: *"a second match therefore cannot reuse this continuation
  occurrence as its own position-zero child."*

⇒ **`consuming_occurrence` is uniquely determined by `key.continuation_origin`
(plus `producer_alternative` for the body).** It carries **zero independent
information** and is **identity-inert for interning** — it can never separate
two keys that `continuation_origin` does not already separate.

⇒ **That slot already means something else, definitionally.** Putting the lagged
value there would not merely fail validation; it would **falsify the injectivity
premise the validator's own correctness argument rests on.**

## THE NAMING CONSTRAINT — this is where the chain will recur if you skip it

**Two different relations would otherwise share one field name, and the roles
are easy to swap in prose.**

| relation | what it names | where it lives |
|---|---|---|
| **source-level** consuming occurrence | the eliminator whose **position-zero child is this continuation** | re-derivable from `continuation_origin`; **already exists**; belongs to the key |
| **required** consuming occurrence | the one the **depth-2+ consumer** needs, **lagged one level** | this node's new surface |

**They coincide at depth 1 — which is exactly why depth 1 advanced** in the
predecessor's measurement, and exactly why the distinction is easy to lose.

⇒ **The new surface may NOT be called `consuming_occurrence`.** Give it a name
that says **required** or **consumer-level**, so no future reader — and no
future ruling — can silently substitute one for the other.

## Deliverables

**`D1` — THE FORK, AND IT IS PRE-AUTHORIZED. Measure it first, before building
anything.**

**Is the required (lagged) occurrence derivable from the plan at the consumer's
level?** The Architect's §4 shape *assumes* it is; **he explicitly declined to
assert it unmeasured.**

- **Derivable** ⇒ **proceed to `D2` on his authority. Do not come back to ask.**
- **NOT derivable** ⇒ **STOP and return to the Architect. Do NOT fall back to a
  bare carrier.** A projection that cannot be re-derived is an unchecked
  assertion crossing a layer boundary; the right answer is then a **different
  shape**, not a weaker version of this one. That is his design call.

**`D2` — derive and validate in PLANNING.** Both stay where the plan lives,
beside `rederive_consuming_occurrence`. **One authority, one derivation.**

**`D3` — project the validated value into lowering.** Lowering **receives a
value it cannot forge and does not re-derive.**

**`D4` — one consumer, and the measurement.** Install the depth-2+ consumer and
record per row what the route does, in the predecessor's shape: advances,
refuses at the same boundary, or refuses at a new one — **naming the boundary
and the refusal each time.**

## Acceptance criteria

**`AC-1` — the projection carries its own re-derivation.** A merely-carried
value with no validator is an **unchecked carrier**, and would make the depth-2+
path **strictly weaker than the depth-1 path it extends** — a regression dressed
as a feature. The predecessor design's virtue is that the key's field is
validated against an independent derivation; this must inherit it.

**`AC-2` — the new relation is named distinctly.** Not `consuming_occurrence`.
See the naming constraint above; this is an acceptance criterion, not a style
note.

**`AC-3` — the key is untouched.** `ContinuationSpecializationKey`'s definition,
its `consuming_occurrence` field, and
`validate_continuation_consuming_occurrences` are unchanged. **The injectivity
premise stays true.**

**`AC-4` — NOT keyed on `defining_function_id`, anywhere.** Neither the
projection nor its validator may use it. The predecessor measured
`(None, None, None)` on the reached recognize/rebind/consume path: these
compiles use `RecursiveDescent`, `defining_function_id` is absent throughout,
and **both `Option<FuncId>` inequality guards pass VACUOUSLY.**

> **Function provenance is the natural way to disambiguate occurrences and it is
> measurably inert exactly where the new consumers live.** Anything built on it
> **would validate nothing on this path while reading as guarded** — the worst
> shape a control can take. **The disambiguator must be the occurrence/position
> structure the plan already provides.**

**`AC-5` — lowering is not handed the plan.** `StaticTransitionPlan` does not
become a field on the lowering context. Re-deriving at the funnel would be a
**far larger** representation widening than the projection and would put
derivation authority in the wrong layer.

**`AC-6` — no closure is assumed.** Same rule the predecessor carried and the
same reason: the original stop named a further `Closure`/static-worker refusal
**and** a retained standalone recognition. **A measured refusal with the refusal
attributed discharges `D4`.**

**`AC-7` — no-regression, in CI** (`COORDINATION §12`).

## Fixed inputs, measured at `46a8ba199`

- **`consuming_occurrence` has ZERO occurrences in `lowering/core.rs`.** The
  "one consumer at the funnel" in the predecessor's report was the **reverted
  probe's**, not an existing seat. **This is a genuinely new edge either way** —
  nothing is being reused.
- **Lowering does not hold a `StaticTransitionPlan`.** The only mention is a
  parameter on a free function (`:981`), not a field on the lowering context.
- Row 4 depth 1 advanced to a **new `Closure` refusal**; row 5 sits behind a
  later `StaticWorkerBinding` refusal; **row 1 is a different class** at
  `NativeJoinPlanV1` and is not this node's population.

## Excluded

- **Row 1** — a different class, not a further depth. Its `None` split is `H4`
  of [[RT-CONTKEY-REFUSAL-PROFILE-SPLIT]].
- **Retiring a residual class** — `enum RecursiveDescentResidual`
  (`cranelift_backend/lowering/core.rs:1979`) still carries both live variants;
  [[RT-RECURSOR-TRANSPORT]] owns that and is active.
- **Reopening `D2k-1c`** in [[RT-LEXICAL-RECURSOR-CONSUMERS]] — a wrong cut.

## Stop condition

**`D1`'s NOT-derivable branch goes to the Architect, not to the Steward** — he
pre-authorized the derivable branch precisely so the ring does not stall a
second time, and reserved the other branch as his. Everything else — sizing,
sequencing, whether an increment is releasable — is the Steward's.

## Why this earns a slot

**The predecessor advanced row 4 depth 1 with a real production consumer and
proved the depth-2+ boundary is a representation problem rather than a missing
fact.** The fact exists and is carried; it has nowhere lawful to live at the
consumer's level. This node builds the place.

**Its cheapest outcome is `D1` returning NOT-derivable**, which costs one
measurement and routes a design question to the person who owns it — and the
Architect has already said that is the correct outcome in that case, not a
failure.
