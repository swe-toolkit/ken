---
name: a-population-held-at-a-degenerate-value-cannot-see-that-axis
description: A control is blind on every axis its rows hold constant, and on any axis held at the value where the right and wrong behaviours coincide (index == depth, arity 0, one binder, only wrong shapes and one correct artifact). Ask which parameters the row set holds fixed, count per axis rather than per row, and run a proposed fixture's arithmetic at its own boundary before accepting it.
metadata:
  type: feedback
---

# A population held at a degenerate value cannot see that axis

A control discriminates two behaviours only on inputs where they differ. Rows
that vary one parameter are N samples on that axis and one sample on every
other, and a parameter held at the value where the candidate behaviours
coincide is not a sample at all. Four Adversary findings, 2026-08-14 to
2026-08-16, merged as one mechanism.

## Instances

**Index held equal to depth** (`bf33ef099..b3f8cbb37`, 2026-08-16, verifying a
node built from the Adversary's own finding). The re-applied original mutation,
which had survived 19 tests, now reddened on the first discriminating row. But
the substitution leaf has three `Bound` arms:

```rust
Bound(i) if *i == depth => *replacement,     // exercised
Bound(i) if *i >  depth => Bound(i - 1),     // NOT exercised
Bound(i)                => Bound(*i),        // NOT exercised
```

Every row set the leaf index equal to its binder count. Dropping the decrement
passed 21 tests across three targets; making the third arm return the
replacement passed all unit tests. The rows varied binder depth and held
`index == depth`, so the index axis was never a variable.

**Arity held at zero** (`fc9408ec`, 2026-08-14). The original defect: every
omission test used a zero-arity constructor, *"where the constructor name and
the most-general pattern coincide"*. The repair changed the payload to a typed
witness, forcing all four emitters to supply an arity, and added one
arity-positive control. Reverting the `Display` wildcard loop to a name-only
render reddened exactly that control (15 passed, 1 failed). Instrumenting the
four emitters: site A reached 11 times with arities 0,0,0,3; site B once at 0;
sites C and D never. **The type change is universal; the evidence is one
observation at one site.** The compiler enforces presence, never correctness,
and a derived count is not a correct one.

**One binder, where two mappings are the same function** (`3df1afe7b...30769f649`,
2026-08-15). A reviewer correctly saw that a symmetric `=` fixture leaves a
de Bruijn mapping invisible, and proposed a one-variable fixture, accepted as
an optional remedy. The mapping is `binders - 1 - index`; at `binders = 1` it
gives `0`, and the identity `k{index}` also gives `0`. The remedy discriminated
strictly less than the two-binder fixture it was meant to strengthen, which at
least reddened on argument order. The working fixture kept the arity and
dropped a **use** (two binders, one variable mentioned): when a fragment admits
only a symmetric relation, the asymmetry must come from which operand is used.

**Wrong shapes and one correct artifact, no near-miss** (`41b49d94a..7726c108c`,
2026-08-16, a hand-authored checker mirroring a Rust reference). The equality
function is enumerated 9x9 with no catch-all, so over-acceptance can only come
from a same-constructor arm ignoring a field. Dropping the atom relation's
object slot, or the implication's consequent, each passed every test (7
passed). That is the soundness direction: the `Init` rule closes on this
equality. Ten rejection cases exercise wrong **shapes**; the acceptance case
exercises a **correct** certificate. **A gate that decides accept versus reject
is pinned only by an input that is nearly acceptable**, differing in exactly
the field the gate must not ignore. The scheduled successor built near-miss
pairs from the malformed cases, which are rule-shape near-misses, so it would
not have reached the equality-field axis: *"a successor exists"* is not *"this
is covered"*.

## How to apply

- **Ask which parameters the row set holds fixed, not only which it varies.**
  For each branch or arm of the mechanism, name the row that reaches it. Report
  the per-axis count, not the row count.
- **Run the arithmetic of a proposed fixture at its boundary values** before
  treating a remedy as settled. A remedy is a claim about discriminating power,
  and discriminating power is a measurement.
- **Find the degenerate value.** Arity zero, one binder, index equal to depth,
  empty set, a symmetric operator: the places where the right and a plausible
  wrong implementation agree. Rows there are true and cannot distinguish.
- **When a fix makes a property expressible everywhere, count the sites where
  it is measured.** Instrument the emission sites, not the tests: grepping tests
  shows what is asserted; instrumenting sites shows what is reached and with
  what values, and separates "exercised only at the degenerate value" from
  "never exercised", which route differently.
- **Use a universal probe.** Mutate the shared rendering or comparison the
  property flows through; the count of reds is the count of tests that can see
  it, in one run.
- **Build the oracle from what the code mirrors.** A hand-written equality
  mirroring a derived `PartialEq` has an oracle: the derived one. A differential
  over generated pairs including field-level near-misses covers every
  multi-field arm at once.
- **When a node argues only one control can see something**, every part of the
  mechanism that control misses is unobservable by construction. The node's
  rationale applies unchanged to the arms its oracle does not reach.
- **Transfer the author's own boundary discipline.** The axis node deliberately
  included a shallow non-discriminating row *"so the boundary is on the record
  rather than merely absent"*, on one axis. Naming the gap as the same
  discipline one dimension over is cheaper to accept, and true. Hand over the
  exact rows with expected values written out.
- **Re-run your own mutation against the control built to catch it.** A node
  built from a finding is where "fixed" is cheapest to check and least often
  checked.

## Side lessons from the same passes

- **Bound severity by tracing the consumer.** The wrong mapping is
  re-substituted and re-checked downstream, so it yields `Unknown`, not a bad
  verdict: completeness, never soundness. Say that beside the finding.
- **An unpinned agreement between two conventions is a finding shaped like a
  non-finding.** Emitter, parser and specializer all agreed index 0 is the
  outermost binder, across three functions in two files, true by inspection
  only. Worth one line.
- **A normalizer applied to both sides of an equality can only weaken it**, so
  read it before trusting the assertion (here `tokenize` was exact).
- **Verify a per-call-site claim by reading the call sites**, and when two
  helpers are grouped under one caveat, check it applies to both. A shared
  sentence is how a property gets attributed to a function that never had it.
- **Keep a census targeted.** Separating "never exercised anywhere" from "never
  exercised by a witnessing test" tempted a whole-crate test run; it was killed
  ten minutes into the build as no longer targeted. A census can outgrow the
  resource discipline while feeling like the same measurement; name the unrun
  read and ship the bounded one. Scope any `pkill` to the crate you started,
  and verify afterwards that what still runs is someone else's work.

Related:
[[a-non-degenerate-pair-fails-to-fail-if-the-assertion-cannot-tell-the-halves-apart]]
(the pair exists; the assertion cannot tell its halves apart),
[[a-control-over-an-or-route-needs-a-row-where-each-disjunct-decides]] (a
disjunct that never fires),
[[a-boundary-claim-needs-a-measurement-at-every-position-it-names]],
[[withdraw-and-relocate-test-different-properties]] (the perturbation set, not
the population, is too narrow), and in build scopes
`taint-axis-orientation-needs-distinguishing-pair` and
`a-controls-fixture-must-instantiate-the-quantifier-its-claim-ranges-over`.
