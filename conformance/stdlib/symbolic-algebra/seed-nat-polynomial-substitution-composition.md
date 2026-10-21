# Composition and identity of expression substitutions

Format: ../../README.md.
Spec: spec/50-stdlib/66-symbolic-algebra.md §4.

This seed records closed cases for the §4 composition and identity
contract. AC-1's approved contract is at exact candidate SHA
`494fa919c13140f420e5dd37fdcfa0106dea1a35`. Architect D0 checked the
generic law bodies and this fixture's reversed-order refusal in scratch.
That is scratch evidence, not execution of this seed. The authored cases
do not claim CI, publication, or landing.

Every case has promise class **durable invariant**.

## Closed fixture and accepted controls

`inner0` inserts `EVar one` at index zero and leaves successor indices
unchanged. `outer0` leaves index zero alone and maps every successor
index to `EConst two`. The expression `e0` is a closed syntax value.
Single-pass substitution gives the forward result
`EAdd (EConst two) (EConst one)`. Reversing the composition gives
`EAdd (EVar one) (EConst one)`, a different structural expression.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, substitute, compose_subst,
   substitute_compose, substitute_var)

const one : Nat = Suc Zero
const two : Nat = Suc one

fn inner0 (v : Nat) : Expr =
  match v {
    Zero ↦ EVar one;
    Suc w ↦ EVar (Suc w)
  }

fn outer0 (v : Nat) : Expr =
  match v {
    Zero ↦ EVar Zero;
    Suc w ↦ EConst two
  }

const e0 : Expr = EAdd (EVar Zero) (EConst one)
const expected_forward0 : Expr = EAdd (EConst two) (EConst one)
const expected_reverse0 : Expr = EAdd (EVar one) (EConst one)

theorem forward_nested_has_expected_tree :
  Equal Expr (substitute outer0 (substitute inner0 e0))
    expected_forward0 = Refl

theorem forward_composed_map_has_expected_tree :
  Equal Expr (substitute (compose_subst outer0 inner0) e0)
    expected_forward0 = Refl

theorem public_forward_composition_instance :
  Equal Expr (substitute outer0 (substitute inner0 e0))
    (substitute (compose_subst outer0 inner0) e0) =
  substitute_compose outer0 inner0 e0

theorem reversed_builder_has_distinct_tree :
  Equal Expr (substitute (compose_subst inner0 outer0) e0)
    expected_reverse0 = Refl

theorem constructor_identity_reduces :
  Equal Expr (substitute EVar e0) e0 = Refl

theorem constructor_identity_public_law :
  Equal Expr (substitute EVar e0) e0 =
  substitute_var e0
```

### `stdlib/symbolic-algebra/substitution-composition-order`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §4.
- promise class: durable invariant.
- given: `inner0`, `outer0`, and `e0`; `inner0 Zero` inserts a variable
  that `outer0` replaces.
- expect: both forward expressions equal
  `EAdd (EConst two) (EConst one)`; the public
  `substitute_compose outer0 inner0 e0` instance is accepted; the reversed
  builder equals `EAdd (EVar one) (EConst one)`.
- measured: structural `Equal Expr` results and a closed application of
  the public composition law.
- claimed: `compose_subst outer0 inner0` represents applying `inner0`
  and then `outer0`, and reversing the order changes the expression.
- the gap: one closed fixture does not establish the universally
  quantified composition theorem.
- why: the two outputs differ on the same fixture, so the order boundary
  is observable.

### `stdlib/symbolic-algebra/substitution-constructor-identity`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §4.
- promise class: durable invariant.
- given: `e0` and the exported constructor `EVar` as the substitution map.
- expect: `substitute EVar e0` equals `e0`; the public
  `substitute_var e0` instance is accepted.
- measured: a structural `Refl` control and the public law application.
- claimed: the constructor-identity substitution preserves this closed
  expression.
- the gap: this closed instance does not state identity for an arbitrary
  extensionally equal function.
- why: the spec binds identity to `EVar`, not function extensionality.

## Reversed-order base client

This second Ken fence is an expected-accept client. Check it alone; once
the §4 catalog exports exist, all six controls should be accepted. Test
the refusal only by appending a separate one-theorem fence to a fresh copy
of this accepted base.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, substitute, compose_subst,
   substitute_compose)

const bad_one : Nat = Suc Zero
const bad_two : Nat = Suc bad_one

fn bad_inner0 (v : Nat) : Expr =
  match v {
    Zero ↦ EVar bad_one;
    Suc w ↦ EVar (Suc w)
  }

fn bad_outer0 (v : Nat) : Expr =
  match v {
    Zero ↦ EVar Zero;
    Suc w ↦ EConst bad_two
  }

const bad_e0 : Expr = EAdd (EVar Zero) (EConst bad_one)
const bad_forward0 : Expr = EAdd (EConst bad_two) (EConst bad_one)
const bad_reverse0 : Expr = EAdd (EVar bad_one) (EConst bad_one)

theorem refusal_fixture_forward_output :
  Equal Expr (substitute bad_outer0 (substitute bad_inner0 bad_e0))
    bad_forward0 = Refl

theorem refusal_fixture_forward_composed_output :
  Equal Expr (substitute (compose_subst bad_outer0 bad_inner0) bad_e0)
    bad_forward0 = Refl

theorem refusal_fixture_forward_law :
  Equal Expr (substitute bad_outer0 (substitute bad_inner0 bad_e0))
    (substitute (compose_subst bad_outer0 bad_inner0) bad_e0) =
  substitute_compose bad_outer0 bad_inner0 bad_e0

theorem refusal_fixture_reverse_nested_output :
  Equal Expr (substitute bad_inner0 (substitute bad_outer0 bad_e0))
    bad_reverse0 = Refl

theorem refusal_fixture_reverse_composed_output :
  Equal Expr (substitute (compose_subst bad_inner0 bad_outer0) bad_e0)
    bad_reverse0 = Refl

theorem refusal_fixture_reverse_law :
  Equal Expr (substitute bad_inner0 (substitute bad_outer0 bad_e0))
    (substitute (compose_subst bad_inner0 bad_outer0) bad_e0) =
  substitute_compose bad_inner0 bad_outer0 bad_e0
```

## Append the reversed-order refusal

For the refusal case, start from a fresh copy of the accepted `bad_*`
base above and append only this fence. Expect the base alone to accept
and the appended input to reject with `TypeMismatch` at
`refuse_reversed_composition_claim`. The package test executes the accepted
base and then the isolated refusal; no official conformance-runner execution
is claimed.

```ken
theorem refuse_reversed_composition_claim :
  Equal Expr (substitute bad_outer0 (substitute bad_inner0 bad_e0))
    (substitute (compose_subst bad_inner0 bad_outer0) bad_e0) =
  substitute_compose bad_inner0 bad_outer0 bad_e0
```

### `stdlib/symbolic-algebra/reject-reversed-substitution-composition`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §4.
- promise class: durable invariant.
- given: check the `bad_*` base fence alone; then append only the refusal
  fence to a fresh copy of that accepted base.
- expect: the base client accepts; the appended input rejects with
  `TypeMismatch` at `refuse_reversed_composition_claim`.
- measured: the package test accepts the base and kernel-refuses the
  isolated theorem with `TypeMismatch`; the authored recipe separates base
  acceptance from the appended refusal.
- claimed: the law cannot be applied with the composition arguments
  reversed.
- the gap: this single closed fixture does not characterize every pair
  of substitutions.
- why: a separately accepted base shows all six controls pass before the
  one false theorem is appended, making the refusal attributable to order.

## Coverage boundary

The first client asserts both forward structural outputs, the public
composition-law instance, the reversed output, and constructor identity
with `EVar`. The `bad_*` base is expected to accept when checked alone;
append only the refusal fence to a fresh copy to test the reversed claim.
The positive fences and isolated refusal are executed by
`crates/ken-elaborator/tests/cat_symbolic_algebra_substitution_composition.rs`.
No official conformance-runner execution, CI, publication, or landing is
claimed. AC-0's scratch checks remain separate evidence.
