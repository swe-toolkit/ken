# Differentiation preserves freshness

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §24.
Frame: `docs/program/issues/SPEC-SYMBOLIC-ALGEBRA-DERIV-FRESH.md`
AC-2; Architect AC-0: `evt_vb7pp34f71n6`.

The seed is authored but unexecuted. Architect's AC-0 measured the
closed clients in scratch on base `4337872ee`; that is not execution of
this seed or the official conformance runner, CI, publication, landing,
or a landed trust measurement.

Each case is a fresh client. The five closed occurrence goals reduce to
`Top` or `Bottom`. `Refl` refuses all five with the exact diagnostic
``Refl expects an `Eq`-shaped goal``; even the accepted goals need
`Proved`, not `Refl`.

## Accepted client: variable two stays absent

### stdlib/symbolic-algebra/deriv-fresh-preserves-absence

- spec: §24, `deriv_fresh`.
- promise class: durable invariant.
- given: `seed = EMul (EVar Zero) (EVar one)` and `v = Zero`.
- expect: accept the occurrence premise for `two` and the law application.
- measured: `two` is absent from `seed` and from `deriv Zero seed`.
- claimed: differentiation in variable zero preserves absence of variable
  two in this fixture.
- gap: this closed client does not establish the generic theorem.
- controls: the direct refusal below checks that the derivative retains
  `one`, while this accepted row uses a different absent query.
- why: the product rule differentiates the first factor while retaining
  `EVar one`; variable two is absent on both sides of the law.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EMul, EVar, deriv, deriv_fresh, occurs)

const one : Nat = Suc Zero
const two : Nat = Suc one
const seed : Expr = EMul (EVar Zero) (EVar one)

theorem premise_two : Equal Bool (occurs two seed) False = Proved

theorem fresh_two : Equal Bool (occurs two (deriv Zero seed)) False =
  deriv_fresh two Zero seed premise_two
```

## Refused pair: variable one occurs in the source

### stdlib/symbolic-algebra/reject-deriv-fresh-occurring-premise

- spec: §24, the `occurs u e = False` premise of `deriv_fresh`.
- promise class: durable invariant.
- given: the same `seed` and `v`, with the query changed from `two` to
  `one`.
- expect: refuse the `premise_one` declaration with
  `KernelRejected(TypeMismatch)`; `fresh_one` is not reached.
- measured: the false absence premise is rejected before the law application.
- claimed: the law cannot be applied when the queried variable occurs in
  the source expression.
- gap: this refusal tests the premise boundary, not the law's conclusion
  under an invalid premise.
- controls: the accepted `premise_two`/`fresh_two` pair has the same input,
  differentiation variable, and proof form; only `u` changes.
- why: `occurs one seed` is true because the source contains `EVar one`.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EMul, EVar, deriv, deriv_fresh, occurs)

const one : Nat = Suc Zero
const two : Nat = Suc one
const seed : Expr = EMul (EVar Zero) (EVar one)

theorem premise_one : Equal Bool (occurs one seed) False = Proved

theorem fresh_one : Equal Bool (occurs one (deriv Zero seed)) False =
  deriv_fresh one Zero seed premise_one
```

## Refused direct conclusion: the derivative retains variable one

### stdlib/symbolic-algebra/reject-direct-derivative-freshness

- spec: §24, `deriv_fresh`; §5, the structural derivative.
- promise class: durable invariant.
- given: a fresh client with the same `seed` and `v`.
- expect: refuse the `direct_one` declaration with
  `KernelRejected(TypeMismatch)`.
- measured: `occurs one (deriv Zero seed)` reduces to `True`, so the
  claimed false-occurrence equation reduces to `Bottom`.
- claimed: the derivative of this product still contains variable one.
- gap: one closed computation does not establish the generic theorem.
- controls: the accepted `u = two` law application above uses the same
  expression and differentiation variable, with only the queried variable
  absent.
- why: the product rule retains `EVar one` in the first product-rule arm;
  `occurs` checks syntactic occurrence, not semantic dependence.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EMul, EVar, deriv, occurs)

const one : Nat = Suc Zero
const seed : Expr = EMul (EVar Zero) (EVar one)

theorem direct_one :
    Equal Bool (occurs one (deriv Zero seed)) False = Proved
```

## Evidence boundary

The scratch D0 results in Architect AC-0 are design evidence, not
execution of this seed. No Ken check of the new `deriv_fresh` import,
official conformance-runner execution, CI, publication, landing, or
landed trust measurement is claimed.
