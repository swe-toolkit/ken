# Cancellation of successor scaling on vectors and matrices

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §21, child
`3560c13a11969afe9a34ece478ac21e3c627b86b`.
Frame: `docs/program/issues/SPEC-LINEAR-ALGEBRA-SCALE-CANCEL.md`.

This seed gives generic clients and closed controls for the two §21
laws. The factor-two clients exercise the public laws at arbitrary
vector and matrix dimensions; closed equal-input controls and
independent value oracles ground the factor and premise forms. The
Zero-factor cases keep a true scaled-equality premise while changing
only the premise factor from two to Zero. Each law call keeps predecessor
`one`. Each refusal is at the whole
appended declaration, not a token-exact span. The seed has not been
executed.

## Accepted shared client

Use `v12 = [1,2]`, `v34 = [3,4]`, and one-row matrices `m12` and
`m34` containing those vectors. Factor two maps `v12` to `[2,4]`.
Factor Zero maps both vectors, and both one-row matrices, to their
respective zero-shaped values.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_scale, matrix_scale,
   vector_scale_cancel, matrix_scale_cancel)

const zero : Nat = Zero
const one : Nat = Suc zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three

fn vector2 (x : Nat) (y : Nat) : Vec Nat two =
  VCons Nat one x (VCons Nat zero y (VNil Nat))

fn matrix1 (row : Vec Nat two) : Vec (Vec Nat two) one =
  VCons (Vec Nat two) zero row (VNil (Vec Nat two))

const v12 : Vec Nat two = vector2 one two
const v34 : Vec Nat two = vector2 three four
const zero2 : Vec Nat two = vector2 zero zero
const m12 : Vec (Vec Nat two) one = matrix1 v12
const m34 : Vec (Vec Nat two) one = matrix1 v34

theorem vector_scale_open
    (n : Nat) (xs : Vec Nat n) (ys : Vec Nat n)
    (h : Equal (Vec Nat n)
      (vector_scale n two xs) (vector_scale n two ys))
  : Equal (Vec Nat n) xs ys =
      vector_scale_cancel n one xs ys h

theorem matrix_scale_open
    (m : Nat) (n : Nat)
    (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat n) m)
    (h : Equal (Vec (Vec Nat n) m)
      (matrix_scale m n two a) (matrix_scale m n two b))
  : Equal (Vec (Vec Nat n) m) a b =
      matrix_scale_cancel m n one a b h

theorem vector_cancel_at_factor
    (h : Equal (Vec Nat two)
      (vector_scale two two v12) (vector_scale two two v34))
  : Equal (Vec Nat two) v12 v34 =
      vector_scale_cancel two one v12 v34 h

theorem matrix_cancel_at_factor
    (h : Equal (Vec (Vec Nat two) one)
      (matrix_scale one two two m12) (matrix_scale one two two m34))
  : Equal (Vec (Vec Nat two) one) m12 m34 =
      matrix_scale_cancel one two one m12 m34 h

theorem vector_cancel_equal_control
  : Equal (Vec Nat two) v12 v12 =
      vector_scale_cancel two one v12 v12 Refl

theorem v12_refl_control
  : Equal (Vec Nat two) v12 v12 = Refl

theorem matrix_cancel_equal_control
  : Equal (Vec (Vec Nat two) one) m12 m12 =
      matrix_scale_cancel one two one m12 m12 Refl

theorem vector_cancel_empty_control
  : Equal (Vec Nat Zero) (VNil Nat) (VNil Nat) =
      vector_scale_cancel Zero one (VNil Nat) (VNil Nat) Proved

theorem vector_scale_two_value
  : Equal (Vec Nat two) (vector_scale two two v12) (vector2 two four) =
      Refl

theorem vector_scale_zero_v12_value
  : Equal (Vec Nat two) (vector_scale two Zero v12) zero2 = Refl

theorem vector_scale_zero_v34_value
  : Equal (Vec Nat two) (vector_scale two Zero v34) zero2 = Refl

theorem vector_scale_zero_premise
  : Equal (Vec Nat two)
      (vector_scale two Zero v12) (vector_scale two Zero v34) =
      Refl

theorem matrix_scale_zero_premise
  : Equal (Vec (Vec Nat two) one)
      (matrix_scale one two Zero m12) (matrix_scale one two Zero m34) =
      Refl
```

### stdlib/linear-algebra/vector-scale-cancel-factor-two

- spec: §21 (`vector_scale_cancel`).
- promise class: durable invariant.
- given: arbitrary `n`, `xs`, and `ys`, with a premise that their
  factor-two scaled values are equal; the same-form `v12`/`v34`
  factor-two client; and the closed `v12` control.
- expect: the open client applies `vector_scale_cancel n one` and
  concludes `Equal (Vec Nat n) xs ys`. The same-form client
  typechecks with an abstract premise. The closed equal-input
  application accepts with `Refl`; the zero-length control accepts
  with `Proved`.
- why: the public law has the exact successor factor and dependent
  argument order, including the zero-dimensional vector case.

### stdlib/linear-algebra/matrix-scale-cancel-factor-two

- spec: §21 (`matrix_scale_cancel`).
- promise class: durable invariant.
- given: arbitrary dimensions and matrices with a factor-two
  scaled-equality premise; the same-form `m12`/`m34` factor-two
  client; and the closed one-row `m12` control.
- expect: the open client applies `matrix_scale_cancel m n one`
  and concludes structural equality of the matrices. The same-form
  client typechecks with an abstract premise. The closed equal-input
  application accepts with `Refl`.
- why: the matrix law cancels the shared successor factor across the
  indexed matrix without changing its dimensions.

## Zero-factor premise controls

The accepted client above checks that the Zero-factor premise is true:
`vector_scale two Zero v12` and `vector_scale two Zero v34` both equal
`zero2`, and the corresponding one-row matrix premise also accepts
with `Refl`. Append each refused declaration below separately to a
fresh copy of the entire accepted client. Each has the same form as its
factor-two client except that `h` equates Zero-scaled inputs. The law
call keeps predecessor `one`, so it still expects factor two; only the
premise factor differs.

### stdlib/linear-algebra/reject-vector-scale-cancel-zero-factor

- spec: §21 (`vector_scale_cancel`).
- promise class: durable invariant.
- given: the accepted client and only the declaration below.
- expect: refuse with `TypeMismatch` over the whole appended
  declaration. Do not assert a finer source span. The call keeps
  predecessor `one` and expects factor two; `h` is an inhabited
  Zero-factor equality, so the mismatch is only at its premise.
- why: Zero scaling identifies the distinct vectors `[1,2]` and
  `[3,4]`; their equal images do not license cancellation.

```ken
theorem vector_cancel_at_zero
    (h : Equal (Vec Nat two)
      (vector_scale two Zero v12) (vector_scale two Zero v34))
  : Equal (Vec Nat two) v12 v34 =
      vector_scale_cancel two one v12 v34 h
```

### stdlib/linear-algebra/reject-matrix-scale-cancel-zero-factor

- spec: §21 (`matrix_scale_cancel`).
- promise class: durable invariant.
- given: the accepted client and only the declaration below.
- expect: refuse with `TypeMismatch` over the whole appended
  declaration. Do not assert a finer source span. The call keeps
  predecessor `one` and expects factor two; `h` is an inhabited
  Zero-factor matrix equality, so the mismatch is only at its premise.
- why: Zero scaling maps both one-row matrices to the same zero
  matrix, while their original rows differ.

```ken
theorem matrix_cancel_at_zero
    (h : Equal (Vec (Vec Nat two) one)
      (matrix_scale one two Zero m12)
      (matrix_scale one two Zero m34))
  : Equal (Vec (Vec Nat two) one) m12 m34 =
      matrix_scale_cancel one two one m12 m34 h
```

## Separate false-conclusion control

Start from a fresh copy of the accepted client and append only
`v12_is_v34` below. The accepted client already contains a direct equal-input `Refl`
control. This refusal does not rely on applying either cancellation
law.

### stdlib/linear-algebra/reject-vector-scale-cancel-false-conclusion

- spec: §21's separate false-conclusion control.
- promise class: durable invariant.
- given: the accepted client and only `v12_is_v34`.
- expect: the equal-input `Refl` control accepts; `v12_is_v34`
  refuses at its `Refl` because `[1,2]` and `[3,4]` differ.
- why: this independently establishes the conclusion is false, not
  merely that the Zero-factor law application has the wrong factor.

```ken
theorem v12_is_v34
  : Equal (Vec Nat two) v12 v34 = Refl
```

## Evidence boundary

Architect's AC-0 report describes scratch checks of the generic laws,
closed controls, Zero-factor premise and declaration-level refusals.
Those observations are not execution of this authored seed or of the
separate Spec, catalog or assembled candidate. No package test,
official conformance run, CI, publication, landing or landed-trust
result is claimed.

Clean-room provenance: no external references were consulted. This
seed was authored from the exact §21 Spec child, Architect AC-0, and
first-principles arithmetic; no reference implementation was read.
