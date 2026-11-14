# Multiplicative laws for expression powers

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §17.
Frame: `docs/program/issues/SPEC-SYMBOLIC-ALGEBRA-EXPR-POWER-MUL.md`.

This seed covers the public power-of-a-power and power-of-a-product laws
through canonical `poly_eq`. The positive client applies each law at the
ruled closed arguments and separately checks its proposition with `Proved`.
Each refusal is a separate fresh input: it carries the same standalone
imports and constants, an accepted `Proved` control, and then one false
claim. The common base does not yet contain the new law exports, so these
source-shaped cases are for the assembled candidate; no official
conformance execution or CI is claimed.

## Accepted closed clients

```ken
import Core.Classes.LawfulClasses (IsTrue)

import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EVar, EMul, poly_eq, expr_pow, expr_pow_mul, expr_pow_emul)

import Data.Numeric.Nat.Arithmetic (add, mul)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

const x : Expr = EVar Zero

const y : Expr = EVar one

theorem power_of_power_law_instance
    : IsTrue (poly_eq (expr_pow x (mul two three)) (expr_pow (expr_pow x two) three)) =
  expr_pow_mul x two three

theorem power_of_power_proved
    : IsTrue (poly_eq (expr_pow x (mul two three)) (expr_pow (expr_pow x two) three)) =
  Proved

theorem power_of_product_law_instance
    : IsTrue (poly_eq (expr_pow (EMul x y) two) (EMul (expr_pow x two) (expr_pow y two))) =
  expr_pow_emul x y two

theorem power_of_product_proved
    : IsTrue (poly_eq (expr_pow (EMul x y) two) (EMul (expr_pow x two) (expr_pow y two))) =
  Proved
```

### stdlib/symbolic-algebra/expression-power-multiplication-laws

- spec: §17, `expr_pow_mul` and `expr_pow_emul`.
- promise class: durable invariant.
- given: `x = EVar Zero`, `y = EVar one`, `m = 2`, and `n = 3` or `2`.
- R-PP: `power_of_power_law_instance` applies `expr_pow_mul` at 2 and 3.
- R-PP-P: `power_of_power_proved` checks the same proposition with `Proved`.
- R-PROD: `power_of_product_law_instance` applies `expr_pow_emul` at 2.
- R-PROD-P: `power_of_product_proved` checks it with `Proved`.
- expect: all four rows accept.
- measured: the D0 client accepts these four rows on the checked scratch
  package; this seed has not been run as assembled conformance.
- claimed: exponent multiplication preserves a power-of-a-power, and
  raising an `EMul` to a power raises both factors.
- gap: these closed instances do not establish the generic laws for all
  expressions and natural exponents.
- why: the first law compares `(x²)³` with `x^(2·3)`; the second compares
  `(x·y)²` with `x²·y²` for distinct variables.

## Refuse an additive exponent in place of multiplication

This is a fresh standalone input. It repeats the imports and constants,
accepts the matching power-of-a-power `Proved` control, then changes only
the claimed left exponent from `mul two three` to `add two three`.

```ken
import Core.Classes.LawfulClasses (IsTrue)

import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EVar, EMul, poly_eq, expr_pow, expr_pow_mul, expr_pow_emul)

import Data.Numeric.Nat.Arithmetic (add, mul)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

const x : Expr = EVar Zero

const y : Expr = EVar one

theorem power_of_power_proved_control
    : IsTrue (poly_eq (expr_pow x (mul two three)) (expr_pow (expr_pow x two) three)) =
  Proved

theorem refuse_power_of_power_add
    : IsTrue (poly_eq (expr_pow x (add two three)) (expr_pow (expr_pow x two) three)) =
  Proved
```

### stdlib/symbolic-algebra/reject-additive-exponent

- case: R-PP-ADD.
- spec: §17, `expr_pow_mul`.
- promise class: durable invariant.
- given: the fresh input above with `x = EVar Zero`, `m = 2`, and `n = 3`.
- expect: the control accepts; `refuse_power_of_power_add` is refused
  with `KernelRejected(TypeMismatch)` at its own claim.
- measured: the D0 refused this false claim with that kind at the
  theorem's span; the accepted control is retained in this fresh input.
- claimed: the exponent in the first law is the product `m·n`, not
  `m+n`.
- gap: the closed refusal does not characterize every incorrect
  implementation of `expr_pow_mul`.
- why: `x^(2+3)` has degree five, while `(x²)³` has degree six.

## Refuse a product that drops the second power

This is a second fresh standalone input with the shared prefix. Its
accepted `Proved` control uses the complete power-of-a-product result;
the false claim changes only the right side by dropping `expr_pow` from
`y`.

```ken
import Core.Classes.LawfulClasses (IsTrue)

import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EVar, EMul, poly_eq, expr_pow, expr_pow_mul, expr_pow_emul)

import Data.Numeric.Nat.Arithmetic (add, mul)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

const x : Expr = EVar Zero

const y : Expr = EVar one

theorem power_of_product_proved_control
    : IsTrue (poly_eq (expr_pow (EMul x y) two) (EMul (expr_pow x two) (expr_pow y two))) =
  Proved

theorem refuse_power_of_product_drop
    : IsTrue (poly_eq (expr_pow (EMul x y) two) (EMul (expr_pow x two) y)) =
  Proved
```

### stdlib/symbolic-algebra/reject-dropped-product-power

- case: R-PROD-DROP.
- spec: §17, `expr_pow_emul`.
- promise class: durable invariant.
- given: the fresh input above with `x = EVar Zero`, `y = EVar one`,
  and `n = 2`.
- expect: the control accepts; `refuse_power_of_product_drop` is
  refused with `KernelRejected(TypeMismatch)` at its own claim.
- measured: the D0 refused this false claim with that kind at the
  theorem's span; the accepted control is retained in this fresh input.
- claimed: both factors are raised to `n` in the power-of-a-product law.
- gap: this closed refusal does not characterize every incorrect
  implementation of `expr_pow_emul`.
- why: the correct result has degree two in `y`, while the false result
  has degree one.

## Coverage boundary

The four accepted rows exercise each public law by application and by a
closed `Proved` claim. The two refusals pair each false claim with its
same-form accepted control in a fresh input. These cases do not prove the
generic laws, measure a trusted-base delta, or establish package-test
execution, official conformance, CI, publication, or landing.
