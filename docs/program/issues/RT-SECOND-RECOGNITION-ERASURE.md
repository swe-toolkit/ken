---
id: RT-SECOND-RECOGNITION-ERASURE
title: "Determine whether the depth-2/3 static-worker rebind is a succession of one obligation or two distinct ones, and select the disposition on that fact -- erasure is REFUTED and is no longer this node's deliverable"
status: active
owner: runtime
size: S
gate: none
depends_on: [RT-EMITTED-WORKER-CONSUMER-WALK]
blocks: []
github: null
origin: "Architect ruling evt_3cxm6654d5cjb, 2026-08-15, splitting the population measured by RT-EMITTED-WORKER-CONSUMER-WALK D0. Every symbol below was located by name against origin/main ac8a73d1b by the Steward before filing, and the ruling's mechanism attribution is corrected here on that basis. Steward-filed per COORDINATION section 2."
---

## ERASURE IS REFUTED. Read this before anything below it.

**Architect disposition `evt_3xdz0j957491`, 2026-08-15, on `D1b`.** **Everything
in this frame written before that measurement was aimed at the wrong fact.**

**The campaign was reasoning about an UNREAD FIELD. What was measured is a
REPLACED TRANSPORT.** T0 is not outstanding because nothing wanted it. It is
outstanding because a later `rebind` superseded it and the consumer took only
the newest. **Construction and transition both happened and both were correct.**

> **`D1b`: the recognition was transitioned and its successor WAS CONSUMED at
> origin 15.** It is a live link in a chain terminating in a real consumption.

⇒ **Erasing it would delete a correct step.** The erasure question was posed on
the premise that nothing downstream wanted it, and **that premise is now false.**
**This node is not closed by erasing, and `D0` below is withdrawn.**

**Two further options are also ruled out, so nobody re-proposes them:**

- **"The earlier recognition should never have been minted" — REFUTED**, same
  measurement.
- **"Relax `close` for a superseded transport" — REFUSED as an exemption.**
  Nothing would then check the successor was consumed, so a chain whose **last**
  member leaks would be indistinguishable from one that discharged. **Strictly
  weaker than today's law, and today's law is the only thing that caught this.**
- **"Make the nested environment extend rather than replace" — DECLINED.** It
  repairs the *lowering* to satisfy the *accounting*, and would make
  *"transitioned-but-unconsumed"* **dissolve rather than be answered.** A
  complete final case environment owning its own transport may well be **correct**
  for a nested match; **nothing has shown that mechanism defective.**

## The Architect's lean was TRANSFER. `D1c` REFUTED IT. Read this as history.

> ### TRANSFER IS REFUTED BY MEASUREMENT. Do not build it.
>
> **`D1c` measured `distinct` recognition ids at every nested rebind edge.** The
> section below is the reasoning that made `transfer` the expected disposition;
> it is kept because it is the record of what was ruled out and why, **and
> because the probe that killed it was designed against it.**
>
> **What actually holds:** depth-2 T0 and depth-3 T0/T1 are **genuine leaks**,
> `close` is right to refuse them, and **the open question is why a worker is
> constructed and then abandoned** — a successor node, not this one.

**The three options above all treat T0/T1/T2 as independent items. The
measurement says they are a SUCCESSION** — one thing, rebound twice, consumed
once at the end.

⇒ **The candidate defect is that the ledger's bijection is over TRANSPORTS where
the invariant it wants is over OBLIGATIONS**, and `rebind` is an
identity-preserving move the ledger records as a fresh construction **without
retiring its predecessor.** The repair would make `rebind` **transfer** the
outstanding obligation, so T0 retires into T1, T1 into T2, and T2's consumption
discharges it — **`close` green with no exemption.**

**The test of whether that shape is right is that the law gets STRONGER**: an
outstanding obligation with no successor and no consumption stays a hard error,
and **a rebind that fails to carry its predecessor forward becomes newly
detectable** — a defect class nothing can see today.

> ### THIS IS A LEAN RESTING ON AN UNMEASURED PREMISE. `D1c` IS NOT OPTIONAL.
>
> **It rests entirely on "`rebind` is identity-preserving", which nobody has
> measured.** The Architect states plainly that he read it from the words
> *"replaces"* and *"newly rebound"* — **a reading of vocabulary, not of the
> mechanism**, and the exact move he was corrected for earlier in this campaign.
>
> **Do not build the repair before `D1c`.** It is one field, on the same
> instrument, and it selects between two repairs that share no code.

## This is HALF a population. The other half is not ruled and is not here.

`RT-EMITTED-WORKER-CONSUMER-WALK` `D0` measured that the five governed
expressions are **two different situations**, and the Architect ruled only one
of them:

- **row4-depth-2, row4-depth-3** — a worker **is** emitted and **is** consumed,
  by `SourceContinuation::CallArgument`. A **second, different** recognition is
  then minted, and `StaticWorkerFieldLedger::close` refuses that one. **This
  node.**
- **row4-depth-1, row5-after-hole** — never enter either emitter. Nothing
  emitted, nothing consumed, **so no prior consumption is available to authorize
  any repair there.**
  [[RT-UNTRANSITIONED-FIELD-CONSUMER-PROBE]], and it must be measured before
  anyone proposes a repair.
- **row1-owned-scope** — retains its `NativeJoinPlanV1` refusal, a different
  site entirely and outside both nodes.

**Nothing here depends on the other half's answer.** The Architect said so
explicitly, and this node must not wait on it.

## THE RULING'S MECHANISM ATTRIBUTION IS CORRECTED HERE. Read this before coding.

**The ruling says `constructor_field_bindings` (`mod.rs:4936`) "mints a
recognition for a field whose worker was already emitted and lawfully
consumed."** Measured against `origin/main` `ac8a73d1b`, that function does not
mint recognitions. **It is the transport event**, and its own body says so:

```rust
// **THE TRANSPORT EVENT.** The field enters lexical binding authority here
let transport = self.static_worker_fields.rebind(*recognition, self.defining_function_id)?;
```

**The production call sites are each exactly one, and they are in different
files:**

| ledger operation | sole production call site | enclosing function |
|---|---|---|
| `recognize` (mint) | `core.rs:15579` | `static_worker_constructor_template` (`core.rs:15545`) |
| `rebind` (transport) | `mod.rs:4958` | `constructor_field_bindings` (`mod.rs:4936`) |

Every other `.recognize(` / `.rebind(` hit in the tree is in
`core/tests/control.rs`. **Verify this before relying on it** — it is the
premise the deliverable is cut against, and this node's predecessor exists
because a cited coordinate was read past its warrant twice.

⇒ **The mint is at `core.rs:15579`, not at `constructor_field_bindings`.** A
ring that looks for a mint in `constructor_field_bindings` will find only
`rebind` and stall — **which is where `D1b` found the succession, so the
coordinate matters more now, not less.** **The ruling's
direction is right; its coordinate is not.**

## The consequence that makes this repair groundable, and it is not "which row"

`core.rs:15579` is the **same** mint that serves row4-depth-1 and
row5-after-hole. **A predicate keyed on the row would be a guess and would
reach into the unruled half.**

**It does not need to be.** `recognize`'s own doc states the enabling fact:

> **Never deduplicated by `field_origin`.** Two constructions of one occurrence
> are two constructed workers, each owing its own transition and its own
> consumption.

⇒ For depth-2/3, `static_worker_constructor_template` mints **twice for one
occurrence** — once for the worker that is emitted and consumed, once during
root-adapter lowering. **The second mint is the defect.**

⇒ **The predicate is "has this occurrence's worker already been emitted and
consumed?", not "is this row 4 at depth 2".** That is precisely the positive
authority the ruling identified, it is available at the mint, and it is
**self-limiting**: for row4-depth-1 and row5-after-hole no prior consumption
exists, so the predicate declines and the unruled half is untouched by
construction rather than by a scope rule.

## Why TRANSITION is barred -- the half of the ruling that survives `D1b`

`D2k-1c-1` hardened `close` into an **agreeing** bijection against exactly this:

> `range(transitioned) ⊆ dom(minted)` still admits
> `transitioned[r1] = transitioned[r2] = T`: **two constructions sharing one
> transport, discharged by that transport's single consumption.**

**That is depth-2/3 exactly** — two constructions, one real consumption. Making
the second recognition transition could only discharge it by leaning on the
first's consumption, **which is the hole `D2k-1c-1` was written to close.**

⇒ **Transition here does not merely lose on merit; it reopens a fixed defect.**

> ### IT SURVIVES, AND IT DOES NOT REACH `TRANSFER`
>
> **Its conclusion in favour of erasure is dead** — see the refutation at the
> head of this frame. **Its argument against TRANSITION stands**, and the
> distinction is load-bearing:
>
> | move | what happens to the predecessor | reaches `D2k-1c-1`'s hole? |
> |---|---|---|
> | **transition** a second recognition onto the same transport | stays outstanding; discharge **borrows** the first's consumption | **yes** — two constructions, one transport, one consumption |
> | **transfer** the obligation on `rebind` | **retired into its successor** | **no** — one obligation, one consumption, at the end of the chain |
>
> **`D1c` closed this out: there IS no succession here** — the ids are distinct,
> so the transfer row below is a counterfactual and the transition row is the
> only live prohibition. The distinction is kept because it is what let `AC-1`
> stay absolute while the lean was still open.
>
> **Sharing and succession are different relations.** A transfer on succession
> leaves the two-constructions-one-transport bijection **exactly as hardened**,
> which is why `AC-1` below can stay absolute while `D1c`'s lean stays live.

## Deliverables

> **`D0` IS WITHDRAWN.** It read *"erase the second recognition at or before its
> mint."* **Erasure is refuted** — see the head of this frame. It is recorded
> rather than deleted so that a reader arriving from the node id, the git
> history, or an older citation finds the withdrawal instead of the instruction.

**`D1a` — DELIVERED** at `30ee4dbf1`. Exactly one recognition per transport,
with full-relation visibility positive (depth 2 also `1->1`; depth 3 `1->1` and
`2->2`). **The `d2f` sharing law is not in play on these rows.**

**`D1b` — DELIVERED** at `e46cd4959`. T0 reaches **neither** production
`note_consuming_call` site. The route diverges at the nested
`Lowering::lower_computational_match_value_composed`, whose complete final case
environment **replaces T0 with a newly rebound transport** (T1 at depth 2; T1
then T2 at depth 3). The origin-15 exact `Var` call reaches
`lower_source_machine_with_continuation_inner` and **consumes only the newest**.
Positive control: the same probe recorded the T1/T2 consumption at origin 15.

**`D1c` — DELIVERED at `e46cd4959`. THE ANSWER IS `DISTINCT`. TRANSFER IS
REFUTED.**

> **Every nested composed-rebind edge maps a DIFFERENT recognition to each
> transport.**
>
> | depth | edge | recognition | field origin |
> |---|---|---|---|
> | 2 | eliminator 21, T0 | R0 | 35 |
> | 2 | origin 5, T1 | R1 | 25 |
> | 3 | origin 31, T0 | R0 | 45 |
> | 3 | origin 21, T1 | R1 | 35 |
> | 3 | origin 5, T2 | R2 | 25 |
>
> `R0 != R1` at depth 2; `R0 != R1 != R2` at depth 3.

**`AC-2a` discharged on its face.** The trace pairs the exact
`ConstructorField::StaticWorker.recognition` input with the binding transport
from that same `constructor_field_bindings` call and prints **opaque recognition
ids**, so **unequal ids cannot collapse behind matching coordinates.**

⇒ **These are separate constructed-worker obligations. Depth-2 T0 and depth-3
T0/T1 are GENUINE LEAKS**, and `close` is right to refuse them.

> ### IT IS NOT A CHAIN, AND THE FIELD ORIGINS ARE WHY
>
> **The `field_origin` secondary column came back DIFFERENT at every level** —
> 45, 35, 25 at depth 3. So this is **not** the `recognize`/`recognize` shape
> (two constructions of *one* occurrence) either.
>
> **Each nesting level constructs a worker for its OWN source field, and only
> the innermost is ever consumed.** The succession reading was wrong in both of
> its parts: the transports do not carry one obligation forward, and the
> obligations are not even about the same field.

**The CAUSE of construction-and-abandonment is a successor component-design
node**, owed by the Steward. **It is not this measurement's to answer and
runtime will not implement a repair from it.**

**How the question was corrected before it ran** — kept because two dispositions
were built on the wrong phrasing:

> ### SUPERSEDED `evt_2mphh3ttmf1v0` — the first phrasing could not select
>
> **It asked whether the transports name the same obligation by "same
> origin/field coordinates."** That equates obligation identity with
> `(origin, field)` — **the exact key already refuted on this campaign**, with
> the refutation written into the struct being asked about
> (`lowering/mod.rs:4342-4347`, verified):
>
> > **NOT keyed by `field_origin`.** It was, with `or_insert`, and that silently
> > dropped the second construction of one occurrence: `recognize`, `recognize`,
> > one `rebind`, one consumption, close **green** — with a constructed worker
> > forgotten before any transport existed to owe for it.
>
> And at `recognize` (`mod.rs:4486-4489`): *"**Never deduplicated by
> `field_origin`.** Two constructions of one occurrence are two constructed
> workers, each owing its own transition and its own consumption."*
>
> ⇒ **A `same coordinates` report is equally consistent with BOTH branches**, so
> the probe would have returned a confident answer and **selected on a
> coin-flip.** That is not weak evidence; it is evidence that does not bear on
> the question.

**The corrected question, which is the one that ran:**

> **At the `rebind` in `lower_computational_match_value_composed`, do the
> predecessor and successor transports trace to the SAME
> `StaticWorkerRecognitionId`, or to DISTINCT ones?**

**What each answer would have selected**, recorded so the refutation is legible:

| answer | meaning | disposition |
|---|---|---|
| same recognition id | succession — one obligation, rebound, consumed once | make `rebind` **transfer** the obligation |
| **distinct — MEASURED** | two constructed workers | **T0 is a genuine leak and `transfer` is REFUTED.** Retiring T0 into T1 would discharge an obligation never consumed, re-introducing the silent drop the keying exists to prevent |

**`D1d` — THE DECIDING READ, and it is this node's only open measurement.**
Architect `evt_5sqzthmqnz4va`. **`D1c` established that the outer recognitions
exist and are unconsumed; it is SILENT on why they exist at all**, and every
remaining disposition turns on that.

**THREE DISPOSITIONS. They share no code and land in different files.**

| | reading | repair site | note |
|---|---|---|---|
| **(A)** | **over-construction** — each level mints a worker for its own field, only the innermost is needed; the outer transports are dead | **the mint** | leak is bookkeeping, not behaviour; subject becomes [[RT-MINT-SITE-STATIC-DISCRIMINATOR]] |
| **(B)** | **under-consumption** — fields 45 and 35 are genuinely required and the emitted code reads only 25 | the composed lowering's **consumer** | **a MISCOMPILE that `close` caught. Highest severity, and it OUTRANKS this node if true** |
| **(C)** | **under-recorded consumption** — the innermost read physically traverses 45 to 35 to 25, so all three discharge at runtime, and `note_consuming_call` observes where only the final transport is visible | **where consumption is RECORDED** | not an exemption; see below |

> **(C) IS NOT THE EXEMPTION `AC-1` BARS.** The law is unchanged and still
> refuses a genuinely unconsumed obligation; what moves is the **observation
> point**, not the strength of the check. **Do not reject it by pattern-match
> against the exemption refused earlier — and do not let anything that IS an
> exemption enter dressed as (C).**
>
> **(A) must not be assumed because it is the cheapest.** It is also the one
> that quietly authorizes erasure at the mint, which is what `AC-3b` exists to
> prevent.
>
> **(C) is a READING, not a measurement — the Architect labels it as his own.**
> It rests on `D1b`'s phrase *"consumes only the newest transport"*, which is
> consistent with a call site crediting one transport while the read traverses
> three. **It is on the list because it is live, not because it is supported.**

**These programs do not compile today, so there is no execution witness.** The
probe needs the refusal **suppressed in-probe only** — an instrument, not a
relaxation, **reverted exactly as `D1a`/`D1b`/`D1c` were.**

**TWO COLUMNS, and the second is the one that will get dropped:**

1. **Execution.** With the refusal suppressed, do the depth-2 and depth-3
   programs compute the **correct result**? **Wrong or crashing ⇒ (B), and STOP
   THERE** — it outranks this node and gets its own escalation. **Correct ⇒ (A)
   or (C).**
2. **Emission.** Does the emitted read of field 25 **traverse** the transports
   bound for 45 and 35, or reach 25 by another route? **Traverses ⇒ (C). Does
   not ⇒ (A).**

> ### COLUMN 1 ALONE SELECTS NOTHING between (A) and (C)
>
> **They prescribe opposite repairs in different files.** A handback that stops
> at *"it runs correctly"* has measured the easy column and left the fork open.
> **If column 2 cannot be read with this instrument, REPORT THAT** rather than
> narrowing to what column 1 can see.

**Positive control — fourth probe, and that discipline is the only reason the
first three were usable.** Show the harness **would have reported a wrong
answer**: perturb one field's value and demonstrate the mismatch is caught. **A
"correct result" from an oracle that cannot see an incorrect one is a silence —
and this time the silence would select (A) by default.**

**`D2` — the refusal message, and `D1c` UNBLOCKED it by answering `distinct`.**
`close` link one says the field is *"neither consumed at an exact-`Var` call nor
erased before construction"* and that a constructor carrying it *"has no runtime
representation."*

**Both halves are wrong as applied, for different reasons, and the message must
not be repaired by weakening it:**

- **"has no runtime representation" is false** — a worker **is** emitted and
  consumed on these compiles, just not this recognition's.
- **"neither consumed nor erased" is TRUE of this recognition** — `D1c` settled
  that. **These are genuine leaks.** The message's *verdict* is right; its
  *explanation* asserts a general impossibility it has not established.

⇒ **The accurate message is about THIS recognition's own transport never
reaching a consumer** — not about a chain (there is none) and not about the
constructor having no representation.

**`D2` may proceed. It is message text only** — `AC-1` still bars touching the
law, and `AC-2b` still bars a repair.

## Acceptance criteria

**`AC-1`.** The ledger's law is unchanged. **No relaxation of `close`, no second
writer of `consumed`, no widening of the agreeing bijection.** A repair that
edits `close` is the wrong repair.

> **`AC-1` NOW BINDS HARDER, because `D1c` removed the one repair that could
> have satisfied `close` without touching it.** This note previously explained
> why a *transfer* repair would not violate `AC-1`. **Transfer is refuted.**
>
> ⇒ **`close` is REFUSING CORRECTLY.** Depth-2 T0 and depth-3 T0/T1 are real
> unconsumed obligations. **Any proposal that makes this red go green by editing
> `close` is now unambiguously the wrong repair** — there is no longer a reading
> under which the refusal is an accounting artifact.

> **`AC-2` IS WITHDRAWN.** It required the erasure to carry *"positive authority
> at or before construction — the measured consumption."* **There is no erasure**
> — see the head of this frame — and the premise that the consumption was
> available to authorize one is exactly what `D1b` refuted.

**`AC-2a`.** **`D1c` reports its answer with a positive control**, per the rule
this campaign has now earned three times. **A `same` verdict from an instrument
never shown to distinguish `distinct` does not discharge `D1c`.**

**`AC-2b`.** **No repair is built in this node.** `D1c` has now selected, and
what it selected is **the leak investigation** — why a worker is constructed and
abandoned. **That is the successor's, not this node's.** `D2` (message text) is
the only implementation work left here.

**`AC-3`.** Row4-depth-1 and row5-after-hole are **behaviourally unchanged**.
Demonstrate it: their refusal is unruled and this node must not move it. **If
the predicate changes them, the predicate is keyed wrongly.**

**`AC-4`.** `D2k-0` still reds if any edge or refusal moves for the rows this
node does not govern. **A red there is information, not a test to update.**

**`AC-5`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Relaxing or editing `close`'s law.** `AC-1`, and it is the point.
- **Anything keyed on row identity or depth.** The predicate is about prior
  consumption.
- **The depth-1 / after-hole half.** [[RT-UNTRANSITIONED-FIELD-CONSUMER-PROBE]].
- **`D2k-1c`**, the planner-owned `ContinuationTemplate` population, and the
  continuation-source surface, all still outside.

## Sequencing

**On the critical path.** [[RT-LEXICAL-RECURSOR-CONSUMERS]] is the only
un-merged `depends_on` of [[RT-RECURSOR-TRANSPORT]], whose `D3` is the joint
retirement that [[RT-DESCENT-RETIRE]] closes. **This node and
[[RT-UNTRANSITIONED-FIELD-CONSUMER-PROBE]] are independent and may run
concurrently or in either order.**

## The finding worth carrying past this node

**A limit stated on one axis of a two-axis claim gets read as universal on the
other.** The `D2k-0` rider hedged *"among the sites that were TAGGED"* — a limit
on the **interval** — and both the Architect and the Steward read the
**population** as universal. Only two of five reach an emitter.

⇒ **A measurement report must say which rows it observed and which it did not,
and a reader must refuse to generalize one that does not.** Twice on this node a
statement about instrumented sites became a statement about the population.
