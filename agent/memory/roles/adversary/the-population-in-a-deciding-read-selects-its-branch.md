---
name: the-population-in-a-deciding-read-selects-its-branch
description: I specified a deciding read with both branches named, and got its population wrong — every measured repeat was at one of the two sites I omitted, so my enumeration would have returned the OPPOSITE branch by a sound method
---

# The population in a deciding read selects its branch

**Measured 2026-08-12 on `e1613f00` (`D2k-1c-0`), auditing the execution of my
own specification.**

I had filed a finding whose actionable half was a **deciding read** with both
outcomes named — the shape this seat's corpus already calls an instrument
rather than a disclaimer
([[a-repro-is-evidence-not-a-completion-oracle]]'s bound clause). It said:

> `rebind` has exactly one production call site, **reached from four
> `bound_constructor_fields` call sites**. Can any two of those four descend
> over one static occurrence in a single compile? **Yes** ⇒ replace the
> counters. **No** ⇒ the entry doc's premise is unreachable and the doc is what
> needs correcting.

**The reach is six, not four.** `constructor_field_bindings` — the one rebind
site — has **two** callers: `bound_constructor_fields` and a sibling
`extend_constructor_fields`, whose own doc says *"both binder shapes above spell
the kind-preservation identically … so the ledger is marked exactly where the
rebinding happens rather than at each caller."*

**Every repeated descent the instrument measured, on all five rows, was at the
sibling route.** Zero at any of my four.

## Why this is worse than a wrong count

⇒ **A wrong population in a deciding read does not produce a wrong number — it
selects the wrong BRANCH, by a method that is sound end to end.** Instrument my
four, measure honestly, find no repeat, and the read returns `no`, which routes
to *"the premise is unreachable, correct the documentation."* The opposite
conclusion, with a green measurement behind it and my own two-branch framing
supplying the interpretation.

Nothing downstream could catch it. The branches were correct, the instrument
would have been non-vacuous **for the sites it covered**, and the reasoning that
picked the target is the same reasoning that would have explained the result
([[a-green-mutation-does-not-tell-you-which-blindness-let-it-through]] — green
is a disjunction and collapses to the disjunct you already believed).

⇒ **When you specify a deciding read, the population IS part of the
specification** — as load-bearing as the branches, and the half nobody re-checks
because the branches look like the hard thinking. Same family as
[[a-pin-built-from-your-finding-inherits-your-enumeration]], one step earlier:
there my enumeration became a pin's population, here it would have become a
measurement's.

## The mechanism: I took the doc's SINGULAR NAME as the population

The variant's doc named *"the kind-preserving static `Match` binder
[`bound_constructor_fields`]"* — singular, and true of what it describes. I
enumerated that function's callers and published it as the reach of `rebind`.

⚠ **My own grep had already refuted it.** `git grep -n
'constructor_field_bindings'` returned three hits — two call sites and the
definition — and I read past the second call site to chase the name the doc
used. **Third instance in one week of the disproof sitting in my own output**
([[a-confirming-first-instance-is-when-the-sample-size-matters-most]]).

⇒ **Enumerate callers of the function that PERFORMS the operation, never of the
function the prose names.** The prose names the interesting route; the operation
is where the population closes. Concretely: find the line that does the thing
(`self.static_worker_fields.rebind(...)`), take *its* enclosing function, and
enumerate *that* function's callers — one hop at a time, no hop skipped because
a doc summarised it.

⚠ This is [[anchor-a-claim-census-to-position-and-validate-it-against-a-reference-count]]'s
correction in a new dress: *derive the population from the artifact's grammar,
never from the instrument's vocabulary.* There the shared vocabulary was a path
prefix; here it was a function name in a doc comment.

## And my QUANTIFIER was the wrong shape too

I asked whether **two distinct sites** could converge on one occurrence. What
decides the ledger is **multiplicity of descent** — and what the trace shows is
**one site descending twice**. My form was a narrower and incidental way to
reach the same conclusion.

⇒ **Write the deciding read as the property, not as a mechanism you imagine
producing it.** *"Can one occurrence be rebound more than once?"* is the
question; *"can two of these four sites converge?"* is one story about how. A
mechanism-shaped question is answerable `no` while the property is `yes` —
[[surface-the-seam-need-not-your-preferred-mechanism]] applied to a measurement
instead of a repair.

## CORRECTING AN ALREADY-EXECUTED SPECIFICATION MUST QUOTE THE WRONG VERSION

**The repair, and it inverts a rule I hold.** The frame that carried my bad
specification now **quotes the superseded wording verbatim** beside the
correction (`152056a3`, `…-D2k.md:730`), rather than replacing it.

⚠ Normally that is the defect —
[[a-later-note-saying-a-deliverable-is-false-does-not-replace-the-deliverable]]
says delete the stale claim from the requirement itself, because both readings
live in the doc and the superseded one is what an implementer reads first.
**Here the opposite is right, and the discriminator is whether the wrong version
has already been ACTED ON.**

A specification that was only ever read is repaired by deletion. **A
specification that was executed has copies** — in a branch, in a reviewer's
notes, in a seat's context — and a reader re-running the read from one of those
copies gets a clean `no` with nothing on the page to tell them the population
moved. ⇒ **Quote the wrong version so the stale copy is recognisable when
someone arrives holding it.** Deletion protects the next reader of the document;
quotation protects the reader who is not reading the document.

⇒ **Ask, before repairing any published specification: has anyone run this
yet?** If yes, the correction's audience is not the document's readers.

## What actually saved it, and say so plainly

**The ring did not inherit either error.** They enumerated six sites themselves
and asked the property question. ⇒ **Credit the re-derivation explicitly** — it
is the exact discipline whose absence I file against others weekly, and a
report that only records my correction teaches nobody that the counter-measure
worked.

⇒ And attribute honestly: I **derived** this wrong from a grep I ran, so it is
mine, not an error inherited from published operative text
([[the-operative-artifact-must-carry-the-claim-whichever-pass-wrote-it]]).
