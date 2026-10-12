# Dimension-safe natural matrix operations

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §1.

These nine cases are **authored behavioral oracles**, not executed
conformance results. Out-of-repository source-shaped Vector with only
the ruled publications (SHA256
`396224f79cade3d0e06e7d2aa11a46c8f5a3f5d2ef4fd9d1629da2698b68bc40`)
and `Algorithm.LinearAlgebra.NatMatrix` with the seven operations
(SHA256
`c77599146e1409641a7cc0d6b67b3ea83b66387ca64dacdad1af9f3f183080ff`)
checked rc=0 against current first-party dependencies. The seed's
positive code (ten closed assertions) was extracted to
`/tmp/ken-la-seed/catalog/packages/Algorithm/LinearAlgebra/SeedPositive.ken`
(SHA256
`9ea82015ae6ef270e3a26d1b918143e1429e84f82ae136550e0c369cf77a7e8b`):
`ken check` returned rc=0. The same code with the isolated negative
call appended was extracted to `SeedNegative.ken` (SHA256
`1eb00ce7569e3ece7d94e7548e61079212850acc836a54ff1f7ace2fb2bf7584`):
`ken check` returned rc=1 `KernelRejected TypeMismatch`. Replacing
only the dot oracle's expected `five` with `six` in the positive code
(`SeedBadDot.ken`, SHA256
`0a1d249969bf589da00532c1ef4e16e0dbee88ac74cc5b61e9175d34fe784b83`)
also returned rc=1 `KernelRejected TypeMismatch`. All three used
`/workspaces/ken/target/debug/ken`, freshly built from the candidate's
Rust base `66cb5e9b` with `scripts/ken-cargo build -p ken-cli --bin ken`
(rc=0). These are source-shaped scratch checks, not execution against
delivered CAT.
The checked generic transpose, vector identity, right matrix
identity and dot commutativity bodies are in scratch
`MatrixRightIdentityCols.ken`
(SHA256
`cfa7de7ac04336ba54a916241a7eafe4c4035898ca26be0c124f067fe3313672`);
the Architect separately checked generic dot distributivity. None
of these scratch checks delivers the catalog exports, executes this
seed against a delivered package, runs Cargo tests, or establishes CI
or publication. Every case has
promise class **durable invariant**.

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

theorem vector_scale_empty_oracle : Equal (Vec Nat Zero)
  (vector_scale Zero three (VNil Nat)) (VNil Nat) = Proved

theorem matrix_vector_oracle : Equal (Vec Nat two)
  (matrix_vector two three a23 (v3 two two one))
  (v2 six five) = Refl

theorem product_oracle : Equal (Vec (Vec Nat two) two)
  (matrix_product two three two a23 b32) expected22 = Refl

theorem right_identity_oracle : Equal (Vec (Vec Nat three) two)
  (matrix_product two three three a23 (identity three)) a23 = Refl

theorem transpose_intermediate_oracle : Equal (Vec (Vec Nat two) three)
  (transpose two three a23) expected32 = Refl

theorem transpose_round_trip_oracle : Equal (Vec (Vec Nat three) two)
  (transpose three two (transpose two three a23)) a23 = Refl
```

The nonempty closed vector equalities check using `Refl`. The
zero-length scaling case uses `Proved`, as the catalog's
`map_vnil` does; `Equal Nat` in the direct dot oracle also takes
`Proved`. Neither asserts a generic Vec extensionality law. These
fixed value oracles must not be used to claim that every matrix is
a unit or that two still-gated generic laws were proved.

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
  identity theorem; left identity remains gated.
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
- measured: multiplication by a factor other than zero or one,
  preservation of an input zero, and an empty indexed boundary.
- claimed: scaling multiplies every component without changing its
  length, and the zero-length result is empty.
- the gap: these two fixtures do not prove a generic scaling law.
- why: multiplying only the head, adding instead of multiplying, or
  fabricating a scalar entry for the empty vector changes a result.

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
  laws or matrix-product associativity. The latter is explicitly
  gated, not silently inferred from this result.
- why: treating the rows of `b32` as its columns, dropping its last
  row, or confusing the two outer dimensions changes at least one
  expected cell.

## Right identity, not a left-identity claim

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
  concrete nondegenerate witness; no left identity theorem follows.
- the gap: a closed result cannot discharge the universal right
  identity proof. The **left** identity is gated on
  `LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL` and not tested here.
- why: replacing the diagonal with all ones or losing the second
  row changes the result. A square-only oracle would not reveal a
  mistaken assumption that both outer dimensions must be equal.

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

The negative fixture belongs in a **separate** checker input from the
accepted cases, to prevent an expected rejection from obscuring
their success:

```ken
const bad_product : Vec (Vec Nat two) two =
  matrix_product two three two a23 expected22
```

## Coverage boundary

These cases exercise the direct dot and identity results, aligned
vector addition, nontrivial and empty vector scaling, ordered
matrix-vector multiplication, exact matrix products, right-sided
identity, transpose orientation and involution, and one real
type-index refusal. They do not assert a source-text census, a floating
approximation, left matrix identity, or matrix-product associativity.
The latter two remain named Language-gated obligations rather than
requirements of this seed or of the first catalog node.
