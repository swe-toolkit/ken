# Commutativity and associativity of indexed sums

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §5.
Frame: `docs/program/issues/SPEC-LINEAR-ALGEBRA-SUM-COMM-ASSOC.md`.

This seed applies all four public laws to closed indexed values and
independently checks componentwise results. It includes the empty
vector, a `0 × 2` matrix, and a `2 × 0` matrix. These instances do
not prove the generic laws. The §5 catalog exports are absent from
this candidate's base; isolated D0-overlay checks are not execution
against shipped exports or an official conformance run. Each case
promises a **durable invariant**.

## Accepted shared client

The two length-two vectors are `[1,2]` and `[3,0]`, with a third
`[2,1]`. The matrices have rows `a=[[1,2],[3,0]]`,
`b=[[2,1],[1,2]]`, and `c=[[3,0],[2,1]]`. The expected results are
constructed independently of either operation under test. The
accepted prefix contains a same-operands `Refl` control for each of
the four isolated refusals below; a fresh copy of the **entire**
prefix precedes each refusal.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_add, matrix_add, vector_add_comm, vector_add_assoc,
   matrix_add_comm, matrix_add_assoc)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four
const six : Nat = Suc five

fn vec2 (x : Nat) (y : Nat) : Vec Nat two =
  VCons Nat one x (VCons Nat Zero y (VNil Nat))
fn matrix2 (r0 : Vec Nat two) (r1 : Vec Nat two)
    : Vec (Vec Nat two) two =
  VCons (Vec Nat two) one r0
    (VCons (Vec Nat two) Zero r1 (VNil (Vec Nat two)))

const xs : Vec Nat two = vec2 one two
const ys : Vec Nat two = vec2 three Zero
const zs : Vec Nat two = vec2 two one
const empty : Vec Nat Zero = VNil Nat
const rows0 : Vec (Vec Nat two) Zero = VNil (Vec Nat two)
const cols0 : Vec (Vec Nat Zero) two =
  VCons (Vec Nat Zero) one (VNil Nat)
    (VCons (Vec Nat Zero) Zero (VNil Nat) (VNil (Vec Nat Zero)))
const a : Vec (Vec Nat two) two = matrix2 xs ys
const b : Vec (Vec Nat two) two = matrix2 zs xs
const c : Vec (Vec Nat two) two = matrix2 ys zs
const changed_zs : Vec Nat two = vec2 two two
const changed_c : Vec (Vec Nat two) two = matrix2 ys ys
const expected_ab : Vec (Vec Nat two) two =
  matrix2 (vec2 three three) (vec2 four two)
const expected_abc : Vec (Vec Nat two) two =
  matrix2 (vec2 six three) (vec2 six three)

theorem vector_comm_closed
    : Equal (Vec Nat two) (vector_add two xs ys) (vector_add two ys xs) =
  vector_add_comm two xs ys

theorem vector_assoc_closed
    : Equal (Vec Nat two)
        (vector_add two (vector_add two xs ys) zs)
        (vector_add two xs (vector_add two ys zs)) =
  vector_add_assoc two xs ys zs

theorem matrix_comm_closed
    : Equal (Vec (Vec Nat two) two)
        (matrix_add two two a b) (matrix_add two two b a) =
  matrix_add_comm two two a b

theorem matrix_assoc_closed
    : Equal (Vec (Vec Nat two) two)
        (matrix_add two two (matrix_add two two a b) c)
        (matrix_add two two a (matrix_add two two b c)) =
  matrix_add_assoc two two a b c

theorem empty_vector_comm
    : Equal (Vec Nat Zero)
        (vector_add Zero empty empty) (vector_add Zero empty empty) =
  vector_add_comm Zero empty empty

theorem empty_vector_assoc
    : Equal (Vec Nat Zero)
        (vector_add Zero (vector_add Zero empty empty) empty)
        (vector_add Zero empty (vector_add Zero empty empty)) =
  vector_add_assoc Zero empty empty empty

theorem zero_row_matrix_comm
    : Equal (Vec (Vec Nat two) Zero)
        (matrix_add Zero two rows0 rows0)
        (matrix_add Zero two rows0 rows0) =
  matrix_add_comm Zero two rows0 rows0

theorem zero_row_matrix_assoc
    : Equal (Vec (Vec Nat two) Zero)
        (matrix_add Zero two (matrix_add Zero two rows0 rows0) rows0)
        (matrix_add Zero two rows0 (matrix_add Zero two rows0 rows0)) =
  matrix_add_assoc Zero two rows0 rows0 rows0

theorem zero_column_matrix_comm
    : Equal (Vec (Vec Nat Zero) two)
        (matrix_add two Zero cols0 cols0)
        (matrix_add two Zero cols0 cols0) =
  matrix_add_comm two Zero cols0 cols0

theorem zero_column_matrix_assoc
    : Equal (Vec (Vec Nat Zero) two)
        (matrix_add two Zero (matrix_add two Zero cols0 cols0) cols0)
        (matrix_add two Zero cols0 (matrix_add two Zero cols0 cols0)) =
  matrix_add_assoc two Zero cols0 cols0 cols0

theorem vector_sum_left_value
    : Equal (Vec Nat two) (vector_add two xs ys) (vec2 four two) =
  Refl

theorem vector_sum_right_value
    : Equal (Vec Nat two) (vector_add two ys xs) (vec2 four two) =
  Refl

theorem vector_regroup_left_value
    : Equal (Vec Nat two)
        (vector_add two (vector_add two xs ys) zs)
        (vec2 six three) =
  Refl

theorem vector_regroup_right_value
    : Equal (Vec Nat two)
        (vector_add two xs (vector_add two ys zs))
        (vec2 six three) =
  Refl

theorem matrix_sum_left_value
    : Equal (Vec (Vec Nat two) two) (matrix_add two two a b) expected_ab =
  Refl

theorem matrix_sum_right_value
    : Equal (Vec (Vec Nat two) two) (matrix_add two two b a) expected_ab =
  Refl

theorem matrix_regroup_left_value
    : Equal (Vec (Vec Nat two) two)
        (matrix_add two two (matrix_add two two a b) c)
        expected_abc =
  Refl

theorem matrix_regroup_right_value
    : Equal (Vec (Vec Nat two) two)
        (matrix_add two two a (matrix_add two two b c))
        expected_abc =
  Refl

theorem changed_vector_right_value
    : Equal (Vec Nat two)
        (vector_add two xs (vector_add two ys changed_zs))
        (vec2 six four) =
  Refl

theorem changed_matrix_right_value
    : Equal (Vec (Vec Nat two) two)
        (matrix_add two two a (matrix_add two two b changed_c))
        (matrix2 (vec2 six three) (vec2 (Suc six) two)) =
  Refl
```

### stdlib/linear-algebra/vector-sum-comm-assoc-closed

- spec: §5.1.
- promise class: durable invariant.
- given: `xs`, `ys`, `zs` and `empty` above.
- expect: both vector laws instantiate; the nonempty results have
  independently constructed `[4,2]` and `[6,3]` controls; both laws
  hold at length zero.
- why: a closed equality alone cannot certify a generic indexed proof
  or detect a repeated wrong operation on both endpoints.

### stdlib/linear-algebra/matrix-sum-comm-assoc-closed

- spec: §5.1.
- promise class: durable invariant.
- given: the three `2 × 2` matrices, plus zero-row and zero-column
  matrices with explicit indices.
- expect: both matrix laws instantiate at every chosen shape; the
  nonempty results have independently constructed matrix controls.
- why: rows must be added in their indexed order; neither zero rows
  nor zero columns may require an excluded special premise.

## Isolated refused claims

For **each** block below, first accept the unchanged client above.
Start from a fresh copy of that client and append **only one** refusal;
do not accumulate earlier failed claims. The matching positive
`Refl` value controls above use the same operands and goal shape, so
the intended mismatch must be in the asserted endpoint, not in
imports, fixtures, an earlier theorem or an unsupported proof form.

### stdlib/linear-algebra/reject-vector-sum-equals-left

- spec: §5.1.
- promise class: durable invariant.
- given: `xs=[1,2]`, `ys=[3,0]`; the accepted result is `[4,2]`.
- expect: reject `xs+ys=xs` at the appended theorem.
- why: the nonzero first component of `ys` cannot be discarded.

```ken
theorem reject_vector_sum_equals_left
    : Equal (Vec Nat two) (vector_add two xs ys) xs =
  Refl
```

### stdlib/linear-algebra/reject-matrix-sum-equals-left

- spec: §5.1.
- promise class: durable invariant.
- given: `a` and `b`; their independently checked sum is
  `[[3,3],[4,2]]`.
- expect: reject `a+b=a` at the appended theorem.
- why: a matrix sum may not discard the second matrix.

```ken
theorem reject_matrix_sum_equals_left
    : Equal (Vec (Vec Nat two) two) (matrix_add two two a b) a =
  Refl
```

### stdlib/linear-algebra/reject-vector-regroup-changed-component

- spec: §5.1.
- promise class: durable invariant.
- given: the accepted `((xs+ys)+zs)` equals `(xs+(ys+zs))`, both
  `[6,3]`; the independently checked changed right operand yields
  `[6,4]` when `[2,2]` replaces `zs`.
- expect: reject this single changed-component regrouping claim.
- why: associativity permits regrouping, not changing an operand.

```ken
theorem reject_vector_regroup_changed_component
    : Equal (Vec Nat two)
        (vector_add two (vector_add two xs ys) zs)
        (vector_add two xs (vector_add two ys changed_zs)) =
  Refl
```

### stdlib/linear-algebra/reject-matrix-regroup-changed-row

- spec: §5.1.
- promise class: durable invariant.
- given: accepted `((a+b)+c)` equals `(a+(b+c))`, both
  `[[6,3],[6,3]]`; on the right only, `changed_c` replaces `c`'s
  second row `[2,1]` with `[3,0]`. The accepted changed-right
  control is `[[6,3],[7,2]]`, with the first row unchanged.
- expect: reject the changed-row regrouping claim.
- why: row-wise associativity preserves every row value, not only
  the first.

```ken
theorem reject_matrix_regroup_changed_row
    : Equal (Vec (Vec Nat two) two)
        (matrix_add two two (matrix_add two two a b) c)
        (matrix_add two two a (matrix_add two two b changed_c)) =
  Refl
```

## Evidence boundary

The exact Ken fences were extracted into temporary fenced clients
under `catalog/packages/Algorithm/LinearAlgebra/` in a scratch
worktree at `0812a8d3d1138c8862a51c5b0feb3834fb7b686c`. Only
Architect's ruled catalog/test-pin patch was applied there (sha256
`33a052c0f15d9ba4a92c387d516060d22856ce19d21bd815890a581e58d11dab`).
A preexisting Ken CLI binary, built **before** this base, returned
`ken check` rc 0 for the full positive client. Each fresh accepted
prefix plus one refusal returned rc 1/E0038 `TypeMismatch` at the
appended `Refl`, not an import, fixture or earlier proof. Every
refusal declaration starts at byte offset 4807; error starts are
4901, 4912, 4995 and 5009 respectively. Logs are
`/tmp/spec-la-sum-Positive.log` and
`/tmp/spec-la-sum-Refusal{1..4}.log`. A scoped attempt to build the
CLI on this exact scratch base **timed out waiting on the shared
build lock**; it has no build verdict and the preexisting binary is
not an exact-base toolchain measurement. Architect's separate D0
measured generic proofs and trust identities on `4e7cf78a6`; it
did not execute these authored fences. These are source-shaped
**D0-overlay** checks, not shipped §5 exports, official conformance,
CI, publication, landing or a landed trust measurement.
