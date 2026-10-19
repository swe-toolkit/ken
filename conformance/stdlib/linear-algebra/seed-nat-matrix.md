# Dimension-safe natural matrix operations

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §§1–3.

These sixteen cases are **authored behavioral oracles**, each with
promise class **durable invariant**. The catalog publishes the §1
operations and five checked §1 laws; its two generic §2 laws are also
landed. Section 3 specifies `matrix_vector_product`, `transpose_product`,
and public `dot_scale`. The two generic §3 laws have D0-checked bodies,
but their catalog exports and the public `dot_scale` export are not yet
delivered. The three §3 positive cases below are closed instances using
the already-published operations; they do not establish either generic
law or the public helper export. Each refusal uses a separate checker
input, and the unreversed-order refusal includes a same-input positive
control for the required reversed order.

The positive block was extracted to scratch `SeedPositive.ken`
(SHA256
`897ade308686f22fd4cfc386858f7a6471d7266c8187b90e3d4d359ea4bae603`);
its sixteen closed assertions checked. The temporary roots-loaded
`cat_linear_algebra_ac2_scratch` test target ran four checks through
`scripts/ken-cargo`; all four passed. The positive block and reversed-order
control passed; the
wrong-inner input returned `KernelRejected TypeMismatch`;
the unreversed equation returned `TypeMismatch` with reason
`Refl expects an Eq-shaped goal`. The scratch sources were
`SeedUnreversedControl.ken` (SHA256
`a18762f5da1ce36a22dfaaa02bf44d79519edd76e6023cfa576bfe57af90541b`),
`SeedBadInner.ken` (SHA256
`d39dc61dec0592edb51f2e80d3bd869bbe2edcf6dd7bb5ba92f2a570c94623f6`),
and `SeedUnreversed.ken` (SHA256
`9e92d693613f8affc8b8141741e08dbf720db5c3840e111a7c7dd2236fddeeac`).

A direct single-file `ken check` cannot resolve these clients' package
imports (`E0012 UnboundName Data.Vector.Vector`); the temporary test used
`ElabEnv::elaborate_module_from_roots` followed by `elaborate_file`.
This scratch check does not establish shipped seed-harness execution,
generic §3 law delivery, an existing catalog test-suite result, CI,
publication, or landing.

## Shared fixtures and closed oracles

The left operand has two rows and three columns; the right operand
has three rows and two columns. Their entries are small but unequal
so that an omitted middle factor, an accidental row/column swap or
a truncating zip changes an observable result. The expected product
is computed directly from four dot products of three terms each,
not read back from `matrix_product`. These are indexed `Vec`
constructors, not a runtime shape tag or an assumed rectangular-list
invariant.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_add, vector_scale, dot, matrix_vector,
   transpose, matrix_product, identity)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four
const six : Nat = Suc five

fn v2 (x : Nat) (y : Nat) : Vec Nat two =
  VCons Nat one x (VCons Nat Zero y (VNil Nat))
fn v3 (x : Nat) (y : Nat) (z : Nat) : Vec Nat three =
  VCons Nat two x (VCons Nat one y (VCons Nat Zero z (VNil Nat)))

const a23 : Vec (Vec Nat three) two =
  VCons (Vec Nat three) one
    (v3 one two Zero)
    (VCons (Vec Nat three) Zero
      (v3 Zero one three) (VNil (Vec Nat three)))
const b32 : Vec (Vec Nat two) three =
  VCons (Vec Nat two) two (v2 two one)
    (VCons (Vec Nat two) one (v2 Zero two)
      (VCons (Vec Nat two) Zero (v2 one Zero)
        (VNil (Vec Nat two))))
const expected22 : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one (v2 two five)
    (VCons (Vec Nat two) Zero (v2 three two)
      (VNil (Vec Nat two)))
const expected32 : Vec (Vec Nat two) three =
  VCons (Vec Nat two) two (v2 one Zero)
    (VCons (Vec Nat two) one (v2 two one)
      (VCons (Vec Nat two) Zero (v2 Zero three)
        (VNil (Vec Nat two))))
const c22 : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one (v2 one two)
    (VCons (Vec Nat two) Zero (v2 three Zero) (VNil (Vec Nat two)))

theorem dot_oracle : Equal Nat
  (dot three (v3 one two Zero) (v3 three one two)) five = Proved

theorem identity_two_oracle : Equal (Vec (Vec Nat two) two) (identity two)
  (VCons (Vec Nat two) one (v2 one Zero)
    (VCons (Vec Nat two) Zero (v2 Zero one) (VNil (Vec Nat two)))) = Refl

theorem vector_add_oracle : Equal (Vec Nat three)
  (vector_add three (v3 one two Zero) (v3 two Zero one))
  (v3 three two one) = Refl

theorem vector_scale_oracle : Equal (Vec Nat three)
  (vector_scale three two (v3 one two Zero))
  (v3 two four Zero) = Refl

theorem vector_scale_three_oracle : Equal (Vec Nat three)
  (vector_scale three three (v3 one two Zero))
  (v3 three six Zero) = Refl

theorem vector_scale_empty_oracle : Equal (Vec Nat Zero)
  (vector_scale Zero three (VNil Nat)) (VNil Nat) = Proved

theorem matrix_vector_oracle : Equal (Vec Nat two)
  (matrix_vector two three a23 (v3 two two one))
  (v2 six five) = Refl

theorem product_oracle : Equal (Vec (Vec Nat two) two)
  (matrix_product two three two a23 b32) expected22 = Refl

theorem right_identity_oracle : Equal (Vec (Vec Nat three) two)
  (matrix_product two three three a23 (identity three)) a23 = Refl

theorem left_identity_oracle : Equal (Vec (Vec Nat three) two)
  (matrix_product two two three (identity two) a23) a23 = Refl

theorem product_assoc_oracle : Equal (Vec (Vec Nat two) two)
  (matrix_product two two two (matrix_product two three two a23 b32) c22)
  (matrix_product two three two a23 (matrix_product three two two b32 c22))
  = Refl

theorem transpose_intermediate_oracle : Equal (Vec (Vec Nat two) three)
  (transpose two three a23) expected32 = Refl

theorem transpose_round_trip_oracle : Equal (Vec (Vec Nat three) two)
  (transpose three two (transpose two three a23)) a23 = Refl

theorem matrix_vector_product_oracle : Equal (Vec Nat two)
  (matrix_vector two two
    (matrix_product two three two a23 b32)
    (v2 Zero one))
  (matrix_vector two three a23
    (matrix_vector three two b32 (v2 Zero one))) = Refl

theorem transpose_product_oracle : Equal (Vec (Vec Nat two) two)
  (transpose two two (matrix_product two three two a23 b32))
  (matrix_product two three two
    (transpose three two b32)
    (transpose two three a23)) = Refl

theorem matrix_vector_product_identity_control : Equal (Vec Nat two)
  (matrix_vector two three
    (matrix_product two three three a23 (identity three))
    (v3 Zero one Zero))
  (matrix_vector two three a23
    (matrix_vector three three (identity three) (v3 Zero one Zero))) = Refl
```

The nonempty closed vector equalities check using `Refl`. The
zero-length scaling case uses `Proved`, as the catalog's
`map_vnil` does; `Equal Nat` in the direct dot oracle also takes
`Proved`. Neither asserts a generic Vec extensionality law. The
closed §2 rows illustrate, but do not themselves prove, universal
left identity or associativity; those generic checked theorems are
separately landed in the catalog.

## A direct dot product pins the public operation

### stdlib/linear-algebra/nat-dot-three

- spec: `spec/50-stdlib/65-linear-algebra.md` §1.2.
- promise class: durable invariant.
- given: the aligned vectors `[1, 2, 0]` and `[3, 1, 2]`.
- expect: `dot three (v3 one two Zero) (v3 three one two)` is
  exactly five: `1·3 + 2·1 + 0·2 = 5`. The closed `Equal Nat`
  proposition checks with `Proved`.
- measured: all three positions, including the middle nonzero
  contribution and a trailing zero; a wrong result of six rejects.
- claimed: the public `dot` sums aligned component products in order.
- the gap: a closed calculation cannot discharge either generic dot law.
- why: a constant-zero dot or a matrix-vector implementation that
  bypasses the public dot cannot pass this direct value check.

## An explicit identity matrix pins the diagonal

### stdlib/linear-algebra/nat-identity-two

- spec: `spec/50-stdlib/65-linear-algebra.md` §1.2.
- promise class: durable invariant.
- given: dimension two and an independently constructed matrix
  whose rows are `[1, 0]` and `[0, 1]`.
- expect: `identity two` equals that exact `Vec (Vec Nat two) two`;
  its closed equality checks with `Refl`.
- measured: two diagonal ones and both off-diagonal zeros, with
  row and column order independently fixed.
- claimed: the public identity constructor places one exactly on
  the diagonal and zero off it.
- the gap: this finite fixture is not a generic left or right
  identity theorem. The generic `matrix_identity_left` theorem is
  independently checked and landed in the catalog.
- why: diagonal placement is checked directly rather than inferred
  only from the product with one selected matrix.

## Componentwise vector addition retains alignment

### stdlib/linear-algebra/nat-vector-add-aligned

- spec: `spec/50-stdlib/65-linear-algebra.md` §1.2.
- promise class: durable invariant.
- given: `[1, 2, 0]` and `[2, 0, 1]`, both indexed by three.
- expect: `vector_add three (v3 one two Zero) (v3 two Zero one)`
  equals `[3, 2, 1]` as a `Vec Nat three`; the closed equality checks.
- measured: three distinct result components, including nontrivial
  addition and zeros in different positions.
- claimed: addition is componentwise, aligned and length-preserving.
- the gap: one closed vector pair is not a generic addition law.
- why: reversing, dropping or misaligning components changes the
  independently constructed expected result.

## Natural vector scaling handles nonzero and empty lengths

### stdlib/linear-algebra/nat-vector-scale

- spec: `spec/50-stdlib/65-linear-algebra.md` §1.2.
- promise class: durable invariant.
- given: factor two and `[1, 2, 0]`, plus the empty vector with
  factor three.
- expect: `vector_scale three two (v3 one two Zero)` equals
  `[2, 4, 0]` at length three; `vector_scale Zero three (VNil Nat)`
  equals `VNil Nat` at length zero. Both closed equalities check.
- measured: component multiplication by two, preservation of an
  input zero, and an empty indexed boundary.
- claimed: scaling multiplies every component without changing its
  length, and the zero-length result is empty.
- the gap: these two fixtures do not prove a generic scaling law;
  for a nonempty input, factor two alone cannot distinguish a body
  that always doubles from one that uses the caller's factor.
- why: multiplying only the head or fabricating a scalar entry for
  the empty vector changes a result; the factor-three row below
  separately checks that the factor is used.

## Scaling by three distinguishes the factor from doubling

### stdlib/linear-algebra/nat-vector-scale-factor-three

- spec: `spec/50-stdlib/65-linear-algebra.md` §1.2.
- promise class: durable invariant.
- given: factor three and the indexed input `[1, 2, 0]`.
- expect: `vector_scale three three (v3 one two Zero)` equals
  `[3, 6, 0]` at length three; the closed equality checks with `Refl`.
- measured: a second nonempty factor, with two nonzero input values
  and a zero retained in its original position.
- claimed: the output uses the supplied scalar factor, rather than
  a fixed factor of two or ignoring it.
- the gap: two finite factors do not establish a generic scaling law;
  a mathematically equivalent implementation is not distinguished.
- why: a body that doubles regardless of `factor` passes the factor-two
  fixture but cannot produce six from the second component here.

## Matrix-vector multiplication uses ordered row dots

### stdlib/linear-algebra/nat-matrix-vector-2x3

- spec: `spec/50-stdlib/65-linear-algebra.md` §1.2.
- promise class: durable invariant.
- given: `a23` and the three-component vector `[2, 2, 1]`.
- expect: `matrix_vector two three a23 (v3 two two one)` equals
  `[6, 5]` at length two: `1·2 + 2·2 + 0·1 = 6`,
  `0·2 + 1·2 + 3·1 = 5`. The closed equality checks.
- measured: two unequal row results, each depending on its
  positional factors, in original row order.
- claimed: every row contributes one dot product; the result length
  follows the number of rows, not the number of columns.
- the gap: a fixed 2×3 fixture cannot establish the generic
  matrix-vector law or the `vector_identity` theorem.
- why: transposing the input matrix, omitting a row or overlooking
  the last component changes a result or its type.

## A non-square product retains order and dimensions

### stdlib/linear-algebra/nat-2x3-times-3x2

- spec: `spec/50-stdlib/65-linear-algebra.md` §§1.1–1.2.
- promise class: durable invariant.
- given: `a23` and `b32`; their common index is three.
- expect: `matrix_product two three two a23 b32` has type
  `Vec (Vec Nat two) two` and equals `expected22`, namely rows
  `[2, 5]` and `[3, 2]`. A closed proof of the equality checks.
- measured: four cells with unequal off-diagonals, calculated from
  all three terms of each dot product in row and column order.
- claimed: the exact product joins equal inner dimensions and
  preserves the outer dimensions without truncating either input.
- the gap: a single pair of matrices does not prove the generic dot
  laws or §2's matrix-product associativity theorem. The latter's
  checked catalog proof is separate from this closed result.
- why: treating the rows of `b32` as its columns, dropping its last
  row, or confusing the two outer dimensions changes at least one
  expected cell.

## Right identity with an independently indexed left operand

### stdlib/linear-algebra/nat-right-identity-2x3

- spec: `spec/50-stdlib/65-linear-algebra.md` §§1.2–1.4.
- promise class: durable invariant.
- given: `a23` and `identity three`, the 3 × 3 natural diagonal
  matrix. The right input's dimension matches `a23`'s columns.
- expect: `matrix_product two three three a23 (identity three)`
  equals `a23`, including both rows and both zero positions. The
  returned type remains `Vec (Vec Nat three) two`.
- measured: a **right** identity product with two different rows
  and a non-square left operand.
- claimed: the public generic `matrix_identity_right` theorem has a
  concrete nondegenerate witness. This case does not test the left side.
- the gap: this closed result does not establish the generic
  `matrix_identity_right` theorem. The generic right- and left-identity
  theorems are separately checked and landed; the left case below is
  a separate closed witness.
- why: replacing the diagonal with all ones or losing the second
  row changes the result. A square-only oracle would not reveal a
  mistaken assumption that both outer dimensions must be equal.

## Left identity preserves a rectangular matrix

### stdlib/linear-algebra/nat-left-identity-2x3

- spec: `spec/50-stdlib/65-linear-algebra.md` §2.
- promise class: durable invariant.
- given: `identity two` on the left of the non-square `a23`, whose
  two rows each have three ordered entries.
- expect: `matrix_product two two three (identity two) a23` equals
  exactly `a23` as `Vec (Vec Nat three) two`; the closed equality
  checks with `Refl`.
- measured: a left unit matrix sized to the **row** dimension,
  distinct from the right-identity fixture's column-sized unit.
- claimed: the landed generic `matrix_identity_left` theorem
  preserves arbitrary well-typed input matrices, including those
  with unequal outer dimensions.
- the gap: one 2×3 value does not establish the generic theorem;
  its checked proof is separate from this closed witness.
- why: a wrong-side implementation that accepts only square
  operands or swaps row and column dimensions cannot satisfy this
  typed, non-square left-identity equality.

## Associativity composes three nontrivial matrices

### stdlib/linear-algebra/nat-product-assoc-2x3x2x2

- spec: `spec/50-stdlib/65-linear-algebra.md` §2.
- promise class: durable invariant.
- given: `a23` (2×3), `b32` (3×2), and independently constructed
  `c22` (2×2), with rows `[1, 2]` and `[3, 0]`.
- expect: `(a23 · b32) · c22` equals `a23 · (b32 · c22)` as a
  `Vec (Vec Nat two) two`; the two fully indexed expressions in
  `product_assoc_oracle` check with `Refl`.
- measured: both parenthesizations over two independent inner
  dimensions and non-identity, nonzero matrices; the pre-existing
  `product_oracle` independently pins `a23 · b32` to `expected22`.
- claimed: the generic §2 associativity law respects order and
  inner indices across all composable matrices.
- the gap: closed equality alone could pass for two identically wrong
  products. The independently pinned intermediate limits, but do
  not eliminate, that finite-fixture gap; the generic checked
  catalog theorem is distinct evidence.
- why: changing the inner right operand in one parenthesization
  rejects this equality, while a shape mismatch is refused by shared
  indices rather than cast to a common matrix type.

## Transpose round trip with an independent intermediate value

### stdlib/linear-algebra/nat-transpose-2x3-round-trip

- spec: `spec/50-stdlib/65-linear-algebra.md` §§1.2–1.3.
- promise class: durable invariant.
- given: `a23` and independently constructed `expected32` with
  rows `[1, 0]`, `[2, 1]`, `[0, 3]`.
- expect: `transpose two three a23` has type
  `Vec (Vec Nat two) three` and equals `expected32`; then
  `transpose three two (transpose two three a23)` equals `a23`.
  Both closed equality proofs check.
- measured: the intermediate orientation **and** the round trip;
  the expected intermediate is not produced by `transpose` itself.
- claimed: the specified transpose swaps the two indices and
  preserves ordered entries; its generic involution theorem still
  requires a checked body.
- the gap: an implementation that permutes entries through an
  involution could pass only the round trip; pinning `expected32`
  rules out that self-consistency escape on this fixture.
- why: a wrong but involutive entry permutation could pass the round
  trip alone, yet fail the independently stated intermediate rows.

## Matrix-vector compatibility composes a non-square product

### stdlib/linear-algebra/nat-matrix-vector-product-2x3-3x2

- spec: `spec/50-stdlib/65-linear-algebra.md` §3.
- promise class: durable invariant.
- given: `a23` (2×3), `b32` (3×2), and `x = [0, 1]`.
- expect: `matrix_vector_product_oracle` checks `(a23 · b32)x =
  a23(b32x)` as an `Equal (Vec Nat two)` proposition, with `Refl`.
- measured: both evaluation routes on non-square operands with
  distinct inner and vector dimensions.
- claimed: a closed instance of the generic `matrix_vector_product`
  law from §3.
- the gap: this finite example does not establish the generic law
  or its public catalog export.
- why: the independently shaped routes use the same 2×3 and 3×2
  factors, and the expected vector is computed from `[0, 1]`.

## Transpose-product compatibility reverses the factors

### stdlib/linear-algebra/nat-transpose-product-2x3-3x2

- spec: `spec/50-stdlib/65-linear-algebra.md` §3.
- promise class: durable invariant.
- given: the same non-square `a23` and `b32`.
- expect: `transpose_product_oracle` checks `(a23 · b32)ᵀ =
  b32ᵀ · a23ᵀ` as a `Vec (Vec Nat two) two` equality with `Refl`.
- measured: transpose of the 2×3 by 3×2 product against the
  transposed factors in the required reverse order.
- claimed: a closed instance of the generic `transpose_product`
  law from §3.
- the gap: a finite instance neither proves the universal law nor
  establishes its catalog export.
- why: the product and transpose cases above independently pin the
  input product and row/column orientation; this case composes them.

## The matrix-vector law has an identity control

### stdlib/linear-algebra/nat-matrix-vector-product-identity-3

- spec: `spec/50-stdlib/65-linear-algebra.md` §3.
- promise class: durable invariant.
- given: `a23`, `identity three` as its right factor, and
  `x = [0, 1, 0]`.
- expect: `matrix_vector_product_identity_control` checks the law
  with `identity three` on the right as an `Equal (Vec Nat two)`
  proposition, with `Refl`.
- measured: the compatibility relation with the identity matrix as
  the right factor and the same length-three input vector.
- claimed: a positive identity control for the matrix-vector
  product law, not a generic proof of it.
- the gap: this identity instance does not establish the law for
  arbitrary matrices or the catalog export.
- why: the right identity leaves the vector unchanged before the
  non-square left matrix acts on it.

## Transpose-product order is not interchangeable

### stdlib/linear-algebra/nat-transpose-product-unreversed-2x2

- spec: `spec/50-stdlib/65-linear-algebra.md` §3.
- promise class: durable invariant.
- given: `s1 = [[1, 2], [3, 0]]` and `s2 = [[0, 1], [2, 3]]`.
- expect: `transpose_product_reversed_square_control` checks with
  `Refl`; the unreversed equality is refused when asserted with
  `Refl`, as `TypeMismatch`.
- measured: the same square operands and product on both statements;
  only the order of the transposed factors changes.
- claimed: transpose reverses product order; `s1ᵀ · s2ᵀ` is not a
  substitute for `s2ᵀ · s1ᵀ`.
- the gap: this counterexample refutes the unreversed equation on
  these values; it does not prove the generic reversed-order law.
- why: `(s1 · s2)ᵀ = [[4, 0], [7, 3]]`, while the unreversed
  `s1ᵀ · s2ᵀ = [[3, 11], [0, 4]]`.

## A wrong inner dimension is refused

### stdlib/linear-algebra/nat-wrong-right-inner-dimension

- spec: `spec/50-stdlib/65-linear-algebra.md` §§1.1–1.2.
- promise class: durable invariant.
- given: the same `a23` and `expected22`, which has **two**, not
  three, rows. Keep `m = 2`, `n = 3` and result width `p = 2`.
- expect: the **positive control** `matrix_product two three two
  a23 b32` checks, while the same call with only the right operand
  changed to `expected22` fails to type-check at its inner row
  index. A result-type change or a proof of false equality cannot
  repair the mismatched input.
- measured: acceptance and rejection on an otherwise identical
  call, using the checker to distinguish `Vec (Vec Nat two) three`
  from `Vec (Vec Nat two) two` at the input boundary.
- claimed: an `m × n` by `n × p` product is only applicable when the
  two occurrences of `n` agree, without a runtime dimension test.
- the gap: this case changes the right operand only; it does not
  separately exercise a wrong left inner dimension or validate a
  shipped CAT implementation. The type signature carries the
  universal property after it is delivered.
- why: a list-based, truncating product might silently compute two
  terms and pass a same-shape happy path, whereas this input must
  never reach the computation.

Each refusal is checked in a separate input from the accepted block
and from the other refusal. The unreversed-order input includes its
own successful reversed-order control on the same operands, so only
factor order changes:

Wrong-inner-dimension input:

```ken
const bad_product : Vec (Vec Nat two) two =
  matrix_product two three two a23 expected22
```

Unreversed-order input, appended to the shared fixture block:

```ken
const s1 : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one (v2 one two)
    (VCons (Vec Nat two) Zero (v2 three Zero)
      (VNil (Vec Nat two)))
const s2 : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one (v2 Zero one)
    (VCons (Vec Nat two) Zero (v2 two three)
      (VNil (Vec Nat two)))

theorem transpose_product_reversed_square_control
    : Equal (Vec (Vec Nat two) two)
        (transpose two two (matrix_product two two two s1 s2))
        (matrix_product two two two
          (transpose two two s2)
          (transpose two two s1)) = Refl

theorem transpose_product_unreversed_refused
    : Equal (Vec (Vec Nat two) two)
        (transpose two two (matrix_product two two two s1 s2))
        (matrix_product two two two
          (transpose two two s1)
          (transpose two two s2)) = Refl
```

The reversed-order control checks; the final false `Refl` is expected
to be refused with `TypeMismatch`.

## Coverage boundary

These cases exercise direct dot and identity values, aligned vector
addition, factor-two and factor-three plus empty vector scaling,
ordered matrix-vector multiplication, exact matrix products, left
and right identity, two parenthesizations of a product, transpose
orientation and involution, both §3 compatibility laws on non-square
values, an identity-factor control, and two refusals. They do not
assert a source-text census, a floating approximation, or a generic
proof from closed instances. The §1–2 generic theorems are landed in
the catalog; §3 D0 proofs, this seed's finite examples, and future
public catalog exports are distinct evidence. Shipped seed execution,
CI, publication, and landing are not established.
