# Least-cost selection from a possibly empty list

Spec: `spec/50-stdlib/72-optimization.md` §1.4.

This seed specifies closed observations for `argmin_option` on an empty
list, a singleton, and a nonempty list with a tied least cost. The shared
tie fixture is `[3, 2, 0, 1]` with costs `[5, 1, 9, 1]`: the selected
`2` is neither the head `3` nor the later tied minimum `1`. The cases
distinguish a head-returning selector, a last-tie selector, and a
selector that returns `None` on nonempty input.

This candidate is based on `origin/main` at
`8546cb58cd50450f2b0a98ceadbde5e62fb5e07b`, where the §1.4 exports
are absent. The Ken fence is authored against the approved §1.4 contract,
not executable against that base. This is source content only; it makes
no shipped-seed execution, catalog delivery, CI, publication, or landing
claim. Architect's `ken check` and false-claim result are scratch D0
evidence (`evt_py40bd5dz90y`), not execution of this seed.

Each case has promise class **durable invariant**. Expected values are
independent closed constants; they are not computed by the selector.

## Shared fixtures and positive assertions

```ken
import Algorithm.Optimization.Argmin
  (argmin, argmin_option, argmin_option_nil, argmin_option_cons,
   argmin_option_member, argmin_option_least)

import Core.Classes.LawfulClasses (IsTrue, leq_nat)

import Data.Collections.Derived
  (list_elem, list_elem_head, list_elem_later)

import Data.Collections.NonEmpty (nonempty_cons)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const five : Nat = Suc (Suc (Suc (Suc (Suc Zero))))
const nine : Nat = Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc Zero))))))))

fn seed_cost (n : Nat) : Nat =
  match n {
    Zero ↦ nine;
    Suc m ↦
      match m {
        Zero ↦ one;
        Suc k ↦
          match k {
            Zero ↦ one;
            Suc j ↦ five
          }
      }
  }

const empty_input : List Nat = Nil Nat

const singleton_input : List Nat =
  Cons Nat three (Nil Nat)

const tie_input : List Nat =
  Cons Nat three
    (Cons Nat two (Cons Nat Zero (Cons Nat one (Nil Nat))))

theorem empty_returns_none
    : Equal (Option Nat)
        (argmin_option Nat seed_cost empty_input)
        (None Nat) =
  Proved

theorem singleton_returns_its_only_value
    : Equal (Option Nat)
        (argmin_option Nat seed_cost singleton_input)
        (Some Nat three) =
  Proved

theorem earliest_tied_minimum_is_selected
    : Equal (Option Nat)
        (argmin_option Nat seed_cost tie_input)
        (Some Nat two) =
  Proved

theorem nil_equation_instance
    : Equal (Option Nat)
        (argmin_option Nat seed_cost empty_input)
        (None Nat) =
  argmin_option_nil Nat seed_cost

theorem cons_equation_instance
    : Equal (Option Nat)
        (argmin_option Nat seed_cost
          (Cons Nat three (Cons Nat two (Nil Nat))))
        (Some Nat
          (argmin Nat seed_cost
            (nonempty_cons Nat three (Cons Nat two (Nil Nat))))) =
  argmin_option_cons Nat seed_cost three (Cons Nat two (Nil Nat))

theorem two_is_a_member_of_tie_input
    : list_elem Nat two tie_input =
  list_elem_later Nat two three
    (Cons Nat two (Cons Nat Zero (Cons Nat one (Nil Nat))))
    (list_elem_head Nat two
      (Cons Nat Zero (Cons Nat one (Nil Nat))))

theorem selected_value_has_input_membership
    : list_elem Nat two tie_input =
  argmin_option_member Nat seed_cost tie_input two Proved

theorem selected_cost_is_least_against_the_head
    : IsTrue (leq_nat (seed_cost two) (seed_cost three)) =
  argmin_option_least Nat seed_cost tie_input two Proved three
    (list_elem_head Nat three
      (Cons Nat two (Cons Nat Zero (Cons Nat one (Nil Nat)))))
```

The singleton and tied cases have `Some` results, while the empty case
has `None`. In the tied fixture, candidate `2` has cost `1`; candidate
`3` has cost `5`, candidate `0` has cost `9`, and candidate `1` has
cost `1`. The membership and least-cost instances exercise the direct
`List` corollaries under an observed `Some two` result.

## Closed operation cases

### stdlib/optimization/argmin-option-nil

- spec: §1.4.
- promise class: durable invariant.
- given: `argmin_option` on `Nil`.
- expect: `None`.
- why: the empty list has no candidate to select.

### stdlib/optimization/argmin-option-singleton

- spec: §1.4.
- promise class: durable invariant.
- given: the singleton list `[3]` under `seed_cost`.
- expect: `Some 3`.
- why: the sole candidate is returned; `None` on nonempty input fails.

### stdlib/optimization/argmin-option-earliest-tied-minimum

- spec: §1.4 and §1.2.
- promise class: durable invariant.
- given: `[3, 2, 0, 1]` with costs `[5, 1, 9, 1]`.
- expect: `Some 2`.
- why: candidate `2` is the earliest minimum, not the head or last
  tied minimum. A head-returning implementation yields `Some 3`; a
  last-tie implementation yields `Some 1`; a selector returning `None`
  for nonempty input yields `None`. Each differs from the expected result.

These are closed behavioral observations of the same list/cost input.
They do not infer selection behavior from the generic corollary proofs.

## Closed instances of the public laws

- `argmin-option-nil-equation` instantiates the exact `Nil` equation.
- `argmin-option-cons-equation` instantiates the exact `Cons` equation,
  naming `argmin` of the public `nonempty_cons` construction.
- `argmin-option-member` instantiates the membership corollary for the
  selected `2` in the tied fixture.
- `argmin-option-least` compares the selected cost against member `3`,
  whose cost is `5` while the selected cost is `1`.

These are closed instances of the public laws, not new proofs of their
generic statements. The four helpers described in §1.4 are not imported
or asserted as public declarations.

## Refused claim

The expected-rejection block is evaluated separately against the shared
positive prelude. Its positive control, `empty_returns_none`, uses the
same empty input. A rejection by itself would not distinguish a false
claim from a broken fixture or an unavailable export.

### stdlib/optimization/argmin-option-rejects-some-on-nil

- spec: §1.4.
- promise class: durable invariant.
- given: claim that `argmin_option` returns `Some 2` on `Nil`.
- expect: reject the equality; the same input is accepted as `None` by
  `empty_returns_none`.
- why: the correct implementation returns `None`; a bug returning
  `Some` on `Nil` would make this claim true. The correct and buggy
  outcomes are different.

```ken
-- Refused: Nil yields None, not Some.
theorem seed_neg
    : Equal (Option Nat)
        (argmin_option Nat seed_cost empty_input)
        (Some Nat two) =
  Proved
```

Architect reports this false-claim shape refused with E0038 in D0
scratch (`evt_py40bd5dz90y`). Because the §1.4 exports are absent from
this seed's base, that report is not shipped-seed execution.

## Coverage boundary

The seed covers the required empty, singleton, and non-degenerate tied
minimum cases. The tied result differs from both the head and the last
tied minimum, and the positive nonempty cases reject a `None`-on-nonempty
selector. It also instantiates each of the four public §1.4 laws. The
refused `Some`-on-`Nil` claim is paired with an accepted observation of
`None` on the identical input. No `DecEq` on the candidate type, order on
that type, new search strategy, or complexity guarantee is asserted. The
seed does not claim the §1.4 catalog exports are delivered, the source
fences were executed, CI passed, or the candidate was published or landed.
