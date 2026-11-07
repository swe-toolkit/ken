# A vector with zero self-dot is zero

Format: `../../README.md`.
Spec: `spec/50-stdlib/65-linear-algebra.md` §14.
Frame: `docs/program/issues/SPEC-LINEAR-ALGEBRA-DOT-SELF-ZERO.md`.

Status: authored; not executed. The public law is generic; these cases
exercise its application at arbitrary inputs and at closed vectors. The
closed cases do not establish the generic proof.

## Accepted client

Use `zeros = [0,0]` and `bumped = [0,1]`. The self-dot of `zeros` is
zero; the self-dot of `bumped` is one.

```ken
import Data.Vector.Vector (Vec, VNil, VCons)
import Algorithm.LinearAlgebra.NatMatrix
  (dot, vector_zero, vector_zero_of_dot_self_zero)

const one : Nat = Suc Zero
const two : Nat = Suc one

fn pair (x : Nat) (y : Nat) : Vec Nat two =
  VCons Nat one x (VCons Nat Zero y (VNil Nat))

const zeros : Vec Nat two = pair Zero Zero
const bumped : Vec Nat two = pair Zero one

theorem dot_self_zero_open
    (n : Nat) (v : Vec Nat n)
    (self_dot_zero : Equal Nat (dot n v v) Zero)
  : Equal (Vec Nat n) v (vector_zero n) =
    vector_zero_of_dot_self_zero n v self_dot_zero

theorem zeros_self : Equal Nat (dot two zeros zeros) Zero = Proved

theorem zeros_is_zero
    : Equal (Vec Nat two) zeros (vector_zero two) =
      vector_zero_of_dot_self_zero two zeros zeros_self

theorem bumped_self
    : Equal Nat (dot two bumped bumped) (Suc Zero) = Proved
```

### stdlib/linear-algebra/dot-self-zero-open

- spec: §14.
- promise class: durable invariant.
- given: arbitrary `n`, `v : Vec Nat n`, and a proof that its self-dot is
  zero.
- expect: the public law applies to the generic operands and returns
  `Equal (Vec Nat n) v (vector_zero n)`.
- claimed: the implication is available at every vector length.
- gap: applying the public theorem does not establish its checked proof
  body.

### stdlib/linear-algebra/dot-self-zero-closed

- spec: §14.
- promise class: durable invariant.
- given: `zeros = [0,0]` and `bumped = [0,1]` at length two.
- expect: `zeros_self` proves the self-dot is zero; `zeros_is_zero`
  applies the public law; `bumped_self` proves the other self-dot is
  `Suc Zero` using the same `Proved` form.
- claimed: the zero vector meets the law's premise and the one-entry
  vector does not.
- gap: these closed values delimit the contract but do not prove the
  generic theorem.

## Isolated refusals

Each refusal starts from a fresh copy of the entire accepted client above
and appends only the stated declaration. The `bumped_self` control checks
the same self-dot expression with the same `Proved` form and the nonzero
result `Suc Zero`.

### stdlib/linear-algebra/reject-bumped-vector-from-zero-premise

- spec: §14.
- promise class: durable invariant.
- given: the accepted client with `bumped = [0,1]` and
  `bumped_self : Equal Nat (dot two bumped bumped) (Suc Zero)`.
- expect: reject the attempted law application because `Proved` cannot
  establish `Equal Nat (dot two bumped bumped) Zero`.
- control: `bumped_self` accepts with `Proved` and the same self-dot
  expression when its value is `Suc Zero`.
- why: the law cannot infer that the nonzero vector is zero from a false
  zero-premise.
- gap: this refusal does not establish the law's generic proof body.

```ken
theorem bumped_is_zero
    : Equal (Vec Nat two) bumped (vector_zero two) =
      vector_zero_of_dot_self_zero two bumped Proved
```

### stdlib/linear-algebra/reject-bumped-self-dot-zero

- spec: §14.
- promise class: durable invariant.
- given: the accepted client with `bumped = [0,1]`.
- expect: reject the false equation at its `Proved` claim.
- control: `bumped_self` uses the same self-dot and proof form, changing
  only the claimed value from `Zero` to `Suc Zero`.
- why: the self-dot is one, not zero.
- gap: this closed false equation does not establish the law's generic
  proof body.

```ken
theorem bumped_self_zero
    : Equal Nat (dot two bumped bumped) Zero = Proved
```

## Evidence boundary

The seed encodes the ruled §14 contract. Architect and Foundation D0
observations are attributed scratch evidence; they are not execution of
this seed or a candidate. CV has not run `ken check`, semantic tests, or
the official conformance runner. No CI, publication, or landing is
claimed. No `local/refs/` or external reference implementation was
consulted.
