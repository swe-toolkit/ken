# Trace of square natural matrices

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §18.
Spec child: `f3bcd6b46010db704f2213ff245b04abf7ed0260`.

The accepted fixture `a` is `[[1,2,3],[4,5,6],[7,8,9]]`. Its diagonal
is `[1,5,9]`, so its trace is 15; its first row instead sums to 6.
Scaling `a` by 3 gives trace 45, while adding 3 to its trace gives 18.
The second fixture `b` has trace 9. The cases apply each public law at
open and closed arguments, and isolate the two false alternatives after
fresh copies of the accepted client.

Architect's AC-0 D0 checked these client forms in detached scratch on
`77a4e9907d78d48389424e4904804cfd0871ceff`. This seed is authored on
`820722005a96376834415414448a9d2520327443`; D0 does not measure that
later base or execute this Markdown seed. CV did not run Ken or the
package tests. No official conformance execution, CI, publication,
landing, or landed-trust result is claimed.

## Open public-law clients

The arbitrary dimension and matrices remain open while each client
applies one of the four public laws through selective imports.

```ken
import Data.Vector.Vector (Vec)

import Data.Numeric.Nat.Arithmetic (add, mul)

import Algorithm.LinearAlgebra.NatMatrix (identity, matrix_add, matrix_scale, transpose, trace, trace_add, trace_scale, trace_transpose, trace_identity)

theorem generic_trace_add
      (n : Nat) (a : Vec (Vec Nat n) n) (b : Vec (Vec Nat n) n)
    : Equal Nat (trace n (matrix_add n n a b)) (add (trace n a) (trace n b)) =
  trace_add n a b

theorem generic_trace_scale
      (n : Nat) (x : Nat) (a : Vec (Vec Nat n) n)
    : Equal Nat (trace n (matrix_scale n n x a)) (mul x (trace n a)) =
  trace_scale n x a

theorem generic_trace_transpose
      (n : Nat) (a : Vec (Vec Nat n) n)
    : Equal Nat (trace n (transpose n n a)) (trace n a) =
  trace_transpose n a

theorem generic_trace_identity (n : Nat) : Equal Nat (trace n (identity n)) n = trace_identity n
```

### `stdlib/linear-algebra/trace-open-laws`

- spec: §18, the four public trace laws.
- promise class: durable invariant.
- given: arbitrary dimension, square matrices, and natural scale.
- expect: each law accepts through a direct public import and
  application.
- measured: the D0 generic client applied all four laws with open
  arguments on its scratch base.
- claimed: the public laws are reusable at arbitrary well-typed inputs.
- gap: theorem application is not a separate check of each generic
  proof body.
- why: the open arguments prevent these applications from being
  established solely by reducing the closed fixture below.

## Accepted closed client

The accepted block checks the trace values, the first-row sum, the
scaled trace, and each law at closed arguments. The value controls are
separate from the public-law applications.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)

import Data.Numeric.Nat.Arithmetic (add, mul)

import Algorithm.LinearAlgebra.NatMatrix (identity, matrix_add, matrix_scale, transpose, trace, trace_add, trace_scale, trace_transpose, trace_identity)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

const four : Nat = Suc three

const five : Nat = Suc four

const six : Nat = Suc five

const seven : Nat = Suc six

const eight : Nat = Suc seven

const nine : Nat = Suc eight

fn vec3 (p : Nat) (q : Nat) (r : Nat) : Vec Nat three =
  VCons Nat two p (VCons Nat one q (VCons Nat Zero r (VNil Nat)))

fn mat3 (r0 : Vec Nat three) (r1 : Vec Nat three) (r2 : Vec Nat three) : Vec (Vec Nat three) three =
  VCons (Vec Nat three) two r0 (VCons (Vec Nat three) one r1 (VCons (Vec Nat three) Zero r2 (VNil (Vec Nat three))))

const a : Vec (Vec Nat three) three = mat3 (vec3 one two three) (vec3 four five six) (vec3 seven eight nine)

const b : Vec (Vec Nat three) three = mat3 (vec3 two Zero one) (vec3 one three Zero) (vec3 Zero one four)

theorem trace_a_value : Equal Nat (trace three a) (add six nine) = Proved

theorem trace_b_value : Equal Nat (trace three b) nine = Proved

theorem first_row_sum_value : Equal Nat (add (add one two) three) six = Proved

theorem trace_add_closed
    : Equal Nat (trace three (matrix_add three three a b)) (add (trace three a) (trace three b)) =
  trace_add three a b

theorem trace_scale_closed
    : Equal Nat (trace three (matrix_scale three three three a)) (mul three (trace three a)) =
  trace_scale three three a

theorem trace_transpose_closed : Equal Nat (trace three (transpose three three a)) (trace three a) =
  trace_transpose three a

theorem trace_identity_closed : Equal Nat (trace three (identity three)) three = trace_identity three

theorem trace_scale_value : Equal Nat (trace three (matrix_scale three three three a)) (mul three (add six nine)) = Proved
```

### `stdlib/linear-algebra/trace-closed-laws`

- spec: §18, the four public trace laws and the closed discriminator.
- promise class: durable invariant.
- given: the two 3 × 3 matrices `a` and `b`, with scale `three`.
- expect: the entire block accepts; `trace a` is 15, `trace b` is 9,
  the first-row sum is 6, and the scaled trace is 45.
- measured: D0 checked the accepted client at closed arguments on
  scratch base `77a4e9907`.
- claimed: the values and law applications hold for this instance.
- gap: this finite instance does not establish the four generic laws.
- why: `a` has distinct diagonal and off-diagonal entries, and its
  first-row sum differs from its diagonal sum.

## Refuse trace equal to the first-row sum

Run this block in a fresh environment. It contains the complete accepted
prefix and appends only `trace_first_row`. Require refusal at that
final theorem, after `trace_a_value` and `first_row_sum_value` accept.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)

import Data.Numeric.Nat.Arithmetic (add, mul)

import Algorithm.LinearAlgebra.NatMatrix (identity, matrix_add, matrix_scale, transpose, trace, trace_add, trace_scale, trace_transpose, trace_identity)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

const four : Nat = Suc three

const five : Nat = Suc four

const six : Nat = Suc five

const seven : Nat = Suc six

const eight : Nat = Suc seven

const nine : Nat = Suc eight

fn vec3 (p : Nat) (q : Nat) (r : Nat) : Vec Nat three =
  VCons Nat two p (VCons Nat one q (VCons Nat Zero r (VNil Nat)))

fn mat3 (r0 : Vec Nat three) (r1 : Vec Nat three) (r2 : Vec Nat three) : Vec (Vec Nat three) three =
  VCons (Vec Nat three) two r0 (VCons (Vec Nat three) one r1 (VCons (Vec Nat three) Zero r2 (VNil (Vec Nat three))))

const a : Vec (Vec Nat three) three = mat3 (vec3 one two three) (vec3 four five six) (vec3 seven eight nine)

const b : Vec (Vec Nat three) three = mat3 (vec3 two Zero one) (vec3 one three Zero) (vec3 Zero one four)

theorem trace_a_value : Equal Nat (trace three a) (add six nine) = Proved

theorem trace_b_value : Equal Nat (trace three b) nine = Proved

theorem first_row_sum_value : Equal Nat (add (add one two) three) six = Proved

theorem trace_add_closed
    : Equal Nat (trace three (matrix_add three three a b)) (add (trace three a) (trace three b)) =
  trace_add three a b

theorem trace_scale_closed
    : Equal Nat (trace three (matrix_scale three three three a)) (mul three (trace three a)) =
  trace_scale three three a

theorem trace_transpose_closed : Equal Nat (trace three (transpose three three a)) (trace three a) =
  trace_transpose three a

theorem trace_identity_closed : Equal Nat (trace three (identity three)) three = trace_identity three

theorem trace_scale_value : Equal Nat (trace three (matrix_scale three three three a)) (mul three (add six nine)) = Proved

theorem trace_first_row : Equal Nat (trace three a) (add (add one two) three) = Proved
```

### `stdlib/linear-algebra/reject-trace-first-row`

- spec: §18, diagonal sum rather than first-row sum.
- promise class: durable invariant.
- given: `a` with diagonal values `1, 5, 9` and first row
  `1, 2, 3`.
- expect: the accepted prefix passes; `trace_first_row` refuses at its
  own `Proved` with `KernelRejected(TypeMismatch)`.
- measured: D0 refused the same isolated row, spanning its appended
  theorem from `theorem` through `Proved`.
- claimed: trace is not the sum of the first row.
- gap: one matrix does not characterize the trace operation.
- controls: `trace_a_value` and `first_row_sum_value` accept in the
  same prefix with the same `Proved` form.
- why: the diagonal sum is 15; the first-row sum is 6.

## Refuse scaling by addition

Run this block in a fresh environment. It contains the complete accepted
prefix and appends only `trace_scale_as_add`. Require refusal at that
final theorem, after `trace_scale_value` accepts.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)

import Data.Numeric.Nat.Arithmetic (add, mul)

import Algorithm.LinearAlgebra.NatMatrix (identity, matrix_add, matrix_scale, transpose, trace, trace_add, trace_scale, trace_transpose, trace_identity)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

const four : Nat = Suc three

const five : Nat = Suc four

const six : Nat = Suc five

const seven : Nat = Suc six

const eight : Nat = Suc seven

const nine : Nat = Suc eight

fn vec3 (p : Nat) (q : Nat) (r : Nat) : Vec Nat three =
  VCons Nat two p (VCons Nat one q (VCons Nat Zero r (VNil Nat)))

fn mat3 (r0 : Vec Nat three) (r1 : Vec Nat three) (r2 : Vec Nat three) : Vec (Vec Nat three) three =
  VCons (Vec Nat three) two r0 (VCons (Vec Nat three) one r1 (VCons (Vec Nat three) Zero r2 (VNil (Vec Nat three))))

const a : Vec (Vec Nat three) three = mat3 (vec3 one two three) (vec3 four five six) (vec3 seven eight nine)

const b : Vec (Vec Nat three) three = mat3 (vec3 two Zero one) (vec3 one three Zero) (vec3 Zero one four)

theorem trace_a_value : Equal Nat (trace three a) (add six nine) = Proved

theorem trace_b_value : Equal Nat (trace three b) nine = Proved

theorem first_row_sum_value : Equal Nat (add (add one two) three) six = Proved

theorem trace_add_closed
    : Equal Nat (trace three (matrix_add three three a b)) (add (trace three a) (trace three b)) =
  trace_add three a b

theorem trace_scale_closed
    : Equal Nat (trace three (matrix_scale three three three a)) (mul three (trace three a)) =
  trace_scale three three a

theorem trace_transpose_closed : Equal Nat (trace three (transpose three three a)) (trace three a) =
  trace_transpose three a

theorem trace_identity_closed : Equal Nat (trace three (identity three)) three = trace_identity three

theorem trace_scale_value : Equal Nat (trace three (matrix_scale three three three a)) (mul three (add six nine)) = Proved

theorem trace_scale_as_add : Equal Nat (trace three (matrix_scale three three three a)) (add three (trace three a)) = Proved
```

### `stdlib/linear-algebra/reject-trace-scale-as-add`

- spec: §18, multiplication by the scale rather than addition of it.
- promise class: durable invariant.
- given: `a` with trace 15 and factor `three`.
- expect: the accepted prefix passes; `trace_scale_as_add` refuses at
  its own `Proved` with `KernelRejected(TypeMismatch)`.
- measured: D0 refused the same isolated row, spanning its appended
  theorem from `theorem` through `Proved`.
- claimed: scaling multiplies the trace; it does not add the scale.
- gap: this one factor and matrix do not characterize every scaling
  law.
- controls: `trace_scale_value` checks the scaled trace by the same
  `Proved` form; `trace_a_value` checks the original trace.
- why: scaling gives 45, while `3 + trace a` is 18.

## Evidence boundary

The Architect's AC-0 scratch D0 at `77a4e9907d78d48389424e4904804cfd0871ceff`
checked the generic client, the accepted closed client, and each
isolated refusal; it also rejected the `trace_identity` mutation. That
D0 did not run this authored Markdown seed on base
`820722005a96376834415414448a9d2520327443`. Its trust and public-name
measurements are scratch evidence, not a landed trust result. No
official conformance run, package test by CV, CI, publication, or
landing is claimed.
