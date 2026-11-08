# Option selectors over appended two-Cons lists

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §§1.8 and 3.8.
Frame: `docs/program/issues/SPEC-OPTIMIZATION-OPTION-APPEND-SPLIT.md`.

The baseline split is `[0, 1] | [2, 3]`. A single cost function assigns
`[5, 1] | [2, 4]`, so the minimum and maximum both stay left. The tie
controls use right heads `6` and `7`; the strict-right controls use heads
`4` and `5`. Each variant keeps the same left list and right tail `[3]`.
These closed cases exercise the four public laws but do not prove their
generic types.

## Accepted client

This entire client accepts before any refused claim is appended. Direct
`Proved` equations pin the costs, part selections, comparison premises and
appended selections independently of the four laws. Its first Option
append-law application is the accepted `argmin_option` keep-left control
required by AC-2.

```ken
import Algorithm.Optimization.Argmin
  (argmin, argmax, argmin_option, argmax_option,
   argmin_option_append_keeps_left,
   argmin_option_append_takes_right,
   argmax_option_append_keeps_left,
   argmax_option_append_takes_right)

import Data.Collections.Derived (list_append)
import Data.Collections.NonEmpty (nonempty_cons)
import Data.Numeric.Nat.Order (lt_nat)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc (Suc (Suc (Suc (Suc Zero))))
const six : Nat = Suc five
const seven : Nat = Suc six

const left_tail : List Nat = Cons Nat one (Nil Nat)
const right_tail : List Nat = Cons Nat three (Nil Nat)
const left_input : List Nat = Cons Nat Zero left_tail
const keep_right_input : List Nat = Cons Nat two right_tail
const min_right_input : List Nat = Cons Nat four right_tail
const max_right_input : List Nat = Cons Nat five right_tail
const min_tie_right_input : List Nat = Cons Nat six right_tail
const max_tie_right_input : List Nat = Cons Nat seven right_tail

const keep_append : List Nat = list_append Nat left_input keep_right_input
const min_right_append : List Nat = list_append Nat left_input min_right_input
const max_right_append : List Nat = list_append Nat left_input max_right_input
const min_tie_append : List Nat = list_append Nat left_input min_tie_right_input
const max_tie_append : List Nat = list_append Nat left_input max_tie_right_input

fn cost (n : Nat) : Nat =
  match n {
    Zero ↦ five;
    Suc m ↦
      match m {
        Zero ↦ one;
        Suc k ↦
          match k {
            Zero ↦ two;
            Suc j ↦
              match j {
                Zero ↦ four;
                Suc k2 ↦
                  match k2 {
                    Zero ↦ Zero;
                    Suc l ↦
                      match l {
                        Zero ↦ six;
                        Suc m2 ↦
                          match m2 {
                            Zero ↦ one;
                            Suc n2 ↦
                              match n2 {
                                Zero ↦ five;
                                Suc r ↦ five
                              }
                          }
                      }
                  }
              }
          }
      }
  }

theorem cost_zero_is_five : Equal Nat (cost Zero) five = Proved
theorem cost_one_is_one : Equal Nat (cost one) one = Proved
theorem cost_two_is_two : Equal Nat (cost two) two = Proved
theorem cost_three_is_four : Equal Nat (cost three) four = Proved
theorem cost_four_is_zero : Equal Nat (cost four) Zero = Proved
theorem cost_five_is_six : Equal Nat (cost five) six = Proved
theorem cost_six_is_one : Equal Nat (cost six) one = Proved
theorem cost_seven_is_five : Equal Nat (cost seven) five = Proved

theorem keep_min_left_selects_one
    : Equal (Option Nat) (argmin_option Nat cost left_input)
        (Some Nat one) = Proved

theorem keep_min_right_selects_two
    : Equal (Option Nat) (argmin_option Nat cost keep_right_input)
        (Some Nat two) = Proved

theorem keep_min_append_selects_left
    : Equal (Option Nat) (argmin_option Nat cost keep_append)
        (Some Nat one) = Proved

theorem keep_max_left_selects_zero
    : Equal (Option Nat) (argmax_option Nat cost left_input)
        (Some Nat Zero) = Proved

theorem keep_max_right_selects_three
    : Equal (Option Nat) (argmax_option Nat cost keep_right_input)
        (Some Nat three) = Proved

theorem keep_max_append_selects_left
    : Equal (Option Nat) (argmax_option Nat cost keep_append)
        (Some Nat Zero) = Proved

theorem min_right_left_selects_one
    : Equal (Option Nat) (argmin_option Nat cost left_input)
        (Some Nat one) = Proved

theorem min_right_part_selects_four
    : Equal (Option Nat) (argmin_option Nat cost min_right_input)
        (Some Nat four) = Proved

theorem min_right_append_selects_four
    : Equal (Option Nat) (argmin_option Nat cost min_right_append)
        (Some Nat four) = Proved

theorem max_right_left_selects_zero
    : Equal (Option Nat) (argmax_option Nat cost left_input)
        (Some Nat Zero) = Proved

theorem max_right_part_selects_five
    : Equal (Option Nat) (argmax_option Nat cost max_right_input)
        (Some Nat five) = Proved

theorem max_right_append_selects_five
    : Equal (Option Nat) (argmax_option Nat cost max_right_append)
        (Some Nat five) = Proved

theorem min_tie_left_selects_one
    : Equal (Option Nat) (argmin_option Nat cost left_input)
        (Some Nat one) = Proved

theorem min_tie_part_selects_six
    : Equal (Option Nat) (argmin_option Nat cost min_tie_right_input)
        (Some Nat six) = Proved

theorem min_tie_append_selects_left
    : Equal (Option Nat) (argmin_option Nat cost min_tie_append)
        (Some Nat one) = Proved

theorem max_tie_left_selects_zero
    : Equal (Option Nat) (argmax_option Nat cost left_input)
        (Some Nat Zero) = Proved

theorem max_tie_part_selects_seven
    : Equal (Option Nat) (argmax_option Nat cost max_tie_right_input)
        (Some Nat seven) = Proved

theorem max_tie_append_selects_left
    : Equal (Option Nat) (argmax_option Nat cost max_tie_append)
        (Some Nat Zero) = Proved

theorem keep_min_comparison_is_false
    : Equal Bool
        (lt_nat
          (cost (argmin Nat cost
            (nonempty_cons Nat two right_tail)))
          (cost (argmin Nat cost
            (nonempty_cons Nat Zero left_tail))))
        False = Proved

theorem min_right_comparison_is_true
    : Equal Bool
        (lt_nat
          (cost (argmin Nat cost
            (nonempty_cons Nat four right_tail)))
          (cost (argmin Nat cost
            (nonempty_cons Nat Zero left_tail))))
        True = Proved

theorem min_tie_comparison_is_false
    : Equal Bool
        (lt_nat
          (cost (argmin Nat cost
            (nonempty_cons Nat six right_tail)))
          (cost (argmin Nat cost
            (nonempty_cons Nat Zero left_tail))))
        False = Proved

theorem keep_max_comparison_is_false
    : Equal Bool
        (lt_nat
          (cost (argmax Nat cost
            (nonempty_cons Nat Zero left_tail)))
          (cost (argmax Nat cost
            (nonempty_cons Nat two right_tail))))
        False = Proved

theorem max_right_comparison_is_true
    : Equal Bool
        (lt_nat
          (cost (argmax Nat cost
            (nonempty_cons Nat Zero left_tail)))
          (cost (argmax Nat cost
            (nonempty_cons Nat five right_tail))))
        True = Proved

theorem max_tie_comparison_is_false
    : Equal Bool
        (lt_nat
          (cost (argmax Nat cost
            (nonempty_cons Nat Zero left_tail)))
          (cost (argmax Nat cost
            (nonempty_cons Nat seven right_tail))))
        False = Proved

theorem keep_min_append_law
    : Equal (Option Nat) (argmin_option Nat cost keep_append)
        (argmin_option Nat cost left_input) =
  argmin_option_append_keeps_left Nat cost Zero left_tail
    two right_tail Proved

theorem min_right_append_law
    : Equal (Option Nat) (argmin_option Nat cost min_right_append)
        (argmin_option Nat cost min_right_input) =
  argmin_option_append_takes_right Nat cost Zero left_tail
    four right_tail Proved

theorem min_tie_append_law
    : Equal (Option Nat) (argmin_option Nat cost min_tie_append)
        (argmin_option Nat cost left_input) =
  argmin_option_append_keeps_left Nat cost Zero left_tail
    six right_tail Proved

theorem keep_max_append_law
    : Equal (Option Nat) (argmax_option Nat cost keep_append)
        (argmax_option Nat cost left_input) =
  argmax_option_append_keeps_left Nat cost Zero left_tail
    two right_tail Proved

theorem max_right_append_law
    : Equal (Option Nat) (argmax_option Nat cost max_right_append)
        (argmax_option Nat cost max_right_input) =
  argmax_option_append_takes_right Nat cost Zero left_tail
    five right_tail Proved

theorem max_tie_append_law
    : Equal (Option Nat) (argmax_option Nat cost max_tie_append)
        (argmax_option Nat cost left_input) =
  argmax_option_append_keeps_left Nat cost Zero left_tail
    seven right_tail Proved
```

### stdlib/optimization/option-append-split

- spec: §1.8 and §3.8 public Option append-split laws, with §1.4,
  §1.6, §3.4 and §3.6 defining the selectors, sibling laws and
  earliest-tie rules.
- promise class: durable invariant.
- given: left `[0, 1]`; baseline right `[2, 3]`; the cost function
  assigns `[5, 1] | [2, 4]`. Other right heads change only the
  comparison axis: `4` makes the minimum strictly cheaper, `5` makes
  the maximum strictly costlier, and `6` / `7` produce left/right
  ties for the minimum / maximum.
- expect: all four public laws accept with their stated premises.
  The tied cases keep the left Option; the strict-right cases take the
  right Option.
- measured: eight direct cost values, eighteen part and append
  selector results, six comparison premises, and six applications of
  the four public laws are asserted by closed `Proved` equations.
- claimed: the Option selectors agree with the selected part on each
  two-`Cons` append, keeping the earlier left choice on a tie.
- gap: these closed Nat instances do not prove the generic laws for
  every candidate type, cost function or pair of non-empty lists.
- why: the right part's strict improvement distinguishes each
  take-right branch, while tied selected values differ and make the
  tie-left rule observable.

## Isolated refused claims

For each refusal below, first accept the complete client above. Then
copy it fresh and append exactly one claim. The expected refusal must
occur at that appended declaration. The accepted same-form law
application and direct selector controls in the prefix exclude an
import, fixture or theorem-form error as the reason for refusal.

### stdlib/optimization/reject-argmin-option-keeps-left-when-right-is-cheaper

- spec: §1.8 premise and §1.6's strict cost comparison.
- promise class: durable invariant.
- given: the accepted left `[0, 1]` and right `[4, 3]` lists, where
  the right minimum is strictly cheaper. This changes only the right
  head from `2` in the accepted keep-left application to `4`.
- expect: refuse the `argmin_option_append_keeps_left` application
  because its `False` premise is not proved by `Proved` on this input.
- measured: `min_right_comparison_is_true` accepts in the prefix; it
  also selects `Some 4` on the appended `[4, 3]` list independently.
- claimed: a strictly cheaper right minimum cannot satisfy the
  keep-left premise.
- gap: this one strict comparison does not cover every false premise.
- why: only the right head changes from the accepted keep-left control,
  and the right result is independently observable.

```ken
theorem refuse_min_keep_left_when_right_is_cheaper
    : Equal (Option Nat) (argmin_option Nat cost min_right_append)
        (argmin_option Nat cost left_input) =
  argmin_option_append_keeps_left Nat cost Zero left_tail
    four right_tail Proved
```

### stdlib/optimization/reject-argmin-option-takes-right-on-tie

- spec: §1.8's strict-right premise and §1.1's earliest-tie rule.
- promise class: durable invariant.
- given: the accepted left `[0, 1]` and right `[6, 3]` lists, where
  both part minima are `1` but their selected labels differ.
- expect: refuse the `argmin_option_append_takes_right` application
  because its `True` premise is not proved by `Proved` on this input.
- measured: `min_tie_comparison_is_false` accepts in the prefix; it
  independently selects `Some 1` for the tied append.
- claimed: a tie between part minima does not satisfy the strict-right
  premise or replace the earlier left value.
- gap: this tied split does not cover every non-strict comparison.
- why: the right value is a distinct later minimum, so taking it would
  visibly disagree with the accepted append result.

```ken
theorem refuse_min_take_right_on_tie
    : Equal (Option Nat) (argmin_option Nat cost min_tie_append)
        (argmin_option Nat cost min_tie_right_input) =
  argmin_option_append_takes_right Nat cost Zero left_tail
    six right_tail Proved
```

### stdlib/optimization/reject-argmax-option-keeps-left-when-right-is-costlier

- spec: §3.8 premise and §3.6's strict cost comparison.
- promise class: durable invariant.
- given: the accepted left `[0, 1]` and right `[5, 3]` lists, where
  the right maximum is strictly costlier. This changes only the right
  head from `2` in the accepted keep-left application to `5`.
- expect: refuse the `argmax_option_append_keeps_left` application
  because its `False` premise is not proved by `Proved` on this input.
- measured: `max_right_comparison_is_true` accepts in the prefix; it
  also selects `Some 5` on the appended `[5, 3]` list independently.
- claimed: a strictly costlier right maximum cannot satisfy the
  keep-left premise.
- gap: this one strict comparison does not cover every false premise.
- why: only the right head changes from the accepted keep-left control,
  and the right result is independently observable.

```ken
theorem refuse_max_keep_left_when_right_is_costlier
    : Equal (Option Nat) (argmax_option Nat cost max_right_append)
        (argmax_option Nat cost left_input) =
  argmax_option_append_keeps_left Nat cost Zero left_tail
    five right_tail Proved
```

### stdlib/optimization/reject-argmax-option-takes-right-on-tie

- spec: §3.8's strict-right premise and §3.1's earliest-tie rule.
- promise class: durable invariant.
- given: the accepted left `[0, 1]` and right `[7, 3]` lists, where
  both part maxima are `5` but their selected labels differ.
- expect: refuse the `argmax_option_append_takes_right` application
  because its `True` premise is not proved by `Proved` on this input.
- measured: `max_tie_comparison_is_false` accepts in the prefix; it
  independently selects `Some 0` for the tied append.
- claimed: a tie between part maxima does not satisfy the strict-right
  premise or replace the earlier left value.
- gap: this tied split does not cover every non-strict comparison.
- why: the right value is a distinct later maximum, so taking it would
  visibly disagree with the accepted append result.

```ken
theorem refuse_max_take_right_on_tie
    : Equal (Option Nat) (argmax_option Nat cost max_tie_append)
        (argmax_option Nat cost max_tie_right_input) =
  argmax_option_append_takes_right Nat cost Zero left_tail
    seven right_tail Proved
```

## Evidence boundary

Architect AC-0 reports that the four generic laws, a closed accepted
client and three refusal rows checked in scratch on base `ddc3830f1`,
with no new `Axiom` and equal scratch trusted GlobalId identity sets
`114 → 114`. Those are D0 scratch checks, not execution of this seed
by CV. This seed adds the `argmax_option_append_takes_right` on-tie
refusal beyond those rows; it has no execution result. No package test,
official conformance-runner, CI, publication, landing or landed-trust
measurement is claimed.
