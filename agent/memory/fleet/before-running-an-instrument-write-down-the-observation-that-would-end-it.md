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
    (c) ONE WAS SPECIFIED, THE WORLD CAN PRODUCE IT, AND SO CAN THE READING
        YOU ARE TRYING TO EXCLUDE. See the section below -- added 2026-09-19
        after this rule's own author failed it within hours of minting it.

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
  **This check is one-sided and it is not sufficient — see `(c)` below.**
  Producible is the easy half; producible only by the reading you are testing
  is the half that makes it a discriminator.
- **If the residue from a bounded pass is too large to work by hand, that is a
  finding to report — not a licence to build the general closure.** Say how
  large it is and stop.
- **A third, unexpected layer is an ANSWER.** Report it and stop; do not force
  again to see what is behind it.
- Related: [[an-enumeration-needs-a-proven-closure-not-a-better-grep]],
  [[a-capped-list-is-a-sound-presence-oracle-and-a-broken-absence-oracle]],
  [[a-differential-over-an-aggregate-is-an-existential-not-a-universal]].

---

## (c) PRODUCIBLE-BY-THE-HYPOTHESIS IS NOT THE TEST. PRODUCIBLE-ONLY-BY-IT IS.

**Measured 2026-09-19 on the RT bracket-release campaign: three instruments in
one hour, built by three different seats, all failing the same half.** One of
them was built by this lesson's own author, hours after minting it, while
citing it.

The rule above says write down the ending observation and check the world can
produce it. **Every one of the three passed that check.** What none of them
checked was whether the reading they were trying to EXCLUDE also produces it.

    a trichotomy      offered as "one of three" with no closure argument.
                      A fourth and fifth reading were found, one of them
                      sitting in source two seats had already read.
    a standing        keyed to a named hypothesis: "if the probe returns
    condition         IDENTITY, amend the node." It expired silently when the
                      enumeration turned out to be open, because a condition
                      keyed to a named reading is blind to an unnamed one.
    a position read   "are the releases contiguous at the end of the trace?"
                      Contiguous-at-end is what the DEFERRED account predicts.
                      It is also what the SCOPE-EXIT account predicts on a
                      program where nothing happens after the last bracket --
                      which was every fixture in the population.

**The third is the cleanest specimen, because the refuting control was free,
already captured, and named in the proposing post as the row to look at.** The
interpreter definitely releases at scope exit, and the interpreter also showed
releases-last. One line of the capture the author was pointing at killed the
predicate.

⇒ **The common form: a predicate is written against the reading you are trying
to confirm, and its behaviour under the reading you are trying to exclude is
never evaluated.** The first half gets written down because it is the half you
are thinking about. The second half is the whole content of the rule.

**Why (c) survives review when (a) and (b) do not.** A `(b)` instrument is
visibly unsatisfiable once someone asks. A `(c)` instrument is satisfiable,
precise, cheap, and returns a well-formed readable answer — it simply answers a
question with two claimants. **It fails by being confirmed**, so nothing in the
output flags it, and the confirmation is what stops anyone looking further.

### How to apply

- **Write BOTH rows before the run, not one.** For each reading in play, say
  what this instrument outputs under it. If two readings share a row, the
  instrument does not separate them and you have not built a discriminator.
  The corpus already states exactly this for acceptance criteria —
  [[an-acceptance-criterion-must-name-an-observation-the-failing-configuration-does-not-also-produce]]
  — and **that it was not carried across from ACs to instruments is the
  transferable failure here, not the bracket campaign.**
- **Run the predicate against a system whose behaviour is already known,
  first.** Here that was the interpreter, in the same log, at zero cost. A
  positive control is not a formality when you have no evidence the
  discriminating observation is producible at all. See
  [[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]].
- **Check the population can exercise the axis.** All three failures trace to
  a predicate that was sound in the abstract and ran on rows that could not
  distinguish its arms. Ask what property a row needs for your instrument to
  say anything, then count the rows that have it.
- **A test built by removing the suspected cause is in the benign cell by
  construction.** If you cannot state, before the run, the placement rule that
  puts your new fixture in the same cell as the anomaly, then the clean outcome
  is a NULL result and must be pre-registered as one — it is the tidy reading,
  it closes the thread, and it will be over-read.
- **Key a standing condition to the artifact's CLAIM, not to a named
  hypothesis.** "If the probe returns IDENTITY, amend the node" dies when the
  enumeration opens. "Does the row's written description match the measured
  mechanism?" does not. The Architect's own carry, kept in their words.
- Related: [[a-green-pilot-is-not-evidence-for-a-shape-it-never-produced]],
  [[an-enumeration-needs-a-proven-closure-not-a-better-grep]].
