---
scope: build
audience: (see scope README)
source: 2026-08-11, adversary `evt_xa93e5r8qxj9` on `LANG-SURFACE-RECORD-DECL`
  S4 — the fifth instance of one shape on a single arc
---

# A control's fixture must instantiate the quantifier its claim ranges over

A well-built assertion on a **degenerate witness** proves something weaker than
it says, and it reads as if it proved the whole thing. **When the claim contains
"every", "all", or "no ... anywhere", count the population the fixture actually
creates.** If that population is one, the universal and the singular are the
same sentence and the control cannot fail the way the claim is meant to catch.

## The instance that named it

`S4` migrated a whole-class-map iteration onto a borrowed class-entry
enumeration. Its control is the strongest shape on that arc — **set equality
against a literal**, not a count, not a comparison derived from the same read:

```rust
assert_eq!(
    class_fields,
    BTreeSet::from(["CensusProbe.first", "CensusProbe.second"]),
    "the class-entry view must preserve every declared class-field producer"
);
```

Omission fails one way, invention the other, and an empty result fails as
`{} != {two}`, so the equality-at-`0 == 0` vacuity is structurally impossible.
**Nothing is wrong with the assertion.**

The fixture declares **one class**, `CensusProbe`, with two fields. So it
discriminates *field*-level omission **within** a class, and cannot discriminate
*class*-level omission **across** classes — the axis the migrated site actually
needs, because that site iterates the whole class **map**. With one class,
*"returns every entry"* and *"returns the only entry"* are indistinguishable.

**Cost to close: one line.** A second class in the same fixture, its fields in
the same literal set, and the same assertion then discriminates both axes.

## The shape recurs, and the assertion is never the defect

Five instances on one arc, all "the control is well-built and the witness is
degenerate for the claim":

| control | the collapse |
|---|---|
| rounding boundary | both extremal ties break the same way, so the tie *rule* is untested |
| producer construct | one child, so "each child" is "the child" |
| recursion depth pair | 64 and 50,000 — both far from any real boundary |
| whole-map census | one class, so "every class" is "the class" |

**The reviewer's instinct goes to the assertion, and the assertion keeps being
fine.** That is why this survives review: reading the `assert_eq!` produces a
justified feeling of rigor, and the fixture three lines above it is skimmed as
setup.

## How to apply

1. **Read the claim's quantifier, then count the fixture.** "Every producer",
   "all entries", "no site anywhere" — each needs a population of at least two,
   and at least two that *differ on the axis named*.
2. **Two of the same thing is still one instance.** Two fields in one class does
   not exercise "every class". Ask which axis the consumer varies over, not
   which nouns appear twice.
3. **Extremal values are not boundary values.** A pair at 64 and 50,000 tests
   neither side of a limit that sits elsewhere; pick the value on each side of
   the actual boundary.
4. **Write the negative you expect.** If you cannot state which mutation this
   fixture would redden that a smaller one would not, the fixture is decorative.

Sibling of [[two-arm-producer-needs-a-case-per-arm]] (a producer's *arms* each
need a case) and [[taint-axis-orientation-needs-distinguishing-pair]] (an
orientation needs a contrasting pair). This one is the population axis: those two
ask whether every *branch* and every *direction* is exercised; this asks whether
the *set* the claim quantifies over has more than one member.
