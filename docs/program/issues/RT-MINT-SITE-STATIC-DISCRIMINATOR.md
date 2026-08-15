---
id: RT-MINT-SITE-STATIC-DISCRIMINATOR
title: "Determine whether a principled static discriminator exists at the static-worker recognition mint that entails the constructed field is never read, and specify the predicate without discharging it"
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-UNTRANSITIONED-FIELD-CONSUMER-PROBE]
blocks: []
github: null
origin: "Architect ruling evt_3czp0t9gnnz61, 2026-08-15, refusing the Steward's fork at evt_5etykb2px44w4 as a false binary. The mint site's in-scope values below were enumerated by the Steward from core.rs:15545-15590 at origin/main 30ee4dbf1 before filing. Steward-filed per COORDINATION section 2."
---

## The fork was false and this is the question it was hiding

The Steward asked whether `RT-UNTRANSITIONED-FIELD-CONSUMER-PROBE`'s measured
`not needed` is the authority for erasure. **The Architect ruled that the two
options did not differ in the standard of evidence — they differ in the SCOPE of
the erasure**, and that the deciding fact is one nobody has read.

- **Erase at the mint** — the property must hold for the mint's **whole
  population**, now and for every future occurrence reaching it.
- **Scope the erasure to the measured population** — lawful under *"authority
  established at or before construction"*, **but only if that population is
  statically identifiable at the erasure site.** An erasure scoped to a set the
  site cannot name is not scoped; **it is an erasure at the mint with a comment
  attached.**

> ### "MEASURE MORE ROWS" CLOSES NOTHING. Do not frame it.
>
> **The gap is not that two rows are too few.** It is that a runtime observation
> over occurrences is **the wrong KIND of fact for a static site.** Measuring
> every row present in the tree today would produce a claim about one SHA and
> still not be a property of the mint.

## The question, in its sharpened form

**Is there a PRINCIPLED static property at the mint that entails "this field's
value is never read"?**

**Principled, not fitted.** A discriminator discovered by looking at what the
two measured rows happen to share **inherits the exact defect the ruling
identified** — it re-encodes the runtime observation as a static-looking test.
**It must express a reason such fields are not read, not a coincidence that
separates the observed ones.**

## What is in scope at the mint, enumerated so the read starts from fact

`static_worker_constructor_template` opens at `core.rs:15545`; the sole
production `recognize` is at `core.rs:15579`. **Verified at `origin/main`
`30ee4dbf1`; report by symbol, since this file moves under every neighbouring
merge.** Available at the call:

| value | source |
|---|---|
| `static_origin: StaticOriginId` | the constructor's origin, a parameter |
| `position: usize` | the field ordinal, loop index over `args` |
| `field_origin` | `self.static_transition_plan.child_static_origin(static_origin, position)?` |
| `constructor: &str` | parameter |
| `self.defining_function_id` | the enclosing generated function |
| `binding: StaticWorkerBinding` | `recognized[position]`, cloned |
| `args: &[RuntimeExpr]`, `recognized: &[Option<StaticWorkerBinding>]`, `env: &[LoweringEnvironmentBinding]` | parameters |

**One structural fact the read should start from, offered as an observation and
not as an answer:** the recognized-worker branch pushes
`ConstructorField::StaticWorker` and `continue`s — **it never routes the
argument through `child_occurrence`/`lower_expr`**, which is what the sibling
branch does for ordinary fields. **Whether that entails anything about
downstream reads is exactly the question and is not settled by the shape.**

## Deliverables

**`D0` — the read.** Does a principled static discriminator exist at the mint
that entails the constructed field is never read? **Answer yes or no with the
property named**, or **not determined by this read**, which is a legitimate
result.

**`D1` — if YES: specify the predicate.** State the discriminator itself.
**Do not discharge it** — see Sequencing.

**`D2` — if NO: say what the deliverable becomes.** Erasure then requires a
property over the mint's whole population, which the predecessor's `D0` does not
supply and cannot. The static predicate becomes the next node's deliverable and
the erasure its successor.

## Acceptance criteria

**`AC-1`.** **The predicate is stated as the DISCRIMINATOR, never as row
labels.** A frame or an assertion naming row4-depth-1 and row5-after-hole
re-encodes the runtime observation and inherits its defect. **This is the
acceptance criterion the ruling asked for by name.**

**`AC-2`.** **No erasure is implemented and no predicate is discharged.** This
node reads and specifies.

**`AC-3`.** If the answer is `yes`, **state why the property is principled
rather than fitted** — what makes such fields unread, not what separates the two
observed ones.

**`AC-4`.** No production logic change; probes reverted, `git diff --stat`
clean.

**`AC-5`.** No-regression, in CI (`COORDINATION §12`).

## Sequencing — the predicate CANNOT be discharged while `D1a` is open

**Architect, explicitly.** If the answer is *establish it statically*, the
predicate quantifies over **every occurrence reaching the mint — which includes
row4-depth-2 and depth-3**, whose membership in a lawful sharing group is
exactly what [[RT-SECOND-RECOGNITION-ERASURE]] `D1a` is measuring.

⇒ **You cannot assert a property over a population while one of its members has
an open question about whether it belongs.** **Specify the predicate now,
discharge it after `D1a`.** That is a constraint on this frame, not a hold on
the work, and it is written here rather than left for whoever assembles the
candidate to notice.

## Banned scope

- **"Measure more rows."** Ruled to close nothing.
- **Implementing the erasure**, at the mint or scoped. `AC-2`.
- **Depth-2/3**, which belongs to [[RT-SECOND-RECOGNITION-ERASURE]] and whose
  `D1a` this node must not pre-empt.
- **Relaxing `close`**, retroactive rollback, planner `ContinuationTemplate`
  work, continuation-source work, `D2k-1c`.

## The standing rule this campaign produced, now binding on every zero

> **A reported count of one is worth precisely what the demonstration that the
> instrument would have seen a second is worth. A zero or a one with no positive
> control is a silence, not a measurement.**

That is why the predecessor's `D0` is usable: a synthetic ledger-minted positive
control reached `specialized_at`, so its zero counts meant *no reader* rather
than *blind instrument*. **Carry it into every count this campaign reports.**

**And the reason the all-zeros shape is not evidence:** an index of `0` is
commonly both the first value allocated and the value meaning absent or default,
so *"every edge reports `0`"* is exactly as consistent with **one shared object
seen from several edges** as with **each edge having its own, every one first.**
**Those readings prescribe opposite repairs.**
