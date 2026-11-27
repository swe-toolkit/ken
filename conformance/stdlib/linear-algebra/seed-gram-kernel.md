# A null vector of the Gram matrix is a null vector of the matrix

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §25.
Spec child: `cee4b80c928dcb77c465a6bff71977d4a77fdd5b`.
Architect AC-0: `evt_51w8rbr649att`.

The law says that if `transpose a × a` sends `v` to zero, then `a`
sends `v` to zero. It does not say that `v` itself is zero. The
accepted client keeps `v` open at the fixed matrix `[[1, 0]]`, with the
Gram-kernel condition as a hypothesis.

## Accepted open client and closed controls

The closed controls compute the Gram matrix and show that
`second_unit = [0, 1]` is in its kernel, has self-dot one, and cannot
be zero: equality with the zero vector would imply `one = Zero`.
They do not replace the open application of the public law.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)
import Core.Logic.Transport (cong)
import Algorithm.LinearAlgebra.NatMatrix
  (dot,
    matrix_vector,
    matrix_product,
    transpose,
    vector_zero,
    matrix_vector_zero_of_gram_vector_zero)

const one : Nat = Suc Zero
const two : Nat = Suc one

fn row2 (x : Nat) (y : Nat) : Vec Nat two = VCons Nat one x (VCons Nat Zero y (VNil Nat))

const pick_first : Vec (Vec Nat two) one =
  VCons (Vec Nat two) Zero (row2 one Zero) (VNil (Vec Nat two))

const second_unit : Vec Nat two = row2 Zero one

theorem pick_first_gram_value
    : Equal
        (Vec (Vec Nat two) two)
        (matrix_product two one two (transpose one two pick_first) pick_first)
        (VCons (Vec Nat two) one (row2 one Zero) (VCons (Vec Nat two) Zero (row2 Zero Zero) (VNil (Vec Nat two)))) =
  Refl

theorem second_unit_in_gram_kernel
    : Equal
        (Vec Nat two)
        (matrix_vector two two (matrix_product two one two (transpose one two pick_first) pick_first) second_unit)
        (vector_zero two) =
  Refl

theorem second_unit_self_dot : Equal Nat (dot two second_unit second_unit) one = Proved

theorem second_unit_not_zero
    : Equal (Vec Nat two) second_unit (vector_zero two) → Equal Nat one Zero =
  λsame. cong (Vec Nat two) Nat second_unit (vector_zero two) (λw. dot two w w) same

theorem gram_kernel_open_in_v (v : Vec Nat two)
    : Equal
        (Vec Nat two)
        (matrix_vector two two (matrix_product two one two (transpose one two pick_first) pick_first) v)
        (vector_zero two)
      → Equal (Vec Nat one) (matrix_vector one two pick_first v) (vector_zero one) =
  λgram_zero. matrix_vector_zero_of_gram_vector_zero one two pick_first v gram_zero
```

### `stdlib/linear-algebra/gram-kernel-open-in-v`

- spec: §25, `matrix_vector_zero_of_gram_vector_zero`.
- promise class: durable invariant.
- given: the accepted block with `v` open and the Gram-kernel
  condition supplied as a hypothesis.
- expect: the block accepts; the theorem body applies the public law to
  show `matrix_vector one two pick_first v` is `vector_zero one`.
- measured: Architect AC-0 checked this exact open client in scratch.
- claimed: for this fixed `1 × 2` matrix, every vector satisfying the
  Gram-kernel premise is sent to zero by the matrix.
- gap: this instance does not check the generic proof body or every
  dimension.
- controls: `pick_first_gram_value` computes the Gram matrix as
  `[[1, 0], [0, 0]]`; `second_unit_in_gram_kernel` checks the premise
  for `[0, 1]`; and the following closed controls distinguish that
  nonzero vector from zero.
- why: `v` stays open and the premise is a hypothesis, so the accepted
  conclusion cannot be closed by computation alone.

## The open conclusion does not close by computation

For each trial, start from a fresh copy of the accepted block's imports,
constants and closed controls. Append the same theorem type for
`gram_kernel_open_in_v`, changing only its body to the indicated
compute-only closing. Keep `v` open in both trials.

```ken
theorem gram_kernel_open_in_v (v : Vec Nat two)
    : Equal
        (Vec Nat two)
        (matrix_vector two two (matrix_product two one two (transpose one two pick_first) pick_first) v)
        (vector_zero two)
      → Equal (Vec Nat one) (matrix_vector one two pick_first v) (vector_zero one) =
  λgram_zero. Proved
```

### `stdlib/linear-algebra/gram-kernel-proved-refusal`

- spec: §25.
- promise class: durable invariant.
- given: the fresh accepted prefix and the same open theorem type,
  closed with `λgram_zero. Proved`.
- expect: refusal with `KernelRejected(TypeMismatch)` at the theorem.
- measured: Architect AC-0 observed this refusal in scratch.
- claimed: computation alone does not establish the open conclusion.
- gap: this does not characterize all proofs of the equality.
- why: the matrix-vector result still depends on open `v` after the
  premise is abstracted.

```ken
theorem gram_kernel_open_in_v (v : Vec Nat two)
    : Equal
        (Vec Nat two)
        (matrix_vector two two (matrix_product two one two (transpose one two pick_first) pick_first) v)
        (vector_zero two)
      → Equal (Vec Nat one) (matrix_vector one two pick_first v) (vector_zero one) =
  λgram_zero. Refl
```

### `stdlib/linear-algebra/gram-kernel-refl-refusal`

- spec: §25.
- promise class: durable invariant.
- given: a fresh accepted prefix and the same open theorem type,
  closed with `λgram_zero. Refl`.
- expect: elaborator `TypeMismatch` because the goal sides are not
  convertible; the refusal is at the `Refl` token.
- measured: Architect AC-0 observed this refusal in scratch.
- claimed: the open conclusion is not reflexive by conversion.
- gap: no finer diagnostic wording is pinned.
- why: with open `v`, the matrix-vector term remains neutral.

## Refuse the stronger claim that `v` is zero

Start from a fresh copy of the accepted block and append only the twin
below. It changes the conclusion from `a v = 0` to `v = 0`.

```ken
theorem refused_gram_kernel_is_zero (v : Vec Nat two)
    : Equal
        (Vec Nat two)
        (matrix_vector two two (matrix_product two one two (transpose one two pick_first) pick_first) v)
        (vector_zero two)
      → Equal (Vec Nat two) v (vector_zero two) =
  λgram_zero. matrix_vector_zero_of_gram_vector_zero one two pick_first v gram_zero
```

### `stdlib/linear-algebra/gram-kernel-stronger-vector-zero-refusal`

- spec: §25, especially its conclusion `a v = 0`, not `v = 0`.
- promise class: durable invariant.
- given: the fresh accepted prefix followed by the stronger claim and
  its application of the public law.
- expect: `KernelRejected(TypeMismatch)` at declaration granularity.
- measured: Architect AC-0 observed this exact refusal; its span
  covers the declaration, not a finer term.
- claimed: the Gram-kernel premise does not imply `v = 0`.
- gap: the refusal alone would not show that the claim is false rather
  than merely missing a proof.
- controls: `second_unit_in_gram_kernel` checks the premise for
  `second_unit = [0, 1]`; `second_unit_self_dot` computes its self-dot
  as `one`; and `second_unit_not_zero` shows that equality to zero would
  imply `Equal Nat one Zero`.
- why: these closed controls establish a nonzero vector in the kernel;
  the negative row is not treated as its own counterexample.

## Evidence boundary

Architect AC-0 (`evt_51w8rbr649att`) reports that
`scripts/ken-cargo test -p ken-elaborator --test arch_d0_gram_kernel`
returned rc 0 with 2/2 tests on scratch base
`3c2b7066dac5e14c1f1a4610a4000c9f2461b7af`. It checked the generic law,
the open client, the compute-only refusals, and the false-twin refusal,
with no added `Axiom` and unchanged exact trusted `GlobalId` sets
(109 → 109). The NatMatrix source and roster test are unchanged on this
seed's base `af9d1d8b168e052097e9c323eb9aeab8ee8f1f7a`; the intervening
main changes are steward/program documents only. These are Architect D0
findings, not execution of this authored Markdown seed. CV ran no tests.
The seed has not been executed by the official conformance runner. No
assembled AC-T, official conformance, CI, publication, landing, or
measured landed-trust result is claimed.

Clean-room provenance: no `local/refs/` or external reference
implementations were consulted. This seed is grounded in §25, Architect
AC-0, and first principles.
