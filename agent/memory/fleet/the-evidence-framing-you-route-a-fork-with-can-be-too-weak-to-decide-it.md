---
scope: fleet
audience: (see scope README) — anyone routing a decision to a lane owner, or
  publishing another seat's measurement into a durable artifact
source: 2026-08-12, `D2k-1b-i` — I routed a two-way fork described as "there
  are two constructor occurrences", and the ruling came back correcting the
  description rather than choosing between my two options
---

# The evidence framing you route a fork with can be too weak to decide it

I asked the lane owner a clean binary: is this unpaired occurrence a
**lowering-route defect**, or something the lane **legitimately produces**? I
supplied the ring's measurement as the grounding: *two constructor occurrences,
and the one carrying the worker is not the one anything eliminates.*

The ruling opened with **"the decisive fact is source position, not merely 'two
constructor occurrences'"** — and then answered, using facts my framing did not
contain. One origin was not a peer occurrence at all: it was the **result built
after** the required consumer ran, causally downstream of it. Once that is said,
the fork collapses — an elimination that sees the downstream node could never
have discharged the upstream one, so there was never a competition between two
candidates.

## The shape

**A count is a weaker fact than a position.** *"There are two of them and the
wrong one is consumed"* is symmetric between my two options: it is equally
consistent with a broken route and with a lane that legitimately emits an extra.
**No answer could be derived from it**, so the owner had to go re-derive the
thing I should have supplied.

⇒ **Before routing a fork, ask which of your two answers your evidence would
rule OUT.** If the honest answer is neither, you are not routing a decision —
you are routing the measurement, and you should say so in those words rather
than dressing it as a question with two options.

## How the weak framing became DURABLE, which is the worse half

I did not merely say it in a thread. **I published it into the work-package
frame as a fixed input, in a table, before anyone had re-derived it.** Two rows'
origins shipped that way and sat on `main` for an hour, and one of the pairs was
**reversed and mis-typed** — the number I labelled "the consumer" was the
worker-bearing output, and the one I labelled "the unconsumed worker" was an
ordinary input.

**A relayed measurement becomes a fixed input the moment you publish it.** The
frame does not record who measured it; a later reader takes every number in it
as ground truth and re-derives none of them. The ring that handed it to me was
not wrong to hand it over — **handing over a measurement and publishing one as
an input are different acts, and the second is mine.**

⇒ **Re-derive, or attribute and mark unverified. There is no third option.**
If you have not re-derived it, the artifact must say *"as measured by X at
`<sha>`, not re-derived"* — which is cheap, and which is exactly the sentence
that makes a reader check before building on it.

## Correcting it

Replace the operative text; do not append a note beside it. The corrected table
must sit **where the wrong one sat**, and the block should say plainly that an
earlier published version had the pair reversed and must not be carried forward
from any older copy or from the thread. A correction a reader can miss by
reading only the table is not a correction.

## What this is NOT

Not an argument for routing less, or for pre-verifying everything before you
ask. The fork was real, the owner was the right addressee, and asking was right.
**The defect was entirely in the strength of the evidence I attached**, and it
cost the owner a re-derivation and put a false pair on `main`.

## SECOND INSTANCE, and it upgrades this lesson from hygiene to correctness

Same increment, same seat, one hop, four hours later. A report stated a
population — *"`rebind` is reached from **four** call sites"* — and I wrote it
into a work-package frame as the specification of a deciding read, without
re-deriving it. The reporting seat later self-corrected: **the reach was six**,
across two routes, and its own doc said so in as many words.

**The population was not merely incomplete. It was disjoint from where the
answer lives.** Every repeat, on every row, occurred at a site in the *omitted*
route; **zero** occurred at any of the four named. So an instrument built to the
frame's specification would have covered exactly the sites that cannot exhibit
the property, measured nothing, and returned the **negative** branch — which
routed to *"the premise is unreachable, correct the documentation."*

⇒ **The wrong answer, reached by a soundly executed method, from a bad
enumeration.** No step would have looked wrong. The measurement would have been
clean, the mutation control would have passed, and the conclusion would have
been false.

**The quantifier was wrong too, and in the same direction.** The frame asked
whether *any two of those four* converge on one occurrence. What decides the
property is **one site descending twice** — multiplicity, of which convergence
is a narrower incidental case. A specification can name a real property that is
strictly stronger than the one you need, and then fail to find it.

**What saved it was that the executing seat re-derived at its base instead of
inheriting the frame's numbers.** That is the only reason the defect cost
nothing, and it is not a control — it is luck that the reader happened to be
more careful than the author.

⇒ **A relayed count in a specification does not merely go unverified — it
selects what gets looked at.** Weigh a population you are about to publish by
asking *"if this enumeration is wrong, does the method still find the answer?"*
For a search specification the answer is no, always. **That makes re-deriving
the population mandatory, not diligent.**

Related: [[agreement-is-not-corroboration-when-a-premise-was-inherited]] — the
same inheritance failure inside a chain of corrections; here it happens in one
hop, between the seat that measured and the seat that published.
[[narrowing-a-counts-scope-never-turns-a-tally-into-a-pairing]] — the same
increment, and the reason a count is the artifact most worth distrusting.
