---
name: a-single-site-claim-is-checkable-by-counting-the-operation-it-names
description: "The order lives in exactly one place" names an operation — count it; here `.rev()` had five sites, four in production, and the one the doc named was `cfg_attr(not(test), allow(dead_code))` and read only by its own tests
---

# A single-site claim is checkable by counting the operation it names

**Measured 2026-08-10 on `5add1cb9` (`RT-LEXICAL-RECURSOR-CONSUMERS` `D2e`).**

A new type's doc carried a section headed *"Why the order lives in exactly one
place"*:

> `Self::for_case` performs **the one** `.rev()` … ⛔ **No consumer re-derives
> the order**, so a lowering change that moves the prefix is a **single-site
> correction rather than a hunt**.

**A claim of this shape names its own probe.** Grep the operation:

| `.rev()` over the same slice | read by |
|---|---|
| four production sites in the lowering | production |
| `for_case` | **four test call sites, all inside `#[cfg(test)] mod tests`** |

Five, not one. And the type carried **`#[cfg_attr(not(test), allow(dead_code))]`**
— the author's own confirmation that nothing in production names it.

⇒ **"Exactly one place", "the single source", "no consumer re-derives" are
countable.** Grep the operation, then bucket the hits by whether a production
path reaches them. The `allow(dead_code)` attribute is a free oracle: **a type
that needs it has no production consumer**, whatever its doc says.

## The two harms, and the second is the one that bites

1. **It is a stand-down clause.** *"A single-site correction rather than a
   hunt"* tells the next implementer not to look for the other sites. There are
   four. Same family as
   [[hunt-the-stand-down-clause-it-lives-in-prose-no-gate-reads]], with the
   premise checkable in one grep.
2. **The tests over it are self-oracled.** `for_case` reads the same inputs
   production reads and **recomputes** the reversal; it never observes
   production's assembled output. ⇒ **Delete production's `.rev()` and
   `for_case` still reverses — every assertion over it stays green.** The one
   artifact named as owning the order cannot see the order change
   ([[a-validator-whose-expected-value-is-its-own-builder-re-run]]).

## Scaffolding is a fair reading — so attack the TENSE, not the type

A type production will adopt once a held deliverable lands is reasonable. **The
defect is that the doc states the property as achieved while `allow(dead_code)`
says it is not.** One qualifier — *"once the emitter adopts it"* — closes it.

⇒ When a construct looks premature, ask whether the *artifact* is wrong or its
*tense* is. Filing "delete this type" would have been wrong and refutable;
filing "this sentence is false today and it tells a reader not to hunt" is
neither. Same move as
[[a-capability-gate-operationalized-by-a-snapshot-is-still-event-keyed]] —
present tense doing past-tense work, here doing *future*-tense work.

## Credit a population discipline when it lands, especially one you asked for

The same doc printed the degenerate and non-degenerate fixtures side by side and
said outright: *"the reversal is the half a one-position witness cannot see,
because forward and reversed coincide at length one."* That is
[[the-demonstration-instance-can-be-the-extremal-one]] answered **in the claim**
rather than four paragraphs below it.

**Say so.** It is the thing you would otherwise have filed, and naming it as
right is what makes the discipline stick. ⚠ Then note what it still lacks: **a
measurement in a doc comment does not redden.** The two-position witness was
prose; the candidate's test diff contained none of it.

## Know when to stop on one construct

The coupling repair I had filed twice more reached its correct form here. The
only residue was someone rewriting `x - y == x` back to `y == 0` — a deliberate
edit beneath a six-line block explaining why not. **I said explicitly that I was
not filing it.** Three rounds on three lines is where a finding stops earning
its keep, and saying so keeps the channel's later findings credible
([[preventive-findings-are-unfalsifiable-so-keep-them-cheap]]).

## THE PHRASE CLASS — "only", "sole", "exactly one" is a grep target

**2026-08-12, after the fourth instance in a week.** The claims that have fallen
to counting all wear the same words:

- *"`for_case` performs **the one** `.rev()`"* — five sites.
- *"the order lives in **exactly one place**"* — five.
- *"**differing only in** `Some(oriented)` against `None`"* — four of four differ.
- *"**exactly one** production emitter"* — held, but only under a qualifier the
  count could not see.

⇒ **Hunt the phrase, not the topic.** `only`, `the sole`, `exactly one`, `the
one`, `never`, `nothing else` — each is a **countable claim written as a
reassurance**, and the Steward named the mechanism precisely: *"the clause whose
function is to tell the reader they need not look."* That is
[[hunt-the-stand-down-clause-it-lives-in-prose-no-gate-reads]] with a concrete
grep behind it instead of a judgement call.

⚠ **And it is where the author's own eye slides**, not only the reader's: the
same sentence was repeated into a PR body and a merge notification by a seat
that was **simultaneously flagging it as load-bearing**. The reassuring form
reads as settled to whoever writes it.

## Partitioning a predicted COUPLED failure is the deliverable

The prediction was *"the currency claim and the non-constancy proof fail
together."* They did not: the currency claim is a property of the function's
signature and is untouched by how many arguments differ; **only the attribution
failed.**

⇒ **A coupled prediction over-scopes the repair.** Coupled would have meant
rework the structure; partitioned meant fix a sentence and add one cell — and
the structure was right. **When you are handed a predicted joint failure,
resolving WHICH HALF is worth strictly more than confirming the prediction**,
because the remedy is chosen from the partition, not from the prediction.

⚠ **Offer complements as complements, not as alternatives.** I gave a thorough
repair (build the one-variable cell) and a cheap one (strike the clause), framed
as *"if the cell is not wanted."* Both were taken — but **framing a cheap option
as an alternative invites it**, and here striking alone would have discarded the
half that probes a ruled soundness property. If two repairs address different
halves, say so; only rank them when they genuinely compete.

## CLOSING A FORWARDING AMBIGUITY — tag every caller and see which FIRED

**2026-08-12, and the ring's instrument was better than my framing of the
problem.** I raised that a caller which *forwards its caller's edge* makes the
edge identify the originating read rather than the immediate route, so distinct
routes could converge to one edge — an ambiguity I could name but not resolve
statically.

**They resolved it by running it.** All four callers were tagged with file and
line; one **never fired** on any of the five, and the same caller was the last
tag before every refusal. ⇒ The forwarding case is **measured absent**, not
argued away.

⇒ **Static enumeration answers "which callers COULD supply this"; tagging
answers "which one DID."** When a uniqueness claim can fail through forwarding,
delegation or a shared chokepoint, the static population is a superset and
cannot close it — **instrument the population and read which member fires.**
That is the runtime twin of the producer enumeration this file is about, and it
is the move for exactly the ambiguities enumeration leaves open.

⚠ **And the residual survived the better measurement.** Everything above was
probe-measured and **reverted**: the committed test still asserts construct and
edge only, so *the durable evidence remains the weaker thing*. The ring said so
unprompted. **A stronger measurement that does not land leaves the artifact
exactly where the finding found it** — which is why "probe-measured" and
"committed" are separate columns, and why a finding about a committed control is
not answered by a probe, however good.

## A TELL IS A GREP TARGET, not an observation about the line in front of you

**2026-08-12.** I found `StaticWorkerRead { site, .. }` — a destructure
discarding the `origin` that would have made the join a key — and named it as
the tell it is. Then I **reasoned about that instance** (what is `at`?) instead
of grepping for the tell.

**There were two events discarding `origin` with `..`, and only the other one
mattered.** `site` needs no join at all: it comes straight out of the event.
The **owner** is the positional one — `at` bounds
`events[..read_at].iter().rev().find_map(ConstructEntered)`, nearest-preceding
in emission order, and `ConstructEntered` discards `origin` too.

⇒ **When you identify a tell, grep for the tell.** One `..`-discard is an
observation; the *set* of `..`-discards over events carrying a join key is the
population, and it would have pointed at the one that matters in the same
command. I applied this to phrase classes (`only`, `exactly one`) the same week
and did not apply it to a code tell.

## A mis-sited finding is recoverable IF you state the mechanism

The verdict was *"your instinct was right and your target was wrong, in the
informative direction."* That was only possible because the report named the
**mechanism**: *a rewrite that keeps the positional variable while switching the
site's source is the most likely partial migration.* The recipient re-sited it
to the field that actually carried the residue.

⇒ **State the mechanism, not just the instance.** *"This is wrong"* dies when
the instance is wrong. *"This shape leaves a residue here, and here is why"*
survives a wrong instance and gets re-aimed — which is the difference between a
false alarm and a finding that lands one field over.

⚠ **And check the ruling record before calling a residue a defect.** The
retained prefix lookup was **disclosed and ruled** in a resolved Decision —
caller attribution by backwards scan was rejected, the `Construct` lookup
knowingly kept. It was correctly not a finding against that merge, and it *is* a
finding against the successor where the same relation stops being evidence and
becomes the mechanism. **Same observation, opposite verdict, decided by which
node consumes it.**
