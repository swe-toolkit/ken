---
id: SPEC-SYMBOLIC-ALGEBRA-CANONICAL-EQUALITY
title: "Spec 66 normalizes a Nat polynomial expression to a distributed sum of monomials, but its normal form is not canonical: two expressions equal under every environment can normalize to different terms, and no operation decides their equality. Author section 2: a canonical sorted and merged normal form and a decision procedure for semantic equality, proved sound and complete at zero trust"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: [CAT-SYMBOLIC-ALGEBRA-NAT-POLYNOMIAL]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Every non-Lane-B roadmap layer through optimization/search has a delivered first package; spec 66 section 1.4 names canonical sorted/merged normal forms and a decision procedure for semantic equality as deferred, needing no signed or Float carrier. Same shape as SPEC-OPTIMIZATION-BOUNDED-SEARCH. Steward-filed per COORDINATION section 2."
---

# Deciding polynomial equality

## Objective

Spec 66 gains a normative section 2: a canonical normal form for its
`Expr` over `Nat` coefficients, and a Boolean decision of semantic
equality whose `True` and `False` answers are both proved. The catalog
node builds against it.

## Settled inputs (on `01acdfab6`)

- **Section 1 exists and is delivered.** `Algorithm.SymbolicAlgebra`
  exports `eval`, `monomials`, `reify`, `normalize` and five laws,
  including `normalize_sound` and `normalize_idempotent` (spec 66
  section 1.3).
- **The deferral.** Section 1.4: "Canonical sorted/merged normal forms
  and a decision procedure for semantic equality are also deferred."
  Subtraction, division, factoring and substitution stay deferred.
- **Nat order is proved.** `Data.Numeric.Nat.Order` exports `leq_nat`,
  `lt_nat` and their order laws.
- **Known elaborator limits.** Pass `map` a named helper, not a λ. A
  dependent `match` refines only its scrutinee as written (CHECKS 11).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** From a checked
   development (rc 0, no Axiom), rule:
   - the canonical form: monomial order, merging of like monomials,
     dropping zero coefficients, and its operation name;
   - the decision operation, for example
     `poly_eq (e1 e2 : Expr) : Bool`;
   - the laws, at least soundness (`True` implies equal evaluation
     under every environment) and completeness (equal evaluation under
     every environment implies `True`), or a measured blocker for
     completeness.
2. **Section 2 of spec 66**, stating exactly the D0's operations and
   proved laws, with a conformance seed of closed cases.

## Acceptance

- **AC-1.** The D0 development checks generically with no Axiom, and
  the section states exactly the laws it proved.
- **AC-2.** The seed has a case per operation and per law, a pair of
  syntactically different but equal expressions decided `True`, and a
  refused claim that two unequal expressions are equal.
- **AC-3.** `spec/50-stdlib/README.md` and `SPEC-PROGRESS.md` record
  the section.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a new primitive.
- Completeness cannot be proved generically: report the measured
  blocker, and do not state that law as proved.
