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

## Isolated refusals

Append each negative declaration below alone to a fresh complete copy of
the accepted client. Keep the accepted narrow-interval control unchanged.
X supplies `Proved` for the y-containment argument; its expected type is
`IsTrue (interval_contains narrow five)`, which is false. This pins the
identity of the second premise. X-ARITY separately omits that argument
to check the theorem cannot be used as a proof before both premises are
supplied. The arity row alone does not discriminate which containment
the second premise requires. Neither row asserts the separate closed
false conclusion.

```ken
-- X: y=5 is outside [1,2]; this is not a proof of y's membership.
theorem reject_width_bound_without_y_containment
    : IsTrue (rational_leq (rational_sub five one)
                (interval_width narrow)) =
  interval_width_bounds_sub narrow one five x_control_x_member Proved
```

### stdlib/numerical/reject-width-bound-without-y-containment

- spec: §18, `interval_width_bounds_sub`.
- promise class: durable invariant.
- given: `[1,2]`, contained `x=1`, and non-contained `y=5`.
- expect: X is refused at the y-containment argument, whose expected
  type is `IsTrue (interval_contains narrow five)`.
- measured: X supplies `Proved` in the y slot after the accepted x
  witness. The control with contained `y=2` and both witnesses accepts.
- claimed: the second premise specifically requires containment of `y`.
- gap: X does not separately assert the false closed inequality `4 ≤ 1`.
- controls: the accepted narrow-interval control uses the same
  interval, x value, operation, and conclusion form with contained
  `y=2` and both witnesses.
- why: `interval_contains narrow five` is false, so `Proved` cannot
  inhabit the expected y-membership type. A second premise that repeats
  x's membership would accept that same `Proved`.

```ken
-- X-ARITY: omit y's membership argument.
theorem reject_width_bound_without_y_argument
    : IsTrue (rational_leq (rational_sub five one)
                (interval_width narrow)) =
  interval_width_bounds_sub narrow one five x_control_x_member
```

### stdlib/numerical/reject-width-bound-without-y-argument

- spec: §18, `interval_width_bounds_sub`.
- promise class: durable invariant.
- given: `[1,2]`, contained `x=1`, and `y=5`.
- expect: X-ARITY is refused because the application still awaits
  y-containment while the declaration expects the result proof.
- measured: X-ARITY supplies only the accepted x witness; the
  narrow-interval control supplies both witnesses for contained `y=2`.
- claimed: the public theorem cannot yield its result before receiving
  the y-containment argument.
- gap: this arity refusal alone does not distinguish the y premise
  from a duplicated x premise; X pins that identity.
- controls: the same operation and target shape accept with `y=2` and
  both membership witnesses.
- why: after one witness the application is still a function, not a
  proof of the bound.

## Evidence boundary

The cited approved Spec child supplies the normative §18 contract. This
CV branch changes only `/conformance`; no Ken check or official
conformance execution was run for this seed. No package test, CI,
publication, landing, or landed trust measurement is claimed. The seed
was written from Ken's `/spec` and first principles; no external or
`local/refs` material was consulted.
