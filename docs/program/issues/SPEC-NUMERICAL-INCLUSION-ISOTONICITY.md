---
id: SPEC-NUMERICAL-INCLUSION-ISOTONICITY
title: "Spec 68 relates intervals only through the membership of single rationals and defers inclusion isotonicity: nothing states that one interval lies inside another, or that interval addition and multiplication preserve that inclusion. Rule an endpoint-wise inclusion test, its soundness, and isotonicity of the two operations from a checked D0 at zero trust, and state them as a section 3 with a conformance seed"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 68 sections 1.4 and 2.3 and the RationalInterval package defer inclusion isotonicity; it needs no signed carrier. Numerical is the roadmap layer after geometry (docs/program/06-catalog-campaign.md); non-Lane-B layers are deepened in place. Same shape as SPEC-GEOMETRY-BOX-CONTAINMENT. Steward-filed per COORDINATION section 2."
---

# Interval arithmetic preserves inclusion

## Objective

Spec 68 states a public Boolean inclusion test between two
`Algorithm.Numeric.RationalInterval` intervals, decided from their
endpoints, with checked laws that it is sound for membership and that
`interval_add` and `interval_mul` preserve it in both arguments.

## Settled inputs (spec 68 §1 and §2, on `main`)

- **Carrier (§1.1).** `Interval = MkInterval Rational Rational` over
  non-negative rationals, inclusive bounds, no `lo ≤ hi` field. An
  inverted interval has no member and is still a valid `Interval`;
  `Equal Interval` compares stored spellings.
- **Operations.** §1.2's `interval_add` and `interval_mul` act on
  corresponding endpoints; §2.1's `interval_hull` and
  `interval_intersect` use `rational_min` and `rational_max`.
- **Order vocabulary, exported.** `Data.Numeric.Rational` publishes
  `refl`, `trans` and `total for rational_leq`, `rational_add_mono`,
  `rational_mul_mono`, and the six min/max proofs of §2.3.
- **Laws today.** §1.3's six and §2.2's four laws are stated per
  rational `x`; none relates two intervals directly. §1.4, §2.3 and
  the package's prose list inclusion isotonicity as deferred.
- **Package.** `catalog/packages/Algorithm/Numeric/RationalInterval.ken.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

A new spec 68 section stating the inclusion test and each ruled law by
its public type, the deferral lists corrected to drop inclusion
isotonicity, and a closed conformance seed under
`conformance/stdlib/numerical/`. The AC-1 candidate also drops the stale
"monotonicity of rational addition or multiplication" item from spec 64
§5's separate-work list, pointing to §68 §1.4 (Steward
`evt_14fbjer0yjckk`).

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0 and no `Axiom`. It rules:
  - the test's name and endpoint-wise definition by `rational_leq`,
    and how it treats an inverted inner interval;
  - soundness: the test holding for `i` in `k` and `x` in `i` give
    `x` in `k`, over every `i`, `k` and `x`;
  - isotonicity: `i` in `i'` and `j` in `j'` give `interval_add i j`
    in `interval_add i' j'`, and the same for `interval_mul`;
  - whether reflexivity, the hull and intersect inclusions, and a
    witnessed converse of soundness are public laws. The converse
    fails on an inverted inner interval as stated, so the D0 states
    its premise or excludes it.
- **AC-1.** The section states each ruled law by its public type, and
  the deferral lists are corrected. The Spec leader and CV review it,
  with a Decision.
- **AC-2.** The seed checks soundness on a closed nested pair and both
  isotonicity laws on closed nested pairs, with a positive control on
  an interval included in itself. It has a refused block that claims
  inclusion of an interval in a strictly smaller one.

## Stop conditions

- A law that needs a signed carrier, subtraction, `rational_equiv`
  stated as `Equal`, a canonical empty interval or member-set equality,
  unless the D0 rules it as the stated premise.
- Any new primitive, kernel or surface feature.
- An ambient name in the package beyond its current census row
  (`lang_mod_strict_resolution_d0`): import it explicitly.
