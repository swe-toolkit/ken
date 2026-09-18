---
scope: fleet
audience: (see scope README)
source: 2026-09-18, Architect — minted after two separate instrument bans on
  one node (RT-CARRIER-PRODUCER-OCCURRENCE) turned out to be one criterion.
  Ruling evt_5pa8y50t8q5ny; placed at fleet scope by the Steward because it
  binds anyone who builds a measurement, not only the Architect.
---

# Before running an instrument, write down the observation that would END it

**And check that the world can produce that observation.**

That is the whole rule. It binds every seat that builds a measurement — a grep
key, a census, a traversal, a forcing step, a sweep.

## Two failure modes, and they look nothing alike from the inside

    (a) NO TERMINATING OBSERVATION WAS SPECIFIED.
        A peeling/forcing ladder: force a guard, observe the next layer,
        repeat. Every run yields "a layer", which is always available, so the
        instrument is ended by fatigue or budget rather than by a result.
    (b) ONE WAS SPECIFIED AND THE WORLD CANNOT PRODUCE IT.
        A caller closure whose stopping condition was "no branch reaches an
        entry point with no bind above it". Unproducible: correct code reaches
        such entries on every compile, so only a residue or a false alarm was
        ever reachable.

**`(a)` feels like diligence** — each run genuinely buys something. **`(b)` feels
like rigour** — the condition is precise, checkable, and formally correct.
**Neither feels like the other, and both are the same defect.** Two separate
rulings were needed before anyone saw they were one rule.

The legitimate forms in each case terminate for a reason you can state in
advance:

    per-function hand pass over a named residue   you run out of functions
    one forcing step, named target, binary        it lands one way or the other

⇒ **The discriminator is whether the next layer is SPECIFIED BEFORE the run or
DISCOVERED BY it.** A bounded residue worked by hand is not a small ladder; a
single forcing step with a named target is not a rung.

## WHY IT MUST HAPPEN AT SPECIFICATION TIME

**Neither banned form errors.** The ladder produces layers. The closure produced
thirteen apparent counterexamples, none real. **Both emit well-formed output
that reads as progress**, so the check cannot be deferred to the output — there
is nothing in the output to check it against. A broken traversal chasing a false
negative produces the same shape as a working one.

⇒ **It has to happen at specification time or not at all, which is exactly when
it is cheapest and feels least necessary.**

## How to apply

- **Publish the terminating observation beside the key.** The Architect's own
  diagnosis: they published sweep keys that day and did not publish termination
  conditions, and *"the two instruments I got wrong were both ones where I had
  the key and not the ending."* A key says what to match. It does not say what
  would make you stop.
- **Say what the negative result looks like, then ask whether correct code
  could produce it.** If correct behaviour satisfies your counterexample
  condition, the instrument cannot terminate in the negative — only a residue
  or a false alarm is reachable. Redesign before running, not after.
- **If the residue from a bounded pass is too large to work by hand, that is a
  finding to report — not a licence to build the general closure.** Say how
  large it is and stop.
- **A third, unexpected layer is an ANSWER.** Report it and stop; do not force
  again to see what is behind it.
- Related: [[an-enumeration-needs-a-proven-closure-not-a-better-grep]],
  [[a-capped-list-is-a-sound-presence-oracle-and-a-broken-absence-oracle]],
  [[a-differential-over-an-aggregate-is-an-existential-not-a-universal]].
