# Exact finite-support statistics

> **Status: DRAFT v0 (SPEC-STATISTICS-EXACT-DISTRIBUTION-CONTRACT;
> SPEC-STATISTICS-EMPIRICAL-SAMPLE-CONTRACT).** Sections 1–2 are normative:
> §1 specifies positive natural-weight atom lists in
> `Algorithm.Statistics.Exact`; §2 specifies exact empirical observations
> of `NonEmpty q` in `Algorithm.Statistics.Empirical`. Neither section
> claims delivery of the empirical package. The contract adds no kernel,
> syntax, primitive, `Float`, `Axiom`, or `trusted_base()` entry.

## 1. Exact distributions and finite sums

An exact distribution over `q : Type` is a finite list of weighted atoms
with positive total mass. The **support is the list itself**: no `Finite q`
or `DecEq q` is needed, and the carrier need not be decidably enumerable.
Repeated atoms contribute their weights separately; zero-weight atoms
are permitted. A distribution has at least one positive weight because
its total is the successor of a natural. All declarations below use
level-zero `Type`; their `Equal` propositions and checked proofs reside
in `Omega` at level zero. `Algorithm.Statistics.Exact` is optional,
explicitly imported Ken, not a built-in numeric operation.

### 1.1 Carrier and natural-weight observations

The package uses the existing checked `Data.Numeric.Nat.Arithmetic.add`
and `mul`. Its private `total_weight` is the sum of the weights in the
atom list: it is `Zero` for `Nil` and `add weight (total_weight rest)`
for `Cons`. The constructor's proof makes that sum **strictly positive**
without a division operation:

```ken
fn total_weight (q : Type) (xs : List (Pair q Nat)) : Nat

pub data ExactDistribution (q : Type) : Type where {
  MkExactDistribution :
    (entries : List (Pair q Nat)) →
    (one_less_total : Nat) →
    Equal Nat (total_weight q entries) (Suc one_less_total) →
    ExactDistribution q
}
export MkExactDistribution

pub fn entries (q : Type) (d : ExactDistribution q) : List (Pair q Nat)
pub fn one_less_total (q : Type) (d : ExactDistribution q) : Nat
pub fn total (q : Type) (d : ExactDistribution q) : Nat
```

`entries` and `one_less_total` project the constructor fields, while
`total q d = total_weight q (entries q d)`. The constructor cannot
certify an empty or all-zero list as positive: the proof field must be
checked, not supplied by `Axiom` or an open obligation. Nothing
identifies two differently ordered lists as equal distributions;
the observable masses below sum them in list order but obey ordinary
proved natural arithmetic. In particular, two atoms with the same
vertex and weights `a` and `b` contribute `add a b`, rather than
silently deduplicating or taking only one weight.

The private `event_weight` sums each weight whose Boolean event holds,
and the private `expectation_list` sums `mul weight (score vertex)`.
Both traverse the stored entries, not a second enumeration:

```ken
fn event_weight
  (q : Type) (xs : List (Pair q Nat)) (event : q → Bool) : Nat
fn expectation_list
  (q : Type) (xs : List (Pair q Nat)) (score : q → Nat) : Nat

pub fn event_mass
  (q : Type) (d : ExactDistribution q) (event : q → Bool) : Nat
pub fn expectation_weight
  (q : Type) (d : ExactDistribution q) (score : q → Nat) : Nat
```

`event_mass q d event = event_weight q (entries q d) event` and
`expectation_weight q d score = expectation_list q (entries q d)
score`. For every atom `(x,w)`, `event_weight` adds `w` when
`event x = True` and `Zero` otherwise; `expectation_list` adds
`mul w (score x)`. The value of an event's **probability** is the
exact numerator/denominator pair `event_mass q d event` over
`total q d`; the value of a score's **expectation** is
`expectation_weight q d score` over `total q d`. The denominator is
always positive by the constructor's checked proof. These are
mathematical interpretations, **not** an exported ratio, `Pair Nat
Nat` return value, rational quotient, or numeric division. For two
such fractions `n₁/d₁` and `n₂/d₂`, equality is exactly the checked
natural equation `mul n₁ d₂ = mul n₂ d₁`; the seed exercises both
its positive and negative outcomes. A value-typed probability or
expectation requires a separate rational-number contract.

### 1.2 Named event and score builders

Ken's theorem types cannot introduce an inline `λ` in a function
argument. These five named builders are public because the law types
below use them. `bool_not`, `bool_or`, and `bool_and` belong to the
existing lawful-class package; `compose` belongs to
`Core.Classes.EffectfulClasses`, not to this package:

```ken
pub fn always_true (q : Type) (x : q) : Bool = True
pub fn always_false (q : Type) (x : q) : Bool = False
pub fn event_union
  (q : Type) (p : q → Bool) (r : q → Bool) (x : q) : Bool =
  bool_or (p x) (r x)
pub fn constant_one (q : Type) (x : q) : Nat = Suc Zero
pub fn score_add
  (q : Type) (f : q → Nat) (g : q → Nat) (x : q) : Nat =
  add (f x) (g x)
```

The complement of `p` is the existing
`compose q Bool Bool bool_not p`; no second complement operation is
introduced. There is no implicit event decidability: the event is
explicitly `q → Bool`. An arbitrary propositional predicate with no
Boolean decision is outside these operations.

### 1.3 Required checked laws

Every named law is a **proved theorem** of the package over the
operations in §1.1, not a postulate, test, or open obligation. The
constructor's proof is projected by `total_positive`. The sure and
impossible events, complements and pointwise-disjoint unions obey
exact natural-weight equations:

```ken
pub theorem total_positive (q : Type) (d : ExactDistribution q) :
  Equal Nat (total q d) (Suc (one_less_total q d))

pub theorem impossible_event (q : Type) (d : ExactDistribution q) :
  Equal Nat (event_mass q d (always_false q)) Zero

pub theorem sure_event (q : Type) (d : ExactDistribution q) :
  Equal Nat (event_mass q d (always_true q)) (total q d)

pub theorem complement_mass
  (q : Type) (d : ExactDistribution q) (p : q → Bool) :
  Equal Nat
    (add (event_mass q d p)
      (event_mass q d (compose q Bool Bool bool_not p)))
    (total q d)

pub theorem disjoint_mass
  (q : Type) (d : ExactDistribution q) (p : q → Bool) (r : q → Bool)
  (disjoint : (x : q) → Equal Bool (bool_and (p x) (r x)) False) :
  Equal Nat
    (event_mass q d (event_union q p r))
    (add (event_mass q d p) (event_mass q d r))
```

Disjointness is a **checked pointwise hypothesis**, not an inference
from two observed totals. A repeated atom remains one Boolean event
observation on each occurrence, and its weight contributes once to
that event on each occurrence. The complement law partitions every
atom even when the same vertex occurs more than once; its right
side is the full total. The positive total witness means each
numerator equation also determines an exact statement about the
corresponding probabilities with the shared denominator.

Natural-valued scores obey two further exact equations for the
**expectation numerator**, both with denominator `total q d`:

```ken
pub theorem expectation_unit (q : Type) (d : ExactDistribution q) :
  Equal Nat
    (expectation_weight q d (constant_one q)) (total q d)

pub theorem expectation_linearity
  (q : Type) (d : ExactDistribution q) (f : q → Nat) (g : q → Nat) :
  Equal Nat
    (expectation_weight q d (score_add q f g))
    (add (expectation_weight q d f) (expectation_weight q d g))
```

There is no function-extensionality assumption. A caller can use
pointwise-equal event functions or scores in these laws through the
two separately checked congruence theorems:

```ken
pub theorem event_mass_ext
  (q : Type) (d : ExactDistribution q) (p : q → Bool) (r : q → Bool)
  (same : (x : q) → Equal Bool (p x) (r x)) :
  Equal Nat (event_mass q d p) (event_mass q d r)

pub theorem expectation_weight_ext
  (q : Type) (d : ExactDistribution q) (f : q → Nat) (g : q → Nat)
  (same : (x : q) → Equal Nat (f x) (g x)) :
  Equal Nat (expectation_weight q d f) (expectation_weight q d g)
```

The two congruence laws must hold for **arbitrary** atoms and checked
pointwise proofs, not just definitionally identical functions. The
D0 development bound both head observations and used checked `cong`
for their equality and list induction for the remaining entries;
weakening either premise from `p x = r x` to `p x = p x` (or from
`f x = g x` to `f x = f x`) kernel-rejected on that development. The
private per-list lemmas and arithmetic reassociation needed to prove
the nine public laws are implementation details, not a separate
trusted source of those laws.

### 1.4 Boundary and deferred operations

`Algorithm.Statistics.Exact` contributes no primitive, `Axiom`,
unchecked assumption, `Float`, new surface form, kernel rule, or
`trusted_base()` entry. The D0 checked development accepted the
representation, sums, and all nine law bodies with the existing
Ken checker; that feasibility evidence does **not** deliver the
catalog package or run these conformance cases. Every theorem must
be checked in the eventual package before its public promise is met.

`pure` and `bind`, including monad laws, are deferred **as a pair**.
Flattening atoms naïvely is not a probability-preserving bind when
inner distributions have different totals: outer weights `1:1`
with respective inner totals `1` and `2` give the intended event
probability `1/2`, but naïve flattening gives `1/3`. A correct
common-denominator construction would also need laws modulo
proportional masses, an equivalence not exported here. An operation
named `uniform` is likewise deferred: a `Finite q` certificate may
list a vertex more than once, so it needs deduplication, not a raw
count of entries. The empirical distribution and its exact counts
and sums are outside §1 but specified in §2. Conditional probability,
variance, sampling from a distribution, approximate statistics,
continuous distributions, and every `Float`-valued result remain
outside §§1–2. The separate `64-rational.md` contract specifies an
exact non-negative rational carrier, arithmetic, equivalence and
order. It does not add value-typed probability, expectation or mean
operations to §§1–2; those statistics adapters remain deferred.
Canonical normalization and rational division are likewise outside
this chapter.

## 2. Empirical distributions over non-empty samples

The empirical tier reads an existing `NonEmpty q` as an ordered list of
observations, assigning **one natural unit of weight per occurrence**.
It is a sibling, explicitly imported package,
`Algorithm.Statistics.Empirical`, which imports
`Algorithm.Statistics.Exact` and constructs its public
`ExactDistribution q`. Neither `q` nor its sample needs `Finite q` or
`DecEq q`. A repeated observation remains a second entry with its own
unit weight. The package reuses `Data.Collections.NonEmpty`'s
`nonempty_to_list` and `nonempty_append`; it does not define another
sample carrier, observation projection, or append operation.

All declarations use level-zero `Type`; equality propositions are in
level-zero `Omega`. `Algorithm.Statistics.Empirical` is ordinary
checked Ken with no extra trust. The names and theorem types in this
section are its required public surface, not a claim that the package
has landed.

### 2.1 Counts, sums and the unit-weight distribution

Let `xs = nonempty_to_list q sample`. `unit_weights q xs` preserves the
order of `xs`, replacing each observation `x` by the pair
`(x, Suc Zero)`. It maps `Nil` to `Nil` and `Cons x rest` to a unit pair
followed by `unit_weights q rest`; it never merges equal observations.
The other operations read the **same** `xs`. `sample_size` is
`length q xs`. `event_count` adds `Suc Zero` for each `x` whose
Boolean `event x` is `True`, and `Zero` otherwise. `score_sum` adds
`score x` for each `x`, where `score : q → Nat`. The sums are naturals,
not returned probabilities, expectations, ratios, or means.

```ken
pub fn sample_size (q : Type) (sample : NonEmpty q) : Nat
pub fn event_count
  (q : Type) (sample : NonEmpty q) (event : q → Bool) : Nat
pub fn score_sum
  (q : Type) (sample : NonEmpty q) (score : q → Nat) : Nat
pub fn unit_weights
  (q : Type) (xs : List q) : List (Pair q Nat)
pub fn empirical_distribution
  (q : Type) (sample : NonEmpty q) : ExactDistribution q
```

The distribution's entries are precisely `unit_weights q xs`; its
positive-total proof is checked when constructing §1's
`MkExactDistribution`, not asserted after construction. A non-empty
sample supplies its structural head and possibly empty tail. The
checked `nonempty_to_list_shape` theorem relates the abstract carrier
to their `Cons` list without exporting `NonEmptyCons`. Induction on
the tail proves that the sum of the unit weights is the successor of
its length, so even a one-observation sample has total `Suc Zero`.
The §1 total is that sum, not a separately stored, potentially stale
sample count.

### 2.2 Required checked laws

Every displayed law is a **proved theorem** of the empirical package:
its body is rechecked by the kernel and cannot be an `Axiom`, test,
or unclosed obligation. The representation and the three observations
are connected to §1 on the same `empirical_distribution`, so its
positive `total` is the common denominator of the corresponding
probability and expectation interpretations.

```ken
pub theorem empirical_entries (q : Type) (sample : NonEmpty q) :
  Equal (List (Pair q Nat))
    (entries q (empirical_distribution q sample))
    (unit_weights q (nonempty_to_list q sample))

pub theorem empirical_total_size (q : Type) (sample : NonEmpty q) :
  Equal Nat
    (total q (empirical_distribution q sample))
    (sample_size q sample)

pub theorem empirical_event_count
  (q : Type) (sample : NonEmpty q) (event : q → Bool) :
  Equal Nat
    (event_mass q (empirical_distribution q sample) event)
    (event_count q sample event)

pub theorem empirical_score_sum
  (q : Type) (sample : NonEmpty q) (score : q → Nat) :
  Equal Nat
    (expectation_weight q (empirical_distribution q sample) score)
    (score_sum q sample score)
```

The equalities hold for arbitrary Boolean events and natural scores,
not only for closed fixtures. A repeated `x` contributes once per
appearance to `event_count` and `score_sum`; `empirical_entries`
prevents replacing the list by a set or a keyed map. The size theorem
uses `nonempty_to_list_shape` to transport the checked unit-weight
sum to the public view. The mass and score theorems use the same
transport and the §1 entry folds. No function-extensionality rule is
needed.

`nonempty_append q xs ys` concatenates the observations in their
existing order. Its checked `nonempty_append::list_view` law reduces
all three additive properties to the ordinary list append and
natural addition laws. Each input is non-empty, and its repeated
observations remain distinct occurrences after append.

```ken
pub theorem sample_size_append
  (q : Type) (xs : NonEmpty q) (ys : NonEmpty q) :
  Equal Nat
    (sample_size q (nonempty_append q xs ys))
    (add (sample_size q xs) (sample_size q ys))

pub theorem event_count_append
  (q : Type) (xs : NonEmpty q) (ys : NonEmpty q)
  (event : q → Bool) :
  Equal Nat
    (event_count q (nonempty_append q xs ys) event)
    (add (event_count q xs event) (event_count q ys event))

pub theorem score_sum_append
  (q : Type) (xs : NonEmpty q) (ys : NonEmpty q)
  (score : q → Nat) :
  Equal Nat
    (score_sum q (nonempty_append q xs ys) score)
    (add (score_sum q xs score) (score_sum q ys score))
```

### 2.3 Boundary and deferred operations

This tier adds a checked mapping from observed samples into §1's
positive natural-weight distributions, not a sampling operation that
generates observations from a distribution. It does not change §1's
carrier, proofs, event/score semantics, or exact-ratio interpretation.
No mean as a returned numeric value, variance, conditional
probability, `pure`/`bind`, `uniform`, rational carrier, approximate
tier, or `Float` result is part of §2. The separate `64-rational.md`
contract supplies a carrier and exact algebra, not a value-returning
Empirical operation. In particular, `score_sum q sample score` over
`sample_size q sample` is an **exact numerator/denominator
interpretation**, not a Ken-valued division.
A post-ruling checked development establishes feasibility of the
seven proof bodies against the landed §1 and `NonEmpty` interfaces;
only a checked CAT package will deliver them as public laws. The
accompanying conformance seed is authored, not evidence that CAT or
CI ran it.
