---
id: SPEC-SYMBOLIC-ALGEBRA-DERIV-FRESH
title: "Spec 66 says a derivative in an absent variable evaluates to zero (§14) and that evaluation ignores an absent variable (§19), and §23 keeps a variable fresh through substitution, but no law says which variables a derivative contains: a client holding deriv v e cannot apply deriv_absent or eval_absent to it in a variable absent from e, so a second partial derivative in an absent variable is not known to be zero. Rule from a checked D0 at zero trust that, if u does not occur in e, then u does not occur in deriv v e for every v, and state it with a seed that separates an absent variable from one the derivative keeps"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Symbolic algebra follows Time in the L3 layer rotation (docs/program/06-catalog-campaign.md). §23 gave the occurrence half of substitution; this gives it for differentiation. Steward-filed per COORDINATION section 2."
---

# Differentiation keeps a variable fresh

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports a public checked law.
For every `u`, `v` and `e`, from `Equal Bool (occurs u e) False`, it
concludes `Equal Bool (occurs u (deriv v e)) False`. It holds at zero
trust, and spec 66 states it by its public type.

## Settled inputs (main `df4e84a7a`; anchors re-checked at release)

- **The operations.**
  - `deriv` (`NatPolynomial.ken.md:988`) is structural. A variable goes to
    `var_deriv (nat_same v w)` (`:2969`), which is `EConst (Suc Zero)` or
    `EConst Zero`, so it contains no variable. A sum goes to the sum of the
    derivatives. A product `EMul a b` goes to
    `EAdd (EMul (deriv v a) b) (EMul a (deriv v b))`.
  - `occurs` (`:1182`) is structural, with `bool_or` at both binary
    constructors and `nat_same u w` at a variable.
- **The argument, for AC-0 to check.** Induct on `e`.
  - `EConst` is `Refl`.
  - At `EVar w`, case on `nat_same v w`. Both arms are constants, so both
    are `Refl`.
  - At `EAdd a b`, split the premise with the private `bool_or_false_left`
    and `bool_or_false_right` (`:2983`, `:2991`), as `deriv_absent`
    (`:1190`) does. Then join the two IHs.
  - At `EMul a b`, the result's occurrence is the `bool_or` of four parts:
    `deriv v a`, `b`, `a` and `deriv v b`. Two of those are IHs, and two
    are the split premise.
- **The premise is needed.** At `e = EMul (EVar 0) (EVar 1)`,
  `deriv 0 e` contains `EVar 1`. The converse fails too: at `e = EVar 1`,
  `deriv 0 e` is `EConst Zero`, so a variable absent from the derivative
  can occur in `e`.
- **No such law exists.** In the package, `occurs` and `deriv` meet only in
  `deriv_absent`, whose conclusion is about `eval`, not `occurs`.
- **Neighbours.**
  - §14 `deriv_absent` (`:1190`) and §19 `eval_absent` (`:1234`) are the
    laws a client composes this one with.
  - §23 `substitute_fresh` (`:1436`) is the substitution form.
- **Spec and tests.**
  - Spec 66 ends with §23 (`66-symbolic-algebra.md:1789`), so the new
    section is §24.
  - The seed directory is `conformance/stdlib/symbolic-algebra/`, after
    `seed-substitute-fresh.md`.
  - The test pattern is
    `crates/ken-elaborator/tests/cat_symbolic_algebra_substitute_fresh.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 66 §24, the catalog law with any private helper, a seed, a package
test and the status lines. No §1 to §23 operation or law type moves, and
there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTE-FRESH.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order and the placement;
  - whether a client corollary (a second derivative in an absent variable
    evaluates to zero, through `deriv_absent`) is also public, or left to
    clients;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`.
- **AC-1.** §24 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed takes `e = EMul (EVar 0) (EVar 1)` and `v = 0`.
  - Its accepted block proves the premise for `u = 2` by `Refl` and
    concludes through the law that variable 2 does not occur in
    `deriv 0 e`.
  - Its refused block, on a fresh accepted prefix and in the same form,
    takes `u = 1`. The premise `Refl` refuses, and so does a direct `Refl`
    of the conclusion, because the derivative keeps `EVar 1`.
  - The two rows differ only in `u`. AC-0 may replace this seed with a
    stronger discriminator.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond `u`'s absence from `e`.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §23 operation or law type.

Checks: 4 fired. I read `deriv`, `var_deriv`, `occurs`, the two
`bool_or_false` helpers and the `deriv_absent` proof on `df4e84a7a`.
Every step of the argument uses one of them, or `Refl`, `cong` and
`trans`. 3 fired. I grepped the package for a law with `occurs` over
`deriv` in its conclusion and found none. 8 fired. The refused row names
the variable the derivative keeps, so a law that ignores its premise fails
it, and the accepted row fails a law that drops the result.
