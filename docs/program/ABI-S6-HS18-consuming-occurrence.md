# ABI-S6 HS18 — the detached consuming occurrence

Architect, 2026-09-13. This ruling **withholds** the bounded continuation-seat
repair that `runtime-leader` requested and names a shared predicate instead.
It is a component-design ruling, not a release vote. Runtime stays held at
`5d977ac7968dff3763d330690a9b4df530925d79`.

## The observation closed the disjunction

The three-question observation authorized by the
[source-routing determination](ABI-S6-HS18-Q2-source-routing.md) returned
clean answers and they settle it:

1. `v1559` / `2497/83` is **withdrawn** — a static unselected arm, not a
   runtime decode. The executed `Context4` stores `ResourceBracketOk`
   `2282/66`, then the outer `Result::Ok` `2229/36`.
2. The relevant four-arm `ResourceBracketResult` match **is emitted**, at
   `funcid53` / `Context1` / `origin560`, and **its tag query never executes**.
   The `mapping_bracket_body` continuation is **omitted, not mis-fed**.
3. `Context4`'s field 0, actual and expected, is `ResourceBracketOk` `2282/66`
   arity 1.

So arm (a) mis-feed is refuted, arm (c) wrong-arm-inside-the-settlement is
refuted, and `Context4` is correct. Arm (b) is confirmed: `file_body`'s bind
continuation `\outcome. mapping_bracket_body outcome` is never applied, and
`Context4`'s `Ret` payload is forwarded through `Context1` and
`ResponseOwner2` to `Context3`, which demands a `ResourceBodyResult` and
correctly refuses a `Result::Ok`.

That is a precise localization and the observation was worth running. It also
falsifies the characterization I attached to inventory entry 19.

## What I got wrong, and the correction

Writing entry 19 before the localization, I said it was *"the first entry on
this node whose subject is the source program's own routing rather than the
compiler's proof apparatus"* and that it did not share the predicate of the
entries before it.

Both halves are wrong. The source program is fine. The consumer **is emitted**
and the compiler routes around it — that is compiler routing, not source
routing. And once the consumer is known to be emitted-but-unreached, entry 19
stops looking new.

## The predicate

**A value is transported from its producer to a downstream sink while the
consuming occurrence the source places between them is emitted but never
applied. The sink therefore receives a pre-consumption value and refuses it.**

This is a design judgment, not a measurement. Its grounding is the inventory's
own text, quoted:

| Entry | The entry's own words | Fits |
|---|---|---|
| 12 | "loaded `ss7+112` with **no caller-side Result write**" | sink loads a value the producer never delivered through a consumer |
| 13 | "rejected **producer identity** `3380/38` against the **sink/context identity** `4442/38`" | producer and sink identities differ across the transport |
| 14 | "**neither ordinary arm was reached** … its only remaining eliminator was `InvocationReturn`, which **returned the bare call word without projecting**" | consumer arms emitted, unreached, bare forward taken |
| 15 | "identity `1605/874` … belongs to **zero confluence classes**"; route is `TailProducerToRet` | the result joins no consumer at all |
| 16 | "the detached target-3 result retains **neither a live consuming occurrence** nor the caller's source return context" | names the predicate outright |
| 17 | "the static response-owner call **forwards only a runtime word and outer Ret shape**" | bare forward through the same response-owner machinery |
| 19 | four-arm match emitted at `Context1`/`origin560`, **tag query never executes** | consumer emitted, unreached, bare forward taken |

Seven entries. Entry 16 states the predicate in its own words without anyone
having named it as a predicate. Entries 14 and 19 are the same sentence about
two different consumers. Entry 17 and entry 19 run through the same static
response-owner machinery.

**Entry 18 is genuinely separate and should be re-partitioned out.** Its
predicate — a locator, declaration or layout position read as an execution
fact — is about the *verifier*, not the lowering. I previously wrote that
predicate as covering entries 13–18. It covers 18. Entries 13–17 belong with
19.

## This makes the verifier work load-bearing, not wasted

The natural reading of "six repairs did not fix it" is that the effort was
misspent. It was not, and the sequence matters:

The entry-18 verifier repair replaced locators-as-execution-facts with
all-path product-state proofs over the real CFG and memory. That is what makes
the predicate **visible and refusable** rather than certifiable. Before it, a
bare producer-to-sink forward could be admitted by treating declared metadata
as a finished certificate — which is exactly what entry 13 records being
tempted into. After it, the same forward refuses honestly.

So the instrument is now correct and it is correctly refusing a real defect.
The error is not in the instrument, and not in any one of the six repairs. It
is that each repair was aimed at the transport, which is faithfully carrying
what it was given.

## Why a seventh point repair is refused

`runtime-leader` asked for a bounded continuation-seat repair for entry 19.
Written as asked, it would wire `Context1`'s emitted match into the path that
currently forwards around it. That repair would very likely work, advance the
witness, and produce entry 20.

Both rationalizations for writing it anyway are true and neither is a reason:

- **Each previous repair was locally correct.** They were. Entry 14's
  projection, entry 16's symbolic retention and entry 17's receipt work each
  answered their own question correctly. Local correctness is what has kept
  the shared predicate invisible for seven entries.
- **The architecture is still viable.** It is. Nothing here says bounded
  contexts, response owners or the transport machinery are the wrong design,
  and no part of the entry-18 verifier repair should be reverted.

Neither answers the question of why a consuming occurrence keeps being
emitted and not applied.

## The closure to build

The fix is structural and it is one thing:

**A producer-to-sink transport edge must be derived from the consuming-occurrence
chain, never minted independently and then certified.**

Concretely, the property to make unrepresentable rather than checkable: the
planner must not be able to construct a transport from a producer to a sink
that skips a consuming occurrence the source places between them. Today such
an edge can be built and then handed to the verifier to certify; the verifier
refuses, correctly, and the refusal is read as a proof gap and answered with
more certificate machinery. The edge should not be constructible.

Entries 15 and 16 both reached for this one entry at a time — *"treating
either result as Direct would invent a join or substitute Direct for Tail"*
and *"requires private symbolic return-context retention, not fabricated live
control"*. Both are the same instinct applied pointwise. Applied as a closure
it is a single rule about what the planner may emit.

**Retain everything already proved.** The entry-18 verifier repair, the Q1
resume-exit repair, every grid and mutation control, the identity domains, the
stack and bound provisions, and the pre-object refusal all stand unchanged.
This is a recut of the repair target, not of the work already banked.

## A falsifiable prediction, offered so it can be refuted

If the predicate is right, **px8f is the same defect, not an independent one.**

px8f publishes a genuine two-field `Vis` `3380/38` at `f55`/`v77` where six
`u3:60` obligations demand `Ret` `4442/38`. Under the predicate, a `Vis` is
precisely a pre-consumption value: the occurrence that should run the effect
and continue is detached, so the raw `Vis` is presented where the post-
consumption `Ret` is demanded. Same shape as `Result::Ok` arriving where
`ResourceBodyResult` is demanded.

**The measurement that decides it:** for one of the six `u3:60` obligations,
determine whether the consuming occurrence that should eliminate the `Vis` is
emitted and unreached, as in entry 19, or genuinely absent from the emitted
program. Emitted-and-unreached confirms the predicate covers px8f. Genuinely
absent, or a third shape, means the predicate is narrower than stated here and
I want that back.

This prediction changes nothing about the px8f refusal, which stays honest
either way. It only changes whether one closure retires both witnesses or one.

## Disposition

- **The bounded continuation-seat repair is WITHHELD.** Do not write it.
- **No revert.** Checkpoint `5d977ac79` stays held and intact; the verifier
  repair stands and should land on its own merits once its two named residuals
  are addressed.
- **The Steward owns the recut scope.** This ruling supplies the predicate and
  the closure requirement; framing is the Steward's.
- **A research prior-art advisory is recommended, in parallel and
  non-blocking**, scoped to how production compilers structurally prevent a
  lowered continuation application from being bypassed by a direct
  producer-to-sink forward. *"Prior art has nothing new here"* is a first-class
  answer and a useful one at this depth.
- Design authority for the closure returns here once the Steward frames it.

## Preserved

Q1 `source.rs` stays blob `38ec787dac261f02d46463ee1e9fca555c76d694`. The px8f
refusal stays honest: actual `3380/38` against demanded `4442/38`, five
non-promoted cuts unchanged. Every grid standing at `01d2ccb11` and
`5d977ac79` stays standing. No QA request, no merge Decision, no candidate
status, no weakening of the pre-object refusal.
