# Least-cost candidate selection

Spec: `spec/50-stdlib/72-optimization.md` §1.

These are authored behavioral oracles for the prospective
`Algorithm.Optimization.Argmin` module. The cases use the D0's closed
fixtures and the exact seven interfaces in §1.2. They do not claim a
shipped package or executed seed. Architect-reported D0 checks are
attributed to the Architect; they are not independent execution of this
seed. Each case has promise class **durable invariant**.

## Shared fixtures and law instances

The finite cost fixture assigns costs 5, 1, 9 and 1 to candidates 3, 2,
0 and 1 respectively. The expected outputs below are independently
named `Nat` constants, not values computed by `argmin`. All `NonEmpty`
values use its public constructors. Membership proofs use the existing
Ω-valued `list_elem`; no equality decision on the candidate type is
introduced.

```ken
import Algorithm.Optimization.Argmin
  (argmin, argmin_member, argmin_least, argmin_no_cheaper,
   argmin_head_on_tie, argmin_singleton,
   argmin_keeps_head, argmin_takes_tail)

import Core.Classes.LawfulClasses (IsTrue, leq_nat)

import Data.Collections.Derived
  (list_elem, list_elem_head, list_elem_later)

import Data.Collections.NonEmpty
  (NonEmpty, nonempty_cons, nonempty_singleton, nonempty_to_list,
   nonempty_to_list_shape)

import Data.Numeric.Nat.Order (lt_nat)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const five : Nat = Suc (Suc (Suc (Suc (Suc Zero))))
const nine : Nat = Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc Zero)))))))

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

fn flat_cost (n : Nat) : Nat = one

const strict_input : NonEmpty Nat =
  nonempty_cons Nat three (Cons Nat Zero (Cons Nat one (Nil Nat)))

const tie_input : NonEmpty Nat =
  nonempty_cons Nat three
    (Cons Nat two (Cons Nat Zero (Cons Nat one (Nil Nat))))

theorem seed_singleton_value
    : Equal Nat (argmin Nat seed_cost (nonempty_singleton Nat three)) three =
  Proved

theorem seed_strict_value
    : Equal Nat (argmin Nat seed_cost strict_input) one =
  Proved

theorem seed_tie_earliest
    : Equal Nat (argmin Nat seed_cost tie_input) two =
  Proved

theorem tie_two_member
    : list_elem Nat two (nonempty_to_list Nat tie_input) =
  J
    (λl _. list_elem Nat two l
      → list_elem Nat two (nonempty_to_list Nat tie_input))
    (λmember. member)
    (nonempty_to_list_shape Nat tie_input)
    (list_elem_later Nat two three
      (Cons Nat two (Cons Nat Zero (Cons Nat one (Nil Nat))))
      (list_elem_head Nat two
        (Cons Nat Zero (Cons Nat one (Nil Nat)))))

theorem seed_law_member
    : list_elem Nat
        (argmin Nat seed_cost tie_input)
        (nonempty_to_list Nat tie_input) =
  argmin_member Nat seed_cost tie_input

theorem seed_law_least
    : IsTrue
        (leq_nat
          (seed_cost (argmin Nat seed_cost tie_input))
          (seed_cost two)) =
  argmin_least Nat seed_cost tie_input two tie_two_member

theorem seed_law_no_cheaper
    : Equal Bool
        (lt_nat (seed_cost two) (seed_cost (argmin Nat seed_cost tie_input)))
        False =
  argmin_no_cheaper Nat seed_cost tie_input two tie_two_member

theorem seed_law_head_on_tie
    : Equal Nat
        (argmin Nat flat_cost
          (nonempty_cons Nat two (Cons Nat one (Nil Nat))))
        two =
  argmin_head_on_tie Nat flat_cost two
    (Cons Nat one (Nil Nat))
    (λy. λmember. (proof refl for leq_nat) one)

theorem seed_law_singleton
    : Equal Nat
        (argmin Nat seed_cost (nonempty_singleton Nat three))
        three =
  argmin_singleton Nat seed_cost three

theorem seed_law_keeps_head
    : Equal Nat
        (argmin Nat seed_cost
          (nonempty_cons Nat one (Cons Nat three (Nil Nat))))
        one =
  argmin_keeps_head Nat seed_cost one three (Nil Nat) Refl

theorem seed_law_takes_tail
    : Equal Nat
        (argmin Nat seed_cost
          (nonempty_cons Nat three (Cons Nat one (Nil Nat))))
        (argmin Nat seed_cost (nonempty_cons Nat one (Nil Nat))) =
  argmin_takes_tail Nat seed_cost three one (Nil Nat) Refl
```

The `argmin_least` and `argmin_no_cheaper` instances use candidate `2`,
whose membership in the ordered input is constructed by `tie_two_member`.
The `argmin_head_on_tie` instance has a nonempty tail and equal costs for
its head and tail; its bound premise holds for every tail member because
`flat_cost` is constant. The two branch-law instances exercise distinct
comparison outcomes: candidate `1` keeps the head over candidate `3`, and
candidate `1` replaces candidate `3` when it is the cheaper tail result.

## Closed operation cases

### stdlib/optimization/argmin-singleton

- spec: §1.1–§1.2.
- promise class: durable invariant.
- given: the singleton non-empty input `[3]` under `seed_cost`.
- expect: `argmin` returns `3`.
- why: the operation returns its sole candidate.

### stdlib/optimization/argmin-strict-minimum

- spec: §1.1–§1.2.
- promise class: durable invariant.
- given: `[3, 0, 1]`, with costs `5, 9, 1`.
- expect: `argmin` returns `1`, the unique minimum.
- why: an always-head implementation would return `3`, and a
  reversed-order argmax implementation would return `0`; both differ
  from the expected `1`.

### stdlib/optimization/argmin-earliest-tie

- spec: §1.1–§1.2.
- promise class: durable invariant.
- given: `[3, 2, 0, 1]`, with costs `5, 1, 9, 1`.
- expect: `argmin` returns `2`, not the later tied candidate `1`.
- why: a latest-tie implementation would return `1` on this same input,
  while the specified selector returns `2`. The three cost laws alone
  do not distinguish those selectors.

## Closed instances of the seven laws

- `stdlib/optimization/argmin-member` instantiates `argmin_member` on
  `tie_input`; the result must have truncated membership in its input.
- `stdlib/optimization/argmin-least` instantiates `argmin_least` on
  `tie_input` against member `2`; the expected bound is cost `1` ≤ cost
  `1`. This witness also rejects an output whose cost exceeds member `2`.
- `stdlib/optimization/argmin-no-cheaper` instantiates
  `argmin_no_cheaper` on member `2`; the expected strict comparison is
  `False`.
- `stdlib/optimization/argmin-head-on-tie` instantiates
  `argmin_head_on_tie` with head `2` and a nonempty tail containing `1`
  under `flat_cost`; the equal-cost tail does not replace the head.
- `stdlib/optimization/argmin-singleton-law` instantiates
  `argmin_singleton` on candidate `3`.
- `stdlib/optimization/argmin-keeps-head` instantiates
  `argmin_keeps_head` with head `1`, tail `[3]`, and a false
  tail-versus-head strict comparison.
- `stdlib/optimization/argmin-takes-tail` instantiates
  `argmin_takes_tail` with head `3`, tail `[1]`, and a true
  tail-versus-head strict comparison.

These are closed instances of the seven theorem interfaces, not new proofs
of their generic statements and not evidence that the catalog package has
been delivered.

## Refused claims

Each declaration below is an individual expected-rejection input appended
to the shared positive prelude, with only one `seed_neg` present at a
time. The positive `seed_tie_earliest` uses the same input and is the
acceptance control for both expected-rejection cases. Architect reported the
corresponding D0 negatives as individually rejected on the scratch
module; this seed's declarations were not run.

### stdlib/optimization/argmin-rejects-later-tie

- spec: §1.2.
- promise class: durable invariant.
- given: claim that `argmin` on `tie_input` returns `1`.
- expect: reject the claimed equality; the result is the earlier tied
  candidate `2`.
- why: correct behavior returns `2` and refuses the claim; a
  latest-tie implementation would return `1`, making the claim true.

```ken
theorem seed_neg
    : Equal Nat (argmin Nat seed_cost tie_input) one =
  Proved
```

### stdlib/optimization/argmin-rejects-beaten-candidate

- spec: §1.2.
- promise class: durable invariant.
- given: claim that `argmin` on `tie_input` returns `3`.
- expect: reject the claimed equality; member `2` has lower cost than
  `3`.
- why: correct behavior returns `2` and refuses the claim; an
  always-head implementation would return `3`, making the claim true.
  This is the frame's required refusal, distinct from the later-tie case.

```ken
theorem seed_neg
    : Equal Nat (argmin Nat seed_cost tie_input) three =
  Proved
```

## Coverage boundary

The seed has direct closed operation cases for a singleton, a unique
minimum and a non-degenerate earliest tie, plus one closed instance of
each of the seven public laws. The beaten-candidate refusal is independent
of the later-tie refusal, so a cost-correct but latest-tie selector cannot
pass by satisfying only the cost observations. Cases use no `DecEq a`,
list position witness, or complexity claim. They do not cover real-valued
or continuous optimization, search over infinite or implicit spaces,
heuristics or approximation guarantees, `argmax`, arbitrary `Ord` costs,
multi-objective or lexicographic costs, top-k or stable multi-result
selection, or an `Option` selector over a possibly empty list; §1.3
defers those behaviors. No catalog build, seed execution, CI, publication,
or landing is claimed.