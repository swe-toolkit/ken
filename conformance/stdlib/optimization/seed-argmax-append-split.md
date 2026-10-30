# Greatest-cost selection over appended non-empty parts

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §3.6.
Frame: `docs/program/issues/SPEC-OPTIMIZATION-ARGMAX-APPEND-SPLIT.md`.

The split `0, 1 | 2, 3` has costs `2, 7 | 7, 1`, so both parts
select cost `7`. The append keeps label `1`, the earlier choice. In a
second case, label `2` costs `8`, so the right part is strictly more
costly and the append selects label `2`. These cases exercise both
public branches without proving the generic laws.

This seed is authored but unexecuted. It has not been Ken-checked or
run through the official conformance runner.

## Accepted client

The entire accepted client checks before the isolated refused claim.
Direct closed `Proved` results establish the fixture costs, comparison
premises, and each part's selected label independently of the append
laws. The public laws then establish the append selections under their
stated Boolean premises.

```ken
import Algorithm.Optimization.Argmin
  (argmax,
    argmax_append_keeps_left,
    argmax_append_takes_right)

import Data.Collections.NonEmpty
  (NonEmpty,
    nonempty_cons,
    nonempty_append)

import Data.Numeric.Nat.Order (lt_nat)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const seven : Nat = Suc (Suc (Suc (Suc (Suc (Suc (Suc Zero))))))
const eight : Nat = Suc seven

fn tied_cost (n : Nat) : Nat =
  match n {
    Zero ↦ two;
    Suc m ↦
      match m {
        Zero ↦ seven;
        Suc k ↦
          match k {
            Zero ↦ seven;
            Suc j ↦ one
          }
      }
  }

fn right_cost (n : Nat) : Nat =
  match n {
    Zero ↦ two;
    Suc m ↦
      match m {
        Zero ↦ seven;
        Suc k ↦
          match k {
            Zero ↦ eight;
            Suc j ↦ one
          }
      }
  }

const left_part : NonEmpty Nat =
  nonempty_cons Nat Zero (Cons Nat one (Nil Nat))

const right_part : NonEmpty Nat =
  nonempty_cons Nat two (Cons Nat three (Nil Nat))

const tied_append : NonEmpty Nat =
  nonempty_append Nat left_part right_part

theorem tied_cost_zero_is_two : Equal Nat (tied_cost Zero) two = Proved

theorem tied_cost_one_is_seven : Equal Nat (tied_cost one) seven = Proved

theorem tied_cost_two_is_seven : Equal Nat (tied_cost two) seven = Proved

theorem tied_cost_three_is_one : Equal Nat (tied_cost three) one = Proved

theorem tied_left_selects_one
    : Equal Nat (argmax Nat tied_cost left_part) one =
  Proved

theorem tied_right_selects_two
    : Equal Nat (argmax Nat tied_cost right_part) two =
  Proved

theorem tied_comparison_is_false
    : Equal Bool
        (lt_nat
          (tied_cost (argmax Nat tied_cost left_part))
          (tied_cost (argmax Nat tied_cost right_part)))
        False =
  Proved

theorem tied_append_keeps_left
    : Equal Nat (argmax Nat tied_cost tied_append) one =
  argmax_append_keeps_left Nat tied_cost left_part right_part
    tied_comparison_is_false

theorem tied_append_selects_one
    : Equal Nat (argmax Nat tied_cost tied_append) one =
  Proved

theorem right_cost_zero_is_two : Equal Nat (right_cost Zero) two = Proved

theorem right_cost_one_is_seven : Equal Nat (right_cost one) seven = Proved

theorem right_cost_two_is_eight : Equal Nat (right_cost two) eight = Proved

theorem right_cost_three_is_one : Equal Nat (right_cost three) one = Proved

theorem right_left_selects_one
    : Equal Nat (argmax Nat right_cost left_part) one =
  Proved

theorem right_right_selects_two
    : Equal Nat (argmax Nat right_cost right_part) two =
  Proved

theorem right_comparison_is_true
    : Equal Bool
        (lt_nat
          (right_cost (argmax Nat right_cost left_part))
          (right_cost (argmax Nat right_cost right_part)))
        True =
  Proved

theorem right_append_takes_right
    : Equal Nat (argmax Nat right_cost tied_append) two =
  argmax_append_takes_right Nat right_cost left_part right_part
    right_comparison_is_true

theorem right_append_selects_two
    : Equal Nat (argmax Nat right_cost tied_append) two =
  Proved
```

### stdlib/optimization/argmax-append-split

- spec: §3.6 public Boolean-premise laws and §3.1 earliest-tie behavior.
- promise class: durable invariant.
- given: non-empty parts `[0, 1]` and `[2, 3]`, with costs `[2, 7]`
  and `[7, 1]`; a second cost function changes label `2` to cost `8`.
- expect: the tied split selects left label `1`; the strictly costlier
  right split selects label `2`.
- measured: eight direct fixture costs, comparison premises, independent
  `Proved` results for both part selections in both cases, and both public
  law applications with independent `Proved` append results.
- claimed: the append selector agrees with the more costly part's
  choice, and retains the earlier left choice when part maxima tie.
- gap: these closed Nat cases do not establish either generic law for
  every candidate type, cost function, or pair of non-empty inputs.
- why: the tied case makes a later-tie implementation observable, while
  the second case makes the right-taking branch observable.

## Isolated refused later-tie claim

On a **fresh full copy** of the accepted client, append only the theorem
below. Its accepted controls establish that the left part selects `1`
and the right part selects `2` on the same tied split; the append's
accepted result is independently `1`. The refusal must occur in this
appended theorem, not in an import or earlier control.

### stdlib/optimization/reject-argmax-append-takes-right-on-tie

- spec: §3.6 tie-left rule and §3.1 earliest occurrence rule.
- promise class: durable invariant.
- given: the accepted tied split, with both part maxima equal to `7`.
- expect: refusal of the false claim that the appended selection equals
  the right part's label `2`.
- measured: the accepted prefix fixes the left result, right result and
  appended result independently; only the appended equality is false.
- claimed: a later tied maximum cannot replace the earlier left choice.
- gap: this one split does not characterize every way a selector could
  violate earliest-tie behavior.
- why: the claim has the same `Equal Nat ... = Proved` form as the
  accepted selection controls and fails only because the append returns
  `1`, not the right part's `2`.

```ken
theorem seed_neg
    : Equal Nat (argmax Nat tied_cost tied_append) two =
  Proved
```
