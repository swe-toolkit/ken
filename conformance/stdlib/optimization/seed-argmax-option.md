# Greatest-cost selection from a possibly empty list

Spec: `spec/50-stdlib/72-optimization.md` §3.4.
Frame: `docs/program/issues/SPEC-OPTIMIZATION-ARGMAX-OPTION.md`.

This authored seed states closed observations for `argmax_option` on `Nil`,
a singleton and a tied-maximum input. The §3.4 exports are absent from the
landed base at authoring, so the checks reported below use an isolated
overlay of Architect's ruled D0 development, not shipped package exports.
No official conformance-runner, CI, publication, landing or shipped-seed
execution is claimed. Each case has promise class **durable invariant**.

## Shared fixtures and accepted assertions

The closed cost fixture assigns `5` to `3`, `9` to `2`, `1` to `0` and
`9` to `1`. On `[3, 2, 0, 1]`, the earliest greatest-cost element is
`2`: it is not the head, ties with later `1` and differs from the
least-cost element `0`. Expected outputs are independent closed `Nat`
constants, not results computed by `argmax_option`.

```ken
import Algorithm.Optimization.Argmin
  (argmax, argmax_option, argmax_option_nil, argmax_option_cons,
   argmax_option_member, argmax_option_greatest, argmin_option)

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
    Zero ↦ one;
    Suc m ↦
      match m {
        Zero ↦ nine;
        Suc k ↦
          match k {
            Zero ↦ nine;
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
        (argmax_option Nat seed_cost empty_input)
        (None Nat) =
  Proved

theorem singleton_returns_its_only_value
    : Equal (Option Nat)
        (argmax_option Nat seed_cost singleton_input)
        (Some Nat three) =
  Proved

theorem earliest_tied_maximum_is_selected
    : Equal (Option Nat)
        (argmax_option Nat seed_cost tie_input)
        (Some Nat two) =
  Proved

theorem same_input_minimum_is_distinct
    : Equal (Option Nat)
        (argmin_option Nat seed_cost tie_input)
        (Some Nat Zero) =
  Proved

theorem nil_equation_instance
    : Equal (Option Nat)
        (argmax_option Nat seed_cost empty_input)
        (None Nat) =
  argmax_option_nil Nat seed_cost

theorem cons_equation_instance
    : Equal (Option Nat)
        (argmax_option Nat seed_cost tie_input)
        (Some Nat
          (argmax Nat seed_cost
            (nonempty_cons Nat three
              (Cons Nat two (Cons Nat Zero (Cons Nat one (Nil Nat))))))) =
  argmax_option_cons Nat seed_cost three
    (Cons Nat two (Cons Nat Zero (Cons Nat one (Nil Nat))))

theorem two_is_a_member_of_tie_input
    : list_elem Nat two tie_input =
  list_elem_later Nat two three
    (Cons Nat two (Cons Nat Zero (Cons Nat one (Nil Nat))))
    (list_elem_head Nat two
      (Cons Nat Zero (Cons Nat one (Nil Nat))))

theorem selected_value_has_input_membership
    : list_elem Nat two tie_input =
  argmax_option_member Nat seed_cost tie_input two Proved

theorem selected_cost_is_greatest_against_later_tie
    : IsTrue (leq_nat (seed_cost one) (seed_cost two)) =
  argmax_option_greatest Nat seed_cost tie_input two Proved one
    (list_elem_later Nat one three
      (Cons Nat two (Cons Nat Zero (Cons Nat one (Nil Nat))))
      (list_elem_later Nat one two
        (Cons Nat Zero (Cons Nat one (Nil Nat)))
        (list_elem_later Nat one Zero
          (Cons Nat one (Nil Nat))
          (list_elem_head Nat one (Nil Nat)))))
```

The accepted `None` and `Some` observations are controls on their exact
operands before any refusal is attempted. The tie output and the
separate `argmin_option` output discriminate greatest from least on
the same list. The four equation/corollary instances use the ruled
public interfaces; membership and greatest cost are stated directly
on `List`, with greatest cost tested against the later tied member.

### stdlib/optimization/argmax-option-nil

- spec: §3.4.
- promise class: durable invariant.
- given: `argmax_option` on `Nil`.
- expect: `None`.
- why: an empty list supplies no selected candidate.

### stdlib/optimization/argmax-option-singleton

- spec: §3.4.
- promise class: durable invariant.
- given: `[3]` under `seed_cost`.
- expect: `Some 3`, not `None`.
- why: the only candidate is the maximum.

### stdlib/optimization/argmax-option-earliest-tied-maximum

- spec: §3.1 and §3.4.
- promise class: durable invariant.
- given: `[3, 2, 0, 1]` with costs `[5, 9, 1, 9]`.
- expect: `Some 2`, the earlier tied maximum.
- why: it differs from the head `3`, the later tie `1`, the
  least-cost `0` and `None`.

## Refused claims on controlled operands

Runner assembly for **each** refusal: first build and accept a client
containing the entire positive Ken fence above. Copy that accepted
client fresh, then append **only one** of the following Ken fences.
A refusal passes only when its false equality is rejected at that
appended declaration. Rejection caused by a missing import, fixture,
§3.4 export, or an earlier positive declaration is not a pass.

### stdlib/optimization/argmax-option-rejects-some-on-nil

- spec: §3.4.
- promise class: durable invariant.
- given: accepted `empty_returns_none` on the same empty input.
- expect: reject the false `Some 2` result.
- why: `Nil` returns `None`.

```ken
theorem refuse_some_on_nil
    : Equal (Option Nat)
        (argmax_option Nat seed_cost empty_input)
        (Some Nat two) =
  Proved
```

### stdlib/optimization/argmax-option-rejects-none-on-nonempty

- spec: §3.4.
- promise class: durable invariant.
- given: accepted `earliest_tied_maximum_is_selected` on `tie_input`.
- expect: reject the false `None` result.
- why: the non-empty input returns `Some two`.

```ken
theorem refuse_none_on_nonempty
    : Equal (Option Nat)
        (argmax_option Nat seed_cost tie_input)
        (None Nat) =
  Proved
```

### stdlib/optimization/argmax-option-rejects-head

- spec: §3.4.
- promise class: durable invariant.
- given: accepted `earliest_tied_maximum_is_selected` on `tie_input`.
- expect: reject `Some 3`, the head rather than the maximum.
- why: the head's cost is `5`, below `2`'s cost of `9`.

```ken
theorem refuse_head
    : Equal (Option Nat)
        (argmax_option Nat seed_cost tie_input)
        (Some Nat three) =
  Proved
```

### stdlib/optimization/argmax-option-rejects-last-tie

- spec: §3.1 and §3.4.
- promise class: durable invariant.
- given: accepted `earliest_tied_maximum_is_selected` on `tie_input`.
- expect: reject `Some 1`, the later tied maximum.
- why: `2` precedes `1` and both cost `9`.

```ken
theorem refuse_last_tie
    : Equal (Option Nat)
        (argmax_option Nat seed_cost tie_input)
        (Some Nat one) =
  Proved
```

### stdlib/optimization/argmax-option-rejects-least

- spec: §3.4.
- promise class: durable invariant.
- given: accepted `earliest_tied_maximum_is_selected` on `tie_input`
  and `same_input_minimum_is_distinct` for the same cost and list.
- expect: reject `Some 0`, the least-cost member.
- why: `0` costs `1`, below the maximum cost of `9`.

```ken
theorem refuse_least
    : Equal (Option Nat)
        (argmax_option Nat seed_cost tie_input)
        (Some Nat Zero) =
  Proved
```

## Coverage boundary and author check

These five refusals, paired with accepted same-input controls, detect
the specified wrong outcomes but do not replace the four general law
proofs. On an isolated scratch worktree based on `80beabbd3` with
Architect's ruled §3.4 D0 patch applied, `ken check` of the entire
accepted Ken fence returned rc 0. Five fresh copies of that positive
source, each with exactly one refusal appended, returned rc 1/K0004
`KernelRejected(TypeMismatch)` at the appended claim, not at an
import, fixture or earlier declaration. The positive and negative
sources were loaded as temporary `catalog/packages/` clients so their
imports resolved; their Ken fences came from this exact seed. The CLI
binary was built in Architect's D0 scratch tree from `914cfbda0`,
whose difference from the checked base affects docs and agent material,
not `crates/` or `catalog/`. This is a local overlay check, not
execution against landed §3.4 exports or by the official conformance
runner. It supplies no runtime-cost, arbitrary candidate order, or
tie-position witness claim.
