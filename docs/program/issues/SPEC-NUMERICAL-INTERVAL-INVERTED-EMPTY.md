---
id: SPEC-NUMERICAL-INTERVAL-INVERTED-EMPTY
title: "Spec 68 §1.1 says an interval whose low endpoint exceeds its high one has no member, the ordinary empty-interval representation, but no checked law proves it, and none proves the converse that an ordered interval holds its endpoints. Rule from a checked D0 at zero trust a public emptiness law and an endpoint-membership law, with a conformance seed"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-GRAPHS-REACHABILITY-ANTISYMMETRIC]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Numerical follows Graphs in the layer rotation. Steward-filed per COORDINATION section 2."
---

# An interval is empty exactly when its endpoints are inverted

## Objective

`Algorithm.Numeric.RationalInterval` exports public checked laws that,
for every `lo`, `hi`, `x : Rational`:

```ken
Equal Bool (rational_leq lo hi) False
  → IsTrue (interval_contains (MkInterval lo hi) x)
  → Bottom

IsTrue (rational_leq lo hi)
  → IsTrue (interval_contains (MkInterval lo hi) lo)
```

The second law has a companion for `hi`. Together they make §1.1's
prose claim a checked fact: an interval has a member exactly when its
endpoints are ordered. Spec 68 states the laws by their public types.

## Settled inputs (main `49a6173a6`)

- **The claim.** Spec 68 §1.1 (`68-numerical.md:96` to `:103`) says
  that if `lo` is greater than `hi`, no value is a member. No law
  proves this. §2.1 (`:286`) and §3.1 (`:438`) rely on it for their
  inverted examples.
- **Membership.** `interval_contains` (`RationalInterval.ken.md:219`)
  is `bool_and (rational_leq lo x) (rational_leq x hi)`.
- **The enablers.** `(proof trans for rational_leq)` and
  `(proof refl for rational_leq)` are used at `:426` and `:437`.
  `bool_and`'s `left`, `right` and `intro` proofs are used at `:434`
  and `:506` to `:513`.
- **Falsity.** `Bottom` is the prelude falsity proposition.
  `EmptyDec.ken.md:99` says `Equal Bool False True` reduces to `Bottom`
  and that `absurd` closes it.
- **Not yet stated.** Spec 68 runs §1 to §12 (§12 at `:1185`, file end
  `:1250`). The package exports 47 public theorems; none mentions `Bottom`.
- **Seeds.** Under `conformance/stdlib/numerical/`, next to
  `seed-interval-contains-respect.md`. The package test pattern is
  `crates/ken-elaborator/tests/cat_numeric_interval_inside_order.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The ruled laws in RationalInterval, a spec 68 section stating them by
their public types, a closed conformance seed under
`conformance/stdlib/numerical/`, a named package test, and spec 68's
status lines, all in one candidate.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the source, and the named-binary commands with a bounded
  timeout. The Foundation implementer runs it on scratch and reports
  rc, the `Axiom` grep and `trusted_base()` as an identity set before
  and after. The Architect rules from that report, and does not build
  or test. It rules:
  - the laws' names, argument order and placement;
  - the section number;
  - the falsity form: `Bottom`, or a premise-shaped contradiction;
  - whether the `hi` companion is a separate law or a conjunction.

  It confirms that no §1 to §12 operation or law type changes.
- **AC-1.** The section states the ruled laws by their public types.
  The Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client uses the emptiness law on the
  closed inverted `MkInterval rational_one rational_zero` to refute
  `rational_zero` being a member. It also uses the endpoint law on
  the ordered `MkInterval rational_zero rational_one`.
  - Its refused block claims `IsTrue (interval_contains (MkInterval
    rational_one rational_zero) rational_zero)` directly. It is refused
    because the decision computes `False`.
  - Its accepted same-form control makes the same claim on the ordered
    interval, so the refusal is the inversion, not the form.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a premise beyond
  the endpoint order.
- A change to an existing §1 to §12 operation or law type.

Checks: 4 fired: each law is written from `interval_contains` at
`:219`, the `rational_leq` refl and trans proofs and the `bool_and`
proofs as used at their lines, and `Bottom`'s `absurd` route at
`EmptyDec.ken.md:99`. 3 fired: the package's public laws and spec 68
§1 to §12 were read for an emptiness or endpoint-membership law; none
exists. 8 fired: the refused row and its control share the claim and
the client, and differ only in the endpoint order.
