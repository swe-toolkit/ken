---
name: an-origin-to-site-map-is-a-relation-and-the-panic-beside-it-states-the-function-as-law
description: A probe run to attack a dynamic-extent attribution tag refuted the attack and, in the same trace, showed a sibling origin crossing twice with two different tags -- falsifying the "at most one crossing" invariant the control panics on. A guard's failure message is a diagnosis; when it states a false law or names the wrong population it misdirects the reader at the moment they can least tell. Read the rows a filtered census discards.
metadata:
  type: feedback
---

# An origin-to-site map is a relation, not the function beside it

**Measured 2026-08-15 on `637781f41`, with a temporary `#[cfg(test)]` probe:
a re-entrancy depth counter plus entry prints, one targeted run, then
reverted.** (The panic quoted below no longer exists on main as of 2026-09-26;
the lesson is about the shape.)

## The attack was refuted, and the refutation was the better result

`invoking_site` is a **thread-local read at the transfer**, with the guard
covering the tagging helper's **entire dynamic extent** -- a dynamic mechanism
the doc describes lexically as "the stable tag on `carry_call_input`". So the
tag names *an enclosing frame*, not *an invoker*, and would over-attribute under
nesting.

**`enclosing_transfers=0` on every transfer in all four compiles.** No nesting,
so the immediate invoker really is the tagged helper.

The node called its selection "provisional" on one axis (six callers share the
helper) and did not know the other axis existed. The probe settled the unstated
one. **When a claim is hedged, check whether the hedge enumerates every way it
could fail** -- a measured second axis is a contribution even when it confirms.

Two cheap dismissals of the discriminator were also wrong: `Direct` is live at
ten origins, and the return surface the rival branch names fires with `Direct`,
so the rival *is* distinguishable. **Say which attacks failed** -- it tells the
next pass which approaches are spent.

## The finding came from the rows nobody was looking at

Same trace, one line above the row under study:

```
transfer_into_carrier origin=StaticOriginId(39) … tag=Direct
…
carry_call_input ENTER origin=StaticOriginId(39)
transfer_into_carrier origin=StaticOriginId(39) … tag=GeneratedUnitCallInput
```

**Origin 39 crosses twice, with two different tags.** Origins 31, 41, 51, 61, 49
likewise cross twice. Beside them sat

```rust
_ => panic!("origin 5 must identify at most one predecessor crossing")
```

"An origin identifies at most one crossing" is a fixture fact about origin 5,
not a property of the mechanism, and the general form is falsified in the same
run by a sibling origin. The claim's wording inherits it: "both rows enter
**that** crossing" presumes there is one.

**A `panic!` whose message states a false law is worse than a red assertion.**
When the fixture fact stops holding, the reader is told the plan is corrupt
instead of "this origin now crosses twice, like its sibling always has" -- a
misdiagnosis, not noise. Repair: report the crossings as a `Vec` and let the
row table pin the count; the expectation stays exactly as strong.

## Second instance: a guard message that names the innocent population

**Measured 2026-08-14 on `294fceac`.** A file's comment named `sort` as
"deliberately not here" because its obligation would enter as an undischarged
postulate -- a trusted-base change. The new delta bracket opened three lines
below that sentence. So **the one declaration documented as violating the
bracket's invariant had its natural insertion point inside the bracket**, and
the guard's failure message named four *other* declarations by spelling.
Mechanism right, diagnosis wrong, and it fails out of the constructor, so
everything breaks rather than one test.

**When a guard's message enumerates names, check whether the thing most likely
to trip it is one of them.** A message listing the innocent population is the
same defect as a filter counting the wrong population: it sends the author to
the wrong file at the moment they are least able to tell.

Related: the bracket is **positional** while its comment says "exactly these
four declarations". It covers whatever lies between the markers -- which is
what makes it robust, and is worth saying, because the nominal phrasing is what
a later reader will trust.

## The general move

**A filtered census dumps the rows it discards. Read them.** The control
filtered to one origin; every other origin in the same dump was evidence about
whether the filter's uniqueness assumption was a law or a coincidence. **The
population you excluded is the cheapest available test of the invariant you
assumed about the population you kept.** And a guard's failure message is a
diagnosis: check that it would be true on the most likely way the guard fires.

Sibling of
[[an-instrument-that-reports-a-verdict-cannot-distinguish-inapplicable-from-false]]
-- same `match`, the other unexpected-population arm.
