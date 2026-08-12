---
name: a-lexer-guard-on-character-adjacency-is-defeated-by-one-space
description: A repair keyed on the character immediately preceding a token is stateless and correct — and one space restores the original defect, which matters most in a crate that also holds the formatter
---

# A lexer guard on character adjacency is defeated by one space

**Measured 2026-08-11 on `1b0b1f66` (`LANG-SURFACE-PAIR`).**

The number scanner had been turning `p.1.2` into `Ident, Dot, FloatLit(1.2)`,
swallowing a projection into a valid literal. The repair:

```rust
let follows_dot = self.src[..start].chars().next_back() == Some('.');
if !follows_dot { /* scan a float */ }
```

**I expected a stale mutable flag and there is none** — it is derived fresh from
the source at each scan, so nothing can carry over. That construction is right,
and worth recognising: *a guard recomputed from the input has no staleness
surface at all.*

**But it tests CHARACTER adjacency, so one space defeats it.** `p. 1.2` has a
space before the `1`, `follows_dot` is false, the float is scanned, and the
original defect is intact behind whitespace the grammar probably ignores.

⇒ **For any lexer guard keyed on neighbouring characters, write the same input
with a space inserted at the boundary and re-read the rule.** The guard's
condition is lexical; the language's rule is grammatical; **whitespace is
exactly where those two come apart**, and a scanner is the one place a
whitespace difference can change meaning silently rather than being normalised
away.

## CORRECTION — I called it silent, and it is a loud refusal

**The finding was right and my severity was wrong.** I wrote that a well-formed
float is produced *"so the parser sees a projection of a float rather than a
chained projection, and nothing reports it."* Measured on the unmodified tree:
the projection loop's lookahead admits only `Ident` or `Nat(1 | 2)`, so a
`FloatLit` builds **no projection at all** — exact
`ParseError { msg: "unexpected token after expression: Dot" }`.

**I asserted a direction from the producer and never asked the consumer.** That
a valid float token is *emitted* says nothing about whether anything downstream
accepts it; **one hop further — what the next stage's lookahead admits —
settles it, and I stopped at the token.**

⇒ This is the twin of
[[an-error-in-the-safe-direction-is-a-claim-about-what-you-did-not-measure]],
inverted: there I called an error safe without measuring the far end, here I
called one **dangerous** without measuring it. **A direction is a comparison in
both directions.** Claiming "silent" is a claim that *no stage reports it*,
which is a statement about every consumer, not about the token.

Flagging my own severity as conditional is what kept this a frame correction
rather than a bad call — **but "conditional" was doing work the measurement
should have done**, and the measurement was one lookahead away.

## THE WEIGHTING AXIS WAS EMPTY — ask a tool's reachable OUTPUT, not its capability

I raised this above a nit on the grounds that the formatter and lossless printer
share the crate, so one emitting a space after a projection dot would be a
meaning-changing reformat. **Measured: the class has no member.** The lossless
printer can only reproduce input that **already parsed**, and the failing
spelling never parses; kenfmt canonicalizes the spaced forms away. Neither can
produce it.

⇒ **A hazard predicated on "if tool X emitted Y" needs X's REACHABLE OUTPUT SET,
not X's expressive capability.** For any round-tripping tool the range is
bounded by what its input side accepted — so a hazard requiring it to emit a
non-parsing form is **empty by construction**, and that is one question asked
before the weighting, not after.

⚠ **And notice which half was empty**: the weighting was my stated reason for
raising it, and the finding stood without it. **A finding that needs an
amplifier is worth re-reading for whether the amplifier is the claim.**

## Weight it by what else lives in the crate

A whitespace-sensitive lexer rule is a nit until you notice **the formatter and
the lossless printer are in the same twelve paths**. A formatter that ever
emitted or preserved a space after a projection dot would turn a correct program
into a differently-parsing one — a **meaning-changing reformat**, the class
[[corpus-property-gate-only-as-strong-as-the-corpus]] exists for.

⇒ **Check what other tools in the same crate produce the text your lexer rule
reads.** A lexer's input is not only what users type.

## State the bound as the deciding parse

I did not establish that the spaced form is grammatical, and **if it is not,
this is nothing.** Saying that plainly, and naming the one-parse test that
settles it, is what makes a conditional finding cheap rather than an alarm.

**And give the control in the property's own terms:** assert that `p.1.2` and
`p. 1.2` produce the **same token stream**. That is what the guard is reaching
for and states nowhere — and if they *must* differ, the guard is right and
deserves a sentence saying so, because **character-adjacency in a lexer reads as
an accident rather than a decision** unless someone writes down that it was one.
