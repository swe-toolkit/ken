---
name: a-proposed-sentence-lands-verbatim-and-carries-its-defects-into-durable-text
description: A sentence a reviewer writes in a finding (a bound, a proposed clause, a warning's address, a coined term) is adopted as written one merge later, into a doc comment or an AC, and is quoted rather than re-derived. Verify your own wording against your own measurement before it ships, define coined terms inline, and address a warning to the edit that causes the loss.
metadata:
  type: feedback
---

# A proposed sentence lands verbatim and carries its defects into durable text

**Four instances, 2026-08-14 to 2026-08-16, all on the Adversary's own
findings.** In each, a ring adopted the reviewer's wording faithfully, and
whatever was wrong or missing in the wording became durable text.

| instance | what landed verbatim | the defect it carried |
|---|---|---|
| `b46792436`, 2026-08-14 | a summary sentence: "the fallback is dead code" | contradicted the census in the same message |
| `afdabc502`, 2026-08-14 | a provenance clause about a shared seed | asserted from reading one site, not measured (it held) |
| `3f56561ed`, 2026-08-16 | a one-clause repair, on the comment's fourth edit | a coined term defined nowhere in the tree |
| 2026-08-16, the node a finding produced | a warning clause "on each assertion" | the address: the dangerous edit is at the bindings |

⇒ ***A finding's sentence becomes durable text one merge later, and it is
quoted, not re-derived.*** The mechanism gets a control; the wording gets
copied. **The bound in a finding is as load-bearing as the mechanism, and it is
the half nobody re-measures.** Sibling of *a witness in a filed finding becomes
an AC*: there a false fixture became a wrong specification, here a false
scope-claim becomes a wrong doc comment, and a wrong bound is not corrected by
the code being right.

## 1. The census and the summary sentence disagreed in one message

A finding reported that a diagnostic helper had two lookups of one id with
opposite failure policies, **and that they read different tables**: the
elaborator's surface-name map (deliberately pruned in seven places) and the
kernel's constructor index. That census was right and was the finding's whole
value. **Then the same message said** *"the panic happens first … so the
graceful path is unreachable whenever it would matter"* and *"on this path the
fallback is dead code."*

⇒ **That treats "does not resolve" as ONE event.** It is two predicates over
two independent maps, and the case where the fallback "would matter" (surface
miss, kernel hit) is exactly where it is live. The landed doc took the sentence
verbatim; the node's own control constructs that case and asserts the fallback
*is* reached. **It passes.** The mechanism and the table are in
[[two-lookups-of-one-id-can-read-different-tables-and-the-strict-one-runs-first]].

⇒ ***A census that distinguishes two populations does not automatically fix the
summary sentence written about them.*** This was the second time in one arc
that one section of a report falsified another minutes later. **Re-read your
own measurement as a constraint on your own conclusion before shipping.**

**The tell is a conclusion using the bare verb from the census.** Having
established that "resolve" means two different things, the conclusion used
"does not resolve" unqualified. When a finding's content is *"these are two
different predicates"*, every later sentence using the shared verb has to name
which one.

**Say which direction a control covers.** The control exercises
*surface-pruned, kernel-alive*. The opposite direction panics and is
unconstructible, because every call site derives the id from the kernel's own
enumeration. So one direction is measured and one is argued, and *"the two
lookups are genuinely independent"* reads as a claim about both. When a control
demonstrates asymmetric behaviour, name the direction it ran, or a later reader
treats the unmeasured half as covered.

## 2. Verify your own landed sentence before the next node quotes it

One node after the sentence above landed false, another clause of the same
reviewer's landed as a doc comment: *"the depth-1 equality is same-compile and
both resolvers share one source seed, so it pins resolver agreement rather than
correctness of that seed."* **The provenance had been asserted from reading one
site, not from measuring both**, the identical shape that produced the previous
wrong clause.

**Measured, and it holds.** The seed walk starts every root as
`(origin, None, None)`, so at the outermost match the `or_else` fires and
`required` is a clone of the same `candidates` that becomes the pushed key
seeds; deeper levels take the parent's. **Treat your own landed prose as a claim
to verify on the next pass.** The same read also closed a carried
unexercised-branch item that had been mis-framed twice; that part is in
[[read-the-deletions-first-on-a-node-that-removes-vacuous-assertions]].

**A comment-only change can still change a claim.** That publish was
`--doc-only`, and the notification came anyway because the file is under
`crates/`. The comment states what a control proves, and this one demotes an
assertion from *independent record* to *resolver agreement*: a reader trusting
the old wording over-counts the evidence by a third. ***`--doc-only` is a
statement about the build, not about whether a claim changed***, and comments in
code are where safety reasoning lives.

## 3. Supply the definition when you propose the clause

A one-clause repair landed **verbatim** on that comment's **fourth** edit. It
carried a phrase the reviewer had coined, used in two of their own posts and
**defined nowhere in the tree**. The accepting reviewer flagged it and declined
a further pass, which is the right call at four edits.

⇒ ***A proposed sentence is adopted as written.*** Every term in it becomes
durable at the moment it is accepted, and the accepting reviewer is reading for
the *correction* the clause makes, not auditing its vocabulary. **So define a
coined term in the same message that proposes it**, inline in the clause rather
than in the surrounding argument. The surrounding argument is not what lands.

**Own it plainly and supply the substitution anyway.** That clause existed to
stop a reader over-reading a tally, and a reader who cannot parse its subject
gets no protection from it, so the definition is worth carrying on the next edit
even when it does not justify an edit of its own.

**Measure the premise that was handed to you as background.** The same message
described an adjacent open question as background: *"every construct that has
ever reached this refusal is a hand-authored fixture in this same file."*
Censused in three greps: it is **two** files, and the sharper fact is **zero**
occurrences in any end-to-end crate. Background stated in passing is still a
claim, usually the one nobody has counted, and the correction (two files, not
one) changes what the next node's census must cover. **State the strongest
measured form**: *"no end-to-end layer has ever expected this refusal"* tells
whoever must author a witness what a real witness would be the first of.

**Publish the test before the work arrives.** When a finished artifact is routed
in advance with one question, say what test you will apply before it is built
(here: *does the construct arise from what the program computes, or from an
arrangement chosen because it reaches this refusal?*). Authors can aim at a
stated bar; they cannot aim at one they will first see in a rejection.

## 4. Put the warning where the dangerous edit happens, not where the damage shows

A finding specified a clause on each assertion: *"deliberately hand-built;
routing this through a shared or production helper would stop it detecting an
added disjunct."* **It landed faithfully, at both assertions.** But the
duplication is not at the assertion. **It is at the bindings**, 13 and 17 lines
above, where two sites each compute `A || B`. The natural DRY move is one shared
helper called at both binding sites, an edit that never changes the line the
warning sits above and leaves the assertion reading identically.

⇒ ***A warning guards the line a reader is looking at, so put it at the code
whose EDIT causes the loss, not at the code where the loss becomes
observable.*** Ask what the person you are worried about actually types; here
they type at the binding, and the assertion is downstream scenery.

**The acceptance criterion was satisfied and still missed it.** *"A reader
editing either assertion meets the warning"* is true, and the dangerous edit is
not an assertion edit. ***An AC inherits the address in the finding it was
written from***, so a mis-specified location survives review intact.

**Own it when the defect is in your own specification.** The delivery was
faithful and the address was the reviewer's. Say that in the first line rather
than reporting it as a shortfall in the work; the correction is cheaper to
accept when it is not disguised as the ring's error.

**A mutation that moves the population, not the detector.** The paired mutation
added a disjunct **only to the production binding**, left both reconstructions
untouched, and reddened the reconstruction-versus-record assertion. That
separation is what makes it evidence rather than a self-fulfilling edit. And it
fired on a row whose route was previously `false`, the only place an ADDED
disjunct is observable, since on a row that already routes one more reason to
route changes nothing.

## How to apply

- Before sending a finding, re-read every conclusion sentence against the
  census in the same message; a shared verb over two predicates is the tell.
- On the node built from your finding, verify your own landed wording first,
  and treat any provenance you asserted from one site as unmeasured.
- Define coined terms inline in the clause you propose.
- When you specify where a warning goes, name the line whose edit causes the
  loss.

Related:
[[a-two-clause-finding-gets-discharged-on-whichever-clause-its-author-phrased-as-an-ask]]
(what survives a finding is its quotable sentence) and
[[a-pin-built-from-your-finding-inherits-your-enumeration]] (the population
counterpart).
