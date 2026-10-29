# Least-cost selection over appended non-empty parts

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §1.6.
Frame: `docs/program/issues/SPEC-OPTIMIZATION-ARGMIN-APPEND-SPLIT.md`.

The split `0, 1 | 2, 3` has costs `5, 2 | 2, 7`, so both parts
select cost `2`. The append keeps label `1`, the earlier choice. In a
second case, label `2` costs `1`, so the right part is strictly cheaper
and the append selects label `2`. These cases exercise both public
branches without proving the generic laws.

## Accepted client

The entire accepted client checks before the isolated refused claim is
appended. Direct closed `Proved` results establish the fixture costs
and each part's selected label independently of the append laws. The
public laws then establish the append selections under their stated
Boolean premises.

```ken
import Algorithm.Optimization.Argmin
  (argmin,
    argmin_append_keeps_left,
    argmin_append_takes_right)

import Data.Collections.NonEmpty
  (NonEmpty,
    nonempty_cons,
    nonempty_append)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const five : Nat = Suc (Suc three)
const seven : Nat = Suc (Suc five)

fn tied_cost (n : Nat) : Nat =
  match n {
    Zero ↦ five;
    Suc m ↦
      match m {
        Zero ↦ two;
        Suc k ↦
          match k {
            Zero ↦ two;
            Suc j ↦ seven
          }
      }
  }

fn right_cheaper_cost (n : Nat) : Nat =
  match n {
    Zero ↦ five;
    Suc m ↦
      match m {
        Zero ↦ two;
        Suc k ↦
          match k {
            Zero ↦ one;
            Suc j ↦ seven
          }
      }
  }

const left_part : NonEmpty Nat =
  nonempty_cons Nat Zero (Cons Nat one (Nil Nat))

const right_part : NonEmpty Nat =
  nonempty_cons Nat two (Cons Nat three (Nil Nat))

const tied_append : NonEmpty Nat =
  nonempty_append Nat left_part right_part

theorem tied_cost_zero_is_five : Equal Nat (tied_cost Zero) five = Proved

theorem tied_cost_one_is_two : Equal Nat (tied_cost one) two = Proved

theorem tied_cost_two_is_two : Equal Nat (tied_cost two) two = Proved

theorem tied_cost_three_is_seven : Equal Nat (tied_cost three) seven = Proved

theorem tied_left_selects_one
    : Equal Nat (argmin Nat tied_cost left_part) one =
  Proved

theorem tied_right_selects_two
    : Equal Nat (argmin Nat tied_cost right_part) two =
  Proved

theorem tied_append_keeps_left
    : Equal Nat (argmin Nat tied_cost tied_append) one =
  argmin_append_keeps_left Nat tied_cost left_part right_part Proved

theorem cheaper_cost_two_is_one
    : Equal Nat (right_cheaper_cost two) one = Proved

theorem cheaper_left_selects_one
    : Equal Nat (argmin Nat right_cheaper_cost left_part) one =
  Proved

theorem cheaper_right_selects_two
    : Equal Nat (argmin Nat right_cheaper_cost right_part) two =
  Proved

theorem cheaper_append_takes_right
    : Equal Nat (argmin Nat right_cheaper_cost tied_append) two =
  argmin_append_takes_right Nat right_cheaper_cost left_part right_part Proved
```

### stdlib/optimization/argmin-append-split

- spec: §1.6 public Boolean-premise laws and §1.1 earliest-tie behavior.
- promise class: durable invariant.
- given: non-empty parts `[0, 1]` and `[2, 3]`, with costs `[5, 2]`
  and `[2, 7]`; a second cost function changes label `2` to cost `1`.
- expect: the tied split selects left label `1`; the strictly cheaper
  right split selects label `2`.
- measured: four direct cost values, independent `Proved` results for
  both part selections in both cases, and both public law applications.
- claimed: the append selector agrees with the cheaper part's choice,
  and retains the earlier left choice when part minima tie.
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

### stdlib/optimization/reject-argmin-append-takes-right-on-tie

- spec: §1.6 tie-left rule and §1.1 earliest occurrence rule.
- promise class: durable invariant.
- given: the accepted tied split, with both part minima equal to `2`.
- expect: refusal of the false claim that the appended selection equals
  the right part's label `2`.
- measured: the accepted prefix fixes the left result, right result and
  appended result independently; only the appended equality is false.
- claimed: a later tied minimum cannot replace the earlier left choice.
- gap: this one split does not characterize every way a selector could
  violate earliest-tie behavior.
- why: the claim has the same `Equal Nat ... = Proved` form as the
  accepted selection controls and fails only because the append returns
  `1`, not the right part's `2`.

```ken
theorem seed_neg
    : Equal Nat (argmin Nat tied_cost tied_append) two =
  Proved
```
