# Inclusion-exclusion and monotonicity for exact event mass

Format: `../../README.md`.
Spec: `spec/50-stdlib/63-statistics.md` §4.

This is a closed behavioral oracle for §4's public intersection builder and
three natural-weight laws. The repeated-support fixture measures the laws at
specific values; it does not prove their generic statements. The shared
fixture includes positive clients for all three laws and a disjoint control
for the landed `disjoint_mass` law.

The fixture and each refusal were checked as temporary client modules against
the AC-1 candidate `e007498d11a8255df40512f925165dee1fc2e752`. The positive
module returned rc 0; six separate refused modules each returned rc 1 with
K0004 `TypeMismatch`. These are scratch client results, not execution through
a shipped seed harness, CI, publication, or landing.

## Shared fixture and positive clients

The support is the list `(0,1), (1,2), (2,3), (1,4)`: vertex `1` occurs
twice, with separate weights `2` and `4`. `low` selects vertices at most `1`;
`high` selects vertices at least `1`. Their masses are `7` and `9`; the union
mass is `10`, the intersection mass is `6`, and their sum is `16`. The
separate `at_zero` / `at_least_one` pair partitions the same support and has
masses `1` and `9`.

```ken
import Algorithm.Statistics.Exact
  (ExactDistribution, MkExactDistribution, total, event_mass,
   expectation_weight, event_union, event_intersection,
   inclusion_exclusion, event_mass_mono, expectation_weight_mono,
   disjoint_mass)
import Core.Classes.LawfulClasses (bool_and, bool_or, leq_nat)
import Data.Numeric.Nat.Arithmetic (add)

const n0 : Nat = Zero
const n1 : Nat = Suc n0
const n2 : Nat = Suc n1
const n3 : Nat = Suc n2
const n4 : Nat = Suc n3
const n5 : Nat = Suc n4
const n6 : Nat = Suc n5
const n7 : Nat = Suc n6
const n8 : Nat = Suc n7
const n9 : Nat = Suc n8
const n10 : Nat = Suc n9
const n16 : Nat = add n7 n9

const overlap_atoms : List (Pair Nat Nat) =
  Cons (Pair Nat Nat) (mk_pair Nat Nat n0 n1)
    (Cons (Pair Nat Nat) (mk_pair Nat Nat n1 (Suc n1))
      (Cons (Pair Nat Nat) (mk_pair Nat Nat n2 n3)
        (Cons (Pair Nat Nat) (mk_pair Nat Nat n1 n4)
          (Nil (Pair Nat Nat)))))
const overlap_distribution : ExactDistribution Nat =
  MkExactDistribution Nat overlap_atoms n9 Proved

const low : Nat → Bool = λx. leq_nat x n1
const high : Nat → Bool = λx. leq_nat n1 x
const at_zero : Nat → Bool = λx. leq_nat x n0
const at_least_one : Nat → Bool = λx. leq_nat n1 x

theorem low_implies_union (x : Nat) (h : Equal Bool (low x) True) :
  Equal Bool (event_union Nat low high x) True =
  J
    (λb _. Equal Bool
      (bool_or (low x) (high x)) (bool_or b (high x)))
    Refl h

theorem high_implies_high (x : Nat) (h : Equal Bool (high x) True) :
  Equal Bool (high x) True = h

theorem zero_successor_disjoint (x : Nat) :
  Equal Bool (bool_and (at_zero x) (at_least_one x)) False =
  match x {
    Zero ↦ Proved;
    Suc rest ↦ Proved
  }

const score_one : Nat → Nat = λx. n1
const score_two : Nat → Nat = λx. n2

theorem score_one_bounded (x : Nat) :
  Equal Bool (leq_nat (score_one x) (score_two x)) True = Proved

theorem score_two_self_bounded (x : Nat) :
  Equal Bool (leq_nat (score_two x) (score_two x)) True = Proved

theorem overlap_total :
  Equal Nat (total Nat overlap_distribution) n10 = Proved

theorem low_mass :
  Equal Nat (event_mass Nat overlap_distribution low) n7 = Proved

theorem high_mass :
  Equal Nat (event_mass Nat overlap_distribution high) n9 = Proved

theorem union_mass :
  Equal Nat
    (event_mass Nat overlap_distribution (event_union Nat low high)) n10 =
  Proved

theorem intersection_mass :
  Equal Nat
    (event_mass Nat overlap_distribution
      (event_intersection Nat low high)) n6 = Proved

theorem event_mass_sum :
  Equal Nat
    (add (event_mass Nat overlap_distribution low)
         (event_mass Nat overlap_distribution high)) n16 = Proved

theorem overlap_inclusion_exclusion :
  Equal Nat
    (add
      (event_mass Nat overlap_distribution (event_union Nat low high))
      (event_mass Nat overlap_distribution
        (event_intersection Nat low high)))
    (add (event_mass Nat overlap_distribution low)
         (event_mass Nat overlap_distribution high)) =
  inclusion_exclusion Nat overlap_distribution low high

theorem low_at_two : Equal Bool (low n2) False = Proved
theorem high_at_two : Equal Bool (high n2) True = Proved

theorem low_mass_leq_union :
  Equal Bool
    (leq_nat (event_mass Nat overlap_distribution low)
             (event_mass Nat overlap_distribution
               (event_union Nat low high)))
    True =
  event_mass_mono Nat overlap_distribution low
    (event_union Nat low high) low_implies_union

theorem low_score_weight :
  Equal Nat (expectation_weight Nat overlap_distribution score_one) n10 =
  Proved

theorem high_score_weight :
  Equal Nat
    (expectation_weight Nat overlap_distribution score_two)
    (add n10 n10) = Proved

theorem expectation_weight_mono_positive :
  Equal Bool
    (leq_nat
      (expectation_weight Nat overlap_distribution score_one)
      (expectation_weight Nat overlap_distribution score_two))
    True =
  expectation_weight_mono Nat overlap_distribution score_one score_two
    score_one_bounded

theorem at_zero_mass :
  Equal Nat (event_mass Nat overlap_distribution at_zero) n1 = Proved

theorem at_least_one_mass :
  Equal Nat (event_mass Nat overlap_distribution at_least_one) n9 = Proved

theorem disjoint_union_mass :
  Equal Nat
    (event_mass Nat overlap_distribution
      (event_union Nat at_zero at_least_one)) n10 = Proved

theorem disjoint_mass_sum :
  Equal Nat
    (add (event_mass Nat overlap_distribution at_zero)
         (event_mass Nat overlap_distribution at_least_one)) n10 =
  Proved

theorem disjoint_mass_positive_control :
  Equal Nat
    (event_mass Nat overlap_distribution
      (event_union Nat at_zero at_least_one))
    (add (event_mass Nat overlap_distribution at_zero)
         (event_mass Nat overlap_distribution at_least_one)) =
  disjoint_mass Nat overlap_distribution at_zero at_least_one
    zero_successor_disjoint
```

### stdlib/statistics/inclusion-exclusion-with-repeated-overlap

- spec: `spec/50-stdlib/63-statistics.md` §4.
- promise class: durable invariant.
- given: `overlap_distribution`, `low`, and `high` from the shared fixture.
  The support contains two separate entries for vertex `1`.
- expect: total `10`; union mass `10`; intersection mass `6`; `low` mass `7`;
  `high` mass `9`; and `low + high = 16`. The closed client of
  `inclusion_exclusion` accepts.
- measured: intersection, union, and both event masses, plus a closed client of
  the public `inclusion_exclusion` theorem.
- claimed: overlap is counted in both individual event masses and once in the
  intersection, while repeated support entries retain their separate weights.
- gap: one closed distribution does not prove inclusion-exclusion for arbitrary
  carriers, lists, or events; the generic checked law is the contract.
- why: the fixture makes intersection mass `2 + 4 = 6`; omitting it leaves
  `10 ≠ 16`.

### stdlib/statistics/disjoint-positive-control

- spec: `spec/50-stdlib/63-statistics.md` §§1.3 and 4.
- promise class: durable invariant.
- given: the same repeated-support distribution with `at_zero` and
  `at_least_one`.
- expect: the pointwise intersection is false; the two masses are `1` and `9`,
  their union mass and sum are both `10`, and the closed `disjoint_mass`
  client accepts.
- measured: a nonzero disjoint partition with both sides represented and the
  existing law's explicit pointwise premise.
- claimed: the overlap case does not merely accept a malformed fixture or an
  unavailable operation; the unchanged disjoint law still agrees on a positive
  control.
- gap: this `Nat` partition does not establish generic disjoint additivity.
- why: both sides contribute, and their union is the same full support.

### stdlib/statistics/event-and-score-monotonicity

- spec: `spec/50-stdlib/63-statistics.md` §4.
- promise class: durable invariant.
- given: `low` implies its union with `high`; constant scores `1` and `2` have
  pointwise order on every atom.
- expect: `event_mass_mono` accepts with `low` mass `7` and union mass `10`;
  `expectation_weight_mono` accepts with score weights `10` and `20`.
- measured: both public monotonicity laws with their respective stated
  pointwise premises, not a conclusion inferred from totals alone.
- claimed: event implication and pointwise score order each bound the
  corresponding natural numerator.
- gap: these closed clients do not prove either generic law.
- why: the inputs satisfy the exact premises required by §4.

## Refusals

Each refusal is a separate closed block against the shared positive fixture.
The two vertex-2 controls establish that `high` selects `2` while `low` does
not; this is the atom where the event implication fails.

### stdlib/statistics/reject-overlap-omitted

- spec: `spec/50-stdlib/63-statistics.md` §4.
- promise class: durable invariant.
- given: the shared `low` and `high` events on `overlap_distribution`.
- expect: claiming union mass equals `low` mass plus `high` mass rejects with
  K0004 `TypeMismatch`.
- measured: refusal of the false equality `10 = 16`; the positive
  inclusion-exclusion client accepts on the same distribution.
- claimed: overlap cannot be omitted from the additive natural-number law.
- gap: this closed counterexample is not a proof of the generic law.
- why: the intersection contributes `6`.

```ken
-- Reject: the union mass omits the repeated-support overlap.
theorem reject_union_equals_sum :
  Equal Nat
    (event_mass Nat overlap_distribution (event_union Nat low high))
    (add (event_mass Nat overlap_distribution low)
         (event_mass Nat overlap_distribution high)) = Proved
```

### stdlib/statistics/reject-false-event-monotonicity-at-vertex-two

- spec: `spec/50-stdlib/63-statistics.md` §4.
- promise class: durable invariant.
- given: `high` selects vertex `2`; `low` does not. Their masses are `9` and
  `7`.
- expect: asserting that `high` mass is at most `low` mass rejects with K0004
  `TypeMismatch`.
- measured: refusal of the false `9 ≤ 7` result; the fixture separately
  proves `high 2 = True` and `low 2 = False`.
- claimed: monotonicity is not available when the pointwise event implication
  fails at an atom.
- gap: this one atom and its finite list do not establish the generic law.
- why: vertex `2`, with weight `3`, belongs to `high` and not `low`.

```ken
-- Reject: high includes vertex 2, while low does not.
theorem reject_high_mass_leq_low :
  Equal Bool
    (leq_nat (event_mass Nat overlap_distribution high)
             (event_mass Nat overlap_distribution low))
    True = Proved
```

### stdlib/statistics/reject-pointwise-implication-at-vertex-two

- spec: `spec/50-stdlib/63-statistics.md` §4.
- promise class: durable invariant.
- given: `high 2 = True` and `low 2 = False` from the shared fixture.
- expect: asserting the pointwise implication at `2` rejects with K0004
  `TypeMismatch`.
- measured: refusal of `high 2 = True → low 2 = True`.
- claimed: the event premise of `event_mass_mono` is required at every atom.
- gap: a single failed pointwise implication does not establish other
  instances' premises.
- why: this is the exact atom where the attempted event ordering is false.

```ken
-- Reject: the event implication fails at vertex 2.
theorem reject_high_implies_low_at_two :
  Equal Bool (high n2) True → Equal Bool (low n2) True =
  λselected. Proved
```

### stdlib/statistics/reject-weakened-event-premise

- spec: `spec/50-stdlib/63-statistics.md` §4.
- promise class: durable invariant.
- given: the true self-implication `high x = True → high x = True`, where
  `event_mass_mono` requires `high x = True → low x = True`.
- expect: using the self-implication as the weaker premise for `event_mass_mono`
  rejects with K0004 `TypeMismatch`.
- measured: the required pointwise codomain differs from the supplied
  self-implication; the same fixture shows the failure at vertex `2`.
- claimed: the event law requires its stated pointwise implication, not an
  arbitrary or weakened premise.
- gap: one attempted weakened premise does not cover every malformed input.
- why: `high` and `low` disagree at the positive-weight vertex `2`.

```ken
-- Reject: high implies itself, not low.
theorem reject_weakened_event_premise :
  Equal Bool
    (leq_nat (event_mass Nat overlap_distribution high)
             (event_mass Nat overlap_distribution low))
    True =
  event_mass_mono Nat overlap_distribution high low high_implies_high
```

### stdlib/statistics/reject-weakened-score-premise

- spec: `spec/50-stdlib/63-statistics.md` §4.
- promise class: durable invariant.
- given: scores `2` and `1`; the supplied self-bound proves `2 ≤ 2`, while
  `expectation_weight_mono` requires the pointwise premise `2 ≤ 1`.
- expect: using the self-bound as the weaker score premise rejects with K0004
  `TypeMismatch`.
- measured: refusal of the supplied `f ≤ f` premise where the law requires
  `f ≤ g`.
- claimed: expectation monotonicity requires the exact pointwise score order.
- gap: this one score pair does not establish other premise failures.
- why: the weighted numerators are `20` and `10`; the required order is false.

```ken
-- Reject: the self-bound is not the required bound from score 2 to score 1.
theorem reject_weakened_score_premise :
  Equal Bool
    (leq_nat
      (expectation_weight Nat overlap_distribution score_two)
      (expectation_weight Nat overlap_distribution score_one))
    True =
  expectation_weight_mono Nat overlap_distribution score_two score_one
    score_two_self_bounded
```

### stdlib/statistics/reject-union-for-intersection

- spec: `spec/50-stdlib/63-statistics.md` §4.
- promise class: durable invariant.
- given: the shared overlapping `low` and `high` events.
- expect: replacing `event_intersection` with `event_union` in the
  inclusion-exclusion equation rejects with K0004 `TypeMismatch`.
- measured: the mutated equation claims `10 + 10 = 16`; the correct
  `event_intersection` value is `6` and the positive law client accepts.
- claimed: the second term is the intersection mass, not a second union mass.
- gap: this single mutation is not a proof of generic intersection behavior.
- why: the overlap is counted twice if the union is substituted.

```ken
-- Reject: union mass cannot replace the intersection mass.
theorem reject_union_for_intersection :
  Equal Nat
    (add
      (event_mass Nat overlap_distribution (event_union Nat low high))
      (event_mass Nat overlap_distribution (event_union Nat low high)))
    (add (event_mass Nat overlap_distribution low)
         (event_mass Nat overlap_distribution high)) = Proved
```

## Coverage boundary

These cases pin one repeated-support instance of the three §4 laws, the
public intersection result, the disjoint-law control, the event premise at its
failing atom, both weakened-premise refusals, and the incorrect
union-for-intersection mutation. They do not establish generic proof bodies,
shipped seed execution, CI, publication, landing, or a measured landed trust
delta. No subtraction, division, `DecEq q`, rational-valued law, new surface
feature, or premise beyond those stated in §4 is part of this seed.
