---
id: SPEC-NUMERICAL-INTERVAL-HULL
title: "Spec 68 section 1 proves interval enclosure for addition and multiplication over non-negative rationals, but section 1.4 defers interval hull and intersection. Rule from a checked D0 which hull and intersection laws hold at zero trust, and state them as section 2 with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 68 section 1.4 defers interval intersection and hull; they need no signed or Float carrier. Numerical is the next roadmap layer after geometry (docs/program/06-catalog-campaign.md). Same shape as SPEC-GEOMETRY-MANHATTAN-METRIC. Steward-filed per COORDINATION section 2."
---

# The hull of two intervals encloses both

## Objective

Spec 68 gains a normative section 2. The hull of two intervals contains
every point of each, and, if the D0 rules it in scope, intersection
contains exactly the common points. Each law is proved generically with
no Axiom, and the catalog node builds against them.

## Settled inputs (on `40fdd16f8`)

- **Section 1 is delivered.** `Algorithm.Numeric.RationalInterval`
  (`catalog/packages/Algorithm/Numeric/RationalInterval.ken.md`) defines
  `Interval` over non-negative `Rational` with `interval_contains`, and
  proves the enclosure and point laws (spec 68 section 1.3).
- **The deferral.** Section 1.4 defers "interval intersection and hull",
  along with signed values, division, floating point, width, inclusion
  isotonicity and sub-distributivity. Those stay deferred.
- **Order is proved up to `rational_equiv`.** Point soundness uses
  antisymmetry up to `rational_equiv`. A hull law stated by structural
  `Equal` of rationals may therefore be false; state laws by containment.
- **Known elaborator limits.** A dependent `match` refines only its
  scrutinee as written (CHECKS 11).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** From a checked
   development (rc 0, no Axiom), rule:
   - the hull's definition and its two enclosure laws, for example
     `IsTrue (interval_contains i x) → IsTrue (interval_contains
     (interval_hull i j) x)` and the `j` twin;
   - whether intersection is in scope. It may be empty, so it may need an
     `Option Interval` result. If in scope, rule its soundness law
     (contained in both) and its completeness law (a common point is
     contained), or report a measured blocker;
   - which Rational order helpers must be exported or become public.
2. **Section 2 of spec 68**, stating exactly the D0's proved laws, with a
   conformance seed of closed cases.

## Acceptance

- **AC-1.** The D0 development checks generically with no Axiom, and the
  section states exactly the laws it proved.
- **AC-2.** The seed has a case per law, including a hull of two disjoint
  intervals that contains a point of each, and a refused claim that the
  hull contains a point outside both endpoints.
- **AC-3.** `spec/50-stdlib/README.md` and `SPEC-PROGRESS.md` record the
  section.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, a new primitive, or a
  signed carrier.
- A law holds only up to `rational_equiv` but is stated by `Equal`: report
  it, and do not state it as proved.
