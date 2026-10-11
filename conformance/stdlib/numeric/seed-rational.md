# Exact non-negative rational values

Format: `../../README.md`.
Spec: `spec/50-stdlib/64-rational.md`.

These five cases are **authored behavioral oracles**, not executed
conformance results. First-party scratch `Data.Numeric.Rational`
(SHA256 `019f291370cdd0e3835b5d1c365fbe706e19da1aa30786eddb7dc0947634bda2`)
and its separate imported consumer checked rc=0 with the existing
Oct 6 CLI on base `3ab95b8a7`, using scratch Nat provider SHA256
`bf343e74c5cd0a774f89bff30f903f4ac7c17ee53516c5351dc1af7e6925589a`.
The D0/AC-0b subset checks positive denominators, both denominator
products, equivalence reflexivity/symmetry/transitivity, addition
respect, four zero/one unit laws and order reflexivity. The separately
extracted fixture code and twenty closed claims (SHA256
`30bcacb5413fefa6ef9eacf1a2f4d21da935ada44ae7ca7fb7b8e6b2fc4582b2`)
checked rc=0. A false fraction-equivalence, false reverse order and
false product each kernel-rejected; a false structural
`Equal Rational half two_quarters` rejected while the positive
`rational_equiv` control checked. Three weakened generic premises
rejected too. These are not delivery of the Nat provider or rational
CAT package, seed execution, a fresh CLI build, Cargo tests, CI or
publication. Each case has promise class **durable invariant**.

## Shared fixtures

`MkRational n k` means `n/(k+1)`. The spelling for `2/4` has a
**different carrier term** from the spelling for `1/2`; the
specified value comparison is `rational_equiv`, not `Equal Rational`.
The fixtures use only public construction, arithmetic, equivalence,
and order operations. Their expected arithmetic is independent of
the functions being checked.

```ken
import Data.Numeric.Rational
  (Rational, MkRational, rational_zero, rational_one,
   numerator, denominator, rational_of_nat,
   rational_add, rational_mul, rational_equiv, rational_leq)

const half : Rational = MkRational (Suc Zero) (Suc Zero)
const two_quarters : Rational =
  MkRational (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))
const third : Rational = MkRational (Suc Zero) (Suc (Suc Zero))
const five_sixths : Rational =
  MkRational (Suc (Suc (Suc (Suc (Suc Zero)))))
    (Suc (Suc (Suc (Suc (Suc Zero)))))
const one_sixth : Rational =
  MkRational (Suc Zero) (Suc (Suc (Suc (Suc (Suc Zero)))))
const zero_sevenths : Rational =
  MkRational Zero (Suc (Suc (Suc (Suc (Suc (Suc Zero))))))
```

The `Proved` assertions below are closed checker-dischargeable
claims over these fixtures. They do not replace the generic proved
laws in §64. There is no gcd, normalization, division, quotient,
`Float`, `Axiom`, or structure-equality claim between different
spellings.

## Equivalence is value equality, not pair identity

### stdlib/numeric/rational-equivalent-not-identical

- spec: `spec/50-stdlib/64-rational.md` §§1–2.
- promise class: durable invariant.
- given: `half = 1/2`, `two_quarters = 2/4`, and the control
  `third = 1/3`. Hold the numerator of `half` and `third` at one.
- expect: `rational_equiv half two_quarters` is proved by
  `mul 1 4 = mul 2 2 = 4`; `rational_equiv half third` is false,
  as `mul 1 3 = 3 ≠ mul 1 2 = 2`. The former does **not** assert
  `Equal Rational half two_quarters`. `denominator half = 2` and
  `denominator two_quarters = 4` remain observable and positive.
- measured: equal versus unequal cross-products for two pairs,
  with different source spellings and a shared-numerator control.
- claimed: `rational_equiv` compares non-negative exact values,
  without pretending two carrier representations are identical.
- the gap: closed numerals cannot establish transitivity or all
  consumers' congruence; those are checked CAT theorem obligations.
- why: raw pair equality rejects the equivalent pair, while raw
  numerator equality accepts the inequivalent pair.

## Exact addition

### stdlib/numeric/rational-sum-one-half-one-third

- spec: `spec/50-stdlib/64-rational.md` §§2–2.1.
- promise class: durable invariant.
- given: `half` and `third`, with distinct positive denominators.
- expect: `rational_equiv (rational_add half third) five_sixths`
  holds: the sum numerator is `1·3 + 1·2 = 5`, and its denominator
  is `2·3 = 6`. `denominator (rational_add half third) = 6` is
  checked independently of the result's equality. Addition with
  `two_quarters` in place of `half` remains equivalent via the
  generic `rational_add::respect` law.
- measured: a cross-denominator sum, both operation denominator and
  equivalence to an independent fixed value, plus a spelling change.
- claimed: addition cross-multiplies both inputs, and its value is
  invariant under equivalent representations.
- the gap: one closed sum does not prove the generic respect or
  distributivity law; the CAT bodies must prove those for all values.
- why: direct addition of numerator and denominator yields `2/5`,
  and a last-operand-only sum yields `1/3`; neither equals `5/6`.

## Exact multiplication

### stdlib/numeric/rational-product-one-sixth

- spec: `spec/50-stdlib/64-rational.md` §§2–2.1.
- promise class: durable invariant.
- given: the same `half` and `third` as the addition case.
- expect: `rational_equiv (rational_mul half third) one_sixth`
  holds: `1·1 = 1` and `2·3 = 6`. Its exposed denominator is `6`.
  Replacing `half` with `two_quarters` keeps the product equivalent
  by `rational_mul::respect`, although the raw stored pair differs.
- measured: product numerator, positive product denominator and
  relation-based equality to an independent one-sixth fixture.
- claimed: multiplication is exact and representation-invariant.
- the gap: the fixture is not a proof of generic commutativity,
  associativity, annihilation or operand respect.
- why: adding the numerators instead returns `2/6`; retaining only
  one denominator returns `1/2` or `1/3`.

## Order compares values

### stdlib/numeric/rational-order-opposite-directions

- spec: `spec/50-stdlib/64-rational.md` §§2 and 2.2.
- promise class: durable invariant.
- given: `third`, `half`, and the equivalent spelling
  `two_quarters`. Keep all denominators positive.
- expect: `rational_leq third half = True` (`1·2 ≤ 1·3`), while
  `rational_leq half third = False` (`1·3 ≤ 1·2` is false).
  `rational_leq half two_quarters = True` and the reverse is
  `True`; their antisymmetric conclusion is `rational_equiv half
  two_quarters`, **not** structural `Equal Rational`. The generic
  `rational_leq::respect` theorem requires that replacing `half`
  by `two_quarters` not alter either comparison with `third`.
- measured: opposite Boolean results for strictly ordered values,
  and both positive results for equivalent but differently spelled
  values.
- claimed: exact order is a well-defined total order on values
  rather than a lexicographic or raw-pair comparison.
- the gap: two directions and one tie do not prove generic
  transitivity, totality or equivalence respect; CAT owes all three.
- why: a reversed comparator flips the strict pair; comparing raw
  numerators reports a tie between unequal `third` and `half`.

## Zero and one identities

### stdlib/numeric/rational-units-across-spellings

- spec: `spec/50-stdlib/64-rational.md` §§1–2.1.
- promise class: durable invariant.
- given: `zero_sevenths` and `rational_zero` as equivalent zeros,
  with `half` nonzero; use `rational_one` for multiplication.
- expect: `rational_equiv zero_sevenths rational_zero` holds.
  Both `rational_add rational_zero half` and `rational_add half
  rational_zero` are equivalent to `half`, and so is
  `rational_add zero_sevenths half` under `rational_add::respect`.
  Both `rational_mul rational_one half` and `rational_mul half
  rational_one` are equivalent to `half`. Each returned
  denominator stays positive; no result is required to be the
  identical carrier term `half`.
- measured: both orientations of each identity on a nonzero value,
  plus a distinct zero spelling exercising the equivalence bridge.
- claimed: zero and one are two-sided rational **value** identities;
  operation respect extends that promise to equivalent spellings.
- the gap: six closed observations do not prove the four generic
  unit laws or general zero annihilation; their CAT bodies remain
  checked obligations.
- why: a raw-pair-only identity fails on `zero_sevenths`; treating
  `0/7` as a nonzero numerator fails the addition bridge.

## Coverage boundary

The five cases fix one equivalent and one inequivalent spelling,
addition and multiplication at unequal denominators, both order
outcomes with an equivalence tie, and two-sided identities with a
noncanonical zero. The CAT package must still check **every** public
law, including equivalence transitivity, all three respect laws,
associativity, commutativity, distributivity, order transitivity,
antisymmetry, totality, and the Nat prerequisite proofs. The seed
does not require a canonical form, structural equality of equivalent
fractions, a statistics value-returning adapter, or a package-delivery
milestone. Every asserted value is a fixed-fixture arithmetic fact;
none inspects repository text.
