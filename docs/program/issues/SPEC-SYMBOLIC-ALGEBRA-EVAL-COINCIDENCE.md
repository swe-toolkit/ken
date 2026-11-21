---
id: SPEC-SYMBOLIC-ALGEBRA-EVAL-COINCIDENCE
title: "Spec 66 §19 says changing the environment at one absent variable leaves eval unchanged, but no law says eval depends only on the variables that occur, so a client whose two environments differ at several unused variables must chain eval_absent once per variable. Rule from a checked D0 at zero trust that eval env1 e equals eval env2 e whenever env1 and env2 agree at every variable occurring in e, and state it with a seed that separates agreement on the occurring variables from a disagreement at one of them"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Symbolic algebra follows Time in the L3 layer rotation (docs/program/06-catalog-campaign.md). §19 handles one absent variable; no law handles the whole occurring set. Steward-filed per COORDINATION section 2."
---

# Evaluation depends only on the occurring variables

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports a public checked law.
For every `e`, `env1` and `env2`, if `env1 w` equals `env2 w` for every
`w` with `occurs w e` true, then `eval env1 e` equals `eval env2 e` as
`Equal Nat`. It holds at zero trust, and spec 66 states it by its public
type.

## Settled inputs (main `df433d9ab`; re-check anchors at release)

- **The operations.** `eval` (`NatPolynomial.ken.md:118`) and `occurs`
  (`:1175`) recurse over the four `Expr` constructors. `occurs` is
  `nat_same v w` at `EVar w` and `bool_or` of both sides at `EAdd` and
  `EMul`.
- **The gap.** `eval_absent` (`:1227`, spec 66 §19) quantifies the second
  environment's agreement over every `w` distinct from one absent `v`. No
  public law lets the two environments differ at more than one variable.
  `substitute_absent` (`:1297`, §20) and `deriv_independent` (`:1137`) are
  single-variable too.
- **Spec and tests.** Spec 66 ends with §20, so the new section is §21.
  The seed directory is `conformance/stdlib/symbolic-algebra/`. The test
  pattern is `crates/ken-elaborator/tests/cat_symbolic_algebra_eval_absent.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 66 §21, the catalog law with any private helper, the exported-law
roster entry if `NatPolynomial` has one, a seed, a package test and the
status lines. No §1 to §20 operation or law type moves, and no kernel or
surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-TIME-MERGE-DISTINCT-TIMES-COMMUTE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the name, the binder order and the placement after `eval_absent`;
  - the agreement premise's form, for example `Equal Bool (occurs w e)
    True` or `IsTrue (occurs w e)`, and whether the `bool_or` split needs
    a private helper or an existing public one;
  - whether `eval_absent` follows from the new law. If it does, it stays
    exported unchanged; it is not reproved or removed here;
  - every consumer root of an exported `NatPolynomial` law, by the sibling
    `eval_absent` across `catalog/`, `conformance/`, `crates/`, `spec/`
    and `library/`, so the candidate's scope is complete.
- **AC-1.** §21 states the law by its public type. The Spec Leader and CV
  review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law to an expression
  and two environments that differ at two variables, neither occurring,
  and concludes the two evaluations equal. A single `eval_absent` cannot
  reach that conclusion. Its refused block, on a fresh accepted prefix,
  makes one of those two variables occur, in the same application form.
  The agreement premise fails to check there, and the two evaluations
  really differ. The two rows differ only in that one occurrence.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond agreement at the occurring variables.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §20 operation or law type.

Checks: 3 fired. I listed every public `NatPolynomial` law on `df433d9ab`;
the three that mention an unused variable each fix exactly one. AC-0
sweeps the consumer roots by the sibling law, because Numerical's first QA
pass found an exported law missing from a roster. 4 fired. I read `eval`
and `occurs`: the obligation splits at `EAdd` and `EMul` through
`bool_or`, which is why AC-0 rules the helper. 8 fired. In the refused
row, an environment pair that disagrees at an occurring variable gives
different values, so the law must refuse there, and it does at the
premise.
