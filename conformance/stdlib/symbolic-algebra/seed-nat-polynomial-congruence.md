# Congruence of canonical equality under expression constructors

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §10.

These cases are closed clients of the two §10 exports. They are acceptance
cases, not evidence of official conformance-runner execution. No official
runner execution, CI, publication, landing, or trust measurement is claimed.
`IsTrue` is used without an import.

Every case has promise class **durable invariant**.

## Shared fixture and accepted public applications

The first pair denotes the same polynomial: `x0_sum` is `x0 + x0`, and
`two_times_x0` is `2 * x0`. The second pair is `x1` on both sides. The
`all_ones` environment distinguishes the false mixed-constructor claim below:
its left value is two and its right value is three.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, eval, poly_eq,
   poly_eq_add, poly_eq_mul)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two

const x0 : Expr = EVar Zero
const x1 : Expr = EVar one
const x0_sum : Expr = EAdd x0 x0
const two_times_x0 : Expr = EMul (EConst two) x0

fn all_ones (v : Nat) : Nat = one

theorem x0_pair_is_true :
  IsTrue (poly_eq x0_sum two_times_x0) = Proved

theorem x1_pair_is_true :
  IsTrue (poly_eq x1 x1) = Proved

theorem add_result_is_true :
  IsTrue (poly_eq (EAdd x0_sum x1) (EAdd two_times_x0 x1)) = Proved

theorem mul_result_is_true :
  IsTrue (poly_eq (EMul x0_sum x1) (EMul two_times_x0 x1)) = Proved

theorem poly_eq_add_closed_client :
  IsTrue (poly_eq (EAdd x0_sum x1) (EAdd two_times_x0 x1)) =
  poly_eq_add x0_sum two_times_x0 x1 x1 x0_pair_is_true x1_pair_is_true

theorem poly_eq_mul_closed_client :
  IsTrue (poly_eq (EMul x0_sum x1) (EMul two_times_x0 x1)) =
  poly_eq_mul x0_sum two_times_x0 x1 x1 x0_pair_is_true x1_pair_is_true

theorem x0_sum_value_at_ones :
  Equal Nat (eval all_ones x0_sum) two = Proved

theorem two_times_x0_value_at_ones :
  Equal Nat (eval all_ones two_times_x0) two = Proved

theorem x1_value_at_ones :
  Equal Nat (eval all_ones x1) one = Proved

theorem mixed_mul_value_at_ones :
  Equal Nat (eval all_ones (EMul x0_sum x1)) two = Proved

theorem mixed_add_value_at_ones :
  Equal Nat (eval all_ones (EAdd two_times_x0 x1)) three = Proved
```

### stdlib/symbolic-algebra/poly-eq-add-mul-congruence-closed-client

- spec: §10.
- promise class: durable invariant.
- given: the complete shared input above.
- expect: the two operand-pair `IsTrue` premises and both direct result
  controls accept. The `poly_eq_add` and `poly_eq_mul` applications accept
  with arguments in §10 order: `x0_sum`, `two_times_x0`, `x1`, `x1`,
  then the two premise proofs.
- measured: closed applications of each public theorem, plus direct
  `IsTrue` controls for both premises and both conclusions.
- claimed: both §10 constructor-preservation contracts are usable by a
  client on this closed pair.
- the gap: these closed applications do not establish the generic theorem
  bodies for every `Expr` quadruple.
- why: the application inputs match the four expression arguments and the
  two required premises exactly; no relational law is imported or inferred.

## Refuse a false outer-constructor claim

The negative input is a fresh checker input containing the complete accepted
shared input above. The checker must accept every declaration in that prefix
before the following single claim is appended:

```ken
theorem reject_mixed_outer_constructors :
  IsTrue (poly_eq (EMul x0_sum x1) (EAdd two_times_x0 x1)) = Proved
```

### stdlib/symbolic-algebra/poly-eq-congruence-refuses-outer-mismatch

- spec: §10 and §2.
- promise class: durable invariant.
- given: the accepted pair and both true premise controls above, followed
  only by `reject_mixed_outer_constructors` in a fresh input.
- expect: the prefix accepts and the appended claim is refused. Both operand
  premises remain true; at `all_ones`, the claimed left and right sides
  evaluate to two and three.
- measured: the same checked `poly_eq` operation accepts the `EAdd` and
  `EMul` positive controls, while the mixed-outer claim is refused; the
  independent `eval` controls establish the two unequal values.
- claimed: the two true operand-pair premises do not entail equality after
  changing only the right outer constructor.
- the gap: this closed counterexample does not prove the generic law bodies
  or characterize all unequal expressions.
- why: the refusal is appended after a fully accepted prefix, retains both
  true premises, and differs from the accepted multiplication result only
  in the right outer constructor.

## Refuse a false second premise

This is another fresh checker input. Start with the complete accepted shared
input above and confirm that prefix accepts. Then add the following controls
and append only the final, deliberately false law application:

```ken
const zero_expr : Expr = EConst Zero

theorem zero_pair_is_true :
  IsTrue (poly_eq zero_expr zero_expr) = Proved

theorem distinct_variables_are_false :
  Equal Bool (poly_eq x0 x1) False = Proved

theorem zero_products_are_true :
  IsTrue (poly_eq (EMul zero_expr x0) (EMul zero_expr x1)) = Proved

theorem reject_mul_with_false_second_premise :
  IsTrue (poly_eq (EMul zero_expr x0) (EMul zero_expr x1)) =
  poly_eq_mul zero_expr zero_expr x0 x1 zero_pair_is_true Proved
```

### stdlib/symbolic-algebra/poly-eq-mul-requires-true-second-premise

- spec: §10 and §2.
- promise class: durable invariant.
- given: the complete accepted shared input, then the three added controls
  above, followed by `reject_mul_with_false_second_premise` in a fresh input.
- expect: the prefix and controls accept, including the direct true result
  for multiplying zero by either variable. The final application is refused
  because `Proved` cannot supply the false premise `IsTrue (poly_eq x0 x1)`.
- measured: the `x0`/`x1` control returns `False`, while the multiplication
  conclusion is independently accepted as `True`; only the second proof
  argument is false.
- claimed: the second operand-pair premise is required even when the
  conclusion happens to be true for another reason.
- the gap: this closed control does not establish all failure modes for
  either public theorem.
- why: the same multiplication law has an accepted true-premise instance
  above; this input keeps its conclusion true while making only its second
  premise unprovable.

## Coverage boundary

The seed covers one closed accepted application of each §10 law, their true
premises and conclusions, a mixed-outer refusal with both premises true, and
a false-second-premise refusal whose conclusion is independently true. It does
not prove the generic laws, test their catalog proof bodies directly, execute
the seed with Ken or the official runner, or establish CI, publication, landing,
or trust measurement.
