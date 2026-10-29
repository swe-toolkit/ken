# Greatest-cost selection under an order-preserving cost transform

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §3.5.
Frame: `docs/program/issues/SPEC-OPTIMIZATION-ARGMAX-COST-ORDER-INVARIANCE.md`.

Four labelled candidates have costs 5, 7, 2 and 7. Their greatest cost is
shared by labels 1 and 3, away from the head, so the earlier tied maximum is
label 1. Adding three preserves every strict comparison and leaves that
selection unchanged. Flattening every cost to zero instead makes the head,
label 0, the selected candidate.

## Accepted client

The complete accepted client checks before appending the refused equality.
The open `Refl` proves that adding three preserves and reflects strict Nat
comparisons; it is not a closed value oracle. Two direct `Proved` facts
independently establish that the shifted-cost and original-cost selectors
both return the earlier tied label. The flattened head result and the
original-cost result control the refused equality on that same input.

```ken
import Algorithm.Optimization.Argmin
  (argmax, argmax_cost_order_invariant)

import Data.Collections.NonEmpty (NonEmpty, nonempty_cons)

import Data.Numeric.Nat.Arithmetic (add)

import Data.Numeric.Nat.Order (lt_nat)

import Core.Function.Combinators (comp)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const five : Nat = Suc (Suc (Suc (Suc (Suc Zero))))
const seven : Nat = Suc (Suc five)

fn label_cost (n : Nat) : Nat =
  match n {
    Zero ↦ five;
    Suc m ↦
      match m {
        Zero ↦ seven;
        Suc k ↦
          match k {
            Zero ↦ two;
            Suc j ↦ seven
          }
      }
  }

fn shifted (c : Nat) : Nat = add c three

fn flattened (c : Nat) : Nat = Zero

const labelled : NonEmpty Nat =
  nonempty_cons Nat Zero
    (Cons Nat one (Cons Nat two (Cons Nat three (Nil Nat))))

theorem zero_cost_is_five : Equal Nat (label_cost Zero) five = Proved

theorem one_cost_is_seven : Equal Nat (label_cost one) seven = Proved

theorem two_cost_is_two : Equal Nat (label_cost two) two = Proved

theorem three_cost_is_seven : Equal Nat (label_cost three) seven = Proved

theorem add_three_same_order (i : Nat) (j : Nat)
    : Equal Bool (lt_nat (shifted i) (shifted j)) (lt_nat i j) =
  Refl

theorem shifted_selects_cost_choice
    : Equal Nat
        (argmax Nat (comp Nat Nat Nat shifted label_cost) labelled)
        (argmax Nat label_cost labelled) =
  argmax_cost_order_invariant Nat label_cost shifted
    add_three_same_order labelled

theorem shifted_selects_earlier_tie
    : Equal Nat
        (argmax Nat (comp Nat Nat Nat shifted label_cost) labelled)
        one =
  Proved

theorem cost_selects_earlier_tie
    : Equal Nat (argmax Nat label_cost labelled) one =
  Proved

theorem flattened_selects_head
    : Equal Nat
        (argmax Nat (comp Nat Nat Nat flattened label_cost) labelled)
        Zero =
  Proved
```

### stdlib/optimization/argmax-preserves-earliest-tied-maximum-under-add-three

- spec: §3.5 public order-equality premise and §3.1 tie behavior.
- promise class: durable invariant.
- given: four distinct Nat labels, costs `[5, 7, 2, 7]`, and a transform
  adding three to every cost.
- expect: strict comparison equality accepts for all natural cost pairs;
  each closed selector independently returns label `1`.
- measured: open `Refl` for the order premise, the public law application,
  four direct fixture costs and two closed `Proved` selection values on the
  same nonempty input.
- claimed: preserving and reflecting every strict Nat comparison leaves
  the chosen candidate unchanged, including the first tied maximum away
  from the head.
- gap: these four labels and the add-three function do not prove the generic
  theorem for every type, cost or transform.
- why: the maximum is tied at labels `1` and `3`; replacing on equality
  would select the wrong candidate.

## Isolated refused flattened-cost equality

On a **fresh full copy** of the accepted client, append only the theorem
below. The flattened-cost selector returns head `0`, while the original
cost-only selector returns label `1`. The accepted `flattened_selects_head`
and `cost_selects_earlier_tie` controls fix both sides on exactly the same
input. The failure must occur in this appended declaration, not in an
import or an earlier control.

### stdlib/optimization/reject-flattened-cost-equality

- spec: §3.5 premise boundary and §3.1 earliest-tie behavior.
- promise class: durable invariant.
- given: the accepted four-label client and `flattened c = Zero`.
- expect: reject the equality at this theorem's own `Proved`; the
  flattened-cost and original-cost selections produce `0` and `1`.
- measured: both separate result controls check in the accepted prefix;
  the appended equality is false on the shared input.
- claimed: the strict-order equality premise cannot be omitted or applied
  to a transform that merges distinct costs into one tie.
- gap: one constant transform is a counterexample to an unguarded claim,
  not a characterization of every order-changing transform.
- why: flattening merges the original costs `5` and `7`; the earlier
  maximum becomes the head of the new four-way tie.

```ken
theorem seed_neg
    : Equal Nat
        (argmax Nat (comp Nat Nat Nat flattened label_cost) labelled)
        (argmax Nat label_cost labelled) =
  Proved
```

## Evidence boundary

Architect's isolated AC-0 on landed base
`6e778c2d5288219287884de49eb53fb278ba98ea` checked the generic law and its
closed client as scratch observations. The §3.5 catalog export was absent
on this seed's base, so this authored seed has not been executed against a
shipped export. Scratch D0 is not official conformance-runner execution,
CI, publication, landing or landed trust.
