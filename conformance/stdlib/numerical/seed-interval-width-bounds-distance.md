# Interval width bounds directed subtraction

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §18, approved child
`081c3a359ea6edcfe4177c976b0ad6272c47b998`.

This seed checks the public theorem's generic call shape, a nontrivial
closed bound, a tight endpoint pair, and the required second containment
premise. The closed applications instantiate the theorem; they do not
establish its generic proof. `MkRational n Zero` denotes the natural
rational `n`. `IsTrue` is available from the client prelude.

For A1, the interval is `[1/2,3]`, `x=1`, and `y=5/2`. Then
`rational_sub y x = 3/2` and the width is `5/2`. A2 uses the endpoints
`x=1/2` and `y=3`; the directed difference equals the width, so both
comparison directions should hold. X uses `[1,2]`, `x=1`, and `y=5`.
Here `x` is contained but `y` is not, and `rational_sub 5 1 = 4` is
greater than the width one.

## Accepted client

Check this whole client before appending X. The generic client fixes the
public argument order without adding an order premise between `x` and
`y`. A1 and A2 check the closed results independently of their public
law applications. The narrow-interval control is the positive control
for X.

```ken
import Data.Numeric.Rational
  (Rational, MkRational, rational_leq, rational_sub)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_contains, interval_width,
   interval_width_bounds_sub)

const half : Rational = MkRational (Suc Zero) (Suc Zero)
const one : Rational = MkRational (Suc Zero) Zero
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const three : Rational = MkRational (Suc (Suc (Suc Zero))) Zero
const five_nat : Nat = Suc (Suc (Suc (Suc (Suc Zero))))
const five : Rational = MkRational five_nat Zero
const five_halves : Rational = MkRational five_nat (Suc Zero)

const wide : Interval = MkInterval half three
const narrow : Interval = MkInterval one two

theorem width_bounds_public_client
      (i : Interval) (x : Rational) (y : Rational)
      (x_member : IsTrue (interval_contains i x))
      (y_member : IsTrue (interval_contains i y))
    : IsTrue (rational_leq (rational_sub y x) (interval_width i)) =
  interval_width_bounds_sub i x y x_member y_member

-- A1: [1/2,3], x=1, y=5/2; the bound is 3/2 <= 5/2.
theorem a1_closed_bound
    : IsTrue (rational_leq (rational_sub five_halves one)
                (interval_width wide)) =
  Proved

theorem a1_width_bounds_application
    : IsTrue (rational_leq (rational_sub five_halves one)
                (interval_width wide)) =
  interval_width_bounds_sub wide one five_halves Proved Proved

-- A2: x=1/2 and y=3 reach the width 5/2 exactly.
theorem a2_closed_forward
    : IsTrue (rational_leq (rational_sub three half)
                (interval_width wide)) =
  Proved

theorem a2_closed_reverse
    : IsTrue (rational_leq (interval_width wide)
                (rational_sub three half)) =
  Proved

theorem a2_width_bounds_application
    : IsTrue (rational_leq (rational_sub three half)
                (interval_width wide)) =
  interval_width_bounds_sub wide half three Proved Proved

-- X control: both 1 and 2 are contained in [1,2].
theorem x_control_x_member : IsTrue (interval_contains narrow one) = Proved

theorem x_control_y_member : IsTrue (interval_contains narrow two) = Proved

theorem x_control_closed_bound
    : IsTrue (rational_leq (rational_sub two one)
                (interval_width narrow)) =
  Proved

theorem x_control_width_bounds_application
    : IsTrue (rational_leq (rational_sub two one)
                (interval_width narrow)) =
  interval_width_bounds_sub narrow one two
    x_control_x_member x_control_y_member
```

### stdlib/numerical/interval-width-bounds-directed-subtraction

- spec: §18, `interval_width_bounds_sub`.
- promise class: durable invariant.
- given: an arbitrary interval and two Rational values with separate
  `IsTrue (interval_contains ...)` witnesses.
- expect: the public theorem yields
  `IsTrue (rational_leq (rational_sub y x) (interval_width i))`.
- measured: `width_bounds_public_client` applies the imported theorem
  with the exact public argument order and no `x ≤ y` premise.
- claimed: the theorem is available to an external generic client with
  both containment premises.
- gap: this client checks public type use, not the theorem's generic
  proof body.
- why: the theorem call carries both witnesses explicitly.

### stdlib/numerical/interval-width-bounds-nontrivial-value

- spec: §18, `interval_width_bounds_sub` and `interval_width`.
- promise class: durable invariant.
- given: `[1/2,3]`, `x=1`, and `y=5/2`.
- expect: `rational_sub y x = 3/2` is no greater than the interval
  width `5/2`.
- measured: A1 checks the closed inequality and separately applies
  the public law to the same values.
- claimed: the bound admits a nonzero directed subtraction below the
  width.
- gap: one closed instance does not establish the generic law.
- why: the subtraction is neither zero nor the full width.

### stdlib/numerical/interval-width-bounds-tight-endpoints

- spec: §18, `interval_width_bounds_sub`.
- promise class: durable invariant.
- given: `[1/2,3]`, `x=1/2`, and `y=3`.
- expect: the directed subtraction equals the interval width `5/2`.
- measured: A2 checks both `rational_sub y x ≤ width` and the reverse
  width comparison; it also applies the public law.
- claimed: the bound is tight at the interval endpoints.
- gap: this endpoint pair does not characterize all contained values.
- why: the two closed comparisons expose equality rather than only a
  loose upper bound.

## Isolated refusal

Append only X to a fresh complete copy of the accepted client. The
positive control keeps the interval and `x` fixed while using contained
`y=2`. X changes to the counterexample `y=5` and omits only its required
y-containment witness. It does not assert the separate closed false
conclusion directly.

```ken
-- X: [1,2] contains 1 but not 5; do not supply y's containment proof.
theorem reject_width_bound_without_y_containment
    : IsTrue (rational_leq (rational_sub five one)
                (interval_width narrow)) =
  interval_width_bounds_sub narrow one five x_control_x_member
```

### stdlib/numerical/reject-width-bound-without-y-containment

- spec: §18, `interval_width_bounds_sub`.
- promise class: durable invariant.
- given: `[1,2]`, contained `x=1`, and non-contained `y=5`.
- expect: the theorem application is refused at the omitted
  y-containment argument.
- measured: X uses the same theorem and target shape as the accepted
  narrow-interval control, supplies its accepted x witness, and omits
  only the second witness. The control with `y=2` accepts.
- claimed: the y-containment premise is required; the bound cannot be
  concluded for the counterexample values without it.
- gap: this argument-omission refusal does not separately assert the
  false closed inequality `4 ≤ 1`.
- controls: the accepted narrow-interval control uses the same
  interval, x value, operation, and conclusion form with contained
  `y=2` and both witnesses.
- why: on the X input, truncated subtraction is four and the interval
  width is one. The accepted prefix contains the theorem import and a
  successful call on the same interval; X is its first call missing the
  second witness.

## Evidence boundary

The cited approved Spec child supplies the normative §18 contract. This
CV branch changes only `/conformance`; no Ken check or official
conformance execution was run for this seed. No package test, CI,
publication, landing, or landed trust measurement is claimed. The seed
was written from Ken's `/spec` and first principles; no external or
`local/refs` material was consulted.
