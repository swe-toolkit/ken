# Interval width under inclusion

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §16.1–§16.3.
Frame: `docs/program/issues/SPEC-NUMERICAL-INTERVAL-WIDTH-INSIDE-MONO.md`.

These closed clients exercise the public interval-width, Rational subtraction
monotonicity and Nat.Order subtraction laws. Each direct closed `Proved`
conclusion is independent of the corresponding law application. The clients
establish instances, not the generic laws. They do not import `IsTrue`; it is
already available in the client prelude.

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
import Data.Numeric.Nat.Arithmetic (mul)

import Data.Numeric.Nat.Order (leq_nat, sub, mul_sub_distrib_r, mul_sub_distrib_l)

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

const nat_one : Nat = Suc Zero

const nat_two : Nat = Suc nat_one

const nat_three : Nat = Suc nat_two

const nat_four : Nat = Suc nat_three

const nat_six : Nat = Suc (Suc nat_four)

-- N1: 4 - 1 = 3 is at most 4, without saturating to zero.
theorem nat_sub_leq_left_closed : IsTrue (leq_nat (sub nat_four nat_one) nat_four) = Proved

theorem nat_sub_leq_left_application : IsTrue (leq_nat (sub nat_four nat_one) nat_four) =
  (proof leq_left for sub) nat_four nat_one

-- N2: subtracting 1 from 2 <= 4 yields 1 <= 3.
theorem nat_sub_left_mono_closed
    : IsTrue (leq_nat (sub nat_two nat_one) (sub nat_four nat_one)) =
  Proved

theorem nat_sub_left_mono_application
    : IsTrue (leq_nat (sub nat_two nat_one) (sub nat_four nat_one)) =
  (proof left_mono for sub) nat_two nat_four nat_one Proved

-- N3: subtracting 3 rather than 1 from 4 yields 1 <= 3.
theorem nat_sub_right_antitone_closed
    : IsTrue (leq_nat (sub nat_four nat_three) (sub nat_four nat_one)) =
  Proved

theorem nat_sub_right_antitone_application
    : IsTrue (leq_nat (sub nat_four nat_three) (sub nat_four nat_one)) =
  (proof right_antitone for sub) nat_four nat_one nat_three Proved

-- N4: 2 * (4 - 1) = (2 * 4) - (2 * 1) = 6.
theorem nat_mul_sub_distrib_r_closed
    : Equal Nat
        (mul nat_two (sub nat_four nat_one))
        (sub (mul nat_two nat_four) (mul nat_two nat_one)) =
  Proved

theorem nat_mul_sub_distrib_r_application
    : Equal Nat
        (mul nat_two (sub nat_four nat_one))
        (sub (mul nat_two nat_four) (mul nat_two nat_one)) =
  mul_sub_distrib_r nat_two nat_four nat_one

theorem nat_mul_sub_distrib_r_left_value
    : Equal Nat (mul nat_two (sub nat_four nat_one)) nat_six =
  Proved

theorem nat_mul_sub_distrib_r_right_value
    : Equal Nat (sub (mul nat_two nat_four) (mul nat_two nat_one)) nat_six =
  Proved

-- N5: (4 - 2) * 3 = (4 * 3) - (2 * 3) = 6.
theorem nat_mul_sub_distrib_l_closed
    : Equal Nat
        (mul (sub nat_four nat_two) nat_three)
        (sub (mul nat_four nat_three) (mul nat_two nat_three)) =
  Proved

theorem nat_mul_sub_distrib_l_application
    : Equal Nat
        (mul (sub nat_four nat_two) nat_three)
        (sub (mul nat_four nat_three) (mul nat_two nat_three)) =
  mul_sub_distrib_l nat_four nat_two nat_three

theorem nat_mul_sub_distrib_l_left_value
    : Equal Nat (mul (sub nat_four nat_two) nat_three) nat_six =
  Proved

theorem nat_mul_sub_distrib_l_right_value
    : Equal Nat (sub (mul nat_four nat_three) (mul nat_two nat_three)) nat_six =
  Proved
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

## Nat.Order direct law cases

The attached proofs are selected through the public `sub` subject; the two
multiplication/subtraction theorems are imported by name. Each N1–N5 pair
checks a closed `Proved` conclusion separately from its checked law application.
The distribution cases also check both sides against six, avoiding an equality
that would pass if both sides collapsed to zero. These finite examples do not
replace the open checked law types in §16.1.

### stdlib/numerical/nat-sub-leq-left

- spec: `spec/50-stdlib/68-numerical.md` §16.1.
- promise class: durable invariant.
- given: `4 ∸ 1 = 3`, where the truncated result is nonzero.
- expect: `3 ≤ 4` is proved by the public `sub::leq_left` proof.
- measured: a direct closed `Proved` comparison and a separate attached-proof
  application to the same closed arguments.
- claimed: truncated subtraction stays below its minuend on this input.
- gap: one instance does not establish the generic bound.
- why: a positive minuend and non-saturating subtrahend test a real bound.

### stdlib/numerical/nat-sub-left-monotonicity

- spec: `spec/50-stdlib/68-numerical.md` §16.1.
- promise class: durable invariant.
- given: `2 ≤ 4`, with one subtracted from both sides.
- expect: `2 ∸ 1 = 1 ≤ 3 = 4 ∸ 1` is proved by `sub::left_mono`.
- measured: the direct closed `Proved` conclusion and a separate application
  of the attached proof with the accepted `2 ≤ 4` premise.
- claimed: subtraction preserves the first argument's order on this input.
- gap: the pair does not prove monotonicity for all naturals.
- why: the positive results differ, so reversing the conclusion is false.

### stdlib/numerical/nat-sub-right-antitonicity

- spec: `spec/50-stdlib/68-numerical.md` §16.1.
- promise class: durable invariant.
- given: `1 ≤ 3`, both subtracted from four.
- expect: `4 ∸ 3 = 1 ≤ 3 = 4 ∸ 1` is proved by `sub::right_antitone`.
- measured: the direct closed `Proved` conclusion and a separate application
  of the attached proof with the accepted `1 ≤ 3` premise.
- claimed: subtraction reverses the second argument's order on this input.
- gap: the pair does not prove antitonicity for all naturals.
- why: the positive results differ, so reversing the conclusion is false.

### stdlib/numerical/nat-mul-sub-distrib-right

- spec: `spec/50-stdlib/68-numerical.md` §16.1.
- promise class: durable invariant.
- given: factor two applied to `4 ∸ 1`.
- expect: `2 · (4 ∸ 1) = (2 · 4) ∸ (2 · 1) = 6`.
- measured: a direct closed `Proved` equality and a separate application of
  `mul_sub_distrib_r`; both sides independently check equal to six.
- claimed: the factor on the left distributes over truncated subtraction here.
- gap: this instance does not establish the generic theorem.
- why: both minuends are positive and each nonzero difference is six.

### stdlib/numerical/nat-mul-sub-distrib-left

- spec: `spec/50-stdlib/68-numerical.md` §16.1.
- promise class: durable invariant.
- given: factor three applied on the right of `4 ∸ 2`.
- expect: `(4 ∸ 2) · 3 = (4 · 3) ∸ (2 · 3) = 6`.
- measured: a direct closed `Proved` equality and a separate application of
  `mul_sub_distrib_l`; both sides independently check equal to six.
- claimed: the factor on the right distributes over truncated subtraction here.
- gap: this instance does not establish the generic theorem.
- why: both minuends are positive and each nonzero difference is six.

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
`evt_604adzbznbep5` supply the earlier scratch evidence only. A scratch
overlay of the D0 catalog cut at base
`750f1622b0aed49ac8deedd415ba06388f068fcc` passed the original A1–A4
accepted block and three isolated controls and twins, with own-theorem-span
`KernelRejected(TypeMismatch)` checks. Its targeted scratch test passed 1/1;
log: `/workspaces/ken/tmp/cv-width-inside-mono-seed-check.log`.

The expanded A1–A4 and N1–N5 client checked from this candidate's catalog root
with a probe-only `IsTrue` import for standalone CLI checking. All three
unchanged refusal fences were rechecked after the expanded accepted prefix:
each same-form control accepted, and each twin refused at its own theorem span.
All four extracted Ken fences passed `ken fmt --check`. These are targeted local
checks, not official conformance-runner execution, product package-test
coverage, CI, publication, landing, or a landed trust measurement. No external
or `local/refs` material was consulted.
