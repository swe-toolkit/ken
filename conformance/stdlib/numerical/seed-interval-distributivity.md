# Rational interval distributivity

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §4, as stated by the
AC-1 candidate `0ad93e3136b779b12d18c5dcff42c31abc873845`.

These are source-shaped closed behavioral oracles for the four §4
interval-inclusion laws. The AC-1 spec candidate is routed but not yet
landed, and the corresponding four catalog theorem exports are absent
from authoring base `77352e68e1bad7c17c1e7e4751e25c54cc8cc692`. This file therefore
records intended behavior only: no Ken fence or conformance runner was
executed. Do not route or land this seed before AC-1 lands. Even after
that, seed execution requires a verified base with the four exports.
No CI, publication, landing, or trust measurement is claimed.

Each case has promise class **durable invariant**. The assertions concern
non-negative rational endpoint values and the public inclusion relation,
not a snapshot of the current catalog. In particular, the oracle does not
assert `Equal Interval`: equal endpoint values need not have identical
Rational spellings.

## Distinct-endpoint fixtures and positive controls

`MkRational n k` denotes `n/(Suc k)`. The intervals are `ii = [1,2]`,
`jj = [1,3]`, and `kk = [1/2,2]`; each has distinct lower and upper
endpoints. The left-hand expressions both have values `[3/2,10]`:
`ii*(jj+kk)` and `ii*jj + ii*kk`. The right-hand expressions have the
same endpoint values. The retained product `ii*jj` is `[1,6]`, so the
full left-hand result is not inside it: its upper endpoint `10` exceeds
`6`.

The client directly checks the four inclusion directions and applies
all four public law exports to the same fixtures. These accepting controls
are paired with the refused dropped-product claim below; they establish
that the fixture and intended API are usable, rather than merely showing
that some unrelated error rejects the negative block.

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Data.Numeric.Rational (Rational, MkRational)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_add, interval_mul, interval_inside,
   interval_distrib_l_inside, interval_factor_l_inside,
   interval_distrib_r_inside, interval_factor_r_inside)

const half : Rational = MkRational (Suc Zero) (Suc Zero)
const one : Rational = MkRational (Suc Zero) Zero
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const three : Rational = MkRational (Suc (Suc (Suc Zero))) Zero

const ii : Interval = MkInterval one two
const jj : Interval = MkInterval one three
const kk : Interval = MkInterval half two

const left_product : Interval = interval_mul ii jj
const left_distributed : Interval = interval_mul ii (interval_add jj kk)
const left_factored : Interval =
  interval_add (interval_mul ii jj) (interval_mul ii kk)
const right_distributed : Interval =
  interval_mul (interval_add jj kk) ii
const right_factored : Interval =
  interval_add (interval_mul jj ii) (interval_mul kk ii)

theorem left_distributivity_control :
  Equal Bool (interval_inside left_distributed left_factored) True = Proved

theorem left_factor_control :
  Equal Bool (interval_inside left_factored left_distributed) True = Proved

theorem right_distributivity_control :
  Equal Bool (interval_inside right_distributed right_factored) True = Proved

theorem right_factor_control :
  Equal Bool (interval_inside right_factored right_distributed) True = Proved

theorem dropped_product_control :
  Equal Bool (interval_inside left_distributed left_product) False = Proved

theorem left_distributivity_law_instance :
  IsTrue (interval_inside left_distributed left_factored) =
  interval_distrib_l_inside ii jj kk

theorem left_factor_law_instance :
  IsTrue (interval_inside left_factored left_distributed) =
  interval_factor_l_inside ii jj kk

theorem right_distributivity_law_instance :
  IsTrue (interval_inside right_distributed right_factored) =
  interval_distrib_r_inside ii jj kk

theorem right_factor_law_instance :
  IsTrue (interval_inside right_factored right_distributed) =
  interval_factor_r_inside ii jj kk
```

## Refuse the claim that drops one product

- spec: `spec/50-stdlib/68-numerical.md` §4, as stated by the AC-1
  candidate `0ad93e3136b779b12d18c5dcff42c31abc873845`.
- promise class: durable invariant.
- given: the same `ii`, `jj`, and `kk` as the positive client; the
  proposed containing interval is only `ii*jj`, dropping `ii*kk`.
- expect: claiming `ii*(jj+kk)` is inside `ii*jj` rejects. The full
  result has upper endpoint `10`, while the retained product's upper
  endpoint is `6`.
- measured: not run in this seed. The positive client checks both left
  inclusion directions and directly checks that the dropped-product
  inclusion is `False`.
- claimed: a product term cannot be omitted from the distributive sum.
- gap: this one closed refusal does not establish the generic laws.
- why: it uses the same distinct-endpoint inputs as the accepting
  controls, and the only changed claim is the RHS with one product
  omitted. The Architect's AC-2 scratch probe reported K0004 for this
  false claim; that scratch result is not execution of this seed.

```ken
-- Reject: the RHS omits ii*kk and ends at 6, below the LHS upper endpoint 10.
theorem reject_dropped_left_product :
  Equal Bool (interval_inside left_distributed left_product) True = Proved
```
