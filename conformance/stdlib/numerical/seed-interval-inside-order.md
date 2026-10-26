# Interval inclusion order and operand swaps

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §5.

These are closed, source-shaped behavioral oracles. Each case has promise
class **durable invariant**. A closed instance does not prove any generic
quantifier. `MkRational n k` denotes `n/(Suc k)`; `half` is `1/2`.
The §5 exports are absent from the candidate base `d6c05223e`.
No check against shipped §5 exports or official conformance run is claimed.

The non-degenerate chain is `inner=[1,2]` inside
`middle=[1/2,3]` inside `outer=[0,4]`. Its six endpoints are
pairwise distinct. The direct `inner`-inside-`outer` check and the
same-input transitivity application both accept; the reverse
`outer`-inside-`inner` Boolean comparison is `False` and the
appended proof of its `IsTrue` proposition must reject.

The two operands `[1/2,1]` and `[2,3]` have different endpoints.
Independent value controls compare each operation's result with the
expected endpoint interval in **both** directions of `interval_inside`:
sum `[5/2,4]`, product `[1,3]`, hull `[1/2,3]`, and inverted
intersection `[2,1]`. Direct Boolean controls compare each operation
with its operand-swapped result in **both** directions. Each of the
four public laws is also applied at `(left_operand, right_operand)`
and at `(right_operand, left_operand)`; this tests the converse by
instantiation, not by assuming an extra public law. In particular,
the intersection control does not infer corner inclusion from members
of an empty interval.

## Accepted client

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Data.Numeric.Rational (Rational, MkRational)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_add, interval_mul, interval_hull,
   interval_intersect, interval_inside, interval_inside_trans,
   interval_add_comm_inside, interval_mul_comm_inside,
   interval_hull_comm_inside, interval_intersect_comm_inside)

const zero : Rational = MkRational Zero Zero
const half : Rational = MkRational (Suc Zero) (Suc Zero)
const one : Rational = MkRational (Suc Zero) Zero
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const three : Rational = MkRational (Suc (Suc (Suc Zero))) Zero
const four : Rational = MkRational (Suc (Suc (Suc (Suc Zero)))) Zero
const five_halves : Rational =
  MkRational (Suc (Suc (Suc (Suc (Suc Zero))))) (Suc Zero)

const inner : Interval = MkInterval one two
const middle : Interval = MkInterval half three
const outer : Interval = MkInterval zero four
const left_operand : Interval = MkInterval half one
const right_operand : Interval = MkInterval two three
const expected_sum : Interval = MkInterval five_halves four
const expected_product : Interval = MkInterval one three
const expected_hull : Interval = MkInterval half three
const expected_intersection : Interval = MkInterval two one

theorem inner_in_middle :
  Equal Bool (interval_inside inner middle) True = Proved

theorem middle_in_outer :
  Equal Bool (interval_inside middle outer) True = Proved

theorem inner_in_outer_direct :
  Equal Bool (interval_inside inner outer) True = Proved

theorem inner_in_outer_by_trans : IsTrue (interval_inside inner outer) =
  interval_inside_trans inner middle outer Proved Proved

theorem outer_not_in_inner :
  Equal Bool (interval_inside outer inner) False = Proved

theorem outer_not_in_middle :
  Equal Bool (interval_inside outer middle) False = Proved

theorem sum_value_lower_upper :
  Equal Bool (interval_inside
    (interval_add left_operand right_operand) expected_sum) True = Proved

theorem sum_value_reverse :
  Equal Bool (interval_inside
    expected_sum (interval_add left_operand right_operand)) True = Proved

theorem product_value_lower_upper :
  Equal Bool (interval_inside
    (interval_mul left_operand right_operand) expected_product) True = Proved

theorem product_value_reverse :
  Equal Bool (interval_inside
    expected_product (interval_mul left_operand right_operand)) True = Proved

theorem hull_value_lower_upper :
  Equal Bool (interval_inside
    (interval_hull left_operand right_operand) expected_hull) True = Proved

theorem hull_value_reverse :
  Equal Bool (interval_inside
    expected_hull (interval_hull left_operand right_operand)) True = Proved

theorem intersection_value_lower_upper :
  Equal Bool (interval_inside
    (interval_intersect left_operand right_operand) expected_intersection)
    True = Proved

theorem intersection_value_reverse :
  Equal Bool (interval_inside
    expected_intersection (interval_intersect left_operand right_operand))
    True = Proved

theorem add_swap_direct :
  Equal Bool (interval_inside
    (interval_add left_operand right_operand)
    (interval_add right_operand left_operand)) True = Proved

theorem add_swap_reverse_direct :
  Equal Bool (interval_inside
    (interval_add right_operand left_operand)
    (interval_add left_operand right_operand)) True = Proved

theorem add_swap_law :
  IsTrue (interval_inside
    (interval_add left_operand right_operand)
    (interval_add right_operand left_operand)) =
  interval_add_comm_inside left_operand right_operand

theorem add_swap_reverse_law :
  IsTrue (interval_inside
    (interval_add right_operand left_operand)
    (interval_add left_operand right_operand)) =
  interval_add_comm_inside right_operand left_operand

theorem mul_swap_direct :
  Equal Bool (interval_inside
    (interval_mul left_operand right_operand)
    (interval_mul right_operand left_operand)) True = Proved

theorem mul_swap_reverse_direct :
  Equal Bool (interval_inside
    (interval_mul right_operand left_operand)
    (interval_mul left_operand right_operand)) True = Proved

theorem mul_swap_law :
  IsTrue (interval_inside
    (interval_mul left_operand right_operand)
    (interval_mul right_operand left_operand)) =
  interval_mul_comm_inside left_operand right_operand

theorem mul_swap_reverse_law :
  IsTrue (interval_inside
    (interval_mul right_operand left_operand)
    (interval_mul left_operand right_operand)) =
  interval_mul_comm_inside right_operand left_operand

theorem hull_swap_direct :
  Equal Bool (interval_inside
    (interval_hull left_operand right_operand)
    (interval_hull right_operand left_operand)) True = Proved

theorem hull_swap_reverse_direct :
  Equal Bool (interval_inside
    (interval_hull right_operand left_operand)
    (interval_hull left_operand right_operand)) True = Proved

theorem hull_swap_law :
  IsTrue (interval_inside
    (interval_hull left_operand right_operand)
    (interval_hull right_operand left_operand)) =
  interval_hull_comm_inside left_operand right_operand

theorem hull_swap_reverse_law :
  IsTrue (interval_inside
    (interval_hull right_operand left_operand)
    (interval_hull left_operand right_operand)) =
  interval_hull_comm_inside right_operand left_operand

theorem intersect_swap_direct :
  Equal Bool (interval_inside
    (interval_intersect left_operand right_operand)
    (interval_intersect right_operand left_operand)) True = Proved

theorem intersect_swap_reverse_direct :
  Equal Bool (interval_inside
    (interval_intersect right_operand left_operand)
    (interval_intersect left_operand right_operand)) True = Proved

theorem intersect_swap_law :
  IsTrue (interval_inside
    (interval_intersect left_operand right_operand)
    (interval_intersect right_operand left_operand)) =
  interval_intersect_comm_inside left_operand right_operand

theorem intersect_swap_reverse_law :
  IsTrue (interval_inside
    (interval_intersect right_operand left_operand)
    (interval_intersect left_operand right_operand)) =
  interval_intersect_comm_inside right_operand left_operand
```

## Inclusion transitivity on a strict chain

### stdlib/numerical/interval-inside-trans

- spec: `spec/50-stdlib/68-numerical.md` §5.
- promise class: durable invariant.
- given: three different intervals with `inner` inside `middle`
  inside `outer`; no validity or membership argument is passed.
- expect: the direct Boolean inclusion and the theorem conclusion both
  accept, while the reversed Boolean comparison is `False`.
- measured: the accepted client checks both premises independently,
  the direct conclusion, the theorem application, and the reverse
  result as `False`.
- claimed: `interval_inside` is transitive for every interval triple.
- gap: one strict chain does not establish the generic law.
- why: a theorem taking the premises in the wrong order will not have
  both at this call site; the isolated refusal below probes reversal.

## Four swapped operations on non-identical endpoints

### stdlib/numerical/interval-swaps-inside

- spec: `spec/50-stdlib/68-numerical.md` §5.
- promise class: durable invariant.
- given: `[1/2,1]` and `[2,3]`, with different lower and upper
  endpoints; their intersection is inverted.
- expect: each of the four swap-law applications accepts in both
  argument orders and direct endpoint comparisons return `True`.
- measured: eight direct swap comparisons, eight corresponding law
  applications, and independent bidirectional endpoint-value oracles
  for sum `[5/2,4]`, product `[1,3]`, hull `[1/2,3]`, and inverted
  intersection `[2,1]`.
- claimed: each operation's swapped result includes its unswapped
  result by stored-endpoint **values**, for every operand pair.
- gap: closed checks do not establish generic quantifiers, structural
  equality, or equivalence of all member sets; mutual inclusion is
  derived generically only by instantiating the same law twice.
- why: result-value oracles do not use a swap law, so a faulty law
  application is not its own expected-result oracle. The inverted
  intersection exercises the endpoint rather than member-set rule.

## Isolated refused claims

Append **one** of the following blocks to a fresh copy of the entire
accepted client. Each has accepted same-operand controls in that
prefix and must reject at its own appended theorem, not during
imports, construction, or an earlier declaration.

### stdlib/numerical/reject-reversed-chain

- spec: `spec/50-stdlib/68-numerical.md` §§3.1, 5.
- promise class: durable invariant.
- given: the same `outer` and `inner` as the accepted strict chain.
- expect: `outer` inside `inner` is refused; the same-input
  `inner`-inside-`outer` direct and transitivity controls accept.
- measured: the prefix checks the reversed Boolean result is `False`;
  the appended false proof must reject at `reject_outer_inside_inner`.
- claimed: transitivity does not reverse inclusion.
- gap: this closed counterexample does not establish an arbitrary
  non-inclusion criterion.
- why: all intervals are valid and ordered; failure is the reversed
  endpoint comparisons, not an unrelated missing import or fixture.

```ken
-- Refuse: the enclosing interval is not inside the narrow one.
theorem reject_outer_inside_inner :
  IsTrue (interval_inside outer inner) = Proved
```

### stdlib/numerical/reject-swapped-transitivity-premise

- spec: `spec/50-stdlib/68-numerical.md` §5.
- promise class: durable invariant.
- given: the same accepted `inner`, `middle`, and `outer` chain.
- expect: supplying `inner` and `outer` as the first premise of a
  purported `inner`-through-`outer`-through-`middle` proof rejects;
  the prefix accepts the real `inner`-through-`middle`-through-`outer`
  theorem call and checks `outer`-inside-`middle` is `False`.
- measured: a fresh positive prefix followed by the false call below;
  refusal must occur at this appended call, not earlier.
- claimed: the two transitivity premises are ordered and cannot be
  swapped merely because the direct conclusion is true.
- gap: one wrong-premise instance does not establish correctness
  for all chains.
- why: the desired conclusion is itself true and accepted directly;
  only the unavailable `outer`-inside-`middle` premise should reject.

```ken
-- Refuse: outer is not inside middle, so this chain has no second link.
theorem reject_swapped_transitivity_premise :
  IsTrue (interval_inside inner middle) =
  interval_inside_trans inner outer middle Proved Proved
```

## Evidence boundary

The Architect's separate AC-0 D0 at `d6c05223e` checked the five
generic proofs, a public-only client and a closed chain/swap shape
in scratch with no new `Axiom`, five owned Transparent laws and exact
scratch `trusted_base()` identity equality 114 → 114.

The authored fences were separately extracted into temporary
catalog-root clients on an isolated worktree at `d6c05223e`, with
the Architect's **catalog-only D0 patch**, with SHA-256
`1e84fec6c5b4c04e05bb66f22cc043778d3092c1fe90bed70894f8e0ab30d1ae`.
The CLI used was the Architect's locally built D0-scratch-base binary
(SHA-256 at this run:
`3be48046691c7eddb19f10e9726a2b9ce205de1b66e20fbb96fa0b0e69544857`),
not a CLI from shipped §5 exports. `ken check` returned
rc 0 on the complete thirty-theorem accepted client. Two fresh
accepted-prefix-plus-one-refusal clients returned rc 1/K0004
`KernelRejected(TypeMismatch)`: the reversed-chain refusal starts at
byte 6186 and errors within its appended theorem at 6186–6269;
the wrong-premise refusal starts at byte 6195 and errors within its
appended theorem at 6195–6339. No import, fixture or earlier theorem
failed. These D0-overlay checks do not establish shipped §5 exports,
official conformance-runner execution, CI, publication, landing or
landed trust.
