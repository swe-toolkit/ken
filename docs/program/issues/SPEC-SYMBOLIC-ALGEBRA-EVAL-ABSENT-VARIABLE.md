---
id: SPEC-SYMBOLIC-ALGEBRA-EVAL-ABSENT-VARIABLE
title: "Spec 66 §14 uses occurs to prove a derivative in an absent variable is zero, but no law says an expression's value ignores a variable that does not occur in it, so a client cannot discharge deriv_independent's environment premise, or move an evaluation between environments, from the occurs decision. Rule from a checked D0 at zero trust that eval agrees on two environments that differ only at a variable absent from the expression, and state it with a conformance seed"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Symbolic algebra follows Time in the L3 layer rotation (docs/program/06-catalog-campaign.md:376). occurs decides absence, but no eval law consumes it. Steward-filed per COORDINATION section 2."
---

# An expression's value ignores an absent variable

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports a public checked law
that, for every `v`, `e`, `env` and `env2`, from
`Equal Bool (occurs v e) False` and
`(w : Nat) → (Equal Nat v w → Bottom) → Equal Nat (env2 w) (env w)`,
concludes `Equal Nat (eval env2 e) (eval env e)`. It holds at zero
trust, and spec 66 states it by its public type.

## Settled inputs (main `fa440cc71`)

- **The decision.** `occurs` (`NatPolynomial.ken.md:1175`) is `False` on
  `EConst`, `nat_same v w` on `EVar w`, and `bool_or` of the operands on
  `EAdd` and `EMul`. Private `nat_same_sound` (`:2760`) and
  `nat_same_complete` (`:2795`) relate `nat_same` to `Equal Nat`.
- **The precedent.** `deriv_absent` (`:1183`) recurses on `e` from the
  same `occurs` premise, splitting it with `bool_or_false_left` and
  `bool_or_false_right`.
- **The consumer.** `deriv_independent` (`:1137`) takes, as premise `h`,
  exactly this agreement for every `env2`. With the new law, a client
  builds `h` from the `occurs` decision.
- **The gap.** No public law in `catalog/packages`, `spec/` or
  `conformance/` concludes anything about `eval` from `occurs`.
- **Spec and tests.** Spec 66 ends at §18
  (`66-symbolic-algebra.md:1433`), so the new section is §19. The test
  pattern is
  `crates/ken-elaborator/tests/cat_symbolic_algebra_derivative_absent.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 66 §19, the catalog law with any private helper, a seed, a package
test and the status lines. No §1 to §18 operation or law type moves, and
no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE-ABSENT-VARIABLE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules the name, the binder order, whether the
  agreement premise is stated as above or as `deriv_independent`'s
  `h` shape, and the proof route.
- **AC-1.** §19 states the ruled law by its public type. The Spec Leader
  and CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law at closed
  arguments: `e = EAdd (EVar 0) (EMul (EConst 2) (EVar 1))` and `v = 2`,
  with `env` sending every variable to 1 and `env2` agreeing except
  `env2 2 = 7`. Both evaluate to 3.

  Its refused block pins, in the same form as an accepted row and on a
  fresh accepted prefix:
  - the agreement claimed for that `e` with `env2` changed instead at
    variable 1, which occurs, to `env2 1 = 5`. That evaluates to 11,
    not 3.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond the two above.
- A change to an existing §1 to §18 operation or law type.

Checks: 4 fired. Induction on `e`, as in `deriv_absent`: `EConst` is
`Refl`. For `EVar w`, `occurs` is `nat_same v w = False`, so
`nat_same_complete` refutes `v = w` and the agreement premise closes the
case. `EAdd` and `EMul` split the premise by `bool_or_false_left` and
`_right`, then apply `cong` under `add` or `mul`. 3 fired: I grepped
`catalog/packages`, `spec/` and `conformance/` for a law taking an
`occurs` premise. The only hit is `deriv_absent`, which concludes about
`deriv`, not `eval`. 8 fired: on the fixture, `eval env e` is `1 + 2·1 =
3`. Changing variable 2 leaves 3, and changing variable 1 to 5 gives
`1 + 2·5 = 11`, so the refused row is false on the closed fixture. It
differs from the accepted row only in which variable `env2` changes.
