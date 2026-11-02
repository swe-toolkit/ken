# Option selectors under order-preserving cost transforms

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §§1.7 and 3.7.
Frame:
`docs/program/issues/SPEC-OPTIMIZATION-ARGMIN-OPTION-COST-ORDER-INVARIANCE.md`.

Five labels have costs `[5, 2, 7, 2, 7]`. Under the original costs,
`argmin_option` selects `Some 1`, the earlier tied minimum, and
`argmax_option` selects `Some 2`, the earlier tied maximum. Adding three
to every cost preserves every strict comparison, so both selectors keep
those results. A constant-zero transform instead makes every cost tie;
both selectors return the head, `Some 0`.

## Accepted client

The complete accepted client checks before either isolated refused
claim is appended. It applies each public law to the tied list and to
`Nil`. Direct `Proved` controls independently establish the selected
values on the same list, including the flattened-cost head results.

```ken
import Algorithm.Optimization.Argmin
  (argmin_option, argmax_option,
   argmin_option_cost_order_invariant,
   argmax_option_cost_order_invariant)

import Data.Numeric.Nat.Arithmetic (add)
import Data.Numeric.Nat.Order (lt_nat)
import Core.Function.Combinators (comp)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc (Suc three)
const seven : Nat = Suc (Suc five)

fn label_cost (n : Nat) : Nat =
  match n {
    Zero ↦ five;
    Suc m ↦
      match m {
        Zero ↦ two;
        Suc k ↦
          match k {
            Zero ↦ seven;
            Suc j ↦
              match j {
                Zero ↦ two;
                Suc i ↦ seven
              }
          }
      }
  }

fn shifted (c : Nat) : Nat = add c three

fn flattened (c : Nat) : Nat = Zero

const labelled : List Nat =
  Cons Nat Zero
    (Cons Nat one
      (Cons Nat two
        (Cons Nat three
          (Cons Nat four (Nil Nat)))))

theorem add_three_same_order (i : Nat) (j : Nat)
    : Equal Bool (lt_nat (shifted i) (shifted j)) (lt_nat i j) =
  Refl

theorem min_law_on_tie
    : Equal (Option Nat)
        (argmin_option Nat (comp Nat Nat Nat shifted label_cost) labelled)
        (argmin_option Nat label_cost labelled) =
  argmin_option_cost_order_invariant Nat label_cost shifted
    add_three_same_order labelled

theorem min_law_on_nil
    : Equal (Option Nat)
        (argmin_option Nat (comp Nat Nat Nat shifted label_cost) (Nil Nat))
        (argmin_option Nat label_cost (Nil Nat)) =
  argmin_option_cost_order_invariant Nat label_cost shifted
    add_three_same_order (Nil Nat)

theorem max_law_on_tie
    : Equal (Option Nat)
        (argmax_option Nat (comp Nat Nat Nat shifted label_cost) labelled)
        (argmax_option Nat label_cost labelled) =
  argmax_option_cost_order_invariant Nat label_cost shifted
    add_three_same_order labelled

theorem max_law_on_nil
    : Equal (Option Nat)
        (argmax_option Nat (comp Nat Nat Nat shifted label_cost) (Nil Nat))
        (argmax_option Nat label_cost (Nil Nat)) =
  argmax_option_cost_order_invariant Nat label_cost shifted
    add_three_same_order (Nil Nat)

theorem min_selects_earlier_tie
    : Equal (Option Nat)
        (argmin_option Nat label_cost labelled) (Some Nat one) =
  Proved

theorem max_selects_earlier_tie
    : Equal (Option Nat)
        (argmax_option Nat label_cost labelled) (Some Nat two) =
  Proved

theorem min_flattened_head
    : Equal (Option Nat)
        (argmin_option Nat
          (comp Nat Nat Nat flattened label_cost) labelled)
        (Some Nat Zero) =
  Proved

theorem max_flattened_head
    : Equal (Option Nat)
        (argmax_option Nat
          (comp Nat Nat Nat flattened label_cost) labelled)
        (Some Nat Zero) =
  Proved

theorem min_control
    : Equal (Option Nat)
        (argmin_option Nat (comp Nat Nat Nat shifted label_cost) labelled)
        (argmin_option Nat label_cost labelled) =
  Proved

theorem max_control
    : Equal (Option Nat)
        (argmax_option Nat (comp Nat Nat Nat shifted label_cost) labelled)
        (argmax_option Nat label_cost labelled) =
  Proved
```

### stdlib/optimization/argmin-option-cost-order-invariance-tie

- spec: §1.7 public law and §1.1 earliest-tie behavior.
- promise class: durable invariant.
- given: labels `0` through `4`, costs `[5, 2, 7, 2, 7]`, and a
  transform adding three to every cost.
- expect: the law application accepts and the selector returns `Some 1`,
  the earlier tied minimum.
- measured: Architect AC-0 reports the open `Refl` order premise, law
  application, direct selector result, and same-form shifted-cost
  control checked on this input; CV has not run this exact seed.
- claimed: preserving and reflecting each strict cost comparison keeps
  the Option selector's selected value unchanged.
- gap: these closed Nat cases do not establish the generic law for every
  candidate type, cost function, transform, or list.
- why: the minimum is tied away from the head, so the earlier-tie rule
  affects an observable result.

### stdlib/optimization/argmin-option-cost-order-invariance-nil

- spec: §1.7 public law and §1.4 Nil behavior.
- promise class: durable invariant.
- given: `argmin_option` on `Nil` under the original and shifted costs.
- expect: the public law application accepts.
- measured: Architect AC-0 reports the law application at `Nil` checked
  in the accepted D0 client; CV has not run this exact seed.
- claimed: the law covers possibly empty lists, including `Nil`.
- gap: this instance does not establish the generic law.
- why: `Nil` exercises the Option selector's empty-list branch.

### stdlib/optimization/argmax-option-cost-order-invariance-tie

- spec: §3.7 public law and §3.1 earliest-tie behavior.
- promise class: durable invariant.
- given: labels `0` through `4`, costs `[5, 2, 7, 2, 7]`, and a
  transform adding three to every cost.
- expect: the law application accepts and the selector returns `Some 2`,
  the earlier tied maximum.
- measured: Architect AC-0 reports the open `Refl` order premise, law
  application, direct selector result, and same-form shifted-cost
  control checked on this input; CV has not run this exact seed.
- claimed: preserving and reflecting each strict cost comparison keeps
  the Option selector's selected value unchanged.
- gap: these closed Nat cases do not establish the generic law for every
  candidate type, cost function, transform, or list.
- why: the maximum is tied away from the head, so the earlier-tie rule
  affects an observable result.

### stdlib/optimization/argmax-option-cost-order-invariance-nil

- spec: §3.7 public law and §3.4 Nil behavior.
- promise class: durable invariant.
- given: `argmax_option` on `Nil` under the original and shifted costs.
- expect: the public law application accepts.
- measured: Architect AC-0 reports the law application at `Nil` checked
  in the accepted D0 client; CV has not run this exact seed.
- claimed: the law covers possibly empty lists, including `Nil`.
- gap: this instance does not establish the generic law.
- why: `Nil` exercises the Option selector's empty-list branch.

## Isolated refused flattened-cost claims

Each refused claim is checked on a fresh full copy of the accepted
client, with only that claim appended. The controls and the refusal share
the list, the equality form, and the `Proved` proof form. They differ
only in the transform: `shifted` preserves the strict order; `flattened`
merges distinct costs into one tie. The refusal must occur at the
appended claim, not in an import or an earlier control.

### stdlib/optimization/reject-argmin-option-flattened-cost-invariance

- spec: §1.7 premise boundary, §1.1 least-cost and tie behavior, and
  §1.4 Option selection.
- promise class: durable invariant.
- given: the accepted five-label list and `flattened c = Zero`.
- expect: reject the false equality between flattened-cost and original-
  cost `argmin_option` results; the results are `Some 0` and `Some 1`.
- measured: Architect AC-0 reports that `min_flattened_head`,
  `min_selects_earlier_tie`, and `min_control` checked the outputs and
  accepted same-form control on this input; CV has not run this seed.
- claimed: the Option selector does not remain unchanged for a transform
  that merges distinct costs into a tie.
- gap: this counterexample does not characterize every
  order-changing transform.
- why: the refusal differs from its accepted control only by replacing
  the order-preserving transform with the constant transform.

```ken
theorem reject_min_flattened
    : Equal (Option Nat)
        (argmin_option Nat
          (comp Nat Nat Nat flattened label_cost) labelled)
        (argmin_option Nat label_cost labelled) =
  Proved
```

### stdlib/optimization/reject-argmax-option-flattened-cost-invariance

- spec: §3.7 premise boundary, §3.1 greatest-cost and tie behavior, and
  §3.4 Option selection.
- promise class: durable invariant.
- given: the accepted five-label list and `flattened c = Zero`.
- expect: reject the false equality between flattened-cost and original-
  cost `argmax_option` results; the results are `Some 0` and `Some 2`.
- measured: Architect AC-0 reports that `max_flattened_head`,
  `max_selects_earlier_tie`, and `max_control` checked the outputs and
  accepted same-form control on this input; CV has not run this seed.
- claimed: the Option selector does not remain unchanged for a transform
  that merges distinct costs into a tie.
- gap: this counterexample does not characterize every
  order-changing transform.
- why: the refusal differs from its accepted control only by replacing
  the order-preserving transform with the constant transform.

```ken
theorem reject_max_flattened
    : Equal (Option Nat)
        (argmax_option Nat
          (comp Nat Nat Nat flattened label_cost) labelled)
        (argmax_option Nat label_cost labelled) =
  Proved
```

The flattened transform is not order-preserving: for example, original
costs `2 < 5` become `0 < 0`, which is false. The minimum refusal and
maximum refusal are separate; each is tested against its own accepted
prefix.

## Evidence boundary

Architect AC-0 reports that the matching accepted client checked on
landed base `dd421ad35` with `ken check` and `ken fmt --check` rc 0,
without new `Axiom` or postulate declarations, and with identical
scratch `trusted_base()` identity sets 114 → 114 over nine providers
(`evt_372fx3rnhcqh2`). AC-0 reports each isolated flattened-cost
claim was rejected with `KernelRejected TypeMismatch` at the appended
claim. These are Architect scratch measurements, not execution of
this exact seed by CV.

Foundation reports that assembled exact-tip candidate
`56b8864bbe064a8da8f059b4537714d8770437cb` passed the new
`cat_optimization_argmin_option_cost_order` target 2/2, with the full
trusted GlobalId identity set unchanged at 114 → 114
(`evt_6ch2ekksk284x`). This Foundation-reported package-test and
identity evidence is distinct from Architect's AC-0 scratch; CV ran no
tests. The measurements apply only to this exact candidate and do not
carry to the status-corrected respin.

Neither report establishes official conformance-runner execution, CI,
publication, landing, or measured landed trust. The closed cases do not
replace the two generic checked catalog laws.
