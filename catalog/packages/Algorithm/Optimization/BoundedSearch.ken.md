# `BoundedSearch` — least witness below a finite bound

`find_below` returns the smallest natural number below a bound that satisfies
an ordinary Boolean predicate, or `None` when no such number exists.

## Contents

- [Motivation](#motivation)
- [Definition](#definition)
- [Using it](#using-it)
- [Laws & proofs](#laws--proofs)
- [Design notes](#design-notes)
- [References](#references)
- [Trust & derivation](#trust--derivation)

## Motivation

Finite search can make a precise claim about both its answers and its absence
of answers. A reported number must satisfy the predicate and precede all other
hits; `None` must mean every number in the specified range fails. The bound
makes these claims provable by induction without a decision procedure for
arbitrary predicates or an unbounded search mechanism.

## Definition

The public operation and its seven laws lead. The private outcome invariant
records either the absence of every earlier hit or a least satisfying witness.

```ken
import Core.Logic.Conjunction (And, and_intro, and_fst, and_snd)

import Core.Logic.Transport (cong, sym, trans)

import Core.Classes.LawfulClasses (leq_nat)

import Data.Numeric.Nat.Order (lt_nat)

pub fn find_below (p : Nat → Bool) (bound : Nat) : Option Nat =
  match bound {
    Zero ↦ None Nat;
    Suc below ↦ find_next p below (find_below p below)
  }

pub theorem find_below_sound
      (p : Nat → Bool) (bound : Nat) (n : Nat)
    : Equal (Option Nat) (find_below p bound) (Some Nat n)
      → And (Equal Bool (lt_nat n bound) True) (Equal Bool (p n) True) =
  λfound.
    found_some_sound
      p
      bound
      n
      (J (λr _. found_spec p bound r) (find_below_correct p bound) found)

pub theorem find_below_least
      (p : Nat → Bool) (bound : Nat) (n : Nat)
    : Equal (Option Nat) (find_below p bound) (Some Nat n)
      → (m : Nat)
      → Equal Bool (lt_nat m n) True
      → Equal Bool (p m) False =
  λfound.
    found_some_least
      p
      bound
      n
      (J (λr _. found_spec p bound r) (find_below_correct p bound) found)

pub theorem find_below_complete
      (p : Nat → Bool) (bound : Nat)
    : Equal (Option Nat) (find_below p bound) (None Nat)
      → (m : Nat)
      → Equal Bool (lt_nat m bound) True
      → Equal Bool (p m) False =
  λmissing. J (λr _. found_spec p bound r) (find_below_correct p bound) missing

pub theorem find_below_refuted
      (p : Nat → Bool) (bound : Nat)
    : ((m : Nat) → Equal Bool (lt_nat m bound) True → Equal Bool (p m) False)
      → Equal (Option Nat) (find_below p bound) (None Nat) =
  refuted_outcome_none p bound (find_below p bound) Refl

pub theorem find_below_least_hit
      (p : Nat → Bool) (bound : Nat) (n : Nat)
    : Equal Bool (lt_nat n bound) True
      → Equal Bool (p n) True
      → ((m : Nat) → Equal Bool (lt_nat m n) True → Equal Bool (p m) False)
      → Equal (Option Nat) (find_below p bound) (Some Nat n) =
  hit_outcome_some p bound n (find_below p bound) Refl

pub theorem find_below_stable
      (p : Nat → Bool) (bound : Nat) (larger : Nat) (n : Nat)
    : Equal Bool (leq_nat bound larger) True
      → Equal (Option Nat) (find_below p bound) (Some Nat n)
      → Equal (Option Nat) (find_below p larger) (Some Nat n) =
  match larger {
    Zero ↦
      match bound {
        Zero ↦ λwithin. λfound. absurd found;
        Suc below ↦ λwithin. λfound. absurd within
      };
    Suc below ↦
      λwithin.
        λfound.
          match leq_nat bound below eqn : earlier {
            True ↦
              cong
                (Option Nat)
                (Option Nat)
                (find_below p below)
                (Some Nat n)
                (find_next p below)
                (find_below_stable p bound below n earlier found);
            False ↦
              trans
                (Option Nat)
                (find_below p (Suc below))
                (find_below p bound)
                (Some Nat n)
                (cong
                  Nat
                  (Option Nat)
                  (Suc below)
                  bound
                  (find_below p)
                  (sym Nat bound (Suc below) (leq_suc_not_leq_same bound below within earlier)))
                found
          }
  }

pub theorem find_below_ext
      (p : Nat → Bool) (q : Nat → Bool) (bound : Nat)
    : ((m : Nat) → Equal Bool (lt_nat m bound) True → Equal Bool (p m) (q m))
      → Equal (Option Nat) (find_below p bound) (find_below q bound) =
  match bound {
    Zero ↦ λagree. Proved;
    Suc below ↦
      λagree.
        trans
          (Option Nat)
          (find_next p below (find_below p below))
          (find_next p below (find_below q below))
          (find_next q below (find_below q below))
          (cong
            (Option Nat)
            (Option Nat)
            (find_below p below)
            (find_below q below)
            (find_next p below)
            (find_below_ext
              p
              q
              below
              (λm.
                λearlier.
                  agree
                    m
                    ((proof trans for lt_nat)
                      m
                      below
                      (Suc below)
                      earlier
                      ((proof self_suc for lt_nat) below)))))
          (find_next_ext
            p
            q
            below
            (find_below q below)
            (agree below ((proof self_suc for lt_nat) below)))
  }

fn find_next (p : Nat → Bool) (below : Nat) (prior : Option Nat) : Option Nat =
  match prior {
    None ↦ found_at below (p below);
    Some n ↦ Some Nat n
  }

fn found_at (candidate : Nat) (hit : Bool) : Option Nat =
  match hit {
    True ↦ Some Nat candidate;
    False ↦ None Nat
  }

fn none_below (p : Nat → Bool) (bound : Nat) : Prop =
  (m : Nat) → Equal Bool (lt_nat m bound) True → Equal Bool (p m) False

fn found_spec (p : Nat → Bool) (bound : Nat) (outcome : Option Nat) : Prop =
  match outcome {
    None ↦ none_below p bound;
    Some n ↦
      And (Equal Bool (lt_nat n bound) True) (And (Equal Bool (p n) True) (none_below p n))
  }

theorem found_some_sound
      (p : Nat → Bool) (bound : Nat) (n : Nat)
    : found_spec p bound (Some Nat n)
      → And (Equal Bool (lt_nat n bound) True) (Equal Bool (p n) True) =
  λspec.
    and_intro
      (Equal Bool (lt_nat n bound) True)
      (Equal Bool (p n) True)
      (and_fst
        (Equal Bool (lt_nat n bound) True)
        (And (Equal Bool (p n) True) (none_below p n))
        spec)
      (and_fst
        (Equal Bool (p n) True)
        (none_below p n)
        (and_snd
          (Equal Bool (lt_nat n bound) True)
          (And (Equal Bool (p n) True) (none_below p n))
          spec))

theorem found_some_least
      (p : Nat → Bool) (bound : Nat) (n : Nat)
    : found_spec p bound (Some Nat n) → none_below p n =
  λspec.
    and_snd
      (Equal Bool (p n) True)
      (none_below p n)
      (and_snd
        (Equal Bool (lt_nat n bound) True)
        (And (Equal Bool (p n) True) (none_below p n))
        spec)

theorem refuted_outcome_none
      (p : Nat → Bool) (bound : Nat) (outcome : Option Nat)
    : Equal (Option Nat) (find_below p bound) outcome
      → none_below p bound
      → Equal (Option Nat) (find_below p bound) (None Nat) =
  match outcome {
    None ↦ λobserved. λrefuted. observed;
    Some n ↦
      λobserved.
        λrefuted.
          absurd
            (trans
              Bool
              True
              (p n)
              False
              (sym
                Bool
                (p n)
                True
                (and_snd
                  (Equal Bool (lt_nat n bound) True)
                  (Equal Bool (p n) True)
                  (find_below_sound p bound n observed)))
              (refuted
                n
                (and_fst
                  (Equal Bool (lt_nat n bound) True)
                  (Equal Bool (p n) True)
                  (find_below_sound p bound n observed))))
  }

theorem hit_outcome_some
      (p : Nat → Bool) (bound : Nat) (n : Nat) (outcome : Option Nat)
    : Equal (Option Nat) (find_below p bound) outcome
      → Equal Bool (lt_nat n bound) True
      → Equal Bool (p n) True
      → none_below p n
      → Equal (Option Nat) (find_below p bound) (Some Nat n) =
  match outcome {
    None ↦
      λobserved.
        λwithin.
          λhit.
            λprefix.
              absurd
                (trans
                  Bool
                  True
                  (p n)
                  False
                  (sym Bool (p n) True hit)
                  (find_below_complete p bound observed n within));
    Some k ↦
      λobserved.
        λwithin.
          λhit.
            λprefix.
              match lt_nat k n eqn : before {
                True ↦
                  absurd
                    (trans
                      Bool
                      True
                      (p k)
                      False
                      (sym
                        Bool
                        (p k)
                        True
                        (and_snd
                          (Equal Bool (lt_nat k bound) True)
                          (Equal Bool (p k) True)
                          (find_below_sound p bound k observed)))
                      (prefix k before));
                False ↦
                  match lt_nat n k eqn : after {
                    True ↦
                      absurd
                        (trans
                          Bool
                          True
                          (p n)
                          False
                          (sym Bool (p n) True hit)
                          (find_below_least p bound k observed n after));
                    False ↦
                      trans
                        (Option Nat)
                        (find_below p bound)
                        (Some Nat k)
                        (Some Nat n)
                        observed
                        (cong
                          Nat
                          (Option Nat)
                          k
                          n
                          (λx. Some Nat x)
                          ((proof antisym for leq_nat)
                            k
                            n
                            ((proof false_reverse_leq for lt_nat) n k after)
                            ((proof false_reverse_leq for lt_nat) k n before)))
                  }
              }
  }

theorem find_below_correct
      (p : Nat → Bool) (bound : Nat)
    : found_spec p bound (find_below p bound) =
  match bound {
    Zero ↦ λm. λbelow_zero. absurd below_zero;
    Suc below ↦ find_next_correct p below (find_below p below) (find_below_correct p below)
  }

theorem find_next_correct
      (p : Nat → Bool) (below : Nat) (prior : Option Nat)
    : found_spec p below prior → found_spec p (Suc below) (find_next p below prior) =
  match prior {
    None ↦ λnone. found_at_correct p below (p below) Refl none;
    Some n ↦
      λspec.
        and_intro
          (Equal Bool (lt_nat n (Suc below)) True)
          (And (Equal Bool (p n) True) (none_below p n))
          ((proof trans for lt_nat)
            n
            below
            (Suc below)
            (and_fst
              (Equal Bool (lt_nat n below) True)
              (And (Equal Bool (p n) True) (none_below p n))
              spec)
            ((proof self_suc for lt_nat) below))
          (and_snd
            (Equal Bool (lt_nat n below) True)
            (And (Equal Bool (p n) True) (none_below p n))
            spec)
  }

theorem find_next_ext
      (p : Nat → Bool) (q : Nat → Bool) (below : Nat) (prior : Option Nat)
    : Equal Bool (p below) (q below)
      → Equal (Option Nat) (find_next p below prior) (find_next q below prior) =
  match prior {
    None ↦ λsame. cong Bool (Option Nat) (p below) (q below) (found_at below) same;
    Some n ↦ λsame. cong Bool (Option Nat) (p below) (q below) (λ_. Some Nat n) same
  }

theorem found_at_correct
      (p : Nat → Bool) (below : Nat) (hit : Bool)
    : Equal Bool (p below) hit
      → none_below p below
      → found_spec p (Suc below) (found_at below hit) =
  match hit {
    True ↦
      λholds.
        λnone.
          and_intro
            (Equal Bool (lt_nat below (Suc below)) True)
            (And (Equal Bool (p below) True) (none_below p below))
            ((proof self_suc for lt_nat) below)
            (and_intro (Equal Bool (p below) True) (none_below p below) holds none);
    False ↦
      λfails.
        λnone.
          λm.
            λwithin.
              match lt_nat m below eqn : earlier {
                True ↦ none m earlier;
                False ↦
                  trans
                    Bool
                    (p m)
                    (p below)
                    False
                    (cong Nat Bool m below p (lt_suc_not_lt_same m below within earlier))
                    fails
              }
  }

theorem lt_suc_not_lt_same
      (m : Nat)
    : (b : Nat)
      → Equal Bool (lt_nat m (Suc b)) True
      → Equal Bool (lt_nat m b) False
      → Equal Nat m b =
  match m {
    Zero ↦
      λb.
        match b {
          Zero ↦ λwithin. λnot_earlier. Proved;
          Suc b2 ↦ λwithin. λnot_earlier. absurd not_earlier
        };
    Suc m2 ↦
      λb.
        match b {
          Zero ↦ λwithin. λnot_earlier. absurd within;
          Suc b2 ↦
            λwithin.
              λnot_earlier. cong Nat Nat m2 b2 Suc (lt_suc_not_lt_same m2 b2 within not_earlier)
        }
  }

theorem leq_suc_not_leq_same
      (a : Nat)
    : (b : Nat)
      → Equal Bool (leq_nat a (Suc b)) True
      → Equal Bool (leq_nat a b) False
      → Equal Nat a (Suc b) =
  match a {
    Zero ↦ λb. λwithin. λnot_earlier. absurd not_earlier;
    Suc a2 ↦
      λb.
        match b {
          Zero ↦
            match a2 {
              Zero ↦ λwithin. λnot_earlier. Proved;
              Suc a3 ↦ λwithin. λnot_earlier. absurd within
            };
          Suc b2 ↦
            λwithin.
              λnot_earlier.
                cong Nat Nat a2 (Suc b2) Suc (leq_suc_not_leq_same a2 b2 within not_earlier)
        }
  }
```

## Using it

For `λn. lt_nat two n`, a bound of ten returns `Some Nat three` and a bound
of three returns `None Nat`. A bound of `Zero` always has an empty search
range. See the checked examples and refusals in the
[bounded-search seed](
  ../../../../conformance/stdlib/optimization/seed-bounded-search.md).
The authored [refuted-range seed](
  ../../../../conformance/stdlib/optimization/seed-bounded-search-refuted-range.md)
exercises the converse with a closed client. The
[least-hit seed](
  ../../../../conformance/stdlib/optimization/seed-bounded-search-least-hit.md)
checks an in-range hit with a refuted prefix against a later-hit refusal and
its same-form control. On a tree byte-identical to
prior candidate `aaf1759d6`, Architect extracted both Ken blocks. The
accepted client checked successfully; a fresh client copy with the appended
false claim was rejected at its theorem (`KernelRejected TypeMismatch`,
span 1271..1377; 1/1, rc 0). This is candidate-local scratch evidence,
not an official conformance run.

## Laws & proofs

`find_below_sound` establishes that a reported witness is within the strict
bound and satisfies the predicate. `find_below_least` establishes that no
smaller number is a hit. `find_below_complete` establishes that an absent
answer entails no hit anywhere below the bound. Conversely,
`find_below_refuted` establishes `None` from a proof that every number strictly
below the bound fails the predicate. In the other direction,
`find_below_least_hit` establishes `Some n` from an in-range hit at `n` and
refutation of every earlier candidate. It takes no observed-outcome premise.
The first three checked proof bodies transport `find_below_correct` across
the actual result equality.
`find_below_stable` proves that an observed `Some n` persists when the bound
grows. Its proof inducts over the larger bound: an earlier bound fits within
the predecessor and reuses the induction hypothesis, or equals the new bound
and transports the observed result. No corresponding `None` law holds: a
larger range can contain a new hit.

`find_below_ext` proves that two predicates agreeing at every candidate strictly
below the bound yield the same result. It inducts over the bound, restricting
agreement to the predecessor with `lt_nat` transitivity and `self_suc` before
applying the induction hypothesis. The private `find_next_ext` transports
agreement at the new candidate through the step. No agreement at or above the
bound, equality of predicates as functions, or predicate-call-count guarantee
is needed.

`refuted_outcome_none` matches the observed result through an explicit outcome
equality. The `None` case returns that equality; the `Some n` case combines
`find_below_sound`'s in-range hit with the premise refuting all in-range hits,
yielding a contradiction. The private `hit_outcome_some` likewise binds the
observed result with its equality. A `None` result contradicts
`find_below_complete` at the given hit. For `Some k`, a hit before `n`
contradicts the refuted prefix, and a hit after `n` contradicts
`find_below_least`. If neither strict comparison holds,
`false_reverse_leq` supplies both non-strict comparisons and `leq_nat`'s
`antisym` identifies `k` with `n`; congruence transports the observed
`Some k` to `Some n`. No trichotomy helper or additional trust is needed.

`find_below_correct` inducts over the bound. Its private `found_spec` states
the stronger outcome invariant: `None` entails `none_below`, while `Some n`
records an in-range hit and no earlier hit. At each successor, `find_next`
keeps an existing least witness, or `found_at_correct` checks the new candidate.
For a failing candidate, `lt_suc_not_lt_same` identifies any member of the
extended range that was not in the prior range, letting the failure at the
new bound complete the no-hit proof. The private
`leq_suc_not_leq_same` proves the equality needed by the stability proof's
new-bound branch without adding a public order law.

## Design notes

The public contract fixes the least result or its absence, not an iteration
order or a number of predicate calls. A bound controls the domain without
introducing an equality operation on predicates. The outcome invariant,
`refuted_outcome_none`, `hit_outcome_some`, `lt_suc_not_lt_same`,
`leq_suc_not_leq_same` and `find_next_ext` are private proof scaffolding, not
additional public laws.
Unlike `argmin` over an existing nonempty list, this package searches a numeric
range and can return `None`.

## References

- [Finite bounded-search contract](
  ../../../../spec/50-stdlib/72-optimization.md#2-least-witness-below-a-finite-bound)
  — the original laws and scope.
- [Least in-range hit](
  ../../../../spec/50-stdlib/72-optimization.md#26-the-least-in-range-hit-is-returned)
  — §2.6's public converse and its premises.
- [Least-hit conformance seed](
  ../../../../conformance/stdlib/optimization/seed-bounded-search-least-hit.md)
  — an accepted law client and a false later-hit claim.
- [Least-hit package test](
  ../../../../crates/ken-elaborator/tests/cat_optimization_bounded_search_least_hit.rs)
  — generic checked law, exact trust identities, and seed controls.
- [Argmin](Argmin.ken.md) — least-cost selection from a nonempty input,
  with a different carrier.
- [Brute-force search](https://en.wikipedia.org/wiki/Brute-force_search) —
  orientation on finite exhaustive search; no algorithm or code is imported
  from it.

## Trust & derivation

The source implements the contract in `spec/50-stdlib/72-optimization.md` §2.
Its public interface consists of `find_below` and the seven named laws above;
all invariant and proof helpers are private. For the operation and its proof
family, read Definition; for a closed client, follow Using it; for proof
structure and scope, read Laws & proofs and Design notes.

The result carrier is the existing `Option Nat`. The implementation uses Nat
recursion, Boolean case analysis, transport/congruence, and the checked strict
Nat order. Its declarations add no `Axiom`, postulate, primitive, or foreign
trust; the package inherits only its imports' existing assumptions. The
[bounded-search seed](
  ../../../../conformance/stdlib/optimization/seed-bounded-search.md)
and [bound-stability seed](
  ../../../../conformance/stdlib/optimization/seed-bounded-search-bound-stability.md)
exercise the operation and the four earlier public laws in package tests.
The generic client checks the fifth law over open predicates and bounds. The
[refuted-range seed](
  ../../../../conformance/stdlib/optimization/seed-bounded-search-refuted-range.md)
is authored; its two Ken blocks received the focused scratch check described
in Using it. For the sixth law, the
[least-hit seed](
  ../../../../conformance/stdlib/optimization/seed-bounded-search-least-hit.md)
pairs an accepted law application and exact-result control with a same-form
later-hit control and a false later-hit refusal. The
[least-hit package test](
  ../../../../crates/ken-elaborator/tests/cat_optimization_bounded_search_least_hit.rs)
checks that seed and the generic §2.6 interface under a selective import,
including package ownership, a private-helper refusal, and unchanged trusted
`GlobalId` identities. The [§2.6 contract](
  ../../../../spec/50-stdlib/72-optimization.md#26-the-least-in-range-hit-is-returned)
states the public law. For the seventh law, the
[agreement seed](
  ../../../../conformance/stdlib/optimization/seed-bounded-search-agree.md)
pairs a bound-three application of `find_below_ext` with a direct same-form
control and a bound-ten refusal where the predicates disagree. The
[agreement package test](
  ../../../../crates/ken-elaborator/tests/cat_optimization_bounded_search_agree.rs)
checks the generic §2.7 public type, package ownership, private-helper
visibility, unchanged trusted `GlobalId` identities, and both seed clients.
The seeds have not been run by the official conformance runner. No time, space,
or predicate-call-count guarantee is claimed.
