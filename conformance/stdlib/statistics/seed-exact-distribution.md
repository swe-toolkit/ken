# Exact finite-support distributions

Format: `../../README.md`.
Spec: `spec/50-stdlib/63-statistics.md` §1.

These are **authored behavioral oracles**, not executed results. The
first-party D0 scratch (SHA256
`8ba4809e8517038fe5a5224d849053543a52ef45c871bfec276066eb4be02487`)
checked the general representation and nine law bodies with an existing
Oct 6 CLI, not a freshly built current-base CLI. Its two independent
weakened-premise mutants each kernel-rejected. A separate scratch copy
with the shared fixtures and seventeen closed equations checked rc=0;
an unequal-ratio equality mutation kernel-rejected. Those probes
check fixture typing and specified closed values, **not** execution
of these cases against a delivered `Algorithm.Statistics.Exact`.
No Cargo tests, CI or publication are claimed. The cases observe
computed natural weights and cross-multiplied ratios; they do not
test a source spelling, an
implementation milestone, or an unexported rational type. Each case
has promise class **durable invariant**.

## Shared fixtures

Use the public `MkExactDistribution` constructor with checked positive
total proofs. `bool_true` is the Boolean event that accepts only
`True`; `bool_false` accepts only `False`. No `Finite Bool` or
`DecEq Bool` certificate is supplied. The fixture's atom order is
not the definition of probability:

```ken
const uniform_atoms : List (Pair Bool Nat) =
  Cons (Pair Bool Nat) (mk_pair Bool Nat True (Suc Zero))
    (Cons (Pair Bool Nat) (mk_pair Bool Nat False (Suc Zero))
      (Nil (Pair Bool Nat)))
const uniform : ExactDistribution Bool =
  MkExactDistribution Bool uniform_atoms (Suc Zero) Proved

const skew_atoms : List (Pair Bool Nat) =
  Cons (Pair Bool Nat) (mk_pair Bool Nat True (Suc Zero))
    (Cons (Pair Bool Nat) (mk_pair Bool Nat False (Suc (Suc Zero)))
      (Nil (Pair Bool Nat)))
const skew : ExactDistribution Bool =
  MkExactDistribution Bool skew_atoms (Suc (Suc Zero)) Proved

const doubled_atoms : List (Pair Bool Nat) =
  Cons (Pair Bool Nat) (mk_pair Bool Nat True (Suc (Suc Zero)))
    (Cons (Pair Bool Nat) (mk_pair Bool Nat False (Suc (Suc Zero)))
      (Nil (Pair Bool Nat)))
const doubled : ExactDistribution Bool =
  MkExactDistribution Bool doubled_atoms (Suc (Suc (Suc Zero))) Proved

const repeated_atoms : List (Pair Bool Nat) =
  Cons (Pair Bool Nat) (mk_pair Bool Nat True (Suc Zero))
    (Cons (Pair Bool Nat) (mk_pair Bool Nat True (Suc (Suc Zero)))
      (Nil (Pair Bool Nat)))
const repeated : ExactDistribution Bool =
  MkExactDistribution Bool repeated_atoms (Suc (Suc Zero)) Proved

const zero_atoms : List (Pair Bool Nat) =
  Cons (Pair Bool Nat) (mk_pair Bool Nat True Zero)
    (Cons (Pair Bool Nat) (mk_pair Bool Nat False (Suc Zero))
      (Nil (Pair Bool Nat)))
const zero_weight : ExactDistribution Bool =
  MkExactDistribution Bool zero_atoms Zero Proved

const bool_true : Bool → Bool = λx. x
const bool_false : Bool → Bool = λx. bool_not x
const score_true : Bool → Nat =
  λx. match x { True ↦ Suc Zero; False ↦ Zero }
```

Use `mul` from `Data.Numeric.Nat.Arithmetic` for cross products.
All ratios in the cases are **observations** `(numerator,total)`, not
Ken values returned by an unimplemented rational API. `Proved` in
fixtures denotes a checked closed proof of the stated positive total;
it is not permission to leave the constructor's proof open.

## Event probability

### stdlib/statistics/uniform-true-probability

- spec: `spec/50-stdlib/63-statistics.md` §1.1 and §1.3.
- promise class: durable invariant.
- given: `uniform` and `bool_true` from the shared fixtures.
- expect: `total Bool uniform = 2`, `one_less_total Bool uniform =
  1`, `event_mass Bool uniform bool_true = 1`, and `event_mass Bool
  uniform bool_false = 1`. The positive-total proof is checked;
  `total_positive` has a nonvacuous closed instance. The exact
  probability of `True` is numerator `1` over denominator `2`.
- measured: two different Boolean event masses and their shared total.
- claimed: two equal positive atom weights give probability `1/2`
  without a rational, a carrier enumeration, or a numeric primitive.
- the gap: the closed Bool pair does not prove general mass or total
  laws; the CAT package must check them for arbitrary atom lists.
- why: a counter that reports list length rather than summing
  natural weights happens to pass here but fails the skew control.

### stdlib/statistics/skew-and-cross-multiplied-ratios

- spec: `spec/50-stdlib/63-statistics.md` §1.1 and §1.3.
- promise class: durable invariant.
- given: compare `skew`, `uniform`, and `doubled`, keeping the Bool
  carrier and the `bool_true` event fixed. `doubled` multiplies both
  of `uniform`'s weights by two; `skew` changes only the `False`
  atom's weight to two.
- expect: `skew` has `True` mass `1`, `False` mass `2`, and total
  `3`; `doubled` has `True` mass `2`, `False` mass `2`, and total
  `4`. `uniform` True probability `1/2` and `doubled` True
  probability `2/4` are equal by `mul 1 4 = mul 2 2 = 4`.
  `uniform` `1/2` and `skew` `1/3` are not equal: `mul 1 3 = 3`
  but `mul 1 2 = 2`.
- measured: both positive and negative cross-product comparisons
  of observed numerators and totals, with one controlled weight
  change and one common scaling of both weights.
- claimed: ratio equality uses **cross multiplication**, not raw
  numerator equality, list length, or approximate floating values.
- the gap: closed ratio observations do not supply an exported
  equality function or value-typed rational; those are deferred.
- why: raw numerator equality mistakes uniform `1/2` and skew
  `1/3` for equal, while raw pair equality mistakes `1/2` and
  doubled `2/4` for different.

### stdlib/statistics/sure-and-impossible-events

- spec: `spec/50-stdlib/63-statistics.md` §1.3.
- promise class: durable invariant.
- given: `skew` with public `always_true Bool` and
  `always_false Bool`; keep the entries and total fixed.
- expect: `event_mass Bool skew (always_true Bool) = 3` and
  `event_mass Bool skew (always_false Bool) = 0`; their exact
  probabilities are `3/3` and `0/3`. Both `sure_event` and
  `impossible_event` have checked closed instances.
- measured: opposite event masses on the same nonempty,
  unequal-weight distribution with the same positive denominator.
- claimed: an impossible event has zero mass, while the sure
  event includes every atom's full weight.
- the gap: these observations do not prove either law for an
  arbitrary event function and arbitrary atom list.
- why: counting only the first entry yields sure mass `1` instead
  of `3`; accepting an impossible event yields positive mass.

### stdlib/statistics/complement-partitions-skew-total

- spec: `spec/50-stdlib/63-statistics.md` §1.2–§1.3.
- promise class: durable invariant.
- given: `skew`, `bool_true`, and its complement
  `compose Bool Bool Bool bool_not bool_true` from the existing
  class package. Compare against `always_true Bool` and the
  independent `bool_false` fixture.
- expect: masses for the event and complement are `1` and `2`;
  `add 1 2 = total Bool skew = 3`. The complement's mass agrees
  with `bool_false`, and `complement_mass` has a checked closed
  instance. Both events use denominator `3`.
- measured: a nonsymmetric positive pair partitioning the full
  total, not a vacuous empty or uniform distribution.
- claimed: complement is the existing `compose` of `bool_not`
  with the predicate; it does not compute a second, incompatible
  event interpretation.
- the gap: one Bool fixture does not prove general complement,
  disjoint additivity, or pointwise congruence; their CAT theorem
  bodies must be checked independently.
- why: mistakenly summing `p` twice produces `1 + 1 = 2`, not
  `3`. A wrong complement that accepts only the first atom also
  returns the wrong `False` mass.

### stdlib/statistics/disjoint-union-counts-each-weight-once

- spec: `spec/50-stdlib/63-statistics.md` §1.3.
- promise class: durable invariant.
- given: `skew`, `bool_true` and `bool_false`, with checked
  pointwise evidence that `bool_and (bool_true x) (bool_false x)
  = False` for both Bool constructors.
- expect: `event_mass Bool skew (event_union Bool bool_true
  bool_false) = 3`, equal to `add 1 2`. `disjoint_mass` applies
  with the pointwise evidence. Neither constituent alone has
  mass `3`.
- measured: a disjoint union on one unequal-weight distribution
  where the two sides contribute different, nonzero masses.
- claimed: the additivity theorem consumes actual pointwise
  disjointness and adds weights for both alternatives once.
- the gap: Bool case evidence is not a proof for arbitrary `q`;
  the theorem's generic hypothesis and body must check in CAT.
- why: treating Boolean `or` like intersection gives zero;
  counting only the left event gives `1`, not `3`.

## Expectations and repeated atoms

### stdlib/statistics/skew-natural-score-expectation

- spec: `spec/50-stdlib/63-statistics.md` §1.1 and §1.3.
- promise class: durable invariant.
- given: `skew` and `score_true`, which scores `True` as `1`
  and `False` as `0`. Compare with `constant_one Bool` on the
  same distribution and keep the denominator `total Bool skew`.
- expect: `expectation_weight Bool skew score_true = 1` with
  denominator `3`; `expectation_weight Bool skew
  (constant_one Bool) = 3` with that **same** denominator.
  `expectation_unit` has a checked closed instance.
- measured: two score numerators on one skew distribution; the
  zero-score atom is present with weight `2` and contributes zero.
- claimed: expectation multiplies **each atom's weight by its
  natural score**, then sums, without normalizing to an integer
  quotient or returning `Float`.
- the gap: the closed score does not prove generic expectation
  linearity or pointwise score congruence; their checked CAT
  theorems carry those obligations.
- why: summing scores without weights returns `1` for this
  fixture too, but fails the constant-one control (`2` vs `3`).

### stdlib/statistics/repeated-atom-adds-not-deduplicates

- spec: `spec/50-stdlib/63-statistics.md` §1.1–§1.3.
- promise class: durable invariant.
- given: `repeated` has two **distinct list entries for the
  same `True` vertex**, of weights `1` and `2`; `skew` has
  weights `1` and `2` on different vertices. Use both
  `bool_true` and `bool_false` on each, with total `3` fixed.
- expect: on `repeated`, `total = 3`, `event_mass bool_true =
  3`, and `event_mass bool_false = 0`; on `skew`, `total = 3`,
  `event_mass bool_true = 1`, and `event_mass bool_false = 2`.
  The repeated `True` entries contribute `add 1 2`, not `1`
  or `2` alone.
- measured: equal totals with different atom locations and
  event masses; one condition changes only whether the second
  weight belongs to `True` or `False`.
- claimed: repeated atoms are **additive** and are not silently
  deduplicated by state identity or a `Finite` enumeration.
- the gap: two Bool lists do not prove every repeated-key
  arrangement; the CAT definitions and generic proofs must
  retain the atom-list semantics.
- why: last-wins or first-wins lookup of the repeated `True`
  loses weight and returns `2` or `1` rather than `3`.

### stdlib/statistics/zero-weight-atom-remains-valid

- spec: `spec/50-stdlib/63-statistics.md` §1.1 and §1.3.
- promise class: durable invariant.
- given: compare `zero_weight` with `uniform`, holding the Bool
  carrier, `False` atom's weight `1`, and entry order fixed. Only
  the `True` atom's weight changes from `1` to `Zero`.
- expect: `zero_weight` is constructible with a checked positive
  total proof. Its total is `1`; `event_mass bool_true = 0`,
  `event_mass bool_false = 1`. In `uniform`, the respective values
  are `2`, `1`, and `1`. The `True` atom **remains in the list**
  despite its zero weight.
- measured: valid positive total, unchanged list length, and
  negative/positive masses for two events across one changed weight.
- claimed: a zero-weight atom is allowed and contributes no event
  mass; only an **all-zero total** is forbidden by the constructor.
- the gap: this fixture does not establish zero-weight invariance
  for all scores and events; the generic sums and checked laws must
  still hold for arbitrary natural weights.
- why: a constructor that rejects any zero-weight atom refuses this
  valid distribution, while list-length counting reports positive
  `True` mass despite its zero weight.

## Coverage boundary

The eight cases have distinct observations: normalization by positive
natural total, unequal skew weights, positive and negative ratio
comparison, sure/impossible events, complement and disjoint
partitioning, natural-score expectation, additive repeated atoms,
and an admitted zero-weight atom. `event_mass_ext`, `expectation_weight_ext`, and
`expectation_linearity` remain **generic checked CAT obligations**;
closed examples do not replace them. The ruled D0 scratch separately
checked both congruence bodies and kernel-rejected two
premise-weakening mutants. The seed does not claim `pure`, `bind`,
`uniform`, conditional probability, variance, continuous laws,
approximate statistics, or a rational-valued API.
