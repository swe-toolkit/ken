---
scope: enclave
audience: (see scope README)
source: RT-LEXICAL-R3-FUSION-EMITTER, blocked 1943650e → approved cd19957d, 2026-08-14
---

# An absence claim is refuted at the consumer, not where the subject is defined

I blocked a merge candidate because a control's header claimed coverage of
*"every currently governed real R selector"* while driving two causes, and a
third — `ProducerArity` — was a key-installing, candidate-forming cause. I wrote
that its lowering reach was **"unmeasured"** and that *"nobody has measured"*
whether it reaches the validator.

**It had been measured, and the record was in the tree at the SHA I blocked.**
`core.rs:2935` carried a quoted diagnostic — *"`ProducerArity` **never reaches
it.** It refuses earlier, at `ComputationalMatch: case … expects 1 constructor
arguments but value has 2`"* — followed by *"⇒ The terminal-stop population is
two roots, not three."* Nobody has that error string without having run it.

**It was ten lines above a line I cited by number in the same review.** I quoted
`core.rs:2945` (`const D2F_EMITTER_ARMED: bool = false`) as my
production-unarmed evidence, in the same message where I asserted the absence.

**The mechanical cause is where I grepped.** I searched `ProducerArity` in
`static_transition.rs` (where the cause is **defined** and its fixture built)
and in `control.rs` (where it is **exercised**). I never searched `core.rs` —
where it is **refused**. A fact of the form *"does X reach Y?"* is recorded at
**Y**, beside the mechanism that admits or rejects it, not at X. Here the
account sat beside the arming constant, because that is where someone reasoning
about what arming does had to write it down.

**How to apply.**

- **Before writing "unmeasured" / "nobody has looked" / "this is not
  established", grep the subject in the file you are already citing.** An
  absence claim is a claim about a population, and it is cheapest to falsify in
  the file open in front of you. The cost is one grep; the claim blocks a merge.
- **Search at the CONSUMER, not at the definition.** Where a thing is declared
  and where it is exercised are the two places you will think of; where it is
  *rejected* is the third, and it is where a reach question is answered. Same
  discipline as [[grep-the-producer-not-the-cited-proxy]], pointed downstream.
  When the question is "what still guards this?", the consumer census — every
  call site of the observable — is the instrument, not the definition.
- **A quoted diagnostic string is measurement; prose about a mechanism is not.**
  When you find an account, check which one it is; and when *you* are the one
  asserting a mechanism, the same bar applies to you —
  [[mechanism-citation-needs-own-empirical-probe]].
- **A block founded on a false premise can still produce a real improvement —
  say both.** The recut named its population explicitly (`Exact`, `ReHomed`)
  where the old wording had no defined extension, and corrected a stale sibling
  comment calling the third cause *"a documented positive, not a refusal"*
  without the armed-lowering qualifier. Report the correction to your own
  premise first, then what the round bought, without using the second to soften
  the first.

Sibling of [[a-report-on-where-to-add-something-is-silent-about-what-exists]]:
there an accurate report's *silence* about adjacent lines was read as a negative
claim; here **I** was the one who did not read the adjacent lines, on a file I
had already opened.
