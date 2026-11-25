---
id: SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTE-FRESH
title: "Spec 66 says how substitution acts on an expression without a variable (§20) and when two substitutions agree (§22), but says nothing about which variables a substituted expression contains: a client cannot show that a variable absent from every replacement it uses is absent from the result, so it cannot apply deriv_absent or eval_absent after a substitution. Rule from a checked D0 at zero trust that, if v occurs in no s w for any w occurring in e, then v does not occur in substitute s e, and state it with a seed that separates the fresh variable from one a replacement introduces"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Symbolic algebra follows Time in the L3 layer rotation (docs/program/06-catalog-campaign.md). §19 to §22 gave absence and coincidence for eval and substitute; this gives the occurrence half of substitution. Steward-filed per COORDINATION section 2."
---

# Substitution keeps a variable fresh

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports a public checked law.
For every `v`, `s` and `e`, from `(w : Nat) → Equal Bool (occurs w e) True
→ Equal Bool (occurs v (s w)) False`, it concludes `Equal Bool (occurs v
(substitute s e)) False`. It holds at zero trust, and spec 66 states it by
its public type.

## Settled inputs (main `f139b7817`; anchors re-checked at release)

- **The operations.** `occurs` (`NatPolynomial.ken.md:1180`) is
  structural, with `bool_or` at the two binary constructors and `nat_same v
  w` at a variable. `substitute` (`:851`) maps `EVar w` to `s w` and is
  structural elsewhere. `bool_or` (`LawfulClasses.ken.md:107`) matches on
  its first argument, so `bool_or False b` reduces to `b`.
- **The argument, for AC-0 to check.** By induction on `e`.
  - `EConst` is `Refl`.
  - `EVar w` applies the premise at `w` to `nat_same_complete w w Refl`, as
    `substitute_coincidence` (`:1399`) does.
  - At `EAdd` and `EMul`, each IH takes the premise narrowed by `or_left`
    or `or_right`, as at `:1413`. `cong` on the first `bool_or` argument
    and then `trans` with the second IH give `False`.
- **The premise is needed.** With `e = EVar 0` and `s 0 = EVar 2`,
  `occurs 2 (substitute s e)` is `True`. A premise over every `w`, not only
  the occurring ones, would be stronger than a client can supply after
  §22.
- **Neighbours.** `substitute_absent` (`:1302`) is §20,
  `substitute_coincidence` (`:1399`) is §22, and `deriv_absent` (`:1188`)
  and `eval_absent` (`:1232`) are the laws a client composes it with.
- **Spec and tests.** Spec 66 ends with §22 (`66-symbolic-algebra.md:1736`),
  so the new section is §23. The seed directory is
  `conformance/stdlib/symbolic-algebra/`. The test pattern is
  `crates/ken-elaborator/tests/cat_symbolic_algebra_substitute_coincidence.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 66 §23, the catalog law with any private helper, a seed, a package
test and the status lines. No §1 to §22 operation or law type moves, and
there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTE-COINCIDENCE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order and the placement;
  - whether the occurrence premise is phrased as here or as the Boolean
    conjunction form;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`.
- **AC-1.** §23 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed takes `e = EAdd (EVar 0) (EVar 1)`, `s 0 = EVar 2` and
  `s 1 = EConst 3`. Its accepted block concludes through the law that
  variable 1 does not occur in `substitute s e`. Its refused block, on a
  fresh accepted prefix and in the same form, claims the same of variable
  2, which `s 0` introduces. The two rows differ only in the variable.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise beyond
  freshness in the occurring replacements.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §22 operation or law type.

Checks: 4 fired. I read `occurs`, `substitute`, `bool_or`, `or_left` and the
§22 proof on `f139b7817`. Every step of the argument uses one of them or
`cong` and `trans`. 3 fired. I grepped the package for an occurrence law
over `substitute`. Only §20 and §22 mention both, and neither concludes
about `occurs`. 8 fired. The refused row names the variable a replacement
introduces, so a law that ignores the replacements fails it, and the
accepted row fails a law that drops the result.
