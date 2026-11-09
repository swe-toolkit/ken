# Dot linearity in the second argument

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §15.
AC-1 child SHA: `20ed02d72ad8e3fbf8caf722eb0ca9cf942c86e3`.
Frame: `docs/program/issues/SPEC-LINEAR-ALGEBRA-DOT-RIGHT-LINEARITY.md`.

Status: authored; not executed. The seed applies the two public checked laws
to closed vectors. Those instances do not establish the generic proof bodies.

## Accepted client

Use `xs = [1,2]`, `ys = [2,1]`, `zs = [3,1]`, and `factor = 2`.
The two sides of right distributivity reduce to `9`; the two sides of
right scaling reduce to `8`. Each side has its own `Proved` check before
the public law is applied.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)
import Data.Numeric.Nat.Arithmetic (add, mul)
import Algorithm.LinearAlgebra.NatMatrix (dot, vector_add, vector_scale, dot_distrib_right, dot_scale_right)

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

fn vec2 (x : Nat) (y : Nat) : Vec Nat two =
  VCons Nat one x (VCons Nat Zero y (VNil Nat))

const xs : Vec Nat two = vec2 one two
const ys : Vec Nat two = vec2 two one
const zs : Vec Nat two = vec2 three one
const alternate_z : Vec Nat two = vec2 one one

theorem dot_distrib_right_left_value :
  Equal Nat (dot two xs (vector_add two ys zs)) nine = Proved

theorem dot_distrib_right_right_value :
  Equal Nat (add (dot two xs ys) (dot two xs zs)) nine = Proved

theorem dot_distrib_right_claim_proved :
  Equal Nat
    (dot two xs (vector_add two ys zs))
    (add (dot two xs ys) (dot two xs zs)) = Proved

theorem dot_distrib_right_closed :
  Equal Nat
    (dot two xs (vector_add two ys zs))
    (add (dot two xs ys) (dot two xs zs)) =
      dot_distrib_right two xs ys zs

theorem dot_scale_right_left_value :
  Equal Nat (dot two xs (vector_scale two two ys)) eight = Proved

theorem dot_scale_right_right_value :
  Equal Nat (mul two (dot two xs ys)) eight = Proved

theorem dot_scale_right_closed :
  Equal Nat
    (dot two xs (vector_scale two two ys))
    (mul two (dot two xs ys)) =
      dot_scale_right two two xs ys

theorem alternate_right_sum_value :
  Equal Nat (add (dot two xs ys) (dot two xs alternate_z)) seven = Proved
```

### stdlib/linear-algebra/dot-right-distrib-closed

- spec: §15.
- promise class: durable invariant.
- given: `xs = [1,2]`, `ys = [2,1]`, and `zs = [3,1]` at length two.
- expect: each side independently reduces to `9` by `Proved`, and
  `dot_distrib_right` applies to the same closed operands.
- measured: the direct left and right equations plus the public theorem
  application.
- why: all vector coordinates contribute, and the expected value is
  independent of `dot_distrib_right`.
- gap: this closed instance does not establish the generic proof body.

### stdlib/linear-algebra/dot-right-scale-closed

- spec: §15.
- promise class: durable invariant.
- given: `xs = [1,2]`, `ys = [2,1]`, and `factor = 2` at length two.
- expect: each side independently reduces to `8` by `Proved`, and
  `dot_scale_right` applies to the same closed operands.
- measured: the direct left and right equations plus the public theorem
  application.
- why: the non-unit factor changes both entries, and the expected value
  is independent of `dot_scale_right`.
- gap: this closed instance does not establish the generic proof body.

## Isolated refusal

The refusal starts from a fresh copy of the complete accepted client above and
appends only the false theorem below. It keeps the left side and first right
summand fixed, replacing only `zs` in the right-hand sum with
`alternate_z`. The unchanged left side is `9`; the altered right side is `7`,
independently checked by `alternate_right_sum_value`. Expect rejection of the
false `Proved` claim; no diagnostic code is specified.

### stdlib/linear-algebra/reject-dot-right-distrib-alternate-summand

- spec: §15.
- promise class: durable invariant.
- given: the accepted length-two vectors above.
- expect: reject the false equality with the original `zs` on the left and
  `alternate_z` on the right.
- control: `dot_distrib_right_claim_proved` proves the same equation shape
  with `zs` on both sides; the closed value claims distinguish `9` from `7`.
- why: only the second summand on the right changes, so unrelated import,
  shape, or type failures cannot explain the refusal.
- gap: this refusal checks the false closed claim, not the generic theorem's
  proof body.

```ken
theorem reject_dot_right_distrib_alternate_summand :
  Equal Nat
    (dot two xs (vector_add two ys zs))
    (add (dot two xs ys) (dot two xs alternate_z)) = Proved
```

## Evidence boundary

The accepted and refused blocks encode the §15 contract at closed inputs. The
AC-0 D0 scratch measurements are not execution of this seed or a candidate.
No official conformance-runner, CI, publication, or landing is claimed.
