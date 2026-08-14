---
id: RT-CONTKEY-CONSUMER-DESCENT-CARRY
title: "the consuming-occurrence relation is complete at depth 1 and absent below it because the descent clones the source seed forward unchanged; the value each level needs is the one derived at the level ABOVE it, so carry it -- seeded from the source relation at the top and advanced by target-derivation at each push, attributed to the CHILD"
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-CONTKEY-CONSUMING-OCCURRENCE]
blocks: [RT-LEXICAL-RECURSOR-CONSUMERS]
github: null
origin: "Architect mechanism ruling evt_6td3bs6j6g14m (route (c), the D5a precedent) plus the probe-correction ruling evt_56dvtaft7ep38, both transcribed in full into docs/program/wp/RT-LEXICAL-RECURSOR-CONSUMERS-D2k.md. The bounded probe was run by runtime at evt_76cmre0qvsmmd and returned No; the Steward raised the raw pairs at evt_3n4e0hs0gy8xm and the Architect reversed his own route selection, the No having been produced by an off-by-one in his probe specification. Filed as a new cut rather than an increment on D2k because D2k's banned scope excludes the planning surface this changes -- the same reason RT-CONTKEY-CONSUMING-OCCURRENCE was cut out of it. Steward-filed per COORDINATION §2."
---

## What this is

**The landed relation is right and it is complete at depth 1. Below depth 1 it
is absent, and the absence is structural rather than a bug.** The Architect
established both halves at `evt_6td3bs6j6g14m`: `consuming_occurrence` is
**source-keyed**, minted by `initial_continuation_discoveries` onto the
position-zero child of an outer `ComputationalMatch`, and at depth 2 or 3 the
consumer is determined by **which specialization realized the body** -- a fact
about generated structure that no source-minted relation can name **in
principle**.

**This node supplies the missing input. It does not claim the route then
closes** -- see *Excluded scope*, which is the part of this frame most likely
to be violated by accident.

## The correction that selects this shape, and why the probe said No

The bounded probe asked whether the consumer identity a refusing boundary
requires equals the one derivable at **that same** push. It returned **No**, and
the No was correct. **The question was one level off.** Runtime reported the raw
pairs rather than only the verdict, which is the only reason this was
recoverable:

| row 4 | derived at this push | required by the next worker-bearing boundary |
|---|---|---|
| depth 2 | `(body 26, eliminator 21)` | `(body 16, eliminator 5)` |
| depth 3 | `(body 36, eliminator 31)` | `(body 26, eliminator 21)` |

**`(26,21)` appears in both columns, one row apart.** The Architect ruled at
`evt_56dvtaft7ep38`:

> **`required(N)` = the consumer established at level `N-1`.**

**Why that is structural and not a coincidence over two points: the descent goes
inward and the consumer is outward.** A constructor produced inside a worker
body is consumed by the enclosing context, so the consumer needed *inside* level
`N` is the one established *at* level `N-1`. **A carrier that appears to hold
"the next level's identity" is a carrier holding the correct identity, read at
the wrong level.**

**The two ends come from different sources, and that difference is the design
statement rather than noise.** Depth 2's requirement `(16,5)` matches the
**seed** relation -- it is what the landed control's `row4-depth-1` observes.
Depth 3's requirement matches the **target-derived** one. So the carry is
**seeded** from the source relation at the top and **advanced by
target-derivation at each push**.

## Fixed inputs, measured at `main` `15c21269`

Re-derive them at your base; a merge-base goes stale without your branch moving.
Every site is
`crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs` unless
named otherwise.

**The carrier struct, `:8105`.** Four fields. `enclosing_specialization`
(`:8122`) is the `D5a` precedent this node follows, and its doc comment states
the precedent's reason in terms: the fixed point used to descend carrying only
the two origins, so the next iteration re-read the raw occurrence owner and lost
the specialization that had selected and invoked that worker.

```rust
struct ContinuationDiscovery {
    continuation_origin: StaticOriginId,
    result_root: StaticOriginId,
    enclosing_specialization: Option<ContinuationSpecializationId>,
    consuming_occurrences: Option<ContinuationConsumingOccurrenceSeeds>,
}
```

**The seed mint, `:10781`-`:10819`.** `initial_continuation_discoveries` records
the outer eliminator and each ordinal-selected case body at the one moment both
are in hand, then pushes the seeds onto **only** the direct position-zero child
(`:10816`). Its own comment: *"Every case body is walked independently with no
inherited parent relation."*

**The descent push, `:11289`.** This is the site the ruling names, and it is
where `target` is already in hand:

```rust
pending.push(ContinuationDiscovery {
    continuation_origin: discovery.continuation_origin,
    result_root: worker.body_origin,
    enclosing_specialization: Some(target),
    consuming_occurrences: discovery.consuming_occurrences.clone(),
});
```

⇒ **`continuation_origin` is held FIXED across the descent and the seeds are
cloned forward unchanged.** That is why `consuming_occurrence_from_seed`
(`:10854`), which reads `discovery.continuation_origin` and the cloned
candidates, cannot produce a level-appropriate answer below depth 1. **The
clone is not the defect; it is the absence of any advancing term beside it.**

**Where the value is derived today, `:11224`.** The key built at `:11215` takes
`consuming_occurrence: consuming_occurrence_from_seed(plan, &discovery,
alternative)?`, and `intern_specialization` at `:11243` returns `target`.

**The landed validator, `:10949`.** `validate_continuation_consuming_occurrences`
reads `unit.key.consuming_occurrence` and re-derives it through
`rederive_consuming_occurrence` (`:10909`), whose first act is
`forward_match_scrutinee(claimed.eliminator_origin) != key.continuation_origin
=> Ok(None)`. **That derivation is source-keyed by construction.** It is the
independent half of the depth-1 relation and this node does not touch it -- see
*Excluded scope*.

**The landed control,
`crates/ken-runtime/src/cranelift_backend/lowering/core/tests/control.rs:5702`,**
`contkey_rows_four_and_five_carry_the_exact_outer_consuming_occurrence`. It
asserts, per row, `count == 1` with the message *"exactly one specialization
edge must carry the outer consuming relation"*, over the fixtures
`host_result_closure_match(px8j_scope_chain_observation_result(1, 0))` (labelled
`row4-depth-1`) and `px8j_equal_payload_hole_placement(AfterReturnHole)`
(`row5-after-hole`).

## Deliverables

**`D1` -- carry the consumer on the discovery, attributed to the child.** Add a
field to `ContinuationDiscovery` holding *"the consumer my producers must
use"*, **beside** `consuming_occurrences` and never repurposing it. Set it at
`:11289` from the value derived at **that** push, so a discovery at depth `N`
already holds `required(N)`. Seed it at the top from the source relation.

**This is the `D5a` shape exactly** -- same struct, same push site, same reason
`enclosing_specialization` was added there.

**`D2` -- state and measure what the advancing term is.** At the first push the
value is the seed selection; at later pushes the seed can no longer name it, so
the advance must be a target-derivation using `target` and `worker`. **The probe
you ran already computed this value** -- the `derived at this push` column above
is its output. `D2` re-instates that derivation as production code at the same
site.

**`D3` -- decide, in one written paragraph, whether the carried value reaches
`ContinuationSpecializationKey`, and measure the interning consequence if it
does.** Widening the key changes what `intern_specialization` (`:11243`) treats
as one unit. Report the interned unit count on both governed fixtures before and
after. **A population that moves without a control that explains the move is a
stop condition, not something to absorb.**

**`D4` -- the control that would go red if the carry were wrong.** At minimum:
on the fixture that exhibits row 4 at depths 2 and 3, assert the carried value
at each depth **equals the required pair measured above**, by identity, not by
"is `Some`". A carrier asserted only to be populated is not pinned.

**`D5` -- reconcile the landed control's `count == 1`.** Name which fixture
exhibits depths 2 and 3. If it is `row4-depth-1`, that count moves and the new
number is evidence the carry reached the levels; update it and say so. If it is
a different fixture, `count == 1` stands and `D4`'s control is separate. **Either
way state which, with the measured number.**

> **A control that passes unchanged after your change means your change did
> nothing.** If every existing assertion holds at its existing value and no new
> number moved, that is the finding and it outranks a green run.

## Acceptance criteria

**`AC-1` -- at row 4 depths 2 and 3 the carried consumer equals the required
pair, asserted by identity.** Report the raw values, not only the verdict. That
practice is what made the probe's wrong branch recoverable and it is expected
again here.

**`AC-2` -- depth 1 is unchanged.** The seed relation, its selection, and
`rederive_consuming_occurrence`'s verdict on it are bit-identical. The landed
`row4-depth-1` and `row5-after-hole` assertions still hold on the same
derivation.

**`AC-3` -- the new carrier has either a control that reds when it is wrong, or
a written sentence saying it is unvalidated at this base and why.** The second is
an acceptable outcome. An unvalidated carrier with neither is not.

**`AC-4` -- no `BodyEmissionDisposition::ContinuationTemplate` population change
and no continuation-source surface touched.** These are the two exclusions the
Architect attached to route (c) at `evt_6td3bs6j6g14m`, and they are the reason
(c) was preferred over (b).

**`AC-5` -- no-regression, in CI.** `COORDINATION §12` -- the venue is CI, never
a local `--workspace` run. Build and test targeted, `-p ken-runtime`.

**There is deliberately no AC about the refusal at the next boundary.** Writing
one would assume exactly what nobody has measured. See below.

## Excluded scope

> ### THAT SUPPLYING THE RELATION CLOSES THE ROUTE IS NOT ESTABLISHED
>
> Architect, `evt_56dvtaft7ep38`, in substance: the original stop reported a
> **further refusal at the next `Closure`/static-worker boundary** and a
> **second recognition retained in the standalone definition**. **(c) supplies a
> missing input; whether the route then closes is a separate measurement.**
>
> ⇒ **Frame, build and accept this node as "carry the consumer to the level that
> needs it" and nothing more.** If the route does close, that is a welcome
> observation to report -- it is not this node's acceptance criterion, and it
> does not license widening the increment to chase whatever refuses next.

- **Not the depth-1 relation, its key field, its threading, or
  `rederive_consuming_occurrence`'s logic.** Those are the landed
  `RT-CONTKEY-CONSUMING-OCCURRENCE` mechanism and they are not in question.
- **Not the refusal-message profile.** The unnamed-cause defects in
  `consuming_occurrence_from_seed` and `rederive_consuming_occurrence` are
  `H3`/`H4` of `RT-CONTKEY-REFUSAL-PROFILE-SPLIT`. **Do not fix a diagnostic
  here**, even one you trip over while debugging -- say which one you tripped
  over and it goes into that node.
- **Not row 1.** The pattern is established for **row 4 at three levels**,
  consistent with row 5's single level, and is **not established for row 1** --
  a different class, correctly left unprobed. Row 1's three-way `None` split has
  **left this chain entirely**: it is `H4` of `RT-CONTKEY-REFUSAL-PROFILE-SPLIT`,
  on the Steward's WP cut, accepted by the Architect at `evt_56dvtaft7ep38` and
  grouped by defect class rather than by which node noticed it. **No blocking
  dependency either way.**
- **Not route (b).** (b) was selected by the off-by-one and is not the remaining
  honest class. **Disregard the runtime-leader handback's closing line naming
  it** -- that line faithfully applied the fork as the Architect specified it,
  and the specification was wrong.

## Stop conditions -- return to the Steward, do not decide

1. **The advancing derivation is not available at the push.** If `target` and
   `worker` do not yield the level's consumer without reaching into a surface
   `AC-4` excludes, stop and report the exact site and what is missing. **Do not
   substitute a lookup, a reverse search, or a second traversal.**
2. **Widening the key moves the interned population in a way no control
   explains** (`D3`).
3. **Depth 1 changes.** Any movement in the landed relation is a mechanism
   question for the Architect, not a repair to absorb.

## Sizing

**`S`.** One field on an existing struct, one seed, one advance at an existing
push site, and the controls. **The Architect declined to size it** and his
reason stands: a forward walk existing somewhere in the file does not establish
that this site sits inside one. **If `D2`'s derivation turns into an
investigation, that is the hard stop and it is a good outcome** -- report it and
the node is re-cut, exactly as this node was cut from `D2k`'s stop.

**Sequencing: this node runs BEFORE
[[RT-CONTKEY-REFUSAL-PROFILE-SPLIT]].** They share `static_transition.rs` and
must not run concurrently. This order is not arbitrary: the Architect ruled that
the split's new refusal variant *"must be named for what it OBSERVES, since if
the successor lands the absence stops being structural."* **This is that
successor** -- landing it first lets the split name reality rather than a
prediction. It is also the operator's standing priority: `#6d` closure gates
[[RT-RECURSOR-TRANSPORT]] `D3`, which gates [[RT-DESCENT-RETIRE]].
