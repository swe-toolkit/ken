# Least-k prefixes across selection bounds

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §4.5.
Frame: `docs/program/issues/SPEC-OPTIMIZATION-LEAST-K-PREFIX.md`.

The fixture uses input `[A, B, C, D]` with costs `2, 1, 2, 3`.
Stable cost sorting gives `[B, A, C, D]`; the equal-cost pair `A, C`
straddles the `k = 2` cut. The public `least_k_prefix` law states that
selecting `k` from the `k + m` result returns the `k` result. The public
`take::prefix` law needs the premise that the outer bound does not exceed
the inner bound.

Architect's AC-0 D0 checked these client rows on scratch base
`fd523a842465c0bc185140167143ad457d51bc37`. This seed is authored at
current base `a731359837ec650464b0d17f3f03d64088ec1609`; the D0 does
not measure that later base or execute this Markdown seed. No official
conformance execution, CI, publication, landing, or landed-trust result
is claimed.

## Open public clients

Check both clients with their selective imports. They apply the generic
public laws at open inputs rather than closing the result by computation.

```ken
import Algorithm.Optimization.Argmin (least_k, least_k_prefix)
import Core.Classes.LawfulClasses (leq_nat)
import Data.Collections.Derived (take)
import Data.Numeric.Nat.Arithmetic (add)

theorem least_k_prefix_generic
      (a : Type) (cost : a → Nat) (k : Nat) (m : Nat) (xs : List a)
    : Equal (List a) (least_k a cost k xs) (take a k (least_k a cost (add k m) xs)) =
  least_k_prefix a cost k m xs

theorem take_prefix_generic
      (a : Type) (k : Nat) (n : Nat) (xs : List a)
    : Equal Bool (leq_nat k n) True → Equal (List a) (take a k (take a n xs)) (take a k xs) =
  (proof prefix for take) a k n xs
```

### stdlib/optimization/least-k-prefix-open

- spec: §4.5, `least_k_prefix` and `take::prefix`.
- promise class: durable invariant.
- given: arbitrary carrier, cost function, bounds, list, and the
  required bound premise for `take::prefix`.
- expect: both clients accept through direct public imports and
  applications.
- measured: the clients type-check with the generic law arguments open.
- claimed: callers can reuse the two laws at arbitrary inputs within
  their stated premises.
- gap: imported theorem availability is not a separate check of its
  generic proof body.
- why: the open arguments prevent these applications from being
  established solely by reducing the closed fixture below.

## Accepted closed clients

Check the whole block. It checks the least two and least three values,
the last two of the least-three result, the closed prefix equality and
its public-law application. It also checks both sides of `take::prefix`
by value and applies the public law with the valid bound `2 ≤ 3`.

```ken
import Algorithm.Optimization.Argmin (least_k, least_k_prefix)
import Data.Collections.Derived (take, reverse)
import Data.Numeric.Nat.Arithmetic (add)

data Candidate : Type where {
  A : Candidate;
  B : Candidate;
  C : Candidate;
  D : Candidate
}

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two

fn seed_cost (x : Candidate) : Nat =
  match x {
    A ↦ two;
    B ↦ one;
    C ↦ two;
    D ↦ three
  }

const input : List Candidate =
  Cons Candidate A (Cons Candidate B (Cons Candidate C (Cons Candidate D (Nil Candidate))))

fn last_two (xs : List Candidate) : List Candidate =
  reverse Candidate (take Candidate two (reverse Candidate xs))

theorem v_least_two
    : Equal (List Candidate) (least_k Candidate seed_cost two input)
        (Cons Candidate B (Cons Candidate A (Nil Candidate))) =
  Refl

theorem v_least_three
    : Equal (List Candidate) (least_k Candidate seed_cost (add two one) input)
        (Cons Candidate B (Cons Candidate A (Cons Candidate C (Nil Candidate)))) =
  Refl

theorem v_last_two_of_three
    : Equal (List Candidate) (last_two (least_k Candidate seed_cost (add two one) input))
        (Cons Candidate A (Cons Candidate C (Nil Candidate))) =
  Refl

theorem c_closed_prefix
    : Equal (List Candidate) (least_k Candidate seed_cost two input)
        (take Candidate two (least_k Candidate seed_cost (add two one) input)) =
  Refl

theorem c_law_prefix
    : Equal (List Candidate) (least_k Candidate seed_cost two input)
        (take Candidate two (least_k Candidate seed_cost (add two one) input)) =
  least_k_prefix Candidate seed_cost two one input

theorem v_take_three_of_two : Equal (List Candidate) (take Candidate three (take Candidate two input))
    (Cons Candidate A (Cons Candidate B (Nil Candidate))) =
  Refl

theorem v_take_three
    : Equal (List Candidate) (take Candidate three input)
        (Cons Candidate A (Cons Candidate B (Cons Candidate C (Nil Candidate)))) =
  Refl

theorem t_closed_take
    : Equal (List Candidate) (take Candidate two (take Candidate three input)) (take Candidate two input) =
  Refl

theorem t_law_take
    : Equal (List Candidate) (take Candidate two (take Candidate three input)) (take Candidate two input) =
  (proof prefix for take) Candidate two three input Proved
```

### stdlib/optimization/least-k-prefix-closed

- spec: §4.5, both public laws.
- promise class: durable invariant.
- given: four distinct candidates with costs `2, 1, 2, 3`, `k = 2`
  and `m = 1`.
- expect: the entire block accepts, including all value controls and
  both public-law applications.
- measured: stable least-two and least-three results, the tied cut,
  the last-two value, and `take::prefix` on valid bounds.
- claimed: the fixture satisfies both public laws and fixes the values
  used by the three refusals.
- gap: closed instances do not establish either generic proof body.
- why: `A` and `C` tie at cost two; the stable input order makes `A`
  precede `C` at the cut.

## Refuse equality with the same list at both bounds

For X-SAME, append only the final theorem to a fresh copy of the
accepted block below. The prefix must accept first. The false equality
compares the two-element least result with the three-element least
result on the same input.

```ken
import Algorithm.Optimization.Argmin (least_k, least_k_prefix)
import Data.Collections.Derived (take, reverse)
import Data.Numeric.Nat.Arithmetic (add)

data Candidate : Type where {
  A : Candidate;
  B : Candidate;
  C : Candidate;
  D : Candidate
}

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two

fn seed_cost (x : Candidate) : Nat =
  match x {
    A ↦ two;
    B ↦ one;
    C ↦ two;
    D ↦ three
  }

const input : List Candidate =
  Cons Candidate A (Cons Candidate B (Cons Candidate C (Cons Candidate D (Nil Candidate))))

fn last_two (xs : List Candidate) : List Candidate =
  reverse Candidate (take Candidate two (reverse Candidate xs))

theorem v_least_two
    : Equal (List Candidate) (least_k Candidate seed_cost two input)
        (Cons Candidate B (Cons Candidate A (Nil Candidate))) =
  Refl

theorem v_least_three
    : Equal (List Candidate) (least_k Candidate seed_cost (add two one) input)
        (Cons Candidate B (Cons Candidate A (Cons Candidate C (Nil Candidate)))) =
  Refl

theorem v_last_two_of_three
    : Equal (List Candidate) (last_two (least_k Candidate seed_cost (add two one) input))
        (Cons Candidate A (Cons Candidate C (Nil Candidate))) =
  Refl

theorem c_closed_prefix
    : Equal (List Candidate) (least_k Candidate seed_cost two input)
        (take Candidate two (least_k Candidate seed_cost (add two one) input)) =
  Refl

theorem c_law_prefix
    : Equal (List Candidate) (least_k Candidate seed_cost two input)
        (take Candidate two (least_k Candidate seed_cost (add two one) input)) =
  least_k_prefix Candidate seed_cost two one input

theorem v_take_three_of_two : Equal (List Candidate) (take Candidate three (take Candidate two input))
    (Cons Candidate A (Cons Candidate B (Nil Candidate))) =
  Refl

theorem v_take_three
    : Equal (List Candidate) (take Candidate three input)
        (Cons Candidate A (Cons Candidate B (Cons Candidate C (Nil Candidate)))) =
  Refl

theorem t_closed_take
    : Equal (List Candidate) (take Candidate two (take Candidate three input)) (take Candidate two input) =
  Refl

theorem t_law_take
    : Equal (List Candidate) (take Candidate two (take Candidate three input)) (take Candidate two input) =
  (proof prefix for take) Candidate two three input Proved

theorem x_same_list : Equal (List Candidate) (least_k Candidate seed_cost two input) (least_k Candidate seed_cost (add two one) input) = Refl
```

### stdlib/optimization/reject-same-list-at-two-bounds

- spec: §4.5, distinguishing the `k` and `k + m` results.
- promise class: durable invariant.
- given: the fresh accepted block followed by X-SAME at `k = 2`,
  `m = 1`.
- expect: the prefix accepts; X-SAME refuses at its own `Refl` with
  `ElabError::TypeMismatch`, reason `Refl expects an `Eq`-shaped goal`.
- measured: the D0 refusal span is inside `x_same_list` at its `Refl`.
- claimed: the `k` result need not equal the full `k + m` result.
- gap: this closed counterexample does not characterize every
  incorrect relation between different selection sizes.
- controls: `v_least_two`, `v_least_three`, `c_closed_prefix` and
  `c_law_prefix` accept on the same input before the false claim.
- why: the accepted values have lengths two and three respectively.

## Refuse equality with the last two of the longer result

For X-LAST, append only the final theorem to a fresh copy of the
accepted block below. The prefix must accept first. This tests order
at the tied cut: the first two of the least-three result are `[B, A]`,
while its last two are `[A, C]`.

```ken
import Algorithm.Optimization.Argmin (least_k, least_k_prefix)
import Data.Collections.Derived (take, reverse)
import Data.Numeric.Nat.Arithmetic (add)

data Candidate : Type where {
  A : Candidate;
  B : Candidate;
  C : Candidate;
  D : Candidate
}

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two

fn seed_cost (x : Candidate) : Nat =
  match x {
    A ↦ two;
    B ↦ one;
    C ↦ two;
    D ↦ three
  }

const input : List Candidate =
  Cons Candidate A (Cons Candidate B (Cons Candidate C (Cons Candidate D (Nil Candidate))))

fn last_two (xs : List Candidate) : List Candidate =
  reverse Candidate (take Candidate two (reverse Candidate xs))

theorem v_least_two
    : Equal (List Candidate) (least_k Candidate seed_cost two input)
        (Cons Candidate B (Cons Candidate A (Nil Candidate))) =
  Refl

theorem v_least_three
    : Equal (List Candidate) (least_k Candidate seed_cost (add two one) input)
        (Cons Candidate B (Cons Candidate A (Cons Candidate C (Nil Candidate)))) =
  Refl

theorem v_last_two_of_three
    : Equal (List Candidate) (last_two (least_k Candidate seed_cost (add two one) input))
        (Cons Candidate A (Cons Candidate C (Nil Candidate))) =
  Refl

theorem c_closed_prefix
    : Equal (List Candidate) (least_k Candidate seed_cost two input)
        (take Candidate two (least_k Candidate seed_cost (add two one) input)) =
  Refl

theorem c_law_prefix
    : Equal (List Candidate) (least_k Candidate seed_cost two input)
        (take Candidate two (least_k Candidate seed_cost (add two one) input)) =
  least_k_prefix Candidate seed_cost two one input

theorem v_take_three_of_two : Equal (List Candidate) (take Candidate three (take Candidate two input))
    (Cons Candidate A (Cons Candidate B (Nil Candidate))) =
  Refl

theorem v_take_three
    : Equal (List Candidate) (take Candidate three input)
        (Cons Candidate A (Cons Candidate B (Cons Candidate C (Nil Candidate)))) =
  Refl

theorem t_closed_take
    : Equal (List Candidate) (take Candidate two (take Candidate three input)) (take Candidate two input) =
  Refl

theorem t_law_take
    : Equal (List Candidate) (take Candidate two (take Candidate three input)) (take Candidate two input) =
  (proof prefix for take) Candidate two three input Proved

theorem x_last_two : Equal (List Candidate) (least_k Candidate seed_cost two input) (last_two (least_k Candidate seed_cost (add two one) input)) = Refl
```

### stdlib/optimization/reject-last-two-as-shorter-prefix

- spec: §4.5, distinguishing the first `k` results from the last
  `k` results of a longer selection.
- promise class: durable invariant.
- given: the fresh accepted block followed by X-LAST at `k = 2`,
  `m = 1`.
- expect: the prefix accepts; X-LAST refuses at its own `Refl` with
  `ElabError::TypeMismatch`, reason `Refl expects an `Eq`-shaped goal`.
- measured: the D0 refusal span is inside `x_last_two` at its `Refl`.
- claimed: a shorter selection need not equal the last segment of
  the longer selection.
- gap: one closed counterexample does not characterize every
  incorrect relation between a prefix and a suffix.
- controls: `v_least_two` and `v_last_two_of_three` fix both sides;
  `c_closed_prefix` and `c_law_prefix` check the correct prefix relation.
- why: stable order retains `A` before `C` in the tied cost-two class;
  the shorter result is `[B, A]`, not `[A, C]`.

## Refuse `take::prefix` with an invalid bound

For X-BOUND, append only the final theorem to a fresh copy of the
accepted block below. The prefix must accept first. The two side-value
controls show the false claim's distinct endpoints, while `t_law_take`
shows that the public law accepts a valid `2 ≤ 3` premise.

```ken
import Algorithm.Optimization.Argmin (least_k, least_k_prefix)
import Data.Collections.Derived (take, reverse)
import Data.Numeric.Nat.Arithmetic (add)

data Candidate : Type where {
  A : Candidate;
  B : Candidate;
  C : Candidate;
  D : Candidate
}

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two

fn seed_cost (x : Candidate) : Nat =
  match x {
    A ↦ two;
    B ↦ one;
    C ↦ two;
    D ↦ three
  }

const input : List Candidate =
  Cons Candidate A (Cons Candidate B (Cons Candidate C (Cons Candidate D (Nil Candidate))))

fn last_two (xs : List Candidate) : List Candidate =
  reverse Candidate (take Candidate two (reverse Candidate xs))

theorem v_least_two
    : Equal (List Candidate) (least_k Candidate seed_cost two input)
        (Cons Candidate B (Cons Candidate A (Nil Candidate))) =
  Refl

theorem v_least_three
    : Equal (List Candidate) (least_k Candidate seed_cost (add two one) input)
        (Cons Candidate B (Cons Candidate A (Cons Candidate C (Nil Candidate)))) =
  Refl

theorem v_last_two_of_three
    : Equal (List Candidate) (last_two (least_k Candidate seed_cost (add two one) input))
        (Cons Candidate A (Cons Candidate C (Nil Candidate))) =
  Refl

theorem c_closed_prefix
    : Equal (List Candidate) (least_k Candidate seed_cost two input)
        (take Candidate two (least_k Candidate seed_cost (add two one) input)) =
  Refl

theorem c_law_prefix
    : Equal (List Candidate) (least_k Candidate seed_cost two input)
        (take Candidate two (least_k Candidate seed_cost (add two one) input)) =
  least_k_prefix Candidate seed_cost two one input

theorem v_take_three_of_two : Equal (List Candidate) (take Candidate three (take Candidate two input))
    (Cons Candidate A (Cons Candidate B (Nil Candidate))) =
  Refl

theorem v_take_three
    : Equal (List Candidate) (take Candidate three input)
        (Cons Candidate A (Cons Candidate B (Cons Candidate C (Nil Candidate)))) =
  Refl

theorem t_closed_take
    : Equal (List Candidate) (take Candidate two (take Candidate three input)) (take Candidate two input) =
  Refl

theorem t_law_take
    : Equal (List Candidate) (take Candidate two (take Candidate three input)) (take Candidate two input) =
  (proof prefix for take) Candidate two three input Proved

theorem x_take_bound : Equal (List Candidate) (take Candidate three (take Candidate two input)) (take Candidate three input) = (proof prefix for take) Candidate three two input Proved
```

### stdlib/optimization/reject-take-prefix-without-valid-bound

- spec: §4.5, premise necessity for `take::prefix`.
- promise class: durable invariant.
- given: the fresh accepted block followed by X-BOUND with `k = 3`
  and `n = 2`.
- expect: the prefix accepts; X-BOUND is refused with
  `KernelRejected(TypeMismatch)` and a span enclosing its own `Proved`.
- measured: the D0 refusal is within the `x_take_bound` theorem and
  encloses the terminal `Proved`.
- claimed: `take::prefix` requires the smaller-or-equal bound premise.
- gap: this false bound is one closed example, not a characterization
  of every invalid bound.
- controls: `t_closed_take` and `t_law_take` accept the corresponding
  `2 ≤ 3` result; `v_take_three_of_two` and `v_take_three` fix the two
  false-claim endpoints independently.
- why: `take 3 (take 2 input)` is `[A, B]`; `take 3 input` is
  `[A, B, C]`, and `3 ≤ 2` is false.

## Evidence boundary

The D0 scratch run on `fd523a842465c0bc185140167143ad457d51bc37`
checked W-GEN, the accepted block, and the three negative rows with
fresh accepted prefixes. The D0 observed X-SAME and X-LAST refusing at
their own `Refl` with `ElabError::TypeMismatch`; X-BOUND refused with
`KernelRejected(TypeMismatch)` over its theorem, enclosing its
`Proved`. This does not constitute execution of this authored seed at
`a731359837ec650464b0d17f3f03d64088ec1609` or on an assembled
candidate. No official conformance run, package test, CI, publication,
landing, or measured landed trust is claimed.
