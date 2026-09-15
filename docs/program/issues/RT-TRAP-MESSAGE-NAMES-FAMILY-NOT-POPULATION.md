---
id: RT-TRAP-MESSAGE-NAMES-FAMILY-NOT-POPULATION
title: "The PatternMatchFailure trap prints only view.family_symbol, so 'no runtime match case selected for X' is the IDENTICAL string across FOUR states -- no arms, a short arm set from a non-exhaustive SOURCE, a short arm set from a DROPPED arm, and complete arms whose tags diverged. It is a verdict with no population: one string for four distinct failure modes, two of which are the user's own bug, and the user who hits it gets a diagnostic they cannot act on."
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
the tag actually observed is not named.** So one string covers **four** distinct
states, two of which are the user's own bug and two of which are compiler
defects:

    no arms at all                                        -> compiler defect
    short arm set because the SOURCE was non-exhaustive    -> THE USER'S OWN BUG
    short arm set because something DROPPED an arm         -> compiler defect
    arms complete, tags diverged                           -> compiler defect

**This is a verdict with no population.** It reports that nothing matched
without reporting what was available to match or what was presented — the
distinction between *"I had nothing to check against"* and *"I checked and
none applied"*.

**Rows 2 and 3 are the pair that matters most and they are the hardest to
separate.** `cases` is built from `view.branches` — the source match's *written*
branches (`erasure.rs:2835-2837` and `:5981-5983`) — so a user who writes a
non-exhaustive match legitimately produces a short arm set. That is their
source, not a defect. Nothing in the current message, and nothing in an arm
count alone, separates it from the compiler having lost an arm.

(Architect, `evt_78d5649dyc51c`: the first filing of this node stated three
states and collapsed exactly this pair, while its own motivation section named
the distinction as the user-facing one.)

## Why it is worth fixing independently of any live hunt

- **The user gets an unactionable diagnostic.** Anyone hitting this trap learns
  the family and nothing else. They cannot tell a compiler bug from a
  non-exhaustive match in their own source.
- **The investigator gets one string for four hypotheses**, which means the
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
- **D0a — THE DECLARED CONSTRUCTOR ROSTER, which is what separates rows 2 and
  3.** The message also reports the constructors the family *declares*. Arms
  versus declared constructors separates non-exhaustive-source from
  dropped-arm; the decoded observed tag separates matched from diverged.
  Without this the message can carry an arm count and still leave the single
  most useful question — *is my match non-exhaustive, or did the compiler lose
  an arm?* — unanswerable from the message alone.

  **Reachability measured, not assumed** (Steward, at `b0a7c2945`):

      semantic.data_metadata.get(family) -> DataMetadata
      DataMetadata.constructors: Vec<ConstructorMetadata>   (checked_core.rs:1138)
      ConstructorMetadata.symbol: StableSymbol              (checked_core.rs:1145)

  `semantic: &CheckedCoreSemanticInputs` is a **parameter of both enclosing
  functions** — `lower_body_term_with_plans` (`erasure.rs:2416`, which contains
  `:2919`) and `lower_match_view` (`:5949`, which contains `:6043`). The idiom
  is already in use in this file at `erasure.rs:3898`
  (`semantic.data_metadata.get(family)`). **No plumbing is required and the node
  stays size S.**

  Note `CheckedCoreDeclarationBodyView` does **not** carry a roster — its fields
  are `symbol`, `level_params`, `checked_type`, `body` (`checked_core.rs:287`).
  A grep for a constructors accessor on that type is a true zero and is evidence
  about that type, not about reachability; the roster comes from `semantic`,
  not from `declarations`.
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

- **AC-1.** From the message text alone — **without the reader holding the
  source** — a reader can distinguish the **four** states above. Demonstrate
  with a real trap from each state that is constructible; for any state that
  cannot be constructed, say so and why rather than asserting it is covered.

  **The non-exhaustive-source versus dropped-arm split is the one this AC
  exists for.** An implementation that reports the family, the arm count, the
  arm symbols and the tag satisfies an arm-count-based reading of this AC
  completely and still cannot answer it. If D0a is descoped for any reason,
  **this AC must be amended in the same change** to say the split requires the
  reader's source — so that a passing AC-1 is never later read as having
  delivered a distinction it did not.
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
