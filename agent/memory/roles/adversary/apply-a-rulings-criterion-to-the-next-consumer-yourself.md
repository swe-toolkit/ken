---
name: apply-a-rulings-criterion-to-the-next-consumer-yourself
description: When a ruling turns on a structural property, that property is a reusable classifier — run it against the next consumer before that consumer is built; and a None from a lookup proves nothing until you show your key is in the lookup's domain
scope: roles/adversary
---

# Apply a ruling's criterion to the next consumer yourself

**Measured 2026-08-10 on `258336bf` (DS-9 `D1`), the first surface consumer of a
kernel restriction lifted for it.**

An Architect ruling had blocked one conformance row on a single structural
distinction: implicit lockstep consumes the motive instance at a **direct guest
leaf** (`support: None`), but a **recursive carrier** whose residual fields
carry `support: Some(...)` — `join : Bag A -> Bag A -> Bag A` — has no source
term denoting the recursive result.

**That distinction is decidable from any carrier's shape.** The new package's
whole recursion surface is `List Json`, and `List` is `Nil | Cons a (List a)` —
a recursive carrier, not a flat one. So the increment that has to fold over it
plausibly needs the same unreleased capability, **and the landed package prose
promises "ordinary structural recursion."**

⇒ **A ruling that turns on a structural property hands you a classifier.**
Extract the property, then run it against every other artifact in the class —
especially one that landed *after* the ruling and does not cite it. This is the
constructive twin of
[[a-ruling-that-widens-a-shared-map-names-only-the-consumer-it-was-about]]:
that lesson says the ruling names only the consumer that provoked it; this one
says **you can classify the others yourself, cheaply, before they are built.**

The yield is a **forward** dependency caught at declaration time rather than a
defect caught after the dependent increment is framed and started. File it as an
axis for the ruling's author to extend — the classification is yours to raise,
the extension is theirs to make.

## A `None` proves nothing until your key is in the domain

I probed whether the nested-support machinery engaged, called
`all_support_origin(Json)` and `all_support_origin(List)`, got `None` for both,
and nearly read it as *"no support was generated."*

**The query takes a *generated family* id; supports are keyed on the *host* and
installed when the host is declared.** Neither operand was ever in the domain.
`None` was the correct answer to a question I had not asked.

⇒ **Before reading a negative lookup as absence, show the key you passed is the
kind of key the lookup is keyed on** — read the accessor's signature and the
field's own doc comment, not the function name. A mis-keyed `None` is
indistinguishable from a real absence and reads as a measurement. Same family as
[[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]], one
layer down: the positive control here is *any* key known to be present.

I reported it as an unresolved measurement rather than a gap. **Say which of
your probes failed to ask their question** — a report that silently drops them
reads as coverage.

## A negative repro result is a deliverable, and must be bounded

Asked whether a formatter corruption reproduced, the answer was no: output
byte-identical to source. **But a normal-form file is a formatter's least
informative input** — it is a fixpoint by construction. So perturb into
legal-but-unformatted space (one-line block, ASCII vs Unicode arrows, extra
indent, blank lines, trailing separator) and check **meaning preservation via
re-elaboration**, not byte equality
([[corpus-property-gate-only-as-strong-as-the-corpus]]). Then bound it: *this
file and five perturbations, not every file in the corpus.*
