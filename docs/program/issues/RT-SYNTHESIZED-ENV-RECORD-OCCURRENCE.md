---
id: RT-SYNTHESIZED-ENV-RECORD-OCCURRENCE
title: "Give the unit-boundary environment record a planner-issued occurrence by extending the synthesized producer arm, so the closure crossing is attempted at the seam that actually refused it"
status: active
owner: runtime
size: M
gate: none
depends_on: [RT-CLOSURE-CROSSING-ELIMINATE]
blocks: []
github: null
origin: "Steward, 2026-08-15, on the operator's challenge to substantiate the claim that covering the refused closure-crossing rows requires inventing a representation. It does not, and the claim is withdrawn. Every fixed input below was measured by the Steward at origin/main 6d56a700c before framing. Steward-filed per COORDINATION section 2."
---

> # THE STEWARD'S SIZING WAS WRONG, AND THE CORRECTION IS THE FRAME
>
> The Steward reported that eliminating the closure crossing needed a
> **cross-unit representation that does not exist**, sized it as a large design,
> and put a product fork to the operator whose cheapest option was to accept a
> capability narrowing. **The operator asked what substantiated the impossibility
> claim. Nothing did.**
>
> **The mechanism for a compiler-created aggregate to carry planner authority is
> production code and is one enum arm wide.** Its own doc says so:
>
> > The two arms are the two ways an aggregate comes to exist... A source
> > aggregate is named by its own occurrence in the program. **A synthesized one
> > has no occurrence to be named by, so it is named by the closed compiler role
> > that builds it** — never by the origin it happens to be emitted at.
>
> ⇒ The question `D1` stopped in front of — *can the planner issue an occurrence
> for an aggregate no source expression produced?* — **was never asked, and the
> answer already in the tree is yes.**
>
> **What was true is much narrower: the synthesized arm's vocabulary is
> host-result-shaped and does not reach a unit boundary today.** That is an
> extension of a closed, checked mechanism, not an invention. This node attacks
> it.

## What refused, restated at the right seam

`RT-CLOSURE-CROSSING-ELIMINATE` `D1` synthesized a `Record` to carry the
captured environment and was refused at `reconcile_source_aggregate`
(`lowering/mod.rs:6744`):

```rust
let Some(occurrence) = value.source_aggregate_producer() else {
    return Err(unsupported(
        lowered_value_kind(value),
        "a source aggregate reached the carrier with no planner-issued producer \
         occurrence, so it would name no ownership record and could only be given \
         the authority of wherever it happened to be transferred",
    ));
};
```

**That is a missing occurrence, not a missing representation.** The Architect
noticed the same thing from the other direction in `evt_1ra9asrda1t94`: the
obligation *"does not require inventing a carrier — which is consistent with
`D1` having refused at an ownership-record seam rather than at a representation
or admissibility seam."* Both readings converge and neither was acted on.

## Fixed inputs, measured at `origin/main` `6d56a700c`

All in `crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs`
unless noted.

| fact | site |
|---|---|
| `AggregateOccurrenceProducer` has two arms, `Source(StaticOriginId)` and `SynthesizedUse { owner, seat, path, role }` | `:3956` |
| both arms are populated in production, then renumbered into `AggregateOccurrenceId` by sorted index | `:5683` source, `:5754` synthesized, `:5776` renumber |
| the synthesized push **hardcodes** `shape: PlannedAggregateShape::Constructor` | `:5764` |
| `SynthesizedAggregateRoot` has exactly two variants, `HostResultError` and `HostResultOk` | `:4208` |
| `SynthesizedAggregatePath` is `{ root, steps }`, steps being `Field(u32)` / `Alternative(u32)` | `:4241`, `:4225` |
| `SynthesizedConstructorRole` is `Fixed(..) \| IoError(..)` | `semantic_ir.rs:157` |
| `seat` is documented as *"the `Effect` occurrence whose lowering builds this producer"* | `:3999` |
| `FieldIdentity` — an artifact-static identity for a **record field** name — already exists | `semantic_ir.rs` |
| every record must name a distinct producer; this is production code, not a test | `:5790` |

⇒ **The synthesized vocabulary is entirely host-result-shaped.** Two roots, both
host-result arms; roles that name host-result constructors; `Constructor` shape
only; seats that are `Effect` occurrences. **Nothing in it is a unit boundary,
and nothing in it forbids one.**

## The a priori best guess — build this

**Operator ruling, 2026-08-15: state the repair as an attackable claim and
attempt it. One attempt, then hand back. Do not open with a survey.**

> **Extend the synthesized producer arm to name the unit-boundary environment
> record: a new `SynthesizedAggregateRoot` arm rooted at the crossing, `Record`
> admitted as a synthesized shape alongside `Constructor`, and a role naming the
> captured environment. The record `D1` already builds then carries a
> planner-issued occurrence, names a real ownership record, and passes
> `reconcile_source_aggregate` unchanged.**

Three legs are already measured and are why this is the guess rather than a
survey:

1. **The arm exists and is production.** No new concept is introduced; the
   extension is to a closed vocabulary that already distinguishes synthesized
   producers from source ones.
2. **Record fields already have artifact-static identities.** `FieldIdentity`
   exists for exactly this namespace, so a synthesized `Record` is not missing
   its field-naming authority.
3. **`InvocationAggregate` already admits `Record`** (`boundary_value.rs:706-711`).
   The crossing lane was never the obstacle; `D1` never reached it.

## The joint that is NOT measured, and it is the first thing the attempt hits

**Stated plainly because it is the likeliest handback and it is not a defect if
it fires.**

> **Whether the crossing has a `seat` the planner can name.** Today `seat` is an
> `Effect` occurrence, and the path discipline requires *"measured structure that
> both sides state independently and can be checked against each other at
> construction"* — explicitly **not** an ordinal counted in lowering's control
> flow, which the planner does not execute.

**The planner must be able to see the crossing to key a record for it.** If the
unit boundary is visible only inside lowering's traversal, no lawful key exists
and the extension cannot be minted. **Attack that first, in code.** If it
refuses, name the exact mechanism and what a lawful key would require, and stop.

**Do not invent an ordinal to get past it.** The comment at `:4225` states why
that is prohibited, and a key that lets a path name a node it does not reach
while comparing equal to one that does is worse than the refusal.

## Deliverables

**`D0` — the extension, attempted.** The synthesized arm reaches a unit-boundary
environment `Record` with a lawful, non-aliasing key. Re-run
`RT-CLOSURE-CROSSING-ELIMINATE` `D1`'s probe and report exactly where it now
lands.

**`D1` — the disposition.** Either the probe passes
`reconcile_source_aggregate`, or the refusing mechanism is named at its site.
**A recorded refusal with its mechanism is a complete deliverable**, on the same
closure criterion the rest of this campaign uses.

**`D2` — the carrier-word question, answered as a question.** If `D0` passes the
ownership seam, state whether the crossing then reaches the second half — the
non-root unit result exiting as an opaque carrier word
(`lowering/units.rs:6227-6234`) — or whether passing the ownership seam is
sufficient. **A recorded "it now stops here instead" is the answer**, and it is
what decides whether a further node exists.

> **`D2` is a report, not a repair.** The carrier-word half is explicitly not
> this node's work, and widening to chase it is banned below. The whole point of
> this node is to establish which of the two halves is actually load-bearing,
> because the Steward asserted both were and measured neither.

## THE FIRST CANDIDATE WENT RED. MECHANISM AND RECUT CONDITION ARE BOTH GROUNDED.

**Candidate `1b8a57de6` was approved on exact SHA (`dec_6758m1a7g7e55`) and
failed CI.** Base `75a91d2ba` was green, all four shards failed, and the failing
controls are pre-existing and untouched by the diff. Regression, not a flake.
Handback `evt_37ht96vrm9nx4`; PR #2335 closed. **A corrected candidate is a new
SHA and needs a fresh exact-SHA Decision.**

**One narrow signature — rows `row4-depth-2` and `row4-depth-3` only**, every
other row byte-identical:

```
expected:  "refused:Closure"
actual:    Backend(PlannerInvariant(
             "aggregate producer has no planned ownership record"))
```

⇒ **A designed, user-facing refusal became a "please report this compiler bug"
panic.** `missing_call_input_callee_child_degrades_the_tag_not_the_compile` is
named for exactly the property that broke.

### The mechanism, grounded by the Architect at `evt_2p007te58p8y3`

**It is not the absent-key path. It is the path where the substitution
SUCCEEDS.** The substitution replaces a `ConstructorField` holding
`Lowered::Closure` with one holding `Lowered::Record`, **which changes the
value's kind — and a downstream consumer dispatches on kind.** In
`reconcile_source_aggregate`'s child loop (`mod.rs:6937`),
`lowered_aggregate_shape(child)` returns `None` for `Closure` and `Some(Record)`
for `Record` (`mod.rs:7050-7056`). Before the change the substituted child hit
`continue` and was invisible to source-producer reconciliation; after it, the
child enters that lane and resolves through `source_aggregate_occurrence`, which
looks up `AggregateOccurrenceProducer::Source(origin)` **exclusively**. The
occurrence the substitution minted is a **synthesized** one, so no `Source`
record exists for it by construction, and that lookup's documented *"absence is a
loud failure, never a default"* fires — correctly, **on a question it should
never have been asked.**

**The actual defect is an asymmetry.** That same loop already gates the **parent**
on producer class at `mod.rs:6934` (`if planned.producer_origin().is_none() {
continue }`), and its comment gives the general reason: a compiler-synthesized
aggregate's children have no occurrence in the program, and re-deriving agreement
from source origins the planner deliberately recorded as absent would be a
second, weaker authority. **That rationale applies verbatim to a synthesized
child. The parent arm has a producer-class gate; the child arm has only a shape
test.** This change introduced the first value that is **synthesized by producer
yet aggregate by shape**, and that combination is what the child arm cannot
express.

> **One step is inferred, not measured, and the Architect flagged it himself:**
> that the depth-2/3 rows take this specific child path. **Measure it before
> building to it.** If the actual path is a different one, the finding is wrong
> and should be reported as wrong rather than fitted.

**The design is not retracted.** The structural `(producer, position)` key and
the non-aliasing argument are untouched, as is the `41-values.md` reading. Two
repair directions are both in scope and the choice is the ring's: give the child
arm a producer-class gate, or do not present a synthesized record where a
source-lane consumer will shape-dispatch on it.

### Binding condition on the next approval

> **A by-construction argument about the substitution function's own early
> returns will not be accepted again.** The re-approval must trace the **success
> path** to every consumer that dispatches on `Lowered` kind, and show each one
> either handles a synthesized-producer aggregate or is unreachable for one.

**Why this is stated as a condition rather than a lesson:** the original review
named the planner/lowering divergence as a non-blocking flag and dispositioned it
*"both directions fail closed, costing coverage not soundness."* **One direction
does not fail closed.** The soundness half of that call stands — nothing unsound
is admitted — but *"fails closed"* was a reading where a test was owed.

## Acceptance criteria

**`AC-1`.** `D0` attempts the stated claim directly. **A handback reporting that
no lawful `seat`/`path` key can name the crossing, with the mechanism and site,
satisfies this criterion** — a refuted guess is the deliverable when an attempt
refutes it.

**`AC-8`.** **Added after `1b8a57de6` went red.** No pre-existing control
changes disposition. In particular, no row that previously produced a designed
refusal may come to produce a `PlannerInvariant` or any other
report-a-compiler-bug failure. **If a control's expectation genuinely should
change, that is a handback and not an edit** — the four controls that caught this
are owned elsewhere and pin dispositions this node was not licensed to move.

**`AC-2`.** The non-aliasing law holds: every ownership record still names a
distinct producer. **Demonstrate that a new root or role cannot alias an
existing one**, rather than asserting it. This is the law that makes an identity
an identity and it is production code at `:5790`.

**`AC-3`.** The new path is measured structure that lowering and the planner
state independently and check against each other at construction. **No ordinal
counted in lowering's control flow**, for the reason `:4225` gives.

**`AC-4`.** No new `(tag, class)` admission and `BOUNDARY_RETIRED_LANES`
unchanged. **This is a scope boundary on this node, not an architectural
prohibition** — a candidate needing one has left this node's route and should
hand back. See `RT-CLOSURE-CROSSING-ELIMINATE` for why that distinction is
stated explicitly.

**`AC-5`.** The refusal arm still refuses everything it refuses today. If the
extension succeeds, an aggregate with **no** lawful occurrence is still refused
at `:6744` with its current message — the seam is passed by minting authority,
never by relaxing the check.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Designing the cross-unit carrier.** `D2` reports whether the crossing
  reaches it. Building it is not this node and is not authorized here.
- **Undoing the unit boundary.** Refused by the runtime ring previously and it
  stays refused.
- **Retiring `RecursiveDescent`.** [[RT-DESCENT-RETIRE]] is downstream, is a
  product call the operator has not made, and is not this node.
- **Relaxing `reconcile_source_aggregate`.** The refusal is correct. This node
  supplies the authority the check asks for; it does not weaken the check.

## What this node does NOT settle, recorded so it is not overread

**It does not establish that the rows can be repaired.** It establishes which
seam is load-bearing. If `D0` passes and `D2` reports the carrier word as the
next stop, the campaign still has an open question — a **better-located** one
than it has today.

**It does not resolve the operator's fork.** Whether retirement may ship a
narrowing remains open at [[RT-DESCENT-RETIRE]], and whether this node gates it
is the operator's call, which is why no `blocks` edge is asserted here.

**The escape-lifetime sub-shape stays unmeasured.** The Architect's ruling covers
the argument-crossing sub-shape only, and the assumption that two sub-shapes
answer alike has already been wrong once on this campaign.

## Provenance

`RT-CLOSURE-CROSSING-ELIMINATE` `D1`'s handback and its refusal site; Architect
ruling `evt_1ra9asrda1t94` on the live-domain question, routed at
`evt_4t9x8hybvf9pz`. Every table row above was read from the tree at
`6d56a700c` by the Steward; none is taken from a report.
