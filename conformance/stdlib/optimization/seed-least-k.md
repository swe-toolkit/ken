# Stable selection of the k least-cost candidates

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §4.

This successor follows §4 in the separate reviewed spec child
`8ccb7a392b96350381e0d6bccb69cc800112160e`. Its base is
`719148a76b5b4e3ae929c3e852b67313b73d97d1`, which does not yet contain
§4, the `least_k` export, or public `take`. The source fences have not
been checked against the package or run by an official conformance runner.
The `MEASURED` labels describe assertions encoded in source, not executed
results. Architect's D0 measurements remain scratch evidence.

The input is `[A, B, C, D]` with costs `2, 1, 2, 3`. `Candidate` is a
local four-constructor carrier; this client declares no candidate `Eq`,
`DecEq`, or order instance, and performs no candidate equality decision.
The outputs distinguish candidates through costs and stable selection.
Stable cost order is `[B, A, C, D]`. The accepted cases preserve the D0
selector results and instantiate all eight public §4.2 laws. Two isolated
refusals distinguish skipping a cheaper candidate from selecting a later
equal-cost candidate.

## Accepted client

The outputs are written independently of `least_k`. The empty-list
comparison uses `Proved`; non-empty list comparisons use `Refl`, as
required by §4.3. The public-law clients use the same `Candidate` fixture.

```ken
import Algorithm.Optimization.Argmin
  (argmin,
    cost_le,
    least_k,
    least_k_length,
    least_k_member,
    least_k_sorted,
    least_k_ties_earliest,
    least_k_cheaper_class_selected,
    least_k_separation,
    least_k_one_nil,
    least_k_one_cons)

import Core.Classes.LawfulClasses (leq_nat)

import Core.Logic.Or (Inl, Inr)

import Data.Collections.Derived
  (eq_from_ord, filter, is_sorted as derived_sorted, length, list_elem, list_elem_head, list_elem_later, take)

import Data.Collections.NonEmpty (nonempty_cons)

import Data.Numeric.Nat.Order (lt_nat, min)

data Candidate : Type where {
  A : Candidate;
  B : Candidate;
  C : Candidate;
  D : Candidate
}

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

const nine : Nat = Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc Zero))))))))

fn seed_cost (x : Candidate) : Nat =
  match x {
    A ↦ two;
    B ↦ one;
    C ↦ two;
    D ↦ three
  }

const input : List Candidate =
  Cons Candidate A (Cons Candidate B (Cons Candidate C (Cons Candidate D (Nil Candidate))))

theorem candidate_a_cost_is_two : Equal Nat (seed_cost A) two = Proved

theorem candidate_b_cost_is_one : Equal Nat (seed_cost B) one = Proved

theorem candidate_c_cost_is_two : Equal Nat (seed_cost C) two = Proved

theorem candidate_d_cost_is_three : Equal Nat (seed_cost D) three = Proved

theorem zero_selects_empty
    : Equal (List Candidate) (least_k Candidate seed_cost Zero input) (Nil Candidate) =
  Proved

theorem two_selects_cheapest_then_earliest_tie
    : Equal
        (List Candidate)
        (least_k Candidate seed_cost two input)
        (Cons Candidate B (Cons Candidate A (Nil Candidate))) =
  Refl

theorem three_selects_both_tied_candidates_in_input_order
    : Equal
        (List Candidate)
        (least_k Candidate seed_cost three input)
        (Cons Candidate B (Cons Candidate A (Cons Candidate C (Nil Candidate)))) =
  Refl

theorem oversized_count_selects_the_full_sorted_list
    : Equal
        (List Candidate)
        (least_k Candidate seed_cost nine input)
        (Cons
          Candidate
          B
          (Cons Candidate A (Cons Candidate C (Cons Candidate D (Nil Candidate))))) =
  Refl

theorem selected_a_is_a_member : list_elem Candidate A (least_k Candidate seed_cost two input) =
  list_elem_later
    Candidate
    A
    B
    (Cons Candidate A (Nil Candidate))
    (list_elem_head Candidate A (Nil Candidate))

theorem input_b_is_a_member : list_elem Candidate B input =
  list_elem_later
    Candidate
    B
    A
    (Cons Candidate B (Cons Candidate C (Cons Candidate D (Nil Candidate))))
    (list_elem_head Candidate B (Cons Candidate C (Cons Candidate D (Nil Candidate))))

theorem input_d_is_a_member : list_elem Candidate D input =
  list_elem_later
    Candidate
    D
    A
    (Cons Candidate B (Cons Candidate C (Cons Candidate D (Nil Candidate))))
    (list_elem_later
      Candidate
      D
      B
      (Cons Candidate C (Cons Candidate D (Nil Candidate)))
      (list_elem_later
        Candidate
        D
        C
        (Cons Candidate D (Nil Candidate))
        (list_elem_head Candidate D (Nil Candidate))))

theorem d_is_absent_from_the_selected_prefix
    : list_elem Candidate D (least_k Candidate seed_cost two input) → Bottom =
  λmember.
    elim_trunc
      Bottom
      (λcase.
        match case {
          Inl d_eq_b ↦ d_eq_b;
          Inr later ↦
            elim_trunc
              Bottom
              (λtail_case.
                match tail_case {
                  Inl d_eq_a ↦ d_eq_a;
                  Inr impossible ↦ impossible
                })
              later
        })
      member

theorem least_k_length_client
    : Equal Nat
        (length Candidate (least_k Candidate seed_cost two input))
        (min two (length Candidate input)) =
  least_k_length Candidate seed_cost two input

theorem least_k_member_client : list_elem Candidate A input =
  least_k_member Candidate seed_cost two input A selected_a_is_a_member

theorem least_k_sorted_client
    : derived_sorted Candidate
        (cost_le Candidate seed_cost)
        (least_k Candidate seed_cost two input) =
  least_k_sorted Candidate seed_cost two input

theorem least_k_ties_earliest_client
    : Equal
        (List Candidate)
        (filter
          Candidate
          (eq_from_ord Candidate (cost_le Candidate seed_cost) A)
          (least_k Candidate seed_cost two input))
        (take
          Candidate
          (length
            Candidate
            (filter
              Candidate
              (eq_from_ord Candidate (cost_le Candidate seed_cost) A)
              (least_k Candidate seed_cost two input)))
          (filter Candidate (eq_from_ord Candidate (cost_le Candidate seed_cost) A) input)) =
  least_k_ties_earliest Candidate seed_cost two input A

theorem least_k_cheaper_class_selected_client
    : Equal
        (List Candidate)
        (filter
          Candidate
          (eq_from_ord Candidate (cost_le Candidate seed_cost) B)
          (least_k Candidate seed_cost two input))
        (filter Candidate (eq_from_ord Candidate (cost_le Candidate seed_cost) B) input) =
  least_k_cheaper_class_selected
    Candidate
    seed_cost
    two
    input
    A
    B
    selected_a_is_a_member
    input_b_is_a_member
    Proved

theorem least_k_separation_client : IsTrue (leq_nat (seed_cost A) (seed_cost D)) =
  least_k_separation
    Candidate
    seed_cost
    two
    input
    A
    D
    selected_a_is_a_member
    input_d_is_a_member
    d_is_absent_from_the_selected_prefix

theorem least_k_one_nil_client
    : Equal
        (List Candidate)
        (least_k Candidate seed_cost (Suc Zero) (Nil Candidate))
        (Nil Candidate) =
  least_k_one_nil Candidate seed_cost

theorem least_k_one_cons_client
    : Equal
        (List Candidate)
        (least_k
          Candidate
          seed_cost
          (Suc Zero)
          (Cons Candidate A (Cons Candidate B (Nil Candidate))))
        (Cons
          Candidate
          (argmin
            Candidate
            seed_cost
            (nonempty_cons Candidate A (Cons Candidate B (Nil Candidate))))
          (Nil Candidate)) =
  least_k_one_cons Candidate seed_cost A (Cons Candidate B (Nil Candidate))
```

### `stdlib/optimization/least-k-closed-prefixes`

- spec: §4.1 and §4.3.
- promise class: durable invariant.
- given: distinct constructors `A`, `B`, `C`, and `D` with costs
  `2`, `1`, `2`, and `3`, in that input order.
- expect: `[]` at `k = 0`; `[B, A]` at `k = 2`; `[B, A, C]` at
  `k = 3`; and `[B, A, C, D]` at `k = 9`.
- MEASURED (source assertions): the client uses a carrier without a
  declared candidate equality/order instance, checks four cost values,
  and states the four selector results with the D0 proof forms.
- CLAIMED: these finite outputs agree with stable least-cost prefix
  selection, and the exercised operation needs no candidate equality
  or order instance.
- THE GAP: this fixture does not establish generic behavior for every
  candidate type, input, cost function, or count; it is not an execution
  result because the seed has not been run.
- why: the candidates tied at cost `2` expose the stable
  earliest-first order; the oversized count exposes the full sorted list.

## Public law clients

Each client below applies the named public theorem on the same
no-equality `Candidate` carrier. `MEASURED` describes the source-level
assertion; it is not a claim that a package test executed.

### `stdlib/optimization/least-k-length-client`

- spec: §4.2 `least_k_length`.
- promise class: durable invariant.
- given: the four-item input and `k = 2`.
- expect: the public law proves the result length equals
  `min 2 (length input)`.
- MEASURED (source assertion): the client applies
  `least_k_length` at the concrete input and count.
- CLAIMED: the operation's output length obeys the public bound here.
- THE GAP: this closed use does not prove the generic law.
- why: the expected length is fixed independently as two.

### `stdlib/optimization/least-k-member-client`

- spec: §4.2 `least_k_member`.
- promise class: durable invariant.
- given: `A` is a member of the selected `k = 2` result.
- expect: the public law transports that membership to the original
  input.
- MEASURED (source assertion): `selected_a_is_a_member` supplies the
  selected-result witness to `least_k_member`.
- CLAIMED: selection invents no candidate on this fixture.
- THE GAP: this instance does not prove generic membership preservation.
- why: the selected value is the second item in `[B, A]`.

### `stdlib/optimization/least-k-sorted-client`

- spec: §4.2 `least_k_sorted`.
- promise class: durable invariant.
- given: the four-item input and `k = 2`.
- expect: the public law proves the selected prefix is sorted by
  `cost_le`.
- MEASURED (source assertion): the client applies `least_k_sorted`
  directly to the closed selection.
- CLAIMED: the returned prefix satisfies the public sortedness predicate.
- THE GAP: this instance does not prove generic sortedness.
- why: `B` precedes `A` by cost, and both tied classes use input order.

### `stdlib/optimization/least-k-ties-earliest-client`

- spec: §4.2 `least_k_ties_earliest`.
- promise class: durable invariant.
- given: the cost-`2` class of the `k = 2` result, with `w = A`.
- expect: the filtered selected class equals the prefix of the input
  cost class of the same length.
- MEASURED (source assertion): the client applies the public law at
  `w = A` on the concrete input.
- CLAIMED: the returned tied class is an initial segment of its input
  class.
- THE GAP: the instance does not prove generic earliest-tie behavior.
- why: only `A`, not later tied `C`, is selected at `k = 2`.

### `stdlib/optimization/least-k-cheaper-class-client`

- spec: §4.2 `least_k_cheaper_class_selected`.
- promise class: durable invariant.
- given: selected `A` costs `2`; input member `B` costs `1`.
- expect: the public law equates the entire `B` cost class in the
  selected result with that class in the input.
- MEASURED (source assertion): the client supplies selected membership
  for `A`, input membership for `B`, and the closed strict-cost premise.
- CLAIMED: a strictly cheaper cost class is wholly retained here.
- THE GAP: this instance does not prove the generic class-selection law.
- why: `B` is the only input with its cost, and it is selected.

### `stdlib/optimization/least-k-separation-client`

- spec: §4.2 `least_k_separation`.
- promise class: durable invariant.
- given: selected `A` costs `2`; input member `D` costs `3` and is
  absent from the selected prefix.
- expect: given the explicit absence premise, the public law proves
  `leq_nat (seed_cost A) (seed_cost D)`.
- MEASURED (source assertion): the client applies the public law with
  closed membership witnesses and an absence premise at this input.
- CLAIMED: selected cost is bounded by this absent input cost.
- THE GAP: this instance does not prove the generic separation law.
- why: the checked conclusion is `2 ≤ 3` on the displayed fixture.

### `stdlib/optimization/least-k-one-nil-client`

- spec: §4.2 `least_k_one_nil`.
- promise class: durable invariant.
- given: `k = 1` and an empty input.
- expect: the public law proves the result is `Nil`.
- MEASURED (source assertion): the client applies `least_k_one_nil`
  at this concrete cost function.
- CLAIMED: the one-element bound preserves the empty-list result.
- THE GAP: this instance does not prove the generic constructor law.
- why: `least_k` on an empty list has no candidate to return.

### `stdlib/optimization/least-k-one-cons-client`

- spec: §4.2 `least_k_one_cons`.
- promise class: durable invariant.
- given: `k = 1` and input `[A, B]`.
- expect: the public law equates the result to the singleton containing
  `argmin` on the corresponding `nonempty_cons`.
- MEASURED (source assertion): the client applies `least_k_one_cons`
  to `A` and tail `[B]`.
- CLAIMED: the one-element selection agrees with `argmin` on this
  non-empty constructor case.
- THE GAP: this instance does not prove generic constructor agreement.
- why: the law connects the finite selector with the existing choice.

## Refuse skipping a cheaper candidate

For this refusal, start from a fresh copy of the complete accepted client
and append only the false theorem below. The accepted `k = 2` result is a
same-form control. Expected refusal is at this theorem's `Refl` proof,
not before the appended claim.

### `stdlib/optimization/reject-least-k-skips-cheaper-candidate`

- spec: §4.3 cheaper-class selection.
- promise class: durable invariant.
- given: the accepted `k = 2` result `[B, A]` on the same input.
- expect: reject `[B, D]` as the selected result.
- MEASURED (source assertion): the accepted prefix supplies the cost
  fixture and true `k = 2` equality; the false claim uses the same
  `Refl` form.
- CLAIMED: a selected cost-`3` candidate cannot displace the unselected
  cost-`2` candidate `A` when `k = 2`.
- THE GAP: this single counterexample does not characterize every
  violation of cheaper-class selection.
- why: the actual result is `[B, A]`; `[B, D]` skips a cheaper input.

```ken
theorem seed_neg
    : Equal
        (List Candidate)
        (least_k Candidate seed_cost two input)
        (Cons Candidate B (Cons Candidate D (Nil Candidate))) =
  Refl
```

## Refuse a later equal-cost candidate

Use a separate fresh copy of the complete accepted client and append only
the false theorem below. Expected refusal is at this theorem's `Refl`
proof, not at an import, fixture, or earlier control.

### `stdlib/optimization/reject-least-k-chooses-later-tie`

- spec: §4.3 earliest-tie selection.
- promise class: durable invariant.
- given: the accepted `k = 2` result `[B, A]` on the same input.
- expect: reject `[B, C]` as the selected result.
- MEASURED (source assertion): the accepted prefix supplies the cost
  fixture and true `k = 2` equality; the false claim uses the same
  `Refl` form.
- CLAIMED: `C`, the later cost-`2` occurrence, cannot replace `A`, the
  earlier cost-`2` occurrence, in the result.
- THE GAP: this single counterexample does not characterize every
  violation of stable ordering or tie handling.
- why: `[B, C]` has the same length and costs as `[B, A]`, but reverses
  the required input-order choice within the tied class.

```ken
theorem seed_neg
    : Equal
        (List Candidate)
        (least_k Candidate seed_cost two input)
        (Cons Candidate B (Cons Candidate C (Nil Candidate))) =
  Refl
```

## Coverage boundary

The seed covers the four finite selector results, a custom candidate
carrier with no declared equality or order instance, one closed client
for each of the eight public laws, and the two distinct `k = 2` failures
specified in §4.3. These finite cases do not prove the generic laws for
all candidate types, inputs, cost functions, or counts. The source fences
are not package-test or official conformance execution evidence. No CI,
publication, or landing claim is made.
