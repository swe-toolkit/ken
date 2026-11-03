# Pointwise congruence for polynomial substitution

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §13, as ruled by AC-0.

This seed specifies closed clients of the AC-0-ruled
`poly_eq_substitute_pointwise` contract. The Spec and implementation
children are separate work; this seed does not establish that either has
landed. It has not been run by the official conformance runner, and no CI,
publication, landing, or trust measurement is claimed. `IsTrue` is used
without an import.

Every case has promise class **durable invariant**.

## Accepted law application and same-form control

The expression is `x0 * x0`. The first substitution maps variable zero
to `x0 + x1`; the second maps it to `x1 + x0`. Both substitutions map
every successor variable to itself. Their images are canonically equal
at each variable, so the public pointwise law applies.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EVar, EAdd, EMul, poly_eq, substitute, poly_eq_refl, poly_eq_substitute_pointwise)

const one : Nat = Suc Zero

const square : Expr = EMul (EVar Zero) (EVar Zero)

fn images_left (v : Nat) : Expr =
  match v {
    Zero ↦ EAdd (EVar Zero) (EVar one);
    Suc w ↦ EVar (Suc w)
  }

fn images_right (v : Nat) : Expr =
  match v {
    Zero ↦ EAdd (EVar one) (EVar Zero);
    Suc w ↦ EVar (Suc w)
  }

theorem images_agree (v : Nat) : IsTrue (poly_eq (images_left v) (images_right v)) =
  match v {
    Zero ↦ Proved;
    Suc w ↦ poly_eq_refl (EVar (Suc w))
  }

theorem pointwise_substitution_law_client
    : IsTrue (poly_eq (substitute images_left square) (substitute images_right square)) =
  poly_eq_substitute_pointwise images_left images_right square images_agree

theorem same_form_proved_control
    : IsTrue (poly_eq (substitute images_left square) (substitute images_right square)) =
  Proved
```

### `stdlib/symbolic-algebra/pointwise-substitution-congruence-client`

- spec: §13 and §2.
- promise class: durable invariant.
- given: `square` and the two substitutions above, with `images_agree`
  proving canonical equality of their images for every variable.
- expect: the public law application and the same-form `Proved` control
  both accept.
- measured: one closed application of `poly_eq_substitute_pointwise`
  and a direct proof of its identical conclusion.
- claimed: pointwise canonical equality of substitution images suffices
  for canonical equality after substituting into this expression.
- the gap: this closed client does not prove the universally quantified
  public theorem.
- why: the direct control makes the law's conclusion independently
  checkable on the exact accepted fixture.

## Refuse a substitution that drops one summand

This is a separate checker input repeating the accepted fixture. It
changes only the second substitution's image at variable zero: instead
of `x1 + x0`, `drop_left` returns `x1`. Its other images still match
`images_left`. The two substituted squares are not canonically equal.
The final `Proved` claim must be refused at its own theorem.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial (Expr, EVar, EAdd, EMul, poly_eq, substitute)

const one : Nat = Suc Zero

const square : Expr = EMul (EVar Zero) (EVar Zero)

fn images_left (v : Nat) : Expr =
  match v {
    Zero ↦ EAdd (EVar Zero) (EVar one);
    Suc w ↦ EVar (Suc w)
  }

fn drop_left (v : Nat) : Expr =
  match v {
    Zero ↦ EVar one;
    Suc w ↦ EVar (Suc w)
  }

theorem drop_left_result_is_false
    : Equal Bool (poly_eq (substitute images_left square) (substitute drop_left square)) False =
  Proved

theorem refuse_drop_left_claim
    : IsTrue (poly_eq (substitute images_left square) (substitute drop_left square)) =
  Proved
```

### `stdlib/symbolic-algebra/refuse-pointwise-substitution-after-image-change`

- spec: §13 and §2.
- promise class: durable invariant.
- given: the repeated square and `images_left`, with only the
  variable-zero image of the second substitution changed to `x1`.
- expect: the direct `False` control accepts; the same-result `IsTrue`
  claim by `Proved` is refused at its own theorem.
- measured: `poly_eq` returns `False` for the two substituted squares;
  the separate positive result control rules out an unrelated refusal.
- claimed: changing an image so that it is not canonically equal to
  the corresponding first image invalidates this example's conclusion.
- the gap: this counterexample does not characterize all unequal image
  pairs or prove the public law.
- why: the refused claim repeats the accepted fixture and varies only
  the second substitution's image at variable zero.

## Coverage boundary

The seed covers a closed application through the public pointwise law,
its same-form direct control, and a separate refusal with the image
change isolated at variable zero. It does not prove the generic law,
measure a trust delta, or establish official conformance-runner
execution, CI, publication, or landing.
