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

Related: [[agreement-is-not-corroboration-when-a-premise-was-inherited]] — the
same inheritance failure inside a chain of corrections; here it happens in one
hop, between the seat that measured and the seat that published.
