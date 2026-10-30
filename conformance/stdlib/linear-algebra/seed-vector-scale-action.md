# Natural scaling acts on vector sums

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md §9`.
Frame: `docs/program/issues/SPEC-LINEAR-ALGEBRA-VECTOR-SCALE-ACTION.md`.

The accepted client applies all four §9 exports at abstract dimensions,
factors and vectors, then instantiates them at length two and independently
checks both sides against closed vector values. The open applications pin the
public generic types; finite instances alone do not. This assembled candidate
includes the checked exports and a package-owned test. Architect's AC-0
reported scratch checks of the AC-2 client and isolated false claim; the
prior-tip candidate-local package target and mutation are separate QA
evidence, not a test run on this corrected seed or an official
conformance-runner, CI, publication, or landing result. Each case
promises a durable invariant.

## Accepted shared client

Use `x=2`, `y=3`, `u=[1,2]`, and `v=[3,0]`. The independently computed
values on both sides of the four laws are `[8,4]`, `[5,10]`, `[6,12]`, and
`[1,2]`, respectively. The client invokes each generic theorem export directly;
the separate `Refl` checks are closed value oracles, not replacements for
open or closed law applications.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)
import Data.Numeric.Nat.Arithmetic (add, mul)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_add, vector_scale, vector_scale_distrib,
   vector_scale_factor_distrib, vector_scale_compose, vector_scale_one)

theorem vector_scale_distrib_open
    (n : Nat) (x : Nat) (u : Vec Nat n) (v : Vec Nat n)
  : Equal
      (Vec Nat n)
      (vector_scale n x (vector_add n u v))
      (vector_add n (vector_scale n x u) (vector_scale n x v)) =
  vector_scale_distrib n x u v

theorem vector_scale_factor_distrib_open
    (n : Nat) (x : Nat) (y : Nat) (u : Vec Nat n)
  : Equal
      (Vec Nat n)
      (vector_scale n (add x y) u)
      (vector_add n (vector_scale n x u) (vector_scale n y u)) =
  vector_scale_factor_distrib n x y u

theorem vector_scale_compose_open
    (n : Nat) (x : Nat) (y : Nat) (u : Vec Nat n)
  : Equal
      (Vec Nat n)
      (vector_scale n (mul x y) u)
      (vector_scale n x (vector_scale n y u)) =
  vector_scale_compose n x y u

theorem vector_scale_one_open
    (n : Nat) (u : Vec Nat n)
  : Equal (Vec Nat n) (vector_scale n (Suc Zero) u) u =
  vector_scale_one n u

const zero : Nat = Zero
const one : Nat = Suc zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four
const six : Nat = Suc five
const seven : Nat = Suc six
const eight : Nat = Suc seven
const nine : Nat = Suc eight
const ten : Nat = Suc nine
const eleven : Nat = Suc ten
const twelve : Nat = Suc eleven

fn vec2 (a : Nat) (b : Nat) : Vec Nat two =
  VCons Nat one a (VCons Nat zero b (VNil Nat))

const u2 : Vec Nat two = vec2 one two
const v2 : Vec Nat two = vec2 three zero
const value_8_4 : Vec Nat two = vec2 eight four
const value_5_10 : Vec Nat two = vec2 five ten
const value_6_12 : Vec Nat two = vec2 six twelve
const value_3_6 : Vec Nat two = vec2 three six
const value_1_2 : Vec Nat two = vec2 one two

theorem vector_scale_distrib_at_two
    : Equal (Vec Nat two)
        (vector_scale two two (vector_add two u2 v2))
        (vector_add two (vector_scale two two u2)
          (vector_scale two two v2)) =
  vector_scale_distrib two two u2 v2

theorem vector_scale_factor_distrib_at_two
    : Equal (Vec Nat two)
        (vector_scale two (add two three) u2)
        (vector_add two (vector_scale two two u2)
          (vector_scale two three u2)) =
  vector_scale_factor_distrib two two three u2

theorem vector_scale_compose_at_two
    : Equal (Vec Nat two)
        (vector_scale two (mul two three) u2)
        (vector_scale two two (vector_scale two three u2)) =
  vector_scale_compose two two three u2

theorem vector_scale_one_at_two
    : Equal (Vec Nat two) (vector_scale two (Suc Zero) u2) u2 =
  vector_scale_one two u2

theorem vector_scale_distrib_left_value
    : Equal (Vec Nat two)
        (vector_scale two two (vector_add two u2 v2)) value_8_4 =
  Refl

theorem vector_scale_distrib_right_value
    : Equal (Vec Nat two)
        (vector_add two (vector_scale two two u2)
          (vector_scale two two v2)) value_8_4 =
  Refl

theorem vector_scale_factor_distrib_left_value
    : Equal (Vec Nat two)
        (vector_scale two (add two three) u2) value_5_10 =
  Refl

theorem vector_scale_factor_distrib_right_value
    : Equal (Vec Nat two)
        (vector_add two (vector_scale two two u2)
          (vector_scale two three u2)) value_5_10 =
  Refl

theorem vector_scale_compose_left_value
    : Equal (Vec Nat two)
        (vector_scale two (mul two three) u2) value_6_12 =
  Refl

theorem vector_scale_compose_right_value
    : Equal (Vec Nat two)
        (vector_scale two two (vector_scale two three u2)) value_6_12 =
  Refl

theorem vector_scale_one_left_value
    : Equal (Vec Nat two)
        (vector_scale two (Suc Zero) u2) value_1_2 =
  Refl

theorem vector_scale_one_right_value
    : Equal (Vec Nat two) u2 value_1_2 =
  Refl

theorem dropped_scale_false_rhs_value
    : Equal (Vec Nat two)
        (vector_add two (vector_scale two two u2) u2) value_3_6 =
  Refl

theorem dropped_scale_same_scope_true_control
    : Equal (Vec Nat two)
        (vector_scale two (add two three) u2)
        (vector_add two (vector_scale two two u2)
          (vector_scale two three u2)) =
  Refl
```

### stdlib/linear-algebra/vector-scale-action-open

- spec: §9.1.
- promise class: durable invariant.
- given: abstract `n`, factors `x` and `y`, and `u` and `v` of type
  `Vec Nat n`.
- expect: all four public exports apply directly at their ruled generic
  types without specializing the dimension, factors or vectors.
- measured: four abstract-variable theorem applications in the accepted
  client; the first binds both vectors, the remaining three bind their
  respective factors and vector.
- claimed: the public interface admits all four §9.1 laws at arbitrary
  input dimensions and operands, including `n=Zero` by quantification.
- the gap: these client applications check the generic interface; the
  checked proof bodies and their trust boundary are separately reviewed.
- why: a dimension-two-only export cannot satisfy an abstract `n` client.

### stdlib/linear-algebra/vector-scale-action-closed

- spec: §9.1 and §9.2.
- promise class: durable invariant.
- given: `u2=[1,2]`, `v2=[3,0]`, `x=2`, and `y=3`.
- expect: all four public laws instantiate in their ruled argument order;
  each side independently computes to `[8,4]`, `[5,10]`, `[6,12]`, and
  `[1,2]`.
- measured: four direct theorem applications and eight independent `Refl`
  equalities against separately constructed vectors.
- claimed: the four generic exports have the specified types and agree with
  the closed operations on these inputs.
- the gap: closed values alone do not establish generic proof bodies;
  the open client above checks the public generic types separately.
- why: the applied theorem identities and separate value oracles distinguish
  unavailable or misordered exports from the closed vector calculations.

## Isolated refused claim

Start from a fresh copy of the entire accepted client above and append only the
false theorem below. Its left endpoint is the same as the accepted
`dropped_scale_same_scope_true_control`; only the final scaled operand is
omitted on the false right side. An accepted `Refl` oracle establishes the
false right side as `[3,6]`, against the accepted `[5,10]` left-side oracle.

### stdlib/linear-algebra/reject-vector-scale-dropped-factor

- spec: §9.2.
- promise class: durable invariant.
- given: the accepted `u2`, `x=2`, and `y=3`.
- expect: refuse the dropped-scale claim at its appended `Refl`.
- measured: the same-scope true control accepts; independent `Refl` oracles
  give `[5,10]` on the left and `[3,6]` on the false right side.
- claimed: a scaling law cannot omit the contribution of the second factor.
- the gap: the same-scope control isolates the value mismatch from an import,
  fixture, or unsupported proof-form failure.
- why: `[5,10]` is not `[3,6]`.

```ken
theorem seed_neg
    : Equal (Vec Nat two)
        (vector_scale two (add two three) u2)
        (vector_add two (vector_scale two two u2) u2) =
  Refl
```
