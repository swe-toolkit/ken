---
id: RT-SYNTHESIZED-ENV-RECORD-OCCURRENCE
title: "Give the unit-boundary environment record a planner-issued occurrence by extending the synthesized producer arm, so the closure crossing is attempted at the seam that actually refused it"
status: merged
owner: runtime
size: M
gate: none
depends_on: [RT-CLOSURE-CROSSING-ELIMINATE]
blocks: []
github: 2352
origin: "Steward, 2026-08-15, on the operator's challenge to substantiate the claim that covering the refused closure-crossing rows requires inventing a representation. It does not, and the claim is withdrawn. Every fixed input below was measured by the Steward at origin/main 6d56a700c before framing. Steward-filed per COORDINATION section 2."
---

> # MERGED 2026-08-15 at `a1c064d5fa2aac810a83516dffb5b16307f8b0ad`, PR #2352
>
> **Candidate exact `4eec77390a84c87db369ea35565d6d2b21e4e8e7`**, three commits
> from merge-base `de551a4ddebeedfb22a93b7f98c0ebd799405ddd`, exactly three
> `ken-runtime` paths, `+567/-71`. Decision `dec_17ma5kt7vbsf0` resolved
> APPROVED (Architect `evt_6ec2r0m9q9jwd`); QA exact approval
> `evt_4czncdaa9t722`. Verified on `main` by blob identity on all three declared
> paths, enumerated from the declared merge-base.
>
> **`dec_391jjtajxhf33` is `rejected`-as-superseded** and names the blocked
> `257a9ddcc`. It is not a verdict on this work.
>
> **`D0`-`D3` are all delivered.** `D3` reported that the governed rows have no
> source-level witness, which voided the product fork rather than answering it.
> `blocks` is empty and no node names this one in `depends_on`, so nothing
> entered the frontier on this merge; the runtime lane continues on
> [[RT-RECURSOR-TRANSPORT]] `D0`-`D2`.

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

**`D3` — ASK THE ORACLE. This is the node's next turn and the whole of it.**
Added 2026-08-15 under the operator's oracle ruling.

> **What does the interpreter do with the program these rows stand for?**

**`row4-depth-2/3` are NOT programs, and a dispatch phrased as if they were is
one level off.** They are in-Rust lowering fixtures —
`host_result_closure_match(px8j_scope_chain_observation_result(2, 0))` at
`lowering/core/tests/control.rs:5688` — handed straight to the backend.
`ken-interp` does not consume that.

The measurement, at the right level:

1. **Find the surface Ken program** whose compilation reaches this shape: a
   closure captured in a scope-chain observation crossing a unit boundary at
   depth 2 and 3.
2. **If none exists, build the smallest one that does**, and confirm it reaches
   the same refusal through the real pipeline rather than through the fixture.
3. **Then ask the interpreter.**

> **Step 2 failing is a result, not an obstacle.** If no surface program can
> reach this shape, these rows have **no source-level witness** — the refusal
> under argument is unreachable by any program a user could write, and the whole
> disposition changes. **Do not manufacture a program that merely resembles the
> fixture** to get past the step.

**Do not derive it** from the lowering path, do not infer it from the
closure-crossing refusal, and do not extend the measurement into a repair in the
same turn. One measurement, reported, then stop.

**Either outcome is a complete deliverable.** "The interpreter refuses them too"
is a real result that changes the disposition, not a failed attempt.

**Why this is stated as a warning and not a method note:** this campaign has
produced five claims restated from prior prose without re-derivation, and every
one was wrong — three of them the Steward's about this node, all in the direction
of *more settled than it is*. **A reading about what the interpreter does is not
a result.**

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

### SUPERSEDED BY MEASUREMENT: there is no key problem underneath this

**The section above was written from a reading. The runtime ring then measured
it, and the measurement is the authority.** Architect withdrawal at
`evt_5h64t36bypwfy`.

**What the ring established:** the synthesized occurrence is **valid**, and every
transfer consumer through emission **accepts it** once the wrong source-only
child lookup is skipped. ⇒ **The absent-key framing is withdrawn.** The defect is
purely the child consumer asking a source-producer question about a synthesized
producer; nothing underneath it is broken, and no key needed inventing.

**What survives unchanged** is the asymmetry — the parent arm gates on producer
class, the child arm tests only shape — and **that gate is the uncontested
repair.**

## `D3` ANSWERED: THE ROWS HAVE NO SOURCE-LEVEL WITNESS. THE FORK IS VOID.

**Measured by the runtime ring at exact `af13cc7e5`** (`evt_qraaq4ytjxx1`,
routed `evt_10e5re104zn4h`). Measurement only — no candidate, no commit, scratch
test deleted, worktree clean.

**The interpreter was never reached, and that is the result.** `D3`'s step 2 was
written to say that failing to find a surface program is itself a finding. It
failed, and here is what it found:

- The governed producer is **test-only hand-authored `RuntimeExpr`**
  (`lowering/core/tests/control.rs:2358`).
- **Both exact surface routes to that shape refuse before checked-artifact
  emission.** Naming the W-style recursive result gives
  `Elaboration(StructuralResultOutOfScope)`; recomputing it by self-call gives
  `KernelRejected(NotTerminating("SCT: idempotent self-loop has no
  strictly-decreasing parameter"))`.
- This **agrees with the operative surface rule**,
  `spec/30-surface/34-data-match.md:443-445`: the W-style result exists as a
  Pi-abstracted kernel IH and **no surface selector exposes it.**
- `Node k` would transport the source closure rather than the recursive result.
  **The ring correctly declined to substitute a merely similar program.**

### The disposition, and it retires the fork rather than answering it

**Both branches of `evt_3yvhf3hz59eb8` presupposed a capability question that
does not exist.** There is no widening — no source program reaches these rows, so
advancing them adds nothing to the accepted language. There is no narrowing —
nothing compiles to them from source, so no capability is lost. **A fork whose
two options are both about unreachable rows is void, not close.**

**The child producer-class gate is authorized to land as a compiler-internal
correction.** No operator decision is owed for these rows.

> ### TWO CONDITIONS, and the first is where this could still go wrong
>
> **1. The gate is GENERAL; the measurement was NOT.** `D3` established that
> `row4-depth-2/3` have no source witness. It established **nothing** about
> whether some other shape reaching the same child arm does. **The recut must
> establish that no source-reachable shape changes disposition** — measured, not
> argued from the fixture's unreachability.
>
> **2. `AC-8` is amended for these rows only.** It says no pre-existing control
> may change disposition and that a control whose expectation should change is a
> handback rather than an edit. **This is that handback, and it is granted:**
> `row4-depth-2/3` may move from `refused:Closure` to their new disposition,
> because they pin a shape **no compiler input in the measured corpus
> produces** — and that corpus excludes the population by its own selection
> rule, per the correction below. **State the new expectation
> and why the control is still worth keeping** — a control over IR unreachable
> from source is pinning the lowering's internal contract, which is a real thing
> to pin and a different claim from what its name suggests.

### The finding underneath, recorded because it outlives this node

**CORRECTED after the merge — this said "not reachable from source" and the
evidence does not carry that.** See the corpus-exclusion correction below: the
measurement's own selection rule removes the closure-at-boundary tests, so
"unreachable from source" is not a thing it can establish. The defensible form
is **"no witness in the measured corpus,"** and the measured corpus is the one
that excludes this population.

**The node's only production witness is absent from the measured corpus.** The
ring established earlier that these crossings were the node's live path; `D3`
establishes that **no program in that corpus** reaches them. **Those two facts
together mean the mechanism this node builds is exercised, today, only by
hand-authored IR** — which is worth knowing before the next node in this
campaign is sized against it. **Whether a program outside the corpus reaches
them is open**, and it is the successor's whole subject.

## THIS CANDIDATE CARRIES TWO MECHANISMS AND THE FRAME SCOPES ONE

**Architect finding in the `evt_24sn918ngybnc` audit ruling, verified against
the objects by the Steward. Recorded here because it is a scope fact and the
scope is the Steward's.**

```
38cbba9c1  17:22  3 files  +480/-35   static_transition.rs +353   ANCESTOR
257a9ddcc  20:46  2 files   +90/-49   mod.rs +23                  the recut
```

**`38cbba9c1` predates both the `D3` disposition (19:04) and the recut dispatch
(20:13).** The substitution mechanism — a source-derived population plus a
lowering substitution that removes a closure child before
`boundary_transfer_admissibility` runs — is **carried forward from the
pre-recut candidate**, and this node's frame is about the child arm.

⇒ **Nobody has been assigned to measure that population, because it is not what
this node is about, and it has now ridden through two review rounds under a
frame that does not scope it.**

### The disposition: no split yet, and the rule that decides it later

**The Architect's block is an existential over a bounded set** — instrument
`unit_boundary_environment_fields` and report whether it returns anything
non-empty for any Ken-source input in the existing suite. One run; he expects
comment-only. **Restructuring on the prediction that a bounded measurement will
turn expensive is the same error as folding a node on an unmeasured
prediction.**

| the measurement returns | disposition |
|---|---|
| empty or cheap | **land as-is.** The carried-forward mechanism is scoped retroactively here, not re-cut |
| a real population | **split: land the producer-class gate, take the substitution to its own node with the population as its subject** |

**The split is NOT "cut back to `D3` plus the `AC-8` handback"** — that would
discard the child-arm gate, which is proved on both mutation legs and is this
node's whole subject.

**Two things unmeasured before any split:** whether the gate is exercised at all
without the substitution (the row4 controls need it to produce a `Record` at
that arm, so the gate may be inert alone), and what the population is.

### THE MEASUREMENT THAT SELECTED "EMPTY" CANNOT SEE THE POPULATION IT BOUNDS

**Adversary hunt `evt_71wmpee00vt3j` on the merged range
`de551a4dd..4eec77390`, verified against the tree by the Steward. This is a
correction to the row above, and it lands after the merge.**

The corpus is the **non-ignored** `ken-cli --tests` paths.
`crates/ken-cli/tests` carries 33 `#[ignore]` attributes, and **six of them
name this exact condition**, verbatim:

```
#[ignore = "RT-CLOSURE-BOUNDARY-LANE: a runtime-local closure has no
            durable lane across the boundary; fails at base 21fd46dc"]
```

in `px8l_recursive_decl_native.rs` (two), `rt_escape_second_resource_native.rs`
(two), `px8ta_oriented_subcontinuation.rs`, and `rt_parity_native.rs`.

⇒ **The sample is "programs that currently compile," and the mechanism exists
to change what compiles.** The exclusion criterion is the negation of the thing
being measured, so **the empty set is guaranteed by construction rather than
observed.** The 7-to-301 occurrence range is genuine anti-vacuity evidence that
plans were *populated*; it is not evidence the corpus *could* contain the shape.

**This does not claim the six would show a non-empty set** — whether any
produces a directly carried empty lexical environment with a planner-issued
synthesized record is what the measurement would have to determine, and cannot,
because they sit outside it. The claim is narrower and harder to dispute: **the
corpus is structurally incapable of answering the question it was used to
answer.**

**Which half of the merge argument this costs.** The fail-safe direction stands
on its own and was independently traced — every absence returns the value
unchanged, the producer-class gate refuses outright when a child has no producer
occurrence, and the `continue` skips only the source-only per-position lookup
with the generic ownership record checked before it. **The empty-population half
is the one carrying the weight, and it is the one that does not hold**, because
"defeats a refusal" is tolerable only if nothing reaches it.

**The landed comment is narrower than the Steward's broadcast of it, and the
overread was the Steward's.** The comment says "non-ignored" and says in terms
that this is a scoped corpus measurement and not a universal property. What it
does not say is that the excluded set is precisely the population of interest —
a reader takes "non-ignored" as incidental scoping. **The Steward's merge
notification said "the source-reachable population is measured empty," which is
the sentence a later reader would cite to conclude this seam was cleared.**
Corrected at `evt_5hremk2yx49kc`.

⇒ Successor: [[RT-BOUNDARY-IGNORED-CORPUS-MEASURE]].

### `COORDINATION §8a`'S PREFIX RULE SELECTS THE WRONG HALF HERE

> *"Prefer a cut that is a straight ancestor of the working tip"* — because a
> contiguous prefix preserves every exact SHA and every verdict below it.

**That rule assumes the proved, cheap work is the ancestor and the risky work
sits on top. This candidate is inverted.** The prefix is the expensive
unmeasured mechanism; the cheap proved fix is the descendant. **Taking the
prefix would land precisely what nobody has measured and drop precisely what is
proved.**

⇒ **The prefix rule is a heuristic about verdict preservation, not about risk
ordering. Check which half carries the unmeasured work before invoking it.** A
split here is a real re-authoring and must be priced as one, not as a prefix.

## Superseded framing: the oracle ruling that produced `D3`

**Operator ruling, verbatim:**

> `RecursiveDescent` should not be taken as de facto spec. It was a failed
> implementation attempt that needs to be replaced. The key oracle is not
> `RecursiveDescent`, but the interpreter.

**This dissolves the joined fork `evt_3yvhf3hz59eb8` rather than answering a side
of it.** The section below is retained because its analysis of the two options is
still correct; what changed is the baseline both options were measured against.

**The narrowing half is withdrawn.** It asked whether retirement may ship a
capability loss measured against `RecursiveDescent`. Parity with `RecursiveDescent`
is not a requirement, because its accepted set was never the specification.

**The widening half becomes a measurement.** "Wider" must be measured against the
interpreter, and it never was. See `D3`.

> ### THE SCOPE CALL, WHICH IS THE STEWARD'S AND IS FLAGGED AS SUCH
>
> **If the interpreter accepts the `row4-depth-2/3` programs, the child
> producer-class gate lands without a further operator decision.** A compiler
> refusing what the oracle runs is a compiler defect, and closing it is
> convergence, not a widening of the accepted language.
>
> **If the interpreter refuses them, the refusal is a real language property** and
> the disposition returns to the Steward. Do not assume which; `D3` measures it.

## The analysis that produced the fork, retained (baseline now superseded)

**Applying the uncontested child gate advances `row4-depth-2/3` from refusal into
compilation.** That is a **widening of the accepted language**, and by the
Architect's `41-values.md §2.1` ruling it is right on the merits — a
compiler-minted unit-boundary environment has no code identity, ABI, export, or
serializable form, so *"a closure cannot cross the boundary"* is not a true
sentence about it, and the refusal is a **transition sentinel that has reached
its boundary** rather than a durable invariant.

**But a widening is the operator's call, not the Architect's and not the
Steward's.**

| option | why it is not available here |
|---|---|
| **advance the rows** | pre-empts the operator's decision |
| **withhold the substitution from these rows** | **empties the node's only production witness.** By the ring's own measurement these crossings are it. The node would ship a planner record nothing consumes, with ACs discharged by controls that have no live path behind them |

> **Option 2 is not the conservative choice, and that is the part most likely to
> be misread.** It is a scope cut wearing caution, and it is worse than the
> widening because it is **invisible afterwards**: a green node whose mechanism is
> unexercised reads as done.

**Routed to the operator as ONE joined question with the `RecursiveDescent`
fork** (`evt_3yvhf3hz59eb8`) — the same product surface, narrowing from one side
and widening from the other. **ANSWERED 2026-08-15 by the oracle ruling above.**

**The recut is now gated on `D3`'s measurement, not on an operator decision.**
Both options above still foreclose each other, so neither is taken until the
interpreter's behaviour is known.

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
- **Retiring `RecursiveDescent`.** [[RT-DESCENT-RETIRE]] is downstream and is not
  this node. **The clause "a product call the operator has not made" is struck**:
  the operator ruled 2026-08-15 that `RecursiveDescent` is a failed implementation
  attempt that needs to be replaced. The direction is settled; the sequencing is
  still not this node's.
- **Relaxing `reconcile_source_aggregate`.** The refusal is correct. This node
  supplies the authority the check asks for; it does not weaken the check.

## What this node does NOT settle, recorded so it is not overread

**It does not establish that the rows can be repaired.** It establishes which
seam is load-bearing. If `D0` passes and `D2` reports the carrier word as the
next stop, the campaign still has an open question — a **better-located** one
than it has today.

**The operator's fork is resolved and this sentence used to say otherwise.**
It read *"whether retirement may ship a narrowing remains open at
[[RT-DESCENT-RETIRE]]"*. **That question no longer exists** — narrowing was
defined against `RecursiveDescent`, which the operator ruled is not the oracle.
No `blocks` edge is asserted here, and the reason is now sequencing rather than a
pending product call.

**The escape-lifetime sub-shape stays unmeasured.** The Architect's ruling covers
the argument-crossing sub-shape only, and the assumption that two sub-shapes
answer alike has already been wrong once on this campaign.

## Provenance

`RT-CLOSURE-CROSSING-ELIMINATE` `D1`'s handback and its refusal site; Architect
ruling `evt_1ra9asrda1t94` on the live-domain question, routed at
`evt_4t9x8hybvf9pz`. Every table row above was read from the tree at
`6d56a700c` by the Steward; none is taken from a report.
