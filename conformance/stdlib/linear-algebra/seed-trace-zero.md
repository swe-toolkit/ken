# Trace of the zero matrix

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §24.
Frame: `docs/program/issues/SPEC-LINEAR-ALGEBRA-TRACE-ZERO.md`.
Architect AC-0: `evt_4cc69yznrxcpn`.

This seed is authored but unexecuted. No claim is made that the official
conformance runner has executed it.

The open pair separates `trace_zero` from a law that gives the trace of
`matrix_zero n n` as `n`, in the form of `trace_identity`. Closed rows
cannot distinguish those laws by themselves. The positive trace-identity
control shows that a claim with the dimension as its result is satisfiable
for the identity matrix.

## Accepted block

Every row below accepts. The open client applies `trace_zero` to arbitrary
`n`; the two closed zero rows check computation and the public law, and the
identity row is the positive value control.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)

import Algorithm.LinearAlgebra.NatMatrix (matrix_zero, trace, trace_zero, trace_identity, identity)

const three : Nat = Suc (Suc (Suc Zero))

theorem open_trace_zero (n : Nat) : Equal Nat (trace n (matrix_zero n n)) Zero =
  trace_zero n

theorem closed_trace_zero : Equal Nat (trace three (matrix_zero three three)) Zero = Proved

theorem closed_trace_zero_by_law : Equal Nat (trace three (matrix_zero three three)) Zero =
  trace_zero three

theorem closed_trace_identity : Equal Nat (trace three (identity three)) three = Proved
```

### stdlib/linear-algebra/trace-zero-open

- spec: §24; `trace n (matrix_zero n n)` equals `Zero` for every `n`.
- promise class: durable invariant.
- given: arbitrary `n : Nat` and `matrix_zero n n`.
- expect: the accepted block accepts, including `open_trace_zero`.
- measured: AC-0 D0 checked the open public-law client on the landed base.
- claimed: the public law establishes the trace of the zero matrix at every
  dimension.
- gap: the finite closed controls do not establish the generic law.
- why: the open dimension prevents the law application from being established
  solely by reducing a closed matrix.

### stdlib/linear-algebra/trace-zero-closed-controls

- spec: §24 and §18.
- promise class: durable invariant.
- given: the accepted block at `three = 3`.
- expect: `closed_trace_zero` and `closed_trace_zero_by_law` accept with `Zero`;
  `closed_trace_identity` accepts with `three`.
- measured: AC-0 D0 checked these closed computations and law application.
- claimed: the zero matrix has trace zero, while the identity matrix has trace
  equal to its dimension.
- gap: closed computations do not distinguish the generic zero-trace law
  from a law giving the dimension.
- why: the identity row is the positive control for that alternative result.

## Refuse the open trace-equals-dimension law application

Run the accepted block in a fresh environment, then append only the theorem
below. It differs from `open_trace_zero` only in claiming `n` rather than
`Zero`. Require refusal inside this theorem. Do not pin a span or GlobalIds.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)

import Algorithm.LinearAlgebra.NatMatrix (matrix_zero, trace, trace_zero, trace_identity, identity)

const three : Nat = Suc (Suc (Suc Zero))

theorem open_trace_zero (n : Nat) : Equal Nat (trace n (matrix_zero n n)) Zero =
  trace_zero n

theorem closed_trace_zero : Equal Nat (trace three (matrix_zero three three)) Zero = Proved

theorem closed_trace_zero_by_law : Equal Nat (trace three (matrix_zero three three)) Zero =
  trace_zero three

theorem closed_trace_identity : Equal Nat (trace three (identity three)) three = Proved

theorem open_trace_zero_is_dimension (n : Nat) : Equal Nat (trace n (matrix_zero n n)) n =
  trace_zero n
```

### stdlib/linear-algebra/reject-open-trace-zero-equals-dimension-by-law

- spec: §24; the law concludes `Zero`, not the dimension.
- promise class: durable invariant.
- given: the full accepted block with the open trace claimed equal to `n` by
  `trace_zero n` appended.
- expect: the prefix accepts; `open_trace_zero_is_dimension` refuses with
  `KernelRejected(TypeMismatch)` inside its theorem.
- measured: AC-0 D0 refused this open law-form twin on a fresh accepted prefix.
- claimed: `trace_zero` cannot establish that the trace of the zero matrix is
  its dimension for arbitrary `n`.
- gap: the refusal does not measure execution of this authored seed.
- controls: `open_trace_zero` accepts with the same open input and public law,
  differing only in the claimed result.
- why: the trace is zero, not `n`, for an arbitrary dimension.

## Refuse the open trace-equals-dimension computation

Run the full accepted block in a fresh environment, then append only the
following theorem. It makes the same open claim as the previous refusal, but
uses `Proved` instead of `trace_zero n`. Require refusal inside this theorem.
Do not pin a span or GlobalIds.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)

import Algorithm.LinearAlgebra.NatMatrix (matrix_zero, trace, trace_zero, trace_identity, identity)

const three : Nat = Suc (Suc (Suc Zero))

theorem open_trace_zero (n : Nat) : Equal Nat (trace n (matrix_zero n n)) Zero =
  trace_zero n

theorem closed_trace_zero : Equal Nat (trace three (matrix_zero three three)) Zero = Proved

theorem closed_trace_zero_by_law : Equal Nat (trace three (matrix_zero three three)) Zero =
  trace_zero three

theorem closed_trace_identity : Equal Nat (trace three (identity three)) three = Proved

theorem open_trace_zero_is_dimension_proved (n : Nat) : Equal Nat (trace n (matrix_zero n n)) n = Proved
```

### stdlib/linear-algebra/reject-open-trace-zero-equals-dimension-proved

- spec: §24; the open trace of the zero matrix is `Zero`, not `n`.
- promise class: durable invariant.
- given: the full accepted block with the open trace claimed equal to `n` by
  `Proved` appended.
- expect: the prefix accepts; `open_trace_zero_is_dimension_proved` refuses
  with `KernelRejected(TypeMismatch)` inside its theorem.
- measured: AC-0 D0 refused this open computation claim on a fresh accepted
  prefix.
- claimed: the open false result is not accepted by computation either.
- gap: the refusal does not measure execution of this authored seed.
- controls: the open law-form twin uses the same trace and claimed result but
  separately tests the public law application.
- why: the dimension is open, so the false result is not a closed computation.

## Refuse the closed trace-equals-dimension value

Run the accepted block in a fresh environment, then append only the theorem
below. It differs from `closed_trace_zero` only in claiming `three` rather
than `Zero`. Require refusal inside this theorem. Do not pin a span or
GlobalIds.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)

import Algorithm.LinearAlgebra.NatMatrix (matrix_zero, trace, trace_zero, trace_identity, identity)

const three : Nat = Suc (Suc (Suc Zero))

theorem open_trace_zero (n : Nat) : Equal Nat (trace n (matrix_zero n n)) Zero =
  trace_zero n

theorem closed_trace_zero : Equal Nat (trace three (matrix_zero three three)) Zero = Proved

theorem closed_trace_zero_by_law : Equal Nat (trace three (matrix_zero three three)) Zero =
  trace_zero three

theorem closed_trace_identity : Equal Nat (trace three (identity three)) three = Proved

theorem closed_trace_zero_is_dimension : Equal Nat (trace three (matrix_zero three three)) three = Proved
```

### stdlib/linear-algebra/reject-closed-trace-zero-equals-dimension

- spec: §24; the trace of `matrix_zero three three` is `Zero`.
- promise class: durable invariant.
- given: the accepted block with `closed_trace_zero_is_dimension` appended.
- expect: the prefix accepts; `closed_trace_zero_is_dimension` refuses with
  `KernelRejected(TypeMismatch)` inside its theorem.
- measured: AC-0 D0 refused this closed false-value claim on a fresh accepted
  prefix.
- claimed: the trace of the 3 × 3 zero matrix is not its dimension.
- gap: this closed value does not establish the generic trace-zero law.
- controls: `closed_trace_zero` accepts with the same matrix and `Proved` form;
  `closed_trace_identity` accepts a positive trace-equals-dimension control.
- why: the zero matrix has trace `Zero`, while `three` is nonzero.

## Evidence boundary

Architect AC-0 checked the client forms and all three refusals on scratch at
`206d4cc532187356b1e03fbbefdcdb1caea7f9fd`. This does not mean the Markdown
seed was executed. No official conformance run, package test by CV, CI,
publication, landing, or landed-trust result is claimed.
