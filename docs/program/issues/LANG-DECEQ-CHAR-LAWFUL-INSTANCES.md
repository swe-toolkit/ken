---
id: LANG-DECEQ-CHAR-LAWFUL-INSTANCES
title: "`37 §2.5` defers the proof-carrying `DecEq String` / `Ord String` instances as a `tracked follow-on` because the transport needs a lawful `DecEq Char` that is not landed -- and the follow-on was never filed, so the second unowned obligation in this chapter sits in spec prose with no tracker row"
status: draft
owner: language
size: unsized
gate: operator
depends_on: []
blocks: []
github: null
origin: "Steward sweep 2026-08-13 at c1b9a1e8, taken while framing LANG-PRELUDE-ELABORATION-DEPTH. This is the second `tracked follow-on` in spec 37 found with no tracker row -- the first was `filter`, which produced LANG-PRELUDE-COLLECTIONS. Found by grepping the chapter for deferral language rather than by grepping the tracker for gaps."
---

## What this is

`spec/30-surface/37-strings-collections.md §2.5` (`:205-213`) states the
deferral and its reason:

> *"these are soundly transportable to lawful `DecEq String` / `Ord String`
> **instances** — the canonicity precondition holds here, unlike `Decimal`. But
> that transport additionally needs a lawful **`DecEq Char`**, which is **not
> yet landed** (only the `eqChar` view + `Ord Char`-by-transport are on `main`);
> so the proof-carrying `DecEq String` / `Ord String` instances are a **tracked
> follow-on**, not delivered here. This WP delivers the *functions*; it does
> **not** ship the lawful instances — filing the functions as proof-carrying
> instances would over-claim the trust level."*

**The deferral is correct and well-reasoned. It was never filed.** A grep of
`docs/program/issues/` finds no node for `DecEq Char` or for the lawful
instances.

## Why it is filed as `draft` with `gate: operator`, not as ready work

**It is blocked on a live operator question, not on framing.** Raised
2026-08-12 and unanswered: *is widening decidable equality worth two irreducible
postulates per registrant?* A lawful `DecEq Char` is precisely a registrant on
that mechanism, so this node cannot be scoped — let alone sized — until that
answer exists. Framing deliverables now would produce a frame whose shape the
ruling may invert.

**Filing it anyway is the point.** The chapter has now produced two obligations
recorded in prose and owned by nobody, and the first one (`filter`) survived
undetected long enough that its stated blocking reason had gone false without
anyone noticing. A `draft` node with a written reason is discoverable; a
sentence in a spec section is not.

## The pattern, worth more than this instance

Both misses were found by **reading the chapter for deferral language**, not by
auditing the tracker for gaps. A tracker audit cannot see an obligation that was
never entered into it — it can only compare rows against rows. **The deferral
phrasing is the searchable artifact**: "tracked follow-on", "a separate change",
"deferred", "not delivered here".

⇒ When a chapter is next swept, grep it for that phrasing and check each hit for
a tracker row. This is cheap and it has now paid twice in one chapter.

## Flip condition

Flip to `ready` and frame it when the operator answers the decidable-equality
TCB question. If the answer forecloses lawful instances at this trust level,
close this node as resolved-without-landing and **amend `37 §2.5` so the spec
stops promising a follow-on that will not come** — a stale promise in the spec
is what created this node.
