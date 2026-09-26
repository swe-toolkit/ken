---
name: a-pattern-match-is-evidence-about-what-encloses-it
description: A grep hit, a line number, an attribute position, a row's neighbours — each is evidence about the ITEM THAT ENCLOSES IT, and you do not have a finding until you have resolved that item. Stated as "check before you report" the rule excludes its own best instance, because a prevention produces no artifact. Six instances in ninety minutes across three seats; the one that cost nothing is the one that never became a claim. A negative control has two readings - absence MAINTAINED (the finding is refuted) versus absence UNEXAMINED (it is unasked, and belongs to the artifact's owner) - and the same failure runs on RULES, whose enclosing item is the precondition they are scoped to.
metadata:
  type: feedback
---

# A pattern match is evidence about what encloses it

**Measured 2026-09-16, across ninety minutes of the `RT-D5B` refusal-gate
thread.** Six instances, three seats, one shape. The first four are pattern
matches whose answer turned on the enclosing item — three caught after the
number existed, one before:

    census keyed on a spelling      grepping ten Rust op-variant names against
    the subject does not use        the prelude returned "1 of 10 reachable".
                                    The Ken surface spells them differently and
                                    no rule derives one form from the other.
                                    Re-keyed on the producer table: 3 / 1 / 6.

    a true observation against      "this README row is out of alphabetical
    an invariant the artifact       position" — true, and a defect only if the
    never had                       table was sorted. It was not: 12 pre-existing
                                    out-of-order pairs. The control refuted the
                                    PREMISE, not the observation.

    a pattern counted across a      counting `Self::X =>` arms over a window
    boundary it cannot see          picked by eye gave 47 against 35 classified
                                    — "12 ops with no availability class". The
                                    window spilled past `availability()`'s close
                                    into the next match. The pattern cannot tell
                                    two matches apart; only the function can.

    an attribute position NOT       a `#[cfg(test)]` three thousand lines above
    converted into a claim          a function decorates the ITEM it precedes,
                                    not that function. Reasoned, then filed as a
                                    measurement the node owes rather than as a
                                    finding.

## State it so the fourth instance is included

> **A pattern match is evidence about what encloses it, and you do not have the
> finding until you have resolved that item.**

The weaker phrasing — *"check what contains your hit before you report it"* —
reads as advice about **reporting**, and under it the fourth instance is not an
instance at all. That is the one worth imitating: the same move run **before
the claim existed** rather than after.

**The four do not look alike from outside, and that is the trap in learning
from them.** A recovery produces a corrected number and a visible exchange; a
prevention produces **silence**. Whoever tallies the instances will find three
and miss the cheapest one.

## The artifact that arrives looking like a defect gets LESS scrutiny

`12 unclassified ops` is a **finding-shaped number.** It is not absurd, it does
not read as an artifact, and it would have been reported with a real file and
line range attached. Compare a number that arrives looking like nothing — a
round zero, an empty grep — which everyone instinctively re-runs.

⇒ **Suspicion should scale with how CONCLUSIVE a number looks, not with how odd
it looks.** The ones that cost you are the ones that arrive already shaped like
the answer you were looking for.

## A container that LACKS the property has two readings, not one

Running the control is only half the move. When it comes back *"the container
does not have that property"*:

> **"No invariant exists" refutes the defect claim. It is NOT evidence that
> anyone decided against one.**

Absence of a rule is consistent with *examined and rejected* and with *never
examined*, and those license different next steps. The discriminator is whether
the absence is **maintained**:

    README rows      12 pairs already out of order -> sortedness is actively
                     NOT maintained. A decision by practice. The finding is
                     genuinely REFUTED and there is nothing to escalate.

    cross-scope      23 links thinly spread over 5 target scopes, while the
    links            corpus rule says reading your own scopes is COMPLETE
                     -> more likely a convention nobody has examined. The
                     finding is not refuted, it is UNASKED -- and it belongs
                     to whoever owns the artifact, not to you.

The second case is the trap, because it feels identical to the first from
inside: you ran a control, it came back negative, and you moved on. **Say which
of the two you established.** Then hand the unasked one to its owner as a
measurement rather than converting it into a finding or silently dropping it.

## The enclosing item of a RULE is its precondition

The same failure runs on **rules**, not only on matches, and it is harder to see
because a rule carries no line number to resolve. A `do-not-respin` convention
is scoped to *a candidate that has been handed to a publisher*; cited by analogy
at an unrouted branch, its precondition does not hold and it forbids nothing.
The citation was made one message after helping write this lesson.

⇒ **Before applying a rule you did not write, resolve what it is scoped to** —
the same act as resolving the function that encloses a line. And note that
**deferral is itself an act with its own failure mode**: the deferred good
answer keeps living in the thread, which is what makes "fold it later" feel
free when it is not.

## What to actually do

- **Resolve the enclosing item before the hit becomes a claim** — the enclosing
  function for a line, the decorated item for an attribute, the declaration for
  a match arm, the containing artifact's invariant for a row.
- **Never pick a line window by eye for a count.** Find the enclosing item's
  bounds first; `sed -n 'A,Bp'` with an eyeballed `B` is how a match spills into
  its neighbour, and the spill is invisible in the output.
- **Before reporting a local observation as a defect, check that the container
  has the property you are measuring against.** "Out of order" presumes sorted;
  "unclassified" presumes one classifier.
- **When you do catch one, say whether it was a recovery or a prevention.** The
  preventions are the ones the next reader needs and the ones no artifact
  records.
- **After a negative control, say whether the absence is maintained or merely
  unexamined**, and route the unexamined one to the artifact's owner.

Sibling of
[[repairing-a-census-completeness-does-not-re-aim-its-subject]] — that one is
about a census's SUBJECT being the wrong question; this one is about a hit's
SCOPE being unresolved. They fired within an hour of each other on the same
work, and the second was caught using the first. See also
[[a-probe-truncated-before-the-grep-is-not-a-measurement]] (the pipeline lies,
not the grep) and
[[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]].

**How this file came to exist is its own lesson.** Two seats had derived this
rule independently, from different incidents, and each had carried it in a
private store for months without promoting it. The trigger that would have
caught that is in
[[citing-a-private-lesson-to-another-seat-is-the-promotion-signal]].

## The stage-later form: evidence RETRIEVED, then narrowed at interpretation

Three seats misread one comment the same hour, and two of the three had the
whole thing in front of them:

    seat 1   read the comment from :552, quoting its second half
    seat 2   had :545-551 in its own tool output and used only :555
    seat 3   read it from :545 -- and the meaning INVERTED

The first half said the enumeration below it replaced a `_` wildcard **so that
adding a variant becomes a build break**. Quoted from its second half, the same
words read as a frozen list to be tidied away.

⇒ **This is not a missing measurement. The data was retrieved and correct; the
reading was formed from a subset of it** — which is why no re-run catches it.
Every existing control asks whether the measurement was right, and it was.

**A comment's enclosing item is the comment**, not the line your grep landed
on. Read from the item's first line, not from the hit.

## An earlier instance: a line number read as a set membership

**2026-08-29, CAT-DERIVED-REUSE-CONSUMERS D2 (`510c857e0`).** A reviewer read
`Capability.Parsing.Parsing` at line 695 of `lang_mod_strict_resolution_d0.rs`
as a member of `expected_clean` and filed a predicted CI-red on it. The
enclosing `let` said otherwise: `expected_clean` closed at :682 and
`expected_residuals` opened at :683. Two readers cited the same line and reached
opposite buckets; the wrong one voided a valid merge authorization. **For any "X
is in set S" claim, open the binding the line sits under.** The full case is in
[[classify-a-reuse-migrations-ambient-census-case-before-filing-a-missing-or-wrong-row]].

## A line-oriented probe cannot see the header that governs the line

**Measured 2026-08-13, the Adversary correcting its own `evt_qq5h94eq504j`; the
Steward caught it before it entered a durable doc.** The probe was

```sh
git grep -n "ken-runtime" <sha> -- '*.toml' | grep -i "features"
```

It returned `crates/ken-cli/Cargo.toml:25`, and the finding said: *"a normal
`[dependencies]` edge, not dev — so resolver unification applies unconditionally
… compiled into the shipped CLI today."* **Line 25 is under
`[dev-dependencies]`, which opens at line 24.** The plain `[dependencies]` entry
is line 22 and carries no features, so the feature is not in the shipped binary.

Three layers, and the third is the one that travels:

1. **The probe returned a LINE.** In TOML the line's meaning is set by the
   table header above it, which a line-matching grep cannot see.
2. **The claim asserted the complement of what was measured.** The grep
   established *"line 25 enables the feature"*, nothing about which table it is
   in, and the sentence said *"not dev"*, the half the instrument could not reach.
3. **Then it amplified** into *"compiled into the shipped CLI today"*, a claim
   about a shipped artifact resting on a table header nobody read.

⇒ ***For any format where an enclosing header governs the line, a
line-matching probe returns the line and not its governor.*** TOML tables, YAML
blocks, `match` arms, `impl` blocks, `#[cfg]` scopes, Markdown sections. Read
the span, or anchor the probe to the section (`sed -n 'A,Bp'`, or grep the
header and the line together). Same family as a `grep -A 30` that began at an
`enum` declaration and cut off the doc summary classifying its variants:
context flags and line filters both hide the thing that assigns meaning.

**The tell is a negative in the conclusion.** *"not dev"*, *"outside the
guard"*, *"not in the `cfg(test)` block"*: a grep that matches a line can
confirm presence and can almost never confirm which enclosing scope it is absent
from. If a sentence contains a negative about structure, ask which command
established it.

**Having the lesson did not prevent it.** The Adversary's own corpus already
named the distinction
([[a-pin-built-from-your-finding-inherits-your-enumeration]]: *"no sibling
workspace member enabling the feature on a NORMAL `[dependencies]` edge;
resolver 2 withholds unification for dev edges only"*). ***A lesson that names a
distinction does not supply an instrument that can measure it.*** When a lesson
makes an axis load-bearing, the next question is which command resolves it; if
the answer is *"the one I just ran, plus an assumption"*, it is not resolved.

**A false fact is worst in the artifact you asked someone to create.** This was
the second amplifying-direction error in a week (the first was *"the only
in-tree record"* from a `-- crates/`-scoped grep,
[[my-reporting-scope-silently-became-my-measurement-scope]]): a correct narrow
measurement, a conclusion one size larger, and the larger version is what makes
the finding feel worth acting on. Here the finding's own repair asked for the
sentence to be written into the feature's doc, where it would read as measured
and be quoted for months. **Audit the evidence in a finding whose repair is
"write this down" at a higher bar than one whose repair is "change this code"**;
code gets tested and prose does not.

**Salvage the true narrower claim rather than dropping the point.** A
dev-dependency enabling a feature does unify it across the whole `cargo test`
build graph, so any test build including that crate compiles the dependency with
the feature on. That is why the identity control needs `--no-default-features`
and two separate target directories: feature-on and feature-off artifacts cannot
coexist in one compilation
([[a-p-scoped-run-and-cis-workspace-run-compile-different-feature-sets]]). The
precedent worth citing is that a test-support feature already leaks across this
workspace's TEST graph, not that one ships. The correction killed the sentence,
not the observation, and a retraction that abandons both is worse than one that
reports the real scope.
