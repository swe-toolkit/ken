# Zero-vector units and closed laws

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md §10`.

Status: authored; not executed. The contract quantifies over every vector
length. This seed applies the four public laws at abstract length and
checks their closed behavior at length two. These closed instances do
not replace the generic law obligations.

## Accepted shared client

Use `n = 2` and `u = [1,2]`. Construct the expected vectors
independently of `vector_zero` and the operations under test.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix
  (vector_add, vector_zero, vector_scale, dot,
   vector_add_zero_left, vector_add_zero_right,
   vector_scale_zero, dot_zero_left)

const one : Nat = Suc Zero
const two : Nat = Suc one

fn pair (x : Nat) (y : Nat) : Vec Nat two =
  VCons Nat one x (VCons Nat Zero y (VNil Nat))

const u : Vec Nat two = pair one two
const zero2 : Vec Nat two = pair Zero Zero

theorem add_zero_left_open (n : Nat) (v : Vec Nat n)
  : Equal (Vec Nat n) (vector_add n (vector_zero n) v) v =
  vector_add_zero_left n v

theorem add_zero_right_open (n : Nat) (v : Vec Nat n)
  : Equal (Vec Nat n) (vector_add n v (vector_zero n)) v =
  vector_add_zero_right n v

theorem scale_zero_open (n : Nat) (v : Vec Nat n)
  : Equal (Vec Nat n) (vector_scale n Zero v) (vector_zero n) =
  vector_scale_zero n v

theorem dot_zero_open (n : Nat) (v : Vec Nat n)
  : Equal Nat (dot n (vector_zero n) v) Zero =
  dot_zero_left n v

theorem add_zero_left_value : Equal (Vec Nat two)
  (vector_add two (vector_zero two) u) (pair one two) = Refl

theorem add_zero_right_value : Equal (Vec Nat two)
  (vector_add two u (vector_zero two)) (pair one two) = Refl

theorem scale_zero_value : Equal (Vec Nat two)
  (vector_scale two Zero u) zero2 = Refl

theorem vector_zero_value : Equal (Vec Nat two)
  (vector_zero two) zero2 = Refl

theorem dot_zero_value : Equal Nat (dot two (vector_zero two) u) Zero =
  Proved

theorem vector_zero_empty : Equal (Vec Nat Zero)
  (vector_zero Zero) (VNil Nat) = Proved

theorem scale_one_same_form : Equal (Vec Nat two)
  (vector_scale two (Suc Zero) u) u = Refl
```

### stdlib/linear-algebra/vector-zero-laws-open

- spec: §10.1.
- promise class: durable invariant.
- given: arbitrary `n` and `v : Vec Nat n`.
- expect: all four public law applications check at their specified
  generic types without specializing the length or vector.
- claimed: the public theorem statements admit clients at any length,
  including zero.
- the gap: applying the exported laws does not establish their
  checked proof bodies; those are a separate package obligation.

### stdlib/linear-algebra/vector-zero-laws-closed

- spec: §10.2.
- promise class: durable invariant.
- given: `u = [1,2]` at length two.
- expect: left and right addition by `vector_zero` equal `[1,2]`,
  scaling `u` by zero equals `[0,0]`, and `vector_zero two` equals
  `[0,0]`. Each vector equality checks with `Refl`. The dot-zero
  equality checks with `Proved`; it does not use `Refl`.
- claimed: the operations and laws have these values on the stated
  nonempty vector.
- the gap: finite values do not establish the generic laws. The
  separate open client applies all four exported laws.

### stdlib/linear-algebra/vector-zero-empty

- spec: §10.1–§10.2.
- promise class: durable invariant.
- given: the zero-length vector.
- expect: `vector_zero Zero` equals `VNil Nat` as
  `Equal (Vec Nat Zero)`, checked with `Proved`.
- claimed: the public zero-vector operation preserves the empty index.
- the gap: this boundary case alone does not establish behavior at
  positive lengths.

## Isolated refused claim

Start from a fresh copy of the entire accepted client above and append
only the false theorem. The accepted same-form scale-one equality is in
the shared prefix. The false and true rows differ only in the factor;
the vector and `Refl` proof form stay fixed.

### stdlib/linear-algebra/reject-vector-scale-zero-as-identity

- spec: §10.2.
- promise class: durable invariant.
- given: the accepted shared client with `u = [1,2]`.
- expect: refuse the false claim at its appended `Refl`:
  scaling by zero gives `[0,0]`, not `[1,2]`. Do not pin an
  unspecified diagnostic variant.
- control: `scale_one_same_form` in the accepted prefix checks with
  `Refl`, for the same vector and result type. This control rejects
  proof-shape or fixture failures as explanations for the negative.
- the gap: this false instance does not negate the generic law's
  checked body; it distinguishes scale-zero content from scale-one.

```ken
theorem seed_neg : Equal (Vec Nat two) (vector_scale two Zero u) u = Refl
```

## Evidence boundary

The vector-zero seed is authored but unexecuted. Foundation's
package-owned test is separate evidence and is not an official
conformance-runner execution. No official runner, CI, publication,
landing, or landed trust measurement is claimed.
