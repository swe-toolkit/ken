# Least witness below a finite bound

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §2.

These authored behavioral oracles target the prospective
`Algorithm.Optimization.BoundedSearch` module. They cover the operation,
one closed instance of each of its three laws, a missing result, the
empty range, and false claims for a later witness and for other incorrect
results. Each case has promise class **durable invariant**. They do not
specify search direction, recursion shape, predicate-call count, early
exit, or a resource bound.

Architect D0 `evt_5mbgykn93snzc` reports its generic development and D0
client seeds checked in scratch. The separate cost correction
`evt_1sexdt34d5vet` retracts the D0's erroneous predicate-call claim;
the landed contract is extensional and states no cost promise. CV also
checked this seed's shared positive block (rc 0) and each false-claim
block alone (rc 1, K0004 `TypeMismatch`) against a temporary scratch
module made from the D0 development. This is not a shipped catalog-module
check. No catalog delivery, CI, publication, or measured
`trusted_base()` delta is claimed.

## Shared fixtures and positive assertions

The named predicate is true exactly above two. The closed inputs use
independent `Nat` constants, so the expected `Some` value is not computed
by `find_below` itself.

```ken
import Algorithm.Optimization.BoundedSearch
  (find_below, find_below_sound, find_below_least, find_below_complete)

import Data.Numeric.Nat.Order (lt_nat)

const two : Nat = Suc (Suc Zero)
const three : Nat = Suc two
const five : Nat = Suc (Suc three)
const six : Nat = Suc five
const ten : Nat = Suc (Suc (Suc (Suc (Suc five))))

fn above_two (n : Nat) : Bool = lt_nat two n

theorem found_least_witness
    : Equal (Option Nat) (find_below above_two ten) (Some Nat three) =
  Proved

theorem found_least_witness_below_six
    : Equal (Option Nat) (find_below above_two six) (Some Nat three) =
  Proved

theorem no_witness_below_three
    : Equal (Option Nat) (find_below above_two three) (None Nat) =
  Proved

theorem empty_bound_has_no_witness
    : Equal (Option Nat) (find_below above_two Zero) (None Nat) =
  Proved

theorem sound_instance
    : And (Equal Bool (lt_nat three ten) True)
        (Equal Bool (above_two three) True) =
  find_below_sound above_two ten three Proved

theorem least_instance
    : Equal Bool (above_two two) False =
  find_below_least above_two ten three Proved two Proved

theorem complete_instance
    : Equal Bool (above_two two) False =
  find_below_complete above_two three Proved two Proved
```

### The operation and finite-range boundaries

#### stdlib/optimization/bounded-search-finds-least-witness

- spec: §2.1.
- promise class: durable invariant.
- given: `above_two` below bound `10`.
- expect: `Some 3`, the least satisfying index.
- why: `3` is a hit below `10`; `5` is also a hit but is not least.

#### stdlib/optimization/bounded-search-later-witness-control

- spec: §2.1.
- promise class: durable invariant.
- given: `above_two` below bound `6`.
- expect: `Some 3`; the later hit `5` is not returned.
- why: a latest-witness search returns `5` on this same input, so the
  expected result distinguishes the two behaviors.

#### stdlib/optimization/bounded-search-reports-none

- spec: §2.1.
- promise class: durable invariant.
- given: `above_two` below bound `3`.
- expect: `None`; the range contains only `0`, `1`, and `2`.
- why: none of those values satisfies `2 < n`.

#### stdlib/optimization/bounded-search-empty-range

- spec: §2.1.
- promise class: durable invariant.
- given: `above_two` below bound `Zero`.
- expect: `None` for the empty range.
- why: no natural number is strictly below `Zero`.

### Closed instances of the three laws

#### stdlib/optimization/bounded-search-sound

- spec: §2.2.
- promise class: durable invariant.
- given: the result `Some 3` below `10`.
- expect: both `3 < 10` and `above_two 3 = True`.
- why: this instance exercises both conjuncts of `find_below_sound`.

#### stdlib/optimization/bounded-search-least

- spec: §2.2.
- promise class: durable invariant.
- given: the result `Some 3` below `10` and earlier candidate `2`.
- expect: `above_two 2 = False`.
- why: this is a concrete earlier candidate for `find_below_least`.

#### stdlib/optimization/bounded-search-complete

- spec: §2.2.
- promise class: durable invariant.
- given: `None` below `3` and in-range candidate `2`.
- expect: `above_two 2 = False`.
- why: this candidate exercises `find_below_complete`'s no-hit result.

These are closed instances of the public generic laws, not new proofs of
their universal statements. The `Option Nat` equations compute to their
expected results and use `Proved`; the law instances use their theorem
premises for the computed result equations.

## Refused claims

Each declaration below is an individual expected-rejection case appended
to the unchanged positive prelude, one at a time. The listed positive
case on the identical input is its acceptance control. A rejection alone
would not distinguish a false result from a broken fixture or harness.

### A later witness is not the least result

- spec: §2.1–§2.2.
- promise class: durable invariant.
- given: claim that the result below `6` is `Some 5`.
- expect: refuse the claim; `found_least_witness_below_six` accepts on
  this identical input.
- why: the earlier satisfying witness `3` beats `5`. A latest-witness
  search returns `5` here, making this false claim true.

```ken
-- Refused: 5 is a hit, but the smaller hit 3 is returned.
theorem reject_later_witness
    : Equal (Option Nat) (find_below above_two six) (Some Nat five) =
  Proved
```

### A hit cannot be reported as absent

- spec: §2.1–§2.2.
- promise class: durable invariant.
- given: claim that the result below `10` is `None`.
- expect: refuse the claim; `found_least_witness` accepts on this
  identical input.
- why: `3` is a satisfying witness below `10`. A never-finds
  implementation would make the false claim true.

```ken
-- Refused: 3 satisfies the predicate below 10.
theorem reject_none_when_witness_exists
    : Equal (Option Nat) (find_below above_two ten) (None Nat) =
  Proved
```

### A non-hit is not a witness

- spec: §2.1–§2.2.
- promise class: durable invariant.
- given: claim that the result below `3` is `Some 2`.
- expect: refuse the claim; `no_witness_below_three` accepts on this
  identical input.
- why: `above_two 2` is `False`, so `2` is not a witness.

```ken
-- Refused: 2 is in range but does not satisfy the predicate.
theorem reject_non_hit
    : Equal (Option Nat) (find_below above_two three) (Some Nat two) =
  Proved
```

## Coverage boundary

The seed covers the operation, each of the three generic laws, an
ordinary `None` result, the empty bound, a later-witness refusal, a
false `None` claim, and a false-hit claim. Each refusal has a positive
control on the same input; the later-witness pair makes the latest-witness
bug return `Some 5` instead of `Some 3`. The finite fixtures do not
assert a search strategy, predicate-call count, or complexity bound,
and do not add §1's deferred `Option` selector over a possibly empty
list. No shipped catalog-module check, CI, publication, or landing is
claimed.
