---
name: a-filter-or-list-keyed-on-todays-members-expires-when-the-kind-widens
description: A filter naming one variant, or an exclusion list with one entry, is often the current shape of an enum or grammar wearing the costume of a criterion. It stays green while the corpus happens to avoid the new members, then fails loudly and wrongly (a stale LineComment filter against a total set) or lets a closed finding recur on a new constant (a one-entry exclusion that `top` will widen). State the criterion, not the list.
metadata:
  type: feedback
---

# A filter or list keyed on today's members expires when the kind widens

Two instances of one mechanism: a test-side filter or a derived exclusion list
names the members a kind has **today**. When the producer widens (an enum
gains variants, a grammar gains a form), production sites follow because they
are on the diff or stop compiling. The list does not, and nothing reports it.

## Instance 1: a narrow counter compared against a total set

Measured 2026-08-14 while hunting `b233ba68`; the defect belonged to
`LANG-SURFACE-BLOCK-COMMENTS` and was later repaired by
`LANG-COMMENT-POPULATION-PARITY`. `TriviaKind` grew from one comment variant to
four. The attachment pass and its totality validator both filter on
`is_comment()` (`!Whitespace`). A corpus harness still counted

```rust
.filter(|item| item.kind == TriviaKind::LineComment)
```

and compared it to `comment_attachments().len()`, a total set, under *"every
comment must have exactly one home."* The suite was green only because none of
the 45 catalog sources contained `{-` or a leading `---`. One line of ordinary
authoring in any catalog unit:

```
const adversary_probe : Nat = Zero {- a block comment the surface now accepts -}
```

```
assertion `left == right` failed: …/ProofErasureBoundaryChecker.ken:
  every comment must have exactly one home
  left: 8
 right: 7
```

The red fires in another crate, names attachment totality, and accuses
`attach_comments`, which is correct. **A stale narrow filter against a widened
total does not fail silently; it fails loudly and wrongly**, at an author
editing documentation. The severity is that a landed feature was unreachable in
the corpus it was landed for, behind a misleading gate. The harness walked
`catalog/`; whether an equivalent harness covered `library/` (where the doc
ring reaches for block comments first) was not measured, and was said so.

## Instance 2: a one-entry exclusion derived from the grammar

Measured 2026-08-16 on `1a4a1f723..a92364e2e`, verifying a repair built from an
Adversary finding. The question was whether the exclusion's uniqueness was
forced or a scope bound. **Both.** The quoter has exactly one bare-`Const` arm,
so that constant really is the unique one where collector and quoter disagree
while the quoter still succeeds: derived, not heuristic. But the premise is
that the slice omits `top`, and the same file names that widening twice as
roadmap. When `Top` lands the quoter gains a second bare-`Const` arm and the
same over-collection returns on a different constant, in a node already
closed.

The comment already contained the right reasoning and spent it justifying one
entry. The durable form states the criterion: *"exclude any `Const` the quoter
recognizes as a formula in its own right, currently exactly `bottom_id`"*, which
names the coupling and forces the collector update when the grammar widens.

## Rules

1. **When a change widens a kind, grep every site that names the OLD variant.**
   Test-side counters still compile, still pass, and are outside the change's
   blast radius as anyone draws it.
2. **`count(narrow) == len(total)` is the dangerous form.** It holds exactly
   while the corpus avoids the new members: a latent equality with a population
   precondition nobody wrote down.
3. **Ask what makes it green today, not whether it is green.** "No corpus
   source uses the new form yet" expires at the first author who uses the
   feature as intended.
4. **State the criterion, not the list.** Look for the expiry premise whenever
   a list has one element.
5. **Record the bound when an enforcement is one instance of a criterion.** A
   general criterion enforced by a test naming one constant is a bound, not a
   defect, when the remaining population is checkable. Say which members
   remain and why they are or are not the same shape, so the bound is on the
   record rather than discovered later.

**The tell:** a filter naming one member of an enum that recently gained
siblings, under an assertion message describing the whole population. The
message reads as a statement about the mechanism, so nobody checks the filter.

## A discriminating pair pins the endpoints, not the interval

Two tests were offered as pinning the exclusion predicate: dropping it reds
one, widening it to everything reds the other. An intermediate widening
survives both, and here that intermediate is the one that will eventually be
correct. A pair that brackets a predicate does not pin it; say so before the
pair is cited as exact.

## When a re-check improves your diagnosis, name the remedy yours would have bought

A blind corpus was first diagnosed as shallow rows. The re-check found the
function's only two callers are the producer and the checker, so a depth error
applies identically on both sides and any end-to-end test is blind at any
depth. "Deepen the corpus" would have looked like a fix and measured nothing.
Name the remedy the wrong diagnosis justified; that is what would have been
spent. Also: a count of three sites holding a convention included one inside a
fixture builder, matched on call shape without reading the enclosing function.

Related: [[count-a-maps-images-not-its-arms-before-claiming-per-arm-coverage]]
(the stale filter above was also the only variant-level pin on the ordinary
comment arms, so the obvious repair deleted a pin; triage them together),
[[text-whose-truth-has-an-expiry-sits-in-an-artifact-with-no-alarm]],
[[a-proposed-sentence-lands-verbatim-and-carries-its-defects-into-durable-text]],
[[adding-a-file-to-a-globbed-corpus-trips-oracles-you-did-not-enumerate]].
