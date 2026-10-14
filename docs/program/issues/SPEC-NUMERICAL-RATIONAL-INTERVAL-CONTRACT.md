---
id: SPEC-NUMERICAL-RATIONAL-INTERVAL-CONTRACT
title: "No numerical-computing contract exists in spec/50-stdlib, so the catalog's next roadmap layer after geometry has nothing to build against. Author its chapter with a first exact section: closed intervals over non-negative rationals, with addition and multiplication proved to enclose their arguments' results at zero trust"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-GEOMETRY-EXACT-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. 06-catalog-campaign.md:376-377 places numerical computing (error-bound refinements) after geometry. Same shape as SPEC-GEOMETRY-EXACT-CONTRACT. Steward-filed per COORDINATION section 2."
---

# An exact error-bound contract

## Objective

A new `spec/50-stdlib` chapter exists, and its first section is the
normative contract for exact error bounds: an interval of non-negative
rationals that provably encloses the result of an operation. The first
numerical catalog node builds against it.

## Settled inputs (on `ed58ab85f`)

- **Rational is proved, with gaps.** `Data.Numeric.Rational` (spec 64)
  exports `Rational` (non-negative, no subtraction), `rational_add`,
  `rational_mul`, `rational_leq`, `rational_equiv`, and their
  commutativity, associativity, unit, distributivity, `respect` and
  `leq` order laws (`refl`, `trans`, `antisym`, `total`).
- **No monotonicity law is exported.** Nothing states that
  `rational_add` or `rational_mul` is monotone under `rational_leq`.
  `Data.Numeric.Nat.Order` exports `leq_nat_mul_right_mono`, which the
  Rational order proofs use internally.
- **No interval or error-bound declaration exists** in `catalog/`,
  `spec/50-stdlib` or `conformance/`.
- **Known elaborator limits.** Pass `map` a named helper, not a λ
  (`LANG-ELABORATED-BETA-REDEX-AT-KERNEL-INFERENCE`). A dependent `match`
  refines only its scrutinee as written (CHECKS 11).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** Rule from a checked
   development (rc=0, no Axiom) on:
   - the chapter number, the package's Section and its module path;
   - the carrier: for example an interval of two rationals with
     `lo ≤ hi` as a proof field or a predicate, and membership;
   - the operations: at least interval addition and multiplication, and
     a point interval;
   - the laws: at least that if `x` lies in `I` and `y` lies in `J`, then
     `x + y` lies in `I + J`, and the same for multiplication;
   - which monotonicity laws `Data.Numeric.Rational` must newly export,
     a change the catalog node then owns;
   - what stays deferred: subtraction and signed values, division,
     floating point, and rounding modes.
2. **The chapter, section 1**, stating the D0's operations and proved
   laws, with a conformance seed of closed cases.

## Acceptance

- **AC-1.** The Architect's D0 development checks generically, with no
  Axiom, and the section states exactly the laws it proved.
- **AC-2.** The seed has at least one case per operation and one per
  law, and a case that refuses an interval sum which excludes the sum
  of two members.
- **AC-3.** `spec/50-stdlib/README.md` and `SPEC-PROGRESS.md` record the
  chapter.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a new primitive.
- The D0 cannot check an enclosure law generically. Report the measured
  blocker, and do not state that law as proved.
