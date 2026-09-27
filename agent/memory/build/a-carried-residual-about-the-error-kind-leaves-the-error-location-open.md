---
name: a-carried-residual-about-the-error-kind-leaves-the-error-location-open
description: A fixture's loose `matches!` was carried as a residual about which error shape it admits — printing the error showed the span is the whole declaration, so the doc's localisation to one argument is unsupported by anything the test observes, and tightening the shape assertion would still not reach it
metadata:
  type: feedback
---

# A carried residual about the error *kind* leaves the error *location* open

**Measured 2026-08-15 on `00efd1f41`, a fixture-only merge whose looseness was
already filed.**

The carried residual:
`matches!(&err, KernelRejected { error: TypeMismatch { .. }, .. })`
passes on **any** kernel type mismatch, so a future regression elsewhere keeps
it green.

⇒ **Printing the error answered a different question and opened a second
residual.**

```
TypeMismatch { expected: ((Dg574 Dg67) @9), found: ((Dg574 Dg67) @4) },
span: Span { start: 0, end: 167 }
```

**The source string is 167 characters.** The span is the **entire
declaration** — it names no argument, no sub-expression, no position. But the
doc comment localises the failure to *"the recursive call's `xs` argument."*

⇒ ***An error's KIND and its LOCATION are separate claims, and a residual filed
about one leaves the other standing.*** Tightening the assertion to pin *"same
head, differing only in the index"* still would not establish **which operand**
mismatched. **Ask both questions of every diagnostic-shaped assertion: what
shape does it admit, and what does it pin about where.**

**A whole-declaration span is the tell.** When the reported span equals the
input's length, the diagnostic carries no positional information at all, and any
prose that names a sub-expression is authoring analysis rather than observation.
**Measure the source length — it is one line and it settles the question.**

## AND CONFIRM THE FIXTURE IS GREEN FOR THE RIGHT REASON *TODAY*

The filed residual was about a **future** regression. *"The assertion cannot
distinguish"* and *"the assertion is currently satisfied by something else"* are
different claims, and only the first had been made.

⇒ **Print the value.** Here it reproduced the reported failure byte-for-byte, so
the committed fixture had not drifted from the measured one. **A looseness
finding is incomplete until someone checks what the loose assertion is matching
right now.**

## A FIXTURE CAN BE A CONJUNCTION AND BE NAMED FOR ONE CONJUNCT

The inner match has one arm against a two-constructor family; it elaborates only
because the other constructor is **index-impossible** at the refined type. So the
fixture reaches its target gap **only while index-impossibility holds**.

⇒ **The successor's "which property moved?" question has two candidates, and
only one is named.** The unsafe direction is closed here — an
index-impossibility regression yields a different error class and reds the
assertion — but **say which conjuncts a fixture rests on, or a later reader
attributes any movement to the named one.**

## A FIRST-OCCURRENCE ANCHOR IS A SILENT MIS-TARGET

My probe anchored on `assert!(matches!(&err, ElabError::KernelRejected {` and
landed in a **different test six functions earlier** that uses the same idiom.
The run passed and printed nothing — **which is exactly what a correct run of an
unrelated test looks like.**

⇒ ***Anchor an inserted probe on something unique to the target — the test
name — never on an assertion idiom.*** **And grep for your own probe after
inserting it**: a silent mis-target is indistinguishable from "the code path was
not reached", which is the very thing a probe is usually inserted to decide.
