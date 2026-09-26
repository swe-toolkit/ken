---
name: unifying-two-scanners-moves-the-seam-to-where-the-one-becomes-many
description: Deleting four parallel scanners for one shared classifier closes end-divergence by construction — and the classification still has to become a behaviour, so the seam moves to the `From` impl at the consumer boundary, where a total match with no catch-all closes the completeness axis and leaves transposition free
metadata:
  type: feedback
---

# Unifying two scanners moves the seam to where the one becomes many

**Measured 2026-08-13 on `457c51ee` (`LANG-COMMENT-CLASSIFIER-SHARED`), the
repair for a durability finding about two hand-mirrored scanners.**

Four parallel scanners deleted, one `classify_comment` with two call sites.
**The repair is real and it is the strongest available form:** both callers bind
`next` straight from one return and neither re-scans, so there is no second
comment end to disagree with the first. **Divergence is unrepresentable rather
than watched** — and on the kind axis it is better still, because only one caller
consumes the kind at all (the lexer binds `_kind`). One authority, one consumer.

⇒ **And the seam did not vanish. It moved one hop.** A classification still has
to become a **behaviour**, and that conversion — `From<CommentKind> for
TriviaKind` — is now the sole place it happens.

⇒ ***When a repair makes two things one, ask where the one becomes many again.***
The unified value has consumers, and the **consumer boundary is the new seam**.
A finding that stops at *"the duplication is gone"* has audited the half the
repair was aimed at.

## TOTAL-WITH-NO-CATCH-ALL CLOSES COMPLETENESS AND LEAVES TRANSPOSITION FREE

The `From` impl is four explicit arms, **no `_`**. Two different edits, two
different protections, and only one is covered:

| edit | caught by the exhaustive match? |
|---|---|
| **add** a variant | **yes** — compile error, this is what "exhaustive by construction" buys |
| **transpose** two arms (`Block ↔ DocBlock`) | **no** — compiles, and nothing else looks |

⇒ ***"Exhaustive" is a completeness property, not a correctness one.*** It
guarantees every variant is *handled*, never that any variant is handled
*correctly*. COORDINATION §7's exhaustive-by-construction rule is about the first
and is routinely read as covering both.

**Measure the second axis separately: does any test name the produced value?**
Here `git grep` for every `TriviaKind::` variant across `tests/` returned
**nothing** — so the per-arm mapping is asserted nowhere directly. Sibling of
[[exhaustiveness-comes-from-an-unguarded-arm-not-from-the-match]],
which is the same rule failing on the *other* axis.

## A FIXTURE WHERE TWO RULES AGREE CANNOT SAY WHICH ONE FIRED

The attachment test does cover the block form and asserts the comment attaches
Leading to the **following** declaration. Under a transposition the comment
would be reclassified as ordinary and fall to *"the existing positional
Leading/Trailing/Interstitial heuristic"*.

⇒ **The test reddens only if the positional heuristic answers differently.** If
it also says Leading-on-following, the fixture is one where the **doc rule and
the positional rule agree**, and the assertion cannot tell which produced the
outcome. Its name promises *discriminatingly* — and it does discriminate
*following versus preceding*, which is not the distinction the mapping needs
([[the-demonstration-instance-can-be-the-extremal-one]];
[[a-non-degenerate-pair-fails-to-fail-if-the-assertion-cannot-tell-the-halves-apart]]).

**I did not read the heuristic, and said so.** One read decides whether this is
a gap or already pinned — **name it as unrun rather than tracing a witness**, the
correction from the previous pass applied at the first opportunity to repeat it.

## "REDUNDANT, X COVERS IT" NAMES AN IDIOM WHERE WHAT MATTERS IS A PROPERTY

A separate strengthening was judged redundant because *"the value-equality idiom
already exists one file over."* It does — on a **different path** (through the
guarded parser site rather than around it), with a **different character class**
(a non-control escape, chosen precisely because control characters are rejected
there), establishing a **different property**.

⇒ **The two share one word and nothing else.** ***The idiom being present is not
the property being covered.*** When a coverage claim cites a test elsewhere, put
the two side by side on **path, input class and asserted property** — a
resemblance in shape is what makes the claim feel safe.

**Separate the disposition from its grounds.** Not spending a code change on a
low-severity item is defensible; recording *"redundant, X covers it"* is false,
and **the grounds are what a later reader inherits** when they wonder why a
control asserts success rather than value. Offer the corrected sentence rather
than the row — same shape as
[[a-precise-fact-can-live-in-an-artifact-its-reader-never-opens]], where the
outcome was right and the reason could not fire.

## TO CLAIM A PROPERTY IS UNASSERTED, GREP THE PROPERTY — NOT THE REPORTED SITE

**The "redundant" judgment resolved against me, and my argument was still
right.** D0 genuinely cannot cover the property — different path, different
character class — **and a row asserting it existed anyway**, in a *different
file*, because a no-amend control forbade strengthening the original in place.

**I went back to the site where the defect was reported, found it unchanged, and
concluded the property was uncovered.** The site *is* still weak; the conclusion
was a claim over **the whole test surface** established from **one location**.

⇒ ***To claim a property is unasserted, grep for the PROPERTY across the test
surface — never re-read the site where the defect was reported.*** A repair can
be legitimately relocated: a no-amend control, a new file, a different crate.
One grep for the fixture string or the matched constructor would have found it.

**Third instance in one week of the same shape** — a `-- crates/`-scoped grep
carrying an unscoped conclusion
([[my-reporting-scope-silently-became-my-measurement-scope]]), a line probe
carrying a claim about a table header
([[a-pattern-match-is-evidence-about-what-encloses-it]]),
and now a one-file read carrying a claim about a suite. **The population is
always narrower than the sentence, and the sentence is always a negative.**

**And the counterpart is worth naming as a real hazard, not just my error: a
repair relocated to satisfy a no-amend control is INVISIBLE to a reviewer
checking the reported site.** The verify loop's default is *"go look where the
defect was"*, and that is exactly where a legitimately-relocated repair is not.
⇒ **A frame that forbids amending in place owes the reviewer the landing
location.**

## THE CORRECTIVE, RUN ON THE SAME SHAPE THAT KEEPS FAILING

**Offered a choice between hunting a frame with no code and confirming a known
gap, I took the gap — because confirming it is a NEGATIVE claim over a test
surface, which is the exact shape I had got wrong three times that week.**

⇒ **Ran the property grep first, over the whole surface, not the file I expected
it in.** Two sweeps: every `CommentPlacement` assertion (five, in **two** files),
and every fixture of the discriminating shape.

**The gap confirmed — and the grep WIDENED it.** `Trailing` is asserted for
exactly one comment form; doc forms are asserted at exactly one placement, in the
one configuration where the two rules agree. So the `From` impl has **two**
transposable pairs that flip attachment, and the rider's witness pins only one.
The second is reachable by the identical fixture with a different opener, at the
cost of one more iteration of a loop that already exists.

⇒ ***A surface-wide grep run to confirm a known gap will often resize it.***
Confirming is not a formality — the population you sweep to verify one claim is
the same population that answers *"where else?"*, and it is already in hand.
**Take the confirm task over the predict task when your recent errors are
population errors**: it is the same work with a measurement attached.

**And it showed the relocation shape from the other side.** `Trailing` lives in
a *different test file* from the suite under review, so a reviewer checking the
obvious file sees only `Leading` and would conclude the axis is thinly covered.
**Coverage is as relocatable as a repair** — the mirror of
[[a-repair-relocated-by-a-no-amend-control-is-invisible-at-the-reported-site]];
here nothing moved, it was simply never where the subject was.

### A PROVENANCE LABEL CAN BECOME AN ACCEPTANCE CRITERION

I flagged the widening as **derived from someone else's read, not independently
executed**. That label did not merely get noted — **it changed the node's
`AC-2`**: the mutation must now be run **separately per pair, both reported**,
and the frame states which half is measured and which is derived so whoever
takes it knows which one they are checking.

⇒ ***Saying "this half is derived" is actionable information, not a hedge.***
It tells the next seat exactly where the evidence thins, and the natural
response is an AC that converts the derivation into a run. **A bound written as
provenance travels further than a bound written as modesty.**

### A COMBINED MUTATION CANNOT DISTINGUISH "BOTH PINNED" FROM "ONE PINNED TWICE"

The Steward's refinement, and it is a measurement rule I did not have. Swapping
**both** arm pairs at once and observing a red proves only that *something*
reddened. **Run one pair, observe its own row red, restore; then the other.**

⇒ **Whenever a mutation perturbs more than one thing, the red is an existential
over the perturbations.** Same family as *a count over a population is not a
pairing within it* — the observation has to be keyed to the individual
perturbation or it collapses to *"at least one of these matters."*

### THIRD CONFIRMATION: A RIDER WITH NO HOST IS OWNED BY NOBODY

The Steward had said twice this would ride on the next node in the crate, then
filed it as its own XS node because the intended host had grown to three files
for unrelated reasons. **"A rider with no suitable host is an obligation
recorded in prose and owned by nobody"** — the same failure this arc has now
produced three times, in three carriers.

⇒ **When a disposition is "it will ride on X", the check is whether X exists,
is in scope, and is still the right size.** All three drift, and the rider is
not re-examined when they do
([[a-precise-fact-can-live-in-an-artifact-its-reader-never-opens]]).

## STOP RE-LISTING AN UNHUNTED ITEM YOU KEEP NOT READING

`error.rs`'s `Display` text has now appeared in three of my coverage statements
without ever being read. ⇒ **A carried-unhunted list is a commitment, not a
disclaimer.** An item that survives three passes is either worth one read or
worth dropping from the list — **repeating it converts an honest bound into
noise**, and the list's value is that a reader can trust every line of it is
live.
