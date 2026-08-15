---
id: RT-PLANNED-CLOSURE-PREEXISTENCE
title: "Suppression cannot answer closure pre-existence because it removes the observation point along with the crossing -- ask the PLANNER instead: does the planned occurrence at origin 5 carry a closure-typed field 0 by construction?"
status: merged
owner: runtime
size: S
gate: none
depends_on: [RT-CROSSING-CALLEE-IDENTITY]
blocks: [RT-CLOSURE-BOUNDARY-LANE]
github: https://github.com/swe-toolkit/ken/pull/2317
origin: Architect, 2026-08-15, resolving dec_6hwh86vdzp2ha on RT-CROSSING-CALLEE-IDENTITY. He named the increment and the reason the previous instrument could not work -- "STOP ASKING THE RUN, ASK THE PLANNER". Steward-filed (agents cannot create tracked work per COORDINATION section 2).
---

> # MERGED 2026-08-15 — PR #2317, squash `dfdc9c153`. THE FORK IS CLOSED.
>
> Exact `1cd9947cf23530b9514eb27f9397a31fcfde41ee` from declared merge-base
> `ad47054a5`; one non-merge commit, six `ken-runtime` paths, `+429/-24`, no
> spec/conformance paths. M6 blob identity **6/6 MATCH** from the declared
> merge-base, path count equal to declared scope. Decision `dec_55a8p5m2f8gzk`
> resolved APPROVE — Architect `evt_20qbjwdb1aahn`, QA `evt_791d1g6cgd9b9`.
>
> **The answer is the one this chain spent four nodes failing to get, and it came
> from changing the instrument rather than sharpening it.** Both governed plans
> give field 0 of `StaticOriginId(12)` a **closed** producer set of exactly
> `[(StaticOriginId(12), LexicalClosure)]`. ⇒ **The source program authored the
> closure. The realization did not manufacture it. Branch 1 is REFUTED and the
> durable-lane branch is selected.**
>
> **`D1` is trustworthy because it did three things**, and they are worth naming
> for the next plan-side measurement: it **reused the planner's existing** closed
> result-producer analysis instead of standing up a parallel one; it **surfaces
> `closed`**, so a classification cannot silently range over a partial producer
> set; and it returns the **whole producer vector**, not a verdict. Then it
> earned the answer — mutating the **real source field** to an integer flips the
> classification `LexicalClosure → Other`. **Measured discriminating power, not
> an argued shape.**
>
> **The Architect's provisional qualifier is retired in the honest direction.**
> Had *"branch 1 selected"* stood as written on the predecessor, the next node
> would have been cut at `realize_required_consumer_locally`'s value production —
> **and the value was never the defect.**
>
> **His "do not fold" is superseded by him, explicitly:** *"That reasoning still
> holds; the EVIDENCE changed."* The rows now carry a measured positive property,
> so routing them into [[RT-CLOSURE-BOUNDARY-LANE]] is warranted where it
> previously was not. **Nobody should read the earlier ruling as still blocking
> the fold.**
>
> **`D3` and `D4` both landed, and `D3` answered the question behind the finding
> rather than the finding.** A 14-binding census with a stated population
> predicate: 11 are mutation/control machinery whose refusal **is** the intended
> behaviour, three were diagnostic-only and repaired, and the five production
> `child_static_origin(…)?` calls are excluded on a clean criterion.
> `generated_unit_call_body_callee` now degrades to `MissingBodyChild { entry }`
> instead of returning early, **proved** by a mutation that preserves the exact
> `Closure` compile outcome while flipping the tag.
>
> ## WHAT THIS DOES NOT ESTABLISH — the receiving node needs it said
>
> **It measures these two rows' class. It does not measure the escape rows.** So
> *"one population"* is still one step ahead of the evidence, and the sub-shapes
> differ in **how** the closure must survive: the original rows are a closure
> **outliving its frame**, these are a closure crossing as a **call argument at a
> unit boundary**. Both need a representable closure, so **pooling them for
> sizing is right — assuming one mechanism is not.**

> # THE FORK IS DOWN TO TWO BRANCHES AND THIS NODE DECIDES WHICH
>
> [[RT-CROSSING-CALLEE-IDENTITY]] **closed branch 3'** by proof, not by label:
> `expected_source_callee` recovers the origin from the plan, destructures it as
> `RuntimeExpr::Match`, and asserts `default.message == "direct HostResult
> default"`. With caller `SourceLexicalClosureArgument`, the delivery is
> **source-program-authored** — the crossing is a call the source program itself
> makes, not the realization exporting through an ABI it promised to avoid.
>
> **Closing 3' does not select branch 1.** Two branches remain, and they are
> separated by exactly one unmeasured property:

| branch | claim | where the repair goes |
|---|---|---|
| **(1)** | the realization produces a **closure-shaped value** into an intended call-input route | a lowering fix in this chain |
| **(durable-lane)** | the source value **legitimately** carries a closure through a source-authored call, and Ken has no lane for it | [[RT-CLOSURE-BOUNDARY-LANE]]'s mechanism, and the recursor rows become its rows |

**Both are consistent with every row measured so far.**

## Do not reach for suppression again. It cannot answer this.

**Twice now the differential has failed to settle pre-existence, and the reason
is structural rather than a matter of instrument quality.** Without the
projection these rows never build the subgraph at all, so on the suppressed leg
there is **no observation point** — `closure_path` is computed only at the
crossing, and the crossing is what suppression removes.

> **A Steward ruling was withdrawn over exactly this**, and it is recorded in
> both [[RT-REQUIRED-CONSUMER-REACH-CENSUS]] and [[RT-CLOSURE-BOUNDARY-LANE]].
> The claim was that the durable-lane branch was excluded because its antecedent
> required `crossing_reached = true` **under suppression** and the measurement
> read `false`. **Suppression makes that reading `false` in every possible
> world**, so the conjunct discriminates nothing. **Any argument of the form
> "the suppressed leg shows X" needs this test applied to it before it is used.**

**The question has an observation point on the planning side.** It is a *static*
property of the plan, available without executing the projected route.

## Deliverables

**`D1` — ask the planner, not the run.** Determine whether the **planned**
occurrence at `StaticOriginId(5)` carries a **closure-typed field 0 by
construction**. This is a property of the planned occurrence, so it is readable
without running the projected route and without suppressing anything. Same
`#[cfg(test)]` discipline and the same transition-sentinel promise class as the
predecessors.

**`D2` — decide the branch, per row, and route the rows.** The Architect's
disposition, stated in advance so this is a measurement and not a judgement
call:

| planner reads | then | branch | the recursor rows |
|---|---|---|---|
| field 0 is closure-typed **by construction** | the closure **pre-exists** | **durable-lane** | are [[RT-CLOSURE-BOUNDARY-LANE]]'s |
| otherwise, **while lowering produced `Lowered::Closure`** | the realization **manufactured** the shape | **1** | are this chain's, and that node is sized on its own row |

**State which row reads which**, and say plainly what it means for
`RT-CLOSURE-BOUNDARY-LANE`'s scope and sizing. That node currently carries a
retraction and an open sizing question that only this measurement closes.

**`D3` — a diagnostic that can change the outcome it exists to record.**
Architect non-blocking finding on `dec_6hwh86vdzp2ha`, carried here because it
landed in the code this chain just merged. Three sites bind a `cfg(test)` local
with `?`, all `child_static_origin(..., 0)?`. **In the test profile that `?` is
an early return production never executes**, so the test profile can refuse a
compile that production accepts — **in a chain whose entire output is which
refusal each row reached.**

> **The three named sites, verified on the candidate `9ba3950af`.** The `?`
> calls are `core.rs:17993`, `:18356` and `:18442`. The Architect cited
> `:17990`, `:18352` and `:18438` — those are the **starts of the same three
> constructs**, not the `?` itself.
>
> **`child_static_origin(...)?` appears at five further sites** — `:4047`,
> `:15222`, `:15240`, `:15565`, `:15712` — and **none is `cfg(test)`-gated**, so
> `?` is correct there.
>
> > ### THOSE THREE ARE NOT THE POPULATION. RE-CENSUS BEFORE YOU WORK THE LIST.
> >
> > **Adversary `evt_161q9d2281s20`, and he is right about my error.** The
> > Architect's finding is keyed on the **name** `child_static_origin(..., 0)?`.
> > **The mechanism is wider: any `#[cfg(test)]` binding that can alter control
> > flow in the test profile.** Measured on `main` — `core.rs` alone carries
> > **14** `#[cfg(test)]` bindings using `?`, against the three this frame
> > originally named.
> >
> > **The enumeration above made this worse rather than better.** By pinning
> > three exact lines and then ruling five others out of scope, it reads as a
> > complete census. **It is a complete census of a name and not of the
> > mechanism**, and a frame that looks exhaustive is what stops the next reader
> > checking. **Derive `D3`'s population from the mechanism at `D3` time.**

It fails **loudly** rather than silently, so severity is low and this is not a
reason to hold anything. **The cheap form is strictly better: let a missing
child degrade the TAG, not the COMPILE**, so the diagnostic cannot alter the
outcome it exists to record.

**`D4` — the `callee` field you are about to read is corrupted under an existing
mutation, at exactly one of its six sites.** Adversary `evt_161q9d2281s20`,
verified on `main` at `mod.rs:7631`.

`carry_source_call_inputs` shadows its own coordinate and then passes **the same
shadowed binding twice** — once as the transfer origin, once as `callee`:

```rust
#[cfg(test)]
let origin = self.call_input_transfer_origin_under_mutation(origin)?;  // shadows
for input in inputs {
    carried.push(self.carry_call_input(
        builder, origin, input,
        #[cfg(test)] GeneratedUnitCallInputCaller::SourceMachineDeclaredUnit,
        #[cfg(test)] origin,        // the SAME shadowed binding, as `callee`
    )?);
}
```

**The mutation is a substitution, not a flag** — it returns
`root_static_origin()`. ⇒ **When the `CallInputTransferOrigin`
mutation is armed, the recorded `callee` is the program root.** The mutation
moves the transfer coordinate **and the callee identity together**.

**`D7`'s own comment, three lines above, now states something false of its own
seam:** *"Same call, same arguments, same moment, **two axes**."* The new field
is a **third** consumer of that variable, and that sentence is what a reader uses
to decide the mutation is attributable.

**Severity: latent, not live — which is why nothing was held for it.**
`SourceMachineDeclaredUnit` is asserted in **zero** controls; the only variant
any control asserts is `SourceLexicalClosureArgument` (`control.rs:6332`,
`:6356`), verified. **Nothing reads the corrupted value today. It is a trap for
whoever reads `callee` next — which is this node.**

**Second symptom, same root: the field holds two different levels.** The other
five sites derive `child_static_origin(..., 0)` — the callee's **body**. The
sixth passes `origin`, the callee's **scheduling entry**. Pre-mutation that is
defensible, but it is one level up, so **a control comparing `callee` across
callers reads a disagreement where there is none.**

**Both symptoms have one cause: the sixth site reuses a binding instead of
deriving one.** Direction, not a prescription — derive it like the other five
from the **pre-mutation** origin, captured before the `D7` line. **Check whether
a child at ordinal 0 exists at that plan node**; if it does not, the entry origin
is the right value and the fix is only to take it before the mutation, plus a
clause saying this variant's `callee` is an entry rather than a body.

## The live stop

**If the planner-side property does not separate the branches either, say so and
stop.** Three instruments will then have failed on one question, and the honest
conclusion is that pre-existence is not decidable from this chain — which is
itself the finding, and it routes to the Architect rather than to a fourth
measurement. **Do not force a selection to close a fork.**

## Acceptance criteria

**`AC-1`.** `D1` reads a **planner/plan-side** property. **A measurement taken
at or after the crossing fails this**, whatever it reports — that is the
observation point this node exists to avoid.

**`AC-2`.** `D2` selects branch 1 or durable-lane **per row**, against the table
above, and states the consequence for `RT-CLOSURE-BOUNDARY-LANE`'s scope.
**Reporting the planner property without routing the rows fails this.**

**`AC-3`.** No suppression differential is used as evidence for pre-existence.
Re-running one for another purpose is fine; **citing its suppressed leg as
evidence about the closure is not**, and the retraction above is why.

**`AC-4`.** `D3`'s three sites no longer let a missing child change the compile
outcome. **Demonstrated** — show the missing-child path degrading the tag while
the compile result is unchanged, in the shape the predecessors' mutation
evidence took.

**`AC-4a`.** `D3`'s population is **derived from the mechanism**, not from the
three sites this frame names — *"any `#[cfg(test)]` binding that can alter
control flow in the test profile"*. **State the census and its predicate.**
Working the three named sites and stopping fails this.

**`AC-4b`.** `D4` lands: the sixth `carry_call_input` site no longer records a
`callee` that the `D7` mutation substitutes, and `D7`'s *"two axes"* comment is
corrected or the third consumer removed. **Demonstrated** — arm
`CallInputTransferOrigin` and show `callee` unchanged. **A control that never
arms the mutation does not discharge this**, since the defect is invisible
unarmed.

**`AC-5`.** No repair to either branch lands here, and no ownership is inferred
beyond the routing `D2` states. **`D3` and `D4` are diagnostic-correctness
work, not branch repairs, and do not breach this.** **This node finishes the
fork; the repair is cut after it, from its result.**

**`AC-6`.** Nothing added to a production build surface, verified by a targeted
`ken-runtime` check rather than asserted.

**`AC-7`.** No-regression, in CI (`COORDINATION §12`).

## Why this earns a slot

**Because it is the last unmeasured step of a fork that has been narrowed three
times, and the Architect named both the increment and the instrument:** *"STOP
ASKING THE RUN, ASK THE PLANNER."*

**And because the repair cannot be cut without it.** Branch 1 puts the repair in
this chain's lowering; durable-lane puts it in `RT-CLOSURE-BOUNDARY-LANE` and
hands it the recursor rows. Different files, different owners, different node
sizing — and one static property of the plan decides it.

**This is lane 1.** `PROJECTION` (merged) → `CENSUS` (merged) → `CALL-SITE`
(merged) → `CALLEE-IDENTITY` (merged) → **this** → repair → `TRANSPORT` →
`DESCENT-RETIRE`.
