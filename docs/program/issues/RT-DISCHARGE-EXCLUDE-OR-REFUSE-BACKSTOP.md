---
id: RT-DISCHARGE-EXCLUDE-OR-REFUSE-BACKSTOP
title: "The discharge path DETECTS a foreign producer identity and then silently discards the proof instead of refusing. Four clear-and-break arms funnel into ONE sink -- `if grounds.is_empty() { continue; }` -- which cannot distinguish its four inputs, so 'I could not establish a ground' is indistinguishable from 'no ground was needed'. The repair belongs at the sink: fixing the foreign-identity arm alone relocates the fail-open rather than closing it. The arc's own contract forbids exactly this. It may NOT land without a witness that makes the check fire -- until then it is a documented invariant, not a check."
status: draft
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Ruled out of the ABI-S6 D5b candidate by the Architect at evt_2ctrn25j8fe39 (2026-09-15) after runtime-implementer measured the repair inert on px8f (evt_6zww772af95c4). The Architect had ruled it the floor and riding the candidate at evt_2ptnm0dyjx4kw; the warrant was that it repairs the failure there, and that was refuted by measurement. Steward-filed per COORDINATION section 2. AMENDED 2026-09-15 after the Architect reported two coordinate/AC defects (evt_7wsf1z27e679a) and the Steward's verification of them found a third, which refutes the mechanism the first filing asserted. AMENDED AGAIN the same night: the Architect ruled the filter question the Steward routed back (evt_5ey9ngwwf0dz8) -- the filter is not a fail-open and :3429 is a dead arm -- and runtime-implementer answered the Steward's open question by measuring that the discharge seat covers two bodies while six functions emit the trap (evt_6k9xx4trry0hc), which makes D0 a conjunction. AMENDED a third time: the Architect examined the two arms the Steward had marked unexamined (evt_1ka2n674qja4m) and established that all four arms share one sink, moving the repair target off the arm and onto the sink -- fixing the foreign-identity arm alone would relocate the fail-open rather than close it."
---

> # FILED 2026-09-15 AS `draft`, DELIBERATELY NOT RELEASED.
>
> **This must not be handed to the D5b ring as an addition.** It was just ruled
> OUT of that candidate. Releasing it while the same seats are hunting the px8f
> trap would re-create exactly the coupling the ruling removed.
>
> It also **must not be released without D0 below being achievable.** A node
> whose acceptance requires a witness nobody has found yet is a research task
> wearing a repair's clothes, and it should be sized as one if that is what it
> turns out to be.

## READ THIS BEFORE ANY COORDINATE BELOW: THE SITES ARE CANDIDATE-ONLY

**Every `units.rs` and `mod.rs` coordinate in this node is at
`b0a7c2945`, the unlanded D5b candidate. NONE of them exist on `main`.**
Measured at `origin/main` = `80e39a64a` (and unchanged at `9f9010dcb`):

| probe | `origin/main` | `b0a7c2945` |
|---|---|---|
| `authority_grounds` in `lowering/units.rs` | **0 hits** | `:3377` |
| `grounds.clear` in `lowering/units.rs` | **0 hits** | 4 hits |
| `lowering/units.rs` length | 8445 lines | 12623 lines |
| the contract doc-comment | **absent** | `lowering/mod.rs:1318` |

**The bare line numbers resolve on `main` to real, unrelated, error-free code**
— `:3377` and `:3424` are both `#[cfg(...)]` attributes on
`px8-ds-test-support`. There is no signal to a reader that they are in the
wrong tree; implausibility of the answer is the only detector, and these
answers are perfectly plausible.

This matters more here than in the case that taught the fleet the lesson,
because **this node is `draft` and will outlive the candidate.** (c) has been
ruled out of D5b, the candidate is respinning, and nothing guarantees these
sites ever reach `main` at these offsets — or at all, in this shape.

⇒ **Lead with the symbol name; the number never travels without its anchor.**
Whoever discharges this re-measures the coordinates at the tip they are working
on. Do not trust a number in this file against a tree that is not
`b0a7c2945`.

## The defect, which survives the withdrawal

True of the code independently of any particular program. All coordinates at
`b0a7c2945`, `crates/ken-runtime/src/cranelift_backend/lowering/units.rs`.

**The filter.** `authority_grounds` (`:3377`) is built from `body.authorities`
keeping only `authority.identity == identity`, so foreign-identity producers
are absent from it.

**The loop, and the sink every arm funnels into.** Over `proof.sources`, each
source reaches one of **four** `grounds.clear(); break;` arms, and all four land
on **one** exhaustion test. Verified line-by-line at `b0a7c2945`:

    3421  for source in &proof.sources {
    3423    if let Some(authority) = body.authorities.get(source) {
    3424      if authority.identity != identity || authority.word != *source {
    3425        grounds.clear(); break;     ARM 1 -- foreign identity, read from the
                                           UNFILTERED map. THE LIVE FAIL-OPEN.
    3428      let Some(ground) = authority_grounds.get(source) else {
    3429        grounds.clear(); break;     ARM 2 -- DEAD (ruled; derived)
    3433    } else if proof_call_seeds.get(source) == Some(&identity) {
    3434      let ValueDef::Result(producer, _) = body.func.dfg.value_def(*source)
    3436      else {
    3437        grounds.clear(); break;     ARM 3 -- a MATCHING call seed whose value
                                           is not an instruction result
    3444    } else {
    3445      grounds.clear(); break;       ARM 4 -- no recognised provenance at all
    3448  }
    3449  if grounds.is_empty() { continue; }        <-- THE SINK

## THE DEFECT IS AT THE SINK, NOT AT THE ARM. REPAIRING ARM 1 ALONE MOVES IT.

**Arms 3 and 4 are silent discards of exactly Arm 1's class.** Arm 3 fires when
a matching call seed's value is not defined by an instruction result — a block
parameter being the obvious candidate, which is what a carrier value becomes
when it crosses a branch. Arm 4 fires when the source has no recognised
provenance at all. **Neither is a proof that nothing is there.** Both are *"I
could not establish a ground"*, discarded without a word.

So the outcome-collapse is **not a property of `:3424`. It is a property of the
loop's exit discipline.** Four different reasons for not having a ground produce
one observable, and that observable is identical to *"no ground was needed"*.

⇒ **Fixing Arm 1 alone relocates the fail-open; it does not close it.** A source
that would have been refused at Arm 1 can reach Arm 4 instead and be discarded
just as silently. The three outcomes must be distinguished **at the sink** —
where `grounds.is_empty()` is consumed — after which each arm either supplies a
reason or is shown unreachable. That is a smaller change than four separate
repairs and it is the only shape that cannot leave a hole.

This is the arc's own standing line applied to the ruling that created this
node: **a partial widening relocates a refusal rather than closing it.**

**The bound, stated with the claim.** That Arm 3 or Arm 4 ever *fires* is **not
measured** — the block-parameter reading of Arm 3 is derived from what
`value_def` can return, not traced to a program that produces one. The claim
that does not need a measurement is structural: **all four arms share one sink,
and the sink cannot distinguish its four inputs.** That holds whether or not any
given arm is reachable, and it is what makes the sink the right target.
(Architect, `evt_1ka2n674qja4m`; structure Steward-verified at `b0a7c2945`.)

Together these collapse two different facts: *"I could not prove this"* and
*"I proved there is nothing here."* The site must distinguish three outcomes,
not two:

    identities agree                        -> emit
    path certifiably excluded (a real cut)  -> emit without that path
    NEITHER                                 -> REFUSE

**That is a fail-open**, and it is the defect whichever identity turns out to
be authoritative.

## THE MECHANISM THIS NODE FIRST ASSERTED IS REFUTED. The defect is worse.

The original filing said the filter *removes the disagreement from the evidence
set*, and that the site then *responds to exhaustion* by clearing. **Measured at
`b0a7c2945`, that is not what happens, and the correction cuts toward severity
rather than away from it.**

`:3424` tests `authority.identity != identity` against **`body.authorities`,
the unfiltered map.** The foreign identity is therefore *detected, explicitly,
at the site*. The clear-and-break at `:3425` is a deliberate response to having
found the disagreement — not blindness manufactured upstream by the filter.

⇒ **The site knows, and discards anyway.** Detection followed by silent discard
is a strictly worse defect than the blindness first described, and it removes
the most sympathetic reading of the code.

A consequence worth stating plainly: because `:3425` is reached only when the
identity matched is *false*, and `:3429` is reached only after `:3424` found
the identity matching, **the filter's deletion cannot be what drives `:3429`** —
an identity-matching source is in `authority_grounds` by construction. The
filter looks load-bearing from the outside and, on this path, is not.

**What this does NOT establish, and the discharging seat must not inherit it:**
what the compiler does downstream with the missing carrier fact. `continue`
skips the tag query; whether the unit then emits without the fact, or a later
stage refuses, is **unmeasured here**. The fail-open claim above rests on the
contract, not on a traced emission. Trace it before repairing it.

## The contract it violates

`lowering/mod.rs:1318` **at `b0a7c2945`** (absent from `main`), on
`generated_constructor_authorities`:

> These record the identity the producer actually emitted, including an
> identity different from a generated function's demanded Result. The latter is
> not silently dropped: the finished proof must either exclude its path or
> refuse, and it must never relabel the word.

A disjunction with a prohibition. Three permitted states, one forbidden, and
the site can reach the forbidden one.

## Why it is NOT in the D5b candidate, and must not be folded back in

It rode that candidate on one warrant: that it repairs the failure there. That
was **refuted by measurement**. At the correct population the implementer
measured `foreign_proof_sources=0` in the body that traps — nothing to exclude,
nothing to refuse — and px8f's trap was unmoved by the repair.

The Architect's own prediction, *"expect px8f to go from compiling-and-trapping
to refusing-to-compile"*, was the falsifiable half of that ruling and it died.
What survives is only **"this site is a defect"**, which is a reason for a node
and not a reason to ride someone else's candidate.

**"The principle survives" must not launder into "so ship the check."**

## D0 — THE WITNESS. This is the load-bearing deliverable, not a formality.

**An input on which the check FIRES.** A program whose discharge genuinely
reaches a proof source carrying a foreign identity, so that the refusal is
observed rather than argued for.

**IT IS A CONJUNCTION OF TWO REQUIREMENTS, AND THE SECOND IS THE ONE THAT WILL
BE MISSED.** The witness must:

1. **carry a foreign producer identity**, and
2. **be a body that REACHES THE PROOF PATH AT ALL.**

A witness satisfying only (1) — the obvious reading — could never reach the
seat, and it would look exactly like a passing AC-1. This is not hypothetical:
measured at `b0a7c2945`, `derive_certified_cuts` runs **twice in the entire
px8f compile** (`funcid55`, `funcid60`), while **six** distinct functions emit
the failing trap (44, 60, 61, 63, 64, 65). Five of the six never reach the seat.
(runtime-implementer `evt_6k9xx4trry0hc`; Architect `evt_5ey9ngwwf0dz8` naming
it a conjunction.)

- **Search the corpus first.** If a witness exists it should be found, not
  built — a found witness also tells us the condition occurs in practice.
- **If it must be constructed, it is a mutation arm**, and it belongs beside
  `RT-MUTATION-ARM-CONTROL-COVERAGE` rather than standing alone. Say so
  explicitly in the closeout; do not quietly construct one and call it found.
- **Until a witness exists this is a documented invariant, not a check.** The
  arc has spent this week removing unfalsifiable guards. It must not add one.

## D0 DISCHARGES A SECOND DEBT AT THE SAME TIME

The witness **is** the positive control that the measured `foreign_proof_sources
= 0` currently lacks. They are the same artifact, so finding it answers both.

**BOTH REASONS ARE NOW SETTLED, AND THEY SETTLED IN OPPOSITE DIRECTIONS** —
which is precisely why neither could substitute for the other. Reason 2 was
confirmed by measurement: the zero was TRUE, and it was about a population
excluding most of the failure. Recorded as stated because the pair is the
reusable part.

1. **Does the selector range over the right set?** The population went 579
   foreign to 0 foreign in one step, and the narrowed selector has never been
   shown capable of reporting non-zero. "No foreign proof source in this body"
   and "my selector cannot see foreign proof sources" are one number. The first
   attempt at this narrowing was wrong by 579, so this is not a hypothetical.
   (Architect, `evt_2ctrn25j8fe39`.)
2. **Is the instrumented seat reached by the trapping dispatch at all?** The
   two DIAG lines report demanded `DenseRange{4362,37}` and
   `DenseRange{3318,37}`; whether either is the discharge for
   `decl:px8f_write_partition::Result` is unestablished. If neither is, the
   zero is a true statement about a population that excludes the failure.
   (Steward, `evt_67jjyszf7g155`.)

   **ANSWERED, AND IT WAS THE SECOND THING.** Measured at `b0a7c2945`
   (runtime-implementer, `evt_6k9xx4trry0hc`): `derive_certified_cuts` runs
   exactly twice — `funcid55` (`authorities=1`, demanded `DenseRange{3318,37}`)
   and `funcid60` (`authorities=593`, demanded `DenseRange{4362,37}`), with zero
   early returns. Of those, `funcid55` emits the trap 0 times and `funcid60`
   emits it 39 times, while **six** functions emit it overall. So the zero was
   a true statement about a two-body population, and five of the six emitters
   never reach the seat.

   The trap could not be mapped to its emitter directly because
   `PlannedTrapIdentity` is a pure dedup index keyed on trap **value**, with no
   source-origin — the same non-discrimination `RT-TRAP-IDENTITY` measured. The
   mapping came from instrumenting the emission seat in `joins.rs` instead.

Both are the zero-versus-never distinction applied one level further out than
where it was correctly applied already.

## HELD AGAINST THE NEXT TOUCH — five items, none yet folded into D0/AC text

This node is `draft` and deliberately unreleased, so nothing can be pulled on a
premise these items change. They are recorded here rather than dissolved into
the deliverables **because a held item that lives only in a thread is not a held
item** — each of the five arrived in conversation after the third amendment, and
this section is where they become durable. **Whoever next touches this node
folds them into D0 and the ACs and deletes this section**; do not release the
node with it still standing.

**1. D0's first conjunct admits benign instances.** The predicate is *a proof
source carrying a foreign producer identity*, evaluated at `:3424` against
`body.authorities`. That map ranges over **every carried constructor**, not over
generated-context ones — measured at `core.rs:14363`, which has no
generated-context gate between its entry and the registrar
(`RT-AUTHORITY-CONTRACT-MISDESCRIBES-ITS-POPULATION`). So an ordinary user
constructor whose identity simply is not the demanded one satisfies D0's first
conjunct. **A non-empty witness search is therefore as misreadable as an empty
one**, and for the mirror-image reason: the predicate no longer separates a
generated-context anomaly from an ordinary non-demanded constructor.

**2. D0 must name the empty-search outcome IN ADVANCE.** "Search the corpus
first" has no stated disposition for finding nothing. Without one, an empty
search reads as *no witness exists* when it equally supports *the selector
cannot see one* — the same zero-versus-never confusion this node already
records one level out. Name the outcomes before the search runs, as
`RT-TRAP-MESSAGE-NAMES-FAMILY-NOT-POPULATION`'s AC-4a does.

**3. AC-4's dead-arm entry needs its evidence attached, not its verdict.** The
`:3429` arm is ruled dead. The entry should carry **how** — derived
structurally, and corroborated at 13 evaluations with 0 fires against a
*reached* control. A verdict with the corroboration stripped off cannot be
re-audited, and a dead-arm claim is exactly the kind that a later change
silently revives.

**4. The reach measurement is scoped to ONE COMPILE, and D0 states it wider.**
"`derive_certified_cuts` runs twice in the entire px8f compile" is a fact about
one compile of one program. D0 currently reads as though two-bodies-reach-the-
seat were a property of the mechanism. **A claim inherits the scope of the site
that was measured, not the scope it was written at** — restate the bound, or
measure a second program.

**5. A BLOCK-PARAMETER PROOF SOURCE REACHES THE SINK CARRYING NO IDENTITY AT
ALL, AND D0 DOES NOT COVER IT.** (Architect, `evt_43kpzzebpxre6`, from a census
of every reader and writer of the map at `b0a7c2945` across `lowering/`
excluding tests.)

The map has **exactly two writers**, both inside
`register_generated_constructor_authority`, reached only from `calls.rs:1895`
and `core.rs:14445` — both constructor-emission sites — plus `units.rs:2752`
taking the whole map wholesale. Every other occurrence is a read. **There is no
propagation step anywhere**, and the map is keyed on `ir::Value`.

⇒ A block parameter is a **fresh** `Value`, distinct from whatever its
predecessors pass to it. Even when a predecessor constructs the value properly
and registers an authority against *its* word, **the authority does not survive
the join** — nothing is dropped and nothing is defective; the authority is
keyed on a word that no longer names the value.

⇒ Such a source misses `body.authorities.get(source)` at `:3423` entirely, so
it **never reaches ARM 1 or ARM 2**. It falls to ARM 3 or ARM 4 and into the
shared `grounds.is_empty()` sink. **That is a concrete, named route into the
fail-open with no foreign identity anywhere in it.** D0's conjunction is about
carrying a foreign identity; this route carries no identity to be foreign, and
the sink absorbs it identically.

**This strengthens the four-arms-one-sink finding rather than disturbing it.**
The collapse was argued to be a property of the loop's exit discipline rather
than of `:3424`; a second, structurally different route to the same
indistinguishable observable is what a shared sink being the defect predicts.

**NOT ESTABLISHED, and it is the whole measurement: whether any real proof
source, in any compile, is ever a block parameter.** The census establishes that
such a source *could not* acquire an authority. It says nothing about whether
one ever arrives. Reachability is not to be asserted from the structural
argument — that is the predicate this arc has already paid four corrections for.

Superseded on the way in: an earlier reading of this miss as *"the scrutinee
arrives across a function boundary, produced in a different function than the
one matching on it"* is **not supported**. `v97 def=Param(block42, 0)` is a
block parameter within `funcid61`; its producers are that block's predecessor
edges, which may well be in the same function. Recorded because the wrong
mechanism was in the thread and read as established.

## Acceptance criteria

- **AC-1 (the witness, and it gates everything else).** The check is
  demonstrated **firing** on a real input — refusal observed, message shown.
  A run in which it does not fire is not evidence it works.
- **AC-2 (agree, does not fire).** On an input where the identities agree, it
  does **not** fire. Show it, from a real compile.
- **AC-3 (THE MIDDLE ARM — a foreign identity that IS legitimately cut).** An
  input carrying a foreign producer identity **whose path is certifiably
  excluded**, on which the check does **not** fire. This is a third input,
  distinct from AC-2's: AC-2's program has no foreign identity at all and
  therefore cannot reach the exclusion path.

  **This AC exists to see the over-refusal direction, which AC-1 and AC-2
  cannot see between them.** An implementation that refuses whenever it meets a
  foreign authority satisfies both of them perfectly and breaks every
  legitimately-cut program. The machinery is real rather than hypothetical:
  `CertifiedInfeasibleEdge` is constructed at `units.rs:3577` and consulted at
  `:3049` and `:3074` (all at `b0a7c2945`), so the arm has code behind it and
  can be wrong.

  **Discharging this by showing the arm is UNREACHABLE is an acceptable and
  valuable result**, not a cop-out — a dead branch in the contract's
  disjunction is worth knowing and simplifies the site to two outcomes. If that
  is the finding, record *why*, with the measurement. What is **not** acceptable
  is the arm living only in AC-4's prose: a mechanism claim in a comment is
  structurally exempt from execution, and an unexercised third arm is the
  unfalsifiable guard this node exists to prevent, one level in.

  (Architect, `evt_7wsf1z27e679a`. Whether such a program is constructible is
  open, and is the same search as D0's — this may cost nothing beyond an AC
  recording that.)
- **AC-4 (THE REPAIR TARGETS THE SINK).** The three outcomes are distinguished
  **where `grounds.is_empty()` is consumed**, not per-arm, and the distinction
  is stated in prose there — a reader who meets `grounds.clear()` later must
  find the reason it is no longer sufficient. **A repair that only changes Arm 1
  does not satisfy this AC**, because a source refused at Arm 1 can reach Arm 4
  and be discarded just as silently.

  Then, and only then, each of the four arms **either supplies a reason into
  the sink or is shown unreachable.** Their standing is: **Arm 2 (`:3429`)
  dead** by the census below (derived, see AC-4a); **Arms 1, 3 and 4 live**, and
  they share the sink. They are not one settled, one dead, and two unknown —
  that framing is superseded.
- **AC-4a (cheap, and it converts three derivations into measurements).**
  Confirm at runtime, across both staged bodies, which of the arms fire. The
  standing instrumentation already reaches all of them, so Arms 3 and 4 come
  for free alongside the `:3429` check.
  - If **`:3429` fires**, the census derivation is wrong and the filter ruling
    below reopens — say so loudly rather than quietly repairing it.
  - If **Arm 3 or Arm 4 fires**, that is a measured second fail-open of Arm 1's
    class, and it is the direct evidence that the sink was the right target.
  - If **neither fires**, say so: the sink repair still stands on the structural
    argument, and *"unreached in this compile"* is not *"unreachable"*.
- **AC-5.** State what drives the discard on each path actually repaired.
  The first filing named the `authority_grounds` filter; that was measured
  wrong for `:3425` and has since been **ruled not a fail-open at all**. Do not
  re-inherit it in either direction — neither as the cause, nor as a site
  needing repair.
- **AC-6.** No regression, green in CI (never a local `--workspace` run;
  COORDINATION §12).

## RULED: the filter is NOT worth repairing, and `:3429` is a DEAD ARM

The mechanism correction above changed what a repair is aimed at, and the
Steward routed the remaining question — *is the filter worth repairing on its
own terms?* — to the Architect rather than presuming an answer. **Ruled at
`evt_5ey9ngwwf0dz8`, by census rather than by opinion.**

`authority_grounds` has **exactly two occurrences under `crates/`** at
`b0a7c2945`, both in `lowering/units.rs` (Steward-verified independently,
repo-wide):

    :3377   the definition -- body.authorities.filter(|a| a.identity == identity)
    :3428   the ONLY consumer -- authority_grounds.get(source), else clear+break

The consumer is reached **only after `:3424` has established
`authority.identity == identity`.** Every source arriving at the lookup is
therefore a key of `authority_grounds` by construction, and there is no second
consumer where that guard is missing.

⇒ **The filter cannot manufacture an absence for anyone. It is not a fail-open;
it is redundant with its only consumer's guard.** It does not get a repair.

⇒ **`:3429` is UNREACHABLE — a dead guard**, sitting inside the node whose whole
subject is dead guards. AC-4 asks which of the four clear-and-break arms the
repair changes and which it leaves: this one is recorded as **dead, with the
derivation**, not repaired.

**The bound on that claim, stated because an unreachability claim is exactly
the kind this arc refuses from others:** it is derived from a two-occurrence
census plus the `:3424` guard. **It is not measured at runtime.** The
instrumentation already standing is in the right place to confirm it — if
`:3429` never fires across both staged bodies, that is the measurement, and it
costs nothing that is not already being printed. Until then the arm is
*derived* dead, and the node says so rather than rounding it to *measured*.

## Contention

Touches `units.rs` discharge and `authority_grounds`. **Contends with any
active backend arc in that plane**, which currently includes the D5b repair —
that is the reason for the `draft` status, not a scheduling nicety. Also
contends with `RT-MUTATION-ARM-CONTROL-COVERAGE` if D0 lands as a mutation arm.

## Related

- `RT-MUTATION-ARM-CONTROL-COVERAGE` — the sibling, and D0's likely home if the
  witness has to be constructed. Same underlying shape: an arm or a guard that
  looks like coverage and drives nothing.
- `RT-OBSERVATION-EXIT-STATUS-LAUNDERS-SIGNAL-DEATH` — the same night's other
  instrument defect.
