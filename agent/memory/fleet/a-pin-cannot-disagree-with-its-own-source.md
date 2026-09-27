---
scope: fleet
audience: (see scope README) — anyone authoring a pin, differential, or
  reconciliation intended to prove that a consumer honours an authority; anyone
  reviewing one
source: RT-FNSPLIT-B2V, 2026-07-26 — FOUR instances in a single WP, tallied by
  the runtime-implementer in its retro and corroborated independently by
  runtime-qa at a site the implementer had not probed
---

# A pin cannot disagree with its own source

**Four times in one work package, a green pin was checking a value against
another expression of the same value.** Not one was a careless test. Every one
was **exhaustive within its own notion of the surface**, and that is exactly why
none of them looked wrong.

| # | the pin | its two sides | why it could never redden |
|---|---|---|---|
| 1 | whole-graph differential, **tag** axis | emitted graph vs emitted graph | one of five `class_guard` sites reverted to a literal → **439 passed / 0 failed** |
| 2 | the same differential, **class** axis — **already CONFIRMED** by review | ditto | same one-site disconnect, same green |
| 3 | `..._closed_over_the_whole_product` | `boundary_class_mask` vs `boundary_relation_admits` | **both are expressions of one hand-written slice**; full product, green, structurally unable to notice that *nothing derived it* |
| 4 | a comment-only checker's own weakness claim | the author's belief vs the author's belief | never executed; it was **over-strict**, not blind — see [[agreement-is-not-corroboration-when-a-premise-was-inherited]] |

## The rule

**Before trusting a pin, ask what would have to be WRONG for it to redden —
and then ask whether that thing is the AUTHORITY or a restatement of it.** If
both operands trace back to the same source, the pin is a **consistency check on
a single value dressed as a correspondence between two.** It will be green in
every world, including the ones you wrote it to exclude.

**"Full product" and "exhaustive" are not defences.** Instance 3 covered the
entire finite product in both directions and was still vacuous, because
coverage measures *how much of the surface you compared*, never *whether the two
sides could differ*. ⇒ **Exhaustiveness is a property of the sweep; independence
is a property of the operands. Only the second one makes a pin evidence.**

## A CONFIRMED axis is where to look, not where to stop

Instance 2 is the one worth the entry. The class axis had been **confirmed by the
Architect**, and the implementer ran the winning mutation against it anyway. It
won there too.

**A code review is accurate about the code it reads and structurally cannot
report whether the pin behind it is as strong as both parties assume.** The
reviewer reads the *consumer* and sees it consult the authority; nothing in that
reading reveals that the *test* would pass with the consultation removed.

⇒ **A confirmation is not a reason to skip the mutation. It is the reason
nobody else will run it.** Both parties now believe the axis is closed, so the
one cheap command that would falsify it has no remaining owner. Corollary for
reviewers: when you confirm an axis, say **what you did not measure**, so the
confirmation does not silently discharge someone else's obligation.

## How to author against it

- **Derive one side.** A pin is evidence when one operand is *computed from* the
  authority and the other is *observed from* the consumer. Two hand-maintained
  tables agreeing tells you a human copied carefully once.
- **Perturb the AUTHORITY, not the test.** If your mutation edits the pin's
  expectation, you measured the pin. Edit the thing the pin claims to be
  downstream of, and require a red.
- **Per-site, not aggregate.** A differential over an aggregate is an
  **existential** — *someone* consumed the authority — not the universal it reads
  as. That is instances 1 and 2, and it has its own entry:
  [[a-differential-over-an-aggregate-is-an-existential-not-a-universal]].
- **Name the seed.** A fold's fail-closed default is only real if some
  perturbation reaches it; see
  [[withdraw-and-relocate-test-different-properties]].

Sibling of [[deriving-from-the-contract-cannot-detect-a-defective-contract]] —
that entry is about a *faithful derivation from a bad contract*; this one is
about a pin that cannot detect **any** contract, good or bad, because it never
had two independent operands. Also
[[never-pin-a-shape-that-cannot-state-its-own-contract]] and
[[an-enumeration-needs-a-proven-closure-not-a-better-grep]].

## Half a struct can be its own source

**Measured 2026-08-14 on `a998d3f6` (Adversary).** An AC's own wording flagged
the risk: *"each agreeing with an independent direct derivation. The agreement
is the claim; a shared derivation path would make it vacuous."* The relation is
a two-field struct, and the re-derivation built every candidate as

```rust
Occurrence {
    body_origin: child_origin(claimed.eliminator_origin, 1 + alternative)?,  // re-derived
    eliminator_origin: claimed.eliminator_origin,                            // COPIED
}
```

`body_origin` is a real derivation. `eliminator_origin` cannot disagree,
because it is copied from the claim into the candidate before the comparison.
The control's own printed output showed it: `carried={16, 5}
direct=Some({16, 5})` and `carried={12, 5} direct=Some({12, 5})`; the `5` is
identical on both sides of both rows because nothing re-derived it. Running the
control, not reading it, is what surfaced this.

- **When a control asserts "derived == carried" on a struct, check the
  derivation field by field.** A struct-level `assert_eq!` reads as one claim
  and is as many claims as it has fields. The copied field is usually the one
  the derivation needed as an input.
- **Compare the fields the mutation moves with the fields the assertion
  covers.** The mutation that measured the refusal perturbed `body_origin`
  only (the other field sat outside the `#[cfg(test)]` seed branch), so the
  measured refusal exercised the half that was already independent. A combined
  mutation makes a red an existential over its perturbations; a single-field
  mutation reported as validating a whole struct is the same attribution gap
  run backwards.
- **A validation is not a derivation, and its strength rests on a premise.**
  The copied field was not unprotected: the re-derivation returns `None` unless
  `forward_match_scrutinee(claimed.eliminator_origin) ==
  key.continuation_origin`. That is as strong as a derivation only under an
  unstated uniqueness premise (at most one parent has a given occurrence at
  position zero). State such a premise and list it as read-but-not-fired; it is
  what a later graph change breaks.
- **Report the per-axis count, not the row count.** The two governed rows
  shared `eliminator_origin` and `consumer_owner` and differed only in
  `body_origin`: two samples on one axis and one on every other. See
  [[a-population-held-at-a-degenerate-value-cannot-see-that-axis]].
- **Check a re-derivation at the site of the shortage.** The field was
  *carried* because the interning site cannot compute it; the re-derivation
  runs where it can, and uses the carried copy without noticing. A value
  carried because a later site cannot compute it will be re-checked at a site
  that can, from the carried copy.

## An expected value computed from the thing under test is a theorem, not a check

Merged 2026-09-27 from `an-assertion-whose-expected-value-is-computed-from-the-thing-under-test-is-a-theorem`
(source: LANG-LOSSLESS-COUNT-ASSERTION-RETIRE, approved `a6c388bd`, 2026-08-14).
It is the same mechanism seen from the assertion's right-hand side: the
expected operand is produced by the subject, so the two sides cannot differ.

A round-trip helper carried this, and it read as a live safety net:

```rust
let comment_count = lossless.trivia().iter()
    .filter(|item| item.kind.is_comment())   // <-- the predicate under test
    .count();
assert_eq!(lossless.comment_attachments().len(), comment_count,
    "every trivia item counted by is_comment() must have exactly one home");
```

It cannot fail for the reason its message implies. `attach_comments` builds the
attachments by filtering the **same** `is_comment` over the **same** collection,
one attachment per survivor. So the assertion compares a set against
itself-mapped: **it is invariant under any change to `is_comment`.** Narrow that
predicate to drop block comments — the exact regression the surrounding work
package existed to fix — and `comment_count` and `attachments.len()` fall to zero
together. Green.

The same shape sat in production one layer down, in the totality validator, and
was equally invariant. What the validator actually checks is not the population
at all: it is the **coupling** between one filter and its own output.

**The general form.** An assertion has real content only when its expected value
comes from somewhere the implementation cannot reach: a literal, a
separately-derived number, an independent oracle, a different mechanism. When
you compute the expected value *with* the code under test, you have written down
a theorem about that code and dressed it as a test. It will survive every
mutation of the thing it names, and it will read — to every later author,
including its own — as coverage.

**How to apply.**

- **Read every assertion's right-hand side and ask where the number came from.**
  If it was produced by the subject, the assertion is vacuous no matter how
  precise the message is. `assert_eq!(f(x).len(), g(x).len())` where `f` and `g`
  share a filter is the canonical instance.
- **Prefer a literal.** In the case above, `assert_eq!(attachments.len(), 2)` on
  a fixture with one block and one doc comment is falsifiable by exactly the
  mutation the derived form misses. Literals are unfashionable and they are the
  point: they are the part the implementation cannot move.
- **A message that states a theorem is a tell.** *"Every X must have exactly one
  Y"* is a claim about the algorithm. A check's message names the *fixture*, not
  the law — if the sentence would be true of a program with no test at all, the
  assertion is probably deriving its expectation.
- **When you retire one, say what still guards the property — and check that it
  does.** Retiring a vacuous assertion is right; the failure mode is the
  successor sentence. Do not name the nearest fixture as the new guard without
  running the mutation against it. Census every consumer of the observable
  instead; the real guards are often in files the frame never mentions.

Kin to [[a-vacuous-law-has-zero-trust-delta]] (a hollow conditional adds no
axiom) and [[discriminating-conformance-verdict-must-flip]] (a case earns its
keep only if the verdict actually moves). Structural counterpart:
[[exhaustiveness-comes-from-an-unguarded-arm-not-from-the-match]] — there the
compiler supplies what a test cannot; here nothing supplies it, and the test
only appeared to.

### The tell is syntactic: does the expected side mention the actual side?

Merged 2026-09-26 from two Adversary lessons (2026-08-13, 2026-08-14) that are
the same mechanism at different distances.

**Same function: one operand defined from the other.** `f8f853b8`
(`RT-LEXICAL-R3-FUSION-EMITTER` `D3`, `static_transition.rs:14284-14307`). An
exact-partition seat `P = O ⊎ F`, closed deliberately as set relations rather
than counts (the right instinct, see
[[narrowing-a-counts-scope-never-turns-a-tally-into-a-pairing]]), still cannot
fail:

```rust
let residual = planned.difference(&fused).collect::<BTreeSet<_>>();
if !residual.is_disjoint(&fused)
    || residual.union(&fused).collect::<BTreeSet<_>>() != planned { return Err(...) }
```

| conjunct | evaluates to |
|---|---|
| `!residual.is_disjoint(&fused)` | `(A \ B) ∩ B = ∅` is a set identity; no input makes it true |
| `residual ∪ fused != planned` | equal to `planned` iff `fused ⊆ planned`, which an `is_subset` guard eight lines above already refused with a return |

When a validator **defines** one operand in terms of the other, the relation
between them is a theorem. It is
[[a-validator-whose-expected-value-is-its-own-builder-re-run]] with the
re-derivation collapsed to one operator: no builder to inspect and no comment
claiming independence, just a `difference` three lines up. **Read the
definition line of every operand in a relational assertion before reading the
assertion.** Shapes: `A.difference(B)` then a law relating the result to `B`;
`xs.filter(p)` then asserting every element satisfies `p`; `map.keys()` then
asserting the keys are in the map. The comment claimed the opposite (*"each
named separately because they fail for different reasons"*); a justification
for splitting two checks is evidence the author believed both live, not that
either is.

**Same function, read off the guarded object.** `555642ba` (`R3` `D3`), one
merge later: an affine ledger's `consume` refuses on `claim.seat() != seat`,
and both callers do `let seat = claim.seat(); ledger.consume(fusion, seat)?`.
That is `x != x` across the whole caller population. A guard written for a
future caller cannot be exercised by any present one, and an auditor asking
*"is this mutation guarded?"* reads two refusals and counts two. Size it
honestly: a vacuous guard removes no protection; what is lost is the hazard
its error message **declares** (here *"the takeover would replace a
continuation prefix this claim does not own"*).

**One layer down: widening a test predicate to match production.** `be8535b9`
(`LANG-COMMENT-POPULATION-PARITY`), the node built from an earlier finding that
a test helper counted with a narrow filter while production attached on a wide
one. The repair widened the test filter to the production predicate, which
removed the false red and with it the assertion's falsifiability. The predicate
then had three call sites: the production attacher, the production totality
validator, and the test. The parse entry point calls that validator and returns
`Err` on mismatch, so the test compared a third copy of the filter against two
sides production already reconciles. Mutating the attacher back to the narrow
filter:

```
panicked at …/kenfmt_b1_lossless.rs:10:33:   <- the parse_lossless expect
  Internal("comment attachment is not total: 2 comments, 0 unique homes, 0 attachments")
```

Line 10 fires; the widened assertion at line 27 is never reached. **The
assertion's falsifiability was carried entirely by the disagreement the repair
removed.** A repair phrased "make the test use the production predicate" is
usually right; also ask what the two operands become once they agree, and
whether a production guard sits between the test and any violating state.
**Settle it by mutating production and reading which check fires first**, one
build. (This finding is what `LANG-LOSSLESS-COUNT-ASSERTION-RETIRE`, the
instance at the top of this section, retired.)

#### Report what survives, and look for a second symptom of the same fact

- The widening candidate's real value was unaffected: the false red on corpus
  authors was gone, and its new fixture drove a previously unreachable
  population through a full byte round-trip and AST reparse. **Separate the
  node's value from the line the node is named for**, or the finding is triaged
  as an attack on a good repair.
- **An unjustified export and a vacuous assertion can be one fact.** The same
  repair made a private predicate `pub`; a caller census found zero production
  consumers outside the module. The only external caller was the vacuous line.
  Drop the line and the export reverts. When two residuals on one candidate
  name a symptom each (found by different instruments: a caller census and a
  mutation), check whether one is the other's cause. Keeping the line as
  defence-in-depth against the validator's deletion is legitimate, but then
  its doc must say so.
- **Re-run the acceptance of a node built from your own finding.** Running it
  (six transpositions, each applied alone and restored from a script; all six
  red with distinct `left`/`right` pairs) is what put the call-site census in
  view. A pass that reads such a repair confirms it; a pass that runs it finds
  the next thing.
- A structural prediction about a diagnostic is a claim about output, so read
  the output. The Adversary predicted a one-test, first-panic-wins layout would
  make three transpositions report identically; `assert_eq!` prints both
  values and all six messages were distinct.

#### The trim hazard runs toward the impressive check

In `f8f853b8` the live content was two plain `is_subset` refusals and the dead
content was a full partition law. A maintainer trimming apparent redundancy
keeps the one that **looks** like more proof (same outcome as
[[rank-a-controls-assertions-by-what-survives-a-redundancy-trim]]). When a
seat holds a cheap live guard beside an elaborate dead one, name which is
load-bearing **in the code**, not only in a report.

#### The blessed population can be discarded while consumers derive their own

`residual_identities` and `residual_targets` occurred at six lines: two
definitions and four uses inside the two dead conditions. The population
consumers read was built in different functions by a different expression, and
nothing checked the two agreed. The consumer's own doc stated the standard:
*"repeated filtering at each consumer would be a second authority ... and the
two would drift silently."* The validation block was that second derivation.
**Grep the rule's words across the whole seat, including the docs of what the
defective code feeds**; the accusing sentence may live on the function the
discarded value should have been compared against. The repair is one edit:
derive the residual from the function consumers read, and the vacuous law
becomes the missing cross-check. Check it is writable first (that function read
a field assigned after the block).

#### "Make it unrepresentable" removes the axis; check the operands are the same thing

The first-ranked repair for the `x != x` guard was to drop the parameter and
read the field inside. It would have been wrong. `claim.seat()` returns the
redirected edge's call-site origin; `claim.consuming_call()` is the consuming
`Call`. Different coordinates, measured 37/33 against 17/13 on the two armed
roots. Collapsing them deletes the mismatch **and** forecloses the cross-check
their distinctness leaves room for, invisibly.

- Before preferring unrepresentability, ask: **are these two operands the same
  thing, or merely equal today?** Same thing: collapse, no custodian needed.
  Merely equal: collapsing is a foreclosure, and *"no future caller can get it
  wrong"* is equally *"no future caller can get it right"*.
- The ranking survived only because it was stated **conditional on** one
  unread fact. Attach the condition to the ranking, not to the report's closing
  paragraph ([[a-repro-is-evidence-not-a-completion-oracle]]).
- **A vacuous guard can be the correct terminal state.** Passing
  `static_origin` would refuse every lawful consumption, and every other source
  was circular (`unit_calls` is keyed by the seat; `target.call_site_origin` is
  the value the redirect wrote from `claim.seat()`). When no independent
  derivation exists, the right output is a **recorded reason**, not a repair.
  That separates an accepted limitation from an undiscovered defect. Whether a
  documented limit is a cop-out turns on whether an independent derivation
  exists, so establish that first.

#### Do not diagnose the author from two instances

After the second instance the Adversary reported the recurrence as *"a property
of how the author is thinking"*, and it was promoted into a frame as a reviewer
heuristic within the hour. The next function opened refuted it:
`static_transition.rs:9736-9741`, same author and increment, names and avoids
the class (*"Against the DESCRIPTOR's own slot walk, not against the vector's
length, which would compare the projection with itself"*), and twelve lines up
the author had measured that a redundant copy of an earlier validation reddened
the control proving the refusals are not an earlier proxy. A third instance (a
child-0 identity copy standing in for the rule it defended) was caught by the
implementer the same morning; the class is real and impersonal.

- **The heuristic that survived:** this shape lives in short utility guards
  written in passing, which carry no comment reasoning about their own
  operands, while the load-bearing validation blocks carry that reasoning and
  are clean. **In a codebase where load-bearing guards carry a rationale, the
  absence of one on a guard is the tell.**
- A diagnosis drawn from defects is a claim about a population you have not
  enumerated, and defects are a biased sample by construction. Say "two
  instances, both in X" or read the population first
  ([[a-confirming-first-instance-is-when-the-sample-size-matters-most]]). A
  finding travels as a fix request; its framing travels as a heuristic nobody
  can check cheaply.

#### Adjacent findings from the same seat

- **The rule is not "all fallible validation precedes the mutation".** Same
  seat as `555642ba`: `// ---- THE ACTION. All fallible validation is above.`,
  then a call is emitted, then a fallible `consume` runs. The comment is false
  and the design is right: `consume`'s live guard (*"consumed twice, or never
  preflighted"*) is the one whose answer can change **because of** the action
  (a reentrant consumption during the call build). The real rule: **above the
  action goes everything hoistable; below it goes only what the action itself
  can change.** A false-but-well-meant comment invites a reader to "fix" the
  code by hoisting, destroying the property. When a stated rule is false and
  the behaviour right, the finding is the sentence; repair it by stating the
  real rule.
- **An extraction claim is decidable by multiset.** *"Extracted verbatim, so
  the two seats cannot become two authorities"* is checkable from the
  extraction commit's diff: strip `+`/`-` and all indentation, sort both sides,
  `comm` in both directions. Removed-only is behaviour that left and did not
  arrive; added-only minus comments is new mechanism smuggled into a
  "factoring" (the direction refactors actually fail). Here removed-only was 22
  lines, every one a `unit.<field>` access; applying `unit.` to `facts.` and
  `&mut builder` to `builder` took it to 0, and the 39 added-only lines were all
  declaration, signature, construction and call. **A structured residue is a
  normalization not yet applied; an unstructured one is a finding.**
- **Verify a convention by counting its setter's callers.** A `cfg(test)` arm's
  doc read *"always pair with the RAII guard"*. The setter was a private `fn`
  with two callers, both inside the guard (`arm()` and `Drop`), and the guard
  was the only exported surface, so visibility enforced it. Do not classify by
  the doc's mood (*always*, *be sure to*); one `git grep` on the producer
  settles it
  ([[a-name-free-slot-beats-a-reserved-spelling-and-the-alias-path-is-the-second-reader]]).
  Residual worth one clause: `let _ = Guard::arm();` drops at once, so the armed
  region is empty and a refusal assertion passes vacuously; `#[must_use]` does
  not fire on `let _ =`. When no mechanism exists, a sentence is the
  proportionate response.
- **State coverage on a candidate too large to read whole.** 4658 lines, eleven
  files, three surfaces read: put the unread list in the report body, not a
  closing hedge
  ([[the-operative-artifact-must-carry-the-claim-whichever-pass-wrote-it]]).
