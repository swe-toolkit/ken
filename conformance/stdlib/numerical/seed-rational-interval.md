# Exact non-negative rational intervals

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §1.

These cases are **authored source-shaped closed behavioral oracles**
for the prospective `Algorithm.Numeric.RationalInterval` module. The
D0 development checks its six **generic** laws only in scratch with
two public monotonicity proofs added to a scratch Rational provider;
no catalog package or conformance suite is delivered by this seed.
Every case has promise class **durable invariant**: the expected
behavior concerns exact rational values, inclusive bounds and stored
endpoint spellings, not a snapshot of the current repository. A
closed instance of a generic law does not prove its quantifiers.

The exact Architect D0 source (SHA256
`4890e265533a5e1417c09c2ff699200fb50b13723f530149ffa3787ea18f3470`)
rejects against the landed Rational provider with `UnboundName
rational_add_mono`. A scratch Rational copy was extended **only**
with the Architect monotonicity appendix (SHA256
`6abde236eb2adb6af1860101753e3ef3b41879a1d92113007901d3c26b403c0a`),
including its two checked public proofs and private helpers. The
resulting scratch Rational source has SHA256
`2a45cc9e10d78248b11baf38c849852276710929c702fb1434a85e774c8b11b8`;
then the unchanged D0 development checks rc=0. No source in this
spec WP changes Rational or the catalog.

The separate client below (SHA256
`a54299d7125a9281ebdd5b773ba0223d324ed202a659774201f58eda9e31a839`)
checks **24** closed assertions, rc=0, using `ken check` on base
`8e0fc5911` after `scripts/ken-cargo build -p ken-cli --bin ken`
(rc=0). Appending any one of B1–B5 **alone** to the same positive
client rejects rc=1: B1–B3 give `KernelRejected TypeMismatch`, and
B4–B5 give `TypeMismatch` at `Refl`. Substituting `Proved` for B4's
rejected `Refl` also rejects `KernelRejected TypeMismatch`; neither
terminal proves equality of differently spelled intervals. An
independently selected scratch mutation of the interval-add lower
endpoint to `rational_add a d` while leaving the checked law
unchanged rejects at `interval_add_encloses` with
`KernelRejected TypeMismatch`. The module was restored byte-for-byte
and both its generic development and the positive client rechecked
rc=0. These are **scratch checks**, not shipped-suite execution,
Cargo tests, CI, catalog publication, or a `trusted_base()`
measurement.

## Shared fixtures and positive assertions

`MkRational n k` denotes `n/(Suc k)`, so `half` and
`two_quarters` are equivalent **values** but different stored
representations. `left_box = [1/2,1]` and `right_box = [1,2]`;
`three_quarters` and `three_halves` lie in them, respectively;
the positive client checks each membership separately. Their exact
sum is `9/4`, which lies in their correct sum interval
`[3/2,3]`. `wrong_sum = [3/2,2]` excludes that same `9/4`;
B1 deliberately claims otherwise. The positive assertion for
`wrong_sum` makes the refusal nonvacuous. The direct endpoint
oracles name independently constructed rational constants rather
than deriving their expected corners by calling the interval
operation under test. `Equal Interval` is structural endpoint
identity, whereas `rational_equiv` compares rational values.

This block is extracted verbatim to the scratch public client. The
six `law_*` cases instantiate checked D0 laws; their finite
applications do not replace the generic proofs.

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Data.Numeric.Rational
  (Rational, MkRational, rational_add, rational_mul, rational_equiv)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_contains, interval_add, interval_mul,
   interval_point, interval_add_encloses, interval_mul_encloses,
   interval_point_contains, interval_point_sound, interval_add_point,
   interval_mul_point)

const zero : Rational = MkRational Zero Zero
const half : Rational = MkRational (Suc Zero) (Suc Zero)
const one : Rational = MkRational (Suc Zero) Zero
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const three : Rational = MkRational (Suc (Suc (Suc Zero))) Zero
const two_quarters : Rational =
  MkRational (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))
const three_quarters : Rational =
  MkRational (Suc (Suc (Suc Zero))) (Suc (Suc (Suc Zero)))
const three_halves : Rational = MkRational (Suc (Suc (Suc Zero))) (Suc Zero)

const left_box : Interval = MkInterval half one
const right_box : Interval = MkInterval one two
const wrong_sum : Interval =
  MkInterval (rational_add half one) (rational_add one one)
const member_sum : Rational = rational_add three_quarters three_halves
const member_product : Rational = rational_mul three_quarters three_halves

theorem lower_in : Equal Bool (interval_contains left_box half) True = Proved
theorem upper_in : Equal Bool (interval_contains left_box one) True = Proved
theorem below_out : Equal Bool (interval_contains left_box zero) False = Proved
theorem above_out : Equal Bool (interval_contains left_box two) False = Proved

theorem sum_endpoints :
  Equal Interval (interval_add left_box right_box)
    (MkInterval three_halves three) = Refl

theorem product_endpoints :
  Equal Interval (interval_mul left_box right_box)
    (MkInterval half two) = Refl

theorem point_endpoints :
  Equal Interval (interval_point half) (MkInterval half half) = Refl

theorem left_selected_member :
  Equal Bool (interval_contains left_box three_quarters) True = Proved

theorem right_selected_member :
  Equal Bool (interval_contains right_box three_halves) True = Proved

theorem empty_input_sum_point :
  Equal Bool
    (interval_contains (interval_add (MkInterval one half) left_box) three_halves)
    True = Proved

theorem sum_member :
  Equal Bool (interval_contains (interval_add left_box right_box) member_sum) True = Proved

theorem product_member :
  Equal Bool (interval_contains (interval_mul left_box right_box) member_product) True = Proved

theorem point_member :
  Equal Bool (interval_contains (interval_point half) two_quarters) True = Proved

theorem value_equivalent : rational_equiv half two_quarters = Proved

theorem point_excludes_one :
  Equal Bool (interval_contains (interval_point half) one) False = Proved

theorem empty_excludes_half :
  Equal Bool (interval_contains (MkInterval one half) half) False = Proved

theorem empty_excludes_one :
  Equal Bool (interval_contains (MkInterval one half) one) False = Proved

theorem wrong_sum_excludes_member_sum :
  Equal Bool (interval_contains wrong_sum member_sum) False = Proved

theorem law_add_encloses :
  IsTrue (interval_contains (interval_add left_box right_box) member_sum) =
  interval_add_encloses left_box right_box three_quarters three_halves Proved Proved

theorem law_mul_encloses :
  IsTrue (interval_contains (interval_mul left_box right_box) member_product) =
  interval_mul_encloses left_box right_box three_quarters three_halves Proved Proved

theorem law_point_contains :
  IsTrue (interval_contains (interval_point three_halves) three_halves) =
  interval_point_contains three_halves

theorem law_point_sound :
  rational_equiv half two_quarters = interval_point_sound half two_quarters Proved

theorem law_add_point :
  Equal Interval (interval_add (interval_point half) (interval_point one))
    (interval_point (rational_add half one)) = interval_add_point half one

theorem law_mul_point :
  Equal Interval (interval_mul (interval_point half) (interval_point two))
    (interval_point (rational_mul half two)) = interval_mul_point half two
```

## Membership compares both inclusive bounds

### stdlib/numerical/interval-membership

- spec: `spec/50-stdlib/68-numerical.md` §1.1.
- promise class: durable invariant.
- given: `[1/2,1]`, with `0` below and `2` above; also the
  inverted interval `[1,1/2]`.
- expect: `half` and `one` are members of `[1/2,1]`;
  `zero` and `two` are not. Neither endpoint of `[1,1/2]` is
  a member of that empty interval.
- measured: two inclusive endpoints, an outside point on each
  side, and both would-be endpoints of inverted bounds.
- claimed: membership is the conjunction of the two exact
  rational-value order tests.
- gap: finite values cannot prove the predicate on all rational
  arguments; the D0 definition fixes that behavior.
- why: omitting either bound admits the corresponding outside point;
  treating inverted bounds as an error instead of a value fails the
  empty controls.

## Addition and multiplication preserve exact endpoint arithmetic

### stdlib/numerical/interval-operations

- spec: `spec/50-stdlib/68-numerical.md` §1.2.
- promise class: durable invariant.
- given: `[1/2,1]` and `[1,2]`, with members `3/4` and
  `3/2`; and the point interval at `half`.
- expect: `interval_add` has stored endpoints `[3/2,3]`,
  `interval_mul` has `[1/2,2]`, and `interval_point half`
  has exactly the endpoint spelling `(half,half)`.
  Both arithmetic results contain the respective sum and product
  of the selected members.
- measured: independently spelled output endpoints, two input
  memberships and membership of their exact arithmetic results.
- claimed: result endpoints are corresponding-endpoint arithmetic,
  not a minimum of four products or a rounded approximation.
- gap: these examples do not prove generic enclosure; the two
  checked enclosure laws carry every admissible pair of members.
- why: substituting the upper bound for the first input's lower
  bound or dropping an input term changes the stored endpoints.

## An empty operand need not yield an empty arithmetic result

### stdlib/numerical/empty-operand-arithmetic

- spec: `spec/50-stdlib/68-numerical.md` §§1.1–1.3.
- promise class: durable invariant.
- given: the empty `[1,1/2]` and the nonempty `[1/2,1]`.
- expect: `3/2` belongs to their endpoint-wise sum `[3/2,3/2]`,
  although no element belongs to the inverted input.
- measured: a directly checked member of the arithmetic result,
  alongside two nonmembership controls for the empty input.
- claimed: total endpoint arithmetic makes **no** general promise
  that empty operands yield empty results; enclosure laws have
  membership premises, which an empty operand cannot furnish.
- gap: this closed case does not characterize all combinations of
  empty inputs and does not weaken the checked enclosure laws.
- why: assuming that empty operands automatically propagate
  emptiness would contradict this valid endpoint calculation.

## Equivalent values are not identical endpoint spellings

### stdlib/numerical/point-values

- spec: `spec/50-stdlib/68-numerical.md` §§1.1–1.3.
- promise class: durable invariant.
- given: `half=1/2`, `two_quarters=2/4`, and `one=1`.
- expect: `rational_equiv half two_quarters` and membership of
  `two_quarters` in `interval_point half` both check;
  `one` is excluded from that point interval.
- measured: value-order membership across distinct rational
  spellings, versus a nonmember at a different value.
- claimed: the checked membership comparison admits this equivalent
  spelling, while the `Equal Interval` point equations preserve the
  exact stored endpoints and do not identify differently spelled
  points.
- gap: a closed equivalent pair does not prove point soundness
  generically; B4 separately refuses structural equality of the
  two point-interval spellings.
- why: using raw structural equality to implement membership would
  wrongly exclude `two_quarters`.

## All six generic laws have closed instances

### stdlib/numerical/interval-laws

- spec: `spec/50-stdlib/68-numerical.md` §1.3.
- promise class: durable invariant.
- given: the same positive fixtures and two `Proved` membership
  premises for each enclosure law.
- expect: `law_add_encloses` and `law_mul_encloses` apply both
  generic enclosure theorems; `law_point_contains` applies its
  reflexive theorem; `law_point_sound` returns
  `rational_equiv half two_quarters`; `law_add_point` and
  `law_mul_point` apply the two structural point equations.
- measured: one closed application per **distinct** public law.
- claimed: these six theorem interfaces are callable with the
  required premises and give their exact result types.
- gap: the finite applications reuse the scratch D0 proofs; they
  are not an independent construction of generic proof bodies.
- why: each law appears as a real imported declaration, not a
  local restatement or an axiom in the client.

## Refusals, appended one at a time

Each following block is appended **alone** to the unchanged
positive client against the same scratch module. A rejected
proposition has force alongside that positive control; none is a
standalone parse-error example. The expected values are fixed by
independently constructed literals and same-input Boolean controls.

### stdlib/numerical/reject-sum-excluding-member-sum

- spec: `spec/50-stdlib/68-numerical.md` §§1.2–1.3.
- promise class: durable invariant.
- given: `3/4` lies in `[1/2,1]` and `3/2` lies in `[1,2]`;
  their `9/4` sum is in the correct `[3/2,3]` result but is
  outside the truncated `[3/2,2]` `wrong_sum`.
- expect: claiming the `9/4` sum belongs to `wrong_sum` rejects.
- measured: `KernelRejected TypeMismatch` against the same
  `member_sum` whose correct inclusion and incorrect exclusion
  already check in the positive client.
- claimed: an interval sum may not omit the sum of two members.
- gap: this one truncated upper endpoint does not establish the
  generic enclosure law; the checked D0 proof supplies it.
- why: the only changed axis is the claimed result interval's
  upper endpoint; the inputs and member sum are unchanged.

```ken
-- Reject: 9/4 exceeds the incorrect upper endpoint 2.
theorem reject_sum_excluding_sum : IsTrue (interval_contains wrong_sum member_sum) = Proved
```

### stdlib/numerical/reject-point-contains-one

- spec: `spec/50-stdlib/68-numerical.md` §1.1–1.2.
- promise class: durable invariant.
- given: the point interval at `half` and the distinct value `one`.
- expect: claiming membership for `one` rejects.
- measured: opposite of `point_excludes_one` in the positive client.
- claimed: a point interval contains its value and equivalent
  spellings, not every rational value.
- gap: one false point cannot characterize all rational values.
- why: the control contains `two_quarters` but excludes `one`.

```ken
-- Reject: one is not equivalent to half.
theorem reject_point_contains_one : IsTrue (interval_contains (interval_point half) one) = Proved
```

### stdlib/numerical/reject-interval-contains-two

- spec: `spec/50-stdlib/68-numerical.md` §1.1.
- promise class: durable invariant.
- given: `[1/2,1]` and `two` above its upper bound.
- expect: claiming membership for `two` rejects.
- measured: opposite of the positive `above_out` assertion.
- claimed: an upper endpoint is inclusive but not unbounded.
- gap: one closed case does not prove the comparator for all values.
- why: `upper_in` checks the endpoint itself on the same interval.

```ken
-- Reject: two exceeds the inclusive upper endpoint one.
theorem reject_interval_contains_two : IsTrue (interval_contains left_box two) = Proved
```

### stdlib/numerical/reject-equal-point-spellings

- spec: `spec/50-stdlib/68-numerical.md` §§1.1–1.3.
- promise class: durable invariant.
- given: equivalent values `half` and `two_quarters` with
  different constructors' fields.
- expect: a structural `Equal Interval` proof between their
  two point intervals rejects, even though both hold the same
  rational value. Replacing `Refl` with `Proved` also rejects.
- measured: `TypeMismatch` at `Refl`, and separately
  `KernelRejected TypeMismatch` at `Proved`.
- claimed: point intervals do not acquire canonical structural
  equality from their members' `rational_equiv`.
- gap: a particular pair does not decide equality for all intervals.
- why: both proof terminals reject while the separate value-
  equivalence and cross-spelling membership controls succeed.

```ken
-- Reject: equivalent values have distinct stored endpoint spellings.
theorem reject_equal_point_spellings : Equal Interval (interval_point half) (interval_point two_quarters) = Refl
```

### stdlib/numerical/reject-wrong-sum-endpoints

- spec: `spec/50-stdlib/68-numerical.md` §1.2.
- promise class: durable invariant.
- given: `[1/2,1] + [1,2]` and a proposed high endpoint `2`.
- expect: equality with `[3/2,2]` rejects; the independent
  positive `sum_endpoints` checks `[3/2,3]` on the same input.
- measured: `TypeMismatch` at `Refl` with a different high term.
- claimed: the endpoint-wise sum must include the upper+upper sum.
- gap: this one structural case cannot prove arbitrary sums.
- why: it isolates the high endpoint without changing the low.

```ken
-- Reject: the upper endpoint must be three, not two.
theorem reject_wrong_sum_endpoints : Equal Interval (interval_add left_box right_box) (MkInterval three_halves two) = Refl
```
