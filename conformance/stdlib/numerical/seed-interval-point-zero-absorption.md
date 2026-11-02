# Interval point zero absorbs multiplication

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §11.

These cases are authored source-shaped closed behavioral oracles for the
prospective `Algorithm.Numeric.RationalInterval` laws in §11. The accepted
client instantiates both laws on `i = [1/2,1]` and on the inverted interval
`[1,1/2]`. These finite applications do not establish the generic laws.

Status: Foundation reports its named package test
`cat_numeric_interval_point_zero_absorption` passed 3/3 on the actual CV seed
fence in the assembled working tree, with trusted `GlobalId` identities
reported at 114→114. This is Foundation's working-tree package-test evidence;
CV did not run it. It is not an official conformance-runner run or an exact
committed-tip gate. No CI, publication, or landing result is claimed. The
Architect's AC-0 report separately records scratch checks of the generic laws
and a separate closed client. It also reports that `Refl` refuses both target claims,
so the paired oracle uses `Proved` for each; those are design observations, not
execution of this seed.

`MkRational n k` represents `n/(Suc k)`. The product is point zero by value
for either interval because `interval_mul` multiplies corresponding
endpoints. The seed does not claim structural equality of intervals or
canonical spelling of rational endpoints.

```ken
import Data.Numeric.Rational
  (Rational, MkRational, rational_zero, rational_one)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_point, interval_mul, interval_inside,
   interval_mul_zero_l_inside, interval_mul_zero_l_rev_inside)

const half : Rational = MkRational (Suc Zero) (Suc Zero)
const left_box : Interval = MkInterval half rational_one
const inverted : Interval = MkInterval rational_one half
const zero_point : Interval = interval_point rational_zero
const one_point : Interval = interval_point rational_one
const left_zero_product : Interval = interval_mul zero_point left_box

theorem left_zero_product_inside_zero_left_box :
  IsTrue (interval_inside left_zero_product zero_point) =
  interval_mul_zero_l_inside left_box

theorem zero_inside_left_zero_product_left_box :
  IsTrue (interval_inside zero_point left_zero_product) =
  interval_mul_zero_l_rev_inside left_box

theorem left_zero_product_inside_zero_inverted :
  IsTrue
    (interval_inside (interval_mul zero_point inverted) zero_point) =
  interval_mul_zero_l_inside inverted

theorem zero_inside_left_zero_product_inverted :
  IsTrue
    (interval_inside zero_point (interval_mul zero_point inverted)) =
  interval_mul_zero_l_rev_inside inverted

theorem point_zero_target_control :
  IsTrue (interval_inside left_zero_product zero_point) = Proved
```

## Left-zero product lies inside point zero

### stdlib/numerical/interval-mul-zero-left-inside

- spec: `spec/50-stdlib/68-numerical.md` §11.
- promise class: durable invariant.
- given: `left_box = [1/2,1]` and the inverted interval `[1,1/2]`.
- expect: `interval_mul_zero_l_inside` applies to each interval and proves
  the product is inside `interval_point rational_zero`.
- measured: the accepted client applies the public law to both fixtures.
- claimed: multiplying the left endpoint-wise interval by point zero yields
  a product included in point zero, without an endpoint-order premise.
- the gap: two closed inputs do not establish the universal quantifier over
  every `Interval`.
- controls: the separate same-form point-zero target control is accepted.
- why: the law is applied at both a nonempty and an inverted interval.

## Point zero lies inside the left-zero product

### stdlib/numerical/interval-mul-zero-left-reverse-inside

- spec: `spec/50-stdlib/68-numerical.md` §11.
- promise class: durable invariant.
- given: `left_box = [1/2,1]` and the inverted interval `[1,1/2]`.
- expect: `interval_mul_zero_l_rev_inside` applies to each interval and proves
  point zero is inside the product.
- measured: the accepted client applies the reverse public law to both
  fixtures.
- claimed: the reverse endpoint-value inclusion also holds without an
  endpoint-order premise.
- the gap: these applications do not prove the universal law.
- controls: the accepted forward and reverse applications share the same
  public product on each fixture.
- why: the inverted input checks that the law does not assume nonemptiness.

## Refuse a zero product inside point one

Append the negative declaration alone to the unchanged accepted client above.
The positive control and refusal use the same product and claim form; only the
target point changes.

### stdlib/numerical/reject-zero-product-inside-point-one

- spec: `spec/50-stdlib/68-numerical.md` §11.
- promise class: durable invariant.
- given: the product of point zero with `left_box = [1/2,1]`.
- expect: the same-form point-zero target control accepts. The point-one claim
  below refuses with `KernelRejected TypeMismatch` at its own claim.
- measured: both rows use `Proved`; only the target point differs. The product
  has zero-valued endpoints, while the point-one target's lower bound is one.
- claimed: the product is included in point zero, not point one.
- the gap: this one false target does not characterize all incorrect targets.
- controls: the accepted `point_zero_target_control` has the same product,
  source shape, and proof terminal as the negative.
- why: changing only the target makes refusal attributable to the claimed
  inclusion, rather than a malformed product or an unavailable theorem.

```ken
theorem reject_zero_product_inside_point_one :
  IsTrue (interval_inside left_zero_product one_point) = Proved
```
