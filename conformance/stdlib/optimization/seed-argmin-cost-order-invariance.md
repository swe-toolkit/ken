# Least-cost selection under an order-preserving cost transform

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §1.5.
Frame: `docs/program/issues/SPEC-OPTIMIZATION-ARGMIN-COST-ORDER-INVARIANCE.md`.

Four labelled candidates have costs 5, 2, 7 and 2. The two minimum
costs are at labels 1 and 3, away from the head: the earlier label 1
wins. Adding three to each cost preserves every strict comparison;
flattening every cost to zero instead ties all four and selects head 0.
This spec-and-seed candidate does not deliver the catalog export.

## Accepted client

The complete accepted client checks before appending any refused
claim. The open `Refl` proves that adding three preserves and reflects
strict Nat comparisons; it is not a closed value oracle. Two direct
`Proved` facts independently evaluate the shifted and original
selections to the earliest tied label. The flattened selection's
accepted head result controls the refused equality on that same input.

```ken
import Algorithm.Optimization.Argmin (argmin, argmin_cost_order_invariant)

import Data.Collections.NonEmpty (NonEmpty, nonempty_cons)

import Data.Numeric.Nat.Arithmetic (add)

import Data.Numeric.Nat.Order (lt_nat)

import Core.Function.Combinators (comp)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
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
            Suc j ↦ two
          }
      }
  }

fn shifted (c : Nat) : Nat = add c three

fn flattened (c : Nat) : Nat = Zero

const labelled : NonEmpty Nat =
  nonempty_cons Nat Zero (Cons Nat one (Cons Nat two (Cons Nat three (Nil Nat))))

theorem zero_cost_is_five : Equal Nat (label_cost Zero) five = Proved

theorem one_cost_is_two : Equal Nat (label_cost one) two = Proved

theorem two_cost_is_seven : Equal Nat (label_cost two) seven = Proved

theorem three_cost_is_two : Equal Nat (label_cost three) two = Proved

theorem add_three_same_order (i : Nat) (j : Nat)
    : Equal Bool (lt_nat (shifted i) (shifted j)) (lt_nat i j) =
  Refl

theorem shifted_selects_cost_choice
    : Equal Nat
        (argmin Nat (comp Nat Nat Nat shifted label_cost) labelled)
        (argmin Nat label_cost labelled) =
  argmin_cost_order_invariant Nat label_cost shifted add_three_same_order labelled

theorem shifted_selects_earlier_tie
    : Equal Nat (argmin Nat (comp Nat Nat Nat shifted label_cost) labelled) one =
  Proved

theorem cost_selects_earlier_tie
    : Equal Nat (argmin Nat label_cost labelled) one =
  Proved

theorem flattened_selects_head
    : Equal Nat (argmin Nat (comp Nat Nat Nat flattened label_cost) labelled) Zero =
  Proved
```

### stdlib/optimization/argmin-preserves-earliest-tie-under-add-three

- spec: §1.5 public order-equality premise and §1.1 tie behavior.
- promise class: durable invariant.
- given: four distinct Nat labels, costs `[5, 2, 7, 2]`, and a
  transform adding three to every cost.
- expect: strict comparison equality accepts for all natural cost
  pairs; each closed selector independently returns label `1`.
- measured: open `Refl` for the order premise, the public law
  application, four direct fixture costs and two closed `Proved`
  selection values on the same nonempty input.
- claimed: preserving and reflecting every strict Nat comparison
  leaves the chosen candidate unchanged, including the first tied
  minimum away from the head.
- gap: these four labels and the add-three function do not prove the
  generic theorem for every type, cost or transform. The law's checked
  catalog proof is a separate Foundation delivery.
- why: separate expected labels do not ask the law to serve as its
  own value oracle; label 3 is a later tied minimum, so replacing
  on equality would be observed as the wrong result.

## Isolated refused flattened-cost equality

On a **fresh full copy** of the accepted client, append only the
following theorem. The transformed selector now selects head `0`,
while the original cost-only selector selects label `1`. The accepted
`flattened_selects_head` and `cost_selects_earlier_tie` controls fix
both sides on exactly the same `labelled` input; each has the same
`Equal Nat ... = Proved` proof form as the refused claim. The failure
must occur in the appended theorem, not an import or earlier control.

### stdlib/optimization/reject-flattened-cost-invariance

- spec: §1.5 premise boundary and §1.1 earliest tie.
- promise class: durable invariant.
- given: the accepted four-label client and `flattened c = Zero`.
- expect: refusal of equality between flattened-cost and original-cost
  selections, which produce `0` and `1`, respectively.
- measured: both separate result controls check in the accepted
  prefix; only the appended equality is false on the shared input.
- claimed: the order-equality premise cannot be omitted or applied
  to a transform that merges distinct costs into one tie.
- gap: one constant transform is a counterexample to an unguarded
  claim, not a characterization of every order-changing transform.
- why: a constant cost loses the distinction between original costs
  5 and 2, so the head becomes the earliest of the new four-way tie.

```ken
theorem seed_neg
    : Equal Nat
        (argmin Nat (comp Nat Nat Nat flattened label_cost) labelled)
        (argmin Nat label_cost labelled) =
  Proved
```

## Evidence boundary

Architect's isolated D0 on `12e91c41b` checked the generic proof
with no added `Axiom`, `ken check` and `ken fmt --check` rc 0,
owner-local Transparent status, private helper import refusals and
identical scratch `trusted_base()` GlobalIds (114 → 114). The D0
patch SHA-256 is
`4be533217c6582adcf0bac70649fdb961a1369dcdb7154c8f24de31fd6f08ac1`.
Its public-name client with this four-label cost shape checked;
its flattened-cost claim refused. The catalog hunk and probe test
were **not** part of the separate spec-and-seed candidate.

For author checks, a disposable detached worktree at `12e91c41b`
had **only** the catalog hunk of that D0 patch applied. In that
overlay, `ken check` and `ken fmt --check` on Argmin both returned
rc 0. The full accepted seed fence checked rc 0, and its temporary
formatted copy passed `ken fmt --check` rc 0. A fresh complete
accepted prefix plus only `seed_neg` failed rc 1/K0004
`KernelRejected TypeMismatch` at bytes 1844–1998, within the
appended declaration; the diagnostic reports `found: g1`. On an
untouched main-base catalog, the independent fixture costs and
both direct selection values checked rc 0 with the absent law
import and its application removed; the complete client failed
rc 1/E0012 `UnboundName` at the absent public theorem import.
Temporary clients and the scratch worktree were removed. The
preexisting CLI was not independently built at the detached base.

Logs under `/workspaces/ken/local/` are
`spec-author-argmin-order-overlay-catalog-check.log`,
`spec-author-argmin-order-overlay-catalog-fmt.log`,
`spec-author-argmin-order-overlay-positive.log`,
`spec-author-argmin-order-overlay-negative.log`,
`spec-author-argmin-order-overlay-seed-format.log`,
`spec-author-argmin-order-overlay-seed-fmt.log`,
`spec-author-argmin-order-base-values.log` and
`spec-author-argmin-order-base-laws.log`. These overlay checks
exercise a proposed catalog proof, not a delivered export. They
do not prove the generic law from closed cases or establish
shipped-export execution, official conformance-runner execution,
CI, publication, landing or landed trust.

The separate Foundation catalog candidate now exports the generic
checked `argmin_cost_order_invariant` proof and runs this seed via
`cat_optimization_argmin_cost_order`. That package test root-loads
the public law and checks the complete accepted client, including
the open order proof and independently reduced selection results.
Its isolated appended `seed_neg` is pinned to the false
flattened-versus-original selection proposition, then refused at
its own proof. A targeted strict-floor Argmin row and unchanged
exact trusted-GlobalId set are candidate-local measurements, not
a full-corpus census or landed trust. The package test is not
official conformance-runner execution, CI, publication or landing.
