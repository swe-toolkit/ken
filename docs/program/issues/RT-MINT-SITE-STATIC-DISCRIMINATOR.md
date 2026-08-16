---
id: RT-MINT-SITE-STATIC-DISCRIMINATOR
title: "Determine whether a principled static discriminator exists at the static-worker recognition mint that entails the constructed field is never read, and specify the predicate without discharging it"
status: closed
owner: runtime
size: S
gate: none
depends_on: [RT-UNTRANSITIONED-FIELD-CONSUMER-PROBE]
blocks: []
github: null
origin: "Architect ruling evt_3czp0t9gnnz61, 2026-08-15, refusing the Steward's fork at evt_5etykb2px44w4 as a false binary. The mint site's in-scope values below were enumerated by the Steward from core.rs:15545-15590 at origin/main 30ee4dbf1 before filing. Amended after release with the Architect's shared-transport constraint, evt_4reh9tgp36cmm, verified by the Steward against control.rs at origin/main e46cd4959. Steward-filed per COORDINATION section 2."
---

## CLOSED. `D0` RETURNED `NO` AT `0a19e3714`. Measurement-only, never `merged`.

**Runtime-leader `evt_5rdvrkf80c5j6`.** **No principled static discriminator
exists at the mint, and the reasons are structural rather than a sampling
limit:**

| candidate | why it fails |
|---|---|
| the `child_occurrence`/`lower_expr` skip | **does not entail no later reader** — `rebind` and possible field use are selected **downstream** of the mint |
| transport | **temporal provenance, not a field-use predicate** |
| `constructor` / `position` | **do not discriminate** — governed-unread and later-read share the same constructor and field 0 |
| origin constants | **would fit rows, not state law** — exactly what `AC-1` banned |

⇒ **The static plan exports no total mint-to-every-`Match`-binder-reader
relation**, so the predicate cannot be specified, let alone discharged.

**`AC-1` and `AC-3a` both held under pressure.** The ring declined to fit a
predicate to the observed rows, and declined to substitute `AC-3a`'s unexercised
shared-transport absence for the missing relation. **A surface absence was not
turned into a repair.**

## `D2` — what the deliverable becomes

**A lawful erasure successor needs a NEW static total per-recognition/field
relation** proving **both**:

1. **no future binder consumer**, and
2. **no later transport traversal or rebind** — the corrected `AC-3b`.

> ### DO NOT FRAME THAT RELATION YET -- `D1d` decides whether it is wanted
>
> Under **(A) over-construction** the relation is the right next build. Under
> **(B) under-consumption** the repair is in the consumer and erasure is
> **wrong**. Under **(C) under-recorded consumption** erasing these recognitions
> is a **miscompile**.
>
> **Two of the three make this relation useless or harmful.** Building it now
> would pick the branch by construction — the same error as framing the
> successor before `D1d` reports.

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

## The predicate may NOT premise on "no shared transport reaches this site"

**Architect constraint `evt_4reh9tgp36cmm`, 2026-08-15, added after release.**
**Verified against `origin/main` before amending**, since a frame that carries a
cited coordinate unread is the defect this campaign keeps paying for.

`control.rs:4610-4617` — inside the doc comment of
`d2f_the_two_binder_projections_share_one_source_field_transport` — carries a
**withdrawal**:

> **The REASON once given here was wrong and is withdrawn.** This paragraph read
> *"the front end does not produce the shape"*. It does produce it ... **why**
> these rows are synthetic is now open rather than answered: **a real-source
> producer exists and no row here uses it.** Read this as a debt with a known
> payer, not as a closed boundary.

The refutation it rests on is `control.rs:4494`: the Architect's bounded
producer probe (`evt_2gzjt1zqy402z`) measured `NESTED_LIFT_NAT_THREE_SOURCE` —
**real source, ordinary front end** — producing a retained generated
lifted-family `ComputationalMatch` in erased Runtime IR under both `Executable`
and `Library` selection.

**`control.rs:4634` — *"Production stays unarmed; the arm is the `cfg(test)`
RAII `D2fEmitterTestArm`"* — is scoped to the compiles that were run.** It is
**not** the statement that production cannot produce a shared transport.

⇒ **The reason for believing production cannot reach that shape is withdrawn,
and no replacement has been established.** [[RT-SECOND-RECOGNITION-ERASURE]]
`D1a` is unaffected — it measured two rows and its exactly-one result stands.
**A STATIC predicate is different: it quantifies over every occurrence reaching
the mint, including whatever the withdrawn-reason population contains.**

> **A predicate resting on "no shared transport reaches this site" would rest on
> an UNEXERCISED ABSENCE — the same defect as resting on a runtime observation,
> arriving by a different road.** `control.rs:4590-4592` records that this row
> has already twice *"reported a sample in the voice of a population."* Inside a
> predicate, it would be the third and the hardest to see.

**If the read concludes the discriminator depends on that premise, THAT
DEPENDENCY IS THE FINDING.** Report it as `D0`'s answer; do not absorb it as an
assumption and do not treat establishing it as in scope here.

## "NEVER READ" and "NEVER CONSUMED" are INDEPENDENT. `D1b` separated them.

**Architect `evt_3xdz0j957491`, 2026-08-15.** **Before that measurement these two
were used interchangeably across this campaign.** They are not the same property
and this node depends on the difference.

| property | object | where measured |
|---|---|---|
| the constructed **field is never read** | the `ConstructorField::StaticWorker` value | [[RT-UNTRANSITIONED-FIELD-CONSUMER-PROBE]] `D0`, rows row4-depth-1 and row5-after-hole |
| the **transport is never consumed** | the ledger's transport obligation | [[RT-SECOND-RECOGNITION-ERASURE]] `D1b`, depth-2 and depth-3 |

**`D1b` is the separating example.** At depth 2 and 3 the transport **is**
consumed — at origin 15, through
`lower_source_machine_with_continuation_inner` — just not the transport the
ledger is watching. The nested
`Lowering::lower_computational_match_value_composed` rebinds T0 to T1 (and T1 to
T2), and the consumer takes only the newest, so `close` reports T0 outstanding
**while the chain terminates in a real consumption.**

⇒ **A recognition can be fully accounted for downstream and still be reported
outstanding.** Its field being unread says nothing about whether its transport
carries an obligation forward.

> ### THE PREDICATE MAY NOT ERASE A LINK IN A REBIND CHAIN
>
> **Even when that link's own field is never read.** A predicate keyed on
> field-readership alone **cannot see the chain**, and erasing a chain link is
> how a ledger complaint becomes a **real miscompile**.
>
> **If the read finds the discriminator cannot tell the two apart, THAT IS THE
> FINDING** — same disposition as `AC-3a`.

**This node's rows are not `D1b`'s rows.** `D1b` measured depth-2/3, where the
transport is consumed; this node's population is the never-read rows. **The
disposition on [[RT-SECOND-RECOGNITION-ERASURE]] does not reach them.**

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

**`AC-3a`.** **The predicate does not premise on "no shared transport reaches
this site."** If it does, the answer is `D0`-with-dependency-reported, not a
specified predicate. **Architect constraint, and it is an acceptance criterion
because a premise is invisible once a predicate is written down.**

**`AC-3b`.** **RESTATED `evt_5sqzthmqnz4va` after `D1c`. The earlier wording
keyed on "a link in a rebind chain"; `D1c` refuted succession, so that phrase
named nothing — and an acceptance criterion with an empty premise CANNOT BE
FAILED, so it reads as a live guard while checking nothing.**

> **The predicate may not erase a recognition whose TRANSPORT is traversed or
> rebound by a later read**, even when that recognition's own field is never read
> directly.

**Field-unreadness and transport-liveness are independent.** `D1b` is the
separating example, and `D1c` shows the transports at successive nesting levels
belong to **different obligations over different source fields** — so *"its
field is unread"* **cannot be used as a proxy for** *"its transport is dead."*

**If the read cannot establish transport-liveness, report that dependency as the
finding. Do not narrow the predicate to the rows that can be seen.** Same shape
as `AC-3a`, and it fails closed for the same reason.

> **Why this is not a formality: the three live dispositions disagree about it.**
> Under **(A) over-construction**, erasing the outer recognitions is precisely
> the **correct** repair. Under **(C) under-recorded consumption**, their
> transports are traversed by the innermost read and erasing them **is a
> miscompile.** **This node cannot assume either, and it is starting now.**

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
- **Establishing whether a real-source producer reaches the shared-transport
  shape.** That is the withdrawn reason's open debt and it is a separate node.
  **Here it is only ever reported as a dependency, per `AC-3a`.**
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
