---
name: count-a-maps-images-not-its-arms-before-claiming-per-arm-coverage
description: A four-arm total map whose consumers are two two-way predicates has only two distinguishable images, so a per-arm coverage claim is really a per-class claim — four arms admit six transpositions, only four cross the class boundary, and the two within-class swaps were never in the mutation set
---

# Count a map's images, not its arms, before claiming per-arm coverage

**Measured 2026-08-14 on `b233ba68` (`LANG-TRIVIA-KIND-MAPPING-PIN`), the pin
built from an earlier Adversary finding that `From<CommentKind> for TriviaKind` was
total-with-no-catch-all and therefore closed on completeness but free on
transposition.**

The node's header claims *"All four `CommentKind` arms are pinned in that
configuration, one row per arm."* **Two of the six transpositions are free, and
one of those is free everywhere.**

## THE ARITHMETIC NOBODY DOES

Census every consumer of the mapped-to type. Here, production reads `TriviaKind`
through exactly two predicates, **both two-way**:

- `is_doc_comment()` — `DocLineComment | DocBlockComment`
- `is_comment()` — `!Whitespace`

⇒ **Four arms, two distinguishable images.** An attachment assertion can only
ever separate *doc* from *ordinary*; nothing anywhere separates `DocLine` from
`DocBlock`, or `Line` from `Block`.

| transposition | crosses the class boundary? | detectable? |
|---|---|---|
| `Line` ↔ `DocLine` | yes | yes — in the node's mutation set |
| `Block` ↔ `DocBlock` | yes | yes — in the node's mutation set |
| `Line` ↔ `DocBlock`, `Block` ↔ `DocLine` | yes | yes, unrun |
| **`DocLine` ↔ `DocBlock`** | **no** | **no — measured green** |
| **`Line` ↔ `Block`** | **no** | only by accident, see below |

**Measured, not derived:** swapping the two doc arms and running `--lib` plus
ten test targets gives **169 tests, zero failures**, including both rows of the
pin itself.

⇒ ***An N-arm map is pinned per arm only if the consumers can distinguish N
images. Count the images.*** A row asserting an outcome pins the arm's **class
membership**; two rows that assert the *same* outcome pin the same class twice
and their conjunction separates nothing. Sibling of *a combined mutation is an
existential over its perturbations* — there the observation was under-keyed,
here the observable itself is coarser than the thing observed.

**The tell is two rows of a coverage table asserting identical expected
values.** They read as thorough. They are the signature of a collapsed image.

## AND A MUTATION SET SELECTED FOR "DID IT RED" SELECTS FOR DETECTABILITY

The node ran exactly the two cross-class pairs, both reddened, both were reported
honestly and separately. **The mutations that would have exposed the gap are the
ones nobody would think to run, because they are the ones nothing catches.**

⇒ ***Enumerate the transposition space (`n choose 2`) before choosing which to
run, not after.*** A set assembled by "which swaps break something" is
guaranteed complete on the detectable half and silent on the rest — the
detectable half is what defines the set.

## THE ONE PIN THAT EXISTS CAN BE INCIDENTAL, AND THE OBVIOUS FIX DELETES IT

`Line` ↔ `Block` *does* red — in **one** place, and not the one the node cites
for the `Line` arm. The cited placement fixture passes; the red comes from a
test helper filtering trivia on `kind == TriviaKind::LineComment`, whose
assertion message is *"every comment must have exactly one home"* — **a message
about attachment totality, not about kind mapping.**

That filter is the only site in any crate naming a comment `TriviaKind` variant.
It is also **stale** (see
[[a-filter-or-list-keyed-on-todays-members-expires-when-the-kind-widens]]; since
repaired by `LANG-COMMENT-POPULATION-PARITY`),
and the obviously-correct modernization — widen it to `is_comment()`, matching
the production totality check — **silently removes the only thing that reds under
that transposition.**

⇒ ***When the sole red for a mutation comes from an assertion whose stated
property is a different one, the pin is incidental and its lifetime is the
lifetime of an unrelated line.*** Report it *with* the defect in that line, or
the repair and the deletion arrive in the same commit and only one of them is
noticed.

Second, independent coupling to the same foreign file the node already
records a residual against. **When a claim reaches into another file once, check
whether it reaches in twice** — the first coupling is documented because someone
noticed it; the second is the same author not looking again.

## Measure at the landed ref

The first census for this hunt ran ref-less against the hunting seat's own
stale branch and returned a confident, false picture of `TriviaKind`. That
instance is recorded in
[[a-git-query-answers-a-different-question-correctly-and-never-errors]]: pass
the SHA in every census command.
