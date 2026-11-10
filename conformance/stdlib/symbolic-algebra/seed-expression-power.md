# Expression powers over natural-coefficient polynomials

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §16.

This seed supplies closed cases for `expr_pow`, `deriv_expr_pow`, and
`expr_pow_add`. It is a separate child of base
`8c39d80a0a1c7ddb5d3c9f909b7784c0d1db5fec`. The §16 catalog declarations
are not on that base, so these source-shaped inputs have not been checked
against the package or executed by an official conformance runner. No CI,
publication, or landing is claimed.

The positive case checks the unit definition, a closed power-rule instance,
and a closed exponent-addition instance. The refused case holds the expression,
exponent, and environment fixed while changing only the power-rule coefficient.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EMul, eval, deriv, poly_eq, expr_pow, deriv_expr_pow, expr_pow_add)

import Data.Numeric.Nat.Arithmetic (add)

const one : Nat = Suc Zero

const two : Nat = Suc one

const x : Expr = EVar Zero

fn env_one (v : Nat) : Nat = one

theorem power_zero_is_unit : Equal Expr (expr_pow x Zero) (EConst one) = Proved

theorem power_rule_law_instance
    : IsTrue
        (poly_eq
          (deriv Zero (expr_pow x two))
          (EMul (EMul (EConst two) (expr_pow x one)) (deriv Zero x))) =
  deriv_expr_pow Zero x one

theorem exponent_addition_law_instance
    : IsTrue (poly_eq (expr_pow x (add one two)) (EMul (expr_pow x one) (expr_pow x two))) =
  expr_pow_add x one two

theorem positive_power_rule_proved
    : IsTrue
        (poly_eq
          (deriv Zero (expr_pow x two))
          (EMul (EMul (EConst two) (expr_pow x one)) (deriv Zero x))) =
  Proved

theorem square_derivative_value_is_two
    : Equal Nat (eval env_one (deriv Zero (expr_pow x two))) two =
  Proved

theorem correct_power_rule_rhs_value_is_two
    : Equal Nat (eval env_one (EMul (EMul (EConst two) (expr_pow x one)) (deriv Zero x))) two =
  Proved

theorem one_coefficient_rhs_value_is_one
    : Equal Nat (eval env_one (EMul (EMul (EConst one) (expr_pow x one)) (deriv Zero x))) one =
  Proved
```

### `stdlib/symbolic-algebra/expression-power-closed-laws`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §16.
- promise class: durable invariant.
- given: `x = EVar Zero`, `env_one`, and the exponents `Zero`, `one`,
  and `two`.
- expect: the zero power is the unit expression; the public power-rule
  and exponent-addition laws apply at these arguments; the closed power
  rule is accepted with coefficient `two`.
- **MEASURED:** closed `Equal` values, two public theorem applications,
  and a `Proved` `IsTrue (poly_eq ...)` power-rule instance.
- **CLAIMED:** the defined operation and both public laws agree on these
  closed instances, including the coefficient for differentiating the
  square of `x`.
- **THE GAP:** these finite instances do not establish the generic laws
  for every expression, exponent, or environment; the checked public
  theorem bodies remain independent evidence.
- why: at `env_one`, the derivative of `x²` and the correct right-hand
  side both evaluate to `two`, while replacing the coefficient by `one`
  yields a right-hand side evaluating to `one`.

## Refuse the one-coefficient power rule

The refusal is a separate checker input. Recreate the same fixture and
accepted positive control, then append only the false coefficient claim.
The exact expression, exponent, and `env_one` are unchanged; only the
coefficient in the claimed right-hand side changes from `two` to `one`.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EMul, eval, deriv, poly_eq, expr_pow)

const one : Nat = Suc Zero

const two : Nat = Suc one

const x : Expr = EVar Zero

fn env_one (v : Nat) : Nat = one

theorem positive_power_rule_control
    : IsTrue
        (poly_eq
          (deriv Zero (expr_pow x two))
          (EMul (EMul (EConst two) (expr_pow x one)) (deriv Zero x))) =
  Proved

theorem square_derivative_value_control
    : Equal Nat (eval env_one (deriv Zero (expr_pow x two))) two =
  Proved

theorem wrong_coefficient_rhs_value_control
    : Equal Nat (eval env_one (EMul (EMul (EConst one) (expr_pow x one)) (deriv Zero x))) one =
  Proved

theorem refuse_one_coefficient_power_rule
    : IsTrue
        (poly_eq
          (deriv Zero (expr_pow x two))
          (EMul (EMul (EConst one) (expr_pow x one)) (deriv Zero x))) =
  Proved
```

### `stdlib/symbolic-algebra/reject-one-coefficient-power-rule`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §16.
- promise class: durable invariant.
- given: the same `x`, exponent `two`, and `env_one` as the positive
  case, with the coefficient changed from `two` to `one`.
- expect: the positive `Proved` control accepts and the final false
  `IsTrue` claim is refused at its own theorem.
- **MEASURED:** the correct-coefficient `poly_eq` claim is accepted;
  independently, the derivative evaluates to `two` and the changed
  right-hand side evaluates to `one` at `env_one`.
- **CLAIMED:** the power-rule coefficient is load-bearing for the
  square of `x`; the right-hand side with coefficient `one` is not
  semantically equal to the derivative.
- **THE GAP:** this single closed counterexample does not characterize
  every exponent, expression, or defective power-rule implementation.
- why: changing only the coefficient makes the expected polynomial
  comparison differ, while the positive control holds all other inputs
  fixed.

## Coverage boundary

The two cases cover the defining unit value, one closed instance of each
public law, and the coefficient discriminator for the power rule. The
closed observations do not prove the generic laws, measure a trusted-base
delta, or establish package-test execution, official conformance, CI,
publication, or landing. The seed makes no claim about signed or interval
subtraction, coefficient domains beyond `Nat`, or other derivative laws.
