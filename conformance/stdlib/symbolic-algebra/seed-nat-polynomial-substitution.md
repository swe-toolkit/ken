# Substitution under a composed environment

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §3.

This seed contains five authored closed behavioral oracles for §3.
The §1 and §2 package work and seed files are present on main. The §3
public operations and laws are Architect-D0-checked scratch work, not
catalog delivery. Architect D0 reports the same closed values `4` and
`3`, identity control, and uncomposed-environment refusal as checked
in a roots-loaded client. That is not shipped seed-harness execution.
These cases do not establish CI, publication, or landing.

Every case has promise class **durable invariant**.

## Shared fixture and positive operation oracles

`env0` maps each index `v` to `Suc v`. `sub0` replaces `Zero` with
`x0 + 1` and every successor index with `x1 * x0`; the inserted
expressions are not substituted again. `e0` adds `x0` and `x1`.
The substituted expression evaluates to `4`; the original expression
under `env0` evaluates to `3`. The code uses the first-party AC-0
fixture and names the public natural-valued environment builder.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, eval, poly_eq,
   compose_env, substitute, eval_substitute, poly_eq_substitute)

const one : Nat = Suc Zero
const three : Nat = Suc (Suc one)
const four : Nat = Suc three

fn env0 (v : Nat) : Nat = Suc v

fn sub0 (v : Nat) : Expr =
  match v {
    Zero ↦ EAdd (EVar Zero) (EConst one);
    Suc w ↦ EMul (EVar one) (EVar Zero)
  }

fn identity_sub (v : Nat) : Expr = EVar v

const e0 : Expr = EAdd (EVar Zero) (EVar one)
const e1 : Expr = EAdd (EVar one) (EVar Zero)

theorem substituted_value_is_four :
  Equal Nat (eval env0 (substitute sub0 e0)) four = Proved

theorem original_value_is_three :
  Equal Nat (eval env0 e0) three = Proved

theorem composed_value_is_four :
  Equal Nat (eval (compose_env env0 sub0) e0) four = Proved

theorem eval_substitute_closed_instance :
  Equal Nat (eval env0 (substitute sub0 e0))
    (eval (compose_env env0 sub0) e0) =
  eval_substitute env0 sub0 e0

theorem identity_substitution_control :
  Equal Nat (eval env0 (substitute identity_sub e0))
    (eval env0 e0) = Proved
```

### `stdlib/symbolic-algebra/substitution-evaluation-sum-product`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §3.
- promise class: durable invariant.
- given: `env0`, `sub0`, and `e0`; `sub0 Zero` is a sum and
  `sub0 (Suc Zero)` is a product.
- expect: the substituted expression evaluates to `4`, the original
  expression under `env0` evaluates to `3`, and the composed-environment
  equation is accepted. The closed theorem applies `eval_substitute`.
- measured: both closed Nat values and a client application of the
  generic public evaluation law.
- claimed: this D0-shaped substitution observes evaluation under
  `compose_env env0 sub0`, not under the uncomposed environment.
- the gap: one closed input does not prove the universally quantified
  evaluation law.
- why: the two independently stated values distinguish the composed
  result from evaluating the original expression unchanged.

### `stdlib/symbolic-algebra/substitution-identity-control`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §3.
- promise class: durable invariant.
- given: the same `env0` and `e0`, with `identity_sub v = EVar v`.
- expect: substituting each variable by itself preserves the evaluation
  of `e0` under `env0`.
- measured: the closed `Equal Nat` equation ending in `Proved`.
- claimed: the substitution operation preserves variables under the
  identity mapping.
- the gap: this is a control at one environment and expression, not a
  generic identity-substitution theorem.
- why: it is the positive control for the substitution operation before
  the separate uncomposed-environment refusal.

## Forward preservation of polynomial equality

The D0 pair `e0 = x0 + x1` and `e1 = x1 + x0` has a `True`
`poly_eq` answer. Applying `sub0` to both remains a closed
forward-law instance.
A separate constant-zero mapping identifies `EVar Zero` and
`EVar one`, whose original `poly_eq` answer is `False`; this is the
specified counterexample to a converse.

```ken
fn zero_sub (v : Nat) : Expr = EConst Zero

theorem equal_pair_before_substitution :
  IsTrue (poly_eq e0 e1) = Proved

theorem poly_eq_substitute_closed_instance :
  IsTrue (poly_eq (substitute sub0 e0) (substitute sub0 e1)) =
  poly_eq_substitute sub0 e0 e1 Proved

theorem distinct_variables_are_not_equal :
  Equal Bool (poly_eq (EVar Zero) (EVar one)) False = Proved

theorem zero_substitution_identifies_distinct_variables :
  Equal Bool
    (poly_eq
      (substitute zero_sub (EVar Zero))
      (substitute zero_sub (EVar one)))
    True = Proved
```

### `stdlib/symbolic-algebra/poly-eq-forward-substitution`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §3.
- promise class: durable invariant.
- given: the §2 equal pair `e0`, `e1` and the same `sub0` as the
  evaluation case.
- expect: the pair's `True` answer remains `True` after applying
  `sub0` to both sides; the public forward-law instance checks.
- measured: the pre-substitution control and the closed application of
  `poly_eq_substitute`.
- claimed: this concrete pair obeys the forward preservation law.
- the gap: one client instance does not prove the generic law body.
- why: `poly_eq` compares semantic polynomial forms rather than the
  syntactic shape of the input expressions.

### `stdlib/symbolic-algebra/poly-eq-substitution-can-identify`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §3.
- promise class: durable invariant.
- given: `EVar Zero`, `EVar one`, and `zero_sub v = EConst Zero`.
- expect: `poly_eq (EVar Zero) (EVar one)` is `False` before
  substitution and `True` after both variables map to the same constant.
- measured: the closed Bool equations on the same pair.
- claimed: `poly_eq_substitute` has no converse: substitution may
  identify expressions that were previously distinct.
- the gap: this example is not a generic characterization of every
  substitution's effect on unequal expressions.
- why: it is the D0 counterexample to an `iff` or reverse-preservation
  claim.

## Refuse evaluation under the uncomposed environment

The negative block is a separate checker input. It repeats the exact
`env0`, `sub0`, and `e0` fixture with fresh local names, then checks the
positive controls before asserting the false uncomposed equation. The
refusal is pinned to that equation, not to an unrelated false theorem.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, eval, compose_env, substitute)

const bad_one : Nat = Suc Zero
const bad_two : Nat = Suc bad_one
const bad_three : Nat = Suc bad_two
const bad_four : Nat = Suc bad_three

fn bad_env0 (v : Nat) : Nat = Suc v

fn bad_sub0 (v : Nat) : Expr =
  match v {
    Zero ↦ EAdd (EVar Zero) (EConst bad_one);
    Suc w ↦ EMul (EVar bad_one) (EVar Zero)
  }

const bad_e0 : Expr = EAdd (EVar Zero) (EVar bad_one)

theorem negative_fixture_substituted_value_is_four :
  Equal Nat (eval bad_env0 (substitute bad_sub0 bad_e0)) bad_four = Proved

theorem negative_fixture_original_value_is_three :
  Equal Nat (eval bad_env0 bad_e0) bad_three = Proved

theorem negative_fixture_composed_value_is_four :
  Equal Nat (eval (compose_env bad_env0 bad_sub0) bad_e0) bad_four = Proved

theorem refuse_evaluation_under_uncomposed_env :
  Equal Nat (eval bad_env0 (substitute bad_sub0 bad_e0))
    (eval bad_env0 bad_e0) = Proved
```

### `stdlib/symbolic-algebra/reject-uncomposed-environment-evaluation`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §3.
- promise class: durable invariant.
- given: the same numerical `env0`, `sub0`, and `e0` operands as the
  positive evaluation case, repeated in a separate input.
- expect: the three preceding controls establish values `4`, `3`, and
  `4` for substitution, uncomposed evaluation, and composed evaluation;
  the claim that the first equals the second is refused at its own
  theorem with `TypeMismatch`.
- measured: the separate `Proved` claim is rejected after its exact
  operands and controls elaborate.
- claimed: evaluation after substitution is not generally evaluation
  under the uncomposed environment.
- the gap: this one closed counterexample does not identify the error
  of every false theorem over other operands.
- why: pinning the proposition to the D0 positive operands prevents an
  unrelated false equality from satisfying the refusal.

## Coverage boundary

The five cases cover a nontrivial closed substitution with sum and
product images, its identity control, one forward `poly_eq` instance,
the no-converse counterexample, and the exact uncomposed-
`env` refusal. The closed rows do not establish either universally
quantified law; Architect D0 separately checked both generic bodies.
No composition of substitutions, subtraction, signed coefficients,
function extensionality, new primitive, kernel rule, or surface feature
is asserted. The seed's scratch checks do not establish shipped
harness execution, CI, publication, or landing.
