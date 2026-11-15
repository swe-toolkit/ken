# Congruence and substitution for expression powers

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §18.
Frame: `docs/program/issues/SPEC-SYMBOLIC-ALGEBRA-EXPR-POWER-CONGRUENCE.md`.

This seed covers the public base-congruence and substitution laws for
`expr_pow`. The first law preserves canonical `poly_eq` under the same
exponent; its premise is required for general exponents. The second law
states structural equality of the expression trees after substitution,
not merely canonical equality. Each refusal is appended alone to a fresh
copy of the accepted block below. The common base does not yet contain
the two new exports; these cases target the assembled candidate. The
Architect's D0 checked these client rows in scratch; that is not official
conformance execution, CI, publication, or landing.

## Open generic clients

Check each client with its selective import. Each applies a public theorem
at the exact quantified premise and conclusion types rather than restating
the claim as a closed computation.

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, poly_eq, expr_pow, substitute, poly_eq_expr_pow, substitute_expr_pow)

theorem congruence_generic
      (n : Nat) (e1 : Expr) (e2 : Expr) (h : IsTrue (poly_eq e1 e2))
    : IsTrue (poly_eq (expr_pow e1 n) (expr_pow e2 n)) =
  poly_eq_expr_pow n e1 e2 h

theorem substitution_generic
      (s : Nat → Expr) (e : Expr) (n : Nat)
    : Equal Expr (substitute s (expr_pow e n)) (expr_pow (substitute s e) n) =
  substitute_expr_pow s e n
```

### stdlib/symbolic-algebra/expression-power-congruence-generic

- spec: §18, `poly_eq_expr_pow` and `substitute_expr_pow`.
- promise class: durable invariant.
- given: arbitrary exponent, expressions, substitution and the stated
  `poly_eq` premise.
- expect: both clients accept through direct import and application of
  the public checked laws.
- measured: the open clients type-check at the law's quantified types.
- claimed: callers can reuse both laws without narrowing their domains
  or adding premises.
- gap: an imported theorem's availability is not a separate proof of
  its body; the package checks the generic proofs.
- why: open parameters prevent these applications from being established
  solely by reducing the closed examples below.

## Accepted closed clients

Check the whole block. The commuted sums are syntactically different but
canonically equal. The two power squares are checked both by computation
and by the public congruence law. The substitution replaces `x0` by a
non-variable expression, and its substituted-power results are checked
against that explicit tree and by the public structural law.

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, poly_eq, expr_pow, substitute,
   poly_eq_expr_pow, substitute_expr_pow)

const one : Nat = Suc Zero
const two : Nat = Suc one

const x0 : Expr = EVar Zero
const x1 : Expr = EVar one
const sum01 : Expr = EAdd x0 x1
const sum10 : Expr = EAdd x1 x0

theorem commuted_sums_premise : IsTrue (poly_eq sum01 sum10) = Proved

theorem c_closed_squares : IsTrue (poly_eq (expr_pow sum01 two) (expr_pow sum10 two)) = Proved

theorem c_law_squares : IsTrue (poly_eq (expr_pow sum01 two) (expr_pow sum10 two)) =
  poly_eq_expr_pow two sum01 sum10 commuted_sums_premise

fn replace_zero (v : Nat) : Expr =
  match v {
    Zero ↦ EAdd x1 (EConst two);
    Suc w ↦ EVar (Suc w)
  }

const replacement : Expr = EAdd x1 (EConst two)
const substituted_square : Expr = EMul (EMul (EConst one) replacement) replacement
const unsubstituted_square : Expr = EMul (EMul (EConst one) x0) x0

theorem s_closed_substituted_power
    : Equal Expr (substitute replace_zero (expr_pow x0 two)) substituted_square =
  Refl

theorem s_closed_power_of_substituted
    : Equal Expr (expr_pow (substitute replace_zero x0) two) substituted_square =
  Refl

theorem s_closed_unsubstituted_power : Equal Expr (expr_pow x0 two) unsubstituted_square = Refl

theorem s_law_square
    : Equal Expr (substitute replace_zero (expr_pow x0 two)) (expr_pow (substitute replace_zero x0) two) =
  substitute_expr_pow replace_zero x0 two
```

### stdlib/symbolic-algebra/expression-power-congruence-closed

- spec: §18, both public laws.
- promise class: durable invariant.
- given: `n = 2`, commuted sums, and `replace_zero` sending zero to
  `EAdd x1 (EConst two)` while fixing other variables.
- expect: the complete block accepts, including closed value controls
  and each public law application.
- measured: canonical equality of the squares, structural values of
  both substituted-power expressions, and the public theorem instances.
- claimed: the specified congruence and structural substitution results
  hold on these closed clients.
- gap: finite instances do not establish either generic proof or measure
  the package's trusted-base delta.
- why: distinct commuted sums test congruence under canonical equality;
  the non-variable replacement distinguishes substitution from omission.

## Refuse congruence without its premise

For X-BASE, append only the final theorem to a fresh copy of the accepted
block. The prefix must accept first. The closed false claim has the same
`IsTrue (poly_eq ...) = Proved` form as the accepted square control.

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, poly_eq, expr_pow, substitute,
   poly_eq_expr_pow, substitute_expr_pow)

const one : Nat = Suc Zero
const two : Nat = Suc one
const x0 : Expr = EVar Zero
const x1 : Expr = EVar one
const sum01 : Expr = EAdd x0 x1
const sum10 : Expr = EAdd x1 x0

theorem commuted_sums_premise : IsTrue (poly_eq sum01 sum10) = Proved

theorem c_closed_squares : IsTrue (poly_eq (expr_pow sum01 two) (expr_pow sum10 two)) = Proved

theorem c_law_squares : IsTrue (poly_eq (expr_pow sum01 two) (expr_pow sum10 two)) =
  poly_eq_expr_pow two sum01 sum10 commuted_sums_premise

fn replace_zero (v : Nat) : Expr =
  match v {
    Zero ↦ EAdd x1 (EConst two);
    Suc w ↦ EVar (Suc w)
  }

const replacement : Expr = EAdd x1 (EConst two)
const substituted_square : Expr = EMul (EMul (EConst one) replacement) replacement
const unsubstituted_square : Expr = EMul (EMul (EConst one) x0) x0

theorem s_closed_substituted_power
    : Equal Expr (substitute replace_zero (expr_pow x0 two)) substituted_square =
  Refl

theorem s_closed_power_of_substituted
    : Equal Expr (expr_pow (substitute replace_zero x0) two) substituted_square =
  Refl

theorem s_closed_unsubstituted_power : Equal Expr (expr_pow x0 two) unsubstituted_square = Refl

theorem s_law_square
    : Equal Expr (substitute replace_zero (expr_pow x0 two)) (expr_pow (substitute replace_zero x0) two) =
  substitute_expr_pow replace_zero x0 two

theorem x_base_squares : IsTrue (poly_eq (expr_pow x0 two) (expr_pow x1 two)) = Proved
```

### stdlib/symbolic-algebra/reject-power-congruence-without-premise

- spec: §18, necessity of the `poly_eq` premise for
  `poly_eq_expr_pow`.
- promise class: durable invariant.
- given: the fresh accepted block followed by X-BASE, with distinct
  variables `x0` and `x1` and exponent two.
- expect: the prefix accepts; X-BASE is refused at its own `Proved`
  with `KernelRejected(TypeMismatch)`.
- measured: the D0 refusal has that kind and a span within
  `x_base_squares` enclosing its terminal `Proved`.
- claimed: the congruence law cannot be used without evidence that the
  bases are `poly_eq`-equal.
- gap: this closed refusal does not characterize every invalid premise
  or every incorrect implementation of the generic law.
- controls: `c_closed_squares` accepts the same claim form; the
  commuted-sums premise and its law instance are also accepted.
- why: `x0` and `x1` have distinct canonical squares, so the false
  proposition is not inhabited by `Proved`.

## Refuse substitution with its action dropped

For X-DROP-SUB, append only the final theorem to a fresh copy of the
accepted block. The prefix must accept first. The false equality drops
`substitute` from the right endpoint, while the non-variable replacement
and both independently checked endpoint values remain unchanged.

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, poly_eq, expr_pow, substitute,
   poly_eq_expr_pow, substitute_expr_pow)

const one : Nat = Suc Zero
const two : Nat = Suc one
const x0 : Expr = EVar Zero
const x1 : Expr = EVar one
const sum01 : Expr = EAdd x0 x1
const sum10 : Expr = EAdd x1 x0

theorem commuted_sums_premise : IsTrue (poly_eq sum01 sum10) = Proved

theorem c_closed_squares : IsTrue (poly_eq (expr_pow sum01 two) (expr_pow sum10 two)) = Proved

theorem c_law_squares : IsTrue (poly_eq (expr_pow sum01 two) (expr_pow sum10 two)) =
  poly_eq_expr_pow two sum01 sum10 commuted_sums_premise

fn replace_zero (v : Nat) : Expr =
  match v {
    Zero ↦ EAdd x1 (EConst two);
    Suc w ↦ EVar (Suc w)
  }

const replacement : Expr = EAdd x1 (EConst two)
const substituted_square : Expr = EMul (EMul (EConst one) replacement) replacement
const unsubstituted_square : Expr = EMul (EMul (EConst one) x0) x0

theorem s_closed_substituted_power
    : Equal Expr (substitute replace_zero (expr_pow x0 two)) substituted_square =
  Refl

theorem s_closed_power_of_substituted
    : Equal Expr (expr_pow (substitute replace_zero x0) two) substituted_square =
  Refl

theorem s_closed_unsubstituted_power : Equal Expr (expr_pow x0 two) unsubstituted_square = Refl

theorem s_law_square
    : Equal Expr (substitute replace_zero (expr_pow x0 two)) (expr_pow (substitute replace_zero x0) two) =
  substitute_expr_pow replace_zero x0 two

theorem x_drop_substitution : Equal Expr (substitute replace_zero (expr_pow x0 two)) (expr_pow x0 two) = Refl
```

### stdlib/symbolic-algebra/reject-power-substitution-dropped

- spec: §18, `substitute_expr_pow`.
- promise class: durable invariant.
- given: the fresh accepted block followed by X-DROP-SUB.
- expect: the prefix accepts; X-DROP-SUB is refused at its own `Refl`
  with `ElabError::TypeMismatch`, reason `Refl expects an `Eq`-shaped
  goal`.
- measured: the D0 refusal span is inside `x_drop_substitution` at its
  `Refl` terminal.
- claimed: substitution cannot be omitted from the right side of the
  structural law.
- gap: this closed refusal does not characterize every incorrect
  implementation of substitution over powers.
- controls: `s_closed_substituted_power` and
  `s_closed_power_of_substituted` independently fix the two endpoint
  values; `s_law_square` applies the public law in the same input.
- why: the left side contains the replacement expression, while the
  right side still contains variable `x0`.

## Refuse structural congruence

For X-STRUCT, append only the final theorem to a fresh copy of the
accepted block. The prefix must accept first. It establishes canonical
congruence of the commuted-sum squares, not structural equality; the
same-form `Equal Expr` controls establish valid `Refl` behavior on the
closed square expressions.

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, poly_eq, expr_pow, substitute,
   poly_eq_expr_pow, substitute_expr_pow)

const one : Nat = Suc Zero
const two : Nat = Suc one
const x0 : Expr = EVar Zero
const x1 : Expr = EVar one
const sum01 : Expr = EAdd x0 x1
const sum10 : Expr = EAdd x1 x0

theorem commuted_sums_premise : IsTrue (poly_eq sum01 sum10) = Proved

theorem c_closed_squares : IsTrue (poly_eq (expr_pow sum01 two) (expr_pow sum10 two)) = Proved

theorem c_law_squares : IsTrue (poly_eq (expr_pow sum01 two) (expr_pow sum10 two)) =
  poly_eq_expr_pow two sum01 sum10 commuted_sums_premise

fn replace_zero (v : Nat) : Expr =
  match v {
    Zero ↦ EAdd x1 (EConst two);
    Suc w ↦ EVar (Suc w)
  }

const replacement : Expr = EAdd x1 (EConst two)
const substituted_square : Expr = EMul (EMul (EConst one) replacement) replacement
const unsubstituted_square : Expr = EMul (EMul (EConst one) x0) x0

theorem s_closed_substituted_power
    : Equal Expr (substitute replace_zero (expr_pow x0 two)) substituted_square =
  Refl

theorem s_closed_power_of_substituted
    : Equal Expr (expr_pow (substitute replace_zero x0) two) substituted_square =
  Refl

theorem s_closed_unsubstituted_power : Equal Expr (expr_pow x0 two) unsubstituted_square = Refl

theorem s_law_square
    : Equal Expr (substitute replace_zero (expr_pow x0 two)) (expr_pow (substitute replace_zero x0) two) =
  substitute_expr_pow replace_zero x0 two

theorem x_structural_squares : Equal Expr (expr_pow sum01 two) (expr_pow sum10 two) = Refl
```

### stdlib/symbolic-algebra/reject-structural-power-congruence

- spec: §18, `poly_eq_expr_pow` concludes `IsTrue (poly_eq ...)`, not
  structural `Equal Expr`.
- promise class: durable invariant.
- given: the fresh accepted block followed by X-STRUCT.
- expect: the prefix accepts; X-STRUCT is refused at its own `Refl`
  with `ElabError::TypeMismatch`, reason `Refl expects an `Eq`-shaped
  goal`.
- measured: the D0 refusal span is inside `x_structural_squares` at
  its `Refl` terminal.
- claimed: canonical equality of the bases does not imply structural
  equality of their expression-power trees.
- gap: this closed refusal does not characterize every pair of
  canonically equal but structurally distinct expressions.
- controls: `c_closed_squares` checks canonical equality for these
  inputs; `s_closed_unsubstituted_power` is an accepted same-form
  structural-equality control on a closed power expression.
- why: the swapped `EAdd` bases are `poly_eq`-equal but their power
  trees retain different operand order.

## Coverage boundary

The open clients apply each public law generically. The accepted block
checks both laws at closed arguments and fixes the values used by the
refusals. Each refusal runs alone after a fresh accepted prefix. The
D0 observations concern the same client rows in scratch; the seed has
not been run through the official conformance runner. These closed
clients do not establish the generic proof bodies, package test
execution, CI, publication, or landing.
