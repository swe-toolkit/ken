---
id: RT-NESTED-CONSTRUCT-ROUTE-EXCLUSIVITY
title: "CLOSED, complete negative result: route exclusivity is not expressible in this node's surfaces (Q2 NO) and the machine it would unify onto is behavior-incomplete for carried arguments (Q4), so the precondition itself has a precondition"
status: closed
owner: runtime
size: M
gate: none
depends_on: [RT-LEDGER-UNNAMEABLE-OBLIGATION-CONTRACT]
blocks: []
github: null
origin: "Steward, 2026-08-16, on the Architect's D1 hard-stop disposition at evt_29ar2vfvxf414 refuting shape (iv) on two grounds and naming the route change as the one surviving direction, explicitly unreleased and owed its own D0. Measured at ec2b4a1eb. Both cited builder docs (mod.rs:3130-3140 and mod.rs:3201) verified verbatim by the Steward before filing. Steward-filed per COORDINATION section 2."
---

## CLOSED. `D0` RETURNED NO, AND THE NODE DIED AT A READ AS DESIGNED.

**Runtime `evt_292ve8cdq1mmx` at `a8688cf89`, no candidate, no probe, no source
diff, no test run. Architect concurred in full at `evt_1dtf5g1xjw8z4`, verifying
the three load-bearing reads himself. No build was ever authorized and none is
now.**

| question | answer | consequence |
|---|---|---|
| **`Q1`** — what removes the construction, at which site | YES at the source-machine caller | the node survived question 1 |
| **`Q2`** — is the exclusion statable at the DISPATCH | **NO** | `OwnedUnitEmission` (`units.rs:5599-5611`) carries `function`, `body_occurrence`, `definition`, `header`, `slots`, `offsets`, `frame_bytes` — **no `SourceControl`, cursor, pending suffix, or eliminator ownership.** The direct traversal cannot tell the population apart |
| **`Q4`** — is the machine behavior-complete for these | **NO, and this is the stronger finding** | the machine forces every argument through `value.specialized_at(...)` (`core.rs:8215-8220`); the direct arm preserves a carried run via `transfer_constructor_operands` (`core.rs:17677-17688`). **Re-routing an ordinary non-worker `Construct` turns a valid carried construction into a refusal** |
| **`Q5`** — failure direction | concurred | under-inclusion is a fail-closed bookkeeping red; over-inclusion is a **miscompile**. False negative is the safe side, and any future predicate on this axis inherits that orientation |

> ### `Q4` IS WHY THIS IS "CLOSED" AND NOT "NOT TODAY".
>
> **`Q2` is a statement about available CONTEXT — contingent.** Widen the
> surfaces and it could change. **`Q4` is a statement about the DESTINATION and
> it is not contingent.** Even granted a perfect predicate, the place you would
> unify onto is behavior-incomplete.
>
> ⇒ **The precondition has a precondition.** This node was framed as a
> precondition for a not-construct; `Q4` shows it is itself gated on the machine
> first acquiring the direct arm's carried-argument handling.

**Both escapes are closed by PRIOR rulings, which is what makes this
complete.** A new caller/plan relation is the banned static-plan-export
widening; template-side outer classification is the refuted mint-site shape
(`RT-MINT-SITE-STATIC-DISCRIMINATOR` `D0`). **Same structure as Ground B: the
shape is defeated by rulings already in the corpus, not by a fresh judgment
call.** A successor cannot re-propose either without first overturning one.

### WHAT REMAINS IS A THREE-ACT CHAIN, AND NONE OF THE ACTS IS AUTHORIZED

**Stated as a chain because a three-act path reads like a one-node path when
only its first act is named** (Architect, `evt_1dtf5g1xjw8z4`):

1. **Factor the machine terminal to delegate its immediate pair to the
   producer-side path** (`core.rs:5007-5123` already holds the broader
   machinery; the terminal does not delegate to it) — this is `Q4`'s blocker.
2. **Then** route exclusivity becomes safe to attempt — but still needs `Q2`'s
   discriminator, which today requires a banned or refuted mechanism.
3. **Then** a terminal-keyed not-construct could finally be total — the
   original (iv).

**What the whole chain buys at the end: a bookkeeping red cleared, and valid
programs at depth 2 and 3 compiling. That is the whole of the prize.**

⇒ **Whether that is worth three acts is a SCOPE call, not a design call. It is
reserved for the operator, and this ring does not measure toward it.** Runtime
retains no candidate and does not open act 1.

## Why this existed: four shapes were dead and this was the only direction left

**`RT-LEDGER-UNNAMEABLE-OBLIGATION-CONTRACT` closed with a complete negative
result.** (i), (ii) and (iii) BARRED by the Architect's `D0` ruling
(`evt_1njg9qsfa3kak`); **(iv) REFUTED on two independent grounds**
(`evt_29ar2vfvxf414`):

| ground | what it says |
|---|---|
| **A — the route** | leg 1 is false, measured. The same nested composed recursive-case occurrences reach construction by **two** entries: the composed routes through `lower_source_machine`, which installs `Terminal::ResumeOuter`, and the direct `lower_expr` arm (`core.rs:17609-17639`), which has no `SourceControl` at all |
| **B — the mint site** | `static_worker_constructor_template` has exactly two call sites (`core.rs:7534`, `core.rs:17631`) and `mod.rs:3201` calls it **"the sole builder of the worker arm."** The construction is a property of **one shared callee's population**, not of either caller's terminal — so a not-construct *at the template* needs the mint-time discriminator `RT-MINT-SITE-STATIC-DISCRIMINATOR` `D0` already ruled does not exist |

**The defect is unchanged and it is not cosmetic: valid programs at depth 2 and
3 do not compile.** `D1d` established it by suppressing the refusal only inside
a disposable probe and observing correct execution and exit `0`.

> ### THE LESSON THAT PRODUCED GROUND B, BECAUSE THIS NODE CAN REPEAT IT
>
> **When a disposition is about whether something gets BUILT, read the
> BUILDER's doc before enumerating the callers' routes.** `mod.rs:3130-3140`
> already said the arm is *"reached from both the direct-descent and
> source-machine `RuntimeExpr::Construct` arms"* — **one sentence that answers
> a totality question about callers**, present in the tree before the
> predecessor opened. A shared-callee doc settles it; a route enumeration costs
> a probe to reach the same place.

## What this node WAS, and the reason it was believed to be attemptable

> **HISTORICAL. `Q2` and `Q4` both returned NO — see the closing block at the
> top.** This section records the hypothesis as it stood at release, because
> the warrant for killing it is only legible against what was proposed. **It
> describes a direction that is now dead; do not read it as live.**

**The surviving direction was a DISPATCH change, not a not-construct: route
nested composed recursive-case `Construct` occurrences so the machine's
terminal is their only entry, and the direct arm is unreachable for them.**

> ### WHY THIS IS NOT (iv) WEARING A NEW NAME
>
> **(iv) failed because it was keyed to the MINT SITE, which has no context.**
> The template is a shared callee; it cannot tell an outer occurrence from the
> innermost, and `RT-MINT-SITE-STATIC-DISCRIMINATOR` `D0` proved no such
> discriminator is exportable.
>
> **The routing decision is made by the CALLER, and the caller does have the
> context** — the specialized composed route (`core.rs:6673-6674`) and the
> carried composed route (`core.rs:17117-17166`) already know they are lowering
> a composed recursive case, which is why they call `lower_source_machine` at
> all. **That is a different kind of fact in a different place, and it is the
> only reason this direction is not already closed by the prior ruling.**

**It is also why the node is a `D0` read first.** The population being changed
is the general `RuntimeExpr::Construct` dispatch — **far wider than the
defect** — and the blast radius is the whole thing to rule on.

## THIS NODE IS A PRECONDITION, NOT A REPAIR. Amended 2026-08-16.

**As first framed, `D0` could return YES on every question and the
over-construction would still be there.** The Architect caught it before the
ring spent a turn (`evt_6eeqd3ad6ween`); the amendment is the Steward's
(`evt_3emw0kkedtpdc`).

**Verified at `core.rs:7517-7540`:** the source machine's **own** `Construct`
arm calls `static_worker_constructor_template` **unconditionally once
`recognized_constructor_worker_fields` answers** — the same sole builder, at
the same point relative to recognition, as the direct arm.

⇒ **Route exclusivity relocates which dispatch entry constructs. It removes
nothing by itself.**

**What it was thought to buy:** with a single entry, *"every route installs the
terminal with the exact pending suffix"* could finally be **true**, so leg 1's
totality — structurally unachievable last node — becomes achievable.

> **`Q4` REFUTED THIS, and the refutation is the node's most durable output.**
> The unification target is behavior-incomplete: the machine forces every
> argument through `specialized_at`, so re-routing an ordinary non-worker
> construction **regresses a valid carried construction into a refusal**.
> ⇒ **The precondition has a precondition** — act 1 of the chain at the top.

## `D0` — a READ, not a build. QUESTION 1 CAN KILL THE NODE CHEAPEST.

**Answer question 1 FIRST and hand it back alone if it is NO.** Questions 2-5
are worth answering **only if** question 1 says something removes the
construction — **measuring the blast radius of a change that fixes nothing is a
wasted turn.**

**1. After the routes are exclusive, WHAT removes the construction, and AT
WHICH SITE?** Concretely: **does the machine's terminal own the immediate
constructor/eliminator pair BEFORE its `Construct` arm reaches the template,
and is that ownership statable there without the template knowing which
occurrence is outer?**

> **If the answer is still "the template would have to know", this node dies
> exactly where (iv) died — and it dies HERE, at a read, before any blast
> radius is paid.** That is a complete result and the cheapest one available.

**2. Is the exclusion statable at the DISPATCH, from context the caller already
has?** Name the predicate and say where it is evaluated. **If stating it
requires knowing at the template which occurrence is outer, the answer is NO —
report that and stop.**

**3. What currently reaches the direct arm, and what would change?** The
occurrences to be re-routed are one population; **the general
`RuntimeExpr::Construct` arm serves many more.** Give the split. **A count of
"how many sites" is not the answer — the answer is which behaviours change.**

**4. Does the machine's terminal handle everything the direct arm did for these
occurrences?** The direct arm lowers args and builds ordinary specialized
fields when the classifier answers "not a worker". **If re-routing changes what
happens to a NON-worker construction, the blast radius includes the ordinary
path and that must be stated, not discovered.**

**5. What is the failure direction if the predicate is wrong?** State it
explicitly. **A field constructed that should not be is a bookkeeping red; a
field NOT constructed that should be is a miscompile.** The predecessor's
`AC-7` exists because that asymmetry was nearly crossed.

**Hand `D0` back alone with no candidate**, whatever it returns. `D1`/`D2` are
released by the Steward on the Architect's ruling.

## Acceptance criteria — ALL SPENT UNMET, because no build was authorized

> **`AC-1` through `AC-7` governed a candidate that was never released.** They
> are not failures and not outstanding work: `D0` returned NO before any build,
> which is the outcome the node was shaped to produce cheaply. **Do not carry
> them into a successor as owed criteria.**
>
> **`AC-8` is the exception — it was DISCHARGED, and by the strongest route.**
> It said a landed exclusivity change is not the defect closed. **Nothing
> landed**, so nothing can be cited as closing the depth-2/3 compile failure.
> The criterion held without ever being tested against a candidate.
>
> **`AC-5`'s two-directional mutation requirement survives as guidance for any
> future predicate on this axis**, together with `Q5`'s orientation: a green
> "the leak is gone" tests neither direction.

**`AC-1`. No mint-time discriminator is required.** The exclusion is evaluated
where the routing decision is already made. **A predicate that needs the
template to classify the occurrence is `RT-MINT-SITE-STATIC-DISCRIMINATOR`
`D0`'s refuted shape and is barred.**

**`AC-2`. The ordinary `Construct` population is unchanged**, demonstrated
rather than asserted. **This is the AC the blast radius actually turns on.**

**`AC-3`. The ledger's law is untouched.** No relaxation of `close`, no second
writer of `consumed`, no widening of the agreeing bijection. **Carried forward
from the whole campaign and not relaxable by this node.**

**`AC-4`. No erasure on authority acquired after construction**, under any
name — the Architect's at-or-before-construction test (`evt_6aarzqdm18vnh`).

**`AC-5`. `D2`'s control is two-directional and mutation-proven.** A mutation
re-admitting the direct entry for these occurrences must red, **and** a
mutation excluding an occurrence that legitimately belongs on the direct arm
must red. **A green "the leak is gone" tests neither.**

**`AC-6`.** The `D2k` controls still pass, and row4-depth-1 and row5-after-hole
are behaviourally unchanged.

**`AC-7`.** No-regression, in CI (`COORDINATION §12`). Local validation
targeted only — `-p ken-runtime`, never `--workspace`.

**`AC-8`. A LANDED EXCLUSIVITY CHANGE IS NOT THE DEFECT CLOSED.** The
over-construction persists until a separate not-construct lands, **and that act
is not authorized by this node.** Nothing here may be reported, flipped, or
cited as closing the depth-2/3 compile failure. **The Steward will not flip any
node `merged` on that reading.**

> **Why this is an AC and not a note.** A dispatch change with a blast radius
> over the general `RuntimeExpr::Construct` population, landing with the defect
> unmoved, is the worst trade available in this campaign — **question 5's
> asymmetry says the failure direction of getting it wrong is a miscompile.**
> The frame permitted exactly that reading until `evt_6eeqd3ad6ween`.

## Banned scope

- **`transfer`** (refuted, `D1c`), **void at supersession** (barred by
  *"positive authority at or before construction"*), **conditional
  transition**, **moving `consumed`'s write point**, and **not recognizing the
  outer field**. All four dead; none is a fallback if `D0` returns NO.
- **A not-construct at `static_worker_constructor_template`.** Refuted on both
  grounds above.
- **Extending the static plan's exports.**
- **Changing a producer so the ledger balances.**
- **Widening the re-route beyond the occurrences that carry the defect** in
  order to make the predicate simpler. **The blast radius is the risk; trading
  correctness of scope for simplicity of predicate inverts the trade.**

## Sequencing

**SPENT. `D0` ran and returned NO; the Architect ruled it; no `D1`/`D2` was
ever released.**

> **This node closes with NO CANDIDATE and can never reach `merged`.** Do not
> gate any successor on its landing. `closed` here means resolved-without-
> landing, which is exactly what a complete negative result is.

**The campaign's option space is exhausted at this node.** Five dispositions are
dead with warrants (conditional transition, binding-payload/`consumed` move,
not recognizing the outer field, not-construct at the template, and route
exclusivity). **The remaining disposition — whether depth-2/3 compile failure
stands as a known limitation, or the three-act chain is funded — is a SCOPE
call the Steward carries to the operator.** It is not more measurement by this
ring, and no seat should open act 1 on its own authority.
