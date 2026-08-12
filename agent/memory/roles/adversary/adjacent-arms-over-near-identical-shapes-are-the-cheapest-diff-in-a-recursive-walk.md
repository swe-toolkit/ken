---
name: adjacent-arms-over-near-identical-shapes-are-the-cheapest-diff-in-a-recursive-walk
description: In a hand-written recursive walk, read sibling arms over near-identical shapes side by side — `Closure` walked only its body while `LexicalClosure` walked its captures too, and a difference in whether an arm recurses is either a shape fact or an omission with nothing saying which
---

# Adjacent arms over near-identical shapes are the cheapest diff in a recursive walk

**Measured 2026-08-10 on `fe28ac7d` (`RT-LEXICAL-RECURSOR-CONSUMERS` `D2e`).**

A hand-written walk classified de Bruijn binders across ~15 expression forms.
Two arms sat adjacent:

```rust
RuntimeExpr::Closure { .. } => {
    derive(child(0), &fresh_environment)?;                    // BODY ONLY
}
RuntimeExpr::LexicalClosure { captures, .. } => {
    for p in 0..captures.len() { derive(child(1 + p), environment)? }   // captures, OUTER env
    derive(child(0), &fresh_environment)?;                    // body
}
```

**A capture is evaluated in the enclosing scope** — which is exactly why the
second arm walks captures under the *outer* environment. The first does not.

⇒ **In any hand-written recursive walk, the highest-yield read is sibling arms
over near-identical shapes, side by side.** A difference in *whether an arm
recurses at all* is either a real difference in the shapes' child layout or an
omission, and **the code never says which**. One arm implementing the
convention is what makes the other's silence legible.

This beats reading the walk arm-by-arm for correctness: you are not checking N
arms against a specification, you are checking one arm against its twin, and the
twin is the specification.

## TWICE NOW — the deciding evidence was OUTSIDE the arms, both times

**Second instance 2026-08-11 on `5f9a11f1`.** I noticed the `f32` float-literal
construction omitted `exp_str` where the sibling `f64` branch had an arm for
exactly that. Resolved: **a guard eleven lines above both branches** refuses any
exponent-plus-suffix combination, so the `f32` arms are only ever entered with
`exp_str` empty. The asymmetry is real and **unreachable**.

| instance | asymmetry | what settled it |
|---|---|---|
| `Closure` / `LexicalClosure` | one walked captures, one did not | a **consumer** that must know the shape — downstream |
| `f32` / `f64` literal build | one consumed `exp_str`, one did not | a **guard** before both branches — upstream |

⇒ **The twin is the specification for what the arms DO. It says nothing about
what REACHES them.** Before filing an arm asymmetry, look **upstream for a
guard** and **downstream for a consumer** — the deciding fact has been outside
the arms both times, and reading two siblings and finding one thinner is not
evidence until you know their reachable inputs.

**Both were cheap because I filed them as QUESTIONS with the deciding fact
named.** That framing cost the recipient one lookup each and cost me nothing in
credibility — which is the whole reason a bounded question is a legitimate
artifact for this seat and an asserted defect on the same evidence would not
have been.

⚠ **And the question surfaced a real limitation even while failing as a
finding**: `3.14e5f32` is well-formed in most languages and Ken now refuses it —
deliberate, spec-consistent today, and recorded as an accepted limitation rather
than re-filed later. **A refuted question can still be the thing that gets a
limitation written down.**

## A REFUTED finding can correct the METHOD that produced the claim

**Measured 2026-08-11 on `006730d4`.** I derived a three-file candidate set for
a wildcard that might absorb a new enum variant. **It resolved empty** — a doc
comment, type positions only, and re-exports.

**The value was not in the set; it was in how the set was PICKED.** The ring's
own census had chosen its files by *"files I touched"* — a chosen population
dressed as a measurement — while mine picked by *"files naming the enum"*, which
is the population the claim is about. They replaced theirs with a crate-wide
type-derived census. **The answer did not move; what moved is whether anyone
should have believed it.**

⇒ **A finding that fails can still be worth filing if its derivation is
transferable.** Report *how* you built the population, not only what it
contained — that is the part that survives the instance being empty, and here it
was the whole yield.

**The reusable instrument, and it is the durable half:** for any newly added
enum variant, *the crate compiles, therefore every `match` over that enum either
carries a `_ =>` or does not match at all.* That converts an open-ended
wildcard search into a **decidable two-outcome question over a named file set**,
and it applies at every variant introduction.

**Why that instrument is the right probe** (the Steward's sharpening, and it is
better than my statement of it): when a variant lands, **the compiler has
already answered "is anything non-exhaustive?" everywhere except the sites that
opted out — and those are exactly the sites it cannot tell you about.** A `_ =>`
never errors, so **it never appears in the error list you would otherwise call
complete.** ⇒ Generalise past enums: *a tool's error list is not a coverage
list*, and the opt-out mechanism is always the residual the tool is blind to.
Ask what a clean run structurally could not have reported.

⚠ **And the population defect has its own name now:** *a population selected by
what the AUTHOR DID rather than by what the CLAIM QUANTIFIES OVER.* The reason
it is worth its own shape — **it survives every check that operates inside the
population.** Rigour applied within a wrongly-chosen set cannot detect the
choice, so no amount of care downstream reaches it. The tell is a census whose
scope matches an edit history.

⚠ **My escape hatch was too generous.** I offered "or the file only uses the
type in a signature"; the real answer was weaker still — a **doc comment**.
Naming a charitable alternative is right, but pick the weakest one that would
still refute you, or you overstate what an empty result had to overcome.

## Bound it on the one fact you did not establish

I could not settle whether `Closure` *has* capture child origins in the plan. If
it does, this is a gap; if it does not, the arm is right and the asymmetry is
only apparent. **File it as one question with the deciding fact named**, not as
a defect — [[a-name-free-slot-beats-a-reserved-spelling-and-the-alias-path-is-the-second-reader]]
has the same posture, and it is what keeps a bounded question from being
answered with a refutation that buries it.

## Name the DIRECTION, because it sets the weight

The failure here *under*-reports. For the identity key this derivation feeds,
fewer recorded hypotheses means two distinct producers can key the same — **the
fail-open direction for an identity**, not the safe one. A structural asymmetry
with no direction stated reads as tidiness; with the direction it is sized.

## A disclosed population-of-one still has POSITIONS it cannot reach

The record disclosed its fixture as one shape carrying *"all three depths plus
an adjacent ordinary child"* — a `Let`, a nested `Match`, and a closure **body**.
**A capture is a different POSITION from a body.** So the disclosure was honest
about depth and still left this cell empty.

⇒ When an author discloses a thin population, **do not stop at agreeing it is
thin — ask which positions the named shape does not contain.** The disclosure
tells you the depths that were covered; the grammar tells you the positions that
exist. Same move as
[[the-demonstration-instance-can-be-the-extremal-one]], applied to *where in the
form* rather than *which member of the family*.

## Say where you stopped in a long match

I read down to `Call`/`Effect` and did not reach the end, so I claimed nothing
about exhaustiveness or a catch-all and said so. **In a fifteen-arm walk,
"I reviewed the walk" is a claim you almost certainly cannot support** — name
the arms you read. The unread tail is where COORDINATION §7b's no-`_ =>` rule
would have been the thing to check.
