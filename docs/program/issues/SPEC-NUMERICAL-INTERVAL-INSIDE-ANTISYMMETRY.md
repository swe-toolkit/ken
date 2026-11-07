---
id: SPEC-NUMERICAL-INTERVAL-INSIDE-ANTISYMMETRY
title: "Spec 68 makes interval inclusion reflexive and transitive, but no law closes the order: two intervals each inside the other are not known to share endpoint values, so a client that brackets one enclosure by another both ways cannot conclude they denote the same interval. NatBox has the analogue (inside_antisym). Rule from a checked D0 at zero trust the public law that mutual inclusion makes both endpoint pairs rational_equiv, and state it with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Numerical follows Graphs in the L3 layer rotation (docs/program/06-catalog-campaign.md, numerical computing). Steward-filed per COORDINATION section 2."
---

# Mutual inclusion fixes the endpoints

## Objective

`Algorithm.Numeric.RationalInterval` exports a public checked
antisymmetry law for `interval_inside`. For all endpoints `a`, `b`,
`c`, `d : Rational`: if `MkInterval a b` is inside `MkInterval c d`
and `MkInterval c d` is inside `MkInterval a b`, then `a` and `c` are
`rational_equiv`, and so are `b` and `d`. Spec 68 states the law by
its public type.

## Settled inputs (main `d29b9c45a`)

- **The order.** `interval_inside` (`RationalInterval.ken.md:436`) is
  `bool_and (rational_leq c a) (rational_leq b d)` on the endpoints.
  `interval_inside_refl` (`:473`) and `interval_inside_trans` (`:483`)
  are public. No antisymmetry law exists.
- **The endpoint fact.** `rational_leq::antisym`
  (`Rational.ken.md:526`, spec 64 `:235`) returns `rational_equiv x y`,
  not `Equal Rational x y` (spec 64 `:247`). So the interval law
  concludes value equivalence, not `Equal Interval`. Spec 68 `:338`
  already refuses structural equality for these operations.
- **No projections.** The package exports `MkInterval` (`:78`) but no
  endpoint accessor, so the law is stated on `MkInterval` endpoints.
- **The precedent.**
  - Spec 68 §13 exports each endpoint claim as its own theorem, not as
    a conjunction.
  - NatBox's `inside_antisym` (`NatBox.ken.md:1895`) concludes
    `Equal Box`, which Nat's order allows.
  - The geometry seed is `seed-box-inside-antisymmetry.md`.
- **The spec.** Spec 68 runs §1 to §13, and the file ends at `:1326`.
- **Seeds and tests.**
  - Seeds go under `conformance/stdlib/numerical/`.
  - `seed-interval-inside-order.md` is the nearest neighbour.
  - The package test pattern is
    `crates/ken-elaborator/tests/cat_numeric_interval_inside_order.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The ruled law or laws in RationalInterval, a spec 68 section stating
them by their public types, a closed conformance seed and a named
package test, in one candidate.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the source, and the named-binary commands with a bounded
  timeout. The Foundation implementer runs it on scratch and reports
  rc, the `Axiom` grep, and `trusted_base()` as an identity set before
  and after. The Architect rules from the report and does not build or
  test. It rules:
  - the names, argument order and placement;
  - one law per endpoint (the §13 precedent) or one conjunction;
  - that no §1 to §13 operation or law type changes.
- **AC-1.** The section states the ruled laws by their public types.
  The Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law to two intervals
  with different endpoint spellings of the same values, for example
  `[1/2, 2]` and `[2/4, 4/2]`, and obtains `rational_equiv` for each
  pair. Its refused block applies the law to `[0, 1]` and `[0, 2]`,
  where only one inclusion holds, and its `Proved` reverse-inclusion
  premise fails. The accepted same-form control is the equal-value
  application. The refusal therefore comes from the failed inclusion,
  not the form.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise
  beyond the two inclusions.
- A change to an existing §1 to §13 operation or law type.
- A conclusion stated as `Equal Interval` or `Equal Rational`.

Checks: 4 fired: `interval_inside`, `interval_inside_refl`,
`interval_inside_trans` and `rational_leq::antisym` were read at their
lines on `d29b9c45a`; antisym yields `rational_equiv`, and no endpoint
projection is exported. 3 fired: grepped catalog/, conformance/ and
spec/ for `inside_antisym`, `antisym` and idempotence names; the only
interval-order antisymmetry is NatBox's. 8 fired: the refused
application and its control differ only in the second interval, and
the failed premise is the reverse inclusion.
