---
id: RT-TRAP-MESSAGE-NAMES-FAMILY-NOT-POPULATION
title: "The PatternMatchFailure trap prints only view.family_symbol, so 'no runtime match case selected for X' is the IDENTICAL string whether the match had zero arms, one arm, or two arms whose tags did not match. It is a verdict with no population: one string for three distinct failure modes, and the user who hits it gets a diagnostic they cannot act on."
status: ready
owner: runtime
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Named by the Architect at evt_7t8na2v88mv71 (2026-09-15) while ruling the next instrument for the px8f trap hunt, and explicitly NOT claimed as that trap's cause: 'your call whether that is worth a node; I am naming it, not routing it, and it must not be folded into the px8f cause.' Steward-filed per COORDINATION section 2, as a separable instrument defect."
---

## The defect

Both production emitters of the message build it from the family symbol alone.
At `b0a7c2945`, `crates/ken-elaborator/src/erasure.rs`:

    2917    let default = RuntimeTrap {
    2918        code: RuntimeTrapCode::PatternMatchFailure,
    2919        message: format!("no runtime match case selected for {}", view.family_symbol),

    6041    let default = RuntimeTrap {
    6042        code: RuntimeTrapCode::PatternMatchFailure,
    6043        message: format!("no runtime match case selected for {}", view.family_symbol),

The family symbol is the only thing that varies. **The arms are not named, and
the tag actually observed is not named.** So one string covers at least three
distinct states:

    the match had NO arms                      -> upstream defect in view.branches
    the match had FEWER arms than constructors -> upstream defect in view.branches
    the arms were right, the tags diverged     -> a tag store/load defect

**This is a verdict with no population.** It reports that nothing matched
without reporting what was available to match or what was presented — the
distinction between *"I had nothing to check against"* and *"I checked and
none applied"*.

## Why it is worth fixing independently of any live hunt

- **The user gets an unactionable diagnostic.** Anyone hitting this trap learns
  the family and nothing else. They cannot tell a compiler bug from a
  non-exhaustive match in their own source.
- **The investigator gets one string for three hypotheses**, which means the
  message can never be the instrument — a separate instrumented compile is
  required to learn what the trap already knew at the moment it fired.
- It was measured costing exactly that on 2026-09-15: the px8f hunt could not
  distinguish these cases from the failure output and had to add compile-time
  prints to partition a space the trap site had in hand.

## NOT the cause of the px8f trap, and must not be folded into that hunt

The Architect named this while ruling a *different* instrument and was explicit
that it is not today's cause and must stay separate. **Do not close this by
fixing px8f, and do not close px8f by fixing this.** Whatever the px8f
divergence turns out to be, this message would still have been uninformative.

Recorded because the opposite mistake was made on this arc already: a node
filed mid-hunt was read as the hunt's cause and stayed `ready` on a premise its
own author had retracted.

## Deliverables

- **D0.** The message reports the population, not only the verdict: the family
  symbol, the arm count, the arm constructor symbols, and the tag actually
  observed.
- **D1.** The observed tag is reported **decoded through the name arena**, never
  as a raw integer. A raw word is unreadable to a user and ambiguous to an
  investigator — see `RESULT-NAMES-TWO-ARTIFACTS`, where a decoded name is what
  separates two live hypotheses that a raw word cannot.
- **D2.** Both emitters (`:2919` and `:6043` at `b0a7c2945`) are changed, and
  the message **states which site produced it.** They sit in different lowering
  paths — `:2919` is the plans-carrying path, `:6043` the plain inner path — and
  a message that does not name its site can send an investigator into the wrong
  function.

## Acceptance criteria

- **AC-1.** From the message text alone, a reader can distinguish the three
  states above. Demonstrate with a real trap from each state that is
  constructible; for any state that cannot be constructed, say so and why
  rather than asserting it is covered.
- **AC-2.** The tag is shown decoded. A test pins a decoded name, not an
  integer, so a future change that regresses to raw words fails.
- **AC-3.** The emitting site is identifiable from the message.
- **AC-4.** No regression, green in CI (never a local `--workspace` run;
  COORDINATION §12).

## Contention

Touches `erasure.rs` at two sites. **Contends with any active elaborator work
in the trap-lowering path**, which on 2026-09-15 includes the px8f diagnosis
(uncommitted instrumentation at `b0a7c2945`). Sequence after that lane is
quiet; nothing here is urgent enough to contend with a live hunt.

## Related

- `RESULT-NAMES-TWO-ARTIFACTS` — the same day's naming defect, and the reason
  D1 requires decoding rather than raw words.
- `RT-OBSERVATION-EXIT-STATUS-LAUNDERS-SIGNAL-DEATH` — same shape one layer
  out: an observation that substitutes a value for absent information and
  carries it onward without marking it.
- `RT-DISCHARGE-EXCLUDE-OR-REFUSE-BACKSTOP` — same shape again, in the
  compiler's own reasoning: four distinct reasons funnelled into one
  indistinguishable observable.
