---
id: RT-EMITTED-WORKER-CONSUMER-WALK
title: "Walk the tag set forward from the successful static-worker emission to the refusal the five governed expressions actually hit, and report the first site that changes disposition"
status: closed
owner: runtime
size: S
gate: none
depends_on: [RT-LEXICAL-RECURSOR-CONSUMERS]
blocks: [RT-SECOND-RECOGNITION-ERASURE, RT-UNTRANSITIONED-FIELD-CONSUMER-PROBE]
github: null
origin: "Architect ruling evt_4hs6teqd9yks2, 2026-08-15, on the Steward's question at evt_4x72jp54qwp after two refuted repair attempts. Every symbol cited below was located by name against origin/main 3d69257b5 by the Steward before filing; the ruling's line numbers were not carried. Steward-filed per COORDINATION section 2."
---

## CLOSED, NOT MERGED — and the distinction matters to the two successors.

**This node is measurement-only. It produced no candidate and never will**, so
it can never reach `merged`. Its successors depend on it, and a successor gated
on a predecessor's `merged` would wait forever.

⇒ **`closed` = resolved-without-landing.** `D0`, `D1`, and `D2` are all
discharged; the result is the section below, and the disposition went to the
Architect at `evt_38tt8vj6hnfn0` and was ruled at `evt_3cxm6654d5cjb`.
**Read the result here; there is no diff to read.**

## MEASURED at `ac8a73d1b`. `D0` DELIVERED, AND IT REFUTED THIS FRAME'S PREMISE.

**Read this before anything below it. Everything downstream was written against
a premise the measurement corrected.**

**The premise this frame asserted — "the five reach the emission, so it cannot
come back empty" — is false.** Tagging **both** production constructors of
`StaticWorkerCallOutcome::Emitted`:

| row | reaches an emitter? | first disposition-changing site |
|---|---|---|
| row1 owned-scope | no | `NativeJoinPlanV1` refusal, retained |
| row4 depth-1 | no | `StaticWorkerFieldLedger::close` |
| row4 depth-2 | **yes** | `StaticWorkerFieldLedger::close` |
| row4 depth-3 | **yes** | `StaticWorkerFieldLedger::close` |
| row5 after-hole | no | `StaticWorkerFieldLedger::close` |

**Where the premise came from, because the error class is the point.**
`D2k-0`'s rider said *"the last tag before every refusal was the successful
emission."* That is a statement about **the sites that were TAGGED** — the four
`value_at` callers plus the `Construct` arm. The Architect read it as a
universal over the five, and this frame amplified it into an `AC`. **The same
tagged-population-read-as-universal error the node was filed to correct, one
level up.**

**`D0`'s actual answer: four of five rows converge on
`StaticWorkerFieldLedger::close`**
(`lowering/mod.rs:4721` at `ac8a73d1b`, named by symbol because that file moves).
For the two emitted rows, `SourceContinuation::CallArgument` consumes the
emission and **root-adapter lowering then creates a DIFFERENT
`ConstructorField::StaticWorker` recognition**, which is what `close` refuses on.
⇒ **The refused recognition is not a consumer of the earlier emission**, which
is the half of the ruling the measurement refutes. **`lower_binder` is not on the
measured suffix**, so the recorded hypothesis is answered: no.

**What `close` link one enforces, in its own words:** a constructed field
*"that no static elimination rebinds, so the field is neither consumed at an
exact-`Var` call nor erased before construction"*, with erasure *"structurally
absent here — a recognition exists only because the field was built."*

⇒ The chain is **construct → transition → consume**, and this population
**constructs and never transitions**, because `rebind` is minted by a static
elimination and these five have no exact-`Var` call. **Attempt one was not
looking at the wrong site. It was looking at the missing one.**

**`D2` disposition: routed to the Architect at `evt_38tt8vj6hnfn0`** as the one
question the ledger's law admits two answers to — **give the population a lawful
transition, or prevent the recognition.** No banned edge was crossed, no repair
site proposed, probes reverted, the three named `D2k` controls pass 1/1, base
equals tip, worktree clean.

> **The refusal text ends *"has no runtime representation."* That is the third
> impossibility claim in this node's history and it is true of an UNCONSUMED
> worker. Whether these five must be unconsumed is the question, not the
> premise.**

## Two attempts missed, and they were the same KIND of attempt

Section 3's exact-`Var` callee path and the constructor-argument branch are
different sites and **the same class of site: somewhere the static worker must
be RECOGNIZED so it can be handled.**

- *Is it recognized at the callee path?* No — for these five there is no call.
- *Is it recognized at the constructor-argument position?* No —
  `recognized_constructor_worker_fields` never fired across the five-row control.

> **Two misses at two recognition sites is the signal to attack the shared
> premise, not to pick a third site.** A third a priori guess is a coin flip
> precisely because it would be drawn from the same class. **This node proposes
> no site.**

## The premise is already refuted by `D2k-0`'s own rider, unread for this

`D2k-0` tagged all four `value_at` callers; `mod.rs:3661` never fired, and **the
last tag before every refusal was the successful static-worker emission** —
`Ok(StaticWorkerCallOutcome::Emitted(emitted.0, emission))`, the tail of
`call_static_worker_with_inputs`.

**That is a success, not a refusal or a fallback.** The worker call is entered,
the operand vector assembled, the call emitted, the outcome `Emitted`.

⇒ **The static worker for these five is already recognized and already emitted.
The refusal happens to a value the mechanism has successfully produced.** No
recognition site fires because there is nothing left to recognize.

⇒ **The repair is at a CONSUMER of a successful `StaticWorkerCallOutcome::Emitted`,
and the whole recognition class is ruled out** — on the ring's own measurement,
not a new one.

> **This is also direct evidence against the impossibility framing.** *"The
> repair needs a representation surface that does not exist"* is hard to sustain
> when the measurement shows the mechanism running to a successful emission.
> **The surface is being produced. What is unresolved is what happens to it
> next.** That claim was struck from [[RT-LEXICAL-RECURSOR-CONSUMERS]] and is
> not this node's premise.

## The honest limit, and it is what makes this a measurement

**"The last tag before every refusal" holds only among the sites that were
TAGGED** — the four `value_at` callers plus the `Construct` arm. It does **not**
mean the refusal fires immediately after the emission with nothing in between.

⇒ **The instrument cannot see untagged ground, and the interval between the
successful emission and the refusal is exactly that ground. The repair lives in
it.**

## Deliverables

**`D0` — extend the tag set FORWARD from the successful emission to the
refusal, over the exact five-row control, and report the first site that changes
disposition.** Nothing else. **A bounded walk over one interval, not a search.**

**`D1` — report the site by SYMBOL, not by line.** This file's line numbers move
under every neighbouring merge. The ruling that produced this node cited
`core.rs:14593`; **the emission is at `14588` on `origin/main` `3d69257b5`** —
the mechanism identification was right and the coordinate was five lines stale
within the hour. Name the function and the construct.

**`D2` — the disposition.** Whether the first disposition-changing site is
inside this node's scope or across one of its banned edges. **If it is across a
banned edge, that is a result and a handback, not a licence to cross it.**

## Acceptance criteria

**`AC-1`.** The walk covers the interval, not a hypothesis. **Do not tag only
the sites named below and stop** — a tag set drawn from the hypothesis can only
confirm or deny the hypothesis, and this node exists because two hypotheses
already missed.

**`AC-2`.** **It cannot come back empty, so an empty result is an instrument
failure and must be reported as one.** The five reach the emission and they
reach a refusal; something between them changes disposition. If the tags show
nothing, the tags are wrong.

**`AC-3`.** No production logic change. This node instruments and reports.
Probes are reverted before handback and `git diff --stat` shown clean, as the
last attempt did.

**`AC-4`.** `D2k-0` still reds if any edge or refusal moves. **A red there is
information, not a test to update.**

**`AC-5`.** No-regression, in CI (`COORDINATION §12`).

## The Architect's hypothesis — test it, do NOT let it bound the walk

Offered at `evt_4hs6teqd9yks2` as a hypothesis and explicitly not as the ruling:
`lower_binder` immediately follows the emission's function, its own doc says its
single new outcome installs a `LoweringEnvironmentBinding::StaticWorker`, and
`StaticWorkerBinding` refusals cluster in it and in
`construct_static_worker_binding`. **If the forward tags land there, the repair
is a binder-installation question and inside this node's scope.**

> ### THE REFUSAL POPULATION IS WIDER THAN THAT CLUSTER. Measured, not assumed.
>
> The Steward counted **twelve or more `"StaticWorkerBinding"` refusal sites in
> `core.rs`** at `3d69257b5`, running well past `construct_static_worker_binding`
> — not the four the hypothesis names. **Tagging only the hypothesis cluster
> would reproduce the exact failure this node was filed to correct**: an
> instrument shaped by the guess it is meant to test. Walk the interval; let the
> tags choose.

## Banned scope

- **Proposing a third repair site before `D0` reports.** That is the class the
  ruling closed.
- **Repairing anything.** This node measures. The repair is the successor and
  its shape is `D0`'s output.
- **`D2k-1c`**, which stays a wrong cut, and the planner-owned
  `ContinuationTemplate` population and continuation-source surface, which stay
  outside.

## Sequencing

**This is the last gate on the `RecursiveDescent` retirement.**
[[RT-LEXICAL-RECURSOR-CONSUMERS]] is the only un-merged `depends_on` of
[[RT-RECURSOR-TRANSPORT]], whose `D3` is the joint retirement, which
[[RT-DESCENT-RETIRE]] closes. **It outranks everything else runtime holds.**

## The finding worth carrying past this node

**When a repair guess misses twice, stop proposing sites. Instrument the route
between the last thing measured to WORK and the first thing measured to FAIL.**

The first two attempts each proposed a site and asked whether the population
reaches it — a yes/no about one candidate, which **returns nothing when the
answer is no.** This instruments an interval the population is already known to
traverse and asks it where the disposition changes.

**`D2k-0` had already established both endpoints. Nobody had walked between
them.**
