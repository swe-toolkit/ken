# Interval width under inclusion

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §16.1–§16.3.
Frame: `docs/program/issues/SPEC-NUMERICAL-INTERVAL-WIDTH-INSIDE-MONO.md`.

These closed clients exercise the public interval-width and Rational
subtraction monotonicity laws. Each direct closed `Proved` conclusion is
independent of the corresponding law application. The clients establish
instances, not the generic laws. They do not import `IsTrue`; it is already
available in the client prelude.

`MkRational n Zero` denotes the natural rational `n`. In A1, `[1,2]` is inside
`[0,4]`, with widths one and four. In A2, inverted `[3,1]` is inside `[0,4]`
and its truncated width is zero. In A3, `[0,3]` intersect `[1,4]` is `[1,3]`,
which is inside `[0,3]`. A4 checks rational subtraction monotonicity with both
arguments moving: `1 − 1 ≤ 3 − 0`.

## Accepted client

The accepted block checks each closed conclusion independently, then applies
the corresponding public law. Each refusal below is appended alone to a fresh
copy of this block.

```ken
import Data.Numeric.Rational
  (Rational, MkRational, rational_leq, rational_sub, rational_sub_mono)

import Algorithm.Numeric.RationalInterval
  (Interval,
    MkInterval,
    interval_width,
    interval_width_inside,
    interval_intersect,
    interval_intersect_inside_left)

const r0 : Rational = MkRational Zero Zero

const r1 : Rational = MkRational (Suc Zero) Zero

const r2 : Rational = MkRational (Suc (Suc Zero)) Zero

const r3 : Rational = MkRational (Suc (Suc (Suc Zero))) Zero

const r4 : Rational = MkRational (Suc (Suc (Suc (Suc Zero)))) Zero

const inner : Interval = MkInterval r1 r2

const outer : Interval = MkInterval r0 r4

const inverted : Interval = MkInterval r3 r1

const intersect_left : Interval = MkInterval r0 r3

const intersect_right : Interval = MkInterval r1 r4

const intersection : Interval = interval_intersect intersect_left intersect_right

-- A1: [1,2] lies inside [0,4], and its width is 1 rather than 4.
theorem a1_closed_conclusion
    : IsTrue (rational_leq (interval_width inner) (interval_width outer)) =
  Proved

theorem a1_width_inside_application
    : IsTrue (rational_leq (interval_width inner) (interval_width outer)) =
  interval_width_inside inner outer Proved

-- A2: inverted [3,1] is included in [0,4]; its width truncates to zero.
theorem a2_closed_conclusion
    : IsTrue (rational_leq (interval_width inverted) (interval_width outer)) =
  Proved

theorem a2_width_inside_application
    : IsTrue (rational_leq (interval_width inverted) (interval_width outer)) =
  interval_width_inside inverted outer Proved

-- A3: intersect([0,3],[1,4]) lies inside [0,3].
theorem a3_closed_conclusion
    : IsTrue (rational_leq (interval_width intersection) (interval_width intersect_left)) =
  Proved

theorem a3_intersection_inside_left_application
    : IsTrue (rational_leq (interval_width intersection) (interval_width intersect_left)) =
  interval_width_inside
    intersection
    intersect_left
    (interval_intersect_inside_left intersect_left intersect_right)

-- A4: both arguments move: 1 - 1 <= 3 - 0.
theorem a4_closed_conclusion : IsTrue (rational_leq (rational_sub r1 r1) (rational_sub r3 r0)) =
  Proved

theorem a4_subtraction_monotonicity_application
    : IsTrue (rational_leq (rational_sub r1 r1) (rational_sub r3 r0)) =
  rational_sub_mono r1 r3 r1 r0 Proved Proved
```

### stdlib/numerical/interval-width-inside-closed-inclusion

- spec: `spec/50-stdlib/68-numerical.md` §§16.2–16.3.
- promise class: durable invariant.
- given: `[1,2]` inside `[0,4]`.
- expect: the inner interval's width is no greater than the outer width.
- measured: a direct closed proposition and an application of
  `interval_width_inside` with the same instance.
- claimed: interval inclusion implies width inclusion.
- gap: a closed instance does not establish the generic implication.
- why: the chosen widths are one and four, so the endpoints are non-degenerate.

### stdlib/numerical/interval-width-inside-inverted

- spec: `spec/50-stdlib/68-numerical.md` §§16.2–16.3.
- promise class: durable invariant.
- given: inverted `[3,1]` inside `[0,4]`.
- expect: the inverted interval's truncated width is no greater than four.
- measured: a direct closed proposition and a separate public-law application.
- claimed: the inclusion law covers an inverted inner interval without an
  additional validity premise.
- gap: this is one inverted interval instance, not all inverted intervals.
- why: endpoint inclusion holds while the inner width is zero by truncation.

### stdlib/numerical/interval-width-inside-intersection

- spec: `spec/50-stdlib/68-numerical.md` §§16.2–16.3.
- promise class: durable invariant.
- given: `[0,3] ∩ [1,4]` inside `[0,3]`.
- expect: the intersection's width is no greater than the left operand's.
- measured: a direct closed proposition and an application using
  `interval_intersect_inside_left` as the inclusion premise.
- claimed: the width law composes with the public intersection inclusion law.
- gap: this closed pair does not establish the generic intersection law.
- why: the intersection is formed by the public operation, not supplied as a
  hand-written endpoint result.

### stdlib/numerical/rational-subtraction-monotonicity-both-arguments

- spec: `spec/50-stdlib/68-numerical.md` §§16.1–16.3.
- promise class: durable invariant.
- given: `1 ≤ 3` and `0 ≤ 1`.
- expect: `1 − 1 ≤ 3 − 0` under truncated rational subtraction.
- measured: a direct closed proposition and an application of
  `rational_sub_mono` with both arguments changing.
- claimed: the public subtraction law checks both monotonicity directions in
  its stated four-argument shape.
- gap: this instance does not prove the generic law or isolate each direction.
- why: both supplied order premises are true, and the conclusion is checked
  independently of the law application.

## Isolated refusals

Append one refusal at a time to a fresh copy of the complete accepted client.
Each twin has an accepted control with the reversed, true claim on the same
closed values. Pin the refusal by `KernelRejected` and a span inside that
refusal's own theorem declaration. Do not pin a reason string or raw id.

### stdlib/numerical/reject-interval-width-inside-reversed

- spec: `spec/50-stdlib/68-numerical.md` §16.3.
- promise class: durable invariant.
- given: `[0,4]` and `[1,2]`; the positive client proves the forward width
  inclusion for these intervals.
- expect: claiming the wider interval's width is no greater than the
  narrower interval's is refused.
- measured: the accepted control proves width `[1,2]` ≤ width `[0,4]`; the
  twin asserts the reverse proposition.
- claimed: width inclusion follows in the specified direction, not its reverse.
- gap: this one reversed pair does not characterize every wrong width
  implementation.
- controls: the accepted client checks the same endpoints and law in the
  true direction.
- why: widths are four and one, so the reversed inequality is false.

```ken
-- Control: [1,2] is no wider than [0,4].
theorem reversed_width_control
    : IsTrue (rational_leq (interval_width inner) (interval_width outer)) =
  Proved

-- Reject: [0,4] is not no wider than [1,2].
theorem reject_interval_width_inside_reversed
    : IsTrue (rational_leq (interval_width outer) (interval_width inner)) =
  Proved
```

### stdlib/numerical/reject-rational-subtraction-first-argument-direction

- spec: `spec/50-stdlib/68-numerical.md` §16.3.
- promise class: durable invariant.
- given: `3 − 1` and `2 − 1`.
- expect: claiming `3 − 1 ≤ 2 − 1` is refused.
- measured: the accepted control directly proves the reversed true closed
  comparison, `2 − 1 ≤ 3 − 1`; the twin reverses its conclusion.
- claimed: the first argument's monotonicity direction is enforced.
- gap: this twin does not test the second argument's direction.
- controls: the same closed values accept with the conclusion reversed.
- why: the first subtrahend is fixed and the first minuend alone changes.

```ken
-- Control: 2 - 1 is at most 3 - 1.
theorem first_argument_direction_control
    : IsTrue (rational_leq (rational_sub r2 r1) (rational_sub r3 r1)) =
  Proved

-- Reject: 3 - 1 is not at most 2 - 1.
theorem reject_rational_subtraction_first_argument_direction
    : IsTrue (rational_leq (rational_sub r3 r1) (rational_sub r2 r1)) =
  Proved
```

### stdlib/numerical/reject-rational-subtraction-second-argument-direction

- spec: `spec/50-stdlib/68-numerical.md` §16.3.
- promise class: durable invariant.
- given: `3 − 0` and `3 − 1`.
- expect: claiming `3 − 0 ≤ 3 − 1` is refused.
- measured: the accepted control directly proves the reversed true closed
  comparison, `3 − 1 ≤ 3 − 0`; the twin reverses its conclusion.
- claimed: the second argument's antitone direction is enforced.
- gap: this twin does not test the first argument's direction.
- controls: the same closed values accept with the conclusion reversed.
- why: the minuend is fixed and the subtrahend alone changes.

```ken
-- Control: 3 - 1 is at most 3 - 0.
theorem second_argument_direction_control
    : IsTrue (rational_leq (rational_sub r3 r1) (rational_sub r3 r0)) =
  Proved

-- Reject: 3 - 0 is not at most 3 - 1.
theorem reject_rational_subtraction_second_argument_direction
    : IsTrue (rational_leq (rational_sub r3 r0) (rational_sub r3 r1)) =
  Proved
```

## Evidence boundary

This seed instantiates `spec/50-stdlib/68-numerical.md` §§16.1–16.3.
Architect AC-0 ruling `evt_aha7hvm88ckz`, item 5, and AC-2 release
`evt_604adzbznbep5` supply the scratch evidence only. A scratch overlay of
the D0 catalog cut at base `750f1622b0aed49ac8deedd415ba06388f068fcc`
passed the extracted accepted block, all three isolated controls and twins,
and an own-theorem-span `KernelRejected(TypeMismatch)` check. The one targeted
scratch test passed 1/1; all four extracted Ken fences passed `ken fmt --check`.
Log:
`/workspaces/ken/tmp/cv-width-inside-mono-seed-check.log`. This is not official
conformance-runner execution, product package-test evidence, CI, publication,
landing, or a landed trust measurement. No external or `local/refs` material
was consulted.
