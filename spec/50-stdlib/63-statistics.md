# Exact finite-support statistics

> **Status: DRAFT v0 (SPEC-STATISTICS-EXACT-DISTRIBUTION-CONTRACT;
> SPEC-STATISTICS-EMPIRICAL-SAMPLE-CONTRACT;
> SPEC-STATISTICS-RATIONAL-VALUES-CONTRACT).** Sections 1–3 are normative:
> §1 specifies positive natural-weight atom lists in
> `Algorithm.Statistics.Exact`; §2 specifies exact empirical observations
> of `NonEmpty q` in `Algorithm.Statistics.Empirical`; §3 specifies their
> exact Rational-valued probability, expectation, and mean adapters.
> This contract adds no kernel, syntax, primitive, `Float`, `Axiom`, or
> `trusted_base()` entry. A specification is not CAT delivery.

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
representation, sums, and all nine law bodies with the then-existing
Ken checker; that feasibility evidence did **not** itself deliver the
catalog package
or run conformance. The landed Exact package now exports the nine
checked laws; §3 adds separate value-typed obligations.

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
outside §§1–2. The separate `64-rational.md` contract specifies the
landed exact non-negative rational carrier, arithmetic, equivalence
and order. Sections 1–2 do not export value-typed statistics
operations; §3 specifies the Rational-valued adapters separately,
without changing any declaration or law above. Canonical
normalization and rational division remain outside this chapter.

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
tier, or `Float` result is part of §2. Section 3 separately specifies
`empirical_mean` using §64's Rational carrier and §1's `expectation`;
it does not redefine `score_sum q sample score` over
`sample_size q sample` as a Ken-valued division.

A post-ruling checked development established feasibility of the
seven proof bodies against the §1 and `NonEmpty` interfaces. The
landed Empirical package now exports those checked laws; its authored
conformance seed alone establishes neither their delivery nor CI.

## 3. Rational-valued statistics adapters

This section extends the existing `Algorithm.Statistics.Exact` and
`Algorithm.Statistics.Empirical` modules with exactly three operations
and twelve proved laws. It leaves the §§1–2 declarations untouched.
`Data.Numeric.Rational` supplies the existing unnormalized, non-negative
carrier, `MkRational`, `rational_equiv`, arithmetic, and Boolean order.
The value represented by `MkRational n k` is `n/(Suc k)`: the second
field is **not** a denominator. No division, normalized fraction, or
new sample/distribution carrier is introduced.

The existing public total witnesses do the conversion without a new
predecessor operation. `total_positive q d` proves `total q d = Suc
(one_less_total q d)`. An empirical sample uses its existing
`empirical_distribution q sample`; `empirical_total_size` then
identifies that distribution's positive total with
`sample_size q sample`. In particular, `empirical_mean` uses the
same `expectation` construction, not an independently constructed
rational with a second positivity proof. Every public law in §3
requires a checked body without `Axiom`, postulate, or open proof
obligation, at the same trust boundary as §§1–2.

### 3.1 Exact event probabilities and expectations

`Algorithm.Statistics.Exact` imports the existing Rational surface
`Rational`, `MkRational`, `numerator`, `denominator`,
`rational_equiv`, `rational_leq`, `rational_add`, and `rational_one`.
It adds `leq_nat` to its existing LawfulClasses import,
`mul_add_distrib_l` to its Nat.Arithmetic import, and imports
`leq_nat_add_left_bound` from Nat.Order. No import adds a primitive
or changes the public surface of those providers.

```ken
pub fn probability
  (q : Type) (d : ExactDistribution q) (event : q → Bool) : Rational =
  MkRational (event_mass q d event) (one_less_total q d)

pub fn expectation
  (q : Type) (d : ExactDistribution q) (score : q → Nat) : Rational =
  MkRational (expectation_weight q d score) (one_less_total q d)
```

The four projection laws connect the actual returned value to the
§1 pair. In particular, the denominator laws are proved from the
stored positive-total witness, not from an unchecked equality or a
zero-permitting natural denominator. Neither operation silently
normalizes, rounds, or changes the event/score fold.

```ken
pub theorem probability_numerator
  (q : Type) (d : ExactDistribution q) (event : q → Bool) :
  Equal Nat (numerator (probability q d event)) (event_mass q d event)

pub theorem probability_denominator
  (q : Type) (d : ExactDistribution q) (event : q → Bool) :
  Equal Nat (denominator (probability q d event)) (total q d)

pub theorem expectation_numerator
  (q : Type) (d : ExactDistribution q) (score : q → Nat) :
  Equal Nat
    (numerator (expectation q d score))
    (expectation_weight q d score)

pub theorem expectation_denominator
  (q : Type) (d : ExactDistribution q) (score : q → Nat) :
  Equal Nat (denominator (expectation q d score)) (total q d)
```

The forward bridges compare an adapter result with any differently
spelled positive fraction `MkRational n k`. Their premise is exactly
the **cross-product equality** between that fraction and §1's
numerator/positive-total interpretation. Their conclusion is
`rational_equiv`, never structural `Equal Rational`. The comparison
fraction need not be reduced. No reverse bridge is exported: its
cross equation follows from the existing denominator law and the
definition of `rational_equiv`, without a second public name.

```ken
pub theorem probability_cross_bridge
  (q : Type) (d : ExactDistribution q) (event : q → Bool)
  (n : Nat) (k : Nat)
  (same : Equal Nat
    (mul (event_mass q d event) (Suc k))
    (mul n (total q d))) :
  rational_equiv (probability q d event) (MkRational n k)

pub theorem expectation_cross_bridge
  (q : Type) (d : ExactDistribution q) (score : q → Nat)
  (n : Nat) (k : Nat)
  (same : Equal Nat
    (mul (expectation_weight q d score) (Suc k))
    (mul n (total q d))) :
  rational_equiv (expectation q d score) (MkRational n k)
```

### 3.2 Empirical mean without a second rational construction

`Algorithm.Statistics.Empirical` adds `expectation` and
`expectation_denominator` to its existing Exact import, and imports
`Rational`, `MkRational`, `numerator`, `denominator`, and
`rational_equiv` from Rational. The new operation accepts the
existing `NonEmpty q`, not a second sample type. Its score is still
natural-valued and its result is non-negative and exact.

```ken
pub fn empirical_mean
  (q : Type) (sample : NonEmpty q) (score : q → Nat) : Rational =
  expectation q (empirical_distribution q sample) score

pub theorem empirical_mean_numerator
  (q : Type) (sample : NonEmpty q) (score : q → Nat) :
  Equal Nat
    (numerator (empirical_mean q sample score))
    (score_sum q sample score)

pub theorem empirical_mean_denominator
  (q : Type) (sample : NonEmpty q) (score : q → Nat) :
  Equal Nat
    (denominator (empirical_mean q sample score))
    (sample_size q sample)

pub theorem empirical_mean_cross_bridge
  (q : Type) (sample : NonEmpty q) (score : q → Nat)
  (n : Nat) (k : Nat)
  (same : Equal Nat
    (mul (score_sum q sample score) (Suc k))
    (mul n (sample_size q sample))) :
  rational_equiv (empirical_mean q sample score) (MkRational n k)
```

The numerator law uses §2's `empirical_score_sum`, while the
denominator law composes §1's `expectation_denominator` with §2's
`empirical_total_size`. The forward bridge transports **both**
checked equations to the §2 cross product; a closed three-observation
fixture is not a substitute for those generic proofs. The sample's
non-emptiness makes its size a positive denominator through the
checked `empirical_distribution`, not a separate assumed count.

### 3.3 Probability bounds and complements

A Boolean event selects some of the distribution's natural weights.
The existing `complement_mass` partitions **all** entries, including
repeated atoms and zero weights. With Nat.Order's checked
`leq_nat_add_left_bound`, this proves the mass is at most the positive
total. The returned probability is therefore at most `rational_one`
under Rational's exact Boolean value order. Adding the event and its
complement gives one under `rational_equiv`, regardless of the
unreduced spelling of their sum.

```ken
pub theorem probability_mass_bound
  (q : Type) (d : ExactDistribution q) (event : q → Bool) :
  Equal Bool (leq_nat (event_mass q d event) (total q d)) True

pub theorem probability_complement_one
  (q : Type) (d : ExactDistribution q) (event : q → Bool) :
  rational_equiv
    (rational_add
      (probability q d event)
      (probability q d (compose q Bool Bool bool_not event)))
    rational_one
```

`probability_leq_one` needs no third private copy of a Nat-order
congruence helper. The following checked D0 body uses the two
`cong` steps directly to eliminate multiplication by one, then
transports the mass bound across `probability_denominator`:

```ken
pub theorem probability_leq_one
    (q : Type) (d : ExactDistribution q) (event : q → Bool)
  : Equal Bool (rational_leq (probability q d event) rational_one) True =
  let
    mass = event_mass q d event;
    den = denominator (probability q d event);
    raw_comparison = leq_nat (mul mass (Suc Zero)) (mul (Suc Zero) den);
    reduced_comparison = leq_nat mass den
  in
    trans Bool raw_comparison reduced_comparison True
      (trans Bool
        raw_comparison
        (leq_nat mass (mul (Suc Zero) den))
        reduced_comparison
        (cong Nat Bool (mul mass (Suc Zero)) mass
          (λv. leq_nat v (mul (Suc Zero) den))
          ((proof one_r for mul) mass))
        (cong Nat Bool (mul (Suc Zero) den) den (leq_nat mass)
          ((proof one_l for mul) den)))
      (trans Bool reduced_comparison (leq_nat mass (total q d)) True
        (cong Nat Bool den (total q d) (leq_nat mass)
          (probability_denominator q d event))
        (probability_mass_bound q d event))
```

The displayed body elaborates in the D0 development; the eventual
catalog package must check every named body, not only this witness.
The complement proof also uses Nat distributivity and Rational's
checked `denominator_product` for `rational_add`; it does not add a
second complement builder or change Rational.

### 3.4 Trust boundary and exclusions

The ruled public inventory is **eleven Exact declarations** (two
operations and nine proved laws) and **four Empirical declarations**
(one operation and three proved laws). No other public name or
existing-declaration change is required; there is no duplicate
Nat-order congruence helper. All value equalities use
`rational_equiv`, not carrier identity. Sections 1–2's raw natural
numerators remain available unchanged. The D0
checked both extended source-shaped modules and a separate imported
consumer with the existing checker, and rejected a weakened cross
premise and two false closed value claims. This establishes
feasibility only: it does not deliver the CAT additions, execute the
new seed against them, rebuild the CLI, or establish CI green.

The natural score of an expectation is unbounded, so **no**
`expectation ≤ rational_one` law holds: a checked closed result is
`4/3`. No score-maximum bound or reverse cross bridge is in the
required public inventory. `pure`/`bind`, `uniform`, conditional
probability, variance, sampling, signed values, subtraction or
division, normalized or reduced forms, quotient equality,
structural `Equal Rational` for differently spelled fractions,
`Float`, and approximate statistics remain outside §3. No law may
be justified by an `Axiom`, primitive, new surface or kernel rule,
or unchecked proof assumption.
