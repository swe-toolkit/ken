# Trace product commutation

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §19.
Spec child: `766bbd27bf5eaa2275bacdbe30928befd9c31437`.
CV seed base: `760b8eccd42a5e3161b00c42eb51b731d1be8976`.

The public law states that for `a : Vec (Vec Nat n) m` and
`b : Vec (Vec Nat m) n`, the traces of `a b` and `b a` are equal,
including rectangular and zero-dimensional factors. It does not state
that the products are equal. This seed pairs an open law application
with independent closed trace and product-value claims, then refuses the
false product-equality claim in the same `Refl` form as the accepted
product-value claims.

Architect's AC-0 at `evt_643c3q2targa0` checked corresponding open,
closed and isolated refusal scratch clients on base `760b8eccd`. That is not
execution of this Markdown seed or a conformance-runner result. The
trusted-identity set was not measured; the package test owns that gate.
CV has not run a Ken check, package test or official conformance runner.
No CI, publication, landing or landed-trust result is claimed.

## Open public-law client

```ken
import Data.Vector.Vector (Vec)

import Algorithm.LinearAlgebra.NatMatrix (matrix_product, trace, trace_product_comm)

theorem generic_trace_product_comm
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (b : Vec (Vec Nat m) n)
    : Equal Nat (trace m (matrix_product m n m a b)) (trace n (matrix_product n m n b a)) =
  trace_product_comm m n a b
```

### `stdlib/linear-algebra/trace-product-comm-open`

- spec: §19, the premise-free public law for exchanging product factors
  under a trace.
- promise class: durable invariant.
- given: arbitrary dimensions and matrices at the displayed indices.
- expect: the client accepts by selective import and direct law
  application.
- claimed: the public theorem is usable at open arguments.
- gap: this application is not a check of the generic proof body or of
  the trusted-identity set.

## Accepted closed client

The rectangular pair has different product dimensions: `a b` is 2 × 2
and `b a` is 3 × 3. Both traces are 9. The square products are unequal
matrices, but both traces are 3.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)

import Algorithm.LinearAlgebra.NatMatrix (matrix_product, trace, trace_product_comm)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

const four : Nat = Suc three

const five : Nat = Suc four

const nine : Nat = Suc (Suc (Suc (Suc five)))

fn vec2 (p : Nat) (q : Nat) : Vec Nat two =
  VCons Nat one p (VCons Nat Zero q (VNil Nat))

fn vec3 (p : Nat) (q : Nat) (r : Nat) : Vec Nat three =
  VCons Nat two p (VCons Nat one q (VCons Nat Zero r (VNil Nat)))

const a : Vec (Vec Nat three) two =
  VCons
    (Vec Nat three)
    one
    (vec3 one two Zero)
    (VCons (Vec Nat three) Zero (vec3 Zero one three) (VNil (Vec Nat three)))

const b : Vec (Vec Nat two) three =
  VCons
    (Vec Nat two)
    two
    (vec2 one Zero)
    (VCons
      (Vec Nat two)
      one
      (vec2 two one)
      (VCons (Vec Nat two) Zero (vec2 Zero one) (VNil (Vec Nat two))))

const c : Vec (Vec Nat two) two =
  VCons
    (Vec Nat two)
    one
    (vec2 one one)
    (VCons (Vec Nat two) Zero (vec2 Zero one) (VNil (Vec Nat two)))

const d : Vec (Vec Nat two) two =
  VCons
    (Vec Nat two)
    one
    (vec2 one Zero)
    (VCons (Vec Nat two) Zero (vec2 one one) (VNil (Vec Nat two)))

theorem rect_left_trace
    : Equal Nat (trace two (matrix_product two three two a b)) nine = Proved

theorem rect_right_trace
    : Equal Nat (trace three (matrix_product three two three b a)) nine = Proved

theorem rect_left_product
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two three two a b)
        (VCons
          (Vec Nat two)
          one
          (vec2 five two)
          (VCons (Vec Nat two) Zero (vec2 two four) (VNil (Vec Nat two)))) =
  Refl

theorem rect_law
    : Equal Nat
        (trace two (matrix_product two three two a b))
        (trace three (matrix_product three two three b a)) =
  trace_product_comm two three a b

theorem square_left_trace
    : Equal Nat (trace two (matrix_product two two two c d)) three = Proved

theorem square_right_trace
    : Equal Nat (trace two (matrix_product two two two d c)) three = Proved

theorem square_law
    : Equal Nat
        (trace two (matrix_product two two two c d))
        (trace two (matrix_product two two two d c)) =
  trace_product_comm two two c d

theorem square_left_product
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two two two c d)
        (VCons
          (Vec Nat two)
          one
          (vec2 two one)
          (VCons (Vec Nat two) Zero (vec2 one one) (VNil (Vec Nat two)))) =
  Refl

theorem square_right_product
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two two two d c)
        (VCons
          (Vec Nat two)
          one
          (vec2 one one)
          (VCons (Vec Nat two) Zero (vec2 one two) (VNil (Vec Nat two)))) =
  Refl
```

### `stdlib/linear-algebra/trace-product-comm-closed`

- spec: §19, the rectangular and square trace-product cases.
- promise class: normative compatibility vector.
- given: the fixed rectangular `a`, `b` and square `c`, `d` above.
- expect: both rectangular traces are 9; `a b` has the displayed 2 × 2
  value; both square traces are 3; the two trace-law applications accept;
  and `c d` and `d c` have their displayed, unequal values.
- claimed: the trace law applies to both rectangular and square factors;
  matrix values and trace values are distinct observations.
- gap: these finite cases do not prove the generic law.
- why: the two shapes exercise distinct trace dimensions, while the
  square values rule out reading the trace law as product commutativity.

## Refuse product commutativity

Run this block in a fresh environment. It contains the complete accepted
prefix above and appends only `square_products_commute`. The accepted
product-value claims are positive controls in the same matrix-valued
`Equal` and `Refl` form. The final claim is false for the fixed `c` and
`d`; require refusal at its own `Refl` token.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)

import Algorithm.LinearAlgebra.NatMatrix (matrix_product, trace, trace_product_comm)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

const four : Nat = Suc three

const five : Nat = Suc four

const nine : Nat = Suc (Suc (Suc (Suc five)))

fn vec2 (p : Nat) (q : Nat) : Vec Nat two =
  VCons Nat one p (VCons Nat Zero q (VNil Nat))

fn vec3 (p : Nat) (q : Nat) (r : Nat) : Vec Nat three =
  VCons Nat two p (VCons Nat one q (VCons Nat Zero r (VNil Nat)))

const a : Vec (Vec Nat three) two =
  VCons
    (Vec Nat three)
    one
    (vec3 one two Zero)
    (VCons (Vec Nat three) Zero (vec3 Zero one three) (VNil (Vec Nat three)))

const b : Vec (Vec Nat two) three =
  VCons
    (Vec Nat two)
    two
    (vec2 one Zero)
    (VCons
      (Vec Nat two)
      one
      (vec2 two one)
      (VCons (Vec Nat two) Zero (vec2 Zero one) (VNil (Vec Nat two))))

const c : Vec (Vec Nat two) two =
  VCons
    (Vec Nat two)
    one
    (vec2 one one)
    (VCons (Vec Nat two) Zero (vec2 Zero one) (VNil (Vec Nat two)))

const d : Vec (Vec Nat two) two =
  VCons
    (Vec Nat two)
    one
    (vec2 one Zero)
    (VCons (Vec Nat two) Zero (vec2 one one) (VNil (Vec Nat two)))

theorem rect_left_trace
    : Equal Nat (trace two (matrix_product two three two a b)) nine = Proved

theorem rect_right_trace
    : Equal Nat (trace three (matrix_product three two three b a)) nine = Proved

theorem rect_left_product
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two three two a b)
        (VCons
          (Vec Nat two)
          one
          (vec2 five two)
          (VCons (Vec Nat two) Zero (vec2 two four) (VNil (Vec Nat two)))) =
  Refl

theorem rect_law
    : Equal Nat
        (trace two (matrix_product two three two a b))
        (trace three (matrix_product three two three b a)) =
  trace_product_comm two three a b

theorem square_left_trace
    : Equal Nat (trace two (matrix_product two two two c d)) three = Proved

theorem square_right_trace
    : Equal Nat (trace two (matrix_product two two two d c)) three = Proved

theorem square_law
    : Equal Nat
        (trace two (matrix_product two two two c d))
        (trace two (matrix_product two two two d c)) =
  trace_product_comm two two c d

theorem square_left_product
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two two two c d)
        (VCons
          (Vec Nat two)
          one
          (vec2 two one)
          (VCons (Vec Nat two) Zero (vec2 one one) (VNil (Vec Nat two)))) =
  Refl

theorem square_right_product
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two two two d c)
        (VCons
          (Vec Nat two)
          one
          (vec2 one one)
          (VCons (Vec Nat two) Zero (vec2 one two) (VNil (Vec Nat two)))) =
  Refl

theorem square_products_commute
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two two two c d)
        (matrix_product two two two d c) =
  Refl
```

### `stdlib/linear-algebra/reject-trace-product-comm-matrices`

- spec: §19, trace equality does not imply matrix-product equality.
- promise class: normative compatibility vector.
- given: the fixed square matrices `c`, `d` above.
- expect: the accepted prefix passes; only `square_products_commute`
  refuses, with `ElabError::TypeMismatch` (E0038), reason
  ``Refl expects an `Eq`-shaped goal``, and a span exactly on the final
  theorem's `Refl` token, inside that theorem.
- claimed: `matrix_product two two two c d` is not equal to
  `matrix_product two two two d c` for this input.
- controls: the preceding `square_left_product` and
  `square_right_product` claims accept in the same matrix-valued
  `Equal`/`Refl` form; `square_law` accepts for the same factors.
- why: the two products are `[[2,1],[1,1]]` and `[[1,1],[1,2]]`;
  both traces are 3, but the matrices differ.
- gap: the fixed counterexample delimits the law and does not prove its
  generic body.
