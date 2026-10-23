# Formal derivative of a natural polynomial expression

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §5.
Frame: `docs/program/issues/SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE.md`.

This source-shaped seed uses only §1–5 public operations and laws, plus
existing checked natural arithmetic. It was authored on AC-1 candidate
`1697a04f94fcb6d7d823c08d1c58cf6d226f0408`. At that base, the §5
catalog exports are absent, so neither case ran against that base or
through an official conformance runner. Architect AC-0 checked generic
proofs in separate scratch. A temporary overlay of that D0 checked a
copy of the full positive client (rc 0) and, historically, the old
open-environment refusal (rc 1/K0004). The closed refusal below instead
has local package-test evidence; no official conformance-runner execution
is claimed. Those scratch checks make no CI, publication, landing or
landed-trust claim.

## Equal functions with different expression trees

Let `x` be variable zero. `factored` is `x·(x+1)` and `distributed`
is `x·x+x`; they differ structurally. A generic natural-arithmetic proof
of their evaluation equality supplies the premise of the public
`deriv_respects_eval` law, rather than assuming that derivative law.
At `env_two`, both originals evaluate to six and both derivatives to
five. These independent value oracles distinguish a derivative that
drops either product-rule summand on these inputs. The positive client
also checks forward preservation of a `True` `poly_eq` answer.

The same client fixes `square = x·x`, `doubled = x+x`, and `env_two` for
the refusal below. Its same-operands control says the derivative of
`square` agrees with `doubled`, generically and at `env_two`: both
have value four there, while `x` has value two.

```ken
import Core.Logic.Transport (cong, trans)
import Data.Numeric.Nat.Arithmetic (add, mul, mul_add_distrib_r)
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, eval, poly_eq,
   poly_eq_sound, poly_eq_complete, deriv,
   deriv_respects_eval, poly_eq_deriv)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four
const six : Nat = Suc five

fn env_two (v : Nat) : Nat =
  match v { Zero ↦ two; Suc w ↦ Zero }

const x : Expr = EVar Zero
const square : Expr = EMul x x
const doubled : Expr = EAdd x x
const factored : Expr = EMul x (EAdd x (EConst one))
const distributed : Expr = EAdd square x

theorem originals_agree_at_every_environment (env : Nat → Nat) :
  Equal Nat (eval env factored) (eval env distributed) =
  trans Nat
    (mul (env Zero) (add (env Zero) one))
    (add (mul (env Zero) (env Zero)) (mul (env Zero) one))
    (add (mul (env Zero) (env Zero)) (env Zero))
    (mul_add_distrib_r (env Zero) (env Zero) one)
    (cong Nat Nat
      (mul (env Zero) one) (env Zero)
      (λz. add (mul (env Zero) (env Zero)) z)
      ((proof one_r for mul) (env Zero)))

theorem factored_evaluates_to_six :
  Equal Nat (eval env_two factored) six = Proved

theorem distributed_evaluates_to_six :
  Equal Nat (eval env_two distributed) six = Proved

theorem factored_derivative_evaluates_to_five :
  Equal Nat (eval env_two (deriv Zero factored)) five = Proved

theorem distributed_derivative_evaluates_to_five :
  Equal Nat (eval env_two (deriv Zero distributed)) five = Proved

theorem derivative_evaluations_agree (env : Nat → Nat) :
  Equal Nat (eval env (deriv Zero factored))
    (eval env (deriv Zero distributed)) =
  deriv_respects_eval Zero factored distributed
    originals_agree_at_every_environment env

theorem derivative_poly_eq_remains_true :
  IsTrue (poly_eq (deriv Zero factored) (deriv Zero distributed)) =
  poly_eq_deriv Zero factored distributed
    (poly_eq_complete factored distributed
      originals_agree_at_every_environment)

theorem square_derivative_matches_doubled (env : Nat → Nat) :
  Equal Nat (eval env (deriv Zero square)) (eval env doubled) =
  poly_eq_sound (deriv Zero square) doubled Proved env

theorem square_derivative_is_four_at_two :
  Equal Nat (eval env_two (deriv Zero square)) four = Proved

theorem doubled_is_four_at_two :
  Equal Nat (eval env_two doubled) four = Proved

theorem x_is_two_at_two :
  Equal Nat (eval env_two x) two = Proved

theorem square_derivative_equals_double_at_two :
  Equal Nat (eval env_two (deriv Zero square))
    (eval env_two doubled) = Proved
```

### `stdlib/symbolic-algebra/derivative-semantic-equality`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §5.1.
- promise class: durable invariant.
- given: structurally different `factored` and `distributed` under any
  total environment, and the closed `env_two` value oracle.
- expect: their evaluations agree generically; the public law yields
  evaluation-equal derivatives generically, with value five on both
  sides at `env_two`; the decision-level law preserves `True`.
- measured: an isolated D0 scratch copy of this full client checked
  rc 0. The source supplies a generic premise from checked Nat
  distributivity and unit, distinct closed value oracles, and both
  §5 public law applications; no shipped seed run is claimed.
- claimed: §5 differentiates equal polynomial functions to equal
  functions, without making their derivative expression trees equal.
- the gap: the finite value oracle does not prove the generic law; its
  public proof body and package export must be checked independently.
- why: the original trees differ but their polynomial function does not,
  while the explicit derivative values expose a missing product summand.

## Refuse the one-summand derivative

Runner assembly: first build and accept a client containing the entire
positive Ken fence above. Copy that accepted client fresh and append
only the refusal fence below. Never check the refusal alone. The
combined client must reach the false closed equality theorem
`refuse_square_derivative_as_one_x`; rejection for a missing import,
fixture or an earlier positive assertion is not the expected result.

The refused theorem claims that the derivative of `square = x·x` equals
`x` at the specific `env_two`. The accepted prefix uses the same
`square`, `x` and `env_two` to check that `deriv Zero square` evaluates
to four, `doubled = x+x` evaluates to four, and `x` evaluates to two.
A product rule that drops either summand instead evaluates the
square's derivative to two. Unlike an open-environment `Proved`
refusal, this closed claim would then be accepted: it distinguishes
the correct rule from either one-summand rule on this input. It does
not characterize all derivatives.

### `stdlib/symbolic-algebra/reject-square-derivative-as-one-x`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §5.1.
- promise class: durable invariant.
- given: a fresh copy of the accepted positive client, followed by
  only this refusal; `square`, `x` and `env_two` are unchanged.
- expect: the combined client rejects at its false closed `Equal Nat`
  theorem, not from missing declarations or a prior failure. The
  accepted prefix witnesses four versus two at `env_two`.
- measured: the package test checks the accepted prefix, then rejects
  this closed claim with a kernel TypeMismatch; a client-local rule
  dropping one product summand accepts the same closed claim. This is
  local test evidence, not an official conformance-runner result.
- claimed: the derivative of `x·x` in `x` is not `x` at `env_two`;
  a one-summand product rule would make this equality hold.
- the gap: this closed counterexample does not characterize every
  expression or every possible defective derivative rule.
- why: the same expression, queried variable and environment accept
  the correct `x+x` control, reject the correct-rule equality to `x`,
  and accept that equality when a summand is dropped.

```ken
theorem refuse_square_derivative_as_one_x :
  Equal Nat (eval env_two (deriv Zero square))
    (eval env_two x) = Proved
```

## Coverage boundary

The positive client checks the structural transformation through
closed evaluation oracles and applies both §5 public laws. The
negative is assembled only after that same accepted client and
asserts a false closed equality on the shared expressions at `env_two`.
The positive value oracles establish four versus two; the local
one-summand rule makes that same closed equality true and checkable.
The seed is source-shaped; the local package test does not claim
an official conformance-runner result, CI or publication.
